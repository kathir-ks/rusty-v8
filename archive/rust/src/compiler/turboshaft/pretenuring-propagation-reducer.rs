// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::{HashMap, HashSet};

/// Represents the type of allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AllocationType {
    kYoung,
    kOld,
}

/// Represents an operation index.
type OpIndex = usize;

/// Represents a base operation.
trait Operation {
    fn opcode(&self) -> Opcode;
    fn as_any(&self) -> &dyn std::any::Any;
    fn is<T: 'static>(&self) -> bool {
        self.as_any().is::<T>()
    }
    fn cast<T: 'static>(&self) -> &T {
        self.as_any().downcast_ref::<T>().unwrap()
    }
    fn try_cast<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

/// Represents a phi operation.
#[derive(Debug)]
struct PhiOp {
    inputs: Vec<OpIndex>,
}

impl PhiOp {
    fn new(inputs: Vec<OpIndex>) -> Self {
        Self { inputs }
    }

    fn inputs(&self) -> &[OpIndex] {
        &self.inputs
    }
}

impl Operation for PhiOp {
    fn opcode(&self) -> Opcode {
        Opcode::kPhi
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents an allocate operation.
#[derive(Debug)]
struct AllocateOp {
    type_: AllocationType,
}

impl AllocateOp {
    fn new(type_: AllocationType) -> Self {
        Self { type_ }
    }

    fn type_(&self) -> AllocationType {
        self.type_
    }
}

impl Operation for AllocateOp {
    fn opcode(&self) -> Opcode {
        Opcode::kAllocate
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents a store operation.
#[derive(Debug)]
struct StoreOp {
    base: OpIndex,
    value: OpIndex,
}

impl StoreOp {
    fn new(base: OpIndex, value: OpIndex) -> Self {
        Self { base, value }
    }

    fn base(&self) -> OpIndex {
        self.base
    }

    fn value(&self) -> OpIndex {
        self.value
    }
}

impl Operation for StoreOp {
    fn opcode(&self) -> Opcode {
        Opcode::kStore
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents an opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    kPhi,
    kAllocate,
    kStore,
    // Add other opcodes as needed
}

/// Represents an input graph.
struct InputGraph {
    operations: Vec<Box<dyn Operation>>,
}

impl InputGraph {
    fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    fn add_operation(&mut self, op: Box<dyn Operation>) -> OpIndex {
        self.operations.push(op);
        self.operations.len() - 1
    }

    fn get(&self, index: OpIndex) -> &dyn Operation {
        &*self.operations[index]
    }

    fn index<T: Operation + 'static>(&self, op: &T) -> OpIndex {
        self.operations
            .iter()
            .position(|o| {
                o.as_any().downcast_ref::<T>().map_or(false, |x| std::ptr::eq(x, op))
            })
            .expect("Operation not found")
    }

    fn all_operations(&self) -> &Vec<Box<dyn Operation>> {
        &self.operations
    }
}

fn could_be_allocate(base: &dyn Operation) -> bool {
    base.is::<PhiOp>() || base.is::<AllocateOp>()
}

/// Analyzes pretenuring propagation.
struct PretenuringPropagationAnalyzer {
    input_graph_: InputGraph,
    store_graph_: HashMap<OpIndex, Vec<OpIndex>>,
    old_allocs_: Vec<OpIndex>,
    queue_: Vec<OpIndex>,
    old_phis_: HashSet<OpIndex>,
}

impl PretenuringPropagationAnalyzer {
    /// Creates a new `PretenuringPropagationAnalyzer`.
    fn new(input_graph: InputGraph) -> Self {
        Self {
            input_graph_: input_graph,
            store_graph_: HashMap::new(),
            old_allocs_: Vec::new(),
            queue_: Vec::new(),
            old_phis_: HashSet::new(),
        }
    }

    fn find_or_create(&mut self, base_idx: OpIndex) -> &mut Vec<OpIndex> {
        self.store_graph_.entry(base_idx).or_insert(Vec::new())
    }

    fn create(&mut self, base_idx: OpIndex) -> &mut Vec<OpIndex> {
        self.store_graph_.insert(base_idx, Vec::new());
        self.store_graph_.get_mut(&base_idx).unwrap()
    }

    fn try_find(&self, base_idx: OpIndex) -> Option<&Vec<OpIndex>> {
        self.store_graph_.get(&base_idx)
    }

    /// Processes a store operation.
    fn process_store(&mut self, store: &StoreOp) {
        let base_idx = store.base();
        let value_idx = store.value();
        let base = self.input_graph_.get(base_idx);
        let value = self.input_graph_.get(value_idx);

        if !could_be_allocate(base) || !could_be_allocate(value) {
            return;
        }

        if value.is::<AllocateOp>() && value.cast::<AllocateOp>().type_() == AllocationType::kOld {
            // {value} is already Old, and we don't care about new-to-old and old-to-old
            // stores.
            return;
        }

        if value.is::<PhiOp>() && self.try_find(value_idx).is_none() {
            // {value} is not worth being recorded, as it's not an Allocation (or a Phi
            // of Allocations) that could be promoted to Old.
            return;
        }

        let stored_in_base = self.find_or_create(base_idx);
        stored_in_base.push(value_idx);
    }

    /// Processes a phi operation.
    fn process_phi(&mut self, phi: &PhiOp) {
        // Phis act as storing all of their inputs. It's not how they work in
        // practice, but if a Phi has a Young input, and is stored in an Old object,
        // it makes sense to Oldify the phi input.

        // For better performance, we only record inputs that could be an allocation:
        // Phis with an entry in {store_graph_} or AllocateOp.
        // Note that this is slightly imprecise for loop Phis (since if the backedge
        // is a Phi itself, it won't have an entry in {store_graph_} yet), but it
        // should still be good enough for most cases.

        let mut interesting_inputs: Vec<OpIndex> = Vec::new();
        for input in phi.inputs() {
            let op = self.input_graph_.get(*input);
            if op.is::<AllocateOp>() {
                interesting_inputs.push(*input);
            } else if op.is::<PhiOp>() && self.try_find(*input).is_some() {
                interesting_inputs.push(*input);
            }
        }
        if interesting_inputs.is_empty() {
            return;
        }

        let stored_in_phi = self.create(self.input_graph_.index(phi));
        for input in interesting_inputs {
            stored_in_phi.push(input);
        }
    }

    /// Processes an allocate operation.
    fn process_allocate(&mut self, allocate: &AllocateOp) {
        if allocate.type_() == AllocationType::kOld {
            // We could be a bit more lazy in storing old AllocateOp into {old_allocs_}
            // (by waiting for a Store or a Phi to use the AllocateOp), but there is
            // usually very few old allocation, so it makes sense to do it eagerly.
            self.old_allocs_.push(self.input_graph_.index(allocate));
        }
    }

    /// Pushes contained values into the queue.
    fn push_contained_values(&mut self, base: OpIndex) -> bool {
        // Push into {queue_} all of the values that are "contained" into {base}:
        // values that are stored to {base} if {base} is an AllocateOp, or Phi inputs
        // if {base} is a Phi.
        let contained = self.try_find(base);
        if contained.is_none() {
            return false;
        }
        let contained = contained.unwrap();
        for index in contained {
            self.queue_.push(*index);
        }
        true
    }

    /// Performs a DFS from {old_alloc} and mark everything it finds as Old. The DFS
    /// stops on already-Old nodes.
    fn oldify_subgraph(&mut self, old_alloc: OpIndex) {
        self.queue_.clear();
        if !self.push_contained_values(old_alloc) {
            return;
        }

        while !self.queue_.is_empty() {
            let idx = self.queue_.pop();
            let idx = match idx {
                Some(x) => x,
                None => continue,
            };

            let op = self.input_graph_.get(idx);

            if let Some(alloc) = op.try_cast::<AllocateOp>() {
                if alloc.type_() == AllocationType::kOld {
                    continue;
                }
                let alloc_mut = unsafe {
                    let ptr = alloc as *const AllocateOp as *mut AllocateOp;
                    &mut *ptr
                };
                alloc_mut.type_ = AllocationType::kOld;
                self.push_contained_values(idx);
            } else {
                //DCHECK(op.Is<PhiOp>());
                if !op.is::<PhiOp>() {
                  panic!("Expected PhiOp");
                }
                if self.old_phis_.contains(&idx) {
                    continue;
                }
                self.old_phis_.insert(idx);
                self.push_contained_values(idx);
            }
        }
    }

    /// Propagates allocation types.
    fn propagate_allocation_types(&mut self) {
        for old_alloc in &self.old_allocs_ {
            self.oldify_subgraph(*old_alloc);
        }
    }

    fn should_skip_operation(_op: &dyn Operation) -> bool {
      // Implement your skip logic here, if any.
      false
    }

    /// Builds the store input graph.
    fn build_store_input_graph(&mut self) {
        for op in self.input_graph_.all_operations() {
            if Self::should_skip_operation(op) {
                continue;
            }
            match op.opcode() {
                Opcode::kStore => {
                    self.process_store(op.cast::<StoreOp>());
                }
                Opcode::kAllocate => {
                    self.process_allocate(op.cast::<AllocateOp>());
                }
                Opcode::kPhi => {
                    self.process_phi(op.cast::<PhiOp>());
                }
                _ => {}
            }
        }
    }

    /// Runs the analysis.
    fn run(&mut self) {
        self.build_store_input_graph();
        self.propagate_allocation_types();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretenuring_propagation() {
        let mut input_graph = InputGraph::new();

        // Create an AllocateOp of type kYoung.
        let alloc_op = AllocateOp::new(AllocationType::kYoung);
        let alloc_index = input_graph.add_operation(Box::new(alloc_op));

        // Create a PhiOp.
        let phi_op = PhiOp::new(vec![alloc_index]);
        let phi_index = input_graph.add_operation(Box::new(phi_op));

        // Create an AllocateOp of type kOld.
        let old_alloc_op = AllocateOp::new(AllocationType::kOld);
        let old_alloc_index = input_graph.add_operation(Box::new(old_alloc_op));

        // Create a StoreOp that stores the PhiOp into the old AllocateOp.
        let store_op = StoreOp::new(old_alloc_index, phi_index);
        input_graph.add_operation(Box::new(store_op));

        let mut analyzer = PretenuringPropagationAnalyzer::new(input_graph);
        analyzer.run();

        // Verify that the original AllocateOp is now of type kOld.
        let updated_alloc_op = analyzer.input_graph_.get(alloc_index).cast::<AllocateOp>();
        assert_eq!(updated_alloc_op.type_(), AllocationType::kOld);
    }
}
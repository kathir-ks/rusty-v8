// Converted from V8 C++ source files:
// Header: pretenuring-propagation-reducer.h
// Implementation: pretenuring-propagation-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/turboshaft/pretenuring-propagation-reducer.h
pub mod pretenuring_propagation_reducer {
    use crate::compiler::turboshaft::assembler::assembler::*;
    use crate::compiler::turboshaft::phase::phase::*;
    use crate::compiler::turboshaft::reducer_traits::*;
    use crate::compiler::turboshaft::utils::*;
    use crate::zone::zone_allocator::*;
    use crate::zone::zone_containers::*;
    use crate::zone::zone::*;
    use crate::compiler::turboshaft::copying_phase::PhiOp;
    use crate::compiler::turboshaft::late_escape_analysis_reducer::AllocateOp;
    use crate::compiler::turboshaft::types::AllocationType;
    use crate::compiler::turboshaft::deopt_data::OpIndex;
    use crate::compiler::turboshaft::graph::Graph;
    use crate::compiler::turboshaft::graph::Operation;
    use crate::base;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use crate::compiler::turboshaft::pretenuring_propagation_reducer;
    use crate::compiler::turboshaft::store_store_elimination_reducer_inl::ShouldSkipOperation;
    use crate::compiler::turboshaft::opcodes::Opcode;
    use std::ops::{Deref, DerefMut};

    pub struct PretenuringPropagationAnalyzer<'a> {
        zone_: &'a Zone,
        input_graph_: &'a mut Graph,
        old_allocs_: Vec<OpIndex>,
        store_graph_: HashMap<OpIndex, Vec<OpIndex>>,
        old_phis_: HashSet<OpIndex>,
        queue_: Vec<OpIndex>,
    }

    impl<'a> PretenuringPropagationAnalyzer<'a> {
        pub fn new(zone_: &'a Zone, input_graph_: &'a mut Graph) -> Self {
            PretenuringPropagationAnalyzer {
                zone_: zone_,
                input_graph_: input_graph_,
                old_allocs_: Vec::new(),
                store_graph_: HashMap::new(),
                old_phis_: HashSet::new(),
                queue_: Vec::new(),
            }
        }

        pub fn run(&mut self) {
            self.build_store_input_graph();
            self.propagate_allocation_types();
        }

        fn process_store(&mut self, store: &StoreOp) {
            let base_idx = store.base();
            let value_idx = store.value();
            let base = self.input_graph_.get(base_idx).unwrap();
            let value = self.input_graph_.get(value_idx).unwrap();

            if !Self::could_be_allocate(base) || !Self::could_be_allocate(value) {
                return;
            }

            if value.opcode == Opcode::kAllocate {
                let allocate_op = value.cast::<AllocateOp>();
                if allocate_op.type_ == AllocationType::kOld {
                    return;
                }
            }

            if value.opcode == Opcode::kPhi && !self.store_graph_.contains_key(&value_idx) {
                return;
            }

            let stored_in_base = self.find_or_create(base_idx);
            stored_in_base.push(value_idx);
        }

        fn process_phi(&mut self, phi: &PhiOp) {
            let mut interesting_inputs: Vec<OpIndex> = Vec::new();
            for input in phi.inputs() {
                if let Some(op) = self.input_graph_.get(input) {
                    if op.opcode == Opcode::kAllocate {
                        interesting_inputs.push(input);
                    } else if op.opcode == Opcode::kPhi && self.store_graph_.contains_key(&input) {
                        interesting_inputs.push(input);
                    }
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

        fn process_allocate(&mut self, allocate: &AllocateOp) {
            if allocate.type_ == AllocationType::kOld {
                self.old_allocs_.push(self.input_graph_.index(allocate));
            }
        }

        fn push_contained_values(&mut self, base: OpIndex) -> bool {
            let contained = self.try_find(base);
            match contained {
                Some(contained_vec) => {
                    for index in contained_vec {
                        self.queue_.push(index);
                    }
                    true
                }
                None => false,
            }
        }

        fn oldify_subgraph(&mut self, old_alloc: OpIndex) {
            self.queue_.clear();
            if !self.push_contained_values(old_alloc) {
                return;
            }

            while !self.queue_.is_empty() {
                let idx = self.queue_.pop().unwrap();
                if let Some(mut op) = self.input_graph_.get_mut(idx) {
                    if op.opcode == Opcode::kAllocate {
                        let mut alloc = op.cast_mut::<AllocateOp>();
                        if alloc.type_ == AllocationType::kOld {
                            continue;
                        }
                        alloc.type_ = AllocationType::kOld;
                        self.push_contained_values(idx);
                    } else {
                        if op.opcode != Opcode::kPhi {
                            continue;
                        }
                        if self.old_phis_.contains(&idx) {
                            continue;
                        }
                        self.old_phis_.insert(idx);
                        self.push_contained_values(idx);
                    }
                }
            }
        }

        fn propagate_allocation_types(&mut self) {
            for old_alloc in &self.old_allocs_ {
                self.oldify_subgraph(*old_alloc);
            }
        }

        fn build_store_input_graph(&mut self) {
            for op in self.input_graph_.all_operations_mut() {
                if ShouldSkipOperation(op) {
                    continue;
                }
                match op.opcode {
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

        fn find_or_create(&mut self, idx: OpIndex) -> &mut Vec<OpIndex> {
            if self.store_graph_.contains_key(&idx) {
                self.store_graph_.get_mut(&idx).unwrap()
            } else {
                self.create(idx)
            }
        }

        fn create(&mut self, idx: OpIndex) -> &mut Vec<OpIndex> {
            self.store_graph_.insert(idx, Vec::new());
            self.store_graph_.get_mut(&idx).unwrap()
        }

        fn try_find(&self, idx: OpIndex) -> Option<&Vec<OpIndex>> {
            self.store_graph_.get(&idx)
        }

        fn could_be_allocate(base: &Operation) -> bool {
            base.opcode == Opcode::kPhi || base.opcode == Opcode::kAllocate
        }
    }

    pub trait NextTrait {
        fn analyze(&mut self);
    }

    pub struct PretenuringPropagationReducer<Next: NextTrait> {
        asm: Assembler,
        next: Next,
    }

    impl<Next: NextTrait> PretenuringPropagationReducer<Next> {
        pub fn new(asm: Assembler, next: Next) -> Self {
            PretenuringPropagationReducer { asm, next }
        }

        pub fn analyze(&mut self) {
            let mut analyzer = PretenuringPropagationAnalyzer::new(
                self.asm.phase_zone(),
                self.asm.modifiable_input_graph(),
            );
            analyzer.run();
            self.next.analyze();
        }
    }

    impl<Next: NextTrait> Deref for PretenuringPropagationReducer<Next> {
        type Target = Assembler;

        fn deref(&self) -> &Self::Target {
            &self.asm
        }
    }

    impl<Next: NextTrait> DerefMut for PretenuringPropagationReducer<Next> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.asm
        }
    }

    impl<Next: NextTrait> NextTrait for PretenuringPropagationReducer<Next> {
        fn analyze(&mut self) {
            self.analyze();
        }
    }

    pub struct StoreOp {
        base_: OpIndex,
        value_: OpIndex,
    }

    impl StoreOp {
        pub fn new(base_: OpIndex, value_: OpIndex) -> Self {
            StoreOp { base_, value_ }
        }

        pub fn base(&self) -> OpIndex {
            self.base_
        }

        pub fn value(&self) -> OpIndex {
            self.value_
        }
    }

    pub trait TurboshaftReducerBoilerplate {
        fn analyze(&mut self);
    }

    impl TurboshaftReducerBoilerplate for PretenuringPropagationAnalyzer<'_> {
        fn analyze(&mut self) {
            self.run();
        }
    }
}

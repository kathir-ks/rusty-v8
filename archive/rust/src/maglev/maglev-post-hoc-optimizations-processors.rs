// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_post_hoc_optimizations_processors {
    use std::collections::HashSet;
    use std::any::Any;

    // Placeholder for v8::internal namespace and its contents
    pub mod internal {
        pub mod maglev {
            use std::cell::RefCell;
            use std::rc::Rc;
            use std::collections::HashMap;

            pub struct MaglevGraphBuilder {} // Placeholder
            impl MaglevGraphBuilder {
                pub fn zone(&self) -> &Zone {
                    unimplemented!()
                }
                pub fn compilation_unit(&self) -> &MaglevCompilationInfo {
                    unimplemented!()
                }
            }

            pub struct MaglevCompilationInfo {} // Placeholder
            impl MaglevCompilationInfo {
                pub fn feedback(&self) -> Feedback {
                    Feedback {}
                }

                pub fn has_graph_labeller(&self) -> bool {
                    unimplemented!()
                }
                pub fn graph_labeller(&self) -> *mut MaglevGraphLabeller {
                    unimplemented!()
                }
            }

            #[derive(Clone, Copy)]
            pub struct Feedback {}
            impl Feedback {
                pub fn was_once_deoptimized(&self) -> bool {
                    false // Placeholder
                }
            }
            
            pub struct MaglevGraphPrinter {} // Placeholder

            pub struct MaglevGraphProcessor {} // Placeholder

            pub struct Graph {
                allocations_escape_map_: RefCell<HashMap<*mut InlinedAllocation, SmallAllocationVector>>
            } // Placeholder
            impl Graph {
                pub fn allocations_escape_map(&self) -> std::cell::RefMut<'_, HashMap<*mut InlinedAllocation, SmallAllocationVector>> {
                    self.allocations_escape_map_.borrow_mut()
                }
            }
            
            pub type SmallAllocationVector = Vec<*mut InlinedAllocation>;

            pub struct MaglevInterpreterFrameState {} // Placeholder

            pub struct Zone {} // Placeholder

            pub struct NodeBase {} // Placeholder
            impl NodeBase {
                pub fn opcode_of<T>() -> Opcode {
                    Opcode::Nop
                }
            }
            
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum Opcode {
                Nop,
                LoadTaggedFieldForContextSlot,
                LoadTaggedFieldForProperty,
                StringLength,
                LoadTypedArrayLength,
                CheckMaps,
                Dead,
                AllocationBlock,
                InlinedAllocation,
                ArgumentsElements,
            }

            pub trait IsNode {
                fn as_node(&self) -> &Node;
            }

            pub struct Node {
                properties: NodeProperties,
                uses: usize,
                unused_inputs_were_visited: bool,
                owner: *mut BasicBlock,
                inputs: Vec<Input>,
                opcode: Opcode
            } // Placeholder
            impl Node {
                pub fn is_used(&self) -> bool {
                    self.uses > 0
                }
                pub fn unused_inputs_were_visited(&self) -> bool {
                    self.unused_inputs_were_visited
                }
                pub fn remove_use(&mut self) {
                    self.uses -= 1;
                }
                pub fn mark_unused_inputs_visited(&mut self) {
                    self.unused_inputs_were_visited = true;
                }
                pub fn owner(&self) -> *mut BasicBlock {
                    self.owner
                }
                pub fn input_count(&self) -> usize {
                    self.inputs.len()
                }
                pub fn input(&self, index: usize) -> &Input {
                    &self.inputs[index]
                }
                pub fn properties(&self) -> &NodeProperties {
                    &self.properties
                }

                pub fn properties_mut(&mut self) -> &mut NodeProperties {
                    &mut self.properties
                }

                pub fn opcode(&self) -> Opcode {
                    self.opcode
                }

                pub fn try_cast<T: IsNode>(&self) -> Option<&T> {
                    if self.opcode() == T::as_node(unsafe { std::mem::transmute::<&T, &Node>(&*(std::ptr::null::<T>() as *const T)) }).opcode() {
                        unsafe { Some(&*(self as *const Node as *const T)) }
                    } else {
                        None
                    }
                }
                
            }
            impl<'a> IntoIterator for &'a Node {
                type Item = &'a Input;
                type IntoIter = std::slice::Iter<'a, Input>;
            
                fn into_iter(self) -> Self::IntoIter {
                    self.inputs.iter()
                }
            }

            pub struct Input {
                node: *mut ValueNode
            }
            impl Input {
                pub fn node(&self) -> *mut ValueNode {
                    self.node
                }
            }

            pub struct ValueNode {
                node: Node
            }
            impl ValueNode {
                pub fn owner(&self) -> *mut BasicBlock {
                    self.node.owner()
                }

                pub fn is<T>(&self) -> bool {
                    self.node.opcode() == T::as_node(unsafe { std::mem::transmute::<&T, &Node>(&*(std::ptr::null::<T>() as *const T)) }).opcode()
                }
            }

            pub struct NodeProperties {
                can_eager_deopt: bool,
                can_lazy_deopt: bool,
                is_required_when_unused: bool,
            }
            impl NodeProperties {
                pub fn can_eager_deopt(&self) -> bool {
                    self.can_eager_deopt
                }
                pub fn can_lazy_deopt(&self) -> bool {
                    self.can_lazy_deopt
                }
                pub fn is_required_when_unused(&self) -> bool {
                    self.is_required_when_unused
                }
            }

            pub struct Phi {
                node: ValueNode,
                is_loop_phi: bool,
                merge_state: *mut MergeState,
            }
            impl Phi: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl Phi {
                pub fn is_loop_phi(&self) -> bool {
                    self.is_loop_phi
                }
                pub fn merge_state(&self) -> *mut MergeState {
                    self.merge_state
                }
            }

            pub struct BasicBlock {
                is_loop: bool,
                state: *mut MergeState,
                predecessors: Vec<*mut BasicBlock>,
                successors: Vec<*mut BasicBlock>,
                control_node: *mut Node
            } // Placeholder
            impl BasicBlock {
                pub fn is_loop(&self) -> bool {
                    self.is_loop
                }
                pub fn state(&self) -> *mut MergeState {
                    self.state
                }
                pub fn predecessor_count(&self) -> usize {
                    self.predecessors.len()
                }
                pub fn predecessor_at(&self, index: usize) -> *mut BasicBlock {
                    self.predecessors[index]
                }
                pub fn successors(&self) -> &Vec<*mut BasicBlock> {
                    &self.successors
                }
                pub fn control_node(&self) -> *mut Node {
                    self.control_node
                }
            }

            pub struct MergeState {} // Placeholder

            pub struct LoopEffects {
                pub may_have_aliasing_contexts: bool,
                pub unstable_aspects_cleared: bool,
                pub context_slot_written: HashSet<std::tuple::Tuple2<*mut ValueNode, usize>>,
                pub keys_cleared: HashSet<KnownNodeAspects::LoadedPropertyMapKey>,
                pub objects_written: HashSet<*mut ValueNode>
            } // Placeholder
            
            pub mod KnownNodeAspects {
                #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
                pub enum LoadedPropertyMapKey {
                    StringLength,
                    TypedArrayLength
                }
            }
            
            pub struct CheckpointedJump {}
            impl CheckpointedJump {
                pub fn eager_deopt_info(&self) -> *mut EagerDeoptInfo {
                    unimplemented!()
                }
            }

            pub struct EagerDeoptInfo {}
            impl EagerDeoptInfo {
                pub fn top_frame(&self) -> i32 {
                    unimplemented!()
                }
                pub fn feedback_to_update(&self) -> i32 {
                    unimplemented!()
                }
            }

            pub struct LoadTaggedFieldForContextSlot {
                node: ValueNode,
                offset: usize
            }
            impl LoadTaggedFieldForContextSlot: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl LoadTaggedFieldForContextSlot {
                pub fn object_input(&self) -> &Input {
                    &self.node.node.inputs[0] // Assuming object is the first input
                }
                pub fn offset(&self) -> usize {
                    self.offset
                }
            }

            pub struct LoadTaggedFieldForProperty {
                node: ValueNode,
                name: KnownNodeAspects::LoadedPropertyMapKey
            }
            impl LoadTaggedFieldForProperty: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl LoadTaggedFieldForProperty {
                pub fn object_input(&self) -> &Input {
                    &self.node.node.inputs[0] // Assuming object is the first input
                }
                pub fn name(&self) -> KnownNodeAspects::LoadedPropertyMapKey {
                    self.name
                }
            }

            pub struct StringLength {
                node: ValueNode
            }
            impl StringLength: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl StringLength {
                pub fn object_input(&self) -> &Input {
                    &self.node.node.inputs[0] // Assuming object is the first input
                }
            }

            pub struct LoadTypedArrayLength {
                node: ValueNode
            }
            impl LoadTypedArrayLength: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl LoadTypedArrayLength {
                pub fn receiver_input(&self) -> &Input {
                    &self.node.node.inputs[0] // Assuming receiver is the first input
                }
            }

            pub struct CheckMaps {
                node: ValueNode
            }
            impl CheckMaps: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl CheckMaps {
                pub fn receiver_input(&self) -> &Input {
                    &self.node.node.inputs[0] // Assuming receiver is the first input
                }
                pub fn SetEagerDeoptInfo(&mut self, zone: &Zone, top_frame: i32, feedback_to_update: i32) {
                    // Placeholder: Implement eager deopt info setting
                    unimplemented!()
                }
                pub fn eager_deopt_info(&self) -> *mut EagerDeoptInfo {
                    unimplemented!()
                }
            }

            pub struct InlinedAllocation {
                node: ValueNode,
                escaped: bool,
                elided: bool,
                analysed: bool,
                offset: usize,
                size: usize
            }
            impl InlinedAllocation: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl InlinedAllocation {
                pub fn HasBeenAnalysed(&self) -> bool {
                    self.analysed
                }
                pub fn HasEscaped(&self) -> bool {
                    self.escaped
                }
                pub fn SetEscaped(&mut self) {
                    self.escaped = true;
                    self.analysed = true;
                }
                pub fn IsEscaping(&self) -> bool {
                    self.escaped
                }
                pub fn SetElided(&mut self) {
                    self.elided = true;
                    self.analysed = true;
                }
                pub fn HasBeenElided(&self) -> bool {
                    self.elided
                }
                pub fn set_offset(&mut self, offset: usize) {
                    self.offset = offset;
                }
                pub fn size(&self) -> usize {
                    self.size
                }
            }

            pub struct AllocationBlock {
                node: ValueNode,
                allocation_list: Vec<*mut InlinedAllocation>,
                size: usize
            }
            impl AllocationBlock: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }
            impl AllocationBlock {
                pub fn allocation_list(&self) -> &Vec<*mut InlinedAllocation> {
                    &self.allocation_list
                }
                pub fn set_size(&mut self, size: usize) {
                    self.size = size;
                }
            }

            pub struct ArgumentsElements {
                node: ValueNode
            }
            impl ArgumentsElements: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }

            pub struct Dead {
                node: ValueNode
            }
            impl Dead: IsNode {
                fn as_node(&self) -> &Node {
                    &self.node.node
                }
            }

            pub struct MaglevGraphLabeller {} // Placeholder
            pub fn PrintNodeLabel(labeller: *mut MaglevGraphLabeller, node: *mut impl IsNode) -> String {
                // Placeholder: Implement node label printing
                unimplemented!()
            }

            #[derive(Debug, PartialEq, Eq)]
            pub enum BlockProcessResult {
                kContinue,
                kSkip,
                kSkipBlock,
            }

            #[derive(Debug, PartialEq, Eq)]
            pub enum ProcessResult {
                kContinue,
                kHoist,
                kRemove,
                kSkipBlock,
            }

            pub struct ProcessingState {} // Placeholder

            // Optimizations involving loops which cannot be done at graph building time.
            // Currently mainly loop invariant code motion.
            pub struct LoopOptimizationProcessor<'a> {
                zone: &'a Zone,
                current_block: *mut BasicBlock,
                loop_effects: *mut LoopEffects,
                was_deoptimized: bool,
            }

            impl<'a> LoopOptimizationProcessor<'a> {
                pub fn new(builder: &'a MaglevGraphBuilder) -> Self {
                    LoopOptimizationProcessor {
                        zone: builder.zone(),
                        current_block: std::ptr::null_mut(),
                        loop_effects: std::ptr::null_mut(),
                        was_deoptimized: builder.compilation_unit().feedback().was_once_deoptimized(),
                    }
                }

                pub fn pre_process_graph(&mut self, _graph: &mut Graph) {}
                pub fn post_phi_processing(&mut self) {}

                pub fn post_process_basic_block(&mut self, _block: *mut BasicBlock) {}
                pub fn pre_process_basic_block(&mut self, block: *mut BasicBlock) -> BlockProcessResult {
                    self.current_block = block;
                    let current_block_ref = unsafe { &*block };
                    if current_block_ref.is_loop() {
                        self.loop_effects = unsafe { (*current_block_ref.state()).loop_effects() };
                        if !self.loop_effects.is_null() {
                            return BlockProcessResult::kContinue;
                        }
                    } else {
                        // TODO(olivf): Some dominance analysis would allow us to keep loop
                        // effects longer than just the first block of the loop.
                        self.loop_effects = std::ptr::null_mut();
                    }
                    BlockProcessResult::kSkip
                }

                pub fn is_loop_phi(&self, input: *mut Node) -> bool {
                    let current_block_ref = unsafe { &*self.current_block };
                    debug_assert!(current_block_ref.is_loop());
                    let input_ref = unsafe { &*input };
                    if let Some(phi) = input_ref.try_cast::<Phi>() {
                        if phi.is_loop_phi() && phi.merge_state() == current_block_ref.state() {
                            return true;
                        }
                    }
                    false
                }

                pub fn can_hoist(&self, candidate: *mut Node) -> bool {
                    let candidate_ref = unsafe { &*candidate };
                    debug_assert_eq!(candidate_ref.input_count(), 1);
                    let current_block_ref = unsafe { &*self.current_block };
                    debug_assert!(current_block_ref.is_loop());
                    let input_node = unsafe { &*candidate_ref.inputs[0].node() };
                    if let Some(_) = input_node.try_cast::<Phi>() {
                      return false;
                    }

                    // For hoisting an instruction we need:
                    // * A unique loop entry block.
                    // * Inputs live before the loop (i.e., not defined inside the loop).
                    // * No hoisting over checks (done eagerly by clearing loop_effects).
                    // TODO(olivf): We should enforce loops having a unique entry block at graph
                    // building time.
                    if current_block_ref.predecessor_count() != 2 {
                        return false;
                    }
                    let loop_entry = unsafe { &*current_block_ref.predecessors[0] };
                    if loop_entry.successors().len() != 1 {
                        return false;
                    }

                    if self.is_constant_node(input_node.opcode()) {
                        return true;
                    }

                    input_node.owner() != self.current_block
                }

                fn is_constant_node(&self, opcode: Opcode) -> bool {
                    // Placeholder: Implement constant node check based on opcode.
                    // This requires a mapping of Opcodes to their properties.
                    false
                }

                pub fn process(&mut self, ltf: *mut LoadTaggedFieldForContextSlot, _state: &ProcessingState) -> ProcessResult {
                    if self.loop_effects.is_null() {
                        return ProcessResult::kContinue;
                    }
                    let ltf_ref = unsafe { &*ltf };
                    let object = ltf_ref.object_input().node();
                    if self.is_loop_phi(object) {
                        return ProcessResult::kContinue;
                    }
                    let key = std::tuple::Tuple2 {
                        field0: object,
                        field1: ltf_ref.offset(),
                    };
                    let loop_effects_ref = unsafe { &*self.loop_effects };
                    if !loop_effects_ref.may_have_aliasing_contexts
                        && !loop_effects_ref.unstable_aspects_cleared
                        && !loop_effects_ref.context_slot_written.contains(&key)
                        && self.can_hoist(ltf as *mut Node)
                    {
                        return ProcessResult::kHoist;
                    }
                    ProcessResult::kContinue
                }

                pub fn process(&mut self, ltf: *mut LoadTaggedFieldForProperty, _state: &ProcessingState) -> ProcessResult {
                    let ltf_ref = unsafe { &*ltf };
                    self.process_named_load(ltf as *mut Node, ltf_ref.object_input().node(), ltf_ref.name())
                }

                pub fn process(&mut self, len: *mut StringLength, _state: &ProcessingState) -> ProcessResult {
                    let len_ref = unsafe { &*len };
                    self.process_named_load(
                        len as *mut Node,
                        len_ref.object_input().node(),
                        KnownNodeAspects::LoadedPropertyMapKey::StringLength(),
                    )
                }

                pub fn process(&mut self, len: *mut LoadTypedArrayLength, _state: &ProcessingState) -> ProcessResult {
                    let len_ref = unsafe { &*len };
                    self.process_named_load(
                        len as *mut Node,
                        len_ref.receiver_input().node(),
                        KnownNodeAspects::LoadedPropertyMapKey::TypedArrayLength(),
                    )
                }

                pub fn process_named_load(
                    &mut self,
                    load: *mut Node,
                    object: *mut ValueNode,
                    name: KnownNodeAspects::LoadedPropertyMapKey,
                ) -> ProcessResult {
                    let load_ref = unsafe { &*load };
                    debug_assert!(!load_ref.properties().can_eager_deopt());
                    if self.loop_effects.is_null() {
                        return ProcessResult::kContinue;
                    }
                    if self.is_loop_phi(object as *mut Node) {
                        return ProcessResult::kContinue;
                    }

                    let loop_effects_ref = unsafe { &*self.loop_effects };
                    if !loop_effects_ref.unstable_aspects_cleared
                        && !loop_effects_ref.keys_cleared.contains(&name)
                        && !loop_effects_ref.objects_written.contains(&object)
                        && self.can_hoist(load)
                    {
                        return ProcessResult::kHoist;
                    }
                    ProcessResult::kContinue
                }

                pub fn process(&mut self, maps: *mut CheckMaps, _state: &ProcessingState) -> ProcessResult {
                    if self.loop_effects.is_null() {
                        return ProcessResult::kContinue;
                    }

                    // Hoisting a check out of a loop can cause it to trigger more than actually
                    // needed (i.e., if the loop is executed 0 times). This could lead to
                    // deoptimization loops as there is no feedback to learn here. Thus, we
                    // abort this optimization if the function deoptimized previously. Also, if
                    // hoisting of this check fails we need to abort (and not continue) to
                    // ensure we are not hoisting other instructions over it.
                    if self.was_deoptimized {
                        return ProcessResult::kSkipBlock;
                    }

                    let maps_ref = unsafe { &*maps };
                    let object = maps_ref.receiver_input().node();
                    if self.is_loop_phi(object) {
                        return ProcessResult::kSkipBlock;
                    }

                    if !unsafe { &*self.loop_effects }.unstable_aspects_cleared && self.can_hoist(maps as *mut Node) {
                        let current_block_ref = unsafe { &*self.current_block };
                        if let Some(j) = unsafe { &*current_block_ref.predecessors[0] }.control_node().try_cast::<CheckpointedJump>() {
                            let maps_mut = unsafe { &mut *maps };
                            maps_mut.SetEagerDeoptInfo(self.zone, unsafe { &*j }.eager_deopt_info().top_frame(), unsafe { &*maps }.eager_deopt_info().feedback_to_update());
                            return ProcessResult::kHoist;
                        }
                    }
                    ProcessResult::kSkipBlock
                }

                pub fn process<NodeT>(&mut self, node: *mut NodeT, _state: &ProcessingState) -> ProcessResult {
                    // Ensure we are not hoisting over checks.
                    let node_ref = unsafe { &*node }.as_node();
                    if node_ref.properties().can_eager_deopt() {
                        self.loop_effects = std::ptr::null_mut();
                        return ProcessResult::kSkipBlock;
                    }
                    ProcessResult::kContinue
                }

                pub fn post_process_graph(&mut self, _graph: &mut Graph) {}
            }

            //Placeholder for CanBeStoreToNonEscapedObject
            pub const fn CanBeStoreToNonEscapedObject<T>() -> bool {
                false
            }

            pub struct AnyUseMarkingProcessor {
                stores_to_allocations_: Vec<*mut Node>
            }

            impl AnyUseMarkingProcessor {
                pub fn new() -> Self {
                    AnyUseMarkingProcessor {
                        stores_to_allocations_: Vec::new()
                    }
                }

                pub fn pre_process_graph(&mut self, _graph: &mut Graph) {}
                pub fn post_process_basic_block(&mut self, _block: *mut BasicBlock) {}
                pub fn pre_process_basic_block(&mut self, _block: *mut BasicBlock) -> BlockProcessResult {
                    BlockProcessResult::kContinue
                }
                pub fn post_phi_processing(&mut self) {}

                pub fn process<NodeT>(&mut self, node: *mut NodeT, _state: &ProcessingState) -> ProcessResult {
                    let node_ref = unsafe { &*node }.as_node();

                    if node_ref.opcode() != Opcode::Dead {
                        if node_ref.opcode() == Node::opcode_of::<NodeT>() &&
                            (!node_ref.properties().is_required_when_unused() ||
                            node_ref.opcode() == Node::opcode_of::<ArgumentsElements>()) {
                            if !node_ref.is_used() {
                                if !node_ref.unused_inputs_were_visited() {
                                    self.drop_input_uses(unsafe { &mut *node }.as_node().inputs.as_mut_slice());
                                }
                                return ProcessResult::kRemove;
                            }
                        }

                        if CanBeStoreToNonEscapedObject::<NodeT>() {
                            let node_input = unsafe { &*node }.as_node().input(0).node();
                            let node_input_ref = unsafe { &*node_input };
                            if let Some(_) = node_input_ref.try_cast::<InlinedAllocation>() {
                                self.stores_to_allocations_.push(node as *mut Node);
                            }
                        }
                    }

                    ProcessResult::kContinue
                }

                #[cfg(debug_assertions)]
                pub fn process(&mut self, node: *mut Dead, _state: &ProcessingState) -> ProcessResult {
                    unreachable!();
                }

                #[cfg(not(debug_assertions))]
                pub fn process(&mut self, node: *mut Dead, _state: &ProcessingState) -> ProcessResult {
                    ProcessResult::kContinue
                }

                pub fn post_process_graph(&mut self, graph: *mut Graph) {
                    self.run_escape_analysis(graph);
                    self.drop_use_of_value_in_stores_to_captured_allocations();
                }

                fn escape_allocation(&self, graph: *mut Graph, alloc: *mut InlinedAllocation, deps: &mut SmallAllocationVector) {
                    let alloc_ref = unsafe { &mut *alloc };
                    if alloc_ref.HasBeenAnalysed() && alloc_ref.HasEscaped() {
                        return;
                    }
                    alloc_ref.SetEscaped();
                    for &dep in deps.iter() {
                        let mut dep_deps = unsafe { &mut *graph }.allocations_escape_map().remove(&(dep)).unwrap();
                        self.escape_allocation(graph, dep, &mut dep_deps);
                    }
                }

                fn verify_escape_analysis(&self, graph: *mut Graph) {
                    #[cfg(debug_assertions)]
                    unsafe {
                        let graph_ref = &*graph;
                        for (alloc_ptr, deps) in graph_ref.allocations_escape_map().borrow().iter() {
                            let alloc = &**alloc_ptr;
                            debug_assert!(alloc.HasBeenAnalysed());
                            if alloc.HasEscaped() {
                                for dep in deps {
                                    debug_assert!((*dep).HasEscaped());
                                }
                            }
                        }
                    }
                }

                fn run_escape_analysis(&mut self, graph: *mut Graph) {
                    let allocations_escape_map = unsafe { &mut *graph }.allocations_escape_map();

                    let keys: Vec<*mut InlinedAllocation> = allocations_escape_map.borrow().keys().cloned().collect();

                    for alloc_ptr in keys {
                        let alloc_ref = unsafe { &mut *alloc_ptr };
                        if alloc_ref.HasBeenAnalysed() {
                            continue;
                        }

                        // Check if all its uses are non escaping.
                        if alloc_ref.IsEscaping() {
                            // Escape this allocation and all its dependencies.
                            let mut deps = unsafe { &mut *graph }.allocations_escape_map().remove(&(alloc_ptr)).unwrap();
                            self.escape_allocation(graph, alloc_ptr, &mut deps);
                        } else {
                            // Try to capture the allocation. This can still change if a escaped
                            // allocation has this value as one of its dependencies.
                            alloc_ref.SetElided();
                        }
                    }
                    // Check that we've reached a fixpoint.
                    self.verify_escape_analysis(graph);
                }

                fn drop_use_of_value_in_stores_to_captured_allocations(&mut self) {
                    for &node in &self.stores_to_allocations_ {
                        let node_ref = unsafe { &*node };
                        let alloc_input = node_ref.input(0).node();
                        if let Some(alloc) = unsafe { &*alloc_input }.try_cast::<InlinedAllocation>() {
                            if alloc.HasBeenElided() {
                                // Skip first input.
                                let mut inputs = unsafe { &mut *node }.inputs.as_mut_slice();
                                for i in 1..inputs.len() {
                                    self.drop_input_uses(&mut [inputs[i]]);
                                }
                            }
                        }
                    }
                }

                fn drop_input_uses(&mut self, inputs: &mut [*mut Input]) {
                  for input in inputs {
                    if input.is_null() {
                      continue;
                    }
                    let input_ref = unsafe { &*(*input) };
                    let input_node = unsafe { &mut *(*input).node() };
                    if input_node.properties().is_required_when_unused() &&
                        !input_node.is::<ArgumentsElements>() {
                        continue;
                    }
                    input_node.remove_use();
                    if !input_node.is_used() && !input_node.unused_inputs_were_visited() {
                        let mut node_inputs = Vec::new();
                        for input in &mut unsafe { &mut *(*input).node() }.node.inputs {
                            node_inputs.push(input as *mut Input);
                        }
                        self.drop_input_uses(node_inputs.as_mut_slice());
                    }
                  }
                }
            }

            pub struct DeadNodeSweepingProcessor<'a> {
                labeller_: *mut MaglevGraphLabeller,
                compilation_info: &'a MaglevCompilationInfo,
            }

            impl<'a> DeadNodeSweepingProcessor<'a> {
                pub fn new(compilation_info: &'a MaglevCompilationInfo) -> Self {
                    let mut labeller_ptr = std::ptr::null_mut();
                    if compilation_info.has_graph_labeller() {
                      labeller_ptr = compilation_info.graph_labeller();
                    }
                    
                    DeadNodeSweepingProcessor {
                        labeller_: labeller_ptr,
                        compilation_info: compilation_info,
                    }
                }

                pub fn pre_process_graph(&mut self, _graph: &mut Graph) {}
                pub fn post_process_graph(&mut self, _graph: &mut Graph) {}
                pub fn post_process_basic_block(&mut self, _block: *mut BasicBlock) {}
                pub fn pre_process_basic_block(&mut self, _block: *mut BasicBlock) -> BlockProcessResult {
                    BlockProcessResult::kContinue
                }
                pub fn post_phi_processing(&mut self) {}

                pub fn process(&mut self, node: *mut AllocationBlock, _state: &ProcessingState) -> ProcessResult {
                    // Note: this need to be done before ValueLocationConstraintProcessor, since
                    // it access the allocation offsets.
                    let mut size = 0;
                    let node_ref = unsafe { &mut *node };
                    for alloc_ptr in node_ref.allocation_list() {
                        let alloc_ref = unsafe { &mut **alloc_ptr };
                        if alloc_ref.HasEscaped() {
                            alloc_ref.set_offset(size);
                            size += alloc_ref.size();
                        }
                    }
                    // ... and update its size.
                    node_ref.set_size(size);
                    // If size is zero, then none of the inlined allocations have escaped, we
                    // can remove the allocation block.
                    if size == 0 {
                        return ProcessResult::kRemove;
                    }
                    ProcessResult::kContinue
                }

                pub fn process(&mut self, node: *mut InlinedAllocation, _state: &ProcessingState) -> ProcessResult {
                    // Remove inlined allocation that became non-escaping.
                    let node_ref = unsafe { &*node };
                    if !node_ref.HasEscaped() {
                        if true {
                            println!(
                                "* Removing allocation node {}",
                                PrintNodeLabel(self.labeller_, node)
                            );
                        }
                        return ProcessResult::kRemove;
                    }
                    ProcessResult::kContinue
                }

                pub fn process<NodeT>(&mut self, node: *mut NodeT, _state: &ProcessingState) -> ProcessResult {
                    let node_ref = unsafe { &*node }.as_node();
                    if node_ref.opcode() != Opcode::Dead {
                        if node_ref.opcode() == Node::opcode_of::<NodeT>() &&
                            (!node_ref.properties().is_required_when_unused() ||
                            node_ref.opcode() == Node::opcode_of::<ArgumentsElements>()) {
                            if !node_ref.is_used() {
                                return ProcessResult::kRemove;
                            }

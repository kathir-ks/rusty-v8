// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_graph {
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo;
    use crate::compiler::heap_refs::ObjectRef;
    use crate::maglev::maglev_basic_block::BasicBlock;
    use crate::maglev::maglev_ir::*;

    use v8::Context;
    use v8::RootIndex;
    use v8::Address;
    use v8::JSHeapBroker;
    use v8::compiler::{
        HeapObjectRef,
        OptionalScopeInfoRef,
        ZoneRefMap
    };

    pub type BlockConstIterator<'a> = std::slice::Iter<'a, Box<BasicBlock>>;
    pub type BlockConstReverseIterator<'a> = std::slice::Iter<'a, Box<BasicBlock>>;

    pub struct MaglevCallSiteInfo {}

    #[derive(Default)]
    pub struct Graph {
        blocks: Vec<Box<BasicBlock>>,
        root: HashMap<RootIndex, Box<RootConstant>>,
        osr_values: Vec<Box<InitialValue>>,
        smi: HashMap<i32, Box<SmiConstant>>,
        tagged_index: HashMap<i32, Box<TaggedIndexConstant>>,
        int32: HashMap<i32, Box<Int32Constant>>,
        uint32: HashMap<u32, Box<Uint32Constant>>,
        float: HashMap<u64, Box<Float64Constant>>,
        external_references: HashMap<Address, Box<ExternalConstant>>,
        parameters: Vec<Box<InitialValue>>,
        inlineable_calls: Vec<Box<MaglevCallSiteInfo>>,
        allocations_escape_map: HashMap<Box<InlinedAllocation>, SmallAllocationVector>,
        allocations_elide_map: HashMap<Box<InlinedAllocation>, SmallAllocationVector>,
        register_inputs: RegList,
        constants: ZoneRefMap<ObjectRef, Box<Constant>>,
        trusted_constants: ZoneRefMap<HeapObjectRef, Box<TrustedConstant>>,
        inlined_functions: Vec<OptimizedCompilationInfo::InlinedFunctionHolder>,
        node_buffer: Vec<Box<Node>>,
        has_recursive_calls: bool,
        total_inlined_bytecode_size: i32,
        total_peeled_bytecode_size: i32,
        is_osr: bool,
        object_ids: u32,
        has_resumable_generator: bool,
        scope_infos: HashMap<*const ValueNode, OptionalScopeInfoRef>, // Using raw pointer for ValueNode key
        tagged_stack_slots_: u32,
        untagged_stack_slots_: u32,
        max_call_stack_args_: u32,
        max_deopted_stack_size_: u32,
    }

    // Running JS2, 99.99% of the cases, we have less than 2 dependencies.
    type SmallAllocationVector = SmallVec<InlinedAllocation>;
    type SmallVec<T> = Vec<Box<T>>; // Replace with SmallVec implementation if needed

    const K_MAX_UINT32: u32 = u32::MAX;

    impl Graph {
        pub fn new(is_osr: bool) -> Self {
            let mut graph = Graph {
                is_osr: is_osr,
                tagged_stack_slots_: K_MAX_UINT32,
                untagged_stack_slots_: K_MAX_UINT32,
                max_call_stack_args_: K_MAX_UINT32,
                max_deopted_stack_size_: K_MAX_UINT32,
                ..Default::default()
            };
            graph.node_buffer.reserve(32);
            graph
        }

        pub fn get_block(&self, i: usize) -> Option<&BasicBlock> {
            self.blocks.get(i).map(|b| &**b)
        }

        pub fn num_blocks(&self) -> usize {
            self.blocks.len()
        }

        pub fn blocks(&mut self) -> &mut Vec<Box<BasicBlock>> {
            &mut self.blocks
        }

        pub fn begin(&self) -> BlockConstIterator {
            self.blocks.iter()
        }

        pub fn end(&self) -> BlockConstIterator {
            self.blocks.iter()
        }

        pub fn rbegin(&self) -> BlockConstReverseIterator {
            self.blocks.iter()
        }

        pub fn rend(&self) -> BlockConstReverseIterator {
            self.blocks.iter()
        }

        pub fn last_block(&self) -> Option<&BasicBlock> {
            self.blocks.last().map(|b| &**b)
        }

        pub fn add(&mut self, block: Box<BasicBlock>) {
            self.blocks.push(block);
        }

        pub fn set_blocks(&mut self, blocks: Vec<Box<BasicBlock>>) {
            self.blocks = blocks;
        }

        pub fn tagged_stack_slots(&self) -> u32 {
            self.tagged_stack_slots_
        }
        pub fn untagged_stack_slots(&self) -> u32 {
            self.untagged_stack_slots_
        }
        pub fn max_call_stack_args(&self) -> u32 {
            self.max_call_stack_args_
        }
        pub fn max_deopted_stack_size(&self) -> u32 {
            self.max_deopted_stack_size_
        }
        pub fn set_tagged_stack_slots(&mut self, stack_slots: u32) {
            assert_eq!(K_MAX_UINT32, self.tagged_stack_slots_);
            assert_ne!(K_MAX_UINT32, stack_slots);
            self.tagged_stack_slots_ = stack_slots;
        }
        pub fn set_untagged_stack_slots(&mut self, stack_slots: u32) {
            assert_eq!(K_MAX_UINT32, self.untagged_stack_slots_);
            assert_ne!(K_MAX_UINT32, stack_slots);
            self.untagged_stack_slots_ = stack_slots;
        }
        pub fn set_max_call_stack_args(&mut self, stack_slots: u32) {
            assert_eq!(K_MAX_UINT32, self.max_call_stack_args_);
            assert_ne!(K_MAX_UINT32, stack_slots);
            self.max_call_stack_args_ = stack_slots;
        }
        pub fn set_max_deopted_stack_size(&mut self, size: u32) {
            assert_eq!(K_MAX_UINT32, self.max_deopted_stack_size_);
            assert_ne!(K_MAX_UINT32, size);
            self.max_deopted_stack_size_ = size;
        }

        pub fn total_inlined_bytecode_size(&self) -> i32 {
            self.total_inlined_bytecode_size
        }

        pub fn add_inlined_bytecode_size(&mut self, size: i32) {
            self.total_inlined_bytecode_size += size;
        }

        pub fn total_peeled_bytecode_size(&self) -> i32 {
            self.total_peeled_bytecode_size
        }

        pub fn add_peeled_bytecode_size(&mut self, size: i32) {
            self.total_peeled_bytecode_size += size;
        }

        pub fn root(&mut self) -> &mut HashMap<RootIndex, Box<RootConstant>> {
            &mut self.root
        }

        pub fn osr_values(&mut self) -> &mut Vec<Box<InitialValue>> {
            &mut self.osr_values
        }

        pub fn smi(&mut self) -> &mut HashMap<i32, Box<SmiConstant>> {
            &mut self.smi
        }

        pub fn tagged_index(&mut self) -> &mut HashMap<i32, Box<TaggedIndexConstant>> {
            &mut self.tagged_index
        }

        pub fn int32(&mut self) -> &mut HashMap<i32, Box<Int32Constant>> {
            &mut self.int32
        }

        pub fn uint32(&mut self) -> &mut HashMap<u32, Box<Uint32Constant>> {
            &mut self.uint32
        }

        pub fn float64(&mut self) -> &mut HashMap<u64, Box<Float64Constant>> {
            &mut self.float
        }

        pub fn external_references(&mut self) -> &mut HashMap<Address, Box<ExternalConstant>> {
            &mut self.external_references
        }

        pub fn parameters(&mut self) -> &mut Vec<Box<InitialValue>> {
            &mut self.parameters
        }

        pub fn inlineable_calls(&mut self) -> &mut Vec<Box<MaglevCallSiteInfo>> {
            &mut self.inlineable_calls
        }

        pub fn node_buffer(&mut self) -> &mut Vec<Box<Node>> {
            &mut self.node_buffer
        }

        pub fn allocations_escape_map(
            &mut self,
        ) -> &mut HashMap<Box<InlinedAllocation>, SmallAllocationVector> {
            &mut self.allocations_escape_map
        }
        pub fn allocations_elide_map(
            &mut self,
        ) -> &mut HashMap<Box<InlinedAllocation>, SmallAllocationVector> {
            &mut self.allocations_elide_map
        }

        pub fn register_inputs(&mut self) -> &mut RegList {
            &mut self.register_inputs
        }

        pub fn constants(&mut self) -> &mut ZoneRefMap<ObjectRef, Box<Constant>> {
            &mut self.constants
        }

        pub fn trusted_constants(&mut self) -> &mut ZoneRefMap<HeapObjectRef, Box<TrustedConstant>> {
            &mut self.trusted_constants
        }

        pub fn inlined_functions(
            &mut self,
        ) -> &mut Vec<OptimizedCompilationInfo::InlinedFunctionHolder> {
            &mut self.inlined_functions
        }

        pub fn has_recursive_calls(&self) -> bool {
            self.has_recursive_calls
        }

        pub fn set_has_recursive_calls(&mut self, value: bool) {
            self.has_recursive_calls = value;
        }

        pub fn is_osr(&self) -> bool {
            self.is_osr
        }

        pub fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {
            assert!(self.is_osr());
            if self.osr_values().is_empty() {
                return InitialValue::stack_slot(0);
            }
            let last_osr_value = self.osr_values().last().unwrap();
            last_osr_value.stack_slot() + 1
        }

        pub fn new_object_id(&mut self) -> u32 {
            self.object_ids += 1;
            self.object_ids -1
        }

        pub fn set_has_resumable_generator(&mut self) {
            self.has_resumable_generator = true;
        }

        pub fn has_resumable_generator(&self) -> bool {
            self.has_resumable_generator
        }

        fn try_get_scope_info_for_context_load(
            &mut self,
            context: *const ValueNode,
            offset: i32,
            broker: &mut JSHeapBroker,
        ) -> OptionalScopeInfoRef {
            let mut cur = self.try_get_scope_info(context, broker);
            if offset == Context::offset_of_element_at(Context::EXTENSION_INDEX) {
                return cur;
            }
            assert_eq!(offset, Context::offset_of_element_at(Context::PREVIOUS_INDEX));
            if cur.has_value() {
                let outer = cur.clone().unwrap().OuterScopeInfo(broker);
                let mut cur = outer;

                while !cur.HasContext() && cur.HasOuterScopeInfo() {
                    let outer = cur.OuterScopeInfo(broker);
                    cur = outer;
                }

                if cur.HasContext() {
                    return cur;
                }
            }
            OptionalScopeInfoRef::empty()
        }

        // Resolve the scope info of a context value.
        // An empty result means we don't statically know the context's scope.
        fn try_get_scope_info(
            &mut self,
            context: *const ValueNode,
            broker: &mut JSHeapBroker,
        ) -> OptionalScopeInfoRef {
            if let Some(res) = self.scope_infos.get(&context).cloned() {
                return res;
            }

            let mut res: OptionalScopeInfoRef = OptionalScopeInfoRef::empty();

            unsafe {
                if let Some(context_const) = (context as *const Node).cast::<Constant>().as_ref() {
                    if let Some(context) = context_const.object().AsContext() {
                        res = context.scope_info(broker);
                        assert!(res.HasContext());
                    }

                } else if let Some(load) = (context as *const Node).cast::<LoadTaggedFieldForContextSlot>().as_ref() {
                    res = self.try_get_scope_info_for_context_load(
                        load.input(0).node() as *const ValueNode,
                        load.offset(),
                        broker,
                    );
                } else if let Some(load_script) = (context as *const Node).cast::<LoadTaggedFieldForScriptContextSlot>().as_ref() {
                    res = self.try_get_scope_info_for_context_load(
                        load_script.input(0).node() as *const ValueNode,
                        load_script.offset(),
                        broker,
                    );
                } else if let Some(_initial_value) = (context as *const Node).cast::<InitialValue>().as_ref() {
                    // We should only fail to keep track of initial contexts originating from
                    // the OSR prequel.
                    // TODO(olivf): Keep track of contexts when analyzing OSR Prequel.
                    assert!(self.is_osr());
                } else {
                    // Any context created within a function must be registered in
                    // graph()->scope_infos(). Initial contexts must be registered before
                    // BuildBody. We don't track context in generators (yet) and around eval
                    // the bytecode compiler creates contexts by calling
                    // Runtime::kNewFunctionInfo directly.
                    let node = context as *const Node;
                    assert!((node as *const Phi).is_null()
                        || (node as *const GeneratorRestoreRegister).is_null()
                        || (node as *const RegisterInput).is_null()
                        || (node as *const CallRuntime).is_null());
                }
            }

            self.scope_infos.insert(context, res.clone());
            res
        }

        pub fn record_scope_info(&mut self, context: *const ValueNode, scope_info: OptionalScopeInfoRef) {
            self.scope_infos.insert(context, scope_info);
        }

        //TODO: Implement Zone
        pub fn zone(&self) -> () {
           ()
        }
    }
}

pub mod codegen {
    pub mod optimized_compilation_info {
        #[derive(Default, Clone)]
        pub struct OptimizedCompilationInfo {}

        impl OptimizedCompilationInfo {
            #[derive(Default, Clone)]
            pub struct InlinedFunctionHolder{}
        }
    }
}

pub mod compiler {
    pub mod heap_refs {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct ObjectRef {}
    }

    use std::collections::HashMap;
    use crate::maglev::maglev_graph::Constant;
    use v8::compiler::HeapObjectRef;

    pub type ZoneRefMap<K, V> = HashMap<K, V>;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HeapObjectRef {}

    pub use v8::compiler::OptionalScopeInfoRef;
}

pub mod maglev {
    pub mod maglev_basic_block {
        #[derive(Default)]
        pub struct BasicBlock {}
    }

    pub mod maglev_ir {
        use v8::compiler::OptionalScopeInfoRef;
        use v8::Address;
        use v8::RootIndex;
        use v8::{Context};
        use crate::compiler::{HeapObjectRef, ObjectRef, ZoneRefMap};

        #[derive(Default, Clone)]
        pub struct NodeInput {
            node: *const Node,
        }

        impl NodeInput {
            pub fn node(&self) -> *const Node {
                self.node
            }
        }

        #[derive(Default)]
        pub struct Node {}

        impl Node {
            pub fn is<T>(&self) -> bool {
                std::any::TypeId::of::<Self>() == std::any::TypeId::of::<T>()
            }
            pub fn try_cast<T>(&self) -> Option<&T> {
                if self.is::<T>() {
                   unsafe { Some(&*(self as *const Self as *const T)) }
                } else {
                    None
                }
            }
        }
        pub struct ValueNode {
            node: Node
        }

        impl ValueNode {
            pub fn try_cast<T>(&self) -> Option<&T> {
                if self.node.is::<T>() {
                   unsafe { Some(&*(self as *const Self as *const T)) }
                } else {
                    None
                }
            }
            pub fn is<T>(&self) -> bool {
                self.node.is::<T>()
            }
        }
        impl Default for ValueNode {
            fn default() -> Self {
                ValueNode{
                    node: Node::default()
                }
            }
        }

        #[derive(Default)]
        pub struct Constant {
            object: ObjectRef
        }
        impl Constant {
            pub fn object(&self) -> ObjectRef{
                self.object
            }
        }

        #[derive(Default)]
        pub struct TrustedConstant {
            heap_object: HeapObjectRef
        }

        #[derive(Default)]
        pub struct RootConstant {}

        #[derive(Default)]
        pub struct InitialValue {}

        impl InitialValue {
            pub fn stack_slot(slot: u32) -> u32 {
                slot
            }
            pub fn stack_slot(&self) -> u32 {
                0
            }
        }

        #[derive(Default)]
        pub struct SmiConstant {}

        #[derive(Default)]
        pub struct TaggedIndexConstant {}

        #[derive(Default)]
        pub struct Int32Constant {}

        #[derive(Default)]
        pub struct Uint32Constant {}

        #[derive(Default)]
        pub struct Float64Constant {}

        #[derive(Default)]
        pub struct ExternalConstant {}

        #[derive(Default)]
        pub struct InlinedAllocation {}

        #[derive(Default)]
        pub struct Phi {}

        #[derive(Default)]
        pub struct GeneratorRestoreRegister {}

        #[derive(Default)]
        pub struct RegisterInput {}

        #[derive(Default)]
        pub struct CallRuntime {}

        #[derive(Default)]
        pub struct LoadTaggedFieldForContextSlot {
            input: [NodeInput; 1],
            offset_: i32,
        }

        impl LoadTaggedFieldForContextSlot {
             pub fn input(&self, i: usize) -> &NodeInput {
                &self.input[i]
             }
             pub fn offset(&self) -> i32 {
                self.offset_
             }
        }

        #[derive(Default)]
        pub struct LoadTaggedFieldForScriptContextSlot {
            input: [NodeInput; 1],
            offset_: i32,
        }

        impl LoadTaggedFieldForScriptContextSlot {
            pub fn input(&self, i: usize) -> &NodeInput {
                &self.input[i]
             }
             pub fn offset(&self) -> i32 {
                self.offset_
             }
        }
    }
}

pub mod v8 {
    pub mod compiler {
        #[derive(Clone)]
        pub struct OptionalScopeInfoRef {
            has_value: bool,
        }

        impl OptionalScopeInfoRef {
            pub fn empty() -> Self {
                OptionalScopeInfoRef { has_value: false }
            }

            pub fn has_value(&self) -> bool {
                self.has_value
            }

             pub fn OuterScopeInfo(&self, _broker: &mut JSHeapBroker) -> OptionalScopeInfoRef {
                OptionalScopeInfoRef::empty()
             }

            pub fn HasOuterScopeInfo(&self) -> bool {
                false
            }

            pub fn HasContext(&self) -> bool {
                false
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Address {}

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct RootIndex {}

    pub struct Context {}

    impl Context {
        pub const EXTENSION_INDEX: i32 = 0;
        pub const PREVIOUS_INDEX: i32 = 1;
        pub fn offset_of_element_at(index: i32) -> i32 {
            index
        }
    }

    pub struct JSHeapBroker {}
}

type RegList = HashSet<i32>;
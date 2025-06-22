// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod instruction_selector_adapter {
    use std::{
        cell::RefCell,
        marker::PhantomData,
        num::TryFromIntError,
        ops::{Deref, DerefMut},
        rc::Rc,
    };

    //use crate::codegen::machine_type::MachineRepresentation; // Assuming this is defined elsewhere
    //use crate::compiler::backend::instruction::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::graph::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::operation_matcher::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::operations::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::opmasks::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::representations::*; // Assuming this is defined elsewhere
    //use crate::compiler::turboshaft::use_map::*; // Assuming this is defined elsewhere

    pub struct Graph {} // Dummy Graph type, replace with the real one.
    pub struct OpIndex {
        id: u32,
    } // Dummy OpIndex type, replace with the real one.

    impl OpIndex {
        pub fn id(&self) -> u32 {
            self.id
        }
        pub fn valid(&self) -> bool {
            true // Placeholder
        }
    }

    pub struct OptionalOpIndex {
        index: Option<OpIndex>,
    }

    impl OptionalOpIndex {
        pub fn value(&self) -> OpIndex {
            self.index.clone().unwrap()
        }

        pub fn value_or_invalid(&self) -> OpIndex {
            self.index.clone().unwrap() // Placeholder
        }
    }

    pub enum Opcode {
        kLoad,
        kSimd128LoadTransform,
        kSimd256LoadTransform,
        // other opcodes
    }

    pub struct Operation {
        pub opcode: Opcode,
        pub input_count: usize,
        inputs: Vec<OpIndex>,
        pub saturated_use_count: SaturatedUseCount,
    }

    impl Operation {
        pub fn input(&self, index: usize) -> OpIndex {
            self.inputs[index].clone()
        }

        pub fn inputs(&self) -> &[OpIndex] {
            &self.inputs
        }

        pub fn is<T>(&self) -> bool {
            false // Placeholder
        }

        pub fn try_cast<T>(&self) -> Option<&Self> {
            Some(self) // Placeholder
        }

        pub fn cast<T>(&self) -> &Self {
            self // Placeholder
        }
    }

    pub struct SaturatedUseCount {
        count: usize,
        saturated: bool,
    }

    impl SaturatedUseCount {
        pub fn get(&self) -> usize {
            self.count
        }

        pub fn is_one(&self) -> bool {
            self.count == 1
        }

        pub fn is_saturated(&self) -> bool {
            self.saturated
        }
    }

    pub struct LoadOp {
        pub kind: LoadOpKind,
        pub machine_type: LoadRepresentation,
        pub loaded_rep: MemoryRepresentation,
        pub result_rep: RegisterRepresentation,
        pub offset: i32,
        pub element_size_log2: u8,
        pub base_index: Option<(OpIndex, OpIndex)>,
        pub base_op_index: OpIndex,
    }

    impl LoadOp {
        pub fn base(&self) -> OpIndex {
            self.base_op_index.clone()
        }

        pub fn index(&self) -> OptionalOpIndex {
            OptionalOpIndex { index: None } // Placeholder
        }
    }

    pub struct LoadOpKind {
        pub with_trap_handler: bool,
        pub trap_on_null: bool,
        pub is_atomic: bool,
        pub tagged_base: bool,
    }

    pub struct Simd128LoadTransformOp {}
    pub struct Simd256LoadTransformOp {}

    pub struct StoreOp {
        pub stored_rep: MemoryRepresentation,
        pub kind: StoreOpKind,
        pub offset: i32,
        pub element_size_log2: u8,
        base_index: Option<(OpIndex, OpIndex)>,
        base_op_index: OpIndex,
        value_op_index: OpIndex,
        indirect_pointer_tag: u32,
    }

    impl StoreOp {
        pub fn base(&self) -> OpIndex {
            self.base_op_index.clone()
        }

        pub fn index(&self) -> OptionalOpIndex {
            OptionalOpIndex { index: None }
        }

        pub fn value(&self) -> OpIndex {
            self.value_op_index.clone()
        }
    }

    pub struct StoreOpKind {
        pub with_trap_handler: bool,
        pub trap_on_null: bool,
        pub is_atomic: bool,
        pub tagged_base: bool,
    }

    pub struct AtomicRMWOp {}

    pub struct AtomicWord32PairOp {}

    pub struct Simd128ShuffleOp {
        pub input_count: i32,
        shuffle: [u8; 16],
        input_op_indices: [OpIndex; 2],
    }

    impl Simd128ShuffleOp {
        pub fn input(&self, index: usize) -> OpIndex {
            self.input_op_indices[index].clone()
        }
    }

    pub struct Block {
        index: BlockIndex,
    }

    impl Block {
        pub fn index(&self) -> &BlockIndex {
            &self.index
        }

        pub fn is_loop(&self) -> bool {
            false // Placeholder
        }
    }

    pub struct BlockIndex {
        id: u32,
    }

    impl BlockIndex {
        pub fn id(&self) -> u32 {
            self.id
        }
    }

    pub struct PhiOp {}

    pub struct RetainOp {}

    pub struct ConstantOp {
        pub kind: ConstantOpKind,
    }

    pub enum ConstantOpKind {
        kHeapObject,
        kExternal,
        kRelocatableWasmCall,
        kRelocatableWasmStubCall,
    }

    pub struct FrameStateOp {}

    #[derive(Clone, Copy)]
    pub enum MachineRepresentation {
        Word32,
        Word64,
        Float64,
    } // Dummy enum, replace with the real one.

    pub struct MemoryRepresentation {
        // Fields for MemoryRepresentation
    }

    impl MemoryRepresentation {
        pub fn to_machine_type(&self) -> MachineRepresentation {
            // Placeholder implementation
            MachineRepresentation::Word32
        }
    }

    pub struct RegisterRepresentation {
        // Fields for RegisterRepresentation
    }

    pub struct LoadRepresentation {
        representation: MachineRepresentation,
    }

    impl LoadRepresentation {
        fn representation(&self) -> MachineRepresentation {
            self.representation
        }
    }

    pub enum AtomicMemoryOrder {
        kSeqCst,
    }

    pub enum MemoryAccessKind {
        kNormal,
        kProtectedByTrapHandler,
    }

    pub enum IndirectPointerTag {
        // Define enum variants here
    }

    pub struct CallDescriptor {}

    pub struct TSCallDescriptor {}

    pub struct RpoNumber {
        number: usize,
    }

    impl RpoNumber {
        pub fn from_int(number: usize) -> Self {
            RpoNumber { number }
        }
    }

    pub trait OperationMatcher {
        fn graph(&self) -> &Graph;
    }

    pub struct TurboshaftAdapter {
        graph_: RefCell<Box<Graph>>,
    }

    impl TurboshaftAdapter {
        pub const IS_TURBOFAN: bool = false;
        pub const IS_TURBOSHAFT: bool = true;
        pub const ALLOWS_IMPLICIT_WORD64_TO_WORD32_TRUNCATION: bool = true;

        pub fn new(graph: Graph) -> Self {
            TurboshaftAdapter {
                graph_: RefCell::new(Box::new(graph)),
            }
        }

        pub fn call_view(&self, node: OpIndex) -> CallView {
            CallView {
                graph: &*self.graph_.borrow(),
                node_: node,
                call_op_: None,
                tail_call_op_: None,
            }
        }

        pub fn load_view(&self, node: OpIndex) -> LoadView {
            assert!(self.is_load(node));
            LoadView {
                graph: &*self.graph_.borrow(),
                node_: node,
                load_: None,
                load_transform_: None,
                load_transform256_: None,
            }
        }

        pub fn store_view(&self, node: OpIndex) -> StoreView {
            StoreView {
                graph: &*self.graph_.borrow(),
                node_: node,
                op_: None,
            }
        }

        pub fn atomic_rmw_view(&self, node: OpIndex) -> AtomicRMWView {
            AtomicRMWView {
                graph: &*self.graph_.borrow(),
                node_: node,
                op_: None,
            }
        }

        pub fn word32_atomic_pair_store_view(&self, node: OpIndex) -> Word32AtomicPairStoreView {
            Word32AtomicPairStoreView {
                graph: &*self.graph_.borrow(),
                store_: AtomicWord32PairOp {},
            }
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn simd_shuffle_view(&self, node: OpIndex) -> SimdShuffleView {
            SimdShuffleView {
                graph: &*self.graph_.borrow(),
                node_: node,
                input_mapping_: vec![],
                op128_: None,
            }
        }

        pub fn is_load(&self, node: OpIndex) -> bool {
            let graph = self.graph_.borrow();
            match graph.get(&node).opcode {
                Opcode::kLoad => true,
                Opcode::kSimd128LoadTransform => true,
                Opcode::kSimd256LoadTransform => true,
                _ => false,
            }
        }

        pub fn is_load_root_register(&self, node: OpIndex) -> bool {
            false // Placeholder
        }

        pub fn turboshaft_graph(&self) -> &Graph {
            &*self.graph_.borrow()
        }

        pub fn block(&self, schedule: &mut Graph, node: OpIndex) -> Block {
            // TODO(nicohartmann@): This might be too slow and we should consider
            // precomputing.
            let block_index = schedule.block_of(node);
            schedule.get_block(&block_index)
        }

        pub fn rpo_number(&self, block: &Block) -> RpoNumber {
            RpoNumber::from_int(block.index().id() as usize)
        }

        // Assuming `blocks_vector` is a method of the `Graph` to get the RPO order.
        pub fn rpo_order(&self, schedule: &mut Graph) -> Vec<Block> {
            schedule.blocks_vector()
        }

        pub fn is_loop_header(&self, block: &Block) -> bool {
            block.is_loop()
        }

        pub fn predecessor_count(&self, block: &Block) -> usize {
            block.predecessor_count()
        }

        pub fn predecessor_at(&self, block: &Block, index: usize) -> Block {
            block.predecessor_at(index)
        }

        pub fn nodes(&self, block: &Block) -> Vec<OpIndex> {
            self.turboshaft_graph().operation_indices(block)
        }

        pub fn is_phi(&self, node: OpIndex) -> bool {
            self.turboshaft_graph().get(&node).is::<PhiOp>()
        }

        pub fn phi_representation_of(&self, node: OpIndex) -> MachineRepresentation {
            assert!(self.is_phi(node));
            let phi = self.turboshaft_graph().get(&node).cast::<PhiOp>();
            phi.rep.machine_representation()
        }

        pub fn is_retain(&self, node: OpIndex) -> bool {
            self.turboshaft_graph().get(&node).is::<RetainOp>()
        }

        pub fn is_heap_constant(&self, node: OpIndex) -> bool {
            if let Some(constant) = self.turboshaft_graph().get(&node).try_cast::<ConstantOp>() {
                constant.kind == ConstantOpKind::kHeapObject
            } else {
                false
            }
        }

        pub fn is_external_constant(&self, node: OpIndex) -> bool {
            if let Some(constant) = self.turboshaft_graph().get(&node).try_cast::<ConstantOp>() {
                constant.kind == ConstantOpKind::kExternal
            } else {
                false
            }
        }

        pub fn is_relocatable_wasm_constant(&self, node: OpIndex) -> bool {
            if let Some(constant) = self.turboshaft_graph().get(&node).try_cast::<ConstantOp>() {
                matches!(
                    constant.kind,
                    ConstantOpKind::kRelocatableWasmCall | ConstantOpKind::kRelocatableWasmStubCall
                )
            } else {
                false
            }
        }

        pub fn is_load_or_load_immutable(&self, node: OpIndex) -> bool {
            self.turboshaft_graph().get(&node).opcode == Opcode::kLoad
        }

        pub fn is_protected_load(&self, node: OpIndex) -> bool {
            if self.turboshaft_graph().get(&node).opcode == Opcode::kSimd128LoadTransform {
                return true;
            }

            if self.turboshaft_graph().get(&node).opcode == Opcode::kSimd256LoadTransform {
                return true;
            }

            if !self.is_load_or_load_immutable(node) {
                return false;
            }

            let mut traps_on_null = false;
            self.load_view(node).is_protected(&mut traps_on_null)
        }

        pub fn value_input_count(&self, node: OpIndex) -> usize {
            self.turboshaft_graph().get(&node).input_count
        }

        pub fn input_at(&self, node: OpIndex, index: usize) -> OpIndex {
            self.turboshaft_graph().get(&node).input(index)
        }

        pub fn inputs(&self, node: OpIndex) -> &[OpIndex] {
            self.turboshaft_graph().get(&node).inputs()
        }

        pub fn opcode(&self, node: OpIndex) -> Opcode {
            self.turboshaft_graph().get(&node).opcode
        }

        pub fn is_exclusive_user_of(&self, user: OpIndex, value: OpIndex) -> bool {
            assert!(user.valid());
            assert!(value.valid());
            let value_op = self.turboshaft_graph().get(&value);
            let user_op = self.turboshaft_graph().get(&user);

            let use_count = user_op
                .inputs()
                .iter()
                .filter(|&input| *input == value)
                .count();

            if use_count == 0 {
                if !value_op.saturated_use_count.is_one() {
                    return false;
                }
                for input in user_op.inputs() {
                    let input_op = self.turboshaft_graph().get(input);
                    let indirect_use_count = input_op
                        .inputs()
                        .iter()
                        .filter(|&input| *input == value)
                        .count();
                    if indirect_use_count > 0 {
                        return input_op.saturated_use_count.is_one();
                    }
                }
                return false;
            }

            if value_op.is::<ProjectionOp>() {
                // Projections always have a Tuple use, but it shouldn't count as a use as
                // far as is_exclusive_user_of is concerned, since no instructions are
                // emitted for the TupleOp, which is just a Turboshaft "meta operation".
                // We thus increase the use_count by 1, to attribute the TupleOp use to
                // the current operation.
                // use_count += 1; // Intentionally removed because we don't have ProjectionOp.
            }

            //DCHECK_LE(use_count, graph_->Get(value).saturated_use_count.Get());
            value_op.saturated_use_count.get() == use_count && !value_op.saturated_use_count.is_saturated()
        }

        pub fn id(&self, node: OpIndex) -> u32 {
            node.id()
        }

        pub fn value(node: OptionalOpIndex) -> OpIndex {
            assert!(node.index.is_some());
            node.value()
        }

        pub fn block_terminator(&self, block: &Block) -> OpIndex {
            self.turboshaft_graph().previous_index(block.end())
        }

        pub fn parent_frame_state(&self, node: OpIndex) -> OpIndex {
            let frame_state = self.turboshaft_graph().get(&node).cast::<FrameStateOp>();
            frame_state.parent_frame_state()
        }

        pub fn is_required_when_unused(&self, node: OpIndex) -> bool {
            self.turboshaft_graph().get(&node).is_required_when_unused()
        }

        pub fn is_commutative(&self, node: OpIndex) -> bool {
            let op = self.turboshaft_graph().get(&node);
            if let Some(word_binop) = op.try_cast::<WordBinopOp>() {
                return WordBinopOp::is_commutative(word_binop.kind);
            } else if let Some(overflow_binop) = op.try_cast::<OverflowCheckedBinopOp>() {
                return OverflowCheckedBinopOp::is_commutative(overflow_binop.kind);
            } else if let Some(float_binop) = op.try_cast::<FloatBinopOp>() {
                return FloatBinopOp::is_commutative(float_binop.kind);
            } else if let Some(comparison) = op.try_cast::<ComparisonOp>() {
                return ComparisonOp::is_commutative(comparison.kind);
            }
            false
        }
    }

    impl OperationMatcher for TurboshaftAdapter {
        fn graph(&self) -> &Graph {
            &*self.graph_.borrow()
        }
    }

    // Dummy implementations to satisfy the compiler
    impl Graph {
        fn get(&self, node: &OpIndex) -> &Operation {
            unimplemented!()
        }

        fn block_of(&self, node: OpIndex) -> BlockIndex {
            unimplemented!()
        }

        fn get_block(&self, block_index: &BlockIndex) -> Block {
            unimplemented!()
        }

        fn blocks_vector(&self) -> Vec<Block> {
            unimplemented!()
        }

        fn operation_indices(&self, block: &Block) -> Vec<OpIndex> {
            unimplemented!()
        }

        fn previous_index(&self, end: OpIndex) -> OpIndex {
            unimplemented!()
        }
    }

    impl Block {
        fn end(&self) -> OpIndex {
            unimplemented!()
        }

        fn predecessor_count(&self) -> usize {
            unimplemented!()
        }

        fn predecessor_at(&self, index: usize) -> Block {
            unimplemented!()
        }
    }

    impl FrameStateOp {
        fn parent_frame_state(&self) -> OpIndex {
            unimplemented!()
        }
    }

    impl Operation {
        fn is_required_when_unused(&self) -> bool {
            unimplemented!()
        }
    }

    impl dyn OperationMatcher {
        pub fn is_load(&self, _node: OpIndex) -> bool {
            unimplemented!()
        }
    }

    impl WordBinopOp {
        pub fn is_commutative(_kind: i32) -> bool {
            unimplemented!()
        }
    }

    impl OverflowCheckedBinopOp {
        pub fn is_commutative(_kind: i32) -> bool {
            unimplemented!()
        }
    }

    impl FloatBinopOp {
        pub fn is_commutative(_kind: i32) -> bool {
            unimplemented!()
        }
    }

    impl ComparisonOp {
        pub fn is_commutative(_kind: i32) -> bool {
            unimplemented!()
        }
    }

    pub struct CallView<'a> {
        graph: &'a Graph,
        node_: OpIndex,
        call_op_: Option<&'a CallOp>,
        tail_call_op_: Option<&'a TailCallOp>,
    }

    impl<'a> CallView<'a> {
        pub fn return_count(&self) -> i32 {
            if let Some(call_op) = self.call_op_ {
                call_op.results_rep().len() as i32
            } else if let Some(tail_call_op) = self.tail_call_op_ {
                tail_call_op.outputs_rep().len() as i32
            } else {
                panic!("UNREACHABLE");
            }
        }

        pub fn callee(&self) -> OpIndex {
            if let Some(call_op) = self.call_op_ {
                call_op.callee().clone()
            } else if let Some(tail_call_op) = self.tail_call_op_ {
                tail_call_op.callee().clone()
            } else {
                panic!("UNREACHABLE");
            }
        }

        pub fn frame_state(&self) -> OpIndex {
            if let Some(call_op) = self.call_op_ {
                call_op.frame_state().unwrap().clone()
            } else {
                panic!("UNREACHABLE");
            }
        }

        pub fn arguments(&self) -> &[OpIndex] {
            if let Some(call_op) = self.call_op_ {
                call_op.arguments()
            } else if let Some(tail_call_op) = self.tail_call_op_ {
                tail_call_op.arguments()
            } else {
                panic!("UNREACHABLE");
            }
        }

        pub fn call_descriptor(&self) -> &CallDescriptor {
            if let Some(call_op) = self.call_op_ {
                call_op.descriptor.descriptor()
            } else if let Some(tail_call_op) = self.tail_call_op_ {
                tail_call_op.descriptor.descriptor()
            } else {
                panic!("UNREACHABLE");
            }
        }

        pub fn ts_call_descriptor(&self) -> &TSCallDescriptor {
            if let Some(call_op) = self.call_op_ {
                call_op.descriptor
            } else if let Some(tail_call_op) = self.tail_call_ {
                tail_call_op.descriptor
            } else {
                panic!("UNREACHABLE");
            }
        }
    }

    pub struct LoadView<'a> {
        graph: &'a Graph,
        node_: OpIndex,
        load_: Option<&'a LoadOp>,
        load_transform_: Option<&'a Simd128LoadTransformOp>,
        load_transform256_: Option<&'a Simd256LoadTransformOp>,
    }

    impl<'a> LoadView<'a> {
        pub fn loaded_rep(&self) -> LoadRepresentation {
            self.load_.as_ref().expect("load_").machine_type.clone()
        }

        pub fn ts_loaded_rep(&self) -> MemoryRepresentation {
            self.load_.as_ref().expect("load_").loaded_rep.clone()
        }

        pub fn ts_result_rep(&self) -> RegisterRepresentation {
            self.load_.as_ref().expect("load_").result_rep.clone()
        }

        pub fn is_protected(&self, traps_on_null: &mut bool) -> bool {
            if self.kind().with_trap_handler {
                if let Some(load) = self.load_ {
                    *traps_on_null = load.kind.trap_on_null;
                } else {
                    #[cfg(feature = "v8_enable_webassembly")]
                    {
                        #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                        {
                            assert!(
                                (self.load_transform_.is_some()
                                    && !self.load_transform_.as_ref().unwrap().load_kind.trap_on_null)
                                    || (self.load_transform256_.is_some()
                                        && !self
                                            .load_transform256_
                                            .as_ref()
                                            .unwrap()
                                            .load_kind
                                            .trap_on_null)
                            );
                        }
                        #[cfg(not(feature = "v8_enable_wasm_simd256_revec"))]
                        {
                            assert!(self.load_transform_.is_some());
                            assert!(!self.load_transform_.as_ref().unwrap().load_kind.trap_on_null);
                        }
                        *traps_on_null = false;
                    }
                }
                true
            } else {
                false
            }
        }

        pub fn is_atomic(&self) -> bool {
            self.kind().is_atomic
        }

        pub fn base(&self) -> OpIndex {
            if let Some(load) = self.load_ {
                load.base()
            } else {
                #[cfg(feature = "v8_enable_webassembly")]
                {
                    if self.load_transform_.is_some() {
                        return self.load_transform_.as_ref().unwrap().base();
                    }
                    #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                    {
                        if self.load_transform256_.is_some() {
                            return self.load_transform256_.as_ref().unwrap().base();
                        }
                    }
                }
                panic!("UNREACHABLE");
            }
        }

        pub fn index(&self) -> OpIndex {
            if let Some(load) = self.load_ {
                load.index().value_or_invalid()
            } else {
                #[cfg(feature = "v8_enable_webassembly")]
                {
                    if self.load_transform_.is_some() {
                        return OpIndex { id: 0 }; // Placeholder return self.load_transform_.as_ref().unwrap().index();
                    }
                    #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                    {
                        if self.load_transform256_.is_some() {
                            return OpIndex { id: 0 }; // Placeholder return self.load_transform256_.as_ref().unwrap().index();
                        }
                    }
                }
                panic!("UNREACHABLE");
            }
        }

        pub fn displacement(&self) -> i32 {
            if let Some(load) = self.load_ {
                let mut offset = load.offset;
                if load.kind.tagged_base {
                    //const K_HEAP_OBJECT_TAG: i32 = 1; // Assuming this is defined elsewhere
                    let k_heap_object_tag: i32 = 1;
                    assert!(offset >= std::i32::MIN + k_heap_object_tag);
                    offset -= k_heap_object_tag;
                }
                offset
            } else {
                #[cfg(feature = "v8_enable_webassembly")]
                {
                    if self.load_transform_.is_some() {
                        let offset = self.load_transform_.as_ref().unwrap().offset;
                        assert!(!self.load_transform_.as_ref().unwrap().load_kind.tagged_base);
                        return offset;
                    }
                    #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                    {
                        if self.load_transform256_.is_some() {
                            let offset = self.load_transform256_.as_ref().unwrap().offset;
                            assert!(!self
                                .load_transform256_
                                .as_ref()
                                .unwrap()
                                .load_kind
                                .tagged_base);
                            return offset;
                        }
                    }
                }
                panic!("UNREACHABLE");
            }
        }

        pub fn element_size_log2(&self) -> u8 {
            if let Some(load) = self.load_ {
                load.element_size_log2
            } else {
                #[cfg(feature = "v8_enable_webassembly")]
                {
                    if self.load_transform_.is_some() {
                        return 0;
                    }
                    #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                    {
                        if self.load_transform256_.is_some() {
                            return 0;
                        }
                    }
                }
                panic!("UNREACHABLE");
            }
        }

        fn kind(&self) -> LoadOpKind {
            if let Some(load) = self.load_ {
                load.kind.clone()
            } else {
                #[cfg(feature = "v8_enable_webassembly")]
                {
                    if self.load_transform_.is_some() {
                        return self.load_transform_.as_ref().unwrap().load_kind.clone();
                    }
                    #[cfg(feature = "v8_enable_wasm_simd256_revec")]
                    {
                        if self.load_transform256_.is_some() {
                            return self.load_transform256_.as_ref().unwrap().load_kind.clone();
                        }
                    }
                }
                panic!("UNREACHABLE");
            }
        }
    }

    pub struct StoreView<'a> {
        graph: &'a Graph,
        node_: OpIndex,
        op_: Option<&'a StoreOp>,
    }

    impl<'a> StoreView<'a> {
        pub fn stored_rep(&self) -> StoreRepresentation {
            let op = self.op_.as_ref().unwrap();
            StoreRepresentation {
                representation: op.stored_rep.to_machine_type(),
                write_barrier: op.write_barrier,
            }
        }

        pub fn ts_stored_rep(&self) -> MemoryRepresentation {
            self.op_.as_ref().unwrap().stored_rep.clone()
        }

        pub fn memory_order(&self) -> Option<AtomicMemoryOrder> {
            // TODO(nicohartmann@): Currently we don't support memory orders.
            if self.op_.as_ref().unwrap().kind.is_atomic {
                return Some(AtomicMemoryOrder::kSeqCst);
            }
            None
        }

        pub fn access_kind(&self) -> MemoryAccessKind {
            if self.op_.as_ref().unwrap().kind.with_trap_handler {
                MemoryAccessKind::kProtectedByTrapHandler
            } else {
                MemoryAccessKind::kNormal
            }
        }

        pub fn is_atomic(&self) -> bool {
            self.op_.as_ref().unwrap().kind.is_atomic
        }

        pub fn base(&self) -> OpIndex {
            self.op_.as_ref().unwrap().base()
        }

        pub fn index(&self) -> OptionalOpIndex {
            self.op_.as_ref().unwrap().index()
        }

        pub fn value(&self) -> OpIndex {
            self.op_.as_ref().unwrap().value()
        }

        pub fn indirect_pointer_tag(&self) -> IndirectPointerTag {
            unimplemented!()
        }

        pub fn displacement(&self) -> i32 {
            let op = self.op_.as_ref().unwrap();
            let mut offset = op.offset;
            if op.kind.tagged_base {
                //const K_HEAP_OBJECT_TAG: i32 = 1; // Assuming this is defined elsewhere
                let k_heap_object_tag: i32 = 1;
                assert!(offset >= std::i32::MIN + k_heap_object_tag);
                offset -= k_heap_object_tag;
            }
            offset
        }

        pub fn element_size_log2(&self) -> u8 {
            self.op_.as_ref().unwrap().element_size_log2
        }

        pub fn is_store_trap_on_null(&self) -> bool {
            let op = self.op_.as_ref().unwrap();
            op.kind.with_trap_handler && op.kind.trap_on_null
        }
    }

    pub struct AtomicRMWView<'a> {
        graph: &'a Graph,
        node_: OpIndex,
        op_: Option<&'a AtomicRMWOp>,
    }

    impl<'a> AtomicRMWView<'a> {
        pub fn base(&self) -> OpIndex {
            self.op_.as_ref().unwrap().base()
        }

        pub fn index(&self) -> OpIndex {
            self.
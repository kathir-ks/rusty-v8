// Converted from V8 C++ source files:
// Header: instruction-selector-adapter.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn size(&self) -> usize {
            self.data.len()
        }
    }

    pub fn count_if<T, F>(iter: impl IntoIterator<Item = T>, predicate: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        iter.into_iter().filter(|item| predicate(item)).count()
    }

    pub struct SmallVector<T, const N: usize> {
        data: Vec<T>,
    }
}

pub mod v8 {
    pub mod internal {
        pub mod compiler {
            pub mod turboshaft {
                use std::any::Any;

                pub enum Opcode {
                    kLoad,
                    kSimd128LoadTransform,
                    kSimd256LoadTransform,
                    kLoadRootRegister,
                    kPhi,
                    kRetain,
                    kCall,
                    kTailCall,
                    kConstant,
                    kStore,
                    kAtomicRMW,
                    kAtomicWord32Pair,
                    kSimd128Shuffle,
                    kFrameState,
                }
                pub struct Graph {
                    operations: Vec<Operation>,
                    blocks: Vec<Block>,
                    blocks_vector_: Vec<Block>,
                }

                impl Graph {
                    pub fn Get(&self, index: OpIndex) -> &Operation {
                        &self.operations[index.id() as usize]
                    }

                    pub fn GetMut(&mut self, index: OpIndex) -> &mut Operation {
                        &mut self.operations[index.id() as usize]
                    }

                    pub fn BlockOf(&self, _node: OpIndex) -> OpIndex {
                        OpIndex { id_: 1 }
                    }

                    pub fn OperationIndices(&self, block: &Block) -> base::iterator_range<Graph::OpIndexIterator> {
                        let start = block.start();
                        let end = block.end();
                        base::iterator_range {
                            begin: Graph::OpIndexIterator { current: start },
                            end: Graph::OpIndexIterator { current: end },
                        }
                    }

                    pub fn PreviousIndex(&self, index: OpIndex) -> OpIndex {
                        OpIndex { id_: index.id() - 1 }
                    }

                    pub fn blocks_vector(&self) -> &Vec<Block> {
                        &self.blocks_vector_
                    }

                    pub struct OpIndexIterator {
                        current: OpIndex,
                    }

                    impl Iterator for OpIndexIterator {
                        type Item = OpIndex;

                        fn next(&mut self) -> Option<Self::Item> {
                            if self.current.valid() {
                                let current = self.current;
                                self.current = OpIndex { id_: self.current.id() + 1 }; // Simplified increment
                                Some(current)
                            } else {
                                None
                            }
                        }
                    }
                }

                #[derive(Clone, Copy, Debug)]
                pub struct OpIndex {
                    id_: u32,
                }

                impl OpIndex {
                    pub fn valid(&self) -> bool {
                        self.id_ != u32::MAX
                    }

                    pub fn id(&self) -> u32 {
                        self.id_
                    }
                }

                #[derive(Debug)]
                pub struct Operation {
                    pub opcode: Opcode,
                    pub inputs: Vec<OpIndex>,
                    pub input_count: i32,
                    pub saturated_use_count: SaturatedUseCount,
                    pub data: Box<dyn Any>, // Generic data storage
                    pub is_required_when_unused: bool
                }

                impl Operation {
                    pub fn Is<T: 'static>(&self) -> bool {
                        self.data.is::<T>()
                    }

                    pub fn TryCast<T: 'static>(&self) -> Option<&T> {
                        self.data.downcast_ref::<T>()
                    }

                    pub fn Cast<T: 'static>(&self) -> &T {
                        self.data.downcast_ref::<T>().unwrap()
                    }

                    pub fn inputs(&self) -> &[OpIndex] {
                        &self.inputs
                    }

                    pub fn IsRequiredWhenUnused(&self) -> bool {
                        self.is_required_when_unused
                    }
                }

                #[derive(Debug, Default)]
                pub struct SaturatedUseCount {
                    count: u32,
                    saturated: bool,
                }

                impl SaturatedUseCount {
                    pub fn IsOne(&self) -> bool {
                        self.count == 1
                    }

                    pub fn Get(&self) -> u32 {
                        self.count
                    }

                    pub fn IsSaturated(&self) -> bool {
                        self.saturated
                    }
                }

                #[derive(Debug)]
                pub struct LoadOp {
                    pub base_: OpIndex,
                    pub index_: Option<OpIndex>,
                    pub offset: i32,
                    pub element_size_log2: u8,
                    pub machine_type_: LoadRepresentation,
                    pub loaded_rep: MemoryRepresentation,
                    pub result_rep: RegisterRepresentation,
                    pub kind: LoadOp::Kind,
                }

                impl LoadOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> Option<OpIndex> {
                        self.index_
                    }
                    pub fn machine_type(&self) -> LoadRepresentation {
                        self.machine_type_
                    }

                    pub struct Kind {
                        pub is_atomic: bool,
                        pub with_trap_handler: bool,
                        pub trap_on_null: bool,
                        pub tagged_base: bool,
                    }
                }

                #[derive(Debug)]
                pub struct Simd128LoadTransformOp {
                    pub base_: OpIndex,
                    pub index_: OpIndex,
                    pub offset: i32,
                    pub load_kind: LoadOp::Kind,
                }

                impl Simd128LoadTransformOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> OpIndex {
                        self.index_
                    }
                }

                #[derive(Debug)]
                pub struct Simd256LoadTransformOp {
                    pub base_: OpIndex,
                    pub index_: OpIndex,
                    pub offset: i32,
                    pub load_kind: LoadOp::Kind,
                }

                impl Simd256LoadTransformOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> OpIndex {
                        self.index_
                    }
                }

                #[derive(Debug)]
                pub struct StoreOp {
                    pub base_: OpIndex,
                    pub index_: OptionalOpIndex,
                    pub value_: OpIndex,
                    pub offset: i32,
                    pub element_size_log2: u8,
                    pub stored_rep: MemoryRepresentation,
                    pub kind: StoreOp::Kind,
                    pub write_barrier: WriteBarrierKind,
                    pub indirect_pointer_tag_: i32
                }

                impl StoreOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> OptionalOpIndex {
                        self.index_
                    }
                    pub fn value(&self) -> OpIndex {
                        self.value_
                    }
                    pub fn indirect_pointer_tag(&self) -> i32 {
                        self.indirect_pointer_tag_
                    }

                    pub struct Kind {
                        pub is_atomic: bool,
                        pub with_trap_handler: bool,
                        pub trap_on_null: bool,
                        pub tagged_base: bool,
                    }
                }

                #[derive(Debug)]
                pub struct AtomicRMWOp {
                    pub base_: OpIndex,
                    pub index_: OpIndex,
                    pub value_: OpIndex,
                    pub expected_: Option<OpIndex>,
                    pub bin_op: AtomicRMWOp::BinOp,
                }

                impl AtomicRMWOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> OpIndex {
                        self.index_
                    }
                    pub fn value(&self) -> OpIndex {
                        self.value_
                    }
                    pub fn expected(&self) -> Option<OpIndex> {
                        self.expected_
                    }

                    pub enum BinOp {
                        kCompareExchange,
                    }
                }

                #[derive(Debug)]
                pub struct AtomicWord32PairOp {
                    pub base_: OpIndex,
                    pub index_: OptionalOpIndex,
                    pub value_low_: OptionalOpIndex,
                    pub value_high_: OptionalOpIndex,
                }

                impl AtomicWord32PairOp {
                    pub fn base(&self) -> OpIndex {
                        self.base_
                    }
                    pub fn index(&self) -> OptionalOpIndex {
                        self.index_
                    }
                    pub fn value_low(&self) -> OptionalOpIndex {
                        self.value_low_
                    }
                    pub fn value_high(&self) -> OptionalOpIndex {
                        self.value_high_
                    }
                }

                #[derive(Debug)]
                pub struct Simd128ShuffleOp {
                    pub inputs_: [OpIndex; 2],
                    pub shuffle: [u8; 16],
                    pub input_count: i32
                }

                impl Simd128ShuffleOp {
                    pub fn input(&self, index: usize) -> OpIndex {
                        self.inputs_[index]
                    }
                }

                #[derive(Debug)]
                pub struct PhiOp {
                    pub rep: PhiRepresentation,
                }

                #[derive(Debug)]
                pub struct RetainOp {}

                #[derive(Debug)]
                pub struct ConstantOp {
                    pub kind: ConstantOp::Kind,
                }

                impl ConstantOp {
                    pub enum Kind {
                        kHeapObject,
                        kExternal,
                        kRelocatableWasmCall,
                        kRelocatableWasmStubCall,
                    }
                }

                #[derive(Debug)]
                pub struct LoadRootRegisterOp {}

                #[derive(Debug)]
                pub struct CallOp {
                    pub callee_: OpIndex,
                    pub arguments_: Vec<OpIndex>,
                    pub frame_state_: Option<OpIndex>,
                    pub descriptor: Box<TSCallDescriptor>,
                }

                impl CallOp {
                    pub fn callee(&self) -> OpIndex {
                        self.callee_
                    }
                    pub fn arguments(&self) -> &[OpIndex] {
                        &self.arguments_
                    }
                    pub fn frame_state(&self) -> Option<OpIndex> {
                        self.frame_state_
                    }

                    pub fn results_rep(&self) -> &Vec<RegisterRepresentation>{
                        &self.descriptor.results_rep
                    }
                }

                #[derive(Debug)]
                pub struct TailCallOp {
                    pub callee_: OpIndex,
                    pub arguments_: Vec<OpIndex>,
                    pub descriptor: Box<TSCallDescriptor>,
                }

                impl TailCallOp {
                    pub fn callee(&self) -> OpIndex {
                        self.callee_
                    }
                    pub fn arguments(&self) -> &[OpIndex] {
                        &self.arguments_
                    }

                    pub fn outputs_rep(&self) -> &Vec<RegisterRepresentation>{
                        &self.descriptor.results_rep
                    }
                }

                #[derive(Debug)]
                pub struct FrameStateOp {
                    pub parent_frame_state_: OpIndex,
                }

                impl FrameStateOp {
                    pub fn parent_frame_state(&self) -> OpIndex {
                        self.parent_frame_state_
                    }
                }

                #[derive(Debug, Clone, Copy)]
                pub struct OptionalOpIndex {
                    index: OpIndex
                }

                impl OptionalOpIndex {
                    pub fn value(&self) -> OpIndex {
                        self.index
                    }
                }

                impl OptionalOpIndex {
                    pub fn value_or_invalid(&self) -> OpIndex {
                        if self.index.valid() {
                            self.index
                        } else {
                            OpIndex { id_: u32::MAX }
                        }
                    }
                }

                #[derive(Debug)]
                pub struct TSCallDescriptor {
                    pub descriptor: *const CallDescriptor,
                    pub results_rep: Vec<RegisterRepresentation>
                }

                #[derive(Debug)]
                pub struct Block {
                    predecessors: Vec<*mut Block>,
                    index_: BlockIndex,
                    loop_: bool,
                    start_: OpIndex,
                    end_: OpIndex,
                }

                impl Block {
                    pub fn PredecessorCount(&self) -> usize {
                        self.predecessors.len()
                    }
                    pub fn Predecessors(&self) -> &Vec<*mut Block> {
                        &self.predecessors
                    }
                    pub fn IsLoop(&self) -> bool {
                        self.loop_
                    }

                    pub fn index(&self) -> BlockIndex {
                        self.index_
                    }

                    pub fn start(&self) -> OpIndex {
                        self.start_
                    }

                    pub fn end(&self) -> OpIndex {
                        self.end_
                    }
                }

                #[derive(Debug, Clone, Copy)]
                pub struct BlockIndex {
                    id: i32
                }

                #[derive(Debug, Clone, Copy)]
                pub enum LoadRepresentation {}

                #[derive(Debug, Clone, Copy)]
                pub enum MemoryRepresentation {}

                #[derive(Debug, Clone, Copy)]
                pub enum RegisterRepresentation {}

                #[derive(Debug, Clone, Copy)]
                pub enum PhiRepresentation {
                    Double
                }

                impl PhiRepresentation {
                    pub fn machine_representation(&self) -> MachineRepresentation {
                        MachineRepresentation::Float64
                    }
                }

                #[derive(Debug, Clone, Copy)]
                pub enum WriteBarrierKind {}

                pub mod any_of {
                    pub trait AnyOf<T> {
                        fn is_any_of(&self, items: &[T]) -> bool;
                    }

                    impl<T: PartialEq> AnyOf<T> for T {
                        fn is_any_of(&self, items: &[T]) -> bool {
                            items.iter().any(|item| self == item)
                        }
                    }
                }
            }
        }
    }
}

pub mod v8_crate {
    pub struct Isolate {}
}

pub mod compiler {
    use super::v8::internal::compiler::turboshaft::OpIndex;
    pub use super::v8::internal::compiler::turboshaft::LoadRepresentation;
    pub use super::v8::internal::compiler::turboshaft::MemoryRepresentation;
    pub use super::v8::internal::compiler::turboshaft::RegisterRepresentation;
    use super::v8::internal::compiler::turboshaft::{Graph, CallOp, TailCallOp, LoadOp, Simd128LoadTransformOp, Simd256LoadTransformOp, StoreOp, AtomicRMWOp, AtomicWord32PairOp, Simd128ShuffleOp, FrameStateOp, Block, PhiOp, ConstantOp};
    use super::v8::internal::compiler::turboshaft::OptionalOpIndex;
    use super::v8::internal::compiler::turboshaft::TSCallDescriptor;
    use super::v8::internal::compiler::turboshaft::Opcode;
    use super::v8::internal::compiler::turboshaft::WriteBarrierKind;
    use super::base;

    pub use super::v8::internal::compiler::turboshaft::BlockIndex;

    pub struct CallDescriptor {}
    pub struct InstructionOperand {}
    pub struct Instruction {}
    pub struct InstructionSequence {}
    pub struct Operator {}
    pub struct PhiRepresentation {}
    pub struct FrameStateData {}

    pub struct TurboshaftAdapter {
        operation_matcher: turboshaft::OperationMatcher,
        graph_: *mut turboshaft::Graph,
    }

    impl TurboshaftAdapter {
        const IS_TURBOFAN: bool = false;
        const IS_TURBOSHAFT: bool = true;
        const ALLOWS_IMPLICIT_WORD64_TO_WORD32_TRUNCATION: bool = true;

        pub fn new(graph: *mut turboshaft::Graph) -> Self {
            TurboshaftAdapter {
                operation_matcher: turboshaft::OperationMatcher::new(unsafe { &*graph }),
                graph_: graph,
            }
        }

        pub fn turboshaft_graph(&self) -> *mut turboshaft::Graph {
            self.graph_
        }

        pub fn block(&self, schedule: *mut turboshaft::Graph, node: OpIndex) -> *mut turboshaft::Block {
            unsafe {
                let schedule = &mut *schedule;
                let block_index = schedule.BlockOf(node);
                let block = &mut schedule.GetMut(block_index);

                block as *mut turboshaft::Block
            }
        }

        pub fn rpo_number(&self, block: *const turboshaft::Block) -> RpoNumber {
            unsafe {
                RpoNumber::from_int((&*block).index().id)
            }
        }

        pub fn rpo_order(&self, schedule: *mut turboshaft::Graph) -> &Vec<turboshaft::Block> {
            unsafe {
                (&(*schedule)).blocks_vector()
            }
        }

         pub fn is_loop_header(&self, block: *const turboshaft::Block) -> bool {
            unsafe {
                (&(*block)).IsLoop()
            }
        }

        pub fn predecessor_count(&self, block: *const turboshaft::Block) -> usize {
            unsafe {
                (&(*block)).PredecessorCount()
            }
        }

        pub fn predecessor_at(&self, block: *const turboshaft::Block, index: usize) -> *mut turboshaft::Block {
            unsafe {
                (&(*block)).Predecessors()[index]
            }
        }

        pub fn nodes(&self, block: *const turboshaft::Block) -> base::iterator_range<Graph::OpIndexIterator> {
            unsafe {
                let graph = &mut *self.graph_;
                graph.OperationIndices(&(*block))
            }
        }

        pub fn is_phi(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).Is::<PhiOp>()
            }
        }

        pub fn phi_representation_of(&self, node: OpIndex) -> MachineRepresentation {
            unsafe {
                let graph = &*self.graph_;
                assert!(self.is_phi(node));
                let phi = graph.Get(node).Cast::<PhiOp>();
                phi.rep.machine_representation()
            }
        }

        pub fn is_retain(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).Is::<turboshaft::RetainOp>()
            }
        }

        pub fn is_heap_constant(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                let constant = graph.Get(node).TryCast::<ConstantOp>();
                match constant {
                    Some(c) => {
                        match c.kind {
                            ConstantOp::Kind::kHeapObject => true,
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
        }

        pub fn is_external_constant(&self, node: OpIndex) -> bool {
             unsafe {
                let graph = &*self.graph_;
                let constant = graph.Get(node).TryCast::<ConstantOp>();
                match constant {
                    Some(c) => {
                        match c.kind {
                            ConstantOp::Kind::kExternal => true,
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
        }

        pub fn is_relocatable_wasm_constant(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                let constant = graph.Get(node).TryCast::<ConstantOp>();
                match constant {
                    Some(c) => {
                        match c.kind {
                            ConstantOp::Kind::kRelocatableWasmCall | ConstantOp::Kind::kRelocatableWasmStubCall => true,
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
        }

        pub fn is_load_or_load_immutable(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).opcode == Opcode::kLoad
            }
        }

        pub fn is_protected_load(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;

                if graph.Get(node).opcode == Opcode::kSimd128LoadTransform {
                    return true;
                }
                if graph.Get(node).opcode == Opcode::kSimd256LoadTransform {
                    return true;
                }

                if !self.is_load_or_load_immutable(node) {
                    return false;
                }

                let mut traps_on_null = false;
                TurboshaftAdapter::LoadView::new(&(*self.graph_), node).is_protected(&mut traps_on_null)
            }
        }

        pub fn value_input_count(&self, node: OpIndex) -> i32 {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).input_count
            }
        }

        pub fn input_at(&self, node: OpIndex, index: usize) -> OpIndex {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).inputs[index]
            }
        }

        pub fn inputs(&self, node: OpIndex) -> base::Vector<OpIndex> {
            unsafe {
                let graph = &*self.graph_;
                let inputs = graph.Get(node).inputs();
                base::Vector { data: inputs.to_vec() }
            }
        }

        pub fn opcode(&self, node: OpIndex) -> Opcode {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).opcode
            }
        }

        pub fn is_exclusive_user_of(&self, user: OpIndex, value: OpIndex) -> bool {
            unsafe {
                assert!(user.valid());
                assert!(value.valid());
                let graph = &*self.graph_;
                let value_op = graph.Get(value);
                let user_op = graph.Get(user);
                let use_count = base::count_if(
                    user_op.inputs(),
                    |input| *input == value
                );

                if use_count == 0 {
                    if !value_op.saturated_use_count.IsOne() {
                        return false;
                    }
                    for input in user_op.inputs() {
                        let input_op = graph.Get(*input);
                        let indirect_use_count = base::count_if(
                            input_op.inputs(),
                            |input| *input == value
                        );
                        if indirect_use_count > 0) {
                            return input_op.saturated_use_count.IsOne();
                        }
                    }
                    return false;
                }

                let mut use_count_mut = use_count;

                if value_op.Is::<turboshaft::ProjectionOp>()) {
                    use_count_mut += 1;
                }

                assert!(use_count_mut <= value_op.saturated_use_count.Get() as usize);
                (value_op.saturated_use_count.Get() as usize == use_count_mut) && !value_op.saturated_use_count.IsSaturated()
            }
        }

        pub fn id(&self, node: OpIndex) -> u32 {
            node.id()
        }

        pub fn value(node: OptionalOpIndex) -> OpIndex {
            node.value()
        }

        pub fn block_terminator(&self, block: *const turboshaft::Block) -> OpIndex {
             unsafe {
                let graph = &*self.graph_;
                graph.PreviousIndex((&*block).end())
            }
        }

        pub fn parent_frame_state(&self, node: OpIndex) -> OpIndex {
            unsafe {
                let graph = &*self.graph_;
                let frame_state = graph.Get(node).Cast::<FrameStateOp>();
                frame_state.parent_frame_state()
            }
        }

        pub fn is_required_when_unused(&self, node: OpIndex) -> bool {
             unsafe {
                let graph = &*self.graph_;
                graph.Get(node).IsRequiredWhenUnused()
            }
        }

        pub fn is_commutative(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                let op = graph.Get(node);

                if let Some(word_binop) = op.TryCast::<turboshaft::WordBinopOp>() {
                    return turboshaft::WordBinopOp::IsCommutative(word_binop.kind);
                } else if let Some(overflow_binop) = op.TryCast::<turboshaft::OverflowCheckedBinopOp>() {
                    return turboshaft::OverflowCheckedBinopOp::IsCommutative(overflow_binop.kind);
                } else if let Some(float_binop) = op.TryCast::<turboshaft::FloatBinopOp>() {
                    return turboshaft::FloatBinopOp::IsCommutative(float_binop.kind);
                } else if let Some(comparison) = op.TryCast::<turboshaft::ComparisonOp>() {
                    return turboshaft::ComparisonOp::IsCommutative(comparison.kind);
                }
                return false;
            }
        }


        pub fn is_load(&self, node: OpIndex) -> bool {
            unsafe {
                let graph = &*self.graph_;
                graph.Get(node).Is::<LoadOp>() || graph.Get(node).Is::<Simd128LoadTransformOp>() || graph.Get(node).Is::<Simd256LoadTransformOp>()
            }
        }

        pub fn is_load_root_register(&self, node: OpIndex) -> bool {
             unsafe {
                let graph = &*self.graph_;
                graph.Get(node).Is::<turboshaft::LoadRootRegisterOp>()
            }
        }

        pub fn call_view(&self, node: OpIndex) -> TurboshaftAdapter::CallView {
            TurboshaftAdapter::CallView::new(unsafe { &mut *self.graph_ }, node)
        }

        pub fn load_view(&self, node: OpIndex) -> TurboshaftAdapter::LoadView {
            assert!(self.is_load(node));
            TurboshaftAdapter::LoadView::new(unsafe { &mut *self.graph_ }, node)
        }

        pub fn store_view(&self, node: OpIndex) -> TurboshaftAdapter::StoreView {
            TurboshaftAdapter::StoreView::new(unsafe { &mut *self.graph_ }, node)
        }

        pub fn atomic_rmw_view(&self, node: OpIndex) -> TurboshaftAdapter::AtomicRMWView {
            TurboshaftAdapter::AtomicRMWView::new(unsafe { &*self.graph_ }, node)
        }

        pub fn word32_atomic_pair_store_view(&self, node: OpIndex) -> TurboshaftAdapter::Word32AtomicPairStoreView {
            TurboshaftAdapter::Word32AtomicPairStoreView::new(unsafe { &*self.graph_ }, node)
        }

        pub fn simd_shuffle_view(&self, node: OpIndex) -> TurboshaftAdapter::SimdShuffleView {
            TurboshaftAdapter::SimdShuffleView::new(unsafe { &*self.graph_ }, node)
        }
    }

    impl TurboshaftAdapter {
        pub struct CallView<'a> {
            node_: OpIndex,
            call_op_: Option<&'a CallOp>,
            tail_call_op_: Option<&'a TailCallOp>,
        }

        impl<'a> CallView<'a> {
            pub fn new(graph: &'a mut turboshaft::Graph, node: OpIndex) -> Self {
                let operation = graph.Get(node);
                let call_op = operation.TryCast::<CallOp>();
                let tail_call_op = operation.TryCast::<TailCallOp>();
                assert!(call_op.is_some() || tail_call_op.is_some());

                CallView {
                    node_: node,
                    call_op_: call_op,
                    tail_call_op_: tail_call_op,
                }
            }

            pub fn return_count(&self) -> i32 {
                match &self.call_op_ {
                    Some(call_op) => call_op.results_rep().len() as i32,
                    None => {
                        match &self.tail_call_op_ {
                            Some(tail_call_op) => tail_call_op.outputs_rep().len() as i32,
                            None => unreachable!(),
                        }
                    }
                }
            }

            pub fn callee(&self) -> OpIndex {
                match &self.call_op_ {
                    Some(call_op) => call_op.callee(),
                    None => {
                        match &self.tail_call_op_ {
                            Some(tail_call_op) => tail_call_op.callee(),
                            None => unreachable!(),
                        }
                    }
                }
            }

            pub fn frame_state(&self) -> OpIndex {
                match &self.call_op_ {
                    Some(call_op) => {
                        match call_op.frame_state() {
                            Some(frame_state) => frame_state,
                            None => unreachable!(),
                        }
                    },
                    None => unreachable!(),
                }
            }

            pub fn arguments(&self) -> &[OpIndex] {
                 match &self.call_op_ {
                    Some(call_op) => call_op.arguments(),
                    None => {
                        match &self.tail_call_op_ {
                            Some(tail_call_op) => tail_call_op.arguments(),
                            None => unreachable!(),
                        }
                    }
                }
            }

            pub fn call_descriptor(&self) -> *const CallDescriptor {
                match &self.call_op_ {
                    Some(call_op) => call_op.descriptor.descriptor,
                    None => {
                        match &self.tail_call_op_ {
                            Some(tail_call_op) => tail_call_op.descriptor.descriptor,
                            None => unreachable!(),
                        }
                    }
                }
            }

             pub fn ts_call_descriptor(&self) -> &TSCallDescriptor {
                match &self.call_op_ {
                    Some(call_op) => &*call_op.descriptor,
                    None => {
                        match &self.tail_call_op_ {
                            Some(tail_call_op) => &*tail_call_op.descriptor,
                            None => unreachable!(),
                        }
                    }
                }
            }
        }

        impl From<TurboshaftAdapter::CallView<'_>> for OpIndex {
            fn from(view: TurboshaftAdapter::CallView) -> Self {
                view.node_
            }
        }
    }

    impl TurboshaftAdapter {
        pub struct LoadView<'a> {
            node_: OpIndex,
            load_: Option<&'a LoadOp>,
            load_transform_: Option<&'a Simd128LoadTransformOp>,
            load_transform256_: Option<&'a Simd256LoadTransformOp>,
        }

        impl<'a> LoadView<'a> {
            pub fn new(graph: &'a mut turboshaft::Graph, node: OpIndex) -> Self {
                let operation = graph.Get(node);
                let load_ = operation.TryCast::<LoadOp>();
                let load_transform_ = operation.TryCast::<Simd128LoadTransformOp>();
                let load_transform256_ = operation.TryCast::<Simd256LoadTransformOp>();

                assert!(load_.is_some() || load_transform_.is_some() || load_transform256_.is_some());

                LoadView {
                    node_: node,
                    load_: load_,
                    load_transform_: load_transform_,
                    load_transform256_: load_transform256_,
                }
            }

            pub fn loaded_rep(&self) -> LoadRepresentation {
                self.load_.unwrap().machine_type()
            }

            pub fn ts_loaded_rep(&self) -> MemoryRepresentation {
                self.load_.unwrap().loaded_rep
            }

            pub fn ts_result_rep(&self) -> RegisterRepresentation {
                self.load_.unwrap().result_rep
            }

            pub fn is_protected(&self, traps_on_null: &mut bool) -> bool {
                if self.kind().with_trap_handler {
                    if let Some(load_) = self.load_ {
                        *traps_on_null = load_.kind.trap_on_null;
                    } else {
                        *traps_on_null = false;
                    }
                    return true;
                }
                false
            }

            pub fn is_atomic(&self) -> bool {
                self.kind().is_atomic
            }

            pub fn base(&self) -> OpIndex {
                match &self.load_ {
                    Some(load_) => load_.base(),
                    None => {
                        match &self.load_transform_ {
                            Some(load_transform_) => load_transform_.base(),
                            None => {
                               

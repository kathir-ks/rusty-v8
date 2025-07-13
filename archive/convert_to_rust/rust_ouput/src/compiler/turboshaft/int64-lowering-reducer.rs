// Converted from V8 C++ source files:
// Header: int64-lowering-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::rc::Rc;
use crate::v8::internal::compiler::turboshaft::V8;
use crate::v8::internal::compiler::turboshaft::RegisterRepresentation;
use crate::v8::internal::compiler::turboshaft::OpIndex;

mod base {
    pub struct Vector<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn size(&self) -> usize {
            0
        }
    }

    pub struct SmallVector<T, const N: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const N: usize> SmallVector<T, N> {
        pub fn new() -> Self {
            SmallVector {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn push_back(&mut self, _value: T) {}
        pub fn reserve(&mut self, _capacity: usize) {}
    }
    pub fn VectorOf<T>(_vec: Vec<T>) -> Vector<T> {
        Vector::new()
    }
}
mod wasm {
    pub enum CallOrigin {
        kCalledFromJS,
        kCalledFromWasm,
    }
    pub mod wasm_engine {
        pub struct WasmEngine {

        }
        impl WasmEngine {
            pub fn new() -> Self {
                WasmEngine {

                }
            }

            pub fn call_descriptors(&self) -> CallDescriptorTable {
                CallDescriptorTable::new()
            }
        }
        pub struct CallDescriptorTable {

        }
        impl CallDescriptorTable {
            pub fn new() -> Self {
                CallDescriptorTable{}
            }
            pub fn GetLoweredCallDescriptor(&self, _desc: &CallDescriptor) -> Option<&CallDescriptor>{
                None
            }
        }
    }
    pub fn GetWasmEngine() -> &'static wasm_engine::WasmEngine {
        Box::leak(Box::new(wasm_engine::WasmEngine::new()))
    }
}
mod compiler {
    pub struct FrameStateFunctionInfo {
        parameter_count: u16,
        max_arguments: i32,
        local_count: i32,
        wasm_liftoff_frame_size: i32,
        wasm_function_index: i32,
    }
    impl FrameStateFunctionInfo {
        pub fn new() -> Self {
            FrameStateFunctionInfo{
                parameter_count: 0,
                max_arguments: 0,
                local_count: 0,
                wasm_liftoff_frame_size: 0,
                wasm_function_index: 0,
            }
        }

        pub fn parameter_count(&self) -> u16 {
            self.parameter_count
        }
        pub fn max_arguments(&self) -> i32 {
            self.max_arguments
        }
        pub fn local_count(&self) -> i32 {
            self.local_count
        }
        pub fn shared_info(&self) -> i32 {
            0
        }
        pub fn wasm_liftoff_frame_size(&self) -> i32 {
            self.wasm_liftoff_frame_size
        }
        pub fn wasm_function_index(&self) -> i32 {
            self.wasm_function_index
        }
        pub fn shared_info(&self) -> i32 {
            0
        }
    }
    pub enum FrameStateType {
        kLiftoffFunction
    }
    pub struct FrameStateInfo {
        bailout_id: i32,
        state_combine: i32,
        function_info: FrameStateFunctionInfo,
    }
    impl FrameStateInfo {
        pub fn new() -> Self {
            FrameStateInfo{
                bailout_id: 0,
                state_combine: 0,
                function_info: FrameStateFunctionInfo::new(),
            }
        }

        pub fn bailout_id(&self) -> i32 {
            self.bailout_id
        }
        pub fn state_combine(&self) -> i32 {
            self.state_combine
        }
        pub fn function_info(&self) -> &FrameStateFunctionInfo {
            &self.function_info
        }
    }
}

mod codegen {
    pub enum MachineType {
        Int32,
        Int64,
        Float64,
    }
}
mod zone {
    pub struct Zone {}
    impl Zone {
        pub fn new() -> Self {
            Zone{}
        }
    }
}
mod v8_types {
    pub struct Signature<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Signature<T> {
        pub fn new() -> Self {
            Signature {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn parameter_count(&self) -> usize {
            0
        }
        pub fn returns(&self) -> Vec<T>{
            Vec::new()
        }
    }
    pub enum MachineRepresentation {
        kWord64,
        kWord32,
    }
    pub struct CallDescriptor {
        parameter_count: usize,
        return_count: usize,
    }
    impl CallDescriptor {
        pub fn new() -> Self {
            CallDescriptor{
                parameter_count: 0,
                return_count: 0
            }
        }
        pub fn ParameterCount(&self) -> usize{
            self.parameter_count
        }
        pub fn ReturnCount(&self) -> usize{
            self.return_count
        }
        pub fn GetParameterType(&self, _index: usize) -> MachineType {
            MachineType::new()
        }
        pub fn GetReturnType(&self, _index: usize) -> MachineType {
            MachineType::new()
        }
    }
    pub struct MachineType {
    }
    impl MachineType {
        pub fn new() -> Self {
            MachineType{}
        }
        pub fn representation(&self) -> MachineRepresentation {
            MachineRepresentation::kWord32
        }
    }
}
mod turboshaft {
    use crate::v8::internal::compiler::turboshaft::OpIndex;
    use crate::v8_types::Signature;
    use crate::v8_types::MachineRepresentation;
    use crate::zone::Zone;
    use crate::codegen::MachineType;
    use crate::v8::internal::compiler::turboshaft::RegisterRepresentation;
    use crate::compiler::FrameStateInfo;
    use crate::compiler::FrameStateFunctionInfo;
    use crate::compiler::FrameStateType;

    pub struct OperationMatcher {}
    impl OperationMatcher {
        pub fn TryCast<T>(&self, _op: OpIndex) -> Option<&T> {
            None
        }
        pub fn Is<T>(&self, _op: OpIndex) -> bool {
            false
        }
        pub fn MatchIntegralWord32Constant(&self, _op: OpIndex, _constant: &mut u32) -> bool {
            false
        }
    }

    pub struct PipelineData {}
    impl PipelineData {
        pub fn new() -> Self {
            PipelineData{}
        }
        pub fn is_js_to_wasm(&self) -> bool{
            false
        }
        pub fn wasm_module_sig(&self) -> Option<&Signature<MachineRepresentation>>{
            None
        }
        pub fn wasm_canonical_sig(&self) -> Option<&Signature<MachineRepresentation>>{
            None
        }
    }
    pub struct Graph {

    }
    impl Graph {
        pub fn new() -> Self {
            Graph{}
        }
    }
    pub struct Phase {
        data: PipelineData,
        zone: Zone,
        graph: Graph,
        matcher: OperationMatcher,
    }

    impl Phase {
        pub fn new() -> Self {
            Phase {
                data: PipelineData::new(),
                zone: Zone::new(),
                graph: Graph::new(),
                matcher: OperationMatcher{},
            }
        }

        pub fn data(&self) -> &PipelineData {
            &self.data
        }
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
        pub fn graph(&self) -> &Graph {
            &self.graph
        }
        pub fn matcher(&self) -> &OperationMatcher {
            &self.matcher
        }
    }
    pub struct PhaseZone<'a> {
        _phantom: std::marker::PhantomData<&'a ()>,
    }
    impl<'a> PhaseZone<'a> {
        pub fn new() -> Self {
            PhaseZone{
                _phantom: std::marker::PhantomData,
            }
        }
    }
    pub struct GraphZone<'a> {
        _phantom: std::marker::PhantomData<&'a ()>,
    }
    impl<'a> GraphZone<'a> {
        pub fn new() -> Self {
            GraphZone{
                _phantom: std::marker::PhantomData,
            }
        }
    }
    pub struct CompilationZone<'a> {
        _phantom: std::marker::PhantomData<&'a ()>,
    }
    impl<'a> CompilationZone<'a> {
        pub fn new() -> Self {
            CompilationZone{
                _phantom: std::marker::PhantomData,
            }
        }
    }
    pub trait NextTrait {
        fn ReduceWordBinop(&mut self, left: OpIndex, right: OpIndex, kind: WordBinopOp::Kind, rep: WordRepresentation) -> OpIndex;
        fn ReduceShift(&mut self, left: OpIndex, right: V<Word32>, kind: ShiftOp::Kind, rep: WordRepresentation) -> OpIndex;
        fn ReduceComparison(&mut self, left: V<Any>, right: V<Any>, kind: ComparisonOp::Kind, rep: RegisterRepresentation) -> V<Word32>;
        fn ReduceTailCall(&mut self, callee: OpIndex, arguments: base::Vector<const OpIndex>, descriptor: &TSCallDescriptor) -> OpIndex;
        fn ReduceCall(&mut self, callee: V<CallTarget>, frame_state: OptionalV<FrameState>, arguments: base::Vector<const OpIndex>, descriptor: &TSCallDescriptor, effects: OpEffects) -> V<Any>;
        fn ReduceConstant(&mut self, kind: ConstantOp::Kind, value: ConstantOp::Storage) -> OpIndex;
        fn ReduceParameter(&mut self, parameter_index: i32, rep: RegisterRepresentation, debug_name: &str) -> OpIndex;
        fn ReduceReturn(&mut self, pop_count: V<Word32>, return_values: base::Vector<const OpIndex>, spill_caller_frame_slots: bool) -> V<None>;
        fn ReduceWordUnary(&mut self, input: OpIndex, kind: WordUnaryOp::Kind, rep: WordRepresentation) -> OpIndex;
        fn ReduceChange(&mut self, input: OpIndex, kind: ChangeOp::Kind, assumption: ChangeOp::Assumption, from: RegisterRepresentation, to: RegisterRepresentation) -> OpIndex;
        fn ReduceLoad(&mut self, base: OpIndex, index: OptionalOpIndex, kind: LoadOp::Kind, loaded_rep: MemoryRepresentation, result_rep: RegisterRepresentation, offset: i32, element_scale: u8) -> OpIndex;
        fn ReduceStore(&mut self, base: OpIndex, index: OptionalOpIndex, value: OpIndex, kind: StoreOp::Kind, stored_rep: MemoryRepresentation, write_barrier: WriteBarrierKind, offset: i32, element_size_log2: u8, maybe_initializing_or_transitioning: bool, maybe_indirect_pointer_tag: IndirectPointerTag) -> V<None>;
        fn ReduceAtomicRMW(&mut self, base: OpIndex, index: OpIndex, value: OpIndex, expected: OptionalOpIndex, bin_op: AtomicRMWOp::BinOp, in_out_rep: RegisterRepresentation, memory_rep: MemoryRepresentation, kind: MemoryAccessKind) -> OpIndex;
        fn ReducePhi(&mut self, inputs: base::Vector<const OpIndex>, rep: RegisterRepresentation) -> OpIndex;
        fn ReducePendingLoopPhi(&mut self, input: OpIndex, rep: RegisterRepresentation) -> OpIndex;
        fn FixLoopPhi(&mut self, input_phi: &PhiOp, output_index: OpIndex, output_graph_loop: &Block);
        fn ReduceSimd128Splat(&mut self, input: V<Any>, kind: Simd128SplatOp::Kind) -> V<Simd128>;
        fn ReduceSimd128ExtractLane(&mut self, input: V<Simd128>, kind: Simd128ExtractLaneOp::Kind, lane: u8) -> V<Any>;
        fn ReduceSimd128ReplaceLane(&mut self, into: V<Simd128>, new_lane: V<Any>, kind: Simd128ReplaceLaneOp::Kind, lane: u8) -> V<Simd128>;
        fn ReduceFrameState(&mut self, inputs: base::Vector<const OpIndex>, inlined: bool, data: *const FrameStateData) -> V<FrameState>;
    }
    pub struct ReducerBoilerplate {
        phase: Phase,
    }
    impl ReducerBoilerplate {
        pub fn new() -> Self {
            ReducerBoilerplate{
                phase: Phase::new(),
            }
        }
    }

    pub struct Assembler {
        data: AssemblerData,
    }
    impl Assembler {
        pub fn new() -> Self {
            Assembler{
                data: AssemblerData::new(),
            }
        }
        pub fn data(&self) -> &AssemblerData {
            &self.data
        }
    }

    pub struct AssemblerData {
        compilation_zone: Zone,
    }
    impl AssemblerData {
        pub fn new() -> Self {
            AssemblerData{
                compilation_zone: Zone::new(),
            }
        }
        pub fn compilation_zone(&self) -> &Zone {
            &self.compilation_zone
        }
    }
    pub struct DefineAssemblerMacros<'a> {
        assembler: &'a Assembler,
    }
    impl<'a> DefineAssemblerMacros<'a> {
        pub fn new(assembler: &'a Assembler) -> Self {
            DefineAssemblerMacros{
                assembler,
            }
        }
        pub fn data(&self) -> &AssemblerData {
            self.assembler.data()
        }
    }

    pub struct LoadOp {
        _phantom: std::marker::PhantomData<u32>
    }

    impl LoadOp {
        pub fn OffsetIsValid(_offset: i32, _tagged_base: bool) -> bool {
            true
        }
    }

    pub enum MemoryRepresentation {
        Int64,
        Uint64,
        Int32,
        Uint32,
    }
    pub struct MemoryAccessKind {
        _phantom: std::marker::PhantomData<u32>
    }

    pub struct StoreOp {
        _phantom: std::marker::PhantomData<u32>
    }

    pub struct AtomicRMWOp {
        _phantom: std::marker::PhantomData<u32>
    }

    impl AtomicRMWOp {
        pub enum BinOp {
            kCompareExchange
        }
    }

    pub enum WriteBarrierKind {
        kNoWriteBarrier
    }
    pub enum IndirectPointerTag {
        kNullTag
    }
    pub struct AtomicWord32PairOp {
        _phantom: std::marker::PhantomData<u32>
    }
    impl AtomicWord32PairOp {
        pub fn new() -> Self {
            AtomicWord32PairOp{
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct TupleOp {
        _phantom: std::marker::PhantomData<u32>,
        pub input_count: i32,
    }
    impl TupleOp {
        pub fn new() -> Self {
            TupleOp{
                _phantom: std::marker::PhantomData,
                input_count: 0,
            }
        }
        pub fn input(&self, _i: usize) -> OpIndex {
            OpIndex{}
        }
    }

    pub struct DidntThrowOp {
        _phantom: std::marker::PhantomData<u32>,
    }
    impl DidntThrowOp {
        pub fn throwing_operation(&self) -> OpIndex {
            OpIndex{}
        }
    }

    pub struct CallOp {
        _phantom: std::marker::PhantomData<u32>,
        pub descriptor: TSCallDescriptorWrapper,
    }

    pub struct PendingLoopPhiOp {
        _phantom: std::marker::PhantomData<u32>,
    }
    impl PendingLoopPhiOp {
        pub fn first(&self) -> OpIndex {
            OpIndex{}
        }
    }

    pub struct ProjectionOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    pub struct ShiftOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl ShiftOp {
        pub enum Kind {
            kShiftLeft,
            kShiftRightArithmetic,
            kShiftRightLogical,
            kRotateRight,
        }
    }

    pub struct WordBinopOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl WordBinopOp {
        pub enum Kind {
            kAdd,
            kSub,
            kMul,
            kBitwiseAnd,
            kBitwiseOr,
            kBitwiseXor,
        }
    }

    pub struct Simd128SplatOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl Simd128SplatOp {
        pub enum Kind {
            kI64x2,
            kI32x4,
        }
    }

    pub struct Simd128ReplaceLaneOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl Simd128ReplaceLaneOp {
        pub enum Kind {
            kI64x2,
            kI32x4,
        }
    }

    pub struct Simd128ExtractLaneOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl Simd128ExtractLaneOp {
        pub enum Kind {
            kI64x2,
            kI32x4,
        }
    }

    pub struct WordUnaryOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl WordUnaryOp {
        pub enum Kind {
            kCountLeadingZeros,
            kCountTrailingZeros,
            kPopCount,
            kSignExtend8,
            kSignExtend16,
            kReverseBytes,
        }
    }

    pub struct ComparisonOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl ComparisonOp {
        pub enum Kind {
            kEqual,
            kSignedLessThan,
            kSignedLessThanOrEqual,
            kUnsignedLessThan,
            kUnsignedLessThanOrEqual,
        }
    }

    pub enum ChangeOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl ChangeOp {
        pub enum Kind {
            kZeroExtend,
            kSignExtend,
            kBitcast,
            kTruncate,
        }

        pub enum Assumption {
            kNoOverflow,
        }
    }
    pub struct ConstantOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl ConstantOp {
        pub enum Kind {
            kWord64,
            kWord32,
        }
        pub struct Storage {
            pub integral: u64
        }
    }
    pub struct FrameStateData {
        frame_state_info: FrameStateInfoWrapper,
        machine_types: Vec<MachineType>,
    }
    impl FrameStateData {
        pub fn new() -> Self {
            FrameStateData{
                frame_state_info: FrameStateInfoWrapper{
                    info: FrameStateInfo::new(),
                },
                machine_types: Vec::new(),
            }
        }

    }

    pub struct FrameStateDataWrapper {
        data: FrameStateData,
    }

    impl FrameStateDataWrapper {
        pub fn new() -> Self {
            FrameStateDataWrapper{
                data: FrameStateData::new(),
            }
        }
    }

    pub struct FrameStateInfoWrapper {
        info: FrameStateInfo,
    }

    impl FrameStateInfoWrapper {
        pub fn new() -> Self {
            FrameStateInfoWrapper{
                info: FrameStateInfo::new(),
            }
        }
    }
    impl FrameStateData {
        pub struct Builder {
            inputs: Vec<OpIndex>,
            inlined: bool,
        }

        impl Builder {
            pub fn new() -> Self {
                Builder {
                    inputs: Vec::new(),
                    inlined: false,
                }
            }

            pub fn AddInput(&mut self, _ty: MachineType, _op: OpIndex) {}
            pub fn AddParentFrameState(&mut self, _op: V<FrameState>){}

            pub fn Inputs(&self) -> base::Vector<const OpIndex> {
                base::Vector::new()
            }
            pub fn inlined(&self) -> bool {
                self.inlined
            }
            pub fn AllocateFrameStateData(&self, _frame_state_info: FrameStateInfo, _zone: &Zone) -> *const FrameStateData {
                std::ptr::null()
            }
        }
    }
    pub struct Block {
        _phantom: std::marker::PhantomData<u32>,
    }
    impl Block {
        pub fn Contains(&self, _phi_index: OpIndex) -> bool {
            false
        }
    }
    pub struct TSCallDescriptorWrapper {
        descriptor: TSCallDescriptor,
    }

    impl TSCallDescriptorWrapper {
        pub fn new() -> Self {
            TSCallDescriptorWrapper{
                descriptor: TSCallDescriptor::new(),
            }
        }
    }
    pub struct TSCallDescriptor {
        can_throw: bool,
        lazy_deopt_on_throw: LazyDeoptOnThrow,
        descriptor: Box<v8_types::CallDescriptor>,
    }

    impl TSCallDescriptor {
        pub fn new() -> Self {
            TSCallDescriptor {
                can_throw: false,
                lazy_deopt_on_throw: LazyDeoptOnThrow::kNo,
                descriptor: Box::new(v8_types::CallDescriptor::new()),
            }
        }
        pub fn Create(_lowered_descriptor: &v8_types::CallDescriptor, _can_throw: bool, _lazy_deopt_on_throw: LazyDeoptOnThrow, _zone: &Zone) -> Self {
            TSCallDescriptor::new()
        }
    }

    pub enum LazyDeoptOnThrow {
        kNo,
    }
    pub struct Word32PairBinopOp {
        _phantom: std::marker::PhantomData<u32>,
    }

    impl Word32PairBinopOp {
        pub enum Kind {
            kAdd,
            kSub,
            kMul,
            kShiftLeft,
            kShiftRightArithmetic,
            kShiftRightLogical,
        }
    }

    pub struct TSCallDescriptor {
        _phantom: std::marker::PhantomData<u32>,
    }
    impl TSCallDescriptor {
        pub fn Create(_call_descriptor: &v8_types::CallDescriptor, _can_throw: bool, _lazy_deopt_on_throw: LazyDeoptOnThrow, _zone: &Zone) -> Self {
            TSCallDescriptor{}
        }
    }

    #[derive(Clone, Copy)]
    pub struct V<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> V<T> {
        pub fn Cast(_op: OpIndex) -> Self {
            V{
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn Invalid() -> Self {
            V{
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn value(&self) -> OpIndex {
            OpIndex{}
        }
    }
    impl V<FrameState>{
        pub fn value(&self) -> &FrameState {
            &FrameState{}
        }
    }
    pub struct Any {}
    pub struct Word32 {}
    pub struct Word32Pair {}
    pub struct Simd128 {}
    pub struct CallTarget {}
    pub struct FrameState {}
    pub struct None {}

    pub struct OptionalV<T> {
        _phantom: std::marker::PhantomData<T>,
        has_value: bool,
    }

    impl<T> OptionalV<T> {
        pub fn Nullopt() -> Self {
            OptionalV {
                _phantom: std::marker::PhantomData,
                has_value: false,
            }
        }
        pub fn has_value(&self) -> bool {
            self.has_value
        }
        pub fn value(&self) -> V<T>{
            V{
                _phantom: std::marker::PhantomData,
            }
        }
    }
    pub struct OptionalOpIndex {
        _phantom: std::marker::PhantomData<u32>,
        has_value: bool,
    }

    impl OptionalOpIndex {
        pub fn Nullopt() -> Self {
            OptionalOpIndex {
                _phantom: std::marker::PhantomData,
                has_value: false,
            }
        }
        pub fn has_value(&self) -> bool {
            self.has_value
        }
    }

    pub enum WordRepresentation {
        Word64(),
        Word32(),
    }
    impl WordRepresentation {
        pub fn from_machine_type(_mt: MachineType) -> Self {
            Self::Word32()
        }
    }
    impl std::cmp::PartialEq for WordRepresentation {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    pub struct OpEffects {}

    impl OpEffects {
        pub fn CanCallAnything(&self) -> Self {
            OpEffects {}
        }
    }
    impl ReducerBoilerplate {
        pub fn Get(&self, _op: OpIndex) -> OperationGetter {
            OperationGetter{}
        }
        pub fn output_graph(&mut self) -> Graph {
            Graph::new()
        }
        pub fn graph_zone(&self) -> GraphZone{
            GraphZone::new()
        }
        pub fn phase_zone(&self) -> PhaseZone{
            PhaseZone::new()
        }
        pub fn compilation_zone(&self) -> CompilationZone{
            CompilationZone::new()
        }
        pub fn matcher(&self) -> OperationMatcher{
            OperationMatcher{}
        }
        pub fn asm(&self) -> Assembler {
            Assembler::new()
        }
        pub fn MapToNewGraph(&self, _op: OpIndex) -> OpIndex {
            OpIndex{}
        }
        pub fn Replace<T>(&mut self, _op: OpIndex, _inputs: base::Vector<const OpIndex>, _rep: RegisterRepresentation){

        }
        pub fn Insert<T>(&mut self, _op: OpIndex, _inputs: base::Vector<const OpIndex>, _rep: RegisterRepresentation){

        }
    }

    pub struct OperationGetter {}
    impl OperationGetter {
        pub fn Cast<T>(&self, _op: OpIndex) -> &T {
            
            unsafe { std::mem::transmute(self) }
        }
    }
}
mod object {

}
mod objects_inl {
    pub struct HeapObject {
    }
}
mod execution {
    pub mod frames_inl {
        pub struct StackFrameInfo {

        }
        impl StackFrameInfo {
            pub fn target(&self) -> objects_inl::HeapObject {
                objects_inl::HeapObject{}
            }
        }
    }
}
mod phase {
    pub mod phase {
        pub struct PipelineData {}
    }
}
pub mod base {
    pub struct Vector<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn size(&self) -> usize {
            0
        }
    }

    pub struct SmallVector<T, const N: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const N: usize> SmallVector<T, N> {
        pub fn new() -> Self {
            SmallVector {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn push_back(&mut self, _value: T) {}
        pub fn reserve(&mut self, _capacity: usize) {}
    }
    pub fn VectorOf<T>(_vec: Vec<T>) -> Vector<T> {
        Vector::new()
    }
}
mod wasm {
    pub enum CallOrigin {
        kCalledFromJS,
        kCalledFromWasm,
    }
    pub mod wasm_engine {
        pub struct WasmEngine {

        }
        impl WasmEngine {
            pub fn new() -> Self {
                WasmEngine {

                }
            }

            pub fn call_descriptors(&self) -> CallDescriptorTable {
                CallDescriptorTable::new()
            }
        }
        pub struct CallDescriptorTable {

        }
        impl CallDescriptorTable {
            pub fn new() -> Self {
                CallDescriptorTable{}
            }
            pub fn GetLoweredCallDescriptor(&self, _desc: &CallDescriptor) -> Option<&CallDescriptor>{
                None
            }
        }
    }
    pub fn GetWasmEngine() -> &'static wasm_engine::WasmEngine {
        Box::leak(Box::new(wasm_engine::WasmEngine::new()))
    }
}
mod compiler {
    pub struct FrameStateFunctionInfo {
        parameter_count: u16,
        max_arguments: i32,
        local_count: i32,
        wasm_liftoff_frame_size: i32,
        wasm_function_index: i32,
    }
    impl FrameStateFunctionInfo {
        pub fn new() -> Self {
            FrameStateFunctionInfo{
                parameter_count: 0,
                max_arguments: 0,
                local_count: 0,
                wasm_liftoff_frame_size: 0,
                wasm_function_index: 0,
            }
        }

        pub fn parameter_count(&self) -> u16 {
            self.parameter_count
        }
        pub fn max_arguments(&self) -> i32 {
            self.max_arguments
        }
        pub fn local_count(&self) -> i32 {
            self.local_count
        }
        pub fn shared_info(&self) -> i32 {
            0
        }
        pub fn wasm_liftoff_frame_size(&self) -> i32 {
            self.wasm_liftoff_frame_size
        }
        pub fn wasm_function_index(&self) -> i32 {
            self.wasm_function_index
        }
        pub fn shared_info(&self) -> i32 {
            0
        }
    }
    pub enum FrameStateType {
        kLiftoffFunction
    }
    pub struct FrameStateInfo {
        bailout_id: i32,
        state_combine: i32,
        function_info: FrameStateFunctionInfo,
    }
    impl FrameStateInfo {
        pub fn new() -> Self {
            FrameStateInfo{
                bailout_id: 0,
                state_combine: 0,
                function_info: FrameStateFunctionInfo::new(),
            }
        }

        pub fn bailout_id(&self) -> i32 {
            self.bailout_id
        }
        pub fn state_combine(&self) -> i32 {
            self.state_combine
        }
        pub fn function_info(&self) -> &FrameStateFunctionInfo {
            &self.function_info
        }
    }
}

mod codegen {
    pub enum MachineType {
        Int32,
        Int64,
        Float64,
    }
}
mod zone {
    pub struct Zone {}
    impl Zone {
        pub fn new() -> Self {
            Zone{}
        }
    }
}
mod v8_types {
    pub struct Signature<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Signature<T> {
        pub fn new() -> Self {
            Signature {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn parameter_count(&self) -> usize {
            0
        }
        pub fn returns(&self) -> Vec<T>{
            Vec::new()
        }
    }
    pub enum MachineRepresentation {
        kWord64,
        kWord32,
    }
    pub struct CallDescriptor {
        parameter_count: usize,
        return_count: usize,
    }
    impl CallDescriptor {
        pub fn new() -> Self {
            CallDescriptor{
                parameter_count: 0,
                return_count: 0
            }
        }
        pub fn ParameterCount(&self) -> usize

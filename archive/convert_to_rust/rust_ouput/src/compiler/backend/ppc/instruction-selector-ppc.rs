// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::Mutex;

//use crate::base::iterator::Reversed;
//use crate::compiler::backend::instruction_selector_impl::InstructionSelectorT;
//use crate::compiler::turboshaft::opmasks::WordBinopOp;
//use crate::execution::ppc::frame_constants_ppc::kStackFrameExtraParamSlot;
//use crate::roots::roots_inl::RootIndex;

pub struct InstructionSelector {}

pub enum ImmediateMode {
    kInt16Imm,
    kInt16Imm_Unsigned,
    kInt16Imm_Negate,
    kInt16Imm_4ByteAligned,
    kShift32Imm,
    kInt34Imm,
    kShift64Imm,
    kNoImmediate,
}

struct PPCOperandGeneratorT<'a> {
    selector: &'a InstructionSelectorT,
}

impl<'a> PPCOperandGeneratorT<'a> {
    pub fn new(selector: &'a InstructionSelectorT) -> Self {
        PPCOperandGeneratorT { selector }
    }

    fn UseOperand(&self, node: OpIndex, mode: ImmediateMode) -> InstructionOperand {
        if self.CanBeImmediate(node, mode) {
            return self.UseImmediate(node);
        }
        return self.UseRegister(node);
    }

    fn CanBeImmediate(&self, node: OpIndex, mode: ImmediateMode) -> bool {
        let constant = self.selector.Get(node).TryCast::<ConstantOp>();
        if constant.is_none() {
            return false;
        }
        let constant = constant.unwrap();

        if constant.kind == ConstantOp::Kind::kCompressedHeapObject {
            if !COMPRESS_POINTERS_BOOL {
                return false;
            }
            // For builtin code we need static roots
            if self.selector.isolate.bootstrapper.is_some() && !V8_STATIC_ROOTS_BOOL {
                return false;
            }
            let roots_table = &self.selector.isolate.roots_table;
            let mut root_index: Option<RootIndex> = None;
            let value = constant.handle;
            if roots_table.IsRootHandle(value, &mut root_index) {
                if !RootsTable::IsReadOnly(root_index.unwrap()) {
                    return false;
                }
                return self.CanBeImmediate(
                    MacroAssemblerBase::ReadOnlyRootPtr(root_index.unwrap(), &self.selector.isolate),
                    mode,
                );
            }
            return false;
        }

        let mut value: i64 = 0;
        if !self.selector.MatchSignedIntegralConstant(node, &mut value) {
            return false;
        }
        return self.CanBeImmediate_i64(value, mode);
    }

    fn CanBeImmediate_i64(&self, value: i64, mode: ImmediateMode) -> bool {
        match mode {
            ImmediateMode::kInt16Imm => is_int16(value),
            ImmediateMode::kInt16Imm_Unsigned => is_uint16(value),
            ImmediateMode::kInt16Imm_Negate => is_int16(-value),
            ImmediateMode::kInt16Imm_4ByteAligned => is_int16(value) && (value & 3) == 0,
            ImmediateMode::kShift32Imm => value >= 0 && value < 32,
            ImmediateMode::kInt34Imm => is_int34(value),
            ImmediateMode::kShift64Imm => value >= 0 && value < 64,
            ImmediateMode::kNoImmediate => false,
        }
    }

    fn UseImmediate(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn UseRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
}

struct InstructionSelectorT {}

impl InstructionSelectorT {
    fn Get(&self, node: OpIndex) -> Operation {
        Operation {}
    }
    fn load_view(&self, node: OpIndex) -> TurboshaftAdapter::LoadView {
        TurboshaftAdapter::LoadView {}
    }
    fn is_load_root_register(&self, base: OpIndex) -> bool {
        false
    }
    fn Get(node: &OpIndex) -> &Operation {
        todo!()
    }
    fn value(&self, index: TurboshaftAdapter::OptionalOpIndex) -> TurboshaftAdapter::OpIndex {
        TurboshaftAdapter::OpIndex {}
    }
    fn store_view(&self, node: OpIndex) -> StoreView {
        StoreView {}
    }
    fn Get(node: &OpIndex) -> &Operation {
        todo!()
    }
    fn input_at(&self, node: OpIndex, index: usize) -> OpIndex {
        OpIndex {}
    }
    fn MatchSignedIntegralConstant(&self, node: OpIndex, value: &mut i64) -> bool {
        false
    }
    fn turboshaft_graph(&self) -> &TurboshaftGraph {
        todo!()
    }
    fn atomic_rmw_view(&self, node: OpIndex) -> AtomicRMWView {
        AtomicRMWView {}
    }
}

struct OperandGenerator {}

impl OperandGenerator {
    fn UseRegister(&self, input_at: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
    fn UseImmediate(&self, is_atomic: i32) -> InstructionOperand {
        InstructionOperand {}
    }
    fn TempRegister(&self) -> InstructionOperand {
        InstructionOperand {}
    }
    fn DefineAsRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
}

struct FlagsContinuation {}

impl FlagsContinuation {
    fn ForSet(kEqual: Condition, node: OpIndex) -> FlagsContinuation {
        FlagsContinuation {}
    }
    fn Commute(&mut self) {}
    fn OverwriteAndNegateIfEqual(&mut self, equal: Condition) {}
}

enum Condition {
    kEqual,
    kUnsignedLessThan,
    kUnsignedLessThanOrEqual,
    kStackPointerGreaterThanCondition,
    kOverflow,
    kNotEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual,
}

impl InstructionSelectorT {
    fn VisitStackSlot(&mut self, node: OpIndex) {}
    fn VisitAbortCSADcheck(&mut self, node: OpIndex) {}
    fn Emit(&mut self, arch_stack_slot: ArchOpcode, define_as_register: InstructionOperand, add_immediate: InstructionOperand, i: i32, null: *const ()) {}
    fn EmitWithContinuation(&mut self, kPPC_Cmp32: ArchOpcode, high32_operand: InstructionOperand, temp_operand: InstructionOperand, cont: &FlagsContinuation) {}
    fn VisitLoad(&mut self, node: OpIndex) {}
    fn VisitProtectedLoad(&mut self, node: OpIndex) {}
    fn VisitUnalignedLoad(&mut self, node: OpIndex) {}
    fn VisitUnalignedStore(&mut self, node: OpIndex) {}
    fn VisitWord32And(&mut self, node: OpIndex) {}
    fn VisitWord64And(&mut self, node: OpIndex) {}
    fn VisitWord32Or(&mut self, node: OpIndex) {}
    fn VisitWord64Or(&mut self, node: OpIndex) {}
    fn VisitWord32Xor(&mut self, node: OpIndex) {}
    fn VisitWord64Xor(&mut self, node: OpIndex) {}
    fn VisitWord32Shl(&mut self, node: OpIndex) {}
    fn VisitWord64Shl(&mut self, node: OpIndex) {}
    fn VisitWord32Shr(&mut self, node: OpIndex) {}
    fn VisitWord64Shr(&mut self, node: OpIndex) {}
    fn VisitWord32Sar(&mut self, node: OpIndex) {}
    fn VisitWord64Sar(&mut self, node: OpIndex) {}
    fn VisitWord32Rol(&mut self, node: OpIndex) {}
    fn VisitWord64Rol(&mut self, node: OpIndex) {}
    fn VisitWord32Ror(&mut self, node: OpIndex) {}
    fn VisitWord64Ror(&mut self, node: OpIndex) {}
    fn VisitWord32Clz(&mut self, node: OpIndex) {}
    fn VisitWord64Clz(&mut self, node: OpIndex) {}
    fn VisitWord32Popcnt(&mut self, node: OpIndex) {}
    fn VisitWord64Popcnt(&mut self, node: OpIndex) {}
    fn VisitWord32Ctz(&mut self, node: OpIndex) {}
    fn VisitWord64Ctz(&mut self, node: OpIndex) {}
    fn VisitWord32ReverseBits(&mut self, node: OpIndex) {}
    fn VisitWord64ReverseBits(&mut self, node: OpIndex) {}
    fn VisitWord32ReverseBytes(&mut self, node: OpIndex) {}
    fn VisitWord64ReverseBytes(&mut self, node: OpIndex) {}
    fn VisitSimd128ReverseBytes(&mut self, node: OpIndex) {}
    fn VisitInt32Add(&mut self, node: OpIndex) {}
    fn VisitInt64Add(&mut self, node: OpIndex) {}
    fn VisitInt32Sub(&mut self, node: OpIndex) {}
    fn VisitInt64Sub(&mut self, node: OpIndex) {}
    fn VisitInt32Mul(&mut self, node: OpIndex) {}
    fn VisitInt64Mul(&mut self, node: OpIndex) {}
    fn VisitInt32MulHigh(&mut self, node: OpIndex) {}
    fn VisitUint32MulHigh(&mut self, node: OpIndex) {}
    fn VisitInt64MulHigh(&mut self, node: OpIndex) {}
    fn VisitUint64MulHigh(&mut self, node: OpIndex) {}
    fn VisitInt32Div(&mut self, node: OpIndex) {}
    fn VisitInt64Div(&mut self, node: OpIndex) {}
    fn VisitUint32Div(&mut self, node: OpIndex) {}
    fn VisitUint64Div(&mut self, node: OpIndex) {}
    fn VisitInt32Mod(&mut self, node: OpIndex) {}
    fn VisitInt64Mod(&mut self, node: OpIndex) {}
    fn VisitUint32Mod(&mut self, node: OpIndex) {}
    fn VisitUint64Mod(&mut self, node: OpIndex) {}
    fn VisitChangeFloat32ToFloat64(&mut self, node: OpIndex) {}
    fn VisitRoundInt32ToFloat32(&mut self, node: OpIndex) {}
    fn VisitRoundUint32ToFloat32(&mut self, node: OpIndex) {}
    fn VisitChangeInt32ToFloat64(&mut self, node: OpIndex) {}
    fn VisitChangeUint32ToFloat64(&mut self, node: OpIndex) {}
    fn VisitChangeFloat64ToInt32(&mut self, node: OpIndex) {}
    fn VisitChangeFloat64ToUint32(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat64ToUint32(&mut self, node: OpIndex) {}
    fn VisitSignExtendWord8ToInt32(&mut self, node: OpIndex) {}
    fn VisitSignExtendWord16ToInt32(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat32ToInt64(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat64ToInt64(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat64ToInt64(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat32ToUint64(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat64ToUint64(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat64ToInt32(&mut self, node: OpIndex) {}
    fn VisitTryTruncateFloat64ToUint32(&mut self, node: OpIndex) {}
    fn VisitBitcastWord32ToWord64(&mut self, node: OpIndex) {}
    fn VisitChangeInt32ToInt64(&mut self, node: OpIndex) {}
    fn VisitSignExtendWord8ToInt64(&mut self, node: OpIndex) {}
    fn VisitSignExtendWord16ToInt64(&mut self, node: OpIndex) {}
    fn VisitSignExtendWord32ToInt64(&mut self, node: OpIndex) {}
    fn ZeroExtendsWord32ToWord64NoPhis(&mut self, node: OpIndex) -> bool {
        false
    }
    fn VisitChangeUint32ToUint64(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat64ToFloat16RawBits(&mut self, node: OpIndex) {}
    fn VisitChangeFloat16RawBitsToFloat64(&mut self, node: OpIndex) {}
    fn VisitChangeFloat64ToUint64(&mut self, node: OpIndex) {}
    fn VisitChangeFloat64ToInt64(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat64ToFloat32(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat64ToWord32(&mut self, node: OpIndex) {}
    fn VisitRoundFloat64ToInt32(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat32ToInt32(&mut self, node: OpIndex) {}
    fn VisitTruncateFloat32ToUint32(&mut self, node: OpIndex) {}
    fn VisitTruncateInt64ToInt32(&mut self, node: OpIndex) {}
    fn VisitRoundInt64ToFloat32(&mut self, node: OpIndex) {}
    fn VisitRoundInt64ToFloat64(&mut self, node: OpIndex) {}
    fn VisitChangeInt64ToFloat64(&mut self, node: OpIndex) {}
    fn VisitRoundUint64ToFloat32(&mut self, node: OpIndex) {}
    fn VisitRoundUint64ToFloat64(&mut self, node: OpIndex) {}
    fn VisitBitcastFloat32ToInt32(&mut self, node: OpIndex) {}
    fn VisitBitcastFloat64ToInt64(&mut self, node: OpIndex) {}
    fn VisitBitcastInt32ToFloat32(&mut self, node: OpIndex) {}
    fn VisitBitcastInt64ToFloat64(&mut self, node: OpIndex) {}
    fn VisitFloat32Add(&mut self, node: OpIndex) {}
    fn VisitFloat64Add(&mut self, node: OpIndex) {}
    fn VisitFloat32Sub(&mut self, node: OpIndex) {}
    fn VisitFloat64Sub(&mut self, node: OpIndex) {}
    fn VisitFloat32Mul(&mut self, node: OpIndex) {}
    fn VisitFloat64Mul(&mut self, node: OpIndex) {}
    fn VisitFloat32Div(&mut self, node: OpIndex) {}
    fn VisitFloat64Div(&mut self, node: OpIndex) {}
    fn VisitFloat64Mod(&mut self, node: OpIndex) {}
    fn VisitFloat32Max(&mut self, node: OpIndex) {}
    fn VisitFloat64Max(&mut self, node: OpIndex) {}
    fn VisitFloat64SilenceNaN(&mut self, node: OpIndex) {}
    fn VisitFloat32Min(&mut self, node: OpIndex) {}
    fn VisitFloat64Min(&mut self, node: OpIndex) {}
    fn VisitFloat32Abs(&mut self, node: OpIndex) {}
    fn VisitFloat64Abs(&mut self, node: OpIndex) {}
    fn VisitFloat32Sqrt(&mut self, node: OpIndex) {}
    fn VisitFloat64Ieee754Unop(&mut self, node: OpIndex, opcode: InstructionCode) {}
    fn VisitFloat64Ieee754Binop(&mut self, node: OpIndex, opcode: InstructionCode) {}
    fn VisitFloat64Sqrt(&mut self, node: OpIndex) {}
    fn VisitFloat32RoundDown(&mut self, node: OpIndex) {}
    fn VisitFloat64RoundDown(&mut self, node: OpIndex) {}
    fn VisitFloat32RoundUp(&mut self, node: OpIndex) {}
    fn VisitFloat64RoundUp(&mut self, node: OpIndex) {}
    fn VisitFloat32RoundTruncate(&mut self, node: OpIndex) {}
    fn VisitFloat64RoundTruncate(&mut self, node: OpIndex) {}
    fn VisitFloat64RoundTiesAway(&mut self, node: OpIndex) {}
    fn VisitFloat32Neg(&mut self, node: OpIndex) {}
    fn VisitFloat64Neg(&mut self, node: OpIndex) {}
    fn VisitInt32AddWithOverflow(&mut self, node: OpIndex) {}
    fn VisitInt32SubWithOverflow(&mut self, node: OpIndex) {}
    fn VisitInt64AddWithOverflow(&mut self, node: OpIndex) {}
    fn VisitInt64SubWithOverflow(&mut self, node: OpIndex) {}
    fn VisitInt64MulWithOverflow(&mut self, node: OpIndex) {}
    fn VisitWordCompareZero(&mut self, user: OpIndex, value: OpIndex, cont: *mut FlagsContinuation) {}
    fn VisitWord32Equal(&mut self, const_node: OpIndex) {}
    fn VisitInt32LessThan(&mut self, node: OpIndex) {}
    fn VisitInt32LessThanOrEqual(&mut self, node: OpIndex) {}
    fn VisitUint32LessThan(&mut self, node: OpIndex) {}
    fn VisitUint32LessThanOrEqual(&mut self, node: OpIndex) {}
    fn VisitWord64Equal(&mut self, const_node: OpIndex) {}
    fn VisitInt64LessThan(&mut self, node: OpIndex) {}
    fn VisitInt64LessThanOrEqual(&mut self, node: OpIndex) {}
    fn VisitUint64LessThan(&mut self, node: OpIndex) {}
    fn VisitUint64LessThanOrEqual(&mut self, node: OpIndex) {}
    fn VisitInt32MulWithOverflow(&mut self, node: OpIndex) {}
    fn VisitFloat32Equal(&mut self, node: OpIndex) {}
    fn VisitFloat32LessThan(&mut self, node: OpIndex) {}
    fn VisitFloat32LessThanOrEqual(&mut self, node: OpIndex) {}
    fn VisitFloat64Equal(&mut self, node: OpIndex) {}
    fn VisitFloat64LessThan(&mut self, node: OpIndex) {}
    fn VisitFloat64LessThanOrEqual(&mut self, node: OpIndex) {}
    fn EmitMoveParamToFPR(&mut self, node: OpIndex, index: i32) {}
    fn EmitMoveFPRToParam(&mut self, op: *mut InstructionOperand, location: LinkageLocation) {}
    fn EmitPrepareArguments(&mut self, arguments: *mut ZoneVector<PushParameter>, call_descriptor: *const CallDescriptor, node: OpIndex) {}
    fn IsTailCallAddressImmediate(&mut self) -> bool {
        false
    }
    fn VisitFloat64ExtractLowWord32(&mut self, node: OpIndex) {}
    fn VisitFloat64ExtractHighWord32(&mut self, node: OpIndex) {}
    fn VisitBitcastWord32PairToFloat64(&mut self, node: OpIndex) {}
    fn VisitFloat64InsertLowWord32(&mut self, node: OpIndex) {}
    fn VisitFloat64InsertHighWord32(&mut self, node: OpIndex) {}
    fn VisitMemoryBarrier(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicLoad(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicLoad(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicStore(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicStore(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicExchange(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicExchange(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicCompareExchange(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicCompareExchange(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicAdd(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicAdd(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicSub(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicSub(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicAnd(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicAnd(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicOr(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicOr(&mut self, node: OpIndex) {}
    fn VisitWord32AtomicXor(&mut self, node: OpIndex) {}
    fn VisitWord64AtomicXor(&mut self, node: OpIndex) {}
    fn VisitInt32AbsWithOverflow(&mut self, node: OpIndex) {}
    fn VisitInt64AbsWithOverflow(&mut self, node: OpIndex) {}
    fn VisitI8x16Shuffle(&mut self, node: OpIndex) {}
    fn VisitS128Zero(&mut self, node: OpIndex) {}
    fn VisitS128Select(&mut self, node: OpIndex) {}
    fn VisitS128Const(&mut self, node: OpIndex) {}
    fn VisitI16x8DotI8x16I7x16S(&mut self, node: OpIndex) {}
    fn VisitI32x4DotI8x16I7x16AddS(&mut self, node: OpIndex) {}
    fn EmitPrepareResults(&mut self, results: *mut ZoneVector<PushParameter>, call_descriptor: *const CallDescriptor, node: OpIndex) {}
    fn VisitLoadLane(&mut self, node: OpIndex) {}
    fn VisitLoadTransform(&mut self, node: OpIndex) {}
    fn VisitStoreLane(&mut self, node: OpIndex) {}
    fn AddOutputToSelectContinuation(&mut self, g: *mut OperandGenerator, first_input_index: i32, node: OpIndex) {}
    fn VisitFloat32RoundTiesEven(&mut self, node: OpIndex) {}
    fn VisitFloat64RoundTiesEven(&mut self, node: OpIndex) {}
    fn VisitF64x2NearestInt(&mut self, node: OpIndex) {}
    fn VisitF32x4NearestInt(&mut self, node: OpIndex) {}
    fn is_load_root_register(
        &self,
        base: OpIndex,
    ) -> bool {
        false
    }
    fn MatchHeapConstant(&self, node: OpIndex, right: &mut Handle<HeapObject>) -> bool {
        false
    }
    fn EmitTableSwitch(&self, sw: SwitchInfo, index_operand: InstructionOperand) {}
    fn EmitBinarySearchSwitch(&self, sw: SwitchInfo, value_operand: InstructionOperand) {}
    fn is(&self, arg: RootIndex) -> bool {
        false
    }
    fn FindProjection(&self, node: OpIndex, i: i32) -> TurboshaftAdapter::OptionalOpIndex {
        TurboshaftAdapter::OptionalOpIndex {}
    }
    fn Cast<T>(&self, node: OpIndex) -> T where T: CastableOperation {
        todo!()
    }
    fn GetComparisonFlagCondition(&self, node: ComparisonOp) -> Condition {
        Condition::kEqual
    }
    fn CanCover(&self, const_node: OpIndex, value: OpIndex) -> bool {
        false
    }
    fn MatchUnsignedIntegralConstant(&self, right: OpIndex, right_value: &mut u64) -> bool {
        false
    }
    fn MatchIntegralWord64Constant(&self, right: OpIndex, mask: &mut i64) -> bool {
        false
    }
    fn simd_shuffle_view(&self, node: OpIndex) -> SimdShuffleView {
        SimdShuffleView {}
    }
    fn VisitSetStackPointer(&mut self, node: OpIndex) {}
    fn MatchIntegralZero(&self, left: OpIndex) -> bool {
        false
    }
    fn is_atomic(&self) -> bool {
        false
    }
}

struct TurboshaftAdapter {}

impl TurboshaftAdapter {
    struct LoadView {}
    struct OptionalOpIndex {}
    struct OpIndex {}
}

struct StoreView {}

struct SimdShuffleView {}

struct RootsTable {
    root_entries: Vec<i32>,
}

impl RootsTable {
    fn IsRootHandle(&self, value: Handle<HeapObject>, root_index: &mut Option<RootIndex>) -> bool {
        false
    }
    fn IsReadOnly(root_index: RootIndex) -> bool {
        false
    }
}

struct MacroAssemblerBase {}

impl MacroAssemblerBase {
    fn ReadOnlyRootPtr(root_index: RootIndex, isolate: &Isolate) -> i64 {
        0
    }
}

struct ConstantOp {
    kind: ConstantOpKind,
    handle: Handle<HeapObject>,
}

impl ConstantOp {
    fn TryCast<T>() -> Option<ConstantOp> {
        Some(ConstantOp{ kind: ConstantOpKind::kHeapObject, handle: Handle{}})
    }
}

struct Operation {}

impl Operation {
    fn Is<T>() -> bool {
        false
    }
    fn TryCast<T>() -> Option<Operation> {
        Some(Operation{})
    }
    fn input(&self, index: i32) -> OpIndex {
        OpIndex{}
    }
}

struct WordBinopOp {}

impl WordBinopOp {
    fn left(&self) -> OpIndex {
        OpIndex{}
    }
    fn right(&self) -> OpIndex {
        OpIndex{}
    }
}

struct ShiftOp {}

impl ShiftOp {
    fn right(&self) -> OpIndex {
        OpIndex{}
    }
    fn left(&self) -> OpIndex {
        OpIndex{}
    }
}

trait CastableOperation {

}

struct MachineType {}

struct LinkageLocation {}

impl LinkageLocation {
    fn IsCallerFrameSlot(&self) -> bool {
        false
    }
    fn GetType(&self) -> MachineType {
        MachineType {}
    }
    fn GetLocation(&self) -> i32 {
        0
    }
}

struct PushParameter {}

struct CallDescriptor {}

impl CallDescriptor {
    fn IsCFunctionCall(&self) -> bool {
        false
    }
    fn ParameterCount(&self) -> i32 {
        0
    }
    fn GetOffsetToReturns(&self) -> i32 {
        0
    }
}

struct ZoneVector<T> {}

struct SwitchInfo {}

struct Simd128ConstantOp {
    value: [u8; 16],
}

impl Simd128ConstantOp {
    fn TryCast<T>() -> Option<Simd128ConstantOp> {
        Some(Simd128ConstantOp{ value: [0u8; 16]})
    }
}

struct StackPointerGreaterThanOp {}

struct OverflowCheckedBinopOp {}

trait TurboshaftGraph {
}
impl TurboshaftGraph for InstructionSelectorT {}

impl LoadOp{
    fn base(&self) -> OpIndex {
        OpIndex{}
    }
    fn index(&self) -> TurboshaftAdapter::OptionalOpIndex {
        TurboshaftAdapter::OptionalOpIndex{}
    }
    fn loaded_rep(&self) -> LoadRepresentation {
        LoadRepresentation{}
    }
}

impl AtomicRMWView {
    fn base(&self) -> OpIndex {
        OpIndex{}
    }
    fn index(&self) -> OpIndex {
        OpIndex{}
    }
    fn value(&self) -> OpIndex {
        OpIndex{}
    }
    fn expected(&self) -> OpIndex {
        OpIndex{}
    }
}

struct AtomicRMWOp {
    memory_rep: MemoryRepresentation
}

impl CastableOperation for AtomicRMWOp {

}

enum MemoryRepresentation {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float16,
    Float32,
    Float64,
    AnyTagged,
    TaggedPointer,
    TaggedSigned,
    AnyUncompressedTagged,
    UncompressedTaggedPointer,
    UncompressedTaggedSigned,
    SandboxedPointer,
    Simd128,
    ProtectedPointer,
    IndirectPointer,
    Simd256,
}
struct AtomicStoreParameters {
    memory_rep: MemoryRepresentation,
    write_barrier_kind: WriteBarrierKind,
    order: Option<AtomicMemoryOrder>,
    access_kind: AccessKind,
}
enum WriteBarrierKind {
    kNoWriteBarrier,
    kFullWriteBarrier,
    kIndirectPointerWriteBarrier
}
enum AtomicMemoryOrder {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent
}
enum AccessKind {
    Normal,
    Volatile,
}

impl AtomicStoreParameters {
    fn store_representation(&self) -> StoreRepresentation {
        StoreRepresentation{}
    }
    fn order(&self) -> Option<AtomicMemoryOrder> {
        todo!()
    }
}

struct StoreRepresentation {}

struct ComparisonOp {
    rep: WordRepresentation,
    kind: ComparisonOpKind
}

enum ComparisonOpKind {
    kEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual
}

enum WordRepresentation {
    Word32,
    Word64
}

impl ComparisonOp {
    fn TryCast<T>() -> Option<ComparisonOp> {
        Some(ComparisonOp{ rep: WordRepresentation::Word32, kind: ComparisonOpKind::kEqual})
    }
}
impl LoadRepresentation {
    fn representation(&self) -> MachineRepresentation {
        MachineRepresentation::kWord32
    }
    fn IsSigned(&self) -> bool {
        false
    }
}

struct Handle<T> {}

enum ArchOpcode {
    kArchStackSlot,
    kArchAbortCSADcheck,
    kArchStackPointerGreaterThan,
    kPPC_LoadWord64,
    kPPC_StoreWord64,
    kPPC_LoadDecodeSandboxedPointer,
    kPPC_StoreEncodeSandboxedPointer,
    kArchPrepareCallCFunction,
    kPPC_Push,
    kArchNop,
    kPPC_Float32ToDouble,
    kPPC_Int32ToFloat32,
    kPPC_Uint32ToFloat32,
    kPPC_Int32ToDouble,
    kPPC_Uint32ToDouble,
    kPPC_DoubleToInt32,
    kPPC_DoubleToUint32,
    kPPC_ExtendSignWord8,
    kPPC_ExtendSignWord16,
    kPPC_DoubleToInt64,
    kArchTruncateDoubleToI,
    kPPC_Uint32ToUint64,
    kPPC_BitcastFloat32ToInt32,
    kPPC_BitcastDoubleToInt64,
    kPPC_BitcastInt32ToFloat32,
    kPPC_BitcastInt64ToDouble,
    kPPC_StoreWithWriteBarrier,
    kPPC_AddDouble,
    kPPC_SubDouble,
    kPPC_MulDouble,
    kPPC_DivDouble,
    kPPC_ModDouble,
    kPPC_MaxDouble,
    kPPC_MinDouble,
    kPPC_AbsDouble,
    kPPC_SqrtDouble,
    kPPC_FloorDouble,
    kPPC_CeilDouble,
    kPPC_TruncateDouble,
    kPPC_RoundDouble,
    kPPC_NegDouble,
    kPPC_AddWithOverflow32,
    kPPC_SubWithOverflow32,
    kPPC_Add32,
    kPPC_Sub,
    kPPC_And,
    kPPC_Or,
    kPPC_Xor,
    kPPC_Mul32,
    kPPC_Div32,
    kPPC_DivU32,
    kPPC_Mod32,
    kPPC_ModU32,
    kPPC_ShiftLeft32,
    kPPC_ShiftRight32,
    kPPC_ShiftRightAlg32,
    kPPC_Cntlz32,
    kPPC_Popcnt32,
    kPPC_DoubleFromWord32Pair,
    kPPC_ShiftLeft64,
    kPPC_ShiftRight64,
    kPPC_ShiftRightAlg64,
    kPPC_RotLeftAndMask32,
    kPPC_ByteRev32,
    kPPC_Cntlz64,
    kPPC_Popcnt64,
    kPPC_Not,
    kPPC_CmpDouble,
    kPPC_Cmp32,
    kPPC_Cmp64,
    kPPC_Tst32,
    kPPC_Tst64,
    kAtomicExchangeInt8,
    kPPC_AtomicExchangeUint8,
    kAtomicExchangeInt16,
    kPPC_AtomicExchangeUint16,
    kPPC_AtomicExchangeWord32,
    kPPC_AtomicExchangeWord64,
    kAtomicCompareExchangeInt8,
    kPPC_AtomicCompareExchangeUint8,
    kAtomicCompareExchangeInt16,
    kPPC_AtomicCompareExchangeUint16,

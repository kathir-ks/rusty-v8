// Converted from V8 C++ source files:
// Header: simplified-lowering-verifier.h
// Implementation: simplified-lowering-verifier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod container_utils {
        use std::vec::Vec;
        pub fn vector_append<T: Clone>(dest: &mut Vec<T>, src: &NodeUses<T>) {
            for item in &src.uses {
                dest.push(item.clone());
            }
        }
    }
}
pub mod compiler {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt;
    use std::ops::{Deref, DerefMut};
    use std::rc::Rc;
    use crate::compiler::machine_operator::IdentifyZeros;
    use crate::compiler::representation_change::TruncationKind;

    pub struct V8 {}
    pub struct v8 {}
    pub enum class JSCallReducerAssembler {}
    pub enum class JSCallReducerAssembler {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct SourcePosition {}
    pub enum void {}
    pub enum void {}
    pub enum void {}
    pub enum Type {
        None,
        Boolean,
        Signed32,
        Unsigned32,
        Number,
        Any,
        Machine,
        Constant(i32),
        SignedSmall,
        NumberOrOddball,
        BigInt,
        SignedBigInt64,
        UnsignedBigInt64,
        Range(i64, i64),
    }
    pub enum Type {}
    pub enum void {}
    pub enum void {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum void {}
    pub enum void {}
    pub enum void {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct v8 {}
    pub struct V8 {}
    pub struct v8 {}
    pub enum void {}
    pub enum Type {}
    pub enum void {}
    pub enum void {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum IdentifyZeros {
        kDistinguishZeros,
    }
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum IdentifyZeros {
        kDistinguishZeros,
    }
    pub enum void {}
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct BigInt {}
    pub struct BigInt {}
    pub struct BigInt {}
    pub struct Number {}
    pub struct Number {}
    pub struct Number {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct BigInt {}
    pub struct BigInt {}
    pub struct BigInt {}
    pub struct Number {}
    pub struct Number {}
    pub struct Number {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub enum Type {}
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub struct Truncation {
        kind: TruncationKind,
        identify_zeros: IdentifyZeros,
    }
    pub enum Type {}
    pub struct Inputs {}
    pub struct Control {}
    pub struct Simplified {}
    pub struct Simplified {}
    pub struct Simplified {}
    pub struct v8 {}
    pub struct TFGraph {}
    pub struct Node {
        id: usize,
        opcode: IrOpcode,
        op: Operator,
    }
    pub type NodeId = usize;
    pub struct NodeUses<T> {
        uses: Vec<T>,
    }
    impl<T: Clone> NodeUses<T> {
        pub fn new() -> Self {
            NodeUses { uses: Vec::new() }
        }
    }
    pub struct JSHeapBroker {}
    pub struct HeapObjectRef {}
    pub struct MapRef {}
    pub struct SharedFunctionInfoRef {}
    pub struct MachineOperatorBuilder {}
    pub struct Uses {}
    pub enum StandardMember {}
    pub struct EncodedCSignature {}
    pub enum MemoryAccessKind {}
    pub struct BranchSemantics {}
    pub enum CheckForMinusZeroMode {
        kCheckForMinusZero,
        kDontCheckForMinusZero,
    }
    pub struct CheckMinusZeroParameters {
        mode: CheckForMinusZeroMode,
    }
    pub struct SLVerifierHintParameters {
        semantics: Option<Operator>,
        override_output_type: Option<Type>,
    }
    pub fn BranchParametersOf(op: &Operator) -> &Operator {
        op
    }
    impl CheckMinusZeroParameters {
        pub fn mode(&self) -> CheckForMinusZeroMode {
            self.mode
        }
    }
    pub fn CheckMinusZeroParametersOf(op: &Operator) -> &CheckMinusZeroParameters {
        unsafe {
            std::mem::transmute::<&Operator, &CheckMinusZeroParameters>(op)
        }
    }
    pub fn SLVerifierHintParametersOf(op: &Operator) -> &SLVerifierHintParameters {
        unsafe {
            std::mem::transmute::<&Operator, &SLVerifierHintParameters>(op)
        }
    }
    pub fn ValueInputCountOfReturn(op: &Operator) -> i32 {
        0
    }
    pub fn MachineTypesOf(op: &Operator) -> &ZoneVector<MachineType> {
        unsafe {
            std::mem::transmute::<&Operator, &ZoneVector<MachineType>>(op)
        }
    }
    pub struct ZoneVector<T> {
        vec: Vec<T>,
    }
    impl<T> ZoneVector<T> {
        pub fn new(zone: &Zone) -> Self {
            ZoneVector { vec: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.vec.push(value);
        }
        pub fn at(&self, index: usize) -> &T {
            &self.vec[index]
        }
        pub fn size(&self) -> usize {
            self.vec.len()
        }
    }
    pub enum IrOpcode {
        Start,
        IfTrue,
        IfFalse,
        Merge,
        End,
        EffectPhi,
        Checkpoint,
        FrameState,
        JSStackCheck,
        Int32Constant,
        Int64Constant,
        Float64Constant,
        HeapConstant,
        CheckedFloat64ToInt32,
        CheckedTaggedToTaggedSigned,
        CheckedTaggedToTaggedPointer,
        TruncateTaggedToBit,
        Int32Add,
        Int32Sub,
        ChangeInt31ToTaggedSigned,
        ChangeInt32ToTagged,
        ChangeFloat32ToFloat64,
        ChangeInt32ToInt64,
        ChangeUint32ToUint64,
        ChangeUint64ToTagged,
        ChangeFloat64ToInt64,
        Int64Add,
        Int64Sub,
        DeadValue,
        TypeGuard,
        TruncateBigIntToWord64,
        ChangeTaggedSignedToInt64,
        CheckBigInt,
        CheckedBigIntToBigInt64,
        Return,
        SLVerifierHint,
        Branch,
        TypedStateValues,
        Parameter,
        EnterMachineGraph,
        ExitMachineGraph,
        Loop,
        Switch,
        IfSuccess,
        IfException,
        IfValue,
        IfDefault,
        Deoptimize,
        DeoptimizeIf,
        DeoptimizeUnless,
        TrapIf,
        TrapUnless,
        Assert,
        TailCall,
        Terminate,
        Throw,
        TraceInstruction,
        TaggedIndexConstant,
        Float32Constant,
        ExternalConstant,
        NumberConstant,
        PointerConstant,
        CompressedHeapConstant,
        TrustedHeapConstant,
        RelocatableInt32Constant,
        RelocatableInt64Constant,
        Select,
        Phi,
        InductionVariablePhi,
        BeginRegion,
        FinishRegion,
        StateValues,
        ArgumentsElementsState,
        ArgumentsLengthState,
        ObjectState,
        ObjectId,
        TypedObjectState,
        Call,
        OsrValue,
        LoopExit,
        LoopExitValue,
        LoopExitEffect,
        Projection,
        Retain,
        MapGuard,
        Unreachable,
        Dead,
        Plug,
        StaticAssert,
        ChangeTaggedSignedToInt32,
        ChangeTaggedToInt32,
        ChangeTaggedToInt64,
        ChangeTaggedToUint32,
        ChangeTaggedToFloat64,
        ChangeTaggedToTaggedSigned,
        ChangeInt64ToTagged,
        ChangeUint32ToTagged,
        ChangeFloat64ToTagged,
        ChangeFloat64ToTaggedPointer,
        ChangeTaggedToBit,
        ChangeBitToTagged,
        ChangeInt64ToBigInt,
        ChangeUint64ToBigInt,
        TruncateTaggedToWord32,
        TruncateTaggedToFloat64,
        TruncateTaggedPointerToBit,
        CheckedInt32Add,
        CheckedInt32Sub,
        CheckedInt32Div,
        CheckedInt32Mod,
        CheckedUint32Div,
        CheckedUint32Mod,
        CheckedInt32Mul,
        CheckedAdditiveSafeIntegerAdd,
        CheckedAdditiveSafeIntegerSub,
        CheckedInt64Add,
        CheckedInt64Sub,
        CheckedInt64Mul,
        CheckedInt64Div,
        CheckedInt64Mod,
        CheckedInt32ToTaggedSigned,
        CheckedInt64ToInt32,
        CheckedInt64ToTaggedSigned,
        CheckedUint32Bounds,
        CheckedUint32ToInt32,
        CheckedUint32ToTaggedSigned,
        CheckedUint64Bounds,
        CheckedUint64ToInt32,
        CheckedUint64ToInt64,
        CheckedUint64ToTaggedSigned,
        CheckedFloat64ToAdditiveSafeInteger,
        CheckedFloat64ToInt64,
        CheckedTaggedSignedToInt32,
        CheckedTaggedToInt32,
        CheckedTaggedToArrayIndex,
        CheckedTruncateTaggedToWord32,
        CheckedTaggedToFloat64,
        CheckedTaggedToAdditiveSafeInteger,
        CheckedTaggedToInt64,
        NumberLessThan,
        NumberLessThanOrEqual,
        NumberGreaterThan,
        NumberGreaterThanOrEqual,
        NumberEqual,
        NumberNotEqual,
        NumberAdd,
        NumberSubtract,
        NumberMultiply,
        NumberDivide,
        NumberModulus,
        NumberBitwiseOr,
        NumberBitwiseXor,
        NumberBitwiseAnd,
        NumberShiftLeft,
        NumberShiftRight,
        NumberShiftRightLogical,
        BigIntAdd,
        BigIntSubtract,
        BigIntMultiply,
        BigIntDivide,
        BigIntModulus,
        SpeculativeNumberAdd,
        SpeculativeNumberSubtract,
        SpeculativeNumberMultiply,
        SpeculativeNumberDivide,
        SpeculativeNumberModulus,
        NumberAbs,
        NumberRoundDown,
        NumberRoundUp,
        NumberRoundTruncate,
        NumberRoundTiesEven,
        NumberSqrt,
        SpeculativeBigIntAdd,
        SpeculativeBigIntSubtract,
        SpeculativeBigIntMultiply,
        SpeculativeBigIntDivide,
        SpeculativeBigIntModulus,
        PlainPrimitiveToNumber,
        PlainPrimitiveToBigInt,
        NumberToBigInt,
        StringToNumber,
        WeakCellValue,
        LoadField,
        LoadElement,
        LoadTypedElement,
        StoreField,
        StoreElement,
        StoreTypedElement,
        Comment,
        DebugBreak,
        Load,
        LoadImmutable,
        Store,
        StorePair,
        StoreIndirectPointer,
        StackSlot,
        Word32Popcnt,
        Word64Popcnt,
        Word64Clz,
        Word64Ctz,
        Word64ClzLowerable,
        Word64CtzLowerable,
        Word64ReverseBits,
        Word64ReverseBytes,
        Simd128ReverseBytes,
        Int64AbsWithOverflow,
        BitcastTaggedToWord,
        BitcastTaggedToWordForTagAndSmiBits,
        BitcastWordToTagged,
        BitcastWordToTaggedSigned,
        TruncateFloat64ToWord32,
        ChangeFloat64ToInt32,
        ChangeFloat64ToUint32,
        ChangeFloat64ToUint64,
        Float64SilenceNaN,
        TruncateFloat64ToInt64,
        TruncateFloat64ToUint32,
        TruncateFloat32ToInt32,
        TruncateFloat32ToUint32,
        TryTruncateFloat32ToInt64,
        TryTruncateFloat64ToInt64,
        TryTruncateFloat32ToUint64,
        TryTruncateFloat64ToUint64,
        TryTruncateFloat64ToInt32,
        TryTruncateFloat64ToUint32,
        ChangeInt32ToFloat64,
        BitcastWord32ToWord64,
        ChangeInt64ToFloat64,
        ChangeUint32ToFloat64,
        ChangeFloat16RawBitsToFloat64,
        TruncateFloat64ToFloat32,
        TruncateFloat64ToFloat16RawBits,
        TruncateInt64ToInt32,
        RoundFloat64ToInt32,
        RoundInt32ToFloat32,
        RoundInt64ToFloat32,
        RoundInt64ToFloat64,
        RoundUint32ToFloat32,
        RoundUint64ToFloat32,
        RoundUint64ToFloat64,
        BitcastFloat32ToInt32,
        BitcastFloat64ToInt64,
        BitcastInt32ToFloat32,
        BitcastInt64ToFloat64,
        Float64ExtractLowWord32,
        Float64ExtractHighWord32,
        Float64InsertLowWord32,
        Float64InsertHighWord32,
        Word32Select,
        Word64Select,
        Float32Select,
        Float64Select,
        LoadStackCheckOffset,
        LoadFramePointer,
        LoadParentFramePointer,
        LoadRootRegister,
        UnalignedLoad,
        UnalignedStore,
        Int32PairAdd,
        Int32PairSub,
        Int32PairMul,
        Word32PairShl,
        Word32PairShr,
        Word32PairSar,
        ProtectedLoad,
        ProtectedStore,
        LoadTrapOnNull,
        StoreTrapOnNull,
        MemoryBarrier,
        SignExtendWord8ToInt32,
        SignExtendWord16ToInt32,
        SignExtendWord8ToInt64,
        SignExtendWord16ToInt64,
        SignExtendWord32ToInt64,
        StackPointerGreaterThan,
        NumberEqualLoose,
        NumberNotEqualLoose,
        StringAdd,
        StringCharCodeAt,
        StringCodePointAt,
        StringFromCharCode,
        TypeOf,
        ToName,
        ToNumber,
        ToNumeric,
        ToObject,
        ToString,
        CreateArguments,
        CreateArray,
        CreateArrayLiteral,
        CreateEmptyLiteral,
        CreateObject,
        CreateRegExpLiteral,
        LoadContext,
        StoreContext,
        CallJSFunction,
        CallRuntime,
        CallWithContext,
        Construct,
        ConstructWithContext,
        JSAsyncFunctionEnter,
        JSAsyncFunctionReject,
        JSAsyncFunctionResolve,
        JSCallRuntime,
        JSForInEnumerate,
        JSForInNext,
        JSForInPrepare,
        JSGetIterator,
        JSLoadMessage,
        JSStoreMessage,
        JSLoadModule,
        JSStoreModule,
        JSGetImportMeta,
        JSGeneratorStore,
        JSGeneratorRestoreContinuation,
        JSGeneratorRestoreContext,
        JSGeneratorRestoreRegister,
        JSGeneratorRestoreInputOrDebugPos,
        JSFulfillPromise,
        JSPerformPromiseThen,
        JSPromiseResolve,
        JSRejectPromise,
        JSResolvePromise,
        JSObjectIsArray,
        JSRegExpTest,
        JSDebugger,
        Word32And,
        Word32Or,
        Word32Xor,
        Word32Shl,
        Word32Shr,
        Word32Sar,
        Word32Rol,
        Word32Ror,
        Int32AddWithOverflow,
        Int32SubWithOverflow,
        Int32Mul,
        Int32MulWithOverflow,
        Int32MulHigh,
        Int32Div,
        Int32Mod,
        Uint32Div,
        Uint32Mod,
        Uint32MulHigh,
        Word64And,
        Word64Or,
        Word64Xor,
        Word64Shl,
        Word64Shr,
        Word64Sar,
        Word64Rol,
        Word64Ror,
        Word64RolLowerable,
        Word64RorLowerable,
        Int64AddWithOverflow,
        Int64SubWithOverflow,
        Int64Mul,
        Int64MulHigh,
        Uint64MulHigh,
        Int64MulWithOverflow,
        Int64Div,
        Int64Mod,
        Uint64Div,
        Uint64Mod,
        Float32Abs,
        Float32Neg,
        Float32Sqrt,
        Float32Add,
        Float32Sub,
        Float32Mul,
        Float32Div,
        Float32Min,
        Float32Max,
        Float64Abs,
        Float64Neg,
        Float64Sqrt,
        Float64Add,
        Float64Sub,
        Float64Mul,
        Float64Div,
        Float64Min,
        Float64Max,
        AbortCSADcheck,
        TailCall,
        Throw,
        Terminate,
        TrapIf,
        TrapUnless,
        Assert,
        MemoryFence,
        LoadStackPointer,
        SetStackPointer,
        UnalignedStore,
        Simd128AddI8x16,
        Simd128AddI16x8,
        Simd128AddI32x4,
        Simd128AddI64x2,
        Simd128AddF32x4,
        Simd128AddF64x2,
        Simd128SubI8x16,
        Simd128SubI16x8,
        Simd128SubI32x4,
        Simd128SubI64x2,
        Simd128SubF32x4,
        Simd128SubF64x2,
        Simd128MulI8x16,
        Simd128MulI16x8,
        Simd128MulI32x4,
        Simd128MulI64x2,
        Simd128MulF32x4,
        Simd128MulF64x2,
        Simd128DivF32x4,
        Simd128DivF64x2,
        Simd128MinS8x16,
        Simd128MinS16x8,
        Simd128MinS32x4,
        Simd128MinU8x16,
        Simd128MinU16x8,
        Simd128MinU32x4,
        Simd128MinF32x4,
        Simd128MinF64x2,
        Simd128MaxS8x16,
        Simd128MaxS16x8,
        Simd128MaxS32x4,
        Simd128MaxU8x16,
        Simd128MaxU16x8,
        Simd128MaxU32x4,
        Simd128MaxF32x4,
        Simd128MaxF64x2,
        Simd128AvgrU8x16,
        Simd128AvgrU16x8,
        Simd128PminS8x16,
        Simd128PminS16x8,
        Simd128PminS32x4,
        Simd128PminU8x16,
        Simd128PminU16x8,
        Simd128PminU32x4,
        Simd128PmaxS8x16,
        Simd128PmaxS16x8,
        Simd128PmaxS32x4,
        Simd128PmaxU8x16,
        Simd128PmaxU16x8,
        Simd128PmaxU32x4,
        Simd128NegI8x16,
        Simd128NegI16x8,
        Simd128NegI32x4,
        Simd128NegF32x4,
        Simd128NegF64x2,
        Simd128AbsF32x4,
        Simd128AbsF64x2,
        Simd128SqrtF32x4,
        Simd128RecipEstimateF32x4,
        Simd128RecipEstimateF64x2,
        Simd128RecipApproxF32x4,
        Simd128RecipApproxF64x2,
        Simd128RSqrtEstimateF32x4,
        Simd128RSqrtEstimateF64x2,
        Simd128RSqrtApproxF32x4,
        Simd128RSqrtApproxF64x2,
        Simd128And,
        Simd128Or,
        Simd128Xor,
        Simd128Not,
        Simd128AndNot,
        Simd128Bitselect,
        Simd128ShlI8x16,
        Simd128ShlI16x8,
        Simd128ShlI32x4,
        Simd128ShlI64x2,
        Simd128ShrI8x16,
        Simd128ShrI16x8,
        Simd128ShrI32x4,
        Simd128ShrI64x2,
        Simd128SrlI8x16,
        Simd128SrlI16x8,
        Simd128SrlI32x4,
        Simd128SrlI64x2,
        Simd128ExtaddPairwiseI16x8S,
        Simd128ExtaddPairwiseI32x4S,
        Simd128ExtaddPairwiseI16x8U,
        Simd128ExtaddPairwiseI32x4U,
        Simd128AnyTrueI8x16,
        Simd128AnyTrueI16x8,
        Simd128AnyTrueI32x4,
        Simd128AnyTrueI64x2,
        Simd128AllTrueI8x16,
        Simd128AllTrueI16x8,
        Simd128AllTrueI32x4,
        Simd128AllTrueI64x2,
        Simd128SConvertF32x4I32x4,
        Simd128UConvertF32x4I32x4,
        Simd128SConvertF64x2I64x2,
        Simd128UConvertF64x2I64x2,
        Simd128SConvertI32x4F32x4,
        Simd128UConvertI32x4F32x4,
        Simd128SConvertI64x2F64x2,
        Simd128UConvertI64x2F64x2,
        Simd128FloatPromoteLowF32x4,
        Simd128FloatConvertLowF64x2,
        Simd128SplatI8x16,
        Simd128SplatI16x8,
        Simd128SplatI32x4,
        Simd128SplatI64x2,
        Simd128SplatF32x4,
        Simd128SplatF64x2,
        Simd128ReplaceLaneI8x16,
        Simd128ReplaceLaneI16x8,
        Simd128ReplaceLaneI32x4,
        Simd128ReplaceLaneI64x2,
        Simd128ReplaceLaneF32x4,
        Simd128ReplaceLaneF64x2,
        Simd128ExtractLaneI8x16S,
        Simd128ExtractLaneI16x8S,
        Simd128ExtractLaneI32x4S,
        Simd128ExtractLaneI64x2S,
        Simd128ExtractLaneI8x16U,
        Simd128ExtractLaneI16x8U,
        Simd128ExtractLaneI32x4U,
        Simd128ExtractLaneI64x2U,
        Simd128ExtractLaneF32x4,
        Simd128ExtractLaneF64x2,
        Simd128Load8SplatI8x16,
        Simd128Load16SplatI16x8,
        Simd128Load32SplatI32x4,
        Simd128Load64SplatI64x2,
        Simd128Load8LaneI8x16,
        Simd128Load16LaneI16x8,
        Simd128Load32LaneI32x4,
        Simd128Load64LaneI64x2,
        Simd128Load32ZeroI32x4,
        Simd128Load64ZeroI64x2,
        Simd128Load32ZeroF32x4,
        Simd128Load64ZeroF64x2,
        Simd128Load8AlignedI8x16,
        Simd128Load16AlignedI16x8,
        Simd128Load32AlignedI32x4,
        Simd128Load64AlignedI64x2,
        Simd128Load32AlignedF32x4,
        Simd128Load64AlignedF64x2,
        Simd128Load8UnalignedI8x16,
        Simd128Load16UnalignedI16x8,
        Simd128Load32UnalignedI32x4,
        Simd128Load64UnalignedI64x2,
        Simd128Load32UnalignedF32x4,
        Simd128Load64UnalignedF64x2,
        Simd128Store8LaneI8x16,
        Simd128Store16LaneI16x8,
        Simd128Store32LaneI32x4,
        Simd128Store64LaneI64x2,
        Simd128Store8AlignedI8x16,
        Simd128Store16AlignedI16x8,
        Simd128Store32AlignedI32x4,
        Simd128Store64AlignedI64x2,
        Simd128Store32AlignedF32x4,
        Simd128Store64AlignedF64x2,
        Simd128Store8UnalignedI8x16,
        Simd128Store16UnalignedI16x8,
        Simd128Store32UnalignedI32x4,
        Simd128Store64UnalignedI64x2,
        Simd128Store32UnalignedF32x4,
        Simd128Store64UnalignedF64x2,
    }
    impl IrOpcode {
        pub fn is_machine_constant_opcode(self) -> bool {
            match self {
                IrOpcode::Int32Constant | IrOpcode::Int64Constant | IrOpcode::Float64Constant => true,
                _ => false,
            }
        }
    }
    pub struct Operator {
        mnemonic: String,
        opcode: IrOpcode,
    }
    impl Operator {
        pub fn mnemonic(&self) -> &str {
            &self.mnemonic
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
    }
    pub struct Zone {
        id: usize,
    }
    impl Zone {
        pub fn new(id: usize) -> Self {
            Zone { id }
        }
    }
    pub struct NodeProperties {}
    impl NodeProperties {
        pub fn is_typed(node: &Node) -> bool {
            match node.opcode {
                IrOpcode::Int32Constant => true,
                _ => false,
            }
        }
        pub fn get_type(node: &Node) -> Type {
            match node.opcode {
                IrOpcode::Int32Constant => Type::Signed32,
                _ => Type::None,
            }
        }
    }
    pub struct OperationTyper {}
    impl OperationTyper {
        pub fn ToBoolean(&self, input_type: Type) -> Type {
            Type::Boolean
        }
        pub fn ToNumber(&self, input_type: Type) -> Type {
            Type::Number
        }
        pub fn NumberAdd(&self, left_type: Type, right_type: Type) -> Type {
            Type::Number
        }
        pub fn NumberSubtract(&self, left_type: Type, right_type: Type) -> Type {
            Type::Number
        }
        pub fn TypeTypeGuard(&self, op: &Operator, input_type: Type) -> Type {
            input_type
        }
        pub fn BigIntAdd(&self, left_type: Type, right_type: Type) -> Type {
            Type::BigInt
        }
        pub fn BigIntSubtract(&self, left_type: Type, right_type: Type) -> Type {
            Type::BigInt
        }
    }
    pub struct SimplifiedLoweringVerifier {
        hints_: ZoneVector<*mut Node>,
        machine_uses_of_constants_: ZoneUnorderedMap<*mut Node, ZoneVector<*mut Node>>,
        data_: ZoneVector<PerNodeData>,
        graph_: *mut TFGraph,
        zone_: *mut Zone,
    }
    pub struct PerNodeData {
        type_: Option<Type>,
        truncation: Truncation,
    }
    impl SimplifiedLoweringVerifier {
        pub fn new(zone: *mut Zone, graph: *mut TFGraph) -> Self {
            SimplifiedLoweringVerifier {
                hints_: ZoneVector::new(unsafe { &*zone }),
                machine_uses_of_constants_: ZoneUnorderedMap::new(unsafe { &*zone }),
                data_: ZoneVector::new(unsafe { &*zone }),
                graph_: graph,
                zone_: zone,
            }
        }
        pub fn visit_node(&mut self, node: *mut Node, op_typer: &mut OperationTyper) {
            unsafe {

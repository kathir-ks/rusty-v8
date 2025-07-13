// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-scheduler-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct InstructionScheduler {}

impl InstructionScheduler {
    pub fn SchedulerSupported() -> bool {
        true
    }

    pub fn GetTargetInstructionFlags(&self, instr: &Instruction) -> i32 {
        match instr.arch_opcode() {
            ArchOpcode::kS390_Abs32
            | ArchOpcode::kS390_Abs64
            | ArchOpcode::kS390_And32
            | ArchOpcode::kS390_And64
            | ArchOpcode::kS390_Or32
            | ArchOpcode::kS390_Or64
            | ArchOpcode::kS390_Xor32
            | ArchOpcode::kS390_Xor64
            | ArchOpcode::kS390_ShiftLeft32
            | ArchOpcode::kS390_ShiftLeft64
            | ArchOpcode::kS390_ShiftRight32
            | ArchOpcode::kS390_ShiftRight64
            | ArchOpcode::kS390_ShiftRightArith32
            | ArchOpcode::kS390_ShiftRightArith64
            | ArchOpcode::kS390_RotRight32
            | ArchOpcode::kS390_RotRight64
            | ArchOpcode::kS390_Not32
            | ArchOpcode::kS390_Not64
            | ArchOpcode::kS390_RotLeftAndClear64
            | ArchOpcode::kS390_RotLeftAndClearLeft64
            | ArchOpcode::kS390_RotLeftAndClearRight64
            | ArchOpcode::kS390_Lay
            | ArchOpcode::kS390_Add32
            | ArchOpcode::kS390_Add64
            | ArchOpcode::kS390_AddFloat
            | ArchOpcode::kS390_AddDouble
            | ArchOpcode::kS390_Sub32
            | ArchOpcode::kS390_Sub64
            | ArchOpcode::kS390_SubFloat
            | ArchOpcode::kS390_SubDouble
            | ArchOpcode::kS390_Mul32
            | ArchOpcode::kS390_Mul32WithOverflow
            | ArchOpcode::kS390_Mul64
            | ArchOpcode::kS390_Mul64WithOverflow
            | ArchOpcode::kS390_MulHighS64
            | ArchOpcode::kS390_MulHighU64
            | ArchOpcode::kS390_MulHigh32
            | ArchOpcode::kS390_MulHighU32
            | ArchOpcode::kS390_MulFloat
            | ArchOpcode::kS390_MulDouble
            | ArchOpcode::kS390_Div32
            | ArchOpcode::kS390_Div64
            | ArchOpcode::kS390_DivU32
            | ArchOpcode::kS390_DivU64
            | ArchOpcode::kS390_DivFloat
            | ArchOpcode::kS390_DivDouble
            | ArchOpcode::kS390_Mod32
            | ArchOpcode::kS390_Mod64
            | ArchOpcode::kS390_ModU32
            | ArchOpcode::kS390_ModU64
            | ArchOpcode::kS390_ModDouble
            | ArchOpcode::kS390_Neg32
            | ArchOpcode::kS390_Neg64
            | ArchOpcode::kS390_NegDouble
            | ArchOpcode::kS390_NegFloat
            | ArchOpcode::kS390_SqrtFloat
            | ArchOpcode::kS390_FloorFloat
            | ArchOpcode::kS390_CeilFloat
            | ArchOpcode::kS390_TruncateFloat
            | ArchOpcode::kS390_FloatNearestInt
            | ArchOpcode::kS390_AbsFloat
            | ArchOpcode::kS390_SqrtDouble
            | ArchOpcode::kS390_FloorDouble
            | ArchOpcode::kS390_CeilDouble
            | ArchOpcode::kS390_TruncateDouble
            | ArchOpcode::kS390_RoundDouble
            | ArchOpcode::kS390_DoubleNearestInt
            | ArchOpcode::kS390_MaxFloat
            | ArchOpcode::kS390_MaxDouble
            | ArchOpcode::kS390_MinFloat
            | ArchOpcode::kS390_MinDouble
            | ArchOpcode::kS390_AbsDouble
            | ArchOpcode::kS390_Cntlz32
            | ArchOpcode::kS390_Cntlz64
            | ArchOpcode::kS390_Popcnt32
            | ArchOpcode::kS390_Popcnt64
            | ArchOpcode::kS390_Cmp32
            | ArchOpcode::kS390_Cmp64
            | ArchOpcode::kS390_CmpFloat
            | ArchOpcode::kS390_CmpDouble
            | ArchOpcode::kS390_Tst32
            | ArchOpcode::kS390_Tst64
            | ArchOpcode::kS390_SignExtendWord8ToInt32
            | ArchOpcode::kS390_SignExtendWord16ToInt32
            | ArchOpcode::kS390_SignExtendWord8ToInt64
            | ArchOpcode::kS390_SignExtendWord16ToInt64
            | ArchOpcode::kS390_SignExtendWord32ToInt64
            | ArchOpcode::kS390_Uint32ToUint64
            | ArchOpcode::kS390_Int64ToInt32
            | ArchOpcode::kS390_Int64ToFloat32
            | ArchOpcode::kS390_Int64ToDouble
            | ArchOpcode::kS390_Uint64ToFloat32
            | ArchOpcode::kS390_Uint64ToDouble
            | ArchOpcode::kS390_Int32ToFloat32
            | ArchOpcode::kS390_Int32ToDouble
            | ArchOpcode::kS390_Uint32ToFloat32
            | ArchOpcode::kS390_Uint32ToDouble
            | ArchOpcode::kS390_Float32ToInt32
            | ArchOpcode::kS390_Float32ToUint32
            | ArchOpcode::kS390_Float32ToUint64
            | ArchOpcode::kS390_Float32ToDouble
            | ArchOpcode::kS390_Float64SilenceNaN
            | ArchOpcode::kS390_DoubleToInt32
            | ArchOpcode::kS390_DoubleToUint32
            | ArchOpcode::kS390_Float32ToInt64
            | ArchOpcode::kS390_DoubleToInt64
            | ArchOpcode::kS390_DoubleToUint64
            | ArchOpcode::kS390_DoubleToFloat32
            | ArchOpcode::kS390_DoubleExtractLowWord32
            | ArchOpcode::kS390_DoubleExtractHighWord32
            | ArchOpcode::kS390_DoubleFromWord32Pair
            | ArchOpcode::kS390_DoubleInsertLowWord32
            | ArchOpcode::kS390_DoubleInsertHighWord32
            | ArchOpcode::kS390_DoubleConstruct
            | ArchOpcode::kS390_BitcastInt32ToFloat32
            | ArchOpcode::kS390_BitcastFloat32ToInt32
            | ArchOpcode::kS390_BitcastInt64ToDouble
            | ArchOpcode::kS390_BitcastDoubleToInt64
            | ArchOpcode::kS390_LoadReverse16RR
            | ArchOpcode::kS390_LoadReverse32RR
            | ArchOpcode::kS390_LoadReverse64RR
            | ArchOpcode::kS390_LoadReverseSimd128RR
            | ArchOpcode::kS390_LoadAndTestWord32
            | ArchOpcode::kS390_LoadAndTestWord64
            | ArchOpcode::kS390_LoadAndTestFloat32
            | ArchOpcode::kS390_LoadAndTestFloat64
            | ArchOpcode::kS390_F64x2Splat
            | ArchOpcode::kS390_F64x2ReplaceLane
            | ArchOpcode::kS390_F64x2Abs
            | ArchOpcode::kS390_F64x2Neg
            | ArchOpcode::kS390_F64x2Sqrt
            | ArchOpcode::kS390_F64x2Add
            | ArchOpcode::kS390_F64x2Sub
            | ArchOpcode::kS390_F64x2Mul
            | ArchOpcode::kS390_F64x2Div
            | ArchOpcode::kS390_F64x2Eq
            | ArchOpcode::kS390_F64x2Ne
            | ArchOpcode::kS390_F64x2Lt
            | ArchOpcode::kS390_F64x2Le
            | ArchOpcode::kS390_F64x2Min
            | ArchOpcode::kS390_F64x2Max
            | ArchOpcode::kS390_F64x2ExtractLane
            | ArchOpcode::kS390_F64x2Qfma
            | ArchOpcode::kS390_F64x2Qfms
            | ArchOpcode::kS390_F64x2Pmin
            | ArchOpcode::kS390_F64x2Pmax
            | ArchOpcode::kS390_F64x2Ceil
            | ArchOpcode::kS390_F64x2Floor
            | ArchOpcode::kS390_F64x2Trunc
            | ArchOpcode::kS390_F64x2NearestInt
            | ArchOpcode::kS390_F64x2ConvertLowI32x4S
            | ArchOpcode::kS390_F64x2ConvertLowI32x4U
            | ArchOpcode::kS390_F64x2PromoteLowF32x4
            | ArchOpcode::kS390_F32x4Splat
            | ArchOpcode::kS390_F32x4ExtractLane
            | ArchOpcode::kS390_F32x4ReplaceLane
            | ArchOpcode::kS390_F32x4Add
            | ArchOpcode::kS390_F32x4Sub
            | ArchOpcode::kS390_F32x4Mul
            | ArchOpcode::kS390_F32x4Eq
            | ArchOpcode::kS390_F32x4Ne
            | ArchOpcode::kS390_F32x4Lt
            | ArchOpcode::kS390_F32x4Le
            | ArchOpcode::kS390_F32x4Abs
            | ArchOpcode::kS390_F32x4Neg
            | ArchOpcode::kS390_F32x4SConvertI32x4
            | ArchOpcode::kS390_F32x4UConvertI32x4
            | ArchOpcode::kS390_F32x4Sqrt
            | ArchOpcode::kS390_F32x4Div
            | ArchOpcode::kS390_F32x4Min
            | ArchOpcode::kS390_F32x4Max
            | ArchOpcode::kS390_F32x4Qfma
            | ArchOpcode::kS390_F32x4Qfms
            | ArchOpcode::kS390_F32x4Pmin
            | ArchOpcode::kS390_F32x4Pmax
            | ArchOpcode::kS390_F32x4Ceil
            | ArchOpcode::kS390_F32x4Floor
            | ArchOpcode::kS390_F32x4Trunc
            | ArchOpcode::kS390_F32x4NearestInt
            | ArchOpcode::kS390_F32x4DemoteF64x2Zero
            | ArchOpcode::kS390_I64x2Neg
            | ArchOpcode::kS390_I64x2Add
            | ArchOpcode::kS390_I64x2Sub
            | ArchOpcode::kS390_I64x2Shl
            | ArchOpcode::kS390_I64x2ShrS
            | ArchOpcode::kS390_I64x2ShrU
            | ArchOpcode::kS390_I64x2Mul
            | ArchOpcode::kS390_I64x2Splat
            | ArchOpcode::kS390_I64x2ReplaceLane
            | ArchOpcode::kS390_I64x2ExtractLane
            | ArchOpcode::kS390_I64x2Eq
            | ArchOpcode::kS390_I64x2BitMask
            | ArchOpcode::kS390_I64x2ExtMulLowI32x4S
            | ArchOpcode::kS390_I64x2ExtMulHighI32x4S
            | ArchOpcode::kS390_I64x2ExtMulLowI32x4U
            | ArchOpcode::kS390_I64x2ExtMulHighI32x4U
            | ArchOpcode::kS390_I64x2SConvertI32x4Low
            | ArchOpcode::kS390_I64x2SConvertI32x4High
            | ArchOpcode::kS390_I64x2UConvertI32x4Low
            | ArchOpcode::kS390_I64x2UConvertI32x4High
            | ArchOpcode::kS390_I64x2Ne
            | ArchOpcode::kS390_I64x2GtS
            | ArchOpcode::kS390_I64x2GeS
            | ArchOpcode::kS390_I64x2Abs
            | ArchOpcode::kS390_I32x4Splat
            | ArchOpcode::kS390_I32x4ExtractLane
            | ArchOpcode::kS390_I32x4ReplaceLane
            | ArchOpcode::kS390_I32x4Add
            | ArchOpcode::kS390_I32x4Sub
            | ArchOpcode::kS390_I32x4Mul
            | ArchOpcode::kS390_I32x4MinS
            | ArchOpcode::kS390_I32x4MinU
            | ArchOpcode::kS390_I32x4MaxS
            | ArchOpcode::kS390_I32x4MaxU
            | ArchOpcode::kS390_I32x4Eq
            | ArchOpcode::kS390_I32x4Ne
            | ArchOpcode::kS390_I32x4GtS
            | ArchOpcode::kS390_I32x4GeS
            | ArchOpcode::kS390_I32x4GtU
            | ArchOpcode::kS390_I32x4GeU
            | ArchOpcode::kS390_I32x4Shl
            | ArchOpcode::kS390_I32x4ShrS
            | ArchOpcode::kS390_I32x4ShrU
            | ArchOpcode::kS390_I32x4Neg
            | ArchOpcode::kS390_I32x4SConvertF32x4
            | ArchOpcode::kS390_I32x4UConvertF32x4
            | ArchOpcode::kS390_I32x4SConvertI16x8Low
            | ArchOpcode::kS390_I32x4SConvertI16x8High
            | ArchOpcode::kS390_I32x4UConvertI16x8Low
            | ArchOpcode::kS390_I32x4UConvertI16x8High
            | ArchOpcode::kS390_I32x4Abs
            | ArchOpcode::kS390_I32x4BitMask
            | ArchOpcode::kS390_I32x4DotI16x8S
            | ArchOpcode::kS390_I32x4ExtMulLowI16x8S
            | ArchOpcode::kS390_I32x4ExtMulHighI16x8S
            | ArchOpcode::kS390_I32x4ExtMulLowI16x8U
            | ArchOpcode::kS390_I32x4ExtMulHighI16x8U
            | ArchOpcode::kS390_I32x4ExtAddPairwiseI16x8S
            | ArchOpcode::kS390_I32x4ExtAddPairwiseI16x8U
            | ArchOpcode::kS390_I32x4TruncSatF64x2SZero
            | ArchOpcode::kS390_I32x4TruncSatF64x2UZero
            | ArchOpcode::kS390_I32x4DotI8x16AddS
            | ArchOpcode::kS390_I16x8Splat
            | ArchOpcode::kS390_I16x8ExtractLaneU
            | ArchOpcode::kS390_I16x8ExtractLaneS
            | ArchOpcode::kS390_I16x8ReplaceLane
            | ArchOpcode::kS390_I16x8Add
            | ArchOpcode::kS390_I16x8Sub
            | ArchOpcode::kS390_I16x8Mul
            | ArchOpcode::kS390_I16x8MinS
            | ArchOpcode::kS390_I16x8MinU
            | ArchOpcode::kS390_I16x8MaxS
            | ArchOpcode::kS390_I16x8MaxU
            | ArchOpcode::kS390_I16x8Eq
            | ArchOpcode::kS390_I16x8Ne
            | ArchOpcode::kS390_I16x8GtS
            | ArchOpcode::kS390_I16x8GeS
            | ArchOpcode::kS390_I16x8GtU
            | ArchOpcode::kS390_I16x8GeU
            | ArchOpcode::kS390_I16x8Shl
            | ArchOpcode::kS390_I16x8ShrS
            | ArchOpcode::kS390_I16x8ShrU
            | ArchOpcode::kS390_I16x8Neg
            | ArchOpcode::kS390_I16x8SConvertI32x4
            | ArchOpcode::kS390_I16x8UConvertI32x4
            | ArchOpcode::kS390_I16x8SConvertI8x16Low
            | ArchOpcode::kS390_I16x8SConvertI8x16High
            | ArchOpcode::kS390_I16x8UConvertI8x16Low
            | ArchOpcode::kS390_I16x8UConvertI8x16High
            | ArchOpcode::kS390_I16x8AddSatS
            | ArchOpcode::kS390_I16x8SubSatS
            | ArchOpcode::kS390_I16x8AddSatU
            | ArchOpcode::kS390_I16x8SubSatU
            | ArchOpcode::kS390_I16x8RoundingAverageU
            | ArchOpcode::kS390_I16x8Abs
            | ArchOpcode::kS390_I16x8BitMask
            | ArchOpcode::kS390_I16x8ExtMulLowI8x16S
            | ArchOpcode::kS390_I16x8ExtMulHighI8x16S
            | ArchOpcode::kS390_I16x8ExtMulLowI8x16U
            | ArchOpcode::kS390_I16x8ExtMulHighI8x16U
            | ArchOpcode::kS390_I16x8ExtAddPairwiseI8x16S
            | ArchOpcode::kS390_I16x8ExtAddPairwiseI8x16U
            | ArchOpcode::kS390_I16x8Q15MulRSatS
            | ArchOpcode::kS390_I16x8DotI8x16S
            | ArchOpcode::kS390_I8x16Splat
            | ArchOpcode::kS390_I8x16ExtractLaneU
            | ArchOpcode::kS390_I8x16ExtractLaneS
            | ArchOpcode::kS390_I8x16ReplaceLane
            | ArchOpcode::kS390_I8x16Add
            | ArchOpcode::kS390_I8x16Sub
            | ArchOpcode::kS390_I8x16MinS
            | ArchOpcode::kS390_I8x16MinU
            | ArchOpcode::kS390_I8x16MaxS
            | ArchOpcode::kS390_I8x16MaxU
            | ArchOpcode::kS390_I8x16Eq
            | ArchOpcode::kS390_I8x16Ne
            | ArchOpcode::kS390_I8x16GtS
            | ArchOpcode::kS390_I8x16GeS
            | ArchOpcode::kS390_I8x16GtU
            | ArchOpcode::kS390_I8x16GeU
            | ArchOpcode::kS390_I8x16Shl
            | ArchOpcode::kS390_I8x16ShrS
            | ArchOpcode::kS390_I8x16ShrU
            | ArchOpcode::kS390_I8x16Neg
            | ArchOpcode::kS390_I8x16SConvertI16x8
            | ArchOpcode::kS390_I8x16UConvertI16x8
            | ArchOpcode::kS390_I8x16AddSatS
            | ArchOpcode::kS390_I8x16SubSatS
            | ArchOpcode::kS390_I8x16AddSatU
            | ArchOpcode::kS390_I8x16SubSatU
            | ArchOpcode::kS390_I8x16RoundingAverageU
            | ArchOpcode::kS390_I8x16Abs
            | ArchOpcode::kS390_I8x16BitMask
            | ArchOpcode::kS390_I8x16Shuffle
            | ArchOpcode::kS390_I8x16Swizzle
            | ArchOpcode::kS390_I8x16Popcnt
            | ArchOpcode::kS390_I64x2AllTrue
            | ArchOpcode::kS390_I32x4AllTrue
            | ArchOpcode::kS390_I16x8AllTrue
            | ArchOpcode::kS390_I8x16AllTrue
            | ArchOpcode::kS390_V128AnyTrue
            | ArchOpcode::kS390_S128And
            | ArchOpcode::kS390_S128Or
            | ArchOpcode::kS390_S128Xor
            | ArchOpcode::kS390_S128Const
            | ArchOpcode::kS390_S128Zero
            | ArchOpcode::kS390_S128AllOnes
            | ArchOpcode::kS390_S128Not
            | ArchOpcode::kS390_S128Select
            | ArchOpcode::kS390_S128AndNot => 0, // kNoOpcodeFlags

            ArchOpcode::kS390_LoadWordS8
            | ArchOpcode::kS390_LoadWordU8
            | ArchOpcode::kS390_LoadWordS16
            | ArchOpcode::kS390_LoadWordU16
            | ArchOpcode::kS390_LoadWordS32
            | ArchOpcode::kS390_LoadWordU32
            | ArchOpcode::kS390_LoadWord64
            | ArchOpcode::kS390_LoadFloat32
            | ArchOpcode::kS390_LoadDouble
            | ArchOpcode::kS390_LoadSimd128
            | ArchOpcode::kS390_LoadReverse16
            | ArchOpcode::kS390_LoadReverse32
            | ArchOpcode::kS390_LoadReverse64
            | ArchOpcode::kS390_LoadReverseSimd128
            | ArchOpcode::kS390_Peek
            | ArchOpcode::kS390_LoadDecompressTaggedSigned
            | ArchOpcode::kS390_LoadDecompressTagged
            | ArchOpcode::kS390_S128Load8Splat
            | ArchOpcode::kS390_S128Load16Splat
            | ArchOpcode::kS390_S128Load32Splat
            | ArchOpcode::kS390_S128Load64Splat
            | ArchOpcode::kS390_S128Load8x8S
            | ArchOpcode::kS390_S128Load8x8U
            | ArchOpcode::kS390_S128Load16x4S
            | ArchOpcode::kS390_S128Load16x4U
            | ArchOpcode::kS390_S128Load32x2S
            | ArchOpcode::kS390_S128Load32x2U
            | ArchOpcode::kS390_S128Load32Zero
            | ArchOpcode::kS390_S128Load64Zero
            | ArchOpcode::kS390_S128Load8Lane
            | ArchOpcode::kS390_S128Load16Lane
            | ArchOpcode::kS390_S128Load32Lane
            | ArchOpcode::kS390_S128Load64Lane => 1, // kIsLoadOperation

            ArchOpcode::kS390_StoreWord8
            | ArchOpcode::kS390_StoreWord16
            | ArchOpcode::kS390_StoreWord32
            | ArchOpcode::kS390_StoreWord64
            | ArchOpcode::kS390_StoreReverseSimd128
            | ArchOpcode::kS390_StoreReverse16
            | ArchOpcode::kS390_StoreReverse32
            | ArchOpcode::kS390_StoreReverse64
            | ArchOpcode::kS390_StoreFloat32
            | ArchOpcode::kS390_StoreDouble
            | ArchOpcode::kS390_StoreSimd128
            | ArchOpcode::kS390_StoreCompressTagged
            | ArchOpcode::kS390_Push
            | ArchOpcode::kS390_PushFrame
            | ArchOpcode::kS390_StoreToStackSlot
            | ArchOpcode::kS390_S128Store8Lane
            | ArchOpcode::kS390_S128Store16Lane
            | ArchOpcode::kS390_S128Store32Lane
            | ArchOpcode::kS390_S128Store64Lane => 2, // kHasSideEffect

            ArchOpcode::kS390_Word64AtomicExchangeUint64
            | ArchOpcode::kS390_Word64AtomicCompareExchangeUint64
            | ArchOpcode::kS390_Word64AtomicAddUint64
            | ArchOpcode::kS390_Word64AtomicSubUint64
            | ArchOpcode::kS390_Word64AtomicAndUint64
            | ArchOpcode::kS390_Word64AtomicOrUint64
            | ArchOpcode::kS390_Word64AtomicXorUint64 => 2, // kHasSideEffect

            _ => {
              0
            }
        }
    }

    pub fn GetInstructionLatency(&self, _instr: &Instruction) -> i32 {
        1
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ArchOpcode {
    kS390_Abs32,
    kS390_Abs64,
    kS390_And32,
    kS390_And64,
    kS390_Or32,
    kS390_Or64,
    kS390_Xor32,
    kS390_Xor64,
    kS390_ShiftLeft32,
    kS390_ShiftLeft64,
    kS390_ShiftRight32,
    kS390_ShiftRight64,
    kS390_ShiftRightArith32,
    kS390_ShiftRightArith64,
    kS390_RotRight32,
    kS390_RotRight64,
    kS390_Not32,
    kS390_Not64,
    kS390_RotLeftAndClear64,
    kS390_RotLeftAndClearLeft64,
    kS390_RotLeftAndClearRight64,
    kS390_Lay,
    kS390_Add32,
    kS390_Add64,
    kS390_AddFloat,
    kS390_AddDouble,
    kS390_Sub32,
    kS390_Sub64,
    kS390_SubFloat,
    kS390_SubDouble,
    kS390_Mul32,
    kS390_Mul32WithOverflow,
    kS390_Mul64,
    k

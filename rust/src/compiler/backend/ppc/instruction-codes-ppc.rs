// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod instruction_codes_ppc {

    /// PPC-specific opcodes that specify which assembly sequence to emit.
    /// Most opcodes specify a single instruction.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PPCOpcode {
        PPC_Peek,
        PPC_Sync,
        PPC_And,
        PPC_AndComplement,
        PPC_Or,
        PPC_OrComplement,
        PPC_Xor,
        PPC_ShiftLeft32,
        PPC_ShiftLeft64,
        PPC_ShiftLeftPair,
        PPC_ShiftRight32,
        PPC_ShiftRight64,
        PPC_ShiftRightPair,
        PPC_ShiftRightAlg32,
        PPC_ShiftRightAlg64,
        PPC_ShiftRightAlgPair,
        PPC_RotRight32,
        PPC_RotRight64,
        PPC_Not,
        PPC_RotLeftAndMask32,
        PPC_RotLeftAndClear64,
        PPC_RotLeftAndClearLeft64,
        PPC_RotLeftAndClearRight64,
        PPC_Add32,
        PPC_Add64,
        PPC_AddWithOverflow32,
        PPC_AddPair,
        PPC_AddDouble,
        PPC_Sub,
        PPC_SubWithOverflow32,
        PPC_SubPair,
        PPC_SubDouble,
        PPC_Mul32,
        PPC_Mul32WithHigh32,
        PPC_Mul64,
        PPC_MulHighS64,
        PPC_MulHighU64,
        PPC_MulHigh32,
        PPC_MulHighU32,
        PPC_MulPair,
        PPC_MulDouble,
        PPC_Div32,
        PPC_Div64,
        PPC_DivU32,
        PPC_DivU64,
        PPC_DivDouble,
        PPC_Mod32,
        PPC_Mod64,
        PPC_ModU32,
        PPC_ModU64,
        PPC_ModDouble,
        PPC_Neg,
        PPC_NegDouble,
        PPC_SqrtDouble,
        PPC_FloorDouble,
        PPC_CeilDouble,
        PPC_TruncateDouble,
        PPC_RoundDouble,
        PPC_MaxDouble,
        PPC_MinDouble,
        PPC_AbsDouble,
        PPC_Cntlz32,
        PPC_Cntlz64,
        PPC_Popcnt32,
        PPC_Popcnt64,
        PPC_Cmp32,
        PPC_Cmp64,
        PPC_CmpDouble,
        PPC_Tst32,
        PPC_Tst64,
        PPC_Push,
        PPC_PushFrame,
        PPC_StoreToStackSlot,
        PPC_ExtendSignWord8,
        PPC_ExtendSignWord16,
        PPC_ExtendSignWord32,
        PPC_Uint32ToUint64,
        PPC_Int64ToInt32,
        PPC_Int64ToFloat32,
        PPC_Int64ToDouble,
        PPC_Uint64ToFloat32,
        PPC_Uint64ToDouble,
        PPC_Int32ToFloat32,
        PPC_Int32ToDouble,
        PPC_Uint32ToFloat32,
        PPC_Float32ToInt32,
        PPC_Float32ToUint32,
        PPC_Uint32ToDouble,
        PPC_Float32ToDouble,
        PPC_Float64SilenceNaN,
        PPC_DoubleToInt32,
        PPC_DoubleToUint32,
        PPC_DoubleToInt64,
        PPC_DoubleToUint64,
        PPC_DoubleToFloat32,
        PPC_DoubleExtractLowWord32,
        PPC_DoubleExtractHighWord32,
        PPC_DoubleFromWord32Pair,
        PPC_DoubleInsertLowWord32,
        PPC_DoubleInsertHighWord32,
        PPC_DoubleConstruct,
        PPC_BitcastInt32ToFloat32,
        PPC_BitcastFloat32ToInt32,
        PPC_BitcastInt64ToDouble,
        PPC_BitcastDoubleToInt64,
        PPC_LoadWordS8,
        PPC_LoadWordU8,
        PPC_LoadWordS16,
        PPC_LoadWordU16,
        PPC_LoadWordS32,
        PPC_LoadWordU32,
        PPC_LoadByteRev32,
        PPC_LoadWord64,
        PPC_LoadByteRev64,
        PPC_LoadFloat32,
        PPC_LoadDouble,
        PPC_LoadSimd128,
        PPC_LoadReverseSimd128RR,
        PPC_StoreWord8,
        PPC_StoreWord16,
        PPC_StoreWord32,
        PPC_StoreByteRev32,
        PPC_StoreWord64,
        PPC_StoreByteRev64,
        PPC_StoreFloat32,
        PPC_StoreDouble,
        PPC_StoreSimd128,
        PPC_ByteRev32,
        PPC_ByteRev64,
        PPC_AtomicExchangeUint8,
        PPC_AtomicExchangeUint16,
        PPC_AtomicExchangeWord32,
        PPC_AtomicExchangeWord64,
        PPC_AtomicCompareExchangeUint8,
        PPC_AtomicCompareExchangeUint16,
        PPC_AtomicCompareExchangeWord32,
        PPC_AtomicCompareExchangeWord64,
        PPC_AtomicAddUint8,
        PPC_AtomicAddUint16,
        PPC_AtomicAddUint32,
        PPC_AtomicAddUint64,
        PPC_AtomicAddInt8,
        PPC_AtomicAddInt16,
        PPC_AtomicAddInt32,
        PPC_AtomicAddInt64,
        PPC_AtomicSubUint8,
        PPC_AtomicSubUint16,
        PPC_AtomicSubUint32,
        PPC_AtomicSubUint64,
        PPC_AtomicSubInt8,
        PPC_AtomicSubInt16,
        PPC_AtomicSubInt32,
        PPC_AtomicSubInt64,
        PPC_AtomicAndUint8,
        PPC_AtomicAndUint16,
        PPC_AtomicAndUint32,
        PPC_AtomicAndUint64,
        PPC_AtomicAndInt8,
        PPC_AtomicAndInt16,
        PPC_AtomicAndInt32,
        PPC_AtomicAndInt64,
        PPC_AtomicOrUint8,
        PPC_AtomicOrUint16,
        PPC_AtomicOrUint32,
        PPC_AtomicOrUint64,
        PPC_AtomicOrInt8,
        PPC_AtomicOrInt16,
        PPC_AtomicOrInt32,
        PPC_AtomicOrInt64,
        PPC_AtomicXorUint8,
        PPC_AtomicXorUint16,
        PPC_AtomicXorUint32,
        PPC_AtomicXorUint64,
        PPC_AtomicXorInt8,
        PPC_AtomicXorInt16,
        PPC_AtomicXorInt32,
        PPC_AtomicXorInt64,
        PPC_F64x2Add,
        PPC_F64x2Sub,
        PPC_F64x2Mul,
        PPC_F64x2Eq,
        PPC_F64x2Ne,
        PPC_F64x2Le,
        PPC_F64x2Lt,
        PPC_F64x2Abs,
        PPC_F64x2Neg,
        PPC_F64x2Sqrt,
        PPC_F64x2Qfma,
        PPC_F64x2Qfms,
        PPC_F64x2Div,
        PPC_F64x2Min,
        PPC_F64x2Max,
        PPC_F64x2Ceil,
        PPC_F64x2Floor,
        PPC_F64x2Trunc,
        PPC_F64x2Pmin,
        PPC_F64x2Pmax,
        PPC_F64x2ConvertLowI32x4S,
        PPC_F64x2ConvertLowI32x4U,
        PPC_F64x2PromoteLowF32x4,
        PPC_F32x4Add,
        PPC_F32x4Sub,
        PPC_F32x4Mul,
        PPC_F32x4Eq,
        PPC_F32x4Ne,
        PPC_F32x4Lt,
        PPC_F32x4Le,
        PPC_F32x4Abs,
        PPC_F32x4Neg,
        PPC_F32x4Sqrt,
        PPC_F32x4SConvertI32x4,
        PPC_F32x4UConvertI32x4,
        PPC_F32x4Div,
        PPC_F32x4Min,
        PPC_F32x4Max,
        PPC_F32x4Ceil,
        PPC_F32x4Floor,
        PPC_F32x4Trunc,
        PPC_F32x4Pmin,
        PPC_F32x4Pmax,
        PPC_F32x4Qfma,
        PPC_F32x4Qfms,
        PPC_F32x4DemoteF64x2Zero,
        PPC_I64x2Add,
        PPC_I64x2Sub,
        PPC_I64x2Mul,
        PPC_I64x2Eq,
        PPC_I64x2Ne,
        PPC_I64x2GtS,
        PPC_I64x2GeS,
        PPC_I64x2Shl,
        PPC_I64x2ShrS,
        PPC_I64x2ShrU,
        PPC_I64x2Neg,
        PPC_I64x2BitMask,
        PPC_I64x2SConvertI32x4Low,
        PPC_I64x2SConvertI32x4High,
        PPC_I64x2UConvertI32x4Low,
        PPC_I64x2UConvertI32x4High,
        PPC_I64x2ExtMulLowI32x4S,
        PPC_I64x2ExtMulHighI32x4S,
        PPC_I64x2ExtMulLowI32x4U,
        PPC_I64x2ExtMulHighI32x4U,
        PPC_I64x2Abs,
        PPC_I32x4Add,
        PPC_I32x4Sub,
        PPC_I32x4Mul,
        PPC_I32x4MinS,
        PPC_I32x4MinU,
        PPC_I32x4MaxS,
        PPC_I32x4MaxU,
        PPC_I32x4Eq,
        PPC_I32x4Ne,
        PPC_I32x4GtS,
        PPC_I32x4GeS,
        PPC_I32x4GtU,
        PPC_I32x4GeU,
        PPC_I32x4Shl,
        PPC_I32x4ShrS,
        PPC_I32x4ShrU,
        PPC_I32x4Neg,
        PPC_I32x4Abs,
        PPC_I32x4SConvertF32x4,
        PPC_I32x4UConvertF32x4,
        PPC_I32x4SConvertI16x8Low,
        PPC_I32x4SConvertI16x8High,
        PPC_I32x4UConvertI16x8Low,
        PPC_I32x4UConvertI16x8High,
        PPC_I32x4BitMask,
        PPC_I32x4DotI16x8S,
        PPC_I32x4ExtAddPairwiseI16x8S,
        PPC_I32x4ExtAddPairwiseI16x8U,
        PPC_I32x4ExtMulLowI16x8S,
        PPC_I32x4ExtMulHighI16x8S,
        PPC_I32x4ExtMulLowI16x8U,
        PPC_I32x4ExtMulHighI16x8U,
        PPC_I32x4TruncSatF64x2SZero,
        PPC_I32x4TruncSatF64x2UZero,
        PPC_I32x4DotI8x16AddS,
        PPC_I16x8Add,
        PPC_I16x8Sub,
        PPC_I16x8Mul,
        PPC_I16x8MinS,
        PPC_I16x8MinU,
        PPC_I16x8MaxS,
        PPC_I16x8MaxU,
        PPC_I16x8Eq,
        PPC_I16x8Ne,
        PPC_I16x8GtS,
        PPC_I16x8GeS,
        PPC_I16x8GtU,
        PPC_I16x8GeU,
        PPC_I16x8Shl,
        PPC_I16x8ShrS,
        PPC_I16x8ShrU,
        PPC_I16x8Neg,
        PPC_I16x8Abs,
        PPC_I16x8SConvertI32x4,
        PPC_I16x8UConvertI32x4,
        PPC_I16x8SConvertI8x16Low,
        PPC_I16x8SConvertI8x16High,
        PPC_I16x8UConvertI8x16Low,
        PPC_I16x8UConvertI8x16High,
        PPC_I16x8AddSatS,
        PPC_I16x8SubSatS,
        PPC_I16x8AddSatU,
        PPC_I16x8SubSatU,
        PPC_I16x8RoundingAverageU,
        PPC_I16x8BitMask,
        PPC_I16x8ExtAddPairwiseI8x16S,
        PPC_I16x8ExtAddPairwiseI8x16U,
        PPC_I16x8Q15MulRSatS,
        PPC_I16x8ExtMulLowI8x16S,
        PPC_I16x8ExtMulHighI8x16S,
        PPC_I16x8ExtMulLowI8x16U,
        PPC_I16x8ExtMulHighI8x16U,
        PPC_I16x8DotI8x16S,
        PPC_I8x16Add,
        PPC_I8x16Sub,
        PPC_I8x16MinS,
        PPC_I8x16MinU,
        PPC_I8x16MaxS,
        PPC_I8x16MaxU,
        PPC_I8x16Eq,
        PPC_I8x16Ne,
        PPC_I8x16GtS,
        PPC_I8x16GeS,
        PPC_I8x16GtU,
        PPC_I8x16GeU,
        PPC_I8x16Shl,
        PPC_I8x16ShrS,
        PPC_I8x16ShrU,
        PPC_I8x16Neg,
        PPC_I8x16Abs,
        PPC_I8x16SConvertI16x8,
        PPC_I8x16UConvertI16x8,
        PPC_I8x16AddSatS,
        PPC_I8x16SubSatS,
        PPC_I8x16AddSatU,
        PPC_I8x16SubSatU,
        PPC_I8x16RoundingAverageU,
        PPC_I8x16Shuffle,
        PPC_I8x16Swizzle,
        PPC_I8x16BitMask,
        PPC_I8x16Popcnt,
        PPC_I64x2AllTrue,
        PPC_I32x4AllTrue,
        PPC_I16x8AllTrue,
        PPC_I8x16AllTrue,
        PPC_V128AnyTrue,
        PPC_S128And,
        PPC_S128Or,
        PPC_S128Xor,
        PPC_S128Const,
        PPC_S128Zero,
        PPC_S128AllOnes,
        PPC_S128Not,
        PPC_S128Select,
        PPC_S128AndNot,
        PPC_S128Load8Splat,
        PPC_S128Load16Splat,
        PPC_S128Load32Splat,
        PPC_S128Load64Splat,
        PPC_S128Load8x8S,
        PPC_S128Load8x8U,
        PPC_S128Load16x4S,
        PPC_S128Load16x4U,
        PPC_S128Load32x2S,
        PPC_S128Load32x2U,
        PPC_S128Load32Zero,
        PPC_S128Load64Zero,
        PPC_S128Load8Lane,
        PPC_S128Load16Lane,
        PPC_S128Load32Lane,
        PPC_S128Load64Lane,
        PPC_S128Store8Lane,
        PPC_S128Store16Lane,
        PPC_S128Store32Lane,
        PPC_S128Store64Lane,
        PPC_FExtractLane,
        PPC_IExtractLane,
        PPC_IExtractLaneU,
        PPC_IExtractLaneS,
        PPC_FReplaceLane,
        PPC_IReplaceLane,
        PPC_FSplat,
        PPC_ISplat,
        PPC_StoreCompressTagged,
        PPC_StoreIndirectPointer,
        PPC_LoadDecodeSandboxedPointer,
        PPC_StoreEncodeSandboxedPointer,
        PPC_LoadDecompressTaggedSigned,
        PPC_LoadDecompressTagged,
    }

    /// Addressing modes represent the "shape" of inputs to an instruction.
    /// Many instructions support multiple addressing modes. Addressing modes
    /// are encoded into the InstructionCode of the instruction and tell the
    /// code generator after register allocation which assembler method to call.
    ///
    /// We use the following local notation for addressing modes:
    ///
    /// R = register
    /// O = register or stack slot
    /// D = double register
    /// I = immediate (handle, external, int32)
    /// MRI = [register + immediate]
    /// MRR = [register + register]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PPCAddressingMode {
        MRI,   /* [%r0 + K] */
        MRR,   /* [%r0 + %r1] */
        Root,  /* [%rr + K] */
    }
}
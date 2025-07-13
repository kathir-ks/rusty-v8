// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-scheduler-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::result;

pub struct InstructionScheduler {}

pub enum InstructionSchedulerError {
    UnsupportedInstruction,
}

impl InstructionScheduler {
    pub fn SchedulerSupported() -> bool {
        true
    }

    pub fn GetTargetInstructionFlags(
        &self,
        instr: &Instruction,
    ) -> Result<i32, InstructionSchedulerError> {
        match instr.arch_opcode {
            ArchOpcode::kArmAdd
            | ArchOpcode::kArmAnd
            | ArchOpcode::kArmBic
            | ArchOpcode::kArmClz
            | ArchOpcode::kArmCmp
            | ArchOpcode::kArmCmn
            | ArchOpcode::kArmTst
            | ArchOpcode::kArmTeq
            | ArchOpcode::kArmOrr
            | ArchOpcode::kArmEor
            | ArchOpcode::kArmSub
            | ArchOpcode::kArmRsb
            | ArchOpcode::kArmMul
            | ArchOpcode::kArmMla
            | ArchOpcode::kArmMls
            | ArchOpcode::kArmSmmul
            | ArchOpcode::kArmSmull
            | ArchOpcode::kArmSmmla
            | ArchOpcode::kArmUmull
            | ArchOpcode::kArmSdiv
            | ArchOpcode::kArmUdiv
            | ArchOpcode::kArmMov
            | ArchOpcode::kArmMvn
            | ArchOpcode::kArmBfc
            | ArchOpcode::kArmUbfx
            | ArchOpcode::kArmSbfx
            | ArchOpcode::kArmSxtb
            | ArchOpcode::kArmSxth
            | ArchOpcode::kArmSxtab
            | ArchOpcode::kArmSxtah
            | ArchOpcode::kArmUxtb
            | ArchOpcode::kArmUxth
            | ArchOpcode::kArmUxtab
            | ArchOpcode::kArmUxtah
            | ArchOpcode::kArmRbit
            | ArchOpcode::kArmRev
            | ArchOpcode::kArmAddPair
            | ArchOpcode::kArmSubPair
            | ArchOpcode::kArmMulPair
            | ArchOpcode::kArmLslPair
            | ArchOpcode::kArmLsrPair
            | ArchOpcode::kArmAsrPair
            | ArchOpcode::kArmVcmpF32
            | ArchOpcode::kArmVaddF32
            | ArchOpcode::kArmVsubF32
            | ArchOpcode::kArmVmulF32
            | ArchOpcode::kArmVmlaF32
            | ArchOpcode::kArmVmlsF32
            | ArchOpcode::kArmVdivF32
            | ArchOpcode::kArmVabsF32
            | ArchOpcode::kArmVnegF32
            | ArchOpcode::kArmVsqrtF32
            | ArchOpcode::kArmVcmpF64
            | ArchOpcode::kArmVaddF64
            | ArchOpcode::kArmVsubF64
            | ArchOpcode::kArmVmulF64
            | ArchOpcode::kArmVmlaF64
            | ArchOpcode::kArmVmlsF64
            | ArchOpcode::kArmVdivF64
            | ArchOpcode::kArmVmodF64
            | ArchOpcode::kArmVabsF64
            | ArchOpcode::kArmVnegF64
            | ArchOpcode::kArmVsqrtF64
            | ArchOpcode::kArmVmullLow
            | ArchOpcode::kArmVmullHigh
            | ArchOpcode::kArmVrintmF32
            | ArchOpcode::kArmVrintmF64
            | ArchOpcode::kArmVrintpF32
            | ArchOpcode::kArmVrintpF64
            | ArchOpcode::kArmVrintzF32
            | ArchOpcode::kArmVrintzF64
            | ArchOpcode::kArmVrintaF64
            | ArchOpcode::kArmVrintnF32
            | ArchOpcode::kArmVrintnF64
            | ArchOpcode::kArmVcvtF32F64
            | ArchOpcode::kArmVcvtF64F32
            | ArchOpcode::kArmVcvtF32S32
            | ArchOpcode::kArmVcvtF32U32
            | ArchOpcode::kArmVcvtF64S32
            | ArchOpcode::kArmVcvtF64U32
            | ArchOpcode::kArmVcvtS32F32
            | ArchOpcode::kArmVcvtU32F32
            | ArchOpcode::kArmVcvtS32F64
            | ArchOpcode::kArmVcvtU32F64
            | ArchOpcode::kArmVmovU32F32
            | ArchOpcode::kArmVmovF32U32
            | ArchOpcode::kArmVmovLowU32F64
            | ArchOpcode::kArmVmovLowF64U32
            | ArchOpcode::kArmVmovHighU32F64
            | ArchOpcode::kArmVmovHighF64U32
            | ArchOpcode::kArmVmovF64U32U32
            | ArchOpcode::kArmVmovU32U32F64
            | ArchOpcode::kArmVcnt
            | ArchOpcode::kArmVpadal
            | ArchOpcode::kArmVpaddl
            | ArchOpcode::kArmFloat32Max
            | ArchOpcode::kArmFloat64Max
            | ArchOpcode::kArmFloat32Min
            | ArchOpcode::kArmFloat64Min
            | ArchOpcode::kArmFloat64SilenceNaN
            | ArchOpcode::kArmF64x2Splat
            | ArchOpcode::kArmF64x2ExtractLane
            | ArchOpcode::kArmF64x2ReplaceLane
            | ArchOpcode::kArmF64x2Abs
            | ArchOpcode::kArmF64x2Neg
            | ArchOpcode::kArmF64x2Sqrt
            | ArchOpcode::kArmF64x2Add
            | ArchOpcode::kArmF64x2Sub
            | ArchOpcode::kArmF64x2Mul
            | ArchOpcode::kArmF64x2Div
            | ArchOpcode::kArmF64x2Min
            | ArchOpcode::kArmF64x2Max
            | ArchOpcode::kArmF64x2Eq
            | ArchOpcode::kArmF64x2Ne
            | ArchOpcode::kArmF64x2Lt
            | ArchOpcode::kArmF64x2Le
            | ArchOpcode::kArmF64x2Qfma
            | ArchOpcode::kArmF64x2Qfms
            | ArchOpcode::kArmF64x2Pmin
            | ArchOpcode::kArmF64x2Pmax
            | ArchOpcode::kArmF64x2Ceil
            | ArchOpcode::kArmF64x2Floor
            | ArchOpcode::kArmF64x2Trunc
            | ArchOpcode::kArmF64x2NearestInt
            | ArchOpcode::kArmF64x2ConvertLowI32x4S
            | ArchOpcode::kArmF64x2ConvertLowI32x4U
            | ArchOpcode::kArmF64x2PromoteLowF32x4
            | ArchOpcode::kArmF32x4Splat
            | ArchOpcode::kArmF32x4ExtractLane
            | ArchOpcode::kArmF32x4ReplaceLane
            | ArchOpcode::kArmF32x4SConvertI32x4
            | ArchOpcode::kArmF32x4UConvertI32x4
            | ArchOpcode::kArmF32x4Abs
            | ArchOpcode::kArmF32x4Neg
            | ArchOpcode::kArmF32x4Sqrt
            | ArchOpcode::kArmF32x4Add
            | ArchOpcode::kArmF32x4Sub
            | ArchOpcode::kArmF32x4Mul
            | ArchOpcode::kArmF32x4Div
            | ArchOpcode::kArmF32x4Min
            | ArchOpcode::kArmF32x4Max
            | ArchOpcode::kArmF32x4Eq
            | ArchOpcode::kArmF32x4Ne
            | ArchOpcode::kArmF32x4Lt
            | ArchOpcode::kArmF32x4Le
            | ArchOpcode::kArmF32x4Qfma
            | ArchOpcode::kArmF32x4Qfms
            | ArchOpcode::kArmF32x4Pmin
            | ArchOpcode::kArmF32x4Pmax
            | ArchOpcode::kArmF32x4DemoteF64x2Zero
            | ArchOpcode::kArmI64x2SplatI32Pair
            | ArchOpcode::kArmI64x2ReplaceLaneI32Pair
            | ArchOpcode::kArmI64x2Abs
            | ArchOpcode::kArmI64x2Neg
            | ArchOpcode::kArmI64x2Shl
            | ArchOpcode::kArmI64x2ShrS
            | ArchOpcode::kArmI64x2Add
            | ArchOpcode::kArmI64x2Sub
            | ArchOpcode::kArmI64x2Mul
            | ArchOpcode::kArmI64x2ShrU
            | ArchOpcode::kArmI64x2BitMask
            | ArchOpcode::kArmI64x2Eq
            | ArchOpcode::kArmI64x2Ne
            | ArchOpcode::kArmI64x2GtS
            | ArchOpcode::kArmI64x2GeS
            | ArchOpcode::kArmI64x2SConvertI32x4Low
            | ArchOpcode::kArmI64x2SConvertI32x4High
            | ArchOpcode::kArmI64x2UConvertI32x4Low
            | ArchOpcode::kArmI64x2UConvertI32x4High
            | ArchOpcode::kArmI32x4Splat
            | ArchOpcode::kArmI32x4ExtractLane
            | ArchOpcode::kArmI32x4ReplaceLane
            | ArchOpcode::kArmI32x4SConvertF32x4
            | ArchOpcode::kArmI32x4SConvertI16x8Low
            | ArchOpcode::kArmI32x4SConvertI16x8High
            | ArchOpcode::kArmI32x4Neg
            | ArchOpcode::kArmI32x4Shl
            | ArchOpcode::kArmI32x4ShrS
            | ArchOpcode::kArmI32x4Add
            | ArchOpcode::kArmI32x4Sub
            | ArchOpcode::kArmI32x4Mul
            | ArchOpcode::kArmI32x4MinS
            | ArchOpcode::kArmI32x4MaxS
            | ArchOpcode::kArmI32x4Eq
            | ArchOpcode::kArmI32x4Ne
            | ArchOpcode::kArmI32x4GtS
            | ArchOpcode::kArmI32x4GeS
            | ArchOpcode::kArmI32x4UConvertF32x4
            | ArchOpcode::kArmI32x4UConvertI16x8Low
            | ArchOpcode::kArmI32x4UConvertI16x8High
            | ArchOpcode::kArmI32x4ShrU
            | ArchOpcode::kArmI32x4MinU
            | ArchOpcode::kArmI32x4MaxU
            | ArchOpcode::kArmI32x4GtU
            | ArchOpcode::kArmI32x4GeU
            | ArchOpcode::kArmI32x4Abs
            | ArchOpcode::kArmI32x4BitMask
            | ArchOpcode::kArmI32x4DotI16x8S
            | ArchOpcode::kArmI16x8DotI8x16S
            | ArchOpcode::kArmI32x4DotI8x16AddS
            | ArchOpcode::kArmI32x4TruncSatF64x2SZero
            | ArchOpcode::kArmI32x4TruncSatF64x2UZero
            | ArchOpcode::kArmI16x8Splat
            | ArchOpcode::kArmI16x8ExtractLaneS
            | ArchOpcode::kArmI16x8ReplaceLane
            | ArchOpcode::kArmI16x8SConvertI8x16Low
            | ArchOpcode::kArmI16x8SConvertI8x16High
            | ArchOpcode::kArmI16x8Neg
            | ArchOpcode::kArmI16x8Shl
            | ArchOpcode::kArmI16x8ShrS
            | ArchOpcode::kArmI16x8SConvertI32x4
            | ArchOpcode::kArmI16x8Add
            | ArchOpcode::kArmI16x8AddSatS
            | ArchOpcode::kArmI16x8Sub
            | ArchOpcode::kArmI16x8SubSatS
            | ArchOpcode::kArmI16x8Mul
            | ArchOpcode::kArmI16x8MinS
            | ArchOpcode::kArmI16x8MaxS
            | ArchOpcode::kArmI16x8Eq
            | ArchOpcode::kArmI16x8Ne
            | ArchOpcode::kArmI16x8GtS
            | ArchOpcode::kArmI16x8GeS
            | ArchOpcode::kArmI16x8ExtractLaneU
            | ArchOpcode::kArmI16x8UConvertI8x16Low
            | ArchOpcode::kArmI16x8UConvertI8x16High
            | ArchOpcode::kArmI16x8ShrU
            | ArchOpcode::kArmI16x8UConvertI32x4
            | ArchOpcode::kArmI16x8AddSatU
            | ArchOpcode::kArmI16x8SubSatU
            | ArchOpcode::kArmI16x8MinU
            | ArchOpcode::kArmI16x8MaxU
            | ArchOpcode::kArmI16x8GtU
            | ArchOpcode::kArmI16x8GeU
            | ArchOpcode::kArmI16x8RoundingAverageU
            | ArchOpcode::kArmI16x8Abs
            | ArchOpcode::kArmI16x8BitMask
            | ArchOpcode::kArmI16x8Q15MulRSatS
            | ArchOpcode::kArmI8x16Splat
            | ArchOpcode::kArmI8x16ExtractLaneS
            | ArchOpcode::kArmI8x16ReplaceLane
            | ArchOpcode::kArmI8x16Neg
            | ArchOpcode::kArmI8x16Shl
            | ArchOpcode::kArmI8x16ShrS
            | ArchOpcode::kArmI8x16SConvertI16x8
            | ArchOpcode::kArmI8x16Add
            | ArchOpcode::kArmI8x16AddSatS
            | ArchOpcode::kArmI8x16Sub
            | ArchOpcode::kArmI8x16SubSatS
            | ArchOpcode::kArmI8x16MinS
            | ArchOpcode::kArmI8x16MaxS
            | ArchOpcode::kArmI8x16Eq
            | ArchOpcode::kArmI8x16Ne
            | ArchOpcode::kArmI8x16GtS
            | ArchOpcode::kArmI8x16GeS
            | ArchOpcode::kArmI8x16ExtractLaneU
            | ArchOpcode::kArmI8x16UConvertI16x8
            | ArchOpcode::kArmI8x16AddSatU
            | ArchOpcode::kArmI8x16SubSatU
            | ArchOpcode::kArmI8x16ShrU
            | ArchOpcode::kArmI8x16MinU
            | ArchOpcode::kArmI8x16MaxU
            | ArchOpcode::kArmI8x16GtU
            | ArchOpcode::kArmI8x16GeU
            | ArchOpcode::kArmI8x16RoundingAverageU
            | ArchOpcode::kArmI8x16Abs
            | ArchOpcode::kArmI8x16BitMask
            | ArchOpcode::kArmS128Const
            | ArchOpcode::kArmS128Zero
            | ArchOpcode::kArmS128AllOnes
            | ArchOpcode::kArmS128Dup
            | ArchOpcode::kArmS128And
            | ArchOpcode::kArmS128Or
            | ArchOpcode::kArmS128Xor
            | ArchOpcode::kArmS128Not
            | ArchOpcode::kArmS128Select
            | ArchOpcode::kArmS128AndNot
            | ArchOpcode::kArmS32x4ZipLeft
            | ArchOpcode::kArmS32x4ZipRight
            | ArchOpcode::kArmS32x4UnzipLeft
            | ArchOpcode::kArmS32x4UnzipRight
            | ArchOpcode::kArmS32x4TransposeLeft
            | ArchOpcode::kArmS32x4TransposeRight
            | ArchOpcode::kArmS32x4Shuffle
            | ArchOpcode::kArmS16x8ZipLeft
            | ArchOpcode::kArmS16x8ZipRight
            | ArchOpcode::kArmS16x8UnzipLeft
            | ArchOpcode::kArmS16x8UnzipRight
            | ArchOpcode::kArmS16x8TransposeLeft
            | ArchOpcode::kArmS16x8TransposeRight
            | ArchOpcode::kArmS8x16ZipLeft
            | ArchOpcode::kArmS8x16ZipRight
            | ArchOpcode::kArmS8x16UnzipLeft
            | ArchOpcode::kArmS8x16UnzipRight
            | ArchOpcode::kArmS8x16TransposeLeft
            | ArchOpcode::kArmS8x16TransposeRight
            | ArchOpcode::kArmS8x16Concat
            | ArchOpcode::kArmI8x16Swizzle
            | ArchOpcode::kArmI8x16Shuffle
            | ArchOpcode::kArmS32x2Reverse
            | ArchOpcode::kArmS16x4Reverse
            | ArchOpcode::kArmS16x2Reverse
            | ArchOpcode::kArmS8x8Reverse
            | ArchOpcode::kArmS8x4Reverse
            | ArchOpcode::kArmS8x2Reverse
            | ArchOpcode::kArmI64x2AllTrue
            | ArchOpcode::kArmI32x4AllTrue
            | ArchOpcode::kArmI16x8AllTrue
            | ArchOpcode::kArmV128AnyTrue
            | ArchOpcode::kArmI8x16AllTrue => Ok(kNoOpcodeFlags),

            ArchOpcode::kArmVldrF32
            | ArchOpcode::kArmVldrF64
            | ArchOpcode::kArmVld1F64
            | ArchOpcode::kArmVld1S128
            | ArchOpcode::kArmLdrb
            | ArchOpcode::kArmLdrsb
            | ArchOpcode::kArmLdrh
            | ArchOpcode::kArmLdrsh
            | ArchOpcode::kArmLdr
            | ArchOpcode::kArmPeek
            | ArchOpcode::kArmWord32AtomicPairLoad
            | ArchOpcode::kArmS128Load8Splat
            | ArchOpcode::kArmS128Load16Splat
            | ArchOpcode::kArmS128Load32Splat
            | ArchOpcode::kArmS128Load64Splat
            | ArchOpcode::kArmS128Load8x8S
            | ArchOpcode::kArmS128Load8x8U
            | ArchOpcode::kArmS128Load16x4S
            | ArchOpcode::kArmS128Load16x4U
            | ArchOpcode::kArmS128Load32x2S
            | ArchOpcode::kArmS128Load32x2U
            | ArchOpcode::kArmS128Load32Zero
            | ArchOpcode::kArmS128Load64Zero
            | ArchOpcode::kArmS128LoadLaneLow
            | ArchOpcode::kArmS128LoadLaneHigh => Ok(kIsLoadOperation),

            ArchOpcode::kArmVstrF32
            | ArchOpcode::kArmVstrF64
            | ArchOpcode::kArmVst1F64
            | ArchOpcode::kArmVst1S128
            | ArchOpcode::kArmStrb
            | ArchOpcode::kArmStrh
            | ArchOpcode::kArmStr
            | ArchOpcode::kArmPush
            | ArchOpcode::kArmPoke
            | ArchOpcode::kArmDmbIsh
            | ArchOpcode::kArmDsbIsb
            | ArchOpcode::kArmWord32AtomicPairStore
            | ArchOpcode::kArmWord32AtomicPairAdd
            | ArchOpcode::kArmWord32AtomicPairSub
            | ArchOpcode::kArmWord32AtomicPairAnd
            | ArchOpcode::kArmWord32AtomicPairOr
            | ArchOpcode::kArmWord32AtomicPairXor
            | ArchOpcode::kArmWord32AtomicPairExchange
            | ArchOpcode::kArmWord32AtomicPairCompareExchange
            | ArchOpcode::kArmS128StoreLaneLow
            | ArchOpcode::kArmS128StoreLaneHigh => Ok(kHasSideEffect),

            ArchOpcode::kInvalid => Err(InstructionSchedulerError::UnsupportedInstruction),
        }
    }

    pub fn GetInstructionLatency(&self, instr: &Instruction) -> i32 {
        1
    }
}

const kNoOpcodeFlags: i32 = 0;
const kIsLoadOperation: i32 = 1;
const kHasSideEffect: i32 = 2;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ArchOpcode {
    kInvalid,
    kArmAdd,
    kArmAnd,
    kArmBic,
    kArmClz,
    kArmCmp,
    kArmCmn,
    kArmTst,
    kArmTeq,
    kArmOrr,
    kArmEor,
    kArmSub,
    kArmRsb,
    kArmMul,
    kArmMla,
    kArmMls,
    kArmSmmul,
    kArmSmull,
    kArmSmmla,
    kArmUmull,
    kArmSdiv,
    kArmUdiv,
    kArmMov,
    kArmMvn,
    kArmBfc,
    kArmUbfx,
    kArmSbfx,
    kArmSxtb,
    kArmSxth,
    kArmSxtab,
    kArmSxtah,
    kArmUxtb,
    kArmUxth,
    kArmUxtab,
    kArmUxtah,
    kArmRbit,
    kArmRev,
    kArmAddPair,
    kArmSubPair,
    kArmMulPair,
    kArmLslPair,
    kArmLsrPair,
    kArmAsrPair,
    kArmVcmpF32,
    kArmVaddF32,
    kArmVsubF32,
    kArmVmulF32,
    kArmVmlaF32,
    kArmVmlsF32,
    kArmVdivF32,
    kArmVabsF32,
    kArmVnegF32,
    kArmVsqrtF32,
    kArmVldrF32,
    kArmVstrF32,
    kArmVcmpF64,
    kArmVaddF64,
    kArmVsubF64,
    kArmVmulF64,
    kArmVmlaF64,
    kArmVmlsF64,
    kArmVdivF64,
    kArmVmodF64,
    kArmVabsF64,
    kArmVnegF64,
    kArmVsqrtF64,
    kArmVldrF64,
    kArmVstrF64,
    kArmVmullLow,
    kArmVmullHigh,
    kArmVrintmF32,
    kArmVrintmF64,
    kArmVrintpF32,
    kArmVrintpF64,
    kArmVrintzF32,
    kArmVrintzF64,
    kArmVrintaF64,
    kArmVrintnF32,
    kArmVrintnF64,
    kArmVcvtF32F64,
    kArmVcvtF64F32,
    kArmVcvtF32S32,
    kArmVcvtF32U32,
    kArmVcvtF64S32,
    kArmVcvtF64U32,
    kArmVcvtS32F32,
    kArmVcvtU32F32,
    kArmVcvtS32F64,
    kArmVcvtU32F64,
    kArmVmovU32F32,
    kArmVmovF32U32,
    kArmVmovLowU32F64,
    kArmVmovLowF64U32,
    kArmVmovHighU32F64,
    kArmVmovHighF64U32,
    kArmVmovF64U32U32,
    kArmVmovU32U32F64,
    kArmVcnt,
    kArmVpadal,
    kArmVpaddl,
    kArmFloat32Max,
    kArmFloat64Max,
    kArmFloat32Min,
    kArmFloat64Min,
    kArmFloat64SilenceNaN,
    kArmF64x2Splat,
    kArmF64x2ExtractLane,
    kArmF64x2ReplaceLane,
    kArmF64x2Abs,
    kArmF64x2Neg,
    kArmF64x2Sqrt,
    kArmF64x2Add,
    kArmF64x2Sub,
    kArmF64x2Mul,
    kArmF64x2Div,
    kArmF64x2Min,
    kArmF64x2Max,
    kArmF64x2Eq,
    kArmF64x2Ne,
    kArmF64x2Lt,
    kArmF64x2Le,
    kArmF64x2Qfma,
    kArmF64x2Qfms,
    kArmF64x2Pmin,
    kArmF64x2Pmax,
    kArmF64x2Ceil,
    kArmF64x2Floor,
    kArmF64x2Trunc,
    kArmF64x2NearestInt,
    kArmF64x2ConvertLowI32x4S,
    kArmF64x2ConvertLowI32x4U,
    kArmF64x2PromoteLowF32x4,
    kArmF32x4Splat,
    kArmF32x4ExtractLane,
    kArmF32x4ReplaceLane,
    kArmF32x4SConvertI32x4,
    kArmF32x4UConvertI32x4,
    kArmF32x4Abs,
    kArmF32x4Neg,
    kArmF32x4Sqrt,
    kArmF32x4Add,
    kArmF32x4Sub,
    kArmF32x4Mul,
    kArmF32x4Div,
    kArmF32x4Min,
    kArmF32x4Max,
    kArmF32x4Eq,
    kArmF32x4Ne,
    kArmF32x4Lt,
    kArmF32x4Le,
    kArmF32x4Qfma,
    kArmF32x4Qfms,
    kArmF32x4Pmin,
    kArmF32x4Pmax,
    kArmF32x4DemoteF64x2Zero,
    kArmI64x2SplatI32Pair,
    kArmI64x2ReplaceLaneI32Pair,
    kArmI64x2Abs,
    kArmI64x2Neg,
    kArmI64x2Shl,
    kArmI64x2ShrS,
    kArmI64x2Add,
    kArmI64x2Sub,
    kArmI64x2Mul,
    kArmI64x2ShrU,
    kArmI64x2BitMask,
    kArmI64x2Eq,
    kArmI64x2Ne,
    kArmI64x2GtS,
    kArmI64x2GeS,
    kArmI64x2SConvertI32x4Low,
    kArmI64x2SConvertI32x4High,
    kArmI64x2UConvertI32x4Low,
    kArmI64x2UConvertI32x4High,
    kArmI32x4Splat,
    kArmI32x4ExtractLane,
    kArmI32x4ReplaceLane,
    kArmI32x4SConvertF32x4,
    kArmI32x4SConvertI16x8Low,
    kArmI32x4SConvertI16x8High,
    kArmI32x4Neg,
    kArmI32x4Shl,
    kArmI32x4ShrS,
    kArmI32x4Add,
    kArmI32x4Sub,
    kArmI32x4Mul,
    kArmI32x4MinS,
    kArmI32x4MaxS,
    kArmI32x4Eq,
    kArmI32x4Ne,
    kArmI32x4GtS,
    kArmI32x4GeS,
    kArmI32x4UConvertF32x4,
    kArmI32x4UConvertI16x8Low,
    kArmI32x4UConvertI16x8High,
    kArmI32x4ShrU,
    kArmI32x4MinU,
    kArmI32x4MaxU,
    kArmI32x4GtU,
    kArmI32x4GeU,
    kArmI32x4Abs,
    kArmI32x4BitMask,
    kArmI32x4DotI16x8S,
    kArmI16x8DotI8x16S,
    kArmI32x4DotI8x16AddS,
    kArmI32x4TruncSatF64x2SZero,
    kArmI32x4TruncSatF64x2UZero,
    kArmI16x8Splat,
    kArmI16x8ExtractLaneS,
    kArmI16x8ReplaceLane,
    kArmI16x8SConvertI8x16Low,
    kArmI16x8SConvertI8x16High,
    kArmI16x8Neg,
    kArmI16x8Shl,
    kArmI16x8ShrS,
    kArmI16x8SConvertI32x4,
    kArmI16x8Add,
    kArmI16x8AddSatS,
    kArmI16x

// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-scheduler-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp;

use crate::compiler::backend::mips64::code_generator_mips64::Use;
use crate::compiler::instruction_scheduler::COMMON_ARCH_OPCODE_LIST;
use crate::compiler::opcodes::Opcode::kAtomicLoadInt8;
use crate::execution::isolate::this;
use crate::execution::isolate::Isolate;
use crate::instruction::{Instruction, InstructionOperand, OpIndex, RootIndex};
use crate::wasm_gc_operator_reducer::v8;

pub struct InstructionScheduler {}

impl InstructionScheduler {
    pub fn new() -> Self {
        InstructionScheduler {}
    }
    pub fn scheduler_supported() -> bool {
        true
    }

    pub fn get_target_instruction_flags(&self, instr: &Instruction) -> i32 {
        match instr.arch_opcode() {
            kRiscvEnableDebugTrace
            | kRiscvDisableDebugTrace => 0,
            #[cfg(target_arch = "riscv64")]
            kRiscvAdd32
            | kRiscvBitcastDL
            | kRiscvBitcastLD
            | kRiscvByteSwap64
            | kRiscvCvtDL
            | kRiscvCvtDUl
            | kRiscvCvtSL
            | kRiscvCvtSUl
            | kRiscvMulHigh64
            | kRiscvMulHighU64
            | kRiscvAdd64
            | kRiscvAddOvf64
            | kRiscvClz64
            | kRiscvCtz64
            | kRiscvDiv64
            | kRiscvDivU64
            | kRiscvZeroExtendWord
            | kRiscvSignExtendWord
            | kRiscvMod64
            | kRiscvModU64
            | kRiscvMul64
            | kRiscvMulOvf64
            | kRiscvPopcnt64
            | kRiscvRor64
            | kRiscvSar64
            | kRiscvShl64
            | kRiscvShr64
            | kRiscvSub64
            | kRiscvSubOvf64
            | kRiscvFloat64RoundDown
            | kRiscvFloat64RoundTiesEven
            | kRiscvFloat64RoundTruncate
            | kRiscvFloat64RoundUp
            | kRiscvSub32
            | kRiscvTruncLD
            | kRiscvTruncLS
            | kRiscvTruncUlD
            | kRiscvTruncUlS
            | kRiscvCmp32
            | kRiscvCmpZero32 => 0,
            #[cfg(target_arch = "riscv32")]
            kRiscvAdd32
            | kRiscvAddPair
            | kRiscvSubPair
            | kRiscvMulPair
            | kRiscvAndPair
            | kRiscvOrPair
            | kRiscvXorPair
            | kRiscvShlPair
            | kRiscvShrPair
            | kRiscvSarPair
            | kRiscvAddOvf
            | kRiscvSubOvf
            | kRiscvSub32 => 0,
            kRiscvSh1add
            | kRiscvSh2add
            | kRiscvSh3add => 0,
            #[cfg(target_arch = "riscv64")]
            kRiscvAdduw
            | kRiscvSh1adduw
            | kRiscvSh2adduw
            | kRiscvSh3adduw
            | kRiscvSlliuw => 0,
            kRiscvAndn
            | kRiscvOrn
            | kRiscvXnor
            | kRiscvClz
            | kRiscvCtz
            | kRiscvCpop => 0,
            #[cfg(target_arch = "riscv64")]
            kRiscvClzw
            | kRiscvCtzw
            | kRiscvCpopw => 0,
            kRiscvMax
            | kRiscvMaxu
            | kRiscvMin
            | kRiscvMinu
            | kRiscvSextb
            | kRiscvSexth
            | kRiscvZexth
            | kRiscvRev8
            | kRiscvBclr
            | kRiscvBclri
            | kRiscvBext
            | kRiscvBexti
            | kRiscvBinv
            | kRiscvBinvi
            | kRiscvBset
            | kRiscvBseti
            | kRiscvAbsD
            | kRiscvAbsS
            | kRiscvAddD
            | kRiscvAddS
            | kRiscvAnd
            | kRiscvAnd32
            | kRiscvAssertEqual
            | kRiscvBitcastInt32ToFloat32
            | kRiscvBitcastFloat32ToInt32
            | kRiscvByteSwap32
            | kRiscvCeilWD
            | kRiscvCeilWS
            | kRiscvClz32
            | kRiscvCmp
            | kRiscvCmpZero
            | kRiscvCmpD
            | kRiscvCmpS
            | kRiscvCtz32
            | kRiscvCvtDS
            | kRiscvCvtDUw
            | kRiscvCvtDW
            | kRiscvCvtSD
            | kRiscvCvtSUw
            | kRiscvMulHighU32
            | kRiscvDiv32
            | kRiscvDivD
            | kRiscvDivS
            | kRiscvDivU32
            | kRiscvF64x2Abs
            | kRiscvF64x2Sqrt
            | kRiscvF64x2Pmin
            | kRiscvF64x2Pmax
            | kRiscvF64x2ConvertLowI32x4S
            | kRiscvF64x2ConvertLowI32x4U
            | kRiscvF64x2PromoteLowF32x4
            | kRiscvF64x2Ceil
            | kRiscvF64x2Floor
            | kRiscvF64x2Trunc
            | kRiscvF64x2NearestInt
            | kRiscvI64x2SplatI32Pair
            | kRiscvI64x2ExtractLane
            | kRiscvI64x2ReplaceLane
            | kRiscvI64x2ReplaceLaneI32Pair
            | kRiscvI64x2Shl
            | kRiscvI64x2ShrS
            | kRiscvI64x2ShrU
            | kRiscvF32x4Abs
            | kRiscvF32x4ExtractLane
            | kRiscvF32x4Sqrt
            | kRiscvF64x2Qfma
            | kRiscvF64x2Qfms
            | kRiscvF32x4Qfma
            | kRiscvF32x4Qfms
            | kRiscvF32x4ReplaceLane
            | kRiscvF32x4SConvertI32x4
            | kRiscvF32x4UConvertI32x4
            | kRiscvF32x4Pmin
            | kRiscvF32x4Pmax
            | kRiscvF32x4DemoteF64x2Zero
            | kRiscvF32x4Ceil
            | kRiscvF32x4Floor
            | kRiscvF32x4Trunc
            | kRiscvF32x4NearestInt
            | kRiscvF64x2ExtractLane
            | kRiscvF64x2ReplaceLane
            | kRiscvFloat32Max
            | kRiscvFloat32Min
            | kRiscvFloat32RoundDown
            | kRiscvFloat32RoundTiesEven
            | kRiscvFloat32RoundTruncate
            | kRiscvFloat32RoundUp
            | kRiscvFloat64ExtractLowWord32
            | kRiscvFloat64ExtractHighWord32
            | kRiscvFloat64InsertLowWord32
            | kRiscvFloat64InsertHighWord32
            | kRiscvFloat64Max
            | kRiscvFloat64Min
            | kRiscvFloat64SilenceNaN
            | kRiscvFloorWD
            | kRiscvFloorWS
            | kRiscvI64x2SConvertI32x4Low
            | kRiscvI64x2SConvertI32x4High
            | kRiscvI64x2UConvertI32x4Low
            | kRiscvI64x2UConvertI32x4High
            | kRiscvI16x8ExtractLaneU
            | kRiscvI16x8ExtractLaneS
            | kRiscvI16x8ReplaceLane
            | kRiscvI16x8Shl
            | kRiscvI16x8ShrS
            | kRiscvI16x8ShrU
            | kRiscvI32x4TruncSatF64x2SZero
            | kRiscvI32x4TruncSatF64x2UZero
            | kRiscvI32x4ExtractLane
            | kRiscvI32x4ReplaceLane
            | kRiscvI32x4SConvertF32x4
            | kRiscvI32x4Shl
            | kRiscvI32x4ShrS
            | kRiscvI32x4ShrU
            | kRiscvI32x4UConvertF32x4
            | kRiscvI8x16ExtractLaneU
            | kRiscvI8x16ExtractLaneS
            | kRiscvI8x16ReplaceLane
            | kRiscvI8x16Shl
            | kRiscvI8x16ShrS
            | kRiscvI8x16ShrU
            | kRiscvI8x16RoundingAverageU
            | kRiscvI8x16Popcnt
            | kRiscvMaxD
            | kRiscvMaxS
            | kRiscvMinD
            | kRiscvMinS
            | kRiscvMod32
            | kRiscvModU32
            | kRiscvMov
            | kRiscvMul32
            | kRiscvMulD
            | kRiscvMulHigh32
            | kRiscvMulOvf32
            | kRiscvMulS
            | kRiscvNegD
            | kRiscvNegS
            | kRiscvOr
            | kRiscvOr32
            | kRiscvPopcnt32
            | kRiscvRor32
            | kRiscvRoundWD
            | kRiscvRoundWS
            | kRiscvVnot
            | kRiscvS128Select
            | kRiscvS128Const
            | kRiscvS128Zero
            | kRiscvS128Load32Zero
            | kRiscvS128Load64Zero
            | kRiscvS128AllOnes
            | kRiscvV128AnyTrue
            | kRiscvI8x16Shuffle
            | kRiscvVwmul
            | kRiscvVwmulu
            | kRiscvVmv
            | kRiscvVandVv
            | kRiscvVorVv
            | kRiscvVnotVv
            | kRiscvVxorVv
            | kRiscvVmvSx
            | kRiscvVmvXs
            | kRiscvVfmvVf
            | kRiscvVcompress
            | kRiscvVaddVv
            | kRiscvVwaddVv
            | kRiscvVwadduVv
            | kRiscvVwadduWx
            | kRiscvVsubVv
            | kRiscvVnegVv
            | kRiscvVfnegVv
            | kRiscvVmaxuVv
            | kRiscvVmax
            | kRiscvVminsVv
            | kRiscvVminuVv
            | kRiscvVmulVv
            | kRiscvVdivu
            | kRiscvVsmulVv
            | kRiscvVmslt
            | kRiscvVgtsVv
            | kRiscvVgesVv
            | kRiscvVgeuVv
            | kRiscvVgtuVv
            | kRiscvVeqVv
            | kRiscvVneVv
            | kRiscvVAbs
            | kRiscvVaddSatUVv
            | kRiscvVaddSatSVv
            | kRiscvVsubSatUVv
            | kRiscvVsubSatSVv
            | kRiscvVrgather
            | kRiscvVslidedown
            | kRiscvVredminuVs
            | kRiscvVAllTrue
            | kRiscvVnclipu
            | kRiscvVnclip
            | kRiscvVsll
            | kRiscvVfaddVv
            | kRiscvVfsubVv
            | kRiscvVfmulVv
            | kRiscvVfdivVv
            | kRiscvVfminVv
            | kRiscvVfmaxVv
            | kRiscvVmfeqVv
            | kRiscvVmfneVv
            | kRiscvVmfltVv
            | kRiscvVmfleVv
            | kRiscvVmergeVx
            | kRiscvVzextVf2
            | kRiscvVsextVf2
            | kRiscvSar32
            | kRiscvSignExtendByte
            | kRiscvSignExtendShort
            | kRiscvShl32
            | kRiscvShr32
            | kRiscvSqrtD
            | kRiscvSqrtS
            | kRiscvSubD
            | kRiscvSubS
            | kRiscvTruncUwD
            | kRiscvTruncUwS
            | kRiscvTruncWD
            | kRiscvTruncWS
            | kRiscvTst32
            | kRiscvXor
            | kRiscvXor32 => 0,
            #[cfg(target_arch = "riscv64")]
            kRiscvTst64
            | kRiscvLd
            | kRiscvLwu
            | kRiscvUlwu
            | kRiscvWord64AtomicLoadUint64
            | kRiscvLoadDecompressTaggedSigned
            | kRiscvLoadDecompressTagged
            | kRiscvLoadDecodeSandboxedPointer
            | kRiscvAtomicLoadDecompressTaggedSigned
            | kRiscvAtomicLoadDecompressTagged
            | kRiscvAtomicStoreCompressTagged
            | kRiscvLoadDecompressProtected => 1,
            #[cfg(target_arch = "riscv32")]
            kRiscvWord32AtomicPairLoad => 1,
            kRiscvLb
            | kRiscvLbu
            | kRiscvLoadDouble
            | kRiscvLh
            | kRiscvLhu
            | kRiscvLw
            | kRiscvLoadFloat
            | kRiscvRvvLd
            | kRiscvPeek
            | kRiscvUld
            | kRiscvULoadDouble
            | kRiscvUlh
            | kRiscvUlhu
            | kRiscvUlw
            | kRiscvULoadFloat
            | kRiscvS128LoadSplat
            | kRiscvS128Load64ExtendU
            | kRiscvS128Load64ExtendS
            | kRiscvS128LoadLane => 1,
            #[cfg(target_arch = "riscv64")]
            kRiscvSd
            | kRiscvUsd
            | kRiscvWord64AtomicStoreWord64
            | kRiscvWord64AtomicAddUint64
            | kRiscvWord64AtomicSubUint64
            | kRiscvWord64AtomicAndUint64
            | kRiscvWord64AtomicOrUint64
            | kRiscvWord64AtomicXorUint64
            | kRiscvWord64AtomicExchangeUint64
            | kRiscvWord64AtomicCompareExchangeUint64
            | kRiscvStoreCompressTagged
            | kRiscvStoreEncodeSandboxedPointer
            | kRiscvStoreIndirectPointer => 2,
            #[cfg(target_arch = "riscv32")]
            kRiscvWord32AtomicPairStore
            | kRiscvWord32AtomicPairAdd
            | kRiscvWord32AtomicPairSub
            | kRiscvWord32AtomicPairAnd
            | kRiscvWord32AtomicPairOr
            | kRiscvWord32AtomicPairXor
            | kRiscvWord32AtomicPairExchange
            | kRiscvWord32AtomicPairCompareExchange => 2,
            kRiscvModD
            | kRiscvModS
            | kRiscvRvvSt
            | kRiscvPush
            | kRiscvSb
            | kRiscvStoreDouble
            | kRiscvSh
            | kRiscvStackClaim
            | kRiscvStoreToStackSlot
            | kRiscvSw
            | kRiscvStoreFloat
            | kRiscvUStoreDouble
            | kRiscvUsh
            | kRiscvUsw
            | kRiscvUStoreFloat
            | kRiscvSync
            | kRiscvS128StoreLane => 2,
            _ => {
                if let Some(opcode) = Self::common_arch_opcode_to_enum(instr.arch_opcode()) {
                    match opcode {
                        CommonArchOpcode::kArchCallCodeObject
                        | CommonArchOpcode::kArchCallWasmFunction
                        | CommonArchOpcode::kArchTailCallCodeObject
                        | CommonArchOpcode::kArchTailCallWasm
                        | CommonArchOpcode::kArchTailCallAddress
                        | CommonArchOpcode::kArchCallJSFunction
                        | CommonArchOpcode::kArchPrepareCallCFunction
                        | CommonArchOpcode::kArchSaveCallerRegisters
                        | CommonArchOpcode::kArchRestoreCallerRegisters
                        | CommonArchOpcode::kArchPrepareTailCall
                        | CommonArchOpcode::kArchCallCFunction
                        | CommonArchOpcode::kArchJmp
                        | CommonArchOpcode::kArchTableSwitch
                        | CommonArchOpcode::kArchAbortCSADcheck
                        | CommonArchOpcode::kArchDebugBreak
                        | CommonArchOpcode::kArchComment
                        | CommonArchOpcode::kArchNop
                        | CommonArchOpcode::kArchThrowTerminator
                        | CommonArchOpcode::kArchDeoptimize
                        | CommonArchOpcode::kArchRet
                        | CommonArchOpcode::kArchFramePointer
                        | CommonArchOpcode::kArchParentFramePointer
                        | CommonArchOpcode::kArchTruncateDoubleToI
                        | CommonArchOpcode::kArchStoreWithWriteBarrier
                        | CommonArchOpcode::kArchStackSlot => 0,
                        CommonArchOpcode::kIeee754Float64Acos
                        | CommonArchOpcode::kIeee754Float64Acosh
                        | CommonArchOpcode::kIeee754Float64Asin
                        | CommonArchOpcode::kIeee754Float64Asinh
                        | CommonArchOpcode::kIeee754Float64Atan
                        | CommonArchOpcode::kIeee754Float64Atanh
                        | CommonArchOpcode::kIeee754Float64Atan2
                        | CommonArchOpcode::kIeee754Float64Cos
                        | CommonArchOpcode::kIeee754Float64Cosh
                        | CommonArchOpcode::kIeee754Float64Cbrt
                        | CommonArchOpcode::kIeee754Float64Exp
                        | CommonArchOpcode::kIeee754Float64Expm1
                        | CommonArchOpcode::kIeee754Float64Log
                        | CommonArchOpcode::kIeee754Float64Log1p
                        | CommonArchOpcode::kIeee754Float64Log10
                        | CommonArchOpcode::kIeee754Float64Log2
                        | CommonArchOpcode::kIeee754Float64Pow
                        | CommonArchOpcode::kIeee754Float64Sin
                        | CommonArchOpcode::kIeee754Float64Sinh
                        | CommonArchOpcode::kIeee754Float64Tan
                        | CommonArchOpcode::kIeee754Float64Tanh => 0,
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn common_arch_opcode_to_enum(arch_opcode: i32) -> Option<CommonArchOpcode> {
        match arch_opcode {
            0 => Some(CommonArchOpcode::kArchCallCodeObject),
            1 => Some(CommonArchOpcode::kArchCallWasmFunction),
            2 => Some(CommonArchOpcode::kArchTailCallCodeObject),
            3 => Some(CommonArchOpcode::kArchTailCallWasm),
            4 => Some(CommonArchOpcode::kArchTailCallAddress),
            5 => Some(CommonArchOpcode::kArchCallJSFunction),
            6 => Some(CommonArchOpcode::kArchPrepareCallCFunction),
            7 => Some(CommonArchOpcode::kArchSaveCallerRegisters),
            8 => Some(CommonArchOpcode::kArchRestoreCallerRegisters),
            9 => Some(CommonArchOpcode::kArchPrepareTailCall),
            10 => Some(CommonArchOpcode::kArchCallCFunction),
            11 => Some(CommonArchOpcode::kArchJmp),
            12 => Some(CommonArchOpcode::kArchTableSwitch),
            13 => Some(CommonArchOpcode::kArchAbortCSADcheck),
            14 => Some(CommonArchOpcode::kArchDebugBreak),
            15 => Some(CommonArchOpcode::kArchComment),
            16 => Some(CommonArchOpcode::kArchNop),
            17 => Some(CommonArchOpcode::kArchThrowTerminator),
            18 => Some(CommonArchOpcode::kArchDeoptimize),
            19 => Some(CommonArchOpcode::kArchRet),
            20 => Some(CommonArchOpcode::kArchFramePointer),
            21 => Some(CommonArchOpcode::kArchParentFramePointer),
            22 => Some(CommonArchOpcode::kArchTruncateDoubleToI),
            23 => Some(CommonArchOpcode::kArchStoreWithWriteBarrier),
            24 => Some(CommonArchOpcode::kArchStackSlot),
            25 => Some(CommonArchOpcode::kIeee754Float64Acos),
            26 => Some(CommonArchOpcode::kIeee754Float64Acosh),
            27 => Some(CommonArchOpcode::kIeee754Float64Asin),
            28 => Some(CommonArchOpcode::kIeee754Float64Asinh),
            29 => Some(CommonArchOpcode::kIeee754Float64Atan),
            30 => Some(CommonArchOpcode::kIeee754Float64Atanh),
            31 => Some(CommonArchOpcode::kIeee754Float64Atan2),
            32 => Some(CommonArchOpcode::kIeee754Float64Cos),
            33 => Some(CommonArchOpcode::kIeee754Float64Cosh),
            34 => Some(CommonArchOpcode::kIeee754Float64Cbrt),
            35 => Some(CommonArchOpcode::kIeee754Float64Exp),
            36 => Some(CommonArchOpcode::kIeee754Float64Expm1),
            37 => Some(CommonArchOpcode::kIeee754Float64Log),
            38 => Some(CommonArchOpcode::kIeee754Float64Log1p),
            39 => Some(CommonArchOpcode::kIeee754Float64Log10),
            40 => Some(CommonArchOpcode::kIeee754Float64Log2),
            41 => Some(CommonArchOpcode::kIeee754Float64Pow),
            42 => Some(CommonArchOpcode::kIeee754Float64Sin),
            43 => Some(CommonArchOpcode::kIeee754Float64Sinh),
            44 => Some(CommonArchOpcode::kIeee754Float64Tan),
            45 => Some(CommonArchOpcode::kIeee754Float64Tanh),
            _ => None,
        }
    }

    pub fn get_instruction_latency(&self, instr: &Instruction) -> i32 {
        match instr.arch_opcode() {
            kArchCallCodeObject | kArchCallWasmFunction => Self::call_latency(),
            kArchTailCallCodeObject | kArchTailCallWasm | kArchTailCallAddress => {
                Self::jump_latency()
            }
            kArchCallJSFunction => {
                let mut latency = 0;
                if false {
                    latency = 1 + Self::assert_latency();
                }
                latency + 1 + Self::add64_latency(false) + Self::call_latency()
            }
            kArchPrepareCallCFunction => Self::prepare_call_c_function_latency(),
            kArchSaveCallerRegisters => {
                let fp_mode = SaveFPRegsMode::KSave;
                Self::push_caller_saved_latency(fp_mode)
            }
            kArchRestoreCallerRegisters => {
                let fp_mode = SaveFPRegsMode::KSave;
                Self::pop_caller_saved_latency(fp_mode)
            }
            kArchPrepareTailCall => 2,
            kArchCallCFunction => Self::call_c_function_latency(),
            kArchJmp => Self::assemble_arch_jump_latency(),
            kArchTableSwitch => Self::assemble_arch_table_switch_latency(),
            kArchAbortCSADcheck => Self::call_latency() + 1,
            kArchDebugBreak => 1,
            kArchComment | kArchNop | kArchThrowTerminator | kArchDeoptimize => 0,
            kArchRet => Self::assembler_return_latency(),
            kArchFramePointer => 1,
            kArchParentFramePointer => Self::aligned_memory_latency(),
            kArchTruncateDoubleToI => Self::truncate_double_to_i_delayed_latency(),
            kArchStoreWithWriteBarrier => {
                Self::add64_latency(true) + 1 + Self::check_page_flag_latency()
            }
            kArchStackSlot => {
                Self::add64_latency(false)
                    + Self::and_latency(false)
                    + Self::assert_latency()
                    + Self::add64_latency(false)
                    + Self::and_latency(false)
                    + Self::branch_short_latency()
                    + 1
                    + Self::sub64_latency(true)
                    + Self::add64_latency(true)
            }
            kIeee754Float64Acos
            | kIeee754Float64Acosh
            | kIeee754Float64Asin
            | kIeee754Float64Asinh
            | kIeee754Float64Atan
            | kIeee754Float64Atanh
            | kIeee754Float64Atan2
            | kIeee754Float64Cos
            | kIeee754Float64Cosh
            | kIeee754Float64Cbrt
            | kIeee754Float64Exp
            | kIeee754Float64Expm1
            | kIeee754Float64Log
            | kIeee754Float64Log1p
            | kIeee754Float64Log10
            | kIeee754Float64Log2
            | kIeee754Float64Pow
            | kIeee754Float64Sin
            | kIeee754Float64Sinh
            | kIeee754Float64Tan
            | kIeee754Float64Tanh => {
                Self::prepare_call_c_function_latency()
                    + Self::mov_to_float_parameters_latency()
                    + Self::call_c_function_latency()
                    + Self::mov_from_float_result_latency()
            }
            #[cfg(target_arch = "riscv64")]
            kRiscvAdd32 | kRiscvAdd64 => Self::add64_latency(instr.input_at(1).is_register()),
            #[cfg(target_arch = "riscv64")]
            kRiscvAddOvf64 => Self::add_overflow64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvSub32 | kRiscvSub64 => Self::sub64_latency(instr.input_at(1).is_register()),
            #[cfg(target_arch = "riscv64")]
            kRiscvSubOvf64 => Self::sub_overflow64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvMulHigh64 => Self::mulh64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvMul64 => Self::mul64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvMulOvf64 => Self::mul_overflow64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvDiv64 => {
                let latency = Self::div64_latency();
                latency + Self::movz_latency()
            }
            #[cfg(target_arch = "riscv64")]
            kRiscvDivU64 => {
                let latency = Self::divu64_latency();
                latency + Self::movz_latency()
            }
            #[cfg(target_arch = "riscv64")]
            kRiscvMod64 => Self::mod64_latency(),
            #[cfg(target_arch = "riscv64")]
            kRiscvModU64 => Self::modu64_latency(),
            #[cfg(target_arch = "riscv32")]
            kRiscvAdd32 => Self::add64_latency(instr.input_at(1).is_register()),
            #[cfg(target_arch = "riscv32")]
            kRiscvAddOvf => Self::add_overflow64_latency(),
            #[cfg(target_arch = "riscv32")]
            kRiscvSub32 => Self::sub64_latency(instr.input_at(1).is_register()),
            #[cfg(target_arch = "riscv32")]
            kRiscvSubOvf => Self::sub_overflow64_latency(),
            kRiscvMul32 => Self::mul32_latency(),
            kRiscvMulOvf32 => Self::mul_overflow32_latency(),
            kRiscvMulHigh32 => Self::mulh32_latency(),
            kRiscvMulHighU32 => Self::mulhu32_latency(),
            kRiscvDiv32 => {
                let latency = Self::div32_latency(instr.input_at(1).is_register());
                latency + Self::movz_latency()
            }
            kRiscvDivU32 => {
                let latency = Self::divu32_latency(instr.input_at(1).is_register());
                latency + Self::movz_latency()
            }
            kRiscvMod32 => Self::mod32_latency(),
            kRiscvModU32 => Self::modu32_latency(),
            kRiscvAnd => Self::and_latency(instr.input_at(

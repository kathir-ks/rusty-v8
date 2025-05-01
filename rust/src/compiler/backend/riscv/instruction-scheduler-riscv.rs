// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::codegen::macro_assembler; // Assuming a corresponding Rust module exists
//use crate::compiler::backend::instruction_scheduler; // Assuming a corresponding Rust module exists

pub mod instruction_scheduler {
    use std::convert::TryFrom;

    pub struct InstructionScheduler {}

    impl InstructionScheduler {
        pub fn scheduler_supported() -> bool {
            true
        }

        pub fn new() -> Self {
            InstructionScheduler {}
        }

        pub fn get_target_instruction_flags(&self, instr: &Instruction) -> i32 {
            match instr.arch_opcode {
                ArchOpcode::kRiscvEnableDebugTrace
                | ArchOpcode::kRiscvDisableDebugTrace => 0, // kNoOpcodeFlags,
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvAdd32
                | ArchOpcode::kRiscvBitcastDL
                | ArchOpcode::kRiscvBitcastLD
                | ArchOpcode::kRiscvByteSwap64
                | ArchOpcode::kRiscvCvtDL
                | ArchOpcode::kRiscvCvtDUl
                | ArchOpcode::kRiscvCvtSL
                | ArchOpcode::kRiscvCvtSUl
                | ArchOpcode::kRiscvMulHigh64
                | ArchOpcode::kRiscvMulHighU64
                | ArchOpcode::kRiscvAdd64
                | ArchOpcode::kRiscvAddOvf64
                | ArchOpcode::kRiscvClz64
                | ArchOpcode::kRiscvCtz64
                | ArchOpcode::kRiscvDiv64
                | ArchOpcode::kRiscvDivU64
                | ArchOpcode::kRiscvZeroExtendWord
                | ArchOpcode::kRiscvSignExtendWord
                | ArchOpcode::kRiscvMod64
                | ArchOpcode::kRiscvModU64
                | ArchOpcode::kRiscvMul64
                | ArchOpcode::kRiscvMulOvf64
                | ArchOpcode::kRiscvPopcnt64
                | ArchOpcode::kRiscvRor64
                | ArchOpcode::kRiscvSar64
                | ArchOpcode::kRiscvShl64
                | ArchOpcode::kRiscvShr64
                | ArchOpcode::kRiscvSub64
                | ArchOpcode::kRiscvSubOvf64
                | ArchOpcode::kRiscvFloat64RoundDown
                | ArchOpcode::kRiscvFloat64RoundTiesEven
                | ArchOpcode::kRiscvFloat64RoundTruncate
                | ArchOpcode::kRiscvFloat64RoundUp
                | ArchOpcode::kRiscvSub32
                | ArchOpcode::kRiscvTruncLD
                | ArchOpcode::kRiscvTruncLS
                | ArchOpcode::kRiscvTruncUlD
                | ArchOpcode::kRiscvTruncUlS
                | ArchOpcode::kRiscvCmp32
                | ArchOpcode::kRiscvCmpZero32 => 0,
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvAdd32
                | ArchOpcode::kRiscvAddPair
                | ArchOpcode::kRiscvSubPair
                | ArchOpcode::kRiscvMulPair
                | ArchOpcode::kRiscvAndPair
                | ArchOpcode::kRiscvOrPair
                | ArchOpcode::kRiscvXorPair
                | ArchOpcode::kRiscvShlPair
                | ArchOpcode::kRiscvShrPair
                | ArchOpcode::kRiscvSarPair
                | ArchOpcode::kRiscvAddOvf
                | ArchOpcode::kRiscvSubOvf
                | ArchOpcode::kRiscvSub32 => 0,
                ArchOpcode::kRiscvSh1add
                | ArchOpcode::kRiscvSh2add
                | ArchOpcode::kRiscvSh3add => 0,
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvAdduw
                | ArchOpcode::kRiscvSh1adduw
                | ArchOpcode::kRiscvSh2adduw
                | ArchOpcode::kRiscvSh3adduw
                | ArchOpcode::kRiscvSlliuw => 0,
                ArchOpcode::kRiscvAndn
                | ArchOpcode::kRiscvOrn
                | ArchOpcode::kRiscvXnor
                | ArchOpcode::kRiscvClz
                | ArchOpcode::kRiscvCtz
                | ArchOpcode::kRiscvCpop => 0,
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvClzw
                | ArchOpcode::kRiscvCtzw
                | ArchOpcode::kRiscvCpopw => 0,
                ArchOpcode::kRiscvMax
                | ArchOpcode::kRiscvMaxu
                | ArchOpcode::kRiscvMin
                | ArchOpcode::kRiscvMinu
                | ArchOpcode::kRiscvSextb
                | ArchOpcode::kRiscvSexth
                | ArchOpcode::kRiscvZexth
                | ArchOpcode::kRiscvRev8
                | ArchOpcode::kRiscvBclr
                | ArchOpcode::kRiscvBclri
                | ArchOpcode::kRiscvBext
                | ArchOpcode::kRiscvBexti
                | ArchOpcode::kRiscvBinv
                | ArchOpcode::kRiscvBinvi
                | ArchOpcode::kRiscvBset
                | ArchOpcode::kRiscvBseti
                | ArchOpcode::kRiscvAbsD
                | ArchOpcode::kRiscvAbsS
                | ArchOpcode::kRiscvAddD
                | ArchOpcode::kRiscvAddS
                | ArchOpcode::kRiscvAnd
                | ArchOpcode::kRiscvAnd32
                | ArchOpcode::kRiscvAssertEqual
                | ArchOpcode::kRiscvBitcastInt32ToFloat32
                | ArchOpcode::kRiscvBitcastFloat32ToInt32
                | ArchOpcode::kRiscvByteSwap32
                | ArchOpcode::kRiscvCeilWD
                | ArchOpcode::kRiscvCeilWS
                | ArchOpcode::kRiscvClz32
                | ArchOpcode::kRiscvCmp
                | ArchOpcode::kRiscvCmpZero
                | ArchOpcode::kRiscvCmpD
                | ArchOpcode::kRiscvCmpS
                | ArchOpcode::kRiscvCtz32
                | ArchOpcode::kRiscvCvtDS
                | ArchOpcode::kRiscvCvtDUw
                | ArchOpcode::kRiscvCvtDW
                | ArchOpcode::kRiscvCvtSD
                | ArchOpcode::kRiscvCvtSUw
                | ArchOpcode::kRiscvCvtSW
                | ArchOpcode::kRiscvMulHighU32
                | ArchOpcode::kRiscvDiv32
                | ArchOpcode::kRiscvDivD
                | ArchOpcode::kRiscvDivS
                | ArchOpcode::kRiscvDivU32
                | ArchOpcode::kRiscvF64x2Abs
                | ArchOpcode::kRiscvF64x2Sqrt
                | ArchOpcode::kRiscvF64x2Pmin
                | ArchOpcode::kRiscvF64x2Pmax
                | ArchOpcode::kRiscvF64x2ConvertLowI32x4S
                | ArchOpcode::kRiscvF64x2ConvertLowI32x4U
                | ArchOpcode::kRiscvF64x2PromoteLowF32x4
                | ArchOpcode::kRiscvF64x2Ceil
                | ArchOpcode::kRiscvF64x2Floor
                | ArchOpcode::kRiscvF64x2Trunc
                | ArchOpcode::kRiscvF64x2NearestInt
                | ArchOpcode::kRiscvI64x2SplatI32Pair
                | ArchOpcode::kRiscvI64x2ExtractLane
                | ArchOpcode::kRiscvI64x2ReplaceLane
                | ArchOpcode::kRiscvI64x2ReplaceLaneI32Pair
                | ArchOpcode::kRiscvI64x2Shl
                | ArchOpcode::kRiscvI64x2ShrS
                | ArchOpcode::kRiscvI64x2ShrU
                | ArchOpcode::kRiscvF32x4Abs
                | ArchOpcode::kRiscvF32x4ExtractLane
                | ArchOpcode::kRiscvF32x4Sqrt
                | ArchOpcode::kRiscvF64x2Qfma
                | ArchOpcode::kRiscvF64x2Qfms
                | ArchOpcode::kRiscvF32x4Qfma
                | ArchOpcode::kRiscvF32x4Qfms
                | ArchOpcode::kRiscvF32x4ReplaceLane
                | ArchOpcode::kRiscvF32x4SConvertI32x4
                | ArchOpcode::kRiscvF32x4UConvertI32x4
                | ArchOpcode::kRiscvF32x4Pmin
                | ArchOpcode::kRiscvF32x4Pmax
                | ArchOpcode::kRiscvF32x4DemoteF64x2Zero
                | ArchOpcode::kRiscvF32x4Ceil
                | ArchOpcode::kRiscvF32x4Floor
                | ArchOpcode::kRiscvF32x4Trunc
                | ArchOpcode::kRiscvF32x4NearestInt
                | ArchOpcode::kRiscvF64x2ExtractLane
                | ArchOpcode::kRiscvF64x2ReplaceLane
                | ArchOpcode::kRiscvFloat32Max
                | ArchOpcode::kRiscvFloat32Min
                | ArchOpcode::kRiscvFloat32RoundDown
                | ArchOpcode::kRiscvFloat32RoundTiesEven
                | ArchOpcode::kRiscvFloat32RoundTruncate
                | ArchOpcode::kRiscvFloat32RoundUp
                | ArchOpcode::kRiscvFloat64ExtractLowWord32
                | ArchOpcode::kRiscvFloat64ExtractHighWord32
                | ArchOpcode::kRiscvFloat64InsertLowWord32
                | ArchOpcode::kRiscvFloat64InsertHighWord32
                | ArchOpcode::kRiscvFloat64Max
                | ArchOpcode::kRiscvFloat64Min
                | ArchOpcode::kRiscvFloat64SilenceNaN
                | ArchOpcode::kRiscvFloorWD
                | ArchOpcode::kRiscvFloorWS
                | ArchOpcode::kRiscvI64x2SConvertI32x4Low
                | ArchOpcode::kRiscvI64x2SConvertI32x4High
                | ArchOpcode::kRiscvI64x2UConvertI32x4Low
                | ArchOpcode::kRiscvI64x2UConvertI32x4High
                | ArchOpcode::kRiscvI16x8ExtractLaneU
                | ArchOpcode::kRiscvI16x8ExtractLaneS
                | ArchOpcode::kRiscvI16x8ReplaceLane
                | ArchOpcode::kRiscvI16x8Shl
                | ArchOpcode::kRiscvI16x8ShrS
                | ArchOpcode::kRiscvI16x8ShrU
                | ArchOpcode::kRiscvI32x4TruncSatF64x2SZero
                | ArchOpcode::kRiscvI32x4TruncSatF64x2UZero
                | ArchOpcode::kRiscvI32x4ExtractLane
                | ArchOpcode::kRiscvI32x4ReplaceLane
                | ArchOpcode::kRiscvI32x4SConvertF32x4
                | ArchOpcode::kRiscvI32x4Shl
                | ArchOpcode::kRiscvI32x4ShrS
                | ArchOpcode::kRiscvI32x4ShrU
                | ArchOpcode::kRiscvI32x4UConvertF32x4
                | ArchOpcode::kRiscvI8x16ExtractLaneU
                | ArchOpcode::kRiscvI8x16ExtractLaneS
                | ArchOpcode::kRiscvI8x16ReplaceLane
                | ArchOpcode::kRiscvI8x16Shl
                | ArchOpcode::kRiscvI8x16ShrS
                | ArchOpcode::kRiscvI8x16ShrU
                | ArchOpcode::kRiscvI8x16RoundingAverageU
                | ArchOpcode::kRiscvI8x16Popcnt
                | ArchOpcode::kRiscvMaxD
                | ArchOpcode::kRiscvMaxS
                | ArchOpcode::kRiscvMinD
                | ArchOpcode::kRiscvMinS
                | ArchOpcode::kRiscvMod32
                | ArchOpcode::kRiscvModU32
                | ArchOpcode::kRiscvMov
                | ArchOpcode::kRiscvMul32
                | ArchOpcode::kRiscvMulD
                | ArchOpcode::kRiscvMulHigh32
                | ArchOpcode::kRiscvMulOvf32
                | ArchOpcode::kRiscvMulS
                | ArchOpcode::kRiscvNegD
                | ArchOpcode::kRiscvNegS
                | ArchOpcode::kRiscvOr
                | ArchOpcode::kRiscvOr32
                | ArchOpcode::kRiscvPopcnt32
                | ArchOpcode::kRiscvRor32
                | ArchOpcode::kRiscvRoundWD
                | ArchOpcode::kRiscvRoundWS
                | ArchOpcode::kRiscvVnot
                | ArchOpcode::kRiscvS128Select
                | ArchOpcode::kRiscvS128Const
                | ArchOpcode::kRiscvS128Zero
                | ArchOpcode::kRiscvS128Load32Zero
                | ArchOpcode::kRiscvS128Load64Zero
                | ArchOpcode::kRiscvS128AllOnes
                | ArchOpcode::kRiscvV128AnyTrue
                | ArchOpcode::kRiscvI8x16Shuffle
                | ArchOpcode::kRiscvVwmul
                | ArchOpcode::kRiscvVwmulu
                | ArchOpcode::kRiscvVmv
                | ArchOpcode::kRiscvVandVv
                | ArchOpcode::kRiscvVorVv
                | ArchOpcode::kRiscvVnotVv
                | ArchOpcode::kRiscvVxorVv
                | ArchOpcode::kRiscvVmvSx
                | ArchOpcode::kRiscvVmvXs
                | ArchOpcode::kRiscvVfmvVf
                | ArchOpcode::kRiscvVcompress
                | ArchOpcode::kRiscvVaddVv
                | ArchOpcode::kRiscvVwaddVv
                | ArchOpcode::kRiscvVwadduVv
                | ArchOpcode::kRiscvVwadduWx
                | ArchOpcode::kRiscvVsubVv
                | ArchOpcode::kRiscvVnegVv
                | ArchOpcode::kRiscvVfnegVv
                | ArchOpcode::kRiscvVmaxuVv
                | ArchOpcode::kRiscvVmax
                | ArchOpcode::kRiscvVminsVv
                | ArchOpcode::kRiscvVminuVv
                | ArchOpcode::kRiscvVmulVv
                | ArchOpcode::kRiscvVdivu
                | ArchOpcode::kRiscvVsmulVv
                | ArchOpcode::kRiscvVmslt
                | ArchOpcode::kRiscvVgtsVv
                | ArchOpcode::kRiscvVgesVv
                | ArchOpcode::kRiscvVgeuVv
                | ArchOpcode::kRiscvVgtuVv
                | ArchOpcode::kRiscvVeqVv
                | ArchOpcode::kRiscvVneVv
                | ArchOpcode::kRiscvVAbs
                | ArchOpcode::kRiscvVaddSatUVv
                | ArchOpcode::kRiscvVaddSatSVv
                | ArchOpcode::kRiscvVsubSatUVv
                | ArchOpcode::kRiscvVsubSatSVv
                | ArchOpcode::kRiscvVrgather
                | ArchOpcode::kRiscvVslidedown
                | ArchOpcode::kRiscvVredminuVs
                | ArchOpcode::kRiscvVAllTrue
                | ArchOpcode::kRiscvVnclipu
                | ArchOpcode::kRiscvVnclip
                | ArchOpcode::kRiscvVsll
                | ArchOpcode::kRiscvVfaddVv
                | ArchOpcode::kRiscvVfsubVv
                | ArchOpcode::kRiscvVfmulVv
                | ArchOpcode::kRiscvVfdivVv
                | ArchOpcode::kRiscvVfminVv
                | ArchOpcode::kRiscvVfmaxVv
                | ArchOpcode::kRiscvVmfeqVv
                | ArchOpcode::kRiscvVmfneVv
                | ArchOpcode::kRiscvVmfltVv
                | ArchOpcode::kRiscvVmfleVv
                | ArchOpcode::kRiscvVmergeVx
                | ArchOpcode::kRiscvVzextVf2
                | ArchOpcode::kRiscvVsextVf2
                | ArchOpcode::kRiscvSar32
                | ArchOpcode::kRiscvSignExtendByte
                | ArchOpcode::kRiscvSignExtendShort
                | ArchOpcode::kRiscvShl32
                | ArchOpcode::kRiscvShr32
                | ArchOpcode::kRiscvSqrtD
                | ArchOpcode::kRiscvSqrtS
                | ArchOpcode::kRiscvSubD
                | ArchOpcode::kRiscvSubS
                | ArchOpcode::kRiscvTruncUwD
                | ArchOpcode::kRiscvTruncUwS
                | ArchOpcode::kRiscvTruncWD
                | ArchOpcode::kRiscvTruncWS
                | ArchOpcode::kRiscvTst32
                | ArchOpcode::kRiscvXor
                | ArchOpcode::kRiscvXor32 => 0,
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvTst64
                | ArchOpcode::kRiscvLd
                | ArchOpcode::kRiscvLwu
                | ArchOpcode::kRiscvUlwu
                | ArchOpcode::kRiscvWord64AtomicLoadUint64
                | ArchOpcode::kRiscvLoadDecompressTaggedSigned
                | ArchOpcode::kRiscvLoadDecompressTagged
                | ArchOpcode::kRiscvLoadDecodeSandboxedPointer
                | ArchOpcode::kRiscvAtomicLoadDecompressTaggedSigned
                | ArchOpcode::kRiscvAtomicLoadDecompressTagged
                | ArchOpcode::kRiscvAtomicStoreCompressTagged
                | ArchOpcode::kRiscvLoadDecompressProtected => 1, // kIsLoadOperation
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvWord32AtomicPairLoad => 1,
                ArchOpcode::kRiscvLb
                | ArchOpcode::kRiscvLbu
                | ArchOpcode::kRiscvLoadDouble
                | ArchOpcode::kRiscvLh
                | ArchOpcode::kRiscvLhu
                | ArchOpcode::kRiscvLw
                | ArchOpcode::kRiscvLoadFloat
                | ArchOpcode::kRiscvRvvLd
                | ArchOpcode::kRiscvPeek
                | ArchOpcode::kRiscvUld
                | ArchOpcode::kRiscvULoadDouble
                | ArchOpcode::kRiscvUlh
                | ArchOpcode::kRiscvUlhu
                | ArchOpcode::kRiscvUlw
                | ArchOpcode::kRiscvULoadFloat
                | ArchOpcode::kRiscvS128LoadSplat
                | ArchOpcode::kRiscvS128Load64ExtendU
                | ArchOpcode::kRiscvS128Load64ExtendS
                | ArchOpcode::kRiscvS128LoadLane => 1,

                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvSd
                | ArchOpcode::kRiscvUsd
                | ArchOpcode::kRiscvWord64AtomicStoreWord64
                | ArchOpcode::kRiscvWord64AtomicAddUint64
                | ArchOpcode::kRiscvWord64AtomicSubUint64
                | ArchOpcode::kRiscvWord64AtomicAndUint64
                | ArchOpcode::kRiscvWord64AtomicOrUint64
                | ArchOpcode::kRiscvWord64AtomicXorUint64
                | ArchOpcode::kRiscvWord64AtomicExchangeUint64
                | ArchOpcode::kRiscvWord64AtomicCompareExchangeUint64
                | ArchOpcode::kRiscvStoreCompressTagged
                | ArchOpcode::kRiscvStoreEncodeSandboxedPointer
                | ArchOpcode::kRiscvStoreIndirectPointer => 2, // kHasSideEffect
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvWord32AtomicPairStore
                | ArchOpcode::kRiscvWord32AtomicPairAdd
                | ArchOpcode::kRiscvWord32AtomicPairSub
                | ArchOpcode::kRiscvWord32AtomicPairAnd
                | ArchOpcode::kRiscvWord32AtomicPairOr
                | ArchOpcode::kRiscvWord32AtomicPairXor
                | ArchOpcode::kRiscvWord32AtomicPairExchange
                | ArchOpcode::kRiscvWord32AtomicPairCompareExchange => 2,
                ArchOpcode::kRiscvModD
                | ArchOpcode::kRiscvModS
                | ArchOpcode::kRiscvRvvSt
                | ArchOpcode::kRiscvPush
                | ArchOpcode::kRiscvSb
                | ArchOpcode::kRiscvStoreDouble
                | ArchOpcode::kRiscvSh
                | ArchOpcode::kRiscvStackClaim
                | ArchOpcode::kRiscvStoreToStackSlot
                | ArchOpcode::kRiscvSw
                | ArchOpcode::kRiscvStoreFloat
                | ArchOpcode::kRiscvUStoreDouble
                | ArchOpcode::kRiscvUsh
                | ArchOpcode::kRiscvUsw
                | ArchOpcode::kRiscvUStoreFloat
                | ArchOpcode::kRiscvSync
                | ArchOpcode::kRiscvS128StoreLane => 2,

                ArchOpcode::kArchNop => 0, // Example
                _ => {
                    // Assuming COMMON_ARCH_OPCODE_LIST and UNREACHABLE are not directly translatable.
                    // Replace with a default or error handling.
                    eprintln!("Unhandled arch_opcode: {:?}", instr.arch_opcode);
                    0 // Or panic!("Unexpected arch_opcode")
                }
            }
        }

        pub fn get_instruction_latency(&self, instr: &Instruction) -> i32 {
            match instr.arch_opcode {
                ArchOpcode::kArchCallCodeObject
                | ArchOpcode::kArchCallWasmFunction => call_latency(),
                ArchOpcode::kArchTailCallCodeObject
                | ArchOpcode::kArchTailCallWasm
                | ArchOpcode::kArchTailCallAddress => jump_latency(),
                ArchOpcode::kArchCallJSFunction => {
                    let mut latency = 0;
                    if cfg!(debug_assertions) {
                        latency = 1 + assert_latency();
                    }
                    latency + 1 + add64_latency(false) + call_latency()
                }
                ArchOpcode::kArchPrepareCallCFunction => prepare_call_c_function_latency(),
                ArchOpcode::kArchSaveCallerRegisters => {
                    //Assuming MiscField::decode and SaveFPRegsMode are implemented
                    //let fp_mode = SaveFPRegsMode::try_from(instr.opcode).unwrap();
                    let fp_mode = SaveFPRegsMode::kSave; // Placeholder
                    push_caller_saved_latency(fp_mode)
                }
                ArchOpcode::kArchRestoreCallerRegisters => {
                    //Assuming MiscField::decode and SaveFPRegsMode are implemented
                    //let fp_mode = SaveFPRegsMode::try_from(instr.opcode).unwrap();
                    let fp_mode = SaveFPRegsMode::kSave; // Placeholder
                    pop_caller_saved_latency(fp_mode)
                }
                ArchOpcode::kArchPrepareTailCall => 2,
                ArchOpcode::kArchCallCFunction => call_c_function_latency(),
                ArchOpcode::kArchJmp => assemble_arch_jump_latency(),
                ArchOpcode::kArchTableSwitch => assemble_arch_table_switch_latency(),
                ArchOpcode::kArchAbortCSADcheck => call_latency() + 1,
                ArchOpcode::kArchDebugBreak => 1,
                ArchOpcode::kArchComment
                | ArchOpcode::kArchNop
                | ArchOpcode::kArchThrowTerminator
                | ArchOpcode::kArchDeoptimize => 0,
                ArchOpcode::kArchRet => assembler_return_latency(),
                ArchOpcode::kArchFramePointer => 1,
                ArchOpcode::kArchParentFramePointer => aligned_memory_latency(),
                ArchOpcode::kArchTruncateDoubleToI => truncate_double_to_i_delayed_latency(),
                ArchOpcode::kArchStoreWithWriteBarrier => {
                    add64_latency(true) + 1 + check_page_flag_latency()
                }
                ArchOpcode::kArchStackSlot => {
                    add64_latency(false) + and_latency(false) + assert_latency()
                        + add64_latency(false) + and_latency(false) + branch_short_latency()
                        + 1 + sub64_latency(true) + add64_latency(true)
                }
                ArchOpcode::kIeee754Float64Acos
                | ArchOpcode::kIeee754Float64Acosh
                | ArchOpcode::kIeee754Float64Asin
                | ArchOpcode::kIeee754Float64Asinh
                | ArchOpcode::kIeee754Float64Atan
                | ArchOpcode::kIeee754Float64Atanh
                | ArchOpcode::kIeee754Float64Atan2
                | ArchOpcode::kIeee754Float64Cos
                | ArchOpcode::kIeee754Float64Cosh
                | ArchOpcode::kIeee754Float64Cbrt
                | ArchOpcode::kIeee754Float64Exp
                | ArchOpcode::kIeee754Float64Expm1
                | ArchOpcode::kIeee754Float64Log
                | ArchOpcode::kIeee754Float64Log1p
                | ArchOpcode::kIeee754Float64Log10
                | ArchOpcode::kIeee754Float64Log2
                | ArchOpcode::kIeee754Float64Pow
                | ArchOpcode::kIeee754Float64Sin
                | ArchOpcode::kIeee754Float64Sinh
                | ArchOpcode::kIeee754Float64Tan
                | ArchOpcode::kIeee754Float64Tanh => {
                    prepare_call_c_function_latency() + mov_to_float_parameters_latency()
                        + call_c_function_latency() + mov_from_float_result_latency()
                }
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvAdd32 | ArchOpcode::kRiscvAdd64 => {
                    add64_latency(instr.input_at_is_register(1))
                }
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvAddOvf64 => add_overflow64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvSub32 | ArchOpcode::kRiscvSub64 => {
                    sub64_latency(instr.input_at_is_register(1))
                }
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvSubOvf64 => sub_overflow64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvMulHigh64 => mulh64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvMul64 => mul64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvMulOvf64 => mul_overflow64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvDiv64 => {
                    let latency = div64_latency();
                    latency + movz_latency()
                }
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvDivU64 => {
                    let latency = divu64_latency();
                    latency + movz_latency()
                }
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvMod64 => mod64_latency(),
                #[cfg(target_arch = "riscv64")]
                ArchOpcode::kRiscvModU64 => modu64_latency(),
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvAdd32 => add64_latency(instr.input_at_is_register(1)),
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvAddOvf => add_overflow64_latency(),
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvSub32 => sub64_latency(instr.input_at_is_register(1)),
                #[cfg(target_arch = "riscv32")]
                ArchOpcode::kRiscvSubOvf => sub_overflow64_latency(),
                ArchOpcode::kRiscvMul32 => mul32_latency(),
                ArchOpcode::kRiscvMulOvf32 => mul_overflow32_latency(),
                ArchOpcode::kRiscvMulHigh32 => mulh32_latency(),
                ArchOpcode::kRiscvMulHighU32 => mulhu32_latency(),
                ArchOpcode::kRiscvDiv32 => {
                    let latency = div32_latency(instr.input_at_is_register(1));
                    latency + movz_latency()
                }
                ArchOpcode::kRiscvDivU32 => {
                    let latency = divu32_latency(instr.input_at_is_register(1));
                    latency + movz_latency()
                }
                ArchOpcode::kRiscvMod32 => mod32_latency(),
                ArchOpcode
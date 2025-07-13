// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-scheduler-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod codegen {
    pub mod macro_assembler {
        pub fn ActivationFrameAlignment() -> i32 {
            16 // Placeholder, needs actual alignment value
        }
    }
}

pub mod compiler {
    pub mod backend {
        pub mod instruction_scheduler {
            pub struct InstructionScheduler {}

            impl InstructionScheduler {
                pub fn SchedulerSupported() -> bool {
                    true
                }

                pub fn GetTargetInstructionFlags(
                    &self,
                    instr: &Instruction,
                ) -> i32 {
                    use InstructionCode::*;

                    match instr.arch_opcode() {
                        kMips64AbsD | kMips64AbsS | kMips64Add | kMips64AddD
                        | kMips64AddS | kMips64And | kMips64And32
                        | kMips64AssertEqual | kMips64BitcastDL | kMips64BitcastLD
                        | kMips64ByteSwap32 | kMips64ByteSwap64 | kMips64CeilWD
                        | kMips64CeilWS | kMips64Clz | kMips64Cmp | kMips64CmpD
                        | kMips64CmpS | kMips64Ctz | kMips64CvtDL | kMips64CvtDS
                        | kMips64CvtDUl | kMips64CvtDUw | kMips64CvtDW | kMips64CvtSD
                        | kMips64CvtSL | kMips64CvtSUl | kMips64CvtSUw | kMips64CvtSW
                        | kMips64DMulHigh | kMips64DMulHighU | kMips64DMulOvf
                        | kMips64MulHighU | kMips64Dadd | kMips64DaddOvf | kMips64Dclz
                        | kMips64Dctz | kMips64Ddiv | kMips64DdivU | kMips64Dext
                        | kMips64Dins | kMips64Div | kMips64DivD | kMips64DivS
                        | kMips64DivU | kMips64Dlsa | kMips64Dmod | kMips64DmodU
                        | kMips64Dmul | kMips64Dpopcnt | kMips64Dror | kMips64Dsar
                        | kMips64Dshl | kMips64Dshr | kMips64Dsub | kMips64DsubOvf
                        | kMips64Ext | kMips64F64x2Abs | kMips64F64x2Neg
                        | kMips64F64x2Sqrt | kMips64F64x2Add | kMips64F64x2Sub
                        | kMips64F64x2Mul | kMips64F64x2Div | kMips64F64x2Min
                        | kMips64F64x2Max | kMips64F64x2Eq | kMips64F64x2Ne
                        | kMips64F64x2Lt | kMips64F64x2Le | kMips64F64x2Pmin
                        | kMips64F64x2Pmax | kMips64F64x2Ceil | kMips64F64x2Floor
                        | kMips64F64x2Trunc | kMips64F64x2NearestInt
                        | kMips64F64x2ConvertLowI32x4S | kMips64F64x2ConvertLowI32x4U
                        | kMips64F64x2PromoteLowF32x4 | kMips64I64x2Splat
                        | kMips64I64x2ExtractLane | kMips64I64x2ReplaceLane
                        | kMips64I64x2Add | kMips64I64x2Sub | kMips64I64x2Mul
                        | kMips64I64x2Neg | kMips64I64x2Shl | kMips64I64x2ShrS
                        | kMips64I64x2ShrU | kMips64I64x2BitMask | kMips64I64x2Eq
                        | kMips64I64x2Ne | kMips64I64x2GtS | kMips64I64x2GeS
                        | kMips64I64x2Abs | kMips64I64x2SConvertI32x4Low
                        | kMips64I64x2SConvertI32x4High | kMips64I64x2UConvertI32x4Low
                        | kMips64I64x2UConvertI32x4High | kMips64ExtMulLow
                        | kMips64ExtMulHigh | kMips64ExtAddPairwise | kMips64F32x4Abs
                        | kMips64F32x4Add | kMips64F32x4Eq | kMips64F32x4ExtractLane
                        | kMips64F32x4Lt | kMips64F32x4Le | kMips64F32x4Max
                        | kMips64F32x4Min | kMips64F32x4Mul | kMips64F32x4Div
                        | kMips64F32x4Ne | kMips64F32x4Neg | kMips64F32x4Sqrt
                        | kMips64F32x4ReplaceLane | kMips64F32x4SConvertI32x4
                        | kMips64F32x4Splat | kMips64F32x4Sub | kMips64F32x4UConvertI32x4
                        | kMips64F32x4Pmin | kMips64F32x4Pmax | kMips64F32x4Ceil
                        | kMips64F32x4Floor | kMips64F32x4Trunc | kMips64F32x4NearestInt
                        | kMips64F32x4DemoteF64x2Zero | kMips64F64x2Splat
                        | kMips64F64x2ExtractLane | kMips64F64x2ReplaceLane
                        | kMips64Float32Max | kMips64Float32Min | kMips64Float32RoundDown
                        | kMips64Float32RoundTiesEven | kMips64Float32RoundTruncate
                        | kMips64Float32RoundUp | kMips64Float64ExtractLowWord32
                        | kMips64Float64ExtractHighWord32 | kMips64Float64FromWord32Pair
                        | kMips64Float64InsertLowWord32 | kMips64Float64InsertHighWord32
                        | kMips64Float64Max | kMips64Float64Min | kMips64Float64RoundDown
                        | kMips64Float64RoundTiesEven | kMips64Float64RoundTruncate
                        | kMips64Float64RoundUp | kMips64Float64SilenceNaN | kMips64FloorWD
                        | kMips64FloorWS | kMips64I16x8Add | kMips64I16x8AddSatS
                        | kMips64I16x8AddSatU | kMips64I16x8Eq | kMips64I16x8ExtractLaneU
                        | kMips64I16x8ExtractLaneS | kMips64I16x8GeS | kMips64I16x8GeU
                        | kMips64I16x8GtS | kMips64I16x8GtU | kMips64I16x8MaxS
                        | kMips64I16x8MaxU | kMips64I16x8MinS | kMips64I16x8MinU
                        | kMips64I16x8Mul | kMips64I16x8Ne | kMips64I16x8Neg
                        | kMips64I16x8ReplaceLane | kMips64I8x16SConvertI16x8
                        | kMips64I16x8SConvertI32x4 | kMips64I16x8SConvertI8x16High
                        | kMips64I16x8SConvertI8x16Low | kMips64I16x8Shl | kMips64I16x8ShrS
                        | kMips64I16x8ShrU | kMips64I16x8Splat | kMips64I16x8Sub
                        | kMips64I16x8SubSatS | kMips64I16x8SubSatU | kMips64I8x16UConvertI16x8
                        | kMips64I16x8UConvertI32x4 | kMips64I16x8UConvertI8x16High
                        | kMips64I16x8UConvertI8x16Low | kMips64I16x8RoundingAverageU
                        | kMips64I16x8Abs | kMips64I16x8BitMask | kMips64I16x8Q15MulRSatS
                        | kMips64I32x4Add | kMips64I32x4Eq | kMips64I32x4ExtractLane
                        | kMips64I32x4GeS | kMips64I32x4GeU | kMips64I32x4GtS
                        | kMips64I32x4GtU | kMips64I32x4MaxS | kMips64I32x4MaxU
                        | kMips64I32x4MinS | kMips64I32x4MinU | kMips64I32x4Mul
                        | kMips64I32x4Ne | kMips64I32x4Neg | kMips64I32x4ReplaceLane
                        | kMips64I32x4SConvertF32x4 | kMips64I32x4SConvertI16x8High
                        | kMips64I32x4SConvertI16x8Low | kMips64I32x4Shl | kMips64I32x4ShrS
                        | kMips64I32x4ShrU | kMips64I32x4Splat | kMips64I32x4Sub
                        | kMips64I32x4UConvertF32x4 | kMips64I32x4UConvertI16x8High
                        | kMips64I32x4UConvertI16x8Low | kMips64I32x4Abs | kMips64I32x4BitMask
                        | kMips64I32x4DotI16x8S | kMips64I32x4TruncSatF64x2SZero
                        | kMips64I32x4TruncSatF64x2UZero | kMips64I8x16Add | kMips64I8x16AddSatS
                        | kMips64I8x16AddSatU | kMips64I8x16Eq | kMips64I8x16ExtractLaneU
                        | kMips64I8x16ExtractLaneS | kMips64I8x16GeS | kMips64I8x16GeU
                        | kMips64I8x16GtS | kMips64I8x16GtU | kMips64I8x16MaxS
                        | kMips64I8x16MaxU | kMips64I8x16MinS | kMips64I8x16MinU
                        | kMips64I8x16Ne | kMips64I8x16Neg | kMips64I8x16ReplaceLane
                        | kMips64I8x16Shl | kMips64I8x16ShrS | kMips64I8x16ShrU
                        | kMips64I8x16Splat | kMips64I8x16Sub | kMips64I8x16SubSatS
                        | kMips64I8x16SubSatU | kMips64I8x16RoundingAverageU
                        | kMips64I8x16Abs | kMips64I8x16Popcnt | kMips64I8x16BitMask
                        | kMips64Ins | kMips64Lsa | kMips64MaxD | kMips64MaxS
                        | kMips64MinD | kMips64MinS | kMips64Mod | kMips64ModU
                        | kMips64Mov | kMips64Mul | kMips64MulD | kMips64MulHigh
                        | kMips64MulOvf | kMips64MulS | kMips64NegD | kMips64NegS
                        | kMips64Nor | kMips64Nor32 | kMips64Or | kMips64Or32
                        | kMips64Popcnt | kMips64Ror | kMips64RoundWD | kMips64RoundWS
                        | kMips64S128And | kMips64S128Or | kMips64S128Not
                        | kMips64S128Select | kMips64S128AndNot | kMips64S128Xor
                        | kMips64S128Const | kMips64S128Zero | kMips64S128AllOnes
                        | kMips64S16x8InterleaveEven | kMips64S16x8InterleaveOdd
                        | kMips64S16x8InterleaveLeft | kMips64S16x8InterleaveRight
                        | kMips64S16x8PackEven | kMips64S16x8PackOdd | kMips64S16x2Reverse
                        | kMips64S16x4Reverse | kMips64I64x2AllTrue | kMips64I32x4AllTrue
                        | kMips64I16x8AllTrue | kMips64I8x16AllTrue | kMips64V128AnyTrue
                        | kMips64S32x4InterleaveEven | kMips64S32x4InterleaveOdd
                        | kMips64S32x4InterleaveLeft | kMips64S32x4InterleaveRight
                        | kMips64S32x4PackEven | kMips64S32x4PackOdd | kMips64S32x4Shuffle
                        | kMips64S8x16Concat | kMips64S8x16InterleaveEven
                        | kMips64S8x16InterleaveOdd | kMips64S8x16InterleaveLeft
                        | kMips64S8x16InterleaveRight | kMips64S8x16PackEven
                        | kMips64S8x16PackOdd | kMips64S8x2Reverse | kMips64S8x4Reverse
                        | kMips64S8x8Reverse | kMips64I8x16Shuffle | kMips64I8x16Swizzle
                        | kMips64Sar | kMips64Seb | kMips64Seh | kMips64Shl
                        | kMips64Shr | kMips64SqrtD | kMips64SqrtS | kMips64Sub
                        | kMips64SubD | kMips64SubS | kMips64TruncLD | kMips64TruncLS
                        | kMips64TruncUlD | kMips64TruncUlS | kMips64TruncUwD
                        | kMips64TruncUwS | kMips64TruncWD | kMips64TruncWS | kMips64Tst
                        | kMips64Xor | kMips64Xor32 => {
                            kNoOpcodeFlags
                        }

                        kMips64Lb | kMips64Lbu | kMips64Ld | kMips64Ldc1 | kMips64Lh
                        | kMips64Lhu | kMips64Lw | kMips64Lwc1 | kMips64Lwu
                        | kMips64MsaLd | kMips64Peek | kMips64Uld | kMips64Uldc1
                        | kMips64Ulh | kMips64Ulhu | kMips64Ulw | kMips64Ulwu
                        | kMips64Ulwc1 | kMips64S128LoadSplat | kMips64S128Load8x8S
                        | kMips64S128Load8x8U | kMips64S128Load16x4S | kMips64S128Load16x4U
                        | kMips64S128Load32x2S | kMips64S128Load32x2U | kMips64S128Load32Zero
                        | kMips64S128Load64Zero | kMips64S128LoadLane
                        | kMips64Word64AtomicLoadUint64 => {
                            kIsLoadOperation
                        }

                        kMips64ModD | kMips64MsaSt | kMips64Push | kMips64Sb | kMips64Sd
                        | kMips64Sdc1 | kMips64Sh | kMips64StackClaim | kMips64StoreToStackSlot
                        | kMips64Sw | kMips64Swc1 | kMips64Usd | kMips64Usdc1
                        | kMips64Ush | kMips64Usw | kMips64Uswc1 | kMips64Sync
                        | kMips64S128StoreLane | kMips64StoreCompressTagged
                        | kMips64Word64AtomicStoreWord64 | kMips64Word64AtomicAddUint64
                        | kMips64Word64AtomicSubUint64 | kMips64Word64AtomicAndUint64
                        | kMips64Word64AtomicOrUint64 | kMips64Word64AtomicXorUint64
                        | kMips64Word64AtomicExchangeUint64
                        | kMips64Word64AtomicCompareExchangeUint64 => {
                            kHasSideEffect
                        }

                        InstructionCode::kArchCallCodeObject
                        | InstructionCode::kArchJmp
                        | InstructionCode::kArchTableSwitch
                        | InstructionCode::kArchRet
                        | InstructionCode::kArchTailCallAddress
                        | InstructionCode::kArchTailCallCodeObject
                        | InstructionCode::kArchDebugBreak
                        | InstructionCode::kArchComment
                        | InstructionCode::kArchNop
                        | InstructionCode::kArchThrowTerminator
                        | InstructionCode::kArchDeoptimize
                        | InstructionCode::kArchFramePointer
                        | InstructionCode::kArchParentFramePointer
                        | InstructionCode::kArchTruncateDoubleToI
                        | InstructionCode::kArchStoreWithWriteBarrier
                        | InstructionCode::kArchStackSlot
                        | InstructionCode::kIeee754Float64Acos
                        | InstructionCode::kIeee754Float64Acosh
                        | InstructionCode::kIeee754Float64Asin
                        | InstructionCode::kIeee754Float64Asinh
                        | InstructionCode::kIeee754Float64Atan
                        | InstructionCode::kIeee754Float64Atanh
                        | InstructionCode::kIeee754Float64Atan2
                        | InstructionCode::kIeee754Float64Cos
                        | InstructionCode::kIeee754Float64Cosh
                        | InstructionCode::kIeee754Float64Cbrt
                        | InstructionCode::kIeee754Float64Exp
                        | InstructionCode::kIeee754Float64Expm1
                        | InstructionCode::kIeee754Float64Log
                        | InstructionCode::kIeee754Float64Log1p
                        | InstructionCode::kIeee754Float64Log10
                        | InstructionCode::kIeee754Float64Log2
                        | InstructionCode::kIeee754Float64Pow
                        | InstructionCode::kIeee754Float64Sin
                        | InstructionCode::kIeee754Float64Sinh
                        | InstructionCode::kIeee754Float64Tan
                        | InstructionCode::kIeee754Float64Tanh
                        | InstructionCode::kArchPrepareCallCFunction
                        | InstructionCode::kArchCallCFunction
                        | InstructionCode::kArchAbortCSADcheck
                        | InstructionCode::kArchSaveCallerRegisters
                        | InstructionCode::kArchRestoreCallerRegisters
                        | InstructionCode::kArchPrepareTailCall
                        | InstructionCode::kArchCallJSFunction
                        | InstructionCode::kArchDebugBreak
                        | InstructionCode::kAtomicLoadInt8
                        | InstructionCode::kAtomicLoadUint8
                        | InstructionCode::kAtomicLoadInt16
                        | InstructionCode::kAtomicLoadUint16
                        | InstructionCode::kAtomicLoadWord32
                        | InstructionCode::kAtomicStoreWord8
                        | InstructionCode::kAtomicStoreWord16
                        | InstructionCode::kAtomicStoreWord32
                        | InstructionCode::kAtomicExchangeInt8
                        | InstructionCode::kAtomicExchangeUint8
                        | InstructionCode::kAtomicExchangeInt16
                        | InstructionCode::kAtomicExchangeUint16
                        | InstructionCode::kAtomicExchangeWord32
                        | InstructionCode::kAtomicCompareExchangeInt8
                        | InstructionCode::kAtomicCompareExchangeUint8
                        | InstructionCode::kAtomicCompareExchangeInt16
                        | InstructionCode::kAtomicCompareExchangeUint16
                        | InstructionCode::kAtomicCompareExchangeWord32
                        | InstructionCode::kMips64AssertEqual => {
                            panic!("UNREACHABLE");
                        }

                        #[cfg(V8_ENABLE_WEBASSEMBLY)]
                        InstructionCode::kArchCallWasmFunction => {
                            panic!("UNREACHABLE");
                        }
                        #[cfg(V8_ENABLE_WEBASSEMBLY)]
                        InstructionCode::kArchTailCallWasm => {
                            panic!("UNREACHABLE");
                        }
                    }
                }

                pub fn GetInstructionLatency(&self, instr: &Instruction) -> i32 {
                    use InstructionCode::*;

                    match instr.arch_opcode() {
                        kArchCallCodeObject => CallLatency(),
                        #[cfg(V8_ENABLE_WEBASSEMBLY)]
                        kArchCallWasmFunction => CallLatency(),
                        kArchTailCallCodeObject => JumpLatency(),
                        #[cfg(V8_ENABLE_WEBASSEMBLY)]
                        kArchTailCallWasm => JumpLatency(),
                        kArchTailCallAddress => JumpLatency(),
                        kArchCallJSFunction => {
                            let mut latency = 0;
                            if v8_flags.debug_code {
                                latency = 1 + AssertLatency();
                            }
                            latency + 1 + DadduLatency(false) + CallLatency()
                        }
                        kArchPrepareCallCFunction => PrepareCallCFunctionLatency(),
                        kArchSaveCallerRegisters => {
                            let fp_mode = SaveFPRegsMode::decode(instr.opcode());
                            PushCallerSavedLatency(fp_mode)
                        }
                        kArchRestoreCallerRegisters => {
                            let fp_mode = SaveFPRegsMode::decode(instr.opcode());
                            PopCallerSavedLatency(fp_mode)
                        }
                        kArchPrepareTailCall => 2,
                        kArchCallCFunction => CallCFunctionLatency(),
                        kArchJmp => AssembleArchJumpLatency(),
                        kArchTableSwitch => AssembleArchTableSwitchLatency(),
                        kArchAbortCSADcheck => CallLatency() + 1,
                        kArchDebugBreak => 1,
                        kArchComment | kArchNop | kArchThrowTerminator | kArchDeoptimize => 0,
                        kArchRet => AssemblerReturnLatency(),
                        kArchFramePointer => 1,
                        kArchParentFramePointer => AlignedMemoryLatency(),
                        kArchTruncateDoubleToI => TruncateDoubleToIDelayedLatency(),
                        kArchStoreWithWriteBarrier => {
                            DadduLatency() + 1 + CheckPageFlagLatency()
                        }
                        kArchStackSlot => {
                            DadduLatency(false) + AndLatency(false) + AssertLatency()
                                + DadduLatency(false) + AndLatency(false) + BranchShortLatency(BranchDelaySlot::PROTECT)
                                + 1 + DsubuLatency() + DadduLatency()
                        }
                        kIeee754Float64Acos | kIeee754Float64Acosh | kIeee754Float64Asin
                        | kIeee754Float64Asinh | kIeee754Float64Atan | kIeee754Float64Atanh
                        | kIeee754Float64Atan2 | kIeee754Float64Cos | kIeee754Float64Cosh
                        | kIeee754Float64Cbrt | kIeee754Float64Exp | kIeee754Float64Expm1
                        | kIeee754Float64Log | kIeee754Float64Log1p | kIeee754Float64Log10
                        | kIeee754Float64Log2 | kIeee754Float64Pow | kIeee754Float64Sin
                        | kIeee754Float64Sinh | kIeee754Float64Tan | kIeee754Float64Tanh => {
                            PrepareCallCFunctionLatency() + MovToFloatParametersLatency()
                                + CallCFunctionLatency() + MovFromFloatResultLatency()
                        }
                        kMips64Add | kMips64Dadd => {
                            DadduLatency(instr.InputAt(1).IsRegister())
                        }
                        kMips64DaddOvf => DaddOverflowLatency(),
                        kMips64Sub | kMips64Dsub => {
                            DsubuLatency(instr.InputAt(1).IsRegister())
                        }
                        kMips64DsubOvf => DsubOverflowLatency(),
                        kMips64Mul => MulLatency(),
                        kMips64MulOvf | kMips64DMulOvf => MulOverflowLatency(),
                        kMips64MulHigh => MulhLatency(),
                        kMips64MulHighU => MulhuLatency(),
                        kMips64DMulHigh => DMulhLatency(),
                        kMips64Div => {
                            let mut latency = DivLatency(instr.InputAt(1).IsRegister());
                            if kArchVariant >= ArchitectureVariant::kMips64r6 {
                                latency += 1;
                            } else {
                                latency += MovzLatency();
                            }
                            latency
                        }
                        kMips64DivU => {
                            let mut latency = DivuLatency(instr.InputAt(1).IsRegister());
                            if kArchVariant >= ArchitectureVariant::kMips64r6 {
                                latency += 1;
                            } else {
                                latency += MovzLatency();
                            }
                            latency
                        }
                        kMips64Mod => ModLatency(),
                        kMips64ModU => ModuLatency(),
                        kMips64Dmul => DmulLatency(),
                        kMips64Ddiv => {
                            let mut latency = DdivLatency();
                            if kArchVariant >= ArchitectureVariant::kMips64r6 {
                                latency += 1;
                            } else {
                                latency += MovzLatency();
                            }
                            latency
                        }
                        kMips64DdivU => {
                            let mut latency = DdivuLatency();
                            if kArchVariant >= ArchitectureVariant::kMips64r6 {
                                latency += 1;
                            } else {
                                latency += MovzLatency();
                            }
                            latency
                        }
                        kMips64Dmod => DmodLatency(),
                        kMips64DmodU => DmoduLatency(),
                        kMips64Dlsa | kMips64Lsa => DlsaLatency(),
                        kMips64And => AndLatency(instr.InputAt(1).IsRegister()),
                        kMips64And32 => {
                            let is_operand_register = instr.InputAt(1).IsRegister();
                            let latency = AndLatency(is_operand_register);
                            if is_operand_register {
                                latency + 2
                            } else {
                                latency + 1
                            }
                        }
                        kMips64Or => OrLatency(instr.InputAt(1).IsRegister()),
                        kMips64Or32 => {
                            let is_operand_register = instr.InputAt(1).IsRegister();
                            let latency = OrLatency(is_operand_register);
                            if is_operand_register {
                                latency + 2
                            } else {
                                latency + 1
                            }
                        }
                        kMips64Nor => NorLatency(instr.InputAt(1).IsRegister()),
                        kMips64Nor32 => {
                            let is_operand_register = instr.InputAt(1).IsRegister();
                            let latency = NorLatency(is_operand_register);
                            if is_operand_register {
                                latency + 2
                            } else {
                                latency + 1
                            }
                        }
                        kMips64Xor => XorLatency(instr.InputAt(1).IsRegister()),
                        kMips64Xor32 => {
                            let is_operand_register = instr.InputAt(1).IsRegister();
                            let latency = XorLatency(is_operand_register);
                            if is_operand_register {
                                latency + 2
                            } else {
                                latency + 1
                            }
                        }
                        kMips64Clz | kMips64Dclz => DclzLatency(),
                        kMips64Ctz => CtzLatency(),
                        kMips64Dctz => DctzLatency(),
                        kMips64Popcnt => PopcntLatency(),
                        kMips64Dpopcnt => DpopcntLatency(),
                        kMips64Shl => 1,
                        kMips64Shr | kMips64Sar => 2,
                        kMips64Ext | kMips64Ins | kMips64Dext | kMips64Dins | kMips64Dshl
                        | kMips64Dshr | kMips64Dsar | kMips64Ror | kMips64Dror => 1,
                        kMips64Tst => AndLatency(instr.InputAt(1).IsRegister()),
                        kMips64Mov => 1,
                        kMips64CmpS => MoveLatency() + CompareF32Latency(),
                        kMips64AddS => Latency::ADD_S,
                        kMips64SubS => Latency::SUB_S,
                        kMips64MulS => Latency::MUL_S,
                        kMips64DivS => Latency::DIV_S,
                        kMips64AbsS => Latency::ABS_S,
                        kMips64NegS => NegdLatency(),
                        kMips64SqrtS => Latency::SQRT_S,
                        kMips64MaxS => Latency::MAX_S,
                        kMips64MinS => Latency::MIN_S,
                        kMips64CmpD => MoveLatency() + CompareF64Latency(),
                        kMips64AddD => Latency::ADD_D,
                        kMips64SubD => Latency::SUB_D,
                        kMips64MulD => Latency::MUL_D,
                        kMips64DivD => Latency::DIV_D,
                        kMips64ModD => {
                            PrepareCallCFunctionLatency() + MovToFloat

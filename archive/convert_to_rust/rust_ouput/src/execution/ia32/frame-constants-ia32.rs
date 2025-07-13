// Converted from V8 C++ source files:
// Header: frame-constants-ia32.h
// Implementation: frame-constants-ia32.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn CountPopulation(bits: u32) -> i32 {
            bits.count_ones() as i32
        }
    }
    pub mod macros{
        #[macro_export]
        macro_rules! USE {
            ($var:ident) => {
                let _ = $var;
            };
        }
    }
}

pub mod codegen {
    pub mod register {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Register(pub i16);

        impl Register {
            pub const INVALID: Register = Register(-1);

            pub fn code(&self) -> i32 {
                self.0 as i32
            }

            pub fn from_code(code: i32) -> Self {
                Register(code as i16)
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct RegList {
            bits_: u32,
        }

        impl RegList {
            pub const DEFAULT: RegList = RegList { bits_: 0 };

            pub fn bits(&self) -> u32 {
                self.bits_
            }

            pub fn Count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct DoubleRegList {
            bits_: u32,
        }

        impl DoubleRegList {
            pub const DEFAULT: DoubleRegList = DoubleRegList { bits_: 0 };

            pub fn bits(&self) -> u32 {
                self.bits_
            }

            pub fn Count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }

        pub const eax: Register = Register(0);
        pub const ecx: Register = Register(1);
        pub const edx: Register = Register(2);
        pub const ebx: Register = Register(3);
        pub const esp: Register = Register(4);
        pub const ebp: Register = Register(5);
        pub const esi: Register = Register(6);
        pub const edi: Register = Register(7);
        pub const xmm0: Register = Register(8);
        pub const xmm1: Register = Register(9);
        pub const xmm2: Register = Register(10);
        pub const xmm3: Register = Register(11);
        pub const xmm4: Register = Register(12);
        pub const xmm5: Register = Register(13);
        pub const xmm6: Register = Register(14);
        pub const xmm7: Register = Register(15);
    }
}

pub mod execution {
    pub mod frame_constants {
        pub struct AllStatic {}
        pub struct TypedFrameConstants {}

        pub const kSimd128Size: i32 = 16;
        pub fn TYPED_FRAME_PUSHED_VALUE_OFFSET(n: i32) -> i32 {
             -((n + 2) * kSystemPointerSize)
        }

    }
    pub mod frames {
        use crate::codegen::register::Register;

        pub struct JavaScriptFrame {}

        impl JavaScriptFrame {
            pub fn fp_register() -> Register {
                Register(5) // Assuming ebp is 5 based on ia32 register definitions
            }
            pub fn context_register() -> Register {
                Register(6) // Assuming esi is 6 based on ia32 register definitions
            }
            pub fn constant_pool_pointer_register() -> Register {
                panic!("UNREACHABLE");
            }
        }

        pub struct UnoptimizedFrameConstants {}

        impl UnoptimizedFrameConstants {
            pub fn RegisterStackSlotCount(register_count: i32) -> i32 {
                register_count
            }
        }

        pub struct BuiltinContinuationFrameConstants {}

        impl BuiltinContinuationFrameConstants {
            pub fn PaddingSlotCount(register_count: i32) -> i32 {
                let _ = register_count;
                0
            }
        }

        pub struct MaglevFrame {}

        impl MaglevFrame {
            pub fn StackGuardFrameSize(register_input_count: i32) -> i64 {
                let _ = register_input_count;
                panic!("UNREACHABLE");
            }
        }
    }

    pub mod ia32 {
        pub mod frame_constants_ia32 {
            use crate::base::bits;
            use crate::codegen::register::*;
            use crate::execution::frame_constants::*;

            pub const kSystemPointerSize: i32 = 4;

            pub struct EntryFrameConstants {}

            impl EntryFrameConstants {
                pub const kNextExitFrameFPOffset: i32 = -6 * kSystemPointerSize;
                pub const kNextFastCallFrameFPOffset: i32 =
                    EntryFrameConstants::kNextExitFrameFPOffset - kSystemPointerSize;
                pub const kNextFastCallFramePCOffset: i32 =
                    EntryFrameConstants::kNextFastCallFrameFPOffset - kSystemPointerSize;
                pub const kRootRegisterValueOffset: i32 = +2 * kSystemPointerSize;
                pub const kNewTargetArgOffset: i32 = +3 * kSystemPointerSize;
                pub const kFunctionArgOffset: i32 = +4 * kSystemPointerSize;
                pub const kReceiverArgOffset: i32 = +5 * kSystemPointerSize;
                pub const kArgcOffset: i32 = +6 * kSystemPointerSize;
                pub const kArgvOffset: i32 = +7 * kSystemPointerSize;
                pub const kMicrotaskQueueArgOffset: i32 = +3 * kSystemPointerSize;
            }

            pub struct WasmLiftoffSetupFrameConstants {}

            impl WasmLiftoffSetupFrameConstants {
                pub const kNumberOfSavedGpParamRegs: i32 = 3;
                pub const kNumberOfSavedFpParamRegs: i32 = 6;
                pub const kInstanceSpillOffset: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(1);

                pub const kParameterSpillsOffset: [i32; 3] = [
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
                ];

                pub const kWasmInstanceDataOffset: i32 = 2 * kSystemPointerSize;
                pub const kDeclaredFunctionIndexOffset: i32 = 1 * kSystemPointerSize;
                pub const kNativeModuleOffset: i32 = 0;
            }

            pub struct WasmLiftoffFrameConstants {}

            impl WasmLiftoffFrameConstants {
                pub const kFeedbackVectorOffset: i32 = 3 * kSystemPointerSize;
                pub const kInstanceDataOffset: i32 = 2 * kSystemPointerSize;
            }

            pub struct WasmDebugBreakFrameConstants {}

            impl WasmDebugBreakFrameConstants {
                pub const kPushedGpRegs: RegList = RegList {
                    bits_: (1 << eax.code() | 1 << ecx.code() | 1 << edx.code() | 1 << esi.code() | 1 << edi.code()) as u32,
                };

                pub const kPushedFpRegs: DoubleRegList = DoubleRegList {
                    bits_: (1 << xmm1.code() | 1 << xmm2.code() | 1 << xmm3.code() | 1 << xmm4.code() | 1 << xmm5.code() | 1 << xmm6.code()) as u32,
                };

                pub const kNumPushedGpRegisters: i32 = WasmDebugBreakFrameConstants::kPushedGpRegs.Count();
                pub const kNumPushedFpRegisters: i32 = WasmDebugBreakFrameConstants::kPushedFpRegs.Count();

                pub const kLastPushedGpRegisterOffset: i32 =
                    -kFixedFrameSizeFromFp - WasmDebugBreakFrameConstants::kNumPushedGpRegisters * kSystemPointerSize;
                pub const kLastPushedFpRegisterOffset: i32 =
                    WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset
                        - WasmDebugBreakFrameConstants::kNumPushedFpRegisters * kSimd128Size;

                pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
                    if WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & (1 << reg_code) == 0 {
                        panic!("DCHECK failed: register not in kPushedGpRegs");
                    }
                    let lower_regs =
                        WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & ((1u32 << reg_code) - 1);
                    WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset
                        + bits::CountPopulation(lower_regs) * kSystemPointerSize
                }

                pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
                    if WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & (1 << reg_code) == 0 {
                        panic!("DCHECK failed: register not in kPushedFpRegs");
                    }
                    let lower_regs =
                        WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & ((1u32 << reg_code) - 1);
                    WasmDebugBreakFrameConstants::kLastPushedFpRegisterOffset
                        + bits::CountPopulation(lower_regs) * kSimd128Size
                }
            }
        }
    }
}

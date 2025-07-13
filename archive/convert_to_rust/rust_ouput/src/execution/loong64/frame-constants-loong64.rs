// Converted from V8 C++ source files:
// Header: frame-constants-loong64.h
// Implementation: frame-constants-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn count_population(bits: u32) -> i32 {
            bits.count_ones() as i32
        }
    }
}

pub mod codegen {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(pub i32);

    impl Register {
        pub fn code(self) -> i32 {
            self.0
        }
    }

    pub const a0: Register = Register(10);
    pub const a1: Register = Register(11);
    pub const a2: Register = Register(12);
    pub const a3: Register = Register(13);
    pub const a4: Register = Register(14);
    pub const a5: Register = Register(15);
    pub const a6: Register = Register(16);
    pub const a7: Register = Register(17);
    pub const t0: Register = Register(4);
    pub const t1: Register = Register(5);
    pub const t2: Register = Register(6);
    pub const t3: Register = Register(7);
    pub const t4: Register = Register(8);
    pub const t5: Register = Register(9);
    pub const s0: Register = Register(18);
    pub const s1: Register = Register(19);
    pub const s2: Register = Register(20);
    pub const s5: Register = Register(23);
    pub const s7: Register = Register(25);
    pub const f0: Register = Register(32);
    pub const f1: Register = Register(33);
    pub const f2: Register = Register(34);
    pub const f3: Register = Register(35);
    pub const f4: Register = Register(36);
    pub const f5: Register = Register(37);
    pub const f6: Register = Register(38);
    pub const f7: Register = Register(39);
    pub const f8: Register = Register(40);
    pub const f9: Register = Register(41);
    pub const f10: Register = Register(42);
    pub const f11: Register = Register(43);
    pub const f12: Register = Register(44);
    pub const f13: Register = Register(45);
    pub const f14: Register = Register(46);
    pub const f15: Register = Register(47);
    pub const f16: Register = Register(48);
    pub const f17: Register = Register(49);
    pub const f18: Register = Register(50);
    pub const f19: Register = Register(51);
    pub const f20: Register = Register(52);
    pub const f21: Register = Register(53);
    pub const f22: Register = Register(54);
    pub const f23: Register = Register(55);
    pub const f24: Register = Register(56);
    pub const f25: Register = Register(57);
    pub const f26: Register = Register(58);
    pub const f27: Register = Register(59);
    pub const f28: Register = Register(60);
    pub const fp: Register = Register(8);
    pub const cp: Register = Register(28);
}

pub mod execution {
    pub mod frame_constants {
        pub const kSystemPointerSize: i32 = 8;
        pub const kDoubleSize: i32 = 8;
    }

    pub mod loong64 {
        pub mod frame_constants_loong64 {
            use crate::base::bits;
            use crate::codegen::{a0, a1, a2, a3, a4, a5, a6, a7, f0, f1, f10, f11, f12, f13, f14, f15, f16, f17, f18, f19, f2, f20, f21, f22, f23, f24, f25, f26, f27, f28, f3, f4, f5, f6, f7, f8, f9, s0, s1, s2, s5, s7, t0, t1, t2, t3, t4, t5, Register};
            use crate::execution::frame_constants::kSystemPointerSize;

            pub struct AllStatic {}

            pub struct EntryFrameConstants {}

            impl AllStatic {
                 const fn new() -> Self { AllStatic{} }
            }
            impl EntryFrameConstants {
                pub const kNextExitFrameFPOffset: i32 = -3 * kSystemPointerSize;
                pub const kNextFastCallFrameFPOffset: i32 =
                    EntryFrameConstants::kNextExitFrameFPOffset - kSystemPointerSize;
                pub const kNextFastCallFramePCOffset: i32 =
                    EntryFrameConstants::kNextFastCallFrameFPOffset - kSystemPointerSize;
            }

            pub struct TypedFrameConstants {}
            impl TypedFrameConstants {
                 const fn new() -> Self { TypedFrameConstants{} }
            }

            const fn TYPED_FRAME_PUSHED_VALUE_OFFSET(x: i32) -> i32 {
                 -(x+1) * kSystemPointerSize
            }

            pub struct WasmLiftoffSetupFrameConstants {}

            impl WasmLiftoffSetupFrameConstants {
                pub const kNumberOfSavedGpParamRegs: i32 = 6;
                pub const kNumberOfSavedFpParamRegs: i32 = 8;
                pub const kNumberOfSavedAllParamRegs: i32 = 14;

                pub const kInstanceSpillOffset: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(0);

                pub const kParameterSpillsOffset: [i32; 6] = [
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(6),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(5),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
                    TYPED_FRAME_PUSHED_VALUE_OFFSET(1),
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

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct RegList {
                bits_: u64,
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct DoubleRegList {
                bits_: u64,
            }

            impl RegList {
                pub const fn new(bits: u64) -> Self {
                    RegList { bits_: bits }
                }

                pub const fn bits(self) -> u64 {
                    self.bits_
                }

                pub const fn Count(self) -> i32 {
                    self.bits_.count_ones() as i32
                }
            }

            impl DoubleRegList {
                pub const fn new(bits: u64) -> Self {
                    DoubleRegList { bits_: bits }
                }

                pub const fn bits(self) -> u64 {
                    self.bits_
                }

                pub const fn Count(self) -> i32 {
                    self.bits_.count_ones() as i32
                }
            }

            impl WasmDebugBreakFrameConstants {
                pub const kPushedGpRegs: RegList = RegList::new(
                    (1 << a0.code())
                        | (1 << a1.code())
                        | (1 << a2.code())
                        | (1 << a3.code())
                        | (1 << a4.code())
                        | (1 << a5.code())
                        | (1 << a6.code())
                        | (1 << a7.code())
                        | (1 << t0.code())
                        | (1 << t1.code())
                        | (1 << t2.code())
                        | (1 << t3.code())
                        | (1 << t4.code())
                        | (1 << t5.code())
                        | (1 << s0.code())
                        | (1 << s1.code())
                        | (1 << s2.code())
                        | (1 << s5.code())
                        | (1 << s7.code()),
                );

                pub const kPushedFpRegs: DoubleRegList = DoubleRegList::new(
                    (1 << f0.code())
                        | (1 << f1.code())
                        | (1 << f2.code())
                        | (1 << f3.code())
                        | (1 << f4.code())
                        | (1 << f5.code())
                        | (1 << f6.code())
                        | (1 << f7.code())
                        | (1 << f8.code())
                        | (1 << f9.code())
                        | (1 << f10.code())
                        | (1 << f11.code())
                        | (1 << f12.code())
                        | (1 << f13.code())
                        | (1 << f14.code())
                        | (1 << f15.code())
                        | (1 << f16.code())
                        | (1 << f17.code())
                        | (1 << f18.code())
                        | (1 << f19.code())
                        | (1 << f20.code())
                        | (1 << f21.code())
                        | (1 << f22.code())
                        | (1 << f23.code())
                        | (1 << f24.code())
                        | (1 << f25.code())
                        | (1 << f26.code())
                        | (1 << f27.code())
                        | (1 << f28.code()),
                );

                pub const kNumPushedGpRegisters: i32 = WasmDebugBreakFrameConstants::kPushedGpRegs.Count();
                pub const kNumPushedFpRegisters: i32 = WasmDebugBreakFrameConstants::kPushedFpRegs.Count();

                pub const kFixedFrameSizeFromFp: i32 = 16; //Example Size;

                pub const kLastPushedGpRegisterOffset: i32 =
                    -WasmDebugBreakFrameConstants::kFixedFrameSizeFromFp
                    - WasmDebugBreakFrameConstants::kNumPushedGpRegisters * kSystemPointerSize;
                pub const kLastPushedFpRegisterOffset: i32 =
                    WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset
                    - WasmDebugBreakFrameConstants::kNumPushedFpRegisters
                    * crate::execution::frame_constants::kDoubleSize;

                pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
                    if WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & (1 << reg_code) == 0 {
                         panic!("DCHECK_NE failed: register code not in kPushedGpRegs");
                    }

                    let lower_regs =
                        WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & ((1 << reg_code) - 1);
                    WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset
                        + bits::count_population(lower_regs as u32) * kSystemPointerSize
                }

                pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
                    if WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & (1 << reg_code) == 0 {
                        panic!("DCHECK_NE failed: register code not in kPushedFpRegs");
                    }
                    let lower_regs =
                        WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & ((1 << reg_code) - 1);
                    WasmDebugBreakFrameConstants::kLastPushedFpRegisterOffset
                        + bits::count_population(lower_regs as u32) * crate::execution::frame_constants::kDoubleSize
                }
            }
        }
    }
}

pub mod internal {
    use crate::codegen::Register;

    pub struct JavaScriptFrame {}

    impl JavaScriptFrame {
        pub fn fp_register() -> Register {
            crate::codegen::fp
        }
        pub fn context_register() -> Register {
            crate::codegen::cp
        }
        pub fn constant_pool_pointer_register() -> Register {
             unreachable!()
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
             unreachable!()
        }
    }
}

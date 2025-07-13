// Converted from V8 C++ source files:
// Header: frame-constants-ppc.h
// Implementation: frame-constants-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn CountPopulation(lower_regs: u32) -> i32 {
            lower_regs.count_ones() as i32
        }
    }
    pub mod macros {
        #[macro_export]
        macro_rules! USE {
            ($x:expr) => {
                let _ = $x;
            };
        }
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("Unreachable code");
            };
        }
    }
}

pub mod codegen {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Register {
        NoRegister,
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        R8,
        R9,
        R10,
        R11,
        R12,
        R13,
        R14,
        R15,
        R16,
        R17,
        R18,
        R19,
        R20,
        R21,
        R22,
        R23,
        R24,
        R25,
        R26,
        R27,
        R28,
        R29,
        R30,
        R31,
        FP,
        SP,
        LR,
        PC,
        CP,
        // Add more registers as needed
    }
}

pub mod execution {
    pub mod frame_constants {
        pub const kSystemPointerSize: i32 = 8;
        pub const TYPED_FRAME_PUSHED_VALUE_OFFSET: fn(i32) -> i32 =
            |i| -i * kSystemPointerSize;
    }
    pub mod ppc {
        use crate::base;
        use crate::codegen::Register;
        use crate::execution::frame_constants::*;

        pub struct AllStatic {}

        pub struct EntryFrameConstants {}

        impl EntryFrameConstants {
            pub const kNextExitFrameFPOffset: i32 =
                if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                    -4 * kSystemPointerSize
                } else {
                    -3 * kSystemPointerSize
                };

            pub const kNextFastCallFrameFPOffset: i32 =
                Self::kNextExitFrameFPOffset - kSystemPointerSize;
            pub const kNextFastCallFramePCOffset: i32 =
                Self::kNextFastCallFrameFPOffset - kSystemPointerSize;
        }

        pub struct TypedFrameConstants {}

        pub struct WasmLiftoffSetupFrameConstants {}

        impl WasmLiftoffSetupFrameConstants {
            pub const kNumberOfSavedGpParamRegs: i32 = 6;
            pub const kNumberOfSavedFpParamRegs: i32 = 8;

            pub const kInstanceSpillOffset: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(1);

            pub const kParameterSpillsOffset: [i32; 6] = [
                TYPED_FRAME_PUSHED_VALUE_OFFSET(7),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(6),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(5),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
            ];

            pub const kWasmInstanceDataOffset: i32 = 2 * kSystemPointerSize;
            pub const kDeclaredFunctionIndexOffset: i32 = 1 * kSystemPointerSize;
            pub const kNativeModuleOffset: i32 = 0;
        }

        pub struct WasmLiftoffFrameConstants {}

        impl WasmLiftoffFrameConstants {
            pub const kFeedbackVectorOffset: i32 =
                (if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                    4
                } else {
                    3
                }) * kSystemPointerSize;
            pub const kInstanceDataOffset: i32 =
                (if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                    3
                } else {
                    2
                }) * kSystemPointerSize;
        }

        pub struct WasmDebugBreakFrameConstants {}

        #[derive(Debug, Copy, Clone)]
        pub struct RegList {
            bits_: u32,
        }

        impl RegList {
            pub const fn new(bits: u32) -> Self {
                Self { bits_: bits }
            }

            pub fn bits(&self) -> u32 {
                self.bits_
            }

            pub fn Count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct DoubleRegList {
            bits_: u32,
        }

        impl DoubleRegList {
            pub const fn new(bits: u32) -> Self {
                Self { bits_: bits }
            }

            pub fn bits(&self) -> u32 {
                self.bits_
            }
            pub fn Count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct Simd128RegList {
            bits_: u32,
        }

        impl Simd128RegList {
            pub const fn new(bits: u32) -> Self {
                Self { bits_: bits }
            }

            pub fn bits(&self) -> u32 {
                self.bits_
            }
            pub fn Count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }
        impl WasmDebugBreakFrameConstants {
            pub const kPushedGpRegs: RegList = RegList::new(
                (1 << Register::R3 as u32)
                    | (1 << Register::R4 as u32)
                    | (1 << Register::R5 as u32)
                    | (1 << Register::R6 as u32)
                    | (1 << Register::R7 as u32)
                    | (1 << Register::R8 as u32)
                    | (1 << Register::R9 as u32)
                    | (1 << Register::R10 as u32)
                    | (1 << Register::R11 as u32)
                    | (1 << Register::R15 as u32)
                    | (1 << Register::CP as u32),
            );

            pub const kPushedFpRegs: DoubleRegList = DoubleRegList::new(
                (1 << 0 as u32)
                    | (1 << 1 as u32)
                    | (1 << 2 as u32)
                    | (1 << 3 as u32)
                    | (1 << 4 as u32)
                    | (1 << 5 as u32)
                    | (1 << 6 as u32)
                    | (1 << 7 as u32)
                    | (1 << 8 as u32)
                    | (1 << 9 as u32)
                    | (1 << 10 as u32)
                    | (1 << 11 as u32)
                    | (1 << 12 as u32),
            );

            pub const kPushedSimd128Regs: Simd128RegList = Simd128RegList::new(
                (1 << 0 as u32)
                    | (1 << 1 as u32)
                    | (1 << 2 as u32)
                    | (1 << 3 as u32)
                    | (1 << 4 as u32)
                    | (1 << 5 as u32)
                    | (1 << 6 as u32)
                    | (1 << 7 as u32)
                    | (1 << 8 as u32)
                    | (1 << 9 as u32)
                    | (1 << 10 as u32)
                    | (1 << 11 as u32)
                    | (1 << 12 as u32),
            );

            pub const kNumPushedGpRegisters: i32 = Self::kPushedGpRegs.Count();
            pub const kNumPushedFpRegisters: i32 = Self::kPushedFpRegs.Count();

            pub const kLastPushedGpRegisterOffset: i32 =
                -TypedFrameConstants::kFixedFrameSizeFromFp
                    - kSystemPointerSize * Self::kNumPushedGpRegisters;
            pub const kLastPushedFpRegisterOffset: i32 =
                Self::kLastPushedGpRegisterOffset
                    - kDoubleSize * Self::kNumPushedFpRegisters;

            pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
                debug_assert_ne!(0, Self::kPushedGpRegs.bits() & (1 << reg_code));
                let lower_regs = Self::kPushedGpRegs.bits() & ((1u32 << reg_code) - 1);
                Self::kLastPushedGpRegisterOffset
                    + base::bits::CountPopulation(lower_regs) * kSystemPointerSize
            }

            pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
                debug_assert_ne!(0, Self::kPushedFpRegs.bits() & (1 << reg_code));
                let lower_regs = Self::kPushedFpRegs.bits() & ((1u32 << reg_code) - 1);
                Self::kLastPushedFpRegisterOffset
                    + base::bits::CountPopulation(lower_regs) * kSimd128Size
            }
        }

        // Constants
        const V8_EMBEDDED_CONSTANT_POOL_BOOL: bool = true;
        const kDoubleSize: i32 = 8;
        const kSimd128Size: i32 = 16;

        lazy_static::lazy_static! {
            static ref fp: Register = Register::FP;
            static ref cp: Register = Register::CP;
            static ref kConstantPoolRegister: Register = Register::R26;
            static ref r3: Register = Register::R3;
            static ref r4: Register = Register::R4;
            static ref r5: Register = Register::R5;
            static ref r6: Register = Register::R6;
            static ref r7: Register = Register::R7;
            static ref r8: Register = Register::R8;
            static ref r9: Register = Register::R9;
            static ref r10: Register = Register::R10;
            static ref r11: Register = Register::R11;
            static ref r15: Register = Register::R15;
            static ref d0: u32 = 0;
            static ref d1: u32 = 1;
            static ref d2: u32 = 2;
            static ref d3: u32 = 3;
            static ref d4: u32 = 4;
            static ref d5: u32 = 5;
            static ref d6: u32 = 6;
            static ref d7: u32 = 7;
            static ref d8: u32 = 8;
            static ref d9: u32 = 9;
            static ref d10: u32 = 10;
            static ref d11: u32 = 11;
            static ref d12: u32 = 12;
            static ref v0: u32 = 0;
            static ref v1: u32 = 1;
            static ref v2: u32 = 2;
            static ref v3: u32 = 3;
            static ref v4: u32 = 4;
            static ref v5: u32 = 5;
            static ref v6: u32 = 6;
            static ref v7: u32 = 7;
            static ref v8: u32 = 8;
            static ref v9: u32 = 9;
            static ref v10: u32 = 10;
            static ref v11: u32 = 11;
            static ref v12: u32 = 12;
        }
    }
    pub mod ppc64 {
        use crate::codegen::Register;
        use crate::execution::ppc;

        pub struct JavaScriptFrame {}

        impl JavaScriptFrame {
            pub fn fp_register() -> &'static Register {
                &ppc::fp
            }
            pub fn context_register() -> &'static Register {
                &ppc::cp
            }
            pub fn constant_pool_pointer_register() -> &'static Register {
                debug_assert!(V8_EMBEDDED_CONSTANT_POOL_BOOL);
                &ppc::kConstantPoolRegister
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
                panic!("Unreachable");
            }
        }

        const V8_EMBEDDED_CONSTANT_POOL_BOOL: bool = true;
    }
}

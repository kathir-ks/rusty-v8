// Converted from V8 C++ source files:
// Header: frame-constants-s390.h
// Implementation: frame-constants-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn CountPopulation(bits: u32) -> i32 {
            bits.count_ones() as i32
        }
    }
}

pub mod codegen {
    pub struct Register {}
}

pub mod execution {
    pub mod frame_constants {
        pub struct AllStatic {}
        pub struct TypedFrameConstants {}
    }
}

pub mod internal {
    use super::codegen::Register;

    pub struct EntryFrameConstants {}

    impl EntryFrameConstants {
        pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 = -3 * K_SYSTEM_POINTER_SIZE;
        pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 =
            Self::K_NEXT_EXIT_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
        pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 =
            Self::K_NEXT_FAST_CALL_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
        pub const K_ARGV_OFFSET: i32 = 20 * K_SYSTEM_POINTER_SIZE;
    }

    pub struct WasmLiftoffSetupFrameConstants {}

    impl WasmLiftoffSetupFrameConstants {
        pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 3;
        pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 4;
        pub const K_INSTANCE_SPILL_OFFSET: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(1);
        pub const K_PARAMETER_SPILLS_OFFSET: [i32; 3] = [
            TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
        ];
        pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
        pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * K_SYSTEM_POINTER_SIZE;
        pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
    }

    pub struct WasmLiftoffFrameConstants {}

    impl WasmLiftoffFrameConstants {
        pub const K_FEEDBACK_VECTOR_OFFSET: i32 = 3 * K_SYSTEM_POINTER_SIZE;
        pub const K_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct RegList {
        bits_: u64,
    }

    impl RegList {
        pub fn new(bits_: u64) -> Self {
            RegList { bits_ }
        }

        pub fn bits(&self) -> u64 {
            self.bits_
        }

        pub fn Count(&self) -> i32 {
            self.bits_.count_ones() as i32
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct DoubleRegList {
        bits_: u64,
    }

    impl DoubleRegList {
        pub fn new(bits_: u64) -> Self {
            DoubleRegList { bits_ }
        }

        pub fn bits(&self) -> u64 {
            self.bits_
        }

        pub fn Count(&self) -> i32 {
            self.bits_.count_ones() as i32
        }
    }

    pub struct WasmDebugBreakFrameConstants {}

    impl WasmDebugBreakFrameConstants {
        pub const K_PUSHED_GP_REGS: RegList = RegList::new(
            (1 << (R2 as u64))
                | (1 << (R3 as u64))
                | (1 << (R4 as u64))
                | (1 << (R5 as u64))
                | (1 << (R6 as u64))
                | (1 << (R7 as u64))
                | (1 << (R8 as u64))
                | (1 << (CP as u64)),
        );
        pub const K_PUSHED_FP_REGS: DoubleRegList = DoubleRegList::new(
            (1 << (D0 as u64))
                | (1 << (D1 as u64))
                | (1 << (D2 as u64))
                | (1 << (D3 as u64))
                | (1 << (D4 as u64))
                | (1 << (D5 as u64))
                | (1 << (D6 as u64))
                | (1 << (D7 as u64))
                | (1 << (D8 as u64))
                | (1 << (D9 as u64))
                | (1 << (D10 as u64))
                | (1 << (D11 as u64))
                | (1 << (D12 as u64)),
        );
        pub const K_NUM_PUSHED_GP_REGISTERS: i32 = Self::K_PUSHED_GP_REGS.Count();
        pub const K_NUM_PUSHED_FP_REGISTERS: i32 = Self::K_PUSHED_FP_REGS.Count();
        pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
            -TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP
            - K_SYSTEM_POINTER_SIZE * Self::K_NUM_PUSHED_GP_REGISTERS;
        pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - K_SIMD128_SIZE * Self::K_NUM_PUSHED_FP_REGISTERS;

        pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
            if Self::K_PUSHED_GP_REGS.bits() & (1 << reg_code) == 0 {
                panic!("DCHECK failed: reg_code not in K_PUSHED_GP_REGS");
            }

            let lower_regs = Self::K_PUSHED_GP_REGS.bits() & ((1 << reg_code) - 1);
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET
                + base::bits::CountPopulation(lower_regs as u32) * K_SYSTEM_POINTER_SIZE
        }

        pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
            if Self::K_PUSHED_FP_REGS.bits() & (1 << reg_code) == 0 {
                panic!("DCHECK failed: reg_code not in K_PUSHED_FP_REGS");
            }

            let lower_regs = Self::K_PUSHED_FP_REGS.bits() & ((1 << reg_code) - 1);
            Self::K_LAST_PUSHED_FP_REGISTER_OFFSET
                + base::bits::CountPopulation(lower_regs as u32) * K_SIMD128_SIZE
        }
    }

    // dummy impl
    #[derive(Debug, PartialEq, Eq)]
    pub enum RegisterEnum {
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
        FP,
        CP,
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
    }

    use RegisterEnum::*;

    const K_SYSTEM_POINTER_SIZE: i32 = 8;
    const K_SIMD128_SIZE: i32 = 16;

    // dummy impl
    fn TYPED_FRAME_PUSHED_VALUE_OFFSET(i: i32) -> i32 {
        i * K_SYSTEM_POINTER_SIZE
    }

    // dummy impl
    pub struct JavaScriptFrame {}

    impl JavaScriptFrame {
        pub fn fp_register() -> Register {
            Register {}
        }
        pub fn context_register() -> Register {
            Register {}
        }
        pub fn constant_pool_pointer_register() -> Register {
            panic!("UNREACHABLE");
        }
    }

    // dummy impl
    pub struct UnoptimizedFrameConstants {}

    impl UnoptimizedFrameConstants {
        pub fn RegisterStackSlotCount(register_count: i32) -> i32 {
            register_count
        }
    }

    // dummy impl
    pub struct BuiltinContinuationFrameConstants {}

    impl BuiltinContinuationFrameConstants {
        pub fn PaddingSlotCount(register_count: i32) -> i32 {
            let _ = register_count;
            0
        }
    }

    // dummy impl
    pub struct MaglevFrame {}

    impl MaglevFrame {
        pub fn StackGuardFrameSize(register_input_count: i32) -> i64 {
            // Include one extra slot for the single argument into StackGuardWithGap +
            // register input count.
            let standard_frame_constants_k_fixed_frame_size_from_fp = 16; // Replace with actual value
            (standard_frame_constants_k_fixed_frame_size_from_fp
                + (1 + register_input_count) * K_SYSTEM_POINTER_SIZE) as i64
        }
    }
}

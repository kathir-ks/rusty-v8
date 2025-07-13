// Converted from V8 C++ source files:
// Header: frame-constants-riscv.h
// Implementation: frame-constants-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod frame_constants_riscv {
    use crate::base::macros::USE;
    use crate::codegen::register::Register;
    use crate::execution::frame_constants::*;
    use crate::wasm::baseline::liftoff_assembler_defs::*;
    use std::mem;

    pub struct EntryFrameConstants {}

    impl EntryFrameConstants {
        pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 = -3 * mem::size_of::<usize>() as i32;
        pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 =
            Self::K_NEXT_EXIT_FRAME_FP_OFFSET - mem::size_of::<usize>() as i32;
        pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 =
            Self::K_NEXT_FAST_CALL_FRAME_FP_OFFSET - mem::size_of::<usize>() as i32;
    }

    pub struct WasmLiftoffSetupFrameConstants {}

    impl WasmLiftoffSetupFrameConstants {
        pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
        pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;
        pub const K_NUMBER_OF_SAVED_ALL_PARAM_REGS: i32 =
            Self::K_NUMBER_OF_SAVED_GP_PARAM_REGS + Self::K_NUMBER_OF_SAVED_FP_PARAM_REGS;
        pub const K_INSTANCE_SPILL_OFFSET: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(0);
        pub const K_PARAMETER_SPILLS_OFFSET: [i32; 6] = [
            TYPED_FRAME_PUSHED_VALUE_OFFSET(1),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(5),
            TYPED_FRAME_PUSHED_VALUE_OFFSET(6),
        ];
        pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * mem::size_of::<usize>() as i32;
        pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * mem::size_of::<usize>() as i32;
        pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
    }

    pub struct WasmLiftoffFrameConstants {}

    impl WasmLiftoffFrameConstants {
        pub const K_FEEDBACK_VECTOR_OFFSET: i32 = 3 * mem::size_of::<usize>() as i32;
        pub const K_INSTANCE_DATA_OFFSET: i32 = 2 * mem::size_of::<usize>() as i32;
    }

    pub struct WasmDebugBreakFrameConstants {}

    impl WasmDebugBreakFrameConstants {
        pub const K_PUSHED_GP_REGS: RegList = kLiftoffAssemblerGpCacheRegs;
        pub const K_PUSHED_FP_REGS: DoubleRegList = kLiftoffAssemblerFpCacheRegs;
        pub const K_NUM_PUSHED_GP_REGISTERS: i32 = Self::K_PUSHED_GP_REGS.count() as i32;
        pub const K_NUM_PUSHED_FP_REGISTERS: i32 = Self::K_PUSHED_FP_REGS.count() as i32;
        pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
            -K_FIXED_FRAME_SIZE_FROM_FP - Self::K_NUM_PUSHED_GP_REGISTERS * mem::size_of::<usize>() as i32;
        pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - Self::K_NUM_PUSHED_FP_REGISTERS * mem::size_of::<f64>() as i32;

        pub fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
            if (Self::K_PUSHED_GP_REGS.bits() & (1 << reg_code)) == 0 {
                panic!("DCHECK_NE failed");
            }

            let lower_regs = Self::K_PUSHED_GP_REGS.bits() & ((1u32 << reg_code) - 1);
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET
                + lower_regs.count_ones() as i32 * mem::size_of::<usize>() as i32
        }

        pub fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
            if (Self::K_PUSHED_FP_REGS.bits() & (1 << reg_code)) == 0 {
                panic!("DCHECK_NE failed");
            }
            let lower_regs = Self::K_PUSHED_FP_REGS.bits() & ((1u32 << reg_code) - 1);
            Self::K_LAST_PUSHED_FP_REGISTER_OFFSET
                + lower_regs.count_ones() as i32 * mem::size_of::<f64>() as i32
        }
    }

    extern "C" {
        static fp: Register;
        static cp: Register;
    }

    pub struct JavaScriptFrame {}

    impl JavaScriptFrame {
        pub fn fp_register() -> Register {
            unsafe { fp }
        }
        pub fn context_register() -> Register {
            unsafe { cp }
        }
        pub fn constant_pool_pointer_register() -> Register {
            unreachable!()
        }
    }

    pub struct UnoptimizedFrameConstants {}

    impl UnoptimizedFrameConstants {
        pub fn register_stack_slot_count(register_count: i32) -> i32 {
            register_count
        }
    }

    pub struct BuiltinContinuationFrameConstants {}

    impl BuiltinContinuationFrameConstants {
        pub fn padding_slot_count(register_count: i32) -> i32 {
            USE(register_count);
            0
        }
    }

    pub struct MaglevFrame {}

    impl MaglevFrame {
        pub fn stack_guard_frame_size(register_input_count: i32) -> isize {
            USE(register_input_count);
            unreachable!()
        }
    }
}

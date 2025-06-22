// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/bits.h and src/base/macros.h functionalities are assumed to be available in Rust.
// src/codegen/register.h is assumed to be implemented in Rust, with corresponding Register, RegList, and DoubleRegList types.
// src/execution/frame-constants.h is assumed to be implemented in Rust, with corresponding FrameConstants and TypedFrameConstants traits/structs.
// src/wasm/baseline/liftoff-assembler-defs.h is assumed to be implemented in Rust, with corresponding constants.

pub mod riscv_frame_constants {
    use crate::codegen::register::{DoubleRegList, RegList};
    use crate::execution::frame_constants::TypedFrameConstants;

    // Assumed constants from other modules.  Replace with actual definitions.
    const K_SYSTEM_POINTER_SIZE: i32 = 8;
    const K_DOUBLE_SIZE: i32 = 8;
    const K_FIXED_FRAME_SIZE_FROM_FP: i32 = 0; // Replace with actual value.
    const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
    const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * K_SYSTEM_POINTER_SIZE;
    const K_NATIVE_MODULE_OFFSET: i32 = 0;

    // Placeholder for wasm constants. Replace with actual values.
    pub mod wasm {
        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: super::RegList = super::RegList(0); // Replace with actual value
        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: super::DoubleRegList = super::DoubleRegList(0); // Replace with actual value
    }

    /// A struct providing constants for entry frames.
    pub struct EntryFrameConstants;

    impl EntryFrameConstants {
        /// The offset to where JSEntry pushes the current value of
        /// Isolate::c_entry_fp onto the stack.
        pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 = -3 * K_SYSTEM_POINTER_SIZE;
        /// The offsets for storing the FP and PC of fast API calls.
        pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 =
            Self::K_NEXT_EXIT_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
        pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 =
            Self::K_NEXT_FAST_CALL_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
    }

    macro_rules! typed_frame_pushed_value_offset {
        ($index:expr) => {
            -($index + 1) * K_SYSTEM_POINTER_SIZE
        };
    }

    /// A struct providing constants for Wasm Liftoff setup frames.
    pub struct WasmLiftoffSetupFrameConstants;

    impl WasmLiftoffSetupFrameConstants {
        // Number of gp parameters, without the instance.
        // Note that {kNumberOfSavedGpParamRegs} = arraysize(wasm::kGpParamRegisters)
        // - 1, {kNumberOfSavedFpParamRegs} = arraysize(wasm::kFpParamRegisters). Here
        // we use immediate values instead to avoid circular references (introduced by
        // linkage_location.h, issue: v8:14035) and resultant compilation errors.
        pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
        pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;
        pub const K_NUMBER_OF_SAVED_ALL_PARAM_REGS: i32 =
            Self::K_NUMBER_OF_SAVED_GP_PARAM_REGS + Self::K_NUMBER_OF_SAVED_FP_PARAM_REGS;
        pub const K_INSTANCE_SPILL_OFFSET: i32 = typed_frame_pushed_value_offset!(0);
        pub const K_PARAMETER_SPILLS_OFFSET: [i32; 6] = [
            typed_frame_pushed_value_offset!(1),
            typed_frame_pushed_value_offset!(2),
            typed_frame_pushed_value_offset!(3),
            typed_frame_pushed_value_offset!(4),
            typed_frame_pushed_value_offset!(5),
            typed_frame_pushed_value_offset!(6),
        ];

        // SP-relative.
        pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
        pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * K_SYSTEM_POINTER_SIZE;
        pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
    }

    /// A struct providing constants for Wasm Liftoff frames.
    pub struct WasmLiftoffFrameConstants;

    impl WasmLiftoffFrameConstants {
        pub const K_FEEDBACK_VECTOR_OFFSET: i32 = 3 * K_SYSTEM_POINTER_SIZE;
        pub const K_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
    }

    /// A struct providing constants for WasmDebugBreak frames.
    pub struct WasmDebugBreakFrameConstants;

    impl WasmDebugBreakFrameConstants {
        pub const K_PUSHED_GP_REGS: RegList = wasm::K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS;

        pub const K_PUSHED_FP_REGS: DoubleRegList = wasm::K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS;

        pub const K_NUM_PUSHED_GP_REGISTERS: i32 = Self::K_PUSHED_GP_REGS.count();
        pub const K_NUM_PUSHED_FP_REGISTERS: i32 = Self::K_PUSHED_FP_REGS.count();

        pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
            -K_FIXED_FRAME_SIZE_FROM_FP - Self::K_NUM_PUSHED_GP_REGISTERS * K_SYSTEM_POINTER_SIZE;
        pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - Self::K_NUM_PUSHED_FP_REGISTERS * K_DOUBLE_SIZE;

        /// Gets the offset of a pushed GP register.
        pub fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
            debug_assert_ne!(0, Self::K_PUSHED_GP_REGS.bits() & (1 << reg_code));
            let lower_regs: u32 =
                (Self::K_PUSHED_GP_REGS.bits() & ((1u32 << reg_code) - 1)) as u32;
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET +
                lower_regs.count_ones() as i32 * K_SYSTEM_POINTER_SIZE
        }

        /// Gets the offset of a pushed FP register.
        pub fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
            debug_assert_ne!(0, Self::K_PUSHED_FP_REGS.bits() & (1 << reg_code));
            let lower_regs: u32 =
                (Self::K_PUSHED_FP_REGS.bits() & ((1u32 << reg_code) - 1)) as u32;
            Self::K_LAST_PUSHED_FP_REGISTER_OFFSET +
                lower_regs.count_ones() as i32 * K_DOUBLE_SIZE
        }
    }
}
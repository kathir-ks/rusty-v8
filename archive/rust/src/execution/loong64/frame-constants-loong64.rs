// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod frame_constants_loong64 {
    use std::mem;

    //use crate::base::bits; // Assuming a similar implementation exists
    //use crate::codegen::register; // Assuming a similar implementation exists
    use crate::execution::frame_constants::TypedFrameConstants;

    // Mock definitions for types and constants that don't have direct equivalents
    // and would require implementing large parts of V8's codebase.
    type RegList = u64;
    type DoubleRegList = u64;
    const a0: u32 = 0;
    const a1: u32 = 1;
    const a2: u32 = 2;
    const a3: u32 = 3;
    const a4: u32 = 4;
    const a5: u32 = 5;
    const a6: u32 = 6;
    const a7: u32 = 7;
    const t0: u32 = 8;
    const t1: u32 = 9;
    const t2: u32 = 10;
    const t3: u32 = 11;
    const t4: u32 = 12;
    const t5: u32 = 13;
    const s0: u32 = 14;
    const s1: u32 = 15;
    const s2: u32 = 16;
    const s5: u32 = 17;
    const s7: u32 = 18;
    const f0: u32 = 19;
    const f1: u32 = 20;
    const f2: u32 = 21;
    const f3: u32 = 22;
    const f4: u32 = 23;
    const f5: u32 = 24;
    const f6: u32 = 25;
    const f7: u32 = 26;
    const f8: u32 = 27;
    const f9: u32 = 28;
    const f10: u32 = 29;
    const f11: u32 = 30;
    const f12: u32 = 31;
    const f13: u32 = 32;
    const f14: u32 = 33;
    const f15: u32 = 34;
    const f16: u32 = 35;
    const f17: u32 = 36;
    const f18: u32 = 37;
    const f19: u32 = 38;
    const f20: u32 = 39;
    const f21: u32 = 40;
    const f22: u32 = 41;
    const f23: u32 = 42;
    const f24: u32 = 43;
    const f25: u32 = 44;
    const f26: u32 = 45;
    const f27: u32 = 46;
    const f28: u32 = 47;

    const K_SYSTEM_POINTER_SIZE: i32 = 8;
    const K_DOUBLE_SIZE: i32 = 8;
    const K_FIXED_FRAME_SIZE_FROM_FP: i32 = 0; // Placeholder, needs actual value

    macro_rules! typed_frame_pushed_value_offset {
        ($index:expr) => {
            - (1 + $index) * K_SYSTEM_POINTER_SIZE
        };
    }

    /// Constants related to the entry frame.
    pub struct EntryFrameConstants {}

    impl EntryFrameConstants {
        /// Offset to where JSEntry pushes the current value of
        /// Isolate::c_entry_fp onto the stack.
        pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 = -3 * K_SYSTEM_POINTER_SIZE;

        /// Offsets for storing the FP and PC of fast API calls.
        pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 =
            Self::K_NEXT_EXIT_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
        pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 =
            Self::K_NEXT_FAST_CALL_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
    }

    /// Constants related to the Wasm Liftoff setup frame.
    pub struct WasmLiftoffSetupFrameConstants {}

    impl WasmLiftoffSetupFrameConstants {
        /// Number of gp parameters, without the instance.
        pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
        pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;
        pub const K_NUMBER_OF_SAVED_ALL_PARAM_REGS: i32 = 14;

        //On loong64, spilled registers are implicitly sorted backwards by number.
        //We spill:
        //  a0, a2, a3, a4, a5, a6: param1, param2, ..., param6
        //in the following FP-relative order: [a6, a5, a4, a3, a2, a0].
        //The instance slot is in position '0', the first spill slot is at '1'.
        //See wasm::kGpParamRegisters and Builtins::Generate_WasmCompileLazy.
        pub const K_INSTANCE_SPILL_OFFSET: i32 = typed_frame_pushed_value_offset!(0);

        pub const K_PARAMETER_SPILLS_OFFSET: [i32; 6] = [
            typed_frame_pushed_value_offset!(6),
            typed_frame_pushed_value_offset!(5),
            typed_frame_pushed_value_offset!(4),
            typed_frame_pushed_value_offset!(3),
            typed_frame_pushed_value_offset!(2),
            typed_frame_pushed_value_offset!(1),
        ];

        /// SP-relative.
        pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
        pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * K_SYSTEM_POINTER_SIZE;
        pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
    }

    /// Constants related to the Wasm Liftoff frame.
    pub struct WasmLiftoffFrameConstants {}

    impl WasmLiftoffFrameConstants {
        pub const K_FEEDBACK_VECTOR_OFFSET: i32 = 3 * K_SYSTEM_POINTER_SIZE;
        pub const K_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
    }

    /// Constants related to the Wasm Debug Break frame.
    pub struct WasmDebugBreakFrameConstants {}

    impl WasmDebugBreakFrameConstants {
        /// {a0 ... a7, t0 ... t5, s0, s1, s2, s5, s7}
        pub const K_PUSHED_GP_REGS: RegList =
            (1 << a0) | (1 << a1) | (1 << a2) | (1 << a3) | (1 << a4) | (1 << a5) | (1 << a6)
            | (1 << a7) | (1 << t0) | (1 << t1) | (1 << t2) | (1 << t3) | (1 << t4) | (1 << t5)
            | (1 << s0) | (1 << s1) | (1 << s2) | (1 << s5) | (1 << s7);

        /// {f0, f1, f2, ... f27, f28}
        pub const K_PUSHED_FP_REGS: DoubleRegList =
            (1 << f0) | (1 << f1) | (1 << f2) | (1 << f3) | (1 << f4) | (1 << f5) | (1 << f6)
            | (1 << f7) | (1 << f8) | (1 << f9) | (1 << f10) | (1 << f11) | (1 << f12)
            | (1 << f13) | (1 << f14) | (1 << f15) | (1 << f16) | (1 << f17) | (1 << f18)
            | (1 << f19) | (1 << f20) | (1 << f21) | (1 << f22) | (1 << f23) | (1 << f24)
            | (1 << f25) | (1 << f26) | (1 << f27) | (1 << f28);

        pub fn count_set_bits(mut value: u64) -> u32 {
            let mut count = 0;
            while value != 0 {
                value &= value - 1;
                count += 1;
            }
            count
        }

        pub const K_NUM_PUSHED_GP_REGISTERS: u32 = Self::count_set_bits(Self::K_PUSHED_GP_REGS);
        pub const K_NUM_PUSHED_FP_REGISTERS: u32 = Self::count_set_bits(Self::K_PUSHED_FP_REGS);

        pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
            -K_FIXED_FRAME_SIZE_FROM_FP - (Self::K_NUM_PUSHED_GP_REGISTERS as i32) * K_SYSTEM_POINTER_SIZE;

        pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - (Self::K_NUM_PUSHED_FP_REGISTERS as i32) * K_DOUBLE_SIZE;

        /// Returns the offset of the pushed general-purpose register relative to the frame
        /// pointer.
        pub fn get_pushed_gp_register_offset(reg_code: u32) -> i32 {
            if (Self::K_PUSHED_GP_REGS & (1 << reg_code)) == 0 {
                panic!("DCHECK_NE failed: Register code not in pushed GP registers.");
            }
            let lower_regs: u64 = Self::K_PUSHED_GP_REGS & ((1u64 << reg_code) - 1);
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET
                + (Self::count_set_bits(lower_regs) as i32) * K_SYSTEM_POINTER_SIZE
        }

        /// Returns the offset of the pushed floating-point register relative to the frame
        /// pointer.
        pub fn get_pushed_fp_register_offset(reg_code: u32) -> i32 {
            if (Self::K_PUSHED_FP_REGS & (1 << reg_code)) == 0 {
                panic!("DCHECK_NE failed: Register code not in pushed FP registers.");
            }
            let lower_regs: u64 = Self::K_PUSHED_FP_REGS & ((1u64 << reg_code) - 1);
            Self::K_LAST_PUSHED_FP_REGISTER_OFFSET
                + (Self::count_set_bits(lower_regs) as i32) * K_DOUBLE_SIZE
        }
    }
}
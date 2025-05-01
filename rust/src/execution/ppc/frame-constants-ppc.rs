// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/ppc/frame-constants-ppc.h

mod base {
    pub mod bits {
        pub fn count_population(x: u32) -> u32 {
            x.count_ones()
        }
    }

    #[macro_export]
    macro_rules! ARRAY_SIZE {
        ($array:expr) => {
            {
                // Suppress unused variable warning
                let _ = &$array;
                core::mem::size_of_val(&$array) / core::mem::size_of_val(&$array[0])
            }
        };
    }

    #[macro_export]
    macro_rules! UNREACHABLE {
        () => {
            panic!("Unreachable code")
        };
    }

    #[macro_export]
    macro_rules! DCHECK {
        ($condition:expr) => {
            if !$condition {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        };

        ($condition:expr, $($arg:tt)+) => {
            if !$condition {
                panic!("DCHECK failed: {}: {}", stringify!($condition), format_args!($($arg)+));
            }
        };
    }
}

mod codegen {
    pub mod register {
        // Placeholder for register definitions
        pub type Reg = u32;
        pub type DoubleReg = u32;
        pub type Simd128Reg = u32;

        pub const NO_REG: Reg = 0;
    }
}

mod execution {
    pub mod frame_constants {
        pub const ARGUMENT_COUNT_OFFSET: i32 = -1; // Replace with the actual value if available
        pub const FIRST_ARGUMENT_OFFSET: i32 = 0; // Replace with the actual value if available
    }
}

pub mod internal {
    use crate::base::bits;
    use crate::codegen::register::{DoubleReg, Reg, Simd128Reg, NO_REG};

    // These should ideally be conditional compilation flags in Rust
    const V8_EMBEDDED_CONSTANT_POOL_BOOL: bool = true;
    const K_SYSTEM_POINTER_SIZE: i32 = 8; // Assuming 64-bit architecture
    const K_DOUBLE_SIZE: i32 = 8; // Assuming 64-bit doubles
    const K_SIMD128_SIZE: i32 = 16; // Assuming 128-bit SIMD registers

    pub struct AllStatic;

    pub struct EntryFrameConstants;

    impl EntryFrameConstants {
        pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 =
            if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                -4 * K_SYSTEM_POINTER_SIZE
            } else {
                -3 * K_SYSTEM_POINTER_SIZE
            };

        pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 =
            Self::K_NEXT_EXIT_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
        pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 =
            Self::K_NEXT_FAST_CALL_FRAME_FP_OFFSET - K_SYSTEM_POINTER_SIZE;
    }

    pub struct TypedFrameConstants;

    impl TypedFrameConstants {
        pub const K_FIXED_FRAME_SIZE_FROM_FP: i32 = 1;
    }

    // Helper function for TYPED_FRAME_PUSHED_VALUE_OFFSET macro
    const fn typed_frame_pushed_value_offset(index: i32) -> i32 {
        -index * K_SYSTEM_POINTER_SIZE
    }

    pub struct WasmLiftoffSetupFrameConstants;

    impl WasmLiftoffSetupFrameConstants {
        pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
        pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;

        pub const K_INSTANCE_SPILL_OFFSET: i32 =
            typed_frame_pushed_value_offset(1);

        pub const K_PARAMETER_SPILLS_OFFSET: [i32; 6] = [
            typed_frame_pushed_value_offset(7),
            typed_frame_pushed_value_offset(6),
            typed_frame_pushed_value_offset(5),
            typed_frame_pushed_value_offset(4),
            typed_frame_pushed_value_offset(3),
            typed_frame_pushed_value_offset(2),
        ];

        pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
        pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * K_SYSTEM_POINTER_SIZE;
        pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
    }

    pub struct WasmLiftoffFrameConstants;

    impl WasmLiftoffFrameConstants {
        pub const K_FEEDBACK_VECTOR_OFFSET: i32 =
            if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                4 * K_SYSTEM_POINTER_SIZE
            } else {
                3 * K_SYSTEM_POINTER_SIZE
            };
        pub const K_INSTANCE_DATA_OFFSET: i32 =
            if V8_EMBEDDED_CONSTANT_POOL_BOOL {
                3 * K_SYSTEM_POINTER_SIZE
            } else {
                2 * K_SYSTEM_POINTER_SIZE
            };
    }

    pub struct WasmDebugBreakFrameConstants;

    impl WasmDebugBreakFrameConstants {
        pub const K_PUSHED_GP_REGS: RegList = RegList { bits: (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 15) | (1 << 28)}; // Assuming r3-r11, r15, cp register numbers are 3, 4, ..., 11, 15, 28 respectively. Adjust the bitmask if needed.
        pub const K_PUSHED_FP_REGS: DoubleRegList = DoubleRegList { bits: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 12) }; // Assuming d0-d12 register numbers are 0, 1, ..., 12. Adjust the bitmask if needed.
        pub const K_PUSHED_SIMD128_REGS: Simd128RegList = Simd128RegList { bits: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 12) }; // Assuming v0-v12 register numbers are 0, 1, ..., 12. Adjust the bitmask if needed.

        pub const K_NUM_PUSHED_GP_REGISTERS: i32 = Self::K_PUSHED_GP_REGS.count();
        pub const K_NUM_PUSHED_FP_REGISTERS: i32 = Self::K_PUSHED_FP_REGS.count();

        pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
            -TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP -
            K_SYSTEM_POINTER_SIZE * Self::K_NUM_PUSHED_GP_REGISTERS;
        pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - K_DOUBLE_SIZE * Self::K_NUM_PUSHED_FP_REGISTERS;

        pub fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
            crate::base::DCHECK!(Self::K_PUSHED_GP_REGS.bits & (1 << reg_code) != 0);
            let lower_regs =
                Self::K_PUSHED_GP_REGS.bits & ((1u32 << reg_code) - 1);
            Self::K_LAST_PUSHED_GP_REGISTER_OFFSET +
                bits::count_population(lower_regs) as i32 * K_SYSTEM_POINTER_SIZE
        }

        pub fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
            crate::base::DCHECK!(Self::K_PUSHED_FP_REGS.bits & (1 << reg_code) != 0);
            let lower_regs =
                Self::K_PUSHED_FP_REGS.bits & ((1u32 << reg_code) - 1);
            Self::K_LAST_PUSHED_FP_REGISTER_OFFSET +
                bits::count_population(lower_regs) as i32 * K_SIMD128_SIZE
        }
    }

    pub struct RegList {
        bits: u32,
    }

    impl RegList {
        const fn count(&self) -> i32 {
            self.bits.count_ones() as i32
        }
        const fn bits(&self) -> u32 {
          self.bits
        }
    }

    pub struct DoubleRegList {
      bits: u32,
    }

    impl DoubleRegList {
        const fn count(&self) -> i32 {
            self.bits.count_ones() as i32
        }
        const fn bits(&self) -> u32 {
          self.bits
        }
    }

    pub struct Simd128RegList {
      bits: u32,
    }

    impl Simd128RegList {
        const fn count(&self) -> i32 {
            self.bits.count_ones() as i32
        }
        const fn bits(&self) -> u32 {
          self.bits
        }
    }
}
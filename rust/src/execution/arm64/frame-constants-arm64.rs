// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/arm64/frame-constants-arm64.h

pub mod arm64 {
    pub mod frame_constants {
        use crate::base::bits::count_population;
        use crate::codegen::register::*;
        use crate::codegen::reglist::*;
        use crate::common::globals::*;
        use crate::execution::frame_constants::*;

        /// Constants related to the layout of an EntryFrame.
        ///
        /// The layout of an EntryFrame is as follows:
        ///
        ///         BOTTOM OF THE STACK   HIGHEST ADDRESS
        ///  slot      Entry frame
        ///       +---------------------+-----------------------
        /// -19   | saved register d15  |
        /// ...   |        ...          |
        /// -12   | saved register d8   |
        ///       |- - - - - - - - - - -|
        /// -11   | saved register x28  |
        /// ...   |        ...          |
        ///  -2   | saved register x19  |
        ///       |- - - - - - - - - - -|
        ///  -1   |   saved lr (x30)    |
        ///       |- - - - - - - - - - -|
        ///   0   |   saved fp (x29)    |  <-- frame ptr
        ///       |- - - - - - - - - - -|
        ///   1   | stack frame marker  |
        ///       |      (ENTRY)        |
        ///       |- - - - - - - - - - -|
        ///   2   | stack frame marker  |
        ///       |        (0)          |
        ///       |- - - - - - - - - - -|
        ///   3   |     C entry FP      |
        ///       |- - - - - - - - - - -|
        ///   4   |   JS entry frame    |
        ///       |       marker        |
        ///       |- - - - - - - - - - -|
        ///   5   |  fast api call fp   |
        ///       |- - - - - - - - - - -|
        ///   6   |  fast api call pc   |  <-- stack ptr
        ///  -----+---------------------+-----------------------
        ///          TOP OF THE STACK     LOWEST ADDRESS
        pub struct EntryFrameConstants;

        impl EntryFrameConstants {
            /// Offset to where JSEntry pushes the current value of
            /// Isolate::c_entry_fp onto the stack.
            pub const K_NEXT_EXIT_FRAME_FP_OFFSET: i32 = -3 * kSystemPointerSize;
            /// Offsets for storing the FP and PC of fast API calls.
            pub const K_NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 = -5 * kSystemPointerSize;
            pub const K_NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 = -6 * kSystemPointerSize;

            pub const K_FIXED_FRAME_SIZE: i32 = 6 * kSystemPointerSize;

            /// Bytes pushed before Fp/Lr pair.
            pub const K_CALLEE_SAVED_REGISTER_BYTES_PUSHED_BEFORE_FP_LR_PAIR: i32 =
                18 * kSystemPointerSize;
            pub const K_CALLEE_SAVED_REGISTER_BYTES_PUSHED_AFTER_FP_LR_PAIR: i32 = 0;
            pub const K_OFFSET_TO_CALLEE_SAVED_REGISTERS: i32 = 0;

            /// Offsets refer to the immediate caller (a native frame), not to the
            /// previous JS exit frame like kCallerFPOffset above.
            pub const K_DIRECT_CALLER_FP_OFFSET: i32 =
                Self::K_CALLEE_SAVED_REGISTER_BYTES_PUSHED_AFTER_FP_LR_PAIR
                    + Self::K_OFFSET_TO_CALLEE_SAVED_REGISTERS;
            pub const K_DIRECT_CALLER_PC_OFFSET: i32 =
                Self::K_DIRECT_CALLER_FP_OFFSET + 1 * kSystemPointerSize;
            pub const K_DIRECT_CALLER_SP_OFFSET: i32 =
                Self::K_DIRECT_CALLER_PC_OFFSET + 1 * kSystemPointerSize
                    + Self::K_CALLEE_SAVED_REGISTER_BYTES_PUSHED_BEFORE_FP_LR_PAIR;
        }

        /// Constants for the Wasm Liftoff setup frame.
        pub struct WasmLiftoffSetupFrameConstants;

        impl WasmLiftoffSetupFrameConstants {
            /// Number of gp parameters, without the instance.
            pub const K_NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
            pub const K_NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;

            /// On arm, spilled registers are implicitly sorted backwards by number.
            /// We spill:
            ///   x0, x2, x3, x4, x5, x6: param1, param2, ..., param6
            /// in the following FP-relative order: [x6, x5, x4, x3, x2, x0].
            /// The instance slot is in position '0', the first spill slot is at '1'.
            pub const K_INSTANCE_SPILL_OFFSET: i32 =
                TypedFrameConstants::typed_frame_pushed_value_offset(0);

            pub const K_PARAMETER_SPILLS_OFFSET: [i32; 6] = [
                TypedFrameConstants::typed_frame_pushed_value_offset(6),
                TypedFrameConstants::typed_frame_pushed_value_offset(5),
                TypedFrameConstants::typed_frame_pushed_value_offset(4),
                TypedFrameConstants::typed_frame_pushed_value_offset(3),
                TypedFrameConstants::typed_frame_pushed_value_offset(2),
                TypedFrameConstants::typed_frame_pushed_value_offset(1),
            ];

            /// SP-relative.
            pub const K_WASM_INSTANCE_DATA_OFFSET: i32 = 2 * kSystemPointerSize;
            pub const K_DECLARED_FUNCTION_INDEX_OFFSET: i32 = 1 * kSystemPointerSize;
            pub const K_NATIVE_MODULE_OFFSET: i32 = 0;
        }

        /// Constants for the Wasm Liftoff frame.
        pub struct WasmLiftoffFrameConstants;

        impl WasmLiftoffFrameConstants {
            pub const K_FEEDBACK_VECTOR_OFFSET: i32 = 3 * kSystemPointerSize;
            pub const K_INSTANCE_DATA_OFFSET: i32 = 2 * kSystemPointerSize;
        }

        /// Frame constructed by the {WasmDebugBreak} builtin.
        /// After pushing the frame type marker, the builtin pushes all Liftoff cache
        /// registers (see liftoff-assembler-defs.h).
        pub struct WasmDebugBreakFrameConstants;

        impl WasmDebugBreakFrameConstants {
            /// x16: ip0, x17: ip1, x18: platform register, x26: root, x28: base, x29: fp,
            /// x30: lr, x31: xzr.
            pub const K_PUSHED_GP_REGS: RegList = RegList {
                bits_: (1 << Register::X0 as u32)
                    | (1 << Register::X1 as u32)
                    | (1 << Register::X2 as u32)
                    | (1 << Register::X3 as u32)
                    | (1 << Register::X4 as u32)
                    | (1 << Register::X5 as u32)
                    | (1 << Register::X6 as u32)
                    | (1 << Register::X7 as u32)
                    | (1 << Register::X8 as u32)
                    | (1 << Register::X9 as u32)
                    | (1 << Register::X10 as u32)
                    | (1 << Register::X11 as u32)
                    | (1 << Register::X12 as u32)
                    | (1 << Register::X13 as u32)
                    | (1 << Register::X14 as u32)
                    | (1 << Register::X15 as u32)
                    | (1 << Register::X19 as u32)
                    | (1 << Register::X20 as u32)
                    | (1 << Register::X21 as u32)
                    | (1 << Register::X22 as u32)
                    | (1 << Register::X23 as u32)
                    | (1 << Register::X24 as u32)
                    | (1 << Register::X25 as u32)
                    | (1 << Register::X27 as u32),
            };

            /// We push FpRegs as 128-bit SIMD registers, so 16-byte frame alignment
            /// is guaranteed regardless of register count.
            pub const K_PUSHED_FP_REGS: DoubleRegList = DoubleRegList {
                bits_: (1 << DoubleRegister::D0 as u32)
                    | (1 << DoubleRegister::D1 as u32)
                    | (1 << DoubleRegister::D2 as u32)
                    | (1 << DoubleRegister::D3 as u32)
                    | (1 << DoubleRegister::D4 as u32)
                    | (1 << DoubleRegister::D5 as u32)
                    | (1 << DoubleRegister::D6 as u32)
                    | (1 << DoubleRegister::D7 as u32)
                    | (1 << DoubleRegister::D8 as u32)
                    | (1 << DoubleRegister::D9 as u32)
                    | (1 << DoubleRegister::D10 as u32)
                    | (1 << DoubleRegister::D11 as u32)
                    | (1 << DoubleRegister::D12 as u32)
                    | (1 << DoubleRegister::D13 as u32)
                    | (1 << DoubleRegister::D14 as u32)
                    | (1 << DoubleRegister::D16 as u32)
                    | (1 << DoubleRegister::D17 as u32)
                    | (1 << DoubleRegister::D18 as u32)
                    | (1 << DoubleRegister::D19 as u32)
                    | (1 << DoubleRegister::D20 as u32)
                    | (1 << DoubleRegister::D21 as u32)
                    | (1 << DoubleRegister::D22 as u32)
                    | (1 << DoubleRegister::D23 as u32)
                    | (1 << DoubleRegister::D24 as u32)
                    | (1 << DoubleRegister::D25 as u32)
                    | (1 << DoubleRegister::D26 as u32)
                    | (1 << DoubleRegister::D27 as u32),
            };

            pub const K_NUM_PUSHED_GP_REGISTERS: i32 = Self::K_PUSHED_GP_REGS.count();
            // TODO: Add static_assert equivalent to Rust?
            // static_assert(kNumPushedGpRegisters % 2 == 0,
            //                "stack frames need to be 16-byte aligned");

            pub const K_NUM_PUSHED_FP_REGISTERS: i32 = Self::K_PUSHED_FP_REGS.count();

            pub const K_LAST_PUSHED_GP_REGISTER_OFFSET: i32 =
                // Header is padded to 16 byte (see {MacroAssembler::EnterFrame}).
                -round_up(TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP, 16)
                - kSystemPointerSize * Self::K_NUM_PUSHED_GP_REGISTERS;
            pub const K_LAST_PUSHED_FP_REGISTER_OFFSET: i32 =
                Self::K_LAST_PUSHED_GP_REGISTER_OFFSET - kSimd128Size * Self::K_NUM_PUSHED_FP_REGISTERS;

            /// Offsets are fp-relative.
            pub fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
                debug_assert_ne!(0, Self::K_PUSHED_GP_REGS.bits() & (1 << reg_code));
                let lower_regs: u32 = Self::K_PUSHED_GP_REGS.bits() & ((1u32 << reg_code) - 1);
                Self::K_LAST_PUSHED_GP_REGISTER_OFFSET
                    + count_population(lower_regs as u64) as i32 * kSystemPointerSize
            }

            pub fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
                debug_assert_ne!(0, Self::K_PUSHED_FP_REGS.bits() & (1 << reg_code));
                let lower_regs: u32 = Self::K_PUSHED_FP_REGS.bits() & ((1u32 << reg_code) - 1);
                Self::K_LAST_PUSHED_FP_REGISTER_OFFSET
                    + count_population(lower_regs as u64) as i32 * kSimd128Size
            }
        }
    }
}

// TODO: Implement these functions from src/base
pub mod base {
    pub mod bits {
        #[inline]
        pub fn count_population(x: u64) -> u32 {
            x.count_ones()
        }
    }
}

// TODO: Dummy implementations for dependencies to allow compilation
pub mod codegen {
    pub mod register {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Register {
            X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11,
            X12, X13, X14, X15, X16, X17, X18, X19, X20, X21,
            X22, X23, X24, X25, X26, X27, X28, X29, X30, X31
        }

        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum DoubleRegister {
            D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
            D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31
        }

    }

    pub mod reglist {
        #[derive(Debug, Copy, Clone)]
        pub struct RegList {
            pub bits_: u32,
        }

        impl RegList {
            pub fn bits(&self) -> u32 {
                self.bits_
            }

            pub fn count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct DoubleRegList {
            pub bits_: u32,
        }

        impl DoubleRegList {
            pub fn bits(&self) -> u32 {
                self.bits_
            }

            pub fn count(&self) -> i32 {
                self.bits_.count_ones() as i32
            }
        }
    }
}

pub mod common {
    pub mod globals {
        pub const kSystemPointerSize: i32 = 8;
        pub const kSimd128Size: i32 = 16;
    }
}

pub mod execution {
    pub mod frame_constants {
        pub struct TypedFrameConstants;

        impl TypedFrameConstants {
            pub const K_FIXED_FRAME_SIZE_FROM_FP: i32 = 16;

            pub const fn typed_frame_pushed_value_offset(index: i32) -> i32 {
                TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP + (index * 8)
            }
        }
    }
}

#[macro_export]
macro_rules! debug_assert_ne {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("debug_assert_ne!({}, {})", $left, $right);
        }
    };
}

pub fn round_up(value: i32, alignment: i32) -> i32 {
    if alignment == 0 {
        return value;
    }
    let remainder = value % alignment;
    if remainder == 0 {
        return value;
    }
    value + alignment - remainder
}
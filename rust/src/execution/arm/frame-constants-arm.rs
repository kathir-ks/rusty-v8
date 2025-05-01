// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/arm/frame-constants-arm.h

#![allow(dead_code)]

mod base {
    pub mod bits {
        pub fn count_population(x: u32) -> u32 {
            x.count_ones()
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! ARRAY_SIZE {
            ($arr:expr) => {
                {
                    #[allow(unused_imports)]
                    use std::mem;
                    mem::size_of_val(&$arr) / mem::size_of_val(&$arr[0])
                }
            };
        }
    }
}

mod codegen {
    pub mod register {
        // Placeholder for Register related definitions
        // Consider using enums or structs to represent Registers in Rust
        pub type RegList = u32;
        pub type DoubleRegList = u32;
    }
}

mod execution {
    pub mod frame_constants {
        pub const kSystemPointerSize: i32 = 4;
        pub const kNumCalleeSaved: i32 = 8; // Includes r11 (fp)
        pub const kDoubleSize: i32 = 8;
        pub const kNumDoubleCalleeSaved: i32 = 8;

        pub const TYPED_FRAME_PUSHED_VALUE_OFFSET_MULTIPLIER: i32 = 1;

        pub fn TYPED_FRAME_PUSHED_VALUE_OFFSET(n: i32) -> i32 {
          -((n + 2) * kSystemPointerSize)
        }

        pub trait TypedFrameConstantsTrait {
            const kFixedFrameSizeFromFp: i32;
        }

        pub struct TypedFrameConstants;

        impl TypedFrameConstantsTrait for TypedFrameConstants {
            const kFixedFrameSizeFromFp: i32 = 2 * kSystemPointerSize; // Type + Padding/Saved PC
        }

    }

    pub mod arm {
        use super::super::base;
        use super::super::codegen::register::*;
        use super::frame_constants::*;

        // The layout of an EntryFrame is as follows:
        //            TOP OF THE STACK     LOWEST ADDRESS
        //         +---------------------+-----------------------
        //   -6    |  outermost marker   |
        //         |- - - - - - - - - - -|
        //   -5    |   fast api call pc  |
        //         |- - - - - - - - - - -|
        //   -4    |   fast api call fp  |
        //         |- - - - - - - - - - -|
        //   -3    |      centry fp      |
        //         |- - - - - - - - - - -|
        //   -2    | stack frame marker  |
        //         |- - - - - - - - - - -|
        //   -1    | stack frame marker  |
        //         |- - - - - - - - - - -|
        //   0     |   saved fp (r11)    |  <-- frame ptr
        //         |- - - - - - - - - - -|
        //   1     |   saved lr (r14)    |
        //         |- - - - - - - - - - -|
        //  2..3   | saved register d8   |
        //  ...    |        ...          |
        //  16..17 | saved register d15  |
        //         |- - - - - - - - - - -|
        //  18     | saved register r4   |
        //  ...    |        ...          |
        //  24     | saved register r10  |
        //    -----+---------------------+-----------------------
        //           BOTTOM OF THE STACK   HIGHEST ADDRESS
        pub struct EntryFrameConstants;

        impl EntryFrameConstants {
            // This is the offset to where JSEntry pushes the current value of
            // Isolate::c_entry_fp onto the stack.
            pub const kNextExitFrameFPOffset: i32 = -3 * kSystemPointerSize;

            pub const kNextFastCallFrameFPOffset: i32 =
                Self::kNextExitFrameFPOffset - kSystemPointerSize;
            pub const kNextFastCallFramePCOffset: i32 =
                Self::kNextFastCallFrameFPOffset - kSystemPointerSize;

            // Stack offsets for arguments passed to JSEntry.
            pub const kArgcOffset: i32 = +0 * kSystemPointerSize;
            pub const kArgvOffset: i32 = +1 * kSystemPointerSize;

            // These offsets refer to the immediate caller (i.e a native frame).
            pub const kDirectCallerFPOffset: i32 = 0;
            pub const kDirectCallerPCOffset: i32 =
                Self::kDirectCallerFPOffset + 1 * kSystemPointerSize;
            pub const kDirectCallerGeneralRegistersOffset: i32 =
                Self::kDirectCallerPCOffset +
                /* saved caller PC */
                kSystemPointerSize +
                /* d8...d15 */
                kNumDoubleCalleeSaved * kDoubleSize;
            pub const kDirectCallerSPOffset: i32 =
                Self::kDirectCallerGeneralRegistersOffset +
                /* r4...r10 (i.e. callee saved without fp) */
                (kNumCalleeSaved - 1) * kSystemPointerSize;
        }

        pub struct WasmLiftoffSetupFrameConstants;

        impl WasmLiftoffSetupFrameConstants {
            // Number of gp parameters, without the instance.
            pub const kNumberOfSavedGpParamRegs: i32 = 3;
            pub const kNumberOfSavedFpParamRegs: i32 = 8;

            // On arm, spilled registers are implicitly sorted backwards by number.
            // We spill:
            //   r3: param0 = instance
            //   r0, r2, r6: param1, param2, param3
            //   lr (== r14): internal usage of the caller
            // in the following FP-relative order: [lr, r6, r3, r2, r0].
            pub const kInstanceSpillOffset: i32 =
                TYPED_FRAME_PUSHED_VALUE_OFFSET(2);

            pub const kParameterSpillsOffset: [i32; 3] = [
                TYPED_FRAME_PUSHED_VALUE_OFFSET(4), TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
                TYPED_FRAME_PUSHED_VALUE_OFFSET(1),
            ];

            // SP-relative.
            pub const kWasmInstanceDataOffset: i32 = 2 * kSystemPointerSize;
            pub const kDeclaredFunctionIndexOffset: i32 = 1 * kSystemPointerSize;
            pub const kNativeModuleOffset: i32 = 0;
        }

        pub struct WasmLiftoffFrameConstants;

        impl WasmLiftoffFrameConstants {
            pub const kFeedbackVectorOffset: i32 = 3 * kSystemPointerSize;
            pub const kInstanceDataOffset: i32 = 2 * kSystemPointerSize;
        }

        // Frame constructed by the {WasmDebugBreak} builtin.
        // After pushing the frame type marker, the builtin pushes all Liftoff cache
        // registers (see liftoff-assembler-defs.h).
        pub struct WasmDebugBreakFrameConstants;

        impl WasmDebugBreakFrameConstants {
            // r10: root, r11: fp, r12: ip, r13: sp, r14: lr, r15: pc.
            pub const kPushedGpRegs: RegList = 0b1111111111; // {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9};

            // d13: zero, d14-d15: scratch
            pub const kPushedFpRegs: DoubleRegList = 0b1111111111111; // {d0, d1, d2, d3,  d4,  d5, d6, d7, d8, d9, d10, d11, d12};

            pub const kNumPushedGpRegisters: i32 = Self::kPushedGpRegs.count_ones() as i32;
            pub const kNumPushedFpRegisters: i32 = Self::kPushedFpRegs.count_ones() as i32;

            pub const kLastPushedGpRegisterOffset: i32 =
                -TypedFrameConstants::kFixedFrameSizeFromFp -
                kSystemPointerSize * Self::kNumPushedGpRegisters;
            pub const kLastPushedFpRegisterOffset: i32 =
                Self::kLastPushedGpRegisterOffset - kDoubleSize * Self::kNumPushedFpRegisters;

            // Offsets are fp-relative.
            pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
                assert_ne!(0, Self::kPushedGpRegs & (1 << reg_code));
                let lower_regs: u32 =
                    Self::kPushedGpRegs & (((1_u32) << reg_code) - 1);
                Self::kLastPushedGpRegisterOffset +
                    base::bits::count_population(lower_regs) as i32 * kSystemPointerSize
            }

            pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
                assert_ne!(0, Self::kPushedFpRegs & (1 << reg_code));
                let lower_regs: u32 =
                    Self::kPushedFpRegs & (((1_u32) << reg_code) - 1);
                Self::kLastPushedFpRegisterOffset +
                    base::bits::count_population(lower_regs) as i32 * kDoubleSize
            }
        }
    }
}
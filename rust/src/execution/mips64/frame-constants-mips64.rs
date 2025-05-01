// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/mips64/frame-constants-mips64.h

#![allow(dead_code)]

mod base {
    pub mod bits {
        pub fn count_population(x: u32) -> u32 {
            x.count_ones()
        }
    }

    pub mod macros {
        // Mock implementation for AllStatic, as it doesn't directly translate
        // to Rust.  We can use an empty struct for similar effect.
        // In Rust, the properties would be constants associated with the struct.
    }
}

mod codegen {
    pub mod register {
        // Mock register definitions
        pub type RegList = u64;
        pub type DoubleRegList = u64;

        pub const v0: u32 = 1;
        pub const v1: u32 = 2;
        pub const a0: u32 = 3;
        pub const a1: u32 = 4;
        pub const a2: u32 = 5;
        pub const a3: u32 = 6;
        pub const a4: u32 = 7;
        pub const a5: u32 = 8;
        pub const a6: u32 = 9;
        pub const a7: u32 = 10;
        pub const t0: u32 = 11;
        pub const t1: u32 = 12;
        pub const t2: u32 = 13;
        pub const s7: u32 = 14;

        pub const f0: u32 = 15;
        pub const f2: u32 = 16;
        pub const f4: u32 = 17;
        pub const f6: u32 = 18;
        pub const f8: u32 = 19;
        pub const f10: u32 = 20;
        pub const f12: u32 = 21;
        pub const f14: u32 = 22;
        pub const f16: u32 = 23;
        pub const f18: u32 = 24;
        pub const f20: u32 = 25;
        pub const f22: u32 = 26;
        pub const f24: u32 = 27;
        pub const f26: u32 = 28;
    }
}

mod execution {
    pub mod frame_constants {
        pub const kSystemPointerSize: i32 = 8;
        pub const kDoubleSize: i32 = 8;
        pub const kFixedFrameSizeFromFp: i32 = 0; // Placeholder value

        pub trait TypedFrameConstants {
            // This is a trait, so no default implementation.
        }

        pub const fn TYPED_FRAME_PUSHED_VALUE_OFFSET(index: i32) -> i32 {
            -(index + 1) * kSystemPointerSize
        }
    }
}

pub mod internal {
    use crate::{
        base::bits,
        codegen::register::{DoubleRegList, RegList, a2, a3, a4, a5, a6, a7, f0, f10, f12, f14, f16, f18, f2, f20, f22, f24, f26, f4, f6, f8, s7, t0, t1, t2, v0, v1},
        execution::frame_constants::{
            kDoubleSize, kFixedFrameSizeFromFp, kSystemPointerSize, TypedFrameConstants, TYPED_FRAME_PUSHED_VALUE_OFFSET,
        },
    };

    pub struct EntryFrameConstants {}

    impl EntryFrameConstants {
        pub const kNextExitFrameFPOffset: i32 = -3 * kSystemPointerSize;
        pub const kNextFastCallFrameFPOffset: i32 =
            EntryFrameConstants::kNextExitFrameFPOffset - kSystemPointerSize;
        pub const kNextFastCallFramePCOffset: i32 =
            EntryFrameConstants::kNextFastCallFrameFPOffset - kSystemPointerSize;
    }

    pub struct WasmLiftoffSetupFrameConstants {}

    impl WasmLiftoffSetupFrameConstants {
        pub const kNumberOfSavedGpParamRegs: i32 = 6;
        pub const kNumberOfSavedFpParamRegs: i32 = 7;
        pub const kNumberOfSavedAllParamRegs: i32 = 13;

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

    impl TypedFrameConstants for WasmLiftoffSetupFrameConstants {}

    pub struct WasmLiftoffFrameConstants {}

    impl WasmLiftoffFrameConstants {
        pub const kFeedbackVectorOffset: i32 = 3 * kSystemPointerSize;
        pub const kInstanceDataOffset: i32 = 2 * kSystemPointerSize;
    }

    impl TypedFrameConstants for WasmLiftoffFrameConstants {}

    pub struct WasmDebugBreakFrameConstants {}

    impl WasmDebugBreakFrameConstants {
        pub const kPushedGpRegs: RegList =
            (1 << v0) | (1 << v1) | (1 << a0) | (1 << a1) | (1 << a2) | (1 << a3) | (1 << a4)
            | (1 << a5) | (1 << a6) | (1 << a7) | (1 << t0) | (1 << t1) | (1 << t2) | (1 << s7);

        pub const kPushedFpRegs: DoubleRegList =
            (1 << f0) | (1 << f2) | (1 << f4) | (1 << f6) | (1 << f8) | (1 << f10) | (1 << f12)
            | (1 << f14) | (1 << f16) | (1 << f18) | (1 << f20) | (1 << f22) | (1 << f24) | (1 << f26);

        pub const kNumPushedGpRegisters: i32 =
            WasmDebugBreakFrameConstants::kPushedGpRegs.count_ones() as i32;
        pub const kNumPushedFpRegisters: i32 =
            WasmDebugBreakFrameConstants::kPushedFpRegs.count_ones() as i32;

        pub const kLastPushedGpRegisterOffset: i32 =
            -kFixedFrameSizeFromFp - WasmDebugBreakFrameConstants::kNumPushedGpRegisters * kSystemPointerSize;
        pub const kLastPushedFpRegisterOffset: i32 =
            WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset - WasmDebugBreakFrameConstants::kNumPushedFpRegisters * kDoubleSize;

        pub fn get_pushed_gp_register_offset(reg_code: u32) -> i32 {
            debug_assert_ne!(0, WasmDebugBreakFrameConstants::kPushedGpRegs & (1 << reg_code));
            let lower_regs: u32 =
                WasmDebugBreakFrameConstants::kPushedGpRegs & ((1u32 << reg_code) - 1);
            WasmDebugBreakFrameConstants::kLastPushedGpRegisterOffset +
                bits::count_population(lower_regs) as i32 * kSystemPointerSize
        }

        pub fn get_pushed_fp_register_offset(reg_code: u32) -> i32 {
            debug_assert_ne!(0, WasmDebugBreakFrameConstants::kPushedFpRegs & (1 << reg_code));
            let lower_regs: u32 =
                WasmDebugBreakFrameConstants::kPushedFpRegs & ((1u32 << reg_code) - 1);
            WasmDebugBreakFrameConstants::kLastPushedFpRegisterOffset +
                bits::count_population(lower_regs) as i32 * kDoubleSize
        }
    }

    impl TypedFrameConstants for WasmDebugBreakFrameConstants {}
}
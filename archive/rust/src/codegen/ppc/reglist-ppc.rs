// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_arch;
mod reglist_base;

use register_arch::{DoubleRegister, Register, Simd128Register};
use reglist_base::RegListBase;

pub type RegList = RegListBase<Register>;
pub type DoubleRegList = RegListBase<DoubleRegister>;
pub type Simd128RegList = RegListBase<Simd128Register>;

// Implement trivially copyable trait (manually, because derive doesn't work on type aliases)
unsafe impl Copy for RegList {}
impl Clone for RegList {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl Copy for DoubleRegList {}
impl Clone for DoubleRegList {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl Copy for Simd128RegList {}
impl Clone for Simd128RegList {
    fn clone(&self) -> Self {
        *self
    }
}

pub mod internal {
    use super::*;
    use register_arch::{r10, r11, r14, r15, r16, r17, r18, r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r3, r30, r4, r5, r6, r7, r8, r9, fp, d0, d1, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19, d2, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d3, d30, d31, d4, d5, d6, d7, d8, d9, v0, v1, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v2, v3, v4, v5, v6, v7, v8, v9};

    pub const K_JS_CALLER_SAVED: RegList = RegList {
        registers: [
            Some(r3),   // a1
            Some(r4),   // a2
            Some(r5),   // a3
            Some(r6),   // a4
            Some(r7),   // a5
            Some(r8),   // a6
            Some(r9),   // a7
            Some(r10),  // a8
            Some(r11),
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        count: 9,
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 9;

    // Return the code of the n-th caller-saved register available to JavaScript
    // e.g. JSCallerSavedReg(0) returns r0.code() == 0
    pub fn js_caller_saved_code(n: usize) -> i32 {
        match K_JS_CALLER_SAVED.registers[n] {
            Some(reg) => reg.code(),
            None => -1, // Or some other error value
        }
    }

    pub const K_CALLEE_SAVED: RegList = RegList {
        registers: [
            Some(r14),
            Some(r15),
            Some(r16),
            Some(r17),
            Some(r18),
            Some(r19),
            Some(r20),
            Some(r21),
            Some(r22),
            Some(r23),
            Some(r24),
            Some(r25),
            Some(r26),
            Some(r27),
            Some(r28),
            Some(r29),
            Some(r30),
            Some(fp),
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        count: 18,
    };

    pub const K_NUM_CALLEE_SAVED: usize = 18;

    pub const K_CALLER_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: [
            Some(d0),
            Some(d1),
            Some(d2),
            Some(d3),
            Some(d4),
            Some(d5),
            Some(d6),
            Some(d7),
            Some(d8),
            Some(d9),
            Some(d10),
            Some(d11),
            Some(d12),
            Some(d13),
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        count: 14,
    };

    pub const K_CALLER_SAVED_SIMD128S: Simd128RegList = Simd128RegList {
        registers: [
            Some(v0),
            Some(v1),
            Some(v2),
            Some(v3),
            Some(v4),
            Some(v5),
            Some(v6),
            Some(v7),
            Some(v8),
            Some(v9),
            Some(v10),
            Some(v11),
            Some(v12),
            Some(v13),
            Some(v14),
            Some(v15),
            Some(v16),
            Some(v17),
            Some(v18),
            Some(v19),
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        count: 20,
    };

    pub const K_NUM_CALLER_SAVED_DOUBLES: usize = 14;

    pub const K_CALLEE_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: [
            Some(d14),
            Some(d15),
            Some(d16),
            Some(d17),
            Some(d18),
            Some(d19),
            Some(d20),
            Some(d21),
            Some(d22),
            Some(d23),
            Some(d24),
            Some(d25),
            Some(d26),
            Some(d27),
            Some(d28),
            Some(d29),
            Some(d30),
            Some(d31),
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        count: 18,
    };

    pub const K_NUM_CALLEE_SAVED_DOUBLES: usize = 18;
}
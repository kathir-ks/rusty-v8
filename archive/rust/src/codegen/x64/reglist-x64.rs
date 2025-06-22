// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_arch;
mod reglist_base;

pub mod internal {
    use crate::register_arch::Register;
    use crate::register_arch::DoubleRegister;
    use crate::reglist_base::RegListBase;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    // Rust doesn't have trivially copyable like C++, but these types should be Copy.
    // static_assertions::assert_impl_all!(RegList: Copy, Clone);
    // static_assertions::assert_impl_all!(DoubleRegList: Copy, Clone);

    pub const K_JS_CALLER_SAVED: RegList = RegList {
        registers: [
            Register::rax,
            Register::rcx,
            Register::rdx,
            Register::rbx, // used as a caller-saved register in JavaScript code
            Register::rdi, // callee function
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
        ],
        len: 5,
    };

    #[cfg(target_os = "windows")]
    pub const K_CALLER_SAVED: RegList = RegList {
        registers: [
            Register::rax,
            Register::rcx,
            Register::rdx,
            Register::r8,
            Register::r9,
            Register::r10,
            Register::r11,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
        ],
        len: 7,
    };

    #[cfg(not(target_os = "windows"))]
    pub const K_CALLER_SAVED: RegList = RegList {
        registers: [
            Register::rax,
            Register::rcx,
            Register::rdx,
            Register::rdi,
            Register::rsi,
            Register::r8,
            Register::r9,
            Register::r10,
            Register::r11,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
            Register::None,
        ],
        len: 9,
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 5;
}
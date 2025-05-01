// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a dummy implementation for when CFI is not enabled.

// src/execution/pointer-authentication-dummy.rs

pub mod pointer_authentication {
    use crate::base::logging::UNREACHABLE;
    use crate::flags::flags;
    use std::ptr::null_mut;

    // Type alias for Address, assuming it's a pointer-sized integer
    pub type Address = usize;

    pub struct PointerAuthentication {}

    impl PointerAuthentication {
        /// Load return address from `pc_address` and return it.
        #[inline]
        pub fn authenticate_pc(pc_address: *mut Address, _: u32) -> Address {
            unsafe { *pc_address }
        }

        /// Return `pc` unmodified.
        #[inline]
        pub fn strip_pac(pc: Address) -> Address {
            pc
        }

        /// Store `new_pc` to `pc_address` without signing.
        #[inline]
        pub fn replace_pc(pc_address: *mut Address, new_pc: Address, _: i32) {
            unsafe {
                *pc_address = new_pc;
            }
        }

        /// Return `pc` unmodified.
        #[inline]
        pub fn sign_and_check_pc(_: *mut Isolate, pc: Address, _: Address) -> Address {
            pc
        }

        #[inline]
        pub fn move_signed_pc(_: *mut Isolate, pc: Address, _: Address, _: Address) -> Address {
            #[cfg(feature = "v8_enable_webassembly")]
            {
                // Only used by wasm deoptimizations and growable stacks.
                assert!(flags::FLAG_wasm_deopt || flags::FLAG_experimental_wasm_growable_stacks);
                pc
            }
            #[cfg(not(feature = "v8_enable_webassembly"))]
            {
                UNREACHABLE();
                pc // Dummy return to satisfy the type checker. UNREACHABLE will halt execution in practice.
            }
        }
    }

    // Dummy Isolate struct, as it is used only as a pointer.
    pub struct Isolate {}
}

pub mod base {
    pub mod logging {
        #[inline(never)]
        #[cold]
        pub fn UNREACHABLE() -> ! {
            panic!("UNREACHABLE");
        }
    }
}

pub mod flags {
    pub mod flags {
        pub static FLAG_wasm_deopt: bool = false;
        pub static FLAG_experimental_wasm_growable_stacks: bool = false;
    }
}
// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Push all callee-saved registers to get them on the stack for conservative
// stack scanning.

// See asm/x64/push_registers_clang.cc for why the function is not generated
// using clang.

// Do not depend on V8_TARGET_OS_* defines as some embedders may override the
// GN toolchain (e.g. ChromeOS) and not provide them.

mod heap {
    pub mod base {
        #[repr(C)]
        pub struct Stack {
            // Add fields here based on the actual Stack struct definition in V8
            // This is just a placeholder.
        }

        #[repr(C)]
        pub struct StackVisitor {
            // Add fields here based on the actual StackVisitor struct definition in V8
            // This is just a placeholder.
        }

        pub type IterateStackCallback = extern "C" fn(
            sp: *const Stack,
            sv: *mut StackVisitor,
            stack_pointer: usize,
        );

        #[cfg(target_arch = "s390x")]
        #[link(name = "v8_lib", kind = "static")]
        extern "C" {
            pub fn PushAllRegistersAndIterateStack(
                sp: *const Stack,
                sv: *mut StackVisitor,
                callback: IterateStackCallback,
            );
        }

        #[cfg(not(target_arch = "s390x"))]
        pub fn PushAllRegistersAndIterateStack(
            _sp: *const Stack,
            _sv: *mut StackVisitor,
            _callback: IterateStackCallback,
        ) {
            // Placeholder implementation for non-s390x architectures.
            // The original C++ code uses inline assembly specific to s390x.
            // A platform-independent alternative would need to be implemented here,
            // or the function could be made s390x-specific using conditional compilation.
            // For now, we just provide an empty implementation for other architectures.
            unimplemented!("PushAllRegistersAndIterateStack is only implemented for s390x");
        }
    }
}
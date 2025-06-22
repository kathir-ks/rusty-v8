// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Clients of this interface shouldn't depend on compiler internals.

mod base {
    pub mod macros {} // Placeholder, might need more specific implementation based on usage
}

mod utils {
    pub mod utils {} // Placeholder, might need more specific implementation based on usage
}

pub mod internal {
    pub struct Isolate {}
    pub struct JSFunction {}

    pub mod compiler {
        use std::ptr::NonNull;

        #[derive(PartialEq, Eq)]
        pub enum IsScriptAvailable {
            No,
            Yes,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct BytecodeOffset(Option<usize>);

        impl BytecodeOffset {
            pub const NONE: BytecodeOffset = BytecodeOffset(None);

            pub fn new(offset: usize) -> Self {
                BytecodeOffset(Some(offset))
            }

            pub fn is_none(&self) -> bool {
                self.0.is_none()
            }

            pub fn get(&self) -> Option<usize> {
                self.0
            }
        }

        impl Default for BytecodeOffset {
            fn default() -> Self {
                Self::NONE
            }
        }


        pub struct TurbofanCompilationJob {}

        extern "C" {
            // TODO: The `v8::internal::compiler::NewCompilationJob` function is marked as
            // `V8_EXPORT_PRIVATE`, which suggests it is intended for internal use within
            // the V8 library.  Without more context, directly translating this function
            // as extern "C" may not be correct. It might require accessing internal V8 APIs,
            // which is generally discouraged unless necessary.
            // If this Rust code is meant to be part of V8, then this extern block is
            // appropriate and the types are assumed to match the C++ definitions.
            // Otherwise, an alternative approach, such as re-implementing the
            // functionality or using a different, publicly accessible V8 API,
            // should be considered.
            fn v8_internal_compiler_new_compilation_job(
                isolate: *mut super::Isolate,
                function: *mut super::JSFunction,
                has_script: IsScriptAvailable,
                osr_offset: BytecodeOffset,
            ) -> *mut TurbofanCompilationJob;
        }

        pub fn new_compilation_job(
            isolate: &mut super::Isolate,
            function: &mut super::JSFunction,
            has_script: IsScriptAvailable,
            osr_offset: BytecodeOffset,
        ) -> Option<Box<TurbofanCompilationJob>> {
            unsafe {
                let job_ptr = v8_internal_compiler_new_compilation_job(
                    isolate,
                    function,
                    has_script,
                    osr_offset,
                );
                if job_ptr.is_null() {
                    None
                } else {
                    NonNull::new(job_ptr).map(|ptr| Box::from_raw(ptr.as_ptr()))
                }
            }
        }
    }
}
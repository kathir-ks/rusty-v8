// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    /// Represents a heap object name.
    #[derive(Debug)]
    pub struct HeapObjectName {
        pub name: *mut i8,
        pub is_internal: bool,
    }

    impl HeapObjectName {
        pub fn new(name: *mut i8, is_internal: bool) -> Self {
            HeapObjectName { name, is_internal }
        }
    }
} // namespace internal

pub mod name_provider {
    pub const K_HIDDEN_NAME: &str = ""; //  C++: constexpr const char NameProvider::kHiddenName[];
    pub const K_NO_NAME_DEDUCIBLE: &str = "<no name>"; // C++: constexpr const char NameProvider::kNoNameDeducible[];
}

pub mod name_trait {
    use super::internal::HeapObjectName;
    use super::name_provider;
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;

    pub struct NameTraitBase {}

    impl NameTraitBase {
        /// Gets the name from a type signature.
        ///
        /// # Arguments
        ///
        /// * `signature` - The type signature.
        pub fn get_name_from_type_signature(signature: Option<&str>) -> HeapObjectName {
            // Parsing string of structure:
            //    static HeapObjectName NameTrait<int>::GetNameFor(...) [T = int]
            match signature {
                None => HeapObjectName::new(
                    name_provider::K_NO_NAME_DEDUCIBLE.as_ptr() as *mut i8,
                    false,
                ),
                Some(sig) => {
                    let raw = sig.to_string();
                    if let Some(start_pos) = raw.rfind("T = ") {
                        let start_pos = start_pos + 4;
                        let len = raw.len() - start_pos - 1;
                        let name = &raw[start_pos..(start_pos + len)];

                        let c_string = CString::new(name).unwrap();
                        let name_ptr = c_string.as_ptr() as *mut c_char;
                        std::mem::forget(c_string); // Prevent deallocation

                        HeapObjectName::new(name_ptr as *mut i8, false)
                    } else {
                        HeapObjectName::new(
                            name_provider::K_NO_NAME_DEDUCIBLE.as_ptr() as *mut i8,
                            false,
                        )
                    }
                }
            }
        }
    }
} // namespace cppgc
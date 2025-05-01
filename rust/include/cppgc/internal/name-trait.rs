// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::marker::PhantomData;
    use std::ptr;

    // TODO: Replace with feature detection once available in Rust.
    const CPPGC_SUPPORTS_OBJECT_NAMES: bool = true;

    #[derive(Clone, Copy)]
    pub struct HeapObjectName {
        pub value: *const std::os::raw::c_char,
        pub name_was_hidden: bool,
    }

    impl HeapObjectName {
        pub fn new(value: *const std::os::raw::c_char, name_was_hidden: bool) -> Self {
            HeapObjectName {
                value,
                name_was_hidden,
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum HeapObjectNameForUnnamedObject {
        kUseClassNameIfSupported,
        kUseHiddenName,
    }

    pub struct NameTraitBase {}

    impl NameTraitBase {
        pub fn get_name_from_type_signature(_signature: *const std::os::raw::c_char) -> HeapObjectName {
            // This function would extract the type name from the signature.
            // The signature format depends on the compiler, and it's not easily
            // portable to Rust. This is a placeholder.
            HeapObjectName::new(ptr::null(), false) // Placeholder implementation.
        }
    }

    // TODO: Define NameProvider in Rust (related to cppgc::NameProvider from C++)
    pub trait NameProvider {
        fn get_human_readable_name(&self) -> *const std::os::raw::c_char;
        const kHiddenName: *const std::os::raw::c_char = ptr::null(); // Placeholder
    }

    pub struct NameTrait<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> NameTrait<T> {
        pub const fn has_non_hidden_name() -> bool {
            if CPPGC_SUPPORTS_OBJECT_NAMES {
                true
            } else {
                // In C++, this checks `std::is_base_of<NameProvider, T>::value`.
                // In Rust, this means checking if T implements NameProvider trait.
                // However, without a concrete NameProvider trait and its implementations,
                // this check cannot be fully implemented.
                false // Placeholder, adjust when NameProvider is properly defined.
            }
        }

        pub fn get_name(obj: *const std::ffi::c_void, name_retrieval_mode: HeapObjectNameForUnnamedObject) -> HeapObjectName {
            unsafe {
                let obj_typed: *const T = obj as *const T;
                NameTrait::<T>::get_name_for(obj_typed, name_retrieval_mode)
            }
        }

        fn get_name_for<NP: NameProvider>(name_provider: *const NP, _name_retrieval_mode: HeapObjectNameForUnnamedObject) -> HeapObjectName {
           unsafe {
              HeapObjectName::new((*name_provider).get_human_readable_name(), false)
           }
        }

        fn get_name_for(_void: *const std::ffi::c_void, name_retrieval_mode: HeapObjectNameForUnnamedObject) -> HeapObjectName {
            if name_retrieval_mode == HeapObjectNameForUnnamedObject::kUseHiddenName {
                // Using static method NameProvider::kHiddenName requires NameProvider to be available
                return HeapObjectName::new(ptr::null(), true);  // Placeholder
            }

            if CPPGC_SUPPORTS_OBJECT_NAMES {
                // Emulating compile-time typename retrieval requires access to compiler features
                // and a mechanism to store and access the name at compile time.
                // This requires a more complex approach in Rust, likely involving macros
                // and static storage.
                return HeapObjectName::new(ptr::null(), false); // Placeholder
            }
            // We wanted to use a class name but were unable to provide one due to
            // compiler limitations or build configuration. As such, return the hidden
            // name with name_was_hidden=false, which will cause this object to be
            // visible in the snapshot.
            HeapObjectName::new(ptr::null(), false)
        }
    }

    pub type NameCallback = fn(*const std::ffi::c_void, HeapObjectNameForUnnamedObject) -> HeapObjectName;
}
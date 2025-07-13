// Converted from V8 C++ source files:
// Header: name-trait.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

    #[derive(Clone, Copy)]
    pub struct NameBuffer<const SIZE: usize> {
        pub name: [i8; SIZE + 1],
    }

    impl<const SIZE: usize> NameBuffer<SIZE> {
        pub fn from_c_string(str: &[i8]) -> NameBuffer<SIZE> {
            let mut result = NameBuffer { name: [0; SIZE + 1] };
            for i in 0..SIZE {
                result.name[i] = str[i];
            }
            result.name[SIZE] = 0;
            result
        }
    }

    pub fn get_typename<T>() -> &'static str {
       core::any::type_name::<T>()
    }

    #[derive(Debug, Clone, Copy)]
    pub struct HeapObjectName {
        pub value: *const i8,
        pub name_was_hidden: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum HeapObjectNameForUnnamedObject {
        kUseClassNameIfSupported,
        kUseHiddenName,
    }

    pub struct NameTraitBase {}

    impl NameTraitBase {
        pub fn get_name_from_type_signature(_name: *const i8) -> HeapObjectName {
            HeapObjectName {
                value: std::ptr::null(),
                name_was_hidden: false,
            }
        }
    }

    pub struct NameTrait {}

    impl NameTrait {
        pub const fn has_non_hidden_name<T>() -> bool {
           true
        }

        pub fn get_name<T>(
            obj: *const T,
            name_retrieval_mode: HeapObjectNameForUnnamedObject,
        ) -> HeapObjectName {
            Self::get_name_for::<T>(obj as *const _, name_retrieval_mode)
        }

        fn get_name_for<T>(
            _name_provider: *const T,
            _mode: HeapObjectNameForUnnamedObject,
        ) -> HeapObjectName {
            let hidden_name: *const i8 =  NameProvider::kHiddenName;
            HeapObjectName {
                value: hidden_name,
                name_was_hidden: true,
            }
        }
    }

    pub type NameCallback = fn(*const std::ffi::c_void, HeapObjectNameForUnnamedObject) -> HeapObjectName;
}
}
pub mod v8 {
}
pub struct String_ExternalOneByteStringResource;

pub struct Local<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Local<'a, T> {
    pub fn new() -> Self {
        Local {
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct Object;

pub struct ArrayBuffer;
pub mod NameProvider {
pub static kHiddenName: *const i8 = std::ptr::null();
}

// Converted from V8 C++ source files:
// Header: logging.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::marker::PhantomData;

    pub struct SourceLocation {}

    impl SourceLocation {
        pub fn Current() -> Self {
            SourceLocation {}
        }
    }

    extern "C" {
        pub fn DCheckImpl(_: *const i8, _: &SourceLocation);
        pub fn FatalImpl(_: *const i8, _: &SourceLocation) -> !;
    }

    pub struct EatParams<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> EatParams<T> {
        pub fn new() -> Self {
            EatParams {
                _phantom: PhantomData,
            }
        }
    }

    #[macro_export]
    macro_rules! CPPGC_DCHECK_MSG {
        ($condition:expr, $message:expr) => {
            if !($condition) {
                unsafe {
                    let message_cstr = std::ffi::CString::new($message).unwrap();
                    ::cppgc::internal::DCheckImpl(message_cstr.as_ptr(), &::cppgc::internal::SourceLocation::Current());
                }
            }
        };
    }

    #[macro_export]
    macro_rules! CPPGC_DCHECK {
        ($condition:expr) => {
            CPPGC_DCHECK_MSG!($condition, stringify!($condition));
        };
    }

    #[macro_export]
    macro_rules! CPPGC_CHECK_MSG {
        ($condition:expr, $message:expr) => {
            if !($condition) {
                unsafe {
                    let message_cstr = std::ffi::CString::new($message).unwrap();
                    ::cppgc::internal::FatalImpl(message_cstr.as_ptr(), &::cppgc::internal::SourceLocation::Current());
                }
            }
        };
    }

    #[macro_export]
    macro_rules! CPPGC_CHECK {
        ($condition:expr) => {
            CPPGC_CHECK_MSG!($condition, stringify!($condition));
        };
    }
}

// Converted from V8 C++ source files:
// Header: utils-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod platform {
        pub mod time {
            pub const kMillisecondsPerSecond: i64 = 1000;
        }
    }
    pub mod strings {
        pub type base = i32;
    }
}
pub mod init {
    pub mod v8 {
        pub struct V8 {}
    }
}
pub mod strings {
    pub mod char_predicates_inl {
        pub fn IsDecimalDigit(c: u32) -> bool {
            (c as u8).is_ascii_digit()
        }
    }
}
pub mod utils {
    pub mod utils {}
}

use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod v8 {
    use super::*;
    pub mod internal {

        pub struct V8_NODISCARDTimedScope<'a> {
            start_: f64,
            result_: &'a mut f64,
        }

        impl<'a> V8_NODISCARDTimedScope<'a> {
            pub fn new(result: &'a mut f64) -> Self {
                V8_NODISCARDTimedScope {
                    start_: Self::timestamp_ms(),
                    result_: result,
                }
            }

            fn timestamp_ms() -> f64 {
                let now = SystemTime::now();
                let since_the_epoch = now
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                since_the_epoch.as_secs_f64() * (base::platform::time::kMillisecondsPerSecond as f64)
            }
        }

        impl<'a> Drop for V8_NODISCARDTimedScope<'a> {
            fn drop(&mut self) {
                *self.result_ = Self::timestamp_ms() - self.start_;
            }
        }

        pub fn try_add_array_index_char(index: &mut u32, c: u32) -> bool {
            assert!(super::super::strings::char_predicates_inl::IsDecimalDigit(c));
            let d = (c as u8 - b'0') as u32;
            if *index > 429496729 - ((d + 3) >> 3) {
                return false;
            }
            *index = (*index) * 10 + d;
            true
        }

        pub fn try_add_integer_index_char(index: &mut u64, c: u32) -> bool {
            assert!(super::super::strings::char_predicates_inl::IsDecimalDigit(c));
            let d = (c as u8 - b'0') as u64;
            *index = (*index) * 10 + d;
            *index <= kMaxSafeIntegerUint64
        }

        #[derive(PartialEq)]
        pub enum ToIndexMode {
            kToArrayIndex,
            kToIntegerIndex,
        }

        const kMaxSafeIntegerUint64: u64 = 9007199254740991;

        pub trait Stream {
            fn get_next(&mut self) -> u32;
            fn has_more(&self) -> bool;
        }

        pub fn string_to_index<S: Stream, T>(stream: &mut S, index: &mut T, mode: ToIndexMode) -> bool
        where
            T: Sized,
        {
            let ch = stream.get_next();

            if ch == '0' as u32 {
                *index = unsafe { std::mem::transmute_copy(&0u64) };
                return !stream.has_more();
            }

            if !super::super::strings::char_predicates_inl::IsDecimalDigit(ch) {
                return false;
            }

            let d = (ch as u8 - b'0') as u64;
            let mut result: u64 = d;

            while stream.has_more() {
                let c = stream.get_next();
                if !super::super::strings::char_predicates_inl::IsDecimalDigit(c) {
                    return false;
                }

                if std::mem::size_of::<T>() == 8 {
                    assert_eq!(ToIndexMode::kToIntegerIndex, mode);
                    if !try_add_integer_index_char(&mut result, c) {
                        return false;
                    }
                } else {
                    let mut result_u32: u32 = result as u32;
                    if !try_add_array_index_char(&mut result_u32, c) {
                        return false;
                    }
                    result = result_u32 as u64;
                }
            }

            *index = unsafe { std::mem::transmute_copy(&result) };
            true
        }
    }
}

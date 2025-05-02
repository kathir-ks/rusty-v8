// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        array,
        fmt,
        i32, i64,
        marker::PhantomData,
        mem,
        num::TryFromIntError,
        string::String,
        string::ToString,
        str,
        str::Utf8Error,
    };

    // Implementation detail, do not use outside this module. The public interface
    // is below.
    mod impl_ {
        use super::*;

        pub struct JoinedStringViews<const STRS: [&'static str]> {
            _phantom: PhantomData<[&'static str; STRS.len()]>,
        }

        impl<const STRS: [&'static str]> JoinedStringViews<STRS> {
            const ARRAY_SIZE: usize = STRS.iter().map(|s| s.len()).sum::<usize>() + 1;

            pub const fn join_into_null_terminated_array() -> [u8; Self::ARRAY_SIZE] {
                let mut arr = [0u8; Self::ARRAY_SIZE];
                let mut ptr = 0;
                let mut i = 0;
                while i < STRS.len() {
                    let str_ = STRS[i];
                    let mut j = 0;
                    while j < str_.len() {
                        arr[ptr] = str_.as_bytes()[j];
                        ptr += 1;
                        j += 1;
                    }
                    i += 1;
                }
                arr[ptr] = 0;
                arr
            }

            pub const ARRAY: [u8; Self::ARRAY_SIZE] = Self::join_into_null_terminated_array();

            pub const STRING_VIEW: &'static str =
                unsafe { str::from_utf8_unchecked(&Self::ARRAY[..Self::ARRAY_SIZE - 1]) };
        }

        pub trait FixedSizeString: Sized {
            const CHAR_ARRAY_SIZE: usize;
            fn as_str(&self) -> &str;
        }

        impl<const N: usize> FixedSizeString for [char; N] {
            const CHAR_ARRAY_SIZE: usize = N;
            fn as_str(&self) -> &str {
                self.iter().collect::<String>().as_str()
            }
        }

        pub trait FormattedStringPart<T> {
            const MAX_LEN: usize;
            const FORMAT_PART: &'static str;
            fn value(&self) -> T;
        }

        macro_rules! impl_formatted_string_part_for_integer {
            ($type:ty, $is_signed:expr, $size:expr, $unsigned_format:expr, $signed_format:expr) => {
                impl FormattedStringPart<$type> for $type {
                    const MAX_LEN: usize = if $size == 8 { 20 } else { 10 } + if $is_signed { 1 } else { 0 };
                    const FORMAT_PART: &'static str = if $size == 8 {
                        if $is_signed {
                            $signed_format
                        } else {
                            $unsigned_format
                        }
                    } else {
                        if $is_signed {
                            $signed_format
                        } else {
                            $unsigned_format
                        }
                    };

                    fn value(&self) -> $type {
                        *self
                    }
                }
            };
        }

        impl_formatted_string_part_for_integer!(i32, true, 4, "%u", "%d");
        impl_formatted_string_part_for_integer!(u32, false, 4, "%u", "%d");
        impl_formatted_string_part_for_integer!(i64, true, 8, "%u", "%d");
        impl_formatted_string_part_for_integer!(u64, false, 8, "%u", "%d");

        impl<S: FixedSizeString> FormattedStringPart<S> for S {
            const CHAR_ARRAY_SIZE: usize = S::CHAR_ARRAY_SIZE;
            const MAX_LEN: usize = Self::CHAR_ARRAY_SIZE - 1;
            const FORMAT_PART: &'static str = "%s";

            fn value(&self) -> S {
                *self
            }
        }

        pub fn print_formatted_string_to_array<const MAX_LEN: usize, const FORMAT: &'static str, Parts>(
            parts: Parts,
        ) -> Result<[u8; MAX_LEN], fmt::Error>
        where
            Parts: PrintfArgs,
        {
            let mut message = [0u8; MAX_LEN];

            if FORMAT.is_empty() {
                message[0] = 0;
            } else {
                let formatted = format!(FORMAT, parts);

                let bytes = formatted.as_bytes();
                if bytes.len() >= MAX_LEN {
                   return Err(fmt::Error);
                }
                message[..bytes.len()].copy_from_slice(bytes);
                message[bytes.len()] = 0;
            }

            Ok(message)
        }

        pub trait PrintfArgs { }

        impl PrintfArgs for () { }

        impl<T, P> fmt::Display for (T,P)
            where
                T: fmt::Display,
                P: PrintfArgs + fmt::Display
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.0, self.1)
            }
        }

        impl<T: fmt::Display> fmt::Display for (T,) {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::Display for String {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    }

    /// `FormattedString` allows to format strings with statically known number and
    /// type of constituents.
    /// The class stores all values that should be printed, and generates the final
    /// string via `format!` into a `std::array`, without any dynamic memory
    /// allocation. The format string is computed statically.
    /// This makes this class not only very performant, but also suitable for
    /// situations where we do not want to perform any memory allocation (like for
    /// reporting OOM or fatal errors).
    ///
    /// Use like this:
    ///   let message = FormattedString::new().append("Cannot allocate ").append(size).append(" bytes");
    ///   // V8::FatalProcessOutOfMemory(nullptr, message.PrintToArray().data()); //Needs adaptation
    ///
    /// This code is compiled into the equivalent of
    ///   let mut message_arr = [0u8; 34];
    ///   let chars = format!(message_arr.as_mut_ptr(), 34, "%s%d%s", "Cannot allocate ",
    ///                        size, " bytes");
    ///   //V8::FatalProcessOutOfMemory(nullptr, message_arr.data()); //Needs adaptation
    #[derive(Clone)]
    pub struct FormattedString<Parts = ()> {
        parts: Parts,
    }

    impl FormattedString {
        pub const fn new() -> Self {
            Self { parts: () }
        }
    }

    impl<Parts> FormattedString<Parts> {
        // Add one more part to the FormattedString.
        pub fn append<T: fmt::Display>(self, t: T) -> FormattedString<(T, Parts)> {
            FormattedString { parts: (t, self.parts) }
        }

        // Print this FormattedString into an array. Does not allocate any dynamic
        // memory. The result lives on the stack of the caller.
        pub fn print_to_array<const MAX_LEN: usize, const FORMAT: &'static str>(
            &self,
        ) -> Result<[u8; MAX_LEN], fmt::Error>
        where
        Parts: impl_::PrintfArgs + fmt::Display
        {
            impl_::print_formatted_string_to_array::<MAX_LEN, FORMAT, Parts>(self.parts)
        }
    }
}
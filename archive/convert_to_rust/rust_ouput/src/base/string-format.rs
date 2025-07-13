// Converted from V8 C++ source files:
// Header: string-format.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {
pub mod platform {
} // platform
} // platform
use std::array;
use std::fmt;
use std::fmt::Write;
use std::marker::PhantomData;
use std::mem;
use std::string::String;
pub mod logging {
    macro_rules! CHECK {
        ($condition:expr) => {
            if !$condition {
                panic!("Check failed: {}", stringify!($condition));
            }
        };
    }
    pub(crate) use CHECK;
}
pub mod os {
    use std::ffi::c_char;
    use std::ffi::c_int;
    use std::fmt::Write;
    pub struct SNPrintFError;
    impl std::fmt::Debug for SNPrintFError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SNPrintFError")
        }
    }
    impl std::fmt::Display for SNPrintFError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SNPrintFError")
        }
    }
    impl std::error::Error for SNPrintFError {}
    pub fn snprintf(buf: &mut [u8], format: &str, args: fmt::Arguments) -> Result<usize, SNPrintFError> {
        let mut buffer = String::new();
        buffer.write_fmt(args).map_err(|_| SNPrintFError {})?;
        let buffer_bytes = buffer.as_bytes();
        let len = std::cmp::min(buf.len() - 1, buffer_bytes.len());
        buf[..len].copy_from_slice(&buffer_bytes[..len]);
        buf[len] = 0;
        Ok(len)
    }
    pub struct OS {}
    impl OS {
        pub fn SNPrintF(buf: *mut char, size: usize, format: *const char, _args: ...) -> i32 {
            use std::ffi::CStr;
            use std::os::raw::c_char;
            use std::slice;
            let format_str = unsafe {
                CStr::from_ptr(format)
                    .to_str()
                    .expect("Failed to convert format string to UTF-8")
            };
            let mut vec: Vec<u8> = Vec::with_capacity(size);
            unsafe {
                vec.set_len(size);
            }
            let result = Self::SNPrintFImpl(vec.as_mut_ptr() as *mut char, size, format_str, ());
            result
        }
        pub fn SNPrintFImpl<T>(buf: *mut char, size: usize, format: &str, args: T) -> i32 {
            use std::ffi::CString;
            use std::fmt::Write;
            use std::os::raw::c_char;
            let mut output = String::new();
            write!(&mut output, "{}", format).unwrap();
            let c_string = CString::new(output).unwrap();
            let c_str = c_string.as_ptr();
            let mut vec: Vec<u8> = Vec::with_capacity(size);
            unsafe {
                vec.set_len(size);
            }
            if format.is_empty() {
                return 0;
            }
            let res =
                format.len() as i32;
            res
        }
    }
} // os
    mod impl_ {
        use std::array;
        use std::string::String;
        pub struct JoinedStringViews<const S: &'static str, const T: &'static str> {
            _phantom: std::marker::PhantomData<(&'static str, &'static str)>,
        }
        impl<const S: &'static str, const T: &'static str> JoinedStringViews<const S, const T> {
            const fn join_into_null_terminated_array() -> [u8; S.len() + T.len() + 1] {
                let mut arr = [0u8; S.len() + T.len() + 1];
                let mut i = 0;
                let s_bytes = S.as_bytes();
                let t_bytes = T.as_bytes();
                while i < s_bytes.len() {
                    arr[i] = s_bytes[i];
                    i += 1;
                }
                let mut j = 0;
                while j < t_bytes.len() {
                    arr[i + j] = t_bytes[j];
                    j += 1;
                }
                arr[i + j] = 0;
                arr
            }
            pub const ARRAY: [u8; S.len() + T.len() + 1] = Self::join_into_null_terminated_array();
            pub const STRING_VIEW: &'static str =
                unsafe { std::str::from_utf8_unchecked(&Self::ARRAY[..Self::ARRAY.len() - 1]) };
        }
    } // impl_
    pub trait FixedSizeString {
        const CHAR_ARRAY_SIZE: usize;
    }
    impl<const N: usize> FixedSizeString for [char; N] {
        const CHAR_ARRAY_SIZE: usize = N;
    }
    pub trait FormattedStringPart {
        const K_MAX_LEN: usize;
        const K_FORMAT_PART: &'static str;
        type StorageType;
        fn value(&self) -> Self::StorageType;
    }
    pub struct IntFormattedStringPart<I> {
        pub value: I,
        _phantom: std::marker::PhantomData<I>,
    }
    impl<I> IntFormattedStringPart<I> {
        pub const fn new(value: I) -> Self {
            Self {
                value,
                _phantom: std::marker::PhantomData,
            }
        }
    }
    impl<I: std::fmt::Display + Copy> FormattedStringPart for IntFormattedStringPart<I>
    where
        I: Sized,
    {
        const K_MAX_LEN: usize = 20;
        const K_FORMAT_PART: &'static str = "%d";
        type StorageType = I;
        fn value(&self) -> Self::StorageType {
            self.value
        }
    }
    pub struct StringFormattedStringPart<S> {
        pub value: S,
        _phantom: std::marker::PhantomData<S>,
    }
    impl<S> StringFormattedStringPart<S> {
        pub const fn new(value: S) -> Self {
            Self {
                value,
                _phantom: std::marker::PhantomData,
            }
        }
    }
    impl FormattedStringPart for StringFormattedStringPart<&'static str> {
        const K_MAX_LEN: usize = 128;
        const K_FORMAT_PART: &'static str = "%s";
        type StorageType = &'static str;
        fn value(&self) -> Self::StorageType {
            self.value
        }
    }
    pub fn print_formatted_string_to_array<const K_FORMAT: &'static str, const K_MAX_LEN: usize, P>(
        parts: P,
    ) -> Result<array::IntoIter<char, K_MAX_LEN>, PrintFormattedStringToArrayError>
    where
        P: PrintFormattedStringTypeArguments,
    {
        if K_FORMAT.is_empty() {
            let mut message: [char; K_MAX_LEN] = ['\0'; K_MAX_LEN];
            message[0] = '\0';
            return Ok(message.into_iter());
        }
        if std::mem::size_of::<P>() == 0 {
            return Err(PrintFormattedStringToArrayError::ZeroSize);
        }
        let mut message: [char; K_MAX_LEN] = ['\0'; K_MAX_LEN];
        let mut format_string = String::new();
        format_string.push_str(K_FORMAT);
        let mut buffer: Vec<u8> = Vec::with_capacity(K_MAX_LEN);
        unsafe {
            buffer.set_len(K_MAX_LEN);
        }
        let characters = crate::base::os::OS::SNPrintFImpl(
            message.as_mut_ptr(),
            K_MAX_LEN,
            &format_string,
            (),
        );
        crate::base::logging::CHECK!(characters >= 0 && characters as usize <= K_MAX_LEN);
        crate::base::logging::CHECK!(message[characters as usize] == '\0');
        Ok(message.into_iter())
    }
    #[derive(Debug)]
    pub enum PrintFormattedStringToArrayError {
        ZeroSize,
        FormatError,
    }
    impl std::fmt::Display for PrintFormattedStringToArrayError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PrintFormattedStringToArrayError::ZeroSize => {
                    write!(f, "PrintFormattedStringToArrayError: ZeroSize")
                }
                PrintFormattedStringToArrayError::FormatError => {
                    write!(f, "PrintFormattedStringToArrayError: FormatError")
                }
            }
        }
    }
    impl std::error::Error for PrintFormattedStringToArrayError {}
    pub trait PrintFormattedStringTypeArguments {}
    pub struct FormattedString<Ts: FormattedStringParts> {
        parts_: Ts,
    }
    impl Default for FormattedString<()> {
        fn default() -> Self {
            Self { parts_: () }
        }
    }
    impl FormattedString<()> {
        pub const fn new() -> Self {
            Self { parts_: () }
        }
    }
    impl<Ts: FormattedStringParts> FormattedString<Ts> {
        pub const K_MAX_LEN: usize = Ts::k_max_len();
        pub const K_FORMAT: &'static str = Ts::k_format();
        pub fn print_to_array(&self) -> Result<array::IntoIter<char, { Self::K_MAX_LEN }>, PrintFormattedStringToArrayError> {
            crate::base::print_formatted_string_to_array::<{ Self::K_FORMAT }, { Self::K_MAX_LEN }, _>(
                self.parts_.clone(),
            )
        }
    }
    impl<Ts: FormattedStringParts> FormattedString<Ts>
    where
        Ts: Clone,
    {
        pub fn add<T>(self, t: T) -> FormattedString<<Ts as AddFormattedStringPart<T>>::Output>
        where
            Ts: AddFormattedStringPart<T>,
        {
            FormattedString {
                parts_: self.parts_.add(t),
            }
        }
    }
    pub trait FormattedStringParts: Sized + Clone {
        type Head;
        type Tail: FormattedStringParts;
        const fn k_max_len() -> usize;
        const fn k_format() -> &'static str;
        fn add<T>(self, t: T) -> <Self as AddFormattedStringPart<T>>::Output
        where
            Self: AddFormattedStringPart<T>,
        {
            <Self as AddFormattedStringPart<T>>::add_formatted_string_part(self, t)
        }
    }
    impl FormattedStringParts for () {
        type Head = ();
        type Tail = ();
        const fn k_max_len() -> usize {
            1
        }
        const fn k_format() -> &'static str {
            ""
        }
    }
    impl PrintFormattedStringTypeArguments for () {}
    pub trait AddFormattedStringPart<T>: Sized {
        type Output: FormattedStringParts;
        fn add_formatted_string_part(self, t: T) -> Self::Output;
    }
    impl<T> AddFormattedStringPart<T> for ()
    where
        T: std::fmt::Display + Copy,
    {
        type Output = (IntFormattedStringPart<T>, ());
        fn add_formatted_string_part(self, t: T) -> Self::Output {
            (IntFormattedStringPart::new(t), ())
        }
    }
    impl<H, T, Hp, Tp> AddFormattedStringPart<T> for (H, (Tp,))
    where
        H: std::fmt::Display + Copy,
        Tp: std::fmt::Display + Copy,
    {
        type Output = (
            StringFormattedStringPart<&'static str>,
            (
                StringFormattedStringPart<&'static str>,
                (),
            ),
        );
        fn add_formatted_string_part(self, _t: T) -> Self::Output {
            (
                StringFormattedStringPart::new(""),
                (StringFormattedStringPart::new(""), ()),
            )
        }
    }
} // base

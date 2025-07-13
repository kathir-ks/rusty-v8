// Converted from V8 C++ source files:
// Header: strings.h
// Implementation: strings.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::ffi::VaList;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::slice;
    use std::vec::Vec;

    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(size: usize) -> Vector<T> {
            Vector {
                data: Vec::with_capacity(size),
            }
        }

        pub fn from_vec(vec: Vec<T>) -> Vector<T> {
            Vector { data: vec }
        }

        pub fn begin(&mut self) -> *mut T {
            self.data.as_mut_ptr()
        }

        pub fn length(&self) -> usize {
            self.data.capacity()
        }

        pub fn as_slice(&self) -> &[T] {
            self.data.as_slice()
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            self.data.as_mut_slice()
        }

        pub fn push(&mut self, value: T) {
            if self.data.len() < self.data.capacity() {
                self.data.push(value);
            }
        }
    }

    pub type uc16 = u16;
    pub type uc32 = u32;
    pub const k_uc16_size: usize = std::mem::size_of::<uc16>();

    pub fn vsnprintf(
        str: &mut [u8],
        format: *const c_char,
        args: VaList,
    ) -> Result<i32, std::fmt::Error> {
        use std::ffi::CStr;
        use std::fmt;
        use std::fmt::Write;

        let format_str = unsafe { CStr::from_ptr(format) }
            .to_str()
            .map_err(|_| std::fmt::Error)?;

        struct VsnPrintfWriter<'a> {
            buffer: &'a mut [u8],
            position: usize,
        }

        impl<'a> VsnPrintfWriter<'a> {
            fn new(buffer: &'a mut [u8]) -> Self {
                VsnPrintfWriter {
                    buffer,
                    position: 0,
                }
            }
        }

        impl<'a> fmt::Write for VsnPrintfWriter<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let bytes = s.as_bytes();
                let remaining = self.buffer.len() - self.position;

                if remaining == 0 {
                    return Err(fmt::Error);
                }

                let len = std::cmp::min(bytes.len(), remaining);
                self.buffer[self.position..self.position + len].copy_from_slice(&bytes[..len]);
                self.position += len;

                Ok(())
            }
        }

        let mut writer = VsnPrintfWriter::new(str);
        let result = match format_str.chars().next() {
            Some(_) => {
                // Since we cannot directly handle va_list in safe Rust, and the
                // 'format!' macro doesn't accept va_list, a true implementation
                // would require a C wrapper that calls vsnprintf and returns the
                // result.  For now, we return a dummy value.

                // In a real implementation, you'd use a C wrapper like this:
                // extern "C" {
                //   fn my_vsnprintf(buf: *mut u8, len: usize, format: *const c_char, args: VaList) -> c_int;
                // }
                // let written = unsafe { my_vsnprintf(str.as_mut_ptr() as *mut i8, str.len(), format, args) };
                // Ok(written)

                // A placeholder result for now.
                write!(&mut writer, "{}", format_str).map(|_| format_str.len() as i32)
            }
            None => Ok(0),
        };

        if let Ok(len) = result {
            if len as usize > str.len() {
                return Ok(-1);
            }
        }

        if writer.position < str.len() {
            str[writer.position] = 0; // Null-terminate
        } else if str.len() > 0 {
            str[str.len() - 1] = 0; // Ensure null-termination if truncated
        }

        result.map_err(|_| std::fmt::Error)
    }

    pub fn snprintf(
        str: &mut [u8],
        format: *const c_char,
        args: ...
    ) -> Result<i32, std::fmt::Error> {
        // This is a placeholder implementation.  A real implementation would
        // need to use a mechanism to pass variadic arguments into the
        // vsnprintf function.  The 'cva_list' crate can be used for this purpose,
        // but it introduces unsafe code.
        //
        // For now, we return a dummy success value.

        use std::ffi::CStr;
        use std::fmt::Write;
        use std::fmt;

        let format_str = unsafe { CStr::from_ptr(format) }
            .to_str()
            .map_err(|_| std::fmt::Error)?;

        struct SnPrintfWriter<'a> {
            buffer: &'a mut [u8],
            position: usize,
        }

        impl<'a> SnPrintfWriter<'a> {
            fn new(buffer: &'a mut [u8]) -> Self {
                SnPrintfWriter {
                    buffer,
                    position: 0,
                }
            }
        }

        impl<'a> fmt::Write for SnPrintfWriter<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let bytes = s.as_bytes();
                let remaining = self.buffer.len() - self.position;

                if remaining == 0 {
                    return Err(fmt::Error);
                }

                let len = std::cmp::min(bytes.len(), remaining);
                self.buffer[self.position..self.position + len].copy_from_slice(&bytes[..len]);
                self.position += len;

                Ok(())
            }
        }

        let mut writer = SnPrintfWriter::new(str);

        let result = write!(&mut writer, "{}", format_str).map(|_| format_str.len() as i32);

        if let Ok(len) = result {
            if len as usize > str.len() {
                return Ok(-1);
            }
        }

        if writer.position < str.len() {
            str[writer.position] = 0; // Null-terminate
        } else if str.len() > 0 {
            str[str.len() - 1] = 0; // Ensure null-termination if truncated
        }

        result.map_err(|_| std::fmt::Error)
    }

    pub fn strncpy(dest: &mut [u8], src: *const c_char, n: usize) {
        use std::ffi::CStr;

        if dest.is_empty() {
            return;
        }

        let src_slice = unsafe { CStr::from_ptr(src) };
        let src_bytes = src_slice.to_bytes();

        let len = std::cmp::min(n, dest.len() - 1);
        let src_len = std::cmp::min(len, src_bytes.len());

        dest[..src_len].copy_from_slice(&src_bytes[..src_len]);
        dest[src_len] = 0;
    }

    pub fn hex_value(c: uc32) -> i32 {
        let mut c = c as i32 - '0' as i32;
        if (c as u32) <= 9 {
            return c;
        }
        c = (c | 0x20) - ('a' as i32 - '0' as i32);
        if (c as u32) <= 5 {
            return c + 10;
        }
        return -1;
    }

    pub fn hex_char_of_value(value: i32) -> char {
        if (0 <= value) && (value <= 15) {
            if value < 10 {
                return (value + '0' as i32) as u8 as char;
            } else {
                return (value - 10 + 'A' as i32) as u8 as char;
            }
        }
        panic!("Value out of range");
    }
}

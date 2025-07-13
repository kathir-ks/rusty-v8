// Converted from V8 C++ source files:
// Header: ostreams.h
// Implementation: ostreams.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ostreams {
    use std::ffi::c_void;
    use std::fmt;
    use std::io::Write;
    use std::mem::MaybeUninit;
    use std::os::raw::c_int;
    use std::ptr;
    use std::slice;
    use std::str;
    use std::string::String;
    use std::sync::Mutex;

    use crate::base::macros::V8_EXPORT_PRIVATE;
    use crate::common::globals::kSystemPointerHexDigits;
    use crate::objects::string::String as V8String;

    #[cfg(target_os = "android")]
    extern "C" {
        fn __android_log_write(priority: c_int, tag: *const i8, text: *const i8) -> c_int;
    }

    pub struct OFStreamBase {
        f_: *mut libc::FILE,
    }

    impl OFStreamBase {
        pub fn new(f: *mut libc::FILE) -> OFStreamBase {
            OFStreamBase { f_: f }
        }

        fn sync(&mut self) -> Result<(), std::io::Error> {
            unsafe {
                if libc::fflush(self.f_) == 0 {
                    Ok(())
                } else {
                    Err(std::io::Error::last_os_error())
                }
            }
        }

        fn overflow(&mut self, c: i32) -> Result<(), std::io::Error> {
            if c != libc::EOF {
                unsafe {
                    if libc::fputc(c, self.f_) == libc::EOF {
                        return Err(std::io::Error::last_os_error());
                    }
                }
            }
            Ok(())
        }

        fn xsputn(&mut self, s: &str, n: usize) -> Result<usize, std::io::Error> {
            unsafe {
                let ptr = s.as_ptr() as *const c_void;
                let result = libc::fwrite(ptr, 1, n, self.f_);
                if result == n {
                    Ok(result)
                } else {
                    Err(std::io::Error::last_os_error())
                }
            }
        }
    }

    pub struct DbgStreamBuf {
        data_: [u8; 256],
        pptr_: *mut u8,
        eback_: *mut u8,
        egptr_: *mut u8,
    }

    impl DbgStreamBuf {
        pub fn new() -> DbgStreamBuf {
            let mut buf = DbgStreamBuf {
                data_: [0; 256],
                pptr_: ptr::null_mut(),
                eback_: ptr::null_mut(),
                egptr_: ptr::null_mut(),
            };
            buf.setp();
            buf
        }

        fn setp(&mut self) {
            self.eback_ = self.data_.as_mut_ptr();
            self.pptr_ = self.data_.as_mut_ptr();
            self.egptr_ = unsafe { self.data_.as_mut_ptr().add(self.data_.len()) };
        }

        fn sync(&mut self) -> Result<(), std::io::Error> {
            #[cfg(V8_OS_WIN)]
            unsafe {
                if IsDebuggerPresent() != 0 {
                    if self.eback_ != self.pptr_ {
                        let len = self.pptr_.offset_from(self.eback_) as usize;
                        let s = String::from_utf8_lossy(slice::from_raw_parts(self.eback_, len));
                        let wide: Vec<u16> = s.encode_utf16().collect();
                        OutputDebugStringW(wide.as_ptr());
                        self.setp();
                    }
                }
            }
            Ok(())
        }

        fn overflow(&mut self, c: i32) -> Result<(), std::io::Error> {
            #[cfg(V8_OS_WIN)]
            unsafe {
                if IsDebuggerPresent() != 0 {
                    self.sync()?;
                    if c != libc::EOF {
                        if self.eback_ == self.egptr_ {
                            let as_char = c as u8;
                            let s = String::from_utf8_lossy(&[as_char]);
                            let wide: Vec<u16> = s.encode_utf16().collect();
                            OutputDebugStringW(wide.as_ptr());
                        } else {
                            self.sputc(c as u8);
                        }
                    }
                }
            }
            Ok(())
        }

        fn sputc(&mut self, c: u8) {
            if self.pptr_ < self.egptr_ {
                unsafe {
                    *self.pptr_ = c;
                    self.pptr_ = self.pptr_.add(1);
                }
            }
        }
    }

    #[cfg(V8_OS_WIN)]
    extern "system" {
        fn IsDebuggerPresent() -> i32;
        fn OutputDebugStringW(lpOutputString: *const u16);
    }

    pub struct DbgStdoutStream {
        streambuf_: DbgStreamBuf,
    }

    impl DbgStdoutStream {
        pub fn new() -> DbgStdoutStream {
            DbgStdoutStream {
                streambuf_: DbgStreamBuf::new(),
            }
        }
    }

    impl fmt::Write for DbgStdoutStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for byte in s.bytes() {
                self.streambuf_
                    .overflow(byte as i32)
                    .map_err(|_| fmt::Error)?;
            }
            Ok(())
        }
    }

    pub struct OFStream {
        buf_: OFStreamBase,
        // Need to hold onto FILE* to keep it alive
        _file: *mut libc::FILE,
    }

    impl OFStream {
        pub fn new(f: *mut libc::FILE) -> OFStream {
            assert!(!f.is_null());
            OFStream {
                buf_: OFStreamBase::new(f),
                _file: f,
            }
        }
    }

    impl fmt::Write for OFStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buf_
                .xsputn(s, s.len())
                .map(|_| ())
                .map_err(|_| fmt::Error)
        }
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    pub struct AndroidLogStream {
        line_buffer_: String,
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    impl AndroidLogStream {
        pub fn new() -> AndroidLogStream {
            AndroidLogStream {
                line_buffer_: String::new(),
            }
        }

        fn xsputn(&mut self, s: &str, n: usize) -> usize {
            let mut written = 0;
            for chunk in s.as_bytes().chunks(n) {
                let chunk_str = str::from_utf8(chunk).unwrap();
                let mut start = 0;
                while let Some(newline_offset) = chunk_str[start..].find('\n') {
                    let line = &chunk_str[start..start + newline_offset];
                    self.line_buffer_.push_str(line);

                    unsafe {
                        let tag = "v8\0".as_ptr() as *const i8;
                        let text = self.line_buffer_.as_ptr() as *const i8;
                        __android_log_write(3, tag, text);
                    }

                    self.line_buffer_.clear();
                    start += newline_offset + 1;
                    written += line.len() + 1;
                }
                if start < chunk_str.len() {
                    self.line_buffer_.push_str(&chunk_str[start..]);
                    written += chunk_str.len() - start;
                }
            }
            written
        }
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    impl Drop for AndroidLogStream {
        fn drop(&mut self) {
            if !self.line_buffer_.is_empty() {
                unsafe {
                    let tag = "v8\0".as_ptr() as *const i8;
                    let text = self.line_buffer_.as_ptr() as *const i8;
                    __android_log_write(3, tag, text);
                }
            }
        }
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    pub struct StdoutStream {
        stream_: AndroidLogStream,
        mutex_guard_: MutexGuard,
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    impl StdoutStream {
        pub fn new() -> StdoutStream {
            StdoutStream {
                stream_: AndroidLogStream::new(),
                mutex_guard_: MutexGuard::new(get_stdout_mutex()),
            }
        }
    }

    #[cfg(all(target_os = "android", not(feature = "v8_android_log_stdout")))]
    impl fmt::Write for StdoutStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.stream_.xsputn(s, s.len());
            Ok(())
        }
    }

    #[cfg(not(all(target_os = "android", not(feature = "v8_android_log_stdout"))))]
    pub struct StdoutStream {
        inner: OFStream,
        mutex_guard_: MutexGuard,
    }

    #[cfg(not(all(target_os = "android", not(feature = "v8_android_log_stdout"))))]
    impl StdoutStream {
        pub fn new() -> StdoutStream {
            let stdout_file = unsafe { libc::stdout };
            StdoutStream {
                inner: OFStream::new(stdout_file),
                mutex_guard_: MutexGuard::new(get_stdout_mutex()),
            }
        }
    }

    #[cfg(not(all(target_os = "android", not(feature = "v8_android_log_stdout"))))]
    impl fmt::Write for StdoutStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.inner.write_str(s)
        }
    }

    pub struct StderrStream {
        inner: OFStream,
        mutex_guard_: MutexGuard,
    }

    impl StderrStream {
        pub fn new() -> StderrStream {
            let stderr_file = unsafe { libc::stderr };
            StderrStream {
                inner: OFStream::new(stderr_file),
                mutex_guard_: MutexGuard::new(get_stdout_mutex()),
            }
        }
    }

    impl fmt::Write for StderrStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.inner.write_str(s)
        }
    }

    struct MutexGuard {
        mutex: &'static Mutex<()>,
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl MutexGuard {
        fn new(mutex: &'static Mutex<()>) -> Self {
            let _lock = mutex.lock().unwrap();
            MutexGuard { mutex, _lock }
        }
    }

    static STDOUT_MUTEX: LazyStaticMutex = LazyStaticMutex::new();

    fn get_stdout_mutex() -> &'static Mutex<()> {
        STDOUT_MUTEX.get()
    }

    struct LazyStaticMutex {
        mutex: MaybeUninit<Mutex<()>>,
    }

    unsafe impl Sync for LazyStaticMutex {}

    impl LazyStaticMutex {
        const fn new() -> Self {
            LazyStaticMutex {
                mutex: MaybeUninit::uninit(),
            }
        }

        fn get(&'static self) -> &'static Mutex<()> {
            unsafe {
                //Only initialize once
                static INIT: std::sync::Once = std::sync::Once::new();
                INIT.call_once(|| {
                    (self.mutex.as_ptr() as *mut Mutex<()>).write(Mutex::new(()));
                });
                self.mutex.assume_init_ref()
            }
        }
    }

    pub struct AsUC16 {
        pub value: u16,
    }

    impl AsUC16 {
        pub fn new(v: u16) -> AsUC16 {
            AsUC16 { value: v }
        }
    }

    pub struct AsUC32 {
        pub value: i32,
    }

    impl AsUC32 {
        pub fn new(v: i32) -> AsUC32 {
            AsUC32 { value: v }
        }
    }

    pub struct AsReversiblyEscapedUC16 {
        pub value: u16,
    }

    impl AsReversiblyEscapedUC16 {
        pub fn new(v: u16) -> AsReversiblyEscapedUC16 {
            AsReversiblyEscapedUC16 { value: v }
        }
    }

    pub struct AsEscapedUC16ForJSON {
        pub value: u16,
    }

    impl AsEscapedUC16ForJSON {
        pub fn new(v: u16) -> AsEscapedUC16ForJSON {
            AsEscapedUC16ForJSON { value: v }
        }
    }

    pub struct AsHex {
        pub value: u64,
        pub min_width: u8,
        pub with_prefix: bool,
    }

    impl AsHex {
        pub fn new(v: u64, min_width: u8, with_prefix: bool) -> AsHex {
            AsHex {
                value: v,
                min_width,
                with_prefix,
            }
        }

        pub fn address(a: usize) -> AsHex {
            AsHex::new(a as u64, kSystemPointerHexDigits as u8, true)
        }
    }

    pub struct AsHexBytes {
        pub value: u64,
        pub min_bytes: u8,
        pub byte_order: ByteOrder,
    }

    impl AsHexBytes {
        pub fn new(v: u64, min_bytes: u8, byte_order: ByteOrder) -> AsHexBytes {
            AsHexBytes {
                value: v,
                min_bytes,
                byte_order,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum ByteOrder {
        kLittleEndian,
        kBigEndian,
    }

    pub struct PrintIteratorRange<T, I>
    where
        T: Iterator<Item = I>,
        I: fmt::Display,
    {
        start: T,
        end: Option<I>,
        separator: &'static str,
        start_bracket: &'static str,
        end_bracket: &'static str,
    }

    impl<T, I> PrintIteratorRange<T, I>
    where
        T: Iterator<Item = I>,
        I: fmt::Display,
    {
        pub fn new(start: T, end: Option<I>) -> Self {
            PrintIteratorRange {
                start,
                end,
                separator: ", ",
                start_bracket: "[",
                end_bracket: "]",
            }
        }

        pub fn without_brackets(mut self) -> Self {
            self.start_bracket = "";
            self.end_bracket = "";
            self
        }

        pub fn with_separator(mut self, new_separator: &'static str) -> Self {
            self.separator = new_separator;
            self
        }
    }

    // Implement fmt::Display for PrintIteratorRange to allow printing
    impl<T, I> fmt::Display for PrintIteratorRange<T, I>
    where
        T: Iterator<Item = I>,
        I: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.start_bracket)?;
            let mut first = true;
            //This iterator needs to be consumed or it won't print. If end is not
            //specified, assume it's an infinite iterator.
            for item in self.start {
                if !first {
                    write!(f, "{}", self.separator)?;
                }
                write!(f, "{}", item)?;
                first = false;
            }
            write!(f, "{}", self.end_bracket)
        }
    }

    fn is_print(c: u16) -> bool {
        0x20 <= c && c <= 0x7E
    }

    fn is_space(c: u16) -> bool {
        (0x9 <= c && c <= 0xD) || c == 0x20
    }

    fn is_ok(c: u16) -> bool {
        (is_print(c) || is_space(c)) && c != '\\'
    }

    fn print_uc16<W: fmt::Write>(
        os: &mut W,
        c: u16,
        pred: fn(u16) -> bool,
    ) -> Result<(), fmt::Error> {
        if pred(c) {
            write!(os, "{}", char::from_u32(c as u32).unwrap())
        } else if c <= 0xFF {
            write!(os, "\\x{:02x}", c)
        } else {
            write!(os, "\\u{:04x}", c)
        }
    }

    fn print_uc16_for_json<W: fmt::Write>(
        os: &mut W,
        c: u16,
        pred: fn(u16) -> bool,
    ) -> Result<(), fmt::Error> {
        if pred(c) {
            write!(os, "{}", char::from_u32(c as u32).unwrap())
        } else {
            write!(os, "\\u{:04x}", c)
        }
    }

    fn print_uc32<W: fmt::Write>(
        os: &mut W,
        c: i32,
        pred: fn(u16) -> bool,
    ) -> Result<(), fmt::Error> {
        if c <= V8String::kMaxUtf16CodeUnit as i32 {
            print_uc16(os, c as u16, pred)
        } else {
            write!(os, "\\u{{{:06x}}}", c)
        }
    }

    impl fmt::Display for AsReversiblyEscapedUC16 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            print_uc16(f, self.value, is_ok)
        }
    }

    impl fmt::Display for AsEscapedUC16ForJSON {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.value {
                '\n' as u16 => write!(f, "\\n"),
                '\r' as u16 => write!(f, "\\r"),
                '\t' as u16 => write!(f, "\\t"),
                '"' as u16 => write!(f, "\\\""),
                _ => print_uc16_for_json(f, self.value, is_ok),
            }
        }
    }

    impl fmt::Display for AsUC16 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            print_uc16(f, self.value, is_print)
        }
    }

    impl fmt::Display for AsUC32 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            print_uc32(f, self.value, is_print)
        }
    }

    impl fmt::Display for AsHex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.min_width == 0 && self.value == 0 {
                return Ok({});
            }
            if self.with_prefix {
                write!(f, "0x")?;
            }
            write!(f, "{:0>width$x}", self.value, width = self.min_width as usize)
        }
    }

    impl fmt::Display for AsHexBytes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut bytes = self.min_bytes;
            while bytes < std::mem::size_of::<u64>() as u8
                && (self.value >> (bytes * 8) != 0)
            {
                bytes += 1;
            }
            for b in 0..bytes {
                if b > 0 {
                    write!(f, " ")?;
                }
                let printed_byte = match self.byte_order {
                    ByteOrder::kLittleEndian => b,
                    ByteOrder::kBigEndian => bytes - b - 1,
                };
                let byte_value = (self.value >> (8 * printed_byte)) & 0xFF;
                write!(f, "{}", AsHex::new(byte_value, 2, false))?;
            }
            Ok(())
        }
    }
}

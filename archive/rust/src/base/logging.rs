use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::panic;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
//use libc; // Consider using `libc` crate if needed for low-level OS interactions
//use backtrace; // Consider using `backtrace` crate for stack trace generation
//use std::any::Any;

pub mod base {
    use super::*;

    static PRINT_STACK_TRACE: Mutex<Option<fn()>> = Mutex::new(None);
    static DCHECK_FUNCTION: Mutex<Option<fn(file: &str, line: i32, message: &str)>> =
        Mutex::new(Some(default_dcheck_handler));
    static FATAL_FUNCTION: Mutex<Option<fn(file: &str, line: i32, message: &str)>> =
        Mutex::new(None);
    static CONTROLLED_CRASHES_ARE_HARMLESS: AtomicBool = AtomicBool::new(false);
    static DCHECK_FAILURES_ARE_IGNORED: AtomicBool = AtomicBool::new(false);

    #[derive(Debug, PartialEq, Eq)]
    pub enum OOMType {
        kProcess,
        kJavaScript,
    }

    fn pretty_print_char(ch: i32) -> String {
        match ch {
            0 => String::from("\\0"),
            39 => String::from("'"),
            92 => String::from("\\\\"),
            7 => String::from("\\a"),
            8 => String::from("\\b"),
            12 => String::from("\\f"),
            10 => String::from("\\n"),
            13 => String::from("\\r"),
            9 => String::from("\\t"),
            11 => String::from("\\v"),
            _ => {
                if (ch as u8).is_ascii_graphic() {
                    format!("'{}'", char::from_u32(ch as u32).unwrap())
                } else {
                    format!("\\x{:x}", ch)
                }
            }
        }
    }

    fn default_dcheck_handler(file: &str, line: i32, message: &str) {
        if cfg!(debug_assertions) {
            unsafe { V8_Fatal(file.as_ptr() as *const i8, line, format!("Debug check failed: {}", message).as_ptr() as *const i8) };
        } else {
            unsafe { V8_Fatal("Debug check failed: %s.".as_ptr() as *const i8, message.as_ptr() as *const i8) };
        }
    }

    pub fn set_print_stack_trace(print_stack_trace: Option<fn()>) {
        let mut guard = PRINT_STACK_TRACE.lock().unwrap();
        *guard = print_stack_trace;
    }

    pub fn set_dcheck_function(
        dcheck_function: Option<fn(file: &str, line: i32, message: &str)>,
    ) {
        let mut guard = DCHECK_FUNCTION.lock().unwrap();
        *guard = dcheck_function.or(Some(default_dcheck_handler));
    }

    pub fn set_fatal_function(
        fatal_function: Option<fn(file: &str, line: i32, message: &str)>,
    ) {
        let mut guard = FATAL_FUNCTION.lock().unwrap();
        *guard = fatal_function;
    }

    pub fn fatal_oom(oom_type: OOMType, msg: &str) {
        let type_str = match oom_type {
            OOMType::kProcess => "process",
            OOMType::kJavaScript => "JavaScript",
        };

        eprintln!("\n\n#\n# Fatal {} out of memory: {}\n#", type_str, msg);

        if let Some(print_stack_trace) = *PRINT_STACK_TRACE.lock().unwrap() {
            print_stack_trace();
        }
        std::io::Write::flush(&mut std::io::stderr()).unwrap();

        if cfg!(feature = "v8_fuzzilli") {
            // When fuzzing, we generally want to ignore OOM failures.
            // It's important that we exit with a non-zero exit status here so that the
            // fuzzer treats it as a failed execution.
            process::exit(1);
        } else {
            abort();
        }
    }

    pub fn controlled_crashes_are_harmless() -> bool {
        CONTROLLED_CRASHES_ARE_HARMLESS.load(Ordering::Relaxed)
    }

    pub fn set_controlled_crashes_are_harmless(value: bool) {
        CONTROLLED_CRASHES_ARE_HARMLESS.store(value, Ordering::Relaxed);
    }

    pub fn dcheck_failures_are_ignored() -> bool {
        DCHECK_FAILURES_ARE_IGNORED.load(Ordering::Relaxed)
    }

    pub fn set_dcheck_failures_are_ignored(value: bool) {
        DCHECK_FAILURES_ARE_IGNORED.store(value, Ordering::Relaxed);
    }

    pub fn print_err(args: fmt::Arguments) {
        std::io::Write::write_fmt(&mut std::io::stderr(), args).unwrap();
    }

    pub fn snprintf(buffer: &mut [u8], format: &str, args: fmt::Arguments) -> Result<usize, fmt::Error> {
        let format_cstr = CString::new(format).unwrap();
        let buffer_ptr = buffer.as_mut_ptr() as *mut i8;
        let buffer_size = buffer.len();

        // This closure captures the arguments and performs the formatting using snprintf.
        let result = {
            use std::fmt::Write;
            struct BufWriter<'a> {
                buf: &'a mut [u8],
                pos: usize,
            }

            impl<'a> BufWriter<'a> {
                fn new(buf: &'a mut [u8]) -> Self {
                    BufWriter { buf, pos: 0 }
                }
            }

            impl<'a> Write for BufWriter<'a> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    if self.pos >= self.buf.len() {
                        return Err(fmt::Error);
                    }
                    let bytes = s.as_bytes();
                    let remaining = self.buf.len() - self.pos;
                    let len = std::cmp::min(bytes.len(), remaining);
                    self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
                    self.pos += len;
                    Ok(())
                }
            }

            let mut writer = BufWriter::new(buffer);
            writer.write_fmt(args).map(|_| writer.pos)
        };

        // Ensure null termination, handling potential out-of-bounds.
        if let Ok(len) = result {
            if len < buffer_size {
                buffer[len] = 0;
            } else if buffer_size > 0 {
                buffer[buffer_size - 1] = 0;
            }
        }

        result
    }

    // Generic function to print an operand for checks.
    pub trait Printable {
        fn print(&self) -> String;
    }

    impl Printable for i32 {
        fn print(&self) -> String {
            format!("{}", self)
        }
    }

    impl Printable for i64 {
        fn print(&self) -> String {
            format!("{}", self)
        }
    }

    impl Printable for u32 {
        fn print(&self) -> String {
            format!("{}", self)
        }
    }

    impl Printable for u64 {
        fn print(&self) -> String {
            format!("{}", self)
        }
    }

    impl<T> Printable for *const T {
        fn print(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl Printable for char {
        fn print(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl Printable for i8 {
        fn print(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl Printable for u8 {
        fn print(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl<T> Printable for *mut T {
        fn print(&self) -> String {
            format!("{:p}", self)
        }
    }

    pub fn format_check_op<T: Printable, U: Printable>(left: T, right: U, op: &str) -> String {
        format!("{} {} {}", left.print(), op, right.print())
    }

    macro_rules! define_make_check_op_string {
        ($type:ty) => {
            impl Printable for $type {
                fn print(&self) -> String {
                    format!("{}", self)
                }
            }
        };
    }

    define_make_check_op_string!(i32);
    define_make_check_op_string!(i64);
    define_make_check_op_string!(u32);
    define_make_check_op_string!(u64);
    define_make_check_op_string!(*const std::ffi::c_void);
    define_make_check_op_string!(*mut std::ffi::c_void);

    //String conversion for chars and strings
    pub trait PrintCheckOperand<T> {
        fn print_operand(&self) -> String;
    }

    impl PrintCheckOperand<char> for char {
        fn print_operand(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl PrintCheckOperand<*const char> for *const char {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl PrintCheckOperand<*mut char> for *mut char {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl PrintCheckOperand<i8> for i8 {
        fn print_operand(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl PrintCheckOperand<*const i8> for *const i8 {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl PrintCheckOperand<*mut i8> for *mut i8 {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl PrintCheckOperand<u8> for u8 {
        fn print_operand(&self) -> String {
            pretty_print_char(*self as i32)
        }
    }

    impl PrintCheckOperand<*const u8> for *const u8 {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    impl PrintCheckOperand<*mut u8> for *mut u8 {
        fn print_operand(&self) -> String {
            format!("{:p}", self)
        }
    }

    #[macro_export]
    macro_rules! define_print_check_operand_char {
        ($type:ty) => {
            impl PrintCheckOperand<$type> for $type {
                fn print_operand(&self) -> String {
                    pretty_print_char(*self as i32)
                }
            }
            impl PrintCheckOperand<*const $type> for *const $type {
                fn print_operand(&self) -> String {
                    format!("{:p}", self)
                }
            }
            impl PrintCheckOperand<*mut $type> for *mut $type {
                fn print_operand(&self) -> String {
                    format!("{:p}", self)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! define_make_check_op_string_all {
        ($type:ty) => {
            impl PrintCheckOperand<$type> for $type {
                fn print_operand(&self) -> String {
                    format!("{}", self)
                }
            }
        };
    }

    define_make_check_op_string_all!(i32);
    define_make_check_op_string_all!(i64);
    define_make_check_op_string_all!(u32);
    define_make_check_op_string_all!(u64);
    define_make_check_op_string_all!(*const std::ffi::c_void);
    define_make_check_op_string_all!(*mut std::ffi::c_void);

    define_print_check_operand_char!(char);
    define_print_check_operand_char!(i8);
    define_print_check_operand_char!(u8);

    pub fn abort() -> ! {
        process::abort()
    }
}

mod failure_message {
    use super::*;

    pub struct FailureMessage {
        start_marker: usize,
        message: [u8; MESSAGE_BUFFER_SIZE],
        end_marker: usize,
    }

    const START_MARKER: usize = 0xdecade10;
    const END_MARKER: usize = 0xdecade11;
    const MESSAGE_BUFFER_SIZE: usize = 512;

    impl FailureMessage {
        pub fn new(format: &str, args: fmt::Arguments) -> Self {
            let mut message: [u8; MESSAGE_BUFFER_SIZE] = [0; MESSAGE_BUFFER_SIZE];
            base::snprintf(&mut message, format, args).unwrap();

            FailureMessage {
                start_marker: START_MARKER,
                message,
                end_marker: END_MARKER,
            }
        }
        pub fn message(&self) -> &str {
             unsafe {
                CStr::from_ptr(self.message.as_ptr() as *const i8)
                .to_str()
                .unwrap()
                }
        }
    }
}

extern "C" {
    fn fflush(stream: *mut std::ffi::c_void) -> i32;
    fn _exit(code: i32) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn V8_Fatal(file: *const c_char, line: i32, format: *const c_char, ...) {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    let file_str = if !file.is_null() {
        CStr::from_ptr(file).to_str().unwrap()
    } else {
        ""
    };

    let format_str = CStr::from_ptr(format).to_str().unwrap();

    let mut args_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
    let args = unsafe {
        let mut va_list: va_list::VaListImpl = va_list::VaListImpl::new(args_ptr);
        va_list
    };

    let message = failure_message::FailureMessage::new(format_str, std::fmt::Arguments::new(format_str, unsafe { args.as_va_list()}));

    if let Some(fatal_function) = *base::FATAL_FUNCTION.lock().unwrap() {
        fatal_function(file_str, line, message.message());
    }

    unsafe {
        fflush(std::io::stdout().as_mut_ptr() as *mut std::ffi::c_void);
        fflush(std::io::stderr().as_mut_ptr() as *mut std::ffi::c_void);
    }

    if base::controlled_crashes_are_harmless() {
        eprintln!(
            "\n\n#\n# Safely terminating process due to error in {}, line {}\n# ",
            file_str, line
        );
        eprint!("The following harmless error was encountered: ");
    } else {
        eprintln!("\n\n#\n# Fatal error in {}, line {}\n# ", file_str, line);
    }

    base::print_err(format_args!("{}", message.message()));

    eprintln!("\n#\n#\n#\n#FailureMessage Object: {:?}", &message);

    if let Some(print_stack_trace) = *base::PRINT_STACK_TRACE.lock().unwrap() {
        print_stack_trace();
    }

    unsafe {
        fflush(std::io::stderr().as_mut_ptr() as *mut std::ffi::c_void);
    }
    base::abort();
}

#[no_mangle]
pub unsafe extern "C" fn V8_Dcheck(file: *const c_char, line: i32, message: *const c_char) {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    let file_str = CStr::from_ptr(file).to_str().unwrap();
    let message_str = CStr::from_ptr(message).to_str().unwrap();

    if base::dcheck_failures_are_ignored() {
        eprintln!(
            "# Ignoring debug check failure in {}, line {}: {}\n",
            file_str, line, message_str
        );
        return;
    }

    if let Some(dcheck_function) = *base::DCHECK_FUNCTION.lock().unwrap() {
        dcheck_function(file_str, line, message_str);
    }
}

//Need this for fmt::Arguments::new()
mod va_list {
    use std::fmt;
    use std::any::Any;

    /// VaListImpl is a struct that holds a pointer to the va_list.
    /// It is used to pass the va_list to the formatter.
    /// The pointer is only valid for the duration of the call to the formatter.
    #[derive(Debug, Copy, Clone)]
    pub struct VaListImpl {
        ptr: *mut std::ffi::c_void,
    }

    impl VaListImpl {
        /// Creates a new VaListImpl from a pointer to a va_list.
        /// The pointer is only valid for the duration of the call to the formatter.
        pub unsafe fn new(ptr: *mut std::ffi::c_void) -> Self {
            VaListImpl { ptr }
        }

        /// Returns a pointer to the va_list.
        /// The pointer is only valid for the duration of the call to the formatter.
        pub unsafe fn as_va_list(&self) -> *mut std::ffi::c_void {
            self.ptr
        }
    }
}
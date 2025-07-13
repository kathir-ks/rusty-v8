// Converted from V8 C++ source files:
// Header: logging.h
// Implementation: logging.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    //use std::os::raw::c_char;
    use std::ffi::{CStr, CString};
    use std::fmt;
    use std::fmt::Write;
    use std::mem;
    use std::os::raw::c_int;
    use std::ptr;
    use std::slice;
    use std::str;
    use std::sync::Mutex;

    pub use crate::V8_NOINLINE;

    #[cfg(debug_assertions)]
    macro_rules! FATAL {
        ($file:expr, $line:expr, $format:expr, $($arg:tt)*) => {
            unsafe { V8_Fatal($file as *const str as *const i8, $line, format!($format, $($arg)*).as_ptr() as *const i8) }
        };
        ($format:expr, $($arg:tt)*) => {
            unsafe { V8_Fatal("unknown" as *const str as *const i8, 0, format!($format, $($arg)*).as_ptr() as *const i8) }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! FATAL {
        ($format:expr, $($arg:tt)*) => {
            unsafe { V8_Fatal(format!($format, $($arg)*).as_ptr() as *const i8) }
        };
    }

    macro_rules! UNIMPLEMENTED {
        () => {
            FATAL!("{}", kUnimplementedCodeMessage);
        };
    }

    macro_rules! UNREACHABLE {
        () => {
            FATAL!("{}", kUnreachableCodeMessage);
        };
    }

    macro_rules! CONSTEXPR_UNREACHABLE {
        () => {
            panic!("constexpr unreachable");
        };
    }

    macro_rules! CHECK_WITH_MSG {
        ($condition:expr, $message:expr) => {
            if !($condition) {
                CHECK_FAILED_HANDLER!($message);
            }
        };
    }

    macro_rules! CHECK {
        ($condition:expr) => {
            CHECK_WITH_MSG!($condition, stringify!($condition));
        };
    }

    macro_rules! CHECK_FAILED_HANDLER {
        ($message:expr) => {
            FATAL!("Check failed: {}.", $message);
        };
    }

    macro_rules! CHECK_OP {
        ($name:ident, $op:tt, $lhs:expr, $rhs:expr) => {
            if let Some(msg) = CheckOpImpl::<$lhs, $rhs>($lhs, $rhs, stringify!($lhs $op $rhs)) {
                FATAL!("Check failed: {}.", msg);
            }
        };
    }

    macro_rules! CHECK_EQ {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(EQ, ==, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_NE {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(NE, !=, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_LE {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(LE, <=, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_LT {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(LT, <, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_GE {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(GE, >=, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_GT {
        ($lhs:expr, $rhs:expr) => {
            CHECK_OP!(GT, >, $lhs, $rhs);
        };
    }

    macro_rules! CHECK_NULL {
        ($val:expr) => {
            CHECK!(($val).is_null());
        };
    }

    macro_rules! CHECK_NOT_NULL {
        ($val:expr) => {
            CHECK!(!($val).is_null());
        };
    }

    macro_rules! CHECK_IMPLIES {
        ($lhs:expr, $rhs:expr) => {
            CHECK_WITH_MSG!(!($lhs) || ($rhs), stringify!($lhs implies $rhs));
        };
    }

    macro_rules! CHECK_BOUNDS {
        ($index:expr, $limit:expr) => {
            CHECK_LT!(($index as u64), ($limit as u64));
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_WITH_MSG {
        ($condition:expr, $message:expr) => {
            if !($condition) {
                V8_Dcheck!("unknown" as *const str as *const i8, 0, $message as *const str as *const i8);
            }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_WITH_MSG {
        ($condition:expr, $message:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK {
        ($condition:expr) => {
            DCHECK_WITH_MSG!($condition, stringify!($condition));
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK {
        ($condition:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_OP {
        ($name:ident, $op:tt, $lhs:expr, $rhs:expr) => {
            if let Some(msg) = CheckOpImpl::<$lhs, $rhs>($lhs, $rhs, stringify!($lhs $op $rhs)) {
                V8_Dcheck!("unknown" as *const str as *const i8, 0, msg as *const str as *const i8);
            }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_OP {
        ($name:ident, $op:tt, $lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_EQ {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(EQ, ==, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_EQ {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_NE {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(NE, !=, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_NE {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_LE {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(LE, <=, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_LE {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_LT {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(LT, <, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_LT {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_GE {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(GE, >=, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_GE {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_GT {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_OP!(GT, >, $lhs, $rhs);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_GT {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_NULL {
        ($val:expr) => {
            DCHECK!(($val).is_null());
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_NULL {
        ($val:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_NOT_NULL {
        ($val:expr) => {
            DCHECK!(!($val).is_null());
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_NOT_NULL {
        ($val:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_IMPLIES {
        ($lhs:expr, $rhs:expr) => {
            DCHECK_WITH_MSG!(!($lhs) || ($rhs), stringify!($lhs implies $rhs));
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_IMPLIES {
        ($lhs:expr, $rhs:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! DCHECK_BOUNDS {
        ($index:expr, $limit:expr) => {
            DCHECK_LT!(($index as u64), ($limit as u64));
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! DCHECK_BOUNDS {
        ($index:expr, $limit:expr) => {
            unsafe {std::hint::unreachable_unchecked()};
        };
    }

    extern "C" {
        pub fn abort() -> !;
        pub fn fflush(stream: *mut libc::FILE) -> c_int;
    }

    pub static kUnimplementedCodeMessage: &str = "unimplemented code";
    pub static kUnreachableCodeMessage: &str = "unreachable code";

    pub struct CheckMessageStream {
        buffer: String,
    }

    impl CheckMessageStream {
        pub fn new() -> CheckMessageStream {
            CheckMessageStream {
                buffer: String::new(),
            }
        }
    }

    impl fmt::Write for CheckMessageStream {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    impl fmt::Display for CheckMessageStream {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.buffer)
        }
    }

    pub type PrintStackTraceFn = extern "C" fn();
    pub type DcheckFunction = extern "C" fn(file: *const i8, line: c_int, message: *const i8);
    pub type FatalFunction = extern "C" fn(file: *const i8, line: c_int, message: *const i8);

    lazy_static::lazy_static! {
        static ref PRINT_STACK_TRACE_FN: Mutex<Option<StackTraceFn>> = Mutex::new(None);
        static ref DCHECK_FUNCTION: Mutex<DcheckFunction> = Mutex::new(default_dcheck_handler);
        static ref FATAL_FUNCTION: Mutex<Option<FatalFunction>> = Mutex::new(None);
    }

    pub fn set_print_stack_trace(print_stack_trace: Option<StackTraceFn>) {
        let mut guard = PRINT_STACK_TRACE_FN.lock().unwrap();
        *guard = print_stack_trace;
    }

    pub fn set_dcheck_function(dcheck_function: Option<DcheckFunction>) {
        let mut guard = DCHECK_FUNCTION.lock().unwrap();
        *guard = dcheck_function.unwrap_or(default_dcheck_handler);
    }

    pub fn set_fatal_function(fatal_function: Option<FatalFunction>) {
        let mut guard = FATAL_FUNCTION.lock().unwrap();
        *guard = fatal_function;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OOMType {
        kJavaScript,
        kProcess,
    }

    pub fn fatal_oom(oom_type: OOMType, msg: &str) -> ! {
        let type_str = match oom_type {
            OOMType::kProcess => "process",
            OOMType::kJavaScript => "JavaScript",
        };

        eprintln!("\n\n#\n# Fatal {} out of memory: {}\n#", type_str, msg);

        let print_stack_trace_fn = PRINT_STACK_TRACE_FN.lock().unwrap();
        if let Some(print_stack_trace) = *print_stack_trace_fn {
            print_stack_trace();
        }

        unsafe {
            fflush(libc::stderr);
        }
        #[cfg(V8_FUZZILLI)]
        {
            unsafe{libc::_exit(1)};
        }
        #[cfg(not(V8_FUZZILLI))]
        {
            unsafe{abort()};
        }
    }

    pub fn default_dcheck_handler(file: *const i8, line: c_int, message: *const i8) {
        #[cfg(debug_assertions)]
        {
            unsafe { V8_Fatal(file, line, format!("Debug check failed: {}", CStr::from_ptr(message).to_str().unwrap()).as_ptr() as *const i8); }
        }
        #[cfg(not(debug_assertions))]
        {
             unsafe { V8_Fatal(format!("Debug check failed: {}", CStr::from_ptr(message).to_str().unwrap()).as_ptr() as *const i8); }
        }
    }

    extern "C" {
        #[cfg(debug_assertions)]
        pub fn V8_Fatal(file: *const i8, line: c_int, format: *const i8) -> !;
        #[cfg(not(debug_assertions))]
        pub fn V8_Fatal(format: *const i8) -> !;
        pub fn V8_Dcheck(file: *const i8, line: c_int, message: *const i8);
    }

    type StackTraceFn = extern "C" fn();

    fn pretty_print_char(ch: i32) -> String {
        match ch {
            0 => "\\0".to_string(),
            39 => "'".to_string(),
            92 => "\\\\".to_string(),
            7 => "\\a".to_string(),
            8 => "\\b".to_string(),
            12 => "\\f".to_string(),
            10 => "\\n".to_string(),
            13 => "\\r".to_string(),
            9 => "\\t".to_string(),
            11 => "\\v".to_string(),
            _ => {
                if ch >= 32 && ch <= 126 {
                    format!("'{}'", ch as u8 as char)
                } else {
                    format!("\\x{:x}", ch)
                }
            }
        }
    }

    pub fn print_check_operand<T: fmt::Display>(val: T) -> String {
        format!("{}", val)
    }

    pub fn make_check_op_string<Lhs: fmt::Display, Rhs: fmt::Display>(lhs: Lhs, rhs: Rhs, msg: &str) -> String {
        let lhs_str = lhs.to_string();
        let rhs_str = rhs.to_string();

        let mut ss = String::new();
        write!(&mut ss, "{}", msg).unwrap();

        let k_max_inline_length: usize = 50;
        if lhs_str.len() <= k_max_inline_length && rhs_str.len() <= k_max_inline_length {
            write!(&mut ss, " ({} vs. {})", lhs_str, rhs_str).unwrap();
        } else {
            write!(&mut ss, "\n   {}\n vs.\n   {}\n", lhs_str, rhs_str).unwrap();
        }

        ss
    }

    pub fn check_eq_impl<Lhs: PartialEq, Rhs: PartialEq>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs == rhs
    }

    pub fn check_ne_impl<Lhs: PartialEq, Rhs: PartialEq>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs != rhs
    }

    pub fn check_le_impl<Lhs: PartialOrd, Rhs: PartialOrd>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs <= rhs
    }

    pub fn check_lt_impl<Lhs: PartialOrd, Rhs: PartialOrd>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs < rhs
    }

    pub fn check_ge_impl<Lhs: PartialOrd, Rhs: PartialOrd>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs >= rhs
    }

    pub fn check_gt_impl<Lhs: PartialOrd, Rhs: PartialOrd>(lhs: Lhs, rhs: Rhs) -> bool {
        lhs > rhs
    }

    pub fn CheckOpImpl<Lhs: fmt::Display, Rhs: fmt::Display>(lhs: Lhs, rhs: Rhs, msg: &str) -> Option<String> {
        let cmp =  make_check_op_string(lhs, rhs, msg);
            Some(cmp)
    }

    pub trait Printable {
        fn to_string(&self) -> String;
    }

    impl Printable for i32 {
        fn to_string(&self) -> String {
            format!("{}", self)
        }
    }
}

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::{io, mem, str};

extern crate libc;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "solaris"
))]
const HAVE_EXECINFO_H: bool = true;

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "solaris"
)))]
const HAVE_EXECINFO_H: bool = false;

#[cfg(HAVE_EXECINFO_H)]
extern "C" {
    fn backtrace(array: *mut *mut c_void, size: c_int) -> c_int;
    fn backtrace_symbols(array: *mut *mut c_void, size: c_int) -> *mut *mut c_char;
    fn backtrace_symbols_fd(array: *mut *mut c_void, size: c_int, fd: c_int);
    fn __cxa_demangle(
        mangled_name: *const c_char,
        output_buffer: *mut c_char,
        length: *mut usize,
        status: *mut c_int,
    ) -> *mut c_char;
}

pub mod base {
    pub mod debug {
        use super::super::*;
        use std::io::Write;

        pub mod internal {
            use super::super::*;

            pub fn itoa_r(i: isize, buf: &mut [u8], base: i32, padding: usize) -> Option<&mut [u8]> {
                let sz = buf.len();
                let mut n = 1;
                if n > sz {
                    return None;
                }

                if base < 2 || base > 16 {
                    buf[0] = 0;
                    return None;
                }

                let mut start = 0;
                let mut j = i as usize;

                if i < 0 && base == 10 {
                    j = (-(i + 1)) as usize + 1;
                    if n + 1 > sz {
                        buf[0] = 0;
                        return None;
                    }
                    buf[start] = b'-';
                    start += 1;
                    n += 1;
                }

                let mut ptr = start;
                loop {
                    if n + 1 > sz {
                        buf[0] = 0;
                        return None;
                    }

                    buf[ptr] =
                        b"0123456789abcdef"[(j % (base as usize)) as usize];
                    ptr += 1;
                    j /= base as usize;

                    if padding > 0 {
                    } else if j == 0 {
                        break;
                    }
                }
                buf[ptr] = 0;
                let mut ptr = ptr - 1;

                while ptr > start {
                    let ch = buf[ptr];
                    buf[ptr] = buf[start];
                    buf[start] = ch;
                    start += 1;
                    ptr -= 1;
                }

                Some(buf)
            }
        }

        static IN_SIGNAL_HANDLER: AtomicBool = AtomicBool::new(false);
        static DUMP_STACK_IN_SIGNAL_HANDLER: Mutex<bool> = Mutex::new(true);

        const MANGLED_SYMBOL_PREFIX: &str = "_Z";
        const SYMBOL_CHARACTERS: &str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

        #[cfg(HAVE_EXECINFO_H)]
        fn demangle_symbols(text: &mut String) {
            let mut search_from = 0;
            while search_from < text.len() {
                let mangled_start = text.find(MANGLED_SYMBOL_PREFIX);
                if mangled_start.is_none() {
                    break;
                }
                let mangled_start = mangled_start.unwrap();

                let mangled_end = text
                    .chars()
                    .enumerate()
                    .skip(mangled_start)
                    .find_map(|(i, c)| {
                        if !SYMBOL_CHARACTERS.contains(c) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(text.len());

                let mangled_symbol = &text[mangled_start..mangled_end];

                let mut status: c_int = 0;
                let demangled_ptr = unsafe {
                    __cxa_demangle(
                        CString::new(mangled_symbol).unwrap().as_ptr(),
                        ptr::null_mut(),
                        ptr::null_mut(),
                        &mut status,
                    )
                };

                if status == 0 {
                    let demangled_symbol = unsafe { CStr::from_ptr(demangled_ptr) }
                        .to_string_lossy()
                        .into_owned();
                    unsafe {
                        libc::free(demangled_ptr as *mut c_void);
                    }
                    text.replace_range(mangled_start..mangled_end, &demangled_symbol);
                    search_from = mangled_start + demangled_symbol.len();
                } else {
                    search_from = mangled_start + 2;
                }
            }
        }

        trait BacktraceOutputHandler {
            fn handle_output(&mut self, output: &str);
            fn output_file_descriptor(&self) -> i32 {
                0
            }
        }

        fn output_pointer<T: BacktraceOutputHandler>(pointer: *mut c_void, handler: &mut T) {
            let mut buf: [u8; 17] = [0; 17];
            handler.handle_output("0x");
            internal::itoa_r(pointer as isize, &mut buf, 16, 12);

            let end = buf.iter().position(|&x| x == 0).unwrap_or(buf.len());
            let s = str::from_utf8(&buf[..end]).unwrap();

            handler.handle_output(s);
        }

        #[cfg(HAVE_EXECINFO_H)]
        fn process_backtrace<T: BacktraceOutputHandler>(trace: &[*mut c_void], handler: &mut T) {
            handler.handle_output("\n");
            handler.handle_output("==== C stack trace ===============================\n");
            handler.handle_output("\n");

            let mut printed = false;
            if !IN_SIGNAL_HANDLER.load(Ordering::Relaxed) {
                let trace_symbols_ptr = unsafe {
                    backtrace_symbols(
                        trace.as_ptr() as *mut *mut c_void,
                        trace.len() as c_int,
                    )
                };

                if !trace_symbols_ptr.is_null() {
                    let trace_symbols = unsafe {
                        let mut len: usize = 0;
                        while !(*(trace_symbols_ptr.add(len))).is_null() {
                            len += 1;
                        }
                        slice::from_raw_parts(trace_symbols_ptr, len)
                    };

                    for i in 0..trace.len() {
                        let symbol_ptr = unsafe { *trace_symbols.get_unchecked(i) };
                        if !symbol_ptr.is_null() {
                            let c_str = unsafe { CStr::from_ptr(symbol_ptr) };
                            let mut trace_symbol = c_str.to_string_lossy().into_owned();
                            demangle_symbols(&mut trace_symbol);
                            handler.handle_output("    ");
                            handler.handle_output(&trace_symbol);
                            handler.handle_output("\n");
                        }
                    }

                    unsafe { libc::free(trace_symbols_ptr as *mut c_void) };

                    printed = true;
                }
            } else if handler.output_file_descriptor() != 0 {
                unsafe {
                    backtrace_symbols_fd(
                        trace.as_ptr() as *mut *mut c_void,
                        trace.len() as c_int,
                        handler.output_file_descriptor(),
                    );
                }
                printed = true;
            }

            if !printed {
                for i in 0..trace.len() {
                    handler.handle_output(" [");
                    output_pointer(trace[i], handler);
                    handler.handle_output("]\n");
                }
            }
        }

        fn print_to_stderr(output: &str) {
            let _ = unsafe {
                libc::write(
                    libc::STDERR_FILENO,
                    output.as_ptr() as *const c_void,
                    output.len(),
                )
            };
        }

        extern "C" fn stack_dump_signal_handler(
            signal: c_int,
            info: *mut libc::siginfo_t,
            _void_context: *mut c_void,
        ) {
            IN_SIGNAL_HANDLER.store(true, Ordering::Relaxed);
            print_to_stderr("Received signal ");

            let mut buf = [0u8; 1024];
            internal::itoa_r(signal as isize, &mut buf, 10, 0);
            let end = buf.iter().position(|&x| x == 0).unwrap_or(buf.len());
            let s = str::from_utf8(&buf[..end]).unwrap();
            print_to_stderr(s);

            if signal == libc::SIGBUS {
                if !info.is_null() {
                    let info_ref = unsafe { &*info };
                    match info_ref.si_code {
                        libc::BUS_ADRALN => print_to_stderr(" BUS_ADRALN "),
                        libc::BUS_ADRERR => print_to_stderr(" BUS_ADRERR "),
                        libc::BUS_OBJERR => print_to_stderr(" BUS_OBJERR "),
                        _ => print_to_stderr(" <unknown> "),
                    }
                }
            } else if signal == libc::SIGFPE {
                if !info.is_null() {
                    let info_ref = unsafe { &*info };
                    match info_ref.si_code {
                        libc::FPE_FLTDIV => print_to_stderr(" FPE_FLTDIV "),
                        libc::FPE_FLTINV => print_to_stderr(" FPE_FLTINV "),
                        libc::FPE_FLTOVF => print_to_stderr(" FPE_FLTOVF "),
                        libc::FPE_FLTRES => print_to_stderr(" FPE_FLTRES "),
                        libc::FPE_FLTSUB => print_to_stderr(" FPE_FLTSUB "),
                        libc::FPE_FLTUND => print_to_stderr(" FPE_FLTUND "),
                        libc::FPE_INTDIV => print_to_stderr(" FPE_INTDIV "),
                        libc::FPE_INTOVF => print_to_stderr(" FPE_INTOVF "),
                        _ => print_to_stderr(" <unknown> "),
                    }
                }
            } else if signal == libc::SIGILL {
                if !info.is_null() {
                    let info_ref = unsafe { &*info };
                    match info_ref.si_code {
                        libc::ILL_BADSTK => print_to_stderr(" ILL_BADSTK "),
                        libc::ILL_COPROC => print_to_stderr(" ILL_COPROC "),
                        libc::ILL_ILLOPN => print_to_stderr(" ILL_ILLOPN "),
                        libc::ILL_ILLADR => print_to_stderr(" ILL_ILLADR "),
                        libc::ILL_ILLTRP => print_to_stderr(" ILL_ILLTRP "),
                        libc::ILL_PRVOPC => print_to_stderr(" ILL_PRVOPC "),
                        libc::ILL_PRVREG => print_to_stderr(" ILL_PRVREG "),
                        _ => print_to_stderr(" <unknown> "),
                    }
                }
            } else if signal == libc::SIGSEGV {
                if !info.is_null() {
                    let info_ref = unsafe { &*info };
                    match info_ref.si_code {
                        libc::SEGV_MAPERR => print_to_stderr(" SEGV_MAPERR "),
                        libc::SEGV_ACCERR => print_to_stderr(" SEGV_ACCERR "),
                        _ => print_to_stderr(" <unknown> "),
                    }
                }
            }

            if signal == libc::SIGBUS || signal == libc::SIGFPE || signal == libc::SIGILL
                || signal == libc::SIGSEGV
            {
                if !info.is_null() {
                    let info_ref = unsafe { &*info };
                    let si_addr = info_ref.si_addr as isize;
                    internal::itoa_r(si_addr, &mut buf, 16, 12);

                    let end = buf.iter().position(|&x| x == 0).unwrap_or(buf.len());
                    let s = str::from_utf8(&buf[..end]).unwrap();
                    print_to_stderr(s);
                }
            }
            print_to_stderr("\n");

            let dump_stack = *DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap();
            if dump_stack {
                StackTrace::new().print();
                print_to_stderr("[end of stack trace]\n");
            }

            unsafe {
                if libc::signal(signal, libc::SIG_DFL) == libc::SIG_ERR {
                    libc::_exit(1);
                }
            }
        }

        struct PrintBacktraceOutputHandler {}

        impl PrintBacktraceOutputHandler {
            fn new() -> Self {
                PrintBacktraceOutputHandler {}
            }
        }

        impl BacktraceOutputHandler for PrintBacktraceOutputHandler {
            fn handle_output(&mut self, output: &str) {
                print_to_stderr(output);
            }

            fn output_file_descriptor(&self) -> i32 {
                libc::STDERR_FILENO
            }
        }

        struct StreamBacktraceOutputHandler<'a> {
            os: &'a mut dyn Write,
        }

        impl<'a> StreamBacktraceOutputHandler<'a> {
            fn new(os: &'a mut dyn Write) -> Self {
                StreamBacktraceOutputHandler { os }
            }
        }

        impl<'a> BacktraceOutputHandler for StreamBacktraceOutputHandler<'a> {
            fn handle_output(&mut self, output: &str) {
                let _ = self.os.write_all(output.as_bytes());
            }
        }

        fn warm_up_backtrace() {
            let _stack_trace = StackTrace::new();
        }

        pub fn enable_in_process_stack_dumping() -> bool {
            let mut sigpipe_action: libc::sigaction = unsafe { mem::zeroed() };
            sigpipe_action.sa_sigaction = libc::SIG_IGN;
            unsafe { libc::sigemptyset(&mut sigpipe_action.sa_mask) };

            let success = unsafe { libc::sigaction(libc::SIGPIPE, &sigpipe_action, ptr::null_mut()) == 0 };

            warm_up_backtrace();

            let mut action: libc::sigaction = unsafe { mem::zeroed() };
            action.sa_flags = libc::SA_RESETHAND | libc::SA_SIGINFO | libc::SA_ONSTACK;
            action.sa_sigaction = stack_dump_signal_handler as usize;
            unsafe { libc::sigemptyset(&mut action.sa_mask) };

            let mut success = success
                & (unsafe { libc::sigaction(libc::SIGILL, &action, ptr::null_mut()) == 0 });
            success = success
                & (unsafe { libc::sigaction(libc::SIGABRT, &action, ptr::null_mut()) == 0 });
            success = success
                & (unsafe { libc::sigaction(libc::SIGFPE, &action, ptr::null_mut()) == 0 });
            success = success
                & (unsafe { libc::sigaction(libc::SIGBUS, &action, ptr::null_mut()) == 0 });
            success = success
                & (unsafe { libc::sigaction(libc::SIGSEGV, &action, ptr::null_mut()) == 0 });
            success = success
                & (unsafe { libc::sigaction(libc::SIGSYS, &action, ptr::null_mut()) == 0 });

            *DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap() = true;

            success
        }

        pub fn disable_signal_stack_dump() {
            *DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap() = false;
        }

        const MAX_FRAMES: usize = 256;

        pub struct StackTrace {
            trace: [*mut c_void; MAX_FRAMES],
            count: usize,
        }

        impl StackTrace {
            pub fn new() -> Self {
                let mut trace: [*mut c_void; MAX_FRAMES] = [ptr::null_mut(); MAX_FRAMES];

                #[cfg(HAVE_EXECINFO_H)]
                let count = unsafe {
                    backtrace(
                        trace.as_mut_ptr() as *mut *mut c_void,
                        trace.len() as i32,
                    ) as usize
                };

                #[cfg(not(HAVE_EXECINFO_H))]
                let count = 0;

                StackTrace { trace, count }
            }

            pub fn print(&self) {
                #[cfg(HAVE_EXECINFO_H)]
                {
                    let mut handler = PrintBacktraceOutputHandler::new();
                    process_backtrace(&self.trace[..self.count], &mut handler);
                }
            }

            pub fn output_to_stream(&self, os: &mut dyn Write) {
                #[cfg(HAVE_EXECINFO_H)]
                {
                    let mut handler = StreamBacktraceOutputHandler::new(os);
                    process_backtrace(&self.trace[..self.count], &mut handler);
                }
            }
        }
    }
}
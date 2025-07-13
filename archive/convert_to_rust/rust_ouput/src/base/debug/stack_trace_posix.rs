// Converted from V8 C++ source files:
// Header: N/A
// Implementation: stack_trace_posix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod debug {
    use std::ffi::CString;
    use std::io::Write;
    use std::mem;
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;
    use std::slice;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    const kMangledSymbolPrefix: &str = "_Z";
    const kSymbolCharacters: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

    #[cfg(target_os = "linux")]
    extern "C" {
        fn backtrace(array: *mut *mut c_void, size: c_int) -> c_int;
        fn backtrace_symbols(
            buffer: *const *mut c_void,
            size: c_int,
        ) -> *mut *mut c_char;
        fn backtrace_symbols_fd(buffer: *const *mut c_void, size: c_int, fd: c_int);
        #[link_name = "__cxa_demangle"]
        fn cxa_demangle(
            mangled_name: *const c_char,
            output_buffer: *mut c_char,
            length: *mut usize,
            status: *mut c_int,
        ) -> *mut c_char;
    }

    #[derive(Debug)]
    pub enum StackTraceError {
        BacktraceFailed,
        DemangleFailed,
        IOError(std::io::Error),
    }

    impl From<std::io::Error> for StackTraceError {
        fn from(err: std::io::Error) -> Self {
            StackTraceError::IOError(err)
        }
    }

    mod internal {
        use std::os::raw::c_char;

        pub fn itoa_r(
            i: isize,
            buf: &mut [c_char],
            base: i32,
            padding: usize,
        ) -> Option<*mut c_char> {
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

            let mut negative = false;
            if i < 0 && base == 10 {
                j = (-(i + 1)) as usize + 1;

                if n + 1 > sz {
                    buf[0] = 0;
                    return None;
                }
                buf[start] = '-' as c_char;
                start += 1;
                n += 1;
                negative = true;
            }

            let mut ptr = start;
            let mut padding_left = padding;
            loop {
                if n + 1 > sz {
                    buf[0] = 0;
                    return None;
                }
                let digit = (j % (base as usize)) as u8;
                buf[ptr] = match digit {
                    0..=9 => (b'0' + digit) as c_char,
                    _ => (b'a' + digit - 10) as c_char,
                };

                j /= base as usize;
                ptr += 1;
                n += 1;
                if padding_left > 0 {
                    padding_left -= 1;
                }

                if j == 0 && padding_left == 0 {
                    break;
                }
            }

            buf[ptr] = 0;

            let mut start_rev = if negative { 1 } else { 0 };
            let mut end_rev = ptr - 1;

            while end_rev > start_rev {
                buf.swap(start_rev, end_rev);
                start_rev += 1;
                end_rev -= 1;
            }

            Some(buf.as_mut_ptr())
        }
    }

    static IN_SIGNAL_HANDLER: AtomicBool = AtomicBool::new(false);
    static DUMP_STACK_IN_SIGNAL_HANDLER: Mutex<bool> = Mutex::new(true);

    pub struct StackTrace {
        trace: [*mut c_void; 256],
        count: usize,
    }

    impl StackTrace {
        pub fn new() -> StackTrace {
            let mut trace: [*mut c_void; 256] = [ptr::null_mut(); 256];
            let count = unsafe {
                #[cfg(target_os = "linux")]
                {
                    backtrace(trace.as_mut_ptr(), trace.len() as i32) as usize
                }
                #[cfg(not(target_os = "linux"))]
                {
                    0
                }
            };
            StackTrace { trace, count }
        }

        pub fn print(&self) {
            let handler = PrintBacktraceOutputHandler {};
            self.process_backtrace(&handler);
        }

        pub fn output_to_stream(&self, os: &mut dyn Write) {
            let handler = StreamBacktraceOutputHandler { os };
            self.process_backtrace(&handler);
        }

        fn process_backtrace(&self, handler: &dyn BacktraceOutputHandler) {
            handler.handle_output("\n");
            handler.handle_output("==== C stack trace ===============================\n");
            handler.handle_output("\n");

            let printed = if !IN_SIGNAL_HANDLER.load(Ordering::Relaxed) {
                #[cfg(target_os = "linux")]
                {
                    let trace_symbols = unsafe {
                        backtrace_symbols(self.trace.as_ptr(), self.count as i32)
                    };

                    if !trace_symbols.is_null() {
                        let symbols = unsafe {
                            slice::from_raw_parts(trace_symbols, self.count)
                        };

                        for i in 0..self.count {
                            let symbol_ptr = unsafe { *symbols.as_ptr().add(i) };
                            if !symbol_ptr.is_null() {
                                let symbol = unsafe {
                                    std::ffi::CStr::from_ptr(symbol_ptr)
                                        .to_string_lossy()
                                        .into_owned()
                                };

                                let mut demangled_symbol = symbol.clone();
                                demangle_symbols(&mut demangled_symbol);

                                handler.handle_output("    ");
                                handler.handle_output(&demangled_symbol);
                                handler.handle_output("\n");
                            }
                        }
                        unsafe { libc::free(trace_symbols as *mut c_void) }; //Memory allocated by backtrace_symbols needs to be freed
                        true
                    } else {
                        false
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    false
                }
            } else if handler.output_file_descriptor() != 0 {
                #[cfg(target_os = "linux")]
                {
                    unsafe {
                        backtrace_symbols_fd(
                            self.trace.as_ptr(),
                            self.count as i32,
                            handler.output_file_descriptor(),
                        );
                    }
                    true
                }
                #[cfg(not(target_os = "linux"))]
                {
                    false
                }
            } else {
                false
            };

            if !printed {
                for i in 0..self.count {
                    handler.handle_output(" [");
                    output_pointer(self.trace[i], handler);
                    handler.handle_output("]\n");
                }
            }
        }
    }

    trait BacktraceOutputHandler {
        fn handle_output(&self, output: &str);
        fn output_file_descriptor(&self) -> i32 {
            0
        }
    }

    fn demangle_symbols(text: &mut String) {
        let mut search_from = 0;
        while search_from < text.len() {
            let mangled_start = text.find(kMangledSymbolPrefix);
            match mangled_start {
                Some(mangled_start) => {
                    let mangled_end = text[mangled_start..].find(|c: char| {
                        !kSymbolCharacters.contains(c)
                    });
                    match mangled_end {
                        Some(relative_mangled_end) => {
                            let mangled_end = mangled_start + relative_mangled_end;
                            let mangled_symbol =
                                text[mangled_start..mangled_end].to_string();

                            let status = Mutex::new(0);
                            let demangled_symbol = unsafe {
                                let mut status_lock = status.lock().unwrap();
                                let mangled_symbol_cstring =
                                    CString::new(mangled_symbol.clone()).unwrap();
                                let demangled_ptr = cxa_demangle(
                                    mangled_symbol_cstring.as_ptr(),
                                    ptr::null_mut(),
                                    ptr::null_mut(),
                                    &mut *status_lock,
                                );
                                if *status_lock == 0 && !demangled_ptr.is_null() {
                                    let demangled_cstr =
                                        std::ffi::CStr::from_ptr(demangled_ptr);
                                    let demangled_string =
                                        demangled_cstr.to_string_lossy().into_owned();
                                    libc::free(demangled_ptr as *mut c_void);
                                    Some(demangled_string)
                                } else {
                                    None
                                }
                            };

                            match demangled_symbol {
                                Some(demangled_symbol) => {
                                    text.replace_range(
                                        mangled_start..mangled_end,
                                        &demangled_symbol,
                                    );
                                    search_from = mangled_start + demangled_symbol.len();
                                }
                                None => {
                                    search_from = mangled_start + 2;
                                }
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    fn output_pointer(pointer: *mut c_void, handler: &dyn BacktraceOutputHandler) {
        let mut buf: [c_char; 17] = [0; 17];
        handler.handle_output("0x");
        let addr = pointer as usize;
        match internal::itoa_r(addr as isize, &mut buf, 16, 12) {
            Some(ptr) => {
                let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
                handler.handle_output(c_str.to_str().unwrap());
            }
            None => handler.handle_output("Conversion failed"),
        }
    }

    struct PrintBacktraceOutputHandler {}

    impl BacktraceOutputHandler for PrintBacktraceOutputHandler {
        fn handle_output(&self, output: &str) {
            let _ = write!(std::io::stderr(), "{}", output);
        }
        fn output_file_descriptor(&self) -> i32 {
            2
        }
    }

    struct StreamBacktraceOutputHandler<'a> {
        os: &'a mut dyn Write,
    }

    impl<'a> BacktraceOutputHandler for StreamBacktraceOutputHandler<'a> {
        fn handle_output(&self, output: &str) {
            let _ = write!(self.os, "{}", output);
        }
    }

    pub fn enable_in_process_stack_dumping() -> bool {
        unsafe {
            let mut sigpipe_action: libc::sigaction = mem::zeroed();
            sigpipe_action.sa_handler = libc::SIG_IGN;
            libc::sigemptyset(&mut sigpipe_action.sa_mask);

            let success = libc::sigaction(
                libc::SIGPIPE,
                &sigpipe_action,
                ptr::null_mut(),
            ) == 0;

            warm_up_backtrace();

            let mut action: libc::sigaction = mem::zeroed();
            action.sa_flags = libc::SA_RESETHAND | libc::SA_SIGINFO | libc::SA_ONSTACK;
            action.sa_sigaction = stack_dump_signal_handler;
            libc::sigemptyset(&mut action.sa_mask);

            let success = success
                && libc::sigaction(libc::SIGILL, &action, ptr::null_mut()) == 0
                && libc::sigaction(libc::SIGABRT, &action, ptr::null_mut()) == 0
                && libc::sigaction(libc::SIGFPE, &action, ptr::null_mut()) == 0
                && libc::sigaction(libc::SIGBUS, &action, ptr::null_mut()) == 0
                && libc::sigaction(libc::SIGSEGV, &action, ptr::null_mut()) == 0
                && libc::sigaction(libc::SIGSYS, &action, ptr::null_mut()) == 0;

            let mut dump_stack_lock = DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap();
            *dump_stack_lock = true;

            success
        }
    }

    pub fn disable_signal_stack_dump() {
        let mut dump_stack_lock = DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap();
        *dump_stack_lock = false;
    }

    unsafe extern "C" fn stack_dump_signal_handler(
        signal: c_int,
        info: *mut libc::siginfo_t,
        _void_context: *mut c_void,
    ) {
        IN_SIGNAL_HANDLER.store(true, Ordering::Relaxed);

        let mut buf: [c_char; 1024] = [0; 1024];

        write_to_stderr("Received signal ");
        match internal::itoa_r(signal as isize, &mut buf, 10, 0) {
            Some(ptr) => {
                let c_str = std::ffi::CStr::from_ptr(ptr);
                write_to_stderr(c_str.to_str().unwrap());
            }
            None => write_to_stderr("Conversion failed"),
        }

        if signal == libc::SIGBUS {
            if (*info).si_code == libc::BUS_ADRALN as i32 {
                write_to_stderr(" BUS_ADRALN ");
            } else if (*info).si_code == libc::BUS_ADRERR as i32 {
                write_to_stderr(" BUS_ADRERR ");
            } else if (*info).si_code == libc::BUS_OBJERR as i32 {
                write_to_stderr(" BUS_OBJERR ");
            } else {
                write_to_stderr(" <unknown> ");
            }
        } else if signal == libc::SIGFPE {
            if (*info).si_code == libc::FPE_FLTDIV as i32 {
                write_to_stderr(" FPE_FLTDIV ");
            } else if (*info).si_code == libc::FPE_FLTINV as i32 {
                write_to_stderr(" FPE_FLTINV ");
            } else if (*info).si_code == libc::FPE_FLTOVF as i32 {
                write_to_stderr(" FPE_FLTOVF ");
            } else if (*info).si_code == libc::FPE_FLTRES as i32 {
                write_to_stderr(" FPE_FLTRES ");
            } else if (*info).si_code == libc::FPE_FLTSUB as i32 {
                write_to_stderr(" FPE_FLTSUB ");
            } else if (*info).si_code == libc::FPE_FLTUND as i32 {
                write_to_stderr(" FPE_FLTUND ");
            } else if (*info).si_code == libc::FPE_INTDIV as i32 {
                write_to_stderr(" FPE_INTDIV ");
            } else if (*info).si_code == libc::FPE_INTOVF as i32 {
                write_to_stderr(" FPE_INTOVF ");
            } else {
                write_to_stderr(" <unknown> ");
            }
        } else if signal == libc::SIGILL {
            if (*info).si_code == libc::ILL_BADSTK as i32 {
                write_to_stderr(" ILL_BADSTK ");
            } else if (*info).si_code == libc::ILL_COPROC as i32 {
                write_to_stderr(" ILL_COPROC ");
            } else if (*info).si_code == libc::ILL_ILLOPN as i32 {
                write_to_stderr(" ILL_ILLOPN ");
            } else if (*info).si_code == libc::ILL_ILLADR as i32 {
                write_to_stderr(" ILL_ILLADR ");
            } else if (*info).si_code == libc::ILL_ILLTRP as i32 {
                write_to_stderr(" ILL_ILLTRP ");
            } else if (*info).si_code == libc::ILL_PRVOPC as i32 {
                write_to_stderr(" ILL_PRVOPC ");
            } else if (*info).si_code == libc::ILL_PRVREG as i32 {
                write_to_stderr(" ILL_PRVREG ");
            } else {
                write_to_stderr(" <unknown> ");
            }
        } else if signal == libc::SIGSEGV {
            if (*info).si_code == libc::SEGV_MAPERR as i32 {
                write_to_stderr(" SEGV_MAPERR ");
            } else if (*info).si_code == libc::SEGV_ACCERR as i32 {
                write_to_stderr(" SEGV_ACCERR ");
            } else {
                write_to_stderr(" <unknown> ");
            }
        }
        if signal == libc::SIGBUS
            || signal == libc::SIGFPE
            || signal == libc::SIGILL
            || signal == libc::SIGSEGV
        {
            match internal::itoa_r((*info).si_addr as isize, &mut buf, 16, 12) {
                Some(ptr) => {
                    let c_str = std::ffi::CStr::from_ptr(ptr);
                    write_to_stderr(c_str.to_str().unwrap());
                }
                None => write_to_stderr("Conversion failed"),
            }
        }
        write_to_stderr("\n");

        let dump_stack_lock = DUMP_STACK_IN_SIGNAL_HANDLER.lock().unwrap();
        if *dump_stack_lock {
            StackTrace::new().print();
            write_to_stderr("[end of stack trace]\n");
        }

        libc::signal(signal, libc::SIG_DFL);
        libc::_exit(1);
    }

    fn write_to_stderr(output: &str) {
        let _ = write!(std::io::stderr(), "{}", output);
    }

    fn warm_up_backtrace() {
        let _stack_trace = StackTrace::new();
    }
}

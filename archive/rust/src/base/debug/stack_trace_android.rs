use std::os::raw::c_void;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fmt, io, mem, os::raw::c_int};

#[cfg(target_os = "android")]
mod android {
    use std::{
        ffi::CStr,
        fs::File,
        io::{BufRead, BufReader},
        mem,
        os::raw::c_int,
        path::Path,
        sync::atomic::{AtomicBool, Ordering},
    };

    #[repr(C)]
    enum UnwindReasonCode {
        NoError = 0,
        UnwindStop = 1,
        FatalPhase2Error = 2,
        FatalPhase1Error = 3,
        NormalStop = 4,
        ForcedUnwind = 5,
    }

    extern "C" {
        fn _Unwind_Backtrace(
            trace_fn: extern "C" fn(context: *mut c_void, arg: *mut StackCrawlState) -> UnwindReasonCode,
            arg: *mut StackCrawlState,
        ) -> UnwindReasonCode;
        fn _Unwind_GetIP(context: *mut c_void) -> usize;
    }

    const MAX_TRACES: usize = 256;

    struct StackCrawlState {
        frames: *mut usize,
        frame_count: usize,
        max_depth: usize,
        have_skipped_self: bool,
    }

    impl StackCrawlState {
        fn new(frames: *mut usize, max_depth: usize) -> Self {
            StackCrawlState {
                frames,
                frame_count: 0,
                max_depth,
                have_skipped_self: false,
            }
        }
    }

    extern "C" fn trace_stack_frame(
        context: *mut c_void,
        arg: *mut StackCrawlState,
    ) -> UnwindReasonCode {
        unsafe {
            let state = &mut *arg;
            let ip = _Unwind_GetIP(context);

            if ip != 0 && !state.have_skipped_self {
                state.have_skipped_self = true;
                return UnwindReasonCode::NoError;
            }

            let frames = state.frames;
            let frame_count = state.frame_count;

            *frames.add(frame_count) = ip;
            state.frame_count += 1;

            if state.frame_count >= state.max_depth {
                return UnwindReasonCode::UnwindStop;
            }
            UnwindReasonCode::NoError
        }
    }

    pub struct StackTrace {
        trace: [usize; MAX_TRACES],
        count: usize,
    }

    impl StackTrace {
        pub fn new() -> Self {
            let mut trace: [usize; MAX_TRACES] = [0; MAX_TRACES];
            let mut state = StackCrawlState::new(trace.as_mut_ptr(), MAX_TRACES);

            unsafe {
                _Unwind_Backtrace(trace_stack_frame, &mut state);
            }

            StackTrace {
                trace,
                count: state.frame_count,
            }
        }

        pub fn print(&self) {
            let backtrace = self.to_string();
            eprintln!("{}", backtrace);
        }

        pub fn output_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            for i in 0..self.count {
                writeln!(writer, "#{:2} {:#x}", i, self.trace[i])?;
            }
            Ok(())
        }
    }

    impl fmt::Display for StackTrace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for i in 0..self.count {
                writeln!(f, "#{:2} {:#x}", i, self.trace[i])?;
            }
            Ok(())
        }
    }
    static SIGPIPE_HANDLED: AtomicBool = AtomicBool::new(false);

    pub fn enable_in_process_stack_dumping() -> bool {
        if SIGPIPE_HANDLED.load(Ordering::Relaxed) {
            return true;
        }

        unsafe {
            let mut action: libc::sigaction = mem::zeroed();
            action.sa_handler = libc::SIG_IGN;
            libc::sigemptyset(&mut action.sa_mask);
            let result = libc::sigaction(libc::SIGPIPE, &action, ptr::null_mut());
            if result == 0 {
                SIGPIPE_HANDLED.store(true, Ordering::Relaxed);
                return true;
            }
            false
        }
    }

    pub fn disable_signal_stack_dump() {}
}

#[cfg(not(target_os = "android"))]
mod android {
    pub fn enable_in_process_stack_dumping() -> bool {
        true
    }
    pub fn disable_signal_stack_dump() {}

    pub struct StackTrace {}

    impl StackTrace {
        pub fn new() -> Self {
            StackTrace {}
        }
        pub fn print(&self) {}
        pub fn output_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl fmt::Display for StackTrace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }
}

pub use android::*;
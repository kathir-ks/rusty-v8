use std::io::{Error, ErrorKind};
use std::os::raw::{c_int, c_void};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern "C" {
    fn __display_backtrace(fd: c_int);
    fn __set_backtrace_on_abort(enable: bool);
    fn raise(sig: c_int);
    fn pause() -> c_int;
    fn abort() -> !;
}

const STDERR_FILENO: c_int = 2;

static IS_DUMP_STACK_IN_SIGNAL_HANDLER: AtomicBool = AtomicBool::new(true);

/// Enables in-process stack dumping.
pub fn enable_in_process_stack_dumping() -> Result<(), Error> {
    IS_DUMP_STACK_IN_SIGNAL_HANDLER.store(true, Ordering::SeqCst);
    let success = start_thread(stack_dumping_signal_thread)?;

    // Block all signals on the main thread:
    let mut set: sigset_t = unsafe { std::mem::zeroed() };
    unsafe {
        sigfillset(&mut set);
        if pthread_sigmask(SIG_BLOCK, &set, std::ptr::null_mut()) != 0 {
            return Err(Error::new(ErrorKind::Other, "pthread_sigmask failed"));
        }
    }

    Ok(success)
}

/// Disables signal stack dumping.
pub fn disable_signal_stack_dump() {
    IS_DUMP_STACK_IN_SIGNAL_HANDLER.store(false, Ordering::SeqCst);
    // zoslib's abort() displays backtrace by default, so disable it:
    unsafe {
        __set_backtrace_on_abort(false);
    }
}

#[derive(Default)]
pub struct StackTrace {}

impl StackTrace {
    pub fn new() -> Self {
        StackTrace {}
    }

    pub fn print(&self) {
        unsafe {
            __display_backtrace(STDERR_FILENO);
        }
    }

    pub fn output_to_stream(&self, _os: &mut dyn std::io::Write) {
        // TODO(gabylb): zos - pending std::osstream version in zoslib:
        // __display_backtrace(os);
        panic!("UNREACHABLE");
    }
}

fn start_thread(thread_entry: fn(usize) -> Result<(), Error>) -> Result<(), Error> {
    let builder = thread::Builder::new();
    builder
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            thread_entry(0).unwrap();
        })
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(())
}

extern "C" {
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(
        signum: c_int,
        act: *const sigaction_t,
        oldact: *mut sigaction_t,
    ) -> c_int;
    fn pthread_sigmask(
        how: c_int,
        set: *const sigset_t,
        oset: *mut sigset_t,
    ) -> c_int;
}

const SIGPIPE: c_int = 13;
const SIG_IGN: usize = 1;
const SIGILL: c_int = 4;
const SIGABRT: c_int = 6;
const SIGFPE: c_int = 8;
const SIGBUS: c_int = 10;
const SIGSEGV: c_int = 11;
const SIGSYS: c_int = 31;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SA_RESETHAND: i32 = 0x80000000;
const SA_SIGINFO: i32 = 4;
const SA_ONSTACK: i32 = 0x08000000;
const SIG_BLOCK: c_int = 0;
const EINTR: i32 = 4;

#[repr(C)]
struct sigaction_t {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    // ... other fields ...
}

#[repr(C)]
struct sigset_t {
    __val: [usize; 16],
}

extern "C" fn stack_dump_signal_handler(
    signal: c_int,
    _info: *mut siginfo_t,
    _void_context: *mut c_void,
) {
    eprintln!("Received signal {}", signal);
    if signal == SIGABRT {
        // From third_party/zoslib, will first call __display_traceback().
        unsafe {
            abort();
        }
    }
    if IS_DUMP_STACK_IN_SIGNAL_HANDLER.load(Ordering::SeqCst) {
        unsafe {
            __display_backtrace(STDERR_FILENO);
        }
    }
    unsafe {
        raise(signal);
    }
}

fn stack_dumping_signal_thread(_data: usize) -> Result<(), Error> {
    unsafe {
        let mut sigpipe_action: sigaction_t = std::mem::zeroed();
        sigpipe_action.sa_sigaction = std::mem::transmute(SIG_IGN);
        sigemptyset(&mut sigpipe_action.sa_mask);
        let mut success = sigaction(SIGPIPE, &sigpipe_action, std::ptr::null_mut()) == 0;

        let mut action: sigaction_t = std::mem::zeroed();
        action.sa_flags = SA_RESETHAND | SA_SIGINFO | SA_ONSTACK;
        action.sa_sigaction = stack_dump_signal_handler;
        sigemptyset(&mut action.sa_mask);

        success &= sigaction(SIGILL, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGABRT, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGFPE, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGBUS, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGSEGV, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGSYS, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGINT, &action, std::ptr::null_mut()) == 0;
        success &= sigaction(SIGTERM, &action, std::ptr::null_mut()) == 0;

        assert!(success);

        loop {
            let result = pause();
            assert_eq!(result, -1);
            assert_eq!(std::io::Error::last_os_error().raw_os_error().unwrap() as i32, EINTR);
        }
    }
}
// Converted from V8 C++ source files:
// Header: N/A
// Implementation: stack_trace_zos.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::os::raw::{c_int, c_void};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use libc::{pthread_attr_t, pthread_create, pthread_attr_init, pthread_attr_destroy, pthread_attr_setstacksize, pthread_t, sigaction, siginfo_t, sigemptyset, raise, pause, SIG_IGN, SIGPIPE, SA_RESETHAND, SA_SIGINFO, SA_ONSTACK, SIGILL, SIGABRT, SIGFPE, SIGBUS, SIGSEGV, SIGSYS, SIGINT, SIGTERM, errno, EINTR, STDERR_FILENO, sigfillset, pthread_sigmask, SIG_BLOCK};
use std::mem::size_of;
use std::io::Write;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_DUMP_STACK_IN_SIGNAL_HANDLER: AtomicBool = AtomicBool::new(true);

extern "C" {
    fn __display_backtrace(fd: c_int);
    fn __set_backtrace_on_abort(enable: bool);
}

fn start_thread(thread_entry: extern "C" fn(*mut c_void) -> *mut c_void) -> bool {
    unsafe {
        let mut attr: pthread_attr_t = std::mem::zeroed();
        let mut result = pthread_attr_init(&mut attr);
        if result != 0 {
            return false;
        }

        let k_default_stack_size: usize = 4 * 1024 * 1024;
        let mut stack_size: usize = 0;
        result = libc::pthread_attr_getstacksize(&attr, &mut stack_size);

        assert_eq!(0, result);
        let mut stack_size_to_use = k_default_stack_size;
        if stack_size < k_default_stack_size {
          stack_size_to_use = k_default_stack_size;
        }

        result = pthread_attr_setstacksize(&mut attr, stack_size_to_use);
        if result != 0 {
            pthread_attr_destroy(&mut attr);
            return false;
        }

        let lock_guard = Mutex::new(()); // dummy mutex
        let mut thread_: pthread_t = 0;
        result = pthread_create(&mut thread_, &attr, thread_entry, ptr::null_mut());
        if result != 0 {
            eprintln!("pthread_create failed: {}", result);
            pthread_attr_destroy(&mut attr);
            return false;
        }

        result = pthread_attr_destroy(&mut attr);
        return result == 0;
    }
}

extern "C" fn stack_dump_signal_handler(signal: c_int, info: *mut siginfo_t, void_context: *mut c_void) {
    eprintln!("Received signal {}", signal);
    if signal == SIGABRT {
        unsafe {
            libc::abort();
        }
    }
    if IS_DUMP_STACK_IN_SIGNAL_HANDLER.load(Ordering::Relaxed) {
        unsafe {
            __display_backtrace(STDERR_FILENO);
        }
    }
    unsafe {
        raise(signal);
    }
}

extern "C" fn stack_dumping_signal_thread(_data: *mut c_void) -> *mut c_void {
    unsafe {
        let mut sigpipe_action: libc::sigaction = std::mem::zeroed();
        sigpipe_action.sa_handler = SIG_IGN;
        libc::sigemptyset(&mut sigpipe_action.sa_mask);
        let mut success = libc::sigaction(SIGPIPE, &sigpipe_action, ptr::null_mut()) == 0;

        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_flags = SA_RESETHAND | SA_SIGINFO | SA_ONSTACK;
        action.sa_sigaction = stack_dump_signal_handler;
        libc::sigemptyset(&mut action.sa_mask);

        success &= libc::sigaction(SIGILL, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGABRT, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGFPE, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGBUS, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGSEGV, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGSYS, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGINT, &action, ptr::null_mut()) == 0;
        success &= libc::sigaction(SIGTERM, &action, ptr::null_mut()) == 0;

        assert_eq!(true, success);

        loop {
            assert_eq!(pause(), -1);
            assert_eq!(errno, EINTR);
        }
    }
}

pub fn enable_in_process_stack_dumping() -> bool {
    IS_DUMP_STACK_IN_SIGNAL_HANDLER.store(true, Ordering::Relaxed);
    let success = start_thread(stack_dumping_signal_thread);
    assert_eq!(true, success);

    unsafe {
        let mut set: libc::sigset_t = std::mem::zeroed();
        libc::sigfillset(&mut set);
        assert_eq!(0, pthread_sigmask(SIG_BLOCK, &set, ptr::null_mut()));
    }
    success
}

pub fn disable_signal_stack_dump() {
    IS_DUMP_STACK_IN_SIGNAL_HANDLER.store(false, Ordering::Relaxed);
    unsafe {
        __set_backtrace_on_abort(false);
    }
}

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

    pub fn output_to_stream(&self) {
      eprintln!("StackTrace::OutputToStream called. This function is currently not fully implemented for zOS.");
    }
}

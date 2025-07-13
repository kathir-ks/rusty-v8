// Converted from V8 C++ source files:
// Header: handler-inside-win.h
// Implementation: handler-inside-win.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::{ffi::c_void, ptr, arch::x86_64::ucontext};
use std::sync::Mutex;

//use winapi::um::winnt::{EXCEPTION_POINTERS, EXCEPTION_RECORD};
//use winapi::shared::minwindef::{DWORD, LONG, PVOID};
//use winapi::um::processthreadsapi::NtCurrentTeb;

pub type PVOID = *mut c_void;
pub type DWORD = u32;
pub type LONG = i32;

#[repr(C)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionAddress: PVOID,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [usize; 15],
}

#[repr(C)]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,

    pub ContextFlags: DWORD,
    pub MxCsr: DWORD,

    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,

    pub EFlags: DWORD,

    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,

    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,

    pub Rip: u64,

    pub Anonymous: CONTEXT_Anonymous,

    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,

    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}

#[repr(C)]
pub union CONTEXT_Anonymous {
    pub FltSave: FLOATING_SAVE_AREA,
    pub Anonymous: CONTEXT_Anonymous2,
}

#[repr(C)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: DWORD,
    pub StatusWord: DWORD,
    pub TagWord: DWORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: DWORD,
    pub DataOffset: DWORD,
    pub DataSelector: DWORD,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: DWORD,
}

#[repr(C)]
pub struct CONTEXT_Anonymous2 {
    pub Legacy: [u8; 32],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}

#[repr(C)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}

#[repr(C)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ContextRecord: *mut CONTEXT,
}

#[cfg(target_arch = "x86_64")]
#[link(name = "kernel32")]
extern "system" {
    pub fn NtCurrentTeb() -> PVOID;
}

const EXCEPTION_ACCESS_VIOLATION: DWORD = 0xC0000005;
const EXCEPTION_CONTINUE_EXECUTION: LONG = -1;
const EXCEPTION_CONTINUE_SEARCH: LONG = 0;

static g_thread_in_wasm_code: AtomicBool = AtomicBool::new(false);

// Simulate thread local storage
lazy_static::lazy_static! {
    static ref THREAD_LOCAL_STORAGE: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

fn IsThreadInWasm() -> bool {
    g_thread_in_wasm_code.load(Ordering::Relaxed)
}

fn IsFaultAddressCovered(_fault_addr: usize) -> bool {
    // Dummy implementation
    true
}

static gLandingPad: Mutex<usize> = Mutex::new(0);

pub fn set_landing_pad(address: usize) {
    let mut guard = gLandingPad.lock().unwrap();
    *guard = address;
}

#[repr(C)]
struct TEB {
    reserved: [PVOID; 11],
    thread_local_storage_pointer: PVOID,
}

//extern "C" {
//    static probe_memory_continuation: [u8; 0];
//}

#[no_mangle]
static v8_simulator_probe_memory_continuation: [u8; 0] = [];

pub fn TryHandleWasmTrap(exception: *mut EXCEPTION_POINTERS) -> bool {
    unsafe {
        let exception = &mut *exception;

        if exception.ExceptionRecord.is_null() || exception.ContextRecord.is_null() {
            return false;
        }

        let record = &*exception.ExceptionRecord;
        let context = &mut *exception.ContextRecord;

        if record.ExceptionCode != EXCEPTION_ACCESS_VIOLATION {
            return false;
        }

        let pteb: *mut TEB = NtCurrentTeb() as *mut TEB;
        if pteb.is_null() || (*pteb).thread_local_storage_pointer.is_null() {
            return false;
        }

        if !IsThreadInWasm() {
            return false;
        }

        g_thread_in_wasm_code.store(false, Ordering::Relaxed);

        let fault_addr = record.ExceptionAddress as usize;

        if !IsFaultAddressCovered(fault_addr) {
            return false;
        }

        let landing_pad = *gLandingPad.lock().unwrap();
        if landing_pad == 0 {
          return false;
        }

        context.Rip = landing_pad as u64;
        context.R10 = fault_addr as u64;
        g_thread_in_wasm_code.store(true, Ordering::Relaxed);
        return true;
    }
}

pub fn HandleWasmTrap(exception: *mut EXCEPTION_POINTERS) -> LONG {
    if TryHandleWasmTrap(exception) {
        EXCEPTION_CONTINUE_EXECUTION
    } else {
        EXCEPTION_CONTINUE_SEARCH
    }
}

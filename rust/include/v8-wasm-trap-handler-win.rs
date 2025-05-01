// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(target_os = "windows")]
pub mod wasm_trap_handler_win {
    use std::os::raw::c_void;

    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8")]
    extern "C" {
        /// This function determines whether a memory access violation has been an
        /// out-of-bounds memory access in WebAssembly. If so, it will modify the
        /// exception parameter and add a return address where the execution can continue
        /// after the exception handling, and return true. Otherwise the return value
        /// will be false.
        ///
        /// The parameter to this function corresponds to the one passed to a Windows
        /// vectored exception handler. Use this function only on Windows.
        ///
        /// # Safety
        ///
        /// This function is inherently unsafe as it directly interacts with Windows exception handling mechanisms.
        /// The caller must ensure that the `exception` pointer is valid and points to a valid `EXCEPTION_POINTERS` structure.
        /// Improper use can lead to undefined behavior, including crashes and security vulnerabilities.
        pub fn TryHandleWebAssemblyTrapWindows(exception: *mut EXCEPTION_POINTERS) -> bool;
    }

    #[cfg(target_arch = "x86_64")]
    #[repr(C)]
    pub struct EXCEPTION_RECORD {
        pub ExceptionCode: u32,
        pub ExceptionFlags: u32,
        pub ExceptionAddress: *mut c_void,
        pub NumberParameters: u32,
        pub ExceptionInformation: [usize; 15], // Assuming usize matches DWORD_PTR on 64-bit Windows
    }

    #[cfg(target_arch = "x86_64")]
    #[repr(C)]
    pub struct CONTEXT {
        pub P1Home: u64,
        pub P2Home: u64,
        pub P3Home: u64,
        pub P4Home: u64,
        pub P5Home: u64,
        pub P6Home: u64,

        pub MxCsr: u32,
        pub FloatControl: u32,

        pub SegGs: u32,
        pub SegFs: u32,
        pub SegEs: u32,
        pub SegDs: u32,

        pub Rdi: u64,
        pub Rsi: u64,
        pub Rbp: u64,
        pub Rsp: u64,
        pub Rbx: u64,
        pub Rdx: u64,
        pub Rcx: u64,
        pub Rax: u64,
        pub Rip: u64,

        pub FloatStatus: u32,
        pub Reserved1: u8,
        pub DebugControl: u64,
        pub LastBranchToRip: u64,
        pub LastBranchFromRip: u64,
        pub LastExceptionToRip: u64,
        pub LastExceptionFromRip: u64,

        pub VectorRegister: [M128A; 26],

        pub VectorControl: u64,
        pub DebugControlMsr: u64,
        pub DebugActiveMsr: u64,
        pub Dr0: u64,
        pub Dr1: u64,
        pub Dr2: u64,
        pub Dr3: u64,
        pub Dr6: u64,
        pub Dr7: u64,

        pub FltSave: XSAVE_FORMAT,
    }

    #[cfg(target_arch = "x86_64")]
    #[repr(C)]
    pub struct M128A {
        pub Low: u64,
        pub High: i64,
    }

    #[cfg(target_arch = "x86_64")]
    #[repr(C)]
    pub struct XSAVE_FORMAT {
        pub ControlWord: u16,
        pub StatusWord: u16,
        pub TagWord: u8,
        pub Reserved1: u8,
        pub ErrorOpcode: u16,
        pub ErrorOffset: u32,
        pub ErrorSelector: u16,
        pub Reserved2: u16,
        pub DataOffset: u32,
        pub DataSelector: u16,
        pub Reserved3: u16,
        pub MxCsr: u32,
        pub MxCsrMask: u32,
        pub FloatRegisters: [M128A; 8],
        pub StiReserved: [M128A; 8],
        pub XmmRegisters: [M128A; 16],
        pub Reserved4: [u8; 96],
    }

    #[cfg(target_arch = "x86_64")]
    #[repr(C)]
    pub struct EXCEPTION_POINTERS {
        pub ExceptionRecord: *mut EXCEPTION_RECORD,
        pub ContextRecord: *mut CONTEXT,
    }

    #[cfg(not(target_arch = "x86_64"))]
    compile_error!("This module only supports x86_64 architecture.");

}
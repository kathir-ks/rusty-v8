// Copyright (c) 2012 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2016 the V8 project authors. All rights reserved.

#![allow(non_snake_case)]

use std::ffi::OsString;
use std::io;
use std::io::Write;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, LPDWORD, ULONG64};
use winapi::shared::ntdef::PVOID;
use winapi::shared::windef::HWND;
use winapi::um::dbghelp::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::GetCurrentProcess;
use winapi::um::processthreadsapi::{GetCurrentThread, GetModuleFileNameW};
use winapi::um::synchapi::Sleep;
use winapi::um::winnt::{CONTEXT, EXCEPTION_POINTERS, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, LPTOP_LEVEL_EXCEPTION_FILTER};
use winapi::um::winuser::SetUnhandledExceptionFilter;

//use crate::base::logging;
//use crate::base::macros;

pub mod debug {
    use super::*;
    use std::sync::Mutex;

    static PREVIOUS_FILTER: Mutex<Option<LPTOP_LEVEL_EXCEPTION_FILTER>> = Mutex::new(None);
    static DUMP_STACK_IN_SIGNAL_HANDLER: AtomicBool = AtomicBool::new(true);
    static INITIALIZED_SYMBOLS: AtomicBool = AtomicBool::new(false);
    static INIT_ERROR: AtomicU32 = AtomicU32::new(0);

    // Prints the exception call stack.
    // This is the unit tests exception filter.
    unsafe extern "system" fn stack_dump_exception_filter(info: *mut EXCEPTION_POINTERS) -> i32 {
        if DUMP_STACK_IN_SIGNAL_HANDLER.load(Ordering::Relaxed) {
            StackTrace::new_from_exception_pointers(info).print();
        }
        let mut previous_filter = PREVIOUS_FILTER.lock().unwrap();
        if let Some(filter) = *previous_filter {
            filter(info)
        } else {
            EXCEPTION_CONTINUE_SEARCH
        }
    }

    fn initialize_symbols() -> bool {
        if INITIALIZED_SYMBOLS.load(Ordering::Relaxed) {
            return INIT_ERROR.load(Ordering::Relaxed) == 0;
        }
        INITIALIZED_SYMBOLS.store(true, Ordering::Relaxed);

        unsafe {
            // Defer symbol load until they're needed, use undecorated names, and get line
            // numbers.
            SymSetOptions(SYMOPT_DEFERRED_LOADS | SYMOPT_UNDNAME | SYMOPT_LOAD_LINES);
            if SymInitialize(GetCurrentProcess(), ptr::null_mut(), TRUE) == FALSE {
                let err = GetLastError();
                INIT_ERROR.store(err, Ordering::Relaxed);
                // TODO(awong): Handle error: SymInitialize can fail with
                // ERROR_INVALID_PARAMETER.
                // When it fails, we should not call debugbreak since it kills the current
                // process (prevents future tests from running or kills the browser
                // process).
                return false;
            }

            // When transferring the binaries e.g. between bots, path put
            // into the executable will get off. To still retrieve symbols correctly,
            // add the directory of the executable to symbol search path.
            // All following errors are non-fatal.
            const K_SYMBOLS_ARRAY_SIZE: usize = 1024;
            let mut symbols_path = Vec::with_capacity(K_SYMBOLS_ARRAY_SIZE);
            let symbols_path_ptr = symbols_path.as_mut_ptr() as *mut u16;

            // Note: The below function takes buffer size as number of characters,
            // not number of bytes!
            if SymGetSearchPathW(GetCurrentProcess(), symbols_path_ptr, K_SYMBOLS_ARRAY_SIZE as DWORD) == FALSE {
                let err = GetLastError();
                INIT_ERROR.store(err, Ordering::Relaxed);
                return false;
            }
            let symbols_path_string = OsString::from_wide(std::slice::from_raw_parts(symbols_path_ptr, K_SYMBOLS_ARRAY_SIZE));
            let symbols_path_str = symbols_path_string.to_string_lossy();

            let mut exe_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            GetModuleFileNameW(ptr::null_mut(), exe_path.as_mut_ptr(), MAX_PATH as DWORD);
            let exe_path_string = OsString::from_wide(&exe_path);
            let exe_path_str = exe_path_string.to_string_lossy();

            // To get the path without the filename, we just need to remove the final
            // slash and everything after it.
            let last_slash = exe_path_str.rfind('\\').or(exe_path_str.rfind('/')).unwrap_or(0);
            let new_path = format!("{};{}", symbols_path_str, &exe_path_str[..last_slash]);
            let new_path_wide: Vec<u16> = new_path.encode_utf16().collect();

            if SymSetSearchPathW(GetCurrentProcess(), new_path_wide.as_ptr()) == FALSE {
                let err = GetLastError();
                INIT_ERROR.store(err, Ordering::Relaxed);
                return false;
            }

            INIT_ERROR.store(0, Ordering::Relaxed);
            true
        }
    }

    // For the given trace, attempts to resolve the symbols, and output a trace
    // to the ostream os.  The format for each line of the backtrace is:
    //
    //    <tab>SymbolName[0xAddress+Offset] (FileName:LineNo)
    //
    // This function should only be called if Init() has been called.  We do not
    // LOG(FATAL) here because this code is called might be triggered by a
    // LOG(FATAL) itself. Also, it should not be calling complex code that is
    // extensible like PathService since that can in turn fire CHECKs.
    fn output_trace_to_stream(trace: &[*const std::ffi::c_void], count: usize, os: &mut dyn Write) {
        for i in 0..count {
            if !os.write_all(b"\t").is_ok() {
                return;
            }

            let frame = trace[i] as usize;

            unsafe {
                const K_MAX_NAME_LENGTH: usize = 256;
                let mut buffer: [u8; (std::mem::size_of::<SYMBOL_INFO>() + K_MAX_NAME_LENGTH * std::mem::size_of::<u16>() + std::mem::size_of::<ULONG64>() - 1) as usize] = [0; (std::mem::size_of::<SYMBOL_INFO>() + K_MAX_NAME_LENGTH * std::mem::size_of::<u16>() + std::mem::size_of::<ULONG64>() - 1) as usize];
                let symbol = buffer.as_mut_ptr() as *mut SYMBOL_INFO;
                (*symbol).SizeOfStruct = std::mem::size_of::<SYMBOL_INFO>() as u32;
                (*symbol).MaxNameLen = (K_MAX_NAME_LENGTH - 1) as u32;

                let mut sym_displacement: ULONG64 = 0;
                let has_symbol = SymFromAddr(GetCurrentProcess(), frame as ULONG64, &mut sym_displacement, symbol);

                let mut line: IMAGEHLP_LINE64 = std::mem::zeroed();
                line.SizeOfStruct = std::mem::size_of::<IMAGEHLP_LINE64>() as u32;
                let mut line_displacement: DWORD = 0;

                let has_line = SymGetLineFromAddr64(GetCurrentProcess(), frame as ULONG64, &mut line_displacement, &mut line);

                if has_symbol == TRUE {
                    let name = std::slice::from_raw_parts((*symbol).Name.as_ptr() as *const u8, (*symbol).NameLen as usize);
                    let name_str = String::from_utf8_lossy(name);
                    write!(os, "{} [0x{:p}+{}]", name_str, trace[i], sym_displacement).unwrap();
                } else {
                    write!(os, "(No symbol) [0x{:p}]", trace[i]).unwrap();
                }

                if has_line == TRUE {
                    let filename = std::slice::from_raw_parts(line.FileName as *const u8, strlen(line.FileName) as usize);
                    let filename_str = String::from_utf8_lossy(filename);
                    write!(os, " ({} : {})", filename_str, line.LineNumber).unwrap();
                }
                writeln!(os).unwrap();
            }
        }
    }

    unsafe fn strlen(s: *const i8) -> usize {
        let mut i = 0;
        while *s.add(i) != 0 {
            i += 1;
        }
        i
    }

    pub fn enable_in_process_stack_dumping() -> bool {
        // Add stack dumping support on exception on windows. Similar to OS_POSIX
        // signal() handling in process_util_posix.cc.
        unsafe {
            let mut previous_filter = PREVIOUS_FILTER.lock().unwrap();
            *previous_filter = SetUnhandledExceptionFilter(Some(stack_dump_exception_filter));
        }
        DUMP_STACK_IN_SIGNAL_HANDLER.store(true, Ordering::Relaxed);

        // Need to initialize symbols early in the process or else this fails on
        // swarming (since symbols are in different directory than in the exes) and
        // also release x64.
        initialize_symbols()
    }

    pub fn disable_signal_stack_dump() {
        DUMP_STACK_IN_SIGNAL_HANDLER.store(false, Ordering::Relaxed);
    }

    #[derive(Debug)]
    pub struct StackTrace {
        trace: [*const std::ffi::c_void; 62], // kMaxFrames = 62
        count: usize,
    }

    impl StackTrace {
        pub fn new() -> Self {
            let mut trace: [*const std::ffi::c_void; 62] = [std::ptr::null(); 62];
            let count = unsafe { CaptureStackBackTrace(0, 62 as u32, trace.as_mut_ptr(), ptr::null_mut()) as usize };
            StackTrace { trace, count }
        }

        pub fn new_from_exception_pointers(exception_pointers: *mut EXCEPTION_POINTERS) -> Self {
            unsafe {
                StackTrace::new_from_context((*exception_pointers).ContextRecord)
            }
        }

        pub fn new_from_context(context: *const CONTEXT) -> Self {
            let mut trace: [*const std::ffi::c_void; 62] = [std::ptr::null(); 62];
            let mut stack_trace = StackTrace { trace, count: 0 };
            stack_trace.init_trace(context);
            stack_trace
        }

        fn init_trace(&mut self, context_record: *const CONTEXT) {
            // StackWalk64 modifies the register context in place, so we have to copy it
            // so that downstream exception handlers get the right context.  The incoming
            // context may have had more register state (YMM, etc) than we need to unwind
            // the stack. Typically StackWalk64 only needs integer and control registers.
            unsafe {
                let mut context_copy: CONTEXT = std::mem::zeroed();
                std::ptr::copy_nonoverlapping(context_record, &mut context_copy, 1);
                context_copy.ContextFlags = CONTEXT_INTEGER | CONTEXT_CONTROL;

                // When walking an exception stack, we need to use StackWalk64().
                self.count = 0;
                // Initialize stack walking.
                let mut stack_frame: STACKFRAME64 = std::mem::zeroed();
                #[cfg(target_arch = "x86_64")]
                {
                    let machine_type = IMAGE_FILE_MACHINE_AMD64 as u16;
                    stack_frame.AddrPC.Offset = (*context_record).Rip;
                    stack_frame.AddrFrame.Offset = (*context_record).Rbp;
                    stack_frame.AddrStack.Offset = (*context_record).Rsp;
                    stack_frame.AddrPC.Mode = AddrModeFlat;
                    stack_frame.AddrFrame.Mode = AddrModeFlat;
                    stack_frame.AddrStack.Mode = AddrModeFlat;

                    while StackWalk64(machine_type as DWORD, GetCurrentProcess(), GetCurrentThread(),
                                       &mut stack_frame, &mut context_copy, ptr::null_mut(),
                                       Some(SymFunctionTableAccess64), Some(SymGetModuleBase64), ptr::null_mut()) == TRUE &&
                        self.count < 62
                    {
                        self.trace[self.count] = stack_frame.AddrPC.Offset as *const std::ffi::c_void;
                        self.count += 1;
                    }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    let machine_type = IMAGE_FILE_MACHINE_ARM64 as u16;
                    stack_frame.AddrPC.Offset = (*context_record).Pc;
                    stack_frame.AddrFrame.Offset = (*context_record).Fp;
                    stack_frame.AddrStack.Offset = (*context_record).Sp;
                    stack_frame.AddrPC.Mode = AddrModeFlat;
                    stack_frame.AddrFrame.Mode = AddrModeFlat;
                    stack_frame.AddrStack.Mode = AddrModeFlat;

                    while StackWalk64(machine_type as DWORD, GetCurrentProcess(), GetCurrentThread(),
                                       &mut stack_frame, &mut context_copy, ptr::null_mut(),
                                       Some(SymFunctionTableAccess64), Some(SymGetModuleBase64), ptr::null_mut()) == TRUE &&
                        self.count < 62
                    {
                        self.trace[self.count] = stack_frame.AddrPC.Offset as *const std::ffi::c_void;
                        self.count += 1;
                    }
                }
                #[cfg(target_arch = "x86")]
                {
                    let machine_type = IMAGE_FILE_MACHINE_I386 as u16;
                    stack_frame.AddrPC.Offset = (*context_record).Eip as u64;
                    stack_frame.AddrFrame.Offset = (*context_record).Ebp as u64;
                    stack_frame.AddrStack.Offset = (*context_record).Esp as u64;
                    stack_frame.AddrPC.Mode = AddrModeFlat;
                    stack_frame.AddrFrame.Mode = AddrModeFlat;
                    stack_frame.AddrStack.Mode = AddrModeFlat;

                    while StackWalk64(machine_type as DWORD, GetCurrentProcess(), GetCurrentThread(),
                                       &mut stack_frame, &mut context_copy, ptr::null_mut(),
                                       Some(SymFunctionTableAccess64), Some(SymGetModuleBase64), ptr::null_mut()) == TRUE &&
                        self.count < 62
                    {
                        self.trace[self.count] = stack_frame.AddrPC.Offset as *const std::ffi::c_void;
                        self.count += 1;
                    }
                }

                for i in self.count..62 {
                    self.trace[i] = ptr::null();
                }
            }
        }

        pub fn print(&self) {
            let mut stderr = std::io::stderr();
            self.output_to_stream(&mut stderr);
        }

        pub fn output_to_stream(&self, os: &mut dyn Write) {
            initialize_symbols();
            if INIT_ERROR.load(Ordering::Relaxed) != 0 {
                writeln!(os, "Error initializing symbols ({}). Dumping unresolved backtrace:", INIT_ERROR.load(Ordering::Relaxed)).unwrap();
                for i in 0..self.count {
                    writeln!(os, "\t{:p}", self.trace[i]).unwrap();
                }
            } else {
                writeln!(os).unwrap();
                writeln!(os, "==== C stack trace ===============================").unwrap();
                writeln!(os).unwrap();
                output_trace_to_stream(&self.trace, self.count, os);
            }
        }
    }
}
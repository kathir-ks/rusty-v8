// Converted from V8 C++ source files:
// Header: N/A
// Implementation: stack_trace_win.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::sync::Mutex;
use std::{ffi::c_void, io::Write, os::windows::prelude::*, ptr, sync::Arc};
use winapi::{
    shared::{
        minwindef::{DWORD, DWORD_PTR, FALSE, ULONG64, TRUE},
        ntdef::CONTEXT,
        windef::HMODULE,
    },
    um::{
        consoleapi::GetConsoleScreenBufferInfo,
        dbghelp::{
            AddrModeFlat, CaptureStackBackTrace, IMAGEHLP_LINE64, PSYMBOL_INFO,
            STACKFRAME64, SYMBOL_INFO, SYMOPT_DEFERRED_LOADS, SYMOPT_LOAD_LINES,
            SYMOPT_UNDNAME,
        },
        errhandlingapi::GetLastError,
        handleapi::GetCurrentProcess,
        processthreadsapi::GetCurrentThread,
        synchapi::InitializeCriticalSection,
        sysinfoapi::GetModuleFileNameW,
        wincon::GetStdHandle,
        wincon::CONSOLE_SCREEN_BUFFER_INFO,
        wincon::STD_ERROR_HANDLE,
        winnt::{
            CONTEXT_CONTROL, CONTEXT_INTEGER, EXCEPTION_CONTINUE_SEARCH,
            EXCEPTION_POINTERS, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64,
            IMAGE_FILE_MACHINE_I386, LPTOP_LEVEL_EXCEPTION_FILTER, MAX_PATH,
        },
    },
};

mod win32 {
    use winapi::shared::minwindef::{BOOL, DWORD};
    use winapi::um::winnt::HANDLE;

    #[link(name = "dbghelp")]
    extern "system" {
        pub fn SymInitialize(ProcessHandle: HANDLE, UserSearchPath: *const u16, fInvadeProcess: BOOL) -> BOOL;
        pub fn SymFromAddr(ProcessHandle: HANDLE, Address: DWORD, Displacement: *mut DWORD, Symbol: *mut winapi::um::dbghelp::SYMBOL_INFO) -> BOOL;
        pub fn SymGetLineFromAddr64(ProcessHandle: HANDLE, Address: DWORD, Displacement: *mut DWORD, Line: *mut winapi::um::dbghelp::IMAGEHLP_LINE64) -> BOOL;
        pub fn SymFunctionTableAccess64(hProcess: HANDLE, AddrBase: DWORD) -> DWORD;
        pub fn SymGetModuleBase64(hProcess: HANDLE, Address: DWORD) -> DWORD;
        pub fn SymSetOptions(options: DWORD) -> DWORD;
        pub fn SymGetSearchPathW(hProcess: HANDLE, path: *mut u16, count: DWORD) -> BOOL;
        pub fn SymSetSearchPathW(hProcess: HANDLE, path: *const u16) -> BOOL;
    }
    #[link(name = "kernel32")]
    extern "system" {
        pub fn SetUnhandledExceptionFilter(
            lpTopLevelExceptionFilter: LPTOP_LEVEL_EXCEPTION_FILTER,
        ) -> LPTOP_LEVEL_EXCEPTION_FILTER;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetModuleHandleW(lpModuleName: *const u16) -> HMODULE;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum StackTraceError {
    SymInitializeFailed(DWORD),
    SymGetSearchPathWFailed(DWORD),
    SymSetSearchPathWFailed(DWORD),
    GetLastError(DWORD),
}

static g_previous_filter: Mutex<Option<LPTOP_LEVEL_EXCEPTION_FILTER>> =
    Mutex::new(None);
static g_dump_stack_in_signal_handler: Mutex<bool> = Mutex::new(true);
static g_initialized_symbols: Mutex<bool> = Mutex::new(false);
static g_init_error: Mutex<DWORD> = Mutex::new(0);

fn StackDumpExceptionFilter(info: *mut EXCEPTION_POINTERS) -> i32 {
    if *g_dump_stack_in_signal_handler.lock().unwrap() {
        let stack_trace = StackTrace::new_with_exception_pointers(info);
        stack_trace.print();
    }

    let previous_filter = *g_previous_filter.lock().unwrap();
    match previous_filter {
        Some(filter) => filter(info),
        None => EXCEPTION_CONTINUE_SEARCH,
    }
}

fn InitializeSymbols() -> Result<bool, StackTraceError> {
    if *g_initialized_symbols.lock().unwrap() {
        return if *g_init_error.lock().unwrap() == 0 {
            Ok(true)
        } else {
            Ok(false) // Previous initialization failed, still return false
        };
    }

    let mut initialized_symbols = g_initialized_symbols.lock().unwrap();
    let mut init_error = g_init_error.lock().unwrap();

    *initialized_symbols = true;

    unsafe {
        win32::SymSetOptions(SYMOPT_DEFERRED_LOADS | SYMOPT_UNDNAME | SYMOPT_LOAD_LINES);
        if win32::SymInitialize(GetCurrentProcess(), ptr::null(), TRUE) == 0 {
            *init_error = GetLastError();
            return Err(StackTraceError::SymInitializeFailed(*init_error));
        }

        const kSymbolsArraySize: usize = 1024;
        let mut symbols_path: Vec<u16> = vec![0; kSymbolsArraySize];

        if win32::SymGetSearchPathW(
            GetCurrentProcess(),
            symbols_path.as_mut_ptr(),
            kSymbolsArraySize as DWORD,
        ) == 0
        {
            let err = GetLastError();
            *init_error = err;
            return Err(StackTraceError::SymGetSearchPathWFailed(err));
        }

        let mut exe_path: Vec<u16> = vec![0; MAX_PATH as usize];
        GetModuleFileNameW(ptr::null_mut(), exe_path.as_mut_ptr(), MAX_PATH as DWORD);

        let exe_path_wstring: Vec<u16> = exe_path.into_iter().take_while(|&c| c != 0).collect();
        let exe_path_string = String::from_utf16(&exe_path_wstring).unwrap();

        let last_slash_index = exe_path_string
            .rfind('\\')
            .or_else(|| exe_path_string.rfind('/'))
            .unwrap_or(0);
        let path_without_filename = &exe_path_string[..last_slash_index];

        let symbols_path_string = String::from_utf16(&symbols_path.into_iter().take_while(|&c| c != 0).collect::<Vec<u16>>()).unwrap();
        let new_path_string = format!("{};{}", symbols_path_string, path_without_filename);

        let mut new_path: Vec<u16> = new_path_string.encode_utf16().collect();
        new_path.push(0);

        if win32::SymSetSearchPathW(GetCurrentProcess(), new_path.as_ptr()) == 0 {
            let err = GetLastError();
            *init_error = err;
            return Err(StackTraceError::SymSetSearchPathWFailed(err));
        }

        *init_error = 0;
        Ok(true)
    }
}

pub fn EnableInProcessStackDumping() -> bool {
    unsafe {
        let mut previous_filter = g_previous_filter.lock().unwrap();
        *previous_filter =
            Some(win32::SetUnhandledExceptionFilter(Some(StackDumpExceptionFilter)));
    }
    *g_dump_stack_in_signal_handler.lock().unwrap() = true;
    InitializeSymbols().unwrap_or(false)
}

pub fn DisableSignalStackDump() {
    *g_dump_stack_in_signal_handler.lock().unwrap() = false;
}

pub struct StackTrace {
    trace: [*mut c_void; 62],
    count_: usize,
}

impl StackTrace {
    pub fn new() -> StackTrace {
        let mut trace: [*mut c_void; 62] = [ptr::null_mut(); 62];
        let count_ = unsafe { CaptureStackBackTrace(0, 62, trace.as_mut_ptr(), ptr::null_mut()) as usize };

        StackTrace { trace, count_ }
    }

    pub fn new_with_exception_pointers(exception_pointers: *mut EXCEPTION_POINTERS) -> StackTrace {
        let context_record = unsafe { (*exception_pointers).ContextRecord };
        StackTrace::new_with_context(context_record)
    }

    pub fn new_with_context(context_record: *const CONTEXT) -> StackTrace {
        let mut stack_trace = StackTrace {
            trace: [ptr::null_mut(); 62],
            count_: 0,
        };
        stack_trace.init_trace(context_record);
        stack_trace
    }

    fn init_trace(&mut self, context_record: *const CONTEXT) {
        unsafe {
            let mut context_copy: CONTEXT;
            std::ptr::copy_nonoverlapping(
                context_record,
                &mut context_copy,
                std::mem::size_of::<CONTEXT>(),
            );
            context_copy.ContextFlags = CONTEXT_INTEGER | CONTEXT_CONTROL;

            let mut stack_frame: STACKFRAME64 = std::mem::zeroed();

            #[cfg(target_arch = "x86_64")]
            {
                stack_frame.AddrPC.Offset = (*context_record).Rip;
                stack_frame.AddrFrame.Offset = (*context_record).Rbp;
                stack_frame.AddrStack.Offset = (*context_record).Rsp;
            }

            #[cfg(target_arch = "aarch64")]
            {
                stack_frame.AddrPC.Offset = (*context_record).Pc;
                stack_frame.AddrFrame.Offset = (*context_record).Fp;
                stack_frame.AddrStack.Offset = (*context_record).Sp;
            }

            #[cfg(target_arch = "x86")]
            {
                stack_frame.AddrPC.Offset = (*context_record).Eip;
                stack_frame.AddrFrame.Offset = (*context_record).Ebp;
                stack_frame.AddrStack.Offset = (*context_record).Esp;
            }

            stack_frame.AddrPC.Mode = AddrModeFlat;
            stack_frame.AddrFrame.Mode = AddrModeFlat;
            stack_frame.AddrStack.Mode = AddrModeFlat;

            let machine_type =
                if cfg!(target_arch = "x86_64") {
                     IMAGE_FILE_MACHINE_AMD64
                } else if cfg!(target_arch = "aarch64") {
                     IMAGE_FILE_MACHINE_ARM64
                } else {
                     IMAGE_FILE_MACHINE_I386
                };

            self.count_ = 0;
            while win32::StackWalk64(
                machine_type as DWORD,
                GetCurrentProcess(),
                GetCurrentThread(),
                &mut stack_frame,
                &mut context_copy,
                ptr::null_mut(),
                Some(std::mem::transmute(win32::SymFunctionTableAccess64 as usize)),
                Some(std::mem::transmute(win32::SymGetModuleBase64 as usize)),
                ptr::null_mut(),
            ) != 0 && self.count_ < self.trace.len()
            {
                self.trace[self.count_] = stack_frame.AddrPC.Offset as *mut c_void;
                self.count_ += 1;
            }

            for i in self.count_..self.trace.len() {
                self.trace[i] = ptr::null_mut();
            }
        }
    }

    pub fn print(&self) {
        self.output_to_stream(&mut std::io::stderr());
    }

    pub fn output_to_stream(&self, os: &mut dyn Write) {
        match InitializeSymbols() {
            Ok(_) => {
                if *g_init_error.lock().unwrap() != 0 {
                    let _ = write!(
                        os,
                        "Error initializing symbols ({}).  Dumping unresolved backtrace:\n",
                        *g_init_error.lock().unwrap()
                    );
                    for i in 0..self.count_ {
                        let _ = write!(os, "\t{:?}\n", self.trace[i]);
                    }
                } else {
                    let _ = write!(os, "\n==== C stack trace ===============================\n\n");
                    self.output_trace_to_stream(os);
                }
            }
            Err(err) => {
                let _ = write!(os, "Error initializing symbols: {:?}\n", err);
            }
        }
    }

    fn output_trace_to_stream(&self, os: &mut dyn Write) {
        for i in 0..self.count_ {
            let frame = self.trace[i] as DWORD_PTR;
            let mut buffer: [u8; 4096] = [0; 4096];
            let symbol: PSYMBOL_INFO =
                unsafe { std::mem::transmute::<*mut u8, PSYMBOL_INFO>(buffer.as_mut_ptr()) };

            unsafe {
                (*symbol).SizeOfStruct = std::mem::size_of::<SYMBOL_INFO>() as u32;
                (*symbol).MaxNameLen = 255;
            }

            let mut sym_displacement: u64 = 0;
            let has_symbol = unsafe {
                win32::SymFromAddr(
                    GetCurrentProcess(),
                    frame as DWORD,
                    &mut sym_displacement as *mut u64 as *mut DWORD,
                    symbol,
                ) != 0
            };

            let mut line: IMAGEHLP_LINE64 = unsafe { std::mem::zeroed() };
            line.SizeOfStruct = std::mem::size_of::<IMAGEHLP_LINE64>() as DWORD;
            let mut line_displacement: DWORD = 0;

            let has_line = unsafe {
                win32::SymGetLineFromAddr64(
                    GetCurrentProcess(),
                    frame as DWORD,
                    &mut line_displacement,
                    &mut line,
                ) != 0
            };

            let _ = write!(os, "\t");

            if has_symbol {
                let name = unsafe {
                    let name_ptr = (*symbol).Name as *const i8;
                    let len = (0..256)
                        .position(|i| *name_ptr.offset(i) == 0)
                        .unwrap_or(256);
                    let slice = std::slice::from_raw_parts(name_ptr as *const u8, len);
                    String::from_utf8_lossy(slice)
                };
                let _ = write!(os, "{} [0x{:p}+{}]", name, self.trace[i], sym_displacement);
            } else {
                let _ = write!(os, "(No symbol) [0x{:p}]", self.trace[i]);
            }

            if has_line {
                let filename = unsafe {
                    let filename_ptr = (*line).FileName as *const i8;
                    let len = (0..256)
                        .position(|i| *filename_ptr.offset(i) == 0)
                        .unwrap_or(256);
                    let slice = std::slice::from_raw_parts(filename_ptr as *const u8, len);
                    String::from_utf8_lossy(slice)
                };

                let _ = write!(os, " ({} : {})", filename, (*line).LineNumber);
            }
            let _ = write!(os, "\n");
        }
    }
}

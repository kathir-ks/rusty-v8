// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::c_char;
use std::process::abort;
use std::ptr;
use std::slice;

// Placeholder for v8-extension.h - Assuming only what's needed is the
// v8::Extension class, but it's not directly used, so omitting it for now.

// Placeholder for v8-primitive.h - Omitted as no direct equivalents are used.

// Placeholder for v8-template.h
// Assuming the v8::FunctionTemplate is only used for creating new functions
// and doesn't have methods that need to be translated directly.

// Placeholder for api.h - Omitted, assuming it provides V8 API functions that
// are implicitly used through the `d8` crate's API
use d8::{
    self, FunctionCallbackInfo, Isolate, Local, Object, String as V8String, Value,
};

// Placeholder for isolate-inl.h and isolate.h -  Assuming these are implicitly
// used through the `d8` crate's `Isolate` struct.

// Placeholder for cov.h - Omitted as coverage functionality isn't implemented.

// Placeholder for sandbox.h and sandbox/testing.h - Omitted as sandbox
// functionality isn't implemented.

#[cfg(target_os = "linux")]
extern "C" {
    fn signal(signum: i32, handler: extern "C" fn(i32)) -> *mut u8;
    fn sleep(seconds: u32);
}

#[cfg(target_os = "linux")]
extern "C" fn signal_handler(_: i32) {
    // Placeholder for signal handler logic
}

pub struct FuzzilliExtension {}

impl FuzzilliExtension {
    pub fn get_native_function_template(
        isolate: &mut Isolate,
        str: Local<V8String>,
    ) -> d8::FunctionTemplate {
        d8::FunctionTemplate::new(isolate, Self::fuzzilli)
    }

    // This is marked as pub static to be accessible from the extension.
    pub fn fuzzilli(info: FunctionCallbackInfo) {
        let isolate = info.isolate();
        let args = info.args();
        let operation_value = args.get(isolate, 0);

        if operation_value.is_undefined() || operation_value.is_null() {
            return;
        }

        let operation_string = operation_value.to_string(isolate).unwrap();
        let operation = operation_string.to_rust_string_lossy(isolate);

        if operation == "FUZZILLI_CRASH" {
            let arg_value = args.get(isolate, 1);

            let arg = arg_value.integer_value(isolate).unwrap_or(0);

            match arg {
                0 => {
                    // IMMEDIATE_CRASH();
                    abort();
                }
                1 => {
                    assert!(false);
                }
                2 => {
                    debug_assert!(false);
                }
                3 => {
                    // Access an invalid address.
                    // The cast ensures that this works correctly on both 32-bit and 64-bit.
                    let addr: usize = 0x414141414141;
                    let mut ptr = addr as *mut u8;
                    for _ in 0..1024 {
                        unsafe {
                            *ptr = b'A';
                            ptr = ptr.add(1 * 1024 * 1024); // MB
                        }
                    }
                }
                4 => {
                    // Use-after-free, likely only crashes in ASan builds.
                    let mut vec = Box::new(vec![0i32; 4]);
                    let vec_ptr = Box::into_raw(vec);
                    unsafe {
                        drop(Box::from_raw(vec_ptr)); // Simulate delete
                        let _val = (*vec_ptr)[0];  // Attempt to access freed memory
                    }
                }
                5 => {
                    // Out-of-bounds access (1), likely only crashes in ASan or
                    // "hardened"/"safe" libc++ builds.
                    let vec = vec![0i32; 5];
                    let _val = vec[5];
                }
                6 => {
                    // Out-of-bounds access (2), likely only crashes in ASan builds.
                    let mut vec = vec![0i32; 6];
                    let data = vec.as_mut_ptr() as *mut u8;
                    let size = 0x100;
                    unsafe {
                        std::ptr::write_bytes(data, 42, size);
                    }
                }
                7 => {
                    if v8_flags::hole_fuzzing {
                        // This should crash with a segmentation fault only
                        // when --hole-fuzzing is used.
                        let addr: usize = 0x414141414141;
                        let mut ptr = addr as *mut u8;
                        for _ in 0..1024 {
                            unsafe {
                                *ptr = b'A';
                                ptr = ptr.add(1 * 1024 * 1024 * 1024); // GB
                            }
                        }
                    }
                }
                8 => {
                    // This allows Fuzzilli to check that DEBUG is defined, which should be
                    // the case if dcheck_always_on is set. This is useful for fuzzing as
                    // there are some integrity checks behind DEBUG.
                    #[cfg(debug_assertions)]
                    {
                        abort();
                    }
                }
                _ => {}
            }
        } else if operation == "FUZZILLI_PRINT" {
            let string_value = args.get(isolate, 1);
            let string = string_value.to_string(isolate).unwrap();
            let output = string.to_rust_string_lossy(isolate);

            lazy_static::lazy_static! {
                static ref FZLIOUT: Mutex<Option<std::fs::File>> = Mutex::new(None);
            }

            let mut fzliout = FZLIOUT.lock().unwrap();
            if fzliout.is_none() {
                match unsafe { std::fs::File::from_raw_fd(3) } {
                    Ok(file) => {
                        *fzliout = Some(file);
                    }
                    Err(_) => {
                        eprintln!("Fuzzer output channel not available, printing to stdout instead");
                        *fzliout = None; // Fallback to stdout later
                    }
                };
            }

            match &mut *fzliout {
                Some(file) => {
                    writeln!(file, "{}", output).unwrap();
                }
                None => {
                    println!("{}", output); // Fallback to stdout
                }
            };

            io::stdout().flush().unwrap(); // Ensure stdout is flushed
        }
    }
}

mod v8_flags {
    pub static hole_fuzzing: bool = false; // Placeholder, set appropriately
}

use std::sync::Mutex;
use std::os::unix::io::FromRawFd;
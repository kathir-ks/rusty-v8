// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ffi::{CString, NulError};
use std::fs::File;
use std::io::{self, Write, Stdout, stdout};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Mutex;

// Placeholder for flags crate.  Replace with actual flags handling.
mod flags {
    use std::sync::atomic::{AtomicBool, Ordering};

    static REDIRECT_CODE_TRACES: AtomicBool = AtomicBool::new(false);
    static REDIRECT_CODE_TRACES_TO: Mutex<Option<String>> = Mutex::new(None);
    
    pub struct Flags {}

    impl Flags {
        pub fn redirect_code_traces() -> bool {
            REDIRECT_CODE_TRACES.load(Ordering::Relaxed)
        }

        pub fn redirect_code_traces_to() -> Option<String> {
            REDIRECT_CODE_TRACES_TO.lock().unwrap().clone()
        }
    }

    lazy_static::lazy_static! {
        pub static ref V8_FLAGS: Flags = Flags {};
    }

    pub fn set_redirect_code_traces(value: bool) {
        REDIRECT_CODE_TRACES.store(value, Ordering::Relaxed);
    }

    pub fn set_redirect_code_traces_to(value: Option<String>) {
        let mut guard = REDIRECT_CODE_TRACES_TO.lock().unwrap();
        *guard = value;
    }
}

use flags::V8_FLAGS;

// Placeholder for base crate.  Replace with actual base functionality.
mod base {
    use std::env;
    use std::ffi::CString;
    use std::io;
    use std::fs::File;
    use std::os::unix::prelude::AsRawFd;
    use libc::dup2;

    pub struct OS {}

    impl OS {
        pub fn get_current_process_id() -> i32 {
            unsafe { libc::getpid() }
        }

        pub fn fopen(filename: &str, mode: &str) -> Result<File, io::Error> {
            File::options().append(true).create(true).open(filename)
        }

        pub fn fclose(file: File) -> Result<(), io::Error> {
            drop(file);
            Ok(())
        }
    }

    pub fn snprintf(format: &str, args: &[&dyn std::fmt::Display]) -> String {
        let mut result = String::new();
        let mut arg_index = 0;

        for part in format.split('%') {
            if arg_index < args.len() {
                if let Some(c) = part.chars().next() {
                  if c == 'd'{
                    result.push_str(&args[arg_index].to_string());
                    result.push_str(&part[1..]);
                  } else if c == 's'{
                    result.push_str(&args[arg_index].to_string());
                    result.push_str(&part[1..]);
                  } else {
                    result.push_str("%");
                    result.push_str(part);
                  }
                } else {
                    result.push_str("%");
                    result.push_str(part);
                }
                arg_index += 1;
            } else {
                result.push_str(part);
            }
        }

        result
    }
}

// Placeholder for utils crate.  Replace with actual utils functionality.
mod utils {
    use std::io::{StdoutLock, self};

    pub struct StdoutStream<'a> {
        lock: StdoutLock<'a>,
    }

    impl<'a> StdoutStream<'a> {
        pub fn new(lock: StdoutLock<'a>) -> Self {
            Self { lock }
        }
    }

    pub struct OFStream {
        file: std::fs::File,
    }

    impl OFStream {
        pub fn new(file: std::fs::File) -> Self {
            Self { file }
        }
    }

    impl std::io::Write for OFStream {
      fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
      }
      fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
      }
    }
}

use utils::{StdoutStream, OFStream};

pub struct CodeTracer {
    filename: String,
    file: Option<File>,
    scope_depth: AtomicI32,
    stdout: bool
}

impl CodeTracer {
    pub fn new(isolate_id: i32) -> Self {
        if !Self::should_redirect() {
            return Self {
                filename: String::new(),
                file: None,
                scope_depth: AtomicI32::new(0),
                stdout: true
            };
        }

        let mut filename = String::new();

        if let Some(redirect_to) = V8_FLAGS.redirect_code_traces_to() {
            filename = redirect_to;
        } else if isolate_id >= 0 {
            filename = base::snprintf("code-%d-%d.asm", &[&base::OS::get_current_process_id(), &isolate_id]);
        } else {
            filename = base::snprintf("code-%d.asm", &[&base::OS::get_current_process_id()]);
        }

        // No direct equivalent of WriteChars in Rust, so just ensure file is created/truncated
        if let Ok(_file) = std::fs::File::create(&filename){}

        Self {
            filename,
            file: None,
            scope_depth: AtomicI32::new(0),
            stdout: false
        }
    }

    fn should_redirect() -> bool {
        V8_FLAGS.redirect_code_traces()
    }

    pub fn open_file(&mut self) -> Result<(), io::Error> {
        if !Self::should_redirect() {
            return Ok(());
        }

        if self.file.is_none() {
            self.file = match base::OS::fopen(&self.filename, "ab") {
                Ok(file) => Some(file),
                Err(e) => {
                    eprintln!("could not open file. If on Android, try passing --redirect-code-traces-to=/sdcard/Download/<file-name>");
                    return Err(e);
                }
            };
        }

        self.scope_depth.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn close_file(&mut self) -> Result<(), io::Error> {
        if !Self::should_redirect() {
            return Ok(());
        }

        if self.scope_depth.fetch_sub(1, Ordering::Relaxed) == 1 {
            if let Some(file) = self.file.take() {
                base::OS::fclose(file)?;
                self.file = None;
            }
        }
        Ok(())
    }

    pub fn file(&self) -> Option<&File> {
        self.file.as_ref()
    }

    pub fn stdout_mode(&self) -> bool {
        self.stdout
    }
}

pub struct Scope<'a> {
    tracer: &'a mut CodeTracer,
}

impl<'a> Scope<'a> {
    pub fn new(tracer: &'a mut CodeTracer) -> Self {
        tracer.open_file().expect("Failed to open file");
        Scope { tracer }
    }

    pub fn file(&self) -> Option<&File> {
        self.tracer.file()
    }
}

impl<'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        self.tracer.close_file().expect("Failed to close file");
    }
}

pub struct StreamScope<'a> {
    scope: Scope<'a>,
    stdout_stream: Option<StdoutStream<'a>>,
    file_stream: Option<OFStream>,
}

impl<'a> StreamScope<'a> {
    pub fn new(tracer: &'a mut CodeTracer) -> Self {
        let scope = Scope::new(tracer);
        let stdout_stream;
        let file_stream;

        if tracer.stdout_mode() {
            stdout_stream = Some(StdoutStream::new(std::io::stdout().lock()));
            file_stream = None;
        } else {
            stdout_stream = None;
            let file = scope.file().map(|f| f.try_clone().expect("Failed to clone file")).expect("File should be open");
            file_stream = Some(OFStream::new(file));
        }

        Self {
            scope,
            stdout_stream,
            file_stream,
        }
    }

    pub fn stream(&mut self) -> &mut dyn std::io::Write {
        if let Some(stream) = &mut self.stdout_stream {
            &mut stream.lock
        } else {
            self.file_stream.as_mut().expect("Stream should be initialized")
        }
    }
}
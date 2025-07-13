// Converted from V8 C++ source files:
// Header: N/A
// Implementation: stack_trace_fuchsia.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod debug {
    use std::fmt;
    use std::io::{self, Write};

    pub fn enable_in_process_stack_dumping() -> bool {
        // The system crashlogger captures and prints backtraces which are then
        // symbolized by a host-side script that runs addr2line. Because symbols are
        // not available on device, there's not much use in implementing in-process
        // capture.
        false
    }

    pub fn disable_signal_stack_dump() {}

    #[derive(Debug)]
    pub struct StackTrace {
        trace: Vec<usize>,
        count_: usize,
    }

    impl StackTrace {
        pub fn new() -> Self {
            StackTrace {
                trace: Vec::new(),
                count_: 0,
            }
        }

        pub fn print(&self) {
            let backtrace = self.to_string();
            print!("{}", backtrace);
            io::stdout().flush().unwrap();
        }

        pub fn output_to_stream<W: Write>(&self, os: &mut W) -> io::Result<()> {
            for i in 0..self.count_ {
                writeln!(os, "#{:2} {:#x}", i, self.trace[i])?;
            }
            Ok(())
        }

        pub fn record(&mut self) {
            let mut frames: Vec<usize> = Vec::with_capacity(32);
            let result = backtrace::trace(|frame| {
                backtrace::resolve_frame(frame, |symbol| {
                    if let Some(addr) = symbol.addr() {
                        frames.push(addr as usize);
                    }
                });
                true // keep going to the next frame
            });

            match result {
                _ => {
                    self.trace = frames;
                    self.count_ = self.trace.len();
                }
            }
        }
    }

    impl fmt::Display for StackTrace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for i in 0..self.count_ {
                writeln!(f, "#{:2} {:#x}", i, self.trace[i])?;
            }
            Ok(())
        }
    }
}

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod logging {
    pub mod internal {
        use crate::source_location::SourceLocation;

        #[cfg(debug_assertions)]
        macro_rules! v8_dcheck {
            ($file:expr, $line:expr, $message:expr) => {
                if !($message) {
                    eprintln!("DCheck failed: {} at {}:{}", stringify!($message), $file, $line);
                    panic!("DCheck failed");
                }
            };
        }

        #[cfg(not(debug_assertions))]
        macro_rules! v8_dcheck {
            ($file:expr, $line:expr, $message:expr) => {};
        }
        
        #[cfg(debug_assertions)]
        macro_rules! v8_fatal {
            ($file:expr, $line:expr, $message:expr, $($arg:tt)*) => {
                eprintln!("Fatal error: {} at {}:{}: {}", format_args!($message, $($arg)*), $file, $line, stringify!($message));
                panic!("Fatal error");
            };
        }

        #[cfg(all(not(debug_assertions), not(feature = "official_build")))]
        macro_rules! v8_fatal {
            ($message:expr, $($arg:tt)*) => {
                eprintln!("Fatal error: {}", format_args!($message, $($arg)*));
                panic!("Fatal error");
            };
        }

        #[cfg(all(not(debug_assertions), feature = "official_build"))]
        macro_rules! v8_fatal {
            ($message:expr) => {
                eprintln!("Fatal error: ignored");
                panic!("Fatal error");
            };
        }
        
        pub fn dcheck_impl(message: &str, loc: &SourceLocation) {
            v8_dcheck!(loc.file_name, loc.line, message);
        }

        pub fn fatal_impl(message: &str, loc: &SourceLocation) {
            #[cfg(debug_assertions)]
            {
                v8_fatal!(loc.file_name, loc.line, "Check failed: %s.", message);
            }

            #[cfg(all(not(debug_assertions), not(feature = "official_build")))]
            {
                 v8_fatal!("Check failed: %s.", message);
            }

            #[cfg(all(not(debug_assertions), feature = "official_build"))]
            {
                v8_fatal!("ignored");
            }
        }
    }
}

pub mod source_location {
    #[derive(Debug)]
    pub struct SourceLocation {
        pub file_name: &'static str,
        pub line: u32,
    }

    impl SourceLocation {
        pub const fn new(file_name: &'static str, line: u32) -> Self {
            Self { file_name, line }
        }
    }
}
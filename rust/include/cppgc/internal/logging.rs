// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod v8config {
    // Placeholder for v8config.h content.
    // This would typically include conditional compilation flags.
    // Example:
    // pub const V8_ENABLE_DEBUG: bool = true;
}

mod source_location {
    /// Represents a source code location.
    #[derive(Debug, Clone)]
    pub struct SourceLocation {
        pub file: &'static str,
        pub line: u32,
        pub column: u32,
    }

    impl SourceLocation {
        /// Creates a new SourceLocation.
        pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
            SourceLocation { file, line, column }
        }

        /// Returns the current source location.
        //  In C++, this used compiler intrinsics, which Rust's macro system can reproduce
        #[macro_export]
        macro_rules! current_source_location {
            () => {
                $crate::source_location::SourceLocation {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }
            };
        }

        pub const fn Current() -> Self {
            current_source_location!()
        }
    }
}

pub mod internal {
    use crate::source_location::SourceLocation;

    // Placeholder for V8_EXPORT.  In V8 this is used for visibility.  In rust, public functions
    // are visible.
    
    /// DCheck implementation.
    ///
    /// This function is called when a DCHECK fails.
    pub fn dcheck_impl(message: &str, location: SourceLocation) {
        eprintln!("DCheck failed at {}:{}: {}", location.file, location.line, message);
        #[cfg(debug_assertions)]
        panic!("DCheck failed: {}", message);
        #[cfg(not(debug_assertions))]
        {
          // In release mode, just log the error but don't panic
          eprintln!("DCheck failed in release mode: {}", message);
        }
    }

    /// Fatal error implementation.
    ///
    /// This function is called when a fatal error occurs.
    pub fn fatal_impl(message: &str, location: SourceLocation) -> ! {
        eprintln!("Fatal error at {}:{}: {}", location.file, location.line, message);
        panic!("Fatal error: {}", message);
    }

    /// Used to ignore unused variables.
    pub struct EatParams<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> EatParams<T> {
        pub fn new() -> Self {
            EatParams {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    /// Checks a condition and calls dcheck_impl if it fails.
    #[macro_export]
    macro_rules! cppgc_dcheck_msg {
        ($condition:expr, $message:expr) => {
            if cfg!(debug_assertions) {
                if !($condition) {
                    $crate::internal::dcheck_impl($message, $crate::source_location::current_source_location!());
                }
            } else {
                let _ = $crate::internal::EatParams::new();
                let _ = ($condition, $message);
            }
        };
    }

    /// Checks a condition and calls dcheck_impl with the condition as the message if it fails.
    #[macro_export]
    macro_rules! cppgc_dcheck {
        ($condition:expr) => {
            $crate::cppgc_dcheck_msg!($condition, stringify!($condition));
        };
    }

    /// Checks a condition and calls fatal_impl if it fails.
    #[macro_export]
    macro_rules! cppgc_check_msg {
        ($condition:expr, $message:expr) => {
            if !($condition) {
                $crate::internal::fatal_impl($message, $crate::source_location::current_source_location!());
            }
        };
    }

    /// Checks a condition and calls fatal_impl with the condition as the message if it fails.
    #[macro_export]
    macro_rules! cppgc_check {
        ($condition:expr) => {
            $crate::cppgc_check_msg!($condition, stringify!($condition));
        };
    }
}
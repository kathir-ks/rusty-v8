// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Encapsulates source location information. Mimics C++20's
/// `std::source_location`.
#[derive(Clone, Debug, Default)]
pub struct SourceLocation {
    function_: Option<&'static str>,
    file_: Option<&'static str>,
    line_: usize,
}

impl SourceLocation {
    /// Construct source location information corresponding to the location of the
    /// call site.
    #[cfg(all(
        any(target_env = "gnu", target_env = "msvc", target_env = "clang"),
        feature = "source_location"
    ))]
    pub const fn current(function: &'static str, file: &'static str, line: usize) -> Self {
        SourceLocation {
            function_: Some(function),
            file_: Some(file),
            line_: line,
        }
    }

    #[cfg(not(all(
        any(target_env = "gnu", target_env = "msvc", target_env = "clang"),
        feature = "source_location"
    )))]
    pub const fn current() -> Self {
        SourceLocation::default()
    }

    /// Constructs unspecified source location information.
    pub const fn new() -> Self {
        SourceLocation {
            function_: None,
            file_: None,
            line_: 0,
        }
    }

    /// Returns the name of the function associated with the position represented
    /// by this object, if any.
    ///
    /// \returns the function name as cstring.
    pub const fn function(&self) -> Option<&'static str> {
        self.function_
    }

    /// Returns the name of the current source file represented by this object.
    ///
    /// \returns the file name as cstring.
    pub const fn file_name(&self) -> Option<&'static str> {
        self.file_
    }

    /// Returns the line number represented by this object.
    ///
    /// \returns the line number.
    pub const fn line(&self) -> usize {
        self.line_
    }

    /// Returns a human-readable string representing this object.
    ///
    /// \returns a human-readable string representing source location information.
    pub fn to_string(&self) -> String {
        match (self.function_, self.file_) {
            (Some(function), Some(file)) => format!("{}@{}::{}", function, file, self.line_),
            _ => String::new(),
        }
    }
}
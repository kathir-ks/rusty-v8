// Converted from V8 C++ source files:
// Header: v8-source-location.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[allow(non_camel_case_types)]
pub struct V8_EXPORT {}

#[derive(Clone, Copy, Debug)]
pub struct SourceLocation {
  function_: Option<&'static str>,
  file_: Option<&'static str>,
  line_: usize,
}

impl SourceLocation {
  #[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(target_os = "linux", target_os = "android")
  ))]
  pub const fn current(function: &'static str, file: &'static str, line: usize) -> Self {
    SourceLocation {
      function_: Some(function),
      file_: Some(file),
      line_: line,
    }
  }

  #[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(target_os = "linux", target_os = "android")
  )))]
  pub const fn current() -> Self {
    SourceLocation {
      function_: None,
      file_: None,
      line_: 0,
    }
  }

  pub const fn new() -> Self {
    SourceLocation {
      function_: None,
      file_: None,
      line_: 0,
    }
  }

  pub const fn function(&self) -> Option<&'static str> {
    self.function_
  }

  pub const fn file_name(&self) -> Option<&'static str> {
    self.file_
  }

  pub const fn line(&self) -> usize {
    self.line_
  }

  pub fn to_string(&self) -> String {
    match (self.function_, self.file_) {
      (Some(function), Some(file)) => format!("{}@{}::{}", function, file, self.line_),
      _ => String::new(),
    }
  }

  const fn from_parts(function: &'static str, file: &'static str, line: usize) -> Self {
    SourceLocation {
      function_: Some(function),
      file_: Some(file),
      line_: line,
    }
  }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self::new()
    }
}

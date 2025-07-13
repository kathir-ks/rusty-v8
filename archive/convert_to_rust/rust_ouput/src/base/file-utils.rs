// Converted from V8 C++ source files:
// Header: file-utils.h
// Implementation: file-utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::path::Path;
    use std::ffi::OsStr;
    use std::borrow::Cow;
    use std::os::unix::ffi::OsStrExt;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::fs;
    use std::io;

    #[cfg(target_os = "windows")]
    fn is_directory_separator(c: char) -> bool {
        c == '\\' || c == '/'
    }

    #[cfg(not(target_os = "windows"))]
    fn is_directory_separator(c: char) -> bool {
        c == '/'
    }

    pub fn relative_path(exec_path: &str, name: &str) -> Result<String, io::Error> {
        if exec_path.is_empty() {
            return Ok(name.to_string());
        }

        let mut basename_start = exec_path.len();
        while basename_start > 0 {
            if is_directory_separator(exec_path.chars().nth(basename_start - 1).unwrap()) {
                break;
            }
            basename_start -= 1;
        }

        let mut result = String::new();
        if basename_start > 0 {
            result.push_str(&exec_path[..basename_start]);
        }
        result.push_str(name);

        Ok(result)
    }

}

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::path::{Path, PathBuf};

/// Determines the relative path from the directory of the executable to a given name.
///
/// # Arguments
///
/// * `exec_path` - The path to the executable.
/// * `name` - The name to create the relative path to.
///
/// # Returns
///
/// A `PathBuf` containing the relative path, or `None` if `exec_path` is empty.
pub fn relative_path(exec_path: &str, name: &str) -> Option<PathBuf> {
    if exec_path.is_empty() {
        return None;
    }

    let path = Path::new(exec_path);
    let basename_start = path
        .parent()
        .map(|p| p.as_os_str().len())
        .unwrap_or(0);

    let mut buffer = PathBuf::new();
    if basename_start > 0 {
        if let Some(parent) = path.parent() {
            buffer.push(parent);
        }
    }

    buffer.push(name);

    Some(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_path() {
        assert_eq!(
            relative_path("/path/to/executable", "name"),
            Some(PathBuf::from("/path/to/name"))
        );
        assert_eq!(
            relative_path("executable", "name"),
            Some(PathBuf::from("name"))
        );
        assert_eq!(
            relative_path("/executable", "name"),
            Some(PathBuf::from("/name"))
        );
        assert_eq!(
            relative_path("", "name"),
            None
        );
    }
}
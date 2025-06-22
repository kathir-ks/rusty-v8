// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Module for source positions and related data.
pub mod source_positions {
    use std::fmt;
    use std::path::{Path, PathBuf};

    /// Represents a source identifier.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SourceId(i32);

    impl SourceId {
        /// Returns an invalid SourceId.
        pub fn invalid() -> Self {
            SourceId(-1)
        }

        /// Checks if the SourceId is valid.
        pub fn is_valid(&self) -> bool {
            self.0 != -1
        }
    }

    /// Represents a line and column position in a source file.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LineAndColumn {
        /// Offset in the source file.
        pub offset: i32,
        /// Line number.
        pub line: i32,
        /// Column number.
        pub column: i32,
    }

    impl LineAndColumn {
        /// Represents an unknown offset.
        pub const UNKNOWN_OFFSET: i32 = -1;

        /// Returns an invalid LineAndColumn.
        pub fn invalid() -> Self {
            LineAndColumn {
                offset: -1,
                line: -1,
                column: -1,
            }
        }

        /// Creates a LineAndColumn with an unknown offset.
        pub fn with_unknown_offset(line: i32, column: i32) -> Self {
            LineAndColumn {
                offset: Self::UNKNOWN_OFFSET,
                line,
                column,
            }
        }
    }

    /// Represents a position in a source file.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SourcePosition {
        /// Source identifier.
        pub source: SourceId,
        /// Start position.
        pub start: LineAndColumn,
        /// End position.
        pub end: LineAndColumn,
    }

    impl SourcePosition {
        /// Returns an invalid SourcePosition.
        pub fn invalid() -> Self {
            SourcePosition {
                source: SourceId::invalid(),
                start: LineAndColumn::invalid(),
                end: LineAndColumn::invalid(),
            }
        }

        /// Compares the start position ignoring the column.
        pub fn compare_start_ignore_column(&self, pos: &SourcePosition) -> bool {
            self.start.line == pos.start.line && self.source == pos.source
        }

        /// Checks if the given LineAndColumn is contained within the SourcePosition.
        pub fn contains(&self, pos: LineAndColumn) -> bool {
            if pos.line < self.start.line || pos.line > self.end.line {
                return false;
            }

            if pos.line == self.start.line && pos.column < self.start.column {
                return false;
            }
            if pos.line == self.end.line && pos.column >= self.end.column {
                return false;
            }
            true
        }
    }

    /// Thread-local variable for the current source file.
    thread_local! {
        pub static CURRENT_SOURCE_FILE: std::cell::RefCell<SourceId> = std::cell::RefCell::new(SourceId::invalid());
        pub static CURRENT_SOURCE_POSITION: std::cell::RefCell<SourcePosition> = std::cell::RefCell::new(SourcePosition::invalid());
    }

    /// Maps SourceIds to file paths.
    pub struct SourceFileMap {
        sources: Vec<String>,
        v8_root: String,
    }

    impl SourceFileMap {
        /// Creates a new SourceFileMap.
        pub fn new(v8_root: String) -> Self {
            SourceFileMap {
                sources: Vec::new(),
                v8_root,
            }
        }

        /// Returns the path from the V8 root for the given SourceId.
        pub fn path_from_v8_root(file: SourceId) -> String {
            // Implement the logic to retrieve the path based on the file SourceId
            // Access the `sources` vector to get the path
            // Handle the case where the SourceId is invalid or out of bounds
            // For now, return an empty string as a placeholder
            if !file.is_valid() {
                return String::new();
            }
            if file.0 as usize >= Self::all_sources_static().len() {
                return String::new();
            }
            Self::all_sources_static()[file.0 as usize].clone()
        }

        /// Returns the path from the V8 root without the extension.
        pub fn path_from_v8_root_without_extension(file: SourceId) -> String {
            let path = Self::path_from_v8_root(file);
            if let Some(index) = path.rfind('.') {
                path[..index].to_string()
            } else {
                path
            }
        }

        /// Returns the absolute path for the given SourceId.
        pub fn absolute_path(file: SourceId) -> String {
            let relative_path = Self::path_from_v8_root(file);
            let mut absolute_path = PathBuf::from(&Self::get_v8_root_static());
            absolute_path.push(relative_path);
            absolute_path.to_string_lossy().into_owned()
        }

        /// Adds a source path to the map and returns its SourceId.
        pub fn add_source(path: String) -> SourceId {
            let mut all_sources = Self::all_sources_static();
            if let Some(index) = all_sources.iter().position(|s| s == &path) {
                return SourceId(index as i32);
            }
            all_sources.push(path);
            SourceId((all_sources.len() - 1) as i32)
        }

        /// Gets the SourceId for a given path.
        pub fn get_source_id(path: &str) -> SourceId {
            let all_sources = Self::all_sources_static();
            if let Some(index) = all_sources.iter().position(|s| s == path) {
                return SourceId(index as i32);
            }
            SourceId::invalid()
        }

        /// Returns a vector of all SourceIds.
        pub fn all_sources() -> Vec<SourceId> {
            Self::all_sources_static().iter().enumerate().map(|(i, _)| SourceId(i as i32)).collect()
        }

        /// Checks if a file relative to the V8 root exists.
        pub fn file_relative_to_v8_root_exists(path: &str) -> bool {
            let mut absolute_path = PathBuf::from(&Self::get_v8_root_static());
            absolute_path.push(path);
            absolute_path.exists()
        }

        // Static accessors to the map

        fn all_sources_static() -> &'static mut Vec<String> {
            static mut ALL_SOURCES: Vec<String> = Vec::new();
            unsafe { &mut ALL_SOURCES }
        }

        fn get_v8_root_static() -> &'static String {
            static mut V8_ROOT: String = String::new();
            unsafe { &mut V8_ROOT }
        }

        pub fn initialize_v8_root(v8_root: String) {
            unsafe {
                SourceFileMap::get_v8_root_static().clear();
                SourceFileMap::get_v8_root_static().push_str(&v8_root);
            }
        }
    }

    /// Formats a SourcePosition as a string.
    pub fn position_as_string(pos: SourcePosition) -> String {
        format!(
            "{}:{}:{}",
            SourceFileMap::path_from_v8_root(pos.source),
            pos.start.line + 1,
            pos.start.column + 1
        )
    }

    impl fmt::Display for SourcePosition {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "https://source.chromium.org/chromium/chromium/src/+/main:v8/{}?l={}&c={}",
                SourceFileMap::path_from_v8_root(self.source),
                self.start.line + 1,
                self.start.column + 1
            )
        }
    }
}
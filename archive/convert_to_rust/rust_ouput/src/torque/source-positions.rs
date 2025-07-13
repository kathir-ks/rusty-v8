// Converted from V8 C++ source files:
// Header: source-positions.h
// Implementation: source-positions.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod torque {
    use std::fmt;
    use std::io::ErrorKind;
    use std::path::Path;
    use std::sync::Mutex;
    use std::{fs, io};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SourceId(i32);

    impl SourceId {
        pub fn invalid() -> Self {
            SourceId(-1)
        }
        pub fn is_valid(&self) -> bool {
            self.0 != -1
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LineAndColumn {
        pub offset: i32,
        pub line: i32,
        pub column: i32,
    }

    impl LineAndColumn {
        pub const K_UNKNOWN_OFFSET: i32 = -1;

        pub fn invalid() -> Self {
            LineAndColumn {
                offset: -1,
                line: -1,
                column: -1,
            }
        }

        pub fn with_unknown_offset(line: i32, column: i32) -> Self {
            LineAndColumn {
                offset: Self::K_UNKNOWN_OFFSET,
                line,
                column,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SourcePosition {
        pub source: SourceId,
        pub start: LineAndColumn,
        pub end: LineAndColumn,
    }

    impl SourcePosition {
        pub fn invalid() -> Self {
            SourcePosition {
                source: SourceId::invalid(),
                start: LineAndColumn::invalid(),
                end: LineAndColumn::invalid(),
            }
        }

        pub fn compare_start_ignore_column(&self, pos: &SourcePosition) -> bool {
            self.start.line == pos.start.line && self.source == pos.source
        }

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

    thread_local! {
        pub static CURRENT_SOURCE_FILE: std::cell::RefCell<SourceId> =
            std::cell::RefCell::new(SourceId::invalid());
        pub static CURRENT_SOURCE_POSITION: std::cell::RefCell<SourcePosition> =
            std::cell::RefCell::new(SourcePosition::invalid());
    }

    pub struct SourceFileMap {
        sources: Mutex<Vec<String>>,
        v8_root: String,
    }

    impl SourceFileMap {
        pub fn new(v8_root: String) -> Self {
            SourceFileMap {
                sources: Mutex::new(Vec::new()),
                v8_root,
            }
        }

        fn get_instance() -> &'static SourceFileMap {
            use std::sync::{Once, Mutex, MutexGuard};
            use std::collections::HashMap;

            static mut INSTANCE: *const SourceFileMap = 0 as *const SourceFileMap;
            static ONCE: Once = Once::new();

            fn initialize_instance() -> SourceFileMap {
                SourceFileMap::new("".to_string())
            }

            unsafe {
                ONCE.call_once(|| {
                    let instance = initialize_instance();
                    INSTANCE = std::mem::transmute(Box::new(instance));
                });
                &*INSTANCE
            }
        }

        pub fn path_from_v8_root(file: SourceId) -> String {
            let sources = SourceFileMap::get_instance().sources.lock().unwrap();
            if !file.is_valid() {
                panic!("Invalid SourceId");
            }
            sources[file.0 as usize].clone()
        }

        pub fn absolute_path(file: SourceId) -> String {
            let root_path = SourceFileMap::path_from_v8_root(file);
            if root_path.starts_with("file://") {
                return root_path;
            }
            SourceFileMap::get_instance().v8_root.clone() + "/" + &root_path
        }

        pub fn path_from_v8_root_without_extension(file: SourceId) -> Result<String, String> {
            let mut path_from_root = SourceFileMap::path_from_v8_root(file);
            if !path_from_root.ends_with(".tq") {
                return Err(format!("Not a .tq file: {}", path_from_root));
            }
            path_from_root.truncate(path_from_root.len() - 3);
            Ok(path_from_root)
        }

        pub fn add_source(path: String) -> SourceId {
            let mut sources = SourceFileMap::get_instance().sources.lock().unwrap();
            sources.push(path);
            SourceId((sources.len() - 1) as i32)
        }

        pub fn get_source_id(path: &str) -> SourceId {
            let sources = SourceFileMap::get_instance().sources.lock().unwrap();
            for (i, source) in sources.iter().enumerate() {
                if source == path {
                    return SourceId(i as i32);
                }
            }
            SourceId::invalid()
        }

        pub fn all_sources() -> Vec<SourceId> {
            let sources = SourceFileMap::get_instance().sources.lock().unwrap();
            let mut result = Vec::with_capacity(sources.len());
            for i in 0..sources.len() {
                result.push(SourceId(i as i32));
            }
            result
        }

        pub fn file_relative_to_v8_root_exists(path: &str) -> bool {
            let file = SourceFileMap::get_instance().v8_root.clone() + "/" + path;
            fs::metadata(file).is_ok()
        }
    }

    pub fn position_as_string(pos: SourcePosition) -> String {
        SourceFileMap::path_from_v8_root(pos.source)
            + ":"
            + &(pos.start.line + 1).to_string()
            + ":"
            + &(pos.start.column + 1).to_string()
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

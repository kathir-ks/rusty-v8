// Converted from V8 C++ source files:
// Header: v8-debugger-id.h
// Implementation: v8-debugger-id.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_inspector {
    pub struct V8DebuggerId {
        first: i64,
        second: i64,
    }

    impl V8DebuggerId {
        pub fn new(pair: (i64, i64)) -> Self {
            V8DebuggerId {
                first: pair.0,
                second: pair.1,
            }
        }

        pub fn to_string(&self) -> String {
            format!("{}.{}", self.first, self.second)
        }

        pub fn is_valid(&self) -> bool {
            self.first != 0 || self.second != 0
        }

        pub fn pair(&self) -> (i64, i64) {
            (self.first, self.second)
        }
    }
}

pub mod internal {
    use super::v8_inspector::V8DebuggerId;

    pub struct V8DebuggerIdInternal {
        m_debugger_id: V8DebuggerId,
    }

    impl V8DebuggerIdInternal {
        pub fn new() -> Self {
            V8DebuggerIdInternal {
                m_debugger_id: V8DebuggerId::new((0, 0)),
            }
        }

        pub fn from_pair(pair: (i64, i64)) -> Self {
            V8DebuggerIdInternal {
                m_debugger_id: V8DebuggerId::new(pair),
            }
        }

        pub fn from_string(debugger_id: &String) -> Self {
            let parts: Vec<&str> = debugger_id.split('.').collect();
            if parts.len() != 2 {
                return V8DebuggerIdInternal::new();
            }

            if let (Ok(first), Ok(second)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                V8DebuggerIdInternal {
                    m_debugger_id: V8DebuggerId::new((first, second)),
                }
            } else {
                V8DebuggerIdInternal::new()
            }
        }

        pub fn generate(inspector: &mut super::V8InspectorImpl) -> Self {
            V8DebuggerIdInternal {
                m_debugger_id: V8DebuggerId::new((inspector.generate_unique_id(), inspector.generate_unique_id())),
            }
        }

        pub fn to_v8_debugger_id(&self) -> V8DebuggerId {
            V8DebuggerId {
                first: self.m_debugger_id.first,
                second: self.m_debugger_id.second,
            }
        }

        pub fn to_string(&self) -> String {
            self.m_debugger_id.to_string()
        }

        pub fn is_valid(&self) -> bool {
            self.m_debugger_id.is_valid()
        }

        pub fn pair(&self) -> (i64, i64) {
            self.m_debugger_id.pair()
        }
    }
}

pub mod V8InspectorImpl {
    pub struct V8InspectorImpl {
        unique_id_counter: i64,
    }

    impl V8InspectorImpl {
        pub fn new() -> Self {
            V8InspectorImpl {
                unique_id_counter: 0,
            }
        }
        pub fn generate_unique_id(&mut self) -> i64 {
            self.unique_id_counter += 1;
            self.unique_id_counter
        }
    }
}

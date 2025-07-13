// Converted from V8 C++ source files:
// Header: N/A
// Implementation: setup-isolate-deserialize.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! CHECK_WITH_MSG {
            ($cond:expr, $($arg:tt)*) => {
                if !$cond {
                    let message = format!($($arg)*);
                    panic!("Check failed: {}", message);
                }
            };
        }
    }
}

pub mod execution {
    pub struct Isolate {}
}

pub mod init {
    pub mod setup_isolate {
        use crate::execution::Isolate;

        pub struct SetupIsolateDelegate {}

        impl SetupIsolateDelegate {
            pub fn setup_heap(&self, isolate: &mut Isolate, create_heap_objects: bool) -> Result<bool, String> {
                // No actual work to be done; heap will be deserialized from the snapshot.
                base::logging::CHECK_WITH_MSG!(!create_heap_objects, "Heap setup supported only in mksnapshot");
                Ok(true)
            }

            pub fn setup_builtins(&self, isolate: &mut Isolate, compile_builtins: bool) -> Result<(), String> {
                // No actual work to be done; builtins will be deserialized from the snapshot.
                base::logging::CHECK_WITH_MSG!(!compile_builtins, "Builtin compilation supported only in mksnapshot");
                Ok(())
            }
        }
    }
}

pub mod v8 {
    pub type IdleTask = Box<dyn FnOnce() + Send>;
    pub type JobHandle = Box<dyn std::any::Any + Send + Sync>;
    pub type JobDelegate = Box<dyn FnMut() -> bool + Send>;
    pub type JobTask = Box<dyn FnOnce() + Send>;
    pub type PageAllocator = Box<dyn std::any::Any + Send + Sync>;
    pub type Task = Box<dyn FnOnce() + Send>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TaskPriority {
        kUserBlocking,
        kUserVisible,
        kBackground,
    }
    pub type TaskRunner = std::sync::Arc<dyn Fn(&'static dyn FnOnce() + Send) + Send + Sync>;
    pub type TracingController = Box<dyn std::any::Any + Send + Sync>;
}

pub mod cppgc {
    use std::sync::Arc;
    use std::any::Any;

    pub use crate::v8::{IdleTask, JobDelegate, JobHandle, JobTask, PageAllocator, Task, TaskPriority, TaskRunner, TracingController};

    pub mod source_location {
        #[derive(Debug, Clone)]
        pub struct SourceLocation {
            file: String,
            line: u32,
            column: u32,
        }

        impl SourceLocation {
            pub fn new(file: String, line: u32, column: u32) -> Self {
                SourceLocation { file, line, column }
            }

            pub fn current() -> Self {
                // This is a placeholder, as Rust doesn't have a direct equivalent to __FILE__, __LINE__, and __COLUMN__ in C++.
                SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                }
            }

            pub fn file(&self) -> &str {
                &self.file
            }

            pub fn line(&self) -> u32 {
                self.line
            }

            pub fn column(&self) -> u32 {
                self.column
            }
        }
    }
    use source_location::SourceLocation;

    pub trait Platform {
        fn get_page_allocator(&self) -> *mut dyn PageAllocator;
        fn monotonically_increasing_time(&self) -> f64;
        fn get_foreground_task_runner(&self) -> Option<TaskRunner> {
            self.get_foreground_task_runner_with_priority(TaskPriority::kUserBlocking)
        }
        fn get_foreground_task_runner_with_priority(&self, priority: TaskPriority) -> Option<TaskRunner> {
            let _ = priority; // Suppress unused variable warning for now
            None
        }
        fn post_job(&self, priority: TaskPriority, job_task: Box<dyn Any + Send>) -> Option<Box<dyn JobHandle>> {
            let _ = priority;
            let _ = job_task;
            None
        }
        fn get_tracing_controller(&self) -> *mut dyn TracingController;
    }

    // #[no_mangle]
    pub extern "C" fn initialize_process(page_allocator: *mut dyn PageAllocator, desired_heap_size: usize) {
        // Placeholder implementation.  Real implementation would initialize
        // the global state of the cppgc library.
        let _ = page_allocator;
        let _ = desired_heap_size;
    }

    // #[no_mangle]
    pub extern "C" fn shutdown_process() {
        // Placeholder implementation.  Real implementation would shutdown
        // the global state of the cppgc library.
    }

    pub mod internal {
        use super::*;
        pub extern "C" fn fatal(reason: Option<String>, location: Option<SourceLocation>) -> ! {
            match (reason, location) {
                (Some(r), Some(l)) => panic!("Fatal error: {} at {}:{}", r, l.file(), l.line()),
                (Some(r), None) => panic!("Fatal error: {}", r),
                (None, Some(l)) => panic!("Fatal error at {}:{}", l.file(), l.line()),
                (None, None) => panic!("Fatal error"),
            }
        }
    }
}
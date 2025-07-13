// Converted from V8 C++ source files:
// Header: platform.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
    pub use v8::IdleTask;
    pub use v8::JobDelegate;
    pub use v8::JobHandle;
    pub use v8::JobTask;
    pub use v8::PageAllocator;
    pub use v8::Task;
    pub use v8::TaskPriority;
    pub use v8::TaskRunner;
    pub use v8::TracingController;

    use crate::v8;
    use std::sync::{Arc, Mutex};
    use std::{string::String, time::Instant};

    pub struct SourceLocation {}

    impl SourceLocation {
        pub fn Current() -> Self {
            SourceLocation {}
        }
    }

    pub trait Platform {
        fn get_page_allocator(&self) -> *mut PageAllocator;
        fn monotonically_increasing_time(&self) -> f64;
        fn get_foreground_task_runner(&self) -> Option<Arc<dyn TaskRunner>> {
            self.get_foreground_task_runner_with_priority(TaskPriority::kUserBlocking)
        }
        fn get_foreground_task_runner_with_priority(
            &self,
            priority: TaskPriority,
        ) -> Option<Arc<dyn TaskRunner>> {
            None
        }
        fn post_job(
            &self,
            priority: TaskPriority,
            job_task: Box<dyn JobTask>,
        ) -> Option<Box<dyn JobHandle>> {
            None
        }
        fn get_tracing_controller(&self) -> *mut TracingController;
    }

    pub static mut PAGE_ALLOCATOR: Option<*mut PageAllocator> = None;
    pub static mut INITIALIZED: bool = false;

    pub fn initialize_process(page_allocator: *mut PageAllocator, desired_heap_size: usize) {
        unsafe {
            if INITIALIZED {
                return;
            }
            PAGE_ALLOCATOR = Some(page_allocator);
            INITIALIZED = true;
        }
    }

    pub fn shutdown_process() {
        unsafe {
            INITIALIZED = false;
            PAGE_ALLOCATOR = None;
        }
    }

    pub mod internal {
        use super::SourceLocation;
        use std::process;
        pub fn fatal(reason: &str, _location: &SourceLocation) {
            eprintln!("Fatal error: {}", reason);
            process::abort();
        }
    }
}

pub mod v8 {
    use std::ffi::c_void;
    use std::sync::Arc;
    pub struct IdleTask {}
    pub trait JobHandle {}
    pub trait JobDelegate {
        fn should_yield(&self) -> bool;
    }
    pub trait JobTask {
        fn run(&self, delegate: &mut dyn JobDelegate);
        fn get_max_concurrency(&self) -> usize;
    }
    pub struct PageAllocator {}
    pub trait Task {}
    #[derive(Debug, Clone, Copy)]
    pub enum TaskPriority {
        kUserBlocking,
        kUserVisible,
        kBackground,
    }
    pub trait TaskRunner {
        fn post_task(&self, task: Box<dyn Task>);
        fn post_delayed_task(&self, task: Box<dyn Task>, delay: std::time::Duration);
    }
    pub struct TracingController {}
}

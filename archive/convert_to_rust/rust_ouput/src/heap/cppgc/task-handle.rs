// Converted from V8 C++ source files:
// Header: task-handle.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct SingleThreadedHandle {
        is_cancelled: Option<Arc<Mutex<bool>>>,
    }

    impl SingleThreadedHandle {
        pub fn new() -> Self {
            SingleThreadedHandle {
                is_cancelled: None,
            }
        }

        pub fn with_tag() -> Self {
            SingleThreadedHandle {
                is_cancelled: Some(Arc::new(Mutex::new(false))),
            }
        }

        pub fn cancel(&mut self) {
            if let Some(is_cancelled) = &self.is_cancelled {
                let mut cancelled = is_cancelled.lock().unwrap();
                *cancelled = true;
            }
        }

        pub fn cancel_if_non_empty(&mut self) {
            if let Some(is_cancelled) = &self.is_cancelled {
                let mut cancelled = is_cancelled.lock().unwrap();
                *cancelled = true;
            }
        }

        pub fn is_canceled(&self) -> bool {
            match &self.is_cancelled {
                Some(is_cancelled) => *is_cancelled.lock().unwrap(),
                None => false,
            }
        }

        pub fn is_active(&self) -> bool {
            match &self.is_cancelled {
                Some(is_cancelled) => {
                    let cancelled = is_cancelled.lock().unwrap();
                    !*cancelled
                }
                None => false,
            }
        }
    }

    impl From<SingleThreadedHandle> for bool {
        fn from(handle: SingleThreadedHandle) -> Self {
            handle.is_active()
        }
    }
}

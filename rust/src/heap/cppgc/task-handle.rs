// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::sync::{Arc, Mutex};

    /// A handle that is used for cancelling individual tasks.
    pub struct SingleThreadedHandle {
        is_cancelled: Option<Arc<Mutex<bool>>>,
    }

    impl SingleThreadedHandle {
        /// Default construction results in empty handle.
        pub fn new() -> Self {
            SingleThreadedHandle { is_cancelled: None }
        }

        pub fn with_non_empty() -> Self {
            SingleThreadedHandle {
                is_cancelled: Some(Arc::new(Mutex::new(false))),
            }
        }

        pub fn cancel(&self) {
            if let Some(is_cancelled) = &self.is_cancelled {
                let mut cancelled = is_cancelled.lock().unwrap();
                *cancelled = true;
            } else {
              panic!("Cannot cancel an empty handle");
            }
        }

        pub fn cancel_if_non_empty(&self) {
            if let Some(is_cancelled) = &self.is_cancelled {
                let mut cancelled = is_cancelled.lock().unwrap();
                *cancelled = true;
            }
        }

        pub fn is_canceled(&self) -> bool {
            if let Some(is_cancelled) = &self.is_cancelled {
                let cancelled = is_cancelled.lock().unwrap();
                *cancelled
            } else {
              panic!("Cannot check cancellation status of an empty handle");
            }
        }
    }

    impl SingleThreadedHandle {
        /// A handle is active if it is non-empty and not cancelled.
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
// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicI32, Ordering};
use std::cell::RefCell;

mod base {
    pub mod lazy_instance {
        // Placeholder for LazyInstance functionality.  Needs actual implementation.
        pub struct LazyInstance<T> {
            data: std::cell::RefCell<Option<T>>,
            initializer: fn() -> T,
        }

        impl<T> LazyInstance<T> {
            pub const fn new(initializer: fn() -> T) -> Self {
                LazyInstance {
                    data: RefCell::new(None),
                    initializer,
                }
            }

            pub fn get(&self) -> std::cell::RefMut<'_, T> {
                let mut data = self.data.borrow_mut();
                if data.is_none() {
                    *data = Some((self.initializer)());
                }
                std::cell::RefMut::map(data, |x| x.as_mut().unwrap())
            }
        }

        unsafe impl<T> Sync for LazyInstance<T> {}
    }

    pub mod platform {
        // Placeholder for platform functionality.
        pub struct Platform {}
    }
}

pub mod internal {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ThreadId(i32);

    impl ThreadId {
        pub fn invalid() -> Self {
            ThreadId(0)
        }

        pub fn is_valid(&self) -> bool {
            self.0 != 0
        }
    }

    thread_local! {
        static THREAD_ID: RefCell<i32> = RefCell::new(0);
    }

    static NEXT_THREAD_ID: AtomicI32 = AtomicI32::new(1);

    impl ThreadId {
        /// Attempts to get the current thread ID. Returns `None` if the thread ID
        /// has not been initialized yet.
        pub fn try_get_current() -> Option<ThreadId> {
            THREAD_ID.with(|id| {
                let current_id = *id.borrow();
                if current_id == 0 {
                    None
                } else {
                    Some(ThreadId(current_id))
                }
            })
        }

        /// Gets the current thread ID. If the thread ID has not been initialized
        /// yet, it initializes it with a new unique ID.
        pub fn get_current_thread_id() -> i32 {
            THREAD_ID.with(|id| {
                let mut current_id = id.borrow_mut();
                if *current_id == 0 {
                    let next_id = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
                    *current_id = next_id;
                    assert!(*current_id >= 1); // Replaces CHECK_LE(1, thread_id)
                }
                *current_id
            })
        }

        pub fn new(id: i32) -> Self {
            ThreadId(id)
        }
    }
}
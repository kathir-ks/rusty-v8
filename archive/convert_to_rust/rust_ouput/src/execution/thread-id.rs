// Converted from V8 C++ source files:
// Header: thread-id.h
// Implementation: thread-id.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {
pub struct Platform {}
}
pub struct LazyInstance<T> {
    data: std::sync::OnceLock<T>,
}

impl<T> LazyInstance<T> {
    pub const fn new() -> Self {
        LazyInstance {
            data: std::sync::OnceLock::new(),
        }
    }

    pub fn get<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.data.get_or_init(f)
    }
}
}
pub mod internal {
use std::sync::atomic::{AtomicI32, Ordering};
use std::cell::RefCell;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ThreadId {
    id_: i32,
}

impl ThreadId {
    pub const fn new() -> Self {
        ThreadId { id_: Self::kInvalidId }
    }

    pub const fn is_valid(&self) -> bool {
        self.id_ != Self::kInvalidId
    }

    pub const fn to_integer(&self) -> i32 {
        self.id_
    }

    pub fn try_get_current() -> Self {
        THREAD_ID.with(|thread_id| {
            if *thread_id.borrow() == 0 {
                Self::invalid()
            } else {
                ThreadId { id_: *thread_id.borrow() }
            }
        })
    }

    pub fn current() -> Self {
        ThreadId { id_: Self::get_current_thread_id() }
    }

    pub const fn invalid() -> Self {
        ThreadId { id_: Self::kInvalidId }
    }

    pub const fn from_integer(id: i32) -> Self {
        ThreadId { id_: id }
    }

    const kInvalidId: i32 = -1;

    fn get_current_thread_id() -> i32 {
        THREAD_ID.with(|thread_id| {
            if *thread_id.borrow() == 0 {
                let mut next_id = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
                if next_id == i32::MAX {
                    NEXT_THREAD_ID.store(1, Ordering::Relaxed);
                    next_id = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
                }
                *thread_id.borrow_mut() = next_id;
            }
            *thread_id.borrow()
        })
    }
}

thread_local! {
    static THREAD_ID: RefCell<i32> = RefCell::new(0);
}

static NEXT_THREAD_ID: AtomicI32 = AtomicI32::new(1);

}
}

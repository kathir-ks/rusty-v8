// Converted from V8 C++ source files:
// Header: atomic-entry-flag.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::atomic::{AtomicI32, Ordering};

pub mod internal {
    use std::sync::atomic::{AtomicI32, Ordering};

    // A flag which provides a fast check whether a scope may be entered on the
    // current thread, without needing to access thread-local storage or mutex.  Can
    // have false positives (i.e., spuriously report that it might be entered), so
    // it is expected that this will be used in tandem with a precise check that the
    // scope is in fact entered on that thread.
    //
    // Example:
    //   g_frobnicating_flag.MightBeEntered() &&
    //   ThreadLocalFrobnicator().IsFrobnicating()
    //
    // Relaxed atomic operations are sufficient, since:
    // - all accesses remain atomic
    // - each thread must observe its own operations in order
    // - no thread ever exits the flag more times than it enters (if used correctly)
    // And so if a thread observes zero, it must be because it has observed an equal
    // number of exits as entries.
    pub struct AtomicEntryFlag {
        entries_: AtomicI32,
    }

    impl AtomicEntryFlag {
        pub fn new() -> Self {
            AtomicEntryFlag {
                entries_: AtomicI32::new(0),
            }
        }

        pub fn enter(&self) {
            self.entries_.fetch_add(1, Ordering::Relaxed);
        }

        pub fn exit(&self) {
            self.entries_.fetch_sub(1, Ordering::Relaxed);
        }

        // Returns false only if the current thread is not between a call to Enter
        // and a call to Exit. Returns true if this thread or another thread may
        // currently be in the scope guarded by this flag.
        pub fn might_be_entered(&self) -> bool {
            self.entries_.load(Ordering::Relaxed) != 0
        }
    }
}

// Converted from V8 C++ source files:
// Header: N/A
// Implementation: handler-shared.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod trap_handler {
    use std::sync::atomic::{AtomicUsize, AtomicPtr, AtomicBool, AtomicU64, Ordering};
    use std::sync::{Mutex, MutexGuard, Arc};
    use std::mem::size_of;
    use std::ptr::null_mut;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::AtomicPtr;
    use std::sync::atomic::Ordering;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::AtomicUptr;
    use std::sync::atomic::AtomicIsize;
    use std::sync::atomic::AtomicU32;
    use std::sync::atomic::AtomicU8;

    thread_local! {
        static G_THREAD_IN_WASM_CODE: i32 = 0;
    }

    #[cfg(target_pointer_width = "64")]
    const POINTER_SIZE: usize = 8;
    #[cfg(target_pointer_width = "32")]
    const POINTER_SIZE: usize = 4;

    pub static mut G_NUM_CODE_OBJECTS: usize = 0;
    pub static mut G_CODE_OBJECTS: *mut CodeProtectionInfoListEntry = null_mut();
    pub static mut G_SANDBOX_RECORDS_HEAD: *mut SandboxRecord = null_mut();
    pub static G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
    pub static G_LANDING_PAD: AtomicUptr = AtomicUptr::new(0);

    pub struct CodeProtectionInfoListEntry {}
    pub struct SandboxRecord {}

    pub struct MetadataLock {
        //spinlock_: std::sync::atomic::AtomicBool,
    }

    impl MetadataLock {
        pub fn new() -> Self {
            MetadataLock {
                //spinlock_: std::sync::atomic::AtomicBool::new(false),
            }
        }

        pub fn lock(&self) -> MetadataLockGuard {
            G_THREAD_IN_WASM_CODE.with(|in_wasm| {
                if *in_wasm != 0 {
                    panic!("Abort: g_thread_in_wasm_code != 0");
                }
            });

            while METADATA_SPINLOCK.load(Ordering::Acquire) {
                // Spin
            }
            METADATA_SPINLOCK.store(true, Ordering::Release);
            MetadataLockGuard {}
        }
    }

    pub struct MetadataLockGuard {}

    impl Drop for MetadataLockGuard {
        fn drop(&mut self) {
            G_THREAD_IN_WASM_CODE.with(|in_wasm| {
                if *in_wasm != 0 {
                    panic!("Abort: g_thread_in_wasm_code != 0");
                }
            });

            METADATA_SPINLOCK.store(false, Ordering::Release);
        }
    }

    pub struct SandboxRecordsLock {}

    impl SandboxRecordsLock {
        pub fn new() -> Self {
            SandboxRecordsLock {}
        }

        pub fn lock(&self) -> SandboxRecordsLockGuard {
             while SANDBOX_SPINLOCK.load(Ordering::Acquire) {
                // Spin
            }
            SANDBOX_SPINLOCK.store(true, Ordering::Release);
            SandboxRecordsLockGuard {}
        }
    }

    pub struct SandboxRecordsLockGuard {}

    impl Drop for SandboxRecordsLockGuard {
        fn drop(&mut self) {
             SANDBOX_SPINLOCK.store(false, Ordering::Release);
        }
    }

    static METADATA_SPINLOCK: AtomicBool = AtomicBool::new(false);
    static SANDBOX_SPINLOCK: AtomicBool = AtomicBool::new(false);

}

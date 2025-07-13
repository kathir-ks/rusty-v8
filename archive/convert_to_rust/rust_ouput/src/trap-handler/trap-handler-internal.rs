// Converted from V8 C++ source files:
// Header: trap-handler-internal.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod trap_handler_internal {
    use std::sync::atomic::{AtomicFlag, Ordering};
    use std::sync::Mutex;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CodeProtectionInfo {
        pub base: usize,
        pub size: usize,
        pub num_protected_instructions: usize,
        pub instructions: [ProtectedInstructionData; 1], // Flexible array member, handle with care
    }

    #[derive(Debug)]
    pub struct ProtectedInstructionData {}

    pub struct MetadataLock {
        // No data needed, the lock is in the static variable
    }

    impl MetadataLock {
        static SPINLOCK: AtomicFlag = AtomicFlag::new(AtomicFlag::NOT_LOCKED);

        pub fn new() -> Self {
            while MetadataLock::SPINLOCK.test_and_set(Ordering::Acquire) {}
            MetadataLock {}
        }

        //RAII implementation of Drop trait unlocks mutex
        //when MetadataLock struct goes out of scope
    }

    impl Drop for MetadataLock {
        fn drop(&mut self) {
            MetadataLock::SPINLOCK.clear(Ordering::Release);
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct CodeProtectionInfoListEntry {
        pub code_info: *mut CodeProtectionInfo,
        pub next_free: usize,
    }

    pub static mut G_NUM_CODE_OBJECTS: usize = 0;
    pub static mut G_CODE_OBJECTS: *mut CodeProtectionInfoListEntry = std::ptr::null_mut();

    #[derive(Debug)]
    #[repr(C)]
    pub struct SandboxRecord {
        pub base: usize,
        pub size: usize,
        pub next: *mut SandboxRecord,
    }

    pub struct SandboxRecordsLock {}

    impl SandboxRecordsLock {
        static SPINLOCK: AtomicFlag = AtomicFlag::new(AtomicFlag::NOT_LOCKED);

        pub fn new() -> Self {
            while SandboxRecordsLock::SPINLOCK.test_and_set(Ordering::Acquire) {}
            SandboxRecordsLock {}
        }
    }

    impl Drop for SandboxRecordsLock {
        fn drop(&mut self) {
            SandboxRecordsLock::SPINLOCK.clear(Ordering::Release);
        }
    }

    pub static mut G_SANDBOX_RECORDS_HEAD: *mut SandboxRecord = std::ptr::null_mut();
    use std::sync::atomic::AtomicUsize;
    pub static G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
    use std::sync::atomic::AtomicPtr;
    pub static G_LANDING_PAD: AtomicPtr<usize> = AtomicPtr::new(std::ptr::null_mut());

    pub fn is_fault_address_covered(fault_addr: usize) -> bool {
        unsafe {
            if G_CODE_OBJECTS.is_null() {
                return false;
            }
            for i in 0..G_NUM_CODE_OBJECTS {
                let entry = G_CODE_OBJECTS.add(i);
                if (*entry).code_info.is_null() {
                    continue;
                }
                let code_info = *(*entry).code_info;
                let base = code_info.base;
                let size = code_info.size;
                if fault_addr >= base && fault_addr < base + size {
                    return true;
                }
            }
            false
        }
    }

    pub fn is_accessed_memory_covered(accessed_addr: usize) -> bool {
        unsafe {
            let mut current = G_SANDBOX_RECORDS_HEAD;
            while !current.is_null() {
                let record = *current;
                if accessed_addr >= record.base && accessed_addr < record.base + record.size {
                    return true;
                }
                current = record.next;
            }
            false
        }
    }
}

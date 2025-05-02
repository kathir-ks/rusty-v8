// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod trap_handler_internal {
    use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};
    use std::sync::Mutex;

    use super::trap_handler::ProtectedInstructionData;

    /// This describes a chunk of code that the trap handler will be able to handle
    /// faults in. {base} points to the beginning of the chunk, and {size} is the
    /// number of bytes in the code chunk. The remainder of the struct is a list of
    /// protected memory access instructions and an offset to a landing pad to handle
    /// faults on that instruction.
    #[repr(C)]
    pub struct CodeProtectionInfo {
        pub base: usize,
        pub size: usize,
        pub num_protected_instructions: usize,
        pub instructions: [ProtectedInstructionData; 1], // Flexible array member, needs special handling
    }

    /// A spinlock-based mutex for protecting metadata.
    pub struct MetadataLock {
        _private: (),
    }

    impl MetadataLock {
        // Initialize the spinlock
        pub fn new() -> Self {
            static SPINLOCK: AtomicBool = AtomicBool::new(false);
            MetadataLock { _private: () }
        }

        // Acquire the lock
        pub fn lock(&self) {
            static SPINLOCK: AtomicBool = AtomicBool::new(false);
            while SPINLOCK.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
                std::hint::spin_loop();
            }
        }

        // Release the lock
        pub fn unlock(&self) {
            static SPINLOCK: AtomicBool = AtomicBool::new(true);
            SPINLOCK.store(false, Ordering::Release);
        }
    }

    /// To enable constant time registration of handler data, we keep a free list of
    /// entries in the gCodeObjects table. Each entry contains a {next_free} field,
    /// which can be used to figure out where the next entry should be inserted.
    /// In order to avoid having to initialize all the links to start with, we use
    /// 0 to indicate that this is a fresh, never-used list entry and that therefore
    /// the next entry is known to be free. If {next_entry} is greater than zero,
    /// then {next_entry - 1} is the index that we should insert into next.
    #[repr(C)]
    pub struct CodeProtectionInfoListEntry {
        pub code_info: *mut CodeProtectionInfo,
        pub next_free: usize,
    }

    lazy_static::lazy_static! {
        pub static ref G_NUM_CODE_OBJECTS: AtomicUsize = AtomicUsize::new(0);
        pub static ref G_CODE_OBJECTS: Mutex<Vec<CodeProtectionInfoListEntry>> = Mutex::new(Vec::new());
    }

    /// This list describes sandboxes as bases and sizes.
    #[repr(C)]
    pub struct SandboxRecord {
        pub base: usize,
        pub size: usize,
        pub next: *mut SandboxRecord,
    }

    /// A spinlock-based mutex for protecting sandbox records.
    pub struct SandboxRecordsLock {
        _private: (),
    }

    impl SandboxRecordsLock {
         // Initialize the spinlock
         pub fn new() -> Self {
            static SPINLOCK: AtomicBool = AtomicBool::new(false);
            SandboxRecordsLock { _private: () }
        }

        // Acquire the lock
        pub fn lock(&self) {
             static SPINLOCK: AtomicBool = AtomicBool::new(false);
            while SPINLOCK.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
                std::hint::spin_loop();
            }
        }

        // Release the lock
        pub fn unlock(&self) {
            static SPINLOCK: AtomicBool = AtomicBool::new(true);
            SPINLOCK.store(false, Ordering::Release);
        }
    }

    lazy_static::lazy_static! {
        pub static ref G_SANDBOX_RECORDS_HEAD: AtomicPtr<SandboxRecord> = AtomicPtr::new(std::ptr::null_mut());
        pub static ref G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
        pub static ref G_LANDING_PAD: AtomicUsize = AtomicUsize::new(0);
    }

    /// Searches the fault location table for an entry matching fault_addr. If found,
    /// returns true, otherwise, returns false.
    pub fn is_fault_address_covered(fault_addr: usize) -> bool {
         let g_code_objects = G_CODE_OBJECTS.lock().unwrap();
        for entry in g_code_objects.iter() {
            if !entry.code_info.is_null() {
                let code_info = unsafe { &*entry.code_info };
                if fault_addr >= code_info.base && fault_addr < (code_info.base + code_info.size) {
                    return true;
                }
            }
        }
        false
    }

    /// Checks whether the accessed memory is covered by the trap handler. In
    /// particular, when the V8 sandbox is enabled, only faulting accesses to memory
    /// inside the sandbox are handled by the trap handler since all Wasm memory
    /// objects are inside the sandbox.
    pub fn is_accessed_memory_covered(accessed_addr: usize) -> bool {
        let sandbox_records_head = G_SANDBOX_RECORDS_HEAD.load(Ordering::Relaxed);
        let mut current = sandbox_records_head;

        while !current.is_null() {
            let record = unsafe { &*current };
            if accessed_addr >= record.base && accessed_addr < (record.base + record.size) {
                return true;
            }
            current = record.next;
        }

        false
    }
}

pub mod trap_handler {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ProtectedInstructionData {
        pub instruction_offset: u32,
        pub landing_pad_offset: u32,
    }
}
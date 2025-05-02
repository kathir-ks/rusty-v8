// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// PLEASE READ BEFORE CHANGING THIS FILE!
//
// This file implements the out of bounds trap handler for
// WebAssembly. Trap handlers are notoriously difficult to get
// right, and getting it wrong can lead to security
// vulnerabilities. In order to minimize this risk, here are some
// rules to follow.
//
// 1. Do not introduce any new external dependencies. This file needs
//    to be self contained so it is easy to audit everything that a
//    trap handler might do.
//
// 2. Any changes must be reviewed by someone from the crash reporting
//    or security team. See OWNERS for suggested reviewers.
//
// For more information, see https://goo.gl/yMeyUY.
//
// This file contains most of the code that actually runs in a trap handler
// context. Some additional code is used both inside and outside the trap
// handler. This code can be found in handler-shared.cc.

// Note: The original C++ code relies heavily on global mutable state.
// Translating this directly into Rust requires unsafe code and careful
// synchronization.  This translation attempts to minimize the `unsafe` blocks
// and uses `std::sync::Mutex` to guard access to shared mutable data.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// Define a macro to conditionally compile the trap handler code.
macro_rules! v8_trap_handler_supported {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

// Recreate the V8_TRAP_HANDLER_SUPPORTED macro
const V8_TRAP_HANDLER_SUPPORTED: bool = true;

// Recreate the V8_ENABLE_DRUMBRAKE macro
const V8_ENABLE_DRUMBRAKE: bool = false;

mod trap_handler_internal {
    // This module is a placeholder for the contents of "src/trap-handler/trap-handler-internal.h"
    // which are not provided in the original code snippet.  It is needed to avoid compilation
    // errors due to missing types.  A complete translation would require understanding what
    // CodeProtectionInfo, ProtectedInstructionData, MetadataLock, etc. represent.
    // In the meantime, we stub out the needed types and functions.

    use std::sync::{Mutex, MutexGuard};

    pub struct CodeProtectionInfo {
        pub base: usize,
        pub size: usize,
        pub num_protected_instructions: u32,
        pub instructions: Vec<ProtectedInstructionData>,
    }

    pub struct ProtectedInstructionData {
        pub instr_offset: u32,
    }

    pub struct CodeObject {
        pub code_info: Option<Box<CodeProtectionInfo>>,
    }

    // This is a stub and it will have to be removed later on if it is important.
    // For now, the important thing is to not break the compilation.
    pub struct MetadataLockGuard<'a> {
        _lock: &'a Mutex<()>,
    }
    impl<'a> MetadataLockGuard<'a> {
        pub fn new(lock: &'a Mutex<()>) -> Self {
            MetadataLockGuard { _lock: lock }
        }
    }

    // This stub may have to be removed later.
    pub struct SandboxRecordsLockGuard<'a> {
        _lock: &'a Mutex<()>,
    }
    impl<'a> SandboxRecordsLockGuard<'a> {
        pub fn new(lock: &'a Mutex<()>) -> Self {
            SandboxRecordsLockGuard { _lock: lock }
        }
    }

    pub struct SandboxRecord {
        pub base: usize,
        pub size: usize,
        pub next: Option<Box<SandboxRecord>>,
    }

    lazy_static::lazy_static! {
        pub static ref METADATA_LOCK: Mutex<()> = Mutex::new(());
        pub static ref SANDBOX_RECORDS_LOCK: Mutex<()> = Mutex::new(());
    }

    #[macro_export]
    macro_rules! th_dcheck {
        ($condition:expr) => {
            if !$condition {
                panic!("TH_DCHECK failed: {}", stringify!($condition));
            }
        };
    }

    pub struct MetadataLock<'a> {
        _guard: MetadataLockGuard<'a>,
    }

    impl<'a> MetadataLock<'a> {
        pub fn new() -> Self {
            MetadataLock {
                _guard: MetadataLockGuard::new(&METADATA_LOCK),
            }
        }
    }

    pub struct SandboxRecordsLock<'a> {
        _guard: SandboxRecordsLockGuard<'a>,
    }

    impl<'a> SandboxRecordsLock<'a> {
        pub fn new() -> Self {
            SandboxRecordsLock {
                _guard: SandboxRecordsLockGuard::new(&SANDBOX_RECORDS_LOCK),
            }
        }
    }
}

mod trap_handler {
    // This module is a placeholder for the contents of "src/trap-handler/trap-handler.h"
    // which are not provided in the original code snippet.  It is needed to avoid compilation
    // errors due to missing types.  A complete translation would require understanding what
    // CodeProtectionInfo, ProtectedInstructionData, MetadataLock, etc. represent.
    // In the meantime, we stub out the needed types and functions.

    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    lazy_static::lazy_static! {
        pub static ref G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
    }

    pub struct CodeObjectInfo {
        pub code_info: Option<Box<crate::trap_handler_internal::CodeProtectionInfo>>,
    }

    lazy_static::lazy_static! {
        pub static ref G_CODE_OBJECTS: Vec<CodeObjectInfo> = Vec::new();
        pub static ref G_NUM_CODE_OBJECTS: usize = 0;

        // This is a stub and needs to be removed if it is not important.
        pub static ref G_SANDBOX_RECORDS_HEAD: Option<Box<crate::trap_handler_internal::SandboxRecord>> = None;
    }

    pub fn recovered_trap_count_inc() {
        G_RECOVERED_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}

pub mod trap_handler_inside {
    use crate::trap_handler::*;
    use crate::trap_handler_internal::*;
    use std::sync::atomic::Ordering;

    #[v8_trap_handler_supported]
    /// Checks if the fault address is within a protected code region.
    pub fn is_fault_address_covered(fault_addr: usize) -> bool {
        // TODO(eholk): broad code range check

        let lock_holder = MetadataLock::new();

        for i in 0..*G_NUM_CODE_OBJECTS {
            if let Some(ref code_object) = G_CODE_OBJECTS.get(i) {
                if let Some(ref data) = code_object.code_info {
                    let base = data.base;

                    if fault_addr >= base && fault_addr < base + data.size {
                        // Hurray, we found the code object. Check for protected addresses.
                        let offset = (fault_addr - base) as u32;
                        // The offset must fit in 32 bit, see comment on
                        // ProtectedInstructionData::instr_offset.
                        th_dcheck!(base + offset as usize == fault_addr);

                        #[cfg(feature = "v8_enable_drumbrake")]
                        {
                            // Ignore the protected instruction offsets if we are running in the Wasm
                            // interpreter.
                            if data.num_protected_instructions == 0 {
                                G_RECOVERED_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
                                return true;
                            }
                        }

                        for j in 0..data.num_protected_instructions {
                            if data.instructions[j as usize].instr_offset == offset {
                                // Hurray again, we found the actual instruction.
                                G_RECOVERED_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    #[v8_trap_handler_supported]
    /// Checks if the accessed memory address is within a valid sandbox region.
    pub fn is_accessed_memory_covered(addr: usize) -> bool {
        let lock_holder = SandboxRecordsLock::new();

        // Check if the access is inside a V8 sandbox (if it is enabled) as all Wasm
        // Memory objects must be located inside some sandbox.
        if G_SANDBOX_RECORDS_HEAD.is_none() {
            return true;
        }

        let mut current = G_SANDBOX_RECORDS_HEAD.as_ref();
        while let Some(record) = current {
            if addr >= record.base && addr < (record.base + record.size) {
                return true;
            }
            current = record.next.as_ref();
        }

        false
    }
}
// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// PLEASE READ BEFORE CHANGING THIS FILE!
//
// This file implements the support code for the out of bounds trap handler.
// Nothing in here actually runs in the trap handler, but the code here
// manipulates data structures used by the trap handler so we still need to be
// careful. In order to minimize this risk, here are some rules to follow.
//
// 1. Avoid introducing new external dependencies. The files in src/trap-handler
//    should be as self-contained as possible to make it easy to audit the code.
//
// 2. Any changes must be reviewed by someone from the crash reporting
//    or security team. See OWNERS for suggested reviewers.
//
// For more information, see https://goo.gl/yMeyUY.
//
// For the code that runs in the trap handler itself, see handler-inside.rs.

use std::mem;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

mod trap_handler_internal;
pub mod trap_handler;

// Assume that `MetadataLock` and `SandboxRecordsLock` are OS-specific mutexes that we cannot
// accurately reproduce without target platform details.
#[derive(Default)]
struct MetadataLock {
    mutex: Mutex<()>,
}

impl MetadataLock {
    fn new() -> Self {
        MetadataLock {
            mutex: Mutex::new(()),
        }
    }
}

#[derive(Default)]
struct SandboxRecordsLock {
    mutex: Mutex<()>,
}

impl SandboxRecordsLock {
    fn new() -> Self {
        SandboxRecordsLock {
            mutex: Mutex::new(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ProtectedInstructionData {
    pub instr_offset: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct CodeProtectionInfo {
    pub base: usize,
    pub size: usize,
    pub num_protected_instructions: usize,
    pub instructions: [ProtectedInstructionData; 0], // Flexible array member, needs custom allocation.
}

#[derive(Debug)]
struct CodeProtectionInfoListEntry {
    code_info: *mut CodeProtectionInfo,
    next_free: usize,
}

static mut G_NEXT_CODE_OBJECT: usize = 0;

#[cfg(feature = "slow_dchecks")]
const K_ENABLE_SLOW_CHECKS: bool = true;

#[cfg(not(feature = "slow_dchecks"))]
const K_ENABLE_SLOW_CHECKS: bool = false;

const K_INITIAL_CODE_OBJECT_SIZE: usize = 1024;
const K_CODE_OBJECT_GROWTH_FACTOR: usize = 2;
const K_INVALID_INDEX: i32 = -1;

fn handler_data_size(num_protected_instructions: usize) -> usize {
    mem::size_of::<CodeProtectionInfo>() +
        num_protected_instructions * mem::size_of::<ProtectedInstructionData>()
}

#[cfg(debug_assertions)]
fn is_disjoint(a: *const CodeProtectionInfo, b: *const CodeProtectionInfo) -> bool {
    if a.is_null() || b.is_null() {
        return true;
    }
    unsafe {
        (*a).base >= (*b).base + (*b).size || (*b).base >= (*a).base + (*a).size
    }
}

#[cfg(debug_assertions)]
fn verify_code_range_is_disjoint(code_info: *const CodeProtectionInfo) {
    unsafe {
        for i in 0..G_NUM_CODE_OBJECTS {
            trap_handler_internal::th_dcheck(is_disjoint(code_info, G_CODE_OBJECTS[i].code_info));
        }
    }
}

#[cfg(debug_assertions)]
fn validate_code_objects() {
    unsafe {
        for i in 0..G_NUM_CODE_OBJECTS {
            let data = G_CODE_OBJECTS[i].code_info;

            if data.is_null() {
                continue;
            }

            for j in 0..(*data).num_protected_instructions {
                trap_handler_internal::th_dcheck((*data).instructions[j].instr_offset >= 0);
                trap_handler_internal::th_dcheck((*data).instructions[j].instr_offset < (*data).size);
            }
        }

        let mut free_count: usize = 0;
        let mut i: usize = G_NEXT_CODE_OBJECT;
        while i != G_NUM_CODE_OBJECTS {
            trap_handler_internal::th_dcheck(i < G_NUM_CODE_OBJECTS);
            free_count += 1;
            trap_handler_internal::th_dcheck(free_count <= G_NUM_CODE_OBJECTS);

            i = G_CODE_OBJECTS[i].next_free;
        }

        let mut free_count2: usize = 0;
        for i in 0..G_NUM_CODE_OBJECTS {
            if G_CODE_OBJECTS[i].code_info.is_null() {
                free_count2 += 1;
            }
        }

        trap_handler_internal::th_dcheck(free_count == free_count2);
    }
}

pub fn create_handler_data(
    base: usize,
    size: usize,
    num_protected_instructions: usize,
    protected_instructions: &[ProtectedInstructionData],
) -> *mut CodeProtectionInfo {
    let alloc_size = handler_data_size(num_protected_instructions);

    unsafe {
        let data = libc::malloc(alloc_size) as *mut CodeProtectionInfo;

        if data.is_null() {
            return std::ptr::null_mut();
        }

        (*data).base = base;
        (*data).size = size;
        (*data).num_protected_instructions = num_protected_instructions;

        if num_protected_instructions > 0 {
            let dest = (*data).instructions.as_mut_ptr() as *mut u8;
            let src = protected_instructions.as_ptr() as *const u8;
            libc::memcpy(
                dest,
                src,
                num_protected_instructions * mem::size_of::<ProtectedInstructionData>(),
            );
        }

        data
    }
}

pub fn register_handler_data(
    base: usize,
    size: usize,
    num_protected_instructions: usize,
    protected_instructions: &[ProtectedInstructionData],
) -> i32 {
    let data = create_handler_data(
        base,
        size,
        num_protected_instructions,
        protected_instructions,
    );

    if data.is_null() {
        std::process::abort();
    }

    let lock = MetadataLock::new();
    let _guard = lock.mutex.lock().unwrap();

    if K_ENABLE_SLOW_CHECKS {
        #[cfg(debug_assertions)]
        verify_code_range_is_disjoint(data);
    }

    unsafe {
        let mut i = G_NEXT_CODE_OBJECT;

        let int_max: usize = i32::MAX as usize;

        if i == G_NUM_CODE_OBJECTS {
            let mut new_size: usize = if G_NUM_CODE_OBJECTS > 0 {
                G_NUM_CODE_OBJECTS * K_CODE_OBJECT_GROWTH_FACTOR
            } else {
                K_INITIAL_CODE_OBJECT_SIZE
            };

            if new_size > int_max {
                new_size = int_max;
            }
            if new_size == G_NUM_CODE_OBJECTS {
                libc::free(data as *mut libc::c_void);
                return K_INVALID_INDEX;
            }

            G_CODE_OBJECTS = libc::realloc(
                G_CODE_OBJECTS as *mut libc::c_void,
                mem::size_of::<CodeProtectionInfoListEntry>() * new_size,
            ) as *mut CodeProtectionInfoListEntry;

            if G_CODE_OBJECTS.is_null() {
                std::process::abort();
            }

            let start = G_CODE_OBJECTS.add(G_NUM_CODE_OBJECTS) as *mut u8;
            let size = mem::size_of::<CodeProtectionInfoListEntry>() * (new_size - G_NUM_CODE_OBJECTS);
            libc::memset(start as *mut libc::c_void, 0, size);

            for j in G_NUM_CODE_OBJECTS..new_size {
                G_CODE_OBJECTS.add(j).write(CodeProtectionInfoListEntry {
                    code_info: std::ptr::null_mut(),
                    next_free: j + 1,
                });
            }
            G_NUM_CODE_OBJECTS = new_size;
        }

        trap_handler_internal::th_dcheck(G_CODE_OBJECTS[i].code_info.is_null());

        G_NEXT_CODE_OBJECT = G_CODE_OBJECTS[i].next_free;

        if i <= int_max {
            G_CODE_OBJECTS[i].code_info = data;

            if K_ENABLE_SLOW_CHECKS {
                #[cfg(debug_assertions)]
                validate_code_objects();
            }

            return i as i32;
        } else {
            libc::free(data as *mut libc::c_void);
            return K_INVALID_INDEX;
        }
    }
}

pub fn release_handler_data(index: i32) {
    if index == K_INVALID_INDEX {
        return;
    }
    trap_handler_internal::th_dcheck(index >= 0);

    let mut data: *mut CodeProtectionInfo = std::ptr::null_mut();
    unsafe {
        let lock = MetadataLock::new();
        let _guard = lock.mutex.lock().unwrap();

        data = G_CODE_OBJECTS[index as usize].code_info;
        G_CODE_OBJECTS[index as usize].code_info = std::ptr::null_mut();

        G_CODE_OBJECTS[index as usize].next_free = G_NEXT_CODE_OBJECT;
        G_NEXT_CODE_OBJECT = index as usize;

        if K_ENABLE_SLOW_CHECKS {
            #[cfg(debug_assertions)]
            validate_code_objects();
        }
    }

    trap_handler_internal::th_dcheck(!data.is_null());

    unsafe { libc::free(data as *mut libc::c_void) };
}

#[derive(Debug)]
struct SandboxRecord {
    base: usize,
    size: usize,
    next: *mut SandboxRecord,
}

static mut G_SANDBOX_RECORDS_HEAD: *mut SandboxRecord = std::ptr::null_mut();

pub fn register_v8_sandbox(base: usize, size: usize) -> bool {
    let lock = SandboxRecordsLock::new();
    let _guard = lock.mutex.lock().unwrap();

    unsafe {
        #[cfg(debug_assertions)]
        {
            let mut current = G_SANDBOX_RECORDS_HEAD;
            while !current.is_null() {
                trap_handler_internal::th_dcheck((*current).base != base);
                current = (*current).next;
            }
        }

        let new_record = libc::malloc(mem::size_of::<SandboxRecord>()) as *mut SandboxRecord;
        if new_record.is_null() {
            return false;
        }

        (*new_record).base = base;
        (*new_record).size = size;
        (*new_record).next = G_SANDBOX_RECORDS_HEAD;
        G_SANDBOX_RECORDS_HEAD = new_record;
        return true;
    }
}

pub fn unregister_v8_sandbox(base: usize, size: usize) {
    let lock = SandboxRecordsLock::new();
    let _guard = lock.mutex.lock().unwrap();

    unsafe {
        let mut current = G_SANDBOX_RECORDS_HEAD;
        let mut previous: *mut SandboxRecord = std::ptr::null_mut();
        while !current.is_null() {
            if (*current).base == base {
                break;
            }
            previous = current;
            current = (*current).next;
        }

        trap_handler_internal::th_check(!current.is_null());
        trap_handler_internal::th_check((*current).size == size);

        if !previous.is_null() {
            (*previous).next = (*current).next;
        } else {
            G_SANDBOX_RECORDS_HEAD = (*current).next;
        }
        libc::free(current as *mut libc::c_void);
    }
}

static mut G_THREAD_IN_WASM_CODE: i32 = 0;

pub fn get_thread_in_wasm_thread_local_address() -> *mut i32 {
    unsafe { &mut G_THREAD_IN_WASM_CODE }
}

static G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn get_recovered_trap_count() -> usize {
    G_RECOVERED_TRAP_COUNT.load(Ordering::Relaxed)
}

// This version is provided for systems that do not support trap handlers.
// Otherwise, the correct one should be implemented in the appropriate
// platform-specific handler-outside.rs.
#[cfg(not(feature = "v8_trap_handler_supported"))]
pub fn register_default_trap_handler() -> bool {
    false
}

#[cfg(not(feature = "v8_trap_handler_supported"))]
pub fn remove_trap_handler() {}

static mut G_IS_TRAP_HANDLER_ENABLED: bool = false;
static G_CAN_ENABLE_TRAP_HANDLER: AtomicBool = AtomicBool::new(true);
static G_LANDING_PAD: AtomicUsize = AtomicUsize::new(0);

pub fn enable_trap_handler(use_v8_handler: bool) -> bool {
    let can_enable = G_CAN_ENABLE_TRAP_HANDLER.exchange(false, Ordering::Relaxed);
    trap_handler_internal::th_check(can_enable);

    #[cfg(not(feature = "v8_trap_handler_supported"))]
    {
        return false;
    }

    unsafe {
        if use_v8_handler {
            G_IS_TRAP_HANDLER_ENABLED = trap_handler_internal::register_default_trap_handler();
            return G_IS_TRAP_HANDLER_ENABLED;
        }
        G_IS_TRAP_HANDLER_ENABLED = true;
        return true;
    }
}

pub fn set_landing_pad(landing_pad: usize) {
    G_LANDING_PAD.store(landing_pad, Ordering::Relaxed);
}

#[cfg(any(feature = "building_v8_shared_private", feature = "using_v8_shared_private"))]
pub fn assert_thread_not_in_wasm() {
    unsafe {
        trap_handler_internal::th_dcheck(!G_IS_TRAP_HANDLER_ENABLED || G_THREAD_IN_WASM_CODE == 0);
    }
}

// Global mutable state. Access to these variables should be protected by a mutex.
// The original C++ uses raw pointers and malloc/free.  This Rust code uses
// raw pointers as well, but does not provide a safe abstraction.  The
// reason for this is that memory allocations/deallocations are called from
// outside the Rust environment and must not trigger Rust's borrow checker.

// NOTE: All static mut variables must be initialized here because Rust only
// allows assignment to static mut variables inside a const context.
static mut G_CODE_OBJECTS: *mut CodeProtectionInfoListEntry = std::ptr::null_mut();
static mut G_NUM_CODE_OBJECTS: usize = 0;
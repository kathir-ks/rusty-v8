// Converted from V8 C++ source files:
// Header: N/A
// Implementation: handler-outside.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{mem, ptr};

#[cfg(debug_assertions)]
use std::sync::Mutex;

const kInvalidIndex: i32 = -1;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ProtectedInstructionData {
    pub instr_offset: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct CodeProtectionInfo {
    pub base: usize,
    pub size: usize,
    pub num_protected_instructions: usize,
    pub instructions: [ProtectedInstructionData; 0], // Flexible array member
}

#[derive(Debug)]
struct CodeProtectionInfoListEntry {
    code_info: *mut CodeProtectionInfo,
    next_free: usize,
}

static mut g_code_objects: *mut CodeProtectionInfoListEntry = ptr::null_mut();
static mut g_num_code_objects: usize = 0;
static mut g_next_code_object: usize = 0;

// Ensure that g_is_trap_handler_enabled is initialized before any other thread can access it.
static g_is_trap_handler_enabled: AtomicBool = AtomicBool::new(false);
static g_can_enable_trap_handler: AtomicBool = AtomicBool::new(true);
static g_recovered_trap_count: AtomicUsize = AtomicUsize::new(0);
static g_landing_pad: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
#[repr(C)]
struct SandboxRecord {
    base: usize,
    size: usize,
    next: *mut SandboxRecord,
}

#[derive(Debug)]
struct MetadataLock {}
impl MetadataLock {
    pub fn new() -> Self {
        MetadataLock {}
    }
}

#[derive(Debug)]
struct SandboxRecordsLock {}
impl SandboxRecordsLock {
    pub fn new() -> Self {
        SandboxRecordsLock {}
    }
}

static mut g_sandbox_records_head: *mut SandboxRecord = ptr::null_mut();
static mut g_thread_in_wasm_code: i32 = 0;

const K_INITIAL_CODE_OBJECT_SIZE: usize = 1024;
const K_CODE_OBJECT_GROWTH_FACTOR: usize = 2;

fn handler_data_size(num_protected_instructions: usize) -> usize {
    std::mem::offset_of!(CodeProtectionInfo, instructions)
        + num_protected_instructions * std::mem::size_of::<ProtectedInstructionData>()
}

#[cfg(debug_assertions)]
fn is_disjoint(a: *const CodeProtectionInfo, b: *const CodeProtectionInfo) -> bool {
    if a.is_null() || b.is_null() {
        return true;
    }

    unsafe {
        let a_ref = &*a;
        let b_ref = &*b;
        a_ref.base >= b_ref.base + b_ref.size || b_ref.base >= a_ref.base + a_ref.size
    }
}

#[cfg(debug_assertions)]
fn verify_code_range_is_disjoint(code_info: *const CodeProtectionInfo) {
    unsafe {
        for i in 0..g_num_code_objects {
            let code_objects_ptr = g_code_objects.add(i);
            let code_objects_ref = &*code_objects_ptr;
            assert!(is_disjoint(code_info, code_objects_ref.code_info));
        }
    }
}

#[cfg(debug_assertions)]
fn validate_code_objects() {
    unsafe {
        // Sanity-check the code objects
        for i in 0..g_num_code_objects {
            let code_objects_ptr = g_code_objects.add(i);
            let code_objects_ref = &*code_objects_ptr;
            let data = code_objects_ref.code_info;

            if data.is_null() {
                continue;
            }

            // Do some sanity checks on the protected instruction data
            let data_ref = &*data;
            for j in 0..data_ref.num_protected_instructions {
                assert!((&*data).instructions[j].instr_offset < data_ref.size);
            }
        }

        // Check the validity of the free list.
        let mut free_count = 0;
        let mut i = g_next_code_object;
        while i != g_num_code_objects {
            assert!(i < g_num_code_objects);
            free_count += 1;
            assert!(free_count <= g_num_code_objects); // Check for cycles

            let code_objects_ptr = g_code_objects.add(i);
            let code_objects_ref = &*code_objects_ptr;
            i = code_objects_ref.next_free;
        }

        let mut free_count2 = 0;
        for i in 0..g_num_code_objects {
            let code_objects_ptr = g_code_objects.add(i);
            let code_objects_ref = &*code_objects_ptr;
            if code_objects_ref.code_info.is_null() {
                free_count2 += 1;
            }
        }

        assert_eq!(free_count, free_count2);
    }
}

fn create_handler_data(
    base: usize,
    size: usize,
    num_protected_instructions: usize,
    protected_instructions: *const ProtectedInstructionData,
) -> *mut CodeProtectionInfo {
    let alloc_size = handler_data_size(num_protected_instructions);

    unsafe {
        let data = libc::malloc(alloc_size) as *mut CodeProtectionInfo;

        if data.is_null() {
            return ptr::null_mut();
        }

        (*data).base = base;
        (*data).size = size;
        (*data).num_protected_instructions = num_protected_instructions;

        if num_protected_instructions > 0 {
            libc::memcpy(
                ptr::addr_of_mut!((*data).instructions) as *mut libc::c_void,
                protected_instructions as *const libc::c_void,
                num_protected_instructions * std::mem::size_of::<ProtectedInstructionData>(),
            );
        }
        data
    }
}

pub fn register_handler_data(
    base: usize,
    size: usize,
    num_protected_instructions: usize,
    protected_instructions: *const ProtectedInstructionData,
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

    #[cfg(debug_assertions)]
    unsafe {
        verify_code_range_is_disjoint(data);
    }

    unsafe {
        let i = g_next_code_object;

        let int_max = i32::MAX as usize;

        if i == g_num_code_objects {
            let mut new_size = if g_num_code_objects > 0 {
                g_num_code_objects * K_CODE_OBJECT_GROWTH_FACTOR
            } else {
                K_INITIAL_CODE_OBJECT_SIZE
            };

            if new_size > int_max {
                new_size = int_max;
            }

            if new_size == g_num_code_objects {
                libc::free(data as *mut libc::c_void);
                return kInvalidIndex;
            }

            g_code_objects = libc::realloc(
                g_code_objects as *mut libc::c_void,
                std::mem::size_of::<CodeProtectionInfoListEntry>() * new_size,
            ) as *mut CodeProtectionInfoListEntry;

            if g_code_objects.is_null() {
                std::process::abort();
            }

            libc::memset(
                g_code_objects.add(g_num_code_objects) as *mut libc::c_void,
                0,
                std::mem::size_of::<CodeProtectionInfoListEntry>() * (new_size - g_num_code_objects),
            );
            for j in g_num_code_objects..new_size {
                (*g_code_objects.add(j)).next_free = j + 1;
            }
            g_num_code_objects = new_size;
        }

        if !((*g_code_objects.add(i)).code_info).is_null() {
             println!("register_handler_data: g_code_objects[i].code_info was unexpectedly not null!");
        }
        

        g_next_code_object = (*g_code_objects.add(i)).next_free;

        if i <= int_max {
            (*g_code_objects.add(i)).code_info = data;

            #[cfg(debug_assertions)]
            validate_code_objects();

            return i as i32;
        } else {
            libc::free(data as *mut libc::c_void);
            return kInvalidIndex;
        }
    }
}

pub fn release_handler_data(index: i32) {
    if index == kInvalidIndex {
        return;
    }
    assert!(index >= 0);

    unsafe {
        let mut data: *mut CodeProtectionInfo = ptr::null_mut();
        {
            let i = index as usize;
            data = (*g_code_objects.add(i)).code_info;
            (*g_code_objects.add(i)).code_info = ptr::null_mut();

            (*g_code_objects.add(i)).next_free = g_next_code_object;
            g_next_code_object = i;

            #[cfg(debug_assertions)]
            validate_code_objects();
        }

        assert!(!data.is_null()); // Make sure we're releasing legitimate handler data.
        libc::free(data as *mut libc::c_void);
    }
}

pub fn register_v8_sandbox(base: usize, size: usize) -> bool {
    unsafe {
        let new_record = libc::malloc(std::mem::size_of::<SandboxRecord>()) as *mut SandboxRecord;
        if new_record.is_null() {
            return false;
        }

        (*new_record).base = base;
        (*new_record).size = size;
        (*new_record).next = g_sandbox_records_head;
        g_sandbox_records_head = new_record;
        return true;
    }
}

pub fn unregister_v8_sandbox(base: usize, size: usize) {
    unsafe {
        let mut current = g_sandbox_records_head;
        let mut previous: *mut SandboxRecord = ptr::null_mut();

        while !current.is_null() {
            if (*current).base == base {
                break;
            }
            previous = current;
            current = (*current).next;
        }

        assert!(!current.is_null());
        assert!((*current).size == size);

        if !previous.is_null() {
            (*previous).next = (*current).next;
        } else {
            g_sandbox_records_head = (*current).next;
        }

        libc::free(current as *mut libc::c_void);
    }
}

pub fn get_thread_in_wasm_thread_local_address() -> *mut i32 {
    unsafe { &mut g_thread_in_wasm_code }
}

pub fn get_recovered_trap_count() -> usize {
    g_recovered_trap_count.load(Ordering::Relaxed)
}

extern "C" {
    fn register_default_trap_handler() -> bool;
    fn remove_trap_handler();
}

pub fn enable_trap_handler(use_v8_handler: bool) -> bool {
    let can_enable = g_can_enable_trap_handler.compare_exchange(
        true,
        false,
        Ordering::Relaxed,
        Ordering::Relaxed,
    ).unwrap_or(false);
    
    if !can_enable {
        println!("EnableTrapHandler called twice, or after IsTrapHandlerEnabled.");
        std::process::abort();
    }

    if use_v8_handler {
        let registered = unsafe { register_default_trap_handler() };
        g_is_trap_handler_enabled.store(registered, Ordering::Relaxed);
        registered
    } else {
        g_is_trap_handler_enabled.store(true, Ordering::Relaxed);
        true
    }
}

pub fn set_landing_pad(landing_pad: usize) {
    g_landing_pad.store(landing_pad, Ordering::Relaxed);
}

#[cfg(any(feature = "building_v8_shared_private", feature = "using_v8_shared_private"))]
pub fn assert_thread_not_in_wasm() {
    assert!(!g_is_trap_handler_enabled.load(Ordering::Relaxed) || unsafe { g_thread_in_wasm_code == 0 });
}

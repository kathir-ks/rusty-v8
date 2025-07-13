// Converted from V8 C++ source files:
// Header: base-page-handle.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/include/cppgc/heap-handle.h
pub struct HeapHandle {}

// From /home/kathirks_gc/v8_go/archive/codebase/include/cppgc/internal/logging.h
macro_rules! CPPGC_DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("CPPGC_DCHECK failed: {}", stringify!($condition));
        }
    };
}

// api_constants::kPageSize
const K_PAGE_SIZE: usize = 4096;

pub struct BasePageHandle {
    heap_handle_: HeapHandle,
}

impl BasePageHandle {
    pub fn from_payload(payload: *mut std::ffi::c_void) -> *mut BasePageHandle {
        let address = payload as usize;
        let aligned_address = address & !(K_PAGE_SIZE - 1);
        aligned_address as *mut BasePageHandle
    }

    pub fn from_payload_const(payload: *const std::ffi::c_void) -> *const BasePageHandle {
        BasePageHandle::from_payload(payload as *mut std::ffi::c_void) as *const BasePageHandle
    }

    pub fn heap_handle(&mut self) -> &mut HeapHandle {
        &mut self.heap_handle_
    }
    pub fn heap_handle_const(&self) -> &HeapHandle {
        &self.heap_handle_
    }
}

impl BasePageHandle {
    pub fn new(heap_handle: HeapHandle) -> Self {
        let handle = BasePageHandle {
            heap_handle_: heap_handle,
        };
        CPPGC_DCHECK!(handle as *const _ as usize % K_PAGE_SIZE == 0);
        handle
    }
}

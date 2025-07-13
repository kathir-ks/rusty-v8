// Converted from V8 C++ source files:
// Header: free_deleter.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::alloc::{dealloc, Layout};
    use std::ptr;

    pub struct FreeDeleter;

    impl FreeDeleter {
        #[inline]
        pub fn call(&self, ptr: *mut std::ffi::c_void) {
            unsafe {
                if !ptr.is_null() {
                    let layout = Layout::new::<*mut std::ffi::c_void>();
                    dealloc(ptr as *mut u8, layout);
                }
            }
        }
    }

    pub fn Free(ptr: *mut std::ffi::c_void) {
        unsafe {
            if !ptr.is_null() {
                let layout = Layout::new::<*mut std::ffi::c_void>();
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

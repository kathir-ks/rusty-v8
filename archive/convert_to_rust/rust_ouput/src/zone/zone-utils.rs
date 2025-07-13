// Converted from V8 C++ source files:
// Header: zone-utils.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: *mut T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector {
                data: std::ptr::null_mut(),
                length: 0,
            }
        }

        pub fn from_raw_parts(data: *mut T, length: usize) -> Self {
            Vector { data, length }
        }

        pub fn data(&self) -> *mut T {
            self.data
        }

        pub fn length(&self) -> usize {
            self.length
        }

        pub fn begin(&self) -> *mut T {
            self.data
        }

        pub fn end(&self) -> *mut T {
            unsafe { self.data.add(self.length) }
        }
    }
}

pub mod zone {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;

    pub struct Zone {
        name: String,
    }

    impl Zone {
        pub fn new(name: &str) -> Self {
            Zone {
                name: name.to_string(),
            }
        }

        pub fn allocate_array<T>(&self, length: usize) -> *mut T {
            if length == 0 {
                return std::ptr::null_mut();
            }

            let layout = Layout::array::<T>(length).unwrap();
            unsafe {
                let ptr = alloc(layout) as *mut T;
                if ptr.is_null() {
                    panic!("Allocation failed in Zone::allocate_array");
                }
                ptr
            }
        }
    }
}

pub mod internal {
    use crate::base::Vector;
    use crate::zone::Zone;
    use std::mem;

    pub fn clone_vector<T: Copy>(zone: &Zone, other: Vector<&T>) -> Vector<T> {
        let length = other.length();
        if length == 0 {
            return Vector::new();
        }

        let data = zone.allocate_array::<T>(length);
        if data.is_null() {
            return Vector::new(); // Or handle allocation failure appropriately
        }

        unsafe {
            let source_ptr = other.data() as *const T;
            std::ptr::copy_nonoverlapping(source_ptr, data, length);
        }
        Vector::from_raw_parts(data, length)
    }

    pub fn mem_copy<T>(dest: *mut T, src: *const T, size: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, size);
        }
    }
}

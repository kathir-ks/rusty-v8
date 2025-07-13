// Converted from V8 C++ source files:
// Header: virtual-memory.h
// Implementation: virtual-memory.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

use std::mem::size_of;
use std::ptr::null_mut;

pub trait PageAllocator {
    fn allocate_page_size(&self) -> usize;
    fn commit_page_size(&self) -> usize;
    fn allocate_pages(&self, hint: *mut std::ffi::c_void, size: usize, alignment: usize, access: AccessMode) -> *mut std::ffi::c_void;
    fn free_pages(&self, start: *mut std::ffi::c_void, size: usize);
}

#[derive(Clone, Copy)]
pub enum AccessMode {
    ReadWrite,
    ReadOnly,
    NoAccess,
}

pub struct VirtualMemory {
    page_allocator: Option<Box<dyn PageAllocator>>,
    start: *mut std::ffi::c_void,
    size: usize,
}

impl VirtualMemory {
    pub fn new() -> Self {
        VirtualMemory {
            page_allocator: None,
            start: null_mut(),
            size: 0,
        }
    }

    pub fn with_page_allocator(page_allocator: Box<dyn PageAllocator>, size: usize, alignment: usize, hint: *mut std::ffi::c_void) -> Self {
        assert!(is_aligned(size, page_allocator.commit_page_size()));

        let page_size = page_allocator.allocate_page_size();
        let aligned_size = round_up(size, page_size);
        let aligned_alignment = round_up(alignment, page_size);

        let start = page_allocator.allocate_pages(hint, aligned_size, aligned_alignment, AccessMode::NoAccess);

        if !start.is_null() {
            VirtualMemory {
                page_allocator: Some(page_allocator),
                start,
                size: aligned_size,
            }
        } else {
            VirtualMemory {
                page_allocator: Some(page_allocator),
                start: null_mut(),
                size: 0,
            }
        }
    }

    pub fn is_reserved(&self) -> bool {
        !self.start.is_null()
    }

    pub fn address(&self) -> *mut std::ffi::c_void {
        assert!(self.is_reserved());
        self.start
    }

    pub fn size(&self) -> usize {
        assert!(self.is_reserved());
        self.size
    }

    fn reset(&mut self) {
        self.start = null_mut();
        self.size = 0;
    }
}

impl Drop for VirtualMemory {
    fn drop(&mut self) {
        if self.is_reserved() {
            if let Some(page_allocator) = &self.page_allocator {
                page_allocator.free_pages(self.start, self.size);
            }
        }
    }
}

impl VirtualMemory {
    pub fn move_from(&mut self, other: &mut VirtualMemory) {
        assert!(!self.is_reserved());
        self.page_allocator = other.page_allocator.take();
        self.start = other.start;
        self.size = other.size;
        other.reset();
    }
}

fn is_aligned(value: usize, alignment: usize) -> bool {
    value % alignment == 0
}

fn round_up(value: usize, alignment: usize) -> usize {
    if value % alignment == 0 {
        value
    } else {
        (value / alignment + 1) * alignment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPageAllocator {
        allocate_page_size: usize,
        commit_page_size: usize,
        allocated: Vec<(*mut std::ffi::c_void, usize)>,
    }

    impl MockPageAllocator {
        fn new(allocate_page_size: usize, commit_page_size: usize) -> Self {
            MockPageAllocator {
                allocate_page_size,
                commit_page_size,
                allocated: Vec::new(),
            }
        }
    }

    impl PageAllocator for MockPageAllocator {
        fn allocate_page_size(&self) -> usize {
            self.allocate_page_size
        }

        fn commit_page_size(&self) -> usize {
            self.commit_page_size
        }

        fn allocate_pages(&self, _hint: *mut std::ffi::c_void, size: usize, _alignment: usize, _access: AccessMode) -> *mut std::ffi::c_void {
            let layout = std::alloc::Layout::from_size_align(size, self.allocate_page_size).unwrap();
            unsafe {
                let ptr = std::alloc::alloc(layout) as *mut std::ffi::c_void;
                 println!("Allocated {} bytes at {:?}", size, ptr);
                ptr
            }
        }

        fn free_pages(&self, start: *mut std::ffi::c_void, size: usize) {
            if start.is_null() {
                return;
            }
            let layout = std::alloc::Layout::from_size_align(size, self.allocate_page_size()).unwrap();
            unsafe {
                std::alloc::dealloc(start as *mut u8, layout);
                println!("Freed {} bytes at {:?}", size, start);
            }
        }
    }

    #[test]
    fn test_virtual_memory_allocation() {
        let allocate_page_size = 4096;
        let commit_page_size = 2048;
        let page_allocator = Box::new(MockPageAllocator::new(allocate_page_size, commit_page_size));
        let size = 5000;
        let alignment = 1024;

        let vm = VirtualMemory::with_page_allocator(page_allocator, size, alignment, null_mut());
        assert!(vm.is_reserved());
        assert!(vm.size() == 8192); // Round up to allocate_page_size
    }

    #[test]
    fn test_virtual_memory_drop() {
        let allocate_page_size = 4096;
        let commit_page_size = 2048;
        let page_allocator = Box::new(MockPageAllocator::new(allocate_page_size, commit_page_size));
        let size = 5000;
        let alignment = 1024;

        let vm = VirtualMemory::with_page_allocator(page_allocator, size, alignment, null_mut());
        let addr = vm.address();
        drop(vm); // Should deallocate
        assert!(!addr.is_null());

    }

    #[test]
    fn test_virtual_memory_move() {
        let allocate_page_size = 4096;
        let commit_page_size = 2048;
        let page_allocator = Box::new(MockPageAllocator::new(allocate_page_size, commit_page_size));
        let size = 5000;
        let alignment = 1024;

        let mut vm1 = VirtualMemory::with_page_allocator(page_allocator, size, alignment, null_mut());
        let addr = vm1.address();
        let mut vm2 = VirtualMemory::new();
        vm2.move_from(&mut vm1);

        assert!(vm1.address().is_null());
        assert!(vm2.address() == addr);
    }
}
}
}

// Converted from V8 C++ source files:
// Header: free-list.h
// Implementation: free-list.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

use std::{
    array,
    cmp::max,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{Mutex, MutexGuard},
};

use crate::HeapStatistics;

const kFreeListGCInfoIndex: usize = 0; // Define a dummy value.  The actual value and usage isn't clear from provided context.
const kPageSizeLog2: usize = 20;
const kPageSize: usize = 1 << kPageSizeLog2;
const kFreeListEntrySize: usize = 16;

pub struct Filler {
    header: HeapObjectHeader,
}

impl Filler {
    pub fn create_at(memory: *mut std::ffi::c_void, size: usize) -> &'static mut Filler {
        unsafe {
            //ASAN_UNPOISON_MEMORY_REGION(memory, size_of::<Filler>()); // Removed ASAN macro

            let filler_ptr = memory as *mut Filler;
            filler_ptr.write(Filler {
                header: HeapObjectHeader::new(size, kFreeListGCInfoIndex),
            });

            &mut *filler_ptr
        }
    }

    pub fn object_end(&self) -> usize {
        (self as *const Self as usize) + self.header.size()
    }
}

#[derive(Debug)]
pub struct Block {
    pub address: *mut std::ffi::c_void,
    pub size: usize,
}

impl Block {
    pub fn new(address: *mut std::ffi::c_void, size: usize) -> Self {
        Block { address, size }
    }
}

pub struct FreeList {
    free_list_heads: array::[Option<NonNull<Entry>>; kPageSizeLog2],
    free_list_tails: array::[Option<NonNull<Entry>>; kPageSizeLog2],
    biggest_free_list_index: usize,
}

impl FreeList {
    pub fn new() -> Self {
        FreeList {
            free_list_heads: array::from_fn(|_| None),
            free_list_tails: array::from_fn(|_| None),
            biggest_free_list_index: 0,
        }
    }

    pub fn allocate(&mut self, allocation_size: usize) -> Block {
        let mut bucket_size = 1 << self.biggest_free_list_index;
        let mut index = self.biggest_free_list_index;

        while index > 0 {
            if let Some(entry_ptr) = self.free_list_heads[index].as_ref() {
                let entry = unsafe { entry_ptr.as_ref() };

                if allocation_size > bucket_size {
                    if entry.header.size() < allocation_size {
                        index -= 1;
                        bucket_size >>= 1;
                        continue;
                    }
                }

                let entry_size = entry.header.size();
                let entry_address = entry as *const Entry as *mut std::ffi::c_void;

                let mut entry_ptr_mut = self.free_list_heads[index].take().unwrap();
                let mut entry_mut = unsafe { entry_ptr_mut.as_mut() };
                
                if let Some(next) = entry_mut.next.take() {
                    entry_mut.unlink(&mut self.free_list_heads[index]);
                    self.free_list_heads[index] = NonNull::new(next);
                    if self.free_list_heads[index].is_none() {
                        self.free_list_tails[index] = None;
                    }
                }
                
                if self.free_list_tails[index] == Some(NonNull::from(entry)) {
                    self.free_list_tails[index] = None;
                }

                self.biggest_free_list_index = index;
                return Block::new(entry_address, entry_size);
            }

            index -= 1;
            bucket_size >>= 1;
        }

        self.biggest_free_list_index = index;
        Block::new(std::ptr::null_mut(), 0)
    }

    pub fn add(&mut self, block: Block) {
        self.add_returning_unused_bounds(block);
    }

    pub fn add_returning_unused_bounds(&mut self, block: Block) -> (usize, usize) {
        let size = block.size;

        if size < size_of::<Entry>() {
            let filler = Filler::create_at(block.address, size);
            let start = (filler as *const Filler as usize) + size_of::<Filler>();
            return (start, start);
        }

        let entry = Entry::create_at(block.address, size);
        let index = bucket_index_for_size(size as u32) as usize;

        unsafe {
            let entry_ptr = NonNull::new(entry).unwrap();
            
            if let Some(mut head) = self.free_list_heads[index] {
                entry_ptr.as_mut().next = Some(head.as_ptr());
            }
            entry_ptr.as_mut().prev = None;

            self.free_list_heads[index] = Some(entry_ptr);

            if self.free_list_tails[index].is_none() {
                self.free_list_tails[index] = self.free_list_heads[index];
            }
        }
        self.biggest_free_list_index = max(self.biggest_free_list_index, index);

        let start = (entry as *const Entry as usize) + size_of::<Entry>();
        let end = (entry as *const Entry as usize) + size;

        (start, end)
    }

    pub fn append(&mut self, mut other: FreeList) {
        for index in 0..self.free_list_tails.len() {
            if let Some(other_tail_ptr) = other.free_list_tails[index].take() {
                let other_tail = unsafe { other_tail_ptr.as_ref() };
                
                if let Some(this_head_ptr) = self.free_list_heads[index] {
                    unsafe {
                        if let Some(mut other_tail_mut_ptr) = other.free_list_tails[index].take() {
                            other_tail_mut_ptr.as_mut().next = Some(this_head_ptr.as_ptr());
                        }
                    }
                } else {
                    self.free_list_tails[index] = other.free_list_tails[index];
                }
                self.free_list_heads[index] = other.free_list_heads[index];
                other.free_list_heads[index] = None;
                other.free_list_tails[index] = None;
            }
        }

        self.biggest_free_list_index = max(
            self.biggest_free_list_index,
            other.biggest_free_list_index,
        );
        other.biggest_free_list_index = 0;
    }

    pub fn clear(&mut self) {
        self.free_list_heads.fill(None);
        self.free_list_tails.fill(None);
        self.biggest_free_list_index = 0;
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        for entry_option in &self.free_list_heads {
            let mut entry = entry_option.as_ref();
            while let Some(e) = entry {
                size += unsafe { e.as_ref().header.size() };
                entry = unsafe { e.as_ref().next_ptr() };
            }
        }
        size
    }

    pub fn is_empty(&self) -> bool {
        self.free_list_heads.iter().all(|&entry| entry.is_none())
    }

    pub fn collect_statistics(&self, free_list_stats: &mut HeapStatistics::FreeListStatistics) {
        let mut bucket_size = Vec::new();
        let mut free_count = Vec::new();
        let mut free_size = Vec::new();

        for i in 0..kPageSizeLog2 {
            let mut entry_count = 0;
            let mut entry_size = 0;

            let mut entry = self.free_list_heads[i];
            while let Some(e) = entry {
                entry_count += 1;
                entry_size += unsafe { e.as_ref().header.size() };
                entry = unsafe { e.as_ref().next_ptr() };
            }

            bucket_size.push(1 << i);
            free_count.push(entry_count);
            free_size.push(entry_size);
        }

        free_list_stats.bucket_size = bucket_size;
        free_list_stats.free_count = free_count;
        free_list_stats.free_size = free_size;
    }

    pub fn contains_for_testing(&self, block: Block) -> bool {
        for list in &self.free_list_heads {
            let mut entry = list.as_ref();
            while let Some(e) = entry {
                let entry_ptr = unsafe { e.as_ref() } as *const Entry as usize;
                let block_addr = block.address as usize;

                if entry_ptr <= block_addr
                    && (block_addr + block.size <= entry_ptr + unsafe { e.as_ref().header.size() })
                {
                    return true;
                }
                entry = unsafe { e.as_ref().next_ptr() };
            }
        }
        false
    }
}

struct Entry {
    header: HeapObjectHeader,
    next: Option<*mut Entry>,
    prev: Option<*mut Entry>,
}

impl Entry {
    fn create_at(memory: *mut std::ffi::c_void, size: usize) -> *mut Entry {
        unsafe {
            //ASAN_UNPOISON_MEMORY_REGION(memory, size_of::<Entry>()); // Removed ASAN macro

            let entry_ptr = memory as *mut Entry;
            entry_ptr.write(Entry {
                header: HeapObjectHeader::new(size, kFreeListGCInfoIndex),
                next: None,
                prev: None,
            });
            entry_ptr
        }
    }
    
    fn next_ptr(&self) -> Option<NonNull<Entry>> {
        match self.next {
            Some(ptr) => NonNull::new(ptr),
            None => None,
        }
    }

    fn link(&mut self, previous_next: &mut Option<NonNull<Entry>>) {
        self.next = previous_next.map(|nn| nn.as_ptr());
        *previous_next = NonNull::new(self);
    }

    fn unlink(&mut self, previous_next: &mut Option<NonNull<Entry>>) {
        if let Some(next_ptr) = self.next {
            if let Some(prev_ptr) = self.prev {
                unsafe { (*prev_ptr).next = Some(next_ptr); }
                unsafe { (*next_ptr).prev = Some(prev_ptr); }
            } else {
                *previous_next = self.next_ptr();
                if let Some(next_ptr) = self.next {
                    unsafe { (*next_ptr).prev = None; }
                }
            }
        } else {
             *previous_next = None;
        }
        
        self.next = None;
        self.prev = None;
    }
}

fn bucket_index_for_size(size: u32) -> u32 {
    v8::base::bits::which_power_of_two(v8::base::bits::round_down_to_power_of_two32(size))
}

} // namespace internal
} // namespace cppgc

pub mod v8 {
pub mod base {
    pub mod bits {
        pub fn which_power_of_two(value: u32) -> u32 {
            if value == 0 {
                return 0;
            }
            31 - value.leading_zeros()
        }

        pub fn round_down_to_power_of_two32(value: u32) -> u32 {
            if value == 0 {
                return 0;
            }
            let mut result = 1;
            while result <= value {
                result <<= 1;
            }
            result >> 1
        }
    }
}
}
pub mod HeapStatistics {
    pub struct FreeListStatistics {
        pub bucket_size: Vec<usize>,
        pub free_count: Vec<usize>,
        pub free_size: Vec<usize>,
    }

    impl FreeListStatistics {
        pub fn new() -> Self {
            FreeListStatistics {
                bucket_size: Vec::new(),
                free_count: Vec::new(),
                free_size: Vec::new(),
            }
        }
    }
}
pub mod internal {
    use std::mem::size_of;

    #[derive(Debug, Clone, Copy)]
    pub struct HeapObjectHeader {
        size_: usize,
        gc_info_index_: usize,
    }

    impl HeapObjectHeader {
        pub fn new(size: usize, gc_info_index: usize) -> Self {
            HeapObjectHeader {
                size_: size,
                gc_info_index_: gc_info_index,
            }
        }

        pub fn size(&self) -> usize {
            self.size_
        }
    }
}
}

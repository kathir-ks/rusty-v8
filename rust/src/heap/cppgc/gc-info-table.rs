// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::alloc::{alloc, dealloc, Layout};
use std::cmp;
use std::mem;
use std::ptr;
use std::sync::{atomic, Mutex, MutexGuard};

use crate::base::bits;
use crate::base::lazy_instance::LeakyObject;
use crate::base::page_allocator::PageAllocator;
use crate::heap::cppgc::platform::GetGlobalOOMHandler;

pub mod internal {
    use super::*;

    // TODO(saikat): Figure out the appropriate representation for GCInfo.
    #[derive(Clone, Copy)]
    pub struct GCInfo {
        // Placeholder for actual GCInfo fields
        _padding: [u8; 64], // Example size
    }

    #[derive(Debug)]
    pub struct GCInfoTable {
        page_allocator_: Box<dyn PageAllocator>,
        oom_handler_: FatalOutOfMemoryHandler,
        table_: *mut GCInfo,
        read_only_table_end_: *mut u8,
        limit_: GCInfoIndex,
        current_index_: GCInfoIndex,
        table_mutex_: Mutex<()>,
    }

    pub type GCInfoIndex = usize;

    impl GCInfoTable {
        pub const kMaxIndex: GCInfoIndex = (1 << 20) - 1; // Example Value
        pub const kMinIndex: GCInfoIndex = 0;
        pub const kInitialWantedLimit: GCInfoIndex = 256; // Example Value

        pub fn new(
            page_allocator_: Box<dyn PageAllocator>,
            oom_handler_: FatalOutOfMemoryHandler,
        ) -> Self {
            let allocate_page_size = page_allocator_.allocate_page_size();
            let max_table_size = Self::round_up(
                Self::kMaxIndex * mem::size_of::<GCInfo>(),
                allocate_page_size,
            );
            let layout = Layout::from_size_align(max_table_size, allocate_page_size).unwrap();
            let table_ = unsafe { alloc(layout) as *mut GCInfo };
            if table_.is_null() {
              oom_handler_("Oilpan: GCInfoTable initial reservation.");
              panic!();
            }
            unsafe { ptr::write_bytes(table_, 0, max_table_size) };

            let mut result = Self {
                page_allocator_: page_allocator_,
                oom_handler_: oom_handler_,
                table_: table_,
                read_only_table_end_: table_ as *mut u8,
                limit_: 0,
                current_index_: 0,
                table_mutex_: Mutex::new(()),
            };
            result.Resize();
            result
        }

        fn round_up(x: usize, multiple: usize) -> usize {
            (x + multiple - 1) & !(multiple - 1)
        }

        pub fn allocator(&self) -> &dyn PageAllocator {
            self.page_allocator_.as_ref()
        }

        pub fn max_table_size(&self) -> usize {
            Self::round_up(
                GCInfoTable::kMaxIndex * mem::size_of::<GCInfo>(),
                self.page_allocator_.allocate_page_size(),
            )
        }

        pub fn initial_table_limit(&self) -> GCInfoIndex {
            let memory_wanted = Self::kInitialWantedLimit * mem::size_of::<GCInfo>();
            let initial_limit = Self::round_up(
                memory_wanted,
                self.page_allocator_.allocate_page_size(),
            ) / mem::size_of::<GCInfo>();
            cmp::min(GCInfoTable::kMaxIndex, initial_limit)
        }

        pub fn Resize(&mut self) {
            let new_limit = if self.limit_ > 0 {
                2 * self.limit_
            } else {
                self.InitialTableLimit()
            };
            assert!(new_limit > self.limit_);
            let old_committed_size = self.limit_ * mem::size_of::<GCInfo>();
            let new_committed_size = new_limit * mem::size_of::<GCInfo>();
            assert!(!self.table_.is_null());
            assert_eq!(
                0,
                new_committed_size % self.page_allocator_.allocate_page_size()
            );
            assert!(self.MaxTableSize() >= new_committed_size);

            let current_table_end = unsafe { (self.table_ as *mut u8).add(old_committed_size) };
            let table_size_delta = new_committed_size - old_committed_size;

            if !self.page_allocator_.set_permissions(
                current_table_end as *mut u8,
                table_size_delta,
                crate::base::page_allocator::PageAllocatorPermissions::ReadWrite,
            ) {
                (self.oom_handler_)("Oilpan: GCInfoTable resize.");
                panic!();
            }

            if self.read_only_table_end_ != current_table_end {
                assert!(current_table_end > self.read_only_table_end_);
                let read_only_delta = current_table_end as usize - self.read_only_table_end_ as usize;
                assert!(self.page_allocator_.set_permissions(
                    self.read_only_table_end_,
                    read_only_delta,
                    crate::base::page_allocator::PageAllocatorPermissions::Read,
                ));
                self.read_only_table_end_ = unsafe { self.read_only_table_end_.add(read_only_delta) };
            }

            self.CheckMemoryIsZeroed(
                current_table_end as *mut usize,
                table_size_delta / mem::size_of::<usize>(),
            );

            self.limit_ = new_limit;
        }

        #[cfg(debug_assertions)]
        fn CheckMemoryIsZeroed(&self, base: *mut usize, len: usize) {
            for i in 0..len {
                unsafe {
                    assert_eq!(*base.add(i), 0);
                }
            }
        }

        #[cfg(not(debug_assertions))]
        fn CheckMemoryIsZeroed(&self, _base: *mut usize, _len: usize) {}

        pub fn RegisterNewGCInfo(
            &mut self,
            registered_index: &atomic::AtomicUsize,
            info: GCInfo,
        ) -> GCInfoIndex {
            let guard: MutexGuard<'_, ()> = self.table_mutex_.lock().unwrap();

            let index = registered_index.load(atomic::Ordering::Relaxed);
            if index > 0 {
                return index;
            }

            if self.current_index_ == self.limit_ {
                self.Resize();
            }

            let new_index = self.current_index_;
            self.current_index_ += 1;
            assert!(new_index < GCInfoTable::kMaxIndex);
            unsafe {
                *self.table_.add(new_index) = info;
            }
            registered_index.store(new_index, atomic::Ordering::Release);
            new_index
        }

        pub fn table(&self) -> *mut GCInfo {
            self.table_
        }
    }

    impl Drop for GCInfoTable {
        fn drop(&mut self) {
            let layout = Layout::from_size_align(self.MaxTableSize(), self.page_allocator_.allocate_page_size()).unwrap();
            unsafe {
                dealloc(self.table_ as *mut u8, layout);
            }
        }
    }

    pub struct GlobalGCInfoTable {
        global_table_: Option<LeakyObject<GCInfoTable>>,
    }

    impl GlobalGCInfoTable {
        pub fn new() -> Self {
            GlobalGCInfoTable {
                global_table_: None,
            }
        }

        pub fn Initialize(&mut self, page_allocator: Box<dyn PageAllocator>) {
            let oom_handler = GetGlobalOOMHandler();
            if self.global_table_.is_none() {
                let table = LeakyObject::new(GCInfoTable::new(page_allocator, oom_handler));
                self.global_table_ = Some(table);
            } else {
                // TODO: Add a check that the page_allocator is the same as the one used to initialize the table
                // if let Some(table) = &self.global_table_ {
                //    assert_eq!(page_allocator, table.get().allocator());
                // }
            }
        }

        pub fn get(&self) -> &GCInfoTable {
            self.global_table_.as_ref().unwrap().get()
        }
    }
}

pub mod base {
    pub mod bits {
        pub fn is_power_of_two(x: usize) -> bool {
            (x != 0) && ((x & (x - 1)) == 0)
        }
    }

    pub mod lazy_instance {
        pub struct LeakyObject<T> {
            instance: Box<T>,
        }

        impl<T> LeakyObject<T> {
            pub fn new(instance: T) -> Self {
                LeakyObject {
                    instance: Box::new(instance),
                }
            }

            pub fn get(&self) -> &T {
                &self.instance
            }
        }
    }

    pub mod page_allocator {
        #[derive(Debug, PartialEq)]
        pub enum PageAllocatorPermissions {
            NoAccess,
            ReadWrite,
            Read,
        }

        pub trait PageAllocator {
            fn allocate_page_size(&self) -> usize;
            fn allocate_pages(
                &self,
                base: *mut u8,
                size: usize,
                alignment: usize,
                permissions: PageAllocatorPermissions,
            ) -> *mut u8;
            fn release_pages(&self, base: *mut u8, size: usize, offset: usize);
            fn set_permissions(
                &self,
                base: *mut u8,
                size: usize,
                permissions: PageAllocatorPermissions,
            ) -> bool;
        }
    }
}

pub mod heap {
    pub mod cppgc {
        pub mod platform {
            use super::super::super::FatalOutOfMemoryHandler;

            static mut GLOBAL_OOM_HANDLER: Option<FatalOutOfMemoryHandler> = None;

            pub fn GetGlobalOOMHandler() -> FatalOutOfMemoryHandler {
                unsafe {
                  GLOBAL_OOM_HANDLER.unwrap_or_else(|| {
                      let default_handler: FatalOutOfMemoryHandler = Box::new(|msg: &str| {
                          eprintln!("Default OOM Handler: {}", msg);
                          std::process::abort();
                      });
                      default_handler
                  })
                }
            }

            pub fn SetGlobalOOMHandler(handler: FatalOutOfMemoryHandler) {
                unsafe {
                    GLOBAL_OOM_HANDLER = Some(handler);
                }
            }
        }
    }
}

// type definition for FatalOutOfMemoryHandler
pub type FatalOutOfMemoryHandler = Box<dyn Fn(&str)>;
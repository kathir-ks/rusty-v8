// Converted from V8 C++ source files:
// Header: gc-info-table.h
// Implementation: gc-info-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod gc_info_table {
    use std::sync::{Mutex, MutexGuard};
    use std::sync::atomic::{AtomicU16, Ordering};

    use crate::heap::cppgc::platform::{PageAllocator, FatalOutOfMemoryHandler, GetGlobalOOMHandler};
    use crate::init::v8::OS;

    use v8::base::bits::IsPowerOfTwo;
    use v8::base::lazy_instance::LeakyObject;

    pub type FinalizationCallback = *mut std::ffi::c_void; // Replace with a more specific type if known
    pub type TraceCallback = *mut std::ffi::c_void; // Replace with a more specific type if known
    pub type NameCallback = *mut std::ffi::c_void; // Replace with a more specific type if known

    // GCInfo contains metadata for objects that are instantiated from classes that
    // inherit from GarbageCollected.
    #[derive(Copy, Clone)]
    pub struct GCInfo {
        pub finalize: FinalizationCallback,
        pub trace: TraceCallback,
        pub name: NameCallback,
        pub padding: usize,
    }

    impl GCInfo {
        pub const fn new(finalize: FinalizationCallback, trace: TraceCallback, name: NameCallback) -> Self {
            Self {
                finalize,
                trace,
                name,
                padding: 0,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GCInfoIndex(pub u16);

    pub struct GCInfoTable {
        page_allocator_: PageAllocator,
        oom_handler_: FatalOutOfMemoryHandler,
        // Holds the per-class GCInfo descriptors; each HeapObjectHeader keeps an
        // index into this table.
        table_: *mut GCInfo,
        read_only_table_end_: *mut u8,
        // Current index used when requiring a new GCInfo object.
        current_index_: GCInfoIndex,
        // The limit (exclusive) of the currently allocated table.
        limit_: GCInfoIndex,
        table_mutex_: Mutex<()>,
    }

    impl GCInfoTable {
        // At maximum |kMaxIndex - 1| indices are supported.
        //
        // We assume that 14 bits are enough to represent all possible types.
        //
        // For Blink during telemetry runs, we see about 1,000 different types;
        // looking at the output of the Oilpan GC clang plugin, there appear to be at
        // most about 6,000 types. Thus 14 bits should be more than twice as many bits
        // as we will ever need. Different contexts may require adjusting this limit.
        pub const K_MAX_INDEX: GCInfoIndex = GCInfoIndex(1 << 14);

        // Minimum index returned. Values smaller |kMinIndex| may be used as
        // sentinels.
        pub const K_MIN_INDEX: GCInfoIndex = GCInfoIndex(1);

        // (Light) experimentation suggests that Blink doesn't need more than this
        // while handling content on popular web properties.
        pub const K_INITIAL_WANTED_LIMIT: GCInfoIndex = GCInfoIndex(512);

        pub fn new(page_allocator: PageAllocator, oom_handler: FatalOutOfMemoryHandler) -> Self {
            let mut table = GCInfoTable {
                page_allocator_: page_allocator,
                oom_handler_: oom_handler,
                table_: std::ptr::null_mut(),
                read_only_table_end_: std::ptr::null_mut(),
                current_index_: Self::K_MIN_INDEX,
                limit_: GCInfoIndex(0),
                table_mutex_: Mutex::new(()),
            };

            unsafe {
                table.table_ = table.page_allocator_.allocate_pages(
                    std::ptr::null_mut(),
                    table.max_table_size(),
                    table.page_allocator_.allocate_page_size(),
                    PageAllocator::K_NO_ACCESS,
                ) as *mut GCInfo;
                table.read_only_table_end_ = table.table_ as *mut u8;
            }
            if table.table_.is_null() {
                (table.oom_handler_)("Oilpan: GCInfoTable initial reservation.");
            }
            table.resize();
            table
        }

        fn resize(&mut self) {
            let new_limit = if self.limit_.0 == 0 {
                self.initial_table_limit()
            } else {
                GCInfoIndex(2 * self.limit_.0)
            };
            if new_limit > self.limit_ {
                let old_committed_size = self.limit_.0 as usize * std::mem::size_of::<GCInfo>();
                let new_committed_size = new_limit.0 as usize * std::mem::size_of::<GCInfo>();

                if self.table_.is_null() == false {
                    assert_eq!(0, new_committed_size % self.page_allocator_.allocate_page_size());
                    assert!(self.max_table_size() >= new_committed_size);

                    // Recommit new area as read/write.
                    unsafe {
                        let current_table_end = (self.table_ as *mut u8).add(old_committed_size);
                        let table_size_delta = new_committed_size - old_committed_size;
                        if !self.page_allocator_.set_permissions(
                            current_table_end as *mut std::ffi::c_void,
                            table_size_delta,
                            PageAllocator::K_READ_WRITE,
                        ) {
                            (self.oom_handler_)("Oilpan: GCInfoTable resize.");
                        }

                        // Recommit old area as read-only.
                        if self.read_only_table_end_ != current_table_end {
                            assert!(current_table_end > self.read_only_table_end_);
                            let read_only_delta = current_table_end as usize - self.read_only_table_end_ as usize;
                            assert!(self.page_allocator_.set_permissions(
                                self.read_only_table_end_ as *mut std::ffi::c_void,
                                read_only_delta,
                                PageAllocator::K_READ,
                            ));
                            self.read_only_table_end_ = self.read_only_table_end_.add(read_only_delta);
                        }

                        // Check that newly-committed memory is zero-initialized.
                        self.check_memory_is_zeroed(
                            current_table_end as *mut usize,
                            table_size_delta / std::mem::size_of::<usize>(),
                        );
                    }

                    self.limit_ = new_limit;
                }
            }
        }

        fn check_memory_is_zeroed(&self, base: *mut usize, len: usize) {
            #[cfg(debug_assertions)]
            unsafe {
                for i in 0..len {
                    assert_eq!(*base.add(i), 0);
                }
            }
        }

        pub fn register_new_gc_info(
            &mut self,
            registered_index: &AtomicU16,
            info: &GCInfo,
        ) -> GCInfoIndex {
            let guard = self.table_mutex_.lock().unwrap();

            let index = GCInfoIndex(registered_index.load(Ordering::Relaxed));
            if index.0 != 0 {
                return index;
            }

            if self.current_index_ == self.limit_ {
                self.resize();
            }

            let new_index = self.current_index_;
            self.current_index_.0 += 1;
            assert!(new_index < Self::K_MAX_INDEX);

            unsafe {
                *self.table_.add(new_index.0 as usize) = *info;
            }

            registered_index.store(new_index.0, Ordering::Release);
            new_index
        }

        pub fn gc_info_from_index(&self, index: GCInfoIndex) -> GCInfo {
            assert!(index >= Self::K_MIN_INDEX);
            assert!(index < Self::K_MAX_INDEX);
            assert!(!self.table_.is_null());
            unsafe { *self.table_.add(index.0 as usize) }
        }

        pub fn number_of_gc_infos(&self) -> GCInfoIndex {
            self.current_index_
        }

        pub fn limit_for_testing(&self) -> GCInfoIndex {
            self.limit_
        }

        pub fn table_slot_for_testing(&mut self, index: GCInfoIndex) -> &mut GCInfo {
            unsafe { &mut *self.table_.add(index.0 as usize) }
        }

        pub fn allocator(&self) -> &PageAllocator {
            &self.page_allocator_
        }

        fn initial_table_limit(&self) -> GCInfoIndex {
            // Different OSes have different page sizes, so we have to choose the minimum
            // of memory wanted and OS page size.
            let memory_wanted = Self::K_INITIAL_WANTED_LIMIT.0 as usize * std::mem::size_of::<GCInfo>();
            let initial_limit =
                round_up(memory_wanted, self.page_allocator_.allocate_page_size()) / std::mem::size_of::<GCInfo>();
            assert!(std::u16::MAX as usize > initial_limit);
            GCInfoIndex(std::cmp::min(Self::K_MAX_INDEX.0 as usize, initial_limit) as u16)
        }

        fn max_table_size(&self) -> usize {
            round_up(
                Self::K_MAX_INDEX.0 as usize * std::mem::size_of::<GCInfo>(),
                self.page_allocator_.allocate_page_size(),
            )
        }
    }

    impl Drop for GCInfoTable {
        fn drop(&mut self) {
            if !self.table_.is_null() {
                unsafe {
                    self.page_allocator_.release_pages(
                        self.table_ as *mut std::ffi::c_void,
                        self.max_table_size(),
                        0,
                    );
                }
            }
        }
    }

    pub struct GlobalGCInfoTable {}

    impl GlobalGCInfoTable {
        static GLOBAL_TABLE: LeakyObject<Option<GCInfoTable>> = LeakyObject::new(None);

        pub fn initialize(page_allocator: PageAllocator) {
            unsafe {
                let mut global_table = GLOBAL_TABLE.get();
                if global_table.is_none() {
                    let table = GCInfoTable::new(page_allocator, GetGlobalOOMHandler());
                    *global_table = Some(table);
                } else {
                    assert_eq!(
                        &page_allocator as *const _,
                        &global_table.as_ref().unwrap().allocator() as *const _
                    );
                }
            }
        }

        pub fn get_mutable() -> &'static mut GCInfoTable {
            unsafe {
                let global_table = GLOBAL_TABLE.get();
                global_table.as_mut().unwrap()
            }
        }

        pub fn get() -> &'static GCInfoTable {
            unsafe {
                let global_table = GLOBAL_TABLE.get();
                global_table.as_ref().unwrap()
            }
        }

        pub fn gc_info_from_index(index: GCInfoIndex) -> GCInfo {
            Self::get().gc_info_from_index(index)
        }
    }

    fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }
}

mod v8 {
    pub mod base {
        pub mod bits {
            pub fn IsPowerOfTwo(x: usize) -> bool {
                (x != 0) && ((x & (x - 1)) == 0)
            }
        }

        pub mod lazy_instance {
            pub struct LeakyObject<T> {
                obj: T,
            }

            impl<T> LeakyObject<T> {
                pub const fn new(obj: T) -> Self {
                    LeakyObject { obj }
                }

                pub fn get(&'static self) -> &'static mut T {
                    unsafe {
                        let ptr: *const T = &self.obj;
                        (ptr as *mut T).as_mut().unwrap()
                    }
                }
            }
        }
    }
}

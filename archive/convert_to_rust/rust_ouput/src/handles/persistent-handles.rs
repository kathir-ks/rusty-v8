// Converted from V8 C++ source files:
// Header: persistent-handles.h
// Implementation: persistent-handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod persistent_handles {
    use std::sync::Mutex;
    use std::vec::Vec;
    use crate::api::api::Isolate;
    use crate::internal::{
        IndirectHandle,
        HandleScopeImplementer,
        LocalHeap,
        kHandleBlockSize,
    };
    use crate::objects::objects::Tagged;
    use crate::internal::Allocation::NewArray;
    use crate::objects::visitors::RootVisitor;
    use crate::internal::Root;
    use std::ptr::null_mut;

    pub struct PersistentHandles {
        isolate_: *mut Isolate,
        blocks_: Vec<*mut Address>,
        block_next_: *mut Address,
        block_limit_: *mut Address,
        prev_: *mut PersistentHandles,
        next_: *mut PersistentHandles,
        #[cfg(debug_assertions)]
        ordered_blocks_: std::collections::HashSet<*mut Address>,
        #[cfg(debug_assertions)]
        owner_: *mut LocalHeap,
    }

    impl PersistentHandles {
        pub fn new(isolate: *mut Isolate) -> PersistentHandles {
            let mut persistent_handles = PersistentHandles {
                isolate_: isolate,
                blocks_: Vec::new(),
                block_next_: null_mut(),
                block_limit_: null_mut(),
                prev_: null_mut(),
                next_: null_mut(),
                #[cfg(debug_assertions)]
                ordered_blocks_: std::collections::HashSet::new(),
                #[cfg(debug_assertions)]
                owner_: null_mut(),
            };
            unsafe {
                (*(*isolate).persistent_handles_list_).add(&mut persistent_handles);
            }

            persistent_handles
        }

        pub fn iterate(&mut self, visitor: *mut RootVisitor) {
            let blocks_len = self.blocks_.len();
            for i in 0..(blocks_len.saturating_sub(1)) {
                unsafe {
                    let block_start = self.blocks_[i];
                    let block_end = block_start.add(kHandleBlockSize);
                    (*visitor).VisitRootPointers(Root::kHandleScope, null_mut(),
                                        block_start as *mut Address,
                                        block_end as *mut Address);
                }
            }

            if !self.blocks_.is_empty() {
                unsafe {
                    let block_start = self.blocks_.last().unwrap();
                    (*visitor).VisitRootPointers(Root::kHandleScope, null_mut(),
                                        *block_start as *mut Address,
                                        self.block_next_ as *mut Address);
                }
            }
        }

        pub fn new_handle<T>(&mut self, obj: Tagged<T>) -> IndirectHandle<T> {
            #[cfg(debug_assertions)]
            self.check_owner_is_not_parked();
            let ptr = obj.ptr();
            let handle = self.get_handle(ptr as *mut Address);
            IndirectHandle {
                location_: handle as *mut Address,
                _phantom: std::marker::PhantomData,
            }
        }

         pub fn new_handle_indirect<T>(&mut self, obj: IndirectHandle<T>) -> IndirectHandle<T> {
             unsafe {
                 let value = *obj.location_;
                 let handle = self.get_handle(value as *mut Address);
                 IndirectHandle {
                     location_: handle as *mut Address,
                     _phantom: std::marker::PhantomData,
                 }
             }
         }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        #[cfg(debug_assertions)]
        pub fn contains(&self, location: *mut Address) -> bool {
            let it = self.ordered_blocks_.range(location..);
            if it.len() == 0 {
                return false;
            }
            let it = it.last().unwrap();

            if *it == self.blocks_.last().unwrap() {
                return location < self.block_next_;
            }
             unsafe {
                return location < it.add(kHandleBlockSize);
             }
        }

        fn add_block(&mut self) {
            assert_eq!(self.block_next_, self.block_limit_);

            let block_start = NewArray::<*mut Address>(kHandleBlockSize);
            self.blocks_.push(block_start);

            self.block_next_ = block_start;
            unsafe {
                self.block_limit_ = block_start.add(kHandleBlockSize);
            }

            #[cfg(debug_assertions)]
            self.ordered_blocks_.insert(block_start);
        }

        fn get_handle(&mut self, value: *mut Address) -> *mut Address {
            if self.block_next_ == self.block_limit_ {
                self.add_block();
            }
            unsafe {
                assert!(self.block_next_ < self.block_limit_);
                *self.block_next_ = value;
                let result = self.block_next_;
                self.block_next_ = self.block_next_.add(1);
                result
            }
        }

        #[cfg(debug_assertions)]
        pub fn attach(&mut self, local_heap: *mut LocalHeap) {
            assert!(self.owner_.is_null());
            self.owner_ = local_heap;
        }

        #[cfg(debug_assertions)]
        pub fn detach(&mut self) {
            assert!(!self.owner_.is_null());
            self.owner_ = null_mut();
        }

        #[cfg(debug_assertions)]
        pub fn check_owner_is_not_parked(&mut self) {
            if !self.owner_.is_null() {
                unsafe {
                  //  assert!(!(*self.owner_).IsParked());
                }
            }
        }
    }

    impl Drop for PersistentHandles {
        fn drop(&mut self) {
            unsafe {
                (*(*self.isolate_).persistent_handles_list_).remove(self);
                for block_start in &self.blocks_ {
                   std::ptr::drop_in_place(*block_start);
                }
            }
        }
    }

    pub struct PersistentHandlesList {
        persistent_handles_mutex_: Mutex<()>,
        persistent_handles_head_: *mut PersistentHandles,
    }

    impl PersistentHandlesList {
        pub fn new() -> PersistentHandlesList {
            PersistentHandlesList {
                persistent_handles_mutex_: Mutex::new(()),
                persistent_handles_head_: null_mut(),
            }
        }

        fn add(&mut self, persistent_handles: *mut PersistentHandles) {
            let _guard = self.persistent_handles_mutex_.lock().unwrap();
            unsafe {
                if !self.persistent_handles_head_.is_null() {
                    (*self.persistent_handles_head_).prev_ = persistent_handles;
                }
                (*persistent_handles).prev_ = null_mut();
                (*persistent_handles).next_ = self.persistent_handles_head_;
                self.persistent_handles_head_ = persistent_handles;
            }
        }

        fn remove(&mut self, persistent_handles: *mut PersistentHandles) {
            let _guard = self.persistent_handles_mutex_.lock().unwrap();
            unsafe {
                if !(*persistent_handles).next_.is_null() {
                    (*(*persistent_handles).next_).prev_ = (*persistent_handles).prev_;
                }
                if !(*persistent_handles).prev_.is_null() {
                    (*(*persistent_handles).prev_).next_ = (*persistent_handles).next_;
                } else {
                    self.persistent_handles_head_ = (*persistent_handles).next_;
                }
            }
        }

        pub fn iterate(&mut self, visitor: *mut RootVisitor, isolate: *mut Isolate) {
            let _guard = self.persistent_handles_mutex_.lock().unwrap();
            unsafe {
                let mut current = self.persistent_handles_head_;
                while !current.is_null() {
                    (*current).iterate(visitor);
                    current = (*current).next_;
                }
            }
        }
    }

    pub struct PersistentHandlesScope {
        first_block_: *mut Address,
        prev_limit_: *mut Address,
        prev_next_: *mut Address,
        impl_: *mut HandleScopeImplementer,
        #[cfg(debug_assertions)]
        handles_detached_: bool,
        #[cfg(debug_assertions)]
        prev_level_: i32,
    }

    impl PersistentHandlesScope {
        pub fn new(isolate: *mut Isolate) -> PersistentHandlesScope {
            unsafe {
                let impl_ = (*isolate).handle_scope_implementer();
                (*impl_).BeginPersistentScope();
                let data = (*isolate).handle_scope_data();
                let new_next = (*impl_).GetSpareOrNewBlock();
                let new_limit = new_next.add(kHandleBlockSize);
                (*(*impl_).blocks_).push(new_next);

                #[cfg(debug_assertions)]
                let prev_level_ = (*data).level;

                (*data).level += 1;
                let first_block_ = new_next;
                let prev_limit_ = (*data).limit;
                let prev_next_ = (*data).next;
                (*data).next = new_next;
                (*data).limit = new_limit;

                PersistentHandlesScope {
                    first_block_: first_block_,
                    prev_limit_: prev_limit_,
                    prev_next_: prev_next_,
                    impl_: impl_,
                    #[cfg(debug_assertions)]
                    handles_detached_: false,
                    #[cfg(debug_assertions)]
                    prev_level_: prev_level_,
                }
            }
        }

        pub fn detach(&mut self) -> Box<PersistentHandles> {
             unsafe {
                let ph = (*self.impl_).DetachPersistent(self.first_block_);
                let data = (*(*self.impl_).isolate_).handle_scope_data();
                (*data).next = self.prev_next_;
                (*data).limit = self.prev_limit_;
                #[cfg(debug_assertions)]
                {
                    self.handles_detached_ = true;
                }
                Box::from_raw(ph)
             }
        }

        pub fn is_active(isolate: *mut Isolate) -> bool {
            unsafe {
                (*isolate).handle_scope_implementer().HasPersistentScope()
            }
        }
    }

    impl Drop for PersistentHandlesScope {
        fn drop(&mut self) {
            unsafe {
                assert!(self.handles_detached_);
                (*(*self.impl_).isolate_).handle_scope_data().level -= 1;
                #[cfg(debug_assertions)]
                assert_eq!((*(*self.impl_).isolate_).handle_scope_data().level, self.prev_level_);
            }
        }
    }

    pub type Address = usize;
}

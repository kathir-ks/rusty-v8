// Converted from V8 C++ source files:
// Header: local-handles.h
// Implementation: local-handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod local_handles {
    use crate::handles::handles::*;
    use crate::heap::local_heap::LocalHeap;
    use crate::V8::internal::Address;
    use crate::V8::internal::RootVisitor;
    use crate::execution::isolate::HandleScopeData;
    use std::mem::MaybeUninit;
    use std::ptr;

    const K_HANDLE_BLOCK_SIZE: usize = 64; // Example size, adjust as needed

    pub struct LocalHandles {
        scope_: HandleScopeData,
        blocks_: Vec<*mut Address>,
    }

    impl LocalHandles {
        pub fn new() -> Self {
            let mut scope_ = HandleScopeData::new();
            scope_.Initialize();
            LocalHandles {
                scope_: scope_,
                blocks_: Vec::new(),
            }
        }

        pub fn iterate(&mut self, visitor: &mut RootVisitor) {
            let blocks_len = self.blocks_.len();
            for i in 0..(blocks_len.saturating_sub(1)) {
                let block = self.blocks_[i];
                visitor.VisitRootPointers(
                    Root::kHandleScope,
                    ptr::null_mut(),
                    FullObjectSlot(block),
                    FullObjectSlot(unsafe { block.add(K_HANDLE_BLOCK_SIZE) }),
                );
            }

            if !self.blocks_.is_empty() {
                let block = self.blocks_.last().unwrap();
                visitor.VisitRootPointers(
                    Root::kHandleScope,
                    ptr::null_mut(),
                    FullObjectSlot(*block),
                    FullObjectSlot(self.scope_.next),
                );
            }
        }

        #[cfg(debug_assertions)]
        pub fn contains(&self, location: *mut Address) -> bool {
            for block in &self.blocks_ {
                let lower_bound = *block;
                let upper_bound = if *block != *self.blocks_.last().unwrap() {
                    unsafe { lower_bound.add(K_HANDLE_BLOCK_SIZE) }
                } else {
                    self.scope_.next
                };

                if lower_bound <= location && location < upper_bound {
                    return true;
                }
            }
            return false;
        }

        fn add_block(&mut self) -> *mut Address {
            assert_eq!(self.scope_.next, self.scope_.limit);

            let layout = std::alloc::Layout::array::<Address>(K_HANDLE_BLOCK_SIZE)
                .expect("Could not create layout");

            let block = unsafe { std::alloc::alloc(layout) } as *mut Address;

            if block.is_null() {
                panic!("Memory allocation failed in AddBlock");
            }

            self.blocks_.push(block);
            self.scope_.next = block;
            self.scope_.limit = unsafe { block.add(K_HANDLE_BLOCK_SIZE) };
            block
        }

        fn remove_unused_blocks(&mut self) {
            while !self.blocks_.is_empty() {
                let block_start = self.blocks_.last().unwrap();
                let block_limit = unsafe { block_start.add(K_HANDLE_BLOCK_SIZE) };

                if block_limit == self.scope_.limit {
                    break;
                }

                let block_start_copy = *block_start; // Copy the pointer before popping

                self.blocks_.pop();
                self.zap_range(block_start_copy, block_limit);

                let layout = std::alloc::Layout::array::<Address>(K_HANDLE_BLOCK_SIZE)
                    .expect("Could not create layout");
                unsafe {
                    std::alloc::dealloc(block_start_copy as *mut u8, layout);
                }
            }
        }

        fn zap_range(&self, start: *mut Address, end: *mut Address) {
            HandleScope::ZapRange(start, end);
        }
    }

    impl Drop for LocalHandles {
        fn drop(&mut self) {
            self.scope_.limit = ptr::null_mut();
            self.remove_unused_blocks();
            assert!(self.blocks_.is_empty());
        }
    }

    pub struct LocalHandleScope<'a> {
        local_heap_: *mut LocalHeap,
        prev_limit_: *mut Address,
        prev_next_: *mut Address,

        #[cfg(debug_assertions)]
        scope_level_: i32,
        _marker: std::marker::PhantomData<&'a mut LocalHeap>,
    }

    impl<'a> LocalHandleScope<'a> {
        pub fn new(local_isolate: *mut LocalHeap) -> Self {
            unsafe {
                let isolate = (*(*local_isolate).heap()).isolate();
                let data = isolate.handle_scope_data();
                let local_heap_ = local_isolate;
                let prev_next_ = data.next;
                let prev_limit_ = data.limit;
                data.level += 1;

                LocalHandleScope {
                    local_heap_: local_heap_,
                    prev_limit_: prev_limit_,
                    prev_next_: prev_next_,
                    #[cfg(debug_assertions)]
                    scope_level_: data.level,
                    _marker: std::marker::PhantomData,
                }
            }
        }
        pub fn new_from_local_heap(local_heap: *mut LocalHeap) -> Self {
          Self::new(local_heap)
      }


        pub fn close_and_escape<T, HandleType: std::convert::AsRef<DirectHandle>>(
            self,
            handle_value: HandleType,
        ) -> HandleType {
            handle_value
        }

        pub fn get_handle(local_heap: *mut LocalHeap, value: *mut Address) -> *mut Address {
            unsafe { Self::get_main_thread_handle(local_heap, value) }
        }

        fn close_scope(local_heap: *mut LocalHeap, prev_next: *mut Address, prev_limit: *mut Address) {
            unsafe {
                let isolate = (*(*local_heap).heap()).isolate();
                HandleScope::CloseScope(isolate, prev_next, prev_limit);
            }
        }

        fn close_main_thread_scope(local_heap: *mut LocalHeap, prev_next: *mut Address, prev_limit: *mut Address) {
            unsafe {
                let isolate = (*(*local_heap).heap()).isolate();
                HandleScope::CloseScope(isolate, prev_next, prev_limit);
            }
        }

        fn open_main_thread_scope(local_heap: *mut LocalHeap) {
            unsafe {
                let isolate = (*(*local_heap).heap()).isolate();
                let data = isolate.handle_scope_data();

                // self.local_heap_ = local_heap;
                // self.prev_next_ = data.next;
                // self.prev_limit_ = data.limit;
                data.level += 1;
            }
        }

        fn get_main_thread_handle(local_heap: *mut LocalHeap, value: *mut Address) -> *mut Address {
            unsafe {
                let isolate = (*(*local_heap).heap()).isolate();
                HandleScope::CreateHandle(isolate, value as *mut Address)
            }
        }

        #[cfg(debug_assertions)]
        fn verify_main_thread_scope(&self) const {
            unsafe {
                let isolate = (*(*self.local_heap_).heap()).isolate();
                assert_eq!(self.scope_level_, isolate.handle_scope_data().level);
            }
        }
    }

    impl<'a> Drop for LocalHandleScope<'a> {
        fn drop(&mut self) {
            unsafe {
                let isolate = (*(*self.local_heap_).heap()).isolate();
                let data = isolate.handle_scope_data();
                data.level -= 1;

                HandleScope::CloseScope(isolate, self.prev_next_, self.prev_limit_);
            }
        }
    }
}

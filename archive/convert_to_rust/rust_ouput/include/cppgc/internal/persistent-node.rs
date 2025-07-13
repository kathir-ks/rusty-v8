// Converted from V8 C++ source files:
// Header: persistent-node.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::array;
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Mutex, MutexGuard};

    pub struct PersistentNode {
        owner_: *mut std::ffi::c_void,
        next_: *mut PersistentNode,
        trace_: Option<TraceRootCallback>,
    }

    impl PersistentNode {
        pub fn new() -> PersistentNode {
            PersistentNode {
                owner_: null_mut(),
                next_: null_mut(),
                trace_: None,
            }
        }

        pub fn initialize_as_used_node(&mut self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) {
            self.owner_ = owner;
            self.trace_ = Some(trace);
        }

        pub fn initialize_as_free_node(&mut self, next: *mut PersistentNode) {
            self.next_ = next;
            self.trace_ = None;
        }

        pub fn update_owner(&mut self, owner: *mut std::ffi::c_void) {
            if self.is_used() {
                self.owner_ = owner;
            }
        }

        pub fn free_list_next(&self) -> *mut PersistentNode {
            if !self.is_used() {
                self.next_
            } else {
                null_mut()
            }
        }

        pub fn trace(&self, root_visitor: &mut RootVisitor) {
            if self.is_used() {
                if let Some(trace) = self.trace_ {
                    trace(root_visitor, self.owner_);
                }
            }
        }

        pub fn is_used(&self) -> bool {
            self.trace_.is_some()
        }

        pub fn owner(&self) -> *mut std::ffi::c_void {
            if self.is_used() {
                self.owner_
            } else {
                null_mut()
            }
        }
    }

    pub type TraceRootCallback = fn(&mut RootVisitor, *mut std::ffi::c_void);

    pub struct RootVisitor {}

    pub struct FatalOutOfMemoryHandler {}

    impl FatalOutOfMemoryHandler {
        pub fn new() -> Self {
            FatalOutOfMemoryHandler {}
        }
    }

    pub struct PersistentRegionBase {
        nodes_: Vec<Box<PersistentNodeSlots>>,
        free_list_head_: *mut PersistentNode,
        nodes_in_use_: usize,
        oom_handler_: FatalOutOfMemoryHandler,
    }

    type PersistentNodeSlots = array::IntoIter<PersistentNode, 256>;

    impl PersistentRegionBase {
        pub fn new(oom_handler: &FatalOutOfMemoryHandler) -> PersistentRegionBase {
            PersistentRegionBase {
                nodes_: Vec::new(),
                free_list_head_: null_mut(),
                nodes_in_use_: 0,
                oom_handler_: FatalOutOfMemoryHandler {},
            }
        }

        pub fn iterate(&mut self, _root_visitor: &mut RootVisitor) {}

        pub fn nodes_in_use(&self) -> usize {
            self.nodes_in_use_
        }

        pub fn clear_all_used_nodes(&mut self) {}

        fn try_allocate_node_from_free_list(
            &mut self,
            owner: *mut std::ffi::c_void,
            trace: TraceRootCallback,
        ) -> *mut PersistentNode {
            if self.free_list_head_.is_null() {
                return null_mut();
            }
            let mut node = unsafe { &mut *self.free_list_head_ };
            self.free_list_head_ = node.free_list_next();
            node.initialize_as_used_node(owner, trace);
            self.nodes_in_use_ += 1;
            node
        }

        fn free_node(&mut self, node: *mut PersistentNode) {
            if node.is_null() {
                return;
            }
            let mut node = unsafe { &mut *node };
            node.initialize_as_free_node(self.free_list_head_);
            self.free_list_head_ = node;
            if self.nodes_in_use_ > 0 {
                self.nodes_in_use_ -= 1;
            }
        }

        fn refill_free_list_and_allocate_node(
            &mut self,
            owner: *mut std::ffi::c_void,
            trace: TraceRootCallback,
        ) -> *mut PersistentNode {
            self.refill_free_list();
            self.try_allocate_node_from_free_list(owner, trace)
        }

        fn refill_free_list(&mut self) {
            let mut new_nodes = Box::new(PersistentNodeSlots::new([PersistentNode::new(); 256]));
            let mut current = new_nodes.as_mut_ptr();
            for i in 0..255 {
                let mut node = unsafe { &mut *current.add(i) };
                node.initialize_as_free_node(unsafe { current.add(i + 1) });
            }
            let mut last_node = unsafe { &mut *current.add(255) };
            last_node.initialize_as_free_node(null_mut());

            let head = unsafe { &mut *current };
            head.next_ = self.free_list_head_;
            self.free_list_head_ = current;
            self.nodes_.push(new_nodes);
        }
    }

    impl Drop for PersistentRegionBase {
        fn drop(&mut self) {
            self.clear_all_used_nodes();
        }
    }

    pub struct HeapBase {}

    impl HeapBase {
        pub fn new() -> HeapBase {
            HeapBase {}
        }
    }

    pub struct PersistentRegion {
        base: PersistentRegionBase,
        heap_: HeapBase,
    }

    impl PersistentRegion {
        pub fn new(heap: &HeapBase, oom_handler: &FatalOutOfMemoryHandler) -> PersistentRegion {
            PersistentRegion {
                base: PersistentRegionBase::new(oom_handler),
                heap_: HeapBase {},
            }
        }

        pub fn allocate_node(&mut self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) -> *mut PersistentNode {
            let node = self.base.try_allocate_node_from_free_list(owner, trace);
            if !node.is_null() {
                return node;
            }

            return self.base.refill_free_list_and_allocate_node(owner, trace);
        }

        pub fn free_node(&mut self, node: *mut PersistentNode) {
            self.base.free_node(node);
        }
    }

    pub struct PersistentRegionLock {}

    impl PersistentRegionLock {
        pub fn new() -> PersistentRegionLock {
            PersistentRegionLock {}
        }

        pub fn assert_locked() {}
    }

    pub struct CrossThreadPersistentRegion {
        base: PersistentRegionBase,
    }

    impl CrossThreadPersistentRegion {
        pub fn new(oom_handler: &FatalOutOfMemoryHandler) -> CrossThreadPersistentRegion {
            CrossThreadPersistentRegion {
                base: PersistentRegionBase::new(oom_handler),
            }
        }

        pub fn allocate_node(&mut self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) -> *mut PersistentNode {
            PersistentRegionLock::assert_locked();
            let node = self.base.try_allocate_node_from_free_list(owner, trace);
            if !node.is_null() {
                return node;
            }

            self.base.refill_free_list_and_allocate_node(owner, trace)
        }

        pub fn free_node(&mut self, node: *mut PersistentNode) {
            PersistentRegionLock::assert_locked();
            self.base.free_node(node);
        }

        pub fn iterate(&mut self, root_visitor: &mut RootVisitor) {
            self.base.iterate(root_visitor);
        }

        pub fn nodes_in_use(&self) -> usize {
            self.base.nodes_in_use()
        }

        pub fn clear_all_used_nodes(&mut self) {
            self.base.clear_all_used_nodes();
        }
    }

    impl Drop for CrossThreadPersistentRegion {
        fn drop(&mut self) {
            self.clear_all_used_nodes();
        }
    }
}

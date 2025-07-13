// Converted from V8 C++ source files:
// Header: N/A
// Implementation: persistent-node.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod persistent_node {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;
    use std::rc::Rc;
    use crate::heap::cppgc::heap_base::FatalOutOfMemoryHandler;
    use crate::heap::cppgc::platform::ProcessGlobalLock;

    pub trait RootVisitor {
        fn visit(&mut self, object: *mut void);
    }

    pub struct PersistentNode {
        owner_: *mut void,
        trace_: Option<TraceRootCallback>,
        next_free_: *mut PersistentNode,
        is_used_: bool,
    }

    impl PersistentNode {
        pub fn new() -> Self {
            PersistentNode {
                owner_: null_mut(),
                trace_: None,
                next_free_: null_mut(),
                is_used_: false,
            }
        }

        pub fn InitializeAsFreeNode(&mut self, free_list_head: *mut PersistentNode) {
            self.is_used_ = false;
            self.next_free_ = free_list_head;
        }

        pub fn IsUsed(&self) -> bool {
            self.is_used_
        }

        pub fn owner(&self) -> *mut void {
            self.owner_
        }

        pub fn set_owner(&mut self, owner: *mut void) {
            self.owner_ = owner;
        }

        pub fn set_trace(&mut self, trace: Option<TraceRootCallback>) {
            self.trace_ = trace;
        }

        pub fn Use(&mut self) {
            self.is_used_ = true;
        }

        pub fn Clear(&mut self) {
            self.is_used_ = false;
            self.owner_ = null_mut();
            self.trace_ = None;
        }

        pub fn FreeListNext(&self) -> *mut PersistentNode {
            self.next_free_
        }

        pub fn Trace(&self, root_visitor: &mut dyn RootVisitor) {
            if let Some(trace) = &self.trace_ {
                trace(root_visitor, self.owner_);
            }
        }
    }

    pub type TraceRootCallback = fn(&mut dyn RootVisitor, *mut void);

    pub struct PersistentNodeSlots {
        nodes: [PersistentNode; 16], // Or some other appropriate size
    }

    impl PersistentNodeSlots {
        pub fn new() -> Self {
            let mut nodes: [MaybeUninit<PersistentNode>; 16] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for node in &mut nodes {
                node.write(PersistentNode::new());
            }
            PersistentNodeSlots {
                nodes: unsafe { std::mem::transmute_copy(&nodes) },
            }
        }

        pub fn iter(&self) -> std::slice::Iter<'_, PersistentNode> {
            self.nodes.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, PersistentNode> {
            self.nodes.iter_mut()
        }

        pub fn front(&self) -> &PersistentNode {
            &self.nodes[0]
        }

        pub fn back(&self) -> &PersistentNode {
            &self.nodes[self.nodes.len() - 1]
        }
    }

    pub struct PersistentRegionBase {
        oom_handler_: FatalOutOfMemoryHandler,
        nodes_: Vec<Box<PersistentNodeSlots>>,
        free_list_head_: *mut PersistentNode,
        nodes_in_use_: usize,
    }

    impl PersistentRegionBase {
        pub fn new(oom_handler: FatalOutOfMemoryHandler) -> Self {
            PersistentRegionBase {
                oom_handler_: oom_handler,
                nodes_: Vec::new(),
                free_list_head_: null_mut(),
                nodes_in_use_: 0,
            }
        }

        fn ClearAllUsedNodes<PersistentBaseClass>(&mut self) {
            for slots in &mut self.nodes_ {
                for node in slots.iter_mut() {
                    if !node.IsUsed() {
                        continue;
                    }

                    // Assuming PersistentBaseClass has a ClearFromGC method.
                    // Calling it on the owner.
                    if let Some(owner) = (node.owner() as *mut PersistentBaseClass).as_mut() {
                        owner.ClearFromGC();
                    }

                    // Add nodes back to the free list.
                    node.InitializeAsFreeNode(self.free_list_head_);
                    self.free_list_head_ = node;
                    assert!(self.nodes_in_use_ > 0);
                    self.nodes_in_use_ -= 1;
                }
            }
            assert_eq!(0, self.nodes_in_use_);
        }

        pub fn ClearAllUsedNodesPersistentBase(&mut self) {
            self.ClearAllUsedNodes::<PersistentBase>();
        }
        pub fn ClearAllUsedNodesCrossThreadPersistentBase(&mut self) {
            self.ClearAllUsedNodes::<CrossThreadPersistentBase>();
        }

        pub fn NodesInUse(&self) -> usize {
            #[cfg(debug_assertions)]
            {
                let accumulated_nodes_in_use_: usize = self.nodes_.iter().map(|slots| {
                    slots.iter().filter(|node| node.IsUsed()).count()
                }).sum();
                assert_eq!(accumulated_nodes_in_use_, self.nodes_in_use_);
            }
            self.nodes_in_use_
        }

        pub fn RefillFreeList(&mut self) {
            let mut node_slots = Box::new(PersistentNodeSlots::new());
            if node_slots.nodes.len() == 0 {
                (self.oom_handler_)("Oilpan: PersistentRegionBase::RefillFreeList()");
                return;
            }
            let node_slots_ptr = node_slots.as_mut() as *mut PersistentNodeSlots;
            self.nodes_.push(node_slots);
            let node_slots_ref = unsafe { &mut *node_slots_ptr };

            for node in node_slots_ref.iter_mut() {
                node.InitializeAsFreeNode(self.free_list_head_);
                self.free_list_head_ = node;
            }
        }

        pub fn RefillFreeListAndAllocateNode(&mut self, owner: *mut void, trace: Option<TraceRootCallback>) -> *mut PersistentNode {
            self.RefillFreeList();
            let node = self.TryAllocateNodeFromFreeList(owner, trace);
            assert!(!node.is_null());
            node
        }

        pub fn Iterate(&mut self, root_visitor: &mut dyn RootVisitor) {
            self.free_list_head_ = null_mut();
            for slots in &mut self.nodes_ {
                let mut is_empty = true;
                for node in slots.iter_mut() {
                    if node.IsUsed() {
                        node.Trace(root_visitor);
                        is_empty = false;
                    } else {
                        node.InitializeAsFreeNode(self.free_list_head_);
                        self.free_list_head_ = node;
                    }
                }
                if is_empty {
                    let first_node = &mut slots.nodes[0];
                    let first_next = first_node.FreeListNext();

                    let slots_ptr = slots.as_mut() as *mut PersistentNodeSlots;

                    // Use a raw pointer to compare addresses.
                    let slots_start = unsafe { &(*slots_ptr).nodes[0] as *const PersistentNode as usize };
                    let slots_end = unsafe { &(*slots_ptr).nodes[slots.nodes.len() - 1] as *const PersistentNode as usize };

                    if !first_next.is_null() {
                        let first_next_ptr = first_next as usize;
                        assert!(first_next_ptr < slots_start || first_next_ptr > slots_end);
                    }

                    self.free_list_head_ = first_next;
                    // drop the slots
                    drop(slots);
                }
            }
            self.nodes_.retain(|ptr| {
                 if let Some(slots) = ptr.as_ref() {
                    for node in slots.iter() {
                        if node.IsUsed() {
                            return true;
                        }
                    }
                 }
                false
            });
        }

        fn TryAllocateNodeFromFreeList(&mut self, owner: *mut void, trace: Option<TraceRootCallback>) -> *mut PersistentNode {
            if self.free_list_head_.is_null() {
                return null_mut();
            }

            let node = self.free_list_head_;
            // Update free list head.
            self.free_list_head_ = unsafe { (*self.free_list_head_).FreeListNext() };

            // Initialize the node.
            unsafe {
                (*node).set_owner(owner);
                (*node).set_trace(trace);
                (*node).Use();
            }

            self.nodes_in_use_ += 1;
            node
        }

    }

    impl Drop for PersistentRegionBase {
        fn drop(&mut self) {
            self.ClearAllUsedNodesPersistentBase();
        }
    }

    pub struct PersistentRegion {
        heap_: Heap,
        base_: PersistentRegionBase,
    }

    impl PersistentRegion {
        pub fn new(heap: Heap, oom_handler: FatalOutOfMemoryHandler) -> Self {
            PersistentRegion {
                heap_: heap,
                base_: PersistentRegionBase::new(oom_handler),
            }
        }
        pub fn base(&mut self) -> &mut PersistentRegionBase{
            &mut self.base_
        }

        pub fn IsCreationThread(&self) -> bool {
            self.heap_.CurrentThreadIsHeapThread()
        }
    }

    pub struct PersistentRegionLock {}

    impl PersistentRegionLock {
        pub fn new() -> Self {
            ProcessGlobalLock::Lock::<
                ProcessGlobalLock::Reason::kForCrossThreadHandleCreation>();
            PersistentRegionLock {}
        }

        pub fn AssertLocked() {
            ProcessGlobalLock::AssertHeld();
        }
    }

    impl Drop for PersistentRegionLock {
        fn drop(&mut self) {
            ProcessGlobalLock::Unlock::<
                ProcessGlobalLock::Reason::kForCrossThreadHandleCreation>();
        }
    }
    pub struct Heap{}

    impl Heap{
        fn CurrentThreadIsHeapThread(&self)->bool{
            true
        }
    }
    pub struct CrossThreadPersistentRegion {
        base_: PersistentRegionBase,
    }

    impl CrossThreadPersistentRegion {
        pub fn new(oom_handler: FatalOutOfMemoryHandler) -> Self {
            CrossThreadPersistentRegion {
                base_: PersistentRegionBase::new(oom_handler),
            }
        }

        pub fn Iterate(&mut self, root_visitor: &mut dyn RootVisitor) {
            PersistentRegionLock::AssertLocked();
            self.base_.Iterate(root_visitor);
        }

        pub fn NodesInUse(&self) -> usize {
            // This method does not require a lock.
            self.base_.NodesInUse()
        }

        pub fn ClearAllUsedNodes(&mut self) {
            PersistentRegionLock::AssertLocked();
            self.base_.ClearAllUsedNodesCrossThreadPersistentBase();
        }

        pub fn base(&mut self) -> &mut PersistentRegionBase{
            &mut self.base_
        }
    }

    impl Drop for CrossThreadPersistentRegion {
        fn drop(&mut self) {
            let _guard = PersistentRegionLock::new();
            self.base_.ClearAllUsedNodesCrossThreadPersistentBase();
            self.base_.nodes_.clear();
        }
    }

    // Dummy structs for template instantiation
    pub struct CrossThreadPersistentBase {}
    impl CrossThreadPersistentBase{
        fn ClearFromGC(&mut self){}
    }
    pub struct PersistentBase {}
    impl PersistentBase{
        fn ClearFromGC(&mut self){}
    }
}

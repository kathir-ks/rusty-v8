// src/heap/cppgc/persistent_node.rs

// use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
// use cppgc::internal::persistent_node::PersistentNode;

// use crate::base::platform::platform; // Assuming a Rust equivalent exists
// use crate::heap::cppgc::heap_base::HeapBase; // Assuming a Rust equivalent exists
// use crate::heap::cppgc::platform::Platform; // Assuming a Rust equivalent exists
// use crate::heap::cppgc::process_heap::ProcessHeap; // Assuming a Rust equivalent exists

pub type FatalOutOfMemoryHandler = Box<dyn Fn(&str)>;
pub type TraceRootCallback = Box<dyn Fn()>;
pub type RootVisitor<'a> = &'a mut dyn FnMut(*mut u8);

macro_rules! CPPGC_DCHECK {
    ($condition:expr) => {
        if !($condition) {
            panic!("CPPGC_DCHECK failed: {}", stringify!($condition));
        }
    };
}

struct PersistentNode {
    owner: *mut u8, // *mut PersistentBase or *mut CrossThreadPersistentBase
    free_list_next: Cell<*mut PersistentNode>,
    is_used: Cell<bool>,
    trace: Cell<Option<TraceRootCallback>>,
}

impl PersistentNode {
    fn new() -> Self {
        PersistentNode {
            owner: null_mut(),
            free_list_next: Cell::new(null_mut()),
            is_used: Cell::new(false),
            trace: Cell::new(None),
        }
    }

    fn initialize_as_free_node(&self, free_list_head: *mut PersistentNode) {
        self.owner = null_mut();
        self.is_used.set(false);
        self.free_list_next.set(free_list_head);
        self.trace.set(None);
    }

    fn allocate(&self, owner: *mut u8, trace: Option<TraceRootCallback>) {
        self.owner = owner;
        self.is_used.set(true);
        self.trace.set(trace);
    }

    fn is_used(&self) -> bool {
        self.is_used.get()
    }

    fn free_list_next(&self) -> *mut PersistentNode {
        self.free_list_next.get()
    }

    fn trace(&self, root_visitor: &mut RootVisitor) {
        if let Some(trace) = self.trace.take() {
            (root_visitor)(self.owner);
            self.trace.set(Some(trace)); // Restore trace after calling visitor
        }
    }

    fn clear(&self) {
        self.owner = null_mut();
        self.is_used.set(false);
        self.trace.set(None);
    }
}

type PersistentNodeSlots = Vec<PersistentNode>;

struct PersistentRegionBase {
    oom_handler_: FatalOutOfMemoryHandler,
    nodes_: Vec<Box<PersistentNodeSlots>>,
    free_list_head_: *mut PersistentNode,
    nodes_in_use_: usize,
}

impl PersistentRegionBase {
    fn new(oom_handler: FatalOutOfMemoryHandler) -> Self {
        PersistentRegionBase {
            oom_handler_: oom_handler,
            nodes_: Vec::new(),
            free_list_head_: null_mut(),
            nodes_in_use_: 0,
        }
    }

    fn clear_all_used_nodes<T>(&mut self) {
        for slots in &mut self.nodes_ {
            for node in slots.iter_mut() {
                if !node.is_used() {
                    continue;
                }

                // Assuming T has a clear_from_gc method (e.g., PersistentBase/CrossThreadPersistentBase)
                unsafe {
                    let owner = node.owner as *mut T;
                    if !owner.is_null() {
                      //TODO: Add trait bound if possible
                      //(*owner).clear_from_gc();
                      node.clear();
                    }
                }

                // Add nodes back to the free list to allow reusing for subsequent creation calls.
                node.initialize_as_free_node(self.free_list_head_);
                self.free_list_head_ = node;
                CPPGC_DCHECK!(self.nodes_in_use_ > 0);
                self.nodes_in_use_ -= 1;
            }
        }
        CPPGC_DCHECK!(0 == self.nodes_in_use_);
    }

    fn nodes_in_use(&self) -> usize {
        #[cfg(debug_assertions)]
        {
            let accumulated_nodes_in_use_: usize = self.nodes_.iter().map(|slots| {
                slots.iter().filter(|node| node.is_used()).count()
            }).sum();
            assert_eq!(accumulated_nodes_in_use_, self.nodes_in_use_);
        }
        self.nodes_in_use_
    }

    fn refill_free_list(&mut self) {
        let mut node_slots = Box::new(vec![PersistentNode::new(); 32]); // Default size, can be adjusted
        if node_slots.is_empty() {
            (self.oom_handler_)("Oilpan: PersistentRegionBase::RefillFreeList()");
        }
        let node_slots_ptr: *mut PersistentNodeSlots = &mut *node_slots;
        self.nodes_.push(node_slots);
        // Safety: node_slots is valid for lifetime of PersistentRegionBase.
        for node in unsafe {&mut *node_slots_ptr}{
            node.initialize_as_free_node(self.free_list_head_);
            self.free_list_head_ = node;
        }
    }

    fn refill_free_list_and_allocate_node(&mut self, owner: *mut u8, trace: Option<TraceRootCallback>) -> *mut PersistentNode {
        self.refill_free_list();
        let node = self.try_allocate_node_from_free_list(owner, trace);
        CPPGC_DCHECK!(!node.is_null());
        node
    }

    fn try_allocate_node_from_free_list(&mut self, owner: *mut u8, trace: Option<TraceRootCallback>) -> *mut PersistentNode {
        unsafe {
            if self.free_list_head_.is_null() {
                return null_mut();
            }

            let node = self.free_list_head_;
            self.free_list_head_ = (*node).free_list_next.get();
            (*node).allocate(owner, trace);
            self.nodes_in_use_ += 1;
            node
        }
    }

    fn iterate(&mut self, root_visitor: &mut RootVisitor) {
        self.free_list_head_ = null_mut();
        for slots in &mut self.nodes_ {
            let mut is_empty = true;
            for node in slots.iter_mut() {
                if node.is_used() {
                    node.trace(root_visitor);
                    is_empty = false;
                } else {
                    node.initialize_as_free_node(self.free_list_head_);
                    self.free_list_head_ = node;
                }
            }
            if is_empty {
                unsafe {
                    let first_next = (*slots)[0].free_list_next();
                    CPPGC_DCHECK!(first_next.is_null() || first_next < &slots[0] || first_next > slots.last().unwrap());
                    self.free_list_head_ = first_next;
                    // Prevent double free by dropping this slot
                    //std::mem::drop(slots); //Is this the right action?
                }
            }
        }
        self.nodes_.retain(|ptr| !ptr.is_empty());
    }
}

impl Drop for PersistentRegionBase {
    fn drop(&mut self) {
        // Manually call clear_all_used_nodes to avoid double-borrow issues
        // with drop.
        //self.clear_all_used_nodes::<PersistentBase>();
        while let Some(mut slots) = self.nodes_.pop() {
            for node in slots.iter_mut() {
              if node.is_used() {
                node.clear();
              }
            }
        }
    }
}

struct PersistentRegion {
    heap_: Box<()>, //HeapBase, // Assuming HeapBase has a Rust equivalent and can be boxed
    base_: PersistentRegionBase,
}

impl PersistentRegion {
    fn new(oom_handler: FatalOutOfMemoryHandler) -> Self {
        PersistentRegion {
            heap_: Box::new(()), //HeapBase::new(), // Assuming HeapBase has a constructor
            base_: PersistentRegionBase::new(oom_handler),
        }
    }

    fn is_creation_thread(&self) -> bool {
        //self.heap_.current_thread_is_heap_thread() // Assuming HeapBase has this method
        true
    }
}

struct ProcessGlobalLock {}
impl ProcessGlobalLock {
    fn lock<const REASON: u32>() -> ProcessGlobalLockGuard {
        ProcessGlobalLockGuard::new()
    }
    fn unlock<const REASON: u32>() {}
    fn assert_held() {}
}

struct ProcessGlobalLockGuard {}
impl ProcessGlobalLockGuard {
    fn new() -> Self {
        ProcessGlobalLockGuard {}
    }
}

struct PersistentRegionLock {}

impl PersistentRegionLock {
    fn new() -> Self {
        ProcessGlobalLock::lock::<0>();
        PersistentRegionLock {}
    }

    fn assert_locked() {
        ProcessGlobalLock::assert_held();
    }
}

impl Drop for PersistentRegionLock {
    fn drop(&mut self) {
        ProcessGlobalLock::unlock::<0>();
    }
}

struct CrossThreadPersistentRegion {
    base_: PersistentRegionBase,
}

impl CrossThreadPersistentRegion {
    fn new(oom_handler: FatalOutOfMemoryHandler) -> Self {
        CrossThreadPersistentRegion {
            base_: PersistentRegionBase::new(oom_handler),
        }
    }

    fn iterate(&mut self, root_visitor: &mut RootVisitor) {
        PersistentRegionLock::assert_locked();
        self.base_.iterate(root_visitor);
    }

    fn nodes_in_use(&self) -> usize {
        // This method does not require a lock.
        self.base_.nodes_in_use()
    }

    fn clear_all_used_nodes<T>(&mut self) {
        PersistentRegionLock::assert_locked();
        self.base_.clear_all_used_nodes::<T>();
    }
}

impl Drop for CrossThreadPersistentRegion {
    fn drop(&mut self) {
        let _guard = PersistentRegionLock::new();
        self.base_.clear_all_used_nodes::<()>(); //TODO add type
    }
}
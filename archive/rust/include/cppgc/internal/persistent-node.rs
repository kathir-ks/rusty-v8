// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::array;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};

// Placeholder for cppgc::internal::logging.h
macro_rules! cppgc_dcheck {
    ($condition:expr) => {
        if !($condition) {
            panic!("CPPGC_DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! cppgc_check {
    ($condition:expr) => {
        if !($condition) {
            panic!("CPPGC_CHECK failed: {}", stringify!($condition));
        }
    };
}

// Placeholder for v8config.h
// Assuming V8_EXPORT is no-op
// Assuming V8_INLINE is inline
// Assuming V8_LIKELY is just the expression

// Placeholder for cppgc::TraceTrait
trait TraceTrait {
    fn trace(&self, root_visitor: &mut RootVisitor);
}

// Placeholder types
type TraceRootCallback = fn(&mut RootVisitor, *mut std::ffi::c_void);

trait HeapTrait {}

struct HeapBase {}
impl HeapTrait for HeapBase {}

struct RootVisitor {}

struct FatalOutOfMemoryHandler {}

/// Represents a variant of two states:
/// 1) traceable node with a back pointer to the Persistent object;
/// 2) freelist entry.
#[derive(Default)]
struct PersistentNode {
    owner_or_next: RefCell<OwnerOrNext>,
    trace: RefCell<Option<TraceRootCallback>>,
}

union OwnerOrNext {
    owner: *mut std::ffi::c_void,
    next: *mut PersistentNode,
}

impl PersistentNode {
    /// Initializes the node as a used node.
    fn initialize_as_used_node(&self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) {
        cppgc_dcheck!(owner as *const _ != std::ptr::null());
        cppgc_dcheck!(trace as *const _ != std::ptr::null());

        unsafe {
            self.owner_or_next.borrow_mut().replace(OwnerOrNext { owner });
        }
        self.trace.borrow_mut().replace(Some(trace));
    }

    /// Initializes the node as a free node.
    fn initialize_as_free_node(&self, next: *mut PersistentNode) {
        unsafe {
            self.owner_or_next.borrow_mut().replace(OwnerOrNext { next });
        }
        self.trace.borrow_mut().take();
    }

    /// Updates the owner of the node.
    fn update_owner(&self, owner: *mut std::ffi::c_void) {
        cppgc_dcheck!(self.is_used());
        unsafe {
            self.owner_or_next.borrow_mut().replace(OwnerOrNext { owner });
        }
    }

    /// Gets the next free node in the freelist.
    fn free_list_next(&self) -> *mut PersistentNode {
        cppgc_dcheck!(!self.is_used());
        unsafe { self.owner_or_next.borrow().as_ref().unwrap().next }
    }

    /// Traces the object.
    fn trace(&self, root_visitor: &mut RootVisitor) {
        cppgc_dcheck!(self.is_used());
        let trace = self.trace.borrow();
        let trace_fn = trace.unwrap();
        unsafe {
            trace_fn(
                root_visitor,
                self.owner_or_next.borrow().as_ref().unwrap().owner,
            );
        }
    }

    /// Returns true if the node is in use.
    fn is_used(&self) -> bool {
        self.trace.borrow().is_some()
    }

    /// Returns the owner of the node.
    fn owner(&self) -> *mut std::ffi::c_void {
        cppgc_dcheck!(self.is_used());
        unsafe { self.owner_or_next.borrow().as_ref().unwrap().owner }
    }
}

/// Base class for persistent regions.
struct PersistentRegionBase {
    nodes: RefCell<Vec<Box<[PersistentNode; 256]>>>,
    free_list_head: RefCell<*mut PersistentNode>,
    nodes_in_use: AtomicUsize,
    oom_handler: FatalOutOfMemoryHandler,
}

impl PersistentRegionBase {
    const NODE_SLOT_SIZE: usize = 256;

    /// Clears Persistent fields to avoid stale pointers after heap teardown.
    fn clear_all_used_nodes(&self) {
        // Placeholder implementation.  Needs template specialization to match C++.
        todo!()
    }

    /// Creates a new `PersistentRegionBase`.
    fn new(oom_handler: &FatalOutOfMemoryHandler) -> Self {
        PersistentRegionBase {
            nodes: RefCell::new(Vec::new()),
            free_list_head: RefCell::new(std::ptr::null_mut()),
            nodes_in_use: AtomicUsize::new(0),
            oom_handler: FatalOutOfMemoryHandler {},
        }
    }

    /// Iterates over all nodes.
    fn iterate(&self, root_visitor: &mut RootVisitor) {
        let nodes = self.nodes.borrow();
        for slot in nodes.iter() {
            for node in slot.iter() {
                if node.is_used() {
                    node.trace(root_visitor);
                }
            }
        }
    }

    /// Returns the number of nodes in use.
    fn nodes_in_use(&self) -> usize {
        self.nodes_in_use.load(Ordering::Relaxed)
    }

    /// Tries to allocate a node from the free list.
    fn try_allocate_node_from_free_list(
        &self,
        owner: *mut std::ffi::c_void,
        trace: TraceRootCallback,
    ) -> *mut PersistentNode {
        let mut free_list_head = self.free_list_head.borrow_mut();
        if !(*free_list_head).is_null() {
            let node = *free_list_head;
            unsafe {
                *free_list_head = (*node).free_list_next();
                cppgc_dcheck!(!(*node).is_used());
                (*node).initialize_as_used_node(owner, trace);
            }
            self.nodes_in_use.fetch_add(1, Ordering::Relaxed);
            node
        } else {
            std::ptr::null_mut()
        }
    }

    /// Frees a node.
    fn free_node(&self, node: *mut PersistentNode) {
        cppgc_dcheck!(!node.is_null());
        unsafe {
            cppgc_dcheck!((*node).is_used());
        }
        let mut free_list_head = self.free_list_head.borrow_mut();
        unsafe {
            (*node).initialize_as_free_node(*free_list_head);
        }
        *free_list_head = node;

        cppgc_dcheck!(self.nodes_in_use.load(Ordering::Relaxed) > 0);
        self.nodes_in_use.fetch_sub(1, Ordering::Relaxed);
    }

    /// Refills the free list and allocates a node.
    fn refill_free_list_and_allocate_node(
        &self,
        owner: *mut std::ffi::c_void,
        trace: TraceRootCallback,
    ) -> *mut PersistentNode {
        self.refill_free_list();
        let node = self.try_allocate_node_from_free_list(owner, trace);
        cppgc_dcheck!(!node.is_null());
        node
    }

    /// Refills the free list.
    fn refill_free_list(&self) {
        let mut new_slot = Box::new([PersistentNode::default(); 256]);
        let mut free_list_head = self.free_list_head.borrow_mut();
        for i in 0..255 {
            new_slot[i]
                .initialize_as_free_node(&mut new_slot[i + 1] as *mut PersistentNode);
        }
        new_slot[255].initialize_as_free_node(*free_list_head);
        *free_list_head = &mut new_slot[0] as *mut PersistentNode;
        self.nodes.borrow_mut().push(new_slot);
    }

    fn clear_all_used_nodes_generic<T>(&self) {
        // Placeholder implementation for the template method.
        // Requires knowing the "PersistentBaseClass" which is not present in the header file.
        todo!()
    }
}

impl Drop for PersistentRegionBase {
    fn drop(&mut self) {
        // Clears Persistent fields to avoid stale pointers after heap teardown.
        self.clear_all_used_nodes();
    }
}

/// Variant of PersistentRegionBase that checks whether the allocation and
/// freeing happens only on the thread that created the heap.
struct PersistentRegion<'a> {
    base: PersistentRegionBase,
    heap: &'a HeapBase,
}

impl<'a> PersistentRegion<'a> {
    /// Creates a new `PersistentRegion`.
    fn new(heap: &'a HeapBase, oom_handler: &FatalOutOfMemoryHandler) -> Self {
        cppgc_dcheck!(Self::is_creation_thread());
        PersistentRegion {
            base: PersistentRegionBase::new(oom_handler),
            heap,
        }
    }

    /// Allocates a node.
    #[inline]
    fn allocate_node(&self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) -> *mut PersistentNode {
        cppgc_dcheck!(Self::is_creation_thread());
        let node = self.base.try_allocate_node_from_free_list(owner, trace);
        if !node.is_null() {
            return node;
        }

        // Slow path allocation allows for checking thread correspondence.
        cppgc_check!(Self::is_creation_thread());
        self.base.refill_free_list_and_allocate_node(owner, trace)
    }

    /// Frees a node.
    #[inline]
    fn free_node(&self, node: *mut PersistentNode) {
        cppgc_dcheck!(Self::is_creation_thread());
        self.base.free_node(node);
    }

    /// Checks if the current thread is the creation thread.
    fn is_creation_thread() -> bool {
        // Placeholder implementation.
        true
    }
}

/// Lock for cross-thread persistent regions.
struct PersistentRegionLock {
    mutex: Mutex<()>,
}

thread_local! {
    static IS_LOCKED: AtomicBool = AtomicBool::new(false);
}

impl PersistentRegionLock {
    /// Creates a new `PersistentRegionLock`.
    fn new() -> Self {
        PersistentRegionLock {
            mutex: Mutex::new(()),
        }
    }

    /// Asserts that the lock is locked.
    fn assert_locked() {
        IS_LOCKED.with(|is_locked| {
            cppgc_dcheck!(is_locked.load(Ordering::Relaxed));
        });
    }

    /// Locks the region.
    fn lock(&self) -> PersistentRegionLockGuard {
        let guard = self.mutex.lock().unwrap();
        IS_LOCKED.with(|is_locked| {
            is_locked.store(true, Ordering::Relaxed);
        });
        PersistentRegionLockGuard { _guard: guard }
    }
}

/// RAII guard for the persistent region lock.
struct PersistentRegionLockGuard<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl<'a> Drop for PersistentRegionLockGuard<'a> {
    fn drop(&mut self) {
        IS_LOCKED.with(|is_locked| {
            is_locked.store(false, Ordering::Relaxed);
        });
    }
}

/// Variant of PersistentRegionBase that checks whether the PersistentRegionLock
/// is locked.
struct CrossThreadPersistentRegion {
    base: PersistentRegionBase,
}

impl CrossThreadPersistentRegion {
    /// Creates a new `CrossThreadPersistentRegion`.
    fn new(oom_handler: &FatalOutOfMemoryHandler) -> Self {
        CrossThreadPersistentRegion {
            base: PersistentRegionBase::new(oom_handler),
        }
    }

    /// Allocates a node.
    #[inline]
    fn allocate_node(&self, owner: *mut std::ffi::c_void, trace: TraceRootCallback) -> *mut PersistentNode {
        PersistentRegionLock::assert_locked();
        let node = self.base.try_allocate_node_from_free_list(owner, trace);
        if !node.is_null() {
            return node;
        }

        self.base.refill_free_list_and_allocate_node(owner, trace)
    }

    /// Frees a node.
    #[inline]
    fn free_node(&self, node: *mut PersistentNode) {
        PersistentRegionLock::assert_locked();
        self.base.free_node(node);
    }

    /// Iterates over all nodes.
    fn iterate(&self, root_visitor: &mut RootVisitor) {
        self.base.iterate(root_visitor);
    }

    /// Returns the number of nodes in use.
    fn nodes_in_use(&self) -> usize {
        self.base.nodes_in_use()
    }

    /// Clears all used nodes.
    fn clear_all_used_nodes(&self) {
        self.base.clear_all_used_nodes();
    }
}
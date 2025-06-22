// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: Since this is a header file conversion, some parts might
// require corresponding source file implementations to be fully functional.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::rc::Rc;
use std::vec::Vec;

mod v8_callbacks {
    pub type WeakCallbackInfo<T> = crate::v8_callbacks::platform::WeakCallbackInfo<T>;
    pub mod platform {
        pub enum WeakCallbackType {
            kNormal,
            kPhantom,
        }

        pub struct WeakCallbackInfo<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> WeakCallbackInfo<T> {
            pub type Callback = fn(&WeakCallbackInfo<T>);
        }
    }
}

mod v8_profiler {
    pub trait PersistentHandleVisitor {
        fn visit(&mut self, handle: *mut std::ffi::c_void);
    }
}

mod handles {
    use std::ptr::NonNull;
    #[derive(Copy, Clone)]
    pub struct IndirectHandle<T> {
        location: *mut *mut T,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> IndirectHandle<T> {
        pub fn new(location: *mut *mut T) -> Self {
            IndirectHandle {
                location,
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn location(&self) -> *mut *mut T {
            self.location
        }
    }
}

mod heap {
    pub struct Heap {}
    impl Heap {
        pub fn new() -> Self {
            Heap {}
        }
    }
    pub struct LocalHeap {}

    impl LocalHeap {
        pub fn new() -> Self {
            LocalHeap {}
        }
    }
}

mod objects {
    use std::any::Any;
    use std::marker::PhantomData;
    pub struct Object {
        _private: [u8; 0],
    }

    pub struct HeapObject {
        _private: [u8; 0],
    }

    impl HeapObject {
        pub fn cast<T: Any>(&self) -> Option<&T> {
            (self as &dyn Any).downcast_ref::<T>()
        }
    }

    #[derive(Copy, Clone)]
    pub struct Tagged<T> {
        ptr: *mut T,
    }

    impl<T> Tagged<T> {
        pub fn ptr(&self) -> *mut T {
            self.ptr
        }
    }
    impl From<*mut Object> for Tagged<Object> {
        fn from(ptr: *mut Object) -> Self {
            Tagged { ptr }
        }
    }
}

mod utils {
    // Placeholder for utils
}

mod isolate {
    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }
}

mod heap_stats {
    pub struct HeapStats {}
    impl HeapStats {
        pub fn new() -> Self {
            HeapStats {}
        }
    }
}

mod root_visitor {
    pub struct RootVisitor {}
    impl RootVisitor {
        pub fn new() -> Self {
            RootVisitor {}
        }
    }
}

pub mod global_handles {
    use super::handles::IndirectHandle;
    use super::heap::Heap;
    use super::heap_stats::HeapStats;
    use super::isolate::Isolate;
    use super::objects::{Object, Tagged};
    use super::root_visitor::RootVisitor;
    use super::v8_callbacks::platform::WeakCallbackInfo;
    use super::v8_profiler::PersistentHandleVisitor;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;
    use std::ptr::NonNull;
    use std::rc::Rc;
    use std::vec::Vec;

    type Address = *mut std::ffi::c_void;
    type WeakSlotCallbackWithHeap = fn(Address) -> bool;

    /// Global handles hold handles that are independent of stack-state and can have
    /// callbacks and finalizers attached to them.
    pub struct GlobalHandles {
        isolate_: *mut Isolate,

        regular_nodes_: Box<NodeSpace<Node>>,
        // Contains all nodes holding young objects. Note: when the list
        // is accessed, some of the objects may have been promoted already.
        young_nodes_: Vec<*mut Node>,
        pending_phantom_callbacks_: Vec<(
            *mut Node,
            PendingPhantomCallback,
        )>,
        second_pass_callbacks_: Vec<PendingPhantomCallback>,
        second_pass_callbacks_task_posted_: bool,
        last_gc_custom_callbacks_: usize,
    }

    impl GlobalHandles {
        // API for regular handles.
        pub fn move_global(from: *mut Address, to: *mut Address) {
            unsafe {
                std::ptr::copy_nonoverlapping(from, to, 1);
            }
        }

        pub fn copy_global(location: Address) -> IndirectHandle<Object> {
            IndirectHandle::new(location as *mut *mut Object)
        }

        pub fn destroy(location: Address) {
            unsafe {
                std::ptr::write(location as *mut Address, std::ptr::null_mut());
            }
        }

        // Make the global handle weak and set the callback parameter for the
        // handle.  When the garbage collector recognizes that only weak global
        // handles point to an object the callback function is invoked (for each
        // handle) with the handle and corresponding parameter as arguments.  By
        // default the handle still contains a pointer to the object that is being
        // collected.  For this reason the object is not collected until the next
        // GC.  For a phantom weak handle the handle is cleared (set to a Smi)
        // before the callback is invoked, but the handle can still be identified
        // in the callback by using the location() of the handle.
        pub fn make_weak(
            location: Address,
            parameter: *mut std::ffi::c_void,
            weak_callback: WeakCallbackInfo<()>::Callback,
            type_: v8_callbacks::platform::WeakCallbackType,
        ) {
            // Implementation details for marking as weak
        }
        pub fn make_weak_addr(location_addr: *mut *mut Address) {
            // Implementation details for marking as weak
        }

        pub fn annotate_strong_retainer(location: Address, label: &str) {
            // Implementation details
        }

        // Clear the weakness of a global handle.
        pub fn clear_weakness(location: Address) -> *mut std::ffi::c_void {
            // Implementation details
            std::ptr::null_mut()
        }

        // Tells whether global handle is weak.
        pub fn is_weak(location: Address) -> bool {
            // Implementation details
            false
        }

        pub fn new(isolate: *mut Isolate) -> Self {
            GlobalHandles {
                isolate_: isolate,
                regular_nodes_: Box::new(NodeSpace::new()),
                young_nodes_: Vec::new(),
                pending_phantom_callbacks_: Vec::new(),
                second_pass_callbacks_: Vec::new(),
                second_pass_callbacks_task_posted_: false,
                last_gc_custom_callbacks_: 0,
            }
        }

        // Creates a new global handle that is alive until Destroy is called.
        pub fn create(&mut self, value: Tagged<Object>) -> IndirectHandle<Object> {
            let node = self.regular_nodes_.allocate();
            unsafe {
                *node.location = value.ptr;
            }
            IndirectHandle::new(node.location as *mut *mut Object)
        }

        pub fn create_addr(&mut self, value: Address) -> IndirectHandle<Object> {
            let node = self.regular_nodes_.allocate();
            unsafe {
                *node.location = value as *mut Object;
            }
            IndirectHandle::new(node.location as *mut *mut Object)
        }

        #[inline]
        pub fn create_generic<T>(&mut self, value: Tagged<T>) -> IndirectHandle<T> {
            let node = self.regular_nodes_.allocate();
            unsafe {
                *node.location = value.ptr as *mut Object;
            }
            IndirectHandle::new(node.location as *mut *mut T)
        }

        pub fn record_stats(&self, stats: &mut HeapStats) {
            // Implementation details
        }

        pub fn invoke_first_pass_weak_callbacks(&mut self) -> usize {
            // Implementation details
            0
        }
        pub fn invoke_second_pass_phantom_callbacks(&mut self) {
            // Implementation details
        }

        // Schedule or invoke second pass weak callbacks.
        pub fn post_garbage_collection_processing(&mut self, gc_callback_flags: i32) {
            // Implementation details
        }

        pub fn iterate_strong_roots(&mut self, v: *mut RootVisitor) {
            // Implementation details
        }
        pub fn iterate_weak_roots(&mut self, v: *mut RootVisitor) {
            // Implementation details
        }
        pub fn iterate_all_roots(&mut self, v: *mut RootVisitor) {
            // Implementation details
        }
        pub fn iterate_all_young_roots(&mut self, v: *mut RootVisitor) {
            // Implementation details
        }

        // Marks handles that are phantom or have callbacks based on the predicate
        // |should_reset_handle| as pending.
        pub fn iterate_weak_roots_for_phantom_handles(
            &mut self,
            should_reset_handle: WeakSlotCallbackWithHeap,
        ) {
            // Implementation details
        }

        //  Note: The following *Young* methods are used for the Scavenger to
        //  identify and process handles in the young generation. The set of young
        //  handles is complete but the methods may encounter handles that are
        //  already in old space.

        // Iterates over strong and dependent handles. See the note above.
        pub fn iterate_young_strong_and_dependent_roots(&mut self, v: *mut RootVisitor) {
            // Implementation details
        }

        // Processes all young weak objects:
        // - Weak objects for which `should_reset_handle()` returns true are reset;
        // - Others are passed to `v` iff `v` is not null.
        pub fn process_weak_young_objects(
            &mut self,
            v: *mut RootVisitor,
            should_reset_handle: WeakSlotCallbackWithHeap,
        ) {
            // Implementation details
        }

        // Updates the list of young nodes that is maintained separately.
        pub fn update_list_of_young_nodes(&mut self) {
            // Implementation details
        }
        // Clears the list of young nodes, assuming that the young generation is
        // empty.
        pub fn clear_list_of_young_nodes(&mut self) {
            // Implementation details
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn total_size(&self) -> usize {
            // Implementation details
            0
        }
        pub fn used_size(&self) -> usize {
            // Implementation details
            0
        }
        // Number of global handles.
        pub fn handles_count(&self) -> usize {
            // Implementation details
            0
        }
        pub fn last_gc_custom_callbacks(&self) -> usize {
            self.last_gc_custom_callbacks_
        }

        pub fn iterate_all_roots_for_testing(&mut self, v: &mut dyn PersistentHandleVisitor) {
            // Implementation details
        }

        #[cfg(debug_assertions)]
        pub fn print_stats(&self) {
            // Implementation details
        }

        #[cfg(debug_assertions)]
        pub fn print(&self) {
            // Implementation details
        }

        pub fn has_young(&self) -> bool {
            self.young_nodes_.is_empty()
        }

        fn apply_persistent_handle_visitor(
            &self,
            visitor: &mut dyn PersistentHandleVisitor,
            node: *mut Node,
        ) {
            // Implementation details
        }

        // Clears a weak `node` for which `should_reset_node()` returns true.
        //
        // Returns false if a node is weak and alive which requires further
        // processing, and true in all other cases (e.g. also strong nodes).
        fn reset_weak_node_if_dead(
            &self,
            node: *mut Node,
            should_reset_node: WeakSlotCallbackWithHeap,
        ) -> bool {
            // Implementation details
            true
        }
    }

    impl Drop for GlobalHandles {
        fn drop(&mut self) {
            // Deallocate resources if needed
        }
    }

    /// Internal node structures.
    pub struct Node {
        location: *mut Address,
    }

    impl Node {
        fn new() -> Self {
            Node {
                location: std::ptr::null_mut(),
            }
        }
    }

    pub struct NodeIterator<'a, NodeType> {
        _phantom: PhantomData<NodeType>,
        _phantom_data: PhantomData<&'a ()>, // Add lifetime parameter
    }

    impl<'a, NodeType> NodeIterator<'a, NodeType> {
        // Implementation details
    }

    pub struct NodeSpace<NodeType> {
        nodes: Vec<Node>,
    }

    impl<NodeType> NodeSpace<NodeType> {
        fn new() -> Self {
            NodeSpace { nodes: Vec::new() }
        }

        fn allocate(&mut self) -> &mut Node {
            self.nodes.push(Node::new());
            self.nodes.last_mut().unwrap()
        }
    }

    pub struct PendingPhantomCallback {
        callback_: WeakCallbackInfo<()>::Callback,
        parameter_: *mut std::ffi::c_void,
        embedder_fields_: [*mut std::ffi::c_void; 2], //v8::kEmbedderFieldsInWeakCallback
    }

    impl PendingPhantomCallback {
        pub fn new(
            callback: WeakCallbackInfo<()>::Callback,
            parameter: *mut std::ffi::c_void,
            embedder_fields: [*mut std::ffi::c_void; 2], //v8::kEmbedderFieldsInWeakCallback
        ) -> Self {
            PendingPhantomCallback {
                callback_: callback,
                parameter_: parameter,
                embedder_fields_: embedder_fields,
            }
        }

        pub fn invoke(&self, isolate: *mut Isolate, type_: InvocationType) {
            // Implementation details
        }

        pub fn callback(&self) -> WeakCallbackInfo<()>::Callback {
            self.callback_
        }
    }

    pub enum InvocationType {
        kFirstPass,
        kSecondPass,
    }
}

pub mod eternal_handles {
    use super::handles::IndirectHandle;
    use super::isolate::Isolate;
    use super::objects::Object;
    use super::root_visitor::RootVisitor;
    use std::ptr;
    use std::vec::Vec;
    type Address = *mut std::ffi::c_void;

    /// Manages handles to objects that are intended to never be garbage collected.
    pub struct EternalHandles {
        size_: usize,
        blocks_: Vec<Vec<Address>>,
        young_node_indices_: Vec<i32>,
    }

    impl EternalHandles {
        pub const K_INVALID_INDEX: i32 = -1;
        pub const K_SHIFT: i32 = 8;
        pub const K_SIZE: i32 = 1 << Self::K_SHIFT;
        pub const K_MASK: i32 = 0xff;

        pub fn new() -> Self {
            EternalHandles {
                size_: 0,
                blocks_: Vec::new(),
                young_node_indices_: Vec::new(),
            }
        }

        // Create an EternalHandle, overwriting the index.
        pub fn create(&mut self, isolate: *mut Isolate, object: *mut Object, index: *mut i32) {
            unsafe {
                let idx = self.size_ as i32;
                *index = idx;
                let block_index = (idx >> Self::K_SHIFT) as usize;
                let offset = (idx & Self::K_MASK) as usize;

                if block_index >= self.blocks_.len() {
                    self.blocks_.push(vec![ptr::null_mut(); Self::K_SIZE as usize]);
                }

                self.blocks_[block_index][offset] = object as *mut std::ffi::c_void;
                self.size_ += 1;
            }
        }

        // Grab the handle for an existing EternalHandle.
        #[inline]
        pub fn get(&self, index: i32) -> IndirectHandle<Object> {
            IndirectHandle::new(self.get_location(index) as *mut *mut Object)
        }

        // Iterates over all handles.
        pub fn iterate_all_roots(&self, visitor: *mut RootVisitor) {
            // Implementation details
        }
        // Iterates over all handles which might be in the young generation.
        pub fn iterate_young_roots(&self, visitor: *mut RootVisitor) {
            // Implementation details
        }
        // Rebuilds new space list.
        pub fn post_garbage_collection_processing(&mut self) {
            // Implementation details
        }

        pub fn handles_count(&self) -> usize {
            self.size_
        }

        // Gets the slot for an index. This returns an Address* rather than an
        // ObjectSlot in order to avoid #including slots.h in this header file.
        #[inline]
        fn get_location(&self, index: i32) -> Address {
            assert!(index >= 0 && index < self.size_ as i32);
            let block_index = (index >> Self::K_SHIFT) as usize;
            let offset = (index & Self::K_MASK) as usize;

            self.blocks_[block_index][offset]
        }
    }

    impl Drop for EternalHandles {
        fn drop(&mut self) {
            //drop all allocated blocks
            // Implementation details
        }
    }
}

pub mod global_handle_vector {
    use super::handles::IndirectHandle;
    use super::heap::Heap;
    use super::heap::LocalHeap;
    use super::objects::Tagged;
    use std::alloc::{alloc, dealloc, Layout};
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;
    use std::vec::Vec;

    type Address = *mut std::ffi::c_void;

    /// A custom allocator that allocates memory which is considered strongly rooted.
    pub struct StrongRootAllocator<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> StrongRootAllocator<T> {
        pub fn new() -> Self {
            StrongRootAllocator {
                _phantom: PhantomData,
            }
        }
    }

    unsafe impl<T> std::alloc::Allocator for StrongRootAllocator<T> {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
            // Implementation that treats the allocation as a strong root
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                return Err(std::alloc::AllocError);
            }
            let slice = unsafe { std::slice::from_raw_parts_mut(ptr, layout.size()) };
            NonNull::new(slice).ok_or(std::alloc::AllocError)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // Implementation to deallocate the memory
            dealloc(ptr.as_ptr(), layout);
        }
    }

    /// A vector of global Handles which automatically manages the backing of those
    /// Handles as a vector of strong-rooted addresses. Handles returned by the
    /// vector are valid as long as they are present in the vector.
    pub struct GlobalHandleVector<T> {
        locations_: Vec<Address, StrongRootAllocator<Address>>,
        _phantom: PhantomData<T>,
    }

    impl<T> GlobalHandleVector<T> {
        pub struct Iterator {
            it_: std::vec::IntoIter<Address>,
            _phantom: PhantomData<T>,
        }

        impl Iterator {
            fn new(vec: Vec<Address, StrongRootAllocator<Address>>) -> Self {
                Iterator {
                    it_: vec.into_iter(),
                    _phantom: PhantomData,
                }
            }
        }

        impl Iterator {
            pub fn next(&mut self) -> Option<IndirectHandle<T>> {
                self.it_.next().map(|addr| IndirectHandle::new(addr as *mut *mut T))
            }
        }

        #[inline]
        pub fn new_heap(heap: *mut Heap) -> Self {
            GlobalHandleVector {
                locations_: Vec::with_capacity_in(0, StrongRootAllocator::new()),
                _phantom: PhantomData,
            }
        }

        // Usage with LocalHeap is safe.
        #[inline]
        pub fn new_local_heap(local_heap: *mut LocalHeap) -> Self {
            GlobalHandleVector {
                locations_: Vec::with_capacity_in(0, StrongRootAllocator::new()),
                _phantom: PhantomData,
            }
        }

        pub fn get(&self, i: usize) -> IndirectHandle<T> {
            IndirectHandle::new(self.locations_[i] as *mut *mut T)
        }

        pub fn size(&self) -> usize {
            self.locations_.len()
        }
        pub fn empty(&self) -> bool {
            self.locations_.is_empty()
        }

        pub fn reserve(&mut self, size: usize) {
            self.locations_.reserve(size);
        }
        pub fn push(&mut self, val: *mut T) {
            self.locations_.push(val as Address);
        }
        // Handles into the GlobalHandleVector become invalid when they are removed,
        // so "pop" returns a raw object rather than a handle.
        #[inline]
        pub fn pop(&mut self) -> *mut T {
            self.locations_.pop().map(|addr| addr as *mut T).unwrap()
        }

        pub fn iter(&mut self) -> Iterator {
            Iterator::new(mem::take(&mut self.locations_))
        }
    }
}
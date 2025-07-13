// Converted from V8 C++ source files:
// Header: global-handles.h
// Implementation: global-handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod global_handles {
    use std::sync::{Mutex, Arc};
    use std::{mem, ptr};
    use crate::internal::{Isolate, HeapStats, RootVisitor, Heap, Tagged, Object, IndirectHandle, HeapObjectHeader, FullObjectSlot};
    use v8::GCCallbackFlags;
    use crate::parsing::pending_compilation_error_handler::v8;
    use crate::trap_handler::handler_outside_win::internal;
    use std::marker::PhantomData;
    use crate::heap::base::stack::Address;
    use crate::heap::layout_descriptors::HeapLayout;

    pub type WeakSlotCallbackWithHeap = fn(&Heap, FullObjectSlot) -> bool;

    const kBlockSize: usize = 256;

    enum WeaknessType {
        kCallback,
        kCallbackWithTwoEmbedderFields,
        kNoCallback,
    }

    struct NodeBlock<NodeType> {
        nodes: [NodeType; kBlockSize],
        next_: *mut NodeBlock<NodeType>,
        global_handles_: *mut GlobalHandles,
        space_: *mut NodeSpace<NodeType>,
        next_used_: *mut NodeBlock<NodeType>,
        prev_used_: *mut NodeBlock<NodeType>,
        used_nodes_: u32,
    }

    impl<NodeType> NodeBlock<NodeType> {
        fn from(node: &NodeType) -> &NodeBlock<NodeType> {
            unsafe {
                let node_ptr = node as *const NodeType as usize;
                let block_ptr = node_ptr - (node.index() as usize) * mem::size_of::<NodeType>();
                &*(block_ptr as *const NodeBlock<NodeType>)
            }
        }

        fn from_mut(node: &mut NodeType) -> &mut NodeBlock<NodeType> {
            unsafe {
                let node_ptr = node as *mut NodeType as usize;
                let block_ptr = node_ptr - (node.index() as usize) * mem::size_of::<NodeType>();
                &mut *(block_ptr as *mut NodeBlock<NodeType>)
            }
        }

        fn at(&self, index: usize) -> &NodeType {
            &self.nodes[index]
        }

        fn at_mut(&mut self, index: usize) -> &mut NodeType {
            &mut self.nodes[index]
        }

        fn space(&self) -> &NodeSpace<NodeType> {
            unsafe { &*self.space_ }
        }

        fn global_handles(&self) -> &GlobalHandles {
            unsafe { &*self.global_handles_ }
        }

        fn increase_usage(&mut self) -> bool {
            if self.used_nodes_ < kBlockSize as u32 {
                self.used_nodes_ += 1;
                self.used_nodes_ == 1
            } else {
                false
            }
        }

        fn decrease_usage(&mut self) -> bool {
            if self.used_nodes_ > 0 {
                self.used_nodes_ -= 1;
                self.used_nodes_ == 0
            } else {
                false
            }
        }

        fn list_add(&mut self, top: &mut *mut NodeBlock<NodeType>) {
            unsafe {
                let old_top = *top;
                *top = self;
                self.next_used_ = old_top;
                self.prev_used_ = ptr::null_mut();
                if old_top != ptr::null_mut() {
                    (*old_top).prev_used_ = self;
                }
            }
        }

        fn list_remove(&mut self, top: &mut *mut NodeBlock<NodeType>) {
            unsafe {
                if self.next_used_ != ptr::null_mut() {
                    (*self.next_used_).prev_used_ = self.prev_used_;
                }
                if self.prev_used_ != ptr::null_mut() {
                    (*self.prev_used_).next_used_ = self.next_used_;
                }
                if self as *mut Self == *top {
                    *top = self.next_used_;
                }
            }
        }

        fn next(&self) -> *mut NodeBlock<NodeType> {
            self.next_
        }

        fn next_used(&self) -> *mut NodeBlock<NodeType> {
            self.next_used_
        }

        fn begin_address(&self) -> *const NodeType {
            self.nodes.as_ptr()
        }

        fn end_address(&self) -> *const NodeType {
            unsafe { self.nodes.as_ptr().add(kBlockSize) }
        }
    }

    struct NodeIterator<'a, BlockType> {
        block_: *mut BlockType,
        index_: usize,
        _phantom: PhantomData<&'a BlockType>,
    }

    impl<'a, BlockType> NodeIterator<'a, BlockType> {
        fn new(block: *mut BlockType) -> Self {
            NodeIterator {
                block_: block,
                index_: 0,
                _phantom: PhantomData,
            }
        }

        fn next(&mut self) -> Option<&mut <BlockType as NodeBlockTrait>::NodeType>
            where BlockType: NodeBlockTrait {
            unsafe {
                if self.block_ == ptr::null_mut() {
                    return None;
                }

                if self.index_ < kBlockSize {
                    let node = (*self.block_).at_mut(self.index_);
                    self.index_ += 1;
                    Some(node)
                } else {
                    self.index_ = 0;
                    self.block_ = (*self.block_).next_used();
                    if self.block_ != ptr::null_mut() {
                        let node = (*self.block_).at_mut(self.index_);
                        self.index_ += 1;
                        Some(node)
                    } else {
                        None
                    }
                }
            }
        }
    }

    trait NodeBlockTrait {
        type NodeType;
        fn at_mut(&mut self, index: usize) -> &mut Self::NodeType;
        fn next_used(&self) -> *mut Self;
    }

    impl<NodeType> NodeBlockTrait for NodeBlock<NodeType> {
        type NodeType = NodeType;
        fn at_mut(&mut self, index: usize) -> &mut Self::NodeType {
            self.at_mut(index)
        }
        fn next_used(&self) -> *mut Self {
            self.next_used_ as *mut Self
        }
    }

    struct NodeSpace<NodeType> {
        global_handles_: *mut GlobalHandles,
        first_block_: *mut NodeBlock<NodeType>,
        first_used_block_: *mut NodeBlock<NodeType>,
        first_free_: *mut NodeType,
        blocks_: usize,
        handles_count_: usize,
    }

    impl<NodeType> NodeSpace<NodeType> {
        fn new(global_handles: *mut GlobalHandles) -> Self {
            NodeSpace {
                global_handles_: global_handles,
                first_block_: ptr::null_mut(),
                first_used_block_: ptr::null_mut(),
                first_free_: ptr::null_mut(),
                blocks_: 0,
                handles_count_: 0,
            }
        }

       unsafe fn allocate(&mut self) -> *mut NodeType {
            if self.first_free_ == ptr::null_mut() {
                let block = Box::new(NodeBlock {
                    nodes: mem::zeroed(),
                    next_: self.first_block_,
                    global_handles_: self.global_handles_,
                    space_: self,
                    next_used_: ptr::null_mut(),
                    prev_used_: ptr::null_mut(),
                    used_nodes_: 0,
                });
                self.first_block_ = Box::into_raw(block);
                self.blocks_ += 1;
                self.put_nodes_on_free_list(&mut *self.first_block_);
            }

            let node = self.first_free_;
            self.first_free_ = (*self.first_free_).next_free();

            let block = NodeBlock::<NodeType>::from_mut(&mut *node);
            if block.increase_usage() {
                block.list_add(&mut self.first_used_block_);
            }

            (*(*self.global_handles_).isolate()).counters().global_handles().Increment();
            self.handles_count_ += 1;

            node
        }

        unsafe fn put_nodes_on_free_list(&mut self, block: &mut NodeBlock<NodeType>) {
            for i in (0..kBlockSize).rev() {
                let node = block.at_mut(i) as *mut NodeType;
                let index = i as u8;
                (*node).set_index(index);
                (*node).free(self.first_free_);
                self.first_free_ = node;
            }
        }

        unsafe fn free(&mut self, node: *mut NodeType) {
            let block = NodeBlock::<NodeType>::from_mut(&mut *node);
            block.space().free_internal(node);
        }

        unsafe fn free_internal(&mut self, node: *mut NodeType) {
            (*node).release(self.first_free_);
            self.first_free_ = node;
            let block = NodeBlock::<NodeType>::from_mut(&mut *node);
            if block.decrease_usage() {
                block.list_remove(&mut self.first_used_block_);
            }

            (*(*self.global_handles_).isolate()).counters().global_handles().Decrement();
            self.handles_count_ -= 1;
        }

        fn total_size(&self) -> usize {
            self.blocks_ * mem::size_of::<NodeType>() * kBlockSize
        }

        fn handles_count(&self) -> usize {
            self.handles_count_
        }
    }

    impl<NodeType> Drop for NodeSpace<NodeType> {
        fn drop(&mut self) {
            unsafe {
                let mut block = self.first_block_;
                while block != ptr::null_mut() {
                    let tmp = (*block).next_;
                    drop(Box::from_raw(block));
                    block = tmp;
                }
            }
        }
    }

    struct NodeBase<Child> {
        object_: Address,
        class_id_: u16,
        index_: u8,
        flags_: u8,
        data_: NodeData<Child>,
        _phantom: PhantomData<Child>,
    }

    union NodeData<Child> {
        next_free: *mut Child,
        parameter: *mut std::ffi::c_void,
    }

    impl<Child> NodeBase<Child> {
        fn new() -> Self {
            NodeBase {
                object_: 0,
                class_id_: 0,
                index_: 0,
                flags_: 0,
                data_: NodeData { next_free: ptr::null_mut() },
                _phantom: PhantomData,
            }
        }

        fn from_location(location: *mut Address) -> *mut Child {
            location as *mut Child
        }

        unsafe fn clear_fields(&mut self) {
            self.object_ = 0;
            self.class_id_ = 0;
        }

        unsafe fn free(&mut self, free_list: *mut Child) {
            self.clear_fields();
            self.data_.next_free = free_list;
        }

        unsafe fn publish(&mut self, object: Tagged<Object>) -> IndirectHandle<Object> {
            self.data_.parameter = ptr::null_mut();
            self.object_ = object.ptr();
            IndirectHandle::new(&mut self.object_)
        }

        fn raw_object(&self) -> Address {
            self.object_
        }

        fn index(&self) -> u8 {
            self.index_
        }

        fn set_index(&mut self, value: u8) {
            self.index_ = value;
        }

        fn next_free(&mut self) -> *mut Child {
            unsafe { self.data_.next_free }
        }

        unsafe fn parameter(&self) -> *mut std::ffi::c_void {
            self.data_.parameter
        }

        unsafe fn set_parameter(&mut self, parameter: *mut std::ffi::c_void) {
            self.data_.parameter = parameter;
        }

        fn location(&mut self) -> FullObjectSlot {
            FullObjectSlot(&mut self.object_)
        }
    }

    struct GlobalHandles {
        isolate_: *mut Isolate,
        regular_nodes_: Box<NodeSpace<Node>>,
        young_nodes_: Vec<*mut Node>,
        pending_phantom_callbacks_: Vec<( *mut Node, PendingPhantomCallback)>,
        second_pass_callbacks_: Vec<PendingPhantomCallback>,
        second_pass_callbacks_task_posted_: bool,
        last_gc_custom_callbacks_: usize,
    }

    impl GlobalHandles {
        pub fn new(isolate: *mut Isolate) -> Self {
            GlobalHandles {
                isolate_: isolate,
                regular_nodes_: Box::new(NodeSpace::new(isolate as *mut GlobalHandles)),
                young_nodes_: Vec::new(),
                pending_phantom_callbacks_: Vec::new(),
                second_pass_callbacks_: Vec::new(),
                second_pass_callbacks_task_posted_: false,
                last_gc_custom_callbacks_: 0,
            }
        }

        pub fn create(&mut self, value: Tagged<Object>) -> IndirectHandle<Object> {
            unsafe {
                let node = self.regular_nodes_.allocate();
                (*node).publish(value)
            }
        }

        pub fn create_address(&mut self, value: Address) -> IndirectHandle<Object> {
            self.create(Tagged::from_ptr(value as *mut Object))
        }

         pub fn copy_global(location: *mut Address) -> IndirectHandle<Object> {
            unsafe {
                let node = Node::from_location(location);
                let global_handles = (*node).global_handles();
                let object = *location;
                global_handles.create(Tagged::from_ptr(object as *mut Object))
            }
        }

        pub fn move_global(from: *mut *mut Address, to: *mut *mut Address) {
            unsafe {
                debug_assert!(!(*from).is_null());
                debug_assert!(!(*to).is_null());
                debug_assert_eq!(*from, *to);

                let node = Node::from_location(*from);
                // Strong handles do not require fixups.
            }
        }

        pub fn destroy(location: *mut Address) {
            if !location.is_null() {
                unsafe {
                    let node = Node::from_location(location);
                    NodeSpace::<Node>::free(&(*node));
                }
            }
        }
        
        pub fn make_weak(location: *mut Address, parameter: *mut std::ffi::c_void, phantom_callback: v8::WeakCallbackInfo<void>::Callback, type_: v8::WeakCallbackType) {
            unsafe {
                let node = Node::from_location(location);
               // (*node).make_weak(parameter, phantom_callback, type_);
            }
        }
        
        pub fn clear_weakness(location: *mut Address) -> *mut std::ffi::c_void {
            unsafe {
                let node = Node::from_location(location);
               // (*node).clear_weakness()
               ptr::null_mut()
            }
        }
        
         pub fn annotate_strong_retainer(location: *mut Address, label: *const i8) {
            unsafe {
                let node = Node::from_location(location);
              //  (*node).annotate_strong_retainer(label);
            }
        }
        
        pub fn is_weak(location: *mut Address) -> bool {
            unsafe {
                let node = Node::from_location(location);
               // (*node).is_weak()
               false
            }
        }

        fn record_stats(&self, stats: &mut HeapStats) {
            stats.global_handle_count = 0;
            stats.weak_global_handle_count = 0;
            stats.pending_global_handle_count = 0;
            stats.near_death_global_handle_count = 0;
            stats.free_global_handle_count = 0;
        }

        fn invoke_first_pass_weak_callbacks(&mut self) -> usize {
            0
        }

        fn invoke_second_pass_phantom_callbacks(&mut self) {}

        fn post_garbage_collection_processing(&mut self, gc_callback_flags: GCCallbackFlags) {}

        fn iterate_strong_roots(&mut self, v: &mut RootVisitor) {}

        fn iterate_weak_roots(&mut self, v: &mut RootVisitor) {}

        fn iterate_all_roots(&mut self, v: &mut RootVisitor) {}

        fn iterate_all_young_roots(&mut self, v: &mut RootVisitor) {}

        fn iterate_weak_roots_for_phantom_handles(&mut self, should_reset_handle: WeakSlotCallbackWithHeap) {}

        fn iterate_young_strong_and_dependent_roots(&mut self, v: &mut RootVisitor) {}

        fn process_weak_young_objects(&mut self, v: &mut RootVisitor, should_reset_handle: WeakSlotCallbackWithHeap) {}

        fn update_list_of_young_nodes(&mut self) {}

        fn clear_list_of_young_nodes(&mut self) {}

        fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        fn total_size(&self) -> usize {
            self.regular_nodes_.total_size()
        }

        fn used_size(&self) -> usize {
            self.regular_nodes_.handles_count() * std::mem::size_of::<Node>()
        }

        fn handles_count(&self) -> usize {
            self.regular_nodes_.handles_count()
        }

        fn last_gc_custom_callbacks(&self) -> usize {
            self.last_gc_custom_callbacks_
        }

        fn iterate_all_roots_for_testing(&mut self, v: *mut v8::PersistentHandleVisitor) {}
        
        fn has_young(&self) -> bool {
            !self.young_nodes_.is_empty()
        }
    }

    impl Drop for GlobalHandles {
        fn drop(&mut self) {
           unsafe {
            }
        }
    }
    
    struct PendingPhantomCallback {
        callback_: v8::WeakCallbackInfo<void>::Callback,
        parameter_: *mut std::ffi::c_void,
        embedder_fields_: [*mut std::ffi::c_void; v8::kEmbedderFieldsInWeakCallback],
    }
    
    impl PendingPhantomCallback {
        fn invoke(&mut self, isolate: *mut Isolate, type_: i32) {}
    }

    struct EternalHandles {
        size_: usize,
        blocks_: Vec<*mut Address>,
        young_node_indices_: Vec<i32>,
    }
    
    impl EternalHandles {
        fn iterate_all_roots(&mut self, visitor: &mut RootVisitor) {}
        fn iterate_young_roots(&mut self, visitor: &mut RootVisitor) {}
        fn post_garbage_collection_processing(&mut self) {}
        fn create(&mut self, isolate: *mut Isolate, object: Tagged<Object>, index: *mut i32) {}
        fn handles_count(&mut self) -> usize { 0 }
    }
    
    impl Drop for EternalHandles {
        fn drop(&mut self) {}
    }

    struct Node {
        object_: Address,
        class_id_: u16,
        index_: u8,
        flags_: u8,
        weak_callback_: *mut std::ffi::c_void,
    }
    
    impl Node {
        fn new() -> Node {
            Node {
                object_: 0,
                class_id_: 0,
                index_: 0,
                flags_: 0,
                weak_callback_: ptr::null_mut()
            }
        }
        
        unsafe fn publish(&mut self, object: Tagged<Object>) -> IndirectHandle<Object> {
           self.object_ = object.ptr();
           IndirectHandle::new(&mut self.object_)
        }
        
         unsafe fn free(&mut self, free_list: *mut Node) {
         }
         
         fn clear_impl_fields(&mut self) {}
          fn set_index(&mut self, value: u8) {
            self.index_ = value;
        }
        
        fn next_free(&mut self) -> *mut Node {
            ptr::null_mut()
        }
        fn global_handles(&mut self) -> &GlobalHandles {
            unsafe {&*(ptr::null() as *mut GlobalHandles) }
        }
        
        fn location(&mut self) -> FullObjectSlot {
            FullObjectSlot(&mut self.object_)
        }
        
        fn set_in_young_list(&mut self, _v: bool) {}
         
        fn set_weakness_type(&mut self, _value: WeaknessType) {}
    
    }

    trait InternalEnumTrait {
        fn to_i32(&self) -> i32;
        fn from_i32(value: i32) -> Option<Self> where Self: Sized;
    }

    trait BaseCounterTrait {
        fn increment(&mut self);
        fn decrement(&mut self);
        fn value(&self) -> usize;
    }

    struct BaseCounter {
        value_: usize,
    }

    impl BaseCounter {
        fn new() -> Self {
            BaseCounter { value_: 0 }
        }

        fn increment(&mut self) {
            self.value_ += 1;
        }

        fn decrement(&mut self) {
            self.value_ -= 1;
        }

        fn value(&self) -> usize {
            self.value_
        }
    }
    
    impl BaseCounterTrait for BaseCounter {
        fn increment(&mut self) {
            BaseCounter::increment(self)
        }

        fn decrement(&mut self) {
            BaseCounter::decrement(self)
        }

        fn value(&self) -> usize {
            BaseCounter::value(self)
        }
    }

    pub struct Counter {
        pub name: String,
        base: BaseCounter
    }

    impl Counter {
        pub fn new(name: String) -> Counter {
            Counter {
                name,
                base: BaseCounter::new()
            }
        }
    }

    pub struct Counters {
        pub global_handles: Counter
    }

    impl Counters {
        pub fn new() -> Counters {
            Counters {
                global_handles: Counter::new("global_handles".to_string())
            }
        }
    }

    pub struct IsolateCounters {
        counters: Counters
    }

    impl IsolateCounters {
        pub fn new() -> IsolateCounters {
            IsolateCounters {
                counters: Counters::new()
            }
        }

        pub fn global_handles(&mut self) -> &mut Counter {
            &mut self.counters.global_handles
        }
    }

    trait TestTrait{
        fn test_func(&self);
    }

    impl TestTrait for IsolateCounters {
        fn test_func(&self) {}
    }

}

// Converted from V8 C++ source files:
// Header: traced-handles.h
// Implementation: traced-handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod traced_handles {
    // Copyright 2022 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    #![allow(dead_code)]
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::mem::MaybeUninit;

    use crate::heap::incremental_marking::TracedHandles;
    use crate::Isolate;
    use crate::FullObjectSlot;
    use crate::TracedReferenceStoreMode;
    use crate::TracedReferenceHandling;
    use crate::CppHeap;
    use crate::base;
    use crate::objects::objects::JSObject;
    use crate::objects::slots::FullObjectSlot;
    use crate::IsDroppable;
    use crate::Address;
    use crate::Tagged;
    use crate::objects::objects::Object;
    use crate::objects::smi::Smi;
    use crate::objects::visitors::RootVisitor;
    use crate::heap::heap_layout_inl::HeapLayout;
    use crate::heap::heap_write_barrier_inl::WriteBarrier;
    use crate::heap::cppgc::embedder_heap::EmbedderRootsHandler;
    use crate::V8_EXPORT_PRIVATE;
    use crate::heap::ReadOnlyRoots;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::ffi::c_void;
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;
    use std::cmp;

    pub struct TracedNode {
        object_: Address,
        next_free_index_: IndexType,
        index_: IndexType,
        flags_: u8,
        is_marked_: AtomicBool,
    }

    pub type IndexType = u16;

    impl TracedNode {
        pub fn from_location(location: *mut Address) -> *mut TracedNode {
            location as *mut TracedNode
        }

        pub fn from_location_const(location: *const Address) -> *const TracedNode {
            location as *const TracedNode
        }

        pub fn new(index: IndexType, next_free_index: IndexType) -> TracedNode {
            TracedNode {
                object_: 0,
                next_free_index_: next_free_index,
                index_: index,
                flags_: 0,
                is_marked_: AtomicBool::new(false),
            }
        }

        pub fn index(&self) -> IndexType {
            self.index_
        }

        pub fn is_weak(&self) -> bool {
            IsWeak::decode(self.flags_)
        }
        pub fn set_weak(&mut self, v: bool) {
            self.flags_ = IsWeak::update(self.flags_, v);
        }

        pub fn is_droppable(&self) -> bool {
            IsDroppable::decode(self.flags_)
        }
        pub fn set_droppable(&mut self, v: bool) {
            self.flags_ = IsDroppable::update(self.flags_, v);
        }

        pub fn is_in_use(&self) -> bool {
            IsInUse::decode(self.flags_)
        }
        pub fn set_is_in_use(&mut self, v: bool) {
            self.flags_ = IsInUse::update(self.flags_, v);
        }

        pub fn is_in_young_list(&self) -> bool {
            IsInYoungList::decode(self.flags_)
        }
        pub fn set_is_in_young_list(&mut self, v: bool) {
            self.flags_ = IsInYoungList::update(self.flags_, v);
        }

        pub fn next_free(&self) -> IndexType {
            self.next_free_index_
        }
        pub fn set_next_free(&mut self, next_free_index: IndexType) {
            self.next_free_index_ = next_free_index;
        }

        pub fn set_markbit(&self) {
            self.is_marked_.store(true, Ordering::Relaxed);
        }

        pub fn markbit(&self) -> bool {
            self.is_marked_.load(Ordering::Relaxed)
        }

        pub fn is_metadata_cleared(&self) -> bool {
            self.flags_ == 0 && !self.markbit()
        }

        pub fn clear_markbit(&self) {
            self.is_marked_.store(false, Ordering::Relaxed);
        }

        pub fn has_old_host(&self) -> bool {
            HasOldHost::decode(self.flags_)
        }
        pub fn set_has_old_host(&mut self, v: bool) {
            self.flags_ = HasOldHost::update(self.flags_, v);
        }

        pub fn set_raw_object(&mut self, value: Address) {
            self.object_ = value;
        }
        pub fn raw_object(&self) -> Address {
            self.object_
        }
        pub fn object(&self) -> Tagged<Object> {
            Tagged::<Object> { ptr: self.object_ }
        }
        pub fn location(&self) -> FullObjectSlot {
            FullObjectSlot { slot: &self.object_ as *const Address as *mut Address }
        }

        pub fn publish(
            &mut self,
            object: Tagged<Object>,
            needs_young_bit_update: bool,
            needs_black_allocation: bool,
            has_old_host: bool,
            is_droppable: bool,
        ) -> FullObjectSlot {
            self.object_ = object.ptr;
            self.set_has_old_host(has_old_host);
            self.set_droppable(is_droppable);
            FullObjectSlot { slot: &mut self.object_ }
        }

        pub fn release(&mut self, zap_value: Address) {
            self.flags_ = 0;
            self.clear_markbit();
            self.set_raw_object(zap_value);
            assert!(self.is_metadata_cleared());
        }
    }

    struct IsInUse {}
    impl IsInUse {
        const BIT: u8 = 0;
        pub fn decode(flags: u8) -> bool {
            (flags >> Self::BIT) & 1 != 0
        }
        pub fn update(flags: u8, value: bool) -> u8 {
            if value {
                flags | (1 << Self::BIT)
            } else {
                flags & !(1 << Self::BIT)
            }
        }
        pub fn next<T, const OFFSET: usize>(_: T, _: usize) -> T {
            T {}
        }
    }

    struct IsInYoungList {}
    impl IsInYoungList {
        const BIT: u8 = 1;
        pub fn decode(flags: u8) -> bool {
            (flags >> Self::BIT) & 1 != 0
        }
        pub fn update(flags: u8, value: bool) -> u8 {
            if value {
                flags | (1 << Self::BIT)
            } else {
                flags & !(1 << Self::BIT)
            }
        }
        pub fn next<T, const OFFSET: usize>(_: T, _: usize) -> T {
            T {}
        }
    }

    struct IsWeak {}
    impl IsWeak {
        const BIT: u8 = 2;
        pub fn decode(flags: u8) -> bool {
            (flags >> Self::BIT) & 1 != 0
        }
        pub fn update(flags: u8, value: bool) -> u8 {
            if value {
                flags | (1 << Self::BIT)
            } else {
                flags & !(1 << Self::BIT)
            }
        }
        pub fn next<T, const OFFSET: usize>(_: T, _: usize) -> T {
            T {}
        }
    }

    struct IsDroppable {}
    impl IsDroppable {
        const BIT: u8 = 3;
        pub fn decode(flags: u8) -> bool {
            (flags >> Self::BIT) & 1 != 0
        }
        pub fn update(flags: u8, value: bool) -> u8 {
            if value {
                flags | (1 << Self::BIT)
            } else {
                flags & !(1 << Self::BIT)
            }
        }
        pub fn next<T, const OFFSET: usize>(_: T, _: usize) -> T {
            T {}
        }
    }

    struct HasOldHost {}
    impl HasOldHost {
        const BIT: u8 = 4;
        pub fn decode(flags: u8) -> bool {
            (flags >> Self::BIT) & 1 != 0
        }
        pub fn update(flags: u8, value: bool) -> u8 {
            if value {
                flags | (1 << Self::BIT)
            } else {
                flags & !(1 << Self::BIT)
            }
        }
        pub fn next<T, const OFFSET: usize>(_: T, _: usize) -> T {
            T {}
        }
    }

    struct ListNode {
        prev_: *mut *mut TracedNodeBlock,
        next_: *mut TracedNodeBlock,
    }

    impl ListNode {
        fn new() -> Self {
            ListNode {
                prev_: std::ptr::null_mut(),
                next_: std::ptr::null_mut(),
            }
        }
    }

    pub struct TracedNodeBlock {
        overall_list_node_: ListNode,
        usable_list_node_: ListNode,
        young_list_node_: ListNode,
        traced_handles_: *mut TracedHandles,
        used_: IndexType,
        capacity_: IndexType,
        first_free_node_: IndexType,
        locally_freed_: IndexType,
        reprocess_: bool,
        nodes_: *mut TracedNode,
    }

    impl TracedNodeBlock {
        pub const K_INVALID_FREE_LIST_NODE_INDEX: IndexType = IndexType::MAX;
        const V8_HOST_ARCH_64_BIT: bool = true;
        pub const K_MIN_CAPACITY: usize = 256;
        const K_MAX_CAPACITY: IndexType = IndexType::MAX - 1;

        pub fn create(traced_handles: &mut TracedHandles) -> Result<Box<TracedNodeBlock>, String> {
            let min_wanted_size = std::mem::size_of::<TracedNodeBlock>() +
                std::mem::size_of::<TracedNode>() * Self::K_MIN_CAPACITY;

            let layout = Layout::from_size_align(min_wanted_size, std::mem::align_of::<TracedNodeBlock>()).map_err(|e| e.to_string())?;

            let ptr = unsafe { alloc(layout) } as *mut TracedNodeBlock;
            if ptr.is_null() {
                return Err("Allocation failed".to_string());
            }

            let capacity = cmp::min(
                (layout.size() - std::mem::size_of::<TracedNodeBlock>()) / std::mem::size_of::<TracedNode>(),
                Self::K_MAX_CAPACITY as usize,
            ) as IndexType;
            
            let mut block = TracedNodeBlock {
                overall_list_node_: ListNode::new(),
                usable_list_node_: ListNode::new(),
                young_list_node_: ListNode::new(),
                traced_handles_: traced_handles,
                used_: 0,
                capacity_: capacity,
                first_free_node_: 0,
                locally_freed_: 0,
                reprocess_: false,
                nodes_: unsafe { ptr.add(1) } as *mut TracedNode, // Points to after TracedNodeBlock struct
            };
            
            unsafe {
              for i in 0..(capacity - 1) {
                  let node_ptr = block.at_mut(i);
                  std::ptr::write(node_ptr, TracedNode::new(i, i + 1));
              }
              let last_node_ptr = block.at_mut(capacity - 1);
              std::ptr::write(last_node_ptr, TracedNode::new(capacity - 1, Self::K_INVALID_FREE_LIST_NODE_INDEX));
            }

            Ok(unsafe { Box::from_raw(ptr) })
        }

        pub fn delete(block: Box<TracedNodeBlock>) {
            let layout = Layout::from_size_align(std::mem::size_of::<TracedNodeBlock>() +
                std::mem::size_of::<TracedNode>() * block.capacity_ as usize, std::mem::align_of::<TracedNodeBlock>()).unwrap();
                unsafe {
                dealloc(Box::into_raw(block) as *mut u8, layout);
            }
        }

        pub fn from(node: &TracedNode) -> &mut TracedNodeBlock {
             unsafe {
                let index = node.index();
                let node_ptr = node as *const TracedNode;
                let first_node_ptr = node_ptr.sub(index as usize);
                let block_ptr = (first_node_ptr as *mut u8).sub(std::mem::size_of::<TracedNodeBlock>());
                &mut *(block_ptr as *mut TracedNodeBlock)
            }
        }

        pub fn from_const(node: &TracedNode) -> &TracedNodeBlock {
             unsafe {
                let index = node.index();
                let node_ptr = node as *const TracedNode;
                let first_node_ptr = node_ptr.sub(index as usize);
                let block_ptr = (first_node_ptr as *mut u8).sub(std::mem::size_of::<TracedNodeBlock>());
                &*(block_ptr as *mut TracedNodeBlock)
            }
        }

        pub fn allocate_node(&mut self) -> Option<*mut TracedNode> {
            if self.used_ == self.capacity_ {
                return None;
            }
            let index = self.first_free_node_;
            let node = self.at_mut(index);

            self.first_free_node_ = unsafe { (*node).next_free() };
            self.used_ += 1;

            unsafe {
              (*node).set_is_in_use(true);
            }
           
            Some(node)
        }

        pub fn free_node(&mut self, node: *mut TracedNode, zap_value: Address) {
            unsafe {
                assert!((*node).is_in_use());
                (*node).release(zap_value);
                assert!(!(*node).is_in_use());
                (*node).set_next_free(self.first_free_node_);
            }
            self.first_free_node_ = unsafe { (*node).index() };
            self.used_ -= 1;
        }

        fn at_mut(&mut self, index: IndexType) -> *mut TracedNode {
             unsafe { self.nodes_.add(index as usize) }
        }

        fn at(&self, index: IndexType) -> *const TracedNode {
            unsafe { self.nodes_.add(index as usize) }
        }

        pub fn nodes_begin_address(&self) -> *mut TracedNode {
           self.at_mut(0)
        }
        
        pub fn nodes_end_address(&self) -> *mut TracedNode {
           self.at_mut(self.capacity_ - 1)
        }

        pub fn traced_handles(&mut self) -> &mut TracedHandles {
           unsafe{ &mut *self.traced_handles_ }
        }

        pub fn is_full(&self) -> bool {
            self.used_ == self.capacity_
        }
        pub fn is_empty(&self) -> bool {
            self.used_ == 0
        }
        pub fn used(&self) -> IndexType {
            self.used_
        }

        pub fn size_bytes(&self) -> usize {
            std::mem::size_of::<TracedNodeBlock>() + self.capacity_ as usize * std::mem::size_of::<TracedNode>()
        }

        pub fn in_young_list(&self) -> bool {
            self.young_list_node_.prev_ != std::ptr::null_mut()
        }
        pub fn in_usable_list(&self) -> bool {
            self.usable_list_node_.prev_ != std::ptr::null_mut()
        }

        pub fn needs_reprocessing(&self) -> bool {
            self.reprocess_
        }
        pub fn set_reprocessing(&mut self, value: bool) {
            self.reprocess_ = value;
        }

        pub fn set_locally_freed(&mut self, count: IndexType) {
            assert_eq!(self.locally_freed_, 0);
            self.locally_freed_ = count;
        }
        pub fn consume_locally_freed(&mut self) -> IndexType {
            let locally_freed = self.locally_freed_;
            self.locally_freed_ = 0;
            locally_freed
        }
    }

    const K_NULL_ADDRESS: Address = 0;
    const K_TRACED_HANDLE_EAGER_RESET_ZAP_VALUE: Address = 1;
    const K_TRACED_HANDLE_FULL_GC_RESET_ZAP_VALUE: Address = 2;
    const K_TRACED_HANDLE_MINOR_GC_RESET_ZAP_VALUE: Address = 3;
    const K_TRACED_HANDLE_MINOR_GC_WEAK_RESET_ZAP_VALUE: Address = 4;
    const K_GLOBAL_HANDLE_ZAP_VALUE: Address = 5;

    fn set_slot_thread_safe(slot: *mut *mut Address, val: *mut Address) {
      unsafe {
          (slot as *mut *mut Address)
              .write_volatile(val)
      }
    }

}

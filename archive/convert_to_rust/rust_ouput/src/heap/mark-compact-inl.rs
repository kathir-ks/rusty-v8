// Converted from V8 C++ source files:
// Header: mark-compact-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_mark_compact_inl {
use crate::common::globals::*;
use crate::heap::heap_visitor_inl::*;
use crate::heap::mark_compact::*;
use crate::heap::marking_state_inl::*;
use crate::heap::marking_visitor_inl::*;
use crate::heap::marking_worklist_inl::*;
use crate::heap::marking_worklist::*;
use crate::heap::marking::*;
use crate::heap::remembered_set_inl::*;
use crate::objects::js_collection_inl::*;
use crate::objects::transitions::*;
use crate::roots::roots::*;
use crate::heap::memory_chunk::MemoryChunk;
use crate::heap::heap::Heap;
use crate::objects::heap_object::HeapObject;
use crate::objects::object::Object;
use crate::objects::object::Tagged;
use crate::heap::marking::MarkingHelper;
use crate::heap::marking::WorklistTarget;
use crate::heap::local_marking_worklists::LocalMarkingWorklists;
use crate::heap::marking_state::MarkingState;
use crate::heap::read_only_heap::ReadOnlyHeap;
use crate::heap::full_object_slot::FullObjectSlot;
use crate::objects::map::MapWord;
use crate::objects::heap_object::Cast;
use std::marker::PhantomData;
use crate::heap::mutable_page_metadata::MutablePageMetadata;
use crate::heap::remembered_set::AccessMode;
use crate::heap::space::Space;
use crate::base::address::Address;
use crate::isolate::isolate::Isolate;
use crate::heap::local_weak_objects::LocalWeakObjects;
use crate::objects::transition_array::TransitionArray;
use crate::objects::string::String;
use crate::base::flags::Flags;

  impl MarkCompactCollector {
    pub fn mark_object(
      &mut self,
      host: Tagged<HeapObject>,
      obj: Tagged<HeapObject>,
      target_worklist: MarkingHelper::WorklistTarget,
    ) {
      if !ReadOnlyHeap::contains(obj) && !self.heap_.contains(obj) {
        return;
      }
      MarkingHelper::try_mark_and_push(
        &mut self.heap_,
        self.local_marking_worklists_.as_mut().unwrap(),
        &mut self.marking_state_,
        target_worklist,
        obj,
      );
    }

    pub fn mark_root_object(
      &mut self,
      root: Root,
      obj: Tagged<HeapObject>,
      target_worklist: MarkingHelper::WorklistTarget,
    ) {
      if !ReadOnlyHeap::contains(obj) && !self.heap_.contains(obj) {
        return;
      }
      MarkingHelper::try_mark_and_push(
        &mut self.heap_,
        self.local_marking_worklists_.as_mut().unwrap(),
        &mut self.marking_state_,
        target_worklist,
        obj,
      );
    }

    pub fn record_slot<THeapObjectSlot>(
      object: Tagged<HeapObject>,
      slot: THeapObjectSlot,
      target: Tagged<HeapObject>,
    ) where THeapObjectSlot: HeapObjectSlotTrait {
      let source_page = MemoryChunk::from_heap_object(object);
      if !source_page.should_skip_evacuation_slot_recording() {
        MarkCompactCollector::record_slot_internal(source_page, slot, target);
      }
    }

    pub fn record_slot_internal<THeapObjectSlot>(
      source_chunk: &mut MemoryChunk,
      slot: THeapObjectSlot,
      target: Tagged<HeapObject>,
    ) where THeapObjectSlot: HeapObjectSlotTrait {
      let target_chunk = MemoryChunk::from_heap_object(target);
      if target_chunk.is_evacuation_candidate() {
        let source_page = MutablePageMetadata::cast(source_chunk.metadata());
        if target_chunk.is_flag_set(MemoryChunk::IS_EXECUTABLE) {
          if !target_chunk.address().inside_sandbox(){
            RememberedSet::<TrustedToCode>::insert::<AccessMode::ATOMIC>(
              source_page,
              source_chunk.offset(slot.address()),
            );
          }
        } else if source_chunk.is_flag_set(MemoryChunk::IS_TRUSTED) &&
                  target_chunk.is_flag_set(MemoryChunk::IS_TRUSTED) {
          if !target_chunk.address().inside_sandbox(){
            RememberedSet::<TrustedToTrusted>::insert::<AccessMode::ATOMIC>(
              source_page,
              source_chunk.offset(slot.address()),
            );
          }
        } else if !target_chunk.in_writable_shared_space() ||
                  source_page.heap().isolate().is_shared_space_isolate() {
          RememberedSet::<OldToOld>::insert::<AccessMode::ATOMIC>(
            source_page,
            source_chunk.offset(slot.address()),
          );
        } else {
          if !source_page.heap().isolate().is_shared_space_isolate() {
            if target_chunk.heap().isolate().is_shared_space_isolate() {
              if target_chunk.in_writable_shared_space() {}
            }
          }
        }
      }
    }

    pub fn add_transition_array(&mut self, array: Tagged<TransitionArray>) {
      self.local_weak_objects().transition_arrays_local.push(array);
    }
  }

  impl RootMarkingVisitor<'_> {
    pub fn visit_root_pointer(&mut self, root: Root, description: &str, p: FullObjectSlot) {
      if !MapWord::is_packed(p.relaxed_load().ptr()) {
        self.mark_object_by_pointer(root, p);
      }
    }

    pub fn visit_root_pointers(
      &mut self,
      root: Root,
      description: &str,
      start: FullObjectSlot,
      end: FullObjectSlot,
    ) {
      let mut p = start;
      while p < end {
        self.mark_object_by_pointer(root, p);
        p = FullObjectSlot{address: p.address.wrapping_add(1)};
      }
    }

    fn mark_object_by_pointer(&mut self, root: Root, p: FullObjectSlot) {
      let object: Tagged<Object> = *p;

      if object.ptr() == kTaggedNullAddress {
        return;
      }

      if !object.is_heap_object() {
        return;
      }

      let heap_object = Cast::<HeapObject>(object);
      let target_worklist =
        MarkingHelper::should_mark_object(&self.collector_.heap(), heap_object);

      if target_worklist.is_none() {
        return;
      }
      self.collector_.mark_root_object(root, heap_object, target_worklist.unwrap());
    }
  }
  trait HeapObjectSlotTrait {
    fn address(&self) -> Address;
  }
  impl HeapObjectSlotTrait for FullObjectSlot {
    fn address(&self) -> Address {
      self.address
    }
  }
}

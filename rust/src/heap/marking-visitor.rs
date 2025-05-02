// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/marking-visitor.h

use std::collections::HashMap;
use std::hash::{Hasher, BuildHasherDefault};
use std::marker::PhantomData;

use crate::base::logging::*; // Assuming logging is in base module
use crate::common::globals::*; // Assuming globals is in common module
use crate::execution::isolate::*; // Assuming isolate is in execution module
use crate::heap::heap_visitor::*; // Assuming heap_visitor is in heap module
use crate::heap::marking_state::*; // Assuming marking_state is in heap module
use crate::heap::marking_worklist::*; // Assuming marking_worklist is in heap module
use crate::heap::marking::*; // Assuming marking is in heap module
use crate::heap::pretenuring_handler::*; // Assuming pretenuring_handler is in heap module
use crate::heap::spaces::*; // Assuming spaces is in heap module
use crate::heap::weak_object_worklists::*; // Assuming weak_object_worklists is in heap module

// type KeyToValues =
//     absl::flat_hash_map<Tagged<HeapObject>,
//                         base::SmallVector<Tagged<HeapObject>, 1>,
//                         Object::Hasher, Object::KeyEqualSafe>;

// Using a standard HashMap since absl::flat_hash_map isn't directly available.
// Assuming `Tagged<HeapObject>` implements `Eq` and `Hash`.
// Also assuming `SmallVector` can be replaced with a `Vec` for simplicity.
pub type KeyToValues = HashMap<TaggedHeapObject, Vec<TaggedHeapObject>>;

pub trait ObjectHasher {
    fn calculate(&self, obj: &TaggedHeapObject) -> u64;
}

// Placeholder for Object::Hasher.  Needs to be implemented based on TaggedHeapObject.
// struct DefaultObjectHasher;

// impl Hasher for DefaultObjectHasher {
//     fn finish(&self) -> u64 {
//         0 // Replace with actual hash calculation logic
//     }

//     fn write(&mut self, bytes: &[u8]) {
//         // Implement writing bytes for hashing if needed.
//         unimplemented!()
//     }
// }

// type KeyToValues = HashMap<TaggedHeapObject, Vec<TaggedHeapObject>, BuildHasherDefault<DefaultObjectHasher>>;

// Placeholder for Object::KeyEqualSafe.  Needs to be implemented based on TaggedHeapObject.
pub trait KeyEqualSafe {
    fn equals(&self, a: &TaggedHeapObject, b: &TaggedHeapObject) -> bool;
}

// struct DefaultKeyEqualSafe;

// impl KeyEqualSafe for DefaultKeyEqualSafe {
//     fn equals(&self, a: &TaggedHeapObject, b: &TaggedHeapObject) -> bool {
//         a == b // Implement equality check based on TaggedHeapObject
//     }
// }

// trait MarkingVisitorMethods {
//     fn can_update_values_in_heap(&self) -> bool;
//     fn add_strong_reference_for_reference_summarizer(&self, host: TaggedHeapObject, obj: TaggedHeapObject);
//     fn add_weak_reference_for_reference_summarizer(&self, host: TaggedHeapObject, obj: TaggedHeapObject);
//     fn marking_state(&self) -> &MarkingState;
//     fn mark_pointer_table_entry(&self, obj: TaggedHeapObject, slot: IndirectPointerSlot);
//     fn record_slot(&self); // Arguments unclear from C++ header
//     fn record_reloc_slot(&self); // Arguments unclear from C++ header
// }

// The base class for all marking visitors (main and concurrent marking) but
// also for e.g. the reference summarizer. It implements marking logic with
// support for bytecode flushing, embedder tracing and weak references.
//
// Derived classes are expected to provide the following methods:
// - CanUpdateValuesInHeap
// - AddStrongReferenceForReferenceSummarizer
// - AddWeakReferenceForReferenceSummarizer
// - marking_state
// - MarkPointerTableEntry
// - RecordSlot
// - RecordRelocSlot

pub struct MarkingVisitorBase<ConcreteVisitor> {
    pub base: ConcurrentHeapVisitor<ConcreteVisitor>,
    local_marking_worklists_: *mut MarkingWorklistsLocal, // Using raw pointer for mutability. Consider alternatives.
    local_weak_objects_: *mut WeakObjectsLocal, // Using raw pointer for mutability. Consider alternatives.
    heap_: *mut Heap,
    mark_compact_epoch_: u32,
    code_flush_mode_: EnumSet<CodeFlushMode>,
    should_keep_ages_unchanged_: bool,
    code_flushing_increase_: u16,
    isolate_in_background_: bool,
    // TODO: Implement V8_COMPRESS_POINTERS feature
    // external_pointer_table_: *mut ExternalPointerTable,
    // shared_external_pointer_table_: *mut ExternalPointerTable,
    // shared_external_pointer_space_: *mut ExternalPointerTableSpace,
    // cpp_heap_pointer_table_: *mut CppHeapPointerTable,
    // TODO: Implement V8_ENABLE_SANDBOX feature
    // trusted_pointer_table_: *mut TrustedPointerTable,
    // shared_trusted_pointer_table_: *mut SharedTrustedPointerTable,
    key_to_values_: *mut KeyToValues,
    _phantom: PhantomData<ConcreteVisitor>,
}

impl<ConcreteVisitor> MarkingVisitorBase<ConcreteVisitor> {
    pub fn new(
        local_marking_worklists: *mut MarkingWorklistsLocal,
        local_weak_objects: *mut WeakObjectsLocal,
        heap: *mut Heap,
        mark_compact_epoch: u32,
        code_flush_mode: EnumSet<CodeFlushMode>,
        should_keep_ages_unchanged: bool,
        code_flushing_increase: u16,
    ) -> Self {
        unsafe {
        let isolate = (*heap).isolate();
        MarkingVisitorBase {
            base: ConcurrentHeapVisitor::new(isolate),
            local_marking_worklists_: local_marking_worklists,
            local_weak_objects_: local_weak_objects,
            heap_: heap,
            mark_compact_epoch_: mark_compact_epoch,
            code_flush_mode_: code_flush_mode,
            should_keep_ages_unchanged_: should_keep_ages_unchanged,
            code_flushing_increase_: code_flushing_increase,
            isolate_in_background_: (*isolate).is_backgrounded(),
            // external_pointer_table_: &heap.isolate().external_pointer_table(),
            // shared_external_pointer_table_: &heap.isolate().shared_external_pointer_table(),
            // shared_external_pointer_space_: heap.isolate().shared_external_pointer_space(),
            // cpp_heap_pointer_table_: &heap.isolate().cpp_heap_pointer_table(),
            // trusted_pointer_table_: &heap.isolate().trusted_pointer_table(),
            // shared_trusted_pointer_table_: &heap.isolate().shared_trusted_pointer_table(),
            key_to_values_: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }
    }

    // V8_INLINE size_t VisitDescriptorArrayStrongly(Tagged<Map> map, Tagged<DescriptorArray> object, MaybeObjectSize);
    pub fn visit_descriptor_array_strongly(&self, _map: TaggedMap, _object: TaggedDescriptorArray, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitDescriptorArray(Tagged<Map> map, Tagged<DescriptorArray> object, MaybeObjectSize);
    pub fn visit_descriptor_array(&self, _map: TaggedMap, _object: TaggedDescriptorArray, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitEphemeronHashTable(Tagged<Map> map, Tagged<EphemeronHashTable> object, MaybeObjectSize);
    pub fn visit_ephemeron_hash_table(&self, _map: TaggedMap, _object: TaggedEphemeronHashTable, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitFixedArray(Tagged<Map> map, Tagged<FixedArray> object, MaybeObjectSize);
    pub fn visit_fixed_array(&self, _map: TaggedMap, _object: TaggedFixedArray, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitJSArrayBuffer(Tagged<Map> map, Tagged<JSArrayBuffer> object, MaybeObjectSize);
    pub fn visit_js_array_buffer(&self, _map: TaggedMap, _object: TaggedJSArrayBuffer, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitJSFunction(Tagged<Map> map, Tagged<JSFunction> object, MaybeObjectSize);
    pub fn visit_js_function(&self, _map: TaggedMap, _object: TaggedJSFunction, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitJSWeakRef(Tagged<Map> map, Tagged<JSWeakRef> object, MaybeObjectSize);
    pub fn visit_js_weak_ref(&self, _map: TaggedMap, _object: TaggedJSWeakRef, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitMap(Tagged<Map> map, Tagged<Map> object, MaybeObjectSize);
    pub fn visit_map(&self, _map: TaggedMap, _object: TaggedMap, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitSharedFunctionInfo(Tagged<Map> map, Tagged<SharedFunctionInfo> object, MaybeObjectSize);
    pub fn visit_shared_function_info(&self, _map: TaggedMap, _object: TaggedSharedFunctionInfo, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitTransitionArray(Tagged<Map> map, Tagged<TransitionArray> object, MaybeObjectSize);
    pub fn visit_transition_array(&self, _map: TaggedMap, _object: TaggedTransitionArray, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // V8_INLINE size_t VisitWeakCell(Tagged<Map> map, Tagged<WeakCell> object, MaybeObjectSize);
    pub fn visit_weak_cell(&self, _map: TaggedMap, _object: TaggedWeakCell, _maybe_object_size: MaybeObjectSize) -> usize {
        0 // Placeholder
    }

    // ObjectVisitor overrides.
    // void VisitMapPointer(Tagged<HeapObject> host) final {
    //   Tagged<Map> map = host->map(ObjectVisitorWithCageBases::cage_base());
    //   ProcessStrongHeapObject(host, host->map_slot(), map);
    // }
    pub fn visit_map_pointer(&self, host: TaggedHeapObject) {
        unimplemented!()
        //let map = host.map(ObjectVisitorWithCageBases::cage_base());
        //self.process_strong_heap_object(host, host.map_slot(), map);
    }

    // V8_INLINE void VisitPointer(Tagged<HeapObject> host, ObjectSlot p) final {
    //   VisitPointersImpl(host, p, p + 1);
    // }
    pub fn visit_pointer(&self, host: TaggedHeapObject, p: ObjectSlot) {
        self.visit_pointers_impl(host, p, unsafe { p.offset(1) });
    }

    // V8_INLINE void VisitPointer(Tagged<HeapObject> host,
    //                             MaybeObjectSlot p) final {
    //   VisitPointersImpl(host, p, p + 1);
    // }
    pub fn visit_pointer_maybe(&self, host: TaggedHeapObject, p: MaybeObjectSlot) {
        self.visit_pointers_impl(host, p, unsafe { p.offset(1) });
    }

    // V8_INLINE void VisitPointers(Tagged<HeapObject> host, ObjectSlot start,
    //                            ObjectSlot end) final {
    //   VisitPointersImpl(host, start, end);
    // }
    pub fn visit_pointers(&self, host: TaggedHeapObject, start: ObjectSlot, end: ObjectSlot) {
        self.visit_pointers_impl(host, start, end);
    }

    // V8_INLINE void VisitPointers(Tagged<HeapObject> host, MaybeObjectSlot start,
    //                            MaybeObjectSlot end) final {
    //   VisitPointersImpl(host, start, end);
    // }
    pub fn visit_pointers_maybe(&self, host: TaggedHeapObject, start: MaybeObjectSlot, end: MaybeObjectSlot) {
        self.visit_pointers_impl(host, start, end);
    }

    // V8_INLINE void VisitInstructionStreamPointer(
    //   Tagged<Code> host, InstructionStreamSlot slot) final {
    //   VisitStrongPointerImpl(host, slot);
    // }
    pub fn visit_instruction_stream_pointer(&self, host: TaggedCode, slot: InstructionStreamSlot) {
        self.visit_strong_pointer_impl(host, slot);
    }

    // V8_INLINE void VisitEmbeddedPointer(Tagged<InstructionStream> host,
    //                                   RelocInfo* rinfo) final;
    pub fn visit_embedded_pointer(&self, _host: TaggedInstructionStream, _rinfo: *mut RelocInfo) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE void VisitCodeTarget(Tagged<InstructionStream> host,
    //                                 RelocInfo* rinfo) final;
    pub fn visit_code_target(&self, _host: TaggedInstructionStream, _rinfo: *mut RelocInfo) {
        // Implementation details missing, placeholder
    }

    // void VisitCustomWeakPointers(Tagged<HeapObject> host, ObjectSlot start,
    //                            ObjectSlot end) final {
    //   // Weak list pointers should be ignored during marking. The lists are
    //   // reconstructed after GC.
    // }
    pub fn visit_custom_weak_pointers(&self, _host: TaggedHeapObject, _start: ObjectSlot, _end: ObjectSlot) {}

    // V8_INLINE void VisitExternalPointer(Tagged<HeapObject> host,
    //                                   ExternalPointerSlot slot) override;
    pub fn visit_external_pointer(&self, _host: TaggedHeapObject, _slot: ExternalPointerSlot) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE void VisitCppHeapPointer(Tagged<HeapObject> host,
    //                                  CppHeapPointerSlot slot) override;
    pub fn visit_cpp_heap_pointer(&self, _host: TaggedHeapObject, _slot: CppHeapPointerSlot) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE void VisitIndirectPointer(Tagged<HeapObject> host,
    //                                   IndirectPointerSlot slot,
    //                                   IndirectPointerMode mode) final;
    pub fn visit_indirect_pointer(&self, _host: TaggedHeapObject, _slot: IndirectPointerSlot, _mode: IndirectPointerMode) {
        // Implementation details missing, placeholder
    }

    // void VisitTrustedPointerTableEntry(Tagged<HeapObject> host,
    //                                  IndirectPointerSlot slot) final;
    pub fn visit_trusted_pointer_table_entry(&self, _host: TaggedHeapObject, _slot: IndirectPointerSlot) {
        // Implementation details missing, placeholder
    }

    // void VisitJSDispatchTableEntry(Tagged<HeapObject> host,
    //                                JSDispatchHandle handle) override;
    pub fn visit_js_dispatch_table_entry(&self, _host: TaggedHeapObject, _handle: JSDispatchHandle) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE void VisitProtectedPointer(Tagged<TrustedObject> host,
    //                                    ProtectedPointerSlot slot) final {
    //   VisitStrongPointerImpl(host, slot);
    // }
    pub fn visit_protected_pointer(&self, host: TaggedTrustedObject, slot: ProtectedPointerSlot) {
        self.visit_strong_pointer_impl(host, slot);
    }

    // V8_INLINE void VisitProtectedPointer(Tagged<TrustedObject> host,
    //                                    ProtectedMaybeObjectSlot slot) final {
    //   VisitPointersImpl(host, slot, slot + 1);
    // }
    pub fn visit_protected_pointer_maybe(&self, host: TaggedTrustedObject, slot: ProtectedMaybeObjectSlot) {
        self.visit_pointers_impl(host, slot, unsafe { slot.offset(1) });
    }

    pub fn synchronize_page_access(&self, _heap_object: TaggedHeapObject) {
        // #[cfg(THREAD_SANITIZER)]
        // // This is needed because TSAN does not process the memory fence
        // // emitted after page initialization.
        // MemoryChunk::from_heap_object(heap_object).synchronized_load();
    }

    // Marks the object  and pushes it on the marking work list. The `host` is
    // used for the reference summarizer to valide that the heap snapshot is in
    // sync with the marker.
    // V8_INLINE bool MarkObject(Tagged<HeapObject> host, Tagged<HeapObject> obj,
    //                           MarkingHelper::WorklistTarget target_worklist);
    pub fn mark_object(&self, _host: TaggedHeapObject, _obj: TaggedHeapObject, _target_worklist: MarkingHelperWorklistTarget) -> bool {
        false // Placeholder
    }

    // V8_INLINE static constexpr bool ShouldVisitReadOnlyMapPointer() {
    //   return false;
    // }
    pub const fn should_visit_read_only_map_pointer() -> bool {
        false
    }

    // V8_INLINE static constexpr bool CanEncounterFillerOrFreeSpace() {
    //   return false;
    // }
    pub const fn can_encounter_filler_or_free_space() -> bool {
        false
    }

    // V8_INLINE static constexpr bool IsTrivialWeakReferenceValue(
    //   Tagged<HeapObject> host, Tagged<HeapObject> heap_object);
    pub const fn is_trivial_weak_reference_value(_host: TaggedHeapObject, _heap_object: TaggedHeapObject) -> bool {
        false // Placeholder
    }

    pub fn set_key_to_values(&mut self, key_to_values: *mut KeyToValues) {
        debug_assert!(self.key_to_values_.is_null());
        self.key_to_values_ = key_to_values;
    }

    // template <typename THeapObjectSlot>
    // void ProcessStrongHeapObject(Tagged<HeapObject> host, THeapObjectSlot slot,
    //                             Tagged<HeapObject> heap_object);
    fn process_strong_heap_object<THeapObjectSlot>(&self, _host: TaggedHeapObject, _slot: THeapObjectSlot, _heap_object: TaggedHeapObject) {
        // Implementation details missing, placeholder
    }

    // template <typename THeapObjectSlot>
    // void ProcessWeakHeapObject(Tagged<HeapObject> host, THeapObjectSlot slot,
    //                            Tagged<HeapObject> heap_object);
    fn process_weak_heap_object<THeapObjectSlot>(&self, _host: TaggedHeapObject, _slot: THeapObjectSlot, _heap_object: TaggedHeapObject) {
        // Implementation details missing, placeholder
    }

    // template <typename TSlot>
    // V8_INLINE void VisitPointersImpl(Tagged<HeapObject> host, TSlot start,
    //                                TSlot end);
    fn visit_pointers_impl<TSlot>(&self, _host: TaggedHeapObject, _start: TSlot, _end: TSlot) {
        // Implementation details missing, placeholder
    }

    // template <typename TSlot>
    // V8_INLINE void VisitStrongPointerImpl(Tagged<HeapObject> host, TSlot slot);
    fn visit_strong_pointer_impl<TSlot>(&self, _host: TaggedHeapObject, _slot: TSlot) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE void VisitDescriptorsForMap(Tagged<Map> map);
    fn visit_descriptors_for_map(&self, _map: TaggedMap) {
        // Implementation details missing, placeholder
    }

    // V8_INLINE size_t VisitFixedArrayWithProgressTracker(Tagged<Map> map, Tagged<FixedArray> object,
    //                                  MarkingProgressTracker& progress_tracker);
    fn visit_fixed_array_with_progress_tracker(&self, _map: TaggedMap, _object: TaggedFixedArray, _progress_tracker: &mut MarkingProgressTracker) -> usize {
        0 // Placeholder
    }

    // Methods needed for supporting code flushing.
    // bool ShouldFlushCode(Tagged<SharedFunctionInfo> sfi) const;
    fn should_flush_code(&self, _sfi: TaggedSharedFunctionInfo) -> bool {
        false // Placeholder
    }

    // bool ShouldFlushBaselineCode(Tagged<JSFunction> js_function) const;
    fn should_flush_baseline_code(&self, _js_function: TaggedJSFunction) -> bool {
        false // Placeholder
    }

    // bool HasBytecodeArrayForFlushing(Tagged<SharedFunctionInfo> sfi) const;
    fn has_bytecode_array_for_flushing(&self, _sfi: TaggedSharedFunctionInfo) -> bool {
        false // Placeholder
    }

    // bool IsOld(Tagged<SharedFunctionInfo> sfi) const;
    fn is_old(&self, _sfi: TaggedSharedFunctionInfo) -> bool {
        false // Placeholder
    }

    // void MakeOlder(Tagged<SharedFunctionInfo> sfi) const;
    fn make_older(&self, _sfi: TaggedSharedFunctionInfo) {
        // Implementation details missing, placeholder
    }
}

// This is the common base class for main and concurrent full marking visitors.
// Derived class are expected to provide the same methods as for
// MarkingVisitorBase except for those defined in this class.
pub struct FullMarkingVisitorBase<ConcreteVisitor> {
    base: MarkingVisitorBase<ConcreteVisitor>,
    marking_state_: *mut MarkingState, // Placeholder
    _phantom: PhantomData<ConcreteVisitor>,
}

impl<ConcreteVisitor> FullMarkingVisitorBase<ConcreteVisitor> {
    pub fn new(
        local_marking_worklists: *mut MarkingWorklistsLocal,
        local_weak_objects: *mut WeakObjectsLocal,
        heap: *mut Heap,
        mark_compact_epoch: u32,
        code_flush_mode: EnumSet<CodeFlushMode>,
        should_keep_ages_unchanged: bool,
        code_flushing_increase: u16,
    ) -> Self {
        unsafe {
            let marking_state_ = (*heap).marking_state();
            FullMarkingVisitorBase {
                base: MarkingVisitorBase::new(
                    local_marking_worklists,
                    local_weak_objects,
                    heap,
                    mark_compact_epoch,
                    code_flush_mode,
                    should_keep_ages_unchanged,
                    code_flushing_increase,
                ),
                marking_state_: marking_state_,
                _phantom: PhantomData,
            }
        }
    }

    // V8_INLINE void AddStrongReferenceForReferenceSummarizer(
    //     Tagged<HeapObject> host, Tagged<HeapObject> obj) {}
    pub fn add_strong_reference_for_reference_summarizer(&self, _host: TaggedHeapObject, _obj: TaggedHeapObject) {}

    // V8_INLINE void AddWeakReferenceForReferenceSummarizer(
    //     Tagged<HeapObject> host, Tagged<HeapObject> obj) {}
    pub fn add_weak_reference_for_reference_summarizer(&self, _host: TaggedHeapObject, _obj: TaggedHeapObject) {}

    // constexpr bool CanUpdateValuesInHeap() { return true; }
    pub const fn can_update_values_in_heap(&self) -> bool {
        true
    }

    // MarkingState* marking_state() const { return marking_state_; }
    pub fn marking_state(&self) -> *mut MarkingState {
        self.marking_state_
    }

    // void MarkPointerTableEntry(Tagged<HeapObject> obj, IndirectPointerSlot slot);
    pub fn mark_pointer_table_entry(&self, _obj: TaggedHeapObject, _slot: IndirectPointerSlot) {
        // Implementation details missing, placeholder
    }
}

// Placeholder types. Replace with actual definitions.
pub struct TaggedHeapObject {}
pub struct TaggedMap {}
pub struct TaggedDescriptorArray {}
pub struct TaggedEphemeronHashTable {}
pub struct TaggedFixedArray {}
pub struct TaggedJSArrayBuffer {}
pub struct TaggedJSFunction {}
pub struct TaggedJSWeakRef {}
pub struct TaggedSharedFunctionInfo {}
pub struct TaggedTransitionArray {}
pub struct TaggedWeakCell {}
pub struct TaggedCode {}
pub struct TaggedInstructionStream {}
pub struct TaggedTrustedObject {}

pub struct ObjectSlot {}
pub struct MaybeObjectSlot {}
pub struct InstructionStreamSlot {}
pub struct RelocInfo {}
pub struct ExternalPointerSlot {}
pub struct CppHeapPointerSlot {}
pub struct IndirectPointerSlot {}
pub struct JSDispatchHandle {}
pub struct ProtectedPointerSlot {}
pub struct ProtectedMaybeObjectSlot {}
pub struct MarkingProgressTracker {}

pub enum IndirectPointerMode {}
pub enum CodeFlushMode {}

pub struct Heap {}
pub struct MarkingWorklistsLocal {}
pub struct WeakObjectsLocal {}
pub struct ExternalPointerTable {}
pub struct ExternalPointerTableSpace {}
pub struct CppHeapPointerTable {}
pub struct TrustedPointerTable {}
pub struct SharedTrustedPointerTable {}
pub struct EnumSet<T> {}
pub struct MarkingState {}

pub enum MarkingHelperWorklistTarget {}

pub struct Isolate {}

impl Isolate {
    pub fn is_backgrounded(&self) -> bool {
        false
    }
}
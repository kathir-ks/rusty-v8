// Converted from V8 C++ source files:
// Header: marking-visitor-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::{Arc, Mutex, RwLock};
use std::ops::Range;
use crate::v8::*;
use crate::heap::ephemeron_remembered_set::*;
use crate::heap::heap_layout_inl::*;
use crate::heap::heap_visitor_inl::*;
use crate::heap::heap_visitor::*;
use crate::heap::marking_progress_tracker::*;
use crate::heap::marking_state_inl::*;
use crate::heap::marking_visitor::*;
use crate::heap::marking_worklist_inl::*;
use crate::heap::marking::*;
use crate::heap::pretenuring_handler_inl::*;
use crate::heap::spaces::*;
use crate::objects::compressed_slots::*;
use crate::objects::descriptor_array::*;
use crate::objects::heap_object::*;
use crate::objects::js_objects::*;
use crate::objects::objects::*;
use crate::objects::property_details::*;
use crate::objects::slots::*;
use crate::objects::smi::*;
use crate::objects::string::*;
use crate::sandbox::external_pointer_inl::*;
use crate::sandbox::indirect_pointer_tag::*;
use crate::sandbox::js_dispatch_table_inl::*;

use std::convert::Infallible;
use std::io::Write;
use crate::compiler::turboshaft::common::*;
use crate::heap::stress_scavenge_observer::*;
use crate::objects::deoptimization_data::*;
use crate::ast::source_range_ast_visitor::*;
use crate::objects::js_regexp_inl::*;
use crate::sandbox::code_pointer_table::*;
use crate::objects::js_date_time_format_inl::*;
use crate::objects::property_cell_inl::*;
use crate::runtime::runtime_regexp::*;
use crate::heap::safepoint::*;
use crate::heap::young_generation_marking_visitor_inl::*;
use crate::objects::fixed_array::*;
use crate::codegen::callable::*;
use crate::interpreter::bytecode_register_optimizer::*;
use crate::codegen::riscv::base_riscv_i::*;
use crate::heap::scavenger_inl::*;
use crate::objects::string_set::*;
use crate::objects::swiss_hash_table_helpers::*;
use crate::heap::new_spaces::*;
use crate::codegen::interface_descriptors::*;
use crate::regexp::arm::regexp_macro_assembler_arm::*;

pub mod object_macros {

}

trait MarkingVisitorInterface {
    fn add_strong_reference_for_reference_summarizer(&mut self, retainer: Tagged<HeapObject>, object: Tagged<HeapObject>);
    fn add_weak_reference_for_reference_summarizer(&mut self, host: Tagged<HeapObject>, heap_object: Tagged<HeapObject>);
    fn record_slot<THeapObjectSlot>(&mut self, host: Tagged<HeapObject>, slot: THeapObjectSlot, heap_object: Tagged<HeapObject>);
    fn record_reloc_slot(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo, object: Tagged<HeapObject>);
    fn can_update_values_in_heap(&self) -> bool;
    fn marking_state(&mut self) -> &mut MarkingState;
    fn template_visit_map_pointer_if_needed<const VISITOR_ID: VisitorId>(&mut self, object: Tagged<HeapObject>);
    fn mark_pointer_table_entry(&mut self, host: Tagged<HeapObject>, slot: IndirectPointerSlot);
}

struct MarkingVisitorBaseFields {
    heap_: *mut Heap,
    local_marking_worklists_: *mut LocalMarkingWorklists,
    local_weak_objects_: *mut LocalWeakObjects,
    code_flush_mode_: CodeFlushMode,
    isolate_in_background_: bool,
    should_keep_ages_unchanged_: bool,
    mark_compact_epoch_: usize,
    code_flushing_increase_: u16,
    external_pointer_table_: *mut ExternalPointerTable,
    shared_external_pointer_table_: *mut ExternalPointerTable,
    shared_external_pointer_space_: *mut ExternalPointerTable::Space,
    cpp_heap_pointer_table_: *mut CppHeapPointerTable,
    trusted_pointer_table_: *mut TrustedPointerTable,
    shared_trusted_pointer_table_: *mut TrustedPointerTable,
    key_to_values_: *mut std::collections::HashMap<Tagged<HeapObject>, Vec<Tagged<HeapObject>>>,
}

impl MarkingVisitorBaseFields {
    fn new(
        heap_: *mut Heap,
        local_marking_worklists_: *mut LocalMarkingWorklists,
        local_weak_objects_: *mut LocalWeakObjects,
        code_flush_mode_: CodeFlushMode,
        isolate_in_background_: bool,
        should_keep_ages_unchanged_: bool,
        mark_compact_epoch_: usize,
        code_flushing_increase_: u16,
        external_pointer_table_: *mut ExternalPointerTable,
        shared_external_pointer_table_: *mut ExternalPointerTable,
        shared_external_pointer_space_: *mut ExternalPointerTable::Space,
        cpp_heap_pointer_table_: *mut CppHeapPointerTable,
        trusted_pointer_table_: *mut TrustedPointerTable,
        shared_trusted_pointer_table_: *mut TrustedPointerTable,
        key_to_values_: *mut std::collections::HashMap<Tagged<HeapObject>, Vec<Tagged<HeapObject>>>,
    ) -> Self {
        Self {
            heap_,
            local_marking_worklists_,
            local_weak_objects_,
            code_flush_mode_,
            isolate_in_background_,
            should_keep_ages_unchanged_,
            mark_compact_epoch_,
            code_flushing_increase_,
            external_pointer_table_,
            shared_external_pointer_table_,
            shared_external_pointer_space_,
            cpp_heap_pointer_table_,
            trusted_pointer_table_,
            shared_trusted_pointer_table_,
            key_to_values_,
        }
    }
}

struct MarkingVisitorBase<ConcreteVisitor> {
    fields: MarkingVisitorBaseFields,
    concrete_visitor: ConcreteVisitor,
}

impl<ConcreteVisitor> MarkingVisitorBase<ConcreteVisitor> {
    fn new(fields: MarkingVisitorBaseFields, concrete_visitor: ConcreteVisitor) -> Self {
        Self {
            fields,
            concrete_visitor,
        }
    }

    fn heap(&self) -> &Heap {
        unsafe { &*self.fields.heap_ }
    }

    fn concrete_visitor(&mut self) -> &mut ConcreteVisitor {
        &mut self.concrete_visitor
    }

    fn synchronize_page_access(&self, object: Tagged<HeapObject>) {
        unsafe { MemoryChunk::FromHeapObject(object).SynchronizedLoad(); }
    }

    fn is_free_space_or_filler(&self, object: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        IsFreeSpace(object) || IsFiller(object, cage_base)
    }

    fn get_object_filter_read_only_and_smi_fast<TSlot>(&self, slot: TSlot) -> Option<Tagged<Object>>
        where TSlot: SlotTrait
    {
        let object = slot.load_relaxed();
        if object.is_smi() || HeapLayout::InReadOnlySpace(object) {
            return None;
        }
        Some(object)
    }
}

impl<ConcreteVisitor: MarkingVisitorInterface> MarkingVisitorBase<ConcreteVisitor> {
    fn mark_object(
        &mut self,
        retainer: Tagged<HeapObject>,
        object: Tagged<HeapObject>,
        target_worklist: MarkingHelper::WorklistTarget,
    ) -> bool {
        unsafe {
            DCHECK!((&*self.fields.heap_).contains(object));
            self.synchronize_page_access(object);
            self.concrete_visitor().add_strong_reference_for_reference_summarizer(retainer, object);
            MarkingHelper::try_mark_and_push(
                &*self.fields.heap_,
                self.fields.local_marking_worklists_,
                self.concrete_visitor().marking_state(),
                target_worklist,
                object,
            )
        }
    }

    fn process_strong_heap_object<THeapObjectSlot>(
        &mut self,
        host: Tagged<HeapObject>,
        slot: THeapObjectSlot,
        heap_object: Tagged<HeapObject>,
    ) where THeapObjectSlot: SlotTrait {
        self.synchronize_page_access(heap_object);
        let target_worklist = unsafe {
            MarkingHelper::should_mark_object(&*self.fields.heap_, heap_object)
        };

        if target_worklist.is_none() {
            return;
        }

        unsafe {
            if V8_UNLIKELY(!MemoryChunk::FromHeapObject(heap_object).IsMarking() &&
                self.is_free_space_or_filler(
                    heap_object, ObjectVisitorWithCageBases::cage_base()))) {
                let isolate = (&*self.fields.heap_).isolate();
                (&*isolate).push_stack_trace_and_die(
                    host.map().ptr() as *mut std::ffi::c_void,
                    host.address() as *mut std::ffi::c_void,
                    slot.address() as *mut std::ffi::c_void,
                    MemoryChunkMetadata::FromHeapObject(heap_object)
                        .owner()
                        .identity() as *mut std::ffi::c_void,
                );
            }
        }

        self.mark_object(host, heap_object, target_worklist.unwrap());
        self.concrete_visitor().record_slot(host, slot, heap_object);
    }

    const fn is_trivial_weak_reference_value(
        host: Tagged<HeapObject>,
        heap_object: Tagged<HeapObject>,
    ) -> bool {
        !IsMap(heap_object) ||
            !(IsMap(host) || IsTransitionArray(host) || IsDescriptorArray(host))
    }

    fn process_weak_heap_object<THeapObjectSlot>(
        &mut self,
        host: Tagged<HeapObject>,
        slot: THeapObjectSlot,
        heap_object: Tagged<HeapObject>,
    ) where THeapObjectSlot: SlotTrait {
        self.synchronize_page_access(heap_object);
        self.concrete_visitor().add_weak_reference_for_reference_summarizer(host, heap_object);

        let target_worklist = unsafe {
            MarkingHelper::should_mark_object(&*self.fields.heap_, heap_object)
        };
        if target_worklist.is_none() {
            return;
        }

        if self.concrete_visitor().marking_state().is_marked(heap_object) {
            self.concrete_visitor().record_slot(host, slot, heap_object);
        } else {
            if SlotHoldsTrustedPointerV::<THeapObjectSlot> {
                unsafe {
                    (&mut *self.fields.local_weak_objects_).weak_references_trusted_local.push(
                        TrustedObjectAndSlot { host, slot },
                    );
                }
            } else if Self::is_trivial_weak_reference_value(host, heap_object) {
                unsafe {
                    (&mut *self.fields.local_weak_objects_).weak_references_trivial_local.push(
                        HeapObjectAndSlot { host, slot },
                    );
                }
            } else {
                unsafe {
                    (&mut *self.fields.local_weak_objects_).weak_references_non_trivial_local.push(
                        HeapObjectAndSlot { host, slot },
                    );
                }
            }
        }
    }

    fn visit_pointers_impl<TSlot>(
        &mut self,
        host: Tagged<HeapObject>,
        start: TSlot,
        end: TSlot,
    ) where
        TSlot: SlotTrait,
    {
        for slot in start.to(end) {
            let object: Tagged<Object>;
            if SlotHoldsTrustedPointerV::<TSlot> {
                object = slot.relaxed_load();
            } else {
                let optional_object = self.get_object_filter_read_only_and_smi_fast(slot);
                if optional_object.is_none() {
                    continue;
                }
                object = optional_object.unwrap();
            }

            if let Some(heap_object) = object.get_heap_object_if_strong() {
                self.process_strong_heap_object(host, slot, heap_object);
            } else if TSlot::kCanBeWeak {
                if let Some(heap_object) = object.get_heap_object_if_weak() {
                    self.process_weak_heap_object(host, slot, heap_object);
                }
            }
        }
    }

    fn visit_strong_pointer_impl<TSlot>(&mut self, host: Tagged<HeapObject>, slot: TSlot)
        where TSlot: SlotTrait
    {
        let object = slot.relaxed_load();
        if let Some(heap_object) = object.get_heap_object() {
            self.process_strong_heap_object(host, slot, heap_object);
        }
    }

    fn visit_embedded_pointer(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
        unsafe {
            DCHECK!(RelocInfo::IsEmbeddedObjectMode((&*rinfo).rmode()));
            let object = (&*rinfo).target_object(ObjectVisitorWithCageBases::cage_base());
            let target_worklist = MarkingHelper::should_mark_object(&*self.fields.heap_, object);
            if target_worklist.is_none() {
                return;
            }

            if !self.concrete_visitor().marking_state().is_marked(object) {
                let code = UncheckedCast::<Code>(host.raw_code(kAcquireLoad));
                if code.is_weak_object(object) {
                    (&mut *self.fields.local_weak_objects_).weak_objects_in_code_local.push(
                        HeapObjectAndCode { object, code },
                    );
                    self.concrete_visitor().add_weak_reference_for_reference_summarizer(host, object);
                } else {
                    self.mark_object(host, object, target_worklist.unwrap());
                }
            }
            self.concrete_visitor().record_reloc_slot(host, rinfo, object);
        }
    }

    fn visit_code_target(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
        unsafe {
            DCHECK!(RelocInfo::IsCodeTargetMode((&*rinfo).rmode()));
            let target = InstructionStream::FromTargetAddress((&*rinfo).target_address());
            let target_worklist = MarkingHelper::should_mark_object(&*self.fields.heap_, target);
            if target_worklist.is_none() {
                return;
            }
            self.mark_object(host, target, target_worklist.unwrap());
            self.concrete_visitor().record_reloc_slot(host, rinfo, target);
        }
    }

    fn visit_external_pointer(&mut self, host: Tagged<HeapObject>, slot: ExternalPointerSlot) {
        #[cfg(V8_COMPRESS_POINTERS)]
        unsafe {
            DCHECK!(!slot.tag_range().is_empty());
            if slot.has_external_pointer_handle() {
                let handle = slot.relaxed_load_handle();
                let table: *mut ExternalPointerTable;
                let space: *mut ExternalPointerTable::Space;
                if IsSharedExternalPointerType(slot.tag_range()) {
                    table = self.fields.shared_external_pointer_table_;
                    space = self.fields.shared_external_pointer_space_;
                } else {
                    table = self.fields.external_pointer_table_;
                    if v8_flags.sticky_mark_bits {
                        DCHECK!(!HeapLayout::InYoungGeneration(host));
                        if handle == kNullExternalPointerHandle {
                            return;
                        }
                        if (&*table).contains((&*self.fields.heap_).young_external_pointer_space(), handle) {
                            space = (&*self.fields.heap_).young_external_pointer_space();
                        } else {
                            DCHECK!((&*table).contains((&*self.fields.heap_).old_external_pointer_space(), handle));
                            space = (&*self.fields.heap_).old_external_pointer_space();
                        }
                    } else {
                        space = if HeapLayout::InYoungGeneration(host) {
                            (&*self.fields.heap_).young_external_pointer_space()
                        } else {
                            (&*self.fields.heap_).old_external_pointer_space()
                        };
                    }
                }
                (&*table).mark((&mut *space), handle, slot.address());
            }
        }
    }

    fn visit_cpp_heap_pointer(&mut self, host: Tagged<HeapObject>, slot: CppHeapPointerSlot) {
        #[cfg(V8_COMPRESS_POINTERS)]
        unsafe {
            let handle = slot.relaxed_load_handle();
            if handle == kNullExternalPointerHandle {
                return;
            }
            let table = self.fields.cpp_heap_pointer_table_;
            let space = (&*self.fields.heap_).cpp_heap_pointer_space();
            (&*table).mark((&mut *space), handle, slot.address());
        }

        if let Some(cpp_heap_pointer) = unsafe {
            slot.try_load((&*self.fields.heap_).isolate(), kAnyCppHeapPointer)
        } {
            unsafe {
                (&mut *self.fields.local_marking_worklists_)
                    .cpp_marking_state()
                    .mark_and_push(cpp_heap_pointer as *mut std::ffi::c_void);
            }
        }
    }

    fn visit_indirect_pointer(
        &mut self,
        host: Tagged<HeapObject>,
        slot: IndirectPointerSlot,
        mode: IndirectPointerMode,
    ) {
        #[cfg(V8_ENABLE_SANDBOX)]
        if mode == IndirectPointerMode::kStrong {
            unsafe {
                let value = slot.relaxed_load_allow_unpublished((&*self.fields.heap_).isolate());
                if IsHeapObject(value) {
                    let obj = Cast::<HeapObject>(value);
                    self.synchronize_page_access(obj);
                    let target_worklist =
                        MarkingHelper::should_mark_object(&*self.fields.heap_, obj);
                    if target_worklist.is_none() {
                        return;
                    }
                    self.mark_object(host, obj, target_worklist.unwrap());
                }
            }
        }
    }

    fn visit_trusted_pointer_table_entry(&mut self, host: Tagged<HeapObject>, slot: IndirectPointerSlot) {
        self.concrete_visitor().mark_pointer_table_entry(host, slot);
    }

    fn visit_js_dispatch_table_entry(&mut self, host: Tagged<HeapObject>, handle: JSDispatchHandle) {
        #[cfg(V8_ENABLE_LEAPTIERING)]
        unsafe {
            let jdt = IsolateGroup::current().js_dispatch_table();
            (&*jdt).mark(handle);
        }
    }

    fn visit_js_function(
        &mut self,
        map: Tagged<Map>,
        js_function: Tagged<JSFunction>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        if self.should_flush_baseline_code(js_function) {
            unsafe {
                if !V8_ENABLE_LEAPTIERING {
                    (&mut *self.fields.local_weak_objects_)
                        .baseline_flushing_candidates_local
                        .push(js_function);
                }
            }
            return self.concrete_visitor().visit_js_function(map, js_function, maybe_object_size);
        }

        #[cfg(V8_ENABLE_LEAPTIERING)]
        unsafe {
            let handle = js_function.relaxed_read_field::<JSDispatchHandle::underlying_type>(
                JSFunction::kDispatchHandleOffset,
            );
            if handle != kNullJSDispatchHandle {
                let obj = IsolateGroup::current().js_dispatch_table().get_code(handle);
                self.synchronize_page_access(obj);
                let target_worklist =
                    MarkingHelper::should_mark_object(&*self.fields.heap_, obj);
                if let Some(target_worklist) = target_worklist {
                    self.mark_object(js_function, obj, target_worklist);
                }
            }
        }

        #[cfg(not(V8_ENABLE_LEAPTIERING))]
        unsafe {
            #[cfg(V8_ENABLE_SANDBOX)]
            self.visit_indirect_pointer(
                js_function,
                js_function.raw_indirect_pointer_field(
                    JSFunction::kCodeOffset,
                    kCodeIndirectPointerTag,
                ),
                IndirectPointerMode::kStrong,
            );
            #[cfg(not(V8_ENABLE_SANDBOX))]
            self.visit_pointer(js_function, js_function.raw_field(JSFunction::kCodeOffset));
        }

        if self.is_byte_code_flushing_enabled() &&
            js_function.needs_reset_due_to_flushed_bytecode((unsafe { &*self.fields.heap_ }).isolate()) {
            unsafe {
                (&mut *self.fields.local_weak_objects_).flushed_js_functions_local.push(js_function);
            }
        }

        self.concrete_visitor().visit_js_function(map, js_function, maybe_object_size)
    }

    fn visit_shared_function_info(
        &mut self,
        map: Tagged<Map>,
        shared_info: Tagged<SharedFunctionInfo>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        let can_flush_bytecode = self.has_bytecode_array_for_flushing(shared_info);

        if can_flush_bytecode && !self.fields.should_keep_ages_unchanged_ {
            self.make_older(shared_info);
        }

        if !can_flush_bytecode || !self.should_flush_code(shared_info) {
            unsafe {
                #[cfg(V8_ENABLE_SANDBOX)]
                self.visit_indirect_pointer(
                    shared_info,
                    shared_info.raw_indirect_pointer_field(
                        SharedFunctionInfo::kTrustedFunctionDataOffset,
                        kUnknownIndirectPointerTag,
                    ),
                    IndirectPointerMode::kStrong,
                );
                #[cfg(not(V8_ENABLE_SANDBOX))]
                self.visit_pointer(
                    shared_info,
                    shared_info.raw_field(SharedFunctionInfo::kTrustedFunctionDataOffset),
                );
                self.visit_pointer(
                    shared_info,
                    shared_info.raw_field(SharedFunctionInfo::kUntrustedFunctionDataOffset),
                );
            }
        } else if !self.is_byte_code_flushing_enabled() {
            unsafe {
                DCHECK!(self.is_baseline_code_flushing_enabled());
                let baseline_code = shared_info.baseline_code(kAcquireLoad);
                self.visit_protected_pointer(
                    baseline_code,
                    baseline_code.raw_protected_pointer_field(
                        Code::kDeoptimizationDataOrInterpreterDataOffset,
                    ),
                );
                (&mut *self.fields.local_weak_objects_)
                    .code_flushing_candidates_local
                    .push(shared_info);
            }
        } else {
            unsafe {
                (&mut *self.fields.local_weak_objects_)
                    .code_flushing_candidates_local
                    .push(shared_info);
            }
        }

        self.concrete_visitor().visit_shared_function_info(map, shared_info, maybe_object_size)
    }

    fn has_bytecode_array_for_flushing(&self, sfi: Tagged<SharedFunctionInfo>) -> bool {
        if self.is_flushing_disabled() {
            return false;
        }

        if IsResumableFunction(sfi.kind()) || !sfi.allows_lazy_compilation() {
            return false;
        }

        let data = unsafe { sfi.get_trusted_data((&*self.fields.heap_).isolate()) };
        if IsCode(data) {
            let baseline_code = Cast::<Code>(data);
            DCHECK_EQ!(baseline_code.kind(), CodeKind::BASELINE);
            if !self.is_baseline_code_flushing_enabled() {
                return false;
            }
            data = baseline_code.bytecode_or_interpreter_data();
        } else if !self.is_byte_code_flushing_enabled() {
            return false;
        }

        IsBytecodeArray(data)
    }

    fn should_flush_code(&self, sfi: Tagged<SharedFunctionInfo>) -> bool {
        self.is_old(sfi) || V8_UNLIKELY(self.is_force_flushing_enabled())
    }

    fn is_old(&self, sfi: Tagged<SharedFunctionInfo>) -> bool {
        if v8_flags.flush_code_based_on_time {
            sfi.age() >= v8_flags.bytecode_old_time
        } else if v8_flags.flush_code_based_on_tab_visibility {
            self.fields.isolate_in_background_ ||
                V8_UNLIKELY(sfi.age() == SharedFunctionInfo::kMaxAge)
        } else {
            sfi.age() >= v8_flags.bytecode_old_age
        }
    }

    fn make_older(&self, sfi: Tagged<SharedFunctionInfo>) {
        if v8_flags.flush_code_based_on_time {
            if self.fields.code_flushing_increase_ == 0 {
                return;
            }

            let mut current_age: u16;
            let mut updated_age: u16;
            unsafe {
                loop {
                    current_age = sfi.age();
                    updated_age = if current_age == 0 {
                        1
                    } else {
                        let sum = current_age as u32 + self.fields.code_flushing_increase_ as u32;
                        if sum > u16::MAX as u32 {
                            u16::MAX
                        } else {
                            sum as u16
                        }
                    };

                    if sfi.compare_exchange_age(current_age, updated_age) == current_age {
                        break;
                    }
                }
            }
        } else if v8_flags.flush_code_based_on_tab_visibility {
        } else {
            unsafe {
                let age = sfi.age();
                if age < v8_flags.bytecode_old_age {
                    sfi.compare_exchange_age(age, age + 1);
                }
                DCHECK_LE!(sfi.age(), v8_flags.bytecode_old_age);
            }
        }
    }

    fn should_flush_baseline_code(&self, js_function: Tagged<JSFunction>) -> bool {
        if !self.is_baseline_code_flushing_enabled() {
            return false;
        }

        unsafe {
            let maybe_shared =
                ACQUIRE_READ_FIELD!(js_function, JSFunction::kSharedFunctionInfoOffset);
            if !IsSharedFunctionInfo(maybe_shared) {
                return false;
            }

            let maybe_code =
                js_function.raw_code((&*self.fields.heap_).isolate(), kAcquireLoad);
            if !IsCode(maybe_code) {
                return false;
            }
            let code = Cast::<Code>(maybe_code);
            if code.kind() != CodeKind::BASELINE {
                return false;
            }

            let shared = Cast::<SharedFunctionInfo>(maybe_shared);
            return self.has_bytecode_array_for_flushing(shared) && self.should_flush_code(shared);
        }
    }

    fn visit_fixed_array_with_progress_tracker(
        &mut self,
        map: Tagged<Map>,
        object: Tagged<FixedArray>,
        progress_tracker: &mut MarkingProgressTracker,
    ) -> usize {
        static_assert!(kMaxRegularHeapObjectSize % kTaggedSize == 0);
        const K_MAX_QUEUED_WORKLIST_ITEMS: usize = 8;
        unsafe {
            DCHECK!(self.concrete_visitor().marking_state().is_marked(object));

            let size = FixedArray::BodyDescriptor::SizeOf(map, object);
            let chunk = progress_tracker.get_next_chunk_to_mark();
            let total_chunks = progress_tracker.total_number_of_chunks();
            let mut start: usize = 0;
            let mut end: usize = 0;
            if chunk == 0 {
                if let Some(target_worklist) =
                    MarkingHelper::should_mark_object(&*self.fields.heap_, object)
                {
                    DCHECK_EQ!(target_worklist, MarkingHelper::WorklistTarget::kRegular);
                    let scheduled_chunks =
                        std::cmp::min(total_chunks, K_MAX_QUEUED_WORKLIST_ITEMS);
                    DCHECK_GT!(scheduled_chunks, 0);
                    for i in 1..scheduled_chunks {
                        (&mut *self.fields.local_marking_worklists_).push(object);
                        (&mut *self.fields.local_marking_worklists_).share_work();
                    }
                }
                self.concrete_visitor()
                    .template_visit_map_pointer_if_needed::<VisitorId::kVisitFixedArray>(object);
                start = FixedArray::BodyDescriptor::kStartOffset;
                end = std::cmp::min(size, MarkingProgressTracker::K_CHUNK_SIZE);
            } else {
                start = chunk * MarkingProgressTracker::K_CHUNK_SIZE;
                end = std::cmp::min(size, start + MarkingProgressTracker::K_CHUNK_SIZE);
            }

            if chunk + K_MAX_QUEUED_WORKLIST_ITEMS < total_chunks {
                if let Some(_target_worklist) =
                    MarkingHelper::should_mark_object(&*self.fields.heap_, object)
                {
                    (&mut *self.fields.local_marking_worklists_).push(object);
                    (&mut *self.fields.local_marking_worklists_).share_work();
                }
            }

            if start < end {
                self.visit_pointers(
                    object,
                    Cast::<HeapObject>(object).raw_field(start as i32),
                    Cast::<HeapObject>(object).raw_field(end as i32),
                );
            }

            return end - start;
        }
    }

    fn visit_fixed_array(
        &mut self,
        map: Tagged<Map>,
        object: Tagged<FixedArray>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        let progress_tracker = unsafe {
            MutablePageMetadata::FromHeapObject(object).marking_progress_tracker()
        };
        if self.concrete_visitor().can_update_values_in_heap() && progress_tracker.is_enabled() {
            self.visit_fixed_array_with_progress_tracker(map, object, &mut progress_tracker)
        } else {
            self.concrete_visitor().visit_fixed_array(map, object, maybe_object_size)
        }
    }

    fn visit_js_array_buffer(
        &mut self,
        map: Tagged<Map>,
        object: Tagged<JSArrayBuffer>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        object.mark_extension();
        self.concrete_visitor().visit_js_array_buffer(map, object, maybe_object_size)
    }

    fn visit_ephemeron_hash_table(
        &mut self,
        map: Tagged<Map>,
        table: Tagged<EphemeronHashTable>,
        _maybe_object_size: MaybeObjectSize,
    ) -> usize {
        unsafe {
            (&mut *self.fields.local_weak_objects_).ephemeron_hash_tables_local.push(table);
        }
        let use_key_to_values = self.fields.key_to_values_ != std::ptr::null_mut();

        for i in table.iterate_entries() {
            let key_slot = unsafe {
                table.raw_field_of_element_at(EphemeronHashTable::EntryToIndex(i))
            };
            let key = Cast::<HeapObject>(unsafe {table.key_at(i, kRelaxedLoad)});

            self.synchronize_page_access(key);
            self.concrete_visitor().record_slot(table, key_slot, key);
            self.concrete_visitor().add_weak_reference_for_reference_summarizer(table, key);

            let value_slot = unsafe {
                table.raw_field_of_element_at(EphemeronHashTable::EntryToValueIndex(i))
            };

            DCHECK!(!HeapLayout::InWritableSharedSpace(key));
            if unsafe {
                MarkingHelper::IsMarkedOrAlwaysLive(
                    &*self.fields.heap_,
                    self.concrete_visitor().marking_state(),
                    key,

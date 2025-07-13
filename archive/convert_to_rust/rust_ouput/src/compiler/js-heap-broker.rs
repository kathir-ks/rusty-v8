// Converted from V8 C++ source files:
// Header: js-heap-broker.h
// Implementation: js-heap-broker.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::base::compiler_specific::*;
use crate::base::macros::*;
use crate::codegen::optimized_compilation_info::*;
use crate::common::globals::*;
use crate::compiler::access_info::*;
use crate::compiler::feedback_source::*;
use crate::compiler::heap_refs::*;
use crate::compiler::processed_feedback::*;
use crate::compiler::refs_map::*;
use crate::execution::local_isolate::*;
use crate::handles::handles::*;
use crate::handles::persistent_handles::*;
use crate::heap::local_heap::*;
use crate::heap::parked_scope::*;
use crate::objects::code_kind::*;
use crate::objects::feedback_vector::*;
use crate::objects::objects::*;
use crate::objects::tagged::*;
use crate::roots::roots::*;
use crate::utils::address_map::*;
use crate::utils::identity_map::*;
use crate::utils::ostreams::*;
use crate::zone::zone_containers::*;
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::convert::TryInto;

pub mod maglev {
    pub struct MaglevCompilationInfo {}
}

pub mod compiler {
    use super::*;
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    pub struct ObjectRef {}

    impl std::fmt::Display for ObjectRef {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ObjectRef")
        }
    }

    pub fn operator_iostream(os: &mut OStream, ref_: ObjectRef) -> std::io::Result<()> {
        write!(os, "{}", ref_).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Write failed"))
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PropertyAccessTarget {
        pub map: MapRef,
        pub name: NameRef,
        pub mode: AccessMode,
    }

    impl PropertyAccessTarget {
        pub fn new(map: MapRef, name: NameRef, mode: AccessMode) -> Self {
            PropertyAccessTarget { map, name, mode }
        }
    }

    pub struct PropertyAccessTargetHash {
        phantom: PhantomData<PropertyAccessTarget>
    }

    impl PropertyAccessTargetHash {
        pub fn new() -> Self {
            PropertyAccessTargetHash{phantom: PhantomData}
        }
    }

    impl Default for PropertyAccessTargetHash {
        fn default() -> Self {
            Self::new()
        }
    }

    impl std::hash::Hasher for PropertyAccessTargetHash {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, bytes: &[u8]) {
             println!("{}", bytes.len());
        }
    }

    impl Hash for PropertyAccessTarget {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.map.object().address().hash(state);
            self.name.object().address().hash(state);
            (self.mode as i32).hash(state);
        }
    }

    impl PropertyAccessTarget {
        pub fn hash_code(&self) -> usize {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            hasher.finish() as usize
        }
    }

    impl std::cmp::PartialEq for JSHeapBroker {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }

    impl std::cmp::Eq for JSHeapBroker {}

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GetOrCreateDataFlag {
        KCrashOnError = 1 << 0,
        KAssumeMemoryFence = 1 << 1,
    }

    impl GetOrCreateDataFlag {
        pub fn bits(self) -> i32 {
            self as i32
        }
    }

    pub type GetOrCreateDataFlags = base::Flags<GetOrCreateDataFlag>;

    #[derive(Debug)]
    pub struct JSHeapBroker {
        isolate_: *mut Isolate,
        #[cfg(V8_COMPRESS_POINTERS)]
        cage_base_: PtrComprCageBase,
        zone_: *mut Zone,
        target_native_context_: Option<NativeContextRef>,
        refs_: *mut RefsMap,
        root_index_map_: RootIndexMap,
        array_and_object_prototypes_: ZoneUnorderedSet<IndirectHandle<JSObject>, IndirectHandle<JSObject>::hash, IndirectHandle<JSObject>::equal_to>,
        mode_: BrokerMode,
        tracing_enabled_: bool,
        code_kind_: CodeKind,
        ph_: Option<std::unique_ptr::UniquePtr<PersistentHandles>>,
        local_isolate_: *mut LocalIsolate,
        canonical_handles_: *mut CanonicalHandlesMap,
        trace_indentation_: u32,
        feedback_: ZoneUnorderedMap<FeedbackSource, *const ProcessedFeedback, FeedbackSource::Hash, FeedbackSource::Equal>,
        property_access_infos_: ZoneUnorderedMap<PropertyAccessTarget, PropertyAccessInfo, PropertyAccessTarget::Hash, PropertyAccessTarget::Equal>,
        array_prototype_: Option<ObjectRef>,
        object_prototype_: Option<ObjectRef>,
        dependencies_: *mut CompilationDependencies,
        map_updater_mutex_depth_: i32,
        boilerplate_migration_mutex_depth_: i32,
        phantom: PhantomData<u32>,
        string_array_string_: Option<OptionalRef<StringRef>>,
        number_string_: Option<OptionalRef<StringRef>>,
        symbol_string_: Option<OptionalRef<StringRef>>,
        bigint_string_: Option<OptionalRef<StringRef>>,
        boolean_string_: Option<OptionalRef<StringRef>>,
        function_string_: Option<OptionalRef<StringRef>>,
    }

    impl JSHeapBroker {
        pub fn new(isolate: *mut Isolate, broker_zone: *mut Zone, tracing_enabled: bool, code_kind: CodeKind) -> JSHeapBroker {
            unsafe {
                let refs = (*broker_zone).New_uninit::<RefsMap>();
                (*refs) = RefsMap::new(8, AddressMatcher::new(), broker_zone);
                JSHeapBroker {
                    isolate_: isolate,
                    #[cfg(V8_COMPRESS_POINTERS)]
                    cage_base_: PtrComprCageBase::new((*isolate).cage_base()),
                    zone_: broker_zone,
                    target_native_context_: None,
                    refs_: refs,
                    root_index_map_: RootIndexMap::new(isolate),
                    array_and_object_prototypes_: ZoneUnorderedSet::new(broker_zone),
                    mode_: BrokerMode::KDisabled,
                    tracing_enabled_: tracing_enabled,
                    code_kind_: code_kind,
                    ph_: None,
                    local_isolate_: std::ptr::null_mut(),
                    canonical_handles_: std::ptr::null_mut(),
                    trace_indentation_: 0,
                    feedback_: ZoneUnorderedMap::new(broker_zone),
                    property_access_infos_: ZoneUnorderedMap::new(broker_zone),
                    array_prototype_: None,
                    object_prototype_: None,
                    dependencies_: std::ptr::null_mut(),
                    map_updater_mutex_depth_: 0,
                    boilerplate_migration_mutex_depth_: 0,
                    phantom: PhantomData,
                    string_array_string_: None,
                    number_string_: None,
                    symbol_string_: None,
                    bigint_string_: None,
                    boolean_string_: None,
                    function_string_: None,
                }
            }
        }

        pub fn tracing_enabled(&self) -> bool {
            self.tracing_enabled_
        }

        pub fn trace(&self) -> String {
            let mut oss = String::new();
            oss.push_str(&format!("[{:p}] ", self));
            for _ in 0..(self.trace_indentation_ * 2) {
                oss.push(' ');
            }
            oss
        }

        pub fn new_for_testing(isolate: *mut Isolate, broker_zone: *mut Zone) -> JSHeapBroker {
            JSHeapBroker::new(isolate, broker_zone, false, CodeKind::TURBOFAN_JS)
        }

        pub fn target_native_context(&self) -> NativeContextRef {
            self.target_native_context_.unwrap()
        }

        pub fn set_target_native_context_ref(&mut self, native_context: DirectHandle<NativeContext>) {
            self.target_native_context_ = Some(MakeRef(self, *native_context));
        }

        pub fn initialize_and_start_serializing(&mut self, native_context: DirectHandle<NativeContext>) {
             self.set_target_native_context_ref(native_context);
             self.mode_ = BrokerMode::KSerializing;
             self.collect_array_and_object_prototypes();
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn cage_base(&self) -> PtrComprCageBase {
            self.cage_base_
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }

        pub fn feedback_nexus_config(&self) -> NexusConfig {
            if self.is_main_thread() {
                unsafe {
                    NexusConfig::from_main_thread(self.isolate())
                }
            } else {
                unsafe {
                    NexusConfig::from_background_thread(self.isolate(), (*self.local_isolate()).heap())
                }
            }
        }

        pub fn mode(&self) -> BrokerMode {
            self.mode_
        }

        pub fn stop_serializing(&mut self) {
            if self.mode_ != BrokerMode::KSerializing {
                panic!("StopSerializing called when not serializing");
            }
            self.trace_scoped("Stopping serialization");
            self.mode_ = BrokerMode::KSerialized;
        }

        pub fn retire(&mut self) {
            if self.mode_ != BrokerMode::KSerialized {
                panic!("Retire called when not serialized");
            }
            self.trace_scoped("Retiring");
            self.mode_ = BrokerMode::KRetired;
        }

        pub fn serializing_allowed(&self) -> bool {
            self.mode_ == BrokerMode::KSerializing
        }

        pub fn attach_local_isolate(&mut self, info: *mut OptimizedCompilationInfo, local_isolate: *mut LocalIsolate) {
            if self.local_isolate_ != std::ptr::null_mut() {
                panic!("AttachLocalIsolate called when local isolate already attached");
            }
            self.local_isolate_ = local_isolate;
            unsafe {
                (*local_isolate).heap().attach_persistent_handles((*info).DetachPersistentHandles());
            }
        }

        pub fn detach_local_isolate(&mut self, info: *mut OptimizedCompilationInfo) {
            if self.ph_.is_some() {
                panic!("DetachLocalIsolate called when persistent handles already attached");
            }
            if self.local_isolate_ == std::ptr::null_mut() {
                panic!("DetachLocalIsolate called when local isolate not attached");
            }
            unsafe {
                let ph = (*self.local_isolate_).heap().detach_persistent_handles();
                self.local_isolate_ = std::ptr::null_mut();
                (*info).set_persistent_handles(ph);
            }
        }
        pub fn attach_local_isolate_for_maglev(&mut self, info: *mut maglev::MaglevCompilationInfo, local_isolate: *mut LocalIsolate) {
            todo!()
        }
        pub fn detach_local_isolate_for_maglev(&mut self, info: *mut maglev::MaglevCompilationInfo) {
            todo!()
        }
        pub fn attach_compilation_info(&mut self, info: *mut OptimizedCompilationInfo) {
            unsafe {
                 self.set_canonical_handles((*info).canonical_handles());
            }
        }

        pub fn stack_has_overflowed(&self) -> bool {
            if self.local_isolate_ == std::ptr::null_mut() {
                unsafe {ThreadId::current() == (*self.isolate_).thread_id()}
            } else {
                unsafe {StackLimitCheck::has_overflowed(self.local_isolate_)}
            }
        }

        pub fn get_root_handle(&self, object: Tagged<Object>) -> DirectHandle<Object> {
            unsafe {
                DirectHandle::new((*self.isolate_).root_handle(object))
            }
        }

        pub fn try_get_or_create_data(&mut self, object: Handle<Object>, flags: GetOrCreateDataFlags) -> *mut ObjectData {
            unsafe {
                if object.is_null() {
                    return std::ptr::null_mut();
                }

                if let Some(data) = (*self.refs_).lookup(&(*object.location())) {
                    return data as *const ObjectData as *mut ObjectData;
                }

                if flags.contains(GetOrCreateDataFlag::KCrashOnError) {
                   panic!("Crash on error not implemented");
                }

                 std::ptr::null_mut()
            }
        }

        pub fn get_or_create_data(&mut self, object: Handle<Object>, flags: GetOrCreateDataFlags) -> *mut ObjectData {
           let res = self.try_get_or_create_data(object, flags);
           if res.is_null() {
                panic!("ObjectData is null for {:?}", object);
           }
           res
        }

        pub fn try_get_or_create_data_tagged(&mut self, object: Tagged<Object>, flags: GetOrCreateDataFlags) -> *mut ObjectData {
            let handle = self.canonical_persistent_handle(object);
            self.try_get_or_create_data(handle, flags)
        }

        pub fn get_or_create_data_tagged(&mut self, object: Tagged<Object>, flags: GetOrCreateDataFlags) -> *mut ObjectData {
            let handle = self.canonical_persistent_handle(object);
            self.get_or_create_data(handle, flags)
        }

        pub fn is_array_or_object_prototype_handle(&self, object: Handle<JSObject>) -> bool {
             if self.mode() == BrokerMode::KDisabled {
                unsafe {
                     (*self.isolate()).is_in_creation_context(*object, Context::INITIAL_ARRAY_PROTOTYPE_INDEX) ||
                     (*object.location()).map((*self.isolate()).heap()).instance_type() == JS_OBJECT_PROTOTYPE_TYPE
                }
            } else {
                 if self.array_and_object_prototypes_.is_empty() {
                     panic!("Array and object prototypes are empty");
                 }
                unsafe {
                     self.array_and_object_prototypes_.find(&object).is_some()
                }
            }
        }

        pub fn is_array_or_object_prototype(&self, object: JSObjectRef) -> bool {
            self.is_array_or_object_prototype_handle(Handle::from_raw(object.object()))
        }

        pub fn has_feedback(&self, source: &FeedbackSource) -> bool {
           if !source.is_valid() {
                panic!("Feedback source is invalid");
           }
            self.feedback_.contains_key(source)
        }

        pub fn set_feedback(&mut self, source: FeedbackSource, feedback: *const ProcessedFeedback) {
            if !source.is_valid() {
                panic!("Feedback source is invalid");
            }
            if self.feedback_.contains_key(&source) {
                panic!("Feedback already set for source");
            }
            self.feedback_.insert(source, feedback);
        }

        pub fn get_feedback_slot_kind(&self, source: &FeedbackSource) -> FeedbackSlotKind {
            if self.has_feedback(source) {
                unsafe { (*self.get_feedback(source)).slot_kind() }
            } else {
                let nexus = FeedbackNexus::new(source.vector, source.slot, self.feedback_nexus_config());
                nexus.kind()
            }
        }

        pub fn process_feedback_maps_for_element_access(&self, maps: &mut ZoneVector<MapRef>, keyed_mode: &KeyedAccessMode, slot_kind: FeedbackSlotKind) -> ElementAccessFeedback {
           todo!()
        }

        pub fn get_feedback_for_binary_operation(&self, source: &FeedbackSource) -> BinaryOperationHint {
             let feedback = self.process_feedback_for_binary_operation(source);
             if feedback.is_insufficient() {
                BinaryOperationHint::kNone
             } else {
                unsafe{feedback.as_binary_operation().value()}
             }
        }

        pub fn get_feedback_for_compare_operation(&self, source: &FeedbackSource) -> CompareOperationHint {
             let feedback = self.process_feedback_for_compare_operation(source);
             if feedback.is_insufficient() {
                CompareOperationHint::kNone
             } else {
                unsafe{feedback.as_compare_operation().value()}
             }
        }

        pub fn get_feedback_for_for_in(&self, source: &FeedbackSource) -> ForInHint {
             let feedback = self.process_feedback_for_for_in(source);
             if feedback.is_insufficient() {
                ForInHint::kNone
             } else {
                unsafe{feedback.as_for_in().value()}
             }
        }

        pub fn get_feedback_for_call(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_call(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_global_access(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_global_access(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_instance_of(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_instance_of(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_type_of(&self, source: &FeedbackSource) -> TypeOfFeedback::Result {
            let feedback = self.process_feedback_for_type_of(source);
             if feedback.is_insufficient() {
                TypeOfFeedback::Result::kNone
             } else {
                unsafe{feedback.as_type_of().value()}
             }
        }

        pub fn get_feedback_for_array_or_object_literal(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_array_or_object_literal(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_reg_exp_literal(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_reg_exp_literal(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_template_object(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_template_object(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn get_feedback_for_property_access(&self, source: &FeedbackSource, mode: AccessMode, static_name: OptionalNameRef) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_property_access(source, mode, static_name);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn process_feedback_for_binary_operation(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_binary_operation(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn process_feedback_for_compare_operation(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_compare_operation(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn process_feedback_for_for_in(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_for_in(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn process_feedback_for_type_of(&self, source: &FeedbackSource) -> ProcessedFeedback {
            if self.has_feedback(source) {
                unsafe { *self.get_feedback(source) }
            } else {
                let feedback = self.read_feedback_for_type_of(source);
                self.set_feedback(source, &feedback);
                feedback
            }
        }

        pub fn feedback_is_insufficient(&self, source: &FeedbackSource) -> bool {
            if self.has_feedback(source) {
                unsafe { (*self.get_feedback(source)).is_insufficient() }
            } else {
                let nexus = FeedbackNexus::new(source.vector, source.slot, self.feedback_nexus_config());
                nexus.is_uninitialized()
            }
        }

        pub fn get_name_feedback(&self, nexus: &FeedbackNexus) -> OptionalNameRef {
            let raw_name = nexus.get_name();
            if raw_name.is_null() {
                None
            } else {
                Some(MakeRefAssumeMemoryFence(self, raw_name))
            }
        }

        pub fn get_property_access_info(&self, map: MapRef, name: NameRef, access_mode: AccessMode) -> PropertyAccessInfo {
           if self.dependencies_.is_null() {
                panic!("Dependencies is null");
           }

            let target = PropertyAccessTarget::new(map, name, access_mode);
            if let Some(info) = self.property_access_infos_.get(&target) {
                return *info;
            }

             let factory = AccessInfoFactory::new(self, self.zone());
             let access_info = factory.compute_property_access_info(map, name, access_mode);
             unsafe{
                 let target_ptr = &target as *const PropertyAccessTarget;
             }
             let target_clone = PropertyAccessTarget::new(map, name, access_mode);
             self.property_access_infos_.insert(target_clone, access_info);
             access_info
        }

        pub fn get_typed_array_string_tag(&self, kind: ElementsKind) -> StringRef {
           if !is_typed_array_or_rab_gsab_typed_array_elements_kind(kind) {
                panic!("Not a typed array elements kind");
           }

           match kind {
            ElementsKind::kUint8Elements => self.uint8array_string(),
            ElementsKind::kInt8Elements => self.int8array_string(),
            ElementsKind::kUint16Elements => self.uint16array_string(),
            ElementsKind::kInt16Elements => self.int16array_string(),
            ElementsKind::kUint32Elements => self.uint32array_string(),
            ElementsKind::kInt32Elements => self.int32array_string(),
            ElementsKind::kFloat32Elements => self.float32array_string(),
            ElementsKind::kFloat64Elements => self.float64array_string(),
            ElementsKind::kBigUint64Elements => self.biguint64array_string(),
            ElementsKind::kBigInt64Elements => self.bigint64array_string(),
            ElementsKind::kUint8ClampedElements => self.uint8clampedarray_string(),
            _ => panic!("ElementsKind not supported")
           }
        }

        pub fn is_main_thread(&self) -> bool {
             if self.local_isolate_ == std::ptr::null_mut() {
                true
             } else {
                unsafe {(*self.local_isolate_).is_main_thread()}
             }
        }

        pub fn local_isolate(&self) -> *mut LocalIsolate {
            self.local_isolate_
        }

        pub fn local_isolate_or_isolate(&self) -> *mut LocalIsolate {
            if self.local_isolate() != std::ptr::null_mut() {
                self.local_isolate()
            } else {
                unsafe {(*self.isolate()).as_local_isolate()}
            }
        }

        pub fn find_root_index(&self, object: HeapObjectRef) -> Option<RootIndex> {
            if object.is_js_receiver() {
                return None;
            }

            let mut root_index = RootIndex::kFirstImmortalImmovableRoot;
            if unsafe {self.root_index_map_.lookup(*object.object(), &mut root_index)} {
                Some(root_index)
            } else {
                None
            }
        }

        pub fn canonical_persistent_handle<T>(&mut self, object: Tagged<T>) -> Handle<T> {
             unsafe{
                if self.canonical_handles_.is_null() {
                     panic!("self.canonical_handles_ is null");
                }
                if let Some(heap_object) = object.try_cast::<HeapObject>() {
                     let mut root_index = RootIndex::kFirstImmortalImmovableRoot;
                     if (*self.root_index_map_).lookup(heap_object, &mut root_index) {
                          return Handle::new((*self.isolate_).root_handle(root_index).location());
                     }
                }

                let find_result = (*self.canonical_handles_).find_or_insert(object);
                if find_result.already_exists {
                     return Handle::new(*find_result.entry);
                }

                if self.local_isolate() != std::ptr::null_mut() {
                     *find_result.entry = (*(*self.local_isolate_).heap()).new_persistent_handle(object).location();
                } else {
                     DCHECK!(PersistentHandlesScope::is_active(self.isolate_));
                     *find_result.entry = IndirectHandle::<T>::new(object, self.isolate_).location();
                }
                Handle::new(*find_result.entry)
             }
        }

        pub fn canonical_persistent_handle_handle<T>(&mut self, object: Handle<T>) -> Handle<T> {
             if object.is_null() {
                return object;
             }
             self.canonical_persistent_handle(*object)
        }

        pub fn is_canonical_handle<T>(&self, handle: Handle<T>) -> bool {
           unsafe{
                if self.canonical_handles_.is_null() {
                     panic!("canonical_handles_ is null");
                }
                if let Some(heap_object) = (*handle).try_cast::<HeapObject>() {
                     let mut root_index = RootIndex::kFirstImmortalImmovableRoot;
                     if (*self.root_index_map_).lookup(heap_object, &mut root_index) {
                          return true;
                     }
                     if (*self.isolate()).is_builtin_table_handle_location(handle.location()) {
                          return true;
                     }
                }
                (*self.canonical_handles_).find(*handle) != std::ptr::null_mut()
           }
        }

        pub fn increment_tracing_indentation(&mut self) {
             self.trace_indentation_ += 1;
        }

        pub fn decrement_tracing_indentation(&mut self) {
             self.trace_indentation_ -= 1;
        }

        pub fn recursive_mutex_guard_if_needed<'a>(&'a mut self, mutex: *mut Mutex<()>, mutex_depth_address: *mut i32) -> RecursiveMutexGuardIfNeeded<'a> {
            RecursiveMutexGuardIfNeeded::new(self.local_isolate(), mutex, mutex_depth_address)
        }

        pub fn map_updater_guard_if_needed<'a>(&'a mut self) -> MapUpdaterGuardIfNeeded<'a> {
             MapUpdaterGuardIfNeeded::new(self)
        }

        pub fn boilerplate_migration_guard_if_needed<'a>(&'a mut self) -> BoilerplateMigrationGuardIfNeeded<'a> {
             BoilerplateMigrationGuardIfNeeded::new(self)
        }

        pub fn object_may_be_uninitialized_handle(&self, object: DirectHandle<Object>) -> bool {
            self.object_may_be_uninitialized(*object)
        }

        pub fn object_may_be_uninitialized_tagged(&self, object: Tagged<Object>) -> bool {
            if !object.is_heap_object() {
                return false;
            }
            self.object_may_be_uninitialized(object.unchecked_cast::<HeapObject>())
        }

        pub fn object_may_be_uninitialized(&self, object: Tagged<HeapObject>) -> bool {
             if !self.is_main_thread() {
                  unsafe{(*self.isolate()).heap().is_pending_allocation(object)}
             } else {
                false
             }
        }

        pub fn set_dependencies(&mut self, dependencies: *mut CompilationDependencies) {
             if !self.dependencies_.is_null() {
                panic!("Dependencies already set");
             }

             if dependencies.is_null() {
                panic!("Dependencies is null");
             }

             self.dependencies_ = dependencies;
        }

        pub fn dependencies(&self) -> *mut CompilationDependencies {
             if self.dependencies_.is_null() {
                panic!("Dependencies is null");
             }

             self.dependencies_
        }

        pub fn string_array_string(&mut self) -> StringRef {
            if self.string_array_string_.is_none() {
                self.init_string_array_string();
            }
            self.string_array_string_.as_ref().unwrap().as_ref().unwrap().clone()
        }

        fn init_string_array_string(&mut self) {
            let handle = unsafe{(*self.isolate()).factory().string_array_string()};
            self.string_array_string_ = Some(Some(MakeRefAssumeMemoryFence(self, handle)));
        }

        pub fn number_string(&mut self) -> StringRef {
            if self.number_string_.is_none() {
                self.init_number_string();
            }
            self.number_string_.as_ref().unwrap().as_ref().unwrap().clone()
        }

        fn init_number_string(&mut self) {
            let handle = unsafe{(*self.isolate()).factory().number_string()};
            self.number_string_ = Some(Some(MakeRefAssumeMemoryFence(self, handle)));
        }

        pub fn symbol_string(&mut self) -> StringRef {
            if self.symbol_string_.is_none() {
                self.init_symbol_string();
            }
            self.symbol_string_.as_ref().unwrap().as_ref().unwrap().clone()
        }

        fn init_symbol_string(&mut self) {
            let handle = unsafe{(*self.isolate()).factory().symbol_string()};
            self.symbol_string_ = Some(Some(MakeRefAssumeMemoryFence(self, handle)));
        }

        pub fn bigint_string(&mut self) -> StringRef {
            if self.bigint_string_.is_none() {
                self.init_bigint_string();
            }
            self.bigint_string_.as_ref().unwrap().as_ref().unwrap().clone()
        }

        fn init_bigint_string(&mut self) {
            let handle = unsafe{(*self.isolate()).factory().bigint_string()};
            self.bigint_string_ = Some(Some(MakeRefAssumeMemoryFence(self, handle)));
        }

        pub fn boolean_string

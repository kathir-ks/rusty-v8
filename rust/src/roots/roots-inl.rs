// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/roots/roots-inl.h

//use crate::common::ptr_compr::*; // Assuming ptr-compr-inl.h functionality is in this module
//use crate::execution::isolate::*; // Assuming isolate.h functionality is in this module
//use crate::execution::local_isolate::*; // Assuming local-isolate.h functionality is in this module
//use crate::handles::handles::*; // Assuming handles.h functionality is in this module
//use crate::heap::page_metadata::*; // Assuming page-metadata-inl.h functionality is in this module
//use crate::heap::read_only_heap::*; // Assuming read-only-heap-inl.h functionality is in this module
//use crate::objects::api_callbacks::*; // Assuming api-callbacks.h functionality is in this module
//use crate::objects::cell::*; // Assuming cell.h functionality is in this module
//use crate::objects::descriptor_array::*; // Assuming descriptor-array.h functionality is in this module
//use crate::objects::feedback_vector::*; // Assuming feedback-vector.h functionality is in this module
//use crate::objects::heap_number::*; // Assuming heap-number.h functionality is in this module
//use crate::objects::hole::*; // Assuming hole.h functionality is in this module
//use crate::objects::literal_objects::*; // Assuming literal-objects.h functionality is in this module
//use crate::objects::map::*; // Assuming map.h functionality is in this module
//use crate::objects::oddball::*; // Assuming oddball.h functionality is in this module
//use crate::objects::property_array::*; // Assuming property-array.h functionality is in this module
//use crate::objects::property_cell::*; // Assuming property-cell.h functionality is in this module
//use crate::objects::scope_info::*; // Assuming scope-info.h functionality is in this module
//use crate::objects::slots::*; // Assuming slots.h functionality is in this module
//use crate::objects::string::*; // Assuming string.h functionality is in this module
//use crate::objects::swiss_name_dictionary::*; // Assuming swiss-name-dictionary.h functionality is in this module
//use crate::objects::tagged::*; // Assuming tagged.h functionality is in this module
use crate::roots::roots::*;
//use crate::roots::static_roots::*; // Assuming static-roots.h functionality is in this module

//#[cfg(V8_ENABLE_WEBASSEMBLY)]
//use crate::wasm::wasm_objects::*; // Assuming wasm-objects.h functionality is in this module

//use std::mem::transmute;
use std::ops::{Add, Sub};
use std::ptr::NonNull;

//use crate::base::range::*;

impl PartialOrd for RootIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}

impl Ord for RootIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialEq for RootIndex {
    fn eq(&self, other: &Self) -> bool {
        (*self as usize) == (*other as usize)
    }
}

impl Eq for RootIndex {}

impl Add<usize> for RootIndex {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        let result = (self as usize) + other;
        Self::from(result)
    }
}

impl RootsTable {
    pub fn is_root_handle_location(&self, handle_location: *mut Address, index: &mut RootIndex) -> bool {
        let location = FullObjectSlot(handle_location);
        let first_root = FullObjectSlot(&self.roots[0] as *const Address as *mut Address);
        let last_root = FullObjectSlot(&self.roots[kEntriesCount] as *const Address as *mut Address);

        if location >= last_root {
            return false;
        }
        if location < first_root {
            return false;
        }

        *index = RootIndex::from(location.0 as usize - first_root.0 as usize);
        true
    }

    pub fn is_root_handle<T>(&self, handle: IndirectHandle<T>, index: &mut RootIndex) -> bool {
        let handle_location = handle.address() as *mut Address;
        self.is_root_handle_location(handle_location, index)
    }

    pub fn handle_at(&self, index: RootIndex) -> IndirectHandle<Object> {
        IndirectHandle::new(&self[index] as *const Address as *mut Address)
    }

    pub fn null_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kNullValue)
    }

    pub fn undefined_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kUndefinedValue)
    }

    pub fn the_hole_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kTheHoleValue)
    }

    pub fn true_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kTrueValue)
    }

    pub fn false_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFalseValue)
    }

    pub fn uninitialized_value(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kUninitializedValue)
    }

    pub fn arguments_marker(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kArgumentsMarker)
    }

    pub fn exception(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kException)
    }

    pub fn termination_exception(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kTerminationException)
    }

    pub fn optimized_out(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kOptimizedOut)
    }

    pub fn deoptimized(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kDeoptimized)
    }

    pub fn stale_register(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kStaleRegister)
    }

    pub fn empty_string(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyString)
    }

    pub fn empty_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyArray)
    }

    pub fn empty_fixed_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyFixedArray)
    }

    pub fn empty_fixed_double_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyFixedDoubleArray)
    }

    pub fn empty_weak_fixed_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyWeakFixedArray)
    }

    pub fn empty_descriptor_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyDescriptorArray)
    }

    pub fn empty_property_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyPropertyArray)
    }

    pub fn empty_constant_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyConstantArray)
    }

    pub fn empty_feedback_cell_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyFeedbackCellArray)
    }

    pub fn empty_closure_feedback_array(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyClosureFeedbackArray)
    }

    pub fn empty_feedback_vector(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyFeedbackVector)
    }

    pub fn empty_serialized_feedback_vector(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptySerializedFeedbackVector)
    }

    pub fn empty_context(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEmptyContext)
    }

    pub fn native_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kNativeContextMap)
    }

    pub fn global_this_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kGlobalThisMap)
    }

    pub fn global_proxy_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kGlobalProxyMap)
    }

    pub fn shared_function_info_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kSharedFunctionInfoMap)
    }

    pub fn bytecode_array_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kBytecodeArrayMap)
    }

    pub fn free_space_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFreeSpaceMap)
    }

    pub fn one_byte_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kOneByteStringMap)
    }

    pub fn internalized_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kInternalizedStringMap)
    }

    pub fn symbol_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kSymbolMap)
    }

    pub fn fixed_array_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFixedArrayMap)
    }

    pub fn fixed_array_with_holes_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFixedArrayWithHolesMap)
    }

    pub fn fixed_double_array_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFixedDoubleArrayMap)
    }

    pub fn heap_number_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kHeapNumberMap)
    }

    pub fn cell_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kCellMap)
    }

    pub fn property_cell_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPropertyCellMap)
    }

    pub fn map_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kMapMap)
    }

    pub fn prototype_info_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPrototypeInfoMap)
    }

    pub fn array_buffer_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kArrayBufferMap)
    }

    pub fn data_view_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kDataViewMap)
    }

    pub fn promise_capability_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPromiseCapabilityMap)
    }

    pub fn promise_reaction_job_task_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPromiseReactionJobTaskMap)
    }

    pub fn promise_resolve_thenable_job_task_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPromiseResolveThenableJobTaskMap)
    }

    pub fn regexp_match_info_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kRegExpMatchInfoMap)
    }

    pub fn scope_info_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kScopeInfoMap)
    }

    pub fn script_context_table_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kScriptContextTableMap)
    }

    pub fn uncompiled_data_with_preparse_data_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kUncompiledDataWithPreparseDataMap)
    }

    pub fn uncompiled_data_without_preparse_data_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kUncompiledDataWithoutPreparseDataMap)
    }

    pub fn script_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kScriptMap)
    }

    pub fn code_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kCodeMap)
    }

    pub fn weak_cell_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWeakCellMap)
    }

    pub fn string_table(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kStringTable)
    }

    pub fn number_string_cache(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kNumberStringCache)
    }

    pub fn function_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFunctionContextMap)
    }

    pub fn block_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kBlockContextMap)
    }

    pub fn module_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kModuleContextMap)
    }

    pub fn eval_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kEvalContextMap)
    }

    pub fn with_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWithContextMap)
    }

    pub fn debugger_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kDebuggerContextMap)
    }

    pub fn script_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kScriptContextMap)
    }

    pub fn native_context_scope_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kNativeContextScopeMap)
    }

    pub fn snapshot_context_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kSnapshotContextMap)
    }

    pub fn module_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kModuleMap)
    }

    pub fn module_info_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kModuleInfoMap)
    }

    pub fn fixed_typed_array_base_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kFixedTypedArrayBaseMap)
    }

    pub fn external_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kExternalStringMap)
    }

    pub fn thin_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kThinStringMap)
    }

    pub fn cons_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kConsStringMap)
    }

    pub fn sliced_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kSlicedStringMap)
    }

    pub fn symbol_to_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kSymbolToStringMap)
    }

    pub fn foreign_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kForeignMap)
    }

    pub fn module_request_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kModuleRequestMap)
    }

    pub fn promise_fulfill_reaction_job_task_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPromiseFulfillReactionJobTaskMap)
    }

    pub fn promise_reject_reaction_job_task_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPromiseRejectReactionJobTaskMap)
    }

    pub fn big_int_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kBigIntMap)
    }

    pub fn private_symbol_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kPrivateSymbolMap)
    }

    pub fn template_object_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kTemplateObjectMap)
    }

    pub fn wtf_8_string_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWtf8StringMap)
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn wasm_instance_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWasmInstanceMap)
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn wasm_module_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWasmModuleMap)
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn wasm_shared_module_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWasmSharedModuleMap)
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn wasm_memory_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWasmMemoryMap)
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn wasm_table_map(&self) -> IndirectHandle<Object> {
        self.handle_at(RootIndex::kWasmTableMap)
    }
}

impl std::ops::Index<RootIndex> for RootsTable {
    type Output = Address;

    fn index(&self, index: RootIndex) -> &Self::Output {
        &self.roots[index as usize]
    }
}

pub struct ReadOnlyRoots {
    read_only_roots_: *mut Address,
}

impl ReadOnlyRoots {
    // TODO: Replace Heap, Isolate and LocalIsolate with their rust equivalents
    // TODO: Find a way to access IsolateGroup::current() to implement GetReadOnlyRoots()

    // pub fn get_read_only_roots() -> Self {
    //     let shared_ro_heap = IsolateGroup::current().shared_read_only_heap();
    //     debug_assert!(shared_ro_heap.is_some() && shared_ro_heap.unwrap().roots_init_complete());
    //     ReadOnlyRoots {
    //         read_only_roots_: shared_ro_heap.unwrap().read_only_roots_
    //     }
    // }

    pub fn new_from_heap(_heap: &()) -> Self {
        //ReadOnlyRoots(Isolate::from_heap(heap))
        unimplemented!()
    }

    pub fn new_from_isolate(_isolate: &()) -> Self {
        // ReadOnlyRoots(isolate.roots_table().read_only_roots_begin().address())
        unimplemented!()
    }

    pub fn new_from_local_isolate(_isolate: &()) -> Self {
        //ReadOnlyRoots(isolate.factory().read_only_roots())
        unimplemented!()
    }

    fn unchecked_null_value(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kNullValue)
    }
    pub fn null_value(&self) -> Tagged<Object> {
        self.unchecked_null_value()
    }

    fn unchecked_undefined_value(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kUndefinedValue)
    }
    pub fn undefined_value(&self) -> Tagged<Object> {
        self.unchecked_undefined_value()
    }

    fn unchecked_the_hole_value(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kTheHoleValue)
    }
    pub fn the_hole_value(&self) -> Tagged<Object> {
        self.unchecked_the_hole_value()
    }

    fn unchecked_true_value(&self) -> Tagged<Boolean> {
        unsafe { Tagged::<Boolean>::from_ptr(self.object_at(RootIndex::kTrueValue).ptr()) }
    }
    pub fn true_value(&self) -> Tagged<Boolean> {
        self.unchecked_true_value()
    }

    fn unchecked_false_value(&self) -> Tagged<Boolean> {
        unsafe { Tagged::<Boolean>::from_ptr(self.object_at(RootIndex::kFalseValue).ptr()) }
    }
    pub fn false_value(&self) -> Tagged<Boolean> {
        self.unchecked_false_value()
    }

    fn unchecked_uninitialized_value(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kUninitializedValue)
    }
    pub fn uninitialized_value(&self) -> Tagged<Object> {
        self.unchecked_uninitialized_value()
    }

    fn unchecked_arguments_marker(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kArgumentsMarker)
    }
    pub fn arguments_marker(&self) -> Tagged<Object> {
        self.unchecked_arguments_marker()
    }

    fn unchecked_exception(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kException)
    }
    pub fn exception(&self) -> Tagged<Object> {
        self.unchecked_exception()
    }

    fn unchecked_termination_exception(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kTerminationException)
    }
    pub fn termination_exception(&self) -> Tagged<Object> {
        self.unchecked_termination_exception()
    }

    fn unchecked_optimized_out(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kOptimizedOut)
    }
    pub fn optimized_out(&self) -> Tagged<Object> {
        self.unchecked_optimized_out()
    }

    fn unchecked_deoptimized(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kDeoptimized)
    }
    pub fn deoptimized(&self) -> Tagged<Object> {
        self.unchecked_deoptimized()
    }

    fn unchecked_stale_register(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kStaleRegister)
    }
    pub fn stale_register(&self) -> Tagged<Object> {
        self.unchecked_stale_register()
    }

    fn unchecked_empty_string(&self) -> Tagged<String> {
        unsafe { Tagged::<String>::from_ptr(self.object_at(RootIndex::kEmptyString).ptr()) }
    }
    pub fn empty_string(&self) -> Tagged<String> {
        self.unchecked_empty_string()
    }

    fn unchecked_empty_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyArray)
    }
    pub fn empty_array(&self) -> Tagged<Object> {
        self.unchecked_empty_array()
    }

    fn unchecked_empty_fixed_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyFixedArray)
    }
    pub fn empty_fixed_array(&self) -> Tagged<Object> {
        self.unchecked_empty_fixed_array()
    }

    fn unchecked_empty_fixed_double_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyFixedDoubleArray)
    }
    pub fn empty_fixed_double_array(&self) -> Tagged<Object> {
        self.unchecked_empty_fixed_double_array()
    }

    fn unchecked_empty_weak_fixed_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyWeakFixedArray)
    }
    pub fn empty_weak_fixed_array(&self) -> Tagged<Object> {
        self.unchecked_empty_weak_fixed_array()
    }

    fn unchecked_empty_descriptor_array(&self) -> Tagged<DescriptorArray> {
        unsafe { Tagged::<DescriptorArray>::from_ptr(self.object_at(RootIndex::kEmptyDescriptorArray).ptr()) }
    }
    pub fn empty_descriptor_array(&self) -> Tagged<DescriptorArray> {
        self.unchecked_empty_descriptor_array()
    }

    fn unchecked_empty_property_array(&self) -> Tagged<PropertyArray> {
        unsafe { Tagged::<PropertyArray>::from_ptr(self.object_at(RootIndex::kEmptyPropertyArray).ptr()) }
    }
    pub fn empty_property_array(&self) -> Tagged<PropertyArray> {
        self.unchecked_empty_property_array()
    }

    fn unchecked_empty_constant_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyConstantArray)
    }
    pub fn empty_constant_array(&self) -> Tagged<Object> {
        self.unchecked_empty_constant_array()
    }

    fn unchecked_empty_feedback_cell_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyFeedbackCellArray)
    }
    pub fn empty_feedback_cell_array(&self) -> Tagged<Object> {
        self.unchecked_empty_feedback_cell_array()
    }

    fn unchecked_empty_closure_feedback_array(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyClosureFeedbackArray)
    }
    pub fn empty_closure_feedback_array(&self) -> Tagged<Object> {
        self.unchecked_empty_closure_feedback_array()
    }

    fn unchecked_empty_feedback_vector(&self) -> Tagged<FeedbackVector> {
        unsafe { Tagged::<FeedbackVector>::from_ptr(self.object_at(RootIndex::kEmptyFeedbackVector).ptr()) }
    }
    pub fn empty_feedback_vector(&self) -> Tagged<FeedbackVector> {
        self.unchecked_empty_feedback_vector()
    }

    fn unchecked_empty_serialized_feedback_vector(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptySerializedFeedbackVector)
    }
    pub fn empty_serialized_feedback_vector(&self) -> Tagged<Object> {
        self.unchecked_empty_serialized_feedback_vector()
    }

    fn unchecked_empty_context(&self) -> Tagged<Object> {
        self.object_at(RootIndex::kEmptyContext)
    }
    pub fn empty_context(&self) -> Tagged<Object> {
        self.unchecked_empty_context()
    }

    fn unchecked_native_context_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kNativeContextMap).ptr()) }
    }
    pub fn native_context_map(&self) -> Tagged<Map> {
        self.unchecked_native_context_map()
    }

    fn unchecked_global_this_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kGlobalThisMap).ptr()) }
    }
    pub fn global_this_map(&self) -> Tagged<Map> {
        self.unchecked_global_this_map()
    }

    fn unchecked_global_proxy_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kGlobalProxyMap).ptr()) }
    }
    pub fn global_proxy_map(&self) -> Tagged<Map> {
        self.unchecked_global_proxy_map()
    }

    fn unchecked_shared_function_info_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kSharedFunctionInfoMap).ptr()) }
    }
    pub fn shared_function_info_map(&self) -> Tagged<Map> {
        self.unchecked_shared_function_info_map()
    }

    fn unchecked_bytecode_array_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kBytecodeArrayMap).ptr()) }
    }
    pub fn bytecode_array_map(&self) -> Tagged<Map> {
        self.unchecked_bytecode_array_map()
    }

    fn unchecked_free_space_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kFreeSpaceMap).ptr()) }
    }
    pub fn free_space_map(&self) -> Tagged<Map> {
        self.unchecked_free_space_map()
    }

    fn unchecked_one_byte_string_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kOneByteStringMap).ptr()) }
    }
    pub fn one_byte_string_map(&self) -> Tagged<Map> {
        self.unchecked_one_byte_string_map()
    }

    fn unchecked_internalized_string_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kInternalizedStringMap).ptr()) }
    }
    pub fn internalized_string_map(&self) -> Tagged<Map> {
        self.unchecked_internalized_string_map()
    }

    fn unchecked_symbol_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kSymbolMap).ptr()) }
    }
    pub fn symbol_map(&self) -> Tagged<Map> {
        self.unchecked_symbol_map()
    }

    fn unchecked_fixed_array_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kFixedArrayMap).ptr()) }
    }
    pub fn fixed_array_map(&self) -> Tagged<Map> {
        self.unchecked_fixed_array_map()
    }

    fn unchecked_fixed_array_with_holes_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kFixedArrayWithHolesMap).ptr()) }
    }
    pub fn fixed_array_with_holes_map(&self) -> Tagged<Map> {
        self.unchecked_fixed_array_with_holes_map()
    }

    fn unchecked_fixed_double_array_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kFixedDoubleArrayMap).ptr()) }
    }
    pub fn fixed_double_array_map(&self) -> Tagged<Map> {
        self.unchecked_fixed_double_array_map()
    }

    fn unchecked_heap_number_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kHeapNumberMap).ptr()) }
    }
    pub fn heap_number_map(&self) -> Tagged<Map> {
        self.unchecked_heap_number_map()
    }

    fn unchecked_cell_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kCellMap).ptr()) }
    }
    pub fn cell_map(&self) -> Tagged<Map> {
        self.unchecked_cell_map()
    }

    fn unchecked_property_cell_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kPropertyCellMap).ptr()) }
    }
    pub fn property_cell_map(&self) -> Tagged<Map> {
        self.unchecked_property_cell_map()
    }

    fn unchecked_map_map(&self) -> Tagged<Map> {
        unsafe { Tagged::<Map>::from_ptr(self.object_at(RootIndex::kMapMap).ptr()) }
    }
    pub fn map_map(&self) -> Tag
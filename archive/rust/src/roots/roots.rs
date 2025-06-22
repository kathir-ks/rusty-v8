// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #ifndef V8_ROOTS_ROOTS_H_
// #define V8_ROOTS_ROOTS_H_

// #include "src/base/macros.h"
// #include "src/builtins/accessors.h"
// #include "src/common/globals.h"
// #include "src/handles/handles.h"
// #include "src/init/heap-symbols.h"
// #include "src/objects/objects-definitions.h"
// #include "src/objects/objects.h"
// #include "src/objects/slots.h"
// #include "src/objects/tagged.h"

// use crate::base::macros::*; // Assuming these are in a crate named 'base'
// use crate::builtins::accessors::*; // Assuming these are in a crate named 'builtins'
// use crate::common::globals::*; // Assuming these are in a crate named 'common'
// use crate::handles::handles::*; // Assuming these are in a crate named 'handles'
// use crate::init::heap_symbols::*; // Assuming these are in a crate named 'init'
// use crate::objects::objects_definitions::*; // Assuming these are in a crate named 'objects'
// use crate::objects::objects::*; // Assuming these are in a crate named 'objects'
// use crate::objects::slots::*; // Assuming these are in a crate named 'objects'
// use crate::objects::tagged::*; // Assuming these are in a crate named 'objects'

pub mod roots {
    // Forward declarations.
    pub struct Boolean {}
    pub enum ElementsKind {
        // Define enum variants here
    }
    pub struct Factory {}
    pub struct FactoryBase {} // TODO: Implement template <typename Impl> class FactoryBase;
    pub struct LocalFactory {}
    pub struct PropertyCell {}
    pub struct ReadOnlyHeap {}
    pub struct RootVisitor {}

    macro_rules! strong_read_only_heap_number_root_list {
        ($v:ident) => {
            $v!(HeapNumber, nan_value, NanValue);
            $v!(HeapNumber, hole_nan_value, HoleNanValue);
            $v!(HeapNumber, infinity_value, InfinityValue);
            $v!(HeapNumber, minus_zero_value, MinusZeroValue);
            $v!(HeapNumber, minus_infinity_value, MinusInfinityValue);
            $v!(HeapNumber, max_safe_integer, MaxSafeInteger);
            $v!(HeapNumber, max_uint_32, MaxUInt32);
            $v!(HeapNumber, smi_min_value, SmiMinValue);
            $v!(HeapNumber, smi_max_value_plus_one, SmiMaxValuePlusOne);
        };
    }

    macro_rules! internalized_string_list_adapter {
        ($v:ident, $name:ident, $($rest:tt)*) => {
            $v!(String, $name, $name);
        };
    }

    macro_rules! extra_important_internalized_string_root_list {
        ($v:ident) => {
            extra_important_internalized_string_list_generator!(
                internalized_string_list_adapter,
                $v
            );
        };
    }

    macro_rules! strong_read_only_root_list {
        ($v:ident) => {
            $v!(Map, free_space_map, FreeSpaceMap);
            $v!(Map, one_pointer_filler_map, OnePointerFillerMap);
            $v!(Map, two_pointer_filler_map, TwoPointerFillerMap);
            $v!(Hole, uninitialized_value, UninitializedValue);
            $v!(Undefined, undefined_value, UndefinedValue);
            $v!(Hole, the_hole_value, TheHoleValue);
            $v!(Null, null_value, NullValue);
            $v!(True, true_value, TrueValue);
            $v!(False, false_value, FalseValue);
            extra_important_internalized_string_root_list!($v);
            $v!(Map, meta_map, MetaMap);
            $v!(Map, byte_array_map, ByteArrayMap);
            $v!(Map, fixed_array_map, FixedArrayMap);
            $v!(Map, fixed_cow_array_map, FixedCOWArrayMap);
            $v!(Map, fixed_double_array_map, FixedDoubleArrayMap);
            $v!(Map, hash_table_map, HashTableMap);
            $v!(Map, symbol_map, SymbolMap);
            $v!(Map, seq_one_byte_string_map, SeqOneByteStringMap);
            $v!(Map, internalized_one_byte_string_map, InternalizedOneByteStringMap);
            $v!(Map, scope_info_map, ScopeInfoMap);
            $v!(Map, shared_function_info_map, SharedFunctionInfoMap);
            $v!(Map, instruction_stream_map, InstructionStreamMap);
            $v!(Map, cell_map, CellMap);
            $v!(Map, global_property_cell_map, GlobalPropertyCellMap);
            $v!(Map, foreign_map, ForeignMap);
            $v!(Map, heap_number_map, HeapNumberMap);
            $v!(Map, transition_array_map, TransitionArrayMap);
            $v!(Map, feedback_vector_map, FeedbackVectorMap);
            $v!(ScopeInfo, empty_scope_info, EmptyScopeInfo);
            $v!(FixedArray, empty_fixed_array, EmptyFixedArray);
            $v!(DescriptorArray, empty_descriptor_array, EmptyDescriptorArray);
            $v!(Hole, arguments_marker, ArgumentsMarker);
            $v!(Hole, exception, Exception);
            $v!(Hole, termination_exception, TerminationException);
            $v!(Hole, optimized_out, OptimizedOut);
            $v!(Hole, stale_register, StaleRegister);
            $v!(Hole, property_cell_hole_value, PropertyCellHoleValue);
            $v!(Hole, hash_table_hole_value, HashTableHoleValue);
            $v!(Hole, promise_hole_value, PromiseHoleValue);
            $v!(Map, script_context_table_map, ScriptContextTableMap);
            $v!(Map, closure_feedback_cell_array_map, ClosureFeedbackCellArrayMap);
            $v!(Map, feedback_metadata_map, FeedbackMetadataArrayMap);
            $v!(Map, array_list_map, ArrayListMap);
            $v!(Map, bigint_map, BigIntMap);
            $v!(Map, object_boilerplate_description_map, ObjectBoilerplateDescriptionMap);
            $v!(Map, bytecode_array_map, BytecodeArrayMap);
            $v!(Map, code_map, CodeMap);
            $v!(Map, coverage_info_map, CoverageInfoMap);
            $v!(Map, dictionary_template_info_map, DictionaryTemplateInfoMap);
            $v!(Map, global_dictionary_map, GlobalDictionaryMap);
            $v!(Map, global_context_side_property_cell_map, GlobalContextSidePropertyCellMap);
            $v!(Map, many_closures_cell_map, ManyClosuresCellMap);
            $v!(Map, mega_dom_handler_map, MegaDomHandlerMap);
            $v!(Map, module_info_map, ModuleInfoMap);
            $v!(Map, name_dictionary_map, NameDictionaryMap);
            $v!(Map, no_closures_cell_map, NoClosuresCellMap);
            $v!(Map, number_dictionary_map, NumberDictionaryMap);
            $v!(Map, one_closure_cell_map, OneClosureCellMap);
            $v!(Map, ordered_hash_map_map, OrderedHashMapMap);
            $v!(Map, ordered_hash_set_map, OrderedHashSetMap);
            $v!(Map, name_to_index_hash_table_map, NameToIndexHashTableMap);
            $v!(Map, registered_symbol_table_map, RegisteredSymbolTableMap);
            $v!(Map, ordered_name_dictionary_map, OrderedNameDictionaryMap);
            $v!(Map, preparse_data_map, PreparseDataMap);
            $v!(Map, property_array_map, PropertyArrayMap);
            $v!(Map, accessor_info_map, AccessorInfoMap);
            $v!(Map, regexp_match_info_map, RegExpMatchInfoMap);
            $v!(Map, regexp_data_map, RegExpDataMap);
            $v!(Map, atom_regexp_data_map, AtomRegExpDataMap);
            $v!(Map, ir_regexp_data_map, IrRegExpDataMap);
            $v!(Map, simple_number_dictionary_map, SimpleNumberDictionaryMap);
            $v!(Map, small_ordered_hash_map_map, SmallOrderedHashMapMap);
            $v!(Map, small_ordered_hash_set_map, SmallOrderedHashSetMap);
            $v!(Map, small_ordered_name_dictionary_map, SmallOrderedNameDictionaryMap);
            $v!(Map, source_text_module_map, SourceTextModuleMap);
            $v!(Map, swiss_name_dictionary_map, SwissNameDictionaryMap);
            $v!(Map, synthetic_module_map, SyntheticModuleMap);
            #[cfg(feature = "wasm")]
            {
                $v!(Map, wasm_import_data_map, WasmImportDataMap);
                $v!(Map, wasm_capi_function_data_map, WasmCapiFunctionDataMap);
                $v!(Map, wasm_continuation_object_map, WasmContinuationObjectMap);
                $v!(Map, wasm_dispatch_table_map, WasmDispatchTableMap);
                $v!(Map, wasm_exported_function_data_map, WasmExportedFunctionDataMap);
                $v!(Map, wasm_internal_function_map, WasmInternalFunctionMap);
                $v!(Map, wasm_func_ref_map, WasmFuncRefMap);
                $v!(Map, wasm_js_function_data_map, WasmJSFunctionDataMap);
                $v!(Map, wasm_null_map, WasmNullMap);
                $v!(Map, wasm_resume_data_map, WasmResumeDataMap);
                $v!(Map, wasm_suspender_object_map, WasmSuspenderObjectMap);
                $v!(Map, wasm_trusted_instance_data_map, WasmTrustedInstanceDataMap);
                $v!(Map, wasm_type_info_map, WasmTypeInfoMap);
            }
            $v!(Map, weak_fixed_array_map, WeakFixedArrayMap);
            $v!(Map, weak_array_list_map, WeakArrayListMap);
            $v!(Map, ephemeron_hash_table_map, EphemeronHashTableMap);
            $v!(Map, embedder_data_array_map, EmbedderDataArrayMap);
            $v!(Map, weak_cell_map, WeakCellMap);
            $v!(Map, trusted_fixed_array_map, TrustedFixedArrayMap);
            $v!(Map, trusted_weak_fixed_array_map, TrustedWeakFixedArrayMap);
            $v!(Map, trusted_byte_array_map, TrustedByteArrayMap);
            $v!(Map, protected_fixed_array_map, ProtectedFixedArrayMap);
            $v!(Map, protected_weak_fixed_array_map, ProtectedWeakFixedArrayMap);
            $v!(Map, interpreter_data_map, InterpreterDataMap);
            $v!(Map, shared_function_info_wrapper_map, SharedFunctionInfoWrapperMap);
            $v!(Map, trusted_foreign_map, TrustedForeignMap);
            $v!(Map, seq_two_byte_string_map, SeqTwoByteStringMap);
            $v!(Map, cons_two_byte_string_map, ConsTwoByteStringMap);
            $v!(Map, cons_one_byte_string_map, ConsOneByteStringMap);
            $v!(Map, thin_two_byte_string_map, ThinTwoByteStringMap);
            $v!(Map, thin_one_byte_string_map, ThinOneByteStringMap);
            $v!(Map, sliced_two_byte_string_map, SlicedTwoByteStringMap);
            $v!(Map, sliced_one_byte_string_map, SlicedOneByteStringMap);
            $v!(Map, external_two_byte_string_map, ExternalTwoByteStringMap);
            $v!(Map, external_one_byte_string_map, ExternalOneByteStringMap);
            $v!(Map, internalized_two_byte_string_map, InternalizedTwoByteStringMap);
            $v!(Map, external_internalized_two_byte_string_map, ExternalInternalizedTwoByteStringMap);
            $v!(Map, external_internalized_one_byte_string_map, ExternalInternalizedOneByteStringMap);
            $v!(Map, uncached_external_internalized_two_byte_string_map, UncachedExternalInternalizedTwoByteStringMap);
            $v!(Map, uncached_external_internalized_one_byte_string_map, UncachedExternalInternalizedOneByteStringMap);
            $v!(Map, uncached_external_two_byte_string_map, UncachedExternalTwoByteStringMap);
            $v!(Map, uncached_external_one_byte_string_map, UncachedExternalOneByteStringMap);
            $v!(Map, shared_seq_one_byte_string_map, SharedSeqOneByteStringMap);
            $v!(Map, shared_seq_two_byte_string_map, SharedSeqTwoByteStringMap);
            $v!(Map, shared_external_one_byte_string_map, SharedExternalOneByteStringMap);
            $v!(Map, shared_external_two_byte_string_map, SharedExternalTwoByteStringMap);
            $v!(Map, shared_uncached_external_one_byte_string_map, SharedUncachedExternalOneByteStringMap);
            $v!(Map, shared_uncached_external_two_byte_string_map, SharedUncachedExternalTwoByteStringMap);
            $v!(Map, undefined_map, UndefinedMap);
            $v!(Map, null_map, NullMap);
            $v!(Map, boolean_map, BooleanMap);
            $v!(Map, hole_map, HoleMap);
            $v!(Map, js_shared_array_map, JSSharedArrayMap);
            $v!(Map, js_atomics_mutex_map, JSAtomicsMutexMap);
            $v!(Map, js_atomics_condition_map, JSAtomicsConditionMap);
            $v!(EnumCache, empty_enum_cache, EmptyEnumCache);
            $v!(PropertyArray, empty_property_array, EmptyPropertyArray);
            $v!(ByteArray, empty_byte_array, EmptyByteArray);
            $v!(ObjectBoilerplateDescription, empty_object_boilerplate_description, EmptyObjectBoilerplateDescription);
            $v!(ArrayBoilerplateDescription, empty_array_boilerplate_description, EmptyArrayBoilerplateDescription);
            $v!(ClosureFeedbackCellArray, empty_closure_feedback_cell_array, EmptyClosureFeedbackCellArray);
            $v!(NumberDictionary, empty_slow_element_dictionary, EmptySlowElementDictionary);
            $v!(OrderedHashMap, empty_ordered_hash_map, EmptyOrderedHashMap);
            $v!(OrderedHashSet, empty_ordered_hash_set, EmptyOrderedHashSet);
            $v!(FeedbackMetadata, empty_feedback_metadata, EmptyFeedbackMetadata);
            $v!(NameDictionary, empty_property_dictionary, EmptyPropertyDictionary);
            $v!(OrderedNameDictionary, empty_ordered_property_dictionary, EmptyOrderedPropertyDictionary);
            $v!(SwissNameDictionary, empty_swiss_property_dictionary, EmptySwissPropertyDictionary);
            $v!(InterceptorInfo, noop_interceptor_info, NoOpInterceptorInfo);
            $v!(ArrayList, empty_array_list, EmptyArrayList);
            $v!(WeakFixedArray, empty_weak_fixed_array, EmptyWeakFixedArray);
            $v!(WeakArrayList, empty_weak_array_list, EmptyWeakArrayList);
            $v!(Cell, invalid_prototype_validity_cell, InvalidPrototypeValidityCell);
            $v!(FeedbackCell, many_closures_cell, ManyClosuresCell);
            strong_read_only_heap_number_root_list!($v);
            $v!(FixedArray, single_character_string_table, SingleCharacterStringTable);
            $v!(Hole, self_reference_marker, SelfReferenceMarker);
            $v!(Hole, basic_block_counters_marker, BasicBlockCountersMarker);
            $v!(ScopeInfo, global_this_binding_scope_info, GlobalThisBindingScopeInfo);
            $v!(ScopeInfo, empty_function_scope_info, EmptyFunctionScopeInfo);
            $v!(ScopeInfo, native_scope_info, NativeScopeInfo);
            $v!(ScopeInfo, shadow_realm_scope_info, ShadowRealmScopeInfo);
            $v!(RegisteredSymbolTable, empty_symbol_table, EmptySymbolTable);
            $v!(ByteArray, hash_seed, HashSeed);
            #[cfg(feature = "wasm")]
            {
                $v!(HeapObject, wasm_null_padding, WasmNullPadding);
                $v!(WasmNull, wasm_null, WasmNull);
            }
        };
    }

    macro_rules! trusted_root_list {
        ($v:ident) => {
            $v!(TrustedByteArray, empty_trusted_byte_array, EmptyTrustedByteArray);
            $v!(TrustedFixedArray, empty_trusted_fixed_array, EmptyTrustedFixedArray);
            $v!(TrustedWeakFixedArray, empty_trusted_weak_fixed_array, EmptyTrustedWeakFixedArray);
            $v!(ProtectedFixedArray, empty_protected_fixed_array, EmptyProtectedFixedArray);
            $v!(ProtectedWeakFixedArray, empty_protected_weak_fixed_array, EmptyProtectedWeakFixedArray);
        };
    }

    macro_rules! builtins_with_sfi_list_generator {
        ($apply:ident, $v:ident) => {
            $apply!($v, ProxyRevoke, proxy_revoke);
            $apply!(
                $v,
                AsyncFromSyncIteratorCloseSyncAndRethrow,
                async_from_sync_iterator_close_sync_and_rethrow
            );
            $apply!(
                $v,
                AsyncFunctionAwaitRejectClosure,
                async_function_await_reject_closure
            );
            $apply!(
                $v,
                AsyncFunctionAwaitResolveClosure,
                async_function_await_resolve_closure
            );
            $apply!(
                $v,
                AsyncGeneratorAwaitRejectClosure,
                async_generator_await_reject_closure
            );
            $apply!(
                $v,
                AsyncGeneratorAwaitResolveClosure,
                async_generator_await_resolve_closure
            );
            $apply!(
                $v,
                AsyncGeneratorYieldWithAwaitResolveClosure,
                async_generator_yield_with_await_resolve_closure
            );
            $apply!(
                $v,
                AsyncGeneratorReturnClosedResolveClosure,
                async_generator_return_closed_resolve_closure
            );
            $apply!(
                $v,
                AsyncGeneratorReturnClosedRejectClosure,
                async_generator_return_closed_reject_closure
            );
            $apply!(
                $v,
                AsyncGeneratorReturnResolveClosure,
                async_generator_return_resolve_closure
            );
            $apply!(
                $v,
                AsyncIteratorValueUnwrap,
                async_iterator_value_unwrap
            );
            $apply!(
                $v,
                ArrayFromAsyncArrayLikeOnFulfilled,
                array_from_async_array_like_on_fulfilled
            );
            $apply!(
                $v,
                ArrayFromAsyncArrayLikeOnRejected,
                array_from_async_array_like_on_rejected
            );
            $apply!(
                $v,
                ArrayFromAsyncIterableOnFulfilled,
                array_from_async_iterable_on_fulfilled
            );
            $apply!(
                $v,
                ArrayFromAsyncIterableOnRejected,
                array_from_async_iterable_on_rejected
            );
            $apply!(
                $v,
                PromiseCapabilityDefaultResolve,
                promise_capability_default_resolve
            );
            $apply!(
                $v,
                PromiseCapabilityDefaultReject,
                promise_capability_default_reject
            );
            $apply!(
                $v,
                PromiseGetCapabilitiesExecutor,
                promise_get_capabilities_executor
            );
            $apply!(
                $v,
                PromiseAllSettledResolveElementClosure,
                promise_all_settled_resolve_element_closure
            );
            $apply!(
                $v,
                PromiseAllSettledRejectElementClosure,
                promise_all_settled_reject_element_closure
            );
            $apply!(
                $v,
                PromiseAllResolveElementClosure,
                promise_all_resolve_element_closure
            );
            $apply!(
                $v,
                PromiseAnyRejectElementClosure,
                promise_any_reject_element_closure
            );
            $apply!(
                $v,
                PromiseThrowerFinally,
                promise_thrower_finally
            );
            $apply!(
                $v,
                PromiseValueThunkFinally,
                promise_value_thunk_finally
            );
            $apply!(
                $v,
                PromiseThenFinally,
                promise_then_finally
            );
            $apply!(
                $v,
                PromiseCatchFinally,
                promise_catch_finally
            );
            $apply!(
                $v,
                ShadowRealmImportValueFulfilled,
                shadow_realm_import_value_fulfilled
            );
            $apply!(
                $v,
                AsyncIteratorPrototypeAsyncDisposeResolveClosure,
                async_iterator_prototype_async_dispose_resolve_closure
            );
        };
    }

    macro_rules! builtins_with_sfi_roots_list_adapter {
        ($v:ident, $camel_name:ident, $underscore_name:ident, $($rest:tt)*) => {
            $v!(
                SharedFunctionInfo,
                $underscore_name##_shared_fun,
                $camel_name##SharedFun
            );
        };
    }

    macro_rules! builtins_with_sfi_roots_list {
        ($v:ident) => {
            builtins_with_sfi_list_generator!(builtins_with_sfi_roots_list_adapter, $v);
        };
    }

    macro_rules! strong_mutable_immovable_root_list {
        ($v:ident) => {
            accessor_info_root_list!($v);
            $v!(Script, empty_script, EmptyScript);
            $v!(PropertyCell, array_constructor_protector, ArrayConstructorProtector);
            $v!(PropertyCell, no_elements_protector, NoElementsProtector);
            $v!(PropertyCell, mega_dom_protector, MegaDOMProtector);
            $v!(PropertyCell, no_profiling_protector, NoProfilingProtector);
            $v!(
                PropertyCell,
                no_undetectable_objects_protector,
                NoUndetectableObjectsProtector
            );
            $v!(
                PropertyCell,
                is_concat_spreadable_protector,
                IsConcatSpreadableProtector
            );
            $v!(PropertyCell, array_species_protector, ArraySpeciesProtector);
            $v!(
                PropertyCell,
                typed_array_length_protector,
                TypedArrayLengthProtector
            );
            $v!(
                PropertyCell,
                typed_array_species_protector,
                TypedArraySpeciesProtector
            );
            $v!(PropertyCell, promise_species_protector, PromiseSpeciesProtector);
            $v!(PropertyCell, regexp_species_protector, RegExpSpeciesProtector);
            $v!(PropertyCell, string_length_protector, StringLengthProtector);
            $v!(PropertyCell, array_iterator_protector, ArrayIteratorProtector);
            $v!(
                PropertyCell,
                array_buffer_detaching_protector,
                ArrayBufferDetachingProtector
            );
            $v!(PropertyCell, promise_hook_protector, PromiseHookProtector);
            $v!(PropertyCell, promise_resolve_protector, PromiseResolveProtector);
            $v!(PropertyCell, map_iterator_protector, MapIteratorProtector);
            $v!(PropertyCell, promise_then_protector, PromiseThenProtector);
            $v!(PropertyCell, set_iterator_protector, SetIteratorProtector);
            $v!(
                PropertyCell,
                string_iterator_protector,
                StringIteratorProtector
            );
            $v!(
                PropertyCell,
                string_wrapper_to_primitive_protector,
                StringWrapperToPrimitiveProtector
            );
            $v!(
                PropertyCell,
                number_string_not_regexp_like_protector,
                NumberStringNotRegexpLikeProtector
            );
            $v!(FixedArray, string_split_cache, StringSplitCache);
            $v!(FixedArray, regexp_multiple_cache, RegExpMultipleCache);
            $v!(
                FixedArray,
                regexp_match_global_atom_cache,
                RegExpMatchGlobalAtomCache
            );
            $v!(
                FixedArray,
                builtins_constants_table,
                BuiltinsConstantsTable
            );
            $v!(
                SharedFunctionInfo,
                source_text_module_execute_async_module_fulfilled_sfi,
                SourceTextModuleExecuteAsyncModuleFulfilledSFI
            );
            $v!(
                SharedFunctionInfo,
                source_text_module_execute_async_module_rejected_sfi,
                SourceTextModuleExecuteAsyncModuleRejectedSFI
            );
            $v!(
                SharedFunctionInfo,
                atomics_mutex_async_unlock_resolve_handler_sfi,
                AtomicsMutexAsyncUnlockResolveHandlerSFI
            );
            $v!(
                SharedFunctionInfo,
                atomics_mutex_async_unlock_reject_handler_sfi,
                AtomicsMutexAsyncUnlockRejectHandlerSFI
            );
            $v!(
                SharedFunctionInfo,
                atomics_condition_acquire_lock_sfi,
                AtomicsConditionAcquireLockSFI
            );
            $v!(
                SharedFunctionInfo,
                async_disposable_stack_on_fulfilled_shared_fun,
                AsyncDisposableStackOnFulfilledSharedFun
            );
            $v!(
                SharedFunctionInfo,
                async_disposable_stack_on_rejected_shared_fun,
                AsyncDisposableStackOnRejectedSharedFun
            );
            $v!(
                SharedFunctionInfo,
                async_dispose_from_sync_dispose_shared_fun,
                AsyncDisposeFromSyncDisposeSharedFun
            );
            builtins_with_sfi_roots_list!($v);
            trusted_root_list!($v);
        };
    }

    macro_rules! strong_mutable_movable_root_list {
        ($v:ident) => {
            $v!(FixedArray, number_string_cache, NumberStringCache);
            $v!(RegisteredSymbolTable, public_symbol_table, PublicSymbolTable);
            $v!(RegisteredSymbolTable, api_symbol_table, ApiSymbolTable);
            $v!(
                RegisteredSymbolTable,
                api_private_symbol_table,
                ApiPrivateSymbolTable
            );
            $v!(WeakArrayList, script_list, ScriptList);
            $v!(FixedArray, materialized_objects, MaterializedObjects);
            $v!(WeakArrayList, detached_contexts, DetachedContexts);
            $v!(
                Object,
                feedback_vectors_for_profiling_tools,
                FeedbackVectorsForProfilingTools
            );
            $v!(HeapObject, serialized_objects, SerializedObjects);
            $v!(
                FixedArray,
                serialized_global_proxy_sizes,
                SerializedGlobalProxySizes
            );
            $v!(ArrayList, message_listeners, MessageListeners);
            $v!(HeapObject, current_microtask, CurrentMicrotask);
            $v!(
                HeapObject,
                weak_refs_keep_during_job,
                WeakRefsKeepDuringJob
            );
            $v!(
                Object,
                functions_marked_for_manual_optimization,
                FunctionsMarkedForManualOptimization
            );
            $v!(
                ArrayList,
                basic_block_profiling_data,
                BasicBlockProfilingData
            );
            $v!(WeakArrayList, shared_wasm_memories, SharedWasmMemories);
            $v!(
                HeapObject,
                locals_block_list_cache,
                DebugLocalsBlockListCache
            );
            #[cfg(feature = "wasm")]
            {
                $v!(HeapObject, active_continuation, ActiveContinuation);
                $v!(HeapObject, active_suspender, ActiveSuspender);
                $v!(WeakFixedArray, js_to_wasm_wrappers, JSToWasmWrappers);
                $v!(WeakFixedArray, wasm_canonical_rtts, WasmCanonicalRtts);
            }
            $v!(
                FunctionTemplateInfo,
                error_stack_getter_fun_template,
                ErrorStackGetterSharedFun
            );
            $v!(
                FunctionTemplateInfo,
                error_stack_setter_fun_template,
                ErrorStackSetterSharedFun
            );
        };
    }

    macro_rules! smi_root_list {
        ($v:ident) => {
            $v!(Smi, last_script_id, LastScriptId);
            $v!(Smi, last_debugging_id, LastDebuggingId);
            $v!(Smi, last_stack_trace_id, LastStackTraceId);
            $v!(
                Smi,
                next_template_serial_number,
                NextTemplateSerialNumber
            );
            $v!(
                Smi,
                construct_stub_create_deopt_pc_offset,
                ConstructStubCreateDeoptPCOffset
            );
            $v!(
                Smi,
                construct_stub_invoke_deopt_pc_offset,
                ConstructStubInvokeDeoptPCOffset
            );
            $v!(
                Smi,
                deopt_pc_offset_after_adapt_shadow_stack,
                DeoptPCOffsetAfterAdaptShadowStack
            );
            $v!(
                Smi,
                interpreter_entry_return_pc_offset,
                InterpreterEntryReturnPCOffset
            );
        };
    }

    macro_rules! internalized_string_root_list {
        ($v:ident) => {
            important_internalized_string_list_generator!(
                internalized_string_list_adapter,
                $v
            );
            not_important_internalized_string_list_generator!(
                internalized_string_list_adapter,
                $v
            );
        };
    }

    macro_rules! symbol_root_list_adapter {
        ($v:ident, $name:ident, $($rest:tt)*) => {
            $v!(Symbol, $name, $name);
        };
    }

    macro_rules! private_symbol_root_list {
        ($v:ident) => {
            private_symbol_list_generator!(symbol_root_list_adapter, $v);
        };
    }

    macro_rules! public_symbol_root_list {
        ($v:ident) => {
            public_symbol_list_generator!(symbol_root_list_adapter, $v);
        };
    }

    macro_rules! well_known_symbol_root_list {
        ($v:ident) => {
            well_known_symbol_list_generator!(symbol_root_list_adapter, $v);
        };
    }

    macro_rules! name_for_protector_root_list {
        ($v:ident) => {
            internalized_string_for_protector_list_generator!(
                internalized_string_list_adapter,
                $v
            );
            symbol_for_protector_list_generator!(symbol_root_list_adapter, $v);
            public_symbol_for_protector_list_generator!(symbol_root_list_adapter, $v);
            well_known_symbol_for_protector_list_generator!(symbol_root_list_adapter, $v);
        };
    }

    macro_rules! accessor_info_root_list_adapter {
        ($v:ident, $name:ident, $camel_name:ident, $($rest:tt)*) => {
            $v!(AccessorInfo, $name##_accessor, $camel_name##Accessor);
        };
    }

    macro_rules! accessor_info_root_list {
        ($v:ident) => {
            accessor_info_list_generator!(accessor_info_root_list_adapter, $v);
        };
    }

    macro_rules! read_only_root_list {
        ($v:ident) => {
            strong_read_only_root_list!($v);
            internalized_string_root_list!($v);
            private_symbol_root_list!($v);
            public_symbol_root_list!($v);
            well_known_symbol_root_list!($v);
            struct_maps_list!($v); // Assuming this macro is defined elsewhere
            torque_defined_map_root_list!($v); // Assuming this macro is defined elsewhere
            allocation_site_maps_list!($v); // Assuming this macro is defined elsewhere
            name_for_protector_root_list!($v);
            data_handler_maps_list!($v); // Assuming this macro is defined elsewhere
            $v!(Map, external_map, ExternalMap);
            $v!(Map, message_object_map, JSMessageObjectMap);
        };
    }

    macro_rules! mutable_root_list {
        ($v:ident) => {
            strong_mutable_immovable_root_list!($v);
            strong_mutable_movable_root_list!($v);
            smi_root_list!($v);
        };
    }

    macro_rules! root_list {
        ($v
// Converted from V8 C++ source files:
// Header: N/A
// Implementation: setup-heap-internal.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::rc::Rc;

pub mod api_api_natives {
    // Dummy implementation
    pub struct FunctionTemplateInfo {}
}
pub mod api_api {
    // Dummy implementation
    pub struct Isolate {}
}
pub mod builtins_accessors {
    // Dummy implementation
    pub struct AccessorInfo {}
}
pub mod codegen_compilation_cache {
    // Dummy implementation
    pub struct CompilationCache {}
}
pub mod common_assert_scope {
    // Dummy implementation
    pub struct AssertScope {}
}
pub mod execution_isolate {
    // Dummy implementation
    pub struct Isolate {
        pub read_only_heap: ReadOnlyHeap,
        pub heap_: Heap,
    }
}
pub mod execution_protectors {
    // Dummy implementation
    pub struct Protectors {}
}
pub mod heap_factory {
    // Dummy implementation
    pub struct Factory {}
}
pub mod heap_heap_inl {
    // Dummy implementation
    pub struct Heap {}
}
pub mod heap_new_spaces {
    // Dummy implementation
    pub struct NewSpaces {}
}
pub mod ic_handler_configuration {
    // Dummy implementation
    pub struct HandlerConfiguration {}
}
pub mod init_heap_symbols {
    // Dummy implementation
    pub struct HeapSymbols {}
}
pub mod init_setup_isolate {
    // Dummy implementation
    pub struct SetupIsolate {}
}
pub mod interpreter_interpreter {
    // Dummy implementation
    pub struct Interpreter {}
}
pub mod objects_arguments {
    // Dummy implementation
    pub struct Arguments {}
}
pub mod objects_call_site_info {
    // Dummy implementation
    pub struct CallSiteInfo {}
}
pub mod objects_cell_inl {
    // Dummy implementation
    pub struct Cell {}
}
pub mod objects_contexts {
    // Dummy implementation
    pub struct Contexts {}
}
pub mod objects_data_handler {
    // Dummy implementation
    pub struct DataHandler {}
}
pub mod objects_debug_objects {
    // Dummy implementation
    pub struct DebugObjects {}
}
pub mod objects_descriptor_array {
    // Dummy implementation
    pub struct DescriptorArray {}
}
pub mod objects_dictionary {
    // Dummy implementation
    pub struct Dictionary {}
}
pub mod objects_foreign {
    // Dummy implementation
    pub struct Foreign {}
}
pub mod objects_heap_number {
    // Dummy implementation
    pub struct HeapNumber {}
}
pub mod objects_instance_type_inl {
    // Dummy implementation
    pub struct InstanceTypeInl {}
}
pub mod objects_instance_type {
    // Dummy implementation
    pub struct InstanceType {}
}
pub mod objects_js_atomics_synchronization {
    // Dummy implementation
    pub struct JsAtomicsSynchronization {}
}
pub mod objects_js_generator {
    // Dummy implementation
    pub struct JsGenerator {}
}
pub mod objects_js_shared_array {
    // Dummy implementation
    pub struct JsSharedArray {}
}
pub mod objects_js_weak_refs {
    // Dummy implementation
    pub struct JsWeakRefs {}
}
pub mod objects_literal_objects_inl {
    // Dummy implementation
    pub struct LiteralObjectsInl {}
}
pub mod objects_lookup_cache {
    // Dummy implementation
    pub struct LookupCache {}
}
pub mod objects_map {
    // Dummy implementation
    pub struct Map {}
}
pub mod objects_microtask {
    // Dummy implementation
    pub struct Microtask {}
}
pub mod objects_objects_inl {
    // Dummy implementation
    pub struct ObjectsInl {}
}
pub mod objects_oddball_inl {
    // Dummy implementation
    pub struct OddballInl {}
}
pub mod objects_ordered_hash_table {
    // Dummy implementation
    pub struct OrderedHashTable {}
}
pub mod objects_promise {
    // Dummy implementation
    pub struct Promise {}
}
pub mod objects_property_descriptor_object {
    // Dummy implementation
    pub struct PropertyDescriptorObject {}
}
pub mod objects_script {
    // Dummy implementation
    pub struct Script {}
}
pub mod objects_shared_function_info {
    // Dummy implementation
    pub struct SharedFunctionInfo {}
}
pub mod objects_smi {
    // Dummy implementation
    pub struct Smi {}
}
pub mod objects_source_text_module {
    // Dummy implementation
    pub struct SourceTextModule {}
}
pub mod objects_string {
    // Dummy implementation
    pub struct String {}
}
pub mod objects_synthetic_module {
    // Dummy implementation
    pub struct SyntheticModule {}
}
pub mod objects_template_objects_inl {
    // Dummy implementation
    pub struct TemplateObjectsInl {}
}
pub mod objects_templates {
    // Dummy implementation
    pub struct Templates {}
}
pub mod objects_torque_defined_classes_inl {
    // Dummy implementation
    pub struct TorqueDefinedClassesInl {}
}
pub mod objects_turbofan_types {
    // Dummy implementation
    pub struct TurbofanTypes {}
}
pub mod objects_turboshaft_types {
    // Dummy implementation
    pub struct TurboshaftTypes {}
}
pub mod regexp_regexp {
    // Dummy implementation
    pub struct RegExp {}
}
pub mod roots_roots {
    // Dummy implementation
    pub struct Roots {}
}
pub mod utils_allocation {
    // Dummy implementation
    pub struct Allocation {}
}

pub mod wasm_wasm_objects {
    // Dummy implementation
    pub struct WasmObjects {}
}

use std::convert::TryInto;
use std::ptr::null_mut;
// Dummy implementations for V8 types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceType {
    JS_OBJECT_TYPE,
    STRING_TYPE,
    SYMBOL_TYPE,
    MAP_TYPE,
    HEAP_NUMBER_TYPE,
    BIGINT_TYPE,
    ODDBALL_TYPE,
    FIXED_ARRAY_TYPE,
    DESCRIPTOR_ARRAY_TYPE,
    SCOPE_INFO_TYPE,
    BYTE_ARRAY_TYPE,
    SHARED_FUNCTION_INFO_TYPE,
    CODE_TYPE,
    FOREIGN_TYPE,
    WEAK_FIXED_ARRAY_TYPE,
    WEAK_ARRAY_LIST_TYPE,
    JS_SHARED_ARRAY_TYPE,
    JS_ATOMICS_MUTEX_TYPE,
    JS_ATOMICS_CONDITION_TYPE,
    FIXED_DOUBLE_ARRAY_TYPE,
    FEEDBACK_METADATA_TYPE,
    BYTECODE_ARRAY_TYPE,
    PROPERTY_ARRAY_TYPE,
    SMALL_ORDERED_HASH_MAP_TYPE,
    SMALL_ORDERED_HASH_SET_TYPE,
    INTERPRETER_DATA_TYPE,
    TRANSITION_ARRAY_TYPE,
    HASH_TABLE_TYPE,
    ORDERED_NAME_DICTIONARY_TYPE,
    NAME_DICTIONARY_TYPE,
    REGISTERED_SYMBOL_TABLE_TYPE,
    ARRAY_LIST_TYPE,
    ACCESSOR_INFO_TYPE,
    SHARED_FUNCTION_INFO_WRAPPER_TYPE,
    HOLE_TYPE,
    TRUSTED_FOREIGN_TYPE,
    SCRIPT_CONTEXT_TABLE_TYPE,
    OBJECT_BOILERPLATE_DESCRIPTION_TYPE,
    WEAK_CELL_TYPE,
    SMALL_ORDERED_NAME_DICTIONARY_TYPE,
    MEGA_DOM_HANDLER_TYPE,
    EPHEMERON_HASH_TABLE_TYPE,
    SIMPLE_NUMBER_DICTIONARY_TYPE,
    NAME_TO_INDEX_HASH_TABLE_TYPE,
    EMBEDDER_DATA_ARRAY_TYPE,
    STRING_TYPE_FIRST,
    STRING_TYPE_LAST,
    FIXED_COW_ARRAY_TYPE,
    PROTECTED_FIXED_ARRAY_TYPE,
    TRUSTED_FIXED_ARRAY_TYPE,
    TRUSTED_WEAK_FIXED_ARRAY_TYPE,
    PROTECTED_WEAK_FIXED_ARRAY_TYPE,
    CELL_TYPE,
    PROPERTY_CELL_TYPE,
    FREE_SPACE_TYPE,
    GLOBAL_DICTIONARY_TYPE,
    NUMBER_DICTIONARY_TYPE,
    SWISS_NAME_DICTIONARY_TYPE,
    COVERAGE_INFO_TYPE,
    REG_EXP_MATCH_INFO_TYPE,
    JS_MESSAGE_OBJECT_TYPE,
    JS_EXTERNAL_OBJECT_TYPE,
    DICTIONARY_TEMPLATE_INFO_TYPE,
    SOURCE_TEXT_MODULE_TYPE,
    SYNTHETIC_MODULE_TYPE,
    CONTEXT_SIDE_PROPERTY_CELL_TYPE,
    IR_REG_EXP_DATA_TYPE,
    WASM_IMPORT_DATA_TYPE,
    WASM_CAPI_FUNCTION_DATA_TYPE,
    WASM_EXPORTED_FUNCTION_DATA_TYPE,
    WASM_INTERNAL_FUNCTION_TYPE,
    WASM_FUNC_REF_TYPE,
    WASM_JS_FUNCTION_DATA_TYPE,
    WASM_RESUME_DATA_TYPE,
    WASM_SUSPENDER_OBJECT_TYPE,
    WASM_TYPE_INFO_TYPE,
    WASM_CONTINUATION_OBJECT_TYPE,
    WASM_NULL_TYPE,
    ATOM_REG_EXP_DATA_TYPE,
    TRUSTED_BYTE_ARRAY_TYPE,
    WASM_TRUSTED_INSTANCE_DATA_TYPE,
    WASM_DISPATCH_TABLE_TYPE,
    FILLER_TYPE,
    FEEDBACK_CELL_TYPE,
    FEEDBACK_VECTOR_TYPE,
    PREPARSE_DATA_TYPE,
    LAST_JS_OBJECT_TYPE,
    IR_TYPE,
    WASM_STRUCT_TYPE,
    WASM_ARRAY_TYPE,
    REG_EXP_DATA_TYPE,
    LAST_TYPE,
    MEGA_DOM_HANDLER_TYPE_VALUE,
    INVALID_TYPE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementsKind {
    PACKED_SMI_ELEMENTS,
    HOLEY_DOUBLE_ELEMENTS,
    TERMINAL_FAST_ELEMENTS_KIND,
    DICTIONARY_ELEMENTS,
    SHARED_ARRAY_ELEMENTS,
    PACKED_ELEMENTS,
    HOLEY_ELEMENTS,
    DOUBLE_ELEMENTS,
    HOLEY_SMI_ELEMENTS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationType {
    kYoung,
    kOld,
    kReadOnly,
    kMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootIndex {
    kUndefinedValue,
    kNullValue,
    kEmptyString,
    kFalseValue,
    kTrueValue,
    kStringMap,
    kSymbolMap,
    kMetaMap,
    kUndefinedMap,
    kNullMap,
    kBooleanMap,
    kHeapNumberMap,
    kBigintMap,
    kFixedArrayMap,
    kTrustedFixedArrayMap,
    kProtectedFixedArrayMap,
    kWeakFixedArrayMap,
    kTrustedWeakFixedArrayMap,
    kProtectedWeakFixedArrayMap,
    kWeakArrayListMap,
    kDescriptorArrayMap,
    kHoleMap,
    kEnumCacheMap,
    kFixedCOWArrayMap,
    kOnePointerFillerMap,
    kTwoPointerFillerMap,
    kBytearrayMap,
    kForeignMap,
    kFixedDoubleArrayMap,
    kFeedbackMetadataMap,
    kByteArrayMap,
    kTrustedByteArrayMap,
    kBytecodeArrayMap,
    kFreeSpaceMap,
    kPropertyArrayMap,
    kSmallOrderedHashMapMap,
    kSmallOrderedHashSetMap,
    kInstructionStreamMap,
    kCellMap,
    kPropertyCellMap,
    kGlobalPropertyCellMap,
    kNoClosuresCellMap,
    kOneClosureCellMap,
    kManyClosuresCellMap,
    kTransitionArrayMap,
    kHashTableMap,
    kOrderedNameDictionaryMap,
    kNameDictionaryMap,
    kRegisteredSymbolTableMap,
    kArrayListMap,
    kAccessorInfoMap,
    kPreparseDataMap,
    kSharedFunctionInfoMap,
    kCodeMap,
    kModuleInfoMap,
    kClosureFeedbackCellArrayMap,
    kFeedbackVectorMap,
    kInvalidPrototypeValidityCell,
    kTheHoleValue,
    kException,
    kArrayBoilerplateDescriptionMap,
    kMinusZeroValue,
    kNanValue,
    kHoleNanValue,
    kInfinityValue,
    kMinusInfinityValue,
    kMaxSafeInteger,
    kMaxUInt32,
    kSmiMinValue,
    kSmiMaxValuePlusOne,
    kEmptyPropertyDictionary,
    kEmptyOrderedPropertyDictionary,
    kEmptyByteArray,
    kEmptyScopeInfo,
    kEmptyPropertyArray,
    kEmptyWeakFixedArray,
    kEmptyArrayList,
    kEmptyEnumCache,
    kEmptyDescriptorArray,
    kGlobalThisBindingScopeInfo,
    kEmptyFunctionScopeInfo,
    kNativeScopeInfo,
    kShadowRealmScopeInfo,
    kManyClosuresCell,
    kStringToSymbolKey,
    kBigIntToWrappedBigInt,
    kObjectToNumber,
    kObjectToString,
    kPromiseToStringTagSymbol,
    kSymbolToStringTagSymbol,
    kAsyncIteratorSymbol,
    kPromiseResolveSymbol,
    kPromiseRejectSymbol,
    kPromiseFinallySymbol,
    kIteratorSymbol,
    kToStringTagSymbol,
    kUnscopablesSymbol,
    kHasInstanceSymbol,
    kIsConcatSpreadableSymbol,
    kMatchAllSymbol,
    kReplaceAllSymbol,
    kAsyncDisposeSymbol,
    kPrivateBrand,
    kNativeContext,
    kPromiseAllResolveElementClosureSharedFun,
    kPromiseCapabilityDefaultResolveSharedFun,
    kPromiseCapabilityDefaultRejectSharedFun,
    kPromiseGetCapabilitiesExecutorSharedFun,
    kPromiseThenFinallySharedFun,
    kPromiseCatchFinallySharedFun,
    kPromiseValueThunkFinallySharedFun,
    kPromiseThrowerFinallySharedFun,
    kProxyRevokeSharedFun,
    kShadowRealmImportValueFulfilledSharedFun,
    kArrayFromAsyncIterableOnFulfilledSharedFun,
    kArrayFromAsyncIterableOnRejectedSharedFun,
    kArrayFromAsyncArrayLikeOnFulfilledSharedFun,
    kArrayFromAsyncArrayLikeOnRejectedSharedFun,
    kArrayIteratorProtector,
    kFastApiCallbackOptionsSymbol,
    kAtomicsMutexAsyncUnlockResolveHandlerSfi,
    kAtomicsMutexAsyncUnlockRejectHandlerSfi,
    kInternalizedToPrimitiveSymbol,
    kPrivateToStringTagSymbol,
    kFirstReadOnlyRoot,
    kLastReadOnlyRoot,
    kUndefined,
    kNull,
    kBoolean,
    kNumber,
    kString,
    kSymbol,
    kBigInt,
    kObject,
    kMegaDomHandler,
    kExternal,
    kInternalizedToPrimitive,
    kPrivateToStringTag,
    kAtomRegExpData,
    kIrRegExpData,
    kWasmNull,
    kWasmImportData,
    kWasmCapiFunctionData,
    kWasmExportedFunctionData,
    kWasmInternalFunction,
    kWasmFuncRef,
    kWasmJSFunctionData,
    kWasmResumeData,
    kWasmSuspenderObject,
    kWasmTypeInfo,
    kWasmContinuationObject,
    kGlobalContextSidePropertyCell,
    kScriptContextTable,
    kEmptyObjectBoilerplateDescription,
    kToString,
    kValueOf,
    kToPrimitive,
    kArguments,
    kWasmDispatchTable,
    kWasmTrustedInstanceData,
    kAsyncFunctionAwaitRejectClosureSharedFun,
    kAsyncFunctionAwaitResolveClosureSharedFun,
    kArrayFromAsyncIterable,
    kAsyncGeneratorAwaitResolveClosureSharedFun,
    kAsyncGeneratorAwaitRejectClosureSharedFun,
    kAsyncGeneratorYieldWithAwaitResolveClosureSharedFun,
    kAsyncGeneratorReturnResolveClosureSharedFun,
    kAsyncGeneratorReturnClosedResolveClosureSharedFun,
    kAsyncGeneratorReturnClosedRejectClosureSharedFun,
    kEmptySwissPropertyDictionary,
    kAsyncDisposeSymbolKey,
    kAtomicsConditionAcquireLockSfi,
    kToStringTag,
    kLastStringRoot,
    kWasmNullPadding,
    kWasmFirstInternalType,
    kWasmLastInternalType,
    kWasmExportedFunctionDataFirstExport,
    kWasmExportedFunctionDataLastExport,
    kWasmExportedFunctionDataAllExports,
    kOnePointerFiller,
    kTwoPointerFiller,
    kCell,
    kGlobalPropertyCell,
    kFreeSpace,
    kFeedbackCell,
    kTransitionArray,
    kHashTable,
    kOrderedNameDictionary,
    kNameDictionary,
    kRegisteredSymbolTable,
    kArrayList,
    kAccessorInfo,
    kPreparseData,
    kSharedFunctionInfo,
    kCode,
    kModuleInfo,
    kClosureFeedbackCellArray,
    kFeedbackVector,
    kMegaDomHandlerMap,
    kForeign,
    kFixedDoubleArray,
    kFeedbackMetadata,
    kByteArray,
    kTrustedByteArray,
    kBytecodeArray,
    kPropertyArray,
    kSmallOrderedHashMap,
    kSmallOrderedHashSet,
    kSimpleNumberDictionary,
    kNameToIndexHashTable,
    kEmbedderDataArray,
    kEphemeronHashTable,
    kNumberDictionary,
    kSwissNameDictionary,
    kGlobalDictionary,
    kScriptContextTableMap,
    kObjectBoilerplateDescriptionMap,
    kWasmNullMap,
    kProtectedFixedArray,
    kProtectedWeakFixedArray,
    kTrustedFixedArray,
    kTrustedWeakFixedArray,
    kTrustedByteArrayMap,
    kWasmTrustedInstanceDataMap,
    kWasmDispatchTableMap,
    kAsyncIteratorValueUnwrapSharedFun,
    kAsyncFromSyncIteratorCloseSyncAndRethrowSharedFun,
    kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun,
    kFirstImportantPrivateSymbol,
    kLastImportantPrivateSymbol,
    kFirstString,
    kFirstPrivateSymbol,
    kLastPrivateSymbol,
    kFirstPublicSymbol,
    kLastPublicSymbol,
    kAllNumbersCache,
    kAsyncDisposableStackOnFulfilledSharedFun,
    kAsyncDisposableStackOnRejectedSharedFun,
    kAsyncDisposeFromSyncDisposeSharedFun,
    kAtomicsMutexAsyncUnlockResolveHandlerSfiKey,
    kAtomicsMutexAsyncUnlockRejectHandlerSfiKey,
    kLastSymbol,
    kStringSplitCache,
    kRegExpMultipleCache,
    kRegExpMatchGlobalAtomCache,
    kDetachedContexts,
    kFeedbackVectorsForProfilingTools,
    kFunctionsMarkedForManualOptimization,
    kSharedWasmMemories,
    kLocalsBlockListCache,
    kActiveContinuation,
    kActiveSuspender,
    kJsToWasmWrappers,
    kWasmCanonicalRtts,
    kScriptList,
    kMaterializedObjects,
    kLastScriptId,
    kLastDebuggingId,
    kLastStackTraceId,
    kNextTemplateSerialNumber,
    kEmptyScript,
    kArrayBufferDetachingProtector,
    kArrayConstructorProtector,
    kArraySpeciesProtector,
    kMapIteratorProtector,
    kNoProfilingProtector,
    kPromiseHookProtector,
    kPromiseResolveProtector,
    kPromiseSpeciesProtector,
    kPromiseThenProtector,
    kRegexpSpeciesProtector,
    kSetIteratorProtector,
    kStringIteratorProtector,
    kStringLengthProtector,
    kTypedArrayLengthProtector,
    kTypedArraySpeciesProtector,
    kNoUndetectableObjectsProtector,
    kStringWrapperToPrimitiveProtector,
    kEmptySymbolTable,
    kEmptyOrderedHashMap,
    kEmptyOrderedHashSet,
    kEmptyFeedbackMetadata,
    kEmptySlowElementDictionary,
    kNumberStringNotRegexpLikeProtector,
    kEmptyTrustedByteArray,
    kEmptyTrustedFixedArray,
    kEmptyTrustedWeakFixedArray,
    kMinusZero,
    kSingleCharacterStringTable,
    kPublicSymbolTable,
    kApiSymbolTable,
    kApiPrivateSymbolTable,
    kDetachedContextMap,
    kDetachedContextWeakSet,
    kPropertyCellHoleValue,
    kHoleHashTableValue,
    kPromiseHoleValue,
    kUninitializedValue,
    kOptimizedOut,
    kStaleRegister,
    kArgumentsMarker,
    kTerminationException,
    kSelfReferenceMarker,
    kBasicBlockCountersMarker,
    kErrorStackGetterFunTemplate,
    kErrorStackSetterFunTemplate,
    kFirstReadOnlyRootForIteration,
    kLastReadOnlyRootForIteration,
    kAsyncIteratorSymbolKey,
    kWasmNullPaddingKey,
    kAsyncFromSyncIteratorResultValueKey,
    kAsyncFromSyncIteratorContinuationKey,
    kCallAsyncModuleRejectedSfiKey,
    kNativeContextKey,
    kKeyToStringTag,
    kSymbolInternal,
    kAsyncFromSyncIteratorIsDoneKey,
    kInternalToPrimitiveKey,
    kNewObjectKey,
    kFirstReadWriteRoot,
    kLastReadWriteRoot,
    kFirstApiConstant,
    kLastApiConstant,
    kFirstPrivateRoot,
    kToThisString,
    kSymbol,
    kFirstCodeConstant,
    kLastCodeConstant,
    kFirstFixedArrayConstant,
    kArrayIteratorNextSharedFun,
    kArrayIteratorNextCode,
    kArrayBufferDetachingProtectorKey,
    kArraySpeciesProtectorKey,
    kTypedArraySpeciesProtectorKey,
    kFirstReadOnlyRootWithAddress,
    kIsConcatSpreadableProtectorKey,
    kStringLengthProtectorKey,
    kMaxNumberOfProtectedRoots,
    kLastFixedArrayConstant,
    kIsConcatSpreadable,
    kAsyncDispose,
    kPromiseHook,
    kMicrotask,
    kSetIteratorProtectorKey,
    kWeakRefsKeepDuringJob,
    kMegaDomProtectorKey,
    kObjectToSymbolKey,
    kWasmFirstWasmInternal,
    kUndefinedValue,
    kNullValue,
    kEmptyString,
    kFalseValue,
    kTrueValue,
    kMetaMapKey,
    kMapFirstPrivateSymbol,
    kDetachedContextMapKey,
    kFixedArrayType,
    kWeakFixedArrayType,
    kArrayListType,
    kArrayFromAsyncSFI,
    kNoUndetectableObjectsProtectorKey,
    kSmallOrderedHashSetMap,
    kHole,
    kLastStringConst,
    kMaxPretenuredMemoryGeneration,
    kSmiZeroKey,
    kWasmTypeFirst,
    kWasmFirstLocalType,
    kWasmLastLocalType,
    kWasmToJsWrapper,
    kWasmTypeLast,
    kMapCacheLimit,
    kNoElementsProtectorKey,
    kWasmTypeIndexOffset,
    kWasmInternalTypes,
    kWasmJsFunctionDataType,
    kFirstWasmConstant,
    kWasmNullKey,
    kFixedArrayMapKey,
    kFirstNumberConstant,
    kHeapMaxConstant,
    kJsExternalObjectType,
    kNativeContextSymbol,
    kContextLocalCount,
    kSmiZero,
    kIsConcatenator,
    kIsDictionaryMapKey,
    kArraySpecies,
    kNoElements,
    kArrayConstructorProtectorKey,
    kNativeContextMapKey,
    kMaxHeapConstant,
    kOneCacheString,
    kInternalConstStrings,
    kWasmTypesCacheFirst,
    kHashSeed,
    kEmptyWeakFixedArrayKey,
    kLastWasmConst,
    kLastNumberConstant,
    kLastImportantStringConst,
    kFirstReadOnlyRootConstant,
    kNextAddress,
    kToStringTagKey,
    kWasmSuspenderObjectKey,
    kWasmFirstExternalType,
    kFirstWasmConstantKey,
    kWasmLastExternalType,
    kLastPublicWellKnownSymbolKey,
    kErrorStackGetterFunTemplateKey,
    kStringToStringNameKey,
    kProtectedFixedArrayType,
    kTrustedFixedArrayType,
    kTypedArrayLengthProtectorKey,
    kManyClosuresKey,
    kArrayIteratorProtectorKey,
    kWasmStackHandlerCode,
    kWasmStackHandlerOffset,
    kPromiseThenProtectorKey,
    kFirstCodeGenerated,
    kErrorStackSetterFunTemplateKey,
    kStringWrapperToPrimitiveProtectorKey,
    kMaxSafeIntegerKey,
    kWasmTypeLastKey,
    kStringIteratorProtectorKey,
    kProtectedWeakTableKey,
    kWasmInternalTypesKey,
    kNumberStringNotRegexpLikeProtectorKey,
    kPromiseThenFinallyCode,
    kFirstInternalPropertyKey,
    kToStringPrimitiveName,
    kNextPrototypeValidityCell,
    kMapDescriptorList,
    kMegaDomHandlerMapKey,
    kSmallOrderedHashSet,
    kKeyMapIndex,
    kDescriptorBitField,
    kCodeConstantsMap,
    kHashMaskOffset,
    kToBooleanBytecodeOffset,
    kWasmCodeFirstKey,
    kWasmGlobalContextKey,
    kSingleCharacterStringTableKey,
    kArrayJoinMap,
    kLastReadWriteKey,
    kToNumberName,
    kDescriptorFlagOffset,
    kFirstInternalProperty,
    kFirstFastApiCallback,
    kGlobalContextScopeInfo,
    kFirstInterestingSymbol,
    kExternalObjectType,
    kDetachedContextKey,
    kRegexpSpeciesProtectorKey,
    kNativeFunctionType,
    kFirstCodeConstantKey,
    kWasmTypesCacheLast,
    kNativeConstStrings,
    kSmiMinValueKey,
    kWeakCellValue,
    kApiConstantsList,
    kNoClosureKey,
    kAsyncFromSyncIteratorKey,
    kPromiseRejectCode,
    kDetachedContextFirst,
    kStringFromCharCode,
    kWasmToJsWrapperKey,
    kWasmStackHandler,
    kWasmEmptyArrayObject,
    kFirstCodeGeneratedKey,
    kHeapMax,
    kStringFromCharCodeName,
    kHashSeedKey,
    kPropertyAttributesOffset,
    kWasmJSFunctionName,
    kSmallOrderedHashSetKey,
    kAllWasmConstantsKey,
    kWasmExternalTypes,
    kContextTypeKey,
    kWasmEmptyTableObject,
    kToSymbolName,
    kWasmLastCodeConstant,
    kToStringLastKey,
    kInternalPropertiesTable,
    kObjectToNumberName,
    kPropertyAccessorOffset,
    kWasmFirstExternal,
    kApiConstantsListKey,
    kSetElementCode,
    kToStringTagSymbolKey,
    kDetachedContextLast,
    kUndefinedName,
    kNativeSymbolTableKey,
    kWasmFromCAPIKey,
    kWasmGCStructTypeFirst,
    kGlobalPropertiesMap,
    kIteratorName,
    kFunctionId,
    kWasmStackHandlerFunction,
    kWasmSuspenderObjectLast,
    kToBooleanMap,
    kEmptyArrayObject,
    kFromCharCodeLast,
    kInternalPropertiesKey,
    kWasmTypesLast,
    kPrivateBrandKey,
    kIsConcatSpreadableSymbolKey,
    kHasInstanceSymbolKey,
    kRegExpMultiple,
    kDetachedContextCode,
    kSetIteratorProtector,
    kExternalObjectTypeKey,
    kContextConstStrings,
    kFromCharCodeFirst,
    kWasmLastExternalTypeKey,
    kSetElementKey,
    kWasmJsEngine,
    kWasmExportedFunctionTableSize,
    kUndefinedMapKey,
    kDetachedContextConstant,
    kWeakMapNewKey,
    kSymbolToCodeMapCode,
    kSymbolToStringMap,
    kSetSizeName,
    kExternalStringKey,
    kWeakRefsKeepDuringJobKey,
    kWasmStructTypeKey,
    kFastArrayIteratorSymbol,
    kToStringCodeName,
    kMapClearCode,
    kWasmSuspenderKey,
    kWasmGcTagTable,
    kSymbolFirst,
    kMegaDomProtector,
    kMapSetCode,
    kEmptyConstantPoolKey,
    kIteratorNameKey,
    kWasmGlobalContextKey,
    kSmallOrderedHashMapType,
    kWasmStackHandlerSignature,
    kFirstInterestingSymbolsKey,
    kWasmFuncRef,
    kSetAddName,
    kSmiMax,
    kNull,
    kLastSymbolName,
    kHasInstanceName,
    kTrueName,
    kNativeFunctionMapKey,
    kFunctionFirst,
    kDescriptorKey,
    kExternalStringCode,
    kNativeFirst,
    kWasmCodeFirst,
    kWasmSignatureForTypeRef,
    kSetPrototype,
    kCodeByteArray,
    kFirstStrongPointerName,
    kStringFromCharCodeNameKey,
    kWasmMemoryCopyCode,
    kNumberDictionarySizeName,
    kWasmStackHandlerSignatures,
    kNumberToStringMap,
    kPrivateName,
    kWasmEmptyTableFirst,
    kCodeTableString,
    kWasmJsPromiseType,
    kFastArrayIteratorProto,
    kLastReadOnlyRootConstantKey,
    kCodeConstants,
    kExternalInternal,
    kMaxElementsOfFastElements,
    kJsSharedArray,
    kNativeConstructorName,
    kKeyList,
    kLastStringWellKnown,
    kObjectCode,
    kNoProfilingName,
    kWasmStructTypeLast,
    kWeakArrayValue,
    kWasmTypes,
    kFalseName,
    kArrayNext,
    kPromiseToStringCode,
    kStrongPointerLast,
    kMapLoadCode,
    kWasmFromCAPI,
    kNewObject,
    kWasmNull16,
    kNumberToString00,
    kWasmTypesCacheFirstKey,
    kWasmFirstConstant,
    kWasmEngineName,
    kWasmCodeEmpty,
    kFastArrayElementLoad,
    kExternalAddress,
    kPrototypeCode,
    kWasmFunctionKey,
    kWasmExternalTypesKey,
    kWasmCodeLastKey,
    kFirstReadWrite,
    kApiConstantsList,
    kNativeContextSymbolKey,
    kNullMapCode,
    kWasmTypeIndexOffsetKey,
    kContextConstructorMap,
    kDescriptorArrayType,
    kDescriptorKeyFirst,
    kPrototypeConstant,
    kMegaDomHandlerSFI,
    kWeakMapLast,
    kHeapMaxMaxSafeIntegerKey,
    kWasmConstantZero,
    kWeakCellKey,
    kPromiseThenCode,
    kFirstApiConstantKey,
    kGlobalContextMap,
    kJsPromiseRejectionTrackerMap,
    kSmiMapCode,
    kWasmMemoryAccessTrapCode,
    kApiConstStringsLast,
    kFeedbackVectorContext,
    kFeedbackSlotContext,
    kWasmEmptyTable,
    kWasmStackHandlerTable,
    kCodeTableConst,
    kGlobalContextSFI,
    kToStringNumberName,
    kWasmMemoryConst,
    kMapSetWeakCode,
    kWasmConstantMaxUInt32,
    kNewObjectCode,
    kWasmHeapOffsetCode,
    kLastExternalSymbol,
    kSymbolToStringName,
    kFromNumberStringCacheName,
    kWasmInternalTypesKey,
    kWasmLoadCode,
    kGlobalFirstPrivateSymbol,
    kNameKey,
    kWasmConstantZeroKey,
    kFirstPublicWellKnownSymbol,
    kArraySpeciesName,
    kPrototypeMapCode,
    kCodeForMapBitOffset,
    kWasmStackHandlerEntry0,
    kWasmStackHandlerEntry1,
    kWasmStackHandlerEntry2,
    kWasmMemoryCopyTable,
    kWasmHeapMaxMaxSafeInteger,
    kNewCode,
    kNumberIteratorNext,
    kStrongPointerFirst,
    kWasmGcArrayTypeFirst,
    kWasmGcArrayTypeLast,
    kLastPrivateConstant,
    kWasmFunctionMap,
    kNull16MapCode,
    kNewGlobalThis,
    kNumberToStringCache,
    kToPrivateName,
    kPublicWellKnownSymbolsMap,
    kWasmExternalTypesMap,
    kWasmModuleOffsetConstant,
    kFastClone,
    kWeakLast,
    kWasmSuspenderObjectValue,
    kPrototype,
    kWasmMemoryOffsetTable,
    kDescriptorName,
    kIsAccessorConstant,
    kWasmConstantMaxUInt32Key,
    kWasmInternalTypesConst,
    kPublicWellKnownLast,
    kFromCodeStringName,
    kPrivateSymbolDescription,
    kExternalStringCodeString,
    kWasmGcRefTypeFirst,
    kWasmConstantZeroOffset,
    kWasmJSFunctionNameKey,
    kUndefined,
    kWasmSuspenderStateSuspended,
    kFirstTemplateRoot,
    kTemplateRoots,
    kSmiMap,
    kFeedbackVectorCache,
    kPrototypeWellKnown,
    kPrivateString,
    kFastLoadMapCode,
    kToNumberCodeName,
    kWasmMemoryAddressTable,
    kFastElementsKind,
    kWasmCodeLast,
    kWasmEngineNameKey,
    kPublicSymbolsMap,
    kWasmExportFunctionTag,
    kWasmCodeArrayConstant,
    kNull00Key,
    kFastLoadElement,
    kWasmTypesCacheLastKey,
    kGlobalMapValue,
    kStrongPointerSFIConstant,
    kLastStringKey,
    kHeapObjectKey,
    kLastImportantPublicSymbolKey,
    kSmiZeroKey,
    kFunctionKindCode,
    kWasmTypeFirstKey,
    kWasmEmptyTableType,
    kWasmMemoryOffsetConstantKey,
    kJsMessageObjectKey,
    kWasmContextOffsetConstant,
    kToStringThisCode,
    kUndefinedNameKey,
    kGlobalKeyNames,
    kToBooleanKey,
    kWasmGCStructTypeFirstKey,
    kExternalAddressConst,
    kStringMapKey,
    kWasmStructMap,
    kWasmStructZero,
    kFromCharCode00,
    kNumberStringCacheString,
    kWasmInternalTypesString,
    kWasmMemoryAccessTrapConst,
    kWasmGCHeapFirst,
    kWasmJSValueTypes,
    kWasmGcTagTableString,
    kWasmEmptyTableOffset,
    kTemplateSymbolKey,
    kPromiseHoleConst,
    kLastTemplateRoot,
    kWasmSuspenderObjectSFI,
    kToNumberMapKey,
    kWeakRefsKeepDuringJobSFI,
    kWasmSuspenderFirst,
    kLastExternalSymbolKey,
    kWasmI31Zero,
    kWasmMemory0,
    kWasmGlobalConstantsFirst,
    kWasmSuspenderConstants,
    kW

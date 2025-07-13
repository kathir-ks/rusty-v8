// Converted from V8 C++ source files:
// Header: object-list-macros.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod object_list_macros {
    //use crate::base::macros::*; // For IF_WASM.
    //use crate::torque_generated::instance_types::*;

    pub mod v8 {
        pub mod internal {
            pub struct ArrayList {}
            pub struct ByteArray {}
            pub struct ClosureFeedbackCellArray {}
            pub struct FixedArray {}
            pub struct FixedDoubleArray {}
            pub struct ObjectBoilerplateDescription {}
            pub struct RegExpMatchInfo {}
            pub struct ScriptContextTable {}
            pub struct WeakFixedArray {}
            pub struct BigInt {}
            pub struct BytecodeArray {}
            pub struct Code {}
            pub struct Context {}
            pub struct ExternalString {}
            pub struct FeedbackMetadata {}
            pub struct FeedbackVector {}
            pub struct FreeSpace {}
            pub struct InstructionStream {}
            pub struct PreparseData {}
            pub struct PropertyArray {}
            pub struct ProtectedFixedArray {}
            pub struct ProtectedWeakFixedArray {}
            pub struct ScopeInfo {}
            pub struct SeqString {}
            pub struct SloppyArgumentsElements {}
            pub struct SwissNameDictionary {}
            pub struct ThinString {}
            pub struct TrustedByteArray {}
            pub struct TrustedFixedArray {}
            pub struct TrustedWeakFixedArray {}
            pub struct UncompiledDataWithoutPreparseData {}
            pub struct WeakArrayList {}
            pub struct WasmArray {}
            pub struct WasmDispatchTable {}
            pub struct WasmStruct {}
            pub struct AbstractCode {}
            pub struct AccessorInfo {}
            pub struct AllocationSite {}
            pub struct BigIntBase {}
            pub struct Boolean {}
            pub struct Cell {}
            pub struct CompilationCacheTable {}
            pub struct ConsString {}
            pub struct ContextSidePropertyCell {}
            pub struct CoverageInfo {}
            pub struct DataHandler {}
            pub struct DeoptimizationData {}
            pub struct DependentCode {}
            pub struct DescriptorArray {}
            pub struct DictionaryTemplateInfo {}
            pub struct EmbedderDataArray {}
            pub struct EphemeronHashTable {}
            pub struct ExternalOneByteString {}
            pub struct ExternalTwoByteString {}
            pub struct FeedbackCell {}
            pub struct FunctionTemplateInfo {}
            pub struct FixedArrayBase {}
            pub struct FixedArrayExact {}
            pub struct Foreign {}
            pub struct GcSafeCode {}
            pub struct GlobalDictionary {}
            pub struct HeapNumber {}
            pub struct InternalizedString {}
            pub struct JSArgumentsObject {}
            pub struct JSArray {}
            pub struct JSArrayBuffer {}
            pub struct JSArrayBufferView {}
            pub struct JSArrayIterator {}
            pub struct JSAsyncFromSyncIterator {}
            pub struct JSAsyncFunctionObject {}
            pub struct JSAsyncGeneratorObject {}
            pub struct JSAtomicsCondition {}
            pub struct JSAtomicsMutex {}
            pub struct JSBoundFunction {}
            pub struct JSCollection {}
            pub struct JSCollectionIterator {}
            pub struct JSCustomElementsObject {}
            pub struct JSDataView {}
            pub struct JSDataViewOrRabGsabDataView {}
            pub struct JSDate {}
            pub struct JSDisposableStackBase {}
            pub struct JSSyncDisposableStack {}
            pub struct JSAsyncDisposableStack {}
            pub struct JSExternalObject {}
            pub struct JSFinalizationRegistry {}
            pub struct JSFunction {}
            pub struct JSFunctionOrBoundFunctionOrWrappedFunction {}
            pub struct JSGeneratorObject {}
            pub struct JSGlobalObject {}
            pub struct JSGlobalProxy {}
            pub struct JSIteratorHelper {}
            pub struct JSIteratorFilterHelper {}
            pub struct JSIteratorMapHelper {}
            pub struct JSIteratorTakeHelper {}
            pub struct JSIteratorDropHelper {}
            pub struct JSIteratorFlatMapHelper {}
            pub struct JSMap {}
            pub struct JSMapIterator {}
            pub struct JSMessageObject {}
            pub struct JSModuleNamespace {}
            pub struct JSObject {}
            pub struct JSAPIObjectWithEmbedderSlots {}
            pub struct JSObjectWithEmbedderSlots {}
            pub struct JSPrimitiveWrapper {}
            pub struct JSPromise {}
            pub struct JSProxy {}
            pub struct JSRabGsabDataView {}
            pub struct JSRawJson {}
            pub struct JSReceiver {}
            pub struct JSRegExp {}
            pub struct JSRegExpStringIterator {}
            pub struct JSSet {}
            pub struct JSSetIterator {}
            pub struct JSShadowRealm {}
            pub struct JSSharedArray {}
            pub struct JSSharedStruct {}
            pub struct JSSpecialObject {}
            pub struct JSStringIterator {}
            pub struct JSSynchronizationPrimitive {}
            pub struct JSTemporalCalendar {}
            pub struct JSTemporalDuration {}
            pub struct JSTemporalInstant {}
            pub struct JSTemporalPlainDate {}
            pub struct JSTemporalPlainTime {}
            pub struct JSTemporalPlainDateTime {}
            pub struct JSTemporalPlainMonthDay {}
            pub struct JSTemporalPlainYearMonth {}
            pub struct JSTemporalTimeZone {}
            pub struct JSTemporalZonedDateTime {}
            pub struct JSTypedArray {}
            pub struct JSValidIteratorWrapper {}
            pub struct JSWeakCollection {}
            pub struct JSWeakRef {}
            pub struct JSWeakMap {}
            pub struct JSWeakSet {}
            pub struct JSWrappedFunction {}
            pub struct LoadHandler {}
            pub struct Map {}
            pub struct MegaDomHandler {}
            pub struct Module {}
            pub struct Microtask {}
            pub struct Name {}
            pub struct NameDictionary {}
            pub struct NameToIndexHashTable {}
            pub struct NativeContext {}
            pub struct NormalizedMapCache {}
            pub struct NumberDictionary {}
            pub struct ObjectHashSet {}
            pub struct ObjectHashTable {}
            pub struct ObjectTemplateInfo {}
            pub struct ObjectTwoHashTable {}
            pub struct Oddball {}
            pub struct Hole {}
            pub struct OrderedHashMap {}
            pub struct OrderedHashSet {}
            pub struct OrderedNameDictionary {}
            pub struct PrimitiveHeapObject {}
            pub struct PromiseReactionJobTask {}
            pub struct PropertyCell {}
            pub struct SeqOneByteString {}
            pub struct SharedFunctionInfo {}
            pub struct SimpleNumberDictionary {}
            pub struct SlicedString {}
            pub struct SmallOrderedHashMap {}
            pub struct SmallOrderedHashSet {}
            pub struct SmallOrderedNameDictionary {}
            pub struct SourceTextModule {}
            pub struct SourceTextModuleInfo {}
            pub struct StoreHandler {}
            pub struct String {}
            pub struct StringSet {}
            pub struct RegisteredSymbolTable {}
            pub struct Struct {}
            pub struct Symbol {}
            pub struct SyntheticModule {}
            pub struct TemplateInfo {}
            pub struct TemplateInfoWithProperties {}
            pub struct TemplateLiteralObject {}
            pub struct TransitionArray {}
            pub struct TurboshaftFloat64RangeType {}
            pub struct TurboshaftFloat64SetType {}
            pub struct TurboshaftFloat64Type {}
            pub struct TurboshaftType {}
            pub struct TurboshaftWord32RangeType {}
            pub struct TurboshaftWord32SetType {}
            pub struct TurboshaftWord32Type {}
            pub struct TurboshaftWord64RangeType {}
            pub struct TurboshaftWord64SetType {}
            pub struct TurboshaftWord64Type {}
            pub struct WasmContinuationObject {}
            pub struct WasmExceptionPackage {}
            pub struct WasmFuncRef {}
            pub struct WasmGlobalObject {}
            pub struct WasmInstanceObject {}
            pub struct WasmMemoryObject {}
            pub struct WasmMemoryMapDescriptor {}
            pub struct WasmModuleObject {}
            pub struct WasmNull {}
            pub struct WasmObject {}
            pub struct WasmResumeData {}
            pub struct WasmSuspenderObject {}
            pub struct WasmSuspendingObject {}
            pub struct WasmTableObject {}
            pub struct WasmTagObject {}
            pub struct WasmTypeInfo {}
            pub struct WasmValueObject {}
            pub struct WeakCell {}
            pub struct AccessCheckNeeded {}
            pub struct AlwaysSharedSpaceJSObject {}
            pub struct BigIntWrapper {}
            pub struct BooleanWrapper {}
            pub struct Callable {}
            pub struct Constructor {}
            pub struct Filler {}
            pub struct HandlerTable {}
            pub struct JSContextExtensionObject {}
            pub struct JSError {}
            pub struct MapCache {}
            pub struct NumberWrapper {}
            pub struct OSROptimizedCodeCache {}
            pub struct ScriptWrapper {}
            pub struct StringWrapper {}
            pub struct SymbolWrapper {}
            pub struct UniqueName {}
            pub struct Undetectable {}
            pub struct JSApiObject {}
            pub struct JSClassConstructor {}
            pub struct JSLastDummyApiObject {}
            pub struct JSPromiseConstructor {}
            pub struct JSArrayConstructor {}
            pub struct JSRegExpConstructor {}
            pub struct JSMapKeyIterator {}
            pub struct JSMapKeyValueIterator {}
            pub struct JSMapValueIterator {}
            pub struct JSSetKeyValueIterator {}
            pub struct JSSetValueIterator {}
            pub struct JSSpecialApiObject {}
            pub struct MaybeReadOnlyJSObject {}
            pub struct ModuleContext {}
            pub struct NonNullForeign {}
            pub struct ScriptContext {}
            pub struct WithContext {}
            pub struct JSInternalPrototypeBase {}
            pub struct JSObjectPrototype {}
            pub struct JSRegExpPrototype {}
            pub struct JSPromisePrototype {}
            pub struct JSSetPrototype {}
            pub struct JSIteratorPrototype {}
            pub struct JSArrayIteratorPrototype {}
            pub struct JSMapIteratorPrototype {}
            pub struct JSTypedArrayPrototype {}
            pub struct JSSetIteratorPrototype {}
            pub struct JSStringIteratorPrototype {}
            pub struct TypedArrayConstructor {}
            pub struct Uint8TypedArrayConstructor {}
            pub struct Int8TypedArrayConstructor {}
            pub struct Uint16TypedArrayConstructor {}
            pub struct Int16TypedArrayConstructor {}
            pub struct Uint32TypedArrayConstructor {}
            pub struct Int32TypedArrayConstructor {}
            pub struct Float16TypedArrayConstructor {}
            pub struct Float32TypedArrayConstructor {}
            pub struct Float64TypedArrayConstructor {}
            pub struct Uint8ClampedTypedArrayConstructor {}
            pub struct Biguint64TypedArrayConstructor {}
            pub struct Bigint64TypedArrayConstructor {}
            pub struct JSV8BreakIterator {}
            pub struct JSCollator {}
            pub struct JSDateTimeFormat {}
            pub struct JSDisplayNames {}
            pub struct JSDurationFormat {}
            pub struct JSListFormat {}
            pub struct JSLocale {}
            pub struct JSNumberFormat {}
            pub struct JSPluralRules {}
            pub struct JSRelativeTimeFormat {}
            pub struct JSSegmentDataObject {}
            pub struct JSSegmentDataObjectWithIsWordLike {}
            pub struct JSSegmentIterator {}
            pub struct JSSegmenter {}
            pub struct JSSegments {}
            pub struct HashTable {}
            pub struct AwaitContext {}
            pub struct BlockContext {}
            pub struct CatchContext {}
            pub struct DebugEvaluateContext {}
            pub struct EvalContext {}
            pub struct FreeSpaceOrFiller {}
            pub struct FunctionContext {}
            pub struct TrustedObject {}
            pub struct ExposedTrustedObject {}
            pub struct UncompiledData {}
            pub struct UncompiledDataWithPreparseData {}
            pub struct UncompiledDataWithPreparseDataAndJob {}
            pub struct UncompiledDataWithoutPreparseDataWithJob {}
            pub struct SharedFunctionInfoWrapper {}
            pub struct TrustedForeign {}
            pub struct AtomRegExpData {}
            pub struct IrRegExpData {}
            pub struct RegExpData {}
            pub struct WasmImportData {}
            pub struct WasmCapiFunctionData {}
            pub struct WasmExportedFunctionData {}
            pub struct WasmJSFunctionData {}
            pub struct WasmInternalFunction {}
            pub struct WasmTrustedInstanceData {}

            #[macro_export]
            macro_rules! simple_heap_object_list_generator {
                ($apply:ident, $v:ident) => {
                    $apply!($v, ArrayList, ARRAY_LIST);
                    $apply!($v, ByteArray, BYTE_ARRAY);
                    $apply!($v, ClosureFeedbackCellArray, CLOSURE_FEEDBACK_CELL_ARRAY);
                    $apply!($v, FixedArray, FIXED_ARRAY);
                    $apply!($v, FixedDoubleArray, FIXED_DOUBLE_ARRAY);
                    $apply!($v, ObjectBoilerplateDescription, OBJECT_BOILERPLATE_DESCRIPTION);
                    $apply!($v, RegExpMatchInfo, REG_EXP_MATCH_INFO);
                    $apply!($v, ScriptContextTable, SCRIPT_CONTEXT_TABLE);
                    $apply!($v, WeakFixedArray, WEAK_FIXED_ARRAY);
                };
            }

            #[macro_export]
            macro_rules! simple_heap_object_list1_adapter {
                ($v:ident, $name:ident, $NAME:ident) => {
                    $v!($name)
                };
            }

            #[macro_export]
            macro_rules! simple_heap_object_list1 {
                ($v:ident) => {
                    simple_heap_object_list_generator!(simple_heap_object_list1_adapter, $v)
                };
            }

            #[macro_export]
            macro_rules! simple_heap_object_list2_adapter {
                ($v:ident, $name:ident, $NAME:ident) => {
                    $v!($name, $NAME)
                };
            }

            #[macro_export]
            macro_rules! simple_heap_object_list2 {
                ($v:ident) => {
                    simple_heap_object_list_generator!(simple_heap_object_list2_adapter, $v)
                };
            }

            #[macro_export]
            macro_rules! dynamically_sized_heap_object_list {
                ($v:ident) => {
                    $v!(ArrayList);
                    $v!(BigInt);
                    $v!(ByteArray);
                    $v!(BytecodeArray);
                    $v!(ClosureFeedbackCellArray);
                    $v!(Code);
                    $v!(Context);
                    $v!(ExternalString);
                    $v!(FeedbackMetadata);
                    $v!(FeedbackVector);
                    $v!(FixedArray);
                    $v!(FixedDoubleArray);
                    $v!(FreeSpace);
                    $v!(InstructionStream);
                    $v!(ObjectBoilerplateDescription);
                    $v!(PreparseData);
                    $v!(PropertyArray);
                    $v!(ProtectedFixedArray);
                    $v!(ProtectedWeakFixedArray);
                    $v!(RegExpMatchInfo);
                    $v!(ScopeInfo);
                    $v!(ScriptContextTable);
                    $v!(SeqString);
                    $v!(SloppyArgumentsElements);
                    $v!(SwissNameDictionary);
                    $v!(ThinString);
                    $v!(TrustedByteArray);
                    $v!(TrustedFixedArray);
                    $v!(TrustedWeakFixedArray);
                    $v!(UncompiledDataWithoutPreparseData);
                    $v!(WeakArrayList);
                    $v!(WeakFixedArray);
                    $v!(WasmArray);
                    $v!(WasmDispatchTable);
                    $v!(WasmStruct);
                };
            }

            #[macro_export]
            macro_rules! heap_object_ordinary_type_list_base {
                ($v:ident) => {
                    $v!(AbstractCode);
                    $v!(AccessorInfo);
                    $v!(AllocationSite);
                    $v!(BigInt);
                    $v!(BigIntBase);
                    $v!(Boolean);
                    $v!(Cell);
                    $v!(CompilationCacheTable);
                    $v!(ConsString);
                    $v!(ContextSidePropertyCell);
                    $v!(Context);
                    $v!(CoverageInfo);
                    $v!(DataHandler);
                    $v!(DeoptimizationData);
                    $v!(DependentCode);
                    $v!(DescriptorArray);
                    $v!(DictionaryTemplateInfo);
                    $v!(EmbedderDataArray);
                    $v!(EphemeronHashTable);
                    $v!(ExternalOneByteString);
                    $v!(ExternalString);
                    $v!(ExternalTwoByteString);
                    $v!(FeedbackCell);
                    $v!(FeedbackMetadata);
                    $v!(FeedbackVector);
                    $v!(FunctionTemplateInfo);
                    $v!(FixedArrayBase);
                    $v!(FixedArrayExact);
                    $v!(Foreign);
                    $v!(FreeSpace);
                    $v!(GcSafeCode);
                    $v!(GlobalDictionary);
                    $v!(HeapNumber);
                    $v!(InternalizedString);
                    $v!(JSArgumentsObject);
                    $v!(JSArray);
                    $v!(JSArrayBuffer);
                    $v!(JSArrayBufferView);
                    $v!(JSArrayIterator);
                    $v!(JSAsyncFromSyncIterator);
                    $v!(JSAsyncFunctionObject);
                    $v!(JSAsyncGeneratorObject);
                    $v!(JSAtomicsCondition);
                    $v!(JSAtomicsMutex);
                    $v!(JSBoundFunction);
                    $v!(JSCollection);
                    $v!(JSCollectionIterator);
                    $v!(JSCustomElementsObject);
                    $v!(JSDataView);
                    $v!(JSDataViewOrRabGsabDataView);
                    $v!(JSDate);
                    $v!(JSDisposableStackBase);
                    $v!(JSSyncDisposableStack);
                    $v!(JSAsyncDisposableStack);
                    $v!(JSExternalObject);
                    $v!(JSFinalizationRegistry);
                    $v!(JSFunction);
                    $v!(JSFunctionOrBoundFunctionOrWrappedFunction);
                    $v!(JSGeneratorObject);
                    $v!(JSGlobalObject);
                    $v!(JSGlobalProxy);
                    $v!(JSIteratorHelper);
                    $v!(JSIteratorFilterHelper);
                    $v!(JSIteratorMapHelper);
                    $v!(JSIteratorTakeHelper);
                    $v!(JSIteratorDropHelper);
                    $v!(JSIteratorFlatMapHelper);
                    $v!(JSMap);
                    $v!(JSMapIterator);
                    $v!(JSMessageObject);
                    $v!(JSModuleNamespace);
                    $v!(JSObject);
                    $v!(JSAPIObjectWithEmbedderSlots);
                    $v!(JSObjectWithEmbedderSlots);
                    $v!(JSPrimitiveWrapper);
                    $v!(JSPromise);
                    $v!(JSProxy);
                    $v!(JSRabGsabDataView);
                    $v!(JSRawJson);
                    $v!(JSReceiver);
                    $v!(JSRegExp);
                    $v!(JSRegExpStringIterator);
                    $v!(JSSet);
                    $v!(JSSetIterator);
                    $v!(JSShadowRealm);
                    $v!(JSSharedArray);
                    $v!(JSSharedStruct);
                    $v!(JSSpecialObject);
                    $v!(JSStringIterator);
                    $v!(JSSynchronizationPrimitive);
                    $v!(JSTemporalCalendar);
                    $v!(JSTemporalDuration);
                    $v!(JSTemporalInstant);
                    $v!(JSTemporalPlainDate);
                    $v!(JSTemporalPlainTime);
                    $v!(JSTemporalPlainDateTime);
                    $v!(JSTemporalPlainMonthDay);
                    $v!(JSTemporalPlainYearMonth);
                    $v!(JSTemporalTimeZone);
                    $v!(JSTemporalZonedDateTime);
                    $v!(JSTypedArray);
                    $v!(JSValidIteratorWrapper);
                    $v!(JSWeakCollection);
                    $v!(JSWeakRef);
                    $v!(JSWeakMap);
                    $v!(JSWeakSet);
                    $v!(JSWrappedFunction);
                    $v!(LoadHandler);
                    $v!(Map);
                    $v!(MegaDomHandler);
                    $v!(Module);
                    $v!(Microtask);
                    $v!(Name);
                    $v!(NameDictionary);
                    $v!(NameToIndexHashTable);
                    $v!(NativeContext);
                    $v!(NormalizedMapCache);
                    $v!(NumberDictionary);
                    $v!(ObjectHashSet);
                    $v!(ObjectHashTable);
                    $v!(ObjectTemplateInfo);
                    $v!(ObjectTwoHashTable);
                    $v!(Oddball);
                    $v!(Hole);
                    $v!(OrderedHashMap);
                    $v!(OrderedHashSet);
                    $v!(OrderedNameDictionary);
                    $v!(PreparseData);
                    $v!(PrimitiveHeapObject);
                    $v!(PromiseReactionJobTask);
                    $v!(PropertyArray);
                    $v!(PropertyCell);
                    $v!(ScopeInfo);
                    $v!(SeqOneByteString);
                    $v!(SeqString);
                    $v!(SeqTwoByteString);
                    $v!(SharedFunctionInfo);
                    $v!(SimpleNumberDictionary);
                    $v!(SlicedString);
                    $v!(SmallOrderedHashMap);
                    $v!(SmallOrderedHashSet);
                    $v!(SmallOrderedNameDictionary);
                    $v!(SourceTextModule);
                    $v!(SourceTextModuleInfo);
                    $v!(StoreHandler);
                    $v!(String);
                    $v!(StringSet);
                    $v!(RegisteredSymbolTable);
                    $v!(Struct);
                    $v!(SwissNameDictionary);
                    $v!(Symbol);
                    $v!(SyntheticModule);
                    $v!(TemplateInfo);
                    $v!(TemplateInfoWithProperties);
                    $v!(TemplateLiteralObject);
                    $v!(ThinString);
                    $v!(TransitionArray);
                    $v!(TurboshaftFloat64RangeType);
                    $v!(TurboshaftFloat64SetType);
                    $v!(TurboshaftFloat64Type);
                    $v!(TurboshaftType);
                    $v!(TurboshaftWord32RangeType);
                    $v!(TurboshaftWord32SetType);
                    $v!(TurboshaftWord32Type);
                    $v!(TurboshaftWord64RangeType);
                    $v!(TurboshaftWord64SetType);
                    $v!(TurboshaftWord64Type);
                    $v!(WasmArray);
                    $v!(WasmContinuationObject);
                    $v!(WasmExceptionPackage);
                    $v!(WasmFuncRef);
                    $v!(WasmGlobalObject);
                    $v!(WasmInstanceObject);
                    $v!(WasmMemoryObject);
                    $v!(WasmMemoryMapDescriptor);
                    $v!(WasmModuleObject);
                    $v!(WasmNull);
                    $v!(WasmObject);
                    $v!(WasmResumeData);
                    $v!(WasmStruct);
                    $v!(WasmSuspenderObject);
                    $v!(WasmSuspendingObject);
                    $v!(WasmTableObject);
                    $v!(WasmTagObject);
                    $v!(WasmTypeInfo);
                    $v!(WasmValueObject);
                    $v!(WeakArrayList);
                    $v!(WeakCell);
                    simple_heap_object_list1!($v);
                };
            }

            #[macro_export]
            macro_rules! virtual_object_type_list {
                ($v:ident) => {
                    $v!(AccessCheckNeeded);
                    $v!(AlwaysSharedSpaceJSObject);
                    $v!(BigIntWrapper);
                    $v!(BooleanWrapper);
                    $v!(Callable);
                    $v!(Constructor);
                    $v!(Filler);
                    $v!(HandlerTable);
                    $v!(JSContextExtensionObject);
                    $v!(JSError);
                    $v!(MapCache);
                    $v!(NumberWrapper);
                    $v!(OSROptimizedCodeCache);
                    $v!(ScriptWrapper);
                    $v!(StringWrapper);
                    $v!(SymbolWrapper);
                    $v!(UniqueName);
                    $v!(Undetectable);
                };
            }

            #[macro_export]
            macro_rules! heap_object_ordinary_type_list {
                ($v:ident) => {
                    heap_object_ordinary_type_list_base!($v);
                    $v!(JSV8BreakIterator);
                    $v!(JSCollator);
                    $v!(JSDateTimeFormat);
                    $v!(JSDisplayNames);
                    $v!(JSDurationFormat);
                    $v!(JSListFormat);
                    $v!(JSLocale);
                    $v!(JSNumberFormat);
                    $v!(JSPluralRules);
                    $v!(JSRelativeTimeFormat);
                    $v!(JSSegmentDataObject);
                    $v!(JSSegmentDataObjectWithIsWordLike);
                    $v!(JSSegmentIterator);
                    $v!(JSSegmenter);
                    $v!(JSSegments);
                };
            }

            #[macro_export]
            macro_rules! abstract_trusted_object_list_generator {
                ($apply:ident, $v:ident) => {
                    $apply!($v, TrustedObject, TRUSTED_OBJECT);
                    $apply!($v, ExposedTrustedObject, EXPOSED_TRUSTED_OBJECT);
                    $apply!($v, UncompiledData, UNCOMPILED_DATA);
                    $apply!($v, WasmFunctionData, WASM_FUNCTION_DATA);
                };
            }

            #[macro_export]
            macro_rules! concrete_trusted_object_list_generator {
                ($apply:ident, $v:ident) => {
                    $apply!($v, BytecodeArray, BYTECODE_ARRAY);
                    $apply!($v, Code, CODE);
                    $apply!($v, InstructionStream, INSTRUCTION_STREAM);
                    $apply!($v, InterpreterData, INTERPRETER_DATA);
                    $apply!($v, UncompiledDataWithPreparseData, UNCOMPILED_DATA_WITH_PREPARSE_DATA);
                    $apply!($v, UncompiledDataWithoutPreparseData, UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA);
                    $apply!($v, UncompiledDataWithPreparseDataAndJob, UNCOMPILED_DATA_WITH_PREPARSE_DATA_AND_JOB);
                    $apply!($v, UncompiledDataWithoutPreparseDataWithJob, UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_WITH_JOB);
                    $apply!($v, SharedFunctionInfoWrapper, SHARED_FUNCTION_INFO_WRAPPER);
                    $apply!($v, ProtectedFixedArray, PROTECTED_FIXED_ARRAY);
                    $apply!($v, ProtectedWeakFixedArray, PROTECTED_WEAK_FIXED_ARRAY);
                    $apply!($v, TrustedByteArray, TRUSTED_BYTE_ARRAY);
                    $apply!($v, TrustedFixedArray, TRUSTED_FIXED_ARRAY);
                    $apply!($v, TrustedForeign, TRUSTED_FOREIGN);
                    $apply!($v, TrustedWeakFixedArray, TRUSTED_WEAK_FIXED_ARRAY);
                    $apply!($v, AtomRegExpData, ATOM_REG_EXP_DATA);
                    $apply!($v, IrRegExpData, IR_REG_EXP_DATA);
                    $apply!($v, RegExpData, REG_EXP_DATA);
                    $apply!($v, WasmImportData, WASM_IMPORT_DATA);
                    $apply!($v, WasmCapiFunctionData, WASM_CAPI_FUNCTION_DATA);
                    $apply!($v, WasmDispatchTable, WASM_DISPATCH_TABLE);
                    $apply!($v, WasmExportedFunctionData, WASM_EXPORTED_FUNCTION_DATA);
                    $apply!($v, WasmJSFunctionData, WASM_JS_FUNCTION_DATA);
                    $apply!($v, WasmInternalFunction, WASM_INTERNAL_FUNCTION);
                    $apply!($v, WasmTrustedInstanceData, WASM_TRUSTED_INSTANCE_DATA);
                };
            }

            #[macro_export]
            macro_rules! trusted_object_list1_adapter {
                ($v:ident, $name:ident, $NAME:ident) => {
                    $v!($name)
                };
            }

            #[macro_export]
            macro_rules! trusted_object_list2_adapter {
                ($v:ident, $name:ident, $NAME:ident) => {
                    $v!($name, $NAME)
                };
            }

            #[macro_export]
            macro_rules! concrete_trusted_object_type_list1 {
                ($v:ident) => {
                    concrete_trusted_object_list_generator!(trusted_object_list1_adapter, $v)
                };
            }

            #[macro_export]
            macro_rules! concrete_trusted_object_type_list2 {
                ($v:ident) => {
                    concrete_trusted_object_list_generator!(trusted_object_list2_adapter, $v)
                };
            }

            #[macro_export]
            macro_rules! heap_object_trusted_type_list {
                ($v:ident) => {
                    abstract_trusted_object_list_generator!(trusted_object_list1_adapter, $v);
                    concrete_trusted_object_list_generator!(trusted_object_list1_adapter, $v)
                };
            }

            #[macro_export]
            macro_rules! heap_object_template_type_list {
                ($v:ident) => {
                    $v!(HashTable)
                };
            }

            #[macro_export]
            macro_rules! heap_object_specialized_type_list {
                ($v:ident) => {
                    $v!(AwaitContext);
                    $v!(BlockContext);
                    $v!(CallableApiObject);
                    $v!(CallableJSFunction);
                    $v!(CallableJSProxy);
                    $v!(CatchContext);
                    $v!(DebugEvaluateContext);
                    $v!(EvalContext);
                    $v!(FreeSpaceOrFiller);
                    $v!(FunctionContext);
                    $v!(JSApiObject);
                    $v!(JSClassConstructor);
                    $v!(JSLastDummyApiObject);
                    $v!(JSPromiseConstructor);
                    $v!(JSArrayConstructor);
                    $v!(JSRegExpConstructor);
                    $v!(JSMapKeyIterator);
                    $v!(JSMapKeyValueIterator);
                    $v!(JSMapValueIterator);
                    $v!(JSSetKeyValueIterator);
                    $v!(JSSetValueIterator);
                    $v!(JSSpecialApiObject);
                    $v!(MaybeReadOnlyJSObject);
                    $v!(ModuleContext);
                    $v!(NonNullForeign);
                    $v!(ScriptContext);
                    $v!(WithContext);
                    $v!(JSInternalPrototypeBase);
                    $v!(JSObjectPrototype);
                    $v!(JSRegExpPrototype);
                    $v!(JSPromisePrototype);
                    $v!(JSSetPrototype);
                    $v!(JSIteratorPrototype);
                    $v!(JSArrayIteratorPrototype);
                    $v!(JSMapIteratorPrototype);
                    $v!(JSTypedArrayPrototype);
                    $v!(JSSetIteratorPrototype);
                    $v!(JSStringIteratorPrototype);
                    $v!(TypedArrayConstructor);
                    $v!(Uint8TypedArrayConstructor);
                    $v!(Int8TypedArrayConstructor);
                    $v!(Uint16TypedArrayConstructor);
                    $v!(Int16TypedArrayConstructor);
                    $v!(Uint32TypedArrayConstructor);
                    $v!(Int32TypedArrayConstructor);
                    $v!(Float16TypedArrayConstructor);
                    $v!(Float32TypedArrayConstructor);
                    $v!(Float64TypedArrayConstructor);
                    $v!(Uint8ClampedTypedArrayConstructor);
                    $v!(Biguint64TypedArrayConstructor);
                    $v!(Bigint64TypedArrayConstructor);
                };
            }

            #[macro_export]
            macro_rules! heap_object_type_list {
                ($v:ident) => {
                    heap_object_ordinary_type_list!($v);
                    virtual_object_type_list!($v);
                    heap_object_trusted_type_list!($v);
                    heap_object_template_type_list!($v);
                    heap_object_specialized_type_list!($v);
                };
            }

            #[macro_export]
            macro_rules! oddball_list {
                ($v:ident) => {
                    $v!(Undefined, undefined_value, UndefinedValue);
                    $v!(Null, null_value, NullValue);
                    $v!(True, true_value, TrueValue);
                    $v!(False, false_value, FalseValue);
                };
            }

            #[macro_export]
            macro_rules! hole_list {
                ($v:ident) => {
                    $v!(TheHole, the_hole_value, TheHoleValue);
                    $v!(PropertyCellHole, property_cell_hole_value, PropertyCellHoleValue);
                    $v!(HashTableHole, hash_table_hole_value, HashTableHoleValue);
                    $v!(PromiseHole, promise_hole_value, PromiseHoleValue);
                    $v!(Exception, exception, Exception);
                    $v!(TerminationException, termination_exception, TerminationException);
                    $v!(Uninitialized, uninitialized_value, UninitializedValue);
                    $v!(ArgumentsMarker, arguments_marker, ArgumentsMarker);
                    $v!(OptimizedOut, optimized_out, OptimizedOut);
                    $v!(StaleRegister, stale_register, StaleRegister);
                    $v!(SelfReferenceMarker, self_reference_marker, SelfReferenceMarker);
                    $v!(BasicBlockCountersMarker, basic_block_counters_marker,
                       BasicBlockCountersMarker);
                };
            }

            #[macro_export]
            macro_rules! object_type_list {
                ($v:ident) => {
                    $v!(Primitive);
                    $v!(Number);
                    $v!(Numeric);
                };
            }

            pub mod object_forward_declarations {
                use super::*;
                macro_rules! def_fwd_declaration {
                    ($Type:ident) => {
                        pub struct $Type;
                    };
                }
                macro_rules! generate_forward_declarations {
                    ($macro:ident) => {
                        $macro!(def_fwd_declaration);
                    };
                }

                generate_forward_declarations!(heap_object_ordinary_type_list);
                generate_forward_declarations!(heap_object_trusted_type_list);
                generate_forward_declarations!(heap_object_specialized_type_list);
                generate_forward_declarations!(virtual_object_type_list);
            }

        }
    }
}

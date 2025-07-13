// Converted from V8 C++ source files:
// Header: local-factory-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

//use crate::heap::factory_base_inl::*;
use crate::heap::local_factory::*;
//use crate::roots::roots_inl::*;
//use crate::objects::*;
//use crate::handles::*;
//use crate::isolate::*;
use crate::heap::new_spaces::AllocationType;
//use crate::heap::heap::Heap;

// Dummy definitions for types used in ACCESSOR_INFO_ROOT_LIST
pub struct Type {}
pub struct String {}
pub struct Symbol {}
pub struct FixedArray {}
pub struct JSFunction {}
pub struct ByteArray {}
pub struct FixedDoubleArray {}
pub struct Script {}
pub struct SharedFunctionInfo {}
pub struct TemplateInfo {}
pub struct UncompiledData {}
pub struct AllocationSite {}
pub struct FeedbackCell {}
pub struct FreeSpaceInfo {}
pub struct MapCache {}
pub struct PropertyCell {}
pub struct ScopeInfo {}
pub struct Code {}
pub struct FeedbackVector {}
pub struct DescriptorArray {}
pub struct BytecodeArray {}
pub struct Module {}
pub struct CoverageInfo {}
pub struct TypeFeedbackVector {}
pub struct WasmExportedFunctionData {}
pub struct AsmWasmData {}
pub struct WasmInstance {}
pub struct WasmMemory {}
pub struct WasmTable {}
pub struct WasmModuleObject {}
pub struct WasmCompiledModule {}
pub struct AccessorInfo {}
pub struct InterceptorInfo {}
pub struct CallHandlerInfo {}
pub struct Struct {}
pub struct Tuple2 {}
pub struct Tuple3 {}
pub struct EmbedderData {}
pub struct ClassPositions {}
pub struct WeakArrayList {}
pub struct HashTable {}
pub struct OrderedHashTable {}
pub struct OrderedHashSet {}
pub struct SeqsOneByteString {}
pub struct SeqsTwoByteString {}
pub struct Name {}
pub struct NumberStringCache {}
pub struct Promise {}
pub struct Foreign {}
pub struct JSArrayBuffer {}
pub struct JSArray {}
pub struct JSDataView {}
pub struct JSFinalizationRegistry {}
pub struct JSIteratorResult {}
pub struct JSMap {}
pub struct JSMessageObject {}
pub struct JSObject {}
pub struct JSProxy {}
pub struct JSRegExp {}
pub struct JSSet {}
pub struct JSSharedArray {}
pub struct JSTypedArray {}
pub struct JSWeakMap {}
pub struct JSWeakSet {}
pub struct StringTable {}
pub struct Uninitialized {}
pub struct Null {}
pub struct Undefined {}
pub struct TheHole {}
pub struct NoChangeSentinel {}
pub struct ArgumentsMarker {}
pub struct Exception {}
pub struct TerminationException {}
pub struct StackOverflow {}
pub struct IllegalAccess {}
pub struct OutOfMemory {}
pub struct InternalError {}
pub struct OptimizeFunctionLater {}
pub struct NotAvailable {}
pub struct OneByteInternalizedString {}
pub struct FreeSpace {}
pub struct Map {}
pub struct FixedCObject {}
pub struct EphemeronHashTable {}
pub struct Boolean {}
pub struct NativeContext {}
pub struct Context {}
pub struct SavedContext {}
pub struct Microtask {}
pub struct Smi {}
pub struct Oddball {}
pub struct TheHoleHashTable {}
pub struct ScopeInfoCache {}
pub struct TransitionArray {}
pub struct CodeCache {}
pub struct PrototypeUsers {}
pub struct TemplateList {}
pub struct CompilationCacheTable {}
pub struct EmbedderTracingCategoryObserver {}
pub struct EmbedderTracingController {}
pub struct StringSplitCache {}
pub struct CodeCoverageInfo {}
pub struct DebugInfo {}
pub struct SymbolTable {}
pub struct StringDictionary {}
pub struct GlobalDictionary {}
pub struct FastAllocationInfo {}
pub struct FeedbackMetadata {}
pub struct InstructionStream {}
pub struct RegisteredSymbolTable {}
pub struct EmbedderDataDWordArray {}
pub struct ByteArrayTable {}
pub struct WeakFixedArray {}
pub struct FixedArrayBase {}
pub struct ModuleDescriptor {}
pub struct SourceTextModuleInfo {}
pub struct SyntheticModule {}
pub struct ClassBoilerplate {}
pub struct CoverageBufferTable {}
pub struct CoverageFunctionTable {}
pub struct HeapNumber {}
pub struct PrimitiveHeapNumber {}
pub struct LargeIntegerLiteral {}
pub struct FeedbackNexus {}
pub struct ProfileDataRecorder {}
pub struct BreakPoint {}
pub struct BreakPointInfo {}
pub struct DebugContext {}
pub struct BigInt {}
pub struct DebugBytecodeArray {}
pub struct StackFrameInfo {}
pub struct PreparseData {}
pub struct SharedArrayBufferBackingStore {}
pub struct WasmAsyncFunctionRequest {}
pub struct DeferredHandle {}
pub struct RegisteredWeakMapTable {}
pub struct WasmResumeData {}
pub struct DependentCode {}
pub struct DependentCodeGroup {}
pub struct NativeSource {};
pub struct WasmStruct {};

impl LocalFactory {
    fn accessor_info_accessor<T>(&self, root_index: RootIndex) -> DirectHandle<T> {
        DirectHandle {
            value: Rc::new(unsafe { std::mem::zeroed() }), // Dummy value
        }
    }

    pub fn accessor_info_accessor_accessorinfo(&self) -> DirectHandle<AccessorInfo> {
        self.accessor_info_accessor(RootIndex::kAccessorInfo)
    }

    pub fn accessor_info_accessor_argumentsmarker(&self) -> DirectHandle<ArgumentsMarker> {
        self.accessor_info_accessor(RootIndex::kArgumentsMarker)
    }

    pub fn accessor_info_accessor_array(&self) -> DirectHandle<JSArray> {
        self.accessor_info_accessor(RootIndex::kArray)
    }

    pub fn accessor_info_accessor_asmwasmdata(&self) -> DirectHandle<AsmWasmData> {
        self.accessor_info_accessor(RootIndex::kAsmWasmData)
    }

    pub fn accessor_info_accessor_allocation_site(&self) -> DirectHandle<AllocationSite> {
        self.accessor_info_accessor(RootIndex::kAllocationSite)
    }

    pub fn accessor_info_accessor_boolean(&self) -> DirectHandle<Boolean> {
        self.accessor_info_accessor(RootIndex::kBoolean)
    }

    pub fn accessor_info_accessor_break_point(&self) -> DirectHandle<BreakPoint> {
        self.accessor_info_accessor(RootIndex::kBreakPoint)
    }

    pub fn accessor_info_accessor_break_point_info(&self) -> DirectHandle<BreakPointInfo> {
        self.accessor_info_accessor(RootIndex::kBreakPointInfo)
    }

    pub fn accessor_info_accessor_bytecode_array(&self) -> DirectHandle<BytecodeArray> {
        self.accessor_info_accessor(RootIndex::kBytecodeArray)
    }

    pub fn accessor_info_accessor_bytearray(&self) -> DirectHandle<ByteArray> {
        self.accessor_info_accessor(RootIndex::kByteArray)
    }

    pub fn accessor_info_accessor_class_boilerplate(&self) -> DirectHandle<ClassBoilerplate> {
        self.accessor_info_accessor(RootIndex::kClassBoilerplate)
    }

    pub fn accessor_info_accessor_classpositions(&self) -> DirectHandle<ClassPositions> {
        self.accessor_info_accessor(RootIndex::kClassPositions)
    }

    pub fn accessor_info_accessor_code(&self) -> DirectHandle<Code> {
        self.accessor_info_accessor(RootIndex::kCode)
    }

   pub fn accessor_info_accessor_code_coverage_info(&self) -> DirectHandle<CodeCoverageInfo> {
        self.accessor_info_accessor(RootIndex::kCodeCoverageInfo)
    }
    
    pub fn accessor_info_accessor_codecache(&self) -> DirectHandle<CodeCache> {
        self.accessor_info_accessor(RootIndex::kCodeCache)
    }

    pub fn accessor_info_accessor_compilationcachetable(&self) -> DirectHandle<CompilationCacheTable> {
        self.accessor_info_accessor(RootIndex::kCompilationCacheTable)
    }

    pub fn accessor_info_accessor_context(&self) -> DirectHandle<Context> {
        self.accessor_info_accessor(RootIndex::kContext)
    }

    pub fn accessor_info_accessor_coveragebuffertable(&self) -> DirectHandle<CoverageBufferTable> {
        self.accessor_info_accessor(RootIndex::kCoverageBufferTable)
    }
    pub fn accessor_info_accessor_coveragefunctiontable(&self) -> DirectHandle<CoverageFunctionTable> {
        self.accessor_info_accessor(RootIndex::kCoverageFunctionTable)
    }

    pub fn accessor_info_accessor_coverageinfo(&self) -> DirectHandle<CoverageInfo> {
        self.accessor_info_accessor(RootIndex::kCoverageInfo)
    }

    pub fn accessor_info_accessor_debugbytecodearray(&self) -> DirectHandle<DebugBytecodeArray> {
        self.accessor_info_accessor(RootIndex::kDebugBytecodeArray)
    }
    pub fn accessor_info_accessor_debugcontext(&self) -> DirectHandle<DebugContext> {
        self.accessor_info_accessor(RootIndex::kDebugContext)
    }

    pub fn accessor_info_accessor_debuginfo(&self) -> DirectHandle<DebugInfo> {
        self.accessor_info_accessor(RootIndex::kDebugInfo)
    }
    
    pub fn accessor_info_accessor_deferredhandle(&self) -> DirectHandle<DeferredHandle> {
        self.accessor_info_accessor(RootIndex::kDeferredHandle)
    }
    pub fn accessor_info_accessor_dependentcode(&self) -> DirectHandle<DependentCode> {
        self.accessor_info_accessor(RootIndex::kDependentCode)
    }
   pub fn accessor_info_accessor_dependentcodegroup(&self) -> DirectHandle<DependentCodeGroup> {
        self.accessor_info_accessor(RootIndex::kDependentCodeGroup)
    }
    pub fn accessor_info_accessor_descriptorarray(&self) -> DirectHandle<DescriptorArray> {
        self.accessor_info_accessor(RootIndex::kDescriptorArray)
    }
    pub fn accessor_info_accessor_embedderdata(&self) -> DirectHandle<EmbedderData> {
        self.accessor_info_accessor(RootIndex::kEmbedderData)
    }
    pub fn accessor_info_accessor_embedderdatadwordarray(&self) -> DirectHandle<EmbedderDataDWordArray> {
        self.accessor_info_accessor(RootIndex::kEmbedderDataDWordArray)
    }

    pub fn accessor_info_accessor_embeddertracingcategoryobserver(&self) -> DirectHandle<EmbedderTracingCategoryObserver> {
        self.accessor_info_accessor(RootIndex::kEmbedderTracingCategoryObserver)
    }

   pub fn accessor_info_accessor_embeddertracingcontroller(&self) -> DirectHandle<EmbedderTracingController> {
        self.accessor_info_accessor(RootIndex::kEmbedderTracingController)
    }
    
    pub fn accessor_info_accessor_ephemeronhashtable(&self) -> DirectHandle<EphemeronHashTable> {
        self.accessor_info_accessor(RootIndex::kEphemeronHashTable)
    }
    pub fn accessor_info_accessor_exception(&self) -> DirectHandle<Exception> {
        self.accessor_info_accessor(RootIndex::kException)
    }
   pub fn accessor_info_accessor_fastallocationinfo(&self) -> DirectHandle<FastAllocationInfo> {
        self.accessor_info_accessor(RootIndex::kFastAllocationInfo)
    }
    pub fn accessor_info_accessor_feedbackcell(&self) -> DirectHandle<FeedbackCell> {
        self.accessor_info_accessor(RootIndex::kFeedbackCell)
    }
   pub fn accessor_info_accessor_feedbackmetadata(&self) -> DirectHandle<FeedbackMetadata> {
        self.accessor_info_accessor(RootIndex::kFeedbackMetadata)
    }
    pub fn accessor_info_accessor_feedbacknexus(&self) -> DirectHandle<FeedbackNexus> {
        self.accessor_info_accessor(RootIndex::kFeedbackNexus)
    }
    pub fn accessor_info_accessor_feedbackvector(&self) -> DirectHandle<FeedbackVector> {
        self.accessor_info_accessor(RootIndex::kFeedbackVector)
    }
    pub fn accessor_info_accessor_fixedarray(&self) -> DirectHandle<FixedArray> {
        self.accessor_info_accessor(RootIndex::kFixedArray)
    }

    pub fn accessor_info_accessor_fixedarraybase(&self) -> DirectHandle<FixedArrayBase> {
        self.accessor_info_accessor(RootIndex::kFixedArrayBase)
    }

   pub fn accessor_info_accessor_fixedcobject(&self) -> DirectHandle<FixedCObject> {
        self.accessor_info_accessor(RootIndex::kFixedCObject)
    }
    pub fn accessor_info_accessor_fixeddoublearray(&self) -> DirectHandle<FixedDoubleArray> {
        self.accessor_info_accessor(RootIndex::kFixedDoubleArray)
    }
    pub fn accessor_info_accessor_foreign(&self) -> DirectHandle<Foreign> {
        self.accessor_info_accessor(RootIndex::kForeign)
    }
    pub fn accessor_info_accessor_freespace(&self) -> DirectHandle<FreeSpace> {
        self.accessor_info_accessor(RootIndex::kFreeSpace)
    }
    pub fn accessor_info_accessor_freespaceinfo(&self) -> DirectHandle<FreeSpaceInfo> {
        self.accessor_info_accessor(RootIndex::kFreeSpaceInfo)
    }
    pub fn accessor_info_accessor_globaldictionary(&self) -> DirectHandle<GlobalDictionary> {
        self.accessor_info_accessor(RootIndex::kGlobalDictionary)
    }
   pub fn accessor_info_accessor_hashtable(&self) -> DirectHandle<HashTable> {
        self.accessor_info_accessor(RootIndex::kHashTable)
    }

   pub fn accessor_info_accessor_heapnumber(&self) -> DirectHandle<HeapNumber> {
        self.accessor_info_accessor(RootIndex::kHeapNumber)
    }
    
    pub fn accessor_info_accessor_illegalaccess(&self) -> DirectHandle<IllegalAccess> {
        self.accessor_info_accessor(RootIndex::kIllegalAccess)
    }

    pub fn accessor_info_accessor_instructionstream(&self) -> DirectHandle<InstructionStream> {
        self.accessor_info_accessor(RootIndex::kInstructionStream)
    }
    
    pub fn accessor_info_accessor_internalerror(&self) -> DirectHandle<InternalError> {
        self.accessor_info_accessor(RootIndex::kInternalError)
    }

    pub fn accessor_info_accessor_interceptorinfo(&self) -> DirectHandle<InterceptorInfo> {
        self.accessor_info_accessor(RootIndex::kInterceptorInfo)
    }

    pub fn accessor_info_accessor_jsarray(&self) -> DirectHandle<JSArray> {
        self.accessor_info_accessor(RootIndex::kJSArray)
    }

    pub fn accessor_info_accessor_jsarraybuffer(&self) -> DirectHandle<JSArrayBuffer> {
        self.accessor_info_accessor(RootIndex::kJSArrayBuffer)
    }

    pub fn accessor_info_accessor_jsdataview(&self) -> DirectHandle<JSDataView> {
        self.accessor_info_accessor(RootIndex::kJSDataView)
    }

    pub fn accessor_info_accessor_jsfinalizationregistry(&self) -> DirectHandle<JSFinalizationRegistry> {
        self.accessor_info_accessor(RootIndex::kJSFinalizationRegistry)
    }

    pub fn accessor_info_accessor_jsfunction(&self) -> DirectHandle<JSFunction> {
        self.accessor_info_accessor(RootIndex::kJSFunction)
    }
    pub fn accessor_info_accessor_jsiteratorresult(&self) -> DirectHandle<JSIteratorResult> {
        self.accessor_info_accessor(RootIndex::kJSIteratorResult)
    }

    pub fn accessor_info_accessor_jsmap(&self) -> DirectHandle<JSMap> {
        self.accessor_info_accessor(RootIndex::kJSMap)
    }
    pub fn accessor_info_accessor_jsmessageobject(&self) -> DirectHandle<JSMessageObject> {
        self.accessor_info_accessor(RootIndex::kJSMessageObject)
    }
    pub fn accessor_info_accessor_jsobject(&self) -> DirectHandle<JSObject> {
        self.accessor_info_accessor(RootIndex::kJSObject)
    }

    pub fn accessor_info_accessor_jsproxy(&self) -> DirectHandle<JSProxy> {
        self.accessor_info_accessor(RootIndex::kJSProxy)
    }

    pub fn accessor_info_accessor_jsregexp(&self) -> DirectHandle<JSRegExp> {
        self.accessor_info_accessor(RootIndex::kJSRegExp)
    }

    pub fn accessor_info_accessor_jsset(&self) -> DirectHandle<JSSet> {
        self.accessor_info_accessor(RootIndex::kJSSet)
    }
    pub fn accessor_info_accessor_jssharedarray(&self) -> DirectHandle<JSSharedArray> {
        self.accessor_info_accessor(RootIndex::kJSSharedArray)
    }

    pub fn accessor_info_accessor_jstypedarray(&self) -> DirectHandle<JSTypedArray> {
        self.accessor_info_accessor(RootIndex::kJSTypedArray)
    }
    pub fn accessor_info_accessor_jsweakmap(&self) -> DirectHandle<JSWeakMap> {
        self.accessor_info_accessor(RootIndex::kJSWeakMap)
    }
    pub fn accessor_info_accessor_jsweakset(&self) -> DirectHandle<JSWeakSet> {
        self.accessor_info_accessor(RootIndex::kJSWeakSet)
    }

    pub fn accessor_info_accessor_largeintegerliteral(&self) -> DirectHandle<LargeIntegerLiteral> {
        self.accessor_info_accessor(RootIndex::kLargeIntegerLiteral)
    }

   pub fn accessor_info_accessor_map(&self) -> DirectHandle<Map> {
        self.accessor_info_accessor(RootIndex::kMap)
    }
   pub fn accessor_info_accessor_mapcache(&self) -> DirectHandle<MapCache> {
        self.accessor_info_accessor(RootIndex::kMapCache)
    }
    pub fn accessor_info_accessor_microtask(&self) -> DirectHandle<Microtask> {
        self.accessor_info_accessor(RootIndex::kMicrotask)
    }
    pub fn accessor_info_accessor_module(&self) -> DirectHandle<Module> {
        self.accessor_info_accessor(RootIndex::kModule)
    }

   pub fn accessor_info_accessor_moduledescriptor(&self) -> DirectHandle<ModuleDescriptor> {
        self.accessor_info_accessor(RootIndex::kModuleDescriptor)
    }
    pub fn accessor_info_accessor_name(&self) -> DirectHandle<Name> {
        self.accessor_info_accessor(RootIndex::kName)
    }
   pub fn accessor_info_accessor_nativecontext(&self) -> DirectHandle<NativeContext> {
        self.accessor_info_accessor(RootIndex::kNativeContext)
    }
    pub fn accessor_info_accessor_nochangesentinel(&self) -> DirectHandle<NoChangeSentinel> {
        self.accessor_info_accessor(RootIndex::kNoChangeSentinel)
    }
    pub fn accessor_info_accessor_notavailable(&self) -> DirectHandle<NotAvailable> {
        self.accessor_info_accessor(RootIndex::kNotAvailable)
    }
    pub fn accessor_info_accessor_null(&self) -> DirectHandle<Null> {
        self.accessor_info_accessor(RootIndex::kNull)
    }
    pub fn accessor_info_accessor_numberstringcache(&self) -> DirectHandle<NumberStringCache> {
        self.accessor_info_accessor(RootIndex::kNumberStringCache)
    }
    pub fn accessor_info_accessor_oddball(&self) -> DirectHandle<Oddball> {
        self.accessor_info_accessor(RootIndex::kOddball)
    }

    pub fn accessor_info_accessor_onebyteinternalizedstring(&self) -> DirectHandle<OneByteInternalizedString> {
        self.accessor_info_accessor(RootIndex::kOneByteInternalizedString)
    }
    pub fn accessor_info_accessor_optimizefunctionlater(&self) -> DirectHandle<OptimizeFunctionLater> {
        self.accessor_info_accessor(RootIndex::kOptimizeFunctionLater)
    }
    pub fn accessor_info_accessor_orderedhashtable(&self) -> DirectHandle<OrderedHashTable> {
        self.accessor_info_accessor(RootIndex::kOrderedHashTable)
    }
   pub fn accessor_info_accessor_orderedhashset(&self) -> DirectHandle<OrderedHashSet> {
        self.accessor_info_accessor(RootIndex::kOrderedHashSet)
    }
    pub fn accessor_info_accessor_outofmemory(&self) -> DirectHandle<OutOfMemory> {
        self.accessor_info_accessor(RootIndex::kOutOfMemory)
    }
    pub fn accessor_info_accessor_preparsedata(&self) -> DirectHandle<PreparseData> {
        self.accessor_info_accessor(RootIndex::kPreparseData)
    }

    pub fn accessor_info_accessor_primitiveheapnumber(&self) -> DirectHandle<PrimitiveHeapNumber> {
        self.accessor_info_accessor(RootIndex::kPrimitiveHeapNumber)
    }

   pub fn accessor_info_accessor_profiledatarecorder(&self) -> DirectHandle<ProfileDataRecorder> {
        self.accessor_info_accessor(RootIndex::kProfileDataRecorder)
    }
   pub fn accessor_info_accessor_promise(&self) -> DirectHandle<Promise> {
        self.accessor_info_accessor(RootIndex::kPromise)
    }
    pub fn accessor_info_accessor_propertycell(&self) -> DirectHandle<PropertyCell> {
        self.accessor_info_accessor(RootIndex::kPropertyCell)
    }

    pub fn accessor_info_accessor_prototypeusers(&self) -> DirectHandle<PrototypeUsers> {
        self.accessor_info_accessor(RootIndex::kPrototypeUsers)
    }
    pub fn accessor_info_accessor_registeredsymboltable(&self) -> DirectHandle<RegisteredSymbolTable> {
        self.accessor_info_accessor(RootIndex::kRegisteredSymbolTable)
    }

    pub fn accessor_info_accessor_registeredweakmaptable(&self) -> DirectHandle<RegisteredWeakMapTable> {
        self.accessor_info_accessor(RootIndex::kRegisteredWeakMapTable)
    }

    pub fn accessor_info_accessor_savedcontext(&self) -> DirectHandle<SavedContext> {
        self.accessor_info_accessor(RootIndex::kSavedContext)
    }

    pub fn accessor_info_accessor_scopeinfo(&self) -> DirectHandle<ScopeInfo> {
        self.accessor_info_accessor(RootIndex::kScopeInfo)
    }
    pub fn accessor_info_accessor_scopeinfocache(&self) -> DirectHandle<ScopeInfoCache> {
        self.accessor_info_accessor(RootIndex::kScopeInfoCache)
    }

    pub fn accessor_info_accessor_script(&self) -> DirectHandle<Script> {
        self.accessor_info_accessor(RootIndex::kScript)
    }
    pub fn accessor_info_accessor_seqsonebytestring(&self) -> DirectHandle<SeqsOneByteString> {
        self.accessor_info_accessor(RootIndex::kSeqsOneByteString)
    }
   pub fn accessor_info_accessor_seqstwobytestring(&self) -> DirectHandle<SeqsTwoByteString> {
        self.accessor_info_accessor(RootIndex::kSeqsTwoByteString)
    }
    pub fn accessor_info_accessor_sharedarraybufferbackingstore(&self) -> DirectHandle<SharedArrayBufferBackingStore> {
        self.accessor_info_accessor(RootIndex::kSharedArrayBufferBackingStore)
    }

    pub fn accessor_info_accessor_sharedfunctioninfo(&self) -> DirectHandle<SharedFunctionInfo> {
        self.accessor_info_accessor(RootIndex::kSharedFunctionInfo)
    }
   pub fn accessor_info_accessor_smi(&self) -> DirectHandle<Smi> {
        self.accessor_info_accessor(RootIndex::kSmi)
    }
    pub fn accessor_info_accessor_sourcetextmoduleinfo(&self) -> DirectHandle<SourceTextModuleInfo> {
        self.accessor_info_accessor(RootIndex::kSourceTextModuleInfo)
    }
    pub fn accessor_info_accessor_stackframeinfo(&self) -> DirectHandle<StackFrameInfo> {
        self.accessor_info_accessor(RootIndex::kStackFrameInfo)
    }

    pub fn accessor_info_accessor_stackoverflow(&self) -> DirectHandle<StackOverflow> {
        self.accessor_info_accessor(RootIndex::kStackOverflow)
    }

    pub fn accessor_info_accessor_stringdictionary(&self) -> DirectHandle<StringDictionary> {
        self.accessor_info_accessor(RootIndex::kStringDictionary)
    }

    pub fn accessor_info_accessor_stringsplitcache(&self) -> DirectHandle<StringSplitCache> {
        self.accessor_info_accessor(RootIndex::kStringSplitCache)
    }

   pub fn accessor_info_accessor_stringtable(&self) -> DirectHandle<StringTable> {
        self.accessor_info_accessor(RootIndex::kStringTable)
    }
    
    pub fn accessor_info_accessor_struct(&self) -> DirectHandle<Struct> {
        self.accessor_info_accessor(RootIndex::kStruct)
    }

    pub fn accessor_info_accessor_symbol(&self) -> DirectHandle<Symbol> {
        self.accessor_info_accessor(RootIndex::kSymbol)
    }

    pub fn accessor_info_accessor_symboltable(&self) -> DirectHandle<SymbolTable> {
        self.accessor_info_accessor(RootIndex::kSymbolTable)
    }

    pub fn accessor_info_accessor_syntheticmodule(&self) -> DirectHandle<SyntheticModule> {
        self.accessor_info_accessor(RootIndex::kSyntheticModule)
    }
    pub fn accessor_info_accessor_templateinfo(&self) -> DirectHandle<TemplateInfo> {
        self.accessor_info_accessor(RootIndex::kTemplateInfo)
    }
    pub fn accessor_info_accessor_templatelist(&self) -> DirectHandle<TemplateList> {
        self.accessor_info_accessor(RootIndex::kTemplateList)
    }

    pub fn accessor_info_accessor_terminationexception(&self) -> DirectHandle<TerminationException> {
        self.accessor_info_accessor(RootIndex::kTerminationException)
    }
   pub fn accessor_info_accessor_thehole(&self) -> DirectHandle<TheHole> {
        self.accessor_info_accessor(RootIndex::kTheHole)
    }

   pub fn accessor_info_accessor_theholehashtable(&self) -> DirectHandle<TheHoleHashTable> {
        self.accessor_info_accessor(RootIndex::kTheHoleHashTable)
    }

    pub fn accessor_info_accessor_transitionarray(&self) -> DirectHandle<TransitionArray> {
        self.accessor_info_accessor(RootIndex::kTransitionArray)
    }
    pub fn accessor_info_accessor_tuple2(&self) -> DirectHandle<Tuple2> {
        self.accessor_info_accessor(RootIndex::kTuple2)
    }
    pub fn accessor_info_accessor_tuple3(&self) -> DirectHandle<Tuple3> {
        self.accessor_info_accessor(RootIndex::kTuple3)
    }

    pub fn accessor_info_accessor_typefeedbackvector(&self) -> DirectHandle<TypeFeedbackVector> {
        self.accessor_info_accessor(RootIndex::kTypeFeedbackVector)
    }
    pub fn accessor_info_accessor_undefined(&self) -> DirectHandle<Undefined> {
        self.accessor_info_accessor(RootIndex::kUndefined)
    }
    pub fn accessor_info_accessor_uncompileddata(&self) -> DirectHandle<UncompiledData> {
        self.accessor_info_accessor(RootIndex::kUncompiledData)
    }
    pub fn accessor_info_accessor_uninitialized(&self) -> DirectHandle<Uninitialized> {
        self.accessor_info_accessor(RootIndex::kUninitialized)
    }
    pub fn accessor_info_accessor_wasmcompiledmodule(&self) -> DirectHandle<WasmCompiledModule> {
        self.accessor_info_accessor(RootIndex::kWasmCompiledModule)
    }
    pub fn accessor_info_accessor_wasmexportedfunctiondata(&self) -> DirectHandle<WasmExportedFunctionData> {
        self.accessor_info_accessor(RootIndex::kWasmExportedFunctionData)
    }

    pub fn accessor_info_accessor_wasminstance(&self) -> DirectHandle<WasmInstance> {
        self.accessor_info_accessor(RootIndex::kWasmInstance)
    }

   pub fn accessor_info_accessor_wasmmemory(&self) -> DirectHandle<WasmMemory> {
        self.accessor_info_accessor(RootIndex::kWasmMemory)
    }

    pub fn accessor_info_accessor_wasmmoduleobject(&self) -> DirectHandle<WasmModuleObject> {
        self.accessor_info_accessor(RootIndex::kWasmModuleObject)
    }

    pub fn accessor_info_accessor_wasmresumedata(&self) -> DirectHandle<WasmResumeData> {
        self.accessor_info_accessor(RootIndex::kWasmResumeData)
    }

    pub fn accessor_info_accessor_wasmstruct(&self) -> DirectHandle<WasmStruct> {
        self.accessor_info_accessor(RootIndex::kWasmStruct)
    }

    pub fn accessor_info_accessor_wasmtable(&self) -> DirectHandle<WasmTable> {
        self.accessor_info_accessor(RootIndex::kWasmTable)
    }

    pub fn accessor_info_accessor_weakarraylist(&self) -> DirectHandle<WeakArrayList> {
        self.accessor_info_accessor(RootIndex::kWeakArrayList)
    }

    pub fn accessor_info_accessor_weakfixedarray(&self) -> DirectHandle<WeakFixedArray> {
        self.accessor_info_accessor(RootIndex::kWeakFixedArray)
    }

    fn accessor_info_accessor_bigint(&self) -> DirectHandle<BigInt> {
        self.accessor_info_accessor(RootIndex::kBigInt)
    }

   fn accessor_info_accessor_callhandlerinfo(&self) -> DirectHandle<CallHandlerInfo> {
        self.accessor_info_accessor(RootIndex::kCallHandlerInfo)
    }
   fn accessor_info_accessor_ephemeronhashtable_keys(&self) -> DirectHandle<EphemeronHashTable> {
        self.accessor_info_accessor(RootIndex::kEphemeronHashTableKeys)
    }

   fn accessor_info_accessor_nativesource(&self) -> DirectHandle<NativeSource> {
        self.accessor_info_accessor(RootIndex::kNativeSource)
    }
    fn accessor_info_accessor_wasmasyncfunctionrequest(&self) -> DirectHandle<WasmAsyncFunctionRequest> {
        self.accessor_info_accessor(RootIndex::kWasmAsyncFunctionRequest)
    }
    fn accessor_info_accessor_dependentcode_transitions(&self) -> DirectHandle<DependentCode> {
        self.accessor_info_accessor(RootIndex::kDependentCodeTransitions)
    }
}

impl LocalFactory {
    pub fn allocation_type_for_in_place_internalizable_string(&self) -> AllocationType {
        self.isolate()
            .heap()
            .AsHeap()
            .allocation_type_for_in_place_internalizable_strings()
    }
}

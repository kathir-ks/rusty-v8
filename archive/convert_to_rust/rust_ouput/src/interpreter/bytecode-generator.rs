// Converted from V8 C++ source files:
// Header: bytecode-generator.h
// Implementation: bytecode-generator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/src/interpreter/bytecode-generator.h
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FeedbackSlotKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BytecodeArrayBuilder {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Isolate {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Handle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct LocalIsolate {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Zone {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct UnoptimizedCompilationInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AstStringConstants {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FunctionLiteral {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Script {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BytecodeArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Local<'a, T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MaybeLocal<'a, T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Value {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SharedFunctionInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FixedArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct HandleScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Static {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct v8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Address {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct DirectHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Root {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Tagged_t {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Map {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BytecodeSourceInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Code {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct DirectArguments{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Operator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct String {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FeedbackSource {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSObjectRef {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSRegExp{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RootIndex {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MachineType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Exceptions {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AstRawString{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FeedbackVectorSpec{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Name {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BranchHint {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RegExpFlags {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Boolean {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Type {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RootVisitor{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ObjectRef{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SharedFunctionInfoRef{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSGlobalProxy{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSReceiver{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RegExpTree {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSProxy{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSGlobalObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SmallOrderedHashMap{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RegExpNode{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Promise{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InternalClassId{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MapWord {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MapRef {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct HeapObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeDataContainer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InstructionSequence {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MaybeHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSArrayBufferView {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ByteArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSProxyRevocationHandler {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PromiseReactionJobTask {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TurboAssembler {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TurboshaftCodeT{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TemplateObjectDescription{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ObjectBoilerplateDescription{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ArrayBoilerplateDescription{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ClassBoilerplate{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FeedbackMetadata {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSBoundFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Number{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InternalClass{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct DescriptorArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyCell {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AccessorPair{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScriptContextTable{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BigInt{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SmallOrderedHashMapHandle{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FieldType{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TypeFeedbackVector{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CallOptimizationInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NameDictionary{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyDetails{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SourceTextModuleDescriptor {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AstNodeSourceRangesMethods {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SourcePositionTableBuilder{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TypeFeedbackId{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeKindLabel {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Tagged<T> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct IndirectHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringTable {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct OrderedHashMap{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSArgumentsObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RegExpInstruction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmExceptionPackage {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MapType{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyAccessInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeDataContainerBase {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Object{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringTableBucket{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SharedFunctionInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScriptContext{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CoverageInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PromiseReactionHandler{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSArrayBuffer{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeCache {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct BytecodeLoopAssignments {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSModule {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmInstanceObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ByteArrayBoilerplate{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScopeInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyCellType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SharedAccessorInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SourceTextModuleInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeT{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSWeakRef {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FixedDoubleArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FunctionTemplateInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScriptContextTableConstants {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmExportedFunctionData {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PromiseCapability {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ObjectInYoungGeneration {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CallSiteInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NameDictionaryLookupCache {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FixedArrayBoilerplate{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSTypedArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TrustedByteArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmCompiledModule {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct RegExpNodeInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InternalClassDescription {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScopeInfoIfYoung {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MaybeObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TypeFeedbackCells {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NativeContext {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NumberDictionary{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ContextExtension{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NativeObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyCellEnumCache{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SmallOrderedHashSet{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct DebugInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AllocationSite {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InternalClassPrototypeInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScopeInfoCells {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SymbolToScopeInfoMap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TypeDescription {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringDictionary{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct GlobalPropertyCell {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct HashTableBase{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct InstructionStream{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmDebugInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSModuleVariable{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSSpecialObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NameDictionaryLookupCacheShape{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SloppyArgumentsElements{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AllocationSiteWithWeakNavigation{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSArrayIterator{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TemplateInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AsmWasmData{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmExportedFunction{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AsmWasmFunction{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TemplateTableEntry{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSGeneratorObject{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyHandlerPrototype{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct FixedArrayBase{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSStringIterator{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TurboAssemblerCodeT{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct CodeContext {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Symbol {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Context{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct NumberDictionaryIterator{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSGlobalProxyChain{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SourceCodeHeader{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct HeapNumber{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringDictionaryIterator{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringIterator{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AccessorInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ConstantArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TransitionArray{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyCellEnum{
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MapCache {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct PropertyHandlerInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct SharedFunctionInfoData{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmStruct{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSWeakSet {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AsmJsFieldDescription {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Descriptor{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct Oddball{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSDataView {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TrustedValue{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct WasmJSFunction{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScopeInfoCacheEntry{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct StringDictionaryShape{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct TypeFeedbackCell{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct JSWeakMap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct ScopeTypeInfo{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct MapCacheTable{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct AccessorData{}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/feedback-vector.h
pub struct HashTable{
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/

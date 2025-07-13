// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-wasm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct V8 {
    }

    pub struct code {
    }

    pub struct Wasm {
    }

    pub enum class StackLimitKind {
    }

    pub struct UseScratchRegisterScope { dummy: i32 }

    pub enum Type {
    }

    pub struct iterator {
    }

    pub struct StackFrameIterator {
    }

    pub struct Address {
    }

    pub struct ValueType {
    }

    pub struct HandleScope {
    }

    pub struct SealHandleScope {
    }

    pub struct DisallowGarbageCollection {
    }

    pub struct StackLimitCheck {
    }

    pub struct SaveAndClearThreadInWasmFlag {
    }

    pub struct Deoptimizer {
    }

    pub struct If {
    }

    pub struct WasmCodePointer {
    }

    pub struct JSFunction {
    }

    pub struct SealHandleScope {}
    pub struct Smi {}
    pub struct Context {}
    pub struct Tagged<T> {}

    pub struct JSObject {}
    pub struct Object {}
    pub struct FixedArray {}
    pub struct WasmExceptionTag {}

    pub struct Builtin {}
    pub struct EphemeronHashTable {}
    pub struct Builtin {}
    pub struct JSReceiver {}
    pub struct StackFrame {}
    pub struct Script {}
    pub struct WeakFixedArray {}
    pub struct Map {}
    pub struct WasmArray {}
    pub struct ByteArray {}
    pub struct Register {}
    pub struct ConstantPoolArray {}
    pub struct SharedFunctionInfo {}
    pub struct Code {}
    pub struct HeapObject {}
    pub struct FullObjectSlot {}
    pub struct NativeModule {}
    pub struct SourceTextModuleDescriptor {}
    pub struct Zone {}

    pub fn Use(&self, node: OpIndex) -> InstructionOperand {}
    pub fn this(&self) -> &T {}
    pub fn source(&self) -> String {}
    pub fn code(&self, _isolate: &Isolate) -> &Code {}
    pub fn is(&self, creg: FPUControlRegister) -> bool {}
    pub fn find(&self, node: *const ZoneObject) -> Option<&dyn AstNodeSourceRangesMethods> {}
    pub fn to(&self) -> i32 {}
    pub fn stack() -> *mut Object {}
    pub fn visit(&mut self, instr: &Instruction, fixed: InstructionMask, mask: InstructionMask) {}
    pub fn values(&self, isolate: *mut Isolate) -> DirectHandle<FixedArray> {}
    pub fn from(csr: ControlStatusReg) -> Self {}
    pub fn and(&mut self, dst: Register, src1: Register, src2: Operand, cond: Condition) {}
    pub fn check(&mut self, cond: Condition, reason: AbortReason) {}
    pub fn frame(&self) -> &Frame {}
    pub fn iterator(&self) -> std::vec::IntoIter<*mut Node> {}
    pub fn Address() {}
    pub fn fp(&mut self) -> &mut File {}
    pub fn Merge(&self, count: usize) -> Operator {}
    pub fn safe(&self) -> bool {}
    pub fn S(&self) -> CPURegister {}
    pub fn at(&self, index: usize) -> &mut Entry {}
    pub fn type(&self) -> CPURegister::RegisterType {}
    pub fn call(&mut self, target: Register, cond: Condition) {}
    pub fn into(self) -> UnoptimizedCompileFlags {}
    pub fn not(self) -> Self::Output {}
    pub fn set(&mut self, value: T) {}
    pub fn exception(&self) -> Object {}
    pub fn This(&self) -> This { This { dummy: 0 } }
    pub fn bool() -> Self {}
    pub fn Store(&self, _rep: StoreRepresentation, _object: *mut Node, _offset: *mut Node, _value: *mut Node) -> *mut Node {}
    pub fn trap(&mut self) {}
    pub fn reason(&self) -> String {}
    pub fn JS() -> Self {}
    pub fn object(&self) -> &JsonObject {}
    pub fn returns(&self) -> Vec<T> {}
    pub fn any() -> Self {}
    pub fn expected(&self) -> OpIndex {}
    pub fn ret(&mut self) {}
    pub fn builtin(&self) -> Builtin {}
    pub fn use(&self, hi: OpIndex) -> InstructionOperand {}
    pub fn remove(&mut self, other: &CPURegList) {}
    pub fn pos(&self) -> i32 {}
    pub fn wire_bytes(&self) -> base::Vector<*const u8> { base::Vector { } }
    pub fn op(&self) -> &Operator {}
    pub fn message(&self) -> &str {}
    pub fn or(&mut self, rd: Register, rs: Register, imm: i32) {}
    pub fn clear(&mut self) {}
    pub fn back(&mut self) -> &mut T {}
    pub fn context(&self) -> Tagged<Context> {}
    pub fn Check(&mut self, cc: Condition, reason: AbortReason) {}
    pub fn module(&self) -> &SourceTextModuleDescriptor {}
    pub fn native_module(&self) -> *mut NativeModule { std::ptr::null_mut() }
    pub fn save(&mut self) -> SaveOptions {}
    pub fn GC(&mut self, gctype: GCType) {}
    pub fn Destroy(_location: *mut usize) {}
    pub fn feedback(&self) -> Option<i32> {}
    pub fn allocate(&mut self, n: i32) -> i32 {}
    pub fn new(isolate: &'a Isolate) -> Self {}
    pub fn frames(&self) -> Vec<V8StackFrame> {}
    pub fn wrapper(&self) -> Rc<BytecodeArrayWrapper> {}
    pub fn start(&self) -> *mut u8 {}
    pub fn bigdecimal_value(&self) -> BigDecimalValue {}
    pub fn fast(&mut self) {}
    pub fn Set(new_value: T) {}
    pub fn isolate(&mut self) -> &mut Isolate {}
    pub fn handle(&self) -> Handle<HeapObject> {}
    pub fn scope(&self) -> *mut DeclarationScope {}
    pub fn Wrap(isolate: *mut Isolate, wrapper: &Local<Object>, wrappable: *mut std::ffi::c_void) {}
    pub fn installed() -> bool {}
    pub fn Reset(&mut self) {}
    pub fn raw(&self) -> i64 {}
    pub fn Replace(&self, node: *mut Node, new_node: *mut Node) {}
    pub fn Iterate(&mut self, _v: Root) {}
    pub fn all(&self) -> Vec<T> {}
    pub fn replace(node: &mut Node, replacement: Rc<RefCell<Node>>) {}
    pub fn signature(&self) -> Option<Object> {}
    pub fn index(&self) -> u32 {}
    pub fn shared(&self, _isolate: &Isolate) -> &SharedFunctionInfo {}
    pub fn kind(&self) -> CodeKind {}
    pub fn cache(&mut self, index: usize) -> &mut InnerPointerToCodeCacheEntry {}
    pub fn source_positions(&self) -> bool {}
    pub fn external(line: i32, file_id: i32) -> Self {}
    pub fn target(&self) -> *mut Node {}
    pub fn table(&self) -> &EphemeronHashTable {}
    pub fn Compile(isolate: *mut Isolate, re_data: DirectHandle<IrRegExpData>) -> bool {}
    pub fn decrement(&mut self) -> &mut Self {}
    pub fn null() -> Self {}
    pub fn reset(&mut self) {}
    pub fn next(&mut self) {}
    pub fn element_type(&self) -> ValueType { self.element_type_ }
    pub fn array(&self) -> *mut FixedArray {}
    pub fn ranges(&self) -> &Vec<CharacterRange> {}
    pub fn copy(&self, zone: &Zone) -> Self {}
    pub fn length(&self) -> i32 {}
    pub fn implementation(&self) -> IrregexpImplementation {}
    pub fn get(&self, _index: usize) -> &Object {}
    pub fn create() {}
    pub fn Returns(_machine_type: MachineType) -> Self {}
    pub fn Enter(&self) {}
    pub fn debug(&mut self) -> &mut Debug { &mut Debug {} }
    pub fn second(&self) -> Value {}
    pub fn switch(&mut self, scratch: Register, value: Register, case_value_base: i32, labels: &mut [*mut Label], num_labels: i32) {}
    pub fn current() -> Option<LocalHeap> {}
    pub fn utf8_length(&self, isolate: *mut Isolate) -> i32 {}
    pub fn save(&mut self) -> SaveOptions {}
    pub fn invalid() -> Self {}
    pub fn count(&self) -> usize {}
    pub fn instance() -> &'static mut CpuFeatures {}
    pub fn put() {}
    pub fn Remove(){}
    pub fn first(&self) -> RegisterT{}
    pub fn ToBoolean(arg0 : bool) -> Object{}
    pub fn then(){}
    pub fn has(&self, reg: Register) -> bool {}
    pub fn initialize(data: &mut CallInterfaceDescriptorData) {}
    pub fn elements(&mut self) -> &mut Vec<TextElement> {}
    pub fn Allocate(&mut self, size: i32) -> *mut Node {}
    pub fn suspender(&self) -> Tagged<Object> {}
    pub fn parent(&self) -> Option<std::cell::Ref<'_, Type>> {}
    pub fn Mark(&mut self) {}
    pub fn bits(&self, hi: i32, lo: i32) -> i32 {}
    pub fn Pass(self) -> Self {}
    pub fn string(&self) -> IndirectHandle<String> {}
    pub fn operation(&self) -> &OperationType {}
    pub fn empty() -> Self {}
    pub fn size(&self) -> usize {}
    pub fn input(&self, i: i32) -> OpIndex {}
    pub fn failed(&self) -> bool {}
    pub fn instruction(instruction: *const InstructionBase, index: usize) -> Self {}
    pub fn valid(&self) -> bool {}
    pub fn end(&self) -> *const T {}
    pub fn bytes(&mut self, start: *const u8, count: u32) {}
    pub fn EraseIf<C, F, T>(container: &mut C, f: F) {}
    pub fn get_ptr(&self) -> Tagged<Object> { Tagged {} }
    pub fn native_context(&self) -> Tagged<Context> { Tagged {} }
    pub fn feedback_vectors(&self) -> Tagged<FixedArray> { Tagged {} }
    pub fn tiering_budget_array(&self) -> Tagged<FixedArray> { Tagged {} }
    pub fn tables(&self) -> Tagged<FixedArray> { Tagged {} }
    pub fn ReadExternalPointerField<T>(&self, a : i32, b: &'static V8) -> *mut T {std::ptr::null_mut()}
    pub fn module_object(&self) -> Tagged<JSObject> { Tagged {} }
    pub fn modules() -> Tagged<WeakFixedArray> { Tagged {} }
    pub fn try_get_func_ref(&self, a :i32, b: &mut Tagged<WasmFuncRef>) -> bool { false }
    pub fn unsafe_type(){}
    pub fn current_length() -> u32 { 0 }
    pub fn data_segment_sizes() -> Tagged<FixedArray> { Tagged {} }
    pub fn data_segment_starts() -> Tagged<FixedArray> { Tagged {} }
    pub fn memory_size(memory : u32) -> u64 { 0 }
    pub fn memory_base(memory : u32) -> *mut u8 { std::ptr::null_mut() }
    pub fn element_segments() -> Tagged<FixedArray> { Tagged {} }
    pub fn is_in_bounds(entry_index : u32) -> bool { true }
    pub fn AsInt64() -> i64 { 0 }
    pub fn NewStringFromAsciiChecked(arg0 : &str) -> DirectHandle<String> {DirectHandle{}}
    pub fn set_compiled_wrapper(){}
    pub fn dispatch_table_for_imports() -> Tagged<WasmDispatchTable> { Tagged {} }
    pub fn set_break_on_entry( arg0 : bool) {}
    pub fn id() -> i32 { 0 }
    pub fn is_liftoff() -> bool { false }
    pub fn wrapper() -> Tagged<Code>{ Tagged {} }

    // Dummy definitions for types and functions used in the converted code
    pub struct Isolate {
    }
    pub struct CPURegister {
        reg: Register
    }
    pub struct Frame {}

    pub enum class CPURegister::RegisterType {}
    pub struct InstructionOperand {}
    pub struct Instruction {}
    pub struct InstructionMask {}
    pub struct FPUControlRegister {}
    pub struct ZoneObject {}
    pub struct ZonePtrList<T> {}
    pub struct CaseClause {}
    pub struct Operand {}
    pub struct AbortReason {}
    pub struct UnoptimizedCompileFlags {}
    pub struct StoreRepresentation {}
    pub struct GCType {}
    pub struct V8StackFrame {}
    pub struct File {}
    pub struct SaveOptions {}
    pub struct JsonObject {}
    pub struct Builtin {}
    pub struct BytecodeArrayWrapper {}
    pub struct IndirectHandle<T> {}
    pub struct Debug {}
    pub struct DebugScope {}
    pub struct NativeModule {}
    pub struct StackGuard {}
    pub struct MessageTemplate {}
    pub struct CallInterfaceDescriptorData {}
    pub struct TextElement {}
    pub struct CharacterRange {}
    pub struct Local<T> {}
    pub struct CallbacksScope {}
    pub struct Context {}
    pub struct WasmSuspenderObject {}
    pub struct WasmContinuationObject {}
    pub struct JSArrayBuffer {}
    pub struct WasmInternalFunction {}
    pub struct WasmFuncRef {}
    pub struct WasmDispatchTable {}
    pub struct WasmExportedFunctionData {}
    pub struct WasmImportData {}
    pub struct UnionOf<T, U> {}
    pub struct DirectHandle<T> {}
    pub struct MaybeDirectHandle<T> {}
    pub struct WasmJSFunctionData {}
    pub struct WasmTableObject {}
    pub struct MachineType {}
    pub enum class CPURegList { }
    pub struct Vector<T> {}
    pub struct IrRegExpData {}
    pub struct WasmExport {}
    pub struct OpIndex {}
    pub struct DirectHandleList {}
    pub struct TaggedList {}
    pub struct Heap {}
    pub struct LocalHeap {}
    pub enum class RootIndex {}
    pub struct SourcePosition {}
    pub struct FullObjectSlot { }
    pub struct UnsafeSmartPointer {}
    pub struct WeakArrayList {}
    pub struct V8_NODISCARD {}
    pub enum class StoreRepresentation {}
    pub struct Root {}
    pub struct Type {}
    pub struct Range {}
    pub struct CFunction {}
    pub enum This {}
    pub struct Entry {}
    pub struct WasmNull {}
    pub struct FixedArrayBase {}
    pub struct DataViewOp {}
    pub struct ExternalReference {}
    pub struct Value {}
    pub struct FunctionLiteral {}
    pub struct WasmToJsCompilationUnit {}
    pub enum Condition {}
    pub struct WasmCodeRefScope {}
    pub struct WasmCode {}
    pub enum ImportCallKind{}
    pub struct WasmExecutionTimer{}
    pub struct WasmModule{}
    pub struct TypeCanonicalizer{}
    pub struct WasmImportWrapperCache {}
    pub enum wasm::JumpBuffer::Suspended {}
    pub struct WasmImportCompilationResult {}
    pub struct AccountingAllocator {}
    pub struct WasmValue {}
    pub struct JSReceiver {}
    pub struct WasmName{}
    pub struct FutexEmulation {}
    pub struct BigInt{}
    pub enum class LocalHeap::HeapType {}
    pub struct DebugInfo{}
    pub struct CodeWrapper{}
    pub struct Symbol {}
    pub struct AddressOf {}
    pub struct SeqTwoByteString {}
    pub enum class RootKind {}
    pub struct FlagList {}
    pub struct Wtf8 {}
    pub struct BytecodeArray{}
    pub struct String::FlatContent {}
    pub struct JsToWasmWrapperCompilationUnit {}
    pub struct CodeGenerator{}
    pub enum IrregexpImplementation {}
    pub struct NativeError {}
    pub struct ExternalEntityTable {}
    pub struct Table {}
    pub struct MapData {}
    pub struct DataViewAccess{}
    pub enum class SideEffectTag {}
    pub struct WasmFunction{}
    pub struct CodeFinalizationQueue{}
    pub enum class CompilationResult {}
    pub struct WasmExceptionPackage {}
    pub struct V8_INLINE{}
    pub enum wasm::Suspender{
      Active,
      Inactive
    }
    pub struct List {};
    pub struct ZoneList<T> {};
    pub struct ResolvedWasmImport{};
    pub enum MemoryProtection{}
    pub struct ThreadingMode{}
    pub struct Address{}
    pub enum TrapReason{}
    pub enum wasm::TrapId{};
    pub struct UnsafeArray{};
    pub enum MemoryAccessMode{}
    pub enum DataViewOp{};
    pub struct MaybeObject {};
    pub struct WellKnownImport {};
    pub struct JSWeakRef {};
    pub struct UncompiledData{};
    pub struct Function{}
    pub struct AbstractCode{};
    pub struct WasmMemoryObject{};
    pub struct JumpBuffer{};
    pub struct CodeGenerationResults{};
    pub enum wasm::Decoder::NoValidationTag{};
    pub enum CodeKind{};
    pub struct OffHeapInstructionStream{};
    pub enum TieringState{};
    pub struct UnsafeWeakFixedArray{};
    pub struct BytecodeArrayWriter{}
    pub struct RegisterConfiguration{};
    pub struct BytecodeArrayBuilder{};
    pub struct JumpTableOffsets{}
    pub struct InterpreterEntryTable{};
    pub struct OperandGenerator{};
    pub struct CompilationDependencies{};
    pub struct BitVector{};
    pub struct Flags{};
    pub struct RelocInfo{}
    pub struct BaseRegList{}
    pub enum CodeDeoptInfo{};
    pub enum InstrumentationBreakpoint{}
    pub struct AbstractCode_Flags{};
    pub struct UnsafeCode{};
    pub struct StringTable{}
    pub struct DirectArguments{}
    pub struct PropertyCell{}
    pub struct EnumCache{}
    pub struct JSProxy{}
    pub struct LookupCache{}
    pub struct JSTypedArray{}
    pub struct ScriptData{}
    pub enum PromiseState{}
    pub struct MaybeHandle<T>{}
    pub struct TaggedField{}
    pub struct CallResult{}
    pub struct JSAsyncFunctionObject{}
    pub struct OrderedHashSet{}
    pub enum ClassPropertyKind{}
    pub struct JSArrayIterator{}
    pub struct DataHandler{}
    pub struct WasmTypeInfo{}
    pub struct WasmOpcodes{};
    pub struct IrregexpImplementation{};
    pub struct NameCache{};
    pub struct ClassBoilerplateDescription{}
    pub struct JSBoundFunction{}
    pub struct JSCollectionIterator{}
    pub struct JSMap{}
    pub struct JSSet{}
    pub struct StructMap{}
    pub enum JSToWasmConversion{};
    pub enum WasmValueConversion{};
    pub struct WasmException{}
    pub struct ExceptionResult{};
    pub struct TemplateInfo{};
    pub struct StringNameLookup{}
    pub struct TransitionArray{}
    pub struct Element{}
    pub struct ConstantDictionary{}
    pub struct PropertyArray{}
    pub struct TemplateContext{}
    pub struct WasmStackFrames{}
    pub struct PromiseHooks{}
    pub struct WasmCompiledModule{};
    pub struct RuntimeArguments{}
    pub struct VectorOf<T> {}
    pub struct WasmDecoder {};
    pub struct IsOnHeap{};
    pub struct IsSmi{};
    pub struct StackFrameIterator{}
    pub struct StringTableInput{};
    pub struct CodeReference{};
    pub enum Encoding {};
    pub enum WasmOpcode {};
    pub struct WasmType{};
    pub struct TypeFeedbackMetadata{};
    pub struct ClassBoilerplateDescription {}
    pub struct Interpreter {}
    pub struct Assembler {}
    pub struct AssemblerOptions {}
    pub struct RegisterList {}
    pub struct FileOutputBuffer {}
    pub struct InstructionStream {}
    pub struct OffHeapInstructionStream {}
    pub struct TaggedMap{}
    pub struct ByteArrayWriter{};
    pub struct AddressOf{}
    pub struct AddressTable{}
    pub struct CodeAgingHelper{}
    pub struct StackFrame{};
    pub struct ByteVector{};
    pub struct StringSearch{}
    pub struct ByteArrayOutputStream{}
    pub struct UnsafeSmartPointerBase{}
    pub enum PromiseState{}
    pub struct PromiseContext{}
    pub struct TemplateContext{}
    pub struct CompilationUnit{}
    pub struct ByteVectorBuffer {}
    pub struct CodeFinalizationQueue{}
    pub struct SharedFunctionInfo{}
    pub struct CodeGenerator{}
    pub struct RegisterConfiguration{}
    pub struct WasmDebugInfo{}
    pub struct Tagged<T>{}
    pub struct BigIntData {}
    pub struct AbstractInternalClass{}
    pub struct OffHeapCompiledCode{}
    pub struct IsSmiableNumber{}
    pub struct TypeFeedbackCells {}
    pub struct StringUtf8Cache{}
    pub struct MaybeObject{}
    pub struct ScopeInfo{}
    pub struct Script {}
    pub struct BytecodeArrayIterator{}
    pub struct PromiseContext{}
    pub struct FixedArrayBase {}
    pub struct CodeReferenceTable {}
    pub struct ByteArrayInput{}
    pub struct TaggedFieldBase {}
    pub struct FrameSummaries {}
    pub struct AsmWasmData {}
    pub struct WeakCell {}
    pub struct BytecodeFinalizationQeue {}
    pub struct ConstantFeedBackCell {}
    pub struct TaggedInternalClass{}
    pub struct ExternalStringResource{}
    pub struct SharedStructDesciptor{}
    pub struct InternalClassEnumDesciptor{}
    pub enum NumberInput{}
    pub struct NumberDictionary{}
    pub enum SourcePositionTableBuldPolicy {}
    pub struct EnumCacheShape {}
    pub struct SharedStruct {}
    pub struct Array{}
    pub struct MaybeHandleAddress<HeapObject>{
      address: *mut HeapObject
    }
    impl MaybeHandleAddress<HeapObject> {
        pub fn new() -> Self {
            Self { address: std::ptr::null_mut() }
        }
        pub fn address(&self) -> *mut HeapObject {
            self.address
        }
    }
    pub struct InternalClassStaticFields {}
    pub struct ExternalResource {};
    pub struct SourcePositionTableBuilder {}
    pub struct Bytevector {length_ : i32}
    impl Bytevector { pub fn length() -> i32{ 10 }}
    pub struct PropertyAccessDetails {}
    pub struct SourcePositionTableIterator {}
    pub struct EnumIterator{}
    pub struct Bigint(){}
    pub struct PropertyCells{}
    pub struct InternalClass {}
    pub struct FixedDoubleArray {}
    pub struct UnsafeSmartPointerBuffer {}
    pub struct DataViewOp{}
    pub struct StructDesciptor{}
    pub struct FunctionTemplateInfo{}
    pub enum SchedularStackData {}
    pub struct MapCache {}
    pub struct NativeRegexpMacroAssembler {}
    pub struct WasmDecoder {}
    pub struct Isolate::PerIsolateThreadData {}
    pub struct MapType {}
    pub enum Bytecode {}
    pub struct RelocInfo {}
    pub struct WasmValueConv {}
    pub struct Scope_SharedClass {}
    pub struct ByteArrayFixedArrayStats {}
    pub struct WasmCode {}
    pub struct JSTypedArrayBase {}
    pub struct MaybeObjectPointer {}
    pub struct InternalFieldMapData {}
    pub struct WasmExceptionConv {}
    pub struct ObjectInYoungGenerationState {}
    pub struct NameDictionary {}
    pub struct OrderedHashTable {}
    pub struct StructDescription {}
    pub struct TransitionTrieDictionary {}
    pub struct DescriptorArray {}
    pub struct JSObjectWithMap {}
    pub struct TransitionArray {}
    pub struct Descriptor {}
    pub struct Scope{}
    pub struct StackFrameIteratorFrameOrEntry{}
    pub struct SourcePositionTable {}
    pub struct SharedFunctionInfo{}
    pub struct OrderedHashMap {}
    pub struct FeedbackCells {}
    pub enum CodeSpecialization{}
    pub struct JSString {}
    pub struct JSFunctionResult {}
    pub enum CodeKindHint {}
    pub struct ObjectSlot {}
    pub struct Code{}
    pub struct FixedArrayByteData {};
    pub enum JSTypedArrayBaseEnum {}
    pub struct HashTable{}
    pub struct JSValueCache{}
    pub struct StringUtf8Table{}
    pub struct MapCacheTable {}
    pub struct NameToScope {}
    pub struct FastNameDictionary{}
    pub struct JSObjectArray {}
    pub struct ScriptContextTable {}
    pub enum UnwindState{}
    pub struct ElementArray {}
    pub struct ScopeSharedClass{}
    pub struct AbstractCodeTypeFeedbackMap{}
    pub struct DirectHandleListHelper{}
    pub struct ObjectSlotBase{}
    pub struct WasmValue{}
    pub struct TransitionTable {}
    pub struct UnsafeFixedArray {}
    pub struct JsFunctionResult {}
    pub struct NativeContext{}
    pub struct IsolateHeapIterable {}
    pub struct ScriptDataDesciptor {}
    pub struct PromiseRejectionTracker {}
    pub struct JSGlobalProxy{}
    pub struct TemplateList{}
    pub struct WasmSharedCode {}
    pub struct PromiseHooks{}
    pub struct ScriptDataContext{}
    pub struct ByteArrayOutputStraeam{}
    pub struct WasmGlobal{};
    pub struct FunctionContexts {};
    pub struct BigInt {}

    impl WasmTrustedInstanceData {
        fn memory_size(&self, memory : u32) -> u64 {
          0
        }
        fn memory_base(&self, memory : u32) -> *mut u8 {
          std::ptr::null_mut()
        }
    }

    impl FutexEmulation{
        pub fn WaitWasm32(isolate: *mut Isolate, array_buffer: DirectHandle<JSArrayBuffer>, offset: uintptr_t, expected_value: i32, timeout_ns: i64) -> Tagged<Object>{Tagged{}}
        pub fn WaitWasm64(isolate: *mut Isolate, array_buffer: DirectHandle<JSArrayBuffer>, offset: uintptr_t, expected_value: i64, timeout_ns: i64) -> Tagged<Object>{Tagged{}}
    }

    impl WasmTableObject{
        pub fn Grow(isolate: *mut Isolate, table: DirectHandle<WasmTableObject>, delta: u32, value: DirectHandle<Object>) -> i32{ 0 }
        pub fn Fill(isolate: *mut Isolate, table: DirectHandle<WasmTableObject>, start: u32, value: DirectHandle<Object>, fill_count: u32){}
        pub fn Get(isolate : *mut Isolate, table: DirectHandle<WasmTableObject>, entry_index: u32) -> DirectHandle<Object>{DirectHandle{}}
        pub fn Set(isolate: *mut Isolate, table: DirectHandle<WasmTableObject>, entry_index: u32, element: DirectHandle<Object>) -> Result<(), String>{Ok(())}
    }
    impl JSObject{
        pub fn AddProperty(isolate: *mut Isolate, error_obj: DirectHandle<JSObject>, wasm_uncatchable_symbol: DirectHandle<Name>, true_value: DirectHandle<Value>, none: i32){}
        pub fn HasProperty(it : &LookupIterator) -> Result<bool, String>{ Ok(false) }
    }
    impl String{
      pub fn Flatten(isolate : *mut Isolate, string: DirectHandle<String>) -> DirectHandle<String>{ string }
      pub fn WriteToFlat(string: Tagged<String>, dst: *mut u16, start: u32, length: u32){}
    }
    impl FixedArray{
      pub fn RawFieldOfElementAt(self : &Self, segment_offset : u32) -> ObjectSlot{ ObjectSlot{} }
    }
    impl WasmTrustedInstanceData{
        pub fn InitTableEntries(isolate: *mut Isolate, trusted_instance_data: DirectHandle<WasmTrustedInstanceData>, arg0: DirectHandle<WasmTrustedInstanceData>, table_index: u32, elem_segment_index: u32, dst: u32, src: u32, count: u32) -> std::optional<MessageTemplate> { None }
        pub fn CopyTableEntries(isolate: *mut Isolate, trusted_instance_data: DirectHandle<WasmTrustedInstanceData>, table_dst_index: u32, table_src_index: u32, dst: u32, src: u32, count: u32) -> bool{ true }
        pub fn module_object() -> Tagged<Script>{ Tagged {} }
        pub fn tiering_budget_array() -> Tagged<FixedArray> { Tagged {}}
        pub fn GetOrCreateFuncRef(isolate : *mut Isolate, trusted_instance_data: DirectHandle<WasmTrustedInstanceData>, function_index: u32) -> DirectHandle<Object>{ DirectHandle {}}
    }
    pub struct FrameSummariesFrames {}
    impl FrameSummariesFrames{
      pub fn AsWasm(self : &Self) -> Self{ Self{} }
      pub fn SourcePosition(self : &Self) -> i32{0}
      pub fn IsWasm(self : &Self) -> bool { true }
    }
    impl WasmFrame{
      pub fn Summarize(self: &Self) -> FrameSummaries{ FrameSummaries{} }
      pub fn wasm_code(self: &Self) -> Tagged<Code> { Tagged{} }
      pub fn function_index(self: &Self) -> i32 { 0 }
      pub fn trusted_instance_data(self: &Self) -> Tagged<WasmTrustedInstanceData> { Tagged{}}
      pub fn position(self : &Self) -> i32{ 0 }
    }
    pub struct DebugInfo{
      pub fn ClearStepping(isolate : *mut Isolate){}
    }
    impl NativeModule{
        pub fn GetDebugInfo(self: &Self) -> &DebugInfo{&DebugInfo{}}
    }
    impl WasmScript{
      pub fn CheckBreakPoints(isolate: *mut Isolate, script: DirectHandle<Script>, position: i32, id: i32) -> MaybeDirectHandle<FixedArray>{ MaybeDirectHandle{} }
    }
    impl Isolate{
        pub fn stack_pool() -> StackPool {StackPool{}}
        pub fn stack_guard() -> StackGuard {StackGuard{}}
        pub fn roots_table() -> RootIndex {RootIndex{}}
    }

    pub struct FrameSummaries{}
    impl FrameSummaries{
        pub fn frames(self: &Self) -> Vector<FrameSummariesFrames>{ Vector{}}
    }
    impl V8{
        pub fn StackOverflow() -> Tagged<Object>{Tagged{}}
        pub fn HandleInterrupts() -> Tagged<Object>{Tagged{}}
    }
    pub struct Scope{}
    pub struct WasmScript {}
    impl SealHandleScope{
        pub fn new(isolate : *mut Isolate) -> Self{Self{}}
    }

    pub struct ThreadLocalTop{}
    impl Isolate{
        pub fn thread_local_top() -> ThreadLocalTop{ ThreadLocalTop{} }
        pub fn c_entry_fp(thread_local_top : ThreadLocalTop) -> Address{ Address{} }
        pub fn IsOnCentralStack() -> bool { true }
        pub fn allow_atomics_wait() -> bool { false }
        pub fn Throw(e : Tagged<JSObject>) -> Tagged<Object>{ Tagged {} }
        pub fn ReThrow(e : Tagged<Object>) -> Tagged<Object>{ Tagged {} }
        pub fn set_context(c: Tagged<Context>){}
        pub fn heap() -> Heap{ Heap{} }
        pub fn wasm_stacks() -> Vec<Box<StackMemory>>{ Vec::new() }
        pub fn wasm_execution_timer() -> WasmExecutionTimer{&WasmExecutionTimer{}}
        pub fn has_exception() -> bool{ false }
        pub fn debug() -> Debug { Debug{}}
        pub fn root(index : RootIndex) -> Tagged<Object> { Tagged{} }
        pub fn clear_pending_exception(){}
        pub fn NewWasmRuntimeError(self: &Self, message: MessageTemplate, args: base::VectorOf<DirectHandle<Object>>) -> DirectHandle<JSObject>{ DirectHandle {} }
        pub fn NewWasmSuspendError(self: &Self, message: MessageTemplate) -> DirectHandle<JSObject>{ DirectHandle {} }
        pub fn NewTypeError(self: &Self, message: MessageTemplate) -> Tagged<JSObject>{ Tagged {} }
        pub fn NewRangeError(self: &Self, message: MessageTemplate) -> Tagged<JSObject>{ Tagged {} }

    }
    pub struct String{
        byte : i8
    }
    impl String{
      pub fn EnsureHash(&self) -> u32 {0}
    }
    pub struct StackMemory{
      size : i32
    }
    impl StackMemory{
      pub fn jmpbuf() -> JumpBuffer { JumpBuffer{} }
      pub fn set_index(&mut self, size : usize){}
      pub fn index(&self) -> usize { 0 }
    }

    pub struct WasmContinuation{}

    pub struct StackPool{}

    impl WasmDispatchTable{
      pub fn InstallCompiledWrapper(index : i32, wrapper : &WasmCode){}
      pub fn sig(&self, table_slot : i32) -> wasm::CanonicalTypeIndex{ wasm::CanonicalTypeIndex{index : 10}}
      pub fn offheap_data() -> Object{ Object{} }
    }

    pub enum class UnwindReason{
      UncaughtException,
      Terminate
    }

    impl DisallowGarbageCollection{
        pub fn new() -> Self{ Self{}}
    }
    impl ClearThreadInWasmScope{
      pub fn new(isolate: *mut Isolate) -> Self{Self{}}
    }

    pub struct TrapHandlerThrowWasmError{}
    impl TrapHandlerThrowWasmError{
        pub fn throwWasmmError(isolate : *mut Isolate)-> Result<(), String>{Ok(())}
    }
    pub struct WasmmError;

    impl SealHandleScope{
      pub fn new(isolate: *mut Isolate) -> Self{Self{}}
    }

    impl Local<WasmSuspenderObject> {
      pub fn Unwrap(self) ->

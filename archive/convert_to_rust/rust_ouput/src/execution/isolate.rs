// Converted from V8 C++ source files:
// Header: isolate.h
// Implementation: isolate.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicU16, AtomicU32, AtomicU64, Ordering};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

pub struct V8_EXECUTION_ISOLATE_H_ {}

pub struct Isolate {
}

pub struct Address {}

pub struct Object {}

pub struct Context {}

pub struct NativeContext {}

pub struct String_ {}

pub struct JSGlobalObject {}

pub struct JSGlobalProxy {}

pub struct FixedArray {}

pub struct Script {}

pub struct WasmInstanceObject {}

pub struct CallSiteInfo {}

pub struct StackFrameInfo {}

pub struct JSMessageObject {}

pub struct External {}

pub struct Tagged<T> {}

pub struct Heap {
}

pub struct ReadOnlyHeap {}

pub struct Handle<T> {}

pub struct DirectHandle<T> {}

pub struct MaybeHandle<T> {}

pub struct Mutex {}

pub struct JSReceiver {}

pub struct HandleScope {
}

pub struct Promise {}

pub struct SharedFunctionInfo {}

pub struct Deoptimizer {}

pub struct RootVisitor {}

pub struct StringTable {}

pub struct MicrotaskQueue {}

pub struct LocalIsolate {}

pub struct Zone {}

pub struct JSArray {}

pub struct StringForwardingTable {}

pub struct SourceTextModule {}

pub struct Function {}

pub struct PromiseCapability {}

pub enum class MessageTemplate {
    kNonObjectImportArgument,
    kNonStringImportAttributeValue,
    kNoAccess,
    kStackOverflow,
    kUnsupported,
	kPlaceholderOnly,
}

impl Isolate {
    pub fn throw(&self, exception: Tagged<Object>,
                        message: *mut MessageLocation) -> Tagged<Object> {
        Tagged::<Object> {}
    }

    pub fn string_table(&self) -> &StringTable {
        &StringTable {}
    }
}

pub struct PageAllocator {}

pub struct MutexGuardIfOffThread<T> {}

pub struct IsolateGroup {}

impl IsolateGroup {
    fn AcquireDefault() -> *mut IsolateGroup {
        &mut IsolateGroup {}
    }
    fn Release(&self) {}
}

pub struct WasmMemoryObject {}

pub struct PerIsolateThreadData {
	thread_id_:ThreadId,
	stack_limit_: usize,
	thread_state_:*mut ThreadState
}

pub struct ThreadState {}
pub struct Local<'a, T> {
}

pub struct PromiseRejectMessage {}

pub struct ScopeInfo {}

pub struct StringSet {}

pub struct Exception {}

pub struct V8StackFrame {}

pub struct MessageLocation {}

pub struct StringView {}

pub struct ModuleStatus {}

pub struct Wasm {
	
}

pub struct ThreadId{
	valid_: bool,
	id_: i32,
}

impl ThreadId {
	pub fn Current() -> ThreadId{
		ThreadId{valid_:false, id_:0}
	}
	pub fn TryGetCurrent() -> ThreadId{
		ThreadId{valid_:false, id_:0}
	}
	pub fn IsValid(&self) -> bool {
		self.valid_
	}
	pub fn ToInteger(&self) -> i32 {
		self.id_
	}
}

pub enum SourceLocation{
	
}

pub struct Proxy {}

pub enum ModuleImportPhase {}

pub struct V8StackFrame {}

pub struct V8 {}

impl V8 {
    pub fn GetCurrentPlatform() -> *mut V8 {
        &mut V8 {}
    }
}

pub struct TryCatch {}

impl TryCatch {
    pub fn Reset(&mut self) {}
    
	pub fn new(isolate: *mut Isolate) -> TryCatch{
		TryCatch {}
	}
}

pub struct String_ExternalOneByteStringResource{}

pub struct String {}

impl String {
	pub fn Equals(&self, str:&String) -> bool {false}
    fn ToCString(&self) -> CString {
        CString::new("").unwrap()
    }
	pub fn Get(&self,i:i32) -> u16 {
		0
	}
	pub fn length(&self) -> i32 {
		0
	}
    fn IsEqualTo(&self, other: base::Vector<&u8>) -> bool{
        false
    }
}

pub struct Object_ {}

impl Object_ {
}
pub struct FrameSummary {
	frame_type_: i32,
	javascript_:JavaScriptFrameSummary,
	script_: *mut Script,
}

impl FrameSummary {
	fn AsWasmInlined(&self) -> WasmInlinedFrameSummary {
		WasmInlinedFrameSummary{}
	}
	fn AsJavaScript(&self) -> JavaScriptFrameSummary {
		JavaScriptFrameSummary{}
	}
	fn AsWasm(&self) -> WasmFrameSummary {
		WasmFrameSummary{}
	}

	fn is_javascript(&self) -> bool{
		false
	}

	fn is_subject_to_debugging(&self) -> bool{
		false
	}

    fn script(&self) -> *mut Script {
        self.script_
    }
	fn AsBuiltin(&self) -> BuiltinFrameSummary {
		BuiltinFrameSummary{}
	}
	fn IsWasmInterpreted(&self) -> bool{
		false
	}
    fn AsWasmInterpreted(&self) -> WasmInterpretedFrameSummary {
        WasmInterpretedFrameSummary{}
    }
	fn GetTopValidFrame(&self) -> FrameSummary{
		FrameSummary{
			frame_type_:0,
			javascript_:JavaScriptFrameSummary{},
			script_:std::ptr::null_mut()
		}
	}

	
	fn are_source_positions_available(&self) -> bool{
		false
	}
	fn SourcePosition(&self) -> i32 {
		0
	}
	fn native_context(&self) -> &NativeContext {
		&NativeContext {}
	}
}

pub struct JavaScriptFrameSummary {
	abstract_code_: *mut Object
}

pub struct StackFrame {}

impl StackFrame {
	fn is_optimized_js(&self) -> bool {
		false
	}
	
	fn is_javascript(&self) -> bool {
		false
	}
	fn is_builtin(&self) -> bool{
		false
	}

	fn type_(&self) -> i32 {0}
	fn id(&self) -> i32 {
		0
	}
	fn pc(&self) -> *mut Address {std::ptr::null_mut()}
	fn LookupCode(&self) -> *mut Code {std::ptr::null_mut()}
	fn fp(&self) -> Address {Address{}}
	fn sp(&self) -> Address {Address{}}
	fn GetFunctions(&self, arg: &Vec<Handle<SharedFunctionInfo>>){}
	

}

pub struct WasmFrameSummary {}
pub struct WasmInlinedFrameSummary {}
pub struct BuiltinFrameSummary{}

pub struct PromiseReaction {}
impl PromiseReaction {
	pub fn fulfill_handler(&self) -> Tagged<Object> {
		Tagged::<Object>{}
	}
	pub fn reject_handler(&self) -> Tagged<Object> {
		Tagged::<Object>{}
	}

	pub fn next(&self) -> i32 {
		0
	}
	
	pub fn promise_or_capability(&self) -> Tagged<Object> {
		Tagged::<Object>{}
	}
}

pub struct SharedArrayBuffer {}
pub struct JSFunction {}

impl JSFunction {
	pub fn context(&self) -> &Context{
		&Context {}
	}

	pub fn shared(&self) -> &SharedFunctionInfo {
		&SharedFunctionInfo {}
	}

	pub fn code(&self, isolate: &Isolate) -> &Code{
		&Code{}
	}

}

pub struct Object {
    
}

impl Object {
    
    fn Size(&self, base: PtrComprCageBase) -> i32 {
        0
    }
	fn to_owned(&self) -> *const Object {
		self
	}
	pub fn NumberValue(&self) -> f64 {
		0.0
	}
}

pub struct Number{}

pub struct PtrComprCageBase {}

impl String {
	pub fn ptr(&self) -> *const String {
		self
	}
}

pub struct FrameSkipMode {}

pub enum GCType {}

pub struct Code {}
impl Code{
		pub fn marked_for_deoptimization(&self) -> bool {false}
		pub fn instruction_start(&self) -> Address {
			Address{}
		}
		pub fn constant_pool(&self) -> Address{
			Address{}
		}
		fn kind(&self, arg: PtrComprCageBase) -> i32 {0}
    pub fn GetCode(&self) -> &Code {
        self
    }
		pub fn is_turbofanned(&self) -> bool {
			false
		}
    pub fn entrypoint_tag(&self) -> i32 {
       0
    }
    pub fn has_handler_table(&self) -> bool {
        false
    }
	pub fn stack_slots(&self) -> i32 {0}
}

pub struct HandlerTable {}

impl HandlerTable {
	pub fn LookupReturn(&self, offset:i32) -> i32{
		0
	}

	pub fn LookupHandlerIndexForRange(&self, pc:i32) -> i32{
		0
	}

	pub fn GetRangePrediction(&self, index:i32) -> i32{
		0
	}
}

pub struct UnoptimizedJSFrame{}

impl UnoptimizedJSFrame {
	fn ReadInterpreterRegister(&self,arg:i32) -> &Object{
		&Object{}
	}
	fn PatchBytecodeOffset(&self, offest:i32) {}
	fn GetBytecodeArray(&self) -> &BytecodeArray{
		&BytecodeArray {}
	}
	fn LookupExceptionHandlerInTable(&self, context_reg: *mut i32, p2: *mut Address) -> i32 {0}
}

pub struct BytecodeArray {}

impl BytecodeArray {
	fn GetFirstBytecodeAddress(&self) -> Address{
		Address {}
	}
}

pub enum GCFlag {
    kNoFlags,
}

pub struct Tagged_t {}

impl Tagged_t {
	
}
pub struct RootsTable{}
impl RootsTable{
	pub fn slot(&mut self, arg:RootIndex) -> &mut Tagged_t {
		&mut Tagged_t{}
	}
	
}

pub enum RootIndex{
    kPublicSymbolTable,
	kActiveContinuation,
	kFirstRoot,
    kFirstReadOnlyRoot,
    kLastRoot,
	kLastReadOnlyRoot,
  kTheHoleValue,
	kNullValue,
	kHeapArguments,
    kApiSymbolTable,
		kApiPrivateSymbolTable
    ,kReadOnlyRootsCount
	
}

impl Isolate {
	
    fn SetCaptureStackTraceForUncaughtExceptions(&self, capture:bool, limit:i32, option:StackTrace::StackTraceOptions) {}
	
    fn CaptureDetailedStackTrace(&self, limit:i32, option:StackTrace::StackTraceOptions) -> DirectHandle<StackTraceInfo> {
        DirectHandle::<StackTraceInfo>{}
    }

    fn Error(&self) -> Handle<Object>{
        Handle::<Object>{}
    }
}

pub struct StackFrameIterator{}

impl StackFrameIterator {
	fn done(&self) -> bool{
		false
	}

    fn Advance(&mut self) {}
	fn GetCode(&self) -> *const Code {
		std::ptr::null()
	}

    fn LookupCode(&self) -> *const Code{
        std::ptr::null()
    }
    fn GetBytecodeOffset(&self) -> i32{
        0
    }
	fn frame(&self) -> *mut StackFrame {std::ptr::null_mut()}
	
}

impl JSGlobalProxy{
    pub fn GetCreationContext(&self) -> std::option::Option<Tagged<Object>>{
        std::option::Option::Some(Tagged::<Object>{})
    }
}

pub struct Base{}
impl Base {
    
}
pub struct JSObject {}

impl JSObject {
    fn ForceSetPrototype(isolate: &Isolate, js_object: &DirectHandle<JSObject>, object: Handle<Object>) {}
    fn AddProperty(isolate: &Isolate, js_object: &DirectHandle<JSObject>, property_name: &Handle<String>, property: Handle<Object>, store_origin: i32) -> Maybe<bool>{
        Maybe::<bool>::Nothing()
    }
	fn HasRealNamedProperty(isolate: &Isolate, js_object: &DirectHandle<JSGlobalObject>, sab_name: &DirectHandle<String>) -> Maybe<bool>{
		Maybe::<bool>::Nothing()
	}
}

pub struct HandleScopeData {}

impl HandleScopeData{
    fn Initialize(&mut self) {}
}

pub struct Factory {}

impl Factory {
		fn NewCodeObjectForEmbeddedBuiltin(&self, a: &DirectHandle<Code>, instruction_start: Address) -> DirectHandle<Code>{
        DirectHandle::<Code>{}
    }
		fn error_stack_symbol(&self) -> DirectHandle<String>{
        DirectHandle::<String>{}
    }
		fn shared_wasm_memories(&self) -> DirectHandle<WeakArrayList>{
			DirectHandle::<WeakArrayList> {}
		}
		fn NewWeakCell(arg0:&DirectHandle<Object>) -> Handle<Object> {Handle::<Object> {}}
		fn CopyFixedArrayUpTo(&self, a: &DirectHandle<Object>, cnt:i32) -> Handle<FixedArray>{
			Handle::<FixedArray>{}
		}
		fn NewStringFromAsciiChecked(&self, t:&String) -> Handle<Object>{
			Handle::<Object>{}
		}
		fn Error(&self) -> Handle<Object>{
			Handle::<Object>{}
		}

		fn null_value(&self) -> Handle<Object>{
			Handle::<Object>{}
		}
		fn empty_fixed_array(&self) -> Handle<FixedArray>{
			Handle::<FixedArray>{}
		}

    fn NewCallSiteInfo(&self, recv: Cast<UnionOf<JSAny, Hole>>, function: &DirectHandle<UnionOf<Smi, JSFunction>>, code: &DirectHandle<HeapObject>, 
												offset:i32,flags:i32,parameters: &DirectHandle<FixedArray>) ->Handle<CallSiteInfo> {
        Handle::<CallSiteInfo> {}
    }
	fn NewErrorStackData(&self,a:DirectHandle<Object>,b:DirectHandle<StackTraceInfo>) -> Handle<Object> {
        Handle::<Object> {}
    }

	fn NewSymbol(&self) -> DirectHandle<Symbol>{
		DirectHandle::<Symbol> {}
	}

	fn NewPrivateSymbol(&self) -> DirectHandle<Symbol>{
		DirectHandle::<Symbol> {}
	}
    fn NewError(&self, arg:Handle<Object>, msg:MessageTemplate) -> Handle<Object>{
        Handle::<Object>{}
    }
		fn NewTypeError(&self, msg:MessageTemplate) -> Handle<Object>{
			Handle::<Object>{}
		}
	
	fn NewStackFrameInfo(&self, script: Handle<Script>,pos:i32, name:Handle<String>, is_const: bool) -> Handle<StackFrameInfo>{
		Handle::<StackFrameInfo> {}
	}
	fn NewStackTraceInfo(&self, frames:Handle<FixedArray>) -> Handle<StackTraceInfo>{
		Handle::<StackTraceInfo> {}
	}
	fn With_string(&self) -> Handle<String> {
        Handle::<String> {}
    }

	fn InternalizeString(&self, handle: &Handle<String>) -> Handle<String> {
		Handle::<String> {}
	}

}

pub struct Exception {}
impl Exception{
    pub fn rethrow(&mut self) -> MaybeLocal<'static, Value> {
        MaybeLocal::<Value> {}
    }
}

pub struct Value{}
pub struct MaybeLocal<'a, T> {
	
}

pub enum Maybe<T>{
    NothingT,
	JustT(T),
}

impl Maybe<bool>{
    
    fn check(&self){}
	fn Nothing() -> Maybe<bool>{
		Maybe::NothingT
	}
}

impl<T> Maybe<T> {
    fn check(&self){}
}

pub struct FunctionCallbackArguments {}
impl FunctionCallbackArguments {
	fn GetTarget(arg: &v8::FunctionCallbackInfo<v8::Value>) -> Tagged<Object>{
		Tagged::<Object>{}
	}

}
pub struct PromiseReactionJobTask {}

impl PromiseReactionJobTask {
	fn handler(&self) -> Tagged<Object>{
		Tagged::<Object>{}
	}

	fn promise_or_capability(&self) -> Tagged<Object>{
		Tagged::<Object>{}
	}
}

impl<T> From<T> for Local<'static, T> {
    fn from(value: T) -> Self {
        Local {  }
    }
}

impl<T> From<T> for Handle<'static, T> {
    fn from(value: T) -> Self {
        Handle{}
    }
}

impl<T> From<T> for DirectHandle<'static, T> {
    fn from(value: T) -> Self {
        DirectHandle{}
    }
}
pub struct ArrayList{}

impl ArrayList{
    fn AddToEnd(arg0:&Isolate, arg1: &Handle<ArrayList>, arg2:MaybeObjectDirectHandle, 
        arg3: Smi) -> Handle<ArrayList>{
        Handle::<ArrayList>{}
    }
	fn Add(arg0:&Isolate, arg1: &DirectHandle<ArrayList>, arg2: DirectHandle<FeedbackVector>) -> DirectHandle<ArrayList>{
        DirectHandle::<ArrayList>{}
    }
	fn New(arg0:&Isolate, arg1:i32) -> DirectHandle<ArrayList>{
        DirectHandle::<ArrayList>{}
    }

	fn Set(&self, i:i32, o:Object) {}
	
    pub fn length(&self) -> i32{
        0
    }
	
    fn Get(&self, i: i32) -> Tagged<Object>{
        Tagged::<Object> {}
    }
}

pub struct FeedbackVector {}

pub struct LocalHeap {}
impl LocalHeap {
 fn ExecuteMainThreadWhileParked<F>(&mut self, callback: F)
    where
        F: FnOnce() ,
 {
 }
   fn Unpark(&mut self) {}
}
pub struct CodeRange{}

impl CodeRange{
		pub fn embedded_blob_code_copy(&mut self) -> *mut CodeRange{
		std::ptr::null_mut()
	}
	pub fn RemapEmbeddedBuiltins(self: &CodeRange,arg: *mut Isolate, embedded_blob_code: *const u8, embedded_blob_code_size: u32) -> *const u8{
		embedded_blob_code
	}
}

pub struct JSTypedArray {}
pub struct StackTraceInfo{}

impl JSTypedArray {
    
}

pub struct StackFrameIteratorScope<'a>{
}

pub struct SourceLocation {}

impl Heap {
    fn new_space(&self) -> bool {
        true
    }
    fn new_lo_space(&self) -> bool {
        true
    }
    fn ActivateMemoryReducerIfNeeded(&mut self){}

	fn heap_profiler(&mut self) -> &HeapProfiler {
        &HeapProfiler{}
    }

    fn NotifyLoadingStarted(&mut self){}
	

    fn set_public_symbol_table(&mut self, sym: RegisteredSymbolTable){}
	fn set_api_symbol_table(&mut self, sym: RegisteredSymbolTable){}
	fn set_api_private_symbol_table(&mut self, sym: RegisteredSymbolTable){}
    fn GC(&mut self, gctype: GCType){}
    fn StartTearDown(&mut self){}
    fn SetStackStart(&self){}
    fn GetCodeRange(&self) -> *mut CodeRange {std::ptr::null_mut()}
	fn FinalizeIncrementalMarkingAtomicallyIfRunning(arg0:i32) {}
    fn AddGlobal(&mut self, object:Handle<HeapObject>){}
	pub fn AttachString(&self, str:&String){}
    fn NextScriptId(&self) -> i32{0}
	fn YoungSpaceContains(&self, to :*mut Object_) -> bool {false}
    fn OldSpaceContains(&self, obj: &Object_) -> bool {false}
    fn CodeSpaceContains(&self, arg: *const Code) -> bool{false}
    fn SetIsMarkingFlag(&mut self, b: bool){}
		fn AllowInlineAllocation(arg:bool){}

	pub fn ClearKeptObjects(&mut self){}

    pub fn Safepoint() -> bool{
        false
    }
    fn Safepoint(&self) -> &Safepoint{
        &Safepoint{}
    }

	pub fn EnsureSweepingCompleted(mode:i32) {}
		fn CollectAllGarbage(mode:i32, reason:i32){}

	fn set_current_microtask(&mut self, object : Object){}
	fn new_string(&mut self, size:usize) -> Handle<String>{
		Handle::<String>{}
	}

	fn root_list_is_iterable(&self) -> bool{false}

	pub fn safepoint(&self) -> &Safepoint{
		&Safepoint{}
	}
		fn SetUp(&mut self, arg0:&LocalHeap){}

        fn TearDownWithSharedHeap(&mut self) {}
	  fn old_space(&self) -> &OldSpace {
        &OldSpace{}
    }
		  fn code_space(&self) -> &CodeSpace {
        &CodeSpace{}
    }
    fn MonotonicallyIncreasingTimeInMs(&self) -> f64{
        0.0
    }
		
		pub fn set_feedback_vectors_for_profiling_tools(&mut self, handle: Object){
			
		}
    pub fn young_space(&self) -> bool {
        false
    }

    pub fn read_only_space(&self) -> &ReadOnlySpace {
        &ReadOnlySpace{}
    }

    pub fn SetUpFromReadOnlyHeap(&mut self, a: *mut ReadOnlyHeap){}
    fn has_heap_object_allocation_tracker(&self) -> bool{
        false
    }
	fn CreateReadOnlyHeap() -> *mut ReadOnlyHeap{
		std::ptr::null_mut()
	}
	pub fn set_global_memento_cache(&mut self, b:i32){}
	fn old_generation_size(&self) -> usize {
		0
	}

		fn MaxOldGenerationSize(&self) -> usize {
        0
    }

	fn incremental_marking(&self) -> &IncrementalMarking{
		&IncrementalMarking {}
	}
        pub fn SetStackStart(&self){}
    fn NewAllocationSite(&mut self) -> Local<'static, Object>{
        Local{}
    }
    fn new_space(&self) -> *mut NewSpace{
        std::ptr::null_mut()
    }
    fn new_lo_space(&self) -> *mut NewLargeObjectSpace{
        std::ptr::null_mut()
    }
		fn cpp_heap_pointer_space(&self) -> &CppHeapPointerTableSpace{
			&CppHeapPointerTableSpace {}
		}
		fn js_dispatch_table_space(&self) -> &JSDispatchTableSpace{
			&JSDispatchTableSpace{}
		}
		pub fn code_region(&self) ->Address{
			Address{}
		}
	    fn SetupSpaces(&mut self, a:i32, b:i32){}
		
      fn GetLocalIsolate(&self) -> &LocalIsolate{
        &LocalIsolate{}
    }
			
  	fn trusted_pointer_space(&self) -> &TrustedPointerSpace{
		&TrustedPointerSpace {}
	}

    fn FatalProcessOutOfMemory(&self, str: &str){}
}

pub struct Truste dPointerSpace {}
pub struct CppHeapPointerTableSpace{}
pub struct JSDispatchTableSpace{}
pub struct WasmFrame{}
pub struct NewSpace {}
pub struct WasmCodeLookupCache{}
pub struct WasmEngine{}

impl WasmEngine{
		pub fn AddIsolate(is: &Isolate){}

	pub fn RemoveIsolate(is: &Isolate){}
		fn GetWasmEngine() -> &WasmEngine {
        &WasmEngine{}
    }

	pub fn NewOrphanedGlobalHandle(handle:*mut WasmOrphanedGlobalHandle) -> *mut WasmOrphanedGlobalHandle{
		std::ptr::null_mut()
	}
	pub fn FreeAllOrphanedGlobalHandles(handle:*mut WasmOrphanedGlobalHandle){}

	pub fn DeleteCompileJobsOnIsolate(iso: *mut Isolate) {}
	pub fn DumpAndResetTurboStatistics() {}

}

pub struct WasmOrphanedGlobalHandle {}

pub struct ZoneChunkListIterator<T,B,C>{
}
impl<T,B,C> ZoneChunkListIterator<T,B,C> {
	fn find(&mut self, index: usize) -> Self {
		ZoneChunkListIterator::<T,B,C>{}
	}
}

pub struct CodeSpace{}
impl CodeSpace {
	fn first_page(&self) -> bool{false}
}
pub struct Safepoint{}
impl Safepoint {
    fn local_heaps_mutex_ -> Mutex {
        Mutex{}
    }
	
}

pub struct KeyAccumulator{
	
}
impl KeyAccumulator {
   fn GetKeys(isolate: &Isolate, recv: &JSReceiver, e:i32, e2:i32, d: i32) -> Handle<FixedArray> {
       Handle::<FixedArray>{}
   }
}

pub struct OldSpace{}
impl OldSpace {
    fn FirstPageAddress(&self) -> *mut Object {std::ptr::null_mut()}
}

pub struct StringStream{
	
}

impl StringStream {
	fn ClearMentionedObjectCache(isolate: &Isolate){}
    fn ToString(&mut self, isolate: &Isolate) -> DirectHandle<String>{
       DirectHandle::<String>{}
    }
	fn OutputToStdOut(&self){}

	fn OutputToFile(&self, f:&mut std::fs::File){}
    fn Log(&mut self, this: &Isolate){}
	
    fn IsMentionedObjectCacheClear(&self, a: &Isolate) -> bool{
        false
    }
	fn str(&mut self) -> String { String{} }

  fn AppendString(&mut self, str: &str) {}

  fn AppendCharacter(&mut self, c: char) {}

	
    
}

pub struct HeapStringAllocator {}

pub struct WeakArrayList {}

impl WeakArrayList {
	fn AddToEnd(iso:&Isolate, alist:&Handle<WeakArrayList>, maybe: MaybeObjectDirectHandle, sm:&Smi) -> Handle<WeakArrayList>{
		Handle::<WeakArrayList>{}
	}

	fn New(iso:&Isolate, len:i32) -> DirectHandle<ArrayList>{
		DirectHandle::<ArrayList>{}
	}
	
	fn length(&self) -> i32 {0}

    fn Get(&self, i:i32) -> Tagged<MaybeObject>{
        Tagged::<MaybeObject>{}
    }

	fn Set(&mut self, i:i32, o: Object){}
}

pub struct Smi {}

impl Smi {
    fn zero() -> Smi {Smi {}}

    fn FromInt(a:i32) -> Smi {Smi {}}

    fn ToInt(&self) -> i32{
        0
    }

	fn value(&self) -> i32 { 0 }
		pub fn uninitialized_deserialization_value() -> Self {
				Smi {}
			}
}

pub struct Debug {}

impl Debug {
    pub fn is_active(&self) -> bool{
        false
    }

	pub fn OnThrow(&mut self, exception: &DirectHandle<Object>) -> std::option::Option<Tagged<Object>>{
		std::option::Option::None
	}
    fn clear_restart_frame(&mut self){}
    fn ShouldRestartFrame(&self, id: i32) -> bool{
        false
    }
	fn Unload(&self) {}
}

pub struct NewRawPointer {}
pub struct WasmContinuationObject{}
impl WasmContinuationObject{
   fn stack(&self) -> *mut Object {std::ptr::null_mut()}
	 fn New(a:&Isolate, stack: *mut Object,b:i32, c:i32) -> DirectHandle<WasmContinuationObject>{
		 DirectHandle::<WasmContinuationObject>{}
	}
	
  fn parent(&self) -> Tagged<Object> {
		Tagged::<Object>{}
	}

}

pub struct CodeKind{}

impl CodeKind {
	
}

pub struct DisallowGarbageCollection{}

impl DisallowGarbageCollection {
   fn new() -> Self{ DisallowGarbageCollection{} }

   fn Release(&self){}
}
struct NoExtension {}

impl NoExtension{

}
pub struct TaskObserver {}
pub struct List{}
pub struct AddressRegion{}

pub struct TypeStats{}

pub struct WasmResumeData{}
impl WasmResumeData{
	 fn suspender(&self) -> Tagged<Object> {
		Tagged::<Object>{}
	}
}

pub struct V8FileLogger{
		a: i32
}
impl V8FileLogger {
		pub fn new(arg: *mut Isolate) -> Self{
		V8FileLogger{a:0}
	}
    fn StopProfilerThread(&mut self) {}
	fn LateSetup(&self, i:&Isolate){}
    fn TearDownAndGetLogFile(&mut self) -> *mut std::fs::File {
        std::ptr::null_mut()
    }
	fn sampler(&self) -> *mut Sampler{
		std::ptr::null_mut()
	}
    fn is_logging(&self) -> bool{
        false
    }
    fn is_listening_to_code_events(&self) -> bool {
        false
    }

}

pub struct SourceTextModule_ {}

pub struct MicrotasksScope<'a> {
}

impl<'a> MicrotasksScope<'a> {
	pub fn new(c: &v8::Local<v8::Context>, d: i32) -> Self {
		MicrotasksScope{}
	}
}

pub struct Task {}

pub enum WasmAsyncSuccess{}

impl MicrotaskQueue {
    pub fn SetUpDefaultMicrotaskQueue(isolate: *mut Isolate) {}
	pub fn microtasks_policy(&self) -> v8::MicrotasksPolicy{
		v8::MicrotasksPolicy::kExplicit
	}
	
        pub fn PerformCheckpoint(&mut self, arg: *mut v8::Isolate) {}
}

pub struct UnwindInfo{}

pub mod base {
	
    pub struct Vector<T>{
        len_: usize,
		d: T
    }

    impl<T> Vector<T> {
        pub fn data(&self) -> &T {
			&self.d
		}
	    pub fn len(&self) -> usize {
        self.len_
    }
	 fn new_uninitialized(s:usize) -> Vector<T>{
				Vector {
					len_:s,
					d:T
				}
			}
    }

pub struct Default<'a, T>{
				va: *mut T,
				d: &'a i32
			}
impl <'a, T> Default<'a, T>{
				fn this(&self) -> &T{
					unsafe {&*self.va}
				}
			}

    pub struct AddressRegion{
        base_: usize,
        size_:usize
    }

	impl AddressRegion {
		
        pub fn contains(&self, addr:Address) -> bool{
            false
        }
	}
pub struct MonotonicallyIncreasingTime {
	
}
pub struct AtomicObject{}
pub struct AtomicUsize {}
pub struct AtomicBool{}
	pub struct OStream{}
	pub fn PrintF(f: *mut std::fs::File,  msg: &str) {}

	pub struct RecursiveMutex{}
    impl RecursiveMutex{
        pub fn Lock(&mut self){}
        pub fn Unlock(&mut self){}
        pub fn TryLock(&mut self) -> bool{false}
    }
        pub struct MutexGuard{}
impl MonotonicallyIncreasingTime {
 pub fn now(&self) -> f64 {
		return 0.0
    }
}

 impl OStream{
        pub fn flush(&self){}
    }

    pub struct VectorOf<T>{
	}
	impl <T> VectorOf <T>{
        
         pub fn new (s:usize) -> Vector<T>{
				Vector {
					len_:s,
					d: T
				}
			}
    }
pub mod OS {
	pub fn GetCurrentProcessId() -> i32{
		0
	}

	pub fn Abort(){
			
	}
    
    pub fn VPrint(format: &str, arguments: core::fmt::Arguments){}
    pub fn PrintError(s:&str){}
	
    pub fn PrintError(s:&str, arg: usize){}
	pub fn AdjustSchedulingParams(){}
	pub fn Fclose(f: *mut std::fs::File) -> i32 {0}
	
}
}

pub struct HandleScopeImplementer{}
impl HandleScopeImplementer {
	pub fn new(isolate: &Isolate) -> Self {
		HandleScopeImplementer {}
	}
}

pub struct KeyedAccessArguments{}

pub mod v8 {
	pub mod internal{
		pub const kRootRegister: i32 = 0;
		pub fn GetRoot(
			i: *mut v8::Isolate,
    		a:i32,
		) -> *mut i::Object { std::ptr::null_mut()}
	}
    
pub struct Global<T> {}
	
	
		pub mod Symbol{
		}
	
    pub enum MicrotasksPolicy {
        kExplicit
    }
	
	
        
		#[derive(PartialEq)]
		

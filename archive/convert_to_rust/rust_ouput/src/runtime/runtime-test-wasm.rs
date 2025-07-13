// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-test-wasm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
    }
    impl<T> Vector<T> {
        pub fn begin(&self) -> *const T {
            std::ptr::null()
        }
        pub fn end(&self) -> *const T {
            std::ptr::null()
        }
    }
}
pub mod v8 {
    pub struct Isolate {}
    impl Isolate {
        pub fn SetWasmModuleCallback(&mut self, _callback: fn(_: &v8::FunctionCallbackInfo<v8::Value>) -> bool) {}
        pub fn SetWasmInstanceCallback(&mut self, _callback: fn(_: &v8::FunctionCallbackInfo<v8::Value>) -> bool) {}
        pub fn SetAllowWasmCodeGenerationCallback(&mut self, _callback: fn(_: v8::Local<v8::Context>, _: v8::Local<v8::String>) -> bool) {}
    }
    pub struct FunctionCallbackInfo<T> {}
    impl<T> FunctionCallbackInfo<T> {
        pub fn GetIsolate(&self) -> *mut Isolate {
            std::ptr::null_mut()
        }
    }
    pub struct Local<'a, T> {
        dummy: i32,
        _phantom: std::marker::PhantomData<&'a T>,
    }
    pub struct Context {}
    pub struct String {}
    impl String {
        pub fn NewFromOneByte(_isolate: *mut Isolate, _buffer: *const u8) -> Local<'static, String> {
            Local{dummy : 10, _phantom: std::marker::PhantomData}
        }
    }
    pub struct Value {}
    impl Value {
        pub fn IsArrayBuffer(&self) -> bool {
            false
        }
        pub fn IsArrayBufferView(&self) -> bool {
            false
        }
        pub fn IsWasmModuleObject(&self) -> bool {
            false
        }
    }
    impl<'a> Local<'a, Value> {
        pub fn As<T>(&self) -> Local<'a, T> {
            Local{dummy : 10, _phantom: std::marker::PhantomData}
        }
    }
    pub struct ArrayBuffer {}
    impl ArrayBuffer {
        pub fn ByteLength(&self) -> u32 {
            0
        }
    }
    pub struct ArrayBufferView {}
    impl ArrayBufferView {
        pub fn ByteLength(&self) -> u32 {
            0
        }
    }
    pub struct WasmModuleObject {}
    impl WasmModuleObject {
        pub fn GetCompiledModule(&self) -> CompiledModule {
            CompiledModule{}
        }
    }
    pub struct Exception {}
    impl Exception {
        pub fn RangeError(_message : Local<'static, String>) -> Local<'static, Value> {
            Local{dummy : 10, _phantom: std::marker::PhantomData}
        }
    }
}
pub mod wasm {
    pub struct NativeModule {
        lazy_compile_frozen: bool,
    }
    impl NativeModule {
        pub fn compilation_state(&self) -> &CompilationState {
            &CompilationState{}
        }
        pub fn module(&self) -> *const WasmModule {
            std::ptr::null()
        }
        pub fn enabled_features(&self) -> EnabledFeatures {
            EnabledFeatures{}
        }
        pub fn wire_bytes(&self) -> base::Vector<u8> {
            base::Vector{}
        }
        pub fn set_lazy_compile_frozen(&mut self, value: bool) {
            self.lazy_compile_frozen = value;
        }
        pub fn GetCode(&self, _func_index: u32) -> *mut WasmCode {
            std::ptr::null_mut()
        }
        pub fn HasCode(&self, _func_index: u32) -> bool {
            false
        }
    }
    pub struct WasmModule {
        pub num_imported_functions: u32,
    }
    pub struct WasmCode {
        is_liftoff : bool,
        for_debugging : bool,
    }
    impl WasmCode {
        pub fn is_liftoff(&self) -> bool {
            self.is_liftoff
        }
        pub fn for_debugging(&self) -> bool {
            self.for_debugging
        }
    }
    pub struct EnabledFeatures {}
    impl EnabledFeatures {
        pub fn FromFlags() -> Self {
            EnabledFeatures{}
        }
    }
    pub struct CompileTimeImports {}
    pub struct CompilationState {}
    impl CompilationState {
        pub fn failed(&self) -> bool {
            false
        }
    }
    pub struct WasmEngine {}
    impl WasmEngine {
        pub fn SyncCompile(_isolate : *mut v8::Isolate, _enabled_features : EnabledFeatures, _compile_time_imports : CompileTimeImports, _error_thrower : &ErrorThrower, _bytes : base::OwnedVector<u8>) -> MaybeDirectHandle<v8::WasmModuleObject> {
            MaybeDirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn SyncInstantiate(_isolate : *mut v8::Isolate, _error_thrower : &ErrorThrower, _module_object : DirectHandle<v8::WasmModuleObject>, _receiver : Handle<JSReceiver>, _array_buffer : MaybeDirectHandle<v8::ArrayBuffer>) -> MaybeDirectHandle<WasmInstanceObject> {
            MaybeDirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn EnterDebuggingForIsolate(_isolate: *mut v8::Isolate) {}
        pub fn LeaveDebuggingForIsolate(_isolate: *mut v8::Isolate) {}
        pub fn FlushLiftoffCode(&self) -> (usize, usize) {
            (0, 0)
        }
        pub fn TriggerCodeGCForTesting(&self) {}
        pub fn EstimateCurrentMemoryConsumption(&self) -> usize {
            0
        }
        pub fn GetDeoptsExecutedCount(&self) -> i32 {
            0
        }
    }
    pub fn GetWasmEngine() -> &'static WasmEngine {
        static ENGINE: WasmEngine = WasmEngine{};
        &ENGINE
    }
    pub struct ErrorThrower {
        error : bool,
        message : String,
    }
    impl ErrorThrower {
        pub fn error(&self) -> bool {
            self.error
        }
        pub fn error_msg(&self) -> &String {
            &self.message
        }
    }
    pub struct ValueType {
        raw_bits: i32,
    }
    impl ValueType {
        pub fn raw_bit_field(&self) -> i32 {
            self.raw_bits
        }
        pub fn Ref(_type_index : ModuleTypeIndex) -> Self {
            ValueType{raw_bits : 0}
        }
    }
    pub struct ModuleTypeIndex {
        index: u32,
    }
    pub struct WasmInstanceObject {}
    pub struct JSReceiver {}
    pub struct WasmCodeRefScope {}
    pub struct WasmFrame {}
    impl WasmFrame {
        pub fn function_index(&self) -> i32 {
            0
        }
        pub fn position(&self) -> i32 {
            0
        }
        pub fn trusted_instance_data(&self) -> *mut WasmTrustedInstanceData {
            std::ptr::null_mut()
        }
        pub fn wasm_code(&self) -> *mut WasmCode {
            std::ptr::null_mut()
        }
        pub fn cast(_it : DebuggableStackFrameIterator) -> *mut Self {
            std::ptr::null_mut()
        }
    }
    pub struct WasmTrustedInstanceData {}
    impl WasmTrustedInstanceData {
        pub fn module(&self) -> *const WasmModule {
            std::ptr::null()
        }
        pub fn dispatch_table_for_imports(&self) -> Tagged<WasmDispatchTable> {
            Tagged{dummy : 10}
        }
        pub fn dispatch_tables(&self) -> Tagged<ProtectedFixedArray> {
            Tagged{dummy : 10}
        }
        pub fn native_module(&self) -> *mut NativeModule {
            std::ptr::null_mut()
        }
        pub fn has_tags_table(&self) -> bool {
            false
        }
    }
    pub struct WasmDispatchTable {}
    impl WasmDispatchTable {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn target(&self, _i : i32) -> WasmCodePointer {
            WasmCodePointer{}
        }
    }
    pub struct ProtectedFixedArray {}
    impl ProtectedFixedArray {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn get(&self, _table_index : i32) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
    }
    pub struct WasmCodePointer {}
    pub struct WasmFunctionData {}
    impl WasmFunctionData {
        pub fn internal(&self) -> *mut WasmInternalFunction {
            std::ptr::null_mut()
        }
        pub fn instance_data(&self) -> Tagged<WasmTrustedInstanceData> {
            Tagged{dummy : 10}
        }
        pub fn function_index(&self) -> i32 {
            0
        }
    }
    pub struct WasmInternalFunction {}
    impl WasmInternalFunction {
        pub fn call_target(&self) -> WasmCodePointer {
            WasmCodePointer{}
        }
    }
    pub struct WasmExceptionPackage {}
    impl WasmExceptionPackage {
        pub fn GetExceptionTag(_isolate : *mut Isolate, _exception : DirectHandle<WasmExceptionPackage>) -> DirectHandle<Object> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn GetExceptionValues(_isolate : *mut Isolate, _exception : DirectHandle<WasmExceptionPackage>) -> DirectHandle<Object> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
    }
    pub struct JSObject {}
    pub struct WasmToJSObjectResult {}
    pub fn TriggerTierUp(_isolate : *mut Isolate, _trusted_data : Tagged<WasmTrustedInstanceData>, _func_index : i32) {}
    pub fn TierUpNowForTesting(_isolate : *mut Isolate, _trusted_data : Tagged<WasmTrustedInstanceData>, _func_index : i32) {}
}
pub mod internal {
    use super::*;
    pub struct Isolate {
        pub zero_value: i32,
    }
    impl Isolate {
        pub fn heap(&self) -> &Heap {
            &Heap{}
        }
        pub fn factory(&self) -> &Factory {
            &Factory{}
        }
        pub fn counters(&self) -> &Counters {
            &Counters{}
        }
        pub fn IsOnCentralStack(&self) -> bool {
            false
        }
        pub fn random_number_generator(&self) -> &RandomNumberGenerator {
            &RandomNumberGenerator{}
        }
    }
    pub struct HandleScope<'a>(_phantom: std::marker::PhantomData<&'a i32>);
    impl<'a> HandleScope<'a> {
        pub fn new(_isolate: *mut Isolate) -> Self {
            HandleScope(std::marker::PhantomData)
        }
    }
    pub struct SealHandleScope<'a>(_phantom: std::marker::PhantomData<&'a i32>);
    impl<'a> SealHandleScope<'a> {
        pub fn new(_isolate: *mut Isolate) -> Self {
            SealHandleScope(std::marker::PhantomData)
        }
    }
    pub struct ReadOnlyRoots {
        _phantom: std::marker::PhantomData<i32>,
    }
    impl ReadOnlyRoots {
        pub fn undefined_value(&self) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
        pub fn exception(&self) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
        pub fn false_value(&self) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
        pub fn true_value(&self) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
        pub fn wasm_null(&self) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
    }
    pub fn ReadOnlyRoots(_isolate: *mut Isolate) -> ReadOnlyRoots {
        ReadOnlyRoots{_phantom: std::marker::PhantomData}
    }
    pub struct Arguments {}
    impl Arguments {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn at<T>(&self, _index : i32) -> DirectHandle<T> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn smi_value_at(&self, _index : i32) -> i32 {
            0
        }
    }
    pub struct Heap {}
    impl Heap {
        pub fn ToBoolean(&self, _value : bool) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
    }
    pub struct Factory {}
    impl Factory {
        pub fn NewNumberFromSize(&self, _size : usize) -> *mut Tagged<Object> {
            std::ptr::null_mut()
        }
        pub fn NewJSArrayBufferAndBackingStore(&self, _byte_length : usize, _initialized : InitializedFlag) -> Result<DirectHandle<super::v8::ArrayBuffer>, String> {
            Ok(DirectHandle{handle : std::ptr::null_mut()})
        }
        pub fn NewFixedArray(&self, _length : i32) -> DirectHandle<FixedArray> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn NewJSArrayWithElements(&self, _externalized_values : DirectHandle<FixedArray>) -> *mut Tagged<Object> {
            std::ptr::null_mut()
        }
        pub fn NewJSObjectWithNullProto(&self) -> DirectHandle<JSObject> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
    }
    pub struct InitializedFlag {}
    pub struct Smi {
        value: i32,
    }
    impl Smi {
        pub fn FromInt(value: i32) -> Self {
            Smi { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
        pub fn ptr(&self) -> *mut i8 {
            std::ptr::null_mut()
        }
    }
    pub fn IsSmi(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct JSFunction {
        _phantom: std::marker::PhantomData<i32>,
    }
    impl JSFunction {
        pub fn shared(&self) -> Tagged<SharedFunctionInfo> {
            Tagged{dummy : 10}
        }
        pub fn code(&self, _isolate: *mut Isolate) -> Tagged<Code> {
            Tagged{dummy : 10}
        }
    }
    pub fn IsJSFunction(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct SharedFunctionInfo {
        _phantom: std::marker::PhantomData<i32>,
    }
    impl SharedFunctionInfo {
        pub fn HasWasmFunctionData(&self) -> bool {
            false
        }
        pub fn wasm_function_data(&self) -> Tagged<super::wasm::WasmFunctionData> {
            Tagged{dummy : 10}
        }
        pub fn HasAsmWasmData(&self) -> bool {
            false
        }
        pub fn HasBuiltinId(&self) -> bool {
            false
        }
        pub fn builtin_id(&self) -> Builtin {
            Builtin::kNoBuiltinId
        }
        pub fn wasm_js_function_data(&self) -> *mut WasmJSFunctionData {
            std::ptr::null_mut()
        }
    }
    pub struct Code {
        _phantom: std::marker::PhantomData<i32>,
    }
    impl Code {
        pub fn kind(&self) -> CodeKind {
            CodeKind::INVALID
        }
        pub fn builtin_id(&self) -> Builtin {
            Builtin::kNoBuiltinId
        }
    }
    pub struct WasmJSFunctionData {}
    impl WasmJSFunctionData {
        pub fn wrapper_code(&self, _isolate: *mut Isolate) -> Tagged<Code> {
            Tagged{dummy : 10}
        }
        pub fn internal(&self) -> *mut WasmInternalFunction {
            std::ptr::null_mut()
        }
    }
    pub struct Object {
        dummy: i32,
    }
    pub struct JSArrayBuffer {}
    impl JSArrayBuffer {
        pub fn was_detached(&self) -> bool {
            false
        }
        pub fn backing_store(&self) -> *mut std::ffi::c_void {
            std::ptr::null_mut()
        }
        pub fn GetByteLength(&self) -> i32 {
            0
        }
    }
    pub fn IsJSArrayBuffer(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct JSTypedArray {}
    impl JSTypedArray {
        pub fn WasDetached(&self) -> bool {
            false
        }
        pub fn GetBuffer(&self) -> DirectHandle<JSArrayBuffer> {
            DirectHandle{handle : std::ptr::null_mut()}
        }
        pub fn DataPtr(&self) -> *mut std::ffi::c_void {
            std::ptr::null_mut()
        }
        pub fn GetByteLength(&self) -> i32 {
            0
        }
        pub fn byte_offset(&self) -> i32 {
            0
        }
    }
    pub fn IsJSTypedArray(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct WeakArrayList {}
    impl WeakArrayList {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn Get(&self, _i : i32) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
    }
    pub struct JSObject {}
    pub fn IsJSObject(_obj : Tagged<Object>) -> bool {
        false
    }
    pub fn IsWasmInstanceObject(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct WasmInstanceObject {}
    impl WasmInstanceObject {
        pub fn trusted_data(&self, _isolate: *mut Isolate) -> *mut super::wasm::WasmTrustedInstanceData {
            std::ptr::null_mut()
        }
        pub fn module_object(&self) -> *mut Tagged<WasmModuleObject> {
            std::ptr::null_mut()
        }
    }
    pub struct WasmModuleObject {}
    impl WasmModuleObject {
        pub fn native_module(&self) -> *mut super::wasm::NativeModule {
            std::ptr::null_mut()
        }
        pub fn script(&self) -> *mut Script {
            std::ptr::null_mut()
        }
    }
    pub struct Script {
        
    }
    impl Script {
        pub fn wasm_weak_instance_list(&self) -> Tagged<WeakArrayList> {
            Tagged{dummy : 10}
        }
    }
    pub struct Counters {}
    impl Counters {
        pub fn wasm_compiled_export_wrapper(&self) -> &AtomicCounter {
            &AtomicCounter{}
        }
    }
    pub struct AtomicCounter {}
    impl AtomicCounter {
        pub fn GetInternalPointer(&self) -> &std::sync::atomic::AtomicI32 {
            static ZERO : std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
            &ZERO
        }
    }
    pub struct RandomNumberGenerator {}
    impl RandomNumberGenerator {
        pub fn NextInt(&self, _max : i32) -> i32 {
            0
        }
        pub fn NextBytes(&self, _data : *mut u8, _num_bytes : i32) {}
    }
    pub struct FixedArray {}
    impl FixedArray {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn get(&self, _index : i32) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
        pub fn set(&mut self, _i : i32, _value : Tagged<Object>) {}
    }
    pub fn IsFixedArray(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct HeapNumber {}
    impl HeapNumber {
        pub fn value(&self) -> f64 {
            0.0
        }
    }
    pub fn IsHeapNumber(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct Builtins {}
    impl Builtins {
        pub fn EntryOf(_builtin : Builtin, _isolate : *mut Isolate) -> Address {
            Address{}
        }
    }
    pub struct Address {}
    impl Isolate {
        pub fn builtins(&self) -> &Builtins {
            &Builtins{}
        }
        pub fn wasm_switch_to_the_central_stack_counter(&self) -> i32 {
            0
        }
    }
    pub enum Builtin {
        kNoBuiltinId,
        kWasmToJsWrapperAsm,
        kJSToJSWrapper,
        kInstantiateAsmJs,
        kGenericJSToWasmInterpreterWrapper,
    }
    pub enum CodeKind {
        INVALID,
        JS_TO_WASM_FUNCTION,
    }
    pub struct DebuggableStackFrameIterator {
        is_wasm: bool,
        is_wasm_interpreter_entry: bool,
    }
    impl DebuggableStackFrameIterator {
        pub fn new(_isolate : *mut Isolate) -> Self {
            DebuggableStackFrameIterator{is_wasm: false, is_wasm_interpreter_entry: false}
        }
        pub fn done(&mut self) -> bool {
            true
        }
        pub fn Advance(&mut self) {}
        pub fn is_wasm(&self) -> bool {
            self.is_wasm
        }
        pub fn is_wasm_interpreter_entry(&self) -> bool {
            self.is_wasm_interpreter_entry
        }
        pub fn frame(&self) -> *mut WasmFrame {
            std::ptr::null_mut()
        }
    }
    pub fn Cast<T>(obj : Tagged<Object>) -> T {
        unsafe { std::mem::transmute_copy(&obj) }
    }
    pub fn IsBoolean(_obj : Tagged<Object>) -> bool {
        false
    }
    pub struct Boolean {}
    impl Boolean {
        pub fn ToBool(_isolate : *mut Isolate) -> bool {
            false
        }
    }
    pub struct WasmExportedFunction {}
    impl WasmExportedFunction {
        pub fn IsWasmExportedFunction(_obj : Tagged<Object>) -> bool {
            false
        }
        pub fn shared(&self) -> Tagged<SharedFunctionInfo> {
            Tagged{dummy : 10}
        }
    }
    pub struct Tagged<T> {
        dummy: i32,
    }
    pub struct DirectHandle<T> {
        handle: *mut T,
    }
    impl<T> DirectHandle<T> {
        pub fn handle(&self) -> *mut T {
            self.handle
        }
    }
    pub struct MaybeDirectHandle<T> {
        handle: *mut T,
    }
    impl<T> MaybeDirectHandle<T> {
        pub fn ToHandle(&self, _module_object : &mut DirectHandle<T>) -> bool {
            false
        }
    }
    pub struct Fuzzing {}
    impl Fuzzing {
        pub fn CrashUnlessFuzzing(_isolate : *mut Isolate) -> Tagged<Object> {
            Tagged{dummy : 10}
        }
    }
}
pub mod trap_handler {
    pub fn IsTrapHandlerEnabled() -> bool {
        false
    }
    pub fn IsThreadInWasm() -> bool {
        false
    }
    pub fn GetRecoveredTrapCount() -> usize {
        0
    }
}
pub mod std {
    pub mod sync {
        pub struct MutexGuard<'a, T> {
            _phantom: std::marker::PhantomData<&'a T>,
        }
        impl<'a, T> MutexGuard<'a, T> {
            pub fn new(_mutex: &Mutex<T>) -> Self {
                MutexGuard{_phantom: std::marker::PhantomData}
            }
        }
        pub struct Mutex<T> {
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> Mutex<T> {
            pub fn new(_value : T) -> Self {
                Mutex{_phantom: std::marker::PhantomData}
            }
            pub fn Pointer(&self) -> *const Self {
                std::ptr::null()
            }
        }
        pub mod atomic {
            pub struct AtomicI32 {}
            impl AtomicI32 {
                pub fn new(_value : i32) -> Self {
                    AtomicI32{}
                }
                pub fn load(&self) -> i32 {
                    0
                }
            }
        }
    }
}
pub mod zone {
    pub struct Zone {}
    impl Zone {
        pub fn new(_allocator : *mut Allocator, _name : &str) -> Self {
            Zone{}
        }
    }
}
pub mod allocator {
    pub struct Allocator {}
}
pub mod base {
    pub struct OwnedVector<T> {}
    impl<T> OwnedVector<T> {
        pub fn begin(&self) -> *const T {
            std::ptr::null()
        }
    }
    pub fn OwnedCopyOf<T>(_data : base::Vector<T>) -> OwnedVector<T> {
        OwnedVector{}
    }
}
pub mod v8_flags {
    pub static mut wasm_lazy_validation: bool = false;
    pub static mut fuzzing: bool = false;
    pub static mut jitless: bool = false;
    pub static mut wasm_jitless: bool = false;
    pub static mut wasm_max_module_size: usize = 0;
}
use internal::*;

fn CrashUnlessFuzzing(isolate: *mut Isolate) -> Tagged<Object> {
    unsafe {
        if v8_flags::fuzzing {
            return ReadOnlyRoots(isolate).undefined_value();
        }
        panic!("CrashUnlessFuzzing");
    }
}

struct WasmCompileControls {
    MaxWasmBufferSize: u32,
    AllowAnySizeForAsync: bool,
}

use std::collections::HashMap;
use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref g_PerIsolateWasmControls: Mutex<HashMap<*mut v8::Isolate, WasmCompileControls>> = Mutex::new(HashMap::new());
}

fn IsWasmCompileAllowed(isolate: *mut v8::Isolate, value: v8::Local<v8::Value>, is_async: bool) -> bool {
    let guard = g_PerIsolateWasmControls.lock().unwrap();
    if !guard.contains_key(&isolate) {
        return false;
    }
    let ctrls = guard.get(&isolate).unwrap();
    (is_async && ctrls.AllowAnySizeForAsync) ||
        (value.IsArrayBuffer() && value.As::<v8::ArrayBuffer>().ByteLength() <= ctrls.MaxWasmBufferSize) ||
        (value.IsArrayBufferView() && value.As::<v8::ArrayBufferView>().ByteLength() <= ctrls.MaxWasmBufferSize)
}

fn IsWasmInstantiateAllowed(isolate: *mut v8::Isolate, module_or_bytes: v8::Local<v8::Value>, is_async: bool) -> bool {
    let guard = g_PerIsolateWasmControls.lock().unwrap();
    if !guard.contains_key(&isolate) {
        return false;
    }
    let ctrls = guard.get(&isolate).unwrap();
    if is_async && ctrls.AllowAnySizeForAsync { return true; }
    if !module_or_bytes.IsWasmModuleObject() {
        return IsWasmCompileAllowed(isolate, module_or_bytes, is_async);
    }
    let module = module_or_bytes.As::<v8::WasmModuleObject>();
    unsafe {
        (module.GetCompiledModule().GetWireBytesRef().size() as u32) <= ctrls.MaxWasmBufferSize
    }
}

fn NewRangeException(isolate: *mut v8::Isolate, message: &str) -> v8::Local<'static, v8::Value> {
    let message_local = v8::String::NewFromOneByte(isolate, message.as_ptr()).ToLocalChecked();
    v8::Exception::RangeError(message_local)
}

fn ThrowRangeException(isolate: *mut v8::Isolate, message: &str) {
    let exception = NewRangeException(isolate, message);
    unsafe { (*isolate).ThrowException(exception); }
}

fn WasmModuleOverride(info: &v8::FunctionCallbackInfo<v8::Value>) -> bool {
    unsafe {
        if IsWasmCompileAllowed(info.GetIsolate(), info[0], false) { return false; }
        ThrowRangeException(info.GetIsolate(), "Sync compile not allowed");
        return true;
    }
}

fn WasmInstanceOverride(info: &v8::FunctionCallbackInfo<v8::Value>) -> bool {
    unsafe {
        if IsWasmInstantiateAllowed(info.GetIsolate(), info[0], false) { return false; }
        ThrowRangeException(info.GetIsolate(), "Sync instantiate not allowed");
        return true;
    }
}

#[no_mangle]
pub extern "C" fn Runtime_SetWasmCompileControls(args: Arguments, isolate: *mut Isolate) -> Tagged<Object> {
    unsafe {
        let scope = HandleScope::new(isolate);
        if args.length() != 2 || !IsSmi(args.at::<Object>(0).handle() as Tagged<Object>) || !IsBoolean(args.at::<Object>(1).handle() as Tagged<Object>) {
            return CrashUnlessFuzzing(isolate);
        }
        let v8_isolate = isolate as *mut v8::Isolate;
        let block_size = args.smi_value_at(0);
        let allow_async = Cast::<Boolean>(args.at::<Object>(1).handle() as Tagged<Object>).ToBool(isolate);

        let mut guard = g_PerIsolateWasmControls.lock().unwrap();
        let ctrl = guard.entry(v8_isolate).or_insert(WasmCompileControls {
            MaxWasmBufferSize: 0,
            AllowAnySizeForAsync: false,
        });

        ctrl.AllowAnySizeForAsync = allow_async;
        ctrl.MaxWasmBufferSize = block_size as u32;
        (*v8_isolate).SetWasmModuleCallback(WasmModuleOverride);
        ReadOnlyRoots(isolate).undefined_value()
    }
}

#[no_mangle]
pub extern "C" fn Runtime_SetWasmInstantiateControls(args: Arguments, isolate: *mut Isolate) -> Tagged<Object> {
    unsafe {
        let scope = HandleScope::new(isolate);
        let v8_isolate = isolate as *mut v8::Isolate;
        (*v8_isolate).SetWasmInstanceCallback(WasmInstanceOverride);
        ReadOnlyRoots(isolate).undefined_value()
    }
}

fn PrintIndentation(stack_size: i32) {
    const MAX_DISPLAY: i32 = 80;
    if stack_size <= MAX_DISPLAY {
        print!("{:4}:{:<width$}", stack_size, "", width = stack_size as usize);
    } else {
        print!("{:4}:{:<width$}", stack_size, "...", width = MAX_DISPLAY as usize);
    }
}

fn WasmStackSize(isolate: *mut Isolate) -> i32 {
    let mut n = 0;
    unsafe {
        let mut it = DebuggableStackFrameIterator::new(isolate);
        while !it.done() {
            if it.is_wasm() { n += 1; }
            it.Advance();
        }
    }
    n
}

#[no_mangle]
pub extern "C" fn Runtime_CountUnoptimizedWasmToJSWrapper(args: Arguments, isolate: *mut Isolate) -> Tagged<Object> {
    unsafe {
        let shs = SealHandleScope::new(isolate);
        if args.length() != 1 || !IsWasmInstanceObject(args.at::<Object>(0).handle() as Tagged<Object>) {
            return CrashUnlessFuzzing(isolate);
        }
        let instance_object = Cast::<WasmInstanceObject>(args.at::<Object>(0).handle() as Tagged<Object>);
        let trusted_data = (*instance_object.trusted_data(isolate));
        let wrapper_entry = Builtins::EntryOf(Builtin::kWasmToJsWrapperAsm, isolate);

        let mut result = 0;
        let dispatch_table = trusted_data.dispatch_table_for_imports();
        let import_count = dispatch_table.length();
        let

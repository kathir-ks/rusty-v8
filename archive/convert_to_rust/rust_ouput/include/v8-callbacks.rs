// Converted from V8 C++ source files:
// Header: v8-callbacks.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::os::raw::c_char;
use std::string::String as StdString;
use std::result;

//use v8_array_buffer::SharedArrayBuffer;
//use v8_template::Promise;
//use v8_primitive::Context;
//use v8_isolate::Isolate;
//use v8_exception::Message;
//use v8_forward::Module;
//use v8_exception::Object;
//use v8_exception::Promise;
//use v8_isolate::ScriptOrModule;
//use v8_exception::String;
//use v8_exception::Value;

pub type MaybeLocal<'a, T> = Result<&'a T, ()>;
pub type Local<'a, T> = &'a T;
pub type Global<T> = T;

pub struct FixedArray {}
pub struct Data {}

pub trait Value {}
impl Value for i32 {}
impl Value for String {}
impl Value for Object {}

pub struct String {}
pub struct Object {}
pub struct Isolate {}
pub struct Message {}
pub struct Context {}
pub struct Module {}
pub struct ScriptOrModule {}
pub struct UnboundScript {}
pub struct Promise {
    pub resolver: PromiseResolver,
}

pub struct PromiseResolver {}

impl PromiseResolver {
    pub fn resolve(&self, _context: Local<'static, Context>, _value: Local<'static, dyn Value>) -> Result<bool, Error> {
        Ok(true)
    }

    pub fn reject(&self, _context: Local<'static, Context>, _value: Local<'static, dyn Value>) -> Result<bool, Error> {
        Ok(false)
    }
}

pub struct SharedArrayBuffer {}

pub enum Error {
    GenericError,
}

pub type Result<T, E> = result::Result<T, E>;

/**
 * A JIT code event is issued each time code is added, moved or removed.
 *
 * \note removal events are not currently issued.
 */
#[derive(Debug)]
pub struct JitCodeEvent {
    pub type_: EventType,
    pub code_type: CodeType,
    // Start of the instructions.
    pub code_start: *mut std::ffi::c_void,
    // Size of the instructions.
    pub code_len: usize,
    // Script info for CODE_ADDED event.
    pub script: Local<'static, UnboundScript>,
    // User-defined data for *_LINE_INFO_* event. It's used to hold the source
    // code line information which is returned from the
    // CODE_START_LINE_INFO_RECORDING event. And it's passed to subsequent
    // CODE_ADD_LINE_POS_INFO and CODE_END_LINE_INFO_RECORDING events.
    pub user_data: *mut std::ffi::c_void,

    pub wasm_source_info: *mut wasm_source_info_t,

    pub data: JitCodeEventData,

    pub isolate: *mut Isolate,
}

#[derive(Debug)]
pub enum JitCodeEventData {
    Name(name_t),
    LineInfo(line_info_t),
    NewCodeStart(*mut std::ffi::c_void),
    None,
}

#[derive(Debug)]
pub enum EventType {
    CODE_ADDED,
    CODE_MOVED,
    CODE_REMOVED,
    CODE_ADD_LINE_POS_INFO,
    CODE_START_LINE_INFO_RECORDING,
    CODE_END_LINE_INFO_RECORDING,
}
// Definition of the code position type. The "POSITION" type means the place
// in the source code which are of interest when making stack traces to
// pin-point the source location of a stack frame as close as possible.
// The "STATEMENT_POSITION" means the place at the beginning of each
// statement, and is used to indicate possible break locations.
#[derive(Debug)]
pub enum PositionType {
    POSITION,
    STATEMENT_POSITION,
}

// There are three different kinds of CodeType, one for JIT code generated
// by the optimizing compiler, one for byte code generated for the
// interpreter, and one for code generated from Wasm. For JIT_CODE and
// WASM_CODE, |code_start| points to the beginning of jitted assembly code,
// while for BYTE_CODE events, |code_start| points to the first bytecode of
// the interpreted function.
#[derive(Debug)]
pub enum CodeType {
    BYTE_CODE,
    JIT_CODE,
    WASM_CODE,
}

#[derive(Debug)]
pub struct name_t {
    // Name of the object associated with the code, note that the string is not
    // zero-terminated.
    pub str_: *const c_char,
    // Number of chars in str.
    pub len: usize,
}

#[derive(Debug)]
pub struct line_info_t {
    // PC offset
    pub offset: usize,
    // Code position
    pub pos: usize,
    // The position type.
    pub position_type: PositionType,
}

#[derive(Debug)]
pub struct wasm_source_info_t {
    // Source file name.
    pub filename: *const c_char,
    // Length of filename.
    pub filename_size: usize,
    // Line number table, which maps offsets of JITted code to line numbers of
    // source file.
    pub line_number_table: *const line_info_t,
    // Number of entries in the line number table.
    pub line_number_table_size: usize,
}

/**
 * Option flags passed to the SetJitCodeEventHandler function.
 */
#[derive(Debug)]
pub enum JitCodeEventOptions {
    kJitCodeEventDefault = 0,
    // Generate callbacks for already existent code.
    kJitCodeEventEnumExisting = 1,

    kLastJitCodeEventOption = kJitCodeEventEnumExisting as isize,
}

/**
 * Callback function passed to SetJitCodeEventHandler.
 *
 * \param event code add, move or removal event.
 */
pub type JitCodeEventHandler = extern "C" fn(event: *const JitCodeEvent);

// --- Garbage Collection Callbacks ---

/**
 * Applications can register callback functions which will be called before and
 * after certain garbage collection operations.  Allocations are not allowed in
 * the callback functions, you therefore cannot manipulate objects (set or
 * delete properties for example) since it is possible such operations will
 * result in the allocation of objects.
 * TODO(v8:12612): Deprecate kGCTypeMinorMarkSweep after updating blink.
 */
#[derive(Debug)]
pub enum GCType {
    kGCTypeScavenge = 1 << 0,
    kGCTypeMinorMarkSweep = 1 << 1,
    kGCTypeMarkSweepCompact = 1 << 2,
    kGCTypeIncrementalMarking = 1 << 3,
    kGCTypeProcessWeakCallbacks = 1 << 4,
    kGCTypeAll = Self::kGCTypeScavenge as isize
        | Self::kGCTypeMinorMarkSweep as isize
        | Self::kGCTypeMarkSweepCompact as isize
        | Self::kGCTypeIncrementalMarking as isize
        | Self::kGCTypeProcessWeakCallbacks as isize,
}

/**
 * GCCallbackFlags is used to notify additional information about the GC
 * callback.
 *   - kGCCallbackFlagConstructRetainedObjectInfos: The GC callback is for
 *     constructing retained object infos.
 *   - kGCCallbackFlagForced: The GC callback is for a forced GC for testing.
 *   - kGCCallbackFlagSynchronousPhantomCallbackProcessing: The GC callback
 *     is called synchronously without getting posted to an idle task.
 *   - kGCCallbackFlagCollectAllAvailableGarbage: The GC callback is called
 *     in a phase where V8 is trying to collect all available garbage
 *     (e.g., handling a low memory notification).
 *   - kGCCallbackScheduleIdleGarbageCollection: The GC callback is called to
 *     trigger an idle garbage collection.
 */
#[derive(Debug)]
pub enum GCCallbackFlags {
    kNoGCCallbackFlags = 0,
    kGCCallbackFlagConstructRetainedObjectInfos = 1 << 1,
    kGCCallbackFlagForced = 1 << 2,
    kGCCallbackFlagSynchronousPhantomCallbackProcessing = 1 << 3,
    kGCCallbackFlagCollectAllAvailableGarbage = 1 << 4,
    kGCCallbackFlagCollectAllExternalMemory = 1 << 5,
    kGCCallbackScheduleIdleGarbageCollection = 1 << 6,
}

pub type GCCallback = extern "C" fn(type_: GCType, flags: GCCallbackFlags);

pub type InterruptCallback = extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void);

pub type PrintCurrentStackTraceFilterCallback =
    extern "C" fn(isolate: *mut Isolate, script_name: Local<String>) -> bool;

/**
 * This callback is invoked when the heap size is close to the heap limit and
 * V8 is likely to abort with out-of-memory error.
 * The callback can extend the heap limit by returning a value that is greater
 * than the current_heap_limit. The initial heap limit is the limit that was
 * set after heap setup.
 */
pub type NearHeapLimitCallback =
    extern "C" fn(data: *mut std::ffi::c_void, current_heap_limit: usize, initial_heap_limit: usize) -> usize;

/**
 * Callback function passed to SetUnhandledExceptionCallback.
 */
#[cfg(defined(V8_OS_WIN))]
pub struct _EXCEPTION_POINTERS {}
#[cfg(defined(V8_OS_WIN))]
pub type UnhandledExceptionCallback = extern "C" fn(exception_pointers: *mut _EXCEPTION_POINTERS) -> i32;

// --- Counters Callbacks ---

pub type CounterLookupCallback = extern "C" fn(name: *const c_char) -> *mut i32;

pub type CreateHistogramCallback =
    extern "C" fn(name: *const c_char, min: i32, max: i32, buckets: usize) -> *mut std::ffi::c_void;

pub type AddHistogramSampleCallback = extern "C" fn(histogram: *mut std::ffi::c_void, sample: i32);

// --- Exceptions ---

pub type FatalErrorCallback = extern "C" fn(location: *const c_char, message: *const c_char);

#[derive(Debug)]
pub struct OOMDetails {
    pub is_heap_oom: bool,
    pub detail: *const c_char,
}

pub type OOMErrorCallback = extern "C" fn(location: *const c_char, details: &OOMDetails);

pub type MessageCallback = extern "C" fn(message: Local<Message>, data: Local<dyn Value>);

// --- Tracing ---

#[derive(Debug)]
pub enum LogEventStatus {
    kStart = 0,
    kEnd = 1,
    kLog = 2,
}
pub type LogEventCallback = extern "C" fn(name: *const c_char, status: i32 /* LogEventStatus */);

// --- Crashkeys Callback ---
#[derive(Debug)]
pub enum CrashKeyId {
    kIsolateAddress,
    kReadonlySpaceFirstPageAddress,
    #[deprecated(since = "Map space got removed")]
    kMapSpaceFirstPageAddress,
    kOldSpaceFirstPageAddress,
    kCodeRangeBaseAddress,
    kCodeSpaceFirstPageAddress,
    kDumpType,
    kSnapshotChecksumCalculated,
    kSnapshotChecksumExpected,
}

pub type AddCrashKeyCallback = extern "C" fn(id: CrashKeyId, value: &StdString);

// --- Enter/Leave Script Callback ---
pub type BeforeCallEnteredCallback = extern "C" fn(isolate: *mut Isolate);
pub type CallCompletedCallback = extern "C" fn(isolate: *mut Isolate);

// --- Modify Code Generation From Strings Callback ---
#[derive(Debug)]
pub struct ModifyCodeGenerationFromStringsResult {
    // If true, proceed with the codegen algorithm. Otherwise, block it.
    pub codegen_allowed: bool,
    // Overwrite the original source with this string, if present.
    // Use the original source if empty.
    // This field is considered only if codegen_allowed is true.
    pub modified_source: MaybeLocal<'static, String>,
}

/**
 * Callback to check if codegen is allowed from a source object, and convert
 * the source to string if necessary. See: ModifyCodeGenerationFromStrings.
 */
pub type ModifyCodeGenerationFromStringsCallback =
    extern "C" fn(context: Local<Context>, source: Local<dyn Value>) -> ModifyCodeGenerationFromStringsResult;
pub type ModifyCodeGenerationFromStringsCallback2 =
    extern "C" fn(context: Local<Context>, source: Local<dyn Value>, is_code_like: bool) -> ModifyCodeGenerationFromStringsResult;

// --- Failed Access Check Callback ---

/**
 * Access type specification.
 */
#[derive(Debug)]
pub enum AccessType {
    ACCESS_GET,
    ACCESS_SET,
    ACCESS_HAS,
    ACCESS_DELETE,
    ACCESS_KEYS,
}

pub type FailedAccessCheckCallback =
    extern "C" fn(target: Local<Object>, type_: AccessType, data: Local<dyn Value>);

// --- WebAssembly compilation callbacks ---
//template <typename T>
//class FunctionCallbackInfo;
pub struct FunctionCallbackInfo {}

pub type ExtensionCallback = extern "C" fn(arg0: &FunctionCallbackInfo) -> bool;

pub type AllowWasmCodeGenerationCallback =
    extern "C" fn(context: Local<Context>, source: Local<String>) -> bool;

// --- Callback for APIs defined on v8-supported objects, but implemented
// by the embedder. Example: WebAssembly.{compile|instantiate}Streaming ---
pub type ApiImplementationCallback = extern "C" fn(arg0: &FunctionCallbackInfo);

// --- Callback for WebAssembly.compileStreaming ---
pub type WasmStreamingCallback = extern "C" fn(arg0: &FunctionCallbackInfo);

#[derive(Debug)]
pub enum WasmAsyncSuccess {
    kSuccess,
    kFail,
}

// --- Callback called when async WebAssembly operations finish ---
pub type WasmAsyncResolvePromiseCallback = extern "C" fn(
    isolate: *mut Isolate,
    context: Local<Context>,
    resolver: Local<PromiseResolver>,
    result: Local<dyn Value>,
    success: WasmAsyncSuccess,
);

// --- Callback for loading source map file for Wasm profiling support
pub type WasmLoadSourceMapCallback = extern "C" fn(isolate: *mut Isolate, name: *const c_char) -> Local<String>;

// --- Callback for checking if WebAssembly imported strings are enabled ---
pub type WasmImportedStringsEnabledCallback = extern "C" fn(context: Local<Context>) -> bool;

// --- Callback for checking if the SharedArrayBuffer constructor is enabled ---
pub type SharedArrayBufferConstructorEnabledCallback = extern "C" fn(context: Local<Context>) -> bool;

// --- Callback for checking if the compile hints magic comments are enabled ---
pub type JavaScriptCompileHintsMagicEnabledCallback = extern "C" fn(context: Local<Context>) -> bool;

// --- Callback for checking if WebAssembly JSPI is enabled ---
pub type WasmJSPIEnabledCallback = extern "C" fn(context: Local<Context>) -> bool;

/**
 * Import phases in import requests.
 */
#[derive(Debug)]
pub enum ModuleImportPhase {
    kSource,
    kEvaluation,
}

/**
 * HostImportModuleDynamicallyCallback is called when we
 * require the embedder to load a module. This is used as part of the dynamic
 * import syntax.
 *
 * The referrer contains metadata about the script/module that calls
 * import.
 *
 * The specifier is the name of the module that should be imported.
 *
 * The import_attributes are import attributes for this request in the form:
 * [key1, value1, key2, value2, ...] where the keys and values are of type
 * v8::String. Note, unlike the FixedArray passed to ResolveModuleCallback and
 * returned from ModuleRequest::GetImportAttributes(), this array does not
 * contain the source Locations of the attributes.
 *
 * The embedder must compile, instantiate, evaluate the Module, and
 * obtain its namespace object.
 *
 * The Promise returned from this function is forwarded to userland
 * JavaScript. The embedder must resolve this promise with the module
 * namespace object. In case of an exception, the embedder must reject
 * this promise with the exception. If the promise creation itself
 * fails (e.g. due to stack overflow), the embedder must propagate
 * that exception by returning an empty MaybeLocal.
 */
pub type HostImportModuleDynamicallyCallback = extern "C" fn(
    context: Local<Context>,
    host_defined_options: Local<Data>,
    resource_name: Local<dyn Value>,
    specifier: Local<String>,
    import_attributes: Local<FixedArray>,
) -> MaybeLocal<'static, Promise>;

/**
 * HostImportModuleWithPhaseDynamicallyCallback is called when we
 * require the embedder to load a module with a specific phase. This is used
 * as part of the dynamic import syntax.
 *
 * The referrer contains metadata about the script/module that calls
 * import.
 *
 * The specifier is the name of the module that should be imported.
 *
 * The phase is the phase of the import requested.
 *
 * The import_attributes are import attributes for this request in the form:
 * [key1, value1, key2, value2, ...] where the keys and values are of type
 * v8::String. Note, unlike the FixedArray passed to ResolveModuleCallback and
 * returned from ModuleRequest::GetImportAttributes(), this array does not
 * contain the source Locations of the attributes.
 *
 * The Promise returned from this function is forwarded to userland
 * JavaScript. The embedder must resolve this promise according to the phase
 * requested:
 * - For ModuleImportPhase::kSource, the promise must be resolved with a
 *   compiled ModuleSource object, or rejected with a SyntaxError if the
 *   module does not support source representation.
 * - For ModuleImportPhase::kEvaluation, the promise must be resolved with a
 *   ModuleNamespace object of a module that has been compiled, instantiated,
 *   and evaluated.
 *
 * In case of an exception, the embedder must reject this promise with the
 * exception. If the promise creation itself fails (e.g. due to stack
 * overflow), the embedder must propagate that exception by returning an empty
 * MaybeLocal.
 *
 * This callback is still experimental and is only invoked for source phase
 * imports.
 */
pub type HostImportModuleWithPhaseDynamicallyCallback = extern "C" fn(
    context: Local<Context>,
    host_defined_options: Local<Data>,
    resource_name: Local<dyn Value>,
    specifier: Local<String>,
    phase: ModuleImportPhase,
    import_attributes: Local<FixedArray>,
) -> MaybeLocal<'static, Promise>;

/**
 * Callback for requesting a compile hint for a function from the embedder. The
 * first parameter is the position of the function in source code and the second
 * parameter is embedder data to be passed back.
 */
pub type CompileHintCallback = extern "C" fn(arg1: i32, arg2: *mut std::ffi::c_void) -> bool;

/**
 * HostInitializeImportMetaObjectCallback is called the first time import.meta
 * is accessed for a module. Subsequent access will reuse the same value.
 *
 * The method combines two implementation-defined abstract operations into one:
 * HostGetImportMetaProperties and HostFinalizeImportMeta.
 *
 * The embedder should use v8::Object::CreateDataProperty to add properties on
 * the meta object.
 */
pub type HostInitializeImportMetaObjectCallback =
    extern "C" fn(context: Local<Context>, module: Local<Module>, meta: Local<Object>);

/**
 * HostCreateShadowRealmContextCallback is called each time a ShadowRealm is
 * being constructed in the initiator_context.
 *
 * The method combines Context creation and implementation defined abstract
 * operation HostInitializeShadowRealm into one.
 *
 * The embedder should use v8::Context::New or v8::Context:NewFromSnapshot to
 * create a new context. If the creation fails, the embedder must propagate
 * that exception by returning an empty MaybeLocal.
 */
pub type HostCreateShadowRealmContextCallback =
    extern "C" fn(initiator_context: Local<Context>) -> MaybeLocal<'static, Context>;

/**
 * IsJSApiWrapperNativeErrorCallback is called on an JSApiWrapper object to
 * determine if Error.isError should return true or false. For instance, in an
 * HTML embedder, DOMExceptions return true when passed to Error.isError.
 */
pub type IsJSApiWrapperNativeErrorCallback =
    extern "C" fn(isolate: *mut Isolate, obj: Local<Object>) -> bool;

/**
 * PrepareStackTraceCallback is called when the stack property of an error is
 * first accessed. The return value will be used as the stack value. If this
 * callback is registed, the |Error.prepareStackTrace| API will be disabled.
 * |sites| is an array of call sites, specified in
 * https://v8.dev/docs/stack-trace-api
 */
pub struct Array {}
pub type PrepareStackTraceCallback = extern "C" fn(
    context: Local<Context>,
    error: Local<dyn Value>,
    sites: Local<Array>,
) -> MaybeLocal<'static, dyn Value>;

#[cfg(defined(V8_OS_WIN))]
/**
 * Callback to selectively enable ETW tracing based on the document URL.
 * Implemented by the embedder, it should never call back into V8.
 *
 * Windows allows passing additional data to the ETW EnableCallback:
 * https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nc-evntprov-penablecallback
 *
 * This data can be configured in a WPR (Windows Performance Recorder)
 * profile, adding a CustomFilter to an EventProvider like the following:
 *
 * <EventProvider Id=".." Name="57277741-3638-4A4B-BDBA-0AC6E45DA56C" Level="5">
 *   <CustomFilter Type="0x80000000" Value="AQABAAAAAAA..." />
 * </EventProvider>
 *
 * Where:
 * - Name="57277741-3638-4A4B-BDBA-0AC6E45DA56C" is the GUID of the V8
 *     ETW provider, (see src/libplatform/etw/etw-provider-win.h),
 * - Type="0x80000000" is EVENT_FILTER_TYPE_SCHEMATIZED,
 * - Value="AQABAAAAAA..." is a base64-encoded byte array that is
 *     base64-decoded by Windows and passed to the ETW enable callback in
 *     the 'PEVENT_FILTER_DESCRIPTOR FilterData' argument; see:
 * https://learn.microsoft.com/en-us/windows/win32/api/evntprov/ns-evntprov-event_filter_descriptor.
 *
 * This array contains a struct EVENT_FILTER_HEADER followed by a
 * variable length payload, and as payload we pass a string in JSON format,
 * with a list of regular expressions that should match the document URL
 * in order to enable ETW tracing:
 *   {
 *     "version": "2.0",
 *     "filtered_urls": [
 *         "https:\/\/.*\.chromium\.org\/.*", "https://v8.dev/";, "..."
 *     ],
 *     "trace_interpreter_frames": true
 *  }
 */
pub type FilterETWSessionByURLCallback =
    extern "C" fn(context: Local<Context>, etw_filter_payload: &StdString) -> bool;

#[cfg(defined(V8_OS_WIN))]
#[derive(Debug)]
pub struct FilterETWSessionByURLResult {
    // If true, enable ETW tracing for the current isolate.
    pub enable_etw_tracing: bool,

    // If true, also enables ETW tracing for interpreter stack frames.
    pub trace_interpreter_frames: bool,
}
#[cfg(defined(V8_OS_WIN))]
pub type FilterETWSessionByURL2Callback =
    extern "C" fn(context: Local<Context>, etw_filter_payload: &StdString) -> FilterETWSessionByURLResult;

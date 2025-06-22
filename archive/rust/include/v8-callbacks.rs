// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod callbacks {
    use std::ffi::{c_char, c_size_t, c_void};
    use std::string::String;

    // TODO: Add cppgc equivalent if needed.
    // use cppgc::common;

    // TODO: Replace with actual Rust types when available in the v8 crate
    pub type Local<T> = *mut T; // Placeholder
    pub type MaybeLocal<T> = *mut T; // Placeholder
    // pub type Value = i32; // Placeholder
    // pub type Object = i32; // Placeholder
    // pub type String = i32; // Placeholder
    pub type Isolate = c_void; // Placeholder
    pub type Message = c_void; // Placeholder
    pub type Module = c_void; // Placeholder
    pub type Promise = c_void; // Placeholder
    pub type ScriptOrModule = c_void; // Placeholder
    pub type UnboundScript = c_void; // Placeholder
    pub type Context = c_void; // Placeholder
    pub type Data = c_void; // Placeholder
    pub type FixedArray = c_void; // Placeholder
    pub type Array = c_void; // Placeholder
    pub type PromiseResolver = c_void;

    // Assuming Value, Object, String, Isolate, Message, Module, Promise, ScriptOrModule, and UnboundScript
    // are defined or available through a v8 crate.  If not, you'll need to define suitable
    // Rust equivalents.  The current placeholders should be replaced.

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct JitCodeEvent {
        pub event_type: JitCodeEventType,
        pub code_type: JitCodeType,
        pub code_start: *mut c_void,
        pub code_len: c_size_t,
        pub script: Local<UnboundScript>,
        pub user_data: *mut c_void,
        pub wasm_source_info: *mut WasmSourceInfo,
        pub data: JitCodeEventData,
        pub isolate: *mut Isolate,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub union JitCodeEventData {
        pub name: Name,
        pub line_info: LineInfo,
        pub new_code_start: *mut c_void,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Name {
        pub str: *const c_char,
        pub len: c_size_t,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct LineInfo {
        pub offset: c_size_t,
        pub pos: c_size_t,
        pub position_type: PositionType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct WasmSourceInfo {
        pub filename: *const c_char,
        pub filename_size: c_size_t,
        pub line_number_table: *const LineInfo,
        pub line_number_table_size: c_size_t,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub enum JitCodeEventType {
        CodeAdded,
        CodeMoved,
        CodeRemoved,
        CodeAddLinePosInfo,
        CodeStartLineInfoRecording,
        CodeEndLineInfoRecording,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub enum PositionType {
        Position,
        StatementPosition,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub enum JitCodeType {
        ByteCode,
        JitCode,
        WasmCode,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum JitCodeEventOptions {
        JitCodeEventDefault = 0,
        JitCodeEventEnumExisting = 1,
        KLastJitCodeEventOption = JitCodeEventOptions::JitCodeEventEnumExisting as isize,
    }

    pub type JitCodeEventHandler =
        Option<unsafe extern "C" fn(event: *const JitCodeEvent)>;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GCType {
        GCTypeScavenge = 1 << 0,
        GCTypeMinorMarkSweep = 1 << 1,
        GCTypeMarkSweepCompact = 1 << 2,
        GCTypeIncrementalMarking = 1 << 3,
        GCTypeProcessWeakCallbacks = 1 << 4,
        GCTypeAll = GCType::GCTypeScavenge as isize
            | GCType::GCTypeMinorMarkSweep as isize
            | GCType::GCTypeMarkSweepCompact as isize
            | GCType::GCTypeIncrementalMarking as isize
            | GCType::GCTypeProcessWeakCallbacks as isize,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GCCallbackFlags {
        NoGCCallbackFlags = 0,
        GCCallbackFlagConstructRetainedObjectInfos = 1 << 1,
        GCCallbackFlagForced = 1 << 2,
        GCCallbackFlagSynchronousPhantomCallbackProcessing = 1 << 3,
        GCCallbackFlagCollectAllAvailableGarbage = 1 << 4,
        GCCallbackFlagCollectAllExternalMemory = 1 << 5,
        GCCallbackScheduleIdleGarbageCollection = 1 << 6,
    }

    pub type GCCallback = Option<unsafe extern "C" fn(gc_type: GCType, flags: GCCallbackFlags)>;

    pub type InterruptCallback = Option<unsafe extern "C" fn(isolate: *mut Isolate, data: *mut c_void)>;

    pub type PrintCurrentStackTraceFilterCallback =
        Option<unsafe extern "C" fn(isolate: *mut Isolate, script_name: Local<String>) -> bool>;

    pub type NearHeapLimitCallback = Option<
        unsafe extern "C" fn(data: *mut c_void, current_heap_limit: c_size_t, initial_heap_limit: c_size_t) -> c_size_t,
    >;

    // --- Counters Callbacks ---

    pub type CounterLookupCallback = Option<unsafe extern "C" fn(name: *const c_char) -> *mut i32>;

    pub type CreateHistogramCallback = Option<
        unsafe extern "C" fn(name: *const c_char, min: i32, max: i32, buckets: c_size_t) -> *mut c_void,
    >;

    pub type AddHistogramSampleCallback = Option<unsafe extern "C" fn(histogram: *mut c_void, sample: i32)>;

    // --- Exceptions ---

    pub type FatalErrorCallback = Option<unsafe extern "C" fn(location: *const c_char, message: *const c_char)>;

    #[repr(C)]
    #[derive(Debug)]
    pub struct OOMDetails {
        pub is_heap_oom: bool,
        pub detail: *const c_char,
    }

    pub type OOMErrorCallback =
        Option<unsafe extern "C" fn(location: *const c_char, details: &OOMDetails)>;

    pub type MessageCallback =
        Option<unsafe extern "C" fn(message: Local<Message>, data: Local<super::callbacks::Value>)>;

    // --- Tracing ---

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LogEventStatus {
        Start = 0,
        End = 1,
        Log = 2,
    }
    pub type LogEventCallback = Option<unsafe extern "C" fn(name: *const c_char, status: i32)>;

    // --- Crashkeys Callback ---
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CrashKeyId {
        IsolateAddress,
        ReadonlySpaceFirstPageAddress,
        OldSpaceFirstPageAddress,
        CodeRangeBaseAddress,
        CodeSpaceFirstPageAddress,
        DumpType,
        SnapshotChecksumCalculated,
        SnapshotChecksumExpected,
    }

    pub type AddCrashKeyCallback = Option<unsafe extern "C" fn(id: CrashKeyId, value: String)>;

    // --- Enter/Leave Script Callback ---
    pub type BeforeCallEnteredCallback = Option<unsafe extern "C" fn(isolate: *mut Isolate)>;
    pub type CallCompletedCallback = Option<unsafe extern "C" fn(isolate: *mut Isolate)>;

    // --- Modify Code Generation From Strings Callback ---
    #[repr(C)]
    #[derive(Debug)]
    pub struct ModifyCodeGenerationFromStringsResult {
        pub codegen_allowed: bool,
        pub modified_source: MaybeLocal<String>,
    }

    pub type ModifyCodeGenerationFromStringsCallback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            source: Local<super::callbacks::Value>,
        ) -> ModifyCodeGenerationFromStringsResult,
    >;

    pub type ModifyCodeGenerationFromStringsCallback2 = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            source: Local<super::callbacks::Value>,
            is_code_like: bool,
        ) -> ModifyCodeGenerationFromStringsResult,
    >;

    // --- Failed Access Check Callback ---
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AccessType {
        AccessGet,
        AccessSet,
        AccessHas,
        AccessDelete,
        AccessKeys,
    }

    pub type FailedAccessCheckCallback = Option<
        unsafe extern "C" fn(
            target: Local<super::callbacks::Object>,
            access_type: AccessType,
            data: Local<super::callbacks::Value>,
        ),
    >;

    // --- WebAssembly compilation callbacks ---
    pub type ExtensionCallback = Option<unsafe extern "C" fn(arg1: *const c_void) -> bool>; // FunctionCallbackInfo<Value>

    pub type AllowWasmCodeGenerationCallback = Option<
        unsafe extern "C" fn(context: Local<Context>, source: Local<String>) -> bool,
    >;

    // --- Callback for APIs defined on v8-supported objects, but implemented
    // by the embedder. Example: WebAssembly.{compile|instantiate}Streaming ---
    pub type ApiImplementationCallback = Option<unsafe extern "C" fn(arg1: *const c_void)>; // FunctionCallbackInfo<Value>

    // --- Callback for WebAssembly.compileStreaming ---
    pub type WasmStreamingCallback = Option<unsafe extern "C" fn(arg1: *const c_void)>; // FunctionCallbackInfo<Value>

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WasmAsyncSuccess {
        Success,
        Fail,
    }

    // --- Callback called when async WebAssembly operations finish ---
    pub type WasmAsyncResolvePromiseCallback = Option<
        unsafe extern "C" fn(
            isolate: *mut Isolate,
            context: Local<Context>,
            resolver: Local<PromiseResolver>,
            result: Local<super::callbacks::Value>,
            success: WasmAsyncSuccess,
        ),
    >;

    // --- Callback for loading source map file for Wasm profiling support
    pub type WasmLoadSourceMapCallback =
        Option<unsafe extern "C" fn(isolate: *mut Isolate, name: *const c_char) -> Local<String>>;

    // --- Callback for checking if WebAssembly imported strings are enabled ---
    pub type WasmImportedStringsEnabledCallback =
        Option<unsafe extern "C" fn(context: Local<Context>) -> bool>;

    // --- Callback for checking if the SharedArrayBuffer constructor is enabled ---
    pub type SharedArrayBufferConstructorEnabledCallback =
        Option<unsafe extern "C" fn(context: Local<Context>) -> bool>;

    // --- Callback for checking if the compile hints magic comments are enabled ---
    pub type JavaScriptCompileHintsMagicEnabledCallback =
        Option<unsafe extern "C" fn(context: Local<Context>) -> bool>;

    // --- Callback for checking if WebAssembly JSPI is enabled ---
    pub type WasmJSPIEnabledCallback = Option<unsafe extern "C" fn(context: Local<Context>) -> bool>;

    /**
     * Import phases in import requests.
     */
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ModuleImportPhase {
        kSource,
        kEvaluation,
    }

    pub type HostImportModuleDynamicallyCallback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            host_defined_options: Local<Data>,
            resource_name: Local<super::callbacks::Value>,
            specifier: Local<String>,
            import_attributes: Local<FixedArray>,
        ) -> MaybeLocal<Promise>,
    >;

    pub type HostImportModuleWithPhaseDynamicallyCallback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            host_defined_options: Local<Data>,
            resource_name: Local<super::callbacks::Value>,
            specifier: Local<String>,
            phase: ModuleImportPhase,
            import_attributes: Local<FixedArray>,
        ) -> MaybeLocal<Promise>,
    >;

    pub type CompileHintCallback = Option<unsafe extern "C" fn(arg1: i32, arg2: *mut c_void) -> bool>;

    pub type HostInitializeImportMetaObjectCallback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            module: Local<Module>,
            meta: Local<super::callbacks::Object>,
        ),
    >;

    pub type HostCreateShadowRealmContextCallback =
        Option<unsafe extern "C" fn(initiator_context: Local<Context>) -> MaybeLocal<Context>>;

    pub type IsJSApiWrapperNativeErrorCallback = Option<
        unsafe extern "C" fn(isolate: *mut Isolate, obj: Local<super::callbacks::Object>) -> bool,
    >;

    pub type PrepareStackTraceCallback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            error: Local<super::callbacks::Value>,
            sites: Local<Array>,
        ) -> MaybeLocal<super::callbacks::Value>,
    >;

    #[cfg(target_os = "windows")]
    pub type FilterETWSessionByURLCallback =
        Option<unsafe extern "C" fn(context: Local<Context>, etw_filter_payload: String) -> bool>;

    #[cfg(target_os = "windows")]
    #[repr(C)]
    #[derive(Debug)]
    pub struct FilterETWSessionByURLResult {
        pub enable_etw_tracing: bool,
        pub trace_interpreter_frames: bool,
    }

    #[cfg(target_os = "windows")]
    pub type FilterETWSessionByURL2Callback = Option<
        unsafe extern "C" fn(
            context: Local<Context>,
            etw_filter_payload: String,
        ) -> FilterETWSessionByURLResult,
    >;
}
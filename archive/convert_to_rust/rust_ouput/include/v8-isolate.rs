// Converted from V8 C++ source files:
// Header: v8-isolate.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(unused_variables)]
use std::sync::{Mutex, RwLock};
use std::{
    ffi::c_char,
    mem::size_of,
    ptr::{null, null_mut, NonNull},
    sync::Arc,
};

pub struct CppHeap;
pub struct HeapProfiler;
pub struct MicrotaskQueue;
pub struct StartupData;
pub struct ScriptOrModule;
pub struct SharedArrayBuffer;

pub mod internal {
    pub struct MicrotaskQueue;
    pub struct ThreadLocalTop;
    pub type Address = usize;
}

pub mod metrics {
    pub struct Recorder;
}

#[repr(C)]
pub struct ResourceConstraints {
    code_range_size_: usize,
    max_old_generation_size_: usize,
    max_young_generation_size_: usize,
    initial_old_generation_size_: usize,
    initial_young_generation_size_: usize,
    stack_limit_: *mut u32,
}

impl ResourceConstraints {
    pub fn configure_defaults_from_heap_size(
        &mut self,
        initial_heap_size_in_bytes: usize,
        maximum_heap_size_in_bytes: usize,
    ) {
        self.code_range_size_ = maximum_heap_size_in_bytes * 2;
        self.max_old_generation_size_ = maximum_heap_size_in_bytes / 2;
        self.max_young_generation_size_ = maximum_heap_size_in_bytes / 4;
        self.initial_old_generation_size_ = initial_heap_size_in_bytes / 2;
        self.initial_young_generation_size_ = initial_heap_size_in_bytes / 4;
    }

    pub fn configure_defaults(&mut self, physical_memory: u64, virtual_memory_limit: u64) {
        self.code_range_size_ = (physical_memory as f64 * 0.25) as usize;
        self.max_old_generation_size_ = (physical_memory as f64 * 0.20) as usize;
        self.max_young_generation_size_ = (physical_memory as f64 * 0.05) as usize;
        self.initial_old_generation_size_ = (physical_memory as f64 * 0.10) as usize;
        self.initial_young_generation_size_ = (physical_memory as f64 * 0.02) as usize;
    }

    pub fn stack_limit(&self) -> *mut u32 {
        self.stack_limit_
    }

    pub fn set_stack_limit(&mut self, value: *mut u32) {
        self.stack_limit_ = value;
    }

    pub fn code_range_size_in_bytes(&self) -> usize {
        self.code_range_size_
    }

    pub fn set_code_range_size_in_bytes(&mut self, limit: usize) {
        self.code_range_size_ = limit;
    }

    pub fn max_old_generation_size_in_bytes(&self) -> usize {
        self.max_old_generation_size_
    }

    pub fn set_max_old_generation_size_in_bytes(&mut self, limit: usize) {
        self.max_old_generation_size_ = limit;
    }

    pub fn max_young_generation_size_in_bytes(&self) -> usize {
        self.max_young_generation_size_
    }

    pub fn set_max_young_generation_size_in_bytes(&mut self, limit: usize) {
        self.max_young_generation_size_ = limit;
    }

    pub fn initial_old_generation_size_in_bytes(&self) -> usize {
        self.initial_old_generation_size_
    }

    pub fn set_initial_old_generation_size_in_bytes(&mut self, initial_size: usize) {
        self.initial_old_generation_size_ = initial_size;
    }

    pub fn initial_young_generation_size_in_bytes(&self) -> usize {
        self.initial_young_generation_size_
    }

    pub fn set_initial_young_generation_size_in_bytes(&mut self, initial_size: usize) {
        self.initial_young_generation_size_ = initial_size;
    }
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        ResourceConstraints {
            code_range_size_: 0,
            max_old_generation_size_: 0,
            max_young_generation_size_: 0,
            initial_old_generation_size_: 0,
            initial_young_generation_size_: 0,
            stack_limit_: null_mut(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryPressureLevel {
    kNone,
    kModerate,
    kCritical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextDependants {
    kNoDependants,
    kSomeDependants,
}

pub type StackState = cppgc::EmbedderStackState;

pub struct IsolateGroup {
    isolate_group_: *mut internal::IsolateGroup,
}

impl IsolateGroup {
    pub fn get_default() -> Self {
        IsolateGroup {
            isolate_group_: null_mut(),
        }
    }

    pub fn can_create_new_groups() -> bool {
        true
    }

    pub fn create() -> Self {
        IsolateGroup {
            isolate_group_: null_mut(),
        }
    }

    pub fn new(isolate_group: *mut internal::IsolateGroup) -> Self {
        IsolateGroup {
            isolate_group_: isolate_group,
        }
    }
}

impl Drop for IsolateGroup {
    fn drop(&mut self) {}
}

impl IsolateGroup {
    fn new_move(isolate_group: *mut internal::IsolateGroup) -> Self {
        IsolateGroup {
            isolate_group_: isolate_group,
        }
    }
}

impl IsolateGroup {
    pub fn operator_eq(&self, other: &IsolateGroup) -> bool {
        self.isolate_group_ == other.isolate_group_
    }

    pub fn operator_ne(&self, other: &IsolateGroup) -> bool {
        !self.operator_eq(other)
    }
}

pub type JitCodeEventHandler =
    extern "C" fn(options: JitCodeEventOptions, event: &JitCodeEvent);
pub type CounterLookupCallback = extern "C" fn(name: *const c_char) -> *mut i32;
pub type CreateHistogramCallback = extern "C" fn(name: *const c_char) -> *mut Histogram;
pub type AddHistogramSampleCallback = extern "C" fn(histogram: *mut Histogram, sample: i32);
pub type FatalErrorCallback = extern "C" fn(location: *const c_char, message: *const c_char);
pub type OOMErrorCallback = extern "C" fn(location: *const c_char, dead: bool);
pub type HostImportModuleDynamicallyCallback =
    extern "C" fn(context: Local<'static, Context>, referrer: Local<'static, ScriptOrModule>, specifier: Local<'static, String>) -> Local<'static, Promise>;
pub type HostImportModuleWithPhaseDynamicallyCallback =
    extern "C" fn(context: Local<'static, Context>, referrer: Local<'static, ScriptOrModule>, specifier: Local<'static, String>, import_attributes: Local<'static, FixedArray>) -> Local<'static, Promise>;
pub type HostInitializeImportMetaObjectCallback =
    extern "C" fn(context: Local<'static, Context>, meta: Local<'static, Object>, module: Local<'static, Module>);
pub type HostCreateShadowRealmContextCallback =
    extern "C" fn(creator_context: Local<'static, Context>, options: Local<'static, Object>) -> Local<'static, Context>;
pub type IsJSApiWrapperNativeErrorCallback =
    extern "C" fn(value: Local<'static, Value>) -> bool;
pub type PrepareStackTraceCallback =
    extern "C" fn(context: Local<'static, Context>, error: Local<'static, Value>, frames: Local<'static, Array>) -> Local<'static, Value>;
pub type FilterETWSessionByURLCallback = extern "C" fn(url: *const c_char) -> bool;
pub type FilterETWSessionByURL2Callback = extern "C" fn(url: *const c_char) -> bool;
pub type InterruptCallback = extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void);
pub type LogEventCallback = extern "C" fn(isolate: *mut Isolate, event: &LogEventData);
pub type BeforeCallEnteredCallback = extern "C" fn(isolate: *mut Isolate);
pub type CallCompletedCallback = extern "C" fn(isolate: *mut Isolate);
pub type PromiseHook = extern "C" fn(isolate: *mut Isolate, type_: PromiseHookType, promise: Local<'static, Promise>, parent: Local<'static, Value>);
pub type PromiseRejectCallback = extern "C" fn(isolate: *mut Isolate, message: Local<'static, Value>, promise: Local<'static, Promise>, reason: Local<'static, Value>);
pub type ExceptionPropagationCallback = extern "C" fn(isolate: *mut Isolate, exception: Local<'static, Value>, data: *mut std::ffi::c_void);
pub type MicrotaskCallback = extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void);
pub type MicrotasksCompletedCallbackWithData = extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void);
pub type UseCounterCallback = extern "C" fn(isolate: *mut Isolate, feature: UseCounterFeature);
pub type AddCrashKeyCallback = extern "C" fn(name: *const c_char, value: *const c_char);
pub type NearHeapLimitCallback = extern "C" fn(isolate: *mut Isolate, initial_heap_limit: usize, current_heap_limit: usize, data: *mut std::ffi::c_void) -> usize;
pub type ModifyCodeGenerationFromStringsCallback2 = extern "C" fn(context: Local<'static, Context>, origin: Local<'static, String>, allow: *mut bool);
pub type AllowWasmCodeGenerationCallback = extern "C" fn(context: Local<'static, Context>) -> bool;
pub type ExtensionCallback = extern "C" fn(isolate: *mut Isolate, extension: Local<'static, ObjectTemplate>);
pub type WasmStreamingCallback = extern "C" fn(context: Local<'static, Context>, url: Local<'static, String>, stream: Local<'static, Value>) -> Local<'static, Promise>;
pub type WasmAsyncResolvePromiseCallback = extern "C" fn(context: Local<'static, Context>, id: u32, promise: Local<'static, Promise>);
pub type WasmLoadSourceMapCallback = extern "C" fn(context: Local<'static, Context>, url: Local<'static, String>, source_map_url: *mut *const c_char);
pub type WasmImportedStringsEnabledCallback = extern "C" fn(isolate: *mut Isolate) -> bool;
pub type SharedArrayBufferConstructorEnabledCallback = extern "C" fn(context: Local<'static, Context>) -> bool;
pub type WasmJSPIEnabledCallback = extern "C" fn(isolate: *mut Isolate) -> bool;
pub type JavaScriptCompileHintsMagicEnabledCallback = extern "C" fn(isolate: *mut Isolate) -> bool;
pub type MessageCallback = extern "C" fn(message: Local<'static, Message>, exception: Local<'static, Value>);
pub type FailedAccessCheckCallback = extern "C" fn(context: Local<'static, Context>, object: Local<'static, Object>, property: Local<'static, Value>, is_native: bool) -> bool;

#[repr(C)]
pub struct HeapStatistics {
    total_heap_size: usize,
    total_heap_size_executable: usize,
    total_physical_memory: usize,
    total_available_size: usize,
    used_heap_size: usize,
    heap_size_limit: usize,
    malloced_memory: usize,
    peak_malloced_memory: usize,
    does_zap_garbage: bool,
    number_of_native_contexts: usize,
    number_of_detached_contexts: usize,
    external_memory: usize,
}

#[repr(C)]
pub struct HeapSpaceStatistics {
    space_name: *const c_char,
    space_size: usize,
    space_used_size: usize,
    space_available_size: usize,
    physical_space_size: usize,
}

#[repr(C)]
pub struct HeapObjectStatistics {
    object_type: *const c_char,
    object_count: usize,
    object_size: usize,
}

#[repr(C)]
pub struct HeapCodeStatistics {
    code_size: usize,
    bytecode_size: usize,
    metadata_size: usize,
}

#[repr(C)]
pub struct RegisterState {
    // Example implementation.  Actual fields are platform-dependent.
    rax: u64,
    rsp: u64,
    rip: u64,
}

#[repr(C)]
pub struct SampleInfo {
    sample_address: usize,
    vm_state: VMState,
    frames_available: usize,
    frames_captured: usize,
}

#[repr(C)]
pub struct MemoryRange {
    address: *mut std::ffi::c_void,
    length: usize,
}

#[repr(C)]
pub struct JitCodeEvent {
    event_type: JitCodeEvent::EventType,
    code_start: *mut std::ffi::c_void,
    code_size: usize,
    name: *const c_char,
    script_name: *const c_char,
    script_line: i32,
    column_start: i32,
    column_end: i32,
    function: Local<'static, Function>,
    source_text: *const c_char,
    function_token_offset: i32,
    function_end_token_offset: i32,
}

impl JitCodeEvent {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(C)]
    pub enum EventType {
        kCodeAdded,
        kCodeMoved,
        kCodeDeleted,
    }
}

#[repr(C)]
pub struct JitCodeEventOptions {
    enabled_options: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VMState {
    kInvalid,
    kGC,
    kCompiling,
    kExecuting,
    kExternal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum PromiseHookType {
    kInit,
    kResolve,
    kReject,
    kBefore,
    kAfter,
}

#[repr(C)]
pub struct LogEventData {
    // Example fields. Actual fields are platform-dependent.
    timestamp: u64,
    event: u32,
    data: *const std::ffi::c_void,
}

#[repr(C)]
pub struct Histogram {
    min: i32,
    max: i32,
    buckets: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCType {
    kGCTypeAll,
    kGCTypeScavenge,
    kGCTypeMarkSweepCompact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCCallbackFlags {
    kGCCallbackFlagNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrotasksPolicy {
    kExplicit,
    kAutomatic,
    kScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseCounterFeature {
    kUseAsm = 0,
    kBreakIterator = 1,
    kOBSOLETE_LegacyConst = 2,
    kOBSOLETE_MarkDequeOverflow = 3,
    kOBSOLETE_StoreBufferOverflow = 4,
    kOBSOLETE_SlotsBufferOverflow = 5,
    kOBSOLETE_ObjectObserve = 6,
    kForcedGC = 7,
    kSloppyMode = 8,
    kStrictMode = 9,
    kOBSOLETE_StrongMode = 10,
    kRegExpPrototypeStickyGetter = 11,
    kRegExpPrototypeToString = 12,
    kRegExpPrototypeUnicodeGetter = 13,
    kOBSOLETE_IntlV8Parse = 14,
    kOBSOLETE_IntlPattern = 15,
    kOBSOLETE_IntlResolved = 16,
    kOBSOLETE_PromiseChain = 17,
    kOBSOLETE_PromiseAccept = 18,
    kOBSOLETE_PromiseDefer = 19,
    kHtmlCommentInExternalScript = 20,
    kHtmlComment = 21,
    kSloppyModeBlockScopedFunctionRedefinition = 22,
    kForInInitializer = 23,
    kOBSOLETE_ArrayProtectorDirtied = 24,
    kArraySpeciesModified = 25,
    kArrayPrototypeConstructorModified = 26,
    kOBSOLETE_ArrayInstanceProtoModified = 27,
    kArrayInstanceConstructorModified = 28,
    kOBSOLETE_LegacyFunctionDeclaration = 29,
    kOBSOLETE_RegExpPrototypeSourceGetter = 30,
    kOBSOLETE_RegExpPrototypeOldFlagGetter = 31,
    kDecimalWithLeadingZeroInStrictMode = 32,
    kLegacyDateParser = 33,
    kDefineGetterOrSetterWouldThrow = 34,
    kFunctionConstructorReturnedUndefined = 35,
    kAssigmentExpressionLHSIsCallInSloppy = 36,
    kAssigmentExpressionLHSIsCallInStrict = 37,
    kPromiseConstructorReturnedUndefined = 38,
    kOBSOLETE_ConstructorNonUndefinedPrimitiveReturn = 39,
    kOBSOLETE_LabeledExpressionStatement = 40,
    kOBSOLETE_LineOrParagraphSeparatorAsLineTerminator = 41,
    kIndexAccessor = 42,
    kErrorCaptureStackTrace = 43,
    kErrorPrepareStackTrace = 44,
    kErrorStackTraceLimit = 45,
    kWebAssemblyInstantiation = 46,
    kDeoptimizerDisableSpeculation = 47,
    kOBSOLETE_ArrayPrototypeSortJSArrayModifiedPrototype = 48,
    kFunctionTokenOffsetTooLongForToString = 49,
    kWasmSharedMemory = 50,
    kWasmThreadOpcodes = 51,
    kOBSOLETE_AtomicsNotify = 52,
    kOBSOLETE_AtomicsWake = 53,
    kCollator = 54,
    kNumberFormat = 55,
    kDateTimeFormat = 56,
    kPluralRules = 57,
    kRelativeTimeFormat = 58,
    kLocale = 59,
    kListFormat = 60,
    kSegmenter = 61,
    kStringLocaleCompare = 62,
    kOBSOLETE_StringToLocaleUpperCase = 63,
    kStringToLocaleLowerCase = 64,
    kNumberToLocaleString = 65,
    kDateToLocaleString = 66,
    kDateToLocaleDateString = 67,
    kDateToLocaleTimeString = 68,
    kAttemptOverrideReadOnlyOnPrototypeSloppy = 69,
    kAttemptOverrideReadOnlyOnPrototypeStrict = 70,
    kOBSOLETE_OptimizedFunctionWithOneShotBytecode = 71,
    kRegExpMatchIsTrueishOnNonJSRegExp = 72,
    kRegExpMatchIsFalseishOnJSRegExp = 73,
    kOBSOLETE_DateGetTimezoneOffset = 74,
    kStringNormalize = 75,
    kCallSiteAPIGetFunctionSloppyCall = 76,
    kCallSiteAPIGetThisSloppyCall = 77,
    kOBSOLETE_RegExpMatchAllWithNonGlobalRegExp = 78,
    kRegExpExecCalledOnSlowRegExp = 79,
    kRegExpReplaceCalledOnSlowRegExp = 80,
    kDisplayNames = 81,
    kSharedArrayBufferConstructed = 82,
    kArrayPrototypeHasElements = 83,
    kObjectPrototypeHasElements = 84,
    kNumberFormatStyleUnit = 85,
    kDateTimeFormatRange = 86,
    kDateTimeFormatDateTimeStyle = 87,
    kBreakIteratorTypeWord = 88,
    kBreakIteratorTypeLine = 89,
    kInvalidatedArrayBufferDetachingProtector = 90,
    kInvalidatedArrayConstructorProtector = 91,
    kInvalidatedArrayIteratorLookupChainProtector = 92,
    kInvalidatedArraySpeciesLookupChainProtector = 93,
    kInvalidatedIsConcatSpreadableLookupChainProtector = 94,
    kInvalidatedMapIteratorLookupChainProtector = 95,
    kInvalidatedNoElementsProtector = 96,
    kInvalidatedPromiseHookProtector = 97,
    kInvalidatedPromiseResolveLookupChainProtector = 98,
    kInvalidatedPromiseSpeciesLookupChainProtector = 99,
    kInvalidatedPromiseThenLookupChainProtector = 100,
    kInvalidatedRegExpSpeciesLookupChainProtector = 101,
    kInvalidatedSetIteratorLookupChainProtector = 102,
    kInvalidatedStringIteratorLookupChainProtector = 103,
    kInvalidatedStringLengthOverflowLookupChainProtector = 104,
    kInvalidatedTypedArraySpeciesLookupChainProtector = 105,
    kWasmSimdOpcodes = 106,
    kVarRedeclaredCatchBinding = 107,
    kWasmRefTypes = 108,
    kOBSOLETE_WasmBulkMemory = 109,
    kOBSOLETE_WasmMultiValue = 110,
    kWasmExceptionHandling = 111,
    kInvalidatedMegaDOMProtector = 112,
    kFunctionPrototypeArguments = 113,
    kFunctionPrototypeCaller = 114,
    kTurboFanOsrCompileStarted = 115,
    kAsyncStackTaggingCreateTaskCall = 116,
    kDurationFormat = 117,
    kInvalidatedNumberStringNotRegexpLikeProtector = 118,
    kOBSOLETE_RegExpUnicodeSetIncompatibilitiesWithUnicodeMode = 119,
    kOBSOLETE_ImportAssertionDeprecatedSyntax = 120,
    kLocaleInfoObsoletedGetters = 121,
    kLocaleInfoFunctions = 122,
    kCompileHintsMagicAll = 123,
    kInvalidatedNoProfilingProtector = 124,
    kWasmMemory64 = 125,
    kWasmMultiMemory = 126,
    kWasmGC = 127,
    kWasmImportedStrings = 128,
    kSourceMappingUrlMagicCommentAtSign = 129,
    kTemporalObject = 130,
    kWasmModuleCompilation = 131,
    kInvalidatedNoUndetectableObjectsProtector = 132,
    kWasmJavaScriptPromiseIntegration = 133,
    kWasmReturnCall = 134,
    kWasmExtendedConst = 135,
    kWasmRelaxedSimd = 136,
    kWasmTypeReflection = 137,
    kWasmExnRef = 138,
    kWasmTypedFuncRef = 139,
    kInvalidatedStringWrapperToPrimitiveProtector = 140,
    kDocumentAllLegacyCall = 141,
    kDocumentAllLegacyConstruct = 142,
    kConsoleContext = 143,
    kWasmImportedStringsUtf8 = 144,
    kResizableArrayBuffer = 145,
    kGrowableSharedArrayBuffer = 146,
    kArrayByCopy = 147,
    kArrayFromAsync = 148,
    kIteratorMethods = 149,
    kPromiseAny = 150,
    kSetMethods = 151,
    kArrayFindLast = 152,
    kArrayGroup = 153,
    kArrayBufferTransfer = 154,
    kPromiseWithResolvers = 155,
    kAtomicsWaitAsync = 156,
    kExtendingNonExtensibleWithPrivate = 157,
    kPromiseTry = 158,
    kStringReplaceAll = 159,
    kStringWellFormed = 160,
    kWeakReferences = 161,
    kErrorIsError = 162,
    kInvalidatedTypedArrayLengthLookupChainProtector = 163,
    kRegExpEscape = 164,
    kFloat16Array = 165,
    kExplicitResourceManagement = 166,
    kWasmBranchHinting = 167,
    kUseCounterFeatureCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageErrorLevel {
    kMessageLog = (1 << 0),
    kMessageDebug = (1 << 1),
    kMessageInfo = (1 << 2),
    kMessageError = (1 << 3),
    kMessageWarning = (1 << 4),
    kMessageAll = kMessageLog | kMessageDebug | kMessageInfo | kMessageError | kMessageWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    kBestEffort,
    kUserVisible,
    kUserBlocking,
}

pub struct CreateParams {
    pub code_event_handler: JitCodeEventHandler,
    pub constraints: ResourceConstraints,
    pub snapshot_blob: *const StartupData,
    pub counter_lookup_callback: CounterLookupCallback,
    pub create_histogram_callback: CreateHistogramCallback,
    pub add_histogram_sample_callback: AddHistogramSampleCallback,
    pub array_buffer_allocator: *mut ArrayBuffer::Allocator,
    pub array_buffer_allocator_shared: Option<Arc<ArrayBuffer::Allocator>>,
    pub external_references: *const i64,
    pub allow_atomics_wait: bool,
    pub embedder_wrapper_type_index: i32,
    pub embedder_wrapper_object_index: i32,
    pub fatal_error_callback: FatalErrorCallback,
    pub oom_error_callback: OOMErrorCallback,
    pub cpp_heap: *mut CppHeap,
}

impl CreateParams {
    pub fn new() -> Self {
        CreateParams {
            code_event_handler: std::mem::transmute(0),
            constraints: ResourceConstraints::default(),
            snapshot_blob: null(),
            counter_lookup_callback: std::mem::transmute(0),
            create_histogram_callback: std::mem::transmute(0),
            add_histogram_sample_callback: std::mem::transmute(0),
            array_buffer_allocator: null_mut(),
            array_buffer_allocator_shared: None,
            external_references: null(),
            allow_atomics_wait: true,
            embedder_wrapper_type_index: -1,
            embedder_wrapper_object_index: -1,
            fatal_error_callback: std::mem::transmute(0),
            oom_error_callback: std::mem::transmute(0),
            cpp_heap: null_mut(),
        }
    }
}

impl Default for CreateParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CreateParams {
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct Isolate {
    data: Mutex<Vec<*mut std::ffi::c_void>>,
    aborted: Mutex<bool>,
    microtasks_policy: Mutex<MicrotasksPolicy>,
}

impl Isolate {
    pub fn allocate() -> *mut Self {
        let isolate = Box::new(Isolate {
            data: Mutex::new(vec![null_mut(); 64]),
            aborted: Mutex::new(false),
            microtasks_policy: Mutex::new(MicrotasksPolicy::kExplicit),
        });
        Box::into_raw(isolate)
    }

    pub fn allocate_with_group(group: &IsolateGroup) -> *mut Self {
        let isolate = Box::new(Isolate {
            data: Mutex::new(vec![null_mut(); 64]),
            aborted: Mutex::new(false),
            microtasks_policy: Mutex::new(MicrotasksPolicy::kExplicit),
        });
        Box::into_raw(isolate)
    }

    pub fn get_group(&self) -> IsolateGroup {
        IsolateGroup {
            isolate_group_: null_mut(),
        }
    }

    pub fn initialize(isolate_ptr: *mut Isolate, params: &CreateParams) {
        unsafe {
            let isolate = &mut *isolate_ptr;
            println!("Initializing isolate");
        }
    }

    pub fn new(params: &CreateParams) -> *mut Self {
        let isolate = Self::allocate();
        Self::initialize(isolate, params);
        isolate
    }

    pub fn new_with_group(group: &IsolateGroup, params: &CreateParams) -> *mut Self {
        let isolate = Self::allocate_with_group(group);
        Self::initialize(isolate, params);
        isolate
    }

    pub fn get_current() -> *mut Self {
        thread_local! {
            static CURRENT_ISOLATE: std::cell::RefCell<*mut Isolate> = std::cell::RefCell::new(null_mut());
        }
        CURRENT_ISOLATE.with(|isolate| *isolate.borrow())
    }

    pub fn try_get_current() -> *mut Self {
        thread_local! {
            static CURRENT_ISOLATE: std::cell::RefCell<*mut Isolate> = std::cell::RefCell::new(null_mut());
        }
        CURRENT_ISOLATE.with(|isolate| *isolate.borrow())
    }

    pub fn is_current(&self) -> bool {
        Self::get_current() as *const Self == self as *const Self
    }

    pub fn clear_kept_objects(&self) {
        println!("clear_kept_objects");
    }

    pub fn set_abort_on_uncaught_exception_callback(
        &self,
        callback: Option<extern "C" fn(*mut Isolate) -> bool>,
    ) {
        println!("set_abort_on_uncaught_exception_callback");
    }

    pub fn set_host_import_module_dynamically_callback(
        &self,
        callback: HostImportModuleDynamicallyCallback,
    ) {
        println!("set_host_import_module_dynamically_callback");
    }

    pub fn set_host_import_module_with_phase_dynamically_callback(
        &self,
        callback: HostImportModuleWithPhaseDynamicallyCallback,
    ) {
        println!("set_host_import_module_with_phase_dynamically_callback");
    }

    pub fn set_host_initialize_import_meta_object_callback(
        &self,
        callback: HostInitializeImportMetaObjectCallback,
    ) {
        println!("set_host_initialize_import_meta_object_callback");
    }

    pub fn set_host_create_shadow_realm_context_callback(
        &self,
        callback: HostCreateShadowRealmContextCallback,
    ) {
        println!("set_host_create_shadow_realm_context_callback");
    }

    pub fn set_is_js_api_wrapper_native_error_callback(
        &self,
        callback: IsJSApiWrapperNativeErrorCallback,
    ) {
        println!("set_is_js_api_wrapper_native_error_callback");
    }

    pub fn set_prepare_stack_trace_callback(&self, callback: PrepareStackTraceCallback) {
        println!("set_prepare_stack_trace_callback");
    }

    pub fn get_stack_trace_limit(&self) -> i32 {
        println!("get_stack_trace_limit");
        10
    }

    pub fn set_filter_etw_session_by_url_callback(&self, callback: FilterETWSessionByURLCallback) {
        println!("set_filter_etw_session_by_url_callback");
    }

    pub fn set_filter_etw_session_by_url2_callback(
        &self,
        callback: FilterETWSessionByURL2Callback,
    ) {
        println!("set_filter_etw_session_by_url2_callback");
    }

    pub fn memory_pressure_notification(&self, level: MemoryPressureLevel) {
        println!("memory_pressure_notification: {:?}", level);
    }

    pub fn set_battery_saver_mode(&self, battery_saver_mode_enabled: bool) {
        println!(
            "set_battery_saver_mode: battery_saver_mode_enabled={}",
            battery_saver_mode_enabled
        );
    }

    pub fn set_memory_saver_mode(&self, memory_saver_mode_enabled: bool) {
        println!(
            "set

// Converted from V8 C++ source files:
// Header: v8-script.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
use std::ffi::c_char;
use std::error::Error;
use std::fmt;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::{Arc, Mutex};
// Import necessary V8 types and functions
// use v8::...; // Import V8 types as needed
// Placeholder structures for types not yet converted

pub struct ScriptOrModule {
    resource_name: Local<'static, Value>,
    host_defined_options: Local<'static, Data>,
}

impl ScriptOrModule {
    pub fn GetResourceName(&self) -> Local<'static, Value> {
        self.resource_name
    }

    pub fn HostDefinedOptions(&self) -> Local<'static, Data> {
        self.host_defined_options
    }
}

pub struct UnboundScript {
    id: i32,
    script_name: Local<'static, Value>,
    source_url: Local<'static, Value>,
    source_mapping_url: Local<'static, Value>,
}

impl UnboundScript {
    pub fn BindToCurrentContext(&self) -> Local<'static, Script> {
        Script {} // Dummy implementation
    }

    pub fn GetId(&self) -> i32 {
        self.id
    }

    pub fn GetScriptName(&self) -> Local<'static, Value> {
        self.script_name
    }

    pub fn GetSourceURL(&self) -> Local<'static, Value> {
        self.source_url
    }

    pub fn GetSourceMappingURL(&self) -> Local<'static, Value> {
        self.source_mapping_url
    }

    pub fn GetLineNumber(&self, code_pos: i32) -> i32 {
        if code_pos == 0 { 1 } else { -1 }
    }

    pub fn GetColumnNumber(&self, code_pos: i32) -> i32 {
        if code_pos == 0 { 1 } else { -1 }
    }

    pub const kNoScriptId: i32 = 0;
}

pub struct UnboundModuleScript {
    source_url: Local<'static, Value>,
    source_mapping_url: Local<'static, Value>,
}

impl UnboundModuleScript {
    pub fn GetSourceURL(&self) -> Local<'static, Value> {
        self.source_url
    }

    pub fn GetSourceMappingURL(&self) -> Local<'static, Value> {
        self.source_mapping_url
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    line_number_: i32,
    column_number_: i32,
}

impl Location {
    pub fn GetLineNumber(&self) -> i32 {
        self.line_number_
    }
    pub fn GetColumnNumber(&self) -> i32 {
        self.column_number_
    }
    pub fn new(line_number: i32, column_number: i32) -> Self {
        Location {
            line_number_: line_number,
            column_number_: column_number,
        }
    }
}

pub struct ModuleRequest {
    specifier: Local<'static, String>,
    phase: ModuleImportPhase,
    source_offset: i32,
    import_attributes: Local<'static, FixedArray>,
}

impl ModuleRequest {
    pub fn GetSpecifier(&self) -> Local<'static, String> {
        self.specifier
    }

    pub fn GetPhase(&self) -> ModuleImportPhase {
        self.phase
    }

    pub fn GetSourceOffset(&self) -> i32 {
        self.source_offset
    }

    pub fn GetImportAttributes(&self) -> Local<'static, FixedArray> {
        self.import_attributes
    }

    pub fn GetImportAssertions(&self) -> Local<'static, FixedArray> {
        self.GetImportAttributes()
    }

    pub fn Cast(data: *mut Data) -> *mut ModuleRequest {
        data as *mut ModuleRequest
    }

    fn CheckCast(obj: *mut Data) {}
}

pub struct Module {
    status: Status,
    exception: Local<'static, Value>,
    module_requests: Local<'static, FixedArray>,
    script_id: i32,
    is_graph_async: bool,
    has_top_level_await: bool,
    is_source_text_module: bool,
    is_synthetic_module: bool,
    module_name: Local<'static, String>,
}

impl Module {
    pub enum Status {
        kUninstantiated,
        kInstantiating,
        kInstantiated,
        kEvaluating,
        kEvaluated,
        kErrored,
    }

    pub fn GetStatus(&self) -> Status {
        self.status
    }

    pub fn GetException(&self) -> Local<'static, Value> {
        self.exception
    }

    pub fn GetModuleRequests(&self) -> Local<'static, FixedArray> {
        self.module_requests
    }

    pub fn SourceOffsetToLocation(&self, offset: i32) -> Location {
        Location::new(1, 1)
    }

    pub fn GetIdentityHash(&self) -> i32 {
        42
    }

    pub fn InstantiateModule(
        &self,
        context: Local<'static, Context>,
        module_callback: fn(
            Local<'static, Context>,
            Local<'static, String>,
            Local<'static, FixedArray>,
            Local<'static, Module>,
        ) -> MaybeLocal<'static, Module>,
        source_callback: Option<
            fn(
                Local<'static, Context>,
                Local<'static, String>,
                Local<'static, FixedArray>,
                Local<'static, Module>,
            ) -> MaybeLocal<'static, Object>,
        >,
    ) -> Maybe<bool> {
        Maybe::new(true)
    }

    pub fn Evaluate(&self, context: Local<'static, Context>) -> MaybeLocal<'static, Value> {
        MaybeLocal::empty()
    }

    pub fn GetModuleNamespace(&self) -> Local<'static, Value> {
        Value {} // Dummy
    }

    pub fn GetUnboundModuleScript(&self) -> Local<'static, UnboundModuleScript> {
        UnboundModuleScript {
            source_url: Value {},
            source_mapping_url: Value {},
        } // Dummy
    }

    pub fn ScriptId(&self) -> i32 {
        self.script_id
    }

    pub fn IsGraphAsync(&self) -> bool {
        self.is_graph_async
    }

    pub fn HasTopLevelAwait(&self) -> bool {
        self.has_top_level_await
    }

    pub fn IsSourceTextModule(&self) -> bool {
        self.is_source_text_module
    }

    pub fn IsSyntheticModule(&self) -> bool {
        self.is_synthetic_module
    }

    pub fn CreateSyntheticModule(
        isolate: *mut Isolate,
        module_name: Local<'static, String>,
        export_names: MemorySpan<'static, Local<'static, String>>,
        evaluation_steps: fn(Local<'static, Context>, Local<'static, Module>) -> MaybeLocal<'static, Value>,
    ) -> Local<'static, Module> {
        Module {
            status: Status::kUninstantiated,
            exception: Value {},
            module_requests: FixedArray {},
            script_id: 0,
            is_graph_async: false,
            has_top_level_await: false,
            is_source_text_module: false,
            is_synthetic_module: false,
            module_name: module_name,
        } // Dummy
    }

    pub fn SetSyntheticModuleExport(
        &self,
        isolate: *mut Isolate,
        export_name: Local<'static, String>,
        export_value: Local<'static, Value>,
    ) -> Maybe<bool> {
        Maybe::new(true)
    }

    pub fn GetStalledTopLevelAwaitMessages(
        &self,
        isolate: *mut Isolate,
    ) -> (LocalVector<'static, Module>, LocalVector<'static, Message>) {
        (LocalVector::new(), LocalVector::new())
    }

    pub fn Cast(data: *mut Data) -> *mut Module {
        data as *mut Module
    }

    fn CheckCast(obj: *mut Data) {}
}

pub struct CompileHintsCollector {}

impl CompileHintsCollector {
    pub fn GetCompileHints(&self, isolate: *mut Isolate) -> Vec<i32> {
        vec![]
    }
}

pub struct Script {
    resource_name: Local<'static, Value>,
    compile_hints_collector: Local<'static, CompileHintsCollector>,
}

impl Script {
    pub fn Compile(context: Local<'static, Context>, source: Local<'static, String>, origin: *mut ScriptOrigin) -> MaybeLocal<'static, Script> {
        MaybeLocal::empty()
    }

    pub fn Run(&self, context: Local<'static, Context>) -> MaybeLocal<'static, Value> {
        MaybeLocal::empty()
    }

    pub fn Run_Data(&self, context: Local<'static, Context>, host_defined_options: Local<'static, Data>) -> MaybeLocal<'static, Value> {
        MaybeLocal::empty()
    }

    pub fn GetUnboundScript(&self) -> Local<'static, UnboundScript> {
        UnboundScript {
            id: 0,
            script_name: Value {},
            source_url: Value {},
            source_mapping_url: Value {},
        } // Dummy
    }

    pub fn GetResourceName(&self) -> Local<'static, Value> {
        self.resource_name
    }

    pub fn GetProducedCompileHints(&self) -> Vec<i32> {
        vec![]
    }

    pub fn GetCompileHintsCollector(&self) -> Local<'static, CompileHintsCollector> {
        self.compile_hints_collector
    }
}

pub struct ScriptCompiler {}

impl ScriptCompiler {
    pub struct CachedData {
        data: *const u8,
        length: i32,
        rejected: bool,
        buffer_policy: BufferPolicy,
    }

    impl CachedData {
        pub enum BufferPolicy {
            BufferNotOwned,
            BufferOwned,
        }

        pub fn new() -> Self {
            CachedData {
                data: ptr::null(),
                length: 0,
                rejected: false,
                buffer_policy: BufferPolicy::BufferNotOwned,
            }
        }

        pub fn new_with_data(data: *const u8, length: i32, buffer_policy: BufferPolicy) -> Self {
            CachedData {
                data: data,
                length: length,
                rejected: false,
                buffer_policy: buffer_policy,
            }
        }

        pub fn CompatibilityCheck(&self, isolate: *mut Isolate) -> CompatibilityCheckResult {
            CompatibilityCheckResult::kSuccess
        }
    }

    impl Drop for CachedData {
        fn drop(&mut self) {
            if let CachedData::BufferPolicy::BufferOwned = self.buffer_policy {
            }
        }
    }

    pub enum InMemoryCacheResult {
        kNotAttempted,
        kHit,
        kMiss,
        kPartial,
    }

    pub struct CompilationDetails {
        in_memory_cache_result: InMemoryCacheResult,
        foreground_time_in_microseconds: i64,
        background_time_in_microseconds: i64,
    }

    impl CompilationDetails {
        const kTimeNotMeasured: i64 = -1;
    }

    pub struct Source {
        source_string: Local<'static, String>,
        resource_name: Local<'static, Value>,
        resource_line_offset: i32,
        resource_column_offset: i32,
        resource_options: ScriptOriginOptions,
        source_map_url: Local<'static, Value>,
        host_defined_options: Local<'static, Data>,
        cached_data: Option<Box<CachedData>>,
        consume_cache_task: Option<Box<ConsumeCodeCacheTask>>,
        compile_hint_callback: Option<fn(Local<'static, Context>, Local<'static, Value>) -> MaybeLocal<'static, Value>>,
        compile_hint_callback_data: *mut std::ffi::c_void,
        compilation_details: CompilationDetails,
    }

    impl Source {
        pub fn new(source_string: Local<'static, String>, origin: &ScriptOrigin, cached_data: Option<Box<CachedData>>, consume_cache_task: Option<Box<ConsumeCodeCacheTask>>) -> Self {
            Source {
                source_string: source_string,
                resource_name: origin.ResourceName(),
                resource_line_offset: origin.LineOffset(),
                resource_column_offset: origin.ColumnOffset(),
                resource_options: origin.Options().clone(),
                source_map_url: origin.SourceMapUrl(),
                host_defined_options: origin.GetHostDefinedOptions(),
                cached_data: cached_data,
                consume_cache_task: consume_cache_task,
                compile_hint_callback: None,
                compile_hint_callback_data: ptr::null_mut(),
                compilation_details: CompilationDetails {
                    in_memory_cache_result: InMemoryCacheResult::kNotAttempted,
                    foreground_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                    background_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                },
            }
        }

        pub fn new_string(source_string: Local<'static, String>, cached_data: Option<Box<CachedData>>, consume_cache_task: Option<Box<ConsumeCodeCacheTask>>) -> Self {
            Source {
                source_string: source_string,
                resource_name: Value {},
                resource_line_offset: -1,
                resource_column_offset: -1,
                resource_options: ScriptOriginOptions {},
                source_map_url: Value {},
                host_defined_options: Data {},
                cached_data: cached_data,
                consume_cache_task: consume_cache_task,
                compile_hint_callback: None,
                compile_hint_callback_data: ptr::null_mut(),
                compilation_details: CompilationDetails {
                    in_memory_cache_result: InMemoryCacheResult::kNotAttempted,
                    foreground_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                    background_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                },
            }
        }

        pub fn new_callback(source_string: Local<'static, String>, origin: &ScriptOrigin, callback: fn(Local<'static, Context>, Local<'static, Value>) -> MaybeLocal<'static, Value>, callback_data: *mut std::ffi::c_void) -> Self {
            Source {
                source_string: source_string,
                resource_name: origin.ResourceName(),
                resource_line_offset: origin.LineOffset(),
                resource_column_offset: origin.ColumnOffset(),
                resource_options: origin.Options().clone(),
                source_map_url: origin.SourceMapUrl(),
                host_defined_options: origin.GetHostDefinedOptions(),
                cached_data: None,
                consume_cache_task: None,
                compile_hint_callback: Some(callback),
                compile_hint_callback_data: callback_data,
                compilation_details: CompilationDetails {
                    in_memory_cache_result: InMemoryCacheResult::kNotAttempted,
                    foreground_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                    background_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                },
            }
        }

        pub fn GetCachedData(&self) -> Option<&CachedData> {
            self.cached_data.as_ref().map(|boxed_data| &**boxed_data)
        }

        pub fn GetResourceOptions(&self) -> &ScriptOriginOptions {
            &self.resource_options
        }

        pub fn GetCompilationDetails(&self) -> &CompilationDetails {
            &self.compilation_details
        }
    }

    pub struct ExternalSourceStream {}
    impl ExternalSourceStream {
        pub fn GetMoreData(&mut self, src: *mut *const u8) -> usize {
            0
        }
    }

    pub struct StreamedSource {
        impl_: Box<internal::ScriptStreamingData>,
        compilation_details_: CompilationDetails,
    }

    impl StreamedSource {
        pub enum Encoding {
            ONE_BYTE,
            TWO_BYTE,
            UTF8,
            WINDOWS_1252,
        }

        pub fn new(source_stream: Box<ExternalSourceStream>, encoding: Encoding) -> Self {
            StreamedSource {
                impl_: Box::new(internal::ScriptStreamingData {}),
                compilation_details_: CompilationDetails {
                    in_memory_cache_result: InMemoryCacheResult::kNotAttempted,
                    foreground_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                    background_time_in_microseconds: CompilationDetails::kTimeNotMeasured,
                },
            }
        }

        pub fn impl_(&self) -> &internal::ScriptStreamingData {
            &self.impl_
        }

        pub fn compilation_details(&mut self) -> &mut CompilationDetails {
            &mut self.compilation_details_
        }
    }

    pub struct ScriptStreamingTask {
        data_: *mut internal::ScriptStreamingData,
    }

    impl ScriptStreamingTask {
        pub fn Run(&mut self) {}

        fn new(data: *mut internal::ScriptStreamingData) -> Self {
            ScriptStreamingTask { data_: data }
        }
    }

    pub struct ConsumeCodeCacheTask {
        impl_: Box<internal::BackgroundDeserializeTask>,
        should_merge_with_existing_script: Arc<Mutex<bool>>,
    }

    impl ConsumeCodeCacheTask {
        fn new(impl_: Box<internal::BackgroundDeserializeTask>) -> Self {
            ConsumeCodeCacheTask {
                impl_: impl_,
                should_merge_with_existing_script: Arc::new(Mutex::new(false)),
            }
        }

        pub fn Run(&mut self) {}

        pub fn SourceTextAvailable(&self, isolate: *mut Isolate, source_text: Local<'static, String>, origin: &ScriptOrigin) {
            let mut should_merge = self.should_merge_with_existing_script.lock().unwrap();
            *should_merge = false;
        }

        pub fn ShouldMergeWithExistingScript(&self) -> bool {
            let should_merge = self.should_merge_with_existing_script.lock().unwrap();
            *should_merge
        }

        pub fn MergeWithExistingScript(&mut self) {}
    }

    impl Drop for ConsumeCodeCacheTask {
        fn drop(&mut self) {}
    }

    pub enum CompileOptions {
        kNoCompileOptions = 0,
        kConsumeCodeCache = 1 << 0,
        kEagerCompile = 1 << 1,
        kProduceCompileHints = 1 << 2,
        kConsumeCompileHints = 1 << 3,
        kFollowCompileHintsMagicComment = 1 << 4,
        kFollowCompileHintsPerFunctionMagicComment = 1 << 5,
    }

    pub fn CompileOptionsIsValid(compile_options: CompileOptions) -> bool {
        match compile_options {
            CompileOptions::kConsumeCodeCache => true,
            CompileOptions::kEagerCompile => true,
            CompileOptions::kNoCompileOptions => true,
            _ => {
                let produce_and_consume =
                    CompileOptions::kProduceCompileHints as i32 | CompileOptions::kConsumeCompileHints as i32;
                if (compile_options as i32 & produce_and_consume) == produce_and_consume {
                    return false;
                }
                true
            }
        }
    }

    pub enum NoCacheReason {
        kNoCacheNoReason = 0,
        kNoCacheBecauseCachingDisabled,
        kNoCacheBecauseNoResource,
        kNoCacheBecauseInlineScript,
        kNoCacheBecauseModule,
        kNoCacheBecauseStreamingSource,
        kNoCacheBecauseInspector,
        kNoCacheBecauseScriptTooSmall,
        kNoCacheBecauseCacheTooCold,
        kNoCacheBecauseV8Extension,
        kNoCacheBecauseExtensionModule,
        kNoCacheBecausePacScript,
        kNoCacheBecauseInDocumentWrite,
        kNoCacheBecauseResourceWithNoCacheHandler,
        kNoCacheBecauseDeferredProduceCodeCache,
        kNoCacheBecauseStaticCodeCache,
    }

    pub fn CompileUnboundScript(
        isolate: *mut Isolate,
        source: &mut Source,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
    ) -> MaybeLocal<'static, UnboundScript> {
        ScriptCompiler::CompileUnboundInternal(isolate, source, options, no_cache_reason)
    }

    pub fn Compile(
        context: Local<'static, Context>,
        source: &mut Source,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
    ) -> MaybeLocal<'static, Script> {
        MaybeLocal::empty()
    }

    pub fn StartStreaming(
        isolate: *mut Isolate,
        source: &mut StreamedSource,
        type_: ScriptType,
        options: CompileOptions,
        compile_hint_callback: Option<fn(Local<'static, Context>, Local<'static, Value>) -> MaybeLocal<'static, Value>>,
        compile_hint_callback_data: *mut std::ffi::c_void,
    ) -> *mut ScriptStreamingTask {
        let data = source.impl_().clone();
        Box::into_raw(Box::new(ScriptStreamingTask::new(Box::into_raw(data))))
    }

    pub fn StartConsumingCodeCache(isolate: *mut Isolate, source: std::unique_ptr<CachedData>) -> *mut ConsumeCodeCacheTask {
       let source_moved = source.into_inner();

        let impl_ = Box::new(internal::BackgroundDeserializeTask {});
        Box::into_raw(Box::new(ConsumeCodeCacheTask::new(impl_)))
    }

    pub fn StartConsumingCodeCacheOnBackground(isolate: *mut Isolate, source: std::unique_ptr<CachedData>) -> *mut ConsumeCodeCacheTask {
      let source_moved = source.into_inner();

        let impl_ = Box::new(internal::BackgroundDeserializeTask {});
        Box::into_raw(Box::new(ConsumeCodeCacheTask::new(impl_)))
    }

    pub fn CompileStreamed(
        context: Local<'static, Context>,
        source: &mut StreamedSource,
        full_source_string: Local<'static, String>,
        origin: &ScriptOrigin,
    ) -> MaybeLocal<'static, Script> {
        MaybeLocal::empty()
    }

    pub fn CachedDataVersionTag() -> u32 {
        0
    }

    pub fn CompileModule(
        isolate: *mut Isolate,
        source: &mut Source,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
    ) -> MaybeLocal<'static, Module> {
        MaybeLocal::empty()
    }

    pub fn CompileModuleStreamed(
        context: Local<'static, Context>,
        v8_source: &mut StreamedSource,
        full_source_string: Local<'static, String>,
        origin: &ScriptOrigin,
    ) -> MaybeLocal<'static, Module> {
        MaybeLocal::empty()
    }

    pub fn CompileFunction(
        context: Local<'static, Context>,
        source: &mut Source,
        arguments_count: usize,
        arguments: *mut Local<'static, String>,
        context_extension_count: usize,
        context_extensions: *mut Local<'static, Object>,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
    ) -> MaybeLocal<'static, Function> {
        MaybeLocal::empty()
    }

    pub fn CreateCodeCache(unbound_script: Local<'static, UnboundScript>) -> *mut CachedData {
        ptr::null_mut()
    }

    pub fn CreateCodeCache_Module(unbound_module_script: Local<'static, UnboundModuleScript>) -> *mut CachedData {
        ptr::null_mut()
    }

    pub fn CreateCodeCacheForFunction(function: Local<'static, Function>) -> *mut CachedData {
        ptr::null_mut()
    }

    fn CompileUnboundInternal(
        isolate: *mut Isolate,
        source: &mut Source,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
    ) -> MaybeLocal<'static, UnboundScript> {
        MaybeLocal::empty()
    }

    fn CompileFunctionInternal(
        context: Local<'static, Context>,
        source: &mut Source,
        arguments_count: usize,
        arguments: *mut Local<'static, String>,
        context_extension_count: usize,
        context_extensions: *mut Local<'static, Object>,
        options: CompileOptions,
        no_cache_reason: NoCacheReason,
        script_or_module_out: *mut Local<'static, ScriptOrModule>,
    ) -> MaybeLocal<'static, Function> {
        MaybeLocal::empty()
    }
}

pub enum class ScriptType { kClassic, kModule }

#[derive(Clone)]
pub struct ScriptOriginOptions {}

pub struct ScriptOrigin {
    resource_name: Local<'static, Value>,
    line_offset: i32,
    column_offset: i32,
    options: ScriptOriginOptions,
    source_map_url: Local<'static, Value>,
    host_defined_options: Local<'static, Data>,
}

impl ScriptOrigin {
   pub fn ResourceName(&self) -> Local<'static, Value> {
        self.resource_name
    }

    pub fn LineOffset(&self) -> i32 {
        self.line_offset
    }

    pub fn ColumnOffset(&self) -> i32 {
        self.column_offset
    }

    pub fn Options(&self) -> &ScriptOriginOptions {
        &self.options
    }

    pub fn SourceMapUrl(&self) -> Local<'static, Value> {
        self.source_map_url
    }

    pub fn GetHostDefinedOptions(&self) -> Local<'static, Data> {
        self.host_defined_options
    }
}

pub type CompileHintCallback = fn(Local<'static, Context>, Local<'static, Value>) -> MaybeLocal<'static, Value>;

pub struct FixedArray {}
pub struct String {}
pub struct Local<'a, T> {}
pub struct Value {}

impl Local<'static, Value> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, String> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, Object> {
    pub fn empty() -> Self {
        Local {}
    }
}
impl Local<'static, Script> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, Data> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, Module> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, CompileHintsCollector> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, UnboundScript> {
    pub fn empty() -> Self {
        Local {}
    }
}

impl Local<'static, FixedArray> {
    pub fn empty() -> Self {
        Local {}
    }
}

pub struct MaybeLocal<'a, T> {}

impl<'a, T> MaybeLocal<'a, T> {
    pub fn empty() -> Self {
        MaybeLocal {}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Maybe<T> {
    has_value: bool,
    value: T,
}

impl<T: Copy> Maybe<T> {
    pub fn new(value: T) -> Self {
        Maybe {
            has_value: true,
            value: value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CompatibilityCheckResult {
    kSuccess = 0,
    kMagicNumberMismatch = 1,
    kVersionMismatch = 2,
    kSourceMismatch = 3,
    kFlagsMismatch = 5,
    kChecksumMismatch = 6,
    kInvalidHeader = 7,
    kLengthMismatch = 8,
    kReadOnlySnapshotChecksumMismatch = 9,
}

pub struct LocalVector<'a, T> {
}

impl<'a, T> LocalVector<'a, T> {
    pub fn new() -> Self {
        LocalVector{}
    }
}

pub struct Isolate {}
pub struct Object {}
pub struct Function {}

mod internal {
    pub struct ScriptStreamingData {}
    pub struct BackgroundDeserializeTask {}
    pub struct SharedObjectConveyorHandles {}
}
struct MemorySpan<'a, T> {
    data: *const T,
    length: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> MemorySpan<'a, T> {
    pub fn new(data: *const T, length: usize) -> Self {
        MemorySpan {
            data,
            length,
            _marker: std::marker::PhantomData,
        }
    }
}

impl From<*mut CachedData> for std::unique_ptr<CachedData> {
    fn from(ptr: *mut CachedData) -> Self {
        unsafe {
            if ptr.is_null() {
                std::unique_ptr::new()
            } else {
                std::unique_ptr::from_raw(ptr)
            }
        }
    }
}
trait IntoUniquePtr<T> {
    fn into_unique_ptr(self) -> std::unique_ptr<T>;
}

impl<T> IntoUniquePtr<T> for Box<T> {
    fn into_unique_ptr(self) -> std::unique_ptr<T> {
        unsafe { std::unique_ptr::from_raw(Box::into_raw(self)) }
    }
}

impl<T> std::unique_ptr<T> {
    pub fn into_inner(self) -> T {
       unsafe {
           let raw = Self::into_raw(self);
           std::ptr::read(raw)
       }
    }
}

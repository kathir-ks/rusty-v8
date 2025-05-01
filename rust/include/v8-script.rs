// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: Many of the types (Local, MaybeLocal, etc.) are placeholders.
//       They need to be replaced with appropriate Rust equivalents
//       based on the actual V8 bindings being used.

use std::any::Any;
use std::fmt;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::vec;

// Placeholder for v8-callbacks.h
mod v8_callbacks {
    pub enum ModuleImportPhase {
        Uninstantiated,
        Instantiating,
        Instantiated,
        Evaluating,
        Evaluated,
        Errored,
    }
}

// Placeholder for v8-data.h
mod v8_data {
    pub trait Data {
        fn as_any(&self) -> &dyn Any;
    }
}

// Placeholder for v8-local-handle.h
mod v8_local_handle {
    use std::ops::Deref;

    #[derive(Debug, Clone)]
    pub struct Local<T> {
        ptr: Arc<T>,
    }

    impl<T> Local<T> {
        pub fn new(value: T) -> Self {
            Local { ptr: Arc::new(value) }
        }
    }

    impl<T> Deref for Local<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.ptr
        }
    }
}

// Placeholder for v8-maybe.h
mod v8_maybe {
    pub type Maybe<T> = Option<T>;
    pub type MaybeLocal<T> = Option<super::v8_local_handle::Local<T>>;

    pub trait Check {
        fn is_nothing(&self) -> bool;
        fn is_just(&self) -> bool;
    }

    impl<T> Check for Maybe<T> {
        fn is_nothing(&self) -> bool {
            self.is_none()
        }
        fn is_just(&self) -> bool {
            self.is_some()
        }
    }

    impl<T> Check for MaybeLocal<T> {
        fn is_nothing(&self) -> bool {
            self.is_none()
        }
        fn is_just(&self) -> bool {
            self.is_some()
        }
    }
}

// Placeholder for v8-memory-span.h
mod v8_memory_span {
    use std::slice;

    #[derive(Debug)]
    pub struct MemorySpan<'a, T> {
        data: &'a [T],
    }

    impl<'a, T> MemorySpan<'a, T> {
        pub fn new(data: &'a [T]) -> Self {
            MemorySpan { data }
        }

        pub fn as_ptr(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    impl<'a, T> From<&'a [T]> for MemorySpan<'a, T> {
        fn from(slice: &'a [T]) -> Self {
            MemorySpan { data: slice }
        }
    }

    impl<'a, T> Into<&'a [T]> for MemorySpan<'a, T> {
        fn into(self) -> &'a [T] {
            self.data
        }
    }
}

// Placeholder for v8-message.h
mod v8_message {
    use super::v8_local_handle::Local;
    use super::v8_value::Value;
    pub struct Message {}

    impl Message {
        // Placeholder
        pub fn get(&self) -> Local<Value> {
            Local::new(Value {})
        }
    }
}

// Placeholder for v8config.h
mod v8config {
    // Configuration settings can be defined here
}

// Placeholder for v8-value.h
mod v8_value {
    pub struct Value {}
}

pub mod v8 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::vec;

    use super::v8_callbacks::*;
    use super::v8_data::*;
    use super::v8_local_handle::*;
    use super::v8_maybe::*;
    use super::v8_memory_span::*;
    use super::v8_message::*;
    use super::v8_value::*;

    pub struct Function {}
    pub struct Message {}
    pub struct Object {}
    pub struct PrimitiveArray {}
    pub struct Script {}
    pub struct String {}
    pub struct FixedArray {}
    pub struct Data {}
    pub struct Isolate {}

    impl Data {
        pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
            self.as_any().downcast_ref::<T>()
        }

        pub fn as_any(&self) -> &dyn std::any::Any {
            unimplemented!()
        }
    }

    impl Script {
        pub fn get_resource_name(&self) -> Local<Value> {
            Local::new(Value {})
        }
    }

    impl FixedArray {
        // Placeholder method
        pub fn get(&self, _index: usize) -> Local<Data> {
            Local::new(Data {})
        }
    }

    impl String {
        // Placeholder method
        pub fn new(_isolate: &Isolate, _str: &str) -> Local<String> {
            Local::new(String {})
        }
    }

    /// A container type that holds relevant metadata for module loading.
    ///
    /// This is passed back to the embedder as part of
    /// HostImportModuleDynamicallyCallback for module loading.
    pub struct ScriptOrModule {}

    impl ScriptOrModule {
        /// The name that was passed by the embedder as ResourceName to the
        /// ScriptOrigin. This can be either a v8::String or v8::Undefined.
        pub fn get_resource_name(&self) -> Local<Value> {
            unimplemented!()
        }

        /// The options that were passed by the embedder as HostDefinedOptions to
        /// the ScriptOrigin.
        pub fn host_defined_options(&self) -> Local<Data> {
            unimplemented!()
        }
    }

    /// A compiled JavaScript script, not yet tied to a Context.
    pub struct UnboundScript {}

    impl UnboundScript {
        /// Binds the script to the currently entered context.
        pub fn bind_to_current_context(&self) -> Local<Script> {
            unimplemented!()
        }

        pub fn get_id(&self) -> i32 {
            unimplemented!()
        }
        pub fn get_script_name(&self) -> Local<Value> {
            unimplemented!()
        }

        /// Data read from magic sourceURL comments.
        pub fn get_source_url(&self) -> Local<Value> {
            unimplemented!()
        }
        /// Data read from magic sourceMappingURL comments.
        pub fn get_source_mapping_url(&self) -> Local<Value> {
            unimplemented!()
        }

        /// Returns zero based line number of the code_pos location in the script.
        /// -1 will be returned if no information available.
        pub fn get_line_number(&self, _code_pos: i32) -> i32 {
            unimplemented!()
        }

        /// Returns zero based column number of the code_pos location in the script.
        /// -1 will be returned if no information available.
        pub fn get_column_number(&self, _code_pos: i32) -> i32 {
            unimplemented!()
        }

        pub const K_NO_SCRIPT_ID: i32 = 0;
    }

    /// A compiled JavaScript module, not yet tied to a Context.
    pub struct UnboundModuleScript {}

    impl UnboundModuleScript {
        /// Data read from magic sourceURL comments.
        pub fn get_source_url(&self) -> Local<Value> {
            unimplemented!()
        }
        /// Data read from magic sourceMappingURL comments.
        pub fn get_source_mapping_url(&self) -> Local<Value> {
            unimplemented!()
        }
    }

    /// A location in JavaScript source.
    #[derive(Debug, Clone, Copy)]
    pub struct Location {
        line_number: i32,
        column_number: i32,
    }

    impl Location {
        pub fn get_line_number(&self) -> i32 {
            self.line_number
        }
        pub fn get_column_number(&self) -> i32 {
            self.column_number
        }

        pub fn new(line_number: i32, column_number: i32) -> Self {
            Location {
                line_number,
                column_number,
            }
        }
    }

    pub struct ModuleRequest {}

    impl ModuleRequest {
        /// Returns the module specifier for this ModuleRequest.
        pub fn get_specifier(&self) -> Local<String> {
            unimplemented!()
        }

        /// Returns the module import phase for this ModuleRequest.
        pub fn get_phase(&self) -> ModuleImportPhase {
            unimplemented!()
        }

        /// Returns the source code offset of this module request.
        /// Use Module::SourceOffsetToLocation to convert this to line/column numbers.
        pub fn get_source_offset(&self) -> i32 {
            unimplemented!()
        }

        /// Contains the import attributes for this request in the form:
        /// [key1, value1, source_offset1, key2, value2, source_offset2, ...].
        /// The keys and values are of type v8::String, and the source offsets are of
        /// type Int32. Use Module::SourceOffsetToLocation to convert the source
        /// offsets to Locations with line/column numbers.
        ///
        /// All attributes present in the module request will be supplied in this
        /// list, regardless of whether they are supported by the host. Per
        /// https://tc39.es/proposal-import-attributes/#sec-hostgetsupportedimportattributes,
        /// hosts are expected to throw for attributes that they do not support (as
        /// opposed to, for example, ignoring them).
        pub fn get_import_attributes(&self) -> Local<FixedArray> {
            unimplemented!()
        }

        #[deprecated(since = "Use GetImportAttributes instead")]
        pub fn get_import_assertions(&self) -> Local<FixedArray> {
            self.get_import_attributes()
        }

        #[inline]
        pub fn cast(data: &Data) -> &ModuleRequest {
            // The check cast mechanism does not exist in Rust, so it's removed
            unsafe { &*(data as *const Data as *const ModuleRequest) }
        }
    }

    /// A compiled JavaScript module.
    pub struct Module {}

    impl Module {
        /// The different states a module can be in.
        ///
        /// This corresponds to the states used in ECMAScript except that "evaluated"
        /// is split into kEvaluated and kErrored, indicating success and failure,
        /// respectively.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Status {
            Uninstantiated,
            Instantiating,
            Instantiated,
            Evaluating,
            Evaluated,
            Errored,
        }

        /// Returns the module's current status.
        pub fn get_status(&self) -> Status {
            unimplemented!()
        }

        /// For a module in kErrored status, this returns the corresponding exception.
        pub fn get_exception(&self) -> Local<Value> {
            unimplemented!()
        }

        /// Returns the ModuleRequests for this module.
        pub fn get_module_requests(&self) -> Local<FixedArray> {
            unimplemented!()
        }

        /// For the given source text offset in this module, returns the corresponding
        /// Location with line and column numbers.
        pub fn source_offset_to_location(&self, offset: i32) -> Location {
            unimplemented!()
        }

        /// Returns the identity hash for this object.
        pub fn get_identity_hash(&self) -> i32 {
            unimplemented!()
        }

        pub type ResolveModuleCallback =
            fn(Local<Context>, Local<String>, Local<FixedArray>, Local<Module>) -> MaybeLocal<Module>;
        pub type ResolveSourceCallback =
            fn(Local<Context>, Local<String>, Local<FixedArray>, Local<Module>) -> MaybeLocal<Object>;

        /// Instantiates the module and its dependencies.
        ///
        /// Returns an empty Maybe<bool> if an exception occurred during
        /// instantiation. (In the case where the callback throws an exception, that
        /// exception is propagated.)
        pub fn instantiate_module(
            &self,
            _context: Local<Context>,
            _module_callback: ResolveModuleCallback,
            _source_callback: Option<ResolveSourceCallback>,
        ) -> Maybe<bool> {
            unimplemented!()
        }

        /// Evaluates the module and its dependencies.
        ///
        /// If status is kInstantiated, run the module's code and return a Promise
        /// object. On success, set status to kEvaluated and resolve the Promise with
        /// the completion value; on failure, set status to kErrored and reject the
        /// Promise with the error.
        ///
        /// If IsGraphAsync() is false, the returned Promise is settled.
        pub fn evaluate(&self, _context: Local<Context>) -> MaybeLocal<Value> {
            unimplemented!()
        }

        /// Returns the namespace object of this module.
        ///
        /// The module's status must be at least kInstantiated.
        pub fn get_module_namespace(&self) -> Local<Value> {
            unimplemented!()
        }

        /// Returns the corresponding context-unbound module script.
        ///
        /// The module must be unevaluated, i.e. its status must not be kEvaluating,
        /// kEvaluated or kErrored.
        pub fn get_unbound_module_script(&self) -> Local<UnboundModuleScript> {
            unimplemented!()
        }

        /// Returns the underlying script's id.
        ///
        /// The module must be a SourceTextModule and must not have a kErrored status.
        pub fn script_id(&self) -> i32 {
            unimplemented!()
        }

        /// Returns whether this module or any of its requested modules is async,
        /// i.e. contains top-level await.
        ///
        /// The module's status must be at least kInstantiated.
        pub fn is_graph_async(&self) -> bool {
            unimplemented!()
        }

        /// Returns whether this module is individually asynchronous (for example,
        /// if it's a Source Text Module Record containing a top-level await).
        /// See [[HasTLA]] in https://tc39.es/ecma262/#sec-cyclic-module-records
        pub fn has_top_level_await(&self) -> bool {
            unimplemented!()
        }

        /// Returns whether the module is a SourceTextModule.
        pub fn is_source_text_module(&self) -> bool {
            unimplemented!()
        }

        /// Returns whether the module is a SyntheticModule.
        pub fn is_synthetic_module(&self) -> bool {
            unimplemented!()
        }

        /*
         * Callback defined in the embedder.  This is responsible for setting
         * the module's exported values with calls to SetSyntheticModuleExport().
         * The callback must return a resolved Promise to indicate success (where no
         * exception was thrown) and return an empy MaybeLocal to indicate falure
         * (where an exception was thrown).
         */
        pub type SyntheticModuleEvaluationSteps =
            fn(Local<Context>, Local<Module>) -> MaybeLocal<Value>;

        /// Creates a new SyntheticModule with the specified export names, where
        /// evaluation_steps will be executed upon module evaluation.
        /// export_names must not contain duplicates.
        /// module_name is used solely for logging/debugging and doesn't affect module
        /// behavior.
        pub fn create_synthetic_module(
            isolate: &Isolate,
            module_name: Local<String>,
            export_names: MemorySpan<Local<String>>,
            evaluation_steps: SyntheticModuleEvaluationSteps,
        ) -> Local<Module> {
            unimplemented!()
        }

        /// Set this module's exported value for the name export_name to the specified
        /// export_value. This method must be called only on Modules created via
        /// CreateSyntheticModule.  An error will be thrown if export_name is not one
        /// of the export_names that were passed in that CreateSyntheticModule call.
        /// Returns Just(true) on success, Nothing<bool>() if an error was thrown.
        pub fn set_synthetic_module_export(
            &self,
            _isolate: &Isolate,
            _export_name: Local<String>,
            _export_value: Local<Value>,
        ) -> Maybe<bool> {
            unimplemented!()
        }

        /// Search the modules requested directly or indirectly by the module for
        /// any top-level await that has not yet resolved. If there is any, the
        /// returned pair of vectors (of equal size) contain the unresolved module
        /// and corresponding message with the pending top-level await.
        /// An embedder may call this before exiting to improve error messages.
        pub fn get_stalled_top_level_await_messages(
            &self,
            _isolate: &Isolate,
        ) -> (Vec<Local<Module>>, Vec<Local<Message>>) {
            unimplemented!()
        }

        #[inline]
        pub fn cast(data: &Data) -> &Module {
            // The check cast mechanism does not exist in Rust, so it's removed
            unsafe { &*(data as *const Data as *const Module) }
        }
    }

    pub struct CompileHintsCollector {}

    impl CompileHintsCollector {
        /// Returns the positions of lazy functions which were compiled and executed.
        pub fn get_compile_hints(&self, _isolate: &Isolate) -> Vec<i32> {
            unimplemented!()
        }
    }

    /// A compiled JavaScript script, tied to a Context which was active when the
    /// script was compiled.
    pub struct Script {}

    impl Script {
        /// A shorthand for ScriptCompiler::Compile().
        pub fn compile(
            _context: Local<Context>,
            _source: Local<String>,
            _origin: Option<&ScriptOrigin>,
        ) -> MaybeLocal<Script> {
            unimplemented!()
        }

        /// Runs the script returning the resulting value. It will be run in the
        /// context in which it was created (ScriptCompiler::CompileBound or
        /// UnboundScript::BindToCurrentContext()).
        pub fn run(&self, _context: Local<Context>) -> MaybeLocal<Value> {
            unimplemented!()
        }
        pub fn run(&self, _context: Local<Context>, _host_defined_options: Local<Data>) -> MaybeLocal<Value> {
            unimplemented!()
        }

        /// Returns the corresponding context-unbound script.
        pub fn get_unbound_script(&self) -> Local<UnboundScript> {
            unimplemented!()
        }

        /// The name that was passed by the embedder as ResourceName to the
        /// ScriptOrigin. This can be either a v8::String or v8::Undefined.
        pub fn get_resource_name(&self) -> Local<Value> {
            unimplemented!()
        }

        #[deprecated(since = "Use GetCompileHintsCollector instead")]
        pub fn get_produced_compile_hints(&self) -> Vec<i32> {
            unimplemented!()
        }

        /// Get a compile hints collector object which we can use later for retrieving
        /// compile hints (= positions of lazy functions which were compiled and
        /// executed).
        pub fn get_compile_hints_collector(&self) -> Local<CompileHintsCollector> {
            unimplemented!()
        }
    }

    pub enum ScriptType {
        Classic,
        Module,
    }

    /// For compiling scripts.
    pub struct ScriptCompiler {}

    impl ScriptCompiler {
        pub struct ConsumeCodeCacheTask {}

        impl ConsumeCodeCacheTask {
            fn new(_impl: std::unique_ptr<super::internal::BackgroundDeserializeTask>) -> Self {
                ConsumeCodeCacheTask {}
            }

            pub fn run(&self) {}

            /// Provides the source text string and origin information to the consumption
            /// task. May be called before, during, or after Run(). This step checks
            /// whether the script matches an existing script in the Isolate's
            /// compilation cache. To check whether such a script was found, call
            /// ShouldMergeWithExistingScript.
            ///
            /// The Isolate provided must be the same one used during
            /// StartConsumingCodeCache and must be currently entered on the thread that
            /// calls this function. The source text and origin provided in this step
            /// must precisely match those used later in the ScriptCompiler::Source that
            /// will contain this ConsumeCodeCacheTask.
            pub fn source_text_available(
                &self,
                _isolate: &Isolate,
                _source_text: Local<String>,
                _origin: &ScriptOrigin,
            ) {
                // unimplemented!()
            }

            /// Returns whether the embedder should call MergeWithExistingScript. This
            /// function may be called from any thread, any number of times, but its
            /// return value is only meaningful after SourceTextAvailable has completed.
            pub fn should_merge_with_existing_script(&self) -> bool {
                // unimplemented!()
                false
            }

            /// Merges newly deserialized data into an existing script which was found
            /// during SourceTextAvailable. May be called only after Run() has completed.
            /// Can execute on any thread, like Run().
            pub fn merge_with_existing_script(&self) {
                // unimplemented!()
            }
        }

        /// Compilation data that the embedder can cache and pass back to speed up
        /// future compilations. The data is produced if the CompilerOptions passed to
        /// the compilation functions in ScriptCompiler contains produce_data_to_cache
        /// = true. The data to cache can then can be retrieved from
        /// UnboundScript.
        pub struct CachedData {
            data: *const u8,
            length: i32,
            rejected: bool,
            buffer_policy: BufferPolicy,
        }

        #[derive(PartialEq)]
        pub enum BufferPolicy {
            BufferNotOwned,
            BufferOwned,
        }

        impl CachedData {
            pub fn new() -> Self {
                CachedData {
                    data: ptr::null(),
                    length: 0,
                    rejected: false,
                    buffer_policy: BufferPolicy::BufferNotOwned,
                }
            }

            // If buffer_policy is BufferNotOwned, the caller keeps the ownership of
            // data and guarantees that it stays alive until the CachedData object is
            // destroyed. If the policy is BufferOwned, the given data will be deleted
            // (with delete[]) when the CachedData object is destroyed.
            pub fn new_with_buffer(data: *const u8, length: i32, buffer_policy: BufferPolicy) -> Self {
                CachedData {
                    data,
                    length,
                    rejected: false,
                    buffer_policy,
                }
            }

            pub fn compatibility_check(_isolate: &Isolate) -> CompatibilityCheckResult {
                CompatibilityCheckResult::Success
            }
            /// Check if the CachedData can be loaded in the given isolate.

            pub enum CompatibilityCheckResult {
                // Don't change order/existing values of this enum since it keys into the
                // `code_cache_reject_reason` histogram. Append-only!
                Success = 0,
                MagicNumberMismatch = 1,
                VersionMismatch = 2,
                SourceMismatch = 3,
                FlagsMismatch = 5,
                ChecksumMismatch = 6,
                InvalidHeader = 7,
                LengthMismatch = 8,
                ReadOnlySnapshotChecksumMismatch = 9,

                // This should always point at the last real enum value.
                Last = ReadOnlySnapshotChecksumMismatch,
            }
        }

        impl Drop for CachedData {
            fn drop(&mut self) {
                if self.buffer_policy == BufferPolicy::BufferOwned {
                    unsafe {
                        // Assuming data was allocated with Vec::into_raw
                        if !self.data.is_null() {
                            let _ = Vec::from_raw_parts(
                                self.data as *mut u8,
                                self.length as usize,
                                self.length as usize,
                            );
                        }
                    }
                }
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum InMemoryCacheResult {
            // V8 did not attempt to find this script in its in-memory cache.
            NotAttempted,

            // V8 found a previously compiled copy of this script in its in-memory
            // cache. Any data generated by a streaming compilation or background
            // deserialization was abandoned.
            Hit,

            // V8 didn't have any previously compiled data for this script.
            Miss,

            // V8 had some previously compiled data for an identical script, but the
            // data was incomplete.
            Partial,
        }

        // Details about what happened during a compilation.
        #[derive(Debug)]
        pub struct CompilationDetails {
            pub in_memory_cache_result: InMemoryCacheResult,
            pub foreground_time_in_microseconds: i64,
            pub background_time_in_microseconds: i64,
        }

        impl CompilationDetails {
            pub const TIME_NOT_MEASURED: i64 = -1;

            pub fn new() -> Self {
                CompilationDetails {
                    in_memory_cache_result: InMemoryCacheResult::NotAttempted,
                    foreground_time_in_microseconds: Self::TIME_NOT_MEASURED,
                    background_time_in_microseconds: Self::TIME_NOT_MEASURED,
                }
            }
        }

        /// Source code which can be then compiled to a UnboundScript or Script.
        pub struct Source {
            source_string: Local<String>,

            // Origin information
            resource_name: Local<Value>,
            resource_line_offset: i32,
            resource_column_offset: i32,
            resource_options: ScriptOriginOptions,
            source_map_url: Local<Value>,
            host_defined_options: Local<Data>,

            // Cached data from previous compilation (if a kConsume*Cache flag is
            // set), or hold newly generated cache data (kProduce*Cache flags) are
            // set when calling a compile method.
            cached_data: Option<Box<CachedData>>,
            consume_cache_task: Option<Box<ConsumeCodeCacheTask>>,

            // For requesting compile hints from the embedder.
            compile_hint_callback: Option<CompileHintCallback>,
            compile_hint_callback_data: *mut std::ffi::c_void,

            // V8 writes this data and never reads it. It exists only to be informative
            // to the embedder.
            compilation_details: CompilationDetails,
        }

        impl Source {
            // Source takes ownership of both CachedData and CodeCacheConsumeTask.
            // The caller *must* ensure that the cached data is from a trusted source.
            #[inline]
            pub fn new(
                source_string: Local<String>,
                origin: &ScriptOrigin,
                cached_data: Option<Box<CachedData>>,
                consume_cache_task: Option<Box<ConsumeCodeCacheTask>>,
            ) -> Self {
                Source {
                    source_string,
                    resource_name: origin.resource_name.clone(),
                    resource_line_offset: origin.line_offset,
                    resource_column_offset: origin.column_offset,
                    resource_options: origin.options.clone(),
                    source_map_url: origin.source_map_url.clone(),
                    host_defined_options: origin.host_defined_options.clone(),
                    cached_data,
                    consume_cache_task,
                    compile_hint_callback: None,
                    compile_hint_callback_data: ptr::null_mut(),
                    compilation_details: CompilationDetails::new(),
                }
            }
            // Source takes ownership of both CachedData and CodeCacheConsumeTask.
            #[inline]
            pub fn new_with_cache(
                source_string: Local<String>,
                cached_data: Option<Box<CachedData>>,
                consume_cache_task: Option<Box<ConsumeCodeCacheTask>>,
            ) -> Self {
                Source {
                    source_string,
                    resource_name: Local::new(Value {}), // Placeholder
                    resource_line_offset: -1,
                    resource_column_offset: -1,
                    resource_options: ScriptOriginOptions::new(), // Placeholder
                    source_map_url: Local::new(Value {}),        // Placeholder
                    host_defined_options: Local::new(Data {}),     // Placeholder
                    cached_data,
                    consume_cache_task,
                    compile_hint_callback: None,
                    compile_hint_callback_data: ptr::null_mut(),
                    compilation_details: CompilationDetails::new(),
                }
            }
            #[inline]
            pub fn new_with_callback(
                source_string: Local<String>,
                origin: &ScriptOrigin,
                callback: CompileHintCallback,
                callback_data: *mut std::ffi::c_void,
            ) -> Self {
                Source {
                    source_string,
                    resource_name: origin.resource_name.clone(),
                    resource_line_offset: origin.line_offset,
                    resource_column_offset: origin.column_offset,
                    resource_options: origin.options.clone(),
                    source_map_url: origin.source_map_url.clone(),
                    host_defined_options: origin.host_defined_options.clone(),
                    cached_data: None,
                    consume_cache_task: None,
                    compile_hint_callback: Some(callback),
                    compile_hint_callback_data: callback_data,
                    compilation_details: CompilationDetails::new(),
                }
            }

            // Ownership of the CachedData or its buffers is *not* transferred to the
            // caller. The CachedData object is alive as long as the Source object is
            // alive.
            #[inline]
            pub fn get_cached_data(&self) -> Option<&CachedData> {
                self.cached_data.as_ref().map(|boxed_data| &**boxed_data)
            }

            #[inline]
            pub fn get_resource_options(&self) -> &ScriptOriginOptions {
                &self.resource_options
            }

            #[inline]
            pub fn get_compilation_details(&self) -> &CompilationDetails {
                &self.compilation_details
            }
        }

        /// For streaming incomplete script data to V8. The embedder should implement a
        /// subclass of this class.
        pub trait ExternalSourceStream {
            /// V8 calls this to request the next chunk of data from the embedder. This
            /// function will be called on a background thread, so it's OK to block and
            /// wait for the data, if the embedder doesn't have data yet. Returns the
            /// length of the data returned. When the data ends, GetMoreData should
            /// return 0. Caller takes ownership of the data.
            ///
            /// When streaming UTF-8 data, V8 handles multi-byte characters split between
            /// two data chunks, but doesn't handle multi-byte characters split between
            /// more than two data chunks. The embedder can avoid this problem by always
            /// returning at least 2 bytes of data.
            ///
            /// When streaming UTF-16 data, V8 does not handle characters split between
            /// two data chunks. The embedder has to make sure that chunks have an even
            /// length.
            ///
            /// If the embedder wants to cancel the streaming, they should make the next
            /// GetMoreData call return 0. V8 will interpret it as end of data (and most
            /// probably, parsing will fail). The streaming task will return as soon as
            /// V8 has parsed the data it received so far.
            fn get_more_data(&mut self) -> (usize, *const u8);
        }

        /// Source code which can be streamed into V8 in pieces. It will be parsed
        /// while streaming and compiled after parsing has completed. StreamedSource
        /// must be kept alive while the streaming task is run (see ScriptStreamingTask
        /// below).
        pub struct StreamedSource {
            impl_: std::unique_ptr<super::internal::ScriptStreamingData>,
            compilation_details: CompilationDetails,
        }

        impl StreamedSource {
            pub enum Encoding {
                ONE_BYTE,
                TWO_BYTE,
                UTF8,
                WINDOWS_1252,
            }

            pub fn new(source_stream: std::unique_ptr<dyn ExternalSourceStream>, encoding: Encoding) -> Self {
                StreamedSource {
                    impl_: std::unique_ptr::default(), // Placeholder
                    compilation_details: CompilationDetails::new(),
                }
            }

            pub fn compilation_details(&mut self) -> &mut CompilationDetails {
                &mut self.compilation_details
            }
        }

        /// A streaming task which the embedder must run on a background thread to
        /// stream scripts into V8. Returned by ScriptCompiler::StartStreaming.
        pub struct ScriptStreamingTask {
            data_: *mut super::internal::ScriptStreamingData,
        }

        impl ScriptStreamingTask {
            fn new(data: *mut super::internal::ScriptStreamingData) -> Self {
                ScriptStreamingTask { data_ }
            }

            pub fn run(&self) {}
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CompileOptions {
            NoCompileOptions = 0,
            ConsumeCodeCache = 1 << 0,
            EagerCompile = 1 << 1,
            ProduceCompileHints = 1 << 2,
            ConsumeCompileHints = 1 << 3,
            FollowCompileHintsMagicComment = 1 << 4,
            FollowCompile
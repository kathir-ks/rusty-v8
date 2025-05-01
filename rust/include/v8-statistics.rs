// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file:
// /home/kathirks_gc/v8_go/codebase/include/v8-statistics.h

pub mod statistics {
    use std::mem::MaybeUninit;

    /// Controls how the default MeasureMemoryDelegate reports the result of
    /// the memory measurement to JS. With kSummary only the total size is reported.
    /// With kDetailed the result includes the size of each native context.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MeasureMemoryMode {
        Summary,
        Detailed,
    }

    /// Controls how promptly a memory measurement request is executed.
    /// By default the measurement is folded with the next scheduled GC which may
    /// happen after a while and is forced after some timeout.
    /// The kEager mode starts incremental GC right away and is useful for testing.
    /// The kLazy mode does not force GC.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MeasureMemoryExecution {
        Default,
        Eager,
        Lazy,
    }

    /// Opaque type representing a V8 Context.  Needs to be defined properly with
    /// the `v8` crate and FFI bindings for full functionality.
    #[repr(C)]
    pub struct Context {
        _private: [u8; 0], // Ensure it's opaque and can't be directly constructed.
    }

    /// Opaque type representing a V8 Isolate. Needs to be defined properly with
    /// the `v8` crate and FFI bindings for full functionality.
    #[repr(C)]
    pub struct Isolate {
        _private: [u8; 0], // Ensure it's opaque and can't be directly constructed.
    }

    // Opaque type representing a V8 Promise Resolver. Needs to be defined properly
    // with the `v8` crate and FFI bindings for full functionality.
    #[repr(C)]
    pub struct PromiseResolver {
        _private: [u8; 0], // Ensure it's opaque and can't be directly constructed.
    }

    /// Opaque type representing a V8 Local Handle. Needs to be defined properly with
    /// the `v8` crate and FFI bindings for full functionality.
    #[repr(C)]
    pub struct Local<'a, T> {
        _private: [u8; 0], // Ensure it's opaque and can't be directly constructed.
        _phantom: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        // This is a placeholder.  In a real implementation, you'd need a
        // way to safely create a Local from a raw pointer, likely using
        // unsafe code and ensuring proper lifetime management.
        pub unsafe fn from_raw(ptr: *mut T) -> Self {
            // Placeholder - this is highly unsafe and needs proper handling.
            Local {
                _private: [],
                _phantom: std::marker::PhantomData,
            }
        }
    }

    /// Represents a span of memory. Mimics v8::MemorySpan.
    #[derive(Debug)]
    pub struct MemorySpan<'a, T> {
        data: &'a [T],
    }

    impl<'a, T> MemorySpan<'a, T> {
        pub fn new(data: &'a [T]) -> Self {
            MemorySpan { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn as_ptr(&self) -> *const T {
            self.data.as_ptr()
        }
    }

    /// The delegate is used in Isolate::MeasureMemory API.
    ///
    /// It specifies the contexts that need to be measured and gets called when
    /// the measurement is completed to report the results.
    ///
    /// Both MeasurementComplete() callbacks will be invoked on completion.
    /// Each implementation of this class should hence implement only one of them,
    /// and leave the other empty.
    pub trait MeasureMemoryDelegate {
        /// Returns true if the size of the given context needs to be measured.
        fn should_measure(&self, context: Local<Context>) -> bool;

        /// Holds the result of a memory measurement request.
        struct Result<'a> {
            /// Two spans of equal length: the first includes each context for which
            /// ShouldMeasure returned true and that was not garbage collected while
            /// the memory measurement was in progress; the second includes the size
            /// of the respective context.
            pub contexts: MemorySpan<'a, Local<'a, Context>>,
            pub sizes_in_bytes: MemorySpan<'a, usize>,

            /// Total size of objects that were not attributed to any context (i.e. are
            /// likely shared objects).
            pub unattributed_size_in_bytes: usize,

            /// Total size of generated code for Wasm (shared across contexts).
            pub wasm_code_size_in_bytes: usize,

            /// Total size of Wasm metadata (except code; shared across contexts).
            pub wasm_metadata_size_in_bytes: usize,
        }

        /// This function is called when memory measurement finishes.
        ///
        /// \param result the result of the measurement.
        fn measurement_complete(&mut self, result: Result);

        /// Returns a default delegate that resolves the given promise when
        /// the memory measurement completes.
        ///
        /// \param isolate the current isolate
        /// \param context the current context
        /// \param promise_resolver the promise resolver that is given the
        ///   result of the memory measurement.
        /// \param mode the detail level of the result.
        fn default(
            isolate: *mut Isolate,
            context: Local<Context>,
            promise_resolver: Local<PromiseResolver>,
            mode: MeasureMemoryMode,
        ) -> Box<dyn MeasureMemoryDelegate>;
    }

    /// Collection of shared per-process V8 memory information.
    ///
    /// Instances of this class can be passed to
    /// v8::V8::GetSharedMemoryStatistics to get shared memory statistics from V8.
    #[derive(Debug, Default)]
    pub struct SharedMemoryStatistics {
        read_only_space_size: usize,
        read_only_space_used_size: usize,
        read_only_space_physical_size: usize,
    }

    impl SharedMemoryStatistics {
        pub fn new() -> Self {
            SharedMemoryStatistics::default()
        }
        pub fn read_only_space_size(&self) -> usize {
            self.read_only_space_size
        }
        pub fn read_only_space_used_size(&self) -> usize {
            self.read_only_space_used_size
        }
        pub fn read_only_space_physical_size(&self) -> usize {
            self.read_only_space_physical_size
        }

        // Methods to set the values, only used internally by V8.
        // These should only be called from V8 through FFI.
        pub(crate) fn set_read_only_space_size(&mut self, size: usize) {
            self.read_only_space_size = size;
        }

        pub(crate) fn set_read_only_space_used_size(&mut self, size: usize) {
            self.read_only_space_used_size = size;
        }

        pub(crate) fn set_read_only_space_physical_size(&mut self, size: usize) {
            self.read_only_space_physical_size = size;
        }
    }

    /// Collection of V8 heap information.
    ///
    /// Instances of this class can be passed to v8::Isolate::GetHeapStatistics to
    /// get heap statistics from V8.
    #[derive(Debug, Default)]
    pub struct HeapStatistics {
        total_heap_size: usize,
        total_heap_size_executable: usize,
        total_physical_size: usize,
        total_available_size: usize,
        used_heap_size: usize,
        heap_size_limit: usize,
        malloced_memory: usize,
        external_memory: usize,
        peak_malloced_memory: usize,
        does_zap_garbage: bool,
        number_of_native_contexts: usize,
        number_of_detached_contexts: usize,
        total_global_handles_size: usize,
        used_global_handles_size: usize,
    }

    impl HeapStatistics {
        pub fn new() -> Self {
            HeapStatistics::default()
        }
        pub fn total_heap_size(&self) -> usize {
            self.total_heap_size
        }
        pub fn total_heap_size_executable(&self) -> usize {
            self.total_heap_size_executable
        }
        pub fn total_physical_size(&self) -> usize {
            self.total_physical_size
        }
        pub fn total_available_size(&self) -> usize {
            self.total_available_size
        }
        pub fn used_heap_size(&self) -> usize {
            self.used_heap_size
        }
        pub fn heap_size_limit(&self) -> usize {
            self.heap_size_limit
        }
        pub fn malloced_memory(&self) -> usize {
            self.malloced_memory
        }
        pub fn external_memory(&self) -> usize {
            self.external_memory
        }
        pub fn peak_malloced_memory(&self) -> usize {
            self.peak_malloced_memory
        }
        pub fn does_zap_garbage(&self) -> bool {
            self.does_zap_garbage
        }
        pub fn number_of_native_contexts(&self) -> usize {
            self.number_of_native_contexts
        }
        pub fn number_of_detached_contexts(&self) -> usize {
            self.number_of_detached_contexts
        }
        pub fn total_global_handles_size(&self) -> usize {
            self.total_global_handles_size
        }
        pub fn used_global_handles_size(&self) -> usize {
            self.used_global_handles_size
        }

        // Methods to set the values, only used internally by V8.
        // These should only be called from V8 through FFI.

        pub(crate) fn set_total_heap_size(&mut self, size: usize) {
            self.total_heap_size = size;
        }

        pub(crate) fn set_total_heap_size_executable(&mut self, size: usize) {
            self.total_heap_size_executable = size;
        }

        pub(crate) fn set_total_physical_size(&mut self, size: usize) {
            self.total_physical_size = size;
        }

        pub(crate) fn set_total_available_size(&mut self, size: usize) {
            self.total_available_size = size;
        }

        pub(crate) fn set_used_heap_size(&mut self, size: usize) {
            self.used_heap_size = size;
        }

        pub(crate) fn set_heap_size_limit(&mut self, size: usize) {
            self.heap_size_limit = size;
        }

        pub(crate) fn set_malloced_memory(&mut self, size: usize) {
            self.malloced_memory = size;
        }

        pub(crate) fn set_external_memory(&mut self, size: usize) {
            self.external_memory = size;
        }

        pub(crate) fn set_peak_malloced_memory(&mut self, size: usize) {
            self.peak_malloced_memory = size;
        }

        pub(crate) fn set_does_zap_garbage(&mut self, value: bool) {
            self.does_zap_garbage = value;
        }

        pub(crate) fn set_number_of_native_contexts(&mut self, count: usize) {
            self.number_of_native_contexts = count;
        }

        pub(crate) fn set_number_of_detached_contexts(&mut self, count: usize) {
            self.number_of_detached_contexts = count;
        }

        pub(crate) fn set_total_global_handles_size(&mut self, size: usize) {
            self.total_global_handles_size = size;
        }

        pub(crate) fn set_used_global_handles_size(&mut self, size: usize) {
            self.used_global_handles_size = size;
        }
    }

    #[derive(Debug, Default)]
    pub struct HeapSpaceStatistics {
        space_name: String,
        space_size: usize,
        space_used_size: usize,
        space_available_size: usize,
        physical_space_size: usize,
    }

    impl HeapSpaceStatistics {
        pub fn new() -> Self {
            HeapSpaceStatistics::default()
        }

        pub fn space_name(&self) -> &str {
            &self.space_name
        }

        pub fn space_size(&self) -> usize {
            self.space_size
        }

        pub fn space_used_size(&self) -> usize {
            self.space_used_size
        }

        pub fn space_available_size(&self) -> usize {
            self.space_available_size
        }

        pub fn physical_space_size(&self) -> usize {
            self.physical_space_size
        }

        // Methods to set the values, only used internally by V8.
        // These should only be called from V8 through FFI.
        pub(crate) fn set_space_name(&mut self, name: String) {
            self.space_name = name;
        }

        pub(crate) fn set_space_size(&mut self, size: usize) {
            self.space_size = size;
        }

        pub(crate) fn set_space_used_size(&mut self, size: usize) {
            self.space_used_size = size;
        }

        pub(crate) fn set_space_available_size(&mut self, size: usize) {
            self.space_available_size = size;
        }

        pub(crate) fn set_physical_space_size(&mut self, size: usize) {
            self.physical_space_size = size;
        }
    }

    #[derive(Debug, Default)]
    pub struct HeapObjectStatistics {
        object_type: String,
        object_sub_type: String,
        object_count: usize,
        object_size: usize,
    }

    impl HeapObjectStatistics {
        pub fn new() -> Self {
            HeapObjectStatistics::default()
        }

        pub fn object_type(&self) -> &str {
            &self.object_type
        }

        pub fn object_sub_type(&self) -> &str {
            &self.object_sub_type
        }

        pub fn object_count(&self) -> usize {
            self.object_count
        }

        pub fn object_size(&self) -> usize {
            self.object_size
        }

        // Methods to set the values, only used internally by V8.
        // These should only be called from V8 through FFI.

        pub(crate) fn set_object_type(&mut self, object_type: String) {
            self.object_type = object_type;
        }

        pub(crate) fn set_object_sub_type(&mut self, object_sub_type: String) {
            self.object_sub_type = object_sub_type;
        }

        pub(crate) fn set_object_count(&mut self, object_count: usize) {
            self.object_count = object_count;
        }

        pub(crate) fn set_object_size(&mut self, object_size: usize) {
            self.object_size = object_size;
        }
    }

    #[derive(Debug, Default)]
    pub struct HeapCodeStatistics {
        code_and_metadata_size: usize,
        bytecode_and_metadata_size: usize,
        external_script_source_size: usize,
        cpu_profiler_metadata_size: usize,
    }

    impl HeapCodeStatistics {
        pub fn new() -> Self {
            HeapCodeStatistics::default()
        }

        pub fn code_and_metadata_size(&self) -> usize {
            self.code_and_metadata_size
        }
        pub fn bytecode_and_metadata_size(&self) -> usize {
            self.bytecode_and_metadata_size
        }
        pub fn external_script_source_size(&self) -> usize {
            self.external_script_source_size
        }
        pub fn cpu_profiler_metadata_size(&self) -> usize {
            self.cpu_profiler_metadata_size
        }

        // Methods to set the values, only used internally by V8.
        // These should only be called from V8 through FFI.

        pub(crate) fn set_code_and_metadata_size(&mut self, size: usize) {
            self.code_and_metadata_size = size;
        }

        pub(crate) fn set_bytecode_and_metadata_size(&mut self, size: usize) {
            self.bytecode_and_metadata_size = size;
        }

        pub(crate) fn set_external_script_source_size(&mut self, size: usize) {
            self.external_script_source_size = size;
        }

        pub(crate) fn set_cpu_profiler_metadata_size(&mut self, size: usize) {
            self.cpu_profiler_metadata_size = size;
        }
    }
}
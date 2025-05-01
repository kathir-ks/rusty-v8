pub mod v8_profiler {
    use std::{
        collections::HashSet,
        fmt,
        mem::MaybeUninit,
        num::NonZeroU32,
        ptr::NonNull,
        sync::{Arc, Mutex, RwLock},
    };

    //use cppgc::common; // Assuming this is an internal detail, stubbing
    //use v8::{Local, String, Value, Context, Isolate, Data, Object, UnboundScript}; // Using stubs for v8 types
    //use v8::{Message};

    /// Profiler support for the V8 JavaScript engine.
    pub mod v8 {
        /// A stub type for v8::Local<T>
        #[derive(Debug, Clone, Copy)]
        pub struct Local<T>(pub(crate) NonNull<T>);

        impl<T> Local<T> {
            pub fn new(ptr: NonNull<T>) -> Self {
                Local(ptr)
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct String {}
        #[derive(Debug, Clone, Copy)]
        pub struct Value {}
        #[derive(Debug, Clone, Copy)]
        pub struct Context {}
        #[derive(Debug, Clone, Copy)]
        pub struct Isolate {}
        #[derive(Debug, Clone, Copy)]
        pub struct Data {}
        #[derive(Debug, Clone, Copy)]
        pub struct Object {}
        #[derive(Debug, Clone, Copy)]
        pub struct UnboundScript {}

        impl UnboundScript {
            pub const K_NO_SCRIPT_ID: i32 = -1;
        }

        pub mod message {
            pub const K_NO_LINE_NUMBER_INFO: i32 = -1;
            pub const K_NO_COLUMN_INFO: i32 = -1;
        }

        #[derive(Debug, Clone, Copy)]
        pub enum EmbedderStateTag {
            // Define variants as needed, based on usage
            Unknown,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum StateTag {
            // Define variants as needed, based on usage
            Unknown,
        }

        pub type NativeObject = *mut std::ffi::c_void;
        pub type SnapshotObjectId = u32;
        pub type ProfilerId = u32;

        #[derive(Debug, Clone)]
        pub struct CpuProfileDeoptFrame {
            pub script_id: i32,
            pub position: usize,
        }

        pub mod internal {
            pub struct CpuProfile {}
        }
    }

    #[derive(Debug, Clone)]
    pub struct CpuProfileDeoptInfo {
        /// A pointer to a static string owned by v8.
        pub deopt_reason: &'static str,
        pub stack: Vec<v8::CpuProfileDeoptFrame>,
    }

    /// CpuProfileNode represents a node in a call graph.
    pub struct CpuProfileNode {
        function_name: StringWrapper,
        script_id: i32,
        script_resource_name: StringWrapper,
        script_shared_cross_origin: bool,
        line_number: i32,
        column_number: i32,
        hit_line_count: u32,
        line_ticks: Arc<Mutex<Vec<LineTick>>>,
        bailout_reason: Option<&'static str>,
        hit_count: u32,
        node_id: u32,
        source_type: SourceType,
        children: Vec<Arc<CpuProfileNode>>,
        parent: Option<NonNull<CpuProfileNode>>,
        deopt_infos: Vec<CpuProfileDeoptInfo>,
    }

    impl CpuProfileNode {
        pub const K_NO_LINE_NUMBER_INFO: i32 = v8::message::K_NO_LINE_NUMBER_INFO;
        pub const K_NO_COLUMN_NUMBER_INFO: i32 = v8::message::K_NO_COLUMN_INFO;

        /// Returns function name (empty string for anonymous functions.)
        pub fn get_function_name(&self) -> v8::Local<v8::String> {
            self.function_name.get_local()
        }

        /// Returns function name (empty string for anonymous functions.)
        /// The string ownership is *not* passed to the caller. It stays valid until
        /// profile is deleted. The function is thread safe.
        pub fn get_function_name_str(&self) -> &str {
            self.function_name.get_str()
        }

        /// Returns id of the script where function is located.
        pub fn get_script_id(&self) -> i32 {
            self.script_id
        }

        /// Returns resource name for script from where the function originates.
        pub fn get_script_resource_name(&self) -> v8::Local<v8::String> {
            self.script_resource_name.get_local()
        }

        /// Returns resource name for script from where the function originates.
        /// The string ownership is *not* passed to the caller. It stays valid until
        /// profile is deleted. The function is thread safe.
        pub fn get_script_resource_name_str(&self) -> &str {
            self.script_resource_name.get_str()
        }

        /// Return true if the script from where the function originates is flagged as
        /// being shared cross-origin.
        pub fn is_script_shared_cross_origin(&self) -> bool {
            self.script_shared_cross_origin
        }

        /// Returns the number, 1-based, of the line where the function originates.
        /// kNoLineNumberInfo if no line number information is available.
        pub fn get_line_number(&self) -> i32 {
            self.line_number
        }

        /// Returns 1-based number of the column where the function originates.
        /// kNoColumnNumberInfo if no column number information is available.
        pub fn get_column_number(&self) -> i32 {
            self.column_number
        }

        /// Returns the number of the function's source lines that collect the samples.
        pub fn get_hit_line_count(&self) -> u32 {
            self.hit_line_count
        }

        /// Returns the set of source lines that collect the samples.
        ///  The caller allocates buffer and responsible for releasing it.
        ///  True if all available entries are copied, otherwise false.
        ///  The function copies nothing if buffer is not large enough.
        pub fn get_line_ticks(&self, entries: &mut [LineTick]) -> bool {
            let guard = self.line_ticks.lock().unwrap();
            if entries.len() < guard.len() {
                return false;
            }

            entries[..guard.len()].copy_from_slice(&guard[..]);
            true
        }

        /// Returns bailout reason for the function
        /// if the optimization was disabled for it.
        pub fn get_bailout_reason(&self) -> Option<&'static str> {
            self.bailout_reason
        }

        /// Returns the count of samples where the function was currently executing.
        pub fn get_hit_count(&self) -> u32 {
            self.hit_count
        }

        /// Returns id of the node. The id is unique within the tree
        pub fn get_node_id(&self) -> u32 {
            self.node_id
        }

        /// Gets the type of the source which the node was captured from.
        pub fn get_source_type(&self) -> SourceType {
            self.source_type
        }

        /// Returns child nodes count of the node.
        pub fn get_children_count(&self) -> usize {
            self.children.len()
        }

        /// Retrieves a child node by index.
        pub fn get_child(&self, index: usize) -> Option<Arc<CpuProfileNode>> {
            self.children.get(index).cloned()
        }

        /// Retrieves the ancestor node, or null if the root.
        pub fn get_parent(&self) -> Option<&CpuProfileNode> {
            unsafe { self.parent.map(|ptr| ptr.as_ref()) }
        }

        /// Retrieves deopt infos for the node.
        pub fn get_deopt_infos(&self) -> &Vec<CpuProfileDeoptInfo> {
            &self.deopt_infos
        }
    }

    impl fmt::Debug for CpuProfileNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CpuProfileNode")
                .field("function_name", &self.function_name.get_str())
                .field("script_id", &self.script_id)
                .field("script_resource_name", &self.script_resource_name.get_str())
                .field("script_shared_cross_origin", &self.script_shared_cross_origin)
                .field("line_number", &self.line_number)
                .field("column_number", &self.column_number)
                .field("hit_line_count", &self.hit_line_count)
                .field("bailout_reason", &self.bailout_reason)
                .field("hit_count", &self.hit_count)
                .field("node_id", &self.node_id)
                .field("source_type", &self.source_type)
                .field("children_count", &self.children.len())
                .field("deopt_infos_count", &self.deopt_infos.len())
                .finish()
        }
    }

    #[derive(Debug, Clone)]
    struct StringWrapper {
        //Using an Arc<String> so that it can be safely used with multiple threads
        string: Arc<StringData>,
    }

    impl StringWrapper {
        fn get_str(&self) -> &str {
            &self.string.data
        }

        fn get_local(&self) -> v8::Local<v8::String> {
            //StringWrapper is used to wrap strings within v8, this could result
            //in a double free if the lifetime of v8::String does not exceed the lifetime of the StringWrapper
            //Therefore, we return a stub type that allows the program to compile
            //but protects against double free's
            unsafe {
                let str: *mut v8::String = std::alloc::alloc(std::alloc::Layout::new::<v8::String>()) as *mut v8::String;
                v8::Local::new(NonNull::new(str).unwrap())
            }
        }
    }

    impl From<&'static str> for StringWrapper {
        fn from(value: &'static str) -> Self {
            StringWrapper {
                string: Arc::new(StringData {
                    data: value.to_string(),
                }),
            }
        }
    }

    impl From<String> for StringWrapper {
        fn from(value: String) -> Self {
            StringWrapper {
                string: Arc::new(StringData { data: value }),
            }
        }
    }

    impl From<&str> for StringWrapper {
        fn from(value: &str) -> Self {
            StringWrapper {
                string: Arc::new(StringData {
                    data: value.to_string(),
                }),
            }
        }
    }

    #[derive(Debug, Clone)]
    struct StringData {
        data: String,
    }

    /// An annotation hinting at the source of a CpuProfileNode.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SourceType {
        /// User-supplied script with associated resource information.
        Script = 0,
        /// Native scripts and provided builtins.
        Builtin = 1,
        /// Callbacks into native code.
        Callback = 2,
        /// VM-internal functions or state.
        Internal = 3,
        /// A node that failed to symbolize.
        Unresolved = 4,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LineTick {
        /// The 1-based number of the source line where the function originates.
        pub line: i32,
        /// The count of samples associated with the source line.
        pub hit_count: u32,
    }

    /// An interface for exporting data from V8, using "push" model.
    pub trait OutputStream {
        /// Notify about the end of stream.
        fn end_of_stream(&mut self);
        /// Get preferred output chunk size. Called only once.
        fn get_chunk_size(&self) -> i32 {
            1024
        }
        /// Writes the next chunk of snapshot data into the stream. Writing
        /// can be stopped by returning kAbort as function result. EndOfStream
        /// will not be called in case writing was aborted.
        fn write_ascii_chunk(&mut self, data: *mut i8, size: i32) -> WriteResult;
        /// Writes the next chunk of heap stats data into the stream. Writing
        /// can be stopped by returning kAbort as function result. EndOfStream
        /// will not be called in case writing was aborted.
        fn write_heap_stats_chunk(&mut self, data: *mut HeapStatsUpdate, count: i32) -> WriteResult {
            WriteResult::Abort
        }
    }

    /// Represents the result of a write operation to the OutputStream.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WriteResult {
        Continue = 0,
        Abort = 1,
    }

    /// CpuProfile contains a CPU profile in a form of top-down call tree
    /// (from main() down to functions that do all the work).
    pub struct CpuProfile {
        title: StringWrapper,
        top_down_root: Arc<CpuProfileNode>,
        samples: Vec<NonNull<CpuProfileNode>>,
        sample_timestamps: Vec<i64>,
        sample_states: Vec<v8::StateTag>,
        sample_embedder_states: Vec<v8::EmbedderStateTag>,
        start_time: i64,
        end_time: i64,
    }

    impl CpuProfile {
        /// Returns CPU profile title.
        pub fn get_title(&self) -> v8::Local<v8::String> {
            self.title.get_local()
        }

        /// Returns the root node of the top down call tree.
        pub fn get_top_down_root(&self) -> Arc<CpuProfileNode> {
            self.top_down_root.clone()
        }

        /// Returns number of samples recorded. The samples are not recorded unless
        /// |record_samples| parameter of CpuProfiler::StartCpuProfiling is true.
        pub fn get_samples_count(&self) -> usize {
            self.samples.len()
        }

        /// Returns profile node corresponding to the top frame the sample at
        /// the given index.
        pub fn get_sample(&self, index: usize) -> Option<Arc<CpuProfileNode>> {
            self.samples.get(index).map(|&ptr| unsafe {
                let ptr = ptr.as_ptr();
                let ptr = &*ptr;
                ptr.into()
            })
        }

        /// Returns the timestamp of the sample. The timestamp is the number of
        /// microseconds since some unspecified starting point.
        /// The point is equal to the starting point used by GetStartTime.
        pub fn get_sample_timestamp(&self, index: usize) -> Option<i64> {
            self.sample_timestamps.get(index).copied()
        }

        /// Returns time when the profile recording was started (in microseconds)
        /// since some unspecified starting point.
        pub fn get_start_time(&self) -> i64 {
            self.start_time
        }

        /// Returns state of the vm when sample was captured.
        pub fn get_sample_state(&self, index: usize) -> Option<v8::StateTag> {
            self.sample_states.get(index).copied()
        }

        /// Returns state of the embedder when sample was captured.
        pub fn get_sample_embedder_state(&self, index: usize) -> Option<v8::EmbedderStateTag> {
            self.sample_embedder_states.get(index).copied()
        }

        /// Returns time when the profile recording was stopped (in microseconds)
        /// since some unspecified starting point.
        /// The point is equal to the starting point used by GetStartTime.
        pub fn get_end_time(&self) -> i64 {
            self.end_time
        }

        /// Deletes the profile and removes it from CpuProfiler's list.
        /// All pointers to nodes previously returned become invalid.
        pub fn delete(self) {
            //drop self here, this deallocates the data
        }

        /// Prepare a serialized representation of the profile. The result
        /// is written into the stream provided in chunks of specified size.
        ///
        /// For the JSON format, heap contents are represented as an object
        /// with the following structure:
        ///
        ///  {
        ///    nodes: [nodes array],
        ///    startTime: number,
        ///    endTime: number
        ///    samples: [strings array]
        ///    timeDeltas: [numbers array]
        ///  }
        ///
        pub fn serialize(&self, stream: &mut dyn OutputStream, format: SerializationFormat) {
            // Placeholder implementation
            match format {
                SerializationFormat::JSON => {
                    // Serialize to JSON format
                    todo!("Implement JSON serialization");
                }
            }
            stream.end_of_stream();
        }
    }

    impl fmt::Debug for CpuProfile {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CpuProfile")
                .field("title", &self.title.get_str())
                .field("samples_count", &self.samples.len())
                .field("start_time", &self.start_time)
                .field("end_time", &self.end_time)
                .finish()
        }
    }

    /// Enum of serialization formats.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SerializationFormat {
        JSON = 0, // See format description near 'Serialize' method.
    }

    /// Determines how line numbers are computed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CpuProfilingMode {
        /// In the resulting CpuProfile tree, intermediate nodes in a stack trace
        /// (from the root to a leaf) will have line numbers that point to the start
        /// line of the function, rather than the line of the callsite of the child.
        LeafNodeLineNumbers,
        /// In the resulting CpuProfile tree, nodes are separated based on the line
        /// number of their callsite in their parent.
        CallerLineNumbers,
    }

    // Determines how names are derived for functions sampled.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CpuProfilingNamingMode {
        // Use the immediate name of functions at compilation time.
        StandardNaming,
        // Use more verbose naming for functions without names, inferred from scope
        // where possible.
        DebugNaming,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CpuProfilingLoggingMode {
        // Enables logging when a profile is active, and disables logging when all
        // profiles are detached.
        LazyLogging,
        // Enables logging for the lifetime of the CpuProfiler. Calls to
        // StartRecording are faster, at the expense of runtime overhead.
        EagerLogging,
    }

    // Enum for returning profiling status. Once StartProfiling is called,
    // we want to return to clients whether the profiling was able to start
    // correctly, or return a descriptive error.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CpuProfilingStatus {
        kStarted,
        kAlreadyStarted,
        kErrorTooManyProfilers,
    }

    /// Result from StartProfiling returning the Profiling Status, and
    /// id of the started profiler, or 0 if profiler is not started
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CpuProfilingResult {
        pub id: v8::ProfilerId,
        pub status: CpuProfilingStatus,
    }

    /// Delegate for when max samples reached and samples are discarded.
    pub trait DiscardedSamplesDelegate {
        fn notify(&mut self);

        fn get_id(&self) -> v8::ProfilerId;
    }

    /// Optional profiling attributes.
    pub struct CpuProfilingOptions {
        mode: CpuProfilingMode,
        max_samples: u32,
        sampling_interval_us: i32,
        filter_context: Option<v8::Local<v8::Context>>,
    }

    impl CpuProfilingOptions {
        /// Indicates that the sample buffer size should not be explicitly limited.
        pub const K_NO_SAMPLE_LIMIT: u32 = u32::MAX;

        /// \param mode Type of computation of stack frame line numbers.
        /// \param max_samples The maximum number of samples that should be recorded by
        ///                    the profiler. Samples obtained after this limit will be
        ///                    discarded.
        /// \param sampling_interval_us controls the profile-specific target
        ///                             sampling interval. The provided sampling
        ///                             interval will be snapped to the next lowest
        ///                             non-zero multiple of the profiler's sampling
        ///                             interval, set via SetSamplingInterval(). If
        ///                             zero, the sampling interval will be equal to
        ///                             the profiler's sampling interval.
        /// \param filter_context If specified, profiles will only contain frames
        ///                       using this context. Other frames will be elided.
        pub fn new(
            mode: CpuProfilingMode,
            max_samples: u32,
            sampling_interval_us: i32,
            filter_context: Option<v8::Local<v8::Context>>,
        ) -> Self {
            Self {
                mode,
                max_samples,
                sampling_interval_us,
                filter_context,
            }
        }

        pub fn mode(&self) -> CpuProfilingMode {
            self.mode
        }
        pub fn max_samples(&self) -> u32 {
            self.max_samples
        }
        pub fn sampling_interval_us(&self) -> i32 {
            self.sampling_interval_us
        }
    }

    impl Default for CpuProfilingOptions {
        fn default() -> Self {
            CpuProfilingOptions {
                mode: CpuProfilingMode::LeafNodeLineNumbers,
                max_samples: CpuProfilingOptions::K_NO_SAMPLE_LIMIT,
                sampling_interval_us: 0,
                filter_context: None,
            }
        }
    }

    impl fmt::Debug for CpuProfilingOptions {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CpuProfilingOptions")
                .field("mode", &self.mode)
                .field("max_samples", &self.max_samples)
                .field("sampling_interval_us", &self.sampling_interval_us)
                .finish()
        }
    }

    /// Interface for controlling CPU profiling. Instance of the
    /// profiler can be created using v8::CpuProfiler::New method.
    pub struct CpuProfiler {
        isolate: *mut v8::Isolate,
        naming_mode: CpuProfilingNamingMode,
        logging_mode: CpuProfilingLoggingMode,
        sampling_interval_us: i32,
        profiles: Arc<Mutex<Vec<Box<CpuProfile>>>>,
        discarded_samples_delegates: Arc<Mutex<Vec<Box<dyn DiscardedSamplesDelegate>>>>,
        use_precise_sampling: bool,
    }

    impl CpuProfiler {
        /// Creates a new CPU profiler for the |isolate|. The isolate must be
        /// initialized. The profiler object must be disposed after use by calling
        /// |Dispose| method.
        pub fn new(isolate: *mut v8::Isolate, naming_mode: CpuProfilingNamingMode, logging_mode: CpuProfilingLoggingMode) -> Self {
            CpuProfiler {
                isolate,
                naming_mode,
                logging_mode,
                sampling_interval_us: 1000,
                profiles: Arc::new(Mutex::new(Vec::new())),
                discarded_samples_delegates: Arc::new(Mutex::new(Vec::new())),
                use_precise_sampling: true,
            }
        }

        /// Synchronously collect current stack sample in all profilers attached to
        /// the |isolate|. The call does not affect number of ticks recorded for
        /// the current top node.
        /// |trace_id| is an optional identifier set to the collected sample.
        /// this is useful to associate the sample with a trace event.
        pub fn collect_sample(isolate: *mut v8::Isolate, trace_id: Option<u64>) {
            // Placeholder implementation
            todo!("Implement CollectSample");
        }

        /// Disposes the CPU profiler object.
        pub fn dispose(self) {
            // Placeholder implementation
            //drop self here to deallocate
        }

        /// Changes default CPU profiler sampling interval to the specified number
        /// of microseconds. Default interval is 1000us. This method must be called
        /// when there are no profiles being recorded.
        pub fn set_sampling_interval(&mut self, us: i32) {
            //Check that there are no running profiles
            if self.profiles.lock().unwrap().is_empty() {
                self.sampling_interval_us = us;
            }
        }

        /// Sets whether or not the profiler should prioritize consistency of sample
        /// periodicity on Windows. Disabling this can greatly reduce CPU usage, but
        /// may result in greater variance in sample timings from the platform's
        /// scheduler. Defaults to enabled. This method must be called when there are
        /// no profiles being recorded.
        pub fn set_use_precise_sampling(&mut self, use_precise_sampling: bool) {
            if self.profiles.lock().unwrap().is_empty() {
                self.use_precise_sampling = use_precise_sampling;
            }
        }

        /// Starts collecting a CPU profile. Several profiles may be collected at once.
        /// Generates an anonymous profiler, without a String identifier.
        pub fn start(
            &self,
            options: CpuProfilingOptions,
            delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
        ) -> CpuProfilingResult {
            // Generate a random title, then start a profile by that title
            let random_title: String = uuid::Uuid::new_v4().to_string();
            self.start_with_title(random_title.into(), options, delegate)
        }

        /// Starts collecting a CPU profile. Title may be an empty string. Several
        /// profiles may be collected at once. Attempts to start collecting several
        /// profiles with the same title are silently ignored.
        pub fn start_with_title(
            &self,
            title: StringWrapper,
            options: CpuProfilingOptions,
            delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
        ) -> CpuProfilingResult {
            let mut profiles = self.profiles.lock().unwrap();

            // Check if a profile with the same title already exists
            for profile in profiles.iter() {
                if profile.title.get_str() == title.get_str() {
                    return CpuProfilingResult {
                        id: 0,
                        status: CpuProfilingStatus::kAlreadyStarted,
                    };
                }
            }

            //Generate the profile id
            let id = profiles.len() as v8::ProfilerId + 1;

            // Create a new CPU profile
            let profile = Box::new(CpuProfile {
                title,
                top_down_root: Arc::new(CpuProfileNode {
                    function_name: StringWrapper::from("root"),
                    script_id: 0,
                    script_resource_name: StringWrapper::from(""),
                    script_shared_cross_origin: false,
                    line_number: 0,
                    column_number: 0,
                    hit_line_count: 0,
                    line_ticks: Arc::new(Mutex::new(Vec::new())),
                    bailout_reason: None,
                    hit_count: 0,
                    node_id: 0,
                    source_type: SourceType::Internal,
                    children: Vec::new(),
                    parent: None,
                    deopt_infos: Vec::new(),
                }),
                samples: Vec::new(),
                sample_timestamps: Vec::new(),
                sample_states: Vec::new(),
                sample_embedder_states: Vec::new(),
                start_time: 0, // Implement timestamp
                end_time: 0,   // Implement timestamp
            });

            //Add the profile to profiles
            profiles.push(profile);

            // If a delegate was passed, set the profile_id on that delegate
            if let Some(mut delegate) = delegate {
                //Store the delegate
                let mut delegates = self.discarded_samples_delegates.lock().unwrap();
                delegates.push(delegate);
            }

            CpuProfilingResult {
                id,
                status: CpuProfilingStatus::kStarted,
            }
        }

        /// Starts profiling with the same semantics as above, except with expanded
        /// parameters.
        ///
        /// |record_samples| parameter controls whether individual samples should
        /// be recorded in addition to the aggregated tree.
        ///
        /// |max_samples| controls the maximum number of samples that should be
        /// recorded by the profiler. Samples obtained after this limit will be
        /// discarded.
        pub fn start_with_expanded_parameters(
            &self,
            title: StringWrapper,
            mode: CpuProfilingMode,
            record_samples: bool,
            max_samples: u32,
        ) -> CpuProfilingResult {
            self.start_with_title(
                title,
                CpuProfilingOptions {
                    mode,
                    max_samples,
                    sampling_interval_us: 0,
                    filter_context: None,
                },
                None,
            )
        }

        /// The same as StartProfiling above, but the CpuProfilingMode defaults to
        /// kLeafNodeLineNumbers mode, which was the previous default behavior of the
        /// profiler.
        pub fn start_with_record_samples(
            &self,
            title: StringWrapper,
            record_samples: bool,
        ) -> CpuProfilingResult {
            self.start_with_title(
                title,
                CpuProfilingOptions {
                    mode: CpuProfilingMode::LeafNodeLineNumbers,
                    max_samples: CpuProfilingOptions::K_NO_SAMPLE_LIMIT,
                    sampling_interval_us: 0,
                    filter_context: None,
                },
                None,
            )
        }

        /// Starts collecting a CPU profile. Title may be an empty string. Several
        /// profiles may be collected at once. Attempts to start collecting several
        /// profiles with the same title are silently ignored.
        pub fn start_profiling_with_options(
            &self,
            title: StringWrapper,
            options: CpuProfilingOptions,
            delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
        ) -> CpuProfilingStatus {
            let result = self.start_with_title(title, options, delegate);
            result.status
        }

        /// Starts profiling with the same semantics as above, except with expanded
        /// parameters.
        ///
        /// |record_samples| parameter controls whether individual samples should
        /// be recorded in addition to the aggregated tree.
        ///
        /// |max_samples| controls the maximum number of samples that should be
        /// recorded by the profiler. Samples obtained after this limit will be
        /// discarded.
        pub fn start_profiling_with_expanded_parameters(
            &self,
            title: StringWrapper,
            mode: CpuProfilingMode,
            record_samples: bool,
            max_samples: u32,
        ) -> CpuProfilingStatus {
            let result = self.start_with_expanded_parameters(title, mode, record_samples, max_samples);
            result.status
        }

        /// The same as StartProfiling above, but the CpuProfilingMode defaults to
        /// kLeafNodeLineNumbers mode, which was the previous default behavior of the
        /// profiler.
        pub fn start_profiling_with_record_samples(
            &self,
            title: StringWrapper,
            record_samples: bool,
        ) -> CpuProfilingStatus {
            let result = self.start_with_record_samples(title, record_samples);
            result.status
        }

        /// Stops collecting CPU profile with a given id and returns it.
        pub fn stop(&self, id: v8::ProfilerId) -> Option<Box<CpuProfile>> {
            let mut profiles = self.profiles.lock().unwrap();
            // Find the profile with the given id
            if let Some(index) = profiles.iter().position(|profile| {
                //Get the profile id from the title StringWrapper
                let profile_id = {
                    //Try to parse the id from the string
                    let title = profile.title.get_str();

                    title.parse::<v8::ProfilerId>().unwrap_or(0)
                };

                profile_id == id
            }) {
                //Remove the profile
                let profile = profiles.remove(index);
                //Return the profile
                Some(profile)
            } else {
                //Profile not found
                None
            }
        }

        /// Stops collecting CPU profile with a given title and returns it.
        /// If the title given is empty, finishes the last profile started.
        pub fn stop_profiling(&self, title: StringWrapper) -> Option<Box<CpuProfile>> {
            let mut profiles = self.profiles.lock().unwrap();

            if title.get_str().is_empty() {
                // If the title is empty, stop the last profile started.
                profiles.pop()
            } else {
                // Find the profile with the given title
                if let Some(index) = profiles.iter().position(|profile| profile.title.get_str() == title.get_str()) {
                    //Remove the profile
                    let profile = profiles.remove(index);
                    //Return the profile
                    Some(profile)
                } else {
                    //Profile not found
                    None
                }
            }
        }

        /// Generate more detailed source positions to code objects. This results in
        
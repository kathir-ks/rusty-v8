// Converted from V8 C++ source files:
// Header: v8-profiler.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::Vec;
use std::{fmt, result};

use libc::c_char;

use crate::v8::{
    internal, Context, Data, Isolate, Local, Message, Name, Object, PropertyAttribute, String,
    StringView, Value,
};
use crate::v8_cppgc::Heap;

pub type NativeObject = *mut std::ffi::c_void;
pub type SnapshotObjectId = u32;
pub type ProfilerId = u32;

#[derive(Debug, Clone)]
pub struct CpuProfileDeoptFrame {
    pub script_id: i32,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct CpuProfileDeoptInfo {
    pub deopt_reason: *const c_char,
    pub stack: Vec<CpuProfileDeoptFrame>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    kScript = 0,
    kBuiltin = 1,
    kCallback = 2,
    kInternal = 3,
    kUnresolved = 4,
}

pub struct CpuProfileNode {
    function_name: String,
    function_name_str: String,
    script_id: i32,
    script_resource_name: String,
    script_resource_name_str: String,
    is_script_shared_cross_origin: bool,
    line_number: i32,
    column_number: i32,
    hit_line_count: u32,
    line_ticks: Vec<LineTick>,
    bailout_reason: String,
    hit_count: u32,
    node_id: u32,
    source_type: SourceType,
    children: Vec<Arc<CpuProfileNode>>,
    parent: Option<Arc<CpuProfileNode>>,
    deopt_infos: Vec<CpuProfileDeoptInfo>,
}

impl CpuProfileNode {
    pub fn get_function_name(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_function_name_str(&self) -> *const c_char {
        self.function_name_str.as_ptr() as *const c_char
    }

    pub fn get_script_id(&self) -> i32 {
        self.script_id
    }

    pub fn get_script_resource_name(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_script_resource_name_str(&self) -> *const c_char {
        self.script_resource_name_str.as_ptr() as *const c_char
    }

    pub fn is_script_shared_cross_origin(&self) -> bool {
        self.is_script_shared_cross_origin
    }

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }

    pub fn get_column_number(&self) -> i32 {
        self.column_number
    }

    pub fn get_hit_line_count(&self) -> u32 {
        self.hit_line_count
    }

    pub fn get_line_ticks(&self, entries: &mut [LineTick]) -> bool {
        if entries.len() < self.line_ticks.len() {
            return false;
        }

        for (i, &tick) in self.line_ticks.iter().enumerate() {
            entries[i] = tick;
        }

        true
    }

    pub fn get_bailout_reason(&self) -> *const c_char {
        self.bailout_reason.as_ptr() as *const c_char
    }

    pub fn get_hit_count(&self) -> u32 {
        self.hit_count
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    pub fn get_source_type(&self) -> SourceType {
        self.source_type
    }

    pub fn get_children_count(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, index: usize) -> Option<&CpuProfileNode> {
        self.children.get(index).map(|arc| Arc::as_ref(arc))
    }

    pub fn get_parent(&self) -> Option<&CpuProfileNode> {
        self.parent.as_ref().map(|arc| Arc::as_ref(arc))
    }

    pub fn get_deopt_infos(&self) -> &Vec<CpuProfileDeoptInfo> {
        &self.deopt_infos
    }

    pub const K_NO_LINE_NUMBER_INFO: i32 = Message::K_NO_LINE_NUMBER_INFO;
    pub const K_NO_COLUMN_NUMBER_INFO: i32 = Message::K_NO_COLUMN_INFO;
}

impl Default for CpuProfileNode {
    fn default() -> Self {
        Self {
            function_name: String::new(),
            function_name_str: String::new(),
            script_id: 0,
            script_resource_name: String::new(),
            script_resource_name_str: String::new(),
            is_script_shared_cross_origin: false,
            line_number: 0,
            column_number: 0,
            hit_line_count: 0,
            line_ticks: Vec::new(),
            bailout_reason: String::new(),
            hit_count: 0,
            node_id: 0,
            source_type: SourceType::kScript,
            children: Vec::new(),
            parent: None,
            deopt_infos: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LineTick {
    pub line: i32,
    pub hit_count: u32,
}

pub trait OutputStream {
    fn end_of_stream(&mut self) -> Result<(), String>;
    fn get_chunk_size(&self) -> usize {
        1024
    }
    fn write_ascii_chunk(&mut self, data: *mut c_char, size: usize) -> WriteResult;
    fn write_heap_stats_chunk(&mut self, data: *mut HeapStatsUpdate, count: usize) -> WriteResult {
        WriteResult::kAbort
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WriteResult {
    kContinue = 0,
    kAbort = 1,
}

pub struct CpuProfile {
    title: String,
    top_down_root: Arc<CpuProfileNode>,
    samples: Vec<Arc<CpuProfileNode>>,
    sample_timestamps: Vec<i64>,
    start_time: i64,
    end_time: i64,
    sample_states: Vec<StateTag>,
    sample_embedder_states: Vec<EmbedderStateTag>,
}

impl CpuProfile {
    pub fn get_title(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_top_down_root(&self) -> &CpuProfileNode {
        Arc::as_ref(&self.top_down_root)
    }

    pub fn get_samples_count(&self) -> usize {
        self.samples.len()
    }

    pub fn get_sample(&self, index: usize) -> Option<&CpuProfileNode> {
        self.samples.get(index).map(|arc| Arc::as_ref(arc))
    }

    pub fn get_sample_timestamp(&self, index: usize) -> i64 {
        self.sample_timestamps[index]
    }

    pub fn get_start_time(&self) -> i64 {
        self.start_time
    }

    pub fn get_sample_state(&self, index: usize) -> StateTag {
        self.sample_states[index]
    }

    pub fn get_sample_embedder_state(&self, index: usize) -> EmbedderStateTag {
        self.sample_embedder_states[index]
    }

    pub fn get_end_time(&self) -> i64 {
        self.end_time
    }

    pub fn delete(&mut self) {}

    pub fn serialize(&self, stream: &mut dyn OutputStream, format: SerializationFormat) {
        match format {
            SerializationFormat::kJSON => {
            }
        }
    }
}

impl Default for CpuProfile {
    fn default() -> Self {
        Self {
            title: String::new(),
            top_down_root: Arc::new(CpuProfileNode::default()),
            samples: Vec::new(),
            sample_timestamps: Vec::new(),
            start_time: 0,
            end_time: 0,
            sample_states: Vec::new(),
            sample_embedder_states: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SerializationFormat {
    kJSON = 0,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuProfilingMode {
    kLeafNodeLineNumbers,
    kCallerLineNumbers,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuProfilingNamingMode {
    kStandardNaming,
    kDebugNaming,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuProfilingLoggingMode {
    kLazyLogging,
    kEagerLogging,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuProfilingStatus {
    kStarted,
    kAlreadyStarted,
    kErrorTooManyProfilers,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CpuProfilingResult {
    pub id: ProfilerId,
    pub status: CpuProfilingStatus,
}

pub trait DiscardedSamplesDelegate {
    fn notify(&mut self);
    fn get_id(&self) -> ProfilerId;
}

pub struct CpuProfilingOptions {
    mode: CpuProfilingMode,
    max_samples: u32,
    sampling_interval_us: i32,
    filter_context: Option<Global<Context>>,
}

impl CpuProfilingOptions {
    pub const K_NO_SAMPLE_LIMIT: u32 = u32::MAX;

    pub fn new(
        mode: CpuProfilingMode,
        max_samples: u32,
        sampling_interval_us: i32,
        filter_context: Option<Local<Context>>,
    ) -> Self {
        Self {
            mode,
            max_samples,
            sampling_interval_us,
            filter_context: filter_context.map(Global::new),
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

    pub fn has_filter_context(&self) -> bool {
        self.filter_context.is_some()
    }

    pub fn raw_filter_context(&self) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
}

impl Default for CpuProfilingOptions {
    fn default() -> Self {
        Self {
            mode: CpuProfilingMode::kLeafNodeLineNumbers,
            max_samples: CpuProfilingOptions::K_NO_SAMPLE_LIMIT,
            sampling_interval_us: 0,
            filter_context: None,
        }
    }
}

pub struct CpuProfiler {
    isolate: *mut Isolate,
    naming_mode: CpuProfilingNamingMode,
    logging_mode: CpuProfilingLoggingMode,
    profiles: Mutex<Vec<(String, CpuProfile)>>,
    sampling_interval_us: Mutex<i32>,
    use_precise_sampling: Mutex<bool>,
    profiler_id_counter: Mutex<ProfilerId>,
}

impl CpuProfiler {
    pub fn new(
        isolate: *mut Isolate,
        naming_mode: CpuProfilingNamingMode,
        logging_mode: CpuProfilingLoggingMode,
    ) -> Self {
        CpuProfiler {
            isolate,
            naming_mode,
            logging_mode,
            profiles: Mutex::new(Vec::new()),
            sampling_interval_us: Mutex::new(1000),
            use_precise_sampling: Mutex::new(true),
            profiler_id_counter: Mutex::new(1),
        }
    }

    pub fn collect_sample(isolate: *mut Isolate, trace_id: Option<u64>) {}

    pub fn dispose(&mut self) {}

    pub fn set_sampling_interval(&self, us: i32) {
        let mut interval = self.sampling_interval_us.lock().unwrap();
        *interval = us;
    }

    pub fn set_use_precise_sampling(&self, use_precise_sampling: bool) {
        let mut precise_sampling = self.use_precise_sampling.lock().unwrap();
        *precise_sampling = use_precise_sampling;
    }

    fn generate_profiler_id(&self) -> ProfilerId {
        let mut id_counter = self.profiler_id_counter.lock().unwrap();
        let id = *id_counter;
        *id_counter += 1;
        id
    }

    pub fn start(
        &self,
        options: CpuProfilingOptions,
        delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
    ) -> CpuProfilingResult {
        let id = self.generate_profiler_id();
        CpuProfilingResult {
            id,
            status: CpuProfilingStatus::kStarted,
        }
    }

    pub fn start_with_title(
        &self,
        title: Local<String>,
        options: CpuProfilingOptions,
        delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
    ) -> CpuProfilingResult {
        let id = self.generate_profiler_id();
        CpuProfilingResult {
            id,
            status: CpuProfilingStatus::kStarted,
        }
    }

    pub fn start_expanded(
        &self,
        title: Local<String>,
        mode: CpuProfilingMode,
        record_samples: bool,
        max_samples: u32,
    ) -> CpuProfilingResult {
        let id = self.generate_profiler_id();
        CpuProfilingResult {
            id,
            status: CpuProfilingStatus::kStarted,
        }
    }

    pub fn start_simple(&self, title: Local<String>, record_samples: bool) -> CpuProfilingResult {
        let id = self.generate_profiler_id();
        CpuProfilingResult {
            id,
            status: CpuProfilingStatus::kStarted,
        }
    }

    pub fn start_profiling(
        &self,
        title: Local<String>,
        options: CpuProfilingOptions,
        delegate: Option<Box<dyn DiscardedSamplesDelegate>>,
    ) -> CpuProfilingStatus {
        CpuProfilingStatus::kStarted
    }

    pub fn start_profiling_expanded(
        &self,
        title: Local<String>,
        mode: CpuProfilingMode,
        record_samples: bool,
        max_samples: u32,
    ) -> CpuProfilingStatus {
        CpuProfilingStatus::kStarted
    }

    pub fn start_profiling_simple(
        &self,
        title: Local<String>,
        record_samples: bool,
    ) -> CpuProfilingStatus {
        CpuProfilingStatus::kStarted
    }

    pub fn stop(&self, id: ProfilerId) -> Option<CpuProfile> {
        None
    }

    pub fn stop_profiling(&self, title: Local<String>) -> Option<CpuProfile> {
        None
    }

    pub fn use_detailed_source_positions_for_profiling(isolate: *mut Isolate) {}
}

impl Default for CpuProfiler {
    fn default() -> Self {
        Self::new(
            std::ptr::null_mut(),
            CpuProfilingNamingMode::kDebugNaming,
            CpuProfilingLoggingMode::kLazyLogging,
        )
    }
}

pub struct HeapGraphEdge {
    edge_type: EdgeType,
    name: Local<Value>,
    from_node: *const HeapGraphNode,
    to_node: *const HeapGraphNode,
}

impl HeapGraphEdge {
    pub fn get_type(&self) -> EdgeType {
        self.edge_type
    }

    pub fn get_name(&self) -> Local<Value> {
        self.name
    }

    pub fn get_from_node(&self) -> *const HeapGraphNode {
        self.from_node
    }

    pub fn get_to_node(&self) -> *const HeapGraphNode {
        self.to_node
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeType {
    kContextVariable = 0,
    kElement = 1,
    kProperty = 2,
    kInternal = 3,
    kHidden = 4,
    kShortcut = 5,
    kWeak = 6,
}

pub struct HeapGraphNode {
    node_type: NodeType,
    name: String,
    id: SnapshotObjectId,
    shallow_size: usize,
    children: Vec<HeapGraphEdge>,
}

impl HeapGraphNode {
    pub fn get_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_name(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_id(&self) -> SnapshotObjectId {
        self.id
    }

    pub fn get_shallow_size(&self) -> usize {
        self.shallow_size
    }

    pub fn get_children_count(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, index: usize) -> Option<&HeapGraphEdge> {
        self.children.get(index)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    kHidden = 0,
    kArray = 1,
    kString = 2,
    kObject = 3,
    kCode = 4,
    kClosure = 5,
    kRegExp = 6,
    kHeapNumber = 7,
    kNative = 8,
    kSynthetic = 9,
    kConsString = 10,
    kSlicedString = 11,
    kSymbol = 12,
    kBigInt = 13,
    kObjectShape = 14,
}

pub struct HeapSnapshot {
    root: HeapGraphNode,
    nodes: Vec<HeapGraphNode>,
    max_snapshot_js_object_id: SnapshotObjectId,
}

impl HeapSnapshot {
    pub fn get_root(&self) -> &HeapGraphNode {
        &self.root
    }

    pub fn get_node_by_id(&self, id: SnapshotObjectId) -> Option<&HeapGraphNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn get_nodes_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, index: usize) -> Option<&HeapGraphNode> {
        self.nodes.get(index)
    }

    pub fn get_max_snapshot_js_object_id(&self) -> SnapshotObjectId {
        self.max_snapshot_js_object_id
    }

    pub fn delete(&mut self) {}

    pub fn serialize(&self, stream: &mut dyn OutputStream, format: SerializationFormat) {
        match format {
            SerializationFormat::kJSON => {
            }
        }
    }
}

pub trait ActivityControl {
    fn report_progress_value(&mut self, done: u32, total: u32) -> ControlOption;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ControlOption {
    kContinue = 0,
    kAbort = 1,
}

pub struct AllocationProfile {
    root_node: Node,
    samples: Vec<Sample>,
}

impl AllocationProfile {
    pub const K_NO_LINE_NUMBER_INFO: i32 = Message::K_NO_LINE_NUMBER_INFO;
    pub const K_NO_COLUMN_NUMBER_INFO: i32 = Message::K_NO_COLUMN_INFO;

    pub fn get_root_node(&mut self) -> &mut Node {
        &mut self.root_node
    }
    pub fn get_samples(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }
}

pub struct Node {
    pub name: String,
    pub script_name: String,
    pub script_id: i32,
    pub start_position: i32,
    pub line_number: i32,
    pub column_number: i32,
    pub node_id: u32,
    pub children: Vec<Box<Node>>,
    pub allocations: Vec<Allocation>,
}

pub struct Allocation {
    pub size: usize,
    pub count: u32,
}

pub struct Sample {
    pub node_id: u32,
    pub size: usize,
    pub count: u32,
    pub sample_id: u64,
}

pub struct EmbedderGraph {
    nodes: Vec<Box<dyn NodeTrait>>,
    edges: Vec<(usize, usize, Option<String>)>,
    native_size: usize,
}

impl EmbedderGraph {
    pub fn v8_node(&mut self, value: &Local<Value>) -> &mut dyn NodeTrait {
        todo!()
    }

    pub fn add_node(&mut self, node: Box<dyn NodeTrait>) -> &mut dyn NodeTrait {
        self.nodes.push(node);
        self.nodes.last_mut().unwrap().as_mut()
    }

    pub fn add_edge(&mut self, from: &mut dyn NodeTrait, to: &mut dyn NodeTrait, name: Option<&str>) {
        let from_index = self
            .nodes
            .iter()
            .position(|n| n.as_ref() as *const dyn NodeTrait == from as *const dyn NodeTrait)
            .unwrap();
        let to_index = self
            .nodes
            .iter()
            .position(|n| n.as_ref() as *const dyn NodeTrait == to as *const dyn NodeTrait)
            .unwrap();

        self.edges.push((
            from_index,
            to_index,
            name.map(|s| s.to_string()),
        ));
    }

    pub fn add_native_size(&mut self, size: usize) {
        self.native_size += size;
    }
}

pub trait NodeTrait {
    fn name(&self) -> &str;
    fn size_in_bytes(&self) -> usize;
    fn wrapper_node(&mut self) -> Option<&mut dyn NodeTrait> {
        None
    }
    fn is_root_node(&self) -> bool {
        false
    }
    fn is_embedder_node(&self) -> bool {
        true
    }
    fn name_prefix(&self) -> Option<&str> {
        None
    }
    fn native_object(&mut self) -> Option<NativeObject> {
        None
    }
    fn detachedness(&self) -> Detachedness {
        Detachedness::kUnknown
    }
    fn address(&self) -> *const std::ffi::c_void {
        std::ptr::null()
    }
}

pub enum Detachedness {
    kUnknown = 0,
    kAttached = 1,
    kDetached = 2,
}

pub trait QueryObjectPredicate {
    fn filter(&mut self, object: &Local<Object>) -> bool;
}

pub struct HeapProfiler {
    snapshots: Mutex<Vec<HeapSnapshot>>,
    build_embedder_graph_callback:
        Mutex<Option<(*mut Isolate, *mut EmbedderGraph, *mut std::ffi::c_void)>>,
    build_embedder_graph_callbacks: Mutex<Vec<BuildEmbedderGraphCallbackEntry>>,
    get_detachedness_callback:
        Mutex<Option<(*mut Isolate, *mut Value, u16, *mut std::ffi::c_void)>>,
    get_detachedness_callbacks: Mutex<Vec<GetDetachednessCallbackEntry>>,
    object_id_map: Mutex<std::collections::HashMap<usize, SnapshotObjectId>>,
    next_object_id: Mutex<SnapshotObjectId>,
}

struct BuildEmbedderGraphCallbackEntry {
    callback: BuildEmbedderGraphCallback,
    data: *mut std::ffi::c_void,
}

struct GetDetachednessCallbackEntry {
    callback: GetDetachednessCallback,
    data: *mut std::ffi::c_void,
}

type BuildEmbedderGraphCallback =
    extern "C" fn(*mut Isolate, *mut EmbedderGraph, *mut std::ffi::c_void);
type GetDetachednessCallback = extern "C" fn(
    *mut Isolate,
    *mut Value,
    u16,
    *mut std::ffi::c_void,
) -> EmbedderGraph::Node::Detachedness;

impl HeapProfiler {
    pub const K_UNKNOWN_OBJECT_ID: SnapshotObjectId = 0;
    pub const K_PERSISTENT_HANDLE_NO_CLASS_ID: u16 = 0;

    pub fn query_objects(
        &self,
        context: &Local<Context>,
        predicate: &mut dyn QueryObjectPredicate,
        objects: &mut Vec<Global<Object>>,
    ) {
    }

    pub fn get_snapshot_count(&self) -> usize {
        let snapshots = self.snapshots.lock().unwrap();
        snapshots.len()
    }

    pub fn get_heap_snapshot(&self, index: usize) -> Option<&HeapSnapshot> {
        let snapshots = self.snapshots.lock().unwrap();
        snapshots.get(index)
    }

    pub fn get_object_id(&self, value: &Local<Value>) -> SnapshotObjectId {
        Self::K_UNKNOWN_OBJECT_ID
    }

    pub fn get_object_id_native(&self, value: NativeObject) -> SnapshotObjectId {
        Self::K_UNKNOWN_OBJECT_ID
    }

    pub fn find_object_by_id(&self, id: SnapshotObjectId) -> Local<Value> {
        Local::new()
    }

    pub fn clear_object_ids(&self) {}

    pub fn take_heap_snapshot_with_options(
        &self,
        options: &HeapSnapshotOptions,
    ) -> Option<HeapSnapshot> {
        None
    }

    pub fn take_heap_snapshot(
        &self,
        control: &mut dyn ActivityControl,
        global_object_name_resolver: Option<&ObjectNameResolver>,
        hide_internals: bool,
        capture_numeric_value: bool,
    ) -> Option<HeapSnapshot> {
        None
    }

    pub fn get_detached_js_wrapper_objects(&self) -> Vec<Local<Value>> {
        Vec::new()
    }

    pub fn start_tracking_heap_objects(&self, track_allocations: bool) {}

    pub fn get_heap_stats(&self, stream: &mut dyn OutputStream, timestamp_us: Option<&mut i64>) -> SnapshotObjectId {
        Self::K_UNKNOWN_OBJECT_ID
    }

    pub fn stop_tracking_heap_objects(&self) {}

    pub fn start_sampling_heap_profiler(
        &self,
        sample_interval: u64,
        stack_depth: i32,
        flags: SamplingFlags,
    ) -> bool {
        false
    }

    pub fn stop_sampling_heap_profiler(&self) {}

    pub fn get_allocation_profile(&self) -> Option<AllocationProfile> {
        None
    }

    pub fn delete_all_heap_snapshots(&self) {}

    pub fn add_build_embedder_graph_callback(
        &self,
        callback: BuildEmbedderGraphCallback,
        data: *mut std::ffi::c_void,
    ) {
    }

    pub fn remove_build_embedder_graph_callback(
        &self,
        callback: BuildEmbedderGraphCallback,
        data: *mut std::ffi::c_void,
    ) {
    }

    pub fn set_get_detachedness_callback(
        &self,
        callback: GetDetachednessCallback,
        data: *mut std::ffi::c_void,
    ) {
    }

    pub fn is_taking_snapshot(&self) -> bool {
        false
    }

    pub fn copy_name_for_heap_snapshot(&self, name: &str) -> *const c_char {
        name.as_ptr() as *const c_char
    }
}

impl Default for HeapProfiler {
    fn default() -> Self {
        Self {
            snapshots: Mutex::new(Vec::new()),
            build_embedder_graph_callback: Mutex::new(None),
            build_embedder_graph_callbacks: Mutex::new(Vec::new()),
            get_detachedness_callback: Mutex::new(None),
            get_detachedness_callbacks: Mutex::new(Vec::new()),
            object_id_map: Mutex::new(std::collections::HashMap::new()),
            next_object_id: Mutex::new(1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SamplingFlags {
    kSamplingNoFlags = 0,
    kSamplingForceGC = 1 << 0,
    kSamplingIncludeObjectsCollectedByMajorGC = 1 << 1,
    kSamplingIncludeObjectsCollectedByMinorGC = 1 << 2,
}

pub trait ObjectNameResolver {
    fn get_name(&self, object: &Local<Object>) -> *const c_char;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HeapSnapshotMode {
    kRegular,
    kExposeInternals,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumericsMode {
    kHideNumericValues,
    kExposeNumericValues,
}

pub struct HeapSnapshotOptions {
    pub control: *mut dyn ActivityControl,
    pub global_object_name_resolver: *mut dyn ObjectNameResolver,
    pub snapshot_mode: HeapSnapshotMode,
    pub numerics_mode: NumericsMode,
    pub stack_state: cppgc::EmbedderStackState,
}

impl Default for HeapSnapshotOptions {
    fn default() -> Self {
        Self {
            control: std::ptr::null_mut(),
            global_object_name_resolver: std::ptr::null_mut(),
            snapshot_mode: HeapSnapshotMode::kRegular,
            numerics_mode: NumericsMode::kHideNumericValues,
            stack_state: cppgc::EmbedderStackState::kMayContainHeapPointers,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HeapStatsUpdate {
    pub index: u32,
    pub count: u32,
    pub size: u32,
}

impl HeapStatsUpdate {
    pub fn new(index: u32, count: u32, size: u32) -> Self {
        Self { index, count, size }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodeEventType {
    kUnknownType = 0,
    kBuiltinType,
    kCallbackType,
    kEvalType,
    kFunctionType,
    kInterpretedFunctionType,
    kHandlerType,
    kBytecodeHandlerType,
    kRegExpType,
    kScriptType,
    kStubType,
    kRelocationType,
}

pub struct CodeEvent {
    code_start_address: usize,
    code_size: usize,
    function_name: String,
    script_name: String,
    script_line: i32,
    script_column: i32,
    code_type: CodeEventType,
    comment: String,
    previous_code_start_address: usize,
}

impl CodeEvent {
    pub fn get_code_start_address(&self) -> usize {
        self.code_start_address
    }

    pub fn get_code_size(&self) -> usize {
        self.code_size
    }

    pub fn get_function_name(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_script_name(&self) -> Local<String> {
        Local::new()
    }

    pub fn get_script_line(&self) -> i32 {
        self.script_line
    }

    pub fn get_script_column(&self) -> i32 {
        self.script_column
    }

    pub fn get_code_type(&self) -> CodeEventType {
        self.code_type
    }

    pub fn get_comment(&self) -> *const c_char {
        self.comment.as_ptr() as *const c_char
    }

    pub fn get_code_event_type_name(code_event_type: CodeEventType) -> &'static str {
        match code_event_type {
            CodeEventType::kUnknownType => "Unknown",
            CodeEventType::kBuiltinType => "Builtin",
            CodeEventType::kCallbackType => "Callback",
            CodeEventType::kEvalType => "Eval",
            CodeEventType::kFunctionType => "Function",
            CodeEventType::kInterpretedFunctionType => "InterpretedFunction",
            CodeEventType::kHandler

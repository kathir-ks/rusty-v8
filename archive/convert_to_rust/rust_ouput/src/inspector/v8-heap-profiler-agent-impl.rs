// Converted from V8 C++ source files:
// Header: v8-heap-profiler-agent-impl.h
// Implementation: v8-heap-profiler-agent-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Duration;
use std::{collections::HashMap, result};

use crate::parsing::pending_compilation_error_handler::Isolate;
//use v8::OutputStream;

//use crate::v8::OutputStream;

pub struct Response {
    pub is_success: bool,
    pub error_message: Option<String>,
}

impl Response {
    pub fn Success() -> Response {
        Response {
            is_success: true,
            error_message: None,
        }
    }

    pub fn ServerError(message: &str) -> Response {
        Response {
            is_success: false,
            error_message: Some(message.to_string()),
        }
    }

    pub fn InternalError() -> Response {
        Response {
            is_success: false,
            error_message: Some("Internal error".to_string()),
        }
    }

    pub fn IsSuccess(&self) -> bool {
        self.is_success
    }
}

pub struct String16 {
    pub data: Vec<u16>,
}

impl String16 {
    pub fn from_str(s: &str) -> String16 {
        String16 {
            data: s.encode_utf16().collect(),
        }
    }

    pub fn fromInteger(i: usize) -> String16 {
        String16::from_str(&i.to_string())
    }

    pub fn toInteger(&self, ok: &mut bool) -> i32 {
        match String::from_utf16(&self.data) {
            Ok(s) => {
                if let Ok(num) = s.parse::<i32>() {
                    *ok = true;
                    num
                } else {
                    *ok = false;
                    0
                }
            }
            Err(_) => {
                *ok = false;
                0
            }
        }
    }
    pub fn length(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn push(&mut self, c: u16) {
        self.data.push(c);
    }
}

impl From<&str> for String16 {
    fn from(s: &str) -> Self {
        String16 {
            data: s.encode_utf16().collect(),
        }
    }
}

impl From<String> for String16 {
    fn from(s: String) -> Self {
        String16 {
            data: s.encode_utf16().collect(),
        }
    }
}

// Dummy implementations
pub struct V8InspectorSessionImpl {}

impl V8InspectorSessionImpl {
    pub fn inspector(&self) -> &V8InspectorImpl {
        todo!()
    }
    pub fn contextGroupId(&self) -> i32 {
        todo!()
    }
    pub fn wrapObject(
        &self,
        context: v8::Local<v8::Context>,
        heapObject: v8::Local<v8::Object>,
        objectGroup: String,
        b: bool,
    ) -> Result<Box<protocol::Runtime::RemoteObject>, String> {
        todo!()
    }

    pub fn unwrapObject(
        &self,
        objectId: String16,
        value: &mut v8::Local<v8::Value>,
        context: &mut v8::Local<v8::Context>,
        arg3: *mut (),
    ) -> Response {
        todo!()
    }
    pub fn addInspectedObject(&self, inspectable_heap_object: Box<InspectableHeapObject>) {
        todo!()
    }
}

pub mod protocol {
    pub mod HeapProfiler {
        pub struct SamplingHeapProfileNode {}
        impl SamplingHeapProfileNode {
            pub fn create() -> SamplingHeapProfileNodeCreate {
                SamplingHeapProfileNodeCreate {}
            }
        }
        pub struct SamplingHeapProfileNodeCreate {}
        impl SamplingHeapProfileNodeCreate {
            pub fn setCallFrame(
                self,
                callFrame: std::unique_ptr<super::Runtime::CallFrame>,
            ) -> SamplingHeapProfileNodeCreate {
                todo!()
            }
            pub fn setSelfSize(self, selfSize: usize) -> SamplingHeapProfileNodeCreate {
                todo!()
            }
            pub fn setChildren(
                self,
                children: std::unique_ptr<protocol::Array<SamplingHeapProfileNode>>,
            ) -> SamplingHeapProfileNodeCreate {
                todo!()
            }
            pub fn setId(self, node_id: i32) -> SamplingHeapProfileNodeCreate {
                todo!()
            }
            pub fn build(self) -> std::unique_ptr<SamplingHeapProfileNode> {
                todo!()
            }
        }
        pub struct SamplingHeapProfile {}
        impl SamplingHeapProfile {
            pub fn create() -> SamplingHeapProfileCreate {
                SamplingHeapProfileCreate {}
            }
        }
        pub struct SamplingHeapProfileCreate {}
        impl SamplingHeapProfileCreate {
            pub fn setHead(
                self,
                head: std::unique_ptr<SamplingHeapProfileNode>,
            ) -> SamplingHeapProfileCreate {
                todo!()
            }
            pub fn setSamples(
                self,
                samples: std::unique_ptr<protocol::Array<SamplingHeapProfileSample>>,
            ) -> SamplingHeapProfileCreate {
                todo!()
            }
            pub fn build(self) -> std::unique_ptr<SamplingHeapProfile> {
                todo!()
            }
        }
        pub struct SamplingHeapProfileSample {}
        impl SamplingHeapProfileSample {
            pub fn create() -> SamplingHeapProfileSampleCreate {
                SamplingHeapProfileSampleCreate {}
            }
        }
        pub struct SamplingHeapProfileSampleCreate {}
        impl SamplingHeapProfileSampleCreate {
            pub fn setSize(self, size: usize) -> SamplingHeapProfileSampleCreate {
                todo!()
            }
            pub fn setNodeId(self, node_id: i32) -> SamplingHeapProfileSampleCreate {
                todo!()
            }
            pub fn setOrdinal(self, ordinal: f64) -> SamplingHeapProfileSampleCreate {
                todo!()
            }
            pub fn build(self) -> protocol::SamplingHeapProfileSample {
                todo!()
            }
        }
        pub trait Backend {
            fn collectGarbage(
                &mut self,
                callback: std::unique_ptr<CollectGarbageCallback>,
            );
            fn enable(&mut self) -> Response;
            fn startTrackingHeapObjects(
                &mut self,
                trackAllocations: std::option::Option<bool>,
            ) -> Response;
            fn stopTrackingHeapObjects(
                &mut self,
                reportProgress: std::option::Option<bool>,
                treatGlobalObjectsAsRoots: std::option::Option<bool>,
                captureNumericValue: std::option::Option<bool>,
                exposeInternals: std::option::Option<bool>,
            ) -> Response;
            fn disable(&mut self) -> Response;
            fn takeHeapSnapshot(
                &mut self,
                reportProgress: std::option::Option<bool>,
                treatGlobalObjectsAsRoots: std::option::Option<bool>,
                captureNumericValue: std::option::Option<bool>,
                exposeInternals: std::option::Option<bool>,
                callback: std::unique_ptr<TakeHeapSnapshotCallback>,
            );
            fn getObjectByHeapObjectId(
                &mut self,
                heapSnapshotObjectId: String16,
                objectGroup: std::option::Option<String16>,
                result: &mut std::unique_ptr<super::Runtime::RemoteObject>,
            ) -> Response;
            fn addInspectedHeapObject(&mut self, inspectedHeapObjectId: String16) -> Response;
            fn getHeapObjectId(
                &mut self,
                objectId: String16,
                heapSnapshotObjectId: &mut String16,
            ) -> Response;
            fn startSampling(
                &mut self,
                samplingInterval: std::option::Option<f64>,
                includeObjectsCollectedByMajorGC: std::option::Option<bool>,
                includeObjectsCollectedByMinorGC: std::option::Option<bool>,
            ) -> Response;
            fn stopSampling(
                &mut self,
                profile: &mut std::unique_ptr<protocol::HeapProfiler::SamplingHeapProfile>,
            ) -> Response;
            fn getSamplingProfile(
                &mut self,
                profile: &mut std::unique_ptr<protocol::HeapProfiler::SamplingHeapProfile>,
            ) -> Response;
        }
        pub trait Frontend {
            fn resetProfiles(&self) {}
            fn reportHeapSnapshotProgress(
                &self,
                done: u32,
                total: u32,
                finished: std::option::Option<bool>,
            );
            fn addHeapSnapshotChunk(&self, chunk: String16);
            fn flush(&self);
            fn heapStatsUpdate(&self, statsDiff: std::unique_ptr<Array<i32>>);
            fn lastSeenObjectId(&self, lastSeenObjectId: i32, time: f64);
        }
    }
    pub mod Runtime {
        pub struct RemoteObject {}
        impl RemoteObject {
            pub fn create() -> RemoteObjectBuilder {
                RemoteObjectBuilder {}
            }
        }
        pub struct RemoteObjectBuilder {}
        impl RemoteObjectBuilder {
            pub fn build(self) -> std::unique_ptr<RemoteObject> {
                std::unique_ptr::new(RemoteObject {})
            }
        }
        pub struct CallFrame {}
        impl CallFrame {
            pub fn create() -> CallFrameBuilder {
                CallFrameBuilder {}
            }
        }
        pub struct CallFrameBuilder {}
        impl CallFrameBuilder {
            pub fn setFunctionName(self, functionName: String16) -> CallFrameBuilder {
                todo!()
            }
            pub fn setScriptId(self, scriptId: String16) -> CallFrameBuilder {
                todo!()
            }
            pub fn setUrl(self, url: String16) -> CallFrameBuilder {
                todo!()
            }
            pub fn setLineNumber(self, lineNumber: i32) -> CallFrameBuilder {
                todo!()
            }
            pub fn setColumnNumber(self, columnNumber: i32) -> CallFrameBuilder {
                todo!()
            }
            pub fn build(self) -> std::unique_ptr<CallFrame> {
                todo!()
            }
        }
    }
    pub struct DictionaryValue {
        properties: HashMap<String, PropertyValue>,
    }
    impl DictionaryValue {
        pub fn new() -> DictionaryValue {
            DictionaryValue {
                properties: HashMap::new(),
            }
        }
        pub fn booleanProperty(&self, name: &str, default_value: bool) -> bool {
            match self.properties.get(name) {
                Some(PropertyValue::Boolean(value)) => *value,
                _ => default_value,
            }
        }
        pub fn doubleProperty(&self, name: &str, default_value: f64) -> f64 {
            match self.properties.get(name) {
                Some(PropertyValue::Double(value)) => *value,
                _ => default_value,
            }
        }
        pub fn integerProperty(&self, name: &str, default_value: i32) -> i32 {
            match self.properties.get(name) {
                Some(PropertyValue::Integer(value)) => *value,
                _ => default_value,
            }
        }
        pub fn setBoolean(&mut self, name: &str, value: bool) {
            self.properties.insert(name.to_string(), PropertyValue::Boolean(value));
        }
        pub fn setDouble(&mut self, name: &str, value: f64) {
            self.properties.insert(name.to_string(), PropertyValue::Double(value));
        }
        pub fn setInteger(&mut self, name: &str, value: i32) {
            self.properties.insert(name.to_string(), PropertyValue::Integer(value));
        }
    }
    enum PropertyValue {
        Boolean(bool),
        Double(f64),
        Integer(i32),
    }
    pub struct Array<T> {
        items: Vec<T>,
    }
    impl<T> Array<T> {
        pub fn new() -> Self {
            Array { items: Vec::new() }
        }
        pub fn emplace_back(&mut self, item: T) {
            self.items.push(item);
        }
    }
}

pub mod v8 {
    pub type Local<'a, T> = &'a T;
    pub struct HeapProfiler {}
    impl HeapProfiler {
        pub fn FindObjectById(&self, id: i32) -> Local<Value> {
            todo!()
        }
        pub fn GetObjectId(&self, value: Local<Value>) -> i32 {
            todo!()
        }
        pub fn StartTrackingHeapObjects(&mut self, trackAllocations: bool) {}
        pub fn StopTrackingHeapObjects(&mut self) {}
        pub fn GetHeapStats(&mut self, stream: &HeapStatsStream) -> i32 {
            todo!()
        }
        pub fn ClearObjectIds(&mut self) {}
        pub fn TakeHeapSnapshot(&self, options: HeapSnapshotOptions) -> *const HeapSnapshot {
            todo!()
        }
        pub fn StartSamplingHeapProfiler(
            &mut self,
            samplingIntervalValue: u64,
            i: i32,
            flags: SamplingFlags,
        ) {
            todo!()
        }
        pub fn StopSamplingHeapProfiler(&mut self) {
            todo!()
        }
        pub fn GetAllocationProfile(&mut self) -> *mut AllocationProfile {
            todo!()
        }
    }
    pub struct Value {}
    impl Value {
        pub fn IsEmpty(&self) -> bool {
            todo!()
        }
        pub fn IsObject(&self) -> bool {
            todo!()
        }
        pub fn IsUndefined(&self) -> bool {
            todo!()
        }
        pub fn As<T>(&self) -> &T {
            todo!()
        }
    }
    pub struct Object {}
    impl Object {
        pub fn GetCreationContext(&self, isolate: *mut Isolate) -> Result<Local<Context>, ()> {
            todo!()
        }
    }
    pub struct Context {}
    impl Context {
        pub fn GetIsolate(&self) -> *mut Isolate {
            todo!()
        }
    }
    pub struct HeapSnapshotOptions<'a> {
        pub global_object_name_resolver: &'a GlobalObjectNameResolver,
        pub control: *const HeapSnapshotProgress,
        pub snapshot_mode: HeapSnapshotMode,
        pub numerics_mode: NumericsMode,
        pub stack_state: cppgc::EmbedderStackState,
    }
    pub enum HeapSnapshotMode {
        kExposeInternals,
        kRegular,
    }
    pub enum NumericsMode {
        kExposeNumericValues,
        kHideNumericValues,
    }
    pub struct HeapSnapshot {}
    impl HeapSnapshot {
        pub fn Serialize(&self, stream: &HeapSnapshotOutputStream) {}
        pub fn Delete(&self) {}
    }
    pub struct OutputStream {
        // private:
        //  protocol::HeapProfiler::Frontend* m_frontend;
    }

    pub enum ActivityControlOption {
        kContinue,
    }

    pub trait ActivityControl {
        fn ReportProgressValue(&self, done: u32, total: u32) -> ActivityControlOption;
    }

    pub struct AllocationProfile {}
    impl AllocationProfile {
        pub fn GetRootNode(&mut self) -> &AllocationProfileNode {
            todo!()
        }
        pub fn GetSamples(&mut self) -> &Vec<AllocationProfileSample> {
            todo!()
        }
    }
    pub struct AllocationProfileNode {
        pub children: Vec<&'static AllocationProfileNode>,
        pub allocations: Vec<Allocation>,
        pub name: String16,
        pub script_id: i32,
        pub script_name: String16,
        pub line_number: i32,
        pub column_number: i32,
        pub node_id: i32,
    }
    pub struct Allocation {
        pub size: usize,
        pub count: usize,
    }
    pub struct AllocationProfileSample {
        pub size: usize,
        pub count: usize,
        pub node_id: i32,
        pub sample_id: i32,
    }
    pub enum SamplingFlags {
        kSamplingForceGC = 0,
        kSamplingIncludeObjectsCollectedByMajorGC = 1,
        kSamplingIncludeObjectsCollectedByMinorGC = 2,
    }

}

pub mod debug {
    pub fn ForceGarbageCollection(isolate: *mut Isolate, stackState: StackState) {}
    pub struct Platform {}
    impl Platform {
        pub fn GetForegroundTaskRunner(&self, isolate: *mut Isolate) -> Box<TaskRunner> {
            todo!()
        }
    }
    pub fn GetCurrentPlatform() -> Box<Platform> {
        todo!()
    }
    pub enum StackState {
        kNoHeapPointers,
    }
}

pub struct HeapStatsUpdate {
    pub index: i32,
    pub count: i32,
    pub size: i32,
}

pub mod base {
    pub mod platform {
        use std::thread;
        use std::time::Duration;

        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Mutex {
                Mutex {
                    inner: std::sync::Mutex::new(()),
                }
            }

            pub fn lock(&self) -> MutexGuard {
                MutexGuard {
                    _guard: self.inner.lock().unwrap(),
                }
            }
        }

        pub struct MutexGuard<'a> {
            _guard: std::sync::MutexGuard<'a, ()>,
        }
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct TimeDelta {
            pub milliseconds: i64,
        }

        impl TimeDelta {
            pub fn FromMilliseconds(milliseconds: i64) -> Self {
                TimeDelta { milliseconds }
            }
            pub fn InSecondsF(&self) -> f64 {
                self.milliseconds as f64 / 1000.0
            }
            pub fn from_seconds(seconds: f64) -> Self {
                TimeDelta {
                    milliseconds: (seconds * 1000.0) as i64,
                }
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct TimeTicks {
            pub nanoseconds: i64,
        }
        impl TimeTicks {
            pub fn Now() -> Self {
                let now = std::time::Instant::now();
                TimeTicks {
                    nanoseconds: now.elapsed().as_nanos() as i64,
                }
            }
        }
        impl std::ops::Sub for TimeTicks {
            type Output = TimeDelta;

            fn sub(self, other: Self) -> Self::Output {
                TimeDelta {
                    milliseconds: (self.nanoseconds - other.nanoseconds) / 1_000_000,
                }
            }
        }
    }
}

pub mod cppgc {
    pub enum EmbedderStackState {
        kNoHeapPointers,
        kMayContainHeapPointers,
    }
}

pub trait Task {
    fn Run(&mut self);
}

pub trait TaskRunner {
    fn PostNonNestableTask(&self, task: Box<dyn Task>);
    fn NonNestableTasksEnabled(&self) -> bool;
}

pub struct V8InspectorImpl {}
impl V8InspectorImpl {
    pub fn isolate(&self) -> *mut Isolate {
        todo!()
    }
    pub fn getContext(&self, group_id: i32, context_id: i32) -> *mut InspectedContext {
        todo!()
    }
}

pub struct InspectedContext {}
impl InspectedContext {
    pub fn contextId(context: v8::Local<v8::Context>) -> i32 {
        todo!()
    }
    pub fn origin(&self) -> String16 {
        todo!()
    }
}

pub trait V8InspectorClient {
    fn isInspectableHeapObject(&self, object: v8::Local<v8::Object>) -> bool;
    fn currentTimeMS(&self) -> f64;
    fn startRepeatingTimer(&self, delay: f64, callback: &dyn Fn(usize), data: usize);
    fn cancelTimer(&self, data: usize);
}

pub struct V8Debugger {}
impl V8Debugger {
    pub fn isPaused(&self) -> bool {
        todo!()
    }
}

pub struct TakeHeapSnapshotCallback {
    success: bool,
}

impl TakeHeapSnapshotCallback {
    pub fn sendSuccess(&self) {}
    pub fn sendFailure(&self, response: Response) {}
}

pub struct CollectGarbageCallback {
    success: bool,
}

impl CollectGarbageCallback {
    pub fn sendSuccess(&self) {}
}

pub struct V8HeapProfilerAgentImpl {
    m_session: *mut V8InspectorSessionImpl,
    m_isolate: *mut Isolate,
    m_frontend: Box<dyn protocol::HeapProfiler::Frontend>,
    m_state: *mut protocol::DictionaryValue,
    m_hasTimer: bool,
    m_timerDelayInSeconds: f64,
    m_asyncCallbacks: Rc<AsyncCallbacks>,
}

impl V8HeapProfilerAgentImpl {
    pub fn new(
        session: *mut V8InspectorSessionImpl,
        frontendChannel: Box<dyn protocol::HeapProfiler::Frontend>,
        state: *mut protocol::DictionaryValue,
    ) -> V8HeapProfilerAgentImpl {
        V8HeapProfilerAgentImpl {
            m_session: session,
            m_isolate: (unsafe { (*session).inspector() }).isolate(),
            m_frontend: frontendChannel,
            m_state: state,
            m_hasTimer: false,
            m_timerDelayInSeconds: 0.0,
            m_asyncCallbacks: Rc::new(AsyncCallbacks::new()),
        }
    }

    fn restore(&mut self) {
        let state = unsafe { &*self.m_state };
        if state.booleanProperty(HeapProfilerAgentState::heapProfilerEnabled, false) {
            self.m_frontend.resetProfiles();
        }
        if state.booleanProperty(
            HeapProfilerAgentState::heapObjectsTrackingEnabled,
            false,
        ) {
            self.startTrackingHeapObjectsInternal(state.booleanProperty(
                HeapProfilerAgentState::allocationTrackingEnabled,
                false,
            ));
        }
        if state.booleanProperty(
            HeapProfilerAgentState::samplingHeapProfilerEnabled,
            false,
        ) {
            let samplingInterval = state.doubleProperty(
                HeapProfilerAgentState::samplingHeapProfilerInterval,
                -1.0,
            );
            assert!(samplingInterval >= 0.0);
            let flags = state.integerProperty(
                HeapProfilerAgentState::samplingHeapProfilerFlags,
                0,
            );
            let major_gc =
                flags & v8::SamplingFlags::kSamplingIncludeObjectsCollectedByMajorGC as i32 != 0;
            let minor_gc =
                flags & v8::SamplingFlags::kSamplingIncludeObjectsCollectedByMinorGC as i32 != 0;
            self.startSampling(
                Some(samplingInterval),
                Some(major_gc),
                Some(minor_gc),
            );
        }
    }

    fn collectGarbage(&mut self, callback: std::unique_ptr<CollectGarbageCallback>) {
        let asyncCallbacks = Rc::clone(&self.m_asyncCallbacks);
        let mut lock = asyncCallbacks.m_mutex.lock().unwrap();
        asyncCallbacks.m_gcCallbacks.push(callback);
        drop(lock);

        let isolate = self.m_isolate;
        let asyncCallbacks_clone = Rc::clone(&asyncCallbacks);
        std::thread::spawn(move || {
            let mut lock = asyncCallbacks_clone.m_mutex.lock().unwrap();
            if asyncCallbacks_clone.m_canceled {
                return;
            }
            //v8::debug::ForceGarbageCollection(isolate, v8::debug::StackState::kNoHeapPointers);
            for callback in &asyncCallbacks_clone.m_gcCallbacks {
                callback.sendSuccess();
            }
            asyncCallbacks_clone.m_gcCallbacks.clear();
        });
    }

    fn enable(&mut self) -> Response {
        unsafe {
            (*self.m_state).setBoolean(HeapProfilerAgentState::heapProfilerEnabled, true);
        }
        Response::Success()
    }

    fn startTrackingHeapObjects(&mut self, trackAllocations: std::option::Option<bool>) -> Response {
        unsafe {
            (*self.m_state).setBoolean(HeapProfilerAgentState::heapObjectsTrackingEnabled, true);
            let allocationTrackingEnabled = trackAllocations.unwrap_or(false);
            (*self.m_state).setBoolean(
                HeapProfilerAgentState::allocationTrackingEnabled,
                allocationTrackingEnabled,
            );
            self.startTrackingHeapObjectsInternal(allocationTrackingEnabled);
        }
        Response::Success()
    }

    fn stopTrackingHeapObjects(
        &mut self,
        reportProgress: std::option::Option<bool>,
        treatGlobalObjectsAsRoots: std::option::Option<bool>,
        captureNumericValue: std::option::Option<bool>,
        exposeInternals: std::option::Option<bool>,
    ) -> Response {
        self.requestHeapStatsUpdate();
        self.takeHeapSnapshotNow(
            HeapSnapshotProtocolOptions {
                m_reportProgress: reportProgress.unwrap_or(false),
                m_treatGlobalObjectsAsRoots: treatGlobalObjectsAsRoots.unwrap_or(true),
                m_captureNumericValue: captureNumericValue.unwrap_or(false),
                m_exposeInternals: exposeInternals.unwrap_or(false),
            },
            cppgc::EmbedderStackState::kMayContainHeapPointers,
        );
        self.stopTrackingHeapObjectsInternal();
        Response::Success()
    }

    fn disable(&mut self) -> Response {
        self.stopTrackingHeapObjectsInternal();
        let profiler = unsafe { (*self.m_isolate).GetHeapProfiler() };
        if unsafe { (*self.m_state) }.booleanProperty(
            HeapProfilerAgentState::samplingHeapProfilerEnabled,
            false,
        ) {
            unsafe { (*self.m_isolate).GetHeapProfiler() }.StopSamplingHeapProfiler();
        }
        unsafe { (*self.m_isolate).GetHeapProfiler() }.ClearObjectIds();
        unsafe {
            (*self.m_state).setBoolean(HeapProfilerAgentState::heapProfilerEnabled, false);
        }
        Response::Success()
    }

    fn takeHeapSnapshot(
        &mut self,
        reportProgress: std::option::Option<bool>,
        treatGlobalObjectsAsRoots: std::option::Option<bool>,
        captureNumericValue: std::option::Option<bool>,
        exposeInternals: std::option::Option<bool>,
        callback: std::unique_ptr<TakeHeapSnapshotCallback>,
    ) {
        let protocolOptions = HeapSnapshotProtocolOptions {
            m_reportProgress: reportProgress.unwrap_or(false),
            m_treatGlobalObjectsAsRoots: treatGlobalObjectsAsRoots.unwrap_or(true),
            m_captureNumericValue: captureNumericValue.unwrap_or(false),
            m_exposeInternals: exposeInternals.unwrap_or(false),
        };
        let debugger = unsafe { (*(unsafe { (*self.m_session).inspector() })).debugger() };
        let task_runner =
            unsafe { (*(unsafe { (*self.m_session).inspector() })).isolate() };
        let paused = false;
        if paused {
            let response = self.takeHeapSnapshotNow(
                protocolOptions,
                cppgc::EmbedderStackState::kMayContainHeapPointers,
            );
            if response.IsSuccess() {
                callback.sendSuccess();
            } else {
                callback.sendFailure(response);
            }
            return;
        }

        let asyncCallbacks = Rc::clone(&self.m_asyncCallbacks);
        let protocolOptions_clone = HeapSnapshotProtocolOptions {
            m_reportProgress: protocolOptions.m_reportProgress,
            m_treatGlobalObjectsAsRoots: protocolOptions.m_treatGlobalObjectsAsRoots,
            m_captureNumericValue: protocolOptions.m_captureNumericValue,
            m_exposeInternals: protocolOptions.m_exposeInternals,
        };
        let this_ptr = self as *mut Self;
        let task = HeapSnapshotTask {
            agent: this_ptr,
            asyncCallbacks: Rc::clone(&asyncCallbacks),
            protocolOptions: protocolOptions_clone,
            callback: callback,
        };
        self.m_asyncCallbacks
            .m_heapSnapshotTasks
            .push(Box::new(task));
    }

    fn getObjectByHeapObjectId(
        &mut self,
        heapSnapshotObjectId: String16,
        objectGroup: std::option::Option<String16>,
        result: &mut std::unique_ptr<protocol::Runtime::RemoteObject>,
    ) -> Response {
        let mut ok = false;
        let id = heapSnapshotObjectId.toInteger(&mut ok);
        if !ok {
            return Response::ServerError("Invalid heap snapshot object id");
        }

        let heapObject = objectByHeapObjectId(unsafe { &mut *self.m_isolate }, id);
        if heapObject.is_null() {
            return Response::ServerError("Object is not available");
        }
        Response::Success()
    }

    fn takePendingHeapSnapshots(&mut self) {
        todo!()
    }

    fn addInspectedHeapObject(&mut self, inspectedHeapObjectId: String16) -> Response {
        let mut ok = false;
        let id = inspectedHeapObjectId.toInteger(&mut ok);
        if !ok {
            return Response::ServerError("Invalid heap snapshot object id");
        }

        let heapObject = objectByHeapObjectId(unsafe { &mut *self.m_isolate }, id);
        if heapObject.is_null() {
            return Response::ServerError("Object is not available");
        }
        let inspectable = InspectableHeapObject {
            m_heapObjectId: id,
        };

        unsafe { (*self.m_session).addInspectedObject(Box::new(inspectable)) };
        Response::Success()
    }

    fn getHeapObjectId(
        &mut self,
        objectId: String16,
        heapSnapshotObjectId: &mut String16,
    ) -> Response {
        let mut value: v8::Local<v8::Value> = unsafe { std::mem::zeroed() };
        let mut context: v8::Local<v8::Context> = unsafe { std::mem::zeroed() };
        let response = unsafe {
            (*self.m_session).unwrapObject(objectId, &mut value, &mut context, std::ptr::null_mut())
        };
        if !response.IsSuccess() {
            return response;
        }
        if value.IsUndefined() {
            return Response::InternalError();
        }

        let id = unsafe { (*self.m_isolate).GetHeapProfiler() }.GetObjectId(&value);
        *heapSnapshotObjectId = String16::fromInteger(id as usize);
        Response::Success()
    }

    fn requestHeapStatsUpdate(&mut self) {
        let stream = HeapStatsStream {
            m_frontend: &self.m_frontend,
        };
        let lastSeenObjectId = unsafe { (*self.m_isolate).GetHeapProfiler() }.GetHeapStats(&stream);
        self.m_frontend.lastSeenObjectId(
            lastSeenObjectId,
            (*(unsafe { (*self.m_session).inspector() })).currentTimeMS(),
        );
    }

    fn onTimerImpl(&mut self) {
        todo!()
    }

    fn startTrackingHeapObjectsInternal(&mut self, trackAllocations: bool) {
        unsafe { (*self.m_isolate).GetHeapProfiler() }.StartTrackingHeapObjects(trackAllocations);
        if !self.m_hasTimer {
            self.m_hasTimer = true;
            self.m_timerDelayInSeconds = kDefaultTimerDelay.InSecondsF();
            todo!()
        }
    }

    fn stopTrackingHeapObjectsInternal(&mut self) {
        if self.m_hasTimer {
            todo!()
            self.m_hasTimer = false;
        }
        unsafe { (*self.m_isolate).GetHeapProfiler() }.StopTrackingHeapObjects();
        unsafe {
            (*self.m_state).setBoolean(HeapProfilerAgentState::heapObjectsTrackingEnabled, false);
            (*self.m_state).setBoolean(HeapProfilerAgentState::allocationTrackingEnabled, false);
        }
    }

    fn startSampling(
        &mut self,
        samplingInterval: std::option::Option<f64>,
        includeObjectsCollectedByMajorGC: std::option::Option<bool>,
        includeObjectsCollectedByMinorGC: std::option::Option<bool>,
    ) -> Response {
        let profiler = unsafe { (*self.m_isolate).GetHeapProfiler() };
        let defaultSamplingInterval: u64 = 1 << 15;
        let samplingIntervalValue = samplingInterval.unwrap_or(defaultSamplingInterval as f64);
        if samplingIntervalValue <= 0.0 {
            return Response::ServerError("Invalid sampling interval");
        }
        unsafe {
            (*self.m_state).setDouble(
                HeapProfilerAgentState::samplingHeapProfilerInterval,
                samplingIntervalValue,
            );
            (*self.m_state).setBoolean(
                HeapProfilerAgentState::samplingHeapProfilerEnabled,
                true,
            );
        }

        let mut flags: i32 = 0;
        if includeObjectsCollectedByMajorGC.unwrap

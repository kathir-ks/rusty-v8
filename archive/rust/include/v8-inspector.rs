// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
//use std::os::raw::c_void;

// Mock v8-isolate.h and v8-local-handle.h
pub mod v8 {
    use std::any::Any;
    use std::fmt;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Copy)]
    pub struct Local<'a, T> {
        //pub handle: *mut T,
        pub value: i32, // Mock Value
        pub _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        pub fn new(value: i32) -> Self {
            Local {
                value,
                _marker: std::marker::PhantomData,
            }
        }
    }

    // Mock v8::Value
    #[derive(Debug, Clone)]
    pub struct Value {
        pub data: String,
    }

    impl Value {
        pub fn to_string(&self) -> String {
            self.data.clone()
        }
    }

    // Mock v8::Context
    #[derive(Debug, Clone)]
    pub struct Context {
        pub id: i32,
    }

    impl Context {
        pub fn new(id: i32) -> Self {
            Context { id }
        }
    }

    // Mock v8::Name
    pub struct Name {}

    // Mock v8::Object
    #[derive(Debug, Clone)]
    pub struct Object {
        pub properties: HashMap<String, Value>,
    }

    impl Object {
        pub fn new() -> Self {
            Object {
                properties: HashMap::new(),
            }
        }
    }
    pub type Isolate = i32; // Mock Isolate
    pub type String = String; // Mock String

    // Mock MaybeLocal
    #[derive(Debug, Clone)]
    pub struct MaybeLocal<'a, T> {
        pub value: Option<Local<'a, T>>,
    }

    impl<'a, T> MaybeLocal<'a, T> {
        pub fn empty() -> Self {
            MaybeLocal { value: None }
        }

        pub fn from(local: Local<'a, T>) -> Self {
            MaybeLocal { value: Some(local) }
        }

        pub fn is_empty(&self) -> bool {
            self.value.is_none()
        }

        pub fn unwrap(&self) -> Local<'a, T> {
            self.value.unwrap()
        }
    }

    pub struct LocalVector<T> {
        pub data: Vec<T>,
    }

    impl<T> LocalVector<T> {
        pub fn new() -> Self {
            LocalVector { data: Vec::new() }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }
    }

    pub struct StackTrace {}

    pub mod Isolate {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MessageErrorLevel {
            MessageError,
            MessageWarning,
        }
    }
}

pub mod v8_inspector {
    use super::v8;
    use std::any::Any;
    use std::fmt;
    use std::sync::{Arc, Mutex};

    pub mod internal {
        pub struct V8DebuggerId;
    }

    pub mod protocol {
        pub mod Debugger {
            pub mod API {
                pub struct SearchMatch;
            }
        }
        pub mod Runtime {
            pub mod API {
                #[derive(Debug, Clone)]
                pub struct RemoteObject {
                    pub object_id: String,
                }
                pub struct StackTrace;
                pub struct StackTraceId;
            }
        }
        pub mod Schema {
            pub mod API {
                pub struct Domain;
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct StringView {
        m_is8Bit: bool,
        m_length: usize,
        m_characters8: Option<Arc<Vec<u8>>>,
        m_characters16: Option<Arc<Vec<u16>>>,
    }

    impl StringView {
        pub fn new() -> Self {
            StringView {
                m_is8Bit: true,
                m_length: 0,
                m_characters8: None,
                m_characters16: None,
            }
        }

        pub fn from_u8(characters: &[u8]) -> Self {
            StringView {
                m_is8Bit: true,
                m_length: characters.len(),
                m_characters8: Some(Arc::new(characters.to_vec())),
                m_characters16: None,
            }
        }

        pub fn from_u16(characters: &[u16]) -> Self {
            StringView {
                m_is8Bit: false,
                m_length: characters.len(),
                m_characters8: None,
                m_characters16: Some(Arc::new(characters.to_vec())),
            }
        }

        pub fn is8Bit(&self) -> bool {
            self.m_is8Bit
        }
        pub fn length(&self) -> usize {
            self.m_length
        }

        pub fn characters8(&self) -> Option<&[u8]> {
            self.m_characters8.as_ref().map(|v| v.as_slice())
        }
        pub fn characters16(&self) -> Option<&[u16]> {
            self.m_characters16.as_ref().map(|v| v.as_slice())
        }
    }

    pub trait StringBuffer {
        fn string(&self) -> StringView;
        //fn as_any(&self) -> &dyn Any;
    }

    pub struct BasicStringBuffer {
        string_view: StringView,
    }

    impl BasicStringBuffer {
        pub fn new(string_view: StringView) -> Self {
            BasicStringBuffer { string_view }
        }
    }

    impl StringBuffer for BasicStringBuffer {
        fn string(&self) -> StringView {
            self.string_view.clone()
        }
    }

    impl dyn StringBuffer {
        pub fn create(string_view: StringView) -> Box<dyn StringBuffer> {
            Box::new(BasicStringBuffer::new(string_view))
        }
    }

    pub struct V8ContextInfo {
        pub context: v8::Local<'static, v8::Context>,
        pub contextGroupId: i32,
        pub humanReadableName: StringView,
        pub origin: StringView,
        pub auxData: StringView,
        pub hasMemoryOnConsole: bool,
    }

    impl V8ContextInfo {
        pub fn new(
            context: v8::Local<'static, v8::Context>,
            contextGroupId: i32,
            humanReadableName: StringView,
        ) -> Self {
            V8ContextInfo {
                context,
                contextGroupId,
                humanReadableName,
                origin: StringView::new(),
                auxData: StringView::new(),
                hasMemoryOnConsole: false,
            }
        }

        pub fn executionContextId(context: v8::Local<'static, v8::Context>) -> i32 {
            context.id
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct V8DebuggerId {
        m_first: i64,
        m_second: i64,
    }

    impl V8DebuggerId {
        pub fn new() -> Self {
            V8DebuggerId { m_first: 0, m_second: 0 }
        }
        fn from_pair(pair: (i64, i64)) -> Self {
            V8DebuggerId {
                m_first: pair.0,
                m_second: pair.1,
            }
        }
        pub fn to_string(&self) -> Box<dyn StringBuffer> {
            let s = format!("({}, {})", self.m_first, self.m_second);
            let string_view = StringView::from_u8(s.as_bytes());
            dyn StringBuffer::create(string_view)
        }
        pub fn is_valid(&self) -> bool {
            self.m_first != 0 || self.m_second != 0
        }
        pub fn pair(&self) -> (i64, i64) {
            (self.m_first, self.m_second)
        }
    }

    impl fmt::Debug for V8DebuggerId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "V8DebuggerId({}, {})", self.m_first, self.m_second)
        }
    }

    #[derive(Debug, Clone)]
    pub struct V8StackFrame {
        pub sourceURL: StringView,
        pub functionName: StringView,
        pub lineNumber: i32,
        pub columnNumber: i32,
        pub scriptId: i32,
    }

    pub trait V8StackTrace {
        fn firstNonEmptySourceURL(&self) -> StringView;
        fn isEmpty(&self) -> bool;
        fn topSourceURL(&self) -> StringView;
        fn topLineNumber(&self) -> i32;
        fn topColumnNumber(&self) -> i32;
        fn topScriptId(&self) -> i32;
        fn topFunctionName(&self) -> StringView;
        fn buildInspectorObject(
            &self,
            maxAsyncDepth: i32,
        ) -> std::unique_ptr::UniquePtr<protocol::Runtime::API::StackTrace>;
        fn to_string(&self) -> Box<dyn StringBuffer>;
        fn clone(&self) -> Box<dyn V8StackTrace>;
        fn frames(&self) -> Vec<V8StackFrame>;
    }

    pub trait Inspectable {
        fn get(&self, context: v8::Local<v8::Context>) -> v8::Local<v8::Value>;
    }

    pub trait V8InspectorSession {
        fn addInspectedObject(&mut self, inspectable: Box<dyn Inspectable>);
        fn canDispatchMethod(method: StringView) -> bool;
        fn dispatchProtocolMessage(&mut self, message: StringView);
        fn state(&self) -> Vec<u8>;
        fn supportedDomains(&self) -> Vec<std::unique_ptr::UniquePtr<protocol::Schema::API::Domain>>;
        fn schedulePauseOnNextStatement(&mut self, breakReason: StringView, breakDetails: StringView);
        fn cancelPauseOnNextStatement(&mut self);
        fn breakProgram(&mut self, breakReason: StringView, breakDetails: StringView);
        fn setSkipAllPauses(&mut self, skip: bool);
        fn resume(&mut self, setTerminateOnResume: bool);
        fn stepOver(&mut self);
        fn searchInTextByLines(
            &mut self,
            text: StringView,
            query: StringView,
            caseSensitive: bool,
            isRegex: bool,
        ) -> Vec<std::unique_ptr::UniquePtr<protocol::Debugger::API::SearchMatch>>;
        fn wrapObject(
            &mut self,
            context: v8::Local<v8::Context>,
            value: v8::Local<v8::Value>,
            groupName: StringView,
            generatePreview: bool,
        ) -> std::unique_ptr::UniquePtr<protocol::Runtime::API::RemoteObject>;
        fn unwrapObject(
            &mut self,
            error: &mut Option<Box<dyn StringBuffer>>,
            objectId: StringView,
            value: &mut v8::Local<v8::Value>,
            context: &mut v8::Local<v8::Context>,
            objectGroup: &mut Option<Box<dyn StringBuffer>>,
        ) -> bool;
        fn releaseObjectGroup(&mut self, objectGroup: StringView);
        fn triggerPreciseCoverageDeltaUpdate(&mut self, occasion: StringView);
        fn evaluate(
            &mut self,
            context: v8::Local<v8::Context>,
            expression: StringView,
            includeCommandLineAPI: bool,
        ) -> EvaluateResult;
        fn stop(&mut self);
    }

    pub struct EvaluateResult {
        pub type_: EvaluateResultType,
        pub value: v8::Local<'static, v8::Value>,
    }

    pub enum EvaluateResultType {
        kNotRun,
        kSuccess,
        kException,
    }

    pub struct DeepSerializedValue {
        pub type_: Box<dyn StringBuffer>,
        pub value: v8::MaybeLocal<'static, v8::Value>,
    }

    impl DeepSerializedValue {
        pub fn new(type_: Box<dyn StringBuffer>, value: v8::MaybeLocal<'static, v8::Value>) -> Self {
            DeepSerializedValue { type_, value }
        }
    }

    pub struct DeepSerializationResult {
        pub serializedValue: Option<DeepSerializedValue>,
        pub errorMessage: Option<Box<dyn StringBuffer>>,
        pub isSuccess: bool,
    }

    impl DeepSerializationResult {
        pub fn new_success(serializedValue: DeepSerializedValue) -> Self {
            DeepSerializationResult {
                serializedValue: Some(serializedValue),
                errorMessage: None,
                isSuccess: true,
            }
        }
        pub fn new_failure(errorMessage: Box<dyn StringBuffer>) -> Self {
            DeepSerializationResult {
                serializedValue: None,
                errorMessage: Some(errorMessage),
                isSuccess: false,
            }
        }
    }

    pub trait V8InspectorClient {
        fn runMessageLoopOnPause(&self, contextGroupId: i32) {}
        fn runMessageLoopOnInstrumentationPause(&self, contextGroupId: i32) {
            self.runMessageLoopOnPause(contextGroupId);
        }
        fn quitMessageLoopOnPause(&self) {}
        fn runIfWaitingForDebugger(&self, contextGroupId: i32) {}
        fn muteMetrics(&self, contextGroupId: i32) {}
        fn unmuteMetrics(&self, contextGroupId: i32) {}
        fn beginUserGesture(&self) {}
        fn endUserGesture(&self) {}
        fn deepSerialize(
            &self,
            v8Value: v8::Local<v8::Value>,
            maxDepth: i32,
            additionalParameters: v8::Local<v8::Object>,
        ) -> Option<DeepSerializationResult> {
            None
        }
        fn valueSubtype(&self, value: v8::Local<v8::Value>) -> Option<Box<dyn StringBuffer>> {
            None
        }
        fn descriptionForValueSubtype(
            &self,
            context: v8::Local<v8::Context>,
            value: v8::Local<v8::Value>,
        ) -> Option<Box<dyn StringBuffer>> {
            None
        }
        fn isInspectableHeapObject(&self, object: v8::Local<v8::Object>) -> bool {
            true
        }
        fn ensureDefaultContextInGroup(&self, contextGroupId: i32) -> v8::Local<v8::Context> {
            v8::Local::new(0) //Return a mock context
        }
        fn beginEnsureAllContextsInGroup(&self, contextGroupId: i32) {}
        fn endEnsureAllContextsInGroup(&self, contextGroupId: i32) {}
        fn installAdditionalCommandLineAPI(&self, context: v8::Local<v8::Context>, object: v8::Local<v8::Object>) {}
        fn consoleAPIMessage(
            &self,
            contextGroupId: i32,
            level: v8::Isolate::MessageErrorLevel,
            message: &StringView,
            url: &StringView,
            lineNumber: u32,
            columnNumber: u32,
            stackTrace: *mut dyn V8StackTrace,
        ) {
        }
        fn memoryInfo(
            &self,
            isolate: *mut v8::Isolate,
            context: v8::Local<v8::Context>,
        ) -> v8::MaybeLocal<v8::Value> {
            v8::MaybeLocal::empty()
        }
        fn consoleTime(&self, isolate: *mut v8::Isolate, label: v8::String) {}
        fn consoleTimeEnd(&self, isolate: *mut v8::Isolate, label: v8::String) {}
        fn consoleTimeStamp(&self, isolate: *mut v8::Isolate, label: v8::String) {}
        fn consoleTimeStampWithArgs(
            &self,
            isolate: *mut v8::Isolate,
            label: v8::String,
            args: &v8::LocalVector<v8::Value>,
        ) {
        }
        fn consoleClear(&self, contextGroupId: i32) {}
        fn currentTimeMS(&self) -> f64 {
            0.0
        }
        type TimerCallback = fn(*mut std::ffi::c_void);
        fn startRepeatingTimer(&self, interval: f64, callback: TimerCallback, data: *mut std::ffi::c_void) {}
        fn cancelTimer(&self, data: *mut std::ffi::c_void) {}
        fn canExecuteScripts(&self, contextGroupId: i32) -> bool {
            true
        }
        fn maxAsyncCallStackDepthChanged(&self, depth: i32) {}
        fn resourceNameToUrl(&self, resourceName: &StringView) -> Option<Box<dyn StringBuffer>> {
            None
        }
        fn generateUniqueId(&self) -> i64 {
            0
        }
        fn dispatchError(
            &self,
            context: v8::Local<v8::Context>,
            message: v8::Local<v8::Message>,
            value: v8::Local<v8::Value>,
        ) {
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct V8StackTraceId {
        id: usize,
        debugger_id: (i64, i64),
        should_pause: bool,
    }

    impl V8StackTraceId {
        pub fn new() -> Self {
            V8StackTraceId {
                id: 0,
                debugger_id: (0, 0),
                should_pause: false,
            }
        }
        pub fn new_with_id(id: usize, debugger_id: (i64, i64)) -> Self {
            V8StackTraceId {
                id,
                debugger_id,
                should_pause: false,
            }
        }
        pub fn new_with_pause(id: usize, debugger_id: (i64, i64), should_pause: bool) -> Self {
            V8StackTraceId {
                id,
                debugger_id,
                should_pause,
            }
        }
        pub fn from_stringview(view: StringView) -> Self {
            //Mock parse StringView to StackTraceId
            V8StackTraceId::new()
        }
        pub fn is_invalid(&self) -> bool {
            self.id == 0 && self.debugger_id == (0, 0)
        }
        pub fn to_string(&self) -> Box<dyn StringBuffer> {
            let s = format!(
                "id: {}, debugger_id: ({}, {}), should_pause: {}",
                self.id, self.debugger_id.0, self.debugger_id.1, self.should_pause
            );
            let string_view = StringView::from_u8(s.as_bytes());
            dyn StringBuffer::create(string_view)
        }
    }

    pub trait V8Inspector {
        fn contextCreated(&mut self, contextInfo: &V8ContextInfo);
        fn contextDestroyed(&mut self, context: v8::Local<v8::Context>);
        fn resetContextGroup(&mut self, contextGroupId: i32);
        fn contextById(&self, contextId: i32) -> Option<v8::Local<'static, v8::Context>>;
        fn uniqueDebuggerId(&self, contextId: i32) -> V8DebuggerId;
        fn isolateId(&self) -> u64;
        fn idleStarted(&mut self);
        fn idleFinished(&mut self);
        fn asyncTaskScheduled(&mut self, taskName: StringView, task: *mut std::ffi::c_void, recurring: bool);
        fn asyncTaskCanceled(&mut self, task: *mut std::ffi::c_void);
        fn asyncTaskStarted(&mut self, task: *mut std::ffi::c_void);
        fn asyncTaskFinished(&mut self, task: *mut std::ffi::c_void);
        fn allAsyncTasksCanceled(&mut self);
        fn storeCurrentStackTrace(&mut self, description: StringView) -> V8StackTraceId;
        fn externalAsyncTaskStarted(&mut self, parent: &V8StackTraceId);
        fn externalAsyncTaskFinished(&mut self, parent: &V8StackTraceId);
        fn exceptionThrown(
            &mut self,
            context: v8::Local<v8::Context>,
            message: StringView,
            exception: v8::Local<v8::Value>,
            detailedMessage: StringView,
            url: StringView,
            lineNumber: u32,
            columnNumber: u32,
            stackTrace: Option<Box<dyn V8StackTrace>>,
            scriptId: i32,
        ) -> u32;
        fn exceptionRevoked(&mut self, context: v8::Local<v8::Context>, exceptionId: u32, message: StringView);
        fn associateExceptionData(
            &mut self,
            context: v8::Local<v8::Context>,
            exception: v8::Local<v8::Value>,
            key: v8::Local<v8::Name>,
            value: v8::Local<v8::Value>,
        ) -> bool;
        fn createStackTrace(&self, stackTrace: v8::Local<v8::StackTrace>) -> Option<Box<dyn V8StackTrace>>;
        fn captureStackTrace(&self, fullStack: bool) -> Option<Box<dyn V8StackTrace>>;
        fn connect(
            &mut self,
            contextGroupId: i32,
            channel: &mut dyn Channel,
            state: StringView,
            client_trust_level: ClientTrustLevel,
            session_pause_state: SessionPauseState,
        ) -> Option<Box<dyn V8InspectorSession>>;
    }

    pub trait Channel {
        fn sendResponse(&mut self, callId: i32, message: Box<dyn StringBuffer>);
        fn sendNotification(&mut self, message: Box<dyn StringBuffer>);
        fn flushProtocolNotifications(&mut self);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ClientTrustLevel {
        kUntrusted,
        kFullyTrusted,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SessionPauseState {
        kWaitingForDebugger,
        kNotWaitingForDebugger,
    }

    pub struct BasicV8Inspector {
        isolate: *mut v8::Isolate,
        client: *mut dyn V8InspectorClient,
        context_map: Arc<Mutex<HashMap<i32, v8::Local<'static, v8::Context>>>>,
    }

    impl BasicV8Inspector {
        pub fn new(isolate: *mut v8::Isolate, client: *mut dyn V8InspectorClient) -> Self {
            BasicV8Inspector {
                isolate,
                client,
                context_map: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl V8Inspector for BasicV8Inspector {
        fn contextCreated(&mut self, contextInfo: &V8ContextInfo) {
            let mut map = self.context_map.lock().unwrap();
            map.insert(contextInfo.context.id, contextInfo.context.clone());
        }

        fn contextDestroyed(&mut self, context: v8::Local<v8::Context>) {
            let mut map = self.context_map.lock().unwrap();
            map.remove(&context.id);
        }

        fn resetContextGroup(&mut self, contextGroupId: i32) {
            // Mock implementation
        }

        fn contextById(&self, contextId: i32) -> Option<v8::Local<'static, v8::Context>> {
            let map = self.context_map.lock().unwrap();
            map.get(&contextId).cloned()
        }

        fn uniqueDebuggerId(&self, contextId: i32) -> V8DebuggerId {
            V8DebuggerId::new() //Mock generate unique DebuggerId
        }

        fn isolateId(&self) -> u64 {
            self.isolate as u64 //Mock address as Id
        }

        fn idleStarted(&mut self) {
            // Mock implementation
        }

        fn idleFinished(&mut self) {
            // Mock implementation
        }

        fn asyncTaskScheduled(&mut self, taskName: StringView, task: *mut std::ffi::c_void, recurring: bool) {
            // Mock implementation
        }

        fn asyncTaskCanceled(&mut self, task: *mut std::ffi::c_void) {
            // Mock implementation
        }

        fn asyncTaskStarted(&mut self, task: *mut std::ffi::c_void) {
            // Mock implementation
        }

        fn asyncTaskFinished(&mut self, task: *mut std::ffi::c_void) {
            // Mock implementation
        }

        fn allAsyncTasksCanceled(&mut self) {
            // Mock implementation
        }

        fn storeCurrentStackTrace(&mut self, description: StringView) -> V8StackTraceId {
            V8StackTraceId::new() // Mock implementation
        }

        fn externalAsyncTaskStarted(&mut self, parent: &V8StackTraceId) {
            // Mock implementation
        }

        fn externalAsyncTaskFinished(&mut self, parent: &V8StackTraceId) {
            // Mock implementation
        }

        fn exceptionThrown(
            &mut self,
            context: v8::Local<v8::Context>,
            message: StringView,
            exception: v8::Local<v8::Value>,
            detailedMessage: StringView,
            url: StringView,
            lineNumber: u32,
            columnNumber: u32,
            stackTrace: Option<Box<dyn V8StackTrace>>,
            scriptId: i32,
        ) -> u32 {
            0 // Mock implementation
        }

        fn exceptionRevoked(&mut self, context: v8::Local<v8::Context>, exceptionId: u32, message: StringView) {
            // Mock implementation
        }

        fn associateExceptionData(
            &mut self,
            context: v8::Local<v8::Context>,
            exception: v8::Local<v8::Value>,
            key: v8::Local<v8::Name>,
            value: v8::Local<v8::Value>,
        ) -> bool {
            false // Mock implementation
        }

        fn createStackTrace(&self, stackTrace: v8::Local<v8::StackTrace>) -> Option<Box<dyn V8StackTrace>> {
            None // Mock implementation
        }

        fn captureStackTrace(&self, fullStack: bool) -> Option<Box<dyn V8StackTrace>> {
            None // Mock implementation
        }
        fn connect(
            &mut self,
            contextGroupId: i32,
            channel: &mut dyn Channel,
            state: StringView,
            client_trust_level: ClientTrustLevel,
            session_pause_state: SessionPauseState,
        ) -> Option<Box<dyn V8InspectorSession>> {
            None // Mock implementation
        }
    }

    impl dyn V8Inspector {
        pub fn create(isolate: *mut v8::Isolate, client: *mut dyn V8InspectorClient) -> Box<dyn V8Inspector> {
            Box::new(BasicV8Inspector::new(isolate, client))
        }
    }
    //Implement std::unique_ptr::UniquePtr because it is used in the original code.
    pub mod std {
        pub mod unique_ptr {
            pub struct UniquePtr<T> {
                value: T,
            }

            impl<T> UniquePtr<T> {
                pub fn new(value: T) -> Self {
                    UniquePtr { value }
                }
            }
        }
    }
}
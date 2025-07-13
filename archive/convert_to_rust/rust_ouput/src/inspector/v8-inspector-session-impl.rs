// Converted from V8 C++ source files:
// Header: v8-inspector-session-impl.h
// Implementation: v8-inspector-session-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use v8::HandleScope;

use third_party_inspector_protocol::crdtp::cbor;
use third_party_inspector_protocol::crdtp::dispatch;
use third_party_inspector_protocol::crdtp::json;

use crate::EvaluateResult;
use crate::StringView;
use crate::V8InspectorSession;

pub struct V8 {}
pub struct Use {}
pub struct code {};
pub enum Status {}
pub struct StringView {}
struct HeapNumber {
    number: i32,
}

struct TaggedField<T, const OFFSET: usize>;

pub enum void {}
pub struct EvaluateResult {
    // Define the fields of EvaluateResult here
}

pub struct V8InspectorSession {}

pub trait Inspectable {}

pub mod protocol {
    pub mod Schema {
        pub mod API {
            pub struct Domain {}
        }
        pub struct Domain {}
    }
    pub mod Runtime {
        pub mod API {
            pub struct RemoteObject {}
        }
        pub struct RemoteObject {}
    }
    pub mod Debugger {
        pub mod API {
            pub struct SearchMatch {}
        }
        pub struct SearchMatch {}
    }
    pub struct Response {
        success: bool,
        message: String,
    }

    impl Response {
        pub fn success() -> Response {
            Response {
                success: true,
                message: String::new(),
            }
        }

        pub fn error(message: String) -> Response {
            Response {
                success: false,
                message,
            }
        }

        pub fn is_success(&self) -> bool {
            self.success
        }

        pub fn is_error(&self) -> bool {
            !self.success
        }

        pub fn message(&self) -> &String {
            &self.message
        }

        pub fn IsSuccess(&self) -> bool {
            self.success
        }
        pub fn IsError(&self) -> bool {
            !self.success
        }
        pub fn Message(&self) -> String {
            self.message.clone()
        }
        pub fn ServerError(message: &str) -> Response {
            Response {
                success: false,
                message: message.to_string(),
            }
        }
        pub fn Success() -> Response {
            Response {
                success: true,
                message: String::new(),
            }
        }
    }

    pub trait Serializable {
        fn serialize(&self) -> Vec<u8>;
    }

    pub struct DictionaryValue {
        data: HashMap<String, Box<Value>>,
    }

    impl DictionaryValue {
        pub fn new() -> DictionaryValue {
            DictionaryValue {
                data: HashMap::new(),
            }
        }
        pub fn create() -> Box<DictionaryValue> {
            Box::new(DictionaryValue::new())
        }

        pub fn set_object(&mut self, name: String, value: Box<DictionaryValue>) {
            self.data.insert(name, Box::new(Value::Dictionary(value)));
        }

        pub fn get_object(&mut self, name: &str) -> Option<&mut DictionaryValue> {
            if let Some(Value::Dictionary(dict)) = self.data.get_mut(name).map(|boxed| &mut **boxed) {
                if let Some(downcasted) = dict.as_any().downcast_mut::<DictionaryValue>() {
                    return Some(downcasted)
                } else {
                    return None
                }
            } else {
                return None
            }
        }
        pub fn get_boolean(&self, name: &str) -> Option<bool> {
            match self.data.get(name) {
                Some(boxed) => match &**boxed {
                    Value::Boolean(b) => Some(*b),
                    _ => None,
                },
                None => None,
            }
        }

        pub fn setBoolean(&mut self, name: &str, value: bool) {
            self.data.insert(name.to_string(), Box::new(Value::Boolean(value)));
        }
        pub fn parse_binary(data: *const u8, size: usize) -> Box<Value> {
            Box::new(Value::String("dummy".to_string()))
        }
        pub fn cast(value: Box<Value>) -> Option<Box<DictionaryValue>> {
            if let Value::Dictionary(dict) = *value {
                Some(dict)
            } else {
                None
            }
        }

        pub fn getBoolean(&self, key: &str, out: &mut bool) {
            if let Some(value) = self.data.get(key) {
                if let Value::Boolean(b) = **value {
                    *out = b;
                }
            }
        }

        pub fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl Serializable for DictionaryValue {
        fn serialize(&self) -> Vec<u8> {
            // Placeholder implementation
            Vec::new()
        }
    }

    pub enum Value {
        String(String),
        Number(f64),
        Boolean(bool),
        Dictionary(Box<DictionaryValue>),
        Array(Vec<Box<Value>>),
        Null,
    }

    impl Value {
        pub fn parseBinary(data: *const u8, size: usize) -> Box<Value> {
            // Placeholder implementation
            Box::new(Value::String("dummy".to_string()))
        }
    }
    
}

pub mod v8_crdtp{
    pub struct Span<T> {
        ptr: *const T,
        len: usize,
    }
    impl<T> Span<T> {
        pub fn data(&self) -> *const T {
            self.ptr
        }
        pub fn size(&self) -> usize {
            self.len
        }
    }
    
    pub fn SpanFrom<T>(vec: Vec<T>) -> Span<T> {
        let ptr = vec.as_ptr();
        let len = vec.len();
        std::mem::forget(vec);
        Span { ptr, len }
    }

    pub mod json {
        use super::Span;
        use crate::Status;
        pub fn ConvertJSONToCBOR(json: Span<u8>, cbor: &mut Vec<u8>) -> Status {
            Status::Success
        }
        pub fn ConvertJSONToCBOR(json: Span<u16>, cbor: &mut Vec<u8>) -> Status {
            Status::Success
        }
        pub fn ConvertCBORToJSON(cbor: Span<u8>, json: &mut Vec<u8>) -> Status {
            Status::Success
        }
    }

    pub mod cbor {
        use super::Span;
        use crate::Status;
        pub fn CheckCBORMessage(message: Span<u8>) -> Status {
            Status::Success
        }
    }

    pub struct Dispatchable{ok_val : bool, call_id : i32, dispatch_error : String}

    impl Dispatchable {
        pub fn new() -> Dispatchable {
            Dispatchable{ok_val : true, call_id : 0, dispatch_error : "".to_string()}
        }
        pub fn DispatchError(&self) -> String {
            "DispatchError".to_string()
        }
        pub fn CallId(&self) -> i32 {
            0
        }
        pub fn HasCallId(&self) -> bool {
            true
        }
        pub fn ok(&self) -> bool {
            true
        }
    }

    pub fn CreateErrorNotification(dispatch_error : String) -> Box<protocol::DictionaryValue> {
        Box::new(protocol::DictionaryValue::new())
    }

    pub fn CreateErrorResponse(call_id : i32, dispatch_error : String) -> Box<protocol::DictionaryValue> {
        Box::new(protocol::DictionaryValue::new())
    }
}

pub mod v8 {
    pub struct Isolate {}
    impl Isolate {
        pub fn scope(&self) -> IsolateScope {
            IsolateScope{}
        }
    }
    pub struct IsolateScope {}

    pub struct Context {}

    pub struct TryCatch {}
    impl TryCatch {
        pub fn HasCaught(&self) -> bool {
            false
        }
        pub fn Exception(&self) -> Local<Value> {
            Local{value : 0}
        }
    }

    pub struct Local<T> {value : i32}
    
    pub struct String {}

    pub mod debug {
        use super::String;
        use super::Isolate;
        use super::Local;
        use super::Value;

        pub enum EvaluateGlobalMode {
            kDefault,
        }
        pub fn EvaluateGlobal(isolate: *mut Isolate, source: Local<String>, mode: EvaluateGlobalMode, repl_mode: bool) -> MaybeLocal<Value> {
            MaybeLocal{local: Local{value : 0}}
        }
    }

    pub struct MicrotasksScope {}
    impl MicrotasksScope {
        pub fn new() -> MicrotasksScope{
            MicrotasksScope{}
        }
    }

    pub struct Value {}

    pub struct MaybeLocal<T> {local: Local<T>}

    impl<T> MaybeLocal<T> {
        pub fn ToLocal(&self, out: &mut Local<T>) -> bool {
            true
        }
    }

    pub struct EscapableHandleScope {}
    impl EscapableHandleScope {
        pub fn new(isolate: *mut Isolate) -> EscapableHandleScope {
            EscapableHandleScope{}
        }
        pub fn Escape(&self, value: Local<Value>) -> Local<Value> {
            Local{value : 0}
        }
    }

    pub struct HandleScope {}
}

pub mod third_party_inspector_protocol {
    pub mod crdtp {
        pub fn DispatchResponse {}
        impl DispatchResponse {
            pub fn ParseError(msg: String) -> String {
                msg
            }
        }
    }
}

fn stringViewStartsWith(method: StringView, commandPrefix: &str) -> bool {
    true
}

fn toString16(string_view: StringView) -> String {
    String::new()
}

fn searchInTextByLinesImpl(
    session: &V8InspectorSessionImpl,
    text: String,
    query: String,
    caseSensitive: bool,
    isRegex: bool,
) -> Vec<std::unique_ptr<protocol::Debugger::SearchMatch>> {
    Vec::new()
}

fn toV8String(isolate: *mut v8::Isolate, expression: StringView) -> v8::String {
    v8::String {}
}

pub struct V8InspectorSessionImpl {
    m_contextGroupId: i32,
    m_sessionId: i32,
    m_inspector: *mut V8InspectorImpl,
    m_channel: *mut dyn V8Inspector::Channel,
    m_customObjectFormatterEnabled: bool,
    m_dispatcher: UberDispatcher,
    m_state: Box<protocol::DictionaryValue>,
    m_runtimeAgent: Option<Box<V8RuntimeAgentImpl>>,
    m_debuggerAgent: Option<Box<V8DebuggerAgentImpl>>,
    m_heapProfilerAgent: Option<Box<V8HeapProfilerAgentImpl>>,
    m_profilerAgent: Option<Box<V8ProfilerAgentImpl>>,
    m_consoleAgent: Option<Box<V8ConsoleAgentImpl>>,
    m_schemaAgent: Option<Box<V8SchemaAgentImpl>>,
    m_inspectedObjects: Vec<Box<dyn Inspectable>>,
    use_binary_protocol_: bool,
    m_clientTrustLevel: V8Inspector::ClientTrustLevel,
    debugger_barrier: std::shared_ptr::SharedPtr<V8DebuggerBarrier>,
}

impl V8InspectorSessionImpl {
    pub fn create(
        inspector: *mut V8InspectorImpl,
        contextGroupId: i32,
        sessionId: i32,
        channel: *mut dyn V8Inspector::Channel,
        state: StringView,
        clientTrustLevel: V8Inspector::ClientTrustLevel,
        debuggerBarrier: std::shared_ptr::SharedPtr<V8DebuggerBarrier>,
    ) -> std::unique_ptr::UniquePtr<V8InspectorSessionImpl> {
        std::unique_ptr::UniquePtr::new(V8InspectorSessionImpl {
            m_contextGroupId: contextGroupId,
            m_sessionId: sessionId,
            m_inspector: inspector,
            m_channel: channel,
            m_customObjectFormatterEnabled: false,
            m_dispatcher: UberDispatcher::new(),
            m_state: ParseState(state),
            m_runtimeAgent: None,
            m_debuggerAgent: None,
            m_heapProfilerAgent: None,
            m_profilerAgent: None,
            m_consoleAgent: None,
            m_schemaAgent: None,
            m_inspectedObjects: Vec::new(),
            use_binary_protocol_: false,
            m_clientTrustLevel: clientTrustLevel,
            debugger_barrier: debuggerBarrier,
        })
    }

    pub fn inspector(&self) -> *mut V8InspectorImpl {
        self.m_inspector
    }
    pub fn consoleAgent(&mut self) -> Option<&mut V8ConsoleAgentImpl> {
        self.m_consoleAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn debuggerAgent(&mut self) -> Option<&mut V8DebuggerAgentImpl> {
        self.m_debuggerAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn schemaAgent(&mut self) -> Option<&mut V8SchemaAgentImpl> {
        self.m_schemaAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn profilerAgent(&mut self) -> Option<&mut V8ProfilerAgentImpl> {
        self.m_profilerAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn runtimeAgent(&mut self) -> Option<&mut V8RuntimeAgentImpl> {
        self.m_runtimeAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn heapProfilerAgent(&mut self) -> Option<&mut V8HeapProfilerAgentImpl> {
        self.m_heapProfilerAgent.as_mut().map(|agent| &mut **agent)
    }
    pub fn contextGroupId(&self) -> i32 {
        self.m_contextGroupId
    }
    pub fn sessionId(&self) -> i32 {
        self.m_sessionId
    }

    pub fn findInjectedScript(
        &mut self,
        contextId: i32,
        injectedScript: &mut *mut InjectedScript,
    ) -> protocol::Response {
        *injectedScript = std::ptr::null_mut();
        let context = unsafe { (*self.m_inspector).getContext(self.m_contextGroupId, contextId) };

        if context.is_null() {
            return protocol::Response::ServerError("Cannot find context with specified id".to_string());
        }

        let injected_script_local = unsafe { (*context).getInjectedScript(self.m_sessionId) };

        if injected_script_local.is_null() {
            let injected_script_created = unsafe { (*context).createInjectedScript(self.m_sessionId) };
            *injectedScript = injected_script_created;
            if self.m_customObjectFormatterEnabled {
                unsafe { (*injected_script_created).setCustomObjectFormatterEnabled(true) };
            }
        } else {
            *injectedScript = injected_script_local;
        }

        protocol::Response::Success()
    }

    pub fn findInjectedScript_remote(
        &mut self,
        objectId: *mut RemoteObjectIdBase,
        injectedScript: &mut *mut InjectedScript,
    ) -> protocol::Response {
        if unsafe { (*objectId).isolateId() } != unsafe { (*self.m_inspector).isolateId() } {
            return protocol::Response::ServerError("Cannot find context with specified id".to_string());
        }
        self.findInjectedScript(unsafe { (*objectId).contextId() }, injectedScript)
    }

    pub fn reset(&mut self) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.reset();
        }
        if let Some(runtime_agent) = &mut self.m_runtimeAgent {
            runtime_agent.reset();
        }
        self.discardInjectedScripts();
    }

    pub fn discardInjectedScripts(&mut self) {
        self.m_inspectedObjects.clear();
        let sessionId = self.m_sessionId;
        let context_group_id = self.m_contextGroupId;
        let inspector = self.m_inspector;
        unsafe {
            (*inspector).forEachContext(context_group_id, &mut |context| {
                (*context).discardInjectedScript(sessionId);
            });
        }
    }

    pub fn reportAllContexts(&mut self, agent: *mut V8RuntimeAgentImpl) {
        let context_group_id = self.m_contextGroupId;
        let inspector = self.m_inspector;
        unsafe {
            (*inspector).forEachContext(context_group_id, &mut |context| {
                (*agent).reportExecutionContextCreated(context);
            });
        }
    }

    pub fn setCustomObjectFormatterEnabled(&mut self, enabled: bool) {
        self.m_customObjectFormatterEnabled = enabled;
        let sessionId = self.m_sessionId;
        let context_group_id = self.m_contextGroupId;
        let inspector = self.m_inspector;

        unsafe {
            (*inspector).forEachContext(context_group_id, &mut |context| {
                let injectedScript = (*context).getInjectedScript(sessionId);
                if !injectedScript.is_null() {
                    (*injectedScript).setCustomObjectFormatterEnabled(enabled);
                }
            });
        }
    }

    pub fn dispatchProtocolMessage(&mut self, message: StringView) {
        use v8_crdtp::{SpanFrom, Span};
        let mut converted_cbor: Vec<u8> = Vec::new();

        if IsCBORMessage(message) {
            self.use_binary_protocol_ = true;
            self.m_state.setBoolean("use_binary_protocol", true);
            let cbor = Span::<u8> {
                ptr: message.characters8(),
                len: message.length(),
            };
        } else {
            let status = ConvertToCBOR(message, &mut converted_cbor);
            if Status::Success != status {
                let notification = v8_crdtp::CreateErrorNotification(v8_crdtp::dispatch::DispatchResponse::ParseError("Could not convert to CBOR".to_string()));
                let serialized = self.serializeForFrontend(Box::new(protocol::DictionaryValue::new()));
                unsafe { (*self.m_channel).sendNotification(serialized) };
                return;
            }
        }

        let dispatchable = v8_crdtp::Dispatchable::new();
        if !dispatchable.ok() {
            if !dispatchable.HasCallId() {
                let notification = v8_crdtp::CreateErrorNotification(dispatchable.DispatchError());
                let serialized = self.serializeForFrontend(Box::new(protocol::DictionaryValue::new()));
                unsafe { (*self.m_channel).sendNotification(serialized) };
            } else {
                let response = v8_crdtp::CreateErrorResponse(dispatchable.CallId(), dispatchable.DispatchError());
                let serialized = self.serializeForFrontend(Box::new(protocol::DictionaryValue::new()));
                unsafe { (*self.m_channel).sendResponse(dispatchable.CallId(), serialized) };
            }
            return;
        }
        self.m_dispatcher.Dispatch(dispatchable).Run();
    }

    pub fn state(&self) -> Vec<u8> {
        self.m_state.serialize()
    }

    pub fn supportedDomains(&self) -> Vec<std::unique_ptr::UniquePtr<protocol::Schema::API::Domain>> {
        let domains = self.supportedDomainsImpl();
        let mut result: Vec<std::unique_ptr::UniquePtr<protocol::Schema::API::Domain>> = Vec::new();
        for i in 0..domains.len() {
            result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::API::Domain {}));
        }
        result
    }

    pub fn addInspectedObject(&mut self, inspectable: Box<dyn Inspectable>) {
        self.m_inspectedObjects.insert(0, inspectable);
        if self.m_inspectedObjects.len() > Self::kInspectedObjectBufferSize as usize {
            self.m_inspectedObjects
                .resize(Self::kInspectedObjectBufferSize as usize);
        }
    }

    pub fn inspectedObject(&self, num: u32) -> Option<&dyn Inspectable> {
        if (num as usize) >= self.m_inspectedObjects.len() {
            return None;
        }
        Some(self.m_inspectedObjects[num as usize].as_ref())
    }

    pub fn schedulePauseOnNextStatement(&mut self, breakReason: StringView, breakDetails: StringView) {
        let mut cbor: Vec<u8> = Vec::new();
        let _ = ConvertToCBOR(breakDetails, &mut cbor);
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.schedulePauseOnNextStatement(
                toString16(breakReason),
                protocol::DictionaryValue::cast(protocol::Value::parseBinary(cbor.as_ptr(), cbor.len())).unwrap(),
            );
        }
    }

    pub fn cancelPauseOnNextStatement(&mut self) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.cancelPauseOnNextStatement();
        }
    }

    pub fn breakProgram(&mut self, breakReason: StringView, breakDetails: StringView) {
        let mut cbor: Vec<u8> = Vec::new();
        let _ = ConvertToCBOR(breakDetails, &mut cbor);
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.breakProgram(
                toString16(breakReason),
                protocol::DictionaryValue::cast(protocol::Value::parseBinary(cbor.as_ptr(), cbor.len())).unwrap(),
            );
        }
    }

    pub fn setSkipAllPauses(&mut self, skip: bool) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.setSkipAllPauses(skip);
        }
    }

    pub fn resume(&mut self, terminateOnResume: bool) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.resume(terminateOnResume);
        }
    }

    pub fn stepOver(&mut self) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.stepOver(Default::default());
        }
    }

    pub fn searchInTextByLines(
        &mut self,
        text: StringView,
        query: StringView,
        caseSensitive: bool,
        isRegex: bool,
    ) -> Vec<std::unique_ptr::UniquePtr<protocol::Debugger::API::SearchMatch>> {
        let matches: Vec<std::unique_ptr::UniquePtr<protocol::Debugger::SearchMatch>> =
            searchInTextByLinesImpl(self, toString16(text), toString16(query), caseSensitive, isRegex)
                .into_iter()
                .map(|m| std::unique_ptr::UniquePtr::new(protocol::Debugger::SearchMatch {}))
                .collect();

        matches
    }

    pub fn releaseObjectGroup(&mut self, objectGroup: StringView) {
        self.releaseObjectGroup_str(toString16(objectGroup));
    }

    pub fn unwrapObject(
        &mut self,
        error: &mut std::unique_ptr::UniquePtr<StringBuffer>,
        objectId: StringView,
        object: &mut v8::Local<v8::Value>,
        context: &mut v8::Local<v8::Context>,
        objectGroup: &mut std::unique_ptr::UniquePtr<StringBuffer>,
    ) -> bool {
        let mut objectGroupString: String = String::new();
        let response = self.unwrapObject_str(toString16(objectId), object, context, if objectGroup.is_null() {
            None
        } else {
            Some(&mut objectGroupString)
        });

        if response.is_error() {
            if !error.is_null() {
                let msg = response.message();
                let string16 = String::from_utf8(msg.as_bytes().to_vec()).unwrap();
                *error = std::unique_ptr::UniquePtr::new(StringBuffer { data: string16 });
            }
            return false;
        }

        if !objectGroup.is_null() {
            *objectGroup = std::unique_ptr::UniquePtr::new(StringBuffer { data: objectGroupString });
        }

        true
    }

    pub fn wrapObject(
        &mut self,
        context: v8::Local<v8::Context>,
        value: v8::Local<v8::Value>,
        groupName: StringView,
        generatePreview: bool,
    ) -> std::unique_ptr::UniquePtr<protocol::Runtime::API::RemoteObject> {
        self.wrapObject_str(context, value, toString16(groupName), generatePreview)
    }

    pub fn triggerPreciseCoverageDeltaUpdate(&mut self, occasion: StringView) {
        if let Some(profiler_agent) = &mut self.m_profilerAgent {
            profiler_agent.triggerPreciseCoverageDeltaUpdate(toString16(occasion));
        }
    }

    pub fn evaluate(
        &mut self,
        context: v8::Local<v8::Context>,
        expression: StringView,
        includeCommandLineAPI: bool,
    ) -> EvaluateResult {
        let isolate = unsafe { (*self.m_inspector).isolate() };
        let handleScope = v8::EscapableHandleScope::new(isolate);

        let mut scope = InjectedScript::ContextScope::new(self, InspectedContext::contextId(context));

        if scope.initialize().is_error() {
            return EvaluateResult { /* ... */ };
        }

        scope.allowCodeGenerationFromStrings();
        scope.setTryCatchVerbose();

        if includeCommandLineAPI {
            scope.installCommandLineAPI();
        }

        let mut maybeResultValue: v8::MaybeLocal<v8::Value> = v8::MaybeLocal {local: v8::Local{value : 0}};
        {
            let microtasksScope = v8::MicrotasksScope::new();
            let source = toV8String(isolate, expression);
            maybeResultValue = v8::debug::EvaluateGlobal(isolate, source, v8::debug::EvaluateGlobalMode::kDefault, false);
        }

        if scope.tryCatch().HasCaught() {
            let exception = handleScope.Escape(scope.tryCatch().Exception());
            return EvaluateResult { /* ... */ };
        }

        let mut result: v8::Local<v8::Value> = v8::Local{value : 0};
        maybeResultValue.ToLocal(&mut result);

        let escapedResult = handleScope.Escape(result);
        EvaluateResult { /* ... */ }
    }

    pub fn stop(&mut self) {
        if let Some(debugger_agent) = &mut self.m_debuggerAgent {
            debugger_agent.stop();
        }
    }

    pub fn clientTrustLevel(&self) -> V8Inspector::ClientTrustLevel {
        self.m_clientTrustLevel
    }
    fn agentState(&mut self, name: String) -> Box<protocol::DictionaryValue> {
        if self.m_state.data.contains_key(&name) {
            let value = self.m_state.data.get_mut(&name).unwrap();
            if let protocol::Value::Dictionary(dict) = &mut **value {
                return dict.clone();
            }
        }

        let new_state = protocol::DictionaryValue::create();
        self.m_state.data.insert(name.clone(), Box::new(protocol::Value::Dictionary(new_state.clone())));
        return new_state;
    }

    fn serializeForFrontend(&mut self, message: Box<protocol::DictionaryValue>) -> std::unique_ptr::UniquePtr<StringBuffer> {
        let cbor = message.serialize();
        let mut json: Vec<u8> = Vec::new();
        let status = v8_crdtp::json::ConvertCBORToJSON(v8_crdtp::SpanFrom(cbor), &mut json);
        let string16 = String::from_utf8(json).unwrap();
        std::unique_ptr::UniquePtr::new(StringBuffer{data: string16})
    }

    fn SendProtocolResponse(&mut self, callId: i32, message: Box<dyn protocol::Serializable>) {
        let serialized = self.serializeForFrontend(Box::new(protocol::DictionaryValue::new()));
        unsafe { (*self.m_channel).sendResponse(callId, serialized) };
    }

    fn SendProtocolNotification(&mut self, message: Box<dyn protocol::Serializable>) {
        let serialized = self.serializeForFrontend(Box::new(protocol::DictionaryValue::new()));
        unsafe { (*self.m_channel).sendNotification(serialized) };
    }

    fn FallThrough(&mut self, callId: i32, method: v8_crdtp::Span<u8>, message: v8_crdtp::Span<u8>) {
        unreachable!();
    }

    fn FlushProtocolNotifications(&mut self) {
        unsafe { (*self.m_channel).flushProtocolNotifications() };
    }

    fn supportedDomainsImpl(&self) -> Vec<std::unique_ptr::UniquePtr<protocol::Schema::Domain>> {
        let mut result: Vec<std::unique_ptr::UniquePtr<protocol::Schema::Domain>> = Vec::new();
        result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::Domain {}));
        result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::Domain {}));
        result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::Domain {}));
        result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::Domain {}));
        result.push(std::unique_ptr::UniquePtr::new(protocol::Schema::Domain {}));
        result
    }

    fn releaseObjectGroup_str(&mut self, objectGroup: String) {
        let sessionId = self.m_sessionId;
        let context_group_id = self.m_contextGroupId;
        let inspector = self.m_inspector;
        unsafe {
            (*inspector).forEachContext(context_group_id, &mut |context| {
                let injectedScript = (*context).getInjectedScript(sessionId);
                if !injectedScript.is_null() {
                    (*injectedScript).releaseObjectGroup(objectGroup.clone());
                }
            });
        }
    }

    fn unwrapObject_str(
        &mut self,
        objectId: String,
        object: &mut v8::Local<v8::Value>,
        context: &mut v8::Local<v8::Context>,
        objectGroup: Option<&mut String>,
    ) -> protocol::Response {
        let mut remoteId: std::unique_ptr::UniquePtr<RemoteObjectId> = std::unique_ptr::UniquePtr::new(RemoteObjectId{});
        let response = RemoteObjectId::parse(objectId, &mut remoteId);
        if response.is_error() {
            return response;
        }

        let mut injectedScript: *mut InjectedScript = std::ptr::null_mut();
        let response = self.findInjectedScript_remote(remoteId.get(), &mut injectedScript);
        if response.is_error() {
            return response;
        }

        let response = unsafe { (*injectedScript).findObject(*remoteId.get(), object) };
        if response.is_error() {
            return response;
        }

        *context = unsafe { (*(*injectedScript).context()).context() };
        if let Some(group) = objectGroup {
            *group = unsafe { (*injectedScript).objectGroupName(*remoteId.get()) };
        }

        protocol::Response::Success()
    }

    fn wrapObject_str(
        &mut self,
        context: v8::Local<v8::Context>,
        value: v8::Local<v8::Value>,
        groupName: String,
        generatePreview: bool,
    ) -> std::unique_ptr::UniquePtr<protocol::Runtime::API::RemoteObject> {
        let mut injectedScript: *mut InjectedScript = std::ptr::null_mut();
        self.findInjectedScript(InspectedContext::contextId(context), &mut injectedScript);

        if injectedScript.is_null() {
            return std::unique_ptr::UniquePtr::new(protocol::Runtime::API::RemoteObject {});
        }

        let mut result: std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject> =
            std::unique_ptr::UniquePtr::new(protocol::Runtime::RemoteObject {});

        unsafe {
            (*injectedScript).wrapObject(
                value,
                groupName,
                WrapOptions { preview: generatePreview },
                &mut result,
            );
        }

        result
    }

    const kInspectedObjectBufferSize: u32 = 5;
}

impl Drop for V8InspectorSessionImpl {
    fn drop(&mut self) {
        let mut scope = unsafe {v8::Isolate::

// src/inspector/v8-inspector-session-impl.rs

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
// use std::sync::Mutex; // Consider using Mutex for thread-safe access if needed

// Placeholder for crdtp crate (replace with actual crate if available)
mod crdtp {
    pub mod cbor {
        #[derive(Debug, PartialEq)]
        pub enum Status {
            Ok,
            Error(String),
        }

        impl Status {
            pub fn ok(&self) -> bool {
                *self == Status::Ok
            }

            pub fn to_ascii_string(&self) -> String {
                match self {
                    Status::Ok => "OK".to_string(),
                    Status::Error(msg) => msg.clone(),
                }
            }
        }

        pub fn check_cbor_message(_bytes: &[u8]) -> Status {
            // Placeholder implementation
            Status::Ok
        }
    }

    pub mod json {
        use super::crdtp::cbor::Status;
        pub fn convert_cbor_to_json(_cbor: &[u8], json: &mut Vec<u8>) -> Status {
            // Placeholder implementation - needs actual CBOR to JSON conversion logic
            // For now just copy the input for demonstration purposes
            *json = _cbor.to_vec();
            Status::Ok
        }

        pub fn convert_json_to_cbor(_json: &[u8], cbor: &mut Vec<u8>) -> Status {
            // Placeholder implementation - needs actual JSON to CBOR conversion logic
            // For now just copy the input for demonstration purposes
            *cbor = _json.to_vec();
            Status::Ok
        }
    }

    pub mod dispatch {
        pub trait DispatchableBase {
            fn call_id(&self) -> Option<i32>;
            fn dispatch_error(&self) -> String;
            fn ok(&self) -> bool;
            fn has_call_id(&self) -> bool;
        }

        pub struct Dispatchable<'a> {
            cbor: &'a [u8], // Placeholder - store the CBOR message for now
            is_valid: bool,
            error: String,
            call_id: Option<i32>,
        }

        impl<'a> Dispatchable<'a> {
            pub fn new(cbor: &'a [u8]) -> Self {
                // Dummy implementation
                Dispatchable {
                    cbor,
                    is_valid: true,
                    error: String::new(),
                    call_id: Some(123),
                }
            }

            pub fn dispatch_error(&self) -> String {
                self.error.clone()
            }

            pub fn ok(&self) -> bool {
                self.is_valid
            }

            pub fn has_call_id(&self) -> bool {
                self.call_id.is_some()
            }

            pub fn call_id(&self) -> Option<i32> {
                self.call_id
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum DispatchResponse {
            ParseError(String),
        }

        pub fn create_error_notification(response: DispatchResponse) -> Box<dyn Serializable> {
            // Placeholder implementation
            Box::new(ErrorNotification {
                message: format!("Error: {:?}", response),
            })
        }

        pub fn create_error_response(call_id: Option<i32>, error: String) -> Box<dyn Serializable> {
             // Placeholder implementation
            Box::new(ErrorResponse {
                call_id,
                message: error,
            })
        }

        pub trait DispatcherBase {
            fn dispatch(&mut self, dispatchable: Dispatchable) -> DispatchResult;
        }

        pub struct Dispatcher<'a> {
            session: &'a mut V8InspectorSessionImpl,
        }

        impl<'a> Dispatcher<'a> {
            pub fn new(session: &'a mut V8InspectorSessionImpl) -> Self {
                Dispatcher { session }
            }
        }

        impl<'a> DispatcherBase for Dispatcher<'a> {
            fn dispatch(&mut self, _dispatchable: Dispatchable) -> DispatchResult {
                 // Placeholder implementation
                 DispatchResult::new()
            }
        }

        pub struct DispatchResult {
            // Placeholder struct for representing the result of a dispatch operation
        }

        impl DispatchResult {
            pub fn new() -> Self {
                DispatchResult {}
            }

            pub fn run(self) {
                 // Placeholder implementation
            }
        }

        pub trait Serializable {
            fn serialize(&self) -> Vec<u8>;
        }

        struct ErrorNotification {
            message: String,
        }

        impl Serializable for ErrorNotification {
            fn serialize(&self) -> Vec<u8> {
                self.message.as_bytes().to_vec()
            }
        }

        struct ErrorResponse {
            call_id: Option<i32>,
            message: String,
        }

        impl Serializable for ErrorResponse {
            fn serialize(&self) -> Vec<u8> {
                self.message.as_bytes().to_vec()
            }
        }
    }

    pub type Span<'a, T> = &'a [T];
    pub fn span_from<T>(v: &[T]) -> Span<T> {
        v
    }
}

// Placeholder for v8 crate (replace with actual crate if available)
mod v8 {
    pub struct Context {}

    impl Context {
        pub fn new() -> Self {
            Context {}
        }
    }
}

// Placeholder for v8-microtask-queue crate (replace with actual crate if available)
mod v8_microtask_queue {}

// Placeholder for base crate (replace with actual crate if available)
mod base {
    #[macro_export]
    macro_rules! unreachable {
        () => {
            panic!("UNREACHABLE")
        };
    }
}

// Placeholder for inspector protocol
mod protocol {
    pub mod runtime {
        pub mod api {
            pub struct RemoteObject {}
        }

        pub struct Metainfo {}

        impl Metainfo {
            pub const command_prefix: &'static str = "Runtime.";
            pub const domain_name: &'static str = "Runtime";
            pub const version: &'static str = "1.3";
        }
        // Placeholder definitions - replace with actual protocol definitions
        pub struct RemoteObject {
            pub value: String,
        }
    }

    pub mod debugger {
        pub mod api {
            pub struct SearchMatch {}
        }
        pub struct Metainfo {}
        impl Metainfo {
            pub const command_prefix: &'static str = "Debugger.";
            pub const domain_name: &'static str = "Debugger";
            pub const version: &'static str = "1.3";
        }

        pub struct SearchMatch {}
    }

    pub mod profiler {
        pub struct Metainfo {}
        impl Metainfo {
            pub const command_prefix: &'static str = "Profiler.";
            pub const domain_name: &'static str = "Profiler";
            pub const version: &'static str = "1.3";
        }
    }

    pub mod heap_profiler {
        pub struct Metainfo {}
        impl Metainfo {
            pub const command_prefix: &'static str = "HeapProfiler.";
            pub const domain_name: &'static str = "HeapProfiler";
            pub const version: &'static str = "1.3";
        }
    }

    pub mod console {
        pub struct Metainfo {}
        impl Metainfo {
            pub const command_prefix: &'static str = "Console.";
            pub const domain_name: &'static str = "Console";
            pub const version: &'static str = "1.3";
        }
    }

    pub mod schema {
        pub struct Metainfo {}
        impl Metainfo {
            pub const command_prefix: &'static str = "Schema.";
            pub const domain_name: &'static str = "Schema";
            pub const version: &'static str = "1.3";
        }

        pub struct Domain {}
    }

    // Placeholder definitions - replace with actual protocol definitions
    pub mod Protocol {
        use std::collections::HashMap;

        pub trait Value {
            fn parse_binary(_data: &[u8], _size: usize) -> Box<dyn Value>;
        }

        pub trait DictionaryValue: Value {
            fn create() -> Box<dyn DictionaryValue>;
            fn get_object(&self, name: &str) -> Option<&dyn DictionaryValue>;
            fn set_object(&mut self, name: &str, value: Box<dyn DictionaryValue>);
            fn get_boolean(&self, name: &str) -> Option<bool>;
            fn set_boolean(&mut self, name: &str, value: bool);
            fn cast(value: Box<dyn Value>) -> Option<Box<dyn DictionaryValue>>;
        }

        pub struct DictionaryValueImpl {
            data: HashMap<String, Box<dyn Value>>,
            use_binary_protocol: bool,
        }

        impl Value for DictionaryValueImpl {
            fn parse_binary(_data: &[u8], _size: usize) -> Box<dyn Value> {
                DictionaryValueImpl::create() as Box<dyn Value>
            }
        }

        impl DictionaryValue for DictionaryValueImpl {
            fn create() -> Box<dyn DictionaryValue> {
                Box::new(DictionaryValueImpl {
                    data: HashMap::new(),
                    use_binary_protocol: false,
                })
            }

            fn get_object(&self, name: &str) -> Option<&dyn DictionaryValue> {
                self.data
                    .get(name)
                    .and_then(|v| DictionaryValue::cast(Value::parse_binary(&[], 0)))
                    .as_deref()
            }

            fn set_object(&mut self, name: &str, value: Box<dyn DictionaryValue>) {
                self.data.insert(name.to_string(), value as Box<dyn Value>);
            }

            fn get_boolean(&self, name: &str) -> Option<bool> {
                if name == "use_binary_protocol" {
                    Some(self.use_binary_protocol)
                } else {
                    None
                }
            }

            fn set_boolean(&mut self, name: &str, value: bool) {
                if name == "use_binary_protocol" {
                    self.use_binary_protocol = value;
                }
            }

            fn cast(value: Box<dyn Value>) -> Option<Box<dyn DictionaryValue>> {
                Some(value.downcast::<DictionaryValueImpl>().unwrap())
            }
        }
    }

    pub mod Schema {
        pub mod API {
            pub struct Domain {}
        }
        pub struct Domain {}
        impl Domain {
            pub fn create() -> DomainBuilder {
                DomainBuilder::new()
            }
        }

        pub struct DomainBuilder {
            name: Option<String>,
            version: Option<String>,
        }

        impl DomainBuilder {
            pub fn new() -> Self {
                DomainBuilder {
                    name: None,
                    version: None,
                }
            }

            pub fn set_name(mut self, name: &'static str) -> Self {
                self.name = Some(name.to_string());
                self
            }

            pub fn set_version(mut self, version: &'static str) -> Self {
                self.version = Some(version.to_string());
                self
            }

            pub fn build(self) -> Domain {
                Domain {}
            }
        }
    }
}

// Placeholder for remote-object-id crate (replace with actual crate if available)
mod remote_object_id {
    pub struct RemoteObjectIdBase {}
}

// Placeholder for search-util crate (replace with actual crate if available)
mod search_util {
    use crate::protocol::debugger::SearchMatch;
    use crate::string_util::String16;
    //Dummy Implementation
    pub fn searchInTextByLinesImpl(
        _session: &V8InspectorSessionImpl,
        _text: String16,
        _query: String16,
        _caseSensitive: bool,
        _isRegex: bool,
    ) -> Vec<std::unique_ptr::UniquePtr<SearchMatch>> {
        let result: Vec<std::unique_ptr::UniquePtr<SearchMatch>> = Vec::new();
        result
    }
}

// Placeholder for string-util crate
mod string_util {
    use std::ffi::OsStr;
    use std::path::Path;

    #[derive(Clone, Debug)]
    pub struct StringView<'a> {
        data: &'a [u8],
    }

    impl<'a> StringView<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            StringView { data }
        }

        pub fn is8Bit(&self) -> bool {
            true // Assuming UTF-8 for simplicity
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }

        pub fn characters8(&self) -> &'a [u8] {
            self.data
        }

        pub fn characters16(&self) -> &'a [u8] {
            self.data
        }
    }

    #[derive(Clone, Debug)]
    pub struct String16 {
        data: Vec<u16>,
    }

    impl String16 {
        pub fn from_utf8(data: &[u8], size: usize) -> Self {
            let utf8_str = String::from_utf8_lossy(data);
            let utf16_iter = utf8_str.encode_utf16();
            let data: Vec<u16> = utf16_iter.collect();
            String16 { data }
        }
    }

    pub fn stringViewStartsWith(method: StringView, prefix: &'static str) -> bool {
        let method_str = String::from_utf8_lossy(method.characters8());
        method_str.starts_with(prefix)
    }

    pub fn toString16(s: StringView) -> String16 {
        let utf8_str = String::from_utf8_lossy(s.characters8());
        let utf16_iter = utf8_str.encode_utf16();
        let data: Vec<u16> = utf16_iter.collect();
        String16 { data }
    }
    pub fn to_string(string16: String16) -> String {
        String::from_utf16_lossy(&string16.data)
    }
}

// Placeholder for other agent implementations
mod v8_console_agent_impl {
    use crate::protocol::console::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8ConsoleAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8ConsoleAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String) -> Self {
            V8ConsoleAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
        pub fn restore(&mut self) {}
        pub fn disable(&mut self) {
            self.enabled = false;
        }
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8ConsoleAgentImpl) {}
    }
}
mod v8_debugger_agent_impl {
    use crate::protocol::debugger::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8DebuggerAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8DebuggerAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String) -> Self {
            V8DebuggerAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
        pub fn restore(&mut self) {}
        pub fn disable(&mut self) {
            self.enabled = false;
        }
        pub fn reset(&mut self) {}
        pub fn schedulePauseOnNextStatement(&mut self, reason: String16, details: Box<dyn crate::protocol::Protocol::DictionaryValue>) {}
        pub fn cancelPauseOnNextStatement(&mut self) {}
        pub fn breakProgram(&mut self, reason: String16, details: Box<dyn crate::protocol::Protocol::DictionaryValue>) {}
        pub fn setSkipAllPauses(&mut self, skip: bool) {}
        pub fn resume(&mut self, terminateOnResume: bool) {}
        pub fn stepOver(&mut self, o: ()) {}
        pub fn stop(&mut self) {}
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8DebuggerAgentImpl) {}
    }
}
mod v8_debugger_barrier {
    pub struct V8DebuggerBarrier {}
}
mod v8_debugger {}
mod v8_heap_profiler_agent_impl {
    use crate::protocol::heap_profiler::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8HeapProfilerAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8HeapProfilerAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String) -> Self {
            V8HeapProfilerAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
        pub fn restore(&mut self) {}
        pub fn disable(&mut self) {
            self.enabled = false;
        }
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8HeapProfilerAgentImpl) {}
    }
}
mod v8_inspector_impl {
    use crate::v8_inspector_session_impl::InspectedContext;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8InspectorImpl {
        isolate_id: i32,
    }
    impl V8InspectorImpl {
        pub fn new() -> Self {
            V8InspectorImpl {
                isolate_id: 123,
            }
        }

        pub fn disconnect(&self, session: *const V8InspectorSessionImpl) {}
        pub fn forEachContext<F>(&self, group_id: i32, mut callback: F)
            where F: FnMut(&InspectedContext) {
        }
        pub fn getContext(&self, group_id: i32, context_id: i32) -> Option<&InspectedContext> {
            None
        }
        pub fn isolateId(&self) -> i32 {
            self.isolate_id
        }
        pub fn isolate(&self) -> &V8InspectorImpl {
            self
        }
    }
    // Dummy Implementations
    pub enum ClientTrustLevel {
        kFullyTrusted,
    }
    pub trait Channel {
        fn sendResponse(&mut self, callId: i32, message: Box<StringBuffer>);
        fn sendNotification(&mut self, message: Box<StringBuffer>);
        fn flushProtocolNotifications(&mut self);
    }
    pub struct StringBuffer {}
}
mod v8_profiler_agent_impl {
    use crate::protocol::profiler::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8ProfilerAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8ProfilerAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String) -> Self {
            V8ProfilerAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
        pub fn restore(&mut self) {}
        pub fn disable(&mut self) {
            self.enabled = false;
        }
        pub fn triggerPreciseCoverageDeltaUpdate(&mut self, occasion: String16) {}
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8ProfilerAgentImpl) {}
    }
}
mod v8_runtime_agent_impl {
    use crate::protocol::runtime::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8RuntimeAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8RuntimeAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String, debugger_barrier: std::shared_ptr::SharedPtr<crate::v8_debugger_barrier::V8DebuggerBarrier>) -> Self {
            V8RuntimeAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
        pub fn restore(&mut self) {}
        pub fn disable(&mut self) {
            self.enabled = false;
        }
        pub fn reset(&mut self) {}
        pub fn reportExecutionContextCreated(&mut self, context: &InspectedContext) {}
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8RuntimeAgentImpl) {}
    }
}
mod v8_schema_agent_impl {
    use crate::protocol::schema::Metainfo;
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::V8InspectorSessionImpl;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct V8SchemaAgentImpl {
        session: Rc<RefCell<V8InspectorSessionImpl>>,
        enabled: bool,
        state: String,
    }
    impl V8SchemaAgentImpl {
        pub fn new(session: Rc<RefCell<V8InspectorSessionImpl>>, owner: &V8InspectorSessionImpl, agent_state: String) -> Self {
            V8SchemaAgentImpl {
                session,
                enabled: false,
                state: agent_state,
            }
        }
    }
    pub struct Dispatcher {}
    impl Dispatcher {
        pub fn wire(dispatcher: &mut super::v8_inspector_session_impl::Dispatcher, agent: *mut V8SchemaAgentImpl) {}
    }
}
mod injected_script {
    use crate::protocol::runtime::{RemoteObject, self};
    use crate::string_util::String16;
    use crate::v8_inspector_session_impl::InspectedContext;
    use std::any::Any;

    pub struct InjectedScript {
        context: Rc<InspectedContext>,
        custom_object_formatter_enabled: bool,
    }

    impl InjectedScript {
        pub fn new(context: Rc<InspectedContext>) -> Self {
            InjectedScript {
                context,
                custom_object_formatter_enabled: false,
            }
        }

        pub fn context(&self) -> &InspectedContext {
            &self.context
        }

        pub fn releaseObjectGroup(&self, objectGroup: String16) {}

        pub fn setCustomObjectFormatterEnabled(&mut self, enabled: bool) {
            self.custom_object_formatter_enabled = enabled;
        }

        pub fn wrapObject(
            &self,
            _value: &dyn Any,
            _groupName: String16,
            _options: WrapOptions,
            result: &mut Option<RemoteObject>,
        ) {
            *result = Some(RemoteObject {
                value: "Dummy Value".to_string(),
            });
        }
    }

    pub struct WrapOptions {
        wrap_mode: Vec<WrapMode>,
    }

    impl WrapOptions {
        pub fn new(wrap_mode: Vec<WrapMode>) -> Self {
            WrapOptions { wrap_mode }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum WrapMode {
        kIdOnly,
        kPreview,
    }
    //Dummy Implementation
    pub fn WrapOptions(options: std::vec::Vec<WrapMode>) -> WrapOptions {
        WrapOptions{wrap_mode: options}
    }
}

mod inspected_context {
    use crate::injected_script::InjectedScript;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct InspectedContext {
        context_id: i32,
        context: String,
        injected_script: RefCell<Option<Rc<InjectedScript>>>,
    }

    impl InspectedContext {
        pub fn new(context_id: i32, context: String) -> Self {
            InspectedContext {
                context_id,
                context,
                injected_script: RefCell::new(None),
            }
        }

        pub fn contextId(context: &String) -> i32 {
            123 //Dummy Implementation
        }

        pub fn context(&self) -> &String {
            &self.context
        }

        pub fn getInjectedScript(&self, session_id: i32) -> Option<Rc<InjectedScript>> {
            self.injected_script.borrow().clone()
        }

        pub fn createInjectedScript(&self, session_id: i32) -> Rc<InjectedScript> {
            let injected_script = Rc::new(InjectedScript::new(Rc::new(InspectedContext::new(123, String::from("Dummy Context"))))); //Rc::new(self) is a placeholder. Replace with self
            *self.injected_script.borrow_mut() = Some(injected_script.clone());
            injected_script
        }

        pub fn discardInjectedScript(&self, session_id: i32) {
            self.injected_script.borrow_mut().take();
        }
    }
}

mod v8_inspector_session {
    use crate::string_util::StringView;
    use std::any::Any;

    pub trait Inspectable {}

    pub enum EvaluateResultType {
        kNotRun,
        kSuccess,
        kException,
    }

    pub struct EvaluateResult {
        pub result_type: EvaluateResultType,
        pub value: Box<dyn Any>,
    }

    pub trait V8InspectorSession {
        fn evaluate(
            &self,
            context: &dyn Any,
            expression: StringView,
            includeCommandLineAPI: bool,
        ) -> EvaluateResult;
    }
}

use crate::crdtp::{
    cbor::CheckCBORMessage,
    cbor::Status,
    dispatch::{
        create_error_notification, create_error_response, Dispatcher as CrdtpDispatcher,
        DispatcherBase, Serializable, Dispatchable,
    },
    json::{convert_cbor_to_json, convert_json_to_cbor},
    span_from, Span,
};
use crate::protocol::{
    console::Metainfo as ConsoleMetainfo,
    debugger::Metainfo as DebuggerMetainfo,
    heap_profiler::Metainfo as HeapProfilerMetainfo,
    profiler::Metainfo as ProfilerMetainfo,
    runtime::Metainfo as RuntimeMetainfo,
    schema::Metainfo as SchemaMetainfo,
    Protocol::{DictionaryValue, Value},
};
use crate::string_util::{stringViewStartsWith, String16, StringView, toString16};
use crate::v8_console_agent_impl::V8ConsoleAgentImpl;
use crate::v8_debugger_agent_impl::V8DebuggerAgentImpl;
use crate::v8_debugger_barrier::V8DebuggerBarrier;
use crate::v8_heap_profiler_agent_impl::V8HeapProfilerAgentImpl;
use crate::v8_inspector_impl::{Channel, ClientTrustLevel, V8InspectorImpl, StringBuffer};
use crate::v8_profiler_agent_impl::V8ProfilerAgentImpl;
use crate::v8_runtime_agent_impl::V8RuntimeAgentImpl;
use crate::v8_schema_agent_impl::V8SchemaAgentImpl;
use std::boxed::Box;
use std::rc::Rc;
use std::vec::Vec;
use crate::injected_script::InjectedScript;
use crate::inspected_context::InspectedContext;
use crate::remote_object_id::RemoteObjectIdBase;
use std::unique_ptr::UniquePtr;
use crate::search_util::searchInTextByLinesImpl;
use crate::v8_inspector_session::V8InspectorSession;
use std::any::Any;
use std::borrow::BorrowMut;
use std::sync::Arc;

pub struct Response {
    success: bool,
    message: String,
}

impl Response {
    pub fn Success() -> Self {
        Response {
            success: true,
            message: String::new(),
        }
    }

    pub fn ServerError(message: &str) -> Self {
        Response {
            success: false,
            message: message.to_string(),
        }
    }

    pub fn IsSuccess(&self) -> bool {
        self.success
    }

    pub fn IsError(&self) -> bool {
        !self.success
    }

    pub fn Message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct StringBufferFromError {
    message: String,
}

impl From<std::string::FromUtf8Error> for StringBufferFromError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        StringBufferFromError {
            message: error.to_string(),
        }
    }
}
pub fn StringBufferFrom(string16: String16) -> Box<StringBuffer> {
    Box::new(StringBuffer {})
}
pub fn StringBufferFrom_vec(vec: Vec<u8>) -> Box<StringBuffer> {
    Box::new(StringBuffer {})
}

pub mod v8_context_info {
    use crate::inspected_context::InspectedContext;
    use crate::v8::Context;

    pub fn execution_context_id(_context: &Context) -> i32 {
        //InspectedContext::contextId(context) //Needs Conversion Logic
        0
    }
}

pub struct V8InspectorSessionImpl {
    m_contextGroupId: i32,
    m_sessionId: i32,
    m_inspector: *const V8InspectorImpl, //raw pointer
    m_channel: *mut dyn Channel, //raw pointer
    m_customObjectFormatterEnabled: bool,
    m_dispatcher: CrdtpDispatcher<'static>,
    m_state: Box<dyn DictionaryValue>,
    m_runtimeAgent: Rc<RefCell<Option<V8RuntimeAgentImpl>>>,
    m_debuggerAgent: Rc<RefCell<Option<V8DebuggerAgentImpl>>>,
    m_heapProfilerAgent: Rc<RefCell<Option<V8HeapProfilerAgentImpl>>>,
    m_profilerAgent: Rc<RefCell<Option<V8ProfilerAgentImpl>>>,
    m_consoleAgent: Rc<RefCell<Option<V8ConsoleAgentImpl>>>,
    m_schemaAgent: Rc<RefCell<Option<V8SchemaAgentImpl>>>,
    m_clientTrustLevel: ClientTrustLevel,
    m_inspectedObjects: VecDeque<Box<dyn v8_inspector_session::Inspectable>>,
    use_binary_protocol_: bool,
}

const K_INSPECTED_OBJECT_BUFFER_SIZE: usize = 50; // Example size

impl V8InspectorSessionImpl {
    pub fn canDispatchMethod(method: StringView) -> bool {
        stringViewStartsWith(method, RuntimeMetainfo::command_prefix)
            || stringViewStartsWith(method, DebuggerMetainfo::command_prefix)
            || stringViewStartsWith(method, ProfilerMetainfo::command_prefix)
            || stringViewStartsWith(method, HeapProfilerMetainfo::command_prefix)
            || stringViewStartsWith(method, ConsoleMetainfo::command_prefix)
            || stringViewStartsWith(method, SchemaMetainfo::command_prefix)
    }

    pub fn create(
        inspector: *const V8InspectorImpl, //raw pointer
        contextGroupId: i32,
        sessionId: i32,
        channel: *mut dyn Channel, //raw pointer
        state: StringView,
        clientTrustLevel: ClientTrustLevel,
        debuggerBarrier: std::shared_ptr::SharedPtr
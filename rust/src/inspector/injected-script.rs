#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};

//use third_party::inspector_protocol::crdtp::json; // Assuming this is a JSON serialization/deserialization library
//use include::v8_container; // Assuming this is related to V8's container types (e.g., Array, Map)
//use include::v8_context; // Assuming this is related to V8's context management
//use include::v8_function; // Assuming this is related to V8's function objects
//use include::v8_inspector; // Assuming this is related to the overall V8 Inspector interface
//use include::v8_microtask_queue; // Assuming this is related to V8's microtask queue
//use src::debug::debug_interface; // Assuming this provides debugging-related interfaces for V8
//use src::inspector::custom_preview; // Assuming this handles custom previews in the inspector
//use src::inspector::inspected_context;
//use src::inspector::protocol::Protocol;
//use src::inspector::remote_object_id;
//use src::inspector::string_util;
//use src::inspector::v8_console;
//use src::inspector::v8_debugger;
//use src::inspector::v8_inspector_impl;
//use src::inspector::v8_inspector_session_impl;
//use src::inspector::v8_serialization_duplicate_tracker;
//use src::inspector::v8_stack_trace_impl;
//use src::inspector::v8_value_utils;
//use src::inspector::value_mirror;

// Mock definitions for the V8 and Inspector APIs.  These would need to be replaced
// with proper Rust bindings to V8.
mod v8 {
    pub struct Isolate {}
    impl Isolate {
        pub fn new() -> Isolate { Isolate {} }
    }
    pub struct Context {}
    impl Context {
        pub fn new(_isolate: &Isolate) -> Context { Context {} }
    }
    pub struct Value {}
    impl Value {
        pub fn is_object(&self) -> bool { false }
        pub fn to_object(&self) -> Option<Object> { None }
    }
    pub struct Object {}
    pub struct String {}
}

mod inspector_protocol {
    pub mod runtime {
        pub struct RemoteObject {}
        pub struct ExceptionDetails {}
        pub struct PropertyDescriptor {}
        pub struct InternalPropertyDescriptor {}
        pub struct PrivatePropertyDescriptor {}
        pub struct CallArgument {}
    }
    pub mod protocol {
        pub struct DispatchResponse {}
        pub mod runtime {
            pub struct RemoteObject {}
            impl RemoteObject {
                pub fn create() -> RemoteObjectBuilder { RemoteObjectBuilder {}}
            }
            pub struct RemoteObjectBuilder {}
            impl RemoteObjectBuilder {
                pub fn build(self) -> RemoteObject { RemoteObject {}}
            }
            pub struct ExceptionDetails {}
            impl ExceptionDetails {
                pub fn create() -> ExceptionDetailsBuilder { ExceptionDetailsBuilder {}}
            }
            pub struct ExceptionDetailsBuilder {}
            impl ExceptionDetailsBuilder {
                pub fn build(self) -> ExceptionDetails { ExceptionDetails {}}
                pub fn setExceptionId(&mut self, id: i32) -> &mut Self {self}
                pub fn setText(&mut self, text: String) -> &mut Self {self}
                pub fn setLineNumber(&mut self, line_number: i32) -> &mut Self {self}
                pub fn setColumnNumber(&mut self, column_number: i32) -> &mut Self {self}
            }
        }
    }
}

type String16 = String;

mod crdtp {
    pub mod json {
        pub fn ConvertCBORToJSON(span: Vec<u8>, json: &mut Vec<u8>) {}
        pub fn SpanFrom(value: &Vec<u8>) -> Vec<u8> { value.clone() }
    }
}

mod protocol {
    pub mod runtime {
        pub mod protocol {
            pub struct DispatchResponse {}
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

                pub fn getProperties(&self) -> &Vec<T> {
                    &self.items
                }
            }

        }
        pub struct RemoteObject {}
        pub struct PropertyDescriptor {}
        pub struct InternalPropertyDescriptor {}
        pub struct PrivatePropertyDescriptor {}
        pub struct ExceptionDetails {}
        impl ExceptionDetails {
            pub fn create() -> ExceptionDetailsBuilder { ExceptionDetailsBuilder { exception_id: 0, text: String::new()}}
        }
        pub struct ExceptionDetailsBuilder {
            exception_id: i32,
            text: String::new(),
        }
        impl ExceptionDetailsBuilder {
            pub fn setExceptionId(&mut self, id: i32) -> &mut Self {
                self.exception_id = id;
                self
            }
            pub fn setText(&mut self, text: String) -> &mut Self {
                self.text = text;
                self
            }
            pub fn setLineNumber(&mut self, line_number: i32) -> &mut Self {
                self
            }
            pub fn setColumnNumber(&mut self, column_number: i32) -> &mut Self {
                self
            }
            pub fn build(self) -> ExceptionDetails {
                ExceptionDetails {}
            }
        }
        pub struct CallArgument {
            object_id: Option<String>,
            value: Option<Vec<u8>>,
            unserializable_value: Option<String>,
        }

        impl CallArgument {
            pub fn new() -> Self {
                CallArgument {
                    object_id: None,
                    value: None,
                    unserializable_value: None,
                }
            }

            pub fn setObjectId(&mut self, object_id: String) {
                self.object_id = Some(object_id);
            }

            pub fn setValue(&mut self, value: Vec<u8>) {
                self.value = Some(value);
            }

            pub fn setUnserializableValue(&mut self, value: String) {
                self.unserializable_value = Some(value);
            }

            pub fn hasObjectId(&self) -> bool {
                self.object_id.is_some()
            }

            pub fn hasValue(&self) -> bool {
                self.value.is_some()
            }

            pub fn hasUnserializableValue(&self) -> bool {
                self.unserializable_value.is_some()
            }

            pub fn getObjectId(&self, _default: &str) -> String {
                self.object_id.clone().unwrap_or_default()
            }

            pub fn getValue(&self, _default: *mut ()) -> Option<&Vec<u8>> { //TODO replace void ptr with type
                self.value.as_ref()
            }

            pub fn getUnserializableValue(&self, _default: &str) -> String {
                self.unserializable_value.clone().unwrap_or_default()
            }
        }
    }

}

// Assuming these types and enums are defined elsewhere in the codebase
enum WrapMode {
    kIdOnly,
    kPreview,
    kDeep,
}

struct WrapOptions {
    mode: WrapMode,
    serializationOptions: SerializationOptions,
}
impl WrapOptions{
    fn new (mode: WrapMode) -> Self {
        WrapOptions{
            mode,
            serializationOptions: SerializationOptions::new(),
        }
    }
}

struct SerializationOptions {
    maxDepth: i32,
    additionalParameters: AdditionalParameters,
}

impl SerializationOptions {
    fn new () -> Self {
        SerializationOptions {
            maxDepth: 0,
            additionalParameters: AdditionalParameters::new(),
        }
    }
}

struct AdditionalParameters {}
impl AdditionalParameters {
    fn new () -> Self {
        AdditionalParameters {}
    }
    fn Get(&self, _isolate: &v8::Isolate) -> Self { // TODO define isolate
        AdditionalParameters {}
    }
}
struct Response {
    success: bool,
    error_message: Option<String>,
}

impl Response {
    fn Success() -> Self {
        Response {
            success: true,
            error_message: None,
        }
    }

    fn InternalError() -> Self {
        Response {
            success: false,
            error_message: Some("Internal Error".to_string()),
        }
    }

    fn ServerError(message: &str) -> Self {
        Response {
            success: false,
            error_message: Some(message.to_string()),
        }
    }

    fn IsSuccess(&self) -> bool {
        self.success
    }
}

struct RemoteObjectId {
    id: i32,
    context_id: i32,
    isolate_id: i32,
}

impl RemoteObjectId {
    fn parse(object_id: String, remote_id: &mut std::unique_ptr<RemoteObjectId>) -> Response {
        Response::Success() // Placeholder
    }
    fn serialize(isolate_id: i32, context_id: i32, id: i32) -> String {
        String::new() // Placeholder
    }
    fn id(&self) -> i32 {
        self.id
    }
    fn contextId(&self) -> i32 {
        self.context_id
    }
    fn isolateId(&self) -> i32 {
        self.isolate_id
    }
}

struct RemoteCallFrameId {
    frame_ordinal: i32,
}

impl RemoteCallFrameId {
    fn parse(remote_object_id: String, remote_id: &mut std::unique_ptr<RemoteCallFrameId>) -> Response {
        Response::Success() // Placeholder
    }

    fn frameOrdinal(&self) -> i32 {
        self.frame_ordinal
    }
}

struct ValueMirror {
}

impl ValueMirror {
    fn create(_context: &v8::Context, _value: &v8::Value) -> std::unique_ptr<ValueMirror> {
        std::unique_ptr::new(ValueMirror {})
    }
    fn buildRemoteObject(&self, _context: &v8::Context, _wrap_options: WrapOptions, _result: &mut std::unique_ptr<inspector_protocol::runtime::RemoteObject>) -> Response{
        Response::Success()
    }
    fn v8Value<'a>(&self, _isolate: &v8::Isolate) -> &'a v8::Value{
        static value: v8::Value = v8::Value {};
        &value
    }
    fn buildDeepSerializedValue(&self, _context: &v8::Context, _max_depth: i32, _additional_parameters: AdditionalParameters, _duplicate_tracker: V8SerializationDuplicateTracker, _deepSerializedValueDict: &mut std::unique_ptr<protocol::runtime::protocol::DispatchResponse>) -> Response{
        Response::Success()
    }
    fn getProperties(_context: &v8::Context, _object: &v8::Object, _ownProperties: bool, _accessorPropertiesOnly: bool, _nonIndexedPropertiesOnly: bool, _accumulator: &mut dyn ValueMirror::PropertyAccumulator) -> bool{
        true
    }
    fn getInternalProperties(_context: &v8::Context, _value_obj: &v8::Object, _internalPropertiesWrappers: &mut Vec<InternalPropertyMirror>) {}
    fn getPrivateProperties(_context: &v8::Context, _value_obj: &v8::Object, _accessorPropertiesOnly: bool) -> Vec<PrivatePropertyMirror>{
        Vec::new()
    }
    fn buildObjectPreview(&self, _context: &v8::Context, _generatePreviewForTable: bool, _limit1: &mut i32, _limit2: &mut i32, _preview: &mut std::unique_ptr<protocol::runtime::protocol::DispatchResponse>) {}
    trait PropertyAccumulator {
        fn Add(&mut self, mirror: PropertyMirror) -> bool;
    }
}

struct PropertyMirror{
    name: String16,
    configurable: bool,
    enumerable: bool,
    isOwn: bool,
    value: Option<std::unique_ptr<ValueMirror>>,
    writable: bool,
    getter: Option<std::unique_ptr<ValueMirror>>,
    setter: Option<std::unique_ptr<ValueMirror>>,
    symbol: Option<std::unique_ptr<ValueMirror>>,
    exception: Option<std::unique_ptr<ValueMirror>>,
}

struct InternalPropertyMirror {
    name: String16,
    value: std::unique_ptr<ValueMirror>,
}

struct PrivatePropertyMirror {
    name: String16,
    value: Option<std::unique_ptr<ValueMirror>>,
    getter: Option<std::unique_ptr<ValueMirror>>,
    setter: Option<std::unique_ptr<ValueMirror>>,
}

struct InspectedContext {
    context_id: i32,
    inspector: Box<V8InspectorImpl>, // Use Box for ownership
}

impl InspectedContext {
    fn new(context_id: i32, inspector: Box<V8InspectorImpl>) -> Self {
        InspectedContext {
            context_id,
            inspector,
        }
    }
    fn getInjectedScript(&self, session_id: i32) -> *mut InjectedScript {
        unimplemented!()
    }
    fn context(&self) -> &v8::Context {
        static context: v8::Context = v8::Context {};
        &context
    }
    fn contextId(&self) -> i32 {
        self.context_id
    }
    fn inspector(&self) -> &V8InspectorImpl {
        &self.inspector
    }
}

struct V8InspectorSessionImpl {}

impl V8InspectorSessionImpl {
    fn findInjectedScript(&self, execution_context_id: i32, injected_script: &mut *mut InjectedScript) -> Response {
        Response::Success() // Placeholder
    }

    fn findInjectedScript_RemoteId(&self, remote_id: &RemoteObjectId, injected_script: &mut *mut InjectedScript) -> Response {
        Response::Success() // Placeholder
    }
    fn clientTrustLevel(&self) -> V8InspectorTrustLevel {
        V8InspectorTrustLevel::kFullyTrusted
    }
    fn inspector(&self) -> &V8InspectorImpl {
        unimplemented!()
    }
    fn contextGroupId(&self) -> i32 {
        unimplemented!()
    }
    fn sessionId(&self) -> i32 {
        unimplemented!()
    }
}

enum V8InspectorTrustLevel {
    kFullyTrusted,
}

type EvaluateCallbackPtr = Arc<EvaluateCallback>;

struct EvaluateCallback {
    // Define the fields of the EvaluateCallback struct here,
    // based on what sendSuccess and sendFailure need to access.
}

impl EvaluateCallback {
    fn sendSuccess(
        callback: Weak<EvaluateCallback>,
        injected_script: *mut InjectedScript,
        result: std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
        exception_details: std::unique_ptr<inspector_protocol::runtime::ExceptionDetails>,
    ) {
        unsafe {
            if let Some(cb) = callback.upgrade() {
                //injected_script.deleteEvaluateCallback(Arc::clone(&cb));
                //assert_eq!(Arc::strong_count(&cb), 1);
                cb.sendSuccess_internal(result, exception_details);
            }
        }
    }

    fn sendFailure(
        callback: Weak<EvaluateCallback>,
        injected_script: *mut InjectedScript,
        response: &protocol::runtime::protocol::DispatchResponse,
    ) {
        unsafe {
            if let Some(cb) = callback.upgrade() {
                //injected_script.deleteEvaluateCallback(Arc::clone(&cb));
                //assert_eq!(Arc::strong_count(&cb), 1);
                cb.sendFailure_internal(response);
            }
        }
    }
    fn sendSuccess_internal(
        &self,
        result: std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
        exception_details: std::unique_ptr<inspector_protocol::runtime::ExceptionDetails>,
    ) {
        // Implementation of sendSuccess
    }

    fn sendFailure_internal(
        &self,
        response: &protocol::runtime::protocol::DispatchResponse,
    ) {
        // Implementation of sendFailure
    }
}

mod promise_handler_tracker {
    pub struct PromiseHandlerTracker {}
    impl PromiseHandlerTracker {
        pub fn new() -> Self { PromiseHandlerTracker {} }
        pub fn create(&self) -> i32 { 0 }
        pub fn discard(&self, _id: i32, _reason: DiscardReason) {}
        pub fn get(&self, _id: i32) -> *mut () {
            std::ptr::null_mut()
        }
    }
    pub enum DiscardReason {
        kTearDown,
    }
}

mod v8_debugger {
    pub struct V8Debugger {}
    impl V8Debugger {
        pub fn new() -> Self { V8Debugger {} }
        pub fn enabled(&self) -> bool { false }
        pub fn getPauseOnExceptionsState(&self) -> ExceptionBreakState { ExceptionBreakState::NoBreakOnException }
        pub fn setPauseOnExceptionsState(&self, _state: ExceptionBreakState) {}
        pub fn createStackTrace(&self, _stack_trace: v8::Value) -> std::unique_ptr<()> {
            std::unique_ptr::new(())
        }
        pub fn captureStackTrace(&self, _b: bool) -> std::unique_ptr<()> {
            std::unique_ptr::new(())
        }
    }
    pub enum ExceptionBreakState {
        NoBreakOnException
    }
}

mod v8_console {
    pub struct V8Console {}
    impl V8Console {
        pub fn new() -> Self { V8Console {} }
        pub fn createCommandLineAPI(&self, _context: &v8::Context, _session_id: i32) -> v8::Object{
            v8::Object {}
        }
    }
    pub struct CommandLineAPIScope {}
}

mod v8_inspector_impl {
    pub struct V8InspectorImpl {
        isolate_id: i32,
        debugger: v8_debugger::V8Debugger,
        console: V8Console,
        exception_id: i32,
        pub promise_handler_tracker: promise_handler_tracker::PromiseHandlerTracker,
    }

    impl V8InspectorImpl {
        pub fn new() -> Box<Self> {
            Box::new(V8InspectorImpl {
                isolate_id: 0,
                debugger: v8_debugger::V8Debugger::new(),
                console: V8Console::new(),
                exception_id: 0,
                promise_handler_tracker: promise_handler_tracker::PromiseHandlerTracker::new(),
            })
        }

        pub fn client(&self) -> &dyn V8InspectorClient {
            static client: MockV8InspectorClient = MockV8InspectorClient {};
            &client
        }
        pub fn isolateId(&self) -> i32 {
            self.isolate_id
        }
        pub fn debugger(&self) -> &v8_debugger::V8Debugger {
            &self.debugger
        }
        pub fn console(&self) -> &V8Console {
            &self.console
        }
        pub fn getContext(&self, _id: i32) -> Option<&InspectedContext>{
            None
        }
        pub fn nextExceptionId(&mut self) -> i32 {
            self.exception_id += 1;
            self.exception_id
        }
        pub fn compileAndRunInternalScript(&self, _context: &v8::Context, _script: v8::String) -> v8::Value {
            v8::Value {}
        }
        pub fn promiseHandlerTracker(&self) -> &promise_handler_tracker::PromiseHandlerTracker {
            &self.promise_handler_tracker
        }
        pub fn sessionById(&self, _context_group_id: i32, _session_id: i32) -> *mut V8InspectorSessionImpl {
            std::ptr::null_mut()
        }
        pub fn muteExceptions(&self, _context_group_id: i32) {}
        pub fn unmuteExceptions(&self, _context_group_id: i32) {}
    }

    pub trait V8InspectorClient {
        fn dispatchError(&self, _context: &v8::Context, _message: v8::Value, _exception: v8::Value) {}
        fn beginUserGesture(&self) {}
        fn endUserGesture(&self) {}
        fn muteMetrics(&self, _context_group_id: i32) {}
        fn unmuteMetrics(&self, _context_group_id: i32) {}
    }

    struct MockV8InspectorClient {}

    impl V8InspectorClient for MockV8InspectorClient {
        fn dispatchError(&self, _context: &v8::Context, _message: v8::Value, _exception: v8::Value) {
            // Mock implementation
        }
        fn beginUserGesture(&self) {}
        fn endUserGesture(&self) {}
        fn muteMetrics(&self, _context_group_id: i32) {}
        fn unmuteMetrics(&self, _context_group_id: i32) {}
    }
}

struct V8SerializationDuplicateTracker {}

impl V8SerializationDuplicateTracker {
    fn new(_context: &v8::Context) -> Self {
        V8SerializationDuplicateTracker {}
    }
}

const K_GLOBAL_HANDLE_LABEL: &str = "DevTools console";

/// Represents a script injected into a V8 context for debugging purposes.
pub struct InjectedScript {
    m_context: *mut InspectedContext, // Assuming InspectedContext is a struct
    m_sessionId: i32,
    m_lastBoundObjectId: i32,
    m_idToWrappedObject: HashMap<i32, v8::Value>,
    m_idToObjectGroupName: HashMap<i32, String16>,
    m_nameToObjectGroup: HashMap<String16, Vec<i32>>,
    m_lastEvaluationResult: v8::Value,
    m_customPreviewEnabled: bool,
    m_commandLineAPI: v8::Object
    // TODO: Add other fields from the C++ class.
}

impl InjectedScript {
    /// Creates a new `InjectedScript` instance.
    pub fn new(context: *mut InspectedContext, sessionId: i32) -> Self {
        InjectedScript {
            m_context: context,
            m_sessionId: sessionId,
            m_lastBoundObjectId: 0,
            m_idToWrappedObject: HashMap::new(),
            m_idToObjectGroupName: HashMap::new(),
            m_nameToObjectGroup: HashMap::new(),
            m_lastEvaluationResult: v8::Value {},
            m_customPreviewEnabled: false,
            m_commandLineAPI: v8::Object{},
        }
    }

    /// Discards the `InjectedScript` instance, releasing any associated resources.
    fn discardEvaluateCallbacks(&mut self) {}

    fn deleteEvaluateCallback(&mut self, _callback: EvaluateCallbackPtr) {}
    /// Wraps a V8 value into a `protocol::Runtime::RemoteObject`.
    pub fn wrapObject(
        &self,
        value: &v8::Value,
        group_name: String16,
        wrap_options: WrapOptions,
        result: &mut std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
    ) -> Response {
        self.wrapObject_ext(value, group_name, wrap_options, std::unique_ptr::new(()), 0, result)
    }

    fn wrapObject_ext(
        &self,
        value: &v8::Value,
        group_name: String16,
        wrap_options: WrapOptions,
        customPreviewConfig: std::unique_ptr<()>,
        maxCustomPreviewDepth: i32,
        result: &mut std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
    ) -> Response {
        let context = unsafe { (*self.m_context).context() };
        //let context_scope = v8::Context::Scope(context);
        let mirror = ValueMirror::create(context, value);
        if mirror.is_null() {
            return Response::InternalError();
        }
        self.wrapObjectMirror(&mirror.unwrap(), group_name, wrap_options, customPreviewConfig, maxCustomPreviewDepth, result)
    }

    fn wrapObjectMirror(
        &self,
        mirror: &ValueMirror,
        group_name: String16,
        wrap_options: WrapOptions,
        customPreviewConfig: std::unique_ptr<()>,
        maxCustomPreviewDepth: i32,
        result: &mut std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
    ) -> Response {
        let customPreviewEnabled = self.m_customPreviewEnabled;
        let sessionId = self.m_sessionId;
        let context = unsafe { (*self.m_context).context() };
        //let context_scope = v8::Context::Scope(context);
        let response = mirror.buildRemoteObject(context, wrap_options, result);
        if !response.IsSuccess() {
            return response;
        }
        let value = mirror.v8Value(unsafe { (*self.m_context).inspector().debugger().as_ref().unwrap() });
        //response = bindRemoteObjectIfNeeded(sessionId, context, value, groupName, result.get());
        //if (!response.IsSuccess()) return response;
        // if (customPreviewEnabled && value.IsObject()) {
        //     std::unique_ptr<protocol::Runtime::CustomPreview> customPreview;
        //     generateCustomPreview(m_context->isolate(), sessionId, groupName,
        //                           value.As<v8::Object>(), customPreviewConfig,
        //                           maxCustomPreviewDepth, &customPreview);
        //     if (customPreview) (*result)->setCustomPreview(std::move(customPreview));
        // }
        if let WrapMode::kDeep = wrap_options.mode {
            let duplicateTracker = V8SerializationDuplicateTracker::new(context);

            let mut deepSerializedValueDict = std::unique_ptr::new(protocol::runtime::protocol::DispatchResponse {});
            let response = mirror.buildDeepSerializedValue(
                context, wrap_options.serializationOptions.maxDepth,
                wrap_options.serializationOptions.additionalParameters.Get(unsafe { (*self.m_context).inspector().debugger().as_ref().unwrap() }),
                duplicateTracker, &mut deepSerializedValueDict);
            if !response.IsSuccess() {
                return response;
            }

            // let mut type: String16 = String16::new();
            // deepSerializedValueDict.getString("type", &type);

            // std::unique_ptr<protocol::Runtime::DeepSerializedValue>
            //     deepSerializedValue = protocol::Runtime::DeepSerializedValue::create()
            //                               .setType(type)
            //                               .build();

            // protocol::Value* maybeValue = deepSerializedValueDict.get("value");
            // if (maybeValue != nullptr) {
            //     deepSerializedValue->setValue(maybeValue->clone());
            // }

            // int weakLocalObjectReference;
            // if (deepSerializedValueDict.getInteger("weakLocalObjectReference",
            //                                      &weakLocalObjectReference)) {
            //     deepSerializedValue->setWeakLocalObjectReference(
            //         weakLocalObjectReference);
            // }

            // if (!response.IsSuccess()) return response;
            // (*result)->setDeepSerializedValue(std::move(deepSerializedValue));
        }

        Response::Success()
    }
    /// Releases an object with the given ID.
    pub fn releaseObject(&self, objectId: String16) {
        let mut remoteId = std::unique_ptr::new(RemoteObjectId { id: 0, context_id: 0, isolate_id: 0 });
        let response = RemoteObjectId::parse(objectId, &mut remoteId);
        if response.IsSuccess() {
            self.unbindObject(remoteId.unwrap().id());
        }
    }

    /// Binds a V8 value to an ID and stores it for later retrieval.
    fn bindObject(&mut self, value: &v8::Value, groupName: String16) -> String16 {
        if self.m_lastBoundObjectId <= 0 {
            self.m_lastBoundObjectId = 1;
        }
        let id = self.m_lastBoundObjectId + 1;
        self.m_lastBoundObjectId = id;
        self.m_idToWrappedObject.insert(id, v8::Value {});
        //self.m_idToWrappedObject[id].AnnotateStrongRetainer(K_GLOBAL_HANDLE_LABEL); //TODO add lifetime annotation
        if !groupName.is_empty() && id > 0 {
            self.m_idToObjectGroupName.insert(id, groupName.clone());
            self.m_nameToObjectGroup.entry(groupName).or_insert(Vec::new()).push(id);
        }
        RemoteObjectId::serialize(unsafe { (*self.m_context).inspector().isolateId() }, unsafe { (*self.m_context).contextId() }, id)
    }

    /// Unbinds an object with the given ID, releasing the stored V8 value.
    fn unbindObject(&self, id: i32) {
        self.m_idToWrappedObject.remove(&id);
        self.m_idToObjectGroupName.remove(&id);
    }

    fn wrapEvaluateResult(
        &self,
        maybeResultValue: Option<&v8::Value>,
        tryCatch: &TryCatch,
        objectGroup: String16,
        wrapOptions: WrapOptions,
        throwOnSideEffect: bool,
        result: &mut std::unique_ptr<inspector_protocol::runtime::RemoteObject>,
        exceptionDetails: &mut std::unique_ptr<inspector_protocol::runtime::ExceptionDetails>,
    ) -> Response {
        // //v8::Local<v8::Value> resultValue;
        if tryCatch.HasCaught() {
            return Response::InternalError();
        }

        let resultValue: &v8::Value = maybeResultValue.unwrap();

        let response =
            self.wrapObject(resultValue, objectGroup, wrapOptions, result);
        if !response.IsSuccess() {
            return response;
        }
        Response::Success()
    }
    fn resolveCallArgument(
        &self,
        callArgument: &protocol::runtime::protocol::runtime::CallArgument,
        result: &mut &v8::Value,
    ) -> Response {
        if callArgument.hasObjectId() {
            let mut remoteObjectId = std::unique_ptr::new(RemoteObjectId { id: 0, context_id: 0, isolate_id: 0 });
            let response =
                RemoteObjectId::parse(callArgument.getObjectId(""), &mut remoteObjectId);
            if !response.IsSuccess() {
                return response;
            }
            if remoteObjectId.as_ref().unwrap().contextId() != unsafe { (*self.m_context).contextId() }
                || remoteObjectId.as_ref().unwrap().isolateId() != unsafe { (*self.m_context).inspector().isolateId() }
            {
                return Response::ServerError(
                    "Argument should belong to the same JavaScript world as target object",
                );
            }
            //return findObject(*remoteObjectId, result);
            return Response::Success();
        }
        if callArgument.hasValue() || callArgument.hasUnserializableValue() {
            let mut value: String16 = String16::new();
            if callArgument.hasValue() {
                let mut json: Vec<u8> = Vec::new();
                let cbor = callArgument.getValue(std::ptr::null_mut()).unwrap();
                crdtp::json::ConvertCBORToJSON(
                    crdtp::json::SpanFrom(cbor),
                    &mut json,
                );
                value =
                    "(" +
                    String16::from(std::str::from_utf8(json.as_slice()).unwrap()) +
                        ")";
            } else {
                let unserializableValue = callArgument.getUnserializableValue("");
                // Protect against potential identifier resolution for NaN and Infinity.
                if isResolvableNumberLike(unserializableValue.clone()) {
                    value = "Number(\"".to_string() + &unserializableValue + "\")";
                } else {
                    value = unserializableValue;
                }
            }
            return Response::Success();
        }
        *result = &v8::Value {};
        return Response::Success();
    }
    fn addPromiseCallback(
        &self,
        _session: *mut V8InspectorSessionImpl,
        _value: Option<&v8::Value>,
        _objectGroup: String16,
        _wrapOptions: WrapOptions,
        _replMode: bool,
        _throwOnSideEffect: bool,
        _callback: std::unique_ptr<EvaluateCallback>,
    ) {
    }

    fn setLastEvaluationResult(&mut self, result: &v8::Value) {
        self.m_lastEvaluationResult = v8::Value {};
        //m_lastEvaluationResult.AnnotateStrongRetainer(K_GLOBAL_HANDLE_LABEL);
    }
    fn objectGroupName(&self, _object_id: &RemoteObjectId) -> String16 {
        String16::new() // Placeholder
    }

    fn getProperties(&self, _object: &v8::Object, _groupName: &String16, _ownProperties: bool, _accessorPropertiesOnly: bool, _nonIndexedPropertiesOnly: bool, _wrapOptions: &WrapOptions, _properties: &mut std::unique_ptr<protocol::runtime::protocol::Array<protocol::runtime::PropertyDescriptor>>, _
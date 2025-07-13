// Converted from V8 C++ source files:
// Header: injected-script.h
// Implementation: injected-script.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Runtime {
        pub struct RemoteObject {}
        pub struct ExceptionDetails {}
        pub struct PropertyDescriptor {}
        pub struct InternalPropertyDescriptor {}
        pub struct PrivatePropertyDescriptor {}
        pub struct CallArgument {}
        pub struct CustomPreview {}
        pub struct DeepSerializedValue {}

        impl RemoteObject {
            pub fn create() -> RemoteObjectBuilder {
                RemoteObjectBuilder::new()
            }
        }

        impl ExceptionDetails {
            pub fn create() -> ExceptionDetailsBuilder {
                ExceptionDetailsBuilder::new()
            }
        }

        impl PropertyDescriptor {
            pub fn create() -> PropertyDescriptorBuilder {
                PropertyDescriptorBuilder::new()
            }
        }

        impl InternalPropertyDescriptor {
            pub fn create() -> InternalPropertyDescriptorBuilder {
                InternalPropertyDescriptorBuilder::new()
            }
        }

        impl PrivatePropertyDescriptor {
            pub fn create() -> PrivatePropertyDescriptorBuilder {
                PrivatePropertyDescriptorBuilder::new()
            }
        }

        pub struct RemoteObjectBuilder {
            inner: RemoteObject,
        }

        impl RemoteObjectBuilder {
            pub fn new() -> Self {
                RemoteObjectBuilder {
                    inner: RemoteObject {},
                }
            }
            pub fn set_custom_preview(mut self, _custom_preview: std::unique_ptr::UniquePtr<CustomPreview>) -> Self {
                self
            }

            pub fn setCustomPreview(mut self, custom_preview: std::unique_ptr::UniquePtr<CustomPreview>) -> Self {
                self
            }

            pub fn setDeepSerializedValue(mut self, deep_serialized_value: std::unique_ptr::UniquePtr<DeepSerializedValue>) -> Self {
                self
            }
            pub fn build(self) -> RemoteObject {
                self.inner
            }
        }

        pub struct ExceptionDetailsBuilder {
            inner: ExceptionDetails,
        }

        impl ExceptionDetailsBuilder {
            pub fn new() -> Self {
                ExceptionDetailsBuilder {
                    inner: ExceptionDetails {},
                }
            }
            pub fn build(self) -> ExceptionDetails {
                self.inner
            }

            pub fn setStackTrace(mut self, _build_inspector_object_impl: usize) -> Self {
                self
            }

            pub fn setException(mut self, _wrapped: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }

            pub fn setScriptId(mut self, _script_id: String16) -> Self {
                self
            }
            pub fn setText(mut self, _text: String16) -> Self {
                self
            }

            pub fn setLineNumber(mut self, _line_number: i32) -> Self {
                self
            }
            pub fn setColumnNumber(mut self, _column_number: i32) -> Self {
                self
            }

            pub fn setExceptionId(mut self, _next_exception_id: i32) -> Self {
                self
            }
        }

        pub struct PropertyDescriptorBuilder {
            inner: PropertyDescriptor,
        }

        impl PropertyDescriptorBuilder {
            pub fn new() -> Self {
                PropertyDescriptorBuilder {
                    inner: PropertyDescriptor {},
                }
            }
            pub fn build(self) -> PropertyDescriptor {
                self.inner
            }

            pub fn setName(mut self, _name: String16) -> Self {
                self
            }

            pub fn setConfigurable(mut self, _configurable: bool) -> Self {
                self
            }

            pub fn setEnumerable(mut self, _enumerable: bool) -> Self {
                self
            }

            pub fn setIsOwn(mut self, _is_own: bool) -> Self {
                self
            }
            pub fn setValue(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }
            pub fn setWritable(mut self, _writable: bool) -> Self {
                self
            }
            pub fn setGet(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }
            pub fn setSet(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }

            pub fn setSymbol(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }

            pub fn setWasThrown(mut self, _b: bool) -> Self {
                self
            }
        }

        pub struct InternalPropertyDescriptorBuilder {
            inner: InternalPropertyDescriptor,
        }

        impl InternalPropertyDescriptorBuilder {
            pub fn new() -> Self {
                InternalPropertyDescriptorBuilder {
                    inner: InternalPropertyDescriptor {},
                }
            }
            pub fn build(self) -> InternalPropertyDescriptor {
                self.inner
            }

            pub fn setName(mut self, _name: String16) -> Self {
                self
            }

            pub fn setValue(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }
        }

        pub struct PrivatePropertyDescriptorBuilder {
            inner: PrivatePropertyDescriptor,
        }

        impl PrivatePropertyDescriptorBuilder {
            pub fn new() -> Self {
                PrivatePropertyDescriptorBuilder {
                    inner: PrivatePropertyDescriptor {},
                }
            }
            pub fn build(self) -> PrivatePropertyDescriptor {
                self.inner
            }

            pub fn setName(mut self, _name: String16) -> Self {
                self
            }

            pub fn setValue(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }
            pub fn setGet(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }

            pub fn setSet(mut self, _remote_object: std::unique_ptr::UniquePtr<RemoteObject>) -> Self {
                self
            }
        }
    }
}
pub mod v8 {
    pub struct Value {}
    pub struct Object {}
    pub struct Context {}
    pub struct Isolate {}
    pub struct Message {}
    pub struct TryCatch {}
    pub struct StackTrace {}
    pub struct String {}
    pub struct Number {}
    pub struct Function {}
    pub enum ConstructorBehavior {
        kThrow
    }
    pub struct Promise {}
    pub struct Array {}

    impl Value {
        pub fn IsObject(&self) -> bool {
            true
        }
        pub fn As<T>(&self) -> &T {
            todo!()
        }

        pub fn ToObject<'a>(&self, _context: &Context) -> MaybeLocal<'a, Object> {
            MaybeLocal {
                local: Some(Object {}),
            }
        }
    }

    impl Object {
        pub fn Get<'a>(&self, _context: &Context, _name: &String) -> MaybeLocal<'a, Value> {
            MaybeLocal {
                local: Some(Value {}),
            }
        }

        pub fn Get(
            &self,
            _context: &Context,
            _name: v8::Local<v8::String>,
        ) -> MaybeLocal<'static, v8::Value> {
            MaybeLocal {
                local: Some(Value {}),
            }
        }

        pub fn Get(
            &self,
            _context: &Context,
            _index: u32,
        ) -> MaybeLocal<'static, v8::Value> {
            MaybeLocal {
                local: Some(Value {}),
            }
        }
        pub fn Length(&self) -> u32 {
            0
        }
    }

    impl Context {
        pub fn Enter(&self) {}
        pub fn Exit(&self) {}
        pub fn AllowCodeGenerationFromStrings(&self, _b: bool) {}

        pub fn GetIsolate(&self) -> *mut Isolate {
            std::ptr::null_mut()
        }

        pub fn Global(&self) -> &Object {
            todo!()
        }
    }

    impl Isolate {
        pub fn GetCurrentContext(&mut self) -> *mut Context {
            std::ptr::null_mut()
        }
    }

    impl Message {
        pub fn Get(&self) -> &Value {
            todo!()
        }

        pub fn GetLineNumber(&self, _context: &Context) -> Maybe<i32> {
            Maybe {
                has_value: true,
                value: 1,
            }
        }

        pub fn GetStartColumn(&self, _context: &Context) -> Maybe<i32> {
            Maybe {
                has_value: true,
                value: 0,
            }
        }

        pub fn GetScriptOrigin(&self) -> ScriptOrigin {
            ScriptOrigin {}
        }

        pub fn GetStackTrace(&self) -> Local<StackTrace> {
            Local {
                handle: StackTrace {},
            }
        }
    }

    pub struct ScriptOrigin {}

    impl ScriptOrigin {
        pub fn ScriptId(&self) -> i32 {
            0
        }
    }

    impl TryCatch {
        pub fn HasCaught(&self) -> bool {
            false
        }

        pub fn Message(&self) -> Local<Message> {
            Local {
                handle: Message {},
            }
        }

        pub fn Exception(&self) -> Local<Value> {
            Local {
                handle: Value {},
            }
        }

        pub fn HasTerminated(&self) -> bool {
            false
        }

        pub fn CanContinue(&self) -> bool {
            true
        }

        pub fn SetVerbose(&self, _b: bool) {}
    }

    impl StackTrace {
        pub fn GetFrameCount(&self) -> i32 {
            0
        }
    }

    impl String {
    }

    impl Number {
        pub fn New(_isolate: *mut Isolate, _handler_id: i64) -> Local<Number> {
            Local {
                handle: Number {},
            }
        }

        pub fn Value(&self) -> f64 {
            0.0
        }
    }

    impl Function {
        pub fn New(
            _context: &Context,
            _then_callback: fn(v8::FunctionCallbackInfo<v8::Value>),
            _data: Local<Number>,
            _i: i32,
            _k_throw: v8::ConstructorBehavior,
        ) -> MaybeLocal<'static, v8::Function> {
            MaybeLocal {
                local: Some(Function {}),
            }
        }
    }

    impl Promise {
        pub fn Then(
            &self,
            _context: &Context,
            _then_callback_function: Local<Function>,
            _catch_callback_function: Local<Function>,
        ) -> MaybeLocal<'static, v8::Promise> {
            MaybeLocal {
                local: Some(Promise {}),
            }
        }
    }

    impl Array {
        pub fn Get(
            &self,
            _context: &Context,
            _i: u32,
        ) -> MaybeLocal<'static, v8::Value> {
            MaybeLocal {
                local: Some(Value {}),
            }
        }
    }

    pub struct Local<'a, T> {
        handle: T,
    }

    impl<'a, T> Local<'a, T> {
        pub fn IsEmpty(&self) -> bool {
            false
        }
        pub fn Get(&self, _isolate: *mut Isolate) -> &T {
            &self.handle
        }
    }

    pub struct MaybeLocal<'a, T> {
        local: Option<T>,
    }

    impl<'a, T> MaybeLocal<'a, T> {
        pub fn ToLocal(&self, _result: &mut Local<T>) -> bool {
            true
        }

        pub fn IsEmpty(&self) -> bool {
            self.local.is_none()
        }
    }

    pub struct Maybe<T> {
        has_value: bool,
        value: T,
    }

    impl<T> Maybe<T> {
        pub fn FromMaybe(&self, default: T) -> T {
            if self.has_value {
                self.value
            } else {
                default
            }
        }
    }

    pub struct FunctionCallbackInfo<'a, T> {
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> FunctionCallbackInfo<'a, T> {
        pub fn Data(&self) -> Local<Value> {
            Local {
                handle: Value {},
            }
        }

        pub fn GetIsolate(&self) -> *mut Isolate {
            std::ptr::null_mut()
        }

        pub fn Length(&self) -> i32 {
            0
        }

        pub fn get(&self, _i: i32) -> Local<Value> {
            Local {
                handle: Value {},
            }
        }
    }

}

pub mod v8_inspector {
    pub struct V8Debugger {}
}

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! CHECK_EQ {
            ($left:expr, $right:expr) => {
                if $left != $right {
                    panic!("CHECK_EQ failed: {} != {}", $left, $right);
                }
            };
        }
    }
}

pub mod crdtp {
    pub mod json {
        pub fn ConvertCBORToJSON(_span_from: usize, _json: &mut Vec<u8>) {}
    }
}

pub mod std {
    pub mod unique_ptr {
        pub struct UniquePtr<T> {
            _marker: std::marker::PhantomData<T>,
        }
    }

    pub mod shared_ptr {
        pub struct AsyncStackTrace {}
    }
}

pub mod v8_crdtp {
    pub fn SpanFrom(_value: *mut protocol::Runtime::CallArgument) -> usize {
        0
    }
}

pub mod src {
    pub mod inspector {
        pub mod protocol {
            pub mod Protocol {

            }
        }

        pub mod string_util {
            pub fn toProtocolString(_isolate: *mut v8::Isolate, _string: *mut v8::String) -> String16 {
                String16 {}
            }
        }

        pub mod v8_inspector_impl {
            pub struct V8InspectorImpl {}
        }

        pub mod v8_inspector_session_impl {
            pub struct V8InspectorSessionImpl {}
        }

        pub mod remote_object_id {
            pub fn serialize(_isolate_id: i32, _context_id: i32, _id: i32) -> String16 {
                String16 {}
            }
        }
    }
}

#[derive(Clone)]
pub struct String16 {}

impl String16 {
    pub fn fromInteger(_integer: i32) -> Self {
        String16 {}
    }
    pub fn isEmpty(&self) -> bool {
        false
    }

    pub fn as_str(&self) -> &str {
        ""
    }

    pub fn from(s: &str) -> Self {
        String16 {}
    }
}

pub mod v8_inspector {
    pub struct InspectedContext {}
    impl InspectedContext {
        pub fn contextId(&self) -> i32 {
            0
        }
    }

    pub enum WrapMode {
        kIdOnly,
        kPreview,
        kDeep
    }
    pub struct WrapOptions {
        pub mode: WrapMode,
        pub serializationOptions: SerializationOptions,
    }
    impl WrapOptions {
        pub fn new(modes: Vec<WrapMode>) -> Self {
            WrapOptions {
                mode: WrapMode::kIdOnly,
                serializationOptions: SerializationOptions {},
            }
        }
    }

    pub struct Response {
        success: bool,
        error_message: Option<String>,
    }

    impl Response {
        pub fn Success() -> Self {
            Response {
                success: true,
                error_message: None,
            }
        }

        pub fn ServerError(message: &str) -> Self {
            Response {
                success: false,
                error_message: Some(message.to_string()),
            }
        }

        pub fn InternalError() -> Self {
            Response {
                success: false,
                error_message: Some("Internal error".to_string()),
            }
        }

        pub fn IsSuccess(&self) -> bool {
            self.success
        }
    }
}

pub mod v8_console {
    pub struct CommandLineAPIScope {}
}

pub mod std {
    pub mod collections {
        pub mod hash_map {
            pub struct HashMap<K, V> {
                _marker: std::marker::PhantomData<(K, V)>,
            }
        }
    }
}

pub mod value_mirror {
    pub struct ValueMirror {}
    pub struct PropertyMirror {}
}

pub mod debug {
    pub fn GetInspector(_isolate: *mut v8::Isolate) -> *mut V8InspectorImpl {
        std::ptr::null_mut()
    }
}

pub mod V8_INSPECTOR_INJECTED_SCRIPT_H_ {
    pub const kGlobalHandleLabel: &'static str = "DevTools console";
}

pub mod value_mirror {
    pub struct InternalPropertyMirror {}
    pub struct PrivatePropertyMirror {}
}

pub mod remote_object_id {
    pub struct RemoteObjectId {}
    pub fn parse(_object_id: String16, _remote_id: &mut std::unique_ptr::UniquePtr<RemoteObjectId>) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }
}

pub mod v8_value_utils {
    pub fn generateCustomPreview(_isolate: *mut v8::Isolate, _session_id: i32, _group_name: String16, _object: *mut v8::Object, _custom_preview_config: v8::MaybeLocal<v8::Value>, _max_custom_preview_depth: i32, _custom_preview: &mut std::unique_ptr::UniquePtr<protocol::Runtime::CustomPreview>) {}
}

pub mod v8_serialization_duplicate_tracker {
    pub struct V8SerializationDuplicateTracker {}
}

pub mod v8_stack_trace_impl {
    pub struct V8StackTraceImpl {}
}

pub mod v8_debugger {
    pub struct RemoteCallFrameId {}
    pub fn parse(_object_id: String16, _remote_id: &mut std::unique_ptr::UniquePtr<RemoteCallFrameId>) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }
}

pub mod kMaxCustomPreviewDepth {
    pub const kMaxCustomPreviewDepth: i32 = 10;
}

pub mod client {
    pub fn dispatchError(_context: &v8::Context, _message: v8::Local<v8::Message>, _exception: v8::Local<v8::Value>) {}
}

pub mod src {
    pub mod debug {
        pub fn GetMessageFromPromise(_promise: *mut v8::Promise) -> v8::MaybeLocal<'static, v8::Message> {
            v8::MaybeLocal {
                local: None,
            }
        }
    }
}

pub mod microtask_queue {
    pub struct MicrotasksScope {}
}

pub mod inspector {
    pub mod custom_preview {
        pub fn generateCustomPreview(
            _isolate: *mut v8::Isolate,
            _session_id: i32,
            _group_name: String16,
            _object: *mut v8::Object,
            _custom_preview_config: v8::MaybeLocal<v8::Value>,
            _max_custom_preview_depth: i32,
            _custom_preview: &mut std::unique_ptr::UniquePtr<protocol::Runtime::CustomPreview>,
        ) {
        }
    }
}

pub mod heap {
    pub mod safepoint {
        pub enum Scope {}
    }
}

pub mod config {
    pub mod compiler {
        pub struct Flags {}
    }
}

pub mod code {
    pub struct AllowedOptions {}
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use v8_inspector::Response;

use self::std::unique_ptr::UniquePtr;
use self::V8_INSPECTOR_INJECTED_SCRIPT_H_::kGlobalHandleLabel;

pub struct EvaluateCallback {}

impl EvaluateCallback {
    pub fn sendSuccess(
        _callback: std::weak_ptr::Weak<EvaluateCallback>,
        _injected_script: *mut InjectedScript,
        _result: std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
        _exception_details: std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    ) {
    }

    pub fn sendFailure(
        _callback: std::weak_ptr::Weak<EvaluateCallback>,
        _injected_script: *mut InjectedScript,
        _response: v8_inspector::Response,
    ) {
    }
}

pub trait EvaluateCallbackTrait {
    fn sendSuccess(
        &self,
        _result: std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
        _exception_details: std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    );
    fn sendFailure(&self, _response: v8_inspector::Response);
}

pub struct InjectedScript {
    m_context: *mut InspectedContext,
    m_sessionId: i32,
    m_lastEvaluationResult: Rc<RefCell<Option<*mut v8::Value>>>,
    m_commandLineAPI: Rc<RefCell<Option<*mut v8::Object>>>,
    m_lastBoundObjectId: i32,
    m_idToWrappedObject: Rc<RefCell<HashMap<i32, *mut v8::Value>>>,
    m_idToObjectGroupName: Rc<RefCell<HashMap<i32, String16>>>,
    m_nameToObjectGroup: Rc<RefCell<HashMap<String16, Vec<i32>>>>,
    m_evaluateCallbacks: Rc<RefCell<std::collections::hash_map::HashMap<usize, Box<dyn EvaluateCallbackTrait>>>>,
    m_customPreviewEnabled: bool,
}

impl InjectedScript {
    pub fn new(context: *mut InspectedContext, sessionId: i32) -> InjectedScript {
        InjectedScript {
            m_context: context,
            m_sessionId: sessionId,
            m_lastEvaluationResult: Rc::new(RefCell::new(None)),
            m_commandLineAPI: Rc::new(RefCell::new(None)),
            m_lastBoundObjectId: 1,
            m_idToWrappedObject: Rc::new(RefCell::new(HashMap::new())),
            m_idToObjectGroupName: Rc::new(RefCell::new(HashMap::new())),
            m_nameToObjectGroup: Rc::new(RefCell::new(HashMap::new())),
            m_evaluateCallbacks: Rc::new(RefCell::new(std::collections::hash_map::HashMap::new())),
            m_customPreviewEnabled: false,
        }
    }

    pub fn context(&self) -> *mut InspectedContext {
        self.m_context
    }

    pub fn getProperties(
        &self,
        _object: *mut v8::Object,
        _group_name: String16,
        _own_properties: bool,
        _accessor_properties_only: bool,
        _non_indexed_properties_only: bool,
        _wrap_options: &v8_inspector::WrapOptions,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Array<protocol::Runtime::PropertyDescriptor>>,
        _exception_details: &mut std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn getInternalAndPrivateProperties(
        &self,
        _value: *mut v8::Value,
        _group_name: String16,
        _accessor_properties_only: bool,
        _internal_properties: &mut std::unique_ptr::UniquePtr<protocol::Array<protocol::Runtime::InternalPropertyDescriptor>>,
        _private_properties: &mut std::unique_ptr::UniquePtr<protocol::Array<protocol::Runtime::PrivatePropertyDescriptor>>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn releaseObject(&self, _object_id: String16) {}

    pub fn wrapObject(
        &self,
        _value: *mut v8::Value,
        _group_name: String16,
        _wrap_options: &v8_inspector::WrapOptions,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn wrapObject1(
        &self,
        _value: *mut v8::Value,
        _group_name: String16,
        _wrap_options: &v8_inspector::WrapOptions,
        _custom_preview_config: v8::MaybeLocal<v8::Value>,
        _max_custom_preview_depth: i32,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn wrapObjectMirror(
        &self,
        _mirror: &value_mirror::ValueMirror,
        _group_name: String16,
        _wrap_options: &v8_inspector::WrapOptions,
        _custom_preview_config: v8::MaybeLocal<v8::Value>,
        _max_custom_preview_depth: i32,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn wrapTable(
        &self,
        _table: *mut v8::Object,
        _columns: v8::MaybeLocal<v8::Array>,
    ) -> std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject> {
        std::unique_ptr::UniquePtr {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn addPromiseCallback(
        &self,
        _session: *mut V8InspectorSessionImpl,
        _value: v8::MaybeLocal<v8::Value>,
        _object_group: String16,
        _wrap_options: std::unique_ptr::UniquePtr<v8_inspector::WrapOptions>,
        _repl_mode: bool,
        _throw_on_side_effect: bool,
        _callback: std::shared_ptr::Shared<EvaluateCallback>,
    ) {
    }

    pub fn findObject(
        &self,
        _object_id: &remote_object_id::RemoteObjectId,
        _out_object: &mut *mut v8::Value,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn objectGroupName(&self, _object_id: &remote_object_id::RemoteObjectId) -> String16 {
        String16 {}
    }

    pub fn releaseObjectGroup(&self, _object_group: String16) {}

    pub fn setCustomObjectFormatterEnabled(&self, _enabled: bool) {}

    pub fn resolveCallArgument(
        &self,
        _call_argument: *mut protocol::Runtime::CallArgument,
        _result: &mut *mut v8::Value,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn createExceptionDetails(
        &self,
        _try_catch: &v8::TryCatch,
        _group_name: String16,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn createExceptionDetails1(
        &self,
        _message: v8::Local<v8::Message>,
        _exception: v8::Local<v8::Value>,
        _group_name: String16,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn wrapEvaluateResult(
        &self,
        _maybe_result_value: v8::MaybeLocal<v8::Value>,
        _try_catch: &v8::TryCatch,
        _object_group: String16,
        _wrap_options: &v8_inspector::WrapOptions,
        _throw_on_side_effect: bool,
        _result: &mut std::unique_ptr::UniquePtr<protocol::Runtime::RemoteObject>,
        _exception_details: &mut std::unique_ptr::UniquePtr<protocol::Runtime::ExceptionDetails>,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn lastEvaluationResult(&self) -> *mut v8::Value {
        std::ptr::null_mut()
    }

    pub fn setLastEvaluationResult(&self, _result: *mut v8::Value) {}

    pub fn commandLineAPI(&self) -> *mut v8::Object {
        std::ptr::null_mut()
    }

    pub fn unbindObject(&self, _id: i32) {}

    pub fn bindObject(&self, _value: *mut v8::Value, _group_name: String16) -> String16 {
        String16 {}
    }

    pub fn addExceptionToDetails(
        &self,
        _exception: *mut v8::Value,
        _exception_details: *mut protocol::Runtime::ExceptionDetails,
        _object_group: String16,
    ) -> v8_inspector::Response {
        v8_inspector::Response::Success()
    }

    pub fn discardEvaluateCallbacks(&self) {}

    pub fn deleteEvaluateCallback(&self, _callback: std::shared_ptr::Shared<EvaluateCallback>) {}
}

impl Drop for InjectedScript {
    fn drop(&mut self) {
        self.discardEvaluateCallbacks();
    }
}

pub mod std {
    pub mod weak_ptr {
        pub struct Weak<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> Weak<T> {
            pub fn lock(&self) -> std::shared_ptr::Shared<T> {
                std::shared_ptr::Shared {
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }
    pub mod shared_ptr {
        pub struct Shared<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> Shared<T> {
            pub fn reset(&self) {}
        }
    }
}

pub mod PromiseHandlerTracker {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::v8_inspector::Response;
    use crate::std::unique_ptr::UniquePtr;
    use crate::InjectedScript;
    use crate::V8InspectorImpl;
    use crate::V8InspectorSessionImpl;
    use crate::String16;
    use crate::protocol::Runtime::ExceptionDetails;

    pub enum DiscardReason {
        kFulfilled,
        kPromiseCollected,
        kTearDown,
    }

    pub type Id = i64;

    pub struct PromiseHandlerTracker {
        m_promiseHandlers: Rc<RefCell<HashMap<Id, UniquePtr<ProtocolPromiseHandler>>>>,
        m_lastUsedId: Id,
    }

    impl PromiseHandlerTracker {
        pub fn new() -> Self {
            PromiseHandlerTracker {
                m_promiseHandlers: Rc::new(RefCell::new(HashMap::new())),
                m_lastUsedId: 1,
            }
        }

        pub fn create(
            &mut self,
            _session: *mut V8InspectorSessionImpl,
            _execution_context_id: i32,
            _object_group: String16,
            _wrap_options: std::unique_ptr::UniquePtr<crate::v8_inspector::WrapOptions>,
            _repl_mode: bool,
            _throw_on_side_effect: bool,
            _callback: std::weak_ptr::Weak<crate::EvaluateCallback>,
            _evaluation_result: *mut v8::Promise,
        ) -> Id {
            self.m_lastUsedId += 1;
            self.m_lastUsedId
        }

        pub fn discard(&self, _id: Id, _reason: DiscardReason) {}

        pub fn get(&self, _id: Id) -> Option<&ProtocolPromiseHandler> {
            None
        }

        fn sendFailure(
            &self,
            _handler: &ProtocolPromiseHandler,
            _response: Response,
        ) {
        }

        fn discardAll(&self) {}
    }

    impl Drop for PromiseHandlerTracker {
        fn drop(&mut self) {
            self.discardAll();
        }
    }

    pub struct ProtocolPromiseHandler {
        pub m_inspector: *mut V8InspectorImpl,
        pub m_sessionId: i32,
        pub m_contextGroupId: i32,
        pub m_executionContextId: i32,
        pub m_objectGroup: String16,
        pub m_wrapOptions: std::unique_ptr::UniquePtr<crate::v8_

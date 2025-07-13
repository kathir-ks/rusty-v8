// Converted from V8 C++ source files:
// Header: custom-preview.h
// Implementation: custom-preview.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Runtime {
        #[derive(Debug)]
        pub struct CustomPreview {
            header: String,
            body_getter_id: Option<i32>,
        }

        impl CustomPreview {
            pub fn create() -> CustomPreviewBuilder {
                CustomPreviewBuilder::default()
            }

            pub fn get_header(&self) -> &String {
                &self.header
            }

            pub fn get_body_getter_id(&self) -> Option<i32> {
                self.body_getter_id
            }
        }

        #[derive(Default, Debug)]
        pub struct CustomPreviewBuilder {
            header: Option<String>,
            body_getter_id: Option<i32>,
        }

        impl CustomPreviewBuilder {
            pub fn set_header(mut self, header: String) -> Self {
                self.header = Some(header);
                self
            }

            pub fn set_body_getter_id(mut self, body_getter_id: i32) -> Self {
                self.body_getter_id = Some(body_getter_id);
                self
            }

            pub fn build(self) -> CustomPreview {
                CustomPreview {
                    header: self.header.unwrap_or_default(),
                    body_getter_id: self.body_getter_id,
                }
            }
        }

        pub struct RemoteObject {
            serialized: Vec<u8>
        }

        impl RemoteObject {
            pub fn serialize(&self) -> &Vec<u8> {
                &self.serialized
            }

            pub fn Serialize(&self) -> Vec<u8> {
                self.serialized.clone()
            }
        }
        
        pub struct RemoteObjectBuilder {
            serialized: Vec<u8>
        }

        impl RemoteObjectBuilder {
            pub fn new() -> Self {
                RemoteObjectBuilder {
                    serialized: Vec::new()
                }
            }

            pub fn set_serialized(mut self, serialized: Vec<u8>) -> Self {
                self.serialized = serialized;
                self
            }

            pub fn build(self) -> RemoteObject {
                RemoteObject {
                    serialized: self.serialized
                }
            }
        }
    }
}

pub mod v8_crdtp {
    pub mod json {
        pub fn ConvertCBORToJSON(span: Span<u8>, json: &mut Vec<u8>) {
            // A placeholder implementation. In real scenario CBOR to JSON conversion would happen here
            json.extend_from_slice(b"{\"type\": \"object\", \"value\": \"converted from CBOR\"}");
        }
    }

    pub struct Span<T> {
        data: *const T,
        size: usize,
    }

    impl<T> Span<T> {
        pub fn from_slice(slice: &[T]) -> Self {
            Span {
                data: slice.as_ptr(),
                size: slice.len(),
            }
        }
    }
}

pub fn v8_crdtp::SpanFrom(data: Vec<u8>) -> v8_crdtp::json::Span<u8> {
    v8_crdtp::json::Span {
        data: data.as_ptr(),
        size: data.len(),
    }
}

use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

use crate::V8InspectorImpl;
use crate::isolate;
use crate::String16;
use crate::V8;
use crate::Use;
use crate::code;

pub const kMaxCustomPreviewDepth: i32 = 20;

pub fn generateCustomPreview(
    isolate: *mut v8::Isolate,
    session_id: i32,
    group_name: String16,
    object: v8::Local<v8::Object>,
    config: v8::MaybeLocal<v8::Value>,
    max_depth: i32,
    preview: &mut Option<protocol::Runtime::CustomPreview>,
) {
    let context = match object.GetCreationContext(unsafe { &mut *isolate }) {
        v8::ReturnValue::Value(context) => context,
        _ => return,
    };

    let microtasks_scope = V8::MicrotasksScope::new(context);
    let try_catch = V8::TryCatch::new();

    let config_value = match config.ToLocal() {
        v8::ReturnValue::Value(val) => val,
        _ => V8::Undefined(unsafe { &mut *isolate }),
    };

    let global = context.Global();
    let formatters_value = match global.Get(context, toV8String(unsafe { &mut *isolate }, "devtoolsFormatters")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !formatters_value.IsArray() {
        return;
    }
    let formatters = formatters_value.As::<v8::Array>();
    let header_literal = toV8String(unsafe { &mut *isolate }, "header");
    let has_body_literal = toV8String(unsafe { &mut *isolate }, "hasBody");

    for i in 0..formatters.Length() {
        let formatter_value = match formatters.Get(context, i) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };
        if !formatter_value.IsObject() {
            reportError(context, &try_catch, "formatter should be an Object");
            return;
        }
        let formatter = formatter_value.As::<v8::Object>();

        let header_value = match formatter.Get(context, header_literal.clone()) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };
        if !header_value.IsFunction() {
            reportError(context, &try_catch, "header should be a Function");
            return;
        }
        let header_function = header_value.As::<v8::Function>();

        let args = [object.clone(), config_value.clone()];
        let formatted_value = match header_function.Call(context, formatter.clone(), 2, &args) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };
        if !formatted_value.IsArray() {
            continue;
        }
        let jsonML = formatted_value.As::<v8::Array>();

        let has_body_function_value = match formatter.Get(context, has_body_literal.clone()) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };
        if !has_body_function_value.IsFunction() {
            continue;
        }
        let has_body_function = has_body_function_value.As::<v8::Function>();
        let has_body_value = match has_body_function.Call(context, formatter.clone(), 2, &args) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };
        let has_body = has_body_value.ToBoolean(unsafe { &mut *isolate }).Value();

        if jsonML.Length() > 0 && !substituteObjectTags(
            session_id,
            group_name.clone(),
            context,
            jsonML.clone(),
            max_depth,
        ) {
            return;
        }

        let header = match V8::JSON::Stringify(context, jsonML.clone()) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return;
            }
        };

        let mut body_function = None;
        if has_body {
            let body_config = V8::Object::New(unsafe { &mut *isolate });
            if body_config
                .CreateDataProperty(
                    context,
                    toV8String(unsafe { &mut *isolate }, "sessionId"),
                    V8::Integer::New(unsafe { &mut *isolate }, session_id),
                )
                .IsNothing()
            {
                reportError(context, &try_catch);
                return;
            }
            if body_config
                .CreateDataProperty(context, toV8String(unsafe { &mut *isolate }, "formatter"), formatter.clone())
                .IsNothing()
            {
                reportError(context, &try_catch);
                return;
            }
            if body_config
                .CreateDataProperty(context, toV8String(unsafe { &mut *isolate }, "groupName"), toV8String(unsafe { &mut *isolate }, group_name.clone()))
                .IsNothing()
            {
                reportError(context, &try_catch);
                return;
            }
            if body_config
                .CreateDataProperty(context, toV8String(unsafe { &mut *isolate }, "config"), config_value.clone())
                .IsNothing()
            {
                reportError(context, &try_catch);
                return;
            }
            if body_config
                .CreateDataProperty(context, toV8String(unsafe { &mut *isolate }, "object"), object.clone())
                .IsNothing()
            {
                reportError(context, &try_catch);
                return;
            }
            match V8::Function::New(context, bodyCallback, body_config) {
                v8::ReturnValue::Value(func) => {
                    body_function = Some(func);
                }
                _ => {
                    reportError(context, &try_catch);
                    return;
                }
            }
        }
        
        let mut custom_preview_builder = protocol::Runtime::CustomPreview::create();
        custom_preview_builder = custom_preview_builder.set_header(toProtocolString(unsafe { &mut *isolate }, header));
        
        let mut custom_preview = custom_preview_builder.build();

        if let Some(func) = body_function {
            let injected_script = getInjectedScript(context, session_id);
            if injected_script.is_none() {
                reportError(context, &try_catch, "cannot find context with specified id");
                return;
            }

            custom_preview.body_getter_id = Some(injected_script.unwrap().bindObject(func, group_name.clone()));
        }

        *preview = Some(custom_preview);
        return;
    }
}

fn reportError(context: v8::Local<v8::Context>, try_catch: &V8::TryCatch) {
    if !try_catch.HasCaught() {
        return;
    }
    let isolate = context.GetIsolate();
    let inspector =
        unsafe { &mut *(V8::debug::GetInspector(isolate) as *mut V8InspectorImpl) };
    let context_id = InspectedContext::contextId(context);
    let group_id = inspector.contextGroupId(context_id);
    let mut message = toV8String(isolate, "<no message available>");
    if !try_catch.Message().IsEmpty() {
        message = try_catch.Message().Get();
    }
    let prefix = toV8String(isolate, "Custom Formatter Failed: ");
    message = V8::String::Concat(isolate, prefix, message);
    let mut arguments = Vec::new();
    arguments.push(message);
    let storage = inspector.ensureConsoleMessageStorage(group_id);
    if storage.is_none() {
        return;
    }
    storage.unwrap().addMessage(V8ConsoleMessage::createForConsoleAPI(
        context,
        context_id,
        group_id,
        inspector,
        inspector.client().currentTimeMS(),
        ConsoleAPIType::kError,
        arguments,
        String16::new(),
        None,
    ));
}

fn reportError(context: v8::Local<v8::Context>, try_catch: &V8::TryCatch, message: &str) {
    let isolate = context.GetIsolate();
    V8::isolate::ThrowException(isolate, toV8String(isolate, message));
    reportError(context, try_catch);
}

fn getInjectedScript(context: v8::Local<v8::Context>, session_id: i32) -> Option<InjectedScript> {
    let isolate = context.GetIsolate();
    let inspector =
        unsafe { &mut *(V8::debug::GetInspector(isolate) as *mut V8InspectorImpl) };
    let inspected_context = inspector.getContext(InspectedContext::contextId(context));
    if inspected_context.is_none() {
        return None;
    }
    inspected_context.unwrap().getInjectedScript(session_id)
}

fn substituteObjectTags(
    session_id: i32,
    group_name: String16,
    context: v8::Local<v8::Context>,
    jsonML: v8::Local<v8::Array>,
    max_depth: i32,
) -> bool {
    if jsonML.Length() == 0 {
        return true;
    }
    let isolate = context.GetIsolate();
    let try_catch = V8::TryCatch::new();

    if max_depth <= 0 {
        reportError(
            context,
            &try_catch,
            "Too deep hierarchy of inlined custom previews",
        );
        return false;
    }

    let first_value = match jsonML.Get(context, 0) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return false;
        }
    };
    let object_literal = toV8String(isolate, "object");
    if jsonML.Length() == 2
        && first_value.IsString()
        && first_value.As::<v8::String>().StringEquals(object_literal.clone())
    {
        let attributes_value = match jsonML.Get(context, 1) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return false;
            }
        };
        if !attributes_value.IsObject() {
            reportError(context, &try_catch, "attributes should be an Object");
            return false;
        }
        let attributes = attributes_value.As::<v8::Object>();
        let origin_value = match attributes.Get(context, object_literal.clone()) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return false;
            }
        };
        if origin_value.IsUndefined() {
            reportError(
                context,
                &try_catch,
                "obligatory attribute \"object\" isn't specified",
            );
            return false;
        }

        let config_value = match attributes.Get(context, toV8String(isolate, "config")) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch);
                return false;
            }
        };

        let injected_script = getInjectedScript(context, session_id);
        if injected_script.is_none() {
            reportError(context, &try_catch, "cannot find context with specified id");
            return false;
        }
        let mut wrapper: Option<protocol::Runtime::RemoteObject> = None;
        let response = injected_script.unwrap().wrapObject(
            origin_value,
            group_name.clone(),
            WrapOptions { wrap_mode: WrapMode::kIdOnly },
            config_value,
            max_depth - 1,
            &mut wrapper,
        );
        if !response.IsSuccess() || wrapper.is_none() {
            reportError(context, &try_catch, "cannot wrap value");
            return false;
        }
        let wrapper_unwrapped = wrapper.unwrap();
        let serialized = wrapper_unwrapped.Serialize();
        let mut json: Vec<u8> = Vec::new();
        v8_crdtp::json::ConvertCBORToJSON(v8_crdtp::SpanFrom(serialized), &mut json);

        let serialized_string_view = StringView::from_vec(json);
        let json_wrapper = match V8::JSON::Parse(context, toV8String(isolate, serialized_string_view)) {
            v8::ReturnValue::Value(val) => val,
            _ => {
                reportError(context, &try_catch, "cannot wrap value");
                return false;
            }
        };
        if jsonML.Set(context, 1, json_wrapper).IsNothing() {
            reportError(context, &try_catch);
            return false;
        }
    } else {
        for i in 0..jsonML.Length() {
            let value = match jsonML.Get(context, i) {
                v8::ReturnValue::Value(val) => val,
                _ => {
                    reportError(context, &try_catch);
                    return false;
                }
            };
            if value.IsArray()
                && value.As::<v8::Array>().Length() > 0
                && !substituteObjectTags(
                    session_id,
                    group_name.clone(),
                    context,
                    value.As::<v8::Array>(),
                    max_depth - 1,
                )
            {
                return false;
            }
        }
    }
    true
}

fn bodyCallback(info: &V8::FunctionCallbackInfo) {
    let isolate = info.GetIsolate();
    let try_catch = V8::TryCatch::new();
    let context = isolate.GetCurrentContext();
    let body_config = info.Data().As::<v8::Object>();

    let object_value = match body_config.Get(context, toV8String(isolate, "object")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !object_value.IsObject() {
        reportError(context, &try_catch, "object should be an Object");
        return;
    }
    let object = object_value.As::<v8::Object>();

    let formatter_value = match body_config.Get(context, toV8String(isolate, "formatter")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !formatter_value.IsObject() {
        reportError(context, &try_catch, "formatter should be an Object");
        return;
    }
    let formatter = formatter_value.As::<v8::Object>();

    let body_value = match formatter.Get(context, toV8String(isolate, "body")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !body_value.IsFunction() {
        reportError(context, &try_catch, "body should be a Function");
        return;
    }
    let body_function = body_value.As::<v8::Function>();

    let config_value = match body_config.Get(context, toV8String(isolate, "config")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };

    let session_id_value = match body_config.Get(context, toV8String(isolate, "sessionId")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !session_id_value.IsInt32() {
        reportError(context, &try_catch, "sessionId should be an Int32");
        return;
    }

    let group_name_value = match body_config.Get(context, toV8String(isolate, "groupName")) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if !group_name_value.IsString() {
        reportError(context, &try_catch, "groupName should be a string");
        return;
    }

    let args = [object.clone(), config_value.clone()];
    let formatted_value = match body_function.Call(context, formatter.clone(), 2, &args) {
        v8::ReturnValue::Value(val) => val,
        _ => {
            reportError(context, &try_catch);
            return;
        }
    };
    if formatted_value.IsNull() {
        info.GetReturnValue().Set(formatted_value);
        return;
    }
    if !formatted_value.IsArray() {
        reportError(context, &try_catch, "body should return an Array");
        return;
    }
    let jsonML = formatted_value.As::<v8::Array>();
    if jsonML.Length() > 0
        && !substituteObjectTags(
            session_id_value.As::<v8::Int32>().Value(),
            toProtocolString(isolate, group_name_value.As::<v8::String>()),
            context,
            jsonML.clone(),
            kMaxCustomPreviewDepth,
        )
    {
        return;
    }
    info.GetReturnValue().Set(jsonML);
}

// Dummy implementations for types and functions used in the code
// These need to be replaced with actual implementations based on the V8 API
pub mod v8 {
    pub struct Isolate {}
    pub struct Context {}
    pub struct Object {}
    pub struct Value {}
    pub struct String {}
    pub struct Array {}
    pub struct Function {}
    pub struct Integer {}
    pub struct Boolean {}

    #[derive(Clone)]
    pub struct Local<T> {
       
    }

    impl<T> Local<T> {
         pub fn StringEquals(self, _other: Local<String>) -> bool {
            true
        }
    }

    pub mod debug {
        pub fn GetInspector(_isolate: *mut Isolate) -> *mut std::ffi::c_void {
            std::ptr::null_mut()
        }
    }

    pub enum ReturnValue<T> {
        Value(T),
        Empty,
    }
    
    impl<T> ReturnValue<T> {
        pub fn IsEmpty(&self) -> bool {
            match self {
                ReturnValue::Empty => true,
                _ => false,
            }
        }
    }

    pub mod JSON {
        pub fn Stringify(_context: Local<Context>, _value: Local<Value>) -> ReturnValue<Local<String>> {
            ReturnValue::Value(Local {  })
        }

        pub fn Parse(_context: Local<Context>, _value: Local<String>) -> ReturnValue<Local<Value>> {
            ReturnValue::Value(Local {  })
        }
    }
}

pub struct V8ConsoleMessage {

}

impl V8ConsoleMessage {
    pub fn createForConsoleAPI(context: v8::Local<v8::Context>, contextId: i32, groupId: i32, inspector: &mut V8InspectorImpl, currentTimeMS: f64, consoleAPIType: ConsoleAPIType, arguments: Vec<v8::Local<v8::Value>>, string16: String16, none: Option<()>) -> V8ConsoleMessage {
         V8ConsoleMessage {}
    }
}

pub struct V8ConsoleMessageStorage {

}

impl V8ConsoleMessageStorage {
    pub fn addMessage(&mut self, message: V8ConsoleMessage) {}
}

pub enum ConsoleAPIType {
    kError
}

pub struct V8MicrotasksScope {
   
}

impl V8MicrotasksScope {
     pub fn new(_context: v8::Local<v8::Context>) -> Self {
        V8MicrotasksScope {}
    }
}

impl Drop for V8MicrotasksScope {
    fn drop(&mut self) {}
}

pub struct StringView {
    data: Vec<u8>
}

impl StringView {
     pub fn from_vec(data: Vec<u8>) -> Self {
        StringView {
            data: data
        }
    }
}

pub struct InjectedScript {

}

impl InjectedScript {
    pub fn wrapObject(&self, originValue: v8::Local<v8::Value>, groupName: String16, wrapOptions: WrapOptions, configValue: v8::Local<v8::Value>, i: i32, wrapper: &mut Option<protocol::Runtime::RemoteObject>) -> Response {
        *wrapper = Some(protocol::Runtime::RemoteObjectBuilder::new().set_serialized(Vec::new()).build());
        Response {success: true}
    }

    pub fn bindObject(&self, func: v8::Local<v8::Function>, groupName: String16) -> i32 {
        1
    }
}

#[derive(Clone, Copy)]
pub struct WrapOptions {
    wrap_mode: WrapMode,
}

#[derive(Clone, Copy)]
pub enum WrapMode {
    kIdOnly,
}

pub struct Response {
    success: bool
}

impl Response {
    pub fn IsSuccess(&self) -> bool {
        self.success
    }
}

pub struct InspectedContext {

}

impl InspectedContext {
    pub fn contextId(context: v8::Local<v8::Context>) -> i32 {
        1
    }

    pub fn getInjectedScript(&self, sessionId: i32) -> Option<InjectedScript> {
        Some(InjectedScript{})
    }
}

pub fn toV8String(isolate: *mut v8::Isolate, value: &str) -> v8::Local<v8::String> {
    v8::Local {  }
}

pub fn toV8String(isolate: *mut v8::Isolate, string_view: StringView) -> v8::Local<v8::String> {
    v8::Local {  }
}

pub fn toV8String(isolate: *mut v8::Isolate, value: String16) -> v8::Local<v8::String> {
    v8::Local {  }
}

pub fn toProtocolString(isolate: *mut v8::Isolate, value: v8::Local<v8::String>) -> String {
    String::from("protocol string")
}

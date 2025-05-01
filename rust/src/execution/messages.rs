// src/execution/messages.rs

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

// Placeholder for v8::Isolate::kMessageError
const K_MESSAGE_ERROR: i32 = 1;

// Placeholder for v8::Isolate::kMessageWarning
const K_MESSAGE_WARNING: i32 = 2;

// Placeholder for v8::Isolate
struct Isolate {
    // Add necessary fields here, e.g., factory, exception, etc.
}

impl Isolate {
    fn has_exception(&self) -> bool {
        // Implement this
        false
    }

    fn exception(&self) -> Object {
        // Implement this
        Object {}
    }

    fn clear_pending_message(&self) {
        // Implement this
    }

    fn clear_exception(&self) {
        // Implement this
    }
}

// Placeholder for v8::Message
struct Message {}

// Placeholder for v8::Value
struct Value {}

// Placeholder for v8::Utils
mod utils {
    use super::*;
    pub fn message_to_local(_message: &JSMessageObject) -> Message {
        Message {} // Implement this
    }
    pub fn to_local(_obj: &Object) -> Value {
        Value {} // Implement this
    }
}

// Placeholder for v8::TryCatch
struct TryCatch {}

impl TryCatch {
    fn new(_isolate: *mut Isolate) -> TryCatch {
        TryCatch {} // Implement this
    }
    fn set_verbose(&mut self, _verbose: bool) {
        // Implement this
    }
    fn set_capture_message(&mut self, _capture_message: bool) {
        // Implement this
    }
    fn reset(&mut self) {
        // Implement this
    }
}

// Placeholder for v8::HandleScope
struct HandleScope<'a> {
    _isolate: &'a Isolate,
}

impl<'a> HandleScope<'a> {
    fn new(_isolate: &'a Isolate) -> Self {
        HandleScope { _isolate }
    }
}

// Placeholder for v8::Local
struct Local<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Local<T> {
    fn error_level(&self) -> i32 {
        K_MESSAGE_ERROR // Implement this
    }
}

// Placeholder for v8::MessageCallback
type MessageCallback = extern "C" fn(Local<Message>, Local<Value>);

// Placeholder for Isolate::ExceptionScope
struct ExceptionScope<'a> {
    _isolate: &'a mut Isolate,
}

impl<'a> ExceptionScope<'a> {
    fn new(_isolate: &'a mut Isolate) -> Self {
        ExceptionScope { _isolate }
    }
}

// Placeholder for DirectHandle
struct DirectHandle<T> {
    value: T,
}

impl<T> DirectHandle<T> {
    fn new(value: T) -> Self {
        DirectHandle { value }
    }
}

fn direct_handle<T>(value: T, _isolate: &Isolate) -> DirectHandle<T> {
    DirectHandle { value }
}

// Placeholder for MaybeDirectHandle
struct MaybeDirectHandle<T> {
    value: Option<T>,
}

impl<T> MaybeDirectHandle<T> {
    fn to_handle(&self, handle: &mut DirectHandle<T>) -> bool {
        if let Some(ref v) = self.value {
            handle.value = v.clone(); // Assuming T is Cloneable
            true
        } else {
            false
        }
    }
    fn to_handle_checked(&self) -> T
    where
        T: Clone,
    {
        self.value.clone().unwrap()
    }
}

// Placeholder for indirect_handle
fn indirect_handle<T>(value: T, _isolate: &Isolate) -> MaybeDirectHandle<T>
where
    T: Clone,
{
    MaybeDirectHandle { value: Some(value) }
}

// Placeholder for base::VectorOf
struct VectorOf<T> {
    data: Vec<T>,
}

impl<T> VectorOf<T> {
    fn new(data: Vec<T>) -> Self {
        VectorOf { data }
    }
}

impl<T> std::ops::Deref for VectorOf<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Placeholder for MessageLocation
#[derive(Debug, Clone)]
struct MessageLocation {
    script_: Script,
    start_pos_: i32,
    end_pos_: i32,
    bytecode_offset_: i32,
    shared_: Option<SharedFunctionInfo>,
}

impl MessageLocation {
    fn new(script: Script, start_pos: i32, end_pos: i32) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: start_pos,
            end_pos_: end_pos,
            bytecode_offset_: -1,
            shared_: None,
        }
    }

    fn new_with_shared(
        script: Script,
        start_pos: i32,
        end_pos: i32,
        shared: SharedFunctionInfo,
    ) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: start_pos,
            end_pos_: end_pos,
            bytecode_offset_: -1,
            shared_: Some(shared),
        }
    }

    fn new_with_bytecode_offset(script: Script, shared: SharedFunctionInfo, bytecode_offset: i32) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: -1,
            end_pos_: -1,
            bytecode_offset_: bytecode_offset,
            shared_: Some(shared),
        }
    }

    fn empty() -> Self {
        MessageLocation {
            script_: Script {},
            start_pos_: -1,
            end_pos_: -1,
            bytecode_offset_: -1,
            shared_: None,
        }
    }

    fn script(&self) -> &Script {
        &self.script_
    }

    fn start_pos(&self) -> i32 {
        self.start_pos_
    }

    fn end_pos(&self) -> i32 {
        self.end_pos_
    }

    fn bytecode_offset(&self) -> i32 {
        self.bytecode_offset_
    }

    fn shared(&self) -> &Option<SharedFunctionInfo> {
        &self.shared_
    }
}

// Placeholder for MessageHandler
struct MessageHandler {}

impl MessageHandler {
    fn default_message_report(
        isolate: &mut Isolate,
        loc: Option<&MessageLocation>,
        message_obj: DirectHandle<Object>,
    ) {
        let str = Self::get_localized_message(isolate, message_obj);
        if let Some(loc) = loc {
            let scope = HandleScope::new(isolate);
            let data = DirectHandle::new(loc.script().name());
            let data_str = data.value.to_cstring();

            println!("{}:{}: {}", data_str.unwrap_or_else(|| "<unknown>".to_string()), loc.start_pos(), str);
        } else {
            println!("{}", str);
        }
    }

    fn make_message_object(
        isolate: &mut Isolate,
        message: MessageTemplate,
        location: Option<&MessageLocation>,
        argument: DirectHandle<Object>,
        stack_trace: DirectHandle<StackTraceInfo>,
    ) -> DirectHandle<JSMessageObject> {
        let mut start = -1;
        let mut end = -1;
        let mut bytecode_offset = -1;
        let mut script_handle = isolate.factory().empty_script();
        let mut shared_info: Option<SharedFunctionInfo> = None;

        if let Some(loc) = location {
            if !v8_flags.correctness_fuzzer_suppressions {
                start = loc.start_pos();
                end = loc.end_pos();
                script_handle = loc.script().clone();
                bytecode_offset = loc.bytecode_offset();
                shared_info = loc.shared().clone();
            }
        }

        isolate.factory().new_js_message_object(
            message,
            argument,
            start,
            end,
            shared_info,
            bytecode_offset,
            script_handle,
            stack_trace,
        )
    }

    fn report_message(isolate: &mut Isolate, loc: Option<&MessageLocation>, message: DirectHandle<JSMessageObject>) {
        let api_message_obj = utils::message_to_local(&message.value);

        if api_message_obj.error_level() != K_MESSAGE_ERROR {
            Self::report_message_no_exceptions(isolate, loc, DirectHandle::new(message.value), Local::<Value> { _phantom: std::marker::PhantomData });
            return;
        }

        let mut exception: DirectHandle<Object> = DirectHandle::new(isolate.factory().undefined_value());
        if isolate.has_exception() {
            exception = direct_handle(isolate.exception(), isolate);
        }

        let mut exception_scope = ExceptionScope::new(isolate);
        isolate.clear_pending_message();

        if message.value.argument().is_js_object() {
            let scope = HandleScope::new(isolate);
            let argument = DirectHandle::new(message.value.argument());

            let maybe_stringified: MaybeDirectHandle<Object>;

            if argument.value.is_js_error() {
                maybe_stringified = Object::no_side_effects_to_string(isolate, DirectHandle::new(argument.value));
            } else {
                let mut catcher = TryCatch::new(isolate);
                catcher.set_verbose(false);
                catcher.set_capture_message(false);
                maybe_stringified = Object::to_string(isolate, DirectHandle::new(argument.value));
            }

            let mut stringified: DirectHandle<Object> = DirectHandle::new(Object{});
            if !maybe_stringified.to_handle(&mut stringified) {
                isolate.clear_pending_message();
                stringified = isolate.factory().exception_string();
            }
            message.value.set_argument(stringified.value);
        }

        let api_exception_obj = utils::to_local(&exception.value);
        Self::report_message_no_exceptions(isolate, loc, DirectHandle::new(message.value), api_exception_obj);
    }

    fn report_message_no_exceptions(
        isolate: &mut Isolate,
        loc: Option<&MessageLocation>,
        message: DirectHandle<Object>,
        api_exception_obj: Local<Value>,
    ) {
        let api_message_obj = utils::message_to_local(&message.value);
        let error_level = api_message_obj.error_level();

        let global_listeners = isolate.factory().message_listeners();
        let global_length = global_listeners.length();
        if global_length == 0 {
            Self::default_message_report(isolate, loc, message);
        } else {
            for i in 0..global_length {
                let scope = HandleScope::new(isolate);
                if global_listeners.get(i).is_undefined(isolate) {
                    continue;
                }
                let listener = global_listeners.get(i).cast::<FixedArray>();
                let callback_obj = listener.get(0).cast::<Foreign>();
                let message_levels = listener.get(2).to_int() as i32;

                if (message_levels & error_level) == 0 {
                    continue;
                }

                let callback = unsafe {
                    std::mem::transmute::<usize, MessageCallback>(
                        callback_obj.foreign_address::<u8>(), // Placeholder kMessageListenerTag
                    )
                };

                let callback_data = DirectHandle::new(listener.get(1));

                //RCS_SCOPE(isolate, RuntimeCallCounterId::kMessageListenerCallback);
                let mut try_catch = TryCatch::new(isolate);
                callback(
                    api_message_obj,
                    if callback_data.value.is_undefined(isolate) {
                        api_exception_obj
                    } else {
                        utils::to_local(&callback_data.value)
                    },
                );
            }
        }
    }

    fn get_message(isolate: &mut Isolate, data: DirectHandle<Object>) -> String {
        let message = data.value.cast::<JSMessageObject>();
        let arg = DirectHandle::new(message.argument());
        MessageFormatter::format(isolate, message.message_type(), VectorOf::new(vec![arg]))
    }

    fn get_localized_message(isolate: &mut Isolate, data: DirectHandle<Object>) -> String {
        let scope = HandleScope::new(isolate);
        Self::get_message(isolate, data)
    }
}

// Placeholder for Factory
struct Factory {}

impl Factory {
    fn empty_script(&self) -> Script {
        Script {} // Implement this
    }
    fn new_js_message_object(
        &self,
        _message: MessageTemplate,
        _argument: DirectHandle<Object>,
        _start: i32,
        _end: i32,
        _shared_info: Option<SharedFunctionInfo>,
        _bytecode_offset: i32,
        _script_handle: Script,
        _stack_trace: DirectHandle<StackTraceInfo>,
    ) -> DirectHandle<JSMessageObject> {
        DirectHandle::new(JSMessageObject {}) // Implement this
    }
    fn message_listeners(&self) -> ArrayList {
        ArrayList {} // Implement this
    }
    fn exception_string(&self) -> DirectHandle<Object> {
        DirectHandle::new(Object {}) // Implement this
    }

    fn cause_string(&self) -> DirectHandle<Name> {
        DirectHandle::new(Name {})
    }

    fn message_string(&self) -> DirectHandle<Name> {
        DirectHandle::new(Name {})
    }

    fn internalize_utf8_string(&self, s: &str) -> Object {
        Object {}
    }

    fn type_error_function(&self) -> JSFunction {
        JSFunction {}
    }

    fn error_stack_symbol(&self) -> DirectHandle<Name> {
        DirectHandle::new(Name {})
    }

    fn iterator_symbol(&self) -> DirectHandle<Symbol> {
        DirectHandle::new(Symbol {})
    }

    fn stack_string(&self) -> DirectHandle<Name> {
        DirectHandle::new(Name {})
    }

    fn error_stack_getter_fun_template(&self) -> Object {
        Object {}
    }
    fn error_stack_setter_fun_template(&self) -> Object {
        Object {}
    }
}

// Placeholder for Object
#[derive(Debug, Clone)]
struct Object {}

impl Object {
    fn to_cstring(&self) -> Option<String> {
        Some("object".to_string())
    }

    fn is_js_object(&self) -> bool {
        false
    }

    fn is_js_error(&self) -> bool {
        false
    }

    fn is_undefined(&self, _isolate: &Isolate) -> bool {
        false
    }

    fn cast<T>(&self) -> T {
        // Implement this
        unimplemented!()
    }

    fn no_side_effects_to_string(_isolate: &mut Isolate, arg: DirectHandle<Object>) -> MaybeDirectHandle<Object> {
        MaybeDirectHandle { value: Some(Object {}) }
    }

    fn to_string(isolate: &mut Isolate, argument: DirectHandle<Object>) -> MaybeDirectHandle<Object> {
        MaybeDirectHandle { value: Some(Object {}) }
    }

    fn type_of(isolate: &Isolate, object: DirectHandle<Object>) -> String {
        "object".to_string()
    }

    fn set_property(
        isolate: &mut Isolate,
        new_error: DirectHandle<JSObject>,
        symbol: DirectHandle<Name>,
        error_stack: DirectHandle<Object>,
        origin: StoreOrigin,
        should_throw: Just,
    ) -> Result<(), ()> {
        Ok(())
    }

    fn is_string(&self) -> bool {
        false
    }

    fn is_null(&self, isolate: &Isolate) -> bool {
        false
    }

    fn is_true(&self, isolate: &Isolate) -> bool {
        false
    }

    fn is_false(&self, isolate: &Isolate) -> bool {
        false
    }

    fn is_number(&self) -> bool {
        false
    }

    fn no_side_effects_to_maybe_string(
        isolate: &Isolate,
        key_handle: DirectHandle<Object>,
    ) -> MaybeDirectHandle<String> {
        MaybeDirectHandle { value: None }
    }

    fn is_primitive(&self) -> bool {
        false
    }
}

// Placeholder for String
#[derive(Debug, Clone)]
struct String {}

impl String {
    fn to_cstring(&self) -> Option<String> {
        Some("string".to_string())
    }

    fn length(&self) -> usize {
        0
    }
}

// Placeholder for Script
#[derive(Debug, Clone)]
struct Script {}

impl Script {
    fn name(&self) -> Object {
        Object {} // Implement this
    }
    fn source(&self) -> Object {
        Object {} // Implement this
    }
}

// Placeholder for JSMessageObject
#[derive(Debug, Clone)]
struct JSMessageObject {}

impl JSMessageObject {
    fn argument(&self) -> Object {
        Object {} // Implement this
    }
    fn set_argument(&mut self, _arg: Object) {
        // Implement this
    }
    fn message_type(&self) -> MessageTemplate {
        MessageTemplate::kMessageCount // Placeholder
    }
}

// Placeholder for StackTraceInfo
struct StackTraceInfo {}

// Placeholder for ArrayList
struct ArrayList {
}

impl ArrayList {
    fn length(&self) -> i32 {
        0 // Implement this
    }
    fn get(&self, _i: i32) -> Object {
        Object{} // Implement this
    }
}

// Placeholder for FixedArray
#[derive(Debug, Clone)]
struct FixedArray {}

impl FixedArray {
    fn get(&self, _i: i32) -> Object {
        Object {} // Implement this
    }
    fn cast<T>(&self) -> T {
        // Implement this
        unimplemented!()
    }
}

// Placeholder for Foreign
#[derive(Debug, Clone)]
struct Foreign {}

impl Foreign {
    fn foreign_address<T>(&self) -> usize {
        0 // Implement this
    }
}

// Placeholder for Smi
#[derive(Debug, Clone)]
struct Smi {}

impl Smi {
    fn to_int(&self) -> i64 {
        0
    }
}

// Placeholder for MessageTemplate
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MessageTemplate {
    kMessageCount, // Placeholder
    kConstAssign,
    kConstructorNotReceiver,
    kDataCloneErrorDetachedArrayBuffer,
    kDataCloneErrorOutOfMemory,
    kIncompatibleMethodReceiver,
    kInvalidArgument,
    kInvalidArrayLength,
    kInvalidAtomicAccessIndex,
    kInvalidDataViewLength,
    kInvalidIndex,
    kInvalidLhsInAssignment,
    kInvalidLhsInFor,
    kInvalidLhsInPostfixOp,
    kInvalidLhsInPrefixOp,
    kInvalidPrivateBrandReinitialization,
    kInvalidPrivateFieldReinitialization,
    kInvalidPrivateMemberWrite,
    kInvalidRegExpExecResult,
    kInvalidTimeValue,
    kInvalidWeakMapKey,
    kInvalidWeakSetValue,
    kIteratorReduceNoInitial,
    kJsonParseShortString,
    kJsonParseUnexpectedEOS,
    kJsonParseUnexpectedTokenEndStringWithContext,
    kJsonParseUnexpectedTokenShortString,
    kJsonParseUnexpectedTokenStartStringWithContext,
    kJsonParseUnexpectedTokenSurroundStringWithContext,
    kMustBePositive,
    kNotIterable,
    kNotTypedArray,
    kProxyNonObject,
    kProxyPrivate,
    kProxyRevoked,
    kProxyTrapReturnedFalsishFor,
    kReduceNoInitial,
    kSpreadIteratorSymbolNonCallable,
    kSymbolIteratorInvalid,
    kTopLevelAwaitStalled,
    kUndefinedOrNullToObject,
    kUnexpectedStrictReserved,
    kUnexpectedTokenIdentifier,
    kWeakRefsCleanupMustBeCallable,
    kShadowRealmErrorStackThrows,
    kShadowRealmErrorStackNonString,
    kNonCoercibleWithProperty,
    kNonCoercible,
    kNonObjectPropertyLoad,
    kNonObjectPropertyLoadWithProperty,
    kNotIterableNoSymbolLoad,
    kCalledNonCallable,
    kNotConstructor,
    kNotCallableOrIterable,
    kNotAsyncIterable,
    kNotCallableOrAsyncIterable,
    kDefineDisallowed,
}

// Placeholder for MessageFormatter
struct MessageFormatter {}

impl MessageFormatter {
    fn format(
        isolate: &mut Isolate,
        index: MessageTemplate,
        args: VectorOf<DirectHandle<Object>>,
    ) -> String {
        let arg_strings: Vec<String> = args
            .data
            .iter()
            .map(|arg| Object::no_side_effects_to_string(isolate, DirectHandle::new(arg.clone())))
            .map(|maybe_string| match maybe_string.value {
                Some(_obj) => "string".to_string(),
                None => "<error>".to_string()
            })
            .collect();

        let mut try_catch = TryCatch::new(isolate);
        try_catch.set_verbose(false);
        try_catch.set_capture_message(false);

        let maybe_result_string = MessageFormatter::try_format(isolate, index, VectorOf::new(
            arg_strings.iter().map(|arg| DirectHandle::new(String{})).collect())); // fix

        match maybe_result_string.value {
            Some(_result_string) => "string".to_string(), //fix
            None => {
                println!("MessageFormatter::TryFormat failed");
                "<error>".to_string()
            }
        }
    }

    fn template_string(index: MessageTemplate) -> &'static str {
        match index {
            MessageTemplate::kMessageCount => unreachable!(), // Placeholder
            MessageTemplate::kConstAssign => "ConstAssign",
            MessageTemplate::kConstructorNotReceiver => "ConstructorNotReceiver",
            MessageTemplate::kDataCloneErrorDetachedArrayBuffer => "DataCloneErrorDetachedArrayBuffer",
            MessageTemplate::kDataCloneErrorOutOfMemory => "DataCloneErrorOutOfMemory",
            MessageTemplate::kIncompatibleMethodReceiver => "IncompatibleMethodReceiver",
            MessageTemplate::kInvalidArgument => "InvalidArgument",
            MessageTemplate::kInvalidArrayLength => "InvalidArrayLength",
            MessageTemplate::kInvalidAtomicAccessIndex => "InvalidAtomicAccessIndex",
            MessageTemplate::kInvalidDataViewLength => "InvalidDataViewLength",
            MessageTemplate::kInvalidIndex => "InvalidIndex",
            MessageTemplate::kInvalidLhsInAssignment => "InvalidLhsInAssignment",
            MessageTemplate::kInvalidLhsInFor => "InvalidLhsInFor",
            MessageTemplate::kInvalidLhsInPostfixOp => "InvalidLhsInPostfixOp",
            MessageTemplate::kInvalidLhsInPrefixOp => "InvalidLhsInPrefixOp",
            MessageTemplate::kInvalidPrivateBrandReinitialization => "InvalidPrivateBrandReinitialization",
            MessageTemplate::kInvalidPrivateFieldReinitialization => "InvalidPrivateFieldReinitialization",
            MessageTemplate::kInvalidPrivateMemberWrite => "InvalidPrivateMemberWrite",
            MessageTemplate::kInvalidRegExpExecResult => "InvalidRegExpExecResult",
            MessageTemplate::kInvalidTimeValue => "InvalidTimeValue",
            MessageTemplate::kInvalidWeakMapKey => "InvalidWeakMapKey",
            MessageTemplate::kInvalidWeakSetValue => "InvalidWeakSetValue",
            MessageTemplate::kIteratorReduceNoInitial => "IteratorReduceNoInitial",
            MessageTemplate::kJsonParseShortString => "JsonParseShortString",
            MessageTemplate::kJsonParseUnexpectedEOS => "JsonParseUnexpectedEOS",
            MessageTemplate::kJsonParseUnexpectedTokenEndStringWithContext => "JsonParseUnexpectedTokenEndStringWithContext",
            MessageTemplate::kJsonParseUnexpectedTokenShortString => "JsonParseUnexpectedTokenShortString",
            MessageTemplate::kJsonParseUnexpectedTokenStartStringWithContext => "JsonParseUnexpectedTokenStartStringWithContext",
            MessageTemplate::kJsonParseUnexpectedTokenSurroundStringWithContext => "JsonParseUnexpectedTokenSurroundStringWithContext",
            MessageTemplate::kMustBePositive => "MustBePositive",
            MessageTemplate::kNotIterable => "NotIterable",
            MessageTemplate::kNotTypedArray => "NotTypedArray",
            MessageTemplate::kProxyNonObject => "ProxyNonObject",
            MessageTemplate::kProxyPrivate => "ProxyPrivate",
            MessageTemplate::kProxyRevoked => "ProxyRevoked",
            MessageTemplate::kProxyTrapReturnedFalsishFor => "ProxyTrapReturnedFalsishFor",
            MessageTemplate::kReduceNoInitial => "ReduceNoInitial",
            MessageTemplate::kSpreadIteratorSymbolNonCallable => "SpreadIteratorSymbolNonCallable",
            MessageTemplate::kSymbolIteratorInvalid => "SymbolIteratorInvalid",
            MessageTemplate::kTopLevelAwaitStalled => "TopLevelAwaitStalled",
            MessageTemplate::kUndefinedOrNullToObject => "UndefinedOrNullToObject",
            MessageTemplate::kUnexpectedStrictReserved => "UnexpectedStrictReserved",
            MessageTemplate::kUnexpectedTokenIdentifier => "UnexpectedTokenIdentifier",
            MessageTemplate::kWeakRefsCleanupMustBeCallable => "WeakRefsCleanupMustBeCallable",
            MessageTemplate::kShadowRealmErrorStackThrows => "ShadowRealmErrorStackThrows",
            MessageTemplate::kShadowRealmErrorStackNonString => "ShadowRealmErrorStackNonString",
            MessageTemplate::kNonCoercibleWithProperty => "NonCoercibleWithProperty",
            MessageTemplate::kNonCoercible => "NonCoercible",
            MessageTemplate::kNonObjectPropertyLoad => "NonObjectPropertyLoad",
            MessageTemplate::kNonObjectPropertyLoadWithProperty => "NonObjectPropertyLoadWithProperty",
            MessageTemplate::kNotIterableNoSymbolLoad => "NotIterableNoSymbolLoad",
            MessageTemplate::kCalledNonCallable => "CalledNonCallable",
            MessageTemplate::kNotConstructor => "NotConstructor",
            MessageTemplate::kNotCallableOrIterable => "NotCallableOrIterable",
            MessageTemplate::kNotAsyncIterable => "NotAsyncIterable",
            MessageTemplate::kNotCallableOrAsyncIterable => "NotCallableOrAsyncIterable",
            MessageTemplate::kDefineDisallowed => "DefineDisallowed",
        }
    }

    fn try_format(
        isolate: &mut Isolate,
        index: MessageTemplate,
        args: VectorOf<DirectHandle<String>>,
    ) -> MaybeDirectHandle<String> {
        let template_string = MessageFormatter::template_string(index);
        let mut builder = IncrementalStringBuilder::new(isolate);

        const TEMPLATES_WITH_MISMATCHED_ARGUMENTS: [MessageTemplate; 55] = [
            MessageTemplate::kConstAssign,
            MessageTemplate::kConstructorNotReceiver,
            MessageTemplate::kDataCloneErrorDetachedArrayBuffer,
            MessageTemplate::kDataCloneErrorOutOfMemory,
            MessageTemplate::kIncompatibleMethodReceiver,
            MessageTemplate::kInvalidArgument,
            MessageTemplate::kInvalidArrayLength,
            MessageTemplate::kInvalidAtomicAccessIndex,
            MessageTemplate::kInvalidDataViewLength,
            MessageTemplate::kInvalidIndex,
            MessageTemplate::kInvalidLhsInAssignment,
            MessageTemplate::kInvalidLhsInFor,
            MessageTemplate::kInvalidLhsInPostfixOp,
            MessageTemplate::kInvalidLhsInPrefixOp,
            MessageTemplate::kInvalidPrivateBrandReinitialization,
            MessageTemplate::kInvalidPrivateFieldReinitialization,
            MessageTemplate::kInvalidPrivateMemberWrite,
            MessageTemplate::kInvalidRegExpExecResult,
            MessageTemplate::kInvalidTimeValue,
            MessageTemplate::kInvalidWeakMapKey,
            MessageTemplate::kInvalidWeakSetValue,
            MessageTemplate::kIteratorReduceNoInitial,
            MessageTemplate::kJsonParseShortString,
            MessageTemplate::kJsonParseUnexpectedEOS,
            MessageTemplate::kJsonParseUnexpectedTokenEndStringWithContext,
            MessageTemplate::kJsonParseUnexpectedTokenShortString,
            MessageTemplate::kJsonParseUnexpectedTokenStartStringWithContext,
            MessageTemplate::kJsonParseUnexpectedTokenSurroundStringWithContext,
            MessageTemplate::kMustBePositive,
            MessageTemplate::kNotIterable,
            MessageTemplate::kNotTypedArray,
            MessageTemplate::kProxyNonObject,
            MessageTemplate::kProxyPrivate,
            MessageTemplate::kProxyRevoked,
            MessageTemplate::kProxyTrapReturnedFalsishFor,
            MessageTemplate::kReduceNoInitial,
            MessageTemplate::kSpreadIteratorSymbolNonCallable,
            MessageTemplate::kSymbolIteratorInvalid,
            MessageTemplate::kTopLevelAwaitStalled,
            MessageTemplate::kUndefinedOrNullToObject,
            MessageTemplate::kUnexpectedStrictReserved,
            MessageTemplate::kUnexpectedTokenIdentifier,
            MessageTemplate::kWeakRefsCleanupMustBeCallable,
            MessageTemplate::kShadowRealmErrorStackThrows,
            MessageTemplate::kShadowRealmErrorStackNonString,
            MessageTemplate::kNonCoercibleWithProperty,
            MessageTemplate::kNonCoercible,
            MessageTemplate::kNonObjectPropertyLoad,
            MessageTemplate::kNonObjectPropertyLoadWithProperty,
            MessageTemplate::kNotIterableNoSymbolLoad,
            MessageTemplate::kCalledNonCallable,
            MessageTemplate::kNotConstructor,
            MessageTemplate::kNotCallableOrIterable,
            MessageTemplate::kNotAsyncIterable,
            MessageTemplate::kNotCallableOrAsyncIterable,
            MessageTemplate::kDefineDisallowed,
        ];

        let mut remaining_args = &args.data[..];
        let mut chars = template_string.chars();

        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(next_c) = chars.next() {
                    if next_c == '%' {
                        builder.append_character('%');
                    } else {
                        if remaining_args.is_empty() {
                            if TEMPLATES_WITH_MISMATCHED_ARGUMENTS.contains(&index) {
                                builder.append_cstring("undefined");
                            } else {
                                println!("Missing argument to template: {}", template_string);
                                panic!("Missing argument to template");
                            }
                        } else {
                            let arg = &remaining_args[0];
                            remaining_args = &remaining_args[1..];
                            // builder.append_string(*arg); // fix
                            builder.append_cstring("string");
                        }
                        continue;
                    }
                }
            } else {
                builder.append_character(c);
            }
        }

        if !remaining_args.is_empty() && !TEMPLATES_WITH_MISMATCHED_ARGUMENTS.contains(&index) {
            println!("Too many arguments to template: {}", template_string);
            panic!("Too many arguments to template");
        }

        MaybeDirectHandle { value: Some(String{}) }
    }
}

// Placeholder for IncrementalStringBuilder
struct IncrementalStringBuilder {
    _isolate: *mut Isolate,
    _string: String,
}

impl IncrementalStringBuilder {
    fn new(isolate: &mut Isolate) -> Self {
        IncrementalStringBuilder {
            _isolate: isolate,
            _string: String::new(),
        }
    }

    fn append_character(&mut self, c: char) {
        self._string.push(c);
    }

    fn append_cstring(&mut self, s: &str) {
        self._string.push_str(s);
    }

    fn finish(&self) -> String {
        self._string.clone()
    }

    fn append_string(&mut self, string: &String) {
        self._string.push_str("string")
    }

    fn append_cstring_literal(&mut self, s: &str) {
        self._string.push_str(s);
    }
}

// Placeholder for ErrorUtils
struct ErrorUtils {}

impl ErrorUtils {
    fn construct(
        isolate: &mut Isolate,
        target: DirectHandle<JSFunction>,
        new_target: DirectHandle<Object>,
        message: DirectHandle<Object>,
        options: DirectHandle<Object>,
    ) -> MaybeDirectHandle<JSObject> {
        let mode = FrameSkipMode::SKIP_FIRST;
        let caller: DirectHandle<Object> = DirectHandle::new(Object{});

        ErrorUtils::construct_with_mode(
            isolate,
            target,
            new_target,
            message,
            options,
            mode,
            caller,
            StackTraceCollection::kEnabled,
        )
    }

    fn construct_with_mode(
        isolate: &mut Isolate,
        target: DirectHandle<JSFunction>,
        new_target: DirectHandle<Object>,
        message: DirectHandle<Object>,
        options: DirectHandle<Object>,
        mode: FrameSkipMode,
        caller: DirectHandle<Object>,
        stack_trace_collection: StackTraceCollection,
    ) -> MaybeDirectHandle<JSObject> {
        if v8_flags.correctness_fuzzer_suppressions {
            if target.value.is_identical_to(isolate.range_error_function()) {
                println!("Aborting on range error");
                panic!("Aborting on range error");
            }

            message.value = isolate.factory().internalize_utf8_string(
                "Message suppressed for fuzzers (--correctness-fuzzer-suppressions)",
            );
        }

        let new_target_recv: DirectHandle<JSReceiver> = if new_target.value.is_js_receiver() {
            new_target.value.cast::<JSReceiver>()
        } else {
            target.value.cast::<JSReceiver>()
        
// Converted from V8 C++ source files:
// Header: messages.h
// Implementation: messages.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2006-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The infrastructure used for (localized) message reporting in V8.
//
// Note: there's a big unresolved issue about ownership of the data
// structures used by this framework.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::sync::Mutex;

use crate::Error;
use crate::V8;

//use v8::base::Vector;
//use v8::common::message_template::MessageTemplate;
//use v8::handles::handles::Handle;
//use v8::handles::maybe_handles::MaybeHandle;

pub mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }
    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }
        pub fn empty() -> Self {
            Vector { data: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }
        pub fn as_slice(&self) -> &[T] {
            self.data.as_slice()
        }
    }

    pub struct VectorOf<'a, T> {
        data: &'a [T],
    }

    impl<'a, T> VectorOf<'a, T> {
        pub fn new(data: &'a [T]) -> Self {
            VectorOf { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }

        pub fn as_slice(&self) -> &[T] {
            self.data
        }
    }
}

pub mod common {
    pub mod message_template {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MessageTemplate {
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
            kShadowRealmErrorStackNonString,
            kShadowRealmErrorStackThrows,
            kNonObjectPropertyLoad,
            kNonObjectPropertyLoadWithProperty,
            kNonCoercible,
            kNonCoercibleWithProperty,
            kNotCallableOrIterable,
            kNotAsyncIterable,
            kNotCallableOrAsyncIterable,
            kNotIterableNoSymbolLoad,
            kCalledNonCallable,
            kNotConstructor,
            kDefineDisallowed,
            kMessageCount,
        }
    }
}

pub mod handles {
    pub mod handles {
        use std::rc::Rc;
        #[derive(Debug, Clone)]
        pub struct Handle<T> {
            pub value: Rc<T>,
        }

        impl<T> Handle<T> {
            pub fn new(value: Rc<T>) -> Self {
                Handle { value }
            }
        }
    }
    pub mod maybe_handles {
        use std::rc::Rc;

        #[derive(Debug, Clone)]
        pub struct MaybeHandle<T> {
            pub value: Option<Rc<T>>,
        }

        impl<T> MaybeHandle<T> {
            pub fn new(value: Option<Rc<T>>) -> Self {
                MaybeHandle { value }
            }
            pub fn to_handle(&self) -> Option<Rc<T>> {
                self.value.clone()
            }
            pub fn is_null(&self) -> bool {
                self.value.is_none()
            }
        }
    }
}

use self::base::Vector;
use self::common::message_template::MessageTemplate;
use self::handles::handles::Handle;
use self::handles::maybe_handles::MaybeHandle;
use crate::JSFunction;
use crate::Object;
use crate::SharedFunctionInfo;
use std::fmt;
use std::fmt::Debug;

pub struct Script {}
impl Script {
    pub fn name(&self) -> Handle<String> {
        Handle::new(Rc::new(String::from("<script_name>")))
    }
    pub fn source(&self) -> Handle<String> {
        Handle::new(Rc::new(String::from("<script_source>")))
    }
}
impl Debug for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Script").finish()
    }
}

pub struct String {
    pub data: String,
}

impl String {
    pub fn from(s: &str) -> String {
        String { data: s.to_string() }
    }
    pub fn utf8_value(&self) -> String {
        self.data.clone()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn to_c_string(&self) -> std::ffi::CString {
        std::ffi::CString::new(self.data.clone()).unwrap()
    }
    pub fn to_string(&self) -> String {
        self.data.clone()
    }
}
impl Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("String").field("data", &self.data).finish()
    }
}

pub mod isolate {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{JSObject, Object};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FrameSkipMode {
        SKIP_FIRST,
        SKIP_UNTIL_SEEN,
        SKIP_NONE,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StackTraceCollection {
        kEnabled,
        kDisabled,
    }

    thread_local! {
        pub static CURRENT_EXCEPTION: RefCell<Option<Rc<Object>>> = RefCell::new(None);
        pub static CURRENT_PENDING_MESSAGE: RefCell<bool> = RefCell::new(false);
    }

    pub fn has_exception() -> bool {
        CURRENT_EXCEPTION.with(|ex| ex.borrow().is_some())
    }

    pub fn exception() -> Option<Rc<Object>> {
        CURRENT_EXCEPTION.with(|ex| ex.borrow().clone())
    }

    pub fn clear_exception() {
        CURRENT_EXCEPTION.with(|ex| ex.borrow_mut().take());
    }

    pub fn set_exception(exception: Rc<Object>) {
        CURRENT_EXCEPTION.with(|ex| {
            *ex.borrow_mut() = Some(exception);
        });
    }

    pub fn clear_pending_message() {
        CURRENT_PENDING_MESSAGE.with(|pm| *pm.borrow_mut() = false);
    }
    pub fn set_pending_message() {
        CURRENT_PENDING_MESSAGE.with(|pm| *pm.borrow_mut() = true);
    }
    pub fn has_pending_message() -> bool {
        CURRENT_PENDING_MESSAGE.with(|pm| *pm.borrow())
    }
}

pub struct JSMessageObject {}
pub struct StackTraceInfo {}
pub struct ArrayList {}
pub struct FixedArray {}
pub struct Foreign {}
pub struct JSArray {}
pub struct CallSiteInfo {}
pub struct NativeContext {}
pub struct Symbol {}
pub struct JSError {}
pub struct ErrorStackData {}
pub struct Name {}
pub struct JSReceiver {}
pub struct ObjectLiteralProperty {}
pub struct JSGlobalObject {}
pub mod objects_inl {
    pub struct Object {}
}

pub struct Isolate {
    pub error_function: Rc<JSFunction>,
    pub type_error_function: Rc<JSFunction>,
    pub range_error_function: Rc<JSFunction>,
    pub factory: Factory,
    pub callsite_function: Rc<JSFunction>,
    pub error_stack_getter_fun_template: Rc<JSFunction>,
    pub error_stack_setter_fun_template: Rc<JSFunction>,
    pub formatting_stack_trace: RefCell<bool>,
    pub message_listeners: Rc<ArrayList>,
}
impl Isolate {
    pub fn new() -> Self {
        Isolate {
            error_function: Rc::new(JSFunction {}),
            type_error_function: Rc::new(JSFunction {}),
            range_error_function: Rc::new(JSFunction {}),
            factory: Factory::new(),
            callsite_function: Rc::new(JSFunction {}),
            error_stack_getter_fun_template: Rc::new(JSFunction {}),
            error_stack_setter_fun_template: Rc::new(JSFunction {}),
            formatting_stack_trace: RefCell::new(false),
            message_listeners: Rc::new(ArrayList {}),
        }
    }
    pub fn has_exception(&self) -> bool {
        isolate::has_exception()
    }
    pub fn exception(&self) -> Rc<Object> {
        isolate::exception().unwrap()
    }
    pub fn clear_exception(&self) {
        isolate::clear_exception()
    }
    pub fn set_exception(&self, exception: Rc<Object>) {
        isolate::set_exception(exception)
    }
    pub fn clear_pending_message(&self) {
        isolate::clear_pending_message()
    }
    pub fn set_pending_message(&self) {
        isolate::set_pending_message()
    }
    pub fn has_pending_message(&self) -> bool {
        isolate::has_pending_message()
    }

    pub fn CaptureAndSetErrorStack(
        &self,
        object: Handle<JSObject>,
        mode: isolate::FrameSkipMode,
        caller: Handle<Object>,
    ) -> Result<(), Error> {
        println!(
            "CaptureAndSetErrorStack: object={:?}, mode={:?}, caller={:?}",
            object, mode, caller
        );
        Ok(())
    }
    pub fn factory(&self) -> &Factory {
        &self.factory
    }

    pub fn internalize_string(&self, s: &str) -> Handle<String> {
        Handle::new(Rc::new(String::from(s)))
    }
    pub fn range_error_function(&self) -> Rc<JSFunction> {
        self.range_error_function.clone()
    }

    pub fn RunPrepareStackTraceCallback(
        &self,
        error_context: Handle<NativeContext>,
        error: Handle<JSObject>,
        sites: Handle<JSArray>,
    ) -> Result<Handle<Object>, Error> {
        println!(
            "RunPrepareStackTraceCallback: error_context={:?}, error={:?}, sites={:?}",
            error_context, error, sites
        );
        Ok(Handle::new(Rc::new(Object {})))
    }

    pub fn callsite_function(&self) -> Rc<JSFunction> {
        self.callsite_function.clone()
    }
    pub fn formatting_stack_trace(&self) -> bool {
        self.formatting_stack_trace.borrow().clone()
    }
    pub fn set_formatting_stack_trace(&self, value: bool) {
        *self.formatting_stack_trace.borrow_mut() = value;
    }
    pub fn type_error_function(&self) -> Rc<JSFunction> {
        self.type_error_function.clone()
    }
    pub fn ThrowAt(&self, error: Handle<JSObject>, location: &MessageLocation) {
        println!("ThrowAt: error={:?}, location={:?}", error, location);
        isolate::set_exception(Rc::new(Object {}));
    }
    pub fn Throw(&self, error: Handle<JSObject>) {
        println!("Throw: error={:?}", error);
        isolate::set_exception(Rc::new(Object {}));
    }

    pub fn HasPrepareStackTraceCallback(&self) -> bool {
        false // For now, always return false
    }

    pub fn CountUsage(&self, _usage: v8::Isolate::Usage) {}
}

impl Debug for Isolate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Isolate")
            .field("error_function", &self.error_function)
            .field("type_error_function", &self.type_error_function)
            .field("range_error_function", &self.range_error_function)
            .field("factory", &self.factory)
            .field("callsite_function", &self.callsite_function)
            .field(
                "error_stack_getter_fun_template",
                &self.error_stack_getter_fun_template,
            )
            .field(
                "error_stack_setter_fun_template",
                &self.error_stack_setter_fun_template,
            )
            .field("formatting_stack_trace", &self.formatting_stack_trace)
            .finish()
    }
}

pub struct Factory {
    pub empty_string: Rc<String>,
    pub message_string: Rc<String>,
    pub cause_string: Rc<String>,
    pub error_message_symbol: Rc<Symbol>,
    pub iterator_symbol: Rc<Symbol>,
    pub error_stack_symbol: Rc<Symbol>,
    pub stack_string: Rc<String>,
    pub undefined_value: Rc<Object>,
    pub exception_string: Rc<String>,
    pub empty_script: Rc<Script>,
}
impl Factory {
    pub fn new() -> Self {
        Factory {
            empty_string: Rc::new(String::from("")),
            message_string: Rc::new(String::from("message")),
            cause_string: Rc::new(String::from("cause")),
            error_message_symbol: Rc::new(Symbol {}),
            iterator_symbol: Rc::new(Symbol {}),
            error_stack_symbol: Rc::new(Symbol {}),
            stack_string: Rc::new(String::from("stack")),
            undefined_value: Rc::new(Object {}),
            exception_string: Rc::new(String::from("<exception>")),
            empty_script: Rc::new(Script {}),
        }
    }
    pub fn NewJSMessageObject(
        &self,
        _message: MessageTemplate,
        _argument: Handle<Object>,
        _start: i32,
        _end: i32,
        _shared_info: Handle<SharedFunctionInfo>,
        _bytecode_offset: i32,
        _script_handle: Handle<Script>,
        _stack_trace: Handle<StackTraceInfo>,
    ) -> Handle<JSMessageObject> {
        Handle::new(Rc::new(JSMessageObject {}))
    }
    pub fn empty_script(&self) -> Handle<Script> {
        Handle::new(self.empty_script.clone())
    }
    pub fn InternalizeString(&self, _s: base::StaticCharVector) -> Handle<String> {
        Handle::new(Rc::new(String::from("<internalized_string>")))
    }
    pub fn InternalizeUtf8String(&self, _s: &str) -> Handle<String> {
        Handle::new(Rc::new(String::from("<internalized_string>")))
    }
    pub fn NewTypeError(&self, _index: MessageTemplate, _args: Handle<String>) -> Handle<JSObject> {
        Handle::new(Rc::new(JSObject {}))
    }
    pub fn NewTypeError(
        &self,
        _index: MessageTemplate,
        _callsite: Handle<String>,
        _object: Handle<Object>,
    ) -> Handle<JSObject> {
        Handle::new(Rc::new(JSObject {}))
    }

    pub fn NewTypeError(
        &self,
        index: MessageTemplate,
        arg1: Handle<String>,
        arg2: Handle<String>,
    ) -> Handle<JSObject> {
        println!(
            "NewTypeError: index={:?}, arg1={:?}, arg2={:?}",
            index, arg1, arg2
        );
        Handle::new(Rc::new(JSObject {}))
    }
    pub fn NewFixedArray(&self, _length: i32) -> Handle<FixedArray> {
        Handle::new(Rc::new(FixedArray {}))
    }
    pub fn call_site_info_symbol(&self) -> Rc<Symbol> {
        self.call_site_info_symbol.clone()
    }
    pub fn NewJSArrayWithElements(&self, _sites: Handle<FixedArray>) -> Handle<JSArray> {
        Handle::new(Rc::new(JSArray {}))
    }
    pub fn name_string(&self) -> Rc<String> {
        self.name_string.clone()
    }
    pub fn Error_string(&self) -> Rc<String> {
        self.Error_string.clone()
    }
    pub fn NewStringFromAsciiChecked(&self, _s: &str) -> Handle<String> {
        Handle::new(Rc::new(String::from("<ascii_string>")))
    }
    pub fn message_string(&self) -> Rc<String> {
        self.message_string.clone()
    }
    pub fn cause_string(&self) -> Rc<String> {
        self.cause_string.clone()
    }
    pub fn error_message_symbol(&self) -> Rc<Symbol> {
        self.error_message_symbol.clone()
    }
    pub fn stack_string(&self) -> Rc<String> {
        self.stack_string.clone()
    }
    pub fn error_stack_symbol(&self) -> Rc<Symbol> {
        self.error_stack_symbol.clone()
    }
    pub fn iterator_symbol(&self) -> Rc<Symbol> {
        self.iterator_symbol.clone()
    }
    pub fn NewProperSubString(&self, string: Handle<String>, start: usize, end: usize) -> Handle<String> {
        let substring_data = string.value.data[start..end].to_string();
        Handle::new(Rc::new(String { data: substring_data }))
    }
    pub fn NumberToString(&self, object: Handle<Object>) -> Handle<String> {
        Handle::new(Rc::new(String { data: "<number_to_string>".to_string() }))
    }
    pub fn NewTypeError(
        &self,
        index: MessageTemplate,
        callsite: Handle<String>,
    ) -> Handle<JSObject> {
        println!(
            "NewTypeError: index={:?}, callsite={:?}",
            index, callsite
        );
        Handle::new(Rc::new(JSObject {}))
    }
    pub fn undefined_value(&self) -> Rc<Object> {
        self.undefined_value.clone()
    }
    pub fn exception_string(&self) -> Rc<String> {
        self.exception_string.clone()
    }
    pub fn NewTypeError(
        &self,
        index: MessageTemplate,
        callsite: Handle<String>,
        symbol: Rc<Symbol>,
    ) -> Handle<JSObject> {
        println!(
            "NewTypeError: index={:?}, callsite={:?}, symbol={:?}",
            index, callsite, symbol
        );
        Handle::new(Rc::new(JSObject {}))
    }
    pub fn NewTypeError(
        &self,
        index: MessageTemplate,
        arg1: Handle<String>,
        arg2: Handle<String>,
        arg3: Handle<Object>,
    ) -> Handle<JSObject> {
        println!(
            "NewTypeError: index={:?}, arg1={:?}, arg2={:?}, arg3={:?}",
            index, arg1, arg2, arg3
        );
        Handle::new(Rc::new(JSObject {}))
    }
    pub fn NewTypeError(
        &self,
        index: MessageTemplate,
        arg1: Handle<String>,
        arg2: Handle<Object>,
    ) -> Handle<JSObject> {
        println!(
            "NewTypeError: index={:?}, arg1={:?}, arg2={:?}",
            index, arg1, arg2
        );
        Handle::new(Rc::new(JSObject {}))
    }
}

impl Debug for Factory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Factory")
            .field("empty_string", &self.empty_string)
            .field("message_string", &self.message_string)
            .field("cause_string", &self.cause_string)
            .field("error_message_symbol", &self.error_message_symbol)
            .field("iterator_symbol", &self.iterator_symbol)
            .field("error_stack_symbol", &self.error_stack_symbol)
            .field("stack_string", &self.stack_string)
            .finish()
    }
}

pub struct UnoptimizedCompileFlags {}
impl UnoptimizedCompileFlags {
    pub fn ForFunctionCompile(isolate: &Isolate, shared: &SharedFunctionInfo) -> Self {
        println!(
            "UnoptimizedCompileFlags::ForFunctionCompile: isolate={:?}, shared={:?}",
            isolate, shared
        );
        UnoptimizedCompileFlags {}
    }
    pub fn set_is_reparse(&mut self, value: bool) {
        println!("UnoptimizedCompileFlags::set_is_reparse: value={:?}", value);
    }
}
pub struct UnoptimizedCompileState {}
pub struct ReusableUnoptimizedCompileState {
    isolate: *mut Isolate,
}
impl ReusableUnoptimizedCompileState {
    pub fn new(isolate: &Isolate) -> Self {
        ReusableUnoptimizedCompileState {
            isolate: isolate as *const Isolate as *mut Isolate,
        }
    }
}
pub struct ParseInfo {
    pub ast_value_factory: Rc<AstValueFactory>,
}
impl ParseInfo {
    pub fn new(
        isolate: &Isolate,
        flags: UnoptimizedCompileFlags,
        compile_state: &UnoptimizedCompileState,
        reusable_state: &ReusableUnoptimizedCompileState,
    ) -> Self {
        println!(
            "ParseInfo::new: isolate={:?}, flags={:?}, compile_state={:?}, reusable_state={:?}",
            isolate, flags, compile_state, reusable_state
        );
        ParseInfo {
            ast_value_factory: Rc::new(AstValueFactory {}),
        }
    }
    pub fn ast_value_factory(&self) -> &AstValueFactory {
        &self.ast_value_factory
    }
}

pub mod parsing {
    use crate::{ParseInfo, SharedFunctionInfo, Isolate, base::Vector, common::message_template::MessageTemplate};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ReportStatisticsMode {
        kNo,
    }
    pub fn ParseAny(
        info: &ParseInfo,
        shared: &SharedFunctionInfo,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        println!(
            "parsing::ParseAny: info={:?}, shared={:?}, isolate={:?}, mode={:?}",
            info, shared, isolate, mode
        );
        true
    }
}

pub struct AstValueFactory {}
impl AstValueFactory {
    pub fn Internalize(&self, isolate: &Isolate) {
        println!("AstValueFactory::Internalize: isolate={:?}", isolate);
    }
}

pub struct CallPrinter {
    pub error_hint: ErrorHint,
    pub spread_arg: *mut Object,
    pub destructuring_assignment: *mut Object,
    pub destructuring_prop: *mut ObjectLiteralProperty,
}

impl CallPrinter {
    pub fn new(isolate: &Isolate, is_user_javascript: bool) -> Self {
        println!(
            "CallPrinter::new: isolate={:?}, is_user_javascript={:?}",
            isolate, is_user_javascript
        );
        CallPrinter {
            error_hint: ErrorHint::kNone,
            spread_arg: std::ptr::null_mut(),
            destructuring_assignment: std::ptr::null_mut(),
            destructuring_prop: std::ptr::null_mut(),
        }
    }
    pub fn Print(&self, literal: &Object, start_pos: i32) -> Handle<String> {
        println!(
            "CallPrinter::Print: literal={:?}, start_pos={:?}",
            literal, start_pos
        );
        Handle::new(Rc::new(String::from("<printed_callsite>")))
    }

    pub fn spread_arg(&self) -> *mut Object {
        self.spread_arg
    }

    pub fn Print(
        &self,
        literal: &Object,
        start_pos: i32,
    ) -> Handle<String> {
        println!(
            "CallPrinter::Print: literal={:?}, start_pos={:?}",
            literal, start_pos
        );
        Handle::new(Rc::new(String::from("<printed_callsite>")))
    }

    pub fn GetErrorHint(&self) -> ErrorHint {
        self.error_hint
    }
    pub fn new(
        isolate: &Isolate,
        is_user_javascript: bool,
        spread_error_in_args_hint: SpreadErrorInArgsHint,
    ) -> Self {
        println!(
            "CallPrinter::new: isolate={:?}, is_user_javascript={:?}, spread_error_in_args_hint={:?}",
            isolate, is_user_javascript, spread_error_in_args_hint
        );
        CallPrinter {
            error_hint: ErrorHint::kNone,
            spread_arg: std::ptr::null_mut(),
            destructuring_assignment: std::ptr::null_mut(),
            destructuring_prop: std::ptr::null_mut(),
        }
    }

    pub fn destructuring_assignment(&self) -> *mut Object {
        self.destructuring_assignment
    }

    pub fn destructuring_prop(&self) -> *mut ObjectLiteralProperty {
        self.destructuring_prop
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorHint {
    kNormalIterator,
    kCallAndNormalIterator,
    kAsyncIterator,
    kCallAndAsyncIterator,
    kNone,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadErrorInArgsHint {
    kErrorInArgs,
}

pub struct MessageLocation {
    script_: Handle<Script>,
    start_pos_: i32,
    end_pos_: i32,
    bytecode_offset_: i32,
    shared_: Handle<SharedFunctionInfo>,
}

impl MessageLocation {
    pub fn new(script: Handle<Script>, start_pos: i32, end_pos: i32) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: start_pos,
            end_pos_: end_pos,
            bytecode_offset_: -1,
            shared_: Handle::new(Rc::new(SharedFunctionInfo {})),
        }
    }

    pub fn new_shared(
        script: Handle<Script>,
        start_pos: i32,
        end_pos: i32,
        shared: Handle<SharedFunctionInfo>,
    ) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: start_pos,
            end_pos_: end_pos,
            bytecode_offset_: -1,
            shared_: shared,
        }
    }

    pub fn new_bytecode_offset(
        script: Handle<Script>,
        shared: Handle<SharedFunctionInfo>,
        bytecode_offset: i32,
    ) -> Self {
        MessageLocation {
            script_: script,
            start_pos_: -1,
            end_pos_: -1,
            bytecode_offset_: bytecode_offset,
            shared_: shared,
        }
    }

    pub fn default() -> Self {
        MessageLocation {
            script_: Handle::new(Rc::new(Script {})),
            start_pos_: -1,
            end_pos_: -1,
            bytecode_offset_: -1,
            shared_: Handle::new(Rc::new(SharedFunctionInfo {})),
        }
    }

    pub fn script(&self) -> Handle<Script> {
        self.script_.clone()
    }
    pub fn start_pos(&self) -> i32 {
        self.start_pos_
    }
    pub fn end_pos(&self) -> i32 {
        self.end_pos_
    }
    pub fn bytecode_offset(&self) -> i32 {
        self.bytecode_offset_
    }
    pub fn shared(&self) -> Handle<SharedFunctionInfo> {
        self.shared_.clone()
    }
}

impl Debug for MessageLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessageLocation")
            .field("script_", &self.script_)
            .field("start_pos_", &self.start_pos_)
            .field("end_pos_", &self.end_pos_)
            .field("bytecode_offset_", &self.bytecode_offset_)
            .field("shared_", &self.shared_)
            .finish()
    }
}

// Determines how stack trace collection skips frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSkipMode {
    // Unconditionally skips the first frame. Used e.g. when the Error constructor
    // is called, in which case the first frame is always a BUILTIN_EXIT frame.
    SKIP_FIRST,
    // Skip all frames until a specified caller function is seen.
    SKIP_UNTIL_SEEN,
    SKIP_NONE,
}

pub struct ErrorUtils {}

impl ErrorUtils {
    // |kDisabled| is useful when you don't need the stack information at all, for
    // example when creating a deserialized error.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StackTraceCollection {
        kEnabled,
        kDisabled,
    }
    pub fn Construct(
        isolate: &mut Isolate,
        target: Handle<JSFunction>,
        new_target: Handle<Object>,
        message: Handle<Object>,
        options: Handle<Object>,
    ) -> MaybeHandle<JSObject> {
        let mode = if matches!(new_target.value.as_ref().unwrap(), JSFunction { .. }) {
            FrameSkipMode::SKIP_UNTIL_SEEN
        } else {
            FrameSkipMode::SKIP_FIRST
        };
        let caller = new_target;
        ErrorUtils::Construct(
            isolate,
            target,
            new_target,
            message,
            options,
            mode,
            caller,
            ErrorUtils::StackTraceCollection::kEnabled,
        )
    }

    pub fn Construct(
        isolate: &mut Isolate,
        target: Handle<JSFunction>,
        new_target: Handle<Object>,
        message: Handle<Object>,
        options: Handle<Object>,
        mode: FrameSkipMode,
        caller: Handle<Object>,
        stack_trace_collection: StackTraceCollection,
    ) -> MaybeHandle<JSObject> {
        println!(
            "Construct: target={:?}, new_target={:?}, message={:?}, options={:?}, mode={:?}, caller={:?}, stack_trace_collection={:?}",
            target, new_target, message, options, mode, caller, stack_trace_collection
        );

        let mut message_val = message;
        let js_options = options;

        let new_target_recv: Handle<JSReceiver> = Handle {
            value: if matches!(new_target.value.as_ref().unwrap(), JSReceiver { .. }) {
                Rc::new(JSReceiver {})
            } else {
                Rc::new(JSReceiver {})
            },
        };

        let err = Handle::new(Rc::new(JSObject {}));

        if !IsUndefined(message_val.value.as_ref().unwrap(), isolate) {
            println!("Message is

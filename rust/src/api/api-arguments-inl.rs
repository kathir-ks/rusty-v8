// src/api/api_arguments.rs

use std::any::Any;
use std::marker::PhantomData;
// use v8::Value;  // Assuming v8 crate provides this
// use v8::Boolean; // Assuming v8 crate provides this
// use v8::Integer; // Assuming v8 crate provides this
// use v8::Array; // Assuming v8 crate provides this
// use v8::None; // Assuming v8 crate provides this
// use v8::Utils; // Assuming v8 crate provides this
// use v8::PropertyDescriptor; // Assuming v8 crate provides this
// use v8::Intercepted; // Assuming v8 crate provides this

pub trait Relocatable {
    fn isolate(&self) -> &Isolate;
}

// Placeholder for Isolate
pub struct Isolate {}

impl Isolate {
    pub fn should_check_side_effects(&self) -> bool {
        false
    }
    pub fn debug(&self) -> &Debug {
        &Debug {}
    }
    pub fn has_exception(&self) -> bool {
        false
    }

    pub fn javascript_execution_counter(&self) -> u32 {
        0 // Placeholder
    }
}

// Placeholder for Debug
pub struct Debug {}

impl Debug {
    pub fn PerformSideEffectCheckForAccessor<T, U>(&self, _accessor_info: T, _receiver: U, _accessor_kind: AccessorKind) -> bool {
        true
    }
    pub fn PerformSideEffectCheckForInterceptor<T>(&self, _interceptor_info: T) -> bool {
        true
    }

    pub fn PerformSideEffectCheckForCallback<T>(&self, _function: T) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccessorKind {
    Getter,
    Setter
}

// Placeholder for Tagged<Object>
#[derive(Debug, Clone, Copy)]
pub struct Tagged<T> {
    _phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new() -> Self {
        Tagged {
            _phantom: PhantomData,
        }
    }
}

// Placeholder for JSObject
pub struct JSObject {}

// Placeholder for JSObjectOrUndefined
pub struct JSObjectOrUndefined {}

// Placeholder for InterceptorInfo
pub struct InterceptorInfo {
    is_named_flag: bool,
}

impl InterceptorInfo {
    pub fn is_named(&self) -> bool {
        self.is_named_flag
    }
    pub fn query(&self) -> usize {0}
    pub fn getter(&self) -> usize {0}
    pub fn descriptor(&self) -> usize {0}
    pub fn setter(&self) -> usize {0}
    pub fn definer(&self) -> usize {0}
    pub fn deleter(&self) -> usize {0}

}

// Placeholder for AccessorInfo
pub struct AccessorInfo {
    getter_addr: usize,
    setter_addr: usize,
}

impl AccessorInfo {
    pub fn getter(&self, _isolate: &Isolate) -> usize {
        self.getter_addr
    }
    pub fn setter(&self, _isolate: &Isolate) -> usize {
        self.setter_addr
    }
}

// Placeholder for Name
pub struct Name {}

impl Name {
    pub fn IsPrivate(&self) -> bool {
        false
    }
}

// Placeholder for Handle<T>
#[derive(Debug)]
pub struct Handle<T> {
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new() -> Self {
        Handle {
            _phantom: PhantomData,
        }
    }
    pub fn location(&self) -> *const T {
        std::ptr::null() // Placeholder
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// Placeholder for DirectHandle<T>
#[derive(Debug, Copy, Clone)]
pub struct DirectHandle<T> {
    _phantom: PhantomData<T>,
}

impl<T> DirectHandle<T> {
    pub fn new() -> Self {
        DirectHandle {
            _phantom: PhantomData,
        }
    }
}

// Placeholder for Boolean
#[derive(Debug)]
pub struct Boolean {}

// Placeholder for JSAny
pub struct JSAny {}

// Placeholder for Object
pub struct Object {}

// Placeholder for FunctionTemplateInfo
pub struct FunctionTemplateInfo {
    callback_addr: usize,
}

impl FunctionTemplateInfo {
    pub fn callback(&self, _isolate: &Isolate) -> usize {
        self.callback_addr
    }
}

// Placeholder for FullObjectSlot
struct FullObjectSlot {}

impl FullObjectSlot {
    fn store<T>(&self, _value: T) {}
}

// Placeholder for Smi
#[derive(Debug)]
struct Smi {}

impl Smi {
    fn FromInt(_value: i32) -> Self {
        Smi {}
    }
    fn zero() -> Self {
        Smi {}
    }
}

// Placeholder for ReadOnlyRoots
struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    fn undefined_value(&self) -> Tagged<Object> {
        Tagged::new()
    }
    fn true_value(&self) -> Tagged<Object> {
        Tagged::new()
    }
}

// Placeholder for ExternalCallbackScope
struct ExternalCallbackScope<'a> {
    _phantom: PhantomData<&'a ()>
}

impl<'a> ExternalCallbackScope<'a> {
    fn new<T, U>(_isolate: &Isolate, _function_addr: usize, _exception_context: T, _callback_info: U) -> Self {
        ExternalCallbackScope{
            _phantom: PhantomData
        }
    }
}

// Placeholder for PropertyCallbackInfo
struct PropertyCallbackInfo<T> {
    _phantom: PhantomData<T>
}

// Placeholder for PropertyAttributes
struct PropertyAttributes {}

// Placeholder for InterceptorResult
#[derive(Debug, PartialEq, Eq)]
enum InterceptorResult {
    kNotIntercepted,
    kTrue,
    kFalse,
}

// Placeholder for FunctionCallbackInfo
pub struct FunctionCallbackInfo<T> {
    _phantom: PhantomData<T>
}

// Placeholder for v8::Value
pub struct Value {}

// Placeholder for PropertyDescriptor
pub struct PropertyDescriptor {}

// Placeholder for v8 types
mod v8 {
    pub type FunctionCallback = unsafe extern "C" fn(info: FunctionCallbackInfo<Value>);
    pub type NamedPropertyQueryCallback = unsafe extern "C" fn(name: Local<Name>, info: &PropertyCallbackInfo<Integer>) -> Intercepted;
    pub type NamedPropertyGetterCallback = unsafe extern "C" fn(name: Local<Name>, info: &PropertyCallbackInfo<Value>) -> Intercepted;
    pub type NamedPropertySetterCallback = unsafe extern "C" fn(name: Local<Name>, value: Local<Object>, info: &PropertyCallbackInfo<std::ffi::c_void>) -> Intercepted;
    pub type NamedPropertyDefinerCallback = unsafe extern "C" fn(name: Local<Name>, desc: PropertyDescriptor, info: &PropertyCallbackInfo<std::ffi::c_void>) -> Intercepted;
    pub type NamedPropertyDeleterCallback = unsafe extern "C" fn(name: Local<Name>, info: &PropertyCallbackInfo<Boolean>) -> Intercepted;
    pub type NamedPropertyDescriptorCallback = unsafe extern "C" fn(name: Local<Name>, info: &PropertyCallbackInfo<Value>) -> Intercepted;
    pub type IndexedPropertyQueryCallbackV2 = unsafe extern "C" fn(index: u32, info: &PropertyCallbackInfo<Integer>) -> Intercepted;
    pub type IndexedPropertyGetterCallbackV2 = unsafe extern "C" fn(index: u32, info: &PropertyCallbackInfo<Value>) -> Intercepted;
    pub type IndexedPropertySetterCallbackV2 = unsafe extern "C" fn(index: u32, value: Local<Object>, info: &PropertyCallbackInfo<std::ffi::c_void>) -> Intercepted;
    pub type IndexedPropertyDefinerCallbackV2 = unsafe extern "C" fn(index: u32, desc: PropertyDescriptor, info: &PropertyCallbackInfo<std::ffi::c_void>) -> Intercepted;
    pub type IndexedPropertyDeleterCallbackV2 = unsafe extern "C" fn(index: u32, info: &PropertyCallbackInfo<Boolean>) -> Intercepted;
    pub type IndexedPropertyDescriptorCallbackV2 = unsafe extern "C" fn(index: u32, info: &PropertyCallbackInfo<Value>) -> Intercepted;
    pub type IndexedPropertyEnumeratorCallback = unsafe extern "C" fn(info: &PropertyCallbackInfo<Array>);

    #[derive(Debug, PartialEq, Eq)]
    pub enum Intercepted {
        kNo,
        kYes,
    }

    // Placeholder for Local<T>
    #[derive(Debug, Copy, Clone)]
    pub struct Local<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> Local<T> {
        pub fn new() -> Self {
            Local {
                _phantom: PhantomData,
            }
        }
    }
}

// Placeholder for CData
mod cdata {
    pub trait CData<T> {
        fn to_cdata(&self) -> T;
    }
}

//Placeholder to convert callback address
fn ToCData<T, const K: u32>(
    _isolate: &Isolate,
    callback_addr: usize,
) -> T {
    unsafe { std::mem::transmute(callback_addr) }
}

// Placeholder for FunctionAddr
macro_rules! FUNCTION_ADDR {
    ($f:expr) => {
        $f as usize
    };
}

// Placeholder for AcceptSideEffects
macro_rules! AcceptSideEffects {
    () => {};
}

// Placeholder for ShouldThrowOnError
macro_rules! ShouldThrowOnError {
    () => {
        false
    };
}

// Placeholder for IsTrue
fn IsTrue(_boolean: &Boolean, _isolate: &Isolate) -> bool {
    true
}

// Placeholder for Cast
fn Cast<T>(_handle: Handle<Object>) -> Handle<T> {
    Handle::new()
}

fn Is<T>(_full_object_slot: &FullObjectSlot) -> bool {
    true
}

// Placeholder for kApi
mod k_api {
    pub const NAMED_PROPERTY_QUERY_CALLBACK_TAG: u32 = 0;
    pub const NAMED_PROPERTY_GETTER_CALLBACK_TAG: u32 = 1;
    pub const NAMED_PROPERTY_SETTER_CALLBACK_TAG: u32 = 2;
    pub const NAMED_PROPERTY_DEFINER_CALLBACK_TAG: u32 = 3;
    pub const NAMED_PROPERTY_DELETER_CALLBACK_TAG: u32 = 4;
    pub const INDEXED_PROPERTY_QUERY_CALLBACK_TAG: u32 = 5;
    pub const INDEXED_PROPERTY_GETTER_CALLBACK_TAG: u32 = 6;
    pub const INDEXED_PROPERTY_SETTER_CALLBACK_TAG: u32 = 7;
    pub const INDEXED_PROPERTY_DEFINER_CALLBACK_TAG: u32 = 8;
    pub const INDEXED_PROPERTY_DELETER_CALLBACK_TAG: u32 = 9;
    pub const INDEXED_PROPERTY_ENUMERATOR_CALLBACK_TAG: u32 = 10;
    pub const NAMED_PROPERTY_DESCRIPTOR_CALLBACK_TAG: u32 = 11;
    pub const INDEXED_PROPERTY_DESCRIPTOR_CALLBACK_TAG: u32 = 12;
}

// Placeholder for ExceptionContext
#[derive(Debug, PartialEq, Eq)]
enum ExceptionContext {
    kConstructor,
    kOperation,
    kNamedQuery,
    kNamedGetter,
    kNamedSetter,
    kNamedDefiner,
    kNamedDeleter,
    kNamedEnumerator,
    kNamedDescriptor,
    kAttributeGet,
    kAttributeSet,
    kIndexedQuery,
    kIndexedGetter,
    kIndexedSetter,
    kIndexedDefiner,
    kIndexedDeleter,
    kIndexedEnumerator,
    kIndexedDescriptor,
}

#[allow(non_snake_case)]
#[macro_export]
macro_rules! PREPARE_CALLBACK_INFO_ACCESSOR {
    ($ISOLATE:expr, $F:expr, $API_RETURN_TYPE:ty, $ACCESSOR_INFO:expr, $RECEIVER:expr, $ACCESSOR_KIND:ident, $EXCEPTION_CONTEXT:ident) => {
        if $ISOLATE.should_check_side_effects() &&
            !$ISOLATE.debug().PerformSideEffectCheckForAccessor(
                $ACCESSOR_INFO,
                $RECEIVER,
                AccessorKind::Getter,
            ) {
            return None;
        }
        let callback_info: PropertyCallbackInfo<$API_RETURN_TYPE> =
            GetPropertyCallbackInfo::<$API_RETURN_TYPE>();
        let call_scope = ExternalCallbackScope::new(
            &$ISOLATE,
            FUNCTION_ADDR!($F),
            ExceptionContext::$EXCEPTION_CONTEXT,
            &callback_info,
        );
    };
}

#[allow(non_snake_case)]
#[macro_export]
macro_rules! PREPARE_CALLBACK_INFO_INTERCEPTOR {
    ($ISOLATE:expr, $F:expr, $API_RETURN_TYPE:ty, $INTERCEPTOR_INFO:expr, $EXCEPTION_CONTEXT:ident) => {
        if $ISOLATE.should_check_side_effects() &&
            !$ISOLATE.debug().PerformSideEffectCheckForInterceptor(
                $INTERCEPTOR_INFO,
            ) {
            return None;
        }
        let callback_info: PropertyCallbackInfo<$API_RETURN_TYPE> =
            GetPropertyCallbackInfo::<$API_RETURN_TYPE>();
        let call_scope = ExternalCallbackScope::new(
            &$ISOLATE,
            FUNCTION_ADDR!($F),
            ExceptionContext::$EXCEPTION_CONTEXT,
            &callback_info,
        );
    };
}

// Placeholder for RuntimeCallCounterId
#[derive(Debug)]
enum RuntimeCallCounterId {
    kFunctionCallback,
    kNamedQueryCallback,
    kNamedGetterCallback,
    kNamedSetterCallback,
    kNamedDefinerCallback,
    kNamedDeleterCallback,
    kNamedDescriptorCallback,
    kIndexedQueryCallback,
    kIndexedGetterCallback,
    kIndexedSetterCallback,
    kIndexedDefinerCallback,
    kIndexedDeleterCallback,
    kIndexedDescriptorCallback,
    kNamedEnumeratorCallback,
    kIndexedEnumeratorCallback,
    kAccessorGetterCallback,
    kAccessorSetterCallback,
}

// Placeholder for RCS_SCOPE macro
macro_rules! RCS_SCOPE {
    ($isolate:expr, $counter_id:expr) => {
        let _rcs_scope = RuntimeCallStatsScope::new($isolate, $counter_id);
    };
}

// Placeholder for RuntimeCallStatsScope
struct RuntimeCallStatsScope<'a> {
    _isolate: &'a Isolate,
    _counter_id: RuntimeCallCounterId,
}

impl<'a> RuntimeCallStatsScope<'a> {
    fn new(isolate: &'a Isolate, counter_id: RuntimeCallCounterId) -> Self {
        RuntimeCallStatsScope {
            _isolate: isolate,
            _counter_id: counter_id,
        }
    }
}

impl<'a> Drop for RuntimeCallStatsScope<'a> {
    fn drop(&mut self) {}
}

// Placeholder for GetPropertyCallbackInfo
fn GetPropertyCallbackInfo<T>() -> PropertyCallbackInfo<T> {
    PropertyCallbackInfo{
        _phantom: PhantomData,
    }
}

const K_RETURN_VALUE_INDEX: usize = 0;
const K_PROPERTY_KEY_INDEX: usize = 1;
const K_THIS_INDEX: usize = 2;
const K_HOLDER_INDEX: usize = 3;

pub struct CustomArgumentsBase {
    isolate: Isolate,
}

impl CustomArgumentsBase {
    pub fn new(isolate: Isolate) -> Self {
        CustomArgumentsBase { isolate }
    }
}

impl Relocatable for CustomArgumentsBase {
    fn isolate(&self) -> &Isolate {
        &self.isolate
    }
}

pub struct CustomArguments<T> {
    base: CustomArgumentsBase,
    slots: [Tagged<Object>; 4], // Placeholder for slots
    _phantom: PhantomData<T>,
}

impl<T> CustomArguments<T> {
    pub fn new(isolate: Isolate) -> Self {
        CustomArguments {
            base: CustomArgumentsBase::new(isolate),
            slots: [Tagged::new(), Tagged::new(), Tagged::new(), Tagged::new()], // Initialize slots
            _phantom: PhantomData,
        }
    }

    fn slot_at(&self, index: usize) -> &FullObjectSlot {
        // Assuming slots are properly initialized
        unsafe { std::mem::transmute(&self.slots[index]) }
    }

    fn isolate(&self) -> &Isolate {
        self.base.isolate()
    }

    fn GetReturnValue<V>(&self, _isolate: &Isolate) -> Handle<V> {
        let slot = self.slot_at(K_RETURN_VALUE_INDEX);
        // Placeholder checks and casts
        Handle::new()
    }
}

impl<T> Drop for CustomArguments<T> {
    fn drop(&mut self) {
        self.slot_at(K_RETURN_VALUE_INDEX).store(Tagged::new()); // Assuming kHandleZapValue equivalent
    }
}

pub struct PropertyCallbackArguments {
    custom_args: CustomArguments<PropertyCallbackArguments>,
    index_: u32,
    javascript_execution_counter_: u32,
}

impl PropertyCallbackArguments {
    pub fn new(isolate: Isolate) -> Self {
        PropertyCallbackArguments {
            custom_args: CustomArguments::new(isolate),
            index_: 0,
            javascript_execution_counter_: isolate.javascript_execution_counter(),
        }
    }

    fn isolate(&self) -> &Isolate {
        self.custom_args.isolate()
    }

    fn slot_at(&self, index: usize) -> &FullObjectSlot {
        self.custom_args.slot_at(index)
    }

    pub fn holder(&self) -> Tagged<JSObject> {
        let slot = self.slot_at(K_HOLDER_INDEX);
        unsafe { std::mem::transmute(*slot) }
    }

    pub fn receiver(&self) -> Tagged<Object> {
        unsafe { std::mem::transmute(*self.slot_at(K_THIS_INDEX)) }
    }

    pub fn CallNamedEnumerator(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
    ) -> DirectHandle<JSObjectOrUndefined> {
        assert!(interceptor.0.is_named());
        RCS_SCOPE!(self.isolate(), RuntimeCallCounterId::kNamedEnumeratorCallback);
        self.CallPropertyEnumerator(interceptor)
    }

    pub fn CallNamedQuery(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
    ) -> Option<DirectHandle<Object>> {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedQueryCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX).store(Smi::FromInt(0)); // v8::None
        let f: v8::NamedPropertyQueryCallback =
            ToCData(isolate, interceptor.0.query());
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Integer, interceptor, ExceptionContext::kNamedQuery);
        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), &GetPropertyCallbackInfo::<v8::Integer>())
        };
        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<Object>(isolate))
    }

    pub fn CallNamedGetter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
    ) -> Option<DirectHandle<JSAny>> {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedGetterCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.undefined_value());
        let f: v8::NamedPropertyGetterCallback =
            ToCData(isolate, interceptor.0.getter());
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Value, interceptor, ExceptionContext::kNamedGetter);
        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), &GetPropertyCallbackInfo::<v8::Value>())
        };

        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<JSAny>(isolate))
    }

    pub fn CallNamedDescriptor(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
    ) -> Option<Handle<JSAny>> {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedDescriptorCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.undefined_value());

        let f: v8::NamedPropertyDescriptorCallback =
            ToCData(isolate, interceptor.0.descriptor());

        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Value, interceptor, ExceptionContext::kNamedDescriptor);
        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), &GetPropertyCallbackInfo::<v8::Value>())
        };
        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<JSAny>(isolate))
    }

    pub fn CallNamedSetter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
        value: DirectHandle<Object>,
    ) -> v8::Intercepted {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedSetterCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::NamedPropertySetterCallback =
            ToCData(isolate, interceptor.0.setter());
        let has_side_effects: DirectHandle<InterceptorInfo> = DirectHandle::new();
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, std::ffi::c_void, has_side_effects, ExceptionContext::kNamedSetter);
        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), v8::Local::new(), &GetPropertyCallbackInfo::<std::ffi::c_void>())
        };
        return intercepted;
    }

    pub fn CallNamedDefiner(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
        desc: &v8::PropertyDescriptor,
    ) -> v8::Intercepted {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedDefinerCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::NamedPropertyDefinerCallback =
            ToCData(isolate, interceptor.0.definer());

        let has_side_effects: DirectHandle<InterceptorInfo> = DirectHandle::new();
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, std::ffi::c_void, has_side_effects, ExceptionContext::kNamedDefiner);

        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), *desc, &GetPropertyCallbackInfo::<std::ffi::c_void>())
        };
        return intercepted;
    }

    pub fn CallNamedDeleter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        name: DirectHandle<Name>,
    ) -> v8::Intercepted {
        //DCHECK_NAME_COMPATIBLE(interceptor, name);
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedDeleterCallback);
        self.slot_at(K_PROPERTY_KEY_INDEX).store(*name);
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::NamedPropertyDeleterCallback =
            ToCData(isolate, interceptor.0.deleter());

        let has_side_effects: DirectHandle<InterceptorInfo> = DirectHandle::new();
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Boolean, has_side_effects, ExceptionContext::kNamedDeleter);
        let intercepted: v8::Intercepted = unsafe {
            f(v8::Local::new(), &GetPropertyCallbackInfo::<v8::Boolean>())
        };
        return intercepted;
    }

    pub fn CallIndexedEnumerator(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
    ) -> DirectHandle<JSObjectOrUndefined> {
        assert!(!interceptor.0.is_named());
        RCS_SCOPE!(self.isolate(), RuntimeCallCounterId::kIndexedEnumeratorCallback);
        self.CallPropertyEnumerator(interceptor)
    }

    pub fn CallIndexedQuery(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
    ) -> Option<DirectHandle<Object>> {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kIndexedQueryCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX).store(Smi::FromInt(0)); // v8::None

        let f: v8::IndexedPropertyQueryCallbackV2 =
            ToCData(isolate, interceptor.0.query());

        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Integer, interceptor, ExceptionContext::kIndexedQuery);
        let intercepted: v8::Intercepted = unsafe {
            f(index, &GetPropertyCallbackInfo::<v8::Integer>())
        };
        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<Object>(isolate))
    }

    pub fn CallIndexedGetter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
    ) -> Option<DirectHandle<JSAny>> {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kNamedGetterCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.undefined_value());

        let f: v8::IndexedPropertyGetterCallbackV2 =
            ToCData(isolate, interceptor.0.getter());

        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Value, interceptor, ExceptionContext::kIndexedGetter);
        let intercepted: v8::Intercepted = unsafe {
            f(index, &GetPropertyCallbackInfo::<v8::Value>())
        };

        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<JSAny>(isolate))
    }

    pub fn CallIndexedDescriptor(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
    ) -> Option<Handle<JSAny>> {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kIndexedDescriptorCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.undefined_value());

        let f: v8::IndexedPropertyDescriptorCallbackV2 =
            ToCData(isolate, interceptor.0.descriptor());

        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Value, interceptor, ExceptionContext::kIndexedDescriptor);
        let intercepted: v8::Intercepted = unsafe {
            f(index, &GetPropertyCallbackInfo::<v8::Value>())
        };

        if intercepted == v8::Intercepted::kNo {
            return None;
        }
        Some(self.custom_args.GetReturnValue::<JSAny>(isolate))
    }

    pub fn CallIndexedSetter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
        value: DirectHandle<Object>,
    ) -> v8::Intercepted {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kIndexedSetterCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::IndexedPropertySetterCallbackV2 =
            ToCData(isolate, interceptor.0.setter());

        let has_side_effects: DirectHandle<InterceptorInfo> = DirectHandle::new();
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, std::ffi::c_void, has_side_effects, ExceptionContext::kIndexedSetter);
        let intercepted: v8::Intercepted = unsafe {
            f(index, v8::Local::new(), &GetPropertyCallbackInfo::<std::ffi::c_void>())
        };
        return intercepted;
    }

    pub fn CallIndexedDefiner(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
        desc: &v8::PropertyDescriptor,
    ) -> v8::Intercepted {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kIndexedDefinerCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::IndexedPropertyDefinerCallbackV2 =
            ToCData(isolate, interceptor.0.definer());

        let has_side_effects: DirectHandle<InterceptorInfo> = DirectHandle::new();
        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, std::ffi::c_void, has_side_effects, ExceptionContext::kIndexedDefiner);
        let intercepted: v8::Intercepted = unsafe {
            f(index, *desc, &GetPropertyCallbackInfo::<std::ffi::c_void>())
        };
        return intercepted;
    }

    pub fn CallIndexedDeleter(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
        index: u32,
    ) -> v8::Intercepted {
        assert!(!interceptor.0.is_named());
        let isolate = self.isolate();
        RCS_SCOPE!(isolate, RuntimeCallCounterId::kIndexedDeleterCallback);
        self.index_ = index;
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero()); // indexed callback marker
        self.slot_at(K_RETURN_VALUE_INDEX)
            .store(ReadOnlyRoots {}.true_value());

        let f: v8::IndexedPropertyDeleterCallbackV2 =
            ToCData(isolate, interceptor.0.deleter());

        PREPARE_CALLBACK_INFO_INTERCEPTOR!(isolate, f, v8::Boolean, interceptor, ExceptionContext::kIndexedDeleter);
        let intercepted: v8::Intercepted = unsafe {
            f(index, &GetPropertyCallbackInfo::<v8::Boolean>())
        };
        return intercepted;
    }

    fn CallPropertyEnumerator(
        &self,
        interceptor: DirectHandle<InterceptorInfo>,
    ) -> DirectHandle<JSObjectOrUndefined> {
        let isolate = self.isolate();
        self.slot_at(K_PROPERTY_KEY_INDEX).store(Smi::zero
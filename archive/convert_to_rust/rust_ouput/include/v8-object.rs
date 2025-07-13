// Converted from V8 C++ source files:
// Header: v8-object.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct Private {}

impl Private {
    pub fn Name(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    pub fn New(isolate: *mut Isolate, name: Local<String>) -> Local<Private> {
        Local::Private(Private {})
    }

    pub fn ForApi(isolate: *mut Isolate, name: Local<String>) -> Local<Private> {
        Local::Private(Private {})
    }

    pub fn Cast(data: *mut Data) -> *mut Private {
        data as *mut Private
    }

    fn CheckCast(that: *mut Data) {}

    fn new() -> Private {
        Private {}
    }
}

pub struct PropertyDescriptor {
    private_: *mut PrivateData,
}

impl PropertyDescriptor {
    pub fn new() -> PropertyDescriptor {
        PropertyDescriptor {
            private_: std::ptr::null_mut(),
        }
    }

    pub fn with_value(value: Local<Value>) -> PropertyDescriptor {
        PropertyDescriptor {
            private_: std::ptr::null_mut(),
        }
    }

    pub fn with_value_and_writable(value: Local<Value>, writable: bool) -> PropertyDescriptor {
        PropertyDescriptor {
            private_: std::ptr::null_mut(),
        }
    }

    pub fn with_get_and_set(get: Local<Value>, set: Local<Value>) -> PropertyDescriptor {
        PropertyDescriptor {
            private_: std::ptr::null_mut(),
        }
    }

    pub fn value(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    pub fn has_value(&self) -> bool {
        false
    }

    pub fn get(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    pub fn has_get(&self) -> bool {
        false
    }

    pub fn set(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    pub fn has_set(&self) -> bool {
        false
    }

    pub fn set_enumerable(&mut self, enumerable: bool) {}

    pub fn enumerable(&self) -> bool {
        false
    }

    pub fn has_enumerable(&self) -> bool {
        false
    }

    pub fn set_configurable(&mut self, configurable: bool) {}

    pub fn configurable(&self) -> bool {
        false
    }

    pub fn has_configurable(&self) -> bool {
        false
    }

    pub fn writable(&self) -> bool {
        false
    }

    pub fn has_writable(&self) -> bool {
        false
    }

    pub fn get_private(&self) -> *mut PrivateData {
        self.private_
    }
}

impl Drop for PropertyDescriptor {
    fn drop(&mut self) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyAttribute {
    None = 0,
    ReadOnly = 1 << 0,
    DontEnum = 1 << 1,
    DontDelete = 1 << 2,
}

pub type AccessorNameGetterCallback =
    unsafe extern "C" fn(property: Local<Name>, info: &PropertyCallbackInfo<Value>);
pub type AccessorNameSetterCallback =
    unsafe extern "C" fn(property: Local<Name>, value: Local<Value>, info: &PropertyCallbackInfo<void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deprecated(since = "V8 12.9", note = "This enum is no longer used and will be removed in V8 12.9.")]
pub enum AccessControl {
    #[deprecated(since = "V8 12.9", note = "not used")]
    DEFAULT = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyFilter {
    ALL_PROPERTIES = 0,
    ONLY_WRITABLE = 1,
    ONLY_ENUMERABLE = 2,
    ONLY_CONFIGURABLE = 4,
    SKIP_STRINGS = 8,
    SKIP_SYMBOLS = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideEffectType {
    kHasSideEffect,
    kHasNoSideEffect,
    kHasSideEffectToReceiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCollectionMode {
    kOwnOnly,
    kIncludePrototypes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFilter {
    kIncludeIndices,
    kSkipIndices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyConversionMode {
    kConvertToString,
    kKeepNumbers,
    kNoNumbers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityLevel {
    kFrozen,
    kSealed,
}

impl Object {
    pub fn Set(
        &self,
        context: Local<Context>,
        key: Local<Value>,
        value: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn Set_receiver(
        &self,
        context: Local<Context>,
        key: Local<Value>,
        value: Local<Value>,
        receiver: MaybeLocal<Object>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn Set_index(
        &self,
        context: Local<Context>,
        index: u32,
        value: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn CreateDataProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
        value: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn CreateDataProperty_index(
        &self,
        context: Local<Context>,
        index: u32,
        value: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn DefineOwnProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
        value: Local<Value>,
        attributes: PropertyAttribute,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn DefineProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
        descriptor: &mut PropertyDescriptor,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn Get(
        &self,
        context: Local<Context>,
        key: Local<Value>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn Get_receiver(
        &self,
        context: Local<Context>,
        key: Local<Value>,
        receiver: MaybeLocal<Object>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn Get_index(
        &self,
        context: Local<Context>,
        index: u32,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn GetPropertyAttributes(
        &self,
        context: Local<Context>,
        key: Local<Value>,
    ) -> Maybe<PropertyAttribute> {
        Maybe::Just(PropertyAttribute::None)
    }

    pub fn GetOwnPropertyDescriptor(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn Has(
        &self,
        context: Local<Context>,
        key: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn Delete(
        &self,
        context: Local<Context>,
        key: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn Has_index(
        &self,
        context: Local<Context>,
        index: u32,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn Delete_index(
        &self,
        context: Local<Context>,
        index: u32,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn SetAccessorProperty(
        &self,
        name: Local<Name>,
        getter: Local<Function>,
        setter: Local<Function>,
        attributes: PropertyAttribute,
    ) {
    }

    pub fn SetNativeDataProperty(
        &self,
        context: Local<Context>,
        name: Local<Name>,
        getter: AccessorNameGetterCallback,
        setter: AccessorNameSetterCallback,
        data: Local<Value>,
        attributes: PropertyAttribute,
        getter_side_effect_type: SideEffectType,
        setter_side_effect_type: SideEffectType,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn SetLazyDataProperty(
        &self,
        context: Local<Context>,
        name: Local<Name>,
        getter: AccessorNameGetterCallback,
        data: Local<Value>,
        attributes: PropertyAttribute,
        getter_side_effect_type: SideEffectType,
        setter_side_effect_type: SideEffectType,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn HasPrivate(
        &self,
        context: Local<Context>,
        key: Local<Private>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn SetPrivate(
        &self,
        context: Local<Context>,
        key: Local<Private>,
        value: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn DeletePrivate(
        &self,
        context: Local<Context>,
        key: Local<Private>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn GetPrivate(
        &self,
        context: Local<Context>,
        key: Local<Private>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn GetPropertyNames(
        &self,
        context: Local<Context>,
    ) -> MaybeLocal<Array> {
        MaybeLocal::empty()
    }

    pub fn GetPropertyNames_mode(
        &self,
        context: Local<Context>,
        mode: KeyCollectionMode,
        property_filter: PropertyFilter,
        index_filter: IndexFilter,
        key_conversion: KeyConversionMode,
    ) -> MaybeLocal<Array> {
        MaybeLocal::empty()
    }

    pub fn GetOwnPropertyNames(
        &self,
        context: Local<Context>,
    ) -> MaybeLocal<Array> {
        MaybeLocal::empty()
    }

    pub fn GetOwnPropertyNames_filter(
        &self,
        context: Local<Context>,
        filter: PropertyFilter,
        key_conversion: KeyConversionMode,
    ) -> MaybeLocal<Array> {
        MaybeLocal::empty()
    }

    #[deprecated(
        since = "V8 will stop providing access to hidden prototype (i.e. JSGlobalObject). Use GetPrototypeV2() instead. See http://crbug.com/333672197."
    )]
    pub fn GetPrototype(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    pub fn GetPrototypeV2(&self) -> Local<Value> {
        Local::Value(Value::Undefined)
    }

    #[deprecated(
        since = "V8 will stop providing access to hidden prototype (i.e. JSGlobalObject). Use SetPrototypeV2() instead. See http://crbug.com/333672197."
    )]
    pub fn SetPrototype(
        &self,
        context: Local<Context>,
        prototype: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn SetPrototypeV2(
        &self,
        context: Local<Context>,
        prototype: Local<Value>,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn FindInstanceInPrototypeChain(
        &self,
        tmpl: Local<FunctionTemplate>,
    ) -> Local<Object> {
        Local::Object(Object {})
    }

    pub fn ObjectProtoToString(
        &self,
        context: Local<Context>,
    ) -> MaybeLocal<String> {
        MaybeLocal::empty()
    }

    pub fn GetConstructorName(&self) -> Local<String> {
        Local::String(String::from("Object"))
    }

    pub fn SetIntegrityLevel(
        &self,
        context: Local<Context>,
        level: IntegrityLevel,
    ) -> Maybe<bool> {
        Maybe::Just(true)
    }

    pub fn InternalFieldCount(&self) -> i32 {
        0
    }

    pub fn InternalFieldCount_static(object: &PersistentBase<Object>) -> i32 {
        object.value::<Object>().InternalFieldCount()
    }

    pub fn InternalFieldCount_traced(object: &BasicTracedReference<Object>) -> i32 {
        object.value::<Object>().InternalFieldCount()
    }

    pub fn GetInternalField(&self, index: i32) -> Local<Data> {
        Local::Data(Data {})
    }

    pub fn SetInternalField(&self, index: i32, data: Local<Data>) {}

    pub fn GetAlignedPointerFromInternalField(&self, index: i32) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    pub fn GetAlignedPointerFromInternalField_isolate(
        &self,
        isolate: *mut Isolate,
        index: i32,
    ) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    pub fn GetAlignedPointerFromInternalField_static(
        object: &PersistentBase<Object>,
        index: i32,
    ) -> *mut std::ffi::c_void {
        object.value::<Object>().GetAlignedPointerFromInternalField(index)
    }

    pub fn GetAlignedPointerFromInternalField_traced(
        object: &BasicTracedReference<Object>,
        index: i32,
    ) -> *mut std::ffi::c_void {
        object.value::<Object>().GetAlignedPointerFromInternalField(index)
    }

    pub fn SetAlignedPointerInInternalField(&self, index: i32, value: *mut std::ffi::c_void) {}

    pub fn SetAlignedPointerInInternalFields(
        &self,
        argc: i32,
        indices: &mut [i32],
        values: &mut [*mut std::ffi::c_void],
    ) {
    }

    pub fn Unwrap<T>(isolate: *mut Isolate, wrapper: &Local<Object>) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Unwrap_persistent<T>(isolate: *mut Isolate, wrapper: &PersistentBase<Object>) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Unwrap_traced<T>(isolate: *mut Isolate, wrapper: &BasicTracedReference<Object>) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Unwrap_tag_range<T>(isolate: *mut Isolate, wrapper: &Local<Object>, tag_range: CppHeapPointerTagRange) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Unwrap_persistent_tag_range<T>(isolate: *mut Isolate, wrapper: &PersistentBase<Object>, tag_range: CppHeapPointerTagRange) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Unwrap_traced_tag_range<T>(isolate: *mut Isolate, wrapper: &BasicTracedReference<Object>, tag_range: CppHeapPointerTagRange) -> *mut T {
        std::ptr::null_mut()
    }

    pub fn Wrap(isolate: *mut Isolate, wrapper: &Local<Object>, wrappable: *mut std::ffi::c_void) {}

    pub fn Wrap_persistent(isolate: *mut Isolate, wrapper: &PersistentBase<Object>, wrappable: *mut std::ffi::c_void) {}

    pub fn Wrap_traced(isolate: *mut Isolate, wrapper: &BasicTracedReference<Object>, wrappable: *mut std::ffi::c_void) {}

    pub fn Wrap_tag(isolate: *mut Isolate, wrapper: &Local<Object>, wrappable: *mut std::ffi::c_void, tag: CppHeapPointerTag) {}

    pub fn Wrap_persistent_tag(isolate: *mut Isolate, wrapper: &PersistentBase<Object>, wrappable: *mut std::ffi::c_void, tag: CppHeapPointerTag) {}

    pub fn Wrap_traced_tag(isolate: *mut Isolate, wrapper: &BasicTracedReference<Object>, wrappable: *mut std::ffi::c_void, tag: CppHeapPointerTag) {}

    pub fn HasOwnProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn HasOwnProperty_index(
        &self,
        context: Local<Context>,
        index: u32,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn HasRealNamedProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn HasRealIndexedProperty(
        &self,
        context: Local<Context>,
        index: u32,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn HasRealNamedCallbackProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn GetRealNamedPropertyInPrototypeChain(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn GetRealNamedPropertyAttributesInPrototypeChain(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> Maybe<PropertyAttribute> {
        Maybe::Just(PropertyAttribute::None)
    }

    pub fn GetRealNamedProperty(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn GetRealNamedPropertyAttributes(
        &self,
        context: Local<Context>,
        key: Local<Name>,
    ) -> Maybe<PropertyAttribute> {
        Maybe::Just(PropertyAttribute::None)
    }

    pub fn HasNamedLookupInterceptor(&self) -> bool {
        false
    }

    pub fn HasIndexedLookupInterceptor(&self) -> bool {
        false
    }

    pub fn GetIdentityHash(&self) -> i32 {
        1
    }

    pub fn Clone(&self, isolate: *mut Isolate) -> Local<Object> {
        Local::Object(Object {})
    }

    pub fn Clone_no_isolate(&self) -> Local<Object> {
        Local::Object(Object {})
    }

    pub fn GetCreationContext(&self, isolate: *mut Isolate) -> MaybeLocal<Context> {
        MaybeLocal::empty()
    }

    #[deprecated(since = "Use the version with the isolate argument.")]
    pub fn GetCreationContext_no_isolate(&self) -> MaybeLocal<Context> {
        MaybeLocal::empty()
    }

    pub fn GetCreationContextChecked(&self, isolate: *mut Isolate) -> Local<Context> {
        Local::Context(Context {})
    }

    #[deprecated(since = "Use the version with the isolate argument.")]
    pub fn GetCreationContextChecked_no_isolate(&self) -> Local<Context> {
        Local::Context(Context {})
    }

    pub fn GetCreationContext_static(isolate: *mut Isolate, object: &PersistentBase<Object>) -> MaybeLocal<Context> {
        object.value::<Object>().GetCreationContext(isolate)
    }

    #[deprecated(since = "Use the version with the isolate argument.")]
    pub fn GetCreationContext_static_no_isolate(object: &PersistentBase<Object>) -> MaybeLocal<Context> {
        MaybeLocal::empty()
    }

    pub fn GetAlignedPointerFromEmbedderDataInCreationContext(
        &self,
        isolate: *mut Isolate,
        index: i32,
    ) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    pub fn GetAlignedPointerFromEmbedderDataInCreationContext_no_isolate(
        &self,
        index: i32,
    ) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    pub fn IsCallable(&self) -> bool {
        false
    }

    pub fn IsConstructor(&self) -> bool {
        false
    }

    pub fn IsApiWrapper(&self) -> bool {
        false
    }

    pub fn IsUndetectable(&self) -> bool {
        false
    }

    pub fn CallAsFunction(
        &self,
        context: Local<Context>,
        recv: Local<Value>,
        argc: i32,
        argv: &mut [Local<Value>],
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn CallAsConstructor(
        &self,
        context: Local<Context>,
        argc: i32,
        argv: &mut [Local<Value>],
    ) -> MaybeLocal<Value> {
        MaybeLocal::empty()
    }

    pub fn GetIsolate(&self) -> *mut Isolate {
        std::ptr::null_mut()
    }

    pub fn GetIsolate_static(handle: &TracedReference<Object>) -> *mut Isolate {
        handle.value::<Object>().GetIsolate()
    }

    pub fn PreviewEntries(&self, is_key_value: *mut bool) -> MaybeLocal<Array> {
        MaybeLocal::empty()
    }

    pub fn New(isolate: *mut Isolate) -> Local<Object> {
        Local::Object(Object {})
    }

    pub fn New_with_properties(
        isolate: *mut Isolate,
        prototype_or_null: Local<Value>,
        names: &mut [Local<Name>],
        values: &mut [Local<Value>],
        length: usize,
    ) -> Local<Object> {
        Local::Object(Object {})
    }

    pub fn Cast(obj: *mut Value) -> *mut Object {
        obj as *mut Object
    }

    pub fn IsCodeLike(&self, isolate: *mut Isolate) -> bool {
        false
    }

    fn Unwrap_internal(
        isolate: *mut Isolate,
        wrapper_obj: internal::Address,
        tag_range: CppHeapPointerTagRange,
    ) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    fn Wrap_internal(
        isolate: *mut Isolate,
        wrapper_obj: internal::Address,
        tag: CppHeapPointerTag,
        wrappable: *mut std::ffi::c_void,
    ) {
    }

    fn new() -> Object {
        Object {}
    }

    fn CheckCast(obj: *mut Value) {}

    fn SlowGetInternalField(&self, index: i32) -> Local<Data> {
        Local::Data(Data {})
    }

    fn SlowGetAlignedPointerFromInternalField(&self, index: i32) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    fn SlowGetAlignedPointerFromInternalField_isolate(
        &self,
        isolate: *mut Isolate,
        index: i32,
    ) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
}

pub struct PropertyCallbackInfo<T> {}

impl<T> PropertyCallbackInfo<T> {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    pub fn is_just(&self) -> bool {
        match self {
            Maybe::Just(_) => true,
            Maybe::Nothing => false,
        }
    }

    pub fn is_nothing(&self) -> bool {
        !self.is_just()
    }

    pub fn Just(value: T) -> Maybe<T> {
        Maybe::Just(value)
    }

    pub fn Nothing() -> Maybe<T> {
        Maybe::Nothing
    }
}

#[derive(Debug, Clone)]
pub enum MaybeLocal<T> {
    Local(T),
    Empty,
}

impl<T> MaybeLocal<T> {
    pub fn Local(value: T) -> MaybeLocal<T> {
        MaybeLocal::Local(value)
    }

    pub fn empty() -> MaybeLocal<T> {
        MaybeLocal::Empty
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Local<T>(T);

impl<T> Local<T> {
    fn new(value: T) -> Self {
        Local(value)
    }
}

// Converted from V8 C++ source files:
// Header: templates-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::ptr::null_mut;
mod heap {
    pub mod heap_write_barrier_inl {}
}
mod objects {
    pub mod dictionary {}
    pub mod objects_inl {}
    pub mod oddball {}
    pub mod shared_function_info {}
    pub mod templates {}
}
mod torque_generated {
    pub mod src {
        pub mod objects {
            pub mod templates_tq_inl {}
        }
    }
}

use crate::objects::templates::*;
use crate::objects::shared_function_info::*;
use crate::objects::objects_inl::*;
use crate::objects::oddball::*;
use crate::objects::dictionary::*;
use v8::internal::Address;

pub mod v8 {
    pub mod internal {
        pub struct Isolate {}
        pub struct HeapObject {}
        pub struct TaggedObject {}
        pub enum CachingMode {}
    }
}

pub mod i {
    pub struct IsolateForSandbox {}
}

pub mod base {
    pub fn IsInRange<T: PartialOrd>(value: T, low: T, high: T) -> bool {
        value >= low && value <= high
    }
}
pub mod internals {
    pub const kFirstJSApiObjectType: i32 = 1;
    pub const kLastJSApiObjectType: i32 = 100;
    pub const kFirstEmbedderJSApiObjectType: i32 = 101;
    pub const kLastEmbedderJSApiObjectType: i32 = 200;
}
pub mod ExternalReference {
    pub const DIRECT_API_CALL: i32 = 1;
    pub fn UnwrapRedirection(result: Address) -> Address {
        result
    }
    pub fn Redirect(value: Address, _type: i32) -> Address {
        value
    }
}
pub const USE_SIMULATOR_BOOL: bool = false;
pub const kNullAddress: Address = Address {};
pub mod compiler {
    pub mod backend {
        pub mod ppc {
            pub mod instruction_selector_ppc {}
        }
    }
}
pub mod codegen {
    pub mod ppc {
        pub mod macro_assembler_ppc {}
    }
}
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
}
impl<T> Tagged<T> {
    pub fn is_null(&self) -> bool {
        true
    }
}
pub struct DirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl FunctionTemplateInfo {
    pub fn relaxed_flag(&self) -> i32 {
        self.flag
    }
    pub fn set_relaxed_flag(&mut self, flags: i32) {
        self.flag = flags;
    }
    pub fn callback(&self, _isolate: i::IsolateForSandbox) -> Address {
        Address {}
    }
    pub fn init_callback(&mut self, _isolate: i::IsolateForSandbox, _initial_value: Address) {}
    pub fn set_callback(&mut self, _isolate: i::IsolateForSandbox, _value: Address) {}
    pub fn init_callback_redirection(&mut self, _isolate: i::IsolateForSandbox) {}
    pub fn remove_callback_redirection(&mut self, _isolate: i::IsolateForSandbox) {}
    pub fn has_callback<IsolateT>(&self, _isolate: *mut IsolateT) -> bool {
        true
    }
    pub fn EnsureFunctionTemplateRareData(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
    ) -> Tagged<FunctionTemplateRareData> {
        Tagged::<FunctionTemplateRareData> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn prototype_template(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, ObjectTemplateInfo>> {
        Tagged::<UnionOf<Undefined, ObjectTemplateInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetPrototypeTemplate(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _prototype_template: DirectHandle<UnionOf<Undefined, ObjectTemplateInfo>>,
    ) {}
    pub fn prototype_provider_template(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, FunctionTemplateInfo>> {
        Tagged::<UnionOf<Undefined, FunctionTemplateInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetPrototypeProviderTemplate(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _prototype_provider_template: DirectHandle<UnionOf<Undefined, FunctionTemplateInfo>>,
    ) {}
    pub fn parent_template(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, FunctionTemplateInfo>> {
        Tagged::<UnionOf<Undefined, FunctionTemplateInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetParentTemplate(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _parent_template: DirectHandle<UnionOf<Undefined, FunctionTemplateInfo>>,
    ) {}
    pub fn named_property_handler(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, InterceptorInfo>> {
        Tagged::<UnionOf<Undefined, InterceptorInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetNamedPropertyHandler(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _named_property_handler: DirectHandle<UnionOf<Undefined, InterceptorInfo>>,
    ) {}
    pub fn indexed_property_handler(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, InterceptorInfo>> {
        Tagged::<UnionOf<Undefined, InterceptorInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetIndexedPropertyHandler(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _indexed_property_handler: DirectHandle<UnionOf<Undefined, InterceptorInfo>>,
    ) {}
    pub fn instance_template(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, ObjectTemplateInfo>> {
        Tagged::<UnionOf<Undefined, ObjectTemplateInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetInstanceTemplate(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _instance_template: DirectHandle<UnionOf<Undefined, ObjectTemplateInfo>>,
    ) {}
    pub fn instance_call_handler(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, FunctionTemplateInfo>> {
        Tagged::<UnionOf<Undefined, FunctionTemplateInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetInstanceCallHandler(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _instance_call_handler: DirectHandle<UnionOf<Undefined, FunctionTemplateInfo>>,
    ) {}
    pub fn access_check_info(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<UnionOf<Undefined, AccessCheckInfo>> {
        Tagged::<UnionOf<Undefined, AccessCheckInfo>> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetAccessCheckInfo(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _access_check_info: DirectHandle<UnionOf<Undefined, AccessCheckInfo>>,
    ) {}
    pub fn c_function_overloads(&self, _cage_base: i32, _kAcquireLoad: i32) -> Tagged<FixedArray> {
        Tagged::<FixedArray> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn SetCFunctionOverloads(
        _isolate: *mut Isolate,
        _function_template_info: DirectHandle<FunctionTemplateInfo>,
        _c_function_overloads: DirectHandle<FixedArray>,
    ) {}
    pub fn GetInstanceType(&self) -> InstanceType {
        InstanceType::kNormalObjectType
    }
    pub fn SetInstanceType(&mut self, _api_instance_type: i32) {}
    pub fn SetAllowedReceiverInstanceTypeRange(&mut self, _api_instance_type_start: i32, _api_instance_type_end: i32) {}
    pub fn instantiated(&self) -> bool {
        true
    }
    pub fn BreakAtEntry(&self, _isolate: *mut Isolate) -> bool {
        false
    }
    pub fn GetParent(&self, _isolate: *mut Isolate) -> Tagged<FunctionTemplateInfo> {
        Tagged::<FunctionTemplateInfo> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn shared_function_info(&self) -> Tagged<Object> {
        Tagged::<Object> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn rare_data(&self, _isolate: *mut Isolate, _kAcquireLoad: i32) -> Tagged<HeapObject> {
        Tagged::<HeapObject> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn flag(&self, _kRelaxedLoad: i32) -> i32 {
        0
    }
    pub fn set_flag(&mut self, _flags: i32, _kRelaxedStore: i32) {}
    pub fn maybe_redirected_callback(&self, _isolate: i::IsolateForSandbox) -> Address {
        Address {}
    }
    pub fn init_maybe_redirected_callback(&mut self, _isolate: i::IsolateForSandbox, _initial_value: Address) {}
    pub fn set_maybe_redirected_callback(&mut self, _isolate: i::IsolateForSandbox, _value: Address) {}
    pub fn instance_type(&self) -> i32 {
        1
    }
    pub fn set_instance_type(&mut self, _instance_type: InstanceType) {}
    pub fn allowed_receiver_instance_type_range_start(&self) -> i32 {
        1
    }
    pub fn set_allowed_receiver_instance_type_range_start(&mut self, _start: InstanceType) {}
    pub fn allowed_receiver_instance_type_range_end(&self) -> i32 {
        1
    }
    pub fn set_allowed_receiver_instance_type_range_end(&mut self, _end: InstanceType) {}
    pub fn IsTemplateFor(&self, _object: Tagged<JSObject>) -> bool {
        true
    }
}

impl ObjectTemplateInfo {
    pub fn GetParent(&self, _isolate: *mut Isolate) -> ObjectTemplateInfo {
        ObjectTemplateInfo {}
    }
    pub fn embedder_field_count(&self) -> i32 {
        0
    }
    pub fn set_embedder_field_count(&mut self, _count: i32) {}
    pub fn immutable_proto(&self) -> bool {
        false
    }
    pub fn set_immutable_proto(&mut self, _immutable: bool) {}
    pub fn code_like(&self) -> bool {
        false
    }
    pub fn set_code_like(&mut self, _is_code_like: bool) {}
    pub fn constructor(&self) -> Tagged<Object> {
        Tagged::<Object> {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn data(&self) -> i32 {
        0
    }
    pub fn set_data(&mut self, _data: i32) {}
}

impl TemplateInfo {
    pub fn TryGetIsolate(&self, isolate: &mut *mut Isolate) -> bool {
        *isolate = null_mut();
        true
    }
    pub fn GetIsolateChecked(&self) -> *mut Isolate {
        null_mut()
    }
    pub fn is_cacheable(&self) -> bool {
        true
    }
    pub fn set_is_cacheable(&mut self, _is_cacheable: bool) {}
    pub fn serial_number(&self) -> u32 {
        0
    }
    pub fn set_serial_number(&mut self, _value: u32) {}
    pub fn EnsureHasSerialNumber(&mut self, _isolate: *mut Isolate) -> u32 {
        0
    }
    pub fn GetHash(&self) -> u32 {
        0
    }
    pub fn template_info_flags(&self) -> i32 {
        0
    }
    pub fn set_template_info_flags(&mut self, _flags: i32) {}
    pub fn ProbeInstantiationsCache(
        _isolate: *mut Isolate,
        _native_context: DirectHandle<NativeContext>,
        _info: DirectHandle<TemplateInfo>,
        _caching_mode: CachingMode,
    ) -> MaybeHandle<Object> {
        MaybeHandle::<Object> {}
    }
    pub fn CacheTemplateInstantiation(
        _isolate: *mut Isolate,
        _native_context: DirectHandle<NativeContext>,
        _info: DirectHandle<TemplateInfo>,
        _caching_mode: CachingMode,
        _object: DirectHandle<Object>,
    ) {}
    pub fn UncacheTemplateInstantiation(
        _isolate: *mut Isolate,
        _native_context: DirectHandle<NativeContext>,
        _info: DirectHandle<TemplateInfo>,
        _caching_mode: CachingMode,
    ) {}
}
pub struct UnionOf<T, U> {
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_u: std::marker::PhantomData<U>,
}
pub struct InterceptorInfo {}
pub struct AccessCheckInfo {}
pub struct FixedArray {}
pub enum InstanceType {
    kNormalObjectType,
}
pub struct JSObject {}
pub struct NativeContext {}
pub struct CachingMode {}
pub struct MaybeHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct HeapObject {}
pub struct Undefined {}
pub struct Object {}
pub const kNoJSApiObjectType: i32 = 0;
pub const JS_API_OBJECT_TYPE: i32 = 1;
pub const LAST_JS_API_OBJECT_TYPE: i32 = 100;
pub const kFastTemplateInstantiationsCacheSize: i32 = 16;
pub const kUninitializedSerialNumber: i32 = 0;
pub const kMaxTemplateInstantiationsCacheSize: i32 = 256;

pub fn AllocateFunctionTemplateRareData(_isolate: *mut Isolate, _function_template_info: DirectHandle<FunctionTemplateInfo>) -> Tagged<FunctionTemplateRareData>{
    Tagged::<FunctionTemplateRareData> {
        _phantom: std::marker::PhantomData,
    }
}
pub struct FunctionTemplateRareData{
    _phantom: std::marker::PhantomData<*const void>,
}
impl FunctionTemplateRareData{
    pub fn set_prototype_template(&mut self, _value : Tagged<UnionOf<Undefined, ObjectTemplateInfo>>) {}
    pub fn set_prototype_provider_template(&mut self, _value : Tagged<UnionOf<Undefined, FunctionTemplateInfo>>) {}
    pub fn set_parent_template(&mut self, _value : Tagged<UnionOf<Undefined, FunctionTemplateInfo>>) {}
    pub fn set_named_property_handler(&mut self, _value : Tagged<UnionOf<Undefined, InterceptorInfo>>) {}
    pub fn set_indexed_property_handler(&mut self, _value : Tagged<UnionOf<Undefined, InterceptorInfo>>) {}
    pub fn set_instance_template(&mut self, _value : Tagged<UnionOf<Undefined, ObjectTemplateInfo>>) {}
    pub fn set_instance_call_handler(&mut self, _value : Tagged<UnionOf<Undefined, FunctionTemplateInfo>>) {}
    pub fn set_access_check_info(&mut self, _value : Tagged<UnionOf<Undefined, AccessCheckInfo>>) {}
    pub fn set_c_function_overloads(&mut self, _value : Tagged<FixedArray>) {}
}
pub struct Isolate{
    _phantom: std::marker::PhantomData<*const void>,
}
impl Isolate{
    pub fn heap(&mut self) -> &mut Heap{
        unsafe {std::mem::transmute(self)}
    }
    pub fn TryGetCurrent() -> *mut Isolate {
        null_mut()
    }
}
pub struct Heap{
    _phantom: std::marker::PhantomData<*const void>,
}
impl Heap{
    pub fn GetNextTemplateSerialNumber(&mut self) -> u32 {
        0
    }
}
pub struct ReadOnlyRoots{
    _phantom: std::marker::PhantomData<*const void>,
}
impl ReadOnlyRoots{
    pub fn the_hole_value(&self) -> i32 {
        0
    }
    pub fn undefined_value(&self) -> Tagged<Undefined> {
        Tagged::<Undefined> {
            _phantom: std::marker::PhantomData,
        }
    }
}
pub fn ComputeUnseededHash(_serial_number : u32) -> u32{
    0
}
pub struct EphemeronHashTable{
    _phantom: std::marker::PhantomData<*const void>,
}
impl EphemeronHashTable{
    pub fn FindEntry(_isolate : *mut Isolate, _roots : ReadOnlyRoots, _info : DirectHandle<TemplateInfo>, _hash : u32) -> InternalIndex{
        InternalIndex{}
    }
    pub fn ValueAt(&self, _entry : InternalIndex) -> Tagged<Object>{
        Tagged::<Object>{
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn Put(_isolate : *mut Isolate, _cache : DirectHandle<EphemeronHashTable>, _info : DirectHandle<TemplateInfo>, _object : DirectHandle<Object>, _hash : u32) -> DirectHandle<EphemeronHashTable> {
        DirectHandle::<EphemeronHashTable>{
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn NumberOfElements(&self) -> i32 {
        0
    }
    pub fn Remove(_isolate : *mut Isolate, _cache : DirectHandle<EphemeronHashTable>, _info : DirectHandle<TemplateInfo>, _was_present : &mut bool, _hash : u32) -> DirectHandle<EphemeronHashTable> {
        DirectHandle::<EphemeronHashTable>{
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct InternalIndex{
    _phantom: std::marker::PhantomData<*const void>,
}
impl InternalIndex{
    pub fn is_found(&self) -> bool {
        true
    }
}
pub mod SKIP_WRITE_BARRIER{
    _phantom: std::marker::PhantomData<*const void>
}
pub fn GetIsolateFromHeapObject<T>(_value : T, _isolate : &mut *mut Isolate) -> bool{
    true
}

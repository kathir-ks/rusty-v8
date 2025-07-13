// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-array.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins_array {
use crate::builtins::builtins_utils_inl::kMaxArrayIndex;
use crate::heap::factory::V8_WARN_UNUSED_RESULT;
use crate::objects::contexts::Context;
use crate::objects::elements_inl::ElementsAccessor;
use crate::objects::elements_kind::ElementsKind;
use crate::objects::elements_kind::ElementsKind::*;
use crate::objects::fixed_array::FixedArray;
use crate::objects::fixed_array_inl::TaggedField;
use crate::objects::js_array_buffer_inl::IsSharedArrayBuffer;
use crate::objects::js_array_inl::JSArray;
use crate::objects::js_collection_inl::kThrowOnError;
use crate::objects::js_shared_array_inl::JSSharedArray;
use crate::objects::lookup::LookupIterator;
use crate::objects::objects_inl::JSObject;
use crate::objects::objects_inl::kReleaseStore;
use crate::objects::property_details::PropertyDetails;
use crate::objects::property_descriptor::ShouldThrow;
use crate::objects::property_descriptor::ShouldThrow::*;
use crate::objects::property_descriptor::*;
use crate::objects::smi::Smi;
use crate::runtime::runtime_classes::PrototypeIterator;
use crate::V8;
use std::cmp;
use std::cmp::min;
use std::convert::TryInto;
use std::mem::MaybeUninit;

// mod builtin;
// mod isolate;
pub struct Isolate {
    pub array_function: *mut Object,
    pub context: *mut Context,
}
pub struct Object {}
pub struct JSReceiver {}
pub struct UnionOf<A, B> {
    a: A,
    b: B,
}
pub struct NumberDictionary {}
pub struct FixedDoubleArray {}
pub struct BuiltinArguments {}
pub struct String {}
pub struct Map {}
pub struct FixedArrayBase {}
pub struct GlobalHandles {}
pub struct DirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct IndirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct HandleScope {}
pub struct Number {}
pub struct HeapObject {}
pub struct PropertyKey {}
pub struct Factory {}
pub struct ReadOnlyRoots {}
pub struct LanguageMode {}
pub struct JSTypedArray {}
pub struct JSPrimitiveWrapper {}

impl IndirectHandle<UnionOf<JSReceiver, FixedArray, NumberDictionary>> {
    pub fn map(&self, _isolate: &Isolate) -> Map {
        Map {}
    }
}
impl String {
    pub fn length(&self) -> i32 {
        0
    }
}
impl PropertyKey {
    pub fn new(_isolate: &Isolate, _k: f64) -> Self {
        PropertyKey {}
    }
}

impl<T> DirectHandle<T> {
    pub fn location(&self) -> *mut Self {
        std::ptr::null_mut()
    }
}

impl BuiltinArguments {
    pub fn at_or_undefined(&self, _isolate: &Isolate, _i: i32) -> DirectHandle<Object> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn at(&self, _i: i32) -> DirectHandle<Object> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn receiver(&self) -> DirectHandle<Object> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn length(&self) -> i32 {
        0
    }
}

impl HeapObject {
    pub fn map(&self) -> Map {
        Map {}
    }
}

impl JSArray {
    pub fn map(&self) -> Map {
        Map {}
    }
    pub fn length(&self) -> DirectHandle<Object> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn elements(&self) -> *mut Object {
        std::ptr::null_mut()
    }
}

impl Isolate {
    pub fn context(&self) -> *mut Context {
        self.context
    }
    pub fn native_context(&self) -> &Context {
        unsafe { &*self.context }
    }
    pub fn array_function(&self) -> *mut Object {
        self.array_function
    }
    pub fn factory(&self) -> Factory {
        Factory {}
    }
    pub fn Throw(&mut self, e: Object) {}
}

impl Factory {
    pub fn NewNumberFromInt(&self, _arg: i32) -> DirectHandle<Number> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn length_string(&self) -> DirectHandle<String> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn NewNumber(&self, _arg: f64) -> DirectHandle<Number> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn NewNumberFromUint(&self, _arg: u32) -> DirectHandle<Number> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn NewFixedArrayWithHoles(&self, _estimate_result_length: u32) -> DirectHandle<FixedArray> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn NewJSArrayWithElements(
        &self,
        _storage: DirectHandle<FixedArrayBase>,
        _kind: ElementsKind,
        _j: i32,
    ) -> DirectHandle<JSArray> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn is_concat_spreadable_symbol(&self) -> DirectHandle<Symbol> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Object {
    pub fn TypeOf(_isolate: &Isolate, _object: DirectHandle<JSArray>) -> Object {
        Object {}
    }
    pub fn ToObject(
        _isolate: &Isolate,
        _object: DirectHandle<Object>,
        _message: &str,
    ) -> Result<DirectHandle<JSReceiver>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn GetLengthFromArrayLike(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn NumberValue(_object: Object) -> f64 {
        0.0
    }
    pub fn BooleanValue(_value: Object, _isolate: &Isolate) -> bool {
        false
    }
    pub fn IsArray(_receiver: DirectHandle<JSReceiver>) -> bool {
        false
    }
    pub fn SetProperty(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _key: DirectHandle<String>,
        _value: DirectHandle<Number>,
        _store_origin: StoreOrigin,
        _should_throw: crate::V8::Maybe<ShouldThrow>,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn SetElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _length: f64,
        _element: DirectHandle<Object>,
        _throw_on_error: ShouldThrow,
    ) -> Result<(), String> {
        Ok(())
    }
    pub fn GetPropertyOrElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _index: PropertyKey,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn ArraySpeciesConstructor(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSAny>,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn ToUint32(_val: Object, _length: &mut u32) -> bool {
        false
    }
    pub fn IntegerValue(_isolate: &Isolate, _index: DirectHandle<Object>) -> Result<f64, String> {
        Ok(0.0)
    }
    pub fn OptimalElementsKind(_value: Object, _isolate: &Isolate) -> ElementsKind {
        PACKED_SMI_ELEMENTS
    }
    pub fn SetPropertyOrElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _key: PropertyKey,
        _value: DirectHandle<Object>,
        _just: crate::V8::Maybe<ShouldThrow>,
        _k_maybe_keyed: StoreOrigin,
    ) -> Result<(), String> {
        Ok(())
    }
    pub fn GetElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _index: i32,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
}

impl NumberDictionary {
    pub fn Set(
        _isolate: &Isolate,
        _dict: DirectHandle<NumberDictionary>,
        _index: u32,
        _elm: DirectHandle<Object>,
        _not_a_prototype_holder: DirectHandle<JSObject>,
    ) -> DirectHandle<NumberDictionary> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn New(
        _isolate: &Isolate,
        _current_storage_length: i32,
    ) -> DirectHandle<NumberDictionary> {
        DirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl FixedArray {
    pub fn length(&self) -> i32 {
        0
    }
    pub fn get(&self, _j: i32) -> *mut Object {
        std::ptr::null_mut()
    }
}

impl GlobalHandles {
    pub fn Create(
        _storage: UnionOf<JSReceiver, FixedArray, NumberDictionary>,
    ) -> IndirectHandle<UnionOf<JSReceiver, FixedArray, NumberDictionary>> {
        IndirectHandle {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn Destroy(_arg: *mut IndirectHandle<UnionOf<JSReceiver, FixedArray, NumberDictionary>>) {}
}
impl Context {
    pub fn get(&self, _index: i32) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayMapIndex(_target_kind: ElementsKind) -> i32 {
        0
    }
    pub fn GetInitialJSArrayMap(&self, _origin_kind: ElementsKind) -> Map {
        Map {}
    }
}

impl JSArray {
    pub fn HasReadOnlyLength(_array: DirectHandle<JSArray>) -> bool {
        false
    }
    pub fn HasArrayPrototype(&self, _isolate: &Isolate) -> bool {
        false
    }
    pub fn GetElementsKind(&self) -> ElementsKind {
        PACKED_SMI_ELEMENTS
    }
    pub fn SetLength(
        _array: DirectHandle<JSArray>,
        _length: u32,
    ) -> crate::V8::Maybe<bool> {
        crate::V8::Maybe { value: true }
    }
}

impl JSReceiver {
    pub fn HasElement(_isolate: &Isolate, _receiver: DirectHandle<JSReceiver>, _i: i32) -> crate::V8::Maybe<bool> {
        crate::V8::Maybe { value: true }
    }
    pub fn CreateDataProperty(
        _isolate: &Isolate,
        _dict: DirectHandle<JSReceiver>,
        _property_key: PropertyKey,
        _element: DirectHandle<Object>,
        _just: crate::V8::Maybe<ShouldThrow>,
    ) -> Result<(), String> {
        Ok(())
    }
    pub fn GetElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _j: i32,
    ) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn DeletePropertyOrElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _index: PropertyKey,
        _strict: LanguageMode,
    ) -> crate::V8::Maybe<()> {
        crate::V8::Maybe { value: () }
    }
    pub fn HasPropertyOrElement(
        _isolate: &Isolate,
        _receiver: DirectHandle<JSReceiver>,
        _from: PropertyKey,
    ) -> Result<bool, String> {
        Ok(false)
    }
}

pub struct AbortReason {}
pub struct Condition {}
impl JSReceiver {
    pub fn Check(&mut self, _cc: Condition, _reason: AbortReason) {}
}

impl ReadOnlyRoots {
    pub fn exception(&self) -> Object {
        Object {}
    }
    pub fn undefined_value(&self) -> Object {
        Object {}
    }
    pub fn the_hole_value(&self) -> Object {
        Object {}
    }
}

pub struct NewTypeError {}
impl Isolate {
    pub fn NewTypeError(&mut self, message: NewTypeError) -> Object {
        Object {}
    }
}

impl DirectHandle<JSAny> {
    pub fn HasArrayPrototype(&self, _isolate: &Isolate) -> bool {
        false
    }
}

pub struct JSAny {}

fn IsJSArrayFastElementMovingAllowed(_isolate: &Isolate, _receiver: JSArray) -> bool {
    true
}

fn HasSimpleElements(_current: &JSObject) -> bool {
    true
}

fn HasOnlySimpleReceiverElements(_isolate: &Isolate, _receiver: &JSObject) -> bool {
    true
}

fn HasOnlySimpleElements(_isolate: &Isolate, _receiver: &JSReceiver) -> bool {
    true
}

fn MatchArrayElementsKindToArguments(
    _isolate: &Isolate,
    _array: DirectHandle<JSArray>,
    _args: *mut BuiltinArguments,
    _first_arg_index: i32,
    _num_arguments: i32,
) {
}
fn IsTheHole(_object: *mut Object, _isolate: &Isolate) -> bool {
    false
}

impl FixedDoubleArray {
    pub fn is_the_hole(&self, _i: i32) -> bool {
        false
    }
    pub fn get_scalar(&self, _j: i32) -> f64 {
        0.0
    }
}

impl ElementsAccessor {
    pub fn Concat(
        _isolate: &Isolate,
        _args: *mut BuiltinArguments,
        _n_arguments: i32,
        _result_len: i32,
    ) -> Result<DirectHandle<JSArray>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
}

enum StoreOrigin {
    kMaybeKeyed,
}

struct MapUpdater {}

impl FixedArray {
    pub fn FillWithHoles(&self, _start: i32, _estimate_result_length: u32) {}
}
pub struct Symbol {}

impl ElementsAccessor {
    pub fn Fill(
        &mut self,
        _array: DirectHandle<JSArray>,
        _value: DirectHandle<Object>,
        _start: u32,
        _end: u32,
    ) -> Result<(), String> {
        Ok(())
    }
    pub fn SetLength(&mut self, _array: DirectHandle<JSArray>, _end: u32) -> crate::V8::Maybe<bool> {
        crate::V8::Maybe { value: true }
    }
    pub fn Push(
        &mut self,
        _array: DirectHandle<JSArray>,
        _args: *mut BuiltinArguments,
        _to_add: i32,
    ) -> Result<u32, String> {
        Ok(0)
    }
    pub fn Pop(&mut self, _array: DirectHandle<JSArray>) -> Result<DirectHandle<Object>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
    pub fn Shift(&mut self, _array: DirectHandle<JSArray>) -> Result<Object, String> {
        Ok(Object {})
    }
    pub fn HasElement(&self, _raw_object: Object, _i: u32) -> bool {
        false
    }
    pub fn Unshift(
        &mut self,
        _array: DirectHandle<JSArray>,
        _args: *mut BuiltinArguments,
        _to_add: i32,
    ) -> Result<u32, String> {
        Ok(0)
    }
}

impl JSObject {
    pub fn HasFastElements(&self) -> bool {
        false
    }
    pub fn PrototypeHasNoElements(_isolate: &Isolate, _receiver: DirectHandle<JSArray>) -> bool {
        true
    }
    pub fn TransitionElementsKind(_array: DirectHandle<JSArray>, _target_kind: ElementsKind) {}
    pub fn GetElementsAccessor(&self) -> *mut ElementsAccessor {
        std::ptr::null_mut()
    }
    pub fn SetMapAndElements(_array: DirectHandle<JSArray>, _new_map: DirectHandle<Map>, _elements: DirectHandle<FixedArrayBase>) {}
     pub fn UpdateAllocationSite(_array: DirectHandle<JSArray>, _target_kind: ElementsKind) {}

}
pub mod crate_V8 {
    pub struct Maybe<T> {
        pub value: T,
    }
}
impl V8 {
    pub fn initialize(_isolate: &mut Isolate) {}
    pub fn ArrayConcat(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayPush(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayPop(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayShift(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayUnshift(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
    pub fn ArrayPrototypeFill(_isolate: &mut Isolate, args: *mut BuiltinArguments) -> *mut Object {
        std::ptr::null_mut()
    }
}
impl DirectHandle<JSReceiver> {
    pub fn map(&self, _isolate: &Isolate) -> Map {
        Map {}
    }
}

fn GetMoreGeneralElementsKind(_origin_kind: ElementsKind, _target_kind: ElementsKind) -> ElementsKind {
    PACKED_SMI_ELEMENTS
}
struct Protectors {}
impl Protectors {
    pub fn IsIsConcatSpreadableLookupChainIntact(_isolate: &Isolate) -> bool {
        true
    }
    pub fn IsArraySpeciesLookupChainIntact(_isolate: &Isolate) -> bool {
        true
    }
}
fn IsDictionaryElementsKind(_kind: ElementsKind) -> bool {
    false
}
fn IsMoreGeneralElementsKindTransition(_origin_kind: ElementsKind, _target_kind: ElementsKind) -> bool {
    false
}

fn IsConstructor(_species: *mut Object) -> bool {
    false
}
fn IsHeapObject(_arg: Object) -> bool {
    false
}
fn IsNumber(_arg: Object) -> bool {
    false
}
fn IsSmi(_obj: Object) -> bool {
    false
}
pub enum MessageTemplate {
    kInvalidArrayLength,
    kStrictReadOnlyProperty,
    kPushPastSafeLength,
}
pub struct DisableGCMole {}

pub struct DisallowJavascriptExecution {}

fn DoubleToUint32IfEqualToSelf(_double_value: f64, _result: &mut u32) -> bool {
    true
}

fn TryFastArrayFill(
    _isolate: &Isolate,
    _args: *mut BuiltinArguments,
    _receiver: DirectHandle<JSReceiver>,
    _value: DirectHandle<Object>,
    _start_index: f64,
    _end_index: f64,
) -> bool {
    false
}

fn GenericArrayFill(
    _isolate: &Isolate,
    _receiver: DirectHandle<JSReceiver>,
    _value: DirectHandle<Object>,
    _start: f64,
    _end: f64,
) -> *mut Object {
    std::ptr::null_mut()
}

fn GetRelativeIndex(
    _isolate: &Isolate,
    _length: f64,
    _index: DirectHandle<Object>,
    _init_if_undefined: f64,
) -> Result<f64, String> {
    Ok(0.0)
}
enum class TaskPriority {
    kHigh,
    kUserBlocking,
};

mod v8_flags {
    pub static mut verify_heap: bool = false;
}

fn IsCustomElementsReceiverMap(_map: Map) -> bool {
    false
}
fn IsJSTypedArray(_object: Object, _isolate: &Isolate) -> bool {
    false
}
fn GetPackedElementsKind(_elements_kind: ElementsKind) -> ElementsKind {
    PACKED_SMI_ELEMENTS
}
fn IsAnyNonextensibleElementsKind(_array_kind: ElementsKind) -> bool {
    false
}
impl HeapObject {
    pub fn elements(&self) -> *mut Object {
        std::ptr::null_mut()
    }
}

impl DirectHandle<FixedArrayBase> {
    pub fn length(&self) -> i32 {
        0
    }
}

mod Execution {
    use super::*;
    pub fn New(
        _isolate: &Isolate,
        _species: DirectHandle<Object>,
        _species1: DirectHandle<Object>,
        _arg: std::vec::Vec<&DirectHandle<Object>>,
    ) -> Result<DirectHandle<JSReceiver>, String> {
        Ok(DirectHandle {
            _phantom: std::marker::PhantomData,
        })
    }
}

impl Object {
    pub fn ToNumber(_val: DirectHandle<Object>) -> Result<Number, String> {
        Ok(Number {})
    }
}

} // mod builtins_array

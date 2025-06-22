// NOTE: This is a partial conversion, as a complete conversion would be extremely large.
//       This focuses on translating key data structures and methods, and omits
//       many dependencies and complex features.  Error handling is simplified.
//       Certain C++ features (like preprocessor macros) are approximated.

// Crates
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// Constants - Approximate C++ Macros
const ABSENT: i32 = 0; // Placeholder for PropertyAttributes::ABSENT
const DONT_ENUM: i32 = 1; // Placeholder for PropertyAttributes::DONT_ENUM
const DONT_DELETE: i32 = 2; // Placeholder for PropertyAttributes::DONT_DELETE
const READ_ONLY: i32 = 4; // Placeholder for PropertyAttributes::READ_ONLY

const kThrowOnError: bool = true;
const kDontThrow: bool = false;

// Enums - Approximate C++ Enums
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LookupIteratorState {
    TRANSITION,
    JSPROXY,
    WASM_OBJECT,
    INTERCEPTOR,
    ACCESS_CHECK,
    TYPED_ARRAY_INDEX_NOT_FOUND,
    ACCESSOR,
    DATA,
    NOT_FOUND,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PropertyKind {
    kData,
    kAccessor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PropertyLocation {
    kField,
    kDescriptor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ShouldThrow {
    ThrowOnError,
    DontThrow,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum InterceptorResult {
    kFalse,
    kTrue,
    kNotIntercepted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PropertiesEnumerationMode {
    kEnumerationOrder,
    kPropertyAdditionOrder,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum IntegrityLevel {
    SEALED,
    FROZEN,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ToPrimitiveHint {
    kNumber,
    kString,
    kDefault,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OrdinaryToPrimitiveHint {
    kNumber,
    kString,
}

type PropertyAttributes = i32; // Placeholder

// Structs - Approximate C++ Classes
struct Isolate {
    // Placeholder for Isolate state
}

impl Isolate {
    fn factory(&self) -> Factory {
        Factory {} // Placeholder
    }

    fn has_exception(&self) -> bool {
        false // Placeholder
    }

    fn context(&self) -> NativeContext {
        NativeContext{} // Placeholder
    }

    fn ReportFailedAccessCheck<T>(&self, _obj: &JSObject) -> Result<T, String> {
        Err("Access Check Failed".to_string())
    }
}

struct Factory {}

impl Factory {
    fn undefined_value(&self) -> Object {
        Object::Undefined
    }

    fn null_value(&self) -> Object {
        Object::Null
    }

    fn NewTypeError(&self, _message: MessageTemplate, _args: String, _obj: &JSReceiver) -> String {
        "TypeError".to_string() // Simplified
    }

    fn constructor_string(&self) -> String {
        "constructor".to_string()
    }

    fn to_string_tag_symbol(&self) -> String {
        "Symbol.toStringTag".to_string()
    }

    fn valueOf_string(&self) -> String {
        "valueOf".to_string()
    }

    fn toString_string(&self) -> String {
        "toString".to_string()
    }

    fn ToPrimitiveHintString(&self, _hint: ToPrimitiveHint) -> String {
        "hint".to_string()
    }

    fn NewRangeError(&self, _message: MessageTemplate) -> String {
        "RangeError".to_string() // Simplified
    }

    fn NewFixedArray(&self, _len: usize) -> FixedArray {
        FixedArray {} // Simplified
    }

    fn NewJSArrayWithElements(&self, _elements: FixedArray, _kind: i32, _len: usize) -> Object {
        Object::JSArray{} // Simplified
    }
}

struct ReadOnlyRoots {}
impl ReadOnlyRoots {
  fn undefined_value(&self) -> Object {
    Object::Undefined
  }
}

fn GetReadOnlyRoots() -> ReadOnlyRoots {
    ReadOnlyRoots {}
}

struct NativeContext {}
impl NativeContext {
  fn is_null(&self) -> bool {
    false
  }
}

struct JSReceiver {
    properties_or_hash: Object,
    map: Map,
}

impl JSReceiver {
    fn GetIsolate(&self) -> &Isolate {
        unimplemented!() // Placeholder
    }

    fn map(&self) -> &Map {
        &self.map
    }

    fn raw_properties_or_hash(&self) -> &Object {
        &self.properties_or_hash
    }

    fn set_raw_properties_or_hash(&mut self, new_properties: Object, _store_order: i32) {
      self.properties_or_hash = new_properties;
    }

    fn GetCreationContext(&self, _isolate: &Isolate) -> Result<NativeContext, String> {
      Ok(NativeContext {})
    }
}

struct JSObject {
    header: JSReceiver,
    elements: Object,
}

impl JSObject {
    fn GetIsolate(&self) -> &Isolate {
        unimplemented!() // Placeholder
    }

    fn elements(&self) -> &Object {
      &self.elements
    }

    fn set_elements(&mut self, _elements: Object) {
      //  self.elements = elements;
    }

    fn HasSloppyArgumentsElements(&self) -> bool {
      false
    }

    fn HasFastProperties(&self) -> bool {
      true
    }

    fn map(&self) -> &Map {
        &self.header.map()
    }
}

struct JSArray {}
impl JSArray {
  fn DefineOwnProperty(_isolate: &Isolate, _array: &JSArray, _key: &Object, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }
}

struct JSProxy {}
impl JSProxy {
  fn DefineOwnProperty(_isolate: &Isolate, _proxy: &JSProxy, _key: &Object, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }

  fn SetPrivateSymbol(_isolate: &Isolate, _proxy: &JSProxy, _key: &String, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }
}

struct JSTypedArray {}
impl JSTypedArray {
  fn DefineOwnProperty(_isolate: &Isolate, _typed_array: &JSTypedArray, _key: &Object, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }
}

struct JSModuleNamespace {}
impl JSModuleNamespace {
  fn DefineOwnProperty(_isolate: &Isolate, _module_namespace: &JSModuleNamespace, _key: &Object, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }
  fn GetPropertyAttributes(_it: &LookupIterator) -> Result<PropertyAttributes, String>{
    Ok(0)
  }
}

struct AlwaysSharedSpaceJSObject {}
impl AlwaysSharedSpaceJSObject {
  fn DefineOwnProperty(_isolate: &Isolate, _always_shared_space_js_object: &AlwaysSharedSpaceJSObject, _key: &Object, _desc: &PropertyDescriptor, _should_throw: Option<bool>) -> Result<bool, String> {
    Ok(true)
  }
}

struct JSBoundFunction {}
impl JSBoundFunction {
  fn bound_target_function(&self) -> &JSReceiver {
    unimplemented!()
  }
}

struct JSWrappedFunction {}
impl JSWrappedFunction {
  fn wrapped_target_function(&self) -> &JSReceiver {
    unimplemented!()
  }
}

struct JSFunction {
  native_context: NativeContext,
  shared: SharedFunctionInfo,
}

impl JSFunction {
  fn has_initial_map(&self) -> bool {
    false
  }
  fn initial_map(&self) -> &Map {
    unimplemented!()
  }
  fn shared(&self) -> &SharedFunctionInfo {
    &self.shared
  }
  fn GetIsolate(&self) -> &Isolate {
    unimplemented!()
  }
  fn native_context(&self) -> &NativeContext {
    &self.native_context
  }
  fn GetDerivedMap(_isolate: &Isolate, _constructor: &JSFunction, _new_target: &JSReceiver) -> Result<Map, String> {
    Ok(Map{})
  }
  fn GetHeaderSize(_function_has_prototype_slot: bool) -> usize {
      10 //Placeholder
  }
}

struct SharedFunctionInfo{

}

impl SharedFunctionInfo {
    fn IsApiFunction(&self) -> bool {
        false
    }
    fn api_func_data(&self) -> &APIFunctionData {
        unimplemented!()
    }
}

struct APIFunctionData{
    instance_template: Object,
}
impl APIFunctionData {
    fn GetInstanceTemplate(&self) -> &Object {
        &self.instance_template
    }
}

struct ObjectTemplateInfo{}
impl ObjectTemplateInfo {
    fn code_like(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct PropertyDescriptor {
    value: Option<Object>,
    writable: Option<bool>,
    enumerable: Option<bool>,
    configurable: Option<bool>,
    get: Option<Object>,
    set: Option<Object>,
    name: Option<String>,
}

impl PropertyDescriptor {
    fn is_empty(&self) -> bool {
        self.value.is_none()
            && self.writable.is_none()
            && self.enumerable.is_none()
            && self.configurable.is_none()
            && self.get.is_none()
            && self.set.is_none()
    }

    fn has_writable(&self) -> bool {
        self.writable.is_some()
    }

    fn writable(&self) -> bool {
        self.writable.unwrap_or(false)
    }

    fn set_writable(&mut self, writable: bool) {
        self.writable = Some(writable);
    }

    fn has_enumerable(&self) -> bool {
        self.enumerable.is_some()
    }

    fn enumerable(&self) -> bool {
        self.enumerable.unwrap_or(false)
    }

    fn set_enumerable(&mut self, enumerable: bool) {
        self.enumerable = Some(enumerable);
    }

    fn has_configurable(&self) -> bool {
        self.configurable.is_some()
    }

    fn configurable(&self) -> bool {
        self.configurable.unwrap_or(false)
    }

    fn set_configurable(&mut self, configurable: bool) {
        self.configurable = Some(configurable);
    }

    fn has_value(&self) -> bool {
        self.value.is_some()
    }

    fn value(&self) -> &Object {
        self.value.as_ref().unwrap()
    }

    fn set_value(&mut self, value: Object) {
        self.value = Some(value);
    }

    fn has_get(&self) -> bool {
        self.get.is_some()
    }

    fn get(&self) -> &Object {
        self.get.as_ref().unwrap()
    }

    fn set_get(&mut self, get: Object) {
        self.get = Some(get);
    }

    fn has_set(&self) -> bool {
        self.set.is_some()
    }

    fn set(&self) -> &Object {
        self.set.as_ref().unwrap()
    }

    fn set_set(&mut self, set: Object) {
        self.set = Some(set);
    }

    fn set_name(&mut self, name: String) {
      self.name = Some(name);
    }

    fn name(&self) -> &String {
      self.name.as_ref().unwrap()
    }

    fn ToAttributes(&self) -> i32 {
        0 // Placeholder
    }

    fn IsAccessorDescriptor(_desc: &PropertyDescriptor) -> bool {
        false
    }

    fn IsDataDescriptor(_desc: &PropertyDescriptor) -> bool {
        false
    }

    fn IsGenericDescriptor(_desc: &PropertyDescriptor) -> bool {
        true
    }

    fn ToPropertyDescriptor(_isolate: &Isolate, _attributes: &JSAny, desc: &mut PropertyDescriptor) -> bool {
        desc.set_value(Object::Number(1.0));
        desc.set_writable(true);
        desc.set_enumerable(true);
        desc.set_configurable(true);
        true
    }
}

struct Map {}

impl Map {
  fn is_deprecated(&self) -> bool {
    false
  }

  fn GetConstructor(&self) -> Object {
    Object::JSFunction{}
  }
  fn new_target_is_base(&self) -> bool {
    false
  }
  fn is_prototype_map(&self) -> bool {
    false
  }
  fn OnlyHasSimpleProperties(&self) -> bool {
    false
  }

    fn GetObjectCreateMap(_isolate: &Isolate, _prototype: &JSPrototype) -> Map {
        Map {} // Placeholder
    }
    fn instance_descriptors<'a>(&'a self, _isolate: &Isolate) -> DescriptorArray{
      DescriptorArray{}
    }
    fn NumberOfOwnDescriptors(&self) -> usize {
      0
    }
}

struct DescriptorArray{}
impl DescriptorArray{
    fn GetKey(&self, _index: InternalIndex) -> String {
        "key".to_string()
    }

    fn GetDetails(&self, _index: InternalIndex) -> PropertyDetails {
        PropertyDetails{}
    }
    fn GetStrongValue(&self, _index: InternalIndex) -> Object {
        Object::Undefined
    }
    fn SetValue(&mut self, _descriptors: DescriptorArray) {
      //self = descriptors;
    }
}

struct PropertyDetails {

}
impl PropertyDetails {
    fn IsEnumerable(&self) -> bool {
        true
    }
    fn kind(&self) -> PropertyKind {
        PropertyKind::kData
    }
    fn location(&self) -> PropertyLocation {
        PropertyLocation::kField
    }
    fn representation(&self) -> i32 {
        0
    }
    fn field_index(&self) -> i32 {
        0
    }
}

struct FieldIndex{}
impl FieldIndex{
    fn ForPropertyIndex(_map: &Map, _index: i32, _representation: i32) -> FieldIndex {
        FieldIndex{}
    }
}

struct FixedArray {}

impl FixedArray {
    fn length(&self) -> usize {
        0 // Placeholder
    }

    fn get(&self, _index: usize) -> Object {
        Object::Undefined // Placeholder
    }
    fn set(&mut self, _index: usize, _value: Object) {

    }

    fn RightTrimOrEmpty(_isolate: &Isolate, _array: &FixedArray, _count: usize) -> FixedArray {
        FixedArray{} // Simplified
    }
}

struct LookupIterator<'a> {
    isolate: &'a Isolate,
    state: LookupIteratorState,
}

impl<'a> LookupIterator<'a> {
    fn new(isolate: &'a Isolate, _object: &JSReceiver, _key: &Object, _mode: i32) -> Self {
        LookupIterator {
            isolate,
            state: LookupIteratorState::NOT_FOUND,
        }
    }

    fn isolate(&self) -> &'a Isolate {
        self.isolate
    }

    fn Next(&mut self) {
        self.state = LookupIteratorState::NOT_FOUND; // Simplified
    }

    fn state(&self) -> LookupIteratorState {
        self.state
    }

    fn IsFound(&self) -> bool {
        self.state != LookupIteratorState::NOT_FOUND
    }

    fn GetName(&self) -> String {
        "name".to_string() // Simplified
    }

    fn GetHolder<T>(&self) -> T {
        unimplemented!()
    }

    fn GetPropertyAttributes(&self) -> Result<PropertyAttributes, String> {
        Ok(0) // Placeholder
    }
    fn GetDataValue(&self, _allocation_policy: i32) -> Object {
        Object::Undefined
    }
    fn UpdateProtector(&self) {

    }
    fn property_attributes(&self) -> i32 {
        0
    }
    fn GetAccessors(&self) -> Object {
        Object::Undefined
    }
    fn GetInterceptor(&self) -> InterceptorInfo {
        InterceptorInfo{}
    }

    fn HasAccess(&self) -> bool {
        true
    }

    fn Restart(&mut self) {
        self.state = LookupIteratorState::NOT_FOUND;
    }

    fn IsConfigurable(&self) -> bool {
        false
    }

    fn Delete(&self) {

    }

    fn ExtendingNonExtensible<T>(&self, _target: T) -> bool {
        false
    }

    fn GetStoreTarget<T>(&self) -> T {
        unimplemented!()
    }

    fn GetInterceptorForFailedAccessCheck(&self) -> InterceptorInfo {
      InterceptorInfo{}
    }

    fn array_index(&self) -> i32 {
      0
    }
    fn IsElement(&self, _holder: &JSObject) -> bool {
      false
    }
    fn name(&self) -> &String {
      unimplemented!()
    }

    fn NotFound(&self) {

    }
    fn GetReceiver(&self) -> Object {
      Object::Undefined
    }
}

struct InterceptorInfo {}

// Unions - Approximation via Enums
#[derive(Debug, Clone)]
enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    JSReceiver(Box<JSReceiver>),
    JSObject(Box<JSObject>),
    JSFunction{},
    JSArray{},
    Undefined,
    Null,
    Symbol(String),
}

impl Object {
    fn ToObject(isolate: &Isolate, object: &Object) -> Result<JSReceiver, String> {
        match object {
            Object::Number(_) | Object::String(_) | Object::Boolean(_) => {
                // Primitive to Object conversion (simplified)
                let prototype = JSPrototype {}; // Placeholder
                let map = Map::GetObjectCreateMap(isolate, &prototype);
                let properties_or_hash = Object::Undefined;

                Ok(JSReceiver {
                    properties_or_hash,
                    map,
                })
            },
            Object::JSReceiver(receiver) => {
                Ok(JSReceiver{
                  properties_or_hash: Object::Undefined,
                  map: Map {},
                }) // Placeholder
            },
            _ => Err("Cannot convert to object".to_string()),
        }
    }

    fn ToPropertyKey(isolate: &Isolate, key: &Object) -> Result<Object, String> {
        match key {
            Object::String(_) | Object::Symbol(_) => Ok(key.clone()),
            Object::Number(n) => Ok(Object::String(n.to_string())),
            _ => Err("Cannot convert to property key".to_string()),
        }
    }

    fn SameValue(a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => x == y,
            (Object::String(x), Object::String(y)) => x == y,
            (Object::Boolean(x), Object::Boolean(y)) => x == y,
            (Object::Undefined, Object::Undefined) => true,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }

    fn ConvertReceiver(isolate: &Isolate, receiver: &Object) -> Result<Object, String> {
        match receiver {
            Object::Number(_) | Object::String(_) | Object::Boolean(_) => {
                // Primitive to Object conversion (simplified)
                let prototype = JSPrototype {}; // Placeholder
                let map = Map::GetObjectCreateMap(isolate, &prototype);
                let properties_or_hash = Object::Undefined;
                Ok(Object::JSReceiver(Box::new(JSReceiver {
                    properties_or_hash,
                    map,
                })))
            }
            Object::JSReceiver(_) => Ok(receiver.clone()),
            _ => Err("Cannot convert receiver".to_string()),
        }
    }

    fn GetProperty(it: &LookupIterator) -> Result<Object, String> {
        match it.state() {
            LookupIteratorState::DATA => Ok(Object::Number(42.0)), // Simplified
            _ => Err("Property not found".to_string()),
        }
    }
    fn GetPropertyOrElement(isolate: &Isolate, object: &JSReceiver, name: &String) -> Result<Object,String> {
      Ok(Object::Number(1.0))
    }

    fn CannotCreateProperty(_isolate: &Isolate, _object: &JSAny, _key: &String, _value: &Object, _should_throw: Option<bool>) -> Result<bool, String> {
        Err("Cannot create property".to_string())
    }

    fn TransitionAndWriteDataProperty(it: &LookupIterator, value: &Object, attrs: PropertyAttributes, should_throw: Option<bool>, store_origin: i32) -> Result<bool,String> {
      Ok(true)
    }
}

trait JSAny{

}
impl JSAny for Object {}

struct JSPrototype {}

type Name = String; // Placeholder
type Symbol = String;

struct KeyAccumulator {}
impl KeyAccumulator {
  fn GetKeys(_isolate: &Isolate, _props: &JSReceiver, _mode: i32, _filter: i32) -> Result<FixedArray, String> {
      Ok(FixedArray{})
  }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(n) => write!(f, "Number({})", n),
            Object::String(s) => write!(f, "String({})", s),
            Object::Boolean(b) => write!(f, "Boolean({})", b),
            Object::JSReceiver(_) => write!(f, "JSReceiver"),
            Object::Undefined => write!(f, "Undefined"),
            Object::Null => write!(f, "Null"),
            Object::Symbol(s) => write!(f, "Symbol({})", s),
            Object::JSObject(_) => write!(f, "JSObject"),
            Object::JSFunction{} => write!(f, "JSFunction"),
            Object::JSArray{} => write!(f, "JSArray"),
        }
    }
}

// Functions - Approximate C++ Static Methods
impl JSReceiver {
    fn HasProperty(it: &mut LookupIterator) -> Result<bool, String> {
        loop {
            match it.state() {
                LookupIteratorState::TRANSITION => {
                    unreachable!();
                }
                LookupIteratorState::JSPROXY => {
                    return Ok(true); // Simplified
                }
                LookupIteratorState::WASM_OBJECT => {
                    return Ok(false);
                }
                LookupIteratorState::INTERCEPTOR => {
                    let result = it.GetPropertyAttributes()?;
                    if result != ABSENT {
                        return Ok(true);
                    }
                    continue;
                }
                LookupIteratorState::ACCESS_CHECK => {
                    if it.HasAccess() {
                        continue;
                    }
                    return Ok(true); // Simplified
                }
                LookupIteratorState::TYPED_ARRAY_INDEX_NOT_FOUND => {
                    return Ok(false);
                }
                LookupIteratorState::ACCESSOR | LookupIteratorState::DATA => {
                    return Ok(true);
                }
                LookupIteratorState::NOT_FOUND => {
                    return Ok(false);
                }
            }
        }
    }

    fn OrdinaryDefineOwnProperty(
        isolate: &Isolate,
        object: &JSObject,
        key: &Object,
        desc: &mut PropertyDescriptor,
        should_throw: Option<bool>,
    ) -> Result<bool, String> {
        let mut it = LookupIterator::new(isolate, &object.header, key, 0);

        while it.state() == LookupIteratorState::ACCESS_CHECK {
            if !it.HasAccess() {
                return isolate.ReportFailedAccessCheck(object);
            }
            it.Next();
        }

        // 1. Let current be O.[[GetOwnProperty]](P).
        // 2. ReturnIfAbrupt(current).
        let mut current = PropertyDescriptor {
            value: None,
            writable: None,
            enumerable: None,
            configurable: None,
            get: None,
            set: None,
            name: None,
        };
        let maybe_prop_attrs = it.GetPropertyAttributes();

        // TODO: Check side effects
        it.Restart();

        while it.state() == LookupIteratorState::ACCESS_CHECK {
            it.Next();
        }

        //3.
        let extensible = true;

        ValidateAndApplyPropertyDescriptor(isolate, &mut it, extensible, desc, &mut current, should_throw, None)
    }

    fn GetPropertyAttributes(_it: &LookupIterator) -> Result<PropertyAttributes, String> {
      Ok(0)
    }
}

// Placeholder for the actual implementation of the ValidateAndApply function
fn ValidateAndApplyPropertyDescriptor(
  _isolate: &Isolate,
  _it: &mut LookupIterator,
  _extensible: bool,
  _desc: &mut PropertyDescriptor,
  _current: &mut PropertyDescriptor,
  _should_throw: Option<bool>,
  _property_name: Option<Name>,
) -> Result<bool, String> {
  Ok(true)
}

impl JSReceiver {
    fn DefineOwnProperty(isolate: &Isolate, object: &JSReceiver, key: &Object, desc: &mut PropertyDescriptor, should_throw: Option<bool>) -> Result<bool, String> {
      if let Object::JSArray(_array) = object.properties_or_hash {
          JSArray::DefineOwnProperty(isolate, &JSArray{}, key, desc, should_throw)
      } else if let Object::JSProxy{} = object.properties_or_hash {
          JSProxy::DefineOwnProperty(isolate, &JSProxy{}, key, desc, should_throw)
      } else if let Object::JSObject(js_object) = object.properties_or_hash {
        JSReceiver::OrdinaryDefineOwnProperty(isolate, &js_object, key, desc, should_throw)
      } else {
        Ok(true)
      }
    }
    fn CreateDataProperty(isolate: &Isolate, object: &JSReceiver, key: Object, value: &Object, should_throw: Option<bool>) -> Result<bool,String> {

      if let Object::JSObject(js_object) = object.properties_or_hash {
        JSReceiver::OrdinaryDefineOwnProperty(isolate, &js_object, &key, &mut PropertyDescriptor{}, should_throw)
      } else {
        Ok(true)
      }
    }
}
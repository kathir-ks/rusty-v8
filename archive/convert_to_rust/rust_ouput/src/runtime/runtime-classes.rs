// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-classes.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::{Arc, Mutex, RwLock};

pub struct Isolate {}
pub struct JSFunction {}
pub struct String {}
pub struct Context {}
pub struct JSReceiver {}
pub struct JSAny {}
pub struct JSObject {}
pub struct Name {}
pub struct Object {}
pub struct PropertyKey {}
pub struct SharedFunctionInfo {}
pub struct FixedArray {}
pub struct Map {}
pub struct PropertyArray {}
pub struct DescriptorArray {}
pub struct NumberDictionary {}
pub struct PropertyDictionary {}
pub struct SwissNameDictionary {}
pub struct ClassBoilerplate {}
pub struct AccessorPair {}
pub struct JSPrototype {}
pub struct MessageTemplate {}
pub struct Oddball {}
pub struct Null {}
pub struct InternalIndex {}
pub struct FieldType {}
pub struct PropertyDetails {}
pub struct ReadOnlyRoots {}
pub struct HeapObject {}
pub struct Handle<T> {}
pub struct MaybeHandle<T> {}
pub struct Tagged<T> {}
pub struct Smi {}
pub struct JSAny {}
pub struct Object {}
pub struct PropertyKey {}
pub struct StoreOrigin {}
pub struct LookupIterator {}
pub struct PrototypeIterator {}
pub struct JSAny {}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ReferenceError(String),
    TypeError(String),
}

pub type Result<T, E> = std::result::Result<T, E>;

impl Isolate {
    fn factory(&self) -> Factory {
        Factory {}
    }
    fn initial_object_prototype(&self) -> DirectHandle<JSPrototype> {
        DirectHandle {}
    }
    fn has_exception(&self) -> bool {
        false
    }
    fn native_context(&self) -> DirectHandle<Context> {
        DirectHandle {}
    }
    fn UpdateProtectorsOnSetPrototype(&self, _prototype: DirectHandle<JSObject>, _prototype_parent: DirectHandle<JSPrototype>) {
        
    }

    fn MayAccess(&self, _native_context: DirectHandle<Context>, _home_object: DirectHandle<JSObject>) -> bool {
        true
    }

    fn ReportFailedAccessCheck(&self, _home_object: DirectHandle<JSObject>) -> Result<(), Error> {
        Ok(())
    }

    fn internalize_name(&self, name: &Name) -> DirectHandle<Name> {
        DirectHandle {}
    }
}

pub struct Factory {}

impl Factory {
    fn null_string(&self) -> DirectHandle<String> {
        DirectHandle {}
    }
    fn prototype_string(&self) -> DirectHandle<String> {
        DirectHandle {}
    }
    fn empty_string(&self) -> DirectHandle<String> {
        DirectHandle {}
    }
    fn get_string(&self) -> DirectHandle<String> {
        DirectHandle {}
    }
    fn set_string(&self) -> DirectHandle<String> {
        DirectHandle {}
    }

    fn NewPropertyArray(&self, _count: i32) -> DirectHandle<PropertyArray> {
        DirectHandle {}
    }
    fn NumberToString(&self, _key: &Object) -> DirectHandle<Name> {
        DirectHandle {}
    }

}

impl Object {
    fn NoSideEffectsToString(_isolate: &Isolate, _object: DirectHandle<Object>) -> DirectHandle<String> {
        DirectHandle {}
    }
    fn FitsRepresentation(_value: Tagged<Object>, _representation: i32) -> bool {
        true
    }
    fn OptimalRepresentation(_value: Tagged<Object>, _isolate: &Isolate) -> i32 {
        0
    }

    fn GetProperty(_it: &LookupIterator) -> Result<DirectHandle<Object>, Error> {
        Ok(DirectHandle {})
    }

    fn SetSuperProperty(_it: &LookupIterator, _value: DirectHandle<Object>, _store_origin: StoreOrigin) -> Result<(), Error> {
        Ok(())
    }
}

impl JSObject {
    fn MakePrototypesFast(_constructor_parent: DirectHandle<JSPrototype>, _start_at_receiver: i32, _isolate: &Isolate) {}
    fn set_map(&self, _isolate: &Isolate, _map: Map, _k_release_store: i32) {}
    fn set_elements(&self, _elements: NumberDictionary) {}
    fn set_raw_properties_or_hash(&self, _properties_dictionary: SwissNameDictionary, _k_relaxed_store: i32) {}
    fn SetProperties(&self, _property_array: PropertyArray) {}
}

impl Name {
    fn AsArrayIndex(&self, _element: &mut u32) -> bool {
        false
    }
    fn IsInteresting(&self, _isolate: &Isolate) -> bool {
        false
    }
}

impl JSFunction {
    fn SetName(_method: &DirectHandle<JSFunction>, _name: DirectHandle<Name>, _name_prefix: DirectHandle<String>) -> bool {
        true
    }
    fn set_prototype_or_initial_map(&self, _prototype: Object, _k_release_store: i32) {}
}

impl SharedFunctionInfo {
    fn DebugName(_isolate: &Isolate, _shared: DirectHandle<SharedFunctionInfo>) -> String {
        String {}
    }
}

impl Map {
    fn Create(_isolate: &Isolate, _i: i32) -> DirectHandle<Map> {
        DirectHandle {}
    }
    fn CopyDropDescriptors(_isolate: &Isolate, _map: DirectHandle<Map>) -> DirectHandle<Map> {
        DirectHandle {}
    }
    fn SetPrototype(_isolate: &Isolate, _map: DirectHandle<Map>, _prototype_parent: DirectHandle<JSPrototype>) {}
    fn SetPrototype(_isolate: &Isolate, _map: DirectHandle<Map>, _constructor_parent: DirectHandle<JSPrototype>, prototype_setup_mode: bool) {}
    fn SetConstructor(&self, _constructor: JSFunction) {}
    fn InitializeDescriptors(&self, _isolate: &Isolate, _descriptors: DescriptorArray) {}
    fn set_elements_kind(&self, _dictionary_elements: i32) {}
    fn set_is_prototype_map(&self, _b: bool) {}
    fn set_is_dictionary_map(&self, _b: bool) {}
    fn set_is_migration_target(&self, _b: bool) {}
    fn set_may_have_interesting_properties(&self, _b: bool) {}
    fn set_construction_counter(&self, _k_no_slack_tracking: i32) {}
}

impl DescriptorArray {
    fn Allocate(_isolate: &Isolate, _nof_descriptors: i32, _i: i32) -> DirectHandle<DescriptorArray> {
        DirectHandle {}
    }
    fn GetKey(&self, _i: InternalIndex) -> Name {
        Name {}
    }
    fn GetDetails(&self, _i: InternalIndex) -> PropertyDetails {
        PropertyDetails {}
    }
    fn GetStrongValue(&self, _i: InternalIndex) -> Tagged<Object> {
        Tagged {}
    }
    fn Set(&self, _i: InternalIndex, _name: Name, _field_type: FieldType, _details: PropertyDetails) {}
    fn Set(&self, _i: InternalIndex, _name: Name, _value: Tagged<Object>, _details: PropertyDetails) {}

    fn number_of_descriptors(&self) -> i32 {
        0
    }

}

impl PropertyArray {
    fn set(&self, _field_index: i32, _value: Object) {}
    fn length(&self) -> i32 {
        0
    }
}

impl ClassBoilerplate {
    const kFirstDynamicArgumentIndex: i32 = 0;
    const kConstructorArgumentIndex: i32 = 0;
    const kPrototypeArgumentIndex: i32 = 0;

    fn arguments_count(&self) -> i32 {
        0
    }
    fn instance_computed_properties(&self) -> FixedArray {
        FixedArray {}
    }
    fn instance_elements_template(&self) -> Object {
        Object {}
    }
    fn instance_properties_template(&self) -> Object {
        Object {}
    }
    fn static_elements_template(&self) -> Object {
        Object {}
    }
    fn static_computed_properties(&self) -> FixedArray {
        FixedArray {}
    }
    fn static_properties_template(&self) -> Object {
        Object {}
    }
    fn AddToElementsTemplate(
        _isolate: &Isolate,
        _elements_dictionary: &Handle<NumberDictionary>,
        _element: u32,
        _key_index: i32,
        _value_kind: i32,
        _value: Tagged<Smi>,
    ) {
    }
    fn AddToPropertiesTemplate<Dictionary>(
        _isolate: &Isolate,
        _properties_dictionary: &Handle<Dictionary>,
        _name: Handle<Name>,
        _key_index: i32,
        _value_kind: i32,
        _value: Tagged<Smi>,
    ) {
    }
}

impl AccessorPair {
    fn Copy(_isolate: &Isolate, pair: DirectHandle<AccessorPair>) -> DirectHandle<AccessorPair> {
        pair
    }
    fn getter(&self) -> Tagged<Object> {
        Tagged {}
    }
    fn setter(&self) -> Tagged<Object> {
        Tagged {}
    }
    fn set_getter(&self, _getter: Tagged<Object>) {}
    fn set_setter(&self, _setter: Tagged<Object>) {}
}

impl InternalIndex {
    fn IterateEntries(&self) -> Self {
        Self {}
    }
    fn Range(_nof_descriptors: i32) -> Self {
        Self {}
    }
}

impl NumberDictionary {
    fn ShallowCopy(_isolate: &Isolate, _dictionary_template: DirectHandle<NumberDictionary>) -> DirectHandle<NumberDictionary> {
        DirectHandle {}
    }
    fn NumberOfElements(&self) -> i32 {
        0
    }
}

impl SwissNameDictionary {
    fn ShallowCopy(_isolate: &Isolate, _dictionary_template: DirectHandle<SwissNameDictionary>) -> DirectHandle<SwissNameDictionary> {
        DirectHandle {}
    }
    fn NumberOfElements(&self) -> i32 {
        0
    }
}

impl PropertyDictionary {
    fn ShallowCopy(_isolate: &Isolate, _dictionary_template: DirectHandle<PropertyDictionary>) -> DirectHandle<PropertyDictionary> {
        DirectHandle {}
    }
    fn NumberOfElements(&self) -> i32 {
        0
    }
}

pub struct DirectHandle<T> {}

impl<T> DirectHandle<T> {
    fn is_null(&self) -> bool {
        false
    }
}

pub struct HandleScope {}

impl HandleScope {
    pub fn new(_isolate: &Isolate) -> Self {
        HandleScope {}
    }
}

pub struct RuntimeArguments {}

impl RuntimeArguments {
    fn at<T>(&mut self, _index: i32) -> DirectHandle<T> {
        DirectHandle {}
    }
    fn change_value_scope(_isolate: &Isolate, _args: &mut RuntimeArguments, _k_prototype_argument_index: i32, _prototype: Object) -> RuntimeArguments {
        RuntimeArguments {}
    }

    fn at_object(&mut self, _i: i32) -> &Object {
        &Object {}
    }
}

impl From<i32> for Smi {
    fn from(_i: i32) -> Self {
        Smi {}
    }
}

impl Smi {
    fn FromInt(_i: i32) -> Self {
        Smi {}
    }
    fn ToInt(_smi: Tagged<Object>) -> i32 {
        0
    }

    fn value(&self) -> i32 {
        0
    }
}

impl Tagged<Object> {
    fn IsInteresting(_isolate: &Isolate) -> bool {
        false
    }
}

impl PropertyDetails {
    fn location(&self) -> i32 {
        0
    }
    fn kind(&self) -> i32 {
        0
    }
    fn attributes(&self) -> i32 {
        0
    }
    fn representation(&self) -> i32 {
        0
    }
    fn constness(&self) -> i32 {
        0
    }
    fn pointer(&self) -> bool {
        false
    }

    fn WithRepresentation(&self, _optimal_representation: i32) -> Self {
        Self {}
    }
    fn set_pointer(&self, _pointer: bool) -> Self {
        Self {}
    }
}

impl ReadOnlyRoots {
    fn exception(&self) -> *mut Object {
        std::ptr::null_mut()
    }
    fn empty_slow_element_dictionary(&self) -> DirectHandle<NumberDictionary> {
        DirectHandle {}
    }
    fn empty_descriptor_array(&self) -> DescriptorArray {
        DescriptorArray {}
    }
}

impl LookupIterator {
    fn UpdateProtector(_isolate: &Isolate, _receiver: DirectHandle<JSObject>, _name: DirectHandle<Name>) {}
}

impl PrototypeIterator {
    fn GetCurrent(_iter: PrototypeIterator) -> DirectHandle<Object> {
        DirectHandle {}
    }
}

impl PropertyKey {
    fn GetName(&self, _isolate: &Isolate) -> DirectHandle<Name> {
        DirectHandle {}
    }
}
const kReleaseStore: i32 = 0;
const kRelaxedStore: i32 = 0;
const kStartAtReceiver: i32 = 0;
const DICTIONARY_ELEMENTS: i32 = 0;
const V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL: bool = false;
struct v8_flags {
    log_maps: bool,
}
static mut v8_flags: v8_flags = v8_flags { log_maps: false };

struct LOG {}
impl LOG {
    fn new(_isolate: &Isolate) -> Self {
        Self {}
    }
}
impl LOG{
    fn log(_isolate: &Isolate, _empty_map: MapEvent) {}
}

struct MapEvent<'a> {
    name : &'a str,
    map1 : DirectHandle<Map>,
    map2 : DirectHandle<Map>,
    message : &'a str,
    shared_function_info : String,
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowUnsupportedSuperError(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let error = Err(Error::ReferenceError("UnsupportedSuper".to_string()));
    match error {
        Ok(_) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowConstructorNonCallableError(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let constructor: DirectHandle<JSFunction> = args.at(0);
    let error = Err(Error::TypeError("ConstructorNonCallable".to_string()));
    match error {
        Ok(_) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowStaticPrototypeError(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let _scope = HandleScope::new(isolate);
    let error = Err(Error::TypeError("StaticPrototype".to_string()));
    match error {
        Ok(_) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowSuperAlreadyCalledError(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let _scope = HandleScope::new(isolate);
    let error = Err(Error::ReferenceError("SuperAlreadyCalled".to_string()));
    match error {
        Ok(_) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowSuperNotCalled(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let _scope = HandleScope::new(isolate);
    let error = Err(Error::ReferenceError("SuperNotCalled".to_string()));
    match error {
        Ok(_) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowNotSuperConstructor(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let constructor: DirectHandle<Object> = args.at(0);
    let function: DirectHandle<JSFunction> = args.at(1);

    if constructor.is_null() {
        return std::ptr::null_mut();
    }

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn Runtime_DefineClass(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let class_boilerplate: DirectHandle<ClassBoilerplate> = args.at(0);
    let constructor: DirectHandle<JSFunction> = args.at(1);
    let super_class: DirectHandle<Object> = args.at(2);

    let result = DefineClass(isolate, class_boilerplate, super_class, constructor, *args);

    match result {
        Ok(_prototype) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

fn DefineClass(
    isolate: &mut Isolate,
    class_boilerplate: DirectHandle<ClassBoilerplate>,
    super_class: DirectHandle<Object>,
    constructor: DirectHandle<JSFunction>,
    mut args: RuntimeArguments,
) -> Result<DirectHandle<Object>, Error> {
    let mut prototype_parent: DirectHandle<JSPrototype>;
    let mut constructor_parent: DirectHandle<JSPrototype>;

    if is_the_hole(super_class, isolate) {
        prototype_parent = isolate.initial_object_prototype();
    } else {
        if is_null(super_class, isolate) {
            prototype_parent = isolate.factory().null_value();
        } else if is_constructor(super_class) {
            constructor_parent = direct_handle(cast::<JSPrototype>(&super_class), isolate);

            let maybe_prototype_parent: Result<DirectHandle<Object>, Error> = Runtime::GetObjectProperty(isolate, cast::<JSAny>(&super_class), isolate.factory().prototype_string());
            match maybe_prototype_parent {
                Ok(maybe_prototype_parent) => {
                    match try_cast::<JSPrototype>(&maybe_prototype_parent) {
                        Some(parent) => {
                            prototype_parent = parent;
                        },
                        None => {
                            return Err(Error::TypeError("PrototypeParentNotAnObject".to_string()));
                        }
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            return Err(Error::TypeError("ExtendsValueNotConstructor".to_string()));
        }
    }

    let prototype: DirectHandle<JSObject> = CreateClassPrototype(isolate);
    let mut set_prototype_value_scope = args.change_value_scope(isolate, &mut args, ClassBoilerplate::kPrototypeArgumentIndex, Object {});

    if !InitClassConstructor(isolate, class_boilerplate, constructor_parent, constructor, args)? || !InitClassPrototype(isolate, class_boilerplate, prototype, prototype_parent, constructor, args)? {
        return Err(Error::TypeError("InitClassFailed".to_string()));
    }

    unsafe {
        if v8_flags.log_maps {
            let empty_map: DirectHandle<Map> = DirectHandle {};
            let map_event_1 = MapEvent {
                name: "InitialMap",
                map1: empty_map,
                map2: direct_handle(&Map {}, isolate),
                message: "init class constructor",
                shared_function_info: SharedFunctionInfo::DebugName(isolate, direct_handle(&SharedFunctionInfo {}, isolate)),
            };
            LOG::log(isolate, map_event_1);

            let map_event_2 = MapEvent {
                name: "InitialMap",
                map1: empty_map,
                map2: direct_handle(&Map {}, isolate),
                message: "init class prototype",
                shared_function_info: String {},
            };
            LOG::log(isolate, map_event_2);
        }
    }
    Ok(prototype)
}

fn is_the_hole(_super_class: DirectHandle<Object>, _isolate: &Isolate) -> bool {
    false
}

fn is_null(_super_class: DirectHandle<Object>, _isolate: &Isolate) -> bool {
    false
}

fn is_constructor(_super_class: DirectHandle<Object>) -> bool {
    false
}

fn cast<T>(_super_class: &DirectHandle<Object>) -> T {
    T {}
}

fn try_cast<T>(_maybe_prototype_parent: &DirectHandle<Object>) -> Option<DirectHandle<T>> {
    None
}

struct Runtime {}
impl Runtime {
    fn GetObjectProperty(_isolate: &Isolate, _super_class: JSAny, _prototype_string: DirectHandle<String>) -> Result<DirectHandle<Object>, Error> {
        Ok(DirectHandle {})
    }
}

fn CreateClassPrototype(_isolate: &Isolate) -> DirectHandle<JSObject> {
    let map: DirectHandle<Map> = Map::Create(_isolate, 0);
    DirectHandle {}
}

fn InitClassConstructor(
    isolate: &mut Isolate,
    class_boilerplate: DirectHandle<ClassBoilerplate>,
    constructor_parent: DirectHandle<JSPrototype>,
    constructor: DirectHandle<JSFunction>,
    args: RuntimeArguments,
) -> Result<bool, Error> {
    let map: DirectHandle<Map> = direct_handle(&constructor.map(), isolate);
    let map = Map::CopyDropDescriptors(isolate, map);

    if !constructor_parent.is_null() {
        Map::SetPrototype(isolate, map, constructor_parent, false);
        JSObject::MakePrototypesFast(constructor_parent, kStartAtReceiver, isolate);
    }

    let elements_dictionary_template: DirectHandle<NumberDictionary> = direct_handle(&cast::<NumberDictionary>(&class_boilerplate.static_elements_template()), isolate);
    let computed_properties: DirectHandle<FixedArray> = direct_handle(&class_boilerplate.static_computed_properties(), isolate);

    let properties_template: DirectHandle<Object> = direct_handle(&class_boilerplate.static_properties_template(), isolate);

    if is_descriptor_array(&properties_template) {
        let descriptors_template: DescriptorArray = cast::<DescriptorArray>(&properties_template);

        AddDescriptorsByTemplate(isolate, map, direct_handle(&descriptors_template, isolate), elements_dictionary_template, constructor, args)
    } else {
        map.set_is_dictionary_map(true);
        map.InitializeDescriptors(isolate, ReadOnlyRoots {}.empty_descriptor_array());
        map.set_is_migration_target(false);
        map.set_may_have_interesting_properties(true);
        map.set_construction_counter(0);

        if V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL {
            let properties_dictionary_template: DirectHandle<SwissNameDictionary> = direct_handle(&cast::<SwissNameDictionary>(&properties_template), isolate);
            AddDescriptorsByTemplate(isolate, map, properties_dictionary_template, elements_dictionary_template, computed_properties, constructor, args)
        } else {
            let properties_dictionary_template: DirectHandle<PropertyDictionary> = direct_handle(&cast::<PropertyDictionary>(&properties_template), isolate);
            AddDescriptorsByTemplate(isolate, map, properties_dictionary_template, elements_dictionary_template, computed_properties, constructor, args)
        }
    }
}

fn InitClassPrototype(
    isolate: &mut Isolate,
    class_boilerplate: DirectHandle<ClassBoilerplate>,
    prototype: DirectHandle<JSObject>,
    prototype_parent: DirectHandle<JSPrototype>,
    constructor: DirectHandle<JSFunction>,
    args: RuntimeArguments,
) -> Result<bool, Error> {
    let map: DirectHandle<Map> = direct_handle(&prototype.map(), isolate);
    let map = Map::CopyDropDescriptors(isolate, map);
    map.set_is_prototype_map(true);
    Map::SetPrototype(isolate, map, prototype_parent);
    isolate.UpdateProtectorsOnSetPrototype(prototype, prototype_parent);
    constructor.set_prototype_or_initial_map(Object {}, kReleaseStore);
    map.SetConstructor(JSFunction {});

    let computed_properties: DirectHandle<FixedArray> = direct_handle(&class_boilerplate.instance_computed_properties(), isolate);
    let elements_dictionary_template: DirectHandle<NumberDictionary> = direct_handle(&cast::<NumberDictionary>(&class_boilerplate.instance_elements_template()), isolate);

    let properties_template: Handle<Object> = Handle {};

    if is_descriptor_array(&direct_handle(&class_boilerplate.instance_properties_template(), isolate)) {
        let descriptors_template: DescriptorArray = cast::<DescriptorArray>(&direct_handle(&class_boilerplate.instance_properties_template(), isolate));

        AddDescriptorsByTemplate(isolate, map, direct_handle(&descriptors_template, isolate), elements_dictionary_template, prototype, args)
    } else {
        map.set_is_dictionary_map(true);
        map.set_is_migration_target(false);
        map.set_may_have_interesting_properties(true);
        map.set_construction_counter(0);

        let properties_dictionary_template: DirectHandle<PropertyDictionary> = direct_handle(&cast::<PropertyDictionary>(&class_boilerplate.instance_properties_template()), isolate);
        AddDescriptorsByTemplate(isolate, map, properties_dictionary_template, elements_dictionary_template, computed_properties, prototype, args)
    }
}

fn direct_handle<T>(_t: &T, _isolate: &Isolate) -> DirectHandle<T> {
    DirectHandle {}
}

fn is_descriptor_array(_properties_template: &DirectHandle<Object>) -> bool {
    false
}

fn AddDescriptorsByTemplate<Dictionary>(
    isolate: &mut Isolate,
    map: DirectHandle<Map>,
    properties_dictionary_template: DirectHandle<Dictionary>,
    elements_dictionary_template: DirectHandle<NumberDictionary>,
    computed_properties: DirectHandle<FixedArray>,
    receiver: DirectHandle<JSObject>,
    args: RuntimeArguments,
) -> Result<bool, Error> {
    let computed_properties_length: i32 = 0;

    let properties_dictionary = ShallowCopyDictionaryTemplate(isolate, properties_dictionary_template);
    let elements_dictionary = ShallowCopyDictionaryTemplate(isolate, elements_dictionary_template);

    let i: i32 = 0;
    while i < computed_properties_length {
        //flags = Smi::ToInt(computed_properties->get(i++));
    }

    if !SubstituteValues(isolate, properties_dictionary, args) {
        return Ok(false);
    }

    UpdateProtectors(isolate, receiver, properties_dictionary);

    if elements_dictionary.NumberOfElements() > 0 {
        if !SubstituteValues(isolate, elements_dictionary, args) {
            return Ok(false);
        }
        map.set_elements_kind(0);
    }

    receiver.set_map(isolate, map, 0);
    receiver.set_raw_properties_or_hash(SwissNameDictionary {}, 0);
    if elements_dictionary.NumberOfElements() > 0 {
        receiver.set_elements(NumberDictionary {});
    }
    Ok(true)
}

fn SubstituteValues<Dictionary>(
    isolate: &mut Isolate,
    dictionary: DirectHandle<Dictionary>,
    args: RuntimeArguments,
) -> bool {
    true
}

fn ShallowCopyDictionaryTemplate<Dictionary>(
    isolate: &mut Isolate,
    dictionary_template: DirectHandle<Dictionary>,
) -> DirectHandle<Dictionary> {
    DirectHandle {}
}

fn UpdateProtectors(
    isolate: &mut Isolate,
    receiver: DirectHandle<JSObject>,
    properties_dictionary: DirectHandle<SwissNameDictionary>,
) {
}

fn AddDescriptorsByTemplate(
    isolate: &mut Isolate,
    map: DirectHandle<Map>,
    descriptors_template: DirectHandle<DescriptorArray>,
    elements_dictionary_template: DirectHandle<NumberDictionary>,
    receiver: DirectHandle<JSObject>,
    args: RuntimeArguments,
) -> Result<bool, Error> {
    let nof_descriptors: i32 = descriptors_template.number_of_descriptors();

    let descriptors: DirectHandle<DescriptorArray> = DescriptorArray::Allocate(isolate, nof_descriptors, 0);

    let elements_dictionary: DirectHandle<NumberDictionary> = if elements_dictionary_template == ReadOnlyRoots {}.empty_slow_element_dictionary() {
        elements_dictionary_template
    } else {
        ShallowCopyDictionaryTemplate(isolate, elements_dictionary_template)
    };

    let mut count: i32 = 0;
    let property_array: DirectHandle<PropertyArray> = isolate.factory().NewPropertyArray(count);

    let mut field_index: i32 = 0;

    UpdateProtectors(isolate, receiver, descriptors_template);

    map.InitializeDescriptors(isolate, DescriptorArray {});
    if elements_dictionary.NumberOfElements() > 0 {
        if !SubstituteValues(isolate, elements_dictionary, args) {
            return Ok(false);
        }
        map.set_elements_kind(0);
    }

    receiver.set_map(isolate, map, 0);
    if elements_dictionary.NumberOfElements() > 0 {
        receiver.set_elements(NumberDictionary {});
    }

    if property_array.length() > 0 {
        receiver.SetProperties(PropertyArray {});
    }
    Ok(true)
}

#[no_mangle]
pub extern "C" fn Runtime_LoadFromSuper(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let receiver: DirectHandle<JSAny> = args.at(0);
    let home_object: DirectHandle<JSObject> = args.at(1);
    let name: DirectHandle<Name> = args.at(2);

    let key = PropertyKey {};

    let result = LoadFromSuper(isolate, receiver, home_object, &key);

    match result {
        Ok(_value) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

fn LoadFromSuper(
    isolate: &mut Isolate,
    receiver: DirectHandle<JSAny>,
    home_object: DirectHandle<JSObject>,
    key: &PropertyKey,
) -> Result<DirectHandle<Object>, Error> {
    let holder = GetSuperHolder(isolate, home_object, SuperMode::kLoad, key)?;
    let mut it = LookupIterator {};
    Object::GetProperty(&it)
}

enum SuperMode {
    kLoad,
    kStore,
}

fn GetSuperHolder(
    isolate: &mut Isolate,
    home_object: DirectHandle<JSObject>,
    mode: SuperMode,
    key: &PropertyKey,
) -> Result<DirectHandle<JSReceiver>, Error> {
    if IsAccessCheckNeeded(home_object) && !isolate.MayAccess(isolate.native_context(), home_object) {
        return Err(Error::TypeError("AccessCheckNeeded".to_string()));
    }

    let iter = PrototypeIterator {};
    let proto = PrototypeIterator::GetCurrent(iter);
    if !is_js_receiver(&proto) {
        let message = match mode {
            SuperMode::kLoad => MessageTemplate {},
            SuperMode::kStore => MessageTemplate {},
        };

        return Err(Error::TypeError("proto is not js receiver".to_string()));
    }

    Ok(cast::<JSReceiver>(&proto))
}

fn IsAccessCheckNeeded(_home_object: DirectHandle<JSObject>) -> bool {
    false
}

fn is_js_receiver(_proto: &DirectHandle<Object>) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn Runtime_LoadKeyedFromSuper(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let receiver: DirectHandle<JSAny> = args.at(0);
    let home_object: DirectHandle<JSObject> = args.at(1);
    let key: DirectHandle<Object> = args.at(2);

    let success = true;
    let lookup_key = PropertyKey {};

    let result = LoadFromSuper(isolate, receiver, home_object, &lookup_key);

    match result {
        Ok(_value) => std::ptr::null_mut(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_StoreToSuper(_args: usize) -> *mut Object {
    let isolate = &mut Isolate {};
    let scope = HandleScope::new(isolate);
    let args = &mut RuntimeArguments {};
    let receiver: DirectHandle<JSAny> = args

// Converted from V8 C++ source files:
// Header: value-mirror.h
// Implementation: value-mirror.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Runtime {
        pub struct RemoteObject {
            object_id: Option<String>,
            type_: String,
            subtype: Option<String>,
            description: Option<String>,
            value: Option<Box<dyn Value>>,
            unserializable_value: Option<String>,
            preview: Option<Box<ObjectPreview>>,
        }
        
        impl RemoteObject {
            pub fn create() -> RemoteObjectBuilder {
                RemoteObjectBuilder::new()
            }
        
            pub fn set_type(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }
        
             pub fn setSubtype(&mut self, subtype: String) {
                self.subtype = Some(subtype);
            }

            pub fn setUnserializableValue(&mut self, descriptionValue: String) {
               self.unserializable_value = Some(descriptionValue);
           }
        
           pub fn setPreview(&mut self, previewValue: Box<ObjectPreview>) {
                self.preview = Some(previewValue);
            }
        
            pub fn setClassName(&mut self, className: String) {
                self.description = Some(className);
            }

            pub fn setDescription(&mut self, description: String) -> Self {
                self.description = Some(description);
                self
            }
        
            pub fn setValue(&mut self, value: Box<dyn Value>) {
                self.value = Some(value);
            }
        
            pub fn build(self) -> Self {
                self
            }
        }
        
        pub struct RemoteObjectBuilder {
            object_id: Option<String>,
            type_: String,
            subtype: Option<String>,
            description: Option<String>,
            value: Option<Box<dyn Value>>,
            unserializable_value: Option<String>,
            preview: Option<Box<ObjectPreview>>,
        }
        
        impl RemoteObjectBuilder {
            pub fn new() -> Self {
                RemoteObjectBuilder {
                    object_id: None,
                    type_: String::new(),
                    subtype: None,
                    description: None,
                    value: None,
                    unserializable_value: None,
                    preview: None,
                }
            }
        
            pub fn set_type(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }

             pub fn setSubtype(&mut self, subtype: String) {
                self.subtype = Some(subtype);
            }

            pub fn setUnserializableValue(&mut self, descriptionValue: String) -> Self {
                self.unserializable_value = Some(descriptionValue);
                self
            }
        
           pub fn setPreview(&mut self, previewValue: Box<ObjectPreview>) -> Self{
                self.preview = Some(previewValue);
                self
            }
        
            pub fn setClassName(&mut self, className: String) -> Self {
                self.description = Some(className);
                self
            }

            pub fn setDescription(mut self, description: String) -> Self {
                self.description = Some(description);
                self
            }
        
            pub fn setValue(mut self, value: Box<dyn Value>) -> Self {
                self.value = Some(value);
                self
            }
        
            pub fn build(self) -> RemoteObject {
                RemoteObject {
                    object_id: self.object_id,
                    type_: self.type_,
                    subtype: self.subtype,
                    description: self.description,
                    value: self.value,
                    unserializable_value: self.unserializable_value,
                    preview: self.preview,
                }
            }
        }

        pub trait Value {}
        
        pub struct FundamentalValue {
            value: i32,
        }
        
        impl FundamentalValue {
            pub fn create(value: i32) -> Box<dyn Value> {
                Box::new(FundamentalValue { value })
            }
        }
        
        impl Value for FundamentalValue {}
        
        pub struct StringValue {
            value: String,
        }
        
        impl StringValue {
            pub fn create(value: String) -> Box<dyn Value> {
                Box::new(StringValue { value })
            }
        }
        
        impl Value for StringValue {}
        
        pub struct ListValue {
            values: Vec<Box<dyn Value>>,
        }
        
        impl ListValue {
            pub fn create() -> ListValue {
                ListValue { values: Vec::new() }
            }
        
            pub fn pushValue(&mut self, value: Box<dyn Value>) {
                self.values.push(value);
            }
        }
        
        impl Value for ListValue {}
        
        pub struct DictionaryValue {
            values: std::collections::HashMap<String, Box<dyn Value>>,
        }
        
        impl DictionaryValue {
            pub fn create() -> DictionaryValue {
                DictionaryValue {
                    values: std::collections::HashMap::new(),
                }
            }
        
            pub fn setValue(&mut self, name: String, value: Box<dyn Value>) {
                self.values.insert(name, value);
            }

            pub fn setString(&mut self, name: &str, value: String) {
                self.values.insert(name.to_string(), Box::new(StringValue{value}));
            }

            pub fn setBoolean(&mut self, name: &str, value: bool) {
                self.values.insert(name.to_string(), Box::new(FundamentalValue{value as i32}));
            }
            pub fn setValueStr(&mut self, name: String, value: String) {
                self.values.insert(name, Box::new(StringValue { value }));
            }
        }
        
        impl Value for DictionaryValue {}
        
        pub struct ObjectPreview {
            type_: String,
            subtype: Option<String>,
            description: String,
            overflow: bool,
            properties: Option<Box<Array<PropertyPreview>>>,
            entries: Option<Box<Array<EntryPreview>>>,
        }

        impl ObjectPreview {
            pub fn create() -> ObjectPreviewBuilder {
                ObjectPreviewBuilder::new()
            }
        
            pub fn set_type(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }

             pub fn setSubtype(&mut self, subtype: String) {
                self.subtype = Some(subtype);
            }

            pub fn setDescription(&mut self, description: String) -> Self {
                self.description = description;
                self
            }

            pub fn setOverflow(mut self, overflow: bool) -> Self {
                self.overflow = overflow;
                self
            }

            pub fn setProperties(mut self, properties: Box<Array<PropertyPreview>>) -> Self{
                self.properties = Some(properties);
                self
            }

            pub fn setEntries(mut self, entries: Box<Array<EntryPreview>>) -> Self {
                self.entries = Some(entries);
                self
            }
        
            pub fn build(self) -> Self {
                self
            }

            pub fn getType(&self) -> String {
                self.type_.clone()
            }
            pub fn getDescription(&self, default: String) -> String{
                self.description.clone()
            }
        }

        pub struct ObjectPreviewBuilder {
            type_: String,
            subtype: Option<String>,
            description: String,
            overflow: bool,
            properties: Option<Box<Array<PropertyPreview>>>,
            entries: Option<Box<Array<EntryPreview>>>,
        }

        impl ObjectPreviewBuilder {
            pub fn new() -> Self {
                ObjectPreviewBuilder {
                    type_: String::new(),
                    subtype: None,
                    description: String::new(),
                    overflow: false,
                    properties: None,
                    entries: None,
                }
            }
        
            pub fn set_type(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }

             pub fn setSubtype(&mut self, subtype: String) -> Self {
                self.subtype = Some(subtype);
                self
            }

            pub fn setDescription(mut self, description: String) -> Self {
                self.description = description;
                self
            }

            pub fn setOverflow(mut self, overflow: bool) -> Self {
                self.overflow = overflow;
                self
            }

            pub fn setProperties(mut self, properties: Box<Array<PropertyPreview>>) -> Self {
                self.properties = Some(properties);
                self
            }

            pub fn setEntries(mut self, entries: Box<Array<EntryPreview>>) -> Self{
                self.entries = Some(entries);
                self
            }
        
            pub fn build(self) -> ObjectPreview {
                ObjectPreview {
                    type_: self.type_,
                    subtype: self.subtype,
                    description: self.description,
                    overflow: self.overflow,
                    properties: self.properties,
                    entries: self.entries,
                }
            }
        }
        
        pub struct PropertyPreview {
            name: String,
            type_: String,
            subtype: Option<String>,
            value: String,
            value_preview: Option<Box<ObjectPreview>>,
        }

        impl PropertyPreview {
            pub fn create() -> PropertyPreviewBuilder {
                PropertyPreviewBuilder::new()
            }

            pub fn setName(&mut self, name: String) {
                self.name = name;
            }
        
            pub fn setType(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }

             pub fn setSubtype(&mut self, subtype: String) {
                self.subtype = Some(subtype);
            }
        
            pub fn build(self) -> Self {
                self
            }
            
            pub fn setValuePreview(&mut self, valuePreview: Box<ObjectPreview>) {
                self.value_preview = Some(valuePreview);
            }
        }

        pub struct PropertyPreviewBuilder {
            name: String,
            type_: String,
            subtype: Option<String>,
            value: String,
            value_preview: Option<Box<ObjectPreview>>,
        }

        impl PropertyPreviewBuilder {
            pub fn new() -> Self {
                PropertyPreviewBuilder {
                    name: String::new(),
                    type_: String::new(),
                    subtype: None,
                    value: String::new(),
                    value_preview: None,
                }
            }

            pub fn setName(mut self, name: String) -> Self{
                self.name = name;
                self
            }
        
            pub fn setType(mut self, type_: String) -> Self {
                self.type_ = type_;
                self
            }

             pub fn setSubtype(mut self, subtype: String) -> Self {
                self.subtype = Some(subtype);
                self
            }
            
            pub fn setValue(mut self, value: String) -> Self {
                self.value = value;
                self
            }
        
            pub fn build(self) -> PropertyPreview {
                PropertyPreview {
                    name: self.name,
                    type_: self.type_,
                    subtype: self.subtype,
                    value: self.value,
                    value_preview: self.value_preview,
                }
            }
        }

        pub struct EntryPreview {
            key: Option<Box<ObjectPreview>>,
            value: Box<ObjectPreview>,
        }

        impl EntryPreview {
            pub fn create() -> EntryPreviewBuilder {
                EntryPreviewBuilder::new()
            }

            pub fn setKey(&mut self, key: Box<ObjectPreview>) {
                self.key = Some(key);
            }

            pub fn setValue(&mut self, value: Box<ObjectPreview>) {
                self.value = value;
            }
        
            pub fn build(self) -> Self {
                self
            }
        }

        pub struct EntryPreviewBuilder {
            key: Option<Box<ObjectPreview>>,
            value: Box<ObjectPreview>,
        }

        impl EntryPreviewBuilder {
            pub fn new() -> Self {
                EntryPreviewBuilder {
                    key: None,
                    value: Box::new(ObjectPreview{ type_: String::new(), subtype: None, description: String::new(), overflow: false, properties: None, entries: None }),
                }
            }

            pub fn setKey(mut self, key: Box<ObjectPreview>) -> Self{
                self.key = Some(key);
                self
            }

            pub fn setValue(mut self, value: Box<ObjectPreview>) -> Self {
                self.value = value;
                self
            }
        
            pub fn build(self) -> EntryPreview {
                EntryPreview {
                    key: self.key,
                    value: self.value,
                }
            }
        }

        pub struct Array<T> {
            items: Vec<T>,
        }

        impl<T> Array<T> {
            pub fn new() -> Self {
                Array { items: Vec::new() }
            }
        
            pub fn emplace_back(&mut self, item: T) {
                self.items.push(item);
            }
        }

        pub mod DeepSerializedValue {
            pub struct DictionaryValue {
                values: std::collections::HashMap<String, Box<dyn Value>>,
            }
            
            impl DictionaryValue {
                pub fn create() -> DictionaryValue {
                    DictionaryValue {
                        values: std::collections::HashMap::new(),
                    }
                }
            
                pub fn setValue(&mut self, name: String, value: Box<dyn Value>) {
                    self.values.insert(name, value);
                }

                pub fn setString(&mut self, name: &str, value: String) {
                    self.values.insert(name.to_string(), Box::new(StringValue{value}));
                }

                pub fn setBoolean(&mut self, name: &str, value: bool) {
                    self.values.insert(name.to_string(), Box::new(FundamentalValue{value as i32}));
                }
                pub fn setValueStr(&mut self, name: String, value: String) {
                    self.values.insert(name, Box::new(StringValue { value }));
                }
            }
            
            impl Value for DictionaryValue {}

            pub mod TypeEnum {
                pub const Undefined: &str = "undefined";
                pub const Null: &str = "null";
                pub const String: &str = "string";
                pub const Boolean: &str = "boolean";
                pub const Number: &str = "number";
                pub const Object: &str = "object";
                pub const Symbol: &str = "symbol";
                pub const Bigint: &str = "bigint";
                pub const Function: &str = "function";
            }
        }
    }
}

pub mod v8 {
    pub mod debug {
        pub struct Location {
            line_number: i32,
            column_number: i32,
        }

        impl Location {
            pub fn GetLineNumber(&self) -> i32 {
                self.line_number
            }
            pub fn GetColumnNumber(&self) -> i32 {
                self.column_number
            }
        }
        pub struct Script{}
        impl Script{
            pub fn Id(&self) -> i32 {
                1
            }
        }
        pub struct GeneratorObject{}
        impl GeneratorObject{
            pub fn Cast(_value: &Object) -> &GeneratorObject{
                unsafe {std::mem::transmute(_value)}
            }
            pub fn IsSuspended(&self) -> bool {
                true
            }
            pub fn Function(&self) -> Function{
                Function{}
            }
            pub fn Script(&self) -> MaybeLocal<Script>{
                MaybeLocal{local: Some(Script{})}
            }
            pub fn SuspendedLocation(&self) -> Location {
                Location{line_number: 1, column_number: 1}
            }
        }
        pub struct AccessorPair{}
        impl AccessorPair{
            pub fn IsAccessorPair(_value: &Value) -> bool {
                true
            }
            pub fn getter(&self) -> Value{
                Value{}
            }
            pub fn setter(&self) -> Value{
                Value{}
            }
        }
        pub enum PrivateMemberFilter{
            kPrivateAccessors,
            kPrivateFields
        }
        pub fn GetPrivateMembers(_context: &Context, _object: &Object, _filter: i32, _names: &mut LocalVector<Value>, _values: &mut LocalVector<Value>) -> bool {
            true
        }
        
        pub fn GetBigIntStringValue(_isolate: *mut Isolate, _value: BigInt) -> StringValue {
            StringValue{value: String::new()}
        }
        pub struct PropertyDescriptor{
            pub has_writable : bool,
            pub writable : bool,
            pub has_enumerable : bool,
            pub enumerable: bool,
            pub has_configurable: bool,
            pub configurable: bool,
            pub get : Local<Value>,
            pub set : Local<Value>,
            pub value : Local<Value>
        }
        pub struct PropertyIterator{}
        impl PropertyIterator{
            pub fn Create(_context : &Context, _object : &Object, _nonIndexedPropertiesOnly : bool) -> Option<PropertyIterator>{
                Some(PropertyIterator{})
            }
            pub fn Done(&self) -> bool{
                true
            }
            pub fn is_own(&self) -> bool {
                true
            }
            pub fn name(&self) -> Local<Name> {
                Local{local:Some(Name{})}
            }
            pub fn attributes(&self) -> Maybe<i32>{
                Maybe{has_value : false, value :0}
            }
            pub fn is_native_accessor(&self) -> bool {
                false
            }
            pub fn has_native_getter(&self) -> bool {
                false
            }
            pub fn has_native_setter(&self) -> bool {
                false
            }
            pub fn descriptor(&self) -> Maybe<PropertyDescriptor>{
                Maybe{has_value : false, value : PropertyDescriptor{has_writable: false, writable: false, has_enumerable: false, enumerable: false, has_configurable: false, configurable: false, get: Local{local: None}, set: Local{local: None}, value: Local{local: None}}}
            }
            pub fn is_array_index(&self) -> bool{
                false
            }
            pub fn Advance(&mut self) -> Maybe<bool>{
                Maybe{has_value : false, value : false}
            }

        }
    }
    pub struct StringValue {
        value: String
    }
    pub struct Function{}
    impl Function{
        pub fn ScriptId(&self) -> i32{
            0
        }
        pub fn GetScriptLineNumber(&self) -> i32{
            0
        }
        pub fn GetScriptColumnNumber(&self) -> i32{
            0
        }
        pub fn kLineOffsetNotFound() -> i32{
            0
        }
        pub fn kNoScriptId() -> i32{
            0
        }
        pub fn IsGeneratorFunction(&self) -> bool{
            true
        }
    }
    pub struct Context{}
    impl Context{
        pub fn GetIsolate(&self) -> *mut Isolate{
            0 as *mut Isolate
        }
        pub fn Global(&self) -> Local<Object>{
            Local{local : Some(Object{})}
        }
    }
    pub struct Object{}
    impl Object{
        pub fn New(_isolate: *mut Isolate) -> Local<Object>{
            Local{local: Some(Object{})}
        }
        pub fn Set(&self, _context: &Context, _name: Local<String>, _value: Local<Value>) -> Maybe<bool>{
            Maybe{has_value:true, value: true}
        }
        pub fn GetRealNamedProperty(&self, _context: &Context, _name: Local<String>) -> MaybeLocal<Value>{
            MaybeLocal{local: Some(Value{})}
        }
        pub fn InstanceOf(&self, _context: &Context, _object: Local<Object>) -> Maybe<bool>{
            Maybe{has_value : false, value : false}
        }
        pub fn GetConstructorName(&self) -> String{
            String::new()
        }
        pub fn GetOwnPropertyNames(&self, _context: &Context) -> MaybeLocal<Array>{
            MaybeLocal{local: Some(Array{})}
        }
         pub fn Get( &self, _context: &Context, _name: Local<Name>)-> MaybeLocal<Value>{
            MaybeLocal{local: Some(Value{})}
        }
        pub fn HasRealNamedProperty(&self, _context: &Context, _name: Local<String>) -> Maybe<bool>{
             Maybe{has_value: false, value: true}
        }
        pub fn PreviewEntries(&self, _isKeyValue: &mut bool) -> MaybeLocal<Array>{
            MaybeLocal{local: Some(Array{})}
        }
        pub fn IsArray(&self) -> bool{
            false
        }
        pub fn IsArgumentsObject(&self) -> bool{
            false
        }
        pub fn GetOwnPropertyDescriptor(&self, _context: &Context, _name: Local<String>) -> MaybeLocal<Value>{
             MaybeLocal{local: None}
        }
        pub fn IsStringObject(&self) -> bool{
            false
        }
        pub fn IsArrayBuffer(&self) -> bool{
            false
        }
        pub fn IsSharedArrayBuffer(&self) -> bool{
            false
        }
        pub fn IsPromise(&self) -> bool{
            false
        }
        pub fn IsGeneratorObject(&self) -> bool{
            false
        }
        pub fn IsWeakRef(&self) -> bool{
            false
        }
        pub fn IsRegExp(&self) -> bool{
            false
        }
        pub fn IsFunction(&self) -> bool{
            false
        }
        pub fn IsDate(&self) -> bool{
            false
        }
        pub fn IsNativeError(&self) -> bool{
            false
        }
        pub fn IsMap(&self) -> bool{
            false
        }
        pub fn IsSet(&self) -> bool{
            false
        }
        pub fn IsWeakMap(&self) -> bool{
            false
        }
        pub fn IsWeakSet(&self) -> bool{
            false
        }
        pub fn IsMapIterator(&self) -> bool{
            false
        }
        pub fn IsSetIterator(&self) -> bool{
            false
        }
        pub fn IsTypedArray(&self) -> bool{
            false
        }
        pub fn IsDataView(&self) -> bool{
            false
        }
        pub fn IsWasmMemoryObject(&self) -> bool{
            false
        }
    }
    pub struct Array{}
    impl Array{
        pub fn New(_isolate: *mut Isolate) -> Local<Array>{
            Local{local: Some(Array{})}
        }
        pub fn Length(&self) -> u32{
            0
        }
         pub fn Get( &self, _context: &Context, _index: u32)-> MaybeLocal<Value>{
            MaybeLocal{local: Some(Value{})}
        }
    }
    pub struct Value{}
    impl Value{
        pub fn IsNull(&self) -> bool{
            false
        }
        pub fn IsBoolean(&self) -> bool{
            false
        }
        pub fn IsNumber(&self) -> bool{
            false
        }
        pub fn IsString(&self) -> bool{
            false
        }
        pub fn IsBigInt(&self) -> bool{
            false
        }
        pub fn IsSymbol(&self) -> bool{
            false
        }
        pub fn IsUndefined(&self) -> bool{
            false
        }
        pub fn IsObject(&self) -> bool{
            false
        }
         pub fn As<T>(&self) -> &T{
            unsafe{std::mem::transmute(self)}
        }
    }
    pub struct Primitive{}
    pub struct Boolean{
        value: bool
    }
    impl Boolean{
        pub fn Value(&self) -> bool{
            self.value
        }
    }
    pub struct Number{
        value: f64
    }
    impl Number{
        pub fn Value(&self) -> f64{
            self.value
        }
    }
    pub struct String{}
    impl String{
        pub fn Length(&self) -> i32{
            0
        }
        pub fn StringEquals(&self, _other: Local<String>) -> bool{
            false
        }
    }
    pub struct Symbol{}
    impl Symbol{
        pub fn Description(&self, _isolate: *mut Isolate) -> MaybeLocal<String>{
            MaybeLocal{local:None}
        }
    }
    pub struct BigInt{}
    pub struct Isolate{}
    pub enum PropertyAttribute{
        ReadOnly,
        DontEnum,
        DontDelete
    }
    pub struct TypedArray{}
    impl TypedArray{
        pub fn Length(&self) -> u32{
            0
        }
    }
    pub struct ArrayBuffer{}
    impl ArrayBuffer{
        pub fn ByteLength(&self) -> u32{
            0
        }
    }
    pub struct SharedArrayBuffer{}
    impl SharedArrayBuffer{
        pub fn ByteLength(&self) -> u32{
            0
        }
    }
    pub struct DataView{}
    impl DataView{
        pub fn ByteLength(&self) -> u32{
            0
        }
    }
    pub struct WasmMemoryObject{}
    impl WasmMemoryObject{
        pub fn Buffer(&self) -> &ArrayBuffer{
            &ArrayBuffer{}
        }
    }
    pub struct WasmValueObject{}
    impl WasmValueObject{
        pub fn IsWasmValueObject(_obj: &Object) -> bool {
            false
        }
        pub fn type_(&self) -> Local<String> {
            Local{local:Some(String{})}
        }
    }
    pub struct Proxy{

    }
    impl Proxy{
         pub fn GetTarget(&self) -> Local<Value>{
            Local{local: Some(Value{})}
        }
    }
    pub struct Date{

    }
    pub struct Promise{

    }
    impl Promise{
        pub fn State(&self) -> i32{
            0
        }
        pub fn kRejected() -> i32{
            0
        }
        pub fn MarkAsHandled(&self){

        }
    }
    pub struct Map{
        

    }
    impl Map{
        pub fn Size(&self) -> u32{
            0
        }
    }
    pub struct Set{
        

    }
    impl Set{
        pub fn Size(&self) -> u32{
            0
        }
        pub fn Add(&self, _context: &Context, _name: Local<String>) -> MaybeLocal<Set>{
            MaybeLocal{local: Some(Set{})}
        }
    }

    pub struct LocalVector<T> {
        data: Vec<T>,
    }

    impl<T> LocalVector<T> {
        pub fn new() -> Self {
            LocalVector { data: Vec::new() }
        }
    
        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }
    
        pub fn size(&self) -> usize {
            self.data.len()
        }
    
        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }
        
    }
}

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE");
            };
        }
    }

    pub mod bit_cast {
        pub fn bit_cast<T, U>(value: T) -> U {
            unsafe { std::mem::transmute_copy(&value) }
        }
    }
}

pub mod inspector {
    use std::error::Error;
    use std::fmt;
    use std::rc::Rc;
    use v8::debug::PropertyIterator;
    use v8::debug::AccessorPair;

    pub use crate::inspector::protocol::Runtime::DeepSerializedValue::TypeEnum;
    use crate::inspector::protocol::Runtime::ObjectPreview;
    use crate::inspector::protocol::Runtime::PropertyPreview;
    use crate::inspector::string_16::String16Builder;
    use crate::inspector::{
        protocol::Runtime::{DictionaryValue, ListValue, RemoteObject, Value},
        string_16::String16,
    };
    use crate::{
        protocol::Runtime::EntryPreview,
        string_16::{
            from_utf8, to_protocol_string, to_protocol_string_with_type_check, String16Impl,
        },
        v8::{self,debug::Script, Function, Context, Isolate, Local, Object, StringValue, Array, Value as V8Value, Boolean, Number, Primitive, Symbol, BigInt, self as v8_2, self as v8_3, self as v8_4},
    };
    use crate::inspector::v8_debugger::V8Debugger;
    use std::fmt::{Display, Formatter};
    use self::protocol::Runtime::Array;
    use self::v8::{LocalVector, Name, Proxy, Date, Promise, Map, Set, TypedArray, ArrayBuffer, SharedArrayBuffer, DataView, WasmMemoryObject};
    use self::v8::debug::GeneratorObject;
    use self::v8::debug::PropertyDescriptor;
    use self::protocol::Runtime::PropertyPreviewBuilder;
    use self::protocol::Runtime::EntryPreviewBuilder;
    use self::v8::WasmValueObject;
    use self::v8::Set as V8Set;

    pub mod v8_debugger {
        pub struct V8Debugger{}
        impl V8Debugger{
            pub fn internalProperties(&self, _context: &super::v8::Context, _object: &super::v8::Object) -> super::v8::MaybeLocal<super::v8::Array>{
                super::v8::MaybeLocal{local: None}
            }
        }
    }
    pub mod v8_inspector_impl {
        pub struct V8InspectorImpl{}
        impl V8InspectorImpl{
            pub fn client(&self) -> &super::V8InspectorClient{
                &super::V8InspectorClient{}
            }
            pub fn getContext(&self, _contextId: i32) -> &super::InspectedContext{
                &super::InspectedContext{}
            }
            pub fn debugger(&mut self) -> &mut super::V8Debugger {
                &mut super::V8Debugger{}
            }
        }
    }

    pub mod v8_deep_serializer {
        use super::*;
        pub fn serializeV8Value(_value : &V8Value, _context: &Context, _maxDepth: i32, _additionalParameters: Local<Object>, _duplicateTracker: V8SerializationDuplicateTracker, _result: &mut DictionaryValue) -> Response {
            Response::Success()
        }
    }

    pub trait DeepSerializationResultTrait {
        fn is_success(&self) -> bool;
        fn serialized_value(&self) -> &SerializedValue;
        fn error_message(&self) -> &String16Impl;
    }

    pub struct DeepSerializationResult {
        is_success: bool,
        serialized_value: SerializedValue,
        error_message: String16Impl,
    }

    impl DeepSerializationResultTrait for DeepSerializationResult {
        fn is_success(&self) -> bool {
            self.is_success
        }

        fn serialized_value(&self) -> &SerializedValue {
            &self.serialized_value
        }

        fn error_message(&self) -> &String16Impl {
            &self.error_message
        }
    }

    pub struct SerializedValue {
        value: V8Value,
        type_: String16Impl,
    }

    pub struct V8InspectorClient {}
    impl V8InspectorClient{
        pub fn deepSerialize(&self, _value: Local<Object>, _maxDepth: i32, _additionalParameters: Local<Object>) -> Option<DeepSerializationResult>{
            None
        }
        pub fn descriptionForValueSubtype(&mut self, _context: Local<Context>, _value: Local<V8Value>) -> Option<Box<dyn StringBuffer>> { None }
        pub fn valueSubtype(&self, _object: Local<Object>) -> Option<&str>{
            None
        }
    }

    pub trait StringBuffer {
        fn string(&self) -> &String;
    }

    pub struct StringStringBuffer {
        buffer: String,
    }

    impl StringBuffer for StringStringBuffer {
        fn string(&self) -> &String {
            &self.buffer
        }
    }

    #[derive(Debug, Clone)]
    pub enum InspectorError {
        InternalError(String),
        ServerError(String),
    }

    impl fmt::Display for InspectorError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                InspectorError::InternalError(msg) => {
                    write!(f, "Inspector Internal Error: {}", msg)
                }
                InspectorError::ServerError(msg) => {
                    write!(f, "Inspector Server Error: {}", msg)
                }
            }
        }
    }

    impl Error for InspectorError {}

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

        pub fn Error(error_message: String) -> Self {
            Response {
                success: false,
                error_message: Some(error_message),
            }
        }

        pub

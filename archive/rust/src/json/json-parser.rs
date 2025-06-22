// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/json/json-parser.h
mod json_parser {
    use std::{borrow::Cow, char, cmp, fmt, ops, rc::Rc, str};

    // Re-export types used in the public interface.
    pub use crate::base::small_vector::SmallVector;
    pub use crate::objects::property_descriptor::PropertyDescriptor;

    // Placeholder for types that are too complex to translate
    // or are internal to the V8 engine.  These need to be
    // implemented with appropriate Rust equivalents to complete
    // the port.

    pub struct Isolate;
    pub struct HandleScope<'a>(&'a Isolate);
    pub struct Factory {
        isolate: *const Isolate,
    }
    pub struct Handle<T>(*const T);
    pub struct DirectHandle<T>(*const T);
    pub struct MaybeHandle<T>(Option<Handle<T>>);
    pub struct MaybeDirectHandle<T>(Option<DirectHandle<T>>);

    impl<T> MaybeHandle<T> {
        pub fn to_handle(&self) -> Result<Handle<T>, ()> {
            match self.0 {
                Some(h) => Ok(h),
                None => Err(()),
            }
        }
        pub fn is_nothing(&self) -> bool {
            self.0.is_none()
        }
    }

    impl<T> MaybeDirectHandle<T> {
        pub fn to_handle(&self) -> Result<DirectHandle<T>, ()> {
            match self.0 {
                Some(h) => Ok(h),
                None => Err(()),
            }
        }
    }

    pub struct String;
    pub struct SeqString;
    pub struct SeqOneByteString;
    pub struct SeqTwoByteString;
    pub struct SlicedString;
    pub struct ThinString;
    pub struct ExternalString;
    pub struct SeqExternalString;
    pub struct JSObject;
    pub struct JSArray;
    pub struct Object;
    pub struct FixedArray;
    pub struct FixedArrayBase;
    pub struct NumberDictionary;
    pub struct Map;
    pub struct ByteArray;
    pub struct HeapNumber;
    pub struct DescriptorArray;
    pub struct Script;
    pub struct SharedFunctionInfo;
    pub struct Context;
    pub struct JSReceiver;
    pub struct ObjectTwoHashTable;
    pub struct FieldType;
    pub struct PropertyDetails;
    pub struct PropertyDescriptorArray;
    pub struct FrameSummary;

    pub struct LookupIterator;

    pub struct StackLimitCheck<'a>(&'a Isolate);

    pub enum MessageTemplate {
        kJsonParseUnexpectedEOS,
        kJsonParseUnexpectedTokenNumber,
        kJsonParseUnexpectedTokenString,
        kJsonParseShortString,
        kJsonParseUnexpectedTokenStartStringWithContext,
        kJsonParseUnexpectedTokenSurroundStringWithContext,
        kJsonParseUnexpectedTokenEndStringWithContext,
        kJsonParseUnexpectedNonWhiteSpaceCharacter,
        kJsonParseExpectedPropNameOrRBrace,
        kJsonParseExpectedDoubleQuotedPropertyName,
        kJsonParseExpectedColonAfterPropertyName,
        kJsonParseExpectedCommaOrRBrace,
        kJsonParseExpectedCommaOrRBrack,
        kJsonParseUnterminatedString,
        kJsonParseBadEscapedCharacter,
        kJsonParseBadUnicodeEscape,
        kJsonParseBadControlCharacter,
        kJsonParseNoNumberAfterMinusSign,
        kJsonParseUnterminatedFractionalNumber,
        kJsonParseExponentPartMissingNumber,
        kInvalidRawJsonValue,
    }

    pub struct MessageLocation<'a>(&'a Script, usize, usize);

    pub enum LanguageMode {
        kSloppy,
        kStrict,
    }

    pub enum KeyCollectionMode {
        kOwnOnly,
        kIncludePrototypes,
    }

    pub enum GetKeysConversion {
        kConvertToString,
        kKeepNumbers,
    }

    pub enum ElementsKind {
        PACKED_SMI_ELEMENTS,
        PACKED_ELEMENTS,
        PACKED_DOUBLE_ELEMENTS,
        HOLEY_SMI_ELEMENTS,
        HOLEY_ELEMENTS,
        HOLEY_DOUBLE_ELEMENTS,
        DICTIONARY_ELEMENTS,
    }

    pub enum PropertyKind {
        kData,
        kAccessor,
    }

    pub enum PropertyLocation {
        kField,
        kDescriptor,
    }

    pub enum PropertyConstness {
        kConst,
        kMutable,
    }

    pub enum Attributes {
        NONE,
        READ_ONLY,
        DONT_ENUM,
        DONT_DELETE,
    }

    pub enum INSERT_TRANSITION {}

    pub struct Float64 {
        value: f64,
    }

    impl Float64 {
        pub fn new(value: f64) -> Self {
            Float64 { value }
        }
        pub fn get_bits(&self) -> u64 {
            self.value.to_bits()
        }
    }

    // dummy implementations for types and methods to satisfy the compiler

    impl DirectHandle<Object> {
        pub fn is_null(&self) -> bool {
            self.0.is_null()
        }
    }

    impl DirectHandle<String> {
        pub fn is_null(&self) -> bool {
            self.0.is_null()
        }
    }

    impl Handle<String> {
        pub fn is_null(&self) -> bool {
            self.0.is_null()
        }
        pub fn as_array_index(&self) -> Result<u32, ()> {
            todo!()
        }
    }

    impl HeapNumber {
        pub fn set_value_as_bits(&self, _bits: u64) {
            todo!()
        }
    }

    impl From<f64> for Float64 {
        fn from(value: f64) -> Self {
            Self { value }
        }
    }

    impl<'a> HandleScope<'a> {
        pub fn new(_isolate: &'a Isolate) -> Self {
            HandleScope(&Isolate) // ignore the provided isolate
        }
        pub fn close_and_escape<T>(&self, value: Handle<T>) -> Handle<T> {
            value
        }
        pub fn close_and_escape_direct<T>(&self, value: DirectHandle<T>) -> DirectHandle<T> {
            value
        }
    }

    impl Factory {
        pub fn lookup_single_character_string_from_code(&self, code: u8) -> DirectHandle<Object> {
            // Placeholder implementation
            DirectHandle(&Object)
        }

        pub fn new_sub_string(&self, _source: &Handle<String>, _start: usize, _end: usize) -> DirectHandle<Object> {
            DirectHandle(&Object)
        }

        pub fn new_script(&self, _source: &Handle<String>) -> Handle<Script> {
            Handle(&Script)
        }

        pub fn new_syntax_error(&self, _message: MessageTemplate, _arg: DirectHandle<Object>, _arg2: DirectHandle<Object>, _arg3: DirectHandle<Object>) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn empty_string(&self) -> DirectHandle<String> {
            DirectHandle(&String)
        }

        pub fn true_value(&self) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn false_value(&self) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn null_value(&self) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn new_js_object(&self, _constructor: &Handle<JSObject>) -> DirectHandle<JSObject> {
            DirectHandle(&JSObject)
        }

        pub fn object_literal_map_from_cache(&self, _context: &Handle<Context>, _expected_properties: i32) -> Handle<Map> {
            Handle(&Map)
        }

        pub fn new_js_object_from_map(&self, _map: &Handle<Map>) -> Handle<JSObject> {
            Handle(&JSObject)
        }

        pub fn new_slow_js_object_from_map(&self, _map: &Handle<Map>, _expected_properties: i32) -> Handle<JSObject> {
            Handle(&JSObject)
        }

        pub fn new_fixed_array_with_holes(&self, _length: i32) -> Handle<FixedArray> {
            Handle(&FixedArray)
        }

        pub fn new_number(&self, _value: i32) -> DirectHandle<Object> {
            DirectHandle(&Object)
        }

        pub fn new_number_f64(&self, _value: f64) -> Handle<HeapNumber> {
            Handle(&HeapNumber)
        }

        pub fn new_heap_number(&self, _value: f64) -> Handle<HeapNumber> {
            Handle(&HeapNumber)
        }

        pub fn new_js_array(&self, _kind: ElementsKind, _length: i32, _capacity: i32) -> Handle<JSArray> {
            Handle(&JSArray)
        }

        pub fn new_fixed_array(&self, _length: i32) -> Handle<FixedArray> {
            Handle(&FixedArray)
        }

        pub fn new_byte_array(&self, _size: i32) -> Handle<ByteArray> {
            Handle(&ByteArray)
        }

        pub fn internalize_string<T: AsRef<[u8]>>(&self, _chars: T, _needs_conversion: bool) -> Handle<String> {
            Handle(&String)
        }

        pub fn internalize_string(&self, _string: &Handle<SeqOneByteString>) -> Handle<String> {
            Handle(&String)
        }

        pub fn internalize_sub_string(&self, _string: &Handle<SeqString>, _start: u32, _length: u32, _needs_conversion: bool) -> Handle<String> {
            Handle(&String)
        }

        pub fn uint32_to_string(&self, _index: u32) -> DirectHandle<String> {
            DirectHandle(&String)
        }

        pub fn new_raw_one_byte_string(&self, _length: i32) -> MaybeHandle<SeqOneByteString> {
            MaybeHandle(Some(Handle(&SeqOneByteString)))
        }

        pub fn new_raw_two_byte_string(&self, _length: i32) -> MaybeHandle<SeqTwoByteString> {
            MaybeHandle(Some(Handle(&SeqTwoByteString)))
        }

        pub fn internalize_string_handle(&self, _string: &Handle<SeqOneByteString>) -> Handle<String> {
            Handle(&String)
        }

        pub fn true_string(&self) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn false_string(&self) -> Handle<Object> {
            Handle(&Object)
        }

        pub fn null_string(&self) -> Handle<Object> {
            Handle(&Object)
        }
    }

    impl Isolate {
        pub fn has_exception(&self) -> bool {
            false
        }
        pub fn throw(&self, _exception: Handle<Object>) {}
        pub fn throw_at(&self, _exception: Handle<Object>, _location: &MessageLocation) {}
        pub fn needs_source_positions(&self) -> bool {
            false
        }
        pub fn debug(&self) -> &Debug {
            &Debug
        }
        pub fn heap(&self) -> &Heap {
            &Heap
        }
        pub fn object_function(&self) -> Handle<JSObject> {
            Handle(&JSObject)
        }
        pub fn native_context(&self) -> Handle<Context> {
            Handle(&Context)
        }
    }

    pub struct Debug;

    impl Debug {
        pub fn on_compile_error(&self, _script: &Handle<Script>) {}
    }

    pub struct Heap;

    impl Heap {
        pub fn ensure_sweeping_completed_for_object(&self, _object: &ByteArray) {}
    }

    pub enum Just<T> {
        Value(T),
    }

    pub enum Nothing<T> {
        Value,
    }

    impl<T> Nothing<T> {
        pub fn new() -> Self {
            Nothing::Value
        }
    }

    impl<T> Just<T> {
        pub fn value(self) -> T {
            match self {
                Just::Value(v) => v,
            }
        }
    }

    pub enum HashTableInsertionOrder {
        kNoChainedHashGrow,
        kAllowChainedHashGrow,
    }

    impl HeapNumber {
        pub fn value(&self) -> f64 {
            todo!()
        }
    }

    impl Object {
        pub fn number_value(&self) -> f64 {
            todo!()
        }
    }

    impl String {
        pub fn length(&self) -> usize {
            0
        }
        pub fn is_equal_to<T: AsRef<[u8]>>(&self, _chars: T) -> bool {
            false
        }
        pub fn as_array_index(&self) -> Result<u32, ()> {
            Err(())
        }
    }

    impl JSReceiver {
        pub fn create_data_property(&self, _isolate: &Isolate, _object: DirectHandle<String>, _value: Handle<Object>, _flag: Just<Attributes>) -> Result<(), ()> {
            Ok(())
        }

        pub fn define_own_property(&self, _isolate: &Isolate, _object: &Handle<String>, _desc: &PropertyDescriptor, _flag: Just<Attributes>) -> Result<bool, ()> {
            Ok(true)
        }

        pub fn delete_property_or_element(&self, _isolate: &Isolate, _name: &Handle<String>, _language_mode: LanguageMode) -> Maybe<bool> {
            Maybe::Just(true)
        }

        pub fn delete_property_or_element_direct(&self, _isolate: &Isolate, _name: DirectHandle<String>, _language_mode: LanguageMode) -> Maybe<bool> {
            Maybe::Just(true)
        }

        pub fn add_property(&self, _isolate: &Isolate, _holder: &DirectHandle<JSObject>, _name: &DirectHandle<String>, _value: &DirectHandle<Object>, _attributes: Attributes) {
            todo!()
        }
    }

    pub enum Maybe<T> {
        Just(T),
        Nothing,
    }

    impl<T> Maybe<T> {
        pub fn is_nothing(&self) -> bool {
            match self {
                Maybe::Nothing => true,
                Maybe::Just(_) => false,
            }
        }

        pub fn from_just(self) -> T {
            match self {
                Maybe::Just(v) => v,
                Maybe::Nothing => panic!("called `Maybe::from_just()` on a `Nothing` value"),
            }
        }
    }

    impl Object {
        pub fn get_property_or_element(_isolate: &Isolate, _holder: &DirectHandle<JSReceiver>, _name: &Handle<String>) -> Result<DirectHandle<Object>, ()> {
            Ok(DirectHandle(&Object))
        }

        pub fn get_length_from_array_like(_isolate: &Isolate, _holder: &Handle<JSReceiver>) -> Result<DirectHandle<Object>, ()> {
            Ok(DirectHandle(&Object))
        }
    }

    impl Script {
        pub fn has_line_ends(&self) -> bool {
            false
        }
        pub fn set_eval_from_shared(&self, _shared: &SharedFunctionInfo) {}
        pub fn set_origin_options(&self, _options: i32) {}
    }

    impl<T> ops::Deref for Handle<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }

    impl FixedArray {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn get(&self, _i: i32) -> *const Object {
            &Object
        }
        pub fn set(&self, _i: i32, _value: *const Object, _mode: WriteBarrierMode) {}
    }

    pub enum WriteBarrierMode {
        SKIP_WRITE_BARRIER,
    }

    impl Map {
        pub fn update(_isolate: &Isolate, _map: DirectHandle<Map>) -> DirectHandle<Map> {
            DirectHandle(&Map)
        }

        pub fn as_elements_kind(_isolate: &Isolate, _map: Handle<Map>, _kind: ElementsKind) -> Handle<Map> {
            Handle(&Map)
        }

        pub fn number_of_own_descriptors(&self) -> i32 {
            0
        }

        pub fn get_in_object_properties(&self) -> i32 {
            0
        }

        pub fn unused_in_object_properties(&self) -> i32 {
            0
        }

        pub fn instance_descriptors(&self, _isolate: &Isolate) -> DescriptorArray {
            DescriptorArray
        }

        pub fn get_in_object_property_offset(&self, _i: i32) -> i32 {
            0
        }

        pub fn find_field_owner(&self, _isolate: &Isolate, _index: InternalIndex) -> *const Map {
            &Map
        }

        pub fn is_dictionary_map(&self) -> bool {
            false
        }

        pub fn elements_kind(&self) -> ElementsKind {
            ElementsKind::PACKED_SMI_ELEMENTS
        }

        pub fn is_detached(&self, _isolate: &Isolate) -> bool {
            false
        }

        pub fn is_deprecated(&self) -> bool {
            false
        }
    }

    impl ObjectTwoHashTable {
        pub fn new(_isolate: &Isolate, _size: i32) -> Handle<ObjectTwoHashTable> {
            Handle(&ObjectTwoHashTable)
        }

        pub fn lookup(&self, _isolate: &Isolate, _name: &Handle<String>) -> [*const Object; 2] {
            [&Object, &Object]
        }

        pub fn put(_isolate: &Isolate, _table: Handle<ObjectTwoHashTable>, _key: DirectHandle<String>, _value: [*const Object; 2]) -> Handle<ObjectTwoHashTable> {
            Handle(&ObjectTwoHashTable)
        }
    }

    impl SeqOneByteString {
        pub fn get_chars(&self, _no_gc: &DisallowGarbageCollection) -> *mut u8 {
            todo!()
        }
    }

    impl SeqTwoByteString {
        pub fn get_chars(&self, _no_gc: &DisallowGarbageCollection) -> *mut u16 {
            todo!()
        }
    }

    impl DescriptorArray {
        pub fn get_key(&self, _index: InternalIndex) -> *const Object {
            &Object
        }

        pub fn get_details(&self, _index: InternalIndex) -> PropertyDetails {
            PropertyDetails
        }

        pub fn get_field_type(&self, _index: InternalIndex) -> FieldType {
            FieldType
        }

        pub fn search_with_cache(&self, _isolate: &Isolate, _key: &String, _map: &Map) -> InternalIndex {
            InternalIndex(0)
        }
    }

    impl PropertyDetails {
        pub fn representation(&self) -> Representation {
            Representation::Tagged
        }

        pub fn kind(&self) -> PropertyKind {
            PropertyKind::kData
        }

        pub fn location(&self) -> PropertyLocation {
            PropertyLocation::kField
        }

        pub fn attributes(&self) -> Attributes {
            Attributes::NONE
        }

        pub fn constness(&self) -> PropertyConstness {
            PropertyConstness::kConst
        }
    }

    impl FieldType {
        pub fn any(_isolate: &Isolate) -> DirectHandle<FieldType> {
            DirectHandle(&FieldType)
        }
    }

    impl ByteArray {
        pub fn begin(&self) -> *mut u8 {
            todo!()
        }
        pub fn end(&self) -> *mut u8 {
            todo!()
        }
        pub fn set_length(&self, _length: i32) {}
    }

    pub struct InternalIndex(i32);

    impl InternalIndex {
        pub fn is_found(&self) -> bool {
            true
        }
    }

    pub struct Roots(Isolate);

    impl Roots {
        pub fn new(_isolate: &Isolate) -> Self {
            Roots(Isolate) // ignore provided isolate
        }
        pub fn heap_number_map(&self) -> *const Map {
            &Map
        }
    }

    pub struct TransitionsAccessor<'a>(&'a Isolate, Map);

    impl<'a> TransitionsAccessor<'a> {
        pub fn new(_isolate: &'a Isolate, _map: Map) -> Self {
            TransitionsAccessor(&Isolate, Map) // ignore isolate and map
        }

        pub fn can_have_more_transitions(_isolate: &Isolate, _map: &Map) -> bool {
            true
        }

        pub fn find_transition_to_field(&self, _key: &Handle<String>) -> MaybeHandle<Map> {
            MaybeHandle(Some(Handle(&Map)))
        }

        pub fn expected_transition<T: AsRef<[u8]>>(&self, _key_chars: T) -> (Handle<String>, DirectHandle<Map>) {
            (Handle(&String), DirectHandle(&Map))
        }
    }

    impl LookupIterator {
        pub fn new(_isolate: &Isolate, _object: Handle<JSObject>, _key: &Handle<String>, _holder: Handle<JSObject>, _options: LookupOptions) -> Self {
            LookupIterator
        }
    }

    pub struct LookupOptions;

    impl JSObject {
        pub fn define_own_property_ignore_attributes(_it: &LookupIterator, _value: &Handle<Object>, _attributes: Attributes) -> Result<(), ()> {
            Ok(())
        }

        pub fn set_elements(&self, _elements: *const FixedArrayBase) {}

        pub fn raw_fast_inobject_property_at_put(&self, _index: FieldIndex, _value: *const Object, _mode: WriteBarrierMode) {}

        pub fn get_write_barrier_mode(&self, _no_gc: &DisallowGarbageCollection) -> WriteBarrierMode {
            WriteBarrierMode::SKIP_WRITE_BARRIER
        }

        pub fn map(&self) -> *const Map {
            &Map
        }
    }

    pub struct FieldIndex;

    impl FieldIndex {
        pub fn for_property_index(_map: &Map, _i: i32) -> Self {
            FieldIndex
        }

        pub fn for_in_object_offset(_offset: i32, _tagged: i32) -> Self {
            FieldIndex
        }

        pub fn is_inobject(&self) -> bool {
            true
        }

        pub fn offset(&self) -> i32 {
            0
        }
    }

    impl PropertyDescriptor {
        pub fn set_value(&mut self, _value: Handle<Object>) {}
        pub fn set_configurable(&mut self, _configurable: bool) {}
        pub fn set_enumerable(&mut self, _enumerable: bool) {}
        pub fn set_writable(&mut self, _writable: bool) {}
    }

    // src/json/json-parser.cc
    const K_MIN_ORIGINAL_SOURCE_LENGTH_FOR_CONTEXT: usize = 80;
    const K_MAX_CONTEXT_CHARACTERS: usize = 20;

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum JsonToken {
        STRING,
        NUMBER,
        LBRACK,
        LBRACE,
        RBRACK,
        RBRACE,
        TRUE_LITERAL,
        FALSE_LITERAL,
        NULL_LITERAL,
        WHITESPACE,
        COLON,
        COMMA,
        ILLEGAL,
        EOS,
    }

    const fn get_one_char_json_token(c: u8) -> JsonToken {
        match c {
            b'"' => JsonToken::STRING,
            b'0'..=b'9' | b'-' => JsonToken::NUMBER,
            b'[' => JsonToken::LBRACK,
            b'{' => JsonToken::LBRACE,
            b']' => JsonToken::RBRACK,
            b'}' => JsonToken::RBRACE,
            b't' => JsonToken::TRUE_LITERAL,
            b'f' => JsonToken::FALSE_LITERAL,
            b'n' => JsonToken::NULL_LITERAL,
            b' ' | b'\t' | b'\r' | b'\n' => JsonToken::WHITESPACE,
            b':' => JsonToken::COLON,
            b',' => JsonToken::COMMA,
            _ => JsonToken::ILLEGAL,
        }
    }

    const ONE_CHAR_JSON_TOKENS: [JsonToken; 256] = {
        const fn generate_scan_flags() -> [JsonToken; 256] {
            let mut arr: [JsonToken; 256] = [JsonToken::ILLEGAL; 256];
            let mut i = 0;
            while i < 256 {
                arr[i] = get_one_char_json_token(i as u8);
                i += 1;
            }
            arr
        }
        generate_scan_flags()
    };

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum EscapeKind {
        Illegal,
        SelfValue,
        Backspace,
        Tab,
        NewLine,
        FormFeed,
        CarriageReturn,
        Unicode,
    }

    struct JsonScanFlags {
        escape_kind: EscapeKind,
        may_terminate_string: bool,
        number_part: bool,
    }

    const fn get_json_scan_flags(c: u8) -> JsonScanFlags {
        let escape_kind = match c {
            b'b' => EscapeKind::Backspace,
            b't' => EscapeKind::Tab,
            b'n' => EscapeKind::NewLine,
            b'f' => EscapeKind::FormFeed,
            b'r' => EscapeKind::CarriageReturn,
            b'u' => EscapeKind::Unicode,
            b'"' | b'\\' | b'/' => EscapeKind::SelfValue,
            _ => EscapeKind::Illegal,
        };

        let may_terminate_string = c < 0x20 || c == b'"' || c == b'\\';

        let number_part = c == b'.'
            || c == b'e'
            || c == b'E'
            || (b'0'..=b'9').contains(&c)
            || c == b'-'
            || c == b'+';

        JsonScanFlags {
            escape_kind,
            may_terminate_string,
            number_part,
        }
    }

    const CHARACTER_JSON_SCAN_FLAGS: [JsonScanFlags; 256] = {
        const fn generate_scan_flags() -> [JsonScanFlags; 256] {
            let mut arr: [JsonScanFlags; 256] = [JsonScanFlags {
                escape_kind: EscapeKind::Illegal,
                may_terminate_string: false,
                number_part: false,
            }; 256];
            let mut i = 0;
            while i < 256 {
                arr[i] = get_json_scan_flags(i as u8);
                i += 1;
            }
            arr
        }
        generate_scan_flags()
    };

    struct JsonParseInternalizer<'a> {
        isolate_: &'a Isolate,
        reviver_: *const JSReceiver,
        source_: Handle<String>,
    }

    impl<'a> JsonParseInternalizer<'a> {
        fn internalize(
            isolate: &'a Isolate,
            result: DirectHandle<Object>,
            reviver: Handle<Object>,
            source: Handle<String>,
            val_node: MaybeHandle<Object>,
        ) -> MaybeHandle<Object> {
            // if !Object::is_callable(&reviver) {
            //     panic!("reviver must be callable");
            // }

            let internalizer = JsonParseInternalizer {
                isolate_: isolate,
                reviver_: reviver.0 as *const JSReceiver, // TODO: static cast only because the real check is missing,
                source_: source,
            };

            let holder = isolate.factory().new_js_object(isolate.object_function());
            let name = isolate.factory().empty_string();
            unsafe {
                JSReceiver::add_property(
                    isolate,
                    &holder,
                    &name,
                    &result,
                    Attributes::NONE,
                );
            }
            internalizer.internalize_json_property::<{ WithOrWithoutSource::WithSource as i32 }>(holder, name, val_node.to_handle().unwrap(), result)
        }

        fn internalize_json_property<const WITH_SOURCE: i32>(
            &self,
            holder: DirectHandle<JSObject>,
            name: DirectHandle<String>,
            val_node: Handle<Object>,
            snapshot: DirectHandle<Object>,
        ) -> MaybeHandle<Object> {
            let isolate_ = self.isolate_;
            {
                let _outer_scope = HandleScope::new(isolate_);
                let value = match unsafe { Object::get_property_or_element(isolate_, &holder, &Handle(name.0)).map(|h| Handle(h.0)) } {
                    Ok(v) => v,
                    Err(_) => return MaybeHandle(None),
                };

                let pass_source_to_reviver = WITH_SOURCE == WithOrWithoutSource::WithSource as i32
                    && value.0 == snapshot.0;

                if value.0 as *const Object as *const _ != std::ptr::null() /*Object::is_js_receiver(&value)*/ {
                    let object = value;
                    let is_array = match unsafe {Object::get_length_from_array_like(isolate_, &Handle(object.0 as *const JSReceiver)).map(|h| Handle(h.0))} {
                        Ok(_len) => {
                            // TODO: add Object::is_array after the type is done
                            Maybe::Just(true)
                        },
                        Err(_) => return MaybeHandle(None)
                    };
                    if is_array.is_nothing() {
                        return MaybeHandle(None);
                    }
                    if is_array.from_just() {
                        let length_object = unsafe { Object::get_length_from_array_like(isolate_, &Handle(object.0 as *const JSReceiver)).unwrap() };
                        let length = unsafe { Object::number_value(&*length_object.0) };

                        if pass_source_to_reviver {
                            // TODO: add Cast to FixedArray after FixedArray type is done
                            let val_nodes_and_snapshots = val_node;
                            let snapshot_length = unsafe { (&*val_nodes_and_snapshots.0 as *const FixedArray)->length() } / 2;

                            for i in 0..length as i32 {
                                {
                                    let _inner_scope = HandleScope::new(isolate_);
                                    let index = isolate_.factory().new_number(i);
                                    let index_name = isolate_.factory().uint32_to_string(i as u32);

                                    // Even if the array pointer snapshot matched, it's possible the
                                    // array had new elements added that are not in the snapshotted
                                    // elements.
                                    let rv = if i < snapshot_length {
                                        // TODO: remove static cast after generic types are implemented
                                        self.recurse_and_apply::<{ WithOrWithoutSource::WithSource as i32 }>(
                                            Handle(object.0 as *const JSReceiver),
                                            Handle(index_name.0),
                                            Handle(unsafe { (&*val_nodes_and_snapshots.0 as *const FixedArray)->get(i * 2) }),
                                            Handle(unsafe { (&*val_nodes_and_snapshots.0 as *const FixedArray)->get(i * 2 + 1) }),
                                        )
                                    } else {
                                        self.recurse_and_apply::<{ WithOrWithoutSource::WithoutSource as i32 }>(
                                            Handle(object.0 as *const JSReceiver),
                                            Handle(index_name.0),
                                            Handle(std::ptr::null()),
                                            Handle(std::ptr::null()),
                                        )
                                    };
                                    if !rv {
                                        return MaybeHandle(None);
                                    }
                                }
                            }
                        } else {
                            for i in 0..length as i32 {
                                {
                                    let _inner_scope = HandleScope::new(isolate_);
                                    let index = isolate_.factory().new_number(i);
                                    let index_name = isolate_.factory().uint32_to_string(i as u32);
                                    // TODO: remove static cast after generic types are implemented
                                    if !self.recurse_and_apply::<{ WithOrWithoutSource::WithoutSource as i32 }>(
                                        Handle(object.0 as *const JSReceiver),
                                        Handle(index_name.0),
                                        Handle(std::ptr::null()),
                                        Handle(std::ptr::null()),
                                    ) {
                                        return MaybeHandle(None);
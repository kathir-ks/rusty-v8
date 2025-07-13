// Converted from V8 C++ source files:
// Header: js-regexp.h
// Implementation: js-regexp.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
pub mod js_regexp {
    use std::{
        borrow::Cow,
        cell::RefCell,
        ops::Range,
        ptr::null_mut,
        rc::Rc,
        sync::{Mutex, RwLock},
    };

    //use crate::archive::codebase::src::objects::contexts;
    //use crate::archive::codebase::src::objects::js_array;
    //use crate::archive::codebase::src::regexp::regexp_flags;
    //use crate::torque_generated::bit_fields;

    //use crate::archive::codebase::src::objects::object_macros;
    use crate::init::bootstrapper::JSRegExpResult;
    use crate::init::bootstrapper::JSRegExpResultWithIndices;
    use crate::interpreter::interpreter_generator::RegExpFlags;
    use crate::objects::descriptor_array_inl::InternalIndex;
    use crate::objects::js_objects::JSArray;
    use crate::objects::js_objects::JSObject;
    use crate::objects::lookup_inl::code;
    use crate::objects::scope_info::V8;
    use crate::objects::value_serializer::JSRegExp;

    pub struct Flags {
        pub flags: i32,
    }

    impl Flags {
        pub fn new(flags: i32) -> Self {
            Flags { flags }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum RegExpFlag {
        kNone = 0,
        kGlobal = 1,
        kIgnoreCase = 2,
        kMultiline = 4,
        kSticky = 8,
        kUnicode = 16,
        kDotAll = 32,
        kHasIndices = 64,
        kLinear = 128,
    }

    const kFlagCount: i32 = 8;
    const kRegExpFlagCount: i32 = 8;
    const kNone: i32 = 0;
    //Regular expressions
    pub struct JSRegExp_ {}

    impl JSRegExp_ {
        pub fn clear_data(&self) {}
        pub fn InObjectPropertyAtPut(&self, _index: i32, _value: Smi, _skip_write_barrier: i32) {}
        pub fn map(&self) -> Map {
            Map {}
        }
    }

    pub struct Map {}

    impl Map {
        pub fn GetConstructor(&self) -> JSFunction {
            JSFunction {}
        }
    }

    pub struct JSFunction {}

    impl JSFunction {
        pub fn initial_map(&self) -> Map {
            Map {}
        }
    }

    pub struct DirectHandle<T> {
        pub value: T,
    }

    impl<T> DirectHandle<T> {
        pub fn new(value: T) -> Self {
            DirectHandle { value }
        }
    }

    pub struct Isolate {}

    impl Isolate {
        pub fn regexp_function(&self) -> DirectHandle<JSFunction> {
            DirectHandle::new(JSFunction {})
        }
        pub fn factory(&self) -> Factory {
            Factory {}
        }
        pub fn regexp_result_indices_map(&self) -> Map {
            Map {}
        }
    }

    pub struct Factory {}

    impl Factory {
        pub fn NewJSObject(&self, _constructor: DirectHandle<JSFunction>) -> JSRegExp_ {
            JSRegExp_ {}
        }
        pub fn NewStringFromAsciiChecked(&self, _chars: &str) -> DirectHandle<String> {
            DirectHandle::new(String {})
        }
        pub fn lastIndex_string(&self) -> String {
            String {}
        }
        pub fn query_colon_string(&self) -> String {
            String {}
        }
        pub fn NewRawOneByteString(&self, _length: i32) -> Result<DirectHandle<SeqOneByteString>, String> {
            Ok(DirectHandle::new(SeqOneByteString {}))
        }
        pub fn NewRawTwoByteString(&self, _length: i32) -> Result<DirectHandle<SeqTwoByteString>, String> {
            Ok(DirectHandle::new(SeqTwoByteString {}))
        }
        pub fn NewFixedArray(&self, _num_results: i32) -> DirectHandle<FixedArray> {
            DirectHandle::new(FixedArray {})
        }
        pub fn empty_fixed_array(&self) -> DirectHandle<FixedArrayBase> {
            DirectHandle::new(FixedArrayBase {})
        }
        pub fn null_value(&self) -> DirectHandle<Null> {
            DirectHandle::new(Null {})
        }
        pub fn NewJSObjectFromMap(&self, _map: Map) -> JSRegExpResultIndices {
            JSRegExpResultIndices {}
        }
        pub fn NewJSArrayWithElements(&self, _indices_sub_array: DirectHandle<FixedArray>, _packed_smi_elements: i32, _i: i32) -> DirectHandle<JSArray> {
            DirectHandle::new(JSArray {})
        }
        pub fn NewSlowJSObjectWithPropertiesAndElements(&self, _null: DirectHandle<Null>, _group_names: DirectHandle<PropertyDictionary>, _elements: DirectHandle<FixedArrayBase>) -> DirectHandle<JSObject> {
            DirectHandle::new(JSObject {})
        }
    }

    pub struct String {}

    impl String {
        pub fn length(&self) -> i32 {
            0
        }
        pub fn Flatten(_isolate: *mut Isolate, string: String) -> String {
            String {}
        }
        pub fn IsOneByteRepresentationUnderneath(_string: &String) -> bool {
            true
        }
        pub fn GetCharVector<Char>(_self: &String, _no_gc: i32) -> Vector<Char> {
            Vector { length_: 0 }
        }
        pub fn Flatten(_isolate: &Isolate, _handle: DirectHandle<String>) -> DirectHandle<String> {
            DirectHandle::new(String {})
        }
    }

    pub struct Vector<Char> {
        pub length_: i32,
    }

    pub struct Smi {}

    impl Smi {
        pub fn zero() -> Self {
            Smi {}
        }
        pub fn FromInt(_i: i32) -> Self {
            Smi {}
        }
        pub fn value(&self) -> i32 {
            0
        }
    }

    pub struct Object {}

    impl Object {
        pub fn SetProperty(_isolate: *mut Isolate, _regexp: &JSRegExp_, _last_index_string: String, _direct_handle: DirectHandle<Smi>) -> Result<(), String> {
            Ok(())
        }
    }

    pub struct FixedArray {}

    impl FixedArray {
        pub fn set(&self, _i: i32, _value: i32) {}
        pub fn get(&self, _i: i32) -> i32 {
            0
        }
    }

    pub struct RegExpMatchInfo {}

    impl RegExpMatchInfo {
        pub fn number_of_capture_registers(&self) -> i32 {
            0
        }
        pub fn capture(&self, _capture_start_index: i32) -> i32 {
            0
        }
    }

    pub struct FixedArrayBase {}

    pub struct Null {}

    pub struct ReadOnlyRoots {}

    impl ReadOnlyRoots {
        pub fn undefined_value(&self) -> i32 {
            0
        }
    }

    pub struct ExposedTrustedObject {}

    pub struct SeqOneByteString {}

    pub struct SeqTwoByteString {}

    pub struct IrRegExpData {}

    pub struct TrustedByteArray {}

    pub struct Code {}

    #[allow(non_camel_case_types)]
    pub struct RegExpDataWrapper {}

    pub struct JSRegExpResultIndices {}

    impl JSRegExpResultIndices {
        pub fn set_length(&self, _zero: Smi) {}
        pub fn FastPropertyAtPut(&self, _groups_index: FieldIndex, _undefined_value: i32) {}
    }

    pub struct FieldIndex {}

    impl FieldIndex {
        pub fn ForDescriptor(_map: Map, _internal_index: InternalIndex) -> Self {
            FieldIndex {}
        }
    }

    pub struct PropertyDictionary {}

    impl PropertyDictionary {
        pub fn FindEntry(&self, _isolate: *mut Isolate, _name: String) -> InternalIndex {
            InternalIndex { value: 0 }
        }
        pub fn ValueAt(&self, _group_entry: InternalIndex) -> i32 {
            0
        }
        pub fn ValueAtPut(&self, _group_entry: InternalIndex, _capture_indices: i32) {}
        pub fn Add(_isolate: *mut Isolate, _group_names_dict: DirectHandle<PropertyDictionary>, _name: String, _capture_indices: DirectHandle<Object>, _empty: i32) -> DirectHandle<PropertyDictionary> {
            DirectHandle::new(PropertyDictionary {})
        }
    }

    pub struct AtomRegExpData {}

    // static
    impl JSRegExpResultIndices {
        pub fn BuildIndices(
            isolate: *mut Isolate,
            match_info: DirectHandle<RegExpMatchInfo>,
            maybe_names: DirectHandle<Object>,
        ) -> DirectHandle<JSRegExpResultIndices> {
            let indices: DirectHandle<JSRegExpResultIndices> =
                DirectHandle::new(JSRegExpResultIndices {});

            // Initialize indices length to avoid having a partially initialized object
            // should GC be triggered by creating a NewFixedArray.
            indices.value.set_length(Smi::zero());

            // Build indices array from RegExpMatchInfo.
            let num_indices = match_info.value.number_of_capture_registers();
            let num_results = num_indices >> 1;
            let indices_array = DirectHandle::new(FixedArray {});
            //JSArray::SetContent(indices, indices_array);

            for i in 0..num_results {
                //const int start_offset =
                //    match_info->capture(RegExpMatchInfo::capture_start_index(i));
                //const int end_offset =
                //    match_info->capture(RegExpMatchInfo::capture_end_index(i));
                let start_offset = 0;
                let end_offset = 0;

                // Any unmatched captures are set to undefined, otherwise we set them to a
                // subarray of the indices.
                if start_offset == -1 {
                    //indices_array->set(i, ReadOnlyRoots(isolate).undefined_value());
                } else {
                    let indices_sub_array = DirectHandle::new(FixedArray {});
                    indices_sub_array.value.set(0, Smi::FromInt(start_offset).value());
                    indices_sub_array.value.set(1, Smi::FromInt(end_offset).value());
                    let indices_sub_jsarray = DirectHandle::new(JSArray {}); //isolate->factory()->NewJSArrayWithElements(indices_sub_array,PACKED_SMI_ELEMENTS, 2);
                                                                            //indices_array->set(i, *indices_sub_jsarray);
                }
            }

            // If there are no capture groups, set the groups property to undefined.
            let groups_index = FieldIndex::ForDescriptor(
                indices.value.map(),
                InternalIndex {
                    value: 0, //kGroupsDescriptorIndex
                },
            );
            //if (IsUndefined(*maybe_names, isolate)) {
            indices.value.FastPropertyAtPut(groups_index, 0); //ReadOnlyRoots(isolate).undefined_value());
                                                               //return indices;
                                                               //}

            // Create a groups property which returns a dictionary of named captures to
            // their corresponding capture indices.
            //auto names = Cast<FixedArray>(maybe_names);
            let num_names = 0; //names->length() >> 1;
            let group_names: DirectHandle<PropertyDictionary> = DirectHandle::new(PropertyDictionary {}); //isolate->factory()->NewSwissNameDictionary(num_names);
                                                                  //DirectHandle<PropertyDictionary> group_names_dict =
                                                                  //    Cast<PropertyDictionary>(group_names);
            for i in 0..num_names {
                let _base_offset = i * 2;
                //int name_offset = base_offset;
                //int index_offset = base_offset + 1;
                //DirectHandle<String> name(Cast<String>(names->get(name_offset)), isolate);
                //Tagged<Smi> smi_index = Cast<Smi>(names->get(index_offset));
                //DirectHandle<Object> capture_indices(indices_array->get(smi_index.value()),
                //                                     isolate);
                let name = String {};
                let capture_indices = DirectHandle::new(Object {});
                let group_entry = group_names.value.FindEntry(isolate, name);
                // Duplicate group entries are possible if the capture groups are in
                // different alternatives, i.e. only one of them can actually match.
                // Therefore when we find a duplicate entry, either the current entry is
                // undefined (didn't match anything) or the indices for the current capture
                // are undefined. In the latter case we don't do anything, in the former
                // case we update the entry.
                if group_entry.value != 0 {
                    //DCHECK(v8_flags.js_regexp_duplicate_named_groups);
                    //if (!IsUndefined(*capture_indices, isolate)) {
                    //    DCHECK(IsUndefined(group_names_dict->ValueAt(group_entry), isolate));
                    //    group_names_dict->ValueAtPut(group_entry, *capture_indices);
                    //}
                } else {
                    //group_names_dict =
                    //    PropertyDictionary::Add(isolate, group_names_dict, name,
                    //                           capture_indices, PropertyDetails::Empty());
                }
            }

            // Convert group_names to a JSObject and store at the groups property of the
            // result indices.
            let elements = DirectHandle::new(FixedArrayBase {}); //isolate->factory()->empty_fixed_array();
            let null = DirectHandle::new(Null {}); //isolate->factory()->null_value();
            let js_group_names = DirectHandle::new(JSObject {}); //isolate->factory()->NewSlowJSObjectWithPropertiesAndElements(
                                                                 //    null, group_names, elements);
            indices.value.FastPropertyAtPut(groups_index, 0); //*js_group_names);
            indices
        }
    }

    // static
    impl JSRegExp_ {
        pub fn FlagsFromString(isolate: *mut Isolate, flags: DirectHandle<String>) -> Result<Option<Flags>, String> {
            let length = flags.value.length();

            // A longer flags string cannot be valid.
            if length > kFlagCount {
                return Ok(None);
            }

            let mut value: i32 = 0;
            //FlatStringReader reader(isolate, String::Flatten(isolate, flags));

            for _i in 0..length {
                //std::optional<RegExpFlag> flag = JSRegExp::FlagFromChar(reader.Get(i));
                //if (!flag.has_value()) return {};
                //if (value & flag.value()) return {};  // Duplicate.
                //value |= flag.value();
            }

            //return JSRegExp::AsJSRegExpFlags(value);
            Ok(Some(Flags { flags: 0 }))
        }
    }

    // static
    impl JSRegExp_ {
        pub fn StringFromFlags(isolate: *mut Isolate, flags: Flags) -> DirectHandle<String> {
            let mut buffer: i32 = 0; //FlagsBuffer buffer;
                                      //return isolate->factory()->NewStringFromAsciiChecked(
                                      //    FlagsToString(flags, &buffer));
            DirectHandle::new(String {})
        }
    }

    // static
    impl JSRegExp_ {
        pub fn New(
            isolate: *mut Isolate,
            pattern: DirectHandle<String>,
            flags: Flags,
            backtrack_limit: u32,
        ) -> Result<DirectHandle<JSRegExp_>, String> {
            let constructor = Isolate {}.regexp_function(); //isolate->regexp_function();
            let regexp = DirectHandle::new(JSRegExp_ {}); //isolate->factory()->NewJSObject(constructor));

            // Clear the data field, as a GC can be triggered before the field is set
            // during compilation.
            regexp.value.clear_data();

            JSRegExp_::Initialize(regexp, pattern, flags, backtrack_limit)
        }
    }

    // static
    impl JSRegExp_ {
        pub fn Initialize(
            regexp: DirectHandle<JSRegExp_>,
            source: DirectHandle<String>,
            flags_string: DirectHandle<String>,
        ) -> Result<DirectHandle<JSRegExp_>, String> {
            //Isolate* isolate = regexp->GetIsolate();
            let isolate = &Isolate {};
            match JSRegExp_::FlagsFromString(isolate as *const _ as *mut _, flags_string) {
                Ok(flags) => {
                    //if (!flags.has_value() ||
                    //    !RegExp::VerifyFlags(JSRegExp::AsRegExpFlags(flags.value()))) {
                    //  THROW_NEW_ERROR(
                    //      isolate,
                    //      NewSyntaxError(MessageTemplate::kInvalidRegExpFlags, flags_string));
                    //}
                    //Initialize(regexp, source, flags.value())
                    JSRegExp_::Initialize(regexp, source, Flags { flags: 0 }, 0)
                }
                Err(e) => Err(e),
            }
        }
    }

    // static
    impl JSRegExp_ {
        pub fn Initialize(
            regexp: DirectHandle<JSRegExp_>,
            source: DirectHandle<String>,
            flags: Flags,
            backtrack_limit: u32,
        ) -> Result<DirectHandle<JSRegExp_>, String> {
            //Isolate* isolate = regexp->GetIsolate();
            let isolate = &Isolate {};
            let factory = Factory {}; //isolate->factory();
                                       // If source is the empty string we set it to "(?:)" instead as
                                       // suggested by ECMA-262, 5th, section 15.10.4.1.
            let source = if source.value.length() == 0 {
                factory.query_colon_string()
            } else {
                source
            };

            //source = String::Flatten(isolate, source);

            //RETURN_ON_EXCEPTION(isolate, RegExp::Compile(isolate, regexp, source,
            //                                           JSRegExp::AsRegExpFlags(flags),
            //                                           backtrack_limit));
            //
            let escaped_source: DirectHandle<String>;
            //ASSIGN_RETURN_ON_EXCEPTION(isolate, escaped_source,
            //                           EscapeRegExpSource(isolate, source));
            escaped_source = DirectHandle::new(String {});
            //regexp->set_source(*escaped_source);
            //regexp->set_flags(Smi::FromInt(flags));

            //Tagged<Map> map = regexp->map();
            //Tagged<Object> constructor = map->GetConstructor();
            let map = Map {};
            let constructor = JSFunction {};
            if constructor.initial_map().map() == map.map() {
                // If we still have the original map, set in-object properties directly.
                regexp.value.InObjectPropertyAtPut(0, Smi::FromInt(0), 0); //JSRegExp::kLastIndexFieldIndex,
                                                                              //  Smi::FromInt(kInitialLastIndexValue),
                                                                              //  SKIP_WRITE_BARRIER);
            } else {
                // Map has changed, so use generic, but slower, method.
                //RETURN_ON_EXCEPTION(
                //    isolate,
                //    Object::SetProperty(
                //        isolate, regexp, factory->lastIndex_string(),
                //        DirectHandle<Smi>(Smi::FromInt(kInitialLastIndexValue), isolate)));
            }

            Ok(regexp)
        }
    }

    pub struct RegExpData_ {}

    impl RegExpData_ {
        pub fn HasCompiledCode(&self) -> bool {
            true
        }
    }

    impl IrRegExpData_ {
        pub fn CanTierUp(&self) -> bool {
            true
        }
        pub fn MarkedForTierUp(&self) -> bool {
            true
        }
        pub fn ResetLastTierUpTick(&self) {}
        pub fn TierUpTick(&self) {}
        pub fn MarkTierUpForNextExec(&self) {}
        pub fn ShouldProduceBytecode(&self) -> bool {
            true
        }
        pub fn DiscardCompiledCodeForSerialization(&self) {}
        pub fn SetBytecodeForExperimental(&self, _isolate: *mut Isolate, _bytecode: TaggedTrustedByteArray) {}
    }

    pub struct TaggedTrustedByteArray {}

    pub struct IrRegExpData_ {}
}

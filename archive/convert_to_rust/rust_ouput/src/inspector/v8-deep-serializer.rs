// Converted from V8 C++ source files:
// Header: v8-deep-serializer.h
// Implementation: v8-deep-serializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/inspector/v8-deep-serializer.h
pub mod v8_deep_serializer {
    use crate::inspector::protocol::Runtime;
    use crate::inspector::v8_serialization_duplicate_tracker::V8SerializationDuplicateTracker;
    use v8::Local;
    use v8::Object;
    use v8::Context;

    pub struct V8DeepSerializer {
        isolate: *mut v8::Isolate,
    }

    impl V8DeepSerializer {
        pub fn serialize_v8_value(
            value: Local<'_, Object>,
            context: Local<'_, Context>,
            max_depth: i32,
            additional_parameters: Local<'_, Object>,
            duplicate_tracker: &mut V8SerializationDuplicateTracker,
            result: &mut Runtime::DictionaryValue,
        ) -> Result<(), String> {
            v8_deep_serializer::serialize_v8_value(value, context, max_depth, additional_parameters, duplicate_tracker, result)
        }

        pub fn new(isolate: *mut v8::Isolate) -> Self {
            V8DeepSerializer { isolate }
        }
    }

    // src/inspector/v8-deep-serializer.cc
    use std::mem::MaybeUninit;
    use std::ptr;
    use std::rc::Rc;

    use v8::{Array, Date, Isolate, Map, RegExp, Set, String as V8String, Value};

    use crate::inspector::protocol::{
        Runtime::{self, DeepSerializedValue::TypeEnum},
        Value as ProtocolValue,
    };
    use crate::inspector::string_util::{String16, String16Builder, toProtocolString};
    use crate::inspector::v8_serialization_duplicate_tracker::DuplicateHandle;
    use crate::inspector::value_mirror::ValueMirror;

    fn description_for_date(
        context: Local<'_, Context>,
        date: Local<'_, Date>,
    ) -> Result<Box<dyn ProtocolValue>, String> {
        let isolate = context.isolate();
        let mut try_catch = v8::TryCatch::new(isolate);

        let date_iso_string = date.to_iso_string(context).ok_or("Failed to convert date to ISO string".to_string())?;
        let protocol_string = toProtocolString(isolate, date_iso_string);
        Ok(Box::new(Runtime::StringValue::create(protocol_string)))
    }

    fn description_for_regexp_flags(value: Local<'_, RegExp>) -> String16 {
        let mut result_string_builder = String16Builder::new();
        let flags = value.get_flags();
        if flags & v8::RegExp::Flags::HAS_INDICES != v8::RegExp::Flags::empty() {
            result_string_builder.append('d');
        }
        if flags & v8::RegExp::Flags::GLOBAL != v8::RegExp::Flags::empty() {
            result_string_builder.append('g');
        }
        if flags & v8::RegExp::Flags::IGNORE_CASE != v8::RegExp::Flags::empty() {
            result_string_builder.append('i');
        }
        if flags & v8::RegExp::Flags::LINEAR != v8::RegExp::Flags::empty() {
            result_string_builder.append('l');
        }
        if flags & v8::RegExp::Flags::MULTILINE != v8::RegExp::Flags::empty() {
            result_string_builder.append('m');
        }
        if flags & v8::RegExp::Flags::DOT_ALL != v8::RegExp::Flags::empty() {
            result_string_builder.append('s');
        }
        if flags & v8::RegExp::Flags::UNICODE != v8::RegExp::Flags::empty() {
            result_string_builder.append('u');
        }
        if flags & v8::RegExp::Flags::UNICODE_SETS != v8::RegExp::Flags::empty() {
            result_string_builder.append('v');
        }
        if flags & v8::RegExp::Flags::STICKY != v8::RegExp::Flags::empty() {
            result_string_builder.append('y');
        }
        result_string_builder.to_string()
    }

    fn serialize_regexp(
        value: Local<'_, RegExp>,
        context: Local<'_, Context>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Regexp.to_string());

        let mut result_value = Runtime::DictionaryValue::create();

        let source = value.get_source(context).ok_or("Could not get regexp source".to_string())?;
        let pattern = toProtocolString(context.isolate(), source);

        result_value.set_value(
            "pattern".to_string(),
            Box::new(Runtime::StringValue::create(pattern)),
        );

        let flags = description_for_regexp_flags(value);
        if !flags.is_empty() {
            result_value.set_value(
                "flags".to_string(),
                Box::new(Runtime::StringValue::create(flags)),
            );
        }

        result.set_value("value".to_string(), Box::new(result_value));
        Ok(())
    }

    fn serialize_date(
        value: Local<'_, Date>,
        context: Local<'_, Context>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Date.to_string());
        let date_description = description_for_date(context, value)?;

        result.set_value("value".to_string(), date_description);
        Ok(())
    }

    fn serialize_array_value(
        value: Local<'_, Array>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::ListValue,
    ) -> Result<(), String> {
        let length = value.length(context).ok_or("Failed to get array length".to_string())?;
        result.reserve(length as usize);
        for i in 0..length {
            let element_value = value.get_index(context, i).ok_or(format!("Failed to get array element at index {}", i))?;

            let mut element_protocol_value = Runtime::DictionaryValue::create();
            let response = ValueMirror::create(context, element_value)
                .build_deep_serialized_value(
                    context,
                    max_depth - 1,
                    additional_parameters,
                    duplicate_tracker,
                    &mut element_protocol_value,
                );
            if let Err(err) = response {
                return Err(err);
            }
            result.push_value(Box::new(element_protocol_value));
        }
        Ok(())
    }

    fn serialize_array(
        value: Local<'_, Array>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Array.to_string());

        if max_depth > 0 {
            let mut serialized_value = Runtime::ListValue::create();
            let response = serialize_array_value(
                value,
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                &mut serialized_value,
            );
            if let Err(err) = response {
                return Err(err);
            }

            result.set_value("value".to_string(), Box::new(serialized_value));
        }

        Ok(())
    }

    fn serialize_map(
        value: Local<'_, Map>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Map.to_string());

        if max_depth > 0 {
            let mut serialized_value = Runtime::ListValue::create();

            let properties_and_values = value.to_array(context).ok_or("Could not convert map to array".to_string())?;

            let length = properties_and_values.length(context).ok_or("Could not get propertiesAndValues length".to_string())?;
            serialized_value.reserve(length as usize);

            for i in (0..length).step_by(2) {
                let key_v8_value = properties_and_values.get_index(context, i).ok_or("Could not get keyV8Value".to_string())?;
                let property_v8_value = properties_and_values.get_index(context, i + 1).ok_or("Could not get propertyV8Value".to_string())?;

                let key_protocol_value: Box<dyn ProtocolValue>;
                let mut property_protocol_value = Runtime::DictionaryValue::create();

                if key_v8_value.is_string() {
                    let key_string = key_v8_value.to_string(context).ok_or("Could not convert keyV8Value to string".to_string())?;
                    key_protocol_value = Box::new(Runtime::StringValue::create(toProtocolString(context.isolate(), key_string)));
                } else {
                    let mut key_dictionary_protocol_value = Runtime::DictionaryValue::create();
                    let response = ValueMirror::create(context, key_v8_value)
                        .build_deep_serialized_value(
                            context,
                            max_depth - 1,
                            additional_parameters,
                            duplicate_tracker,
                            &mut key_dictionary_protocol_value,
                        );
                    if let Err(err) = response {
                        return Err(err);
                    }
                    key_protocol_value = Box::new(key_dictionary_protocol_value);
                }

                let response = ValueMirror::create(context, property_v8_value)
                    .build_deep_serialized_value(
                        context,
                        max_depth - 1,
                        additional_parameters,
                        duplicate_tracker,
                        &mut property_protocol_value,
                    );
                if let Err(err) = response {
                    return Err(err);
                }

                let mut key_value_list = Runtime::ListValue::create();

                key_value_list.push_value(key_protocol_value);
                key_value_list.push_value(Box::new(property_protocol_value));

                serialized_value.push_value(Box::new(key_value_list));
            }
            result.set_value("value".to_string(), Box::new(serialized_value));
        }

        Ok(())
    }

    fn serialize_set(
        value: Local<'_, Set>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Set.to_string());

        if max_depth > 0 {
            let mut serialized_value = Runtime::ListValue::create();
            let array = value.to_array(context).ok_or("Could not convert set to array".to_string())?;
            let response = serialize_array_value(
                array,
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                &mut serialized_value,
            );
            if let Err(err) = response {
                return Err(err);
            }
            result.set_value("value".to_string(), Box::new(serialized_value));
        }
        Ok(())
    }

    fn serialize_object_value(
        value: Local<'_, Object>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::ListValue,
    ) -> Result<(), String> {
        let property_names = value
            .get_own_property_names(
                context,
                v8::PropertyFilter::ONLY_ENUMERABLE | v8::PropertyFilter::SKIP_SYMBOLS,
            )
            .ok_or("Failed to get own property names".to_string())?;

        let length = property_names.length(context).ok_or("Failed to get property names length".to_string())?;
        result.reserve(length as usize);

        for i in 0..length {
            let key_v8_value = property_names.get_index(context, i).ok_or(format!("Failed to get property name at index {}", i))?;

            if !key_v8_value.is_string() {
                continue;
            }

            let key_string = key_v8_value.to_string(context).ok_or("Could not convert keyV8Value to string".to_string())?;

            let has_real_named_property = value.has_real_named_property(context, key_string);

            if has_real_named_property.is_none() || !has_real_named_property.unwrap() {
                continue;
            }

            let key_protocol_value = Box::new(Runtime::StringValue::create(toProtocolString(
                context.isolate(),
                key_string,
            )));

            let property_v8_value = value.get(context, key_v8_value).ok_or(format!("Failed to get property value for key {:?}", key_v8_value))?;

            let mut property_protocol_value = Runtime::DictionaryValue::create();
            let response = ValueMirror::create(context, property_v8_value)
                .build_deep_serialized_value(
                    context,
                    max_depth - 1,
                    additional_parameters,
                    duplicate_tracker,
                    &mut property_protocol_value,
                );
            if let Err(err) = response {
                return Err(err);
            }

            let mut key_value_list = Runtime::ListValue::create();

            key_value_list.push_value(key_protocol_value);
            key_value_list.push_value(Box::new(property_protocol_value));

            result.push_value(Box::new(key_value_list));
        }
        Ok(())
    }

    fn serialize_object(
        value: Local<'_, Object>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        result.set_string("type".to_string(), TypeEnum::Object.to_string());

        if max_depth > 0 {
            let mut serialized_value = Runtime::ListValue::create();
            let response = serialize_object_value(
                value,
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                &mut serialized_value,
            );
            if let Err(err) = response {
                return Err(err);
            }
            result.set_value("value".to_string(), Box::new(serialized_value));
        }
        Ok(())
    }

    pub fn serialize_v8_value(
        value: Local<'_, Object>,
        context: Local<'_, Context>,
        max_depth: i32,
        additional_parameters: Local<'_, Object>,
        duplicate_tracker: &mut V8SerializationDuplicateTracker,
        result: &mut Runtime::DictionaryValue,
    ) -> Result<(), String> {
        if value.is_array() {
            return serialize_array(
                value.try_into().unwrap(),
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                result,
            );
        }
        if value.is_regexp() {
            return serialize_regexp(
                value.try_into().unwrap(),
                context,
                duplicate_tracker,
                result,
            );
        }
        if value.is_date() {
            return serialize_date(
                value.try_into().unwrap(),
                context,
                duplicate_tracker,
                result,
            );
        }
        if value.is_map() {
            return serialize_map(
                value.try_into().unwrap(),
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                result,
            );
        }
        if value.is_set() {
            return serialize_set(
                value.try_into().unwrap(),
                context,
                max_depth,
                additional_parameters,
                duplicate_tracker,
                result,
            );
        }
        if value.is_weak_map() {
            result.set_string("type".to_string(), TypeEnum::Weakmap.to_string());
            return Ok(());
        }
        if value.is_weak_set() {
            result.set_string("type".to_string(), TypeEnum::Weakset.to_string());
            return Ok(());
        }
        if value.is_native_error() {
            result.set_string("type".to_string(), TypeEnum::Error.to_string());
            return Ok(());
        }
        if value.is_proxy() {
            result.set_string("type".to_string(), TypeEnum::Proxy.to_string());
            return Ok(());
        }
        if value.is_promise() {
            result.set_string("type".to_string(), TypeEnum::Promise.to_string());
            return Ok(());
        }
        if value.is_typed_array() {
            result.set_string("type".to_string(), TypeEnum::Typedarray.to_string());
            return Ok(());
        }
        if value.is_array_buffer() {
            result.set_string("type".to_string(), TypeEnum::Arraybuffer.to_string());
            return Ok(());
        }
        if value.is_function() {
            result.set_string("type".to_string(), TypeEnum::Function.to_string());
            return Ok(());
        }
        if value.is_generator_object() {
            result.set_string("type".to_string(), TypeEnum::Generator.to_string());
            return Ok(());
        }

        serialize_object(
            value,
            context,
            max_depth,
            additional_parameters,
            duplicate_tracker,
            result,
        )
    }
}

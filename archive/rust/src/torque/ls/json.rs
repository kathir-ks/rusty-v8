// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ls {
    use std::collections::HashMap;

    /// Represents a JSON object.
    pub type JsonObject = HashMap<String, JsonValue>;

    /// Represents a JSON array.
    pub type JsonArray = Vec<JsonValue>;

    /// Represents a JSON value.
    #[derive(Debug, PartialEq, Clone)]
    pub enum JsonValue {
        Object(JsonObject),
        Array(JsonArray),
        String(String),
        Number(f64),
        Bool(bool),
        Null,
    }

    impl JsonValue {
        /// Creates a JsonValue from a number.
        pub fn from_number(number: f64) -> Self {
            JsonValue::Number(number)
        }

        /// Creates a JsonValue from an object.
        pub fn from_object(object: JsonObject) -> Self {
            JsonValue::Object(object)
        }

        /// Creates a JsonValue from a boolean.
        pub fn from_bool(b: bool) -> Self {
            JsonValue::Bool(b)
        }

        /// Creates a JsonValue from a string.
        pub fn from_string(string: String) -> Self {
            JsonValue::String(string)
        }

        /// Creates a JsonValue from an array.
        pub fn from_array(array: JsonArray) -> Self {
            JsonValue::Array(array)
        }

        /// Creates a JsonValue representing null.
        pub fn json_null() -> Self {
            JsonValue::Null
        }

        /// Checks if the JsonValue is a number.
        pub fn is_number(&self) -> bool {
            matches!(self, JsonValue::Number(_))
        }

        /// Converts the JsonValue to a number. Panics if it's not a number.
        pub fn to_number(&self) -> f64 {
            match self {
                JsonValue::Number(n) => *n,
                _ => panic!("JsonValue is not a number"),
            }
        }

        /// Checks if the JsonValue is a boolean.
        pub fn is_bool(&self) -> bool {
            matches!(self, JsonValue::Bool(_))
        }

        /// Converts the JsonValue to a boolean. Panics if it's not a boolean.
        pub fn to_bool(&self) -> bool {
            match self {
                JsonValue::Bool(b) => *b,
                _ => panic!("JsonValue is not a boolean"),
            }
        }

        /// Checks if the JsonValue is a string.
        pub fn is_string(&self) -> bool {
            matches!(self, JsonValue::String(_))
        }

        /// Converts the JsonValue to a string. Panics if it's not a string.
        pub fn to_string(&self) -> &String {
            match self {
                JsonValue::String(s) => s,
                _ => panic!("JsonValue is not a string"),
            }
        }

        /// Checks if the JsonValue is an object.
        pub fn is_object(&self) -> bool {
            matches!(self, JsonValue::Object(_))
        }

        /// Converts the JsonValue to an object. Panics if it's not an object.
        pub fn to_object(&self) -> &JsonObject {
            match self {
                JsonValue::Object(o) => o,
                _ => panic!("JsonValue is not an object"),
            }
        }

        /// Converts the JsonValue to a mutable object. Panics if it's not an object.
        pub fn to_object_mut(&mut self) -> &mut JsonObject {
            match self {
                JsonValue::Object(o) => o,
                _ => panic!("JsonValue is not an object"),
            }
        }

        /// Checks if the JsonValue is an array.
        pub fn is_array(&self) -> bool {
            matches!(self, JsonValue::Array(_))
        }

        /// Converts the JsonValue to an array. Panics if it's not an array.
        pub fn to_array(&self) -> &JsonArray {
            match self {
                JsonValue::Array(a) => a,
                _ => panic!("JsonValue is not an array"),
            }
        }

        /// Converts the JsonValue to a mutable array. Panics if it's not an array.
        pub fn to_array_mut(&mut self) -> &mut JsonArray {
            match self {
                JsonValue::Array(a) => a,
                _ => panic!("JsonValue is not an array"),
            }
        }
    }

    /// Serializes a JsonValue to a string.
    pub fn serialize_to_string(value: &JsonValue) -> String {
        match value {
            JsonValue::Object(obj) => {
                let mut result = String::from("{");
                let mut first = true;
                for (key, value) in obj {
                    if !first {
                        result.push_str(",");
                    }
                    result.push_str(&format!("\"{}\":{}", key, serialize_to_string(value)));
                    first = false;
                }
                result.push_str("}");
                result
            }
            JsonValue::Array(arr) => {
                let mut result = String::from("[");
                let mut first = true;
                for value in arr {
                    if !first {
                        result.push_str(",");
                    }
                    result.push_str(&serialize_to_string(value));
                    first = false;
                }
                result.push_str("]");
                result
            }
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Null => String::from("null"),
        }
    }
}
// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ls {
    use serde_json::{Map, Value};
    use std::any::Any;

    pub trait JsonObject {
        fn object(&self) -> &Map<String, Value>;
        fn object_mut(&mut self) -> &mut Map<String, Value>;

        fn get_object<T: JsonObject + From<JsonObjectWrapper>>(&self, name: &str) -> T {
            let value = self.object().get(name).expect(&format!("Key '{}' not found", name));
            let object = value.as_object().expect(&format!("Value for key '{}' is not an object", name)).clone();
            T::from(JsonObjectWrapper { object })
        }

        fn get_array_property(&self, name: &str) -> &Vec<Value> {
            self.object().get(name).expect(&format!("Key '{}' not found", name))
                .as_array().expect(&format!("Value for key '{}' is not an array", name))
        }

        fn add_object_element_to_array_property(&mut self, name: &str) -> JsonObjectWrapper {
            let mut array = self.object_mut().get_mut(name)
                .expect(&format!("Key '{}' not found", name))
                .as_array_mut().expect(&format!("Value for key '{}' is not an array", name));
            let new_object = Value::Object(Map::new());
            array.push(new_object);
            let last_index = array.len() - 1;

            let object = array.get(last_index).unwrap().as_object().unwrap().clone();

            JsonObjectWrapper { object }

        }
    }

    pub struct JsonObjectWrapper {
        object: Map<String, Value>,
    }

    impl JsonObject for JsonObjectWrapper {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }
        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl From<JsonObjectWrapper> for JsonObjectWrapper {
        fn from(wrapper: JsonObjectWrapper) -> Self {
            wrapper
        }
    }

    macro_rules! json_string_accessors {
        ($name:ident) => {
            #[allow(dead_code)]
            pub fn $name(&self) -> String {
                self.object().get(stringify!($name)).expect(&format!("Key '{}' not found", stringify!($name))).as_str().unwrap().to_string()
            }

            #[allow(dead_code)]
            pub fn set_$name(&mut self, str: String) {
                self.object_mut().insert(stringify!($name).to_string(), Value::String(str));
            }

            #[allow(dead_code)]
            pub fn has_$name(&self) -> bool {
                self.object().contains_key(stringify!($name))
            }
        };
    }

    macro_rules! json_bool_accessors {
        ($name:ident) => {
            #[allow(dead_code)]
            pub fn $name(&self) -> bool {
                self.object().get(stringify!($name)).expect(&format!("Key '{}' not found", stringify!($name))).as_bool().unwrap()
            }

            #[allow(dead_code)]
            pub fn set_$name(&mut self, b: bool) {
                self.object_mut().insert(stringify!($name).to_string(), Value::Bool(b));
            }
        };
    }

    macro_rules! json_int_accessors {
        ($name:ident) => {
            #[allow(dead_code)]
            pub fn $name(&self) -> i64 {
                self.object().get(stringify!($name)).expect(&format!("Key '{}' not found", stringify!($name))).as_i64().unwrap()
            }

            #[allow(dead_code)]
            pub fn set_$name(&mut self, n: i64) {
                self.object_mut().insert(stringify!($name).to_string(), Value::Number(serde_json::Number::from(n)));
            }
        };
    }

    macro_rules! json_object_accessors {
        ($type:ty, $name:ident) => {
            #[allow(dead_code)]
            pub fn $name(&self) -> $type {
                self.get_object(stringify!($name))
            }
        };
    }

    macro_rules! json_dynamic_object_accessors {
        ($name:ident) => {
            #[allow(dead_code)]
            pub fn $name<T: JsonObject + From<JsonObjectWrapper>>(&self) -> T {
                self.get_object(stringify!($name))
            }
        };
    }

    macro_rules! json_array_object_accessors {
        ($type:ty, $name:ident) => {
            #[allow(dead_code)]
            pub fn add_$name(&mut self) -> $type {
                let new_element = self.add_object_element_to_array_property(stringify!($name));
                $type::from(new_element)
            }
            #[allow(dead_code)]
            pub fn $name##_size(&self) -> usize {
                self.get_array_property(stringify!($name)).len()
            }

            #[allow(dead_code)]
            pub fn $name(&self, idx: usize) -> $type {
                let array = self.get_array_property(stringify!($name));
                assert!(idx < array.len());
                let object = array[idx].as_object().unwrap().clone();
                $type::from(JsonObjectWrapper { object })
            }
        };
    }

    pub(crate) use json_string_accessors;
    pub(crate) use json_bool_accessors;
    pub(crate) use json_int_accessors;
    pub(crate) use json_object_accessors;
    pub(crate) use json_dynamic_object_accessors;
    pub(crate) use json_array_object_accessors;
}
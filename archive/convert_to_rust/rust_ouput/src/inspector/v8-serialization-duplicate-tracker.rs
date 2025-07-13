// Converted from V8 C++ source files:
// Header: v8-serialization-duplicate-tracker.h
// Implementation: v8-serialization-duplicate-tracker.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Runtime {
        pub struct DictionaryValue {
        }
        impl DictionaryValue {
            pub fn create() -> Box<DictionaryValue> {
                Box::new(DictionaryValue{})
            }
            pub fn set_string(&mut self, _name: &str, _value: String16) {}
            pub fn set_integer(&mut self, _name: &str, _value: i32) {}
            pub fn get_string(&mut self, _name: &str, _value: &mut String16) -> bool {
                false
            }
            pub fn get_integer(&mut self, _name: &str, _value: &mut i32) -> bool {
                false
            }
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct V8SerializationDuplicateTracker {
    m_context: v8::Local<v8::Context>,
    m_counter: i32,
    m_v8ObjectToSerializedDictionary: v8::Local<v8::Map>,
    serialized_values: Rc<RefCell<HashMap<usize, Box<protocol::Runtime::DictionaryValue>>>>
}

impl V8SerializationDuplicateTracker {
    pub fn link_existing_or_create(
        &mut self,
        v8_value: v8::Local<v8::Value>,
        is_known: &mut bool,
    ) -> Result<Box<protocol::Runtime::DictionaryValue>, String> {
        let mut result = protocol::Runtime::DictionaryValue::create();

        let maybe_known_serialized_value = self.find_known_serialized_value(v8_value);

        match maybe_known_serialized_value {
            None => {
                *is_known = false;
                // Keep reference to the serialized value, so that
                // `weakLocalObjectReference` can be set later.
                self.set_known_serialized_value(v8_value, result.as_mut())?;
                Ok(result)
            }
            Some(known_value) => {
                *is_known = true;

                let mut type_value = String16 {value: String::new()};
                known_value.borrow_mut().get_string("type", &mut type_value);
                result.set_string("type", type_value);

                let mut weak_local_object_reference: i32 = 0;
                // If `maybeKnownSerializedValue` has no `weakLocalObjectReference` yet,
                // it's need to be set.
                if !known_value.borrow_mut().get_integer("weakLocalObjectReference", &mut weak_local_object_reference) {
                    weak_local_object_reference = self.m_counter;
                    self.m_counter += 1;
                    known_value.borrow_mut().set_integer("weakLocalObjectReference", weak_local_object_reference);
                }
                result.set_integer("weakLocalObjectReference", weak_local_object_reference);

                Ok(result)
            }
        }
    }

    fn set_known_serialized_value(
        &mut self,
        v8_value: v8::Local<v8::Value>,
        serialized_value: *mut protocol::Runtime::DictionaryValue,
    ) -> Result<(), String> {
        unsafe {
          let serialized_value_ref = serialized_value.as_mut().unwrap();
          let key = v8_value.value(); // Assuming v8::Value has a method value() that returns a unique identifier
          let mut map = self.m_v8ObjectToSerializedDictionary.clone();
          let external = v8::External::New(self.m_context.GetIsolate(), serialized_value_ref as *mut _ as *mut std::ffi::c_void);
          map = map.Set(self.m_context.clone(), v8_value.clone(), external.clone()).ToLocalChecked();
          self.m_v8ObjectToSerializedDictionary = map;
          Ok(())
        }
    }

    fn find_known_serialized_value(
        &self,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Rc<RefCell<Box<protocol::Runtime::DictionaryValue>>>> {
      let isolate = self.m_context.GetIsolate();
      let scope = v8::EscapableHandleScope::new(isolate);
      let undefined = v8::Undefined(isolate);

      let known_value = self.m_v8ObjectToSerializedDictionary.Get(self.m_context.clone(), v8_value.clone());
      if known_value.ToLocalChecked().IsUndefined() {
          return None;
      }

      let external_value = known_value.ToLocalChecked().As::<v8::External>();
      if external_value.IsEmpty() {
          return None;
      }

      let value_ptr = external_value.Value();
      if value_ptr.is_null() {
          return None;
      }

      unsafe {
          let dict_value_ptr = value_ptr as *mut protocol::Runtime::DictionaryValue;
          if dict_value_ptr.is_null() {
              return None;
          }

        Some(Rc::new(RefCell::new(Box::from_raw(dict_value_ptr))))
      }
    }

    pub fn new(context: v8::Local<v8::Context>) -> V8SerializationDuplicateTracker {
        V8SerializationDuplicateTracker {
            m_context: context.clone(),
            m_counter: 1,
            m_v8ObjectToSerializedDictionary: v8::Map::New(context.GetIsolate()),
            serialized_values: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

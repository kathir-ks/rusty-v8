// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-forin.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::rc::Rc;

  pub use crate::execution::isolate::Isolate;
  pub use crate::heap::factory::Factory;
  //pub use crate::heap::heap::Heap;
  pub use crate::objects::keys::KeyCollectionMode;
  pub use crate::objects::keys::FastKeyAccumulator;
  pub use crate::objects::module::IsJSModuleNamespace;
  pub use crate::objects::objects::JSObject;
  pub use crate::runtime::runtime::RUNTIME_FUNCTION;
  pub use crate::runtime::runtime::args;
  pub use crate::objects::objects::PropertyKey;
  pub use crate::codegen::code_stub_assembler::JSProxy;
  pub use crate::execution::isolate::LookupIterator;
  pub use crate::objects::objects::PropertyAttributes;
  pub use crate::objects::objects::AccessorInfo;
  pub use crate::objects::objects::WasmExportedFunction;
  pub use crate::objects::objects::DescriptorArray;
  pub use crate::objects::objects::InternalIndex;
  pub use crate::objects::objects::FieldIndex;
  pub use crate::objects::objects::MaybeObject;
  pub use crate::objects::objects::Name;
  pub use crate::objects::objects::String;
  pub use crate::objects::objects::HeapObject;
  pub use crate::objects::objects::JSReceiver;
  pub use crate::objects::objects::Smi;
  pub use crate::objects::objects::FixedArray;
  pub use crate::objects::map::Map;
  pub use crate::objects::objects::Object;
  pub use crate::execution::isolate::MaybeDirectHandle;
  pub use crate::execution::isolate::DirectHandle;
  pub use crate::runtime::runtime_wasm::HandleScope;
  pub use crate::runtime::runtime_wasm::Local;
  pub use crate::runtime::runtime_wasm::Code;
  pub use crate::runtime::runtime_wasm::This;
  pub use crate::runtime::runtime_wasm::V8;
  pub use crate::runtime::runtime_wasm::RUNTIME_FUNCTION;

  #[derive(Debug)]
  pub enum Error {
    Exception,
    Abort,
    TypeError(String),
  }

  #[allow(dead_code)]
  enum GetKeysConversion {
    kConvertToString,
    kNoNumbers,
  }

  #[allow(dead_code)]
  enum StartAtReceiver {
    kStartAtReceiver,
  }

  #[allow(dead_code)]
  enum EnumerableStrings {
    ENUMERABLE_STRINGS,
  }

  #[allow(dead_code)]
  enum Absent {
    ABSENT,
  }

  #[allow(dead_code)]
  enum DontEnum {
    DONT_ENUM,
  }

  #[allow(dead_code)]
  enum MessageTemplate {
    kWasmObjectsAreOpaque,
  }

  fn enumerate(
    isolate: &mut Isolate,
    receiver: DirectHandle<JSReceiver>,
  ) -> Result<MaybeDirectHandle<HeapObject>, Error> {
    JSObject::MakePrototypesFast(receiver, StartAtReceiver::kStartAtReceiver, isolate);
    let mut accumulator = FastKeyAccumulator::new(
      isolate,
      receiver,
      KeyCollectionMode::kIncludePrototypes,
      EnumerableStrings::ENUMERABLE_STRINGS,
      true,
    );

    if !accumulator.is_receiver_simple_enum() {
      let keys = accumulator.GetKeys(if accumulator.may_have_elements() {
        GetKeysConversion::kConvertToString
      } else {
        GetKeysConversion::kNoNumbers
      })?;

      if !accumulator.is_receiver_simple_enum() {
        return Ok(MaybeDirectHandle::new(keys));
      }
    }

    assert!(!IsJSModuleNamespace(*receiver));
    Ok(MaybeDirectHandle::from_map(direct_handle(receiver.get().map(), isolate)))
  }

  fn has_enumerable_property(
    isolate: &mut Isolate,
    receiver: DirectHandle<JSReceiver>,
    key: DirectHandle<Object>,
  ) -> Result<MaybeDirectHandle<Object>, Error> {
    let mut success = false;
    let mut result = Absent::ABSENT;

    let lookup_key = PropertyKey::new(isolate, key, &mut success);
    if !success {
      return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
    }

    let mut it = LookupIterator::new(isolate, receiver, lookup_key);

    loop {
      it.Next();

      match it.state() {
        LookupIterator::TRANSITION => unreachable!(),
        LookupIterator::JSPROXY => {
          let res = JSProxy::GetPropertyAttributes(&it);
          if res.is_nothing() {
            return Err(Error::Exception);
          }

          if let Some(attr) = res.value() {
            if attr == Absent::ABSENT as i32 {
              let proxy = it.GetHolder::<JSProxy>();
              let prototype = JSProxy::GetPrototype(proxy)?;
              if prototype.is_null(isolate) {
                return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
              }

              return has_enumerable_property(
                isolate,
                DirectHandle::unchecked_cast(prototype),
                key,
              );
            } else if (attr as i32) & (DontEnum::DONT_ENUM as i32) != 0 {
              return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
            } else {
              return Ok(MaybeDirectHandle::new(it.GetName()));
            }
          } else {
            return Err(Error::Exception);
          }
        }
        LookupIterator::WASM_OBJECT => {
          return Err(Error::TypeError(
            "Wasm objects are opaque".to_string(),
          ));
        }
        LookupIterator::INTERCEPTOR => {
          let res = JSObject::GetPropertyAttributesWithInterceptor(&it);
          if res.is_nothing() {
            return Err(Error::Exception);
          }
          if let Some(attr) = res.value() {
            if attr != Absent::ABSENT as i32 {
              return Ok(MaybeDirectHandle::new(it.GetName()));
            }
          }
        }
        LookupIterator::ACCESS_CHECK => {
          if it.HasAccess() {
            continue;
          }
          let res = JSObject::GetPropertyAttributesWithFailedAccessCheck(&it);
          if res.is_nothing() {
            return Err(Error::Exception);
          }
          if let Some(attr) = res.value() {
            if attr != Absent::ABSENT as i32 {
              return Ok(MaybeDirectHandle::new(it.GetName()));
            }
          }
          return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
        }
        LookupIterator::TYPED_ARRAY_INDEX_NOT_FOUND => {
          return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
        }
        LookupIterator::ACCESSOR => {
          if IsJSModuleNamespace(*DirectHandle::unchecked_cast(it.GetHolder::<Object>())) {
            let res = JSModuleNamespace::GetPropertyAttributes(&it);
            if res.is_nothing() {
              return Err(Error::Exception);
            }
            if let Some(attr) = res.value() {
              assert_eq!(0, (attr as i32) & (DontEnum::DONT_ENUM as i32));
            }
          }
          return Ok(MaybeDirectHandle::new(it.GetName()));
        }
        LookupIterator::DATA => {
          return Ok(MaybeDirectHandle::new(it.GetName()));
        }
        LookupIterator::NOT_FOUND => {
          return Ok(MaybeDirectHandle::new(isolate.factory().undefined_value()));
        }
      }
      unreachable!()
    }
  }

  impl From<String> for Error {
    fn from(s: String) -> Self {
      Error::TypeError(s)
    }
  }

  impl From<&str> for Error {
    fn from(s: &str) -> Self {
      Error::TypeError(s.to_string())
    }
  }

  impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
      panic!("cannot convert from Infallible")
    }
  }

  pub struct RuntimeForIn {}

  impl RuntimeForIn {
    pub fn runtime_for_in_enumerate(
      isolate: &mut Isolate,
      args: &args,
    ) -> Result<HeapObject, Error> {
      let receiver = args.at::<JSReceiver>(0);
      match enumerate(isolate, receiver) {
        Ok(maybe_handle) => match maybe_handle.handle() {
          Some(handle) => Ok(handle.get().clone()),
          None => Err(Error::Abort),
        },
        Err(e) => Err(e),
      }
    }

    pub fn runtime_for_in_has_property(
      isolate: &mut Isolate,
      args: &args,
    ) -> Result<bool, Error> {
      let receiver = args.at::<JSReceiver>(0);
      let key = args.at::<Object>(1);
      let result = has_enumerable_property(isolate, receiver, key)?;
      Ok(!result.handle().map_or(true, |r| r.get().is_undefined(isolate)))
    }
  }

  RUNTIME_FUNCTION!(Runtime_ForInEnumerate);

  impl Runtime_ForInEnumerate {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn call(
      isolate: &mut Isolate,
      args: &args,
    ) -> *mut v8::Value {
      let scope = HandleScope::new(isolate);
      match RuntimeForIn::runtime_for_in_enumerate(isolate, args) {
        Ok(result) => {
          let handle: Local<'static, Object> = Local::from(result);
          *handle
        }
        Err(_e) => {
          //println!("Error: {:?}", e);
          //isolate.ThrowException(String::NewFromUtf8(isolate, "error".to_string().as_bytes(), NewStringType::kNormal).into());
          // isolate.null_value().into()
          let undefined: Local<'static, Object> = Local::from(isolate.undefined_value());
          *undefined
        }
      }
    }
  }

  RUNTIME_FUNCTION!(Runtime_ForInHasProperty);

  impl Runtime_ForInHasProperty {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn call(
      isolate: &mut Isolate,
      args: &args,
    ) -> *mut v8::Value {
      let scope = HandleScope::new(isolate);
      match RuntimeForIn::runtime_for_in_has_property(isolate, args) {
        Ok(result) => {
          let handle: Local<'static, Object> = Local::from(isolate.heap().to_boolean(result));
          *handle
        }
        Err(_e) => {
          let undefined: Local<'static, Object> = Local::from(isolate.undefined_value());
          *undefined
        }
      }
    }
  }
}

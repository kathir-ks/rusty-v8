// Converted from V8 C++ source files:
// Header: v8-function-callback.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use crate::Persistent;
use crate::Local;
use crate::Global;
use crate::Isolate;
use crate::Object;
use crate::Value;
use crate::Boolean;
use crate::Integer;
use crate::Number;
use crate::Primitive;
use crate::String;
use std::os::raw::c_char;

pub struct V8 {}

pub struct FunctionCallbackInfo<T> {
    implicit_args_: *mut internal::Address,
    values_: *mut internal::Address,
    length_: usize,
    _phantom: PhantomData<T>,
}

pub struct PropertyCallbackInfo<T> {
    args_: [internal::Address; 8],
    _phantom: PhantomData<T>,
}

pub type FunctionCallback = fn(info: &FunctionCallbackInfo<Value>);

pub struct ReturnValue<T> {
    value_: *mut internal::Address,
    _phantom: PhantomData<T>,
}

mod internal {
    pub type Address = usize;

    pub const kApiSystemPointerSize: usize = 8;

    pub struct Internals {}
    impl Internals {
        pub fn IsValidSmi(_i: i32) -> bool {
            true
        }
        pub fn IntegralToSmi(i: i64) -> Address {
            i as Address
        }
        pub fn StaticReadOnlyRoot::kTrueValue() -> Address { 1 }
        pub fn StaticReadOnlyRoot::kFalseValue() -> Address { 0 }
        pub fn StaticReadOnlyRoot::kNullValue() -> Address { 2 }
        pub fn StaticReadOnlyRoot::kUndefinedValue() -> Address { 3 }
        pub fn StaticReadOnlyRoot::kEmptyString() -> Address { 4 }

        pub const kTrueValueRootIndex: usize = 0;
        pub const kFalseValueRootIndex: usize = 1;
        pub const kNullValueRootIndex: usize = 2;
        pub const kUndefinedValueRootIndex: usize = 3;
        pub const kEmptyStringRootIndex: usize = 4;

        pub fn GetRoot(_isolate: *mut crate::Isolate, index: usize) -> Address {
            index as Address
        }
        pub fn TryIntegralToSmi(i: i64) -> Option<Address> {
            if i >= -1073741824 && i <= 1073741823 {
                Some(i as Address)
            } else {
                None
            }
        }
         pub fn CompressTagged(value: Address) -> Address {
            value
        }
        pub fn DecompressTaggedField(old_value: Address, new_value: Address) -> Address {
            new_value
        }
         pub fn HasHeapObjectTag(_address: Address) -> bool {
            true
        }
        pub const kInferShouldThrowMode: i32 = 0;
        pub const kDontThrow: i32 = 1;
    }
}

mod api_internal {
    use super::*;
    pub fn GetFunctionTemplateData(
        _isolate: *mut Isolate,
        _raw_target: Local<Value>,
    ) -> Local<Value> {
        Local::empty()
    }
     pub fn ConvertToJSGlobalProxyIfNecessary(holder: internal::Address) -> internal::Address {
        holder
    }
}
impl<T> ReturnValue<T> {
    #[inline]
    pub fn new(slot: *mut internal::Address) -> Self {
        ReturnValue {
            value_: slot,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn set_internal(&mut self, value: internal::Address) {
        unsafe {
            *self.value_ = value;
        }
    }

    #[inline]
    pub fn set<S>(&mut self, handle: &Global<S>)
    where
        T: 'static,
        S: 'static,
    {
        if handle.is_empty() {
            self.set_default_value();
        } else {
            self.set_internal(handle.ptr() as internal::Address);
        }
    }

    #[inline]
    pub fn set_non_empty<S>(&mut self, handle: &Global<S>)
    where
        T: 'static,
        S: 'static,
    {
        self.set_internal(handle.ptr() as internal::Address);
    }

    #[inline]
    pub fn set_bt<S>(&mut self, handle: &crate::BasicTracedReference<S>)
    where
        T: 'static,
        S: 'static,
    {
         if handle.is_empty() {
            self.set_default_value();
        } else {
            self.set_internal(handle.ptr() as internal::Address);
        }
    }

    #[inline]
    pub fn set_bt_non_empty<S>(&mut self, handle: &crate::BasicTracedReference<S>)
    where
        T: 'static,
        S: 'static,
    {
        self.set_internal(handle.ptr() as internal::Address);
    }


    #[inline]
    pub fn set_local<S>(&mut self, handle: Local<S>)
    where
        T: 'static,
        S: 'static,
    {
        if handle.is_empty() {
            self.set_default_value();
        } else {
            self.set_internal(handle.ptr() as internal::Address);
        }
    }

     #[inline]
    pub fn set_local_non_empty<S>(&mut self, handle: Local<S>)
    where
        T: 'static,
        S: 'static,
    {
            self.set_internal(handle.ptr() as internal::Address);
    }

    #[inline]
    pub fn set_bool(&mut self, value: bool)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(if value {
            internal::Internals::StaticReadOnlyRoot::kTrueValue()
        } else {
            internal::Internals::StaticReadOnlyRoot::kFalseValue()
        });
    }

    #[inline]
    pub fn set_double(&mut self, i: f64)
    where
        T: 'static,
    {
        let isolate = self.get_isolate();
        let number = Number::new(isolate, i);
        self.set_local_non_empty(number);
    }

    #[inline]
    pub fn set_i16(&mut self, i: i16)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(internal::Internals::IntegralToSmi(i as i64));
    }

    #[inline]
    pub fn set_i32(&mut self, i: i32)
    where
        T: 'static,
    {
         let result = internal::Internals::TryIntegralToSmi(i as i64);
        match result {
            Some(smi) => {
                self.set_internal(smi);
            }
            None => {
                let isolate = self.get_isolate();
                let integer = Integer::new(isolate, i);
                self.set_local_non_empty(integer);
            }
        }
    }

    #[inline]
    pub fn set_i64(&mut self, i: i64)
    where
        T: 'static,
    {
        let result = internal::Internals::TryIntegralToSmi(i);
        match result {
            Some(smi) => {
                self.set_internal(smi);
            }
            None => {
                let isolate = self.get_isolate();
                let number = Number::new(isolate, i as f64);
                self.set_local_non_empty(number);
            }
        }
    }

    #[inline]
    pub fn set_u16(&mut self, i: u16)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(internal::Internals::IntegralToSmi(i as i64));
    }

    #[inline]
    pub fn set_u32(&mut self, i: u32)
    where
        T: 'static,
    {
         let result = internal::Internals::TryIntegralToSmi(i as i64);
        match result {
            Some(smi) => {
                self.set_internal(smi);
            }
            None => {
                let isolate = self.get_isolate();
                let integer = Integer::new_from_unsigned(isolate, i);
                self.set_local_non_empty(integer);
            }
        }
    }

    #[inline]
    pub fn set_u64(&mut self, i: u64)
    where
        T: 'static,
    {
        let result = internal::Internals::TryIntegralToSmi(i as i64);
        match result {
            Some(smi) => {
                self.set_internal(smi);
            }
            None => {
                let isolate = self.get_isolate();
                let number = Number::new(isolate, i as f64);
                self.set_local_non_empty(number);
            }
        }
    }

    #[inline]
    pub fn set_null(&mut self)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(internal::Internals::StaticReadOnlyRoot::kNullValue());
    }

    #[inline]
    pub fn set_undefined(&mut self)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(internal::Internals::StaticReadOnlyRoot::kUndefinedValue());
    }

    #[inline]
    pub fn set_false(&mut self)
    where
        T: 'static,
    {
         let i = internal::Internals{};
        self.set_internal(internal::Internals::StaticReadOnlyRoot::kFalseValue());
    }

    #[inline]
    pub fn set_empty_string(&mut self)
    where
        T: 'static,
    {
        let i = internal::Internals{};
        self.set_internal(internal::Internals::StaticReadOnlyRoot::kEmptyString());
    }

    #[inline]
    pub fn get_isolate(&self) -> *mut Isolate {
        unsafe {
            *(&self.value_.add(-2) as *const *mut Isolate)
        }
    }

    #[inline]
    pub fn get(&self) -> Local<Value> {
        let isolate = self.get_isolate();
        Local::new_from_slot(isolate, self.value_)
    }

    #[inline]
    fn set_default_value(&mut self)
    where
        T: 'static,
    {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<void>()
            || std::any::TypeId::of::<T>() == std::any::TypeId::of::<Boolean>()
        {
            self.set_bool(true);
        } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Integer>() {
            let i = internal::Internals{};
            self.set_internal(internal::Internals::IntegralToSmi(0));
        } else {
            let i = internal::Internals{};
            self.set_internal(internal::Internals::StaticReadOnlyRoot::kUndefinedValue());
        }
    }

}

impl<T> FunctionCallbackInfo<T> {
    #[inline]
    pub fn new(
        implicit_args: *mut internal::Address,
        values: *mut internal::Address,
        length: usize,
    ) -> Self {
        FunctionCallbackInfo {
            implicit_args_: implicit_args,
            values_: values,
            length_: length,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn length(&self) -> i32 {
        self.length_ as i32
    }

    #[inline]
    pub fn get(&self, i: i32) -> Local<Value> {
        if i < 0 || self.length() <= i {
            return Local::undefined(self.get_isolate());
        }
        Local::new_from_slot(self.get_isolate(), unsafe { self.values_.add(i as usize) })
    }

    #[inline]
    pub fn this(&self) -> Local<Object> {
        Local::new_from_slot(self.get_isolate(), unsafe { self.values_.add(-1) })
    }

    #[inline]
    pub fn new_target(&self) -> Local<Value> {
        Local::new_from_slot(self.get_isolate(), unsafe { self.implicit_args_.add(5) })
    }

    #[inline]
    pub fn data(&self) -> Local<Value> {
        let target = Local::<Value>::new_from_slot(self.get_isolate(), unsafe { self.implicit_args_.add(4) });
        api_internal::GetFunctionTemplateData(self.get_isolate(), target)
    }

    #[inline]
    pub fn get_isolate(&self) -> *mut Isolate {
        unsafe {
            *(&self.implicit_args_.add(1) as *const *mut Isolate)
        }
    }

    #[inline]
    pub fn get_return_value(&self) -> ReturnValue<T> {
        ReturnValue::new(unsafe { self.implicit_args_.add(3) })
    }

    #[inline]
    pub fn is_construct_call(&self) -> bool {
        !self.new_target().is_undefined()
    }
}

impl<T> PropertyCallbackInfo<T> {
    #[inline]
    pub fn get_isolate(&self) -> *mut Isolate {
        unsafe {
            *(&self.args_[3] as *const *mut Isolate)
        }
    }

    #[inline]
    pub fn data(&self) -> Local<Value> {
        Local::new_from_slot(self.get_isolate(), &mut self.args_[6])
    }

    #[inline]
    pub fn this(&self) -> Local<Object> {
        Local::new_from_slot(self.get_isolate(), &mut self.args_[7])
    }

    #[inline]
    pub fn holder(&self) -> Local<Object> {
        Local::new_from_slot(self.get_isolate(), &mut self.args_[2])
    }

    #[inline]
    pub fn holder_v2(&self) -> Local<Object> {
        let i = internal::Internals{};
         Local::new_from_slot(self.get_isolate(), &mut self.args_[4])
    }

    #[inline]
    pub fn get_return_value(&self) -> ReturnValue<T> {
        ReturnValue::new(&mut self.args_[5])
    }

    #[inline]
    pub fn should_throw_on_error(&self) -> bool {
        let i = internal::Internals{};
         if self.args_[1] != internal::Internals::IntegralToSmi(internal::Internals::kInferShouldThrowMode) {
            return self.args_[1] != internal::Internals::IntegralToSmi(internal::Internals::kDontThrow);
        }
        unsafe {
            crate::internal::ShouldThrowOnError(self.get_isolate() as *mut v8::internal::Isolate)
        }
    }
}

mod v8 {
    pub(crate) mod internal {
        pub struct Isolate {}
    }
}
pub mod internal {
    pub unsafe fn ShouldThrowOnError(_isolate: *mut v8::internal::Isolate) -> bool {
        true
    }
}
pub struct BasicTracedReference<T> {
    _phantom: PhantomData<T>,
    is_empty: bool,
    ptr: usize,
}
impl<T> BasicTracedReference<T> {
     #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
    #[inline]
    pub fn ptr(&self) -> usize {
        self.ptr
    }
}
impl<T> Default for BasicTracedReference<T> {
    fn default() -> Self {
        Self {
             _phantom: PhantomData,
            is_empty: true,
            ptr: 0,
        }
    }
}

impl<T> From<Local<'static, T>> for BasicTracedReference<T>
where T: 'static{
    fn from(local: Local<'static, T>) -> Self {
        Self {
            _phantom: PhantomData,
            is_empty: local.is_empty(),
            ptr: local.ptr() as usize,
        }
    }
}

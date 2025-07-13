// Converted from V8 C++ source files:
// Header: option-utils.h
// Implementation: option-utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{vec, string};
use std::convert::TryInto;
use std::ptr;
use std::ffi::CString;
use std::mem::MaybeUninit;

// use crate::common::globals::; // Assuming globals.h functionality is needed
// use crate::execution::isolate::Isolate; // Assuming Isolate struct is needed
// use crate::objects::js_objects::JSReceiver; // Assuming JSReceiver struct is needed
// use crate::objects::string::String; // Assuming String struct is needed

// Mock implementations
pub struct Isolate {}
pub struct JSReceiver {}
pub struct String {}
pub struct Object {}
pub struct Number {}
pub struct Factory {}

impl Isolate {
    pub fn factory(&mut self) -> &mut Factory {
        todo!()
    }
}

impl Object {
    pub fn ToNumber(_isolate : &mut Isolate, _obj : DirectHandle<Object>) -> Result<DirectHandle<Number>, Error> {
      Ok(DirectHandle{obj : Object{}})
    }
    pub fn NumberValue(&self) -> f64{
      0.0
    }
    pub fn BooleanValue(_object : Object, _isolate : &mut Isolate) -> bool{
      true
    }
    pub fn GetPropertyOrElement(_isolate: &mut Isolate, _options: DirectHandle<JSReceiver>, _property_str: DirectHandle<String>) -> Result<DirectHandle<Object>, Error> {
      Ok(DirectHandle{obj : Object{}})
    }
    pub fn ToObject(_isolate : &mut Isolate, _options: DirectHandle<Object>, _method_name : &str) -> Result<DirectHandle<Object>, Error> {
      Ok(DirectHandle{obj : Object{}})
    }
    pub fn ToString(_isolate : &mut Isolate, _obj : DirectHandle<Object>) -> Result<DirectHandle<String>, Error> {
      Ok(DirectHandle{obj : String{}})
    }
}
impl Number{
    pub fn NumberValue(&self) -> f64 {
        0.0
    }
}

impl Factory {
    pub fn NewStringFromAsciiChecked(&mut self, _str: &str) -> DirectHandle<String> {
        DirectHandle{obj : String{}}
    }
    pub fn NewJSObjectWithNullProto(&mut self) -> Result<DirectHandle<JSReceiver>, Error> {
      Ok(DirectHandle{obj : JSReceiver{}})
    }
    pub fn true_string(&self) -> DirectHandle<String> {
      DirectHandle{obj : String{}}
    }
    pub fn false_string(&self) -> DirectHandle<String> {
      DirectHandle{obj : String{}}
    }
}

#[derive(Debug)]
pub enum Error {
    TypeError,
    RangeError,
}

#[derive(Debug)]
pub struct DirectHandle<T> {
    obj: T,
}

impl <T> DirectHandle<T> {
  pub fn cast<U>(&self) -> &DirectHandle<U> {
      unsafe {
          &*(self as *const DirectHandle<T> as *const DirectHandle<U>)
      }
  }
}

impl DirectHandle<Object> {
  pub fn cast<U>(&self) -> &DirectHandle<U> {
      unsafe {
          &*(self as *const DirectHandle<Object> as *const DirectHandle<U>)
      }
  }
}

pub type MaybeDirectHandle<T> = Result<DirectHandle<T>, Error>;

pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    fn from_just(self) -> T {
        match self {
            Maybe::Just(t) => t,
            Maybe::Nothing => panic!("Called from_just on a Nothing"),
        }
    }
}

fn IsUndefined<T>(_obj: T, _isolate: &mut Isolate) -> bool {
    false
}

fn IsJSReceiver<T>(_obj: T) -> bool {
    true
}
fn IsNumber<T>(_obj: T) -> bool {
    true
}
fn IsNaN<T>(_obj: T) -> bool {
    false
}
fn IsTrue<T>(_obj: T, _isolate : &mut Isolate) -> bool {
    false
}

// Mock MessageTemplate
pub enum MessageTemplate {
    kInvalidArgument,
    kValueOutOfRange,
    kPropertyValueOutOfRange,
}

fn NewTypeError(_isolate: &mut Isolate, _template: MessageTemplate) -> Error {
    Error::TypeError
}

fn NewRangeError(_template: MessageTemplate, _value : DirectHandle<Object>, _method_str : DirectHandle<String>, _property_str : DirectHandle<String>) -> Error {
    Error::RangeError
}

fn THROW_NEW_ERROR<T>(_isolate: &mut Isolate, error: Error) -> Result<T, Error> {
    Err(error)
}

fn THROW_NEW_ERROR_RETURN_VALUE<T>(_isolate: &mut Isolate, error: Error, _nothing: Maybe<T>) -> Result<T, Error> {
    Err(error)
}

macro_rules! ASSIGN_RETURN_ON_EXCEPTION {
    ($isolate:expr, $var:ident, $expr:expr, $method_name:expr) => {
        let result = $expr;
        match result {
            Ok(value) => {
                $var = value;
            }
            Err(err) => {
                return Err(err);
            }
        }
    };
}

macro_rules! ASSIGN_RETURN_ON_EXCEPTION_VALUE {
    ($isolate:expr, $var:ident, $expr:expr, $value:expr) => {
        let result = $expr;
        match result {
            Ok(value) => {
                $var = value;
            }
            Err(err) => {
                return $value;
            }
        }
    };
}

fn FastD2I(d : f64) -> i32 {
  d as i32
}

fn floor(d : f64) -> f64 {
  d.floor()
}
impl JSReceiver{
  pub fn GetProperty(_isolate: &mut Isolate, _options: DirectHandle<JSReceiver>, _property: DirectHandle<String>) -> Result<DirectHandle<Object>, Error> {
    Ok(DirectHandle{obj : Object{}})
  }
}

impl String {
  pub fn ToCString(&self) -> std::unique_ptr<[i8]> {
      let s = "test".to_string();
      let cs = CString::new(s).unwrap();
      let ptr = cs.as_ptr() as *mut i8;
      let len = cs.as_bytes().len();
      unsafe {
          let slice = std::slice::from_raw_parts_mut(ptr, len + 1);
          std::unique_ptr::from_raw(slice as *mut [i8])
      }
  }
  pub fn Equals(_isolate : &mut Isolate, _str1 : DirectHandle<String>, _str2 : DirectHandle<String>) -> bool {
    true
  }
  pub fn Flatten(_isolate : &mut Isolate, _str : DirectHandle<String>) -> DirectHandle<String> {
    DirectHandle{obj : String{}}
  }
  pub fn length(&self) -> i32 {
    0
  }
  pub fn GetFlatContent(&self, _no_gc : DisallowGarbageCollection) -> FlatContent {
    FlatContent{dummy : 0}
  }
}

pub struct DisallowGarbageCollection {}

pub struct FlatContent {
  dummy : i32
}

impl FlatContent {
  pub fn IsOneByte(&self) -> bool{
    true
  }
  pub fn ToOneByteVector(&self) -> OneByteVector {
    OneByteVector{}
  }
  pub fn ToUC16Vector(&self) -> UC16Vector {
    UC16Vector{}
  }
}

pub struct OneByteVector {}
impl OneByteVector{
  pub fn begin(&self) -> *const u8 {
    ptr::null()
  }
}

pub struct UC16Vector {}
impl UC16Vector{
  pub fn begin(&self) -> *const u16 {
    ptr::null()
  }
}

//mod std {
  extern "C" {
    #[link_name = "memcmp"]
    fn memcmp_impl(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
  }

  // pub unsafe fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 {
  //   memcmp_impl(s1 as *const c_char, s2 as *const c_char, n)
  // }
//}

fn CompareCharsEqual(str1 : &str, str2 : *const u8, length : i32) -> bool{
  let str1_bytes = str1.as_bytes();
  if str1_bytes.len() != length as usize {
      return false;
  }
  let str2_bytes = unsafe { std::slice::from_raw_parts(str2, length as usize) };
  str1_bytes == str2_bytes
}

fn CompareCharsEqual(str1 : &str, str2 : *const u16, length : i32) -> bool{
  //TODO(kathy): fix this
  true
}

pub mod internal {
// ecma402/#sec-getoptionsobject and temporal/#sec-getoptionsobject
pub fn GetOptionsObject(
    isolate: &mut Isolate,
    options: DirectHandle<Object>,
    method_name: &str,
) -> MaybeDirectHandle<JSReceiver> {
    // 1. If options is undefined, then
    if IsUndefined(options, isolate) {
        // a. Return ! ObjectCreate(null).
        return isolate.factory().NewJSObjectWithNullProto();
    }
    // 2. If Type(options) is Object, then
    if IsJSReceiver(options) {
        // a. Return options.
        return Ok(DirectHandle{obj : unsafe { std::mem::transmute_copy(&options) }}); //Cast::<JSReceiver>(&options);
    }
    // 3. Throw a TypeError exception.
    THROW_NEW_ERROR(isolate, NewTypeError(MessageTemplate::kInvalidArgument))
}

// ecma402/#sec-coerceoptionstoobject
pub fn CoerceOptionsToObject(
    isolate: &mut Isolate,
    options: DirectHandle<Object>,
    method_name: &str,
) -> MaybeDirectHandle<JSReceiver> {
    // 1. If options is undefined, then
    if IsUndefined(options, isolate) {
        // a. Return ! ObjectCreate(null).
        return isolate.factory().NewJSObjectWithNullProto();
    }
    // 2. Return ? ToObject(options).
    ASSIGN_RETURN_ON_EXCEPTION!(isolate, options, Object::ToObject(isolate, options, method_name), method_name);
    return Ok(DirectHandle{obj : unsafe { std::mem::transmute_copy(&options) }}); //Cast::<JSReceiver>(&options);
}

// ECMA402 9.2.10. GetOption( options, property, type, values, fallback)
// ecma402/#sec-getoption and temporal/#sec-getoption
//
// This is specialized for the case when type is string.
//
// Instead of passing undefined for the values argument as the spec
// defines, pass in an empty vector.
//
// Returns true if options object has the property and stores the
// result in value. Returns false if the value is not found. The
// caller is required to use fallback value appropriately in this
// case.
//
// method_name is a string denoting the method the call from; used when
// printing the error message.
pub fn GetStringOption(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    property: &str,
    values: &Vec<&str>,
    method_name: &str,
    result: &mut std::unique_ptr<[i8]>,
) -> Maybe<bool> {
    let property_str = isolate.factory().NewStringFromAsciiChecked(property);

    // 1. Let value be ? Get(options, property).
    let value: DirectHandle<Object>;
    match Object::GetPropertyOrElement(isolate, options, property_str) {
        Ok(v) => value = v,
        Err(_e) => return Maybe::Nothing,
    }

    if IsUndefined(value, isolate) {
        return Maybe::Just(false);
    }

    // 2. c. Let value be ? ToString(value).
    let value_str: DirectHandle<String>;
    match Object::ToString(isolate, value) {
        Ok(v) => value_str = v,
        Err(_e) => return Maybe::Nothing,
    }

    let value_cstr = value_str.ToCString();

    // 2. d. if values is not undefined, then
    if !values.is_empty() {
        // 2. d. i. If values does not contain an element equal to value,
        // throw a RangeError exception.
        for i in 0..values.len() {
            let c_str = unsafe { CStr::from_ptr(value_cstr.as_ptr() as *const i8).to_str().unwrap() };
            if values[i] == c_str {
                // 2. e. return value
                *result = value_cstr;
                return Maybe::Just(true);
            }
        }

        let method_str = isolate.factory().NewStringFromAsciiChecked(method_name);
        THROW_NEW_ERROR_RETURN_VALUE(
            isolate,
            NewRangeError(MessageTemplate::kValueOutOfRange, value, method_str, property_str),
            Maybe::Nothing,
        );
    }

    // 2. e. return value
    *result = value_cstr;
    return Maybe::Just(true);
}

// A helper template to get string from option into a enum.
// The enum in the enum_values is the corresponding value to the strings
// in the str_values. If the option does not contains name,
// default_value will be return.
pub fn GetStringOption1<T: Copy>(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    name: &str,
    method_name: &str,
    str_values: &Vec<&str>,
    enum_values: &Vec<T>,
    default_value: T,
) -> Maybe<T> {
    assert_eq!(str_values.len(), enum_values.len());
    let mut cstr: std::unique_ptr<[i8]> = std::unique_ptr::new();
    let found = GetStringOption(isolate, options, name, str_values, method_name, &mut cstr);
    match found {
        Maybe::Just(b) => {
            if b {
                //assert!(cstr.get() != ptr::null_mut());
                for i in 0..str_values.len() {
                    let c_str = unsafe { CStr::from_ptr(cstr.as_ptr() as *const i8).to_str().unwrap() };
                    if c_str == str_values[i] {
                        return Maybe::Just(enum_values[i]);
                    }
                }
                panic!("UNREACHABLE");
            }
            Maybe::Just(default_value)
        }
        Maybe::Nothing => Maybe::Nothing,
    }
}

// A helper template to get string from option into a enum.
// The enum in the enum_values is the corresponding value to the strings
// in the str_values. If the option does not contains name,
// default_value will be return.
pub fn GetStringOrBooleanOption<T: Copy>(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    property: &str,
    method: &str,
    str_values: &Vec<&str>,
    enum_values: &Vec<T>,
    true_value: T,
    false_value: T,
    fallback_value: T,
) -> Maybe<T> {
    assert_eq!(str_values.len(), enum_values.len());
    let factory = isolate.factory();
    let property_str = factory.NewStringFromAsciiChecked(property);

    // 1. Let value be ? Get(options, property).
    let value: DirectHandle<Object>;
    match Object::GetPropertyOrElement(isolate, options, property_str) {
        Ok(v) => value = v,
        Err(_e) => return Maybe::Nothing,
    }
    // 2. If value is undefined, then return fallback.
    if IsUndefined(value, isolate) {
        return Maybe::Just(fallback_value);
    }
    // 3. If value is true, then return trueValue.
    if IsTrue(value, isolate) {
        return Maybe::Just(true_value);
    }
    // 4. Let valueBoolean be ToBoolean(value).
    let value_boolean = Object::BooleanValue(value, isolate);
    // 5. If valueBoolean is false, then return valueBoolean.
    if !value_boolean {
        return Maybe::Just(false_value);
    }

    let value_str: DirectHandle<String>;
    // 6. Let value be ? ToString(value).
    match Object::ToString(isolate, value) {
        Ok(v) => value_str = v,
        Err(_e) => return Maybe::Nothing,
    }
    // 7. If value is *"true"* or *"false"*, return _fallback_.
    if String::Equals(isolate, value_str, factory.true_string())
        || String::Equals(isolate, value_str, factory.false_string())
    {
        return Maybe::Just(fallback_value);
    }
    // 8. If values does not contain an element equal to _value_, throw a
    // *RangeError* exception.
    // 9. Return value.
    let value_str = String::Flatten(isolate, value_str);
    {
        let no_gc = DisallowGarbageCollection {};
        let flat = value_str.GetFlatContent(no_gc);
        let length = value_str.length();
        for i in 0..str_values.len() {
            if str_values[i].len() as i32 == length {
                if flat.IsOneByte() {
                    let str_values_i = str_values[i];
                    let equal = CompareCharsEqual(str_values_i, flat.ToOneByteVector().begin(), length);
                    if equal {
                        return Maybe::Just(enum_values[i]);
                    }
                } else {
                    let str_values_i = str_values[i];
                    let equal = CompareCharsEqual(str_values_i, flat.ToUC16Vector().begin(), length);
                    if equal {
                        return Maybe::Just(enum_values[i]);
                    }
                }
            }
        }
    } // end of no_gc
    THROW_NEW_ERROR_RETURN_VALUE(
        isolate,
        NewRangeError(
            MessageTemplate::kValueOutOfRange,
            value,
            factory.NewStringFromAsciiChecked(method),
            property_str,
        ),
        Maybe::Nothing,
    )
}

// ECMA402 9.2.10. GetOption( options, property, type, values, fallback)
// ecma402/#sec-getoption
//
// This is specialized for the case when type is boolean.
//
// Returns true if options object has the property and stores the
// result in value. Returns false if the value is not found. The
// caller is required to use fallback value appropriately in this
// case.
//
// method_name is a string denoting the method it called from; used when
// printing the error message.
pub fn GetBoolOption(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    property: &str,
    method_name: &str,
    result: &mut bool,
) -> Maybe<bool> {
    let property_str = isolate.factory().NewStringFromAsciiChecked(property);

    // 1. Let value be ? Get(options, property).
    let value: DirectHandle<Object>;
    match Object::GetPropertyOrElement(isolate, options, property_str) {
        Ok(v) => value = v,
        Err(_e) => return Maybe::Nothing,
    }

    // 2. If value is not undefined, then
    if !IsUndefined(value, isolate) {
        // 2. b. i. Let value be ToBoolean(value).
        *result = Object::BooleanValue(value, isolate);

        // 2. e. return value
        return Maybe::Just(true);
    }

    return Maybe::Just(false);
}

pub fn GetNumberOption(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    property: DirectHandle<String>,
    min: i32,
    max: i32,
    fallback: i32,
) -> Maybe<i32> {
    // 1. Let value be ? Get(options, property).
    let value: DirectHandle<Object>;
    match JSReceiver::GetProperty(isolate, options, property) {
        Ok(v) => value = v,
        Err(_e) => return Maybe::Nothing,
    }

    // Return ? DefaultNumberOption(value, minimum, maximum, fallback).
    DefaultNumberOption(isolate, value, min, max, fallback, property)
}

// #sec-getoption while type is "number"
pub fn GetNumberOptionAsDouble(
    isolate: &mut Isolate,
    options: DirectHandle<JSReceiver>,
    property: DirectHandle<String>,
    default_value: f64,
) -> Maybe<f64> {
    // 1. Let value be ? Get(options, property).
    let value: DirectHandle<Object>;
    match JSReceiver::GetProperty(isolate, options, property) {
        Ok(v) => value = v,
        Err(_e) => return Maybe::Nothing,
    }
    // 2. If value is undefined, then
    if IsUndefined(value, isolate) {
        // b. Return default.
        return Maybe::Just(default_value);
    }
    // 4. Else if type is "number", then
    // a. Set value to ? ToNumber(value).
    let value_num: DirectHandle<Number>;
    match Object::ToNumber(isolate, value) {
        Ok(v) => value_num = v,
        Err(_e) => return Maybe::Nothing,
    }
    // b. If value is NaN, throw a RangeError exception.
    if IsNaN(value_num) {
        THROW_NEW_ERROR_RETURN_VALUE(
            isolate,
            NewRangeError(MessageTemplate::kPropertyValueOutOfRange, value, isolate.factory().NewStringFromAsciiChecked(""), isolate.factory().NewStringFromAsciiChecked("")),
            Maybe::Nothing,
        );
    }

    // 7. Return value.
    return Maybe::Just(Object::NumberValue(*value_num.cast()));
}

// ecma402/#sec-defaultnumberoption
pub fn DefaultNumberOption(
    isolate: &mut Isolate,
    value: DirectHandle<Object>,
    min: i32,
    max: i32,
    fallback: i32,
    property: DirectHandle<String>,
) -> Maybe<i32> {
    // 2. Else, return fallback.
    if IsUndefined(value, isolate) {
        return Maybe::Just(fallback);
    }

    // 1. If value is not undefined, then
    // a. Let value be ? ToNumber(value).
    let value_num: DirectHandle<Number>;
    match Object::ToNumber(isolate, value) {
        Ok(v) => value_num = v,
        Err(_e) => return Maybe::Nothing,
    }
    // b. If value is NaN or less than minimum or greater than maximum, throw a
    // RangeError exception.
    if IsNaN(value_num)
        || Object::NumberValue(*value_num.cast()) < min as f64
        || Object::NumberValue(*value_num.cast()) > max as f64
    {
        THROW_NEW_ERROR_RETURN_VALUE(
            isolate,
            NewRangeError(MessageTemplate::kPropertyValueOutOfRange, value, isolate.factory().NewStringFromAsciiChecked(""), isolate.factory().NewStringFromAsciiChecked("")),
            Maybe::Nothing,
        );
    }

    // The max and min arguments are integers and the above check makes
    // sure that we are within the integer range making this double to
    // int conversion safe.
    //
    // c. Return floor(value).
    return Maybe::Just(FastD2I(floor(Object::NumberValue(*value_num.cast()))));
}
}

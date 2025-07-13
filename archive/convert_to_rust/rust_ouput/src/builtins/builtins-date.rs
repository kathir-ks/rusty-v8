// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-date.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::convert::TryInto;
  use std::f64::NAN;
  use std::mem::size_of;
  use std::ptr::null_mut;
  
  use crate::v8::internal::date::dateparser_inl::ParseDateTimeString;
  use crate::v8::internal::numbers::conversions::DoubleToInteger;
  use crate::v8::internal::objects::bigint::BigInt;
  
  use std::fmt;
  
  // Mocked data structures for compilation.
  
  pub struct Isolate {
      pub date_cache_: DateCache,
  }
  
  impl Isolate {
      pub fn date_cache(&mut self) -> &mut DateCache {
          &mut self.date_cache_
      }
      pub fn factory(&mut self) -> Factory {
          Factory {}
      }
  
      pub fn count_usage(&mut self, _feature: UseCounterFeature) {}
  }
  
  #[derive(Debug)]
  pub enum UseCounterFeature {
      kDateToLocaleDateString,
      kDateToLocaleString,
      kDateToLocaleTimeString,
  }
  
  
  pub struct Factory {}
  
  impl Factory {
      pub fn NewStringFromUtf8(&mut self, buffer: base::Vector<u8>) -> Result<String, String> {
          match String::from_utf8(buffer.to_vec()) {
              Ok(s) => Ok(s),
              Err(e) => Err(format!("Failed to create string from UTF8: {}", e)),
          }
      }
  
      pub fn NewNumber(&mut self, value: f64) -> Tagged<Object> {
          Tagged::new(Object {})
      }
  
      pub fn NewNumberFromInt64(&mut self, value: i64) -> Tagged<Object> {
          Tagged::new(Object {})
      }
  }
  
  pub struct HandleScope {}
  
  impl HandleScope {
      pub fn new(_isolate: &Isolate) -> Self {
          HandleScope {}
      }
  }
  
  
  pub struct JSDate {
      value_: f64,
  }
  
  impl JSDate {
      pub fn value(&self) -> f64 {
          self.value_
      }
  
      pub fn SetValue(&mut self, value: f64) {
          self.value_ = value;
      }
  
      pub fn SetNanValue(&mut self) {
          self.value_ = std::f64::NAN;
      }
  
      pub fn CurrentTimeValue(_isolate: &Isolate) -> i64 {
          1678886400000 // Mocked current time value
      }
  
      pub fn New(_target: DirectHandle<JSFunction>, _new_target: DirectHandle<JSReceiver>, time_val: f64) -> Result<Tagged<JSDate>, String> {
          Ok(Tagged::new(JSDate { value_: time_val }))
      }
  }
  
  pub struct DateCache {}
  
  impl DateCache {
      pub const kMaxTimeBeforeUTCInMs: i64 = 8640000000000000;
  
      pub fn ToUTC(&mut self, time_val: i64) -> f64 {
          time_val as f64 // Mocked UTC conversion
      }
  
      pub fn TryTimeClip(time_val: &mut f64) -> bool {
          if time_val.is_nan() || time_val.is_infinite() {
              return false;
          }
          *time_val = (*time_val as i64) as f64;
          true
      }
  
      pub fn ToLocal(&mut self, time_ms: i64) -> i64 {
          time_ms // Mocked local conversion
      }
  
      pub fn DaysFromTime(&mut self, local_time_ms: i64) -> i32 {
          (local_time_ms / (24 * 60 * 60 * 1000)) as i32
      }
  
      pub fn TimeInDay(&mut self, local_time_ms: i64, days: i32) -> i32 {
          (local_time_ms - (days as i64) * (24 * 60 * 60 * 1000)) as i32
      }
  
      pub fn YearMonthDayFromDays(&mut self, days: i32, year: &mut i32, month: &mut i32, day: &mut i32) {
          *year = 2023;
          *month = 3;
          *day = 15;
      }
  }
  
  pub struct ReadOnlyRoots {}
  
  impl ReadOnlyRoots {
      pub fn nan_value(&self) -> Tagged<Object> {
          Tagged::new(Object {}) // Mocked NaN value
      }
  }
  
  pub struct Object {}
  
  impl Object {
      pub fn ToNumber(_isolate: &Isolate, object: DirectHandle<Object>) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
      pub fn ToString(_isolate: &Isolate, object: DirectHandle<Object>) -> Result<Tagged<String>, String> {
          Ok(Tagged::new(String {}))
      }
      pub fn ToObject(_isolate: &Isolate, object: DirectHandle<Object>) -> Result<Tagged<JSReceiver>, String> {
          Ok(Tagged::new(JSReceiver {}))
      }
  
      pub fn NumberValue(_object: &Object) -> f64 {
          0.0
      }
  
      pub fn GetProperty(_isolate: &Isolate, _receiver: DirectHandle<JSReceiver>, _name: DirectHandle<String>) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
  
      pub fn SameNumberValue(a: f64, b: f64) -> bool {
          if a.is_nan() && b.is_nan() {
              return true;
          }
          if a == 0.0 && b == 0.0 && a.is_sign_negative() != b.is_sign_negative() {
              return false;
          }
          a == b
      }
  
      pub fn ToPrimitive(_isolate: &Isolate, object: DirectHandle<Object>) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
      pub fn ToPrimitive(_isolate: &Isolate, object: DirectHandle<JSReceiver>, _hint: ToPrimitiveHint) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
  }
  
  
  pub struct JSReceiver {}
  
  pub struct JSFunction {}
  
  pub struct String {}
  
  impl String {}
  
  pub struct Smi {}
  
  impl Smi {
      pub fn FromInt(value: i32) -> Self {
          Smi {}
      }
  }
  
  pub struct Tagged<T> {
      _phantom: std::marker::PhantomData<T>,
  }
  
  impl<T> Tagged<T> {
      pub fn new(_data: T) -> Self {
          Tagged {
              _phantom: std::marker::PhantomData,
          }
      }
  }
  
  impl<T> Copy for Tagged<T> {}
  
  impl<T> Clone for Tagged<T> {
      fn clone(&self) -> Self {
          *self
      }
  }
  
  #[derive(Clone, Copy)]
  pub struct DirectHandle<T> {
      _phantom: std::marker::PhantomData<T>,
  }
  
  impl<T> DirectHandle<T> {
      pub fn new() -> Self {
          DirectHandle {
              _phantom: std::marker::PhantomData,
          }
      }
  }
  
  
  
  
  
  
  
  
  
  pub mod base {
      use std::ops::Deref;
  
      #[derive(Debug)]
      pub struct Vector<T> {
          data: Vec<T>,
      }
  
      impl<T> Vector<T> {
          pub fn new(data: Vec<T>) -> Self {
              Vector { data }
          }
  
          pub fn to_vec(&self) -> Vec<T> {
              self.data.clone()
          }
  
          pub fn of(data: Vec<u8>) -> Vector<u8> {
              Vector { data: data }
          }
      }
  
      impl<T> Deref for Vector<T> {
          type Target = [T];
  
          fn deref(&self) -> &Self::Target {
              &self.data
          }
      }
      
      pub fn VectorOf(buffer: DateBuffer) -> Vector<u8> {
          Vector::new(buffer.0)
      }
  }
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  pub struct NewRangeError {}
  
  impl NewRangeError {
      pub fn new(_message_template: MessageTemplate) -> Self {
          NewRangeError {}
      }
  }
  
  pub struct NewTypeError {}
  
  impl NewTypeError {
      pub fn new(_message_template: MessageTemplate, _arg: DirectHandle<String>) -> Self {
          NewTypeError {}
      }
  }
  
  #[derive(Debug)]
  pub enum MessageTemplate {
      kInvalidTimeValue,
      kCalledNonCallable,
  }
  
  pub mod Execution {
      use crate::v8::internal::DirectHandle;
      use crate::v8::internal::Isolate;
      use crate::v8::internal::JSReceiver;
      use crate::v8::internal::Object;
      use crate::v8::internal::Tagged;
  
      pub fn Call(_isolate: &Isolate, _function: DirectHandle<Object>, _receiver: DirectHandle<JSReceiver>, _args: Vec<Tagged<Object>>) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
  }
  
  #[derive(Debug)]
  pub enum ToDateStringMode {
      kLocalDateAndTime,
      kLocalDate,
      kLocalTime,
      kISODateAndTime,
      kUTCDateAndTime,
  }
  
  pub struct DateBuffer(Vec<u8>);
  
  impl DateBuffer {
      pub fn new(vec: Vec<u8>) -> Self {
          DateBuffer(vec)
      }
  }
  
  
  pub fn ToDateString(time_val: f64, _date_cache: &DateCache, mode: ToDateStringMode) -> DateBuffer {
      let mut buffer = Vec::new();
  
      match mode {
          ToDateStringMode::kLocalDateAndTime => {
              buffer.extend_from_slice(b"Wed Mar 15 2023 10:30:00 GMT+0000 (Coordinated Universal Time)");
          }
          ToDateStringMode::kLocalDate => {
              buffer.extend_from_slice(b"Wed Mar 15 2023");
          }
          ToDateStringMode::kLocalTime => {
              buffer.extend_from_slice(b"10:30:00 GMT+0000 (Coordinated Universal Time)");
          }
          ToDateStringMode::kISODateAndTime => {
              if time_val.is_nan() {
                  buffer.extend_from_slice(b"Invalid Date");
              } else {
                  buffer.extend_from_slice(b"2023-03-15T10:30:00.000Z");
              }
          }
          ToDateStringMode::kUTCDateAndTime => {
              buffer.extend_from_slice(b"Wed, 15 Mar 2023 10:30:00 GMT");
          }
      }
  
      DateBuffer(buffer)
  }
  
  #[derive(Debug)]
  pub enum ToPrimitiveHint {
      kNumber,
      kString,
      kDefault,
  }
  
  
  pub mod temporal {
      use crate::v8::internal::BigInt;
      use crate::v8::internal::DirectHandle;
      use crate::v8::internal::Isolate;
      use crate::v8::internal::Object;
      use crate::v8::internal::Tagged;
  
      pub fn CreateTemporalInstant(_isolate: &Isolate, _ns: DirectHandle<BigInt>) -> Result<Tagged<Object>, String> {
          Ok(Tagged::new(Object {}))
      }
  }
  
  // ES #sec-date-constructor
  pub fn DateConstructor(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
  
      let new_target = args.get(0).cloned();
      if new_target.is_none() {
          let time_val = JSDate::CurrentTimeValue(isolate) as f64;
          let buffer = ToDateString(time_val, &isolate.date_cache_, ToDateStringMode::kLocalDateAndTime);
          
          return isolate.factory().NewStringFromUtf8(base::VectorOf(buffer)).map(|_| Tagged::new(Object {}));
      }
  
      let argc = args.len() - 1;
      let target = DirectHandle::<JSFunction>::new();
      let new_target = DirectHandle::<JSReceiver>::new();
      let mut time_val: f64;
  
      if argc == 0 {
          time_val = JSDate::CurrentTimeValue(isolate) as f64;
      } else if argc == 1 {
          let value = args.get(1).cloned().ok_or("Argument at index 1 is missing".to_string())?;
          if false { // IsJSDate(*value)
              time_val = 0.0; //Cast::<JSDate>(value)->value();
          } else {
              let value = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, value,Object::ToPrimitive(isolate, value));
              if false { //IsString(*value)
                  time_val = 0.0; //ParseDateTimeString(isolate, Cast::<String>(value));
              } else {
                  let value = DirectHandle::<Object>::new();
                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, value,Object::ToNumber(isolate, value));
                  time_val = 0.0;//Object::NumberValue(*value);
              }
          }
      } else {
          let year_object = DirectHandle::<Object>::new();
          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, year_object,Object::ToNumber(isolate, args.at(1)));
  
          let month_object = DirectHandle::<Object>::new();
          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, month_object,Object::ToNumber(isolate, args.at(2)));
          
          let year = 0.0;//Object::NumberValue(*year_object);
          let month = 0.0;//Object::NumberValue(*month_object);
          let mut date = 1.0;
          let mut hours = 0.0;
          let mut minutes = 0.0;
          let mut seconds = 0.0;
          let mut ms = 0.0;
  
          if argc >= 3 {
              let date_object = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, date_object,Object::ToNumber(isolate, args.at(3)));
              date = 0.0;//Object::NumberValue(*date_object);
  
              if argc >= 4 {
                  let hours_object = DirectHandle::<Object>::new();
                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, hours_object,Object::ToNumber(isolate, args.at(4)));
                  hours = 0.0;//Object::NumberValue(*hours_object);
  
                  if argc >= 5 {
                      let minutes_object = DirectHandle::<Object>::new();
                      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, minutes_object,Object::ToNumber(isolate, args.at(5)));
                      minutes = 0.0;//Object::NumberValue(*minutes_object);
  
                      if argc >= 6 {
                          let seconds_object = DirectHandle::<Object>::new();
                          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, seconds_object,Object::ToNumber(isolate, args.at(6)));
                          seconds = 0.0;//Object::NumberValue(*seconds_object);
  
                          if argc >= 7 {
                              let ms_object = DirectHandle::<Object>::new();
                              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, ms_object,Object::ToNumber(isolate, args.at(7)));
                              ms = 0.0;//Object::NumberValue(*ms_object);
                          }
                      }
                  }
              }
          }
  
          let mut year = year;
          if !year.is_nan() {
              let y = DoubleToInteger(year);
              if 0.0 <= y && y <= 99.0 {
                  year = 1900.0 + y;
              }
          }
  
          let day = MakeDay(year, month, date);
          let time = MakeTime(hours, minutes, seconds, ms);
          time_val = MakeDate(day, time);
          if time_val >= -(DateCache::kMaxTimeBeforeUTCInMs as f64) && time_val <= (DateCache::kMaxTimeBeforeUTCInMs as f64) {
              time_val = isolate.date_cache().ToUTC(time_val as i64);
          } else {
              time_val = std::f64::NAN;
          }
      }
  
      JSDate::New(target, new_target, time_val).map(|_| Tagged::new(Object {}))
  }
  
  // ES6 section 20.3.3.1 Date.now ( )
  pub fn DateNow(args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
      Ok(isolate.factory().NewNumberFromInt64(JSDate::CurrentTimeValue(isolate)))
  }
  
  // ES6 section 20.3.3.2 Date.parse ( string )
  pub fn DateParse(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
      let string = DirectHandle::<String>::new();
  
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, string,Object::ToString(isolate, args.atOrUndefined(isolate, 1)));
  
      Ok(isolate.factory().NewNumber(ParseDateTimeString(isolate, string)))
  }
  
  // ES6 section 20.3.3.4 Date.UTC (year,month,date,hours,minutes,seconds,ms)
  pub fn DateUTC(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
      let argc = args.len() - 1;
      let mut year = std::f64::NAN;
      let mut month = 0.0;
      let mut date = 1.0;
      let mut hours = 0.0;
      let mut minutes = 0.0;
      let mut seconds = 0.0;
      let mut ms = 0.0;
  
      if argc >= 1 {
          let year_object = DirectHandle::<Object>::new();
          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, year_object, Object::ToNumber(isolate, args.at(1)));
          year = 0.0;//Object::NumberValue(*year_object);
          if argc >= 2 {
              let month_object = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, month_object, Object::ToNumber(isolate, args.at(2)));
              month = 0.0;//Object::NumberValue(*month_object);
              if argc >= 3 {
                  let date_object = DirectHandle::<Object>::new();
                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, date_object, Object::ToNumber(isolate, args.at(3)));
                  date = 0.0;//Object::NumberValue(*date_object);
                  if argc >= 4 {
                      let hours_object = DirectHandle::<Object>::new();
                      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, hours_object, Object::ToNumber(isolate, args.at(4)));
                      hours = 0.0;//Object::NumberValue(*hours_object);
                      if argc >= 5 {
                          let minutes_object = DirectHandle::<Object>::new();
                          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, minutes_object, Object::ToNumber(isolate, args.at(5)));
                          minutes = 0.0;//Object::NumberValue(*minutes_object);
                          if argc >= 6 {
                              let seconds_object = DirectHandle::<Object>::new();
                              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, seconds_object, Object::ToNumber(isolate, args.at(6)));
                              seconds = 0.0;//Object::NumberValue(*seconds_object);
                              if argc >= 7 {
                                  let ms_object = DirectHandle::<Object>::new();
                                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, ms_object, Object::ToNumber(isolate, args.at(7)));
                                  ms = 0.0;//Object::NumberValue(*ms_object);
                              }
                          }
                      }
                  }
              }
          }
      }
  
      if !year.is_nan() {
          let y = DoubleToInteger(year);
          if 0.0 <= y && y <= 99.0 {
              year = 1900.0 + y;
          }
      }
  
      let day = MakeDay(year, month, date);
      let time = MakeTime(hours, minutes, seconds, ms);
      let mut value = MakeDate(day, time);
      if DateCache::TryTimeClip(&mut value) {
          Ok(isolate.factory().NewNumber(value))
      } else {
          Ok(ReadOnlyRoots {}.nan_value())
      }
  }
  
  // ES6 section 20.3.4.20 Date.prototype.setDate ( date )
  pub fn DatePrototypeSetDate(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
  
      //CHECK_RECEIVER(JSDate, date, "Date.prototype.setDate");
      let date = DirectHandle::<JSDate>::new();
  
      let value = DirectHandle::<Object>::new();
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, value, Object::ToNumber(isolate, value));
  
      let mut time_val = 0.0; //date->value();
      if !time_val.is_nan() {
          let time_ms = time_val as i64;
          let local_time_ms = isolate.date_cache().ToLocal(time_ms);
          let days = isolate.date_cache().DaysFromTime(local_time_ms);
          let time_within_day = isolate.date_cache().TimeInDay(local_time_ms, days);
          let mut year = 0;
          let mut month = 0;
          let mut day = 0;
          isolate.date_cache().YearMonthDayFromDays(days, &mut year, &mut month, &mut day);
          
          time_val = MakeDate(MakeDay(year as f64, month as f64, 0.0), time_within_day);
      }
  
      SetLocalDateValue(isolate, DirectHandle::<JSDate>::new(), time_val)
  }
  
  // ES6 section 20.3.4.21 Date.prototype.setFullYear (year, month, date)
  pub fn DatePrototypeSetFullYear(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
  
      //CHECK_RECEIVER(JSDate, date, "Date.prototype.setFullYear");
      let date = DirectHandle::<JSDate>::new();
  
      let argc = args.len() - 1;
      let year = DirectHandle::<Object>::new();
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, year,Object::ToNumber(isolate, year));
  
      let year_double = 0.0;//Object::NumberValue(*year);
      let mut month_double = 0.0;
      let mut day_double = 1.0;
      let mut time_within_day = 0;
  
      if false {//!std::isnan(date->value())
          let time_ms = 0;//static_cast::<int64_t>(date->value());
          let local_time_ms = isolate.date_cache().ToLocal(time_ms);
          let days = isolate.date_cache().DaysFromTime(local_time_ms);
          time_within_day = isolate.date_cache().TimeInDay(local_time_ms, days);
          let mut year_int = 0;
          let mut month_int = 0;
          let mut day_int = 0;
          isolate.date_cache().YearMonthDayFromDays(days, &mut year_int, &mut month_int, &mut day_int);
          month_double = month_int as f64;
          day_double = day_int as f64;
      }
  
      if argc >= 2 {
          let month = DirectHandle::<Object>::new();
          //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, month,Object::ToNumber(isolate, month));
          month_double = 0.0;//Object::NumberValue(*month);
          if argc >= 3 {
              let day = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, day,Object::ToNumber(isolate, day));
              day_double = 0.0;//Object::NumberValue(*day);
          }
      }
      let time_val = MakeDate(MakeDay(year_double, month_double, day_double), time_within_day);
      SetLocalDateValue(isolate, DirectHandle::<JSDate>::new(), time_val)
  }
  
  // ES6 section 20.3.4.22 Date.prototype.setHours(hour, min, sec, ms)
  pub fn DatePrototypeSetHours(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
      //CHECK_RECEIVER(JSDate, date, "Date.prototype.setHours");
      let date = DirectHandle::<JSDate>::new();
      let argc = args.len() - 1;
      let hour = DirectHandle::<Object>::new();
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, hour,Object::ToNumber(isolate, hour));
      let h = 0.0;//Object::NumberValue(*hour);
      let mut time_val = 0.0;//date->value();
      if !time_val.is_nan() {
          let time_ms = 0;//static_cast::<int64_t>(time_val);
          let local_time_ms = isolate.date_cache().ToLocal(time_ms);
          let day = isolate.date_cache().DaysFromTime(local_time_ms);
          let time_within_day = isolate.date_cache().TimeInDay(local_time_ms, day);
          let m = (time_within_day / (60 * 1000)) % 60;
          let s = (time_within_day / 1000) % 60;
          let milli = time_within_day % 1000;
          let mut m = m as f64;
          let mut s = s as f64;
          let mut milli = milli as f64;
          if (argc >= 2) {
              let min = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, min,Object::ToNumber(isolate, min));
              m = 0.0;//Object::NumberValue(*min);
              if (argc >= 3) {
                  let sec = DirectHandle::<Object>::new();
                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, sec,Object::ToNumber(isolate, sec));
                  s = 0.0;//Object::NumberValue(*sec);
                  if (argc >= 4) {
                      let ms = DirectHandle::<Object>::new();
                      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, ms,Object::ToNumber(isolate, ms));
                      milli = 0.0;//Object::NumberValue(*ms);
                  }
              }
          }
          time_val = MakeDate(day as f64, MakeTime(h, m, s, milli));
      }
      SetLocalDateValue(isolate, DirectHandle::<JSDate>::new(), time_val)
  }
  
  // ES6 section 20.3.4.23 Date.prototype.setMilliseconds(ms)
  pub fn DatePrototypeSetMilliseconds(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
  
      //CHECK_RECEIVER(JSDate, date, "Date.prototype.setMilliseconds");
      let date = DirectHandle::<JSDate>::new();
  
      let ms = DirectHandle::<Object>::new();
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, ms,Object::ToNumber(isolate, ms));
  
      let mut time_val = 0.0; //date->value();
      if !time_val.is_nan() {
          let time_ms = time_val as i64;
          let local_time_ms = isolate.date_cache().ToLocal(time_ms);
          let day = isolate.date_cache().DaysFromTime(local_time_ms);
          let time_within_day = isolate.date_cache().TimeInDay(local_time_ms, day);
          let h = time_within_day / (60 * 60 * 1000);
          let m = (time_within_day / (60 * 1000)) % 60;
          let s = (time_within_day / 1000) % 60;
          time_val = MakeDate(day as f64, MakeTime(h as f64, m as f64, s as f64, 0.0));
      }
      SetLocalDateValue(isolate, DirectHandle::<JSDate>::new(), time_val)
  }
  
  // ES6 section 20.3.4.24 Date.prototype.setMinutes ( min, sec, ms )
  pub fn DatePrototypeSetMinutes(mut args: Vec<Tagged<Object>>, isolate: &mut Isolate) -> Result<Tagged<Object>, String> {
      let scope = HandleScope::new(isolate);
  
      //CHECK_RECEIVER(JSDate, date, "Date.prototype.setMinutes");
      let date = DirectHandle::<JSDate>::new();
  
      let argc = args.len() - 1;
      let min = DirectHandle::<Object>::new();
      //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, min,Object::ToNumber(isolate, min));
  
      let mut time_val = 0.0; //date->value();
      if !time_val.is_nan() {
          let time_ms = time_val as i64;
          let local_time_ms = isolate.date_cache().ToLocal(time_ms);
          let day = isolate.date_cache().DaysFromTime(local_time_ms);
          let time_within_day = isolate.date_cache().TimeInDay(local_time_ms, day);
          let h = time_within_day / (60 * 60 * 1000);
          let m = 0.0; //Object::NumberValue(*min);
          let s = (time_within_day / 1000) % 60;
          let milli = time_within_day % 1000;
          let mut s = s as f64;
          let mut milli = milli as f64;
  
          if argc >= 2 {
              let sec = DirectHandle::<Object>::new();
              //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, sec,Object::ToNumber(isolate, sec));
              s = 0.0; //Object::NumberValue(*sec);
  
              if argc >= 3 {
                  let ms = DirectHandle::<Object>::new();
                  //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, ms,Object::ToNumber(isolate, ms));
                  milli = 0.0;//Object::NumberValue(*ms);
              }
          }
          time_val = MakeDate(day as f64, MakeTime(h as f64, m, s, milli));
      }
      SetLocalDateValue(isolate, DirectHandle::<JSDate>::new(), time_val)
  }
  
  // ES6 section 

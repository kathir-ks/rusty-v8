// Converted from V8 C++ source files:
// Header: js-date-time-format.h
// Implementation: js-date-time-format.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub mod js_date_time_format_tq_inc {
}
pub mod object_macros {

}
use std::collections::HashSet;
use std::string::String;
use crate::v8::internal::base::bit_field::BitField;
use crate::v8::internal::execution::isolate::Isolate;
use crate::v8::internal::objects::intl_objects::Intl;
use crate::v8::internal::objects::managed::Managed;

pub mod u_icu_namespace {
    pub struct DateIntervalFormat {}
    pub struct Locale {}
    pub struct SimpleDateFormat {}
    pub struct TimeZone {}
}

pub mod v8 {
    pub mod internal {
        pub struct JSObject {}
        pub struct String {}
        pub struct Map {}
        pub struct Object {}
        pub struct JSArray {}
        pub struct Handle<T> {
            dummy: i32
        }
        pub struct JSReceiver {}
        pub struct JSFunction {}
        pub struct Context {}
        pub struct FixedArray {}
        pub struct Tagged<T> {
            dummy: i32
        }
        pub struct TaggedObject {}
        pub struct Factory {}
        pub struct BigInt {}
        pub struct JSDate {}
        pub struct PtrComprCageBase {}
    }
}
pub mod objects {
    pub struct JSDateTimeFormat {}
}
pub struct JSDateTimeFormat {
dummy: i32
}

impl JSDateTimeFormat {
    pub enum class RequiredOption {
        kDate,
        kTime,
        kAny
    }
    pub enum class DefaultsOption {
        kDate,
        kTime,
        kAll
    }
    pub struct DateTimeFormat { dummy: i32 }
    pub fn new(isolate: &Isolate, map: &v8::internal::Map, locales: &Object, options: &Options) -> Self {Self{dummy : 1}}
    pub struct Internal {}
    pub struct Options {}
    pub struct JSPluralRules {}
    pub fn of(_value: v8::internal::TaggedObject) -> Self {Self{dummy : 1}}
    pub struct Object {}
    pub struct String {}
}
impl JSDateTimeFormat{
  // ecma-402/#sec-todatetimeoptions
  pub enum RequiredOption { kDate, kTime, kAny }
  pub enum DefaultsOption { kDate, kTime, kAll }

  //V8_WARN_UNUSED_RESULT 
  pub fn New(
      isolate: *mut Isolate, map: DirectHandle<v8::internal::Map>, locales: DirectHandle<v8::internal::Object>,
      options: DirectHandle<v8::internal::Object>, service: *const i8) -> Result<MaybeDirectHandle<JSDateTimeFormat>, Box<dyn std::error::Error>> {
          Err("Unimplemented".into())
  }

  //V8_WARN_UNUSED_RESULT 
  pub fn CreateDateTimeFormat(isolate: *mut Isolate, map: DirectHandle<v8::internal::Map>,
                       locales: DirectHandle<v8::internal::Object>,
                       options: DirectHandle<v8::internal::Object>, required: RequiredOption,
                       defaults: DefaultsOption, service: *const i8) -> Result<MaybeDirectHandle<JSDateTimeFormat>, Box<dyn std::error::Error>> {
        Err("Unimplemented".into())
  }

  //V8_WARN_UNUSED_RESULT 
  pub fn ResolvedOptions(isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>) -> Result<MaybeDirectHandle<v8::internal::JSObject>, Box<dyn std::error::Error>> {
         Err("Unimplemented".into())
  }

  //V8_WARN_UNUSED_RESULT 
  pub fn Calendar(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>) -> Result<DirectHandle<v8::internal::String>, Box<dyn std::error::Error>> {
        Err("Unimplemented".into())
  }

  //V8_WARN_UNUSED_RESULT 
  pub fn TimeZone(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>) -> Result<DirectHandle<v8::internal::Object>, Box<dyn std::error::Error>> {
         Err("Unimplemented".into())
  }

  // ecma402/#sec-unwrapdatetimeformat
  //V8_WARN_UNUSED_RESULT 
  pub fn UnwrapDateTimeFormat(isolate: *mut Isolate, format_holder: v8::internal::Handle<v8::internal::JSReceiver>) -> Result<MaybeDirectHandle<JSDateTimeFormat>, Box<dyn std::error::Error>>{
        Err("Unimplemented".into())
  }

  // ecma402/#sec-datetime-format-functions
  // DateTime Format Functions
  //V8_WARN_UNUSED_RESULT 
  pub fn DateTimeFormat(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>,
      date: DirectHandle<v8::internal::Object>, method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::String>, Box<dyn std::error::Error>> {
         Err("Unimplemented".into())
  }

  // ecma402/#sec-Intl.DateTimeFormat.prototype.formatToParts
  //V8_WARN_UNUSED_RESULT 
  pub fn FormatToParts(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>,
      x: DirectHandle<v8::internal::Object>, output_source: bool, method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::JSArray>, Box<dyn std::error::Error>> {
          Err("Unimplemented".into())
  }

  // ecma402/#sec-intl.datetimeformat.prototype.formatRange
  //V8_WARN_UNUSED_RESULT 
  pub fn FormatRange(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>,
      x_date_value: DirectHandle<v8::internal::Object>, y_date_value: DirectHandle<v8::internal::Object>,
      method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::String>, Box<dyn std::error::Error>> {
         Err("Unimplemented".into())
  }

  // ecma402/sec-Intl.DateTimeFormat.prototype.formatRangeToParts
  //V8_WARN_UNUSED_RESULT 
  pub fn FormatRangeToParts(
      isolate: *mut Isolate, date_time_format: DirectHandle<JSDateTimeFormat>,
      x_date_value: DirectHandle<v8::internal::Object>, y_date_value: DirectHandle<v8::internal::Object>,
      method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::JSArray>, Box<dyn std::error::Error>>{
          Err("Unimplemented".into())
  }

  //V8_WARN_UNUSED_RESULT 
  pub fn ToLocaleDateTime(
      isolate: *mut Isolate, date: DirectHandle<v8::internal::Object>, locales: DirectHandle<v8::internal::Object>,
      options: DirectHandle<v8::internal::Object>, required: RequiredOption,
      defaults: DefaultsOption, method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::String>, Box<dyn std::error::Error>>{
        Err("Unimplemented".into())
  }

  // Function to support Temporal
  //V8_WARN_UNUSED_RESULT 
  pub fn TemporalToLocaleString(
      isolate: *mut Isolate, temporal: DirectHandle<v8::internal::JSReceiver>,
      locales: DirectHandle<v8::internal::Object>, options: DirectHandle<v8::internal::Object>,
      method_name: *const i8) -> Result<MaybeDirectHandle<v8::internal::String>, Box<dyn std::error::Error>> {
        Err("Unimplemented".into())
  }

  
  pub fn GetAvailableLocales() -> &'static HashSet<String> {
        Box::leak(Box::new(HashSet::new()))
  }

  pub fn TimeZoneId(isolate: *mut Isolate,
                                         tz: &u_icu_namespace::TimeZone) -> DirectHandle<v8::internal::Object> {
                                             DirectHandle{dummy: 1}
                                         }
  //V8_WARN_UNUSED_RESULT 
  pub fn TimeZoneIdToString(
      isolate: *mut Isolate, id: &icu::UnicodeString) -> Result<v8::internal::MaybeHandle<v8::internal::String>, Box<dyn std::error::Error>>{
         Err("Unimplemented".into())
  }

  pub fn CreateTimeZone(
      isolate: *mut Isolate, time_zone: DirectHandle<v8::internal::String>) -> Result<std::unique_ptr<u_icu_namespace::TimeZone>, Box<dyn std::error::Error>> {
         Err("Unimplemented".into())
  }

  
  pub fn CanonicalizeTimeZoneID(
      input: &String) -> String {
          String::new()
      }

  pub fn HourCycleAsString(&self, isolate: *mut Isolate) -> v8::internal::Handle<v8::internal::String> {
          v8::internal::Handle{dummy: 1}
      }

  // ecma-402/#sec-properties-of-intl-datetimeformat-instances
  pub enum DateTimeStyle { kUndefined, kFull, kLong, kMedium, kShort }

  // enum for "hourCycle" option.
  pub enum HourCycle { kUndefined, kH11, kH12, kH23, kH24 }

  
  pub fn set_hour_cycle(&mut self, hour_cycle: HourCycle) {}
  
  pub fn hour_cycle(&self) -> HourCycle {HourCycle::kUndefined}

  
  pub fn set_date_style(&mut self, date_style: DateTimeStyle) {}
  
  pub fn date_style(&self) -> DateTimeStyle {DateTimeStyle::kUndefined}

  
  pub fn set_time_style(&mut self, time_style: DateTimeStyle) {}
  
  pub fn time_style(&self) -> DateTimeStyle {DateTimeStyle::kUndefined}

  // Bit positions in |flags|.
  //DEFINE_TORQUE_GENERATED_JS_DATE_TIME_FORMAT_FLAGS()

  //static_assert(HourCycleBits::is_valid(HourCycle::kUndefined));
  //static_assert(HourCycleBits::is_valid(HourCycle::kH11));
  //static_assert(HourCycleBits::is_valid(HourCycle::kH12));
  //static_assert(HourCycleBits::is_valid(HourCycle::kH23));
  //static_assert(HourCycleBits::is_valid(HourCycle::kH24));

  //static_assert(DateStyleBits::is_valid(DateTimeStyle::kUndefined));
  //static_assert(DateStyleBits::is_valid(DateTimeStyle::kFull));
  //static_assert(DateStyleBits::is_valid(DateTimeStyle::kLong));
  //static_assert(DateStyleBits::is_valid(DateTimeStyle::kMedium));
  //static_assert(DateStyleBits::is_valid(DateTimeStyle::kShort));

  //static_assert(TimeStyleBits::is_valid(DateTimeStyle::kUndefined));
  //static_assert(TimeStyleBits::is_valid(DateTimeStyle::kFull));
  //static_assert(TimeStyleBits::is_valid(DateTimeStyle::kLong));
  //static_assert(TimeStyleBits::is_valid(DateTimeStyle::kMedium));
  //static_assert(TimeStyleBits::is_valid(DateTimeStyle::kShort));

  //DECL_ACCESSORS(icu_locale, Tagged<Managed<icu::Locale>>)
 pub fn set_icu_locale(&mut self, value: v8::internal::Tagged<Managed<u_icu_namespace::Locale>>) {}
 pub fn icu_locale(&self) -> v8::internal::Tagged<Managed<u_icu_namespace::Locale>> {v8::internal::Tagged{dummy : 1}}
  //DECL_ACCESSORS(icu_simple_date_format, Tagged<Managed<icu::SimpleDateFormat>>)
   pub fn set_icu_simple_date_format(&mut self, value: v8::internal::Tagged<Managed<u_icu_namespace::SimpleDateFormat>>) {}
 pub fn icu_simple_date_format(&self) -> v8::internal::Tagged<Managed<u_icu_namespace::SimpleDateFormat>> {v8::internal::Tagged{dummy : 1}}
  //DECL_ACCESSORS(icu_date_interval_format,
  //               Tagged<Managed<icu::DateIntervalFormat>>)
  
    pub fn set_icu_date_interval_format(&mut self, value: v8::internal::Tagged<Managed<u_icu_namespace::DateIntervalFormat>>) {}
 public fn icu_date_interval_format(&self) -> v8::internal::Tagged<Managed<u_icu_namespace::DateIntervalFormat>> {v8::internal::Tagged{dummy : 1}}
  //DECL_PRINTER(JSDateTimeFormat)

  //TQ_OBJECT_CONSTRUCTORS(JSDateTimeFormat)
}

pub type DirectHandle<T> = v8::internal::Handle<T>;
pub type MaybeDirectHandle<T> = Result<DirectHandle<T>, Box<dyn std::error::Error>>;

pub mod base{
    pub mod bit_field{
      pub struct BitField{dummy: i32}
      impl BitField{
         pub fn update(val:i32,flag: bool)->i32{val}
      }
   }
}
pub mod date{
    pub fn Date{}
}
pub mod i18n{
    pub struct Internationalization{}
}
pub mod tagged_impl_inl{
  pub struct TaggedField<T, const OFFSET: usize>{dummy: i32}
}
pub mod js_temporal_objects_inl{
  pub struct JSTemporalPlainDate{}
}

pub enum DateStyleBits {
    kShift
}
impl DateStyleBits {
    pub fn decode(_value: i32) -> bool {
        false
    }
}
pub enum HourCycleBits {
    kShift
}
impl HourCycleBits {
    pub fn decode(_value: i32) -> bool {
        false
    }
}
pub enum TimeStyleBits {
    kShift
}
impl TimeStyleBits {
    pub fn decode(_value: i32) -> bool {
        false
    }
}

pub enum Weekday {
    kShift
}
impl Weekday {
    pub fn decode(_value: i32) -> bool {
        false
    }
    pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Era {
    kShift
}
impl Era {
    pub fn decode(_value: i32) -> bool {
        false
    }
    pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Year {
    kShift
}
impl Year {
    pub fn decode(_value: i32) -> bool {
        false
    }
    pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Month {
    kShift
}
impl Month {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Day {
    kShift
}
impl Day {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum DayPeriod {
    kShift
}
impl DayPeriod {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Hour {
    kShift
}
impl Hour {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Minute {
    kShift
}
impl Minute {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum Second {
    kShift
}
impl Second {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}
pub enum TimeZoneName {
    kShift
}
impl TimeZoneName {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}

pub enum FractionalSecondDigits {
    kShift
}
impl FractionalSecondDigits {
    pub fn decode(_value: i32) -> bool {
        false
    }
      pub fn update(_val: i32, _bit:bool) -> i32 {
      1
    }
}

pub struct JSDate {
 dummy: i32
}
impl JSDate{
    pub fn CurrentTimeValue(isolate: *mut Isolate) -> i64 {1}
}
pub struct TimeClip{}
impl TimeClip{
     pub fn TryTimeClip(_input: &mut f64)->bool{true}
}
pub struct IsUndefined{}
impl IsUndefined{
  pub fn IsUndefined(_obj: &v8::internal::Object, _isolate: *mut Isolate) -> bool{true}
}
pub mod option_utils{
  pub fn CoerceOptionsToObject(isolate: *mut Isolate, input_options:DirectHandle<v8::internal::Object>, _service: *const i8) -> Result<DirectHandle<v8::internal::JSReceiver>, Box<dyn std::error::Error>>{
       Err("".into())
  }
    pub fn GetBoolOption(isolate: *mut Isolate, options:DirectHandle<v8::internal::JSReceiver>,_option_name: *const i8, service: *const i8, flag: &mut bool)-> Result<Maybe<bool>, Box<dyn std::error::Error>>{
         Err("".into())
    }
     pub struct Maybe<T> {
        pub value: Option<T>,
    }
    impl<T> Maybe<T> {
        pub fn FromJust(self) -> T {
            self.value.unwrap()
        }
         pub fn IsNothing(&self) -> bool {
            self.value.is_none()
        }
    }
}
pub mod bigint{
  pub struct Digits{}
    pub fn Divide(isolate: *mut Isolate, number: v8::internal::DirectHandle<v8::internal::BigInt>, divider: v8::internal::DirectHandle<v8::internal::BigInt>) -> Result<v8::internal::DirectHandle<v8::internal::BigInt>, Box<dyn std::error::Error>> {
      Err("".into())
    }
    pub fn FromInt64(isolate: *mut Isolate, value: i64) -> v8::internal::DirectHandle<v8::internal::BigInt>{
      v8::internal::DirectHandle{dummy: 1}
    }
}

pub mod temporal {
    pub struct JSTemporalPlainDateTime{dummy: i32}
    pub struct JSTemporalPlainDate{dummy: i32}
        pub struct JSTemporalTimeZone{dummy: i32}
    pub struct JSTemporalInstant{dummy: i32}
    pub struct JSTemporalPlainYearMonth{dummy: i32}
    pub struct JSTemporalPlainMonthDay{dummy: i32}
    pub struct JSTemporalPlainTime{dummy: i32}
    pub struct JSTemporalZonedDateTime{dummy: i32}
    pub fn GetISO8601Calendar() -> v8::internal::JSReceiver{
      v8::internal::JSReceiver{}
    }
    pub fn CreateTemporalDateTime(isolate: *mut Isolate, dt: DateTime, calendar: v8::internal::JSReceiver) -> Result<v8::internal::DirectHandle<JSTemporalPlainDateTime>, Box<dyn std::error::Error>>{
        Err("".into())
    }
     pub fn BuiltinTimeZoneGetInstantForCompatible(isolate: *mut Isolate, time_zone: v8::internal::DirectHandle<JSTemporalTimeZone>, plain_date_time: v8::internal::DirectHandle<JSTemporalPlainDateTime>, s: *const i8) -> Result<v8::internal::DirectHandle<JSTemporalInstant>, Box<dyn std::error::Error>>{
      Err("".into())
    }
    pub fn CreateTemporalTimeZone(isolate: *mut Isolate, id:v8::internal::String) -> Result<v8::internal::Handle<JSTemporalTimeZone>, Box<dyn std::error::Error>>{
      Err("".into())
    }
    pub fn CreateTemporalInstant(isolate: *mut Isolate, ns: v8::internal::Handle<v8::internal::BigInt>) -> Result<v8::internal::Handle<JSTemporalInstant>, Box<dyn std::error::Error>>{
      Err("".into())
    }
        pub fn GetBuiltinCalendar(isolate: *mut Isolate, id:DirectHandle<v8::internal::String>) -> Result<v8::internal::DirectHandle<v8::internal::JSReceiver>, Box<dyn std::error::Error>>{
        Err("".into())
    }
}
pub struct DateTime{
    pub iso_year: i32,
    pub iso_month: i32,
    pub iso_day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub millisecond: i32,
    pub microsecond: i32,
    pub nanosecond: i32,

}
impl DateTime{
pub fn new(iso_year: i32,iso_month: i32, iso_day: i32, hour: i32, minute: i32,second: i32,millisecond: i32,microsecond: i32,nanosecond: i32,
)->Self{
        DateTime{iso_year:iso_year,iso_month:iso_month, iso_day: iso_day, hour: hour, minute: minute,second: second,millisecond: millisecond,microsecond: microsecond,nanosecond: nanosecond}
    }

}
impl v8::internal::JSObject{
}
impl v8::internal::String{

}
impl v8::internal::Map{

}
impl v8::internal::Object{
  pub fn NumberValue(self) -> f64{1.0}
  pub fn ToString(_isolate: *mut Isolate, handle: v8::internal::DirectHandle<v8::internal::Object>) -> Result<v8::internal::DirectHandle<v8::internal::String>, Box<dyn std::error::Error>>{
         Err("".into())
  }
     pub fn GetPropertyOrElement(isolate: *mut Isolate, obj: v8::internal::DirectHandle<v8::internal::JSReceiver>, name: v8::internal::DirectHandle<v8::internal::String>) -> Result<v8::internal::DirectHandle<v8::internal::Object>, Box<dyn std::error::Error>>{
       Err("".into())
  }

}
impl v8::internal::JSArray{
    pub fn push(_value: i32) {

    }
    pub fn ValidateElements(_result: v8::internal::JSArray){

    }
}
impl v8::internal::Handle<JSDateTimeFormat>{

}
pub struct DisallowGarbageCollection{dummy : i32}
impl v8::internal::Context{
  pub fn native_context(&self) -> &v8::internal::Context{self}
  pub fn intl_date_time_format_function(&self) -> v8::internal::JSFunction{v8::internal::JSFunction{dummy:1}}
}
impl v8::internal::Factory{
    pub fn NewFastOrSlowJSObjectFromMap(&self, _map :v8::internal::DirectHandle<v8::internal::Map>)->v8::internal::JSObject{v8::internal::JSObject{}}
    pub fn locale_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn calendar_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn numberingSystem_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn timeZone_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn hourCycle_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn hour12_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn weekday_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn era_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn year_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn month_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn day_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn hour_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn minute_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn second_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn fractionalSecondDigits_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn timeZoneName_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn full_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn long_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn medium_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn short_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn true_value(&self) -> v8::internal::Object{v8::internal::Object{}}
    pub fn false_value(&self) -> v8::internal::Object{v8::internal::Object{}}
     pub fn undefined_value(&self) -> v8::internal::Object{v8::internal::Object{}}
      pub fn NewStringFromAsciiChecked(&self, str: *const i8)->v8::internal::String{v8::internal::String{}}
            pub fn iso8601_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn Invalid_Date_string(&self) -> v8::internal::String{v8::internal::String{}}
    pub fn NewNumberFromInt64(&self,val: i64)->v8::internal::Object{v8::internal::Object{}}
                pub fn relatedYear_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn yearName_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
        pub fn fractionalSecond_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn NewNumberFromInt(&self,val: i32)->v8::internal::Object{v8::internal::Object{}}
    pub fn source_string(&self) -> v8::internal::String{v8::internal::String{}}
     pub fn shared_string(&self) -> v8::internal::String{v8::internal::String{}}
         pub fn full_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn long_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn medium_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn short_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
            pub fn h11_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn h12_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn h23_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }
    pub fn h24_string(&self) -> v8::internal::String {
        v8::internal::String {}
    }

}
pub mod flags {

}

pub mod object_macros_undef {

}

// Converted from V8 C++ source files:
// Header: js-temporal-objects.h
// Implementation: js-temporal-objects.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_temporal_objects {
use crate::objects::js_objects::*;
use crate::objects::module::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;
pub struct JSTemporalCalendar {}
pub struct JSTemporalDuration {}
pub struct JSTemporalInstant {}
pub struct JSTemporalPlainDate {}
pub struct JSTemporalPlainDateTime {}
pub struct JSTemporalPlainMonthDay {}
pub struct JSTemporalPlainTime {}
pub struct JSTemporalPlainYearMonth {}
pub struct JSTemporalTimeZone {}
pub struct JSTemporalZonedDateTime {}
pub enum Unit {
    kNotPresent,
    kAuto,
    kYear,
    kMonth,
    kWeek,
    kDay,
    kHour,
    kMinute,
    kSecond,
    kMillisecond,
    kMicrosecond,
    kNanosecond,
}
struct UnbalancedTimeRecord {
    hour: f64,
    minute: f64,
    second: f64,
    millisecond: f64,
    microsecond: f64,
    nanosecond: f64,
}
pub struct DateRecord {
    year: i32,
    month: i32,
    day: i32,
}
pub struct TimeRecord {
    hour: i32,
    minute: i32,
    second: i32,
    millisecond: i32,
    microsecond: i32,
    nanosecond: i32,
}
struct DateTimeRecord {
    date: DateRecord,
    time: TimeRecord,
}
struct DateRecordWithCalendar {
    date: DateRecord,
    calendar: Rc<RefCell<dyn ObjectTrait>>,
}
struct TimeRecordWithCalendar {
    time: TimeRecord,
    calendar: Rc<RefCell<dyn ObjectTrait>>,
}
struct TimeZoneRecord {
    z: bool,
    offset_string: Rc<RefCell<dyn ObjectTrait>>,
    name: Rc<RefCell<dyn ObjectTrait>>,
}
struct DateTimeRecordWithCalendar {
    date: DateRecord,
    time: TimeRecord,
    time_zone: TimeZoneRecord,
    calendar: Rc<RefCell<dyn ObjectTrait>>,
}
struct InstantRecord {
    date: DateRecord,
    time: TimeRecord,
    offset_string: Rc<RefCell<dyn ObjectTrait>>,
}
pub struct DurationRecord {
    years: f64,
    months: f64,
    weeks: f64,
    time_duration: TimeDurationRecord,
}
struct TimeDurationRecord {
    days: f64,
    hours: f64,
    minutes: f64,
    seconds: f64,
    milliseconds: f64,
    microseconds: f64,
    nanoseconds: f64,
}
struct DurationRecordWithRemainder {
    record: DurationRecord,
    remainder: f64,
}
struct DateDurationRecord {
    years: f64,
    months: f64,
    weeks: f64,
    days: f64,
}
enum Disambiguation {
    kCompatible,
    kEarlier,
    kLater,
    kReject,
}
enum ShowOverflow {
    kConstrain,
    kReject,
}
enum ShowCalendar {
    kAuto,
    kAlways,
    kNever,
}
enum ShowTimeZone {
    kAuto,
    kNever,
}
enum ShowOffset {
    kAuto,
    kNever,
}
enum Precision {
    k0,
    k1,
    k2,
    k3,
    k4,
    k5,
    k6,
    k7,
    k8,
    k9,
    kAuto,
    kMinute,
}
enum Arithmetic {
    kAdd,
    kSubtract,
}
enum TimePreposition {
    kSince,
    kUntil,
}
enum Offset {
    kPrefer,
    kUse,
    kIgnore,
    kReject,
}
enum RoundingMode {
    kCeil,
    kFloor,
    kExpand,
    kTrunc,
    kHalfCeil,
    kHalfFloor,
    kHalfExpand,
    kHalfTrunc,
    kHalfEven,
}
enum UnsignedRoundingMode {
    kInfinity,
    kZero,
    kHalfInfinity,
    kHalfZero,
    kHalfEven,
}
enum MatchBehaviour {
    kMatchExactly,
    kMatchMinutes,
}
enum UnitGroup {
    kDate,
    kTime,
    kDateTime,
}
struct DifferenceSettings {
    smallest_unit: Unit,
    largest_unit: Unit,
    rounding_mode: RoundingMode,
    rounding_increment: f64,
    options: Rc<RefCell<dyn JSReceiverTrait>>,
}
enum DisallowedUnitsInDifferenceSettings {
    kNone,
    kWeekAndDay,
}
enum Skip {
  kSkip,
}
// struct Options {}
// struct TimeZone {}
// struct Duration {}
struct Internal {}
}

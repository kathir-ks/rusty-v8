// TODO: Missing implementations for:
// - Isolate
// - HandleScope
// - DirectHandle
// - JSFunction
// - JSReceiver
// - JSDate
// - Object
// - String
// - NumberValue
// - ToPrimitive
// - Object::ToNumber
// - Object::ToString
// - NewRangeError
// - NewTypeError
// - Execution::Call
// - BigInt
// - Factory
// - DateCache
// - ReadOnlyRoots
// - Arguments
// - MessageTemplate
// - ToDateString
// - base::VectorOf
// - Cast
// - IsUndefined
// - IsString
// - IsJSDate
// - IsCallable
// - Smi
// - temporal::CreateTemporalInstant
// - ASSIGN_RETURN_FAILURE_ON_EXCEPTION
// - CHECK_RECEIVER
// - RETURN_RESULT_OR_FAILURE
// - THROW_NEW_ERROR_RETURN_FAILURE
// - JSDateTimeFormat::ToLocaleDateTime
// - Object::GetProperty
// - Object::ToObject
// - Object::NumberValue
// - Object::SameNumberValue
// - DoubleToInteger
// - ToPrimitiveHint

// TODO: Implement the date/time functions and constants
const K_MAX_TIME_BEFORE_UTC_IN_MS: f64 = 8640000000000.0; // Example value

mod date {
    // Implementations for MakeDay, MakeTime, MakeDate
    pub fn make_day(year: f64, month: f64, date: f64) -> f64 {
        // Placeholder implementation
        0.0
    }

    pub fn make_time(hours: f64, minutes: f64, seconds: f64, ms: f64) -> f64 {
        // Placeholder implementation
        0.0
    }

    pub fn make_date(day: f64, time: f64) -> f64 {
        // Placeholder implementation
        0.0
    }

    pub fn parse_date_time_string(_isolate: &Isolate, _string: &String) -> f64 {
        // Placeholder implementation
        0.0
    }
}

use std::f64;
use std::ops::Deref;

//use std::os::raw::c_char;

#[macro_export]
macro_rules! CHECK_RECEIVER {
    ($obj_type:ident, $receiver:ident, $method_name:literal) => {
        /*
        if !($receiver is $obj_type) {
            // TODO: Replace with proper error handling when available
            panic!(concat!($method_name, " called on non-object of type ", stringify!($obj_type)));
        }
        */
    };
}

#[macro_export]
macro_rules! ASSIGN_RETURN_FAILURE_ON_EXCEPTION {
    ($isolate:ident, $var:ident, $expression:expr) => {
        /*
        let result = $expression;
        if result.is_err() {
            return result.unwrap_err(); // Assuming you want to return the error
        }
        let $var = result.unwrap();
        */
    };
}

#[macro_export]
macro_rules! RETURN_RESULT_OR_FAILURE {
    ($isolate:ident, $expression:expr) => {
        /*
        let result = $expression;
        match result {
            Ok(value) => return value,
            Err(err) => return err,
        }
        */
    };
}

#[macro_export]
macro_rules! THROW_NEW_ERROR_RETURN_FAILURE {
    ($isolate:ident, $expression:expr) => {
        /*
        return $expression;
        */
    };
}

#[derive(Debug)]
struct Isolate {}

impl Isolate {
    fn new() -> Self {
        Isolate {}
    }

    fn date_cache(&self) -> &DateCache {
        unimplemented!()
    }

    fn factory(&self) -> &Factory {
        unimplemented!()
    }

    fn count_usage(_feature: i32) {
        unimplemented!()
    }
}

struct HandleScope<'a> {
    isolate: &'a Isolate,
}

impl<'a> HandleScope<'a> {
    fn new(isolate: &'a Isolate) -> Self {
        HandleScope { isolate }
    }
}

struct DirectHandle<T> {
    value: T,
}

impl<T> Deref for DirectHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DirectHandle<T> {
    fn new(value: T) -> Self {
        DirectHandle { value }
    }
}

trait Object {}

trait String: Object {}

trait Number: Object {}

trait JSReceiver: Object {}

trait JSFunction: JSReceiver {}

trait JSDate: Object {
    fn value(&self) -> f64;
    fn set_value(&mut self, value: f64);
    fn set_nan_value(&mut self);
}

struct Arguments {}

impl Arguments {
    fn length(&self) -> i32 {
        0
    }
    fn at(&self, _index: i32) -> DirectHandle<dyn Object> {
        unimplemented!()
    }
    fn at_or_undefined<'a>(&self, _isolate: &'a Isolate, _index: i32) -> DirectHandle<dyn Object> {
        unimplemented!()
    }
    fn new_target(&self) -> DirectHandle<dyn Object> {
        unimplemented!()
    }
    fn target(&self) -> DirectHandle<dyn JSFunction> {
        unimplemented!()
    }
}

struct Factory {}

impl Factory {
    fn new_string_from_utf8(_vec: String) -> Result<DirectHandle<dyn String>, ()> {
        unimplemented!()
    }
    fn new_number_from_int64(_value: i64) -> DirectHandle<dyn Number> {
        unimplemented!()
    }
    fn new_number(_value: f64) -> DirectHandle<dyn Number> {
        unimplemented!()
    }
}

struct DateCache {}

impl DateCache {
    fn to_utc(&self, time_val: i64) -> f64 {
        unimplemented!()
    }
    fn try_time_clip(time_val: &mut f64) -> bool {
        unimplemented!()
    }
    fn to_local(&self, time_ms: i64) -> i64 {
        unimplemented!()
    }
    fn days_from_time(&self, local_time_ms: i64) -> i32 {
        unimplemented!()
    }
    fn time_in_day(&self, local_time_ms: i64, days: i32) -> i32 {
        unimplemented!()
    }
    fn year_month_day_from_days(&self, days: i32, year: &mut i32, month: &mut i32, day: &mut i32) {
        unimplemented!()
    }
}

struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    fn nan_value(&self) -> DirectHandle<dyn Number> {
        unimplemented!()
    }
    fn null_value(&self) -> DirectHandle<dyn Object> {
        unimplemented!()
    }
}

mod temporal {
    use super::*;
    pub fn create_temporal_instant(_isolate: &Isolate, _ns: DirectHandle<BigInt>) -> Result<DirectHandle<dyn Object>, ()> {
        unimplemented!()
    }
}

mod js_date_time_format {
    use super::*;
    pub fn to_locale_date_time(
        _isolate: &Isolate,
        _date: &DirectHandle<dyn JSDate>,
        _locales: DirectHandle<dyn Object>,
        _options: DirectHandle<dyn Object>,
        _required: i32,
        _defaults: i32,
        _method_name: &str,
    ) -> Result<DirectHandle<dyn Object>, ()> {
        unimplemented!()
    }
}

// ES6 section 20.3 Date Objects

mod builtins_date {
    use super::*;
    use super::date::*;

    mod private {
        use super::*;
        use super::date::*;
        // Implementations for SetLocalDateValue and SetDateValue

        fn set_local_date_value(
            isolate: &Isolate,
            date: &mut DirectHandle<dyn JSDate>,
            time_val: f64,
        ) -> DirectHandle<dyn Object> {
            if time_val >= -K_MAX_TIME_BEFORE_UTC_IN_MS && time_val <= K_MAX_TIME_BEFORE_UTC_IN_MS {
                let mut time_val = isolate.date_cache().to_utc(time_val as i64);
                if isolate.date_cache().try_time_clip(&mut time_val) {
                    date.set_value(time_val);
                    return isolate.factory().new_number(time_val);
                }
            }
            date.set_nan_value();
            ReadOnlyRoots {}.nan_value()
        }

        fn set_date_value(
            isolate: &Isolate,
            date: &mut DirectHandle<dyn JSDate>,
            time_val: f64,
        ) -> DirectHandle<dyn Object> {
            let mut time_val_local = time_val;
            if isolate.date_cache().try_time_clip(&mut time_val_local) {
                date.set_value(time_val_local);
                return isolate.factory().new_number(time_val_local);
            }
            date.set_nan_value();
            ReadOnlyRoots {}.nan_value()
        }
    }

    use private::*;

    fn js_date_current_time_value(_isolate: &Isolate) -> i64 {
        // Placeholder implementation
        0
    }

    type DateBuffer = String;

    fn to_date_string(
        _time_val: f64,
        _date_cache: &DateCache,
        _mode: i32,
    ) -> DateBuffer {
        // Placeholder implementation
        String::from("")
    }

    // ES #sec-date-constructor
    pub fn date_constructor(
        isolate: &Isolate,
        args: &Arguments,
    ) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        if isolate.factory().is_undefined(*args.new_target(), isolate) {
            let time_val = js_date_current_time_value(isolate) as f64;
            let buffer = to_date_string(time_val, isolate.date_cache(), 0);
            return isolate.factory().new_string_from_utf8(buffer).unwrap();
        }

        // [Construct]
        let argc = args.length() - 1;
        let target = args.target();
        let new_target = args.new_target();
        let time_val: f64;

        if argc == 0 {
            time_val = js_date_current_time_value(isolate) as f64;
        } else if argc == 1 {
            let value = args.at(1);
            time_val = 0.0;
            /*
            if value.is_js_date() {
                time_val = value.cast::<JSDate>().value();
            } else {
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, value, Object::to_primitive(isolate, value));
                if value.is_string() {
                    time_val = date::parse_date_time_string(isolate, value.cast::<String>());
                } else {
                    ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, value, Object::to_number(isolate, value));
                    time_val = value.number_value();
                }
            }
            */
        } else {
            let year_object = args.at(1);
            let month_object = args.at(2);
            let year = 0.0; //year_object.number_value();
            let month = 0.0; //month_object.number_value();
            let date = 1.0;
            let hours = 0.0;
            let minutes = 0.0;
            let seconds = 0.0;
            let ms = 0.0;

            time_val = 0.0;

            if !year.is_nan() {
                //let y = DoubleToInteger(year);
                //if 0.0 <= y && y <= 99.0 {
                //    year = 1900.0 + y;
                //}
            }
            let day = date::make_day(year, month, date);
            let time = date::make_time(hours, minutes, seconds, ms);
            //time_val = MakeDate(day, time);

            /*
            if time_val >= -DateCache::kMaxTimeBeforeUTCInMs &&
                time_val <= DateCache::kMaxTimeBeforeUTCInMs {
                time_val = isolate.date_cache().to_utc(time_val as i64);
            } else {
                time_val = f64::NAN;
            }
            */
        }

        unimplemented!()
        //JSDate::new(target, new_target, time_val)
    }

    // ES6 section 20.3.3.1 Date.now ( )
    pub fn date_now(isolate: &Isolate) -> DirectHandle<dyn Number> {
        let scope = HandleScope::new(isolate);
        isolate.factory().new_number_from_int64(js_date_current_time_value(isolate))
    }

    // ES6 section 20.3.3.2 Date.parse ( string )
    pub fn date_parse(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Number> {
        let scope = HandleScope::new(isolate);
        let string = args.at_or_undefined(isolate, 1);
        unimplemented!()
        //let string = Object::to_string(isolate, string);
        //isolate.factory().new_number(date::parse_date_time_string(isolate, string))
    }

    // ES6 section 20.3.3.4 Date.UTC (year,month,date,hours,minutes,seconds,ms)
    pub fn date_utc(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        let argc = args.length() - 1;
        let mut year = f64::NAN;
        let mut month = 0.0;
        let mut date = 1.0;
        let mut hours = 0.0;
        let mut minutes = 0.0;
        let mut seconds = 0.0;
        let mut ms = 0.0;

        if argc >= 1 {
            let year_object = args.at(1);
            unimplemented!()
            //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, year_object, Object::to_number(isolate, args.at(1)));
            //year = year_object.number_value();
            if argc >= 2 {
                let month_object = args.at(2);
                unimplemented!()
                //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, month_object, Object::to_number(isolate, args.at(2)));
                //month = month_object.number_value();
                if argc >= 3 {
                    let date_object = args.at(3);
                    unimplemented!()
                    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, date_object, Object::to_number(isolate, args.at(3)));
                    //date = date_object.number_value();
                    if argc >= 4 {
                        let hours_object = args.at(4);
                        unimplemented!()
                        //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, hours_object, Object::to_number(isolate, args.at(4)));
                        //hours = hours_object.number_value();
                        if argc >= 5 {
                            let minutes_object = args.at(5);
                            unimplemented!()
                            //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, minutes_object, Object::to_number(isolate, args.at(5)));
                            //minutes = minutes_object.number_value();
                            if argc >= 6 {
                                let seconds_object = args.at(6);
                                unimplemented!()
                                //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, seconds_object, Object::to_number(isolate, args.at(6)));
                                //seconds = seconds_object.number_value();
                                if argc >= 7 {
                                    let ms_object = args.at(7);
                                    unimplemented!()
                                    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, ms_object, Object::to_number(isolate, args.at(7)));
                                    //ms = ms_object.number_value();
                                }
                            }
                        }
                    }
                }
            }
        }

        if !year.is_nan() {
            //let y = DoubleToInteger(year);
            //if 0.0 <= y && y <= 99.0 {
            //    year = 1900.0 + y;
            //}
        }

        let day = date::make_day(year, month, date);
        let time = date::make_time(hours, minutes, seconds, ms);
        let mut value = date::make_date(day, time);
        unimplemented!()
        /*
        if DateCache::try_time_clip(&mut value) {
            return isolate.factory().new_number(value);
        }
        ReadOnlyRoots {}.nan_value()
        */
    }

    // ES6 section 20.3.4.20 Date.prototype.setDate ( date )
    pub fn date_prototype_set_date(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setDate");
        //let value = args.at_or_undefined(isolate, 1);
        //ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, value, Object::to_number(isolate, value));

        /*
        let mut time_val = date.value();
        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let days = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, days);
            let mut year = 0;
            let mut month = 0;
            let mut day = 0;
            isolate.date_cache().year_month_day_from_days(days, &mut year, &mut month, &mut day);
            time_val = MakeDate(MakeDay(year as f64, month as f64, value.number_value()), time_within_day as f64);
        }

        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.21 Date.prototype.setFullYear (year, month, date)
    pub fn date_prototype_set_full_year(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setFullYear");
        /*
        let argc = args.length() - 1;
        let year = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, year, Object::to_number(isolate, year));
        let year_double = year.number_value();
        let mut month_double = 0.0;
        let mut day_double = 1.0;
        let mut time_within_day = 0;

        if !date.value().is_nan() {
            let time_ms = date.value() as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let days = isolate.date_cache().days_from_time(local_time_ms);
            time_within_day = isolate.date_cache().time_in_day(local_time_ms, days);
            let mut year_int = 0;
            let mut month_int = 0;
            let mut day_int = 0;
            isolate.date_cache().year_month_day_from_days(days, &mut year_int, &mut month_int, &mut day_int);
            month_double = month_int as f64;
            day_double = day_int as f64;
        }

        if argc >= 2 {
            let month = args.at(2);
            ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, month, Object::to_number(isolate, month));
            month_double = month.number_value();

            if argc >= 3 {
                let day = args.at(3);
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, day, Object::to_number(isolate, day));
                day_double = day.number_value();
            }
        }

        let time_val = MakeDate(MakeDay(year_double, month_double, day_double), time_within_day as f64);
        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.22 Date.prototype.setHours(hour, min, sec, ms)
    pub fn date_prototype_set_hours(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setHours");
        /*
        let argc = args.length() - 1;
        let hour = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, hour, Object::to_number(isolate, hour));
        let h = hour.number_value();
        let mut time_val = date.value();

        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let day = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, day);
            let mut m = (time_within_day / (60 * 1000)) % 60;
            let mut s = (time_within_day / 1000) % 60;
            let mut milli = time_within_day % 1000;

            if argc >= 2 {
                let min = args.at(2);
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, min, Object::to_number(isolate, min));
                m = min.number_value();

                if argc >= 3 {
                    let sec = args.at(3);
                    ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, sec, Object::to_number(isolate, sec));
                    s = sec.number_value();

                    if argc >= 4 {
                        let ms = args.at(4);
                        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, ms, Object::to_number(isolate, ms));
                        milli = ms.number_value();
                    }
                }
            }

            time_val = MakeDate(day as f64, MakeTime(h, m as f64, s as f64, milli as f64));
        }

        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.23 Date.prototype.setMilliseconds(ms)
    pub fn date_prototype_set_milliseconds(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setMilliseconds");
        /*
        let ms = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, ms, Object::to_number(isolate, ms));
        let mut time_val = date.value();

        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let day = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, day);
            let h = time_within_day / (60 * 60 * 1000);
            let m = (time_within_day / (60 * 1000)) % 60;
            let s = (time_within_day / 1000) % 60;

            time_val = MakeDate(day as f64, MakeTime(h as f64, m as f64, s as f64, ms.number_value()));
        }

        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.24 Date.prototype.setMinutes ( min, sec, ms )
    pub fn date_prototype_set_minutes(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setMinutes");
        /*
        let argc = args.length() - 1;
        let min = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, min, Object::to_number(isolate, min));
        let mut time_val = date.value();

        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let day = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, day);
            let h = time_within_day / (60 * 60 * 1000);
            let m = min.number_value();
            let mut s = (time_within_day / 1000) % 60;
            let mut milli = time_within_day % 1000;

            if argc >= 2 {
                let sec = args.at(2);
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, sec, Object::to_number(isolate, sec));
                s = sec.number_value();

                if argc >= 3 {
                    let ms = args.at(3);
                    ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, ms, Object::to_number(isolate, ms));
                    milli = ms.number_value();
                }
            }

            time_val = MakeDate(day as f64, MakeTime(h as f64, m, s as f64, milli as f64));
        }

        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.25 Date.prototype.setMonth ( month, date )
    pub fn date_prototype_set_month(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, this_date, "Date.prototype.setMonth");
        /*
        let argc = args.length() - 1;
        let month = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, month, Object::to_number(isolate, month));
        let mut time_val = this_date.value();

        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let days = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, days);
            let mut year = 0;
            let mut unused = 0;
            let mut day = 0;
            isolate.date_cache().year_month_day_from_days(days, &mut year, &mut unused, &mut day);
            let m = month.number_value();
            let mut dt = day as f64;

            if argc >= 2 {
                let date = args.at(2);
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, date, Object::to_number(isolate, date));
                dt = date.number_value();
            }

            time_val = MakeDate(MakeDay(year as f64, m, dt), time_within_day as f64);
        }

        SetLocalDateValue(isolate, this_date, time_val)
        */
    }

    // ES6 section 20.3.4.26 Date.prototype.setSeconds ( sec, ms )
    pub fn date_prototype_set_seconds(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setSeconds");
        /*
        let argc = args.length() - 1;
        let sec = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, sec, Object::to_number(isolate, sec));
        let mut time_val = date.value();

        if !time_val.is_nan() {
            let time_ms = time_val as i64;
            let local_time_ms = isolate.date_cache().to_local(time_ms);
            let day = isolate.date_cache().days_from_time(local_time_ms);
            let time_within_day = isolate.date_cache().time_in_day(local_time_ms, day);
            let h = time_within_day / (60 * 60 * 1000);
            let m = (time_within_day / (60 * 1000)) % 60;
            let s = sec.number_value();
            let mut milli = time_within_day % 1000;

            if argc >= 2 {
                let ms = args.at(2);
                ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, ms, Object::to_number(isolate, ms));
                milli = ms.number_value();
            }

            time_val = MakeDate(day as f64, MakeTime(h as f64, m as f64, s, milli as f64));
        }

        SetLocalDateValue(isolate, date, time_val)
        */
    }

    // ES6 section 20.3.4.27 Date.prototype.setTime ( time )
    pub fn date_prototype_set_time(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_RECEIVER!(JSDate, date, "Date.prototype.setTime");
        /*
        let value = args.at_or_undefined(isolate, 1);
        ASSIGN_RETURN_FAILURE_ON_EXCEPTION!(isolate, value, Object::to_number(isolate, value));
        let value_double = value.number_value();

        // Don't use SetDateValue here, since we might already have a tagged value for
        // the time, and we don't want to reallocate it.
        let mut clipped_value = value_double;
        if DateCache::try_time_clip(&mut clipped_value) {
            date.set_value(clipped_value);
            // If the clipping didn't change the value (i.e. the value was already an
            // integer), we can reuse the incoming value for the return value.
            // Otherwise, we have to allocate a new value. Make sure to use
            // SameNumberValue so that -0 is _not_ treated as equal to the 0.
            if Object::SameNumberValue(clipped_value, value_double) {
                return value;
            }
            return isolate.factory().new_number(clipped_value);
        }

        date.set_nan_value();
        ReadOnlyRoots {}.nan_value()
        */
    }

    // ES6 section 20.3.4.28 Date.prototype.setUTCDate ( date )
    pub fn date_prototype_set_utc_date(isolate: &Isolate, args: &Arguments) -> DirectHandle<dyn Object> {
        let scope = HandleScope::new(isolate);
        unimplemented!()
        //CHECK_
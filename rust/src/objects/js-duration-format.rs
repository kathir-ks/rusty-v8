// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]

// #![cfg(feature = "intl")] //Conditional compilation based on feature

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use icu::number::{
    IntegerWidth, LocalizedNumberFormatter, NumberFormatRoundingMode, NumberGroupingStrategy,
    Precision, SignDisplay, UnlocalizedNumberFormatter,
};
use icu::string::StringConversion;
use icu::ulistformatter::{ListFormatter, ListFormatterType, ListFormatterWidth};
use icu::{
    locid::{Locale, LocaleNegotiation},
    number::FormattedNumber,
    ErrorCode, UErrorCode,
};
use icu_locid::canonicalize;

pub mod temporal {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct TimeDuration {
        pub days: f64,
        pub hours: f64,
        pub minutes: f64,
        pub seconds: f64,
        pub milliseconds: f64,
        pub microseconds: f64,
        pub nanoseconds: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct DurationRecord {
        pub years: f64,
        pub months: f64,
        pub weeks: f64,
        pub time_duration: TimeDuration,
    }

    impl DurationRecord {
        pub fn sign(self) -> i32 {
            if self.years.is_sign_negative()
                || self.months.is_sign_negative()
                || self.weeks.is_sign_negative()
                || self.time_duration.days.is_sign_negative()
                || self.time_duration.hours.is_sign_negative()
                || self.time_duration.minutes.is_sign_negative()
                || self.time_duration.seconds.is_sign_negative()
                || self.time_duration.milliseconds.is_sign_negative()
                || self.time_duration.microseconds.is_sign_negative()
                || self.time_duration.nanoseconds.is_sign_negative()
            {
                -1
            } else {
                1
            }
        }
    }

    // Placeholder for ToPartialDuration and IsValidDuration, replace with actual implementations
    pub fn to_partial_duration(
        isolate: &Isolate,
        input: &Object,
        default_value: DurationRecord,
    ) -> Result<DurationRecord, Error> {
        // Dummy implementation.  Replace with real logic.
        Ok(default_value)
    }

    pub fn is_valid_duration(isolate: &Isolate, record: DurationRecord) -> bool {
        //Dummy implementation.  Replace with real logic.
        true
    }

    pub fn ToPartialDuration(
        isolate: &Isolate,
        input: &Object,
        default_value: DurationRecord,
    ) -> Result<DurationRecord, Error> {
        //Dummy implementation.  Replace with real logic.
        Ok(default_value)
    }

    pub fn IsValidDuration(isolate: &Isolate, record: DurationRecord) -> bool {
        //Dummy implementation.  Replace with real logic.
        true
    }
}

pub mod factory {
    use super::*;
    pub struct Factory {
        // Placeholder strings
        pub long_string: String,
        pub short_string: String,
        pub narrow_string: String,
        pub numeric_string: String,
        pub two_digit_string: String,
        pub fractionalDigits_string: String,
        pub auto_string: String,
        pub always_string: String,
        pub locale_string: String,
        pub numberingSystem_string: String,
        pub style_string: String,
        pub years_string: String,
        pub months_string: String,
        pub weeks_string: String,
        pub days_string: String,
        pub hours_string: String,
        pub minutes_string: String,
        pub seconds_string: String,
        pub milliseconds_string: String,
        pub microseconds_string: String,
        pub nanoseconds_string: String,
        pub yearsDisplay_string: String,
        pub monthsDisplay_string: String,
        pub weeksDisplay_string: String,
        pub daysDisplay_string: String,
        pub hoursDisplay_string: String,
        pub minutesDisplay_string: String,
        pub secondsDisplay_string: String,
        pub millisecondsDisplay_string: String,
        pub microsecondsDisplay_string: String,
        pub nanosecondsDisplay_string: String,
        pub literal_string: String,
        pub object_string: String,
    }

    impl Factory {
        pub fn new() -> Self {
            Factory {
                long_string: "long".to_string(),
                short_string: "short".to_string(),
                narrow_string: "narrow".to_string(),
                numeric_string: "numeric".to_string(),
                two_digit_string: "2-digit".to_string(),
                fractionalDigits_string: "fractionalDigits".to_string(),
                auto_string: "auto".to_string(),
                always_string: "always".to_string(),
                locale_string: "locale".to_string(),
                numberingSystem_string: "numberingSystem".to_string(),
                style_string: "style".to_string(),
                years_string: "years".to_string(),
                months_string: "months".to_string(),
                weeks_string: "weeks".to_string(),
                days_string: "days".to_string(),
                hours_string: "hours".to_string(),
                minutes_string: "minutes".to_string(),
                seconds_string: "seconds".to_string(),
                milliseconds_string: "milliseconds".to_string(),
                microseconds_string: "microseconds".to_string(),
                nanoseconds_string: "nanoseconds".to_string(),
                yearsDisplay_string: "yearsDisplay".to_string(),
                monthsDisplay_string: "monthsDisplay".to_string(),
                weeksDisplay_string: "weeksDisplay".to_string(),
                daysDisplay_string: "daysDisplay".to_string(),
                hoursDisplay_string: "hoursDisplay".to_string(),
                minutesDisplay_string: "minutesDisplay".to_string(),
                secondsDisplay_string: "secondsDisplay".to_string(),
                millisecondsDisplay_string: "millisecondsDisplay".to_string(),
                microsecondsDisplay_string: "microsecondsDisplay".to_string(),
                nanosecondsDisplay_string: "nanosecondsDisplay".to_string(),
                literal_string: "literal".to_string(),
                object_string: "object".to_string(),
            }
        }

        pub fn NewJSObject(&self, object_function: ()) -> JSObject {
            JSObject::new()
        }
        pub fn NewStringFromAsciiChecked(&self, s: &str) -> String {
            s.to_string()
        }
        pub fn NewFastOrSlowJSObjectFromMap(&self, map: DirectHandle<Map>) -> JSObject {
            JSObject::new() // Replace with real logic.
        }
    }
}

pub mod intl {
    use super::*;
    pub enum MatcherOption {
        Lookup,
        BestFit,
    }

    pub fn canonicalize_locale_list(
        isolate: &Isolate,
        locales: &Object,
    ) -> Result<Vec<String>, Error> {
        // Placeholder implementation. Replace with real logic.
        let mut result = Vec::new();
        // Assuming the object is an array of locales
        // You'll need to iterate through the array and canonicalize each locale
        Ok(result)
    }

    pub fn get_locale_matcher(
        isolate: &Isolate,
        options: &JSReceiver,
        method_name: &str,
    ) -> Result<MatcherOption, Error> {
        // Placeholder implementation.  Replace with real logic.
        Ok(MatcherOption::BestFit)
    }

    pub fn get_numbering_system(
        isolate: &Isolate,
        options: &JSReceiver,
        method_name: &str,
        numbering_system_str: &mut Option<String>,
    ) -> Result<bool, Error> {
        // Placeholder implementation.  Replace with real logic.
        Ok(true)
    }

    #[derive(Debug)]
    pub struct ResolvedLocale {
        pub icu_locale: icu::locid::Locale,
        pub extensions: HashMap<String, String>,
    }

    pub fn resolve_locale(
        isolate: &Isolate,
        available_locales: &HashSet<String>,
        requested_locales: Vec<String>,
        matcher: MatcherOption,
        relevant_extension_keys: std::collections::HashSet<String>,
    ) -> Result<ResolvedLocale, Error> {
        // Placeholder implementation.  Replace with real logic.

        let mut error_code = ErrorCode::new();

        let r = Locale::for_language_tag("en").expect("failed to parse locale");

        Ok(ResolvedLocale {
            icu_locale: r,
            extensions: HashMap::new(),
        })
    }

    pub fn is_valid_numbering_system(numbering_system: &str) -> bool {
        // Placeholder implementation.  Replace with real logic.
        true
    }

    pub fn get_numbering_system(locale: icu::locid::Locale) -> String {
        // Placeholder implementation.  Replace with real logic.
        "latn".to_string()
    }

    pub fn to_language_tag(locale: &icu::locid::Locale) -> Result<String, Error> {
        // Placeholder implementation. Replace with real logic.
        Ok(locale.to_string())
    }

    pub fn to_string(isolate: &Isolate, s: icu::UnicodeString) -> Result<String, Error> {
        // Placeholder implementation. Replace with real logic.
        Ok("".to_string())
    }

    pub fn to_string(
        isolate: &Isolate,
        s: icu::UnicodeString,
        start: i32,
        limit: i32,
    ) -> Result<String, Error> {
        // Placeholder implementation. Replace with real logic.
        Ok("".to_string())
    }

    pub fn add_element(
        isolate: &Isolate,
        array: &JSArray,
        index: i32,
        literal_string: String,
        substring: String,
    ) -> bool {
        // Placeholder implementation. Replace with real logic.
        true
    }

    pub fn add_number_elements(
        isolate: &Isolate,
        formatted: icu::number::FormattedNumber,
        array: &JSArray,
        index: i32,
        type_string: String,
    ) -> Result<i32, Error> {
        // Placeholder implementation. Replace with real logic.
        Ok(index)
    }

    pub fn FormattedToString(isolate: &Isolate, formatted: icu::FormattedValue) -> Result<String, Error> {
       Ok("".to_string())
    }
}

// Dummy implementations for V8 types
pub type Isolate = usize;
pub type Handle<T> = usize;
pub type DirectHandle<T> = usize;
pub type MaybeDirectHandle<T> = Result<usize, Error>;
pub type String = std::string::String;
pub type JSReceiver = usize;
pub type Object = usize;
pub type JSObject = usize;
pub type Map = usize;
pub type JSArray = usize;
pub type Smi = usize;

impl JSObject {
    pub fn new() -> Self {
        0
    }
    pub fn ValidateElements(_array: JSArray) {}
}

#[derive(Debug)]
pub enum Error {
    RangeError,
    TypeError,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RangeError => write!(f, "RangeError"),
            Error::TypeError => write!(f, "TypeError"),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

// Helper functions for throwing errors
fn throw_new_error<T>(isolate: &Isolate, error: Error) -> Result<T, Error> {
    Err(error)
}

fn NewRangeError(message: MessageTemplate, object_string: String, input: Object) -> Error {
    Error::RangeError
}

fn NewTypeError(message: MessageTemplate) -> Error {
    Error::TypeError
}

#[derive(Debug)]
pub enum MessageTemplate {
    kInvalid,
    kIcuError,
    // Add other message types as needed
}

// Option utils
fn GetOptionsObject(isolate: &Isolate, input_options: DirectHandle<Object>, method_name: &str) -> Result<DirectHandle<JSReceiver>, Error> {
    Ok(0) // Dummy value, replace with actual logic
}

fn GetStringOption<T>(
    isolate: &Isolate,
    options: &JSReceiver,
    property_name: &str,
    method_name: &str,
    value_strings: std::collections::HashSet<&str>,
    value_enums: std::collections::HashSet<T>,
    default_value: T,
) -> Result<T, Error>
where
    T: Copy + std::cmp::PartialEq,
    T: From<JSDurationFormat::Style>,
    T: From<JSDurationFormat::FieldStyle>,
    T: From<JSDurationFormat::Display>,
{
    // Placeholder implementation.  Replace with real logic.
    Ok(default_value)
}

fn GetNumberOption(
    isolate: &Isolate,
    options: &JSReceiver,
    property_name: String,
    min: i32,
    max: i32,
    default_value: i32,
) -> Result<i32, Error> {
    // Placeholder implementation.  Replace with real logic.
    Ok(default_value)
}

pub mod js_duration_format {
    use super::*;
    const UNUM_ROUND_HALFUP: icu::number::NumberFormatRoundingMode =
        icu::number::NumberFormatRoundingMode::HalfEven;
    const UNUM_GROUPING_OFF: icu::number::NumberGroupingStrategy =
        icu::number::NumberGroupingStrategy::Off;
    const UNUM_SIGN_NEVER: icu::number::SignDisplay = icu::number::SignDisplay::Never;
    const UNUM_ROUND_DOWN: icu::number::NumberFormatRoundingMode =
        icu::number::NumberFormatRoundingMode::Down;
    const ULISTFMT_ELEMENT_FIELD: i32 = icu::ulistformatter::Field::Element as i32;
    const kUndefinedFractionalDigits: i32 = -1;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Style {
        kLong,
        kShort,
        kNarrow,
        kDigital,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum FieldStyle {
        kLong,
        kShort,
        kNarrow,
        kNumeric,
        k2Digit,
        kFractional,
        kUndefined,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Display {
        kAuto,
        kAlways,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Separator {
        kColon,
        kFullStop,
        kFullwidthColon,
        kArabicDecimalSeparator,
    }

    pub struct JSDurationFormat {
        style_flags: u32,
        display_flags: u32,
        style: Style,
        years_style: FieldStyle,
        months_style: FieldStyle,
        weeks_style: FieldStyle,
        days_style: FieldStyle,
        hours_style: FieldStyle,
        minutes_style: FieldStyle,
        seconds_style: FieldStyle,
        milliseconds_style: FieldStyle,
        microseconds_style: FieldStyle,
        nanoseconds_style: FieldStyle,
        separator: Separator,

        years_display: Display,
        months_display: Display,
        weeks_display: Display,
        days_display: Display,
        hours_display: Display,
        minutes_display: Display,
        seconds_display: Display,
        milliseconds_display: Display,
        microseconds_display: Display,
        nanoseconds_display: Display,

        fractional_digits: i32,
        icu_locale: Rc<icu::locid::Locale>,
        icu_number_formatter: Rc<icu::number::LocalizedNumberFormatter>,
    }

    impl JSDurationFormat {
        pub fn new() -> Self {
            JSDurationFormat {
                style_flags: 0,
                display_flags: 0,
                style: Style::kShort, // Provide a default value
                years_style: FieldStyle::kUndefined,
                months_style: FieldStyle::kUndefined,
                weeks_style: FieldStyle::kUndefined,
                days_style: FieldStyle::kUndefined,
                hours_style: FieldStyle::kUndefined,
                minutes_style: FieldStyle::kUndefined,
                seconds_style: FieldStyle::kUndefined,
                milliseconds_style: FieldStyle::kUndefined,
                microseconds_style: FieldStyle::kUndefined,
                nanoseconds_style: FieldStyle::kUndefined,
                separator: Separator::kColon,

                years_display: Display::kAuto,
                months_display: Display::kAuto,
                weeks_display: Display::kAuto,
                days_display: Display::kAuto,
                hours_display: Display::kAuto,
                minutes_display: Display::kAuto,
                seconds_display: Display::kAuto,
                milliseconds_display: Display::kAuto,
                microseconds_display: Display::kAuto,
                nanoseconds_display: Display::kAuto,
                fractional_digits: kUndefinedFractionalDigits,
                icu_locale: Rc::new(Locale::for_language_tag("en").expect("failed to parse locale")),
                icu_number_formatter: Rc::new(
                    UnlocalizedNumberFormatter {}.locale(Locale::for_language_tag("en").expect("failed to parse locale"))
                ), // Provide a default value,
            }
        }
        pub fn set_style_flags(&mut self, flags: u32) {
            self.style_flags = flags;
        }
        pub fn set_display_flags(&mut self, flags: u32) {
            self.display_flags = flags;
        }
        pub fn set_style(&mut self, style: Style) {
            self.style = style;
        }
        pub fn set_years_style(&mut self, style: FieldStyle) {
            self.years_style = style;
        }
        pub fn set_months_style(&mut self, style: FieldStyle) {
            self.months_style = style;
        }
        pub fn set_weeks_style(&mut self, style: FieldStyle) {
            self.weeks_style = style;
        }
        pub fn set_days_style(&mut self, style: FieldStyle) {
            self.days_style = style;
        }
        pub fn set_hours_style(&mut self, style: FieldStyle) {
            self.hours_style = style;
        }
        pub fn set_minutes_style(&mut self, style: FieldStyle) {
            self.minutes_style = style;
        }
        pub fn set_seconds_style(&mut self, style: FieldStyle) {
            self.seconds_style = style;
        }
        pub fn set_milliseconds_style(&mut self, style: FieldStyle) {
            self.milliseconds_style = style;
        }
        pub fn set_microseconds_style(&mut self, style: FieldStyle) {
            self.microseconds_style = style;
        }
        pub fn set_nanoseconds_style(&mut self, style: FieldStyle) {
            self.nanoseconds_style = style;
        }
        pub fn set_separator(&mut self, separator: Separator) {
            self.separator = separator;
        }

        pub fn set_years_display(&mut self, display: Display) {
            self.years_display = display;
        }
        pub fn set_months_display(&mut self, display: Display) {
            self.months_display = display;
        }
        pub fn set_weeks_display(&mut self, display: Display) {
            self.weeks_display = display;
        }
        pub fn set_days_display(&mut self, display: Display) {
            self.days_display = display;
        }
        pub fn set_hours_display(&mut self, display: Display) {
            self.hours_display = display;
        }
        pub fn set_minutes_display(&mut self, display: Display) {
            self.minutes_display = display;
        }
        pub fn set_seconds_display(&mut self, display: Display) {
            self.seconds_display = display;
        }
        pub fn set_milliseconds_display(&mut self, display: Display) {
            self.milliseconds_display = display;
        }
        pub fn set_microseconds_display(&mut self, display: Display) {
            self.microseconds_display = display;
        }
        pub fn set_nanoseconds_display(&mut self, display: Display) {
            self.nanoseconds_display = display;
        }

        pub fn set_fractional_digits(&mut self, fractional_digits: i32) {
            self.fractional_digits = fractional_digits;
        }

        pub fn set_icu_locale(&mut self, locale: Rc<icu::locid::Locale>) {
            self.icu_locale = locale;
        }

        pub fn icu_locale(&self) -> &Rc<icu::locid::Locale> {
            &self.icu_locale
        }

        pub fn set_icu_number_formatter(
            &mut self,
            formatter: Rc<icu::number::LocalizedNumberFormatter>,
        ) {
            self.icu_number_formatter = formatter;
        }

        pub fn icu_number_formatter(&self) -> &Rc<icu::number::LocalizedNumberFormatter> {
            &self.icu_number_formatter
        }

        pub fn style(&self) -> Style {
            self.style
        }
        pub fn years_style(&self) -> FieldStyle {
            self.years_style
        }
        pub fn months_style(&self) -> FieldStyle {
            self.months_style
        }
        pub fn weeks_style(&self) -> FieldStyle {
            self.weeks_style
        }
        pub fn days_style(&self) -> FieldStyle {
            self.days_style
        }
        pub fn hours_style(&self) -> FieldStyle {
            self.hours_style
        }
        pub fn minutes_style(&self) -> FieldStyle {
            self.minutes_style
        }
        pub fn seconds_style(&self) -> FieldStyle {
            self.seconds_style
        }
        pub fn milliseconds_style(&self) -> FieldStyle {
            self.milliseconds_style
        }
        pub fn microseconds_style(&self) -> FieldStyle {
            self.microseconds_style
        }
        pub fn nanoseconds_style(&self) -> FieldStyle {
            self.nanoseconds_style
        }
        pub fn separator(&self) -> Separator {
            self.separator
        }

        pub fn years_display(&self) -> Display {
            self.years_display
        }
        pub fn months_display(&self) -> Display {
            self.months_display
        }
        pub fn weeks_display(&self) -> Display {
            self.weeks_display
        }
        pub fn days_display(&self) -> Display {
            self.days_display
        }
        pub fn hours_display(&self) -> Display {
            self.hours_display
        }
        pub fn minutes_display(&self) -> Display {
            self.minutes_display
        }
        pub fn seconds_display(&self) -> Display {
            self.seconds_display
        }
        pub fn milliseconds_display(&self) -> Display {
            self.milliseconds_display
        }
        pub fn microseconds_display(&self) -> Display {
            self.microseconds_display
        }
        pub fn nanoseconds_display(&self) -> Display {
            self.nanoseconds_display
        }

        pub fn fractional_digits(&self) -> i32 {
            self.fractional_digits
        }

        pub fn New(
            isolate: &Isolate,
            map: DirectHandle<Map>,
            locales: DirectHandle<Object>,
            input_options: DirectHandle<Object>,
        ) -> Result<DirectHandle<JSDurationFormat>, Error> {
            let factory = crate::factory::Factory::new();
            let method_name = "Intl.DurationFormat";

            let requested_locales = intl::canonicalize_locale_list(isolate, &locales)?;

            let options = GetOptionsObject(isolate, input_options, method_name)?;

            let matcher = intl::get_locale_matcher(isolate, &options, method_name)?;

            let mut numbering_system_str: Option<String> = None;
            let get =
                intl::get_numbering_system(isolate, &options, method_name, &mut numbering_system_str)?;

            let relevant_extension_keys: std::collections::HashSet<String> =
                ["nu".to_string()].into_iter().collect();
            let r = intl::resolve_locale(
                isolate,
                &JSDurationFormat::GetAvailableLocales(),
                requested_locales,
                matcher,
                relevant_extension_keys,
            )?;

            let mut icu_locale = r.icu_locale.clone();
            let mut status = ErrorCode::new();

            if let Some(numbering_system_str) = &numbering_system_str {
                if let Some(nu_extension_it) = r.extensions.get("nu") {
                    if nu_extension_it != numbering_system_str {
                        icu_locale.set_unicode_keyword_value("nu", None, &mut status);
                        if status.is_failure() {
                            println!("ICU error: {:?}", status);
                        }
                    }
                }
            }

            if let Some(numbering_system_str) = &numbering_system_str {
                if intl::is_valid_numbering_system(numbering_system_str) {
                    icu_locale.set_unicode_keyword_value("nu", Some(numbering_system_str), &mut status);
                    if status.is_failure() {
                        println!("ICU error: {:?}", status);
                    }
                }
            }

            let numbering_system = intl::get_numbering_system(icu_locale.clone());
            let separator = GetSeparator(&icu_locale);

            let style_strings: std::collections::HashSet<&str> =
                ["long", "short", "narrow", "digital"].into_iter().collect();
            let style_enums: std::collections::HashSet<Style> = [
                Style::kLong,
                Style::kShort,
                Style::kNarrow,
                Style::kDigital,
            ]
            .into_iter()
            .collect();
            let style: Style = GetStringOption(
                isolate,
                &options,
                "style",
                method_name,
                style_strings,
                style_enums,
                Style::kShort,
            )?;

            let managed_locale = Rc::new(icu_locale.clone());

            let long_short_narrow_strings: std::collections::HashSet<&str> =
                ["long", "short", "narrow"].into_iter().collect();
            let long_short_narrow_enums: std::collections::HashSet<FieldStyle> = [
                FieldStyle::kLong,
                FieldStyle::kShort,
                FieldStyle::kNarrow,
            ]
            .into_iter()
            .collect();
            macro_rules! call_get_duration_unit_options {
                ($unit:ident, $property:ident, $strings:ident, $enums:ident, $digital_base:ident, $prev_style:expr) => {
                    let $property = GetDurationUnitOptions(
                        isolate,
                        Unit::$unit,
                        stringify!($property),
                        concat!(stringify!($property), "Display"),
                        &options,
                        style,
                        $strings.clone(),
                        $enums.clone(),
                        FieldStyle::$digital_base,
                        $prev_style,
                    )?;
                };
            }

            call_get_duration_unit_options!(
                kYears,
                years_option,
                long_short_narrow_strings,
                long_short_narrow_enums,
                kShort,
                FieldStyle::kUndefined
            );
            let years_option_style = years_option.style;

            call_get_duration_unit_options!(
                kMonths,
                months_option,
                long_short_narrow_strings,
                long_short_narrow_enums,
                kShort,
                years_option_style
            );
            let months_option_style = months_option.style;
            call_get_duration_unit_options!(
                kWeeks,
                weeks_option,
                long_short_narrow_strings,
                long_short_narrow_enums,
                kShort,
                months_option_style
            );
            let weeks_option_style = weeks_option.style;

            call_get_duration_unit_options!(
                kDays,
                days_option,
                long_short_narrow_strings,
                long_short_narrow_enums,
                kShort,
                weeks_option_style
            );
            let days_option_style = days_option.style;

            let long_short_narrow_numeric2_digit_strings: std::collections::HashSet<&str> =
                ["long", "short", "narrow", "numeric", "2-digit"]
                    .into_iter()
                    .collect();
            let long_short_narrow_numeric2_digit_enums: std::collections::HashSet<FieldStyle> = [
                FieldStyle::kLong,
                FieldStyle::kShort,
                FieldStyle::kNarrow,
                FieldStyle::kNumeric,
                FieldStyle::k2Digit,
            ]
            .into_iter()
            .collect();
            call_get_duration_unit_options!(
                kHours,
                hours_option,
                long_short_narrow_numeric2_digit_strings,
                long_short_narrow_numeric2_digit_enums,
                kNumeric,
                days_option_style
            );
            let hours_option_style = hours_option.style;
            call_get_duration_unit_options!(
                kMinutes,
                minutes_option,
                long_short_narrow_numeric2_digit_strings,
                long_short_narrow_numeric2_digit_enums,
                kNumeric,
                hours_option_style
            );
            let minutes_option_style = minutes_option.style;
            call_get_duration_unit_options!(
                kSeconds,
                seconds_option,
                long_short_narrow_numeric2_digit_strings,
                long_short_narrow_numeric2_digit_enums,
                kNumeric,
                minutes_option_style
            );
            let seconds_option_style = seconds_option.style;

            let long_short_narrow_numeric_strings: std::collections::HashSet<&str> =
                ["long", "short", "narrow", "numeric"].into_iter().collect();
            let long_short_narrow_numeric_enums: std::collections::HashSet<FieldStyle> = [
                FieldStyle::kLong,
                FieldStyle::kShort,
                FieldStyle::kNarrow,
                FieldStyle::kNumeric,
            ]
            .into_iter()
            .collect();
            call_get_duration_unit_options!(
                kMilliseconds,
                milliseconds_option,
                long_short_narrow_numeric_strings,
                long_short_narrow_numeric_enums,
                kNumeric,
                seconds_option_style
            );
            let milliseconds_option_style = milliseconds_option.style;

            call_get_duration_unit_options!(
                kMicroseconds,
                microseconds_option,
                long_short_narrow_numeric_strings,
                long_short_narrow_numeric_enums,
                kNumeric,
                milliseconds_option_style
            );
            let microseconds_option_style = microseconds_option.style;

            call_get_duration_unit_options!(
                kNanoseconds,
                nanoseconds_option,
                long_short_narrow_numeric_strings,
                long_short_narrow_numeric_enums,
                kNumeric,
                microseconds_option_style
            );
            let fractional_digits = GetNumberOption(
                isolate,
                &options,
                factory.fractionalDigits_string.clone(),
                0,
                9,
                kUndefinedFractionalDigits,
            )?;

            let mut fmt = UnlocalizedNumberFormatter {}.rounding_mode(UNUM_ROUND_HALFUP).locale(icu_locale.clone());
            if !numbering_system.is_empty() && numbering_system != "latn" {
                fmt = fmt.adopt_symbols(icu::number::NumberingSystem::create_instance_by_name(
                    numbering_system.as_str(),
                    &mut status,
                ));
                if status.is_failure() {
                    println!("ICU error: {:?}", status);
                }
            }

            let managed_number_formatter = Rc::new(fmt);

            let mut duration_format = JSDurationFormat::new();
            duration_format.set_style_flags(0);
            duration_format.set_display_flags(0);
            
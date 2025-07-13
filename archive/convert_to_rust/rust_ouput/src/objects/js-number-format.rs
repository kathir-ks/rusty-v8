// Converted from V8 C++ source files:
// Header: js-number-format.h
// Implementation: js-number-format.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
use std::collections::HashSet;
use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

use crate::objects::intl_objects::*;
use crate::objects::js_locale::*;
use crate::objects::objects::*;
use crate::objects::string::*;

pub enum Style {
    DECIMAL,
    PERCENT,
    CURRENCY,
    UNIT,
}

pub enum CurrencyDisplay {
    CODE,
    SYMBOL,
    NAME,
    NARROW_SYMBOL,
}

pub enum CurrencySign {
    STANDARD,
    ACCOUNTING,
}

pub enum UnitDisplay {
    SHORT,
    NARROW,
    LONG,
}

pub enum Notation {
    STANDARD,
    SCIENTIFIC,
    ENGINEERING,
    COMPACT,
}

pub enum CompactDisplay {
    SHORT,
    LONG,
}

pub enum SignDisplay {
    AUTO,
    ALWAYS,
    NEVER,
    EXCEPT_ZERO,
    NEGATIVE,
}

pub enum UseGrouping {
    OFF,
    MIN2,
    AUTO,
    ALWAYS,
}

struct icu_number_LocalizedNumberFormatter {}

pub struct JSNumberFormat {}

impl JSNumberFormat {
    pub fn New(
        isolate: *mut V8,
        map: *mut V8,
        locales: *mut V8,
        options: *mut V8,
        service: *const i8,
    ) -> Result<*mut JSNumberFormat, String> {
        Err("unimplemented".to_string())
    }

    pub fn UnwrapNumberFormat(
        isolate: *mut V8,
        format_holder: *mut V8,
    ) -> Result<*mut JSNumberFormat, String> {
        Err("unimplemented".to_string())
    }

    pub fn NumberFormatFunction(
        isolate: *mut V8,
        number_format: *mut JSNumberFormat,
        numeric_obj: *mut V8,
    ) -> Result<*mut String, String> {
        Err("unimplemented".to_string())
    }

    pub fn ResolvedOptions(isolate: *mut V8, number_format: *mut JSNumberFormat) -> *mut JSObject {
        todo!()
    }

    pub fn FormatToParts(
        isolate: *mut V8,
        number_format: *mut JSNumberFormat,
        numeric_obj: *mut V8,
    ) -> Result<*mut V8, String> {
        Err("unimplemented".to_string())
    }

    pub fn FormatNumericRange(
        isolate: *mut V8,
        number_format: *mut JSNumberFormat,
        x: *mut V8,
        y: *mut V8,
    ) -> Result<*mut String, String> {
        Err("unimplemented".to_string())
    }

    pub fn FormatNumericRangeToParts(
        isolate: *mut V8,
        number_format: *mut JSNumberFormat,
        x: *mut V8,
        y: *mut V8,
    ) -> Result<*mut V8, String> {
        Err("unimplemented".to_string())
    }

    pub fn FormatNumeric(
        isolate: *mut V8,
        number_format: &icu::number::LocalizedNumberFormatter,
        numeric_obj: *mut V8,
    ) -> Result<*mut String, String> {
        Err("unimplemented".to_string())
    }

    pub fn GetAvailableLocales() -> &'static HashSet<String> {
        lazy_static::lazy_static! {
            static ref AVAILABLE_LOCALES: HashSet<String> = {
                let mut set = HashSet::new();
                set.insert("en-US".to_string());
                set.insert("de-DE".to_string());
                set
            };
        }
        &AVAILABLE_LOCALES
    }

    pub fn MinimumIntegerDigitsFromSkeleton(skeleton: &str) -> i32 {
        1
    }

    pub fn FractionDigitsFromSkeleton(skeleton: &str, minimum: *mut i32, maximum: *mut i32) -> bool {
        false
    }

    pub fn SignificantDigitsFromSkeleton(skeleton: &str, minimum: *mut i32, maximum: *mut i32) -> bool {
        false
    }

    pub fn RoundingModeString(isolate: *mut V8, skeleton: &str) -> *mut String {
        ptr::null_mut()
    }

    pub fn RoundingPriorityString(isolate: *mut V8, skeleton: &str) -> *mut String {
        ptr::null_mut()
    }

    pub fn TrailingZeroDisplayString(isolate: *mut V8, skeleton: &str) -> *mut String {
        ptr::null_mut()
    }

    pub fn RoundingIncrement(isolate: *mut V8, skeleton: &str) -> *mut V8 {
        ptr::null_mut()
    }

    pub fn SetDigitOptionsToFormatter(
        settings: icu::number::UnlocalizedNumberFormatter,
        digit_options: &Intl::NumberFormatDigitOptions,
    ) -> icu::number::UnlocalizedNumberFormatter {
        settings
    }

    pub fn NumberingSystemFromSkeleton(skeleton: &str) -> String {
        "latn".to_string()
    }

    pub fn GetRangeFormatter(
        isolate: *mut V8,
        locale: *mut String,
        number_formatter: &icu::number::LocalizedNumberFormatter,
    ) -> Result<icu::number::LocalizedNumberRangeFormatter, String> {
        Err("unimplemented".to_string())
    }

    pub fn icu_number_formatter(&self) -> &Tagged<Managed<icu::number::LocalizedNumberFormatter>> {
        todo!()
    }

    pub fn locale(&self) -> Tagged<String> {
        todo!()
    }

    pub fn set_icu_number_formatter(&mut self, _value: Tagged<Managed<icu::number::LocalizedNumberFormatter>>) {
         
    }

    pub fn set_locale(&mut self, _value: String) {
        
    }
}

pub struct IntlMathematicalValue {
    approx_: f64,
    value_: *mut Object,
}

impl IntlMathematicalValue {
    pub fn new() -> Self {
        IntlMathematicalValue {
            approx_: 0.0,
            value_: ptr::null_mut(),
        }
    }

    pub fn IsNaN(&self) -> bool {
        self.approx_.is_nan()
    }

    pub fn From(isolate: *mut V8, value: *mut V8) -> Result<IntlMathematicalValue, String> {
        Err("unimplemented".to_string())
    }

    pub fn FormatNumeric(
        isolate: *mut V8,
        number_format: &icu::number::LocalizedNumberFormatter,
        x: &IntlMathematicalValue,
    ) -> Result<icu::number::FormattedNumber, String> {
        Err("unimplemented".to_string())
    }

    pub fn FormatRange(
        isolate: *mut V8,
        number_range_format: &icu::number::LocalizedNumberRangeFormatter,
        x: &IntlMathematicalValue,
        y: &IntlMathematicalValue,
    ) -> Result<icu::number::FormattedNumberRange, String> {
        Err("unimplemented".to_string())
    }

    fn ToString(&self, isolate: *mut V8) -> Result<*mut String, String> {
        Err("unimplemented".to_string())
    }

    fn ToFormattable(&self, isolate: *mut V8) -> Result<icu::Formattable, String> {
        Err("unimplemented".to_string())
    }
}

struct NumberFormatSpan {
    field_id: i32,
    begin_pos: i32,
    end_pos: i32,
}

impl NumberFormatSpan {
    fn new(field_id: i32, begin_pos: i32, end_pos: i32) -> Self {
        NumberFormatSpan {
            field_id,
            begin_pos,
            end_pos,
        }
    }
}

mod icu {
    pub mod number {
        pub struct LocalizedNumberFormatter {}
        impl LocalizedNumberFormatter{
            pub fn toSkeleton(&self, _status : i32) -> String {
                todo!()
            }
        }

        pub struct LocalizedNumberRangeFormatter{}

        pub struct UnlocalizedNumberFormatter{
           
        }
        impl UnlocalizedNumberFormatter {
            pub fn roundingMode(self, _mode :i32) -> Self {
                todo!()
            }

            pub fn adoptSymbols(self, _sym : *mut V8) -> Self {
                todo!()
            }
        }
        

        pub struct FormattedNumberRange{}
        pub struct FormattedNumber{}
        
        pub struct NumberFormatter{}

        impl NumberFormatter{
            pub fn forSkeleton(_skeleton : String, _perror : i32, _status : i32) -> NumberFormatter {
                todo!()
            }
        }

        pub struct Scale {}
        impl Scale {
            pub fn powerOfTen(_n : i32) -> Scale {
                todo!()
            }
        }

         pub struct Precision {}

         impl Precision {
            pub fn unlimited() -> Self{
                todo!()
            }
            pub fn minMaxSignificantDigits(_a : i32,_b : i32)-> Self{
                 todo!()
             }

             pub fn incrementExact(_a : i32,_b : i32)-> Self{
                todo!()
             }

             pub fn minMaxFraction(_a : i32,_b : i32)-> Self{
                todo!()
             }
         }

        pub struct IntegerWidth{}
        impl IntegerWidth {
            pub fn zeroFillTo(_a : i32) -> IntegerWidth {
                todo!()
            }
        }
    }
     
    pub struct UnicodeString{}
     
    pub struct Formattable{

    }

    impl Formattable {
        pub fn new(_a : f64){
            todo!()
        }
    }

    pub struct MeasureUnit{}

    pub struct CurrencyUnit{

    }
    impl CurrencyUnit{
        pub fn new(_a : f64){
            todo!()
        }
    }
    

    pub mod Locale {
        pub fn forLanguageTag(_lang : &str, _status : i32) -> super::UnicodeString{
            todo!()
        }
    }
}

mod Intl {
    pub struct NumberFormatDigitOptions{

    }
    impl NumberFormatDigitOptions{
        pub fn new(_a : f64){
            todo!()
        }
    }
     pub mod RoundingMode{

     }

    pub fn IsValidNumberingSystem(_a : &str) -> bool {
        todo!()
    }

    pub fn GetNumberingSystem(_a : &str) -> String {
        todo!()
    }

    pub fn SetNumberFormatDigitOptions(_a : i32, _b : i32, _c : i32, _d:i32,_e:i32,_f: &str) -> Result<NumberFormatDigitOptions, String> {
        Err("unimplemented".to_string())
    }

}


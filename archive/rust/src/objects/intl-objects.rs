// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a placeholder; actual crate imports would depend on how the ICU bindings are structured.
// For example:
// extern crate icu;
// use icu::*;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::mem;
use std::num::TryFromIntError;
use std::ptr;
use std::str;

// Placeholder for base crate; adjust as needed.
mod base {
    pub type Vector<T> = Vec<T>;
    impl<T> Vector<T> {
        pub fn begin(&self) -> &[T] {
            &self[..]
        }

        pub fn end(&self) -> &[T] {
            &self[self.len()..]
        }

        pub fn sub_vector(&self, start: usize, end: usize) -> Vec<T>
            where
                T: Clone,
        {
            self[start..end].to_vec()
        }
    }

    pub type uc16 = u16;

    // Placeholder for ICU StringPiece
    #[derive(Debug, Clone)]
    pub struct StringPiece<'a> {
        data: &'a [u8],
    }

    impl<'a> StringPiece<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            StringPiece { data }
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.data
        }
    }
}

// Placeholder for date crate; adjust as needed.
mod date {
    pub struct Date {} // Placeholder
}

// Placeholder for execution crate; adjust as needed.
mod execution {
    pub struct Isolate {}

    impl Isolate {
        pub fn default_locale(&self) -> String {
            "en-US".to_string() // Placeholder
        }
        pub fn set_icu_object_in_cache(&self, _cache_type: ICUObjectCacheType, _locales: &Object, _object: std::sync::Arc<dyn icu::Memory>) {}
        pub fn get_cached_icu_object(&self, _cache_type: ICUObjectCacheType, _locales: &Object) -> *mut icu::Collator {
            ptr::null_mut() //Placeholder
        }
        pub fn stack_overflow(&self) {}
        pub fn context(&self) -> &Context {
            unimplemented!()
        }
    }

    pub enum ICUObjectCacheType {
        kDefaultCollator,
        kDefaultNumberFormat,
    }

    pub struct LocalIsolate {}
}

// Placeholder for handles crate; adjust as needed.
mod handles {
    pub struct GlobalHandles {}
}

// Placeholder for heap crate; adjust as needed.
mod heap {
    pub struct Factory {}

    impl Factory {
        pub fn empty_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn new_raw_two_byte_string(&self, _length: u32) -> Result<SeqTwoByteString, ()> {
            Ok(SeqTwoByteString::default()) // Placeholder
        }
        pub fn new_raw_one_byte_string(&self, _length: i32) -> Result<SeqOneByteString, ()> {
            Ok(SeqOneByteString::default()) // Placeholder
        }
         pub fn new_string_from_ascii_checked(&self, _string: &str) -> String {
            String::default() // Placeholder
        }
        pub fn new_js_object(&self, _function: &Object) -> JSObject {
            JSObject::default()
        }
        pub fn type_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn value_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn calendar_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn collation_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn currency_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn numbering_system_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn time_zone_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn unit_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn minimum_integer_digits_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn minimum_fraction_digits_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn maximum_fraction_digits_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn minimum_significant_digits_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn maximum_significant_digits_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn rounding_increment_string(&self) -> String {
            String::default() // Placeholder
        }
        pub fn new_string_from_utf8(&self, _string: base::StringPiece) -> Result<String, ()> {
            Ok(String::default()) // Placeholder
        }
        pub fn new_js_array(&self, _i: i32) -> JSArray {
            JSArray::default() // Placeholder
        }
        pub fn new_fixed_array(&self, _size: i32) -> FixedArray {
            FixedArray::default() // Placeholder
        }
        pub fn new_js_array_with_elements(&self, _fixed_array: FixedArray) -> JSArray {
            JSArray::default() // Placeholder
        }
    }
}

// Placeholder for objects crate; adjust as needed.
mod objects {
    use super::*;

    pub trait ObjectTrait {} //Placeholder

    #[derive(Default, Clone)]
    pub struct Object {}

    impl Object {
        pub fn boolean_value(&self, _obj: &Object, _isolate: &Isolate) -> bool {
            false
        }
         pub fn number_value(&self, _obj: &Object) -> f64 {
            0.0
        }
        pub fn get_length_from_array_like(_isolate: &Isolate, _o: &JSReceiver) -> Result<Object,()> {
            Ok(Object::default())
        }
        pub fn ordinary_has_instance(_isolate: &Isolate, _constructor: &JSFunction, _receiver: &JSReceiver) -> Result<Object, ()> {
            Ok(Object::default())
        }
        pub fn to_object(_isolate: &Isolate, _locales: &Object) -> Result<JSReceiver,()> {
            Ok(JSReceiver::default()) // Placeholder
        }
        pub fn get_property(_iterator: &LookupIterator) -> Result<Object, ()> {
            Ok(Object::default())
        }
    }
    
    impl ObjectTrait for Object {}

    #[derive(Default, Clone)]
    pub struct JSObject {
        // Placeholders; actual fields would be defined here.
    }

    impl JSObject {
        pub fn add_property(_isolate: &Isolate, _element: &JSObject, _type_string: String, _field_type_string: String, _none: PropertyAttributes) {}
        pub fn add_data_element(_array: &JSArray, _index: i32, _element: &JSObject, _none: PropertyAttributes) -> Result<(),()> {
            Ok(()) // Placeholder
        }
    }
    
    impl ObjectTrait for JSObject {}

    #[derive(Default, Clone)]
    pub struct JSArray {
        // Placeholders; actual fields would be defined here.
    }

    impl JSArray {}
    
    impl ObjectTrait for JSArray {}

    #[derive(Default, Clone)]
    pub struct JSFunction {
        // Placeholders; actual fields would be defined here.
    }

    impl JSFunction {
        pub fn get_derived_map(_isolate: &Isolate, _constructor: &JSFunction, _constructor2: &JSFunction) -> Result<Map, ()> {
            Ok(Map::default()) // Placeholder
        }
    }
    
    impl ObjectTrait for JSFunction {}

    #[derive(Default, Clone)]
    pub struct JSCollator {
        // Placeholders; actual fields would be defined here.
    }

    impl JSCollator {
        pub fn icu_collator(&self) -> &Managed<icu::Collator>{
            unimplemented!()
        }
    }
    
    impl ObjectTrait for JSCollator {}

    #[derive(Default, Clone)]
    pub struct JSNumberFormat {
        // Placeholders; actual fields would be defined here.
    }

     impl JSNumberFormat {
        pub fn icu_number_formatter(&self) -> &Managed<icu::number::LocalizedNumberFormatter> {
            unimplemented!()
        }
    }
    
    impl ObjectTrait for JSNumberFormat {}

    #[derive(Default, Clone)]
    pub struct JSLocale {}
    impl JSLocale {
        pub fn to_string(locale: &JSLocale) -> String {
            String::default() // Placeholder
        }
         pub fn starts_with_unicode_language_id(tag: String) -> bool {
            false // Placeholder
        }
    }
    impl ObjectTrait for JSLocale {}

    #[derive(Default, Clone)]
    pub struct Map {}
    
    impl ObjectTrait for Map {}

    #[derive(Default, Clone)]
    pub struct PropertyDescriptor {}

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum RoundingMode {
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

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum RoundingPriority {
        kAuto,
        kMorePrecision,
        kLessPrecision,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum TrailingZeroDisplay {
        kAuto,
        kStripIfInteger,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum RoundingType {
        kMorePrecision,
        kLessPrecision,
        kSignificantDigits,
        kFractionDigits,
    }

    #[derive(Clone)]
    pub struct String {
        // Placeholder; actual string data would be defined here.
        data: StringData,
    }

    #[derive(Clone)]
    enum StringData {
        OneByte(Vec<u8>),
        TwoByte(Vec<u16>),
    }

    impl String {
        pub fn to_c_string(&self) -> std::ffi::CString {
            match &self.data {
                StringData::OneByte(bytes) => {
                    let vec: Vec<u8> = bytes.iter().map(|&b| b).collect();
                    std::ffi::CString::new(vec).unwrap()
                }
                StringData::TwoByte(chars) => {
                    // Convert UTF-16 to UTF-8
                    let utf8_string: String = String::from_utf16_lossy(&chars);
                    std::ffi::CString::new(utf8_string.into_bytes()).unwrap()
                }
            }
        }
        pub fn length(&self) -> i32 {
            match &self.data {
                StringData::OneByte(bytes) => bytes.len() as i32,
                StringData::TwoByte(chars) => chars.len() as i32,
            }
        }

        pub fn is_one_byte_representation(&self) -> bool {
            match self.data {
                StringData::OneByte(_) => true,
                _ => false,
            }
        }

        pub fn flatten(_isolate: &Isolate, string: String) -> String {
            string
        }

        pub fn get(&self, index: i32) -> u16 {
            match &self.data {
                StringData::OneByte(bytes) => bytes[index as usize] as u16,
                StringData::TwoByte(chars) => chars[index as usize],
            }
        }

        pub fn is_flat(&self) -> bool {
            true // Placeholder; assuming all strings are flat
        }

        pub fn get_flat_content(&self, _no_gc: DisallowGarbageCollection) -> FlatContent {
            match &self.data {
                StringData::OneByte(bytes) => FlatContent::OneByte(bytes.clone()),
                StringData::TwoByte(chars) => FlatContent::TwoByte(chars.clone()),
            }
        }

        pub fn is_equal_to(&self, _other: base::StringPiece, _isolate: &Isolate) -> bool {
            // Placeholder
            false
        }

        pub fn from_utf16_lossy(v: &[u16]) -> String {
            String {
                data: StringData::TwoByte(v.to_vec()),
            }
        }
    }

    impl Default for String {
        fn default() -> Self {
            String {
                data: StringData::OneByte(Vec::new()),
            }
        }
    }

    #[derive(Clone)]
    pub enum FlatContent {
        OneByte(Vec<u8>),
        TwoByte(Vec<u16>),
    }

    impl FlatContent {
        pub fn is_flat(&self) -> bool {
            true // Placeholder; assuming all are flat
        }

        pub fn is_one_byte(&self) -> bool {
            match self {
                FlatContent::OneByte(_) => true,
                _ => false,
            }
        }

        pub fn is_two_byte(&self) -> bool {
            match self {
                FlatContent::TwoByte(_) => true,
                _ => false,
            }
        }

        pub fn to_one_byte_vector(&self) -> base::Vector<u8> {
            match self {
                FlatContent::OneByte(bytes) => bytes.clone(),
                _ => panic!("Not a one-byte string"),
            }
        }

        pub fn to_uc16_vector(&self) -> base::Vector<u16> {
            match self {
                FlatContent::TwoByte(chars) => chars.clone(),
                _ => panic!("Not a two-byte string"),
            }
        }

        pub fn length(&self) -> i32 {
            match self {
                FlatContent::OneByte(bytes) => bytes.len() as i32,
                FlatContent::TwoByte(chars) => chars.len() as i32,
            }
        }

        pub fn get(&self, index: i32) -> u16 {
            match self {
                FlatContent::OneByte(bytes) => bytes[index as usize] as u16,
                FlatContent::TwoByte(chars) => chars[index as usize],
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct SeqOneByteString {
        chars: Vec<u8>,
    }
    impl SeqOneByteString {
        pub fn seq_one_byte_string_set(&mut self, index: i32, value: u8) {
            self.chars[index as usize] = value;
        }
         pub fn get_chars(&self, _no_gc: DisallowGarbageCollection) -> *mut u8 {
            self.chars.as_ptr() as *mut u8 // Placeholder
        }
    }
    
    impl ObjectTrait for SeqOneByteString {}

    #[derive(Default, Clone)]
    pub struct SeqTwoByteString {}
    impl SeqTwoByteString {
         pub fn get_chars(&self, _no_gc: DisallowGarbageCollection) -> *mut u16 {
             ptr::null_mut()// Placeholder
         }
    }
    
    impl ObjectTrait for SeqTwoByteString {}

    #[derive(Default, Clone)]
    pub struct FixedArray {}
    impl FixedArray {
        pub fn set(&self, _index: i32, _str: String) {}
    }
    
    impl ObjectTrait for FixedArray {}

    #[derive(Clone, Copy)]
    pub enum PropertyAttributes {
        NONE,
    }
    impl Default for PropertyAttributes {
        fn default() -> Self {
            PropertyAttributes::NONE
        }
    }
    
    impl ObjectTrait for PropertyAttributes {}

    #[derive(Default, Clone)]
    pub struct SharedStringAccessGuardIfNeeded {}

    impl SharedStringAccessGuardIfNeeded {
        pub fn is_needed(_string: &String) -> bool {
            false
        }
    }

    #[derive(Default, Clone)]
    pub struct Context {
       pub native_context: NativeContext,
    }

    #[derive(Default, Clone)]
    pub struct NativeContext {
       pub intl_collator_function: String,
       pub intl_number_format_function: String,
    }

    #[derive(Default, Clone)]
    pub struct LookupIterator {}
}

// Placeholder for option-utils crate; adjust as needed.
mod option_utils {
    use super::*;

    pub fn get_number_option(_isolate: &Isolate, _options: &JSReceiver, _key: String, _min: i32, _max: i32, _fallback: i32) -> Result<i32, ()> {
        Ok(_fallback) // Placeholder
    }

    pub fn get_string_option<T>(_isolate: &Isolate, _options: &JSReceiver, _key: &str, _service: &str, _values: Vec<&str>, _enum_values: Vec<T>, _default: T) -> Result<T, ()>
        where
            T: Copy,
    {
        Ok(_default) // Placeholder
    }

    pub fn default_number_option(_isolate: &Isolate, _obj: &Object, _min: i32, _max: i32, _fallback: i32, _key: String) -> Result<i32, ()> {
        Ok(_fallback) // Placeholder
    }
}

// Placeholder for strings crate; adjust as needed.
mod strings {
    pub struct StringCase {}
}

// Placeholder for uvernum crate; adjust as needed.
mod uvernum {
    pub const U_ICU_VERSION_MAJOR_NUM: i32 = 70; // Placeholder
}

// Placeholder for base crate; adjust as needed.
mod common {
    pub const kMaxUInt8: u16 = 255; // Placeholder
    pub const kMaxUInt32: u32 = 4294967295; // Placeholder
    pub mod globals {
        pub const V8_MINIMUM_ICU_VERSION: i32 = 70; // Placeholder
    }
}

// Placeholder for isolate crate; adjust as needed.
mod isolate {
    pub struct Isolate {}
}

use common::*;
use objects::*;

// Define macro for static assertion
macro_rules! static_assert {
    ($cond:expr, $msg:expr) => {
        const _: [(); 1] = if !($cond) {
            panic!($msg);
        } else {
            [()]
        };
    };
}

// Apply static assertion for ICU version
static_assert!(
    common::globals::V8_MINIMUM_ICU_VERSION <= uvernum::U_ICU_VERSION_MAJOR_NUM,
    "v8 is required to build with ICU V8_MINIMUM_ICU_VERSION and up"
);

// Define module for Intl objects
pub mod intl_objects {
    use super::*;
    use std::any::Any;
    use std::ffi::CString;
    use std::sync::{Arc, Mutex};

    const NONE: PropertyAttributes = PropertyAttributes::NONE;
    // Macro to convert to one byte
    macro_rules! as_one_byte {
        ($ch:expr) => {
            {
                debug_assert!($ch <= common::kMaxUInt8);
                $ch as u8
            }
        };
    }

    const K_TO_LOWER: [u8; 256] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
        0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23,
        0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
        0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73,
        0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
        0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B,
        0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x80, 0x81, 0x82, 0x83,
        0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
        0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B,
        0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7,
        0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF, 0xB0, 0xB1, 0xB2, 0xB3,
        0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF,
        0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB,
        0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xD7,
        0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3,
        0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF,
        0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB,
        0xFC, 0xFD, 0xFE, 0xFF,
    ];

    /// Converts a Latin-1 character to lowercase.
    #[inline]
    const fn to_latin1_lower(ch: u8) -> u8 {
        K_TO_LOWER[ch as usize]
    }

    /// Converts a Latin-1 character to uppercase.
    #[inline]
    const fn to_latin1_upper(ch: u8) -> u8 {
        debug_assert!(ch != 0xDF && ch != 0xB5 && ch != 0xFF);
        ch & !(if is_ascii_lower(ch) || (((ch & 0xE0) == 0xE0) && ch != 0xF7) { 0x20 } else { 0 })
    }

    /// Finds the index of the first uppercase or non-ASCII character in a string.
    fn find_first_upper_or_non_ascii(s: &String, length: i32) -> i32 {
        for index in 0..length {
            let ch = s.get(index);
            if is_ascii_upper(ch as u8) || (ch & !0x7F != 0) {
                return index;
            }
        }
        length
    }

    /// Converts a string to lowercase using a fast ASCII conversion if possible.
    fn fast_ascii_convert<const TO_LOWER: bool>(dst: &mut [char], src: &[u8], length: i32, has_changed_character: &mut bool) -> i32 {
        let mut index_to_first_unprocessed = length as usize;
        for index in 0..length as usize {
            let src_char = src[index];
            if src_char > 127 {
                index_to_first_unprocessed = index;
                break;
            }

            let converted_char = if TO_LOWER {
                if src_char >= b'A' && src_char <= b'Z' {
                    *has_changed_character = true;
                    (src_char + 32) as char
                } else {
                    src_char as char
                }
            } else {
                if src_char >= b'a' && src_char <= b'z' {
                    *has_changed_character = true;
                    (src_char - 32) as char
                } else {
                    src_char as char
                }
            };
            dst[index] = converted_char;
        }
        index_to_first_unprocessed as i32
    }

    /// Determines if a character is an ASCII uppercase letter.
    #[inline]
    const fn is_ascii_upper(ch: u8) -> bool {
        ch >= b'A' && ch <= b'Z'
    }

    /// Determines if a character is an ASCII lowercase letter.
    #[inline]
    const fn is_ascii_lower(ch: u8) -> bool {
        ch >= b'a' && ch <= b'z'
    }

    /// Converts an ASCII character to uppercase.
    #[inline]
    const fn to_ascii_upper(ch: u16) -> u8 {
        if ch >= b'a' as u16 && ch <= b'z' as u16 {
            (ch - 32) as u8
        } else {
            ch as u8
        }
    }

    /// Structure to disallow garbage collection.  Rust does not require explicit disabling of GC.
    #[derive(Default, Clone)]
    pub struct DisallowGarbageCollection {}

    impl DisallowGarbageCollection {}

    /// A struct representing Intl-related functionality.
    pub struct Intl {}

    impl Intl {
        /// Retrieves the Latin-1 lowercase conversion table.
        pub fn to_latin1_lower_table() -> &'static [u8] {
            &K_TO_LOWER
        }

        /// Converts a V8 string to an ICU UnicodeString.
        pub fn to_icu_unicode_string(isolate: &Isolate, string: String, offset: i32) -> icu::UnicodeString {
            debug_assert!(string.is_flat());
            let _no_gc = DisallowGarbageCollection {};
           // Placeholder implementation
            let flat = string.get_flat_content(_no_gc);
            match flat {
                FlatContent::OneByte(one_byte_string) => {
                    let sub_string = &one_byte_string[offset as usize..];
                    let utf8_string = String::from_utf8(sub_string.to_vec()).unwrap();
                    icu::UnicodeString::from_string(&utf8_string)
                }
                FlatContent::TwoByte(two_byte_string) => {
                    let sub_string = &two_byte_string[offset as usize..];
                    let utf16_string = String::from_utf16(sub_string).unwrap();
                    icu::UnicodeString::from_string(&utf16_string)
                }
            }
        }

        /// Converts a string to lowercase.
        pub fn convert_to_lower(isolate: &Isolate, s: String) -> Result<String, ()> {
            if !s.is_one_byte_representation() {
                // Use a slower implementation for strings with characters beyond U+00FF.
                return Self::locale_convert_case(isolate, s, false, "");
            }

            let length = s.length();

            // We depend here on the invariant that the length of a Latin1
            // string is invariant under ToLowerCase, and the result always
            // fits in the Latin1 range in the *root locale*. It does not hold
            // for ToUpperCase even in the root locale.

            // Scan the string for uppercase and non-ASCII characters for strings
            // shorter than a machine-word without any memory allocation overhead.
            // TODO(jshin): Apply this to a longer input by breaking FastAsciiConvert()
            // to two parts, one for scanning the prefix with no change and the other for
            // handling ASCII-only characters.

            let is_short = length < mem::size_of::<usize>() as i32;
            if is_short {
                let is_lower_ascii = find_first_upper_or_non_ascii(&s, length) == length;
                if is_lower_ascii {
                    return Ok(s);
                }
            }

            let mut result = isolate.factory().new_raw_one_byte_string(length).unwrap();

            Ok(Self::convert_one_byte_to_lower(s, result))
        }

        fn convert_one_byte_to_lower(src: String, dst: SeqOneByteString) -> String {
            let length = src.length();
            debug_assert_eq!(src.length(), dst.chars.len() as i32);
            debug_assert!(src.is_one_byte_representation());
            debug_assert!(src.is_flat());

            let _no_gc = DisallowGarbageCollection {};

            let mut src_flat = src.get_flat_content(_no_gc);
            let mut dst_data = dst.chars;
            // let dst_data = Cast::<SeqOneByteString>(&dst).get_chars(_no_gc);

            match src_flat {
                FlatContent::OneByte(src_data) => {
                    let mut has_changed_character = false;
                    let index_to_first_unprocessed = fast_ascii_convert::<true>(
                        &mut vec![Default::default(); length as usize],
                        &src_data,
                        length,
                        &mut has_changed_character,
                    );

                    if index_to_first_unprocessed == length {
                        if has_changed_character {
                            let dst_one_byte = dst_data;
                            
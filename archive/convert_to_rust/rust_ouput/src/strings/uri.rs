// Converted from V8 C++ source files:
// Header: uri.h
// Implementation: uri.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

//use v8::base::uc16;
//use v8::internal::base::Vector;

pub struct V8 {}

pub enum v8 {}

pub enum class SnapshotSpace : u8 {}

pub struct AllStatic {}

struct Local<'a, T> {}

pub enum void {}

pub struct DisallowGarbageCollection {}

fn Use(&self, node: OpIndex) -> InstructionOperand {}

fn this(&self) -> &T {
    panic!()
}

pub fn source(&self) -> String {
    panic!()
}

pub fn code(&self) -> i32 {
    panic!()
}

pub fn is(c: uchar) -> bool {
    panic!()
}

pub fn bool() -> Self {
    panic!()
}

pub fn switch(&self, reg: Register, case_value_base: i32, labels: &[*mut Label], num_labels: i32) {}

pub fn replacement(&self) -> Option<Rc<RefCell<Node>>> {
    panic!()
}

pub fn character(&self) -> i32 {
    panic!()
}

pub fn at(&self, index: usize) -> &T {
    panic!()
}

pub fn value(&self) -> &Object {
    panic!()
}

fn high(&self) -> DwVfpRegister {
    panic!()
}

fn low(&self) -> DwVfpRegister {
    panic!()
}

pub fn EraseIf<C, F, T>(container: &mut C, f: F) {}

pub fn first(&self) -> Key {
    panic!()
}

pub fn second(&self) -> Value {
    panic!()
}

pub fn uri(&self) -> String {
    panic!()
}

fn contents(&self) -> String {
    panic!()
}

fn not(self) -> Self::Output {
    panic!()
}

pub fn change(&self) -> i32 {
    panic!()
}

pub fn error(&self) -> bool {
    panic!()
}

fn GC(&mut self, gctype: GCType) {}

fn string(&self) -> StringView {
    panic!()
}

fn decode<D: DecoderTraits, Char>(slf: &Utf8DecoderBase<D>, out: &mut [Char], data: Vector<u8>) {}

pub fn length(&self) -> usize {
    panic!()
}

fn index(&self, index: i32) -> &Self::Output {
    panic!()
}

// Dummy definitions for types used
type Isolate = V8;
type Handle<T> = Rc<T>;
type DirectHandle<T> = Rc<T>;
type MaybeHandle<T> = Option<Rc<T>>;
type MaybeDirectHandle<T> = Option<Rc<T>>;
type String = Vec<u16>;
type SeqTwoByteString = Vec<u16>;
type SeqOneByteString = Vec<u8>;
type Object = u32;
type Vector<T> = Vec<T>;
type base::uc16 = u16;
type base::uc32 = u32;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type GCType = u32;
type OpIndex = u32;
type InstructionOperand = u32;
type Register = u32;
type Label = u32;
type Key = u32;
type Value = u32;
type DwVfpRegister = u32;
type StringView = u32;
type Utf8DecoderBase<D> = u32;
trait DecoderTraits {}
type uchar = u8;

mod base {
    pub type uc16 = u16;

    pub fn HexValue(c: uc16) -> i32 {
        match c {
            '0'..='9' => (c as i32) - ('0' as i32),
            'a'..='f' => (c as i32) - ('a' as i32) + 10,
            'A'..='F' => (c as i32) - ('A' as i32) + 10,
            _ => -1,
        }
    }

    pub fn HexCharOfValue(value: u8) -> u8 {
        if value < 10 {
            b'0' + value
        } else {
            b'A' + (value - 10)
        }
    }
    pub struct VectorOf {}
}

mod unibrow {
    pub mod Utf8 {
        pub const kMaxOneByteChar: u16 = 0x7f;
        pub const kBadChar: u32 = 0xFFFD;
        pub const kMaxEncodedSize: usize = 4;

        pub fn ValueOf(octets: &[u8], length: i32, cursor: &mut usize) -> u32 {
            if length == 0 {
                return kBadChar;
            }

            let byte = octets[0];
            if byte <= 0x7F {
                *cursor = 1;
                return byte as u32;
            } else if byte >= 0xC2 && byte <= 0xDF {
                if length < 2 {
                    return kBadChar;
                }
                let byte2 = octets[1];
                if byte2 < 0x80 || byte2 > 0xBF {
                    return kBadChar;
                }
                *cursor = 2;
                return ((byte as u32 & 0x1F) << 6) | (byte2 as u32 & 0x3F);
            } else if byte >= 0xE0 && byte <= 0xEF {
                if length < 3 {
                    return kBadChar;
                }
                let byte2 = octets[1];
                let byte3 = octets[2];
                if byte2 < 0x80 || byte2 > 0xBF || byte3 < 0x80 || byte3 > 0xBF {
                    return kBadChar;
                }
                *cursor = 3;
                return ((byte as u32 & 0x0F) << 12) | ((byte2 as u32 & 0x3F) << 6) | (byte3 as u32 & 0x3F);
            } else if byte >= 0xF0 && byte <= 0xF4 {
                if length < 4 {
                    return kBadChar;
                }
                let byte2 = octets[1];
                let byte3 = octets[2];
                let byte4 = octets[3];
                if byte2 < 0x80 || byte2 > 0xBF || byte3 < 0x80 || byte3 > 0xBF || byte4 < 0x80 || byte4 > 0xBF {
                    return kBadChar;
                }
                *cursor = 4;
                let codepoint = ((byte as u32 & 0x07) << 18) | ((byte2 as u32 & 0x3F) << 12) | ((byte3 as u32 & 0x3F) << 6) | (byte4 as u32 & 0x3F);
                if codepoint > 0x10FFFF {
                    return kBadChar;
                }
                return codepoint;
            } else {
                return kBadChar;
            }
        }
        
        pub fn Encode(s: &mut [char], c: u16, previous: i32, ignore_previous: bool) -> i32 {
            if c <= 0x7F {
                s[0] = c as u8 as char;
                1
            } else if c <= 0x7FF {
                s[0] = (0xC0 | (c >> 6)) as u8 as char;
                s[1] = (0x80 | (c & 0x3F)) as u8 as char;
                2
            } else {
                s[0] = (0xE0 | (c >> 12)) as u8 as char;
                s[1] = (0x80 | ((c >> 6) & 0x3F)) as u8 as char;
                s[2] = (0x80 | (c & 0x3F)) as u8 as char;
                3
            }
        }
    }
    pub mod Utf16 {
        pub const kMaxNonSurrogateCharCode: u32 = 0xD7FF;

        pub fn LeadSurrogate(value: u32) -> u16 {
            ((value - 0x10000) >> 10 + 0xD800) as u16
        }

        pub fn TrailSurrogate(value: u32) -> u16 {
            ((value - 0x10000) & 0x3FF + 0xDC00) as u16
        }
        
        pub fn CombineSurrogatePair(lead: u16, trail: u16) -> u32 {
          0x10000 + (((lead as u32) - 0xD800) << 10) + ((trail as u32) - 0xDC00)
        }
        
        pub const kNoPreviousCharacter: i32 = 0;
    }
}

mod strings {
    pub mod char_predicates_inl {
        pub fn IsAlphaNumeric(c: u16) -> bool {
            (c >= 'a' as u16 && c <= 'z' as u16)
                || (c >= 'A' as u16 && c <= 'Z' as u16)
                || (c >= '0' as u16 && c <= '9' as u16)
        }
    }

    pub mod string_search {
        use super::super::{base,Vector,String};
        
        pub struct StringSearch<T, Char> {
            isolate: u32, //Isolate
            pattern: Vector<T>
        }
        
        impl <T, Char> StringSearch<T, Char> {
            pub fn new(isolate: u32, pattern: Vector<T>) -> StringSearch<T, Char> {
                StringSearch {
                    isolate,
                    pattern
                }
            }
            
            pub fn Search(&self, vector: base::Vector<const Char>, start_index: i32) -> i32 {
                -1 // Default value if not found
            }
        }
    }
    
    pub mod unicode_inl {
        
    }
}

impl String {
    pub const kMaxLength: u32 = 1024 * 1024;
    pub const kMaxOneByteCharCode: u16 = 255;

    fn Flatten(isolate: &mut Isolate, uri: Rc<String>) -> Rc<String> {
        uri
    }
    
    fn Get(self: &String, index: usize) -> u16 {
        self[index]
    }

    fn IsOneByteRepresentationUnderneath(_string: &String) -> bool {
        true
    }
    
    pub struct FlatContent {}
    
    impl String {
        fn GetFlatContent<'a>(_string: &'a String, _no_gc: DisallowGarbageCollection) -> FlatContent {
            FlatContent {}
        }
    }
}

impl FlatContent {
    fn Get(&self, index: usize) -> base::uc16 {
        0
    }
}

impl SeqTwoByteString {
    fn SeqTwoByteStringSet(&mut self, index: usize, value: u16) {
        self[index] = value;
    }
}

impl SeqOneByteString {
    fn SeqOneByteStringSet(&mut self, index: usize, value: u8) {
        self[index] = value;
    }
}

mod factory {
    use super::{String, SeqTwoByteString, SeqOneByteString,base};
    use std::rc::Rc;
    
    pub fn NewStringFromOneByte(bytes: base::Vector<const u8>) -> Result<Rc<String>, StringError> {
        let s = String::from_iter(bytes.iter().map(|&x| x as u16));
        Ok(Rc::new(s))
    }

    pub fn NewProperSubString(string: Rc<String>, start: i32, end: i32) -> Rc<String> {
        let sub = string[(start as usize)..(end as usize)].to_vec();
        Rc::new(sub)
    }
    
    pub fn NewConsString(first: Rc<String>, second: Rc<String>) -> Rc<String> {
        let mut combined = first.clone();
        combined.extend_from_slice(&second);
        Rc::new(combined)
    }
    
    pub fn NewRawTwoByteString(length: i32) -> Result<Rc<SeqTwoByteString>, StringError> {
        if length < 0 {
            return Err(StringError::InvalidLength);
        }
        let vec = vec![0; length as usize];
        Ok(Rc::new(vec))
    }
    
    pub fn NewRawOneByteString(length: u32) -> Result<Rc<SeqOneByteString>, StringError> {
        if length > String::kMaxLength {
          return Err(StringError::InvalidLength);
        }
        
        let vec = vec![0; length as usize];
        Ok(Rc::new(vec))
    }
}

impl Isolate {
    fn factory(&mut self) -> Factory {
        Factory {}
    }
}

struct Factory {}

impl Factory {
    fn NewStringFromOneByte(&mut self, bytes: base::Vector<const u8>) -> Result<Rc<String>, StringError> {
        factory::NewStringFromOneByte(bytes)
    }
    
    fn NewProperSubString(&mut self, string: Rc<String>, start: i32, end: i32) -> Rc<String> {
        factory::NewProperSubString(string, start, end)
    }
    
    fn NewConsString(&mut self, first: Rc<String>, second: Rc<String>) -> Rc<String> {
        factory::NewConsString(first, second)
    }
    
    fn NewRawTwoByteString(&mut self, length: i32) -> Result<Rc<SeqTwoByteString>, StringError> {
        factory::NewRawTwoByteString(length)
    }
    
    fn NewRawOneByteString(&mut self, length: u32) -> Result<Rc<SeqOneByteString>, StringError> {
        factory::NewRawOneByteString(length)
    }
}

#[derive(Debug)]
enum StringError {
    URIError,
    InvalidLength,
    GenericError,
}

macro_rules! THROW_NEW_ERROR {
    ($isolate:expr, $error:expr) => {
        return Err(StringError::URIError);
    };
}

macro_rules! ASSIGN_RETURN_ON_EXCEPTION {
    ($isolate:expr, $var:expr, $val:expr) => {
        match $val {
            Ok(v) => $var = v,
            Err(e) => return Err(e),
        }
    };
}

fn CopyChars<T: Copy>(dest: &mut [T], src: &[T], size: usize) {
    dest[..size].copy_from_slice(src);
}

fn NewURIError() -> StringError {
    StringError::URIError
}

mod internal {
    use super::{
        base, DisallowGarbageCollection, DirectHandle, Isolate, MaybeDirectHandle, NewURIError,
        SeqOneByteString, SeqTwoByteString, String, StringError, THROW_NEW_ERROR, unibrow,
        Factory,Handle, ASSIGN_RETURN_ON_EXCEPTION, CopyChars
    };
    use std::rc::Rc;

    pub struct Uri {}

    impl Uri {
        pub fn DecodeUri(isolate: &mut Isolate, uri: DirectHandle<String>) -> MaybeDirectHandle<String> {
            Uri::Decode(isolate, uri, true)
        }

        pub fn DecodeUriComponent(
            isolate: &mut Isolate,
            component: DirectHandle<String>,
        ) -> MaybeDirectHandle<String> {
            Uri::Decode(isolate, component, false)
        }

        pub fn EncodeUri(isolate: &mut Isolate, uri: DirectHandle<String>) -> MaybeDirectHandle<String> {
            Uri::Encode(isolate, uri, true)
        }

        pub fn EncodeUriComponent(
            isolate: &mut Isolate,
            component: DirectHandle<String>,
        ) -> MaybeDirectHandle<String> {
            Uri::Encode(isolate, component, false)
        }

        pub fn Escape(isolate: &mut Isolate, string: Handle<String>) -> MaybeDirectHandle<String> {
            string = String::Flatten(isolate, string);
            Uri::EscapePrivate(isolate, string)
        }

        pub fn Unescape(isolate: &mut Isolate, string: Handle<String>) -> MaybeDirectHandle<String> {
            string = String::Flatten(isolate, string);
            Uri::UnescapePrivate(isolate, string)
        }

        fn Decode(isolate: &mut Isolate, uri: DirectHandle<String>, is_uri: bool) -> MaybeDirectHandle<String> {
            let uri = String::Flatten(isolate, uri);
            let mut one_byte_buffer: Vec<u8> = Vec::new();
            let mut two_byte_buffer: Vec<base::uc16> = Vec::new();

            if !Uri::into_one_and_two_byte(uri, is_uri, &mut one_byte_buffer, &mut two_byte_buffer) {
                THROW_NEW_ERROR!(isolate, NewURIError());
            }

            if two_byte_buffer.is_empty() {
                let result = isolate.factory().NewStringFromOneByte(base::Vector::from(one_byte_buffer))?;
                return Some(result);
            }

            let mut result: Rc<SeqTwoByteString>;
            let result_length: i32 = (one_byte_buffer.len() + two_byte_buffer.len()) as i32;
            ASSIGN_RETURN_ON_EXCEPTION!(
                isolate,
                result,
                isolate.factory().NewRawTwoByteString(result_length)
            );

            let no_gc = DisallowGarbageCollection {};
            let mut chars: &mut [u16] = Rc::get_mut(&mut result).unwrap();

            if !one_byte_buffer.is_empty() {
                CopyChars(chars, &one_byte_buffer.iter().map(|&x| x as u16).collect::<Vec<u16>>(), one_byte_buffer.len());
                chars = &mut chars[one_byte_buffer.len()..];
            }
            if !two_byte_buffer.is_empty() {
                CopyChars(chars, &two_byte_buffer, two_byte_buffer.len());
            }

            Some(result)
        }

        fn Encode(isolate: &mut Isolate, uri: DirectHandle<String>, is_uri: bool) -> MaybeDirectHandle<String> {
            let uri = String::Flatten(isolate, uri);
            let uri_length: usize = uri.len();
            let mut buffer: Vec<u8> = Vec::with_capacity(uri_length);

            let mut throw_error: bool = false;
            {
                let no_gc = DisallowGarbageCollection {};
                //let uri_content: String::FlatContent = uri.GetFlatContent(no_gc);

                for k in 0..uri_length {
                    let cc1: base::uc16 = uri[k];
                    if unibrow::Utf16::IsLeadSurrogate(cc1) {
                        let mut k = k + 1;
                        if k < uri_length {
                            let cc2: base::uc16 = uri[k];
                            if unibrow::Utf16::IsTrailSurrogate(cc2) {
                                Uri::encode_pair(cc1, cc2, &mut buffer);
                                continue;
                            }
                        }
                    } else if !unibrow::Utf16::IsTrailSurrogate(cc1) {
                        if Uri::is_unescape_predicate_in_uri_component(cc1) || (is_uri && Uri::is_uri_separator(cc1))
                        {
                            buffer.push(cc1 as u8);
                        } else {
                            Uri::encode_single(cc1, &mut buffer);
                        }
                        continue;
                    }
                    throw_error = true;
                    break;
                }
            }
            if throw_error {
                THROW_NEW_ERROR!(isolate, NewURIError());
            }
            let result = isolate.factory().NewStringFromOneByte(base::Vector::from(buffer))?;
            Some(result)
        }

        fn into_one_and_two_byte(
            uri: DirectHandle<String>,
            is_uri: bool,
            one_byte_buffer: &mut Vec<u8>,
            two_byte_buffer: &mut Vec<base::uc16>,
        ) -> bool {
            let no_gc = DisallowGarbageCollection {};
            //let uri_content: String::FlatContent = uri.GetFlatContent(no_gc);

            let uri_length: usize = uri.len();
            for k in 0..uri_length {
                let code: base::uc16 = uri[k];
                if code == '%' as u16 {
                    let mut two_digits: i32;
                    if k + 2 >= uri_length
                        || {
                            two_digits = Uri::two_digit_hex(uri[k + 1], uri[k + 2]);
                            two_digits
                        } < 0
                    {
                        return false;
                    }

                    let decoded: base::uc16 = two_digits as base::uc16;
                    if decoded > unibrow::Utf8::kMaxOneByteChar {
                        return Uri::into_two_byte(k, is_uri, uri_length, uri, two_byte_buffer);
                    }

                    Uri::add_to_buffer(decoded, uri[k], is_uri, one_byte_buffer);
                   let mut k = k + 2;
                   
                } else {
                    if code > unibrow::Utf8::kMaxOneByteChar {
                        return Uri::into_two_byte(k, is_uri, uri_length, uri, two_byte_buffer);
                    }
                    one_byte_buffer.push(code as u8);
                }
            }
            true
        }

        fn into_two_byte(
            index: usize,
            is_uri: bool,
            uri_length: usize,
            uri_content: DirectHandle<String>,
            buffer: &mut Vec<base::uc16>,
        ) -> bool {
            let mut k = index;
            while k < uri_length {
                let code: base::uc16 = uri_content[k];
                if code == '%' as u16 {
                    let mut two_digits: i32;
                    if k + 2 >= uri_length
                        || {
                            two_digits = Uri::two_digit_hex(uri_content[k + 1], uri_content[k + 2]);
                            two_digits
                        } < 0
                    {
                        return false;
                    }
                    let mut k = k + 2;
                    let decoded: base::uc16 = two_digits as base::uc16;
                    if decoded > unibrow::Utf8::kMaxOneByteChar {
                        let mut octets: [u8; unibrow::Utf8::kMaxEncodedSize] =
                            [0; unibrow::Utf8::kMaxEncodedSize];
                        octets[0] = decoded as u8;

                        let mut number_of_continuation_bytes: i32 = 0;
                        while ((decoded << (number_of_continuation_bytes + 1)) & 0x80) != 0 {
                            number_of_continuation_bytes = number_of_continuation_bytes + 1;
                            if number_of_continuation_bytes > 3 || k + 3 >= uri_length {
                                return false;
                            }
                            let mut k = k + 1;
                            if uri_content[k] != '%' as u16
                                || {
                                    two_digits = Uri::two_digit_hex(uri_content[k + 1], uri_content[k + 2]);
                                    two_digits
                                } < 0
                            {
                                return false;
                            }
                            let mut k = k + 2;
                            let continuation_byte: base::uc16 = two_digits as base::uc16;
                            octets[number_of_continuation_bytes as usize] = continuation_byte as u8;
                        }

                        if !Uri::decode_octets(&octets, number_of_continuation_bytes as usize, buffer) {
                            return false;
                        }
                    } else {
                        Uri::add_to_buffer(decoded, uri_content[k - 2], is_uri, buffer);
                    }
                } else {
                    buffer.push(code);
                }
                let mut k = k + 1;
            }
            true
        }

        fn decode_octets(octets: &[u8], length: usize, buffer: &mut Vec<base::uc16>) -> bool {
            let mut cursor: usize = 0;
            let value: base::uc32 = unibrow::Utf8::ValueOf(octets, length as i32, &mut cursor);
            if value == unibrow::Utf8::kBadChar && !Uri::is_replacement_character(octets, length) {
                return false;
            }

            if value <= unibrow::Utf16::kMaxNonSurrogateCharCode {
                buffer.push(value as base::uc16);
            } else {
                buffer.push(unibrow::Utf16::LeadSurrogate(value));
                buffer.push(unibrow::Utf16::TrailSurrogate(value));
            }
            true
        }

        fn is_replacement_character(octets: &[u8], length: usize) -> bool {
            if length != 3 || octets[0] != 0xEF || octets[1] != 0xBF || octets[2] != 0xBD {
                return false;
            }
            true
        }

        fn add_to_buffer<T: From<u16>>(decoded: base::uc16, original_char: base::uc16, is_uri: bool, buffer: &mut Vec<T>) {
            if is_uri && Uri::is_reserved_predicate(decoded) {
                buffer.push(('%' as u8) as T);
                let first: T = T::from((original_char as u16));
                let second: T = T::from((original_char as u16));
                buffer.push(first);
                buffer.push(second);
            } else {
                buffer.push(decoded.into());
            }
        }

        fn is_reserved_predicate(c: base::uc16) -> bool {
            match c {
                '#' | '$' | '&' | '+' | ',' | '/' | ':' | ';' | '=' | '?' | '@' => true,
                _ => false,
            }
        }

        fn two_digit_hex(character1: base::uc16, character2: base::uc16) -> i32 {
            if character1 > 'f' as u16 {
                return -1;
            }
            let high: i32 = base::HexValue(character1);
            if high == -1 {
                return -1;
            }
            if character2 > 'f' as u16 {
                return -1;
            }
            let low: i32 = base::HexValue(character2);
            if low == -1 {
                return -1;
            }
            (high << 4) + low
        }

        fn is_unescape_predicate_in_uri_component(c: base::uc16) -> bool {
            if strings::char_predicates_inl::IsAlphaNumeric(c) {
                return true;
            }

            match c {
                '!' | '\'' | '(' | ')' | '*' | '-' | '.' | '_' | '~' => true,
                _ => false,
            }
        }

        fn is_uri_separator(c: base::uc16) -> bool {
            match c {
                '#' | ':' | ';' | '/' | '?' | '$' | '&' | '+' | ',' | '@' | '=' => true,
                _ => false,
            }
        }

        fn add_encoded_octet_to_buffer(octet: u8, buffer: &mut Vec<u8>) {
            buffer.push('%' as u8);
            buffer.push(base::HexCharOfValue(octet >> 4));
            buffer.push(base::HexCharOfValue(octet & 0x0F));
        }

        fn encode_single(c: base::uc16, buffer: &mut Vec<u8>) {
            let mut s: [char; 4] = ['\0'; 4];
            let number_of_bytes: i32 = unibrow::Utf8::Encode(
                &mut s,
                c,
                unibrow::Utf16::kNoPreviousCharacter,
                false,
            );
            for k in 0..number_of_bytes {
                Uri::add_encoded_octet_to_buffer(s[k as usize] as u8, buffer);
            }
        }

        fn encode_pair(cc1: base::uc16, cc2: base::uc16, buffer: &mut Vec<u8>) {
            let mut s: [char; 4] = ['\0'; 4];
            let number_of_bytes: i32 = unibrow::Utf8::Encode(
                &mut s,
                unibrow::Utf16::CombineSurrogatePair(cc1, cc2) as u16,
                unibrow::Utf16::kNoPreviousCharacter,
                false,
            );
            for k in 0..number_of_bytes {
                Uri::add_encoded_octet_to_buffer(s[k as usize] as u8, buffer);
            }
        }
        
        fn EscapePrivate(isolate: &mut Isolate, string: Handle<String>) -> MaybeDirectHandle<String> {
            if String::IsOneByteRepresentationUnderneath(*string) {
                Uri::EscapePrivateImpl::<u8>(isolate, string)
            } else {
                Uri::EscapePrivateImpl::<base::uc16>(isolate, string)
            }
        }
        
        fn UnescapePrivate(isolate: &mut Isolate, string: Handle<String>) -> MaybeDirectHandle<String> {
            if String::IsOneByteRepresentationUnderneath(*string) {
                Uri::UnescapePrivateImpl::<u8>(isolate, string)
            } else {
                Uri::UnescapePrivateImpl::<base::uc16>(isolate, string)
            }
        }
        
        fn EscapePrivateImpl<Char>(isolate: &mut Isolate, string: Handle<String>) -> MaybeDirectHandle<String>
            where Char: From<u8> + Into<u16> + Copy + std::cmp::PartialEq, {
            let length = string.len() as u32;
            let mut escaped_length: u32 = 0;

            for i in 0..length {
                let c: u16 = string[i as usize];
                if c >= 256 {
                    escaped_length += 6;
                } else if Uri::is_not_escaped(c) {
                    escaped_length += 1;
                } else {
                    escaped_length += 3;
                }

                if escaped_length > String::kMaxLength {
                    return Some(string.clone()); // Provoke exception.
                }
            }

            if escaped_length == length {
                return Some(string);
            }

            let mut dest: Rc<SeqOneByteString>;
            ASSIGN_RETURN_ON_EXCEPTION!(
                isolate,
                dest,
                isolate.factory().NewRawOneByteString(escaped_length)
            );
            let mut dest_position: usize = 0;
            
            let mut dest_mut = Rc::get_mut(&mut dest).unwrap();

            for i in 0..length {
                let c: u16 = string[i as usize];
                if c >= 256 {
                    dest_mut[dest_position] = '%' as u8;
                    dest_mut[dest_position + 1] = 'u' as u8;
                    dest_mut[dest_position + 2] = base::HexCharOfValue((c >> 12) as u8);
                    dest_mut[dest_position + 3] = base::HexCharOfValue(((c >> 8) & 0xF) as u8);
                    dest_mut[dest_position + 4] = base::HexCharOfValue(((c >> 4) & 0xF) as u8);
                    dest_mut[dest_position + 5] = base::HexCharOfValue((c & 0xF) as u8);
                    dest_position += 6;
                } else if Uri::is_not_escaped(c) {
                    dest_mut[dest_position] = c as u8;
                    dest_position += 1;
                } else {
                    dest_mut[dest_position] = '%' as u8;
                    dest_mut[dest_position + 1] = base::HexCharOfValue((c >> 4) as u8);
                    dest_mut[dest_position + 2] = base::HexCharOfValue((

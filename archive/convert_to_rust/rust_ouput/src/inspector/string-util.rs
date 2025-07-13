// Converted from V8 C++ source files:
// Header: string-util.h
// Implementation: string-util.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    use std::rc::Rc;
    use std::cell::RefCell;

    pub struct String;

    pub struct Binary {
        bytes_: std::sync::Arc<std::vec::Vec<u8>>,
    }

    impl Binary {
        pub fn new() -> Binary {
            Binary {
                bytes_: std::sync::Arc::new(std::vec::Vec::new()),
            }
        }

        pub fn data(&self) -> *const u8 {
            self.bytes_.as_ptr() as *const u8
        }

        pub fn size(&self) -> usize {
            self.bytes_.len()
        }

        pub fn to_base64(&self) -> String16 {
            let table =
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            if self.size() == 0 {
                return String16::new();
            }
            let mut result = String16Builder::new();
            result.reserve(4 * ((self.size() + 2) / 3));
            let mut last = 0;
            let bytes = self.bytes_.clone();
            for n in 0..self.size() {
                let split = Self::split_byte(bytes[n], 2 + 2 * (n % 3));
                result.append_char(table.as_bytes()[split.0 as usize] as char);

                if (n + 1) < self.size() && (n + 1) % 3 == 0 {
                    result.append_char(table.as_bytes()[split.1 as usize] as char);
                    last = 0;
                } else {
                    last = split.1;
                }
            }
            result.append_char(table.as_bytes()[last as usize] as char);
            while result.len() % 4 > 0 {
                result.append_char('=');
            }
            result.to_string()
        }

        pub fn from_base64(base64: &String16, success: &mut bool) -> Binary {
            if base64.is_empty() {
                *success = true;
                return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) };
            }

            *success = false;
            if base64.length() % 4 != 0 || base64.length() + 4 < base64.length() {
                return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) };
            }

            let mut result = std::vec::Vec::new();
            result.reserve(3 * base64.length() / 4);
            let pad = '=';
            for i in (0..base64.length()).step_by(4) {
                let mut a: u8 = 0;
                let mut b: u8 = 0;
                let mut c: u8 = 0;
                let mut d: u8 = 0;

                match Self::decode_byte(base64.string()[i]).unwrap() {
                    Some(val) => a = val,
                    None => return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) },
                };
                match Self::decode_byte(base64.string()[i + 1]).unwrap() {
                    Some(val) => b = val,
                    None => return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) },
                };
                match Self::decode_byte(base64.string()[i + 2]).unwrap() {
                    Some(val) => c = val,
                    None => {
                        if i + 4 < base64.length() || base64.string()[i + 2] != pad || base64.string()[i + 3] != pad {
                            return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) };
                        }
                    }
                };
                match Self::decode_byte(base64.string()[i + 3]).unwrap() {
                    Some(val) => d = val,
                    None => {
                        if i + 4 < base64.length() || base64.string()[i + 3] != pad {
                            return Binary { bytes_: std::sync::Arc::new(std::vec::Vec::new()) };
                        }
                    }
                };

                result.push((a << 2) | (b >> 4));
                if base64.string()[i + 2] != '=' {
                    result.push((0xFF & (b << 4)) | (c >> 2));
                }
                if base64.string()[i + 3] != '=' {
                    result.push((0xFF & (c << 6)) | d);
                }
            }
            *success = true;
            Binary { bytes_: std::sync::Arc::new(result) }
        }

        fn split_byte(byte: u8, split: u8) -> (u8, u8) {
            (byte >> split, (byte & ((1 << split) - 1)) << (6 - split))
        }

        fn decode_byte(byte: char) -> Result<Option<u8>, String> {
            if 'A' <= byte && byte <= 'Z' {
                return Ok(Some((byte as u8) - ('A' as u8)));
            }
            if 'a' <= byte && byte <= 'z' {
                return Ok(Some((byte as u8) - ('a' as u8) + 26));
            }
            if '0' <= byte && byte <= '9' {
                return Ok(Some((byte as u8) - ('0' as u8) + 26 + 26));
            }
            if byte == '+' {
                return Ok(Some(62));
            }
            if byte == '/' {
                return Ok(Some(63));
            }
            Ok(None)
        }

        pub fn from_span(span: v8_crdtp::Span<u8>) -> Binary {
            Binary {
                bytes_: std::sync::Arc::new(span.data.to_vec()),
            }
        }
    }
}

pub struct String16Builder {
    string: std::string::String,
}

impl String16Builder {
    pub fn new() -> String16Builder {
        String16Builder { string: std::string::String::new() }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.string.reserve(additional);
    }

    pub fn append_char(&mut self, c: char) {
        self.string.push(c);
    }

    pub fn append_number(&mut self, num: usize) {
        self.string.push_str(&num.to_string());
    }

    pub fn to_string(self) -> String16 {
        String16 { string: self.string.encode_utf16().collect() }
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}

pub struct String16 {
    string: std::vec::Vec<u16>,
}

impl String16 {
    pub fn new() -> String16 {
        String16 { string: std::vec::Vec::new() }
    }

    pub fn from_utf8(data: &str, length: usize) -> String16 {
        let mut string: Vec<u16> = Vec::new();
        for c in data.chars() {
            string.push(c as u16);
        }
        String16 { string }
    }

    pub fn from_utf16le(data: &[u16], length: usize) -> String16 {
        String16 { string: data.to_vec() }
    }
    pub fn characters16(&self) -> *const u16 {
        self.string.as_ptr()
    }
    pub fn length(&self) -> usize {
        self.string.len()
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub fn string(&self) -> Vec<char> {
        self.string.iter().map(|&c| char::from_u32(c as u32).unwrap()).collect()
    }
}

pub struct StringView {
    data: *const u8,
    length: usize,
    is_8bit: bool,
}

impl StringView {
    pub fn new() -> StringView {
        StringView { data: std::ptr::null(), length: 0, is_8bit: true }
    }

    pub fn from_string(s: &String16) -> StringView {
        StringView {
            data: s.string.as_ptr() as *const u8,
            length: s.length(),
            is_8bit: false,
        }
    }

    pub fn characters8(&self) -> *const u8 {
        self.data
    }

    pub fn characters16(&self) -> *const u16 {
        self.data as *const u16
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is8Bit(&self) -> bool {
        self.is_8bit
    }
}

pub struct StringBuffer {}

impl StringBuffer {
    pub fn create(string: StringView) -> Box<dyn StringBufferTrait> {
        if string.length() == 0 {
            return Box::new(EmptyStringBuffer {});
        }
        if string.is8Bit() {
            let data = unsafe { std::slice::from_raw_parts(string.characters8(), string.length()) };
            return Box::new(StringBuffer8 { data: data.to_vec() });
        }
        let data = unsafe { std::slice::from_raw_parts(string.characters16() as *const u8, string.length() * 2) };
        let utf16_vec: Vec<u16> = data.chunks(2).map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]])).collect();

        Box::new(StringBuffer16 { data: String16 { string: utf16_vec } })
    }
}

pub trait StringBufferTrait {
    fn string(&self) -> StringView;
}

struct EmptyStringBuffer {}

impl StringBufferTrait for EmptyStringBuffer {
    fn string(&self) -> StringView {
        StringView::new()
    }
}

struct StringBuffer8 {
    data: std::vec::Vec<u8>,
}

impl StringBufferTrait for StringBuffer8 {
    fn string(&self) -> StringView {
        StringView {
            data: self.data.as_ptr(),
            length: self.data.len(),
            is_8bit: true,
        }
    }
}

struct StringBuffer16 {
    data: String16,
}

impl StringBufferTrait for StringBuffer16 {
    fn string(&self) -> StringView {
        StringView::from_string(&self.data)
    }
}

pub fn string_view_starts_with(string: &StringView, prefix: &str) -> bool {
    if string.length() == 0 {
        return prefix.len() == 0;
    }
    if string.is8Bit() {
        let string_bytes = unsafe { std::slice::from_raw_parts(string.characters8(), string.length()) };
        let prefix_bytes = prefix.as_bytes();
        if prefix_bytes.len() > string.length() {
            return false;
        }
        for i in 0..prefix_bytes.len() {
            if string_bytes[i] != prefix_bytes[i] {
                return false;
            }
        }
        return true;
    } else {
        let string_u16 = unsafe { std::slice::from_raw_parts(string.characters16(), string.length()) };
        let prefix_u16: Vec<u16> = prefix.encode_utf16().collect();

        if prefix_u16.len() > string.length() {
            return false;
        }

        for i in 0..prefix_u16.len() {
            if string_u16[i] != prefix_u16[i] {
                return false;
            }
        }
        return true;
    }
}

pub fn string_buffer_from(str: String16) -> Box<dyn StringBufferTrait> {
    if str.is_empty() {
        return Box::new(EmptyStringBuffer {});
    }
    Box::new(StringBuffer16 { data: str })
}

pub fn string_buffer_from_vec(str: std::vec::Vec<u8>) -> Box<dyn StringBufferTrait> {
    if str.is_empty() {
        return Box::new(EmptyStringBuffer {});
    }
    Box::new(StringBuffer8 { data: str })
}

pub fn stack_trace_id_to_string(id: usize) -> String16 {
    let mut builder = String16Builder::new();
    builder.append_number(id);
    builder.to_string()
}

pub mod v8_crdtp {
    use super::{String16, StringView};
    use super::protocol::Binary;
    use std::any::Any;

    #[derive(Debug, Clone)]
    pub struct Span<T> {
        pub data: Vec<T>,
    }

    impl<T> Span<T> {
        pub fn new(data: Vec<T>) -> Self {
            Span { data }
        }

        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }
    }

    pub struct DeserializerState<'a> {
        tokenizer: &'a mut CBORTokenizer,
        errors: Vec<Error>,
    }

    impl<'a> DeserializerState<'a> {
        pub fn new(tokenizer: &'a mut CBORTokenizer) -> Self {
            DeserializerState {
                tokenizer,
                errors: Vec::new(),
            }
        }

        pub fn tokenizer(&mut self) -> &mut CBORTokenizer {
            &mut self.tokenizer
        }

        pub fn register_error(&mut self, error: Error) {
            self.errors.push(error);
        }

        pub fn errors(&self) -> &Vec<Error> {
            &self.errors
        }
    }

    pub enum Error {
        BINDINGS_STRING_VALUE_EXPECTED,
        BINDINGS_BINARY_VALUE_EXPECTED,
        Other(String),
    }

    pub trait ProtocolTypeTraits<T> {
        fn deserialize(state: &mut DeserializerState, value: &mut T) -> bool;
        fn serialize(value: &T, bytes: &mut Vec<u8>);
    }

    impl ProtocolTypeTraits<String16> for String16 {
        fn deserialize(state: &mut DeserializerState, value: &mut String16) -> bool {
            let tokenizer = state.tokenizer();
            match tokenizer.token_tag() {
                CBORTokenTag::STRING8 => {
                    let str = tokenizer.get_string8();
                    *value = String16::from_utf8(&String::from_utf8(str.data).unwrap(), str.size);
                    true
                }
                CBORTokenTag::STRING16 => {
                    let str = tokenizer.get_string16_wire_rep();
                    let utf16_slice: &[u16] = unsafe {
                        std::slice::from_raw_parts(str.data as *const u16, str.size / 2)
                    };
                    *value = String16::from_utf16le(utf16_slice, str.size / 2);
                    true
                }
                _ => {
                    state.register_error(Error::BINDINGS_STRING_VALUE_EXPECTED);
                    false
                }
            }
        }

        fn serialize(value: &String16, bytes: &mut Vec<u8>) {
            encode_from_utf16(
                Span {
                    data: value.string.clone(),
                },
                bytes,
            );
        }
    }

    impl ProtocolTypeTraits<Binary> for Binary {
        fn deserialize(state: &mut DeserializerState, value: &mut Binary) -> bool {
            let tokenizer = state.tokenizer();
            match tokenizer.token_tag() {
                CBORTokenTag::BINARY => {
                    *value = Binary::from_span(tokenizer.get_binary());
                    true
                }
                CBORTokenTag::STRING8 => {
                    let str_span = tokenizer.get_string8();
                    let str = String16::from_utf8(&String::from_utf8(str_span.data).unwrap(), str_span.size);
                    let mut success = false;
                    *value = Binary::from_base64(&str, &mut success);
                    success
                }
                _ => {
                    state.register_error(Error::BINDINGS_BINARY_VALUE_EXPECTED);
                    false
                }
            }
        }

        fn serialize(value: &Binary, bytes: &mut Vec<u8>) {
            encode_binary(
                Span {
                    data: unsafe { std::slice::from_raw_parts(value.data(), value.size()).to_vec() },
                },
                bytes,
            );
        }
    }

    #[derive(Debug)]
    pub struct CBORTokenizer {
        token_tag: CBORTokenTag,
    }

    impl CBORTokenizer {
        pub fn new(token_tag: CBORTokenTag) -> Self {
            CBORTokenizer { token_tag }
        }

        pub fn token_tag(&self) -> CBORTokenTag {
            self.token_tag
        }

        pub fn get_string8(&self) -> String8 {
            String8 { data: b"test".to_vec(), size: 4 }
        }

        pub fn get_string16_wire_rep(&self) -> String16WireRep {
            String16WireRep { data: b"test".to_vec(), size: 4 }
        }

        pub fn get_binary(&self) -> Span<u8> {
            Span::new(b"test".to_vec())
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum CBORTokenTag {
        STRING8,
        STRING16,
        BINARY,
        NONE,
    }

    #[derive(Debug)]
    pub struct String8 {
        pub data: Vec<u8>,
        pub size: usize,
    }

    #[derive(Debug)]
    pub struct String16WireRep {
        pub data: Vec<u8>,
        pub size: usize,
    }

    fn encode_from_utf16(value: Span<u16>, bytes: &mut Vec<u8>) {
        println!("Encoding from UTF16: {:?}", value.data);
        bytes.extend_from_slice(&[0x60]);
    }

    fn encode_binary(value: Span<u8>, bytes: &mut Vec<u8>) {
        println!("Encoding binary data: {:?}", value.data);
        bytes.extend_from_slice(&[0x40]);
    }
}

extern crate libc;
use self::libc::{c_char, c_void};

pub struct V8 {}

pub fn to_v8_string(isolate: *mut V8, string: &String16) -> *mut V8 {
    isolate
}

pub fn to_v8_string_internalized(isolate: *mut V8, string: &String16) -> *mut V8 {
    isolate
}

pub fn to_v8_string_internalized_char(isolate: *mut V8, str: *const c_char) -> *mut V8 {
    isolate
}

pub fn to_v8_string_view(isolate: *mut V8, string: &StringView) -> *mut V8 {
    isolate
}

pub fn to_protocol_string(isolate: *mut V8, value: *mut V8) -> String16 {
    String16::new()
}

pub fn to_protocol_string_with_type_check(isolate: *mut V8, value: *mut V8) -> String16 {
    String16::new()
}

pub fn to_string16(string: &StringView) -> String16 {
    if string.length() == 0 {
        return String16::new();
    }
    if string.is8Bit() {
        let slice = unsafe { std::slice::from_raw_parts(string.characters8(), string.length()) };
        let s = String::from_utf8_lossy(slice);
        return String16::from_utf8(&s, s.len());
    } else {
        let slice = unsafe { std::slice::from_raw_parts(string.characters16(), string.length()) };
        return String16::from_utf16le(slice, string.length());
    }
}

pub fn to_string_view(string: &String16) -> StringView {
    if string.is_empty() {
        return StringView::new();
    }
    StringView::from_string(string)
}

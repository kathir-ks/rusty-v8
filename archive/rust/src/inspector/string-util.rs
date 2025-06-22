// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use base64::{decode, encode};
use std::char;
use std::fmt;
use std::string::FromUtf16Error;

pub mod protocol {
    use super::*;
    use std::borrow::Cow;

    pub struct Binary {
        bytes_: Option<std::sync::Arc<Vec<u8>>>,
    }

    impl Binary {
        pub fn new(bytes: Vec<u8>) -> Self {
            Binary {
                bytes_: Some(std::sync::Arc::new(bytes)),
            }
        }

        pub fn empty() -> Self {
            Binary { bytes_: None }
        }

        pub fn from_span(span: &[u8]) -> Self {
            Binary::new(span.to_vec())
        }

        pub fn size(&self) -> usize {
            self.bytes_.as_ref().map(|v| v.len()).unwrap_or(0)
        }

        pub fn data(&self) -> &[u8] {
            self.bytes_.as_ref().map(|v| v.as_slice()).unwrap_or(&[])
        }

        pub fn to_base64(&self) -> String16 {
            if self.size() == 0 {
                return String16::new();
            }

            let bytes = self.bytes_.as_ref().unwrap();
            let encoded = encode(bytes);
            String16::from_str(&encoded)
        }

        pub fn from_base64(base64: &String16, success: &mut bool) -> Self {
            if base64.is_empty() {
                *success = true;
                return Binary::empty();
            }

            *success = false;
            if base64.len() % 4 != 0 || base64.len().checked_add(4).is_none() {
                return Binary::empty();
            }

            let decoded = match decode(base64.to_string()) {
                Ok(data) => data,
                Err(_) => return Binary::empty(),
            };

            *success = true;
            Binary::new(decoded)
        }

        pub fn bytes(&self) -> Option<std::sync::Arc<Vec<u8>>> {
            self.bytes_.clone()
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash, Default)]
    pub struct String16 {
        data: Vec<u16>,
    }

    impl String16 {
        pub fn new() -> Self {
            String16 { data: Vec::new() }
        }

        pub fn from_str(s: &str) -> Self {
            String16 {
                data: s.encode_utf16().collect(),
            }
        }

        pub fn from_utf16(data: &[u16]) -> Result<Self, FromUtf16Error> {
            String::from_utf16(data).map(|s| String16::from_str(&s))
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn characters16(&self) -> &[u16] {
            &self.data
        }

        pub fn to_string(&self) -> String {
            String::from_utf16_lossy(&self.data)
        }
    }

    impl fmt::Display for String16 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    pub mod StringUtil {
        use super::*;
        use std::str;

        pub fn from_utf8(data: &[u8], size: usize) -> String16 {
            match str::from_utf8(&data[..size]) {
                Ok(s) => String16::from_str(s),
                Err(_) => String16::new(), // Handle error appropriately
            }
        }

        pub fn from_utf16le(data: &[u16], size: usize) -> String16 {
            let utf16_data: Vec<u16> = data[..size].to_vec();
            match String::from_utf16(&utf16_data) {
                Ok(s) => String16::from_str(&s),
                Err(_) => String16::new(), // Handle error appropriately
            }
        }
    }
}

// The following code requires a V8 Isolate and local string implementation,
// which are not available in a standard Rust environment.
// Therefore, the conversion is omitted, and a placeholder is provided.
// Relevant V8 API calls include:
// - v8::String::Empty(isolate)
// - v8::String::NewFromTwoByte(...)
// - v8::String::NewFromUtf8(...)
// - v8::String::NewFromOneByte(...)
// - value->WriteV2(...)
// - value->IsString()

/*
fn to_v8_string(isolate: &Isolate, string: &String16) -> Local<String> {
    // Placeholder implementation
    String::NewFromUtf8(isolate, string.to_string().as_ptr() as *const i8, NewStringType::kNormal)
        .ToLocalChecked()
}
*/

pub struct StringView<'a> {
    characters8: Option<&'a [u8]>,
    characters16: Option<&'a [u16]>,
    length: usize,
    is8bit: bool,
}

impl<'a> StringView<'a> {
    pub fn new() -> Self {
        StringView {
            characters8: None,
            characters16: None,
            length: 0,
            is8bit: false,
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        StringView {
            characters8: Some(s.as_bytes()),
            characters16: None,
            length: s.len(),
            is8bit: true,
        }
    }

    pub fn from_u16(s: &'a [u16]) -> Self {
        StringView {
            characters8: None,
            characters16: Some(s),
            length: s.len(),
            is8bit: false,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is8bit(&self) -> bool {
        self.is8bit
    }

    pub fn characters8(&self) -> &[u8] {
        self.characters8.unwrap_or(&[])
    }

    pub fn characters16(&self) -> &[u16] {
        self.characters16.unwrap_or(&[])
    }
}

pub fn to_string16(string: &StringView) -> protocol::String16 {
    if string.length() == 0 {
        return protocol::String16::new();
    }
    if string.is8bit() {
        let s = String::from_utf8_lossy(string.characters8());
        protocol::String16::from_str(&s)
    } else {
        protocol::String16::from_utf16(string.characters16()).unwrap() //FIXME: Handle error
    }
}

pub fn to_string_view(string: &protocol::String16) -> StringView {
    if string.is_empty() {
        return StringView::new();
    }
    StringView::from_u16(string.characters16())
}

pub fn string_view_starts_with(string: &StringView, prefix: &str) -> bool {
    if string.length() == 0 {
        return prefix.is_empty();
    }
    if string.is8bit() {
        let s = string.characters8();
        s.starts_with(prefix.as_bytes())
    } else {
        let s = string.characters16();
        let prefix_u16: Vec<u16> = prefix.encode_utf16().collect();
        s.starts_with(&prefix_u16)
    }
}

pub trait StringBuffer {
    fn string(&self) -> StringView;
}

struct EmptyStringBuffer {}

impl StringBuffer for EmptyStringBuffer {
    fn string(&self) -> StringView {
        StringView::new()
    }
}

struct StringBuffer8 {
    data: Vec<u8>,
}

impl StringBuffer8 {
    fn new(data: Vec<u8>) -> Self {
        StringBuffer8 { data }
    }
}

impl StringBuffer for StringBuffer8 {
    fn string(&self) -> StringView {
        StringView::from_str(std::str::from_utf8(&self.data).unwrap_or(""))
    }
}

struct StringBuffer16 {
    data: protocol::String16,
}

impl StringBuffer16 {
    fn new(data: protocol::String16) -> Self {
        StringBuffer16 { data }
    }
}

impl StringBuffer for StringBuffer16 {
    fn string(&self) -> StringView {
        StringView::from_u16(self.data.characters16())
    }
}

pub fn string_buffer_create(string: StringView) -> Box<dyn StringBuffer> {
    if string.length() == 0 {
        Box::new(EmptyStringBuffer {})
    } else if string.is8bit() {
        Box::new(StringBuffer8::new(string.characters8().to_vec()))
    } else {
        Box::new(StringBuffer16::new(protocol::String16::from_utf16(string.characters16()).unwrap())) // FIXME: Handle Error
    }
}

pub fn string_buffer_from(str: protocol::String16) -> Box<dyn StringBuffer> {
    if str.is_empty() {
        Box::new(EmptyStringBuffer {})
    } else {
        Box::new(StringBuffer16::new(str))
    }
}

pub fn string_buffer_from_vec(str: Vec<u8>) -> Box<dyn StringBuffer> {
    if str.is_empty() {
        Box::new(EmptyStringBuffer {})
    } else {
        Box::new(StringBuffer8::new(str))
    }
}

pub fn stack_trace_id_to_string(id: usize) -> protocol::String16 {
    id.to_string().into()
}

impl From<String> for protocol::String16 {
    fn from(s: String) -> Self {
        protocol::String16::from_str(&s)
    }
}

pub mod v8_crdtp {
    use super::*;
    use cbor::{Decoder, Encoder};

    pub trait ProtocolTypeTraits<T> {
        fn deserialize(state: &mut DeserializerState, value: &mut T) -> bool;
        fn serialize(value: &T, bytes: &mut Vec<u8>);
    }

    pub struct DeserializerState<'a> {
        tokenizer: &'a mut Decoder<'a>,
        errors: Vec<String>,
    }

    impl<'a> DeserializerState<'a> {
        pub fn new(tokenizer: &'a mut Decoder<'a>) -> Self {
            DeserializerState {
                tokenizer,
                errors: Vec::new(),
            }
        }

        pub fn tokenizer(&mut self) -> &mut Decoder<'a> {
            self.tokenizer
        }

        pub fn register_error(&mut self, error: Error) {
            self.errors.push(format!("{:?}", error));
        }

        pub fn has_errors(&self) -> bool {
            !self.errors.is_empty()
        }
    }

    #[derive(Debug)]
    pub enum Error {
        BINDINGS_STRING_VALUE_EXPECTED,
        BINDINGS_BINARY_VALUE_EXPECTED,
    }

    impl ProtocolTypeTraits<protocol::String16> for protocol::String16 {
        fn deserialize(state: &mut DeserializerState, value: &mut protocol::String16) -> bool {
            match state.tokenizer().token_type() {
                Some(cbor::Type::Text) => {
                    let s = state.tokenizer().text().unwrap();
                    *value = protocol::String16::from_str(s);
                    true
                }
                _ => {
                    state.register_error(Error::BINDINGS_STRING_VALUE_EXPECTED);
                    false
                }
            }
        }

        fn serialize(value: &protocol::String16, bytes: &mut Vec<u8>) {
            let mut encoder = Encoder::new(bytes);
            encoder.text(&value.to_string()).unwrap();
        }
    }

    impl ProtocolTypeTraits<protocol::Binary> for protocol::Binary {
        fn deserialize(state: &mut DeserializerState, value: &mut protocol::Binary) -> bool {
            match state.tokenizer().token_type() {
                Some(cbor::Type::Bytes) => {
                    let b = state.tokenizer().bytes().unwrap();
                    *value = protocol::Binary::from_span(b);
                    true
                }
                Some(cbor::Type::Text) => {
                    let s = state.tokenizer().text().unwrap();
                    let mut success = false;
                    let string16 = protocol::String16::from_str(s);
                    *value = protocol::Binary::from_base64(&string16, &mut success);
                    success
                }
                _ => {
                    state.register_error(Error::BINDINGS_BINARY_VALUE_EXPECTED);
                    false
                }
            }
        }

        fn serialize(value: &protocol::Binary, bytes: &mut Vec<u8>) {
            let mut encoder = Encoder::new(bytes);
            encoder.bytes(value.data()).unwrap();
        }
    }
}
// Converted from V8 C++ source files:
// Header: string-16.h
// Implementation: string-16.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/inspector/string-16.h
use std::{
    cmp,
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Add, AddAssign, Index},
    slice,
    str,
};

pub type UChar = u16;

#[derive(Clone, Debug, Eq, Ord)]
pub struct String16 {
    m_impl: Vec<UChar>,
    hash_code: std::cell::Cell<usize>,
}

impl String16 {
    pub const kNotFound: usize = usize::MAX;

    pub fn new() -> Self {
        String16 {
            m_impl: Vec::new(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        String16 {
            m_impl: Vec::with_capacity(capacity),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_str(s: &str) -> Self {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        String16 {
            m_impl: utf16,
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_utf16(utf16: &[u16]) -> Result<Self, std::string::FromUtf16Error> {
        String::from_utf16(utf16).map(|s| String16::from_str(&s))
    }

    pub fn from_utf16_lossy(utf16: &[u16]) -> Self {
        String::from_utf16_lossy(utf16).into()
    }

    pub fn from_string(s: String) -> Self {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        String16 {
            m_impl: utf16,
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn as_string(&self) -> String {
        String::from_utf16_lossy(&self.m_impl)
    }

    pub fn from_chars(characters: &[UChar]) -> Self {
        String16 {
            m_impl: characters.to_vec(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_u8_slice(characters: &[u8]) -> Self {
        let mut utf16 = Vec::with_capacity(characters.len());
        for &byte in characters {
            utf16.push(byte as u16);
        }
        String16 {
            m_impl: utf16,
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_integer(number: i32) -> Self {
        String16::from_str(&number.to_string())
    }

    pub fn from_usize(number: usize) -> Self {
        String16::from_str(&number.to_string())
    }

    pub fn from_integer64(number: i64) -> Self {
        String16::from_str(&number.to_string())
    }

    pub fn from_uint64(number: u64) -> Self {
        String16::from_str(&number.to_string())
    }

    pub fn from_double(number: f64) -> Self {
        String16::from_str(&number.to_string())
    }

    pub fn from_double_precision(number: f64, precision: i32) -> Self {
        String16::from_str(&format!("{:.prec$}", number, prec = precision as usize))
    }

    pub fn to_integer64(&self, ok: Option<&mut bool>) -> i64 {
        if self.m_impl.is_empty() {
            if let Some(ok_ptr) = ok {
                *ok_ptr = false;
            }
            return 0;
        }

        let s = self.as_string();
        match s.parse::<i64>() {
            Ok(result) => {
                if let Some(ok_ptr) = ok {
                    *ok_ptr = true;
                }
                result
            }
            Err(_) => {
                if let Some(ok_ptr) = ok {
                    *ok_ptr = false;
                }
                0
            }
        }
    }

    pub fn to_uint64(&self, ok: Option<&mut bool>) -> u64 {
        if self.m_impl.is_empty() {
            if let Some(ok_ptr) = ok {
                *ok_ptr = false;
            }
            return 0;
        }

        let s = self.as_string();
        match s.parse::<u64>() {
            Ok(result) => {
                if let Some(ok_ptr) = ok {
                    *ok_ptr = true;
                }
                result
            }
            Err(_) => {
                if let Some(ok_ptr) = ok {
                    *ok_ptr = false;
                }
                0
            }
        }
    }

    pub fn to_integer(&self, ok: Option<&mut bool>) -> i32 {
        let result64 = self.to_integer64(ok);
        if let Some(ok_ptr) = ok {
            if *ok_ptr {
                *ok_ptr = result64 <= i32::max_value() as i64 && result64 >= i32::min_value() as i64;
                if !*ok_ptr {
                   return 0;
                }
            } else {
                return 0;
            }
        }
        result64 as i32
    }

    pub fn get_trimmed_offset_and_length(&self) -> (usize, usize) {
        if self.m_impl.is_empty() {
            return (0, 0);
        }

        let mut start = 0;
        let mut end = self.m_impl.len() - 1;

        while start <= end && is_space_or_newline(self.m_impl[start]) {
            start += 1;
        }

        if start > end {
            return (0, 0);
        }

        while end > 0 && is_space_or_newline(self.m_impl[end]) {
            end -= 1;
        }

        (start, end + 1 - start)
    }

    pub fn strip_white_space(&self) -> Self {
        let (offset, length) = self.get_trimmed_offset_and_length();
        if length == 0 {
            String16::new()
        } else if offset == 0 && length == self.m_impl.len() {
            self.clone()
        } else {
            self.substring(offset, length)
        }
    }

    pub fn characters16(&self) -> &[u16] {
        &self.m_impl
    }

    pub fn length(&self) -> usize {
        self.m_impl.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m_impl.is_empty()
    }

    pub fn substring(&self, pos: usize, len: usize) -> Self {
        let end = cmp::min(pos + len, self.m_impl.len());
        if pos >= self.m_impl.len() {
            return String16::new();
        }
        String16 {
            m_impl: self.m_impl[pos..end].to_vec(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn find(&self, str: &String16, start: usize) -> usize {
        if start > self.m_impl.len() {
            return String16::kNotFound;
        }

        if str.m_impl.is_empty() {
            return start;
        }

        let mut i = start;
        while i + str.m_impl.len() <= self.m_impl.len() {
            if self.m_impl[i..i + str.m_impl.len()] == str.m_impl[..] {
                return i;
            }
            i += 1;
        }
        String16::kNotFound
    }

    pub fn reverse_find(&self, str: &String16, start: usize) -> usize {
        if self.m_impl.is_empty() || str.m_impl.is_empty() {
            return String16::kNotFound;
        }

        let mut start_index = if start == String16::kNotFound {
            self.m_impl.len().saturating_sub(1)
        } else {
            cmp::min(start, self.m_impl.len().saturating_sub(1))
        };

        if start_index >= self.m_impl.len() {
            return String16::kNotFound;
        }

        while start_index >= str.m_impl.len() - 1 {
            if self.m_impl[start_index - (str.m_impl.len() - 1)..=start_index] == str.m_impl[..] {
                return start_index - (str.m_impl.len() - 1);
            }
            if start_index == 0 {
                break;
            }
            start_index -= 1;
        }
        String16::kNotFound
    }

    pub fn find_char(&self, c: UChar, start: usize) -> usize {
        if start >= self.m_impl.len() {
            return String16::kNotFound;
        }
        for (i, &ch) in self.m_impl[start..].iter().enumerate() {
            if ch == c {
                return start + i;
            }
        }
        String16::kNotFound
    }

    pub fn reverse_find_char(&self, c: UChar, start: usize) -> usize {
        if self.m_impl.is_empty() {
            return String16::kNotFound;
        }

        let mut start_index = if start == String16::kNotFound {
            self.m_impl.len().saturating_sub(1)
        } else {
            cmp::min(start, self.m_impl.len().saturating_sub(1))
        };

        if start_index >= self.m_impl.len() {
            return String16::kNotFound;
        }

        for i in (0..=start_index).rev() {
            if self.m_impl[i] == c {
                return i;
            }
        }
        String16::kNotFound
    }

    pub fn swap(&mut self, other: &mut String16) {
        mem::swap(&mut self.m_impl, &mut other.m_impl);
        mem::swap(&mut self.hash_code, &mut other.hash_code);
    }

    pub fn utf8(&self) -> String {
         let mut utf8 = String::new();
        for &c in &self.m_impl {
            if c <= 0x7f {
                utf8.push(c as u8 as char);
            } else {
               
                let mut buf = [0u8; 4];
                let len = c.encode_utf8(&mut buf).len();
                utf8.push_str(str::from_utf8(&buf[..len]).unwrap());
            }
        }
        utf8
    }

    pub fn from_utf8(string_start: &str, length: usize) -> Self {
        String16::from_str(&string_start[..length])
    }

    pub fn from_utf16le(string_start: &[UChar], length: usize) -> Self {
       let mut utf16be = Vec::with_capacity(length);
        for i in 0..length {
            let utf16be_char = (string_start[i] << 8) | (string_start[i] >> 8 & 0x00FF);
            utf16be.push(utf16be_char);
        }
        String16::from_chars(&utf16be)
    }

     pub fn from_utf16le_u16(string_start: &[u16], length: usize) -> Self {
       let mut utf16be = Vec::with_capacity(length);
        for i in 0..length {
            let utf16be_char = (string_start[i] << 8) | (string_start[i] >> 8 & 0x00FF);
            utf16be.push(utf16be_char);
        }
        String16::from_chars(&utf16be)
    }

    pub fn hash(&self) -> usize {
        let mut hash_code = self.hash_code.get();
        if hash_code == 0 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            for &c in &self.m_impl {
                hasher.write_u16(c);
            }
            hash_code = hasher.finish() as usize;
            if hash_code == 0 {
                hash_code = 1;
            }
            self.hash_code.set(hash_code);
        }
        hash_code
    }

    pub fn concat<T: AsRef<str>>(args: &[T]) -> Self {
        let mut builder = String16Builder::new();
        for arg in args {
            builder.append(String16::from_str(arg.as_ref()));
        }
        builder.to_string()
    }
}

impl Default for String16 {
    fn default() -> Self {
        String16::new()
    }
}

impl PartialEq for String16 {
    fn eq(&self, other: &Self) -> bool {
        self.m_impl == other.m_impl
    }
}

impl PartialOrd for String16 {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.m_impl.partial_cmp(&other.m_impl)
    }
}

impl Hash for String16 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.hash());
    }
}

impl Add for String16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_impl = self.m_impl;
        new_impl.extend_from_slice(&other.m_impl);
        String16 {
            m_impl: new_impl,
            hash_code: std::cell::Cell::new(0),
        }
    }
}

impl AddAssign for String16 {
    fn add_assign(&mut self, other: Self) {
        self.m_impl.extend_from_slice(&other.m_impl);
        self.hash_code.set(0);
    }
}

impl Index<usize> for String16 {
    type Output = UChar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m_impl[index]
    }
}

impl From<String> for String16 {
    fn from(s: String) -> Self {
        String16::from_string(s)
    }
}

impl From<&str> for String16 {
    fn from(s: &str) -> Self {
        String16::from_str(s)
    }
}

impl fmt::Display for String16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.utf8())
    }
}

fn is_ascii(c: UChar) -> bool {
    !(c & !0x7F)
}

fn is_space_or_newline(c: UChar) -> bool {
    is_ascii(c) && c <= ' ' && (c == ' ' || (c <= 0xD && c >= 0x9))
}

#[derive(Default)]
pub struct String16Builder {
    m_buffer: Vec<UChar>,
}

impl String16Builder {
    pub fn new() -> Self {
        String16Builder {
            m_buffer: Vec::new(),
        }
    }

    pub fn append(&mut self, s: String16) {
        self.m_buffer.extend_from_slice(&s.m_impl);
    }

    pub fn append_char(&mut self, c: UChar) {
        self.m_buffer.push(c);
    }

    pub fn append_u8(&mut self, c: u8) {
        self.m_buffer.push(c as UChar);
    }

    pub fn append_chars(&mut self, characters: &[UChar]) {
        self.m_buffer.extend_from_slice(characters);
    }

    pub fn append_u8_slice(&mut self, characters: &[u8]) {
       for &byte in characters {
            self.m_buffer.push(byte as UChar);
        }
    }

    pub fn append_number(&mut self, number: i32) {
        let s = number.to_string();
        for c in s.chars() {
            self.m_buffer.push(c as u16);
        }
    }

    pub fn append_usize(&mut self, number: usize) {
        let s = number.to_string();
        for c in s.chars() {
            self.m_buffer.push(c as u16);
        }
    }

    pub fn append_unsigned_as_hex(&mut self, number: u64) {
        let s = format!("{:016x}", number);
        for c in s.chars() {
            self.m_buffer.push(c as u16);
        }
    }

    pub fn append_unsigned_as_hex_u32(&mut self, number: u32) {
        let s = format!("{:08x}", number);
        for c in s.chars() {
            self.m_buffer.push(c as u16);
        }
    }

    pub fn append_unsigned_as_hex_u8(&mut self, number: u8) {
        let s = format!("{:02x}", number);
        for c in s.chars() {
            self.m_buffer.push(c as u16);
        }
    }

    pub fn to_string(&self) -> String16 {
        String16 {
            m_impl: self.m_buffer.clone(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn reserve_capacity(&mut self, capacity: usize) {
        self.m_buffer.reserve(capacity);
    }

    pub fn append_all(&mut self, args: &[String16]) {
        for arg in args {
            self.append(arg.clone());
        }
    }
}

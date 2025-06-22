// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    char,
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Add, AddAssign, Index},
    str,
};

pub type UChar = u16;

#[derive(Clone, Debug, Default, Eq, Ord)]
pub struct String16 {
    m_impl: Vec<UChar>,
    hash_code: std::cell::Cell<usize>,
}

impl String16 {
    pub const K_NOT_FOUND: usize = usize::MAX;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_utf16(characters: &[UChar]) -> Self {
        String16 {
            m_impl: characters.to_vec(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_utf16_u16(characters: &[u16]) -> Self {
        String16 {
            m_impl: characters.to_vec(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_utf16_ptr(characters: *const UChar) -> Self {
        unsafe {
            let mut len = 0;
            while *characters.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(characters, len);
            String16::from_utf16(slice)
        }
    }

    pub fn from_ascii(characters: &str) -> Self {
        String16 {
            m_impl: characters.encode_utf16().collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_ascii_sized(characters: &str, size: usize) -> Self {
        String16 {
            m_impl: characters.encode_utf16().take(size).collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_string_view(string: &str) -> Self {
        String16 {
            m_impl: string.encode_utf16().collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_basic_string(impl_str: &Vec<UChar>) -> Self {
        String16 {
            m_impl: impl_str.clone(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_basic_string_owned(impl_str: Vec<UChar>) -> Self {
        String16 {
            m_impl: impl_str,
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_integer(num: i32) -> Self {
        String16::from_ascii(&num.to_string())
    }

    pub fn from_integer_usize(num: usize) -> Self {
        String16::from_ascii(&num.to_string())
    }

    pub fn from_integer64(num: i64) -> Self {
        String16::from_ascii(&num.to_string())
    }

    pub fn from_uint64(num: u64) -> Self {
        String16::from_ascii(&num.to_string())
    }

    pub fn from_double(num: f64) -> Self {
        String16::from_ascii(&num.to_string())
    }

    pub fn from_double_precision(num: f64, precision: usize) -> Self {
        String16::from_ascii(&format!("{:.precision$}", num))
    }

    pub fn to_integer64(&self, ok: Option<&mut bool>) -> i64 {
        match self.utf8().parse::<i64>() {
            Ok(val) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = true;
                }
                val
            }
            Err(_) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = false;
                }
                0
            }
        }
    }

    pub fn to_uint64(&self, ok: Option<&mut bool>) -> u64 {
        match self.utf8().parse::<u64>() {
            Ok(val) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = true;
                }
                val
            }
            Err(_) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = false;
                }
                0
            }
        }
    }

    pub fn to_integer(&self, ok: Option<&mut bool>) -> i32 {
        match self.utf8().parse::<i32>() {
            Ok(val) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = true;
                }
                val
            }
            Err(_) => {
                if let Some(mutable_ok) = ok {
                    *mutable_ok = false;
                }
                0
            }
        }
    }

    pub fn get_trimmed_offset_and_length(&self) -> (usize, usize) {
        let mut start = 0;
        while start < self.m_impl.len() && self.m_impl[start].is_ascii_whitespace() {
            start += 1;
        }

        let mut end = self.m_impl.len();
        while end > start && self.m_impl[end - 1].is_ascii_whitespace() {
            end -= 1;
        }

        (start, end - start)
    }

    pub fn strip_white_space(&self) -> String16 {
        let (start, len) = self.get_trimmed_offset_and_length();
        self.substring(start, len)
    }

    pub fn characters16(&self) -> &[u16] {
        unsafe { mem::transmute(&self.m_impl[..]) }
    }

    pub fn length(&self) -> usize {
        self.m_impl.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m_impl.is_empty()
    }

    pub fn substring(&self, pos: usize, len: usize) -> String16 {
        String16 {
            m_impl: self.m_impl.iter().skip(pos).take(len).copied().collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn find(&self, str: &String16, start: usize) -> usize {
        if start > self.m_impl.len() {
            return String16::K_NOT_FOUND;
        }
        let needle = &str.m_impl;
        let haystack = &self.m_impl[start..];

        if needle.is_empty() {
            return start;
        }

        if needle.len() > haystack.len() {
            return String16::K_NOT_FOUND;
        }

        for i in 0..=haystack.len() - needle.len() {
            if haystack[i..i + needle.len()] == *needle {
                return start + i;
            }
        }

        String16::K_NOT_FOUND
    }

    pub fn reverse_find(&self, str: &String16, start: usize) -> usize {
        let start = std::cmp::min(start, self.m_impl.len().saturating_sub(1));

        let needle = &str.m_impl;

        if needle.is_empty() {
            return start;
        }
        if needle.len() > self.m_impl.len() {
            return String16::K_NOT_FOUND;
        }

        for i in (0..=start.saturating_sub(needle.len() - 1)).rev() {
            if self.m_impl[i..i + needle.len()] == *needle {
                return i;
            }
        }

        String16::K_NOT_FOUND
    }

    pub fn find_char(&self, c: UChar, start: usize) -> usize {
        if start >= self.m_impl.len() {
            return String16::K_NOT_FOUND;
        }
        match self.m_impl[start..].iter().position(|&x| x == c) {
            Some(index) => start + index,
            None => String16::K_NOT_FOUND,
        }
    }

    pub fn reverse_find_char(&self, c: UChar, start: usize) -> usize {
        if self.m_impl.is_empty() {
            return String16::K_NOT_FOUND;
        }
        let start = std::cmp::min(start, self.m_impl.len() - 1);
        match self.m_impl[..=start].iter().rposition(|&x| x == c) {
            Some(index) => index,
            None => String16::K_NOT_FOUND,
        }
    }

    pub fn swap(&mut self, other: &mut String16) {
        self.m_impl.swap(&mut other.m_impl);
        std::mem::swap(&mut self.hash_code, &mut other.hash_code);
    }

    pub fn utf8(&self) -> String {
        String::from_utf16_lossy(&self.m_impl)
    }

    pub fn from_utf8(string_start: &str) -> Self {
        String16 {
            m_impl: string_start.encode_utf16().collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_utf8_sized(string_start: &str, length: usize) -> Self {
        String16 {
            m_impl: string_start.encode_utf16().take(length).collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn from_utf16le(string_start: &[UChar]) -> Self {
        // Assuming native endianness is Little Endian or handling byte swap if Big Endian.
        // Since Rust's char is always UTF-32, and String is UTF-8, UTF-16 needs explicit handling.
        let mut buffer: Vec<UChar> = Vec::with_capacity(string_start.len());
        for &c in string_start {
            buffer.push(u16::from_le(c));
        }
        String16 {
            m_impl: buffer,
            hash_code: std::cell::Cell::new(0),
        }
    }
    
    pub fn from_utf16le_u16(string_start: &[u16]) -> Self {
        // Assuming native endianness is Little Endian or handling byte swap if Big Endian.
        // Since Rust's char is always UTF-32, and String is UTF-8, UTF-16 needs explicit handling.
        let mut buffer: Vec<UChar> = Vec::with_capacity(string_start.len());
        for &c in string_start {
            buffer.push(u16::from_le(c));
        }
        String16 {
            m_impl: buffer,
            hash_code: std::cell::Cell::new(0),
        }
    }

    pub fn hash(&self) -> usize {
        let current_hash = self.hash_code.get();

        if current_hash != 0 {
            return current_hash;
        }

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for &c in &self.m_impl {
            hasher.write_u16(c);
        }
        let hash_value = hasher.finish() as usize;

        let hash_to_store = if hash_value == 0 { 1 } else { hash_value };
        self.hash_code.set(hash_to_store);
        hash_to_store
    }
    
    pub fn concat<T: AsRef<str>>(args: &[T]) -> Self {
        let mut builder = String16Builder::new();
        for arg in args {
            builder.append(&String16::from_string_view(arg.as_ref()));
        }
        builder.to_string()
    }
}

impl Index<usize> for String16 {
    type Output = UChar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m_impl[index]
    }
}

impl PartialEq for String16 {
    fn eq(&self, other: &Self) -> bool {
        self.m_impl == other.m_impl
    }
}

impl PartialOrd for String16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.m_impl.partial_cmp(&other.m_impl)
    }
}

impl Add for String16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        String16 {
            m_impl: self.m_impl.iter().chain(other.m_impl.iter()).copied().collect(),
            hash_code: std::cell::Cell::new(0),
        }
    }
}

impl AddAssign for String16 {
    fn add_assign(&mut self, other: Self) {
        self.m_impl.extend_from_slice(&other.m_impl);
        self.hash_code = std::cell::Cell::new(0);
    }
}

impl From<&str> for String16 {
    fn from(s: &str) -> Self {
        String16::from_ascii(s)
    }
}

#[derive(Debug, Default)]
pub struct String16Builder {
    m_buffer: Vec<UChar>,
}

impl String16Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, string: &String16) {
        self.m_buffer.extend_from_slice(&string.m_impl);
        
    }

    pub fn append_char(&mut self, c: UChar) {
        self.m_buffer.push(c);
    }

    pub fn append_ascii(&mut self, c: char) {
        self.m_buffer.push(c as u16);
    }

    pub fn append_utf16(&mut self, chars: &[UChar]) {
        self.m_buffer.extend_from_slice(chars);
    }

    pub fn append_ascii_sized(&mut self, chars: &str, size: usize) {
        self.m_buffer.extend(chars.encode_utf16().take(size));
    }

    pub fn append_number(&mut self, num: i32) {
        let s = num.to_string();
        self.append(&String16::from_ascii(&s));
    }

    pub fn append_number_usize(&mut self, num: usize) {
        let s = num.to_string();
        self.append(&String16::from_ascii(&s));
    }

    pub fn append_unsigned_as_hex(&mut self, num: u64) {
        let s = format!("{:x}", num);
        self.append(&String16::from_ascii(&s));
    }

    pub fn append_unsigned_as_hex_u32(&mut self, num: u32) {
        let s = format!("{:x}", num);
        self.append(&String16::from_ascii(&s));
    }

    pub fn append_unsigned_as_hex_u8(&mut self, num: u8) {
        let s = format!("{:x}", num);
        self.append(&String16::from_ascii(&s));
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
}

impl Hash for String16 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.hash());
    }
}

mod std_shim {
    use super::String16;
    use std::hash::{Hash, Hasher};

    pub struct String16Hasher {
        delegate: std::collections::hash_map::DefaultHasher,
    }

    impl String16Hasher {
        #[allow(dead_code)]
        pub fn new() -> Self {
            String16Hasher {
                delegate: std::collections::hash_map::DefaultHasher::new(),
            }
        }

        #[allow(dead_code)]
        pub fn finish(&self) -> u64 {
            self.delegate.finish()
        }
    }

    impl Hasher for String16Hasher {
        fn finish(&self) -> u64 {
            self.delegate.finish()
        }

        fn write(&mut self, bytes: &[u8]) {
            self.delegate.write(bytes)
        }
    }
}
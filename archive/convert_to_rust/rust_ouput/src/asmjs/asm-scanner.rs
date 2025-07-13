// Converted from V8 C++ source files:
// Header: asm-scanner.h
// Implementation: asm-scanner.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod asm_names;
mod base;
mod flags;
mod numbers;
mod parsing;
mod strings;

use std::collections::HashMap;
use std::string::String;
use crate::asmjs::asm_names::*;
use crate::base::logging::*;
use crate::base::strings::*;
use crate::flags::flags::Flag;
use crate::numbers::conversions::*;
use crate::parsing::scanner::*;
use crate::strings::char_predicates::*;
use std::convert::TryInto;

pub struct AsmJsScanner {
    stream_: *mut Utf16CharacterStream,
    token_: i32,
    preceding_token_: i32,
    next_token_: i32,
    position_: usize,
    preceding_position_: usize,
    next_position_: usize,
    rewind_: bool,
    identifier_string_: String,
    in_local_scope_: bool,
    local_names_: HashMap<String, i32>,
    global_names_: HashMap<String, i32>,
    property_names_: HashMap<String, i32>,
    global_count_: i32,
    double_value_: f64,
    unsigned_value_: u32,
    preceded_by_newline_: bool,
}

impl AsmJsScanner {
    pub fn new(stream: *mut Utf16CharacterStream) -> Self {
        let mut scanner = AsmJsScanner {
            stream_: stream,
            token_: kUninitialized,
            preceding_token_: kUninitialized,
            next_token_: kUninitialized,
            position_: 0,
            preceding_position_: 0,
            next_position_: 0,
            rewind_: false,
            identifier_string_: String::new(),
            in_local_scope_: false,
            local_names_: HashMap::new(),
            global_names_: HashMap::new(),
            property_names_: HashMap::new(),
            global_count_: 0,
            double_value_: 0.0,
            unsigned_value_: 0,
            preceded_by_newline_: false,
        };

        {
            let mut insert_property = |name: &str, value: i32| {
                scanner.property_names_.insert(name.to_string(), value);
            };
            STDLIB_MATH_FUNCTION_LIST!{V(insert_property)}
            STDLIB_ARRAY_TYPE_LIST!{V(insert_property)}
            STDLIB_MATH_VALUE_LIST!{V(insert_property)}
            STDLIB_OTHER_LIST!{V(insert_property)}
        }
        {
            let mut insert_global = |name: &str, value: i32| {
                scanner.global_names_.insert(name.to_string(), value);
            };
            KEYWORD_NAME_LIST!{V(insert_global)}
        }

        scanner.Next();
        scanner
    }

    pub fn Token(&self) -> i32 {
        self.token_
    }

    pub fn Position(&self) -> usize {
        self.position_
    }

    pub fn Next(&mut self) {
        if self.rewind_ {
            self.preceding_token_ = self.token_;
            self.preceding_position_ = self.position_;
            self.token_ = self.next_token_;
            self.position_ = self.next_position_;
            self.next_token_ = kUninitialized;
            self.next_position_ = 0;
            self.rewind_ = false;
            return;
        }

        if self.token_ == kEndOfInput || self.token_ == kParseError {
            return;
        }

        self.preceded_by_newline_ = false;
        self.preceding_token_ = self.token_;
        self.preceding_position_ = self.position_;

        loop {
            self.position_ = unsafe { (*self.stream_).pos() };
            let ch = unsafe { (*self.stream_).Advance() };
            match ch {
                ' ' | '\t' | '\r' => {
                    // Ignore whitespace.
                }

                '\n' => {
                    // Track when we've passed a newline for optional semicolon support,
                    // but keep scanning.
                    self.preceded_by_newline_ = true;
                }

                kEndOfInputU => {
                    self.token_ = kEndOfInput;
                    return;
                }

                '\'' | '"' => {
                    self.ConsumeString(ch);
                    return;
                }

                '/' => {
                    let ch = unsafe { (*self.stream_).Advance() };
                    if ch == '/' {
                        self.ConsumeCPPComment();
                    } else if ch == '*' {
                        if !self.ConsumeCComment() {
                            self.token_ = kParseError;
                            return;
                        }
                    } else {
                        unsafe { (*self.stream_).Back() };
                        self.token_ = '/' as i32;
                        return;
                    }
                    // Breaks out of switch, but loops again (i.e. the case when we parsed
                    // a comment, but need to continue to look for the next token).
                }

                '<' | '>' | '=' | '!' => {
                    self.ConsumeCompareOrShift(ch);
                    return;
                }
                _ => {
                    match ch.try_into() {
                        Ok(c) => {
                        match c {
                            '(' => self.token_ = '(' as i32,
                            ')' => self.token_ = ')' as i32,
                            '{' => self.token_ = '{' as i32,
                            '}' => self.token_ = '}' as i32,
                            '[' => self.token_ = '[' as i32,
                            ']' => self.token_ = ']' as i32,
                            '.' => self.token_ = '.' as i32,
                            ';' => self.token_ = ';' as i32,
                            ',' => self.token_ = ',' as i32,
                            '?' => self.token_ = '?' as i32,
                            ':' => self.token_ = ':' as i32,
                            '+' => self.token_ = '+' as i32,
                            '-' => self.token_ = '-' as i32,
                            '*' => self.token_ = '*' as i32,
                            '%' => self.token_ = '%' as i32,
                            '|' => self.token_ = '|' as i32,
                            '^' => self.token_ = '^' as i32,
                            '&' => self.token_ = '&' as i32,
                            '~' => self.token_ = '~' as i32,
                            _ => {
                                if self.IsIdentifierStart(ch) {
                                    self.ConsumeIdentifier(ch);
                                } else if self.IsNumberStart(ch) {
                                    self.ConsumeNumber(ch);
                                } else {
                                    self.token_ = kParseError;
                                }
                                return;
                            }
                        }
                        },
                        Err(_) => {
                            self.token_ = kParseError;
                            return;
                        }
                    };
                     return;
                }
            }
        }
    }

    pub fn Rewind(&mut self) {
        assert_ne!(kUninitialized, self.preceding_token_);
        assert!(!self.rewind_);
        self.next_token_ = self.token_;
        self.next_position_ = self.position_;
        self.token_ = self.preceding_token_;
        self.position_ = self.preceding_position_;
        self.preceding_token_ = kUninitialized;
        self.preceding_position_ = 0;
        self.rewind_ = true;
        self.identifier_string_.clear();
    }

    pub fn GetIdentifierString(&self) -> &String {
        assert!(!self.rewind_);
        &self.identifier_string_
    }

    pub fn IsPrecededByNewline(&self) -> bool {
        assert!(!self.rewind_);
        self.preceded_by_newline_
    }

    pub fn ResetLocals(&mut self) {
        self.local_names_.clear();
    }

    pub fn Seek(&mut self, pos: usize) {
        unsafe { (*self.stream_).Seek(pos) };
        self.preceding_token_ = kUninitialized;
        self.token_ = kUninitialized;
        self.next_token_ = kUninitialized;
        self.preceding_position_ = 0;
        self.position_ = 0;
        self.next_position_ = 0;
        self.rewind_ = false;
        self.Next();
    }

    pub fn EnterLocalScope(&mut self) {
        self.in_local_scope_ = true;
    }

    pub fn EnterGlobalScope(&mut self) {
        self.in_local_scope_ = false;
    }

    pub fn IsLocal(&self) -> bool {
        Self::IsLocal(self.Token())
    }

    pub fn IsGlobal(&self) -> bool {
        Self::IsGlobal(self.Token())
    }

    pub fn IsLocalStatic(token: i32) -> bool {
        Self::IsLocal(token)
    }

    pub fn IsGlobalStatic(token: i32) -> bool {
        Self::IsGlobal(token)
    }

    pub fn LocalIndex(token: i32) -> usize {
        assert!(Self::IsLocal(token));
        (-(token - kLocalsStart)) as usize
    }

    pub fn GlobalIndex(token: i32) -> usize {
        assert!(Self::IsGlobal(token));
        (token - kGlobalsStart) as usize
    }

    pub fn IsUnsigned(&self) -> bool {
        self.Token() == kUnsigned
    }

    pub fn AsUnsigned(&self) -> u32 {
        assert!(self.IsUnsigned());
        self.unsigned_value_
    }

    pub fn IsDouble(&self) -> bool {
        self.Token() == kDouble
    }

    pub fn AsDouble(&self) -> f64 {
        assert!(self.IsDouble());
        self.double_value_
    }

    fn ConsumeIdentifier(&mut self, ch: u32) {
        self.identifier_string_.clear();
        let mut current_ch = ch;
        while self.IsIdentifierPart(current_ch) {
            self.identifier_string_.push(current_ch as u8 as char);
            current_ch = unsafe { (*self.stream_).Advance() };
        }
        unsafe { (*self.stream_).Back() };

        if self.preceding_token_ == '.' as i32 {
            if let Some(&token) = self.property_names_.get(&self.identifier_string_) {
                self.token_ = token;
                return;
            }
        } else {
            if let Some(&token) = self.local_names_.get(&self.identifier_string_) {
                self.token_ = token;
                return;
            }
            if !self.in_local_scope_ {
                if let Some(&token) = self.global_names_.get(&self.identifier_string_) {
                    self.token_ = token;
                    return;
                }
            }
        }

        if self.preceding_token_ == '.' as i32 {
            assert!(self.global_count_ < kMaxIdentifierCount as i32);
            self.token_ = kGlobalsStart + self.global_count_;
            self.global_count_ += 1;
            self.property_names_.insert(self.identifier_string_.clone(), self.token_);
        } else if self.in_local_scope_ {
            assert!((self.local_names_.len() as i32) < kMaxIdentifierCount as i32);
            self.token_ = kLocalsStart - self.local_names_.len() as i32;
            self.local_names_.insert(self.identifier_string_.clone(), self.token_);
        } else {
            assert!(self.global_count_ < kMaxIdentifierCount as i32);
            self.token_ = kGlobalsStart + self.global_count_;
            self.global_count_ += 1;
            self.global_names_.insert(self.identifier_string_.clone(), self.token_);
        }
    }

    fn ConsumeNumber(&mut self, ch: u32) {
        let mut number = String::new();
        number.push(ch as u8 as char);
        let mut has_dot = ch == '.' as u32;
        let mut has_prefix = false;
        loop {
            let ch = unsafe { (*self.stream_).Advance() };
            if (ch >= '0' as u32 && ch <= '9' as u32) || (ch >= 'a' as u32 && ch <= 'f' as u32) ||
                (ch >= 'A' as u32 && ch <= 'F' as u32) || ch == '.' as u32 || ch == 'b' as u32 ||
                ch == 'o' as u32 || ch == 'x' as u32 ||
                ((ch == '-' as u32 || ch == '+' as u32) && !has_prefix &&
                    (number.ends_with('e') || number.ends_with('E'))) {
                if ch == '.' as u32 {
                    has_dot = true;
                }
                if ch == 'b' as u32 || ch == 'o' as u32 || ch == 'x' as u32 {
                    has_prefix = true;
                }
                number.push(ch as u8 as char);
            } else {
                unsafe { (*self.stream_).Back() };
                break;
            }
        }

        if number.len() == 1 && number.as_bytes()[0] == '0' as u8 {
            self.unsigned_value_ = 0;
            self.token_ = kUnsigned;
            return;
        }

        if number.len() == 1 && number.as_bytes()[0] == '.' as u8 {
            self.token_ = '.' as i32;
            return;
        }

        if has_prefix && number.starts_with('0') {
            if number.len() <= 2 {
                self.token_ = kParseError;
                return;
            }
            match number.as_bytes()[1] as char {
                'b' => {
                    self.double_value_ = binary_string_to_double(number.as_str());
                }
                'o' => {
                    self.double_value_ = octal_string_to_double(number.as_str());
                }
                'x' => {
                    self.double_value_ = hex_string_to_double(number.as_str());
                }
                _ => {
                    self.token_ = kParseError;
                }
            }
        } else if number.starts_with('0') && !has_prefix && Self::IsValidImplicitOctalString(&number) {
            self.double_value_ = implicit_octal_string_to_double(number.as_str());
        } else {
            self.double_value_ = string_to_double(number.as_str());
        }

        if self.double_value_.is_nan() {
            if number.starts_with('.') {
                for _ in 1..number.len() {
                    unsafe { (*self.stream_).Back() };
                }
                self.token_ = '.' as i32;
                return;
            }
            self.token_ = kParseError;
            return;
        }

        if has_dot || self.double_value_.trunc() != self.double_value_ {
            self.token_ = kDouble;
        } else {
            if self.double_value_ > kMaxUInt32 as f64 {
                self.token_ = kParseError;
                return;
            }
            self.unsigned_value_ = self.double_value_ as u32;
            self.token_ = kUnsigned;
        }
    }

    fn ConsumeCComment(&mut self) -> bool {
        loop {
            let ch = unsafe { (*self.stream_).Advance() };
            let mut current_ch = ch;
            while current_ch == '*' as u32 {
                current_ch = unsafe { (*self.stream_).Advance() };
                if current_ch == '/' as u32 {
                    return true;
                }
            }
            if ch == '\n' as u32 {
                self.preceded_by_newline_ = true;
            }
            if ch == kEndOfInputU {
                return false;
            }
        }
    }

    fn ConsumeCPPComment(&mut self) {
        loop {
            let ch = unsafe { (*self.stream_).Advance() };
            if ch == '\n' as u32 {
                self.preceded_by_newline_ = true;
                return;
            }
            if ch == kEndOfInputU {
                return;
            }
        }
    }

    fn ConsumeString(&mut self, quote: u32) {
        let expected = "use asm";
        for &byte in expected.as_bytes() {
            if unsafe { (*self.stream_).Advance() } != byte as u32 {
                self.token_ = kParseError;
                return;
            }
        }
        if unsafe { (*self.stream_).Advance() } != quote {
            self.token_ = kParseError;
            return;
        }
        self.token_ = kToken_UseAsm;
    }

    fn ConsumeCompareOrShift(&mut self, ch: u32) {
        let next_ch = unsafe { (*self.stream_).Advance() };
        if next_ch == '=' as u32 {
            match ch {
                '<' as u32 => self.token_ = kToken_LE,
                '>' as u32 => self.token_ = kToken_GE,
                '=' as u32 => self.token_ = kToken_EQ,
                '!' as u32 => self.token_ = kToken_NE,
                _ => unreachable!(),
            }
        } else if ch == '<' as u32 && next_ch == '<' as u32 {
            self.token_ = kToken_SHL;
        } else if ch == '>' as u32 && next_ch == '>' as u32 {
            let next_next_ch = unsafe { (*self.stream_).Advance() };
            if next_next_ch == '>' as u32 {
                self.token_ = kToken_SHR;
            } else {
                self.token_ = kToken_SAR;
                unsafe { (*self.stream_).Back() };
            }
        } else {
            unsafe { (*self.stream_).Back() };
            self.token_ = ch as i32;
        }
    }

    fn IsIdentifierStart(&self, ch: u32) -> bool {
        let ch_lower = ascii_alpha_to_lower(ch);
        (ch_lower >= 'a' as u32 && ch_lower <= 'z' as u32) || ch == '_' as u32 || ch == '$' as u32
    }

    fn IsIdentifierPart(&self, ch: u32) -> bool {
        is_ascii_identifier(ch)
    }

    fn IsNumberStart(&self, ch: u32) -> bool {
        ch == '.' as u32 || is_decimal_digit(ch)
    }

    fn IsValidImplicitOctalString(number: &str) -> bool {
        if number.len() > 0 && number.as_bytes()[0] == '0' as u8 {
            return number.as_bytes().iter().skip(1).all(|&c| is_octal_digit(c as u32));
        }
        false
    }

    pub fn IsLocal(token: i32) -> bool {
        token <= kLocalsStart
    }

    pub fn IsGlobal(token: i32) -> bool {
        token >= kGlobalsStart
    }

}

extern "C" {
    fn binary_string_to_double(str: &str) -> f64;
    fn octal_string_to_double(str: &str) -> f64;
    fn hex_string_to_double(str: &str) -> f64;
    fn implicit_octal_string_to_double(str: &str) -> f64;
    fn string_to_double(str: &str) -> f64;
}

#[repr(C)]
pub struct Utf16CharacterStream {
    // Omitted fields
}

impl Utf16CharacterStream {
    pub unsafe fn Advance(&mut self) -> u32 {
        1 as u32
    }

    pub unsafe fn Back(&mut self) {}

    pub unsafe fn Seek(&mut self, _pos: usize) {}

    pub unsafe fn pos(&self) -> usize {
        0
    }
}

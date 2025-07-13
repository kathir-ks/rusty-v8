// Converted from V8 C++ source files:
// Header: js-segments-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::rc::Rc;
use crate::objects::js_segments::*;
use crate::objects::objects::*;
use crate::objects::object_macros::*;

pub struct icu {}

#[derive(Debug, Clone)]
pub struct Managed<T> {
    value: Rc<T>,
}

impl<T> Managed<T> {
    pub fn new(value: T) -> Self {
        Managed { value: Rc::new(value) }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

pub struct JSSegments {
    icu_break_iterator: Tagged<Managed<icu::BreakIterator>>,
    raw_string: Tagged<String>,
    unicode_string: Tagged<Managed<icu::UnicodeString>>,
    flags: i32,
}

impl JSSegments {
    pub fn icu_break_iterator(&self) -> &Tagged<Managed<icu::BreakIterator>> {
        &self.icu_break_iterator
    }

    pub fn set_icu_break_iterator(&mut self, value: Tagged<Managed<icu::BreakIterator>>) {
        self.icu_break_iterator = value;
    }

    pub fn raw_string(&self) -> &Tagged<String> {
        &self.raw_string
    }

    pub fn set_raw_string(&mut self, value: Tagged<String>) {
        self.raw_string = value;
    }

    pub fn unicode_string(&self) -> &Tagged<Managed<icu::UnicodeString>> {
        &self.unicode_string
    }

    pub fn set_unicode_string(&mut self, value: Tagged<Managed<icu::UnicodeString>>) {
        self.unicode_string = value;
    }
    pub fn flags(&self) -> i32 {
        self.flags
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = flags;
    }

    pub fn set_granularity(&mut self, granularity: JSSegmenter::Granularity) {
        let hints = self.flags();
        let hints = GranularityBits::update(hints, granularity as i32);
        self.set_flags(hints);
    }

    pub fn granularity(&self) -> JSSegmenter::Granularity {
        let flags = self.flags();
        let decoded = GranularityBits::decode(flags) as u32;
        match decoded {
            0 => JSSegmenter::Granularity::Grapheme,
            1 => JSSegmenter::Granularity::Word,
            2 => JSSegmenter::Granularity::Sentence,
            3 => JSSegmenter::Granularity::Line,
            _ => JSSegmenter::Granularity::Grapheme, 
        }
    }
}

mod GranularityBits {
    pub fn is_valid(granularity: JSSegmenter::Granularity) -> bool {
        match granularity {
            JSSegmenter::Granularity::Grapheme |
            JSSegmenter::Granularity::Word |
            JSSegmenter::Granularity::Sentence |
            JSSegmenter::Granularity::Line => true,
        _ => false,
        }
    }

    pub fn update(hints: i32, granularity: i32) -> i32 {
        let mask: i32 = 0b11;
        let shifted_granularity = granularity & mask;
        (hints & !mask) | shifted_granularity
    }

    pub fn decode(flags: i32) -> i32 {
       flags & 0b11
    }
}

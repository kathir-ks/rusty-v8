// Converted from V8 C++ source files:
// Header: js-segment-iterator-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
struct Managed<T> {
    ptr: *mut T,
}
impl<T> Managed<T> {
    fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}
use std::ops::Deref;
use std::ops::DerefMut;

impl<T> Deref for Managed<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Managed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

struct JSSegmenter {}

impl JSSegmenter {
    pub enum Granularity {
        Grapheme,
        Word,
        Sentence,
    }
}
struct String {}
struct JSSegmentIterator {
    icu_break_iterator: Tagged<Managed<icu::BreakIterator>>,
    raw_string: Tagged<String>,
    unicode_string: Tagged<Managed<icu::UnicodeString>>,
    flags: i32,
}
struct JSSegmentDataObject {}
struct JSSegmentDataObjectWithIsWordLike {}
const kIcuBreakIteratorOffset: usize = 0;
const kRawStringOffset: usize = 8;
const kUnicodeStringOffset: usize = 16;
struct icu {}
impl icu {
    pub struct BreakIterator {}
    pub struct UnicodeString {}
}
struct Tagged<T> {
    value: T,
}
trait GranularityBitsTrait {
    fn is_valid(granularity: JSSegmenter::Granularity) -> bool;
    fn update(hints: i32, granularity: JSSegmenter::Granularity) -> i32;
    fn decode(flags: i32) -> JSSegmenter::Granularity;
}
struct GranularityBits {}
impl GranularityBitsTrait for GranularityBits {
    fn is_valid(_granularity: JSSegmenter::Granularity) -> bool {
        true
    }
    fn update(hints: i32, _granularity: JSSegmenter::Granularity) -> i32 {
        hints + 1
    }
    fn decode(_flags: i32) -> JSSegmenter::Granularity {
        JSSegmenter::Granularity::Grapheme
    }
}
macro_rules! ACCESSORS {
    ($struct_name:ident, $field_name:ident, $field_type:ty, $offset:expr) => {
        impl $struct_name {
            #[allow(clippy::missing_safety_doc)]
            pub fn $field_name(&self) -> &$field_type {
                unsafe { &*(&self as *const Self).add($offset) }
            }
            #[allow(clippy::missing_safety_doc)]
            pub fn $field_name##_mut(&mut self) -> &mut $field_type {
                unsafe { &mut *(&mut self as *mut Self).add($offset) }
            }
        }
    };
}
macro_rules! TQ_OBJECT_CONSTRUCTORS_IMPL {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    icu_break_iterator: Tagged {
                        value: Managed::new(),
                    },
                    raw_string: Tagged { value: String {} },
                    unicode_string: Tagged {
                        value: Managed::new(),
                    },
                    flags: 0,
                }
            }
        }
    };
}
impl JSSegmentIterator {
    pub fn set_granularity(&mut self, granularity: JSSegmenter::Granularity) {
        if !GranularityBits::is_valid(granularity) {
            panic!("Invalid granularity");
        }
        let hints = self.flags();
        let hints = GranularityBits::update(hints, granularity);
        self.set_flags(hints);
    }
    pub fn granularity(&self) -> JSSegmenter::Granularity {
        GranularityBits::decode(self.flags())
    }
    fn flags(&self) -> i32 {
        self.flags
    }
    fn set_flags(&mut self, flags: i32) {
        self.flags = flags;
    }
}
ACCESSORS!(
    JSSegmentIterator,
    icu_break_iterator,
    Tagged<Managed<icu::BreakIterator>>,
    kIcuBreakIteratorOffset
);
ACCESSORS!(
    JSSegmentIterator,
    raw_string,
    Tagged<String>,
    kRawStringOffset
);
ACCESSORS!(
    JSSegmentIterator,
    unicode_string,
    Tagged<Managed<icu::UnicodeString>>,
    kUnicodeStringOffset
);
TQ_OBJECT_CONSTRUCTORS_IMPL!(JSSegmentIterator);
TQ_OBJECT_CONSTRUCTORS_IMPL!(JSSegmentDataObject);
TQ_OBJECT_CONSTRUCTORS_IMPL!(JSSegmentDataObjectWithIsWordLike);

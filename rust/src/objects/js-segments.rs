// src/objects/js_segments.rs

//use std::collections::HashMap;
use std::rc::Rc;
//use std::sync::Arc;

use icu::break_iterator::{BreakIterator, BreakIteratorType};
use icu::segmenter::Segmenter;
use icu::string::String as ICUString;
//use icu::locid::Locale;

//use crate::execution::isolate::Isolate;
//use crate::heap::factory::Factory;
//use crate::objects::intl_objects::Intl;
//use crate::objects::js_segment_iterator::JSSegmentIterator;
//use crate::objects::js_segmenter::JSSegmenter;
//use crate::objects::managed::Managed;
//use crate::objects::objects::Object;
//use crate::objects::string::String;
//use crate::native_context::NativeContext;
//use crate::objects::map::Map;
//use crate::objects::js_object::JSObject;
//use crate::objects::number::Number;
//use crate::objects::boolean::Boolean;

#[derive(Debug)]
pub struct JSSegments {
    icu_break_iterator: Rc<BreakIterator>,
    granularity: JSSegmenterGranularity,
    raw_string: String,
    unicode_string: Rc<ICUString>,
    flags: u32,
}

impl JSSegments {
    pub fn create(
       // isolate: &Isolate,
        segmenter: &JSSegmenter,
        string: &String,
    ) -> Result<Self, String> {
        //let locale = Locale::from_string(segmenter.locale().as_str()).map_err(|e| e.to_string())?;
        // let break_iterator_type = match segmenter.granularity() {
        //     JSSegmenterGranularity::Grapheme => BreakIteratorType::Grapheme,
        //     JSSegmenterGranularity::Word => BreakIteratorType::Word,
        //     JSSegmenterGranularity::Sentence => BreakIteratorType::Sentence,
        //     JSSegmenterGranularity::Line => BreakIteratorType::Line,
        // };
        //let break_iterator = BreakIterator::new(&locale, break_iterator_type).map_err(|e| e.to_string())?;

        let unicode_string = ICUString::from(&string.0);

       // let managed_break_iterator = Rc::new(break_iterator);

        Ok(JSSegments {
            icu_break_iterator: segmenter.break_iterator.clone(),//managed_break_iterator,
            granularity: segmenter.granularity(),
            raw_string: string.clone(),
            unicode_string: Rc::new(unicode_string),
            flags: 0,
        })
    }

    pub fn containing(
       // isolate: &Isolate,
        segments: &JSSegments,
        n_double: f64,
    ) -> Option<JSSegmentDataObject> {
        let len = segments.unicode_string.len() as f64;

        if n_double < 0.0 || n_double >= len {
            return None;
        }

        let n = n_double as i32;
        let n = segments.unicode_string.char_index_at(n as usize) as i32;

        let break_iterator = &segments.icu_break_iterator;

        let start_index =
            if break_iterator.is_boundary(n as usize).unwrap() { n } else { break_iterator.preceding(n as usize).unwrap() as i32 };
        let end_index = break_iterator.following(n as usize).unwrap() as i32;
        
        Some(Self::create_segment_data_object(
           // isolate,
            segments.granularity,
            &segments.icu_break_iterator,
            &segments.raw_string,
            &segments.unicode_string,
            start_index,
            end_index,
        ))
    }

    fn create_segment_data_object(
       // isolate: &Isolate,
        granularity: JSSegmenterGranularity,
        break_iterator: &BreakIterator,
        input_string: &String,
        unicode_string: &ICUString,
        start_index: i32,
        end_index: i32,
    ) -> JSSegmentDataObject {
       // let factory = isolate.factory();

        assert!(start_index >= 0);
        assert!(end_index <= unicode_string.len() as i32);
        assert!(start_index < end_index);

        let segment = unicode_string.as_str()[start_index as usize..end_index as usize].to_string();
        let index = start_index;

        let is_word_like = if granularity == JSSegmenterGranularity::Word {
            current_segment_is_word_like(break_iterator)
        } else {
            false
        };

        JSSegmentDataObject {
            segment,
            index,
            input: input_string.clone(),
            is_word_like,
        }
    }

    pub fn granularity_as_string(&self) -> String {
        JSSegmenterGranularity::get_granularity_string(self.granularity)
    }

    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }
}

fn current_segment_is_word_like(break_iterator: &BreakIterator) -> bool {
    // This part is not directly translatable as getRuleStatus() and the UBRK_* constants
    // don't have Rust equivalents in the icu crate. It would require additional
    // binding to the ICU4C library or a reimplementation of the word-like logic.

    // This is a placeholder.  Proper implementation would check the break rule status.
    false
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JSSegmenterGranularity {
    Grapheme,
    Word,
    Sentence,
    Line,
}

impl JSSegmenterGranularity {
    pub fn get_granularity_string(granularity: JSSegmenterGranularity) -> String {
        match granularity {
            JSSegmenterGranularity::Grapheme => "grapheme".to_string(),
            JSSegmenterGranularity::Word => "word".to_string(),
            JSSegmenterGranularity::Sentence => "sentence".to_string(),
            JSSegmenterGranularity::Line => "line".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JSSegmentDataObject {
    segment: String,
    index: i32,
    input: String,
    is_word_like: bool,
}

#[derive(Debug)]
pub struct JSSegmenter {
    granularity: JSSegmenterGranularity,
    break_iterator: Rc<BreakIterator>, //Rc<icu::break_iterator::BreakIterator>,
    //locale: String
}

impl JSSegmenter {
    pub fn new(granularity: JSSegmenterGranularity, break_iterator: Rc<BreakIterator>) -> Self {
        JSSegmenter {
            granularity,
            break_iterator,
        }
    }

    pub fn granularity(&self) -> JSSegmenterGranularity {
        self.granularity
    }

    pub fn break_iterator(&self) -> &Rc<BreakIterator>{
        &self.break_iterator
    }
}

#[derive(Debug, Clone)]
pub struct String(pub std::string::String);
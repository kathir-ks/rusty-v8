// Converted from V8 C++ source files:
// Header: template-objects-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::v8::internal::Object;
use crate::v8::internal::Tagged;

pub mod template_objects {
    use crate::v8::internal::Object;
    use crate::v8::internal::Tagged;
    use crate::objects::js_array::JSArray;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct TemplateObjectDescription {
        raw_strings: Rc<JSArray>,
        cooked_strings: Rc<JSArray>,
    }

    impl TemplateObjectDescription {
        pub fn new(raw_strings: Rc<JSArray>, cooked_strings: Rc<JSArray>) -> Self {
            TemplateObjectDescription {
                raw_strings,
                cooked_strings,
            }
        }

        pub fn raw_strings(&self) -> Rc<JSArray> {
            Rc::clone(&self.raw_strings)
        }

        pub fn cooked_strings(&self) -> Rc<JSArray> {
            Rc::clone(&self.cooked_strings)
        }
    }
}
pub mod js_array {
    #[derive(Debug)]
    pub struct JSArray {
        length: usize,
        elements: Vec<String>,
    }

    impl JSArray {
        pub fn new(length: usize, elements: Vec<String>) -> Self {
            JSArray { length, elements }
        }

        pub fn length(&self) -> usize {
            self.length
        }

        pub fn get_element(&self, index: usize) -> Option<&String> {
            self.elements.get(index)
        }
    }
}

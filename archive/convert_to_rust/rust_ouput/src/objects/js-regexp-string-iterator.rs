// Converted from V8 C++ source files:
// Header: js-regexp-string-iterator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_regexp_string_iterator {
    use crate::objects::js_objects::JSObject;
    use crate::objects::object_macros::*;
    use crate::V8;

    pub struct JSRegExpStringIterator {
        pub js_object: JSObject,
        pub done: bool,
        pub global: bool,
        pub unicode: bool,
    }

    impl JSRegExpStringIterator {
        pub fn done(&self) -> bool {
            self.done
        }

        pub fn set_done(&mut self, value: bool) {
            self.done = value;
        }

        pub fn global(&self) -> bool {
            self.global
        }

        pub fn set_global(&mut self, value: bool) {
            self.global = value;
        }

        pub fn unicode(&self) -> bool {
            self.unicode
        }

        pub fn set_unicode(&mut self, value: bool) {
            self.unicode = value;
        }
    }

    impl JSRegExpStringIterator {
        pub fn new() -> Self {
            JSRegExpStringIterator {
                js_object: JSObject {},
                done: false,
                global: false,
                unicode: false,
            }
        }
    }
}

// Converted from V8 C++ source files:
// Header: js-raw-json-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_raw_json_inl {
    use crate::objects::js_raw_json::JSRawJson;
    use crate::objects::objects_inl::*;
    use crate::V8;

    pub trait TqObjectConstructorsImpl<T> {
        fn tq_object_constructors_impl() -> Self;
    }

    impl JSRawJson {
        pub fn has_initial_layout(&self, isolate: *mut Isolate) -> bool {
            unsafe {
                self.map() == (*isolate).js_raw_json_map()
            }
        }
    }
}

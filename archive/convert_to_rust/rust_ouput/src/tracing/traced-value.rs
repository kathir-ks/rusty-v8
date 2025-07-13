// Converted from V8 C++ source files:
// Header: traced-value.h
// Implementation: traced-value.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tracing {
    use std::string::String;
    use std::vec::Vec;
    use std::fmt;
    use std::fmt::Write;

    pub struct V8_EXPORT_PRIVATE {}

    pub trait ConvertableToTraceFormat {
        fn append_as_trace_format(&self, out: &mut String);
    }

    #[cfg(feature = "V8_USE_PERFETTO")]
    pub trait DebugAnnotation {
        fn add(&self, annotation: &mut perfetto::protos::pbzero::DebugAnnotation);
    }

    pub struct TracedValue {
        data_: String,
        first_item_: bool,
        #[cfg(debug_assertions)]
        nesting_stack_: Vec<bool>,
    }

    impl fmt::Debug for TracedValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TracedValue")
                .field("data_", &self.data_)
                .field("first_item_", &self.first_item_)
                .finish()
        }
    }

    const K_STACK_TYPE_DICT: bool = false;
    const K_STACK_TYPE_ARRAY: bool = true;

    impl TracedValue {
        pub fn create() -> Box<TracedValue> {
            let mut traced_value = TracedValue {
                data_: String::new(),
                first_item_: true,
                #[cfg(debug_assertions)]
                nesting_stack_: Vec::new(),
            };

            #[cfg(debug_assertions)]
            traced_value.nesting_stack_.push(K_STACK_TYPE_DICT);

            Box::new(traced_value)
        }

        fn write_comma(&mut self) {
            if self.first_item_ {
                self.first_item_ = false;
            } else {
                self.data_.push(',');
            }
        }

        fn write_name(&mut self, name: &str) {
            self.write_comma();
            self.data_.push('"');
            self.data_.push_str(name);
            self.data_.push_str("\":");
        }

        pub fn set_integer(&mut self, name: &str, value: i32) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            self.data_.push_str(&value.to_string());
        }

        pub fn set_unsigned_integer(&mut self, name: &str, value: u64) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            self.data_.push_str(&value.to_string());
        }

        pub fn set_double(&mut self, name: &str, value: f64) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            let mut buffer = ryu::Buffer::new();
            let formatted = buffer.format(value);
            self.data_.push_str(formatted);
        }

        pub fn set_boolean(&mut self, name: &str, value: bool) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            self.data_.push_str(if value { "true" } else { "false" });
        }

        pub fn set_string(&mut self, name: &str, value: &str) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            escape_and_append_string(value, &mut self.data_);
        }

        pub fn set_value(&mut self, name: &str, value: &TracedValue) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            self.write_name(name);
            let mut tmp = String::new();
            value.append_as_trace_format(&mut tmp);
            self.data_.push_str(&tmp);
        }

        pub fn begin_dictionary(&mut self, name: &str) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            #[cfg(debug_assertions)]
            self.nesting_stack_.push(K_STACK_TYPE_DICT);
            self.write_name(name);
            self.data_.push('{');
            self.first_item_ = true;
        }

        pub fn begin_array(&mut self, name: &str) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            #[cfg(debug_assertions)]
            self.nesting_stack_.push(K_STACK_TYPE_ARRAY);
            self.write_name(name);
            self.data_.push('[');
            self.first_item_ = true;
        }

        pub fn append_integer(&mut self, value: i32) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            self.write_comma();
            self.data_.push_str(&value.to_string());
        }

        pub fn append_double(&mut self, value: f64) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            self.write_comma();
            let mut buffer = ryu::Buffer::new();
            let formatted = buffer.format(value);
            self.data_.push_str(formatted);
        }

        pub fn append_boolean(&mut self, value: bool) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            self.write_comma();
            self.data_.push_str(if value { "true" } else { "false" });
        }

        pub fn append_string(&mut self, value: &str) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            self.write_comma();
            escape_and_append_string(value, &mut self.data_);
        }

        pub fn begin_dictionary_no_name(&mut self) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            #[cfg(debug_assertions)]
            self.nesting_stack_.push(K_STACK_TYPE_DICT);
            self.write_comma();
            self.data_.push('{');
            self.first_item_ = true;
        }

        pub fn begin_array_no_name(&mut self) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            #[cfg(debug_assertions)]
            self.nesting_stack_.push(K_STACK_TYPE_ARRAY);
            self.write_comma();
            self.data_.push('[');
            self.first_item_ = true;
        }

        pub fn end_dictionary(&mut self) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            #[cfg(debug_assertions)]
            self.nesting_stack_.pop();
            self.data_.push('}');
            self.first_item_ = false;
        }

        pub fn end_array(&mut self) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_ARRAY));
            #[cfg(debug_assertions)]
            self.nesting_stack_.pop();
            self.data_.push(']');
            self.first_item_ = false;
        }
    }

    impl Drop for TracedValue {
        fn drop(&mut self) {
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.last().cloned(), Some(K_STACK_TYPE_DICT));
            #[cfg(debug_assertions)]
            self.nesting_stack_.pop();
            #[cfg(debug_assertions)]
            assert_eq!(self.nesting_stack_.len(), 0);
        }
    }

    impl ConvertableToTraceFormat for TracedValue {
        fn append_as_trace_format(&self, out: &mut String) {
            out.push('{');
            out.push_str(&self.data_);
            out.push('}');
        }
    }

    #[cfg(feature = "V8_USE_PERFETTO")]
    impl DebugAnnotation for TracedValue {
        fn add(&self, annotation: &mut perfetto::protos::pbzero::DebugAnnotation) {
            let mut json = String::new();
            json.push('{');
            json.push_str(&self.data_);
            json.push('}');
            annotation.set_legacy_json_value(json);
        }
    }

    fn escape_and_append_string(value: &str, result: &mut String) {
        result.push('"');
        for c in value.chars() {
            match c {
                '\u{0008}' => result.push_str("\\b"),
                '\u{000C}' => result.push_str("\\f"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                _ => {
                    if c < '\x20' || c == '\x7F' {
                        let mut number_buffer = String::new();
                        write!(&mut number_buffer, "\\u{:04X}", c as u32).unwrap();
                        result.push_str(&number_buffer);
                    } else {
                        result.push(c);
                    }
                }
            }
        }
        result.push('"');
    }
} // namespace tracing

// src/tracing/traced_value.rs

use std::fmt::Write;

// Mock for base::platform::platform.h
mod platform {
    pub mod os {
        pub fn snprintf(buffer: &mut [u8], format: &str, value: u32) -> usize {
            let formatted = format!("{}", value);
            let len = std::cmp::min(buffer.len(), formatted.len());
            buffer[..len].copy_from_slice(formatted.as_bytes()[..len].as_ref());
            len
        }
    }
}

// Mock for src/numbers/conversions.h
mod internal {
    pub fn double_to_string_view(value: f64, buffer: &mut String) -> &str {
        write!(buffer, "{}", value).unwrap();
        buffer.as_str()
    }
}

// Mock for base::vector.h
mod base {
    pub struct EmbeddedVector<T, const N: usize> {
        data: [Option<T>; N],
        size: usize,
    }

    impl<T: Copy + Default, const N: usize> EmbeddedVector<T, N> {
        pub fn new() -> Self {
            EmbeddedVector {
                data: [None; N],
                size: 0,
            }
        }
    }
}

// Mock for perfetto protos
#[cfg(feature = "perfetto")]
mod perfetto {
    pub mod protos {
        pub mod pbzero {
            pub struct DebugAnnotation {
                legacy_json_value: String,
            }

            impl DebugAnnotation {
                pub fn new() -> Self {
                    DebugAnnotation {
                        legacy_json_value: String::new(),
                    }
                }
                pub fn set_legacy_json_value(&mut self, value: String) {
                    self.legacy_json_value = value;
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum StackType {
    Dict,
    Array,
}

struct TracedValue {
    data: String,
    first_item: bool,
    nesting_stack: Vec<StackType>,
}

impl TracedValue {
    pub fn create() -> Box<TracedValue> {
        let mut tv = TracedValue {
            data: String::new(),
            first_item: true,
            nesting_stack: Vec::new(),
        };
        tv.nesting_stack.push(StackType::Dict);
        Box::new(tv)
    }

    fn dcheck_current_container_is(&self, x: StackType) {
        debug_assert_eq!(Some(&x), self.nesting_stack.last());
    }

    fn dcheck_container_stack_depth_eq(&self, x: usize) {
        debug_assert_eq!(x, self.nesting_stack.len());
    }

    fn debug_push_container(&mut self, x: StackType) {
        self.nesting_stack.push(x);
    }

    fn debug_pop_container(&mut self) {
        self.nesting_stack.pop();
    }

    pub fn set_integer(&mut self, name: &str, value: i32) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        self.data.push_str(&value.to_string());
    }

    pub fn set_unsigned_integer(&mut self, name: &str, value: u64) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        self.data.push_str(&value.to_string());
    }

    pub fn set_double(&mut self, name: &str, value: f64) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        let mut buffer = String::new();
        internal::double_to_string_view(value, &mut buffer);
        self.data.push_str(&buffer);
    }

    pub fn set_boolean(&mut self, name: &str, value: bool) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        self.data.push_str(if value { "true" } else { "false" });
    }

    pub fn set_string(&mut self, name: &str, value: &str) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        self.escape_and_append_string(value);
    }

    pub fn set_value(&mut self, name: &str, value: &TracedValue) {
        self.dcheck_current_container_is(StackType::Dict);
        self.write_name(name);
        let mut tmp = String::new();
        value.append_as_trace_format(&mut tmp);
        self.data.push_str(&tmp);
    }

    pub fn begin_dictionary(&mut self, name: &str) {
        self.dcheck_current_container_is(StackType::Dict);
        self.debug_push_container(StackType::Dict);
        self.write_name(name);
        self.data.push('{');
        self.first_item = true;
    }

    pub fn begin_array(&mut self, name: &str) {
        self.dcheck_current_container_is(StackType::Dict);
        self.debug_push_container(StackType::Array);
        self.write_name(name);
        self.data.push('[');
        self.first_item = true;
    }

    pub fn append_integer(&mut self, value: i32) {
        self.dcheck_current_container_is(StackType::Array);
        self.write_comma();
        self.data.push_str(&value.to_string());
    }

    pub fn append_double(&mut self, value: f64) {
        self.dcheck_current_container_is(StackType::Array);
        self.write_comma();
        let mut buffer = String::new();
        internal::double_to_string_view(value, &mut buffer);
        self.data.push_str(&buffer);
    }

    pub fn append_boolean(&mut self, value: bool) {
        self.dcheck_current_container_is(StackType::Array);
        self.write_comma();
        self.data.push_str(if value { "true" } else { "false" });
    }

    pub fn append_string(&mut self, value: &str) {
        self.dcheck_current_container_is(StackType::Array);
        self.write_comma();
        self.escape_and_append_string(value);
    }

    pub fn begin_dictionary_no_name(&mut self) {
        self.dcheck_current_container_is(StackType::Array);
        self.debug_push_container(StackType::Dict);
        self.write_comma();
        self.data.push('{');
        self.first_item = true;
    }

    pub fn begin_array_no_name(&mut self) {
        self.dcheck_current_container_is(StackType::Array);
        self.debug_push_container(StackType::Array);
        self.write_comma();
        self.data.push('[');
        self.first_item = true;
    }

    pub fn end_dictionary(&mut self) {
        self.dcheck_current_container_is(StackType::Dict);
        self.debug_pop_container();
        self.data.push('}');
        self.first_item = false;
    }

    pub fn end_array(&mut self) {
        self.dcheck_current_container_is(StackType::Array);
        self.debug_pop_container();
        self.data.push(']');
        self.first_item = false;
    }

    fn write_comma(&mut self) {
        if self.first_item {
            self.first_item = false;
        } else {
            self.data.push(',');
        }
    }

    fn write_name(&mut self, name: &str) {
        self.write_comma();
        self.data.push('"');
        self.data.push_str(name);
        self.data.push_str("\":");
    }

    fn escape_and_append_string(&mut self, value: &str) {
        self.data.push('"');
        for c in value.chars() {
            match c {
                '\u{0008}' => self.data.push_str("\\b"),
                '\u{000c}' => self.data.push_str("\\f"),
                '\n' => self.data.push_str("\\n"),
                '\r' => self.data.push_str("\\r"),
                '\t' => self.data.push_str("\\t"),
                '"' => self.data.push_str("\\\""),
                '\\' => self.data.push_str("\\\\"),
                _ => {
                    if c < '\x20' || c == '\x7F' {
                        let mut number_buffer = [0u8; 8];
                        let len = platform::os::snprintf(&mut number_buffer, "\\u%04X", c as u32);
                        self.data.push_str(std::str::from_utf8(&number_buffer[..len]).unwrap());
                    } else {
                        self.data.push(c);
                    }
                }
            }
        }
        self.data.push('"');
    }

    fn append_as_trace_format(&self, out: &mut String) {
        out.push('{');
        out.push_str(&self.data);
        out.push('}');
    }

    #[cfg(feature = "perfetto")]
    pub fn add(&self, annotation: &mut perfetto::protos::pbzero::DebugAnnotation) {
        let mut json = String::new();
        json.push('{');
        json.push_str(&self.data);
        json.push('}');
        annotation.set_legacy_json_value(json);
    }

    #[cfg(not(feature = "perfetto"))]
    pub fn add(&self, _annotation: &mut ()) {
        // Dummy implementation for when perfetto feature is disabled
    }
}

impl Drop for TracedValue {
    fn drop(&mut self) {
        self.dcheck_current_container_is(StackType::Dict);
        self.debug_pop_container();
        self.dcheck_container_stack_depth_eq(0);
    }
}
// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/libplatform/tracing/trace-writer.h equivalent
mod trace_writer {
    use std::any::Any;
    use std::io::Write;

    pub trait ConvertableToTraceFormat {
        fn append_as_trace_format(&self, out: &mut String);
    }

    pub struct ArgValue {
        pub as_uint: u64,
        pub as_int: i64,
        pub as_double: f64,
        pub as_pointer: *const std::ffi::c_void,
        pub as_string: *const std::ffi::c_char, //Need to convert to Rust Strings when used
    }

    impl ArgValue {
        pub fn default() -> Self {
            ArgValue {
                as_uint: 0,
                as_int: 0,
                as_double: 0.0,
                as_pointer: std::ptr::null(),
                as_string: std::ptr::null(),
            }
        }
    }

    pub trait TraceWriter {
        fn append_trace_event(&mut self, trace_event: &TraceObject);
        fn flush(&mut self);
    }

    pub trait TraceObjectTrait {
        fn pid(&self) -> i32;
        fn tid(&self) -> i32;
        fn ts(&self) -> i64;
        fn tts(&self) -> i64;
        fn phase(&self) -> &str;
        fn category_enabled_flag(&self) -> *const std::ffi::c_char;
        fn name(&self) -> &str;
        fn duration(&self) -> i64;
        fn cpu_duration(&self) -> i64;
        fn flags(&self) -> u32;
        fn bind_id(&self) -> u64;
        fn scope(&self) -> *const std::ffi::c_char;
        fn id(&self) -> u64;
        fn arg_names(&self) -> &[*const std::ffi::c_char];
        fn arg_types(&self) -> &[u8];
        fn arg_values(&self) -> &[ArgValue];
        fn arg_convertables(&self) -> &[*mut dyn ConvertableToTraceFormat];
        fn num_args(&self) -> usize;
    }

    // Dummy implementation, needs to be implemented properly
    pub struct TraceObject {
        pid: i32,
        tid: i32,
        ts: i64,
        tts: i64,
        phase: String,
        category_enabled_flag: *const std::ffi::c_char,
        name: String,
        duration: i64,
        cpu_duration: i64,
        flags: u32,
        bind_id: u64,
        scope: *const std::ffi::c_char,
        id: u64,
        arg_names: Vec<*const std::ffi::c_char>,
        arg_types: Vec<u8>,
        arg_values: Vec<ArgValue>,
        arg_convertables: Vec<*mut dyn ConvertableToTraceFormat>,
        num_args: usize,
    }

    impl TraceObject {
        pub fn new() -> Self {
            TraceObject {
                pid: 0,
                tid: 0,
                ts: 0,
                tts: 0,
                phase: String::new(),
                category_enabled_flag: std::ptr::null(),
                name: String::new(),
                duration: 0,
                cpu_duration: 0,
                flags: 0,
                bind_id: 0,
                scope: std::ptr::null(),
                id: 0,
                arg_names: Vec::new(),
                arg_types: Vec::new(),
                arg_values: Vec::new(),
                arg_convertables: Vec::new(),
                num_args: 0,
            }
        }

        pub fn set_pid(&mut self, pid: i32) {
            self.pid = pid;
        }
    }

    impl TraceObjectTrait for TraceObject {
        fn pid(&self) -> i32 {
            self.pid
        }
        fn tid(&self) -> i32 {
            self.tid
        }
        fn ts(&self) -> i64 {
            self.ts
        }
        fn tts(&self) -> i64 {
            self.tts
        }
        fn phase(&self) -> &str {
            &self.phase
        }
        fn category_enabled_flag(&self) -> *const std::ffi::c_char {
            self.category_enabled_flag
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn duration(&self) -> i64 {
            self.duration
        }
        fn cpu_duration(&self) -> i64 {
            self.cpu_duration
        }
        fn flags(&self) -> u32 {
            self.flags
        }
        fn bind_id(&self) -> u64 {
            self.bind_id
        }
        fn scope(&self) -> *const std::ffi::c_char {
            self.scope
        }
        fn id(&self) -> u64 {
            self.id
        }
        fn arg_names(&self) -> &[*const std::ffi::c_char] {
            &self.arg_names
        }
        fn arg_types(&self) -> &[u8] {
            &self.arg_types
        }
        fn arg_values(&self) -> &[ArgValue] {
            &self.arg_values
        }
        fn arg_convertables(&self) -> &[*mut dyn ConvertableToTraceFormat] {
            &self.arg_convertables
        }
        fn num_args(&self) -> usize {
            self.num_args
        }
    }

    pub fn create_json_trace_writer<W: Write>(stream: W) -> Box<dyn TraceWriter> {
        Box::new(JSONTraceWriter::new(stream, "traceEvents".to_string()))
    }

    pub fn create_json_trace_writer_with_tag<W: Write>(stream: W, tag: String) -> Box<dyn TraceWriter> {
        Box::new(JSONTraceWriter::new(stream, tag))
    }

    // Values for arg types
    pub const TRACE_VALUE_TYPE_BOOL: u8 = 0;
    pub const TRACE_VALUE_TYPE_UINT: u8 = 1;
    pub const TRACE_VALUE_TYPE_INT: u8 = 2;
    pub const TRACE_VALUE_TYPE_DOUBLE: u8 = 3;
    pub const TRACE_VALUE_TYPE_POINTER: u8 = 4;
    pub const TRACE_VALUE_TYPE_STRING: u8 = 5;
    pub const TRACE_VALUE_TYPE_COPY_STRING: u8 = 6;
    pub const TRACE_VALUE_TYPE_CONVERTABLE: u8 = 7;

    // Flags for trace events
    pub const TRACE_EVENT_FLAG_FLOW_IN: u32 = 1 << 0;
    pub const TRACE_EVENT_FLAG_FLOW_OUT: u32 = 1 << 1;
    pub const TRACE_EVENT_FLAG_HAS_ID: u32 = 1 << 2;

    //TODO: Add SystemInstrumentationTraceWriter implementation when Recorder is implemented.
}

// src/libplatform/tracing/trace-writer.cc equivalent
pub mod tracing {
    use super::trace_writer::*;
    use std::io::Write;
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::fmt::Write as FmtWrite;

    // Writes the given string to a stream, taking care to escape characters
    // when necessary.
    fn write_json_string_to_stream<W: Write>(str: &str, stream: &mut W) -> std::io::Result<()> {
        stream.write_all(b"\"")?;
        for char in str.chars() {
            match char {
                '\u{0008}' => stream.write_all(b"\\b")?,
                '\u{000C}' => stream.write_all(b"\\f")?,
                '\n' => stream.write_all(b"\\n")?,
                '\r' => stream.write_all(b"\\r")?,
                '\t' => stream.write_all(b"\\t")?,
                '"' => stream.write_all(b"\\\"")?,
                '\\' => stream.write_all(b"\\\\")?,
                _ => {
                    let mut buf = [0u8; 4];
                    let s = char.encode_utf8(&mut buf);
                    stream.write_all(s.as_bytes())?
                },
            }
        }
        stream.write_all(b"\"")?;
        Ok(())
    }

    pub struct JSONTraceWriter<W: Write> {
        stream: W,
        append_comma: bool,
        tag: String,
    }

    impl<W: Write> JSONTraceWriter<W> {
        pub fn new(stream: W, tag: String) -> Self {
            let mut writer = JSONTraceWriter {
                stream,
                append_comma: false,
                tag,
            };
            write!(&mut writer.stream, "{{\"{}\":[", writer.tag).unwrap();
            writer
        }

        fn append_arg_value(&mut self, type_: u8, value: ArgValue) -> std::io::Result<()> {
            match type_ {
                TRACE_VALUE_TYPE_BOOL => {
                    if value.as_uint != 0 {
                        write!(&mut self.stream, "true")?;
                    } else {
                        write!(&mut self.stream, "false")?;
                    }
                }
                TRACE_VALUE_TYPE_UINT => {
                    write!(&mut self.stream, "{}", value.as_uint)?;
                }
                TRACE_VALUE_TYPE_INT => {
                    write!(&mut self.stream, "{}", value.as_int)?;
                }
                TRACE_VALUE_TYPE_DOUBLE => {
                    let val = value.as_double;
                    if val.is_finite() {
                        let real = val.to_string();
                        if !real.contains('.') && !real.contains('e') && !real.contains('E') {
                            write!(&mut self.stream, "{}.0", real)?;
                        } else {
                            write!(&mut self.stream, "{}", real)?;
                        }
                    } else if val.is_nan() {
                        write!(&mut self.stream, "\"NaN\"")?;
                    } else if val < 0.0 {
                        write!(&mut self.stream, "\"-Infinity\"")?;
                    } else {
                        write!(&mut self.stream, "\"Infinity\"")?;
                    }
                }
                TRACE_VALUE_TYPE_POINTER => {
                    write!(&mut self.stream, "\"{:p}\"", value.as_pointer)?;
                }
                TRACE_VALUE_TYPE_STRING | TRACE_VALUE_TYPE_COPY_STRING => {
                    if value.as_string.is_null() {
                        write!(&mut self.stream, "\"nullptr\"")?;
                    } else {
                        let c_str = unsafe { CStr::from_ptr(value.as_string) };
                        let str = c_str.to_str().unwrap();
                        write_json_string_to_stream(str, &mut self.stream)?;
                    }
                }
                _ => {
                    panic!("UNREACHABLE");
                }
            }
            Ok(())
        }

        fn append_arg_value_convertable(&mut self, value: &mut dyn ConvertableToTraceFormat) -> std::io::Result<()> {
            let mut arg_stringified = String::new();
            value.append_as_trace_format(&mut arg_stringified);
            write!(&mut self.stream, "{}", arg_stringified)?;
            Ok(())
        }
    }

    impl<W: Write> TraceWriter for JSONTraceWriter<W> {
        fn append_trace_event(&mut self, trace_event: &TraceObject) {
            if self.append_comma {
                write!(&mut self.stream, ",").unwrap();
            }
            self.append_comma = true;

            write!(&mut self.stream,
                "{{\"pid\":{},\"tid\":{},\"ts\":{},\"tts\":{},\"ph\":\"{}\",\"cat\":\"{}\",\"name\":\"{}\",\"dur\":{},\"tdur\":{}",
                trace_event.pid(),
                trace_event.tid(),
                trace_event.ts(),
                trace_event.tts(),
                trace_event.phase(),
                unsafe {
                    CStr::from_ptr(trace_event.category_enabled_flag()).to_str().unwrap()
                },
                trace_event.name(),
                trace_event.duration(),
                trace_event.cpu_duration()
            ).unwrap();

            if trace_event.flags() & (TRACE_EVENT_FLAG_FLOW_IN | TRACE_EVENT_FLAG_FLOW_OUT) != 0 {
                write!(&mut self.stream, ",\"bind_id\":\"0x{:x}\"", trace_event.bind_id()).unwrap();

                if trace_event.flags() & TRACE_EVENT_FLAG_FLOW_IN != 0 {
                    write!(&mut self.stream, ",\"flow_in\":true").unwrap();
                }
                if trace_event.flags() & TRACE_EVENT_FLAG_FLOW_OUT != 0 {
                    write!(&mut self.stream, ",\"flow_out\":true").unwrap();
                }
            }

            if trace_event.flags() & TRACE_EVENT_FLAG_HAS_ID != 0 {
                if !trace_event.scope().is_null() {
                    let scope = unsafe { CStr::from_ptr(trace_event.scope()).to_str().unwrap() };
                    write!(&mut self.stream, ",\"scope\":\"{}\"", scope).unwrap();
                }

                write!(&mut self.stream, ",\"id\":\"0x{:x}\"", trace_event.id()).unwrap();
            }

            write!(&mut self.stream, ",\"args\":{{").unwrap();
            let arg_names = trace_event.arg_names();
            let arg_types = trace_event.arg_types();
            let arg_values = trace_event.arg_values();
            // let arg_convertables = trace_event.arg_convertables(); //TODO: Handle arg_convertables

            for i in 0..trace_event.num_args() {
                if i > 0 {
                    write!(&mut self.stream, ",").unwrap();
                }
                let arg_name = unsafe { CStr::from_ptr(arg_names[i]).to_str().unwrap() };
                write!(&mut self.stream, "\"{}\":", arg_name).unwrap();
                
                //TODO: Handle arg_convertables, need to figure out lifetimes here for the dyn trait
                // if arg_types[i] == TRACE_VALUE_TYPE_CONVERTABLE {
                //     if let Some(convertable) = arg_convertables.get(i) {
                //         self.append_arg_value_convertable(unsafe { &mut **convertable }).unwrap();
                //     }
                // } else {
                    self.append_arg_value(arg_types[i], arg_values[i]).unwrap();
                // }
            }

            write!(&mut self.stream, "}}}}").unwrap();
        }

        fn flush(&mut self) {}
    }

    impl<W: Write> Drop for JSONTraceWriter<W> {
        fn drop(&mut self) {
            write!(&mut self.stream, "]}}").unwrap();
        }
    }

    //Need Recorder and other parts of SystemInstrumentationTraceWriter to implement
    #[cfg(feature = "system_instrumentation")]
    pub struct SystemInstrumentationTraceWriter {
        recorder: std::unique_ptr::UniquePtr<Recorder>,
    }

    #[cfg(feature = "system_instrumentation")]
    impl SystemInstrumentationTraceWriter {
        pub fn new() -> Self {
            SystemInstrumentationTraceWriter {
                recorder: std::unique_ptr::UniquePtr::new(Recorder::new()),
            }
        }
    }

    #[cfg(feature = "system_instrumentation")]
    impl TraceWriter for SystemInstrumentationTraceWriter {
        fn append_trace_event(&mut self, trace_event: &TraceObject) {
            if self.recorder.is_enabled() {
                self.recorder.add_event(trace_event);
            }
        }

        fn flush(&mut self) {}
    }

    #[cfg(feature = "system_instrumentation")]
    impl Drop for SystemInstrumentationTraceWriter {
        fn drop(&mut self) {
            self.recorder.reset(nullptr);
        }
    }

    #[cfg(feature = "system_instrumentation")]
    pub fn create_system_instrumentation_trace_writer() -> Box<dyn TraceWriter> {
        Box::new(SystemInstrumentationTraceWriter::new())
    }

    //Stubs for compilation purposes.

    pub struct TracingController {}
    impl TracingController {
        pub fn get_category_group_name(_flag: *const c_char) -> &'static str {
            "dummy_category"
        }
    }
}
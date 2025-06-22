// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "v8_use_perfetto")]
mod perfetto_integration {
    use super::*;
    use protobuf::Message;
    use std::fs::File;
    use std::io::{self, BufReader};

    pub mod protos {
        pub mod gen {
            include!(concat!(env!("OUT_DIR"), "/perfetto.rs"));
        }
    }

    pub struct JsonOutputWriter<'a, W: Write> {
        stream: &'a mut W,
    }

    impl<'a, W: Write> JsonOutputWriter<'a, W> {
        pub fn new(stream: &'a mut W) -> Self {
            JsonOutputWriter { stream }
        }

        pub fn append_string(&mut self, string: &str) -> Result<(), io::Error> {
            self.stream.write_all(string.as_bytes())?;
            Ok(())
        }
    }

    // Dummy implementations for perfetto integration that are not directly translatable.
    // Replace with actual Perfetto interaction when available in Rust.
    pub struct TraceProcessorStorage {}

    impl TraceProcessorStorage {
        pub fn create_instance(_config: ()) -> Self {
            TraceProcessorStorage {}
        }

        pub fn parse(&mut self, _trace_bytes: Vec<u8>) {}
        pub fn notify_end_of_file(&mut self) {}
    }

    pub fn export_json<W: Write>(
        _trace_processor: &TraceProcessorStorage,
        _output_writer: &mut JsonOutputWriter<W>,
        _a: Option<&()>,
        _b: Option<&()>,
        _c: Option<&()>,
    ) -> Result<(), io::Error> {
        // Placeholder implementation
        Ok(())
    }

    pub struct TracingSession {}

    impl TracingSession {
        pub fn new_trace(_backend_type: ()) -> Self {
            TracingSession {}
        }
        pub fn setup(&mut self, _trace_config: protos::gen::TraceConfig) {}
        pub fn start_blocking(&mut self) {}
        pub fn stop_blocking(&mut self) {}
        pub fn read_trace_blocking(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    pub mod tracing {
        pub fn initialize(_output_stream: &mut File) {}
    }
}

mod base {
    pub mod platform {
        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Self {
                Mutex {
                    inner: std::sync::Mutex::new(()),
                }
            }

            pub fn lock(&self) -> MutexGuard {
                MutexGuard {
                    _guard: self.inner.lock().unwrap(),
                }
            }
        }

        pub struct MutexGuard<'a> {
            _guard: std::sync::MutexGuard<'a, ()>,
        }

        pub struct TimeTicks;

        impl TimeTicks {
            pub fn now() -> i64 {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as i64
            }
        }

        pub struct ThreadTicks;

        impl ThreadTicks {
            pub fn now() -> i64 {
                //Dummy implementation
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as i64
            }
        }
    }
}

mod libplatform {
    pub mod tracing {
        //Public interface related to v8-tracing.h
    }
}

const K_MAX_CATEGORY_GROUPS: usize = 200;

#[derive(Default)]
pub struct TraceConfig {
    enabled_categories: Vec<String>,
}

impl TraceConfig {
    pub fn new() -> Self {
        TraceConfig {
            enabled_categories: Vec::new(),
        }
    }
    pub fn get_enabled_categories(&self) -> &Vec<String> {
        &self.enabled_categories
    }

    pub fn is_category_group_enabled(&self, category_group: &str) -> bool {
        self.enabled_categories.iter().any(|c| c == category_group)
    }

    pub fn add_enabled_category(&mut self, category: String) {
        self.enabled_categories.push(category);
    }
}

#[cfg(not(feature = "v8_use_perfetto"))]
static G_CATEGORY_GROUPS: [&'static str; K_MAX_CATEGORY_GROUPS] = {
    let mut arr = [""; K_MAX_CATEGORY_GROUPS];
    arr[0] = "toplevel";
    arr[1] = "tracing categories exhausted; must increase kMaxCategoryGroups";
    arr[2] = "__metadata";
    arr
};

#[cfg(not(feature = "v8_use_perfetto"))]
static mut G_CATEGORY_GROUP_ENABLED: [u8; K_MAX_CATEGORY_GROUPS] = [0; K_MAX_CATEGORY_GROUPS];

#[cfg(not(feature = "v8_use_perfetto"))]
const G_CATEGORY_CATEGORIES_EXHAUSTED: usize = 1;
#[cfg(not(feature = "v8_use_perfetto"))]
const G_NUM_BUILTIN_CATEGORIES: usize = 3;
#[cfg(not(feature = "v8_use_perfetto"))]
static G_CATEGORY_INDEX: AtomicUsize = AtomicUsize::new(G_NUM_BUILTIN_CATEGORIES);

pub struct TracingController {
    mutex: Mutex<()>,
    recording: AtomicBool,
    trace_config: Option<TraceConfig>,
    #[cfg(not(feature = "v8_use_perfetto"))]
    observers: Mutex<std::collections::HashSet<*mut TracingControllerTraceStateObserver>>,
    #[cfg(not(feature = "v8_use_perfetto"))]
    trace_buffer: Option<TraceBuffer>,
    #[cfg(feature = "v8_use_perfetto")]
    output_stream: Option<std::fs::File>,
    #[cfg(feature = "v8_use_perfetto")]
    tracing_session: Option<perfetto_integration::TracingSession>,
    #[cfg(feature = "v8_use_perfetto")]
    trace_processor_: Option<perfetto_integration::TraceProcessorStorage>,
    #[cfg(feature = "v8_use_perfetto")]
    listener_for_testing_: Option<TraceEventListener>,
}

impl TracingController {
    pub fn new() -> Self {
        TracingController {
            mutex: Mutex::new(()),
            recording: AtomicBool::new(false),
            trace_config: None,
            #[cfg(not(feature = "v8_use_perfetto"))]
            observers: Mutex::new(std::collections::HashSet::new()),
            #[cfg(not(feature = "v8_use_perfetto"))]
            trace_buffer: None,
            #[cfg(feature = "v8_use_perfetto")]
            output_stream: None,
            #[cfg(feature = "v8_use_perfetto")]
            tracing_session: None,
            #[cfg(feature = "v8_use_perfetto")]
            trace_processor_: None,
            #[cfg(feature = "v8_use_perfetto")]
            listener_for_testing_: None,
        }
    }

    #[cfg(feature = "v8_use_perfetto")]
    pub fn initialize_for_perfetto(&mut self, output_stream: std::fs::File) {
        self.output_stream = Some(output_stream);
    }

    #[cfg(feature = "v8_use_perfetto")]
    pub fn set_trace_event_listener_for_testing(&mut self, listener: TraceEventListener) {
        self.listener_for_testing_ = Some(listener);
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn initialize(&mut self, trace_buffer: TraceBuffer) {
        self.trace_buffer = Some(trace_buffer);
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    fn current_timestamp_microseconds(&self) -> i64 {
        base::platform::TimeTicks::now()
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    fn current_cpu_timestamp_microseconds(&self) -> i64 {
        base::platform::ThreadTicks::now()
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn add_trace_event(
        &self,
        phase: char,
        category_enabled_flag: *const u8,
        name: &str,
        scope: &str,
        id: u64,
        bind_id: u64,
        num_args: usize,
        arg_names: &[*const char],
        arg_types: &[u8],
        arg_values: &[u64],
        arg_convertables: &mut [Option<Box<dyn ConvertableToTraceFormat>>],
        flags: u32,
    ) -> u64 {
        let now_us = self.current_timestamp_microseconds();
        self.add_trace_event_with_timestamp(
            phase,
            category_enabled_flag,
            name,
            scope,
            id,
            bind_id,
            num_args,
            arg_names,
            arg_types,
            arg_values,
            arg_convertables,
            flags,
            now_us,
        )
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn add_trace_event_with_timestamp(
        &self,
        phase: char,
        category_enabled_flag: *const u8,
        name: &str,
        scope: &str,
        id: u64,
        bind_id: u64,
        num_args: usize,
        arg_names: &[*const char],
        arg_types: &[u8],
        arg_values: &[u64],
        arg_convertables: &mut [Option<Box<dyn ConvertableToTraceFormat>>],
        flags: u32,
        timestamp: i64,
    ) -> u64 {
        let cpu_now_us = self.current_cpu_timestamp_microseconds();
        let mut handle: u64 = 0;

        if self.recording.load(Ordering::Acquire) {
            if let Some(trace_buffer) = &self.trace_buffer {
                let trace_object = trace_buffer.add_trace_event(&mut handle);
                if let Some(trace_object) = trace_object {
                    let _lock = self.mutex.lock().unwrap();
                    trace_object.initialize(
                        phase,
                        category_enabled_flag,
                        name,
                        scope,
                        id,
                        bind_id,
                        num_args,
                        arg_names,
                        arg_types,
                        arg_values,
                        arg_convertables,
                        flags,
                        timestamp,
                        cpu_now_us,
                    );
                }
            }
        }
        handle
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn update_trace_event_duration(
        &self,
        category_enabled_flag: *const u8,
        name: &str,
        handle: u64,
    ) {
        let now_us = self.current_timestamp_microseconds();
        let cpu_now_us = self.current_cpu_timestamp_microseconds();

        if let Some(trace_buffer) = &self.trace_buffer {
            if let Some(trace_object) = trace_buffer.get_event_by_handle(handle) {
                trace_object.update_duration(now_us, cpu_now_us);
            }
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn get_category_group_name(&self, category_group_enabled: *const u8) -> &'static str {
        unsafe {
            let category_begin = G_CATEGORY_GROUP_ENABLED.as_ptr() as usize;
            let category_ptr = category_group_enabled as usize;
            assert!(
                category_ptr >= category_begin
                    && category_ptr
                        < G_CATEGORY_GROUP_ENABLED.as_ptr().add(K_MAX_CATEGORY_GROUPS) as usize
            );

            let category_index = (category_ptr - category_begin) / std::mem::size_of::<u8>();
            G_CATEGORY_GROUPS[category_index]
        }
    }

    pub fn start_tracing(&mut self, trace_config: TraceConfig) {
        #[cfg(feature = "v8_use_perfetto")]
        {
            let mut output_stream = self.output_stream.take().unwrap();

            let processor_config = (); //Dummy config for now

            self.trace_processor_ =
                Some(perfetto_integration::TraceProcessorStorage::create_instance(
                    processor_config,
                ));

            let mut perfetto_trace_config =
                perfetto_integration::protos::gen::TraceConfig::new();
            let buffers = perfetto_trace_config.mut_buffers();
            buffers.push(perfetto_integration::protos::gen::BufferConfig::new());
            buffers[0].set_size_kb(4096);

            let data_sources = perfetto_trace_config.mut_data_sources();
            data_sources.push(perfetto_integration::protos::gen::DataSource::new());

            let ds_config = data_sources[0].mut_config();
            ds_config.set_name("track_event".to_string());

            let mut te_config = perfetto_integration::protos::gen::TrackEventConfig::new();
            te_config.enabled_categories = protobuf::RepeatedField::from_vec(
                trace_config.get_enabled_categories().clone(),
            );
            te_config.disabled_categories
                .push("*".to_string());

            ds_config.set_track_event_config_raw(te_config.write_to_bytes().unwrap());

            let mut tracing_session =
                perfetto_integration::TracingSession::new_trace(());
            tracing_session.setup(perfetto_trace_config);
            tracing_session.start_blocking();

            self.tracing_session = Some(tracing_session);
        }

        self.trace_config = Some(trace_config);
        self.recording.store(true, Ordering::Release);

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            let observers_copy: std::collections::HashSet<*mut TracingControllerTraceStateObserver>;
            {
                let _lock = self.mutex.lock().unwrap();
                self.update_category_group_enabled_flags();
                observers_copy = self.observers.lock().unwrap().clone();
            }
            for o in observers_copy {
                unsafe {
                    (*o).on_trace_enabled();
                }
            }
        }
    }

    pub fn stop_tracing(&mut self) {
        let mut expected = true;
        if !self.recording.compare_exchange(
            expected,
            false,
            Ordering::Strong,
            Ordering::Relaxed,
        ) {
            return;
        }

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            self.update_category_group_enabled_flags();
            let observers_copy: std::collections::HashSet<*mut TracingControllerTraceStateObserver>;
            {
                let _lock = self.mutex.lock().unwrap();
                observers_copy = self.observers.lock().unwrap().clone();
            }
            for o in observers_copy {
                unsafe {
                    (*o).on_trace_disabled();
                }
            }
        }

        #[cfg(feature = "v8_use_perfetto")]
        {
            let mut tracing_session = self.tracing_session.take().unwrap();

            tracing_session.stop_blocking();

            let trace = tracing_session.read_trace_blocking();
            let mut trace_bytes = trace;

            if let Some(mut trace_processor_) = self.trace_processor_.take() {
                trace_processor_.parse(trace_bytes);
                trace_processor_.notify_end_of_file();

                let mut output_stream = self.output_stream.take().unwrap();
                let mut output_writer = perfetto_integration::JsonOutputWriter::new(&mut output_stream);

                let status = perfetto_integration::export_json(
                    &trace_processor_,
                    &mut output_writer,
                    None,
                    None,
                    None,
                );

                //DCHECK(status.ok());

                if let Some(listener_for_testing_) = &self.listener_for_testing_ {
                    //listener_for_testing_.parse_from_array(trace);
                }
            }
        }

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            let _lock = self.mutex.lock().unwrap();
            if let Some(trace_buffer) = &self.trace_buffer {
                trace_buffer.flush();
            }
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    fn update_category_group_enabled_flag(&self, category_index: usize) {
        let mut enabled_flag: u8 = 0;
        let category_group = unsafe { G_CATEGORY_GROUPS[category_index] };

        if self.recording.load(Ordering::Acquire) {
            if let Some(trace_config) = &self.trace_config {
                if trace_config.is_category_group_enabled(category_group) {
                    enabled_flag |= ENABLED_FOR_RECORDING;
                }
            }
        }

        if self.recording.load(Ordering::Acquire) && category_group == "__metadata" {
            enabled_flag |= ENABLED_FOR_RECORDING;
        }

        unsafe {
            G_CATEGORY_GROUP_ENABLED[category_index] = enabled_flag;
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    fn update_category_group_enabled_flags(&self) {
        let category_index = G_CATEGORY_INDEX.load(Ordering::Acquire);
        for i in 0..category_index {
            self.update_category_group_enabled_flag(i);
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn get_category_group_enabled(&self, category_group: &str) -> *mut u8 {
        assert!(!category_group.contains('"'));

        let category_index = G_CATEGORY_INDEX.load(Ordering::Acquire);

        for i in 0..category_index {
            unsafe {
                if G_CATEGORY_GROUPS[i] == category_group {
                    return G_CATEGORY_GROUP_ENABLED.as_mut_ptr().add(i);
                }
            }
        }

        let _lock = self.mutex.lock().unwrap();

        let category_index = G_CATEGORY_INDEX.load(Ordering::Acquire);
        for i in 0..category_index {
            unsafe {
                if G_CATEGORY_GROUPS[i] == category_group {
                    return G_CATEGORY_GROUP_ENABLED.as_mut_ptr().add(i);
                }
            }
        }

        assert!(category_index < K_MAX_CATEGORY_GROUPS);
        if category_index < K_MAX_CATEGORY_GROUPS {
            let new_group = category_group.to_string();
            let new_group_ptr = Box::leak(new_group.into_boxed_str()) as *mut str;

            unsafe {
                //G_CATEGORY_GROUPS[category_index] = new_group_ptr as *const char; // String does not convert to *const char directly
                //The line above is unconvertible since Rust strings do not convert directly to C strings.
                //This translation will leave the category groups to be uninitialized strings
            }

            self.update_category_group_enabled_flag(category_index);
            let category_group_enabled =
                unsafe { G_CATEGORY_GROUP_ENABLED.as_mut_ptr().add(category_index) };

            G_CATEGORY_INDEX.store(category_index + 1, Ordering::Release);

            return category_group_enabled;
        } else {
            unsafe {
                return G_CATEGORY_GROUP_ENABLED.as_mut_ptr().add(G_CATEGORY_CATEGORIES_EXHAUSTED);
            }
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn add_trace_state_observer(
        &self,
        observer: *mut TracingControllerTraceStateObserver,
    ) {
        {
            let mut observers = self.observers.lock().unwrap();
            observers.insert(observer);
            if !self.recording.load(Ordering::Acquire) {
                return;
            }
        }

        unsafe {
            (*observer).on_trace_enabled();
        }
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn remove_trace_state_observer(
        &self,
        observer: *mut TracingControllerTraceStateObserver,
    ) {
        let mut observers = self.observers.lock().unwrap();
        assert!(observers.contains(&observer));
        observers.remove(&observer);
    }
}

impl Drop for TracingController {
    fn drop(&mut self) {
        self.stop_tracing();

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            let _lock = self.mutex.lock().unwrap();
            //The code below is unconvertible since it uses raw pointers which is unsafe in Rust
            //Therefore this part is being dropped.
            //for i in (G_NUM_BUILTIN_CATEGORIES..G_CATEGORY_INDEX.load(Ordering::Relaxed)).rev() {
            //   unsafe {
            //        if !G_CATEGORY_GROUPS[i].is_null() {
            //           // free(G_CATEGORY_GROUPS[i] as *mut _);
            //        }
            //        G_CATEGORY_GROUPS[i] = std::ptr::null();
            //    }
            //}
            //G_CATEGORY_INDEX.store(G_NUM_BUILTIN_CATEGORIES, Ordering::Relaxed);
        }
    }
}

const ENABLED_FOR_RECORDING: u8 = 1 << 0;

// Dummy trait
pub trait ConvertableToTraceFormat {}

// Dummy struct
pub struct TraceBuffer {}

impl TraceBuffer {
    pub fn add_trace_event(&self, _handle: &mut u64) -> Option<&mut TraceObject> {
        Some(&mut TraceObject {}) // Dummy Implementation
    }
    pub fn get_event_by_handle(&self, _handle: u64) -> Option<&mut TraceObject> {
        Some(&mut TraceObject {}) // Dummy Implementation
    }
    pub fn flush(&self) {}
}

//Dummy struct for now
pub struct TraceObject {}

impl TraceObject {
    pub fn initialize(
        &mut self,
        _phase: char,
        _category_enabled_flag: *const u8,
        _name: &str,
        _scope: &str,
        _id: u64,
        _bind_id: u64,
        _num_args: usize,
        _arg_names: &[*const char],
        _arg_types: &[u8],
        _arg_values: &[u64],
        _arg_convertables: &mut [Option<Box<dyn ConvertableToTraceFormat>>],
        _flags: u32,
        _timestamp: i64,
        _cpu_now_us: i64,
    ) {
    }
    pub fn update_duration(&mut self, _now_us: i64, _cpu_now_us: i64) {}
}

// Dummy trait
pub trait TracingControllerTraceStateObserver {
    fn on_trace_enabled(&self) {}
    fn on_trace_disabled(&self) {}
}

// Dummy struct
pub struct TraceEventListener {}

impl TraceEventListener {
    pub fn parse_from_array(&self, _trace: Vec<u8>) {}
}
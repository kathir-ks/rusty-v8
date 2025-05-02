// src/libplatform/tracing/recorder_win.rs

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::winerror::HRESULT;
use winapi::um::processthreadsapi::{GetCurrentProcessId, GetCurrentThreadId};
use winapi::um::synchapi::Sleep;
use winapi::um::winnt::{EVENT_DESCRIPTOR, ULONGLONG, USHORT};
use winapi::um::winnt::{EVENT_DATA_DESCRIPTOR, TRACE_PROVIDER_GUID};
use winapi::um::evntrace::{
    EventRegister, EventUnregister, EventWrite, TRACEHANDLE, REGHANDLE
};
use widestring::U16CString;

// Define a macro for declaring and defining the provider GUID
macro_rules! define_trace_provider {
    ($provider_name:ident, $guid:expr) => {
        lazy_static::lazy_static! {
            pub static ref $provider_name: TRACE_PROVIDER_GUID = $guid;
        }
    };
}

// Define a macro for declaring and defining the provider handle
macro_rules! define_trace_handle {
    ($handle_name:ident) => {
        static $handle_name: std::sync::Mutex<REGHANDLE> = std::sync::Mutex::new(0 as REGHANDLE);
    };
}

// Define a macro for registering a trace provider
macro_rules! register_trace_provider {
    ($provider_guid:expr, $handle_name:ident) => {
        unsafe {
            let mut handle = $handle_name.lock().unwrap();
            let result = EventRegister(
                $provider_guid as *const _,
                None,
                None,
                handle.deref_mut()
            );
            if result != 0 {
                eprintln!("EventRegister failed with error code: {}", result);
            }
        }
    };
}

// Define a macro for unregistering a trace provider
macro_rules! unregister_trace_provider {
    ($handle_name:ident) => {
        unsafe {
            let mut handle = $handle_name.lock().unwrap();
            if *handle != 0 as REGHANDLE {
                let result = EventUnregister(*handle);
                if result != 0 {
                    eprintln!("EventUnregister failed with error code: {}", result);
                }
                *handle = 0 as REGHANDLE;
            }
        }
    };
}

// Define a macro for writing trace events
macro_rules! write_trace_event {
    ($handle_name:ident, $event_descriptor:expr, $($property_name:literal => $property_value:expr),*) => {
        unsafe {
            let handle = $handle_name.lock().unwrap();
            if *handle != 0 as REGHANDLE {
                let mut data_descriptors: Vec<EVENT_DATA_DESCRIPTOR> = Vec::new();
                $(
                    let property_ptr = $property_value as *const _ as *mut _;
                    let property_size = std::mem::size_of_val(&$property_value) as u32;
                    let data_descriptor = EVENT_DATA_DESCRIPTOR {
                        Ptr: property_ptr as ULONGLONG,
                        Size: property_size,
                        Reserved: 0,
                    };
                    data_descriptors.push(data_descriptor);
                )*

                let result = EventWrite(
                    *handle,
                    $event_descriptor as *const _,
                    data_descriptors.len() as u32,
                    data_descriptors.as_mut_ptr(),
                );
                if result != 0 {
                    eprintln!("EventWrite failed with error code: {}", result);
                }
            }
        }
    };
}

pub mod etw {
    pub mod etw_provider_win {
        use winapi::shared::guiddef::GUID;

        //Example usage, replace with actual provider GUID
        pub const V8_LIB_PROVIDER_GUID: GUID = GUID {
            Data1: 0x12345678,
            Data2: 0x1234,
            Data3: 0x1234,
            Data4: [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef],
        };
    }
}

pub mod tracing {
    use super::*;
    use crate::etw::etw_provider_win::V8_LIB_PROVIDER_GUID;
    use std::sync::Mutex;
    use std::ops::DerefMut;
    use std::convert::TryInto;

    define_trace_provider!(g_v8_lib_provider, V8_LIB_PROVIDER_GUID);
    define_trace_handle!(g_v8_lib_provider_handle);

    pub struct Recorder {}

    impl Recorder {
        pub fn new() -> Self {
            register_trace_provider!(&*g_v8_lib_provider, g_v8_lib_provider_handle);
            Recorder {}
        }

        pub fn is_enabled() -> bool {
            // Placeholder implementation.  Needs access to ETW session info.
            true
        }

        pub fn is_enabled_with_level(level: u8) -> bool {
            // Placeholder implementation.  Needs access to ETW session info and level.
            level > 0
        }

        pub fn add_event(&self, trace_event: &TraceObject) {
            // TODO(sartang@microsoft.com): Figure out how to write the conditional
            // arguments

            let wname = U16CString::from_str(trace_event.name).unwrap();

            #[cfg(not(feature = "perfetto"))]
            let wcategory_group_name =
                U16CString::from_str(TracingController::get_category_group_name(
                    trace_event.category_enabled_flag,
                ))
                .unwrap();

            #[cfg(feature = "perfetto")]
            let wcategory_group_name = U16CString::from_str("").unwrap();

            unsafe {
                let event_descriptor = EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: 0,
                    Task: 0,
                    Keyword: 0,
                };

                write_trace_event!(
                    g_v8_lib_provider_handle,
                    &event_descriptor,
                    "Event Name" => wname.as_ptr(),
                    "pid" => trace_event.pid,
                    "tid" => trace_event.tid,
                    "ts" => trace_event.ts,
                    "tts" => trace_event.tts,
                    "phase" => trace_event.phase,
                    "category" => wcategory_group_name.as_ptr(),
                    "dur" => trace_event.duration,
                    "tdur" => trace_event.cpu_duration
                );
            }
        }
    }

    impl Drop for Recorder {
        fn drop(&mut self) {
            unregister_trace_provider!(g_v8_lib_provider_handle);
        }
    }

    pub struct TraceObject {
        name: String,
        pid: u32,
        tid: u32,
        ts: u64,
        tts: u64,
        phase: char,
        category_enabled_flag: u32,
        duration: u64,
        cpu_duration: u64,
    }

    impl TraceObject {
        pub fn new(
            name: String,
            pid: u32,
            tid: u32,
            ts: u64,
            tts: u64,
            phase: char,
            category_enabled_flag: u32,
            duration: u64,
            cpu_duration: u64,
        ) -> Self {
            TraceObject {
                name,
                pid,
                tid,
                ts,
                tts,
                phase,
                category_enabled_flag,
                duration,
                cpu_duration,
            }
        }
    }

    pub struct TracingController {}

    impl TracingController {
        pub fn get_category_group_name(_category_enabled_flag: u32) -> String {
            // Placeholder
            "Default".to_string()
        }
    }
}
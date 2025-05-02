// src/heap/cppgc/trace_event.rs

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(not(feature = "cppgc_is_standalone"))]
mod trace_event_wrapper {
    pub use v8::ConvertableToTraceFormat;
}

#[cfg(feature = "cppgc_is_standalone")]
mod trace_event_standalone {
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::{mem, ptr};

    use crate::base::atomicops::*;
    use crate::tracing::trace_event_no_perfetto::*;
    use crate::cppgc::platform::Platform;

    // These values must be in sync with macro values in trace_log.h in chromium.
    enum CategoryGroupEnabledFlags {
        // Category group enabled for the recording mode.
        kEnabledForRecording_CategoryGroupEnabledFlags = 1 << 0,
        // Category group enabled by SetEventCallbackEnabled().
        kEnabledForEventCallback_CategoryGroupEnabledFlags = 1 << 2,
    }

    macro_rules! internal_trace_event_category_group_enabled_for_recording_mode {
        () => {
            trace_event_api_load_category_group_enabled!()
                & (CategoryGroupEnabledFlags::kEnabledForRecording_CategoryGroupEnabledFlags as u8
                    | CategoryGroupEnabledFlags::kEnabledForEventCallback_CategoryGroupEnabledFlags as u8)
        };
    }

    macro_rules! trace_event_api_get_category_group_enabled {
        ($platform:expr) => {
            $platform.get_tracing_controller().get_category_group_enabled
        };
    }

    macro_rules! trace_event_api_add_trace_event {
        () => {
            crate::cppgc::internal::add_trace_event_impl
        };
    }

    macro_rules! trace_event_api_atomic_word {
        () => {
            AtomicWord
        };
    }

    macro_rules! trace_event_api_atomic_load {
        ($var:expr) => {
            acquire_load($var)
        };
    }

    macro_rules! trace_event_api_atomic_store {
        ($var:expr, $value:expr) => {
            release_store($var, $value)
        };
    }

    macro_rules! trace_event_api_load_category_group_enabled {
        () => {{
            unsafe {
                relaxed_load(
                    (&*internal_trace_event_uid!(category_group_enabled) as *const AtomicU8) as *const AtomicU8,
                )
            }
        }};
    }

    macro_rules! internal_trace_event_uid3 {
        ($a:ident, $b:ident) => {
            concat!("cppgc_trace_event_unique_", stringify!($a), stringify!($b))
        };
    }

    macro_rules! internal_trace_event_uid2 {
        ($a:ident, $b:ident) => {
            internal_trace_event_uid3!($a, $b)
        };
    }

    macro_rules! internal_trace_event_uid {
        ($name_prefix:ident) => {
            concat!("cppgc_trace_event_unique_", stringify!($name_prefix), line!())
        };
    }

    macro_rules! internal_trace_event_get_category_info_custom_variables {
        ($category_group:expr, $atomic:expr, $category_group_enabled:expr) => {
            unsafe {
                $category_group_enabled =
                    &*trace_event_api_atomic_load!($atomic) as *const u8;
                if ($category_group_enabled as *const u8).is_null() {
                   // let platform : &Platform = crate::stats_collector::get_platform();
                    $category_group_enabled = {
                        let category_group = $category_group;
                        let platform : &Platform = &crate::stats_collector::StatsCollector::get_platform();
                        platform.get_tracing_controller().get_category_group_enabled(category_group)
                    };

                    trace_event_api_atomic_store!(
                        $atomic,
                        $category_group_enabled as usize
                    );
                }
            }
        };
    }

    macro_rules! internal_trace_event_get_category_info {
        ($category_group:expr) => {
            static mut atomic: AtomicWord = 0;
            static mut category_group_enabled: *const u8 = ptr::null();
            internal_trace_event_get_category_info_custom_variables!(
                $category_group,
                atomic,
                category_group_enabled
            );
        };
    }

    // Dummy StatsCollector for standalone
    #[cfg(feature = "cppgc_is_standalone")]
    mod stats_collector {
        use crate::cppgc::platform::Platform;
        pub struct StatsCollector {}

        impl StatsCollector {
            pub fn get_platform() -> &'static Platform {
                static PLATFORM: Platform = Platform {};
                &PLATFORM
            }
        }
    }

    macro_rules! internal_trace_event_add {
        ($phase:expr, $category_group:expr, $name:expr, $flags:expr, $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                assert!(!$name.is_null());
            }
            {
                let platform: &crate::cppgc::platform::Platform = &stats_collector::StatsCollector::get_platform();
                internal_trace_event_get_category_info!($category_group);
                if internal_trace_event_category_group_enabled_for_recording_mode!() != 0 {
                    #[allow(unused_unsafe)]
                    unsafe {
                        crate::cppgc::internal::add_trace_event(
                            $phase,
                            category_group_enabled,
                            $name,
                            ptr::null(), /* scope */
                            0, /* id */
                            0, /* bind_id */
                            $flags,
                            platform,
                            $($arg)*
                        );
                    }
                }
            }
        };
    }

    pub mod cppgc {
        pub mod internal {
            use std::{mem, ptr};
            use crate::cppgc::platform::Platform;
            use crate::tracing::trace_event_no_perfetto::*;

            pub type ConvertableToTraceFormat = v8::ConvertableToTraceFormat;

            pub struct TraceEventHelper {}

            impl TraceEventHelper {
                pub fn get_tracing_controller() -> *mut dyn TracingController {
                    // Implementation detail is hidden, so return a null pointer as a placeholder.
                    ptr::null_mut()
                }
            }

            pub fn add_trace_event_impl(
                phase: char,
                category_group_enabled: *const u8,
                name: *const i8,
                scope: *const i8,
                id: u64,
                bind_id: u64,
                num_args: i32,
                arg_names: *const *const i8,
                arg_types: *const u8,
                arg_values: *const u64,
                flags: u32,
                platform: &Platform
            ) -> u64 {
                let mut arg_convertables: [Option<Box<ConvertableToTraceFormat>>; 2] = [None, None];
                if num_args > 0 {
                    unsafe {
                        if !arg_types.is_null() && !arg_values.is_null() {
                            let arg_types_slice = std::slice::from_raw_parts(arg_types, num_args as usize);
                            let arg_values_slice = std::slice::from_raw_parts(arg_values, num_args as usize);

                            if arg_types_slice[0] == TRACE_VALUE_TYPE_CONVERTABLE {
                                arg_convertables[0] = Some(unsafe {
                                    Box::from_raw(arg_values_slice[0] as *mut ConvertableToTraceFormat)
                                });
                            }
                            if num_args > 1 && arg_types_slice[1] == TRACE_VALUE_TYPE_CONVERTABLE {
                                arg_convertables[1] = Some(unsafe {
                                    Box::from_raw(arg_values_slice[1] as *mut ConvertableToTraceFormat)
                                });
                            }
                        }
                    }
                }
                assert!(num_args <= 2);
                let controller = platform.get_tracing_controller();

                let arg_names_slice = if !arg_names.is_null() {
                  unsafe {std::slice::from_raw_parts(arg_names, num_args as usize)}
                } else {
                  &[]
                };

                let arg_types_slice = if !arg_types.is_null() {
                  unsafe {std::slice::from_raw_parts(arg_types, num_args as usize)}
                } else {
                  &[]
                };

                let arg_values_slice = if !arg_values.is_null() {
                  unsafe {std::slice::from_raw_parts(arg_values, num_args as usize)}
                } else {
                  &[]
                };

                controller.add_trace_event(
                    phase,
                    category_group_enabled,
                    name,
                    scope,
                    id,
                    bind_id,
                    num_args,
                    arg_names_slice,
                    arg_types_slice,
                    arg_values_slice,
                    &arg_convertables,
                    flags,
                )
            }

            // Define SetTraceValue for each allowed type. It stores the type and value
            // in the return arguments. This allows this API to avoid declaring any
            // structures so that it is portable to third_party libraries.
            // This is the base implementation for integer types (including bool) and enums.
            pub fn set_trace_value<T>(arg: T, type_out: &mut u8, value_out: &mut u64)
            where
                T: Sized,
            {
                if mem::size_of::<T>() == 0 {
                    *type_out = 0;
                    *value_out = 0;
                } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<bool>() {
                    *type_out = TRACE_VALUE_TYPE_BOOL;
                    *value_out = if unsafe { mem::transmute::<T, bool>(arg) } {
                        1
                    } else {
                        0
                    };
                } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<i32>() {
                    *type_out = TRACE_VALUE_TYPE_INT;
                    *value_out = unsafe { mem::transmute::<T, i32>(arg) } as u64;
                } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<i64>() {
                    *type_out = TRACE_VALUE_TYPE_INT;
                    *value_out = unsafe { mem::transmute::<T, i64>(arg) } as u64;
                }  else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<isize>() {
                  *type_out = TRACE_VALUE_TYPE_INT;
                  *value_out = unsafe { mem::transmute::<T, isize>(arg) } as u64;
                }
                else {
                    *type_out = TRACE_VALUE_TYPE_UINT;
                    *value_out = unsafe { mem::transmute::<T, u64>(arg) };
                }
            }

            pub fn set_trace_value_double(arg: f64, type_out: &mut u8, value_out: &mut u64) {
                *type_out = TRACE_VALUE_TYPE_DOUBLE;
                *value_out = 0;
                unsafe {
                    ptr::copy_nonoverlapping(
                        &arg as *const f64 as *const u8,
                        value_out as *mut u64 as *mut u8,
                        mem::size_of::<f64>(),
                    );
                }
            }

            pub fn set_trace_value_string(arg: *const i8, type_out: &mut u8, value_out: &mut u64) {
                *type_out = TRACE_VALUE_TYPE_STRING;
                *value_out = 0;
                unsafe {
                    *value_out = arg as u64;
                }
            }

            pub fn add_trace_event(
                phase: char,
                category_group_enabled: *const u8,
                name: *const i8,
                scope: *const i8,
                id: u64,
                bind_id: u64,
                flags: u32,
                platform: &Platform,
            ) -> u64 {
                trace_event_api_add_trace_event!()(
                    phase,
                    category_group_enabled,
                    name,
                    scope,
                    id,
                    bind_id,
                    0,
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    flags,
                    platform
                )
            }

            pub fn add_trace_event<ARG1_TYPE: Sized>(
                phase: char,
                category_group_enabled: *const u8,
                name: *const i8,
                scope: *const i8,
                id: u64,
                bind_id: u64,
                flags: u32,
                platform: &Platform,
                arg1_name: *const i8,
                arg1_val: ARG1_TYPE,
            ) -> u64 {
                let num_args = 1;
                let mut arg_type: u8 = 0;
                let mut arg_value: u64 = 0;

                if core::any::TypeId::of::<ARG1_TYPE>() == core::any::TypeId::of::<f64>() {
                    set_trace_value_double(unsafe { mem::transmute_copy(&arg1_val) }, &mut arg_type, &mut arg_value);
                } else if core::any::TypeId::of::<ARG1_TYPE>() == core::any::TypeId::of::<*const i8>() {
                    set_trace_value_string(unsafe { mem::transmute_copy(&arg1_val) }, &mut arg_type, &mut arg_value);
                }
                 else {
                    set_trace_value(arg1_val, &mut arg_type, &mut arg_value);
                }

                trace_event_api_add_trace_event!()(
                    phase,
                    category_group_enabled,
                    name,
                    scope,
                    id,
                    bind_id,
                    num_args,
                    &arg1_name,
                    &arg_type,
                    &arg_value,
                    flags,
                    platform
                )
            }

            pub fn add_trace_event<ARG1_TYPE: Sized, ARG2_TYPE: Sized>(
                phase: char,
                category_group_enabled: *const u8,
                name: *const i8,
                scope: *const i8,
                id: u64,
                bind_id: u64,
                flags: u32,
                platform: &Platform,
                arg1_name: *const i8,
                arg1_val: ARG1_TYPE,
                arg2_name: *const i8,
                arg2_val: ARG2_TYPE,
            ) -> u64 {
                let num_args = 2;
                let arg_names: [*const i8; 2] = [arg1_name, arg2_name];
                let mut arg_types: [u8; 2] = [0, 0];
                let mut arg_values: [u64; 2] = [0, 0];

               if core::any::TypeId::of::<ARG1_TYPE>() == core::any::TypeId::of::<f64>() {
                    set_trace_value_double(unsafe { mem::transmute_copy(&arg1_val) }, &mut arg_types[0], &mut arg_values[0]);
                } else if core::any::TypeId::of::<ARG1_TYPE>() == core::any::TypeId::of::<*const i8>() {
                    set_trace_value_string(unsafe { mem::transmute_copy(&arg1_val) }, &mut arg_types[0], &mut arg_values[0]);
                }
                else {
                    set_trace_value(arg1_val, &mut arg_types[0], &mut arg_values[0]);
                }

                if core::any::TypeId::of::<ARG2_TYPE>() == core::any::TypeId::of::<f64>() {
                    set_trace_value_double(unsafe { mem::transmute_copy(&arg2_val) }, &mut arg_types[1], &mut arg_values[1]);
                } else if core::any::TypeId::of::<ARG2_TYPE>() == core::any::TypeId::of::<*const i8>() {
                    set_trace_value_string(unsafe { mem::transmute_copy(&arg2_val) }, &mut arg_types[1], &mut arg_values[1]);
                }
                else {
                    set_trace_value(arg2_val, &mut arg_types[1], &mut arg_values[1]);
                }

                trace_event_api_add_trace_event!()(
                    phase,
                    category_group_enabled,
                    name,
                    scope,
                    id,
                    bind_id,
                    num_args,
                    arg_names.as_ptr(),
                    arg_types.as_ptr(),
                    arg_values.as_ptr(),
                    flags,
                    platform
                )
            }
        } // namespace internal
    }     // namespace cppgc
}

#[cfg(not(feature = "cppgc_is_standalone"))]
pub use trace_event_wrapper::*;

#[cfg(feature = "cppgc_is_standalone")]
pub use trace_event_standalone::*;
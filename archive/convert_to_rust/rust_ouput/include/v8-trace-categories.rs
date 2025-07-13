// Converted from V8 C++ source files:
// Header: v8-trace-categories.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
pub mod perfetto {
    pub mod internal {
        pub struct TrackEventCategoryRegistry {}
    }
    pub mod tracing {
        pub struct TrackEvent {}
    }
}

#[cfg(feature = "V8_USE_PERFETTO")]
pub mod v8 {
    use crate::perfetto::internal::TrackEventCategoryRegistry;
    use crate::V8_EXPORT;

    // Returns the perfetto TrackEventCategoryRegistry for v8 tracing categories.
    #[no_mangle]
    pub extern "C" fn GetTrackEventCategoryRegistry() -> *const TrackEventCategoryRegistry {
        lazy_static::lazy_static! {
            static ref REGISTRY: TrackEventCategoryRegistry = TrackEventCategoryRegistry {};
        }
        &(*REGISTRY) as *const TrackEventCategoryRegistry
    }
}

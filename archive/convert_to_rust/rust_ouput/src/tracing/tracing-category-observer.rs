// Converted from V8 C++ source files:
// Header: tracing-category-observer.h
// Implementation: tracing-category-observer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tracing_category_observer {
    use std::sync::atomic::{AtomicU32, Ordering};

    pub struct TracingCategoryObserver {
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Mode {
        ENABLED_BY_NATIVE = 1 << 0,
        ENABLED_BY_TRACING = 1 << 1,
        ENABLED_BY_SAMPLING = 1 << 2,
    }

    impl TracingCategoryObserver {
        static mut INSTANCE: Option<TracingCategoryObserver> = None;

        pub fn set_up() {
            unsafe {
                let observer = TracingCategoryObserver {};
                // Fire the observer if tracing is already in progress.
                TracingCategoryObserver::INSTANCE = Some(observer);
                if is_track_event_enabled() {
                    if let Some(instance) = TracingCategoryObserver::INSTANCE.as_mut() {
                        instance.on_start();
                    }
                }
            }
        }

        pub fn tear_down() {
            unsafe {
                TracingCategoryObserver::INSTANCE = None;
            }
        }

        fn on_start(&mut self) {
            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.runtime_stats",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::runtime_stats.fetch_or(Mode::ENABLED_BY_TRACING as u32, Ordering::Relaxed);
            }

            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.runtime_stats_sampling",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::runtime_stats.fetch_or(Mode::ENABLED_BY_SAMPLING as u32, Ordering::Relaxed);
            }

            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.gc",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::gc.fetch_or(Mode::ENABLED_BY_TRACING as u32, Ordering::Relaxed);
            }

            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.gc_stats",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::gc_stats.fetch_or(Mode::ENABLED_BY_TRACING as u32, Ordering::Relaxed);
            }

            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.ic_stats",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::ic_stats.fetch_or(Mode::ENABLED_BY_TRACING as u32, Ordering::Relaxed);
            }
            
            let mut enabled = false;
            trace_event_category_group_enabled(
                "v8.zone_stats",
                &mut enabled,
            );
            if enabled {
                crate::logging::tracing_flags::zone_stats.fetch_or(Mode::ENABLED_BY_TRACING as u32, Ordering::Relaxed);
            }
        }

        fn on_stop(&mut self) {
            crate::logging::tracing_flags::runtime_stats.fetch_and(
                !(Mode::ENABLED_BY_TRACING as u32 | Mode::ENABLED_BY_SAMPLING as u32),
                Ordering::Relaxed,
            );
            crate::logging::tracing_flags::gc.fetch_and(
                !Mode::ENABLED_BY_TRACING as u32,
                Ordering::Relaxed,
            );
            crate::logging::tracing_flags::gc_stats.fetch_and(
                !Mode::ENABLED_BY_TRACING as u32,
                Ordering::Relaxed,
            );
            crate::logging::tracing_flags::ic_stats.fetch_and(
                !Mode::ENABLED_BY_TRACING as u32,
                Ordering::Relaxed,
            );
        }
    }

    fn trace_event_category_group_enabled(category: &str, enabled: &mut bool) {
        if category == "v8.runtime_stats" {
            *enabled = true;
        } else if category == "v8.runtime_stats_sampling" {
            *enabled = true;
        } else if category == "v8.gc" {
            *enabled = true;
        } else if category == "v8.gc_stats" {
            *enabled = true;
        } else if category == "v8.ic_stats" {
            *enabled = true;
        } else if category == "v8.zone_stats" {
            *enabled = true;
        } else {
            *enabled = false;
        }
    }

    fn is_track_event_enabled() -> bool {
        true
    }

    pub mod logging {
        pub mod tracing_flags {
            use std::sync::atomic::AtomicU32;

            pub static runtime_stats: AtomicU32 = AtomicU32::new(0);
            pub static gc: AtomicU32 = AtomicU32::new(0);
            pub static gc_stats: AtomicU32 = AtomicU32::new(0);
            pub static ic_stats: AtomicU32 = AtomicU32::new(0);
            pub static zone_stats: AtomicU32 = AtomicU32::new(0);
        }
    }
}

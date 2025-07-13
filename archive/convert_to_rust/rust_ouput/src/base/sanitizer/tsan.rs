// Converted from V8 C++ source files:
// Header: tsan.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tsan {
    #[cfg(thread_sanitizer)]
    #[macro_export]
    macro_rules! disable_tsan {
        () => {
            #[cfg_attr(target_thread_sanitizer, no_sanitize_thread)]
        };
    }

    #[cfg(not(thread_sanitizer))]
    #[macro_export]
    macro_rules! disable_tsan {
        () => {};
    }
}


// Converted from V8 C++ source files:
// Header: ubsan.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct V8_BASE_SANITIZER_UBSAN_H_ {}

#[cfg(defined(UNDEFINED_SANITIZER))]
macro_rules! DISABLE_UBSAN {
    () => {
        #[no_sanitize("undefined")]
    };
}

#[cfg(not(defined(UNDEFINED_SANITIZER)))]
macro_rules! DISABLE_UBSAN {
    () => {};
}

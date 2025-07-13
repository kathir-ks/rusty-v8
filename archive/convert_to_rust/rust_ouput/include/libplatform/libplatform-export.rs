// Converted from V8 C++ source files:
// Header: libplatform-export.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! V8_PLATFORM_EXPORT {
    () => {
        #[cfg(feature = "v8_platform_shared")]
        #[link(name = "v8_platform")]
        extern {}
    };
}

#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! V8_PLATFORM_EXPORT {
    () => {};
}

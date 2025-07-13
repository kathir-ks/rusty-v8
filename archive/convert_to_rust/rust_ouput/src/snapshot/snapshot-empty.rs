// Converted from V8 C++ source files:
// Header: N/A
// Implementation: snapshot-empty.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod v8 {
    pub struct StartupData {
        data: *const u8,
        raw_size: i32,
    }
    impl StartupData {
        pub fn new(data: *const u8, raw_size: i32) -> Self {
            StartupData { data, raw_size }
        }

        pub fn empty() -> Self {
            StartupData {
                data: std::ptr::null(),
                raw_size: 0,
            }
        }
    }
}

pub mod internal {
    use super::v8;

    #[cfg(feature = "V8_USE_EXTERNAL_STARTUP_DATA")]
    pub fn set_natives_from_file(data: &mut v8::StartupData) {
        panic!("UNREACHABLE");
    }

    #[cfg(feature = "V8_USE_EXTERNAL_STARTUP_DATA")]
    pub fn set_snapshot_from_file(data: &mut v8::StartupData) {
        panic!("UNREACHABLE");
    }

    #[cfg(feature = "V8_USE_EXTERNAL_STARTUP_DATA")]
    pub fn read_natives() {}

    #[cfg(feature = "V8_USE_EXTERNAL_STARTUP_DATA")]
    pub fn dispose_natives() {}

    pub struct Snapshot {}

    impl Snapshot {
        pub fn default_snapshot_blob() -> *const v8::StartupData {
            std::ptr::null()
        }

        pub fn should_verify_checksum(data: *const v8::StartupData) -> bool {
            false
        }
    }
}

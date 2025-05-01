// Copyright 2006-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Used for building without snapshots.

mod snapshot {
    pub struct StartupData {
        data: *const i8,
        raw_size: i32,
    }

    impl StartupData {
        pub fn new(data: *const i8, raw_size: i32) -> Self {
            StartupData { data, raw_size }
        }
    }

    pub struct Snapshot {}

    impl Snapshot {
        pub fn default_snapshot_blob() -> *const StartupData {
            std::ptr::null()
        }

        pub fn should_verify_checksum(_data: *const StartupData) -> bool {
            false
        }
    }
}

mod internal {
    pub use super::snapshot::*;

    #[cfg(feature = "v8_use_external_startup_data")]
    pub mod external_startup_data {
        use super::StartupData;

        // Dummy implementations of Set*FromFile(..) APIs.
        //
        // These are meant for use with snapshot-external.rs. Should this file
        // be compiled with those options we just supply these dummy implementations
        // below. This happens when compiling the mksnapshot utility.
        pub fn set_natives_from_file(_data: &mut StartupData) {
            panic!("UNREACHABLE");
        }
        pub fn set_snapshot_from_file(_data: &mut StartupData) {
            panic!("UNREACHABLE");
        }
        pub fn read_natives() {}
        pub fn dispose_natives() {}
    }
}

mod v8 {
    pub use super::internal::*;
}
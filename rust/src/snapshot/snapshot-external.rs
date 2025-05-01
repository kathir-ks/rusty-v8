// Copyright 2006-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Used for building with external snapshots.

use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
//use crate::base::platform::mutex; // Assuming similar functionality exists
//use crate::flags::flags; // Assuming similar functionality exists
//use crate::init::v8; // Assuming similar functionality exists
//use crate::snapshot::snapshot_source_sink; // Assuming similar functionality exists
//use crate::snapshot::snapshot; // Assuming similar functionality exists

#[cfg(not(feature = "v8_use_external_startup_data"))]
compile_error!("snapshot-external.rs is used only for the external snapshot build.");

lazy_static! {
    static ref EXTERNAL_STARTUP_DATA_MUTEX: Mutex<()> = Mutex::new(());
    static ref EXTERNAL_STARTUP_BLOB: Mutex<StartupData> = Mutex::new(StartupData { data: std::ptr::null(), raw_size: 0 });
}

#[cfg(target_os = "android")]
lazy_static! {
    static ref EXTERNAL_STARTUP_CHECKSUM_VERIFIED: Mutex<bool> = Mutex::new(false);
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct StartupData {
    pub data: *const u8,
    pub raw_size: usize,
}

unsafe impl Send for StartupData {}
unsafe impl Sync for StartupData {}

pub fn set_snapshot_from_file(snapshot_blob: &StartupData) {
    let lock_guard: MutexGuard<'_, ()> = EXTERNAL_STARTUP_DATA_MUTEX.lock().unwrap();
    assert!(!snapshot_blob.data.is_null());
    assert!(snapshot_blob.raw_size > 0);

    let mut blob = EXTERNAL_STARTUP_BLOB.lock().unwrap();
    assert!(blob.data.is_null());

    // Assuming `Snapshot::snapshot_is_valid` is a function that determines
    // whether `snapshot_blob` represents a valid snapshot.
    // assert!(Snapshot::snapshot_is_valid(snapshot_blob)); // Assuming similar functionality exists
    *blob = *snapshot_blob;
    drop(lock_guard);

    #[cfg(target_os = "android")]
    {
        let mut checksum_verified = EXTERNAL_STARTUP_CHECKSUM_VERIFIED.lock().unwrap();
        *checksum_verified = false;
    }
}

pub struct Snapshot {}

impl Snapshot {
    pub fn should_verify_checksum(data: &StartupData) -> bool {
        #[cfg(target_os = "android")]
        {
            let lock_guard: MutexGuard<'_, ()> = EXTERNAL_STARTUP_DATA_MUTEX.lock().unwrap();
            let blob = EXTERNAL_STARTUP_BLOB.lock().unwrap();

            let verify = if data as *const StartupData == &*blob as *const StartupData {
                let mut checksum_verified = EXTERNAL_STARTUP_CHECKSUM_VERIFIED.lock().unwrap();
                if *checksum_verified {
                    false
                } else {
                    *checksum_verified = true;
                   // flags::v8_flags.verify_snapshot_checksum // Assuming similar functionality exists
                   true
                }
            } else {
              //flags::v8_flags.verify_snapshot_checksum // Assuming similar functionality exists
              true
            };
            drop(lock_guard);

           return verify
        }
        #[cfg(not(target_os = "android"))]
        {
          // flags::v8_flags.verify_snapshot_checksum // Assuming similar functionality exists
          true
        }
    }
    
    pub fn default_snapshot_blob() -> StartupData {
        let _lock_guard: MutexGuard<'_, ()> = EXTERNAL_STARTUP_DATA_MUTEX.lock().unwrap();
        let blob = EXTERNAL_STARTUP_BLOB.lock().unwrap();
        *blob
    }
}
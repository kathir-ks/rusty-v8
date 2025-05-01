// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/snapshot-utils.h (Rust module definition)
pub mod snapshot_utils {
    /// Calculates a checksum for the provided byte slice.
    pub fn checksum(payload: &[u8]) -> u32 {
        #[cfg(feature = "v8_use_zlib")]
        {
            checksum_zlib(payload)
        }
        #[cfg(not(feature = "v8_use_zlib"))]
        {
            checksum_fletcher32(payload)
        }
    }

    #[cfg(feature = "v8_use_zlib")]
    fn checksum_zlib(payload: &[u8]) -> u32 {
        // Priming the adler32 call is not necessary in Rust's zlib implementation
        let mut adler = adler32::Adler32::new();
        adler.update(payload);
        adler.finish()
    }

    #[cfg(not(feature = "v8_use_zlib"))]
    fn checksum_fletcher32(payload: &[u8]) -> u32 {
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;

        for &data in payload {
            sum1 = (sum1 + data as u32) % 65535;
            sum2 = (sum2 + sum1) % 65535;
        }

        (sum2 << 16) | sum1
    }
}

#[cfg(feature = "v8_use_zlib")]
extern crate adler32;
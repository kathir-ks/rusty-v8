// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module provides the V8 version string.

mod v8_version_string {
    use super::v8_version;

    #[cfg(feature = "v8_is_candidate_version")]
    const V8_CANDIDATE_STRING: &str = " (candidate)";
    #[cfg(not(feature = "v8_is_candidate_version"))]
    const V8_CANDIDATE_STRING: &str = "";

    #[cfg(feature = "v8_embedder_string")]
    const V8_EMBEDDER_STRING: &str = env!("V8_EMBEDDER_STRING"); // Assuming this is an environment variable or build config.
    #[cfg(not(feature = "v8_embedder_string"))]
    const V8_EMBEDDER_STRING: &str = "";

    macro_rules! v8_s {
        ($x:expr) => {
            stringify!($x)
        };
    }

    #[cfg(feature = "v8_patch_level")]
    pub const V8_VERSION_STRING: &str = {
        use v8_version::*;
        concat!(
            v8_s!(V8_MAJOR_VERSION),
            ".",
            v8_s!(V8_MINOR_VERSION),
            ".",
            v8_s!(V8_BUILD_NUMBER),
            ".",
            v8_s!(V8_PATCH_LEVEL),
            V8_EMBEDDER_STRING,
            V8_CANDIDATE_STRING
        )
    };

    #[cfg(not(feature = "v8_patch_level"))]
    pub const V8_VERSION_STRING: &str = {
        use v8_version::*;
        concat!(
            v8_s!(V8_MAJOR_VERSION),
            ".",
            v8_s!(V8_MINOR_VERSION),
            ".",
            v8_s!(V8_BUILD_NUMBER),
            V8_EMBEDDER_STRING,
            V8_CANDIDATE_STRING
        )
    };
}

pub use v8_version_string::V8_VERSION_STRING;

mod v8_version {
    // Placeholder for v8-version.h content.
    // Actual values should be defined based on the original v8-version.h content
    pub const V8_MAJOR_VERSION: i32 = 1;
    pub const V8_MINOR_VERSION: i32 = 0;
    pub const V8_BUILD_NUMBER: i32 = 0;

    // Optional patch level. Enable the feature "v8_patch_level" to use this
    #[cfg(feature = "v8_patch_level")]
    pub const V8_PATCH_LEVEL: i32 = 0;
}
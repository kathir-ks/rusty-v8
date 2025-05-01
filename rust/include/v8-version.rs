// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines the version number for the current version.
/// These constants are used by some of the tool scripts and the build system
/// so their names cannot be changed without changing the scripts.
pub mod v8_version {
    /// Major version number.
    pub const V8_MAJOR_VERSION: u32 = 13;
    /// Minor version number.
    pub const V8_MINOR_VERSION: u32 = 6;
    /// Build number.
    pub const V8_BUILD_NUMBER: u32 = 0;
    /// Patch level.
    pub const V8_PATCH_LEVEL: u32 = 0;

    /// Indicates if this is a candidate version (1) or not (0).
    pub const V8_IS_CANDIDATE_VERSION: bool = true;
}
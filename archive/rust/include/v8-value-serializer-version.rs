// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Compile-time constants.
///
/// This module provides access to information about the value serializer at
/// compile time, without declaring or defining any symbols that require linking
/// to V8.

pub mod value_serializer_version {
    /// Returns the current value serializer format version.
    pub const fn current_value_serializer_format_version() -> u32 {
        15
    }
}
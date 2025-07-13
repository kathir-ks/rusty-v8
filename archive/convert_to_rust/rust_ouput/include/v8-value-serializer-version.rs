// Converted from V8 C++ source files:
// Header: v8-value-serializer-version.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/**
 * Compile-time constants.
 *
 * This header provides access to information about the value serializer at
 * compile time, without declaring or defining any symbols that require linking
 * to V8.
 */

pub mod value_serializer_version {

    pub const fn current_value_serializer_format_version() -> u32 {
        15
    }

}

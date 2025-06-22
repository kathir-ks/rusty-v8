// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a Rust equivalent of the cppgc::SourceLocation header.
// It relies on the `v8` crate to provide the v8::SourceLocation type.

pub mod source_location {
    // Assuming v8::SourceLocation is directly available through the v8 crate.
    // If not, a wrapper might be necessary.
    pub use v8::SourceLocation;
}

pub use source_location::SourceLocation;
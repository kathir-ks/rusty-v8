// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This Rust module corresponds to the C++ header file v8-inspector-protocol.h.
// It includes declarations that mirror the C++ includes in the original header.

// The original C++ header includes several other headers from the inspector folder.
// Here we are creating a module that declares and potentially exports the
// the submodules corresponding to the included C++ headers.

// Note: This translation assumes that Debugger, Runtime, and Schema are modules defined elsewhere
// and contain the necessary Rust translations.
// It also assumes that v8_inspector is a crate defined elsewhere.
// Since there is no implementation of any class in the C++ header,
// only modules that reference the original includes are declared here.

pub mod debugger;
pub mod runtime;
pub mod schema;

// Assuming `v8_inspector` is a Rust crate that provides necessary functionality
// mimicking the original `v8-inspector.h`.

pub use v8_inspector; // Use the v8_inspector crate

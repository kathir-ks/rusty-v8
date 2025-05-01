// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! V8 API Reference Guide
//!
//! V8 is Google's open source JavaScript engine.
//!
//! This set of documents provides reference material generated from the
//! V8 header files in the include/ subdirectory.
//!
//! For other documentation see https://v8.dev/.

// The original C++ header file includes many other header files.
// In Rust, we would typically define separate modules for each of
// these functionalities.  This top-level module then re-exports
// those sub-modules to create a single cohesive API.
//
// Note: The actual implementations of these modules would require
// substantial reverse engineering or access to internal V8 details.
// This conversion focuses on creating the module structure and
// public API surface.

pub mod array_buffer;
pub mod container;
pub mod context;
pub mod data;
pub mod date;
pub mod debug;
pub mod exception;
pub mod extension;
pub mod external;
pub mod function;
pub mod initialization;
pub mod internal;
pub mod isolate;
pub mod json;
pub mod local_handle;
pub mod locker;
pub mod maybe;
pub mod memory_span;
pub mod message;
pub mod microtask_queue;
pub mod microtask;
pub mod object;
pub mod persistent_handle;
pub mod primitive_object;
pub mod primitive;
pub mod promise;
pub mod proxy;
pub mod regexp;
pub mod script;
pub mod snapshot;
pub mod statistics;
pub mod template;
pub mod traced_handle;
pub mod typed_array;
pub mod unwinder;
pub mod value_serializer;
pub mod value;
pub mod version;
pub mod wasm;
pub mod config;

pub use array_buffer::*;
pub use container::*;
pub use context::*;
pub use data::*;
pub use date::*;
pub use debug::*;
pub use exception::*;
pub use extension::*;
pub use external::*;
pub use function::*;
pub use initialization::*;
pub use internal::*;
pub use isolate::*;
pub use json::*;
pub use local_handle::*;
pub use locker::*;
pub use maybe::*;
pub use memory_span::*;
pub use message::*;
pub use microtask_queue::*;
pub use microtask::*;
pub use object::*;
pub use persistent_handle::*;
pub use primitive_object::*;
pub use primitive::*;
pub use promise::*;
pub use proxy::*;
pub use regexp::*;
pub use script::*;
pub use snapshot::*;
pub use statistics::*;
pub use template::*;
pub use traced_handle::*;
pub use typed_array::*;
pub use unwinder::*;
pub use value_serializer::*;
pub use value::*;
pub use version::*;
pub use wasm::*;
pub use config::*;

/// The v8 JavaScript engine.
pub mod v8 {
    /// Represents the platform used by V8.
    pub trait Platform {}

    // This is a placeholder.  The actual implementation would
    // need to interact with the underlying V8 engine.
    pub struct DefaultPlatform;

    impl Platform for DefaultPlatform {}

    // Ideally this would return Result<Box<dyn Platform>, V8Error>, but
    // V8Error isn't defined in this context.
    pub fn new_default_platform() -> Box<dyn Platform> {
        Box::new(DefaultPlatform)
    }
}

// V8_* macros would be converted to const or macro_rules!
// Example:
// const V8_MAJOR_VERSION: i32 = 9;
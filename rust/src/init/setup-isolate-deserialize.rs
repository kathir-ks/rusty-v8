// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/logging.h is replaced with Rust's built-in logging
use log::debug;

// src/execution/isolate.h
pub struct Isolate {
    // Opaque structure representing the isolate.
}

impl Isolate {
    pub fn new() -> Self {
        Isolate {}
    }
}

// src/init/setup-isolate.h
pub trait SetupIsolateDelegate {
    fn setup_heap(&mut self, isolate: &mut Isolate, create_heap_objects: bool) -> bool;
    fn setup_builtins(&mut self, isolate: &mut Isolate, compile_builtins: bool);
}

pub struct DefaultSetupIsolateDelegate {}

impl DefaultSetupIsolateDelegate {
    pub fn new() -> Self {
        DefaultSetupIsolateDelegate {}
    }
}

impl SetupIsolateDelegate for DefaultSetupIsolateDelegate {
    fn setup_heap(&mut self, _isolate: &mut Isolate, create_heap_objects: bool) -> bool {
        // No actual work to be done; heap will be deserialized from the snapshot.
        debug_assert!(!create_heap_objects, "Heap setup supported only in mksnapshot");
        true
    }

    fn setup_builtins(&mut self, _isolate: &mut Isolate, compile_builtins: bool) {
        // No actual work to be done; builtins will be deserialized from the snapshot.
        debug_assert!(!compile_builtins, "Builtin compilation supported only in mksnapshot");
    }
}
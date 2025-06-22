// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {

    pub trait SetupIsolateDelegateTrait {
        fn setup_heap(&mut self, isolate: &mut Isolate, create_heap_objects: bool) -> bool;
        fn setup_builtins(&mut self, isolate: &mut Isolate, compile_builtins: bool);
    }

    pub struct SetupIsolateDelegate {}

    impl SetupIsolateDelegate {
        pub fn new() -> Self {
            SetupIsolateDelegate {}
        }
    }

    impl SetupIsolateDelegateTrait for SetupIsolateDelegate {
        fn setup_heap(&mut self, _isolate: &mut Isolate, _create_heap_objects: bool) -> bool {
            // Placeholder implementation
            false
        }

        fn setup_builtins(&mut self, _isolate: &mut Isolate, _compile_builtins: bool) {
            // Placeholder implementation
        }
    }

    impl SetupIsolateDelegate {
        fn setup_builtins_internal(_isolate: &mut Isolate) {
            // Placeholder implementation
        }

        fn add_builtin(_builtins: &mut Builtins, _builtin: Builtin, _code: Tagged<Code>) {
            // Placeholder implementation
        }

        fn populate_with_placeholders(_isolate: &mut Isolate) {
            // Placeholder implementation
        }

        fn replace_placeholders(_isolate: &mut Isolate) {
            // Placeholder implementation
        }

        fn setup_heap_internal(_isolate: &mut Isolate) -> bool {
            // Placeholder implementation
            false
        }
    }

    // Dummy definitions to allow compilation. These would be replaced with
    // the actual definitions from the V8 codebase during a full port.
    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    pub struct Builtins {}

    impl Builtins {
        pub fn new() -> Self {
            Builtins {}
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Builtin {
        Default
    }

    pub struct Tagged<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn new() -> Self {
            Tagged {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct Code {}

    impl Code {
        pub fn new() -> Self {
            Code {}
        }
    }

    pub struct Heap {}

    impl Heap {
        pub fn new() -> Self {
            Heap {}
        }
    }
}
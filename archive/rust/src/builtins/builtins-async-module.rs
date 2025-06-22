// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Implement equivalents for these includes
// #include "src/builtins/builtins-utils-inl.h"
// #include "src/objects/module-inl.h"
// #include "src/objects/objects-inl.h"

// Placeholder for BuiltinArguments, Isolate, Handle, etc.
mod v8 {
    pub mod internal {
        pub struct BuiltinArguments {}
        impl BuiltinArguments {
            pub fn length(&self) -> usize {
                0 // Placeholder
            }
            pub fn at(&self, _index: usize) -> Object {
                Object {} // Placeholder
            }
        }

        pub struct Isolate {}
        impl Isolate {
            pub fn context(&self) -> &Context {
                &Context{} // Placeholder
            }
            pub fn is_execution_terminating(&self) -> bool {
                false // Placeholder
            }
        }

        pub struct Handle<T> {
            pub value: T,
        }

        impl<T> Handle<T> {
            pub fn new(value: T) -> Self {
                Handle { value }
            }
        }

        pub struct DirectHandle<T> {
            pub value: T,
        }

        impl<T> DirectHandle<T> {
            pub fn new(value: T) -> Self {
                DirectHandle { value }
            }
        }

        pub struct Context {}
        impl Context {
            pub fn get(&self, _slot: SourceTextModuleExecuteAsyncModuleContextSlots) -> SourceTextModule {
                SourceTextModule {} // Placeholder
            }
        }

        #[derive(Copy, Clone)]
        pub enum SourceTextModuleExecuteAsyncModuleContextSlots {
            kModule
        }

        pub struct SourceTextModule {}
        impl SourceTextModule {
            pub fn async_module_execution_fulfilled(_isolate: &Isolate, _module: &Handle<SourceTextModule>) -> Result<(),()> {
                Ok(()) // Placeholder
            }

            pub fn async_module_execution_rejected(_isolate: &Isolate, _module: &DirectHandle<SourceTextModule>, _exception: &DirectHandle<Object>) {
                // Placeholder
            }
        }

        pub struct Object {}

        pub struct ReadOnlyRoots {}
        impl ReadOnlyRoots {
            pub fn exception(&self) -> Object {
                Object {} // Placeholder
            }
            pub fn undefined_value(&self) -> Object {
                Object {} // Placeholder
            }
        }
    }
}

use v8::internal::*;

mod builtins_async_module {
    use super::*;

    // Placeholder for v8_flags
    pub struct V8Flags {
        pub strict_termination_checks: bool,
    }

    impl V8Flags {
        pub const fn new() -> Self {
            V8Flags {
                strict_termination_checks: false,
            }
        }
    }

    const V8_FLAGS: V8Flags = V8Flags::new();

    pub fn call_async_module_fulfilled(isolate: &mut Isolate, args: &BuiltinArguments) -> Object {
        // HandleScope handle_scope(isolate); // Handled implicitly by Rust lifetimes
        let module = Handle::new(isolate.context().get(SourceTextModuleExecuteAsyncModuleContextSlots::kModule));
        if SourceTextModule::async_module_execution_fulfilled(isolate, &module).is_err() {
            // The evaluation of async module can not throwing a JavaScript observable
            // exception.
            if V8_FLAGS.strict_termination_checks {
                assert!(isolate.is_execution_terminating());
            }
            ReadOnlyRoots{}.exception()
        } else {
            ReadOnlyRoots{}.undefined_value()
        }
    }

    pub fn call_async_module_rejected(isolate: &mut Isolate, args: &BuiltinArguments) -> Object {
        // HandleScope handle_scope(isolate); // Handled implicitly by Rust lifetimes
        let module = DirectHandle::new(isolate.context().get(SourceTextModuleExecuteAsyncModuleContextSlots::kModule));

        // Arguments should be an exception object, with receiver.
        assert_eq!(args.length(), 2);
        let exception = DirectHandle::new(args.at(1));
        SourceTextModule::async_module_execution_rejected(isolate, &module, &exception);
        ReadOnlyRoots{}.undefined_value()
    }
}

pub use builtins_async_module::*;
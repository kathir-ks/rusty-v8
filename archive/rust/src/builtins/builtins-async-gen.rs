// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod async_gen {
    use std::rc::Rc;

    use crate::builtins::promise_gen::PromiseBuiltinsAssembler;
    use crate::objects::js_generator::JSGeneratorObject;
    //use crate::compiler::code_assembler_state::CodeAssemblerState;
    //use crate::objects::js_function::JSFunction;
    //use crate::objects::js_promise::JSPromise;
    //use crate::objects::object::Object;
    //use crate::objects::contexts::Context;
    //use crate::objects::contexts::NativeContext;
    //use crate::roots::RootIndex;
    //use crate::types::JSAny;
    //use crate::types::Boolean;

    pub struct AsyncBuiltinsAssembler {
        promise_assembler: PromiseBuiltinsAssembler,
    }

    impl AsyncBuiltinsAssembler {
        /// Constructs a new `AsyncBuiltinsAssembler`.
        //pub fn new(state: *mut CodeAssemblerState) -> Self {
        //    let promise_assembler = PromiseBuiltinsAssembler::new(state);
        //    AsyncBuiltinsAssembler {
        //        promise_assembler,
        //    }
        //}

        // NOTE: Since CodeAssemblerState is not fully defined,
        // We use dummy implementations instead

        pub fn new() -> Self {
            AsyncBuiltinsAssembler {
                promise_assembler: PromiseBuiltinsAssembler::new(),
            }
        }

        // Perform steps to resume generator after `value` is resolved.
        // `on_reject` is the SharedFunctioninfo instance used to create the reject
        // closure. `on_resolve` is the SharedFunctioninfo instance used to create the
        // resolve closure. Returns the Promise-wrapped `value`.
        //pub fn await_value<F>(
        //    &mut self,
        //    context: *mut Context,
        //    generator: *mut JSGeneratorObject,
        //    value: *mut JSAny,
        //    outer_promise: *mut JSPromise,
        //    create_closures: F,
        //) -> *mut Object
        //where
        //    F: Fn(*mut Context, *mut NativeContext) -> (
        //        *mut JSFunction,
        //        *mut JSFunction,
        //    ),
        //{
        //    // Dummy implementation
        //    std::ptr::null_mut()
        //}
        //
        //pub fn await_with_root_index(
        //    &mut self,
        //    context: *mut Context,
        //    generator: *mut JSGeneratorObject,
        //    value: *mut JSAny,
        //    outer_promise: *mut JSPromise,
        //    on_resolve_sfi: RootIndex,
        //    on_reject_sfi: RootIndex,
        //) -> *mut Object {
        //    // Dummy implementation
        //    std::ptr::null_mut()
        //}

        // TODO: Implement the Await methods.  These are difficult because they use
        // function pointers to C++ closures (CreateClosures).  Rust closures are
        // different, and it's difficult to pass them to C++ without significant
        // re-architecting.

        // Return a new built-in function object as defined in
        // Async Iterator Value Unwrap Functions
        //pub fn create_unwrap_closure(
        //    &mut self,
        //    native_context: *mut NativeContext,
        //    done: *mut Boolean,
        //) -> *mut JSFunction {
        //    // Dummy implementation
        //    std::ptr::null_mut()
        //}

        // TODO: Implement CreateUnwrapClosure.  This requires NativeContext,
        // which is not fully defined.

        //private:
        //TNode<Context> AllocateAsyncIteratorValueUnwrapContext(
        //    TNode<NativeContext> native_context, TNode<Boolean> done);
        // TODO: Implement AllocateAsyncIteratorValueUnwrapContext.  This requires
        // TNode, NativeContext, and Boolean, which are not fully defined.
    }
}
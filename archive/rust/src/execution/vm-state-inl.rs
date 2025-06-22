// Copyright 2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/vm-state-inl.h

pub mod vm_state {
    use std::cell::Cell;
    use std::fmt;
    use std::time::Instant;

    //use crate::execution::isolate; // Assuming a corresponding Rust module exists
    //use crate::execution::simulator; // Assuming a corresponding Rust module exists
    //use crate::logging::log; // Assuming a corresponding Rust module exists
    //use crate::tracing::trace_event; // Assuming a corresponding Rust module exists

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StateTag {
        JS,
        GC,
        PARSER,
        BYTECODE_COMPILER,
        COMPILER,
        OTHER,
        EXTERNAL,
        ATOMICS_WAIT,
        IDLE,
        LOGGING,
    }

    impl fmt::Display for StateTag {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", state_to_string(*self))
        }
    }

    pub fn state_to_string(state: StateTag) -> &'static str {
        match state {
            StateTag::JS => "JS",
            StateTag::GC => "GC",
            StateTag::PARSER => "PARSER",
            StateTag::BYTECODE_COMPILER => "BYTECODE_COMPILER",
            StateTag::COMPILER => "COMPILER",
            StateTag::OTHER => "OTHER",
            StateTag::EXTERNAL => "EXTERNAL",
            StateTag::ATOMICS_WAIT => "ATOMICS_WAIT",
            StateTag::IDLE => "IDLE",
            StateTag::LOGGING => "LOGGING",
        }
    }

    pub struct Isolate {
        current_vm_state: Cell<StateTag>,
        external_callback_scope: Cell<Option<Box<ExternalCallbackScope>>>,
        //topmost_script_having_context: Cell<Option<*mut Script>>, //Needs a replacement type
        //counters: Box<Counters>,  //Needs a replacement type
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                current_vm_state: Cell::new(StateTag::IDLE),
                external_callback_scope: Cell::new(None),
                //topmost_script_having_context: Cell::new(None),
                //counters: Box::new(Counters::new()),
            }
        }
        pub fn current_vm_state(&self) -> StateTag {
            self.current_vm_state.get()
        }

        pub fn set_current_vm_state(&self, tag: StateTag) {
            self.current_vm_state.set(tag);
        }

        pub fn external_callback_scope(&self) -> Option<&ExternalCallbackScope> {
            self.external_callback_scope.borrow().as_deref()
        }

        pub fn set_external_callback_scope(&self, scope: Option<Box<ExternalCallbackScope>>) {
            self.external_callback_scope.replace(scope);
        }

        //pub fn topmost_script_having_context(&self) -> Option<*mut Script> {
        //   self.topmost_script_having_context.get()
        //}
        //
        //pub fn clear_topmost_script_having_context(&self) {
        //    self.topmost_script_having_context.set(None);
        //}
        //pub fn counters(&self) -> &Counters {
        //    &self.counters
        //}
    }

    pub struct VMState<'a, T: 'static> {
        isolate_: &'a Isolate,
        previous_tag_: StateTag,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<'a, T> VMState<'a, T> {
        pub fn new(isolate: &'a Isolate) -> Self {
            let previous_tag_ = isolate.current_vm_state();
            isolate.set_current_vm_state(vm_state_tag::<T>());
            VMState {
                isolate_: isolate,
                previous_tag_: previous_tag_,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'a, T> Drop for VMState<'a, T> {
        fn drop(&mut self) {
            self.isolate_.set_current_vm_state(self.previous_tag_);
        }
    }

    pub trait VMStateTag {
        const TAG: StateTag;
    }

    macro_rules! define_vm_state_tag {
        ($name:ident, $tag:expr) => {
            pub struct $name;
            impl VMStateTag for $name {
                const TAG: StateTag = $tag;
            }
        };
    }

    define_vm_state_tag!(JsVMState, StateTag::JS);
    define_vm_state_tag!(GcVMState, StateTag::GC);
    define_vm_state_tag!(ParserVMState, StateTag::PARSER);
    define_vm_state_tag!(BytecodeCompilerVMState, StateTag::BYTECODE_COMPILER);
    define_vm_state_tag!(CompilerVMState, StateTag::COMPILER);
    define_vm_state_tag!(OtherVMState, StateTag::OTHER);
    define_vm_state_tag!(ExternalVMState, StateTag::EXTERNAL);
    define_vm_state_tag!(AtomicsWaitVMState, StateTag::ATOMICS_WAIT);
    define_vm_state_tag!(IdleVMState, StateTag::IDLE);
    define_vm_state_tag!(LoggingVMState, StateTag::LOGGING);

    pub const fn vm_state_tag<T: VMStateTag>() -> StateTag {
        T::TAG
    }

    #[derive(Debug)]
    pub enum ExceptionContext {
        // Define the variants based on what v8::ExceptionContext represents
        Empty,
        Context(usize), // Placeholder
    }

    pub struct ExternalCallbackScope {
        callback_: usize, //Address
        callback_info_: *const std::ffi::c_void,
        previous_scope_: Option<Box<ExternalCallbackScope>>,
        vm_state_: VMState<'static, ExternalVMState>,
        exception_context_: ExceptionContext,
        pause_timed_histogram_scope_: PauseTimedHistogramScope,
        //js_stack_comparable_address_: usize, //Address,
    }

    impl ExternalCallbackScope {
        pub fn new(
            isolate: &'static Isolate,
            callback: usize, //Address
            exception_context: ExceptionContext,
            callback_info: *const std::ffi::c_void,
        ) -> Box<Self> {
            let previous_scope_ = isolate.external_callback_scope().map(|scope| {
                let raw_ptr = scope as *const ExternalCallbackScope;
                unsafe {
                  let copied = std::ptr::read(raw_ptr);
                  Box::new(copied)
                }
            });

            //let js_stack_comparable_address_ =
            //    simulator::register_js_stack_comparable_address(isolate);

            let mut scope = Box::new(ExternalCallbackScope {
                callback_: callback,
                callback_info_: callback_info,
                previous_scope_: previous_scope_,
                vm_state_: VMState::new(isolate),
                exception_context_: exception_context,
                pause_timed_histogram_scope_: PauseTimedHistogramScope::new(), //isolate.counters().execute()
                //js_stack_comparable_address_: js_stack_comparable_address_,
            });

            isolate.set_external_callback_scope(Some(scope));

            //isolate.clear_topmost_script_having_context();
            //trace_event::begin0("V8.ExternalCallback");
            scope
        }

        pub fn js_stack_comparable_address(&self) -> usize {
            //js_stack_comparable_address_
            self as *const Self as usize
        }
    }

    impl Drop for ExternalCallbackScope {
        fn drop(&mut self) {
            // This takes ownership of the Box, which then is dropped at the end of the scope.
            if let Some(prev_scope) = self.vm_state_.isolate_.external_callback_scope.take() {
                self.vm_state_.isolate_.set_external_callback_scope(self.previous_scope_.take());
            }
            //self.vm_state_.isolate_.clear_topmost_script_having_context();
            //trace_event::end0("V8.ExternalCallback");
            //simulator::unregister_js_stack_comparable_address(self.vm_state_.isolate_);
        }
    }

    struct PauseTimedHistogramScope {
        start: Instant,
    }

    impl PauseTimedHistogramScope {
        fn new() -> Self {
            PauseTimedHistogramScope {
                start: Instant::now(),
            }
        }
    }

    impl Drop for PauseTimedHistogramScope {
        fn drop(&mut self) {
            let _elapsed = self.start.elapsed();
            //TODO: Add implementation for tracing and histogram
            // Add histogram recording here, using 'elapsed'
        }
    }

    //struct Counters {}
    //impl Counters {
    //    fn new() -> Self {
    //        Counters {}
    //    }
    //    fn execute(&self) -> &Counters {
    //        self
    //    }
    //}
}
// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note 1: Any file that includes this one should include api-macros-undef.h
// at the bottom.  This is not directly translatable to Rust's module system.

// Note 2: This file is deliberately missing the include guards.  This is not
// directly translatable.

/*
 * Most API methods should use one of the three macros:
 *
 * ENTER_V8, ENTER_V8_NO_SCRIPT, ENTER_V8_NO_SCRIPT_NO_EXCEPTION.
 *
 * The latter two assume that no script is executed, and no exceptions are
 * scheduled in addition (respectively). Creating an exception and
 * removing it before returning is ok.
 *
 * Exceptions should be handled either by invoking one of the
 * RETURN_ON_FAILED_EXECUTION* macros.
 *
 * API methods that are part of the debug interface should use
 *
 * PREPARE_FOR_DEBUG_INTERFACE_EXECUTION_WITH_ISOLATE
 *
 * in a similar fashion to ENTER_V8.
 */

macro_rules! api_rcs_scope {
    ($i_isolate:expr, $class_name:ident, $function_name:ident) => {
        rcs_scope!($i_isolate, RuntimeCallCounterId::kAPI_$class_name_$function_name);
    };
}

macro_rules! enter_v8_basic {
    ($i_isolate:expr) => {
        // Embedders should never enter V8 after terminating it
        if i::v8_flags.strict_termination_checks {
            debug_assert!(!$i_isolate.is_execution_terminating());
        }
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
    };
}

macro_rules! enter_v8_helper_internal {
    ($i_isolate:expr, $context:expr, $class_name:ident, $function_name:ident, $handle_scope_class:ident, $do_callback:expr) => {
        debug_assert!(!$i_isolate.is_execution_terminating());
        let handle_scope = $handle_scope_class::new($i_isolate);
        let call_depth_scope = CallDepthScope::<$do_callback>::new($i_isolate, $context);
        api_rcs_scope!($i_isolate, $class_name, $function_name);
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
        let mut has_exception = false;
    };
}

macro_rules! prepare_for_debug_interface_execution_with_isolate {
    ($i_isolate:expr, $context:expr, $t:ty) => {
        debug_assert!(!$i_isolate.is_execution_terminating());
        let handle_scope = InternalEscapableScope::new($i_isolate);
        let call_depth_scope = CallDepthScope::<false>::new($i_isolate, $context);
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
        let mut has_exception = false;
    };
}

macro_rules! prepare_for_execution {
    ($context:expr, $class_name:ident, $function_name:ident) => {
        let i_isolate = unsafe { ($context.get_isolate() as *mut i::Isolate).as_mut().unwrap() };
        i_isolate.clear_internal_exception();
        enter_v8_helper_internal!(
            i_isolate,
            $context,
            $class_name,
            $function_name,
            InternalEscapableScope,
            false
        );
    };
}

macro_rules! enter_v8 {
    ($i_isolate:expr, $context:expr, $class_name:ident, $function_name:ident, $handle_scope_class:ident) => {
        enter_v8_helper_internal!(
            $i_isolate,
            $context,
            $class_name,
            $function_name,
            $handle_scope_class,
            true
        );
    };
}

#[cfg(debug_assertions)]
macro_rules! enter_v8_no_script {
    ($i_isolate:expr, $context:expr, $class_name:ident, $function_name:ident, $handle_scope_class:ident) => {
        enter_v8_helper_internal!(
            $i_isolate,
            $context,
            $class_name,
            $function_name,
            $handle_scope_class,
            false
        );
        let __no_script__ = i::DisallowJavascriptExecutionDebugOnly::new($i_isolate);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! enter_v8_no_script {
    ($i_isolate:expr, $context:expr, $class_name:ident, $function_name:ident, $handle_scope_class:ident) => {
        enter_v8_helper_internal!(
            $i_isolate,
            $context,
            $class_name,
            $function_name,
            $handle_scope_class,
            false
        );
    };
}

#[cfg(debug_assertions)]
macro_rules! dcheck_no_script_no_exception {
    ($i_isolate:expr) => {
        let __no_script__ = i::DisallowJavascriptExecutionDebugOnly::new($i_isolate);
        let __no_exceptions__ = i::DisallowExceptions::new($i_isolate);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dcheck_no_script_no_exception {
    ($i_isolate:expr) => {};
}

#[cfg(debug_assertions)]
macro_rules! enter_v8_no_script_no_exception {
    ($i_isolate:expr) => {
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
        dcheck_no_script_no_exception!($i_isolate);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! enter_v8_no_script_no_exception {
    ($i_isolate:expr) => {
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
    };
}

#[cfg(debug_assertions)]
macro_rules! enter_v8_for_new_context {
    ($i_isolate:expr) => {
        if i::v8_flags.strict_termination_checks {
            debug_assert!(!$i_isolate.is_execution_terminating());
        }
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
        let __no_exceptions__ = i::DisallowExceptions::new($i_isolate);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! enter_v8_for_new_context {
    ($i_isolate:expr) => {
        let __state__ = i::VMState::<v8::OTHER>::new($i_isolate);
    };
}

macro_rules! return_on_failed_execution {
    ($t:ty) => {
        if has_exception {
            return None; // Assuming MaybeLocal<T> is Option<T>
        }
    };
}

macro_rules! return_on_failed_execution_primitive {
    ($t:ty) => {
        if has_exception {
            return None; // Assuming Nothing<T> is Option<T>
        }
    };
}

macro_rules! return_escaped {
    ($value:expr) => {
        return handle_scope.escape($value);
    };
}

// Placeholder modules and structs.  These would need to be fleshed out based
// on the actual V8 implementation.
mod i {
    pub struct Isolate {
        terminating: bool,
    }

    impl Isolate {
        pub fn clear_internal_exception(&mut self) {}
        pub fn is_execution_terminating(&self) -> bool {
            self.terminating
        }
    }

    pub struct VMState<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> VMState<T> {
        pub fn new(_isolate: &Isolate) -> Self {
            VMState {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct DisallowJavascriptExecutionDebugOnly {
        _isolate: *mut Isolate,
    }

    impl DisallowJavascriptExecutionDebugOnly {
        pub fn new(isolate: &Isolate) -> Self {
            DisallowJavascriptExecutionDebugOnly {
                _isolate: isolate as *const Isolate as *mut Isolate,
            }
        }
    }

    pub struct DisallowExceptions {
        _isolate: *mut Isolate,
    }

    impl DisallowExceptions {
        pub fn new(isolate: &Isolate) -> Self {
            DisallowExceptions {
                _isolate: isolate as *const Isolate as *mut Isolate,
            }
        }
    }

    pub mod v8_flags {
        pub static strict_termination_checks: bool = false;
    }
}

mod v8 {
    pub enum OTHER {}
}

enum RuntimeCallCounterId {
    kAPI_ClassName_FunctionName, // Placeholder. Actual enum would go here.
}

fn rcs_scope(_isolate: &i::Isolate, _id: RuntimeCallCounterId) {}

struct CallDepthScope<const DO_CALLBACK: bool> {
    _isolate: *mut i::Isolate,
    _context: *mut std::ffi::c_void, // Placeholder.  Should be actual context type.
    _phantom: std::marker::PhantomData<bool>,
}

impl<const DO_CALLBACK: bool> CallDepthScope<DO_CALLBACK> {
    fn new(_isolate: &i::Isolate, _context: *mut std::ffi::c_void) -> Self {
        CallDepthScope {
            _isolate: _isolate as *const i::Isolate as *mut i::Isolate,
            _context,
            _phantom: std::marker::PhantomData,
        }
    }
}

struct InternalEscapableScope {
    _isolate: *mut i::Isolate,
}

impl InternalEscapableScope {
    fn new(_isolate: &i::Isolate) -> Self {
        InternalEscapableScope {
            _isolate: _isolate as *const i::Isolate as *mut i::Isolate,
        }
    }

    fn escape<T>(&self, value: T) -> T {
        value
    }
}

trait GetIsolate {
    fn get_isolate(&self) -> *mut std::ffi::c_void;
}
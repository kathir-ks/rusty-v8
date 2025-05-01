// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ffi::CStr;
use std::os::raw::c_char;

use v8::{Context, FunctionCallbackInfo, FunctionTemplate, HandleScope, Isolate, Local, String, Value};

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! check {
            ($e:expr) => {
                if !$e {
                    panic!("Check failed: {}", stringify!($e));
                }
            };
        }

        #[macro_export]
        macro_rules! dcheck {
            ($e:expr) => {
                if cfg!(debug_assertions) {
                    if !$e {
                        panic!("DCheck failed: {}", stringify!($e));
                    }
                }
            };
        }

        #[macro_export]
        macro_rules! slow_dcheck {
            ($e:expr) => {
                if cfg!(debug_assertions) {
                    if !$e {
                        panic!("Slow DCheck failed: {}", stringify!($e));
                    }
                }
            };
        }
    }
}

pub mod extensions {
    use super::*;
    pub struct TriggerFailureExtension {}

    impl TriggerFailureExtension {
        pub const SOURCE: &'static str =
            "native function triggerCheckFalse();\n"
            "native function triggerAssertFalse();\n"
            "native function triggerSlowAssertFalse();";

        pub fn get_native_function_template<'s>(
            isolate: &mut Isolate,
            str: Local<'s, String>,
        ) -> Local<'s, FunctionTemplate> {
            let scope = &mut HandleScope::new(isolate);
            let s = str.to_rust_string_lossy(scope);

            if s == "triggerCheckFalse" {
                FunctionTemplate::new(scope, TriggerFailureExtension::trigger_check_false)
                    .into()
            } else if s == "triggerAssertFalse" {
                FunctionTemplate::new(scope, TriggerFailureExtension::trigger_assert_false)
                    .into()
            } else {
                base::logging::check!(s == "triggerSlowAssertFalse");
                FunctionTemplate::new(scope, TriggerFailureExtension::trigger_slow_assert_false)
                    .into()
            }
        }

        fn trigger_check_false(info: &FunctionCallbackInfo) {
            base::logging::check!(false);
        }

        fn trigger_assert_false(info: &FunctionCallbackInfo) {
            base::logging::dcheck!(false);
        }

        fn trigger_slow_assert_false(info: &FunctionCallbackInfo) {
            base::logging::slow_dcheck!(false);
        }
    }
}
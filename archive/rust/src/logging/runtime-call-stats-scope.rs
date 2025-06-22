// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod runtime_call_stats_scope {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::execution::isolate::Isolate;
    use crate::execution::local_isolate::LocalIsolate;
    use crate::logging::counters::Counters;
    use crate::logging::runtime_call_stats::{RuntimeCallCounterId, RuntimeCallStats, Timer};
    use crate::logging::tracing_flags::TracingFlags;

    // This cfg attribute conditionally compiles the code inside the module
    #[cfg(feature = "runtime_call_stats")]
    pub mod enabled {
        use super::*;

        // Define a macro to mimic the C++ RCS_SCOPE macro.  Since we can't
        // directly use the line number in Rust macros to generate unique
        // identifiers, we'll omit that part.  This might lead to shadowing
        // if multiple RCS_SCOPE macros are used in the same scope, but it's
        // the best we can do without procedural macros.
        #[macro_export]
        macro_rules! rcs_scope {
            ($isolate:expr, $counter_id:expr) => {
                let _rcs_timer_scope =
                    v8::internal::runtime_call_stats_scope::enabled::RuntimeCallTimerScope::new(
                        $isolate,
                        $counter_id,
                    );
            };
            ($isolate:expr, $counter_id:expr, $mode:expr) => {
                let _rcs_timer_scope =
                    v8::internal::runtime_call_stats_scope::enabled::RuntimeCallTimerScope::new_with_mode(
                        $isolate,
                        $counter_id,
                        $mode,
                    );
            };
        }

        pub struct RuntimeCallTimerScope<'a> {
            stats_: Option<Rc<RefCell<&'a mut RuntimeCallStats>>>, // Need to figure out lifetimes, using this for now
            timer_: Timer,
        }

        impl<'a> RuntimeCallTimerScope<'a> {
            pub fn new(isolate: &'a mut Isolate, counter_id: RuntimeCallCounterId) -> Self {
                if !TracingFlags::is_runtime_stats_enabled() {
                    return RuntimeCallTimerScope { stats_: None, timer_: Timer::default() };
                }
                let stats = isolate.counters().runtime_call_stats();
                let mut timer_ = Timer::default();

                stats.borrow_mut().enter(&mut timer_, counter_id);

                RuntimeCallTimerScope {
                    stats_: Some(stats.clone()),
                    timer_: timer_,
                }
            }

            pub fn new_with_mode(
                isolate: &'a mut LocalIsolate,
                counter_id: RuntimeCallCounterId,
                mode: RuntimeCallStats::CounterMode,
            ) -> Self {
                if !TracingFlags::is_runtime_stats_enabled() {
                    return RuntimeCallTimerScope { stats_: None, timer_: Timer::default() };
                }

                let stats = isolate.runtime_call_stats().expect("Runtime call stats should not be null");
                let mut timer_ = Timer::default();

                let counter_id = if mode == RuntimeCallStats::CounterMode::kThreadSpecific {
                    stats.borrow().counter_id_for_thread(counter_id)
                } else {
                    counter_id
                };

                assert!(stats.borrow().is_counter_appropriate_for_thread(counter_id));
                stats.borrow_mut().enter(&mut timer_, counter_id);

                RuntimeCallTimerScope {
                    stats_: Some(stats.clone()),
                    timer_: timer_,
                }
            }
        }

        impl<'a> Drop for RuntimeCallTimerScope<'a> {
            fn drop(&mut self) {
                // The exit logic should go here, but it is not fully defined in the C++ code
            }
        }
    }

    #[cfg(not(feature = "runtime_call_stats"))]
    pub mod disabled {
        #[macro_export]
        macro_rules! rcs_scope {
            ($isolate:expr, $counter_id:expr) => {};
            ($isolate:expr, $counter_id:expr, $mode:expr) => {};
        }
    }

    #[cfg(feature = "runtime_call_stats")]
    pub use enabled::*;

    #[cfg(not(feature = "runtime_call_stats"))]
    pub use disabled::*;
}
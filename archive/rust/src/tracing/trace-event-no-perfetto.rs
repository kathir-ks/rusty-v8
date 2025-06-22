// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is the legacy implementation of tracing macros. There have been two
// concurrent implementations within chromium after perfetto was introduced.
// As of 2024-05, V8 is the only remaining customer of the legacy implementation
// and moved the legacy part from its previous location at
// chromium/src/base/trace_event/common/trace_event_common.h into V8 directly.

// New projects wishing to enable tracing should use the Perfetto SDK. See
// https://perfetto.dev/docs/instrumentation/tracing-sdk for details.

// NOTE: Since the functionality of this header relies heavily on preprocessor
// macros and internal implementation details (INTERNAL_TRACE_EVENT_ADD, etc.),
// a direct translation is not feasible without replicating the entire tracing
// system. The following provides a basic framework.  The actual
// implementation would require a more extensive refactoring.

pub mod trace_event {
    /// This will mark the trace event as disabled by default. The user will need
    /// to explicitly enable the event.
    pub const TRACE_DISABLED_BY_DEFAULT_PREFIX: &str = "disabled-by-default-";

    /// Records a pair of begin and end events called "name" for the current
    /// scope, with 0, 1 or 2 associated arguments. If the category is not
    /// enabled, then this does nothing.
    /// - category and name strings must have application lifetime (statics or
    ///   literals). They may not include " chars.
    #[macro_export]
    macro_rules! trace_event0 {
        ($category_group:expr, $name:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_SCOPED($category_group, $name)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event0! {} {}", $category_group, $name);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_with_flow0 {
        ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW($category_group, $name, $bind_id, $flow_flags)
            #[cfg(feature = "tracing")]
            {
                 println!("trace_event_with_flow0! {} {} {} {}", $category_group, $name, $bind_id, $flow_flags);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event1 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_SCOPED($category_group, $name, $arg1_name, $arg1_val)
             #[cfg(feature = "tracing")]
            {
                println!("trace_event1! {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_with_flow1 {
        ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW($category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_with_flow1! {} {} {} {} {} {}", $category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event2 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD_SCOPED($category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event2! {} {} {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_with_flow2 {
        ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW($category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_with_flow2! {} {} {} {} {} {} {} {}", $category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    // Records a single event called "name" immediately, with 0, 1 or 2
    // associated arguments. If the category is not enabled, then this
    // does nothing.
    // - category and name strings must have application lifetime (statics or
    //   literals). They may not include " chars.
    #[macro_export]
    macro_rules! trace_event_instant0 {
        ($category_group:expr, $name:expr, $scope:expr) => {
           // INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope)
           #[cfg(feature = "tracing")]
           {
               println!("trace_event_instant0! {} {} {}", $category_group, $name, $scope);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant1 {
        ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_instant1! {} {} {} {} {}", $category_group, $name, $scope, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant2 {
        ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_instant2! {} {} {} {} {} {} {}", $category_group, $name, $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_instant0 {
        ($category_group:expr, $name:expr, $scope:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_instant0! {} {} {}", $category_group, $name, $scope);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_instant1 {
        ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_instant1! {} {} {} {} {}", $category_group, $name, $scope, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_instant2 {
        ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_instant2! {} {} {} {} {} {} {}", $category_group, $name, $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant_with_flags0 {
        ($category_group:expr, $name:expr, $scope_and_flags:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $scope_and_flags)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_instant_with_flags0! {} {} {}", $category_group, $name, $scope_and_flags);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant_with_flags1 {
        ($category_group:expr, $name:expr, $scope_and_flags:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $scope_and_flags, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_instant_with_flags1! {} {} {} {} {}", $category_group, $name, $scope_and_flags, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant_with_timestamp0 {
        ($category_group:expr, $name:expr, $scope:expr, $timestamp:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE | $scope)
            #[cfg(feature = "tracing")]
            {
               println!("trace_event_instant_with_timestamp0! {} {} {} {}", $category_group, $name, $scope, $timestamp);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_instant_with_timestamp1 {
        ($category_group:expr, $name:expr, $scope:expr, $timestamp:expr, $arg_name:expr, $arg_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE | $scope, $arg_name, $arg_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_instant_with_timestamp1! {} {} {} {} {} {}", $category_group, $name, $scope, $timestamp, $arg_name, $arg_val);
            }
        };
    }

    // Records a single BEGIN event called "name" immediately, with 0, 1 or 2
    // associated arguments. If the category is not enabled, then this
    // does nothing.
    // - category and name strings must have application lifetime (statics or
    //   literals). They may not include " chars.
    #[macro_export]
    macro_rules! trace_event_begin0 {
        ($category_group:expr, $name:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_begin0! {} {}", $category_group, $name);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_begin1 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_begin1! {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_begin2 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_begin2! {} {} {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_begin_with_flags0 {
        ($category_group:expr, $name:expr, $flags:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, $flags)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_begin_with_flags0! {} {} {}", $category_group, $name, $flags);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_begin_with_flags1 {
        ($category_group:expr, $name:expr, $flags:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, $flags, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_begin_with_flags1! {} {} {} {} {}", $category_group, $name, $flags, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_begin2 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
           #[cfg(feature = "tracing")]
           {
               println!("trace_event_copy_begin2! {} {} {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
           }
        };
    }

    // Similar to TRACE_EVENT_BEGINx but with a custom |timestamp| provided.
    // - |id| is used to match the _BEGIN event with the _END event.
    //   Events are considered to match if their category_group, name and id values
    //   all match. |id| must either be a pointer or an integer value up to 64 bits.
    //   If it's a pointer, the bits will be xored with a hash of the process ID so
    //   that the same pointer on two different processes will not collide.
    // - |timestamp| must be non-null or it crashes. Use DCHECK(timestamp) before
    //   calling this to detect an invalid timestamp even when tracing is not
    //   enabled, as the commit queue doesn't run all tests with tracing enabled.
    // Note: This legacy macro is deprecated. It should not be used in new code.
    //       If thread_id is different from current thread id, it will result into
    //       DCHECK failure. This note is also applicable to `_COPY` and `_END`
    //       variant of this macro.
    #[macro_export]
    macro_rules! trace_event_begin_with_id_tid_and_timestamp0 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_NONE)
            #[cfg(feature = "tracing")]
            {
               println!("trace_event_begin_with_id_tid_and_timestamp0! {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_begin_with_id_tid_and_timestamp0 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_begin_with_id_tid_and_timestamp0! {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_begin_with_id_tid_and_timestamp1 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_begin_with_id_tid_and_timestamp1! {} {} {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_begin_with_id_tid_and_timestamp2 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_begin_with_id_tid_and_timestamp2! {} {} {} {} {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    // Records a single END event for "name" immediately. If the category
    // is not enabled, then this does nothing.
    // - category and name strings must have application lifetime (statics or
    //   literals). They may not include " chars.
    #[macro_export]
    macro_rules! trace_event_end0 {
        ($category_group:expr, $name:expr) => {
           // INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_end0! {} {}", $category_group, $name);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_end1 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_end1! {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_end2 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
           #[cfg(feature = "tracing")]
           {
               println!("trace_event_end2! {} {} {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_end_with_flags0 {
        ($category_group:expr, $name:expr, $flags:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, $flags)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_end_with_flags0! {} {} {}", $category_group, $name, $flags);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_end_with_flags1 {
        ($category_group:expr, $name:expr, $flags:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, $flags, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_end_with_flags1! {} {} {} {} {}", $category_group, $name, $flags, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_end2 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_end2! {} {} {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    // Adds a trace event with the given |name| and |timestamp|. |timestamp| must be
    // non-null or it crashes. Use DCHECK(timestamp) before calling this to detect
    // an invalid timestamp even when tracing is not enabled, as the commit queue
    // doesn't run all tests with tracing enabled.
    #[macro_export]
    macro_rules! trace_event_mark_with_timestamp0 {
        ($category_group:expr, $name:expr, $timestamp:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_mark_with_timestamp0! {} {} {}", $category_group, $name, $timestamp);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_mark_with_timestamp1 {
        ($category_group:expr, $name:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_mark_with_timestamp1! {} {} {} {} {}", $category_group, $name, $timestamp, $arg1_name, $arg1_val);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_mark_with_timestamp2 {
        ($category_group:expr, $name:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_mark_with_timestamp2! {} {} {} {} {} {} {}", $category_group, $name, $timestamp, $arg1_name, $arg1_val, $arg2_name, $arg2_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_mark {
        ($category_group:expr, $name:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_MARK, $category_group, $name, TRACE_EVENT_FLAG_COPY)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_mark! {} {}", $category_group, $name);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_mark1 {
        ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
            //INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_MARK, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_mark1! {} {} {} {}", $category_group, $name, $arg1_name, $arg1_val);
            }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_mark_with_timestamp {
        ($category_group:expr, $name:expr, $timestamp:expr) => {
            //INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_COPY)
            #[cfg(feature = "tracing")]
            {
                println!("trace_event_copy_mark_with_timestamp! {} {} {}", $category_group, $name, $timestamp);
            }
        };
    }

    // Similar to TRACE_EVENT_ENDx but with a custom |timestamp| provided.
    // - |id| is used to match the _BEGIN event with the _END event.
    //   Events are considered to match if their category_group, name and id values
    //   all match. |id| must either be a pointer or an integer value up to 64 bits.
    //   If it's a pointer, the bits will be xored with a hash of the process ID so
    //   that the same pointer on two different processes will not collide.
    // - |timestamp| must be non-null or it crashes. Use DCHECK(timestamp) before
    //   calling this to detect an invalid timestamp even when tracing is not
    //   enabled, as the commit queue doesn't run all tests with tracing enabled.
    #[macro_export]
    macro_rules! trace_event_end_with_id_tid_and_timestamp0 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
           // INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_NONE)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_end_with_id_tid_and_timestamp0! {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_end_with_id_tid_and_timestamp0 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
           // INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_copy_end_with_id_tid_and_timestamp0! {} {} {} {} {}", $category_group, $name, $id, $thread_id, $timestamp);
           }
        };
    }

    #[macro_export]
    macro_rules! trace_event_copy_end_with_id_tid_and_timestamp1 {
        ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
           // INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
           #[cfg(feature = "tracing")]
           {
                println!("trace_event_copy_end_with_id_tid_and_timestamp1! {} {} {} {} {} {} {}", $category_group, $name, $id, $
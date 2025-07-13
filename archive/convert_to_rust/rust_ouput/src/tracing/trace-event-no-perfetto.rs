// Converted from V8 C++ source files:
// Header: trace-event-no-perfetto.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

// This is the legacy implementation of tracing macros. There have been two
// concurrent implementations within chromium after perfetto was introduced.
// As of 2024-05, V8 is the only remaining customer of the legacy implementation
// and moved the legacy part from its previous location at
// chromium/src/base/trace_event/common/trace_event_common.h into V8 directly.

// New projects wishing to enable tracing should use the Perfetto SDK. See
// https://perfetto.dev/docs/instrumentation/tracing-sdk for details.

// This will mark the trace event as disabled by default. The user will need
// to explicitly enable the event.
macro_rules! TRACE_DISABLED_BY_DEFAULT {
    ($name:ident) => {
        concat!("disabled-by-default-", stringify!($name))
    };
}

// Records a pair of begin and end events called "name" for the current
// scope, with 0, 1 or 2 associated arguments. If the category is not
// enabled, then this does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_EVENT0 {
    ($category_group:expr, $name:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED!($category_group, $name)
    };
}

macro_rules! TRACE_EVENT_WITH_FLOW0 {
    ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW!($category_group, $name, $bind_id, $flow_flags)
    };
}

macro_rules! TRACE_EVENT1 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED!($category_group, $name, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_WITH_FLOW1 {
    ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW!($category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT2 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED!($category_group, $name, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_WITH_FLOW2 {
    ($category_group:expr, $name:expr, $bind_id:expr, $flow_flags:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW!($category_group, $name, $bind_id, $flow_flags, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

// Records a single event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_EVENT_INSTANT0 {
    ($category_group:expr, $name:expr, $scope:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope)
    };
}

macro_rules! TRACE_EVENT_INSTANT1 {
    ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_INSTANT2 {
    ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_NONE | $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_COPY_INSTANT0 {
    ($category_group:expr, $name:expr, $scope:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope)
    };
}

macro_rules! TRACE_EVENT_COPY_INSTANT1 {
    ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_INSTANT2 {
    ($category_group:expr, $name:expr, $scope:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, TRACE_EVENT_FLAG_COPY | $scope, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_INSTANT_WITH_FLAGS0 {
    ($category_group:expr, $name:expr, $scope_and_flags:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $scope_and_flags)
    };
}

macro_rules! TRACE_EVENT_INSTANT_WITH_FLAGS1 {
    ($category_group:expr, $name:expr, $scope_and_flags:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $scope_and_flags, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_INSTANT_WITH_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $scope:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE | $scope)
    };
}

macro_rules! TRACE_EVENT_INSTANT_WITH_TIMESTAMP1 {
    ($category_group:expr, $name:expr, $scope:expr, $timestamp:expr, $arg_name:expr, $arg_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_INSTANT, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE | $scope, $arg_name, $arg_val)
    };
}

// Records a single BEGIN event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_EVENT_BEGIN0 {
    ($category_group:expr, $name:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_BEGIN1 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_BEGIN2 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_BEGIN_WITH_FLAGS0 {
    ($category_group:expr, $name:expr, $flags:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, $flags)
    };
}

macro_rules! TRACE_EVENT_BEGIN_WITH_FLAGS1 {
    ($category_group:expr, $name:expr, $flags:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, $flags, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_BEGIN2 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_BEGIN, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
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
macro_rules! TRACE_EVENT_BEGIN_WITH_ID_TID_AND_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY)
    };
}

macro_rules! TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP1 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP2 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

// Records a single END event for "name" immediately. If the category
// is not enabled, then this does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_EVENT_END0 {
    ($category_group:expr, $name:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_END1 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_END2 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_END_WITH_FLAGS0 {
    ($category_group:expr, $name:expr, $flags:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, $flags)
    };
}

macro_rules! TRACE_EVENT_END_WITH_FLAGS1 {
    ($category_group:expr, $name:expr, $flags:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, $flags, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_END2 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_END, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

// Adds a trace event with the given |name| and |timestamp|. |timestamp| must be
// non-null or it crashes. Use DCHECK(timestamp) before calling this to detect
// an invalid timestamp even when tracing is not enabled, as the commit queue
// doesn't run all tests with tracing enabled.
macro_rules! TRACE_EVENT_MARK_WITH_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_MARK_WITH_TIMESTAMP1 {
    ($category_group:expr, $name:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_MARK_WITH_TIMESTAMP2 {
    ($category_group:expr, $name:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

macro_rules! TRACE_EVENT_COPY_MARK {
    ($category_group:expr, $name:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_MARK, $category_group, $name, TRACE_EVENT_FLAG_COPY)
    };
}

macro_rules! TRACE_EVENT_COPY_MARK1 {
    ($category_group:expr, $name:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_MARK, $category_group, $name, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_MARK_WITH_TIMESTAMP {
    ($category_group:expr, $name:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_MARK, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_COPY)
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
macro_rules! TRACE_EVENT_END_WITH_ID_TID_AND_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP0 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY)
    };
}

macro_rules! TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP1 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP2 {
    ($category_group:expr, $name:expr, $id:expr, $thread_id:expr, $timestamp:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP!(TRACE_EVENT_PHASE_ASYNC_END, $category_group, $name, $id, $thread_id, $timestamp, TRACE_EVENT_FLAG_COPY, $arg1_name, $arg1_val, $arg2_name, $arg2_val)
    };
}

// Records the value of a counter called "name" immediately. Value
// must be representable as a 32 bit integer.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_COUNTER1 {
    ($category_group:expr, $name:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, TRACE_EVENT_FLAG_NONE, "value", $value as i32)
    };
}

macro_rules! TRACE_COUNTER_WITH_FLAG1 {
    ($category_group:expr, $name:expr, $flag:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $flag, "value", $value as i32)
    };
}

macro_rules! TRACE_COPY_COUNTER1 {
    ($category_group:expr, $name:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, TRACE_EVENT_FLAG_COPY, "value", $value as i32)
    };
}

// Records the values of a multi-parted counter called "name" immediately.
// The UI will treat value1 and value2 as parts of a whole, displaying their
// values as a stacked-bar chart.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
macro_rules! TRACE_COUNTER2 {
    ($category_group:expr, $name:expr, $value1_name:expr, $value1_val:expr, $value2_name:expr, $value2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, TRACE_EVENT_FLAG_NONE, $value1_name, $value1_val as i32, $value2_name, $value2_val as i32)
    };
}

macro_rules! TRACE_COPY_COUNTER2 {
    ($category_group:expr, $name:expr, $value1_name:expr, $value1_val:expr, $value2_name:expr, $value2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, TRACE_EVENT_FLAG_COPY, $value1_name, $value1_val as i32, $value2_name, $value2_val as i32)
    };
}

// Similar to TRACE_COUNTERx, but with a custom |timestamp| provided.
// - |timestamp| must be non-null or it crashes. Use DCHECK(timestamp) before
//   calling this to detect an invalid timestamp even when tracing is not
//   enabled, as the commit queue doesn't run all tests with tracing enabled.
macro_rules! TRACE_COUNTER_WITH_TIMESTAMP1 {
    ($category_group:expr, $name:expr, $timestamp:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, "value", $value as i32)
    };
}

macro_rules! TRACE_COUNTER_WITH_TIMESTAMP2 {
    ($category_group:expr, $name:expr, $timestamp:expr, $value1_name:expr, $value1_val:expr, $value2_name:expr, $value2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $timestamp, TRACE_EVENT_FLAG_NONE, $value1_name, $value1_val as i32, $value2_name, $value2_val as i32)
    };
}

// Records the value of a counter called "name" immediately. Value
// must be representable as a 32 bit integer.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - |id| is used to disambiguate counters with the same name. It must either
//   be a pointer or an integer value up to 64 bits. If it's a pointer, the bits
//   will be xored with a hash of the process ID so that the same pointer on
//   two different processes will not collide.
macro_rules! TRACE_COUNTER_ID1 {
    ($category_group:expr, $name:expr, $id:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $id, TRACE_EVENT_FLAG_NONE, "value", $value as i32)
    };
}

macro_rules! TRACE_COPY_COUNTER_ID1 {
    ($category_group:expr, $name:expr, $id:expr, $value:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $id, TRACE_EVENT_FLAG_COPY, "value", $value as i32)
    };
}

// Records the values of a multi-parted counter called "name" immediately.
// The UI will treat value1 and value2 as parts of a whole, displaying their
// values as a stacked-bar chart.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - |id| is used to disambiguate counters with the same name. It must either
//   be a pointer or an integer value up to 64 bits. If it's a pointer, the bits
//   will be xored with a hash of the process ID so that the same pointer on
//   two different processes will not collide.
macro_rules! TRACE_COUNTER_ID2 {
    ($category_group:expr, $name:expr, $id:expr, $value1_name:expr, $value1_val:expr, $value2_name:expr, $value2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $id, TRACE_EVENT_FLAG_NONE, $value1_name, $value1_val as i32, $value2_name, $value2_val as i32)
    };
}

macro_rules! TRACE_COPY_COUNTER_ID2 {
    ($category_group:expr, $name:expr, $id:expr, $value1_name:expr, $value1_val:expr, $value2_name:expr, $value2_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_COUNTER, $category_group, $name, $id, TRACE_EVENT_FLAG_COPY, $value1_name, $value1_val as i32, $value2_name, $value2_val as i32)
    };
}

macro_rules! TRACE_EVENT_SAMPLE_WITH_ID1 {
    ($category_group:expr, $name:expr, $id:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_SAMPLE, $category_group, $name, $id, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
    };
}

// -- TRACE_EVENT_ASYNC is DEPRECATED! --
//
// TRACE_EVENT_ASYNC_* APIs should be only used by legacy code. New code should
// use TRACE_EVENT_NESTABLE_ASYNC_* APIs instead.
//
// Records a single ASYNC_BEGIN event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - |id| is used to match the ASYNC_BEGIN event with the ASYNC_END event. ASYNC
//   events are considered to match if their category_group, name and id values
//   all match. |id| must either be a pointer or an integer value up to 64 bits.
//   If it's a pointer, the bits will be xored with a hash of the process ID so
//   that the same pointer on two different processes will not collide.
//
// An asynchronous operation can consist of multiple phases. The first phase is
// defined by the ASYNC_BEGIN calls. Additional phases can be defined using the
// ASYNC_STEP_INTO or ASYNC_STEP_PAST macros. The ASYNC_STEP_INTO macro will
// annotate the block following the call. The ASYNC_STEP_PAST macro will
// annotate the block prior to the call. Note that any particular event must use
// only STEP_INTO or STEP_PAST macros; they can not mix and match. When the
// operation completes, call ASYNC_END.
//
// An ASYNC trace typically occurs on a single thread (if not, they will only be
// drawn on the thread defined in the ASYNC_BEGIN event), but all events in that
// operation must use the same |name| and |id|. Each step can have its own
// args.
macro_rules! TRACE_EVENT_ASYNC_BEGIN0 {
    ($category_group:expr, $name:expr, $id:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, TRACE_EVENT_FLAG_NONE)
    };
}

macro_rules! TRACE_EVENT_ASYNC_BEGIN1 {
    ($category_group:expr, $name:expr, $id:expr, $arg1_name:expr, $arg1_val:expr) => {
        INTERNAL_TRACE_EVENT_ADD_WITH_ID!(TRACE_EVENT_PHASE_ASYNC_BEGIN, $category_group, $name, $id, TRACE_EVENT_FLAG_NONE, $arg1_name, $arg1_val)
    };
}

macro_rules! TRACE_EVENT_ASYNC_BEGIN2 {
    ($category_group:expr, $name:expr, $id:expr, $arg1_name:expr, $arg1_val:expr, $arg2_name:expr, $arg2_val:expr) => {
        INTERNAL_

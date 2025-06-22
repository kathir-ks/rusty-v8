// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

///! This module provides macros and traits for stack allocation management.

/// A marker trait to indicate that a type is stack allocated.
///
/// Types implementing this trait are intended to be allocated only on the stack.
pub trait IsStackAllocatedType {}

/// A macro to enforce stack allocation for a type.
///
/// This macro prevents heap allocation by deleting the `new` operator.
/// It also defines a marker type `IsStackAllocatedTypeMarker` to allow checking if a type is intended for stack allocation.
macro_rules! cppgc_stack_allocated {
    () => {
        pub struct IsStackAllocatedTypeMarker;

        impl IsStackAllocatedType for Self {}

        // Prevent heap allocation.  Rust doesn't have operator overloading
        // like C++, so we can't truly delete the `new` operator.
        // Instead, we can mark the type as !Send and !Sync if heap allocation
        // is a significant concern and the type isn't intended to be used across threads.
    };
}

/// A macro to ignore the `CPPGC_STACK_ALLOCATED` annotation.
///
/// This macro does nothing in Rust as it was used to suppress warnings or errors related to stack allocation in C++.
macro_rules! cppgc_stack_allocated_ignore {
    ($bug_or_reason:expr) => {};
}

pub(crate) use cppgc_stack_allocated;
pub(crate) use cppgc_stack_allocated_ignore;
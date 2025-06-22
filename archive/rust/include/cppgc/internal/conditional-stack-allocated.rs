// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a way to conditionally mark a type as stack allocated
// based on whether the type is marked as stack allocated, traceable, or a garbage collected type.

/// A trait that can be implemented to mark a type as stack allocated.
pub trait StackAllocated {}

/// A marker trait for types that are traceable by the garbage collector.
pub trait Traceable {}

/// A marker trait for types that are either garbage collected or mixed-in.
pub trait GarbageCollectedOrMixinType {}

mod internal {
    use super::*;

    /// A trait alias that represents the condition for a type to require stack allocation.
    pub trait RequiresStackAllocated<T> {}

    impl<T> RequiresStackAllocated<T> for T
    where
        T: ?Sized,
        T: IsNotVoid,
        T: StackAllocatedTraitOrTraceableOrGarbageCollected,
    {
    }

    pub trait IsNotVoid {}

    impl<T> IsNotVoid for T {}

    // Implementations for the missing pieces of the `RequiresStackAllocated` trait bound.

    pub trait StackAllocatedTraitOrTraceableOrGarbageCollected {}

    impl<T> StackAllocatedTraitOrTraceableOrGarbageCollected for T
    where
        T: ?Sized,
        (T: StackAllocated) | (T: Traceable) | (T: GarbageCollectedOrMixinType),
    {
    }

    /// A helper struct to conditionally mark a type as stack allocated based on the `RequiresStackAllocated` trait.
    pub struct ConditionalStackAllocatedBase<T>(core::marker::PhantomData<T>);

    impl<T> ConditionalStackAllocatedBase<T> {
        pub fn new() -> Self {
            ConditionalStackAllocatedBase(core::marker::PhantomData)
        }
    }

    impl<T> ConditionalStackAllocatedBase<T> where T: RequiresStackAllocated<T> + ?Sized + StackAllocated {
        // Marks the type as stack allocated using the `StackAllocated` trait.
    }

    impl<T> ConditionalStackAllocatedBase<T> where T: ?Sized, T: NotRequiresStackAllocated {}

    pub trait NotRequiresStackAllocated {}

    impl<T> !RequiresStackAllocated<T> for T where T: NotRequiresStackAllocated {}
}
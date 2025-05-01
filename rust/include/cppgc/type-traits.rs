// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod cppgc {
    pub mod internal {
        use std::marker::PhantomData;
        use std::mem;
        use std::ops::Deref;

        pub trait Visitor {}

        pub struct BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
            ptr: *mut T,
            _weakness_tag: PhantomData<WeaknessTag>,
            _write_barrier_policy: PhantomData<WriteBarrierPolicy>,
            _checking_policy: PhantomData<CheckingPolicy>,
            _storage_type: PhantomData<StorageType>,
        }

        impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
            pub fn new(ptr: *mut T) -> Self {
                BasicMember {
                    ptr,
                    _weakness_tag: PhantomData,
                    _write_barrier_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _storage_type: PhantomData,
                }
            }

            pub fn get(&self) -> *mut T {
                self.ptr
            }
        }

        pub struct DijkstraWriteBarrierPolicy;
        pub struct NoWriteBarrierPolicy;
        pub struct StrongMemberTag;
        pub struct UntracedMemberTag;
        pub struct WeakMemberTag;

        // Not supposed to be specialized by the user.
        pub struct IsWeak<T>(PhantomData<T>);

        impl<T> IsWeak<T> {
            pub const VALUE: bool = false;
        }

        // IsTraceMethodConst is used to verify that all Trace methods are marked as
        // const. It is equivalent to IsTraceable but for a non-const object.
        pub struct IsTraceMethodConst<T>(PhantomData<T>);

        impl<T> IsTraceMethodConst<T> {
            pub const VALUE: bool = {
                // This requires a `Trace` method to be defined.  Since this is a negative
                // trait bound we skip it for now.  This needs compiler support.
                false
            };
        }

        pub struct IsTraceable<T>(PhantomData<T>);

        impl<T> IsTraceable<T> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                // This requires a `Trace` method to be defined.  Since this is a negative
                // trait bound we skip it for now.  This needs compiler support.
                false
            };
        }

        pub const IS_TRACEABLE_V: bool = false;

        pub struct HasGarbageCollectedMixinTypeMarker<T>(PhantomData<T>);

        impl<T> HasGarbageCollectedMixinTypeMarker<T> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                // This requires a `IsGarbageCollectedMixinTypeMarker` type to be defined.  Since this is a negative
                // trait bound we skip it for now.  This needs compiler support.
                false
            };
        }

        pub struct HasGarbageCollectedTypeMarker<T>(PhantomData<T>);

        impl<T> HasGarbageCollectedTypeMarker<T> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                // This requires a `IsGarbageCollectedTypeMarker` type to be defined.  Since this is a negative
                // trait bound we skip it for now.  This needs compiler support.
                false
            };
        }

        pub struct IsGarbageCollectedMixinType<T, const B1: bool, const B2: bool>(PhantomData<T>);

        impl<T, const B1: bool, const B2: bool> IsGarbageCollectedMixinType<T, B1, B2> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                false
            };
        }

        impl<T> IsGarbageCollectedMixinType<T, false, true> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                true
            };
        }

        pub struct IsGarbageCollectedType<T, const B: bool>(PhantomData<T>);

        impl<T, const B: bool> IsGarbageCollectedType<T, B> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                false
            };
        }

        impl<T> IsGarbageCollectedType<T, true> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                true
            };
        }

        pub struct IsGarbageCollectedOrMixinType<T>(PhantomData<T>);

        impl<T> IsGarbageCollectedOrMixinType<T> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                IsGarbageCollectedType::<T, { HasGarbageCollectedTypeMarker::<T>::VALUE }>::VALUE || IsGarbageCollectedMixinType::<T, { HasGarbageCollectedTypeMarker::<T>::VALUE }, { HasGarbageCollectedMixinTypeMarker::<T>::VALUE }>::VALUE
            };
        }

        pub struct IsGarbageCollectedWithMixinType<T, const B: bool>(PhantomData<T>);

        impl<T, const B: bool> IsGarbageCollectedWithMixinType<T, B> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                false
            };
        }

        impl<T> IsGarbageCollectedWithMixinType<T, true> {
            pub const VALUE: bool = {
                assert!(mem::size_of::<T>() > 0, "T must be fully defined");
                true
            };
        }

        pub struct IsSubclassOfBasicMemberTemplate<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy>(PhantomData<(BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy)>);

        impl<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy> IsSubclassOfBasicMemberTemplate<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy> {
            // This is difficult to represent directly in Rust without using `unsafe` and transmuting types.
            // We would need to check if the BasicMemberCandidate can be safely cast to a BasicMember<_, WeaknessTag, WriteBarrierPolicy, _, _>.
            pub const VALUE: bool = false;
        }

        pub struct IsMemberType<T, const B: bool>(PhantomData<T>);

        impl<T, const B: bool> IsMemberType<T, B> {
            pub const VALUE: bool = false;
        }

        impl<T> IsMemberType<T, true> {
            pub const VALUE: bool = true;
        }

        pub struct IsWeakMemberType<T, const B: bool>(PhantomData<T>);

        impl<T, const B: bool> IsWeakMemberType<T, B> {
            pub const VALUE: bool = false;
        }

        impl<T> IsWeakMemberType<T, true> {
            pub const VALUE: bool = true;
        }

        pub struct IsUntracedMemberType<T, const B: bool>(PhantomData<T>);

        impl<T, const B: bool> IsUntracedMemberType<T, B> {
            pub const VALUE: bool = false;
        }

        impl<T> IsUntracedMemberType<T, true> {
            pub const VALUE: bool = true;
        }

        pub struct IsComplete<T>(PhantomData<T>);

        impl<T> IsComplete<T> {
            pub const VALUE: bool = {
                // In Rust, types are generally known to be complete at compile time.
                // If a type is incomplete, the compiler will usually throw an error when
                // it's used. This is a simplified version of the C++ check.
                mem::size_of::<T>() > 0
            };
        }

        pub const IS_ANY_MEMBER_TYPE_V: bool = false;

        impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> IsAnyMemberTypeV for BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
            const IS_ANY_MEMBER_TYPE_V: bool = true;
        }
    }  // namespace internal

    /**
     * Value is true for types that inherit from `GarbageCollectedMixin` but not
     * `GarbageCollected<T>` (i.e., they are free mixins), and false otherwise.
     */
    pub const IS_GARBAGE_COLLECTED_MIXIN_TYPE_V: bool = false;

    /**
     * Value is true for types that inherit from `GarbageCollected<T>`, and false
     * otherwise.
     */
    pub const IS_GARBAGE_COLLECTED_TYPE_V: bool = false;

    /**
     * Value is true for types that inherit from either `GarbageCollected<T>` or
     * `GarbageCollectedMixin`, and false otherwise.
     */
    pub const IS_GARBAGE_COLLECTED_OR_MIXIN_TYPE_V: bool = false;

    /**
     * Value is true for types that inherit from `GarbageCollected<T>` and
     * `GarbageCollectedMixin`, and false otherwise.
     */
    pub const IS_GARBAGE_COLLECTED_WITH_MIXIN_TYPE_V: bool = false;

    /**
     * Value is true for types of type `Member<T>`, and false otherwise.
     */
    pub const IS_MEMBER_TYPE_V: bool = false;

    /**
     * Value is true for types of type `UntracedMember<T>`, and false otherwise.
     */
    pub const IS_UNTRACED_MEMBER_TYPE_V: bool = false;

    /**
     * Value is true for types of type `WeakMember<T>`, and false otherwise.
     */
    pub const IS_WEAK_MEMBER_TYPE_V: bool = false;

    /**
     * Value is true for types that are considered weak references, and false
     * otherwise.
     */
    pub const IS_WEAK_V: bool = false;

    /**
     * Value is true for types that are complete, and false otherwise.
     */
    pub const IS_COMPLETE_V: bool = true;

    /**
     * Value is true for member types `Member<T>` and `WeakMember<T>`.
     */
    pub const IS_MEMBER_OR_WEAK_MEMBER_TYPE_V: bool = false;

    /**
     * Value is true for any member type.
     */
    pub const IS_ANY_MEMBER_TYPE_V: bool = false;
}  // namespace cppgc
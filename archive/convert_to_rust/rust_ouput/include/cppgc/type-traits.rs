// Converted from V8 C++ source files:
// Header: type-traits.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {

    pub struct Visitor {}

    pub mod internal {

        pub struct BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
            _phantom: std::marker::PhantomData<(
                T,
                WeaknessTag,
                WriteBarrierPolicy,
                CheckingType,
                StorageType,
            )>,
        }
        pub struct DijkstraWriteBarrierPolicy {}
        pub struct NoWriteBarrierPolicy {}
        pub struct StrongMemberTag {}
        pub struct UntracedMemberTag {}
        pub struct WeakMemberTag {}

        pub struct IsWeak<T>(pub std::marker::PhantomData<T>);
        impl<T> IsWeak<T> {
            pub const value: bool = false;
        }

        pub struct IsTraceMethodConst<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> IsTraceMethodConst<T> {
            pub const value: bool = false;
        }

        pub struct IsTraceable<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> IsTraceable<T> {
            pub const value: bool = false;
        }

        pub const IS_TRACEABLE_V: bool = false;

        pub struct HasGarbageCollectedMixinTypeMarker<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> HasGarbageCollectedMixinTypeMarker<T> {
            pub const value: bool = false;
        }

        pub struct HasGarbageCollectedTypeMarker<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> HasGarbageCollectedTypeMarker<T> {
            pub const value: bool = false;
        }

        pub struct IsGarbageCollectedMixinType<T, const A: bool, const B: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool, const B: bool> IsGarbageCollectedMixinType<T, A, B> {
            pub const value: bool = false;
        }

        pub struct IsGarbageCollectedType<T, const A: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool> IsGarbageCollectedType<T, A> {
            pub const value: bool = false;
        }

        pub struct IsGarbageCollectedOrMixinType<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> IsGarbageCollectedOrMixinType<T> {
            pub const value: bool = false;
        }

        pub struct IsGarbageCollectedWithMixinType<T, const A: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool> IsGarbageCollectedWithMixinType<T, A> {
            pub const value: bool = false;
        }

        pub struct IsSubclassOfBasicMemberTemplate<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy> {
            _phantom: std::marker::PhantomData<(BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy)>;
        }

        impl<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy> IsSubclassOfBasicMemberTemplate<BasicMemberCandidate, WeaknessTag, WriteBarrierPolicy> {
            pub const value: bool = false;
        }

        pub struct IsMemberType<T, const A: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool> IsMemberType<T, A> {
            pub const value: bool = false;
        }

        pub struct IsWeakMemberType<T, const A: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool> IsWeakMemberType<T, A> {
            pub const value: bool = false;
        }

        pub struct IsUntracedMemberType<T, const A: bool> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const A: bool> IsUntracedMemberType<T, A> {
            pub const value: bool = false;
        }

        pub struct IsComplete<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> IsComplete<T> {
            pub const value: bool = true;
        }

        pub const IS_DECAYED_SAME_V: bool = false;

        pub const IS_STRICTLY_BASE_OF_V: bool = false;

        pub const IS_ANY_MEMBER_TYPE_V: bool = false;
    }

    pub const IS_GARBAGE_COLLECTED_MIXIN_TYPE_V: bool =
        internal::IsGarbageCollectedMixinType::<(), false, false>::value;

    pub const IS_GARBAGE_COLLECTED_TYPE_V: bool = internal::IsGarbageCollectedType::<(), false>::value;

    pub const IS_GARBAGE_COLLECTED_OR_MIXIN_TYPE_V: bool =
        internal::IsGarbageCollectedOrMixinType::<()>::value;

    pub const IS_GARBAGE_COLLECTED_WITH_MIXIN_TYPE_V: bool =
        internal::IsGarbageCollectedWithMixinType::<(), false>::value;

    pub const IS_MEMBER_TYPE_V: bool = internal::IsMemberType::<(), false>::value;

    pub const IS_UNTRACED_MEMBER_TYPE_V: bool = internal::IsUntracedMemberType::<(), false>::value;

    pub const IS_WEAK_MEMBER_TYPE_V: bool = internal::IsWeakMemberType::<(), false>::value;

    pub const IS_WEAK_V: bool = internal::IsWeak::<()>::value;

    pub const IS_COMPLETE_V: bool = internal::IsComplete::<()>::value;

    pub const IS_MEMBER_OR_WEAK_MEMBER_TYPE_V: bool = IS_MEMBER_TYPE_V || IS_WEAK_MEMBER_TYPE_V;

    pub const IS_ANY_MEMBER_TYPE_V: bool = internal::IS_ANY_MEMBER_TYPE_V;
}

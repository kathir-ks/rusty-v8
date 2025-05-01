// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    // use std::ptr::NonNull;
    use std::marker::PhantomData;

    // use crate::cppgc::SourceLocation;
    // use crate::cppgc::WriteBarrierSlotType;

    // Placeholder for v8config.h, define V8_INLINE and V8_EXPORT
    macro_rules! V8_INLINE {
        () => {};
    }

    macro_rules! V8_EXPORT {
        () => {};
    }

    // Placeholder for WriteBarrier
    mod write_barrier {
        pub struct Params;
        #[allow(dead_code)]
        pub enum Type {
            kGenerational,
            kMarking,
            kNone,
        }
        #[allow(dead_code)]
        pub enum GenerationalBarrierType {
            kPreciseSlot,
        }

        pub fn is_enabled() -> bool {
            false
        }

        pub fn combined_write_barrier_slow<T>(_slot: *const std::ffi::c_void) {
            // Placeholder implementation
        }
        pub fn get_write_barrier_type(
            _slot: *const std::ffi::c_void,
            _value: *const std::ffi::c_void,
            _params: &mut Params,
        ) -> Type {
            Type::kNone
        }
        pub fn dijkstra_marking_barrier(_params: &Params, _value: *const std::ffi::c_void) {}
        pub fn generational_barrier<T>(
            _params: &Params,
            _slot: *const std::ffi::c_void,
        )
        where
            T: Sized,
        {
            // Placeholder implementation
        }
    }

    // Placeholder for RawPointer and CompressedPointer
    mod member_storage {
        #[derive(Clone, Copy)]
        pub struct RawPointer(*mut std::ffi::c_void);
        impl RawPointer {
            pub fn is_cleared(&self) -> bool {
                self.0.is_null()
            }
            pub fn is_sentinel(&self) -> bool {
                false // Dummy implementation
            }
            pub fn load(&self) -> *mut std::ffi::c_void {
                self.0
            }
        }

        #[derive(Clone, Copy)]
        pub struct CompressedPointer(*mut std::ffi::c_void);
        impl CompressedPointer {
            pub fn is_cleared(&self) -> bool {
                self.0.is_null()
            }
            pub fn is_sentinel(&self) -> bool {
                false // Dummy implementation
            }
            pub fn load(&self) -> *mut std::ffi::c_void {
                self.0
            }
        }
    }

    use member_storage::*;

    #[allow(dead_code)]
    pub enum WriteBarrierSlotType {
        kUncompressed,
        kCompressed,
    }

    pub struct DijkstraWriteBarrierPolicy {}

    impl DijkstraWriteBarrierPolicy {
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _arg2: *const std::ffi::c_void) {
        }
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _storage: RawPointer) {}

        #[cfg(feature = "pointer_compression")]
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _storage: CompressedPointer) {}

        V8_INLINE! {}
        pub fn assigning_barrier<SlotType: Sized>(slot: *const std::ffi::c_void, value: *const std::ffi::c_void) {
            #[cfg(feature = "slim_write_barrier")]
            {
                if write_barrier::is_enabled() {
                    write_barrier::combined_write_barrier_slow::<SlotType>(slot);
                }
            }
            #[cfg(not(feature = "slim_write_barrier"))]
            {
                let mut params = write_barrier::Params;
                let type_ = write_barrier::get_write_barrier_type(slot, value, &mut params);
                DijkstraWriteBarrierPolicy::write_barrier(type_, &params, slot, value);
            }
        }

        V8_INLINE! {}
        pub fn assigning_barrier<SlotType: Sized>(slot: *const std::ffi::c_void, storage: RawPointer) {
            assert_eq!(
                std::any::TypeId::of::<SlotType>(),
                std::any::TypeId::of::<WriteBarrierSlotType>()
            );

            #[cfg(feature = "slim_write_barrier")]
            {
                if write_barrier::is_enabled() {
                    write_barrier::combined_write_barrier_slow::<SlotType>(slot);
                }
            }
            #[cfg(not(feature = "slim_write_barrier"))]
            {
                let mut params = write_barrier::Params;
                let type_ = write_barrier::get_write_barrier_type(slot, storage.load(), &mut params);
                DijkstraWriteBarrierPolicy::write_barrier(type_, &params, slot, storage.load());
            }
        }

        #[cfg(feature = "pointer_compression")]
        V8_INLINE! {}
        pub fn assigning_barrier<SlotType: Sized>(slot: *const std::ffi::c_void, storage: CompressedPointer) {
            assert_eq!(
                std::any::TypeId::of::<SlotType>(),
                std::any::TypeId::of::<WriteBarrierSlotType>()
            );

            #[cfg(feature = "slim_write_barrier")]
            {
                if write_barrier::is_enabled() {
                    write_barrier::combined_write_barrier_slow::<SlotType>(slot);
                }
            }
            #[cfg(not(feature = "slim_write_barrier"))]
            {
                let mut params = write_barrier::Params;
                let type_ = write_barrier::get_write_barrier_type(slot, storage.load(), &mut params);
                DijkstraWriteBarrierPolicy::write_barrier(type_, &params, slot, storage.load());
            }
        }

        V8_INLINE! {}
        fn write_barrier(
            type_: write_barrier::Type,
            params: &write_barrier::Params,
            slot: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
        ) {
            match type_ {
                write_barrier::Type::kGenerational => {
                    write_barrier::generational_barrier::<write_barrier::GenerationalBarrierType>(params, slot);
                }
                write_barrier::Type::kMarking => {
                    write_barrier::dijkstra_marking_barrier(params, value);
                }
                write_barrier::Type::kNone => {}
            }
        }
    }

    pub struct NoWriteBarrierPolicy {}

    impl NoWriteBarrierPolicy {
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _arg2: *const std::ffi::c_void) {}
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _storage: RawPointer) {}

        #[cfg(feature = "pointer_compression")]
        V8_INLINE! {}
        pub fn initializing_barrier(_arg1: *const std::ffi::c_void, _storage: CompressedPointer) {}

        V8_INLINE! {}
        pub fn assigning_barrier<SlotType: Sized>(_slot: *const std::ffi::c_void, _value: *const std::ffi::c_void) {}

        V8_INLINE! {}
        pub fn assigning_barrier<SlotType: Sized, MemberStorage>(_slot: *const std::ffi::c_void, _storage: MemberStorage) {}
    }

    pub struct SameThreadEnabledCheckingPolicyBase {
        heap_: *const HeapBase,
    }

    impl SameThreadEnabledCheckingPolicyBase {
        #[allow(unused_variables)]
        fn check_pointer_impl(
            &self,
            ptr: *const std::ffi::c_void,
            points_to_payload: bool,
            check_off_heap_assignments: bool,
        ) {
            // Implementation details omitted
        }
    }

    pub struct SameThreadEnabledCheckingPolicy<const kCheckOffHeapAssignments: bool> {
        base: SameThreadEnabledCheckingPolicyBase,
    }

    impl<const kCheckOffHeapAssignments: bool> SameThreadEnabledCheckingPolicy<kCheckOffHeapAssignments> {
        V8_INLINE! {}
        #[allow(dead_code)]
        fn check_pointer<T>(&self, raw_pointer: RawPointer) {
            if raw_pointer.is_cleared() || raw_pointer.is_sentinel() {
                return;
            }
            CheckPointersImplTrampoline::<T>::call(
                self,
                unsafe { &*(raw_pointer.load() as *const T) },
            );
        }
        #[cfg(feature = "pointer_compression")]
        V8_INLINE! {}
        #[allow(dead_code)]
        fn check_pointer<T>(&self, compressed_pointer: CompressedPointer) {
            if compressed_pointer.is_cleared() || compressed_pointer.is_sentinel() {
                return;
            }
            CheckPointersImplTrampoline::<T>::call(
                self,
                unsafe { &*(compressed_pointer.load() as *const T) },
            );
        }

        fn check_pointer<T>(&self, ptr: *const T) {
            if ptr.is_null() {
                return;
            }

            CheckPointersImplTrampoline::<T>::call(self, unsafe { &*ptr });
        }
    }

    struct CheckPointersImplTrampoline<T, const B: bool = false> {
        _phantom: PhantomData<T>,
    }

    impl<T, const B: bool> CheckPointersImplTrampoline<T, B> {
        fn call(policy: &SameThreadEnabledCheckingPolicy<false>, ptr: &T) {
            if std::any::TypeId::of::<B>() == std::any::TypeId::of::<true>() {
                policy.base.check_pointer_impl(ptr as *const T as *const std::ffi::c_void, true, false); //Assumed that true = is_garbage_collected_typev
            } else {
                policy.base.check_pointer_impl(ptr as *const T as *const std::ffi::c_void, false, false);
            }

        }
    }

    pub struct DisabledCheckingPolicy {}

    impl DisabledCheckingPolicy {
        V8_INLINE! {}
        #[allow(dead_code)]
        fn check_pointer<T>(_ptr: *mut T) {}
        V8_INLINE! {}
        #[allow(dead_code)]
        fn check_pointer<T>(_raw_pointer: RawPointer) {}

        #[cfg(feature = "pointer_compression")]
        V8_INLINE! {}
        #[allow(dead_code)]
        fn check_pointer<T>(_compressed_pointer: CompressedPointer) {}
    }

    #[cfg(feature = "enable_slow_api_checks")]
    pub type DefaultMemberCheckingPolicy = SameThreadEnabledCheckingPolicy<false>;
    #[cfg(feature = "enable_slow_api_checks")]
    pub type DefaultPersistentCheckingPolicy = SameThreadEnabledCheckingPolicy<true>;

    #[cfg(not(feature = "enable_slow_api_checks"))]
    pub type DefaultMemberCheckingPolicy = DisabledCheckingPolicy;
    #[cfg(not(feature = "enable_slow_api_checks"))]
    pub type DefaultPersistentCheckingPolicy = DisabledCheckingPolicy;

    pub type DefaultCrossThreadPersistentCheckingPolicy = DisabledCheckingPolicy;

    pub struct KeepLocationPolicy {
        location_: SourceLocation,
    }

    impl KeepLocationPolicy {
        pub const fn location(&self) -> &SourceLocation {
            &self.location_
        }
    }

    impl KeepLocationPolicy {
        const fn new() -> Self {
            Self {
                location_: SourceLocation {},
            }
        }
        const fn with_location(location: &SourceLocation) -> Self {
            Self {
                location_: location.clone(),
            }
        }
    }

    impl Default for KeepLocationPolicy {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for KeepLocationPolicy {
        fn drop(&mut self) {}
    }

    impl KeepLocationPolicy {
        fn clone(self) -> Self {
            Self {
                location_: self.location_
            }
        }
    }

    pub struct IgnoreLocationPolicy {}

    impl IgnoreLocationPolicy {
        pub const fn location(&self) -> SourceLocation {
            SourceLocation {}
        }
    }

    impl IgnoreLocationPolicy {
        const fn new() -> Self {
            Self {}
        }
        const fn with_location(_location: &SourceLocation) -> Self {
            Self {}
        }
    }

    impl Default for IgnoreLocationPolicy {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(feature = "supports_object_names")]
    pub type DefaultLocationPolicy = KeepLocationPolicy;
    #[cfg(not(feature = "supports_object_names"))]
    pub type DefaultLocationPolicy = IgnoreLocationPolicy;

    pub struct StrongPersistentPolicy {}

    impl StrongPersistentPolicy {
        pub type IsStrongPersistent = std::marker::PhantomData<TrueType>;
        extern "C" {
            pub fn get_persistent_region(object: *const std::ffi::c_void) -> PersistentRegion;
        }
    }

    pub struct WeakPersistentPolicy {}

    impl WeakPersistentPolicy {
        pub type IsStrongPersistent = std::marker::PhantomData<FalseType>;
        extern "C" {
            pub fn get_persistent_region(object: *const std::ffi::c_void) -> PersistentRegion;
        }
    }

    pub struct StrongCrossThreadPersistentPolicy {}

    impl StrongCrossThreadPersistentPolicy {
        pub type IsStrongPersistent = std::marker::PhantomData<TrueType>;
        extern "C" {
            pub fn get_persistent_region(object: *const std::ffi::c_void) -> CrossThreadPersistentRegion;
        }
    }

    pub struct WeakCrossThreadPersistentPolicy {}

    impl WeakCrossThreadPersistentPolicy {
        pub type IsStrongPersistent = std::marker::PhantomData<FalseType>;
        extern "C" {
            pub fn get_persistent_region(object: *const std::ffi::c_void) -> CrossThreadPersistentRegion;
        }
    }

    // Forward declarations setting up the default policies.
    pub struct BasicCrossThreadPersistent<T, WeaknessPolicy, LocationPolicy = DefaultLocationPolicy, CheckingPolicy = DefaultCrossThreadPersistentCheckingPolicy> {
        _phantom: PhantomData<(T, WeaknessPolicy, LocationPolicy, CheckingPolicy)>,
    }
    pub struct BasicPersistent<T, WeaknessPolicy, LocationPolicy = DefaultLocationPolicy, CheckingPolicy = DefaultPersistentCheckingPolicy> {
        _phantom: PhantomData<(T, WeaknessPolicy, LocationPolicy, CheckingPolicy)>,
    }
    pub struct BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy = DefaultMemberCheckingPolicy, StorageType = DefaultMemberStorage> {
        _phantom: PhantomData<(T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType)>,
    }

    // Dummy declarations for types not fully implemented
    pub struct HeapBase {}
    pub struct PersistentRegion {}
    pub struct CrossThreadPersistentRegion {}
    pub struct SourceLocation {}
    pub struct TrueType {}
    pub struct FalseType {}
    pub struct DefaultMemberStorage {}
}
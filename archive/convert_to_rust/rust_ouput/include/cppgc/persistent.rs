// Converted from V8 C++ source files:
// Header: persistent.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod persistent {
    use crate::internal::{
        persistent_node::PersistentNode, pointer_policies::kSentinelPointer,
        root_visitor::RootVisitor, weak_persistent_policy::WeakPersistentPolicy,
        strong_persistent_policy::StrongPersistentPolicy,
    };
    use crate::v8config::v8_clang_no_sanitize;
    use std::{marker::PhantomData, mem::ManuallyDrop};

    pub struct SourceLocation {}

    impl SourceLocation {
        pub fn Current() -> Self {
            SourceLocation {}
        }
    }

    pub mod internal {
        use super::*;

        pub trait LocationPolicy {}
        impl LocationPolicy for NoSourceLocation {}
        pub struct NoSourceLocation {}
        impl NoSourceLocation {
            pub fn new() -> Self {
                NoSourceLocation {}
            }
        }

        pub trait CheckingPolicy {
            fn check_pointer<T>(&self, _ptr: *mut T);
        }

        pub struct NoCheckingPolicy {}

        impl NoCheckingPolicy {
            pub fn new() -> Self {
                NoCheckingPolicy {}
            }
        }

        impl CheckingPolicy for NoCheckingPolicy {
            fn check_pointer<T>(&self, _ptr: *mut T) {}
        }

        pub struct PersistentRegionBase {}

        impl PersistentRegionBase {
            pub fn allocate_node<T>(
                &self,
                _owner: *mut T,
                _trace: unsafe extern "C" fn(&mut RootVisitor, *const void),
            ) -> *mut PersistentNode {
                std::ptr::null_mut() // Placeholder
            }
            pub fn free_node(&self, _node: *mut PersistentNode) {}
        }

        pub mod persistent_node {
            pub struct PersistentNode {}
        }

        pub mod root_visitor {
            pub struct RootVisitor {}
            impl RootVisitor {
                pub fn trace<T>(&mut self, _persistent: &BasicPersistent<T, StrongPersistentPolicy>) {}
                pub fn trace<T>(&mut self, _persistent: &BasicPersistent<T, WeakPersistentPolicy>) {}
            }
        }

        pub mod weak_persistent_policy {
            use super::PersistentRegionBase;

            pub struct WeakPersistentPolicy {}

            impl WeakPersistentPolicy {
                pub fn get_persistent_region<T>(_value: *const T) -> PersistentRegionBase {
                    PersistentRegionBase {}
                }
            }
        }

        pub mod strong_persistent_policy {
            use super::PersistentRegionBase;

            pub struct StrongPersistentPolicy {}

            impl StrongPersistentPolicy {
                pub fn get_persistent_region<T>(_value: *const T) -> PersistentRegionBase {
                    PersistentRegionBase {}
                }
            }
        }

        pub static kSentinelPointer: *const usize = std::ptr::null();

        pub struct BasicMember<T, WB, WT, MC, MS> {
            raw: *mut T,
            _write_barrier: PhantomData<WB>,
            _weakness_tag: PhantomData<WT>,
            _member_checking: PhantomData<MC>,
            _member_storage: PhantomData<MS>,
        }

        impl<T, WB, WT, MC, MS> BasicMember<T, WB, WT, MC, MS> {
            pub fn get(&self) -> *mut T {
                self.raw
            }
        }

        impl WeakPersistentPolicy {
            pub type IsStrongPersistent = std::marker::PhantomData<()>;
        }

        impl StrongPersistentPolicy {
            pub type IsStrongPersistent = std::marker::PhantomData<()>;
        }

        #[derive(Default)]
        pub struct PersistentBase {
            raw_: *const void,
            node_: *mut PersistentNode,
        }

        impl PersistentBase {
            pub fn new(raw: *const void) -> Self {
                PersistentBase {
                    raw_: raw,
                    node_: std::ptr::null_mut(),
                }
            }

            pub fn get_value(&self) -> *const void {
                self.raw_
            }

            pub fn set_value(&mut self, value: *const void) {
                self.raw_ = value;
            }

            pub fn get_node(&self) -> *mut PersistentNode {
                self.node_
            }

            pub fn set_node(&mut self, node: *mut PersistentNode) {
                self.node_ = node;
            }

            pub fn clear_from_gc(&mut self) {
                self.raw_ = std::ptr::null();
                self.node_ = std::ptr::null_mut();
            }
        }

        pub struct BasicPersistent<
            T,
            WeaknessPolicy = StrongPersistentPolicy,
            LocationPolicy = NoSourceLocation,
            CheckingPolicy = NoCheckingPolicy,
        > {
            persistent_base: PersistentBase,
            location_policy: LocationPolicy,
            _weakness_policy: PhantomData<WeaknessPolicy>,
            _checking_policy: PhantomData<CheckingPolicy>,
            _phantom: PhantomData<T>,
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
            BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            pub type IsStrongPersistent =
                <WeaknessPolicy as super::internal::WeaknessPolicy>::IsStrongPersistent;
            pub type PointeeType = T;

            pub fn new(loc: &SourceLocation) -> Self
            where
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                BasicPersistent {
                    persistent_base: PersistentBase::default(),
                    location_policy: LocationPolicy::new(),
                    _weakness_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _phantom: PhantomData,
                }
            }

            pub fn from_nullptr(
                _nullptr: std::option::Option<&std::convert::Infallible>,
                loc: &SourceLocation,
            ) -> Self
            where
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                BasicPersistent {
                    persistent_base: PersistentBase::default(),
                    location_policy: LocationPolicy::new(),
                    _weakness_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _phantom: PhantomData,
                }
            }

            pub fn from_sentinel(s: *const usize, loc: &SourceLocation) -> Self
            where
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                BasicPersistent {
                    persistent_base: PersistentBase::new(s as *const _),
                    location_policy: LocationPolicy::new(),
                    _weakness_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _phantom: PhantomData,
                }
            }

            pub fn from_raw(raw: *mut T, loc: &SourceLocation) -> Self
            where
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                let mut persistent = BasicPersistent {
                    persistent_base: PersistentBase::new(raw as *const _),
                    location_policy: LocationPolicy::new(),
                    _weakness_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _phantom: PhantomData,
                };

                if !persistent.is_valid() {
                    return persistent;
                }
                let node = WeaknessPolicy::get_persistent_region(persistent.get_value())
                    .allocate_node(&mut persistent as *mut _ as *mut void, Self::trace_as_root);
                persistent.persistent_base.set_node(node);
                CheckingPolicy::check_pointer(&CheckingPolicy::new(), persistent.get());
                persistent
            }

            pub fn from_ref(raw: &mut T, loc: &SourceLocation) -> Self
            where
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                Self::from_raw(raw as *mut T, loc)
            }

            pub fn from_other<
                U,
                OtherWeaknessPolicy,
                OtherLocationPolicy,
                OtherCheckingPolicy,
            >(
                other: &BasicPersistent<
                    U,
                    OtherWeaknessPolicy,
                    OtherLocationPolicy,
                    OtherCheckingPolicy,
                >,
                loc: &SourceLocation,
            ) -> Self
            where
                T: 'static,
                U: 'static,
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                Self::from_raw(other.get() as *mut T, loc)
            }

            pub fn from_member<
                U,
                MemberBarrierPolicy,
                MemberWeaknessTag,
                MemberCheckingPolicy,
                MemberStorageType,
            >(
                member: &BasicMember<
                    U,
                    MemberBarrierPolicy,
                    MemberWeaknessTag,
                    MemberCheckingPolicy,
                    MemberStorageType,
                >,
                loc: &SourceLocation,
            ) -> Self
            where
                T: 'static,
                U: 'static,
                LocationPolicy: LocationPolicy,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                Self::from_raw(member.get() as *mut T, loc)
            }

            pub fn get(&self) -> *mut T {
                unsafe {
                    let ptr = self.persistent_base.get_value() as *mut void;
                    ptr as *mut T
                }
            }

            pub fn clear(&mut self)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
            {
                if self.is_valid() {
                    let region =
                        WeaknessPolicy::get_persistent_region(self.persistent_base.get_value());
                    region.free_node(self.persistent_base.get_node());
                    self.persistent_base.set_node(std::ptr::null_mut());
                }
                self.persistent_base.set_value(std::ptr::null());
            }

            pub fn release(&mut self) -> *mut T {
                let result = self.get();
                self.clear();
                result
            }

            pub fn to<
                U,
                OtherWeaknessPolicy,
                OtherLocationPolicy,
                OtherCheckingPolicy,
            >(
                &self,
            ) -> BasicPersistent<
                U,
                OtherWeaknessPolicy,
                OtherLocationPolicy,
                OtherCheckingPolicy,
            >
            where
                OtherWeaknessPolicy: super::internal::WeaknessPolicy,
                OtherLocationPolicy: LocationPolicy,
                OtherCheckingPolicy: CheckingPolicy,
            {
                BasicPersistent::<
                    U,
                    OtherWeaknessPolicy,
                    OtherLocationPolicy,
                    OtherCheckingPolicy,
                >::from_raw(self.get() as *mut U, &SourceLocation::Current())
            }

            fn trace_as_root(root_visitor: &mut RootVisitor, ptr: *const void)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
            {
                unsafe {
                    let persistent = ptr as *const Self;
                    root_visitor.trace(&(*persistent));
                }
            }

            fn is_valid(&self) -> bool {
                self.persistent_base.get_value() != std::ptr::null()
                    && self.persistent_base.get_value() != kSentinelPointer
            }

            fn assign(&mut self, ptr: *mut T)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                if self.is_valid() {
                    if !ptr.is_null() && ptr as *const _ != kSentinelPointer {
                        self.persistent_base.set_value(ptr as *const _);
                        CheckingPolicy::check_pointer(&CheckingPolicy::new(), ptr);
                        return;
                    }
                    let region =
                        WeaknessPolicy::get_persistent_region(self.persistent_base.get_value());
                    region.free_node(self.persistent_base.get_node());
                    self.persistent_base.set_node(std::ptr::null_mut());
                }

                self.persistent_base.set_value(ptr as *const _);

                if !self.is_valid() {
                    return;
                }

                let node = WeaknessPolicy::get_persistent_region(self.persistent_base.get_value())
                    .allocate_node(&mut self as *mut _ as *mut void, Self::trace_as_root);
                self.persistent_base.set_node(node);
                CheckingPolicy::check_pointer(&CheckingPolicy::new(), self.get());
            }

            pub fn clear_from_gc(&mut self)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
            {
                if self.is_valid() {
                    let region =
                        WeaknessPolicy::get_persistent_region(self.persistent_base.get_value());
                    region.free_node(self.persistent_base.get_node());
                    self.persistent_base.clear_from_gc();
                }
            }

            #[v8_clang_no_sanitize("cfi-unrelated-cast")]
            pub fn get_from_gc(&self) -> *mut T {
                unsafe {
                    let ptr = self.persistent_base.get_value() as *mut void;
                    ptr as *mut T
                }
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> Drop
            for BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            fn drop(&mut self) {
                self.clear();
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> From<std::ptr::null_mut<T>>
            for BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            fn from(_: std::ptr::null_mut<T>) -> Self {
                BasicPersistent {
                    persistent_base: PersistentBase::default(),
                    location_policy: NoSourceLocation::new(),
                    _weakness_policy: PhantomData,
                    _checking_policy: PhantomData,
                    _phantom: PhantomData,
                }
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> std::ops::Deref
            for BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.get() }
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> std::ops::DerefMut
            for BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *self.get() }
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
            BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            pub fn as_bool(&self) -> bool {
                !self.get().is_null()
            }
        }

        impl<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
            BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>
        {
            pub fn assign_from_other<
                U,
                OtherWeaknessPolicy,
                OtherLocationPolicy,
                OtherCheckingPolicy,
            >(
                &mut self,
                other: &BasicPersistent<
                    U,
                    OtherWeaknessPolicy,
                    OtherLocationPolicy,
                    OtherCheckingPolicy,
                >,
            ) where
                T: 'static,
                U: 'static,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                self.assign(other.get() as *mut T);
            }

            pub fn assign_from_member<
                U,
                MemberBarrierPolicy,
                MemberWeaknessTag,
                MemberCheckingPolicy,
                MemberStorageType,
            >(
                &mut self,
                member: &BasicMember<
                    U,
                    MemberBarrierPolicy,
                    MemberWeaknessTag,
                    MemberCheckingPolicy,
                    MemberStorageType,
                >,
            ) where
                T: 'static,
                U: 'static,
                WeaknessPolicy: super::internal::WeaknessPolicy,
                CheckingPolicy: CheckingPolicy,
            {
                self.assign(member.get() as *mut T);
            }

            pub fn assign_from_nullptr(&mut self)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
            {
                self.clear();
            }

            pub fn assign_from_sentinel(&mut self, s: *const usize)
            where
                WeaknessPolicy: super::internal::WeaknessPolicy,
            {
                self.assign(s as *mut T);
            }
        }

        impl<
            T1,
            WeaknessPolicy1,
            LocationPolicy1,
            CheckingPolicy1,
            T2,
            WeaknessPolicy2,
            LocationPolicy2,
            CheckingPolicy2,
        > PartialEq<
            BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>,
        > for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
        {
            fn eq(
                &self,
                other: &BasicPersistent<
                    T2,
                    WeaknessPolicy2,
                    LocationPolicy2,
                    CheckingPolicy2,
                >,
            ) -> bool {
                self.get() == other.get()
            }
        }

        impl<
            T1,
            WeaknessPolicy1,
            LocationPolicy1,
            CheckingPolicy1,
            T2,
            WeaknessPolicy2,
            LocationPolicy2,
            CheckingPolicy2,
        > Eq for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
        where
            T1: Eq,
            T2: Eq,
        {
        }

        impl<
            T1,
            WeaknessPolicy1,
            LocationPolicy1,
            CheckingPolicy1,
            T2,
            WeaknessPolicy2,
            LocationPolicy2,
            CheckingPolicy2,
        > PartialOrd<
            BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>,
        > for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
        {
            fn partial_cmp(
                &self,
                other: &BasicPersistent<
                    T2,
                    WeaknessPolicy2,
                    LocationPolicy2,
                    CheckingPolicy2,
                >,
            ) -> Option<std::cmp::Ordering> {
                (self.get() as usize).partial_cmp(&(other.get() as usize))
            }
        }

        impl<
            T1,
            WeaknessPolicy1,
            LocationPolicy1,
            CheckingPolicy1,
            T2,
            WeaknessPolicy2,
            LocationPolicy2,
            CheckingPolicy2,
        > Ord for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
        where
            T1: Ord,
            T2: Ord,
        {
            fn cmp(
                &self,
                other: &BasicPersistent<
                    T2,
                    WeaknessPolicy2,
                    LocationPolicy2,
                    CheckingPolicy2,
                >,
            ) -> std::cmp::Ordering {
                (self.get() as usize).cmp(&(other.get() as usize))
            }
        }

        impl<
            T1,
            PersistentWeaknessPolicy,
            PersistentLocationPolicy,
            PersistentCheckingPolicy,
            T2,
            MemberWriteBarrierPolicy,
            MemberWeaknessTag,
            MemberCheckingPolicy,
            MemberStorageType,
        > PartialEq<
            BasicMember<
                T2,
                MemberWeaknessTag,
                MemberWriteBarrierPolicy,
                MemberCheckingPolicy,
                MemberStorageType,
            >,
        > for BasicPersistent<
            T1,
            PersistentWeaknessPolicy,
            PersistentLocationPolicy,
            PersistentCheckingPolicy,
        >
        {
            fn eq(
                &self,
                m: &BasicMember<
                    T2,
                    MemberWeaknessTag,
                    MemberWriteBarrierPolicy,
                    MemberCheckingPolicy,
                    MemberStorageType,
                >,
            ) -> bool {
                self.get() == m.get()
            }
        }

        impl<
            T1,
            PersistentWeaknessPolicy,
            PersistentLocationPolicy,
            PersistentCheckingPolicy,
            T2,
            MemberWriteBarrierPolicy,
            MemberWeaknessTag,
            MemberCheckingPolicy,
            MemberStorageType,
        > PartialEq<
            BasicPersistent<
                T1,
                PersistentWeaknessPolicy,
                PersistentLocationPolicy,
                PersistentCheckingPolicy,
            >,
        > for BasicMember<
            T2,
            MemberWeaknessTag,
            MemberWriteBarrierPolicy,
            MemberCheckingPolicy,
            MemberStorageType,
        >
        {
            fn eq(
                &self,
                p: &BasicPersistent<
                    T1,
                    PersistentWeaknessPolicy,
                    PersistentLocationPolicy,
                    PersistentCheckingPolicy,
                >,
            ) -> bool {
                self.get() == p.get()
            }
        }

        pub struct IsWeak<T> {
            _phantom: PhantomData<T>,
        }

        impl<T, LocationPolicy, CheckingPolicy> std::marker::StructuralEq
            for BasicPersistent<T, WeakPersistentPolicy, LocationPolicy, CheckingPolicy>
        {
        }
        impl<T, LocationPolicy, CheckingPolicy> std::marker::StructuralEq
            for BasicPersistent<T, StrongPersistentPolicy, LocationPolicy, CheckingPolicy>
        {
        }
    }

    pub type Persistent<T> = internal::BasicPersistent<T, StrongPersistentPolicy>;

    pub type WeakPersistent<T> = internal::BasicPersistent<T, WeakPersistentPolicy>;

    type void = ();
}

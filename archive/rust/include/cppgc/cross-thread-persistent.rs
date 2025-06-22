// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicPtr, Ordering};
use std::marker::PhantomData;
//use std::mem::MaybeUninit;

//use crate::cppgc::internal::persistent_node::PersistentNode;
//use crate::cppgc::internal::pointer_policies::PointerPolicies;
//use crate::cppgc::persistent::Persistent;
//use crate::cppgc::visitor::Visitor;
//use crate::cppgc::source_location::SourceLocation;
//use crate::cppgc::internal::persistent_region::{PersistentRegionLock, CrossThreadPersistentRegion};

pub mod internal {

    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::marker::PhantomData;
    use std::ptr::null_mut;
    use crate::*;

    // Placeholder types, replace with actual implementations
    pub struct PersistentNode {}

    pub struct CrossThreadPersistentRegion {}

    impl CrossThreadPersistentRegion {
        pub fn FreeNode(&self, _node: *mut PersistentNode) {}
        pub fn AllocateNode<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>(
            &self,
            _owner: *const BasicCrossThreadPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy>,
            _trace_func: &dyn Fn(RootVisitor, *const std::ffi::c_void),
        ) -> *mut PersistentNode {
            null_mut()
        }
    }

    pub struct PersistentRegionLock {}
    impl PersistentRegionLock {
        pub fn new() -> Self { Self{} }
    }

    pub trait PointerPolicies {}

    pub struct WeakCrossThreadPersistentPolicy {}
    impl WeakCrossThreadPersistentPolicy {
        pub fn GetPersistentRegion(_ptr: *const std::ffi::c_void) -> CrossThreadPersistentRegion {
            CrossThreadPersistentRegion {}
        }
    }
    pub struct StrongCrossThreadPersistentPolicy {}

    pub struct NoCheckingPolicy {}

    pub struct SourceLocation {}
    impl SourceLocation {
        pub fn Current() -> Self { Self{} }
    }

    // Wrapper around PersistentBase that allows accessing poisoned memory when
    // using ASAN. This is needed as the GC of the heap that owns the value
    // of a CTP, may clear it (heap termination, weakness) while the object
    // holding the CTP may be poisoned as itself may be deemed dead.
    pub struct CrossThreadPersistentBase {
        raw_: *const std::ffi::c_void,
        node_: *mut PersistentNode,
    }

    impl CrossThreadPersistentBase {
        pub fn new() -> Self {
            Self {
                raw_: std::ptr::null(),
                node_: std::ptr::null_mut(),
            }
        }

        pub fn from_raw(raw: *const std::ffi::c_void) -> Self {
            Self {
                raw_: raw,
                node_: std::ptr::null_mut(),
            }
        }

        //#[cfg(not(feature = "address_sanitizer"))]
        pub fn GetValueFromGC(&self) -> *const std::ffi::c_void {
            self.raw_
        }

        //#[cfg(not(feature = "address_sanitizer"))]
        pub fn GetNodeFromGC(&self) -> *mut PersistentNode {
            self.node_
        }

        //#[cfg(not(feature = "address_sanitizer"))]
        pub fn ClearFromGC(&self) {
            self.raw_ = std::ptr::null();
            self.SetNodeSafe(std::ptr::null_mut());
        }

        // GetNodeSafe() can be used for a thread-safe IsValid() check in a
        // double-checked locking pattern. See ~BasicCrossThreadPersistent.
        pub fn GetNodeSafe(&self) -> *mut PersistentNode {
            let atomic_node = unsafe { &*(std::ptr::addr_of!(self.node_) as *const AtomicPtr<PersistentNode>) };
            atomic_node.load(Ordering::Acquire)
        }

        // The GC writes using SetNodeSafe() while holding the lock.
        //#[cfg(not(feature = "address_sanitizer"))]
        pub fn SetNodeSafe(&self, value: *mut PersistentNode) {
            let atomic_node = unsafe { &*(std::ptr::addr_of!(self.node_) as *const AtomicPtr<PersistentNode>) };
            atomic_node.store(value, Ordering::Release);
        }
    }

    pub trait WeaknessPolicy {
        type IsStrongPersistent;
        fn GetPersistentRegion(ptr: *const std::ffi::c_void) -> CrossThreadPersistentRegion;
    }

    pub trait LocationPolicy {}

    pub trait CheckingPolicy {}

    pub struct BasicCrossThreadPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
    where
        WeaknessPolicyT: WeaknessPolicy,
        LocationPolicyT: LocationPolicy,
        CheckingPolicyT: CheckingPolicy,
    {
        base: CrossThreadPersistentBase,
        location_policy: LocationPolicyT,
        weakness_policy: PhantomData<WeaknessPolicyT>,
        checking_policy: PhantomData<CheckingPolicyT>,
        _marker: PhantomData<T>,
    }

    impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> Drop for BasicCrossThreadPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
    where
        WeaknessPolicyT: WeaknessPolicy,
        LocationPolicyT: LocationPolicy,
        CheckingPolicyT: CheckingPolicy,
    {
        fn drop(&mut self) {
            //  This implements fast path for destroying empty/sentinel.
            //
            // Simplified version of `AssignUnsafe()` to allow calling without a
            // complete type `T`. Uses double-checked locking with a simple thread-safe
            // check for a valid handle based on a node.
            if !self.base.GetNodeSafe().is_null() {
                let guard = PersistentRegionLock::new();
                let old_value = self.GetValue();
                // The fast path check (GetNodeSafe()) does not acquire the lock. Recheck
                // validity while holding the lock to ensure the reference has not been
                // cleared.
                if Self::IsValid(old_value) {
                    let region = WeaknessPolicyT::GetPersistentRegion(old_value);
                    region.FreeNode(self.GetNode());
                    self.SetNode(std::ptr::null_mut());
                } else {
                    //CPPGC_DCHECK(!GetNode());
                    assert!(self.GetNode().is_null());
                }
            }
            // No need to call SetValue() as the handle is not used anymore. This can
            // leave behind stale sentinel values but will always destroy the underlying
            // node.
        }
    }

    impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> BasicCrossThreadPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
    where
        WeaknessPolicyT: WeaknessPolicy,
        LocationPolicyT: LocationPolicy,
        CheckingPolicyT: CheckingPolicy,
    {
        pub type IsStrongPersistent = bool;
        pub type PointeeType = T;

        pub fn new(loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            Self {
                base: CrossThreadPersistentBase::new(),
                location_policy: LocationPolicyT::default(),
                weakness_policy: PhantomData,
                checking_policy: PhantomData,
                _marker: PhantomData,
            }
        }

        pub fn from_nullptr(loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            Self {
                base: CrossThreadPersistentBase::new(),
                location_policy: LocationPolicyT::default(),
                weakness_policy: PhantomData,
                checking_policy: PhantomData,
                _marker: PhantomData,
            }
        }

        pub fn from_sentinel(s: SentinelPointer, loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            Self {
                base: CrossThreadPersistentBase::from_raw(s as *const std::ffi::c_void),
                location_policy: LocationPolicyT::default(),
                weakness_policy: PhantomData,
                checking_policy: PhantomData,
                _marker: PhantomData,
            }
        }

        pub fn from_raw(raw: *mut T, loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            let mut me = Self {
                base: CrossThreadPersistentBase::from_raw(raw as *const std::ffi::c_void),
                location_policy: LocationPolicyT::default(),
                weakness_policy: PhantomData,
                checking_policy: PhantomData,
                _marker: PhantomData,
            };
            if !Self::IsValid(raw as *const std::ffi::c_void) {
                return me;
            }
            let guard = PersistentRegionLock::new();
            let region = WeaknessPolicyT::GetPersistentRegion(raw as *const std::ffi::c_void);
            me.SetNode(region.AllocateNode(
                &me as *const Self,
                &Self::TraceAsRoot,
            ));
            me.CheckPointer(raw);
            me
        }

        struct UnsafeCtorTag {}

        pub fn from_raw_unsafe(tag: UnsafeCtorTag, raw: *mut T, loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            let mut me = Self {
                base: CrossThreadPersistentBase::from_raw(raw as *const std::ffi::c_void),
                location_policy: LocationPolicyT::default(),
                weakness_policy: PhantomData,
                checking_policy: PhantomData,
                _marker: PhantomData,
            };
            if !Self::IsValid(raw as *const std::ffi::c_void) {
                return me;
            }
            let region = WeaknessPolicyT::GetPersistentRegion(raw as *const std::ffi::c_void);
            me.SetNode(region.AllocateNode(
                &me as *const Self,
                &Self::TraceAsRoot,
            ));
            me.CheckPointer(raw);
            me
        }

        pub fn from_ref(raw: &mut T, loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            Self::from_raw(raw, loc)
        }

        // BasicCrossThreadPersistent(
        //     internal::BasicMember<U, MemberBarrierPolicy, MemberWeaknessTag,
        //                           MemberCheckingPolicy, MemberStorageType>
        //         member,
        //     const SourceLocation& loc = SourceLocation::Current())
        //     : BasicCrossThreadPersistent(member.Get(), loc) {}

        pub fn copy(other: &Self, loc: SourceLocation) -> Self
        where LocationPolicyT: Default + Clone
        {
            let mut me = Self::new(loc);
            // Invoke operator=.
            me = me.assign(other);
            me
        }

        // Heterogeneous ctor.
        // template <typename U, typename OtherWeaknessPolicy,
        //           typename OtherLocationPolicy, typename OtherCheckingPolicy,
        //           typename = std::enable_if_t<std::is_base_of<T, U>::value>>
        // BasicCrossThreadPersistent(
        //     const BasicCrossThreadPersistent<U, OtherWeaknessPolicy,
        //                                      OtherLocationPolicy,
        //                                      OtherCheckingPolicy>& other,
        //     const SourceLocation& loc = SourceLocation::Current())
        //     : BasicCrossThreadPersistent(loc) {
        //   *this = other;
        // }

        pub fn from_moved(other: &mut Self, loc: SourceLocation) -> Self
        where LocationPolicyT: Default
        {
            let mut me = Self::new(loc);
            // Invoke operator=.
            me = std::mem::take(&mut me).assign_moved(other);
            me
        }

        pub fn assign(&mut self, other: &Self) -> Self {
            let guard = PersistentRegionLock::new();
            self.AssignSafe(guard, other.Get());
            std::mem::take(self)
        }

        // template <typename U, typename OtherWeaknessPolicy,
        //           typename OtherLocationPolicy, typename OtherCheckingPolicy,
        //           typename = std::enable_if_t<std::is_base_of<T, U>::value>>
        // BasicCrossThreadPersistent& operator=(
        //     const BasicCrossThreadPersistent<U, OtherWeaknessPolicy,
        //                                      OtherLocationPolicy,
        //                                      OtherCheckingPolicy>& other) {
        //   PersistentRegionLock guard;
        //   AssignSafe(guard, other.Get());
        //   return *this;
        // }

        pub fn assign_moved(&mut self, other: &mut Self) -> Self {
            if self as *mut Self == other as *mut Self {
                return std::mem::take(self);
            }
            self.Clear();
            let guard = PersistentRegionLock::new();
            self.base = std::mem::take(&mut other.base);
            //LocationPolicy::operator=(std::move(other));
            //TODO: move LocationPolicy
            if !Self::IsValid(self.GetValue()) {
                return std::mem::take(self);
            }
            self.GetNode().as_mut().unwrap().UpdateOwner(self);
            other.SetValue(std::ptr::null_mut());
            other.SetNode(std::ptr::null_mut());
            self.CheckPointer(self.Get());
            std::mem::take(self)
        }

        /**
         * Assigns a raw pointer.
         *
         * Note: **Not thread-safe.**
         */
        pub fn assign_raw(&mut self, other: *mut T) -> Self {
            self.AssignUnsafe(other);
            std::mem::take(self)
        }

        // Assignment from member.
        // template <typename U, typename MemberBarrierPolicy,
        //           typename MemberWeaknessTag, typename MemberCheckingPolicy,
        //           typename MemberStorageType,
        //           typename = std::enable_if_t<std::is_base_of<T, U>::value>>
        // BasicCrossThreadPersistent& operator=(
        //     internal::BasicMember<U, MemberBarrierPolicy, MemberWeaknessTag,
        //                           MemberCheckingPolicy, MemberStorageType>
        //         member) {
        //   return operator=(member.Get());
        // }

        /**
         * Assigns a nullptr.
         *
         * \returns the handle.
         */
        pub fn assign_nullptr(&mut self) -> Self {
            self.Clear();
            std::mem::take(self)
        }

        /**
         * Assigns the sentinel pointer.
         *
         * \returns the handle.
         */
        pub fn assign_sentinel(&mut self, s: SentinelPointer) -> Self {
            let guard = PersistentRegionLock::new();
            self.AssignSafe(guard, s as *mut T);
            std::mem::take(self)
        }

        /**
         * Returns a pointer to the stored object.
         *
         * Note: **Not thread-safe.**
         *
         * \returns a pointer to the stored object.
         */
        // CFI cast exemption to allow passing SentinelPointer through T* and support
        // heterogeneous assignments between different Member and Persistent handles
        // based on their actual types.
        pub fn Get(&self) -> *mut T {
            self.GetValue() as *mut T
        }

        /**
         * Clears the stored object.
         */
        pub fn Clear(&mut self) {
            let guard = PersistentRegionLock::new();
            self.AssignSafe(guard, std::ptr::null_mut());
        }

        /**
         * Returns a pointer to the stored object and releases it.
         *
         * Note: **Not thread-safe.**
         *
         * \returns a pointer to the stored object.
         */
        pub fn Release(&mut self) -> *mut T {
            let result = self.Get();
            self.Clear();
            result
        }

        /**
         * Conversio to boolean.
         *
         * Note: **Not thread-safe.**
         *
         * \returns true if an actual object has been stored and false otherwise.
         */
        pub fn as_bool(&self) -> bool {
            !self.Get().is_null()
        }

        /**
         * Conversion to object of type T.
         *
         * Note: **Not thread-safe.**
         *
         * \returns the object.
         */
        // operator T*() const { return Get(); }

        /**
         * Dereferences the stored object.
         *
         * Note: **Not thread-safe.**
         */
        // T* operator->() const { return Get(); }
        // T& operator*() const { return *Get(); }

        // template <typename U, typename OtherWeaknessPolicy = WeaknessPolicy,
        //           typename OtherLocationPolicy = LocationPolicy,
        //           typename OtherCheckingPolicy = CheckingPolicy>
        // BasicCrossThreadPersistent<U, OtherWeaknessPolicy, OtherLocationPolicy,
        //                            OtherCheckingPolicy>
        // To() const {
        //   using OtherBasicCrossThreadPersistent =
        //       BasicCrossThreadPersistent<U, OtherWeaknessPolicy, OtherLocationPolicy,
        //                                  OtherCheckingPolicy>;
        //   PersistentRegionLock guard;
        //   return OtherBasicCrossThreadPersistent(
        //       typename OtherBasicCrossThreadPersistent::UnsafeCtorTag(),
        //       static_cast<U*>(Get()));
        // }

        // template <typename U = T,
        //           typename = typename std::enable_if<!BasicCrossThreadPersistent<
        //               U, WeaknessPolicy>::IsStrongPersistent::value>::type>
        // BasicCrossThreadPersistent<U, internal::StrongCrossThreadPersistentPolicy>
        // Lock() const {
        //   return BasicCrossThreadPersistent<
        //       U, internal::StrongCrossThreadPersistentPolicy>(*this);
        // }

        fn IsValid(ptr: *const std::ffi::c_void) -> bool {
            !ptr.is_null() && ptr != kSentinelPointer as *const std::ffi::c_void
        }

        fn TraceAsRoot(root_visitor: RootVisitor, ptr: *const std::ffi::c_void) {
            root_visitor.Trace(unsafe { &*(ptr as *const Self) });
        }

        fn AssignUnsafe(&mut self, ptr: *mut T) {
            let old_value = self.GetValue();
            if Self::IsValid(old_value) {
                let guard = PersistentRegionLock::new();
                let old_value = self.GetValue();
                // The fast path check (IsValid()) does not acquire the lock. Reload
                // the value to ensure the reference has not been cleared.
                if Self::IsValid(old_value) {
                    let region = WeaknessPolicyT::GetPersistentRegion(old_value);
                    if Self::IsValid(ptr as *const std::ffi::c_void) && (&region as *const CrossThreadPersistentRegion == &WeaknessPolicyT::GetPersistentRegion(ptr as *const std::ffi::c_void) as *const CrossThreadPersistentRegion) {
                        self.SetValue(ptr);
                        self.CheckPointer(ptr);
                        return;
                    }
                    region.FreeNode(self.GetNode());
                    self.SetNode(std::ptr::null_mut());
                } else {
                    //CPPGC_DCHECK(!GetNode());
                    assert!(self.GetNode().is_null());
                }
            }
            self.SetValue(ptr);
            if !Self::IsValid(ptr as *const std::ffi::c_void) {
                return;
            }
            let guard = PersistentRegionLock::new();
            let region = WeaknessPolicyT::GetPersistentRegion(ptr as *const std::ffi::c_void);
            self.SetNode(region.AllocateNode(
                self as *const Self,
                &Self::TraceAsRoot,
            ));
            self.CheckPointer(ptr);
        }

        fn AssignSafe(&mut self, _guard: PersistentRegionLock, ptr: *mut T) {
            //PersistentRegionLock::AssertLocked(); //TODO: Implement AssertLocked
            let old_value = self.GetValue();
            if Self::IsValid(old_value) {
                let region = WeaknessPolicyT::GetPersistentRegion(old_value);
                if Self::IsValid(ptr as *const std::ffi::c_void) && (&region as *const CrossThreadPersistentRegion == &WeaknessPolicyT::GetPersistentRegion(ptr as *const std::ffi::c_void) as *const CrossThreadPersistentRegion) {
                    self.SetValue(ptr);
                    self.CheckPointer(ptr);
                    return;
                }
                region.FreeNode(self.GetNode());
                self.SetNode(std::ptr::null_mut());
            }
            self.SetValue(ptr);
            if !Self::IsValid(ptr as *const std::ffi::c_void) {
                return;
            }
            let region = WeaknessPolicyT::GetPersistentRegion(ptr as *const std::ffi::c_void);
            self.SetNode(region.AllocateNode(
                self as *const Self,
                &Self::TraceAsRoot,
            ));
            self.CheckPointer(ptr);
        }

        fn ClearFromGC(&self) {
            if Self::IsValid(self.base.GetValueFromGC()) {
                WeaknessPolicyT::GetPersistentRegion(self.base.GetValueFromGC())
                    .FreeNode(self.base.GetNodeFromGC());
                self.base.ClearFromGC();
            }
        }

        // See Get() for details.
        //V8_CLANG_NO_SANITIZE("cfi-unrelated-cast")
        fn GetFromGC(&self) -> *mut T {
            self.base.GetValueFromGC() as *mut T
        }

        fn GetValue(&self) -> *const std::ffi::c_void {
            self.base.raw_
        }

        fn SetValue(&mut self, value: *mut T) {
            self.base.raw_ = value as *const std::ffi::c_void;
        }

        fn GetNode(&self) -> *mut PersistentNode {
            self.base.node_
        }

        fn SetNode(&mut self, node: *mut PersistentNode) {
            self.base.node_ = node;
        }

        fn CheckPointer(&self, _ptr: *mut T) {}

        fn GetPersistentRegion(_ptr: *const std::ffi::c_void) -> CrossThreadPersistentRegion {
            CrossThreadPersistentRegion {}
        }
    }

    pub struct RootVisitor {}
    impl RootVisitor {
        pub fn Trace<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>(&self, _persistent: &BasicCrossThreadPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>)
        where
            WeaknessPolicyT: WeaknessPolicy,
            LocationPolicyT: LocationPolicy,
            CheckingPolicyT: CheckingPolicy,
        {}
    }

    impl WeaknessPolicy for WeakCrossThreadPersistentPolicy {
        type IsStrongPersistent = std::marker::PhantomData<()>;
        fn GetPersistentRegion(ptr: *const std::ffi::c_void) -> CrossThreadPersistentRegion {
            Self::GetPersistentRegion(ptr)
        }
    }
    impl WeaknessPolicy for StrongCrossThreadPersistentPolicy {
        type IsStrongPersistent = bool;
        fn GetPersistentRegion(ptr: *const std::ffi::c_void) -> CrossThreadPersistentRegion {
            Self::GetPersistentRegion(ptr)
        }
    }

    impl LocationPolicy for SourceLocation {}
    impl Default for SourceLocation {
        fn default() -> Self {
            Self::Current()
        }
    }

    impl CheckingPolicy for NoCheckingPolicy {}
    impl Default for NoCheckingPolicy {
        fn default() -> Self {
            Self{}
        }
    }

    pub struct IsWeak<T>(PhantomData<T>);

    impl<T, LocationPolicyT, CheckingPolicyT> IsWeak<BasicCrossThreadPersistent<T, WeakCrossThreadPersistentPolicy, LocationPolicyT, CheckingPolicyT>>
    where
        LocationPolicyT: LocationPolicy,
        CheckingPolicyT: CheckingPolicy,
    {
        pub const VALUE: bool = true;
    }
}  // namespace internal

pub mod subtle {
    use crate::internal::*;

    /// **DO NOT USE: Has known caveats, see below.**
    ///
    /// CrossThreadPersistent allows retaining objects from threads other than the
    /// thread the owning heap is operating on.
    ///
    /// Known caveats:
    /// - Does not protect the heap owning an object from terminating.
    /// - Reaching transitively through the graph is unsupported as objects may be
    ///   moved concurrently on the thread owning the object.
    pub type CrossThreadPersistent<T> =
        internal::BasicCrossThreadPersistent<T, internal::StrongCrossThreadPersistentPolicy, SourceLocation, NoCheckingPolicy>;

    /// **DO NOT USE: Has known caveats, see below.**
    ///
    /// CrossThreadPersistent allows weakly retaining objects from threads other than
    /// the thread the owning heap is operating on.
    ///
    /// Known caveats:
    /// - Does not protect the heap owning an object from terminating.
    /// - Reaching transitively through the graph is unsupported as objects may be
    ///   moved concurrently on the thread owning the object.
    pub type WeakCrossThreadPersistent<T> =
        internal::BasicCrossThreadPersistent<T, internal::WeakCrossThreadPersistentPolicy, SourceLocation, NoCheckingPolicy>;
}  // namespace subtle

pub type SentinelPointer = *const std::ffi::c_void;
pub const kSentinelPointer: SentinelPointer = 1 as *const std::ffi::c_void;
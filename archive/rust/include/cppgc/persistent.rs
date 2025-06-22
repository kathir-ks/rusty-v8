// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

mod internal {
    pub mod persistent_node;
    pub mod pointer_policies;
}

mod source_location;
pub use source_location::SourceLocation;

mod type_traits;

pub mod visitor;

// Placeholder for v8config.h, replace with appropriate Rust configuration
// const V8_CLANG_NO_SANITIZE_CFI_UNRELATED_CAST: &str = "cfi-unrelated-cast";

// Placeholder for SentinelPointer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SentinelPointer;

pub struct PersistentBase {
    raw: *const std::ffi::c_void,
    node: *mut internal::persistent_node::PersistentNode,
}

impl PersistentBase {
    pub fn new() -> Self {
        PersistentBase {
            raw: std::ptr::null(),
            node: std::ptr::null_mut(),
        }
    }

    pub fn from_raw(raw: *const std::ffi::c_void) -> Self {
        PersistentBase { raw, node: std::ptr::null_mut() }
    }

    pub fn get_value(&self) -> *const std::ffi::c_void {
        self.raw
    }

    pub fn set_value(&mut self, value: *const std::ffi::c_void) {
        self.raw = value;
    }

    pub fn get_node(&self) -> *mut internal::persistent_node::PersistentNode {
        self.node
    }

    pub fn set_node(&mut self, node: *mut internal::persistent_node::PersistentNode) {
        self.node = node;
    }

    pub fn clear_from_gc(&mut self) {
        self.raw = std::ptr::null();
        self.node = std::ptr::null_mut();
    }
}

pub trait WeaknessPolicy {
    type IsStrongPersistent: std::marker::Sized;
    fn get_persistent_region(ptr: *const std::ffi::c_void) -> PersistentRegionBase;
}

pub trait LocationPolicy {
    fn location(&self) -> &SourceLocation;
}

pub trait CheckingPolicy {
    fn check_pointer<T>(&self, ptr: *mut T);
}

pub struct BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    base: PersistentBase,
    location_policy: LocationPolicyT,
    weakness_policy: PhantomData<WeaknessPolicyT>,
    checking_policy: PhantomData<CheckingPolicyT>,
    _phantom: PhantomData<T>,
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
    BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    pub type PointeeType = T;
    pub type IsStrongPersistent = WeaknessPolicyT::IsStrongPersistent;

    pub fn new(loc: &SourceLocation) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default
    {
        BasicPersistent {
            base: PersistentBase::new(),
            location_policy: LocationPolicyT::default(),
            weakness_policy: PhantomData,
            checking_policy: PhantomData,
            _phantom: PhantomData,
        }
    }

    pub fn from_nullptr(
        _null: std::ptr::NonNull<()>,
        loc: &SourceLocation,
    ) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default
    {
        BasicPersistent {
            base: PersistentBase::new(),
            location_policy: LocationPolicyT::default(),
            weakness_policy: PhantomData,
            checking_policy: PhantomData,
            _phantom: PhantomData,
        }
    }

    pub fn from_sentinel(
        s: SentinelPointer,
        loc: &SourceLocation,
    ) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default
    {
        BasicPersistent {
            base: PersistentBase::from_raw(&s as *const SentinelPointer as *const std::ffi::c_void),
            location_policy: LocationPolicyT::default(),
            weakness_policy: PhantomData,
            checking_policy: PhantomData,
            _phantom: PhantomData,
        }
    }

    pub fn from_raw(raw: *mut T, loc: &SourceLocation) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default
    {
        let mut persistent = BasicPersistent {
            base: PersistentBase::from_raw(raw as *const std::ffi::c_void),
            location_policy: LocationPolicyT::default(),
            weakness_policy: PhantomData,
            checking_policy: PhantomData,
            _phantom: PhantomData,
        };
        if !persistent.is_valid() {
            return persistent;
        }
        let region = WeaknessPolicyT::get_persistent_region(persistent.base.get_value());
        // TODO: Implement allocate_node, trace_as_root correctly
        let node = region.allocate_node(&persistent, Self::trace_as_root);
        persistent.base.set_node(node);
        persistent.check_pointer(persistent.get());
        persistent
    }

    pub fn from_ref(raw: &mut T, loc: &SourceLocation) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default
    {
        Self::from_raw(raw as *mut T, loc)
    }

    pub fn from_basic_persistent<U, OtherWeaknessPolicy, OtherLocationPolicy, OtherCheckingPolicy>(
        other: &BasicPersistent<U, OtherWeaknessPolicy, OtherLocationPolicy, OtherCheckingPolicy>,
        loc: &SourceLocation,
    ) -> Self
    where
        LocationPolicyT: Default,
        CheckingPolicyT: Default,
        U: std::marker::Sized,
        OtherWeaknessPolicy: WeaknessPolicy,
        OtherLocationPolicy: LocationPolicy,
        OtherCheckingPolicy: CheckingPolicy,
    {
        Self::from_raw(other.get() as *mut T, loc)
    }

    pub fn to<U, OtherWeaknessPolicy, OtherLocationPolicy, OtherCheckingPolicy>(
        &self,
    ) -> BasicPersistent<U, OtherWeaknessPolicy, OtherLocationPolicy, OtherCheckingPolicy>
    where
        U: std::marker::Sized,
        OtherWeaknessPolicy: WeaknessPolicy,
        OtherLocationPolicy: LocationPolicy,
        OtherCheckingPolicy: CheckingPolicy,
    {
        BasicPersistent::from_raw(self.get() as *mut U, self.location_policy.location())
    }

    pub fn clear(&mut self) {
        if self.is_valid() {
            let region = WeaknessPolicyT::get_persistent_region(self.base.get_value());
            region.free_node(self.base.get_node());
            self.base.set_node(std::ptr::null_mut());
        }
        self.base.set_value(std::ptr::null());
    }

    pub fn release(&mut self) -> *mut T {
        let result = self.get();
        self.clear();
        result
    }

    fn trace_as_root(root_visitor: &mut visitor::RootVisitor, ptr: *const std::ffi::c_void) {
        root_visitor.trace(unsafe { &*(ptr as *const Self) });
    }

    fn is_valid(&self) -> bool {
        self.base.get_value() != std::ptr::null()
            && self.base.get_value() as *const SentinelPointer != &SentinelPointer as *const SentinelPointer as *const std::ffi::c_void
    }

    fn assign(&mut self, ptr: *mut T) {
        if self.is_valid() {
            if !ptr.is_null() && ptr as *const SentinelPointer != &SentinelPointer as *const SentinelPointer as *const T {
                self.base.set_value(ptr as *const std::ffi::c_void);
                self.check_pointer(ptr);
                return;
            }
            let region = WeaknessPolicyT::get_persistent_region(self.base.get_value());
            region.free_node(self.base.get_node());
            self.base.set_node(std::ptr::null_mut());
        }
        self.base.set_value(ptr as *const std::ffi::c_void);
        if !self.is_valid() {
            return;
        }

        let region = WeaknessPolicyT::get_persistent_region(self.base.get_value());
        let node = region.allocate_node(self, Self::trace_as_root);
        self.base.set_node(node);
        self.check_pointer(ptr);
    }

    fn clear_from_gc(&mut self) {
        if self.is_valid() {
            let region = WeaknessPolicyT::get_persistent_region(self.base.get_value());
            region.free_node(self.base.get_node());
            self.base.clear_from_gc();
        }
    }

    // #[cfg(V8_CLANG_NO_SANITIZE_CFI_UNRELATED_CAST)]
    fn get_from_gc(&self) -> *mut T {
        self.base.get_value() as *mut T
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    // #[cfg(V8_CLANG_NO_SANITIZE_CFI_UNRELATED_CAST)]
    pub fn get(&self) -> *mut T {
        self.base.get_value() as *mut T
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> Drop
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> Deref
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.get() }
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> DerefMut
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.get() }
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> LocationPolicy
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    fn location(&self) -> &SourceLocation {
        self.location_policy.location()
    }
}

impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> CheckingPolicy
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    fn check_pointer<U>(&self, ptr: *mut U) {
        self.checking_policy.check_pointer(ptr)
    }
}

// Implement Default trait if LocationPolicyT and CheckingPolicyT implements Default
impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT> Default
    for BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy + Default,
    CheckingPolicyT: CheckingPolicy + Default,
{
    fn default() -> Self {
        BasicPersistent {
            base: PersistentBase::new(),
            location_policy: LocationPolicyT::default(),
            weakness_policy: PhantomData,
            checking_policy: PhantomData,
            _phantom: PhantomData,
        }
    }
}

//Implement Assign
impl<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
    BasicPersistent<T, WeaknessPolicyT, LocationPolicyT, CheckingPolicyT>
where
    WeaknessPolicyT: WeaknessPolicy,
    LocationPolicyT: LocationPolicy,
    CheckingPolicyT: CheckingPolicy,
{
    pub fn assign(&mut self, other: *mut T) -> &mut Self {
        self.assign(other);
        self
    }
}

// Placeholder structs and enums, replace with actual implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistentRegionBase;

impl PersistentRegionBase {
    pub fn allocate_node<F>(&self, owner: *const std::ffi::c_void, trace: F) -> *mut internal::persistent_node::PersistentNode
        where F: Fn(&mut visitor::RootVisitor, *const std::ffi::c_void) {
        // TODO: Implement the correct allocation and tracing logic
        std::ptr::null_mut()
    }
    pub fn free_node(&self, node: *mut internal::persistent_node::PersistentNode) {
        // TODO: Implement the correct freeing logic
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrongPersistentPolicy;

impl WeaknessPolicy for StrongPersistentPolicy {
    type IsStrongPersistent = std::marker::PhantomData<()>;
    fn get_persistent_region(_ptr: *const std::ffi::c_void) -> PersistentRegionBase {
        // TODO: Implement the correct region retrieval logic
        PersistentRegionBase
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeakPersistentPolicy;

impl WeaknessPolicy for WeakPersistentPolicy {
    type IsStrongPersistent = ();
    fn get_persistent_region(_ptr: *const std::ffi::c_void) -> PersistentRegionBase {
        // TODO: Implement the correct region retrieval logic
        PersistentRegionBase
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DefaultLocationPolicy {
    location: SourceLocation,
}

impl LocationPolicy for DefaultLocationPolicy {
    fn location(&self) -> &SourceLocation {
        &self.location
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DefaultCheckingPolicy;

impl CheckingPolicy for DefaultCheckingPolicy {
    fn check_pointer<T>(&self, _ptr: *mut T) {
        // TODO: Implement the correct checking logic
    }
}

// Equality operators
impl<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1, T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>
    PartialEq<BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>>
    for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
where
    WeaknessPolicy1: WeaknessPolicy,
    LocationPolicy1: LocationPolicy,
    CheckingPolicy1: CheckingPolicy,
    WeaknessPolicy2: WeaknessPolicy,
    LocationPolicy2: LocationPolicy,
    CheckingPolicy2: CheckingPolicy,
{
    fn eq(&self, other: &BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

impl<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1, T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>
    PartialEq<&BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>>
    for BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>
where
    WeaknessPolicy1: WeaknessPolicy,
    LocationPolicy1: LocationPolicy,
    CheckingPolicy1: CheckingPolicy,
    WeaknessPolicy2: WeaknessPolicy,
    LocationPolicy2: LocationPolicy,
    CheckingPolicy2: CheckingPolicy,
{
    fn eq(&self, other: &&BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

impl<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1, T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>
    PartialEq<BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>>
    for &BasicPersistent<T2, WeaknessPolicy2, LocationPolicy2, CheckingPolicy2>
where
    WeaknessPolicy1: WeaknessPolicy,
    LocationPolicy1: LocationPolicy,
    CheckingPolicy1: CheckingPolicy,
    WeaknessPolicy2: WeaknessPolicy,
    LocationPolicy2: LocationPolicy,
    CheckingPolicy2: CheckingPolicy,
{
    fn eq(&self, other: &BasicPersistent<T1, WeaknessPolicy1, LocationPolicy1, CheckingPolicy1>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

// Define Member traits and structs, if needed
// struct BasicMember...

// Implement comparison operators between BasicPersistent and BasicMember, if BasicMember is defined

mod member {
    // Placeholder for BasicMember
    pub struct BasicMember<T> {
        ptr: *mut T,
    }

    impl<T> BasicMember<T> {
        pub fn get(&self) -> *mut T {
            self.ptr
        }
    }
}

use member::BasicMember;

// Implement comparison operators between BasicPersistent and BasicMember
impl<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy, T2>
    PartialEq<BasicMember<T2>>
    for BasicPersistent<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy>
where
    PersistentWeaknessPolicy: WeaknessPolicy,
    PersistentLocationPolicy: LocationPolicy,
    PersistentCheckingPolicy: CheckingPolicy,
{
    fn eq(&self, other: &BasicMember<T2>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

impl<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy, T2>
    PartialEq<&BasicMember<T2>>
    for BasicPersistent<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy>
where
    PersistentWeaknessPolicy: WeaknessPolicy,
    PersistentLocationPolicy: LocationPolicy,
    PersistentCheckingPolicy: CheckingPolicy,
{
    fn eq(&self, other: &&BasicMember<T2>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

impl<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy, T2>
    PartialEq<BasicPersistent<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy>>
    for &BasicMember<T2>
where
    PersistentWeaknessPolicy: WeaknessPolicy,
    PersistentLocationPolicy: LocationPolicy,
    PersistentCheckingPolicy: CheckingPolicy,
{
    fn eq(&self, other: &BasicPersistent<T1, PersistentWeaknessPolicy, PersistentLocationPolicy, PersistentCheckingPolicy>) -> bool {
        self.get() as *const std::ffi::c_void == other.get() as *const std::ffi::c_void
    }
}

// Add corresponding impls for != if needed.

mod is_weak {
    use super::*;
    use std::marker::PhantomData;

    pub struct IsWeak<T> {
        _phantom: PhantomData<T>,
    }

    impl<T, LocationPolicyT, CheckingPolicyT> IsWeak<BasicPersistent<T, WeakPersistentPolicy, LocationPolicyT, CheckingPolicyT>> {
        pub const VALUE: bool = true;
    }
}

/// Persistent is a way to create a strong pointer from an off-heap object to
/// another on-heap object. As long as the Persistent handle is alive the GC will
/// keep the object pointed to alive. The Persistent handle is always a GC root
/// from the point of view of the GC. Persistent must be constructed and
/// destructed in the same thread.
pub type Persistent<T> = BasicPersistent<T, StrongPersistentPolicy, DefaultLocationPolicy, DefaultCheckingPolicy>;

/// WeakPersistent is a way to create a weak pointer from an off-heap object to
/// an on-heap object. The pointer is automatically cleared when the pointee gets
/// collected. WeakPersistent must be constructed and destructed in the same
/// thread.
pub type WeakPersistent<T> = BasicPersistent<T, WeakPersistentPolicy, DefaultLocationPolicy, DefaultCheckingPolicy>;
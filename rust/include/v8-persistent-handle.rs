// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of v8-persistent-handle.h.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod v8_internal;
mod v8_local_handle;
mod v8_weak_callback_info;
mod v8config;

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use v8_internal::*;
use v8_local_handle::*;
use v8_weak_callback_info::*;
use v8config::*;

pub mod api_internal {
    use super::*;

    extern "C" {
        pub fn Eternalize(isolate: *mut Isolate, handle: *mut Value) -> *mut Address;
        pub fn CopyGlobalReference(from: *mut Address) -> *mut Address;
        pub fn DisposeGlobal(global_handle: *mut Address);
        pub fn MakeWeak(location_addr: *mut *mut Address);
        pub fn ClearWeak(location: *mut Address) -> *mut std::ffi::c_void;
        pub fn AnnotateStrongRetainer(location: *mut Address, label: *const i8);
        pub fn GlobalizeReference(isolate: *mut Isolate, value: Address) -> *mut Address;
        pub fn MoveGlobalReference(from: *mut *mut Address, to: *mut *mut Address);
        pub fn MakeWeak(
            location: *mut Address,
            data: *mut std::ffi::c_void,
            weak_callback: WeakCallbackInfo<std::ffi::c_void>::Callback,
            type_: WeakCallbackType,
        );
    }
}

/// Eternal handles are set-once handles that live for the lifetime of the
/// isolate.
pub struct Eternal<T> {
    inner: api_internal::IndirectHandleBase,
    _phantom: PhantomData<T>,
}

impl<T> Eternal<T> {
    #[inline]
    pub fn new() -> Self {
        Eternal {
            inner: api_internal::IndirectHandleBase::new(std::ptr::null_mut()),
            _phantom: PhantomData,
        }
    }

    /// Constructor for handling automatic up casting.
    #[inline]
    pub fn from_local<S>(isolate: *mut Isolate, handle: Local<S>) -> Self
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        let mut eternal = Eternal::new();
        eternal.set(isolate, handle);
        eternal
    }

    // Can only be safely called if already set.
    #[inline]
    pub fn get(&self, isolate: *mut Isolate) -> Local<T> {
        // The eternal handle will never go away, so as with the roots, we don't
        // even need to open a handle.
        Local::<T>::from_slot(self.inner.slot())
    }

    #[inline]
    pub fn set<S>(&mut self, isolate: *mut Isolate, handle: Local<S>)
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        unsafe {
            self.inner.set_slot(api_internal::Eternalize(
                isolate,
                handle.unsafe_as::<Value>(),
            ));
        }
    }
}

/// An object reference that is independent of any handle scope.  Where
/// a Local handle only lives as long as the HandleScope in which it was
/// allocated, a PersistentBase handle remains valid until it is explicitly
/// disposed using Reset().
///
/// A persistent handle contains a reference to a storage cell within
/// the V8 engine which holds an object value and which is updated by
/// the garbage collector whenever the object is moved.  A new storage
/// cell can be created using the constructor or PersistentBase::Reset and
/// existing handles can be disposed using PersistentBase::Reset.
pub struct PersistentBase<T> {
    inner: api_internal::IndirectHandleBase,
    _phantom: PhantomData<T>,
}

impl<T> PersistentBase<T> {
    /// If non-empty, destroy the underlying storage cell
    /// IsEmpty() will return true after this call.
    #[inline]
    pub fn reset(&mut self) {
        if self.is_empty() {
            return;
        }
        unsafe {
            api_internal::DisposeGlobal(self.inner.slot());
        }
        self.inner.clear();
    }

    /// If non-empty, destroy the underlying storage cell
    /// and create a new one with the contents of other if other is non empty
    #[inline]
    pub fn reset_from_local<S>(&mut self, isolate: *mut Isolate, other: &Local<S>)
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        self.reset();
        if other.is_empty() {
            return;
        }
        unsafe {
            self.inner.set_slot(Self::new_internal(isolate, &**other));
        }
    }

    /// If non-empty, destroy the underlying storage cell
    /// and create a new one with the contents of other if other is non empty
    #[inline]
    pub fn reset_from_persistent<S>(&mut self, isolate: *mut Isolate, other: &PersistentBase<S>)
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        self.reset();
        if other.is_empty() {
            return;
        }
        unsafe {
            self.inner.set_slot(Self::new_internal(isolate, &**other));
        }
    }

    #[inline]
    pub fn get(&self, isolate: *mut Isolate) -> Local<T> {
        Local::<T>::new(isolate, self)
    }

    #[inline]
    pub fn equals<S>(&self, that: &PersistentBase<S>) -> bool
    where
        T: ValueType,
        S: ValueType,
    {
        unsafe { HandleHelper::equal_handles(self, that) }
    }

    #[inline]
    pub fn equals_local<S>(&self, that: &Local<S>) -> bool
    where
        T: ValueType,
        S: ValueType,
    {
        unsafe { HandleHelper::equal_handles(self, that) }
    }

    #[inline]
    pub fn not_equals<S>(&self, that: &PersistentBase<S>) -> bool
    where
        T: ValueType,
        S: ValueType,
    {
        !self.equals(that)
    }

    #[inline]
    pub fn not_equals_local<S>(&self, that: &Local<S>) -> bool
    where
        T: ValueType,
        S: ValueType,
    {
        !self.equals_local(that)
    }

    /// Install a finalization callback on this object.
    /// NOTE: There is no guarantee as to *when* or even *if* the callback is
    /// invoked. The invocation is performed solely on a best effort basis.
    /// As always, GC-based finalization should *not* be relied upon for any
    /// critical form of resource management!
    ///
    /// The callback is supposed to reset the handle. No further V8 API may be
    /// called in this callback. In case additional work involving V8 needs to be
    /// done, a second callback can be scheduled using
    /// WeakCallbackInfo<void>::SetSecondPassCallback.
    #[inline]
    pub fn set_weak<P>(
        &mut self,
        parameter: *mut P,
        callback: WeakCallbackInfo<P>::Callback,
        type_: WeakCallbackType,
    ) {
        unsafe {
            api_internal::MakeWeak(
                self.inner.slot(),
                parameter as *mut std::ffi::c_void,
                std::mem::transmute(callback),
                type_,
            );
        }
    }

    /// Turns this handle into a weak phantom handle without finalization callback.
    /// The handle will be reset automatically when the garbage collector detects
    /// that the object is no longer reachable.
    #[inline]
    pub fn set_weak_phantom(&mut self) {
        unsafe {
            api_internal::MakeWeak(self.inner.slot() as *mut *mut Address);
        }
    }

    #[inline]
    pub fn clear_weak<P>(&mut self) -> *mut P {
        unsafe { api_internal::ClearWeak(self.inner.slot()) as *mut P }
    }

    // TODO(dcarney): remove this.
    #[inline]
    pub fn clear_weak_void(&mut self) {
        self.clear_weak::<std::ffi::c_void>();
    }

    /// Annotates the strong handle with the given label, which is then used by the
    /// heap snapshot generator as a name of the edge from the root to the handle.
    /// The function does not take ownership of the label and assumes that the
    /// label is valid as long as the handle is valid.
    #[inline]
    pub fn annotate_strong_retainer(&mut self, label: *const i8) {
        unsafe {
            api_internal::AnnotateStrongRetainer(self.inner.slot(), label);
        }
    }

    /// Returns true if the handle's reference is weak.
    #[inline]
    pub fn is_weak(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        unsafe {
            Internals::get_node_state(self.inner.slot()) == Internals::kNodeStateIsWeakValue
        }
    }

    /// Assigns a wrapper class ID to the handle.
    #[inline]
    pub fn set_wrapper_class_id(&mut self, class_id: u16) {
        if self.is_empty() {
            return;
        }
        unsafe {
            let addr = (self.inner.slot() as *mut u8).add(Internals::kNodeClassIdOffset);
            *(addr as *mut u16) = class_id;
        }
    }

    /// Returns the class ID previously assigned to this handle or 0 if no class ID
    /// was previously assigned.
    #[inline]
    pub fn wrapper_class_id(&self) -> u16 {
        if self.is_empty() {
            return 0;
        }
        unsafe {
            let addr = (self.inner.slot() as *mut u8).add(Internals::kNodeClassIdOffset);
            *(addr as *mut u16)
        }
    }

    #[inline]
    fn new_internal(isolate: *mut Isolate, that: &T) -> *mut Address
    where
        T: ValueType,
    {
        if ValueHelper::is_empty(that) {
            return std::ptr::null_mut();
        }
        unsafe {
            api_internal::GlobalizeReference(
                isolate,
                ValueHelper::value_as_address(that),
            )
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.slot().is_null()
    }

    #[inline]
    pub fn empty() -> Self {
        PersistentBase {
            inner: api_internal::IndirectHandleBase::new(std::ptr::null_mut()),
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for PersistentBase<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.slot().cast() }
    }
}

impl<T> DerefMut for PersistentBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.slot().cast() }
    }
}

impl<T> PersistentBase<T>
where
    T: ValueType,
{
    #[inline]
    pub fn new() -> Self {
        PersistentBase {
            inner: api_internal::IndirectHandleBase::new(std::ptr::null_mut()),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn with_address(location: *mut Address) -> Self {
        PersistentBase {
            inner: api_internal::IndirectHandleBase::new(location),
            _phantom: PhantomData,
        }
    }
}

/// Default traits for Persistent. This class does not allow
/// use of the copy constructor or assignment operator.
/// At present kResetInDestructor is not set, but that will change in a future
/// version.
pub struct NonCopyablePersistentTraits<T> {
    _phantom: PhantomData<T>,
}

impl<T> NonCopyablePersistentTraits<T> {
    pub const k_reset_in_destructor: bool = false;
}

/// A PersistentBase which allows copy and assignment.
///
/// Copy, assignment and destructor behavior is controlled by the traits
/// class M.
///
/// CAVEAT: Persistent objects do not have proper destruction behavior by default
/// and as such will leak the object without explicit clear. Consider using
/// `v8::Global` instead which has proper destruction and move semantics.
pub struct Persistent<T, M = NonCopyablePersistentTraits<T>> {
    inner: PersistentBase<T>,
    _phantom: PhantomData<M>,
}

impl<T, M> Persistent<T, M> {
    /// A Persistent with no storage cell.
    #[inline]
    pub fn new() -> Self {
        Persistent {
            inner: PersistentBase::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T, M> Persistent<T, M>
where
    T: ValueType,
{
    /// Construct a Persistent from a Local with automatic up casting.
    /// When the Local is non-empty, a new storage cell is created
    /// pointing to the same object, and no flags are set.
    #[inline]
    pub fn from_local<S>(isolate: *mut Isolate, that: Local<S>) -> Self
    where
        S: ValueType + Deref<Target = T>,
    {
        Persistent {
            inner: PersistentBase::with_address(PersistentBase::<T>::new_internal(
                isolate,
                &**that,
            )),
            _phantom: PhantomData,
        }
    }

    /// Construct a Persistent from a Persistent with automatic up casting.
    /// When the Persistent is non-empty, a new storage cell is created
    /// pointing to the same object, and no flags are set.
    #[inline]
    pub fn from_persistent<S, M2>(isolate: *mut Isolate, that: &Persistent<S, M2>) -> Self
    where
        S: ValueType + Deref<Target = T>,
    {
        Persistent {
            inner: PersistentBase::with_address(PersistentBase::<T>::new_internal(
                isolate,
                &**that,
            )),
            _phantom: PhantomData,
        }
    }
}

impl<T, M> Drop for Persistent<T, M> {
    /// The destructor will dispose the Persistent based on the
    /// kResetInDestructor flags in the traits class.  Since not calling dispose
    /// can result in a memory leak, it is recommended to always set this flag.
    #[inline]
    fn drop(&mut self) {
        // FIXME
        // if M::kResetInDestructor {
        //     self.inner.reset();
        // }
    }
}

impl<T: ValueType, M> Persistent<T, M> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: ValueType, M> Deref for Persistent<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: ValueType, M> DerefMut for Persistent<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T, M> Persistent<T, M>
where
    T: ValueType,
{
    #[inline]
    fn copy<S, M2>(&mut self, that: &Persistent<S, M2>)
    where
        S: ValueType + Deref<Target = T>,
    {
        self.inner.reset();
        if that.is_empty() {
            return;
        }
        unsafe {
            self.inner
                .inner
                .set_slot(api_internal::CopyGlobalReference(that.inner.inner.slot()));
        }
        //M::Copy(that, self); //FIXME: this should be trait dependent
    }

    // TODO(dcarney): this is pretty useless, fix or remove
    #[inline]
    pub fn cast<S, M2>(that: &Persistent<S, M2>) -> &mut Persistent<T, M>
    where
        S: ValueType + Deref<Target = T>,
    {
        // If we're going to perform the type check then we have to check
        // that the handle isn't empty before doing the checked cast.
        // TODO: add type check
        //if !that.is_empty() {
        //    T::Cast(that.template value<S>());
        //}
        unsafe {
            &mut *(that as *const Persistent<S, M2> as *mut Persistent<S, M2> as *mut Persistent<T, M>)
        }
    }

    // TODO(dcarney): this is pretty useless, fix or remove
    #[inline]
    pub fn as_ref<S, M2>(&self) -> &Persistent<S, M2>
    where
        S: ValueType + Deref<Target = T>,
    {
        Persistent::<S, M2>::cast(self)
    }
}

impl<T: ValueType, M> Clone for Persistent<T, M> {
    fn clone(&self) -> Self {
        let mut new = Persistent::<T, M>::new();
        new.copy(self);
        new
    }
}

/// A PersistentBase which has move semantics.
///
/// Note: Persistent class hierarchy is subject to future changes.
pub struct Global<T> {
    inner: PersistentBase<T>,
}

impl<T> Global<T> {
    /// A Global with no storage cell.
    #[inline]
    pub fn new() -> Self {
        Global {
            inner: PersistentBase::new(),
        }
    }

    /// Construct a Global from a Local with automatic up casting.
    /// When the Local is non-empty, a new storage cell is created
    /// pointing to the same object, and no flags are set.
    #[inline]
    pub fn from_local<S>(isolate: *mut Isolate, that: Local<S>) -> Self
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        Global {
            inner: PersistentBase::with_address(PersistentBase::<T>::new_internal(
                isolate,
                &**that,
            )),
        }
    }

    /// Construct a Global from a PersistentBase with automatic up casting.
    /// When the Persistent is non-empty, a new storage cell is created
    /// pointing to the same object, and no flags are set.
    #[inline]
    pub fn from_persistent_base<S>(isolate: *mut Isolate, that: &PersistentBase<S>) -> Self
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        Global {
            inner: PersistentBase::with_address(PersistentBase::<T>::new_internal(
                isolate,
                &**that,
            )),
        }
    }
}

impl<T> Drop for Global<T> {
    #[inline]
    fn drop(&mut self) {
        self.inner.reset();
    }
}

impl<T> Deref for Global<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> DerefMut for Global<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T> Global<T> {
    #[inline]
    pub fn pass(self) -> Self {
        self
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// Implement move constructor
impl<T> Global<T> {
    #[inline]
    pub fn move_from(other: &mut Global<T>) -> Self {
        let mut result = Global::new();
        unsafe {
            if !other.is_empty() {
                result.inner.inner.set_slot(other.inner.inner.slot());
                api_internal::MoveGlobalReference(
                    &mut other.inner.inner.slot(),
                    &mut result.inner.inner.slot(),
                );
                other.inner.inner.clear();
            }
        }
        result
    }
}

// Implement move assignment
impl<T> Global<T> {
    #[inline]
    pub fn move_assign<S>(&mut self, rhs: &mut Global<S>)
    where
        T: ValueType,
        S: ValueType + Deref<Target = T>,
    {
        if self as *mut _ != rhs as *mut _ {
            self.inner.reset();
            unsafe {
                if !rhs.is_empty() {
                    self.inner.inner.set_slot(rhs.inner.inner.slot());
                    api_internal::MoveGlobalReference(
                        &mut rhs.inner.inner.slot(),
                        &mut self.inner.inner.slot(),
                    );
                    rhs.inner.inner.clear();
                }
            }
        }
    }
}

// UniquePersistent is an alias for Global for historical reason.
pub type UniquePersistent<T> = Global<T>;

/// Interface for iterating through all the persistent handles in the heap.
pub trait PersistentHandleVisitor {
    fn visit_persistent_handle(&mut self, value: &mut Persistent<Value>, class_id: u16);
}

// Implementation of the visitor as a struct.
pub struct GenericPersistentHandleVisitor<F>
where
    F: FnMut(&mut Persistent<Value>, u16),
{
    callback: F,
}

impl<F> GenericPersistentHandleVisitor<F>
where
    F: FnMut(&mut Persistent<Value>, u16),
{
    pub fn new(callback: F) -> Self {
        GenericPersistentHandleVisitor { callback }
    }
}

impl<F> PersistentHandleVisitor for GenericPersistentHandleVisitor<F>
where
    F: FnMut(&mut Persistent<Value>, u16),
{
    fn visit_persistent_handle(&mut self, value: &mut Persistent<Value>, class_id: u16) {
        (self.callback)(value, class_id);
    }
}
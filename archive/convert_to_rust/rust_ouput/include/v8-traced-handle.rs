// Converted from V8 C++ source files:
// Header: v8-traced-handle.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
pub mod internal {
    pub struct Address {}
    pub struct Isolate {}
    pub enum TracedReferenceStoreMode {
        kInitializingStore,
        kAssigningStore,
    }
    pub enum TracedReferenceHandling {
        kDefault,
        kDroppable,
    }
    extern "C" {
        pub fn GlobalizeTracedReference(
            isolate: *mut Isolate,
            value: Address,
            slot: *mut Address,
            store_mode: TracedReferenceStoreMode,
            reference_handling: TracedReferenceHandling,
        ) -> *mut Address;
        pub fn MoveTracedReference(from: *mut *mut Address, to: *mut *mut Address);
        pub fn CopyTracedReference(from: *const *const Address, to: *mut *mut Address);
        pub fn DisposeTracedReference(global_handle: *mut Address);
    }
    pub mod ValueHelper {
        use super::Address;
        pub fn ValueAsAddress<T>(_value: *const T) -> Address {
            Address {}
        }
    }
    pub mod HandleHelper {
        use super::Address;
        use crate::TracedReferenceBase;
        use crate::Local;
        pub fn EqualHandles(_lhs: &TracedReferenceBase, _rhs: &TracedReferenceBase) -> bool {
            true
        }
        pub fn EqualHandles_local<U>(_lhs: &TracedReferenceBase, _rhs: &Local<U>) -> bool {
            true
        }
    }
}

use std::sync::atomic::{AtomicPtr, Ordering};

pub struct TracedReferenceBase {
    slot_: AtomicPtr<internal::Address>,
}

impl TracedReferenceBase {
    pub fn reset(&self) {
        if self.is_empty() {
            return;
        }
        unsafe {
            internal::DisposeTracedReference(self.slot_.load(Ordering::Relaxed));
        }
        self.set_slot_thread_safe(std::ptr::null_mut());
    }

    pub fn get<Data>(&self, _isolate: *mut internal::Isolate) -> Local<Data> {
        if self.is_empty() {
            return Local {
                ptr: std::ptr::null_mut(),
            };
        }
        Local {
            ptr: self.value::<Data>(),
        }
    }

    pub fn is_empty_thread_safe(&self) -> bool {
        self.get_slot_thread_safe().is_null()
    }

    pub fn check_value(&self) {}

    fn set_slot_thread_safe(&self, new_val: *mut internal::Address) {
        self.slot_.store(new_val, Ordering::Relaxed);
    }

    fn get_slot_thread_safe(&self) -> *mut internal::Address {
        self.slot_.load(Ordering::Relaxed)
    }
    fn slot(&self) -> *mut *mut internal::Address {
        &self.slot_.load(Ordering::Relaxed) as *const *mut internal::Address as *mut *mut internal::Address
    }
    fn is_empty(&self) -> bool {
        self.slot_.load(Ordering::Relaxed).is_null()
    }
    fn value<Data>(&self) -> *mut Data {
        self.slot_.load(Ordering::Relaxed) as *mut Data
    }
}

impl Default for TracedReferenceBase {
    fn default() -> Self {
        TracedReferenceBase {
            slot_: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}

pub struct BasicTracedReference<T> {
    base: TracedReferenceBase,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BasicTracedReference<T> {
    pub fn get(&self, isolate: *mut internal::Isolate) -> Local<T> {
        Local {
            ptr: self.base.value::<T>(),
        }
    }
    pub fn as_ref<S>(&self) -> &BasicTracedReference<S> {
        unsafe { &*(self as *const Self as *const BasicTracedReference<S>) }
    }
}

impl<T> Default for BasicTracedReference<T> {
    fn default() -> Self {
        BasicTracedReference {
            base: TracedReferenceBase::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T> BasicTracedReference<T> {
    fn new_from_non_empty_value(
        isolate: *mut internal::Isolate,
        that: *mut T,
        slot: *mut *mut internal::Address,
        store_mode: internal::TracedReferenceStoreMode,
        reference_handling: internal::TracedReferenceHandling,
    ) -> *mut internal::Address {
        unsafe {
            internal::GlobalizeTracedReference(
                isolate,
                internal::ValueHelper::ValueAsAddress(that),
                slot,
                store_mode,
                reference_handling,
            )
        }
    }
}

pub struct TracedReference<T> {
    base: BasicTracedReference<T>,
}

impl<T> Default for TracedReference<T> {
    fn default() -> Self {
        TracedReference {
            base: BasicTracedReference::default(),
        }
    }
}

impl<T> TracedReference<T> {
    pub struct IsDroppable {}
    pub fn reset(&self) {
        self.base.base.reset();
    }
    pub fn new<S>(isolate: *mut internal::Isolate, that: Local<S>) -> Self
    where
        T: std::marker::Sized,
        S: std::marker::Sized,
        T: ?Sized,
        S: ?Sized,
        T: 'static,
        S: 'static,
        S: std::borrow::Borrow<T>,
    {
        if that.is_empty() {
            return TracedReference {
                base: BasicTracedReference::default(),
            };
        }

        let mut traced_ref = TracedReference {
            base: BasicTracedReference::default(),
        };
        let slot = traced_ref.slot_mut();
        unsafe {
            *slot = BasicTracedReference::<T>::new_from_non_empty_value(
                isolate,
                *that,
                slot,
                internal::TracedReferenceStoreMode::kInitializingStore,
                internal::TracedReferenceHandling::kDefault,
            );
        }
        traced_ref
    }

    pub fn new_droppable<S>(
        isolate: *mut internal::Isolate,
        that: Local<S>,
        _is_droppable: IsDroppable,
    ) -> Self
    where
        T: std::marker::Sized,
        S: std::marker::Sized,
        T: ?Sized,
        S: ?Sized,
        T: 'static,
        S: 'static,
        S: std::borrow::Borrow<T>,
    {
        if that.is_empty() {
            return TracedReference {
                base: BasicTracedReference::default(),
            };
        }
        let mut traced_ref = TracedReference {
            base: BasicTracedReference::default(),
        };
        let slot = traced_ref.slot_mut();
        unsafe {
            *slot = BasicTracedReference::<T>::new_from_non_empty_value(
                isolate,
                *that,
                slot,
                internal::TracedReferenceStoreMode::kInitializingStore,
                internal::TracedReferenceHandling::kDroppable,
            );
        }
        traced_ref
    }
    pub fn slot_mut(&mut self) -> *mut *mut internal::Address {
        self.base.base.slot()
    }
}

impl<T> std::ops::Drop for TracedReference<T> {
    fn drop(&mut self) {
        self.reset();
    }
}

impl<T> TracedReference<T> {
    pub fn as_ref<S>(&self) -> &TracedReference<S> {
        unsafe { &*(self as *const Self as *const TracedReference<S>) }
    }
}

impl<T> TracedReference<T> {
    pub fn reset_local<S>(&mut self, isolate: *mut internal::Isolate, other: &Local<S>)
    where
        T: std::marker::Sized,
        S: std::marker::Sized,
        T: ?Sized,
        S: ?Sized,
        T: 'static,
        S: 'static,
        S: std::borrow::Borrow<T>,
    {
        self.reset();
        if other.is_empty() {
            return;
        }
        let slot = self.slot_mut();
        unsafe {
            *slot = BasicTracedReference::<T>::new_from_non_empty_value(
                isolate,
                *(*other),
                slot,
                internal::TracedReferenceStoreMode::kAssigningStore,
                internal::TracedReferenceHandling::kDefault,
            );
        }
    }

    pub fn reset_local_droppable<S>(
        &mut self,
        isolate: *mut internal::Isolate,
        other: &Local<S>,
        _is_droppable: TracedReference<T>::IsDroppable,
    ) where
        T: std::marker::Sized,
        S: std::marker::Sized,
        T: ?Sized,
        S: ?Sized,
        T: 'static,
        S: 'static,
        S: std::borrow::Borrow<T>,
    {
        self.reset();
        if other.is_empty() {
            return;
        }

        let slot = self.slot_mut();
        unsafe {
            *slot = BasicTracedReference::<T>::new_from_non_empty_value(
                isolate,
                *(*other),
                slot,
                internal::TracedReferenceStoreMode::kAssigningStore,
                internal::TracedReferenceHandling::kDroppable,
            );
        }
    }
}

impl<T> std::ops::Drop for BasicTracedReference<T> {
    fn drop(&mut self) {
        self.base.reset();
    }
}

impl<T> Clone for TracedReference<T> {
    fn clone(&self) -> Self {
        let mut new_traced_reference: TracedReference<T> = TracedReference {
            base: BasicTracedReference::default(),
        };
        if !self.base.base.is_empty() {
            unsafe {
                internal::CopyTracedReference(
                    &self.base.base.slot() as *const *mut internal::Address
                        as *const *const internal::Address,
                    new_traced_reference.slot_mut(),
                );
            }
        }
        new_traced_reference
    }
}

impl<T> Copy for TracedReference<T> where T: Copy {}
impl<T> Copy for BasicTracedReference<T> where T: Copy {}
impl<T> Clone for BasicTracedReference<T> {
    fn clone(&self) -> Self {
        let mut new_basic_traced_reference: BasicTracedReference<T> = BasicTracedReference::default();
        if !self.base.is_empty() {
        }
        new_basic_traced_reference
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Local<T> {
    ptr: *mut T,
}

impl<T> Local<T> {
    pub fn new(_isolate: *mut internal::Isolate, that: &TracedReferenceBase) -> Self {
        if that.get_slot_thread_safe().is_null() {
            return Local { ptr: std::ptr::null_mut() };
        }
        Local {
            ptr: that.get_slot_thread_safe() as *mut T,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T> std::ops::Deref for Local<T> {
    type Target = *mut T;
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> std::ops::DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}

impl<T> Local<T> {
  pub fn new_internal(_isolate: *mut internal::Isolate, ptr: *mut T) -> Self {
      Local { ptr }
  }
}

impl<T> PartialEq<TracedReferenceBase> for Local<T> {
    fn eq(&self, other: &TracedReferenceBase) -> bool {
        internal::HandleHelper::EqualHandles_local(other, self)
    }
}

impl<T> PartialEq<Local<T>> for Local<T> {
    fn eq(&self, other: &Local<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for Local<T> {}
impl TracedReferenceBase {
    pub fn slot(&self) -> *mut *mut internal::Address {
        unsafe {std::mem::transmute::<& AtomicPtr<internal::Address>, *mut *mut internal::Address>(&self.slot_)}
    }
}

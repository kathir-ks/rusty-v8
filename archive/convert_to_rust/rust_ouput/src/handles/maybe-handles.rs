// Converted from V8 C++ source files:
// Header: maybe-handles.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::parsing::pending_compilation_error_handler::v8;

pub struct NullMaybeHandleType {}

pub const kNullMaybeHandle: NullMaybeHandleType = NullMaybeHandleType {};

pub trait IsSubtype<T> {
    fn is_subtype(&self) -> bool;
}

impl<T> IsSubtype<T> for T {
    fn is_subtype(&self) -> bool {
        true
    }
}

// Mock implementations for types used in requires clauses
struct JSArray {}
struct Object {}
struct Smi {}

trait LocalHeapTrait {
    fn allocate<T>(&self, object: T) -> *mut T;
}
struct LocalHeap {}
impl LocalHeapTrait for LocalHeap{
    fn allocate<T>(&self, object: T) -> *mut T {
        Box::into_raw(Box::new(object))
    }
}

trait IsolateTrait {
    fn allocate<T>(&self, object: T) -> *mut T;
}
struct Isolate {}
impl IsolateTrait for Isolate{
    fn allocate<T>(&self, object: T) -> *mut T {
        Box::into_raw(Box::new(object))
    }
}

// ----------------------------------------------------------------------------
// A Handle can be converted into a MaybeHandle. Converting a MaybeHandle
// into a Handle requires checking that it does not point to nullptr. This
// ensures nullptr checks before use.
//
// Also note that MaybeHandles do not provide default equality comparison or
// hashing operators on purpose. Such operators would be misleading, because
// intended semantics is ambiguous between handle location and object identity.
#[derive(Copy, Clone)]
pub struct MaybeHandle<T> {
    location_: *mut Address,
    _phantom: PhantomData<T>,
}

impl<T> MaybeHandle<T> {
    #[inline]
    pub fn new() -> Self {
        MaybeHandle {
            location_: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_null(_: NullMaybeHandleType) -> Self {
        MaybeHandle {
            location_: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting from Handle.
    // Ex. Handle<JSArray> can be passed when MaybeHandle<Object> is expected.
    #[inline]
    pub fn from_handle<S: IsSubtype<T>>(handle: Handle<S>) -> Self {
        MaybeHandle {
            location_: handle.location_,
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting.
    // Ex. MaybeHandle<JSArray> can be passed when MaybeHandle<Object> is
    // expected.
    #[inline]
    pub fn from_maybe_handle<S: IsSubtype<T>>(maybe_handle: MaybeHandle<S>) -> Self {
        MaybeHandle {
            location_: maybe_handle.location_,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_tagged(object: Tagged<T>, isolate: &mut Isolate) -> Self {
        let ptr = isolate.allocate(object);
        MaybeHandle {
            location_: ptr as *mut Address,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_tagged_local_heap(object: Tagged<T>, local_heap: &mut LocalHeap) -> Self {
        let ptr = local_heap.allocate(object);
        MaybeHandle {
            location_: ptr as *mut Address,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn assert(&self) {
        assert!(!self.location_.is_null());
    }

    #[inline]
    pub fn check(&self) {
        if self.location_.is_null() {
            panic!("Check failed: location_ is null");
        }
    }

    #[inline]
    pub fn to_handle_checked(&self) -> Handle<T> {
        self.check();
        Handle { location_: self.location_ }
    }

    // Convert to a Handle with a type that can be upcasted to.
    #[inline]
    pub fn to_handle<S>(&self, out: &mut Handle<S>) -> bool {
        if self.location_.is_null() {
            *out = Handle::<T>::null().into();
            false
        } else {
            *out = Handle::<T> { location_: self.location_ }.into();
            true
        }
    }

    #[inline]
    pub fn to_direct_handle<S>(&self, out: &mut DirectHandle<S>) -> bool {
        if self.location_.is_null() {
            *out = DirectHandle::<T>::null().into();
            false
        } else {
            *out = DirectHandle::<T> { handle_: Handle::<T> {location_: self.location_} }.into();
            true
        }
    }

    // Location equality.
    pub fn equals(&self, other: MaybeHandle<T>) -> bool {
        self.address() == other.address()
    }

    // Returns the raw address where this handle is stored. This should only be
    // used for hashing handles; do not ever try to dereference it.
    #[inline]
    pub fn address(&self) -> Address {
        self.location_ as Address
    }

    pub fn is_null(&self) -> bool {
        self.location_.is_null()
    }

    #[inline]
    pub fn from_location(location: *mut Address) -> Self {
        MaybeHandle {
            location_: location,
            _phantom: PhantomData,
        }
    }

    // Allow access to location_ by MaybeHandles of different classes.
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub(crate) fn get_location(&self) -> &mut *mut Address {
        &mut self.location_
    }
}

impl<T> Default for MaybeHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for MaybeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaybeHandle")
            .field("location_", &self.location_)
            .finish()
    }
}

impl<T> Deref for MaybeHandle<T> {
    type Target = *mut Address;

    fn deref(&self) -> &Self::Target {
        &self.location_
    }
}

impl<T> DerefMut for MaybeHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.location_
    }
}

impl<T> From<MaybeHandle<T>> for *mut Address {
    fn from(handle: MaybeHandle<T>) -> Self {
        handle.location_
    }
}

impl<T> From<&MaybeHandle<T>> for *mut Address {
    fn from(handle: &MaybeHandle<T>) -> Self {
        handle.location_
    }
}

impl<T> From<*mut Address> for MaybeHandle<T> {
    fn from(location: *mut Address) -> Self {
        MaybeHandle { location_: location, _phantom: PhantomData }
    }
}

impl<T> From<Handle<T>> for MaybeHandle<T> {
    fn from(handle: Handle<T>) -> Self {
        MaybeHandle { location_: handle.location_, _phantom: PhantomData }
    }
}

impl<T> From<&Handle<T>> for MaybeHandle<T> {
    fn from(handle: &Handle<T>) -> Self {
        MaybeHandle { location_: handle.location_, _phantom: PhantomData }
    }
}

impl<T> From<&mut Handle<T>> for MaybeHandle<T> {
    fn from(handle: &mut Handle<T>) -> Self {
        MaybeHandle { location_: handle.location_, _phantom: PhantomData }
    }
}

pub fn UncheckedCast<To, From>(value: MaybeHandle<From>) -> MaybeHandle<To> {
    MaybeHandle {
        location_: value.location_,
        _phantom: PhantomData,
    }
}

impl<T> std::fmt::Display for MaybeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaybeHandle<{}> {{ location_: {:?} }}", std::any::type_name::<T>(), self.location_)
    }
}

#[allow(unused_variables)]
impl<T> std::fmt::Display for MaybeIndirectHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaybeIndirectHandle<{}> {{ location_: {:?} }}", std::any::type_name::<T>(), self.location_)
    }
}

#[allow(unused_variables)]
impl<T> std::fmt::Display for MaybeDirectHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaybeDirectHandle<{}> {{ location_: {:?} }}", std::any::type_name::<T>(), self.location_)
    }
}

pub struct IndirectHandle<T> {
    location_: *mut Address,
    _phantom: PhantomData<T>,
}

impl<T> IndirectHandle<T> {
    pub fn is_null(&self) -> bool {
        self.location_.is_null()
    }
}

impl<T> From<MaybeHandle<T>> for IndirectHandle<T> {
    fn from(maybe_handle: MaybeHandle<T>) -> Self {
        IndirectHandle { location_: maybe_handle.location_, _phantom: PhantomData }
    }
}

impl<T> From<&MaybeHandle<T>> for IndirectHandle<T> {
    fn from(maybe_handle: &MaybeHandle<T>) -> Self {
        IndirectHandle { location_: maybe_handle.location_, _phantom: PhantomData }
    }
}

impl<T> From<&mut MaybeHandle<T>> for IndirectHandle<T> {
    fn from(maybe_handle: &mut MaybeHandle<T>) -> Self {
        IndirectHandle { location_: maybe_handle.location_, _phantom: PhantomData }
    }
}

pub struct MaybeIndirectHandle<T> {
    location_: *mut *mut Address,
    _phantom: PhantomData<T>,
}

impl<T> MaybeIndirectHandle<T> {
    pub fn is_null(&self) -> bool {
        unsafe { self.location_.is_null() || (*self.location_).is_null() }
    }
}

impl<T> From<MaybeHandle<T>> for MaybeIndirectHandle<T> {
    fn from(maybe_handle: MaybeHandle<T>) -> Self {
        let boxed_ptr = Box::new(maybe_handle.location_);
        MaybeIndirectHandle { location_: Box::into_raw(boxed_ptr), _phantom: PhantomData }
    }
}

impl<T> From<&MaybeHandle<T>> for MaybeIndirectHandle<T> {
    fn from(maybe_handle: &MaybeHandle<T>) -> Self {
        let boxed_ptr = Box::new(maybe_handle.location_);
        MaybeIndirectHandle { location_: Box::into_raw(boxed_ptr), _phantom: PhantomData }
    }
}

impl<T> From<&mut MaybeHandle<T>> for MaybeIndirectHandle<T> {
    fn from(maybe_handle: &mut MaybeHandle<T>) -> Self {
        let boxed_ptr = Box::new(maybe_handle.location_);
        MaybeIndirectHandle { location_: Box::into_raw(boxed_ptr), _phantom: PhantomData }
    }
}

impl<T> Drop for MaybeIndirectHandle<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.location_.is_null() {
                drop(Box::from_raw(self.location_));
            }
        }
    }
}

// A handle which contains a potentially weak pointer. Keeps it alive (strongly)
// while the MaybeObjectHandle is alive.
#[derive(Copy, Clone)]
pub struct MaybeObjectHandle {
    reference_type_: HeapObjectReferenceType,
    handle_: MaybeHandle<Object>,
}

impl MaybeObjectHandle {
    #[inline]
    pub fn new() -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::new(),
        }
    }

    #[inline]
    pub fn from_tagged_maybe_object(object: Tagged<MaybeObject>, isolate: &mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged(
                unsafe { std::mem::transmute(object) },
                isolate,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_object(object: Tagged<Object>, isolate: &mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged(object, isolate),
        }
    }

    #[inline]
    pub fn from_tagged_smi(object: Tagged<Smi>, isolate: &mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged(
                unsafe { std::mem::transmute(object) },
                isolate,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_maybe_object_local_heap(object: Tagged<MaybeObject>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged_local_heap(
                unsafe { std::mem::transmute(object) },
                local_heap,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_object_local_heap(object: Tagged<Object>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged_local_heap(object, local_heap),
        }
    }

    #[inline]
    pub fn from_tagged_smi_local_heap(object: Tagged<Smi>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_tagged_local_heap(
                unsafe { std::mem::transmute(object) },
                local_heap,
            ),
        }
    }

    #[inline]
    pub fn from_handle(object: Handle<Object>) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeHandle::from_handle(object),
        }
    }

    pub fn weak_tagged(object: Tagged<Object>, isolate: &mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::WEAK,
            handle_: MaybeHandle::from_tagged(object, isolate),
        }
    }

    pub fn weak_handle(object: Handle<Object>) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::WEAK,
            handle_: MaybeHandle::from_handle(object),
        }
    }

    #[inline]
    pub fn operator_star(&self) -> Tagged<MaybeObject> {
        unsafe { std::mem::transmute_copy(&self.handle_.location_) }
    }

    #[inline]
    pub fn operator_arrow(&self) -> Tagged<MaybeObject> {
        unsafe { std::mem::transmute_copy(&self.handle_.location_) }
    }

    #[inline]
    pub fn object(&self) -> IndirectHandle<Object> {
        IndirectHandle { location_: self.handle_.location_, _phantom: PhantomData }
    }

    #[inline]
    pub fn is_identical_to(&self, other: &MaybeObjectHandle) -> bool {
        self.handle_.location_ == other.handle_.location_
    }

    pub fn is_null(&self) -> bool {
        self.handle_.is_null()
    }

    pub fn reference_type(&self) -> HeapObjectReferenceType {
        self.reference_type_
    }

    #[inline]
    pub fn from_tagged_object_reference_type(
        object: Tagged<Object>,
        reference_type: HeapObjectReferenceType,
        isolate: &mut Isolate,
    ) -> Self {
        MaybeObjectHandle {
            reference_type_: reference_type,
            handle_: MaybeHandle::from_tagged(object, isolate),
        }
    }

    #[inline]
    pub fn from_handle_reference_type(
        object: Handle<Object>,
        reference_type: HeapObjectReferenceType,
    ) -> Self {
        MaybeObjectHandle {
            reference_type_: reference_type,
            handle_: MaybeHandle::from_handle(object),
        }
    }
}

impl Default for MaybeObjectHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
pub struct MaybeDirectHandle<T> {
    location_: Address,
    _phantom: PhantomData<T>,
}

impl<T> MaybeDirectHandle<T> {
    #[inline]
    pub fn new() -> Self {
        MaybeDirectHandle {
            location_: kTaggedNullAddress,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_null(_: NullMaybeHandleType) -> Self {
        MaybeDirectHandle {
            location_: kTaggedNullAddress,
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting from DirectHandle.
    // Ex. DirectHandle<JSArray> can be passed when MaybeDirectHandle<Object> is
    // expected.
    #[inline]
    pub fn from_direct_handle<S: IsSubtype<T>>(handle: DirectHandle<S>) -> Self {
        MaybeDirectHandle {
            location_: handle.address(),
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting from Handle.
    // Ex. Handle<JSArray> can be passed when MaybeDirectHandle<Object> is
    // expected.
    #[inline]
    pub fn from_handle<S: IsSubtype<T>>(handle: Handle<S>) -> Self {
        MaybeDirectHandle {
            location_: DirectHandle::<S>::new_with_handle(handle).address(),
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting.
    // Ex. MaybeDirectHandle<JSArray> can be passed when MaybeDirectHandle<Object>
    // is expected.
    #[inline]
    pub fn from_maybe_direct_handle<S: IsSubtype<T>>(maybe_handle: MaybeDirectHandle<S>) -> Self {
        MaybeDirectHandle {
            location_: maybe_handle.location_,
            _phantom: PhantomData,
        }
    }

    // Constructor for handling automatic up casting from MaybeHandle.
    // Ex. MaybeHandle<JSArray> can be passed when
    // MaybeDirectHandle<Object> is expected.
    #[inline]
    pub fn from_maybe_indirect_handle<S: IsSubtype<T>>(maybe_handle: MaybeIndirectHandle<S>) -> Self {
        let location = unsafe {
            if maybe_handle.location_.is_null() {
                kTaggedNullAddress
            } else {
                *maybe_handle.location_
            }
        };
        MaybeDirectHandle {
            location_: location,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_tagged(object: Tagged<T>, isolate: &mut Isolate) -> Self {
        let ptr = isolate.allocate(object);
        MaybeDirectHandle {
            location_: ptr as Address,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_tagged_local_heap(object: Tagged<T>, local_heap: &mut LocalHeap) -> Self {
        let ptr = local_heap.allocate(object);
        MaybeDirectHandle {
            location_: ptr as Address,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn assert(&self) {
        assert_ne!(self.location_, kTaggedNullAddress);
    }

    #[inline]
    pub fn check(&self) {
        assert_ne!(self.location_, kTaggedNullAddress);
    }

    #[inline]
    pub fn to_handle_checked(&self) -> DirectHandle<T> {
        self.check();
        DirectHandle { handle_: Handle { location_: self.location_ as *mut Address } }
    }

    // Convert to a DirectHandle with a type that can be upcasted to.
    #[inline]
    pub fn to_handle<S>(&self, out: &mut DirectHandle<S>) -> bool {
        if self.location_ == kTaggedNullAddress {
            *out = DirectHandle::<T>::null();
            false
        } else {
            *out = DirectHandle { handle_: Handle { location_: self.location_ as *mut Address } };
            true
        }
    }

    // Address equality.
    pub fn equals(&self, other: MaybeHandle<T>) -> bool {
        self.address() == other.address()
    }

    // Returns the raw address where this direct handle is stored.
    #[inline]
    pub fn address(&self) -> Address {
        self.location_
    }

    pub fn is_null(&self) -> bool {
        self.location_ == kTaggedNullAddress
    }

    #[inline]
    pub fn from_location(location: Address) -> Self {
        MaybeDirectHandle {
            location_: location,
            _phantom: PhantomData,
        }
    }

    // Allow access to location_ by MaybeDirectHandles of different classes.
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub(crate) fn get_location(&self) -> &mut Address {
        &mut self.location_
    }
}

impl<T> Default for MaybeDirectHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for MaybeDirectHandle<T> {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.location_
    }
}

impl<T> DerefMut for MaybeDirectHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.location_
    }
}

impl<T> From<MaybeDirectHandle<T>> for Address {
    fn from(handle: MaybeDirectHandle<T>) -> Self {
        handle.location_
    }
}

impl<T> From<&MaybeDirectHandle<T>> for Address {
    fn from(handle: &MaybeDirectHandle<T>) -> Self {
        handle.location_
    }
}

impl<T> From<Address> for MaybeDirectHandle<T> {
    fn from(location: Address) -> Self {
        MaybeDirectHandle { location_: location, _phantom: PhantomData }
    }
}

impl<T> From<DirectHandle<T>> for MaybeDirectHandle<T> {
    fn from(handle: DirectHandle<T>) -> Self {
        MaybeDirectHandle { location_: handle.address(), _phantom: PhantomData }
    }
}

impl<T> From<&DirectHandle<T>> for MaybeDirectHandle<T> {
    fn from(handle: &DirectHandle<T>) -> Self {
        MaybeDirectHandle { location_: handle.address(), _phantom: PhantomData }
    }
}

impl<T> From<&mut DirectHandle<T>> for MaybeDirectHandle<T> {
    fn from(handle: &mut DirectHandle<T>) -> Self {
        MaybeDirectHandle { location_: handle.address(), _phantom: PhantomData }
    }
}

pub fn UncheckedCast_direct<To, From>(value: MaybeDirectHandle<From>) -> MaybeDirectHandle<To> {
    MaybeDirectHandle {
        location_: value.location_,
        _phantom: PhantomData,
    }
}

impl Default for MaybeObjectHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
pub struct MaybeObjectDirectHandle {
    reference_type_: HeapObjectReferenceType,
    handle_: MaybeDirectHandle<Object>,
}

impl MaybeObjectDirectHandle {
    #[inline]
    pub fn new() -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::new(),
        }
    }

    #[inline]
    pub fn from_tagged_maybe_object(object: Tagged<MaybeObject>, isolate: &mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged(
                unsafe { std::mem::transmute(object) },
                isolate,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_object(object: Tagged<Object>, isolate: &mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged(object, isolate),
        }
    }

    #[inline]
    pub fn from_tagged_smi(object: Tagged<Smi>, isolate: &mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged(
                unsafe { std::mem::transmute(object) },
                isolate,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_maybe_object_local_heap(object: Tagged<MaybeObject>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged_local_heap(
                unsafe { std::mem::transmute(object) },
                local_heap,
            ),
        }
    }

    #[inline]
    pub fn from_tagged_object_local_heap(object: Tagged<Object>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged_local_heap(object, local_heap),
        }
    }

    #[inline]
    pub fn from_tagged_smi_local_heap(object: Tagged<Smi>, local_heap: &mut LocalHeap) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_tagged_local_heap(
                unsafe { std::mem::transmute(object) },
                local_heap,
            ),
        }
    }

    #[inline]
    pub fn from_direct_handle(object: DirectHandle<Object>) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: MaybeDirectHandle::from_direct_handle(object),
        }
    }

    #[inline]
    pub fn from_maybe_object_handle(obj: MaybeObjectHandle) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: obj.reference_type_,
            handle_: MaybeDirectHandle::from_maybe_indirect_handle(unsafe { std::mem::transmute(obj.handle_) }),
        }
    }

    pub fn weak_tagged(object: Tagged<Object>, isolate: &mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::WEAK,
            handle_: MaybeDirectHandle::from_tagged(object, isolate),
        }
    }

    pub fn weak_direct_handle(object: DirectHandle<Object>) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::WEAK,
            handle_: MaybeDirectHandle::from_direct_handle(object),
        }
    }

    #[inline]
    pub fn operator_star(&self) -> Tagged<MaybeObject> {
        unsafe { std::mem::transmute_copy(&self.handle_.location_) }
    }

    #[inline]
    pub fn operator_arrow(&self) -> Tagged<MaybeObject> {
        unsafe { std::mem::transmute_copy(&self.handle_.location_) }
    }

    #[inline]
    pub fn object(&self) -> DirectHandle<Object> {
        self.handle_.to_handle_checked()
    }

    #[inline]
    pub fn is_identical_to(&self, other: &MaybeObjectDirectHandle) -> bool {
        self.handle_.location_ == other.handle_.location_
    }

    #[inline]
    pub fn is_identical_to_maybe_object_handle(&self, other: &MaybeObjectHandle) -> bool {
        unsafe { self.handle_.location_ as *mut Address == *other.handle_.get_location() }
    }

    pub fn is_null(&self) -> bool {
        self.handle_.is_null()
    }

    pub fn reference_type(&self) -> HeapObjectReferenceType {
        self.reference_type_
    }

    #[inline]
    pub fn from_tagged_object_reference_type(
        object: Tagged<Object>,
        reference_type: HeapObjectReferenceType,
        isolate: &mut Isolate,
    ) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: reference_type,
            handle_: MaybeDirectHandle::from_tagged(object, isolate),
        }
    }

    #[inline]
    pub fn from_direct_handle_reference_type(
        object: DirectHandle<Object>,
        reference_type: HeapObjectReferenceType,
    ) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: reference_type,
            handle_: MaybeDirectHandle::from_direct_handle(object),
        }
    }
}

impl Default for MaybeObjectDirectHandle {
    fn default() -> Self {
        Self::new()
    }
}

pub type Address = usize;

#[derive(Copy, Clone)]
pub struct Tagged<T> {
    address: Address,
    _phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new(address: Address) -> Self {
        Tagged { address, _phantom: PhantomData }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HeapObjectReferenceType {
    STRONG,
    WEAK,
}

#[derive(Copy, Clone)]
pub struct Handle<T> {
    location_: *mut Address,
}

impl<T> Handle<T> {
    pub fn null() -> Self {
        Handle { location_: std::ptr::null_mut() }
    }
}

impl<T> From<Handle<T>> for Handle<T> {
    fn from(handle: Handle<T>) -> Self {
        Handle { location_: handle.location_ }
    }
}

#[derive(Copy, Clone)]
pub struct DirectHandle<T> {
    handle_: Handle<T>,
}

impl<T> DirectHandle<T> {
    pub fn null() -> Self {
        DirectHandle { handle_: Handle { location_: std::ptr::null_mut() } }
    }

    pub fn new_with_handle(handle: Handle<T>) -> Self {
        DirectHandle { handle_: handle }
    }

    pub fn address(&self) -> Address {
        self.handle_.location_ as Address
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Direct_Handle {
    address: Address,
}

const kTaggedNullAddress: Address = 0;

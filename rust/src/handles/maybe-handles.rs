// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    fmt,
};

// Placeholder for handles.h
mod handles {
    pub struct Handle<T>(*mut T);

    impl<T> Handle<T> {
        pub fn null() -> Self {
            Handle(std::ptr::null_mut())
        }

        pub fn location_(&self) -> *mut T {
            self.0
        }

        pub fn new(ptr: *mut T) -> Self {
            Handle(ptr)
        }
    }

    impl<T> Copy for Handle<T> {}
    impl<T> Clone for Handle<T> {
        fn clone(&self) -> Self {
            Handle(self.0)
        }
    }

    pub struct IndirectHandle<T>(pub(crate) *mut T);

    impl<T> IndirectHandle<T> {
        pub fn is_null(&self) -> bool {
            self.0.is_null()
        }

        pub fn location_(&self) -> *mut T {
            self.0
        }

        pub fn new(ptr: *mut T) -> Self {
            IndirectHandle(ptr)
        }
    }

    impl<T> Copy for IndirectHandle<T> {}
    impl<T> Clone for IndirectHandle<T> {
        fn clone(&self) -> Self {
            IndirectHandle(self.0)
        }
    }

    pub struct DirectHandle<T>(pub(crate) Address);

    impl<T> DirectHandle<T> {
        pub fn null() -> Self {
            DirectHandle(0)
        }

        pub fn address(&self) -> Address {
            self.0
        }

        pub fn new(address: Address) -> Self {
            DirectHandle(address)
        }
    }

    impl<T> Copy for DirectHandle<T> {}
    impl<T> Clone for DirectHandle<T> {
        fn clone(&self) -> Self {
            DirectHandle(self.0)
        }
    }
}

use handles::*;

type Address = usize;

pub mod internal {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    pub struct NullMaybeHandleType {}

    pub const K_NULL_MAYBE_HANDLE: NullMaybeHandleType = NullMaybeHandleType {};

    /// A Handle can be converted into a MaybeHandle. Converting a MaybeHandle
    /// into a Handle requires checking that it does not point to nullptr. This
    /// ensures nullptr checks before use.
    ///
    /// Also note that MaybeHandles do not provide default equality comparison or
    /// hashing operators on purpose. Such operators would be misleading, because
    /// intended semantics is ambiguous between handle location and object identity.
    #[derive(Copy, Clone)]
    pub struct MaybeHandle<T> {
        location_: *mut Address,
        _phantom: PhantomData<T>,
    }

    impl<T> MaybeHandle<T> {
        pub fn new() -> Self {
            MaybeHandle {
                location_: std::ptr::null_mut(),
                _phantom: PhantomData,
            }
        }

        pub fn from_null(_: NullMaybeHandleType) -> Self {
            MaybeHandle {
                location_: std::ptr::null_mut(),
                _phantom: PhantomData,
            }
        }

        pub fn from_handle<S: IsSubtype<T>>(handle: Handle<S>) -> Self {
            MaybeHandle {
                location_: handle.location_() as *mut Address,
                _phantom: PhantomData,
            }
        }

        pub fn from_maybe_handle<S: IsSubtype<T>>(maybe_handle: MaybeHandle<S>) -> Self {
            MaybeHandle {
                location_: maybe_handle.location_,
                _phantom: PhantomData,
            }
        }

        // TODO: Add Tagged and Isolate/LocalHeap types and implementations.
        pub fn from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        // TODO: Implement Assert and Check functions (likely using debug_assert! and assert!)
        pub fn assert(&self) {
            assert!(!self.location_.is_null());
        }

        pub fn check(&self) {
            assert!(!self.location_.is_null());
        }

        pub fn to_handle_checked(&self) -> Handle<T> {
            self.check();
            Handle::<T>(self.location_ as *mut T)
        }

        pub fn to_handle<S>(&self, out: &mut Handle<S>) -> bool {
            if self.location_.is_null() {
                *out = Handle::<T>::null();
                false
            } else {
                *out = Handle::<T>(self.location_ as *mut T);
                true
            }
        }

        //TODO: Implement DirectHandle and TaggedNullAddress
        pub fn to_direct_handle<S>(&self, _out: &mut DirectHandle<S>) -> bool {
           unimplemented!()
        }

        pub fn equals(&self, other: MaybeHandle<T>) -> bool {
            self.address() == other.address()
        }

        pub fn address(&self) -> Address {
            self.location_ as Address
        }

        pub fn is_null(&self) -> bool {
            self.location_.is_null()
        }

        // protected constructor
        fn from_address(location: *mut Address) -> Self {
            MaybeHandle {
                location_: location,
                _phantom: PhantomData,
            }
        }
    }

    impl<T> Default for MaybeHandle<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> fmt::Debug for MaybeHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MaybeHandle")
                .field("location_", &self.location_)
                .finish()
        }
    }

    pub struct MaybeObjectHandle {
        reference_type_: HeapObjectReferenceType,
        handle_: MaybeHandle<Object>,
    }

    impl MaybeObjectHandle {
        pub fn new() -> Self {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: MaybeHandle::new(),
            }
        }

        // TODO: Add Tagged and Isolate/LocalHeap types and implementations.
        pub fn from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_object_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_smi_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

         pub fn from_tagged_object_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_smi_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_handle(object: Handle<Object>) -> Self {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: MaybeHandle::from_handle(object),
            }
        }

        pub fn weak_from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn weak_from_handle(_object: Handle<Object>) -> Self {
             unimplemented!()
        }

        // TODO: Implement deref operators.
        pub fn deref(&self) -> usize {
            unimplemented!()
        }

        pub fn object(&self) -> IndirectHandle<Object> {
            unimplemented!()
        }

        pub fn is_identical_to(&self, other: &MaybeObjectHandle) -> bool {
            unimplemented!()
        }

        pub fn is_null(&self) -> bool {
            self.handle_.is_null()
        }

        pub fn reference_type(&self) -> HeapObjectReferenceType {
            self.reference_type_
        }

        fn from_object_reference_type_isolate(_object: usize, _reference_type: HeapObjectReferenceType, _isolate: usize) -> Self {
            unimplemented!()
        }

        fn from_handle_reference_type(_object: Handle<Object>, _reference_type: HeapObjectReferenceType) -> Self {
             unimplemented!()
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum HeapObjectReferenceType {
        STRONG,
        WEAK,
    }

    pub trait IsSubtype<T> {}

    // Example implementations of IsSubtype
    impl IsSubtype<Object> for Object {}
    impl IsSubtype<Object> for JSArray {}
    impl<T> IsSubtype<T> for T {}

    // Dummy types for now
    #[derive(Debug, Copy, Clone)]
    pub struct Object {}
    #[derive(Debug, Copy, Clone)]
    pub struct JSArray {}
    #[derive(Debug, Copy, Clone)]
    pub struct Smi {}

    // Trait to represent a Tagged type.
    pub trait Tagged<T> {}

    impl Tagged<Object> for usize {}
    impl Tagged<JSArray> for usize {}
    impl Tagged<Smi> for usize {}
    impl Tagged<MaybeObject> for usize {}

    #[derive(Debug, Copy, Clone)]
    pub struct MaybeObject {}

    // Implementing Debug for MaybeIndirectHandle
    impl<T> fmt::Debug for MaybeIndirectHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MaybeIndirectHandle")
                .field("location_", &self.location_)
                .finish()
        }
    }

    #[derive(Copy, Clone)]
    pub struct MaybeIndirectHandle<T> {
        pub(crate) location_: *mut *mut T,
        _phantom: PhantomData<T>,
    }

    impl<T> MaybeIndirectHandle<T> {
        pub fn new() -> Self {
            MaybeIndirectHandle {
                location_: std::ptr::null_mut(),
                _phantom: PhantomData,
            }
        }

        pub fn is_null(&self) -> bool {
            self.location_.is_null()
        }

        pub fn assert(&self) {
            assert!(!self.location_.is_null());
        }

        pub fn check(&self) {
            assert!(!self.location_.is_null());
        }

        pub fn to_handle_checked(&self) -> DirectHandle<T> {
           unimplemented!()
        }

        pub fn to_handle<S>(&self, _out: &mut DirectHandle<S>) -> bool {
             unimplemented!()
        }
    }

    impl<T> Default for MaybeIndirectHandle<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> fmt::Display for MaybeIndirectHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MaybeIndirectHandle(location: {:?})", self.location_)
        }
    }

    //TODO: Implement MaybeDirectHandle when V8_ENABLE_DIRECT_HANDLE is enabled.

    #[cfg(not(feature = "v8_enable_direct_handle"))]
    pub struct MaybeDirectHandle<T> {
        handle_: MaybeIndirectHandle<T>,
    }

    #[cfg(not(feature = "v8_enable_direct_handle"))]
    impl<T> MaybeDirectHandle<T> {
        pub fn new() -> Self {
            MaybeDirectHandle {
                handle_: MaybeIndirectHandle::new(),
            }
        }

        pub fn from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_direct_handle<S: IsSubtype<T>>(_handle: DirectHandle<S>) -> Self {
            unimplemented!()
        }

        pub fn from_indirect_handle<S: IsSubtype<T>>(_handle: IndirectHandle<S>) -> Self {
            unimplemented!()
        }

        pub fn from_maybe_direct_handle<S: IsSubtype<T>>(_handle: MaybeDirectHandle<S>) -> Self {
            unimplemented!()
        }

        pub fn from_maybe_indirect_handle<S: IsSubtype<T>>(_handle: MaybeIndirectHandle<S>) -> Self {
            unimplemented!()
        }

        pub fn assert(&self) {
            self.handle_.assert();
        }

        pub fn check(&self) {
            self.handle_.check();
        }

        pub fn to_handle_checked(&self) -> DirectHandle<T> {
            self.handle_.to_handle_checked()
        }

        pub fn to_handle<S>(&self, out: &mut DirectHandle<S>) -> bool {
            self.handle_.to_handle(out)
        }

        pub fn is_null(&self) -> bool {
            self.handle_.is_null()
        }
    }

    #[cfg(not(feature = "v8_enable_direct_handle"))]
    impl<T> Default for MaybeDirectHandle<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(not(feature = "v8_enable_direct_handle"))]
    impl<T> fmt::Debug for MaybeDirectHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MaybeDirectHandle")
                .field("handle_", &self.handle_)
                .finish()
        }
    }

    #[cfg(feature = "v8_enable_direct_handle")]
    pub struct MaybeDirectHandle<T> {
        location_: Address,
        _phantom: PhantomData<T>,
    }

    #[cfg(feature = "v8_enable_direct_handle")]
    impl<T> MaybeDirectHandle<T> {
        pub fn new() -> Self {
             MaybeDirectHandle {
                location_: 0,
                _phantom: PhantomData,
            }
        }

        pub fn from_null(_: NullMaybeHandleType) -> Self {
           MaybeDirectHandle {
                location_: 0,
                _phantom: PhantomData,
            }
        }

        pub fn from_direct_handle<S: IsSubtype<T>>(handle: DirectHandle<S>) -> Self {
            MaybeDirectHandle {
                location_: handle.address(),
                _phantom: PhantomData,
            }
        }

        pub fn from_handle<S: IsSubtype<T>>(handle: Handle<S>) -> Self {
             MaybeDirectHandle {
                location_: 0,
                _phantom: PhantomData,
            }
        }

        pub fn from_maybe_direct_handle<S: IsSubtype<T>>(maybe_handle: MaybeDirectHandle<S>) -> Self {
            MaybeDirectHandle {
                location_: maybe_handle.location_,
                _phantom: PhantomData,
            }
        }

        pub fn from_maybe_indirect_handle<S: IsSubtype<T>>(_maybe_handle: MaybeIndirectHandle<S>) -> Self {
           MaybeDirectHandle {
                location_: 0,
                _phantom: PhantomData,
            }
        }

        pub fn from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_local_heap(_object: usize, _local_heap: usize) -> Self {
             unimplemented!()
        }

        pub fn assert(&self) {
            assert_ne!(self.location_, 0);
        }

        pub fn check(&self) {
            assert_ne!(self.location_, 0);
        }

        pub fn to_handle_checked(&self) -> DirectHandle<T> {
            self.check();
            DirectHandle::new(self.location_)
        }

        pub fn to_handle<S>(&self, out: &mut DirectHandle<S>) -> bool {
            if self.location_ == 0 {
                *out = DirectHandle::null();
                false
            } else {
                *out = DirectHandle::new(self.location_);
                true
            }
        }

        pub fn equals(&self, other: MaybeHandle<T>) -> bool {
            self.address() == other.address()
        }

        pub fn address(&self) -> Address {
            self.location_
        }

        pub fn is_null(&self) -> bool {
            self.location_ == 0
        }
    }

    #[cfg(feature = "v8_enable_direct_handle")]
    impl<T> Default for MaybeDirectHandle<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(feature = "v8_enable_direct_handle")]
    impl<T> fmt::Debug for MaybeDirectHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MaybeDirectHandle")
                .field("location_", &self.location_)
                .finish()
        }
    }

    #[derive(Copy, Clone)]
    pub struct MaybeObjectDirectHandle {
        reference_type_: HeapObjectReferenceType,
        handle_: MaybeDirectHandle<Object>,
    }

    impl MaybeObjectDirectHandle {
        pub fn new() -> Self {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: MaybeDirectHandle::new(),
            }
        }

        pub fn from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_object_isolate(_object: usize, _isolate: usize) -> Self {
             unimplemented!()
        }

        pub fn from_tagged_smi_isolate(_object: usize, _isolate: usize) -> Self {
             unimplemented!()
        }

        pub fn from_tagged_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_object_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_tagged_smi_local_heap(_object: usize, _local_heap: usize) -> Self {
            unimplemented!()
        }

        pub fn from_direct_handle(object: DirectHandle<Object>) -> Self {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: MaybeDirectHandle::from_direct_handle(object),
            }
        }

        pub fn from_maybe_object_handle(_obj: MaybeObjectHandle) -> Self {
            unimplemented!()
        }

        pub fn weak_from_tagged_isolate(_object: usize, _isolate: usize) -> Self {
             unimplemented!()
        }

        pub fn weak_from_direct_handle(_object: DirectHandle<Object>) -> Self {
            unimplemented!()
        }

        pub fn deref(&self) -> usize {
            unimplemented!()
        }

        pub fn object(&self) -> DirectHandle<Object> {
            self.handle_.to_handle_checked()
        }

        pub fn is_identical_to(&self, _other: &MaybeObjectDirectHandle) -> bool {
            unimplemented!()
        }

        pub fn is_identical_to_maybe_object_handle(&self, _other: &MaybeObjectHandle) -> bool {
            unimplemented!()
        }

        pub fn is_null(&self) -> bool {
            self.handle_.is_null()
        }

        pub fn reference_type(&self) -> HeapObjectReferenceType {
            self.reference_type_
        }

        fn from_object_reference_type_isolate(_object: usize, _reference_type: HeapObjectReferenceType, _isolate: usize) -> Self {
            unimplemented!()
        }
    }

    // Implement Debug for MaybeObjectDirectHandle
    impl fmt::Debug for MaybeObjectDirectHandle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MaybeObjectDirectHandle")
                .field("reference_type_", &self.reference_type_)
                .field("handle_", &self.handle_)
                .finish()
        }
    }

    macro_rules! assert_ne {
        ($left:expr, $right:expr) => {
            if $left == $right {
                panic!("assertion failed: `(left != right)`\
                       \n  left: `{}`,\
                       \n right: `{}`", $left, $right)
            }
        }
    }
}

use internal::*;

// Implement Debug for MaybeHandle
impl<T> fmt::Display for MaybeHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MaybeHandle(location: {:?})", self.location_)
    }
}

// Implement Display for MaybeDirectHandle
impl<T> fmt::Display for MaybeDirectHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "v8_enable_direct_handle"))]
        return write!(f, "MaybeDirectHandle(handle: {:?})", self.handle_);

        #[cfg(feature = "v8_enable_direct_handle")]
        return write!(f, "MaybeDirectHandle(location: {:?})", self.location_);
    }
}
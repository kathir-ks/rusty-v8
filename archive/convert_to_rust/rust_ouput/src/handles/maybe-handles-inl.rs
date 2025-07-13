// Converted from V8 C++ source files:
// Header: maybe-handles-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::Once;

struct Isolate {}
struct LocalHeap {}
struct Handle<T> {
    location_: *mut T,
    phantom: PhantomData<T>,
}
impl<T> Handle<T> {
    fn is_identical_to(&self, other: &Handle<T>) -> bool {
        self.location_ == other.location_
    }
    fn ToHandleChecked(&self) -> &T {
        unsafe { &*self.location_ }
    }
}
#[derive(Clone, Copy)]
struct DirectHandle<T> {
    location_: *mut T,
    phantom: PhantomData<T>,
}
impl<T> DirectHandle<T> {
    fn null() -> Self {
        DirectHandle {
            location_: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }
    fn ToHandle(&self, out: &mut DirectHandle<T>) -> bool {
        if self.location_.is_null() {
            *out = DirectHandle::<T>::null();
            false
        } else {
            *out = DirectHandle::<T> {
                location_: self.location_,
                phantom: PhantomData,
            };
            true
        }
    }
    fn is_identical_to(&self, other: &DirectHandle<T>) -> bool {
        self.location_ == other.location_
    }
    fn ToHandleChecked(&self) -> &T {
        unsafe { &*self.location_ }
    }
}

struct IndirectHandle<T> {
    location_: *mut *mut T,
    phantom: PhantomData<T>,
}
impl<T> IndirectHandle<T> {
    fn ToHandleChecked(&self) -> &T {
        unsafe { &*(*self.location_) }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HeapObjectReferenceType {
    WEAK,
    STRONG,
}

struct MaybeHandle<T> {
    location_: *mut T,
    phantom: PhantomData<T>,
}

impl<T> MaybeHandle<T> {
    fn new(object: Tagged<T>, isolate: *mut Isolate) -> Self {
        MaybeHandle {
            location_: unsafe { (object.ptr as *mut T) },
            phantom: PhantomData,
        }
    }

    fn new_local(object: Tagged<T>, local_heap: *mut LocalHeap) -> Self {
        MaybeHandle {
            location_: unsafe { (object.ptr as *mut T) },
            phantom: PhantomData,
        }
    }
    fn ToHandle<S>(&self, out: &mut DirectHandle<S>) -> bool {
        if self.location_.is_null() {
            *out = DirectHandle::<S>::null();
            false
        } else {
            *out = DirectHandle::<S> {
                location_: self.location_ as *mut S,
                phantom: PhantomData,
            };
            true
        }
    }
}

#[derive(Clone, Copy)]
struct Tagged<T> {
    ptr: *mut T,
    phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    fn GetHeapObjectIfWeak<U>(&self, out: &mut Tagged<U>) -> bool {
        if self.ptr as usize % 2 == 1 {
            out.ptr = (self.ptr as usize & !1) as *mut U;
            true
        } else {
            false
        }
    }
    fn IsCleared(&self) -> bool {
        self.ptr.is_null()
    }
}

struct MaybeObject {}
impl MaybeObject {
    fn GetHeapObjectIfWeak<T>(&self, out: &mut Tagged<T>) -> bool {
        true
    }
}
struct Object {}
impl Object {
    fn IsCleared(&self) -> bool {
        true
    }
}
struct Smi {}

fn handle<T>(object: Tagged<T>, isolate: *mut Isolate) -> Handle<T> {
    Handle {
        location_: object.ptr,
        phantom: PhantomData,
    }
}

fn handle<T>(object: Tagged<T>, local_heap: *mut LocalHeap) -> Handle<T> {
    Handle {
        location_: object.ptr,
        phantom: PhantomData,
    }
}
fn direct_handle<T>(object: Tagged<T>, isolate: *mut Isolate) -> DirectHandle<T> {
    DirectHandle {
        location_: object.ptr,
        phantom: PhantomData,
    }
}

fn direct_handle<T>(object: Tagged<T>, local_heap: *mut LocalHeap) -> DirectHandle<T> {
    DirectHandle {
        location_: object.ptr,
        phantom: PhantomData,
    }
}
fn indirect_handle<T>(handle: DirectHandle<T>, isolate: *mut Isolate) -> MaybeIndirectHandle<T> {
    MaybeIndirectHandle {
        dummy: 0,
        phantom: PhantomData,
    }
}
fn indirect_handle<T>(handle: DirectHandle<T>, isolate: *mut LocalIsolate) -> MaybeIndirectHandle<T> {
    MaybeIndirectHandle {
        dummy: 0,
        phantom: PhantomData,
    }
}
struct LocalIsolate {}

fn Cast<T>(object: Tagged<MaybeObject>) -> Tagged<Object> {
    Tagged {
        ptr: object.ptr as *mut Object,
        phantom: PhantomData,
    }
}

fn MakeWeak(object: &Object) -> Tagged<MaybeObject> {
    Tagged {
        ptr: object as *const Object as *mut MaybeObject,
        phantom: PhantomData,
    }
}

pub struct MaybeIndirectHandle<T> {
    dummy: i32,
    phantom: PhantomData<T>,
}
impl<T> MaybeIndirectHandle<T> {
    fn ToHandle(&self, handle: &mut IndirectHandle<T>) -> bool {
        true
    }
    fn is_null(&self) -> bool {
        true
    }
    fn ToHandleChecked(&self) -> &T {
       unsafe { std::mem::transmute(1usize) }
    }
}

pub struct MaybeDirectHandle<T> {
    location_: *mut T,
    phantom: PhantomData<T>,
}
impl<T> MaybeDirectHandle<T> {
    fn new(object: Tagged<T>, isolate: *mut Isolate) -> Self {
        MaybeDirectHandle {
            location_: unsafe { (object.ptr as *mut T) },
            phantom: PhantomData,
        }
    }

    fn new_local(object: Tagged<T>, local_heap: *mut LocalHeap) -> Self {
        MaybeDirectHandle {
            location_: unsafe { (object.ptr as *mut T) },
            phantom: PhantomData,
        }
    }
    fn ToHandle(&self, out: &mut DirectHandle<T>) -> bool {
        if self.location_.is_null() {
            *out = DirectHandle::<T>::null();
            false
        } else {
            *out = DirectHandle::<T> {
                location_: self.location_,
                phantom: PhantomData,
            };
            true
        }
    }
    fn is_null(&self) -> bool {
        true
    }
    fn ToHandleChecked(&self) -> &T {
        unsafe { &*self.location_ }
    }
    fn is_identical_to(&self, other: &MaybeObjectHandle) -> bool{
        true
    }
}
impl MaybeObjectDirectHandle {
    fn is_identical_to(&self, other: &MaybeObjectDirectHandle) -> bool{
        true
    }
}
impl Object {
    fn is_identical_to(&self, other: &Object) -> bool {
        true
    }
}

impl DirectHandle<Object> {
    fn is_identical_to(&self, other: &DirectHandle<Object>) -> bool{
        true
    }
}

impl Handle<Object> {
    fn is_identical_to(&self, other: &Handle<Object>) -> bool{
        true
    }
    fn ToHandleChecked(&self) -> &Object{
        unsafe { &*self.location_ }
    }
}

impl MaybeHandle<Object> {
    fn ToHandle(&self, out: &mut DirectHandle<Object>) -> bool{
        true
    }
}

impl MaybeObjectHandle {
    fn is_identical_to(&self, other: &MaybeObjectDirectHandle) -> bool{
        true
    }
}
impl IndirectHandle<Object> {
    fn ToHandleChecked(&self) -> &Object{
       unsafe { std::mem::transmute(1usize) }
    }
}

impl MaybeDirectHandle<Object> {
    fn ToHandle(&self, out: &mut DirectHandle<Object>) -> bool{
        true
    }
    fn ToHandleChecked(&self) -> &Object{
        unsafe { &*self.location_ }
    }
}

fn Is<T, U>(value: &U) -> bool {
    true
}

fn UncheckedCast<To, From>(value: MaybeIndirectHandle<From>) -> MaybeIndirectHandle<To> {
    MaybeIndirectHandle {
        dummy: 0,
        phantom: PhantomData,
    }
}

struct MaybeObjectHandle {
    reference_type_: HeapObjectReferenceType,
    handle_: Handle<Object>,
}

impl MaybeObjectHandle {
    fn new(object: Tagged<MaybeObject>, isolate: *mut Isolate) -> Self {
        let mut heap_object: Tagged<HeapObject> = Tagged {
            ptr: std::ptr::null_mut(),
            phantom: PhantomData,
        };
        assert!(!object.IsCleared());
        if object.GetHeapObjectIfWeak(&mut heap_object) {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::WEAK,
                handle_: handle(heap_object, isolate),
            }
        } else {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: handle(Cast(object), isolate),
            }
        }
    }

    fn new_local(object: Tagged<MaybeObject>, local_heap: *mut LocalHeap) -> Self {
        let mut heap_object: Tagged<HeapObject> = Tagged {
            ptr: std::ptr::null_mut(),
            phantom: PhantomData,
        };
        assert!(!object.IsCleared());
        if object.GetHeapObjectIfWeak(&mut heap_object) {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::WEAK,
                handle_: handle(heap_object, local_heap),
            }
        } else {
            MaybeObjectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: handle(Cast(object), local_heap),
            }
        }
    }

    fn from_handle(object: Handle<Object>) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: object,
        }
    }

    fn from_tagged_object(object: Tagged<Object>, isolate: *mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: handle(object, isolate),
        }
    }

    fn from_tagged_smi(object: Tagged<Smi>, isolate: *mut Isolate) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: handle(
                Tagged {
                    ptr: object.ptr as *mut Object,
                    phantom: PhantomData,
                },
                isolate,
            ),
        }
    }

    fn from_tagged_object_local(object: Tagged<Object>, local_heap: *mut LocalHeap) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: handle(object, local_heap),
        }
    }

    fn from_tagged_smi_local(object: Tagged<Smi>, local_heap: *mut LocalHeap) -> Self {
        MaybeObjectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: handle(
                Tagged {
                    ptr: object.ptr as *mut Object,
                    phantom: PhantomData,
                },
                local_heap,
            ),
        }
    }

    fn from_tagged_object_with_type(
        object: Tagged<Object>,
        reference_type: HeapObjectReferenceType,
        isolate: *mut Isolate,
    ) -> Self {
        MaybeObjectHandle {
            reference_type_: reference_type,
            handle_: handle(object, isolate),
        }
    }

    fn from_handle_with_type(object: Handle<Object>, reference_type: HeapObjectReferenceType) -> Self {
        MaybeObjectHandle {
            reference_type_: reference_type,
            handle_: object,
        }
    }

    fn Weak(object: Handle<Object>) -> MaybeObjectHandle {
        MaybeObjectHandle::from_handle_with_type(object, HeapObjectReferenceType::WEAK)
    }

    fn Weak_tagged(object: Tagged<Object>, isolate: *mut Isolate) -> MaybeObjectHandle {
        MaybeObjectHandle::from_tagged_object_with_type(
            object,
            HeapObjectReferenceType::WEAK,
            isolate,
        )
    }

    fn is_identical_to(&self, other: &MaybeObjectHandle) -> bool {
        let mut this_handle: DirectHandle<Object> = DirectHandle::<Object>::null();
        let mut other_handle: DirectHandle<Object> = DirectHandle::<Object>::null();
        self.reference_type_ == other.reference_type_
            && self.handle_.location_ == other.handle_.location_
    }

    fn operator_star(&self) -> Tagged<MaybeObject> {
        if self.reference_type_ == HeapObjectReferenceType::WEAK {
            return MakeWeak(self.handle_.ToHandleChecked());
        } else {
            return Tagged {
                ptr: self.handle_.location_ as *mut MaybeObject,
                phantom: PhantomData,
            };
        }
    }

    fn operator_arrow(&self) -> Tagged<MaybeObject> {
        if self.reference_type_ == HeapObjectReferenceType::WEAK {
            return MakeWeak(self.handle_.ToHandleChecked());
        } else {
            return Tagged {
                ptr: self.handle_.location_ as *mut MaybeObject,
                phantom: PhantomData,
            };
        }
    }

    fn object(&self) -> IndirectHandle<Object> {
        IndirectHandle {
            location_: &mut self.handle_.location_,
            phantom: PhantomData,
        }
    }
    fn ToHandle(&self, out: &mut DirectHandle<Object>) -> bool {
        if self.handle_.location_.is_null() {
            *out = DirectHandle::<Object>::null();
            false
        } else {
            *out = DirectHandle::<Object> {
                location_: self.handle_.location_,
                phantom: PhantomData,
            };
            true
        }
    }
}

fn handle_maybe_object(object: Tagged<MaybeObject>, isolate: *mut Isolate) -> MaybeObjectHandle {
    MaybeObjectHandle::new(object, isolate)
}

fn handle_maybe_object_local(object: Tagged<MaybeObject>, local_heap: *mut LocalHeap) -> MaybeObjectHandle {
    MaybeObjectHandle::new_local(object, local_heap)
}

use std::fmt;

impl<T> fmt::Display for MaybeIndirectHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else {
            write!(f, "{:?}", self.ToHandleChecked())
        }
    }
}

impl<T> fmt::Display for MaybeDirectHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else {
            write!(f, "{:?}", self.ToHandleChecked())
        }
    }
}

struct HeapObject {}

struct MaybeObjectDirectHandle {
    reference_type_: HeapObjectReferenceType,
    handle_: DirectHandle<Object>,
}

impl MaybeObjectDirectHandle {
    fn new(object: Tagged<MaybeObject>, isolate: *mut Isolate) -> Self {
        let mut heap_object: Tagged<HeapObject> = Tagged {
            ptr: std::ptr::null_mut(),
            phantom: PhantomData,
        };
        assert!(!object.IsCleared());
        if object.GetHeapObjectIfWeak(&mut heap_object) {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::WEAK,
                handle_: direct_handle(heap_object, isolate),
            }
        } else {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: direct_handle(Cast(object), isolate),
            }
        }
    }

    fn new_local(object: Tagged<MaybeObject>, local_heap: *mut LocalHeap) -> Self {
        let mut heap_object: Tagged<HeapObject> = Tagged {
            ptr: std::ptr::null_mut(),
            phantom: PhantomData,
        };
        assert!(!object.IsCleared());
        if object.GetHeapObjectIfWeak(&mut heap_object) {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::WEAK,
                handle_: direct_handle(heap_object, local_heap),
            }
        } else {
            MaybeObjectDirectHandle {
                reference_type_: HeapObjectReferenceType::STRONG,
                handle_: direct_handle(Cast(object), local_heap),
            }
        }
    }

    fn from_tagged_object(object: Tagged<Object>, isolate: *mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: direct_handle(object, isolate),
        }
    }

    fn from_tagged_smi(object: Tagged<Smi>, isolate: *mut Isolate) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: direct_handle(
                Tagged {
                    ptr: object.ptr as *mut Object,
                    phantom: PhantomData,
                },
                isolate,
            ),
        }
    }

    fn from_tagged_object_local(object: Tagged<Object>, local_heap: *mut LocalHeap) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: direct_handle(object, local_heap),
        }
    }

    fn from_tagged_smi_local(object: Tagged<Smi>, local_heap: *mut LocalHeap) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: HeapObjectReferenceType::STRONG,
            handle_: direct_handle(
                Tagged {
                    ptr: object.ptr as *mut Object,
                    phantom: PhantomData,
                },
                local_heap,
            ),
        }
    }

    fn from_tagged_object_with_type(
        object: Tagged<Object>,
        reference_type: HeapObjectReferenceType,
        isolate: *mut Isolate,
    ) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: reference_type,
            handle_: direct_handle(object, isolate),
        }
    }

    fn from_maybe_object_handle(object: MaybeObjectHandle) -> Self {
        MaybeObjectDirectHandle {
            reference_type_: object.reference_type_,
            handle_: DirectHandle{
                location_: object.handle_.location_,
                phantom: PhantomData
            },
        }
    }

    fn Weak(object: Tagged<Object>, isolate: *mut Isolate) -> MaybeObjectDirectHandle {
        MaybeObjectDirectHandle::from_tagged_object_with_type(
            object,
            HeapObjectReferenceType::WEAK,
            isolate,
        )
    }

    fn is_identical_to(&self, other: &MaybeObjectDirectHandle) -> bool {
        let mut this_handle: DirectHandle<Object> = DirectHandle::<Object>::null();
        let mut other_handle: DirectHandle<Object> = DirectHandle::<Object>::null();
        self.reference_type_ == other.reference_type_
            && self.handle_.location_ == other.handle_.location_
            && this_handle.is_identical_to(&other_handle)
    }

    fn is_identical_to(&self, other: &MaybeObjectHandle) -> bool {
        let mut this_handle: DirectHandle<Object> = DirectHandle::<Object>::null();
        let mut other_handle: Handle<Object> = Handle::<Object> {
            location_: std::ptr::null_mut(),
            phantom: PhantomData
        };
        self.reference_type_ == other.reference_type_
            && self.handle_.ToHandle(&mut this_handle) == true
            && true
            && this_handle.is_identical_to(&this_handle)
    }

    fn operator_star(&self) -> Tagged<MaybeObject> {
        if self.reference_type_ == HeapObjectReferenceType::WEAK {
            return MakeWeak(self.handle_.ToHandleChecked());
        } else {
            return Tagged {
                ptr: self.handle_.location_ as *mut MaybeObject,
                phantom: PhantomData,
            };
        }
    }

    fn operator_arrow(&self) -> Tagged<MaybeObject> {
        if self.reference_type_ == HeapObjectReferenceType::WEAK {
            return MakeWeak(self.handle_.ToHandleChecked());
        } else {
            return Tagged {
                ptr: self.handle_.location_ as *mut MaybeObject,
                phantom: PhantomData,
            };
        }
    }
}

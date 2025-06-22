// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod tagged_impl;
mod union;

use std::{
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use tagged_impl::*;
use union::*;

// Assume these are defined elsewhere or provided by a crate
// For simplicity, using usize as a placeholder for Address, TaggedIndex, etc.
pub type Address = usize;
pub type TaggedIndex = usize;

// Constant for Smi tag size
const K_SMI_TAG_SIZE: usize = 1; // Placeholder value

// Constant for HeapObject tag
const K_HEAP_OBJECT_TAG: usize = 1; // Placeholder value

// Null address constant
const K_NULL_ADDRESS: Address = 0;

// Constant for Weak Heap Object tag
const K_WEAK_HEAP_OBJECT_TAG: usize = 2; // Placeholder Value

// Placeholder traits to simulate C++ inheritance and type traits
pub trait HeapObjectTrait {}
pub trait HeapObjectLayoutTrait {}
pub trait TrustedObjectTrait {}
pub trait TrustedObjectLayoutTrait {}

// Placeholder structs, implement HeapObjectTrait if applicable
pub struct BigInt {}
pub struct FieldType {}
pub struct HeapObject {}
pub struct HeapNumber {}
pub struct HeapObjectLayout {}
pub struct TrustedObject {}
pub struct TrustedObjectLayout {}
pub struct Object {}
pub struct Smi {}

impl HeapObjectTrait for HeapObject {}
impl HeapObjectTrait for HeapObjectLayout {} //added because of casting
impl HeapObjectTrait for TrustedObject {}

pub struct ClearedWeakValue {}

// Marker trait for types that can be skipped type checking in raw pointer conversion
pub trait SkipTypeCheck {}

// Represents a reference to T that may be either strong or weak.
pub struct MaybeWeak<T>(PhantomData<T>);

// Trait for checking if a type is MaybeWeak
pub trait IsMaybeWeak {
    const VALUE: bool;
}

impl<T> IsMaybeWeak for MaybeWeak<T> {
    const VALUE: bool = true;
}

impl<T> IsMaybeWeak for T {
    const VALUE: bool = false;
}

macro_rules! is_maybe_weak {
    ($t:ty) => {
        <$t as IsMaybeWeak>::VALUE
    };
}

// Helper macro to define a trait and blanket implementations for `IsSubType`
macro_rules! define_is_subtype {
    ($trait_name:ident, $($base:ty => $($derived:ty),*);*) => {
        pub trait $trait_name<Base> {
            const VALUE: bool;
        }

        $(
            $(
                impl $trait_name<$base> for $derived {
                    const VALUE: bool = true;
                }
            )*
        )*

        impl<Derived, Base> $trait_name<Base> for Derived {
            default const VALUE: bool = false;
        }
    };
}

// Define `IsSubType` trait
define_is_subtype!(IsSubType,
    Object => Object, Smi, TaggedIndex, FieldType, HeapObject, HeapObjectLayout;
    MaybeWeak<T> where T: 'static => T, MaybeWeak<T>;
    MaybeWeak<T> where T: 'static => MaybeWeak<T>;
    MaybeWeak<Object> => Smi;
    Union<HeapObject, MaybeWeak<HeapObject>, Smi> => MaybeWeak<Object>;
    Union<HeapObject, Smi> => Object;
    Union<HeapObject, MaybeWeak<HeapObject>, Smi> => Object;
    FixedArrayBase => FixedArray, FixedDoubleArray, ByteArray, NameDictionary, NumberDictionary, OrderedHashMap, OrderedHashSet, OrderedNameDictionary, ScriptContextTable, ArrayList, SloppyArgumentsElements;
    FixedArray => FixedArrayBase;
    FixedDoubleArray => FixedArrayBase;
    ByteArray => FixedArrayBase;
    NameDictionary => FixedArrayBase;
    NumberDictionary => FixedArrayBase;
    OrderedHashMap => FixedArrayBase;
    OrderedHashSet => FixedArrayBase;
    OrderedNameDictionary => FixedArrayBase;
    ScriptContextTable => FixedArrayBase;
    ArrayList => FixedArrayBase;
    SloppyArgumentsElements => FixedArrayBase
);

// Macro for `is_subtype_v`
macro_rules! is_subtype_v {
    ($derived:ty, $base:ty) => {
        <$derived as IsSubType<$base>>::VALUE
    };
}

pub(crate) mod detail {
    use super::*;
    // TaggedOperatorArrowRef
    pub struct TaggedOperatorArrowRef<T> {
        object: T,
    }

    impl<T> TaggedOperatorArrowRef<T> {
        #[inline]
        pub(crate) const fn new(object: T) -> Self {
            TaggedOperatorArrowRef { object }
        }
    }

    impl<T> std::ops::Deref for TaggedOperatorArrowRef<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.object
        }
    }

    impl<T> std::ops::DerefMut for TaggedOperatorArrowRef<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.object
        }
    }

    pub struct BaseForTagged<T>(PhantomData<T>);

    impl BaseForTagged<FieldType> {
        pub type Type = Tagged<Object>;
    }

    impl BaseForTagged<Object> {
        pub type Type = Tagged<HeapObject>;
    }

    impl<T> BaseForTagged<MaybeWeak<T>> {
        pub type Type = Tagged<MaybeWeak<HeapObject>>;
    }

    impl<T> BaseForTagged<T> {
        pub type Type = Tagged<HeapObject>;
    }
}

// Main Tagged struct, specialized versions are below
pub struct Tagged<T> {
    ptr: Address,
    _phantom: PhantomData<T>,
}

impl<T> Copy for Tagged<T> {}
impl<T> Clone for Tagged<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Tagged<T> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn ptr(&self) -> Address {
        self.ptr
    }
}

// Specialization for Object
impl Tagged<Object> {
    #[inline]
    pub const fn new(o: Address) -> Self {
        Tagged {
            ptr: o,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: K_NULL_ADDRESS,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }
}

// Specialization for Smi
impl Tagged<Smi> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn is_heap_object(&self) -> bool {
        false
    }

    #[inline]
    pub const fn is_smi(&self) -> bool {
        true
    }

    #[inline]
    pub const fn value(&self) -> i32 {
        unsafe { internal_smi_value(self.ptr) as i32 }
    }
}

// Placeholder function for SmiValue, replace with actual implementation
unsafe fn internal_smi_value(ptr: Address) -> usize {
    ptr // Dummy implementation
}

// Specialization for TaggedIndex
impl Tagged<TaggedIndex> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn is_heap_object(&self) -> bool {
        false
    }

    #[inline]
    pub const fn is_smi(&self) -> bool {
        true
    }

    #[inline]
    pub const fn value(&self) -> isize {
        (self.ptr >> K_SMI_TAG_SIZE) as isize
    }
}

// Specialization for HeapObject
impl Tagged<HeapObject> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn deref(&self) -> &HeapObject {
        unsafe { &*self.to_raw_ptr() }
    }

    #[inline]
    pub const fn operator_arrow(&self) -> detail::TaggedOperatorArrowRef<HeapObject> {
        let raw_ptr = unsafe { &*self.to_raw_ptr() };
        detail::TaggedOperatorArrowRef::new(raw_ptr)
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.ptr == K_NULL_ADDRESS
    }

    #[inline]
    pub const fn is_heap_object(&self) -> bool {
        true
    }

    #[inline]
    pub const fn is_smi(&self) -> bool {
        false
    }

    #[inline]
    pub fn address(&self) -> Address {
        self.ptr - K_HEAP_OBJECT_TAG
    }

    #[inline]
    unsafe fn to_raw_ptr(&self) -> *mut HeapObject {
        assert!(is_taggable_v!(HeapObject, MaybeWeak<Object>));
        (self.ptr - K_HEAP_OBJECT_TAG) as *mut HeapObject
    }
}

// Specialization for MaybeWeak<Object>
impl Tagged<MaybeWeak<Object>> {
    #[inline]
    pub const fn new(o: Address) -> Self {
        Tagged {
            ptr: o,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: K_NULL_ADDRESS,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }
}

// Specialization for MaybeWeak<HeapObject>
impl Tagged<MaybeWeak<HeapObject>> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.ptr == K_NULL_ADDRESS
    }

    #[inline]
    pub const fn is_smi(&self) -> bool {
        false
    }
}

// Specialization for ClearedWeakValue
impl Tagged<ClearedWeakValue> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }
}

// Generic Tagged<T> for HeapObject
impl<T: HeapObjectTrait> Tagged<T> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_ptr(ptr: *const T) -> Self {
        assert!(std::mem::size_of::<T>() > 0);
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn to_raw_ptr(&self) -> *mut T {
        assert!(is_taggable_v!(T, MaybeWeak<Object>));
        (self.ptr - K_HEAP_OBJECT_TAG) as *mut T
    }
}

// Generic Tagged<MaybeWeak<T>> for HeapObject
impl<T: HeapObjectTrait> Tagged<MaybeWeak<T>> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_ptr(ptr: *const T) -> Self {
        assert!(std::mem::size_of::<T>() > 0);
        Tagged {
            ptr: (ptr as Address) + K_HEAP_OBJECT_TAG,
            _phantom: PhantomData,
        }
    }
}

// Generic Tagged<Union<Ts...>>
impl<Ts> Tagged<Union<Ts>> {
    #[inline]
    pub const fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn default() -> Self {
        Tagged {
            ptr: 0,
            _phantom: PhantomData,
        }
    }
}

// is_taggable definition, this determines which types are valid for Tagged
pub trait IsTaggable<T> {
    const VALUE: bool;
}

impl<T> IsTaggable<T> for Tagged<T> {
    default const VALUE: bool = false;
}

impl<T> IsTaggable<T> for Tagged<T>
where
    T: 'static,
{
    const VALUE: bool = is_subtype_v!(T, MaybeWeak<Object>);
}

macro_rules! is_taggable_v {
    ($t:ty, $m:ty) => {
        <Tagged<$t> as IsTaggable<$t>>::VALUE
    };
}

// is_castable definition
pub trait IsCastable<From, To> {
    const VALUE: bool;
}

impl<From, To> IsCastable<From, To> for Tagged<From> {
    default const VALUE: bool = false;
}

impl<From, To> IsCastable<From, To> for Tagged<From> {
    const VALUE: bool = is_subtype_v!(To, From) || is_subtype_v!(From, To);
}

macro_rules! is_castable_v {
    ($from:ty, $to:ty) => {
        <Tagged<$from> as IsCastable<$from, $to>>::VALUE
    };
}

const K_TAGGED_CAN_CONVERT_TO_RAW_OBJECTS: bool = true;

// Helper functions
#[inline]
pub fn make_weak<T>(value: Tagged<T>) -> Tagged<MaybeWeak<T>> {
    Tagged::<MaybeWeak<T>>::new(value.ptr() | K_WEAK_HEAP_OBJECT_TAG)
}

#[inline]
pub fn make_weak_maybe_weak<T>(value: Tagged<MaybeWeak<T>>) -> Tagged<MaybeWeak<T>> {
    Tagged::<MaybeWeak<T>>::new(value.ptr() | K_WEAK_HEAP_OBJECT_TAG)
}

#[inline]
pub fn make_strong<T>(value: Tagged<T>) -> Tagged<T> {
    Tagged::<T>::new(value.ptr() & !(K_WEAK_HEAP_OBJECT_TAG | K_HEAP_OBJECT_TAG))
}

#[inline]
pub fn make_strong_maybe_weak<T>(value: Tagged<MaybeWeak<T>>) -> Tagged<T> {
    Tagged::<T>::new(value.ptr() & !(K_WEAK_HEAP_OBJECT_TAG | K_HEAP_OBJECT_TAG))
}

pub type MaybeObject = MaybeWeak<Object>;
pub type HeapObjectReference = MaybeWeak<HeapObject>;

// FixedArrayBase hierarchy
pub struct FixedArrayBase {}
pub struct FixedArray {}
pub struct FixedDoubleArray {}
pub struct ByteArray {}
pub struct NameDictionary {}
pub struct NumberDictionary {}
pub struct OrderedHashMap {}
pub struct OrderedHashSet {}
pub struct OrderedNameDictionary {}
pub struct ScriptContextTable {}
pub struct ArrayList {}
pub struct SloppyArgumentsElements {}

// Remove Tagged Type
pub trait RemoveTagged {
    type Output;
}

impl<T> RemoveTagged for T {
    type Output = T;
}

impl<T> RemoveTagged for Tagged<T> {
    type Output = T;
}

// Impl block for std::common_type emulation
pub mod std_compat {
    use super::*;

    pub trait CommonType<T> {
        type Output;
    }

    impl<T> CommonType<T> for Object
    where
        Tagged<T>: IsTaggable<T>,
        T: 'static,
    {
        type Output = Object;
    }
}
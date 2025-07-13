// Converted from V8 C++ source files:
// Header: handles.h
// Implementation: handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(*mut u8);

impl Address {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn from_usize(addr: usize) -> Self {
        Address(addr as *mut u8)
    }
}

impl Default for Address {
    fn default() -> Self {
        Address(null_mut())
    }
}

struct NoCheckingTag {}

pub struct Handle<T> {
    location_: Address,
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn null() -> Self {
        Handle {
            location_: Address(null_mut()),
            _phantom: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.location_.is_null()
    }

    pub fn location(&self) -> Address {
        self.location_
    }
}

pub struct DirectHandle<T> {
    obj_: Address,
    _phantom: PhantomData<T>,
}

impl<T> DirectHandle<T> {
    pub fn null() -> Self {
        DirectHandle {
            obj_: Address(null_mut()),
            _phantom: PhantomData,
        }
    }

     pub fn is_null(&self) -> bool {
        self.obj_.is_null()
    }
}

pub struct DirectHandleUnchecked<T> {
    direct_handle: DirectHandle<T>,
}

impl<T> DirectHandleUnchecked<T> {
    pub fn new() -> Self {
        DirectHandleUnchecked {
            direct_handle: DirectHandle {
                obj_: Address(null_mut()),
                _phantom: PhantomData,
            },
        }
    }
}

pub struct HandleScope {
    isolate_: *mut Isolate,
}

impl HandleScope {
    pub fn new(isolate: *mut Isolate) -> Self {
        HandleScope { isolate_: isolate }
    }
}

pub struct SealHandleScope {
    isolate_: *mut Isolate,
}

impl SealHandleScope {
    pub fn new(isolate: *mut Isolate) -> Self {
        SealHandleScope { isolate_: isolate }
    }
}

struct HandleScopeData {
    next: Address,
    limit: Address,
    level: i32,
    sealed_level: i32,
}

impl HandleScopeData {
    fn new() -> Self {
        HandleScopeData {
            next: Address(null_mut()),
            limit: Address(null_mut()),
            level: 0,
            sealed_level: 0,
        }
    }
}

struct LocalHeap {}
struct LocalIsolate {}
struct Isolate {
    handle_scope_data_: HandleScopeData,
}

impl Isolate {
    fn new() -> Self {
        Isolate {
            handle_scope_data_: HandleScopeData::new(),
        }
    }
}

mod api_internal {
    pub struct StackAllocated<const kCheck: bool>;
}
mod base {
    pub struct SmallVector<T, const SIZE: usize>;
    pub type Vector<T> = std::vec::Vec<T>;
}
pub struct HeapObject {}
pub struct Tagged<T> {
    ptr: Address,
    _phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }
    fn ptr(&self) -> Address {
        self.ptr
    }
}

impl Tagged<HeapObject> {
     fn from_address(address: Address) -> Self {
        Tagged {
            ptr: address,
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for Tagged<T> {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> DerefMut for Tagged<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}

struct MaybeHandle<T> {
    handle: Handle<T>,
}

struct MaybeDirectHandle<T> {
    direct_handle: DirectHandle<T>,
}

impl<T> MaybeDirectHandle<T> {
    fn new() -> Self {
        MaybeDirectHandle {
            direct_handle: DirectHandle::null(),
        }
    }
}

trait Subtype<T> {}
struct JSFunction {}
struct Object {}

impl Subtype<Object> for JSFunction {}

struct IndirectHandle<T> {
   location_: Address,
    _phantom: PhantomData<T>,
}

impl<T> IndirectHandle<T> {
      pub fn is_null(&self) -> bool {
        self.location_.is_null()
    }

    pub fn location(&self) -> Address {
        self.location_
    }
}

mod Utils {
    pub fn ApiCheck(condition: bool, _api_name: &str, _error_message: &str) -> bool {
        condition
    }
}

struct HandleScopeImplementer {}

impl HandleScopeImplementer {
    fn blocks(&self) -> &Vec<Address> {
        todo!()
    }
    fn DeleteExtensions(&self, _limit: Address){
        todo!()
    }
    fn GetSpareOrNewBlock(&self) -> Address {
       todo!()
    }
}

const kHandleBlockSize: usize = 1024;
const kSystemPointerSize: usize = 8;
const kInt32Size: usize = 4;

macro_rules! ASSERT_TRIVIALLY_COPYABLE {
    ($t:ty) => {
        static_assertions::assert_impl_all!($t: Copy, Clone);
    };
}

macro_rules! V8_INLINE {
    ($item:item) => {
        #[inline(always)]
        $item
    };
}

V8_INLINE! {
    fn IsSmi(_obj: Address) -> bool {
        false
    }
}

V8_INLINE! {
    fn Cast<T>(_obj: Address) -> Tagged<T> {
        Tagged {
            ptr: Address(null_mut()),
            _phantom: PhantomData,
        }
    }
}

mod static_assertions {
    pub use static_assertions::*;
}

mod std {
    pub mod marker {
        pub use std::marker::PhantomData;
    }
    pub mod ops {
        pub use std::ops::{Deref, DerefMut};
    }
    pub mod collections {
        pub mod hash_map {
            pub use std::collections::hash_map::HashMap;
        }
    }
    pub mod ptr {
        pub use std::ptr::null_mut;
    }
}
mod v8 {
    pub mod base {
        pub fn hash<T>() -> impl Fn(T) -> usize {
            |t: T| {
                use std::any::Any;
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                let any = &t as &dyn Any;
                if let Some(s) = any.downcast_ref::<String>() {
                    s.hash(&mut hasher);
                } else if let Some(i) = any.downcast_ref::<i32>() {
                    i.hash(&mut hasher);
                } else if let Some(u) = any.downcast_ref::<usize>() {
                    u.hash(&mut hasher);
                } else {
                   panic!("Unsupported type for hashing.");
                }
                 hasher.finish() as usize
            }
        }
        #[allow(non_camel_case_types)]
        pub struct SmallVector<T, const SIZE: usize>;
       pub type Vector<T> = std::vec::Vec<T>;
    }
}

pub mod v8_handle_base {
    pub struct no_checking_tag {}
}

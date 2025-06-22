// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

// TODO: Add mock implementations for v8-handle-base.h and v8-internal.h
// For now, using usize as a placeholder for internal::Address

pub mod internal {
    pub type Address = usize;

    pub trait ValueHelper {
        fn value_as_address<T>(value: *mut T) -> Address;
        fn is_empty<T>(value: *mut T) -> bool;
        type InternalRepresentationType; // Placeholder
    }

    pub trait HandleHelper {
        fn equal_handles<T, U>(handle1: &Local<T>, handle2: &Local<U>) -> bool;
        fn equal_handles<T, U>(handle1: &Local<T>, handle2: &PersistentBase<U>) -> bool;
    }

    pub struct LocalUnchecked<T>(pub(crate) Local<T>);

    pub trait SamplingHeapProfiler {}

    pub trait CustomArguments<T> {}
}

pub mod api_internal {
    pub trait DirectHandleBase {}
    pub trait IndirectHandleBase {}
    pub trait StackAllocated<const B: bool> {}

    extern "C" {
        pub fn ToLocalEmpty();
    }
}

/// A stack-allocated class that governs a number of local handles.
/// After a handle scope has been created, all local handles will be
/// allocated within that handle scope until either the handle scope is
/// deleted or another handle scope is created.  If there is already a
/// handle scope and a new one is created, all allocations will take
/// place in the new handle scope until it is deleted.  After that,
/// new handles will again be allocated in the original handle scope.
///
/// After the handle scope of a local handle has been deleted the
/// garbage collector will no longer track the object stored in the
/// handle and may deallocate it.  The behavior of accessing a handle
/// for which the handle scope has been deleted is undefined.
#[derive(Debug)]
pub struct HandleScope<'i> {
    i_isolate_: *mut Isolate,
    prev_next_: *mut internal::Address,
    prev_limit_: *mut internal::Address,
    scope_level_: i32,
    _phantom: PhantomData<&'i Isolate>,
}

impl<'i> HandleScope<'i> {
    pub fn new(isolate: &'i mut Isolate) -> Self {
        let mut scope = Self {
            i_isolate_: isolate,
            prev_next_: std::ptr::null_mut(),
            prev_limit_: std::ptr::null_mut(),
            scope_level_: 0,
            _phantom: PhantomData,
        };
        scope.initialize(isolate);
        scope
    }

    fn initialize(&mut self, isolate: &mut Isolate) {
         // Placeholder implementation. Real implementation would manipulate the isolate's internal handle stack.
        self.i_isolate_ = isolate;
    }

    /// Counts the number of allocated handles.
    pub fn number_of_handles(isolate: &Isolate) -> i32 {
        // Placeholder implementation.  Real implementation would query the isolate's internal handle state.
        0
    }

    #[inline]
    pub fn get_isolate(&self) -> &Isolate {
        unsafe { &*self.i_isolate_ }
    }

    pub fn create_handle_for_current_isolate(value: internal::Address) -> *mut internal::Address {
        // Placeholder implementation. Real implementation would allocate and manage a handle.
        Box::into_raw(Box::new(value)) as *mut internal::Address
    }

    fn create_handle(i_isolate: *mut Isolate, value: internal::Address) -> *mut internal::Address {
        // Placeholder implementation. Real implementation would allocate and manage a handle.
        Box::into_raw(Box::new(value)) as *mut internal::Address
    }
}

impl<'i> Drop for HandleScope<'i> {
    fn drop(&mut self) {
        // Placeholder implementation.  Real implementation would unwind handle allocations.
    }
}

/// A base class for local handles.
/// Its implementation depends on whether direct handle support is enabled.
/// When it is, a local handle contains a direct pointer to the referenced
/// object, otherwise it contains an indirect pointer.
#[derive(Debug)]
pub struct LocalBase<T> {
    ptr: internal::Address,
    _phantom: PhantomData<T>,
}

impl<T> LocalBase<T> {
    #[inline]
    fn new(ptr: internal::Address) -> Self {
        LocalBase {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn new_with_isolate(isolate: &mut Isolate, value: internal::Address) -> Self {
        // Placeholder implementation.  Should use the Isolate to create a handle.
        let _ = isolate;
        LocalBase {
            ptr: value,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn new_with_isolate_t(isolate: &mut Isolate, that: *mut T) -> Self {
        let _ = isolate;
        let addr = unsafe {internal::ValueHelper::value_as_address(that)};
        LocalBase {
            ptr: addr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn from_slot(slot: *mut internal::Address) -> Self {
        if slot.is_null() {
            LocalBase {
                ptr: 0,
                _phantom: PhantomData,
            }
        } else {
            let ptr = unsafe { *slot };
            LocalBase {
                ptr,
                _phantom: PhantomData,
            }
        }
    }

    #[inline]
    fn from_repr(repr: <internal::ValueHelper as internal::ValueHelper>::InternalRepresentationType) -> Self {
        //TODO
        let _ = repr;
        LocalBase {
            ptr: 0, //PLACEHOLDER
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for LocalBase<T> {
    fn clone(&self) -> Self {
        LocalBase {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for LocalBase<T> {}

/// An object reference managed by the v8 garbage collector.
///
/// All objects returned from v8 have to be tracked by the garbage collector so
/// that it knows that the objects are still alive.  Also, because the garbage
/// collector may move objects, it is unsafe to point directly to an object.
/// Instead, all objects are stored in handles which are known by the garbage
/// collector and updated whenever an object moves.  Handles should always be
/// passed by value (except in cases like out-parameters) and they should never
/// be allocated on the heap.
///
/// There are two types of handles: local and persistent handles.
///
/// Local handles are light-weight and transient and typically used in local
/// operations.  They are managed by HandleScopes. That means that a HandleScope
/// must exist on the stack when they are created and that they are only valid
/// inside of the HandleScope active during their creation. For passing a local
/// handle to an outer HandleScope, an EscapableHandleScope and its Escape()
/// method must be used.
///
/// Persistent handles can be used when storing objects across several
/// independent operations and have to be explicitly deallocated when they're no
/// longer used.
///
/// It is safe to extract the object stored in the handle by dereferencing the
/// handle (for instance, to extract the Object* from a Local<Object>); the value
/// will still be governed by a handle behind the scenes and the same rules apply
/// to these values as to their handles.
#[derive(Debug, Clone, Copy)]
pub struct Local<T> {
    base: LocalBase<T>,
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> Local<T> {
    pub(crate) const do_not_check: NoCheckingTag = NoCheckingTag {};

    #[inline]
    pub fn new() -> Self {
        Local {
            base: LocalBase::new(0),
            _phantom: PhantomData,
        }
    }

    pub fn unsafe_as<S>(&self) -> Local<S> {
        Local {
            base: LocalBase {
                ptr: self.base.ptr,
                _phantom: PhantomData,
            },
            _phantom: PhantomData,
        }
    }

    pub fn as_<S>(&self) -> Local<S> {
        // TODO: Add type checking.
        Local {
            base: LocalBase {
                ptr: self.base.ptr,
                _phantom: PhantomData,
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn cast<S>(_that: Local<S>) -> Local<T> {
        // TODO: Add type checking.
        Local {
            base: LocalBase {
                ptr: _that.base.ptr,
                _phantom: PhantomData,
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new_with_isolate(isolate: &mut Isolate, that: Local<T>) -> Self {
        Local {
            base: LocalBase::new_with_isolate(isolate, that.base.ptr),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new_with_isolate_base(isolate: &mut Isolate, that: &PersistentBase<T>) -> Self {
        Local {
            base: LocalBase::new_with_isolate(isolate, that.ptr),
            _phantom: PhantomData,
        }
    }

        #[inline]
    pub fn new_with_isolate_bt(isolate: &mut Isolate, that: &BasicTracedReference<T>) -> Self {
        Local {
            base: LocalBase::new_with_isolate(isolate, that.ptr),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_slot(slot: *mut internal::Address) -> Self {
        Local {
            base: LocalBase::from_slot(slot),
            _phantom: PhantomData,
        }
    }

        #[inline]
    pub fn from_repr(repr: <internal::ValueHelper as internal::ValueHelper>::InternalRepresentationType) -> Self {
        Local {
            base: LocalBase::from_repr(repr),
            _phantom: PhantomData,
        }
    }

        #[inline]
    pub fn new_with_isolate_t(isolate: &mut Isolate, that: *mut T) -> Self {
        Local {
            base: LocalBase::new_with_isolate_t(isolate, that),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_address(ptr: internal::Address) -> Self {
        Local {
            base: LocalBase::new(ptr),
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for Local<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //TODO: Implement
        panic!()
        //unsafe { &*(self.base.ptr as *const T) } // Placeholder
    }
}

impl<T> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        //TODO: Implement
        panic!()
       // unsafe { &mut *(self.base.ptr as *mut T) } // Placeholder
    }
}

impl<T, U> PartialEq<Local<U>> for Local<T> {
    fn eq(&self, other: &Local<U>) -> bool {
        // Placeholder implementation. Real implementation should compare handles.
        self.base.ptr == other.base.ptr
    }
}

impl<T, U> PartialEq<PersistentBase<U>> for Local<T> {
    fn eq(&self, other: &PersistentBase<U>) -> bool {
         self.base.ptr == other.ptr
    }
}

impl<T> Eq for Local<T> {}

// Implement the "not equal" operator using the "equal" operator
impl<T, U> std::cmp::PartialEq<&Persistent<U>> for &Local<T> {
    fn eq(&self, other: &&Persistent<U>) -> bool {
        internal::HandleHelper::equal_handles(self, other)
    }
}

// Implement the "not equal" operator using the "equal" operator
impl<T, U> std::cmp::PartialEq<&Local<U>> for &Local<T> {
    fn eq(&self, other: &&Local<U>) -> bool {
        internal::HandleHelper::equal_handles(self, other)
    }
}

pub struct LocalVector<T> {
    backing_: Vec<internal::LocalUnchecked<T>>,
    _phantom: PhantomData<T>,
}

impl<T> LocalVector<T> {
    pub fn new(isolate: &mut Isolate) -> Self {
        let _ = isolate;
        LocalVector {
            backing_: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn with_capacity(isolate: &mut Isolate, n: usize) -> Self {
         let _ = isolate;
        LocalVector {
            backing_: Vec::with_capacity(n),
            _phantom: PhantomData,
        }
    }

    pub fn from_initializer_list(isolate: &mut Isolate, init: &[Local<T>]) -> Self {
         let _ = isolate;
        let mut vec = Vec::new();
        vec.reserve(init.len());
        for &item in init {
            vec.push(internal::LocalUnchecked(item));
        }
        LocalVector {
            backing_: vec,
            _phantom: PhantomData,
        }
    }

    pub fn begin(&self) -> std::slice::Iter<internal::LocalUnchecked<T>> {
        self.backing_.iter()
    }

    pub fn end(&self) -> std::slice::Iter<internal::LocalUnchecked<T>> {
        self.backing_.iter()
    }

    pub fn size(&self) -> usize {
        self.backing_.len()
    }

    pub fn empty(&self) -> bool {
        self.backing_.is_empty()
    }

    pub fn reserve(&mut self, n: usize) {
        self.backing_.reserve(n);
    }

    pub fn shrink_to_fit(&mut self) {
        self.backing_.shrink_to_fit();
    }

    pub fn get(&self, n: usize) -> Local<T> {
        self.backing_[n].0
    }

    pub fn get_mut(&mut self, n: usize) -> Local<T> {
        self.backing_[n].0
    }

    pub fn at(&self, n: usize) -> Local<T> {
        self.backing_.at(n).map(|l|l.0).unwrap()
    }

    pub fn at_mut(&mut self, n: usize) -> Local<T> {
        self.backing_.at_mut(n).map(|l|l.0).unwrap()
    }

    pub fn front(&self) -> Local<T> {
        self.backing_.first().map(|l|l.0).unwrap()
    }

    pub fn front_mut(&mut self) -> Local<T> {
        self.backing_.first_mut().map(|l|l.0).unwrap()
    }

    pub fn back(&self) -> Local<T> {
        self.backing_.last().map(|l|l.0).unwrap()
    }

    pub fn back_mut(&mut self) -> Local<T> {
        self.backing_.last_mut().map(|l|l.0).unwrap()
    }

    pub fn data(&self) -> *const internal::LocalUnchecked<T> {
        self.backing_.as_ptr()
    }

    pub fn data_mut(&mut self) -> *mut internal::LocalUnchecked<T> {
        self.backing_.as_mut_ptr()
    }

    pub fn insert(&mut self, pos: usize, value: Local<T>) {
        self.backing_.insert(pos, internal::LocalUnchecked(value));
    }

    pub fn insert_from_slice(&mut self, pos: usize, slice: &[Local<T>]) {
        let unchecked_slice: Vec<internal::LocalUnchecked<T>> =
            slice.iter().map(|&local| internal::LocalUnchecked(local)).collect();
        self.backing_.splice(pos..pos, unchecked_slice.into_iter());
    }

    pub fn push_back(&mut self, x: Local<T>) {
        self.backing_.push(internal::LocalUnchecked(x));
    }

    pub fn pop_back(&mut self) {
        self.backing_.pop();
    }

    pub fn clear(&mut self) {
        self.backing_.clear();
    }

    pub fn resize(&mut self, n: usize, value: Local<T>) {
        self.backing_.resize(n, internal::LocalUnchecked(value));
    }

    pub fn swap(&mut self, other: &mut LocalVector<T>) {
        self.backing_.swap_with_slice(&mut other.backing_);
    }
}

impl<T> PartialEq for LocalVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.backing_ == other.backing_
    }
}

impl<T> Eq for LocalVector<T> {}

pub type Handle<T> = Local<T>;

/// A MaybeLocal<> is a wrapper around Local<> that enforces a check whether
/// the Local<> is empty before it can be used.
///
/// If an API method returns a MaybeLocal<>, the API method can potentially fail
/// either because an exception is thrown, or because an exception is pending,
/// e.g. because a previous API call threw an exception that hasn't been caught
/// yet, or because a TerminateExecution exception was thrown. In that case, an
/// empty MaybeLocal is returned.
#[derive(Debug, Clone, Copy)]
pub struct MaybeLocal<T> {
    local_: Local<T>,
}

impl<T> MaybeLocal<T> {
    #[inline]
    pub fn new() -> Self {
        MaybeLocal { local_: Local::new() }
    }

    #[inline]
    pub fn from_local(that: Local<T>) -> Self {
        MaybeLocal { local_: that }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.local_.base.ptr == 0
    }

    /// Converts this MaybeLocal<> to a Local<>. If this MaybeLocal<> is empty,
    /// |false| is returned and |out| is assigned with nullptr.
    #[inline]
    pub fn to_local(&self) -> Option<Local<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.local_)
        }
    }

    /// Converts this MaybeLocal<> to a Local<>. If this MaybeLocal<> is empty,
    /// V8 will crash the process.
    #[inline]
    pub fn to_local_checked(&self) -> Local<T> {
        if self.is_empty() {
            unsafe { api_internal::ToLocalEmpty() };
        }
        self.local_
    }

    /// Converts this MaybeLocal<> to a Local<>, using a default value if this
    /// MaybeLocal<> is empty.
    #[inline]
    pub fn from_maybe(&self, default_value: Local<T>) -> Local<T> {
        if self.is_empty() {
            default_value
        } else {
            self.local_
        }
    }

     /// Cast a handle to a subclass, e.g. MaybeLocal<Value> to MaybeLocal<Object>.
    /// This is only valid if the handle actually refers to a value of the target
    /// type.
    #[inline]
    pub fn cast<S>(that: MaybeLocal<S>) -> MaybeLocal<T> {
        // TODO: Add type checking.
        if that.is_empty() {
           return MaybeLocal { local_: Local::new() };
        }
        MaybeLocal { local_: Local::cast(that.local_) }
    }

      /// Calling this is equivalent to MaybeLocal<S>::Cast().
    /// In particular, this is only valid if the handle actually refers to a value
    /// of the target type.
    #[inline]
    pub fn as_<S>(&self) -> MaybeLocal<S> {
        // TODO: Add type checking.
         MaybeLocal { local_: self.local_.as_() }
    }
}

/// A HandleScope which first allocates a handle in the current scope
/// which will be later filled with the escape value.
#[derive(Debug)]
pub struct EscapableHandleScopeBase<'i> {
    handle_scope: HandleScope<'i>,
    escape_slot_: *mut internal::Address,
}

impl<'i> EscapableHandleScopeBase<'i> {
    pub fn new(isolate: &'i mut Isolate) -> Self {
        EscapableHandleScopeBase {
            handle_scope: HandleScope::new(isolate),
            escape_slot_: std::ptr::null_mut(),
        }
    }

    /// Pushes the value into the previous scope and returns a handle to it.
    /// Cannot be called twice.
    fn escape_slot(&mut self, escape_value: *mut internal::Address) -> *mut internal::Address {
        // Placeholder implementation. Real implementation would manage the escape slot in the Isolate.
        self.escape_slot_ = escape_value;
        self.escape_slot_
    }
}

/// A SealHandleScope acts like a handle scope in which no handle allocations
/// are allowed. It can be useful for debugging handle leaks.
/// Handles can be allocated within inner normal HandleScopes.
#[derive(Debug)]
pub struct SealHandleScope<'i> {
    i_isolate_: *mut Isolate,
    prev_limit_: *mut internal::Address,
    prev_sealed_level_: i32,
    _phantom: PhantomData<&'i Isolate>,
}

impl<'i> SealHandleScope<'i> {
    pub fn new(isolate: &'i mut Isolate) -> Self {
        // Placeholder implementation. Real implementation would manage Isolate's internal state.
        SealHandleScope {
            i_isolate_: isolate,
            prev_limit_: std::ptr::null_mut(),
            prev_sealed_level_: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'i> Drop for SealHandleScope<'i> {
    fn drop(&mut self) {
        // Placeholder implementation.  Real implementation would revert Isolate's internal state.
    }
}

impl<'i> Drop for EscapableHandleScopeBase<'i> {
    fn drop(&mut self) {
        // Placeholder implementation.  Real implementation would revert Isolate's internal state.
    }
}

#[derive(Debug)]
pub struct EscapableHandleScope<'i> : EscapableHandleScopeBase<'i> {
    _phantom: PhantomData<&'i Isolate>,
}

impl<'i> EscapableHandleScope<'i> {
    pub fn new(isolate: &'i mut Isolate) -> Self {
        EscapableHandleScope{
            _phantom: PhantomData,
            ..EscapableHandleScopeBase::new(isolate)
        }
    }

    #[inline]
    pub fn escape<T>(&mut self, value: Local<T>) -> Local<T> {
        if value.base.ptr == 0 {
            return value;
        }
        let slot = value.base.ptr as *mut internal::Address;
        Local::from_slot(EscapableHandleScopeBase::escape_slot(self, slot))
    }

      #[inline]
    pub fn escape_maybe<T>(&mut self, value: MaybeLocal<T>) -> MaybeLocal<T> {
        MaybeLocal::from_local(self.escape(value.from_maybe(Local::new())))
    }
}

// Forward declarations
pub struct Isolate {}
pub struct PersistentBase<T> {
    ptr: internal::Address,
    _phantom: PhantomData<T>,
}
pub struct BasicTracedReference<T> {
    ptr: internal::Address,
    _phantom: PhantomData<T>,
}
pub struct Persistent<T, M> {
    _phantom: PhantomData<(T, M)>,
}

pub struct Boolean {}
pub struct Context {}
pub struct Object {}
pub struct Primitive {}
pub struct Private {}
pub struct String {}
pub struct TypecheckWitness {}
pub struct Utils {}

pub mod debug {
    pub struct ConsoleCallArguments {}
}

pub mod internal_escapable_scope {}
pub struct NoCheckingTag {}
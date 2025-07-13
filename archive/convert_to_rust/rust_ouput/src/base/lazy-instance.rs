// Converted from V8 C++ source files:
// Header: lazy-instance.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem::MaybeUninit;
use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;

pub struct V8_ONCE_INIT;

impl V8_ONCE_INIT {
    pub const fn new() -> Self {
        V8_ONCE_INIT {}
    }
}

macro_rules! lazy_static_instance_initializer {
    () => {
        (ONCE_INIT, MaybeUninit::uninit())
    };
}

macro_rules! lazy_dynamic_instance_initializer {
    () => {
        (ONCE_INIT, std::ptr::null_mut())
    };
}

// Default to static mode.
macro_rules! lazy_instance_initializer {
    () => {
        lazy_static_instance_initializer!()
    };
}

pub(crate) use lazy_dynamic_instance_initializer;
pub(crate) use lazy_instance_initializer;
pub(crate) use lazy_static_instance_initializer;

pub struct LeakyInstanceTrait<T> {
    _phantom: PhantomData<T>,
}

impl<T> LeakyInstanceTrait<T> {
    pub fn destroy(_instance: *mut T) {}
}

pub struct StaticallyAllocatedInstanceTrait<T> {
    _phantom: PhantomData<T>,
}

impl<T> StaticallyAllocatedInstanceTrait<T> {
    pub type StorageType = MaybeUninit<T>;
    pub type AlignmentType = T;

    pub fn mutable_instance(storage: &mut Self::StorageType) -> *mut T {
        storage.as_mut_ptr() as *mut T
    }

    pub fn init_storage_using_trait<ConstructTrait>(storage: &mut Self::StorageType)
    where
        ConstructTrait: ConstructTraitInterface<T>,
    {
        unsafe {
            ConstructTrait::construct(storage.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }
}

pub struct DynamicallyAllocatedInstanceTrait<T> {
    _phantom: PhantomData<T>,
}

impl<T> DynamicallyAllocatedInstanceTrait<T> {
    pub type StorageType = *mut T;
    pub type AlignmentType = *mut T;

    pub fn mutable_instance(storage: &mut Self::StorageType) -> *mut T {
        *storage
    }

    pub fn init_storage_using_trait<CreateTrait>(storage: &mut Self::StorageType)
    where
        CreateTrait: CreateTraitInterface<T>,
    {
        *storage = CreateTrait::create();
    }
}

pub trait ConstructTraitInterface<T> {
    fn construct(allocated_ptr: *mut std::ffi::c_void);
}

pub struct DefaultConstructTrait<T> {
    _phantom: PhantomData<T>,
}

impl<T> ConstructTraitInterface<T> for DefaultConstructTrait<T> {
    fn construct(allocated_ptr: *mut std::ffi::c_void) {
        unsafe {
            let ptr = allocated_ptr as *mut T;
            ptr.write(MaybeUninit::zeroed().assume_init());
        }
    }
}

pub trait CreateTraitInterface<T> {
    fn create() -> *mut T;
}

pub struct DefaultCreateTrait<T> {
    _phantom: PhantomData<T>,
}

impl<T> CreateTraitInterface<T> for DefaultCreateTrait<T> {
    fn create() -> *mut T {
        Box::into_raw(Box::new(MaybeUninit::zeroed().assume_init()))
    }
}

pub struct ThreadSafeInitOnceTrait;

impl ThreadSafeInitOnceTrait {
    pub fn init<F, S>(once: &Once, function: F, storage: *mut S)
    where
        F: FnOnce(*mut S),
    {
        once.call_once(|| function(storage));
    }
}

pub struct SingleThreadInitOnceTrait;

impl SingleThreadInitOnceTrait {
    pub fn init<F, S>(once: &mut Once, function: F, storage: *mut S)
    where
        F: FnOnce(*mut S),
    {
        if once.is_completed() == false {
          function(storage);
          *once = Once::new(); //Mark as done. Not really thread safe.
        }
    }
}

pub struct LazyInstanceImpl<
    T,
    AllocationTrait,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> where
    AllocationTrait: AllocationTraitInterface<T>,
    CreateTrait: CreateTraitInterface<T>,
    InitOnceTrait: InitOnceTraitInterface,
    DestroyTrait: DestroyTraitInterface<T>,
{
    pub type StorageType =
        <AllocationTrait as AllocationTraitInterface<T>>::StorageType;
    pub type AlignmentType =
        <AllocationTrait as AllocationTraitInterface<T>>::AlignmentType;

    once_: Once,
    storage_: MaybeUninit<<AllocationTrait as AllocationTraitInterface<T>>::StorageType>,
    _phantom: PhantomData<(T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait)>,
}

impl<
    T,
    AllocationTrait,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait>
where
    AllocationTrait: AllocationTraitInterface<T>,
    CreateTrait: CreateTraitInterface<T>,
    InitOnceTrait: InitOnceTraitInterface,
    DestroyTrait: DestroyTraitInterface<T>,
{
    fn init_instance(storage: *mut std::ffi::c_void) {
        let storage = storage as *mut <AllocationTrait as AllocationTraitInterface<T>>::StorageType;
        unsafe {
            AllocationTrait::init_storage_using_trait::<CreateTrait>(&mut *storage);
        }
    }

    fn init(&self) {
        let storage = &self.storage_ as *const _ as *mut std::ffi::c_void;
        InitOnceTrait::init(&self.once_, |s| Self::init_instance(s), storage);
    }

    pub fn pointer(&self) -> *mut T {
        self.init();
        let storage = &self.storage_ as *const _ as *mut <AllocationTrait as AllocationTraitInterface<T>>::StorageType;
        unsafe {
            AllocationTrait::mutable_instance(&mut *storage)
        }
    }

    pub fn get(&self) -> &T {
        self.init();
        let storage = &self.storage_ as *const _ as *mut <AllocationTrait as AllocationTraitInterface<T>>::StorageType;
        unsafe {
            &*AllocationTrait::mutable_instance(&mut *storage)
        }
    }
}

unsafe impl<
    T,
    AllocationTrait,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> Sync for LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait>
where
    AllocationTrait: AllocationTraitInterface<T>,
    CreateTrait: CreateTraitInterface<T>,
    InitOnceTrait: InitOnceTraitInterface,
    DestroyTrait: DestroyTraitInterface<T>,
    <AllocationTrait as AllocationTraitInterface<T>>::StorageType: Sync,
    <AllocationTrait as AllocationTraitInterface<T>>::AlignmentType: Sync,
{}

impl<
    T,
    AllocationTrait,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait>
where
    AllocationTrait: AllocationTraitInterface<T>,
    CreateTrait: CreateTraitInterface<T>,
    InitOnceTrait: InitOnceTraitInterface,
    DestroyTrait: DestroyTraitInterface<T>,
{
    pub const fn new() -> Self {
        LazyInstanceImpl {
            once_: Once::new(),
            storage_: MaybeUninit::uninit(),
            _phantom: PhantomData,
        }
    }
}

pub trait AllocationTraitInterface<T> {
    type StorageType;
    type AlignmentType;

    fn mutable_instance(storage: &mut Self::StorageType) -> *mut T;
    fn init_storage_using_trait<CreateTrait>(storage: &mut Self::StorageType)
    where
        CreateTrait: CreateTraitInterface<T>;
}

pub trait InitOnceTraitInterface {
    fn init<F, S>(once: &Once, function: F, storage: *mut S)
    where
        F: FnOnce(*mut S);
}

impl InitOnceTraitInterface for ThreadSafeInitOnceTrait {
    fn init<F, S>(once: &Once, function: F, storage: *mut S)
    where
        F: FnOnce(*mut S),
    {
        ThreadSafeInitOnceTrait::init(once, function, storage);
    }
}

pub trait DestroyTraitInterface<T> {
    fn destroy(instance: *mut T);
}

impl<T> DestroyTraitInterface<T> for LeakyInstanceTrait<T> {
    fn destroy(_instance: *mut T) {}
}

pub struct LazyStaticInstance<
    T,
    CreateTrait = DefaultConstructTrait<T>,
    InitOnceTrait = ThreadSafeInitOnceTrait,
    DestroyTrait = LeakyInstanceTrait<T>,
> {
    _phantom: PhantomData<(T, CreateTrait, InitOnceTrait, DestroyTrait)>,
}

impl<
    T,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> LazyStaticInstance<T, CreateTrait, InitOnceTrait, DestroyTrait>
{
    pub type type_ = LazyInstanceImpl<
        T,
        StaticallyAllocatedInstanceTrait<T>,
        CreateTrait,
        InitOnceTrait,
        DestroyTrait,
    >;
}

pub struct LazyInstance<
    T,
    CreateTrait = DefaultConstructTrait<T>,
    InitOnceTrait = ThreadSafeInitOnceTrait,
    DestroyTrait = LeakyInstanceTrait<T>,
> {
    _phantom: PhantomData<(T, CreateTrait, InitOnceTrait, DestroyTrait)>,
}

impl<
    T,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> LazyInstance<T, CreateTrait, InitOnceTrait, DestroyTrait>
{
    pub type type_ =
        LazyStaticInstance<T, CreateTrait, InitOnceTrait, DestroyTrait>::type_;
}

pub struct LazyDynamicInstance<
    T,
    CreateTrait = DefaultCreateTrait<T>,
    InitOnceTrait = ThreadSafeInitOnceTrait,
    DestroyTrait = LeakyInstanceTrait<T>,
> {
    _phantom: PhantomData<(T, CreateTrait, InitOnceTrait, DestroyTrait)>,
}

impl<
    T,
    CreateTrait,
    InitOnceTrait,
    DestroyTrait,
> LazyDynamicInstance<T, CreateTrait, InitOnceTrait, DestroyTrait>
{
    pub type type_ = LazyInstanceImpl<
        T,
        DynamicallyAllocatedInstanceTrait<T>,
        CreateTrait,
        InitOnceTrait,
        DestroyTrait,
    >;
}

pub struct LeakyObject<T> {
    storage_: MaybeUninit<T>,
}

impl<T> LeakyObject<T> {
    pub fn new<Args>(args: Args) -> Self
    where
        T: LeakyObjectConstruct<Args>,
    {
        let mut storage_ = MaybeUninit::uninit();
        T::construct(&mut storage_, args);
        LeakyObject { storage_ }
    }

    pub fn get(&self) -> &T {
        unsafe { self.storage_.assume_init_ref() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.storage_.assume_init_mut() }
    }
}

unsafe impl<T> Sync for LeakyObject<T> where T: Sync {}

pub trait LeakyObjectConstruct<Args> {
    fn construct(storage: &mut MaybeUninit<Self>, args: Args);
}

impl<T> LeakyObject<T> {
    pub const fn zeroed() -> Self {
        LeakyObject {
            storage_: MaybeUninit::zeroed(),
        }
    }
}

macro_rules! define_lazy_leaky_object_getter {
    ($T:ty, $FunctionName:ident, $($arg:expr),*) => {
        fn $FunctionName() -> *mut $T {
            static object: ::v8::base::LeakyObject<$T> =
                ::v8::base::LeakyObject::new(($($arg),*));
            object.get_mut() as *mut $T
        }
    };
}

pub(crate) use define_lazy_leaky_object_getter;

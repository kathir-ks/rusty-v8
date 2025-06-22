// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::cell::UnsafeCell;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;
    use std::sync::{Once, ONCE_INIT};

    /// Macro for static lazy instance initialization.
    #[macro_export]
    macro_rules! lazy_static_instance_initializer {
        () => {
            LazyInitializer {
                once: std::sync::ONCE_INIT,
                storage: std::mem::MaybeUninit::uninit(),
            }
        };
    }

    /// Macro for dynamic lazy instance initialization.
    #[macro_export]
    macro_rules! lazy_dynamic_instance_initializer {
        () => {
            LazyInitializer {
                once: std::sync::ONCE_INIT,
                storage: std::mem::MaybeUninit::uninit(), // TODO: check this
            }
        };
    }

    /// Default macro for lazy instance initialization (static mode).
    #[macro_export]
    macro_rules! lazy_instance_initializer {
        () => {
            $crate::base::lazy_static_instance_initializer!()
        };
    }

    /// A trait that defines how to destroy an instance.
    pub trait LeakyInstanceTrait<T> {
        /// Destroys the instance.  By default, does nothing (leaky).
        fn destroy(_instance: &mut T) {}
    }

    /// Default leaky instance trait implementation.
    impl<T> LeakyInstanceTrait<T> for () {}

    /// Traits that define how an instance is allocated and accessed.
    pub trait StaticallyAllocatedInstanceTrait<T> {
        /// The type used for storage.
        type StorageType;
        /// The type used for alignment.
        type AlignmentType;

        /// Returns a mutable reference to the instance.
        fn mutable_instance(storage: &mut Self::StorageType) -> &mut T;

        /// Initializes the storage using the provided trait.
        fn init_storage_using_trait<ConstructTrait: ConstructTrait<T>>(
            storage: &mut Self::StorageType,
        );
    }

    /// Statically allocated instance trait implementation.
    pub struct DefaultStaticallyAllocatedInstanceTrait<T>(PhantomData<T>);

    impl<T> StaticallyAllocatedInstanceTrait<T> for DefaultStaticallyAllocatedInstanceTrait<T> {
        type StorageType = MaybeUninit<T>;
        type AlignmentType = T;

        fn mutable_instance(storage: &mut Self::StorageType) -> &mut T {
            unsafe { storage.assume_init_mut() }
        }

        fn init_storage_using_trait<ConstructTrait: ConstructTrait<T>>(
            storage: &mut Self::StorageType,
        ) {
            ConstructTrait::construct(storage);
        }
    }

    /// Dynamically allocated instance trait.
    pub trait DynamicallyAllocatedInstanceTrait<T> {
        /// The type used for storage.
        type StorageType;
        /// The type used for alignment.
        type AlignmentType;

        /// Returns a mutable reference to the instance.
        fn mutable_instance(storage: &mut Self::StorageType) -> &mut T;

        /// Initializes the storage using the provided trait.
        fn init_storage_using_trait<CreateTrait: CreateTrait<T>>(storage: &mut Self::StorageType);
    }

    /// Dynamically allocated instance trait implementation.
    pub struct DefaultDynamicallyAllocatedInstanceTrait<T>(PhantomData<T>);

    impl<T> DynamicallyAllocatedInstanceTrait<T> for DefaultDynamicallyAllocatedInstanceTrait<T> {
        type StorageType = Option<Box<T>>;
        type AlignmentType = T;

        fn mutable_instance(storage: &mut Self::StorageType) -> &mut T {
            storage.as_mut().unwrap()
        }

        fn init_storage_using_trait<CreateTrait: CreateTrait<T>>(storage: &mut Self::StorageType) {
            *storage = Some(CreateTrait::create());
        }
    }

    /// Trait for constructing an instance in pre-allocated memory.
    pub trait ConstructTrait<T> {
        /// Constructs the provided object which was already allocated.
        fn construct(allocated_ptr: &mut MaybeUninit<T>);
    }

    /// Default construct trait implementation.
    pub struct DefaultConstructTrait<T>(PhantomData<T>);

    impl<T> ConstructTrait<T> for DefaultConstructTrait<T> {
        fn construct(allocated_ptr: &mut MaybeUninit<T>) {
            unsafe {
                allocated_ptr.as_mut_ptr().write(T::default());
            }
        }
    }

    /// Trait for creating an instance (dynamic allocation).
    pub trait CreateTrait<T> {
        /// Creates the instance (allocating memory).
        fn create() -> Box<T>;
    }

    /// Default create trait implementation.
    pub struct DefaultCreateTrait<T>(PhantomData<T>);

    impl<T> CreateTrait<T> for DefaultCreateTrait<T> {
        fn create() -> Box<T> {
            Box::new(T::default())
        }
    }

    /// Thread-safe initialization trait.
    pub struct ThreadSafeInitOnceTrait;

    impl ThreadSafeInitOnceTrait {
        /// Initializes the instance using a thread-safe `Once`.
        pub fn init<F, S>(once: &Once, function: F, storage: &mut S)
        where
            F: FnOnce(&mut S),
        {
            once.call_once(|| {
                function(storage);
            });
        }
    }

    /// Initialization trait for users who don't care about thread-safety.
    pub struct SingleThreadInitOnceTrait;

    impl SingleThreadInitOnceTrait {
        /// Initializes the instance without thread-safety.
        pub fn init<F, S>(once: &mut OnceType, function: F, storage: &mut S)
        where
            F: FnOnce(&mut S),
        {
            if *once == ONCE_STATE_UNINITIALIZED {
                function(storage);
                *once = ONCE_STATE_DONE;
            }
        }
    }

    /// Once state types for single thread init.
    pub type OnceType = i32;
    pub const ONCE_STATE_UNINITIALIZED: OnceType = 0;
    pub const ONCE_STATE_DONE: OnceType = 1;

    /// Struct representing the initialization state and storage.
    #[derive(Copy, Clone)]
    pub struct LazyInitializer<T> {
        pub once: Once,
        pub storage: MaybeUninit<T>,
    }

    unsafe impl<T> Sync for LazyInitializer<T> {} // Assuming T is Sync if it needs to be initialized

    /// Core implementation of the lazy instance logic.
    pub struct LazyInstanceImpl<
        T,
        AllocationTrait: StaticallyAllocatedInstanceTrait<T>,
        CreateTrait: ConstructTrait<T>,
        InitOnceTrait,
        DestroyTrait: LeakyInstanceTrait<T>,
    > {
        once_: Once,
        storage_: UnsafeCell<AllocationTrait::StorageType>,
        _phantom: PhantomData<(T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait)>,
    }

    impl<
        T,
        AllocationTrait: StaticallyAllocatedInstanceTrait<T>,
        CreateTrait: ConstructTrait<T>,
        InitOnceTrait,
        DestroyTrait: LeakyInstanceTrait<T>,
    > LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait>
    {
        const fn new() -> Self {
            LazyInstanceImpl {
                once_: ONCE_INIT,
                storage_: UnsafeCell::new(MaybeUninit::uninit()),
                _phantom: PhantomData,
            }
        }

        /// Initializes the instance if it hasn't been already.
        #[inline]
        fn init(&self) {
            ThreadSafeInitOnceTrait::init(&self.once_, |storage| {
                AllocationTrait::init_storage_using_trait::<CreateTrait>(
                    unsafe { &mut *storage.cast::<AllocationTrait::StorageType>() },
                );
            }, unsafe { &mut *self.storage_.get() });
        }

        /// Returns a mutable pointer to the contained object.
        #[inline]
        pub fn pointer(&self) -> &mut T {
            self.init();
            AllocationTrait::mutable_instance(unsafe { &mut *self.storage_.get() })
        }

        /// Returns a const reference to the contained object.
        #[inline]
        pub fn get(&self) -> &T {
            self.init();
            AllocationTrait::mutable_instance(unsafe { &mut *self.storage_.get() })
        }
    }

    impl<
        T,
        AllocationTrait: DynamicallyAllocatedInstanceTrait<T>,
        CreateTrait: CreateTrait<T>,
        InitOnceTrait,
        DestroyTrait: LeakyInstanceTrait<T>,
    > LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait>
    {
        const fn new() -> Self {
            LazyInstanceImpl {
                once_: ONCE_INIT,
                storage_: UnsafeCell::new(None),
                _phantom: PhantomData,
            }
        }

        /// Initializes the instance if it hasn't been already.
        #[inline]
        fn init(&self) {
            ThreadSafeInitOnceTrait::init(&self.once_, |storage| {
                AllocationTrait::init_storage_using_trait::<CreateTrait>(
                    unsafe { &mut *storage.cast::<AllocationTrait::StorageType>() },
                );
            }, unsafe { &mut *self.storage_.get() });
        }

        /// Returns a mutable pointer to the contained object.
        #[inline]
        pub fn pointer(&self) -> &mut T {
            self.init();
            AllocationTrait::mutable_instance(unsafe { &mut *self.storage_.get() })
        }

        /// Returns a const reference to the contained object.
        #[inline]
        pub fn get(&self) -> &T {
            self.init();
            AllocationTrait::mutable_instance(unsafe { &mut *self.storage_.get() })
        }
    }
    unsafe impl<
        T,
        AllocationTrait: DynamicallyAllocatedInstanceTrait<T>,
        CreateTrait: CreateTrait<T>,
        InitOnceTrait,
        DestroyTrait: LeakyInstanceTrait<T>,
    > Sync for LazyInstanceImpl<T, AllocationTrait, CreateTrait, InitOnceTrait, DestroyTrait> {}

    /// Lazy static instance implementation.
    pub type LazyStaticInstance<
        T,
        CreateTrait = DefaultConstructTrait<T>,
        InitOnceTrait = ThreadSafeInitOnceTrait,
        DestroyTrait = (),
    > = LazyInstanceImpl<
        T,
        DefaultStaticallyAllocatedInstanceTrait<T>,
        CreateTrait,
        InitOnceTrait,
        DestroyTrait,
    >;

    /// Lazy instance type alias (defaults to static).
    pub type LazyInstance<
        T,
        CreateTrait = DefaultConstructTrait<T>,
        InitOnceTrait = ThreadSafeInitOnceTrait,
        DestroyTrait = (),
    > = LazyStaticInstance<T, CreateTrait, InitOnceTrait, DestroyTrait>;

    /// Lazy dynamic instance implementation.
    pub type LazyDynamicInstance<
        T,
        CreateTrait = DefaultCreateTrait<T>,
        InitOnceTrait = ThreadSafeInitOnceTrait,
        DestroyTrait = (),
    > = LazyInstanceImpl<
        T,
        DefaultDynamicallyAllocatedInstanceTrait<T>,
        CreateTrait,
        InitOnceTrait,
        DestroyTrait,
    >;

    /// Wraps an object of type T, which is initialized in the constructor but never destructed.
    /// Thus LeakyObject<T> is trivially destructible and can be used in static (lazily initialized) variables.
    pub struct LeakyObject<T> {
        storage_: MaybeUninit<T>,
    }

    impl<T> LeakyObject<T> {
        /// Constructs a new LeakyObject, initializing the contained value.
        #[inline]
        pub fn new<Args>(args: Args) -> Self
        where
            T: LeakyConstruct<Args>,
        {
            LeakyObject {
                storage_: T::leaky_construct(args),
            }
        }

        /// Returns a pointer to the contained object.
        #[inline]
        pub fn get(&self) -> &mut T {
            unsafe { self.storage_.assume_init_mut() }
        }
    }

    unsafe impl<T> Sync for LeakyObject<T> {}

    /// Trait for types that can be constructed in-place in a LeakyObject.
    pub trait LeakyConstruct<Args> {
        /// Constructs the object in-place and returns a `MaybeUninit` containing it.
        fn leaky_construct(args: Args) -> MaybeUninit<Self>;
    }

    impl<T, Args, F> LeakyConstruct<Args> for T
    where
        F: FnOnce(Args) -> T,
    {
        default fn leaky_construct(_args: Args) -> MaybeUninit<Self> {
            panic!("Type must implement LeakyConstruct or provide a suitable `new` function.");
        }
    }

    impl<T: Default> LeakyConstruct<()> for T {
        fn leaky_construct(_args: ()) -> MaybeUninit<Self> {
            MaybeUninit::new(T::default())
        }
    }

    /// Macro to define a function that returns a pointer to a lazily initialized leaky object.
    #[macro_export]
    macro_rules! define_lazy_leaky_object_getter {
        ($type:ty, $function_name:ident, $($arg:expr),*) => {
            fn $function_name() -> &'static mut $type {
                static OBJECT: $crate::base::LeakyObject<$type> =
                    $crate::base::LeakyObject::new(( $($arg),* ));
                OBJECT.get()
            }
        };
    }
}
// Converted from V8 C++ source files:
// Header: mutex.h
// Implementation: mutex.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {
pub mod mutex {

use std::sync::{Mutex as StdMutex, MutexGuard as StdMutexGuard, TryLockError, PoisonError};
use std::sync::atomic::{AtomicI32, Ordering};
use crate::base::lazy_instance::{LazyStaticInstance, DefaultConstructTrait, ThreadSafeInitOnceTrait, LAZY_STATIC_INSTANCE_INITIALIZER};
use crate::base::logging::DCHECK_EQ;
use crate::base::platform::platform::OS;
use crate::base::logging::DCHECK_LT;
use crate::base::logging::DCHECK_NOT_NULL;
use std::optional::Optional;

pub struct ConditionVariable {}

pub struct Mutex {
    native_handle_: StdMutex<()>,
    #[cfg(debug_assertions)]
    level_: i32,
}

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            native_handle_: StdMutex::new(()),
            #[cfg(debug_assertions)]
            level_: 0,
        }
    }

    pub fn lock(&self) {
        #[cfg(debug_assertions)]
        self.assert_unheld_and_mark();
        self.native_handle_.lock().unwrap();
    }

    pub fn unlock(&self) {
        #[cfg(debug_assertions)]
        self.assert_held_and_unmark();
        self.native_handle_.unlock().unwrap();
    }

    pub fn try_lock(&self) -> bool {
        match self.native_handle_.try_lock() {
            Ok(_guard) => {
                #[cfg(debug_assertions)]
                self.assert_unheld_and_mark();
                true
            }
            Err(_e) => false,
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn assert_held(&self) const {
        DCHECK_EQ!(1, self.level_);
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn assert_held_and_unmark(&self) {
        DCHECK_EQ!(1, self.level_);
        self.level_ -= 1;
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn assert_unheld_and_mark(&self) {
        DCHECK_EQ!(0, self.level_);
        self.level_ += 1;
    }
}

pub type LazyMutex = LazyStaticInstance<Mutex, DefaultConstructTrait<Mutex>, ThreadSafeInitOnceTrait>;

pub struct RecursiveMutex {
    thread_id_: AtomicI32,
    level_: i32,
    mutex_: Mutex,
}

impl RecursiveMutex {
    pub fn new() -> Self {
        RecursiveMutex {
            thread_id_: AtomicI32::new(0),
            level_: 0,
            mutex_: Mutex::new(),
        }
    }

    pub fn lock(&self) {
        let own_id = OS::get_current_thread_id() as i32;
        if self.thread_id_.load(Ordering::Relaxed) == own_id {
            self.level_ += 1;
            return;
        }
        self.mutex_.lock();
        //DCHECK_EQ!(0, self.level_); //TODO: Add debug assertion
        self.thread_id_.store(own_id, Ordering::Relaxed);
        self.level_ = 1;
    }

    pub fn unlock(&self) {
        #[cfg(debug_assertions)]
        {
            let own_id = OS::get_current_thread_id() as i32;
           // CHECK_EQ!(self.thread_id_.load(Ordering::Relaxed), own_id); //TODO: Add debug assertion
        }
        self.level_ -= 1;
        if self.level_ == 0 {
            self.thread_id_.store(0, Ordering::Relaxed);
            self.mutex_.unlock();
        }
    }

    pub fn try_lock(&self) -> bool {
        let own_id = OS::get_current_thread_id() as i32;
        if self.thread_id_.load(Ordering::Relaxed) == own_id {
            self.level_ += 1;
            return true;
        }
        if self.mutex_.try_lock() {
            //DCHECK_EQ!(0, self.level_); //TODO: Add debug assertion
            self.thread_id_.store(own_id, Ordering::Relaxed);
            self.level_ = 1;
            return true;
        }
        return false;
    }

    #[inline]
    pub fn assert_held(&self) const {
        DCHECK_LT!(0, self.level_);
    }
}

impl Drop for RecursiveMutex {
    fn drop(&mut self) {
        //DCHECK_EQ!(0, self.level_); //TODO: Add debug assertion
    }
}
pub type LazyRecursiveMutex = LazyStaticInstance<RecursiveMutex, DefaultConstructTrait<RecursiveMutex>, ThreadSafeInitOnceTrait>;

pub struct LockGuard<'a, T> {
    mutex_: Option<&'a T>,
}

impl<'a, T> LockGuard<'a, T> {
    pub fn new(mutex: &'a T) -> Self
    where T: Lockable {
        mutex.lock();
        LockGuard {
            mutex_: Some(mutex),
        }
    }
}

impl<'a, T> Drop for LockGuard<'a, T>
where T: Lockable {
    fn drop(&mut self) {
        if let Some(mutex) = self.mutex_ {
            mutex.unlock();
        }
    }
}

pub trait Lockable {
    fn lock(&self);
    fn unlock(&self);
}

impl Lockable for Mutex {
    fn lock(&self) {
        Mutex::lock(self);
    }
    fn unlock(&self) {
        Mutex::unlock(self);
    }
}

impl Lockable for RecursiveMutex {
    fn lock(&self) {
        RecursiveMutex::lock(self);
    }
    fn unlock(&self) {
        RecursiveMutex::unlock(self);
    }
}

pub type MutexGuard<'a> = LockGuard<'a, Mutex>;
pub type RecursiveMutexGuard<'a> = LockGuard<'a, RecursiveMutex>;

pub struct MutexGuardIf<'a> {
    mutex_: Option<MutexGuard<'a>>,
}

impl<'a> MutexGuardIf<'a> {
    pub fn new(mutex: &'a Mutex, enable_mutex: bool) -> Self {
        if enable_mutex {
            MutexGuardIf {
                mutex_: Some(MutexGuard::new(mutex)),
            }
        } else {
            MutexGuardIf {
                mutex_: None,
            }
        }
    }
}

}
}
}
pub mod base {
pub mod lazy_instance {
use std::sync::{Once, Mutex};

pub struct LazyStaticInstance<T, C, I> {
    instance: Mutex<Option<T>>,
    once: Once,
    construct_trait: C,
    init_once_trait: I,
}

impl<T, C: DefaultConstructTrait<T>, I: ThreadSafeInitOnceTrait> LazyStaticInstance<T, C, I> {
    pub const fn new(construct_trait: C, init_once_trait: I) -> Self {
        LazyStaticInstance {
            instance: Mutex::new(None),
            once: Once::new(),
            construct_trait,
            init_once_trait,
        }
    }

    pub fn pointer(&self) -> &T {
        self.once.call_once(|| {
            let mut instance = self.instance.lock().unwrap();
            *instance = Some(self.construct_trait.construct());
        });
        let instance = self.instance.lock().unwrap();
        instance.as_ref().unwrap()
    }
}

unsafe impl<T, C, I> Sync for LazyStaticInstance<T, C, I> {}

pub trait DefaultConstructTrait<T> {
    fn construct(&self) -> T;
}

pub struct DefaultConstructTraitImpl<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DefaultConstructTraitImpl<T> {
    pub const fn new() -> Self {
        DefaultConstructTraitImpl {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Default> DefaultConstructTrait<T> for DefaultConstructTraitImpl<T> {
    fn construct(&self) -> T {
        T::default()
    }
}

pub type DefaultConstructTrait<T> = DefaultConstructTraitImpl<T>;

impl<T> Default for DefaultConstructTraitImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ThreadSafeInitOnceTrait {}

pub struct ThreadSafeInitOnceTraitImpl {}

impl ThreadSafeInitOnceTrait for ThreadSafeInitOnceTraitImpl {}

impl Default for ThreadSafeInitOnceTraitImpl {
    fn default() -> Self {
        Self {}
    }
}

pub type ThreadSafeInitOnceTrait = ThreadSafeInitOnceTraitImpl;

pub const LAZY_STATIC_INSTANCE_INITIALIZER: () = ();
}
}
pub mod base {
pub mod logging {
    macro_rules! CHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!("CHECK_EQ! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

    macro_rules! DCHECK_EQ {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left != $right {
                    panic!("DCHECK_EQ! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK_NE {
        ($left:expr, $right:expr) => {
            if $left == $right {
                panic!("CHECK_NE! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

    macro_rules! DCHECK_NE {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left == $right {
                    panic!("DCHECK_NE! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK_LT {
        ($left:expr, $right:expr) => {
            if $left >= $right {
                panic!("CHECK_LT! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

     macro_rules! DCHECK_LT {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left >= $right {
                    panic!("DCHECK_LT! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK_GT {
        ($left:expr, $right:expr) => {
            if $left <= $right {
                panic!("CHECK_GT! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

    macro_rules! DCHECK_GT {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left <= $right {
                    panic!("DCHECK_GT! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK_LE {
        ($left:expr, $right:expr) => {
            if $left > $right {
                panic!("CHECK_LE! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

    macro_rules! DCHECK_LE {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left > $right {
                    panic!("DCHECK_LE! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK_GE {
        ($left:expr, $right:expr) => {
            if $left < $right {
                panic!("CHECK_GE! failed: left = {}, right = {}", $left, $right);
            }
        };
    }

    macro_rules! DCHECK_GE {
        ($left:expr, $right:expr) => {
            if cfg!(debug_assertions) {
                if $left < $right {
                    panic!("DCHECK_GE! failed: left = {}, right = {}", $left, $right);
                }
            }
        };
    }

    macro_rules! CHECK {
        ($cond:expr) => {
            if !$cond {
                panic!("CHECK! failed: {}", stringify!($cond));
            }
        };
    }

    macro_rules! DCHECK {
        ($cond:expr) => {
            if cfg!(debug_assertions) {
                if !$cond {
                    panic!("DCHECK! failed: {}", stringify!($cond));
                }
            }
        };
    }

    macro_rules! CHECK_NOT_NULL {
        ($ptr:expr) => {
            if $ptr.is_null() {
                panic!("CHECK_NOT_NULL! failed: pointer is null");
            }
        };
    }

    macro_rules! DCHECK_NOT_NULL {
        ($ptr:expr) => {
            if cfg!(debug_assertions) {
                if $ptr.is_null() {
                    panic!("DCHECK_NOT_NULL! failed: pointer is null");
                }
            }
        };
    }
}
}
pub mod base {
pub mod platform {
pub mod platform {
    pub struct OS {}
    impl OS {
        pub fn get_current_thread_id() -> usize {
            0
        }
    }
}
}
}

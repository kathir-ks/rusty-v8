// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicPtr, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, Ordering,
};
use std::{
    mem,
    ops::{Add, Sub},
};

macro_rules! use_ {
    ($x:ident) => {
        let _ = $x;
    };
}

mod base {
    pub type Atomic8 = std::sync::atomic::AtomicI8;
    pub type Atomic16 = std::sync::atomic::AtomicI16;
    pub type Atomic32 = std::sync::atomic::AtomicI32;

    #[cfg(target_arch = "x86_64")]
    pub type Atomic64 = std::sync::atomic::AtomicI64;

    #[cfg(target_pointer_width = "64")]
    pub type AtomicWord = std::sync::atomic::AtomicI64;
    #[cfg(target_pointer_width = "32")]
    pub type AtomicWord = std::sync::atomic::AtomicI32;

    #[derive(Clone, Copy)]
    pub struct StrongAlias<T, U>(U, std::marker::PhantomData<T>);

    impl<T, U> StrongAlias<T, U> {
        pub fn new(value: U) -> Self {
            StrongAlias(value, std::marker::PhantomData)
        }

        pub fn value(&self) -> U {
            self.0
        }
    }

    // Atomic Operations
    pub fn acquire_load<T>(atomic: &std::sync::atomic::AtomicPtr<T>) -> *mut T {
        atomic.load(Ordering::Acquire)
    }

    pub fn release_store<T>(atomic: &std::sync::atomic::AtomicPtr<T>, value: *mut T) {
        atomic.store(value, Ordering::Release)
    }

    pub fn relaxed_load<T>(atomic: &std::sync::atomic::AtomicPtr<T>) -> *mut T {
        atomic.load(Ordering::Relaxed)
    }

    pub fn seqcst_load<T>(atomic: &std::sync::atomic::AtomicPtr<T>) -> *mut T {
        atomic.load(Ordering::SeqCst)
    }

    pub fn seqcst_store<T>(atomic: &std::sync::atomic::AtomicPtr<T>, value: *mut T) {
        atomic.store(value, Ordering::SeqCst)
    }

    pub fn relaxed_store<T>(atomic: &std::sync::atomic::AtomicPtr<T>, value: *mut T) {
        atomic.store(value, Ordering::Relaxed)
    }

    pub fn seqcst_atomic_exchange<T>(
        atomic: &std::sync::atomic::AtomicPtr<T>,
        new_value: *mut T,
    ) -> *mut T {
        atomic.swap(new_value, Ordering::SeqCst)
    }

    pub fn release_compare_and_swap<T>(
        atomic: &std::sync::atomic::AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
    ) -> *mut T {
        atomic.compare_and_swap(current, new, Ordering::Release)
    }

    pub fn relaxed_compare_and_swap<T>(
        atomic: &std::sync::atomic::AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
    ) -> *mut T {
        atomic.compare_and_swap(current, new, Ordering::Relaxed)
    }

    pub fn acquire_release_compare_and_swap<T>(
        atomic: &std::sync::atomic::AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
    ) -> *mut T {
        atomic.compare_and_swap(current, new, Ordering::AcqRel)
    }

    pub fn seqcst_compare_and_swap<T>(
        atomic: &std::sync::atomic::AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
    ) -> *mut T {
        atomic.compare_and_swap(current, new, Ordering::SeqCst)
    }

    pub fn acquire_load_i8(atomic: &AtomicI8) -> i8 {
        atomic.load(Ordering::Acquire)
    }

    pub fn release_store_i8(atomic: &AtomicI8, value: i8) {
        atomic.store(value, Ordering::Release)
    }

    pub fn relaxed_load_i8(atomic: &AtomicI8) -> i8 {
        atomic.load(Ordering::Relaxed)
    }

    pub fn seqcst_load_i8(atomic: &AtomicI8) -> i8 {
        atomic.load(Ordering::SeqCst)
    }

    pub fn seqcst_store_i8(atomic: &AtomicI8, value: i8) {
        atomic.store(value, Ordering::SeqCst)
    }

    pub fn relaxed_store_i8(atomic: &AtomicI8, value: i8) {
        atomic.store(value, Ordering::Relaxed)
    }

    pub fn seqcst_atomic_exchange_i8(atomic: &AtomicI8, new_value: i8) -> i8 {
        atomic.swap(new_value, Ordering::SeqCst)
    }

    pub fn release_compare_and_swap_i8(atomic: &AtomicI8, current: i8, new: i8) -> i8 {
        atomic.compare_and_swap(current, new, Ordering::Release)
    }

    pub fn relaxed_compare_and_swap_i8(atomic: &AtomicI8, current: i8, new: i8) -> i8 {
        atomic.compare_and_swap(current, new, Ordering::Relaxed)
    }

    pub fn acquire_release_compare_and_swap_i8(atomic: &AtomicI8, current: i8, new: i8) -> i8 {
        atomic.compare_and_swap(current, new, Ordering::AcqRel)
    }

    pub fn seqcst_compare_and_swap_i8(atomic: &AtomicI8, current: i8, new: i8) -> i8 {
        atomic.compare_and_swap(current, new, Ordering::SeqCst)
    }

    pub fn acquire_load_i16(atomic: &AtomicI16) -> i16 {
        atomic.load(Ordering::Acquire)
    }

    pub fn release_store_i16(atomic: &AtomicI16, value: i16) {
        atomic.store(value, Ordering::Release)
    }

    pub fn relaxed_load_i16(atomic: &AtomicI16) -> i16 {
        atomic.load(Ordering::Relaxed)
    }

    pub fn seqcst_load_i16(atomic: &AtomicI16) -> i16 {
        atomic.load(Ordering::SeqCst)
    }

    pub fn seqcst_store_i16(atomic: &AtomicI16, value: i16) {
        atomic.store(value, Ordering::SeqCst)
    }

    pub fn relaxed_store_i16(atomic: &AtomicI16, value: i16) {
        atomic.store(value, Ordering::Relaxed)
    }

    pub fn seqcst_atomic_exchange_i16(atomic: &AtomicI16, new_value: i16) -> i16 {
        atomic.swap(new_value, Ordering::SeqCst)
    }

    pub fn release_compare_and_swap_i16(atomic: &AtomicI16, current: i16, new: i16) -> i16 {
        atomic.compare_and_swap(current, new, Ordering::Release)
    }

    pub fn relaxed_compare_and_swap_i16(atomic: &AtomicI16, current: i16, new: i16) -> i16 {
        atomic.compare_and_swap(current, new, Ordering::Relaxed)
    }

    pub fn acquire_release_compare_and_swap_i16(atomic: &AtomicI16, current: i16, new: i16) -> i16 {
        atomic.compare_and_swap(current, new, Ordering::AcqRel)
    }

    pub fn seqcst_compare_and_swap_i16(atomic: &AtomicI16, current: i16, new: i16) -> i16 {
        atomic.compare_and_swap(current, new, Ordering::SeqCst)
    }

    pub fn acquire_load_i32(atomic: &AtomicI32) -> i32 {
        atomic.load(Ordering::Acquire)
    }

    pub fn release_store_i32(atomic: &AtomicI32, value: i32) {
        atomic.store(value, Ordering::Release)
    }

    pub fn relaxed_load_i32(atomic: &AtomicI32) -> i32 {
        atomic.load(Ordering::Relaxed)
    }

    pub fn seqcst_load_i32(atomic: &AtomicI32) -> i32 {
        atomic.load(Ordering::SeqCst)
    }

    pub fn seqcst_store_i32(atomic: &AtomicI32, value: i32) {
        atomic.store(value, Ordering::SeqCst)
    }

    pub fn relaxed_store_i32(atomic: &AtomicI32, value: i32) {
        atomic.store(value, Ordering::Relaxed)
    }

    pub fn seqcst_atomic_exchange_i32(atomic: &AtomicI32, new_value: i32) -> i32 {
        atomic.swap(new_value, Ordering::SeqCst)
    }

    pub fn release_compare_and_swap_i32(atomic: &AtomicI32, current: i32, new: i32) -> i32 {
        atomic.compare_and_swap(current, new, Ordering::Release)
    }

    pub fn relaxed_compare_and_swap_i32(atomic: &AtomicI32, current: i32, new: i32) -> i32 {
        atomic.compare_and_swap(current, new, Ordering::Relaxed)
    }

    pub fn acquire_release_compare_and_swap_i32(atomic: &AtomicI32, current: i32, new: i32) -> i32 {
        atomic.compare_and_swap(current, new, Ordering::AcqRel)
    }

    pub fn seqcst_compare_and_swap_i32(atomic: &AtomicI32, current: i32, new: i32) -> i32 {
        atomic.compare_and_swap(current, new, Ordering::SeqCst)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn acquire_load_i64(atomic: &AtomicI64) -> i64 {
        atomic.load(Ordering::Acquire)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn release_store_i64(atomic: &AtomicI64, value: i64) {
        atomic.store(value, Ordering::Release)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn relaxed_load_i64(atomic: &AtomicI64) -> i64 {
        atomic.load(Ordering::Relaxed)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn seqcst_load_i64(atomic: &AtomicI64) -> i64 {
        atomic.load(Ordering::SeqCst)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn seqcst_store_i64(atomic: &AtomicI64, value: i64) {
        atomic.store(value, Ordering::SeqCst)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn relaxed_store_i64(atomic: &AtomicI64, value: i64) {
        atomic.store(value, Ordering::Relaxed)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn seqcst_atomic_exchange_i64(atomic: &AtomicI64, new_value: i64) -> i64 {
        atomic.swap(new_value, Ordering::SeqCst)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn release_compare_and_swap_i64(atomic: &AtomicI64, current: i64, new: i64) -> i64 {
        atomic.compare_and_swap(current, new, Ordering::Release)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn relaxed_compare_and_swap_i64(atomic: &AtomicI64, current: i64, new: i64) -> i64 {
        atomic.compare_and_swap(current, new, Ordering::Relaxed)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn acquire_release_compare_and_swap_i64(atomic: &AtomicI64, current: i64, new: i64) -> i64 {
        atomic.compare_and_swap(current, new, Ordering::AcqRel)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn seqcst_compare_and_swap_i64(atomic: &AtomicI64, current: i64, new: i64) -> i64 {
        atomic.compare_and_swap(current, new, Ordering::SeqCst)
    }
}

/// Deprecated. Use `std::sync::atomic::Atomic<T>` for new code.
/// Flag using `T` atomically. Also accepts `void*` as `T`.
pub struct AtomicValue<T> {
    value: base::AtomicWord,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> AtomicValue<T> {
    /// Creates a new `AtomicValue` with a zero initial value.
    pub fn new() -> Self {
        AtomicValue {
            value: base::AtomicWord::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `AtomicValue` with the given initial value.
    pub fn with_value(initial: T) -> Self
    where
        T: Into<isize>,
    {
        AtomicValue {
            value: base::AtomicWord::new(initial.into()),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Gets the current value.
    pub fn value(&self) -> T
    where
        isize: Into<T>,
    {
        base::acquire_load_i32(&self.value).into()
    }

    /// Sets the value.
    pub fn set_value(&self, new_value: T)
    where
        T: Into<isize>,
    {
        base::release_store_i32(&self.value, new_value.into());
    }
}

impl<T> Default for AtomicValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AtomicValue<*mut T> {
    pub fn with_ptr(initial: *mut T) -> Self {
        AtomicValue {
            value: base::AtomicWord::new(initial as isize),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> *mut T {
        base::acquire_load(&std::sync::atomic::AtomicPtr::new(self.value.load(Ordering::Relaxed) as *mut T))
    }

    pub fn set_value(&self, new_value: *mut T) {
        base::release_store(&std::sync::atomic::AtomicPtr::new(self.value.load(Ordering::Relaxed) as *mut T), new_value);
    }
}

/// Provides atomic operations for a values stored at some address.
pub struct AsAtomicImpl<TAtomicStorageType> {
    _phantom: std::marker::PhantomData<TAtomicStorageType>,
}

impl<TAtomicStorageType> AsAtomicImpl<TAtomicStorageType> {
    pub type AtomicStorageType = TAtomicStorageType;

    pub fn seqcst_load<T>(addr: *mut T) -> T
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &*(addr as *mut TAtomicStorageType) };
        base::seqcst_load_i32(atomic_ptr as *const TAtomicStorageType as *const base::Atomic32 as *mut base::Atomic32).into()
    }

    pub fn acquire_load<T>(addr: *mut T) -> T
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &*(addr as *mut TAtomicStorageType) };
        base::acquire_load_i32(atomic_ptr as *const TAtomicStorageType as *const base::Atomic32 as *mut base::Atomic32).into()
    }

    pub fn relaxed_load<T>(addr: *mut T) -> T
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &*(addr as *mut TAtomicStorageType) };
        base::relaxed_load_i32(atomic_ptr as *const TAtomicStorageType as *const base::Atomic32 as *mut base::Atomic32).into()
    }

    pub fn seqcst_store<T>(addr: *mut T, new_value: T)
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::seqcst_store_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, new_value.into())
    }

    pub fn release_store<T>(addr: *mut T, new_value: T)
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::release_store_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, new_value.into())
    }

    pub fn relaxed_store<T>(addr: *mut T, new_value: T)
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::relaxed_store_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, new_value.into())
    }

    pub fn seqcst_swap<T>(addr: *mut T, new_value: T) -> T
    where
        T: Copy,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::seqcst_atomic_exchange_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, new_value.into()).into()
    }

    pub fn release_compare_and_swap<T>(addr: *mut T, old_value: T, new_value: T) -> T
    where
        T: Copy + PartialEq,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::release_compare_and_swap_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, old_value.into(), new_value.into()).into()
    }

    pub fn relaxed_compare_and_swap<T>(addr: *mut T, old_value: T, new_value: T) -> T
    where
        T: Copy + PartialEq,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::relaxed_compare_and_swap_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, old_value.into(), new_value.into()).into()
    }

    pub fn acquire_release_compare_and_swap<T>(addr: *mut T, old_value: T, new_value: T) -> T
    where
        T: Copy + PartialEq,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::acquire_release_compare_and_swap_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, old_value.into(), new_value.into()).into()
    }

    pub fn seqcst_compare_and_swap<T>(addr: *mut T, old_value: T, new_value: T) -> T
    where
        T: Copy + PartialEq,
        TAtomicStorageType: From<T> + Into<T>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let atomic_ptr = unsafe { &mut *(addr as *mut TAtomicStorageType) };
        base::seqcst_compare_and_swap_i32(atomic_ptr as *mut TAtomicStorageType as *mut base::Atomic32, old_value.into(), new_value.into()).into()
    }

    pub fn release_set_bits<T>(addr: *mut T, bits: T, mask: T) -> bool
    where
        T: Copy
            + PartialEq
            + std::ops::BitAnd<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::BitOr<Output = T>
            + From<i32>
            + Into<i32>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let zero: T = 0.into();
        debug_assert_eq!(bits & !mask, zero);

        let mut old_value = Self::relaxed_load(addr);
        let mut old_value_before_cas;
        loop {
            if (old_value & mask) == bits {
                return false;
            }
            let new_value = (old_value & !mask) | bits;
            old_value_before_cas = old_value;
            old_value = Self::release_compare_and_swap(addr, old_value, new_value);
            if old_value == old_value_before_cas {
                return true;
            }
        }
    }

    pub fn relaxed_set_bits<T>(addr: *mut T, bits: T, mask: T) -> bool
    where
        T: Copy
            + PartialEq
            + std::ops::BitAnd<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::BitOr<Output = T>
            + From<i32>
            + Into<i32>,
    {
        assert!(mem::size_of::<T>() <= mem::size_of::<TAtomicStorageType>());
        let zero: T = 0.into();
        debug_assert_eq!(bits & !mask, zero);

        let mut old_value = Self::relaxed_load(addr);
        let mut old_value_before_cas;
        loop {
            if (old_value & mask) == bits {
                return false;
            }
            let new_value = (old_value & !mask) | bits;
            old_value_before_cas = old_value;
            old_value = Self::relaxed_compare_and_swap(addr, old_value, new_value);
            if old_value == old_value_before_cas {
                return true;
            }
        }
    }
}

pub type AsAtomic8 = AsAtomicImpl<base::Atomic8>;
pub type AsAtomic16 = AsAtomicImpl<base::Atomic16>;
pub type AsAtomic32 = AsAtomicImpl<base::Atomic32>;
pub type AsAtomicWord = AsAtomicImpl<base::AtomicWord>;

pub struct AtomicTypeFromByteWidth<const Width: usize>;

impl AtomicTypeFromByteWidth<1> {
    pub type type_ = base::Atomic8;
}

impl AtomicTypeFromByteWidth<2> {
    pub type type_ = base::Atomic16;
}

impl AtomicTypeFromByteWidth<4> {
    pub type type_ = base::Atomic32;
}

#[cfg(target_arch = "x86_64")]
impl AtomicTypeFromByteWidth<8> {
    pub type type_ = base::Atomic64;
}

/// This is similar to `AsAtomicWord` but it explicitly deletes functionality
/// provided atomic access to bit representation of stored values.
pub struct AsAtomicPointerImpl<TAtomicStorageType> {
    _phantom: std::marker::PhantomData<TAtomicStorageType>,
}

impl<TAtomicStorageType> AsAtomicPointerImpl<TAtomicStorageType> {
    // pub fn set_bits<T>(addr: *mut T, bits: T, mask: T) -> bool {
    //     panic!("This function is deleted");
    // }
}

pub type AsAtomicPointer = AsAtomicPointerImpl<base::AtomicWord>;

pub fn checked_increment<T>(number: &std::sync::atomic::AtomicUsize, amount: usize, order: Ordering)
where
    T: Copy + Add<Output = T>,
{
    let old = number.fetch_add(amount, order);
    debug_assert!(old.wrapping_add(amount) >= old);
    use_(old);
}

pub fn checked_decrement<T>(number: &std::sync::atomic::AtomicUsize, amount: usize, order: Ordering)
where
    T: Copy + Sub<Output = T>,
{
    let old = number.fetch_sub(amount, order);
    debug_assert!(old >= amount);
    use_(old);
}

pub unsafe fn as_atomic_ptr<'a, T>(t: *mut T) -> &'a std::sync::atomic::AtomicPtr<T> {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<std::sync::atomic::AtomicPtr<T>>());
    assert!(mem::align_of::<T>() >= mem::align_of::<std::sync::atomic::AtomicPtr<T>>());
    &*(t as *mut std::sync::atomic::AtomicPtr<T>)
}

pub unsafe fn as_atomic_ptr_const<'a, T>(t: *const T) -> &'a std::sync::atomic::AtomicPtr<T> {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<std::sync::atomic::AtomicPtr<T>>());
    assert!(mem::align_of::<T>() >= mem::align_of::<std::sync::atomic::AtomicPtr<T>>());
    &*(t as *const std::sync::atomic::AtomicPtr<T>)
}
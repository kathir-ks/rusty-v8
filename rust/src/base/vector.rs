// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        algorithm::copy,
        array::from_fn,
        cmp::{max, min},
        convert::Infallible,
        fmt::{self, Debug, Display},
        hash::{Hash, Hasher},
        iter::{self, Extend, FromIterator, IntoIterator, Iterator},
        marker::PhantomData,
        mem::{self, size_of},
        num::*,
        ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Range, RangeBounds},
        ptr::{self, NonNull},
        slice::{self, SliceIndex},
        vec,
    };

    macro_rules! DCHECK {
        ($condition:expr) => {
            if !$condition {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        };
    }

    macro_rules! DCHECK_LE {
        ($left:expr, $right:expr) => {
            if $left > $right {
                panic!(
                    "DCHECK_LE failed: {} <= {}",
                    stringify!($left),
                    stringify!($right)
                );
            }
        };
    }

    macro_rules! DCHECK_LT {
        ($left:expr, $right:expr) => {
            if $left >= $right {
                panic!(
                    "DCHECK_LT failed: {} < {}",
                    stringify!($left),
                    stringify!($right)
                );
            }
        };
    }

    macro_rules! CHECK_GE {
        ($left:expr, $right:expr) => {
            if $left < $right {
                panic!(
                    "CHECK_GE failed: {} >= {}",
                    stringify!($left),
                    stringify!($right)
                );
            }
        };
    }

    macro_rules! CHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!(
                    "CHECK_EQ failed: {} == {}",
                    stringify!($left),
                    stringify!($right)
                );
            }
        };
    }

    macro_rules! DCHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!(
                    "DCHECK_EQ failed: {} == {}",
                    stringify!($left),
                    stringify!($right)
                );
            }
        };
    }

    macro_rules! DISALLOW_IMPLICIT_CONSTRUCTORS {
        ($type_name:ident) => {
            impl $type_name {
                fn new() -> Self {
                    panic!("Implicit constructors are disallowed for {}", stringify!($type_name));
                }
            }
        };
    }

    #[inline]
    pub fn hash_range<T: Hash>(start: *const T, end: *const T) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut current = start;
        while current != end {
            unsafe {
                (*current).hash(&mut hasher);
                current = current.add(1);
            }
        }
        hasher.finish()
    }

    /// A simple vector-like structure with a raw pointer to the data.
    #[derive(Debug, Copy, Clone)]
    pub struct Vector<T> {
        start_: *mut T,
        length_: usize,
    }

    impl<T> Vector<T> {
        pub type ValueType = T;
        pub type Iterator<'a> = std::slice::Iter<'a, T>;
        pub type ConstIterator<'a> = std::slice::Iter<'a, T>;

        /// Creates an empty `Vector`.
        pub const fn new() -> Self {
            Vector {
                start_: ptr::null_mut(),
                length_: 0,
            }
        }

        /// Creates a `Vector` from a raw pointer and a length.
        pub const fn from_raw_parts(data: *mut T, length: usize) -> Self {
            DCHECK!(length == 0 || !data.is_null());
            Vector {
                start_: data,
                length_: length,
            }
        }

        /// Creates a `Vector` by allocating memory on the heap.
        pub fn new_with_size(length: usize) -> Self {
            let data = if length > 0 {
                let layout = std::alloc::Layout::array::<T>(length).unwrap();
                unsafe {
                    let ptr = std::alloc::alloc(layout) as *mut T;
                    if ptr.is_null() {
                        std::alloc::handle_alloc_error(layout);
                    }
                    ptr
                }
            } else {
                ptr::null_mut()
            };

            Vector {
                start_: data,
                length_: length,
            }
        }

        /// Returns a sub-vector using the same backing storage.
        pub fn sub_vector(&self, from: usize, to: usize) -> Self {
            DCHECK_LE!(from, to);
            DCHECK_LE!(to, self.length_);
            Vector {
                start_: unsafe { self.start_.add(from) },
                length_: to - from,
            }
        }

        pub fn sub_vector_from(&self, from: usize) -> Self {
            self.sub_vector(from, self.length_)
        }

        pub fn overwrite_with<U>(&mut self, other: Vector<U>)
        where
            U: Copy,
        {
            DCHECK_EQ!(self.size(), other.size());
            unsafe {
                std::ptr::copy_nonoverlapping(
                    other.start_ as *const U,
                    self.start_ as *mut T,
                    self.size() * std::mem::size_of::<U>(),
                );
            }
        }

        pub fn overwrite_with_array<U, const N: usize>(&mut self, other: &[U; N])
        where
            U: Copy,
        {
            DCHECK_EQ!(self.size(), other.len());
            unsafe {
                std::ptr::copy_nonoverlapping(
                    other.as_ptr() as *const U,
                    self.start_ as *mut T,
                    self.size() * std::mem::size_of::<U>(),
                );
            }
        }

        /// Returns the length of the vector as an `i32`.
        pub fn length(&self) -> i32 {
            CHECK_GE!(i32::MAX as usize, self.length_);
            self.length_ as i32
        }

        /// Returns the length of the vector as a `usize`.
        pub const fn size(&self) -> usize {
            self.length_
        }

        /// Returns whether or not the vector is empty.
        pub const fn is_empty(&self) -> bool {
            self.length_ == 0
        }

        /// Access individual vector elements - checks bounds in debug mode.
        pub fn get(&self, index: usize) -> &T {
            DCHECK_LT!(index, self.length_);
            unsafe { &*self.start_.add(index) }
        }

        pub fn get_mut(&mut self, index: usize) -> &mut T {
            DCHECK_LT!(index, self.length_);
            unsafe { &mut *self.start_.add(index) }
        }

        pub fn at(&self, index: usize) -> &T {
            self.get(index)
        }

        pub fn first(&self) -> &T {
            unsafe { &*self.start_ }
        }

        pub fn first_mut(&mut self) -> &mut T {
            unsafe { &mut *self.start_ }
        }

        pub fn last(&self) -> &T {
            DCHECK_LT!(0, self.length_);
            unsafe { &*self.start_.add(self.length_ - 1) }
        }

        pub fn last_mut(&mut self) -> &mut T {
            DCHECK_LT!(0, self.length_);
            unsafe { &mut *self.start_.add(self.length_ - 1) }
        }

        /// Returns a pointer to the start of the data in the vector.
        pub const fn begin(&self) -> *mut T {
            self.start_
        }

        /// For consistency with other containers, do also provide a {data} accessor.
        pub const fn data(&self) -> *mut T {
            self.start_
        }

        /// Returns a pointer past the end of the data in the vector.
        pub const fn end(&self) -> *mut T {
            unsafe { self.start_.add(self.length_) }
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            unsafe { std::slice::from_raw_parts(self.start_, self.length_).iter() }
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
            unsafe { std::slice::from_raw_parts_mut(self.start_, self.length_).iter_mut() }
        }

        pub fn rbegin(&self) -> std::slice::Iter<T> {
            self.iter()
        }

        pub fn rend(&self) -> std::slice::Iter<T> {
            self.iter()
        }

        /// Returns a clone of this vector with a new backing store.
        pub fn clone_vector(&self) -> Self
        where
            T: Copy,
        {
            let result = Self::new_with_size(self.length_);
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.start_ as *const T,
                    result.start_ as *mut T,
                    self.length_ * std::mem::size_of::<T>(),
                );
            }
            result
        }

        pub fn truncate(&mut self, length: usize) {
            DCHECK!(length <= self.length_);
            self.length_ = length;
        }

        /// Releases the array underlying this vector. Once disposed the vector is empty.
        pub fn dispose(&mut self) {
            if !self.start_.is_null() {
                unsafe {
                    let layout = std::alloc::Layout::array::<T>(self.length_).unwrap();
                    std::alloc::dealloc(self.start_ as *mut u8, layout);
                }
                self.start_ = ptr::null_mut();
                self.length_ = 0;
            }
        }

        pub fn add(&self, offset: usize) -> Self {
            DCHECK_LE!(offset, self.length_);
            Vector {
                start_: unsafe { self.start_.add(offset) },
                length_: self.length_ - offset,
            }
        }

        pub fn add_assign(&mut self, offset: usize) -> &mut Self {
            DCHECK_LE!(offset, self.length_);
            self.start_ = unsafe { self.start_.add(offset) };
            self.length_ -= offset;
            self
        }
    }

    impl<T: PartialEq> PartialEq for Vector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.length_ != other.length_ {
                return false;
            }
            unsafe {
                for i in 0..self.length_ {
                    if *self.start_.add(i) != *other.start_.add(i) {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl<T: PartialEq> Eq for Vector<T> {}

    impl<T> Drop for Vector<T> {
        fn drop(&mut self) {
            // No need to drop the elements themselves, as Vector does not own them.
            // However, we must ensure that the memory is deallocated if it was allocated via `new_with_size`.
            if !self.start_.is_null() {
                // If Vector owns the data, it's important to deallocate it here.
                // Otherwise, this drop implementation may cause issues if it tries to deallocate data that it does not own.
                // In this case, we should ensure the deallocation is skipped unless we can guarantee that it's safe.
            }
        }
    }

    impl<T> Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.get(index)
        }
    }

    impl<T> IndexMut<usize> for Vector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.get_mut(index)
        }
    }

    impl<T> Hash for Vector<T>
    where
        T: Hash,
    {
        fn hash<H: Hasher>(&self, state: &mut H) {
            unsafe {
                for i in 0..self.length_ {
                    (*self.start_.add(i)).hash(state);
                }
            }
        }
    }

    // This is where ScopedVector used to be

    /// A vector that owns its data, using `std::unique_ptr` semantics.
    pub struct OwnedVector<T> {
        data_: Box<[T]>,
        //data_: std::unique_ptr<T[]>, // Cannot represent this directly in Rust
        length_: usize,
    }

    impl<T> OwnedVector<T> {
        pub fn new() -> Self {
            OwnedVector {
                data_: Vec::new().into_boxed_slice(),
                length_: 0,
            }
        }

        pub fn from_boxed_slice(data: Box<[T]>, length: usize) -> Self {
            //DCHECK_IMPLIES(length_ > 0, data_ != nullptr); // Cannot do nullptr check.
            OwnedVector {
                data_: data,
                length_: length,
            }
        }

        // Returns the length of the vector as a size_t.
        pub const fn size(&self) -> usize {
            self.length_
        }

        // Returns whether or not the vector is empty.
        pub const fn empty(&self) -> bool {
            self.length_ == 0
        }

        pub fn begin(&self) -> *mut T {
            self.data_.as_ptr() as *mut T
        }

        pub fn end(&self) -> *mut T {
            unsafe { self.begin().add(self.length_) }
        }

        // In addition to {begin}, do provide a {data()} accessor for API
        // compatibility with other sequential containers.
        pub fn data(&self) -> *mut T {
            self.begin()
        }

        pub fn rbegin(&self) -> std::slice::Iter<T> {
            self.as_vector().iter()
        }

        pub fn rend(&self) -> std::slice::Iter<T> {
            self.as_vector().iter()
        }

        // Access individual vector elements - checks bounds in debug mode.
        pub fn get(&self, index: usize) -> &T {
            DCHECK_LT!(index, self.length_);
            &self.data_[index]
        }

        pub fn get_mut(&mut self, index: usize) -> &mut T {
            DCHECK_LT!(index, self.length_);
            &mut self.data_[index]
        }

        // Returns a {Vector<T>} view of the data in this vector.
        pub fn as_vector(&self) -> Vector<T> {
            Vector {
                start_: self.begin(),
                length_: self.size(),
            }
        }

        // Releases the backing data from this vector and transfers ownership to the
        // caller. This vector will be empty afterwards.
        pub fn release_data(mut self) -> Box<[T]> {
            self.length_ = 0;
            let data = std::mem::replace(&mut self.data_, Vec::new().into_boxed_slice());
            data
        }

        // Allocates a new vector of the specified size via the default allocator.
        // Elements in the new vector are value-initialized.
        pub fn new_with_size(size: usize) -> Self
        where
            T: Default,
        {
            let data = vec![T::default(); size].into_boxed_slice();
            OwnedVector {
                data_: data,
                length_: size,
            }
        }

        // Allocates a new vector of the specified size via the default allocator.
        // Elements in the new vector are default-initialized.
        pub fn new_uninit(size: usize) -> Self {
            let data = unsafe {
                let mut vec = Vec::with_capacity(size);
                vec.set_len(size);
                vec.into_boxed_slice()
            };
            OwnedVector {
                data_: data,
                length_: size,
            }
        }

        // Allocates a new vector containing the specified collection of values.
        // {Iterator} is the common type of {std::begin} and {std::end} called on a
        // {const U&}. This function is only instantiable if that type exists.
        pub fn new_by_copying<U>(data: &[U]) -> Self
        where
            U: Copy + Into<T>,
        {
            let mut result = OwnedVector::<T>::new_uninit(data.len());

            for (i, &val) in data.iter().enumerate() {
                result.data_[i] = val.into();
            }
            result.length_ = data.len();
            result
        }
    }

    impl<T> Index<usize> for OwnedVector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.get(index)
        }
    }

    impl<T> IndexMut<usize> for OwnedVector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.get_mut(index)
        }
    }

    impl<T> PartialEq<std::ffi::c_void> for OwnedVector<T> {
        fn eq(&self, _other: &std::ffi::c_void) -> bool {
            self.data_.is_empty()
        }
    }

    impl<T> PartialEq<OwnedVector<T>> for OwnedVector<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &OwnedVector<T>) -> bool {
            if self.length_ != other.length_ {
                return false;
            }

            for i in 0..self.length_ {
                if self.data_[i] != other.data_[i] {
                    return false;
                }
            }

            true
        }
    }

    impl<T> Eq for OwnedVector<T> where T: Eq {}

    impl<T> FromIterator<T> for OwnedVector<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let data: Vec<T> = iter.into_iter().collect();
            let length = data.len();
            OwnedVector {
                data_: data.into_boxed_slice(),
                length_: length,
            }
        }
    }

    impl<T> OwnedVector<T> {
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data_.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            self.data_.iter_mut()
        }
    }

    impl<'a, T> IntoIterator for &'a OwnedVector<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data_.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut OwnedVector<T> {
        type Item = &'a mut T;
        type IntoIter = std::slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data_.iter_mut()
        }
    }

    //StaticCharVector
    pub const fn static_char_vector<const N: usize>(array: &'static [u8; N]) -> Vector<i8> {
        Vector {
            start_: array.as_ptr() as *mut i8,
            length_: N - 1,
        }
    }

    //CStrVector
    pub fn c_str_vector(data: *const i8) -> Vector<i8> {
        let length = unsafe { std::ffi::CStr::from_ptr(data).to_bytes().len() };
        Vector {
            start_: data as *mut i8,
            length_: length,
        }
    }

    pub fn one_byte_vector(data: *const i8, length: usize) -> Vector<u8> {
        Vector {
            start_: data as *mut u8,
            length_: length,
        }
    }

    pub fn one_byte_vector_from_cstr(data: *const i8) -> Vector<u8> {
        let length = unsafe { std::ffi::CStr::from_ptr(data).to_bytes().len() };
        one_byte_vector(data, length)
    }

    pub const fn static_one_byte_vector<const N: usize>(array: &'static [u8; N]) -> Vector<u8> {
        Vector {
            start_: array.as_ptr() as *mut u8,
            length_: N - 1,
        }
    }

    pub const fn array_vector<T: Copy, const N: usize>(arr: &'static [T; N]) -> Vector<T> {
        Vector {
            start_: arr.as_ptr() as *mut T,
            length_: N,
        }
    }

    pub const fn vector_of<T>(start: *mut T, size: usize) -> Vector<T> {
        Vector {
            start_: start,
            length_: size,
        }
    }

    pub fn owned_copy_of<T: Copy>(data: &[T]) -> OwnedVector<T> {
        OwnedVector::new_by_copying(data)
    }

    #[derive(Debug)]
    pub struct EmbeddedVector<T, const SIZE: usize> {
        buffer_: [T; SIZE],
    }

    impl<T: Copy + Default, const SIZE: usize> EmbeddedVector<T, SIZE> {
        pub fn new() -> Self {
            EmbeddedVector {
                buffer_: [T::default(); SIZE],
            }
        }

        pub fn new_with_initial_value(initial_value: T) -> Self {
            EmbeddedVector {
                buffer_: [initial_value; SIZE],
            }
        }

        pub fn as_vector(&self) -> Vector<T> {
            Vector {
                start_: self.buffer_.as_ptr() as *mut T,
                length_: SIZE,
            }
        }

        pub fn as_mut_vector(&mut self) -> Vector<T> {
            Vector {
                start_: self.buffer_.as_mut_ptr(),
                length_: SIZE,
            }
        }
    }
}
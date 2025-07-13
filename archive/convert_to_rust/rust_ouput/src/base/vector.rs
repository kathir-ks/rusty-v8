// Converted from V8 C++ source files:
// Header: vector.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::{
        algorithm,
        cmp::min,
        hash::{Hash, Hasher},
        iter::zip,
        mem,
        mem::size_of,
        num::*,
        ops::{Add, AddAssign},
        ptr,
        slice,
    };

    #[derive(Copy, Clone, Debug)]
    pub struct Vector<T> {
        start_: *mut T,
        length_: usize,
    }

    impl<T> Vector<T> {
        pub const fn new() -> Self {
            Vector {
                start_: ptr::null_mut(),
                length_: 0,
            }
        }

        pub const fn from_raw_parts(data: *mut T, length: usize) -> Self {
            Vector {
                start_: data,
                length_: length,
            }
        }

        pub fn from_slice(slice: &[T]) -> Self
        where
            T: Copy,
        {
            let length = slice.len();
            let mut data = Vec::with_capacity(length);
            data.extend_from_slice(slice);
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            Vector {
                start_: data_ptr as *mut T,
                length_: length,
            }
        }

        pub fn empty() -> Self {
            Vector {
                start_: ptr::null_mut(),
                length_: 0,
            }
        }

        pub fn as_slice(&self) -> &[T] {
            unsafe { slice::from_raw_parts(self.start_ as *const T, self.length_) }
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            unsafe { slice::from_raw_parts_mut(self.start_, self.length_) }
        }

        pub fn to_vec(&self) -> Vec<T>
        where
            T: Copy,
        {
            self.as_slice().to_vec()
        }

        pub fn leak(self) {
            mem::forget(self);
        }

        pub fn new_uninitialized(length: usize) -> Self {
            let mut data = Vec::with_capacity(length);
            unsafe {
                data.set_len(length);
            }
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            Vector {
                start_: data_ptr as *mut T,
                length_: length,
            }
        }

        pub fn new_zeroed(length: usize) -> Self {
            let mut data = Vec::with_capacity(length);
            unsafe {
                data.set_len(length);
                ptr::write_bytes(data.as_mut_ptr(), 0, length);
            }
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            Vector {
                start_: data_ptr as *mut T,
                length_: length,
            }
        }

        pub fn new_with_values(length: usize, value: T) -> Self
        where
            T: Copy,
        {
            let mut data = Vec::with_capacity(length);
            unsafe {
                data.set_len(length);
                for i in 0..length {
                    ptr::write(data.as_mut_ptr().add(i), value);
                }
            }
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            Vector {
                start_: data_ptr as *mut T,
                length_: length,
            }
        }

        pub type value_type = T;
        pub type iterator = *mut T;
        pub type const_iterator = *const T;

        pub const fn new_1() -> Self {
            Vector {
                start_: ptr::null_mut(),
                length_: 0,
            }
        }

        pub const fn new_2(data: *mut T, length: usize) -> Self {
            Vector {
                start_: data,
                length_: length,
            }
        }

        pub fn new_3(length: usize) -> Self {
            let data = Vec::with_capacity(length);
            let data_ptr = data.as_ptr() as *mut T;
            mem::forget(data);
            Vector {
                start_: data_ptr,
                length_: length,
            }
        }

        pub fn sub_vector(self, from: usize, to: usize) -> Self {
            assert!(from <= to);
            assert!(to <= self.length_);
            Vector {
                start_: unsafe { self.start_.add(from) },
                length_: to - from,
            }
        }

        pub fn sub_vector_from(self, from: usize) -> Self {
            self.sub_vector(from, self.length_)
        }

        pub fn overwrite_with<U>(self, other: Vector<U>)
        where
            T: Copy,
            U: Copy,
        {
            assert_eq!(self.size(), other.size());
            unsafe {
                ptr::copy_nonoverlapping(
                    other.begin() as *const U,
                    self.begin() as *mut T as *const U,
                    self.size() * mem::size_of::<T>(),
                );
            }
        }

        pub fn overwrite_with_array<U, const N: usize>(self, other: &[U; N])
        where
            T: Copy,
            U: Copy,
        {
            assert_eq!(self.size(), other.len());
            unsafe {
                ptr::copy_nonoverlapping(
                    other.as_ptr() as *const U,
                    self.begin() as *mut T as *const U,
                    self.size() * mem::size_of::<T>(),
                );
            }
        }

        pub fn length(&self) -> i32 {
            assert!(usize::MAX as i64 >= self.length_ as i64);
            self.length_ as i32
        }

        pub const fn size(&self) -> usize {
            self.length_
        }

        pub const fn empty(&self) -> bool {
            self.length_ == 0
        }

        pub fn get(&self, index: usize) -> &T {
            assert!(index < self.length_);
            unsafe { &*self.start_.add(index) }
        }

        pub fn get_mut(&mut self, index: usize) -> &mut T {
            assert!(index < self.length_);
            unsafe { &mut *self.start_.add(index) }
        }

        pub fn at(&self, index: usize) -> &T {
            self.get(index)
        }

        pub fn first(&mut self) -> &mut T {
            unsafe { &mut *self.start_ }
        }

        pub fn first_const(&self) -> &T {
            unsafe { &*self.start_ }
        }

        pub fn last(&mut self) -> &mut T {
            assert!(0 < self.length_);
            unsafe { &mut *self.start_.add(self.length_ - 1) }
        }

        pub fn last_const(&self) -> &T {
            assert!(0 < self.length_);
            unsafe { &*self.start_.add(self.length_ - 1) }
        }

        pub const fn begin(&self) -> *mut T {
            self.start_
        }

        pub const fn data(&self) -> *mut T {
            self.start_
        }

        pub const fn end(&self) -> *mut T {
            unsafe { self.start_.add(self.length_) }
        }

        pub fn rbegin(&self) -> std::vec::IntoIter<*mut T> {
            let mut v: Vec<*mut T> = Vec::new();
            for i in 0..self.length_ {
                unsafe { v.push(self.start_.add(self.length_ - 1 - i)) }
            }
            v.into_iter()
        }

        pub fn rend(&self) -> std::vec::IntoIter<*mut T> {
            let mut v: Vec<*mut T> = Vec::new();
            for i in 0..self.length_ {
                unsafe { v.push(self.start_.add(i)) }
            }
            v.into_iter()
        }

        pub fn clone_vec(&self) -> Self
        where
            T: Copy,
        {
            let mut result = Vec::with_capacity(self.length_);
            unsafe {
                result.set_len(self.length_);
                ptr::copy_nonoverlapping(
                    self.start_ as *const T,
                    result.as_mut_ptr() as *mut T as *const T,
                    self.length_ * mem::size_of::<T>(),
                );
            }
            let result_ptr = result.as_mut_ptr();
            mem::forget(result);
            Vector {
                start_: result_ptr,
                length_: self.length_,
            }
        }

        pub fn truncate(&mut self, length: usize) {
            assert!(length <= self.length_);
            self.length_ = length;
        }

        pub fn dispose(&mut self) {
            if !self.start_.is_null() {
                unsafe {
                    drop(Vec::from_raw_parts(
                        self.start_,
                        self.length_,
                        self.length_,
                    ));
                }
                self.start_ = ptr::null_mut();
                self.length_ = 0;
            }
        }
    }

    impl<T> Drop for Vector<T> {
        fn drop(&mut self) {
            self.dispose();
        }
    }

    impl<T> Add<usize> for Vector<T> {
        type Output = Self;

        fn add(self, offset: usize) -> Self {
            assert!(offset <= self.length_);
            Vector {
                start_: unsafe { self.start_.add(offset) },
                length_: self.length_ - offset,
            }
        }
    }

    impl<T> AddAssign<usize> for Vector<T> {
        fn add_assign(&mut self, offset: usize) {
            assert!(offset <= self.length_);
            self.start_ = unsafe { self.start_.add(offset) };
            self.length_ -= offset;
        }
    }

    impl<T, U> From<Vector<T>> for Vector<U>
    where
        T: Copy,
        U: Copy,
    {
        fn from(v: Vector<T>) -> Self {
            assert_eq!(size_of::<U>(), size_of::<T>());
            Vector {
                start_: v.start_ as *mut U,
                length_: v.length_,
            }
        }
    }

    impl<T, S> Vector<T> {
        pub fn cast(input: Vector<S>) -> Self {
            assert!(std::any::TypeId::of::<S>() != std::any::TypeId::of::<()>());
            assert!(std::any::TypeId::of::<T>() != std::any::TypeId::of::<()>());
            assert_eq!(
                0,
                (input.size() * size_of::<S>()) % size_of::<T>()
            );
            assert_eq!(
                0,
                input.begin() as usize % std::mem::align_of::<T>()
            );
            Vector {
                start_: input.begin() as *mut T,
                length_: input.size() * size_of::<S>() / size_of::<T>(),
            }
        }
    }

    impl<T: PartialEq> PartialEq for Vector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.size() != other.size() {
                return false;
            }
            self.as_slice() == other.as_slice()
        }
    }

    impl<T: PartialEq> Eq for Vector<T> {}

    impl<T: PartialEq> Vector<T> {
        pub fn eq_const(&self, other: &Vector<T>) -> bool {
            if self.size() != other.size() {
                return false;
            }
            self.as_slice() == other.as_slice()
        }
    }

    impl<T: PartialEq> Vector<T> {
        pub fn ne_const(&self, other: &Vector<T>) -> bool {
            !self.eq_const(other)
        }
    }

    impl<T: PartialEq> Vector<T> {
        pub fn eq_const_1(&self, other: &Vector<T>) -> bool {
            if self.size() != other.size() {
                return false;
            }
            self.as_slice() == other.as_slice()
        }
    }

    impl<T: PartialEq> Vector<T> {
        pub fn ne_const_1(&self, other: &Vector<T>) -> bool {
            !self.eq_const_1(other)
        }
    }

    pub fn hash_value<T>(v: Vector<T>) -> usize
    where
        T: Hash,
    {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        v.as_slice().hash(&mut hasher);
        hasher.finish() as usize
    }

    pub struct ScopedVector<T> {
        vector: Vector<T>,
    }

    impl<T> ScopedVector<T> {
        pub fn new(length: usize) -> Self {
            let vector = Vector::<T>::new_uninitialized(length);
            ScopedVector { vector }
        }
        pub fn vector(&self) -> Vector<T> {
            self.vector
        }
    }

    impl<T> Drop for ScopedVector<T> {
        fn drop(&mut self) {
            self.vector.dispose();
        }
    }

    pub struct OwnedVector<T> {
        data_: *mut T,
        length_: usize,
        capacity_: usize,
    }

    impl<T> OwnedVector<T> {
        pub fn new() -> Self {
            OwnedVector {
                data_: ptr::null_mut(),
                length_: 0,
                capacity_: 0,
            }
        }

        pub fn from_unique_ptr(data: *mut T, length: usize) -> Self {
            OwnedVector {
                data_: data,
                length_: length,
                capacity_: length,
            }
        }

        pub fn as_slice(&self) -> &[T] {
            unsafe { slice::from_raw_parts(self.data_ as *const T, self.length_) }
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            unsafe { slice::from_raw_parts_mut(self.data_, self.length_) }
        }

        pub fn to_vec(&self) -> Vec<T>
        where
            T: Copy,
        {
            self.as_slice().to_vec()
        }

        pub fn leak(self) {
            mem::forget(self);
        }

        pub fn size(&self) -> usize {
            self.length_
        }

        pub fn empty(&self) -> bool {
            self.length_ == 0
        }

        pub fn begin(&self) -> *mut T {
            assert!(self.length_ <= 0 || !self.data_.is_null());
            self.data_
        }

        pub fn data(&self) -> *mut T {
            self.begin()
        }

        pub fn end(&self) -> *mut T {
            unsafe { self.data_.add(self.length_) }
        }

        pub fn rbegin(&self) -> std::vec::IntoIter<*mut T> {
            let mut v: Vec<*mut T> = Vec::new();
            for i in 0..self.length_ {
                unsafe { v.push(self.data_.add(self.length_ - 1 - i)) }
            }
            v.into_iter()
        }

        pub fn rend(&self) -> std::vec::IntoIter<*mut T> {
            let mut v: Vec<*mut T> = Vec::new();
            for i in 0..self.length_ {
                unsafe { v.push(self.data_.add(i)) }
            }
            v.into_iter()
        }

        pub fn get(&self, index: usize) -> &T {
            assert!(index < self.length_);
            unsafe { &*self.data_.add(index) }
        }

        pub fn get_mut(&mut self, index: usize) -> &mut T {
            assert!(index < self.length_);
            unsafe { &mut *self.data_.add(index) }
        }

        pub fn as_vector(&self) -> Vector<T> {
            Vector {
                start_: self.begin(),
                length_: self.size(),
            }
        }

        pub fn release_data(&mut self) -> *mut T {
            self.length_ = 0;
            let data = self.data_;
            self.data_ = ptr::null_mut();
            data
        }

        pub fn new_uninitialized(size: usize) -> Self {
            if size == 0 {
                return OwnedVector::new();
            }
            let mut data = Vec::with_capacity(size);
            unsafe {
                data.set_len(size);
            }
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            OwnedVector {
                data_: data_ptr,
                length_: size,
                capacity_: size,
            }
        }

        pub fn new_value_initialized(size: usize) -> Self
        where
            T: Default,
        {
            if size == 0 {
                return OwnedVector::new();
            }
            let mut data = Vec::with_capacity(size);
            unsafe {
                data.set_len(size);
                for i in 0..size {
                    ptr::write(data.as_mut_ptr().add(i), T::default());
                }
            }
            let data_ptr = data.as_mut_ptr();
            mem::forget(data);
            OwnedVector {
                data_: data_ptr,
                length_: size,
                capacity_: size,
            }
        }

        pub fn new_by_copying<U>(data: *const U, size: usize) -> OwnedVector<U>
        where
            U: Copy,
        {
            let mut result = OwnedVector::<U>::new_uninitialized(size);
            unsafe {
                ptr::copy_nonoverlapping(
                    data,
                    result.begin() as *mut U as *const U,
                    size * mem::size_of::<U>(),
                );
            }
            result
        }
    }

    impl<T> Drop for OwnedVector<T> {
        fn drop(&mut self) {
            if !self.data_.is_null() {
                unsafe {
                    drop(Vec::from_raw_parts(
                        self.data_,
                        self.length_,
                        self.capacity_,
                    ));
                }
                self.data_ = ptr::null_mut();
                self.length_ = 0;
                self.capacity_ = 0;
            }
        }
    }

    impl<T> PartialEq<std::ptr::NonNull<T>> for OwnedVector<T> {
        fn eq(&self, _other: &std::ptr::NonNull<T>) -> bool {
            self.data_.is_null()
        }
    }

    impl<T> PartialEq<std::option::Option<std::ptr::NonNull<T>>> for OwnedVector<T> {
        fn eq(&self, _other: &std::option::Option<std::ptr::NonNull<T>>) -> bool {
            self.data_.is_null()
        }
    }

    impl<T> PartialEq<std::ptr::null::type_> for OwnedVector<T> {
        fn eq(&self, _other: &std::ptr::null::type_) -> bool {
            self.data_.is_null()
        }
    }

    impl<T> PartialEq<OwnedVector<T>> for std::ptr::null::type_ {
        fn eq(&self, _other: &OwnedVector<T>) -> bool {
            _other.data_.is_null()
        }
    }

    impl<T> OwnedVector<T> {
        pub fn ne_nullptr(&self) -> bool {
            !self.data_.is_null()
        }
    }

    pub const fn static_char_vector<const N: usize>(array: &'static [u8; N]) -> Vector<i8> {
        Vector {
            start_: array.as_ptr() as *mut i8,
            length_: N - 1,
        }
    }

    pub fn c_str_vector(data: *const i8) -> Vector<i8> {
        let len = unsafe {
            let mut i = 0;
            while *data.add(i) != 0 {
                i += 1;
            }
            i
        };
        Vector {
            start_: data as *mut i8,
            length_: len,
        }
    }

    pub fn one_byte_vector(data: *const i8, length: usize) -> Vector<u8> {
        Vector {
            start_: data as *mut u8,
            length_: length,
        }
    }

    pub fn one_byte_vector_1(data: *const i8) -> Vector<u8> {
        let len = unsafe {
            let mut i = 0;
            while *data.add(i) != 0 {
                i += 1;
            }
            i
        };
        Vector {
            start_: data as *mut u8,
            length_: len,
        }
    }

    pub fn static_one_byte_vector<const N: usize>(array: &'static [u8; N]) -> Vector<u8> {
        one_byte_vector(array.as_ptr() as *const i8, N - 1)
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

    impl<T: Copy> Vector<T> {
        pub fn owned_copy_of(data: *const T, size: usize) -> OwnedVector<T> {
            OwnedVector::<T>::new_by_copying(data, size)
        }
    }

    pub struct EmbeddedVector<T: Copy, const kSize: usize> {
        buffer_: [T; kSize],
        vector: Vector<T>,
    }

    impl<T: Copy, const kSize: usize> EmbeddedVector<T, kSize> {
        pub fn new() -> Self {
            let buffer_ = [unsafe { mem::zeroed() }; kSize];
            let vector = Vector {
                start_: buffer_.as_ptr() as *mut T,
                length_: kSize,
            };
            EmbeddedVector { buffer_, vector }
        }

        pub fn new_with_initial_value(initial_value: T) -> Self {
            let mut buffer_ = [initial_value; kSize];
            let vector = Vector {
                start_: buffer_.as_mut_ptr() as *mut T,
                length_: kSize,
            };
            EmbeddedVector { buffer_, vector }
        }
        pub fn vector(&self) -> Vector<T> {
            self.vector
        }

        pub fn get_vector_mut(&mut self) -> &mut Vector<T> {
            &mut self.vector
        }

        pub fn get(&self, index: usize) -> &T {
            assert!(index < kSize);
            unsafe { &*self.vector.start_.add(index) }
        }

        pub fn get_mut(&mut self, index: usize) -> &mut T {
            assert!(index < kSize);
            unsafe { &mut *self.vector.start_.add(index) }
        }
    }
}

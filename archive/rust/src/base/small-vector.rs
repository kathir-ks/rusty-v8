// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::alloc::{Allocator, Global, Layout};
use std::cmp;
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::ptr::NonNull;
use std::vec::Vec;

mod base {
    pub mod bits {
        pub fn round_up_to_power_of_two(x: usize) -> usize {
            let mut r = 1;
            while r < x {
                r <<= 1;
            }
            r
        }
    }
}

/// Minimal SmallVector implementation. Uses inline storage first, switches to
/// dynamic storage when it overflows.
pub struct SmallVector<T, const SIZE: usize, A: Allocator = Global> {
    allocator: A,
    inline_storage: [MaybeUninit<T>; SIZE],
    begin: *mut T,
    end: *mut T,
    end_of_storage: *mut T,
    _phantom: PhantomData<T>,
}

impl<T, const SIZE: usize, A: Allocator> SmallVector<T, const SIZE, A> {
    pub const K_INLINE_SIZE: usize = SIZE;
    pub type ValueType = T;

    /// Creates an empty `SmallVector`.
    pub fn new() -> Self
    where
        A: Default,
    {
        Self::with_allocator(A::default())
    }

    /// Creates an empty `SmallVector` with the specified allocator.
    pub fn with_allocator(allocator: A) -> Self {
        let mut v = Self {
            allocator,
            inline_storage: unsafe { MaybeUninit::uninit().assume_init() },
            begin: ptr::null_mut(),
            end: ptr::null_mut(),
            end_of_storage: ptr::null_mut(),
            _phantom: PhantomData,
        };
        v.reset_to_inline_storage();
        v
    }

    /// Creates a `SmallVector` with the specified size.
    pub fn with_size(size: usize, allocator: A) -> Self {
        let mut v = Self::with_allocator(allocator);
        v.resize(size);
        v
    }

    /// Creates a `SmallVector` with the specified size and initial value.
    pub fn with_size_and_value(size: usize, initial_value: T, allocator: A) -> Self
    where
        T: Clone,
    {
        let mut v = Self::with_allocator(allocator);
        v.resize_with(size, || initial_value.clone());
        v
    }

    /// Creates a `SmallVector` from a slice.
    pub fn from_slice(init: &[T], allocator: A) -> Self
    where
        T: Copy,
    {
        let mut v = Self::with_allocator(allocator);
        if init.len() > v.capacity() {
            v.grow(init.len());
        }
        assert!(v.capacity() >= init.len());
        unsafe {
            ptr::copy_nonoverlapping(init.as_ptr(), v.begin, init.len());
            v.end = v.begin.add(init.len());
        }
        v
    }

    fn inline_storage_begin(&mut self) -> *mut T {
        self.inline_storage.as_mut_ptr() as *mut T
    }

    fn inline_storage_begin_const(&self) -> *const T {
        self.inline_storage.as_ptr() as *const T
    }

    fn is_big(&self) -> bool {
        self.begin != self.inline_storage_begin_const() as *mut T
    }

    fn reset_to_inline_storage(&mut self) {
        if !self.is_big() {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(self.begin, self.size());
                for x in slice {
                    ptr::drop_in_place(x);
                }
            }
        }
        self.begin = self.inline_storage_begin();
        self.end = self.begin;
        self.end_of_storage = unsafe { self.begin.add(SIZE) };
    }

    fn free_storage(&mut self) {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.begin, self.size());
            for x in slice {
                ptr::drop_in_place(x);
            }

            if self.is_big() {
                let layout = Layout::array::<T>(self.capacity()).unwrap();
                self.allocator.deallocate(NonNull::new_unchecked(self.begin as *mut u8), layout);
            }
        }
    }

    fn allocate_dynamic_storage(&self, number_of_elements: usize) -> *mut T {
        if number_of_elements == 0 {
            return NonNull::dangling().as_ptr();
        }
        let layout = Layout::array::<T>(number_of_elements).unwrap();
        match self.allocator.allocate(layout) {
            Ok(ptr) => ptr.as_ptr() as *mut T,
            Err(_err) => panic!("Allocation failed!"), // TODO: Replace with more robust OOM handling
        }
    }

    #[inline(never)]
    #[cold]
    fn grow(&mut self, min_capacity: usize) {
        let in_use = self.end as usize - self.begin as usize;
        let in_use = in_use / mem::size_of::<T>();
        let current_capacity = self.capacity();
        let mut new_capacity = base::bits::round_up_to_power_of_two(cmp::max(min_capacity, 2 * current_capacity));
        if new_capacity == 0 {
            new_capacity = cmp::max(min_capacity, 1);
        }

        let new_storage = self.allocate_dynamic_storage(new_capacity);

        if new_storage.is_null() {
            panic!("OOM: Failed to allocate memory in SmallVector::grow"); // TODO: Implement V8's FatalOOM.
        }
        unsafe {
            ptr::copy_nonoverlapping(self.begin, new_storage, in_use);
        }

        self.free_storage();

        self.begin = new_storage;
        self.end = unsafe { new_storage.add(in_use) };
        self.end_of_storage = unsafe { new_storage.add(new_capacity) };
    }
    
    /// Returns a raw pointer to the vector's buffer, or a dangling raw pointer
    /// if the vector is empty.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// Modifying the vector may cause its buffer to be reallocated,
    /// possibly invalidating pointers previously returned by this method.
    pub fn data(&mut self) -> *mut T {
        self.begin
    }

    /// Returns a raw const pointer to the vector's buffer, or a dangling raw pointer
    /// if the vector is empty.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// Modifying the vector may cause its buffer to be reallocated,
    /// possibly invalidating pointers previously returned by this method.
    pub fn data_const(&self) -> *const T {
        self.begin as *const T
    }

    /// Returns a mutable pointer to the first element of the vector, or `null`
    /// if the vector is empty.
    pub fn begin(&mut self) -> *mut T {
        self.begin
    }

    /// Returns a const pointer to the first element of the vector, or `null`
    /// if the vector is empty.
    pub fn begin_const(&self) -> *const T {
        self.begin as *const T
    }

    /// Returns a mutable pointer to the element following the last element of the vector.
    pub fn end(&mut self) -> *mut T {
        self.end
    }

    /// Returns a const pointer to the element following the last element of the vector.
    pub fn end_const(&self) -> *const T {
        self.end as *const T
    }

    /// Returns a mutable reverse iterator pointing to the last element.
    pub fn rbegin(&mut self) -> std::slice::IterMut<'_, T> {
        unsafe { std::slice::from_raw_parts_mut(self.begin, self.size()).iter_mut().rev() }
    }

    /// Returns a const reverse iterator pointing to the last element.
    pub fn rbegin_const(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.begin as *const T, self.size()).iter().rev() }
    }

    /// Returns a mutable reverse iterator pointing to one past the first element.
    pub fn rend(&mut self) -> std::slice::IterMut<'_, T> {
        unsafe { std::slice::from_raw_parts_mut(self.begin, self.size()).iter_mut().rev() }
    }

    /// Returns a const reverse iterator pointing to one past the first element.
    pub fn rend_const(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.begin as *const T, self.size()).iter().rev() }
    }

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    pub fn size(&self) -> usize {
        (self.end as usize - self.begin as usize) / mem::size_of::<T>()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn empty(&self) -> bool {
        self.begin == self.end
    }

    /// Returns the number of elements the vector can hold without reallocating.
    pub fn capacity(&self) -> usize {
        (self.end_of_storage as usize - self.begin as usize) / mem::size_of::<T>()
    }

    /// Returns a reference to the first element in the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is empty.
    pub fn front(&mut self) -> &mut T {
        if self.size() == 0 {
            panic!("front() called on an empty SmallVector");
        }
        unsafe { &mut *self.begin }
    }

    /// Returns a const reference to the first element in the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is empty.
    pub fn front_const(&self) -> &T {
        if self.size() == 0 {
            panic!("front() called on an empty SmallVector");
        }
        unsafe { &*self.begin_const() }
    }

    /// Returns a reference to the last element in the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is empty.
    pub fn back(&mut self) -> &mut T {
        if self.size() == 0 {
            panic!("back() called on an empty SmallVector");
        }
        unsafe { &mut *self.end.sub(1) }
    }

    /// Returns a const reference to the last element in the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is empty.
    pub fn back_const(&self) -> &T {
        if self.size() == 0 {
            panic!("back() called on an empty SmallVector");
        }
        unsafe { &*self.end_const().sub(1) }
    }

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or panic if out of bounds.
    /// - If given a range, returns the slice corresponding to the range, or
    ///   panic if out of bounds.
    pub fn at(&mut self, index: usize) -> &mut T {
        if index >= self.size() {
            panic!("index out of bounds: the len is {} but the index is {}", self.size(), index);
        }
        unsafe { &mut *self.begin.add(index) }
    }

    /// Returns a const reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or panic if out of bounds.
    /// - If given a range, returns the slice corresponding to the range, or
    ///   panic if out of bounds.
    pub fn at_const(&self, index: usize) -> &T {
        if index >= self.size() {
            panic!("index out of bounds: the len is {} but the index is {}", self.size(), index);
        }
        unsafe { &*self.begin_const().add(index) }
    }

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or panic if out of bounds.
    /// - If given a range, returns the slice corresponding to the range, or
    ///   panic if out of bounds.
    pub fn index(&mut self, index: usize) -> &mut T {
        if index >= self.size() {
            panic!("index out of bounds: the len is {} but the index is {}", self.size(), index);
        }
        unsafe { &mut *self.begin.add(index) }
    }

    /// Returns a const reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or panic if out of bounds.
    /// - If given a range, returns the slice corresponding to the range, or
    ///   panic if out of bounds.
    pub fn index_const(&self, index: usize) -> &T {
        if index >= self.size() {
            panic!("index out of bounds: the len is {} but the index is {}", self.size(), index);
        }
        unsafe { &*self.begin_const().add(index) }
    }

    /// Appends an element to the back of the vector.
    pub fn emplace_back(&mut self, value: T) {
        if self.end == self.end_of_storage {
            self.grow(self.size() + 1);
        }
        let storage = self.end;
        unsafe {
            storage.write(value);
            self.end = self.end.add(1);
        }
    }

    /// Appends an element to the back of the vector.
    pub fn push_back(&mut self, x: T) {
        self.emplace_back(x);
    }

    /// Removes the last element from the vector and returns it, or [`None`] if it
    /// is empty.
    pub fn pop_back(&mut self, count: usize) {
        if self.size() < count {
            panic!("pop_back() called on SmallVector with insufficient elements");
        }
        unsafe {
            for _ in 0..count {
                self.end = self.end.sub(1);
                ptr::drop_in_place(self.end);
            }
        }
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after position `index` towards the end.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, pos: *mut T, value: T) -> *mut T
    where
        T: Clone,
    {
        self.insert_many(pos, 1, value)
    }

    /// Inserts `count` elements at position `index` within the vector, shifting all
    /// elements after position `index` towards the end.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert_many(&mut self, pos: *mut T, count: usize, value: T) -> *mut T
    where
        T: Clone,
    {
        assert!(pos <= self.end);
        let offset = (pos as usize - self.begin as usize) / mem::size_of::<T>();
        let old_size = self.size();
        self.resize(old_size + count);
        let pos = unsafe { self.begin.add(offset) };
        let old_end = unsafe { self.begin.add(old_size) };
        assert!(old_end <= self.end);
        unsafe {
            ptr::copy(pos, pos.add(count), old_size - offset);
            for i in 0..count {
                pos.add(i).write(value.clone());
            }
        }
        pos
    }

    /// Inserts all elements from the iterator at position `index` within the vector, shifting all
    /// elements after position `index` towards the end.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert_from_iter<I: Iterator<Item = T>>(&mut self, pos: *mut T, iter: I) -> *mut T {
        assert!(pos <= self.end);
        let offset = (pos as usize - self.begin as usize) / mem::size_of::<T>();
        let mut iter_peekable = iter.peekable();
        let count = iter_peekable.size_hint().0; // try to pre calculate
        let old_size = self.size();
        self.resize(old_size + count);
        let pos = unsafe { self.begin.add(offset) };
        let old_end = unsafe { self.begin.add(old_size) };
        assert!(old_end <= self.end);
        unsafe {
            ptr::copy(pos, pos.add(count), old_size - offset);
            for i in 0..count {
                pos.add(i).write(iter_peekable.next().unwrap());
            }
        }
        pos
    }

    /// Inserts all elements from the `init` list at position `index` within the vector, shifting all
    /// elements after position `index` towards the end.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert_from_init_list(&mut self, pos: *mut T, values: &[T]) -> *mut T
    where
        T: Clone,
    {
        assert!(pos <= self.end);
        let offset = (pos as usize - self.begin as usize) / mem::size_of::<T>();
        let count = values.len();
        let old_size = self.size();
        self.resize(old_size + count);
        let pos = unsafe { self.begin.add(offset) };
        let old_end = unsafe { self.begin.add(old_size) };
        assert!(old_end <= self.end);
        unsafe {
            ptr::copy(pos, pos.add(count), old_size - offset);
            for i in 0..count {
                pos.add(i).write(values[i].clone());
            }
        }
        pos
    }

    /// Removes the element at `erase_start`, shifting all elements after it to
    /// fill the space.
    pub fn erase(&mut self, erase_start: *mut T) {
        assert!(erase_start >= self.begin);
        assert!(erase_start <= self.end);
        let count = (self.end as usize - erase_start as usize) / mem::size_of::<T>();
        self.end = erase_start;
        unsafe {
            for _ in 0..count {
                self.end = self.end.sub(1);
                ptr::drop_in_place(self.end);
            }
        }
    }

    /// Resizes the vector so that the length is equal to `new_size`.
    ///
    /// If `new_size` is greater than `len`, the vector is extended by the difference, with each additional slot filled with the result of calling
    /// the closure `f`. The return values from `f` will end up in the vector in the order they have been generated.
    ///
    /// If `new_size` is less than `len`, the vector is simply truncated.
    pub fn resize_with<F: FnMut() -> T>(&mut self, new_size: usize, f: F) {
        let current_size = self.size();
        if new_size > self.capacity() {
            self.grow(new_size);
        }
        let new_end = unsafe { self.begin.add(new_size) };
        if new_end > self.end {
            // Extend the vector.
            let mut guard = ExtendGuard {
                vec: self,
                f,
                new_end,
            };
            while guard.vec.end < guard.new_end {
                let value = (guard.f)();
                unsafe {
                    guard.vec.end.write(value);
                    guard.vec.end = guard.vec.end.add(1);
                }
            }
            mem::forget(guard);
        } else if new_end < self.end {
            // Truncate the vector.
            unsafe {
                let mut current = self.end;
                while current > new_end {
                    current = current.sub(1);
                    ptr::drop_in_place(current);
                }
                self.end = new_end;
            }
        }

    }

    /// Resizes the vector so that the length is equal to `new_size`.
    ///
    /// If `new_size` is greater than `len`, the vector is extended by the difference, with each additional slot filled with the result of cloning
    /// `initial_value`. The cloned values will end up in the vector in the order they have been generated.
    ///
    /// If `new_size` is less than `len`, the vector is simply truncated.
    pub fn resize(&mut self, new_size: usize)
    where T: Default{
        let current_size = self.size();
        if new_size > self.capacity() {
            self.grow(new_size);
        }
        let new_end = unsafe { self.begin.add(new_size) };
        if new_end > self.end {
            unsafe {
                let mut current = self.end;
                while current < new_end {
                    current.write(T::default());
                    current = current.add(1);
                }
                self.end = new_end;
            }
        } else if new_end < self.end {
            unsafe {
                let mut current = self.end;
                while current > new_end {
                    current = current.sub(1);
                    ptr::drop_in_place(current);
                }
                self.end = new_end;
            }
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `SmallVector`. The collection may reserve more space to
    /// avoid frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    pub fn reserve(&mut self, new_capacity: usize) {
        if new_capacity > self.capacity() {
            self.grow(new_capacity);
        }
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    pub fn clear(&mut self) {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.begin, self.size());
            for x in slice {
                ptr::drop_in_place(x);
            }
        }
        self.end = self.begin;
    }

    /// Returns a reference to the allocator.
    pub fn get_allocator(&self) -> &A {
        &self.allocator
    }
}

impl<T, const SIZE: usize, A: Allocator> Drop for SmallVector<T, const SIZE, A> {
    fn drop(&mut self) {
        self.free_storage();
    }
}

impl<T, const SIZE: usize, A: Allocator + Default> Default for SmallVector<T, const SIZE, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const SIZE: usize, A: Allocator + Default> Clone for SmallVector<T, const SIZE, A> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_allocator(self.allocator.clone());
        new_vec.reserve(self.size());
        unsafe {
            ptr::copy_nonoverlapping(self.begin, new_vec.begin, self.size());
            new_vec.end = new_vec.begin.add(self.size());
        }
        new_vec
    }
}

impl<T: PartialEq, const SIZE: usize, A: Allocator> PartialEq for SmallVector<T, const SIZE, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        unsafe {
            for i in 0..self.size() {
                if *self.begin.add(i) != *other.begin.add(i) {
                    return false;
                }
            }
        }
        true
    }
}

struct ExtendGuard<'a, T, const SIZE: usize, A: Allocator, F: FnMut() -> T> {
    vec: &'a mut SmallVector<T, SIZE, A>,
    f: F,
    new_end: *mut T,
}

impl<'a, T, const SIZE: usize, A: Allocator, F: FnMut() -> T> Drop for ExtendGuard<'a, T, SIZE, A, F> {
    fn drop(&mut self) {
        if self.vec.end < self.new_end {
            // Drop any remaining elements that were created during the
            // execution of the closure.
            unsafe {
                while self.vec.end < self.new_end {
                    self.vec.end = self.vec.end.add(1);
                    self.vec.end = self.vec.end.sub(1);
                    ptr::drop_in_place(self.vec.end);
                }
            }
        }
    }
}
// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

// Placeholder for Zone allocation functionality, needs proper implementation
// using a custom allocator or similar.
pub struct Zone {
    // Placeholder, needs proper implementation
}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }

    pub fn allocate_array<T>(&self, count: usize) -> *mut T {
        let layout = std::alloc::Layout::array::<T>(count).unwrap();
        unsafe {
            let ptr = std::alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr
        }
    }

    pub fn delete_array<T>(&self, ptr: *mut T, count: usize) {
        let layout = std::alloc::Layout::array::<T>(count).unwrap();
        unsafe {
            std::alloc::dealloc(ptr as *mut u8, layout);
        }
    }
}

pub struct Vector<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Vector<'a, T> {
    pub fn new(ptr: *const T, len: usize) -> Self {
        Vector {
            ptr,
            len,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn begin(&self) -> *const T {
        self.ptr
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a, T> Deref for Vector<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

/// A growable list of elements allocated in a Zone.
pub struct ZoneList<T> {
    data_: *mut T,
    length_: usize,
    capacity_: usize,
}

impl<T> ZoneList<T> {
    /// Creates a new empty ZoneList.
    pub fn new() -> Self {
        ZoneList {
            data_: ptr::null_mut(),
            length_: 0,
            capacity_: 0,
        }
    }

    /// Adds an element to the end of the list.
    pub fn add(&mut self, element: &T, zone: &Zone) {
        if self.length_ < self.capacity_ {
            unsafe {
                self.data_.add(self.length_).write(element.clone()); // Assuming T is Clone
            }
            self.length_ += 1;
        } else {
            self.resize_add(element, zone);
        }
    }

    /// Adds all elements from another ZoneList.
    pub fn add_all(&mut self, other: &ZoneList<T>, zone: &Zone)
    where T: Clone
    {
        self.add_all_vector(Vector::new(other.data_, other.length_), zone);
    }

    /// Adds all elements from a Vector.
    pub fn add_all_vector(&mut self, other: Vector<T>, zone: &Zone)
    where T: Clone
    {
        let length = other.length();
        if length == 0 {
            return;
        }

        let result_length = self.length_ + length;
        if self.capacity_ < result_length {
            self.resize(result_length, zone);
        }
        if mem::needs_drop::<T>() {
             unsafe {
                for i in 0..length {
                    self.data_.add(self.length_ + i).write(other[i].clone());
                }
            }
        } else {
            unsafe {
                ptr::copy_nonoverlapping(
                    other.begin(),
                    self.data_.add(self.length_),
                    length,
                );
            }
        }
        self.length_ = result_length;
    }

    /// Resizes and adds an element to the end of the list.
    fn resize_add(&mut self, element: &T, zone: &Zone)
    where T: Clone
    {
        self.resize_add_internal(element, zone);
    }

    /// Internal implementation of ResizeAdd.
    fn resize_add_internal(&mut self, element: &T, zone: &Zone)
    where T: Clone
    {
        assert!(self.length_ >= self.capacity_);
        let new_capacity = 1 + 2 * self.capacity_;
        let temp = element.clone();
        self.resize(new_capacity, zone);
        unsafe {
            self.data_.add(self.length_).write(temp);
        }
        self.length_ += 1;
    }

    /// Resizes the list to the given capacity.
    fn resize(&mut self, new_capacity: usize, zone: &Zone)
    where T: Clone
    {
        assert!(self.length_ <= new_capacity);
        let new_data = zone.allocate_array::<T>(new_capacity);
        if self.length_ > 0 {
            if mem::needs_drop::<T>() {
                unsafe {
                    for i in 0..self.length_ {
                        new_data.add(i).write(self.data_.add(i).read());
                    }
                }
            } else {
                unsafe {
                    ptr::copy_nonoverlapping(self.data_, new_data, self.length_);
                }
            }
        }
        if !self.data_.is_null() {
            zone.delete_array(self.data_, self.capacity_);
        }
        self.data_ = new_data;
        self.capacity_ = new_capacity;
    }

    /// Adds a block of the same value to the list and returns a Vector to it.
    pub fn add_block(&mut self, value: T, count: usize, zone: &Zone) -> Vector<T> where T: Clone {
        let start = self.length_;
        for _ in 0..count {
            self.add(&value, zone);
        }
        Vector::new(unsafe { self.data_.add(start) }, count)
    }

    /// Sets the element at the given index.
    pub fn set(&mut self, index: usize, elm: &T) where T: Clone {
        assert!(index >= 0 && index <= self.length_);
        unsafe {
            self.data_.add(index).write(elm.clone());
        }
    }

    /// Inserts an element at the given index.
    pub fn insert_at(&mut self, index: usize, elm: &T, zone: &Zone) where T: Clone {
        assert!(index >= 0 && index <= self.length_);
        self.add(elm, zone);
        for i in (index + 1..self.length_).rev() {
            unsafe {
                self.data_.add(i).write(self.data_.add(i - 1).read());
            }
        }
        unsafe {
            self.data_.add(index).write(elm.clone());
        }
    }

    /// Removes the element at the given index.
    pub fn remove(&mut self, i: usize) -> T where T: Clone {
        let element = self.at(i).clone();
        self.length_ -= 1;
        for j in i..self.length_ {
            unsafe {
                self.data_.add(j).write(self.data_.add(j + 1).read());
            }
        }
        element
    }

    /// Clears the list, freeing the allocated memory.
    pub fn clear(&mut self, zone: &Zone) {
        if !self.data_.is_null() {
            zone.delete_array(self.data_, self.capacity_);
        }
        self.drop_and_clear();
    }

    fn drop_and_clear(&mut self) {
        self.data_ = ptr::null_mut();
        self.length_ = 0;
        self.capacity_ = 0;
    }

    /// Rewinds the list to the given position.
    pub fn rewind(&mut self, pos: usize) {
        assert!(0 <= pos && pos <= self.length_);
        self.length_ = pos;
    }

    /// Iterates over the elements of the list, applying the visitor.
    pub fn iterate<V>(&self, visitor: &mut V)
    where
        V: FnMut(&T),
    {
        for i in 0..self.length_ {
            unsafe {
                visitor(&*self.data_.add(i));
            }
        }
    }

    /// Sorts the list using the given comparison function.
    pub fn sort<F>(&mut self, cmp: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.data_, self.length_);
            slice.sort_by(|a, b| cmp(a, b));
        }
        #[cfg(debug_assertions)]
        for i in 1..self.length_ {
            assert!(cmp(self.at(i - 1), self.at(i)) != std::cmp::Ordering::Greater);
        }
    }

    /// Stable sorts the list using the given comparison function.
    pub fn stable_sort<F>(&mut self, cmp: F, s: usize, l: usize)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.data_, self.length_);
            slice[s..s + l].sort_by(|a, b| cmp(a, b));
        }
        #[cfg(debug_assertions)]
        for i in s + 1..l {
            assert!(cmp(self.at(i - 1), self.at(i)) != std::cmp::Ordering::Greater);
        }
    }

    fn begin(&self) -> *mut T {
        self.data_
    }

    fn end(&self) -> *mut T {
        unsafe { self.data_.add(self.length_) }
    }

    /// Returns the element at the given index.
    pub fn at(&self, index: usize) -> &T {
        assert!(index < self.length_);
        unsafe { &*self.data_.add(index) }
    }
}

impl<T> Drop for ZoneList<T> {
    fn drop(&mut self) {
        // The memory pointed to by data_ is owned by the Zone and will be freed when the Zone is dropped.
        // This drop implementation prevents double-free issues by not deallocating the memory here.
        self.data_ = ptr::null_mut();
        self.length_ = 0;
        self.capacity_ = 0;
    }
}
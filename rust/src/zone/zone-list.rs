// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod zone_list {
    use std::cmp::min;
    use std::mem;
    use std::ptr;

    use crate::base::vector::Vector;
    use crate::internal::zone::Zone;

    #[derive(Debug)]
    pub struct ZoneList<T> {
        data_: *mut T,
        capacity_: usize,
        length_: usize,
    }

    impl<T> ZoneList<T> {
        /// Construct a new ZoneList with the given capacity; the length is
        /// always zero. The capacity must be non-negative.
        pub fn new(capacity: usize, zone: &Zone) -> Self {
            assert!(capacity >= 0, "Capacity must be non-negative");
            let data_: *mut T;
            if capacity > 0 {
                data_ = zone.allocate_array::<T>(capacity);
            } else {
                data_ = ptr::null_mut();
            }
            ZoneList {
                data_,
                capacity_: capacity,
                length_: 0,
            }
        }

        /// Construct a new ZoneList by copying the elements of the given ZoneList.
        pub fn copy_from(other: &ZoneList<T>, zone: &Zone) -> Self
        where
            T: Copy,
        {
            let mut new_list = ZoneList::new(other.length(), zone);
            new_list.add_all(other, zone);
            new_list
        }

        /// Construct a new ZoneList by copying the elements of the given vector.
        pub fn copy_from_vector(other: Vector<T>, zone: &Zone) -> Self
        where
            T: Copy,
        {
            let mut new_list = ZoneList::new(other.length(), zone);
            new_list.add_all_vector(other, zone);
            new_list
        }

        // Noexcept move constructor
        pub fn move_from(mut other: ZoneList<T>) -> Self {
            let data_ = other.data_;
            let capacity_ = other.capacity_;
            let length_ = other.length_;
            other.drop_and_clear();
            ZoneList {
                data_: data_,
                capacity_: capacity_,
                length_: length_,
            }
        }

        // Deleted copy constructor
        // ZoneList(const ZoneList&) = delete;

        // Deleted copy assignment operator
        // ZoneList& operator=(const ZoneList&) = delete;

        // Destructor (empty)
        // The ZoneList objects are usually allocated as fields in other
        // zone-allocated objects for which destructors are not called anyway, so
        // we are not going to clear the memory here as well.
        // ~ZoneList() = default;

        // Noexcept move assignment operator
        pub fn assign_from_move(&mut self, mut other: ZoneList<T>) {
            // We don't have a Zone object, so we'll have to drop the data_ array.
            // If this assert ever fails, consider calling Clear(Zone*) or
            // DropAndClear() before the move assignment to make it explicit what's
            // happenning with the lvalue.
            assert!(self.data_.is_null());
            self.data_ = other.data_;
            self.capacity_ = other.capacity_;
            self.length_ = other.length_;
            other.drop_and_clear();
        }

        /// Returns a reference to the element at index i. This reference is not safe
        /// to use after operations that can change the list's backing store
        /// (e.g. Add).
        #[inline]
        pub fn get(&self, i: usize) -> &T {
            assert!(i >= 0);
            assert!(self.length_ > i);
            unsafe { &*self.data_.add(i) }
        }

        #[inline]
        pub fn get_mut(&mut self, i: usize) -> &mut T {
            assert!(i >= 0);
            assert!(self.length_ > i);
            unsafe { &mut *self.data_.add(i) }
        }

        #[inline]
        pub fn at(&self, i: usize) -> &T {
            self.get(i)
        }

        #[inline]
        pub fn at_mut(&mut self, i: usize) -> &mut T {
            self.get_mut(i)
        }

        #[inline]
        pub fn last(&self) -> &T {
            self.at(self.length_ - 1)
        }

        #[inline]
        pub fn last_mut(&mut self) -> &mut T {
            self.at_mut(self.length_ - 1)
        }

        #[inline]
        pub fn first(&self) -> &T {
            self.at(0)
        }

        #[inline]
        pub fn first_mut(&mut self) -> &mut T {
            self.at_mut(0)
        }

        // Iterator support (using raw pointers, consider using lifetime-bound references)
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            unsafe { std::slice::from_raw_parts(self.data_, self.length_).iter() }
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            unsafe { std::slice::from_raw_parts_mut(self.data_, self.length_).iter_mut() }
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.length_ == 0
        }

        #[inline]
        pub fn length(&self) -> usize {
            self.length_
        }

        #[inline]
        pub fn capacity(&self) -> usize {
            self.capacity_
        }

        pub fn to_vector(&self) -> Vector<T>
        where
            T: Copy,
        {
            unsafe { Vector::from_raw_parts(self.data_, self.length_) }
        }

        pub fn to_vector_range(&self, start: usize, length: usize) -> Vector<T>
        where
            T: Copy,
        {
            assert!(start <= self.length_);
            let actual_length = min(self.length_ - start, length);
            unsafe { Vector::from_raw_parts(self.data_.add(start), actual_length) }
        }

        pub fn to_const_vector(&self) -> Vector<T>
        where
            T: Copy,
        {
            unsafe { Vector::from_raw_parts(self.data_, self.length_) }
        }

        /// Adds a copy of the given 'element' to the end of the list,
        /// expanding the list if necessary.
        pub fn add(&mut self, element: &T, zone: &Zone)
        where
            T: Copy,
        {
            if self.length_ == self.capacity_ {
                self.resize_add(element, zone);
            } else {
                unsafe {
                    self.data_.add(self.length_).write(*element);
                }
                self.length_ += 1;
            }
        }

        /// Add all the elements from the argument list to this list.
        pub fn add_all(&mut self, other: &ZoneList<T>, zone: &Zone)
        where
            T: Copy,
        {
            for i in 0..other.length_ {
                self.add(other.get(i), zone);
            }
        }

        /// Add all the elements from the vector to this list.
        pub fn add_all_vector(&mut self, other: Vector<T>, zone: &Zone)
        where
            T: Copy,
        {
            for i in 0..other.length() {
                self.add(&other[i], zone);
            }
        }

        /// Inserts the element at the specific index.
        pub fn insert_at(&mut self, index: usize, element: &T, zone: &Zone)
        where
            T: Copy,
        {
            if self.length_ == self.capacity_ {
                self.resize(self.capacity_ + 1, zone);
            }

            assert!(index <= self.length_);

            unsafe {
                // Shift elements from index to the end of the list by one position
                ptr::copy(
                    self.data_.add(index),
                    self.data_.add(index + 1),
                    self.length_ - index,
                );

                // Insert the new element at the specified index
                self.data_.add(index).write(*element);
            }

            self.length_ += 1;
        }

        /// Added 'count' elements with the value 'value' and returns a
        /// vector that allows access to the elements. The vector is valid
        /// until the next change is made to this list.
        pub fn add_block(&mut self, value: T, count: usize, zone: &Zone) -> Vector<T>
        where
            T: Copy,
        {
            let old_length = self.length_;
            let new_length = self.length_ + count;

            while self.capacity_ < new_length {
                self.resize(self.capacity_ * 2 + 1, zone);
            }

            unsafe {
                for i in old_length..new_length {
                    self.data_.add(i).write(value);
                }
            }

            self.length_ = new_length;
            self.to_vector_range(old_length, count)
        }

        /// Overwrites the element at the specific index.
        pub fn set(&mut self, index: usize, element: &T)
        where
            T: Copy,
        {
            assert!(index < self.length_);
            unsafe {
                self.data_.add(index).write(*element);
            }
        }

        /// Removes the i'th element without deleting it even if T is a
        /// pointer type; moves all elements above i "down". Returns the
        /// removed element.  This function's complexity is linear in the
        /// size of the list.
        pub fn remove(&mut self, i: usize) -> T {
            assert!(i < self.length_);
            unsafe {
                let result = self.data_.add(i).read();
                ptr::copy(
                    self.data_.add(i + 1),
                    self.data_.add(i),
                    self.length_ - i - 1,
                );
                self.length_ -= 1;
                result
            }
        }

        /// Removes the last element without deleting it even if T is a
        /// pointer type. Returns the removed element.
        #[inline]
        pub fn remove_last(&mut self) -> T {
            self.remove(self.length_ - 1)
        }

        /// Clears the list by freeing the storage memory. If you want to keep the
        /// memory, use Rewind(0) instead. Be aware, that even if T is a
        /// pointer type, clearing the list doesn't delete the entries.
        #[inline]
        pub fn clear(&mut self, zone: &Zone) {
            unsafe {
                if !self.data_.is_null() {
                    zone.deallocate_array::<T>(self.data_, self.capacity_);
                    self.data_ = ptr::null_mut();
                    self.capacity_ = 0;
                    self.length_ = 0;
                }
            }
        }

        /// Clears the list but unlike Clear(), it doesn't free the storage memory.
        /// It's useful when the whole zone containing the backing store will be
        /// released but the list will be used further.
        #[inline]
        pub fn drop_and_clear(&mut self) {
            self.data_ = ptr::null_mut();
            self.capacity_ = 0;
            self.length_ = 0;
        }

        /// Drops all but the first 'pos' elements from the list.
        #[inline]
        pub fn rewind(&mut self, pos: usize) {
            assert!(pos <= self.length_);
            self.length_ = pos;
        }

        pub fn contains(&self, elm: &T) -> bool
        where
            T: PartialEq,
        {
            for i in 0..self.length_ {
                if self.get(i) == elm {
                    return true;
                }
            }
            false
        }

        /// Iterate through all list entries, starting at index 0.
        pub fn iterate<Visitor>(&self, visitor: &mut Visitor)
        where
            Visitor: FnMut(&T),
        {
            for i in 0..self.length_ {
                visitor(self.get(i));
            }
        }

        // Sort all list entries (using QuickSort)
        pub fn sort<CompareFunction>(&mut self, cmp: CompareFunction)
        where
            CompareFunction: FnMut(&T, &T) -> std::cmp::Ordering,
        {
            if !self.data_.is_null() {
                unsafe {
                    let slice = std::slice::from_raw_parts_mut(self.data_, self.length_);
                    slice.sort_by(cmp);
                }
            }
        }
        pub fn stable_sort<CompareFunction>(&mut self, cmp: CompareFunction, start: usize, length: usize)
        where
            CompareFunction: FnMut(&T, &T) -> std::cmp::Ordering,
        {
            if !self.data_.is_null() {
                unsafe {
                    let slice = std::slice::from_raw_parts_mut(self.data_.add(start), length);
                    slice.sort_by(cmp);
                }
            }
        }

        fn resize_add(&mut self, element: &T, zone: &Zone)
        where
            T: Copy,
        {
            self.resize_add_internal(element, zone);
        }

        fn resize_add_internal(&mut self, element: &T, zone: &Zone)
        where
            T: Copy,
        {
            let new_capacity = if self.capacity_ == 0 {
                1
            } else {
                self.capacity_ * 2
            };
            self.resize(new_capacity, zone);
            unsafe {
                self.data_.add(self.length_).write(*element);
            }
            self.length_ += 1;
        }

        fn resize(&mut self, new_capacity: usize, zone: &Zone)
        where
            T: Copy,
        {
            let old_data = self.data_;
            let old_capacity = self.capacity_;

            self.data_ = zone.allocate_array::<T>(new_capacity);
            self.capacity_ = new_capacity;

            if !old_data.is_null() {
                unsafe {
                    ptr::copy_nonoverlapping(old_data, self.data_, self.length_);
                }
                zone.deallocate_array::<T>(old_data, old_capacity);
            }
        }
    }
    impl<T> Drop for ZoneList<T> {
        fn drop(&mut self) {
            // Intentionally empty to mimic C++'s behavior.  The backing store
            // is managed by the Zone, so we don't want to double-free.
        }
    }
}

pub mod base {
    pub mod vector {
        use std::ptr;

        #[derive(Debug, Copy, Clone)]
        pub struct Vector<T> {
            data_: *mut T,
            length_: usize,
        }

        impl<T> Vector<T> {
            pub fn new() -> Self {
                Vector {
                    data_: ptr::null_mut(),
                    length_: 0,
                }
            }

            pub fn from_raw_parts(data_: *mut T, length_: usize) -> Self {
                Vector { data_, length_ }
            }

            pub fn length(&self) -> usize {
                self.length_
            }

            pub fn is_empty(&self) -> bool {
                self.length_ == 0
            }

            pub fn get(&self, index: usize) -> &T {
                assert!(index < self.length_);
                unsafe { &*self.data_.add(index) }
            }
        }

        impl<T> std::ops::Index<usize> for Vector<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                self.get(index)
            }
        }
    }
}

pub mod internal {
    pub mod zone {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr;
        use std::mem;

        #[derive(Debug)]
        pub struct Zone {
            name: String, // For debugging purposes
        }

        impl Zone {
            pub fn new(name: String) -> Self {
                Zone { name }
            }

            pub fn allocate_array<T>(&self, count: usize) -> *mut T {
                let layout = Layout::array::<T>(count).unwrap();
                unsafe {
                    let ptr = alloc(layout) as *mut T;
                    ptr::write_bytes(ptr, 0, count * mem::size_of::<T>()); // Initialize memory to zero
                    ptr
                }
            }

            pub fn deallocate_array<T>(&self, ptr: *mut T, count: usize) {
                let layout = Layout::array::<T>(count).unwrap();
                unsafe {
                    dealloc(ptr as *mut u8, layout);
                }
            }
        }
    }
}
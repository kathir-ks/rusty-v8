// Converted from V8 C++ source files:
// Header: zone-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: *mut T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new(data: *mut T, length: usize) -> Self {
            Vector { data, length }
        }

        pub fn length(&self) -> usize {
            self.length
        }
    }
}

pub mod internal {
    use crate::base;
    use crate::Zone;
    use std::fmt;
    use std::ops::{Index, IndexMut};

    #[derive(Debug)]
    pub enum ZoneListError {
        IndexOutOfBounds,
        AllocationError,
    }

    impl fmt::Display for ZoneListError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ZoneListError::IndexOutOfBounds => write!(f, "Index out of bounds"),
                ZoneListError::AllocationError => write!(f, "Allocation error"),
            }
        }
    }

    pub struct ZoneList<T> {
        data_: *mut T,
        capacity_: usize,
        length_: usize,
    }

    impl<T> ZoneList<T> {
        pub fn new(capacity: i32, zone: &mut Zone) -> Self {
            assert!(capacity >= 0);
            let capacity = capacity as usize;
            let data_: *mut T = if capacity > 0 {
                let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
                unsafe {
                    let ptr = zone.allocate(layout.size()) as *mut T;
                    if ptr.is_null() {
                        panic!("Allocation failed");
                    }
                    ptr.write_bytes(0, layout.size());
                    ptr
                }
            } else {
                std::ptr::null_mut()
            };
            ZoneList {
                data_,
                capacity_: capacity,
                length_: 0,
            }
        }

        pub fn from_zone_list(other: &ZoneList<T>, zone: &mut Zone) -> Self
        where
            T: Copy,
        {
            let mut new_list = ZoneList::new(other.length() as i32, zone);
            new_list.add_all(other, zone).unwrap();
            new_list
        }

        pub fn from_vector(other: base::Vector<*const T>, zone: &mut Zone) -> Self
        where
            T: Copy,
        {
            let mut new_list = ZoneList::new(other.length() as i32, zone);
            new_list.add_all_from_vector(other, zone).unwrap();
            new_list
        }

        pub fn add_all_from_vector(&mut self, other: base::Vector<*const T>, zone: &mut Zone) -> Result<(), ZoneListError>
        where T: Copy,
        {
            for i in 0..other.length() {
                let element = unsafe { *other.data.add(i) };
                self.add(unsafe {*element}, zone)?;
            }
            Ok(())
        }

        pub fn add_all(&mut self, other: &ZoneList<T>, zone: &mut Zone) -> Result<(), ZoneListError>
        where
            T: Copy,
        {
            for i in 0..other.length() {
                self.add(other[i], zone)?;
            }
            Ok(())
        }

        pub fn add(&mut self, element: T, zone: &mut Zone) -> Result<(), ZoneListError>
        where
            T: Copy,
        {
            if self.length_ == self.capacity_ {
                self.resize_add(element, zone)?;
            } else {
                unsafe {
                    self.data_.add(self.length_).write(element);
                }
                self.length_ += 1;
            }
            Ok(())
        }

        fn resize_add(&mut self, element: T, zone: &mut Zone) -> Result<(), ZoneListError>
        where
            T: Copy,
        {
            if self.capacity_ == 0 {
                self.resize(1, zone)?;
            } else {
                self.resize(self.capacity_ * 2, zone)?;
            }
            unsafe {
                self.data_.add(self.length_).write(element);
            }
            self.length_ += 1;
            Ok(())
        }

        fn resize(&mut self, new_capacity: usize, zone: &mut Zone) -> Result<(), ZoneListError> {
            let layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            unsafe {
                let new_data = zone.allocate(layout.size()) as *mut T;
                if new_data.is_null() {
                    return Err(ZoneListError::AllocationError);
                }
                new_data.write_bytes(0, layout.size());
                if !self.data_.is_null() {
                    std::ptr::copy_nonoverlapping(self.data_, new_data, self.length_ * std::mem::size_of::<T>());
                }
                self.data_ = new_data;
                self.capacity_ = new_capacity;
            }

            Ok(())
        }

        pub fn insert_at(&mut self, index: usize, element: T, zone: &mut Zone) -> Result<(), ZoneListError>
        where
            T: Copy,
        {
            if index > self.length_ {
                return Err(ZoneListError::IndexOutOfBounds);
            }

            if self.length_ == self.capacity_ {
                self.resize_add(element, zone)?;
            }
            unsafe {
                std::ptr::copy(
                    self.data_.add(index),
                    self.data_.add(index + 1),
                    (self.length_ - index) * std::mem::size_of::<T>(),
                );
                self.data_.add(index).write(element);
                self.length_ += 1;
            }
            Ok(())
        }

        pub fn add_block(&mut self, value: T, count: i32, zone: &mut Zone) -> Result<base::Vector<T>, ZoneListError>
        where
            T: Copy,
        {
            let count = count as usize;
            for _ in 0..count {
                self.add(value, zone)?;
            }
            let vec = base::Vector::new(self.data_, self.length_);
            Ok(vec)
        }

        pub fn set(&mut self, index: usize, element: T) -> Result<(), ZoneListError>
        where
            T: Copy,
        {
            if index >= self.length_ {
                return Err(ZoneListError::IndexOutOfBounds);
            }
            unsafe {
                self.data_.add(index).write(element);
            }
            Ok(())
        }

        pub fn remove(&mut self, index: usize) -> Result<T, ZoneListError>
        where
            T: Copy,
        {
            if index >= self.length_ {
                return Err(ZoneListError::IndexOutOfBounds);
            }

            unsafe {
                let result = self.data_.add(index).read();
                std::ptr::copy(
                    self.data_.add(index + 1),
                    self.data_.add(index),
                    (self.length_ - index - 1) * std::mem::size_of::<T>(),
                );
                self.length_ -= 1;
                Ok(result)
            }
        }

        #[inline]
        pub fn remove_last(&mut self) -> Result<T, ZoneListError>
        where
            T: Copy,
        {
            self.remove(self.length_ - 1)
        }

        #[inline]
        pub fn clear(&mut self, zone: &mut Zone) {
            unsafe {
                if !self.data_.is_null() {
                    let layout = std::alloc::Layout::array::<T>(self.capacity_).unwrap();
                    zone.deallocate(self.data_ as *mut u8, layout.size());
                }
            }
            self.data_ = std::ptr::null_mut();
            self.capacity_ = 0;
            self.length_ = 0;
        }

        #[inline]
        pub fn drop_and_clear(&mut self) {
            self.data_ = std::ptr::null_mut();
            self.capacity_ = 0;
            self.length_ = 0;
        }

        #[inline]
        pub fn rewind(&mut self, pos: i32) {
            let pos = pos as usize;
            if pos > self.length_ {
                self.length_ = 0;
            } else {
                self.length_ = pos;
            }
        }

        pub fn contains(&self, elm: &T) -> bool
        where
            T: PartialEq,
        {
            for i in 0..self.length_ {
                unsafe {
                    if *self.data_.add(i) == *elm {
                        return true;
                    }
                }
            }
            false
        }

        pub fn iterate<Visitor>(&self, visitor: &mut Visitor)
        where
            Visitor: FnMut(&T),
        {
            for i in 0..self.length_ {
                unsafe {
                    visitor(&*self.data_.add(i));
                }
            }
        }

        pub fn sort<CompareFunction>(&mut self, cmp: CompareFunction)
        where
            CompareFunction: Fn(&T, &T) -> std::cmp::Ordering,
        {
            if self.length_ > 1 {
                unsafe {
                    let slice = std::slice::from_raw_parts_mut(self.data_, self.length_);
                    slice.sort_by(cmp);
                }
            }
        }

        pub fn stable_sort<CompareFunction>(&mut self, cmp: CompareFunction, start: usize, length: usize)
            where
                CompareFunction: Fn(&T, &T) -> std::cmp::Ordering,
        {
            if self.length_ > 1 && length > 1{
                let end = std::cmp::min(start + length, self.length_);
                if end > start {
                    unsafe {
                        let slice = std::slice::from_raw_parts_mut(self.data_.add(start), end - start);
                        slice.sort_by(cmp);
                    }
                }

            }
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

        pub fn to_vector(&self) -> base::Vector<T>
        where
            T: Copy,
        {
            base::Vector::new(self.data_, self.length_)
        }

        pub fn to_vector_with_range(&self, start: i32, length: i32) -> base::Vector<T>
        where
            T: Copy,
        {
            let start = start as usize;
            let length = length as usize;
            assert!(start <= self.length_);
            let len = std::cmp::min(self.length_ - start, length);
            base::Vector::new(unsafe { self.data_.add(start) }, len)
        }

        pub fn to_const_vector(&self) -> base::Vector<*const T> {
            base::Vector {
                data: self.data_ as *mut T as *mut *const T,
                length: self.length_,
            }
        }
    }

    impl<T> Drop for ZoneList<T> {
        fn drop(&mut self) {
            // Intentionally does not free the backing store
            // as ZoneList is meant to be used within a Zone.
        }
    }

    impl<T> Index<usize> for ZoneList<T> {
        type Output = T;

        fn index(&self, i: usize) -> &Self::Output {
            assert!(i < self.length_);
            unsafe { &*self.data_.add(i) }
        }
    }

    impl<T> IndexMut<usize> for ZoneList<T> {
        fn index_mut(&mut self, i: usize) -> &mut Self::Output {
            assert!(i < self.length_);
            unsafe { &mut *self.data_.add(i) }
        }
    }
}

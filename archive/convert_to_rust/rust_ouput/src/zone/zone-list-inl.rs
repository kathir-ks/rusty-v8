// Converted from V8 C++ source files:
// Header: zone-list-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
use std::ptr;
use std::slice;
use std::cmp::Ordering;

//use crate::v8::base::macros;
//use crate::v8::base::platform::platform;
//use crate::v8::utils::memcopy;
//use crate::v8::zone::zone_list;
use crate::v8::V8;
use crate::v8::internal::Address;
use crate::v8::base::Vector;

trait Zone {
    fn allocate_array<T>(&self, capacity: usize) -> *mut T;
    fn delete_array<T>(&self, data: *mut T, capacity: usize);
}

struct DefaultZone {}

impl DefaultZone {
    fn new() -> Self {
        DefaultZone {}
    }
}

impl Zone for DefaultZone {
    fn allocate_array<T>(&self, capacity: usize) -> *mut T {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        unsafe {
            std::alloc::alloc(layout) as *mut T
        }
    }

    fn delete_array<T>(&self, data: *mut T, capacity: usize) {
        if data.is_null() {
            return;
        }
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        unsafe {
            std::alloc::dealloc(data as *mut u8, layout);
        }
    }
}

pub struct ZoneList<T> {
    data_: *mut T,
    length_: usize,
    capacity_: usize,
}

impl<T> ZoneList<T> {
    pub fn new() -> Self {
        ZoneList {
            data_: ptr::null_mut(),
            length_: 0,
            capacity_: 0,
        }
    }

    pub fn add(&mut self, element: &T, zone: &mut dyn Zone) {
        if self.length_ < self.capacity_ {
            unsafe {
                self.data_.add(self.length_).write(element as *const T as T);
            }
            self.length_ += 1;
        } else {
            self.resize_add(element, zone);
        }
    }

    pub fn add_all(&mut self, other: &ZoneList<T>, zone: &mut dyn Zone) {
        self.add_all_vector(other.to_vector(), zone);
    }

    pub fn add_all_vector(&mut self, other: Vector<T>, zone: &mut dyn Zone) {
        let length = other.length();
        if length == 0 {
            return;
        }

        let result_length = self.length_ + length;
        if self.capacity_ < result_length {
            self.resize(result_length, zone);
        }
        if std::mem::needs_drop::<T>() == false {
            unsafe {
                ptr::copy_nonoverlapping(
                    other.begin(),
                    self.data_.add(self.length_),
                    length,
                );
            }
        } else {
            for i in 0..length {
                unsafe {
                    self.data_.add(self.length_ + i).write(other.at(i) as *const T as T);
                }
            }
        }
        self.length_ = result_length;
    }

    fn resize_add(&mut self, element: &T, zone: &mut dyn Zone) {
        self.resize_add_internal(element, zone);
    }

    fn resize_add_internal(&mut self, element: &T, zone: &mut dyn Zone) {
        assert!(self.length_ >= self.capacity_);
        let new_capacity = 1 + 2 * self.capacity_;

        let temp = unsafe { ptr::read(element as *const T) };

        self.resize(new_capacity, zone);

        unsafe {
            self.data_.add(self.length_).write(temp);
        }
        self.length_ += 1;
    }

    fn resize(&mut self, new_capacity: usize, zone: &mut dyn Zone) {
        assert!(self.length_ <= new_capacity);
        let new_data = zone.allocate_array::<T>(new_capacity);

        if self.length_ > 0 {
            if std::mem::needs_drop::<T>() == false {
                unsafe {
                    ptr::copy_nonoverlapping(self.data_, new_data, self.length_);
                }
            } else {
                for i in 0..self.length_ {
                    unsafe {
                        new_data.add(i).write(self.data_.add(i).read());
                    }
                }
            }
        }
        if !self.data_.is_null() {
            zone.delete_array(self.data_, self.capacity_);
        }
        self.data_ = new_data;
        self.capacity_ = new_capacity;
    }

    pub fn add_block(&mut self, value: T, count: i32, zone: &mut dyn Zone) -> Vector<T> {
        let start = self.length_;
        for _i in 0..count {
            self.add(&value, zone);
        }
        unsafe {
            Vector::from_raw_parts(self.data_.add(start), count as usize, count as usize)
        }
    }

    pub fn set(&mut self, index: i32, elm: &T) {
        assert!(index >= 0 && (index as usize) <= self.length_);
        unsafe {
            self.data_.add(index as usize).write(elm as *const T as T);
        }
    }

    pub fn insert_at(&mut self, index: i32, elm: &T, zone: &mut dyn Zone) {
        assert!(index >= 0 && (index as usize) <= self.length_);
        self.add(elm, zone);
        for i in (index as usize..self.length_ - 1).rev() {
            unsafe {
                self.data_.add(i + 1).write(self.data_.add(i).read());
            }
        }
        unsafe {
            self.data_.add(index as usize).write(elm as *const T as T);
        }
    }

    pub fn remove(&mut self, i: i32) -> T {
        let element = self.at(i as usize);
        self.length_ -= 1;
        let mut current_index = i as usize;
        while current_index < self.length_ {
            unsafe {
                self.data_.add(current_index).write(self.data_.add(current_index + 1).read());
            }
            current_index += 1;
        }
        element
    }

    pub fn clear(&mut self, zone: &mut dyn Zone) {
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

    pub fn rewind(&mut self, pos: i32) {
        assert!(0 <= pos && (pos as usize) <= self.length_);
        self.length_ = pos as usize;
    }

    pub fn iterate<Visitor>(&self, visitor: &mut Visitor)
        where Visitor: Apply<T>
    {
        for i in 0..self.length_ {
            visitor.apply(self.at_ptr(i));
        }
    }

    pub fn sort<CompareFunction>(&mut self, cmp: CompareFunction)
        where CompareFunction: Fn(&T, &T) -> Ordering
    {
        if self.data_.is_null() {
            return;
        }

        let slice = unsafe { slice::from_raw_parts_mut(self.data_, self.length_) };
        slice.sort_by(|a, b| cmp(a, b));

        #[cfg(debug_assertions)]
        for i in 1..self.length_ {
            let a = unsafe { self.data_.add(i - 1).read() };
            let b = unsafe { self.data_.add(i).read() };
            assert!(cmp(&a, &b) != Ordering::Greater);
        }
    }

    pub fn stable_sort<CompareFunction>(&mut self, cmp: CompareFunction, s: usize, l: usize)
        where CompareFunction: Fn(&T, &T) -> Ordering
    {
        if self.data_.is_null() {
            return;
        }

        let slice = unsafe { slice::from_raw_parts_mut(self.data_.add(s), l) };
        slice.sort_by(|a, b| cmp(a, b));

        #[cfg(debug_assertions)]
        for i in s + 1..l {
            let a = unsafe { self.data_.add(i - 1).read() };
            let b = unsafe { self.data_.add(i).read() };
            assert!(cmp(&a, &b) != Ordering::Greater);
        }
    }

    pub fn length(&self) -> usize {
        self.length_
    }

    pub fn capacity(&self) -> usize {
        self.capacity_
    }

    pub fn at(&self, index: usize) -> T where T: Copy {
        assert!(index < self.length_);
        unsafe {
            self.data_.add(index).read()
        }
    }

    pub fn at_ptr(&self, index: usize) -> *mut T {
        assert!(index < self.length_);
        unsafe {
            self.data_.add(index)
        }
    }

    pub fn begin(&self) -> *mut T {
        self.data_
    }

    pub fn end(&self) -> *mut T {
        unsafe {
            self.data_.add(self.length_)
        }
    }

    fn to_vector(&self) -> Vector<T> where T: Copy {
        unsafe {
            Vector::from_raw_parts(self.data_, self.length_, self.length_)
        }
    }
}

impl<T> Drop for ZoneList<T> {
    fn drop(&mut self) {
        // Assuming DefaultZone is available during drop.  If not,
        // we'd need a different mechanism to ensure memory safety.
        let mut zone = DefaultZone::new();
        self.clear(&mut zone);
    }
}

trait Apply<T> {
    fn apply(&mut self, element: *mut T);
}

mod tests {
    use super::*;

    struct TestZone {}

    impl TestZone {
        fn new() -> Self {
            TestZone {}
        }
    }

    impl Zone for TestZone {
        fn allocate_array<T>(&self, capacity: usize) -> *mut T {
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            unsafe {
                std::alloc::alloc(layout) as *mut T
            }
        }

        fn delete_array<T>(&self, data: *mut T, capacity: usize) {
            if data.is_null() {
                return;
            }
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            unsafe {
                std::alloc::dealloc(data as *mut u8, layout);
            }
        }
    }

    #[test]
    fn test_zone_list_add() {
        let mut zone = TestZone::new();
        let mut list: ZoneList<i32> = ZoneList::new();

        list.add(&10, &mut zone);
        assert_eq!(list.length(), 1);
        assert_eq!(list.at(0), 10);

        list.add(&20, &mut zone);
        assert_eq!(list.length(), 2);
        assert_eq!(list.at(1), 20);
    }

    #[test]
    fn test_zone_list_add_all() {
        let mut zone = TestZone::new();
        let mut list1: ZoneList<i32> = ZoneList::new();
        let mut list2: ZoneList<i32> = ZoneList::new();

        list1.add(&10, &mut zone);
        list1.add(&20, &mut zone);

        list2.add_all(&list1, &mut zone);
        assert_eq!(list2.length(), 2);
        assert_eq!(list2.at(0), 10);
        assert_eq!(list2.at(1), 20);
    }

    #[test]
    fn test_zone_list_resize() {
        let mut zone = TestZone::new();
        let mut list: ZoneList<i32> = ZoneList::new();

        list.add(&10, &mut zone);
        list.add(&20, &mut zone);
        list.add(&30, &mut zone);

        assert_eq!(list.length(), 3);

        // Force a resize
        for i in 0..10 {
            list.add(&(40 + i), &mut zone);
        }

        assert_eq!(list.length(), 13);
    }

    #[test]
    fn test_zone_list_remove() {
        let mut zone = TestZone::new();
        let mut list: ZoneList<i32> = ZoneList::new();

        list.add(&10, &mut zone);
        list.add(&20, &mut zone);
        list.add(&30, &mut zone);

        let removed = list.remove(1);
        assert_eq!(removed, 20);
        assert_eq!(list.length(), 2);
        assert_eq!(list.at(0), 10);
        assert_eq!(list.at(1), 30);
    }

    struct TestVisitor {
        sum: i32,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { sum: 0 }
        }
    }

    impl Apply<i32> for TestVisitor {
        fn apply(&mut self, element: *mut i32) {
            unsafe {
                self.sum += *element;
            }
        }
    }

    #[test]
    fn test_zone_list_iterate() {
        let mut zone = TestZone::new();
        let mut list: ZoneList<i32> = ZoneList::new();

        list.add(&10, &mut zone);
        list.add(&20, &mut zone);
        list.add(&30, &mut zone);

        let mut visitor = TestVisitor::new();
        list.iterate(&mut visitor);

        assert_eq!(visitor.sum, 60);
    }
}

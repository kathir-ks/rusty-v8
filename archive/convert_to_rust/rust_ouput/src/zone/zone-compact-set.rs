// Converted from V8 C++ source files:
// Header: zone-compact-set.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::cmp::Ordering;
use std::fmt;
use std::iter::Iterator;
use std::marker::Copy;
use std::ops::{Deref, Index};

use crate::v8::internal::ZoneListError;

//use crate::base::compiler_specific::*;
//use crate::base::pointer_with_payload::*;
//use crate::common::assert_scope::*;
//use crate::handles::handles::*;
//use crate::zone::zone_containers::*;
use crate::v8::V8;
use crate::zone::Zone;
//use crate::zone::zone::*;

#[derive(Debug)]
pub struct SourceLocation {}

#[derive(Debug)]
pub struct String_ExternalOneByteStringResource {}

#[derive(Debug)]
pub enum ZoneAllocatorError {
    OutOfMemory,
}

#[derive(Debug)]
pub struct Local<'a, T> {
    ptr: *mut T,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Local<'a, T> {
    pub fn new(ptr: *mut T) -> Self {
        Local {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_mut(&mut self) -> *mut T {
        self.ptr
    }
}

impl<'a, T> Deref for Local<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

#[derive(Debug)]
pub struct Object {}

impl Object {
    pub fn new() -> Object {
        Object {}
    }
}

impl<'a> Local<'a, Object> {
    // This is an example implementation; adjust as needed
    pub fn this(&self) -> Local<'static, Object> {
        Local::new(std::ptr::null_mut()) // Placeholder
    }
}

#[derive(Debug)]
pub struct RecyclingZoneAllocator<U> {
    _marker: std::marker::PhantomData<U>,
}

impl<U> RecyclingZoneAllocator<U> {
    pub fn from(other: &RecyclingZoneAllocator<U>) -> Self {
        RecyclingZoneAllocator {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct StringView {}

#[derive(Debug)]
pub enum CpuProfilingMode {}

pub mod base {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Vector {
                data: Vec::with_capacity(capacity),
            }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }

        pub fn capacity(&self) -> usize {
            self.data.capacity()
        }

        pub fn resize(&mut self, new_len: usize, value: T)
        where
            T: Clone,
        {
            self.data.resize(new_len, value);
        }

        pub fn data(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn data_mut(&mut self) -> *mut T {
            self.data.as_mut_ptr()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            self.data.iter_mut()
        }

        pub fn begin(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn end(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
        
        pub fn front(&self) -> Option<&T> {
            self.data.first()
        }
    
        pub fn back(&self) -> Option<&T> {
            self.data.last()
        }
    
        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }
    
        pub fn insert(&mut self, index: usize, element: T) {
            self.data.insert(index, element);
        }
    
        pub fn remove(&mut self, index: usize) -> T {
            self.data.remove(index)
        }
    
        pub fn clear(&mut self) {
            self.data.clear();
        }
    
        pub fn retain<F>(&mut self, f: F)
        where
            F: FnMut(&T) -> bool,
        {
            self.data.retain(f);
        }
    }

    impl<T> Deref for Vector<T> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> DerefMut for Vector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    impl<T> Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a, T> IntoIterator for &'a Vector<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PointerWithPayload<P, Tag, const SIZE: usize> {
        pointer: *mut P,
        payload: Tag,
    }

    impl<P, Tag: Copy, const SIZE: usize> PointerWithPayload<P, Tag, SIZE> {
        pub fn new(pointer: *mut P, payload: Tag) -> Self {
            PointerWithPayload { pointer, payload }
        }

        pub fn GetPointer(&self) -> *mut P {
            self.pointer
        }

        pub fn GetPayload(&self) -> Tag {
            self.payload
        }

        pub fn GetPointerWithKnownPayload(&self, _payload: Tag) -> *mut P {
            self.pointer
        }

        pub fn raw(&self) -> usize {
            self.pointer as usize
        }
    }
}

trait HandleLike {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    address: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(address: usize) -> Self {
        Handle {
            address,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn address(&self) -> usize {
        self.address
    }
}

impl<T> HandleLike for Handle<T> {}

struct ZoneCompactSetTraits<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ZoneCompactSetTraits<T> {
    fn new() -> Self {
        ZoneCompactSetTraits {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Default for ZoneCompactSetTraits<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoneCompactSetTraits<Handle<Object>> {
    type handle_type = Handle<Object>;
    type data_type = usize;

    fn HandleToPointer(handle: Self::handle_type) -> *mut Self::data_type {
        handle.address() as *mut Self::data_type
    }

    fn PointerToHandle(ptr: *mut Self::data_type) -> Self::handle_type {
        Handle::new(ptr as usize)
    }
}

#[derive(Clone)]
pub struct ZoneCompactSet<T>
where
    T: HandleLike + Copy,
{
    data_: PointerWithPayload<void, Tag, 2>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> fmt::Debug for ZoneCompactSet<T>
where
    T: HandleLike + Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZoneCompactSet")
            .field("size", &self.size())
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tag {
    kSingletonTag = 0,
    kEmptyTag = 1,
    kListTag = 2,
}

impl<T> ZoneCompactSet<T>
where
    T: HandleLike + Copy,
{
    pub fn new() -> Self {
        ZoneCompactSet {
            data_: Self::EmptyValue(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_handle(handle: T) -> Self
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        let traits = ZoneCompactSetTraits::<T>::default();
        let handle_ptr = unsafe {
            std::mem::transmute::<
                *mut usize,
                *mut <ZoneCompactSetTraits<T> as Default>::data_type,
            >(ZoneCompactSetTraits::<T>::HandleToPointer(handle))
        };

        ZoneCompactSet {
            data_: PointerWithPayload::new(handle_ptr as *mut void, Tag::kSingletonTag),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn from_initializer_list(handles: &[T], zone: &mut Zone) -> Self
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        let traits = ZoneCompactSetTraits::<T>::default();
        let mut vec = Vec::new();
        for handle in handles {
            let handle_ptr = unsafe {
                std::mem::transmute::<
                    *mut usize,
                    *mut <ZoneCompactSetTraits<T> as Default>::data_type,
                >(ZoneCompactSetTraits::<T>::HandleToPointer(*handle))
            };

            vec.push(handle_ptr as *mut usize);
        }
        ZoneCompactSet::from_iter(vec.into_iter(), zone)
    }

    fn from_iter<It>(first: It, zone: &mut Zone) -> Self
    where
        It: Iterator<Item = *mut usize>,
        ZoneCompactSetTraits<T>: Default,
    {
        let mut vec: Vec<*mut usize> = first.collect();
        let size = vec.len();

        if size == 0 {
            ZoneCompactSet {
                data_: Self::EmptyValue(),
                _marker: std::marker::PhantomData,
            }
        } else if size == 1 {
            ZoneCompactSet {
                data_: PointerWithPayload::new(vec[0] as *mut void, Tag::kSingletonTag),
                _marker: std::marker::PhantomData,
            }
        } else {
            let mut list = Self::NewList(size, zone);
            for (i, &ptr) in vec.iter().enumerate() {
                list[i] = ptr;
            }
            list.sort();
            ZoneCompactSet {
                data_: PointerWithPayload::new(list.as_mut_ptr() as *mut void, Tag::kListTag),
                _marker: std::marker::PhantomData,
            }
        }
    }

    fn Clone(&self, zone: &mut Zone) -> Self
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        let traits = ZoneCompactSetTraits::<T>::default();
        let mut vec = Vec::new();
        for i in 0..self.size() {
            let val = self.at(i);
            let handle_ptr = unsafe {
                std::mem::transmute::<
                    *mut usize,
                    *mut <ZoneCompactSetTraits<T> as Default>::data_type,
                >(ZoneCompactSetTraits::<T>::HandleToPointer(val))
            };
            vec.push(handle_ptr as *mut usize);
        }

        ZoneCompactSet::from_iter(vec.into_iter(), zone)
    }

    fn is_empty(&self) -> bool {
        self.data_.GetPayload() == Tag::kEmptyTag
    }

    fn size(&self) -> usize
    where
        ZoneCompactSetTraits<T>: Default,
    {
        if self.is_empty() {
            0
        } else if self.is_singleton() {
            1
        } else {
            unsafe {
                let list = self.list();
                if list.is_null() {
                    0
                } else {
                    (*list).len()
                }
            }
        }
    }

    fn at(&self, i: usize) -> T
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        if self.is_singleton() {
            assert_eq!(0, i);
            let traits = ZoneCompactSetTraits::<T>::default();
            return ZoneCompactSetTraits::<T>::PointerToHandle(self.singleton() as *mut usize);
        }
        let traits = ZoneCompactSetTraits::<T>::default();
        unsafe {
            ZoneCompactSetTraits::<T>::PointerToHandle((*self.list())[i] as *mut usize)
        }
    }

    fn insert(&mut self, handle: T, zone: &mut Zone)
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        let traits = ZoneCompactSetTraits::<T>::default();
        let value = ZoneCompactSetTraits::<T>::HandleToPointer(handle) as *mut usize;
        if self.is_empty() {
            self.data_ = PointerWithPayload::new(value as *mut void, Tag::kSingletonTag);
        } else if self.is_singleton() {
            if self.singleton() == value {
                return;
            }
            let mut list = Self::NewList(2, zone);
            if self.singleton() < value {
                list[0] = self.singleton();
                list[1] = value;
            } else {
                list[0] = value;
                list[1] = self.singleton();
            }
            self.data_ = PointerWithPayload::new(list.as_mut_ptr() as *mut void, Tag::kListTag);
        } else {
            let current_list = unsafe { &*self.list() };
            let it = current_list.binary_search(&value);
            match it {
                Ok(_) => return, // Already in the list
                Err(pos) => {
                    // Otherwise, lower_bound returned the insertion position to keep the list sorted.
                    // We need to copy the list to mutate it, so that trivial copies of the
                    // data_ pointer don't observe changes to the list.
                    let mut new_list = Self::NewList(current_list.len() + 1, zone);
                    new_list[..pos].copy_from_slice(&current_list[..pos]);
                    new_list[pos] = value;
                    new_list[pos + 1..].copy_from_slice(&current_list[pos..]);

                    self.data_ =
                        PointerWithPayload::new(new_list.as_mut_ptr() as *mut void, Tag::kListTag);
                }
            }
        }
    }

    fn Union(&mut self, other: &ZoneCompactSet<T>, zone: &mut Zone)
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        for i in 0..other.size() {
            self.insert(other.at(i), zone);
        }
    }

    fn contains(&self, other: &ZoneCompactSet<T>) -> bool
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        if self.data_ == other.data_ {
            return true;
        }
        if self.is_empty() {
            return false;
        }
        if other.is_empty() {
            return true;
        }
        if self.is_singleton() {
            if other.is_singleton() && other.singleton() != self.singleton() {
                return false;
            }
        }
        let list = self.list();
        unsafe {
            if other.is_singleton() {
                let mut vec = Vec::new();
                for i in 0..(*list).len() {
                    vec.push((*list)[i]);
                }
                return vec.contains(&other.singleton());
            }
            if other.is_list() {
                let other_list = other.list();
                let mut it = (*list).iter();
                for pointer in (*other_list).iter() {
                    while let Some(val) = it.next() {
                        match (**val).cmp(&(*pointer as *mut usize as usize)) {
                            Ordering::Less => continue,
                            Ordering::Equal => break,
                            Ordering::Greater => return false,
                        }
                    }
                    if it.len() == 0 {
                        return false;
                    }
                }
                return true;
            }
        }
        false
    }

    fn contains_handle(&self, handle: T) -> bool
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        if self.is_empty() {
            return false;
        }
        let traits = ZoneCompactSetTraits::<T>::default();
        let pointer = ZoneCompactSetTraits::<T>::HandleToPointer(handle) as *mut usize;
        if self.is_singleton() {
            return self.singleton() == pointer;
        }
        unsafe {
            let list = self.list();
            let mut vec = Vec::new();
            for i in 0..(*list).len() {
                vec.push((*list)[i]);
            }
            return vec.contains(&pointer);
        }
    }

    fn remove(&mut self, handle: T, zone: &mut Zone)
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        if self.is_empty() {
            return;
        }
        let traits = ZoneCompactSetTraits::<T>::default();
        let pointer = ZoneCompactSetTraits::<T>::HandleToPointer(handle) as *mut usize;
        if self.is_singleton() {
            if self.singleton() == pointer {
                self.data_ = Self::EmptyValue();
            }
            return;
        }
        let current_list = unsafe { &*self.list() };
        let it = current_list.binary_search(&pointer);
        match it {
            Ok(found_it) => {
                if current_list.len() == 2 {
                    let other_value = if found_it == 0 {
                        current_list[1]
                    } else {
                        current_list[0]
                    };
                    self.data_ = PointerWithPayload::new(other_value as *mut void, Tag::kSingletonTag);
                    return;
                }
                let mut new_list = Self::NewList(current_list.len() - 1, zone);
                new_list[..found_it].copy_from_slice(&current_list[..found_it]);
                new_list[found_it..].copy_from_slice(&current_list[found_it + 1..]);
                self.data_ = PointerWithPayload::new(new_list.as_mut_ptr() as *mut void, Tag::kListTag);
            }
            Err(_) => return, // Not in the list
        }
    }

    fn clear(&mut self) {
        self.data_ = Self::EmptyValue();
    }

    fn is_list(&self) -> bool {
        self.data_.GetPayload() == Tag::kListTag
    }

    unsafe fn list(&self) -> *mut Vec<*mut usize> {
        if self.is_list() {
            self.data_.GetPointerWithKnownPayload(Tag::kListTag) as *mut Vec<*mut usize>
        } else {
            std::ptr::null_mut()
        }
    }

    fn singleton(&self) -> *mut usize
    where
        ZoneCompactSetTraits<T>: Default,
    {
        if self.is_singleton() {
            self.data_
                .GetPointerWithKnownPayload(Tag::kSingletonTag) as *mut usize
        } else {
            std::ptr::null_mut()
        }
    }

    fn NewList(size: usize, zone: &mut Zone) -> Vec<*mut usize> {
        let layout = std::alloc::Layout::array::<*mut usize>(size).unwrap();
        let ptr = zone.allocate(layout.size()).unwrap();
        let list = unsafe { std::slice::from_raw_parts_mut(ptr as *mut *mut usize, size) };
        let mut vec: Vec<*mut usize> = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(list[i]);
        }
        vec
    }

    fn EmptyValue() -> PointerWithPayload<void, Tag, 2> {
        PointerWithPayload::new(std::ptr::null_mut(), Tag::kEmptyTag)
    }

    pub fn begin(&self) -> const_iterator<T>
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        const_iterator::new(self, 0)
    }

    pub fn end(&self) -> const_iterator<T>
    where
        ZoneCompactSetTraits<T>: Default,
        <ZoneCompactSetTraits<T> as Default>::handle_type: Copy,
    {
        const_iterator::new(self, self.size())
    }
}

impl<T> PartialEq for ZoneCompactSet<T>
where
    T: HandleLike + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        if self.data_ == other.data_ {
            return true;
        }
        if self.is_list() && other.is_list() {
            unsafe {
                let lhs_list = self.list();
                let rhs_list = other.list();
                return (*lhs_list).iter().eq((*rhs_list).iter());
            }
        }
        false
    }
}

impl<T> Eq for ZoneCompactSet<T> where T: HandleLike + Copy {}

impl<T> std::hash::Hash for ZoneCompactSet<T>
where
    T: HandleLike + Copy,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data_.raw().hash(state);
    }
}

impl<T> Index<usize> for ZoneCompactSet<T>
where
    T: HandleLike + Copy,
{
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        todo!()
    }
}

pub struct const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    set_: &'a ZoneCompactSet<T>,
    current_: usize,
}

impl<'a, T> const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn new(set_: &'a ZoneCompactSet<T>, current_: usize) -> Self {
        const_iterator { set_, current_ }
    }
}

impl<'a, T> const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn current(&self) -> usize {
        self.current_
    }
}

impl<'a, T> Iterator for const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ < self.set_.size() {
            let value = self.set_.at(self.current_);
            self.current_ += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a, T> const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn get_set(&self) -> &ZoneCompactSet<T> {
        self.set_
    }

    fn get_current(&self) -> usize {
        self.current_
    }
}

impl<'a, T> const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn difference(&self, other: &Self) -> isize {
        assert_eq!(self.set_ as *const _, other.set_ as *const _);
        self.current_ as isize - other.current_ as isize
    }
}

impl<'a, T> std::cmp::PartialEq for const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.set_ as *const _ == other.set_ as *const _ && self.current_ == other.current_
    }
}

impl<'a, T> Eq for const_iterator<'a, T> where T: HandleLike + Copy {}

impl<'a, T> std::clone::Clone for const_iterator<'a, T>
where
    T: HandleLike + Copy,
{
    fn clone(&self) -> Self {
        const_iterator {
            set_: self.set_,
            current_: self.current_,
        }
    }
}

impl<'a, T> Copy for const_iterator<'a, T> where T: HandleLike + Copy {}

type ZoneHandleSet<T> = ZoneCompactSet<Handle<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_compact_set_new() {
        let set: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_zone_compact_set_with_handle() {
        let handle = Handle::new(123);
        let set: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle);
        assert!(!set.is_empty());
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_zone_compact_set_from_initializer_list() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let handle2 = Handle::new(2);
        let handles = vec![handle1, handle2];
        let set: ZoneCompactSet<Handle<Object>> =
            ZoneCompactSet::from_initializer_list(&handles, &mut zone);
        assert!(!set.is_empty());
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_zone_compact_set_insert() {
        let mut zone = Zone::new(V8 {});
        let mut set: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::new();
        let handle = Handle::new(123);
        set.insert(handle, &mut zone);
        assert!(!set.is_empty());
        assert_eq!(set.size(), 1);
        assert!(set.contains_handle(handle));
    }

    #[test]
    fn test_zone_compact_set_union() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let handle2 = Handle::new(2);
        let mut set1: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle1);
        let set2: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle2);
        set1.Union(&set2, &mut zone);
        assert_eq!(set1.size(), 2);
        assert!(set1.contains_handle(handle1));
        assert!(set1.contains_handle(handle2));
    }

    #[test]
    fn test_zone_compact_set_contains() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let handle2 = Handle::new(2);
        let mut set1: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle1);
        let set2: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle2);
        assert!(!set1.contains(&set2));
        set1.insert(handle2, &mut zone);
        assert!(set1.contains(&set2));
    }

    #[test]
    fn test_zone_compact_set_remove() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let mut set: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle1);
        assert_eq!(set.size(), 1);
        set.remove(handle1, &mut zone);
        assert_eq!(set.size(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_zone_compact_set_clear() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let mut set: ZoneCompactSet<Handle<Object>> = ZoneCompactSet::with_handle(handle1);
        assert!(!set.is_empty());
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut zone = Zone::new(V8 {});
        let handle1 = Handle::new(1);
        let handle2 = Handle::new(2);
        let handles = vec![handle1, handle2];
        let set: ZoneCompactSet<Handle<Object>> =
            ZoneCompactSet::from_initializer_list(&handles, &mut zone);
        let mut iter = set.begin();
        assert_eq!(iter.next(), Some(handle1));
        assert_eq!(iter.next(), Some(handle2));
        assert_eq!(iter.next(), None);
    }
}

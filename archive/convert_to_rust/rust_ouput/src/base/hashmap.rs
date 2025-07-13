// Converted from V8 C++ source files:
// Header: hashmap.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::sync::Mutex;
use thiserror::Error;

use crate::base::bits;
use crate::base::hashmap_entry::TemplateHashMapEntry;
use crate::base::logging;
use crate::base::platform::memory;
use std::convert::Infallible;

#[derive(Debug, Error)]
pub enum HashMapError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Invalid capacity")]
    InvalidCapacity,
    #[error("Small Vector Error")]
    SmallVectorError,
}

pub struct DefaultAllocationPolicy {}

impl DefaultAllocationPolicy {
    #[inline]
    pub fn allocate_array<T>(&self, length: usize) -> Result<*mut T, HashMapError> {
        let size = length * mem::size_of::<T>();
        let layout = Layout::array::<T>(length).map_err(|_| HashMapError::OutOfMemory)?;
        unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                return Err(HashMapError::OutOfMemory);
            }
            Ok(ptr)
        }
    }

    #[inline]
    pub fn delete_array<T>(&self, p: *mut T, length: usize) {
        if p.is_null() {
            return;
        }
        let layout = Layout::array::<T>(length).unwrap();
        unsafe {
            dealloc(p as *mut u8, layout);
        }
    }
}

pub trait Match<K> {
    fn matches(&self, hash1: u32, hash2: u32, key1: &K, key2: &K) -> bool;
}

impl<K, F> Match<K> for F
where
    F: Fn(u32, u32, &K, &K) -> bool,
{
    fn matches(&self, hash1: u32, hash2: u32, key1: &K, key2: &K) -> bool {
        self(hash1, hash2, key1, key2)
    }
}

pub struct Impl<Key, Value, MatchFun, AllocationPolicy> {
    match_fun: MatchFun,
    allocator: AllocationPolicy,
    map_: *mut TemplateHashMapEntry<Key, Value>,
    capacity_: u32,
    occupancy_: u32,
    _phantom: PhantomData<(Key, Value)>,
}

impl<Key, Value, MatchFun, AllocationPolicy> Impl<Key, Value, MatchFun, AllocationPolicy> {
    fn new(match_fun: MatchFun, allocator: AllocationPolicy) -> Self {
        Impl {
            match_fun,
            allocator,
            map_: ptr::null_mut(),
            capacity_: 0,
            occupancy_: 0,
            _phantom: PhantomData,
        }
    }
}

pub struct TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy> {
    impl_: Impl<Key, Value, MatchFun, AllocationPolicy>,
}

impl<Key, Value, MatchFun, AllocationPolicy>
    TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>
where
    MatchFun: Match<Key>,
    AllocationPolicy: Default,
{
    pub const K_DEFAULT_HASH_MAP_CAPACITY: u32 = 8;

    pub fn new(
        capacity: u32,
        match_fun: MatchFun,
        allocator: AllocationPolicy,
    ) -> TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy> {
        let mut map = TemplateHashMapImpl {
            impl_: Impl::new(match_fun, allocator),
        };
        map.initialize(capacity).unwrap();
        map
    }
    pub fn new_with_default_allocator(
        capacity: u32,
        match_fun: MatchFun,
    ) -> TemplateHashMapImpl<Key, Value, MatchFun, DefaultAllocationPolicy> {
        let allocator = DefaultAllocationPolicy {};
        let mut map = TemplateHashMapImpl {
            impl_: Impl::new(match_fun, allocator),
        };
        map.initialize(capacity).unwrap();
        map
    }

    pub fn clone_from(
        original: &TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>,
        allocator: AllocationPolicy,
    ) -> TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>
    where AllocationPolicy: Clone{
        let mut map = TemplateHashMapImpl {
            impl_: Impl::new(original.impl_.match_fun, allocator),
        };
        map.impl_.capacity_ = original.capacity();
        map.impl_.occupancy_ = original.occupancy();
        map.impl_.map_ = map.impl_.allocator.allocate_array(map.capacity() as usize).unwrap();

        unsafe {
            ptr::copy_nonoverlapping(
                original.impl_.map_,
                map.impl_.map_,
                (map.capacity() * mem::size_of::<TemplateHashMapEntry<Key, Value>>() as u32) as usize,
            );
        }
        map
    }

    pub fn lookup(&self, key: &Key, hash: u32) -> Option<&mut TemplateHashMapEntry<Key, Value>> {
        let entry = self.probe(key, hash);
        if unsafe { entry.as_ref().unwrap().exists() } {
            Some(unsafe { entry.as_mut().unwrap() })
        } else {
            None
        }
    }

    pub fn lookup_or_insert(
        &mut self,
        key: Key,
        hash: u32,
    ) -> Result<&mut TemplateHashMapEntry<Key, Value>, HashMapError>
    where
        Value: Default,
    {
        self.lookup_or_insert_with(key, hash, || Value::default())
    }

    pub fn lookup_or_insert_with<Func>(
        &mut self,
        key: Key,
        hash: u32,
        value_func: Func,
    ) -> Result<&mut TemplateHashMapEntry<Key, Value>, HashMapError>
    where
        Func: FnOnce() -> Value,
    {
        let entry = self.probe(&key, hash);
        if unsafe { entry.as_ref().unwrap().exists() } {
            Ok(unsafe { entry.as_mut().unwrap() })
        } else {
            self.fill_empty_entry(entry, key, value_func(), hash)
        }
    }

    pub fn insert_new(
        &mut self,
        key: Key,
        hash: u32,
    ) -> Result<&mut TemplateHashMapEntry<Key, Value>, HashMapError>
    where
        Value: Default,
    {
        let entry = self.probe(&key, hash);
        self.fill_empty_entry(entry, key, Value::default(), hash)
    }

    pub fn remove(&mut self, key: &Key, hash: u32) -> Option<Value> {
        let p = self.probe(key, hash);
        if !unsafe { p.as_ref().unwrap().exists() } {
            return None;
        }

        let value = unsafe { ptr::read(&(*p).as_mut().unwrap().value) };

        unsafe {
            DCHECK!(self.occupancy() < self.capacity());

            let mut q = p;

            loop {
                q = q.add(1);
                if q == self.map_end() {
                    q = self.impl_.map_;
                }
                if !(*q).exists() {
                    break;
                }

                let r = self.impl_.map_.add(((*q).hash & (self.capacity() - 1)) as usize);

                if (q > p && (r <= p || r > q)) || (q < p && (r <= p && r > q)) {
                    ptr::copy_nonoverlapping(*q, *p, 1);
                    p = q;
                }
            }
            (*p).clear();
        }

        self.impl_.occupancy_ -= 1;
        Some(value)
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity() {
            unsafe {
                self.impl_.map_.add(i as usize).as_mut().unwrap().clear();
            }
        }
        self.impl_.occupancy_ = 0;
    }

    pub fn invalidate(&mut self) {
        DCHECK!(self.impl_.map_ != ptr::null_mut());
        self.impl_
            .allocator
            .delete_array(self.impl_.map_, self.capacity() as usize);
        self.impl_ = Impl::new(self.impl_.match_fun, self.impl_.allocator);
    }

    pub fn occupancy(&self) -> u32 {
        self.impl_.occupancy_
    }

    pub fn capacity(&self) -> u32 {
        self.impl_.capacity_
    }

    pub fn start(&self) -> *mut TemplateHashMapEntry<Key, Value> {
        self.next(unsafe { self.impl_.map_.sub(1) })
    }

    pub fn next(&self, entry: *mut TemplateHashMapEntry<Key, Value>) -> *mut TemplateHashMapEntry<Key, Value> {
        let end = self.map_end();
        DCHECK!(unsafe { self.impl_.map_.sub(1) <= entry && entry < end });

        unsafe {
            let mut current = entry.add(1);
            while current < end {
                if (*current).exists() {
                    return current;
                }
                current = current.add(1);
            }
        }
        ptr::null_mut()
    }

    pub fn allocator(&self) -> &AllocationPolicy {
        &self.impl_.allocator
    }

    fn initialize(&mut self, capacity: u32) -> Result<(), HashMapError> {
        if !bits::is_power_of_two(capacity) {
            return Err(HashMapError::InvalidCapacity);
        }
        self.impl_.map_ = self
            .impl_
            .allocator
            .allocate_array(capacity as usize)?;
        if self.impl_.map_.is_null() {
            return Err(HashMapError::OutOfMemory);
        }
        self.impl_.capacity_ = capacity;
        self.clear();
        Ok(())
    }

    fn resize(&mut self) -> Result<(), HashMapError> {
        let old_map = self.impl_.map_;
        let old_capacity = self.capacity();
        let mut n = self.occupancy();

        self.initialize(self.capacity() * 2)?;

        unsafe {
            let mut entry = old_map;
            while n > 0 {
                if (*entry).exists() {
                    let new_entry = self.probe(&(*entry).key, (*entry).hash);
                    self.fill_empty_entry(new_entry, (*entry).key, (*entry).value, (*entry).hash)?;
                    n -= 1;
                }
                entry = entry.add(1);
            }
        }
        self.impl_
            .allocator
            .delete_array(old_map, old_capacity as usize);
        Ok(())
    }

    fn map_end(&self) -> *mut TemplateHashMapEntry<Key, Value> {
        unsafe { self.impl_.map_.add(self.impl_.capacity_ as usize) }
    }

    fn probe(&self, key: &Key, hash: u32) -> *mut TemplateHashMapEntry<Key, Value>
    where
        MatchFun: Match<Key>,
    {
        DCHECK!(bits::is_power_of_two(self.capacity()));

        let mut i = hash & (self.capacity() - 1);
        DCHECK!(i < self.capacity());

        DCHECK!(self.occupancy() < self.capacity());

        let mut map = self.impl_.map_;
        unsafe {
            while (*map.add(i as usize)).exists()
                && !self.impl_.match_fun.matches(
                    hash,
                    (*map.add(i as usize)).hash,
                    key,
                    &(*map.add(i as usize)).key,
                )
            {
                i = (i + 1) & (self.capacity() - 1);
            }
            map.add(i as usize)
        }
    }

    fn fill_empty_entry(
        &mut self,
        entry: *mut TemplateHashMapEntry<Key, Value>,
        key: Key,
        value: Value,
        hash: u32,
    ) -> Result<&mut TemplateHashMapEntry<Key, Value>, HashMapError> {
        DCHECK!(!unsafe { (*entry).exists() });

        unsafe {
            ptr::write(entry, TemplateHashMapEntry::new(key, value, hash));
        }
        self.impl_.occupancy_ += 1;

        if self.occupancy() + self.occupancy() / 4 >= self.capacity() {
            self.resize()?;
            return Ok(unsafe { &mut *self.probe(&key, hash) });
        }

        Ok(unsafe { &mut *entry })
    }
}

pub struct HashEqualityThenKeyMatcher<Key, MatchFun> {
    match_: MatchFun,
    _phantom: PhantomData<Key>,
}

impl<Key, MatchFun> HashEqualityThenKeyMatcher<Key, MatchFun> {
    pub fn new(match_: MatchFun) -> Self {
        HashEqualityThenKeyMatcher {
            match_,
            _phantom: PhantomData,
        }
    }
}

impl<Key, MatchFun> Match<Key> for HashEqualityThenKeyMatcher<Key, MatchFun>
where
    MatchFun: Fn(&Key, &Key) -> bool,
{
    fn matches(&self, hash1: u32, hash2: u32, key1: &Key, key2: &Key) -> bool {
        hash1 == hash2 && (self.match_)(key1, key2)
    }
}

pub type CustomMatcherTemplateHashMapImpl<AllocationPolicy> =
    TemplateHashMapImpl<
        *mut std::ffi::c_void,
        *mut std::ffi::c_void,
        HashEqualityThenKeyMatcher<*mut std::ffi::c_void, fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> bool>,
        AllocationPolicy,
    >;

impl<AllocationPolicy: Default> CustomMatcherTemplateHashMapImpl<AllocationPolicy> {
    pub type MatchFun = fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> bool;

    pub fn new(
        match_: Self::MatchFun,
        capacity: u32,
        allocator: AllocationPolicy,
    ) -> CustomMatcherTemplateHashMapImpl<AllocationPolicy> {
        TemplateHashMapImpl::new(
            capacity,
            HashEqualityThenKeyMatcher::new(match_),
            allocator,
        )
    }

    pub fn clone_from(
        original: &CustomMatcherTemplateHashMapImpl<AllocationPolicy>,
        allocator: AllocationPolicy,
    ) -> CustomMatcherTemplateHashMapImpl<AllocationPolicy>
    where AllocationPolicy: Clone {
        TemplateHashMapImpl::clone_from(original, allocator)
    }
}

pub type CustomMatcherHashMap = CustomMatcherTemplateHashMapImpl<DefaultAllocationPolicy>;

pub struct KeyEqualityMatcher<Key> {
    _phantom: PhantomData<Key>,
}

impl<Key> KeyEqualityMatcher<Key> {
    pub fn new() -> Self {
        KeyEqualityMatcher {
            _phantom: PhantomData,
        }
    }
}

impl<Key: PartialEq> Match<Key> for KeyEqualityMatcher<Key> {
    fn matches(&self, hash1: u32, hash2: u32, key1: &Key, key2: &Key) -> bool {
        key1 == key2
    }
}

pub type PointerTemplateHashMapImpl<AllocationPolicy> =
    TemplateHashMapImpl<*mut std::ffi::c_void, *mut std::ffi::c_void, KeyEqualityMatcher<*mut std::ffi::c_void>, AllocationPolicy>;

impl<AllocationPolicy: Default> PointerTemplateHashMapImpl<AllocationPolicy> {
    pub fn new(
        capacity: u32,
        allocator: AllocationPolicy,
    ) -> PointerTemplateHashMapImpl<AllocationPolicy> {
        TemplateHashMapImpl::new(capacity, KeyEqualityMatcher::new(), allocator)
    }

    pub fn clone_from(
        other: &PointerTemplateHashMapImpl<AllocationPolicy>,
        allocator: AllocationPolicy,
    ) -> PointerTemplateHashMapImpl<AllocationPolicy>
    where AllocationPolicy: Clone{
        TemplateHashMapImpl::clone_from(other, allocator)
    }
}

pub type HashMap = PointerTemplateHashMapImpl<DefaultAllocationPolicy>;

pub trait Hashable {
    fn hash(&self) -> u32;
}

pub struct TemplateHashMap<Key, Value, MatchFun, AllocationPolicy> {
    base: TemplateHashMapImpl<
        *mut std::ffi::c_void,
        *mut std::ffi::c_void,
        HashEqualityThenKeyMatcher<*mut std::ffi::c_void, MatchFun>,
        AllocationPolicy,
    >,
    _phantom: PhantomData<(Key, Value)>,
}

impl<Key, Value, MatchFun, AllocationPolicy> TemplateHashMap<Key, Value, MatchFun, AllocationPolicy>
where
    Key: Hashable,
    MatchFun: Fn(*mut Key, *mut Key) -> bool,
    AllocationPolicy: Default,
{
    pub fn new(match_: MatchFun, allocator: AllocationPolicy) -> Self {
        TemplateHashMap {
            base: TemplateHashMapImpl::new(
                TemplateHashMapImpl::<
                    *mut std::ffi::c_void,
                    *mut std::ffi::c_void,
                    HashEqualityThenKeyMatcher<
                        *mut std::ffi::c_void,
                        MatchFun,
                    >,
                    AllocationPolicy,
                >::K_DEFAULT_HASH_MAP_CAPACITY,
                HashEqualityThenKeyMatcher::new(match_),
                allocator,
            ),
            _phantom: PhantomData,
        }
    }

    pub fn begin(&self) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
        Iterator {
            map_: self,
            entry_: self.base.start(),
        }
    }

    pub fn end(&self) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
        Iterator {
            map_: self,
            entry_: ptr::null_mut(),
        }
    }

    pub fn find(&mut self, key: &mut Key, insert: bool) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
        let key_ptr = key as *mut Key as *mut std::ffi::c_void;
        if insert {
            let entry = self
                .base
                .lookup_or_insert_with(key_ptr, key.hash(), || ptr::null_mut())
                .unwrap();
            Iterator {
                map_: self,
                entry_: entry as *mut TemplateHashMapEntry<*mut std::ffi::c_void, *mut std::ffi::c_void>,
            }
        } else {
            match self.base.lookup(key_ptr, key.hash()) {
                Some(entry) => Iterator {
                    map_: self,
                    entry_: entry as *mut TemplateHashMapEntry<*mut std::ffi::c_void, *mut std::ffi::c_void>,
                },
                None => Iterator {
                    map_: self,
                    entry_: ptr::null_mut(),
                },
            }
        }
    }
}

pub struct Iterator<'a, Key, Value, MatchFun, AllocationPolicy> {
    map_: &'a TemplateHashMap<Key, Value, MatchFun, AllocationPolicy>,
    entry_: *mut TemplateHashMapEntry<*mut std::ffi::c_void, *mut std::ffi::c_void>,
}

impl<'a, Key, Value, MatchFun, AllocationPolicy> Iterator<'a, Key, Value, MatchFun, AllocationPolicy>
where
    Key: Hashable,
    MatchFun: Fn(*mut Key, *mut Key) -> bool,
    AllocationPolicy: Default,
{
    pub fn next(&mut self) -> &mut Self {
        self.entry_ = self.map_.base.next(self.entry_);
        self
    }

    pub fn get(&mut self) -> Option<ValueType<Key, Value>> {
        if self.entry_.is_null() {
            None
        } else {
            unsafe {
                let key_ptr = (*self.entry_).key as *mut Key;
                let value_ptr = (*self.entry_).value as *mut Value;
                Some(ValueType {
                    first: &mut *key_ptr,
                    second: &mut *value_ptr,
                })
            }
        }
    }

    pub fn ne(&self, other: &Self) -> bool {
        self.entry_ != other.entry_
    }
}

pub struct ValueType<'a, Key, Value> {
    pub first: &'a mut Key,
    pub second: &'a mut Value,
}

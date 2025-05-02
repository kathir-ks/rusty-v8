// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod identity_map {
    use std::{
        any::Any,
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        marker::PhantomData,
        mem::{self, MaybeUninit},
        ptr::{self, NonNull},
    };

    // Forward declarations.
    struct Heap {} // Placeholder
    struct StrongRootsEntry {} // Placeholder

    pub struct IdentityMapFindResult<T> {
        pub entry: *mut T,
        pub already_exists: bool,
    }

    // Base class of identity maps contains shared code for all template
    // instantiations.
    pub struct IdentityMapBase {
        heap_: *mut Heap,
        gc_counter_: i32,
        size_: usize,
        capacity_: usize,
        mask_: usize,
        keys_: *mut *mut u8, // Address type is a raw pointer
        strong_roots_entry_: *mut StrongRootsEntry,
        values_: *mut usize, // uintptr_t equivalent
        is_iterable_: bool,
    }

    impl IdentityMapBase {
        pub fn empty(&self) -> bool {
            self.size_ == 0
        }
        pub fn size(&self) -> usize {
            self.size_
        }
        pub fn capacity(&self) -> usize {
            self.capacity_
        }
        pub fn is_iterable(&self) -> bool {
            self.is_iterable_
        }

        pub fn new(heap: *mut Heap) -> Self {
            IdentityMapBase {
                heap_: heap,
                gc_counter_: -1,
                size_: 0,
                capacity_: 0,
                mask_: 0,
                keys_: ptr::null_mut(),
                strong_roots_entry_: ptr::null_mut(),
                values_: ptr::null_mut(),
                is_iterable_: false,
            }
        }

        // Placeholder since the actual implementation requires
        // memory management and unsafe operations.
        fn find_or_insert_entry(&mut self, key: *mut u8) -> IdentityMapFindResult<usize> {
            IdentityMapFindResult {
                entry: ptr::null_mut(),
                already_exists: false,
            }
        }

        fn find_entry(&self, key: *mut u8) -> *mut usize {
            ptr::null_mut()
        }

        fn insert_entry(&mut self, key: *mut u8) -> *mut usize {
            ptr::null_mut()
        }

        fn delete_entry(&mut self, key: *mut u8, deleted_value: &mut usize) -> bool {
            false
        }

        fn clear(&mut self) {}

        fn key_at_index(&self, index: usize) -> *mut u8 {
            ptr::null_mut()
        }

        fn entry_at_index(&self, index: usize) -> *mut usize {
            ptr::null_mut()
        }

        fn next_index(&self, index: usize) -> usize {
            0
        }

        fn enable_iteration(&mut self) {
            self.is_iterable_ = true;
        }

        fn disable_iteration(&mut self) {
            self.is_iterable_ = false;
        }
    }

    impl Drop for IdentityMapBase {
        fn drop(&mut self) {}
    }

    // Implements an identity map from object addresses to a given value type {V}.
    // The map is robust w.r.t. garbage collection by synchronization with the
    // supplied {heap}.
    //
    //  * Keys are treated as strong roots.
    //  * The value type {V} must be reinterpret_cast'able to {uintptr_t}
    //  * The value type {V} must not be a heap type.
    //
    // Note: IdentityMap methods must not be called during the mark-compact phase
    // since rehashing there may lead to incorrect results.
    // Note: When using IdentityMap in concurrent settings, be aware that reads
    // (e.g. `Find`) may trigger lazy rehashing and thus must be treated as write
    // operations wrt synchronization.
    pub struct IdentityMap<V, A: Allocator> {
        base: IdentityMapBase,
        allocator_: A,
        _phantom: PhantomData<V>,
    }

    impl<V, A: Allocator> IdentityMap<V, A> {
        pub fn new(heap: *mut Heap, allocator: A) -> Self {
            IdentityMap {
                base: IdentityMapBase::new(heap),
                allocator_: allocator,
                _phantom: PhantomData,
            }
        }

        // Searches this map for the given key using the object's address
        // as the identity, returning:
        //    found => a pointer to the storage location for the value, true
        //    not found => a pointer to a new storage location for the value, false
        pub fn find_or_insert(&mut self, key: *mut u8) -> IdentityMapFindResult<V> {
            let raw = self.base.find_or_insert_entry(key);
            IdentityMapFindResult {
                entry: raw.entry as *mut V,
                already_exists: raw.already_exists,
            }
        }

        // Searches this map for the given key using the object's address
        // as the identity, returning:
        //    found => a pointer to the storage location for the value
        //    not found => {nullptr}
        pub fn find(&self, key: *mut u8) -> *mut V {
            self.base.find_entry(key) as *mut V
        }

        // Insert the value for the given key. The key must not have previously
        // existed.
        pub fn insert(&mut self, key: *mut u8, v: V) {
            let entry = self.base.insert_entry(key) as *mut V;
            unsafe {
                *entry = v;
            }
        }

        pub fn delete(&mut self, key: *mut u8, deleted_value: &mut V) -> bool {
            let mut v: usize = 0;
            let deleted_something = self.base.delete_entry(key, &mut v);
            if deleted_something {
                unsafe {
                    *deleted_value = mem::transmute_copy::<usize, V>(&v);
                }
            }
            deleted_something
        }

        // Removes all elements from the map.
        pub fn clear(&mut self) {
            self.base.clear();
        }
    }

    impl<V, A: Allocator> Drop for IdentityMap<V, A> {
        fn drop(&mut self) {
            self.clear();
        }
    }

    // Iterator over IdentityMap. The IteratableScope used to create this Iterator
    // must be live for the duration of the iteration.
    pub struct Iterator<'a, V, A: Allocator> {
        map_: &'a IdentityMap<V, A>,
        index_: usize,
    }

    impl<'a, V, A: Allocator> Iterator<'a, V, A> {
        fn key(&self) -> *mut u8 {
            self.map_.base.key_at_index(self.index_)
        }
        fn entry(&self) -> *mut V {
            self.map_.base.entry_at_index(self.index_) as *mut V
        }
    }

    impl<'a, V, A: Allocator> Iterator<'a, V, A> {
        // pub type Item = *mut V;
    }

    impl<'a, V, A: Allocator> Iterator<'a, V, A> {
        // fn next(&mut self) -> Option<Self::Item> {
        //     self.index_ = self.map_.base.next_index(self.index_);
        //     if self.index_ == self.map_.base.capacity() {
        //         return None;
        //     }
        //     Some(self.entry())
        // }
    }

    // TODO: Implement Iterator trait when needed
    impl<'a, V, A: Allocator> PartialEq for Iterator<'a, V, A> {
        fn eq(&self, other: &Self) -> bool {
            self.index_ == other.index_
        }
    }

    pub struct IteratableScope<'a, V, A: Allocator> {
        map_: &'a mut IdentityMap<V, A>,
    }

    impl<'a, V, A: Allocator> IteratableScope<'a, V, A> {
        pub fn new(map: &'a mut IdentityMap<V, A>) -> Self {
            assert!(!map.base.is_iterable());
            map.base.enable_iteration();
            IteratableScope { map_: map }
        }

        pub fn begin(&self) -> Iterator<'a, V, A> {
            Iterator {
                map_: &self.map_,
                index_: self.map_.base.next_index(usize::MAX), // -1 as usize
            }
        }

        pub fn end(&self) -> Iterator<'a, V, A> {
            Iterator {
                map_: &self.map_,
                index_: self.map_.base.capacity(),
            }
        }
    }

    impl<'a, V, A: Allocator> Drop for IteratableScope<'a, V, A> {
        fn drop(&mut self) {
            assert!(self.map_.base.is_iterable());
            self.map_.base.disable_iteration();
        }
    }

    pub trait Allocator {
        fn allocate_array<T>(&mut self, length: usize) -> *mut T;
        fn deallocate_array<T>(&mut self, array: *mut T, length: usize);
    }

    pub struct DefaultAllocator {}

    impl Allocator for DefaultAllocator {
        fn allocate_array<T>(&mut self, length: usize) -> *mut T {
            let layout = std::alloc::Layout::array::<T>(length).unwrap();
            unsafe { std::alloc::alloc(layout) as *mut T }
        }

        fn deallocate_array<T>(&mut self, array: *mut T, length: usize) {
            let layout = std::alloc::Layout::array::<T>(length).unwrap();
            unsafe { std::alloc::dealloc(array as *mut u8, layout) }
        }
    }
}
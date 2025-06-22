// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::ops::{Index, IndexMut};
use std::ptr;

mod base {
    // Mock base::Vector for now, needs a proper implementation
    #[derive(Debug, Clone)]
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

        pub fn begin(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn end(&self) -> std::slice::Iter<T> {
            self.data.iter().skip(self.data.len())
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }

        pub fn at(&self, index: usize) -> &T {
            &self.data[index]
        }

        pub fn at_mut(&mut self, index: usize) -> &mut T {
            &mut self.data[index]
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
            self.data.iter_mut()
        }

        pub fn clear(&mut self) {
            self.data.clear();
        }
    }

    impl<T> Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T> IndexMut<usize> for Vector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PointerWithPayload<P, T, const BITS: usize> {
        pointer: *mut P,
        payload: u32,
    }

    impl<P, T, const BITS: usize> PointerWithPayload<P, T, BITS> {
        pub fn new(pointer: *mut P, payload: T) -> Self
        where
            T: Into<u32>,
        {
            PointerWithPayload {
                pointer,
                payload: payload.into(),
            }
        }

        pub fn get_pointer(&self) -> *mut P {
            self.pointer
        }

        pub fn get_payload(&self) -> u32 {
            self.payload
        }

        pub fn get_pointer_with_known_payload(&self, _payload: T) -> *mut P
        where
            T: Into<u32>,
        {
            self.pointer
        }

        pub fn raw(&self) -> usize {
            self.pointer as usize | self.payload as usize
        }
    }
}

mod common {
    pub struct AssertScope {}
    impl AssertScope {
        pub fn new() -> Self {
            AssertScope {}
        }
    }
}

mod handles {
    // Mock Handle for now
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Handle<T> {
        address: *mut T,
    }

    impl<T> Handle<T> {
        pub fn new(address: *mut T) -> Self {
            Handle { address }
        }

        pub fn address(&self) -> *mut *mut T {
            &mut self.address
        }

        pub fn location(&self) -> *mut T {
            self.address
        }
    }
}

mod zone {
    use super::base::Vector;

    // Mock Zone for now
    #[derive(Debug)]
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }

        pub fn allocate_array<T>(&self, size: usize) -> *mut T {
            let mut vec = Vec::with_capacity(size);
            let ptr = vec.as_mut_ptr();
            std::mem::forget(vec); // Prevent deallocation
            ptr
        }

        pub fn new_in_zone<T>(&self) -> Box<T>
        where
            T: Default,
        {
            Box::<T>::new(Default::default())
        }

        pub fn new<T>(&self, data: *mut T, size: usize) -> *mut Vector<T> {
            let vec = unsafe { Vec::from_raw_parts(data, size, size) };
            let boxed = Box::new(Vector::new(vec));
            Box::into_raw(boxed)
        }
    }

    impl Default for Zone {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod internal {
    use super::base;
    use super::handles::Handle;
    use super::zone::Zone;
    use std::cmp::Ordering;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;

    pub struct ZoneCompactSetTraits<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> ZoneCompactSetTraits<T> {
        pub fn new() -> Self {
            ZoneCompactSetTraits {
                _phantom: PhantomData,
            }
        }
    }

    impl ZoneCompactSetTraits<Handle<i32>> {
        pub type handle_type = Handle<i32>;
        pub type data_type = *mut i32;

        pub fn handle_to_pointer(handle: Self::handle_type) -> Self::data_type {
            // Use address() instead of location() to get around handle access checks
            // (we're not actually dereferencing the handle so it's safe to read its
            // location)
            unsafe { *(handle.address() as *mut Self::data_type) }
        }

        pub fn pointer_to_handle(ptr: Self::data_type) -> Self::handle_type {
            Handle::new(ptr as *mut i32)
        }
    }

    // A Zone-allocated set which has a compact encoding of zero and one values.
    // Two or more values will be stored as a sorted list, which is copied on write
    // to keep the ZoneCompactSet copy constructor trivial. Note that this means
    // that insertions past the first value will trigger an allocation and copy of
    // the existing elements -- ZoneCompactSet should be preferred for cases where
    // we mostly have only zero or one values.
    //
    // T must be a Handle-like type with a specialization of ZoneCompactSetTraits.
    // In particular, it must be a trivial wrapper of a pointer to actual data --
    // ZoneCompactSet will store this pointer rather than the T type.
    #[derive(Clone)]
    pub struct ZoneCompactSet<T> {
        data_: base::PointerWithPayload<i32, Tag, 2>,
        _phantom: PhantomData<T>,
    }

    #[allow(dead_code)]
    impl<T> ZoneCompactSet<T> {
        const K_SINGLETON_TAG: u32 = 0;
        const K_EMPTY_TAG: u32 = 1;
        const K_LIST_TAG: u32 = 2;

        //TODO add static_assert(std::is_trivially_copyable_v<T>);
        //TODO add static_assert(std::is_trivially_destructible_v<T>);
        type Traits = ZoneCompactSetTraits<T>;
        type handle_type = <Self as ZoneCompactSetTrait>::HandleType;
        type data_type = <Self as ZoneCompactSetTrait>::DataType;
    }

    pub trait ZoneCompactSetTrait {
        type HandleType;
        type DataType;
    }

    impl ZoneCompactSetTrait for ZoneCompactSet<Handle<i32>> {
        type HandleType = Handle<i32>;
        type DataType = *mut i32;
    }

    impl ZoneCompactSet<Handle<i32>> {
        pub fn new() -> Self {
            ZoneCompactSet {
                data_: Self::empty_value(),
                _phantom: PhantomData,
            }
        }

        pub fn from_handle(handle: Handle<i32>) -> Self {
            let data = <Self as ZoneCompactSetTrait>::DataType;
            let pointer = ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);

            ZoneCompactSet {
                data_: base::PointerWithPayload::new(pointer, Tag::Singleton),
                _phantom: PhantomData,
            }
        }

        pub fn from_initializer_list(handles: &[Handle<i32>], zone: &Zone) -> Self {
            Self::from_iter(handles.iter().cloned(), zone)
        }

        pub fn from_iter<It: Iterator<Item = Handle<i32>>>(mut iter: It, zone: &Zone) -> Self {
            let mut data: Vec<Handle<i32>> = Vec::new();
            while let Some(value) = iter.next() {
                data.push(value);
            }
            let size = data.len();

            if size == 0 {
                ZoneCompactSet {
                    data_: Self::empty_value(),
                    _phantom: PhantomData,
                }
            } else if size == 1 {
                let handle = data[0];
                let pointer =
                    ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);

                ZoneCompactSet {
                    data_: base::PointerWithPayload::new(pointer, Tag::Singleton),
                    _phantom: PhantomData,
                }
            } else {
                let list = Self::new_list(size, zone);
                unsafe {
                    for i in 0..size {
                        let handle = data[i];
                        let pointer =
                            ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);
                        (*list).at_mut(i).write(pointer);
                    }
                    (*list).data.sort_unstable_by(|a, b| {
                        a.partial_cmp(b).unwrap_or(Ordering::Equal)
                    });
                }
                ZoneCompactSet {
                    data_: base::PointerWithPayload::new(list as *mut i32, Tag::List),
                    _phantom: PhantomData,
                }
            }
        }

        pub fn clone_into(&self, zone: &Zone) -> Self {
            Self::from_iter(self.begin().cloned(), zone)
        }

        pub fn is_empty(&self) -> bool {
            self.data_.get_payload() == Tag::Empty as u32
        }

        pub fn size(&self) -> usize {
            if self.is_empty() {
                0
            } else if self.is_singleton() {
                1
            } else {
                unsafe { (*self.list()).size() }
            }
        }

        pub fn at(&self, i: usize) -> Handle<i32> {
            debug_assert_ne!(Tag::Empty as u32, self.data_.get_payload());
            if self.is_singleton() {
                debug_assert_eq!(0, i);
                ZoneCompactSetTraits::<Handle<i32>>::pointer_to_handle(self.singleton())
            } else {
                unsafe {
                    ZoneCompactSetTraits::<Handle<i32>>::pointer_to_handle((*self.list()).at(i))
                }
            }
        }

        pub fn insert(&mut self, handle: Handle<i32>, zone: &Zone) {
            let value = ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);
            if self.is_empty() {
                self.data_ = base::PointerWithPayload::new(value, Tag::Singleton);
            } else if self.is_singleton() {
                if self.singleton() == value {
                    return;
                }
                let list = Self::new_list(2, zone);
                unsafe {
                    if self.singleton() < value {
                        (*list)[0] = self.singleton();
                        (*list)[1] = value;
                    } else {
                        (*list)[0] = value;
                        (*list)[1] = self.singleton();
                    }
                }
                self.data_ = base::PointerWithPayload::new(list as *mut i32, Tag::List);
            } else {
                let current_list = self.list();
                let it = unsafe {
                    (*current_list)
                        .data
                        .binary_search_by(|probe| probe.partial_cmp(&value).unwrap())
                };

                if let Ok(index) = it {
                    if unsafe { (*current_list).data[index] } == value {
                        // Already in the list.
                        return;
                    }
                }
                // Otherwise, lower_bound returned the insertion position to keep the list
                // sorted.

                // We need to copy the list to mutate it, so that trivial copies of the
                // data_ pointer don't observe changes to the list.
                // TODO(leszeks): Avoid copying on every insertion by introducing some
                // concept of mutable/immutable/frozen/CoW sets.
                let current_list_size = unsafe { (*current_list).size() };
                let new_list = Self::new_list(current_list_size + 1, zone);

                unsafe {
                    let mut new_list_it = (*new_list).begin();
                    let mut current_list_it = (*current_list).begin();

                    while current_list_it != (*current_list).end() {
                        if (*current_list_it) < value {
                            *new_list_it = *current_list_it;
                            new_list_it = new_list_it.add(1);
                            current_list_it = current_list_it.add(1);
                        } else {
                            break;
                        }
                    }

                    *new_list_it = value;
                    new_list_it = new_list_it.add(1);

                    while current_list_it != (*current_list).end() {
                        *new_list_it = *current_list_it;
                        new_list_it = new_list_it.add(1);
                        current_list_it = current_list_it.add(1);
                    }
                }

                self.data_ = base::PointerWithPayload::new(new_list as *mut i32, Tag::List);
            }
        }

        pub fn union(&mut self, other: &ZoneCompactSet<Handle<i32>>, zone: &Zone) {
            for i in 0..other.size() {
                self.insert(other.at(i), zone);
            }
        }

        pub fn contains_set(&self, other: &ZoneCompactSet<Handle<i32>>) -> bool {
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
                (*list).data.sort_unstable_by(|a, b| {
                    a.partial_cmp(b).unwrap_or(Ordering::Equal)
                });
            }

            if other.is_singleton() {
                let other_singleton = other.singleton();
                unsafe {
                    return (*list)
                        .data
                        .binary_search_by(|probe| probe.partial_cmp(&other_singleton).unwrap())
                        .is_ok();
                }
            }

            unsafe {
                (*other.list()).data.sort_unstable_by(|a, b| {
                    a.partial_cmp(b).unwrap_or(Ordering::Equal)
                });
            }

            // For each element in the `other` list, find the matching element in this
            // list. Since both lists are sorted, each search candidate will be larger
            // than the previous, and each found element will be the lower bound for
            // the search of the next element.
            let mut it = unsafe { (*list).begin() };
            for pointer in unsafe { (*other.list()).data.iter() } {
                while it != unsafe { (*list).end() } {
                    if *it == *pointer {
                        break;
                    } else if *it < *pointer {
                        it = it.add(1);
                    } else {
                        return false;
                    }
                }
                if it == unsafe { (*list).end() } {
                    return false;
                }
            }
            true
        }

        pub fn contains_handle(&self, handle: Handle<i32>) -> bool {
            if self.is_empty() {
                return false;
            }
            let pointer = ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);
            if self.is_singleton() {
                return self.singleton() == pointer;
            }
            let list = self.list();
            unsafe {
                (*list).data.sort_unstable_by(|a, b| {
                    a.partial_cmp(b).unwrap_or(Ordering::Equal)
                });
            }

            unsafe {
                (*list)
                    .data
                    .binary_search_by(|probe| probe.partial_cmp(&pointer).unwrap())
                    .is_ok()
            }
        }

        pub fn remove(&mut self, handle: Handle<i32>, zone: &Zone) {
            if self.is_empty() {
                return;
            }
            let pointer = ZoneCompactSetTraits::<Handle<i32>>::handle_to_pointer(handle);
            if self.is_singleton() {
                if self.singleton() == pointer {
                    self.data_ = Self::empty_value();
                }
                return;
            }

            let current_list = self.list();

            unsafe {
                (*current_list).data.sort_unstable_by(|a, b| {
                    a.partial_cmp(b).unwrap_or(Ordering::Equal)
                });
            }

            let found_it = unsafe {
                (*current_list)
                    .data
                    .binary_search_by(|probe| probe.partial_cmp(&pointer).unwrap())
            };

            if let Err(_err) = found_it {
                // Not in the list.
                return;
            }

            let index = found_it.unwrap();

            // Drop back down to singleton mode if the size will drops to 1 -- this is
            // needed to ensure that comparisons are correct. We never have to drop down
            // from list to zero size.
            let current_list_size = unsafe { (*current_list).size() };
            debug_assert!(current_list_size >= 2);
            if current_list_size == 2 {
                let other_value = unsafe {
                    if index == 0 {
                        (*current_list).at(1)
                    } else {
                        (*current_list).at(0)
                    }
                };

                self.data_ = base::PointerWithPayload::new(other_value, Tag::Singleton);
                return;
            }

            // We need to copy the list to mutate it, so that trivial copies of the
            // data_ pointer don't observe changes to the list.
            let new_list = Self::new_list(current_list_size - 1, zone);

            unsafe {
                let mut new_it = (*new_list).begin();
                let mut current_it = (*current_list).begin();
                for i in 0..current_list_size {
                    if i == index {
                        current_it = current_it.add(1);
                        continue;
                    }
                    *new_it = *current_it;
                    new_it = new_it.add(1);
                    current_it = current_it.add(1);
                }
            }

            self.data_ = base::PointerWithPayload::new(new_list as *mut i32, Tag::List);
        }

        pub fn clear(&mut self) {
            self.data_ = Self::empty_value();
        }

        fn is_singleton(&self) -> bool {
            self.data_.get_payload() == Tag::Singleton as u32
        }

        fn is_list(&self) -> bool {
            self.data_.get_payload() == Tag::List as u32
        }

        fn list(&self) -> *mut base::Vector<*mut i32> {
            debug_assert!(self.is_list());
            self.data_.get_pointer_with_known_payload(Tag::List) as *mut base::Vector<*mut i32>
        }

        fn singleton(&self) -> *mut i32 {
            self.data_.get_pointer_with_known_payload(Tag::Singleton) as *mut i32
        }

        fn new_list(size: usize, zone: &Zone) -> *mut base::Vector<*mut i32> {
            unsafe { zone.new(zone.allocate_array::<*mut i32>(size), size) }
        }

        fn empty_value() -> base::PointerWithPayload<i32, Tag, 2> {
            base::PointerWithPayload::new(ptr::null_mut(), Tag::Empty)
        }

        pub fn begin(&self) -> ConstIterator<Handle<i32>> {
            ConstIterator::new(self, 0)
        }

        pub fn end(&self) -> ConstIterator<Handle<i32>> {
            ConstIterator::new(self, self.size())
        }
    }

    impl PartialEq for ZoneCompactSet<Handle<i32>> {
        fn eq(&self, other: &Self) -> bool {
            if self.data_ == other.data_ {
                return true;
            }
            if self.is_list() && other.is_list() {
                let lhs_list = self.list();
                let rhs_list = other.list();
                unsafe {
                    (*lhs_list)
                        .data
                        .iter()
                        .eq((*rhs_list).data.iter())
                }
            } else {
                false
            }
        }
    }

    impl Eq for ZoneCompactSet<Handle<i32>> {}

    impl Hash for ZoneCompactSet<Handle<i32>> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.data_.raw().hash(state);
        }
    }

    impl fmt::Display for ZoneCompactSet<Handle<i32>> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.at(i))?;
            }
            Ok(())
        }
    }

    impl Index<usize> for ZoneCompactSet<Handle<i32>> {
        type Output = Handle<i32>;

        fn index(&self, i: usize) -> &Self::Output {
            // Implement indexing logic here, potentially using the `at` method
            // This is a placeholder to satisfy the trait requirements
            todo!()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Tag {
        Singleton = 0,
        Empty = 1,
        List = 2,
    }

    impl From<Tag> for u32 {
        fn from(tag: Tag) -> Self {
            match tag {
                Tag::Singleton => 0,
                Tag::Empty => 1,
                Tag::List => 2,
            }
        }
    }

    //Iterator for ZoneCompactSet
    #[derive(Clone)]
    pub struct ConstIterator<'a, T> {
        set_: &'a ZoneCompactSet<T>,
        current_: usize,
    }

    impl<'a, T> ConstIterator<'a, T> {
        fn new(set_: &'a ZoneCompactSet<T>, current_: usize) -> Self {
            ConstIterator { set_, current_ }
        }
    }

    impl<'a> ConstIterator<'a, Handle<i32>> {
        pub fn value(&self) -> Handle<i32> {
            self.set_.at(self.current_)
        }
    }

    impl<'a> Iterator for ConstIterator<'a, Handle<i32>> {
        type Item = Handle<i32>;

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

    pub type ZoneHandleSet = ZoneCompactSet<Handle<i32>>;
}
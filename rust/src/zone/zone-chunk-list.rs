// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr;

// use crate::base::iterator; // Assuming this is a custom iterator
// use crate::common::globals; // Assuming this contains global constants
// use crate::utils::memcopy; // Assuming this is for memory copying
// use crate::zone::zone; // Assuming this is a zone allocator

const V8_ASSUME_ENABLED: bool = true;

macro_rules! v8_assume {
    ($condition:expr) => {
        if V8_ASSUME_ENABLED {
            assert!($condition);
        }
    };
}

// TODO: Implement a Zone allocator or use a standard allocator.
// For now, using Box<[T]> for Chunk storage.

pub struct Zone {
    // Placeholder, needs a proper allocator implementation.
}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }

    pub fn allocate<T>(&self) -> Box<T> {
        Box::new(unsafe { mem::zeroed() })
    }

    pub fn allocate_slice<T>(&self, count: usize) -> Box<[T]> {
        let mut v = Vec::with_capacity(count);
        unsafe {
            v.set_len(count);
        }
        v.into_boxed_slice()
    }
}

pub trait ZoneObject {
    // Placeholder for ZoneObject trait
}

/// A zone-backed hybrid of a vector and a linked list.
///
/// Use it if you need a collection that:
/// * needs to grow indefinitely,
/// * will mostly grow at the back, but may sometimes grow in front as well
///   (preferably in batches),
/// * needs to have very low overhead,
/// * offers forward- and backwards-iteration,
/// * offers relatively fast seeking,
/// * offers bidirectional iterators,
/// * can be rewound without freeing the backing store,
/// * can be split and joined again efficiently.
///
/// This list will maintain a doubly-linked list of chunks.  When a chunk is
/// filled up, a new one gets appended.  New chunks appended at the end will
/// grow in size up to a certain limit to avoid over-allocation and to keep
/// the zone clean. Chunks may be partially filled. In particular, chunks may
/// be empty after rewinding, such that they can be reused when inserting
/// again at a later point in time.
pub struct ZoneChunkList<T> {
    zone_: Box<Zone>,
    size_: usize,
    front_: Option<Box<Chunk<T>>>,
    last_nonempty_: Option<*mut Chunk<T>>, // Raw pointer because of self-referential struct
}

impl<T> ZoneObject for ZoneChunkList<T> {}

impl<T> ZoneChunkList<T> {
    pub const K_INITIAL_CHUNK_CAPACITY: u32 = 8;
    pub const K_MAX_CHUNK_CAPACITY: u32 = 256;

    pub fn new(zone: Box<Zone>) -> Self {
        ZoneChunkList {
            zone_: zone,
            size_: 0,
            front_: None,
            last_nonempty_: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size_
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn front(&self) -> Option<&T> {
        self.begin().map(|it| it.deref())
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.begin_mut().map(|it| it.deref_mut())
    }

    pub fn back(&self) -> Option<&T> {
        self.rbegin().map(|it| it.deref())
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.rbegin_mut().map(|it| it.deref_mut())
    }

    pub fn push_back(&mut self, item: T) {
        if self.last_nonempty_.is_none() {
            // Initially empty chunk list.
            let mut new_chunk = self.new_chunk(Self::K_INITIAL_CHUNK_CAPACITY);
            let new_chunk_ptr = Box::into_raw(new_chunk);
            self.front_ = Some(unsafe { Box::from_raw(new_chunk_ptr) });
            self.last_nonempty_ = Some(new_chunk_ptr);
        } else {
            let last_nonempty_ptr = self.last_nonempty_.unwrap();
            let last_nonempty = unsafe { &mut *last_nonempty_ptr };

            if last_nonempty.is_full() {
                // If there is an empty chunk following, reuse that, otherwise allocate.
                if last_nonempty.next_.is_none() {
                    let mut chunk = self.new_chunk(Self::next_chunk_capacity(last_nonempty.capacity_));
                    last_nonempty.next_ = Some(Box::into_raw(chunk));
                    let new_chunk_ptr = last_nonempty.next_.unwrap();
                    unsafe {
                        (&mut *new_chunk_ptr).previous_ = Some(last_nonempty_ptr);
                    }
                }
                self.last_nonempty_ = last_nonempty.next_;
                v8_assume!(unsafe { !(&mut *self.last_nonempty_.unwrap()).is_full() });
            }
        }

        let last_nonempty_ptr = self.last_nonempty_.unwrap();
        let last_nonempty = unsafe { &mut *last_nonempty_ptr };

        last_nonempty.items_[last_nonempty.position_ as usize] = item;
        last_nonempty.position_ += 1;
        self.size_ += 1;
        v8_assume!(last_nonempty.position_ <= last_nonempty.capacity_);
    }

    pub fn push_front(&mut self, item: T) {
        if self.front_.is_none() {
            // Initially empty chunk list.
            let mut new_chunk = self.new_chunk(Self::K_INITIAL_CHUNK_CAPACITY);
            let new_chunk_ptr = Box::into_raw(new_chunk);
            self.front_ = Some(unsafe { Box::from_raw(new_chunk_ptr) });
            self.last_nonempty_ = Some(new_chunk_ptr);
        } else {
            let mut front = self.front_.take().unwrap();
            if front.is_full() {
                // First chunk at capacity, so prepend a new chunk.
                v8_assume!(front.previous_.is_none());
                let mut chunk = self.new_chunk(Self::next_chunk_capacity(front.capacity_));
                let chunk_ptr = Box::into_raw(chunk);

                front.previous_ = Some(chunk_ptr);
                unsafe {
                    (&mut *chunk_ptr).next_ = Some(Box::into_raw(front));
                }
                self.front_ = Some(unsafe { Box::from_raw(chunk_ptr) });
            } else {
                self.front_ = Some(front);
            }
        }
        let mut front = self.front_.take().unwrap();
        v8_assume!(!front.is_full());

        // Assuming T is Copy or Clone
        for i in (0..front.position_ as usize).rev() {
            front.items_[i + 1] = front.items_[i].clone();
        }

        front.items_[0] = item;
        front.position_ += 1;
        self.size_ += 1;
        v8_assume!(front.position_ <= front.capacity_);
        self.front_ = Some(front);
    }

    pub fn rewind(&mut self, limit: usize) {
        if limit >= self.size() {
            return;
        }

        let seek_result = self.seek_index(limit);
        v8_assume!(seek_result.chunk_.is_some());

        // Do a partial rewind of the chunk containing the index.

        let chunk_ptr = seek_result.chunk_.unwrap();
        let chunk = unsafe { &mut *chunk_ptr };
        chunk.position_ = seek_result.chunk_index_ as u32;

        // Set last_nonempty_ so iterators will work correctly.
        self.last_nonempty_ = seek_result.chunk_;

        // Do full rewind of all subsequent chunks.
        let mut current = chunk.next_;
        while let Some(current_ptr) = current {
            let current_chunk = unsafe { &mut *current_ptr };
            current_chunk.position_ = 0;
            current = current_chunk.next_;
        }

        self.size_ = limit;

        #[cfg(debug_assertions)]
        self.verify();
    }

    pub fn find(&mut self, index: usize) -> Option<ZoneChunkListIterator<'_, T, true>> {
        let seek_result = self.seek_index(index);
        seek_result.chunk_.map(|chunk| ZoneChunkListIterator::new(chunk, seek_result.chunk_index_ as u32))
    }

    pub fn find_const(&self, index: usize) -> Option<ZoneChunkListIterator<'_, T, false>> {
        let seek_result = self.seek_index(index);
        seek_result.chunk_.map(|chunk| ZoneChunkListIterator::new(chunk as *const Chunk<T>, seek_result.chunk_index_ as u32))
    }

    pub fn split_at(&mut self, split_begin: ZoneChunkListIterator<'_, T, true>) -> ZoneChunkList<T> {
        let mut result = ZoneChunkList::new(Box::new(Zone::new())); // Assuming Zone is cloneable

        if split_begin == self.end_mut().unwrap_or(ZoneChunkListIterator::null()) {
            return result;
        }

        if split_begin == self.begin_mut().unwrap_or(ZoneChunkListIterator::null()) {
            std::mem::swap(self, &mut result);
            return result;
        }

        let split_chunk_ptr = split_begin.current_;
        let split_chunk = unsafe { &mut *split_chunk_ptr };
        v8_assume!(split_begin.position_ <= split_chunk.position_);

        let new_chunk_size = (split_chunk.position_ - split_begin.position_) as usize;
        let new_chunk_capacity = cmp::max(
            Self::K_INITIAL_CHUNK_CAPACITY as usize,
            new_chunk_size.next_power_of_two(),
        ) as u32;
        v8_assume!(new_chunk_size <= new_chunk_capacity as usize);

        let mut new_chunk = self.new_chunk(new_chunk_capacity);
        for i in 0..new_chunk_size {
            new_chunk.items_[i] = split_chunk.items_[(split_begin.position_ as usize) + i].clone();
        }
        new_chunk.position_ = new_chunk_size as u32;
        split_chunk.position_ = split_begin.position_;

        result.front_ = Some(new_chunk);
        let new_chunk_ptr = unsafe {self.front_.as_mut().unwrap() as *mut Chunk<T>};
        result.last_nonempty_ = if self.last_nonempty_ == Some(split_chunk_ptr) {
            Some(new_chunk_ptr)
        } else {
            self.last_nonempty_
        };

        let new_chunk = result.front_.take().unwrap();
        let new_chunk_ptr = Box::into_raw(new_chunk);
        unsafe {
            (&mut *new_chunk_ptr).next_ = split_chunk.next_;
        }
        if let Some(next_ptr) = split_chunk.next_ {
            unsafe {
                (&mut *next_ptr).previous_ = Some(new_chunk_ptr);
            }
        }
        result.front_ = Some(unsafe { Box::from_raw(new_chunk_ptr) });

        self.last_nonempty_ = Some(split_chunk_ptr);
        split_chunk.next_ = None;

        let mut new_size = 0;
        let mut current = self.front_.as_deref();
        let split_chunk_addr = split_chunk as *mut Chunk<T>;

        while let Some(chunk) = current {
            if chunk as *const Chunk<T> == split_chunk_addr {
                break;
            }

            v8_assume!(!chunk.is_empty());
            new_size += chunk.position_ as usize;
            current = chunk.next_.as_ref().map(|ptr| unsafe { &*(*ptr as *const Chunk<T>)});
        }
        new_size += split_chunk.position_ as usize;

        v8_assume!(new_size < self.size());
        result.size_ = self.size() - new_size;
        self.size_ = new_size;

        #[cfg(debug_assertions)]
        self.verify();
        #[cfg(debug_assertions)]
        result.verify();

        return result;
    }

    pub fn append(&mut self, other: &mut ZoneChunkList<T>) {
        v8_assume!(*self.zone_ as *const Zone == *other.zone_ as *const Zone); // Pointer comparison, assuming zones are unique instances.

        if other.front_.is_none() {
            return;
        }

        if let Some(last_nonempty_ptr) = self.last_nonempty_ {
            let last_nonempty = unsafe { &mut *last_nonempty_ptr };
            let other_front = other.front_.take().unwrap();
            let other_front_ptr = Box::into_raw(other_front);
            last_nonempty.next_ = Some(other_front_ptr);

            let other_front_ptr_ref = last_nonempty.next_.unwrap();
            unsafe {
                (&mut *other_front_ptr_ref).previous_ = Some(last_nonempty_ptr);
            }
        } else {
            self.front_ = other.front_.take();
        }

        self.last_nonempty_ = other.last_nonempty_;

        self.size_ += other.size_;

        #[cfg(debug_assertions)]
        self.verify();

        // Leave `other` in empty, but valid state.
        other.front_ = None;
        other.last_nonempty_ = None;
        other.size_ = 0;
    }

    pub fn copy_to(&self, ptr: *mut T) {
        let mut current = self.front_.as_ref();
        let mut current_ptr = ptr;

        while let Some(chunk) = current {
            unsafe {
                ptr::copy_nonoverlapping(
                    chunk.items_.as_ptr(),
                    current_ptr,
                    chunk.position_ as usize,
                );
                current_ptr = current_ptr.add(chunk.position_ as usize);
            }
            current = chunk.next_.as_ref().map(|ptr| unsafe { &*(*ptr as *const Chunk<T>)});
        }
    }

    pub fn begin(&self) -> Option<ZoneChunkListIterator<'_, T, false>> {
        if self.front_.is_none() {
            return None;
        }
        Some(ZoneChunkListIterator::<'_, T, false>::new(unsafe{self.front_.as_ref().unwrap() as *const Chunk<T>}, 0))
    }

    pub fn begin_mut(&mut self) -> Option<ZoneChunkListIterator<'_, T, true>> {
        if self.front_.is_none() {
            return None;
        }
        unsafe {
            Some(ZoneChunkListIterator::<'_, T, true>::new(self.front_.as_mut().unwrap() as *mut Chunk<T>, 0))
        }
    }

    pub fn end(&self) -> Option<ZoneChunkListIterator<'_, T, false>> {
        if self.is_empty() {
            return self.begin();
        }

        if let Some(last_nonempty_ptr) = self.last_nonempty_ {
            let last_nonempty = unsafe { &*last_nonempty_ptr };
            return last_nonempty.next_.map(|ptr| ZoneChunkListIterator::<'_, T, false>::new(ptr, 0));
        }

        None
    }

    pub fn end_mut(&mut self) -> Option<ZoneChunkListIterator<'_, T, true>> {
        if self.is_empty() {
            return self.begin_mut();
        }

        if let Some(last_nonempty_ptr) = self.last_nonempty_ {
            let last_nonempty = unsafe { &mut *last_nonempty_ptr };
            return last_nonempty.next_.map(|ptr| unsafe { ZoneChunkListIterator::<'_, T, true>::new(ptr, 0)});
        }

        None
    }

    pub fn rbegin(&self) -> Option<ZoneChunkListIterator<'_, T, false>> {
        if self.is_empty() {
            return None;
        }

        if let Some(last_nonempty_ptr) = self.last_nonempty_ {
            let last_nonempty = unsafe { &*last_nonempty_ptr };
            v8_assume!(!last_nonempty.is_empty());
            return Some(ZoneChunkListIterator::<'_, T, false>::new(last_nonempty_ptr as *const Chunk<T>, last_nonempty.position_ - 1));
        }

        None
    }

    pub fn rbegin_mut(&mut self) -> Option<ZoneChunkListIterator<'_, T, true>> {
        if self.is_empty() {
            return None;
        }

        if let Some(last_nonempty_ptr) = self.last_nonempty_ {
            let last_nonempty = unsafe { &mut *last_nonempty_ptr };
            v8_assume!(!last_nonempty.is_empty());
            return Some(unsafe { ZoneChunkListIterator::<'_, T, true>::new(last_nonempty_ptr, last_nonempty.position_ - 1)});
        }

        None
    }

    pub fn rend(&self) -> Option<ZoneChunkListIterator<'_, T, false>> {
        return self.begin();
    }

    pub fn rend_mut(&mut self) -> Option<ZoneChunkListIterator<'_, T, true>> {
        return self.begin_mut();
    }

    pub fn swap(&mut self, other: &mut ZoneChunkList<T>) {
        v8_assume!(*self.zone_ as *const Zone == *other.zone_ as *const Zone);

        std::mem::swap(&mut self.size_, &mut other.size_);
        std::mem::swap(&mut self.front_, &mut other.front_);
        std::mem::swap(&mut self.last_nonempty_, &mut other.last_nonempty_);
    }

    fn new_chunk(&self, capacity: u32) -> Box<Chunk<T>> {
        let items: Box<[T]> = self.zone_.allocate_slice(capacity as usize);

        Box::new(Chunk {
            capacity_: capacity,
            position_: 0,
            next_: None,
            previous_: None,
            items_: items,
        })
    }

    fn next_chunk_capacity(previous_capacity: u32) -> u32 {
        cmp::min(previous_capacity * 2, Self::K_MAX_CHUNK_CAPACITY)
    }

    fn seek_index(&self, index: usize) -> SeekResult<T> {
        v8_assume!(index < self.size());

        let mut current_option = self.front_.as_ref();
        let mut current: *mut Chunk<T>;

        unsafe {
            current = self.front_.as_mut().map_or(ptr::null_mut(), |b| b as *mut Chunk<T>);
        }

        let mut mutable_current: *mut Chunk<T> = match self.front_ {
            Some(_) => {current},
            None => {panic!("Mutable Chunk pointer cannot be created")}
        };
        
        let mut mutable_index = index;
        let front_ref = self.front_.as_ref();

        while let Some(chunk) = front_ref {
             if mutable_index >= chunk.capacity_ as usize {
                 mutable_index -= chunk.capacity_ as usize;
                 if let Some(mut next_ptr) = unsafe { (&mut *mutable_current).next_ } {
                     unsafe {
                         mutable_current = next_ptr;
                     }
                 } else {
                     break;
                 }
             } else {
                 break;
             }
        }

        v8_assume!(mutable_index < unsafe { (&mut *mutable_current).capacity_ } as usize);
        let current_some: Option<*mut Chunk<T>> = Some(mutable_current);
        SeekResult {
            chunk_: current_some,
            chunk_index_: mutable_index as u32,
        }
    }

    #[cfg(debug_assertions)]
    fn verify(&self) {
        if self.front_.is_none() {
            // Initial empty state.
            v8_assume!(self.last_nonempty_.is_none());
            v8_assume!(self.size() == 0);
        } else if self.is_empty() {
            // Special case: Fully rewound list, with only empty chunks.
            let front_ptr = unsafe { self.front_.as_ref().unwrap() as *const Chunk<T> };
            let last_nonempty_ptr = match self.last_nonempty_ {
                Some(ptr) => ptr as *const Chunk<T>,
                None => ptr::null() as *const Chunk<T>,
            };
            v8_assume!(front_ptr == last_nonempty_ptr);
            v8_assume!(self.size() == 0);
            let mut chunk = self.front_.as_ref();
            while let Some(c) = chunk {
                v8_assume!(c.is_empty());
                chunk = c.next_.as_ref().map(|ptr| unsafe { &*(*ptr as *const Chunk<T>) });
            }
        } else {
            // Normal state: Somewhat filled and (partially) rewound.
            v8_assume!(self.last_nonempty_.is_some());

            let mut size_check = 0;
            let mut in_empty_tail = false;
            let mut chunk = self.front_.as_ref();

            while let Some(c) = chunk {
                let c_ptr = c as *const Chunk<T>;
                let last_nonempty_ptr = match self.last_nonempty_ {
                    Some(ptr) => ptr as *const Chunk<T>,
                    None => ptr::null() as *const Chunk<T>,
                };
                // Chunks from `front_` to `last_nonempty_` (inclusive) are non-empty.
                v8_assume!(in_empty_tail == c.is_empty());
                size_check += c.position_ as usize;

                if c_ptr == last_nonempty_ptr {
                    in_empty_tail = true;
                }

                chunk = c.next_.as_ref().map(|ptr| unsafe { &*(*ptr as *const Chunk<T>) });
            }
            v8_assume!(size_check == self.size());
        }
    }
}

impl<T> Drop for ZoneChunkList<T> {
    fn drop(&mut self) {
        // Manually deallocate all Chunks in the list.
        let mut current = self.front_.take();
        while let Some(mut chunk) = current {
            current = chunk.next_.take().map(|ptr| unsafe { Box::from_raw(ptr) });
        }
    }
}

#[derive(Debug)]
struct Chunk<T> {
    capacity_: u32,
    position_: u32,
    next_: Option<*mut Chunk<T>>,
    previous_: Option<*mut Chunk<T>>,
    items_: Box<[T]>, // Using Box<[T]> for allocation in Zone
}

impl<T> Chunk<T> {
    fn size(&self) -> u32 {
        v8_assume!(self.position_ <= self.capacity_);
        self.position_
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn is_full(&self) -> bool {
        self.size() == self.capacity_
    }
}

impl<T> Index<usize> for Chunk<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items_[index]
    }
}

impl<T> IndexMut<usize> for Chunk<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items_[index]
    }
}

struct SeekResult<T> {
    chunk_: Option<*mut Chunk<T>>,
    chunk_index_: u32,
}

#[derive(Clone, Copy)]
pub struct ZoneChunkListIterator<'a, T, const MODIFIABLE: bool> {
    current_: *const Chunk<T>,
    position_: u32,
    phantom_: PhantomData<(&'a T, MODIFIABLE)>,
}

impl<'a, T, const MODIFIABLE: bool> ZoneChunkListIterator<'a, T, MODIFIABLE> {
    fn new(current: *const Chunk<T>, position: u32) -> Self {
        unsafe {
            if !current.is_null() {
                assert!(position < (&*current).capacity_);
            }
        }

        ZoneChunkListIterator {
            current_: current,
            position_: position,
            phantom_: PhantomData,
        }
    }

    fn null() -> Self {
        ZoneChunkListIterator {
            current_: ptr::null(),
            position_: 0,
            phantom_: PhantomData,
        }
    }

    pub fn advance(&mut self, amount: u32) {
        assert!(!MODIFIABLE, "Advance only works on forward iterators");

        let mut position = self.position_;
        if !amount.checked_add(position).is_some() {
            panic!("UnsignedAddOverflow32");
        }
        let mut result = self.position_ + amount;

        while result > 0 && result >= unsafe { (&*self.current_).position_ } {
            let overshoot = result - unsafe { (&*self.current_).position_ };
            unsafe {
                self.current_ = (&*self.current_).next_.map_or(ptr::null(), |ptr| ptr);
            }
            result = overshoot;
            assert!(result == 0 || !self.current_.is_null());
        }

        self.position_ = result;
    }
}

impl<'a, T, const MODIFIABLE: bool> PartialEq for ZoneChunkListIterator<'a, T, MODIFIABLE> {
    fn eq(&self, other: &Self) -> bool {
        self.current_ == other.current_ && self.position_ == other.position_
    }
}

impl<'a, T, const MODIFIABLE: bool> Eq for ZoneChunkListIterator<'a, T, MODIFIABLE> {}

impl<'a, T> Deref for ZoneChunkListIterator<'a, T, true> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let chunk = &*self.current_;
            &chunk.items_[self.position_ as usize]
        }
    }
}

impl<'a, T> DerefMut for ZoneChunkListIterator<'a, T, true> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let chunk = &mut *(self.current_ as *mut Chunk<T>);
            &mut chunk.items_[self.position_ as usize]
        }
    }
}

impl<'a, T> Deref for ZoneChunkListIterator<'a, T, false> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let chunk = &*self.current_;
            &chunk.items_[self.position_ as usize]
        }
    }
}

impl<'a, T> ZoneChunkListIterator<'a, T, true> {
    pub fn next(&mut self) -> Option<&mut T> {
        if self.current_.is_null() {
            return None;
        }

        let chunk = unsafe { &mut *(self.current_ as *mut Chunk<T>) };

        self.position_ += 1;
        if self.position_ >= chunk.position_ {
            self.current_ = chunk.next_.map_or(ptr::null(), |ptr| ptr);
            self.position_ = 0;
        }

        if self.current_.is_null() {
            return None;
        }

        let chunk = unsafe { &mut *(self.current_ as *mut Chunk<T>) };
        Some(&mut chunk.items_[self.position_ as usize])
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        if self.current_.is_null() {
            return None;
        }

        if self.position_ == 0 {
            let chunk = unsafe { &mut *(self.current_ as *mut Chunk<T>) };
            self.current_ = chunk.previous_.map_or(ptr::null(), |ptr| ptr);
            if self.current_.is_null() {
                return None;
            }
            let chunk = unsafe { &mut *(self.current_ as *mut Chunk<T>) };
            self.position_ = chunk.position_ - 1;
        } else {
            self.position_ -= 1;
        }

        let chunk = unsafe { &mut *(self.current_ as *mut Chunk<T>) };
        Some(&mut chunk.items_[self.position_ as usize])
    }
}

impl<'a, T> ZoneChunkListIterator<'a, T, false> {
    pub fn next(&mut self) -> Option<&T> {
        if self.current_.is_null() {
            return None;
        }

        let chunk = unsafe { &*self.current_ };

        self.position_ += 1;
        if self.position_ >= chunk.position_ {
            self.current_ = chunk.next_.map_or(ptr::null(), |ptr| ptr);
            self.position_ = 0;
        }

        if self.current_.is_null() {
            return None;
        }

        let chunk = unsafe { &*self.current_ };
        Some(&chunk.items_[self.position_ as usize])
    }

    pub fn prev(&mut self) -> Option<&T> {
        if self.current_.is_null() {
            return None;
        }

        if self.position_ == 0 {
            let chunk = unsafe { &*self.current_ };
            self.current_ = chunk.previous_.map_or(ptr::null(), |ptr| ptr);
            if self.current_.is_null() {
                return None;
            }
            let chunk = unsafe { &*self.current_ };
            self.position_ = chunk.position_ - 1;
        } else {
            self.position_ -= 1;
        }

        let chunk = unsafe { &*self.current_ };
        Some(&chunk.items_[self.position_ as usize])
    }
}
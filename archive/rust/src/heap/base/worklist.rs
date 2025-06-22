// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicSize, Ordering};
use std::sync::{Mutex, MutexGuard};

mod base {
    pub(crate) mod internal {
        use std::sync::atomic::{AtomicU16, Ordering};

        pub struct SegmentBase {
            capacity_: u16,
            index_: AtomicU16,
        }

        impl SegmentBase {
            // TODO: Implement GetSentinelSegmentAddress.  Requires static mut and unsafe.
            // pub static fn GetSentinelSegmentAddress() -> *mut SegmentBase {
            //     unimplemented!()
            // }

            pub const fn new(capacity: u16) -> Self {
                Self {
                    capacity_: capacity,
                    index_: AtomicU16::new(0),
                }
            }

            #[inline]
            pub fn size(&self) -> usize {
                self.index_.load(Ordering::Relaxed) as usize
            }

            #[inline]
            pub fn capacity(&self) -> usize {
                self.capacity_ as usize
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.size() == 0
            }

            #[inline]
            pub fn is_full(&self) -> bool {
                self.size() == self.capacity()
            }

            pub fn clear(&self) {
                self.index_.store(0, Ordering::Relaxed);
            }
        }
    }

    pub struct WorklistBase {
        predictable_order_: bool,
    }

    impl WorklistBase {
        pub fn enforce_predictable_order(&mut self) {
            self.predictable_order_ = true;
        }
        pub fn predictable_order(&self) -> bool {
            self.predictable_order_
        }
    }

    impl Default for WorklistBase {
        fn default() -> Self {
            WorklistBase {
                predictable_order_: false,
            }
        }
    }

    // A global worklist based on segments which allows for a thread-local
    // producer/consumer pattern with global work stealing.
    //
    // - Entries in the worklist are of type `EntryType`.
    // - Segments have a capacity of at least `MinSegmentSize` but possibly more.
    //
    // All methods on the worklist itself are safe for concurrent usage but only
    // consider published segments. Unpublished work in views using `Local` is not
    // visible.
    pub struct Worklist<EntryType, const MinSegmentSize: u16> {
        lock_: Mutex<()>,
        top_: AtomicPtr<Segment<EntryType, MinSegmentSize>>,
        size_: AtomicSize,
    }

    impl<EntryType, const MinSegmentSize: u16> Worklist<EntryType, const MinSegmentSize> {
        pub const K_MIN_SEGMENT_SIZE: u16 = MinSegmentSize;

        pub fn new() -> Self {
            Self {
                lock_: Mutex::new(()),
                top_: AtomicPtr::new(std::ptr::null_mut()),
                size_: AtomicSize::new(0),
            }
        }

        // Returns true if the global worklist is empty and false otherwise. May be
        // read concurrently for an approximation.
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        // Returns the number of segments in the global worklist. May be read
        // concurrently for an approximation.
        pub fn size(&self) -> usize {
            // It is safe to read |size_| without a lock since this variable is
            // atomic, keeping in mind that threads may not immediately see the new
            // value when it is updated.
            self.size_.load(Ordering::Relaxed)
        }

        // Moves the segments from `other` into this worklist, leaving behind `other`
        // as empty.
        pub fn merge(&self, other: &Self) {
            let mut other_top;
            let mut other_size;
            {
                let _guard = other.lock_.lock().unwrap();
                let other_top_ptr = other.top_.load(Ordering::Relaxed);

                if other_top_ptr.is_null() {
                    return;
                }
                other_top = unsafe { &*other_top_ptr };
                other.top_.store(std::ptr::null_mut(), Ordering::Relaxed);
                other_size = other.size_.swap(0, Ordering::Relaxed);
            }

            // It's safe to iterate through these segments because the top was
            // extracted from `other`.
            let mut end = other_top;
            while !end.next_.load(Ordering::Relaxed).is_null() {
                end = unsafe { &*end.next_.load(Ordering::Relaxed) };
            }

            {
                let _guard = self.lock_.lock().unwrap();
                self.size_.fetch_add(other_size, Ordering::Relaxed);
                end.next_.store(self.top_.load(Ordering::Relaxed), Ordering::Relaxed);
                self.top_.store(other_top as *const _ as *mut _, Ordering::Relaxed);
            }
        }

        // Removes all segments from the worklist.
        pub fn clear(&self) {
            let _guard = self.lock_.lock().unwrap();
            self.size_.store(0, Ordering::Relaxed);
            let mut current = self.top_.load(Ordering::Relaxed);
            while !current.is_null() {
                let tmp = current;
                current = unsafe { (*current).next_.load(Ordering::Relaxed) };
                unsafe {
                    Segment::<EntryType, MinSegmentSize>::delete(tmp);
                }
            }
            self.top_.store(std::ptr::null_mut(), Ordering::Relaxed);
        }

        // Invokes `callback` on each item. Callback is of type `bool(EntryType&)` and
        // should return true if the entry should be kept and false if the entry
        // should be removed.
        pub fn update<Callback>(&self, callback: Callback)
        where
            Callback: Fn(&mut EntryType) -> bool,
        {
            let _guard = self.lock_.lock().unwrap();
            let mut prev: *mut Segment<EntryType, MinSegmentSize> = std::ptr::null_mut();
            let mut current = self.top_.load(Ordering::Relaxed);
            let mut num_deleted: usize = 0;

            while !current.is_null() {
                unsafe {
                    (*current).update(&callback);
                }

                if unsafe { (*current).is_empty() } {
                    self.size_.fetch_sub(1, Ordering::Relaxed);
                    num_deleted += 1;

                    if prev.is_null() {
                        unsafe {
                            self.top_.store((*current).next_.load(Ordering::Relaxed), Ordering::Relaxed);
                        }
                    } else {
                        unsafe {
                            (*prev).next_.store((*current).next_.load(Ordering::Relaxed), Ordering::Relaxed);
                        }
                    }

                    let tmp = current;
                    current = unsafe { (*current).next_.load(Ordering::Relaxed) };
                    unsafe {
                        Segment::<EntryType, MinSegmentSize>::delete(tmp);
                    }
                } else {
                    prev = current;
                    current = unsafe { (*current).next_.load(Ordering::Relaxed) };
                }
            }
            self.size_.fetch_sub(num_deleted, Ordering::Relaxed);
        }

        // Invokes `callback` on each item. Callback is of type `void(EntryType&)`.
        pub fn iterate<Callback>(&self, callback: Callback)
        where
            Callback: Fn(&EntryType),
        {
            let _guard = self.lock_.lock().unwrap();
            let mut current = self.top_.load(Ordering::Relaxed);
            while !current.is_null() {
                unsafe {
                    (*current).iterate(&callback);
                }
                current = unsafe { (*current).next_.load(Ordering::Relaxed) };
            }
        }

        fn push(&self, segment: *mut Segment<EntryType, MinSegmentSize>) {
            let _guard = self.lock_.lock().unwrap();
            unsafe {
                (*segment).next_.store(self.top_.load(Ordering::Relaxed), Ordering::Relaxed);
            }
            self.top_.store(segment, Ordering::Relaxed);
            self.size_.fetch_add(1, Ordering::Relaxed);
        }

        fn pop(&self, segment: &mut *mut Segment<EntryType, MinSegmentSize>) -> bool {
            let _guard = self.lock_.lock().unwrap();
            let top_ptr = self.top_.load(Ordering::Relaxed);

            if top_ptr.is_null() {
                return false;
            }
            self.size_.fetch_sub(1, Ordering::Relaxed);
            *segment = top_ptr;
            unsafe {
                self.top_.store((*top_ptr).next_.load(Ordering::Relaxed), Ordering::Relaxed);
            }
            true
        }
    }

    impl<EntryType, const MinSegmentSize: u16> Drop for Worklist<EntryType, const MinSegmentSize> {
        fn drop(&mut self) {
            assert!(self.is_empty());
        }
    }

    pub struct Segment<EntryType, const MinSegmentSize: u16> {
        base_: internal::SegmentBase,
        next_: AtomicPtr<Segment<EntryType, MinSegmentSize>>,
        entries_: UnsafeCell<[MaybeUninit<EntryType>]>, // Dynamically sized array
    }

    impl<EntryType, const MinSegmentSize: u16> Segment<EntryType, const MinSegmentSize> {
        pub fn create(min_segment_size: u16) -> *mut Self {
            let wanted_bytes = Self::malloc_size_for_capacity(min_segment_size as usize);

            // TODO: add predictable order to the allocate at least
            let layout = std::alloc::Layout::from_size_align(wanted_bytes, std::mem::align_of::<Self>()).unwrap();
            unsafe {
                let ptr = std::alloc::alloc(layout) as *mut Segment<EntryType, MinSegmentSize>;

                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }

                let capacity = Self::capacity_for_malloc_size(layout.size());
                ptr.write(Segment {
                    base_: internal::SegmentBase::new(capacity as u16),
                    next_: AtomicPtr::new(std::ptr::null_mut()),
                    entries_: UnsafeCell::new(MaybeUninit::uninit_array()),
                });
                ptr
            }
        }

        pub unsafe fn delete(segment: *mut Self) {
            let layout = std::alloc::Layout::from_size_align(Self::malloc_size_for_capacity((*segment).base_.capacity() as usize), std::mem::align_of::<Self>()).unwrap();

            std::alloc::dealloc(segment as *mut u8, layout);
        }

        #[inline]
        pub fn push(&self, e: EntryType) {
            let index = self.base_.index_.fetch_add(1, Ordering::Relaxed);
            unsafe {
                (*self.entries_.get())[index as usize].write(e);
            }
        }

        #[inline]
        pub fn pop(&self, e: &mut EntryType) {
            let index = self.base_.index_.fetch_sub(1, Ordering::Relaxed);
            unsafe {
                *e = (*self.entries_.get())[(index - 1) as usize].assume_init_read();
            }
        }

        pub fn update<Callback>(&self, callback: &Callback)
        where
            Callback: Fn(&mut EntryType) -> bool,
        {
            let mut new_index: usize = 0;
            let index = self.base_.size();

            for i in 0..index {
                unsafe {
                    let entry = &mut (*self.entries_.get())[i].assume_init_mut();
                    if callback(entry) {
                        if i != new_index {
                            std::mem::swap(
                                &mut (*self.entries_.get())[new_index],
                                &mut (*self.entries_.get())[i],
                            );
                        }
                        new_index += 1;
                    } else {
                        std::mem::drop(std::mem::replace(&mut (*self.entries_.get())[i], MaybeUninit::uninit()));
                    }
                }
            }

            self.base_.index_.store(new_index as u16, Ordering::Relaxed);
        }

        pub fn iterate<Callback>(&self, callback: &Callback)
        where
            Callback: Fn(&EntryType),
        {
            let index = self.base_.size();

            for i in 0..index {
                unsafe {
                    callback(&*(*self.entries_.get())[i].as_ptr());
                }
            }
        }

        pub fn next(&self) -> *mut Self {
            self.next_.load(Ordering::Relaxed)
        }

        pub fn set_next(&self, segment: *mut Self) {
            self.next_.store(segment, Ordering::Relaxed);
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.base_.is_empty()
        }

        const fn malloc_size_for_capacity(num_entries: usize) -> usize {
            std::mem::size_of::<Segment<EntryType, MinSegmentSize>>() + std::mem::size_of::<EntryType>() * num_entries
        }

        const fn capacity_for_malloc_size(malloc_size: usize) -> usize {
            (malloc_size - std::mem::size_of::<Segment<EntryType, MinSegmentSize>>()) / std::mem::size_of::<EntryType>()
        }
    }

    // A thread-local on a given worklist.
    pub struct Local<'a, EntryType, const MinSegmentSize: u16> {
        worklist_: &'a Worklist<EntryType, MinSegmentSize>,
        push_segment_: *mut Segment<EntryType, MinSegmentSize>,
        pop_segment_: *mut Segment<EntryType, MinSegmentSize>,
    }

    impl<'a, EntryType, const MinSegmentSize: u16> Local<'a, EntryType, MinSegmentSize> {
        pub type ItemType = EntryType;

        pub fn new(worklist: &'a Worklist<EntryType, MinSegmentSize>) -> Self {
            Local {
                worklist_: worklist,
                push_segment_: std::ptr::null_mut(),
                pop_segment_: std::ptr::null_mut(),
            }
        }

        #[inline]
        pub fn push(&mut self, entry: EntryType) {
            if self.push_segment_.is_null() || unsafe { (*self.push_segment_).base_.is_full() } {
                self.publish_push_segment();
                self.push_segment_ = self.new_segment();
            }
            unsafe {
                (*self.push_segment_).push(entry);
            }
        }

        #[inline]
        pub fn pop(&mut self, entry: &mut EntryType) -> bool {
            if self.pop_segment_.is_null() || unsafe { (*self.pop_segment_).base_.is_empty() } {
                if !self.push_segment_.is_null() && unsafe { !(*self.push_segment_).base_.is_empty() } {
                    std::mem::swap(&mut self.push_segment_, &mut self.pop_segment_);
                } else if !self.steal_pop_segment() {
                    return false;
                }
            }
            unsafe {
                (*self.pop_segment_).pop(entry);
            }
            true
        }

        pub fn is_local_and_global_empty(&self) -> bool {
            self.is_local_empty() && self.is_global_empty()
        }

        pub fn is_local_empty(&self) -> bool {
            (self.push_segment_.is_null() || unsafe { (*self.push_segment_).base_.is_empty() })
                && (self.pop_segment_.is_null() || unsafe { (*self.pop_segment_).base_.is_empty() })
        }

        pub fn is_global_empty(&self) -> bool {
            self.worklist_.is_empty()
        }

        pub fn push_segment_size(&self) -> usize {
            if self.push_segment_.is_null() {
                0
            } else {
                unsafe { (*self.push_segment_).base_.size() }
            }
        }

        pub fn publish(&mut self) {
            if !self.push_segment_.is_null() && unsafe { !(*self.push_segment_).base_.is_empty() } {
                self.publish_push_segment();
                self.push_segment_ = std::ptr::null_mut();
            }
            if !self.pop_segment_.is_null() && unsafe { !(*self.pop_segment_).base_.is_empty() } {
                self.publish_pop_segment();
                self.pop_segment_ = std::ptr::null_mut();
            }
        }

        pub fn merge(&mut self, other: &mut Local<'a, EntryType, MinSegmentSize>) {
            self.worklist_.merge(self.worklist_);
        }

        pub fn clear(&mut self) {
            if !self.push_segment_.is_null() {
                unsafe { (*self.push_segment_).base_.clear() };
            }
            if !self.pop_segment_.is_null() {
                unsafe { (*self.pop_segment_).base_.clear() };
            }
        }

        fn publish_push_segment(&mut self) {
            if !self.push_segment_.is_null() {
                self.worklist_.push(self.push_segment_);
            }
        }

        fn publish_pop_segment(&mut self) {
            if !self.pop_segment_.is_null() {
                self.worklist_.push(self.pop_segment_);
            }
        }

        fn steal_pop_segment(&mut self) -> bool {
            if self.worklist_.is_empty() {
                return false;
            }
            let mut new_segment: *mut Segment<EntryType, MinSegmentSize> = std::ptr::null_mut();
            if self.worklist_.pop(&mut new_segment) {
                self.delete_segment(self.pop_segment_);
                self.pop_segment_ = new_segment;
                return true;
            }
            false
        }

        fn new_segment(&self) -> *mut Segment<EntryType, MinSegmentSize> {
            // Bottleneck for filtering in crash dumps.
            Segment::<EntryType, MinSegmentSize>::create(MinSegmentSize)
        }

        fn delete_segment(&self, segment: *mut Segment<EntryType, MinSegmentSize>) {
            if !segment.is_null() {
                unsafe {
                    Segment::<EntryType, MinSegmentSize>::delete(segment);
                }
            }
        }
    }

    impl<'a, EntryType, const MinSegmentSize: u16> Drop for Local<'a, EntryType, MinSegmentSize> {
        fn drop(&mut self) {
            // TODO: enable this when the local is actually working
            // assert!((self.push_segment_.is_null() || unsafe { (*self.push_segment_).base_.is_empty() }), "Push segment not empty on drop");
            // assert!((self.pop_segment_.is_null() || unsafe {(*self.pop_segment_).base_.is_empty() }), "Pop segment not empty on drop");
            self.delete_segment(self.push_segment_);
            self.delete_segment(self.pop_segment_);
        }
    }
}
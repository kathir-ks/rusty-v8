// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::alloc::{alloc, dealloc, Layout};
use std::any::Any;
use std::cmp;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

macro_rules! v8_unlikely {
    ($cond:expr) => {
        $cond
    };
}

macro_rules! v8_noinline {
    ($func:item) => {
        #[inline(never)]
        $func
    };
}

macro_rules! v8_preserve_most {
    ($func:item) => {
        $func
    };
}

macro_rules! debug_assert_le {
    ($x:expr, $y:expr) => {
        debug_assert!($x <= $y);
    };
}

macro_rules! debug_assert_eq {
    ($x:expr, $y:expr) => {
        debug_assert!($x == $y);
    };
}

macro_rules! debug_assert_ne {
    ($x:expr, $y:expr) => {
        debug_assert!($x != $y);
    };
}

macro_rules! debug_assert_not_null {
    ($x:expr) => {
        debug_assert!(!$x.is_null());
    };
}

macro_rules! debug_assert_implies {
    ($x:expr, $y:expr) => {
        if $x {
            debug_assert!($y);
        }
    };
}

macro_rules! unreachble {
    () => {
        panic!("Unreachable");
    };
}

const COMPRESS_ZONES_BOOL: bool = false;

mod base {
    #[derive(Debug)]
    pub struct Vector<T> {
        data: *mut T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new(data: *mut T, length: usize) -> Self {
            Vector { data, length }
        }

        pub fn as_ptr(&self) -> *mut T {
            self.data
        }

        pub fn len(&self) -> usize {
            self.length
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        pub fn begin(&self) -> *mut T {
            self.data
        }

        pub fn end(&self) -> *mut T {
            unsafe { self.data.add(self.length) }
        }

        pub fn size(&self) -> usize {
            self.length
        }
    }
}

mod common {
    pub type Address = usize;
}

mod zone {
    use super::base::Vector;
    use super::common::Address;
    use super::v8_noinline;
    use super::v8_preserve_most;
    use super::v8_unlikely;
    use super::{debug_assert_eq, debug_assert_implies, debug_assert_le, debug_assert_ne, debug_assert_not_null, unreachble};
    use std::alloc::{alloc, dealloc, Layout};
    use std::any::Any;
    use std::cmp;
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Placeholder for AccountingAllocator.  In a real implementation, this
    // would manage memory segments.
    pub struct AccountingAllocator {}

    impl AccountingAllocator {
        pub fn new() -> Self {
            AccountingAllocator {}
        }

        pub fn allocate_segment(&self, size: usize) -> *mut u8 {
            unsafe {
                let layout = Layout::from_size_align(size, 8).unwrap();
                alloc(layout)
            }
        }

        pub fn deallocate_segment(&self, ptr: *mut u8, size: usize) {
            unsafe {
                let layout = Layout::from_size_align(size, 8).unwrap();
                dealloc(ptr, layout);
            }
        }
    }

    pub struct TypeStats {}

    impl TypeStats {
        pub fn new() -> Self {
            TypeStats {}
        }

        pub fn add_allocated<T>(&mut self, size: usize) {
            // Placeholder.  Implementation would track allocation stats for type T.
        }

        pub fn add_deallocated<T>(&mut self, size: usize) {
            // Placeholder. Implementation would track deallocation stats for type T.
        }
    }

    // Placeholder for TracingFlags.
    pub mod TracingFlags {
        pub fn is_zone_stats_enabled() -> bool {
            false
        }
    }

    struct Segment {
        start: Address,
        size: usize,
        next: *mut Segment,
    }

    impl Segment {
        fn new(start: Address, size: usize) -> Self {
            Segment {
                start,
                size,
                next: ptr::null_mut(),
            }
        }

        fn start(&self) -> Address {
            self.start
        }

        fn size(&self) -> usize {
            self.size
        }
    }

    // Define RoundUp function
    fn round_up(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    pub struct ZoneSnapshot {
        allocation_size_for_tracing_: usize,
        freed_size_for_tracing_: usize,
        allocation_size_: usize,
        segment_bytes_allocated_: usize,
        position_: Address,
        limit_: Address,
        segment_head_: *mut Segment,
    }

    impl ZoneSnapshot {
        fn new(zone: &Zone) -> Self {
            ZoneSnapshot {
                allocation_size_for_tracing_: zone.allocation_size_for_tracing(),
                freed_size_for_tracing_: zone.freed_size_for_tracing(),
                allocation_size_: zone.allocation_size(),
                segment_bytes_allocated_: zone.segment_bytes_allocated(),
                position_: zone.position_,
                limit_: zone.limit_,
                segment_head_: zone.segment_head_,
            }
        }

        pub fn restore(&self, zone: &mut Zone) {
            zone.allocation_size_for_tracing_.store(self.allocation_size_for_tracing_, Ordering::SeqCst);
            zone.freed_size_for_tracing_.store(self.freed_size_for_tracing_, Ordering::SeqCst);
            zone.allocation_size_.store(self.allocation_size_, Ordering::SeqCst);
            zone.segment_bytes_allocated_.store(self.segment_bytes_allocated_, Ordering::SeqCst);
            zone.position_ = self.position_;
            zone.limit_ = self.limit_;
            zone.segment_head_ = self.segment_head_;
        }
    }

    /// The Zone supports very fast allocation of small chunks of
    /// memory. The chunks cannot be deallocated individually, but instead
    /// the Zone supports deallocating all chunks in one fast
    /// operation. The Zone is used to hold temporary data structures like
    /// the abstract syntax tree, which is deallocated after compilation.
    ///
    /// Note: There is no need to initialize the Zone; the first time an
    /// allocation is attempted, a segment of memory will be requested
    /// through the allocator.
    ///
    /// Note: The implementation is inherently not thread safe. Do not use
    /// from multi-threaded code.
    pub struct Zone {
        allocator_: *mut AccountingAllocator,
        name_: *const i8, // C-style string
        supports_compression_: bool,
        sealed_: bool,
        segment_head_: *mut Segment,
        position_: Address,
        limit_: Address,
        allocation_size_: AtomicUsize,
        segment_bytes_allocated_: AtomicUsize,
        allocation_size_for_tracing_: AtomicUsize,
        freed_size_for_tracing_: AtomicUsize,
        type_stats_: TypeStats,
    }

    impl Zone {
        const K_ALIGNMENT_IN_BYTES: usize = 8;
        const K_MINIMUM_SEGMENT_SIZE: usize = 8 * 1024;
        const K_MAXIMUM_SEGMENT_SIZE: usize = 32 * 1024;

        /// Creates a new Zone.
        pub fn new(allocator: *mut AccountingAllocator, name: *const i8, support_compression: bool) -> Self {
            Zone {
                allocator_: allocator,
                name_: name,
                supports_compression_: support_compression,
                sealed_: false,
                segment_head_: ptr::null_mut(),
                position_: 0,
                limit_: 0,
                allocation_size_: AtomicUsize::new(0),
                segment_bytes_allocated_: AtomicUsize::new(0),
                allocation_size_for_tracing_: AtomicUsize::new(0),
                freed_size_for_tracing_: AtomicUsize::new(0),
                type_stats_: TypeStats::new(),
            }
        }

        /// Returns true if the zone supports zone pointer compression.
        pub fn supports_compression(&self) -> bool {
            COMPRESS_ZONES_BOOL && self.supports_compression_
        }

        /// Allocate 'size' bytes of uninitialized memory in the Zone; expands the Zone
        /// by allocating new segments of memory on demand using AccountingAllocator
        /// (see AccountingAllocator::AllocateSegment()).
        ///
        /// When V8_ENABLE_PRECISE_ZONE_STATS is defined, the allocated bytes are
        /// associated with the provided TypeTag type.
        pub fn allocate<TypeTag>(&mut self, size: usize) -> *mut u8 {
            let mut size = size;
            size = round_up(size, Self::K_ALIGNMENT_IN_BYTES);

            if v8_unlikely!(super::zone::TracingFlags::is_zone_stats_enabled()) {
                self.type_stats_.add_allocated::<TypeTag>(size);
            }
            self.allocation_size_for_tracing_
                .fetch_add(size, Ordering::SeqCst);

            if v8_unlikely!(size > self.limit_ - self.position_) {
                self.expand(size);
            }

            debug_assert_le!(self.position_, self.limit_);
            debug_assert_le!(size, self.limit_ - self.position_);
            debug_assert_eq!(0, self.position_ % Self::K_ALIGNMENT_IN_BYTES);

            let result = self.position_ as *mut u8;
            self.position_ += size;
            result
        }

        /// Return 'size' bytes of memory back to Zone. These bytes can be reused
        /// for following allocations.
        ///
        /// When V8_ENABLE_PRECISE_ZONE_STATS is defined, the deallocated bytes are
        /// associated with the provided TypeTag type.
        pub fn delete<TypeTag>(&mut self, pointer: *mut u8, size: usize) {
            debug_assert_not_null!(pointer);
            debug_assert_ne!(size, 0);
            let mut size = size;
            size = round_up(size, Self::K_ALIGNMENT_IN_BYTES);

            if v8_unlikely!(super::zone::TracingFlags::is_zone_stats_enabled()) {
                self.type_stats_.add_deallocated::<TypeTag>(size);
            }
            self.freed_size_for_tracing_.fetch_add(size, Ordering::SeqCst);

            #[cfg(debug_assertions)]
            {
                const K_ZAP_DEAD_BYTE: u8 = 0xcd;
                unsafe {
                    std::ptr::write_bytes(pointer, K_ZAP_DEAD_BYTE, size);
                }
            }
        }

        /// Allocates memory for T instance and constructs object by calling respective
        /// Args... constructor.
        ///
        /// When V8_ENABLE_PRECISE_ZONE_STATS is defined, the allocated bytes are
        /// associated with the T type.
        pub fn new_object<T>(&mut self) -> *mut T {
            assert!(std::mem::align_of::<T>() <= Self::K_ALIGNMENT_IN_BYTES);
            let memory = self.allocate::<T>(std::mem::size_of::<T>()) as *mut T;
            memory
        }

        pub fn allocate_array<T, TypeTag>(&mut self, length: usize) -> *mut T {
            assert!(std::mem::align_of::<T>() <= Self::K_ALIGNMENT_IN_BYTES);
            debug_assert_implies!(
                false, //is_compressed_pointer::<T>::value,
                self.supports_compression()
            );
            debug_assert_le!(
                length,
                std::usize::MAX / std::mem::size_of::<T>()
            );
            self.allocate::<TypeTag>(length * std::mem::size_of::<T>()) as *mut T
        }

        pub fn allocate_vector<T, TypeTag>(&mut self, length: usize) -> Vector<T> {
            let new_array = self.allocate_array::<T, TypeTag>(length);
            Vector::new(new_array, length)
        }

        pub fn new_vector<T, TypeTag>(&mut self, length: usize) -> Vector<T> {
            let new_array = self.allocate_array::<T, TypeTag>(length);
            unsafe {
                std::slice::from_raw_parts_mut(new_array, length)
                    .fill_with(|| std::mem::zeroed());
            }
            Vector::new(new_array, length)
        }

        pub fn new_vector_with_value<T: Copy, TypeTag>(&mut self, length: usize, value: T) -> Vector<T> {
            let new_array = self.allocate_array::<T, TypeTag>(length);
            unsafe {
                std::slice::from_raw_parts_mut(new_array, length).fill(value);
            }
            Vector::new(new_array, length)
        }

        pub fn clone_vector<T: Copy, TypeTag>(&mut self, v: Vector<T>) -> Vector<T> {
            let new_array = self.allocate_array::<T, TypeTag>(v.size());
            unsafe {
                std::ptr::copy_nonoverlapping(v.begin(), new_array, v.size());
            }
            Vector::new(new_array, v.size())
        }

        pub fn delete_array<T, TypeTag>(&mut self, pointer: *mut T, length: usize) {
            self.delete::<TypeTag>(pointer as *mut u8, length * std::mem::size_of::<T>());
        }

        /// Seals the zone to prevent any further allocation.
        pub fn seal(&mut self) {
            self.sealed_ = true;
        }

        /// Allows the zone to be safely reused. Releases the memory except for the
        /// last page, and fires zone destruction and creation events for the
        /// accounting allocator.
        pub fn reset(&mut self) {
            // TODO: Implement reset
            // unimplemented!()
        }

        pub fn segment_bytes_allocated(&self) -> usize {
            self.segment_bytes_allocated_.load(Ordering::SeqCst)
        }

        pub fn name(&self) -> *const i8 {
            self.name_
        }

        pub fn allocation_size(&self) -> usize {
            let extra = if !self.segment_head_.is_null() {
                unsafe { self.position_ - (*self.segment_head_).start() }
            } else {
                0
            };
            self.allocation_size_.load(Ordering::SeqCst) + extra
        }

        pub fn allocation_size_for_tracing(&self) -> usize {
            self.allocation_size_for_tracing_.load(Ordering::SeqCst)
        }

        pub fn freed_size_for_tracing(&self) -> usize {
            self.freed_size_for_tracing_.load(Ordering::SeqCst)
        }

        pub fn allocator(&self) -> *mut AccountingAllocator {
            self.allocator_
        }

        pub fn type_stats(&self) -> &TypeStats {
            &self.type_stats_
        }

        pub fn snapshot(&self) -> ZoneSnapshot {
            ZoneSnapshot::new(self)
        }

        #[v8_noinline]
        #[v8_preserve_most]
        fn expand(&mut self, size: usize) {
            let new_segment_size = cmp::max(
                Self::K_MINIMUM_SEGMENT_SIZE,
                cmp::min(size, Self::K_MAXIMUM_SEGMENT_SIZE),
            );

            unsafe {
                let allocator = &mut *self.allocator_;
                let new_segment_start = allocator.allocate_segment(new_segment_size);

                self.segment_bytes_allocated_
                    .fetch_add(new_segment_size, Ordering::SeqCst);

                let new_segment = Segment::new(new_segment_start as Address, new_segment_size);
                let new_segment_ptr = Box::into_raw(Box::new(new_segment));

                if !self.segment_head_.is_null() {
                    (*new_segment_ptr).next = self.segment_head_;
                }
                self.segment_head_ = new_segment_ptr;
                self.position_ = new_segment_start as Address;
                self.limit_ = new_segment_start as Address + new_segment_size;
            }
        }

        fn release_segment(&mut self, segment: *mut Segment) {
            unsafe {
                let allocator = &mut *self.allocator_;
                allocator.deallocate_segment(
                    (*segment).start as *mut u8,
                    (*segment).size,
                );
                self.segment_bytes_allocated_.fetch_sub((*segment).size, Ordering::SeqCst);
            }
        }

        fn delete_all(&mut self) {
            while !self.segment_head_.is_null() {
                unsafe {
                    let current = self.segment_head_;
                    self.segment_head_ = (*current).next;
                    self.release_segment(current);
                    drop(Box::from_raw(current)); // Deallocate the Segment struct
                }
            }
            self.position_ = 0;
            self.limit_ = 0;
            self.allocation_size_.store(0, Ordering::SeqCst);
        }
    }

    impl Drop for Zone {
        fn drop(&mut self) {
            self.delete_all();
        }
    }

    /// Similar to the HandleScope, the ZoneScope defines a region of validity for
    /// zone memory. All memory allocated in the given Zone during the scope's
    /// lifetime is freed when the scope is destructed, i.e. the Zone is reset to
    /// the state it was in when the scope was created.
    pub struct ZoneScope<'a> {
        zone_: &'a mut Zone,
        snapshot_: ZoneSnapshot,
    }

    impl<'a> ZoneScope<'a> {
        /// Creates a new ZoneScope.
        pub fn new(zone: &'a mut Zone) -> Self {
            let snapshot_ = zone.snapshot();
            ZoneScope {
                zone_: zone,
                snapshot_: snapshot_,
            }
        }
    }

    impl<'a> Drop for ZoneScope<'a> {
        fn drop(&mut self) {
            self.snapshot_.restore(self.zone_);
        }
    }

    /// ZoneObject is an abstraction that helps define classes of objects
    /// allocated in the Zone. Use it as a base class; see ast.h.
    pub struct ZoneObject {}

    impl ZoneObject {
        // Intentionally empty.  This type mainly serves as a marker.
    }

    /// The ZoneAllocationPolicy is used to specialize generic data
    /// structures to allocate themselves and their elements in the Zone.
    #[derive(Clone, Copy)]
    pub struct ZoneAllocationPolicy<'a> {
        zone_: Option<&'a mut Zone>,
    }

    impl<'a> ZoneAllocationPolicy<'a> {
        /// Creates unusable allocation policy.
        pub fn new() -> Self {
            ZoneAllocationPolicy { zone_: None }
        }

        /// Creates allocation policy with provided zone.
        pub fn with_zone(zone: &'a mut Zone) -> Self {
            ZoneAllocationPolicy { zone_: Some(zone) }
        }

        pub fn allocate_array<T, TypeTag>(&mut self, length: usize) -> *mut T {
            self.zone().unwrap().allocate_array::<T, TypeTag>(length)
        }

        pub fn delete_array<T, TypeTag>(&mut self, p: *mut T, length: usize) {
            self.zone().unwrap().delete_array::<T, TypeTag>(p, length);
        }

        pub fn zone(&self) -> Option<&mut Zone> {
            unsafe {
                self.zone_.map(|z| {
                    let ptr = z as *mut Zone;
                    &mut *ptr
                })
            }
        }
    }
}
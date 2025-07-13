// Converted from V8 C++ source files:
// Header: zone.h
// Implementation: zone.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    alloc::{alloc, dealloc, Layout},
    marker::PhantomData,
    mem::{align_of, size_of},
    ptr::{self, NonNull},
};

use crate::Address;

const KB: usize = 1024;

#[macro_export]
macro_rules! RoundUp {
    ($x:expr, $alignment:expr) => {
        (($x + $alignment - 1) / $alignment) * $alignment
    };
}

#[macro_export]
macro_rules! RoundDown {
    ($x:expr, $alignment:expr) => {
        $x - ($x % $alignment)
    };
}

#[macro_export]
macro_rules! IsAligned {
    ($x:expr, $alignment:expr) => {
        $x % $alignment == 0
    };
}
//use crate::globals::kAlignmentInBytes;

#[derive(Debug)]
struct Segment {
    start_: Address,
    end_: Address,
    total_size_: usize,
    next_: Option<Box<Segment>>,
    zone_: *mut Zone,
    compressed: bool,
}

impl Segment {
    fn start(&self) -> Address {
        self.start_
    }

    fn end(&self) -> Address {
        self.end_
    }

    fn total_size(&self) -> usize {
        self.total_size_
    }

    fn next(&self) -> Option<&Segment> {
        self.next_.as_ref().map(|n| &**n)
    }

    fn set_next(&mut self, next: Option<Box<Segment>>) {
        self.next_ = next;
    }

    fn set_zone(&mut self, zone: *mut Zone) {
        self.zone_ = zone;
    }

    fn capacity(&self) -> usize {
        self.total_size_ - size_of::<Segment>()
    }

    fn zap_contents(&mut self) {
        unsafe {
            ptr::write_bytes(self.start_ as *mut u8, 0xcd, self.capacity());
        }
    }
}
#[derive(Debug)]
pub struct AccountingAllocator {}

impl AccountingAllocator {
    pub fn new() -> Self {
        AccountingAllocator {}
    }
    fn allocate_segment(
        &self,
        size: usize,
        supports_compression: bool,
    ) -> Result<Box<Segment>, String> {
        let layout = Layout::from_size_align(size, 8).map_err(|e| e.to_string())?;
        let ptr = unsafe { alloc(layout) } as Address;
        if ptr == 0 {
            return Err("Allocation failed".to_string());
        }

        let segment = Segment {
            start_: ptr + size_of::<Segment>() as Address,
            end_: ptr + size as Address,
            total_size_: size,
            next_: None,
            zone_: ptr::null_mut(),
            compressed: supports_compression,
        };

        Ok(Box::new(segment))
    }

    fn return_segment(&self, segment: Box<Segment>, _supports_compression: bool) {
        let size = segment.total_size_;
        let ptr = segment.start_ - size_of::<Segment>() as Address;

        drop(segment);

        let layout = Layout::from_size_align(size, 8).unwrap();
        unsafe {
            dealloc(ptr as *mut u8, layout);
        }
    }

    fn trace_zone_creation(&self, _zone: *mut Zone) {}
    fn trace_zone_destruction(&self, _zone: *mut Zone) {}
    fn trace_allocate_segment(&self, _segment: *mut Segment) {}
}

//#[thread_local]
static COMPRESS_ZONES_BOOL: bool = false;

#[derive(Debug)]
pub struct Zone {
    allocator_: *mut AccountingAllocator,
    name_: String,
    supports_compression_: bool,
    sealed_: bool,
    position_: Address,
    limit_: Address,
    segment_head_: Option<Box<Segment>>,
    allocation_size_: AtomicUsize,
    segment_bytes_allocated_: AtomicUsize,
    allocation_size_for_tracing_: AtomicUsize,
    freed_size_for_tracing_: AtomicUsize,
    type_stats_: TypeStats,
}

impl Zone {
    pub fn new(allocator: *mut AccountingAllocator, name: &str, support_compression: bool) -> Self {
        Zone {
            allocator_: allocator,
            name_: name.to_string(),
            supports_compression_: support_compression,
            sealed_: false,
            position_: 0,
            limit_: 0,
            segment_head_: None,
            allocation_size_: AtomicUsize::new(0),
            segment_bytes_allocated_: AtomicUsize::new(0),
            allocation_size_for_tracing_: AtomicUsize::new(0),
            freed_size_for_tracing_: AtomicUsize::new(0),
            type_stats_: TypeStats::new(),
        }
    }

    pub fn supports_compression(&self) -> bool {
        COMPRESS_ZONES_BOOL && self.supports_compression_
    }

    pub fn allocate<TypeTag>(&self, size: usize) -> *mut u8 {
        let size = RoundUp!(size, Self::kAlignmentInBytes);

        if size > self.limit_ - self.position_ {
            self.expand(size);
        }

        let result = self.position_ as *mut u8;
        self.position_ += size;
        result
    }

    pub fn delete<TypeTag>(&self, pointer: *mut u8, size: usize) {
        let size = RoundUp!(size, Self::kAlignmentInBytes);
    }

    pub fn new_object<T>(&self) -> *mut T {
        let memory = self.allocate::<T>(size_of::<T>()) as *mut T;
        memory
    }

    pub fn allocate_array<T, TypeTag>(&self, length: usize) -> *mut T {
        let memory = self.allocate::<TypeTag>(length * size_of::<T>()) as *mut T;
        memory
    }

    pub fn allocate_vector<T, TypeTag>(&self, length: usize) -> Vec<T> {
        let memory = self.allocate_array::<T, TypeTag>(length);
        unsafe { Vec::from_raw_parts(memory, 0, length) }
    }

    pub fn delete_array<T, TypeTag>(&self, pointer: *mut T, length: usize) {
        self.delete::<TypeTag>(pointer as *mut u8, length * size_of::<T>());
    }

    pub fn seal(&mut self) {
        self.sealed_ = true;
    }

    pub fn reset(&mut self) {
        if self.segment_head_.is_none() {
            return;
        }

        let mut keep = self.segment_head_.take().unwrap();
        self.segment_head_ = keep.next_.take();

        if let Some(ref mut segment_head) = self.segment_head_ {
            self.position_ = segment_head.end();
            self.allocation_size_
                .fetch_sub((segment_head.end() - segment_head.start()) as usize, Ordering::SeqCst);
        }

        unsafe {
            let allocator = &mut *self.allocator_;
            allocator.trace_zone_creation(self);
        }

        unsafe {
            ptr::write_bytes(keep.start_ as *mut u8, 0xcd, keep.capacity());
        }

        self.position_ = RoundUp!(keep.start(), Self::kAlignmentInBytes);
        self.limit_ = keep.end();
        self.segment_bytes_allocated_.store(keep.total_size(), Ordering::SeqCst);

        self.segment_head_ = Some(keep);
    }

    pub fn segment_bytes_allocated(&self) -> usize {
        self.segment_bytes_allocated_.load(Ordering::SeqCst)
    }

    pub fn name(&self) -> &str {
        &self.name_
    }

    pub fn allocation_size(&self) -> usize {
        let extra = match &self.segment_head_ {
            Some(head) => self.position_ - head.start(),
            None => 0,
        };
        self.allocation_size_.load(Ordering::SeqCst) + extra as usize
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

    fn asan_new(&mut self, size: usize) -> *mut u8 {
        if self.sealed_ {
            panic!("Zone is sealed");
        }

        let size = RoundUp!(size, Self::kAlignmentInBytes);

        let size_with_redzone = size + Self::kASanRedzoneBytes;
        if size_with_redzone > self.limit_ - self.position_ {
            self.expand(size_with_redzone);
        }

        let result = self.position_ as *mut u8;
        self.position_ += size_with_redzone;

        let redzone_position = (result as Address) + size;

        result
    }

    fn delete_all(&mut self) {
        if let Some(head) = &self.segment_head_ {
            self.allocation_size_.store(self.allocation_size(), Ordering::SeqCst);
        }

        unsafe {
            let allocator = &mut *self.allocator_;
            allocator.trace_zone_destruction(self);
        }

        let mut current = self.segment_head_.take();

        while let Some(mut segment) = current {
            self.segment_bytes_allocated_.fetch_sub(segment.total_size(), Ordering::SeqCst);
            current = segment.next_.take();
            self.release_segment(segment);
        }

        self.position_ = 0;
        self.limit_ = 0;
        self.allocation_size_.store(0, Ordering::SeqCst);
        self.allocation_size_for_tracing_.store(0, Ordering::SeqCst);
    }

    fn release_segment(&mut self, mut segment: Box<Segment>) {
        unsafe {
            let allocator = &mut *self.allocator_;
            allocator.return_segment(segment, self.supports_compression_);
        }
    }

    #[cold]
    #[inline(never)]
    fn expand(&mut self, size: usize) {
        if size != RoundDown!(size, Self::kAlignmentInBytes) {
            panic!("Size is not aligned");
        }
        if self.limit_ - self.position_ >= size {
            panic!("There is enough room in the Zone");
        }

        let old_size = self
            .segment_head_
            .as_ref()
            .map_or(0, |head| head.total_size());
        let k_segment_overhead = size_of::<Segment>() + Self::kAlignmentInBytes;
        let new_size_no_overhead = size + (old_size << 1);
        let mut new_size = k_segment_overhead + new_size_no_overhead;
        let min_new_size = k_segment_overhead + size;

        if new_size_no_overhead < size || new_size < k_segment_overhead {
            panic!("Out of memory");
        }

        if new_size < Self::kMinimumSegmentSize {
            new_size = Self::kMinimumSegmentSize;
        } else if new_size >= Self::kMaximumSegmentSize {
            new_size = std::cmp::max(min_new_size, Self::kMaximumSegmentSize);
        }

        if new_size > isize::MAX as usize {
            panic!("Out of memory");
        }

        let segment = unsafe {
            let allocator = &mut *self.allocator_;
            allocator
                .allocate_segment(new_size, self.supports_compression_)
                .expect("Allocation failed")
        };

        self.segment_bytes_allocated_.fetch_add(segment.total_size(), Ordering::SeqCst);

        unsafe {
            segment.set_zone(self);
        }

        let mut segment_box = segment;
        segment_box.next_ = self.segment_head_.take();

        self.allocation_size_.store(self.allocation_size(), Ordering::SeqCst);
        self.segment_head_ = Some(segment_box);

        unsafe {
            let allocator = &mut *self.allocator_;
            if let Some(ref sh) = &self.segment_head_ {
                allocator.trace_allocate_segment(sh.as_ref() as *const Segment as *mut Segment);
            }
        }

        if let Some(ref head) = self.segment_head_ {
            self.position_ = RoundUp!(head.start(), Self::kAlignmentInBytes);
            self.limit_ = head.end();
        }
    }

    const kAlignmentInBytes: usize = 8;
    const kMinimumSegmentSize: usize = 8 * KB;
    const kMaximumSegmentSize: usize = 32 * KB;

    pub fn snapshot(&self) -> ZoneSnapshot {
        ZoneSnapshot {
            allocation_size_for_tracing_: self.allocation_size_for_tracing_.load(Ordering::SeqCst),
            freed_size_for_tracing_: self.freed_size_for_tracing_.load(Ordering::SeqCst),
            allocation_size_: self.allocation_size_.load(Ordering::SeqCst),
            segment_bytes_allocated_: self.segment_bytes_allocated_.load(Ordering::SeqCst),
            position_: self.position_,
            limit_: self.limit_,
            segment_head_: self.segment_head_.as_ref().map(|s| &**s as *const Segment),
        }
    }
}

impl Drop for Zone {
    fn drop(&mut self) {
        self.delete_all();
    }
}

#[derive(Debug)]
struct ZoneSnapshot {
    allocation_size_for_tracing_: usize,
    freed_size_for_tracing_: usize,
    allocation_size_: usize,
    segment_bytes_allocated_: usize,
    position_: Address,
    limit_: Address,
    segment_head_: *const Segment,
}

impl ZoneSnapshot {
    fn restore(&self, zone: &mut Zone) {
        let mut current = zone.segment_head_.take();
        while let Some(segment) = current {
            if segment.as_ref() as *const Segment == self.segment_head_ {
                zone.segment_head_ = Some(segment);
                break;
            }
            current = segment.next_.take();
        }

        zone.allocation_size_.store(self.allocation_size_, Ordering::SeqCst);
        zone.segment_bytes_allocated_.store(self.segment_bytes_allocated_, Ordering::SeqCst);
        zone.position_ = self.position_;
        zone.limit_ = self.limit_;
        zone.allocation_size_for_tracing_.store(self.allocation_size_for_tracing_, Ordering::SeqCst);
        zone.freed_size_for_tracing_.store(self.freed_size_for_tracing_, Ordering::SeqCst);
    }
}

struct ZoneScope<'a> {
    zone_: &'a mut Zone,
    snapshot_: ZoneSnapshot,
}

impl<'a> ZoneScope<'a> {
    fn new(zone: &'a mut Zone) -> Self {
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

#[derive(Debug)]
struct ZoneObject {}

impl ZoneObject {
    pub fn operator_new(_size: usize, _ptr: *mut Zone) {}
    pub fn operator_delete(_ptr: *mut u8, _size: usize) {
        unreachable!();
    }
}

struct ZoneAllocationPolicy<'a> {
    zone_: Option<&'a Zone>,
}

impl<'a> ZoneAllocationPolicy<'a> {
    fn new() -> Self {
        ZoneAllocationPolicy { zone_: None }
    }
    fn with_zone(zone: &'a Zone) -> Self {
        ZoneAllocationPolicy { zone_: Some(zone) }
    }

    fn allocate_array<T, TypeTag>(&self, length: usize) -> *mut T {
        self.zone().allocate_array::<T, TypeTag>(length)
    }

    fn delete_array<T, TypeTag>(&self, p: *mut T, length: usize) {
        self.zone().delete_array::<T, TypeTag>(p, length)
    }

    fn zone(&self) -> &Zone {
        self.zone_.expect("Zone is None")
    }
}

#[derive(Debug)]
struct TypeStats {
    stats: std::collections::HashMap<String, usize>,
}

impl TypeStats {
    fn new() -> Self {
        TypeStats {
            stats: std::collections::HashMap::new(),
        }
    }
}

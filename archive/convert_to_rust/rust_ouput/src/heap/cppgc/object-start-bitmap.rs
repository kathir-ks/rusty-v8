// Converted from V8 C++ source files:
// Header: object-start-bitmap.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::sync::atomic::{AtomicPtr, Ordering};
use std::{array, mem, ptr, slice};

use crate::heap::cppgc::globals::kAllocationGranularity;
use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
use crate::heap::safepoint::AtomicThreadState;

const kPageSize: usize = 4096; // Assuming a typical page size
const kAllocationMask: usize = kAllocationGranularity - 1;
const kBlinkPageSize: usize = 4096;
const kPageBaseMask: usize = !(kPageSize - 1);
const kPageOffsetMask: usize = kPageSize - 1;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AccessMode {
    kNonAtomic,
    kAtomic,
}

pub struct ObjectStartBitmap {
    fully_populated_: bool,
    object_start_bit_map_: array::[u8; kReservedForBitmap],
}

const kBitsPerCell: usize = 8;
const kCellMask: usize = kBitsPerCell - 1;
const kBitmapSize: usize =
    (kPageSize + ((kBitsPerCell * kAllocationGranularity) - 1)) / (kBitsPerCell * kAllocationGranularity);
const kReservedForBitmap: usize = ((kBitmapSize + kAllocationMask) & !kAllocationMask);

impl ObjectStartBitmap {
    pub const fn Granularity() -> usize {
        kAllocationGranularity
    }

    pub const fn MaxEntries() -> usize {
        kReservedForBitmap * kBitsPerCell
    }

    pub fn new() -> Self {
        let mut bitmap = Self {
            fully_populated_: false,
            object_start_bit_map_: [0u8; kReservedForBitmap],
        };
        bitmap.Clear();
        bitmap.MarkAsFullyPopulated();
        bitmap
    }

    pub fn FindHeader(
        &self,
        address_maybe_pointing_to_the_middle_of_object: usize,
    ) -> *mut HeapObjectHeader {
        self.FindHeaderInternal::<AccessMode::kNonAtomic>(address_maybe_pointing_to_the_middle_of_object)
    }

    fn FindHeaderInternal<const MODE: AccessMode>(
        &self,
        address_maybe_pointing_to_the_middle_of_object: usize,
    ) -> *mut HeapObjectHeader {
        assert!(self.fully_populated_);

        let page_base = address_maybe_pointing_to_the_middle_of_object & kPageBaseMask;
        assert_eq!(
            page_base,
            (self as *const Self as usize) & kPageBaseMask
        );

        let object_offset = address_maybe_pointing_to_the_middle_of_object & kPageOffsetMask;
        let object_start_number = object_offset / kAllocationGranularity;
        let mut cell_index = object_start_number / kBitsPerCell;
        assert!(self.object_start_bit_map_.len() > cell_index);
        let bit = object_start_number & kCellMask;

        let mut byte = self.load::<MODE>(cell_index) & ((1 << (bit + 1)) - 1);

        while byte == 0 && cell_index > 0 {
            cell_index -= 1;
            byte = self.load::<MODE>(cell_index);
        }

        let leading_zeroes = byte.leading_zeros() as usize;
        let object_start_number =
            (cell_index * kBitsPerCell) + (kBitsPerCell - 1) - leading_zeroes;
        let object_offset = object_start_number * kAllocationGranularity;

        (page_base + object_offset) as *mut HeapObjectHeader
    }

    pub fn SetBit(&mut self, header_address: usize) {
        self.SetBitInternal::<AccessMode::kNonAtomic>(header_address);
    }

    fn SetBitInternal<const MODE: AccessMode>(&mut self, header_address: usize) {
        let (mut cell_index, object_bit) = self.ObjectStartIndexAndBit(header_address);

        let current_value = self.load::<MODE>(cell_index);
        self.store::<MODE>(
            cell_index,
            (current_value | (1 << object_bit)) as u8,
        );
    }

    pub fn ClearBit(&mut self, header_address: usize) {
        self.ClearBitInternal::<AccessMode::kNonAtomic>(header_address);
    }

    fn ClearBitInternal<const MODE: AccessMode>(&mut self, header_address: usize) {
        let (mut cell_index, object_bit) = self.ObjectStartIndexAndBit(header_address);
        let current_value = self.load::<MODE>(cell_index);
        self.store::<MODE>(
            cell_index,
            (current_value & !(1 << object_bit)) as u8,
        );
    }

    pub fn CheckBit(&self, header_address: usize) -> bool {
        self.CheckBitInternal::<AccessMode::kNonAtomic>(header_address)
    }

    fn CheckBitInternal<const MODE: AccessMode>(&self, header_address: usize) -> bool {
        let (mut cell_index, object_bit) = self.ObjectStartIndexAndBit(header_address);
        self.load::<MODE>(cell_index) & (1 << object_bit) != 0
    }

    fn store<const MODE: AccessMode>(&mut self, cell_index: usize, value: u8) {
        if MODE == AccessMode::kNonAtomic {
            self.object_start_bit_map_[cell_index] = value;
            return;
        }

        let atomic_ptr = AtomicPtr::new(&mut self.object_start_bit_map_[cell_index]);
        atomic_ptr.store(value, Ordering::Release);
    }

    fn load<const MODE: AccessMode>(&self, cell_index: usize) -> u8 {
        if MODE == AccessMode::kNonAtomic {
            return self.object_start_bit_map_[cell_index];
        }
        let atomic_ptr = AtomicPtr::new(&self.object_start_bit_map_[cell_index] as *const u8 as *mut u8);
        unsafe { *atomic_ptr.load(Ordering::Acquire) }
    }

    fn ObjectStartIndexAndBit(&self, header_address: usize) -> (usize, usize) {
        let object_offset = header_address & kPageOffsetMask;
        assert_eq!(object_offset & kAllocationMask, 0);

        let object_start_number = object_offset / kAllocationGranularity;
        let mut cell_index = object_start_number / kBitsPerCell;
        assert!(kBitmapSize > cell_index);
        let bit = object_start_number & kCellMask;

        (cell_index, bit)
    }

    pub fn Iterate<F>(&self, mut callback: F)
    where
        F: FnMut(usize),
    {
        let page_base = (self as *const Self as usize) & kPageBaseMask;

        for cell_index in 0..kReservedForBitmap {
            if self.object_start_bit_map_[cell_index] == 0 {
                continue;
            }

            let mut value = self.object_start_bit_map_[cell_index];
            while value != 0 {
                let trailing_zeroes = value.trailing_zeros() as usize;
                let object_start_number = (cell_index * kBitsPerCell) + trailing_zeroes;
                let object_address = page_base + (kAllocationGranularity * object_start_number);
                callback(object_address);

                value &= !(1 << trailing_zeroes);
            }
        }
    }

    pub fn MarkAsFullyPopulated(&mut self) {
        assert!(!self.fully_populated_);
        self.fully_populated_ = true;
    }

    pub fn Clear(&mut self) {
        self.fully_populated_ = false;
        self.object_start_bit_map_.fill(0);
    }
}

pub struct PlatformAwareObjectStartBitmap {
    base: ObjectStartBitmap
}

impl PlatformAwareObjectStartBitmap {
    pub fn new() -> Self {
        PlatformAwareObjectStartBitmap{base: ObjectStartBitmap::new()}
    }

    pub fn SetBit(&mut self, header_address: usize) {
        self.SetBitInternal::<AccessMode::kNonAtomic>(header_address);
    }

    fn SetBitInternal<const MODE: AccessMode>(&mut self, header_address: usize) {
        if Self::ShouldForceNonAtomic::<MODE>() {
            self.base.SetBitInternal::<AccessMode::kNonAtomic>(header_address);
            return;
        }
        self.base.SetBitInternal::<MODE>(header_address);
    }

    pub fn ClearBit(&mut self, header_address: usize) {
        self.ClearBitInternal::<AccessMode::kNonAtomic>(header_address);
    }

    fn ClearBitInternal<const MODE: AccessMode>(&mut self, header_address: usize) {
        if Self::ShouldForceNonAtomic::<MODE>() {
            self.base.ClearBitInternal::<AccessMode::kNonAtomic>(header_address);
            return;
        }
        self.base.ClearBitInternal::<MODE>(header_address);
    }

    fn ShouldForceNonAtomic<const MODE: AccessMode>() -> bool {
        #[cfg(target_arch = "arm")]
        {
            if MODE == AccessMode::kAtomic {
                if true { //if V8_LIKELY(!WriteBarrier::IsEnabled())
                    return true;
                }
            }
        }
        #[cfg(not(target_arch = "arm"))]
        {
            return false;
        }
    }
}

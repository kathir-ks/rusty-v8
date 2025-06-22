// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/cppgc/object-start-bitmap.h

use std::{
    array::from_fn,
    mem::size_of,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::base::bits;

mod base;

pub mod cppgc {
    pub mod internal {
        use std::{
            array::from_fn,
            mem::size_of,
            sync::atomic::{AtomicU8, Ordering},
        };

        use crate::base::bits;

        //use super::{kAllocationGranularity, kPageSize, kAllocationMask, kPageBaseMask, kPageOffsetMask}; // Assuming these are defined elsewhere
        //use super::{HeapObjectHeader, Address, ConstAddress}; // Assuming these are defined elsewhere
        //use super::{WriteBarrier}; // Assuming this is defined elsewhere

        // Dummy definitions for constants and types used in the original C++ code.
        // Replace with actual definitions when available.
        const K_BLINK_PAGE_SIZE: usize = 4096; // Example value, replace with actual value.
        const K_ALLOCATION_GRANULARITY: usize = 8; // Example value, replace with actual value.
        const K_PAGE_SIZE: usize = K_BLINK_PAGE_SIZE;
        const K_ALLOCATION_MASK: usize = K_ALLOCATION_GRANULARITY - 1;
        const K_PAGE_BASE_MASK: usize = !(K_PAGE_SIZE - 1);
        const K_PAGE_OFFSET_MASK: usize = K_PAGE_SIZE - 1;

        type Address = usize;
        type ConstAddress = usize;

        pub struct HeapObjectHeader {}

        pub mod write_barrier {
            pub fn is_enabled() -> bool {
                false
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum AccessMode {
            KNonAtomic,
            KAtomic,
        }

        /// A bitmap for recording object starts. Objects have to be allocated at
        /// minimum granularity of `Granularity()`.
        ///
        /// Depends on internals such as:
        /// - `kBlinkPageSize`
        /// - `kAllocationGranularity`
        ///
        /// `ObjectStartBitmap` supports concurrent reads from multiple threads but
        /// only a single mutator thread can write to it. `ObjectStartBitmap` relies on
        /// being allocated inside the same normal page.
        pub struct ObjectStartBitmap {
            fully_populated_: bool,
            object_start_bit_map_: [u8; K_RESERVED_FOR_BITMAP],
        }

        const K_BITS_PER_CELL: usize = size_of::<u8>() * 8;
        const K_CELL_MASK: usize = K_BITS_PER_CELL - 1;
        const K_BITMAP_SIZE: usize =
            (K_PAGE_SIZE + ((K_BITS_PER_CELL * K_ALLOCATION_GRANULARITY) - 1))
            / (K_BITS_PER_CELL * K_ALLOCATION_GRANULARITY);
        const K_RESERVED_FOR_BITMAP: usize =
            (K_BITMAP_SIZE + K_ALLOCATION_MASK) & !K_ALLOCATION_MASK;

        impl ObjectStartBitmap {
            /// Granularity of addresses added to the bitmap.
            pub const fn granularity() -> usize {
                K_ALLOCATION_GRANULARITY
            }

            /// Maximum number of entries in the bitmap.
            pub const fn max_entries() -> usize {
                K_RESERVED_FOR_BITMAP * K_BITS_PER_CELL
            }

            pub fn new() -> Self {
                let mut bitmap = Self {
                    fully_populated_: false,
                    object_start_bit_map_: [0; K_RESERVED_FOR_BITMAP],
                };
                bitmap.clear();
                bitmap.mark_as_fully_populated();
                bitmap
            }

            /// Finds an object header based on an
            /// `address_maybe_pointing_to_the_middle_of_object`. Will search for an object
            /// start in decreasing address order.
            pub fn find_header<const MODE: AccessMode>(
                &self,
                address_maybe_pointing_to_the_middle_of_object: ConstAddress,
            ) -> *mut HeapObjectHeader {
                assert!(self.fully_populated_);
                let page_base = address_maybe_pointing_to_the_middle_of_object & K_PAGE_BASE_MASK;
                assert_eq!(page_base, (self as *const Self as usize) & K_PAGE_BASE_MASK);
                let object_offset =
                    address_maybe_pointing_to_the_middle_of_object & K_PAGE_OFFSET_MASK;
                let object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
                let mut cell_index = object_start_number / K_BITS_PER_CELL;
                assert!(self.object_start_bit_map_.len() > cell_index);
                let bit = object_start_number & K_CELL_MASK;
                let mut byte = self.load::<MODE>(cell_index) & ((1 << (bit + 1)) - 1);
                while byte == 0 && cell_index > 0 {
                    cell_index -= 1;
                    byte = self.load::<MODE>(cell_index);
                }
                let leading_zeroes = bits::count_leading_zeros(byte as usize) as usize;
                let object_start_number =
                    (cell_index * K_BITS_PER_CELL) + (K_BITS_PER_CELL - 1) - leading_zeroes;
                let object_offset = object_start_number * K_ALLOCATION_GRANULARITY;
                (page_base + object_offset) as *mut HeapObjectHeader
            }

            pub fn set_bit<const MODE: AccessMode>(&mut self, header_address: ConstAddress) {
                let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
                // Only a single mutator thread can write to the bitmap, so no need for CAS.
                let current_value = self.load::<AccessMode::KNonAtomic>(cell_index);
                self.store::<MODE>(
                    cell_index,
                    (current_value | (1 << object_bit)) as u8,
                );
            }

            pub fn clear_bit<const MODE: AccessMode>(&mut self, header_address: ConstAddress) {
                let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
                let current_value = self.load::<AccessMode::KNonAtomic>(cell_index);
                self.store::<MODE>(
                    cell_index,
                    (current_value & !(1 << object_bit)) as u8,
                );
            }

            pub fn check_bit<const MODE: AccessMode>(&self, header_address: ConstAddress) -> bool {
                let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
                self.load::<MODE>(cell_index) & (1 << object_bit) != 0
            }

            fn store<const MODE: AccessMode>(&mut self, cell_index: usize, value: u8) {
                if MODE == AccessMode::KNonAtomic {
                    self.object_start_bit_map_[cell_index] = value;
                    return;
                }
                // It appears that AsAtomicPtr is a way to obtain a mutable reference as an atomic
                // without consuming the original value. Since this is unsafe in Rust, we'll need to
                // use atomic types directly. This requires changing the type of the bit map array.
                let atomic_cell = &AtomicU8::new(self.object_start_bit_map_[cell_index]);
                atomic_cell.store(value, Ordering::Release);
                self.object_start_bit_map_[cell_index] = value;
            }

            fn load<const MODE: AccessMode>(&self, cell_index: usize) -> u8 {
                if MODE == AccessMode::KNonAtomic {
                    return self.object_start_bit_map_[cell_index];
                }
                let atomic_cell = &AtomicU8::new(self.object_start_bit_map_[cell_index]);
                atomic_cell.load(Ordering::Acquire)
            }

            fn object_start_index_and_bit(
                &self,
                header_address: ConstAddress,
            ) -> (usize, usize) {
                let object_offset = header_address & K_PAGE_OFFSET_MASK;
                assert!(object_offset & K_ALLOCATION_MASK == 0);
                let object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
                let cell_index = object_start_number / K_BITS_PER_CELL;
                assert!(K_BITMAP_SIZE > cell_index);
                let bit = object_start_number & K_CELL_MASK;
                (cell_index, bit)
            }

            /// Iterates all object starts recorded in the bitmap.
            ///
            /// The callback is of type
            ///   `FnMut(Address)`
            /// and is passed the object start address as parameter.
            pub fn iterate<F>(&self, mut callback: F)
            where
                F: FnMut(Address),
            {
                let page_base = (self as *const Self as usize) & K_PAGE_BASE_MASK;
                for cell_index in 0..K_RESERVED_FOR_BITMAP {
                    if self.object_start_bit_map_[cell_index] == 0 {
                        continue;
                    }

                    let mut value = self.object_start_bit_map_[cell_index];
                    while value != 0 {
                        let trailing_zeroes = bits::count_trailing_zeros(value as usize) as usize;
                        let object_start_number =
                            (cell_index * K_BITS_PER_CELL) + trailing_zeroes;
                        let object_address =
                            page_base + (K_ALLOCATION_GRANULARITY * object_start_number);
                        callback(object_address);
                        // Clear current object bit in temporary value to advance iteration.
                        value &= !(1 << (object_start_number & K_CELL_MASK));
                    }
                }
            }

            /// Marks the bitmap as fully populated. Unpopulated bitmaps are in an
            /// inconsistent state and must be populated before they can be used to find
            /// object headers.
            pub fn mark_as_fully_populated(&mut self) {
                assert!(!self.fully_populated_);
                self.fully_populated_ = true;
            }

            /// Clear the object start bitmap.
            pub fn clear(&mut self) {
                self.fully_populated_ = false;
                self.object_start_bit_map_.fill(0);
            }
        }

        /// A platform aware version of `ObjectStartBitmap` to provide platform specific
        /// optimizations (e.g. Use non-atomic stores on ARMv7 when not marking).
        pub struct PlatformAwareObjectStartBitmap {
            base: ObjectStartBitmap,
        }

        impl PlatformAwareObjectStartBitmap {
            pub fn new() -> Self {
                Self {
                    base: ObjectStartBitmap::new(),
                }
            }

            pub fn set_bit<const MODE: AccessMode>(&mut self, header_address: ConstAddress) {
                if Self::should_force_non_atomic::<MODE>() {
                    self.base.set_bit::<AccessMode::KNonAtomic>(header_address);
                    return;
                }
                self.base.set_bit::<MODE>(header_address);
            }

            pub fn clear_bit<const MODE: AccessMode>(&mut self, header_address: ConstAddress) {
                if Self::should_force_non_atomic::<MODE>() {
                    self.base.clear_bit::<AccessMode::KNonAtomic>(header_address);
                    return;
                }
                self.base.clear_bit::<MODE>(header_address);
            }

            fn should_force_non_atomic<const MODE: AccessMode>() -> bool {
                // This is a placeholder for architecture-specific optimization logic
                // like the ARMv7 check in the original C++ code.
                #[cfg(all(target_arch = "arm"))]
                {
                    if MODE == AccessMode::KAtomic {
                        if !write_barrier::is_enabled() {
                            return true;
                        }
                    }
                }

                false
            }
             pub fn find_header<const MODE: AccessMode>(
                &self,
                address_maybe_pointing_to_the_middle_of_object: ConstAddress,
            ) -> *mut HeapObjectHeader {
                self.base.find_header::<MODE>(address_maybe_pointing_to_the_middle_of_object)
            }

             pub fn check_bit<const MODE: AccessMode>(&self, header_address: ConstAddress) -> bool {
                self.base.check_bit::<MODE>(header_address)
            }

             pub fn iterate<F>(&self, callback: F)
                where
                    F: FnMut(Address),{
                self.base.iterate(callback)
            }
             pub fn mark_as_fully_populated(&mut self) {
                 self.base.mark_as_fully_populated()
             }

            /// Clear the object start bitmap.
            pub fn clear(&mut self) {
                self.base.clear();
            }
        }
    }
}
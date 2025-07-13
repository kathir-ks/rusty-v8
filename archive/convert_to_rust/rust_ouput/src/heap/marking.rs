// Converted from V8 C++ source files:
// Header: marking.h
// Implementation: marking.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::heap::marking_worklist::MarkingWorklists;
use crate::objects::heap_object::HeapObject;
use crate::heap::page_metadata::PageMetadata;
use crate::heap::spaces::Address;

const kBitsPerByte: usize = 8;
const kPageSizeBits: usize = 17;
const kTaggedSizeLog2: usize = 1;

pub struct MarkBit {
    cell_: *mut usize,
    mask_: usize,
}

impl MarkBit {
    pub type CellType = usize;

    #[allow(unused_variables)]
    pub fn From(address: Address) -> MarkBit {
        MarkBit {
            cell_: std::ptr::null_mut(),
            mask_: 0,
        }
    }

    #[allow(unused_variables)]
    pub fn From_tagged(heap_object: Tagged<HeapObject>) -> MarkBit {
        MarkBit {
            cell_: std::ptr::null_mut(),
            mask_: 0,
        }
    }

    #[allow(unused_variables)]
    pub fn FromForTesting(address: Address) -> MarkBit {
        MarkBit {
            cell_: std::ptr::null_mut(),
            mask_: 0,
        }
    }

    #[allow(unused_variables)]
    pub fn FromForTesting_tagged(heap_object: Tagged<HeapObject>) -> MarkBit {
        MarkBit {
            cell_: std::ptr::null_mut(),
            mask_: 0,
        }
    }

    pub fn Set(&self) -> bool {
        unsafe {
            let old_value = *self.cell_;
            if (old_value & self.mask_) == self.mask_ {
                return false;
            }
            *self.cell_ = old_value | self.mask_;
            true
        }
    }

    pub fn Get(&self) -> bool {
        unsafe { (*self.cell_ & self.mask_) != 0 }
    }

    pub fn Clear(&self) -> bool {
        unsafe {
            let old_value = *self.cell_;
            *self.cell_ = old_value & !self.mask_;
            (old_value & self.mask_) == self.mask_
        }
    }

    pub fn CellAddress(&self) -> *const CellType {
        self.cell_
    }

    pub fn Mask(&self) -> CellType {
        self.mask_
    }
}

impl PartialEq for MarkBit {
    fn eq(&self, other: &Self) -> bool {
        self.cell_ == other.cell_ && self.mask_ == other.mask_
    }
}

pub struct MarkingBitmap {
    cells_: [usize; MarkingBitmap::kCellsCount],
}

impl MarkingBitmap {
    pub type CellType = MarkBit::CellType;
    pub type CellIndex = u32;
    pub type MarkBitIndex = u32;

    pub const kBitsPerCell: u32 = (std::mem::size_of::<CellType>() * kBitsPerByte) as u32;
    pub const kBitsPerCellLog2: u32 = Self::kBitsPerCell.trailing_zeros();
    pub const kBitIndexMask: u32 = Self::kBitsPerCell - 1;
    pub const kBytesPerCell: u32 = Self::kBitsPerCell / kBitsPerByte as u32;
    pub const kBytesPerCellLog2: u32 = Self::kBitsPerCellLog2 - (kBitsPerByte as u32).trailing_zeros();
    pub const kLength: usize = ((1 << kPageSizeBits) >> kTaggedSizeLog2);
    pub const kCellsCount: usize = (Self::kLength + Self::kBitsPerCell as usize - 1) / Self::kBitsPerCell as usize;
    pub const kSize: usize = Self::kCellsCount * Self::kBytesPerCell as usize;

    pub const fn AddressToIndex(address: Address) -> MarkBitIndex {
        (address as usize >> kTaggedSizeLog2) as u32
    }

    pub const fn LimitAddressToIndex(address: Address) -> MarkBitIndex {
        (address as usize >> kTaggedSizeLog2) as u32
    }

    pub const fn IndexToCell(index: MarkBitIndex) -> CellIndex {
        index >> Self::kBitsPerCellLog2
    }

    pub const fn IndexToAddressOffset(index: MarkBitIndex) -> Address {
        (index << kTaggedSizeLog2) as Address
    }

    pub const fn CellToBase(cell_index: CellIndex) -> Address {
        Self::IndexToAddressOffset(cell_index << Self::kBitsPerCellLog2)
    }

    pub const fn IndexInCell(index: MarkBitIndex) -> u32 {
        index & Self::kBitIndexMask
    }

    pub const fn IndexInCellMask(index: MarkBitIndex) -> CellType {
        1 << Self::IndexInCell(index)
    }

    pub fn CellAlignIndex(index: u32) -> u32 {
        index & !Self::kBitIndexMask
    }

    pub fn Cast(addr: Address) -> *mut MarkingBitmap {
        addr as *mut MarkingBitmap
    }

    pub fn MarkBitFromAddress(address: Address) -> MarkBit {
        unsafe {
            let bitmap_addr = (address as usize & !((1 << kPageSizeBits) - 1)) as *mut MarkingBitmap;
            let bitmap = &mut *bitmap_addr;
            Self::MarkBitFromAddress_bitmap(bitmap, address)
        }
    }

    pub fn MarkBitFromAddress_bitmap(bitmap: *mut MarkingBitmap, address: Address) -> MarkBit {
        unsafe {
            let index = Self::AddressToIndex(address);
            let mask = Self::IndexInCellMask(index);
            let cell_index = Self::IndexToCell(index) as usize;
            let cell = (*bitmap).cells_.as_mut_ptr().add(cell_index);
            MarkBit { cell_: cell, mask_: mask }
        }
    }

    pub fn new() -> MarkingBitmap {
        MarkingBitmap {
            cells_: [0; MarkingBitmap::kCellsCount],
        }
    }

    pub fn cells(&mut self) -> &mut [CellType] {
        &mut self.cells_
    }

    pub fn cells_const(&self) -> &[CellType] {
        &self.cells_
    }

    pub fn AllBitsClearInRange(&self, start_index: MarkBitIndex, end_index: MarkBitIndex) -> bool {
        if start_index >= end_index {
            return true;
        }
        let mut end_index = end_index;
        end_index -= 1;

        let start_cell_index = Self::IndexToCell(start_index);
        let start_index_mask = Self::IndexInCellMask(start_index);
        let end_cell_index = Self::IndexToCell(end_index);
        let end_index_mask = Self::IndexInCellMask(end_index);

        let mut matching_mask: CellType;
        if start_cell_index != end_cell_index {
            matching_mask = !(start_index_mask - 1);
            if (self.cells_[start_cell_index as usize] & matching_mask) != 0 {
                return false;
            }
            for i in (start_cell_index + 1)..end_cell_index {
                if self.cells_[i as usize] != std::usize::MAX {
                    return false;
                }
            }
            matching_mask = end_index_mask | (end_index_mask - 1);
            return (self.cells_[end_cell_index as usize] & matching_mask) == 0;
        } else {
            matching_mask = end_index_mask | (end_index_mask - start_index_mask);
            return (self.cells_[end_cell_index as usize] & matching_mask) == 0;
        }
    }

    pub fn AllBitsSetInRange(&self, start_index: MarkBitIndex, end_index: MarkBitIndex) -> bool {
        if start_index >= end_index {
            return false;
        }
        let mut end_index = end_index;
        end_index -= 1;

        let start_cell_index = Self::IndexToCell(start_index);
        let start_index_mask = Self::IndexInCellMask(start_index);
        let end_cell_index = Self::IndexToCell(end_index);
        let end_index_mask = Self::IndexInCellMask(end_index);

        let mut matching_mask: CellType;
        if start_cell_index != end_cell_index {
            matching_mask = !(start_index_mask - 1);
            if (self.cells_[start_cell_index as usize] & matching_mask) != matching_mask {
                return false;
            }
            for i in (start_cell_index + 1)..end_cell_index {
                if self.cells_[i as usize] != std::usize::MAX {
                    return false;
                }
            }
            matching_mask = end_index_mask | (end_index_mask - 1);
            return (self.cells_[end_cell_index as usize] & matching_mask) == matching_mask;
        } else {
            matching_mask = end_index_mask | (end_index_mask - start_index_mask);
            return (self.cells_[end_cell_index as usize] & matching_mask) == matching_mask;
        }
    }

    pub fn Clear(&mut self) {
        for cell in &mut self.cells_ {
            *cell = 0;
        }
    }

    pub fn SetRange(&mut self, start_index: MarkBitIndex, end_index: MarkBitIndex) {
        let start_cell_index = Self::IndexToCell(start_index);
        let end_cell_index = Self::IndexToCell(end_index);

        if start_cell_index == end_cell_index {
            let mask = ((1 as CellType) << (end_index - start_index)) - 1 << Self::IndexInCell(start_index);
            unsafe {
                *self.cells_.as_mut_ptr().add(start_cell_index as usize) |= mask;
            }
            return;
        }

        let start_mask = !((1 as CellType) << Self::IndexInCell(start_index) - 1);
        unsafe {
            *self.cells_.as_mut_ptr().add(start_cell_index as usize) |= start_mask;
        }

        for i in start_cell_index + 1..end_cell_index {
            unsafe {
                *self.cells_.as_mut_ptr().add(i as usize) = std::usize::MAX;
            }
        }

        let end_mask = ((1 as CellType) << Self::IndexInCell(end_index)) - 1;
        unsafe {
            *self.cells_.as_mut_ptr().add(end_cell_index as usize) |= end_mask;
        }
    }

    pub fn ClearRange(&mut self, start_index: MarkBitIndex, end_index: MarkBitIndex) {
        let start_cell_index = Self::IndexToCell(start_index);
        let end_cell_index = Self::IndexToCell(end_index);

        if start_cell_index == end_cell_index {
            let mask = ((1 as CellType) << (end_index - start_index)) - 1 << Self::IndexInCell(start_index);
            unsafe {
                *self.cells_.as_mut_ptr().add(start_cell_index as usize) &= !mask;
            }
            return;
        }

        let start_mask = (1 as CellType) << Self::IndexInCell(start_index) - 1;
        unsafe {
            *self.cells_.as_mut_ptr().add(start_cell_index as usize) &= start_mask;
        }

        for i in start_cell_index + 1..end_cell_index {
            unsafe {
                *self.cells_.as_mut_ptr().add(i as usize) = 0;
            }
        }

        let end_mask = !(((1 as CellType) << Self::IndexInCell(end_index)) - 1);
        unsafe {
            *self.cells_.as_mut_ptr().add(end_cell_index as usize) &= end_mask;
        }
    }

    pub fn IsClean(&self) -> bool {
        for cell in &self.cells_ {
            if *cell != 0 {
                return false;
            }
        }
        true
    }

    pub fn Print(&self) {
        println!("MarkingBitmap::Print() not fully implemented");
    }

    pub fn MarkBitFromIndexForTesting(&self, index: u32) -> MarkBit {
        let mask = Self::IndexInCellMask(index);
        let cell = &self.cells_[Self::IndexToCell(index) as usize] as *const CellType as *mut CellType;
        MarkBit {
            cell_: cell,
            mask_: mask,
        }
    }

    pub fn FindPreviousValidObject(page: *const PageMetadata, maybe_inner_ptr: Address) -> Address {
        maybe_inner_ptr
    }

    pub fn FromAddress(address: Address) -> *mut MarkingBitmap {
        address as *mut MarkingBitmap
    }

    pub fn SetBitsInCell(&mut self, cell_index: u32, mask: CellType) {
        self.cells_[cell_index as usize] |= mask;
    }

    pub fn ClearBitsInCell(&mut self, cell_index: u32, mask: CellType) {
        self.cells_[cell_index as usize] &= !mask;
    }

    pub fn SetCellRangeRelaxed(&mut self, start_cell_index: u32, end_cell_index: u32) {
        for i in start_cell_index..end_cell_index {
            self.cells_[i as usize] = std::usize::MAX;
        }
    }

    pub fn ClearCellRangeRelaxed(&mut self, start_cell_index: u32, end_cell_index: u32) {
        for i in start_cell_index..end_cell_index {
            self.cells_[i as usize] = 0;
        }
    }
}

pub struct MarkingHelper {}

impl MarkingHelper {
    pub fn ShouldMarkObject(heap: *mut Heap, object: Tagged<HeapObject>) -> Option<WorklistTarget> {
        Some(WorklistTarget::kRegular)
    }

    pub fn GetLivenessMode(heap: *mut Heap, object: Tagged<HeapObject>) -> LivenessMode {
        LivenessMode::kMarkbit
    }

    pub fn IsMarkedOrAlwaysLive<MarkingStateT>(
        heap: *mut Heap,
        marking_state: &mut MarkingStateT,
        object: Tagged<HeapObject>,
    ) -> bool {
        true
    }

    pub fn IsUnmarkedAndNotAlwaysLive<MarkingStateT>(
        heap: *mut Heap,
        marking_state: &mut MarkingStateT,
        object: Tagged<HeapObject>,
    ) -> bool {
        false
    }

    pub fn TryMarkAndPush<MarkingState>(
        heap: *mut Heap,
        marking_worklist: *mut MarkingWorklists::Local,
        marking_state: &mut MarkingState,
        target_worklis: WorklistTarget,
        object: Tagged<HeapObject>,
    ) -> bool {
        true
    }
}

pub enum WorklistTarget {
    kRegular,
}

pub enum LivenessMode {
    kMarkbit,
    kAlwaysLive,
}

// Dummy structs and enums needed for compilation
pub struct Heap {}
pub struct Tagged<T> {
    ptr: Address,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn ptr(&self) -> Address {
        self.ptr
    }
}

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of v8/src/heap/marking-inl.h

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::{
    marker::PhantomData,
    num::Wrapping,
    sync::atomic::{AtomicU32, Ordering},
};

mod base {
    pub mod bits {
        pub fn CountLeadingZeros(x: u32) -> u32 {
            x.leading_zeros()
        }
    }

    pub mod AsAtomicWord {
        use std::sync::atomic::{AtomicU32, Ordering};

        pub fn Relaxed_SetBits(target: &AtomicU32, value: u32, mask: u32) {
            let mut current = target.load(Ordering::Relaxed);
            loop {
                let new_value = (current & !mask) | (value & mask);
                match target.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(x) => current = x,
                }
            }
        }
    }

    pub use std::sync::atomic::fence as SeqCst_MemoryFence;

    pub mod atomic {
        use std::sync::atomic::{AtomicU32, Ordering};

        pub fn Relaxed_Store(target: &AtomicU32, value: u32) {
            target.store(value, Ordering::Relaxed);
        }
    }
}

mod heap {
    use std::sync::atomic::AtomicU32;

    pub const kTaggedSizeLog2: usize = 3;

    #[derive(Debug, PartialEq, Eq)]
    pub enum AccessMode {
        NON_ATOMIC,
        ATOMIC,
    }

    pub struct MarkBit {
        cell: *mut u32, // Assuming CellType is u32
        mask: u32,
    }

    impl MarkBit {
        pub fn from(address: usize) -> Self {
            MarkingBitmap::MarkBitFromAddress(address)
        }
    }

    pub struct MarkingBitmap {
        cells: Box<[AtomicU32]>,
        kCellsCount: usize,
        kBitsPerCell: usize,
        kLength: usize,
        area_start: usize,
        chunk_address: usize,
    }

    impl MarkingBitmap {
        pub fn new(
            cells: Box<[AtomicU32]>,
            kCellsCount: usize,
            kBitsPerCell: usize,
            kLength: usize,
            area_start: usize,
            chunk_address: usize,
        ) -> Self {
            MarkingBitmap {
                cells,
                kCellsCount,
                kBitsPerCell,
                kLength,
                area_start,
                chunk_address,
            }
        }

        fn cells(&self) -> &[AtomicU32] {
            &self.cells
        }

        pub fn SetBitsInCell<const MODE: usize>(&self, cell_index: u32, mask: u32)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if MODE == 0 {
                self.cells[cell_index as usize].fetch_or(mask, Ordering::Relaxed);
            } else {
                base::AsAtomicWord::Relaxed_SetBits(&self.cells[cell_index as usize], mask, mask);
            }
        }

        pub fn ClearBitsInCell<const MODE: usize>(&self, cell_index: u32, mask: u32)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if MODE == 0 {
                self.cells[cell_index as usize].fetch_and(!mask, Ordering::Relaxed);
            } else {
                base::AsAtomicWord::Relaxed_SetBits(
                    &self.cells[cell_index as usize],
                    0,
                    mask,
                );
            }
        }

        pub fn ClearCellRangeRelaxed<const MODE: usize>(&self, start_cell_index: u32, end_cell_index: u32)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if MODE == 0 {
                for i in start_cell_index..end_cell_index {
                    self.cells[i as usize].store(0, Ordering::Relaxed);
                }
            } else {
                for i in start_cell_index..end_cell_index {
                    base::atomic::Relaxed_Store(&self.cells[i as usize], 0);
                }
            }
        }

        pub fn SetCellRangeRelaxed<const MODE: usize>(&self, start_cell_index: u32, end_cell_index: u32)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if MODE == 0 {
                for i in start_cell_index..end_cell_index {
                    self.cells[i as usize].store(u32::MAX, Ordering::Relaxed);
                }
            } else {
                for i in start_cell_index..end_cell_index {
                    base::atomic::Relaxed_Store(&self.cells[i as usize], u32::MAX);
                }
            }
        }

        pub fn Clear<const MODE: usize>(&self)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            self.ClearCellRangeRelaxed::<MODE>(0, self.kCellsCount as u32);
            if MODE == 1 {
                base::SeqCst_MemoryFence();
            }
        }

        type MarkBitIndex = usize;
        type CellIndex = usize;
        type CellType = u32;

        fn IndexToCell(index: Self::MarkBitIndex) -> Self::CellIndex {
            index / Self::kBitsPerCell
        }

        fn IndexInCell(index: Self::MarkBitIndex) -> usize {
            index % Self::kBitsPerCell
        }

        fn IndexInCellMask(index: Self::MarkBitIndex) -> Self::CellType {
            1 << (Self::IndexInCell(index) as u32)
        }

        pub fn SetRange<const MODE: usize>(&self, start_index: Self::MarkBitIndex, end_index: Self::MarkBitIndex)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if start_index >= end_index {
                return;
            }

            let mut end_index = end_index - 1;

            let start_cell_index = Self::IndexToCell(start_index);
            let start_index_mask = Self::IndexInCellMask(start_index);
            let end_cell_index = Self::IndexToCell(end_index);
            let end_index_mask = Self::IndexInCellMask(end_index);

            if start_cell_index != end_cell_index {
                // Firstly, fill all bits from the start address to the end of the first
                // cell with 1s.
                self.SetBitsInCell::<MODE>(start_cell_index as u32, !(start_index_mask - 1));
                // Then fill all in between cells with 1s.
                self.SetCellRangeRelaxed::<MODE>(
                    (start_cell_index + 1) as u32,
                    end_cell_index as u32,
                );
                // Finally, fill all bits until the end address in the last cell with 1s.
                self.SetBitsInCell::<MODE>(
                    end_cell_index as u32,
                    end_index_mask | (end_index_mask - 1),
                );
            } else {
                self.SetBitsInCell::<MODE>(
                    start_cell_index as u32,
                    end_index_mask | (end_index_mask - start_index_mask),
                );
            }

            if MODE == 1 {
                base::SeqCst_MemoryFence();
            }
        }

        pub fn ClearRange<const MODE: usize>(&self, start_index: Self::MarkBitIndex, end_index: Self::MarkBitIndex)
        where
            [(); (MODE)]:, // Compile-time check for MODE = 0 or 1
        {
            if start_index >= end_index {
                return;
            }
            let mut end_index = end_index - 1;

            let start_cell_index = Self::IndexToCell(start_index);
            let start_index_mask = Self::IndexInCellMask(start_index);
            let end_cell_index = Self::IndexToCell(end_index);
            let end_index_mask = Self::IndexInCellMask(end_index);

            if start_cell_index != end_cell_index {
                // Firstly, fill all bits from the start address to the end of the first
                // cell with 0s.
                self.ClearBitsInCell::<MODE>(start_cell_index as u32, !(start_index_mask - 1));
                // Then fill all in between cells with 0s.
                self.ClearCellRangeRelaxed::<MODE>(
                    (start_cell_index + 1) as u32,
                    end_cell_index as u32,
                );
                // Finally, set all bits until the end address in the last cell with 0s.
                self.ClearBitsInCell::<MODE>(
                    end_cell_index as u32,
                    end_index_mask | (end_index_mask - 1),
                );
            } else {
                self.ClearBitsInCell::<MODE>(
                    start_cell_index as u32,
                    end_index_mask | (end_index_mask - start_index_mask),
                );
            }

            if MODE == 1 {
                base::SeqCst_MemoryFence();
            }
        }

        // static
        pub fn FromAddress(address: usize) -> *mut MarkingBitmap {
            let metadata_address = MutablePageMetadata::FromAddress(address).MetadataAddress();
            (metadata_address + MutablePageMetadata::MarkingBitmapOffset()) as *mut MarkingBitmap
        }

        // static
        pub fn MarkBitFromAddress(address: usize) -> MarkBit {
            Self::MarkBitFromAddress_internal(Self::FromAddress(address), address)
        }

        // static
        fn MarkBitFromAddress_internal(bitmap: *mut MarkingBitmap, address: usize) -> MarkBit {
            //DCHECK_EQ(bitmap, FromAddress(address));
            let bitmap_ref = unsafe { &*bitmap };
            let index = Self::AddressToIndex(address);
            let mask = Self::IndexInCellMask(index);
            let cell_index = Self::IndexToCell(index);
            let cell = &bitmap_ref.cells()[cell_index];
            MarkBit {
                cell: cell as *const AtomicU32 as *mut u32,
                mask,
            }
        }

        // static
        const fn AddressToIndex(address: usize) -> Self::MarkBitIndex {
            MemoryChunk::AddressToOffset(address) >> kTaggedSizeLog2
        }

        // static
        const fn LimitAddressToIndex(address: usize) -> Self::MarkBitIndex {
            if MemoryChunk::IsAligned(address) {
                return Self::kLength;
            }
            Self::AddressToIndex(address)
        }

        // static
        fn FindPreviousValidObject(page: &PageMetadata, maybe_inner_ptr: usize) -> usize {
            //DCHECK(page.Contains(maybe_inner_ptr));
            let bitmap = page.marking_bitmap();
            let cells = &bitmap.cells();

            // The first actual bit of the bitmap, corresponding to page->area_start(),
            // is at start_index which is somewhere in (not necessarily at the start of)
            // start_cell_index.
            let start_index = MarkingBitmap::AddressToIndex(page.area_start);
            let start_cell_index = MarkingBitmap::IndexToCell(start_index);
            // We assume that all markbits before start_index are clear:
            // SLOW_DCHECK(bitmap->AllBitsClearInRange(0, start_index));
            // This has already been checked for the entire bitmap before starting marking
            // by MarkCompactCollector::VerifyMarkbitsAreClean.

            let index = MarkingBitmap::AddressToIndex(maybe_inner_ptr);
            let mut cell_index = MarkingBitmap::IndexToCell(index);
            let index_in_cell = MarkingBitmap::IndexInCell(index);
            //DCHECK_GT(MarkingBitmap::kBitsPerCell, index_in_cell);
            let mut cell = cells[cell_index].load(Ordering::Relaxed);

            // Clear the bits corresponding to higher addresses in the cell.
            cell &= ((!0) >> (MarkingBitmap::kBitsPerCell - index_in_cell - 1));

            // Traverse the bitmap backwards, until we find a markbit that is set and
            // whose previous markbit (if it exists) is unset.
            // First, iterate backwards to find a cell with any set markbit.
            while cell == 0 && cell_index > start_cell_index {
                cell_index -= 1;
                cell = cells[cell_index].load(Ordering::Relaxed);
            }
            if cell == 0 {
                //DCHECK_EQ(start_cell_index, cell_index);
                // We have reached the start of the page.
                return page.area_start;
            }

            // We have found such a cell.
            let leading_zeros = base::bits::CountLeadingZeros(cell);
            let leftmost_ones = base::bits::CountLeadingZeros(!(cell << leading_zeros));
            let index_of_last_leftmost_one =
                MarkingBitmap::kBitsPerCell as u32 - leading_zeros - leftmost_ones;

            let chunk = page.chunk; // Replaced page.Chunk() with a field access

            // If the leftmost sequence of set bits does not reach the start of the cell,
            // we found it.
            if index_of_last_leftmost_one > 0 {
                return chunk.address
                    + MarkingBitmap::IndexToAddressOffset(
                        cell_index * MarkingBitmap::kBitsPerCell
                            + index_of_last_leftmost_one as usize,
                    );
            }

            // The leftmost sequence of set bits reaches the start of the cell. We must
            // keep traversing backwards until we find the first unset markbit.
            if cell_index == start_cell_index {
                // We have reached the start of the page.
                return page.area_start;
            }

            // Iterate backwards to find a cell with any unset markbit.
            while (cell == u32::MAX) && cell_index > start_cell_index {
                cell_index -= 1;
                cell = cells[cell_index].load(Ordering::Relaxed);
            }
            if cell == u32::MAX {
                //DCHECK_EQ(start_cell_index, cell_index);
                // We have reached the start of the page.
                return page.area_start;
            }

            // We have found such a cell.
            let leading_ones = base::bits::CountLeadingZeros(!cell);
            let index_of_last_leading_one =
                MarkingBitmap::kBitsPerCell as u32 - leading_ones;
            //DCHECK_LT(0, index_of_last_leading_one);
            return chunk.address
                + MarkingBitmap::IndexToAddressOffset(
                    cell_index * MarkingBitmap::kBitsPerCell + index_of_last_leading_one as usize,
                );
        }

        const fn IndexToAddressOffset(index: usize) -> usize {
            index << kTaggedSizeLog2
        }
    }

    #[derive(Clone, Copy)]
    pub struct HeapObject {
        ptr: usize,
    }

    impl HeapObject {
        pub fn ptr(&self) -> usize {
            self.ptr
        }
    }

    #[derive(Clone, Copy)]
    pub struct Tagged<T> {
        object: T,
    }

    impl Tagged<HeapObject> {
        pub fn ptr(&self) -> usize {
            self.object.ptr()
        }
    }

    impl From<HeapObject> for Tagged<HeapObject> {
        fn from(object: HeapObject) -> Self {
            Tagged { object }
        }
    }

    pub struct MemoryChunk {
        address: usize,
        flags: u32,
    }

    impl MemoryChunk {
        pub const READ_ONLY_HEAP: u32 = 1 << 0;
        pub const BLACK_ALLOCATED: u32 = 1 << 1;
        pub const IN_WRITABLE_SHARED_SPACE: u32 = 1 << 2;

        pub fn IsAligned(address: usize) -> bool {
            address % 8 == 0
        }

        pub const kIsInYoungGenerationMask: u32 = 1 << 3;

        pub fn AddressToOffset(address: usize) -> usize {
            address & 0xFFFFFFFF // Example: Assuming offset fits in 32 bits.
        }

        pub fn FromHeapObject(object: Tagged<HeapObject>) -> &'static MemoryChunk {
            //This is a placeholder since MemoryChunk creation/management wasn't provided.
            unsafe { &*(object.ptr() as *const MemoryChunk) }
        }

        pub fn GetFlags(&self) -> u32 {
            self.flags
        }
    }

    pub struct PageMetadata {
        area_start: usize,
        chunk: &'static MemoryChunk,
        marking_bitmap_ptr: *mut MarkingBitmap,
    }

    impl PageMetadata {
        pub fn marking_bitmap(&self) -> &MarkingBitmap {
            unsafe { &*self.marking_bitmap_ptr }
        }
    }

    pub struct MutablePageMetadata {}

    impl MutablePageMetadata {
        pub fn FromAddress(address: usize) -> &'static MutablePageMetadata {
            unsafe { &*(address as *const MutablePageMetadata) }
        }

        pub fn MetadataAddress(&self) -> usize {
            0 // Placeholder
        }
        pub fn MarkingBitmapOffset() -> usize {
            0 // Placeholder
        }
    }

    pub struct Heap {
        isolate: Isolate,
    }

    impl Heap {
        pub fn isolate(&self) -> &Isolate {
            &self.isolate
        }

        pub fn Contains(&self, object: Tagged<HeapObject>) -> bool {
            true // Placeholder: Need a better heap containment check
        }
    }

    pub struct Isolate {
        shared_space_isolate: bool,
    }

    impl Isolate {
        pub fn is_shared_space_isolate(&self) -> bool {
            self.shared_space_isolate
        }
    }

    pub mod MarkingHelper {
        use super::{Heap, HeapObject, MemoryChunk, Tagged};
        use std::option::Option;

        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum WorklistTarget {
            kRegular,
        }

        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum LivenessMode {
            kAlwaysLive,
            kMarkbit,
        }

        pub fn ShouldMarkObject(
            heap: &Heap,
            object: Tagged<HeapObject>,
        ) -> Option<WorklistTarget> {
            let chunk = MemoryChunk::FromHeapObject(object);
            let flags = chunk.GetFlags();
            if flags & MemoryChunk::READ_ONLY_HEAP != 0 {
                return None;
            }

            if V8_FLAGS.black_allocated_pages && (flags & MemoryChunk::BLACK_ALLOCATED != 0) {
                if flags & MemoryChunk::kIsInYoungGenerationMask != 0 {
                    //DCHECK!
                    unreachable!();
                }
                return None;
            }

            if flags & MemoryChunk::IN_WRITABLE_SHARED_SPACE == 0 {
                return Some(WorklistTarget::kRegular);
            }

            //Object in shared writable space. Only mark it if the Isolate is owning the
            //shared space.
            //TODO(340989496): Speed up check here by keeping the flag on Heap.
            if heap.isolate().is_shared_space_isolate() {
                return Some(WorklistTarget::kRegular);
            }

            None
        }

        pub fn GetLivenessMode(heap: &Heap, object: Tagged<HeapObject>) -> LivenessMode {
            let chunk = MemoryChunk::FromHeapObject(object);
            let flags = chunk.GetFlags();
            if flags & MemoryChunk::READ_ONLY_HEAP != 0 {
                return LivenessMode::kAlwaysLive;
            }

            if V8_FLAGS.black_allocated_pages && (flags & MemoryChunk::BLACK_ALLOCATED != 0) {
                return LivenessMode::kAlwaysLive;
            }

            if flags & MemoryChunk::IN_WRITABLE_SHARED_SPACE == 0 {
                return LivenessMode::kMarkbit;
            }

            //Object in shared writable space. Only mark it if the Isolate is owning the
            //shared space.
            //TODO(340989496): Speed up check here by keeping the flag on Heap.
            if heap.isolate().is_shared_space_isolate() {
                return LivenessMode::kMarkbit;
            }

            LivenessMode::kAlwaysLive
        }

        pub fn IsMarkedOrAlwaysLive<MarkingStateT: MarkingState>(
            heap: &Heap,
            marking_state: &MarkingStateT,
            object: Tagged<HeapObject>,
        ) -> bool {
            MarkingHelper::GetLivenessMode(heap, object) == LivenessMode::kAlwaysLive
                || marking_state.IsMarked(object)
        }

        pub fn IsUnmarkedAndNotAlwaysLive<MarkingStateT: MarkingState>(
            heap: &Heap,
            marking_state: &MarkingStateT,
            object: Tagged<HeapObject>,
        ) -> bool {
            MarkingHelper::GetLivenessMode(heap, object) != LivenessMode::kAlwaysLive
                && marking_state.IsUnmarked(object)
        }

        pub fn TryMarkAndPush<MarkingState: MarkingState>(
            heap: &Heap,
            marking_worklist: &mut MarkingWorklistsLocal,
            marking_state: &mut MarkingState,
            target_worklist: WorklistTarget,
            object: Tagged<HeapObject>,
        ) -> bool {
            //DCHECK(heap.Contains(object));
            if marking_state.TryMark(object) {
                if target_worklist == WorklistTarget::kRegular {
                    marking_worklist.Push(object);
                }
                return true;
            }
            false
        }

        pub trait MarkingState {
            fn IsMarked(&self, object: Tagged<HeapObject>) -> bool;
            fn IsUnmarked(&self, object: Tagged<HeapObject>) -> bool;
            fn TryMark(&mut self, object: Tagged<HeapObject>) -> bool;
        }
    }

    pub struct MarkingWorklistsLocal {}
    impl MarkingWorklistsLocal {
        fn Push(&mut self, object: Tagged<HeapObject>) {} // Placeholder
    }
}

const V8_FLAGS: Flags = Flags {
    black_allocated_pages: false,
};

struct Flags {
    black_allocated_pages: bool,
}
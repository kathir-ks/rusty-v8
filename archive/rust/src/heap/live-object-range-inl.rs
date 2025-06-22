// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod live_object_range {
    use crate::heap::{
        heap_inl::Heap,
        page_metadata::PageMetadata,
        page_metadata_inl::MemoryChunk,
    };
    use crate::objects::{instance_type_inl::InstanceTypeChecker};
    use crate::heap::marking_bitmap::{MarkingBitmap, MarkBit};
    use crate::objects::map::MapWord;
    use std::ptr::NonNull;
    use std::mem::size_of;

    const K_TAGGED_SIZE: usize = 8; // Assuming tagged size is 8 bytes
    const K_ALIGNMENT: usize = 8; // Assuming alignment is 8 bytes
    const K_NULL_ADDRESS: usize = 0; // Assuming null address is 0

    #[derive(Clone, Copy)]
    pub struct HeapObject {
        address: usize, // Using usize to represent address
    }

    impl HeapObject {
        pub fn from_address(address: usize) -> Self {
            HeapObject { address }
        }

        pub fn address(&self) -> usize {
            self.address
        }

        pub fn is_null(&self) -> bool {
            self.address == 0
        }

        pub fn map(&self, _cage_base: usize, _load_order: usize) -> MapWord {
            // Dummy implementation, replace with actual logic
            MapWord::new(self.address as u64)
        }

        pub fn size_from_map(&self, map: MapWord) -> usize {
            // Dummy implementation, replace with actual logic
            (map.value() & 0x3f) as usize * K_TAGGED_SIZE
        }
    }

    pub struct LiveObjectRange {
        page_: *const PageMetadata,
    }

    impl LiveObjectRange {
        pub fn new(page_: *const PageMetadata) -> Self {
            LiveObjectRange { page_ }
        }

        pub fn begin(&self) -> iterator {
            iterator::new_with_page(unsafe {&*self.page_})
        }

        pub fn end(&self) -> iterator {
            iterator::new()
        }
    }

    pub struct iterator {
        page_: *const PageMetadata,
        cells_: *const MarkBit::CellType,
        cage_base_: usize,
        current_cell_index_: usize,
        current_cell_: MarkBit::CellType,
        current_object_: HeapObject,
        current_map_: MapWord,
        current_size_: usize,
    }

    impl iterator {
        pub fn new() -> Self {
            iterator {
                page_: std::ptr::null(),
                cells_: std::ptr::null(),
                cage_base_: K_NULL_ADDRESS,
                current_cell_index_: 0,
                current_cell_: 0,
                current_object_: HeapObject::from_address(0),
                current_map_: MapWord::new(0),
                current_size_: 0,
            }
        }

        pub fn new_with_page(page: &PageMetadata) -> Self {
            let cells = page.marking_bitmap().cells();
            let isolate = page.heap().isolate();

            let area_start = page.area_start();
            let current_cell_index = MarkingBitmap::index_to_cell(
                MarkingBitmap::address_to_index(area_start),
            );

            iterator {
                page_: page,
                cells_: cells,
                cage_base_: isolate,
                current_cell_index_: current_cell_index,
                current_cell_: unsafe { *cells.add(current_cell_index)},
                current_object_: HeapObject::from_address(0),
                current_map_: MapWord::new(0),
                current_size_: 0,
            }
        }
    }

    impl Iterator for iterator {
        type Item = HeapObject;

        fn next(&mut self) -> Option<Self::Item> {
            self.advance_to_next_valid_object();
            if self.current_object_.is_null() {
                None
            } else {
                Some(self.current_object_)
            }
        }
    }

    impl iterator {
        fn advance_to_next_valid_object(&mut self) {
            // If we found a regular object we are done. In case of free space, we
            // need to continue.
            //
            // Reading the instance type of the map is safe here even in the presence
            // of the mutator writing a new Map because Map objects are published with
            // release stores (or are otherwise read-only) and the map is retrieved  in
            // `AdvanceToNextMarkedObject()` using an acquire load.
            while self.advance_to_next_marked_object() &&
                   InstanceTypeChecker::is_free_space_or_filler(self.current_map_) {
            }
        }

        fn advance_to_next_marked_object(&mut self) -> bool {
            // The following block moves the iterator to the next cell from the current
            // object. This means skipping all possibly set mark bits (in case of black
            // allocation).
            if !self.current_object_.is_null() {
                // Compute an end address that is inclusive. This allows clearing the cell
                // up and including the end address. This works for one word fillers as
                // well as other objects.
                let next_object = self.current_object_.address() + self.current_size_;
                self.current_object_ = HeapObject::from_address(0);

                if MemoryChunk::is_aligned(next_object) {
                    return false;
                }
                // Area end may not be exactly aligned to kAlignment. We don't need to bail
                // out for area_end() though as we are guaranteed to have a bit for the
                // whole page.
                let page = unsafe { &*self.page_ };
                if next_object > page.area_end() {
                  panic!("next_object out of area!");
                }

                // Move to the corresponding cell of the end index.
                let next_markbit_index = MarkingBitmap::address_to_index(next_object);
                if MarkingBitmap::index_to_cell(next_markbit_index) < self.current_cell_index_ {
                  panic!("MarkingBitmap::index_to_cell(next_markbit_index) < self.current_cell_index_");
                }

                self.current_cell_index_ = MarkingBitmap::index_to_cell(next_markbit_index);
                if self.current_cell_index_ >= MarkingBitmap::K_CELLS_COUNT {
                   panic!("current_cell_index_ >= MarkingBitmap::K_CELLS_COUNT");
                }
                // Mask out lower addresses in the cell.
                let mask = MarkingBitmap::index_in_cell_mask(next_markbit_index);
                self.current_cell_ = unsafe {*self.cells_.add(self.current_cell_index_)} & !(mask.wrapping_sub(1));
            }

            // The next block finds any marked object starting from the current cell.
            let chunk = unsafe { (&*self.page_).chunk() };
            loop {
                if self.current_cell_ != 0 {
                    let trailing_zeros = self.current_cell_.trailing_zeros() as usize;
                    let current_cell_base = chunk.address() + MarkingBitmap::cell_to_base(self.current_cell_index_);
                    let object_address = current_cell_base + trailing_zeros * K_TAGGED_SIZE;
                    // The object may be a filler which we want to skip.
                    self.current_object_ = HeapObject::from_address(object_address);
                    self.current_map_ = self.current_object_.map(self.cage_base_, 0); // Replace 0 with kAcquireLoad if defined
                    if !MapWord::is_map_or_forwarded(self.current_map_){
                        panic!("current_map_ is not map or forwarded");
                    }

                    self.current_size_ = align_to_allocation_alignment(
                        self.current_object_.size_from_map(self.current_map_),
                    );

                    if !unsafe { (&*self.page_).contains_limit(object_address + self.current_size_)}{
                        panic!("page doesn't contains limit");
                    }

                    return true;
                }

                self.current_cell_index_ += 1;
                if self.current_cell_index_ >= MarkingBitmap::K_CELLS_COUNT {
                    break;
                }
                self.current_cell_ = unsafe {*self.cells_.add(self.current_cell_index_)};
            }

            false
        }
    }

    fn align_to_allocation_alignment(size: usize) -> usize {
        (size + (K_ALIGNMENT - 1)) & !(K_ALIGNMENT - 1)
    }
}

pub mod heap {
    pub mod heap_inl {
        pub struct Heap {}

        impl Heap {
            pub fn isolate(&self) -> usize {
                0 // Dummy isolate value
            }
        }
    }
    pub mod page_metadata {
        use crate::heap::marking_bitmap::MarkingBitmap;
        use crate::heap::page_metadata_inl::MemoryChunk;
        use crate::heap::heap_inl::Heap;

        pub struct PageMetadata {
            chunk_: MemoryChunk,
            area_start_: usize,
            area_end_: usize,
            marking_bitmap_: MarkingBitmap,
            heap_: Heap,
        }

        impl PageMetadata {
            pub fn new(chunk_: MemoryChunk, area_start_: usize, area_end_: usize, marking_bitmap_: MarkingBitmap, heap_:Heap) -> Self {
                PageMetadata { chunk_, area_start_, area_end_, marking_bitmap_, heap_ }
            }

            pub fn marking_bitmap(&self) -> &MarkingBitmap {
                &self.marking_bitmap_
            }

            pub fn area_start(&self) -> usize {
                self.area_start_
            }

            pub fn area_end(&self) -> usize {
                self.area_end_
            }
            
            pub fn chunk(&self) -> &MemoryChunk {
              &self.chunk_
            }

            pub fn contains_limit(&self, address: usize) -> bool {
                address <= self.area_end_
            }

            pub fn heap(&self) -> &Heap {
              &self.heap_
            }
        }
    }
    pub mod page_metadata_inl {
        #[derive(Clone, Copy)]
        pub struct MemoryChunk {
            address_: usize,
        }

        impl MemoryChunk {
            pub fn new(address_: usize) -> Self {
                MemoryChunk { address_ }
            }

            pub fn address(&self) -> usize {
                self.address_
            }

            pub fn is_aligned(address: usize) -> bool {
                address % 8 == 0 // Assuming alignment of 8
            }
        }
    }
    pub mod marking_bitmap {
        pub mod MarkBit {
            pub type CellType = u64;
        }

        pub struct MarkingBitmap {
            cells_: [*const MarkBit::CellType; 16],
        }

        impl MarkingBitmap {
            pub const K_CELLS_COUNT: usize = 16; // Define constant

            pub fn new(cells_: [*const MarkBit::CellType; 16]) -> Self {
                MarkingBitmap { cells_ }
            }

            pub fn cells(&self) -> *const MarkBit::CellType {
                self.cells_[0] as *const MarkBit::CellType
            }

            pub fn address_to_index(address: usize) -> usize {
                address / 8 // Assuming tagged size of 8
            }

            pub fn index_to_cell(index: usize) -> usize {
                index / 64 // Assuming cell size of 64 bytes (512 bits)
            }

            pub fn cell_to_base(cell_index: usize) -> usize {
                cell_index * 512
            }

            pub fn index_in_cell_mask(index: usize) -> MarkBit::CellType {
                1 << (index % 64)
            }
        }
    }
}

pub mod objects {
    pub mod instance_type_inl {
        #[derive(Clone, Copy)]
        pub struct InstanceType {
            value: u32,
        }

        impl InstanceType {
            pub const FREE_SPACE_TYPE: u32 = 1;
            pub const FILLER_TYPE: u32 = 2;

            pub fn new(value: u32) -> Self {
                InstanceType { value }
            }

            pub fn is_free_space_type(&self) -> bool {
                self.value == InstanceType::FREE_SPACE_TYPE
            }

            pub fn is_filler_type(&self) -> bool {
                self.value == InstanceType::FILLER_TYPE
            }
        }
        
        pub struct InstanceTypeChecker;

        impl InstanceTypeChecker {
          pub fn is_free_space_or_filler(_map: crate::objects::map::MapWord) -> bool {
              false
          }
        }
    }

    pub mod map {
        #[derive(Clone, Copy)]
        pub struct MapWord {
            value: u64,
        }

        impl MapWord {
            pub fn new(value: u64) -> Self {
                MapWord { value }
            }

            pub fn value(&self) -> u64 {
                self.value
            }

            pub fn is_map_or_forwarded(map: MapWord) -> bool {
                map.value != 0 // Dummy implementation
            }
        }
    }
}
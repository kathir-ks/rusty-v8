// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NOTE: This translation assumes the existence of equivalent Rust definitions
// for types and constants like `MemoryChunk`, `InstructionStream`,
// `kCodeAlignment`, `kRegularPageSize`, `kDoubleSize`,
// `kMaxRegularHeapObjectSize`, `AllocationSpace`, `CODE_SPACE`,
// `kTaggedSize`, `IsAnyCodeSpace`, `RoundUp`, `ALIGN_TO_ALLOCATION_ALIGNMENT`,
// `RoundDown`, `DCHECK_NE`.
// These would typically be defined in other modules within the larger project.

/// Layout information for memory chunks in the V8 heap.
pub struct MemoryChunkLayout;

impl MemoryChunkLayout {
    /// Code pages have padding on the first page for code alignment, so the
    /// ObjectStartOffset will not be page aligned.
    pub const fn object_start_offset_in_code_page() -> usize {
        // The instruction stream data (so after the header) should be aligned to
        // kCodeAlignment.
        round_up(
            std::mem::size_of::<MemoryChunk>() + InstructionStream::k_header_size as usize,
            k_code_alignment as usize,
        ) - InstructionStream::k_header_size as usize
    }

    /// Returns the amount of allocatable memory in a code page.
    pub const fn allocatable_memory_in_code_page() -> usize {
        k_regular_page_size as usize - Self::object_start_offset_in_code_page()
    }

    /// Returns the object start offset in a data page.
    pub const fn object_start_offset_in_data_page() -> usize {
        round_up(
            std::mem::size_of::<MemoryChunk>(),
            align_to_allocation_alignment(std::mem::size_of::<f64>())
        )
    }

    /// Returns the amount of allocatable memory in a data page.
    pub const fn allocatable_memory_in_data_page() -> usize {
        let k_allocatable_memory_in_data_page =
            k_regular_page_size as usize - Self::object_start_offset_in_data_page();
        assert!(k_max_regular_heap_object_size as usize <= k_allocatable_memory_in_data_page);
        k_allocatable_memory_in_data_page
    }

    /// Returns the object start offset in a memory chunk for a given allocation
    /// space.
    pub const fn object_start_offset_in_memory_chunk(space: AllocationSpace) -> usize {
        if is_any_code_space(space) {
            Self::object_start_offset_in_code_page()
        } else {
            // Read-only pages use the same layout as regular pages.
            Self::object_start_offset_in_data_page()
        }
    }

    /// Returns the amount of allocatable memory in a memory chunk for a given
    /// allocation space.
    pub const fn allocatable_memory_in_memory_chunk(space: AllocationSpace) -> usize {
        // Note: Replaced DCHECK_NE with an assert.
        assert!(space != AllocationSpace::CODE_LO_SPACE);
        if space == AllocationSpace::CODE_SPACE {
            Self::allocatable_memory_in_code_page()
        } else {
            // Read-only pages use the same layout as regular pages.
            Self::allocatable_memory_in_data_page()
        }
    }

    /// Returns the maximum size of a regular code object.
    pub const fn max_regular_code_object_size() -> usize {
        let k_max_regular_code_object_size = round_down(
            Self::allocatable_memory_in_code_page() / 2,
            std::mem::size_of::<usize>()
        ) ;
        assert!(k_max_regular_code_object_size <= k_max_regular_heap_object_size as usize);
        k_max_regular_code_object_size
    }
}

// Dummy definitions for types and constants.  These need to be replaced with
// actual definitions from the V8 codebase.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AllocationSpace {
    NEW_SPACE,
    OLD_SPACE,
    CODE_SPACE,
    MAP_SPACE,
    LO_SPACE,
    CODE_LO_SPACE,
}

pub struct MemoryChunk {}

pub struct InstructionStream {
  pub k_header_size: i32
}

impl InstructionStream {
    pub const k_header_size: i32 = 16;
}

const k_code_alignment: i32 = 16;
const k_regular_page_size: i32 = 16384;
//const k_double_size: i32 = 8;
const k_max_regular_heap_object_size: i32 = 8192;
//const k_tagged_size: i32 = 8;

const fn is_any_code_space(space: AllocationSpace) -> bool {
  space == AllocationSpace::CODE_SPACE || space == AllocationSpace::CODE_LO_SPACE
}

const fn round_up(x: usize, multiple: usize) -> usize {
    (x + multiple - 1) / multiple * multiple
}

const fn align_to_allocation_alignment(size: usize) -> usize {
    round_up(size, 8)
}

const fn round_down(x: usize, multiple: usize) -> usize {
    x / multiple * multiple
}
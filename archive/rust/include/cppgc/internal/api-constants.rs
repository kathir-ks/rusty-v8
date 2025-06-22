// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    pub mod api_constants {
        // Embedders should not rely on this code!

        pub const KB: usize = 1024;
        pub const MB: usize = KB * 1024;
        pub const GB: usize = MB * 1024;

        // Offset of the uint16_t bitfield from the payload contaning the
        // in-construction bit. This is subtracted from the payload pointer to get
        // to the right bitfield.
        pub const FULLY_CONSTRUCTED_BIT_FIELD_OFFSET_FROM_PAYLOAD: usize =
            2 * std::mem::size_of::<u16>();
        // Mask for in-construction bit.
        pub const FULLY_CONSTRUCTED_BIT_MASK: u16 = 1;

        pub const PAGE_SIZE_BITS: usize = 17;
        pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;

        pub const LARGE_OBJECT_SIZE_THRESHOLD: usize = PAGE_SIZE / 2;

        #[cfg(feature = "cppgc_pointer_compression")]
        pub const POINTER_COMPRESSION_SHIFT: u32 = {
            #[cfg(feature = "cppgc_enable_larger_cage")]
            {
                3
            }
            #[cfg(not(feature = "cppgc_enable_larger_cage"))]
            {
                1
            }
        };

        #[cfg(feature = "cppgc_caged_heap")]
        pub const CAGED_HEAP_DEFAULT_RESERVATION_SIZE: usize = 4 * GB;

        #[cfg(feature = "cppgc_caged_heap")]
        pub const CAGED_HEAP_MAX_RESERVATION_SIZE: usize = {
            #[cfg(feature = "cppgc_pointer_compression")]
            {
                1 << (31 + POINTER_COMPRESSION_SHIFT) as usize
            }
            #[cfg(not(feature = "cppgc_pointer_compression"))]
            {
                CAGED_HEAP_DEFAULT_RESERVATION_SIZE
            }
        };

        #[cfg(feature = "cppgc_caged_heap")]
        pub const CAGED_HEAP_RESERVATION_ALIGNMENT: usize = CAGED_HEAP_MAX_RESERVATION_SIZE;

        pub const DEFAULT_ALIGNMENT: usize = std::mem::size_of::<usize>();

        // Maximum support alignment for a type as in `alignof(T)`.
        pub const MAX_SUPPORTED_ALIGNMENT: usize = 2 * DEFAULT_ALIGNMENT;

        // Granularity of heap allocations.
        pub const ALLOCATION_GRANULARITY: usize = std::mem::size_of::<usize>();

        // Default cacheline size.
        pub const CACHELINE_SIZE: usize = 64;
    }
}
// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: The V8_EXPORT_PRIVATE macro and AllStatic class are not directly
// translatable to Rust. We'll represent the functionality using a module
// and associated constants.

/// Constants related to memory chunks.
pub mod memory_chunk_constants {
    // Assuming kPtrComprCageReservationSize, kRegularPageSize,
    // kMaximalCodeRangeSize, and kMaximalTrustedRangeSize are defined elsewhere.
    // Replace these placeholders with actual values or functions.
    const K_PTR_COMPR_CAGE_RESERVATION_SIZE: usize = 1 << 30; // Example: 1GB
    const K_REGULAR_PAGE_SIZE: usize = 4096; // Example: 4KB
    const K_MAXIMAL_CODE_RANGE_SIZE: usize = 1 << 27; // Example: 128MB
    const K_MAXIMAL_TRUSTED_RANGE_SIZE: usize = 1 << 24; // Example: 16MB

    #[cfg(feature = "v8_enable_sandbox")]
    pub mod sandbox {
        use super::*;

        pub const K_PAGES_IN_MAIN_CAGE: usize =
            K_PTR_COMPR_CAGE_RESERVATION_SIZE / K_REGULAR_PAGE_SIZE;
        pub const K_PAGES_IN_CODE_CAGE: usize =
            K_MAXIMAL_CODE_RANGE_SIZE / K_REGULAR_PAGE_SIZE;
        pub const K_PAGES_IN_TRUSTED_CAGE: usize =
            K_MAXIMAL_TRUSTED_RANGE_SIZE / K_REGULAR_PAGE_SIZE;

        pub const K_MAIN_CAGE_METADATA_OFFSET: usize = 0;
        pub const K_TRUSTED_SPACE_METADATA_OFFSET: usize =
            K_MAIN_CAGE_METADATA_OFFSET + K_PAGES_IN_MAIN_CAGE;
        pub const K_CODE_RANGE_METADATA_OFFSET: usize =
            K_TRUSTED_SPACE_METADATA_OFFSET + K_PAGES_IN_TRUSTED_CAGE;

        // Assuming base::bits::BitWidth is equivalent to usize::BITS
        pub const K_METADATA_POINTER_TABLE_SIZE_LOG2: u32 =
            (K_PAGES_IN_MAIN_CAGE + K_PAGES_IN_CODE_CAGE + K_PAGES_IN_TRUSTED_CAGE)
                .next_power_of_two()
                .trailing_zeros();
        pub const K_METADATA_POINTER_TABLE_SIZE: usize = 1 << K_METADATA_POINTER_TABLE_SIZE_LOG2;
        pub const K_METADATA_POINTER_TABLE_SIZE_MASK: usize =
            K_METADATA_POINTER_TABLE_SIZE - 1;
    }
}
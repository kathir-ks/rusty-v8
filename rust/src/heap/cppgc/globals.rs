// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod cppgc {
    pub mod internal {
        /// Represents a raw memory address.
        pub type Address = *mut u8;
        /// Represents a constant raw memory address.
        pub type ConstAddress = *const u8;

        /// Kilobyte constant.
        pub const KB: usize = 1024;
        /// Megabyte constant.
        pub const MB: usize = KB * 1024;
        /// Gigabyte constant.
        pub const GB: usize = MB * 1024;

        /// AccessMode used for choosing between atomic and non-atomic accesses.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum AccessMode {
            /// Non-atomic access.
            NonAtomic,
            /// Atomic access.
            Atomic,
        }

        // See 6.7.6 (http://eel.is/c++draft/basic.align) for alignment restrictions. We
        // do not fully support all alignment restrictions (following
        // alignof(std​::​max_­align_­t)) but limit to alignof(double).
        //
        // This means that any scalar type with stricter alignment requirements (in
        // practice: long double) cannot be used unrestricted in garbage-collected
        // objects.
        #[cfg(target_pointer_width = "64")]
        /// Allocation granularity.
        pub const ALLOCATION_GRANULARITY: usize = 8;
        #[cfg(not(target_pointer_width = "64"))]
        /// Allocation granularity.
        pub const ALLOCATION_GRANULARITY: usize = 4;
        /// Allocation mask.
        pub const ALLOCATION_MASK: usize = ALLOCATION_GRANULARITY - 1;

        /// Base 2 logarithm of the page size.
        pub const PAGE_SIZE_LOG2: usize = 17;
        /// Page size.
        pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_LOG2;
        /// Page offset mask.
        pub const PAGE_OFFSET_MASK: usize = PAGE_SIZE - 1;
        /// Page base mask.
        pub const PAGE_BASE_MASK: usize = !PAGE_OFFSET_MASK;

        /// Threshold for large object size.
        pub const LARGE_OBJECT_SIZE_THRESHOLD: usize = PAGE_SIZE / 2;

        /// GCInfo index for free list.
        pub const FREE_LIST_GC_INFO_INDEX: usize = 0; // Assuming GCInfoIndex is usize
        /// Size of a free list entry.
        pub const FREE_LIST_ENTRY_SIZE: usize = 2 * std::mem::size_of::<usize>();

        #[cfg(feature = "pointer_compression")]
        /// Size of a slot.
        pub const SLOT_SIZE: usize = std::mem::size_of::<u32>();
        #[cfg(not(feature = "pointer_compression"))]
        /// Size of a slot.
        pub const SLOT_SIZE: usize = std::mem::size_of::<usize>();
    }
}
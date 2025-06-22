// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::sync::atomic::{AtomicUsize, Ordering};

macro_rules! define_operators_for_flags {
    ($flags_type:ty) => {
        impl BitOr for $flags_type {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }

        impl BitOrAssign for $flags_type {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0;
            }
        }

        impl BitAnd for $flags_type {
            type Output = Self;

            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }

        impl BitAndAssign for $flags_type {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0;
            }
        }

        impl Not for $flags_type {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    };
}

pub mod internal {
    use super::*;

    pub mod debug_helper_internal {
        pub struct ReadStringVisitor {}
    }

    pub struct Heap {}

    impl Heap {
        pub fn new() -> Self {
            Heap {}
        }
    }

    #[derive(Debug)]
    pub struct MemoryChunkMetadata {}

    pub struct ReadOnlyPageMetadata {}

    pub struct PageMetadata {}

    pub struct LargePageMetadata {}

    pub struct CodeStubAssembler {}

    pub struct ExternalReference {}

    #[derive(Debug, Copy, Clone)]
    pub struct Tagged<T> {
        ptr: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn new(ptr: usize) -> Self {
            Tagged {
                ptr,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn ptr(&self) -> usize {
            self.ptr
        }
    }

    pub struct TestDebugHelper {}

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum MarkingMode {
        kNoMarking,
        kMinorMarking,
        kMajorMarking,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MainThreadFlags(usize);

    impl MainThreadFlags {
        pub fn contains(&self, other: Self) -> bool {
            (self.0 & other.0) == other.0
        }

        pub fn without(self, other: Self) -> Self {
            Self(self.0 & !other.0)
        }
    }

    #[derive(Debug)]
    pub struct MemoryChunk {
        main_thread_flags_: MainThreadFlags,
        metadata_: *mut MemoryChunkMetadata, // Raw pointer to MemoryChunkMetadata
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Executability {
        EXECUTABLE,
        NOT_EXECUTABLE,
    }

    impl fmt::Display for Executability {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Executability::EXECUTABLE => write!(f, "EXECUTABLE"),
                Executability::NOT_EXECUTABLE => write!(f, "NOT_EXECUTABLE"),
            }
        }
    }

    // Constants
    const kPageSizeBits: usize = 12;
    const kAlignment: usize = 1 << kPageSizeBits;
    const kAlignmentMask: usize = kAlignment - 1;
    const kAllFlagsMaskValue: usize = !0; // All bits set to 1
    const POINTERS_TO_HERE_ARE_INTERESTING_VALUE: usize = 1 << 1;
    const POINTERS_FROM_HERE_ARE_INTERESTING_VALUE: usize = 1 << 2;
    const EVACUATION_CANDIDATE_VALUE: usize = 1 << 11;
    const FROM_PAGE_VALUE: usize = 1 << 3;
    const TO_PAGE_VALUE: usize = 1 << 4;
    const INCREMENTAL_MARKING_VALUE: usize = 1 << 5;
    const READ_ONLY_HEAP_VALUE: usize = 1 << 6;
    const CONTAINS_ONLY_OLD_VALUE: usize = 1 << 8;
    const IS_MAJOR_GC_IN_PROGRESS_VALUE: usize = 1 << 7;
    const LARGE_PAGE_VALUE: usize = 1 << 10;
    const COMPACTION_WAS_ABORTED_VALUE: usize = 1 << 17;
    const PINNED_VALUE: usize = 1 << 20;
    const IS_EXECUTABLE_VALUE: usize = 1 << 21;
    const IS_TRUSTED_VALUE: usize = 1 << 22;
    const IS_QUARANTINED_VALUE: usize = 1 << 23;
    const IN_WRITABLE_SHARED_SPACE_VALUE: usize = 1 << 0;

    impl MemoryChunk {
        pub const NO_FLAGS: MainThreadFlags = MainThreadFlags(0);
        pub const IN_WRITABLE_SHARED_SPACE: MainThreadFlags = MainThreadFlags(IN_WRITABLE_SHARED_SPACE_VALUE);
        pub const POINTERS_TO_HERE_ARE_INTERESTING: MainThreadFlags =
            MainThreadFlags(POINTERS_TO_HERE_ARE_INTERESTING_VALUE);
        pub const POINTERS_FROM_HERE_ARE_INTERESTING: MainThreadFlags =
            MainThreadFlags(POINTERS_FROM_HERE_ARE_INTERESTING_VALUE);
        pub const FROM_PAGE: MainThreadFlags = MainThreadFlags(1 << 3);
        pub const TO_PAGE: MainThreadFlags = MainThreadFlags(1 << 4);
        pub const INCREMENTAL_MARKING: MainThreadFlags = MainThreadFlags(INCREMENTAL_MARKING_VALUE);
        pub const READ_ONLY_HEAP: MainThreadFlags = MainThreadFlags(READ_ONLY_HEAP_VALUE);
        pub const IS_MAJOR_GC_IN_PROGRESS: MainThreadFlags = MainThreadFlags(IS_MAJOR_GC_IN_PROGRESS_VALUE);
        pub const CONTAINS_ONLY_OLD: MainThreadFlags = MainThreadFlags(CONTAINS_ONLY_OLD_VALUE);
        pub const BLACK_ALLOCATED: MainThreadFlags = MainThreadFlags(1 << 9);
        pub const LARGE_PAGE: MainThreadFlags = MainThreadFlags(LARGE_PAGE_VALUE);
        pub const EVACUATION_CANDIDATE: MainThreadFlags = MainThreadFlags(EVACUATION_CANDIDATE_VALUE);
        pub const NEVER_EVACUATE: MainThreadFlags = MainThreadFlags(1 << 12);
        pub const PAGE_NEW_OLD_PROMOTION: MainThreadFlags = MainThreadFlags(1 << 13);
        pub const FORCE_EVACUATION_CANDIDATE_FOR_TESTING: MainThreadFlags =
            MainThreadFlags(1 << 14);
        pub const NEVER_ALLOCATE_ON_PAGE: MainThreadFlags = MainThreadFlags(1 << 15);
        pub const PRE_FREED: MainThreadFlags = MainThreadFlags(1 << 16);
        pub const COMPACTION_WAS_ABORTED: MainThreadFlags = MainThreadFlags(COMPACTION_WAS_ABORTED_VALUE);
        pub const NEW_SPACE_BELOW_AGE_MARK: MainThreadFlags = MainThreadFlags(1 << 18);
        pub const UNREGISTERED: MainThreadFlags = MainThreadFlags(1 << 19);
        pub const PINNED: MainThreadFlags = MainThreadFlags(PINNED_VALUE);
        pub const IS_EXECUTABLE: MainThreadFlags = MainThreadFlags(IS_EXECUTABLE_VALUE);
        pub const IS_TRUSTED: MainThreadFlags = MainThreadFlags(IS_TRUSTED_VALUE);
        pub const IS_QUARANTINED: MainThreadFlags = MainThreadFlags(IS_QUARANTINED_VALUE);

        pub const kAllFlagsMask: MainThreadFlags = MainThreadFlags(kAllFlagsMaskValue);
        pub const kPointersToHereAreInterestingMask: MainThreadFlags =
            MemoryChunk::POINTERS_TO_HERE_ARE_INTERESTING;
        pub const kPointersFromHereAreInterestingMask: MainThreadFlags =
            MemoryChunk::POINTERS_FROM_HERE_ARE_INTERESTING;
        pub const kEvacuationCandidateMask: MainThreadFlags = MemoryChunk::EVACUATION_CANDIDATE;
        pub const kIsInYoungGenerationMask: MainThreadFlags =
            MemoryChunk::FROM_PAGE | MemoryChunk::TO_PAGE;
        pub const kIsInReadOnlyHeapMask: MainThreadFlags = MemoryChunk::READ_ONLY_HEAP;
        pub const kIsLargePageMask: MainThreadFlags = MemoryChunk::LARGE_PAGE;
        pub const kInSharedHeap: MainThreadFlags = MemoryChunk::IN_WRITABLE_SHARED_SPACE;
        pub const kIncrementalMarking: MainThreadFlags = MemoryChunk::INCREMENTAL_MARKING;
        pub const kSkipEvacuationSlotsRecordingMask: MainThreadFlags =
            MemoryChunk::kEvacuationCandidateMask | MemoryChunk::kIsInYoungGenerationMask;
        pub const kIsOnlyOldOrMajorGCInProgressMask: MainThreadFlags =
            MemoryChunk::CONTAINS_ONLY_OLD | MemoryChunk::IS_MAJOR_GC_IN_PROGRESS;

        pub fn new(flags: MainThreadFlags, metadata: *mut MemoryChunkMetadata) -> Self {
            MemoryChunk {
                main_thread_flags_: flags,
                metadata_: metadata,
            }
        }

        pub fn address(&self) -> usize {
            self as *const Self as usize
        }

        pub const fn base_address(a: usize) -> usize {
            a & !kAlignmentMask
        }

        pub fn from_address(addr: usize) -> *mut MemoryChunk {
            Self::base_address(addr) as *mut MemoryChunk
        }

        pub fn from_heap_object<HeapObject>(object: Tagged<HeapObject>) -> *mut MemoryChunk {
            Self::from_address(object.ptr())
        }

        pub fn metadata(&self) -> *mut MemoryChunkMetadata {
            self.metadata_
        }

        pub fn metadata_const(&self) -> *const MemoryChunkMetadata {
            self.metadata_ as *const MemoryChunkMetadata
        }

        pub fn is_flag_set(&self, flag: MainThreadFlags) -> bool {
            self.main_thread_flags_.contains(flag)
        }

        pub fn is_marking(&self) -> bool {
            self.is_flag_set(MemoryChunk::INCREMENTAL_MARKING)
        }

        pub fn in_writable_shared_space(&self) -> bool {
            self.is_flag_set(MemoryChunk::IN_WRITABLE_SHARED_SPACE)
        }

        pub fn in_young_generation(&self) -> bool {
            let k_young_generation_mask = MemoryChunk::FROM_PAGE | MemoryChunk::TO_PAGE;
            (self.get_flags().0 & k_young_generation_mask.0) != 0
        }

        pub fn is_young_or_shared_chunk(&self) -> bool {
            let k_young_or_shared_chunk_mask =
                MemoryChunk::FROM_PAGE | MemoryChunk::TO_PAGE | MemoryChunk::IN_WRITABLE_SHARED_SPACE;
            (self.get_flags().0 & k_young_or_shared_chunk_mask.0) != 0
        }

        pub fn set_flag_slow(&mut self, flag: MainThreadFlags) {
            self.set_flag_unlocked(flag);
        }

        pub fn clear_flag_slow(&mut self, flag: MainThreadFlags) {
            self.clear_flag_unlocked(flag);
        }

        pub fn get_flags(&self) -> MainThreadFlags {
            self.main_thread_flags_
        }

        pub fn set_flag_unlocked(&mut self, flag: MainThreadFlags) {
            self.main_thread_flags_ |= flag;
        }

        pub fn clear_flag_unlocked(&mut self, flag: MainThreadFlags) {
            self.main_thread_flags_ = self.main_thread_flags_.without(flag);
        }

        pub fn clear_flags_unlocked(&mut self, flags: MainThreadFlags) {
            self.main_thread_flags_ = MainThreadFlags(self.main_thread_flags_.0 & !flags.0);
        }

        pub fn set_flags_unlocked(&mut self, flags: MainThreadFlags, mask: MainThreadFlags) {
            self.main_thread_flags_ =
                MainThreadFlags((self.main_thread_flags_.0 & !mask.0) | (flags.0 & mask.0));
        }

        pub fn set_flag_non_executable(&mut self, flag: MainThreadFlags) {
            self.set_flag_unlocked(flag);
        }

        pub fn clear_flag_non_executable(&mut self, flag: MainThreadFlags) {
            self.clear_flag_unlocked(flag);
        }

        pub fn set_flags_non_executable(&mut self, flags: MainThreadFlags, mask: MainThreadFlags) {
            self.set_flags_unlocked(flags, mask);
        }

        pub fn clear_flags_non_executable(&mut self, flags: MainThreadFlags) {
            self.clear_flags_unlocked(flags);
        }

        pub fn set_major_gc_in_progress(&mut self) {
            self.set_flag_unlocked(MemoryChunk::IS_MAJOR_GC_IN_PROGRESS);
        }

        pub fn reset_major_gc_in_progress(&mut self) {
            self.clear_flag_unlocked(MemoryChunk::IS_MAJOR_GC_IN_PROGRESS);
        }

        pub fn get_heap(&self) -> Heap {
            Heap::new()
        }

        pub fn initialization_memory_fence(&self) {
            std::sync::atomic::fence(Ordering::SeqCst); // Use SeqCst for strong ordering
        }

        pub fn in_read_only_space(&self) -> bool {
            self.is_flag_set(MemoryChunk::READ_ONLY_HEAP)
        }

        pub fn sandbox_safe_in_read_only_space(&self) -> bool {
            self.in_read_only_space()
        }

        pub fn in_code_space(&self) -> bool {
            self.is_flag_set(MemoryChunk::IS_EXECUTABLE)
        }

        pub fn in_trusted_space(&self) -> bool {
            self.is_flag_set(MemoryChunk::IS_TRUSTED)
        }

        pub fn never_evacuate(&self) -> bool {
            self.is_flag_set(MemoryChunk::NEVER_EVACUATE)
        }

        pub fn mark_never_evacuate(&mut self) {
            self.set_flag_slow(MemoryChunk::NEVER_EVACUATE);
        }

        pub fn can_allocate(&self) -> bool {
            !self.is_evacuation_candidate() && !self.is_flag_set(MemoryChunk::NEVER_ALLOCATE_ON_PAGE)
        }

        pub fn is_evacuation_candidate(&self) -> bool {
            !(self.is_flag_set(MemoryChunk::NEVER_EVACUATE) && self.is_flag_set(MemoryChunk::EVACUATION_CANDIDATE))
                && self.is_flag_set(MemoryChunk::EVACUATION_CANDIDATE)
        }

        pub fn should_skip_evacuation_slot_recording(&self) -> bool {
            let flags = self.get_flags();
            (flags.0 & MemoryChunk::kSkipEvacuationSlotsRecordingMask.0 != 0)
                && (flags.0 & MemoryChunk::COMPACTION_WAS_ABORTED.0 == 0)
        }

        pub fn executable(&self) -> Executability {
            if self.is_flag_set(MemoryChunk::IS_EXECUTABLE) {
                Executability::EXECUTABLE
            } else {
                Executability::NOT_EXECUTABLE
            }
        }

        pub fn is_from_page(&self) -> bool {
            self.is_flag_set(MemoryChunk::FROM_PAGE)
        }

        pub fn is_to_page(&self) -> bool {
            self.is_flag_set(MemoryChunk::TO_PAGE)
        }

        pub fn is_large_page(&self) -> bool {
            self.is_flag_set(MemoryChunk::LARGE_PAGE)
        }

        pub fn in_new_space(&self) -> bool {
            self.in_young_generation() && !self.is_large_page()
        }

        pub fn in_new_large_object_space(&self) -> bool {
            self.in_young_generation() && self.is_large_page()
        }

        pub fn is_pinned(&self) -> bool {
            self.is_flag_set(MemoryChunk::PINNED)
        }

        pub fn is_only_old_or_major_marking_on(&self) -> bool {
            (self.get_flags().0 & MemoryChunk::kIsOnlyOldOrMajorGCInProgressMask.0) != 0
        }

        pub fn is_quarantined(&self) -> bool {
            self.is_flag_set(MemoryChunk::IS_QUARANTINED)
        }

        pub const fn is_aligned(address: usize) -> bool {
            (address & kAlignmentMask) == 0
        }

        pub fn old_generation_page_flags(
            marking_mode: MarkingMode,
            space: AllocationSpace,
        ) -> MainThreadFlags {
            let mut flags = MainThreadFlags::from(0);

            if marking_mode == MarkingMode::kMajorMarking {
                flags |= MemoryChunk::INCREMENTAL_MARKING;
            }

            if space == AllocationSpace::RO_SPACE {
                flags |= MemoryChunk::READ_ONLY_HEAP;
            }

            flags
        }

        pub fn young_generation_page_flags(marking_mode: MarkingMode) -> MainThreadFlags {
            let mut flags = MainThreadFlags::from(0);

            if marking_mode == MarkingMode::kMinorMarking {
                flags |= MemoryChunk::INCREMENTAL_MARKING;
            }

            flags | MemoryChunk::FROM_PAGE
        }

        pub fn set_old_generation_page_flags(&mut self, marking_mode: MarkingMode, space: AllocationSpace) {
            let flags = Self::old_generation_page_flags(marking_mode, space);
            self.set_flags_unlocked(flags, MainThreadFlags(kAllFlagsMaskValue));
        }

        pub fn set_young_generation_page_flags(&mut self, marking_mode: MarkingMode) {
            let flags = Self::young_generation_page_flags(marking_mode);
            self.set_flags_unlocked(flags, MainThreadFlags(kAllFlagsMaskValue));
        }

        pub fn is_trusted(&self) -> bool {
            self.is_flag_set(MemoryChunk::IS_TRUSTED)
        }

        pub const fn get_alignment_for_allocation() -> usize {
            kAlignment
        }

        pub const fn get_alignment_mask_for_assembler() -> usize {
            kAlignmentMask
        }

        pub const fn address_to_offset(address: usize) -> u32 {
            (address & kAlignmentMask) as u32
        }

        pub fn offset(&self, addr: usize) -> usize {
            addr - self.address()
        }

        pub fn offset_maybe_out_of_range(&self, addr: usize) -> usize {
            self.offset(addr)
        }

        //Missing ClearMetadataPointer and sandboxing implementations due to complexity and missing context.
    }

    impl Drop for MemoryChunk {
        fn drop(&mut self) {
            // Ensure the raw pointer is not dangling by potentially deallocating it if necessary
            // This depends on how MemoryChunkMetadata is managed (e.g., whether it's owned by MemoryChunk).
            // If MemoryChunk does not own the metadata, this drop impl is not required, or might even be incorrect.

            // SAFETY: Accessing the raw pointer metadata_ is only safe if it's valid.
            // This may require additional checks or refactoring to ensure safety.
            unsafe {
                if !self.metadata_.is_null() {
                    // Example: If metadata_ points to a Box-allocated MemoryChunkMetadata, deallocate it:
                    // drop(Box::from_raw(self.metadata_));

                    // Example: if metadata_ points to memory managed by another system, ensure it's
                    // correctly released by that system.
                }
            }
            // Reset metadata_ to null after dropping.
            self.metadata_ = std::ptr::null_mut();
        }
    }

    impl From<usize> for MainThreadFlags {
        fn from(value: usize) -> Self {
            MainThreadFlags(value)
        }
    }

    define_operators_for_flags!(MainThreadFlags);

    pub enum AllocationSpace {
        RO_SPACE,
        OTHER_SPACE,
    }
}

pub mod base {
    use super::internal;
    use std::hash::{Hash, Hasher};

    impl Hash for internal::MemoryChunk {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let chunk_ptr = self as *const internal::MemoryChunk as usize;
            (chunk_ptr >> internal::kPageSizeBits).hash(state);
        }
    }
}
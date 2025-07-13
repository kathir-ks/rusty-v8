// Converted from V8 C++ source files:
// Header: memory-chunk.h
// Implementation: memory-chunk.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::sync::Mutex;

use crate::heap::safepoint::V8;
use crate::heap::stress_scavenge_observer::code;
use crate::heap::stress_scavenge_observer::v8;

pub struct Heap;
pub struct MemoryChunkMetadata;
pub struct ReadOnlyPageMetadata;
pub struct PageMetadata;
pub struct LargePageMetadata;
pub struct CodeStubAssembler;
pub struct ExternalReference;

pub struct Tagged<T> {
    ptr: *mut T,
}

impl<T> Tagged<T> {
    pub fn ptr(&self) -> Address {
        self.ptr as Address
    }
}
pub type Address = usize;

pub enum class MarkingMode {
    kNoMarking,
    kMinorMarking,
    kMajorMarking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    NO_FLAGS = 0,
    IN_WRITABLE_SHARED_SPACE = 1 << 0,
    POINTERS_TO_HERE_ARE_INTERESTING = 1 << 1,
    POINTERS_FROM_HERE_ARE_INTERESTING = 1 << 2,
    FROM_PAGE = 1 << 3,
    TO_PAGE = 1 << 4,
    INCREMENTAL_MARKING = 1 << 5,
    READ_ONLY_HEAP = 1 << 6,
    IS_MAJOR_GC_IN_PROGRESS = 1 << 7,
    CONTAINS_ONLY_OLD = 1 << 8,
    BLACK_ALLOCATED = 1 << 9,
    LARGE_PAGE = 1 << 10,
    EVACUATION_CANDIDATE = 1 << 11,
    NEVER_EVACUATE = 1 << 12,
    PAGE_NEW_OLD_PROMOTION = 1 << 13,
    FORCE_EVACUATION_CANDIDATE_FOR_TESTING = 1 << 14,
    NEVER_ALLOCATE_ON_PAGE = 1 << 15,
    PRE_FREED = 1 << 16,
    COMPACTION_WAS_ABORTED = 1 << 17,
    NEW_SPACE_BELOW_AGE_MARK = 1 << 18,
    UNREGISTERED = 1 << 19,
    PINNED = 1 << 20,
    IS_EXECUTABLE = 1 << 21,
    IS_TRUSTED = 1 << 22,
    IS_QUARANTINED = 1 << 23,
}

impl Flag {
    pub fn contains(&self, other: Flag) -> bool {
        (*self as usize) & (other as usize) != 0
    }
}

pub struct MainThreadFlags {
    flags: usize,
}

impl MainThreadFlags {
    pub fn new(flags: usize) -> Self {
        MainThreadFlags { flags }
    }

    pub fn contains(&self, flag: Flag) -> bool {
        (self.flags & (flag as usize)) != 0
    }

    pub fn insert(&mut self, flag: Flag) {
        self.flags |= flag as usize;
    }

    pub fn remove(&mut self, flag: Flag) {
        self.flags &= !(flag as usize);
    }

    pub fn without(&self, flag: Flag) -> Self {
        MainThreadFlags {
            flags: self.flags & !(flag as usize),
        }
    }

    pub fn flags(&self) -> usize {
        self.flags
    }
}

impl std::ops::BitOr for MainThreadFlags {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        MainThreadFlags {
            flags: self.flags | other.flags,
        }
    }
}

impl std::ops::BitAnd for MainThreadFlags {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        MainThreadFlags {
            flags: self.flags & other.flags,
        }
    }
}

impl std::ops::BitAndAssign for MainThreadFlags {
    fn bitand_assign(&mut self, other: Self) {
        self.flags &= other.flags;
    }
}

impl std::ops::Not for MainThreadFlags {
    type Output = Self;

    fn not(self) -> Self {
        MainThreadFlags { flags: !self.flags }
    }
}

impl From<Flag> for MainThreadFlags {
    fn from(flag: Flag) -> Self {
        MainThreadFlags { flags: flag as usize }
    }
}

pub struct MemoryChunk {
    main_thread_flags_: MainThreadFlags,
    metadata_: *mut MemoryChunkMetadata,
}

const kPageSizeBits: usize = 12;
const kAlignment: usize = 1 << kPageSizeBits;
const kAlignmentMask: usize = kAlignment - 1;

impl MemoryChunk {
    pub const kAllFlagsMask: MainThreadFlags = MainThreadFlags {
        flags: !0, // All bits set
    };
    pub const kPointersToHereAreInterestingMask: MainThreadFlags =
        MainThreadFlags::new(Flag::POINTERS_TO_HERE_ARE_INTERESTING as usize);
    pub const kPointersFromHereAreInterestingMask: MainThreadFlags =
        MainThreadFlags::new(Flag::POINTERS_FROM_HERE_ARE_INTERESTING as usize);
    pub const kEvacuationCandidateMask: MainThreadFlags =
        MainThreadFlags::new(Flag::EVACUATION_CANDIDATE as usize);
    pub const kIsInYoungGenerationMask: MainThreadFlags = MainThreadFlags::new(
        (Flag::FROM_PAGE as usize) | (Flag::TO_PAGE as usize),
    );
    pub const kIsInReadOnlyHeapMask: MainThreadFlags =
        MainThreadFlags::new(Flag::READ_ONLY_HEAP as usize);
    pub const kIsLargePageMask: MainThreadFlags =
        MainThreadFlags::new(Flag::LARGE_PAGE as usize);
    pub const kSkipEvacuationSlotsRecordingMask: MainThreadFlags =
        MainThreadFlags::new(
            (Self::kEvacuationCandidateMask.flags()) | (Self::kIsInYoungGenerationMask.flags()),
        );
    const kIsOnlyOldOrMajorGCInProgressMask: MainThreadFlags = MainThreadFlags::new(
        (Flag::CONTAINS_ONLY_OLD as usize) | (Flag::IS_MAJOR_GC_IN_PROGRESS as usize),
    );
    const kInSharedHeap: MainThreadFlags =
        MainThreadFlags::new(Flag::IN_WRITABLE_SHARED_SPACE as usize);

    pub fn new(flags: MainThreadFlags, metadata: *mut MemoryChunkMetadata) -> Self {
        MemoryChunk {
            main_thread_flags_: flags,
            metadata_: metadata,
        }
    }

    #[inline]
    pub fn address(&self) -> Address {
        self as *const Self as Address
    }

    pub fn base_address(a: Address) -> Address {
        a & !kAlignmentMask
    }

    pub fn from_address(addr: Address) -> *mut MemoryChunk {
        Self::base_address(addr) as *mut MemoryChunk
    }

    pub fn from_heap_object<HeapObject>(object: Tagged<HeapObject>) -> *mut MemoryChunk {
        Self::from_address(object.ptr())
    }

    #[inline]
    pub fn metadata(&self) -> *mut MemoryChunkMetadata {
        self.metadata_
    }

    #[inline]
    pub fn metadata_const(&self) -> *const MemoryChunkMetadata {
        self.metadata_ as *const MemoryChunkMetadata
    }

    #[inline]
    pub fn is_flag_set(&self, flag: Flag) -> bool {
        self.main_thread_flags_.contains(flag)
    }

    #[inline]
    pub fn is_marking(&self) -> bool {
        self.is_flag_set(Flag::INCREMENTAL_MARKING)
    }

    #[inline]
    pub fn in_writable_shared_space(&self) -> bool {
        self.is_flag_set(Flag::IN_WRITABLE_SHARED_SPACE)
    }

    #[inline]
    pub fn in_young_generation(&self) -> bool {
        let k_young_generation_mask: usize =
            Flag::FROM_PAGE as usize | Flag::TO_PAGE as usize;
        (self.main_thread_flags_.flags() & k_young_generation_mask) != 0
    }

    #[inline]
    pub fn is_young_or_shared_chunk(&self) -> bool {
        let k_young_or_shared_chunk_mask: usize = Flag::FROM_PAGE as usize
            | Flag::TO_PAGE as usize
            | Flag::IN_WRITABLE_SHARED_SPACE as usize;
        (self.main_thread_flags_.flags() & k_young_or_shared_chunk_mask) != 0
    }

    pub fn set_flag_slow(&mut self, flag: Flag) {
        if self.executable() == Executability::EXECUTABLE {
            let _scope = RwxMemoryWriteScope::new("Set a MemoryChunk flag in executable memory.");
            self.set_flag_unlocked(flag);
        } else {
            self.set_flag_non_executable(flag);
        }
    }

    pub fn clear_flag_slow(&mut self, flag: Flag) {
        if self.executable() == Executability::EXECUTABLE {
            let _scope = RwxMemoryWriteScope::new("Clear a MemoryChunk flag in executable memory.");
            self.clear_flag_unlocked(flag);
        } else {
            self.clear_flag_non_executable(flag);
        }
    }

    #[inline]
    pub fn get_flags(&self) -> MainThreadFlags {
        self.main_thread_flags_.clone()
    }

    #[inline]
    pub fn set_flag_unlocked(&mut self, flag: Flag) {
        self.main_thread_flags_.insert(flag);
    }

    #[inline]
    pub fn clear_flag_unlocked(&mut self, flag: Flag) {
        self.main_thread_flags_.remove(flag);
    }

    #[inline]
    pub fn clear_flags_unlocked(&mut self, flags: MainThreadFlags) {
        self.main_thread_flags_ &= !flags;
    }

    #[inline]
    pub fn set_flags_unlocked(&mut self, flags: MainThreadFlags, mask: MainThreadFlags) {
        self.main_thread_flags_ =
            (self.main_thread_flags_ & !mask) | (flags & mask);
    }

    #[inline]
    pub fn set_flag_non_executable(&mut self, flag: Flag) {
        self.set_flag_unlocked(flag);
    }

    #[inline]
    pub fn clear_flag_non_executable(&mut self, flag: Flag) {
        self.clear_flag_unlocked(flag);
    }

    #[inline]
    pub fn set_flags_non_executable(&mut self, flags: MainThreadFlags, mask: MainThreadFlags) {
        self.set_flags_unlocked(flags, mask);
    }

    #[inline]
    pub fn clear_flags_non_executable(&mut self, flags: MainThreadFlags) {
        self.clear_flags_unlocked(flags);
    }

    #[inline]
    pub fn set_major_gc_in_progress(&mut self) {
        self.set_flag_unlocked(Flag::IS_MAJOR_GC_IN_PROGRESS);
    }

    #[inline]
    pub fn reset_major_gc_in_progress(&mut self) {
        self.clear_flag_unlocked(Flag::IS_MAJOR_GC_IN_PROGRESS);
    }

    #[inline]
    pub fn get_heap(&self) -> *mut Heap {
        unsafe { (*(*self.metadata()).owner()).heap() }
    }

    pub fn initialization_memory_fence(&self) {
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }

    #[inline]
    pub fn in_read_only_space(&self) -> bool {
        self.is_flag_set(Flag::READ_ONLY_HEAP)
    }

    #[inline]
    pub fn in_code_space(&self) -> bool {
        self.is_flag_set(Flag::IS_EXECUTABLE)
    }

    #[inline]
    pub fn in_trusted_space(&self) -> bool {
        self.is_flag_set(Flag::IS_TRUSTED)
    }

    pub fn never_evacuate(&self) -> bool {
        self.is_flag_set(Flag::NEVER_EVACUATE)
    }

    pub fn mark_never_evacuate(&mut self) {
        self.set_flag_slow(Flag::NEVER_EVACUATE);
    }

    pub fn can_allocate(&self) -> bool {
        !self.is_evacuation_candidate() && !self.is_flag_set(Flag::NEVER_ALLOCATE_ON_PAGE)
    }

    pub fn is_evacuation_candidate(&self) -> bool {
        if self.is_flag_set(Flag::NEVER_EVACUATE) && self.is_flag_set(Flag::EVACUATION_CANDIDATE) {
            panic!("DCHECK(!(IsFlagSet(NEVER_EVACUATE) && IsFlagSet(EVACUATION_CANDIDATE))) failed");
        }
        self.is_flag_set(Flag::EVACUATION_CANDIDATE)
    }

    pub fn should_skip_evacuation_slot_recording(&self) -> bool {
        let flags = self.get_flags();
        (flags.flags() & Self::kSkipEvacuationSlotsRecordingMask.flags()) != 0
            && ((flags.flags() & Flag::COMPACTION_WAS_ABORTED as usize) == 0)
    }

    pub fn executable(&self) -> Executability {
        if self.is_flag_set(Flag::IS_EXECUTABLE) {
            Executability::EXECUTABLE
        } else {
            Executability::NOT_EXECUTABLE
        }
    }

    pub fn is_from_page(&self) -> bool {
        self.is_flag_set(Flag::FROM_PAGE)
    }

    pub fn is_to_page(&self) -> bool {
        self.is_flag_set(Flag::TO_PAGE)
    }

    pub fn is_large_page(&self) -> bool {
        self.is_flag_set(Flag::LARGE_PAGE)
    }

    pub fn in_new_space(&self) -> bool {
        self.in_young_generation() && !self.is_large_page()
    }

    pub fn in_new_large_object_space(&self) -> bool {
        self.in_young_generation() && self.is_large_page()
    }

    pub fn is_pinned(&self) -> bool {
        self.is_flag_set(Flag::PINNED)
    }

    pub fn is_only_old_or_major_marking_on(&self) -> bool {
        (self.get_flags().flags() & Self::kIsOnlyOldOrMajorGCInProgressMask.flags()) != 0
    }

    pub fn is_quarantined(&self) -> bool {
        self.is_flag_set(Flag::IS_QUARANTINED)
    }

    #[inline]
    pub const fn is_aligned(address: Address) -> bool {
        (address & kAlignmentMask) == 0
    }

    pub fn old_generation_page_flags(
        marking_mode: MarkingMode,
        space: AllocationSpace,
    ) -> MainThreadFlags {
        let mut flags_to_set = MainThreadFlags::new(0);

        if space != AllocationSpace::OLD_SPACE {
            flags_to_set.insert(Flag::CONTAINS_ONLY_OLD);
        }

        if let MarkingMode::kMajorMarking = marking_mode {
            flags_to_set.insert(Flag::POINTERS_TO_HERE_ARE_INTERESTING);
            flags_to_set.insert(Flag::POINTERS_FROM_HERE_ARE_INTERESTING);
            flags_to_set.insert(Flag::INCREMENTAL_MARKING);
            flags_to_set.insert(Flag::IS_MAJOR_GC_IN_PROGRESS);
        } else if let AllocationSpace::SHARED_SPACE = space {
            flags_to_set.insert(Flag::POINTERS_TO_HERE_ARE_INTERESTING);
        } else {
            flags_to_set.insert(Flag::POINTERS_FROM_HERE_ARE_INTERESTING);
            if let MarkingMode::kMinorMarking = marking_mode {
                flags_to_set.insert(Flag::INCREMENTAL_MARKING);
            }
        }

        flags_to_set
    }

    pub fn young_generation_page_flags(marking_mode: MarkingMode) -> MainThreadFlags {
        let mut flags = MainThreadFlags::new(Flag::POINTERS_TO_HERE_ARE_INTERESTING as usize);
        if let MarkingMode::kNoMarking = marking_mode {
        } else {
            flags.insert(Flag::POINTERS_FROM_HERE_ARE_INTERESTING);
            flags.insert(Flag::INCREMENTAL_MARKING);
            if let MarkingMode::kMajorMarking = marking_mode {
                flags.insert(Flag::IS_MAJOR_GC_IN_PROGRESS);
            }
        }
        flags
    }

    pub fn set_old_generation_page_flags(&mut self, marking_mode: MarkingMode, space: AllocationSpace) {
        let flags_to_set = Self::old_generation_page_flags(marking_mode, space);
        let mut flags_to_clear = MainThreadFlags::new(0);

        if let MarkingMode::kMajorMarking = marking_mode {
        } else if let AllocationSpace::SHARED_SPACE = space {
            flags_to_clear.insert(Flag::POINTERS_FROM_HERE_ARE_INTERESTING);
            flags_to_clear.insert(Flag::INCREMENTAL_MARKING);
        } else {
            flags_to_clear.insert(Flag::POINTERS_TO_HERE_ARE_INTERESTING);
            if let MarkingMode::kMinorMarking = marking_mode {
            } else {
                flags_to_clear.insert(Flag::INCREMENTAL_MARKING);
            }
        }

        self.set_flags_unlocked(flags_to_set, flags_to_set);
        self.clear_flags_unlocked(flags_to_clear);
    }

    pub fn set_young_generation_page_flags(&mut self, marking_mode: MarkingMode) {
        let flags_to_set = Self::young_generation_page_flags(marking_mode);
        let mut flags_to_clear = MainThreadFlags::new(0);

        if let MarkingMode::kNoMarking = marking_mode {
            flags_to_clear.insert(Flag::POINTERS_FROM_HERE_ARE_INTERESTING);
            flags_to_clear.insert(Flag::INCREMENTAL_MARKING);
        }

        self.set_flags_non_executable(flags_to_set, flags_to_set);
        self.clear_flags_non_executable(flags_to_clear);
    }

    #[cfg(debug_assertions)]
    pub fn is_trusted(&self) -> bool {
        let is_trusted = self.is_flag_set(Flag::IS_TRUSTED);
        let id = unsafe { (*self.metadata()).owner().identity() };
        if is_trusted != (Self::is_any_trusted_space(id) || Self::is_any_code_space(id)) {
            panic!("DCHECK_EQ(is_trusted, IsAnyTrustedSpace(id) || IsAnyCodeSpace(id)) failed");
        }
        is_trusted
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn is_trusted(&self) -> bool {
        self.is_flag_set(Flag::IS_TRUSTED)
    }

    pub fn get_alignment_for_allocation() -> usize {
        kAlignment
    }

    pub const fn get_alignment_mask_for_assembler() -> usize {
        kAlignmentMask
    }

    pub fn address_to_offset(address: Address) -> u32 {
        (address & kAlignmentMask) as u32
    }

    #[cfg(debug_assertions)]
    pub fn offset(&self, addr: Address) -> usize {
        if addr < unsafe { (*self.metadata()).area_start() } {
            panic!("DCHECK_GE(addr, Metadata()->area_start()) failed");
        }
        if addr > self.address() + unsafe { (*self.metadata()).size() } {
            panic!("DCHECK_LE(addr, address() + Metadata()->size()) failed");
        }
        addr - self.address()
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn offset(&self, addr: Address) -> usize {
        addr - self.address()
    }

    #[cfg(debug_assertions)]
    pub fn offset_maybe_out_of_range(&self, addr: Address) -> usize {
        if addr < unsafe { (*self.metadata()).area_start() } {
            panic!("DCHECK_GE(addr, Metadata()->area_start()) failed");
        }
        addr - self.address()
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn offset_maybe_out_of_range(&self, addr: Address) -> usize {
        self.offset(addr)
    }

    fn is_any_trusted_space(_space: AllocationSpace) -> bool {
        false
    }

    fn is_any_code_space(_space: AllocationSpace) -> bool {
        false
    }
}

pub enum Executability {
    EXECUTABLE,
    NOT_EXECUTABLE,
}

pub struct RwxMemoryWriteScope {
    message: String,
}

impl RwxMemoryWriteScope {
    pub fn new(message: &str) -> Self {
        RwxMemoryWriteScope {
            message: message.to_string(),
        }
    }
}

impl Drop for RwxMemoryWriteScope {
    fn drop(&mut self) {}
}

pub enum AllocationSpace {
    OLD_SPACE,
    SHARED_SPACE,
    NEW_SPACE,
}

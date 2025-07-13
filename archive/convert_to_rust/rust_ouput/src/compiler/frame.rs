// Converted from V8 C++ source files:
// Header: frame.h
// Implementation: frame.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/frame.h
use std::sync::Mutex;
use std::cell::RefCell;
use crate::AlignedSlotAllocatorError;

const kDoubleSize: i32 = 8;
const kSimd128Size: i32 = 16;

pub struct Frame {
    fixed_slot_count_: i32,
    spill_slot_count_: i32,
    return_slot_count_: i32,
    slot_allocator_: AlignedSlotAllocator,
    allocated_registers_: Option<Box<BitVector>>,
    allocated_double_registers_: Option<Box<BitVector>>,
    zone_: *mut Zone,
    tagged_slots_bits_: GrowableBitVector,
    spill_slots_finished_: bool,
    frame_aligned_: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameOffset {
    offset_: i32,
}

pub struct FrameAccessState<'a> {
    frame_: &'a Frame,
    access_frame_with_fp_: bool,
    fp_relative_only_: bool,
    sp_delta_: i32,
    has_frame_: bool,
}

// src/compiler/frame.cc
use crate::compiler::frame::kDoubleSize;
use crate::compiler::frame::kSimd128Size;
use crate::compiler::AlignedSlotAllocator;
use crate::compiler::AlignedSlotAllocatorError;
use crate::compiler::GrowableBitVector;
use crate::compiler::BitVector;
use crate::execution::frame_constants::StandardFrameConstants;
use crate::utils::bits;
use crate::execution::frame_constants::kElidedFrameSlots;
use crate::execution::frame_constants::kFixedSlotCountAboveFp;

const kSystemPointerSize: i32 = 8;

impl Frame {
    pub fn new(fixed_frame_size_in_slots: i32, zone: *mut Zone) -> Self {
        let mut slot_allocator_ = AlignedSlotAllocator::new();
        slot_allocator_.AllocateUnaligned(fixed_frame_size_in_slots).unwrap();
        Frame {
            fixed_slot_count_: fixed_frame_size_in_slots,
            spill_slot_count_: 0,
            return_slot_count_: 0,
            slot_allocator_: slot_allocator_,
            allocated_registers_: None,
            allocated_double_registers_: None,
            zone_: zone,
            tagged_slots_bits_: GrowableBitVector::new(),
            spill_slots_finished_: false,
            frame_aligned_: false,
        }
    }

    #[inline]
    pub fn get_total_frame_slot_count(&self) -> i32 {
        self.slot_allocator_.Size() + self.return_slot_count_
    }

    #[inline]
    pub fn get_fixed_slot_count(&self) -> i32 {
        self.fixed_slot_count_
    }

    #[inline]
    pub fn get_spill_slot_count(&self) -> i32 {
        self.spill_slot_count_
    }

    #[inline]
    pub fn get_return_slot_count(&self) -> i32 {
        self.return_slot_count_
    }

    pub fn set_allocated_registers(&mut self, regs: Box<BitVector>) {
        debug_assert!(self.allocated_registers_.is_none());
        self.allocated_registers_ = Some(regs);
    }

    pub fn set_allocated_double_registers(&mut self, regs: Box<BitVector>) {
        debug_assert!(self.allocated_double_registers_.is_none());
        self.allocated_double_registers_ = Some(regs);
    }

    pub fn did_allocate_double_registers(&self) -> bool {
        match &self.allocated_double_registers_ {
            Some(regs) => !regs.IsEmpty(),
            None => true,
        }
    }

    pub fn align_saved_callee_register_slots(&mut self, alignment: i32) {
        debug_assert!(!self.frame_aligned_);
        debug_assert!(self.spill_slots_finished_);
        debug_assert!(bits::IsPowerOfTwo(alignment));
        debug_assert!(alignment <= kSimd128Size);
        let alignment_in_slots = AlignedSlotAllocator::NumSlotsForWidth(alignment);
        let padding = self.slot_allocator_.Align(alignment_in_slots).unwrap();
        self.spill_slot_count_ += padding;
    }

    pub fn allocate_saved_callee_register_slots(&mut self, count: i32) {
        debug_assert!(!self.frame_aligned_);
        debug_assert!(self.spill_slots_finished_);
        self.slot_allocator_.AllocateUnaligned(count).unwrap();
    }

    pub fn allocate_spill_slot(&mut self, width: i32, alignment: i32, is_tagged: bool) -> i32 {
        debug_assert_eq!(
            self.get_total_frame_slot_count(),
            self.fixed_slot_count_ + self.spill_slot_count_ + self.return_slot_count_
        );
        debug_assert!(!is_tagged || width == std::mem::size_of::<usize>() as i32);
        debug_assert!(!is_tagged || alignment == std::mem::size_of::<usize>() as i32);
        debug_assert!(!self.spill_slots_finished_);
        debug_assert!(!self.frame_aligned_);

        let actual_width = std::cmp::max(width, AlignedSlotAllocator::kSlotSize);
        let actual_alignment = std::cmp::max(alignment, AlignedSlotAllocator::kSlotSize);
        let slots = AlignedSlotAllocator::NumSlotsForWidth(actual_width);
        let old_end = self.slot_allocator_.Size();
        let slot: i32;

        if actual_width == actual_alignment {
            slot = self.slot_allocator_.Allocate(slots).unwrap();
        } else {
            if actual_alignment > AlignedSlotAllocator::kSlotSize {
                let alignment_in_slots = AlignedSlotAllocator::NumSlotsForWidth(actual_alignment);
                self.slot_allocator_.Align(alignment_in_slots).unwrap();
            }
            slot = self.slot_allocator_.AllocateUnaligned(slots).unwrap();
        }

        let end = self.slot_allocator_.Size();
        self.spill_slot_count_ += end - old_end;
        let result_slot = slot + slots - 1;
        if is_tagged {
            self.tagged_slots_bits_.Add(result_slot, self.zone_);
        }
        result_slot
    }

    pub fn ensure_return_slots(&mut self, count: i32) {
        debug_assert!(!self.frame_aligned_);
        self.return_slot_count_ = std::cmp::max(self.return_slot_count_, count);
    }

    pub fn align_frame(&mut self, alignment: i32) {
        self.spill_slots_finished_ = true;
        self.frame_aligned_ = true;

        debug_assert!(bits::IsPowerOfTwo(alignment));
        let alignment_in_slots = AlignedSlotAllocator::NumSlotsForWidth(alignment);
        let mask = alignment_in_slots - 1;

        let return_delta = alignment_in_slots - (self.return_slot_count_ & mask);
        if return_delta != alignment_in_slots {
            self.return_slot_count_ += return_delta;
        }

        let delta = alignment_in_slots - (self.slot_allocator_.Size() & mask);
        if delta != alignment_in_slots {
            self.slot_allocator_.Align(alignment_in_slots).unwrap();
            if self.spill_slot_count_ != 0 {
                self.spill_slot_count_ += delta;
            }
        }
    }

    pub fn reserve_spill_slots(&mut self, slot_count: usize) -> i32 {
        debug_assert_eq!(0, self.spill_slot_count_);
        debug_assert!(!self.frame_aligned_);
        self.spill_slot_count_ += slot_count as i32;
        self.slot_allocator_.AllocateUnaligned(slot_count as i32).unwrap();
        self.slot_allocator_.Size() - 1
    }

    pub fn tagged_slots(&self) -> &GrowableBitVector {
        &self.tagged_slots_bits_
    }
}

impl FrameOffset {
    #[inline]
    pub fn from_stack_pointer(offset: i32) -> Self {
        debug_assert_eq!(0, offset & 1);
        FrameOffset { offset_: offset | FrameOffset::kFromSp }
    }

    #[inline]
    pub fn from_frame_pointer(offset: i32) -> Self {
        debug_assert_eq!(0, offset & 1);
        FrameOffset { offset_: offset | FrameOffset::kFromFp }
    }

    #[inline]
    pub fn from_stack_pointer_u32(offset: u32) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_u32(offset: u32) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_usize(offset: usize) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_usize(offset: usize) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_isize(offset: isize) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_isize(offset: isize) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_i64(offset: i64) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_i64(offset: i64) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_u64(offset: u64) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_u64(offset: u64) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_i128(offset: i128) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_i128(offset: i128) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_u128(offset: u128) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_u128(offset: u128) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_u8(offset: u8) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_u8(offset: u8) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_i8(offset: i8) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_i8(offset: i8) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_u16(offset: u16) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_u16(offset: u16) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn from_stack_pointer_i16(offset: i16) -> Self {
        Self::from_stack_pointer(offset as i32)
    }

    #[inline]
    pub fn from_frame_pointer_i16(offset: i16) -> Self {
        Self::from_frame_pointer(offset as i32)
    }

    #[inline]
    pub fn is_from_stack_pointer(&self) -> bool {
        (self.offset_ & 1) == FrameOffset::kFromSp
    }

    #[inline]
    pub fn is_from_frame_pointer(&self) -> bool {
        (self.offset_ & 1) == FrameOffset::kFromFp
    }

    #[inline]
    pub fn offset(&self) -> i32 {
        self.offset_ & !1
    }

    const kFromSp: i32 = 1;
    const kFromFp: i32 = 0;
}

impl<'a> FrameAccessState<'a> {
    pub fn new(frame: &'a Frame) -> Self {
        FrameAccessState {
            frame_: frame,
            access_frame_with_fp_: false,
            fp_relative_only_: false,
            sp_delta_: 0,
            has_frame_: false,
        }
    }

    pub fn frame(&self) -> &Frame {
        self.frame_
    }

    pub fn mark_has_frame(&mut self, state: bool) {
        self.has_frame_ = state;
        self.set_frame_access_to_default();
    }

    pub fn set_fp_relative_only(&mut self, state: bool) {
        self.fp_relative_only_ = state;
    }

    pub fn fp_relative_only(&self) -> bool {
        self.fp_relative_only_
    }

    pub fn sp_delta(&self) -> i32 {
        self.sp_delta_
    }

    pub fn clear_sp_delta(&mut self) {
        self.sp_delta_ = 0;
    }

    pub fn increase_sp_delta(&mut self, amount: i32) {
        self.sp_delta_ += amount;
    }

    pub fn access_frame_with_fp(&self) -> bool {
        self.access_frame_with_fp_
    }

    pub fn has_frame(&self) -> bool {
        self.has_frame_
    }

    pub fn set_frame_access_to_default(&mut self) {
        if self.has_frame() {
            self.set_frame_access_to_fp();
        } else {
            self.set_frame_access_to_sp();
        }
    }

    pub fn set_frame_access_to_fp(&mut self) {
        self.access_frame_with_fp_ = true;
    }

    pub fn set_frame_access_to_sp(&mut self) {
        self.access_frame_with_fp_ = false;
    }

    pub fn get_sp_to_fp_slot_count(&self) -> i32 {
        let frame_slot_count =
            (if self.has_frame() {
                self.frame().get_total_frame_slot_count()
            } else {
                kElidedFrameSlots
            }) - StandardFrameConstants::kFixedSlotCountAboveFp;
        frame_slot_count + self.sp_delta()
    }

    pub fn get_sp_to_fp_offset(&self) -> i32 {
        self.get_sp_to_fp_slot_count() * kSystemPointerSize
    }

    pub fn get_frame_offset(&self, spill_slot: i32) -> FrameOffset {
        let frame_offset = Self::frame_slot_to_fp_offset(spill_slot);
        if self.access_frame_with_fp() {
            FrameOffset::from_frame_pointer(frame_offset)
        } else {
            let sp_offset = frame_offset + self.get_sp_to_fp_offset();
            debug_assert!(sp_offset >= 0);
            FrameOffset::from_stack_pointer(sp_offset)
        }
    }

    fn frame_slot_to_fp_offset(spill_slot: i32) -> i32 {
        spill_slot * kSystemPointerSize
    }
}

pub struct Zone{}

impl Zone{
    pub fn new() -> Self {Zone{}}
}

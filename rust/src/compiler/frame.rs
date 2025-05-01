// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod frame {
    use std::cmp::{max, min};
    use std::mem::size_of;

    use crate::base::bits;
    use crate::codegen::aligned_slot_allocator::AlignedSlotAllocator;
    use crate::execution::frame_constants::*;
    use crate::utils::bit_vector::GrowableBitVector;
    use crate::utils::bit_vector::BitVector;

    // Placeholder for CallDescriptor, as its definition is not provided.
    pub struct CallDescriptor;

    const K_DOUBLE_SIZE: usize = 8;
    const K_SIMD128_SIZE: usize = 16;
    const K_SYSTEM_POINTER_SIZE: usize = 8;
    const K_ELIDED_FRAME_SLOTS: i32 = 0;

    /// Collects the spill slot and other frame slot requirements for a compiled
    /// function. Frames are usually populated by the register allocator and are used
    /// by Linkage to generate code for the prologue and epilogue to compiled
    /// code. Frame objects must be considered immutable once they've been
    /// instantiated and the basic information about the frame has been collected
    /// into them. Mutable state associated with the frame is stored separately in
    /// FrameAccessState.
    ///
    /// Frames are divided up into four regions.
    /// - The first is the fixed header, which always has a constant size and can be
    ///   predicted before code generation begins depending on the type of code being
    ///   generated.
    /// - The second is the region for spill slots, which is immediately below the
    ///   fixed header and grows as the register allocator needs to spill to the
    ///   stack and asks the frame for more space.
    /// - The third region, which contains the callee-saved registers must be
    ///   reserved after register allocation, since its size can only be precisely
    ///   determined after register allocation once the number of used callee-saved
    ///   register is certain.
    /// - The fourth region is a scratch area for return values from other functions
    ///   called, if multiple returns cannot all be passed in registers. This region
    ///   Must be last in a stack frame, so that it is positioned immediately below
    ///   the stack frame of a callee to store to.
    ///
    /// The frame region immediately below the fixed header contains spill slots
    /// starting at slot 4 for JSFunctions.  The callee-saved frame region below that
    /// starts at 4+spill_slot_count_.  Callee stack slots correspond to
    /// parameters that are accessible through negative slot ids.
    ///
    /// Every slot of a caller or callee frame is accessible by the register
    /// allocator and gap resolver with a SpillSlotOperand containing its
    /// corresponding slot id.
    ///
    /// Below an example JSFunction Frame with slot ids, frame regions and contents:
    ///
    ///  slot      JS frame
    ///       +-----------------+--------------------------------
    ///  -n-1 |  parameter n    |                            ^
    ///       |- - - - - - - - -|                            |
    ///  -n   |  parameter n-1  |                          Caller
    ///  ...  |       ...       |                       frame slots
    ///  -2   |  parameter 1    |                       (slot < 0)
    ///       |- - - - - - - - -|                            |
    ///  -1   |  parameter 0    |                            v
    ///  -----+-----------------+--------------------------------
    ///   0   |   return addr   |   ^                        ^
    ///       |- - - - - - - - -|   |                        |
    ///   1   | saved frame ptr | Fixed                      |
    ///       |- - - - - - - - -| Header <-- frame ptr       |
    ///   2   |Context/Frm. Type|   |                        |
    ///       |- - - - - - - - -|   |                        |
    ///   3   |   [JSFunction]  |   v                        |
    ///       +-----------------+----                        |
    ///   4   |    spill 1      |   ^                      Callee
    ///       |- - - - - - - - -|   |                   frame slots
    ///  ...  |      ...        | Spill slots           (slot >= 0)
    ///       |- - - - - - - - -|   |                        |
    ///  m+3  |    spill m      |   v                        |
    ///       +-----------------+----                        |
    ///  m+4  |  callee-saved 1 |   ^                        |
    ///       |- - - - - - - - -|   |                        |
    ///       |      ...        | Callee-saved               |
    ///       |- - - - - - - - -|   |                        |
    /// m+r+3 |  callee-saved r |   v                        |
    ///       +-----------------+----                        |
    /// m+r+4 |    return 0     |   ^                        |
    ///       |- - - - - - - - -|   |                        |
    ///       |      ...        | Return                     |
    ///       |- - - - - - - - -|   |                        |
    ///       |    return q-1   |   v                        v
    ///  -----+-----------------+----- <-- stack ptr -------------
    ///
    pub struct Frame {
        fixed_slot_count_: i32,
        spill_slot_count_: i32,
        return_slot_count_: i32,
        slot_allocator_: AlignedSlotAllocator,
        allocated_registers_: Option<Box<BitVector>>,
        allocated_double_registers_: Option<Box<BitVector>>,
        zone_: *mut (), // Zone*, using *mut () as a placeholder
        tagged_slots_bits_: GrowableBitVector,
        #[cfg(debug_assertions)]
        spill_slots_finished_: bool,
        #[cfg(debug_assertions)]
        frame_aligned_: bool,
    }

    impl Frame {
        pub fn new(fixed_frame_size_in_slots: i32, zone: *mut ()) -> Self {
            Frame {
                fixed_slot_count_: fixed_frame_size_in_slots,
                spill_slot_count_: 0,
                return_slot_count_: 0,
                slot_allocator_: AlignedSlotAllocator::new(),
                allocated_registers_: None,
                allocated_double_registers_: None,
                zone_: zone,
                tagged_slots_bits_: GrowableBitVector::new(),
                #[cfg(debug_assertions)]
                spill_slots_finished_: false,
                #[cfg(debug_assertions)]
                frame_aligned_: false,
            }
        }

        #[inline]
        pub fn get_total_frame_slot_count(&self) -> i32 {
            self.slot_allocator_.size() as i32 + self.return_slot_count_
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
                Some(regs) => !regs.is_empty(),
                None => true
            }
        }

        pub fn align_saved_callee_register_slots(&mut self, alignment: usize) {
            let alignment = alignment.max(K_DOUBLE_SIZE);
            debug_assert!(!self.frame_aligned_);

            #[cfg(debug_assertions)]
            {
                self.spill_slots_finished_ = true;
            }

            debug_assert!(bits::is_power_of_two(alignment as u64));
            debug_assert!(alignment <= K_SIMD128_SIZE);

            let alignment_in_slots = AlignedSlotAllocator::num_slots_for_width(alignment);
            let padding = self.slot_allocator_.align(alignment_in_slots);
            self.spill_slot_count_ += padding as i32;
        }

        pub fn allocate_saved_callee_register_slots(&mut self, count: i32) {
            debug_assert!(!self.frame_aligned_);

            #[cfg(debug_assertions)]
            {
                self.spill_slots_finished_ = true;
            }

            self.slot_allocator_.allocate_unaligned(count as usize);
        }

        pub fn allocate_spill_slot(&mut self, width: usize, alignment: usize, is_tagged: bool) -> i32 {
            debug_assert_eq!(
                self.get_total_frame_slot_count(),
                self.fixed_slot_count_ + self.spill_slot_count_ + self.return_slot_count_
            );

            debug_assert!(!is_tagged || width == size_of::<usize>());
            debug_assert!(!is_tagged || alignment == size_of::<usize>());

            // Never allocate spill slots after the callee-saved slots are defined.
            debug_assert!(!self.spill_slots_finished_);
            debug_assert!(!self.frame_aligned_);

            let actual_width = max(width, AlignedSlotAllocator::K_SLOT_SIZE);
            let actual_alignment = max(alignment, AlignedSlotAllocator::K_SLOT_SIZE);
            let slots = AlignedSlotAllocator::num_slots_for_width(actual_width);
            let old_end = self.slot_allocator_.size();

            let slot: usize;
            if actual_width == actual_alignment {
                // Simple allocation, alignment equal to width.
                slot = self.slot_allocator_.allocate(slots);
            } else {
                // Complex allocation, alignment different from width.
                if actual_alignment > AlignedSlotAllocator::K_SLOT_SIZE {
                    // Alignment required.
                    let alignment_in_slots = AlignedSlotAllocator::num_slots_for_width(actual_alignment);
                    self.slot_allocator_.align(alignment_in_slots);
                }
                slot = self.slot_allocator_.allocate_unaligned(slots);
            }

            let end = self.slot_allocator_.size();

            self.spill_slot_count_ += (end - old_end) as i32;
            let result_slot = slot + slots - 1;

            if is_tagged {
                self.tagged_slots_bits_.add(result_slot, self.zone_);
            }

            result_slot as i32
        }

        pub fn ensure_return_slots(&mut self, count: i32) {
            debug_assert!(!self.frame_aligned_);
            self.return_slot_count_ = max(self.return_slot_count_, count);
        }

        pub fn align_frame(&mut self, alignment: usize) {
            let alignment = alignment.max(K_DOUBLE_SIZE);
           // Frame::AlignFrame(int) is never called with an elided frame (EnsureFrameAlign)
            #[cfg(debug_assertions)]
            {
              self.frame_aligned_ = true;
            }

            self.align_saved_callee_register_slots(alignment);
        }

        pub fn reserve_spill_slots(&mut self, slot_count: usize) -> i32 {
            debug_assert_eq!(0, self.spill_slot_count_);
            debug_assert!(!self.frame_aligned_);
            self.spill_slot_count_ += slot_count as i32;
            self.slot_allocator_.allocate_unaligned(slot_count);
            (self.slot_allocator_.size() - 1) as i32
        }

        pub fn tagged_slots(&self) -> &GrowableBitVector {
            &self.tagged_slots_bits_
        }
    }

    /// Represents an offset from either the stack pointer or frame pointer.
    #[derive(Clone, Copy)]
    pub struct FrameOffset {
        offset_: i32, // Encodes SP or FP in the low order bit.
    }

    impl FrameOffset {
        #[inline]
        pub fn from_stack_pointer(&self) -> bool {
            (self.offset_ & 1) == Self::K_FROM_SP
        }

        #[inline]
        pub fn from_frame_pointer(&self) -> bool {
            (self.offset_ & 1) == Self::K_FROM_FP
        }

        #[inline]
        pub fn offset(&self) -> i32 {
            self.offset_ & !1
        }

        #[inline]
        pub fn new_from_stack_pointer(offset: i32) -> Self {
            debug_assert_eq!(0, offset & 1);
            FrameOffset { offset_: offset | Self::K_FROM_SP }
        }

        #[inline]
        pub fn new_from_frame_pointer(offset: i32) -> Self {
            debug_assert_eq!(0, offset & 1);
            FrameOffset { offset_: offset | Self::K_FROM_FP }
        }

        const K_FROM_SP: i32 = 1;
        const K_FROM_FP: i32 = 0;
    }

    /// Encapsulates the mutable state maintained during code generation about the
    /// current function's frame.
    pub struct FrameAccessState {
        frame_: *const Frame,
        access_frame_with_fp_: bool,
        fp_relative_only_: bool,
        sp_delta_: i32,
        has_frame_: bool,
    }

    impl FrameAccessState {
        pub fn new(frame: *const Frame) -> Self {
            FrameAccessState {
                frame_: frame,
                access_frame_with_fp_: false,
                fp_relative_only_: false,
                sp_delta_: 0,
                has_frame_: false,
            }
        }

        pub fn frame(&self) -> *const Frame {
            self.frame_
        }

        pub fn mark_has_frame(&mut self, state: bool) {
            self.has_frame_ = state;
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

        // Regardless of how we access slots on the stack - using sp or fp - do we
        // have a frame, at the current stage in code generation.
        pub fn has_frame(&self) -> bool {
            self.has_frame_
        }

        pub fn set_frame_access_to_default(&mut self) {
            // Placeholder implementation.  Original C++ code has platform-specific behavior.
            // For now, we simply set access_frame_with_fp_ to false.
            self.set_frame_access_to_sp();
        }

        pub fn set_frame_access_to_fp(&mut self) {
            self.access_frame_with_fp_ = true;
        }

        pub fn set_frame_access_to_sp(&mut self) {
            self.access_frame_with_fp_ = false;
        }

        pub fn get_sp_to_fp_slot_count(&self) -> i32 {
            let frame: &Frame = unsafe { &*self.frame_ };
            let frame_slot_count =
                (if self.has_frame() { frame.get_total_frame_slot_count() } else { K_ELIDED_FRAME_SLOTS }) -
                k_fixed_slot_count_above_fp();
            frame_slot_count + self.sp_delta()
        }

        pub fn get_sp_to_fp_offset(&self) -> i32 {
            self.get_sp_to_fp_slot_count() * K_SYSTEM_POINTER_SIZE as i32
        }

        // Get the frame offset for a given spill slot. The location depends on the
        // calling convention and the specific frame layout, and may thus be
        // architecture-specific. Negative spill slots indicate arguments on the
        // caller's frame.
        pub fn get_frame_offset(&self, spill_slot: i32) -> FrameOffset {
            // Placeholder implementation.  Original C++ code has platform-specific behavior.
            // This is a simplified version that assumes frame pointer access.
            let offset = spill_slot * K_SYSTEM_POINTER_SIZE as i32;
            FrameOffset::new_from_frame_pointer(offset)
        }
    }
} // mod frame

mod base {
    pub mod bits {
        pub fn is_power_of_two(x: u64) -> bool {
            (x != 0) && ((x & (x - 1)) == 0)
        }
    }
}

mod codegen {
    pub mod aligned_slot_allocator {
        use std::cmp::max;

        #[derive(Default)]
        pub struct AlignedSlotAllocator {
            size_: usize,
        }

        impl AlignedSlotAllocator {
            pub const K_SLOT_SIZE: usize = 8;

            pub fn new() -> Self {
                AlignedSlotAllocator { size_: 0 }
            }

            pub fn size(&self) -> usize {
                self.size_
            }

            pub fn allocate(&mut self, slots: usize) -> usize {
                let result = self.size_;
                self.allocate_unaligned(slots);
                result
            }

            pub fn allocate_unaligned(&mut self, slots: usize) {
                self.size_ += slots;
            }

            pub fn align(&mut self, alignment_in_slots: usize) -> usize {
                let mask = alignment_in_slots - 1;
                let mis_alignment = self.size_ & mask;
                if mis_alignment == 0 {
                    0 // Already aligned
                } else {
                    let padding = alignment_in_slots - mis_alignment;
                    self.size_ += padding;
                    padding
                }
            }

            pub fn num_slots_for_width(width: usize) -> usize {
                max(1, (width + Self::K_SLOT_SIZE - 1) / Self::K_SLOT_SIZE)
            }
        }
    }
}

mod execution {
    pub mod frame_constants {
        pub fn k_fixed_slot_count_above_fp() -> i32 {
            3
        }
    }
}

mod utils {
    pub mod bit_vector {
        use std::collections::HashSet;

        #[derive(Debug, Default)]
        pub struct GrowableBitVector {
            bits: HashSet<usize>,
        }

        impl GrowableBitVector {
            pub fn new() -> Self {
                GrowableBitVector {
                    bits: HashSet::new(),
                }
            }

            pub fn add(&mut self, bit: usize, _zone: *mut ()) {
                // Zone is ignored in this simplified implementation
                self.bits.insert(bit);
            }

            pub fn contains(&self, bit: usize) -> bool {
                self.bits.contains(&bit)
            }

            pub fn is_empty(&self) -> bool {
              self.bits.is_empty()
            }
        }

        pub struct BitVector {
            bits: HashSet<usize>,
        }

        impl BitVector {

            pub fn new() -> Self {
                BitVector {
                    bits: HashSet::new(),
                }
            }
        
            pub fn is_empty(&self) -> bool {
              self.bits.is_empty()
            }
        }
    }
}
// Converted from V8 C++ source files:
// Header: aligned-slot-allocator.h
// Implementation: aligned-slot-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/aligned-slot-allocator.h
use std::cmp;

const kSystemPointerSize: i32 = 8; // Assuming 64-bit architecture

pub struct AlignedSlotAllocator {
    next1_: i32,
    next2_: i32,
    next4_: i32,
    size_: i32,
}

impl AlignedSlotAllocator {
    pub const kSlotSize: i32 = kSystemPointerSize;

    pub fn num_slots_for_width(bytes: i32) -> i32 {
        assert!(bytes > 0);
        (bytes + Self::kSlotSize - 1) / Self::kSlotSize
    }

    pub fn new() -> Self {
        AlignedSlotAllocator {
            next1_: Self::kInvalidSlot,
            next2_: Self::kInvalidSlot,
            next4_: 0,
            size_: 0,
        }
    }

    pub fn allocate(&mut self, n: i32) -> i32 {
        assert!(n == 1 || n == 2 || n == 4);
        assert_eq!(0, self.next4_ & 3);
        if Self::is_valid(self.next2_) {
            assert_eq!(0, self.next2_ & 1);
        }

        let mut result = Self::kInvalidSlot;
        match n {
            1 => {
                if Self::is_valid(self.next1_) {
                    result = self.next1_;
                    self.next1_ = Self::kInvalidSlot;
                } else if Self::is_valid(self.next2_) {
                    result = self.next2_;
                    self.next1_ = result + 1;
                    self.next2_ = Self::kInvalidSlot;
                } else {
                    result = self.next4_;
                    self.next1_ = result + 1;
                    self.next2_ = result + 2;
                    self.next4_ += 4;
                }
            }
            2 => {
                if Self::is_valid(self.next2_) {
                    result = self.next2_;
                    self.next2_ = Self::kInvalidSlot;
                } else {
                    result = self.next4_;
                    self.next2_ = result + 2;
                    self.next4_ += 4;
                }
            }
            4 => {
                result = self.next4_;
                self.next4_ += 4;
            }
            _ => unreachable!(),
        }
        assert!(Self::is_valid(result));
        self.size_ = cmp::max(self.size_, result + n);
        result
    }

    pub fn next_slot(&self, n: i32) -> i32 {
        assert!(n == 1 || n == 2 || n == 4);
        if n <= 1 && Self::is_valid(self.next1_) {
            return self.next1_;
        }
        if n <= 2 && Self::is_valid(self.next2_) {
            return self.next2_;
        }
        assert!(Self::is_valid(self.next4_));
        self.next4_
    }

    pub fn allocate_unaligned(&mut self, n: i32) -> i32 {
        assert!(n >= 0);
        assert_eq!(0, self.next4_ & 3);
        if Self::is_valid(self.next2_) {
            assert_eq!(0, self.next2_ & 1);
        }

        let result = self.size_;
        self.size_ += n;
        match self.size_ & 3 {
            0 => {
                self.next1_ = Self::kInvalidSlot;
                self.next2_ = Self::kInvalidSlot;
                self.next4_ = self.size_;
            }
            1 => {
                self.next1_ = self.size_;
                self.next2_ = self.size_ + 1;
                self.next4_ = self.size_ + 3;
            }
            2 => {
                self.next1_ = Self::kInvalidSlot;
                self.next2_ = self.size_;
                self.next4_ = self.size_ + 2;
            }
            3 => {
                self.next1_ = self.size_;
                self.next2_ = Self::kInvalidSlot;
                self.next4_ = self.size_ + 1;
            }
            _ => unreachable!(),
        }
        result
    }

    pub fn align(&mut self, n: i32) -> i32 {
        assert!(n.is_power_of_two());
        assert!(n <= 4);
        let mask = n - 1;
        let misalignment = self.size_ & mask;
        let padding = (n - misalignment) & mask;
        self.allocate_unaligned(padding);
        padding
    }

    pub fn size(&self) -> i32 {
        self.size_
    }

    const kInvalidSlot: i32 = -1;

    fn is_valid(slot: i32) -> bool {
        slot > Self::kInvalidSlot
    }
}

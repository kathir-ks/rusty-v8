// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/aligned-slot-allocator.h (Module definition)

const K_INVALID_SLOT: i32 = -1;

#[inline]
fn is_valid(slot: i32) -> bool {
    slot != K_INVALID_SLOT
}

#[derive(Debug)]
pub struct AlignedSlotAllocator {
    next1_: i32,
    next2_: i32,
    next4_: i32,
    size_: i32,
}

impl AlignedSlotAllocator {
    pub fn new() -> Self {
        AlignedSlotAllocator {
            next1_: K_INVALID_SLOT,
            next2_: K_INVALID_SLOT,
            next4_: 0,
            size_: 0,
        }
    }

    pub fn next_slot(&self, n: i32) -> i32 {
        debug_assert!(n == 1 || n == 2 || n == 4);
        if n <= 1 && is_valid(self.next1_) {
            return self.next1_;
        }
        if n <= 2 && is_valid(self.next2_) {
            return self.next2_;
        }
        debug_assert!(is_valid(self.next4_));
        self.next4_
    }

    pub fn allocate(&mut self, n: i32) -> i32 {
        debug_assert!(n == 1 || n == 2 || n == 4);
        debug_assert_eq!(0, self.next4_ & 3);
        debug_assert!(!is_valid(self.next2_) || (self.next2_ & 1) == 0);

        let mut result = K_INVALID_SLOT;
        match n {
            1 => {
                if is_valid(self.next1_) {
                    result = self.next1_;
                    self.next1_ = K_INVALID_SLOT;
                } else if is_valid(self.next2_) {
                    result = self.next2_;
                    self.next1_ = result + 1;
                    self.next2_ = K_INVALID_SLOT;
                } else {
                    result = self.next4_;
                    self.next1_ = result + 1;
                    self.next2_ = result + 2;
                    self.next4_ += 4;
                }
            }
            2 => {
                if is_valid(self.next2_) {
                    result = self.next2_;
                    self.next2_ = K_INVALID_SLOT;
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
        debug_assert!(is_valid(result));
        self.size_ = std::cmp::max(self.size_, result + n);
        result
    }

    pub fn allocate_unaligned(&mut self, n: i32) -> i32 {
        debug_assert!(n >= 0);
        debug_assert_eq!(0, self.next4_ & 3);
        debug_assert!(!is_valid(self.next2_) || (self.next2_ & 1) == 0);

        let result = self.size_;
        self.size_ += n;
        match self.size_ & 3 {
            0 => {
                self.next1_ = K_INVALID_SLOT;
                self.next2_ = K_INVALID_SLOT;
                self.next4_ = self.size_;
            }
            1 => {
                self.next1_ = self.size_;
                self.next2_ = self.size_ + 1;
                self.next4_ = self.size_ + 3;
            }
            2 => {
                self.next1_ = K_INVALID_SLOT;
                self.next2_ = self.size_;
                self.next4_ = self.size_ + 2;
            }
            3 => {
                self.next1_ = self.size_;
                self.next2_ = K_INVALID_SLOT;
                self.next4_ = self.size_ + 1;
            }
            _ => unreachable!(),
        }
        result
    }

    pub fn align(&mut self, n: i32) -> i32 {
        debug_assert!(n.is_power_of_two());
        debug_assert!(n <= 4);
        let mask = n - 1;
        let misalignment = self.size_ & mask;
        let padding = (n - misalignment) & mask;
        self.allocate_unaligned(padding);
        padding
    }
}
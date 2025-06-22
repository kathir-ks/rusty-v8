// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Sub};

pub mod base {
    pub mod bits {
        pub fn count_population(x: u64) -> u32 {
            x.count_ones()
        }

        pub fn count_trailing_zeros_nonzero(x: u64) -> u32 {
            x.trailing_zeros()
        }

        pub fn count_leading_zeros(x: u64) -> u32 {
            x.leading_zeros()
        }
    }

    pub mod iterator {
        pub struct ForwardIteratorTag;
    }
}

pub mod template_utils {
    // Placeholder for template_utils functionality.  Implementation depends on
    // the specific use cases, which are not evident in the provided header.
}

pub mod internal {
    pub trait RegisterTrait: Copy + Clone + PartialEq + Eq + fmt::Display {
        const K_NUM_REGISTERS: u32;
        fn code(self) -> u32;
        fn from_code(code: u32) -> Self;
        fn is_valid(self) -> bool;
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Register(u32);

    impl RegisterTrait for Register {
        const K_NUM_REGISTERS: u32 = 64; // Example value
        fn code(self) -> u32 {
            self.0
        }
        fn from_code(code: u32) -> Self {
            Register(code)
        }
        fn is_valid(self) -> bool {
            self.0 < Self::K_NUM_REGISTERS
        }
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "R{}", self.0)
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct RegListBase<RegisterT: RegisterTrait> {
        regs_: u64, //Using u64 by default since ARM64 part is conditionally compiled in C++
        _phantom: std::marker::PhantomData<RegisterT>,
    }

    impl<RegisterT: RegisterTrait> RegListBase<RegisterT> {
        pub const fn new() -> Self {
            RegListBase {
                regs_: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn from_bits(bits: u64) -> Self {
            RegListBase {
                regs_: bits,
                _phantom: std::marker::PhantomData,
            }
        }

        pub const fn set(&mut self, reg: RegisterT) {
            if !reg.is_valid() {
                return;
            }
            self.regs_ |= 1 << reg.code();
        }

        pub const fn clear(&mut self, reg: RegisterT) {
            if !reg.is_valid() {
                return;
            }
            self.regs_ &= !(1 << reg.code());
        }

        pub const fn has(&self, reg: RegisterT) -> bool {
            if !reg.is_valid() {
                return false;
            }
            (self.regs_ & (1 << reg.code())) != 0
        }

        pub const fn clear_other(&mut self, other: RegListBase<RegisterT>) {
            self.regs_ &= !other.regs_;
        }

        pub const fn is_empty(&self) -> bool {
            self.regs_ == 0
        }

        pub const fn count(&self) -> u32 {
            crate::base::bits::count_population(self.regs_)
        }
    }

    impl<RegisterT: RegisterTrait> BitAnd for RegListBase<RegisterT> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ & other.regs_)
        }
    }

    impl<RegisterT: RegisterTrait> BitOr for RegListBase<RegisterT> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ | other.regs_)
        }
    }

    impl<RegisterT: RegisterTrait> BitXor for RegListBase<RegisterT> {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ ^ other.regs_)
        }
    }

    impl<RegisterT: RegisterTrait> Sub for RegListBase<RegisterT> {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ & !other.regs_)
        }
    }

    impl<RegisterT: RegisterTrait> BitAndAssign for RegListBase<RegisterT> {
        fn bitand_assign(&mut self, other: Self) {
            self.regs_ &= other.regs_;
        }
    }

    impl<RegisterT: RegisterTrait> BitOrAssign for RegListBase<RegisterT> {
        fn bitor_assign(&mut self, other: Self) {
            self.regs_ |= other.regs_;
        }
    }

    impl<RegisterT: RegisterTrait> PartialEq for RegListBase<RegisterT> {
        fn eq(&self, other: &Self) -> bool {
            self.regs_ == other.regs_
        }
    }

    impl<RegisterT: RegisterTrait> RegListBase<RegisterT> {
        pub fn first(&self) -> RegisterT {
            assert!(!self.is_empty());
            let first_code = crate::base::bits::count_trailing_zeros_nonzero(self.regs_);
            RegisterT::from_code(first_code as u32)
        }

        pub fn last(&self) -> RegisterT {
            assert!(!self.is_empty());
            let last_code =
                (8 * std::mem::size_of::<u64>() as u32) - 1 - crate::base::bits::count_leading_zeros(self.regs_) as u32;
            RegisterT::from_code(last_code)
        }

        pub fn pop_first(&mut self) -> RegisterT {
            let reg = self.first();
            self.clear(reg);
            reg
        }

        pub const fn bits(&self) -> u64 {
            self.regs_
        }

        pub fn iter(&self) -> Iterator<RegisterT> {
            Iterator {
                remaining_: *self,
            }
        }

        pub fn riter(&self) -> ReverseIterator<RegisterT> {
            ReverseIterator {
                remaining_: *self,
            }
        }
    }

    impl<RegisterT: RegisterTrait> FromIterator<RegisterT> for RegListBase<RegisterT> {
        fn from_iter<I: IntoIterator<Item = RegisterT>>(iter: I) -> Self {
            let mut reg_list = RegListBase::new();
            for reg in iter {
                reg_list.set(reg);
            }
            reg_list
        }
    }

    pub struct Iterator<RegisterT: RegisterTrait> {
        remaining_: RegListBase<RegisterT>,
    }

    impl<RegisterT: RegisterTrait> Iterator<RegisterT> {
        fn new(reg_list: RegListBase<RegisterT>) -> Self {
            Iterator {
                remaining_: reg_list,
            }
        }
    }

    impl<RegisterT: RegisterTrait> std::iter::Iterator for Iterator<RegisterT> {
        type Item = RegisterT;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_.is_empty() {
                return None;
            }
            let first = self.remaining_.first();
            self.remaining_.clear(first);
            Some(first)
        }
    }

    pub struct ReverseIterator<RegisterT: RegisterTrait> {
        remaining_: RegListBase<RegisterT>,
    }

    impl<RegisterT: RegisterTrait> ReverseIterator<RegisterT> {
        fn new(reg_list: RegListBase<RegisterT>) -> Self {
            ReverseIterator {
                remaining_: reg_list,
            }
        }
    }

    impl<RegisterT: RegisterTrait> std::iter::Iterator for ReverseIterator<RegisterT> {
        type Item = RegisterT;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_.is_empty() {
                return None;
            }
            let last = self.remaining_.last();
            self.remaining_.clear(last);
            Some(last)
        }
    }

    impl<RegisterT: RegisterTrait> fmt::Display for RegListBase<RegisterT> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let mut reglist = *self;
            let mut first = true;
            while !reglist.is_empty() {
                let reg = reglist.first();
                reglist.clear(reg);
                write!(f, "{}{}", if first { "" } else { ", " }, reg)?;
                first = false;
            }
            write!(f, "}}")
        }
    }
}
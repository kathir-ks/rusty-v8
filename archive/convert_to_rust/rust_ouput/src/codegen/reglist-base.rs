// Converted from V8 C++ source files:
// Header: reglist-base.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod bits {
        pub fn CountPopulation(bits: u64) -> u32 {
            bits.count_ones()
        }

        pub fn CountTrailingZerosNonZero(bits: u64) -> i32 {
           bits.trailing_zeros() as i32
        }

        pub fn CountLeadingZeros(bits: u64) -> i32 {
            bits.leading_zeros() as i32
        }
    }
    pub mod iterator {
        pub struct iterator<T1, T2> {
            _phantom: ::std::marker::PhantomData<(T1, T2)>,
        }
    }
}
pub mod internal {
    use std::ops::{BitAnd, BitOr, BitXor, Sub, Not};

    pub trait RegisterT {
        const kNumRegisters: u32;
        fn code(&self) -> i8;
        fn from_code(code: i32) -> Self;
        fn is_valid(&self) -> bool;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code_: i8,
    }

    impl Register {
        pub fn new(code: i8) -> Self {
            Register { code_: code }
        }
    }

    impl RegisterT for Register {
        const kNumRegisters: u32 = 64;

        fn code(&self) -> i8 {
            self.code_
        }

        fn from_code(code: i32) -> Self {
            Register { code_: code as i8 }
        }

        fn is_valid(&self) -> bool {
            self.code_ >= 0
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct RegListBase<RegisterT: RegisterT> {
        regs_: u64,
        _phantom: std::marker::PhantomData<RegisterT>,
    }

    impl<RegisterT: RegisterT> RegListBase<RegisterT> {
        pub const fn new() -> Self {
            RegListBase {
                regs_: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub const fn from_initializer_list(regs: &[RegisterT]) -> Self {
            let mut reg_list = RegListBase::new();
            for &reg in regs {
                reg_list.set(reg);
            }
            reg_list
        }

        pub const fn set(&mut self, reg: RegisterT) {
            if !reg.is_valid() {
                return;
            }
            self.regs_ |= 1u64 << reg.code();
        }

        pub const fn clear(&mut self, reg: RegisterT) {
            if !reg.is_valid() {
                return;
            }
            self.regs_ &= !(1u64 << reg.code());
        }

        pub const fn has(&self, reg: RegisterT) -> bool {
            if !reg.is_valid() {
                return false;
            }
            (self.regs_ & (1u64 << reg.code())) != 0
        }

        pub const fn clear_other(&mut self, other: RegListBase<RegisterT>) {
            self.regs_ &= !other.regs_;
        }

        pub const fn is_empty(&self) -> bool {
            self.regs_ == 0
        }

        pub const fn count(&self) -> u32 {
            base::bits::CountPopulation(self.regs_)
        }

        pub const fn bit_and(&self, other: RegListBase<RegisterT>) -> RegListBase<RegisterT> {
            RegListBase::from_bits(self.regs_ & other.regs_)
        }

        pub const fn bit_or(&self, other: RegListBase<RegisterT>) -> RegListBase<RegisterT> {
            RegListBase::from_bits(self.regs_ | other.regs_)
        }

        pub const fn bit_xor(&self, other: RegListBase<RegisterT>) -> RegListBase<RegisterT> {
            RegListBase::from_bits(self.regs_ ^ other.regs_)
        }

        pub const fn sub(&self, other: RegListBase<RegisterT>) -> RegListBase<RegisterT> {
             RegListBase::from_bits(self.regs_ & !other.regs_)
        }

        pub const fn bit_or_reg(&self, reg: RegisterT) -> RegListBase<RegisterT> {
            self.bit_or(RegListBase::from_initializer_list(&[reg]))
        }

        pub const fn sub_reg(&self, reg: RegisterT) -> RegListBase<RegisterT> {
            self.sub(RegListBase::from_initializer_list(&[reg]))
        }

        pub const fn bitand_assign(&mut self, other: RegListBase<RegisterT>) {
            self.regs_ &= other.regs_;
        }

        pub const fn bitor_assign(&mut self, other: RegListBase<RegisterT>) {
            self.regs_ |= other.regs_;
        }

        pub const fn bits(&self) -> u64 {
            self.regs_
        }

        pub fn first(&self) -> RegisterT {
            assert!(!self.is_empty());
            let first_code = base::bits::CountTrailingZerosNonZero(self.regs_);
            RegisterT::from_code(first_code)
        }

         pub fn last(&self) -> RegisterT {
            assert!(!self.is_empty());
            let last_code =
                8 * std::mem::size_of::<u64>() as i32 - 1 - base::bits::CountLeadingZeros(self.regs_);
            RegisterT::from_code(last_code)
        }

        pub fn pop_first(&mut self) -> RegisterT {
            let reg = self.first();
            self.clear(reg);
            reg
        }

        pub fn from_bits(bits: u64) -> Self {
            RegListBase {
                regs_: bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<RegisterT: RegisterT> From<u64> for RegListBase<RegisterT> {
        fn from(bits: u64) -> Self {
            RegListBase {
                regs_: bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<RegisterT: RegisterT> BitAnd for RegListBase<RegisterT> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ & other.regs_)
        }
    }

    impl<RegisterT: RegisterT> BitOr for RegListBase<RegisterT> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            RegListBase::from_bits(self.regs_ | other.regs_)
        }
    }

    impl<RegisterT: RegisterT> BitXor for RegListBase<RegisterT> {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self {
             RegListBase::from_bits(self.regs_ ^ other.regs_)
        }
    }

    impl<RegisterT: RegisterT> Sub for RegListBase<RegisterT> {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
             RegListBase::from_bits(self.regs_ & !other.regs_)
        }
    }

    impl<RegisterT: RegisterT> PartialEq for RegListBase<RegisterT> {
        fn eq(&self, other: &Self) -> bool {
            self.regs_ == other.regs_
        }
    }

    impl<RegisterT: RegisterT> std::fmt::Display for RegListBase<RegisterT> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{")?;
            let mut reglist = *self;
            let mut first = true;
            while !reglist.is_empty() {
                let reg = reglist.first();
                reglist.clear(reg);
                write!(f, "{}{}", if first { "" } else { ", " }, reg.code())?;
                first = false;
            }
            write!(f, "}}")
        }
    }

    pub struct RegListBaseIterator<RegisterT: RegisterT> {
        remaining_: RegListBase<RegisterT>,
    }

    impl<RegisterT: RegisterT> RegListBaseIterator<RegisterT> {
        fn new(remaining: RegListBase<RegisterT>) -> Self {
            RegListBaseIterator { remaining_: remaining }
        }
    }

    impl<RegisterT: RegisterT> Iterator for RegListBaseIterator<RegisterT> {
        type Item = RegisterT;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_.is_empty() {
                return None;
            }
            let reg = self.remaining_.first();
            self.remaining_.clear(reg);
            Some(reg)
        }
    }

    impl<RegisterT: RegisterT> IntoIterator for RegListBase<RegisterT> {
        type Item = RegisterT;
        type IntoIter = RegListBaseIterator<RegisterT>;

        fn into_iter(self) -> Self::IntoIter {
            RegListBaseIterator::new(self)
        }
    }

    pub struct RegListBaseReverseIterator<RegisterT: RegisterT> {
        remaining_: RegListBase<RegisterT>,
    }

    impl<RegisterT: RegisterT> RegListBaseReverseIterator<RegisterT> {
        fn new(remaining: RegListBase<RegisterT>) -> Self {
            RegListBaseReverseIterator { remaining_: remaining }
        }
    }

    impl<RegisterT: RegisterT> Iterator for RegListBaseReverseIterator<RegisterT> {
        type Item = RegisterT;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_.is_empty() {
                return None;
            }
            let reg = self.remaining_.last();
            self.remaining_.clear(reg);
            Some(reg)
        }
    }

    pub struct ReverseRegListBase<RegisterT: RegisterT>(RegListBase<RegisterT>);

    impl<RegisterT: RegisterT> IntoIterator for ReverseRegListBase<RegisterT> {
        type Item = RegisterT;
        type IntoIter = RegListBaseReverseIterator<RegisterT>;

        fn into_iter(self) -> Self::IntoIter {
            RegListBaseReverseIterator::new(self.0)
        }
    }
}
}

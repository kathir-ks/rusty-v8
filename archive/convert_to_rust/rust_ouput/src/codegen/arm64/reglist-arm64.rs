// Converted from V8 C++ source files:
// Header: reglist-arm64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arm64 {
pub mod utils_arm64 {
// Empty module as the corresponding C++ file contains only utility functions,
// and their Rust equivalents will be implemented directly in the code where they are used.
}
}
pub mod codegen {
pub mod register_arch {
// Empty module as the corresponding C++ file contains only register architecture definitions.
}
pub mod reglist_base {
// Empty module as the corresponding C++ file contains only reglist base definitions.
}
}
pub mod common {
pub mod globals {
// Empty module as the corresponding C++ file contains only global definitions.
}
}
use std::ops::BitOr;
use std::ops::BitAnd;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Register {
    code_: i32,
}

impl Register {
    pub fn code(&self) -> i32 {
        self.code_
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DoubleRegister {
    code_: i32,
}

impl DoubleRegister {
    pub fn code(&self) -> i32 {
        self.code_
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegListBase<T> {
    bits_: u64,
}

impl<T> RegListBase<T> {
    pub fn bits(&self) -> u64 {
        self.bits_
    }
}

impl<T> From<u64> for RegListBase<T> {
    fn from(bits: u64) -> Self {
        RegListBase { bits_: bits }
    }
}

type RegList = RegListBase<Register>;
type DoubleRegList = RegListBase<DoubleRegister>;

const kBitsPerByte: usize = 8;

const kRegListSizeInBits: usize = std::mem::size_of::<RegList>() * kBitsPerByte;

pub struct CPURegList {
    list_: u64,
    size_: i32,
    type_: CPURegister::RegisterType,
}

#[allow(non_upper_case_globals)]
static NoCPUReg: CPURegister = CPURegister {
    code_: -1,
    type_: CPURegister::RegisterType::kNoRegister,
    size_: 0
};

impl CPURegList {
    pub fn new(size: i32, list: RegList) -> Self {
        CPURegList {
            list_: list.bits(),
            size_: size,
            type_: CPURegister::RegisterType::kRegister,
        }
    }

    pub fn new_double(size: i32, list: DoubleRegList) -> Self {
        CPURegList {
            list_: list.bits(),
            size_: size,
            type_: CPURegister::RegisterType::kVRegister,
        }
    }

   pub fn from_registers(
        reg0: CPURegister,
        regs: &[CPURegister],
    ) -> Self {
        let mut list = 1u64 << reg0.code();
        let mut size = reg0.size_in_bits();
        let type_ = reg0.type_;

        for reg in regs {
            if reg.is_valid() {
                list |= 1u64 << reg.code();
            }
           
        }

         let cpur_list = CPURegList{
                list_: list,
                size_: size,
                type_: type_,
            };
            assert!(cpur_list.is_valid());
         cpur_list
    }


    pub fn new_range(
        type_: CPURegister::RegisterType,
        size: i32,
        first_reg: i32,
        last_reg: i32,
    ) -> Self {
        assert!(
            ((type_ == CPURegister::RegisterType::kRegister) && (last_reg < kNumberOfRegisters as i32))
                || ((type_ == CPURegister::RegisterType::kVRegister)
                    && (last_reg < kNumberOfVRegisters as i32))
        );
        assert!(last_reg >= first_reg);
        let mut list = ((1u64 << (last_reg + 1)) - 1);
        list &= !((1u64 << first_reg) - 1);
        let cpur_list = CPURegList {
            list_: list,
            size_: size,
            type_: type_,
        };
        assert!(cpur_list.is_valid());
        cpur_list
    }

    pub fn type_(&self) -> CPURegister::RegisterType {
        self.type_
    }

    pub fn bits(&self) -> u64 {
        self.list_
    }

    pub fn set_bits(&mut self, new_bits: u64) {
        self.list_ = new_bits;
        assert!(self.is_valid());
    }

    pub fn combine(&mut self, other: &CPURegList) {
        assert_eq!(self.type_, other.type_);
        assert_eq!(self.size_, other.size_);
        self.list_ |= other.list_;
    }

    pub fn remove(&mut self, other: &CPURegList) {
        assert_eq!(self.type_, other.type_);
        self.list_ &= !other.list_;
    }

    pub fn combine_register(&mut self, other: &CPURegister) {
        if other.type() == self.type_ {
            self.list_ |= 1u64 << other.code();
        }
    }

    pub fn remove_register(
        &mut self,
        other1: &CPURegister,
        other2: &CPURegister,
        other3: &CPURegister,
        other4: &CPURegister,
    ) {
        if other1.type() == self.type_ && !other1.is_none() {
            self.list_ &= !(1u64 << other1.code());
        }
        if other2.type() == self.type_ && !other2.is_none() {
            self.list_ &= !(1u64 << other2.code());
        }
        if other3.type() == self.type_ && !other3.is_none() {
            self.list_ &= !(1u64 << other3.code());
        }
        if other4.type() == self.type_ && !other4.is_none() {
            self.list_ &= !(1u64 << other4.code());
        }
    }

    pub fn combine_code(&mut self, code: i32) {
        self.list_ |= 1u64 << code;
    }

    pub fn remove_code(&mut self, code: i32) {
        self.list_ &= !(1u64 << code);
    }

    pub fn align(&mut self) {
        // Placeholder implementation
        // Add actual alignment logic here if needed
    }

    pub fn pop_lowest_index(&mut self) -> CPURegister {
        if self.list_ == 0 {
            return NoCPUReg;
        }

        let lowest_bit = self.list_.trailing_zeros() as i32;
        self.list_ &= !(1u64 << lowest_bit);

        CPURegister {
            code_: lowest_bit,
            type_: self.type_,
            size_: self.size_,
        }
    }

    pub fn pop_highest_index(&mut self) -> CPURegister {
        if self.list_ == 0 {
            return NoCPUReg;
        }

        let highest_bit = 63 - self.list_.leading_zeros() as i32;
        self.list_ &= !(1u64 << highest_bit);

        CPURegister {
            code_: highest_bit,
            type_: self.type_,
            size_: self.size_,
        }
    }

    pub fn get_callee_saved(size: i32) -> CPURegList {
        // Placeholder implementation
        CPURegList {
            list_: 0x0,
            size_: size,
            type_: CPURegister::RegisterType::kRegister,
        }
    }

    pub fn get_callee_saved_v(size: i32) -> CPURegList {
        // Placeholder implementation
        CPURegList {
            list_: 0x0,
            size_: size,
            type_: CPURegister::RegisterType::kVRegister,
        }
    }

    pub fn get_caller_saved(size: i32) -> CPURegList {
        // Placeholder implementation
        CPURegList {
            list_: 0x0,
            size_: size,
            type_: CPURegister::RegisterType::kRegister,
        }
    }

    pub fn get_caller_saved_v(size: i32) -> CPURegList {
        // Placeholder implementation
        CPURegList {
            list_: 0x0,
            size_: size,
            type_: CPURegister::RegisterType::kVRegister,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list_ == 0
    }

    pub fn includes_alias_of(
        &self,
        other1: &CPURegister,
        other2: &CPURegister,
        other3: &CPURegister,
        other4: &CPURegister,
    ) -> bool {
        let mut list: u64 = 0;
        if !other1.is_none() && (other1.type() == self.type_) {
            list |= 1u64 << other1.code();
        }
        if !other2.is_none() && (other2.type() == self.type_) {
            list |= 1u64 << other2.code();
        }
        if !other3.is_none() && (other3.type() == self.type_) {
            list |= 1u64 << other3.code();
        }
        if !other4.is_none() && (other4.type() == self.type_) {
            list |= 1u64 << other4.code();
        }
        (self.list_ & list) != 0
    }

    fn count_set_bits(mut n: u64, size: usize) -> i32 {
        let mut count = 0;
        for _ in 0..size {
            count += (n & 1) as i32;
            n >>= 1;
        }
        count
    }

    pub fn count(&self) -> i32 {
         CPURegList::count_set_bits(self.list_, kRegListSizeInBits)
    }

    pub fn register_size_in_bits(&self) -> i32 {
        self.size_
    }

    pub fn register_size_in_bytes(&self) -> i32 {
        let size_in_bits = self.register_size_in_bits();
        assert_eq!(size_in_bits % kBitsPerByte as i32, 0);
        size_in_bits / kBitsPerByte as i32
    }

    pub fn total_size_in_bytes(&self) -> i32 {
        self.register_size_in_bytes() * self.count()
    }

    fn is_valid(&self) -> bool {
        const kValidRegisters: u64 = 0x8000000ffffffff;
        const kValidVRegisters: u64 = 0x0000000ffffffff;
        match self.type_ {
            CPURegister::RegisterType::kRegister => (self.list_ & kValidRegisters) == self.list_,
            CPURegister::RegisterType::kVRegister => (self.list_ & kValidVRegisters) == self.list_,
            CPURegister::RegisterType::kNoRegister => self.list_ == 0,
            _ => panic!("UNREACHABLE"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CPURegister {
    code_: i32,
    type_: CPURegister::RegisterType,
    size_: i32,
}

impl CPURegister {
    pub fn code(&self) -> i32 {
        self.code_
    }

    pub fn type(&self) -> CPURegister::RegisterType {
        self.type_
    }
    pub fn size_in_bits(&self) -> i32 {
        self.size_
    }
    pub fn is_none(&self) -> bool {
        self.type_ == CPURegister::RegisterType::kNoRegister
    }

    pub fn is_valid(&self) -> bool {
       self.code_ >= 0
    }
}

pub mod CPURegister {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RegisterType {
        kNoRegister,
        kRegister,
        kVRegister,
    }
}

const kXRegSizeInBits: i32 = 64;
const kDRegSizeInBits: i32 = 64;
const kNumberOfRegisters: usize = 32;
const kNumberOfVRegisters: usize = 32;

// AAPCS64 callee-saved registers.
#[allow(non_upper_case_globals)]
static kCalleeSaved: CPURegList = CPURegList::get_callee_saved(kXRegSizeInBits);
#[allow(non_upper_case_globals)]
static kCalleeSavedV: CPURegList = CPURegList::get_callee_saved_v(kDRegSizeInBits);

// AAPCS64 caller-saved registers. Note that this includes lr.
#[allow(non_upper_case_globals)]
static kCallerSaved: CPURegList = CPURegList::get_caller_saved(kXRegSizeInBits);
#[allow(non_upper_case_globals)]
static kCallerSavedV: CPURegList = CPURegList::get_caller_saved_v(kDRegSizeInBits);

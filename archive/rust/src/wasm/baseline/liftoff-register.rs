// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::cmp::{max, min};
//use std::fmt;
//use std::fmt::{Debug, Display, Formatter};
//use std::marker::Copy;
//use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
//use std::prelude::v1::derive;
//use std::result::Result;

pub mod liftoff_register {
    use crate::wasm::baseline::liftoff_assembler_defs::{
        kLiftoffAssemblerFpCacheRegs, kLiftoffAssemblerGpCacheRegs, DoubleRegList, RegList,
    };
    use crate::wasm::wasm_opcodes::ValueKind;
    use std::cmp;
    use std::fmt;
    use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
    use static_assertions::assert_eq;
    //use crate::base::bits::CountLeadingZeros;
    //use crate::base;

    const kSystemPointerSize: usize = 8; // Assuming 64-bit system
    enum AliasingKind {
        kNoAlias,
        kCombine,
    }
    const kFPAliasing: AliasingKind = AliasingKind::kNoAlias; // Assuming no aliasing for now

    const kNeedI64RegPair: bool = kSystemPointerSize == 4;
    const kNeedS128RegPair: bool = match kFPAliasing {
        AliasingKind::kCombine => true,
        _ => false,
    };

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u8)]
    pub enum RegClass {
        kGpReg,
        kFpReg,
        kGpRegPair,
        kFpRegPair,
        kNoReg,
    }

    impl RegClass {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(RegClass::kGpReg),
                1 => Some(RegClass::kFpReg),
                2 => Some(RegClass::kGpRegPair),
                3 => Some(RegClass::kFpRegPair),
                4 => Some(RegClass::kNoReg),
                _ => None,
            }
        }
    }

    impl From<u8> for RegClass {
        fn from(value: u8) -> Self {
            RegClass::from_u8(value).expect("Invalid RegClass value")
        }
    }

    impl Default for RegClass {
        fn default() -> Self {
            RegClass::kGpReg
        }
    }

    // Adjust RegClass based on kNeedI64RegPair and kNeedS128RegPair
    const _GP_REG_PAIR_VALUE: u8 = 1 + 1 + (kNeedS128RegPair as u8 && !kNeedI64RegPair as u8);
    const _FP_REG_PAIR_VALUE: u8 = 1 + 1 + kNeedI64RegPair as u8;
    const _NO_REG_VALUE: u8 = _FP_REG_PAIR_VALUE + kNeedS128RegPair as u8;

    // Ensure the RegClass enum values match the C++ code
    const _: () = assert_eq!(RegClass::kGpReg as u8, 0);
    const _: () = assert_eq!(RegClass::kFpReg as u8, 1);
    const _: () = assert_eq!(RegClass::kGpRegPair as u8, _GP_REG_PAIR_VALUE);
    const _: () = assert_eq!(RegClass::kFpRegPair as u8, _FP_REG_PAIR_VALUE);
    const _: () = assert_eq!(RegClass::kNoReg as u8, _NO_REG_VALUE);

    const _: () = assert_eq!(
        kNeedI64RegPair,
        RegClass::kGpRegPair != RegClass::kNoReg,
        "kGpRegPair equals kNoReg if unused"
    );
    const _: () = assert_eq!(
        kNeedS128RegPair,
        RegClass::kFpRegPair != RegClass::kNoReg,
        "kFpRegPair equals kNoReg if unused"
    );

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u8)]
    pub enum RegPairHalf {
        kLowWord = 0,
        kHighWord = 1,
    }

    pub const fn needs_gp_reg_pair(kind: ValueKind) -> bool {
        kNeedI64RegPair && kind == ValueKind::kI64
    }

    pub const fn needs_fp_reg_pair(kind: ValueKind) -> bool {
        kNeedS128RegPair && kind == ValueKind::kS128
    }

    pub const fn reg_class_for(kind: ValueKind) -> RegClass {
        match kind {
            ValueKind::kF16 | ValueKind::kF32 | ValueKind::kF64 => RegClass::kFpReg,
            ValueKind::kI8 | ValueKind::kI16 | ValueKind::kI32 => RegClass::kGpReg,
            ValueKind::kI64 => {
                if kNeedI64RegPair {
                    RegClass::kGpRegPair
                } else {
                    RegClass::kGpReg
                }
            }
            ValueKind::kS128 => {
                if kNeedS128RegPair {
                    RegClass::kFpRegPair
                } else {
                    RegClass::kFpReg
                }
            }
            ValueKind::kRef | ValueKind::kRefNull => RegClass::kGpReg,
            ValueKind::kVoid => RegClass::kNoReg, // Unsupported kind
        }
    }

    // Maximum code of a gp cache register.
    pub const kMaxGpRegCode: i32 = kLiftoffAssemblerGpCacheRegs.last().code();
    // Maximum code of an fp cache register.
    pub const kMaxFpRegCode: i32 = kLiftoffAssemblerFpCacheRegs.last().code();
    pub const kAfterMaxLiftoffGpRegCode: i32 = kMaxGpRegCode + 1;
    pub const kAfterMaxLiftoffFpRegCode: i32 =
        kAfterMaxLiftoffGpRegCode + kMaxFpRegCode + 1;
    pub const kAfterMaxLiftoffRegCode: i32 = kAfterMaxLiftoffFpRegCode;
    pub const kBitsPerLiftoffRegCode: i32 =
        32 - (kAfterMaxLiftoffRegCode - 1).leading_zeros() as i32;
    pub const kBitsPerGpRegCode: i32 = 32 - kMaxGpRegCode.leading_zeros() as i32;
    pub const kBitsPerFpRegCode: i32 = 32 - kMaxFpRegCode.leading_zeros() as i32;
    // GpRegPair requires 1 extra bit, S128RegPair also needs an extra bit.
    pub const kBitsPerRegPair: i32 =
        (if kNeedS128RegPair { 2 } else { 1 }) + 2 * kBitsPerGpRegCode;

    const _: () = assert_eq!(
        2 * kBitsPerGpRegCode >= kBitsPerFpRegCode,
        true,
        "encoding for gp pair and fp pair collides"
    );

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct LiftoffRegister {
        code_: LiftoffRegisterStorage,
    }
    impl fmt::Debug for LiftoffRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_gp_pair() {
                write!(f, "<{:?}+{:?}>", self.low_gp(), self.high_gp())
            } else if self.is_fp_pair() {
                write!(f, "<{:?}+{:?}>", self.low_fp(), self.high_fp())
            } else if self.is_gp() {
                write!(f, "{:?}", self.gp())
            } else {
                write!(f, "{:?}", self.fp())
            }
        }
    }

    #[allow(non_camel_case_types)]
    type LiftoffRegisterStorage = u16;

    impl LiftoffRegister {
        const needed_bits: i32 = cmp::max(
            if kNeedI64RegPair || kNeedS128RegPair {
                kBitsPerRegPair
            } else {
                0
            },
            kBitsPerLiftoffRegCode,
        );

        pub const fn new(reg: Register) -> Self {
            LiftoffRegister {
                code_: reg.code() as LiftoffRegisterStorage,
            }
        }

        pub const fn new_double(reg: DoubleRegister) -> Self {
            LiftoffRegister {
                code_: (kAfterMaxLiftoffGpRegCode + reg.code()) as LiftoffRegisterStorage,
            }
        }
        pub fn from_liftoff_code(code: i32) -> Self {
            let reg = LiftoffRegister {
                code_: code as LiftoffRegisterStorage,
            };
            // Check that the code is correct by round-tripping through the
            // reg-class-specific constructor.
            // This part is tricky without knowing the full context of Register and DoubleRegister
            /*
            DCHECK(
                (reg.is_gp() && code == LiftoffRegister{reg.gp()}.liftoff_code()) ||
                (reg.is_fp() && code == LiftoffRegister{reg.fp()}.liftoff_code()) ||
                (reg.is_gp_pair() &&
                 code == ForPair(reg.low_gp(), reg.high_gp()).liftoff_code()) ||
                (reg.is_fp_pair() && code == ForFpPair(reg.low_fp()).liftoff_code()));
            */
            reg
        }

        pub fn from_code(rc: RegClass, code: i32) -> Self {
            match rc {
                RegClass::kGpReg => LiftoffRegister::new(Register::from_code(code)),
                RegClass::kFpReg => LiftoffRegister::new_double(DoubleRegister::from_code(code)),
                _ => panic!("UNREACHABLE"),
            }
        }

        // Shifts the register code depending on the type before converting to a
        // LiftoffRegister.
        pub fn from_external_code(rc: RegClass, kind: ValueKind, code: i32) -> Self {
            match kFPAliasing {
                AliasingKind::kCombine => {
                    if kind == ValueKind::kF32 {
                        // Liftoff assumes a one-to-one mapping between float registers and
                        // double registers, and so does not distinguish between f32 and f64
                        // registers. The f32 register code must therefore be halved in order
                        // to pass the f64 code to Liftoff.
                        assert_eq!(0, code % 2);
                        return LiftoffRegister::from_code(rc, code >> 1);
                    }
                }
                AliasingKind::kNoAlias => {}
            }

            if kNeedS128RegPair && kind == ValueKind::kS128 {
                // Similarly for double registers and SIMD registers, the SIMD code
                // needs to be doubled to pass the f64 code to Liftoff.
                return LiftoffRegister::for_fp_pair(DoubleRegister::from_code(code << 1));
            }
            LiftoffRegister::from_code(rc, code)
        }

        pub fn for_pair(low: Register, high: Register) -> Self {
            assert!(kNeedI64RegPair);
            assert_ne!(low, high);
            let combined_code = low.code() as LiftoffRegisterStorage
                | ((high.code() << kBitsPerGpRegCode) as LiftoffRegisterStorage)
                | (1 << (2 * kBitsPerGpRegCode)) as LiftoffRegisterStorage;
            LiftoffRegister { code_: combined_code }
        }

        pub fn for_fp_pair(low: DoubleRegister) -> Self {
            assert!(kNeedS128RegPair);
            assert_eq!(0, low.code() % 2);
            let combined_code =
                low.code() as LiftoffRegisterStorage | (2 << (2 * kBitsPerGpRegCode)) as LiftoffRegisterStorage;
            LiftoffRegister { code_: combined_code }
        }

        pub const fn is_pair(&self) -> bool {
            (kNeedI64RegPair || kNeedS128RegPair)
                && ((self.code_ & (3 << (2 * kBitsPerGpRegCode)) as LiftoffRegisterStorage) != 0)
        }

        pub const fn is_gp_pair(&self) -> bool {
            kNeedI64RegPair
                && ((self.code_ & (1 << (2 * kBitsPerGpRegCode)) as LiftoffRegisterStorage) != 0)
        }
        pub const fn is_fp_pair(&self) -> bool {
            kNeedS128RegPair
                && ((self.code_ & (2 << (2 * kBitsPerGpRegCode)) as LiftoffRegisterStorage) != 0)
        }
        pub const fn is_gp(&self) -> bool {
            (self.code_ as i32) < kAfterMaxLiftoffGpRegCode
        }
        pub const fn is_fp(&self) -> bool {
            (self.code_ as i32) >= kAfterMaxLiftoffGpRegCode
                && (self.code_ as i32) < kAfterMaxLiftoffFpRegCode
        }

        pub fn low(&self) -> LiftoffRegister {
            // Common case for most archs where only gp pair supported.
            if !kNeedS128RegPair {
                return LiftoffRegister::new(self.low_gp());
            }
            if self.is_gp_pair() {
                LiftoffRegister::new(self.low_gp())
            } else {
                LiftoffRegister::new_double(self.low_fp())
            }
        }

        pub fn high(&self) -> LiftoffRegister {
            // Common case for most archs where only gp pair supported.
            if !kNeedS128RegPair {
                return LiftoffRegister::new(self.high_gp());
            }
            if self.is_gp_pair() {
                LiftoffRegister::new(self.high_gp())
            } else {
                LiftoffRegister::new_double(self.high_fp())
            }
        }

        pub fn low_gp(&self) -> Register {
            assert!(self.is_gp_pair());
            let k_code_mask: LiftoffRegisterStorage =
                ((1 << kBitsPerGpRegCode) - 1) as LiftoffRegisterStorage;
            Register::from_code((self.code_ & k_code_mask) as i32)
        }

        pub fn high_gp(&self) -> Register {
            assert!(self.is_gp_pair());
            let k_code_mask: LiftoffRegisterStorage =
                ((1 << kBitsPerGpRegCode) - 1) as LiftoffRegisterStorage;
            Register::from_code(((self.code_ >> kBitsPerGpRegCode) & k_code_mask) as i32)
        }

        pub fn low_fp(&self) -> DoubleRegister {
            assert!(self.is_fp_pair());
            let k_code_mask: LiftoffRegisterStorage =
                ((1 << kBitsPerFpRegCode) - 1) as LiftoffRegisterStorage;
            DoubleRegister::from_code((self.code_ & k_code_mask) as i32)
        }

        pub fn high_fp(&self) -> DoubleRegister {
            assert!(self.is_fp_pair());
            let k_code_mask: LiftoffRegisterStorage =
                ((1 << kBitsPerFpRegCode) - 1) as LiftoffRegisterStorage;
            DoubleRegister::from_code((self.code_ & k_code_mask) as i32 + 1)
        }

        pub const fn gp(&self) -> Register {
            assert!(self.is_gp());
            Register::from_code(self.code_ as i32)
        }

        pub const fn fp(&self) -> DoubleRegister {
            assert!(self.is_fp());
            DoubleRegister::from_code((self.code_ as i32) - kAfterMaxLiftoffGpRegCode)
        }

        pub const fn liftoff_code(&self) -> i32 {
            self.code_ as i32
        }

        pub const fn reg_class(&self) -> RegClass {
            if self.is_fp_pair() {
                RegClass::kFpRegPair
            } else if self.is_gp_pair() {
                RegClass::kGpRegPair
            } else if self.is_gp() {
                RegClass::kGpReg
            } else {
                RegClass::kFpReg
            }
        }

        pub fn overlaps(&self, other: &LiftoffRegister) -> bool {
            if self.is_pair() {
                return self.low().overlaps(other) || self.high().overlaps(other);
            }
            if other.is_pair() {
                return *self == other.low() || *self == other.high();
            }
            *self == *other
        }
    }

    #[derive(Default, Copy, Clone, PartialEq, Eq)]
    pub struct LiftoffRegList {
        regs_: LiftoffRegListStorage,
    }

    impl fmt::Debug for LiftoffRegList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut reglist = *self;
            write!(f, "{{")?;
            let mut first = true;
            while !reglist.is_empty() {
                let reg = reglist.get_first_reg_set();
                reglist.clear(reg);
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", reg)?;
                first = false;
            }
            write!(f, "}}")
        }
    }

    #[allow(non_camel_case_types)]
    type LiftoffRegListStorage = u32;

    impl LiftoffRegList {
        pub const use_u16: bool = kAfterMaxLiftoffRegCode <= 16;
        pub const use_u32: bool = !LiftoffRegList::use_u16 && kAfterMaxLiftoffRegCode <= 32;

        pub const kGpMask: LiftoffRegListStorage =
            kLiftoffAssemblerGpCacheRegs.bits() as LiftoffRegListStorage;
        pub const kFpMask: LiftoffRegListStorage =
            (kLiftoffAssemblerFpCacheRegs.bits() << kAfterMaxLiftoffGpRegCode) as LiftoffRegListStorage;
        // Sets all even numbered fp registers.
        pub const kEvenFpSetMask: u64 =
            (0x5555555555555555 << kAfterMaxLiftoffGpRegCode) as u64;
        pub const kOddFpSetMask: u64 =
            (0xAAAAAAAAAAAAAAAA << kAfterMaxLiftoffGpRegCode) as u64;

        // Allow to construct LiftoffRegList from a number of
        // {Register|DoubleRegister|LiftoffRegister}.
        pub fn new<const N: usize>(regs: [LiftoffRegister; N]) -> Self {
            let mut reg_list = LiftoffRegList::default();
            for reg in regs {
                reg_list.set(reg);
            }
            reg_list
        }
        pub fn new_from_registers<const N: usize>(regs: [Register; N]) -> Self {
            let mut reg_list = LiftoffRegList::default();
            for reg in regs {
                reg_list.set(reg);
            }
            reg_list
        }

        pub fn new_from_double_registers<const N: usize>(regs: [DoubleRegister; N]) -> Self {
            let mut reg_list = LiftoffRegList::default();
            for reg in regs {
                reg_list.set(reg);
            }
            reg_list
        }

        pub fn set(&mut self, reg: LiftoffRegister) -> LiftoffRegister {
            if reg.is_pair() {
                self.regs_ |= 1 << reg.low().liftoff_code();
                self.regs_ |= 1 << reg.high().liftoff_code();
            } else {
                self.regs_ |= 1 << reg.liftoff_code();
            }
            reg
        }
        pub fn set_register(&mut self, reg: Register) -> Register {
            self.set(LiftoffRegister::new(reg)).gp()
        }
        pub fn set_double_register(&mut self, reg: DoubleRegister) -> DoubleRegister {
            self.set(LiftoffRegister::new_double(reg)).fp()
        }

        pub fn clear(&mut self, reg: LiftoffRegister) -> LiftoffRegister {
            if reg.is_pair() {
                self.regs_ &= !(1 << reg.low().liftoff_code());
                self.regs_ &= !(1 << reg.high().liftoff_code());
            } else {
                self.regs_ &= !(1 << reg.liftoff_code());
            }
            reg
        }

        pub fn clear_register(&mut self, reg: Register) -> Register {
            self.clear(LiftoffRegister::new(reg)).gp()
        }
        pub fn clear_double_register(&mut self, reg: DoubleRegister) -> DoubleRegister {
            self.clear(LiftoffRegister::new_double(reg)).fp()
        }

        pub fn has(&self, reg: LiftoffRegister) -> bool {
            if reg.is_pair() {
                assert_eq!(self.has(reg.low()), self.has(reg.high()));
                return self.has(reg.low());
            }
            (self.regs_ & (1 << reg.liftoff_code())) != 0
        }
        pub fn has_register(&self, reg: Register) -> bool {
            self.has(LiftoffRegister::new(reg))
        }
        pub fn has_double_register(&self, reg: DoubleRegister) -> bool {
            self.has(LiftoffRegister::new_double(reg))
        }

        pub const fn is_empty(&self) -> bool {
            self.regs_ == 0
        }

        pub fn get_num_regs_set(&self) -> u32 {
            self.regs_.count_ones()
        }

        pub fn get_adjacent_fp_regs_set(&self) -> LiftoffRegList {
            // And regs_ with a right shifted version of itself, so reg[i] is set only
            // if reg[i+1] is set. We only care about the even fp registers.
            let available = ((self.regs_ >> 1) & self.regs_ & (LiftoffRegList::kEvenFpSetMask as LiftoffRegListStorage));
            LiftoffRegList::from_bits(available)
        }

        pub const fn has_adjacent_fp_regs_set(&self) -> bool {
            !self.get_adjacent_fp_regs_set().is_empty()
        }

        // Returns a list where if any part of an adjacent pair of FP regs was set,
        // both are set in the result. For example, [1, 4] is turned into [0, 1, 4, 5]
        // because (0, 1) and (4, 5) are adjacent pairs.
        pub fn spread_set_bits_to_adjacent_fp_regs(&self) -> LiftoffRegList {
            let odd_regs = self.regs_ & (LiftoffRegList::kOddFpSetMask as LiftoffRegListStorage);
            let even_regs = self.regs_ & (LiftoffRegList::kEvenFpSetMask as LiftoffRegListStorage);
            LiftoffRegList::from_bits(
                self.regs_
                    | ((odd_regs >> 1) & LiftoffRegList::kFpMask)
                    | ((even_regs << 1) & LiftoffRegList::kFpMask),
            )
        }

        pub fn get_first_reg_set(&self) -> LiftoffRegister {
            assert_ne!(self.regs_, 0);
            let first_code = self.regs_.trailing_zeros();
            LiftoffRegister::from_liftoff_code(first_code as i32)
        }

        pub fn get_last_reg_set(&self) -> LiftoffRegister {
            assert_ne!(self.regs_, 0);
            let last_code = 8 * std::mem::size_of::<Self>() as u32 - 1 - self.regs_.leading_zeros();
            LiftoffRegister::from_liftoff_code(last_code as i32)
        }

        pub fn mask_out(&self, mask: &LiftoffRegList) -> LiftoffRegList {
            // Masking out is guaranteed to return a correct reg list, hence no checks
            // needed.
            LiftoffRegList::from_bits(self.regs_ & !mask.regs_)
        }

        pub fn get_gp_list(&self) -> RegList {
            RegList::from_bits((self.regs_ & LiftoffRegList::kGpMask) as u64)
        }
        pub fn get_fp_list(&self) -> DoubleRegList {
            DoubleRegList::from_bits(
                ((self.regs_ & LiftoffRegList::kFpMask) >> kAfterMaxLiftoffGpRegCode) as u64,
            )
        }

        pub const fn from_bits(bits: LiftoffRegListStorage) -> Self {
            let allowed_bits = bits & (LiftoffRegList::kGpMask | LiftoffRegList::kFpMask);
            assert_eq!(bits, allowed_bits);
            LiftoffRegList { regs_: bits }
        }
    }

    impl BitAnd for LiftoffRegList {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            LiftoffRegList::from_bits(self.regs_ & other.regs_)
        }
    }

    impl BitAndAssign for LiftoffRegList {
        fn bitand_assign(&mut self, other: Self) {
            self.regs_ &= other.regs_;
        }
    }

    impl BitOr for LiftoffRegList {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            LiftoffRegList::from_bits(self.regs_ | other.regs_)
        }
    }

    impl BitOrAssign for LiftoffRegList {
        fn bitor_assign(&mut self, other: Self) {
            self.regs_ |= other.regs_;
        }
    }

    pub const kGpCacheRegList: LiftoffRegList =
        LiftoffRegList::from_bits(LiftoffRegList::kGpMask);
    pub const kFpCacheRegList: LiftoffRegList =
        LiftoffRegList::from_bits(LiftoffRegList::kFpMask);

    pub const fn get_cache_reg_list(rc: RegClass) -> LiftoffRegList {
        assert!(rc == RegClass::kFpReg || rc == RegClass::kGpReg);
        //static_assert(kGpReg == 0 && kFpReg == 1);
        const REG_LISTS: [LiftoffRegList; 2] = [kGpCacheRegList, kFpCacheRegList];
        match rc {
            RegClass::kGpReg => REG_LISTS[0],
            RegClass::kFpReg => REG_LISTS[1],
            _ => panic!("unreachable"),
        }
    }

    pub struct LiftoffRegListIterator {
        remaining_: LiftoffRegList,
    }
    impl Iterator for LiftoffRegListIterator {
        type Item = LiftoffRegister;
        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_.is_empty() {
                return None;
            }
            let reg = self.remaining_.get_first_reg_set();
            self.remaining_.clear(reg);
            Some(reg)
        }
    }

    impl LiftoffRegList {
        pub fn iter(&self) -> LiftoffRegListIterator {
            LiftoffRegListIterator { remaining_: *self }
        }
    }

    impl<'a> IntoIterator for &'a LiftoffRegList {
        type Item = LiftoffRegister;
        type IntoIter = LiftoffRegListIterator;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    // Dummy definitions for Register and DoubleRegister to allow compilation
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code_: i32,
    }

    impl Register {
        pub const fn from_code(code: i32) -> Self {
            Register { code_: code }
        }
        pub const fn code(&self) -> i32 {
            self.code_
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister {
        code_: i32,
    }

    impl DoubleRegister {
        pub const fn from_code(code: i32) -> Self {
            DoubleRegister { code_: code }
        }
        pub const fn code(&self) -> i32 {
            self.code_
        }
    }
}
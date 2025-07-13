// Converted from V8 C++ source files:
// Header: utils.h
// Implementation: utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn CountLeadingZeros(x: usize) -> u32 {
            x.leading_zeros()
        }
    }
    pub mod hashing {
        pub struct hash<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> hash<T> {
            pub fn new() -> Self {
                hash {
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn call(&self, value: T) -> usize {
                let mut s = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&value, &mut s);
                use std::hash::Hasher;
                s.finish() as usize
            }
        }

        impl<T> Default for hash<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    }
    pub mod platform {
        pub mod platform {
            use std::process;

            pub fn GetCurrentProcessId() -> u32 {
                process::id()
            }
        }
        pub mod wrappers {
            use std::fs::File;
            use std::io::{self, Write};

            pub fn FOpen(filename: &str, mode: &str) -> Option<File> {
                File::open(filename).ok()
            }

            pub fn Fclose(file: File) {
                drop(file);
            }

            pub fn PrintError(format: &str, args: ...) {
                use std::ffi::CString;
                use std::os::raw::c_char;

                unsafe {
                    let c_format = CString::new(format).unwrap();
                    let mut arguments: va_list::VaListImpl;
                    va_start!(arguments, format);
                    let result = vprintf(c_format.as_ptr() as *const c_char, arguments);
                    va_end!(arguments);

                    if result < 0 {
                        eprintln!("Error during vprintf: {}", io::Error::last_os_error());
                    }
                }

                extern "C" {
                    use std::arch::wasm::VaList;
                    #[link_name = "vfprintf"]
                    fn vprintf(format: *const c_char, arg: VaList) -> i32;
                }
            }
        }
    }
}
pub mod common {
    pub mod globals {
        pub const kBitsPerByte: usize = 8;
        pub const kMaxUInt64: u64 = u64::MAX;
    }
}

use std::any::Any;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};
use std::mem;
use std::os::raw::c_char;
use std::rc::Rc;
use std::{f64, i32, u32, usize};

#[macro_use]
extern crate std;

pub mod v8 {
    pub mod internal {
        use super::*;

        pub fn ArithmeticShiftRight<T>(x: T, shift: i32) -> T
        where
            T: std::ops::Shr<i32, Output = T>
                + std::ops::BitOr<T, Output = T>
                + std::ops::Not<Output = T>
                + Copy
                + std::convert::TryFrom<i32>,
            <T as std::convert::TryFrom<i32>>::Error: std::fmt::Debug,
        {
            assert!(shift >= 0);
            let shift_usize = shift as usize;

            if x < T::try_from(0).unwrap() {
                let unsigned_t = mem::size_of::<T>() * 8;
                let mask = !(T::try_from(-1).unwrap() >> shift);
                (x >> shift) | mask
            } else {
                x >> shift
            }
        }

        pub fn JSMax<T>(x: T, y: T) -> T
        where
            T: std::cmp::PartialOrd + std::marker::Copy,
        {
            if is_nan(&x) {
                return x;
            }
            if is_nan(&y) {
                return y;
            }
            if signbit(&x) < signbit(&y) {
                return x;
            }
            if x > y {
                x
            } else {
                y
            }
        }

        pub fn JSMin<T>(x: T, y: T) -> T
        where
            T: std::cmp::PartialOrd + std::marker::Copy,
        {
            if is_nan(&x) {
                return x;
            }
            if is_nan(&y) {
                return y;
            }
            if signbit(&x) < signbit(&y) {
                return y;
            }
            if x > y {
                y
            } else {
                x
            }
        }

        pub fn Abs<T>(a: T) -> <T as AbsHelper>::Unsigned
        where
            T: std::cmp::PartialOrd + std::marker::Copy + AbsHelper,
        {
            <T as AbsHelper>::abs(a)
        }

        trait AbsHelper {
            type Unsigned;
            fn abs(self) -> Self::Unsigned;
        }

        impl AbsHelper for i32 {
            type Unsigned = u32;
            fn abs(self) -> Self::Unsigned {
                let x = self as u32;
                let y = (self >> 31) as u32;
                (x ^ y).wrapping_sub(y)
            }
        }

        impl AbsHelper for i64 {
            type Unsigned = u64;
            fn abs(self) -> Self::Unsigned {
                let x = self as u64;
                let y = (self >> 63) as u64;
                (x ^ y).wrapping_sub(y)
            }
        }

        pub fn Modulo(x: f64, y: f64) -> f64 {
            #[cfg(target_os = "windows")]
            {
                // Workaround MS fmod bugs. ECMA-262 says:
                // dividend is finite and divisor is an infinity => result equals dividend
                // dividend is a zero and divisor is nonzero finite => result equals dividend
                if (!(x.is_finite() && (!y.is_finite() && !y.is_nan()))
                    && !(x == 0.0 && (y != 0.0 && y.is_finite())))
                {
                    let mut result = x % y;
                    if x < 0.0 && result == 0.0 {
                        result = -0.0;
                    }
                    result
                } else {
                    x
                }
            }
            #[cfg(target_os = "aix")]
            {
                unsafe {
                    feclearexcept(FE_ALL_EXCEPT);
                    let result = x % y;
                    let exception = fetestexcept(FE_UNDERFLOW);
                    if exception != 0 {
                        x
                    } else {
                        result
                    }
                }
            }
            #[cfg(not(any(target_os = "windows", target_os = "aix")))]
            {
                x % y
            }
        }

        #[cfg(target_os = "aix")]
        extern "C" {
            fn feclearexcept(excepts: i32) -> i32;
            fn fetestexcept(excepts: i32) -> i32;
        }

        #[cfg(target_os = "aix")]
        const FE_ALL_EXCEPT: i32 = 127;
        #[cfg(target_os = "aix")]
        const FE_UNDERFLOW: i32 = 4;

        pub fn SaturateAdd<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<T, Output = T>
                + std::cmp::PartialOrd
                + std::marker::Copy
                + std::fmt::Debug
                + std::ops::Sub<T, Output = T>
                + std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>
                + std::convert::TryFrom<i32>,
            <T as std::convert::TryFrom<i32>>::Error: std::fmt::Debug,
            T: std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>,
            T: std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>,
            T: std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>,
            T: std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>,
            T: std::ops::Sub<<T as std::ops::Add<T>>::Output, Output = T>,
        {
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
                let a = a as i32;
                let b = b as i32;
                if a > 0 && b > 0 {
                    if a > i32::MAX - b {
                        return i32::MAX as T;
                    }
                } else if a < 0 && b < 0 {
                    if a < i32::MIN - b {
                        return i32::MIN as T;
                    }
                }
                (a + b) as T
            } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
                let a = a as u32;
                let b = b as u32;
                if a > u32::MAX - b {
                    return u32::MAX as T;
                }
                (a + b) as T
            } else {
                panic!("Unsupported type for SaturateAdd");
            }
        }

        pub fn SaturateSub<T>(a: T, b: T) -> T
        where
            T: std::ops::Sub<T, Output = T>
                + std::cmp::PartialOrd
                + std::marker::Copy
                + std::fmt::Debug
                + std::ops::Add<T, Output = T>,
            T: std::convert::TryFrom<i32>,
            <T as std::convert::TryFrom<i32>>::Error: std::fmt::Debug,
            T: std::ops::Add<<T as std::ops::Sub<T>>::Output, Output = T>,
            T: std::ops::Add<<T as std::ops::Sub<T>>::Output, Output = T>,
        {
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
                let a = a as i32;
                let b = b as i32;
                if a >= 0 && b < 0 {
                    if a > i32::MAX + b {
                        return i32::MAX as T;
                    }
                } else if a < 0 && b > 0 {
                    if a < i32::MIN + b {
                        return i32::MIN as T;
                    }
                }
                (a - b) as T
            } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
                let a = a as u32;
                let b = b as u32;
                if a < b {
                    return 0 as T;
                }
                (a - b) as T
            } else {
                panic!("Unsupported type for SaturateSub");
            }
        }

        pub fn SaturateRoundingQMul(a: i32, b: i32) -> i32 {
            let size_in_bits = mem::size_of::<i32>() * 8;
            let round_const: i64 = 1 << (size_in_bits - 2);
            let mut product: i64 = (a as i64) * (b as i64);
            product += round_const;
            product >>= (size_in_bits - 1);
            if product > i32::MAX as i64 {
                return i32::MAX;
            }
            if product < i32::MIN as i64 {
                return i32::MIN;
            }
            product as i32
        }

        pub fn MultiplyLong<Wide, Narrow>(a: Narrow, b: Narrow) -> Wide
        where
            Wide: std::ops::Mul<Wide, Output = Wide>
                + std::convert::From<Narrow>
                + std::marker::Copy,
            Narrow: std::marker::Copy,
        {
            let a_wide: Wide = Wide::from(a);
            let b_wide: Wide = Wide::from(b);
            a_wide * b_wide
        }

        pub fn AddLong<Wide, Narrow>(a: Narrow, b: Narrow) -> Wide
        where
            Wide: std::ops::Add<Wide, Output = Wide>
                + std::convert::From<Narrow>
                + std::marker::Copy,
            Narrow: std::marker::Copy,
        {
            let a_wide: Wide = Wide::from(a);
            let b_wide: Wide = Wide::from(b);
            a_wide + b_wide
        }

        pub fn RoundingAverageUnsigned<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<T, Output = T> + std::ops::Shr<usize, Output = T> + Copy,
            u64: From<T>,
        {
            let a_u64 = u64::from(a);
            let b_u64 = u64::from(b);
            ((a_u64 + b_u64 + 1) >> 1) as T
        }

        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub struct FeedbackSlot {
            id_: i32,
        }

        impl FeedbackSlot {
            pub fn new() -> Self {
                FeedbackSlot {
                    id_: Self::kInvalidSlot,
                }
            }
            pub fn with_id(id: i32) -> Self {
                FeedbackSlot { id_: id }
            }

            pub fn ToInt(&self) -> i32 {
                self.id_
            }

            pub fn Invalid() -> Self {
                FeedbackSlot {
                    id_: Self::kInvalidSlot,
                }
            }

            pub fn IsInvalid(&self) -> bool {
                self.id_ == Self::kInvalidSlot
            }

            pub fn WithOffset(&self, offset: i32) -> Self {
                FeedbackSlot {
                    id_: self.id_ + offset,
                }
            }

            const kInvalidSlot: i32 = -1;
        }

        impl fmt::Display for FeedbackSlot {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "#{}", self.id_)
            }
        }

        impl Hash for FeedbackSlot {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.id_.hash(state);
            }
        }

        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub struct BytecodeOffset {
            id_: i32,
        }

        impl BytecodeOffset {
            pub const fn new(id: i32) -> Self {
                BytecodeOffset { id_: id }
            }
            pub const fn ToInt(&self) -> i32 {
                self.id_
            }
            pub const fn None() -> Self {
                BytecodeOffset {
                    id_: Self::kNoneId,
                }
            }
            pub const fn ConstructStubCreate() -> Self {
                BytecodeOffset { id_: 1 }
            }
            pub const fn ConstructStubInvoke() -> Self {
                BytecodeOffset { id_: 2 }
            }
            pub const fn IsNone(&self) -> bool {
                self.id_ == Self::kNoneId
            }

            const kNoneId: i32 = -1;
        }

        impl fmt::Display for BytecodeOffset {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.id_)
            }
        }
        impl Hash for BytecodeOffset {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.id_.hash(state);
            }
        }

        pub fn TenToThe(exponent: u32) -> u64 {
            assert!(exponent <= 19);
            assert!(exponent >= 0);
            let mut answer: u64 = 1;
            for _i in 0..exponent {
                answer *= 10;
            }
            answer
        }

        pub fn unsigned_bitextract_32(msb: i32, lsb: i32, x: u32) -> u32 {
            (x >> lsb) & ((1 << (1 + msb - lsb)) - 1)
        }

        pub fn unsigned_bitextract_64(msb: i32, lsb: i32, x: u64) -> u64 {
            (x >> lsb) & (((1 as u64) << (1 + msb - lsb)) - 1)
        }

        pub fn signed_bitextract_32(msb: i32, lsb: i32, x: u32) -> i32 {
            (x << (31 - msb)) as i32 >> (lsb + 31 - msb)
        }

        pub const fn is_intn(x: i64, n: u32) -> bool {
            assert!((0 < n) && (n < 64));
            let limit: i64 = 1 << (n - 1);
            (-limit <= x) && (x < limit)
        }

        pub const fn is_uintn(x: i64, n: u32) -> bool {
            assert!((0 < n) && (n < (mem::size_of::<i64>() * 8) as u32));
            !(x >> n != 0)
        }

        pub fn truncate_to_intn<T>(x: T, n: u32) -> T
        where
            T: std::ops::BitAnd<Output = T>
                + std::convert::TryFrom<u64>
                + std::marker::Copy,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            assert!((0 < n) && (n < (mem::size_of::<T>() * 8) as u32));
            let mask = (1u64 << n) - 1;
            (x & T::try_from(mask).unwrap())
        }

        macro_rules! declare_is_int_n {
            ($n:expr) => {
                paste::item! {
                    pub const fn [<is_int $n>](x: i64) -> bool {
                        is_intn(x, $n)
                    }
                }
            };
        }

        macro_rules! declare_is_uint_n {
            ($n:expr) => {
                paste::item! {
                    pub const fn [<is_uint $n>]<T>(x: T) -> bool
                    where T: Into<i64> + Copy
                    {
                        is_uintn(x.into(), $n)
                    }
                }
            };
        }

        macro_rules! declare_truncate_to_int_n {
            ($n:expr) => {
                paste::item! {
                    pub fn [<truncate_to_int $n>]<T>(x: T) -> T
                    where
                        T: std::ops::BitAnd<Output = T>
                            + std::convert::TryFrom<u64>
                            + std::marker::Copy,
                        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
                    {
                        truncate_to_intn(x, $n)
                    }
                }
            };
        }

        macro_rules! declare_checked_truncate_to_int_n {
            ($n:expr) => {
                paste::item! {
                    pub fn [<checked_truncate_to_int $n>]<T>(x: T) -> T
                    where
                        T: std::ops::BitAnd<Output = T>
                            + std::convert::TryFrom<u64>
                            + std::marker::Copy
                            + std::fmt::Debug
                            + std::convert::TryFrom<i64>,
                         <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
                        <T as std::convert::TryFrom<i64>>::Error: std::fmt::Debug,
                    {
                        assert!([<is_int $n>](x.into()));
                        truncate_to_intn(x, $n)
                    }
                }
            };
        }

        macro_rules! int_1_to_63_list {
            ($macro:ident) => {
                $macro!(1);
                $macro!(2);
                $macro!(3);
                $macro!(4);
                $macro!(5);
                $macro!(6);
                $macro!(7);
                $macro!(8);
                $macro!(9);
                $macro!(10);
                $macro!(11);
                $macro!(12);
                $macro!(13);
                $macro!(14);
                $macro!(15);
                $macro!(16);
                $macro!(17);
                $macro!(18);
                $macro!(19);
                $macro!(20);
                $macro!(21);
                $macro!(22);
                $macro!(23);
                $macro!(24);
                $macro!(25);
                $macro!(26);
                $macro!(27);
                $macro!(28);
                $macro!(29);
                $macro!(30);
                $macro!(31);
                $macro!(32);
                $macro!(33);
                $macro!(34);
                $macro!(35);
                $macro!(36);
                $macro!(37);
                $macro!(38);
                $macro!(39);
                $macro!(40);
                $macro!(41);
                $macro!(42);
                $macro!(43);
                $macro!(44);
                $macro!(45);
                $macro!(46);
                $macro!(47);
                $macro!(48);
                $macro!(49);
                $macro!(50);
                $macro!(51);
                $macro!(52);
                $macro!(53);
                $macro!(54);
                $macro!(55);
                $macro!(56);
                $macro!(57);
                $macro!(58);
                $macro!(59);
                $macro!(60);
                $macro!(61);
                $macro!(62);
                $macro!(63);
            };
        }

        int_1_to_63_list!(declare_is_int_n);
        int_1_to_63_list!(declare_is_uint_n);
        int_1_to_63_list!(declare_truncate_to_int_n);
        int_1_to_63_list!(declare_checked_truncate_to_int_n);

        extern "C" {
            pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
        }

        pub fn CompareCharsEqualUnsigned<lchar, rchar>(
            lhs: *const lchar,
            rhs: *const rchar,
            chars: usize,
        ) -> bool
        where
            lchar: std::marker::Copy,
            rchar: std::marker::Copy,
            lchar: std::convert::Into<u32>,
            rchar: std::convert::Into<u32>,
        {
            if std::mem::size_of::<lchar>() == std::mem::size_of::<rchar>() {
                unsafe {
                    let lhs_ptr = lhs as *const u8;
                    let rhs_ptr = rhs as *const u8;
                    memcmp(lhs_ptr, rhs_ptr, chars * std::mem::size_of::<lchar>()) == 0
                }
            } else {
                let mut l = lhs;
                let mut r = rhs;
                for _ in 0..chars {
                    unsafe {
                        if (*l).into() != (*r).into() {
                            return false;
                        }
                        l = l.offset(1);
                        r = r.offset(1);
                    }
                }
                true
            }
        }

        pub fn CompareCharsEqual<lchar, rchar>(
            lhs: *const lchar,
            rhs: *const rchar,
            chars: usize,
        ) -> bool
        where
            lchar: std::marker::Copy,
            rchar: std::marker::Copy,
            lchar: std::convert::Into<u32>,
            rchar: std::convert::Into<u32>,
        {
            CompareCharsEqualUnsigned(lhs, rhs, chars)
        }

        pub fn CompareCharsUnsigned<lchar, rchar>(
            lhs: *const lchar,
            rhs: *const rchar,
            chars: usize,
        ) -> i32
        where
            lchar: std::marker::Copy,
            rchar: std::marker::Copy,
            lchar: std::convert::Into<i32>,
            rchar: std::convert::Into<i32>,
        {
            unsafe {
                let mut l = lhs;
                let mut r = rhs;
                for _ in 0..chars {
                    let r_val = (*l).into() - (*r).into();
                    if r_val != 0 {
                        return r_val;
                    }
                    l = l.offset(1);
                    r = r.offset(1);
                }
            }
            0
        }

        pub fn CompareChars<lchar, rchar>(
            lhs: *const lchar,
            rhs: *const rchar,
            chars: usize,
        ) -> i32
        where
            lchar: std::marker::Copy,
            rchar: std::marker::Copy,
            lchar: std::convert::Into<i32>,
            rchar: std::convert::Into<i32>,
        {
            CompareCharsUnsigned(lhs, rhs, chars)
        }
        pub fn ComputeUnseededHash(key: u32) -> u32 {
            let mut hash = key;
            hash = !hash + (hash << 15);
            hash = hash ^ (hash >> 12);
            hash = hash + (hash << 2);
            hash = hash ^ (hash >> 4);
            hash = hash * 2057;
            hash = hash ^ (hash >> 16);
            hash & 0x3fffffff
        }
        pub fn ComputeLongHash(key: u64) -> u32 {
            let mut hash = key;
            hash = !hash + (hash << 18);
            hash = hash ^ (hash >> 31);
            hash = hash * 21;
            hash = hash ^ (hash >> 11);
            hash = hash + (hash << 6);
            hash = hash ^ (hash >> 22);
            (hash & 0x3fffffff) as u32
        }
        pub fn ComputeSeededHash(key: u32, seed: u64) -> u32 {
            ComputeLongHash((key as u64) ^ seed)
        }
        pub fn ComputePointerHash(ptr: *mut std::ffi::c_void) -> u32 {
            ComputeUnseededHash(ptr as usize as u32)
        }
        pub fn ComputeAddressHash(address: usize) -> u32 {
            ComputeUnseededHash((address & 0xFFFFFFFF) as u32)
        }
        pub fn ReadFile(filename: &str) -> Result<String, io::Error> {
            use std::fs::File;
            use std::io::Read;

            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        pub fn DoubleToBoolean(d: f64) -> bool {
            if d.is_nan() {
                return false;
            }

            if d == 0.0 {
                return false;
            }

            true
        }
        pub fn TryAddIndexChar(index: &mut u32, c: char) -> bool {
            if c.is_digit(10) {
                if let Some(digit) = c.to_digit(10) {
                    if let Some(new_index) = index.checked_mul(10).and_then(|i| i.checked_add(digit)) {
                        *index = new_index;
                        return true;
                    }
                }
            }
            false
        }
        pub fn StringToIndex(stream: &str, index: &mut u32) -> bool {
            *index = 0;
            for c in stream.chars() {
                if !TryAddIndexChar(index, c) {
                    return false;
                }
            }
            true
        }
        pub fn GetCurrentStackPosition() -> usize {
            unsafe {
                let mut sp: *mut usize = std::mem::zeroed();
                asm!("mov {}, rsp", out(reg) sp);
                sp as usize
            }
        }

        pub fn ByteReverse16(value: u16) -> u16 {
            value.swap_bytes()
        }
        pub fn ByteReverse32(value: u32) -> u32 {
            value.swap_bytes()
        }

        pub fn ByteReverse64(value: u64) -> u64 {
            value.swap_bytes()
        }

        pub fn ByteReverse<V>(value: V) -> V
        where V: std::marker::Copy,
              V: From<u16>,
              V: From<u32>,
              V: From<u64>,
        {
            let size_of_v = std::mem::size_of::<V>();
            match size_of_v {
                1 => value,
                2 => ByteReverse16(value as u16).into(),
                4 => ByteReverse32(value as u32).into(),
                8 => ByteReverse64(value as u64).into(),
                _ => panic!("Unsupported size"),
            }
        }
        
        pub fn PassesFilter(name: &str, filter: &str) -> bool {
          if filter.is_empty() {
              return name.is_empty();
          }
          let mut filter_chars = filter.chars();
          let positive_filter = match filter_chars.next() {
              Some('-') => {
                  filter_chars.next();
                  false
              }
              _ => true,
          };
          let filter_string: String = filter_chars.collect();

          if filter_string.is_empty() {
              return !name.is_empty();
          }
          if filter_string == "*" {
              return positive_filter;
          }
          if filter_string == "~" {
              return !positive_filter;
          }

          let prefix_match = filter.ends_with("*");
          let min_match_length = if prefix_match {
              filter.len() - if positive_filter { 1 } else { 2 }
          } else {
              filter.len() - if positive_filter { 0 } else { 1 }
          };

          if name.len() < min_match_length {
              return !positive_filter;
          }

          if prefix_match {
              if name.starts_with(&filter_string[..filter_string.len() - 1]) {
                  return positive_filter;
              }
              return !positive_filter;
          }

          if filter_string == name {
              return positive_filter;
          }
          return !positive_filter;
        }
    }
}

extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}
pub fn is_nan<T>(v: &T) -> bool
where
    T: std::marker::Copy,
{
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() {
        let v: f64 = unsafe { std::mem::transmute_copy(v) };
        v.is_nan()
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>() {
        let v: f32 = unsafe { std::mem::transmute_copy(v) };
        v.is_nan()
    } else {
        false
    }
}

fn signbit<T>(v: &T) -> i32
where
    T: std::marker::Copy,
{
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() {
        let v: f64 = unsafe

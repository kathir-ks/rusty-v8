// Converted from V8 C++ source files:
// Header: smi.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod smi {
    use std::mem::size_of;
    use std::os::raw::c_int;
    use crate::v8::internal::*;

    #[repr(C)]
    pub struct Smi {}

    impl Smi {
        pub const kMinValue: i32 = i32::MIN / 2;
        pub const kMaxValue: i32 = i32::MAX / 2;
        pub const kSmiTag: usize = 0;
        pub const kSmiTagSize: usize = 1;
        pub const kSmiShiftSize: usize = 0;
        pub const kSmiValueSize: usize = 31;

        pub fn value(&self) -> i32 {
            unsafe { *(self as *const Self as *const i32) }
        }

        pub fn is_valid<T>(value: T) -> bool
        where
            T: std::ops::Add + std::ops::Sub + std::cmp::PartialOrd + Copy,
            T: Into<i64>,
        {
            let min_value: i64 = Smi::kMinValue.into();
            let max_value: i64 = Smi::kMaxValue.into();
            let value_i64: i64 = value.into();

            value_i64 >= min_value && value_i64 <= max_value
        }

        pub fn from_int(value: i32) -> Tagged<Smi> {
            assert!(Smi::is_valid(value));
            let smi_value = (value as i64) << 1;
            Tagged::from_raw(smi_value as usize)
        }
        
        pub fn from_intptr(value: isize) -> Tagged<Smi> {
             assert!(Smi::is_valid(value));
            let smi_shift_bits = Smi::kSmiTagSize + Smi::kSmiShiftSize;
            Tagged::from_raw(((value as usize) << smi_shift_bits) | Smi::kSmiTag)
        }

        pub fn to_uint32_smi(smi: Tagged<Smi>) -> Tagged<Smi> {
            let value = smi.value() as i32;
            if value <= 0 {
                Smi::from_int(0)
            } else {
                Smi::from_int(value as u32 as i32)
            }
        }

        pub fn to_int(object: Tagged<Object>) -> i32 {
            Tagged::<Smi>::from_raw(object.ptr()).value() as i32
        }

        pub fn from_31_bit_pattern(value: i32) -> Tagged<Smi> {
            Smi::from_int((value << (32 - Smi::kSmiValueSize)) >> (32 - Smi::kSmiValueSize))
        }

        pub fn lexicographic_compare(
            _isolate: *mut Isolate,
            x: Tagged<Smi>,
            y: Tagged<Smi>,
        ) -> Address {
            let x_val = x.value();
            let y_val = y.value();

            let result = match x_val.cmp(&y_val) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            (Smi::from_int(result)).ptr()
        }

        pub fn smi_print(smi: Tagged<Smi>, os: &mut std::ostream) {
            os.write(format!("{}", smi.value()).as_bytes()).unwrap();
        }

        pub fn zero() -> Tagged<Smi> {
            Smi::from_int(0)
        }

       pub fn uninitialized_deserialization_value() -> Tagged<Smi> {
            Tagged::<Smi>::from_raw(0)
       }
    }
}

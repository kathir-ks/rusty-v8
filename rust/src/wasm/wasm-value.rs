#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::convert::TryInto;
use std::mem;
use std::string::String;
use std::{fmt, mem::MaybeUninit, os::raw::c_char};

use crate::simd128::Simd128;

mod base {
    pub fn write_unaligned_value<T>(dst: *mut u8, value: T) {
        unsafe {
            (dst as *mut T).write_unaligned(value);
        }
    }

    pub fn read_unaligned_value<T>(src: *const u8) -> T {
        unsafe { (src as *const T).read_unaligned() }
    }
}

mod common {
    pub mod simd128 {
        #[derive(Clone, Copy)]
        #[repr(C, align(16))]
        pub struct Simd128 {
            pub data: [u8; 16],
        }

        impl Simd128 {
            pub fn new(data: [u8; 16]) -> Self {
                Simd128 { data }
            }
        }
        impl std::fmt::Debug for Simd128 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Simd128")
                    .field("data", &self.data)
                    .finish()
            }
        }

        impl Default for Simd128 {
            fn default() -> Self {
                Simd128 { data: [0u8; 16] }
            }
        }

        impl PartialEq for Simd128 {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }

        impl Eq for Simd128 {}
    }
}

mod handles {
    use std::ptr::NonNull;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DirectHandle<T> {
        ptr: NonNull<T>,
    }

    impl<T> DirectHandle<T> {
        pub fn new(ptr: NonNull<T>) -> Self {
            DirectHandle { ptr }
        }

        pub fn address(&self) -> usize {
            self.ptr.as_ptr() as usize
        }
    }
}

mod utils {
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Float32 {
        value: f32,
    }

    impl Float32 {
        pub fn new(value: f32) -> Self {
            Float32 { value }
        }
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Float64 {
        value: f64,
    }

    impl Float64 {
        pub fn new(value: f64) -> Self {
            Float64 { value }
        }
    }
}

mod wasm {
    use super::*;
    use crate::common::simd128::Simd128;
    use crate::handles::DirectHandle;
    use crate::utils::{Float32, Float64};
    use fp16::fp16_ieee::half::f16;
    use std::any::Any;
    use std::fmt;
    use std::mem::size_of;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum ValueTypeKind {
        I8,
        I16,
        I32,
        I64,
        F16,
        F32,
        F64,
        S128,
        RefNull,
        Ref,
        Void,
        Top,
        Bottom,
    }

    impl ValueTypeKind {
        pub fn value_kind_size(&self) -> usize {
            match self {
                ValueTypeKind::I8 => 1,
                ValueTypeKind::I16 => 2,
                ValueTypeKind::I32 => 4,
                ValueTypeKind::I64 => 8,
                ValueTypeKind::F16 => 2,
                ValueTypeKind::F32 => 4,
                ValueTypeKind::F64 => 8,
                ValueTypeKind::S128 => 16,
                ValueTypeKind::RefNull => std::mem::size_of::<usize>(),
                ValueTypeKind::Ref => std::mem::size_of::<usize>(),
                ValueTypeKind::Void => 0,
                ValueTypeKind::Top => 0,
                ValueTypeKind::Bottom => 0,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct CanonicalValueType {
        kind: ValueTypeKind,
    }

    impl CanonicalValueType {
        pub fn new(kind: ValueTypeKind) -> Self {
            CanonicalValueType { kind }
        }

        pub fn kind(&self) -> ValueTypeKind {
            self.kind
        }

        pub fn is_numeric(&self) -> bool {
            matches!(
                self.kind,
                ValueTypeKind::I8
                    | ValueTypeKind::I16
                    | ValueTypeKind::I32
                    | ValueTypeKind::I64
                    | ValueTypeKind::F16
                    | ValueTypeKind::F32
                    | ValueTypeKind::F64
                    | ValueTypeKind::S128
            )
        }

        pub fn is_reference(&self) -> bool {
            matches!(self.kind, ValueTypeKind::RefNull | ValueTypeKind::Ref)
        }

        pub fn value_kind_size(&self) -> usize {
            self.kind.value_kind_size()
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct ValueType {
        kind: ValueTypeKind,
    }

    impl ValueType {
        pub fn new(kind: ValueTypeKind) -> Self {
            ValueType { kind }
        }
    }

    pub const kWasmI8: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::I8,
    };
    pub const kWasmI16: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::I16,
    };
    pub const kWasmI32: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::I32,
    };
    pub const kWasmI64: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::I64,
    };
    pub const kWasmF16: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::F16,
    };
    pub const kWasmF32: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::F32,
    };
    pub const kWasmF64: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::F64,
    };
    pub const kWasmS128: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::S128,
    };
    pub const kWasmVoid: CanonicalValueType = CanonicalValueType {
        kind: ValueTypeKind::Void,
    };

    pub const kWasmI8_PACKED: ValueType = ValueType {
        kind: ValueTypeKind::I8,
    };
    pub const kWasmI16_PACKED: ValueType = ValueType {
        kind: ValueTypeKind::I16,
    };
    pub const kWasmI32_PACKED: ValueType = ValueType {
        kind: ValueTypeKind::I32,
    };

    pub struct WasmModule {}

    /// A wasm value with type information.
    #[derive(Clone, Copy)]
    pub struct WasmValue {
        type_: CanonicalValueType,
        bit_pattern_: [u8; 16],
        module_: *const WasmModule,
    }

    impl fmt::Debug for WasmValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WasmValue")
                .field("type_", &self.type_)
                .field("bit_pattern_", &self.bit_pattern_)
                .finish()
        }
    }

    impl WasmValue {
        pub fn new() -> Self {
            WasmValue {
                type_: kWasmVoid,
                bit_pattern_: [0u8; 16],
                module_: std::ptr::null(),
            }
        }

        pub fn new_i8(v: i8) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI8,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_i16(v: i16) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI16,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_i32(v: i32) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI32,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_u32(v: u32) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI32,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_i64(v: i64) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI64,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_u64(v: u64) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmI64,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_f16(v: f16) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v.to_bits());
            WasmValue {
                type_: kWasmF16,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_f32(v: f32) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmF32,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_f32_boxed(v: Float32) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v.value);
            WasmValue {
                type_: kWasmF32,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_f64(v: f64) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmF64,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_f64_boxed(v: Float64) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v.value);
            WasmValue {
                type_: kWasmF64,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_s128(v: Simd128) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), v);
            WasmValue {
                type_: kWasmS128,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn to_i8(&self) -> i8 {
            assert_eq!(self.type_, kWasmI8);
            self.to_i8_unchecked()
        }

        pub fn to_i8_unchecked(&self) -> i8 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_i16(&self) -> i16 {
            assert_eq!(self.type_, kWasmI16);
            self.to_i16_unchecked()
        }

        pub fn to_i16_unchecked(&self) -> i16 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_i32(&self) -> i32 {
            assert_eq!(self.type_, kWasmI32);
            self.to_i32_unchecked()
        }

        pub fn to_i32_unchecked(&self) -> i32 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_u32(&self) -> u32 {
            assert_eq!(self.type_, kWasmI32);
            self.to_u32_unchecked()
        }

        pub fn to_u32_unchecked(&self) -> u32 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_i64(&self) -> i64 {
            assert_eq!(self.type_, kWasmI64);
            self.to_i64_unchecked()
        }

        pub fn to_i64_unchecked(&self) -> i64 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_u64(&self) -> u64 {
            assert_eq!(self.type_, kWasmI64);
            self.to_u64_unchecked()
        }

        pub fn to_u64_unchecked(&self) -> u64 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_f16(&self) -> u16 {
            assert_eq!(self.type_, kWasmF16);
            self.to_f16_unchecked()
        }

        pub fn to_f16_unchecked(&self) -> u16 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_f32(&self) -> f32 {
            assert_eq!(self.type_, kWasmF32);
            self.to_f32_unchecked()
        }

        pub fn to_f32_unchecked(&self) -> f32 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_f64(&self) -> f64 {
            assert_eq!(self.type_, kWasmF64);
            self.to_f64_unchecked()
        }

        pub fn to_f64_unchecked(&self) -> f64 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn to_s128(&self) -> Simd128 {
            assert_eq!(self.type_, kWasmS128);
            self.to_s128_unchecked()
        }

        pub fn to_s128_unchecked(&self) -> Simd128 {
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn new_from_raw_bytes(raw_bytes: &[u8], type_: CanonicalValueType) -> Self {
            assert!(type_.is_numeric());
            let mut bit_pattern_ = [0u8; 16];
            let size = type_.value_kind_size();
            bit_pattern_[..size].copy_from_slice(&raw_bytes[..size]);

            WasmValue {
                type_,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn new_ref(ref_: DirectHandle<dyn Any>, type_: CanonicalValueType) -> Self {
            let mut bit_pattern_ = [0u8; 16];
            assert!(type_.is_reference());
            base::write_unaligned_value(bit_pattern_.as_mut_ptr(), ref_);
            WasmValue {
                type_,
                bit_pattern_,
                module_: std::ptr::null(),
            }
        }

        pub fn to_ref(&self) -> DirectHandle<dyn Any> {
            assert!(self.type_.is_reference());
            base::read_unaligned_value(self.bit_pattern_.as_ptr() as *const u8)
        }

        pub fn type_(&self) -> CanonicalValueType {
            self.type_
        }

        pub fn module(&self) -> *const WasmModule {
            self.module_
        }

        pub fn copy_to(&self, to: &mut [u8]) {
            assert!(size_of::<f32>() == size_of::<Float32>());
            assert!(size_of::<f64>() == size_of::<Float64>());
            assert!(self.type_.is_numeric());
            let size = self.type_.value_kind_size();
            to[..size].copy_from_slice(&self.bit_pattern_[..size]);
        }

        pub fn packed(&self, packed_type: ValueType) -> WasmValue {
            if packed_type.kind == ValueTypeKind::I8 {
                assert_eq!(self.type_, kWasmI32);
                WasmValue::new_i8(self.to_i32() as i8)
            } else if packed_type.kind == ValueTypeKind::I16 {
                assert_eq!(self.type_, kWasmI32);
                WasmValue::new_i16(self.to_i32() as i16)
            } else {
                *self
            }
        }

        pub fn for_uintptr(value: usize) -> Self {
            if std::mem::size_of::<usize>() == 8 {
                WasmValue::new_u64(value as u64)
            } else {
                WasmValue::new_u32(value as u32)
            }
        }

        pub fn zero_byte_representation(&self) -> bool {
            assert!(self.type_().is_numeric());
            let byte_count = self.type_().value_kind_size();
            self.bit_pattern_[..byte_count].iter().all(|&x| x == 0)
        }
    }

    impl PartialEq for WasmValue {
        fn eq(&self, other: &Self) -> bool {
            if self.type_ != other.type_ {
                return false;
            }

            let size = if self.type_.is_reference() {
                std::mem::size_of::<DirectHandle<dyn Any>>()
            } else {
                self.type_.value_kind_size()
            };

            self.bit_pattern_[..size] == other.bit_pattern_[..size]
        }
    }

    impl Eq for WasmValue {}

    impl WasmValue {
        fn to_string_internal(&self) -> String {
            match self.type_.kind() {
                ValueTypeKind::I8 => self.to_i8().to_string(),
                ValueTypeKind::I16 => self.to_i16().to_string(),
                ValueTypeKind::I32 => self.to_i32().to_string(),
                ValueTypeKind::I64 => self.to_i64().to_string(),
                ValueTypeKind::F16 => {
                    fp16::fp16_ieee::to_f32(f16::from_bits(self.to_f16())).to_string()
                }
                ValueTypeKind::F32 => self.to_f32().to_string(),
                ValueTypeKind::F64 => self.to_f64().to_string(),
                ValueTypeKind::S128 => {
                    let mut stream = String::new();
                    stream.push_str("0x");
                    for byte in self.bit_pattern_.iter() {
                        stream.push_str(&format!("{:02x}", byte));
                    }
                    stream
                }
                ValueTypeKind::RefNull | ValueTypeKind::Ref => {
                    format!("DirectHandle [{}]", self.to_ref().address())
                }
                ValueTypeKind::Void | ValueTypeKind::Top | ValueTypeKind::Bottom => {
                    unreachable!()
                }
            }
        }
    }

    impl fmt::Display for WasmValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string_internal())
        }
    }

    macro_rules! declare_cast {
        ($name:ident, $localtype:ident, $ctype:ty) => {
            impl WasmValue {
                #[inline]
                pub fn to_unchecked<T: 'static>(&self) -> T {
                    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<$ctype>() {
                        self.to_$name##_unchecked() as T
                    } else {
                        panic!("Type mismatch in to_unchecked: expected {}, got {:?}",
                               stringify!($ctype), std::any::TypeId::of::<T>());
                    }
                }

                #[inline]
                pub fn to<T: 'static>(&self) -> T {
                    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<$ctype>() {
                        self.to_$name() as T
                    } else {
                        panic!("Type mismatch in to: expected {}, got {:?}",
                               stringify!($ctype), std::any::TypeId::of::<T>());
                    }
                }
            }
        };
    }

    declare_cast!(i8, kWasmI8, i8);
    declare_cast!(i16, kWasmI16, i16);
    declare_cast!(i32, kWasmI32, i32);
    declare_cast!(u32, kWasmI32, u32);
    declare_cast!(i64, kWasmI64, i64);
    declare_cast!(u64, kWasmI64, u64);
    declare_cast!(f16, kWasmF16, u16);
    declare_cast!(f32, kWasmF32, f32);
    // declare_cast!(f32_boxed, kWasmF32, Float32); //Float32 is not primitive type
    declare_cast!(f64, kWasmF64, f64);
    // declare_cast!(f64_boxed, kWasmF64, Float64); //Float64 is not primitive type
    declare_cast!(s128, kWasmS128, Simd128);
}
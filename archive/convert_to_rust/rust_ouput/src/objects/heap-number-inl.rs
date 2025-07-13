// Converted from V8 C++ source files:
// Header: heap-number-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
mod base {
    pub mod memory {}
}
mod objects {
    pub mod primitive_heap_object_inl {
        pub struct PrimitiveHeapObject {}
    }
    use crate::base::memory;

    #[derive(Debug, Copy, Clone)]
    pub struct HeapNumber {
        value_: HeapNumberWithValue,
    }

    impl HeapNumber {
        pub fn value(&self) -> f64 {
            self.value_.value()
        }
        pub fn set_value(&mut self, value: f64) {
            #[cfg(feature = "v8_enable_experimental_undefined_double")]
            {
                if value.is_nan() {
                    panic!("Value cannot be NaN when V8_ENABLE_EXPERIMENTAL_UNDEFINED_DOUBLE is enabled")
                }
            }
            self.value_.set_value(value);
        }

        pub fn value_as_bits(&self) -> u64 {
            self.value_.value_as_bits()
        }

        pub fn set_value_as_bits(&mut self, bits: u64) {
            #[cfg(feature = "v8_enable_experimental_undefined_double")]
            {
                if bits == k_undefined_nan_int64 {
                    panic!("Bits cannot be kUndefinedNanInt64 when V8_ENABLE_EXPERIMENTAL_UNDEFINED_DOUBLE is enabled");
                }
            }
            self.value_.set_value_as_bits(bits);
        }
    }

    const k_undefined_nan_int64: u64 = 0x7ff8000000000000;

    #[derive(Debug, Copy, Clone)]
    struct HeapNumberWithValue {
        value: f64,
    }

    impl HeapNumberWithValue {
        pub fn value(&self) -> f64 {
            self.value
        }

        pub fn set_value(&mut self, value: f64) {
            self.value = value;
        }

        pub fn value_as_bits(&self) -> u64 {
            self.value.to_bits()
        }

        pub fn set_value_as_bits(&mut self, bits: u64) {
            self.value = f64::from_bits(bits);
        }
    }
}

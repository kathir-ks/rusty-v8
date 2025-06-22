// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/oddball-inl.h

pub mod oddball_inl {
    use crate::handles::handle::Handle;
    use crate::heap::heap::Heap;
    use crate::objects::number::Number;
    use crate::objects::oddball::Oddball;
    use crate::objects::string::String;
    use crate::isolate::isolate::Isolate;
    use crate::objects::smi::Smi;
    use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

    impl Oddball {
        #[inline]
        pub fn to_number_raw(&self) -> f64 {
            // Assuming to_number_raw_ is a field that can be directly accessed as f64
            self.to_number_raw_.load(Ordering::Relaxed).to_bits().into()
        }

        #[inline]
        pub fn set_to_number_raw(&self, value: f64) {
             let bits = value.to_bits();
             self.to_number_raw_.store(bits.into(), Ordering::Relaxed);
        }

        #[inline]
        pub fn set_to_number_raw_as_bits(&self, bits: u64) {
            // Bug(v8:8875): HeapNumber's double may be unaligned.
            self.to_number_raw_.store(bits.into(), Ordering::Relaxed);
        }

        #[inline]
        pub fn to_string(&self) -> String {
            // Assuming to_string_ is a field that can be directly accessed as String
            self.to_string_.load(Ordering::Relaxed)
        }

        #[inline]
        pub fn set_to_string(&self, value: String) {
            self.to_string_.store(value, Ordering::Relaxed);
        }

        #[inline]
        pub fn to_number(&self) -> Number {
            // Assuming to_number_ is a field that can be directly accessed as Number
            self.to_number_.load(Ordering::Relaxed)
        }

        #[inline]
        pub fn set_to_number(&self, value: Number) {
            self.to_number_.store(value, Ordering::Relaxed);
        }

        #[inline]
        pub fn type_of(&self) -> String {
            // Assuming type_of_ is a field that can be directly accessed as String
            self.type_of_.load(Ordering::Relaxed)
        }

        #[inline]
        pub fn set_type_of(&self, value: String) {
            self.type_of_.store(value, Ordering::Relaxed);
        }

        #[inline]
        pub fn kind(&self) -> u8 {
            // Assuming kind_ is a field that can be directly accessed as u8
            self.kind_.load(Ordering::Relaxed) as u8
        }

        #[inline]
        pub fn set_kind(&self, value: u8) {
            self.kind_.store(value, Ordering::Relaxed);
        }

        pub fn to_number_handle<'a>(isolate: &'a Isolate, input: &'a Oddball) -> Handle<'a, Number> {
            Handle::new(input.to_number(), isolate)
        }
    }

    pub fn is_boolean(obj: &Oddball) -> bool {
        (obj.kind() & Oddball::K_NOT_BOOLEAN_MASK) == 0
    }

    impl Oddball {
        pub const K_NOT_BOOLEAN_MASK: u8 = 0b0000_0001;
    }

    pub fn to_bool(boolean: &Oddball, _isolate: &Isolate) -> bool {
        //DCHECK(IsBoolean(this, isolate));
        is_boolean(boolean) && is_true(boolean)
    }

    pub fn is_true(boolean: &Oddball) -> bool {
        boolean.kind() == 1 // Assuming True is represented by kind 1.
    }
}
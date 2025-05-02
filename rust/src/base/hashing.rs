// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        convert::TryInto,
        hash::{Hash, Hasher},
        mem::transmute,
        ops::{BitXor, Mul, Not, Shl, Shr},
    };

    // Placeholder for V8_INLINE.  Rust functions are often inlined automatically.
    macro_rules! v8_inline {
        ($x:item) => {
            #[inline]
            $x
        };
    }

    // Placeholder for V8_HOST_ARCH_32_BIT, assuming 64-bit architecture for now.
    // Modify as needed for different architectures.
    const V8_HOST_ARCH_32_BIT: bool = cfg!(target_pointer_width = "32");

    /// Combines two hash values together. This code was taken from MurmurHash.
    v8_inline! {
        pub fn hash_combine(seed: usize, hash: usize) -> usize {
            if V8_HOST_ARCH_32_BIT {
                let c1: u32 = 0xCC9E2D51;
                let c2: u32 = 0x1B873593;

                let mut hash: u32 = hash.try_into().unwrap(); // Convert usize to u32
                hash = hash.wrapping_mul(c1);
                hash = hash.rotate_right(15);
                hash = hash.wrapping_mul(c2);

                let mut seed: u32 = seed.try_into().unwrap(); // Convert usize to u32
                seed ^= hash;
                seed = seed.rotate_right(13);
                seed = seed.wrapping_mul(5).wrapping_add(0xE6546B64);

                seed.try_into().unwrap() // Convert u32 back to usize
            } else {
                let m: u64 = 0xC6A4A7935BD1E995;
                let r: u32 = 47;

                let mut hash: u64 = hash.try_into().unwrap(); // Convert usize to u64
                hash = hash.wrapping_mul(m);
                hash ^= hash >> r;
                hash = hash.wrapping_mul(m);

                let mut seed: u64 = seed.try_into().unwrap(); // Convert usize to u64
                seed ^= hash;
                seed = seed.wrapping_mul(m);

                seed.try_into().unwrap() // Convert u64 back to usize
            }
        }
    }

    /// `Hasher` makes it easier to combine multiple fields into one hash and
    /// avoids the ambiguity of the different `hash_combine` methods.
    pub struct HasherState {
        hash: usize,
    }

    impl Default for HasherState {
        fn default() -> Self {
            HasherState { hash: 0 }
        }
    }

    impl HasherState {
        /// Creates a new `Hasher` with a default seed of 0.
        pub const fn new() -> Self {
            HasherState { hash: 0 }
        }

        /// Creates a new `Hasher` with the given seed.
        pub const fn with_seed(seed: usize) -> Self {
            HasherState { hash: seed }
        }

        /// Retrieve the current hash.
        pub const fn finish(&self) -> usize {
            self.hash
        }

        /// Combine an existing hash value into this hasher's hash.
        pub fn add_hash(&mut self, other_hash: usize) -> &mut Self {
            self.hash = hash_combine(self.hash, other_hash);
            self
        }

        /// Hash a value {t} and combine its hash into this hasher's hash.
        pub fn add<T: Hash>(&mut self, t: &T) -> &mut Self {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            self.add_hash(s.finish() as usize);
            self
        }

        /// Hash a range of values and combine the hashes into this hasher's hash.
        pub fn add_range<I, T>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = T>,
            T: Hash,
        {
            for item in iter {
                self.add(&item);
            }
            self
        }

        /// Hash multiple values and combine their hashes.
        pub fn combine<T: Hash>(values: &[T]) -> usize {
            let mut hasher = HasherState::new();
            for value in values {
                hasher.add(value);
            }
            hasher.finish()
        }
    }

    /// Thomas Wang, Integer Hash Functions.
    /// https://gist.github.com/badboy/6267743
    v8_inline! {
        fn hash_value_unsigned_impl<T>(v: T) -> usize
        where
            T: Copy +
                Not<Output = T> +
                Shl<usize, Output = T> +
                Shr<usize, Output = T> +
                Mul<Output = T> +
                BitXor<Output = T> +
                std::convert::TryInto<usize>,
            usize: std::convert::TryFrom<T>,
        {
            let size = std::mem::size_of::<T>();
            match size {
                4 => {
                    // "32 bit Mix Functions"
                    let mut v = v;
                    v = !v + (v << 15); // v = (v << 15) - v - 1;
                    v = v ^ (v >> 12);
                    v = v + (v << 2);
                    v = v ^ (v >> 4);
                    v = v * unsafe { std::mem::transmute::<i32, T>(2057) }; // v = (v + (v << 3)) + (v << 11);
                    v = v ^ (v >> 16);
                    v.try_into().unwrap()
                }
                8 => {
                    match std::mem::size_of::<usize>() {
                        4 => {
                            // "64 bit to 32 bit Hash Functions"
                            let mut v = v;
                            v = !v + (v << 18); // v = (v << 18) - v - 1;
                            v = v ^ (v >> 31);
                            v = v * unsafe { std::mem::transmute::<i64, T>(21) }; // v = (v + (v << 2)) + (v << 4);
                            v = v ^ (v >> 11);
                            v = v + (v << 6);
                            v = v ^ (v >> 22);
                            v.try_into().unwrap()
                        }
                        8 => {
                            // "64 bit Mix Functions"
                            let mut v = v;
                            v = !v + (v << 21); // v = (v << 21) - v - 1;
                            v = v ^ (v >> 24);
                            v = (v + (v << 3)) + (v << 8); // v * 265
                            v = v ^ (v >> 14);
                            v = (v + (v << 2)) + (v << 4); // v * 21
                            v = v ^ (v >> 28);
                            v = v + (v << 31);
                            v.try_into().unwrap()
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    macro_rules! v8_base_hash_value_trivial {
        ($type:ty) => {
            v8_inline! {
                impl Hash for $type {
                    fn hash<H: Hasher>(&self, state: &mut H) {
                        (*self as usize).hash(state);
                    }
                }
            }
        };
    }

    v8_base_hash_value_trivial!(bool);
    v8_base_hash_value_trivial!(u8);
    v8_base_hash_value_trivial!(u16);

    v8_inline! {
        impl Hash for u32 {
            fn hash<H: Hasher>(&self, state: &mut H) {
                hash_value_unsigned_impl(*self).hash(state);
            }
        }
    }

    v8_inline! {
        impl Hash for u64 {
            fn hash<H: Hasher>(&self, state: &mut H) {
                hash_value_unsigned_impl(*self).hash(state);
            }
        }
    }

    v8_inline! {
        impl Hash for u128 {
            fn hash<H: Hasher>(&self, state: &mut H) {
                hash_value_unsigned_impl(*self).hash(state);
            }
        }
    }

    macro_rules! v8_base_hash_value_signed {
        ($type:ty) => {
            v8_inline! {
                impl Hash for $type {
                    fn hash<H: Hasher>(&self, state: &mut H) {
                         let unsigned_value: <$type as std::convert::TryInto<usize>>::Error = unsafe { std::mem::transmute(*self) };
                        unsigned_value.hash(state);
                    }
                }
            }
        };
    }

    v8_base_hash_value_signed!(i8);
    v8_base_hash_value_signed!(i16);
    v8_base_hash_value_signed!(i32);
    v8_base_hash_value_signed!(i64);
    v8_base_hash_value_signed!(i128);

    v8_inline! {
        impl Hash for f32 {
            fn hash<H: Hasher>(&self, state: &mut H) {
                if *self != 0.0f32 {
                    unsafe { transmute::<f32, u32>(*self) }.hash(state);
                } else {
                    0.hash(state);
                }
            }
        }
    }

    v8_inline! {
        impl Hash for f64 {
            fn hash<H: Hasher>(&self, state: &mut H) {
                if *self != 0.0f64 {
                    unsafe { transmute::<f64, u64>(*self) }.hash(state);
                } else {
                    0.hash(state);
                }
            }
        }
    }

    v8_inline! {
        impl<T: Hash, const N: usize> Hash for [T; N] {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let mut hasher = HasherState::new();
                hasher.add_range(self);
                state.write_usize(hasher.finish());
            }
        }
    }

    v8_inline! {
        impl<T: Hash> Hash for *const T {
            fn hash<H: Hasher>(&self, state: &mut H) {
                ( *self as usize).hash(state);
            }
        }
    }

    v8_inline! {
        impl<T: Hash, U: Hash> Hash for (T, U) {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let mut hasher = HasherState::new();
                hasher.add(&self.0);
                hasher.add(&self.1);
                state.write_usize(hasher.finish());
            }
        }
    }

    v8_inline! {
        impl<T: Hash> Hash for (T,) {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let mut hasher = HasherState::new();
                hasher.add(&self.0);
                state.write_usize(hasher.finish());
            }
        }
    }

    //Need to implement a hash value for tuples with more than two elements

    // Implement Hash for enums by casting to the underlying type.
    v8_inline! {
        impl<T> Hash for T
        where
            T: std::marker::Copy,
            T: std::convert::TryInto<usize>,
            T: std::convert::TryFrom<usize>,
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                (*self as usize).hash(state);
            }
        }
    }

    ///A trait for types that can be converted to hash values.
    pub trait Hashable {
        fn get_hash(&self) -> usize;
    }

    //TODO: Implement trait `Hashable` for any type with function `hash_value()`

    /// Implements a bitwise equality comparison.
    pub struct bit_equal_to<T>(std::marker::PhantomData<T>);

    impl<T> bit_equal_to<T> {
        pub fn new() -> Self {
            bit_equal_to(std::marker::PhantomData)
        }
    }

    impl<T> Default for bit_equal_to<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Implements a bitwise hash function.
    pub struct bit_hash<T>(std::marker::PhantomData<T>);

        impl<T> bit_hash<T> {
        pub fn new() -> Self {
            bit_hash(std::marker::PhantomData)
        }
    }

    impl<T> Default for bit_hash<T> {
        fn default() -> Self {
            Self::new()
        }
    }


    macro_rules! v8_base_bit_specialize_trivial {
        ($type:ty) => {
            impl bit_equal_to<$type> {
                v8_inline! {
                    pub fn call(&self, lhs: $type, rhs: $type) -> bool {
                        lhs == rhs
                    }
                }
            }

            impl bit_hash<$type> {
                v8_inline! {
                    pub fn call(&self, v: $type) -> usize {
                        let mut s = std::collections::hash_map::DefaultHasher::new();
                        v.hash(&mut s);
                        s.finish() as usize
                    }
                }
            }
        };
    }

    v8_base_bit_specialize_trivial!(i8);
    v8_base_bit_specialize_trivial!(u8);
    v8_base_bit_specialize_trivial!(i16);
    v8_base_bit_specialize_trivial!(u16);
    v8_base_bit_specialize_trivial!(i32);
    v8_base_bit_specialize_trivial!(u32);
    v8_base_bit_specialize_trivial!(i64);
    v8_base_bit_specialize_trivial!(u64);
    v8_base_bit_specialize_trivial!(i128);
    v8_base_bit_specialize_trivial!(u128);

    macro_rules! v8_base_bit_specialize_bit_cast {
        ($type:ty, $btype:ty) => {
            impl bit_equal_to<$type> {
                v8_inline! {
                    pub fn call(&self, lhs: $type, rhs: $type) -> bool {
                        unsafe { transmute::<$type, $btype>(lhs) == transmute::<$type, $btype>(rhs) }
                    }
                }
            }

            impl bit_hash<$type> {
                v8_inline! {
                    pub fn call(&self, v: $type) -> usize {
                        let h = bit_hash::<$btype>::new();
                        h.call(unsafe { transmute::<$type, $btype>(v) })
                    }
                }
            }
        };
    }

    v8_base_bit_specialize_bit_cast!(f32, u32);
    v8_base_bit_specialize_bit_cast!(f64, u64);
}
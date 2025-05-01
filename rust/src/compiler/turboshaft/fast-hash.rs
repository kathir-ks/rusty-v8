// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod fast_hash {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        iter::Iterator,
        marker::Copy,
        mem,
    };

    // fast_hash_combine() / fast_hash_value() produce a bad but very fast to
    // compute hash, intended for hash-tables and only usable for data that is
    // sufficiently random already and has high variance in their low bits.

    #[inline]
    pub fn fast_hash_combine() -> usize {
        0
    }

    #[inline]
    pub fn fast_hash_combine1(acc: usize) -> usize {
        acc
    }

    #[inline]
    pub fn fast_hash_combine2(acc: usize, value: usize) -> usize {
        17 * acc + value
    }

    #[inline]
    pub fn fast_hash_combine_variadic<T: Hash>(v: T) -> usize {
        let mut s = DefaultHasher::new();
        v.hash(&mut s);
        s.finish() as usize
    }
    
    #[inline]
    pub fn fast_hash_combine_variadic2<T: Hash>(acc: usize, v: T) -> usize {
        let mut s = DefaultHasher::new();
        v.hash(&mut s);
        fast_hash_combine2(acc, s.finish() as usize)
    }

    pub trait FastHash {
        fn fast_hash(&self) -> usize;
    }

    impl<T> FastHash for T
    where
        T: Hash,
    {
        default fn fast_hash(&self) -> usize {
            let mut s = DefaultHasher::new();
            self.hash(&mut s);
            s.finish() as usize
        }
    }

    impl<T> FastHash for T
    where
        T: Copy + Into<usize>,
    {
        fn fast_hash(&self) -> usize {
            (*self).into()
        }
    }

    impl<T1: Hash, T2: Hash> FastHash for (T1, T2) {
        fn fast_hash(&self) -> usize {
            fast_hash_combine2(self.0.fast_hash(), self.1.fast_hash())
        }
    }

    impl<T: Hash, const N: usize> FastHash for [T; N] {
        fn fast_hash(&self) -> usize {
            let mut acc = 0;
            for item in self.iter() {
                acc = fast_hash_combine2(acc, item.fast_hash());
            }
            acc
        }
    }

    impl FastHash for String {
        fn fast_hash(&self) -> usize {
            let mut s = DefaultHasher::new();
            self.hash(&mut s);
            s.finish() as usize
        }
    }

    impl<T: Hash> FastHash for Vec<T> {
        fn fast_hash(&self) -> usize {
            let mut acc = 0;
            for item in self.iter() {
                acc = fast_hash_combine2(acc, item.fast_hash());
            }
            acc
        }
    }

    pub fn fast_hash_range<I, T>(mut iter: I) -> usize
    where
        I: Iterator<Item = T>,
        T: Hash,
    {
        let mut acc = 0;
        while let Some(value) = iter.next() {
            acc = fast_hash_combine2(acc, value.fast_hash());
        }
        acc
    }
}
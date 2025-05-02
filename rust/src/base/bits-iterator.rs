// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::iter::Iterator;
use std::marker::PhantomData;
use num_traits::PrimInt;

pub mod bits {
    use super::*;

    /// An iterator over the set bits in a number.
    ///
    /// Iterates from LSB to MSB by default.  The `kMSBFirst` template
    /// parameter can be used to iterate from MSB to LSB.
    pub struct BitsIterator<T: PrimInt, const MSB_FIRST: bool = false> {
        bits: T,
        _marker: PhantomData<T>,
    }

    impl<T: PrimInt, const MSB_FIRST: bool> BitsIterator<T, MSB_FIRST> {
        /// Creates a new `BitsIterator`.
        pub fn new(bits: T) -> Self {
            BitsIterator {
                bits,
                _marker: PhantomData,
            }
        }
    }

    impl<T: PrimInt, const MSB_FIRST: bool> Iterator for BitsIterator<T, MSB_FIRST> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.bits == T::zero() {
                return None;
            }

            let index = if MSB_FIRST {
                (T::BITS as usize) - 1 - self.bits.leading_zeros() as usize
            } else {
                self.bits.trailing_zeros() as usize
            };

            self.bits &= !(T::one() << index);
            Some(index)
        }
    }

    impl<T: PrimInt, const MSB_FIRST: bool> PartialEq for BitsIterator<T, MSB_FIRST> {
        fn eq(&self, other: &Self) -> bool {
            self.bits == other.bits
        }
    }

    impl<T: PrimInt, const MSB_FIRST: bool> Eq for BitsIterator<T, MSB_FIRST> {}

    /// Creates an iterable over the bits in {bits}, from LSB to MSB.
    pub fn iterate_bits<T: PrimInt>(bits: T) -> impl Iterator<Item = usize> {
        BitsIterator::<T>::new(bits)
            .take_while(move |_| bits != T::zero())

    }

    /// Creates an iterable over the bits in {bits}, from MSB to LSB.
    pub fn iterate_bits_backwards<T: PrimInt>(bits: T) -> impl Iterator<Item = usize> {
        BitsIterator::<T, true>::new(bits)
            .take_while(move |_| bits != T::zero())
    }
}
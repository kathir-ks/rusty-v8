// Converted from V8 C++ source files:
// Header: bits-iterator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bits_iterator {
    use std::iter::Iterator;
    use std::marker::PhantomData;
    use std::ops::{BitAndAssign, Shl, Shr};

    use num_traits::{PrimInt, Unsigned, Zero};

    use crate::base::bits::{CountLeadingZeros, CountTrailingZeros};
    use crate::base::iterator::make_iterator_range;

    pub struct BitsIterator<T, const MSB_FIRST: bool = false>
    where
        T: PrimInt + Unsigned,
    {
        bits_: T,
        phantom: PhantomData<[(); MSB_FIRST as usize]>,
    }

    impl<T, const MSB_FIRST: bool> BitsIterator<T, MSB_FIRST>
    where
        T: PrimInt + Unsigned,
    {
        pub fn new(bits: T) -> Self {
            BitsIterator {
                bits_: bits,
                phantom: PhantomData,
            }
        }
    }

    impl<T, const MSB_FIRST: bool> Iterator for BitsIterator<T, MSB_FIRST>
    where
        T: PrimInt + Unsigned,
    {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.bits_.is_zero() {
                return None;
            }

            let bit_index = if MSB_FIRST {
                (8 * std::mem::size_of::<T>() as i32) - 1 - CountLeadingZeros(self.bits_)
            } else {
                CountTrailingZeros(self.bits_)
            };

            self.bits_ &= !(T::one() << bit_index);

            Some(bit_index)
        }
    }

    impl<T, const MSB_FIRST: bool> PartialEq for BitsIterator<T, MSB_FIRST>
    where
        T: PrimInt + Unsigned,
    {
        fn eq(&self, other: &Self) -> bool {
            self.bits_ == other.bits_
        }
    }

    impl<T, const MSB_FIRST: bool> Eq for BitsIterator<T, MSB_FIRST> where T: PrimInt + Unsigned {}

    pub fn iterate_bits<T>(bits: T) -> impl Iterator<Item = i32>
    where
        T: PrimInt + Unsigned,
    {
        make_iterator_range(BitsIterator::<T, false>::new(bits), BitsIterator::<T, false>::new(T::zero()))
    }

    pub fn iterate_bits_backwards<T>(bits: T) -> impl Iterator<Item = i32>
    where
        T: PrimInt + Unsigned,
    {
        make_iterator_range(
            BitsIterator::<T, true>::new(bits),
            BitsIterator::<T, true>::new(T::zero()),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_iterate_bits_lsb() {
            let bits: u32 = 0b10110;
            let mut iterator = iterate_bits(bits);

            assert_eq!(iterator.next(), Some(1));
            assert_eq!(iterator.next(), Some(2));
            assert_eq!(iterator.next(), Some(4));
            assert_eq!(iterator.next(), None);
        }

        #[test]
        fn test_iterate_bits_msb() {
            let bits: u32 = 0b10110;
            let mut iterator = iterate_bits_backwards(bits);

            assert_eq!(iterator.next(), Some(4));
            assert_eq!(iterator.next(), Some(2));
            assert_eq!(iterator.next(), Some(1));
            assert_eq!(iterator.next(), None);
        }

        #[test]
        fn test_iterate_bits_empty() {
            let bits: u32 = 0;
            let mut iterator = iterate_bits(bits);
            assert_eq!(iterator.next(), None);

            let mut iterator_backwards = iterate_bits_backwards(bits);
            assert_eq!(iterator_backwards.next(), None);
        }
    }
}

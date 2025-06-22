// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        cmp::{max, min},
        marker::PhantomData,
    };

    /// `BitField` is a helper struct for encoding and decoding bitfields with
    /// unsigned content.
    /// Instantiate them via 'type alias', which is cheaper than deriving a new struct:
    /// `type MyBitField = BitField<MyEnum, 4, 2>;`
    /// The BitField struct is final to enforce this style over derivation.
    pub struct BitField<T, const SHIFT: usize, const SIZE: usize, U = u32> {
        _phantom_t: PhantomData<T>,
        _phantom_u: PhantomData<U>,
    }

    impl<T, const SHIFT: usize, const SIZE: usize, U> BitField<T, SHIFT, SIZE, U>
    where
        U: std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Sub<Output = U>
            + std::convert::From<u8>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::fmt::Debug,
        T: std::convert::Into<U> + std::convert::From<U> + std::marker::Copy,
        U: std::convert::TryInto<T>,
        <U as std::convert::TryInto<T>>::Error: std::fmt::Debug,
    {
        const _: () = {
            assert!(std::mem::size_of::<U>() > 0);
            assert!(SHIFT < 8 * std::mem::size_of::<U>());
            assert!(SIZE < 8 * std::mem::size_of::<U>());
            assert!(SHIFT + SIZE <= 8 * std::mem::size_of::<U>());
            assert!(SIZE > 0);
        };

        pub type FieldType = T;
        pub type BaseType = U;

        pub const K_SHIFT: usize = SHIFT;
        pub const K_SIZE: usize = SIZE;
        pub const K_MASK: U = ((U::from(1) << SHIFT) << SIZE) - (U::from(1) << SHIFT);
        pub const K_LAST_USED_BIT: usize = SHIFT + SIZE - 1;
        pub const K_NUM_VALUES: U = U::from(1) << SIZE;
        pub const K_MAX: U = Self::K_NUM_VALUES - U::from(1);

        pub type Next<T2, const SIZE2: usize> = BitField<T2, { SHIFT + SIZE }, SIZE2, U>;

        /// Tells whether the provided value fits into the bit field.
        pub const fn is_valid(value: T) -> bool {
            let value_u: U = value.into();
            (value_u & !Self::K_MAX) == U::from(0)
        }

        /// Returns a type U with the bit field value encoded.
        pub const fn encode(value: T) -> U {
            assert!(Self::is_valid(value));
            let value_u: U = value.into();
            value_u << SHIFT
        }

        /// Returns a type U with the bit field value updated.
        pub const fn update(previous: U, value: T) -> U {
            (previous & !Self::K_MASK) | Self::encode(value)
        }

        /// Extracts the bit field from the value.
        pub fn decode(value: U) -> T {
            let decoded_value: U = (value & Self::K_MASK) >> SHIFT;
            decoded_value
                .try_into()
                .expect("Failed to convert decoded value to T")
        }
    }

    /// `BitFieldUnion` can be used to combine two linear BitFields.
    /// So far only the static mask is computed. Encoding and decoding tbd.
    /// Can be used for example as a quick combined check:
    ///   `if BitFieldUnion::<BFA, BFB>::K_MASK & bitfield { ... }`
    pub struct BitFieldUnion<A, B> {
        _phantom_a: PhantomData<A>,
        _phantom_b: PhantomData<B>,
    }

    impl<A, B> BitFieldUnion<A, B>
    where
        A: MaskedBitField,
        B: MaskedBitField<BaseType = A::BaseType>,
    {
        const _: () = {
            assert!((A::K_MASK & B::K_MASK) == A::BaseType::from(0));
        };

        pub const K_SHIFT: usize = min(A::K_SHIFT, B::K_SHIFT);
        pub const K_MASK: A::BaseType = A::K_MASK | B::K_MASK;
        pub const K_SIZE: usize =
            A::K_SIZE + B::K_SIZE + (max(A::K_SHIFT, B::K_SHIFT) - Self::K_SHIFT);
    }

    pub trait MaskedBitField {
        type BaseType;
        const K_SHIFT: usize;
        const K_MASK: Self::BaseType;
        const K_SIZE: usize;
    }

    impl<T, const SHIFT: usize, const SIZE: usize, U> MaskedBitField
        for BitField<T, SHIFT, SIZE, U>
    where
        U: std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Sub<Output = U>
            + std::convert::From<u8>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::fmt::Debug,
        T: std::convert::Into<U> + std::convert::From<U> + std::marker::Copy,
        U: std::convert::TryInto<T>,
        <U as std::convert::TryInto<T>>::Error: std::fmt::Debug,
    {
        type BaseType = U;
        const K_SHIFT: usize = SHIFT;
        const K_MASK: Self::BaseType = BitField::<T, SHIFT, SIZE, U>::K_MASK;
        const K_SIZE: usize = SIZE;
    }

    pub type BitField8<T, const SHIFT: usize, const SIZE: usize> =
        BitField<T, SHIFT, SIZE, u8>;
    pub type BitField16<T, const SHIFT: usize, const SIZE: usize> =
        BitField<T, SHIFT, SIZE, u16>;
    pub type BitField64<T, const SHIFT: usize, const SIZE: usize> =
        BitField<T, SHIFT, SIZE, u64>;

    // Helper macros for defining a contiguous sequence of bit fields. Example:
    // (backslashes at the ends of respective lines of this multi-line macro
    // definition are omitted here to please the compiler)
    //
    // #define MAP_BIT_FIELD1(V, _)
    //   V(IsAbcBit, bool, 1, _)
    //   V(IsBcdBit, bool, 1, _)
    //   V(CdeBits, int, 5, _)
    //   V(DefBits, MutableMode, 1, _)
    //
    // DEFINE_BIT_FIELDS(MAP_BIT_FIELD1)
    // or
    // DEFINE_BIT_FIELDS_64(MAP_BIT_FIELD1)
    //
    macro_rules! define_bit_field_range_type {
        ($Name:ident, $Type:ty, $Size:expr, _) => {
            $Name##Start, $Name##End = $Name##Start + $Size - 1,
        };
    }

    macro_rules! define_bit_ranges {
        ($LIST_MACRO:ident) => {
            struct $LIST_MACRO##_Ranges {
                #[allow(dead_code)]
                bits: u32,
            }
            impl $LIST_MACRO##_Ranges {
                const {
                    #[allow(non_snake_case, unused_braces)]
                    struct Ranges {
                        $LIST_MACRO!(define_bit_field_range_type, _),
                    }
                    let ranges = Ranges {
                        $LIST_MACRO!(define_bit_field_range_type, _),
                    };
                    let mut bits_count = 0;
                    bits_count
                }
                /*
                enum {
                    $LIST_MACRO!(define_bit_field_range_type, _) kBitsCount
                }
                */
            }
        };
    }

    macro_rules! define_bit_field_type {
        ($Name:ident, $Type:ty, $Size:expr, $RangesName:ident) => {
            #[allow(dead_code)]
            type $Name = BitField<$Type, {$RangesName::$Name##Start}, $Size>;
        };
    }

    macro_rules! define_bit_field_64_type {
        ($Name:ident, $Type:ty, $Size:expr, $RangesName:ident) => {
            #[allow(dead_code)]
            type $Name = BitField64<$Type, {$RangesName::$Name##Start}, $Size>;
        };
    }

    macro_rules! define_bit_fields {
        ($LIST_MACRO:ident) => {
            define_bit_ranges!($LIST_MACRO);
            $LIST_MACRO!(define_bit_field_type, $LIST_MACRO##_Ranges);
        };
    }

    macro_rules! define_bit_fields_64 {
        ($LIST_MACRO:ident) => {
            define_bit_ranges!($LIST_MACRO);
            $LIST_MACRO!(define_bit_field_64_type, $LIST_MACRO##_Ranges);
        };
    }

    /// `BitSetComputer` is a helper struct for encoding and decoding information for
    /// a variable number of items in an array.
    ///
    /// To encode boolean data in a smi array you would use:
    ///  `type BoolComputer = BitSetComputer<bool, 1, kSmiValueSize, u32>;`
    pub struct BitSetComputer<T, const BITS_PER_ITEM: usize, const BITS_PER_WORD: usize, U> {
        _phantom_t: PhantomData<T>,
        _phantom_u: PhantomData<U>,
    }

    impl<T, const BITS_PER_ITEM: usize, const BITS_PER_WORD: usize, U>
        BitSetComputer<T, BITS_PER_ITEM, BITS_PER_WORD, U>
    where
        U: std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Shr<usize, Output = U>
            + std::ops::Not<Output = U>
            + std::convert::From<u8>
            + std::marker::Copy,
        T: std::convert::Into<u32> + std::convert::From<u32> + std::marker::Copy,
    {
        pub const K_ITEMS_PER_WORD: usize = BITS_PER_WORD / BITS_PER_ITEM;
        pub const K_MASK: usize = (1 << BITS_PER_ITEM) - 1;

        /// The number of array elements required to embed T information for each item.
        pub const fn word_count(items: usize) -> usize {
            if items == 0 {
                return 0;
            }
            (items - 1) / Self::K_ITEMS_PER_WORD + 1
        }

        /// The array index to look at for item.
        pub const fn index(base_index: usize, item: usize) -> usize {
            base_index + item / Self::K_ITEMS_PER_WORD
        }

        /// Extract T data for a given item from data.
        pub fn decode(data: U, item: usize) -> T {
            let shift_value = Self::shift(item) as u32;
            let mask_value = Self::K_MASK as u32;
            let shifted_data = (data >> shift_value) & U::from(mask_value as u8);
            (shifted_data.into()).into()
        }

        /// Return the encoding for a store of value for item in previous.
        pub fn encode(previous: U, item: usize, value: T) -> U {
            let shift_value = Self::shift(item);
            let value_u32: u32 = value.into();
            let set_bits: u32 = (value_u32 << shift_value) as u32;

            let mask: u32 = Self::K_MASK as u32;
            let shift_value_u32: u32 = shift_value as u32;
            let shifted_mask: u32 = mask << shift_value_u32;
            let not_shifted_mask: u32 = !shifted_mask;
            let previous_u32: u32 = previous.into() as u32;
            let masked_previous: u32 = previous_u32 & not_shifted_mask;
            let result: u32 = masked_previous | set_bits;

            result.into()
        }

        pub const fn shift(item: usize) -> usize {
            (item % Self::K_ITEMS_PER_WORD) * BITS_PER_ITEM
        }
    }
}
// Converted from V8 C++ source files:
// Header: bit-field.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::{
        marker::PhantomData,
        mem::{size_of, MaybeUninit},
    };

    // ----------------------------------------------------------------------------
    // BitField is a help template for encoding and decode bitfield with
    // unsigned content.
    // Instantiate them via 'using', which is cheaper than deriving a new class:
    // using MyBitField = base::BitField<MyEnum, 4, 2>;
    // The BitField class is final to enforce this style over derivation.
    pub struct BitField<T, const SHIFT: usize, const SIZE: usize, U = u32> {
        _phantom_t: PhantomData<T>,
        _phantom_u: PhantomData<U>,
    }

    impl<T, const SHIFT: usize, const SIZE: usize, U> BitField<T, SHIFT, SIZE, U>
    where
        U: std::marker::Copy
            + std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Shr<usize, Output = U>
            + std::ops::Sub<Output = U>
            + From<u8>,
    {
        const_assert!(std::mem::size_of::<U>() > 0);
        const_assert!(SHIFT < 8 * size_of::<U>()); // Otherwise shifts by {shift} are UB.
        const_assert!(SIZE < 8 * size_of::<U>()); // Otherwise shifts by {size} are UB.
        const_assert!(SHIFT + SIZE <= 8 * size_of::<U>());
        const_assert!(SIZE > 0);

        pub type FieldType = T;
        pub type BaseType = U;

        // A type U mask of bit field.  To use all bits of a type U of x bits
        // in a bitfield without compiler warnings we have to compute 2^x
        // without using a shift count of x in the computation.
        pub const K_SHIFT: usize = SHIFT;
        pub const K_SIZE: usize = SIZE;
        pub const K_MASK: U = ((U::from(1) << SHIFT) << SIZE) - (U::from(1) << SHIFT);
        pub const K_LAST_USED_BIT: usize = SHIFT + SIZE - 1;
        pub const K_NUM_VALUES: U = U::from(1) << SIZE;
        pub const K_MAX: U = Self::K_NUM_VALUES - U::from(1);

        pub type Next<T2, const SIZE2: usize> = BitField<T2, { SHIFT + SIZE }, SIZE2, U>;

        // Tells whether the provided value fits into the bit field.
        pub const fn is_valid(value: T) -> bool
        where
            T: std::convert::TryInto<U>,
        {
            match value.try_into() {
                Ok(val) => {
                    let converted_value: U = val;
                    (converted_value & !Self::K_MAX) == U::from(0)
                }
                Err(_) => false,
            }
        }

        // Returns a type U with the bit field value encoded.
        pub const fn encode(value: T) -> U
        where
            T: std::convert::TryInto<U>,
        {
            assert!(Self::is_valid(value));
            match value.try_into() {
                Ok(val) => {
                    let converted_value: U = val;
                    converted_value << SHIFT
                }
                Err(_) => panic!("Failed to convert T to U"),
            }
        }

        // Returns a type U with the bit field value updated.
        #[must_use]
        pub const fn update(previous: U, value: T) -> U
        where
            T: std::convert::TryInto<U>,
        {
            (previous & !Self::K_MASK) | Self::encode(value)
        }

        // Extracts the bit field from the value.
        pub const fn decode(value: U) -> T
        where
            U: std::convert::TryInto<T>,
        {
            match ((value & Self::K_MASK) >> SHIFT).try_into() {
                Ok(val) => val,
                Err(_) => panic!("Failed to convert U to T"),
            }
        }
    }

    // ----------------------------------------------------------------------------
    // BitFieldUnion can be used to combine two linear BitFields.
    // So far only the static mask is computed. Encoding and decoding tbd.
    // Can be used for example as a quick combined check:
    //   `if (BitFieldUnion<BFA, BFB>::kMask & bitfield) ...`
    pub struct BitFieldUnion<A, B> {
        _phantom_a: PhantomData<A>,
        _phantom_b: PhantomData<B>,
    }

    impl<A, B> BitFieldUnion<A, B>
    where
        A: MaskedBitfield,
        B: MaskedBitfield<BaseType = <A as MaskedBitfield>::BaseType>,
        <A as MaskedBitfield>::BaseType:
            std::ops::BitAnd<Output = <A as MaskedBitfield>::BaseType>
            + std::ops::BitOr<Output = <A as MaskedBitfield>::BaseType>,
    {
        const_assert!((A::k_mask() & B::k_mask()) == A::BaseType::from(0));

        pub const K_SHIFT: usize = std::cmp::min(A::k_shift(), B::k_shift());
        pub const K_MASK: <A as MaskedBitfield>::BaseType = A::k_mask() | B::k_mask();
        pub const K_SIZE: usize = A::k_size() + B::k_size()
            + (std::cmp::max(A::k_shift(), B::k_shift()) - Self::K_SHIFT);
    }

    pub trait MaskedBitfield {
        type BaseType: std::marker::Copy;
        fn k_shift() -> usize;
        fn k_mask() -> Self::BaseType;
        fn k_size() -> usize;
    }

    impl<T, const SHIFT: usize, const SIZE: usize, U> MaskedBitfield
        for BitField<T, SHIFT, SIZE, U>
    where
        U: std::marker::Copy
            + std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Shr<usize, Output = U>
            + std::ops::Sub<Output = U>
            + From<u8>,
    {
        type BaseType = U;
        fn k_shift() -> usize {
            Self::K_SHIFT
        }
        fn k_mask() -> Self::BaseType {
            Self::K_MASK
        }
        fn k_size() -> usize {
            Self::K_SIZE
        }
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
    // NOTE: Converted macros to functions due to limitations in Rust's macro system.

    // ----------------------------------------------------------------------------
    // BitSetComputer is a help template for encoding and decoding information for
    // a variable number of items in an array.
    //
    // To encode boolean data in a smi array you would use:
    //  using BoolComputer = BitSetComputer<bool, 1, kSmiValueSize, uint32_t>;
    pub struct BitSetComputer<T, const K_BITS_PER_ITEM: usize, const K_BITS_PER_WORD: usize, U> {
        _phantom_t: PhantomData<T>,
        _phantom_u: PhantomData<U>,
    }

    impl<T, const K_BITS_PER_ITEM: usize, const K_BITS_PER_WORD: usize, U>
        BitSetComputer<T, K_BITS_PER_ITEM, K_BITS_PER_WORD, U>
    where
        U: std::marker::Copy
            + std::ops::BitAnd<Output = U>
            + std::ops::BitOr<Output = U>
            + std::ops::Shl<usize, Output = U>
            + std::ops::Shr<usize, Output = U>
            + From<u8>
            + std::convert::TryInto<T>,
        T: std::marker::Copy + std::convert::From<u8>,
    {
        pub const K_ITEMS_PER_WORD: usize = K_BITS_PER_WORD / K_BITS_PER_ITEM;
        pub const K_MASK: usize = (1 << K_BITS_PER_ITEM) - 1;

        // The number of array elements required to embed T information for each item.
        pub fn word_count(items: int) -> int {
            if items == 0 {
                return 0;
            }
            (items - 1) / Self::K_ITEMS_PER_WORD + 1
        }

        // The array index to look at for item.
        pub fn index(base_index: int, item: int) -> int {
            base_index + item / Self::K_ITEMS_PER_WORD
        }

        // Extract T data for a given item from data.
        pub fn decode(data: U, item: int) -> T {
            let shift_value = Self::shift(item);
            let masked_value = (data >> shift_value) & U::from(Self::K_MASK as u8);
            match masked_value.try_into() {
                Ok(val) => val,
                Err(_) => panic!("Failed to convert U to T"),
            }
        }

        // Return the encoding for a store of value for item in previous.
        pub fn encode(previous: U, item: int, value: T) -> U {
            let shift_value = Self::shift(item);
            let value_as_u8 = u8::from(value);
            let set_bits: U = U::from(value_as_u8) << shift_value;
            let mask: U = U::from(Self::K_MASK as u8) << shift_value;
            (previous & !mask) | set_bits
        }

        pub fn shift(item: int) -> usize {
            (item % Self::K_ITEMS_PER_WORD) * K_BITS_PER_ITEM
        }
    }
} // namespace base

#[macro_export]
macro_rules! const_assert {
    ($condition:expr) => {
        #[allow(dead_code)]
        const _: [(); 0 - !{$condition} as usize] = [];
    };
}

type int = i32;

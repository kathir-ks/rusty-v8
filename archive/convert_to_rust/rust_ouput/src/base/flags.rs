// Converted from V8 C++ source files:
// Header: flags.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {

    use std::hash::{Hash, Hasher};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Flags<EnumT, BitfieldT = i32, BitfieldStorageT = i32>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        mask_: BitfieldStorageT,
        _phantom: std::marker::PhantomData<EnumT>,
        _phantom2: std::marker::PhantomData<BitfieldT>,
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        pub const fn new() -> Self {
            Self {
                mask_: 0.into(),
                _phantom: std::marker::PhantomData,
                _phantom2: std::marker::PhantomData,
            }
        }

        pub const fn from_flag(flag: EnumT) -> Self {
            Self {
                mask_: (flag.into()).into(),
                _phantom: std::marker::PhantomData,
                _phantom2: std::marker::PhantomData,
            }
        }

        pub const fn from_mask(mask: BitfieldT) -> Self {
            Self {
                mask_: mask.into(),
                _phantom: std::marker::PhantomData,
                _phantom2: std::marker::PhantomData,
            }
        }

        pub const fn operator_eq(&self, flag: EnumT) -> bool {
            let flag_as_bitfield: BitfieldT = flag.into();
            let mask_as_bitfield: BitfieldT = self.mask_.into();
            mask_as_bitfield == flag_as_bitfield
        }

        pub const fn operator_ne(&self, flag: EnumT) -> bool {
            let flag_as_bitfield: BitfieldT = flag.into();
            let mask_as_bitfield: BitfieldT = self.mask_.into();
            mask_as_bitfield != flag_as_bitfield
        }

        pub fn operator_and_assign(&mut self, flags: &Self) -> &mut Self {
            let flags_mask: BitfieldT = flags.mask_.into();
            let self_mask: BitfieldT = self.mask_.into();
            self.mask_ = (self_mask & flags_mask).into();
            self
        }

        pub fn operator_or_assign(&mut self, flags: &Self) -> &mut Self {
            let flags_mask: BitfieldT = flags.mask_.into();
            let self_mask: BitfieldT = self.mask_.into();
            self.mask_ = (self_mask | flags_mask).into();
            self
        }

        pub fn operator_xor_assign(&mut self, flags: &Self) -> &mut Self {
            let flags_mask: BitfieldT = flags.mask_.into();
            let self_mask: BitfieldT = self.mask_.into();
            self.mask_ = (self_mask ^ flags_mask).into();
            self
        }

        pub const fn operator_and(&self, flags: &Self) -> Self {
            let self_mask: BitfieldT = self.mask_.into();
            let flags_mask: BitfieldT = flags.mask_.into();
            Self::from_mask(self_mask & flags_mask)
        }

        pub const fn operator_or(&self, flags: &Self) -> Self {
            let self_mask: BitfieldT = self.mask_.into();
            let flags_mask: BitfieldT = flags.mask_.into();
            Self::from_mask(self_mask | flags_mask)
        }

        pub const fn operator_xor(&self, flags: &Self) -> Self {
            let self_mask: BitfieldT = self.mask_.into();
            let flags_mask: BitfieldT = flags.mask_.into();
            Self::from_mask(self_mask ^ flags_mask)
        }

        pub fn operator_and_assign_flag(&mut self, flag: EnumT) -> &mut Self {
            *self &= Self::from_flag(flag);
            self
        }

        pub fn operator_or_assign_flag(&mut self, flag: EnumT) -> &mut Self {
            *self |= Self::from_flag(flag);
            self
        }

        pub fn operator_xor_assign_flag(&mut self, flag: EnumT) -> &mut Self {
            *self ^= Self::from_flag(flag);
            self
        }

        pub fn set(&mut self, flag: EnumT, value: bool) -> &mut Self {
            if value {
                *self |= flag;
            } else {
                *self &= !Self::from_flag(flag);
            }
            self
        }

        pub const fn operator_and_flag(&self, flag: EnumT) -> Self {
            self.operator_and(&Self::from_flag(flag))
        }

        pub const fn operator_or_flag(&self, flag: EnumT) -> Self {
            self.operator_or(&Self::from_flag(flag))
        }

        pub const fn operator_xor_flag(&self, flag: EnumT) -> Self {
            self.operator_xor(&Self::from_flag(flag))
        }

        pub const fn operator_not(&self) -> Self {
            let mask: BitfieldT = self.mask_.into();
            Self::from_mask(!mask)
        }

        pub const fn without(&self, flag: EnumT) -> Self {
            self.operator_and((!Self::from_flag(flag)))
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitAndAssign for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        fn bitand_assign(&mut self, rhs: Self) {
            self.operator_and_assign(&rhs);
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitOrAssign for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        fn bitor_assign(&mut self, rhs: Self) {
            self.operator_or_assign(&rhs);
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitXorAssign for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.operator_xor_assign(&rhs);
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitAnd for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            self.operator_and(&rhs)
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitOr for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            self.operator_or(&rhs)
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitXor for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            self.operator_xor(&rhs)
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::Not for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Output = Self;

        fn not(self) -> Self::Output {
            self.operator_not()
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> From<Flags<EnumT, BitfieldT, BitfieldStorageT>> for BitfieldT
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        fn from(flags: Flags<EnumT, BitfieldT, BitfieldStorageT>) -> Self {
            flags.mask_.into()
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::Deref for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Target = BitfieldT;

        fn deref(&self) -> &Self::Target {
            let bitfield: &BitfieldT = unsafe { std::mem::transmute(&self.mask_) };
            bitfield
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::Neg for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash + std::ops::BitAnd<Output = BitfieldT>
            + std::ops::BitOr<Output = BitfieldT>
            + std::ops::BitXor<Output = BitfieldT>
            + std::ops::Not<Output = BitfieldT>,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash + From<BitfieldT> + Into<BitfieldT>,
    {
        type Output = bool;

        fn neg(self) -> Self::Output {
            self.mask_ == 0.into()
        }
    }

    impl<EnumT, BitfieldT, BitfieldStorageT> Hash for Flags<EnumT, BitfieldT, BitfieldStorageT>
    where
        EnumT: Copy + Clone + Eq + PartialEq + Hash + Into<BitfieldT>,
        BitfieldT: Copy + Clone + Eq + PartialEq + Hash,
        BitfieldStorageT: Copy + Clone + Eq + PartialEq + Hash,
    {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.mask_.hash(state);
        }
    }

    pub trait FlagType<Type> {
        type FlagTypeValue;
    }

    #[macro_export]
    macro_rules! define_operators_for_flags {
        ($Type:ident) => {
            #[allow(unused)]
            const fn operator_and(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as FlagType<$Type>>::FlagTypeValue) -> $Type {
                $Type::from_flag(lhs) & rhs
            }

            #[allow(unused)]
            const fn operator_and_2(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: &$Type) -> $Type {
                rhs & lhs
            }

            #[allow(unused)]
            fn operator_and_3(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as From<$Type>>::from) {}

            #[allow(unused)]
            const fn operator_or(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as FlagType<$Type>>::FlagTypeValue) -> $Type {
                $Type::from_flag(lhs) | rhs
            }

            #[allow(unused)]
            const fn operator_or_2(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: &$Type) -> $Type {
                rhs | lhs
            }

            #[allow(unused)]
            fn operator_or_3(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as From<$Type>>::from) {}

            #[allow(unused)]
            const fn operator_xor(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as FlagType<$Type>>::FlagTypeValue) -> $Type {
                $Type::from_flag(lhs) ^ rhs
            }

            #[allow(unused)]
            const fn operator_xor_2(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: &$Type) -> $Type {
                rhs ^ lhs
            }

            #[allow(unused)]
            fn operator_xor_3(lhs: <$Type as FlagType<$Type>>::FlagTypeValue, rhs: <$Type as From<$Type>>::from) {}

            #[allow(unused)]
            const fn operator_not(val: <$Type as FlagType<$Type>>::FlagTypeValue) -> $Type {
                !$Type::from_flag(val)
            }
        };
    }
}

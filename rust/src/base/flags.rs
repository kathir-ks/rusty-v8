// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// The Flags struct provides a type-safe way of storing OR-combinations of enum
/// values.
///
/// The traditional C++ approach for storing OR-combinations of enum values is to
/// use an int or unsigned int variable. The inconvenience with this approach is
/// that there's no type checking at all; any enum value can be OR'd with any
/// other enum value and passed on to a function that takes an int or unsigned
/// int.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Flags<EnumT, BitfieldT = i32, BitfieldStorageT = BitfieldT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    mask_: BitfieldStorageT,
    _phantom: std::marker::PhantomData<(EnumT, BitfieldT)>,
}

impl<EnumT, BitfieldT, BitfieldStorageT> Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy
        + PartialEq,
    BitfieldT: Copy,
    EnumT: Copy,
{
    /// The type of the flag.
    pub type flag_type = EnumT;
    /// The type of the mask.
    pub type mask_type = BitfieldT;

    /// Creates a new empty Flags instance.
    pub const fn new() -> Self {
        Self {
            mask_: BitfieldStorageT::from(BitfieldT::from(0)),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new Flags instance from a flag.
    pub const fn from_flag(flag: flag_type) -> Self {
        Self {
            mask_: BitfieldStorageT::from(flag),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new Flags instance from a mask.
    pub const fn from_mask(mask: mask_type) -> Self {
        Self {
            mask_: BitfieldStorageT::from(mask),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Checks if the Flags instance is equal to a flag.
    pub const fn eq_flag(&self, flag: flag_type) -> bool {
        self.mask_ == BitfieldStorageT::from(flag)
    }

    /// Checks if the Flags instance is not equal to a flag.
    pub const fn ne_flag(&self, flag: flag_type) -> bool {
        self.mask_ != BitfieldStorageT::from(flag)
    }

    /// Bitwise AND assignment.
    pub fn and_assign(&mut self, flags: &Self) -> &mut Self {
        self.mask_ = self.mask_ & flags.mask_;
        self
    }

    /// Bitwise OR assignment.
    pub fn or_assign(&mut self, flags: &Self) -> &mut Self {
        self.mask_ = self.mask_ | flags.mask_;
        self
    }

    /// Bitwise XOR assignment.
    pub fn xor_assign(&mut self, flags: &Self) -> &mut Self {
        self.mask_ = self.mask_ ^ flags.mask_;
        self
    }

    /// Bitwise AND.
    pub const fn bitand(&self, flags: &Self) -> Self {
        Self {
            mask_: self.mask_ & flags.mask_,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Bitwise OR.
    pub const fn bitor(&self, flags: &Self) -> Self {
        Self {
            mask_: self.mask_ | flags.mask_,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Bitwise XOR.
    pub const fn bitxor(&self, flags: &Self) -> Self {
        Self {
            mask_: self.mask_ ^ flags.mask_,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Bitwise AND assignment with a flag.
    pub fn and_assign_flag(&mut self, flag: flag_type) -> &mut Self {
        self.and_assign(&Self::from_flag(flag))
    }

    /// Bitwise OR assignment with a flag.
    pub fn or_assign_flag(&mut self, flag: flag_type) -> &mut Self {
        self.or_assign(&Self::from_flag(flag))
    }

    /// Bitwise XOR assignment with a flag.
    pub fn xor_assign_flag(&mut self, flag: flag_type) -> &mut Self {
        self.xor_assign(&Self::from_flag(flag))
    }

    /// Sets or clears given flag.
    pub fn set(&mut self, flag: flag_type, value: bool) -> &mut Self {
        if value {
            self.or_assign(&Self::from_flag(flag));
        } else {
            self.and_assign(&(!Self::from_flag(flag)));
        }
        self
    }

    /// Bitwise AND with a flag.
    pub const fn bitand_flag(&self, flag: flag_type) -> Self {
        self.bitand(&Self::from_flag(flag))
    }

    /// Bitwise OR with a flag.
    pub const fn bitor_flag(&self, flag: flag_type) -> Self {
        self.bitor(&Self::from_flag(flag))
    }

    /// Bitwise XOR with a flag.
    pub const fn bitxor_flag(&self, flag: flag_type) -> Self {
        self.bitxor(&Self::from_flag(flag))
    }

    /// Bitwise NOT.
    pub const fn not(&self) -> Self {
        Self {
            mask_: !self.mask_,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Converts to the mask type.
    pub const fn as_mask(&self) -> mask_type {
        BitfieldT::from(self.mask_)
    }

    /// Checks if the Flags instance is empty.
    pub const fn is_empty(&self) -> bool {
        self.mask_ == BitfieldStorageT::from(BitfieldT::from(0))
    }

    /// Returns a new Flags instance without the given flag.
    pub const fn without(&self, flag: flag_type) -> Self {
        self.bitand((!Self::from_flag(flag)))
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitAnd for Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.bitand(&rhs)
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitOr for Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitor(&rhs)
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::BitXor for Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bitxor(&rhs)
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> std::ops::Not for Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> From<Flags<EnumT, BitfieldT, BitfieldStorageT>> for BitfieldT
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy,
    BitfieldT: Copy,
    EnumT: Copy,
{
    fn from(flags: Flags<EnumT, BitfieldT, BitfieldStorageT>) -> Self {
        flags.as_mask()
    }
}

impl<EnumT, BitfieldT, BitfieldStorageT> std::hash::Hash for Flags<EnumT, BitfieldT, BitfieldStorageT>
where
    BitfieldStorageT: std::ops::BitAnd<Output = BitfieldStorageT>
        + std::ops::BitOr<Output = BitfieldStorageT>
        + std::ops::BitXor<Output = BitfieldStorageT>
        + std::ops::Not<Output = BitfieldStorageT>
        + std::convert::From<EnumT>
        + std::convert::From<BitfieldT>
        + Copy
        + std::hash::Hash,
    BitfieldT: Copy,
    EnumT: Copy,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mask_.hash(state);
    }
}

macro_rules! define_operators_for_flags {
    ($Type:ident) => {
        impl $Type {
            const V8_ALLOW_UNUSED: () = ();
            const V8_WARN_UNUSED_RESULT: () = ();
        }
        
        impl std::ops::BitAnd<$Type::flag_type> for $Type::flag_type {
            type Output = $Type;

            fn bitand(self, rhs: Self) -> Self::Output {
                $Type::from_flag(self) & $Type::from_flag(rhs)
            }
        }

        impl std::ops::BitAnd<$Type> for $Type::flag_type {
            type Output = $Type;

            fn bitand(self, rhs: $Type) -> Self::Output {
                rhs & $Type::from_flag(self)
            }
        }
        
        // The following does nothing as designed in the original code.
        // impl std::ops::BitAnd<$Type::mask_type> for $Type::flag_type {
        //     type Output = ();

        //     fn bitand(self, _rhs: $Type::mask_type) -> Self::Output {
        //     }
        // }

        impl std::ops::BitOr<$Type::flag_type> for $Type::flag_type {
            type Output = $Type;

            fn bitor(self, rhs: Self) -> Self::Output {
                $Type::from_flag(self) | $Type::from_flag(rhs)
            }
        }

        impl std::ops::BitOr<$Type> for $Type::flag_type {
            type Output = $Type;

            fn bitor(self, rhs: $Type) -> Self::Output {
                rhs | $Type::from_flag(self)
            }
        }
        
        // The following does nothing as designed in the original code.
        // impl std::ops::BitOr<$Type::mask_type> for $Type::flag_type {
        //     type Output = ();

        //     fn bitor(self, _rhs: $Type::mask_type) -> Self::Output {
        //     }
        // }

        impl std::ops::BitXor<$Type::flag_type> for $Type::flag_type {
            type Output = $Type;

            fn bitxor(self, rhs: Self) -> Self::Output {
                $Type::from_flag(self) ^ $Type::from_flag(rhs)
            }
        }

        impl std::ops::BitXor<$Type> for $Type::flag_type {
            type Output = $Type;

            fn bitxor(self, rhs: $Type) -> Self::Output {
                rhs ^ $Type::from_flag(self)
            }
        }
        
        // The following does nothing as designed in the original code.
        // impl std::ops::BitXor<$Type::mask_type> for $Type::flag_type {
        //     type Output = ();

        //     fn bitxor(self, _rhs: $Type::mask_type) -> Self::Output {
        //     }
        // }

        impl std::ops::Not for $Type::flag_type {
            type Output = $Type;

            fn not(self) -> Self::Output {
                !$Type::from_flag(self)
            }
        }
    };
}

pub mod base {
    pub use super::Flags;
    pub use super::define_operators_for_flags;
}

pub mod v8 {
    pub use super::base;
}
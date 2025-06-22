// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod interpreter {
    pub use super::bytecode_operands::*;
    use std::array;
    use std::marker::PhantomData;

    pub struct OperandTypeInfoTraits<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> OperandTypeInfoTraits<T> {
        pub const K_IS_SCALABLE: bool = false;
        pub const K_IS_UNSIGNED: bool = false;
        pub const K_UNSCALED_SIZE: OperandSize = OperandSize::Byte;
    }

    // Macro emulation.  Since we don't know the actual values of the enums,
    // we can't create a generic macro.  Instead, we have to hardcode
    // implementations for each enum value.  This is clearly not ideal.

    macro_rules! declare_operand_type_info {
        ($name:ident, $scalable:expr, $unsigned:expr, $base_size:expr) => {
            impl OperandTypeInfoTraits<OperandTypeInfo::$name> {
                pub const K_IS_SCALABLE: bool = $scalable;
                pub const K_IS_UNSIGNED: bool = $unsigned;
                pub const K_UNSCALED_SIZE: OperandSize = $base_size;
            }
        };
    }

    // This would be replaced by the actual OPERAND_TYPE_INFO_LIST macro call.
    declare_operand_type_info!(kNone, false, false, OperandSize::Byte);
    declare_operand_type_info!(kReg, false, false, OperandSize::Byte);
    declare_operand_type_info!(kImm8, false, false, OperandSize::Byte);
    declare_operand_type_info!(kImm16, false, false, OperandSize::Short);
    declare_operand_type_info!(kImm32, false, false, OperandSize::Int);
    declare_operand_type_info!(kUImm8, true, true, OperandSize::Byte);
    declare_operand_type_info!(kUImm16, true, true, OperandSize::Short);
    declare_operand_type_info!(kUImm32, true, true, OperandSize::Int);
    declare_operand_type_info!(kIdx8, false, false, OperandSize::Byte);
    declare_operand_type_info!(kIdx16, false, false, OperandSize::Short);
    declare_operand_type_info!(kIdx32, false, false, OperandSize::Int);

    pub struct OperandTraits<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> OperandTraits<T> {
        pub type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kNone>;
        pub const K_OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kNone;
    }

    macro_rules! declare_operand_type_traits {
        ($name:ident, $info_type:ident) => {
            impl OperandTraits<OperandType::$name> {
                pub type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::$info_type>;
                pub const K_OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::$info_type;
            }
        };
    }

    // This would be replaced by the actual OPERAND_TYPE_LIST macro call.
    declare_operand_type_traits!(kNone, kNone);
    declare_operand_type_traits!(kReg, kReg);
    declare_operand_type_traits!(kImm8, kImm8);
    declare_operand_type_traits!(kImm16, kImm16);
    declare_operand_type_traits!(kImm32, kImm32);
    declare_operand_type_traits!(kUImm8, kUImm8);
    declare_operand_type_traits!(kUImm16, kUImm16);
    declare_operand_type_traits!(kUImm32, kUImm32);
    declare_operand_type_traits!(kIdx8, kIdx8);
    declare_operand_type_traits!(kIdx16, kIdx16);
    declare_operand_type_traits!(kIdx32, kIdx32);

    pub struct OperandScaler<const OPERAND_TYPE: OperandType, const OPERAND_SCALE: OperandScale>;

    impl<const OPERAND_TYPE: OperandType, const OPERAND_SCALE: OperandScale> OperandScaler<OPERAND_TYPE, OPERAND_SCALE> {
        pub const K_SIZE: i32 = {
            let unscaled_size = OperandTraits::<OPERAND_TYPE>::TypeInfoTraits::K_UNSCALED_SIZE as i32;
            let is_scalable = OperandTraits::<OPERAND_TYPE>::TypeInfoTraits::K_IS_SCALABLE;
            let scale = OPERAND_SCALE as i32;

            unscaled_size * if is_scalable { scale } else { 1 }
        };
        pub const K_OPERAND_SIZE: OperandSize = match Self::K_SIZE {
            1 => OperandSize::Byte,
            2 => OperandSize::Short,
            4 => OperandSize::Int,
            _ => OperandSize::None, // Or handle the error case as appropriate
        };
    }

    pub struct BytecodeTraits<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse, const OPERANDS: &'static [OperandType]> {
    }

    impl<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse, const OPERANDS: &'static [OperandType]> BytecodeTraits<IMPLICIT_REGISTER_USE, OPERANDS> {
        pub const K_OPERAND_TYPES: &'static [OperandType] = OPERANDS;
        pub const K_OPERAND_TYPE_INFOS: [OperandTypeInfo; OPERANDS.len()] = {
            let mut infos: [OperandTypeInfo; OPERANDS.len()] = [OperandTypeInfo::kNone; OPERANDS.len()];
            let mut i = 0;
            while i < OPERANDS.len() {
                infos[i] = OperandTraits::<{OPERANDS[i]}>::K_OPERAND_TYPE_INFO;
                i += 1;
            }
            infos
        };

        pub const K_SINGLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS.len()] = {
            let mut sizes: [OperandSize; OPERANDS.len()] = [OperandSize::None; OPERANDS.len()];
            let mut i = 0;
            while i < OPERANDS.len() {
                sizes[i] = OperandScaler::<{OPERANDS[i]}, {OperandScale::kSingle}>::K_OPERAND_SIZE;
                i += 1;
            }
            sizes
        };

        pub const K_DOUBLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS.len()] = {
            let mut sizes: [OperandSize; OPERANDS.len()] = [OperandSize::None; OPERANDS.len()];
            let mut i = 0;
            while i < OPERANDS.len() {
                sizes[i] = OperandScaler::<{OPERANDS[i]}, {OperandScale::kDouble}>::K_OPERAND_SIZE;
                i += 1;
            }
            sizes
        };

        pub const K_QUADRUPLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS.len()] = {
            let mut sizes: [OperandSize; OPERANDS.len()] = [OperandSize::None; OPERANDS.len()];
            let mut i = 0;
            while i < OPERANDS.len() {
                sizes[i] = OperandScaler::<{OPERANDS[i]}, {OperandScale::kQuadruple}>::K_OPERAND_SIZE;
                i += 1;
            }
            sizes
        };

        const fn calculate_operand_offsets<const SCALE: OperandScale>() -> [i32; OPERANDS.len() + 1] {
            let mut result = [0i32; OPERANDS.len() + 1];
            let mut offset = 1;
            let mut i = 0;
            while i <= OPERANDS.len() {
                if i < OPERANDS.len() {
                   result[i] = offset;
                   offset += OperandScaler::<{OPERANDS[i]}, SCALE>::K_SIZE;
                } else {
                   result[i] = offset;
                }
                i += 1;
            }
            result
        }


        pub const K_SINGLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS.len() + 1] = Self::calculate_operand_offsets::<{OperandScale::kSingle}>();
        pub const K_DOUBLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS.len() + 1] = Self::calculate_operand_offsets::<{OperandScale::kDouble}>();
        pub const K_QUADRUPLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS.len() + 1] = Self::calculate_operand_offsets::<{OperandScale::kQuadruple}>();

        pub const K_SINGLE_SCALE_SIZE: i32 = {
            let mut size = 1;
            let mut i = 0;
            while i < OPERANDS.len() {
                size += OperandScaler::<{OPERANDS[i]}, {OperandScale::kSingle}>::K_SIZE;
                i += 1;
            }
            size
        };

        pub const K_DOUBLE_SCALE_SIZE: i32 = {
            let mut size = 1;
            let mut i = 0;
            while i < OPERANDS.len() {
                size += OperandScaler::<{OPERANDS[i]}, {OperandScale::kDouble}>::K_SIZE;
                i += 1;
            }
            size
        };

        pub const K_QUADRUPLE_SCALE_SIZE: i32 = {
            let mut size = 1;
            let mut i = 0;
            while i < OPERANDS.len() {
                size += OperandScaler::<{OPERANDS[i]}, {OperandScale::kQuadruple}>::K_SIZE;
                i += 1;
            }
            size
        };

        pub const K_IMPLICIT_REGISTER_USE: ImplicitRegisterUse = IMPLICIT_REGISTER_USE;
        pub const K_OPERAND_COUNT: usize = OPERANDS.len();
    }

    impl<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse> BytecodeTraits<IMPLICIT_REGISTER_USE, &[]> {
        pub const K_OPERAND_TYPES: Option<&'static [OperandType]> = None;
        pub const K_OPERAND_TYPE_INFOS: Option<&'static [OperandTypeInfo]> = None;
        pub const K_SINGLE_SCALE_OPERAND_SIZES: Option<&'static [OperandSize]> = None;
        pub const K_DOUBLE_SCALE_OPERAND_SIZES: Option<&'static [OperandSize]> = None;
        pub const K_QUADRUPLE_SCALE_OPERAND_SIZES: Option<&'static [OperandSize]> = None;

        pub const K_SINGLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];
        pub const K_DOUBLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];
        pub const K_QUADRUPLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];

        pub const K_SINGLE_SCALE_SIZE: i32 = 1;
        pub const K_DOUBLE_SCALE_SIZE: i32 = 1;
        pub const K_QUADRUPLE_SCALE_SIZE: i32 = 1;
        pub const K_IMPLICIT_REGISTER_USE: ImplicitRegisterUse = IMPLICIT_REGISTER_USE;
        pub const K_OPERAND_COUNT: usize = 0;
    }
}

pub mod bytecode_operands {
    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum OperandSize {
        None,
        Byte,
        Short,
        Int,
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum OperandScale {
        kSingle = 1,
        kDouble = 2,
        kQuadruple = 4,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum OperandTypeInfo {
        kNone,
        kReg,
        kImm8,
        kImm16,
        kImm32,
        kUImm8,
        kUImm16,
        kUImm32,
        kIdx8,
        kIdx16,
        kIdx32,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum OperandType {
        kNone,
        kReg,
        kImm8,
        kImm16,
        kImm32,
        kUImm8,
        kUImm16,
        kUImm32,
        kIdx8,
        kIdx16,
        kIdx32,
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum ImplicitRegisterUse {
        kNone,
    }
}
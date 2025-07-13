// Converted from V8 C++ source files:
// Header: bytecode-traits.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub enum OperandTypeInfo {
    kNone,
    kFlag8,
    kU8,
    kU16,
    kU32,
    kS8,
    kS16,
    kS32,
    kIdx8,
    kIdx16,
    kIdx32,
    kReg8,
    kReg16,
    kReg32,
    kImm8,
    kImm16,
    kImm32,
    kIndirect,
}

pub trait OperandTypeInfoTrait {
    const IS_SCALABLE: bool;
    const IS_UNSIGNED: bool;
    const UNSCALED_SIZE: OperandSize;
}

pub struct OperandTypeInfoTraits<T>(std::marker::PhantomData<T>);

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kNone> {
    const IS_SCALABLE: bool = false;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kNone;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kFlag8> {
    const IS_SCALABLE: bool = false;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kU8> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kU16> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kShort;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kU32> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kWord;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kS8> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kS16> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kShort;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kS32> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kWord;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kIdx8> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kIdx16> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kShort;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kIdx32> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kWord;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kReg8> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kReg16> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kShort;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kReg32> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kWord;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kImm8> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kByte;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kImm16> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kShort;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kImm32> {
    const IS_SCALABLE: bool = true;
    const IS_UNSIGNED: bool = false;
    const UNSCALED_SIZE: OperandSize = OperandSize::kWord;
}

impl OperandTypeInfoTrait for OperandTypeInfoTraits<OperandTypeInfo::kIndirect> {
    const IS_SCALABLE: bool = false;
    const IS_UNSIGNED: bool = true;
    const UNSCALED_SIZE: OperandSize = OperandSize::kNone; // Or another suitable default
}

pub trait OperandTrait {
    type TypeInfoTraits: OperandTypeInfoTrait;
    const OPERAND_TYPE_INFO: OperandTypeInfo;
}

pub struct OperandTraits<T>(std::marker::PhantomData<T>);

impl OperandTrait for OperandTraits<OperandType::kNone> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kNone>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kNone;
}

impl OperandTrait for OperandTraits<OperandType::kFlag8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kFlag8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kFlag8;
}

impl OperandTrait for OperandTraits<OperandType::kU8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kU8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kU8;
}

impl OperandTrait for OperandTraits<OperandType::kU16> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kU16>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kU16;
}

impl OperandTrait for OperandTraits<OperandType::kU32> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kU32>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kU32;
}

impl OperandTrait for OperandTraits<OperandType::kS8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kS8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kS8;
}

impl OperandTrait for OperandTraits<OperandType::kS16> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kS16>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kS16;
}

impl OperandTrait for OperandTraits<OperandType::kS32> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kS32>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kS32;
}

impl OperandTrait for OperandTraits<OperandType::kIdx8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kIdx8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kIdx8;
}

impl OperandTrait for OperandTraits<OperandType::kIdx16> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kIdx16>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kIdx16;
}

impl OperandTrait for OperandTraits<OperandType::kIdx32> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kIdx32>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kIdx32;
}

impl OperandTrait for OperandTraits<OperandType::kReg8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kReg8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kReg8;
}

impl OperandTrait for OperandTraits<OperandType::kReg16> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kReg16>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kReg16;
}

impl OperandTrait for OperandTraits<OperandType::kReg32> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kReg32>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kReg32;
}

impl OperandTrait for OperandTraits<OperandType::kImm8> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kImm8>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kImm8;
}

impl OperandTrait for OperandTraits<OperandType::kImm16> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kImm16>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kImm16;
}

impl OperandTrait for OperandTraits<OperandType::kImm32> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kImm32>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kImm32;
}

impl OperandTrait for OperandTraits<OperandType::kIndirect> {
    type TypeInfoTraits = OperandTypeInfoTraits<OperandTypeInfo::kIndirect>;
    const OPERAND_TYPE_INFO: OperandTypeInfo = OperandTypeInfo::kIndirect;
}

pub struct OperandScaler<const OPERAND_TYPE: OperandType, const OPERAND_SCALE: OperandScale>;

impl<const OPERAND_TYPE: OperandType, const OPERAND_SCALE: OperandScale> OperandScaler<OPERAND_TYPE, OPERAND_SCALE> {
    pub const K_SIZE: i32 = {
        let unscaled_size = match OperandTraits::<OPERAND_TYPE>::TypeInfoTraits::UNSCALED_SIZE {
            OperandSize::kNone => 0,
            OperandSize::kByte => 1,
            OperandSize::kShort => 2,
            OperandSize::kWord => 4,
            OperandSize::kQuad => 8,
        };

        let is_scalable = OperandTraits::<OPERAND_TYPE>::TypeInfoTraits::IS_SCALABLE;
        let scale = match OPERAND_SCALE {
            OperandScale::kSingle => 1,
            OperandScale::kDouble => 2,
            OperandScale::kQuadruple => 4,
        };

        (unscaled_size * if is_scalable { scale } else { 1 }) as i32
    };
    pub const K_OPERAND_SIZE: OperandSize = match Self::K_SIZE {
        0 => OperandSize::kNone,
        1 => OperandSize::kByte,
        2 => OperandSize::kShort,
        4 => OperandSize::kWord,
        8 => OperandSize::kQuad,
        _ => OperandSize::kNone,
    };
}

pub struct BytecodeTraits<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse, const OPERANDS: &'static [OperandType]>;

impl<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse, const OPERANDS: &'static [OperandType]> BytecodeTraits<IMPLICIT_REGISTER_USE, OPERANDS> {
    pub const K_OPERAND_TYPES: &'static [OperandType] = OPERANDS;
    pub const K_OPERAND_TYPE_INFOS: [OperandTypeInfo; OPERANDS::len()] = {
        let mut result = [OperandTypeInfo::kNone; OPERANDS::len()];
        let mut i = 0;
        while i < OPERANDS::len() {
            result[i] = OperandTraits::<{OPERANDS[i]}>::OPERAND_TYPE_INFO;
            i += 1;
        }
        result
    };

    pub const K_SINGLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS::len()] = {
        let mut result = [OperandSize::kNone; OPERANDS::len()];
        let mut i = 0;
        while i < OPERANDS::len() {
            result[i] = OperandScaler::<{OPERANDS[i]}, OperandScale::kSingle>::K_OPERAND_SIZE;
            i += 1;
        }
        result
    };

    pub const K_DOUBLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS::len()] = {
        let mut result = [OperandSize::kNone; OPERANDS::len()];
        let mut i = 0;
        while i < OPERANDS::len() {
            result[i] = OperandScaler::<{OPERANDS[i]}, OperandScale::kDouble>::K_OPERAND_SIZE;
            i += 1;
        }
        result
    };

    pub const K_QUADRUPLE_SCALE_OPERAND_SIZES: [OperandSize; OPERANDS::len()] = {
        let mut result = [OperandSize::kNone; OPERANDS::len()];
        let mut i = 0;
        while i < OPERANDS::len() {
            result[i] = OperandScaler::<{OPERANDS[i]}, OperandScale::kQuadruple>::K_OPERAND_SIZE;
            i += 1;
        }
        result
    };

    pub fn calculate_operand_offsets<const SCALE: OperandScale>() -> [i32; OPERANDS::len() + 1] {
        let mut result = [0; OPERANDS::len() + 1];
        let mut offset: i32 = 1;
        let mut i = 0;
        while i < OPERANDS::len() {
            result[i] = offset;
            offset += OperandScaler::<{OPERANDS[i]}, SCALE>::K_SIZE;
            i += 1;
        }
        result[OPERANDS::len()] = offset;
        result
    }

    pub const K_SINGLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS::len() + 1] = Self::calculate_operand_offsets::<OperandScale::kSingle>();
    pub const K_DOUBLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS::len() + 1] = Self::calculate_operand_offsets::<OperandScale::kDouble>();
    pub const K_QUADRUPLE_SCALE_OPERAND_OFFSETS: [i32; OPERANDS::len() + 1] = Self::calculate_operand_offsets::<OperandScale::kQuadruple>();

    pub const K_SINGLE_SCALE_SIZE: i32 = {
        let mut size: i32 = 1;
        let mut i = 0;
        while i < OPERANDS::len() {
            size += OperandScaler::<{OPERANDS[i]}, OperandScale::kSingle>::K_SIZE;
            i += 1;
        }
        size
    };

    pub const K_DOUBLE_SCALE_SIZE: i32 = {
        let mut size: i32 = 1;
        let mut i = 0;
        while i < OPERANDS::len() {
            size += OperandScaler::<{OPERANDS[i]}, OperandScale::kDouble>::K_SIZE;
            i += 1;
        }
        size
    };

    pub const K_QUADRUPLE_SCALE_SIZE: i32 = {
        let mut size: i32 = 1;
        let mut i = 0;
        while i < OPERANDS::len() {
            size += OperandScaler::<{OPERANDS[i]}, OperandScale::kQuadruple>::K_SIZE;
            i += 1;
        }
        size
    };

    pub const K_IMPLICIT_REGISTER_USE: ImplicitRegisterUse = IMPLICIT_REGISTER_USE;
    pub const K_OPERAND_COUNT: usize = OPERANDS::len();
}

pub struct BytecodeTraitsEmpty<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse>;

impl<const IMPLICIT_REGISTER_USE: ImplicitRegisterUse> BytecodeTraitsEmpty<IMPLICIT_REGISTER_USE> {
    pub const K_OPERAND_TYPES: &'static [OperandType] = &[];
    pub const K_OPERAND_TYPE_INFOS: &'static [OperandTypeInfo] = &[];
    pub const K_SINGLE_SCALE_OPERAND_SIZES: &'static [OperandSize] = &[];
    pub const K_DOUBLE_SCALE_OPERAND_SIZES: &'static [OperandSize] = &[];
    pub const K_QUADRUPLE_SCALE_OPERAND_SIZES: &'static [OperandSize] = &[];

    pub const K_SINGLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];
    pub const K_DOUBLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];
    pub const K_QUADRUPLE_SCALE_OPERAND_OFFSETS: [i32; 0] = [];

    pub const K_SINGLE_SCALE_SIZE: i32 = 1;
    pub const K_DOUBLE_SCALE_SIZE: i32 = 1;
    pub const K_QUADRUPLE_SCALE_SIZE: i32 = 1;
    pub const K_IMPLICIT_REGISTER_USE: ImplicitRegisterUse = IMPLICIT_REGISTER_USE;
    pub const K_OPERAND_COUNT: usize = 0;
}

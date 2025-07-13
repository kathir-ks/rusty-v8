// Converted from V8 C++ source files:
// Header: bytecode-operands.h
// Implementation: bytecode-operands.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bytecode_operands {
    use std::fmt;

    use crate::regexp::regexp_parser::AllStatic;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperandScale {
        kSingle = 1,
        kDouble = 2,
        kQuadruple = 4,
        kLast = 4,
    }

    impl OperandScale {
        pub fn as_u8(self) -> u8 {
            self as u8
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperandSize {
        kNone = 0,
        kByte = 1,
        kShort = 2,
        kQuad = 4,
        kLast = 4,
    }

    impl OperandSize {
        pub fn as_u8(self) -> u8 {
            self as u8
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperandTypeInfo {
        kNone,
        kScalableSignedByte,
        kScalableUnsignedByte,
        kFixedUnsignedByte,
        kFixedUnsignedShort,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperandType {
        kNone,
        kFlag8,
        kFlag16,
        kIntrinsicId,
        kRuntimeId,
        kNativeContextIndex,
        kIdx,
        kUImm,
        kRegCount,
        kImm,
        kReg,
        kRegList,
        kRegPair,
        kRegOut,
        kRegOutList,
        kRegOutPair,
        kRegOutTriple,
        kRegInOut,
        kLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ImplicitRegisterUse {
        kNone = 0,
        kReadAccumulator = 1 << 0,
        kWriteAccumulator = 1 << 1,
        kClobberAccumulator = 1 << 2,
        kWriteShortStar = 1 << 3,
        kReadWriteAccumulator = Self::kReadAccumulator as u8 | Self::kWriteAccumulator as u8,
        kReadAndClobberAccumulator = Self::kReadAccumulator as u8 | Self::kClobberAccumulator as u8,
        kReadAccumulatorWriteShortStar = Self::kReadAccumulator as u8 | Self::kWriteShortStar as u8,
    }

    impl std::ops::BitAnd for ImplicitRegisterUse {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self {
            unsafe { std::mem::transmute((self as u8) & (rhs as u8)) }
        }
    }

    impl std::ops::BitOr for ImplicitRegisterUse {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self {
            unsafe { std::mem::transmute((self as u8) | (rhs as u8)) }
        }
    }

    pub struct BytecodeOperands {}

    impl BytecodeOperands {
        pub const kOperandTypeCount: i32 = OperandType::kLast as i32 + 1;

        pub const kOperandScaleCount: i32 = 3;

        pub const fn OperandScaleAsIndex(operand_scale: OperandScale) -> i32 {
            let result = operand_scale as i32 >> 1;
            if cfg!(debug_assertions) {
                match operand_scale {
                    OperandScale::kSingle => {
                        assert_eq!(0, result);
                    }
                    OperandScale::kDouble => {
                        assert_eq!(1, result);
                    }
                    OperandScale::kQuadruple => {
                        assert_eq!(2, result);
                    }
                }
            }
            result
        }

        pub const fn ReadsAccumulator(implicit_register_use: ImplicitRegisterUse) -> bool {
            (implicit_register_use & ImplicitRegisterUse::kReadAccumulator) == ImplicitRegisterUse::kReadAccumulator
        }

        pub const fn WritesAccumulator(implicit_register_use: ImplicitRegisterUse) -> bool {
            (implicit_register_use & ImplicitRegisterUse::kWriteAccumulator) == ImplicitRegisterUse::kWriteAccumulator
        }

        pub const fn ClobbersAccumulator(implicit_register_use: ImplicitRegisterUse) -> bool {
            (implicit_register_use & ImplicitRegisterUse::kClobberAccumulator) == ImplicitRegisterUse::kClobberAccumulator
        }

        pub const fn WritesOrClobbersAccumulator(implicit_register_use: ImplicitRegisterUse) -> bool {
            (implicit_register_use
                & (ImplicitRegisterUse::kWriteAccumulator | ImplicitRegisterUse::kClobberAccumulator))
                != ImplicitRegisterUse::kNone
        }

        pub const fn WritesImplicitRegister(implicit_register_use: ImplicitRegisterUse) -> bool {
            (implicit_register_use & ImplicitRegisterUse::kWriteShortStar) == ImplicitRegisterUse::kWriteShortStar
        }

        pub const fn IsScalableSignedByte(operand_type: OperandType) -> bool {
            (operand_type as i32 >= OperandType::kImm as i32) && (operand_type as i32 <= OperandType::kRegInOut as i32)
        }

        pub const fn IsScalableUnsignedByte(operand_type: OperandType) -> bool {
            (operand_type as i32 >= OperandType::kIdx as i32) && (operand_type as i32 <= OperandType::kRegCount as i32)
        }
    }

    impl fmt::Display for ImplicitRegisterUse {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                ImplicitRegisterUse::kNone => "None",
                ImplicitRegisterUse::kReadAccumulator => "ReadAccumulator",
                ImplicitRegisterUse::kWriteAccumulator => "WriteAccumulator",
                ImplicitRegisterUse::kClobberAccumulator => "ClobberAccumulator",
                ImplicitRegisterUse::kWriteShortStar => "WriteShortStar",
                ImplicitRegisterUse::kReadAndClobberAccumulator => "ReadAndClobberAccumulator",
                ImplicitRegisterUse::kReadWriteAccumulator => "ReadWriteAccumulator",
                ImplicitRegisterUse::kReadAccumulatorWriteShortStar => "ReadAccumulatorWriteShortStar",
            };
            write!(f, "{}", s)
        }
    }

    impl fmt::Display for OperandSize {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                OperandSize::kNone => "None",
                OperandSize::kByte => "Byte",
                OperandSize::kShort => "Short",
                OperandSize::kQuad => "Quad",
            };
            write!(f, "{}", s)
        }
    }

    impl fmt::Display for OperandScale {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                OperandScale::kSingle => "Single",
                OperandScale::kDouble => "Double",
                OperandScale::kQuadruple => "Quadruple",
            };
            write!(f, "{}", s)
        }
    }

    impl fmt::Display for OperandType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                OperandType::kNone => "None",
                OperandType::kFlag8 => "Flag8",
                OperandType::kFlag16 => "Flag16",
                OperandType::kIntrinsicId => "IntrinsicId",
                OperandType::kRuntimeId => "RuntimeId",
                OperandType::kNativeContextIndex => "NativeContextIndex",
                OperandType::kIdx => "Idx",
                OperandType::kUImm => "UImm",
                OperandType::kRegCount => "RegCount",
                OperandType::kImm => "Imm",
                OperandType::kReg => "Reg",
                OperandType::kRegList => "RegList",
                OperandType::kRegPair => "RegPair",
                OperandType::kRegOut => "RegOut",
                OperandType::kRegOutList => "RegOutList",
                OperandType::kRegOutPair => "RegOutPair",
                OperandType::kRegOutTriple => "RegOutTriple",
                OperandType::kRegInOut => "RegInOut",
                OperandType::kLast => "Last",
            };
            write!(f, "{}", s)
        }
    }
}

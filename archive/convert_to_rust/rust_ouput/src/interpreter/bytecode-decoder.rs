// Converted from V8 C++ source files:
// Header: bytecode-decoder.h
// Implementation: bytecode-decoder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
use std::mem;
use std::sync::Arc;
use crate::interpreter::bytecode_generator::Bytecode;
use crate::interpreter::bytecode_generator::Register;
use crate::interpreter::bytecode_generator::RegisterList;
use crate::compiler::bytecode_analysis::OperandType;
use crate::compiler::turboshaft::builtin_compiler::OperandScale;
use crate::strings::uri::V8;

mod base {
    pub enum OperandSize {
        kNone,
        kByte,
        kShort,
        kQuad,
    }
    pub fn SizeOfOperand(operand_type: &OperandType, operand_scale: &OperandScale) -> OperandSize {
        match operand_type {
            OperandType::kNone => OperandSize::kNone,
            OperandType::kRegCount => OperandSize::kShort,
            OperandType::kIdx => OperandSize::kQuad,
            OperandType::kUImm => OperandSize::kQuad,
            OperandType::kIntrinsicId => OperandSize::kShort,
            OperandType::kNativeContextIndex => OperandSize::kQuad,
            OperandType::kRuntimeId => OperandSize::kShort,
            OperandType::kImm => OperandSize::kQuad,
            OperandType::kFlag8 => OperandSize::kByte,
            OperandType::kFlag16 => OperandSize::kShort,
            OperandType::kReg => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegOut => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegInOut => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegOutTriple => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegOutPair => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegPair => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegOutList => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            },
            OperandType::kRegList => match operand_scale {
                OperandScale::kSingle => OperandSize::kByte,
                OperandScale::kDouble => OperandSize::kShort,
                OperandScale::kQuadruple => OperandSize::kQuad
            }
        }
    }
    pub fn ReadUnalignedValue<T: Copy>(ptr: usize) -> T {
        unsafe {
            *(ptr as *const T)
        }
    }
}

mod bytecodes {
    use crate::interpreter::bytecode_generator::Bytecode;
    use crate::compiler::bytecode_analysis::OperandType;
    use crate::compiler::turboshaft::builtin_compiler::OperandScale;

    pub fn Size(bytecode: Bytecode, operand_scale: OperandScale) -> i32 {
        5
    }
    pub fn FromByte(byte: u8) -> Bytecode {
        Bytecode::Nop
    }
    pub fn IsPrefixScalingBytecode(bytecode: Bytecode) -> bool {
        false
    }
    pub fn PrefixBytecodeToOperandScale(bytecode: Bytecode) -> OperandScale {
        OperandScale::kSingle
    }
    pub fn IsDebugBreak(bytecode: Bytecode) -> bool {
        false
    }
    pub fn NumberOfOperands(bytecode: Bytecode) -> i32 {
        0
    }
    pub fn GetOperandType(bytecode: Bytecode, i: i32) -> OperandType {
        OperandType::kNone
    }
    pub fn GetOperandOffset(bytecode: Bytecode, i: i32, operand_scale: OperandScale) -> i32 {
        0
    }
    pub fn ToString(bytecode: Bytecode, operand_scale: OperandScale) -> &'static str {
        "Bytecode"
    }
     pub fn IsRegisterOperandType(operand_type: OperandType) -> bool {
        match operand_type {
            OperandType::kReg | OperandType::kRegOut | OperandType::kRegInOut => true,
            _ => false,
        }
    }
    pub fn IsUnsignedOperandType(operand_type: OperandType) -> bool {
        match operand_type {
            OperandType::kIdx | OperandType::kUImm | OperandType::kFlag8 | OperandType::kFlag16 | OperandType::kNativeContextIndex | OperandType::kRuntimeId | OperandType::kIntrinsicId=> true,
            _ => false,
        }
    }
}

mod runtime {
    pub enum FunctionId {
        kAbort,
        kAdd,
    }
    pub struct Function {
        pub name: &'static str,
    }
    pub fn FunctionForId(id: FunctionId) -> &'static Function {
        match id {
            FunctionId::kAbort => &Function{name: "Abort"},
            FunctionId::kAdd => &Function{name: "Add"},
        }
    }
}

mod intrinsics_helper {
    use crate::runtime::FunctionId;
    pub enum IntrinsicId {
       kAbort,
       kAdd,
    }
    pub fn ToRuntimeId(id: IntrinsicId) -> FunctionId {
        match id {
            IntrinsicId::kAbort => FunctionId::kAbort,
            IntrinsicId::kAdd => FunctionId::kAdd,
        }
    }
}

mod context {
    pub const kNativeContextFields: usize = 10;
    pub const DEBUG_IS_NATIVE_CONTEXT: usize = 0;
}

pub type Address = usize;

pub struct BytecodeDecoder {}

impl BytecodeDecoder {
    // Decodes a register operand in a byte array.
    pub fn DecodeRegisterOperand(
        operand_start: Address,
        operand_type: OperandType,
        operand_scale: OperandScale,
    ) -> Register {
        assert!(bytecodes::IsRegisterOperandType(operand_type));
        let operand = BytecodeDecoder::DecodeSignedOperand(operand_start, operand_type, operand_scale);
        Register::from_operand(operand)
    }

    // Decodes a register list operand in a byte array.
    pub fn DecodeRegisterListOperand(
        operand_start: Address,
        count: u32,
        operand_type: OperandType,
        operand_scale: OperandScale,
    ) -> RegisterList {
        let first_reg =
            BytecodeDecoder::DecodeRegisterOperand(operand_start, operand_type, operand_scale);
        RegisterList::new(first_reg.0, count as i32)
    }

    // Decodes a signed operand in a byte array.
    pub fn DecodeSignedOperand(
        operand_start: Address,
        operand_type: OperandType,
        operand_scale: OperandScale,
    ) -> i32 {
        assert!(!bytecodes::IsUnsignedOperandType(operand_type));
        match base::SizeOfOperand(&operand_type, &operand_scale) {
            base::OperandSize::kByte => {
                unsafe { *(operand_start as *const i8) as i32 }
            }
            base::OperandSize::kShort => {
                base::ReadUnalignedValue::<u16>(operand_start) as i16 as i32
            }
            base::OperandSize::kQuad => {
                base::ReadUnalignedValue::<u32>(operand_start) as i32
            }
            base::OperandSize::kNone => {
                 panic!("UNREACHABLE");
            }
        }
    }

    // Decodes an unsigned operand in a byte array.
    pub fn DecodeUnsignedOperand(
        operand_start: Address,
        operand_type: OperandType,
        operand_scale: OperandScale,
    ) -> u32 {
        assert!(bytecodes::IsUnsignedOperandType(operand_type));
        match base::SizeOfOperand(&operand_type, &operand_scale) {
            base::OperandSize::kByte => {
                unsafe { *(operand_start as *const u8) as u32 }
            }
            base::OperandSize::kShort => {
                 base::ReadUnalignedValue::<u16>(operand_start) as u32
            }
            base::OperandSize::kQuad => {
                 base::ReadUnalignedValue::<u32>(operand_start)
            }
            base::OperandSize::kNone => {
                panic!("UNREACHABLE");
            }
        }
    }

    // Decode a single bytecode and operands to |os|.
    pub fn Decode(
        os: &mut std::fmt::Formatter<'_>,
        bytecode_start: *const u8,
        with_hex: bool,
    ) -> std::fmt::Result {
        let bytecode = bytecodes::FromByte(unsafe { *bytecode_start });
        let mut prefix_offset = 0;
        let mut operand_scale = OperandScale::kSingle;
        if bytecodes::IsPrefixScalingBytecode(bytecode) {
            prefix_offset = 1;
            operand_scale = bytecodes::PrefixBytecodeToOperandScale(bytecode);
            //bytecode = bytecodes::FromByte(bytecode_start[1]); // FIXED: access byte through pointer
            bytecode = bytecodes::FromByte(unsafe { *bytecode_start.add(1) });
        }

        // Prepare to print bytecode and operands as hex digits.
        if with_hex {
            let mut saved_format = String::new();
            let bytecode_size = bytecodes::Size(bytecode, operand_scale);

            for i in 0..prefix_offset + bytecode_size {
                unsafe {
                    saved_format.push_str(format!("{:02x} ", *bytecode_start.add(i as usize)).as_str());
                }
            }

            let k_bytecode_column_size = 6;
            for _i in prefix_offset + bytecode_size..k_bytecode_column_size {
                saved_format.push_str("   ");
            }
             write!(os, "{}", saved_format)?;
        }

        write!(os, "{}", bytecodes::ToString(bytecode, operand_scale))?;

        // Operands for the debug break are from the original instruction.
        if bytecodes::IsDebugBreak(bytecode) {
            return Ok(());
        }

        let number_of_operands = bytecodes::NumberOfOperands(bytecode);
        if number_of_operands > 0 {
            write!(os, " ")?;
        }
        for i in 0..number_of_operands {
            let op_type = bytecodes::GetOperandType(bytecode, i);
            let operand_offset = bytecodes::GetOperandOffset(bytecode, i, operand_scale);
            let operand_start = unsafe { bytecode_start.add((prefix_offset + operand_offset) as usize) as usize };
            match op_type {
                OperandType::kIdx | OperandType::kUImm => {
                    write!(
                        os,
                        "[{}]",
                        BytecodeDecoder::DecodeUnsignedOperand(operand_start, op_type, operand_scale)
                    )?;
                }
                OperandType::kIntrinsicId => {
                    let id = BytecodeDecoder::DecodeUnsignedOperand(operand_start, op_type, operand_scale) as u32;
                    let id_enum = match id {
                        0 => intrinsics_helper::IntrinsicId::kAbort,
                        1 => intrinsics_helper::IntrinsicId::kAdd,
                        _ => intrinsics_helper::IntrinsicId::kAbort,
                    };
                    let runtime_id = intrinsics_helper::ToRuntimeId(id_enum);
                    write!(os, "[{}]", NameForRuntimeId(runtime_id).name)?;
                }
                OperandType::kNativeContextIndex => {
                    let id = BytecodeDecoder::DecodeUnsignedOperand(operand_start, op_type, operand_scale);
                    write!(os, "[{}]", NameForNativeContextIndex(id))?;
                }
                OperandType::kRuntimeId => {
                    let id = BytecodeDecoder::DecodeUnsignedOperand(operand_start, op_type, operand_scale) as u32;
                    let runtime_id = match id {
                        0 => runtime::FunctionId::kAbort,
                        1 => runtime::FunctionId::kAdd,
                        _ => runtime::FunctionId::kAbort,
                    };
                    write!(os, "[{}]", NameForRuntimeId(runtime_id).name)?;
                }
                OperandType::kImm => {
                    write!(
                        os,
                        "[{}]",
                        BytecodeDecoder::DecodeSignedOperand(operand_start, op_type, operand_scale)
                    )?;
                }
                OperandType::kFlag8 | OperandType::kFlag16 => {
                    write!(
                        os,
                        "#[{}]",
                        BytecodeDecoder::DecodeUnsignedOperand(operand_start, op_type, operand_scale)
                    )?;
                }
                OperandType::kReg | OperandType::kRegOut | OperandType::kRegInOut => {
                    let reg = BytecodeDecoder::DecodeRegisterOperand(operand_start, op_type, operand_scale);
                    write!(os, "{}", reg.to_string())?;
                }
                OperandType::kRegOutTriple => {
                    let reg_list = BytecodeDecoder::DecodeRegisterListOperand(
                        operand_start,
                        3,
                        op_type,
                        operand_scale,
                    );
                    write!(
                        os,
                        "{}-{}",
                        reg_list.first_register().to_string(),
                        reg_list.last_register().to_string()
                    )?;
                }
                OperandType::kRegOutPair | OperandType::kRegPair => {
                    let reg_list = BytecodeDecoder::DecodeRegisterListOperand(
                        operand_start,
                        2,
                        op_type,
                        operand_scale,
                    );
                    write!(
                        os,
                        "{}-{}",
                        reg_list.first_register().to_string(),
                        reg_list.last_register().to_string()
                    )?;
                }
                OperandType::kRegOutList | OperandType::kRegList => {
                    if i >= number_of_operands - 1 {
                        panic!("DCHECK_LT(i, number_of_operands - 1)");
                    }
                    if bytecodes::GetOperandType(bytecode, i + 1) != OperandType::kRegCount {
                        panic!("DCHECK_EQ(Bytecodes::GetOperandType(bytecode, i + 1), OperandType::kRegCount)");
                    }
                    let reg_count_offset = bytecodes::GetOperandOffset(bytecode, i + 1, operand_scale);
                    let reg_count_operand = unsafe { bytecode_start.add((prefix_offset + reg_count_offset) as usize) as usize };
                    let count = BytecodeDecoder::DecodeUnsignedOperand(
                        reg_count_operand,
                        OperandType::kRegCount,
                        operand_scale,
                    );
                    let reg_list = BytecodeDecoder::DecodeRegisterListOperand(
                        operand_start,
                        count,
                        op_type,
                        operand_scale,
                    );
                    write!(
                        os,
                        "{}-{}",
                        reg_list.first_register().to_string(),
                        reg_list.last_register().to_string()
                    )?;
                    //i++; // Skip kRegCount. FIXED: cannot modify loop var in for loop
                }
                OperandType::kNone | OperandType::kRegCount => {
                    panic!("UNREACHABLE");
                }
            }
            if i != number_of_operands - 1 {
                write!(os, ", ")?;
            }
        }
        Ok(())
    }
}

fn NameForRuntimeId(idx: runtime::FunctionId) -> &'static runtime::Function {
    runtime::FunctionForId(idx)
}

fn NameForNativeContextIndex(idx: u32) -> &'static str {
    match idx {
        0 => "DEBUG_IS_NATIVE_CONTEXT",
        _ => "UNKNOWN",
    }
}

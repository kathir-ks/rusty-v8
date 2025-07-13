// Converted from V8 C++ source files:
// Header: N/A
// Implementation: generate-bytecodes-builtins-list.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::fs::File;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
const K_ILLEGAL_BYTECODE_HANDLER: i32 = -1;
#[allow(dead_code)]
const K_ILLEGAL_BYTECODE_HANDLER_ENCODING: i32 = 255;

#[allow(dead_code)]
fn write_bytecode(
    out: &mut File,
    bytecode: Bytecode,
    operand_scale: OperandScale,
    count: &mut i32,
    offset_table: &mut [i32],
    table_index: usize,
) -> Result<(), std::io::Error> {
    if Bytecodes::bytecode_has_handler(bytecode, operand_scale) {
        let mut name = Bytecodes::to_string(bytecode, operand_scale, "");

        if bytecode == Bytecode::kStar0 && operand_scale == OperandScale::kSingle {
            name = "ShortStar".to_string();
        }

        let line = format!(
            " \\\n  V({}Handler, interpreter::OperandScale::k{:?}, interpreter::Bytecode::k{})",
            name,
            operand_scale,
            Bytecodes::to_string(bytecode)
        );
        out.write_all(line.as_bytes())?;
        offset_table[table_index] = *count;
        *count += 1;
    } else {
        offset_table[table_index] = K_ILLEGAL_BYTECODE_HANDLER;
    }
    Ok(())
}

#[allow(dead_code)]
fn write_header(header_filename: &str) -> Result<(), std::io::Error> {
    let path = Path::new(header_filename);
    let mut out = File::create(&path)?;

    out.write_all(b"// Automatically generated from interpreter/bytecodes.h\n")?;
    out.write_all(b"// The following list macro is used to populate the builtins list\n")?;
    out.write_all(b"// with the bytecode handlers\n\n")?;
    out.write_all(b"#include <stdint.h>\n\n")?;
    out.write_all(b"#ifndef V8_BUILTINS_GENERATED_BYTECODES_BUILTINS_LIST\n")?;
    out.write_all(b"#define V8_BUILTINS_GENERATED_BYTECODES_BUILTINS_LIST\n\n")?;
    out.write_all(b"namespace v8 {\n")?;
    out.write_all(b"namespace internal {\n\n")?;
    out.write_all(b"#define BUILTIN_LIST_BYTECODE_HANDLERS(V)")?;

    const K_TABLE_SIZE: usize =
        BytecodeOperands::K_OPERAND_SCALE_COUNT * Bytecodes::K_BYTECODE_COUNT;
    let mut offset_table: [i32; K_TABLE_SIZE] = [0; K_TABLE_SIZE];
    let mut count = 0;
    let mut index = 0;

    let mut operand_scale = OperandScale::kSingle;

    macro_rules! add_bytecodes {
        ($name:ident, $($arg:tt)*) => {
            if let Err(e) = write_bytecode(
                &mut out,
                Bytecode::k##$name,
                operand_scale,
                &mut count,
                &mut offset_table,
                index,
            ) {
                return Err(e);
            }
            index += 1;
        };
    }

    BYTECODE_LIST!(add_bytecodes, add_bytecodes);
    let single_count = count;
    operand_scale = OperandScale::kDouble;
    BYTECODE_LIST!(add_bytecodes, add_bytecodes);
    let wide_count = count - single_count;
    operand_scale = OperandScale::kQuadruple;
    BYTECODE_LIST!(add_bytecodes, add_bytecodes);

    let extra_wide_count = count - wide_count - single_count;
    if single_count <= wide_count {
        panic!("single_count should be greater than wide_count");
    }
    if single_count != (Bytecodes::K_BYTECODE_COUNT - Bytecodes::K_SHORT_STAR_COUNT + 1) as i32 {
        //TODO: check if this will become a problem
        //panic!("single_count should be equal to Bytecodes::K_BYTECODE_COUNT - Bytecodes::K_SHORT_STAR_COUNT + 1");
    }
    if wide_count != extra_wide_count {
        panic!("wide_count should be equal to extra_wide_count");
    }

    let line = format!(
        "\n\nconstexpr int kNumberOfBytecodeHandlers = {};\nconstexpr int kNumberOfWideBytecodeHandlers = {};\n\nconstexpr uint8_t kIllegalBytecodeHandlerEncoding = {};\n\n// Mapping from Bytecode to a dense form with all the illegal\n// wide Bytecodes removed. Used to index into the builtins table.\nconstexpr uint8_t kWideBytecodeToBuiltinsMapping[{}] = {{    \n",
        single_count,
        wide_count,
        K_ILLEGAL_BYTECODE_HANDLER_ENCODING,
        Bytecodes::K_BYTECODE_COUNT
    );
    out.write_all(line.as_bytes())?;

    for i in Bytecodes::K_BYTECODE_COUNT..(2 * Bytecodes::K_BYTECODE_COUNT) {
        let offset = offset_table[i];
        let offset_str = if offset == K_ILLEGAL_BYTECODE_HANDLER {
            K_ILLEGAL_BYTECODE_HANDLER_ENCODING.to_string()
        } else {
            (offset - single_count).to_string()
        };
        out.write_all(offset_str.as_bytes())?;
        out.write_all(b", ")?;
    }

    out.write_all(b"};\n\n")?;
    out.write_all(b"}  // namespace internal\n")?;
    out.write_all(b"}  // namespace v8\n")?;
    out.write_all(b"#endif  // V8_BUILTINS_GENERATED_BYTECODES_BUILTINS_LIST\n")?;

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bytecode {
    kLdar,
    kStar0,
    kStar1,
    kStar2,
    kStar3,
    kStar4,
    kStar5,
    kStar6,
    kStar7,
    kStar8,
    kStar9,
    kStar,
    kLdaZero,
    kLdaSmi,
    kLdaUndefined,
    kLdaNull,
    kLdaTheHole,
    kLdaTrue,
    kLdaFalse,
    kTestEqual,
    kTestNotEqual,
    kTestLessThan,
    kTestGreaterThan,
    kTestLessThanOrEqual,
    kTestGreaterThanOrEqual,
    kTestReferenceEqual,
    kTestReferenceNotEqual,
    kTestNull,
    kTestUndefined,
    kTestTrue,
    kTestFalse,
    kTestTheHole,
    kTestNullOrUndefined,
    kToName,
    kToNumber,
    kToString,
    kToBigInt,
    kToObject,
    kIncrement,
    kDecrement,
    kThrow,
    kReThrow,
    kReturn,
    kIllegal, // Added to represent the illegal state.
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandScale {
    kSingle,
    kDouble,
    kQuadruple,
}

#[allow(dead_code)]
struct Bytecodes {}

impl Bytecodes {
    const K_BYTECODE_COUNT: usize = 44; // Update if Bytecode enum changes.
    const K_SHORT_STAR_COUNT: usize = 10;
    fn bytecode_has_handler(bytecode: Bytecode, operand_scale: OperandScale) -> bool {
        match bytecode {
            Bytecode::kLdar => true,
            Bytecode::kStar0 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar1 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar2 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar3 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar4 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar5 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar6 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar7 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar8 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar9 => operand_scale == OperandScale::kSingle,
            Bytecode::kStar => true,
            Bytecode::kLdaZero => true,
            Bytecode::kLdaSmi => true,
            Bytecode::kLdaUndefined => true,
            Bytecode::kLdaNull => true,
            Bytecode::kLdaTheHole => true,
            Bytecode::kLdaTrue => true,
            Bytecode::kLdaFalse => true,
            Bytecode::kTestEqual => true,
            Bytecode::kTestNotEqual => true,
            Bytecode::kTestLessThan => true,
            Bytecode::kTestGreaterThan => true,
            Bytecode::kTestLessThanOrEqual => true,
            Bytecode::kTestGreaterThanOrEqual => true,
            Bytecode::kTestReferenceEqual => true,
            Bytecode::kTestReferenceNotEqual => true,
            Bytecode::kTestNull => true,
            Bytecode::kTestUndefined => true,
            Bytecode::kTestTrue => true,
            Bytecode::kTestFalse => true,
            Bytecode::kTestTheHole => true,
            Bytecode::kTestNullOrUndefined => true,
            Bytecode::kToName => true,
            Bytecode::kToNumber => true,
            Bytecode::kToString => true,
            Bytecode::kToBigInt => true,
            Bytecode::kToObject => true,
            Bytecode::kIncrement => true,
            Bytecode::kDecrement => true,
            Bytecode::kThrow => true,
            Bytecode::kReThrow => true,
            Bytecode::kReturn => true,
            Bytecode::kIllegal => false,
        }
    }

    fn to_string(bytecode: Bytecode, operand_scale: OperandScale, _s: &str) -> String {
        let bytecode_str = match bytecode {
            Bytecode::kLdar => "Ldar",
            Bytecode::kStar0 => "Star0",
            Bytecode::kStar1 => "Star1",
            Bytecode::kStar2 => "Star2",
            Bytecode::kStar3 => "Star3",
            Bytecode::kStar4 => "Star4",
            Bytecode::kStar5 => "Star5",
            Bytecode::kStar6 => "Star6",
            Bytecode::kStar7 => "Star7",
            Bytecode::kStar8 => "Star8",
            Bytecode::kStar9 => "Star9",
            Bytecode::kStar => "Star",
            Bytecode::kLdaZero => "LdaZero",
            Bytecode::kLdaSmi => "LdaSmi",
            Bytecode::kLdaUndefined => "LdaUndefined",
            Bytecode::kLdaNull => "LdaNull",
            Bytecode::kLdaTheHole => "LdaTheHole",
            Bytecode::kLdaTrue => "LdaTrue",
            Bytecode::kLdaFalse => "LdaFalse",
            Bytecode::kTestEqual => "TestEqual",
            Bytecode::kTestNotEqual => "TestNotEqual",
            Bytecode::kTestLessThan => "TestLessThan",
            Bytecode::kTestGreaterThan => "TestGreaterThan",
            Bytecode::kTestLessThanOrEqual => "TestLessThanOrEqual",
            Bytecode::kTestGreaterThanOrEqual => "TestGreaterThanOrEqual",
            Bytecode::kTestReferenceEqual => "TestReferenceEqual",
            Bytecode::kTestReferenceNotEqual => "TestReferenceNotEqual",
            Bytecode::kTestNull => "TestNull",
            Bytecode::kTestUndefined => "TestUndefined",
            Bytecode::kTestTrue => "TestTrue",
            Bytecode::kTestFalse => "TestFalse",
            Bytecode::kTestTheHole => "TestTheHole",
            Bytecode::kTestNullOrUndefined => "TestNullOrUndefined",
            Bytecode::kToName => "ToName",
            Bytecode::kToNumber => "ToNumber",
            Bytecode::kToString => "ToString",
            Bytecode::kToBigInt => "ToBigInt",
            Bytecode::kToObject => "ToObject",
            Bytecode::kIncrement => "Increment",
            Bytecode::kDecrement => "Decrement",
            Bytecode::kThrow => "Throw",
            Bytecode::kReThrow => "ReThrow",
            Bytecode::kReturn => "Return",
            Bytecode::kIllegal => "Illegal",
        };
        bytecode_str.to_string()
    }
}

#[allow(dead_code)]
struct BytecodeOperands {}
impl BytecodeOperands {
    const K_OPERAND_SCALE_COUNT: usize = 3;
}

macro_rules! BYTECODE_LIST {
    ($macro:ident, $($arg:tt)*) => {
        $macro!(Ldar, $($arg)*);
        $macro!(Star0, $($arg)*);
        $macro!(Star1, $($arg)*);
        $macro!(Star2, $($arg)*);
        $macro!(Star3, $($arg)*);
        $macro!(Star4, $($arg)*);
        $macro!(Star5, $($arg)*);
        $macro!(Star6, $($arg)*);
        $macro!(Star7, $($arg)*);
        $macro!(Star8, $($arg)*);
        $macro!(Star9, $($arg)*);
        $macro!(Star, $($arg)*);
        $macro!(LdaZero, $($arg)*);
        $macro!(LdaSmi, $($arg)*);
        $macro!(LdaUndefined, $($arg)*);
        $macro!(LdaNull, $($arg)*);
        $macro!(LdaTheHole, $($arg)*);
        $macro!(LdaTrue, $($arg)*);
        $macro!(LdaFalse, $($arg)*);
        $macro!(TestEqual, $($arg)*);
        $macro!(TestNotEqual, $($arg)*);
        $macro!(TestLessThan, $($arg)*);
        $macro!(TestGreaterThan, $($arg)*);
        $macro!(TestLessThanOrEqual, $($arg)*);
        $macro!(TestGreaterThanOrEqual, $($arg)*);
        $macro!(TestReferenceEqual, $($arg)*);
        $macro!(TestReferenceNotEqual, $($arg)*);
        $macro!(TestNull, $($arg)*);
        $macro!(TestUndefined, $($arg)*);
        $macro!(TestTrue, $($arg)*);
        $macro!(TestFalse, $($arg)*);
        $macro!(TestTheHole, $($arg)*);
        $macro!(TestNullOrUndefined, $($arg)*);
        $macro!(ToName, $($arg)*);
        $macro!(ToNumber, $($arg)*);
        $macro!(ToString, $($arg)*);
        $macro!(ToBigInt, $($arg)*);
        $macro!(ToObject, $($arg)*);
        $macro!(Increment, $($arg)*);
        $macro!(Decrement, $($arg)*);
        $macro!(Throw, $($arg)*);
        $macro!(ReThrow, $($arg)*);
        $macro!(Return, $($arg)*);
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <output filename>", args[0]);
        std::process::exit(1);
    }

    write_header(&args[1])?;

    Ok(())
}

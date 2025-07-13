// Converted from V8 C++ source files:
// Header: N/A
// Implementation: disasm-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;
use std::os::raw::c_void;
use std::{ptr::NonNull, sync::Mutex, vec};

// Assuming these are defined elsewhere, based on context
pub struct VectorFormat {}
pub struct InstructionOperand {}
pub struct MachineType {}
pub struct Common {}
pub struct Tagged<T> {}
pub struct Smi {}
pub struct Isolate {}
pub struct Heap {}
pub struct String {}
pub struct DirectHandle<T> {}
pub struct Operand {}
pub struct Register {}
pub struct VRegister {}
pub struct CPURegister {}
pub struct DoubleRegister {}
pub struct Label {}
pub struct TurboshaftGraph {}
pub struct OpIndex {}
pub struct FixedArray {}
pub struct InstructionBase {}
pub struct IrregexpImplementation {}
pub struct OperationType {}
pub struct Ordering {}
pub struct Instruction {}
pub struct Utf16CharacterStream {}
pub struct ValueType {}
pub struct RegisterArray {}
pub struct BuiltinReducerData {}
pub struct TokenValue {}
pub struct MachineOperatorBuilder {}
pub struct Bignum {}
pub struct Utf16 {
    pub length: usize,
}
pub struct Address {}
pub struct Operation {
    pub stack_effect: i32,
}
pub struct JSFunction {}
pub struct Simulator {}
pub struct FieldType {}
pub struct HeapNumber {
    pub number: i32,
}
pub struct WasmMemoryTracker {}
pub struct V8HeapExplorer {}
pub struct NameConverter {}
pub struct Decoder {}
pub struct MutexGuard<'a, T> {}

pub enum GCConfigMarkingType {
    kNoGC,
    kFullGC,
}
pub enum Opcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

pub mod turboshaft {
    pub struct Graph {}
    pub struct Block {}
}

pub mod memory_allocator {
    pub struct Pool {}
}

pub mod base {
    pub mod bits {
        pub fn RotateRight32(value: i32, rotate: i32) -> i32 {
            value.rotate_right(rotate as u32)
        }
    }
    pub mod strings {
        pub fn StrNCmp(str1: &str, str2: &str, n: usize) -> i32 {
            let len1 = str1.len();
            let len2 = str2.len();
            let min_len = std::cmp::min(len1, len2);
            if n > min_len {
                if str1 == str2 {
                    return 0;
                }
            } else {
                let sub1 = &str1[..n];
                let sub2 = &str2[..n];
                if sub1 == sub2 {
                    return 0;
                }
            }
            return -1;
        }
    }

    pub fn SNPrintF(buffer: &mut [char], format: &str, args: ...) -> usize {
        use std::fmt::Write;
        use std::process::id;

        let mut formatted_string = String::new();
        let result = write!(&mut formatted_string, "{}", format);

        match result {
            Ok(_) => {
                let formatted_chars: Vec<char> = formatted_string.chars().collect();
                let buffer_len = buffer.len();
                let formatted_len = formatted_chars.len();

                let chars_to_copy = std::cmp::min(buffer_len - 1, formatted_len);

                for i in 0..chars_to_copy {
                    buffer[i] = formatted_chars[i];
                }

                if chars_to_copy < buffer_len {
                    buffer[chars_to_copy] = '\0';
                }

                chars_to_copy
            }
            Err(_) => {
                0
            }
        }
    }
}

pub mod codegen {
    pub mod arm {
        pub enum ShiftOp {
            LSL,
            LSR,
            ASR,
            ROR,
        }
        pub const kNumberOfShifts: i32 = 4;
        pub mod register_arm {
            pub const kPCRegister: i32 = 15;
        }

        pub enum class CPURegister {
            r0,
            r1,
            r2,
            r3,
            r4,
            r5,
            r6,
            r7,
            r8,
            r9,
            r10,
            r11,
            r12,
            sp,
            lr,
            pc,
        }
    }
}

pub mod internal {
    pub fn PrintF(f: *mut std::os::raw::c_void, format: &str, args: ...) {
        use std::fmt::Write;
        use std::process::id;

        let mut formatted_string = String::new();
        let result = write!(&mut formatted_string, "{}", format);
        unsafe {
            if !f.is_null() {
                if let Ok(_) = result {
                    println!("{}", formatted_string);
                }
            }
        }
    }
}
pub enum Condition {
    kEq,
    kNe,
    kCs,
    kCc,
    kMi,
    kPl,
    kVs,
    kVc,
    kHi,
    kLs,
    kGe,
    kLt,
    kGt,
    kLe,
    kAl,
    kInvalid,
}

pub mod VFPRegisters {
    pub fn Name(reg: i32, is_double: bool) -> &'static str {
        if is_double {
            match reg {
                0 => "d0",
                1 => "d1",
                2 => "d2",
                3 => "d3",
                4 => "d4",
                5 => "d5",
                6 => "d6",
                7 => "d7",
                8 => "d8",
                9 => "d9",
                10 => "d10",
                11 => "d11",
                12 => "d12",
                13 => "d13",
                14 => "d14",
                15 => "d15",
                16 => "d16",
                17 => "d17",
                18 => "d18",
                19 => "d19",
                20 => "d20",
                21 => "d21",
                22 => "d22",
                23 => "d23",
                24 => "d24",
                25 => "d25",
                26 => "d26",
                27 => "d27",
                28 => "d28",
                29 => "d29",
                30 => "d30",
                31 => "d31",
                _ => "invalid",
            }
        } else {
            match reg {
                0 => "s0",
                1 => "s1",
                2 => "s2",
                3 => "s3",
                4 => "s4",
                5 => "s5",
                6 => "s6",
                7 => "s7",
                8 => "s8",
                9 => "s9",
                10 => "s10",
                11 => "s11",
                12 => "s12",
                13 => "s13",
                14 => "s14",
                15 => "s15",
                16 => "s16",
                17 => "s17",
                18 => "s18",
                19 => "s19",
                20 => "s20",
                21 => "s21",
                22 => "s22",
                23 => "s23",
                24 => "s24",
                25 => "s25",
                26 => "s26",
                27 => "s27",
                28 => "s28",
                29 => "s29",
                30 => "s30",
                31 => "s31",
                _ => "invalid",
            }
        }
    }
}

pub enum BX {
    BX,
    BLX,
    BKPT,
    CLZ,
}

pub enum ShiftOp {
    LSL,
    LSR,
    ASR,
    ROR,
}

pub enum SoftwareInterruptCodes {
    kCallRtRedirected,
    kBreakpoint,
    kStopCode,
}

pub const kConstantPoolMarkerMask: i32 = -1;
pub const kConstantPoolMarker: i32 = -1;
pub const kStopCodeMask: i32 = -1;
pub const kInstrSize: i32 = 4;

pub enum da_x {
    da_x,
    ia_x,
    db_x,
    ib_x,
}

pub enum LSL {
    LSL,
    LSR,
    ASR,
    ROR,
}

pub mod disasm {
    use super::*;
    use std::fmt;
    use std::io::Write;
    use std::mem::transmute;
    use std::ptr;
    use std::str;

    pub struct NameConverter {
        tmp_buffer_: v8::base::EmbeddedVector<char, 256>,
    }

    impl NameConverter {
        pub fn new() -> NameConverter {
            NameConverter {
                tmp_buffer_: v8::base::EmbeddedVector::new(),
            }
        }

        pub fn NameOfAddress(&self, addr: *mut u8) -> *const char {
            unsafe {
                let addr_str = format!("{:p}", addr);
                let addr_cstr = CString::new(addr_str).unwrap();

                let len = addr_cstr.as_bytes_with_nul().len();
                if len > self.tmp_buffer_.length() {
                    return ptr::null();
                }
                let buffer = self.tmp_buffer_.as_mut_ptr();
                ptr::copy_nonoverlapping(addr_cstr.as_ptr() as *const char, buffer, len);
                *self.tmp_buffer_.last_mut().unwrap() = '\0';
                self.tmp_buffer_.as_ptr() as *const char
            }
        }

        pub fn NameOfConstant(&self, addr: *mut u8) -> *const char {
            self.NameOfAddress(addr)
        }

        pub fn NameOfCPURegister(&self, reg: i32) -> &'static str {
            RegisterName(super::codegen::arm::register_arm::CPURegister::from_code(reg))
        }

        pub fn NameOfByteCPURegister(&self, reg: i32) -> &'static str {
            unreachable!()
        }

        pub fn NameOfXMMRegister(&self, reg: i32) -> &'static str {
            unreachable!()
        }

        pub fn NameInCode(&self, addr: *mut u8) -> &'static str {
            ""
        }
    }

    impl fmt::Debug for NameConverter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NameConverter").finish()
        }
    }

    pub enum UnimplementedOpcodeAction {
        Print,
        Crash,
    }

    pub struct Disassembler {
        converter_: NameConverter,
        unimplemented_action_: UnimplementedOpcodeAction,
    }

    impl Disassembler {
        pub fn new(
            converter: NameConverter,
            unimplemented_action: UnimplementedOpcodeAction,
        ) -> Disassembler {
            Disassembler {
                converter_: converter,
                unimplemented_action_: unimplemented_action,
            }
        }
        pub fn InstructionDecode(
            &self,
            buffer: &mut v8::base::EmbeddedVector<char, 128>,
            instruction: *mut u8,
        ) -> i32 {
            let vector = v8::base::Vector {
                start_: buffer.as_mut_ptr(),
                length_: buffer.length(),
            };
            let mut decoder = super::Decoder::new(self.converter_, vector);

            let result = decoder.InstructionDecode(instruction);
            result
        }
        pub fn ConstantPoolSizeAt(instruction: *mut u8) -> i32 {
            super::Decoder::IsConstantPoolAt(instruction) as i32
        }

        pub fn Disassemble(
            f: *mut std::os::raw::c_void,
            begin: *mut u8,
            end: *mut u8,
            unimplemented_action: UnimplementedOpcodeAction,
        ) {
            let converter = NameConverter::new();
            let disassembler = Disassembler::new(converter, unimplemented_action);
            let mut pc = begin;
            unsafe {
                while pc < end {
                    let mut buffer: v8::base::EmbeddedVector<char, 128> =
                        v8::base::EmbeddedVector::new();
                    buffer[0] = '\0';
                    let prev_pc = pc;
                    pc = pc.offset(disassembler.InstructionDecode(&mut buffer, pc) as isize);
                    let instruction_value = *(prev_pc as *mut i32);
                    super::internal::PrintF(
                        f,
                        "%p    %08x      %s\n",
                        prev_pc as *mut std::os::raw::c_void,
                        instruction_value,
                        buffer.as_ptr() as *const i8,
                    );
                }
            }
        }
    }
}

pub struct EmbeddedVector<T, const SIZE: usize> {
    data: [T; SIZE],
    length: usize,
}

impl<T: Copy + Default, const SIZE: usize> EmbeddedVector<T, SIZE> {
    pub fn new() -> Self {
        EmbeddedVector {
            data: [T::default(); SIZE],
            length: 0,
        }
    }

    pub fn begin(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn length(&self) -> usize {
        SIZE
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if SIZE > 0 {
            Some(&mut self.data[SIZE - 1])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < SIZE {
            Some(&mut self.data[index])
        } else {
            None
        }
    }
}

impl<T: Copy + Default, const SIZE: usize> std::ops::Index<usize> for EmbeddedVector<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < SIZE {
            &self.data[index]
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<T: Copy + Default, const SIZE: usize> std::ops::IndexMut<usize> for EmbeddedVector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < SIZE {
            &mut self.data[index]
        } else {
            panic!("Index out of bounds");
        }
    }
}

pub struct Vector<T> {
    start_: *mut T,
    length_: usize,
}
pub enum VFPRegPrecision {
    kSinglePrecision,
    kDoublePrecision,
    kSimd128Precision,
}

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.length_
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub enum NeonListType {
    nlt_1,
    nlt_2,
    nlt_3,
    nlt_4,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterArm {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    sp,
    lr,
    pc,
}

pub fn RegisterName(r: codegen::arm::register_arm::CPURegister) -> &'static str {
    use codegen::arm::register_arm::CPURegister::*;
    match r {
        r0 => "r0",
        r1 => "r1",
        r2 => "r2",
        r3 => "r3",
        r4 => "r4",
        r5 => "r5",
        r6 => "r6",
        r7 => "r7",
        r8 => "r8",
        r9 => "r9",
        r10 => "r10",
        r11 => "r11",
        r12 => "r12",
        super::codegen::arm::register_arm::CPURegister::sp => "sp",
        super::codegen::arm::register_arm::CPURegister::lr => "lr",
        super::codegen::arm::register_arm::CPURegister::pc => "pc",
    }
}

pub struct Instruction {
    instruction_bits: i32,
    address: usize,
}

pub enum Neon64 {
    Neon64,
}

impl Instruction {
    pub fn At(address: usize) -> *mut Instruction {
        address as *mut Instruction
    }
    pub fn InstructionBits(&self) -> i32 {
        self.instruction_bits
    }

    pub fn ConditionField(&self) -> Condition {
        unsafe { transmute(self.Bits(27, 24) as u8) }
    }
    pub fn TypeValue(&self) -> i32 {
        self.Bits(27, 25)
    }
    pub fn SvcValue(&self) -> SoftwareInterruptCodes {
        unsafe { transmute(self.Bits(7, 0) as u8) }
    }

    pub fn Bits(&self, msb: i32, lsb: i32) -> i32 {
        let mask: i32 = ((1 << (msb - lsb + 1)) - 1) << lsb;
        (self.instruction_bits & mask) >> lsb
    }

    pub fn IsSpecialType0(&self) -> bool {
        self.Bit(26) == 1
    }
    pub fn HasH(&self) -> bool {
        self.Bit(5) == 1
    }
    pub fn HasB(&self) -> bool {
        self.Bit(22) == 1
    }
    pub fn HasLink(&self) -> bool {
        self.Bit(24) == 1
    }
    pub fn SImmed24Value(&self) -> i32 {
        let imm24 = self.instruction_bits & 0x00FFFFFF;
        if (imm24 & 0x00800000) != 0 {
            imm24 | 0xFF000000
        } else {
            imm24
        }
    }
    pub fn Immed8Value(&self) -> i32 {
        self.Bits(7, 0)
    }
    pub fn ImmedHValue(&self) -> i32 {
        self.Bits(11, 8)
    }
    pub fn ImmedLValue(&self) -> i32 {
        self.Bits(3, 0)
    }
    pub fn ImmedMovwMovtValue(&self) -> i32 {
        (self.Bits(15, 12) << 16) | (self.Bits(3, 0) << 12) | (self.Bits(11, 8) << 4) | (self.Bit(26) << 15) | (self.Bit(27) << 19)
    }
    pub fn Offset12Value(&self) -> i32 {
        self.instruction_bits & 0xFFF
    }
    pub fn HasL(&self) -> bool {
        (self.instruction_bits & (1 << 20)) != 0
    }
    pub fn RnValue(&self) -> i32 {
        self.Bits(19, 16)
    }
    pub fn VfValue(&self) -> i32 {
        self.Bits(15, 12)
    }
    pub fn PUField(&self) -> da_x {
        unsafe { transmute(self.Bits(24, 23) as u8) }
    }
    pub fn SvcValue(&self) -> SoftwareInterruptCodes {
        unsafe { transmute(self.Bits(7, 0) as u8) }
    }

    pub fn bits(self, hi: Tagged<Smi>, lo: Tagged<Smi>) -> Result<(), String> {
        Ok(())
    }
    pub fn HasSign(&self) -> bool {
        self.Bit(6) == 1
    }
    pub fn ShiftField(&self) -> codegen::arm::ShiftOp {
        unsafe { transmute(self.Bits(5, 4) as u8) }
    }
    pub fn ShiftValue(&self) -> i32 {
        self.Bits(5, 4)
    }
    pub fn RsValue(&self) -> i32 {
        self.Bits(11, 8)
    }
    pub fn RmValue(&self) -> i32 {
        self.Bits(3, 0)
    }
    pub fn RotateValue(&self) -> i32 {
        self.Bits(11, 8)
    }
    pub fn HasS(&self) -> bool {
        self.Bit(20) == 1
    }

    pub fn rd(&mut self, dst: Register, src1: Register, src2: Operand, cond: Condition) {}

    pub fn Add(
        &mut self,
        rd: Register,
        rn: Register,
        imm: i32,
        carry: bool,
        set_flags: bool,
    ) -> Result<(), String> {
        Ok(())
    }
    pub fn ShiftAmountValue(&self) -> i32 {
        self.Bits(11, 7)
    }
    pub fn CondValue(&self) -> i32 {
        self.Bits(27, 24)
    }
    pub fn RdValue(&self) -> i32 {
        self.Bits(15, 12)
    }
    pub fn RtValue(&self) -> i32 {
        self.Bits(15, 12)
    }
    pub fn RlistValue(&self) -> i32 {
        self.Bits(15, 0)
    }
    pub fn OpcodeField(&self) -> Opcode {
        unsafe { transmute(self.Bits(24, 21) as u8) }
    }
    pub fn CoprocessorValue(&self) -> i32 {
        self.Bits(11, 8)
    }
    pub fn VcValue(&self) -> i32 {
        self.Bits(7, 7)
    }
    pub fn VaValue(&self) -> i32 {
        self.Bits(4, 4)
    }
    pub fn VlValue(&self) -> i32 {
        self.Bits(8, 8)
    }
    pub fn VFPNRegValue(&self, precision: VFPRegPrecision) -> i32 {
        match precision {
            VFPRegPrecision::kSinglePrecision => self.Bits(15, 12) | (self.Bit(22) << 4),
            VFPRegPrecision::kDoublePrecision => self.Bits(15, 12) | (self.Bit(22) << 4),
            VFPRegPrecision::kSimd128Precision => self.Bits(15, 12) | (self.Bit(22) << 4),
        }
    }
    pub fn VFPMRegValue(&self, precision: VFPRegPrecision) -> i32 {
        match precision {
            VFPRegPrecision::kSinglePrecision => self.Bits(3, 0) | (self.Bit(8) << 4),
            VFPRegPrecision::kDoublePrecision => self.Bits(3, 0) | (self.Bit(8) << 4),
            VFPRegPrecision::kSimd128Precision => self.Bits(3, 0) | (self.Bit(8) << 4),
        }
    }
    pub fn SzValue(&self) -> i32 {
        self.Bits(19, 18)
    }
    pub fn Opc1Value(&self) -> i32 {
        self.Bits(15, 12)
    }
    pub fn Opc2Value(&self) -> i32 {
        self.Bits(6, 4)
    }
    pub fn Opc3Value(&self) -> i32 {
        self.Bits(3, 0)
    }
    pub fn DoubleImmedVmov(&self) -> HeapNumber {
        HeapNumber { number: 0 }
    }
    pub fn VdValue(&self) -> i32 {
        self.Bits(15, 12)
    }
    pub fn IsNopLikeType1(&self) -> bool {
        self.Bits(27, 20) == 0 && self.Bits(19, 16) == 0xF
    }
    pub fn VnValue(&self) -> i32 {
        self.Bits(19, 16)
    }
    pub fn VmValue(&self) -> i32 {
        self.Bits(3, 0)
    }
    pub fn bit(&self, nr: i32) -> i32 {
        (self.InstructionBits() >> nr) & 1
    }
    pub fn ConditionValue(&self) -> Condition {
        unsafe { transmute(self.Bits(27, 24) as u8) }
    }
    pub fn bitfield(self, arg0: i32, arg1: i32) -> BX {
        todo!()
    }
}

impl codegen::arm::register_arm::CPURegister {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => codegen::arm::register_arm::CPURegister::r0,
            1 => codegen::arm::register_arm::CPURegister::r1,
            2 => codegen::arm::register_arm::CPURegister::r2,
            3 => codegen::arm::register_arm::CPURegister::r3,
            4 => codegen::arm::register_arm::CPURegister::r4,
            5 => codegen::arm::register_arm::CPURegister::r5,
            6 => codegen::arm::register_arm::CPURegister::r6,
            7 => codegen::arm::register_arm::CPURegister::r7,
            8 => codegen::arm::register_arm::CPURegister::r8,
            9 => codegen::arm::register_arm::CPURegister::r9,
            10 => codegen::arm::register_arm::CPURegister::r10,
            11 => codegen::arm::register_arm::CPURegister::r11,
            12 => codegen::arm::register_arm::CPURegister::r12,
            13 => codegen::arm::register_arm::CPURegister::sp,
            14 => codegen::arm::register_arm::CPURegister::lr,
            15 => codegen::arm::register_arm::CPURegister::pc,
            _ => panic!("invalid register code"),
        }
    }
}

impl From<std::string::FromUtf8Error> for disasm::NameConverter {
    fn from(err: std::string::FromUtf8Error) -> Self {
        todo!()
    }
}
#[derive(Debug)]
pub struct NeonListOperand {
    register: DwVfpRegister,
    len: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum DwVfpRegister {
    D0,
    D1,
}

impl DwVfpRegister {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => DwVfpRegister::D0,
            1 => DwVfpRegister::D1,
            _ => DwVfpRegister::D0,
        }
    }
}

impl NeonListOperand {
    pub fn new(register: DwVfpRegister, len: i32) -> Self {
        NeonListOperand { register, len }
    }

    pub fn register(&self) -> DwVfpRegister {
        self.register
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn type_(&self) -> NeonListType {
        match self.len {
            1 => NeonListType::nlt_1,
            2 => NeonListType::nlt_2,
            3 => NeonListType::nlt_3,
            4 => NeonListType::nlt_4,
            _ => NeonListType::nlt_1,
        }
    }
}
#[derive(Copy, Clone)]
pub enum VFPRegPrecision {
    kSinglePrecision,
    kDoublePrecision,
    kSimd128Precision,
}

impl From<std::string::FromUtf8Error> for NeonListOperand {
    fn from(err: std::string::FromUtf8Error) -> Self {
        todo!()
    }
}

pub fn DecodeConstantPoolLength(instruction_bits: i32) -> i32 {
    0
}

pub enum UnimplementedOpcodeAction {
    Print,
}
enum nlt_1 {
    nlt_1,
}
enum nlt_2 {
    nlt_2,
}

pub struct Decoder {
    converter_: NameConverter,
    out_buffer_: v8::base::Vector<char>,
    out_buffer_pos_: usize,
}

impl Decoder {
    pub fn new(converter_: NameConverter, out_buffer_: v8::base::Vector<char>) -> Self {
        let mut decoder = Decoder {
            converter_: converter_,
            out_buffer_: out_buffer_,
            out_buffer_pos_: 0,
        };
        decoder.out_buffer_[decoder.out_buffer_pos_] = '\0';
        decoder
    }

    fn PrintChar(&mut self, ch: char) {
        self.out_buffer_[self.out_buffer_pos_] = ch;
        self.out_buffer_pos_ += 1;
    }

    fn Print(&mut self, str: &str) {
        for &cur in str.as_bytes() {
            if self.out_buffer_pos_ < (self.out_buffer_.length() - 1) {
                self.out_buffer_[self.out_buffer_pos_] = cur as char;
                self.out_buffer_pos_ += 1;
            } else {
                break;
            }
        }
        self.out_buffer_[self.out_buffer_pos_] = '\0';
    }
    fn PrintRegister(&mut self, reg: i32) {
        self.Print(self.converter_.NameOfCPURegister(reg));
    }

    fn PrintSRegister(&mut self, reg: i32) {
        self.Print(VFPRegisters::Name(reg, false));
    }

    fn PrintDRegister(&mut self, reg: i32) {
        self.Print(VFPRegisters::Name(reg, true));
    }
    fn PrintQRegister(&mut self, reg: i32) {
        self.Print(RegisterName(QwNeonRegister::from_code(reg)));
    }
    fn FormatVFPRegister(
        &mut self,
        instr: &Instruction,
        format: &str,
        precision: VFPRegPrecision,
    ) -> i32 {
        let mut retval = 2;
        let mut reg = -1;

        if format.chars().nth(1) == Some('n') {
            reg = instr.VFPNRegValue(precision);
        } else if format.chars().nth(1) == Some('m') {
            reg = instr.VFPMRegValue(precision);
        } else if format.chars().nth(1) == Some('d') {
            if instr.TypeValue() == 7 && instr.bit(24) == 0x0 && instr.Bits(11, 9) == 0x5 && instr.bit(4) == 0x1 {
                reg = instr.Bits(19, 16) | (instr.bit(7) << 4);
            } else {
                reg = instr.VFPDRegValue(precision);
            }

            if format.chars().nth(2) == Some('+')

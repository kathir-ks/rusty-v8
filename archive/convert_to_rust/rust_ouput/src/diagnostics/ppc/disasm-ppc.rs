// Converted from V8 C++ source files:
// Header: N/A
// Implementation: disasm-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::error::Error;
use std::ffi::c_void;
use std::fmt;
use std::io::Write;
use std::mem;
use std::ptr;
use std::result;
use std::str;

use crate::internal::{V8};

pub struct NameConverter {
    tmp_buffer_: [u8; 256],
}

impl NameConverter {
    pub fn new() -> Self {
        NameConverter {
            tmp_buffer_: [0; 256],
        }
    }

    pub fn NameOfAddress(&self, addr: *mut u8) -> *const i8 {
        let addr_ptr = addr as *const c_void;
        let buffer = &mut self.tmp_buffer_;
        let len = format!("{:p}", addr_ptr).len();
        let formatted = format!("{:p}", addr_ptr);

        if len < buffer.len() {
            for (i, byte) in formatted.bytes().enumerate() {
                buffer[i] = byte;
            }
            buffer[len] = 0;
            return buffer.as_ptr() as *const i8;
        } else {
            // Handle the case where the formatted string is too long.
            // Perhaps truncate or return a default string.
            return "AddressTooLong\0".as_ptr() as *const i8;
        }
    }

    pub fn NameOfConstant(&self, addr: *mut u8) -> *const i8 {
        self.NameOfAddress(addr)
    }

    pub fn NameOfCPURegister(&self, reg: i32) -> *const i8 {
        let reg_name = register_name(reg);
        reg_name.as_ptr() as *const i8
    }

    pub fn NameOfByteCPURegister(&self, reg: i32) -> *const i8 {
        "ByteRegisterUnsupported\0".as_ptr() as *const i8
    }

    pub fn NameOfXMMRegister(&self, reg: i32) -> *const i8 {
        "XMMRegisterUnsupported\0".as_ptr() as *const i8
    }

    pub fn NameInCode(&self, addr: *mut u8) -> *const i8 {
        "".as_ptr() as *const i8
    }
}

fn register_name(reg: i32) -> String {
    format!("r{}", reg)
}

#[derive(Debug)]
pub enum DisassemblerError {
    GenericError(String),
    Utf8Error(str::Utf8Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisassemblerError::GenericError(msg) => write!(f, "Generic disassembler error: {}", msg),
            DisassemblerError::Utf8Error(err) => write!(f, "UTF-8 error: {}", err),
            DisassemblerError::FromUtf8Error(err) => write!(f, "From UTF-8 error: {}", err),
        }
    }
}

impl Error for DisassemblerError {}

impl From<str::Utf8Error> for DisassemblerError {
    fn from(err: str::Utf8Error) -> Self {
        DisassemblerError::Utf8Error(err)
    }
}

impl From<std::string::FromUtf8Error> for DisassemblerError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        DisassemblerError::FromUtf8Error(err)
    }
}

pub enum UnimplementedOpcodeAction {
    Ignore,
    Print,
    Abort,
}

pub struct Disassembler<'a> {
    converter_: &'a NameConverter,
    unimplemented_action_: UnimplementedOpcodeAction,
}

impl<'a> Disassembler<'a> {
    pub fn new(converter: &'a NameConverter, unimplemented_action: UnimplementedOpcodeAction) -> Self {
        Disassembler {
            converter_: converter,
            unimplemented_action_: unimplemented_action,
        }
    }
    pub fn length(&self) -> usize {
        0 // Provide a default implementation
    }
    pub fn Disassemble(&self, f: &mut dyn Write, begin: *mut u8, end: *mut u8) {}

    pub fn InstructionDecode(&self, buffer: &mut [u8], instruction: *mut u8) -> usize {
        let converter = self.converter_;
        let vector = crate::base::vector::Vector {
            start_: buffer.as_mut_ptr() as *mut i8,
            length_: buffer.len(),
        };

        let mut d = Decoder::new(converter, vector);
        let result = d.InstructionDecode(instruction);

        result
    }

    pub fn ConstantPoolSizeAt(&self, instruction: *mut u8) -> i32 {
        -1
    }

    pub fn DisassembleCode(f: &mut dyn Write, begin: *mut u8, end: *mut u8, unimplemented_action: UnimplementedOpcodeAction) {
        let converter = NameConverter::new();
        let disassembler = Disassembler::new(&converter, unimplemented_action);
        for pc in (begin as usize..end as usize).step_by(4) {
            let pc_ptr = pc as *mut u8;
            let mut buffer: [u8; 128] = [0; 128];
            let instruction_size = disassembler.InstructionDecode(&mut buffer, pc_ptr);

            if instruction_size > 0 {
                let formatted_output = format!(
                    "{:p}    {:08x}      {}",
                    pc_ptr,
                    unsafe { *(pc_ptr as *mut i32) },
                    String::from_utf8_lossy(&buffer[..instruction_size])
                );
                writeln!(f, "{}", formatted_output).unwrap();
            } else {
                writeln!(f, "{:p}    Invalid Instruction", pc_ptr).unwrap();
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PrefixType {
    NotPrefixed,
    IsPrefixed,
}

struct Decoder<'a> {
    converter_: &'a NameConverter,
    out_buffer_: crate::base::vector::Vector,
    out_buffer_pos_: i32,
    prefix_status_: PrefixType,
    prefix_value_: u64,
}

impl<'a> Decoder<'a> {
    fn new(converter_: &'a NameConverter, out_buffer_: crate::base::vector::Vector) -> Self {
        Decoder {
            converter_: converter_,
            out_buffer_: out_buffer_,
            out_buffer_pos_: 0,
            prefix_status_: PrefixType::NotPrefixed,
            prefix_value_: 0,
        }
    }

    fn InstructionDecode(&mut self, instruction: *mut u8) -> usize {
        let instr = Instruction::At(instruction);

        let opcode = instr.OpcodeValue() << 26;

        if opcode != EXTP {
            let bits = instr.InstructionBits();
            let format_string = format!("{:08x}       ", bits);
            self.Print(&format_string);
        } else {
            let next_instr = unsafe { instruction.add(4) };
            let next_instr_bits = Instruction::At(next_instr).InstructionBits();
            let format_string = format!("{:08x}|{:08x} ", instr.InstructionBits(), next_instr_bits);
            self.Print(&format_string);
        }

        if is_abi_uses_function_descriptors() && instr.InstructionBits() == 0 {
            self.Format(&instr, "constant");
            return 4;
        }

        match opcode {
            TWI => {
                self.PrintSoftwareInterrupt(instr.SvcValue());
            }
            MULLI => {
                self.UnknownFormat(&instr, "mulli");
            }
            SUBFIC => {
                self.Format(&instr, "subfic  'rt, 'ra, 'int16");
            }
            CMPLI => {
                if instr.Bit(21) == 1 {
                    self.Format(&instr, "cmpli   'ra, 'uint16");
                } else {
                    self.Format(&instr, "cmplwi  'ra, 'uint16");
                }
            }
            CMPI => {
                if instr.Bit(21) == 1 {
                    self.Format(&instr, "cmpi    'ra, 'int16");
                } else {
                    self.Format(&instr, "cmpwi   'ra, 'int16");
                }
            }
            ADDIC => {
                self.Format(&instr, "addic   'rt, 'ra, 'int16");
            }
            ADDICx => {
                self.UnknownFormat(&instr, "addicx");
            }
            ADDI => {
                if instr.RAValue() == 0 {
                    self.Format(&instr, "li      'rt, 'int16");
                } else {
                    self.Format(&instr, "addi    'rt, 'ra, 'int16");
                }
            }
            ADDIS => {
                if instr.RAValue() == 0 {
                    self.Format(&instr, "lis     'rt, 'int16");
                } else {
                    self.Format(&instr, "addis   'rt, 'ra, 'int16");
                }
            }
            BCX => {
                let bo = instr.Bits(25, 21) << 21;
                let bi = instr.Bits(20, 16);
                let cond = bi & (CRWIDTH - 1);
                match bo {
                    BT => {
                        match cond {
                            CR_EQ => self.Format(&instr, "beq'l'a'cr 'target16"),
                            CR_GT => self.Format(&instr, "bgt'l'a'cr 'target16"),
                            CR_LT => self.Format(&instr, "blt'l'a'cr 'target16"),
                            CR_SO => self.Format(&instr, "bso'l'a'cr 'target16"),
                            _ => {}
                        }
                    }
                    BF => {
                        match cond {
                            CR_EQ => self.Format(&instr, "bne'l'a'cr 'target16"),
                            CR_GT => self.Format(&instr, "ble'l'a'cr 'target16"),
                            CR_LT => self.Format(&instr, "bge'l'a'cr 'target16"),
                            CR_SO => self.Format(&instr, "bnso'l'a'cr 'target16"),
                            _ => {}
                        }
                    }
                    DCBNZ => {
                        self.Format(&instr, "bdnz'l'a 'target16");
                    }
                    _ => {
                        self.Format(&instr, "bc'l'a'cr 'target16");
                    }
                }
            }
            SC => {
                self.UnknownFormat(&instr, "sc");
            }
            BX => {
                self.Format(&instr, "b'l'a 'target26");
            }
            EXTP => {
                self.DecodeExtP(&instr);
            }
            EXT0 => {
                self.DecodeExt0(&instr);
            }
            EXT1 => {
                self.DecodeExt1(&instr);
            }
            RLWIMIX => {
                self.Format(&instr, "rlwimi'. 'ra, 'rs, 'sh, 'me, 'mb");
            }
            RLWINMX => {
                self.Format(&instr, "rlwinm'. 'ra, 'rs, 'sh, 'me, 'mb");
            }
            RLWNMX => {
                self.Format(&instr, "rlwnm'.  'ra, 'rs, 'rb, 'me, 'mb");
            }
            ORI => {
                self.Format(&instr, "ori     'ra, 'rs, 'uint16");
            }
            ORIS => {
                self.Format(&instr, "oris    'ra, 'rs, 'uint16");
            }
            XORI => {
                self.Format(&instr, "xori    'ra, 'rs, 'uint16");
            }
            XORIS => {
                self.Format(&instr, "xoris   'ra, 'rs, 'uint16");
            }
            ANDIx => {
                self.Format(&instr, "andi.   'ra, 'rs, 'uint16");
            }
            ANDISx => {
                self.Format(&instr, "andis.  'ra, 'rs, 'uint16");
            }
            EXT2 => {
                self.DecodeExt2(&instr);
            }
            LWZ => {
                self.Format(&instr, "lwz     'rt, 'int16('ra)");
            }
            LWZU => {
                self.Format(&instr, "lwzu    'rt, 'int16('ra)");
            }
            LBZ => {
                self.Format(&instr, "lbz     'rt, 'int16('ra)");
            }
            LBZU => {
                self.Format(&instr, "lbzu    'rt, 'int16('ra)");
            }
            STW => {
                self.Format(&instr, "stw     'rs, 'int16('ra)");
            }
            STWU => {
                self.Format(&instr, "stwu    'rs, 'int16('ra)");
            }
            STB => {
                self.Format(&instr, "stb     'rs, 'int16('ra)");
            }
            STBU => {
                self.Format(&instr, "stbu    'rs, 'int16('ra)");
            }
            LHZ => {
                self.Format(&instr, "lhz     'rt, 'int16('ra)");
            }
            LHZU => {
                self.Format(&instr, "lhzux   'rt, 'int16('ra)");
            }
            LHA => {
                self.Format(&instr, "lha     'rt, 'int16('ra)");
            }
            LHAU => {
                self.Format(&instr, "lhau    'rt, 'int16('ra)");
            }
            STH => {
                self.Format(&instr, "sth     'rs, 'int16('ra)");
            }
            STHU => {
                self.Format(&instr, "sthu    'rs, 'int16('ra)");
            }
            LMW => {
                self.UnknownFormat(&instr, "lmw");
            }
            STMW => {
                self.UnknownFormat(&instr, "stmw");
            }
            LFS => {
                self.Format(&instr, "lfs     'Dt, 'int16('ra)");
            }
            LFSU => {
                self.Format(&instr, "lfsu    'Dt, 'int16('ra)");
            }
            LFD => {
                self.Format(&instr, "lfd     'Dt, 'int16('ra)");
            }
            LFDU => {
                self.Format(&instr, "lfdu    'Dt, 'int16('ra)");
            }
            STFS => {
                self.Format(&instr, "stfs    'Dt, 'int16('ra)");
            }
            STFSU => {
                self.Format(&instr, "stfsu   'Dt, 'int16('ra)");
            }
            STFD => {
                self.Format(&instr, "stfd    'Dt, 'int16('ra)");
            }
            STFDU => {
                self.Format(&instr, "stfdu   'Dt, 'int16('ra)");
            }
            EXT3 => {
                self.DecodeExt3(&instr);
            }
            EXT4 => {
                self.DecodeExt4(&instr);
            }
            EXT5 => {
                self.DecodeExt5(&instr);
            }
            EXT6 => {
                self.DecodeExt6(&instr);
            }
            LD => {
                match instr.Bits(1, 0) {
                    0 => self.Format(&instr, "ld      'rt, 'd('ra)"),
                    1 => self.Format(&instr, "ldu     'rt, 'd('ra)"),
                    2 => self.Format(&instr, "lwa     'rt, 'd('ra)"),
                    _ => {}
                }
            }
            STD => {
                if instr.Bit(0) == 0 {
                    self.Format(&instr, "std     'rs, 'd('ra)");
                } else {
                    self.Format(&instr, "stdu    'rs, 'd('ra)");
                }
            }
            _ => {
                self.Unknown(&instr);
            }
        }

        if self.is_prefixed() {
            self.ResetPrefix();
            return 8;
        }

        return 4;
    }

    fn GetPrefixValue(&self) -> u64 {
        self.prefix_value_
    }

    fn SetAsPrefixed(&mut self, v: u64) {
        self.prefix_status_ = PrefixType::IsPrefixed;
        self.prefix_value_ = v;
    }

    fn ResetPrefix(&mut self) {
        self.prefix_status_ = PrefixType::NotPrefixed;
        self.prefix_value_ = 0;
    }

    fn is_prefixed(&self) -> bool {
        match self.prefix_status_ {
            PrefixType::IsPrefixed => true,
            PrefixType::NotPrefixed => false,
        }
    }

    fn PrintChar(&mut self, ch: char) {
        if self.out_buffer_pos_ < (self.out_buffer_.length_ as i32 - 1) {
            unsafe {
                *self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize) = ch as i8;
            }
            self.out_buffer_pos_ += 1;
        }
        if self.out_buffer_pos_ >= (self.out_buffer_.length_ as i32 -1) {
            unsafe {
                *self.out_buffer_.start_.offset((self.out_buffer_.length_ as i32 - 1) as isize) = 0 as i8;
            }
        }

    }

    fn Print(&mut self, str: &str) {
        for char in str.chars() {
            self.PrintChar(char);
        }
    }

    fn PrintRegister(&mut self, reg: i32) {
        let register_name = self.converter_.NameOfCPURegister(reg);
        let register_name_str = unsafe {
            let mut len = 0;
            while *register_name.offset(len) != 0 {
                len += 1;
            }
            str::from_utf8(std::slice::from_raw_parts(register_name as *const u8, len as usize)).unwrap()
        };
        self.Print(register_name_str);
    }

    fn PrintDRegister(&mut self, reg: i32) {
        self.Print(&format!("d{}", reg));
    }

    fn PrintVectorRegister(&mut self, reg: i32) {
        self.Print(&format!("v{}", reg));
    }

    fn PrintSoftwareInterrupt(&mut self, svc: i32) {
        match svc {
            kCallRtRedirected => self.Print("call rt redirected"),
            kBreakpoint => self.Print("breakpoint"),
            _ => {
                if svc >= kStopCode {
                    let formatted = format!("{} - 0x{:x}", svc & kStopCodeMask, svc & kStopCodeMask);
                    self.Print(&formatted);
                } else {
                    let formatted = format!("{}", svc);
                    self.Print(&formatted);
                }
            }
        }
    }

    fn FormatRegister(&mut self, instr: &Instruction, format: &str) -> i32 {
        if format.len() < 2 { return 0; }

        if format.starts_with("rt") || format.starts_with("rs") {
            let reg = instr.RTValue();
            self.PrintRegister(reg);
            2
        } else if format.starts_with("ra") {
            let reg = instr.RAValue();
            self.PrintRegister(reg);
            2
        } else if format.starts_with("rb") {
            let reg = instr.RBValue();
            self.PrintRegister(reg);
            2
        } else {
            0
        }
    }

    fn FormatFPRegister(&mut self, instr: &Instruction, format: &str) -> i32 {
        let mut retval = 2;
        let reg =
            if format.chars().nth(1) == Some('t') || format.chars().nth(1) == Some('s') {
                instr.RTValue()
            } else if format.chars().nth(1) == Some('a') {
                instr.RAValue()
            } else if format.chars().nth(1) == Some('b') {
                instr.RBValue()
            } else if format.chars().nth(1) == Some('c') {
                instr.RCValue()
            } else {
                -1 // Handle error case appropriately
            };

        self.PrintDRegister(reg);
        retval
    }

    fn FormatVectorRegister(&mut self, instr: &Instruction, format: &str) -> i32 {
        let reg =
            if format.chars().nth(1) == Some('t') || format.chars().nth(1) == Some('s') {
                instr.RTValue()
            } else if format.chars().nth(1) == Some('a') {
                instr.RAValue()
            } else if format.chars().nth(1) == Some('b') {
                instr.RBValue()
            } else if format.chars().nth(1) == Some('c') {
                instr.RCValue()
            } else {
                -1 // Handle error case appropriately
            };
        self.PrintVectorRegister(reg);
        2
    }

    fn FormatOption(&mut self, instr: &Instruction, format: &str) -> i32 {
        if format.is_empty() { return 0; }

        match format.chars().next().unwrap() {
            'o' => {
                if instr.Bit(10) == 1 {
                    self.Print("o");
                }
                1
            }
            '.' => {
                if instr.Bit(0) == 1 {
                    self.Print(".");
                } else {
                    self.Print(" ");
                }
                1
            }
            'r' => self.FormatRegister(instr, format),
            'D' => self.FormatFPRegister(instr, format),
            'X' => {
                if instr.Bit(0) == 1 {
                    self.FormatVectorRegister(instr, format)
                } else {
                    self.FormatFPRegister(instr, format)
                }
            }
            'V' => self.FormatVectorRegister(instr, format),
            'i' => {
                let imm_value = instr.Bits(15, 0) as u32;
                let value = if self.is_prefixed() {
                    let prefix_value = self.GetPrefixValue();
                    sign_ext_imm34(((prefix_value << 16) | imm_value) as i64)
                } else {
                    let imm_value = imm_value as i64;
                    (imm_value << 48) >> 48
                };
                let formatted = format!("{}", value);
                self.Print(&formatted);
                5
            }
            'I' => {
                let value = instr.Bits(18, 11) as i8;
                let formatted = format!("{}", value);
                self.Print(&formatted);
                4
            }
            'u' => {
                let value = instr.Bits(15, 0) as i32;
                let formatted = format!("{}", value);
                self.Print(&formatted);
                6
            }
            'F' => {
                let value = instr.Bits(19, 12) as u8;
                let formatted = format!("{}", value);
                self.Print(&formatted);
                3
            }
            'S' => {
                let value = sign_ext_imm5(instr.Bits(20, 16)) as i32;
                let formatted = format!("{}", value);
                self.Print(&formatted);
                3
            }
            'U' => {
                let value = instr.Bits(19, 16) as u8;
                let formatted = format!("{}", value);
                self.Print(&formatted);
                3
            }
            'l' => {
                if instr.Bit(0) == 1 {
                    self.Print("l");
                }
                1
            }
            'a' => {
                if instr.Bit(1) == 1 {
                    self.Print("a");
                }
                1
            }
            'c' => {
                let code = instr.Bits(20, 18);
                if code != 7 {
                    self.Print(&format!(" cr{}", code));
                }
                2
            }
            't' => {
                if format.starts_with("target26") {
                    let off = ((instr.Bits(25, 2) as i32) << 8) >> 6;
                    let address = unsafe { (instr as *const Instruction as *const u8).add(off as usize) as *mut u8 };
                    let address_str = unsafe { std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(address))
                        .to_string_lossy()
                        .into_owned() };
                    self.Print(&format!("{}{}{}", "+", off, " -> "));
                    self.Print(&address_str);
                    8
                } else if format.starts_with("target16") {
                    let off = ((instr.Bits(15, 2) as i32) << 18) >> 16;
                    let address = unsafe { (instr as *const Instruction as *const u8).add(off as usize) as *mut u8 };
                    let address_str = unsafe { std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(address))
                        .to_string_lossy()
                        .into_owned() };
                    self.Print(&format!("{}{}{}", "+", off, " -> "));
                    self.Print(&address_str);
                    8
                } else {
                    0
                }
            }
            's' => {
                if format.chars().nth(1) == Some('h') {
                    let mut value = 0;
                    let opcode = instr.OpcodeValue() << 26;
                    let sh = instr.Bits(15, 11) as i32;
                    if opcode == EXT5 ||
                        (opcode == EXT2 && instr.Bits(10, 2) << 2 == SRADIX) {
                        value = (sh | (instr.Bit(1) << 5)) as i32;
                    } else {
                        value = (sh << 26) >> 26;
                    }
                    self.Print(&format!("{}", value));
                    2
                } else {
                    0
                }
            }
            'm' => {
                let mut value = 0;
                if format.chars().nth(1) == Some('e') {
                    if instr.OpcodeValue() << 26 != EXT5 {
                        value = (instr.Bits(10, 6) << 26) >> 26;
                    } else {
                        value = (instr.Bits(10, 6) | (instr.Bit(5) << 5)) as i32;
                    }
                } else if format.chars().nth(1) == Some('b') {
                    if instr.OpcodeValue() << 26 != EXT5 {
                        value = (instr.Bits(5, 1) << 26) >> 26;
                    } else {
                        value = (instr.Bits(10, 6) | (instr.Bit(5) << 5)) as i32;
                    }
                } else {
                    return 0;
                }
                self.Print(&format!("{}", value));
                2
            }
            'd' => {
                let value = sign_ext_imm16((instr.Bits(15, 0) & !3) as i32);
                self.Print(&format!("{}", value));
                1
            }
            _ => 0
        }
    }

    fn Format(&mut self, instr: &Instruction, format: &str) {
        let mut cur = 0;
        while cur < format.len() && (self.out_buffer_pos_ < (self.out_buffer_.length_ as i32 - 1)) {
            if format.chars().nth(cur) == Some('\'') {
                let consumed = self.FormatOption(instr, &format[cur + 1..]);
                cur += (consumed + 1) as usize;
            } else {
                self.PrintChar(format.chars().nth(cur).unwrap());
                cur += 1;
            }
        }
        if self.out_buffer_pos_ < (self.out_buffer_.length_ as i32) {
             unsafe {
                 *self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize) = 0 as i8;
             }
        }
    }

    fn Unknown(&mut self, instr: &Instruction) {
        self.Format(instr, "unknown");
    }

    fn UnknownFormat(&mut self, instr: &Instruction, name: &str) {
        let buffer = format!("{} (unknown-format)", name);
        self.Format(instr, &buffer);
    }

    fn DecodeExtP(&mut self, instr: &Instruction) {
        match EXTP | (instr.BitField(25, 25)) {
            PLOAD_STORE_8LS | PLOAD_STORE_MLS => {
                if instr.Bit(20) != 1 {
                    self.SetAsPrefixed(instr.Bits(17, 0) as u64);
                    let next_instr_ptr = unsafe { (instr as *const Instruction as *mut u8).add(4) };
                    let next_instr = Instruction::At(next_instr_ptr);

                    match next_instr.OpcodeBase() {
                        ADDI => {
                            if next_instr.RAValue() == 0 {
                                self.Format(instr, "pli");
                                self.Format(&next_instr, "     'rt, ");
                            } else {
                                self.Format(instr, "paddi");
                                self.Format(&next_instr, "   'rt, 'ra, ");
                            }
                            self.Format(&next_instr, "'int34");
                        }
                        LBZ => self.Format(&next_instr, "plbz    'rt, 'int34('ra)"),
                        LHZ => self.Format(&next_instr, "plhz    'rt, 'int34('ra)"),
                        LHA => self.Format(&next_instr, "plha    'rt, 'int34('ra)"),
                        LWZ => self.Format(&next_instr, "plwz    'rt, 'int34('ra)"),
                        PPLWA => self.Format(&next_instr, "plwa    'rt, 'int34('ra)"),
                        PPLD => self.Format(&next_instr, "pld     'rt, 'int34('ra)"),
                        LFS => self.Format(&next_instr, "plfs    'Dt, 'int34('ra)"),
                        LFD => self.Format(&next_instr, "plfd    'Dt, 'int34('ra)"),
                        STB => self.Format(&next_instr, "pstb    'rs, 'int34('ra)"),
                        STH => self.Format(&next_instr, "psth    'rs, 'int34('ra)"),
                        STW => self.Format(&next_instr, "pstw    'rs, 'int34('ra)"),
                        PPSTD => self.Format(&next_instr, "pstd    'rs, 'int34('ra)"),
                        STFS => self.Format(&next_instr, "pstfs   'Dt, 'int34('ra)"),
                        STFD => self.Format(&next_instr, "pstfd   'Dt, 'int34('ra)"),
                        _ => self.Unknown(instr),
                    }
                } else {
                    self.Unknown(instr);
                }
            }
            _ => self.Unknown(instr),
        }
    }

    fn DecodeExt0(&mut self, instr: &Instruction) {
        match EXT0 | (instr.BitField(20, 16)) | (instr.BitField(10, 0)) {
            _ => {}
        }

        match EXT0 | (instr.BitField(5, 0)) {
            _ => {}
        }

        match EXT0 | (instr.BitField(9, 0)) {
            _ => {}
        }

        match EXT0 | (instr.BitField(10, 0)) {
            _ => {}
        }
    }

    fn DecodeExt1(&mut self, instr: &Instruction) {
        match EXT1 | (instr.BitField(10, 1)) {
            MCRF => self.UnknownFormat(instr, "mcrf"),
            BCLRX => {
                let bo =

// Converted from V8 C++ source files:
// Header: register-arm64.h
// Implementation: register-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

pub enum RegisterCode {
    kRegCode_x0,
    kRegCode_x1,
    kRegCode_x2,
    kRegCode_x3,
    kRegCode_x4,
    kRegCode_x5,
    kRegCode_x6,
    kRegCode_x7,
    kRegCode_x8,
    kRegCode_x9,
    kRegCode_x10,
    kRegCode_x11,
    kRegCode_x12,
    kRegCode_x13,
    kRegCode_x14,
    kRegCode_x15,
    kRegCode_x16,
    kRegCode_x17,
    kRegCode_x18,
    kRegCode_x19,
    kRegCode_x20,
    kRegCode_x21,
    kRegCode_x22,
    kRegCode_x23,
    kRegCode_x24,
    kRegCode_x25,
    kRegCode_x26,
    kRegCode_x27,
    kRegCode_x28,
    kRegCode_x29,
    kRegCode_x30,
    kRegCode_x31,
    kRegAfterLast,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CPURegister {
    code_: i32,
    reg_size_: u8,
    reg_type_: RegisterType,
}

impl CPURegister {
    pub const fn no_reg() -> Self {
        CPURegister {
            code_: 0,
            reg_size_: 0,
            reg_type_: RegisterType::kNoRegister,
        }
    }

    pub const fn create(code: i32, size: i32, reg_type: RegisterType) -> Self {
        CPURegister {
            code_: code,
            reg_size_: size as u8,
            reg_type_: reg_type,
        }
    }

    pub fn reg_type(&self) -> RegisterType {
        self.reg_type_
    }

    pub fn size_in_bits(&self) -> i32 {
        self.reg_size_ as i32
    }

    pub fn size_in_bytes(&self) -> i32 {
        (self.size_in_bits() / 8) as i32
    }

    pub fn is_8_bits(&self) -> bool {
        self.size_in_bits() == 8
    }

    pub fn is_16_bits(&self) -> bool {
        self.size_in_bits() == 16
    }

    pub fn is_32_bits(&self) -> bool {
        self.size_in_bits() == 32
    }

    pub fn is_64_bits(&self) -> bool {
        self.size_in_bits() == 64
    }

    pub fn is_128_bits(&self) -> bool {
        self.size_in_bits() == 128
    }

    pub fn is_none(&self) -> bool {
        self.reg_type_ == RegisterType::kNoRegister
    }

    pub fn aliases(&self, other: &CPURegister) -> bool {
        self.code_ == other.code_ && self.reg_type_ == other.reg_type_
    }

    pub fn is_zero(&self) -> bool {
        self.code_ == 31 && self.reg_type_ == RegisterType::kRegister
    }

    pub fn is_sp(&self) -> bool {
        self.code_ == 29 && self.reg_type_ == RegisterType::kRegister
    }

    pub fn is_register(&self) -> bool {
        self.reg_type_ == RegisterType::kRegister
    }

    pub fn is_vregister(&self) -> bool {
        self.reg_type_ == RegisterType::kVRegister
    }

    pub fn is_fpregister(&self) -> bool {
        self.is_s() || self.is_d()
    }

    pub fn is_w(&self) -> bool {
        self.is_register() && self.is_32_bits()
    }

    pub fn is_x(&self) -> bool {
        self.is_register() && self.is_64_bits()
    }

    pub fn is_v(&self) -> bool {
        self.is_vregister()
    }

    pub fn is_b(&self) -> bool {
        self.is_v() && self.is_8_bits()
    }

    pub fn is_h(&self) -> bool {
        self.is_v() && self.is_16_bits()
    }

    pub fn is_s(&self) -> bool {
        self.is_v() && self.is_32_bits()
    }

    pub fn is_d(&self) -> bool {
        self.is_v() && self.is_64_bits()
    }

    pub fn is_q(&self) -> bool {
        self.is_v() && self.is_128_bits()
    }

    pub fn reg(&self) -> Register {
        Register {
            code_: self.code_,
            reg_size_: self.reg_size_,
            reg_type_: self.reg_type_,
        }
    }

    pub fn vreg(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn x(&self) -> Register {
        Register {
            code_: self.code_,
            reg_size_: 64,
            reg_type_: self.reg_type_,
        }
    }

    pub fn w(&self) -> Register {
        Register {
            code_: self.code_,
            reg_size_: 32,
            reg_type_: self.reg_type_,
        }
    }

    pub fn v(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn b(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn h(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn d(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn s(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn q(&self) -> VRegister {
        VRegister {
            cpu_register: *self,
            lane_count_: 1,
        }
    }

    pub fn is_same_size_and_type(&self, other: &CPURegister) -> bool {
        self.reg_size_ == other.reg_size_ && self.reg_type_ == other.reg_type_
    }

    pub fn is_even(&self) -> bool {
        (self.code_ % 2) == 0
    }

    pub fn max_code(&self) -> i32 {
        if self.is_vregister() {
            31
        } else {
            31
        }
    }

    pub const fn code(&self) -> i32 {
        self.code_
    }

    pub const fn size(&self) -> u8 {
        self.reg_size_
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterType {
    kRegister,
    kVRegister,
    kNoRegister,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    code_: i32,
    reg_size_: u8,
    reg_type_: RegisterType,
}

impl Register {
    pub const fn no_reg() -> Self {
        Register {
            code_: 0,
            reg_size_: 0,
            reg_type_: RegisterType::kNoRegister,
        }
    }

    pub const fn create(code: i32, size: i32) -> Self {
        Register {
            code_: code,
            reg_size_: size as u8,
            reg_type_: RegisterType::kRegister,
        }
    }

    pub fn xreg_from_code(code: u32) -> Register {
        Register::create(code as i32, 64)
    }

    pub fn wreg_from_code(code: u32) -> Register {
        Register::create(code as i32, 32)
    }

    pub const fn from_code(code: i32) -> Self {
        Register::create(code, 64)
    }

    pub fn get_special_register_name(code: i32) -> &'static str {
        if code == 29 {
            "sp"
        } else {
            "UNKNOWN"
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VRegister {
    cpu_register: CPURegister,
    lane_count_: i8,
}

impl VRegister {
    pub const fn no_reg() -> Self {
        VRegister {
            cpu_register: CPURegister::no_reg(),
            lane_count_: 0,
        }
    }

    pub const fn create(code: i32, size: i32, lane_count: i32) -> Self {
        VRegister {
            cpu_register: CPURegister::create(code, size, RegisterType::kVRegister),
            lane_count_: lane_count as i8,
        }
    }

    pub fn create_from_format(reg_code: i32, format: VectorFormat) -> Self {
        let reg_size = register_size_in_bits_from_format(format) as i32;
        let reg_count = if is_vector_format(format) {
            lane_count_from_format(format)
        } else {
            1
        } as i32;
        VRegister::create(reg_code, reg_size, reg_count)
    }

    pub fn breg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 8, 1)
    }

    pub fn hreg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 16, 1)
    }

    pub fn sreg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 32, 1)
    }

    pub fn dreg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 64, 1)
    }

    pub fn qreg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 128, 1)
    }

    pub fn vreg_from_code(code: u32) -> VRegister {
        VRegister::create(code as i32, 128, 1)
    }

    pub fn v8b(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 64, 8)
    }

    pub fn v16b(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 128, 16)
    }

    pub fn v4h(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 64, 4)
    }

    pub fn v8h(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 128, 8)
    }

    pub fn v2s(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 64, 2)
    }

    pub fn v4s(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 128, 4)
    }

    pub fn v2d(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 128, 2)
    }

    pub fn v1d(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 64, 1)
    }

    pub fn v1q(&self) -> VRegister {
        VRegister::create(self.cpu_register.code(), 128, 1)
    }

    pub fn format(&self, f: VectorFormat) -> VRegister {
        VRegister::create_from_format(self.cpu_register.code(), f)
    }

    pub fn is_8b(&self) -> bool {
        self.cpu_register.is_64_bits() && (self.lane_count_ == 8)
    }

    pub fn is_16b(&self) -> bool {
        self.cpu_register.is_128_bits() && (self.lane_count_ == 16)
    }

    pub fn is_4h(&self) -> bool {
        self.cpu_register.is_64_bits() && (self.lane_count_ == 4)
    }

    pub fn is_8h(&self) -> bool {
        self.cpu_register.is_128_bits() && (self.lane_count_ == 8)
    }

    pub fn is_2s(&self) -> bool {
        self.cpu_register.is_64_bits() && (self.lane_count_ == 2)
    }

    pub fn is_4s(&self) -> bool {
        self.cpu_register.is_128_bits() && (self.lane_count_ == 4)
    }

    pub fn is_1d(&self) -> bool {
        self.cpu_register.is_64_bits() && (self.lane_count_ == 1)
    }

    pub fn is_2d(&self) -> bool {
        self.cpu_register.is_128_bits() && (self.lane_count_ == 2)
    }

    pub fn is_1q(&self) -> bool {
        self.cpu_register.is_128_bits() && (self.lane_count_ == 1)
    }

    pub fn is_1b(&self) -> bool {
        if self.cpu_register.is_8_bits() && self.is_vector() {
            return false;
        }
        self.cpu_register.is_8_bits()
    }

    pub fn is_1h(&self) -> bool {
        if self.cpu_register.is_16_bits() && self.is_vector() {
            return false;
        }
        self.cpu_register.is_16_bits()
    }

    pub fn is_1s(&self) -> bool {
        if self.cpu_register.is_32_bits() && self.is_vector() {
            return false;
        }
        self.cpu_register.is_32_bits()
    }

    pub fn is_lane_size_b(&self) -> bool {
        self.lane_size_in_bits() == 8
    }

    pub fn is_lane_size_h(&self) -> bool {
        self.lane_size_in_bits() == 16
    }

    pub fn is_lane_size_s(&self) -> bool {
        self.lane_size_in_bits() == 32
    }

    pub fn is_lane_size_d(&self) -> bool {
        self.lane_size_in_bits() == 64
    }

    pub fn is_scalar(&self) -> bool {
        self.lane_count_ == 1
    }

    pub fn is_vector(&self) -> bool {
        self.lane_count_ > 1
    }

    pub fn is_same_format(&self, other: &VRegister) -> bool {
        self.cpu_register.reg_size_ == other.cpu_register.reg_size_ && self.lane_count_ == other.lane_count_
    }

    pub fn lane_count(&self) -> i32 {
        self.lane_count_ as i32
    }

    pub fn lane_size_in_bytes(&self) -> u32 {
        (self.cpu_register.size_in_bytes() / self.lane_count() ) as u32
    }

    pub fn lane_size_in_bits(&self) -> u32 {
        self.lane_size_in_bytes() * 8
    }
    
    pub const fn code(&self) -> i32 {
        self.cpu_register.code()
    }

    pub const fn from_code(code: i32) -> Self {
        VRegister::create(code, 64, 1)
    }
}

pub const NoReg: Register = Register::no_reg();
pub const NoVReg: VRegister = VRegister::no_reg();
pub const NoCPUReg: CPURegister = CPURegister::no_reg();
pub const no_reg: Register = NoReg;
pub const no_dreg: VRegister = NoVReg;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VectorFormat {
    kFormatUndefined = 0xffffffff,
    kFormat8B = 0,
    kFormat16B = 1,
    kFormat4H = 2,
    kFormat8H = 3,
    kFormat2S = 4,
    kFormat4S = 5,
    kFormat1D = 6,
    kFormat2D = 7,
    kFormatB = 8,
    kFormatH = 9,
    kFormatS = 10,
    kFormatD = 11,
    kFormat1Q = 12,
}

fn vector_format_half_width(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormat8H => VectorFormat::kFormat8B,
        VectorFormat::kFormat4S => VectorFormat::kFormat4H,
        VectorFormat::kFormat2D => VectorFormat::kFormat2S,
        VectorFormat::kFormat1Q => VectorFormat::kFormat1D,
        VectorFormat::kFormatH => VectorFormat::kFormatB,
        VectorFormat::kFormatS => VectorFormat::kFormatH,
        VectorFormat::kFormatD => VectorFormat::kFormatS,
        _ => panic!("Unexpected vector format"),
    }
}

fn vector_format_double_width(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormat8B => VectorFormat::kFormat8H,
        VectorFormat::kFormat4H => VectorFormat::kFormat4S,
        VectorFormat::kFormat2S => VectorFormat::kFormat2D,
        VectorFormat::kFormatB => VectorFormat::kFormatH,
        VectorFormat::kFormatH => VectorFormat::kFormatS,
        VectorFormat::kFormatS => VectorFormat::kFormatD,
        _ => panic!("Unexpected vector format"),
    }
}

fn vector_format_fill_q(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => VectorFormat::kFormat16B,
        VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => VectorFormat::kFormat8H,
        VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => VectorFormat::kFormat4S,
        VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => VectorFormat::kFormat2D,
        _ => panic!("Unexpected vector format"),
    }
}

fn vector_format_half_width_double_lanes(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormat4H => VectorFormat::kFormat8B,
        VectorFormat::kFormat8H => VectorFormat::kFormat16B,
        VectorFormat::kFormat2S => VectorFormat::kFormat4H,
        VectorFormat::kFormat4S => VectorFormat::kFormat8H,
        VectorFormat::kFormat1D => VectorFormat::kFormat2S,
        VectorFormat::kFormat2D => VectorFormat::kFormat4S,
        VectorFormat::kFormat1Q => VectorFormat::kFormat2D,
        _ => panic!("Unexpected vector format"),
    }
}

fn vector_format_double_lanes(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormat8B => VectorFormat::kFormat16B,
        VectorFormat::kFormat4H => VectorFormat::kFormat8H,
        VectorFormat::kFormat2S => VectorFormat::kFormat4S,
        _ => panic!("Unexpected vector format"),
    }
}

fn vector_format_half_lanes(vform: VectorFormat) -> VectorFormat {
    match vform {
        VectorFormat::kFormat16B => VectorFormat::kFormat8B,
        VectorFormat::kFormat8H => VectorFormat::kFormat4H,
        VectorFormat::kFormat4S => VectorFormat::kFormat2S,
        _ => panic!("Unexpected vector format"),
    }
}

fn scalar_format_from_lane_size(lane_size: i32) -> VectorFormat {
    match lane_size {
        8 => VectorFormat::kFormatB,
        16 => VectorFormat::kFormatH,
        32 => VectorFormat::kFormatS,
        64 => VectorFormat::kFormatD,
        _ => panic!("Unexpected lane size"),
    }
}

fn vector_format_fill_q_int(lane_size: i32) -> VectorFormat {
    vector_format_fill_q(scalar_format_from_lane_size(lane_size))
}

fn scalar_format_from_format(vform: VectorFormat) -> VectorFormat {
    scalar_format_from_lane_size(lane_size_in_bits_from_format(vform) as i32)
}

fn register_size_in_bytes_from_format(vform: VectorFormat) -> u32 {
    register_size_in_bits_from_format(vform) / 8
}

fn register_size_in_bits_from_format(vform: VectorFormat) -> u32 {
    match vform {
        VectorFormat::kFormatB => 8,
        VectorFormat::kFormatH => 16,
        VectorFormat::kFormatS => 32,
        VectorFormat::kFormatD => 64,
        VectorFormat::kFormat8B | VectorFormat::kFormat4H | VectorFormat::kFormat2S | VectorFormat::kFormat1D => 64,
        _ => 128,
    }
}

fn lane_size_in_bits_from_format(vform: VectorFormat) -> u32 {
    match vform {
        VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 8,
        VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 16,
        VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 32,
        VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 64,
        VectorFormat::kFormat1Q => 128,
        _ => panic!("Unexpected vector format"),
    }
}

fn lane_size_in_bytes_from_format(vform: VectorFormat) -> i32 {
    (lane_size_in_bits_from_format(vform) / 8) as i32
}

fn lane_size_in_bytes_log2_from_format(vform: VectorFormat) -> i32 {
    match vform {
        VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 0,
        VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 1,
        VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 2,
        VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 3,
        _ => panic!("Unexpected vector format"),
    }
}

fn lane_count_from_format(vform: VectorFormat) -> i32 {
    match vform {
        VectorFormat::kFormat16B => 16,
        VectorFormat::kFormat8B | VectorFormat::kFormat8H => 8,
        VectorFormat::kFormat4H | VectorFormat::kFormat4S => 4,
        VectorFormat::kFormat2S | VectorFormat::kFormat2D => 2,
        VectorFormat::kFormat1D | VectorFormat::kFormat1Q | VectorFormat::kFormatB | VectorFormat::kFormatH | VectorFormat::kFormatS | VectorFormat::kFormatD => 1,
        _ => panic!("Unexpected vector format"),
    }
}

fn max_lane_count_from_format(vform: VectorFormat) -> i32 {
    match vform {
        VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 16,
        VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 8,
        VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 4,
        VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 2,
        _ => panic!("Unexpected vector format"),
    }
}

fn is_vector_format(vform: VectorFormat) -> bool {
    match vform {
        VectorFormat::kFormatB | VectorFormat::kFormatH | VectorFormat::kFormatS | VectorFormat::kFormatD => false,
        _ => true,
    }
}

fn max_int_from_format(vform: VectorFormat) -> i64 {
    i64::max_value() >> (64 - lane_size_in_bits_from_format(vform))
}

fn min_int_from_format(vform: VectorFormat) -> i64 {
    i64::min_value() >> (64 - lane_size_in_bits_from_format(vform))
}

fn max_uint_from_format(vform: VectorFormat) -> u64 {
    u64::max_value() >> (64 - lane_size_in_bits_from_format(vform))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AliasingKind {
    kNone,
    kOverlap,
}

const kWRegSizeInBits: i32 = 32;
const kXRegSizeInBits: i32 = 64;
const kBRegSizeInBits: i32 = 8;
const kHRegSizeInBits: i32 = 16;
const kSRegSizeInBits: i32 = 32;
const kDRegSizeInBits: i32 = 64;
const kQRegSizeInBits: i32 = 128;
const kSPRegInternalCode: i32 = 29;
const kNumberOfRegisters: i32 = 32;
const kNumberOfVRegisters: i32 = 32;
const NEON_8B: i32 = 0;
const NEON_16B: i32 = 1;
const NEON_4H: i32 = 2;
const NEON_8H: i32 = 3;
const NEON_2S: i32 = 4;
const NEON_4S: i32 = 5;
const NEON_1D: i32 = 6;
const NEON_2D: i32 = 7;
const NEON_B: i32 = 8;
const NEON_H: i32 = 9;
const NEON_S: i32 = 10;
const NEON_D: i32 = 11;
const NEONScalar: i32 = 0x80000000;

fn argument_padding_slots(argument_count: i32) -> i32 {
    let k_stack_frame_alignment = 16;
    let alignment_mask = k_stack_frame_alignment / 8 - 1;
    argument_count & alignment_mask
}

pub fn are_aliased(
    reg1: &CPURegister,
    reg2: &CPURegister,
    reg3: &CPURegister,
    reg4: &CPURegister,
    reg5: &CPURegister,
    reg6: &CPURegister,
    reg7: &CPURegister,
    reg8: &CPURegister,
) -> bool {
    let regs = vec![reg1, reg2, reg3, reg4, reg5, reg6, reg7, reg8];
    let mut valid_regs: Vec<&CPURegister> = Vec::new();
    for reg in regs {
        if reg.reg_type_ != RegisterType::kNoRegister {
            valid_regs.push(reg);
        }
    }

    for i in 0..valid_regs.len() {
        for j in (i + 1)..valid_regs.len() {
            if valid_regs[i] == valid_regs[j] && valid_regs[i].reg_type_ != RegisterType::kNoRegister {
                return true;
            }
        }
    }

    false
}

pub fn are_same_size_and_type(
    reg1: &CPURegister,
    reg2: &CPURegister,
    reg3: &CPURegister,
    reg4: &CPURegister,
    reg5: &CPURegister,
    reg6: &CPURegister,
    reg7: &CPURegister,
    reg8: &CPURegister,
) -> bool {
    if reg1.reg_type_ == RegisterType::kNoRegister {
        return true;
    }

    let regs = vec![reg2, reg3, reg4, reg5, reg6, reg7, reg8];
    for reg in regs {
        if reg.reg_type_ != RegisterType::kNoRegister &&
           (reg.reg_size_ != reg1.reg_size_ || reg.reg_type_ != reg1.reg_type_) {
            return false;
        } else if reg.reg_type_ == RegisterType::kNoRegister {
            break;
        }
    }
    true
}

fn are_same_format_register(reg1: &Register, reg2: &Register, reg3: &Register, reg4: &Register) -> bool {
    let regs = vec![reg2, reg3, reg4];
    for reg in regs {
        if reg.reg_type_ != RegisterType::kNoRegister &&
           (reg.reg_size_ != reg1.reg_size_ || reg.reg_type_ != reg1.reg_type_) {
            return false;
        } else if reg.reg_type_ == RegisterType::kNoRegister {
            break;
        }
    }
    true
}

fn are_same_format_vregister(reg1: &VRegister, reg2: &VRegister, reg3: &VRegister, reg4: &VRegister) -> bool {
    let regs = vec![reg2, reg3, reg4];
    for reg in regs {
        if reg.cpu_register.reg_type_ != RegisterType::kNoRegister &&
           (reg.cpu_register.reg_size_ != reg1.cpu_register.reg_size_ || reg.lane_count_ != reg1.lane_count_) {
            return false;
        } else if reg.cpu_register.reg_type_ == RegisterType::kNoRegister {
            break;
        }
    }
    true
}

pub fn are_consecutive(
    reg1: &CPURegister,
    reg2: &CPURegister,
    reg3: &CPURegister,
    reg4: &CPURegister,
) -> bool {
    let regs = vec![reg1, reg2, reg3, reg4];
    let mut valid_regs: Vec<&CPURegister> = Vec::new();
    for reg in regs {
        if reg.reg_type_ != RegisterType::kNoRegister {
            valid_regs.push(reg);
        }
    }

    if valid_regs.is_empty() {
        return true;
    }

    for i in 0..(valid_regs.len() - 1) {
        if valid_regs[i+1].code_

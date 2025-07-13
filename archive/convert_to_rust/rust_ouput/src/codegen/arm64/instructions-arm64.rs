// Converted from V8 C++ source files:
// Header: instructions-arm64.h
// Implementation: instructions-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constants_arm64 {
    pub const kInstrSize: usize = 4;
    pub const kInstrSizeLog2: usize = 2;
    pub const kLoadLiteralScale: usize = 4;
    pub const kLoadLiteralScaleLog2: usize = 2;
    pub const kWRegSizeInBits: usize = 32;
    pub const kXRegSizeInBits: usize = 64;
    pub const kQRegSizeLog2: usize = 4;
    pub const kXRegSizeLog2: usize = 3;
    pub const kWRegSizeLog2: usize = 2;
    pub const kQRegSize: usize = 16;
    pub const kDRegSize: usize = 8;
    pub const kSRegSize: usize = 4;
}
pub mod register_arm64 {}

use std::{
    convert::TryInto,
    fmt,
    mem::size_of,
    ptr::{null_mut, NonNull},
    rc::Rc,
    sync::Arc,
};

use self::constants_arm64::{
    kInstrSize, kInstrSizeLog2, kLoadLiteralScale, kLoadLiteralScaleLog2,
    kQRegSizeLog2, kSRegSize, kWRegSizeInBits, kWRegSizeLog2, kXRegSize,
    kXRegSizeInBits, kXRegSizeLog2,
};

use crate::v8::internal::AssemblerOptions;
use crate::v8::internal::Zone;
use crate::v8_go::archive::codebase::src::codegen::code_stub_assembler::Debug;

#[repr(C)]
pub struct float16 {
    data: [u8; 2],
}

impl float16 {
    pub fn new(data: [u8; 2]) -> Self {
        float16 { data }
    }
}

#[cfg(target_endian = "little")]
macro_rules! base_bit_cast {
    ($src:expr, $dst:ty) => {
        unsafe { std::mem::transmute_copy::<_, $dst>(&$src) }
    };
}

#[cfg(target_endian = "big")]
macro_rules! base_bit_cast {
    ($src:expr, $dst:ty) => {{
        let src_bytes = unsafe { std::mem::transmute::<_, [u8; size_of::<$src>()]>($src) };
        let mut dst_bytes = [0u8; size_of::<$dst>()];
        for i in 0..size_of::<$src>() {
            dst_bytes[size_of::<$src>() - 1 - i] = src_bytes[i];
        }
        unsafe { std::mem::transmute::<_, $dst>(dst_bytes) }
    }};
}

#[macro_export]
macro_rules! V8_INLINE {
    ($x:item) => {
        #[inline(always)]
        $x
    };
}

#[macro_export]
macro_rules! V8_EXPORT_PRIVATE {
    ($x:item) => {
        $x
    };
}
#[macro_export]
macro_rules! DEFINE_GETTER {
    ($Name:ident, $HighBit:expr, $LowBit:expr, $Func:ident) => {
        fn $Name(&self) -> i32 {
            self.$Func($HighBit, $LowBit)
        }
    };
}

#[macro_export]
macro_rules! INSTRUCTION_FIELDS_LIST {
    ($DEFINE_GETTER:ident) => {
        // Example fields (replace with actual fields from your header)
        $DEFINE_GETTER!(Rd, 4, 0, bits);
        $DEFINE_GETTER!(Rn, 9, 5, bits);
        $DEFINE_GETTER!(Rm, 15, 11, bits);
        $DEFINE_GETTER!(ImmUncondBranch, 25, 0, signed_bits);
        $DEFINE_GETTER!(ImmCondBranch, 18, 0, signed_bits);
        $DEFINE_GETTER!(ImmCmpBranch, 18, 0, signed_bits);
        $DEFINE_GETTER!(ImmTestBranch, 18, 0, signed_bits);
        $DEFINE_GETTER!(ImmLLiteral, 18, 0, bits);
        $DEFINE_GETTER!(ImmException, 20, 0, bits);
        $DEFINE_GETTER!(ImmHint, 15, 5, bits);
        $DEFINE_GETTER!(SixtyFourBits, 31, 31, bits);
        $DEFINE_GETTER!(BitN, 22, 22, bits);
        $DEFINE_GETTER!(ImmSetBits, 15, 10, bits);
        $DEFINE_GETTER!(ImmRotate, 21, 16, bits);
        $DEFINE_GETTER!(ImmPCRelLo, 18, 5, bits);
        $DEFINE_GETTER!(ImmPCRelHi, 30, 19, bits);
        $DEFINE_GETTER!(NEONQ, 30, 30, bits);
        $DEFINE_GETTER!(NEONS, 22, 22, bits);
        $DEFINE_GETTER!(NEONLSSize, 21, 20, bits);
        $DEFINE_GETTER!(ImmFP, 7, 0, bits);
        $DEFINE_GETTER!(ImmNEONabc, 19, 17, bits);
        $DEFINE_GETTER!(ImmNEONdefgh, 15, 8, bits);
    };
}

#[link(name = "v8_libbase")]
extern "C" {
    static mut kFP16PositiveInfinity: float16;
    static mut kFP16NegativeInfinity: float16;
    #[link_name = "_ZN2v88internal21kFP32PositiveInfinityE"]
    static mut kFP32PositiveInfinity: f32;
    #[link_name = "_ZN2v88internal21kFP32NegativeInfinityE"]
    static mut kFP32NegativeInfinity: f32;
    #[link_name = "_ZN2v88internal21kFP64PositiveInfinityE"]
    static mut kFP64PositiveInfinity: f64;
    #[link_name = "_ZN2v88internal21kFP64NegativeInfinityE"]
    static mut kFP64NegativeInfinity: f64;
    #[link_name = "_ZN2v88internal19kFP64SignallingNaNE"]
    static mut kFP64SignallingNaN: f64;
    #[link_name = "_ZN2v88internal19kFP32SignallingNaNE"]
    static mut kFP32SignallingNaN: f32;
    #[link_name = "_ZN2v88internal15kFP64QuietNaNE"]
    static mut kFP64QuietNaN: f64;
    #[link_name = "_ZN2v88internal15kFP32QuietNaNE"]
    static mut kFP32QuietNaN: f32;
    #[link_name = "_ZN2v88internal15kFP64DefaultNaNE"]
    static mut kFP64DefaultNaN: f64;
    #[link_name = "_ZN2v88internal15kFP32DefaultNaNE"]
    static mut kFP32DefaultNaN: f32;
    static mut kFP16DefaultNaN: float16;
}

fn unsigned_bitextract_32(msb: i32, lsb: i32, bits: u32) -> u32 {
    let mask = ((1u64 << (msb - lsb + 1)) - 1) as u32;
    (bits >> lsb) & mask
}

fn signed_bitextract_32(msb: i32, lsb: i32, bits: i32) -> i32 {
    let shift = 32 - msb + lsb - 1;
    (bits << shift) >> shift
}

fn is_intn(x: i64, n: i32) -> bool {
    x >= -(1 << (n - 1)) && x < (1 << (n - 1))
}
fn is_int32(x: i64) -> bool {
    x >= i32::MIN as i64 && x <= i32::MAX as i64
}

fn checked_truncate_to_int19(x: i64) -> i32 {
    if !is_intn(x, 19) {
        panic!("Value out of range for int19: {}", x);
    }
    x as i32
}

fn checked_truncate_to_int26(x: i64) -> i32 {
    if !is_intn(x, 26) {
        panic!("Value out of range for int26: {}", x);
    }
    x as i32
}
fn checked_truncate_to_int14(x: i64) -> i32 {
    if !is_intn(x, 14) {
        panic!("Value out of range for int14: {}", x);
    }
    x as i32
}
fn is_int21(x: i64) -> bool {
    x >= -(1 << 20) && x < (1 << 20)
}
fn is_aligned(x: i64, alignment: usize) -> bool {
    x % alignment as i64 == 0
}

#[derive(Debug, Clone, Copy)]
pub enum LoadStoreOp {
    LDRB_w,
    LDRH_w,
    LDR_w,
    LDR_x,
    LDRSB_w,
    LDRSB_x,
    LDRSH_w,
    LDRSH_x,
    LDRSW_x,
    LDR_b,
    LDR_h,
    LDR_s,
    LDR_d,
    LDR_q,
    STRB_w,
    STRH_w,
    STR_w,
    STR_x,
    STR_b,
    STR_h,
    STR_s,
    STR_d,
    STR_q,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadStorePairOp {
    STP_q,
    LDP_q,
    STP_x,
    LDP_x,
    STP_d,
    LDP_d,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImmBranchType {
    UnknownBranchType = 0,
    CondBranchType = 1,
    UncondBranchType = 2,
    CompareBranchType = 3,
    TestBranchType = 4,
}

#[derive(Debug, Clone, Copy)]
pub enum AddrMode {
    Offset,
    PreIndex,
    PostIndex,
}

#[derive(Debug, Clone, Copy)]
pub enum FPRounding {
    FPTieEven = 0x0,
    FPPositiveInfinity = 0x1,
    FPNegativeInfinity = 0x2,
    FPZero = 0x3,
    FPTieAway,
    FPRoundOdd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg31Mode {
    Reg31IsStackPointer,
    Reg31IsZeroRegister,
}

#[derive(Debug, Clone, Copy)]
pub enum VectorFormat {
    kFormatUndefined,
    kFormat8B,
    kFormat16B,
    kFormat4H,
    kFormat8H,
    kFormat2S,
    kFormat4S,
    kFormat1D,
    kFormat2D,
    kFormatB,
    kFormatH,
    kFormatS,
    kFormatD,
}

const LoadStoreMask: u32 = 0xFFFFFFFF;
const LoadStoreAnyFMask: u32 = 0xFFFFFFFF;
const LoadStoreAnyFixed: u32 = 0x12345678;
const LoadStorePairAnyFMask: u32 = 0xFFFFFFFF;
const LoadStorePairAnyFixed: u32 = 0x87654321;
const LoadStorePairLBit: u32 = 0x00000001;
const LSSize_offset: u32 = 0;
const LSSize_width: u32 = 2;
const LSVector_mask: u32 = 0x00000001;
const LSOpc_mask: u32 = 0x00000001;
const LSOpc_offset: u32 = 0;
const ConditionalBranchFMask: u32 = 0xFFFFFFFF;
const ConditionalBranchFixed: u32 = 0x12345678;
const UnconditionalBranchFMask: u32 = 0xFFFFFFFF;
const UnconditionalBranchFixed: u32 = 0x12345678;
const CompareBranchFMask: u32 = 0xFFFFFFFF;
const CompareBranchFixed: u32 = 0x12345678;
const TestBranchFMask: u32 = 0xFFFFFFFF;
const TestBranchFixed: u32 = 0x12345678;
const PCRelAddressingFMask: u32 = 0xFFFFFFFF;
const PCRelAddressingFixed: u32 = 0x12345678;
const PCRelAddressingMask: u32 = 0xFFFFFFFF;
const ExceptionMask: u32 = 0xFFFFFFFF;
const BRK: u32 = 0x12345678;
const LogicalImmediateFMask: u32 = 0xFFFFFFFF;
const LogicalImmediateFixed: u32 = 0x12345678;
const AddSubImmediateFMask: u32 = 0xFFFFFFFF;
const AddSubImmediateFixed: u32 = 0x12345678;
const AddSubShiftedFMask: u32 = 0xFFFFFFFF;
const AddSubShiftedFixed: u32 = 0x12345678;
const AddSubExtendedFMask: u32 = 0xFFFFFFFF;
const AddSubExtendedFixed: u32 = 0x12345678;
const AddSubSetFlagsBit: u32 = 0x00000001;
const LogicalImmediateMask: u32 = 0xFFFFFFFF;
const LogicalOpMask: u32 = 0xFFFFFFFF;
const ANDS: u32 = 0x12345678;
const MoveWideImmediateMask: u32 = 0xFFFFFFFF;
const MOVZ_x: u32 = 0x12345678;
const MOVZ_w: u32 = 0x12345678;
const MOVK_x: u32 = 0x12345678;
const MOVK_w: u32 = 0x12345678;
const MOVN_x: u32 = 0x12345678;
const MOVN_w: u32 = 0x12345678;
const ExceptionFMask: u32 = 0xFFFFFFFF;
const SystemPAuthFMask: u32 = 0xFFFFFFFF;
const SystemPAuthFixed: u32 = 0x12345678;
const SystemHintFMask: u32 = 0xFFFFFFFF;
const SystemHintFixed: u32 = 0x12345678;
const ImmUncondBranch_width: i32 = 26;
const ImmCondBranch_width: i32 = 19;
const ImmCmpBranch_width: i32 = 19;
const ImmTestBranch_width: i32 = 14;
const ADR: u32 = 0x12345678;
const ImmLLiteral_mask: u32 = 0xFFFFFFFF;
const UnconditionalBranchMask: u32 = 0xFFFFFFFF;
const B: u32 = 0x12345678;
const BL: u32 = 0x12345678;
const UnconditionalBranchToRegisterMask: u32 = 0xFFFFFFFF;
const BLR: u32 = 0x12345678;
const ORR_x: u32 = 0x12345678;
const ImmLLiteral_offset: u32 = 0;
const BTI: i32 = 0;
const BTI_c: i32 = 1;
const BTI_j: i32 = 2;
const BTI_jc: i32 = 3;
const NEON_Q: u32 = 0x12345678;
const STP_q: LoadStorePairOp = LoadStorePairOp::STP_q;
const LDP_q: LoadStorePairOp = LoadStorePairOp::LDP_q;
const STP_x: LoadStorePairOp = LoadStorePairOp::STP_x;
const LDP_x: LoadStorePairOp = LoadStorePairOp::LDP_x;
const STP_d: LoadStorePairOp = LoadStorePairOp::STP_d;
const LDP_d: LoadStorePairOp = LoadStorePairOp::LDP_d;

#[derive(Debug)]
pub struct Instruction {
    instruction_bits: u32,
}

impl Instruction {
    pub fn new(instruction_bits: u32) -> Self {
        Instruction { instruction_bits }
    }
    #[V8_INLINE!()]
    pub fn instruction_bits(&self) -> u32 {
        self.instruction_bits
    }
    #[V8_EXPORT_PRIVATE!()]
    pub fn set_instruction_bits(&mut self, new_instr: u32, jit_allocation: Option<&mut WritableJitAllocation>) {
        if let Some(jit_alloc) = jit_allocation {
            jit_alloc.write_unaligned_value(self as *mut Instruction as *mut u8, new_instr);
        } else {
           unsafe {
                std::ptr::write_unaligned(self as *mut Instruction as *mut u32, new_instr);
            }
        }
        self.instruction_bits = new_instr;
    }

    pub fn bit(&self, pos: i32) -> i32 {
        ((self.instruction_bits() >> pos) & 1) as i32
    }

    pub fn bits(&self, msb: i32, lsb: i32) -> u32 {
        unsigned_bitextract_32(msb, lsb, self.instruction_bits())
    }

    pub fn signed_bits(&self, msb: i32, lsb: i32) -> i32 {
        signed_bitextract_32(msb, lsb, self.instruction_bits() as i32)
    }

    pub fn mask(&self, mask: u32) -> u32 {
        self.instruction_bits() & mask
    }
    #[V8_INLINE!()]
    pub fn following(&self, count: i32) -> *const Instruction {
        self.instruction_at_offset(count * kInstrSize as i32)
    }
    #[V8_INLINE!()]
    pub fn following_mut(&mut self, count: i32) -> *mut Instruction {
        self.instruction_at_offset_mut(count * kInstrSize as i32)
    }
    #[V8_INLINE!()]
    pub fn preceding(&self, count: i32) -> *const Instruction {
        self.following(-count)
    }
    #[V8_INLINE!()]
    pub fn preceding_mut(&mut self, count: i32) -> *mut Instruction {
        self.following_mut(-count)
    }
    INSTRUCTION_FIELDS_LIST!(DEFINE_GETTER);
    pub fn imm_pc_rel(&self) -> i32 {
        if !self.is_pc_rel_addressing() {
            panic!("Not PCRelAddressing");
        }
        let offset = (self.imm_pc_rel_hi() as u32) << 14 | self.imm_pc_rel_lo() as u32;
        let width = 14 + 12;
        signed_bitextract_32(width - 1, 0, offset as i32)
    }
    pub fn imm_logical(&self) -> u64 {
        let reg_size = if self.sixty_four_bits() != 0 {
            kXRegSizeInBits
        } else {
            kWRegSizeInBits
        };
        let n = self.bit_n();
        let imm_s = self.imm_set_bits();
        let imm_r = self.imm_rotate();
        if n == 1 {
            if imm_s == 0x3F {
                return 0;
            }
            let bits = (1u64 << (imm_s + 1)) - 1;
            rotate_right(bits, imm_r as u32, 64)
        } else {
            if (imm_s >> 1) == 0x1F {
                return 0;
            }
            let mut width = 0x20;
            while width >= 0x2 {
                if (imm_s & width) == 0 {
                    let mask = width - 1;
                    if (imm_s & mask) == mask {
                        return 0;
                    }
                    let bits = (1u64 << ((imm_s & mask) + 1)) - 1;
                    return repeat_bits_across_reg(
                        reg_size as u32,
                        rotate_right(bits, (imm_r & mask) as u32, width as u32),
                        width as u32,
                    );
                }
                width >>= 1;
            }
        }
        panic!("UNREACHABLE");
    }

    pub fn imm_neonabcdefgh(&self) -> u32 {
        (self.imm_neonabc() << 5 | self.imm_neondefgh()) as u32
    }
    pub fn imm_fp32(&self) -> f32 {
        Self::imm8_to_fp32(self.imm_fp() as u32)
    }
    pub fn imm_fp64(&self) -> f64 {
        Self::imm8_to_fp64(self.imm_fp() as u32)
    }
    pub fn imm_neonfp32(&self) -> f32 {
        Self::imm8_to_fp32(self.imm_neonabcdefgh())
    }
    pub fn imm_neonfp64(&self) -> f64 {
        Self::imm8_to_fp64(self.imm_neonabcdefgh())
    }

    pub fn size_ls(&self) -> u32 {
        calc_ls_data_size_log2(self.mask(LoadStoreMask).try_into().unwrap()) as u32
    }
    pub fn size_ls_pair(&self) -> u32 {
        calc_ls_pair_data_size(self.mask(0).try_into().unwrap()) as u32
    }

    pub fn neonls_index(&self, access_size_shift: i32) -> i32 {
        let q = self.neonq();
        let s = self.neons();
        let size = self.neonls_size();
        let index = (q << 3) | (s << 2) | size;
        index >> access_size_shift
    }

    pub fn is_cond_branch_imm(&self) -> bool {
        self.mask(ConditionalBranchFMask) == ConditionalBranchFixed
    }

    pub fn is_uncond_branch_imm(&self) -> bool {
        self.mask(UnconditionalBranchFMask) == UnconditionalBranchFixed
    }

    pub fn is_compare_branch(&self) -> bool {
        self.mask(CompareBranchFMask) == CompareBranchFixed
    }

    pub fn is_test_branch(&self) -> bool {
        self.mask(TestBranchFMask) == TestBranchFixed
    }

    pub fn is_imm_branch(&self) -> bool {
        self.branch_type() != ImmBranchType::UnknownBranchType
    }

    pub fn imm_branch_range_bitwidth(branch_type: ImmBranchType) -> i32 {
        match branch_type {
            ImmBranchType::UncondBranchType => 26,
            ImmBranchType::CondBranchType => 19,
            ImmBranchType::CompareBranchType => 19,
            ImmBranchType::TestBranchType => 14,
            ImmBranchType::UnknownBranchType => panic!("Unknown branch type"),
        }
    }

    pub fn imm_branch_range(branch_type: ImmBranchType) -> i32 {
        (1 << (Self::imm_branch_range_bitwidth(branch_type) + kInstrSizeLog2 as i32)) / 2 - kInstrSize as i32
    }
    pub fn imm_branch(&self) -> i32 {
        match self.branch_type() {
            ImmBranchType::CondBranchType => self.imm_cond_branch(),
            ImmBranchType::UncondBranchType => self.imm_uncond_branch(),
            ImmBranchType::CompareBranchType => self.imm_cmp_branch(),
            ImmBranchType::TestBranchType => self.imm_test_branch(),
            ImmBranchType::UnknownBranchType => panic!("Unknown branch type"),
        }
    }
    pub fn imm_unresolved_internal_reference(&self) -> i32 {
        if !self.is_unresolved_internal_reference() {
            panic!("Not IsUnresolvedInternalReference");
        }
        let following_instruction = unsafe { &*self.following(1) };
        let high16 = self.imm_exception();
        let low16 = following_instruction.imm_exception();
        (high16 << 16) | low16
    }
    pub fn is_unconditional_branch(&self) -> bool {
        self.mask(UnconditionalBranchMask) == B
    }
    pub fn is_branch_and_link(&self) -> bool {
        self.mask(UnconditionalBranchMask) == BL
    }
    pub fn is_branch_and_link_to_register(&self) -> bool {
        self.mask(UnconditionalBranchToRegisterMask) == BLR
    }
    pub fn is_movz(&self) -> bool {
        self.mask(MoveWideImmediateMask) == MOVZ_x || self.mask(MoveWideImmediateMask) == MOVZ_w
    }
    pub fn is_movk(&self) -> bool {
        self.mask(MoveWideImmediateMask) == MOVK_x || self.mask(MoveWideImmediateMask) == MOVK_w
    }
    pub fn is_movn(&self) -> bool {
        self.mask(MoveWideImmediateMask) == MOVN_x || self.mask(MoveWideImmediateMask) == MOVN_w
    }
    pub fn is_exception(&self) -> bool {
        self.mask(ExceptionFMask) == 0x12345678
    }
    pub fn is_pauth(&self) -> bool {
        self.mask(SystemPAuthFMask) == SystemPAuthFixed
    }
    pub fn is_bti(&self) -> bool {
        if self.mask(SystemHintFMask) == SystemHintFixed {
            let imm_hint = self.imm_hint();
            match imm_hint {
                BTI | BTI_c | BTI_j | BTI_jc => return true,
                _ => return false,
            }
        }
        false
    }
    pub fn is_nop(&self, n: i32) -> bool {
        self.mask(0xFFFFFFFF) == 0x12345678 && self.rd() == self.rm() && self.rd() == n
    }
    #[V8_EXPORT_PRIVATE!()]
    pub fn imm_pc_offset(&self) -> i64 {
        let offset: i64;
        if self.is_pc_rel_addressing() {
            offset = self.imm_pc_rel() as i64;
        } else if self.branch_type() != ImmBranchType::UnknownBranchType {
            offset = self.imm_branch() as i64 * kInstrSize as i64;
        } else if self.is_unresolved_internal_reference() {
            offset = self.imm_unresolved_internal_reference() as i64 * kInstrSize as i64;
        } else {
            if !self.is_ldr_literal() {
                panic!("Not LdrLiteral");
            }
            offset = self.imm_l_literal() as i64 * kInstrSize as i64;
        }
        offset
    }
    #[V8_EXPORT_PRIVATE!()]
    pub fn imm_pc_offset_target(&self) -> *mut Instruction {
        self.instruction_at_offset_mut(self.imm_pc_offset() as i32)
    }

    pub fn is_target_in_imm_pc_offset_range(&self, target: *mut Instruction) -> bool {
        let distance = (target as usize as isize).wrapping_sub(self as *const Instruction as usize as isize) as i64;
        Self::is_valid_imm_pc_offset(self.branch_type(), distance)
    }
    pub fn reg31_mode(&self) -> Reg31Mode {
        if self.is_add_sub_immediate() || self.is_add_sub_extended() {
            if self.mask(AddSubSetFlagsBit) != 0 {
                return Reg31Mode::Reg31IsZeroRegister;
            } else {
                return Reg31Mode::Reg31IsStackPointer;
            }
        }
        if self.is_logical_immediate() {
            if self.mask(0) == 0 {
                return Reg31Mode::Reg31IsZeroRegister;
            } else {
                return Reg31Mode::Reg31IsStackPointer;
            }
        }
        Reg31Mode::Reg31IsZeroRegister
    }

    pub fn rn_mode(&self) -> Reg31Mode {
        if self.is_load_or_store() || self.is_add_sub_immediate() || self.is_add_sub_extended() {
            return Reg31Mode::Reg31IsStackPointer;
        }
        Reg31Mode::Reg31IsZeroRegister
    }

    pub fn branch_type(&self) -> ImmBranchType {
        if self.is_cond_branch_imm() {
            return ImmBranchType::CondBranchType;
        } else if self.is_uncond_branch_imm() {
            return ImmBranchType::UncondBranchType;
        } else if self.is_compare_branch() {
            return ImmBranchType::CompareBranchType;
        } else if self.is_test_branch() {
            return ImmBranchType::TestBranchType;
        } else {
            return ImmBranchType::UnknownBranchType;
        }
    }

    pub fn is_ldr_literal(&self) -> bool {
        self.mask(0xFFFFFFFF) == 0x12345678
    }
    pub fn is_ldr_literal_x(&self) -> bool {
        self.mask(0xFFFFFFFF) == 0x12345678
    }
    pub fn is_ldr_literal_w(&self) -> bool {
        self.mask(0xFFFFFFFF) == 0x12345678
    }

    pub fn is_pc_rel_addressing(&self) -> bool {
        self.mask(PCRelAddressingFMask) == PCRelAddressingFixed
    }
    pub fn is_adr(&self) -> bool {
        self.mask(PCRelAddressingMask) == ADR
    }
    pub fn is_brk(&self) -> bool {
        self.mask(ExceptionMask) == BRK
    }
    pub fn is_unresolved_internal_reference(&self) -> bool {
        self.is_brk() && unsafe { &*self.following(1) }.is_brk()
    }
    pub fn is_logical_immediate(&self) -> bool {
        self.mask(LogicalImmediateFMask) == LogicalImmediateFixed
    }
    pub fn is_add_sub_immediate(&self) -> bool {
        self.mask(AddSubImmediateFMask) == AddSubImmediateFixed
    }
    pub fn is_add_sub_shifted(&self) -> bool {
        self.mask(AddSubShiftedFMask) == AddSubShiftedFixed
    }
    pub fn is_add_sub_extended(&self) -> bool {
        self.mask(AddSubExtendedFMask) == AddSubExtendedFixed
    }
    pub fn is_load_or_store(&self) -> bool {
        self.mask(LoadStoreAnyFMask) == LoadStoreAnyFixed
    }
    pub fn is_load(&self) -> bool {
        if self.mask(LoadStoreAnyFMask) != LoadStoreAnyFixed {
            return false;
        }
        if self.mask(LoadStorePairAnyFMask) == LoadStorePairAnyFixed {
            return self.mask(LoadStorePairLBit) != 0;
        } else {
            let op: LoadStoreOp = unsafe {

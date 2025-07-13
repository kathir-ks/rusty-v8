// Converted from V8 C++ source files:
// Header: constants-arm64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod macros {}
}
pub mod common {
pub mod globals {}
}
#[cfg(target_arch = "aarch64")]
static_assert!(std::mem::size_of::<i32>() == 4);
#[cfg(target_arch = "aarch64")]
#[cfg(target_os = "windows")]
static_assert!(std::mem::size_of::<i32>() == 4);
#[cfg(target_arch = "aarch64")]
#[cfg(not(target_os = "windows"))]
static_assert!(std::mem::size_of::<i64>() == 8);
#[cfg(target_arch = "aarch64")]
#[cfg(not(target_os = "windows"))]
static_assert!(std::mem::size_of::<i64>() == 8);
#[cfg(target_arch = "aarch64")]
static_assert!(std::mem::size_of::<*const std::ffi::c_void>() == 8);
#[cfg(target_arch = "aarch64")]
static_assert!(std::mem::size_of::<i32>() == 4);
pub mod v8 {
pub mod internal {
    pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 128;
    pub const K_INSTR_SIZE: u8 = 4;
    pub const K_INSTR_SIZE_LOG2: u8 = 2;
    pub const K_LOAD_LITERAL_SCALE_LOG2: u8 = 2;
    pub const K_LOAD_LITERAL_SCALE: u8 = 1 << K_LOAD_LITERAL_SCALE_LOG2;
    pub const K_MAX_LOAD_LITERAL_RANGE: i32 = 1 * 1024 * 1024;
    pub const K_NUMBER_OF_REGISTERS: i32 = 32;
    pub const K_NUMBER_OF_VREGISTERS: i32 = 32;
    pub const K_NUMBER_OF_CALLEE_SAVED_REGISTERS: i32 = 10;
    pub const K_NUMBER_OF_CALLEE_SAVED_VREGISTERS: i32 = 8;
    pub const K_WREG_SIZE_IN_BITS: i32 = 32;
    pub const K_WREG_SIZE_IN_BITS_LOG2: i32 = 5;
    pub const K_WREG_SIZE: i32 = K_WREG_SIZE_IN_BITS >> 3;
    pub const K_WREG_SIZE_LOG2: i32 = K_WREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_XREG_SIZE_IN_BITS: i32 = 64;
    pub const K_XREG_SIZE_IN_BITS_LOG2: i32 = 6;
    pub const K_XREG_SIZE: i32 = K_XREG_SIZE_IN_BITS >> 3;
    pub const K_XREG_SIZE_LOG2: i32 = K_XREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_SREG_SIZE_IN_BITS: i32 = 32;
    pub const K_SREG_SIZE_IN_BITS_LOG2: i32 = 5;
    pub const K_SREG_SIZE: i32 = K_SREG_SIZE_IN_BITS >> 3;
    pub const K_SREG_SIZE_LOG2: i32 = K_SREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_DREG_SIZE_IN_BITS: i32 = 64;
    pub const K_DREG_SIZE_IN_BITS_LOG2: i32 = 6;
    pub const K_DREG_SIZE: i32 = K_DREG_SIZE_IN_BITS >> 3;
    pub const K_DREG_SIZE_LOG2: i32 = K_DREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_DREG_SIZE_IN_BYTES_LOG2: i32 = K_DREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_BREG_SIZE_IN_BITS: i32 = 8;
    pub const K_BREG_SIZE: i32 = K_BREG_SIZE_IN_BITS >> 3;
    pub const K_HREG_SIZE_IN_BITS: i32 = 16;
    pub const K_HREG_SIZE: i32 = K_HREG_SIZE_IN_BITS >> 3;
    pub const K_QREG_SIZE_IN_BITS: i32 = 128;
    pub const K_QREG_SIZE_IN_BITS_LOG2: i32 = 7;
    pub const K_QREG_SIZE: i32 = K_QREG_SIZE_IN_BITS >> 3;
    pub const K_QREG_SIZE_LOG2: i32 = K_QREG_SIZE_IN_BITS_LOG2 - 3;
    pub const K_VREG_SIZE_IN_BITS: i32 = K_QREG_SIZE_IN_BITS;
    pub const K_VREG_SIZE: i32 = K_VREG_SIZE_IN_BITS >> 3;
    pub const K_WREG_MASK: i64 = 0x00000000ffffffff;
    pub const K_XREG_MASK: i64 = 0xffffffffffffffff;
    pub const K_SREG_MASK: i64 = 0x00000000ffffffff;
    pub const K_DREG_MASK: i64 = 0xffffffffffffffff;
    pub const K_D_SIGN_BIT: i64 = 63;
    pub const K_D_SIGN_MASK: i64 = 0x1 << K_D_SIGN_BIT;
    pub const K_S_SIGN_BIT: i64 = 31;
    pub const K_S_SIGN_MASK: i64 = 0x1 << K_S_SIGN_BIT;
    pub const K_X_SIGN_BIT: i64 = 63;
    pub const K_X_SIGN_MASK: i64 = 0x1 << K_X_SIGN_BIT;
    pub const K_W_SIGN_BIT: i64 = 31;
    pub const K_W_SIGN_MASK: i64 = 0x1 << K_W_SIGN_BIT;
    pub const K_D_QUIET_NAN_BIT: i64 = 51;
    pub const K_D_QUIET_NAN_MASK: i64 = 0x1 << K_D_QUIET_NAN_BIT;
    pub const K_S_QUIET_NAN_BIT: i64 = 22;
    pub const K_S_QUIET_NAN_MASK: i64 = 0x1 << K_S_QUIET_NAN_BIT;
    pub const K_H_QUIET_NAN_BIT: i64 = 9;
    pub const K_H_QUIET_NAN_MASK: i64 = 0x1 << K_H_QUIET_NAN_BIT;
    pub const K_BYTE_MASK: i64 = 0xff;
    pub const K_HALF_WORD_MASK: i64 = 0xffff;
    pub const K_WORD_MASK: i64 = 0xffffffff;
    pub const K_X_MAX_UINT: u64 = 0xffffffffffffffff;
    pub const K_W_MAX_UINT: u64 = 0xffffffff;
    pub const K_X_MAX_INT: i64 = 0x7fffffffffffffff;
    pub const K_X_MIN_INT: i64 = 0x8000000000000000;
    pub const K_W_MAX_INT: i32 = 0x7fffffff;
    pub const K_W_MIN_INT: i32 = 0x80000000;
    pub const K_IP0_CODE: i32 = 16;
    pub const K_IP1_CODE: i32 = 17;
    pub const K_FRAME_POINTER_REG_CODE: i32 = 29;
    pub const K_LINK_REG_CODE: i32 = 30;
    pub const K_ZERO_REG_CODE: i32 = 31;
    pub const K_SP_REG_INTERNAL_CODE: i32 = 63;
    pub const K_REG_CODE_MASK: u32 = 0x1f;
    pub const K_SHIFT_AMOUNT_WREG_MASK: u32 = 0x1f;
    pub const K_SHIFT_AMOUNT_XREG_MASK: u32 = 0x3f;
    pub const K_HALF_WORD_SIZE: u32 = 16;
    pub const K_HALF_WORD_SIZE_LOG2: u32 = 4;
    pub const K_HALF_WORD_SIZE_IN_BYTES: u32 = K_HALF_WORD_SIZE >> 3;
    pub const K_HALF_WORD_SIZE_IN_BYTES_LOG2: u32 = K_HALF_WORD_SIZE_LOG2 - 3;
    pub const K_WORD_SIZE: u32 = 32;
    pub const K_WORD_SIZE_LOG2: u32 = 5;
    pub const K_WORD_SIZE_IN_BYTES: u32 = K_WORD_SIZE >> 3;
    pub const K_WORD_SIZE_IN_BYTES_LOG2: u32 = K_WORD_SIZE_LOG2 - 3;
    pub const K_DOUBLE_WORD_SIZE: u32 = 64;
    pub const K_DOUBLE_WORD_SIZE_IN_BYTES: u32 = K_DOUBLE_WORD_SIZE >> 3;
    pub const K_QUAD_WORD_SIZE: u32 = 128;
    pub const K_QUAD_WORD_SIZE_IN_BYTES: u32 = K_QUAD_WORD_SIZE >> 3;
    pub const K_MAX_LANES_PER_VECTOR: i32 = 16;
    pub const K_ADDRESS_TAG_OFFSET: u32 = 56;
    pub const K_ADDRESS_TAG_WIDTH: u32 = 8;
    pub const K_ADDRESS_TAG_MASK: u64 =
        ((1u64 << K_ADDRESS_TAG_WIDTH) - 1) << K_ADDRESS_TAG_OFFSET;
    pub const K_TTBR_MASK: u64 = 1 << 55;
    pub const K_DOUBLE_MANTISSA_BITS: u32 = 52;
    pub const K_DOUBLE_EXPONENT_BITS: u32 = 11;
    pub const K_DOUBLE_EXPONENT_BIAS: u32 = 1023;
    pub const K_FLOAT_MANTISSA_BITS: u32 = 23;
    pub const K_FLOAT_EXPONENT_BITS: u32 = 8;
    pub const K_FLOAT_EXPONENT_BIAS: u32 = 127;
    pub const K_FLOAT16_MANTISSA_BITS: u32 = 10;
    pub const K_FLOAT16_EXPONENT_BITS: u32 = 5;
    pub const K_FLOAT16_EXPONENT_BIAS: u32 = 15;
    pub const K_ROOT_REGISTER_BIAS: i32 = 256;
    pub type Float16 = u16;
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Condition {
        Eq = 0,
        Ne = 1,
        Hs = 2,
        Cs = 2,
        Lo = 3,
        Cc = 3,
        Mi = 4,
        Pl = 5,
        Vs = 6,
        Vc = 7,
        Hi = 8,
        Ls = 9,
        Ge = 10,
        Lt = 11,
        Gt = 12,
        Le = 13,
        Al = 14,
        Nv = 15,
        KEqual = 0,
        KNotEqual = 1,
        KLessThan = 11,
        KGreaterThan = 12,
        KLessThanEqual = 13,
        KGreaterThanEqual = 10,
        KUnsignedLessThan = 3,
        KUnsignedGreaterThan = 8,
        KUnsignedLessThanEqual = 9,
        KUnsignedGreaterThanEqual = 2,
        KOverflow = 6,
        KNoOverflow = 7,
        KZero = 0,
        KNotZero = 1,
    }
    #[inline]
    pub fn negate_condition(cond: Condition) -> Condition {
        assert!((cond != Condition::Al) && (cond != Condition::Nv));
        unsafe { std::mem::transmute((cond as i32) ^ 1) }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FlagsUpdate {
        SetFlags = 1,
        LeaveFlags = 0,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StatusFlags {
        NoFlag = 0,
        NFlag = N_MASK,
        ZFlag = Z_MASK,
        CFlag = C_MASK,
        VFlag = V_MASK,
        NZFlag = N_MASK | Z_MASK,
        NCFlag = N_MASK | C_MASK,
        NVFlag = N_MASK | V_MASK,
        ZCFlag = Z_MASK | C_MASK,
        ZVFlag = Z_MASK | V_MASK,
        CVFlag = C_MASK | V_MASK,
        NZCFlag = N_MASK | Z_MASK | C_MASK,
        NZVFlag = N_MASK | Z_MASK | V_MASK,
        NCVFlag = N_MASK | C_MASK | V_MASK,
        ZCVFlag = Z_MASK | C_MASK | V_MASK,
        NZCVFlag = N_MASK | Z_MASK | C_MASK | V_MASK,
        FPEqualFlag = Z_MASK | C_MASK,
        FPLessThanFlag = N_MASK,
        FPGreaterThanFlag = C_MASK,
        FPUnorderedFlag = C_MASK | V_MASK,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Shift {
        NoShift = -1,
        LSL = 0x0,
        LSR = 0x1,
        ASR = 0x2,
        ROR = 0x3,
        MSL = 0x4,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Extend {
        NoExtend = -1,
        UXTB = 0,
        UXTH = 1,
        UXTW = 2,
        UXTX = 3,
        SXTB = 4,
        SXTH = 5,
        SXTW = 6,
        SXTX = 7,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SystemHint {
        NOP = 0,
        YIELD = 1,
        WFE = 2,
        WFI = 3,
        SEV = 4,
        SEVL = 5,
        CSDB = 20,
        BTI = 32,
        BTI_c = 34,
        BTI_j = 36,
        BTI_jc = 38,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BranchTargetIdentifier {
        kNone,
        kBti,
        kBtiCall,
        kBtiJump,
        kBtiJumpCall,
        kPacibsp,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BarrierDomain {
        OuterShareable = 0,
        NonShareable = 1,
        InnerShareable = 2,
        FullSystem = 3,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BarrierType {
        BarrierOther = 0,
        BarrierReads = 1,
        BarrierWrites = 2,
        BarrierAll = 3,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SystemRegister {
        NZCV = (((0x1 << SYS_O0_OFFSET)
            | (0x3 << SYS_OP1_OFFSET)
            | (0x4 << CRN_OFFSET)
            | (0x2 << CRM_OFFSET)
            | (0x0 << SYS_OP2_OFFSET))
            >> IMM_SYSTEM_REGISTER_OFFSET),
        FPCR = (((0x1 << SYS_O0_OFFSET)
            | (0x3 << SYS_OP1_OFFSET)
            | (0x4 << CRN_OFFSET)
            | (0x4 << CRM_OFFSET)
            | (0x0 << SYS_OP2_OFFSET))
            >> IMM_SYSTEM_REGISTER_OFFSET),
    }
    pub type GenericInstrField = u32;
    pub const SIXTY_FOUR_BITS: GenericInstrField = 0x80000000;
    pub const THIRTY_TWO_BITS: GenericInstrField = 0x00000000;
    pub const FP32: GenericInstrField = 0x00000000;
    pub const FP64: GenericInstrField = 0x00400000;
    pub type NeonFormatField = u32;
    pub const NEON_FORMAT_FIELD_MASK: NeonFormatField = 0x40C00000;
    pub const NEON_Q: NeonFormatField = 0x40000000;
    pub const NEON_SZ: NeonFormatField = 0x00400000;
    pub const NEON_8B: NeonFormatField = 0x00000000;
    pub const NEON_16B: NeonFormatField = NEON_8B | NEON_Q;
    pub const NEON_4H: NeonFormatField = 0x00400000;
    pub const NEON_8H: NeonFormatField = NEON_4H | NEON_Q;
    pub const NEON_2S: NeonFormatField = 0x00800000;
    pub const NEON_4S: NeonFormatField = NEON_2S | NEON_Q;
    pub const NEON_1D: NeonFormatField = 0x00C00000;
    pub const NEON_2D: NeonFormatField = 0x00C00000 | NEON_Q;
    pub type NeonFpFormatField = u32;
    pub const NEON_FP_FORMAT_FIELD_MASK: NeonFpFormatField = 0x40400000;
    pub const NEON_FP_4H: NeonFpFormatField = 0x00000000;
    pub const NEON_FP_8H: NeonFpFormatField = NEON_Q;
    pub const NEON_FP_2S: NeonFpFormatField = FP32;
    pub const NEON_FP_4S: NeonFpFormatField = FP32 | NEON_Q;
    pub const NEON_FP_2D: NeonFpFormatField = FP64 | NEON_Q;
    pub type NeonLsFormatField = u32;
    pub const NEON_LS_FORMAT_FIELD_MASK: NeonLsFormatField = 0x40000C00;
    pub const LS_NEON_8B: NeonLsFormatField = 0x00000000;
    pub const LS_NEON_16B: NeonLsFormatField = LS_NEON_8B | NEON_Q;
    pub const LS_NEON_4H: NeonLsFormatField = 0x00000400;
    pub const LS_NEON_8H: NeonLsFormatField = LS_NEON_4H | NEON_Q;
    pub const LS_NEON_2S: NeonLsFormatField = 0x00000800;
    pub const LS_NEON_4S: NeonLsFormatField = LS_NEON_2S | NEON_Q;
    pub const LS_NEON_1D: NeonLsFormatField = 0x00000C00;
    pub const LS_NEON_2D: NeonLsFormatField = LS_NEON_1D | NEON_Q;
    pub type NeonScalarFormatField = u32;
    pub const NEON_SCALAR_FORMAT_FIELD_MASK: NeonScalarFormatField = 0x00C00000;
    pub const NEON_SCALAR: NeonScalarFormatField = 0x10000000;
    pub const NEON_B: NeonScalarFormatField = 0x00000000;
    pub const NEON_H: NeonScalarFormatField = 0x00400000;
    pub const NEON_S: NeonScalarFormatField = 0x00800000;
    pub const NEON_D: NeonScalarFormatField = 0x00C00000;
    pub type PCRelAddressingOp = u32;
    pub const PC_REL_ADDRESSING_FIXED: PCRelAddressingOp = 0x10000000;
    pub const PC_REL_ADDRESSING_FMASK: PCRelAddressingOp = 0x1F000000;
    pub const PC_REL_ADDRESSING_MASK: PCRelAddressingOp = 0x9F000000;
    pub const ADR: PCRelAddressingOp = PC_REL_ADDRESSING_FIXED | 0x00000000;
    pub const ADRP: PCRelAddressingOp = PC_REL_ADDRESSING_FIXED | 0x80000000;
    pub const K_SF_OFFSET: i32 = 31;
    pub type AddSubOp = u32;
    pub const ADD_SUB_OP_MASK: AddSubOp = 0x60000000;
    pub const ADD_SUB_SET_FLAGS_BIT: AddSubOp = 0x20000000;
    pub const ADD: AddSubOp = 0x00000000;
    pub const ADDS: AddSubOp = ADD | ADD_SUB_SET_FLAGS_BIT;
    pub const SUB: AddSubOp = 0x40000000;
    pub const SUBS: AddSubOp = SUB | ADD_SUB_SET_FLAGS_BIT;
    pub type AddSubImmediateOp = u32;
    pub const ADD_SUB_IMMEDIATE_FIXED: AddSubImmediateOp = 0x11000000;
    pub const ADD_SUB_IMMEDIATE_FMASK: AddSubImmediateOp = 0x1F000000;
    pub const ADD_SUB_IMMEDIATE_MASK: AddSubImmediateOp = 0xFF000000;
    pub const ADD_W_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | ADD;
    pub const ADD_X_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | ADD | SIXTY_FOUR_BITS;
    pub const ADDS_W_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | ADDS;
    pub const ADDS_X_IMM: AddSubImmediateOp =
        ADD_SUB_IMMEDIATE_FIXED | ADDS | SIXTY_FOUR_BITS;
    pub const SUB_W_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | SUB;
    pub const SUB_X_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | SUB | SIXTY_FOUR_BITS;
    pub const SUBS_W_IMM: AddSubImmediateOp = ADD_SUB_IMMEDIATE_FIXED | SUBS;
    pub const SUBS_X_IMM: AddSubImmediateOp =
        ADD_SUB_IMMEDIATE_FIXED | SUBS | SIXTY_FOUR_BITS;
    pub type AddSubShiftedOp = u32;
    pub const ADD_SUB_SHIFTED_FIXED: AddSubShiftedOp = 0x0B000000;
    pub const ADD_SUB_SHIFTED_FMASK: AddSubShiftedOp = 0x1F200000;
    pub const ADD_SUB_SHIFTED_MASK: AddSubShiftedOp = 0xFF200000;
    pub const ADD_W_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | ADD;
    pub const ADD_X_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | ADD | SIXTY_FOUR_BITS;
    pub const ADDS_W_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | ADDS;
    pub const ADDS_X_SHIFT: AddSubShiftedOp =
        ADD_SUB_SHIFTED_FIXED | ADDS | SIXTY_FOUR_BITS;
    pub const SUB_W_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | SUB;
    pub const SUB_X_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | SUB | SIXTY_FOUR_BITS;
    pub const SUBS_W_SHIFT: AddSubShiftedOp = ADD_SUB_SHIFTED_FIXED | SUBS;
    pub const SUBS_X_SHIFT: AddSubShiftedOp =
        ADD_SUB_SHIFTED_FIXED | SUBS | SIXTY_FOUR_BITS;
    pub type AddSubExtendedOp = u32;
    pub const ADD_SUB_EXTENDED_FIXED: AddSubExtendedOp = 0x0B200000;
    pub const ADD_SUB_EXTENDED_FMASK: AddSubExtendedOp = 0x1F200000;
    pub const ADD_SUB_EXTENDED_MASK: AddSubExtendedOp = 0xFFE00000;
    pub const ADD_W_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | ADD;
    pub const ADD_X_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | ADD | SIXTY_FOUR_BITS;
    pub const ADDS_W_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | ADDS;
    pub const ADDS_X_EXT: AddSubExtendedOp =
        ADD_SUB_EXTENDED_FIXED | ADDS | SIXTY_FOUR_BITS;
    pub const SUB_W_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | SUB;
    pub const SUB_X_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | SUB | SIXTY_FOUR_BITS;
    pub const SUBS_W_EXT: AddSubExtendedOp = ADD_SUB_EXTENDED_FIXED | SUBS;
    pub const SUBS_X_EXT: AddSubExtendedOp =
        ADD_SUB_EXTENDED_FIXED | SUBS | SIXTY_FOUR_BITS;
    pub type AddSubWithCarryOp = u32;
    pub const ADD_SUB_WITH_CARRY_FIXED: AddSubWithCarryOp = 0x1A000000;
    pub const ADD_SUB_WITH_CARRY_FMASK: AddSubWithCarryOp = 0x1FE00000;
    pub const ADD_SUB_WITH_CARRY_MASK: AddSubWithCarryOp = 0xFFE0FC00;
    pub const ADC_W: AddSubWithCarryOp = ADD_SUB_WITH_CARRY_FIXED | ADD;
    pub const ADC_X: AddSubWithCarryOp =
        ADD_SUB_WITH_CARRY_FIXED | ADD | SIXTY_FOUR_BITS;
    pub const ADC: AddSubWithCarryOp = ADC_W;
    pub const ADCS_W: AddSubWithCarryOp = ADD_SUB_WITH_CARRY_FIXED | ADDS;
    pub const ADCS_X: AddSubWithCarryOp =
        ADD_SUB_WITH_CARRY_FIXED | ADDS | SIXTY_FOUR_BITS;
    pub const SBC_W: AddSubWithCarryOp = ADD_SUB_WITH_CARRY_FIXED | SUB;
    pub const SBC_X: AddSubWithCarryOp =
        ADD_SUB_WITH_CARRY_FIXED | SUB | SIXTY_FOUR_BITS;
    pub const SBC: AddSubWithCarryOp = SBC_W;
    pub const SBCS_W: AddSubWithCarryOp = ADD_SUB_WITH_CARRY_FIXED | SUBS;
    pub const SBCS_X: AddSubWithCarryOp =
        ADD_SUB_WITH_CARRY_FIXED | SUBS | SIXTY_FOUR_BITS;
    pub type LogicalOp = u32;
    pub const LOGICAL_OP_MASK: LogicalOp = 0x60200000;
    pub const NOT: LogicalOp = 0x00200000;
    pub const AND: LogicalOp = 0x00000000;
    pub const BIC: LogicalOp = AND | NOT;
    pub const ORR: LogicalOp = 0x20000000;
    pub const ORN: LogicalOp = ORR | NOT;
    pub const EOR: LogicalOp = 0x40000000;
    pub const EON: LogicalOp = EOR | NOT;
    pub const ANDS: LogicalOp = 0x60000000;
    pub const BICS: LogicalOp = ANDS | NOT;
    pub type LogicalImmediateOp = u32;
    pub const LOGICAL_IMMEDIATE_FIXED: LogicalImmediateOp = 0x12000000;
    pub const LOGICAL_IMMEDIATE_FMASK: LogicalImmediateOp = 0x1F800000;
    pub const LOGICAL_IMMEDIATE_MASK: LogicalImmediateOp = 0xFF800000;
    pub const AND_W_IMM: LogicalImmediateOp = LOGICAL_IMMEDIATE_FIXED | AND;
    pub const AND_X_IMM: LogicalImmediateOp =
        LOGICAL_IMMEDIATE_FIXED | AND | SIXTY_FOUR_BITS;
    pub const ORR_W_IMM: LogicalImmediateOp = LOGICAL_IMMEDIATE_FIXED | ORR;
    pub const ORR_X_IMM: LogicalImmediateOp =
        LOGICAL_IMMEDIATE_FIXED | ORR | SIXTY_FOUR_BITS;
    pub const EOR_W_IMM: LogicalImmediateOp = LOGICAL_IMMEDIATE_FIXED | EOR;
    pub const EOR_X_IMM: LogicalImmediateOp =
        LOGICAL_IMMEDIATE_FIXED | EOR | SIXTY_FOUR_BITS;
    pub const ANDS_W_IMM: LogicalImmediateOp = LOGICAL_IMMEDIATE_FIXED | ANDS;
    pub const ANDS_X_IMM: LogicalImmediateOp =
        LOGICAL_IMMEDIATE_FIXED | ANDS | SIXTY_FOUR_BITS;
    pub type LogicalShiftedOp = u32;
    pub const LOGICAL_SHIFTED_FIXED: LogicalShiftedOp = 0x0A000000;
    pub const LOGICAL_SHIFTED_FMASK: LogicalShiftedOp = 0x1F000000;
    pub const LOGICAL_SHIFTED_MASK: LogicalShiftedOp = 0xFF200000;
    pub const AND_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED | AND;
    pub const AND_X: LogicalShiftedOp =
        LOGICAL_SHIFTED_FIXED | AND | SIXTY_FOUR_BITS;
    pub const AND_SHIFT: LogicalShiftedOp = AND_W;
    pub const BIC_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED | BIC;
    pub const BIC_X: LogicalShiftedOp =
        LOGICAL_SHIFTED_FIXED | BIC | SIXTY_FOUR_BITS;
    pub const BIC_SHIFT: LogicalShiftedOp = BIC_W;
    pub const ORR_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED | ORR;
    pub const ORR_X: LogicalShiftedOp =
        LOGICAL_SHIFTED_FIXED | ORR | SIXTY_FOUR_BITS;
    pub const ORR_SHIFT: LogicalShiftedOp = ORR_W;
    pub const ORN_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED | ORN;
    pub const ORN_X: LogicalShiftedOp =
        LOGICAL_SHIFTED_FIXED | ORN | SIXTY_FOUR_BITS;
    pub const ORN_SHIFT: LogicalShiftedOp = ORN_W;
    pub const EOR_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED | EOR;
    pub const EOR_X: LogicalShiftedOp =
        LOGICAL_SHIFTED_FIXED | EOR | SIXTY_FOUR_BITS;
    pub const EOR_SHIFT: LogicalShiftedOp = EOR_W;
    pub const EON_W: LogicalShiftedOp = LOGICAL_SHIFTED_FIXED

// Converted from V8 C++ source files:
// Header: constants-loong64.h
// Implementation: constants-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub const kMaxPCRelativeCodeRangeInMB: usize = 128;

pub const kNumRegisters: i32 = 32;
pub const kInvalidRegister: i32 = -1;

pub const kNumSimuRegisters: i32 = 33;

pub const kPCRegister: i32 = 32;

pub const kNumFPURegisters: i32 = 32;
pub const kInvalidFPURegister: i32 = -1;

pub const kFCSRRegister: i32 = 0;
pub const kInvalidFPUControlRegister: i32 = -1;
pub const kFPUInvalidResult: u32 = (1u32 << 31) - 1;
pub const kFPUInvalidResultNegative: i32 = 1i32 << 31;
pub const kFPU64InvalidResult: u64 = (1u64 << 63) - 1;
pub const kFPU64InvalidResultNegative: i64 = 1i64 << 63;

pub const kFCSRInexactCauseBit: u32 = 24;
pub const kFCSRUnderflowCauseBit: u32 = 25;
pub const kFCSROverflowCauseBit: u32 = 26;
pub const kFCSRDivideByZeroCauseBit: u32 = 27;
pub const kFCSRInvalidOpCauseBit: u32 = 28;

pub const kFCSRInexactCauseMask: u32 = 1 << kFCSRInexactCauseBit;
pub const kFCSRUnderflowCauseMask: u32 = 1 << kFCSRUnderflowCauseBit;
pub const kFCSROverflowCauseMask: u32 = 1 << kFCSROverflowCauseBit;
pub const kFCSRDivideByZeroCauseMask: u32 = 1 << kFCSRDivideByZeroCauseBit;
pub const kFCSRInvalidOpCauseMask: u32 = 1 << kFCSRInvalidOpCauseBit;

pub const kFCSRCauseMask: u32 = kFCSRInexactCauseMask
    | kFCSRUnderflowCauseMask
    | kFCSROverflowCauseMask
    | kFCSRDivideByZeroCauseMask
    | kFCSRInvalidOpCauseMask;

pub const kFCSRExceptionCauseMask: u32 = kFCSRCauseMask ^ kFCSRInexactCauseMask;

pub const kRootRegisterBias: i32 = 256;

pub struct Registers {}

impl Registers {
    pub fn name(reg: i32) -> &'static str {
        if (0 <= reg) && (reg < kNumSimuRegisters) {
            Registers::NAMES[reg as usize]
        } else {
            "noreg"
        }
    }

    pub fn number(name: &str) -> i32 {
        for (i, &reg_name) in Registers::NAMES.iter().enumerate() {
            if reg_name == name {
                return i as i32;
            }
        }

        for alias in Registers::ALIASES.iter() {
            if let Some(alias_name) = alias.name {
                if alias_name == name {
                    return alias.reg;
                }
            }
        }

        kInvalidRegister
    }

    pub const kMaxValue: i64 = 0x7fffffffffffffffl;
    pub const kMinValue: i64 = 0x8000000000000000l;

    const NAMES: [&'static str; kNumSimuRegisters as usize] = [
        "zero_reg", "ra", "tp", "sp", "a0", "a1", "a2", "a3", "a4", "a5", "a6",
        "a7", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8", "x_reg",
        "fp", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "pc",
    ];

    const ALIASES: [RegisterAlias; 3] = [
        RegisterAlias { reg: 0, name: Some("zero") },
        RegisterAlias { reg: 30, name: Some("cp") },
        RegisterAlias { reg: kInvalidRegister, name: None },
    ];
}

#[derive(Copy, Clone)]
pub struct RegisterAlias {
    reg: i32,
    name: Option<&'static str>,
}

pub struct FPURegisters {}

impl FPURegisters {
    pub fn name(reg: i32) -> &'static str {
        if (0 <= reg) && (reg < kNumFPURegisters) {
            FPURegisters::NAMES[reg as usize]
        } else {
            "nocreg"
        }
    }

    pub fn number(name: &str) -> i32 {
        for (i, &reg_name) in FPURegisters::NAMES.iter().enumerate() {
            if reg_name == name {
                return i as i32;
            }
        }

        for alias in FPURegisters::ALIASES.iter() {
            if let Some(alias_name) = alias.name {
                if alias_name == name {
                    return alias.creg;
                }
            }
        }

        kInvalidFPURegister
    }

    const NAMES: [&'static str; kNumFPURegisters as usize] = [
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10",
        "f11", "f12", "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21",
        "f22", "f23", "f24", "f25", "f26", "f27", "f28", "f29", "f30", "f31",
    ];

    const ALIASES: [RegisterAliasFPU; 1] = [
        RegisterAliasFPU { creg: kInvalidFPURegister, name: None },
    ];
}

#[derive(Copy, Clone)]
pub struct RegisterAliasFPU {
    creg: i32,
    name: Option<&'static str>,
}

pub type Instr = i32;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SoftwareInterruptCodes {
    CallRtRedirected = 0x7fff,
}

pub const kMaxWatchpointCode: u32 = 31;
pub const kMaxStopCode: u32 = 127;

pub const kRjShift: i32 = 5;
pub const kRjBits: i32 = 5;
pub const kRkShift: i32 = 10;
pub const kRkBits: i32 = 5;
pub const kRdShift: i32 = 0;
pub const kRdBits: i32 = 5;
pub const kSaShift: i32 = 15;
pub const kSa2Bits: i32 = 2;
pub const kSa3Bits: i32 = 3;
pub const kCdShift: i32 = 0;
pub const kCdBits: i32 = 3;
pub const kCjShift: i32 = 5;
pub const kCjBits: i32 = 3;
pub const kCodeShift: i32 = 0;
pub const kCodeBits: i32 = 15;
pub const kCondShift: i32 = 15;
pub const kCondBits: i32 = 5;
pub const kUi5Shift: i32 = 10;
pub const kUi5Bits: i32 = 5;
pub const kUi6Shift: i32 = 10;
pub const kUi6Bits: i32 = 6;
pub const kUi12Shift: i32 = 10;
pub const kUi12Bits: i32 = 12;
pub const kSi12Shift: i32 = 10;
pub const kSi12Bits: i32 = 12;
pub const kSi14Shift: i32 = 10;
pub const kSi14Bits: i32 = 14;
pub const kSi16Shift: i32 = 10;
pub const kSi16Bits: i32 = 16;
pub const kSi20Shift: i32 = 5;
pub const kSi20Bits: i32 = 20;
pub const kMsbwShift: i32 = 16;
pub const kMsbwBits: i32 = 5;
pub const kLsbwShift: i32 = 10;
pub const kLsbwBits: i32 = 5;
pub const kMsbdShift: i32 = 16;
pub const kMsbdBits: i32 = 6;
pub const kLsbdShift: i32 = 10;
pub const kLsbdBits: i32 = 6;
pub const kFdShift: i32 = 0;
pub const kFdBits: i32 = 5;
pub const kFjShift: i32 = 5;
pub const kFjBits: i32 = 5;
pub const kFkShift: i32 = 10;
pub const kFkBits: i32 = 5;
pub const kFaShift: i32 = 15;
pub const kFaBits: i32 = 5;
pub const kCaShift: i32 = 15;
pub const kCaBits: i32 = 3;
pub const kHint15Shift: i32 = 0;
pub const kHint15Bits: i32 = 15;
pub const kHint5Shift: i32 = 0;
pub const kHint5Bits: i32 = 5;
pub const kOffsLowShift: i32 = 10;
pub const kOffsLowBits: i32 = 16;
pub const kOffs26HighShift: i32 = 0;
pub const kOffs26HighBits: i32 = 10;
pub const kOffs21HighShift: i32 = 0;
pub const kOffs21HighBits: i32 = 5;
pub const kImm12Shift: i32 = 0;
pub const kImm12Bits: i32 = 12;
pub const kImm16Shift: i32 = 0;
pub const kImm16Bits: i32 = 16;
pub const kImm26Shift: i32 = 0;
pub const kImm26Bits: i32 = 26;
pub const kImm28Shift: i32 = 0;
pub const kImm28Bits: i32 = 28;
pub const kImm32Shift: i32 = 0;
pub const kImm32Bits: i32 = 32;

pub const kRjFieldMask: i32 = ((1 << kRjBits) - 1) << kRjShift;
pub const kRkFieldMask: i32 = ((1 << kRkBits) - 1) << kRkShift;
pub const kRdFieldMask: i32 = ((1 << kRdBits) - 1) << kRdShift;
pub const kSa2FieldMask: i32 = ((1 << kSa2Bits) - 1) << kSaShift;
pub const kSa3FieldMask: i32 = ((1 << kSa3Bits) - 1) << kSaShift;
pub const kHiMaskOf32: i32 = 0xffff << 16;
pub const kLoMaskOf32: i32 = 0xffff;
pub const kSignMaskOf32: i32 = 0x80000000;
pub const kTop16MaskOf64: i64 = 0xffffi64 << 48;
pub const kHigher16MaskOf64: i64 = 0xffffi64 << 32;
pub const kUpper16MaskOf64: i64 = 0xffffi64 << 16;

pub const kImm12Mask: i32 = ((1 << kImm12Bits) - 1) << kImm12Shift;
pub const kImm16Mask: i32 = ((1 << kImm16Bits) - 1) << kImm16Shift;
pub const kImm26Mask: i32 = ((1 << kImm26Bits) - 1) << kImm26Shift;
pub const kImm28Mask: i32 = ((1 << kImm28Bits) - 1) << kImm28Shift;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Opcode {
    BEQZ = 0x10U << 26,
    BNEZ = 0x11U << 26,
    BCZ = 0x12U << 26,
    JIRL = 0x13U << 26,
    B = 0x14U << 26,
    BL = 0x15U << 26,
    BEQ = 0x16U << 26,
    BNE = 0x17U << 26,
    BLT = 0x18U << 26,
    BGE = 0x19U << 26,
    BLTU = 0x1aU << 26,
    BGEU = 0x1bU << 26,

    ADDU16I_D = 0x4U << 26,

    LU12I_W = 0xaU << 25,
    LU32I_D = 0xbU << 25,
    PCADDI = 0xcU << 25,
    PCALAU12I = 0xdU << 25,
    PCADDU12I = 0xeU << 25,
    PCADDU18I = 0xfU << 25,

    LL_W = 0x20U << 24,
    SC_W = 0x21U << 24,
    LL_D = 0x22U << 24,
    SC_D = 0x23U << 24,
    LDPTR_W = 0x24U << 24,
    STPTR_W = 0x25U << 24,
    LDPTR_D = 0x26U << 24,
    STPTR_D = 0x27U << 24,

    BSTR_W = 0x1U << 22,
    BSTRINS_W = BSTR_W,
    BSTRPICK_W = BSTR_W,
    BSTRINS_D = 0x2U << 22,
    BSTRPICK_D = 0x3U << 22,

    SLTI = 0x8U << 22,
    SLTUI = 0x9U << 22,
    ADDI_W = 0xaU << 22,
    ADDI_D = 0xbU << 22,
    LU52I_D = 0xcU << 22,
    ANDI = 0xdU << 22,
    ORI = 0xeU << 22,
    XORI = 0xfU << 22,

    LD_B = 0xa0U << 22,
    LD_H = 0xa1U << 22,
    LD_W = 0xa2U << 22,
    LD_D = 0xa3U << 22,
    ST_B = 0xa4U << 22,
    ST_H = 0xa5U << 22,
    ST_W = 0xa6U << 22,
    ST_D = 0xa7U << 22,
    LD_BU = 0xa8U << 22,
    LD_HU = 0xa9U << 22,
    LD_WU = 0xaaU << 22,
    FLD_S = 0xacU << 22,
    FST_S = 0xadU << 22,
    FLD_D = 0xaeU << 22,
    FST_D = 0xafU << 22,

    FMADD_S = 0x81U << 20,
    FMADD_D = 0x82U << 20,
    FMSUB_S = 0x85U << 20,
    FMSUB_D = 0x86U << 20,
    FNMADD_S = 0x89U << 20,
    FNMADD_D = 0x8aU << 20,
    FNMSUB_S = 0x8dU << 20,
    FNMSUB_D = 0x8eU << 20,
    FCMP_COND_S = 0xc1U << 20,
    FCMP_COND_D = 0xc2U << 20,

    BYTEPICK_D = 0x3U << 18,
    BYTEPICK_W = 0x2U << 18,

    FSEL = 0x340U << 18,

    ALSL = 0x1U << 18,
    ALSL_W = ALSL,
    ALSL_WU = ALSL,

    ALSL_D = 0xbU << 18,

    SLLI_W = 0x40U << 16,
    SRLI_W = 0x44U << 16,
    SRAI_W = 0x48U << 16,
    ROTRI_W = 0x4cU << 16,

    SLLI_D = 0x41U << 16,
    SRLI_D = 0x45U << 16,
    SRAI_D = 0x49U << 16,
    ROTRI_D = 0x4dU << 16,

    SLLI = 0x10U << 18,
    SRLI = 0x11U << 18,
    SRAI = 0x12U << 18,
    ROTRI = 0x13U << 18,

    ADD_W = 0x20U << 15,
    ADD_D = 0x21U << 15,
    SUB_W = 0x22U << 15,
    SUB_D = 0x23U << 15,
    SLT = 0x24U << 15,
    SLTU = 0x25U << 15,
    MASKEQZ = 0x26U << 15,
    MASKNEZ = 0x27U << 15,
    NOR = 0x28U << 15,
    AND = 0x29U << 15,
    OR = 0x2aU << 15,
    XOR = 0x2bU << 15,
    ORN = 0x2cU << 15,
    ANDN = 0x2dU << 15,
    SLL_W = 0x2eU << 15,
    SRL_W = 0x2fU << 15,
    SRA_W = 0x30U << 15,
    SLL_D = 0x31U << 15,
    SRL_D = 0x32U << 15,
    SRA_D = 0x33U << 15,
    ROTR_W = 0x36U << 15,
    ROTR_D = 0x37U << 15,
    MUL_W = 0x38U << 15,
    MULH_W = 0x39U << 15,
    MULH_WU = 0x3aU << 15,
    MUL_D = 0x3bU << 15,
    MULH_D = 0x3cU << 15,
    MULH_DU = 0x3dU << 15,
    MULW_D_W = 0x3eU << 15,
    MULW_D_WU = 0x3fU << 15,

    DIV_W = 0x40U << 15,
    MOD_W = 0x41U << 15,
    DIV_WU = 0x42U << 15,
    MOD_WU = 0x43U << 15,
    DIV_D = 0x44U << 15,
    MOD_D = 0x45U << 15,
    DIV_DU = 0x46U << 15,
    MOD_DU = 0x47U << 15,

    BREAK = 0x54U << 15,

    FADD_S = 0x201U << 15,
    FADD_D = 0x202U << 15,
    FSUB_S = 0x205U << 15,
    FSUB_D = 0x206U << 15,
    FMUL_S = 0x209U << 15,
    FMUL_D = 0x20aU << 15,
    FDIV_S = 0x20dU << 15,
    FDIV_D = 0x20eU << 15,
    FMAX_S = 0x211U << 15,
    FMAX_D = 0x212U << 15,
    FMIN_S = 0x215U << 15,
    FMIN_D = 0x216U << 15,
    FMAXA_S = 0x219U << 15,
    FMAXA_D = 0x21aU << 15,
    FMINA_S = 0x21dU << 15,
    FMINA_D = 0x21eU << 15,
    FSCALEB_S = 0x221U << 15,
    FSCALEB_D = 0x222U << 15,
    FCOPYSIGN_S = 0x225U << 15,
    FCOPYSIGN_D = 0x226U << 15,

    LDX_B = 0x7000U << 15,
    LDX_H = 0x7008U << 15,
    LDX_W = 0x7010U << 15,
    LDX_D = 0x7018U << 15,
    STX_B = 0x7020U << 15,
    STX_H = 0x7028U << 15,
    STX_W = 0x7030U << 15,
    STX_D = 0x7038U << 15,
    LDX_BU = 0x7040U << 15,
    LDX_HU = 0x7048U << 15,
    LDX_WU = 0x7050U << 15,
    FLDX_S = 0x7060U << 15,
    FLDX_D = 0x7068U << 15,
    FSTX_S = 0x7070U << 15,
    FSTX_D = 0x7078U << 15,

    AMSWAP_W = 0x70c0U << 15,
    AMSWAP_D = 0x70c1U << 15,
    AMADD_W = 0x70c2U << 15,
    AMADD_D = 0x70c3U << 15,
    AMAND_W = 0x70c4U << 15,
    AMAND_D = 0x70c5U << 15,
    AMOR_W = 0x70c6U << 15,
    AMOR_D = 0x70c7U << 15,
    AMXOR_W = 0x70c8U << 15,
    AMXOR_D = 0x70c9U << 15,
    AMMAX_W = 0x70caU << 15,
    AMMAX_D = 0x70cbU << 15,
    AMMIN_W = 0x70ccU << 15,
    AMMIN_D = 0x70cdU << 15,
    AMMAX_WU = 0x70ceU << 15,
    AMMAX_DU = 0x70cfU << 15,
    AMMIN_WU = 0x70d0U << 15,
    AMMIN_DU = 0x70d1U << 15,
    AMSWAP_DB_W = 0x70d2U << 15,
    AMSWAP_DB_D = 0x70d3U << 15,
    AMADD_DB_W = 0x70d4U << 15,
    AMADD_DB_D = 0x70d5U << 15,
    AMAND_DB_W = 0x70d6U << 15,
    AMAND_DB_D = 0x70d7U << 15,
    AMOR_DB_W = 0x70d8U << 15,
    AMOR_DB_D = 0x70d9U << 15,
    AMXOR_DB_W = 0x70daU << 15,
    AMXOR_DB_D = 0x70dbU << 15,
    AMMAX_DB_W = 0x70dcU << 15,
    AMMAX_DB_D = 0x70ddU << 15,
    AMMIN_DB_W = 0x70deU << 15,
    AMMIN_DB_D = 0x70dfU << 15,
    AMMAX_DB_WU = 0x70e0U << 15,
    AMMAX_DB_DU = 0x70e1U << 15,
    AMMIN_DB_WU = 0x70e2U << 15,
    AMMIN_DB_DU = 0x70e3U << 15,

    DBAR = 0x70e4U << 15,
    IBAR = 0x70e5U << 15,

    CLO_W = 0X4U << 10,
    CLZ_W = 0X5U << 10,
    CTO_W = 0X6U << 10,
    CTZ_W = 0X7U << 10,
    CLO_D = 0X8U << 10,
    CLZ_D = 0X9U << 10,
    CTO_D = 0XaU << 10,
    CTZ_D = 0XbU << 10,
    REVB_2H = 0XcU << 10,
    REVB_4H = 0XdU << 10,
    REVB_2W = 0XeU << 10,
    REVB_D = 0XfU << 10,
    REVH_2W = 0X10U << 10,
    REVH_D = 0X11U << 10,
    BITREV_4B = 0X12U << 10,
    BITREV_8B = 0X13U << 10,
    BITREV_W = 0X14U << 10,
    BITREV_D = 0X15U << 10,
    EXT_W_H = 0X16U << 10,
    EXT_W_B = 0X17U << 10,

    FABS_S = 0X4501U << 10,
    FABS_D = 0X4502U << 10,
    FNEG_S = 0X4505U << 10,
    FNEG_D = 0X4506U << 10,
    FLOGB_S = 0X4509U << 10,
    FLOGB_D = 0X450aU << 10,
    FCLASS_S = 0X450dU << 10,
    FCLASS_D = 0X450eU << 10,
    FSQRT_S = 0X4511U << 10,
    FSQRT_D = 0X4512U << 10,
    FRECIP_S = 0X4515U << 10,
    FRECIP_D = 0X4516U << 10,
    FRSQRT_S = 0X4519U << 10,
    FRSQRT_D = 0X451aU << 10,
    FMOV_S = 0X4525U << 10,
    FMOV_D = 0X4526U << 10,
    MOVGR2FR_W = 0X4529U << 10,
    MOVGR2FR_D = 0X452aU << 10,
    MOVGR2FRH_W = 0X452bU << 10,
    MOVFR2GR_S = 0X452dU << 10,
    MOVFR2GR_D = 0X452eU << 10,
    MOVFRH2GR_S = 0X452fU << 10,
    MOVGR2FCSR = 0X4530U << 10,
    MOVFCSR2GR = 0X4532U << 10,
    MOVFR2CF = 0X4534U << 10,
    MOVGR2CF = 0X4536U << 10,

    FCVT_S_D = 0x4646U << 10,
    FCVT_D_S = 0x4649U << 10,
    FTINTRM_W_S = 0x4681U << 10,
    FTINTRM_W_D = 0x4682U << 10,
    FTINTRM_L_S = 0x4689U << 10,
    FTINTRM_L_D = 0x468aU << 10,
    FTINTRP_W_S = 0x4691U << 10,
    FTINTRP_W_D = 0x4692U << 10,
    FTINTRP_L_S = 0x4699U << 10,
    FTINTRP_L_D = 0x469aU << 10,
    FTINTRZ_W_S = 0x46a1U << 10,
    FTINTRZ_W_D = 0x46a2U << 10,
    FTINTRZ_L_S = 0x46a9U << 10,
    FTINTRZ_L_D = 0x46aaU << 10,
    FTINTRNE_W_S = 0x46b1U << 10,
    FTINTRNE_W_D = 0x46b2U << 10,
    FTINTRNE_L_S = 0x46b9U << 10,
    FTINTRNE_L_D = 0x46baU << 10,
    FTINT_W_S = 0x46c1U << 10,
    FTINT_W_D = 0x46c2U << 10,
    FTINT_L_S = 0x46c9U << 10,
    FTINT_L_D = 0x46caU << 10,
    FFINT_S_W = 0x4744U << 10,
    FFINT

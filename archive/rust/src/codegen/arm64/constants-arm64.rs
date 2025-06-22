// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! ARM64-specific constants.

#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]

use std::mem::size_of;

/// Asserts that a condition is true at compile time.
macro_rules! static_assert {
    ($cond:expr) => {
        const _: [(); 0 - !{$cond} as usize] = [];
    };
}

static_assert!(size_of::<i32>() == size_of::<i32>());
#[cfg(target_os = "windows")]
static_assert!(size_of::<i32>() == size_of::<i32>()); // Equivalent of sizeof(1L) on Windows
#[cfg(not(target_os = "windows"))]
static_assert!(size_of::<i64>() == size_of::<i64>()); // Equivalent of sizeof(long) on non-Windows
#[cfg(not(target_os = "windows"))]
static_assert!(size_of::<i64>() == size_of::<i64>()); // Equivalent of sizeof(1L) on non-Windows
static_assert!(size_of::<*const ()>() == size_of::<i64>());
static_assert!(size_of::<i32>() == size_of::<i32>());

pub mod internal {

    /// The maximum size of the code range such that pc-relative calls are possible
    /// between all Code objects in the range.
    pub const kMaxPCRelativeCodeRangeInMB: usize = 128;

    pub const kInstrSize: u8 = 4;
    pub const kInstrSizeLog2: u8 = 2;
    pub const kLoadLiteralScaleLog2: u8 = 2;
    pub const kLoadLiteralScale: u8 = 1 << kLoadLiteralScaleLog2;
    pub const kMaxLoadLiteralRange: i32 = 1 * 1024 * 1024; //1 * MB;

    pub const kNumberOfRegisters: i32 = 32;
    pub const kNumberOfVRegisters: i32 = 32;
    // Callee saved registers are x19-x28.
    pub const kNumberOfCalleeSavedRegisters: i32 = 10;
    // Callee saved FP registers are d8-d15.
    pub const kNumberOfCalleeSavedVRegisters: i32 = 8;
    pub const kWRegSizeInBits: i32 = 32;
    pub const kWRegSizeInBitsLog2: i32 = 5;
    pub const kWRegSize: i32 = kWRegSizeInBits >> 3;
    pub const kWRegSizeLog2: i32 = kWRegSizeInBitsLog2 - 3;
    pub const kXRegSizeInBits: i32 = 64;
    pub const kXRegSizeInBitsLog2: i32 = 6;
    pub const kXRegSize: i32 = kXRegSizeInBits >> 3;
    pub const kXRegSizeLog2: i32 = kXRegSizeInBitsLog2 - 3;
    pub const kSRegSizeInBits: i32 = 32;
    pub const kSRegSizeInBitsLog2: i32 = 5;
    pub const kSRegSize: i32 = kSRegSizeInBits >> 3;
    pub const kSRegSizeLog2: i32 = kSRegSizeInBitsLog2 - 3;
    pub const kDRegSizeInBits: i32 = 64;
    pub const kDRegSizeInBitsLog2: i32 = 6;
    pub const kDRegSize: i32 = kDRegSizeInBits >> 3;
    pub const kDRegSizeLog2: i32 = kDRegSizeInBitsLog2 - 3;
    pub const kDRegSizeInBytesLog2: i32 = kDRegSizeInBitsLog2 - 3;
    pub const kBRegSizeInBits: i32 = 8;
    pub const kBRegSize: i32 = kBRegSizeInBits >> 3;
    pub const kHRegSizeInBits: i32 = 16;
    pub const kHRegSize: i32 = kHRegSizeInBits >> 3;
    pub const kQRegSizeInBits: i32 = 128;
    pub const kQRegSizeInBitsLog2: i32 = 7;
    pub const kQRegSize: i32 = kQRegSizeInBits >> 3;
    pub const kQRegSizeLog2: i32 = kQRegSizeInBitsLog2 - 3;
    pub const kVRegSizeInBits: i32 = kQRegSizeInBits;
    pub const kVRegSize: i32 = kVRegSizeInBits >> 3;
    pub const kWRegMask: i64 = 0x00000000ffffffff;
    pub const kXRegMask: i64 = 0xffffffffffffffff;
    pub const kSRegMask: i64 = 0x00000000ffffffff;
    pub const kDRegMask: i64 = 0xffffffffffffffff;
    // TODO(all) check if the expression below works on all compilers or if it
    // triggers an overflow error.
    pub const kDSignBit: i64 = 63;
    pub const kDSignMask: i64 = 0x1 << kDSignBit;
    pub const kSSignBit: i64 = 31;
    pub const kSSignMask: i64 = 0x1 << kSSignBit;
    pub const kXSignBit: i64 = 63;
    pub const kXSignMask: i64 = 0x1 << kXSignBit;
    pub const kWSignBit: i64 = 31;
    pub const kWSignMask: i64 = 0x1 << kWSignBit;
    pub const kDQuietNanBit: i64 = 51;
    pub const kDQuietNanMask: i64 = 0x1 << kDQuietNanBit;
    pub const kSQuietNanBit: i64 = 22;
    pub const kSQuietNanMask: i64 = 0x1 << kSQuietNanBit;
    pub const kHQuietNanBit: i64 = 9;
    pub const kHQuietNanMask: i64 = 0x1 << kHQuietNanBit;
    pub const kByteMask: i64 = 0xff;
    pub const kHalfWordMask: i64 = 0xffff;
    pub const kWordMask: i64 = 0xffffffff;
    pub const kXMaxUInt: u64 = 0xffffffffffffffff;
    pub const kWMaxUInt: u32 = 0xffffffff;
    pub const kXMaxInt: i64 = 0x7fffffffffffffff;
    pub const kXMinInt: i64 = 0x8000000000000000;
    pub const kWMaxInt: i32 = 0x7fffffff;
    pub const kWMinInt: i32 = 0x80000000;
    pub const kIp0Code: i32 = 16;
    pub const kIp1Code: i32 = 17;
    pub const kFramePointerRegCode: i32 = 29;
    pub const kLinkRegCode: i32 = 30;
    pub const kZeroRegCode: i32 = 31;
    pub const kSPRegInternalCode: i32 = 63;
    pub const kRegCodeMask: u32 = 0x1f;
    pub const kShiftAmountWRegMask: u32 = 0x1f;
    pub const kShiftAmountXRegMask: u32 = 0x3f;
    // Standard machine types defined by AAPCS64.
    pub const kHalfWordSize: u32 = 16;
    pub const kHalfWordSizeLog2: u32 = 4;
    pub const kHalfWordSizeInBytes: u32 = kHalfWordSize >> 3;
    pub const kHalfWordSizeInBytesLog2: u32 = kHalfWordSizeLog2 - 3;
    pub const kWordSize: u32 = 32;
    pub const kWordSizeLog2: u32 = 5;
    pub const kWordSizeInBytes: u32 = kWordSize >> 3;
    pub const kWordSizeInBytesLog2: u32 = kWordSizeLog2 - 3;
    pub const kDoubleWordSize: u32 = 64;
    pub const kDoubleWordSizeInBytes: u32 = kDoubleWordSize >> 3;
    pub const kQuadWordSize: u32 = 128;
    pub const kQuadWordSizeInBytes: u32 = kQuadWordSize >> 3;
    pub const kMaxLanesPerVector: i32 = 16;

    pub const kAddressTagOffset: u32 = 56;
    pub const kAddressTagWidth: u32 = 8;
    pub const kAddressTagMask: u64 = ((1 << kAddressTagWidth) - 1) << kAddressTagOffset;
    static_assert!(kAddressTagMask == 0xff00000000000000,);

    pub const kTTBRMask: u64 = 1 << 55;

    // AArch64 floating-point specifics. These match IEEE-754.
    pub const kDoubleMantissaBits: u32 = 52;
    pub const kDoubleExponentBits: u32 = 11;
    pub const kDoubleExponentBias: u32 = 1023;
    pub const kFloatMantissaBits: u32 = 23;
    pub const kFloatExponentBits: u32 = 8;
    pub const kFloatExponentBias: u32 = 127;
    pub const kFloat16MantissaBits: u32 = 10;
    pub const kFloat16ExponentBits: u32 = 5;
    pub const kFloat16ExponentBias: u32 = 15;

    // The actual value of the kRootRegister is offset from the IsolateData's start
    // to take advantage of negative displacement values.
    pub const kRootRegisterBias: i32 = 256;

    pub type float16 = u16;

    macro_rules! declare_fields_offsets {
        ($(#[$attr:meta])* $name:ident, $high_bit:expr, $low_bit:expr, $unused_1:expr, $unused_2:expr) => {
            $(#[$attr])*
            pub const $name##_offset: i32 = $low_bit;
            $(#[$attr])*
            pub const $name##_width: i32 = $high_bit - $low_bit + 1;
            $(#[$attr])*
            pub const $name##_mask: u32 = ((1 << $name##_width) - 1) << $low_bit;
        };
    }

    macro_rules! declare_instruction_fields_offsets {
        ($(#[$attr:meta])* $name:ident, $high_bit:expr, $low_bit:expr, $unused_1:expr) => {
            declare_fields_offsets!($name, $high_bit, $low_bit, $unused_1,);
        };
    }

    macro_rules! instruction_fields_list {
        ($v:ident) => {
            /* Register fields */
            $v!(Rd, 4, 0, Bits); /* Destination register.     */
            $v!(Rn, 9, 5, Bits); /* First source register.    */
            $v!(Rm, 20, 16, Bits); /* Second source register.   */
            $v!(Ra, 14, 10, Bits); /* Third source register.    */
            $v!(Rt, 4, 0, Bits); /* Load dest / store source. */
            $v!(Rt2, 14, 10, Bits); /* Load second dest /        */
            /* store second source.      */
            $v!(Rs, 20, 16, Bits); /* Store-exclusive status    */
            $v!(PrefetchMode, 4, 0, Bits);

            /* Common bits */
            $v!(SixtyFourBits, 31, 31, Bits);
            $v!(FlagsUpdate, 29, 29, Bits);

            /* PC relative addressing */
            $v!(ImmPCRelHi, 23, 5, SignedBits);
            $v!(ImmPCRelLo, 30, 29, Bits);

            /* Add/subtract/logical shift register */
            $v!(ShiftDP, 23, 22, Bits);
            $v!(ImmDPShift, 15, 10, Bits);

            /* Add/subtract immediate */
            $v!(ImmAddSub, 21, 10, Bits);
            $v!(ShiftAddSub, 23, 22, Bits);

            /* Add/subtract extend */
            $v!(ImmExtendShift, 12, 10, Bits);
            $v!(ExtendMode, 15, 13, Bits);

            /* Move wide */
            $v!(ImmMoveWide, 20, 5, Bits);
            $v!(ShiftMoveWide, 22, 21, Bits);

            /* Logical immediate, bitfield and extract */
            $v!(BitN, 22, 22, Bits);
            $v!(ImmRotate, 21, 16, Bits);
            $v!(ImmSetBits, 15, 10, Bits);
            $v!(ImmR, 21, 16, Bits);
            $v!(ImmS, 15, 10, Bits);

            /* Test and branch immediate */
            $v!(ImmTestBranch, 18, 5, SignedBits);
            $v!(ImmTestBranchBit40, 23, 19, Bits);
            $v!(ImmTestBranchBit5, 31, 31, Bits);

            /* Conditionals */
            $v!(Condition, 15, 12, Bits);
            $v!(ConditionBranch, 3, 0, Bits);
            $v!(Nzcv, 3, 0, Bits);
            $v!(ImmCondCmp, 20, 16, Bits);
            $v!(ImmCondBranch, 23, 5, SignedBits);

            /* Floating point */
            $v!(FPType, 23, 22, Bits);
            $v!(ImmFP, 20, 13, Bits);
            $v!(FPScale, 15, 10, Bits);

            /* Load Store */
            $v!(ImmLS, 20, 12, SignedBits);
            $v!(ImmLSUnsigned, 21, 10, Bits);
            $v!(ImmLSPair, 21, 15, SignedBits);
            $v!(ImmShiftLS, 12, 12, Bits);
            $v!(LSOpc, 23, 22, Bits);
            $v!(LSVector, 26, 26, Bits);
            $v!(LSSize, 31, 30, Bits);

            /* NEON generic fields */
            $v!(NEONQ, 30, 30, Bits);
            $v!(NEONSize, 23, 22, Bits);
            $v!(NEONLSSize, 11, 10, Bits);
            $v!(NEONS, 12, 12, Bits);
            $v!(NEONL, 21, 21, Bits);
            $v!(NEONM, 20, 20, Bits);
            $v!(NEONH, 11, 11, Bits);
            $v!(ImmNEONExt, 14, 11, Bits);
            $v!(ImmNEON5, 20, 16, Bits);
            $v!(ImmNEON4, 14, 11, Bits);

            /* Other immediates */
            $v!(ImmUncondBranch, 25, 0, SignedBits);
            $v!(ImmCmpBranch, 23, 5, SignedBits);
            $v!(ImmLLiteral, 23, 5, SignedBits);
            $v!(ImmException, 20, 5, Bits);
            $v!(ImmHint, 11, 5, Bits);
            $v!(ImmBarrierDomain, 11, 10, Bits);
            $v!(ImmBarrierType, 9, 8, Bits);

            /* System (MRS, MSR) */
            $v!(ImmSystemRegister, 19, 5, Bits);
            $v!(SysO0, 19, 19, Bits);
            $v!(SysOp1, 18, 16, Bits);
            $v!(SysOp2, 7, 5, Bits);
            $v!(CRn, 15, 12, Bits);
            $v!(CRm, 11, 8, Bits);

            /* Load-/store-exclusive */
            $v!(LoadStoreXLoad, 22, 22, Bits);
            $v!(LoadStoreXNotExclusive, 23, 23, Bits);
            $v!(LoadStoreXAcquireRelease, 15, 15, Bits);
            $v!(LoadStoreXSizeLog2, 31, 30, Bits);
            $v!(LoadStoreXPair, 21, 21, Bits);

            /* NEON load/store */
            $v!(NEONLoad, 22, 22, Bits);

            /* NEON Modified Immediate fields */
            $v!(ImmNEONabc, 18, 16, Bits);
            $v!(ImmNEONdefgh, 9, 5, Bits);
            $v!(NEONModImmOp, 29, 29, Bits);
            $v!(NEONCmode, 15, 12, Bits);

            /* NEON Shift Immediate fields */
            $v!(ImmNEONImmhImmb, 22, 16, Bits);
            $v!(ImmNEONImmh, 22, 19, Bits);
            $v!(ImmNEONImmb, 18, 16, Bits);
        };
    }

    instruction_fields_list!(declare_instruction_fields_offsets);

    macro_rules! nothing {
        ($name:ident, $mask:ident) => {};
    }

    #[derive(Debug, Copy, Clone)]
    pub enum FPRounding {
        Nearest = 0,
        PositiveInfinity = 1,
        NegativeInfinity = 2,
        Zero = 3,
    }

    macro_rules! system_register_fields_list {
        ($v:ident, $m:ident) => {
            /* NZCV */
            $v!(Flags, 31, 28, Bits, u32);
            $v!(N, 31, 31, Bits, bool);
            $v!(Z, 30, 30, Bits, bool);
            $v!(C, 29, 29, Bits, bool);
            $v!(V, 28, 28, Bits, bool);
            $m!(NZCV, Flags_mask);

            /* FPCR */
            $v!(AHP, 26, 26, Bits, bool);
            $v!(DN, 25, 25, Bits, bool);
            $v!(FZ, 24, 24, Bits, bool);
            $v!(RMode, 23, 22, Bits, FPRounding);
            $m!(FPCR, AHP_mask | DN_mask | FZ_mask | RMode_mask);
        };
    }

    system_register_fields_list!(declare_fields_offsets, nothing);

    // ImmPCRel is a compound field (not present in INSTRUCTION_FIELDS_LIST), formed
    // from ImmPCRelLo and ImmPCRelHi.
    pub const ImmPCRel_mask: i32 = ImmPCRelLo_mask as i32 | ImmPCRelHi_mask as i32;

    /// Condition codes.
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum Condition {
        Eq = 0,   // Equal
        Ne = 1,   // Not equal
        Hs = 2,   // Unsigned higher or same (or carry set)
        Cs = 2,   //   --
        Lo = 3,   // Unsigned lower (or carry clear)
        Cc = 3,   //   --
        Mi = 4,   // Negative
        Pl = 5,   // Positive or zero
        Vs = 6,   // Signed overflow
        Vc = 7,   // No signed overflow
        Hi = 8,   // Unsigned higher
        Ls = 9,   // Unsigned lower or same
        Ge = 10,  // Signed greater than or equal
        Lt = 11,  // Signed less than
        Gt = 12,  // Signed greater than
        Le = 13,  // Signed less than or equal
        Al = 14,  // Always executed
        Nv = 15,  // Behaves as always/al.

        // Unified cross-platform condition names/aliases.
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

    impl Condition {
        pub fn negate(self) -> Self {
            // Conditions al and nv behave identically, as "always true". They can't be
            // inverted, because there is no never condition.
            debug_assert!((self != Condition::Al) && (self != Condition::Nv));
            unsafe { std::mem::transmute(self as i32 ^ 1) }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum FlagsUpdate {
        SetFlags = 1,
        LeaveFlags = 0,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u32)]
    pub enum StatusFlags {
        NoFlag = 0,

        // Derive the flag combinations from the system register bit descriptions.
        NFlag = N_mask,
        ZFlag = Z_mask,
        CFlag = C_mask,
        VFlag = V_mask,
        NZFlag = NFlag | ZFlag,
        NCFlag = NFlag | CFlag,
        NVFlag = NFlag | VFlag,
        ZCFlag = ZFlag | CFlag,
        ZVFlag = ZFlag | VFlag,
        CVFlag = CFlag | VFlag,
        NZCFlag = NFlag | ZFlag | CFlag,
        NZVFlag = NFlag | ZFlag | VFlag,
        NCVFlag = NFlag | CFlag | VFlag,
        ZCVFlag = ZFlag | CFlag | VFlag,
        NZCVFlag = NFlag | ZFlag | CFlag | VFlag,

        // Floating-point comparison results.
        FPEqualFlag = ZCFlag,
        FPLessThanFlag = NFlag,
        FPGreaterThanFlag = CFlag,
        FPUnorderedFlag = CVFlag,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum Shift {
        NoShift = -1,
        Lsl = 0x0,
        Lsr = 0x1,
        Asr = 0x2,
        Ror = 0x3,
        Msl = 0x4,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum Extend {
        NoExtend = -1,
        Uxtb = 0,
        Uxth = 1,
        Uxtw = 2,
        Uxtx = 3,
        Sxtb = 4,
        Sxth = 5,
        Sxtw = 6,
        Sxtx = 7,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum SystemHint {
        Nop = 0,
        Yield = 1,
        Wfe = 2,
        Wfi = 3,
        Sev = 4,
        Sevl = 5,
        Csdb = 20,
        Bti = 32,
        BtiC = 34,
        BtiJ = 36,
        BtiJc = 38,
    }

    /// In a guarded page, only BTI and PACI[AB]SP instructions are allowed to be
    /// the target of indirect branches. Details on which kinds of branches each
    /// instruction allows follow in the comments below:
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum BranchTargetIdentifier {
        /// Do not emit a BTI instruction.
        KNone,

        /// Emit a BTI instruction. Cannot be the target of indirect jumps/calls.
        KBti,

        /// Emit a "BTI c" instruction. Can be the target of indirect jumps (BR) with
        /// x16/x17 as the target register, or indirect calls (BLR).
        KBtiCall,

        /// Emit a "BTI j" instruction. Can be the target of indirect jumps (BR).
        KBtiJump,

        /// Emit a "BTI jc" instruction, which is a combination of "BTI j" and "BTI c".
        KBtiJumpCall,

        /// Emit a PACIBSP instruction, which acts like a "BTI c" or a "BTI jc",
        /// based on the value of SCTLR_EL1.BT0.
        KPacibsp,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum BarrierDomain {
        OuterShareable = 0,
        NonShareable = 1,
        InnerShareable = 2,
        FullSystem = 3,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i32)]
    pub enum BarrierType {
        BarrierOther = 0,
        BarrierReads = 1,
        BarrierWrites = 2,
        BarrierAll = 3,
    }

    // System/special register names.
    // This information is not encoded as one field but as the concatenation of
    // multiple fields (Op0<0>, Op1, Crn, Crm, Op2).
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u32)]
    pub enum SystemRegister {
        Nzcv = (((0x1 << SysO0_offset)
            | (0x3 << SysOp1_offset)
            | (0x4 << CRn_offset)
            | (0x2 << CRm_offset)
            | (0x0 << SysOp2_offset))
            >> ImmSystemRegister_offset) as u32,
        Fpcr = (((0x1 << SysO0_offset)
            | (0x3 << SysOp1_offset)
            | (0x4 << CRn_offset)
            | (0x4 << CRm_offset)
            | (0x0 << SysOp2_offset))
            >> ImmSystemRegister_offset) as u32,
    }

    // Instruction enumerations.
    //
    // These are the masks that define a class of instructions, and the list of
    // instructions within each class. Each enumeration has a Fixed, FMask and
    // Mask value.
    //
    // Fixed: The fixed bits in this instruction class.
    // FMask: The mask used to extract the fixed bits in the class.
    // Mask:  The mask used to identify the instructions within a class.
    //
    // The enumerations can be used like this:
    //
    // DCHECK(instr->Mask(PCRelAddressingFMask) == PCRelAddressingFixed);
    // switch(instr->Mask(PCRelAddressingMask)) {
    //   case ADR:  Format("adr 'Xd, 'AddrPCRelByte"); break;
    //   case ADRP: Format("adrp 'Xd, 'AddrPCRelPage"); break;
    //   default:   printf("Unknown instruction\n");
    // }

    // Used to corrupt encodings by setting all bits when orred. Although currently
    // unallocated in AArch64, this encoding is not guaranteed to be undefined
    // indefinitely.
    pub const kUnallocatedInstruction: u32 = 0xffffffff;

    // Generic fields.
    pub type GenericInstrField = u32;
    pub const SixtyFourBits: GenericInstrField = 0x80000000;
    pub const ThirtyTwoBits: GenericInstrField = 0x00000000;
    pub const FP32: GenericInstrField = 0x00000000;
    pub const FP64: GenericInstrField = 0x00400000;

    pub type NEONFormatField = u32;
    pub const NEONFormatFieldMask: NEONFormatField = 0x40C00000;
    pub const NEON_Q: NEONFormatField = 0x40000000;
    pub const NEON_sz: NEONFormatField = 0x00400000;
    pub const NEON_8B: NEONFormatField = 0x00000000;
    pub const NEON_16B: NEONFormatField = NEON_8B | NEON_Q;
    pub const NEON_4H: NEONFormatField = 0x00400000;
    pub const NEON_8H: NEONFormatField = NEON_4H | NEON_Q;
    pub const NEON_2S: NEONFormatField = 0x00800000;
    pub const NEON_4S: NEONFormatField = NEON_2S | NEON_Q;
    pub const NEON_1D: NEONFormatField = 0x00C00000;
    pub const NEON_2D: NEONFormatField = 0x00C00000 | NEON_Q;

    pub type NEONFPFormatField = u32;
    pub const NEONFPFormatFieldMask: NEONFPFormatField = 0x40400000;
    pub const NEON_FP_4H: NEONFPFormatField = 0x00000000;
    pub const NEON_FP_8H: NEONFPFormatField = NEON_Q;
    pub const NEON_FP_2S: NEONFPFormatField = FP32;
    pub const NEON_FP_4S: NEONFPFormatField = FP32 | NEON_Q;
    pub const NEON_FP_2D: NEONFPFormatField = FP64 | NEON_Q;

    pub type NEONLSFormatField = u32;
    pub const NEONLSFormatFieldMask: NEONLSFormatField = 0x40000C00;
    pub const LS_NEON_8B: NEONLSFormatField = 0x00000000;
    pub const LS_NEON_16B: NEONLSFormatField = LS_NEON_8B | NEON_Q;
    pub const LS_NEON_4H: NEONLSFormatField = 0x00000400;
    pub const LS_NEON_8H: NEONLSFormatField = LS_NEON_4H | NEON_Q;
    pub const LS_NEON_2S: NEONLSFormatField = 0x00000800;
    pub const LS_NEON_4S: NEONLSFormatField = LS_NEON_2S | NEON_Q;
    pub const LS_NEON_1D: NEONLSFormatField = 0x00000C00;
    pub const LS_NEON_2D: NEONLSFormatField = LS_NEON_1D | NEON_Q;

    pub type NEONScalarFormatField = u32;
    pub const NEONScalarFormatFieldMask: NEONScalarFormatField = 0x00C00000;
    pub const NEONScalar: NEONScalarFormatField = 0x10000000;
    pub const NEON_B: NEONScalarFormatField = 0x00000000;
    pub const NEON_H: NEONScalarFormatField = 0x00400000;
    pub const NEON_S: NEONScalarFormatField = 0x00800000;
    pub const NEON_D:
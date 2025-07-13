// Converted from V8 C++ source files:
// Header: constants-arm.h
// Implementation: constants-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constants_arm {
    // Copyright 2011 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    #![allow(non_camel_case_types)]

    use std::rc::Rc;
    use std::sync::Arc;

    // ARM EABI is required.
    // #[cfg(all(target_arch = "arm", not(target_feature = "eabi")))]
    // compile_error!("ARM EABI support is required.");

    // Constant pool marker.
    // Use UDF, the permanently undefined instruction.
    pub const K_CONSTANT_POOL_MARKER_MASK: i32 = 0xfff000f0;
    pub const K_CONSTANT_POOL_MARKER: i32 = 0xe7f000f0;
    pub const K_CONSTANT_POOL_LENGTH_MAX_MASK: i32 = 0xffff;
    #[inline]
    pub fn encode_constant_pool_length(length: i32) -> i32 {
        debug_assert!((length & K_CONSTANT_POOL_LENGTH_MAX_MASK) == length);
        ((length & 0xfff0) << 4) | (length & 0xf)
    }
    #[inline]
    pub fn decode_constant_pool_length(instr: i32) -> i32 {
        debug_assert_eq!(instr & K_CONSTANT_POOL_MARKER_MASK, K_CONSTANT_POOL_MARKER);
        ((instr >> 4) & 0xfff0) | (instr & 0xf)
    }

    // Number of registers in normal ARM mode.
    pub const K_NUM_REGISTERS: i32 = 16;
    pub const K_REG_SIZE_IN_BITS_LOG2: i32 = 5;

    // VFP support.
    pub const K_NUM_VFP_SINGLE_REGISTERS: i32 = 32;
    pub const K_NUM_VFP_DOUBLE_REGISTERS: i32 = 32;
    pub const K_NUM_VFP_REGISTERS: i32 =
        K_NUM_VFP_SINGLE_REGISTERS + K_NUM_VFP_DOUBLE_REGISTERS;

    // PC is register 15.
    pub const K_PC_REGISTER: i32 = 15;
    pub const K_NO_REGISTER: i32 = -1;

    // Used in embedded constant pool builder - max reach in bits for
    // various load instructions (unsigned)
    pub const K_LDR_MAX_REACH_BITS: i32 = 12;
    pub const K_VLDR_MAX_REACH_BITS: i32 = 10;

    // The actual value of the kRootRegister is offset from the IsolateData's start
    // to take advantage of negative displacement values.
    //
    // Loads allow a uint12 value with a separate sign bit (range [-4095, +4095]),
    // so the first root is still addressable with a single load instruction.
    pub const K_ROOT_REGISTER_BIAS: i32 = 4095;

    // -----------------------------------------------------------------------------
    // Conditions.

    // Defines constants and accessor classes to assemble, disassemble and
    // simulate ARM instructions.
    //
    // Section references in the code refer to the "ARM Architecture Reference
    // Manual" from July 2005 (available at http://www.arm.com/miscPDFs/14128.pdf)
    //
    // Constants for specific fields are defined in their respective named enums.
    // General constants are in an anonymous enum in class Instr.

    // Values for the condition field as defined in section A3.2
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Condition {
        kNoCondition = -1,
        eq = 0 << 28,   // Z set            Equal.
        ne = 1 << 28,   // Z clear          Not equal.
        cs = 2 << 28,   // C set            Unsigned higher or same.
        cc = 3 << 28,   // C clear          Unsigned lower.
        mi = 4 << 28,   // N set            Negative.
        pl = 5 << 28,   // N clear          Positive or zero.
        vs = 6 << 28,   // V set            Overflow.
        vc = 7 << 28,   // V clear          No overflow.
        hi = 8 << 28,   // C set, Z clear   Unsigned higher.
        ls = 9 << 28,   // C clear or Z set Unsigned lower or same.
        ge = 10 << 28,  // N == V           Greater or equal.
        lt = 11 << 28,  // N != V           Less than.
        gt = 12 << 28,  // Z clear, N == V  Greater than.
        le = 13 << 28,  // Z set or N != V  Less then or equal
        al = 14 << 28,  //                  Always.

        // Special condition (refer to section A3.2.1).
        kSpecialCondition = 15 << 28,
        kNumberOfConditions = 16,

        // Aliases.
        hs = cs,  // C set            Unsigned higher or same.
        lo = cc,  // C clear          Unsigned lower.

        // Unified cross-platform condition names/aliases.
        kEqual = eq,
        kNotEqual = ne,
        kLessThan = lt,
        kGreaterThan = gt,
        kLessThanEqual = le,
        kGreaterThanEqual = ge,
        kUnsignedLessThan = lo,
        kUnsignedGreaterThan = hi,
        kUnsignedLessThanEqual = ls,
        kUnsignedGreaterThanEqual = hs,
        kOverflow = vs,
        kNoOverflow = vc,
        kZero = eq,
        kNotZero = ne,
    }

    #[inline]
    pub fn negate_condition(cond: Condition) -> Condition {
        debug_assert!(cond != Condition::al);
        unsafe { std::mem::transmute((cond as i32 ^ Condition::ne as i32) as i32) }
    }

    // -----------------------------------------------------------------------------
    // Instructions encoding.

    // Instr is merely used by the Assembler to distinguish 32bit integers
    // representing instructions from usual 32 bit values.
    // Instruction objects are pointers to 32bit values, and provide methods to
    // access the various ISA fields.
    pub type Instr = i32;

    // Opcodes for Data-processing instructions (instructions with a type 0 and 1)
    // as defined in section A3.4
    pub type Opcode = i32;
    pub const AND: Opcode = 0 << 21;   // Logical AND.
    pub const EOR: Opcode = 1 << 21;   // Logical Exclusive OR.
    pub const SUB: Opcode = 2 << 21;   // Subtract.
    pub const RSB: Opcode = 3 << 21;   // Reverse Subtract.
    pub const ADD: Opcode = 4 << 21;   // Add.
    pub const ADC: Opcode = 5 << 21;   // Add with Carry.
    pub const SBC: Opcode = 6 << 21;   // Subtract with Carry.
    pub const RSC: Opcode = 7 << 21;   // Reverse Subtract with Carry.
    pub const TST: Opcode = 8 << 21;   // Test.
    pub const TEQ: Opcode = 9 << 21;   // Test Equivalence.
    pub const CMP: Opcode = 10 << 21;  // Compare.
    pub const CMN: Opcode = 11 << 21;  // Compare Negated.
    pub const ORR: Opcode = 12 << 21;  // Logical (inclusive) OR.
    pub const MOV: Opcode = 13 << 21;  // Move.
    pub const BIC: Opcode = 14 << 21;  // Bit Clear.
    pub const MVN: Opcode = 15 << 21;  // Move Not.

    // The bits for bit 7-4 for some type 0 miscellaneous instructions.
    pub type MiscInstructionsBits74 = i32;
    // With bits 22-21 01.
    pub const BX: MiscInstructionsBits74 = 1 << 4;
    pub const BXJ: MiscInstructionsBits74 = 2 << 4;
    pub const BLX: MiscInstructionsBits74 = 3 << 4;
    pub const BKPT: MiscInstructionsBits74 = 7 << 4;

    // With bits 22-21 11.
    pub const CLZ: MiscInstructionsBits74 = 1 << 4;

    // Instruction encoding bits and masks.
    pub const H: i32 = 1 << 5;   // Halfword (or byte).
    pub const S6: i32 = 1 << 6;  // Signed (or unsigned).
    pub const L: i32 = 1 << 20;  // Load (or store).
    pub const S: i32 = 1 << 20;  // Set condition code (or leave unchanged).
    pub const W: i32 = 1 << 21;  // Writeback base register (or leave unchanged).
    pub const A: i32 = 1 << 21;  // Accumulate in multiply instruction (or not).
    pub const B: i32 = 1 << 22;  // Unsigned byte (or word).
    pub const N: i32 = 1 << 22;  // Long (or short).
    pub const U: i32 = 1 << 23;  // Positive (or negative) offset/index.
    pub const P: i32 =
        1 << 24;  // Offset/pre-indexed addressing (or post-indexed addressing).
    pub const I: i32 = 1 << 25;  // Immediate shifter operand (or not).
    pub const B0: i32 = 1 << 0;
    pub const B4: i32 = 1 << 4;
    pub const B5: i32 = 1 << 5;
    pub const B6: i32 = 1 << 6;
    pub const B7: i32 = 1 << 7;
    pub const B8: i32 = 1 << 8;
    pub const B9: i32 = 1 << 9;
    pub const B10: i32 = 1 << 10;
    pub const B12: i32 = 1 << 12;
    pub const B16: i32 = 1 << 16;
    pub const B17: i32 = 1 << 17;
    pub const B18: i32 = 1 << 18;
    pub const B19: i32 = 1 << 19;
    pub const B20: i32 = 1 << 20;
    pub const B21: i32 = 1 << 21;
    pub const B22: i32 = 1 << 22;
    pub const B23: i32 = 1 << 23;
    pub const B24: i32 = 1 << 24;
    pub const B25: i32 = 1 << 25;
    pub const B26: i32 = 1 << 26;
    pub const B27: i32 = 1 << 27;
    pub const B28: i32 = 1 << 28;

    // Instruction bit masks.
    pub const K_COND_MASK: i32 = 15 << 28;
    pub const K_ALU_MASK: i32 = 0x6f << 21;
    pub const K_RD_MASK: i32 = 15 << 12; // In str instruction.
    pub const K_COPROCESSOR_MASK: i32 = 15 << 8;
    pub const K_OP_CODE_MASK: i32 = 15 << 21; // In data-processing instructions.
    pub const K_IMM24_MASK: i32 = (1 << 24) - 1;
    pub const K_IMM16_MASK: i32 = (1 << 16) - 1;
    pub const K_IMM8_MASK: i32 = (1 << 8) - 1;
    pub const K_OFF12_MASK: i32 = (1 << 12) - 1;
    pub const K_OFF8_MASK: i32 = (1 << 8) - 1;

    pub type BarrierOption = i32;
    pub const OSHLD: BarrierOption = 0x1;
    pub const OSHST: BarrierOption = 0x2;
    pub const OSH: BarrierOption = 0x3;
    pub const NSHLD: BarrierOption = 0x5;
    pub const NSHST: BarrierOption = 0x6;
    pub const NSH: BarrierOption = 0x7;
    pub const ISHLD: BarrierOption = 0x9;
    pub const ISHST: BarrierOption = 0xa;
    pub const ISH: BarrierOption = 0xb;
    pub const LD: BarrierOption = 0xd;
    pub const ST: BarrierOption = 0xe;
    pub const SY: BarrierOption = 0xf;

    // -----------------------------------------------------------------------------
    // Addressing modes and instruction variants.

    // Condition code updating mode.
    pub type SBit = i32;
    pub const SET_CC: SBit = 1 << 20;   // Set condition code.
    pub const LEAVE_CC: SBit = 0 << 20; // Leave condition code unchanged.

    // Status register selection.
    pub type SRegister = i32;
    pub const CPSR: SRegister = 0 << 22;
    pub const SPSR: SRegister = 1 << 22;

    // Shifter types for Data-processing operands as defined in section A5.1.2.
    pub type ShiftOp = i32;
    pub const LSL: ShiftOp = 0 << 5; // Logical shift left.
    pub const LSR: ShiftOp = 1 << 5; // Logical shift right.
    pub const ASR: ShiftOp = 2 << 5; // Arithmetic shift right.
    pub const ROR: ShiftOp = 3 << 5; // Rotate right.

    // RRX is encoded as ROR with shift_imm == 0.
    // Use a special code to make the distinction. The RRX ShiftOp is only used
    // as an argument, and will never actually be encoded. The Assembler will
    // detect it and emit the correct ROR shift operand with shift_imm == 0.
    pub const RRX: ShiftOp = -1;
    pub const K_NUMBER_OF_SHIFTS: i32 = 4;

    // Status register fields.
    pub type SRegisterField = i32;
    pub const CPSR_c: SRegisterField = CPSR | 1 << 16;
    pub const CPSR_x: SRegisterField = CPSR | 1 << 17;
    pub const CPSR_s: SRegisterField = CPSR | 1 << 18;
    pub const CPSR_f: SRegisterField = CPSR | 1 << 19;
    pub const SPSR_c: SRegisterField = SPSR | 1 << 16;
    pub const SPSR_x: SRegisterField = SPSR | 1 << 17;
    pub const SPSR_s: SRegisterField = SPSR | 1 << 18;
    pub const SPSR_f: SRegisterField = SPSR | 1 << 19;

    // Status register field mask (or'ed SRegisterField enum values).
    pub type SRegisterFieldMask = u32;

    // Memory operand addressing mode.
    pub type AddrMode = i32;
    // Bit encoding P U W.
    pub const OFFSET: AddrMode = (8 | 4 | 0)
                                << 21; // Offset (without writeback to base).
    pub const PRE_INDEX: AddrMode = (8 | 4 | 1)
                                  << 21; // Pre-indexed addressing with writeback.
    pub const POST_INDEX: AddrMode =
        (0 | 4 | 0) << 21; // Post-indexed addressing with writeback.
    pub const NEG_OFFSET: AddrMode =
        (8 | 0 | 0) << 21; // Negative offset (without writeback to base).
    pub const NEG_PRE_INDEX: AddrMode = (8 | 0 | 1)
                                   << 21; // Negative pre-indexed with writeback.
    pub const NEG_POST_INDEX: AddrMode =
        (0 | 0 | 0) << 21; // Negative post-indexed with writeback.

    // Load/store multiple addressing mode.
    pub type BlockAddrMode = i32;
    // Bit encoding P U W .
    pub const DA: BlockAddrMode = (0 | 0 | 0) << 21; // Decrement after.
    pub const IA: BlockAddrMode = (0 | 4 | 0) << 21; // Increment after.
    pub const DB: BlockAddrMode = (8 | 0 | 0) << 21; // Decrement before.
    pub const IB: BlockAddrMode = (8 | 4 | 0) << 21; // Increment before.
    pub const DA_W: BlockAddrMode =
        (0 | 0 | 1) << 21; // Decrement after with writeback to base.
    pub const IA_W: BlockAddrMode =
        (0 | 4 | 1) << 21; // Increment after with writeback to base.
    pub const DB_W: BlockAddrMode =
        (8 | 0 | 1) << 21; // Decrement before with writeback to base.
    pub const IB_W: BlockAddrMode =
        (8 | 4 | 1) << 21; // Increment before with writeback to base.

    // Alias modes for comparison when writeback does not matter.
    pub const DA_X: BlockAddrMode = (0 | 0 | 0) << 21; // Decrement after.
    pub const IA_X: BlockAddrMode = (0 | 4 | 0) << 21; // Increment after.
    pub const DB_X: BlockAddrMode = (8 | 0 | 0) << 21; // Decrement before.
    pub const IB_X: BlockAddrMode = (8 | 4 | 0) << 21; // Increment before.

    pub const K_BLOCK_ADDR_MODE_MASK: BlockAddrMode = (8 | 4 | 1) << 21;

    // Coprocessor load/store operand size.
    pub type LFlag = i32;
    pub const LONG: LFlag = 1 << 22;   // Long load/store coprocessor.
    pub const SHORT: LFlag = 0 << 22;  // Short load/store coprocessor.

    // Neon sizes.
    pub type NeonSize = i32;
    pub const NEON8: NeonSize = 0x0;
    pub const NEON16: NeonSize = 0x1;
    pub const NEON32: NeonSize = 0x2;
    pub const NEON64: NeonSize = 0x3;

    // NEON data type, top bit set for unsigned data types.
    pub type NeonDataType = i32;
    pub const NEON_S8: NeonDataType = 0;
    pub const NEON_S16: NeonDataType = 1;
    pub const NEON_S32: NeonDataType = 2;
    pub const NEON_S64: NeonDataType = 3;
    pub const NEON_U8: NeonDataType = 4;
    pub const NEON_U16: NeonDataType = 5;
    pub const NEON_U32: NeonDataType = 6;
    pub const NEON_U64: NeonDataType = 7;

    #[inline]
    pub fn neon_u(dt: NeonDataType) -> i32 {
        (dt >> 2) as i32
    }
    #[inline]
    pub fn neon_sz(dt: NeonDataType) -> i32 {
        dt & 0x3
    }

    // Convert sizes to data types (U bit is clear).
    #[inline]
    pub fn neon_size_to_data_type(size: NeonSize) -> NeonDataType {
        debug_assert!(size != NEON64);
        size
    }

    #[inline]
    pub fn neon_data_type_to_size(dt: NeonDataType) -> NeonSize {
        neon_sz(dt)
    }

    pub type NeonListType = i32;
    pub const NLT_1: NeonListType = 0x7;
    pub const NLT_2: NeonListType = 0xA;
    pub const NLT_3: NeonListType = 0x6;
    pub const NLT_4: NeonListType = 0x2;

    // -----------------------------------------------------------------------------
    // Supervisor Call (svc) specific support.

    // Special Software Interrupt codes when used in the presence of the ARM
    // simulator.
    // svc (formerly swi) provides a 24bit immediate value. Use bits 22:0 for
    // standard SoftwareInterrupCode. Bit 23 is reserved for the stop feature.
    pub type SoftwareInterruptCodes = i32;
    // transition to C code
    pub const K_CALL_RT_REDIRECTED: SoftwareInterruptCodes = 0x10;
    // break point
    pub const K_BREAKPOINT: SoftwareInterruptCodes = 0x20;
    // stop
    pub const K_STOP_CODE: SoftwareInterruptCodes = 1 << 23;
    pub const K_STOP_CODE_MASK: u32 = K_STOP_CODE as u32 - 1;
    pub const K_MAX_STOP_CODE: u32 = K_STOP_CODE as u32 - 1;
    pub const K_DEFAULT_STOP_CODE: i32 = -1;

    // Type of VFP register. Determines register encoding.
    pub type VFPRegPrecision = i32;
    pub const K_SINGLE_PRECISION: VFPRegPrecision = 0;
    pub const K_DOUBLE_PRECISION: VFPRegPrecision = 1;
    pub const K_SIMD128_PRECISION: VFPRegPrecision = 2;

    // VFP FPSCR constants.
    pub type VFPConversionMode = i32;
    pub const K_FPSCR_ROUNDING: VFPConversionMode = 0;
    pub const K_DEFAULT_ROUND_TO_ZERO: VFPConversionMode = 1;

    // This mask does not include the "inexact" or "input denormal" cumulative
    // exceptions flags, because we usually don't want to check for it.
    pub const K_VFP_EXCEPTION_MASK: u32 = 0xf;
    pub const K_VFP_INVALID_OP_EXCEPTION_BIT: u32 = 1 << 0;
    pub const K_VFP_OVERFLOW_EXCEPTION_BIT: u32 = 1 << 2;
    pub const K_VFP_UNDERFLOW_EXCEPTION_BIT: u32 = 1 << 3;
    pub const K_VFP_INEXACT_EXCEPTION_BIT: u32 = 1 << 4;
    pub const K_VFP_FLUSH_TO_ZERO_MASK: u32 = 1 << 24;
    pub const K_VFP_DEFAULT_NAN_MODE_CONTROL_BIT: u32 = 1 << 25;

    pub const K_VFP_N_CONDITION_FLAG_BIT: u32 = 1 << 31;
    pub const K_VFP_Z_CONDITION_FLAG_BIT: u32 = 1 << 30;
    pub const K_VFP_C_CONDITION_FLAG_BIT: u32 = 1 << 29;
    pub const K_VFP_V_CONDITION_FLAG_BIT: u32 = 1 << 28;

    // VFP rounding modes. See ARM DDI 0406B Page A2-29.
    pub type VFPRoundingMode = i32;
    pub const RN: VFPRoundingMode = 0 << 22; // Round to Nearest.
    pub const RP: VFPRoundingMode = 1 << 22; // Round towards Plus Infinity.
    pub const RM: VFPRoundingMode = 2 << 22; // Round towards Minus Infinity.
    pub const RZ: VFPRoundingMode = 3 << 22; // Round towards zero.

    // Aliases.
    pub const K_ROUND_TO_NEAREST: VFPRoundingMode = RN;
    pub const K_ROUND_TO_PLUS_INF: VFPRoundingMode = RP;
    pub const K_ROUND_TO_MINUS_INF: VFPRoundingMode = RM;
    pub const K_ROUND_TO_ZERO: VFPRoundingMode = RZ;

    pub const K_VFP_ROUNDING_MODE_MASK: u32 = 3 << 22;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CheckForInexactConversion {
        kCheckForInexactConversion,
        kDontCheckForInexactConversion,
    }

    // -----------------------------------------------------------------------------
    // Hints.

    // Branch hints are not used on the ARM.  They are defined so that they can
    // appear in shared function signatures, but will be ignored in ARM
    // implementations.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Hint {
        no_hint,
    }

    // Hints are not used on the arm.  Negating is trivial.
    #[inline]
    pub fn negate_hint(ignored: Hint) -> Hint {
        Hint::no_hint
    }

    // -----------------------------------------------------------------------------
    // Instruction abstraction.

    // The class Instruction enables access to individual fields defined in the ARM
    // architecture instruction set encoding as described in figure A3-1.
    // Note that the Assembler uses typedef int32_t Instr.
    //
    // Example: Test whether the instruction at ptr does set the condition code
    // bits.
    //
    // bool InstructionSetsConditionCodes(uint8_t* ptr) {
    //   Instruction* instr = Instruction::At(ptr);
    //   int type = instr->TypeValue();
    //   return ((type == 0) || (type == 1)) && instr->HasS();
    // }

    pub const K_INSTR_SIZE: u8 = 4;
    pub const K_INSTR_SIZE_LOG2: u8 = 2;

    pub struct Instruction {
        instruction_bits: i32, // Instr
    }

    impl Instruction {
        // Difference between address of current opcode and value read from pc
        // register.
        pub const K_PC_LOAD_DELTA: i32 = 8;

        // Helper macro to define static accessors.
        // We use the cast to char* trick to bypass the strict anti-aliasing rules.
        #[inline]
        pub fn instruction_bits(&self) -> i32 {
            self.instruction_bits
        }

        // Set the raw instruction bits to value.
        pub fn set_instruction_bits(
            &mut self,
            value: Instr,
           
        ) {
            self.instruction_bits = value;
        }

        // Extract a single bit from the instruction bits and return it as bit 0 in
        // the result.
        #[inline]
        pub fn bit(&self, nr: i32) -> i32 {
            ((self.instruction_bits() >> nr) & 1) as i32
        }

        // Extract a bit field <hi:lo> from the instruction bits and return it in the
        // least-significant bits of the result.
        #[inline]
        pub fn bits(&self, hi: i32, lo: i32) -> i32 {
            ((self.instruction_bits() >> lo) & ((2 << (hi - lo)) - 1)) as i32
        }

        // Read a bit field <hi:lo>, leaving its position unchanged in the result.
        #[inline]
        pub fn bit_field(&self, hi: i32, lo: i32) -> i32 {
            self.instruction_bits() & (((2 << (hi - lo)) - 1) << lo)
        }

        // Accessors for the different named fields used in the ARM encoding.
        // The naming of these accessor corresponds to figure A3-1.
        //
        // Two kind of accessors are declared:
        // - <Name>Field() will return the raw field, i.e. the field's bits at their
        //   original place in the instruction encoding.
        //   e.g. if instr is the 'addgt r0, r1, r2' instruction, encoded as
        //   0xC0810002 ConditionField(instr) will return 0xC0000000.
        // - <Name>Value() will return the field value, shifted back to bit 0.
        //   e.g. if instr is the 'addgt r0, r1, r2' instruction, encoded as
        //   0xC0810002 ConditionField(instr) will return 0xC.

        // Generally applicable fields
        #[inline]
        pub fn condition_value(&self) -> i32 {
            self.bits(31, 28)
        }
        #[inline]
        pub fn condition_field(&self) -> Condition {
            unsafe { std::mem::transmute(self.bit_field(31, 28)) }
        }
      
        #[inline]
        pub fn type_value(&self) -> i32 {
            self.bits(27, 25)
        }
        #[inline]
        pub fn special_value(&self) -> i32 {
            self.bits(27, 23)
        }

        #[inline]
        pub fn rn_value(&self) -> i32 {
            self.bits(19, 16)
        }
       
        #[inline]
        pub fn rd_value(&self) -> i32 {
            self.bits(15, 12)
        }
      
        #[inline]
        pub fn coprocessor_value(&self) -> i32 {
            self.bits(11, 8)
        }
        // Support for VFP.
        // Vn(19-16) | Vd(15-12) |  Vm(3-0)
        #[inline]
        pub fn vn_value(&self) -> i32 {
            self.bits(19, 16)
        }
        #[inline]
        pub fn vm_value(&self) -> i32 {
            self.bits(3, 0)
        }
        #[inline]
        pub fn vd_value(&self) -> i32 {
            self.bits(15, 12)
        }
        #[inline]
        pub fn n_value(&self) -> i32 {
            self.bit(7)
        }
        #[inline]
        pub fn m_value(&self) -> i32 {
            self.bit(5)
        }
        #[inline]
        pub fn d_value(&self) -> i32 {
            self.bit(22)
        }
        #[inline]
        pub fn rt_value(&self) -> i32 {
            self.bits(15, 12)
        }
        #[inline]
        pub fn p_value(&self) -> i32 {
            self.bit(24)
        }
        #[inline]
        pub fn u_value(&self) -> i32 {
            self.bit(23)
        }
        #[inline]
        pub fn opc1_value(&self) -> i32 {
            (self.bit(23) << 2) | self.bits(21, 20)
        }
        #[inline]
        pub fn opc2_value(&self) -> i32 {
            self.bits(19, 16)
        }
        #[inline]
        pub fn opc3_value(&self) -> i32 {
            self.bits(7, 6)
        }
        #[inline]
        pub fn sz_value(&self) -> i32 {
            self.bit(8)
        }
        #[inline]
        pub fn vl_value(&self) -> i32 {
            self.bit(20)
        }
        #[inline]
        pub fn vc_value(&self) -> i32 {
            self.bit(8)
        }
        #[inline]
        pub fn va_value(&self) -> i32 {
            self.bits(23, 21)
        }
        #[inline]
        pub fn vb_value(&self) -> i32 {
            self.bits(6, 5)
        }
        #[inline]
        pub fn vfpn_reg_value(&self, pre: VFPRegPrecision) -> i32 {
            self.vfpglue_reg_value(pre, 16, 7)
        }
        #[inline]
        pub fn vfpm_reg_value(&self, pre: V

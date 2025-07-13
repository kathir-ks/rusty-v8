// Converted from V8 C++ source files:
// Header: base-constants-riscv.h
// Implementation: base-constants-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

pub mod base {
    // Copyright 2022 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    use std::cmp::PartialEq;
    use std::fmt;
    use std::string::String;
    use std::sync::Arc;
    use std::{i16, i32, i64, u32, u8};
    extern crate libc;
    use self::libc::{c_char, strcmp};
    use std::rc::Rc;

    #[macro_export]
    macro_rules! UNIMPLEMENTED_RISCV {
        () => {
            #[cfg(debug_assertions)]
            {
                println!(
                    "{}, \tline {}: \tfunction {} not implemented.\n",
                    file!(),
                    line!(),
                    function!()
                );
            }
            #[cfg(not(debug_assertions))]
            {}
        };
    }

    #[macro_export]
    macro_rules! UNSUPPORTED_RISCV {
        () => {{
            println!("Unsupported instruction {}.\n", line!());
            UNIMPLEMENTED!();
        }};
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Endianness {
        kLittle,
        kBig,
    }

    #[cfg(target_endian = "little")]
    pub const kArchEndian: Endianness = Endianness::kLittle;

    #[cfg(target_endian = "big")]
    pub const kArchEndian: Endianness = Endianness::kBig;

    #[cfg(not(any(target_endian = "little", target_endian = "big")))]
    compile_error!("Unknown endianness");

    #[cfg(target_endian = "little")]
    pub const kLeastSignificantByteInInt32Offset: u32 = 0;

    #[cfg(target_endian = "little")]
    pub const kLessSignificantWordInDoublewordOffset: u32 = 0;

    #[cfg(target_endian = "big")]
    pub const kLeastSignificantByteInInt32Offset: u32 = 3;

    #[cfg(target_endian = "big")]
    pub const kLessSignificantWordInDoublewordOffset: u32 = 4;

    #[cfg(not(any(target_endian = "little", target_endian = "big")))]
    compile_error!("Unknown endianness");

    pub type Opcode = u32;

    // Actual value of root register is offset from the root array's start
    // to take advantage of negative displacement values.
    pub const kRootRegisterBias: i32 = 256;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Vlmul {
        m1,
        m2,
        m4,
        m8,
        RESERVERD,
        mf8,
        mf4,
        mf2,
        kVlInvalid,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum VSew {
        E8,
        E16,
        E32,
        E64,
        kVsInvalid,
    }

    // RISC-V can perform PC-relative jumps within a 32-bit range using the
    // following two instructions:
    //   auipc   t6, imm20    ; t0 = PC + imm20 * 2^12
    //   jalr    ra, t6, imm12; ra = PC + 4, PC = t0 + imm12,
    // Both imm20 and imm12 are treated as two's-complement signed values, usually
    // calculated as:
    //   imm20 = (offset + 0x800) >> 12
    //   imm12 = offset & 0xfff
    // offset is the signed offset from the auipc instruction. Adding 0x800 handles
    // the offset, but if the offset is >= 2^31 - 2^11, it will overflow. Therefore,
    // the true 32-bit range is:
    //   [-2^31 - 2^11, 2^31 - 2^11)
    pub const kMaxPCRelativeCodeRangeInMB: usize = 2047;

    // -----------------------------------------------------------------------------
    // Registers and FPURegisters.

    // Number of general purpose registers.
    pub const kNumRegisters: i32 = 32;
    pub const kInvalidRegister: i32 = -1;

    // Number of registers with pc.
    pub const kNumSimuRegisters: i32 = 33;

    // In the simulator, the PC register is simulated as the 34th register.
    pub const kPCRegister: i32 = 34;

    // Number coprocessor registers.
    pub const kNumFPURegisters: i32 = 32;
    pub const kInvalidFPURegister: i32 = -1;

    // Number vectotr registers
    pub const kNumVRegisters: i32 = 32;
    pub const kInvalidVRegister: i32 = -1;
    // 'pref' instruction hints
    pub const kPrefHintLoad: i32 = 0;
    pub const kPrefHintStore: i32 = 1;
    pub const kPrefHintLoadStreamed: i32 = 4;
    pub const kPrefHintStoreStreamed: i32 = 5;
    pub const kPrefHintLoadRetained: i32 = 6;
    pub const kPrefHintStoreRetained: i32 = 7;
    pub const kPrefHintWritebackInvalidate: i32 = 25;
    pub const kPrefHintPrepareForStore: i32 = 30;

    // Helper functions for converting between register numbers and names.
    pub struct Registers {}

    impl Registers {
        // Return the name of the register.
        pub fn name(reg: i32) -> &'static str {
            if (0 <= reg) && (reg < kNumSimuRegisters) {
                Registers::names()[reg as usize]
            } else {
                "noreg"
            }
        }

        // Lookup the register number for the name provided.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for i in 0..kNumSimuRegisters {
                if Registers::names()[i as usize] == name {
                    return i;
                }
            }

            // Look through the alias names.
            for alias in Registers::aliases() {
                if alias.name == name {
                    return alias.reg;
                }
            }

            // No register with the requested name found.
            return kInvalidRegister;
        }

        fn names() -> [&'static str; kNumSimuRegisters as usize] {
            [
                "zero_reg", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1",
                "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8",
                "s9", "s10", "s11", "t3", "t4", "t5", "t6", "pc",
            ]
        }

        fn aliases() -> [RegisterAlias; 3] {
            [
                RegisterAlias { reg: 0, name: "zero" },
                RegisterAlias { reg: 33, name: "pc" },
                RegisterAlias { reg: 8, name: "s0_fp" },
            ]
        }
    }

    #[derive(Copy, Clone)]
    pub struct RegisterAlias {
        pub reg: i32,
        pub name: &'static str,
    }

    // Helper functions for converting between register numbers and names.
    pub struct FPURegisters {}

    impl FPURegisters {
        // Return the name of the register.
        pub fn name(reg: i32) -> &'static str {
            if (0 <= reg) && (reg < kNumFPURegisters) {
                FPURegisters::names()[reg as usize]
            } else {
                "nocreg"
            }
        }

        // Lookup the register number for the name provided.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for i in 0..kNumFPURegisters {
                if FPURegisters::names()[i as usize] == name {
                    return i;
                }
            }

            // Look through the alias names.
            for alias in FPURegisters::aliases() {
                if alias.name == name {
                    return alias.creg;
                }
            }

            // No Cregister with the requested name found.
            return kInvalidFPURegister;
        }

        fn names() -> [&'static str; kNumFPURegisters as usize] {
            [
                "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0",
                "fa1", "fa2", "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5",
                "fs6", "fs7", "fs8", "fs9", "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
            ]
        }

        fn aliases() -> [RegisterAlias; 1] {
            [RegisterAlias {
                creg: kInvalidFPURegister,
                name: std::ptr::null() as *const str as &'static str,
            }]
        }
    }

    // Helper functions for converting between register numbers and names.
    pub struct VRegisters {}

    impl VRegisters {
        // Return the name of the register.
        pub fn name(reg: i32) -> &'static str {
            if (0 <= reg) && (reg < kNumVRegisters) {
                VRegisters::names()[reg as usize]
            } else {
                "nocreg"
            }
        }

        // Lookup the register number for the name provided.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for i in 0..kNumVRegisters {
                if VRegisters::names()[i as usize] == name {
                    return i;
                }
            }

            // Look through the alias names.
            for alias in VRegisters::aliases() {
                if alias.name == name {
                    return alias.creg;
                }
            }

            // No Cregister with the requested name found.
            return kInvalidVRegister;
        }

        fn names() -> [&'static str; kNumVRegisters as usize] {
            [
                "v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8", "v9", "v10", "v11", "v12",
                "v13", "v14", "v15", "v16", "v17", "v18", "v19", "v20", "v21", "v22", "v23",
                "v24", "v25", "v26", "v27", "v28", "v29", "v30", "v31",
            ]
        }

        fn aliases() -> [RegisterAlias; 1] {
            [RegisterAlias {
                creg: kInvalidVRegister,
                name: std::ptr::null() as *const str as &'static str,
            }]
        }
    }

    // -----------------------------------------------------------------------------
    // Instructions encoding constants.

    // On RISCV all instructions are 32 bits, except for RVC.
    pub type Instr = i32;
    pub type ShortInstr = i16;

    // Special Software Interrupt codes when used in the presence of the RISC-V
    // simulator.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum SoftwareInterruptCodes {
        // Transition to C code.
        call_rt_redirected = 0xfffff,
    }

    // On RISC-V Simulator breakpoints can have different codes:
    // - Breaks between 0 and kMaxWatchpointCode are treated as simple watchpoints,
    //   the simulator will run through them and print the registers.
    // - Breaks between kMaxWatchpointCode and kMaxStopCode are treated as stop()
    //   instructions (see Assembler::stop()).
    // - Breaks larger than kMaxStopCode are simple breaks, dropping you into the
    //   debugger.
    pub const kMaxTracepointCode: u32 = 63;
    pub const kMaxWatchpointCode: u32 = 31;
    // Indicate that the stack is being switched, so the simulator must update its
    // stack limit. The new stack limit is passed in t6.
    pub const kExceptionIsSwitchStackLimit: u32 = 128;
    pub const kMaxStopCode: u32 = 127;

    // Debug parameters.
    //
    // For example:
    //
    // __ Debug(TRACE_ENABLE | LOG_TRACE);
    // starts tracing: set v8_flags.trace-sim is true.
    // __ Debug(TRACE_ENABLE | LOG_REGS);
    // PrintAllregs.
    // __ Debug(TRACE_DISABLE | LOG_TRACE);
    // stops tracing: set v8_flags.trace-sim is false.
    pub const kDebuggerTracingDirectivesMask: u32 = 0b111 << 3;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum DebugParameters {
        NO_PARAM = 1 << 5,
        BREAK = 1 << 0,
        LOG_TRACE = 1 << 1,
        LOG_REGS = 1 << 2,
        LOG_ALL = LOG_TRACE as u32,
        // Trace control.
        TRACE_ENABLE = 1 << 3 | NO_PARAM as u32,
        TRACE_DISABLE = 1 << 4 | NO_PARAM as u32,
    }

    // ----- Fields offset and length.
    // RISCV constants
    pub const kBaseOpcodeShift: i32 = 0;
    pub const kBaseOpcodeBits: i32 = 7;
    pub const kFunct6Shift: i32 = 26;
    pub const kFunct6Bits: i32 = 6;
    pub const kFunct7Shift: i32 = 25;
    pub const kFunct7Bits: i32 = 7;
    pub const kFunct5Shift: i32 = 27;
    pub const kFunct5Bits: i32 = 5;
    pub const kFunct3Shift: i32 = 12;
    pub const kFunct3Bits: i32 = 3;
    pub const kFunct2Shift: i32 = 25;
    pub const kFunct2Bits: i32 = 2;
    pub const kRs1Shift: i32 = 15;
    pub const kRs1Bits: i32 = 5;
    pub const kVs1Shift: i32 = 15;
    pub const kVs1Bits: i32 = 5;
    pub const kVs2Shift: i32 = 20;
    pub const kVs2Bits: i32 = 5;
    pub const kVdShift: i32 = 7;
    pub const kVdBits: i32 = 5;
    pub const kRs2Shift: i32 = 20;
    pub const kRs2Bits: i32 = 5;
    pub const kRs3Shift: i32 = 27;
    pub const kRs3Bits: i32 = 5;
    pub const kRdShift: i32 = 7;
    pub const kRdBits: i32 = 5;
    pub const kRlShift: i32 = 25;
    pub const kAqShift: i32 = 26;
    pub const kImm12Shift: i32 = 20;
    pub const kImm12Bits: i32 = 12;
    pub const kImm11Shift: i32 = 2;
    pub const kImm11Bits: i32 = 11;
    pub const kShamtShift: i32 = 20;
    pub const kShamtBits: i32 = 5;
    pub const kShamtMask: u32 = (((1 << kShamtBits) - 1) << kShamtShift) as u32;
    pub const kShamtWShift: i32 = 20;
    // FIXME: remove this once we have a proper way to handle the wide shift amount
    pub const kShamtWBits: i32 = 6;
    pub const kArithShiftShift: i32 = 30;
    pub const kImm20Shift: i32 = 12;
    pub const kImm20Bits: i32 = 20;
    pub const kCsrShift: i32 = 20;
    pub const kCsrBits: i32 = 12;
    pub const kMemOrderBits: i32 = 4;
    pub const kPredOrderShift: i32 = 24;
    pub const kSuccOrderShift: i32 = 20;

    // for C extension
    pub const kRvcFunct4Shift: i32 = 12;
    pub const kRvcFunct4Bits: i32 = 4;
    pub const kRvcFunct3Shift: i32 = 13;
    pub const kRvcFunct3Bits: i32 = 3;
    pub const kRvcRs1Shift: i32 = 7;
    pub const kRvcRs1Bits: i32 = 5;
    pub const kRvcRs2Shift: i32 = 2;
    pub const kRvcRs2Bits: i32 = 5;
    pub const kRvcRdShift: i32 = 7;
    pub const kRvcRdBits: i32 = 5;
    pub const kRvcRs1sShift: i32 = 7;
    pub const kRvcRs1sBits: i32 = 3;
    pub const kRvcRs2sShift: i32 = 2;
    pub const kRvcRs2sBits: i32 = 3;
    pub const kRvcFunct2Shift: i32 = 5;
    pub const kRvcFunct2BShift: i32 = 10;
    pub const kRvcFunct2Bits: i32 = 2;
    pub const kRvcFunct6Shift: i32 = 10;
    pub const kRvcFunct6Bits: i32 = 6;

    pub const kRvcOpcodeMask: u32 =
        0b11 | (((1 << kRvcFunct3Bits) - 1) << kRvcFunct3Shift);
    pub const kRvcFunct3Mask: u32 =
        (((1 << kRvcFunct3Bits) - 1) << kRvcFunct3Shift);
    pub const kRvcFunct4Mask: u32 =
        (((1 << kRvcFunct4Bits) - 1) << kRvcFunct4Shift);
    pub const kRvcFunct6Mask: u32 =
        (((1 << kRvcFunct6Bits) - 1) << kRvcFunct6Shift);
    pub const kRvcFunct2Mask: u32 =
        (((1 << kRvcFunct2Bits) - 1) << kRvcFunct2Shift);
    pub const kRvcFunct2BMask: u32 =
        (((1 << kRvcFunct2Bits) - 1) << kRvcFunct2BShift);
    pub const kCRTypeMask: u32 = kRvcOpcodeMask | kRvcFunct4Mask;
    pub const kCSTypeMask: u32 = kRvcOpcodeMask | kRvcFunct6Mask;
    pub const kCATypeMask: u32 =
        kRvcOpcodeMask | kRvcFunct6Mask | kRvcFunct2Mask;
    pub const kRvcBImm8Mask: u32 =
        (((1 << 5) - 1) << 2) | (((1 << 3) - 1) << 10);

    // for RVV extension
    pub const kRvvELEN: i32 = 64;
    pub const kRvvVLEN: i32 = 128;
    pub const kRvvSLEN: i32 = kRvvVLEN;

    pub const kRvvFunct6Shift: i32 = 26;
    pub const kRvvFunct6Bits: i32 = 6;
    pub const kRvvFunct6Mask: u32 =
        (((1 << kRvvFunct6Bits) - 1) << kRvvFunct6Shift);

    pub const kRvvVmBits: i32 = 1;
    pub const kRvvVmShift: i32 = 25;
    pub const kRvvVmMask: u32 = (((1 << kRvvVmBits) - 1) << kRvvVmShift);

    pub const kRvvVs2Bits: i32 = 5;
    pub const kRvvVs2Shift: i32 = 20;
    pub const kRvvVs2Mask: u32 = (((1 << kRvvVs2Bits) - 1) << kRvvVs2Shift);

    pub const kRvvVs1Bits: i32 = 5;
    pub const kRvvVs1Shift: i32 = 15;
    pub const kRvvVs1Mask: u32 = (((1 << kRvvVs1Bits) - 1) << kRvvVs1Shift);

    pub const kRvvRs1Bits: i32 = kRvvVs1Bits;
    pub const kRvvRs1Shift: i32 = kRvvVs1Shift;
    pub const kRvvRs1Mask: u32 = (((1 << kRvvRs1Bits) - 1) << kRvvRs1Shift);

    pub const kRvvRs2Bits: i32 = 5;
    pub const kRvvRs2Shift: i32 = 20;
    pub const kRvvRs2Mask: u32 = (((1 << kRvvRs2Bits) - 1) << kRvvRs2Shift);

    pub const kRvvImm5Bits: i32 = kRvvVs1Bits;
    pub const kRvvImm5Shift: i32 = kRvvVs1Shift;
    pub const kRvvImm5Mask: u32 = (((1 << kRvvImm5Bits) - 1) << kRvvImm5Shift);

    pub const kRvvVdBits: i32 = 5;
    pub const kRvvVdShift: i32 = 7;
    pub const kRvvVdMask: u32 = (((1 << kRvvVdBits) - 1) << kRvvVdShift);

    pub const kRvvRdBits: i32 = kRvvVdBits;
    pub const kRvvRdShift: i32 = kRvvVdShift;
    pub const kRvvRdMask: u32 = (((1 << kRvvRdBits) - 1) << kRvvRdShift);

    pub const kRvvZimmBits: i32 = 11;
    pub const kRvvZimmShift: i32 = 20;
    pub const kRvvZimmMask: u32 = (((1 << kRvvZimmBits) - 1) << kRvvZimmShift);

    pub const kRvvUimmShift: i32 = kRvvRs1Shift;
    pub const kRvvUimmBits: i32 = kRvvRs1Bits;
    pub const kRvvUimmMask: u32 = (((1 << kRvvUimmBits) - 1) << kRvvUimmShift);

    pub const kRvvWidthBits: i32 = 3;
    pub const kRvvWidthShift: i32 = 12;
    pub const kRvvWidthMask: u32 = (((1 << kRvvWidthBits) - 1) << kRvvWidthShift);

    pub const kRvvMopBits: i32 = 2;
    pub const kRvvMopShift: i32 = 26;
    pub const kRvvMopMask: u32 = (((1 << kRvvMopBits) - 1) << kRvvMopShift);

    pub const kRvvMewBits: i32 = 1;
    pub const kRvvMewShift: i32 = 28;
    pub const kRvvMewMask: u32 = (((1 << kRvvMewBits) - 1) << kRvvMewShift);

    pub const kRvvNfBits: i32 = 3;
    pub const kRvvNfShift: i32 = 29;
    pub const kRvvNfMask: u32 = (((1 << kRvvNfBits) - 1) << kRvvNfShift);

    // RISCV Instruction bit masks
    pub const kBaseOpcodeMask: u32 = ((1 << kBaseOpcodeBits) - 1) << kBaseOpcodeShift;
    pub const kFunct3Mask: u32 = ((1 << kFunct3Bits) - 1) << kFunct3Shift;
    pub const kFunct5Mask: u32 = ((1 << kFunct5Bits) - 1) << kFunct5Shift;
    pub const kFunct6Mask: u32 = ((1 << kFunct6Bits) - 1) << kFunct6Shift;
    pub const kFunct7Mask: u32 = ((1 << kFunct7Bits) - 1) << kFunct7Shift;
    pub const kFunct2Mask: u32 = 0b11 << kFunct7Shift;
    pub const kRTypeMask: u32 = kBaseOpcodeMask | kFunct3Mask | kFunct7Mask;
    pub const kRATypeMask: u32 = kBaseOpcodeMask | kFunct3Mask | kFunct5Mask;
    pub const kRFPTypeMask: u32 = kBaseOpcodeMask | kFunct7Mask;
    pub const kR4TypeMask: u32 = kBaseOpcodeMask | kFunct3Mask | kFunct2Mask;
    pub const kITypeMask: u32 = kBaseOpcodeMask | kFunct3Mask;
    pub const kSTypeMask: u32 = kBaseOpcodeMask | kFunct3Mask;
    pub const kBTypeMask: u32 = kBaseOpcodeMask | kFunct3Mask;
    pub const kUTypeMask: u32 = kBaseOpcodeMask;
    pub const kJTypeMask: u32 = kBaseOpcodeMask;
    pub const kVTypeMask: u32 = kRvvFunct6Mask | kFunct3Mask | kBaseOpcodeMask;
    pub const kRs1FieldMask: u32 = ((1 << kRs1Bits) - 1) << kRs1Shift;
    pub const kRs2FieldMask: u32 = ((1 << kRs2Bits) - 1) << kRs2Shift;
    pub const kRs3FieldMask: u32 = ((1 << kRs3Bits) - 1) << kRs3Shift;
    pub const kRdFieldMask: u32 = ((1 << kRdBits) - 1) << kRdShift;
    pub const kBImm12Mask: u32 = kFunct7Mask | kRdFieldMask;
    pub const kImm20Mask: u32 = ((1 << kImm20Bits) - 1) << kImm20Shift;
    pub const kImm12Mask: u32 = ((1 << kImm12Bits) - 1) << kImm12Shift;
    pub const kImm11Mask: u32 = ((1 << kImm11Bits) - 1) << kImm11Shift;
    pub const kImm31_12Mask: u32 = ((1 << 20) - 1) << 12;
    pub const kImm19_0Mask: u32 = ((1 << 20) - 1);

    pub const kNopByte: i32 = 0x00000013;
    // Original MIPS constants
    pub const kImm16Shift: i32 = 0;
    pub const kImm16Bits: i32 = 16;
    pub const kImm16Mask: u32 = ((1 << kImm16Bits) - 1) << kImm16Shift;

    // ----- Emulated conditions.
    // On RISC-V we use this enum to abstract from conditional branch instructions.
    // The 'U' prefix is used to specify unsigned comparisons.
    // Opposite conditions must be paired as odd/even numbers
    // because 'NegateCondition' function flips LSB to negate condition.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Condition {
        overflow = 0,
        no_overflow = 1,
        Uless = 2,
        Ugreater_equal = 3,
        Uless_equal = 4,
        Ugreater = 5,
        equal = 6,
        not_equal = 7, // Unordered or Not Equal.
        less = 8,
        greater_equal = 9,
        less_equal = 10,
        greater = 11,
        cc_always = 12,
    }

    impl Condition {
        // Returns the equivalent of !cc.
        pub fn negate_condition(cc: Condition) -> Condition {
            if cc == Condition::cc_always {
                panic!("Cannot negate cc_always condition");
            }
            unsafe { std::mem::transmute((cc as i32) ^ 1) }
        }

        pub fn negate_fpu_condition(cc: Condition) -> Condition {
            if cc == Condition::cc_always {
                return cc;
            }
            match cc {
                Condition::Uless => Condition::greater_equal,
                Condition::Ugreater => Condition::less_equal,
                Condition::Ugreater_equal => Condition::less,
                Condition::Uless_equal => Condition::greater,
                Condition::less => Condition::Ugreater_equal,
                Condition::greater => Condition::Uless_equal,
                Condition::greater_equal => Condition::Uless,
                Condition::less_equal => Condition::Ugreater,
                Condition::equal => Condition::not_equal,
                Condition::not_equal => Condition::equal,
                _ => cc,
            }
        }

        pub fn kEqual() -> Self {
            Condition::equal
        }

        pub fn kNotEqual() -> Self {
            Condition::not_equal
        }

        pub fn kLessThan() -> Self {
            Condition::less
        }

        pub fn kGreaterThan() -> Self {
            Condition::greater
        }

        pub fn kLessThanEqual() -> Self {
            Condition::less_equal
        }

        pub fn kGreaterThanEqual() -> Self {
            Condition::greater_equal
        }

        pub fn kUnsignedLessThan() -> Self {
            Condition::Uless
        }

        pub fn kUnsignedGreaterThan() -> Self {
            Condition::Ugreater
        }

        pub fn kUnsignedLessThanEqual() -> Self {
            Condition::Uless_equal
        }

        pub fn kUnsignedGreaterThanEqual() -> Self {
            Condition::Ugreater_equal
        }

        pub fn kOverflow() -> Self {
            Condition::overflow
        }

        pub fn kNoOverflow() -> Self {
            Condition::no_overflow
        }

        pub fn kZero() -> Self {
            Condition::equal
        }

        pub fn kNotZero() -> Self {
            Condition::not_equal
        }

    }

    // ----- Coprocessor conditions.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum FPUCondition {
        kNoFPUCondition = -1,
        EQ = 0x02, // Ordered and Equal
        NE = 0x03, // Unordered or Not Equal
        LT = 0x04, // Ordered and Less Than
        GE = 0x05, // Ordered and Greater Than or Equal
        LE = 0x06, // Ordered and Less Than or Equal
        GT = 0x07, // Ordered and Greater Than
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum CheckForInexactConversion {
        kCheckForInexactConversion,
        kDontCheckForInexactConversion,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum MaxMinKind {
        kMin = 0,
        kMax = 1,
    }

    // ----------------------------------------------------------------------------
    // RISCV flags

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum ControlStatusReg {
        csr_fflags = 0x001, // Floating-Point Accrued Exceptions (RW)

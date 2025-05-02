// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(target_arch = "s390x")]
mod s390_disasm {
    use std::{ffi::CString, fmt, mem::size_of, os::raw::c_char, ptr, slice};

    //use crate::base::platform::platform; // Assuming a similar structure in Rust
    //use crate::base::strings; // Assuming a similar structure in Rust
    //use crate::base::vector::Vector; // Assuming a similar structure in Rust
    //use crate::codegen::macro_assembler::MacroAssembler; // Assuming a similar structure in Rust
    //use crate::codegen::register_configuration::RegisterConfiguration; // Assuming a similar structure in Rust
    //use crate::codegen::s390::constants_s390::*; // Assuming a similar structure in Rust
    //use crate::diagnostics::disasm::*; // Assuming a similar structure in Rust

    // Placeholder traits and enums

    trait InstructionTrait {
        fn instruction_length(&self) -> usize;
        fn bits<T, U>(&self, start: usize, end: usize) -> U;
        fn bit(&self, bit: usize) -> u8;
        fn s390_opcode_value(&self) -> Opcode;
        fn instruction_bits<T>(&self) -> T;
    }

    trait RRInstructionTrait {
        fn r1_value(&self) -> i32;
        fn r2_value(&self) -> i32;
    }

    trait RXInstructionTrait {
        fn b2_value(&self) -> i32;
    }

    trait RRFInstructionTrait {
        fn m4_value(&self) -> i32;
    }

    trait VRR_C_InstructionTrait {
        fn m4_value(&self) -> i32;
        fn m5_value(&self) -> i32;
        fn m6_value(&self) -> i32;
    }

    trait VRR_E_InstructionTrait {
        fn r4_value(&self) -> i32;
    }

    trait RSInstructionTrait {
        fn d2_value(&self) -> u16;
    }

    trait RXYInstructionTrait {
        fn d2_value(&self) -> i32;
    }

    trait SSInstructionTrait {
        fn d2_value(&self) -> u16;
        fn d1_value(&self) -> u16;
        fn length(&self) -> u8;
    }

    trait RIInstructionTrait {
        fn i2_value(&self) -> i16;
        fn i2_unsigned_value(&self) -> u16;
    }

    trait RILInstructionTrait {
        fn i2_value(&self) -> i32;
        fn i2_unsigned_value(&self) -> u32;
    }

    trait IInstructionTrait {
        fn ivalue(&self) -> i8;
    }

    trait RIEInstructionTrait {
        fn i3_value(&self) -> u8;
        fn i4_value(&self) -> u8;
        fn i5_value(&self) -> u8;
    }

    trait SILInstructionTrait {
        fn i2_value(&self) -> i16;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Opcode {
        Unknown,
        BKPT,
        DUMY,
        LDR,
        BCR,
        OR,
        CR,
        MR,
        HER_Z,
        BRAS,
        MDBR,
        SDBR,
        ADBR,
        CDBR,
        MEEBR,
        SQDBR,
        SQEBR,
        LCDBR,
        LCEBR,
        LTEBR,
        LDEBR,
        CEBR,
        AEBR,
        SEBR,
        DEBR,
        LTDBR,
        LDGR,
        DDBR,
        LZDR,
        FIEBRA,
        FIDBRA,
        IC_z,
        AL,
        LE,
        LD,
        STE,
        STD,
        TRAP4,
        CFI,
        CGFI,
        AFI,
        AGFI,
        MSFI,
        MSGFI,
        ALSIH,
        ALSIHN,
        CIH,
        AIH,
        LGFI,
        ASI,
        AGSI,
        LT,
        LDY,
        LEY,
        STDY,
        STEY,
        LDEB,

        // Generic opcodes
        LR,
        L,
        LA,
        ST,
        AR,
        SR,
        DR,
        MR2,
        DR2,
        NR,
        XR,
        OC,
        TM,
        CLC,
        MVC,
        MVCL,
        CLCL,
        EX,
        BALR,
        BASR,

        // RS A opcodes
        LRA,
        SLA,
        SDA,
        MULA,
        DDA,

        // RSI opcodes
        LLSI,
        RLSI,

        // RI A opcodes
        LH,
        AH,
        CH,
        STH,

        // RI B opcodes
        LGF,
        AGF,
        CGF,
        STG,

        // RI C opcodes
        NI,
        XI,
        OI,

        // RRE opcodes
        MD,
        SD,
        AD,
        CD,
        MEEB,
        SQD,
        SQE,
        LCD,
        LCE,
        LTE,
        LDE,
        CE,
        AE,
        SE,
        DE,
        LTD,
        LDG,
        DD,
        LZD,

        // RRF A opcodes
        FIEBR,
        FIDEBR,

        // RRF C opcodes
        LRV,
        MRV,
        DRV,
        NRV,
        XRV,
        ORV,
        CRV,

        // RRF E opcodes
        LPER,
        LNER,
        LTER,
        LDR2,
        CER,
        AER,
        SER,
        DER,
        LTDR2,
        CDRB,
        ADRB,
        SDRB,
        DDRB,

        // RX A opcodes
        IC,
        L_z,
        ST_z,
        LH_z,
        STH_z,
        AL_z,
        S_z,
        M_z,
        D_z,
        LE_z,
        LD_z,
        STE_z,
        STD_z,
        LGF_z,
        STG_z,

        // RX B opcodes
        TM_z,
        NI_z,
        XI_z,
        OI_z,

        // RRD opcodes
        LRER,
        LRDR,
        LPER2,
        LNER2,
        LTER2,
        LDRB,
        CER2,
        AER2,
        SER2,
        DER2,
        LTDRB,
        LDGR2,
        CDRB2,
        ADRB2,
        SDRB2,
        DDRB2,

        // SI opcodes
        CLI,
        MVI,

        // VRR A opcodes
        VFMAEB,
        VMSAEB,
        VMAEGB,
        VMSEGB,
        VPDI,

        // VRR B opcodes
        VMULEB,
        VMSLEB,
        VMLAGB,
        VMSLGB,
        VMFI,
        VLEIF,

        // VRR C opcodes
        VLDE,
        VSTE,
        VLSD,
        VSTD,
        VLVGP,
        VLVEP,
        VLVSGP,
        VLVSEP,

        // VRR E opcodes
        VSTEB,
        VSTDB,

        // VRR F opcodes
        VLR,
        VLER,
        VLDR,

        // VRX opcodes
        VLLE,
        VLLD,
        VSTLL,
        VSTLG,

        // VRS A opcodes
        VFMA,
        VMSAL,
        VMSU,
        VMLA,
        VMSL,
        VMSAU,

        // VRS B opcodes
        VLRV,
        VLEV,
        VLDV,
        VSTLV,
        VSTGV,

        // VRS C opcodes
        VLV,

        // VRI A opcodes
        VFAI,
        VFSI,
        VFMI,
        VFDI,
        VFEEI,
        VFDEI,

        // VRI C opcodes
        VLAE,
        VLSE,
        VMAE,
        VMSE,
        VLEE,
        VLDE,
        VLSEE,
        VLDEE,

        // RIL A opcodes
        LB,
        STB,
        LG,
        STG2,
        LLGH,
        LLGF,
        LLG2,
        LLGB,
        IGF,
        MVG,
        CS,
        CDS,
        OCB,
        NCB,
        XCB,

        // RIL B opcodes
        LB_z,
        LG_z,
        STB_z,
        STG_z,

        // RIL C opcodes
        NI_z2,
        XI_z2,
        OI_z2,

        // SIY opcodes
        CLIY,
        MVIY,

        // RIE D opcodes
        CLRIO,
        CLGRJ,

        // RIE E opcodes
        CRJ,
        CERJ,
        ARJ,
        AERJ,
        SRJ,
        SERJ,
        MRJ,
        MERJ,
        DRJ,
        DERJ,
        ALRJ,
        ALERJ,
        SLRJ,
        SLERJ,

        // RIE F opcodes
        TRTT,
        TRTO,

        // RSY A opcodes
        LHY,
        AHY,
        CHY,
        STHY,

        // RSY B opcodes
        TMY,
        NIY,
        XIY,
        OIY,

        // RXY A opcodes
        L_z2,
        ST_z2,
        LH_z2,
        STH_z2,
        AL_z2,
        S_z2,
        M_z2,
        D_z2,
        LGF_z2,
        STG_z2,
        LT_z,
        LGFY,

        // RXY B opcodes
        TM_z2,
        NI_z3,
        XI_z3,
        OI_z3,

        // RXE opcodes
        LDE,
        LEE,
        STE2,
        STD2,
        LDEB2,

        // SIL opcodes
        LLILL,

        // SS A opcodes
        MVC2,
        CLMP,
        MVCP,
        TR,
        TRT,
    }

    // Implementations for the traits

    struct Instruction {
        bytes: Vec<u8>,
    }

    impl Instruction {
        fn at(instr_ptr: *const u8) -> Self {
            // Assuming the instruction is at least 2 bytes long to read the opcode.
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for S390
            Instruction { bytes: instruction_slice.to_vec() }
        }
    }

    impl InstructionTrait for Instruction {
        fn instruction_length(&self) -> usize {
            if self.bytes.len() < 2 {
                return 0;
            }

            let first_byte = self.bytes[0];
            let second_byte = self.bytes[1];

            if (first_byte & 0xF0) != 0 {
                2 // RR, SI, RI, RX
            } else if (first_byte == 0x00 || first_byte == 0x01) && (second_byte & 0x02) != 0 {
                6 // S format
            }
             else if (first_byte == 0x00 || first_byte == 0x01) {
                4 // trap4 format
            }
            else if (first_byte & 0x04) != 0{
                4 // RS
            } else {
                6 // SS, RIL, RIE, RSY, RXY, RXE, VRS, VRI, VRR
            }
        }

        fn bits<T, U>(&self, start: usize, end: usize) -> U
        where
            U: From<i32>,
        {
            let mut value: i32 = 0;
            let instruction_length = self.instruction_length();
            for i in start..=end {
                if i < instruction_length * 8 {
                    let byte_index = i / 8;
                    let bit_index = 7 - (i % 8); // Assuming big-endian
                    let bit = (self.bytes[byte_index] >> bit_index) & 1;
                    value = (value << 1) | (bit as i32);
                } else {
                    value <<= 1; // Pad with zeros for out-of-bounds bits.
                }
            }
            value.into()
        }

        fn bit(&self, bit: usize) -> u8 {
            if bit < self.bytes.len() * 8 {
                let byte_index = bit / 8;
                let bit_index = 7 - (bit % 8); // Assuming big-endian
                (self.bytes[byte_index] >> bit_index) & 1
            } else {
                0 // Return 0 for out-of-bounds bits.
            }
        }

        fn s390_opcode_value(&self) -> Opcode {
            if self.bytes.is_empty() {
                return Opcode::Unknown;
            }
            // Simplified opcode decoding logic based on the first byte for example
            match self.bytes[0] {
                0x0A => Opcode::L,
                0x18 => Opcode::LR,
                0x16 => Opcode::OC,
                0xB2 => Opcode::VLR,
                0xC0 => Opcode::MVC,
                _ => Opcode::Unknown, // Replace with more complete decoding
            }
        }
        fn instruction_bits<T>(&self) -> T {
            unsafe {
                let ptr = self.bytes.as_ptr() as *const T;
                ptr.read_unaligned()
            }
        }
    }

    struct RRInstruction {
        bytes: Vec<u8>,
    }

    impl RRInstruction {
        fn at(instr_ptr: *const u8) -> Self {
             let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 2) }; // Max 2 bytes for RR
            RRInstruction { bytes: instruction_slice.to_vec() }
        }
    }

    impl RRInstructionTrait for RRInstruction {
        fn r1_value(&self) -> i32 {
            (self.bytes[0] as i32 & 0x0F)
        }

        fn r2_value(&self) -> i32 {
            (self.bytes[1] as i32 & 0xF0) >> 4
        }
    }

    struct RXInstruction {
        bytes: Vec<u8>,
    }

     impl RXInstruction {
        fn at(instr_ptr: *const u8) -> Self {
             let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 4) }; // Max 4 bytes for RX
            RXInstruction { bytes: instruction_slice.to_vec() }
        }
    }

    impl RXInstructionTrait for RXInstruction {
        fn b2_value(&self) -> i32 {
            (self.bytes[2] as i32 & 0xF0) >> 4
        }
    }

    struct RRFInstruction {
        bytes: Vec<u8>,
    }

    impl RRFInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 4) }; // Max 4 bytes for RRF
           RRFInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RRFInstructionTrait for RRFInstruction {
        fn m4_value(&self) -> i32 {
            (self.bytes[3] as i32 & 0x0F)
        }
    }

    struct VRR_C_Instruction {
        bytes: Vec<u8>,
    }

    impl VRR_C_Instruction {
        fn at(instr_ptr: *const u8) -> Self {
             let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for VRR_C
            VRR_C_Instruction { bytes: instruction_slice.to_vec() }
        }
    }

    impl VRR_C_InstructionTrait for VRR_C_Instruction {
        fn m4_value(&self) -> i32 {
            (self.bytes[4] as i32 & 0xF0) >> 4
        }

        fn m5_value(&self) -> i32 {
            (self.bytes[4] as i32 & 0x0F)
        }

        fn m6_value(&self) -> i32 {
            (self.bytes[5] as i32 & 0xF0) >> 4
        }
    }

    struct VRR_E_Instruction {
        bytes: Vec<u8>,
    }

    impl VRR_E_Instruction {
        fn at(instr_ptr: *const u8) -> Self {
             let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for VRR_E
            VRR_E_Instruction { bytes: instruction_slice.to_vec() }
        }
    }

    impl VRR_E_InstructionTrait for VRR_E_Instruction {
        fn r4_value(&self) -> i32 {
            (self.bytes[3] as i32 & 0x0F)
        }
    }

    struct RSInstruction {
        bytes: Vec<u8>,
    }

    impl RSInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 4) }; // Max 4 bytes for RS
           RSInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RSInstructionTrait for RSInstruction {
        fn d2_value(&self) -> u16 {
           ((self.bytes[2] as u16) << 4) | ((self.bytes[3] as u16) & 0x0F)
        }
    }

    struct RXYInstruction {
        bytes: Vec<u8>,
    }

    impl RXYInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for RXY
           RXYInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RXYInstructionTrait for RXYInstruction {
        fn d2_value(&self) -> i32 {
            ((self.bytes[2] as i32) << 20) | ((self.bytes[3] as i32) << 12) | ((self.bytes[4] as i32) << 4) | (self.bytes[5] as i32 & 0x0F)
        }
    }

    struct SSInstruction {
        bytes: Vec<u8>,
    }

    impl SSInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for SS
           SSInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl SSInstructionTrait for SSInstruction {
        fn d2_value(&self) -> u16 {
            ((self.bytes[4] as u16) << 4) | ((self.bytes[5] as u16) & 0x0F)
        }

        fn d1_value(&self) -> u16 {
           ((self.bytes[2] as u16) << 4) | ((self.bytes[3] as u16) & 0x0F)
        }

        fn length(&self) -> u8 {
            self.bytes[1]
        }
    }

    struct RIInstruction {
        bytes: Vec<u8>,
    }

    impl RIInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 4) }; // Max 4 bytes for RI
           RIInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RIInstructionTrait for RIInstruction {
        fn i2_value(&self) -> i16 {
            ((self.bytes[2] as i16) << 8) | (self.bytes[3] as i16)
        }

        fn i2_unsigned_value(&self) -> u16 {
            ((self.bytes[2] as u16) << 8) | (self.bytes[3] as u16)
        }
    }

    struct RILInstruction {
        bytes: Vec<u8>,
    }

    impl RILInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for RIL
           RILInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RILInstructionTrait for RILInstruction {
        fn i2_value(&self) -> i32 {
            ((self.bytes[2] as i32) << 24) | ((self.bytes[3] as i32) << 16) | ((self.bytes[4] as i32) << 8) | (self.bytes[5] as i32)
        }

        fn i2_unsigned_value(&self) -> u32 {
            ((self.bytes[2] as u32) << 24) | ((self.bytes[3] as u32) << 16) | ((self.bytes[4] as u32) << 8) | (self.bytes[5] as u32)
        }
    }

    struct IInstruction {
        bytes: Vec<u8>,
    }

    impl IInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 4) }; // Max 4 bytes for I
           IInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl IInstructionTrait for IInstruction {
        fn ivalue(&self) -> i8 {
            self.bytes[3] as i8
        }
    }

    struct RIEInstruction {
        bytes: Vec<u8>,
    }

    impl RIEInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for RIE
           RIEInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl RIEInstructionTrait for RIEInstruction {
        fn i3_value(&self) -> u8 {
            self.bytes[2]
        }

        fn i4_value(&self) -> u8 {
            self.bytes[3]
        }

        fn i5_value(&self) -> u8 {
            self.bytes[4]
        }
    }

    struct SILInstruction {
        bytes: Vec<u8>,
    }

    impl SILInstruction {
        fn at(instr_ptr: *const u8) -> Self {
            let instruction_slice = unsafe { slice::from_raw_parts(instr_ptr, 6) }; // Max 6 bytes for SIL
           SILInstruction { bytes: instruction_slice.to_vec() }
       }
   }

    impl SILInstructionTrait for SILInstruction {
        fn i2_value(&self) -> i16 {
            ((self.bytes[4] as i16) << 8) | (self.bytes[5] as i16)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum SoftwareInterruptCodes {
        kCallRtRedirected,
        kBreakpoint,
        kStopCode,
    }

    const kStopCodeMask: i32 = 0x0000FFFF;

    //------------------------------------------------------------------------------

    /// Decoder decodes and disassembles instructions into an output buffer.
    /// It uses the converter to convert register names and call destinations into
    /// more informative description.
    struct Decoder<'a> {
        converter: &'a dyn disasm::NameConverter,
        out_buffer: Vec<u8>,
        out_buffer_pos: usize,
    }

    impl<'a> Decoder<'a> {
        fn new(converter: &'a dyn disasm::NameConverter, buffer_size: usize) -> Self {
            let mut out_buffer = vec![0u8; buffer_size];
            out_buffer[0] = 0;
            Decoder {
                converter,
                out_buffer,
                out_buffer_pos: 0,
            }
        }

        /// Writes one disassembled instruction into 'buffer' (0-terminated).
        /// Returns the length of the disassembled machine instruction in bytes.
        fn instruction_decode(&mut self, instruction_ptr: *mut u8) -> usize {
            let instruction = Instruction::at(instruction_ptr);
            let instruction_length = instruction.instruction_length();

            // Print the Instruction bits.
            if instruction_length == 2 {
                self.out_buffer_pos += format!(
                    self,
                    "{:04x}           ",
                    instruction.instruction_bits::<u16>()
                )
                .unwrap();
            } else if instruction_length == 4 {
                self.out_buffer_pos += format!(
                    self,
                    "{:08x}       ",
                    instruction.instruction_bits::<u32>()
                )
                .unwrap();
            } else {
                self.out_buffer_pos += format!(
                    self,
                    "{:012x}   ",
                    instruction.instruction_bits::<u64>()
                )
                .unwrap();
            }

            let mut decoded = self.decode_special(&instruction);
            if !decoded {
                decoded = self.decode_generic(&instruction);
            }
            if !decoded {
                self.unknown(&instruction);
            }
            instruction_length
        }

        /// Bottleneck functions to print into the out_buffer.
        fn print_char(&mut self, ch: char) {
            if self.out_buffer_pos < (self.out_buffer.len() - 1) {
                self.out_buffer[self.out_buffer_pos] = ch as u8;
                self.out_buffer_pos += 1;
                self.out_buffer[self.out_buffer_pos] = 0;
            }
        }

        fn print(&mut self, str: &str) {
            for ch in str.chars() {
                self.print_char(ch);
            }
        }

        /// Printing of common values.
        fn print_register(&mut self, reg: i32) {
            self.print(self.converter.name_of_cpu_register(reg));
        }

        fn print_d_register(&mut self, reg: i32) {
            // Assuming DoubleRegister::from_code(reg) is handled within RegisterName
            // and RegisterName function exists
            self.print(&register_name(DoubleRegister::from_code(reg)));
        }

        fn print_software_interrupt(&mut self, svc: SoftwareInterruptCodes) {
            match svc {
                SoftwareInterruptCodes::kCallRtRedirected => self.print("call rt redirected"),
                SoftwareInterruptCodes::kBreakpoint => self.print("breakpoint"),
                _ => {
                    if svc as i32 >= SoftwareInterruptCodes::kStopCode as i32 {
                        self.out_buffer_pos += format!(
                            self,
                            "{} - 0x{:x}",
                            (svc as i32) & kStopCodeMask,
                            (svc as i32) & kStopCodeMask
                        )
                        .unwrap();
                    } else {
                        self.out_buffer_pos += format!(self, "{}", svc as i32).unwrap();
                    }
                }
            }
        }

        /// Handle all register based formatting in this function to reduce the
        /// complexity of FormatOption.
        fn format_register(&mut self, instr: &Instruction, format: &str) -> usize {
            assert_eq!(format.chars().next().unwrap(), 'r');

            if format.chars().nth(1) == Some('1') {
                // 'r1: register resides in bit 8-11
                let reg: i32 = instr.bits::<SixByteInstr, i32>(39, 36);
                self.print_register(reg);
                2
            } else if format.chars().nth(1) == Some('2') {
                // 'r2: register resides in bit 12-15
                let reg: i32 = instr.bits::<SixByteInstr, i32>(35, 32);
                // indicating it is a r0 for displacement, in which case the offset
                // should be 0.
                if format.chars().nth(2) == Some('d') {
                    if reg == 0 {
                        return 4;
                    }
                    self.print_register(reg);
                    3
                } else {
                    self.print_register(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('3') {
                // 'r3: register resides in bit 16-19
                let reg: i32 = instr.bits::<SixByteInstr, i32>(31, 28);
                self.print_register(reg);
                2
            } else if format.chars().nth(1) == Some('4') {
                // 'r4: register resides in bit 20-23
                let reg: i32 = instr.bits::<SixByteInstr, i32>(27, 24);
                self.print_register(reg);
                2
            } else if format.chars().nth(1) == Some('5') {
                // 'r5: register resides in bit 24-27
                let reg: i32 = instr.bits::<SixByteInstr, i32>(23, 20);
                self.print_register(reg);
                2
            } else if format.chars().nth(1) == Some('6') {
                // 'r6: register resides in bit 28-31
                let reg: i32 = instr.bits::<SixByteInstr, i32>(19, 16);
                self.print_register(reg);
                2
            } else if format.chars().nth(1) == Some('7') {
                // 'r6: register resides in bit 32-35
                let reg: i32 = instr.bits::<SixByteInstr, i32>(15, 12);
                self.print_register(reg);
                2
            } else {
                unreachable!();
            }
        }

        fn format_floating_register(&mut self, instr: &Instruction, format: &str) -> usize {
            assert_eq!(format.chars().next().unwrap(), 'f');

            // reuse 1, 5 and 6 because it is coresponding
            if format.chars().nth(1) == Some('1') {
                // 'f1: register resides in bit 8-11
                let rrinstr = RRInstruction::at(instr.bytes.as_ptr());
                let reg = rrinstr.r1_value();
                self.print_d_register(reg);
                2
            } else if format.chars().nth(1) == Some('2') {
                // 'f2: register resides in bit 12-15
                let rrinstr = RRInstruction::at(instr.bytes.as_ptr());
                let reg = rrinstr.r2_value();
                self.print_d_register(reg);
                2
            } else if format.chars().nth(1) == Some('3') {
                // 'f3: register resides in bit 16-19
                // RRDInstruction is not defined in the code
                // let rrdinstr = RRDInstruction::at(instr.bytes.as_ptr());
                // let reg = rrdinstr.r1_value();
                // self.print_d_register(reg);
                todo!()
            } else if format.chars().nth(1) == Some('5') {
                
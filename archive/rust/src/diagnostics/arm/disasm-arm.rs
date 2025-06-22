// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A Disassembler object is used to disassemble a block of code instruction by
// instruction. The default implementation of the NameConverter object can be
// overriden to modify register names or to do symbol lookup on addresses.
//
// The example below will disassemble a block of code and print it to stdout.
//
//   NameConverter converter;
//   Disassembler d(converter);
//   for (uint8_t* pc = begin; pc < end;) {
//     v8::base::EmbeddedVector<char, 256> buffer;
//     uint8_t* prev_pc = pc;
//     pc += d.InstructionDecode(buffer, pc);
//     printf("%p    %08x      %s\n",
//            prev_pc, *reinterpret_cast<int32_t*>(prev_pc), buffer);
//   }
//
// The Disassembler class also has a convenience method to disassemble a block
// of code into a FILE*, meaning that the above functionality could also be
// achieved by just calling Disassembler::Disassemble(stdout, begin, end);

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(target_arch = "arm")]
mod arm {
    use std::{
        fmt::{self, Write},
        mem::MaybeUninit,
        ptr,
        slice,
        str,
    };

    use libc::FILE;

    // use crate::base::bits;  // Assuming base crate has bits module
    // use crate::base::platform::platform; // Assuming base crate has platform module
    // use crate::base::strings; // Assuming base crate has strings module
    // use crate::base::vector; // Assuming base crate has vector module
    // use crate::codegen::arm::assembler_arm; // Assuming codegen crate has assembler_arm module
    // use crate::codegen::arm::constants_arm; // Assuming codegen crate has constants_arm module
    // use crate::codegen::arm::register_arm; // Assuming codegen crate has register_arm module
    // use crate::diagnostics::disasm; // Assuming diagnostics crate has disasm module

    // Re-export potentially needed items from other modules
    // pub use assembler_arm::*;
    // pub use constants_arm::*;
    // pub use register_arm::*;
    // pub use disasm::*;

    // Placeholder definitions - replace with actual implementations

    pub type Address = usize; // Assuming Address is a usize
    pub const kInstrSize: usize = 4; // Assuming instruction size is 4 bytes
    pub type SoftwareInterruptCodes = i32;
    pub const kCallRtRedirected: SoftwareInterruptCodes = 1;
    pub const kBreakpoint: SoftwareInterruptCodes = 2;
    pub const kStopCode: SoftwareInterruptCodes = 1000;
    pub const kStopCodeMask: SoftwareInterruptCodes = 0xFFF;
    pub const kNumberOfConditions: usize = 16;
    pub const kPCRegister: i32 = 15;
    pub const kConstantPoolMarkerMask: i32 = 0xFF000000;
    pub const kConstantPoolMarker: i32 = 0xE2000000;
    pub const BX: i32 = 0;
    pub const BLX: i32 = 1;
    pub const BKPT: i32 = 2;
    pub const CLZ: i32 = 3;

    pub enum ShiftOp {
        LSL,
        LSR,
        ASR,
        ROR,
    }

    impl ShiftOp {
        fn to_index(&self) -> usize {
            match self {
                ShiftOp::LSL => 0,
                ShiftOp::LSR => 1,
                ShiftOp::ASR => 2,
                ShiftOp::ROR => 3,
            }
        }
    }

    pub const LSL: ShiftOp = ShiftOp::LSL;
    pub const LSR: ShiftOp = ShiftOp::LSR;
    pub const ASR: ShiftOp = ShiftOp::ASR;
    pub const ROR: ShiftOp = ShiftOp::ROR;
    pub const kNumberOfShifts: usize = 4;

    #[derive(Debug, Copy, Clone)]
    pub enum PUField {
        da_x,
        ia_x,
        db_x,
        ib_x,
    }

    use PUField::*;

    pub fn decode_constant_pool_length(bits: i32) -> i32 {
        (bits & 0x00FFFFFF) as i32
    }

    pub enum VFPRegPrecision {
        kSinglePrecision,
        kDoublePrecision,
        kSimd128Precision,
    }

    pub enum NeonListType {
        nlt_1,
        nlt_2,
        nlt_3,
        nlt_4,
    }

    use NeonListType::*;
    pub struct NeonListOperand {
        reg: DwVfpRegister,
        len: i32,
    }
    impl NeonListOperand {
        fn new(reg: DwVfpRegister, len: i32) -> Self {
            NeonListOperand { reg, len }
        }

        fn type_(&self) -> NeonListType {
            match self.len {
                1 => nlt_1,
                2 => nlt_2,
                3 => nlt_3,
                4 => nlt_4,
                _ => panic!("unexpected NeonList len"),
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum SwVfpRegister {
        SwVfpRegister(i32),
    }

    impl From<i32> for SwVfpRegister {
        fn from(code: i32) -> Self {
            SwVfpRegister(code)
        }
    }

    #[derive(Copy, Clone)]
    pub enum DwVfpRegister {
        DwVfpRegister(i32),
    }

    impl DwVfpRegister {
        fn from_code(code: i32) -> Self {
            DwVfpRegister(code)
        }
    }

    pub enum QwNeonRegister {
        QwNeonRegister(i32),
    }

    impl QwNeonRegister {
        fn from_code(code: i32) -> Self {
            QwNeonRegister(code)
        }
    }

    // Placeholder register type and functions. Needs real implementation.
    pub mod VFPRegisters {
        pub fn name(reg: i32, is_double: bool) -> &'static str {
            if is_double {
                Box::leak(format!("d{}", reg).into_boxed_str())
            } else {
                Box::leak(format!("s{}", reg).into_boxed_str())
            }
        }
    }

    pub fn register_name(reg: QwNeonRegister) -> &'static str {
        match reg {
            QwNeonRegister::QwNeonRegister(reg) => Box::leak(format!("q{}", reg).into_boxed_str()),
        }
    }

    pub trait NameConverterTrait {
        fn name_of_address(&self, addr: *const u8) -> &'static str;
        fn name_of_constant(&self, addr: *const u8) -> &'static str;
        fn name_of_cpu_register(&self, reg: i32) -> &'static str;
        fn name_of_byte_cpu_register(&self, reg: i32) -> &'static str;
        fn name_of_xmm_register(&self, reg: i32) -> &'static str;
        fn name_in_code(&self, addr: *const u8) -> &'static str;
    }

    /// A default name converter.
    pub struct NameConverter {}

    impl NameConverter {
        pub fn new() -> Self {
            NameConverter {}
        }
    }

    impl NameConverterTrait for NameConverter {
        fn name_of_address(&self, addr: *const u8) -> &'static str {
            Box::leak(format!("{:p}", addr).into_boxed_str())
        }

        fn name_of_constant(&self, addr: *const u8) -> &'static str {
            self.name_of_address(addr)
        }

        fn name_of_cpu_register(&self, reg: i32) -> &'static str {
            //Needs real implementation, register name must be immutable
            //Otherwise NameOfRegister will have a lifetime to the `self`
            Box::leak(format!("r{}", reg).into_boxed_str())
        }

        fn name_of_byte_cpu_register(&self, _reg: i32) -> &'static str {
            unimplemented!() // ARM does not have the concept of a byte register
        }

        fn name_of_xmm_register(&self, _reg: i32) -> &'static str {
            unimplemented!() // ARM does not have any XMM registers
        }

        fn name_in_code(&self, _addr: *const u8) -> &'static str {
            "" // The default name converter is called for unknown code.
        }
    }

    // Instruction related structs and enums.
    // Placeholder implementations. Replace with actual decoding logic.
    pub struct Instruction {
        instruction_bits: u32,
        address: Address,
    }

    impl Instruction {
        pub fn at(address: Address) -> *mut Instruction {
            address as *mut Instruction // Here be dragons!
        }

        pub fn instruction_bits(&self) -> u32 {
            self.instruction_bits
        }

        pub fn condition_field(&self) -> i32 {
            (self.instruction_bits >> 28 & 0xF) as i32
        }

        pub fn type_value(&self) -> i32 {
            (self.instruction_bits >> 25 & 0x7) as i32
        }

        pub fn has_b(&self) -> bool {
            (self.instruction_bits >> 22 & 0x1) == 1
        }

        pub fn has_h(&self) -> bool {
            (self.instruction_bits >> 22 & 0x1) == 1
        }

        pub fn has_link(&self) -> bool {
            (self.instruction_bits >> 24 & 0x1) == 1
        }

        pub fn has_s(&self) -> bool {
            (self.instruction_bits >> 20 & 0x1) == 1
        }

        pub fn has_l(&self) -> bool {
            (self.instruction_bits >> 20 & 0x1) == 1
        }

        pub fn has_w(&self) -> bool {
            (self.instruction_bits >> 21 & 0x1) == 1
        }

        pub fn has_sign(&self) -> bool {
            (self.instruction_bits >> 22 & 0x1) == 1
        }

        pub fn rn_value(&self) -> i32 {
            (self.instruction_bits >> 16 & 0xF) as i32
        }

        pub fn rd_value(&self) -> i32 {
            (self.instruction_bits >> 12 & 0xF) as i32
        }

        pub fn rs_value(&self) -> i32 {
            (self.instruction_bits >> 8 & 0xF) as i32
        }

        pub fn rm_value(&self) -> i32 {
            (self.instruction_bits & 0xF) as i32
        }

        pub fn rt_value(&self) -> i32 {
            (self.instruction_bits >> 12 & 0xF) as i32
        }

        pub fn rlist_value(&self) -> i32 {
            (self.instruction_bits & 0xFFFF) as i32
        }

        pub fn offset12_value(&self) -> i32 {
            (self.instruction_bits & 0xFFF) as i32
        }

        pub fn simmed24_value(&self) -> i32 {
            ((self.instruction_bits & 0xFFFFFF) << 8 >> 8) as i32
        }

        pub fn rotate_value(&self) -> i32 {
            (self.instruction_bits >> 8 & 0xF) as i32
        }

        pub fn immed8_value(&self) -> i32 {
            (self.instruction_bits & 0xFF) as i32
        }

        pub fn immovw_movt_value(&self) -> i32 {
            ((self.instruction_bits >> 4 & 0xF000) | (self.instruction_bits >> 16 & 0xFFF)) as i32
        }

        pub fn immedh_value(&self) -> i32 {
            (self.instruction_bits >> 8 & 0xF) as i32
        }

        pub fn immedl_value(&self) -> i32 {
            (self.instruction_bits & 0xF) as i32
        }

        pub fn shift_field(&self) -> ShiftOp {
            match (self.instruction_bits >> 5) & 0x3 {
                0 => ShiftOp::LSL,
                1 => ShiftOp::LSR,
                2 => ShiftOp::ASR,
                3 => ShiftOp::ROR,
                _ => panic!("Unexpected ShiftOp value"),
            }
        }

        pub fn shift_value(&self) -> i32 {
            self.shift_field().to_index() as i32
        }

        pub fn shift_amount_value(&self) -> i32 {
            (self.instruction_bits >> 7 & 0x1F) as i32
        }

        pub fn reg_shift_value(&self) -> i32 {
            (self.instruction_bits >> 4 & 0x1) as i32
        }

        pub fn pu_field(&self) -> PUField {
            match (self.instruction_bits >> 23) & 0x3 {
                0 => da_x,
                1 => ia_x,
                2 => db_x,
                3 => ib_x,
                _ => panic!("Unexpected PUField value"),
            }
        }

        pub fn condition_value(&self) -> i32 {
            (self.instruction_bits >> 28 & 0xF) as i32
        }

        pub fn svc_value(&self) -> SoftwareInterruptCodes {
            (self.instruction_bits & 0xFFFFFF) as SoftwareInterruptCodes
        }

        pub fn bits(&self, msb: i32, lsb: i32) -> i32 {
            ((self.instruction_bits >> lsb) & ((1 << (msb - lsb + 1)) - 1)) as i32
        }

        pub fn bit(&self, bit: i32) -> i32 {
            ((self.instruction_bits >> bit) & 0x1) as i32
        }

        pub fn bit_field(&self, msb: i32, lsb: i32) -> i32 {
            ((self.instruction_bits >> lsb) & ((1 << (msb - lsb + 1)) - 1)) as i32
        }

        pub fn is_special_type0(&self) -> bool {
            (self.instruction_bits >> 4 & 0x1) == 1
        }

        pub fn is_misc_type0(&self) -> bool {
            (self.instruction_bits >> 4 & 0x1) == 0
        }

        pub fn is_nop_like_type1(&self) -> bool {
            (self.instruction_bits >> 4 & 0x1) == 0
        }

        pub fn opcode_field(&self) -> i32 {
            (self.instruction_bits >> 21 & 0xF) as i32
        }

        pub fn coprocessor_value(&self) -> i32 {
            (self.instruction_bits >> 8 & 0xF) as i32
        }

        pub fn opc1_value(&self) -> i32 {
            (self.instruction_bits >> 19 & 0x7) as i32
        }

        pub fn opc2_value(&self) -> i32 {
            (self.instruction_bits >> 16 & 0x7) as i32
        }

        pub fn opc3_value(&self) -> i32 {
            (self.instruction_bits >> 5 & 0x3) as i32
        }

        pub fn sz_value(&self) -> i32 {
            (self.instruction_bits >> 18 & 0x1) as i32
        }

        pub fn vc_value(&self) -> i32 {
            (self.instruction_bits >> 7 & 0x1) as i32
        }

        pub fn va_value(&self) -> i32 {
            (self.instruction_bits >> 6 & 0x1) as i32
        }

        pub fn vl_value(&self) -> i32 {
            (self.instruction_bits >> 22 & 0x1) as i32
        }

        pub fn vfpn_reg_value(&self, precision: VFPRegPrecision) -> i32 {
            match precision {
                VFPRegPrecision::kSinglePrecision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
                VFPRegPrecision::kDoublePrecision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
                VFPRegPrecision::kSimd128Precision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
            }
        }

        pub fn vfpm_reg_value(&self, precision: VFPRegPrecision) -> i32 {
            match precision {
                VFPRegPrecision::kSinglePrecision => ((self.instruction_bits & 0xF) | (self.instruction_bits >> 5 & 0x10)) as i32,
                VFPRegPrecision::kDoublePrecision => ((self.instruction_bits & 0xF) | (self.instruction_bits >> 5 & 0x10)) as i32,
                VFPRegPrecision::kSimd128Precision => ((self.instruction_bits & 0xF) | (self.instruction_bits >> 5 & 0x10)) as i32,
            }
        }

        pub fn vfpd_reg_value(&self, precision: VFPRegPrecision) -> i32 {
            match precision {
                VFPRegPrecision::kSinglePrecision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
                VFPRegPrecision::kDoublePrecision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
                VFPRegPrecision::kSimd128Precision => ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32,
            }
        }

        pub fn vd_value(&self) -> i32 {
            ((self.instruction_bits >> 12 & 0xF) | (self.instruction_bits >> 16 & 0x10)) as i32
        }

        pub fn vn_value(&self) -> i32 {
            (self.instruction_bits >> 16 & 0xF) as i32
        }

        pub fn vm_value(&self) -> i32 {
            (self.instruction_bits & 0xF) as i32
        }

        pub const kPcLoadDelta: usize = 8;

        pub fn double_immed_vmov(&self) -> ImmdData {
            ImmdData {}
        }
    }

    pub struct ImmdData {}
    impl ImmdData {
        pub fn get_scalar(&self) -> f64 {
            0.0
        }
    }

    // Placeholder implementations - replace with actual implementations

    /// Decoder decodes and disassembles instructions into an output buffer.
    /// It uses the converter to convert register names and call destinations into
    /// more informative description.
    pub struct Decoder<'a> {
        converter_: &'a dyn NameConverterTrait,
        out_buffer_: Vec<u8>,
        out_buffer_pos_: usize,
    }

    impl<'a> Decoder<'a> {
        /// Creates a new Decoder.
        pub fn new(converter_: &'a dyn NameConverterTrait, buffer_size: usize) -> Self {
            let mut out_buffer_ = Vec::with_capacity(buffer_size);
            out_buffer_.resize(buffer_size, 0);
            let out_buffer_pos_ = 0;
            Decoder {
                converter_: converter_,
                out_buffer_: out_buffer_,
                out_buffer_pos_: out_buffer_pos_,
            }
        }

        /// Writes one disassembled instruction into 'buffer' (0-terminated).
        /// Returns the length of the disassembled machine instruction in bytes.
        pub fn instruction_decode(&mut self, instruction: *mut u8) -> i32 {
            let instr_ptr = instruction;
            let instr = unsafe { Instruction::at(instr_ptr as usize) };
            let instruction_bits = unsafe { (*(instr_ptr as *mut i32)) as u32 };
            unsafe { (*instr).instruction_bits = instruction_bits };
            self.format(&format!("{:08x}       ", unsafe { (*instr).instruction_bits() }));
            if unsafe { (*instr).condition_field() } == kSpecialCondition {
                self.decode_special_condition(unsafe { &*instr });
                return kInstrSize as i32;
            }

            if (instruction_bits as i32 & kConstantPoolMarkerMask) == kConstantPoolMarker {
                let len = decode_constant_pool_length(instruction_bits as i32);
                self.format(&format!("constant pool begin (length {})", len));
                return kInstrSize as i32;
            }

            match unsafe { (*instr).type_value() } {
                0 | 1 => self.decode_type01(unsafe { &*instr }),
                2 => self.decode_type2(unsafe { &*instr }),
                3 => self.decode_type3(unsafe { &*instr }),
                4 => self.decode_type4(unsafe { &*instr }),
                5 => self.decode_type5(unsafe { &*instr }),
                6 => self.decode_type6(unsafe { &*instr }),
                7 => return self.decode_type7(unsafe { &*instr }) as i32,
                _ => unreachable!(), // The type field is 3-bits in the ARM encoding.
            }

            kInstrSize as i32
        }

        fn format(&mut self, format_str: &str) {
            for &byte in format_str.as_bytes() {
                if self.out_buffer_pos_ < self.out_buffer_.len() - 1 {
                    self.out_buffer_[self.out_buffer_pos_] = byte;
                    self.out_buffer_pos_ += 1;
                }
            }
            self.out_buffer_[self.out_buffer_pos_] = 0;
        }

        fn format_option(&mut self, instr: &Instruction, format: &str) -> usize {
            match format.chars().next() {
                Some('a') => {
                    // 'a: accumulate multiplies
                    if instr.bit(21) == 0 {
                        self.format("ul");
                    } else {
                        self.format("la");
                    }
                    1
                }
                Some('b') => {
                    // 'b: byte loads or stores
                    if instr.has_b() {
                        self.format("b");
                    }
                    1
                }
                Some('c') => {
                    // 'cond: conditional execution
                    self.print_condition(instr);
                    4
                }
                Some('d') => {
                    // 'd: vmov double immediate.
                    let d = instr.double_immed_vmov().get_scalar();
                    self.format(&format!("#{}", d));
                    1
                }
                Some('f') => {
                    // 'f: bitfield instructions - v7 and above.
                    let lsbit = instr.bits(11, 7);
                    let width = instr.bits(20, 16) + 1;
                    if instr.bit(21) == 0 {
                        // BFC/BFI:
                        // Bits 20-16 represent most-significant bit. Convert to width.
                        let mut width = width - lsbit;
                        assert!(width > 0);
                    }
                    assert!(width + lsbit <= 32);
                    self.format(&format!("#{}, #{}", lsbit, width));
                    1
                }
                Some('h') => {
                    // 'h: halfword operation for extra loads and stores
                    if instr.has_h() {
                        self.format("h");
                    } else {
                        self.format("b");
                    }
                    1
                }
                Some('i') => {
                    // 'i: immediate value from adjacent bits.
                    let width = (format.chars().nth(3).unwrap() as i32 - '0' as i32) * 10
                        + (format.chars().nth(4).unwrap() as i32 - '0' as i32);
                    let lsb = (format.chars().nth(6).unwrap() as i32 - '0' as i32) * 10
                        + (format.chars().nth(7).unwrap() as i32 - '0' as i32);

                    assert!((width >= 1) && (width <= 32));
                    assert!((lsb >= 0) && (lsb <= 31));
                    assert!(width + lsb <= 32);

                    self.format(&format!("{}", instr.bits(width + lsb - 1, lsb)));
                    8
                }
                Some('l') => {
                    // 'l: branch and link
                    if instr.has_link() {
                        self.format("l");
                    }
                    1
                }
                Some('m') => {
                    match format.chars().nth(1) {
                        Some('w') => {
                            // 'mw: movt/movw instructions.
                            self.print_movw_movt(instr);
                            2
                        }
                        Some('e') => {
                            // 'memop: load/store instructions.
                            if instr.has_l() {
                                self.format("ldr");
                            } else {
                                if (instr.bits(27, 25) == 0)
                                    && (instr.bit(20) == 0)
                                    && (instr.bits(7, 6) == 3)
                                    && (instr.bit(4) == 1)
                                {
                                    if instr.bit(5) == 1 {
                                        self.format("strd");
                                    } else {
                                        self.format("ldrd");
                                    }
                                    5
                                } else {
                                    self.format("str");
                                }
                            }
                            5
                        }
                        Some('s') => {
                            // 'msg: for simulator break instructions
                            let str_ptr =
                                (instr.instruction_bits() & 0x0FFFFFFF) as *const u8;
                            let str_ = self.converter_.name_in_code(str_ptr);
                            self.format(str_);
                            3
                        }
                        _ => unreachable!(),
                    }
                }
                Some('o') => {
                    if format.chars().nth(3) == Some('1') && format.chars().nth(4) == Some('2') {
                        // 'off12: 12-bit offset for load and store instructions
                        self.format(&format!("{}", instr.offset12_value()));
                        5
                    } else if format.chars().nth(3) == Some('0') {
                        // 'off0to3and8to19 16-bit immediate encoded in bits 19-8 and 3-0.
                        self.format(&format!(
                            "{}",
                            (instr.bits(19, 8) << 4) + instr.bits(3, 0)
                        ));
                        15
                    } else {
                        // 'off8: 8-bit offset for extra load and store instructions
                        let offs8 = (instr.immedh_value() << 4) | instr.immedl_value();
                        self.format(&format!("{}", offs8));
                        4
                    }
                }
                Some('p') => {
                    // 'pu: P and U bits for load and store instructions
                    self.print_pu(instr);
                    2
                }
                Some('r') => return self.format_register(instr, format),
                Some('s') => {
                    if format.chars().nth(1) == Some('h') {
                        // 'shift_op or 'shift_rm or 'shift_sat.
                        match format.chars().nth(6) {
                            Some('o') => {
                                // 'shift_op
                                if instr.type_value() == 0 {
                                    self.print_shift_rm(instr);
                                } else {
                                    assert_eq!(instr.type_value(), 1);
                                    self.print_shift_imm(instr);
                                }
                                8
                            }
                            Some('s') => {
                                // 'shift_sat.
                                self.print_shift_sat(instr);
                                9
                            }
                            _ => {
                                // 'shift_rm
                                self.print_shift_rm(instr);
                                8
                            }
                        }
                    } else if format.chars().nth(1) == Some('v') {
                        // 'svc
                        self.print_software_interrupt(instr.svc_value());
                        3
                    } else if format.chars().nth(1) == Some('i') {
                        // 'sign: signed extra loads and stores
                        if format.chars().nth(2) == Some('g') {
                            if instr.has_sign() {
                                self.format("s");
                            }
                            4
                        } else {
                            // 'size2 or 'size3, for Advanced SIMD instructions, 2 or 3 registers.
                            let sz = 8 << (if format.chars().nth(4) == Some('2') {
                                instr.bits(19, 18)
                            } else {
                                instr.bits(21, 20)
                            });
                            self.format(&format!("{}", sz));
                            5
                        }
                    } else if format.chars().nth(1) == Some('p') {
                        if format.chars().nth(8) == Some('_') {
                            self.format("_");
                            let mask = instr.bits(19, 16);
                            if mask == 0 {
                                self.format("(none)");
                            }
                            if (mask & 0x8) != 0 {
                                self.format("f");
                            }
                            if (mask & 0x4) != 0 {
                                self.format("s");
                            }
                            if (mask & 0x2) != 0 {
                                self.format("x");
                            }
                            if (mask & 0x1) != 0 {
                                self.format("c");
                            }
                            15
                        } else {
                            if instr.bit(22) == 0 {
                                self.format("CPSR");
                            } else {
                                self.format("SPSR");
                            }
                            8
                        }
                    } else {
                        // 's: S field of data processing instructions
                        if instr.has_s() {
                            self.format("s");
                        }
                        1
                    }
                }
                Some('t') => {
                    // 'target: target of branch instructions
                    let off = ((instr.simmed24_value() as u32) << 2) as i32 + 8;
                    let target_addr = (instr as *const Instruction as usize as i32 + off) as usize;
                    let target_name = self.converter_.name_of_address(target_addr as *const u8);
                    self.format(&format!("+{} -> {}", off, target_name));
                    6
                }
                Some('u') => {
                    // 'u: signed or unsigned multiplies
                    if instr.bit(22) == 0 {
                        self.format("u");
                    } else {
                        self.format("s");
                    }
                    1
                }
                Some('v') => return self.format_vfpinstruction(instr, format),
                Some('A') => {
                    // Print pc-relative address.
                    let offset = instr.offset12_value();
                    let pc =
                        instr as *const Instruction as usize as *const u8;
                    let addr = match instr.pu_field() {
                        db_x => unsafe { pc.offset(Instruction::kPcLoadDelta as isize).offset(-(offset as isize)) },
                        ib_x => unsafe { pc.offset(Instruction::kPcLoadDelta as isize).offset(offset as isize) },
                        _ => unreachable!(),
                    };
                    self.format(&format!("0x{:08x}", addr as usize));
                    1
                }

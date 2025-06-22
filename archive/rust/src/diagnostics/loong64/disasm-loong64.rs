// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

#[cfg(target_arch = "loongarch64")]
mod loong64 {
    use std::fmt;
    use std::slice;
    use std::str;

    //use crate::base::platform::platform;
    //use crate::base::strings;
    //use crate::base::vector::Vector;
    //use crate::codegen::loong64::constants_loong64;
    //use crate::codegen::macro_assembler;
    //use crate::diagnostics::disasm;

    const K_INSTR_SIZE: usize = 4;

    macro_rules! string_starts_with {
        ($string:expr, $compare_string:expr) => {
            $string.starts_with($compare_string)
        };
    }

    /// Decoder decodes and disassembles instructions into an output buffer.
    /// It uses the converter to convert register names and call destinations into
    /// more informative description.
    pub struct Decoder<'a> {
        converter: &'a dyn NameConverter,
        out_buffer: Vec<char>,
        out_buffer_pos: usize,
    }

    impl<'a> Decoder<'a> {
        pub fn new(converter: &'a dyn NameConverter, capacity: usize) -> Self {
            let mut out_buffer = Vec::with_capacity(capacity);
            out_buffer.push('\0'); // Ensure null termination
            Decoder {
                converter,
                out_buffer,
                out_buffer_pos: 0,
            }
        }

        /// Writes one disassembled instruction into 'buffer' (0-terminated).
        /// Returns the length of the disassembled machine instruction in bytes.
        pub fn instruction_decode(&mut self, instruction: *mut u8) -> i32 {
            let instr = Instruction::at(instruction);
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{:08x}       ", instr.instruction_bits())) };

            match instr.instruction_type() {
                InstructionType::kOp6Type => {
                    self.decode_typek_op6(&instr);
                }
                InstructionType::kOp7Type => {
                    self.decode_typek_op7(&instr);
                }
                InstructionType::kOp8Type => {
                    self.decode_typek_op8(&instr);
                }
                InstructionType::kOp10Type => {
                    self.decode_typek_op10(&instr);
                }
                InstructionType::kOp12Type => {
                    self.decode_typek_op12(&instr);
                }
                InstructionType::kOp14Type => {
                    self.decode_typek_op14(&instr);
                }
                InstructionType::kOp17Type => {
                    return self.decode_typek_op17(&instr);
                }
                InstructionType::kOp22Type => {
                    self.decode_typek_op22(&instr);
                }
                InstructionType::kUnsupported => {
                    self.format(&instr, "UNSUPPORTED");
                }
                _ => {
                    self.format(&instr, "UNSUPPORTED");
                }
            }
            K_INSTR_SIZE as i32
        }

        /// Bottleneck functions to print into the out_buffer.
        fn print_char(&mut self, ch: char) {
            if self.out_buffer_pos < (self.out_buffer.capacity() - 1) {
                if self.out_buffer_pos >= self.out_buffer.len() {
                    self.out_buffer.push(ch);
                } else {
                    self.out_buffer[self.out_buffer_pos] = ch;
                }
                self.out_buffer_pos += 1;
                if self.out_buffer_pos == self.out_buffer.len() {
                    self.out_buffer.push('\0');
                } else {
                    self.out_buffer[self.out_buffer_pos] = '\0';
                }
            }
        }

        /// Append the str to the output buffer.
        fn print(&mut self, s: &str) {
            for &cur in s.as_bytes() {
                if self.out_buffer_pos < (self.out_buffer.capacity() - 1) {
                    if self.out_buffer_pos >= self.out_buffer.len() {
                        self.out_buffer.push(cur as char);
                    } else {
                        self.out_buffer[self.out_buffer_pos] = cur as char;
                    }
                    self.out_buffer_pos += 1;
                    if self.out_buffer_pos == self.out_buffer.len() {
                        self.out_buffer.push('\0');
                    } else {
                        self.out_buffer[self.out_buffer_pos] = '\0';
                    }
                }
            }
        }

        /// Print the register name according to the active name converter.
        fn print_register(&mut self, reg: i32) {
            self.print(self.converter.name_of_cpu_register(reg));
        }

        fn print_rj(&mut self, instr: &Instruction) {
            let reg = instr.rj_value();
            self.print_register(reg);
        }

        fn print_rk(&mut self, instr: &Instruction) {
            let reg = instr.rk_value();
            self.print_register(reg);
        }

        fn print_rd(&mut self, instr: &Instruction) {
            let reg = instr.rd_value();
            self.print_register(reg);
        }

        /// Print the FPUregister name according to the active name converter.
        fn print_fpu_register(&mut self, freg: i32) {
            self.print(self.converter.name_of_xmm_register(freg));
        }

        fn print_fj(&mut self, instr: &Instruction) {
            let freg = instr.fj_value();
            self.print_fpu_register(freg);
        }

        fn print_fk(&mut self, instr: &Instruction) {
            let freg = instr.fk_value();
            self.print_fpu_register(freg);
        }

        fn print_fd(&mut self, instr: &Instruction) {
            let freg = instr.fd_value();
            self.print_fpu_register(freg);
        }

        fn print_fa(&mut self, instr: &Instruction) {
            let freg = instr.fa_value();
            self.print_fpu_register(freg);
        }

        /// Print the integer value of the sa field.
        fn print_sa2(&mut self, instr: &Instruction) {
            let mut sa = instr.sa2_value();
            let opcode = (instr.instruction_bits() >> 18) << 18;
            if opcode == ALSL || opcode == ALSL_D {
                sa += 1;
            }
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", sa)) };
        }

        fn print_sa3(&mut self, instr: &Instruction) {
            let sa = instr.sa3_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", sa)) };
        }

        fn print_ui5(&mut self, instr: &Instruction) {
            let ui = instr.ui5_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", ui)) };
        }

        fn print_ui6(&mut self, instr: &Instruction) {
            let ui = instr.ui6_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", ui)) };
        }

        fn print_ui12(&mut self, instr: &Instruction) {
            let ui = instr.ui12_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", ui)) };
        }

        fn print_xi12(&mut self, instr: &Instruction) {
            let xi = instr.ui12_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}", xi)) };
        }

        fn print_xi20(&mut self, instr: &Instruction) {
            let xi = instr.si20_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}", xi)) };
        }

        fn print_msbd(&mut self, instr: &Instruction) {
            let msbd = instr.msbd_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", msbd)) };
        }

        fn print_lsbd(&mut self, instr: &Instruction) {
            let lsbd = instr.lsbd_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", lsbd)) };
        }

        fn print_msbw(&mut self, instr: &Instruction) {
            let msbw = instr.msbw_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", msbw)) };
        }

        fn print_lsbw(&mut self, instr: &Instruction) {
            let lsbw = instr.lsbw_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", lsbw)) };
        }

        fn print_si12(&mut self, instr: &Instruction) {
            const K_SI12_BITS: i32 = 12;
            let si = ((instr.si12_value() as i32) << (32 - K_SI12_BITS)) >> (32 - K_SI12_BITS);
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}(0x{:x})", si, instr.si12_value())) };
        }

        fn print_si14(&mut self, instr: &Instruction) {
            const K_SI14_BITS: i32 = 14;
            let mut si = ((instr.si14_value() as i32) << (32 - K_SI14_BITS)) >> (32 - K_SI14_BITS);
            si <<= 2;
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}(0x{:x})", si, instr.si14_value() << 2) };
        }

        fn print_si16(&mut self, instr: &Instruction) {
            const K_SI16_BITS: i32 = 16;
            let si = ((instr.si16_value() as i32) << (32 - K_SI16_BITS)) >> (32 - K_SI16_BITS);
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}(0x{:x})", si, instr.si16_value()) };
        }

        fn print_si20(&mut self, instr: &Instruction) {
            const K_SI20_BITS: i32 = 20;
            let si = ((instr.si20_value() as i32) << (32 - K_SI20_BITS)) >> (32 - K_SI20_BITS);
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}(0x{:x})", si, instr.si20_value()) };
        }

        fn print_cj(&mut self, instr: &Instruction) {
            let cj = instr.cj_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", cj)) };
        }

        fn print_cd(&mut self, instr: &Instruction) {
            let cd = instr.cd_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", cd)) };
        }

        fn print_ca(&mut self, instr: &Instruction) {
            let ca = instr.ca_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", ca)) };
        }

        fn print_code(&mut self, instr: &Instruction) {
            let code = instr.code_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}({})", code, code)) };
        }

        fn print_hint5(&mut self, instr: &Instruction) {
            let hint = instr.hint5_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}({})", hint, hint)) };
        }

        fn print_hint15(&mut self, instr: &Instruction) {
            let hint = instr.hint15_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}({})", hint, hint)) };
        }

        fn print_pc_offs16(&mut self, instr: &Instruction) {
            const K_OFFS_LOW_BITS: i32 = 2;
            let n_bits = 2;
            let offs = instr.offs16_value();
            let target = ((offs << n_bits) << (32 - K_OFFS_LOW_BITS - n_bits)) >> (32 - K_OFFS_LOW_BITS - n_bits);
            let target_addr = unsafe { (instr as *const Instruction as *const u8).add(target as usize) };
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", self.converter.name_of_address(target_addr as *mut u8))) };
        }

        fn print_pc_offs21(&mut self, instr: &Instruction) {
            const K_OFFS_LOW_BITS: i32 = 2;
            const K_OFFS21_HIGH_BITS: i32 = 1;
            let n_bits = 2;
            let offs = instr.offs21_value();
            let target = ((offs << n_bits) << (32 - K_OFFS_LOW_BITS - K_OFFS21_HIGH_BITS - n_bits)) >> (32 - K_OFFS_LOW_BITS - K_OFFS21_HIGH_BITS - n_bits);

            let target_addr = unsafe { (instr as *const Instruction as *const u8).add(target as usize) };
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", self.converter.name_of_address(target_addr as *mut u8))) };
        }

        fn print_pc_offs26(&mut self, instr: &Instruction) {
            const K_OFFS_LOW_BITS: i32 = 2;
            const K_OFFS26_HIGH_BITS: i32 = 1;
            let n_bits = 2;
            let offs = instr.offs26_value();
            let target = ((offs << n_bits) << (32 - K_OFFS_LOW_BITS - K_OFFS26_HIGH_BITS - n_bits)) >> (32 - K_OFFS_LOW_BITS - K_OFFS26_HIGH_BITS - n_bits);

            let target_addr = unsafe { (instr as *const Instruction as *const u8).add(target as usize) };
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("{}", self.converter.name_of_address(target_addr as *mut u8))) };
        }

        fn print_offs16(&mut self, instr: &Instruction) {
            let offs = instr.offs16_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}", offs << 2)) };
        }

        fn print_offs21(&mut self, instr: &Instruction) {
            let offs = instr.offs21_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}", offs << 2)) };
        }

        fn print_offs26(&mut self, instr: &Instruction) {
            let offs = instr.offs26_value();
            self.out_buffer_pos += unsafe { snprintf(&mut self.out_buffer, format!("0x{:x}", offs << 2)) };
        }

        /// Handle all register based formatting in this function to reduce the
        /// complexity of FormatOption.
        fn format_register(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'r');
            if format.chars().nth(1).unwrap() == 'j' {
                // 'rj: Rj register.
                let reg = instr.rj_value();
                self.print_register(reg);
                2
            } else if format.chars().nth(1).unwrap() == 'k' {
                // 'rk: rk register.
                let reg = instr.rk_value();
                self.print_register(reg);
                2
            } else if format.chars().nth(1).unwrap() == 'd' {
                // 'rd: rd register.
                let reg = instr.rd_value();
                self.print_register(reg);
                2
            } else {
                unreachable!()
            }
        }

        /// Handle all FPUregister based formatting in this function to reduce the
        /// complexity of FormatOption.
        fn format_fpu_register(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'f');
            if format.chars().nth(1).unwrap() == 'j' {
                // 'fj: fj register.
                let reg = instr.fj_value();
                self.print_fpu_register(reg);
                2
            } else if format.chars().nth(1).unwrap() == 'k' {
                // 'fk: fk register.
                let reg = instr.fk_value();
                self.print_fpu_register(reg);
                2
            } else if format.chars().nth(1).unwrap() == 'd' {
                // 'fd: fd register.
                let reg = instr.fd_value();
                self.print_fpu_register(reg);
                2
            } else if format.chars().nth(1).unwrap() == 'a' {
                // 'fa: fa register.
                let reg = instr.fa_value();
                self.print_fpu_register(reg);
                2
            } else {
                unreachable!()
            }
        }

        /// FormatOption takes a formatting string and interprets it based on
        /// the current instructions. The format string points to the first
        /// character of the option string (the option escape has already been
        /// consumed by the caller.)  FormatOption returns the number of
        /// characters that were consumed from the formatting string.
        fn format_option(&mut self, instr: &Instruction, format: &str) -> i32 {
            match format.chars().next().unwrap() {
                'c' => {
                    match format.chars().nth(1).unwrap() {
                        'a' => {
                            assert!(string_starts_with!(format, "ca"));
                            self.print_ca(instr);
                            2
                        }
                        'd' => {
                            assert!(string_starts_with!(format, "cd"));
                            self.print_cd(instr);
                            2
                        }
                        'j' => {
                            assert!(string_starts_with!(format, "cj"));
                            self.print_cj(instr);
                            2
                        }
                        'o' => {
                            assert!(string_starts_with!(format, "code"));
                            self.print_code(instr);
                            4
                        }
                        _ => 0,
                    }
                }
                'f' => self.format_fpu_register(instr, format),
                'h' => {
                    if format.chars().nth(4).unwrap() == '5' {
                        assert!(string_starts_with!(format, "hint5"));
                        self.print_hint5(instr);
                        5
                    } else if format.chars().nth(4).unwrap() == '1' {
                        assert!(string_starts_with!(format, "hint15"));
                        self.print_hint15(instr);
                        6
                    } else {
                        0
                    }
                }
                'l' => {
                    match format.chars().nth(3).unwrap() {
                        'w' => {
                            assert!(string_starts_with!(format, "lsbw"));
                            self.print_lsbw(instr);
                            4
                        }
                        'd' => {
                            assert!(string_starts_with!(format, "lsbd"));
                            self.print_lsbd(instr);
                            4
                        }
                        _ => 0,
                    }
                }
                'm' => {
                    if format.chars().nth(3).unwrap() == 'w' {
                        assert!(string_starts_with!(format, "msbw"));
                        self.print_msbw(instr);
                    } else if format.chars().nth(3).unwrap() == 'd' {
                        assert!(string_starts_with!(format, "msbd"));
                        self.print_msbd(instr);
                    }
                    4
                }
                'o' => {
                    if format.chars().nth(1).unwrap() == 'f' {
                        if format.chars().nth(4).unwrap() == '1' {
                            assert!(string_starts_with!(format, "offs16"));
                            self.print_offs16(instr);
                            6
                        } else if format.chars().nth(4).unwrap() == '2' {
                            if format.chars().nth(5).unwrap() == '1' {
                                assert!(string_starts_with!(format, "offs21"));
                                self.print_offs21(instr);
                                6
                            } else if format.chars().nth(5).unwrap() == '6' {
                                assert!(string_starts_with!(format, "offs26"));
                                self.print_offs26(instr);
                                6
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                'p' => {
                    if format.chars().nth(6).unwrap() == '1' {
                        assert!(string_starts_with!(format, "pcoffs16"));
                        self.print_pc_offs16(instr);
                        8
                    } else if format.chars().nth(6).unwrap() == '2' {
                        if format.chars().nth(7).unwrap() == '1' {
                            assert!(string_starts_with!(format, "pcoffs21"));
                            self.print_pc_offs21(instr);
                            8
                        } else if format.chars().nth(7).unwrap() == '6' {
                            assert!(string_starts_with!(format, "pcoffs26"));
                            self.print_pc_offs26(instr);
                            8
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                'r' => self.format_register(instr, format),
                's' => {
                    match format.chars().nth(1).unwrap() {
                        'a' => {
                            if format.chars().nth(2).unwrap() == '2' {
                                assert!(string_starts_with!(format, "sa2"));
                                self.print_sa2(instr);
                            } else if format.chars().nth(2).unwrap() == '3' {
                                assert!(string_starts_with!(format, "sa3"));
                                self.print_sa3(instr);
                            }
                            3
                        }
                        'i' => {
                            if format.chars().nth(2).unwrap() == '2' {
                                assert!(string_starts_with!(format, "si20"));
                                self.print_si20(instr);
                                4
                            } else if format.chars().nth(2).unwrap() == '1' {
                                match format.chars().nth(3).unwrap() {
                                    '2' => {
                                        assert!(string_starts_with!(format, "si12"));
                                        self.print_si12(instr);
                                        4
                                    }
                                    '4' => {
                                        assert!(string_starts_with!(format, "si14"));
                                        self.print_si14(instr);
                                        4
                                    }
                                    '6' => {
                                        assert!(string_starts_with!(format, "si16"));
                                        self.print_si16(instr);
                                        4
                                    }
                                    _ => 0,
                                }
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    }
                }
                'u' => {
                    if format.chars().nth(2).unwrap() == '5' {
                        assert!(string_starts_with!(format, "ui5"));
                        self.print_ui5(instr);
                        3
                    } else if format.chars().nth(2).unwrap() == '6' {
                        assert!(string_starts_with!(format, "ui6"));
                        self.print_ui6(instr);
                        3
                    } else if format.chars().nth(2).unwrap() == '1' {
                        assert!(string_starts_with!(format, "ui12"));
                        self.print_ui12(instr);
                        4
                    } else {
                        0
                    }
                }
                'x' => {
                    if format.chars().nth(2).unwrap() == '2' {
                        assert!(string_starts_with!(format, "xi20"));
                        self.print_xi20(instr);
                        4
                    } else if format.chars().nth(3).unwrap() == '2' {
                        assert!(string_starts_with!(format, "xi12"));
                        self.print_xi12(instr);
                        4
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }

        /// Format takes a formatting string for a whole instruction and prints it into
        /// the output buffer. All escaped options are handed to FormatOption to be
        /// parsed further.
        fn format(&mut self, instr: &Instruction, format: &str) {
            let mut iter = format.chars();
            while let Some(cur) = iter.next() {
                if self.out_buffer_pos < (self.out_buffer.capacity() - 1) {
                    if cur == '\'' {
                        // Single quote is used as the formatting escape.
                        let remaining_format: String = iter.clone().collect();
                        let consumed = self.format_option(instr, &remaining_format);
                        for _ in 0..consumed {
                            iter.next();
                        }
                    } else {
                        self.print_char(cur);
                    }
                } else {
                    break;
                }
            }
        }

        /// For currently unimplemented decodings the disassembler calls Unknown(instr)
        /// which will just print "unknown" of the instruction bits.
        fn unknown(&mut self, instr: &Instruction) {
            self.format(instr, "unknown");
        }

        fn decode_break_instr(&mut self, instr: &Instruction) -> i32 {
            // This is already known to be BREAK instr, just extract the code.
            self.format(instr, "break        code: 'code");
            K_INSTR_SIZE as i32
        }

        fn decode_typek_op6(&mut self, instr: &Instruction) {
            match instr.bits(31, 26) << 26 {
                ADDU16I_D => self.format(instr, "addu16i.d    'rd, 'rj, 'si16"),
                BEQZ => self.format(instr, "beqz         'rj, 'offs21 -> 'pcoffs21"),
                BNEZ => self.format(instr, "bnez         'rj, 'offs21 -> 'pcoffs21"),
                BCZ => {
                    if instr.bit(8) {
                        self.format(instr, "bcnez        fcc'cj, 'offs21 -> 'pcoffs21")
                    } else {
                        self.format(instr, "bceqz        fcc'cj, 'offs21 -> 'pcoffs21")
                    }
                }
                JIRL => self.format(instr, "jirl         'rd, 'rj, 'offs16"),
                B => self.format(instr, "b            'offs26 -> 'pcoffs26"),
                BL => self.format(instr, "bl           'offs26 -> 'pcoffs26"),
                BEQ => self.format(instr, "beq          'rj, 'rd, 'offs16 -> 'pcoffs16"),
                BNE => self.format(instr, "bne          'rj, 'rd, 'offs16 -> 'pcoffs16"),
                BLT => self.format(instr, "blt          'rj, 'rd, 'offs16 -> 'pcoffs16"),
                BGE => self.format(instr, "bge          'rj, 'rd, 'offs16 -> 'pcoffs16"),
                BLTU => self.format(instr, "bltu         'rj, 'rd, 'offs16 -> 'pcoffs16"),
                BGEU => self.format(instr, "bgeu         'rj, 'rd, 'offs16 -> 'pcoffs16"),
                _ => unreachable!(),
            }
        }

        fn decode_typek_op7(&mut self, instr: &Instruction) {
            match instr.bits(31, 25) << 25 {
                LU12I_W => self.format(instr, "lu12i.w      'rd, 'xi20"),
                LU32I_D => self.format(instr, "lu32i.d      'rd, 'xi20"),
                PCADDI => self.format(instr, "pcaddi       'rd, 'xi20"),
                PCALAU12I => self.format(instr, "pcalau12i    'rd, 'xi20"),
                PCADDU12I => self.format(instr, "pcaddu12i    'rd, 'xi20"),
                PCADDU18I => self.format(instr, "pcaddu18i    'rd, 'xi20"),
                _ => unreachable!(),
            }
        }

        fn decode_typek_op8(&mut self, instr: &Instruction) {
            match instr.bits(31, 24) << 24 {
                LDPTR_W => self.format(instr, "ldptr.w      'rd, 'rj, 'si14"),
                STPTR_W => self.format(instr, "stptr.w      'rd, 'rj, 'si14"),
                LDPTR_D => self.format(instr, "ldptr.d      'rd, 'rj, 'si14"),
                STPTR_D => self.format(instr, "stptr.d      'rd, 'rj, 'si14"),
                LL_W => self.format(instr, "ll.w         'rd, 'rj, 'si14"),
                SC_W => self.format(instr, "sc.w         'rd, 'rj, 'si14"),
                LL_D => self.format(instr, "ll.d         'rd, 'rj, 'si14"),
                SC_D => self.format(instr, "sc.d         'rd, 'rj, 'si14"),
                _ => unreachable!(),
            }
        }

        fn decode_typek_op10(&mut self, instr: &Instruction) {
            match instr.bits(31, 22) << 22 {
                BSTR_W => {
                    if instr.bit(21) != 0 {
                        if instr.bit(15) == 0 {
                            self.format(instr, "bstrins.w    'rd, 'rj, 'msbw, 'lsbw");
                        } else {
                            self.format(instr, "bstrpick.w   'rd, 'rj, 'msbw, 'lsbw");
                        }
                    }
                }
                BSTRINS_D => self.format(instr, "bstrins.d    'rd, 'rj, 'msbd, 'lsbd"),
                BSTRPICK_D => self.format(instr, "bstrpick.d   'rd, 'rj, 'msbd, 'lsbd"),
                SLTI => self.format(instr, "slti         'rd, 'rj, 'si12"),
                SLTUI => self.format(instr, "sltui        'rd, 'rj, 'si12"),
                ADDI_W => self.format(instr, "addi.w       'rd, 'rj, 'si12"),
                ADDI_D => self.format(instr, "addi.d       'rd, 'rj, 'si1
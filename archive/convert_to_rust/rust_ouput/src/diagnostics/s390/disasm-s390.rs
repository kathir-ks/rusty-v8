// Converted from V8 C++ source files:
// Header: N/A
// Implementation: disasm-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod disasm {
    use std::fmt;
    use std::string::String;

    pub struct NameConverter {
        tmp_buffer_: base::EmbeddedVector<char, 256>,
    }

    impl NameConverter {
        pub fn new() -> Self {
            NameConverter {
                tmp_buffer_: base::EmbeddedVector::new(),
            }
        }

        pub fn NameOfAddress(&self, addr: *mut u8) -> *const char {
            let addr_str = format!("{:p}", addr);
            self.tmp_buffer_.clear();
            for c in addr_str.chars() {
                self.tmp_buffer_.push(c as char);
            }
            self.tmp_buffer_.push('\0');
            self.tmp_buffer_.begin() as *const char
        }

        pub fn NameOfConstant(&self, addr: *mut u8) -> *const char {
            self.NameOfAddress(addr)
        }

        pub fn NameOfCPURegister(&self, reg: i32) -> *const char {
            let reg_name = RegisterName(i::Register::from_code(reg));
            reg_name.as_ptr() as *const char
        }

        pub fn NameOfByteCPURegister(&self, _reg: i32) -> *const char {
            unreachable!("S390 does not have the concept of a byte register")
        }

        pub fn NameOfXMMRegister(&self, _reg: i32) -> *const char {
            unreachable!("S390 does not have XMM register. Consider update this for Vector Regs");
        }

        pub fn NameInCode(&self, _addr: *mut u8) -> *const char {
            "" as *const char
        }
    }

    pub enum UnimplementedOpcodeAction {
        // Define actions if needed, for now just a placeholder
        DoNothing,
    }

    pub struct Disassembler {
        converter_: NameConverter,
        unimplemented_action_: UnimplementedOpcodeAction,
    }

    impl Disassembler {
        pub fn new(converter: NameConverter, unimplemented_action: UnimplementedOpcodeAction) -> Self {
            Disassembler {
                converter_: converter,
                unimplemented_action_: unimplemented_action,
            }
        }

        pub fn InstructionDecode(&self, buffer: base::Vector<char>, instruction: *mut u8) -> i32 {
            let mut decoder = v8::internal::Decoder::new(&self.converter_, buffer);
            decoder.InstructionDecode(instruction)
        }

        pub fn ConstantPoolSizeAt(&self, _instruction: *mut u8) -> i32 {
            -1
        }

        pub fn Disassemble(&self, f: *mut std::ffi::c_void, begin: *mut u8, end: *mut u8) {
            let mut pc = begin;
            while pc < end {
                let mut buffer: base::EmbeddedVector<char, 128> = base::EmbeddedVector::new();
                buffer.push('\0');
                let prev_pc = pc;
                let instruction_length = self.InstructionDecode(
                    base::Vector {
                        start: buffer.begin(),
                        length: 128,
                        owned: false,
                    },
                    pc,
                );
                pc = unsafe { pc.offset(instruction_length as isize) };

                let prev_pc_addr = prev_pc as usize;
                let instruction_bits = unsafe { *(prev_pc as *mut i32) }; // Assuming 32-bit instruction

                let buffer_string = unsafe { std::ffi::CStr::from_ptr(buffer.begin()).to_string_lossy() };
                println!("{:p}    {:08x}      {}", prev_pc_addr as *mut std::ffi::c_void, instruction_bits, buffer_string);
            }
        }
    }
}

pub mod internal {
    use super::disasm;
    use crate::base;
    use std::string::String;
    use std::vec::Vec;
    use std::fmt;

    pub struct Decoder<'a> {
        converter_: &'a disasm::NameConverter,
        out_buffer_: base::Vector<char>,
        out_buffer_pos_: i32,
    }

    impl<'a> Decoder<'a> {
        pub fn new(converter: &'a disasm::NameConverter, out_buffer: base::Vector<char>) -> Self {
            let mut decoder = Decoder {
                converter_: converter,
                out_buffer_: out_buffer,
                out_buffer_pos_: 0,
            };
            decoder.out_buffer_[0] = '\0';
            decoder
        }

        pub fn InstructionDecode(&mut self, instruction: *mut u8) -> i32 {
            let instr = Instruction::At(instruction);
            let instrLength = instr.InstructionLength();

            if instrLength == 2 {
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%04x           ",
                    instr.InstructionBits::<TwoByteInstr>(),
                );
            } else if (instrLength == 4) {
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%08x       ",
                    instr.InstructionBits::<FourByteInstr>(),
                );
            } else {
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%012x   ",
                    instr.InstructionBits::<SixByteInstr>(),
                );
            }

            let mut decoded = self.DecodeSpecial(&instr);
            if !decoded {
                decoded = self.DecodeGeneric(&instr);
            }
            if !decoded {
                self.Unknown(&instr);
            }
            return instrLength;
        }

        fn PrintChar(&mut self, ch: char) {
            self.out_buffer_[(self.out_buffer_pos_ as usize)] = ch;
            self.out_buffer_pos_ += 1;
        }

        fn Print(&mut self, str: &str) {
            for cur in str.chars() {
                if self.out_buffer_pos_ < (self.out_buffer_.length - 1) as i32 {
                    self.PrintChar(cur);
                } else {
                    break;
                }
            }
            self.out_buffer_[(self.out_buffer_pos_ as usize)] = '\0';
        }

        fn PrintRegister(&mut self, reg: i32) {
            let name = unsafe { std::ffi::CStr::from_ptr(self.converter_.NameOfCPURegister(reg)).to_string_lossy() };
            self.Print(&name);
        }

        fn PrintDRegister(&mut self, reg: i32) {
            let name = RegisterName(DoubleRegister::from_code(reg));
            self.Print(&name);
        }

        fn PrintSoftwareInterrupt(&mut self, svc: SoftwareInterruptCodes) {
            match svc {
                SoftwareInterruptCodes::kCallRtRedirected => self.Print("call rt redirected"),
                SoftwareInterruptCodes::kBreakpoint => self.Print("breakpoint"),
                _ => {
                    if svc >= SoftwareInterruptCodes::kStopCode {
                        self.out_buffer_pos_ += base::SNPrintF(
                           self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                            "%d - 0x%x",
                            (svc as i32) & (SoftwareInterruptCodes::kStopCode as i32) & 0xFF,
                            (svc as i32) & (SoftwareInterruptCodes::kStopCode as i32) & 0xFF,
                        );
                    } else {
                        self.out_buffer_pos_ += base::SNPrintF(
                           self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                            "%d",
                            svc as i32,
                        );
                    }
                }
            }
        }

        fn FormatRegister(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'r');

            if format.chars().nth(1) == Some('1') {
                let reg = instr.Bits::<SixByteInstr, i32>(39, 36);
                self.PrintRegister(reg);
                2
            } else if format.chars().nth(1) == Some('2') {
                let reg = instr.Bits::<SixByteInstr, i32>(35, 32);

                if format.chars().nth(2) == Some('d') {
                    if reg == 0 {
                        return 4;
                    }
                    self.PrintRegister(reg);
                    return 3;
                } else {
                    self.PrintRegister(reg);
                    return 2;
                }
            } else if format.chars().nth(1) == Some('3') {
                let reg = instr.Bits::<SixByteInstr, i32>(31, 28);
                self.PrintRegister(reg);
                2
            } else if format.chars().nth(1) == Some('4') {
                let reg = instr.Bits::<SixByteInstr, i32>(27, 24);
                self.PrintRegister(reg);
                2
            } else if format.chars().nth(1) == Some('5') {
                let reg = instr.Bits::<SixByteInstr, i32>(23, 20);
                self.PrintRegister(reg);
                2
            } else if format.chars().nth(1) == Some('6') {
                let reg = instr.Bits::<SixByteInstr, i32>(19, 16);
                self.PrintRegister(reg);
                2
            } else if format.chars().nth(1) == Some('7') {
                let reg = instr.Bits::<SixByteInstr, i32>(15, 12);
                self.PrintRegister(reg);
                2
            } else {
                unreachable!();
            }
        }

        fn FormatFloatingRegister(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'f');

            if format.chars().nth(1) == Some('1') {
                let rrinstr = instr.as_rr_instruction();
                let reg = rrinstr.R1Value();
                self.PrintDRegister(reg);
                2
            } else if format.chars().nth(1) == Some('2') {
                let rrinstr = instr.as_rr_instruction();
                let reg = rrinstr.R2Value();
                self.PrintDRegister(reg);
                2
            } else if format.chars().nth(1) == Some('3') {
                let rrdinstr = instr.as_rrd_instruction();
                let reg = rrdinstr.R1Value();
                self.PrintDRegister(reg);
                2
            } else if format.chars().nth(1) == Some('5') {
                let rreinstr = instr.as_rre_instruction();
                let reg = rreinstr.R1Value();
                self.PrintDRegister(reg);
                2
            } else if format.chars().nth(1) == Some('6') {
                let rreinstr = instr.as_rre_instruction();
                let reg = rreinstr.R2Value();
                self.PrintDRegister(reg);
                2
            } else if format.chars().nth(1) == Some('4') {
                let vrreinstr = instr.as_vrr_e_instruction();
                let reg = vrreinstr.R4Value();
                self.PrintDRegister(reg);
                2
            } else {
                unreachable!();
            }
        }

        fn FormatMask(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'm');
            let mut value: i32 = 0;

            if format.chars().nth(1) == Some('1') {
                value = instr.as_rr_instruction().R1Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('2') {
                value = instr.as_rx_instruction().B2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('3') {
                value = instr.as_rrf_instruction().M4Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('4') {
                value = instr.as_vrr_c_instruction().M4Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('5') {
                value = instr.as_vrr_c_instruction().M5Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('6') {
                value = instr.as_vrr_c_instruction().M6Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "0x%x",
                    value,
                );
                2
            } else {
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            }
        }

        fn FormatDisplacement(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'd');

            if format.chars().nth(1) == Some('1') {
                let rsinstr = instr.as_rs_instruction();
                let value = rsinstr.D2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('2') {
                let rxyinstr = instr.as_rxy_instruction();
                let value = rxyinstr.D2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('4') {
                let ssInstr = instr.as_ss_instruction();
                let value = ssInstr.D2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('3') {
                let ssInstr = instr.as_ss_instruction();
                let value = ssInstr.D1Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else {
                let value = SIGN_EXT_IMM16(instr.Bits::<SixByteInstr, i32>(15, 0) & !3);
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                1
            }
        }

        fn FormatImmediate(&mut self, instr: &Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'i');

            if format.chars().nth(1) == Some('1') {
                let riinstr = instr.as_ri_instruction();
                let value = riinstr.I2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('2') {
                let rilinstr = instr.as_ril_instruction();
                let value = rilinstr.I2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('3') {
                let iinstr = instr.as_i_instruction();
                let value = iinstr.IValue();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('4') {
                let riinstr = instr.as_ri_instruction();
                let value = riinstr.I2Value() * 2;
                if value >= 0 {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*+",
                    );
                } else {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*",
                    );
                }
                let address_name = unsafe {std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(instruction.add(value) as *mut u8)).to_string_lossy()};
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d -> %s",
                    value,
                    address_name.as_ptr() as *const i8,
                );
                2
            } else if format.chars().nth(1) == Some('5') {
                let rilinstr = instr.as_ril_instruction();
                let value = rilinstr.I2Value() * 2;
                if value >= 0 {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*+",
                    );
                } else {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*",
                    );
                }
                let address_name = unsafe {std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(instruction.add(value) as *mut u8)).to_string_lossy()};
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d -> %s",
                    value,
                   address_name.as_ptr() as *const i8,
                );
                2
            } else if format.chars().nth(1) == Some('6') {
                let riinstr = instr.as_ri_instruction();
                let value = riinstr.I2UnsignedValue();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('7') {
                let rilinstr = instr.as_ril_instruction();
                let value = rilinstr.I2UnsignedValue();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('8') {
                let ssinstr = instr.as_ss_instruction();
                let value = ssinstr.Length();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('9') {
                let rie_instr = instr.as_rie_instruction();
                let value = rie_instr.I3Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('a') {
                let rie_instr = instr.as_rie_instruction();
                let value = rie_instr.I4Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('b') {
                let rie_instr = instr.as_rie_instruction();
                let value = rie_instr.I5Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('c') {
                let ssinstr = instr.as_ss_instruction();
                let value = ssinstr.Length();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('d') {
                let silinstr = instr.as_sil_instruction();
                let value = silinstr.I2Value();
                self.out_buffer_pos_ += base::SNPrintF(
                   self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d",
                    value,
                );
                2
            } else if format.chars().nth(1) == Some('e') {
                let rilinstr = instr.as_ril_instruction();
                let value = rilinstr.I2Value() * 2;
                if value >= 0 {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*+",
                    );
                } else {
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "*",
                    );
                }
                let address_name = unsafe {std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(instruction.add(value) as *mut u8)).to_string_lossy()};
                 self.out_buffer_pos_ += base::SNPrintF(
                    self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                    "%d -> %s",
                    value,
                    address_name.as_ptr() as *const i8,
                );
                2
            } else {
                unreachable!();
            }
        }

        fn FormatOption(&mut self, instr: &Instruction, format: &str) -> i32 {
            match format.chars().next().unwrap() {
                'o' => {
                    if instr.Bit(10) == 1 {
                        self.Print("o");
                    }
                    1
                }
                '.' => {
                    if instr.Bit(0) == 1 {
                        self.Print(".");
                    } else {
                        self.Print(" ");
                    }
                    1
                }
                'r' => self.FormatRegister(instr, format),
                'f' => self.FormatFloatingRegister(instr, format),
                'i' => self.FormatImmediate(instr, format),
                'u' => {
                    let value = instr.Bits::<SixByteInstr, i32>(15, 0);
                     self.out_buffer_pos_ += base::SNPrintF(
                        self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                        "%d",
                        value,
                    );
                    6
                }
                'l' => {
                    if instr.Bit(0) == 1 {
                        self.Print("l");
                    }
                    1
                }
                'a' => {
                    if instr.Bit(1) == 1 {
                        self.Print("a");
                    }
                    1
                }
                't' => {
                    if format.starts_with("target") {
                        if format.chars().nth(6) == Some('2') && format.chars().nth(7) == Some('6') {
                            let off = ((instr.Bits::<SixByteInstr, i32>(25, 2)) << 8) >> 6;
                             let address_name = unsafe {std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(instruction.add(off) as *mut u8)).to_string_lossy()};
                             self.out_buffer_pos_ += base::SNPrintF(
                                self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                                "%+d -> %s",
                                off,
                                address_name.as_ptr() as *const i8,
                            );
                            8
                        } else if format.chars().nth(6) == Some('1') && format.chars().nth(7) == Some('6') {
                            let off = ((instr.Bits::<SixByteInstr, i32>(15, 2)) << 18) >> 16;
                             let address_name = unsafe {std::ffi::CStr::from_ptr(self.converter_.NameOfAddress(instruction.add(off) as *mut u8)).to_string_lossy()};
                             self.out_buffer_pos_ += base::SNPrintF(
                                self.out_buffer_.start.add(self.out_buffer_pos_ as usize),
                                "%+d -> %s",
                                off,
                                address_name.as_ptr() as *const i8,
                            );
                            8
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                }
                'm' => self.FormatMask(instr, format),
                'd' => self.FormatDisplacement(instr, format),
                _ => unreachable!(),
            }
        }

        fn Format(&mut self, instr: &Instruction, format: &str) {
            let mut chars = format.chars();
            while let Some(cur) = chars.next() {
                if self.out_buffer_pos_ >= (self.out_buffer_.length - 1) as i32 {
                    break;
                }
                if cur == '\'' {
                    let remaining_format = chars.as_str();
                    let consumed = self.FormatOption(instr, remaining_format);
                    for _ in 0..consumed {
                        chars.next();
                    }
                } else {
                    self.out_buffer_[(self.out_buffer_pos_ as usize)] = cur;
                    self.out_buffer_pos_ += 1;
                }
            }
            self.out_buffer_[(self.out_buffer_pos_ as usize)] = '\0';
        }

        fn Unknown(&mut self, instr: &Instruction) {
            self.Format(instr, "unknown");
        }

        fn UnknownFormat(&mut self, instr: &Instruction, name: &str) {
            let buffer = format!("{} (unknown-format)", name);
            self.Format(instr, &buffer);
        }

        fn DecodeSpecial(&mut self, instr: &Instruction) -> bool {
            let opcode = instr.S390OpcodeValue();
            match opcode {
                Opcode::BKPT => self.Format(instr, "bkpt"),
                Opcode::DUMY => self.Format(instr, "dumy\t'r1, 'd2 ( 'r2d, 'r3 )"),
                Opcode::LDR => self.Format(instr, "ldr\t'f1,'f2"),
                Opcode::BCR => self.Format(instr, "bcr\t'm1,'r2"),
                Opcode::OR => self.Format(instr, "or\t'r1,'r2"),
                Opcode::CR => self.Format(instr, "cr\t'r1,'r2"),
                Opcode::MR => self.Format(instr, "mr\t'r1,'r2"),
                Opcode::HER_Z => self.Format(instr, "her\t'r1,'r2"),
                Opcode::BRAS => self.Format(instr, "bras\t'r1,'i1"),
                Opcode::MDBR => self.Format(instr, "mdbr\t'f5,'f6"),
                Opcode::SDBR => self.Format(instr, "sdbr\t'f5,'f6"),
                Opcode::ADBR => self.Format(instr, "adbr\t'f5,'f6"),
                Opcode::CDBR => self.Format(instr, "cdbr\t'f5,'f6"),
                Opcode::MEEBR => self.Format(instr, "meebr\t'f5,'f6"),
                Opcode::SQDBR => self.Format(instr, "sqdbr\t'f5,'f6"),
                Opcode::SQEBR => self.Format(instr, "sqebr\t'f5,'f6"),
                Opcode::LCDBR => self.Format(instr, "lcdbr\t'f5,'f6"),
                Opcode::LCEBR => self.Format(instr, "lcebr\t'f5,'f6"),
                Opcode::LTEBR => self.Format(instr, "ltebr\t'f5,'f6"),
                Opcode::LDEBR => self.Format(instr, "ldebr\t'f5,'f6"),
                Opcode::CEBR => self.Format(instr, "cebr\t'f5,'f6"),
                Opcode::AEBR => self.Format(instr, "aebr\t'f5,'f6"),
                Opcode::SEBR => self.Format(instr, "sebr\t'f5,'f6"),
                Opcode::DEBR => self.Format(instr, "debr\t'f5,'f6"),
                Opcode::LTDBR => self.Format(instr, "ltdbr\t'f5,'f6"),
                Opcode::LDGR => self.Format(instr, "ldgr\t'f5,'f6"),
                Opcode::DDBR => self.Format(instr, "ddbr\t'f5,'f6"),
                Opcode::LZDR => self.Format(instr, "lzdr\t'f5"),
                Opcode::FIEBRA => self.Format(instr, "fiebra\t'f5,'m2,'f6,'m3"),
                Opcode::FIDBRA => self.Format(instr, "fidbra\t'f5,'m2,'f6,'m3"),
                Opcode::IC_z => self.Format(instr, "ic\t'r1,'d1('r2d,'r3)"),
                Opcode::AL => self.Format(instr, "al\t'r1,'d1('r2d,'r3)"),
                Opcode::LE => self.Format(instr, "le\t'f1,'d1('r2d,'r3)"),
                Opcode::LD => self.Format(instr, "ld\t'f1,'d1('r2d,'r3)"),


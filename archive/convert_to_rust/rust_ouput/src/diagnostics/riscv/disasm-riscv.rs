// Converted from V8 C++ source files:
// Header: N/A
// Implementation: disasm-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod disasm {
    use std::fmt;
    use std::io::Write;

    pub struct NameConverter {
        tmp_buffer_: [u8; 256],
    }

    impl NameConverter {
        pub fn new() -> Self {
            NameConverter {
                tmp_buffer_: [0; 256],
            }
        }

        pub fn NameOfAddress(&self, addr: *mut u8) -> *const i8 {
            unsafe {
                let formatted = format!("{:p}", addr);
                let bytes = formatted.as_bytes();

                let mut i = 0;
                for &byte in bytes {
                    if i < self.tmp_buffer_.len() - 1 {
                        self.tmp_buffer_[i] = byte;
                        i += 1;
                    } else {
                        break;
                    }
                }
                self.tmp_buffer_[i] = 0;

                self.tmp_buffer_.as_ptr() as *const i8
            }
        }

        pub fn NameOfConstant(&self, addr: *mut u8) -> *const i8 {
            self.NameOfAddress(addr)
        }

        pub fn NameOfCPURegister(&self, reg: i32) -> *const i8 {
            unsafe {
                crate::internal::Registers::Name(reg).as_ptr() as *const i8
            }
        }

        pub fn NameOfXMMRegister(&self, reg: i32) -> *const i8 {
            unsafe {
                crate::internal::FPURegisters::Name(reg).as_ptr() as *const i8
            }
        }

        pub fn NameOfByteCPURegister(&self, _reg: i32) -> *const i8 {
            panic!("RISC-V does not have the concept of a byte register.");
        }

        pub fn NameInCode(&self, _addr: *mut u8) -> *const i8 {
            "".as_ptr() as *const i8
        }
    }

    pub enum UnimplementedOpcodeAction {
        Print,
        Abort,
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

        pub fn InstructionDecode(&self, buffer: &mut Vec<u8>, instruction: *mut u8) -> i32 {
            unsafe {
                let buffer_vector = v8::base::Vector {
                    start_: buffer.as_mut_ptr() as *mut i8,
                    length_: buffer.capacity(),
                };
                let decoder = crate::internal::Decoder::new(&self.converter_, buffer_vector);
                let result = decoder.InstructionDecode(instruction);
                result
            }
        }

        pub fn ConstantPoolSizeAt(instruction: *mut u8) -> i32 {
            crate::internal::Assembler::ConstantPoolSizeAt(instruction as *mut crate::internal::Instruction)
        }

        pub fn Disassemble(
            f: &mut dyn Write,
            begin: *mut u8,
            end: *mut u8,
            unimplemented_action: UnimplementedOpcodeAction,
        ) {
            let converter = NameConverter::new();
            let d = Disassembler::new(converter, unimplemented_action);

            let mut pc = begin;
            while pc < end {
                unsafe {
                    let mut buffer: Vec<u8> = Vec::with_capacity(128);
                    buffer.push(0);
                    let prev_pc = pc;
                    let instruction_size = d.InstructionDecode(&mut buffer, pc);
                    pc = pc.add(instruction_size as usize);

                    let instruction_bits = *(prev_pc as *mut u32);
		    let buffer_str = String::from_utf8_lossy(&buffer);
                    writeln!(
                        f,
                        "{:p}    {:08x}      {}",
                        prev_pc, instruction_bits, buffer_str
                    )
                    .unwrap();
                }
            }
        }
    }
}

pub mod internal {
    use std::ffi::CString;

    pub struct Decoder<'a> {
        converter_: &'a disasm::NameConverter,
        out_buffer_: v8::base::Vector<i8>,
        out_buffer_pos_: i32,
    }

    impl<'a> Decoder<'a> {
        pub fn new(converter_: &'a disasm::NameConverter, out_buffer_: v8::base::Vector<i8>) -> Self {
            let mut decoder = Decoder {
                converter_: converter_,
                out_buffer_: out_buffer_,
                out_buffer_pos_: 0,
            };

            decoder.out_buffer_[decoder.out_buffer_pos_ as usize] = 0;

            decoder
        }

        fn PrintChar(&mut self, ch: char) {
            if self.out_buffer_pos_ < (self.out_buffer_.length() - 1) as i32 {
                self.out_buffer_[(self.out_buffer_pos_) as usize] = ch as i8;
                self.out_buffer_pos_ += 1;
            }
        }

        fn Print(&mut self, str: &str) {
            for ch in str.chars() {
                if self.out_buffer_pos_ < (self.out_buffer_.length() - 1) as i32 {
                    self.PrintChar(ch);
                } else {
                    break;
                }
            }
            self.out_buffer_[(self.out_buffer_pos_) as usize] = 0;
        }

        fn PrintRegister(&mut self, reg: i32) {
            unsafe {
                let name = self.converter_.NameOfCPURegister(reg);
                let c_str = CString::from_raw(name as *mut i8);
                self.Print(c_str.to_str().unwrap());
            }
        }
	fn PrintVRegister(&mut self, reg: i32) {
          self.Print(crate::VRegisters::Name(reg));
        }
        fn PrintFPURegister(&mut self, freg: i32) {
          unsafe {
            let name = self.converter_.NameOfXMMRegister(freg);
            let c_str = CString::from_raw(name as *mut i8);
            self.Print(c_str.to_str().unwrap());
          }
        }

        fn PrintRs1(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).Rs1Value();
                self.PrintRegister(reg);
            }
        }

        fn PrintRs2(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).Rs2Value();
                self.PrintRegister(reg);
            }
        }

        fn PrintRd(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).RdValue();
                self.PrintRegister(reg);
            }
        }
	fn PrintVs1(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).Vs1Value();
                self.PrintVRegister(reg);
            }
        }

        fn PrintVs2(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).Vs2Value();
                self.PrintVRegister(reg);
            }
        }

        fn PrintVd(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).VdValue();
                self.PrintVRegister(reg);
            }
        }
        fn PrintFRs1(&mut self, instr: *mut Instruction) {
          unsafe {
            let reg = (*instr).Rs1Value();
            self.PrintFPURegister(reg);
          }
        }

        fn PrintFRs2(&mut self, instr: *mut Instruction) {
          unsafe {
            let reg = (*instr).Rs2Value();
            self.PrintFPURegister(reg);
          }
        }

        fn PrintFRs3(&mut self, instr: *mut Instruction) {
          unsafe {
            let reg = (*instr).Rs3Value();
            self.PrintFPURegister(reg);
          }
        }

        fn PrintFRd(&mut self, instr: *mut Instruction) {
          unsafe {
            let reg = (*instr).RdValue();
            self.PrintFPURegister(reg);
          }
        }


        fn PrintUimm(&mut self, instr: *mut Instruction) {
            unsafe {
                let val = (*instr).Rs1Value();
		let formatted = format!("0x{:x}",val);
                self.out_buffer_pos_ += snprintf(
                    self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                    self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                    formatted.as_str(),
                ) as i32;
            }
        }

        fn PrintImm12X(&mut self, instr: *mut Instruction) {
            unsafe {
                let imm = (*instr).Imm12Value();
		let formatted = format!("0x{:x}",imm);
                self.out_buffer_pos_ += snprintf(
                    self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                    self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                    formatted.as_str(),
                ) as i32;
            }
        }
      fn PrintRvcImm6U(&mut self, instr: *mut Instruction) {
            unsafe {
                let imm = (*instr).RvcImm6Value() & 0xFFFFF;
		let formatted = format!("0x{:x}",imm);
                self.out_buffer_pos_ += snprintf(
                    self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                    self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                    formatted.as_str(),
                ) as i32;
            }
        }

        fn PrintImm12(&mut self, instr: *mut Instruction) {
            unsafe {
                let imm = (*instr).Imm12Value();
		let formatted = format!("{}",imm);
                self.out_buffer_pos_ += snprintf(
                    self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                    self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                    formatted.as_str(),
                ) as i32;
            }
        }
	unsafe fn PrintTarget(&mut self, instr: *mut Instruction) {
        if Assembler::IsJalr((*instr).InstructionBits()) {
          if Assembler::IsAuipc(((*instr).offset(-4)).InstructionBits())
              && ((*instr).offset(-4)).RdValue() == (*instr).Rs1Value() {
            let imm = Assembler::BrachlongOffset(((*instr).offset(-4)).InstructionBits(),
                                                  (*instr).InstructionBits());
            let target = self.converter_.NameOfAddress(((*instr).offset(-4) as *mut u8).offset(imm as isize));
	    let formatted = format!(" -> {}", CString::from_raw(target as *mut i8).to_str().unwrap());
            self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                                self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                                formatted.as_str()) as i32;
            return;
          }
        }
      }
	unsafe fn PrintBranchOffset(&mut self, instr: *mut Instruction) {
        let imm = (*instr).BranchOffset();
        let target = self.converter_.NameOfAddress((instr as *mut u8).offset(imm as isize));
	let formatted = format!("{} -> {}", imm, CString::from_raw(target as *mut i8).to_str().unwrap());
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

	unsafe fn PrintStoreOffset(&mut self, instr: *mut Instruction) {
        let imm = (*instr).StoreOffset();
	let formatted = format!("{}",imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }
      unsafe fn PrintRvvSEW(&mut self, instr: *mut Instruction) {
        let sew = (*instr).RvvSEW();
        let formatted = format!("{}", sew);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvvLMUL(&mut self, instr: *mut Instruction) {
        let lmul = (*instr).RvvLMUL();
	let formatted = format!("{}", lmul);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

      unsafe fn PrintRvvSimm5(&mut self, instr: *mut Instruction) {
        let simm5 = (*instr).RvvSimm5();
	let formatted = format!("{}", simm5);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvvUimm5(&mut self, instr: *mut Instruction) {
        let uimm5 = (*instr).RvvUimm5();
	let formatted = format!("{}", uimm5);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintImm20U(&mut self, instr: *mut Instruction) {
        let imm = (*instr).Imm20UValue();
	let formatted = format!("0x{:x}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintImm20J(&mut self, instr: *mut Instruction) {
        let imm = (*instr).Imm20JValue();
        let target = self.converter_.NameOfAddress((instr as *mut u8).offset(imm as isize));
	let formatted = format!("{} -> {}", imm, CString::from_raw(target as *mut i8).to_str().unwrap());
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintShamt(&mut self, instr: *mut Instruction) {
        let imm = (*instr).Shamt();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintShamt32(&mut self, instr: *mut Instruction) {
        let imm = (*instr).Shamt32();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm6(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6Value();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }
	unsafe fn PrintRvcImm6Addi16sp(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6Addi16spValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcShamt(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcShamt6();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm6Ldsp(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6LdspValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm6Lwsp(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6LwspValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm6Swsp(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6SwspValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm6Sdsp(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm6SdspValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm5W(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm5WValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm5D(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm5DValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm8Addi4spn(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm8Addi4spnValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm11CJ(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm11CJValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn PrintRvcImm8B(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvcImm8BValue();
	let formatted = format!("{}", imm);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }
      unsafe fn PrintRvvVm(&mut self, instr: *mut Instruction) {
        let imm = (*instr).RvvVM();
        if imm == 0 {
          let formatted = format!("  v0.t");
          self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                              self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                              formatted.as_str()) as i32;
        }
      }

      unsafe fn PrintAcquireRelease(&mut self, instr: *mut Instruction) {
        let aq = (*instr).AqValue();
        let rl = (*instr).RlValue();
        if aq || rl {
          let mut s = String::new();
          if aq {
            s.push_str(".aq");
          }
          if (rl) {
            s.push_str(".rl");
          }
          let formatted = format!("{}", s);
          self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                              self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                              formatted.as_str()) as i32;
        }
      }
      unsafe fn PrintCSRReg(&mut self, instr: *mut Instruction) {
        let csr_reg = (*instr).CsrValue();
        let s = match csr_reg {
          csr_fflags => "csr_fflags".to_string(),
          csr_frm => "csr_frm".to_string(),
          csr_fcsr => "csr_fcsr".to_string(),
          csr_cycle => "csr_cycle".to_string(),
          csr_time => "csr_time".to_string(),
          csr_instret => "csr_instret".to_string(),
          csr_cycleh => "csr_cycleh".to_string(),
          csr_timeh => "csr_timeh".to_string(),
          csr_instreth => "csr_instreth".to_string(),
          _ => panic!("UNREACHABLE"),
        };
        let formatted = format!("{}", s);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

      unsafe fn PrintRoundingMode(&mut self, instr: *mut Instruction) {
        let frm = (*instr).RoundMode();
        let s = match frm {
          RNE => "RNE".to_string(),
          RTZ => "RTZ".to_string(),
          RDN => "RDN".to_string(),
          RUP => "RUP".to_string(),
          RMM => "RMM".to_string(),
          DYN => "DYN".to_string(),
          _ => panic!("UNREACHABLE"),
        };
        let formatted = format!("{}", s);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

      unsafe fn PrintMemoryOrder(&mut self, instr: *mut Instruction, is_pred: bool) {
        let mem_order = (*instr).MemoryOrder(is_pred);
        let mut s = String::new();
        if mem_order & PSI == PSI {
          s += "i";
        }
        if mem_order & PSO == PSO {
          s += "o";
        }
        if mem_order & PSR == PSR {
          s += "r";
        }
        if mem_order & PSW == PSW {
          s += "w";
        }
	let formatted = format!("{}", s);
        self.out_buffer_pos_ += snprintf(self.out_buffer_.start_.offset(self.out_buffer_pos_ as isize),
                            self.out_buffer_.length_ - self.out_buffer_pos_ as usize,
                            formatted.as_str()) as i32;
      }

    unsafe fn FormatRegister(&mut self, instr: *mut Instruction, format: &str) -> i32 {
        assert_eq!(&format[0..1], "r");
        if &format[1..2] == "s" {
            if &format[2..3] == "1" {
                let reg = (*instr).Rs1Value();
                self.PrintRegister(reg);
                3
            } else if &format[2..3] == "2" {
                let reg = (*instr).Rs2Value();
                self.PrintRegister(reg);
                3
            } else {
                panic!();
            }
        } else if &format[1..2] == "d" {
            let reg = (*instr).RdValue();
            self.PrintRegister(reg);
            2
        } else {
            panic!();
        }
    }
	unsafe fn FormatFPURegisterOrRoundMode(&mut self, instr: *mut Instruction,
                                          format: &str) -> i32 {
    assert_eq!(&format[0..1], "f");
    if &format[1..2] == "s" {  // 'fs[1-3]: Rs register.
      if &format[2..3] == "1" {
        let reg = (*instr).Rs1Value();
        self.PrintFPURegister(reg);
        return 3;
      } else if &format[2..3] == "2" {
        let reg = (*instr).Rs2Value();
        self.PrintFPURegister(reg);
        return 3;
      } else if &format[2..3] == "3" {
        let reg = (*instr).Rs3Value();
        self.PrintFPURegister(reg);
        return 3;
      }
      panic!();
    } else if &format[1..2] == "d" {  // 'fd: fd register.
      let reg = (*instr).RdValue();
      self.PrintFPURegister(reg);
      return 2;
    } else if &format[1..2] == "r" {  // 'frm
      assert_eq!(&format[0..3], "frm");
      self.PrintRoundingMode(instr);
      return 3;
    }
    panic!();
  }

	unsafe fn FormatRvcRegister(&mut self, instr: *mut Instruction, format: &str) -> i32 {
        assert_eq!(&format[0..1], "C");
        assert!(&format[1..2] == "r" || &format[1..2] == "f");
        if &format[2..3] == "s" {
            if &format[3..4] == "1" {
                if &format[4..5] == "s" {
                    let reg = (*instr).RvcRs1sValue();
                    if &format[1..2] == "r" {
                        self.PrintRegister(reg);
                    } else if &format[1..2] == "f" {
                        self.PrintFPURegister(reg);
                    }
                    return 5;
                }
                let reg = (*instr).RvcRs1Value();
                if &format[1..2] == "r" {
                    self.PrintRegister(reg);
                } else if &format[1..2] == "f" {
                    self.PrintFPURegister(reg);
                }
                return 4;
            } else if &format[3..4] == "2" {
                if &format[4..5] == "s" {
                    let reg = (*instr).RvcRs2sValue();
                    if &format[1..2] == "r" {
                        self.PrintRegister(reg);
                    } else if &format[1..2] == "f" {
                        self.PrintFPURegister(reg);
                    }
                    return 5;
                }
                let reg = (*instr).RvcRs2Value();
                if &format[1..2] == "r" {
                    self.PrintRegister(reg);
                } else if &format[1..2] == "f" {
                    self.PrintFPURegister(reg);
                }
                return 4;
            } else {
                panic!();
            }
        } else if &format[2..3] == "d" {
            let reg = (*instr).RvcRdValue();
            if &format[1..2] == "r" {
                self.PrintRegister(reg);
            } else if &format[1..2] == "f" {
                self.PrintFPURegister(reg);
            }
            return 3;
        } else {
            panic!();
        }
    }
    unsafe fn FormatRvcImm(&mut self, instr: *mut Instruction, format: &str) -> i32 {
        assert_eq!(&format[0..4], "Cimm");
        if &format[4..5] == "6" {
            if &format[5..6] == "U" {
                assert_eq!(&format[0..6], "Cimm6U");
                self.PrintRvcImm6U(instr);
                return 6;
            } else if &format[5..6] == "A" {
                if &format[9..11] == "16" {
                    assert_eq!(&format[0..13], "Cimm6Addi16sp");
                    self.PrintRvcImm6Addi16sp(instr);
                    return 13;
                } else {
                    panic!();
                }
            } else if &format[5..6] == "L" {
                if &format[6..7] == "d" {
                    if &format[7..8] == "s" {
                        assert_eq!(&format[0..9], "Cimm6Ldsp");
                        self.PrintRvcImm6Ldsp(instr);
                        return 9;
                    }
                } else if &format[6..7] == "w" {
                    if &format[7..8] == "s" {
                        assert_eq!(&format[0..9], "Cimm6Lwsp");
                        self.PrintRvcImm6Lwsp(instr);
                        return 9;
                    }
                } else {
                    panic!();
                }
            } else if &format[5..6] == "S" {
                if &format[6..7] == "w" {
                    assert_eq!(&format[0..9], "Cimm6Swsp

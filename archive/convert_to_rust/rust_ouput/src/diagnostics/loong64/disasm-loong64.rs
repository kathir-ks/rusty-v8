// Converted from V8 C++ source files:
// Header: N/A
// Implementation: disasm-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::fmt;
use std::mem;
use std::ptr;

use crate::base::strings::string;
use crate::builtins::riscv::builtins_riscv::Instruction;
use crate::builtins::riscv::builtins_riscv::instruction;

pub mod base {
    pub mod platform {
        pub mod platform_posix {
            #[allow(dead_code)]
            pub fn print(format: &str, args: Vec<std::ffi::CString>) {}
        }
    }
    pub mod strings {
        pub struct string {}
    }
    pub mod vector {
        pub struct Vector<T> {
            data: Vec<T>,
        }

        impl<T> Vector<T> {
            pub fn new(data: Vec<T>) -> Self {
                Vector { data }
            }

            pub fn length(&self) -> usize {
                self.data.len()
            }

            pub fn begin(&self) -> *const T {
                self.data.as_ptr()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }

            pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                self.data.get_mut(index)
            }

            pub fn push(&mut self, value: T) {
                self.data.push(value);
            }
        }
    }
}

pub mod codegen {
    pub mod loong64 {
        pub mod constants_loong64 {
            // Define constants related to LoongArch64 architecture
            pub const kSi12Bits: i32 = 12;
            pub const kSi14Bits: i32 = 14;
            pub const kSi16Bits: i32 = 16;
            pub const kSi20Bits: i32 = 20;
            pub const kOffsLowBits: i32 = 2;
            pub const kOffs21HighBits: i32 = 19;
            pub const kOffs26HighBits: i32 = 24;
        }
    }
    pub mod macro_assembler {
        pub struct MacroAssembler {}
    }
}

pub mod diagnostics {
    pub mod disasm {
        pub struct NameConverter {
            tmp_buffer_: crate::base::vector::Vector<char>,
        }

        impl NameConverter {
            pub fn new() -> Self {
                let tmp_buffer_data: Vec<char> = vec!['\0'; 256];
                NameConverter {
                    tmp_buffer_: crate::base::vector::Vector::new(tmp_buffer_data),
                }
            }
            pub fn NameOfAddress(&self, addr: *mut u8) -> *const char {
                unsafe {
                    let addr_str = format!("{:p}", addr);
                    let mut buffer = self.tmp_buffer_.data.clone();

                    for (i, c) in addr_str.chars().enumerate() {
                        if i < buffer.len() - 1 {
                            buffer[i] = c;
                        }
                    }
                    buffer[addr_str.len()] = '\0';

                    self.tmp_buffer_.data = buffer;
                    self.tmp_buffer_.begin() as *const char
                }
            }
            pub fn NameOfConstant(&self, addr: *mut u8) -> *const char {
                self.NameOfAddress(addr)
            }
            pub fn NameOfCPURegister(&self, reg: i32) -> *const char {
                crate::internal::Registers::Name(reg)
            }
            pub fn NameOfXMMRegister(&self, reg: i32) -> *const char {
                crate::internal::FPURegisters::Name(reg)
            }
            pub fn NameOfByteCPURegister(&self, reg: i32) -> *const char {
                std::ptr::null()
            }
            pub fn NameInCode(&self, addr: *mut u8) -> *const char {
                ""
            }
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

            pub fn InstructionDecode(&self, buffer: crate::base::vector::Vector<char>, instruction: *mut u8) -> i32 {
                let mut decoder = crate::internal::Decoder::new(&self.converter_, buffer);
                decoder.InstructionDecode(instruction)
            }

            pub fn ConstantPoolSizeAt(&self, instruction: *mut u8) -> i32 {
                -1
            }

            pub fn Disassemble(&self, f: *mut std::ffi::c_void, begin: *mut u8, end: *mut u8, unimplemented_action: UnimplementedOpcodeAction) {
                unsafe {
                    let mut pc = begin;
                    while pc < end {
                        let mut buffer_data: Vec<char> = vec!['\0'; 128];
                        let mut buffer = crate::base::vector::Vector::new(buffer_data);
                        buffer.data[0] = '\0';

                        let prev_pc = pc;
                        pc = pc.offset(self.InstructionDecode(buffer, pc) as isize);

                        let instruction_bits = *(prev_pc as *mut i32);

                        let formatted_string = format!(
                            "{:p}    {:08x}      {}",
                            prev_pc,
                            instruction_bits,
                            String::from_utf8_lossy(buffer.data.as_slice()),
                        );
                        println!("{}", formatted_string);
                    }
                }
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub enum UnimplementedOpcodeAction {
            kDefault,
        }
    }
}

pub mod internal {
    use std::fmt;
    use std::mem;
    use std::ptr;

    use crate::base::strings::string;
    use crate::base::vector::Vector;
    use crate::builtins::riscv::builtins_riscv::Instruction;
    use crate::codegen::loong64::constants_loong64;
    use crate::diagnostics::disasm::NameConverter;

    // Instruction type enum (replace with actual instruction representation)
    pub enum InstructionType {
        kOp6Type,
        kOp7Type,
        kOp8Type,
        kOp10Type,
        kOp12Type,
        kOp14Type,
        kOp17Type,
        kOp22Type,
        kUnsupported,
    }

    impl Instruction {
        pub fn InstructionBits(&self) -> u32 {
            0
        }
        pub fn InstructionType(&self) -> InstructionType {
            InstructionType::kUnsupported
        }
        pub fn At(_instr_ptr: *mut u8) -> *mut Instruction {
            std::ptr::null_mut()
        }
        pub fn RjValue(&self) -> i32 {
            0
        }
        pub fn RkValue(&self) -> i32 {
            0
        }
        pub fn RdValue(&self) -> i32 {
            0
        }
        pub fn FjValue(&self) -> i32 {
            0
        }
        pub fn FkValue(&self) -> i32 {
            0
        }
        pub fn FdValue(&self) -> i32 {
            0
        }
        pub fn FaValue(&self) -> i32 {
            0
        }
        pub fn Sa2Value(&self) -> i32 {
            0
        }
        pub fn Sa3Value(&self) -> i32 {
            0
        }
        pub fn Ui5Value(&self) -> i32 {
            0
        }
        pub fn Ui6Value(&self) -> i32 {
            0
        }
        pub fn Ui12Value(&self) -> i32 {
            0
        }
        pub fn Si12Value(&self) -> i32 {
            0
        }
        pub fn Si14Value(&self) -> i32 {
            0
        }
        pub fn Si16Value(&self) -> i32 {
            0
        }
        pub fn Si20Value(&self) -> i32 {
            0
        }
        pub fn Offs16Value(&self) -> i32 {
            0
        }
        pub fn Offs21Value(&self) -> i32 {
            0
        }
        pub fn Offs26Value(&self) -> i32 {
            0
        }
        pub fn MsbdValue(&self) -> i32 {
            0
        }
        pub fn LsbdValue(&self) -> i32 {
            0
        }
        pub fn MsbwValue(&self) -> i32 {
            0
        }
        pub fn LsbwValue(&self) -> i32 {
            0
        }
        pub fn CjValue(&self) -> i32 {
            0
        }
        pub fn CdValue(&self) -> i32 {
            0
        }
        pub fn CaValue(&self) -> i32 {
            0
        }
        pub fn CodeValue(&self) -> i32 {
            0
        }
        pub fn Hint5Value(&self) -> i32 {
            0
        }
        pub fn Hint15Value(&self) -> i32 {
            0
        }
        pub fn Bit(&self, _bit: i32) -> i32 {
            0
        }
    }

    // Decoder decodes and disassembles instructions into an output buffer.
    // It uses the converter to convert register names and call destinations into
    // more informative description.
    pub struct Decoder<'a> {
        converter_: &'a NameConverter,
        out_buffer_: Vector<char>,
        out_buffer_pos_: i32,
    }

    impl<'a> Decoder<'a> {
        pub fn new(converter: &'a NameConverter, out_buffer: Vector<char>) -> Self {
            let mut decoder = Decoder {
                converter_: converter,
                out_buffer_: out_buffer,
                out_buffer_pos_: 0,
            };
            decoder.out_buffer_[decoder.out_buffer_pos_ as usize] = '\0';
            decoder
        }

        // Writes one disassembled instruction into 'buffer' (0-terminated).
        // Returns the length of the disassembled machine instruction in bytes.
        pub fn InstructionDecode(&mut self, instruction: *mut u8) -> i32 {
            unsafe {
                let instr = Instruction::At(instruction);
                self.out_buffer_pos_ +=
                    crate::base::strings::string::format(format_args!("{:08x}       ", (*instr).InstructionBits())).len() as i32;
                match (*instr).InstructionType() {
                    InstructionType::kOp6Type => {
                        self.DecodeTypekOp6(instr);
                    }
                    InstructionType::kOp7Type => {
                        self.DecodeTypekOp7(instr);
                    }
                    InstructionType::kOp8Type => {
                        self.DecodeTypekOp8(instr);
                    }
                    InstructionType::kOp10Type => {
                        self.DecodeTypekOp10(instr);
                    }
                    InstructionType::kOp12Type => {
                        self.DecodeTypekOp12(instr);
                    }
                    InstructionType::kOp14Type => {
                        self.DecodeTypekOp14(instr);
                    }
                    InstructionType::kOp17Type => {
                        return self.DecodeTypekOp17(instr);
                    }
                    InstructionType::kOp22Type => {
                        self.DecodeTypekOp22(instr);
                    }
                    InstructionType::kUnsupported => {
                        self.Format(instr, "UNSUPPORTED");
                    }
                    _ => {
                        self.Format(instr, "UNSUPPORTED");
                    }
                }
                8
            }
        }

        // Bottleneck functions to print into the out_buffer.
        fn PrintChar(&mut self, ch: char) {
            if self.out_buffer_pos_ < (self.out_buffer_.length() as i32 - 1) {
                self.out_buffer_.get_mut(self.out_buffer_pos_ as usize).map(|x| *x = ch);
                self.out_buffer_pos_ += 1;
                self.out_buffer_.get_mut(self.out_buffer_pos_ as usize).map(|x| *x = '\0');
            }
        }

        fn Print(&mut self, str: &str) {
            for cur in str.chars() {
                if self.out_buffer_pos_ < (self.out_buffer_.length() as i32 - 1) {
                    self.PrintChar(cur);
                } else {
                    break;
                }
            }
        }

        // Printing of common values.
        fn PrintRegister(&mut self, reg: i32) {
            unsafe {
                let name = self.converter_.NameOfCPURegister(reg);
                let c_str = std::ffi::CStr::from_ptr(name);
                let rust_str = c_str.to_str().unwrap();
                self.Print(rust_str);
            }
        }

        fn PrintFPURegister(&mut self, freg: i32) {
            unsafe {
                let name = self.converter_.NameOfXMMRegister(freg);
                let c_str = std::ffi::CStr::from_ptr(name);
                let rust_str = c_str.to_str().unwrap();
                self.Print(rust_str);
            }
        }

        fn PrintFPUStatusRegister(&mut self, freg: i32) {
            // Placeholder implementation
            self.Print("fcsr");
        }

        fn PrintRj(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).RjValue();
                self.PrintRegister(reg);
            }
        }

        fn PrintRk(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).RkValue();
                self.PrintRegister(reg);
            }
        }

        fn PrintRd(&mut self, instr: *mut Instruction) {
            unsafe {
                let reg = (*instr).RdValue();
                self.PrintRegister(reg);
            }
        }

        fn PrintFj(&mut self, instr: *mut Instruction) {
            unsafe {
                let freg = (*instr).FjValue();
                self.PrintFPURegister(freg);
            }
        }

        fn PrintFk(&mut self, instr: *mut Instruction) {
            unsafe {
                let freg = (*instr).FkValue();
                self.PrintFPURegister(freg);
            }
        }

        fn PrintFd(&mut self, instr: *mut Instruction) {
            unsafe {
                let freg = (*instr).FdValue();
                self.PrintFPURegister(freg);
            }
        }

        fn PrintFa(&mut self, instr: *mut Instruction) {
            unsafe {
                let freg = (*instr).FaValue();
                self.PrintFPURegister(freg);
            }
        }

        fn PrintSa2(&mut self, instr: *mut Instruction) {
            unsafe {
                let sa = (*instr).Sa2Value();
                let opcode = ((*instr).InstructionBits() >> 18) << 18;
                let mut printed = String::new();
                if opcode == ALSL || opcode == ALSL_D {
                    printed = format!("{}", sa + 1);
                } else {
                    printed = format!("{}", sa);
                }
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintSa3(&mut self, instr: *mut Instruction) {
            unsafe {
                let sa = (*instr).Sa3Value();
                let printed = format!("{}", sa);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintUi5(&mut self, instr: *mut Instruction) {
            unsafe {
                let ui = (*instr).Ui5Value();
                let printed = format!("{}", ui);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintUi6(&mut self, instr: *mut Instruction) {
            unsafe {
                let ui = (*instr).Ui6Value();
                let printed = format!("{}", ui);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintUi12(&mut self, instr: *mut Instruction) {
            unsafe {
                let ui = (*instr).Ui12Value();
                let printed = format!("{}", ui);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintMsbw(&mut self, instr: *mut Instruction) {
            unsafe {
                let msbw = (*instr).MsbwValue();
                let printed = format!("{}", msbw);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintLsbw(&mut self, instr: *mut Instruction) {
            unsafe {
                let lsbw = (*instr).LsbwValue();
                let printed = format!("{}", lsbw);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintMsbd(&mut self, instr: *mut Instruction) {
            unsafe {
                let msbd = (*instr).MsbdValue();
                let printed = format!("{}", msbd);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintLsbd(&mut self, instr: *mut Instruction) {
            unsafe {
                let lsbd = (*instr).LsbdValue();
                let printed = format!("{}", lsbd);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintSi12(&mut self, instr: *mut Instruction) {
            unsafe {
                let si = ((*instr).Si12Value() << (32 - constants_loong64::kSi12Bits)) >> (32 - constants_loong64::kSi12Bits);
                let printed = format!("{}(0x{:x})", si, (*instr).Si12Value());
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintSi14(&mut self, instr: *mut Instruction) {
            unsafe {
                let mut si = ((*instr).Si14Value() << (32 - constants_loong64::kSi14Bits)) >> (32 - constants_loong64::kSi14Bits);
                si <<= 2;
                let printed = format!("{}(0x{:x})", si, (*instr).Si14Value() << 2);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintSi16(&mut self, instr: *mut Instruction) {
            unsafe {
                let si = ((*instr).Si16Value() << (32 - constants_loong64::kSi16Bits)) >> (32 - constants_loong64::kSi16Bits);
                let printed = format!("{}(0x{:x})", si, (*instr).Si16Value());
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintSi20(&mut self, instr: *mut Instruction) {
            unsafe {
                let si = ((*instr).Si20Value() << (32 - constants_loong64::kSi20Bits)) >> (32 - constants_loong64::kSi20Bits);
                let printed = format!("{}(0x{:x})", si, (*instr).Si20Value());
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintXi12(&mut self, instr: *mut Instruction) {
            unsafe {
                let xi = (*instr).Ui12Value();
                let printed = format!("0x{:x}", xi);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintXi20(&mut self, instr: *mut Instruction) {
            unsafe {
                let xi = (*instr).Si20Value();
                let printed = format!("0x{:x}", xi);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintCj(&mut self, instr: *mut Instruction) {
            unsafe {
                let cj = (*instr).CjValue();
                let printed = format!("{}", cj);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintCd(&mut self, instr: *mut Instruction) {
            unsafe {
                let cd = (*instr).CdValue();
                let printed = format!("{}", cd);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintCa(&mut self, instr: *mut Instruction) {
            unsafe {
                let ca = (*instr).CaValue();
                let printed = format!("{}", ca);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintCode(&mut self, instr: *mut Instruction) {
            unsafe {
                let code = (*instr).CodeValue();
                let printed = format!("0x{:x}({})", code, code);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintHint5(&mut self, instr: *mut Instruction) {
            unsafe {
                let hint = (*instr).Hint5Value();
                let printed = format!("0x{:x}({})", hint, hint);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintHint15(&mut self, instr: *mut Instruction) {
            unsafe {
                let hint = (*instr).Hint15Value();
                let printed = format!("0x{:x}({})", hint, hint);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintPCOffs16(&mut self, instr: *mut Instruction) {
            unsafe {
                let n_bits = 2;
                let offs = (*instr).Offs16Value();
                let target = ((offs << n_bits) << (32 - constants_loong64::kOffsLowBits - n_bits)) >>
                             (32 - constants_loong64::kOffsLowBits - n_bits);
                let target_addr = (instr as usize as isize + target as isize) as *mut u8;
                let address_name = self.converter_.NameOfAddress(target_addr);
                let c_str = std::ffi::CStr::from_ptr(address_name);
                let address_name_str = c_str.to_str().unwrap();
                self.Print(address_name_str);
            }
        }

        fn PrintPCOffs21(&mut self, instr: *mut Instruction) {
            unsafe {
                let n_bits = 2;
                let offs = (*instr).Offs21Value();
                let target =
                    ((offs << n_bits) << (32 - constants_loong64::kOffsLowBits - constants_loong64::kOffs21HighBits - n_bits)) >>
                    (32 - constants_loong64::kOffsLowBits - constants_loong64::kOffs21HighBits - n_bits);
                let target_addr = (instr as usize as isize + target as isize) as *mut u8;
                let address_name = self.converter_.NameOfAddress(target_addr);
                let c_str = std::ffi::CStr::from_ptr(address_name);
                let address_name_str = c_str.to_str().unwrap();
                self.Print(address_name_str);
            }
        }

        fn PrintPCOffs26(&mut self, instr: *mut Instruction) {
            unsafe {
                let n_bits = 2;
                let offs = (*instr).Offs26Value();
                let target =
                    ((offs << n_bits) << (32 - constants_loong64::kOffsLowBits - constants_loong64::kOffs26HighBits - n_bits)) >>
                    (32 - constants_loong64::kOffsLowBits - constants_loong64::kOffs26HighBits - n_bits);
                let target_addr = (instr as usize as isize + target as isize) as *mut u8;
                let address_name = self.converter_.NameOfAddress(target_addr);
                let c_str = std::ffi::CStr::from_ptr(address_name);
                let address_name_str = c_str.to_str().unwrap();
                self.Print(address_name_str);
            }
        }

        fn PrintOffs16(&mut self, instr: *mut Instruction) {
            unsafe {
                let offs = (*instr).Offs16Value();
                let printed = format!("0x{:x}", offs << 2);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintOffs21(&mut self, instr: *mut Instruction) {
            unsafe {
                let offs = (*instr).Offs21Value();
                let printed = format!("0x{:x}", offs << 2);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        fn PrintOffs26(&mut self, instr: *mut Instruction) {
            unsafe {
                let offs = (*instr).Offs26Value();
                let printed = format!("0x{:x}", offs << 2);
                self.out_buffer_pos_ += printed.len() as i32;
                self.Print(&printed);
            }
        }

        // Handle all register based formatting in this function to reduce the
        // complexity of FormatOption.
        fn FormatRegister(&mut self, instr: *mut Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'r');
            if format.chars().nth(1) == Some('j') {
                // 'rj: Rj register.
                unsafe {
                    let reg = (*instr).RjValue();
                    self.PrintRegister(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('k') {
                // 'rk: rk register.
                unsafe {
                    let reg = (*instr).RkValue();
                    self.PrintRegister(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('d') {
                // 'rd: rd register.
                unsafe {
                    let reg = (*instr).RdValue();
                    self.PrintRegister(reg);
                    2
                }
            } else {
                unreachable!();
            }
        }

        // Handle all FPUregister based formatting in this function to reduce the
        // complexity of FormatOption.
        fn FormatFPURegister(&mut self, instr: *mut Instruction, format: &str) -> i32 {
            assert_eq!(format.chars().next().unwrap(), 'f');
            if format.chars().nth(1) == Some('j') {
                // 'fj: fj register.
                unsafe {
                    let reg = (*instr).FjValue();
                    self.PrintFPURegister(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('k') {
                // 'fk: fk register.
                unsafe {
                    let reg = (*instr).FkValue();
                    self.PrintFPURegister(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('d') {
                // 'fd: fd register.
                unsafe {
                    let reg = (*instr).FdValue();
                    self.PrintFPURegister(reg);
                    2
                }
            } else if format.chars().nth(1) == Some('a') {
                // 'fa: fa register.
                unsafe {
                    let reg = (*instr).FaValue();
                    self.PrintFPURegister(reg);
                    2
                }
            } else {
                unreachable!();
            }
        }

        // FormatOption takes a formatting string and interprets it based on
        // the current instructions. The format string points to the first
        // character of the option string (the option escape has already been
        // consumed by the caller.)  FormatOption returns the number of
        // characters that were consumed from the formatting string.
        fn FormatOption(&mut self, instr: *mut Instruction, format: &str) -> i32 {
            match format.chars().next() {
                Some('c') => match format.chars().nth(1) {
                    Some('a') => {
                        assert!(format.starts_with("ca"));
                        self.PrintCa(instr);
                        2
                    }
                    Some('d') => {
                        assert!(format.starts_with("cd"));
                        self.PrintCd(instr);
                        2
                    }
                    Some('j') => {
                        assert!(format.starts_with("cj"));
                        self.PrintCj(instr);
                        2
                    }
                    Some('o') => {
                        assert!(format.starts_with("code"));
                        self.PrintCode(instr);
                        4
                    }
                    _ => 0,
                },
                Some('f') => self.FormatFPURegister(instr, format),
                Some('h') => {
                    if format.chars().nth(4) == Some('5') {
                        assert!(format.starts_with("hint5"));
                        self.PrintHint5(instr);
                        5
                    } else if format.chars().nth(4) == Some('1') {
                        assert!(format.starts_with("hint15"));
                        self.PrintHint15(instr);
                        6
                    } else {
                        0
                    }
                }
                Some('l') => match format.chars().nth(3) {
                    Some('w') => {
                        assert!(format.starts_with("lsbw"));
                        self.PrintLsbw(instr);
                        4
                    }
                    Some('d') => {
                        assert!(format.starts_with("lsbd"));
                        self.PrintLsbd(instr);
                        4
                    }
                    _ => 0,
                },
                Some('m') => {
                    if format.chars().nth(3) == Some('w') {
                        assert!(format.starts_with("msbw"));
                        self.PrintMsbw(instr);
                    } else if format.chars().nth(3) == Some('d') {
                        assert!(format.starts_with("msbd"));
                        self.PrintMsbd(instr);
                    }
                    4
                }
                Some('o') => {
                    if format.chars().nth(1) == Some('f') {
                        if format.chars().nth(4) == Some('1') {
                            assert!(format.starts_with("offs16"));
                            self.PrintOffs16(instr);
                            6
                        } else if format.chars().nth(4) == Some('2') {
                            if format.chars().nth(5) == Some('1') {
                                assert!(format.starts_with("offs21"));
                                self.PrintOffs21(instr);
                                6
                            } else if format.chars().nth(5) == Some('6') {
                                assert!(format.starts_with("offs26"));
                                self.PrintOffs26(instr);
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
                Some('p') => {
                    if format.chars().nth(6) == Some('1') {
                        assert!(format.starts_with("pcoffs16"));
                        self.PrintPCOffs16(instr);
                        8
                    } else if format.chars().nth(6) == Some('2') {
                        if format.chars().nth(7) == Some('1') {
                            assert!(format.starts_with("pcoffs21

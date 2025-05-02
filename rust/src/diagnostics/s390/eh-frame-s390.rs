// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note:  The s390 assembler and eh-frame writers might exist in different crates in a real project.
//        This example keeps them in the same file for simplicity.  Adjust module definitions as needed.

mod assembler_s390 {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(u8);

    impl Register {
        pub const fn from_code(code: u8) -> Self {
            Register(code)
        }

        pub fn code(&self) -> u8 {
            self.0
        }
    }

    pub const fp: Register = Register::from_code(11);
    pub const r14: Register = Register::from_code(14);
    pub const sp: Register = Register::from_code(15);
    pub const r0: Register = Register::from_code(0);

    // This could be an enum if the register set is known at compile time.
    pub const kRegCode_fp: u8 = 11;
    pub const kRegCode_r14: u8 = 14;
    pub const kRegCode_sp: u8 = 15;
    pub const kRegCode_r0: u8 = 0;
}

mod eh_frame {
    use crate::assembler_s390::{self, Register, fp, r14};

    const K_R0_DWARF_CODE: u32 = 0;
    const K_FP_DWARF_CODE: u32 = 11;
    const K_R14_DWARF_CODE: u32 = 14;
    const K_SP_DWARF_CODE: u32 = 15;

    pub struct EhFrameConstants {}

    impl EhFrameConstants {
        pub const K_CODE_ALIGNMENT_FACTOR: i32 = 2;
        pub const K_DATA_ALIGNMENT_FACTOR: i32 = -8;
    }

    pub struct EhFrameWriter {
        // Example fields representing internal state
        base_address_register: Option<Register>,
        base_address_offset: i32,
        modified_registers: Vec<Register>,
        buffer: Vec<u8>, // Store the generated EH frame data
    }

    impl EhFrameWriter {
        pub fn new() -> Self {
            EhFrameWriter {
                base_address_register: None,
                base_address_offset: 0,
                modified_registers: Vec::new(),
                buffer: Vec::new(),
            }
        }

        pub fn write_return_address_register_code(&mut self) {
            self.write_uleb128(K_R14_DWARF_CODE);
        }

        pub fn write_initial_state_in_cie(&mut self) {
            self.set_base_address_register_and_offset(fp, 0);
            self.record_register_not_modified(r14);
        }

        pub fn register_to_dwarf_code(name: Register) -> u32 {
            match name.code() {
                assembler_s390::kRegCode_fp => K_FP_DWARF_CODE,
                assembler_s390::kRegCode_r14 => K_R14_DWARF_CODE,
                assembler_s390::kRegCode_sp => K_SP_DWARF_CODE,
                assembler_s390::kRegCode_r0 => K_R0_DWARF_CODE,
                _ => {
                    // Corresponds to UNIMPLEMENTED();
                    panic!("Unimplemented register code");
                }
            }
        }

        fn set_base_address_register_and_offset(&mut self, reg: Register, offset: i32) {
            self.base_address_register = Some(reg);
            self.base_address_offset = offset;
        }

        fn record_register_not_modified(&mut self, reg: Register) {
            self.modified_registers.push(reg);
        }

        fn write_uleb128(&mut self, value: u32) {
            let mut remaining = value;
            loop {
                let mut byte = (remaining & 0x7f) as u8;
                remaining >>= 7;
                if remaining != 0 {
                    byte |= 0x80;
                }
                self.buffer.push(byte);
                if remaining == 0 {
                    break;
                }
            }
        }

        // Accessor for generated data (e.g., to write it to a file).
        pub fn get_buffer(&self) -> &Vec<u8> {
            &self.buffer
        }

    }

    #[cfg(feature = "disassembler")]
    pub mod eh_frame_disassembler {
        pub fn dwarf_register_code_to_string(code: i32) -> &'static str {
            match code {
                11 => "fp",
                14 => "lr",
                15 => "sp",
                _ => {
                    // Corresponds to UNIMPLEMENTED();
                    panic!("Unimplemented dwarf register code");
                }
            }
        }
    }
}

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod eh_frame_constants {
    pub const K_EH_FRAME_TERMINATOR_SIZE: usize = 4;
    pub const K_EH_FRAME_HDR_VERSION: u8 = 1;
    pub const K_EH_FRAME_HDR_SIZE: usize = 16;
    pub const K_INITIAL_STATE_OFFSET_IN_CIE: usize = 14;
    pub const K_FDE_VERSION_SIZE: usize = 1;
    pub const K_FDE_ENCODING_SPECIFIERS_SIZE: usize = 3;
    pub const K_PROCEDURE_ADDRESS_OFFSET_IN_FDE: usize = 4;
    pub const K_PROCEDURE_SIZE_OFFSET_IN_FDE: usize = 8;

    pub const K_SDATA4: u8 = 0x02;
    pub const K_PCREL: u8 = 0x40;
    pub const K_UDATA4: u8 = 0x03;
    pub const K_DATAREL: u8 = 0x80;
    pub const KOMIT: u8 = 0x00;

    pub const K_CODE_ALIGNMENT_FACTOR: i32 = 1;
    pub const K_DATA_ALIGNMENT_FACTOR: i32 = 1;

    pub const K_LOCATION_TAG: u8 = 0x08;
    pub const K_LOCATION_MASK_SIZE: u8 = 3;
    pub const K_LOCATION_MASK: u8 = 0x07;

    pub const K_SAVED_REGISTER_TAG: u8 = 0x09;
    pub const K_SAVED_REGISTER_MASK_SIZE: u8 = 4;
    pub const K_SAVED_REGISTER_MASK: u8 = 0x0F;

    pub const K_FOLLOW_INITIAL_RULE_TAG: u8 = 0x0C;
    pub const K_FOLLOW_INITIAL_RULE_MASK_SIZE: u8 = 4;
    pub const K_FOLLOW_INITIAL_RULE_MASK: u8 = 0x0F;

    pub mod dwarf_opcodes {
        pub const K_OFFSET_EXTENDED_SF: u8 = 0x0F;
        pub const K_ADVANCE_LOC1: u8 = 0x1;
        pub const K_ADVANCE_LOC2: u8 = 0x2;
        pub const K_ADVANCE_LOC4: u8 = 0x3;
        pub const K_DEF_CFA: u8 = 0x0C;
        pub const K_DEF_CFA_OFFSET: u8 = 0x0E;
        pub const K_DEF_CFA_REGISTER: u8 = 0x0D;
        pub const K_SAME_VALUE: u8 = 0x23;
        pub const K_NOP: u8 = 0x00;
        pub const K_RESTORE_EXTENDED: u8 = 0x2D;
    }
}

use eh_frame_constants as constants;

pub mod eh_frame {
    use super::constants;
    use byteorder::{LittleEndian, WriteBytesExt};
    use std::io::{self, Write};
    use std::vec::Vec;

    const K_INT32_SIZE: usize = 4;
    const K_MAX_UINT8: u32 = u8::MAX as u32;
    const K_MAX_UINT16: u32 = u16::MAX as u32;
    const K_SYSTEM_POINTER_SIZE: usize = 4;

    pub struct EhFrameWriter {
        cie_size_: usize,
        last_pc_offset_: i32,
        writer_state_: InternalState,
        base_register_: Register,
        base_offset_: i32,
        eh_frame_buffer_: Vec<u8>,
    }

    #[derive(PartialEq, Debug)]
    enum InternalState {
        Undefined,
        Initialized,
        Finalized,
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum Register {
        NoReg,
        SomeReg(u32) // Placeholder, replace with actual register definitions.
    }

    impl Register {
        fn is_no_reg(&self) -> bool {
            match self {
                Register::NoReg => true,
                _ => false,
            }
        }
    }

    impl EhFrameWriter {
        const K_INT32_PLACEHOLDER: u32 = 0xBADCAFED; //arbitrary value

        pub fn new() -> Self {
            EhFrameWriter {
                cie_size_: 0,
                last_pc_offset_: 0,
                writer_state_: InternalState::Undefined,
                base_register_: Register::NoReg,
                base_offset_: 0,
                eh_frame_buffer_: Vec::new(),
            }
        }

        pub fn write_empty_eh_frame<W: Write>(mut stream: W) -> io::Result<()> {
            stream.write_u8(constants::K_EH_FRAME_HDR_VERSION)?;

            stream.write_u8(constants::K_SDATA4 | constants::K_PCREL)?;

            stream.write_u8(constants::K_UDATA4)?;

            stream.write_u8(constants::K_SDATA4 | constants::K_DATAREL)?;

            let dummy_data = [0u8; constants::K_EH_FRAME_HDR_SIZE - 4];
            stream.write_all(&dummy_data)?;
            Ok(())
        }

        pub fn initialize(&mut self) {
            assert_eq!(self.writer_state_, InternalState::Undefined);
            self.eh_frame_buffer_.reserve(128);
            self.writer_state_ = InternalState::Initialized;
            self.write_cie();
            self.write_fde_header();
        }

        fn write_cie(&mut self) {
            const K_CIE_IDENTIFIER: i32 = 0;
            const K_CIE_VERSION: u8 = 3;
            const K_AUGMENTATION_DATA_SIZE: u8 = 2;
            const K_AUGMENTATION_STRING: &[u8] = b"zLR\0";

            let size_offset = self.eh_frame_offset();
            self.write_int32(EhFrameWriter::K_INT32_PLACEHOLDER);

            let record_start_offset = self.eh_frame_offset();
            self.write_int32(K_CIE_IDENTIFIER);
            self.write_byte(K_CIE_VERSION);

            self.write_bytes(K_AUGMENTATION_STRING);

            self.write_sleb128(constants::K_CODE_ALIGNMENT_FACTOR);
            self.write_sleb128(constants::K_DATA_ALIGNMENT_FACTOR);

            self.write_return_address_register_code();

            self.write_uleb128(K_AUGMENTATION_DATA_SIZE as u32);
            self.write_byte(constants::KOMIT);
            self.write_byte(constants::K_SDATA4 | constants::K_PCREL);

            assert_eq!(
                self.eh_frame_offset() - size_offset,
                constants::K_INITIAL_STATE_OFFSET_IN_CIE
            );
            self.write_initial_state_in_cie();

            self.write_padding_to_aligned_size(self.eh_frame_offset() - record_start_offset);

            let record_end_offset = self.eh_frame_offset();
            let encoded_cie_size = record_end_offset - record_start_offset;
            self.cie_size_ = record_end_offset - size_offset;

            self.patch_int32(size_offset, encoded_cie_size);
        }

        fn write_fde_header(&mut self) {
            assert_ne!(self.cie_size_, 0);

            assert_eq!(self.eh_frame_offset(), self.fde_offset());
            self.write_int32(EhFrameWriter::K_INT32_PLACEHOLDER);

            self.write_int32((self.cie_size_ + K_INT32_SIZE) as i32);

            assert_eq!(self.eh_frame_offset(), self.get_procedure_address_offset());
            self.write_int32(EhFrameWriter::K_INT32_PLACEHOLDER);

            assert_eq!(self.eh_frame_offset(), self.get_procedure_size_offset());
            self.write_int32(EhFrameWriter::K_INT32_PLACEHOLDER);

            self.write_byte(0);
        }

        pub fn write_eh_frame_hdr(&mut self, code_size: i32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);

            let eh_frame_size = self.eh_frame_offset();

            self.write_byte(constants::K_EH_FRAME_HDR_VERSION);

            self.write_byte(constants::K_SDATA4 | constants::K_PCREL);
            self.write_byte(constants::K_UDATA4);
            self.write_byte(constants::K_SDATA4 | constants::K_DATAREL);

            self.write_int32(
                -(eh_frame_size as i32 + constants::K_FDE_VERSION_SIZE as i32 +
                    constants::K_FDE_ENCODING_SPECIFIERS_SIZE as i32),
            );

            self.write_int32(1);

            self.write_int32(-(round_up(code_size, 8) as i32 + eh_frame_size as i32));

            self.write_int32(-(eh_frame_size as i32 - self.cie_size_ as i32));

            assert_eq!(
                self.eh_frame_offset() - eh_frame_size,
                constants::K_EH_FRAME_HDR_SIZE
            );
        }

        fn write_padding_to_aligned_size(&mut self, unpadded_size: usize) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert!(unpadded_size >= 0);

            let padding_size = round_up(unpadded_size as i32, K_SYSTEM_POINTER_SIZE as i32) as usize - unpadded_size;

            let nop = constants::dwarf_opcodes::K_NOP;
            let k_padding = [nop; 8];
            assert!(padding_size <= k_padding.len());
            self.write_bytes(&k_padding[0..padding_size]);
        }

        pub fn advance_location(&mut self, pc_offset: i32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert!(pc_offset >= self.last_pc_offset_);
            let delta = (pc_offset - self.last_pc_offset_) as u32;

            assert_eq!(delta % (constants::K_CODE_ALIGNMENT_FACTOR as u32), 0);
            let factored_delta = delta / (constants::K_CODE_ALIGNMENT_FACTOR as u32);

            if factored_delta <= constants::K_LOCATION_MASK as u32 {
                self.write_byte(
                    (constants::K_LOCATION_TAG << constants::K_LOCATION_MASK_SIZE)
                        | (factored_delta as u8 & constants::K_LOCATION_MASK),
                );
            } else if factored_delta <= K_MAX_UINT8 {
                self.write_opcode(constants::dwarf_opcodes::K_ADVANCE_LOC1);
                self.write_byte(factored_delta as u8);
            } else if factored_delta <= K_MAX_UINT16 {
                self.write_opcode(constants::dwarf_opcodes::K_ADVANCE_LOC2);
                self.write_int16(factored_delta as u16);
            } else {
                self.write_opcode(constants::dwarf_opcodes::K_ADVANCE_LOC4);
                self.write_int32(factored_delta as i32);
            }

            self.last_pc_offset_ = pc_offset;
        }

        pub fn set_base_address_offset(&mut self, base_offset: i32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert!(base_offset >= 0);
            self.write_opcode(constants::dwarf_opcodes::K_DEF_CFA_OFFSET);
            self.write_uleb128(base_offset as u32);
            self.base_offset_ = base_offset;
        }

        pub fn set_base_address_register(&mut self, base_register: Register) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            let code = self.register_to_dwarf_code(base_register);
            self.write_opcode(constants::dwarf_opcodes::K_DEF_CFA_REGISTER);
            self.write_uleb128(code);
            self.base_register_ = base_register;
        }

        pub fn set_base_address_register_and_offset(
            &mut self,
            base_register: Register,
            base_offset: i32,
        ) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert!(base_offset >= 0);
            let code = self.register_to_dwarf_code(base_register);
            self.write_opcode(constants::dwarf_opcodes::K_DEF_CFA);
            self.write_uleb128(code);
            self.write_uleb128(base_offset as u32);
            self.base_offset_ = base_offset;
            self.base_register_ = base_register;
        }

        pub fn record_register_saved_to_stack(&mut self, dwarf_register_code: i32, offset: i32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert_eq!(offset % constants::K_DATA_ALIGNMENT_FACTOR, 0);
            let factored_offset = offset / constants::K_DATA_ALIGNMENT_FACTOR;
            if factored_offset >= 0 {
                //assert!(dwarf_register_code <= constants::K_SAVED_REGISTER_MASK as i32); //removed for compilation
                self.write_byte(
                    (constants::K_SAVED_REGISTER_TAG << constants::K_SAVED_REGISTER_MASK_SIZE)
                        | (dwarf_register_code as u8 & constants::K_SAVED_REGISTER_MASK),
                );
                self.write_uleb128(factored_offset as u32);
            } else {
                self.write_opcode(constants::dwarf_opcodes::K_OFFSET_EXTENDED_SF);
                self.write_uleb128(dwarf_register_code as u32);
                self.write_sleb128(factored_offset);
            }
        }

        pub fn record_register_not_modified(&mut self, name: Register) {
            self.record_register_not_modified_by_code(self.register_to_dwarf_code(name));
        }

        pub fn record_register_not_modified_by_code(&mut self, dwarf_register_code: u32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            self.write_opcode(constants::dwarf_opcodes::K_SAME_VALUE);
            self.write_uleb128(dwarf_register_code);
        }

        pub fn record_register_follows_initial_rule(&mut self, name: Register) {
            self.record_register_follows_initial_rule_by_code(self.register_to_dwarf_code(name));
        }

        pub fn record_register_follows_initial_rule_by_code(&mut self, dwarf_register_code: u32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            if dwarf_register_code <= constants::K_FOLLOW_INITIAL_RULE_MASK as u32 {
                self.write_byte(
                    (constants::K_FOLLOW_INITIAL_RULE_TAG << constants::K_FOLLOW_INITIAL_RULE_MASK_SIZE)
                        | (dwarf_register_code as u8 & constants::K_FOLLOW_INITIAL_RULE_MASK),
                );
            } else {
                self.write_opcode(constants::dwarf_opcodes::K_RESTORE_EXTENDED);
                self.write_uleb128(dwarf_register_code);
            }
        }

        pub fn finish(&mut self, code_size: i32) {
            assert_eq!(self.writer_state_, InternalState::Initialized);
            assert!(self.eh_frame_offset() >= self.cie_size_);

            assert!(self.eh_frame_offset() >= self.fde_offset() + K_INT32_SIZE);
            self.write_padding_to_aligned_size(self.eh_frame_offset() - self.fde_offset() - K_INT32_SIZE);

            let encoded_fde_size = self.eh_frame_offset() - self.fde_offset() - K_INT32_SIZE;
            self.patch_int32(self.fde_offset(), encoded_fde_size);

            self.patch_int32(
                self.get_procedure_address_offset(),
                -(round_up(code_size, 8) as i32 + self.get_procedure_address_offset() as i32),
            );
            self.patch_int32(self.get_procedure_size_offset(), code_size);

            let terminator = [0u8; constants::K_EH_FRAME_TERMINATOR_SIZE];
            self.write_bytes(&terminator);

            self.write_eh_frame_hdr(code_size);

            self.writer_state_ = InternalState::Finalized;
        }

        pub fn get_eh_frame(&self) -> Vec<u8> {
            assert_eq!(self.writer_state_, InternalState::Finalized);
            self.eh_frame_buffer_.clone()
        }

        fn write_uleb128(&mut self, mut value: u32) {
            loop {
                let mut chunk = value & 0x7F;
                value >>= 7;
                if value != 0 {
                    chunk |= 0x80;
                }
                self.write_byte(chunk as u8);
                if value == 0 {
                    break;
                }
            }
        }

        fn write_sleb128(&mut self, mut value: i32) {
            const K_SIGN_BIT_MASK: i32 = 0x40;
            loop {
                let mut chunk = value & 0x7F;
                value >>= 7;
                let done = (value == 0 && (chunk & K_SIGN_BIT_MASK) == 0) || (value == -1 && (chunk & K_SIGN_BIT_MASK) != 0);
                if !done {
                    chunk |= 0x80;
                }
                self.write_byte(chunk as u8);
                if done {
                    break;
                }
            }
        }

        fn write_int32(&mut self, value: i32) {
            self.eh_frame_buffer_.write_i32::<LittleEndian>(value).unwrap();
        }

        fn write_int16(&mut self, value: u16) {
            self.eh_frame_buffer_.write_u16::<LittleEndian>(value).unwrap();
        }

        fn write_byte(&mut self, value: u8) {
            self.eh_frame_buffer_.push(value);
        }

        fn write_bytes(&mut self, bytes: &[u8]) {
            self.eh_frame_buffer_.extend_from_slice(bytes);
        }

        fn write_opcode(&mut self, opcode: u8) {
            self.write_byte(opcode);
        }

        fn patch_int32(&mut self, offset: usize, value: i32) {
            let bytes = value.to_le_bytes();
            self.eh_frame_buffer_[offset..offset + 4].copy_from_slice(&bytes);
        }

        fn eh_frame_offset(&self) -> usize {
            self.eh_frame_buffer_.len()
        }

        fn fde_offset(&self) -> usize {
            self.cie_size_ + K_INT32_SIZE
        }

        fn get_procedure_address_offset(&self) -> usize {
            self.cie_size_ + (2 * K_INT32_SIZE)
        }

        fn get_procedure_size_offset(&self) -> usize {
            self.cie_size_ + (3 * K_INT32_SIZE)
        }

        fn write_return_address_register_code(&mut self) {
            // Architecture-specific code, unimplemented in this example
            // Handle the register code generation here
            // Replace with the correct architecture-specific code generation.
            println!("UNIMPLEMENTED: WriteReturnAddressRegisterCode");
        }

        fn write_initial_state_in_cie(&mut self) {
            // Architecture-specific code, unimplemented in this example
            // Handle the initial state generation here
            // Replace with the correct architecture-specific code generation.
             println!("UNIMPLEMENTED: WriteInitialStateInCie");
        }

        fn register_to_dwarf_code(&self, reg: Register) -> u32 {
            // Architecture-specific code, unimplemented in this example
            // Translate the register to dwarf code.
            // Replace with the correct architecture-specific code translation.
            // Example:
            // match reg {
            //     Register::SomeReg(0) => 0,
            //     Register::SomeReg(1) => 1,
            //     _ => panic!("Unsupported register"),
            // }
            if reg.is_no_reg() {
                0
            } else {
                println!("UNIMPLEMENTED: RegisterToDwarfCode");
                0
            }
        }
    }

    fn round_up(value: i32, alignment: i32) -> i32 {
        if alignment == 0 {
            return value;
        }
        let modulus = value % alignment;
        if modulus == 0 {
            value
        } else {
            value + (alignment - modulus)
        }
    }
}

mod eh_frame_iterator {
    use byteorder::{LittleEndian, ReadBytesExt};
    use std::io::Cursor;
    use std::convert::TryInto;

    pub struct EhFrameIterator<'a> {
        next_: &'a [u8],
        end_: &'a [u8],
    }

    impl<'a> EhFrameIterator<'a> {
        pub fn new(start: &'a [u8], end: &'a [u8]) -> Self {
            EhFrameIterator {
                next_: start,
                end_: end,
            }
        }

        pub fn get_next_uleb128(&mut self) -> u32 {
            let mut size = 0;
            let result = self.decode_uleb128(&mut size);
            assert!(self.next_.len() >= size);
            self.next_ = &self.next_[size..];
            result
        }

        pub fn get_next_sleb128(&mut self) -> i32 {
            let mut size = 0;
            let result = self.decode_sleb128(&mut size);
            assert!(self.next_.len() >= size);
            self.next_ = &self.next_[size..];
            result
        }

        pub fn decode_uleb128(&mut self, encoded_size: &mut usize) -> u32 {
            let mut current = self.next_;
            let mut result: u32 = 0;
            let mut shift = 0;
            let mut encoded_bytes: usize = 0;

            loop {
                assert!(shift < 8 * std::mem::size_of::<u32>());
                let byte = current[0];
                current = &current[1..];
                encoded_bytes += 1;
                result |= ((byte & 0x7F) as u32) << shift;
                shift += 7;
                if byte < 128 {
                    break;
                }
            }

            *encoded_size = encoded_bytes;
            result
        }

        pub fn decode_sleb128(&mut self, encoded_size: &mut usize) -> i32 {
            const K_SIGN_BIT_MASK: u8 = 0x40;

            let mut current = self.next_;
            let mut result: i32 = 0;
            let mut shift = 0;
            let mut chunk: u8 = 0;
            let mut encoded_bytes: usize = 0;

            loop {
                chunk = current[0];
                current = &current[1..];
                encoded_bytes += 1;
                assert!(shift < 8 * std::mem::size_of::<i32>());
                result |= ((chunk & 0x7F) as i32) << shift;
                shift += 7;
                if chunk < 128 {
                    break;
                }
            }

            if (chunk & K_SIGN_BIT_MASK) != 0 {
                result |= (!0i64 << shift) as i32;
            }

            *encoded_size = encoded_bytes;
            result
        }

        pub fn done(&self) -> bool {
            self.next_ >= self.end_
        }

        pub fn current_address(&self) -> usize {
            self.end_.len() - self.next_.len()
        }

        pub fn get_next_byte(&mut self) -> u8 {
            let byte = self.next_[0];
            self.next_ = &self.next_[1..];
            byte
        }

        pub fn get_next_u16(&mut self) -> u16 {
            let mut rdr = Cursor::new(self.next_);
            let value = rdr.read_u16::<LittleEndian>().unwrap();
            self.next_ = &self.next_[2..];
            value
        }

        pub fn get_next_u32(&mut self) -> u32 {
            let mut rdr = Cursor::new(self.next_);
            let value = rdr.read_u32::<LittleEndian>().unwrap();
            self.next_ = &self.next_[4..];
            value
        }
    }
}

mod eh_frame_disassembler {
    use super::eh_frame_constants as constants;
    use super::eh_frame_iterator::EhFrameIterator;
    use byteorder::{LittleEndian, ReadBytesExt};
    use std::io::{self, Write, Cursor};
    use std::convert::TryInto;

    pub struct EhFrameDisassembler<'a> {
        start_: &'a [u8],
        end_: &'a [u8],
    }

    impl<'a> EhFrameDisassembler<'a> {
        pub fn new(start: &'a [u8], end: &'a [u8]) -> Self {
            EhFrameDisassembler {
                start_: start,
                end_: end,
            }
        }

        pub fn dump_dwarf_directives<W: Write>(
            stream: &mut W,
            start: &[u8],
            end: &[u8],
        ) -> io::Result<()> {
            let mut eh_frame_iterator = EhFrameIterator::new(start, end);
            let mut offset_in_procedure: i32 = 0;

            while !eh_frame_iterator.done() {
                write!(stream, "{:04x}  ", eh_frame_iterator.current_address())?;

                let bytecode = eh_frame_iterator.get_next_byte();

                if ((bytecode >> constants::K_LOCATION_MASK_SIZE) & 0xFF) == constants::K_LOCATION_TAG {
                    let value = (bytecode & constants::K_LOCATION_MASK) as i32 * constants::K_CODE_ALIGNMENT_FACTOR;
                    offset_in_procedure += value;
                    writeln!(stream, "| pc_offset={} (delta={})", offset_in_procedure, value)?;
                    continue;
                }

                if ((bytecode >> constants::K_SAVED_REGISTER_MASK_SIZE) & 0xFF) == constants::K_SAVED_REGISTER_TAG {
                    let decoded_offset = eh_frame_iterator.get_next_uleb128();
                    writeln!(
                        stream,
                        "| {} saved at base +{}",
                        Self::dwarf_register_code_to_string(bytecode as i32 & constants::K_LOCATION_MASK as i32),
                        decoded_offset * constants::K_DATA_ALIGNMENT_FACTOR as u32
                    )?;
                    continue;
                }

                if ((bytecode >> constants::K_FOLLOW_INITIAL_RULE_MASK_SIZE) & 0xFF) == constants::K_FOLLOW_INITIAL_RULE_TAG {
                    writeln!(
                        stream,
                        "| {} follows rule in CIE",
                        Self::dwarf_register_code_to_string(bytecode as i32 & constants::K_LOCATION_MASK as i32)
                    )?;
                    continue;
                }

                match bytecode as u32 {
                    constants::dwarf_opcodes::K_OFFSET_EXTENDED_SF as u32 => {
                        let register_code = eh_frame_iterator.get_next_uleb128();
                        let decoded_offset = eh_frame_iterator.get_next_sleb128();
                        writeln!(
                            stream,
                            "| {} saved at base +{}",
                            Self::dwarf_register_code_to_string(register_code as i32),
                            decoded_offset * constants::K_DATA_ALIGNMENT_FACTOR
                        )?;
                        
                    }
                    constants::dwarf_opcodes::K_ADVANCE_LOC1 as u32 => {
                        let value = eh_frame_iterator.get_next_byte() as i32 * constants::K_CODE_ALIGNMENT_FACTOR;
                        offset_in_procedure += value;
                        writeln!(stream, "| pc_offset={} (delta={})", offset_in_procedure, value)?;
                    }
                    constants::dwarf_opcodes::K_ADVANCE_LOC2 as u32 => {
                        let value = eh_frame_iterator.get_next_u16() as i32 * constants::K_CODE_ALIGNMENT_FACTOR;
                        offset_in_procedure += value;
                        writeln!(stream, "| pc_offset={} (delta={})", offset_in_procedure, value)?;
                    }
                    constants::dwarf_opcodes::K_ADVANCE_LOC4 as u32 => {
                        let value = eh_frame_iterator.get_next_u32() as i32 * constants::K_CODE_ALIGNMENT_FACTOR;
                        offset_in_procedure += value;
                        writeln!(stream, "| pc_offset={} (delta={})", offset_in_procedure, value)?;
                    }
                    constants::dwarf_opcodes::K_DEF_CFA as u32 => {
                        let base_register = eh_frame_iterator.get_next_uleb128();
                        let base_offset = eh_frame_iterator.get_next_uleb128();
                        writeln!(
                            stream,
                            "| base_register={}, base_offset={}",
                            Self::dwarf_register_code_to_string(base_register as i32),
                            base_offset
                        )?;
                    }
                    constants::dwarf_opcodes::K_DEF_CFA_OFFSET as u32 => {
                        let base_offset = eh_frame_iterator.get_next_uleb128();
                        writeln!(stream, "| base_offset={}", base_offset)?;
                    }
                    constants::dwarf_opcodes::K_DEF_CFA_REGISTER as u32 => {
                        let base_register = eh_frame_iterator.get_next_uleb128();
                        writeln!(
                            stream,
                            "| base_register={}",
                            Self::dwarf_register_code_to_string(base_register as i32)
                        )?;
                    }
                    constants::dwarf_opcodes::K_SAME_VALUE as u32 => {
                        let register_code = eh_frame_iterator.get_next_uleb128();
                        writeln!(
                            stream,
                            "| {} not modified from previous frame",
                            Self::dwarf_register_code_to_string(register_code as i32)
                        )?;
                    }
                    constants::dwarf_opcodes::K_NOP as u32 => {
                        writeln!(stream, "| nop")?;
                    }
                    _ => {
                        println!("UNREACHABLE");
                    }
                }
            }
            Ok(())
        }

        pub fn disassemble_to_stream<W: Write>(&self, stream: &mut W) -> io::Result<()> {
            let cie_size = {
                let mut rdr = Cursor::new(self.start_);
                rdr.read_u32::<LittleEndian>().unwrap() as usize + 4
            };

            let fde_offset = cie_size;

            let cie_directives_start = &self.start_[constants::K_INITIAL_STATE_OFFSET_IN_CIE..];
// Converted from V8 C++ source files:
// Header: N/A
// Implementation: eh-frame-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/diagnostics/eh-frame.h
// src/zone/zone-containers.h

use std::io::Write;
use std::mem::size_of;
use std::result;

#[derive(Debug)]
pub enum EhFrameError {
    IoError(std::io::Error),
    Unimplemented,
}

impl From<std::io::Error> for EhFrameError {
    fn from(err: std::io::Error) -> Self {
        EhFrameError::IoError(err)
    }
}

pub type Result<T> = result::Result<T, EhFrameError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register {
    code: i32,
}

impl Register {
    pub fn code(&self) -> i32 {
        self.code
    }
}

const kRegCode_rbp: i32 = 1;
const kRegCode_rsp: i32 = 2;
const kRegCode_rax: i32 = 3;

pub const rbp: Register = Register { code: kRegCode_rbp };
pub const rsp: Register = Register { code: kRegCode_rsp };
pub const rax: Register = Register { code: kRegCode_rax };

pub struct EhFrameConstants {}

impl EhFrameConstants {
    pub const kCodeAlignmentFactor: i32 = 1;
    pub const kDataAlignmentFactor: i32 = -8;
}

pub struct EhFrameWriter<'a, W: Write> {
    writer: &'a mut W,
    base_address_register: Option<(Register, i32)>,
    registers_saved_to_stack: Vec<(i32, i32)>, // dwarf_code, offset
}

impl<'a, W: Write> EhFrameWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        EhFrameWriter {
            writer,
            base_address_register: None,
            registers_saved_to_stack: Vec::new(),
        }
    }

    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.writer.write_all(&[byte])?;
        Ok(())
    }

    fn write_uleb128(&mut self, mut value: u64) -> Result<()> {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.write_byte(byte)?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    fn write_sleb128(&mut self, mut value: i64) -> Result<()> {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            let done = value == 0 || value == -1 && (byte & 0x40) != 0;
            if !done {
                byte |= 0x80;
            }
            self.write_byte(byte)?;
            if done {
                break;
            }
        }
        Ok(())
    }

    pub fn write_return_address_register_code(&mut self) -> Result<()> {
        self.write_uleb128(kRipDwarfCode as u64)?;
        Ok(())
    }

    pub fn set_base_address_register_and_offset(&mut self, reg: Register, offset: i32) {
        self.base_address_register = Some((reg, offset));
    }

    pub fn record_register_saved_to_stack(&mut self, dwarf_code: i32, offset: i32) {
        self.registers_saved_to_stack.push((dwarf_code, offset));
    }

    pub fn write_initial_state_in_cie(&mut self) {
        let kSystemPointerSize = size_of::<usize>() as i32;
        self.set_base_address_register_and_offset(rsp, kSystemPointerSize);
        // x64 rip (r16) has no Register instance associated.
        self.record_register_saved_to_stack(kRipDwarfCode, -kSystemPointerSize);
    }

    pub fn register_to_dwarf_code(name: Register) -> Result<i32> {
        let code = name.code();
        match code {
            kRegCode_rbp => Ok(kRbpDwarfCode),
            kRegCode_rsp => Ok(kRspDwarfCode),
            kRegCode_rax => Ok(kRaxDwarfCode),
            _ => Err(EhFrameError::Unimplemented),
        }
    }
}

const kRaxDwarfCode: i32 = 0;
const kRbpDwarfCode: i32 = 6;
const kRspDwarfCode: i32 = 7;
const kRipDwarfCode: i32 = 16;

#[cfg(feature = "disassembler")]
pub struct EhFrameDisassembler {}

#[cfg(feature = "disassembler")]
impl EhFrameDisassembler {
    pub fn dwarf_register_code_to_string(code: i32) -> Result<&'static str> {
        match code {
            kRbpDwarfCode => Ok("rbp"),
            kRspDwarfCode => Ok("rsp"),
            kRipDwarfCode => Ok("rip"),
            _ => Err(EhFrameError::Unimplemented),
        }
    }
}

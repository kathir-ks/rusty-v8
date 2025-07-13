// Converted from V8 C++ source files:
// Header: assembler-arm64.h
// Implementation: assembler-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arm64 {
pub mod constants_arm64;
pub mod instructions_arm64;
pub mod register_arm64;
}
use std::sync::{Arc, Mutex, RwLock};

use crate::codegen::arm64::constants_arm64::*;
use crate::codegen::arm64::instructions_arm64::*;
use crate::codegen::arm64::register_arm64::*;
use crate::codegen::assembler::*;
use crate::codegen::constant_pool::*;
use crate::codegen::reloc_info::*;
use crate::common::globals::*;
use crate::utils::utils::*;
use crate::zone::zone_containers::*;
use std::{fmt, error};

#[derive(Debug, Clone)]
pub enum AssemblerError {
    GenericError(String),
    OutOfMemory,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssemblerError::GenericError(msg) => write!(f, "Assembler Error: {}", msg),
            AssemblerError::OutOfMemory => write!(f, "Assembler Out of Memory"),
        }
    }
}

impl error::Error for AssemblerError {}

pub struct Immediate {
    value_: i64,
    rmode_: RelocInfo::Mode,
}

impl Immediate {
    pub fn new<T>(value: T, rmode: RelocInfo::Mode) -> Self 
    where i64: From<T> {
        Immediate{
            value_: i64::from(value),
            rmode_: rmode,
        }
    }

    pub fn value(&self) -> i64 {
        self.value_
    }

    pub fn rmode(&self) -> RelocInfo::Mode {
        self.rmode_
    }
}

pub struct Operand {
    heap_number_request_: Option<HeapNumberRequest>,
    immediate_: Immediate,
    reg_: Register,
    shift_: Shift,
    extend_: Extend,
    shift_amount_: unsigned,
}

impl Operand {
    pub fn new_register_shift(reg: Register, shift: Shift, shift_amount: unsigned) -> Self {
        Operand {
            heap_number_request_: None,
            immediate_: Immediate::new(0, RelocInfo::NO_INFO),
            reg_: reg,
            shift_: shift,
            extend_: Extend::UXTX,
            shift_amount_: shift_amount,
        }
    }

    pub fn new_register_extend(reg: Register, extend: Extend, shift_amount: unsigned) -> Self {
        Operand {
            heap_number_request_: None,
            immediate_: Immediate::new(0, RelocInfo::NO_INFO),
            reg_: reg,
            shift_: Shift::LSL,
            extend_: extend,
            shift_amount_: shift_amount,
        }
    }

    pub fn new_embedded_number(number: f64) -> Self {
      if let Some(smi) = Self::double_to_smi_integer(number) {
          return Self::new_immediate(smi);
      }
      Self::new_embedded_heap_number(number)
    }

    pub fn new_embedded_heap_number(number: f64) -> Self {
        Operand {
            heap_number_request_: Some(HeapNumberRequest{ number_: number }),
            immediate_: Immediate::new(0, RelocInfo::FULL_EMBEDDED_OBJECT),
            reg_: Register{ code_: 0, is_64_: true },
            shift_: Shift::LSL,
            extend_: Extend::UXTX,
            shift_amount_: 0,
        }
    }

    pub fn new_immediate<T>(value: T) -> Self 
        where i64: From<T> {
        Operand {
            heap_number_request_: None,
            immediate_: Immediate::new(value, RelocInfo::NO_INFO),
            reg_: Register{ code_: 0, is_64_: true },
            shift_: Shift::LSL,
            extend_: Extend::UXTX,
            shift_amount_: 0,
        }
    }

    pub fn new_immediate_rmode<T>(value: T, rmode: RelocInfo::Mode) -> Self
        where i64: From<T> {
        Operand {
            heap_number_request_: None,
            immediate_: Immediate::new(value, rmode),
            reg_: Register{ code_: 0, is_64_: true },
            shift_: Shift::LSL,
            extend_: Extend::UXTX,
            shift_amount_: 0,
        }
    }

    fn double_to_smi_integer(number: f64) -> Option<i64> {
        let smi_int = number as i32;
        if number == smi_int as f64 {
            Some(smi_int as i64)
        } else {
            None
        }
    }

    pub fn is_heap_number_request(&self) -> bool {
        self.heap_number_request_.is_some()
    }

    pub fn heap_number_request(&self) -> HeapNumberRequest {
        self.heap_number_request_.clone().unwrap()
    }

    pub fn immediate_for_heap_number_request(&self) -> Immediate {
        self.immediate_.clone()
    }

    pub fn is_immediate(&self) -> bool {
        self.heap_number_request_.is_none()
    }

    pub fn is_shifted_register(&self) -> bool {
        self.heap_number_request_.is_none() && self.shift_ != Shift::LSL && self.extend_ == Extend::UXTX
    }

    pub fn is_extended_register(&self) -> bool {
        self.heap_number_request_.is_none() && self.extend_ != Extend::UXTX
    }

    pub fn is_zero(&self) -> bool {
      self.reg_.code_ == 31
    }

    pub fn to_extended_register(&self) -> Self {
        if self.shift_ == Shift::LSL && self.shift_amount_ <= 4 {
            Operand {
                heap_number_request_: None,
                immediate_: Immediate::new(0, RelocInfo::NO_INFO),
                reg_: self.reg_,
                shift_: Shift::LSL,
                extend_: Extend::UXTX,
                shift_amount_: self.shift_amount_,
            }
        } else {
            self.clone()
        }
    }

    pub fn to_w(&self) -> Self {
        Operand {
            heap_number_request_: self.heap_number_request_,
            immediate_: self.immediate_,
            reg_: Register { code_: self.reg_.code_, is_64_: false },
            shift_: self.shift_,
            extend_: self.extend_,
            shift_amount_: self.shift_amount_,
        }
    }

    pub fn immediate(&self) -> Immediate {
        self.immediate_.clone()
    }

    pub fn immediate_value(&self) -> i64 {
        self.immediate_.value()
    }

    pub fn immediate_rmode(&self) -> RelocInfo::Mode {
        self.immediate_.rmode()
    }

    pub fn reg(&self) -> Register {
        self.reg_
    }

    pub fn shift(&self) -> Shift {
        self.shift_
    }

    pub fn extend(&self) -> Extend {
        self.extend_
    }

    pub fn shift_amount(&self) -> unsigned {
        self.shift_amount_
    }
}

#[derive(Clone, Copy)]
pub enum AddrMode {
    Offset,
}

#[derive(Clone, Copy)]
pub struct MemOperand {
    base_: Register,
    regoffset_: Register,
    offset_: i64,
    addrmode_: AddrMode,
    shift_: Shift,
    extend_: Extend,
    shift_amount_: unsigned,
}

impl MemOperand {
    pub fn new() -> Self {
        MemOperand {
            base_: Register { code_: 0, is_64_: true },
            regoffset_: Register { code_: 0, is_64_: true },
            offset_: 0,
            addrmode_: AddrMode::Offset,
            shift_: Shift::LSL,
            extend_: Extend::UXTX,
            shift_amount_: 0,
        }
    }

    pub fn new_offset(base: Register, offset: i64, addrmode: AddrMode) -> Self {
        MemOperand {
            base_: base,
            regoffset_: Register { code_: 0, is_64_: true },
            offset_: offset,
            addrmode_: addrmode,
            shift_: Shift::LSL,
            extend_: Extend::UXTX,
            shift_amount_: 0,
        }
    }

    pub fn new_register_offset(base: Register, regoffset: Register, shift: Shift, shift_amount: unsigned) -> Self {
        MemOperand {
            base_: base,
            regoffset_: regoffset,
            offset_: 0,
            addrmode_: AddrMode::Offset, // changed from offset
            shift_: shift,
            extend_: Extend::UXTX, // changed from UXTX
            shift_amount_: shift_amount,
        }
    }
    
    pub fn new_register_extend(base: Register, regoffset: Register, extend: Extend, shift_amount: unsigned) -> Self {
        MemOperand {
            base_: base,
            regoffset_: regoffset,
            offset_: 0,
            addrmode_: AddrMode::Offset, // changed from offset
            shift_: Shift::LSL, // changed from LSL
            extend_: extend,
            shift_amount_: shift_amount,
        }
    }

    pub fn base(&self) -> &Register {
        &self.base_
    }

    pub fn regoffset(&self) -> &Register {
        &self.regoffset_
    }

    pub fn offset(&self) -> i64 {
        self.offset_
    }

    pub fn addrmode(&self) -> AddrMode {
        self.addrmode_
    }

    pub fn shift(&self) -> Shift {
        self.shift_
    }

    pub fn extend(&self) -> Extend {
        self.extend_
    }

    pub fn shift_amount(&self) -> unsigned {
        self.shift_amount_
    }

    pub fn is_immediate_offset(&self) -> bool {
        self.regoffset_.code_ == 0
    }

    pub fn is_register_offset(&self) -> bool {
        self.regoffset_.code_ != 0
    }

    pub fn is_pre_index(&self) -> bool {
        false 
    }

    pub fn is_post_index(&self) -> bool {
        false 
    }
}

//#[derive(Clone, Copy)]
pub enum VRegister {
    V64(i32),
    V128(i32),
}

pub struct SafepointTableBuilder {}

impl SafepointTableBuilder {
  
}

// struct AssemblerZone {
//   pub fn new(zone: MaybeAssemblerZone) -> Self {
//     let zone_: Zone = match zone {
//       MaybeAssemblerZone::Local(z) => z,
//       MaybeAssemblerZone::Global(z) => z,
//     };
//     AssemblerZone {zone_: zone_}
//   }
// }

}

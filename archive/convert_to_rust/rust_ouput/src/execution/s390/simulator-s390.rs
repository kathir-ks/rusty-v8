// Converted from V8 C++ source files:
// Header: simulator-s390.h
// Implementation: simulator-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::sync::Mutex;
use std::{cmp, f64, i32, i64, u32, u64};

// globals.h defines USE_SIMULATOR.
// #include "src/common/globals.h"

// Declares a Simulator for S390 instructions if we are not generating a native
// S390 binary. This Simulator allows us to run and debug S390 code generation
// on regular desktop machines.
// V8 calls into generated code via the GeneratedCode wrapper,
// which will start execution in the Simulator or forwards to the real entry
// on a S390 hardware platform.

// globals.h defines USE_SIMULATOR.
// #include "src/common/globals.h"

// #if defined(USE_SIMULATOR)
// Running with a simulator.

// #include "src/base/hashmap.h"
// #include "src/codegen/assembler.h"
// #include "src/codegen/s390/constants-s390.h"
// #include "src/execution/simulator-base.h"
// #include "src/utils/allocation.h"

// namespace heap::base {
// class StackVisitor;
// }

// namespace v8 {
// namespace internal {
pub struct CachePage {
    validity_map_: [i8; CachePage::K_VALIDITY_MAP_SIZE],
    data_: [u8; CachePage::K_PAGE_SIZE],
}

impl CachePage {
    pub const LINE_VALID: i32 = 0;
    pub const LINE_INVALID: i32 = 1;

    pub const K_PAGE_SHIFT: i32 = 12;
    pub const K_PAGE_SIZE: usize = 1 << Self::K_PAGE_SHIFT;
    pub const K_PAGE_MASK: usize = Self::K_PAGE_SIZE - 1;
    pub const K_LINE_SHIFT: i32 = 2; // The cache line is only 4 bytes right now.
    pub const K_LINE_LENGTH: i32 = 1 << Self::K_LINE_SHIFT;
    pub const K_LINE_MASK: i32 = Self::K_LINE_LENGTH - 1;
    pub const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;

    pub fn new() -> Self {
        CachePage {
            validity_map_: [Self::LINE_INVALID as i8; Self::K_VALIDITY_MAP_SIZE],
            data_: [0; Self::K_PAGE_SIZE],
        }
    }

    pub fn validity_byte(&mut self, offset: i32) -> &mut i8 {
        &mut self.validity_map_[(offset >> Self::K_LINE_SHIFT) as usize]
    }

    pub fn cached_data(&mut self, offset: i32) -> &mut u8 {
        &mut self.data_[offset as usize]
    }
}

pub enum RoundingMode {
  ROUND_TO_NEAREST_AWAY_FROM_0,
  ROUND_TO_NEAREST_TO_EVEN,
  ROUND_TOWARD_0,
  ROUND_TOWARD_POS_INF,
  ROUND_TOWARD_NEG_INF,
}

pub fn compute_rounding<T>(a: T, mode: RoundingMode) -> T
where
    T: std::fmt::Debug + Copy + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match mode {
      RoundingMode::ROUND_TO_NEAREST_AWAY_FROM_0 => {
        let a_f64 = format!("{:?}", a).parse::<f64>().unwrap();
        a_f64.round() as T
      }
      RoundingMode::ROUND_TO_NEAREST_TO_EVEN => {
        let a_f64 = format!("{:?}", a).parse::<f64>().unwrap();
        a_f64.floor() as T
      }
      RoundingMode::ROUND_TOWARD_0 => {
        let a_f64 = format!("{:?}", a).parse::<f64>().unwrap();
        a_f64.trunc() as T
      }
      RoundingMode::ROUND_TOWARD_POS_INF => {
        let a_f64 = format!("{:?}", a).parse::<f64>().unwrap();
        a_f64.ceil() as T
      }
      RoundingMode::ROUND_TOWARD_NEG_INF => {
        let a_f64 = format!("{:?}", a).parse::<f64>().unwrap();
        a_f64.floor() as T
      }
    }
}

#[derive(Debug)]
pub enum SimulatorError {}

#[derive(Debug, Clone)]
pub struct Instr {}

use std::sync::Arc;
pub struct SimulatorBase {}
pub struct Redirection {}
pub struct Isolate {}
pub struct HeapObject {}
pub struct StackHandler {}
pub struct String {}
pub struct NativeContext {}
pub struct SourceLocation {}
pub struct HeapHandle {}
pub struct StringView {}
pub struct Object {}
pub struct MicrotaskQueue {}
use std::collections::HashMap;
pub struct RootVisitor {}
pub struct ArrayBuffer {}
pub struct ProfilerId {}
pub struct CpuProfile {}
pub struct GCType {}
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct HeapObject {}
pub struct StackHandler {}
pub struct String_ExternalOneByteStringResource {}
pub struct Context {}
pub struct Value {}
pub struct Local<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}
pub struct MaybeLocal<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}
pub struct CFunction {}
pub struct Smi {}
pub struct IsolateData {}
pub struct RWDigits {}
pub struct Digits {}
pub struct ArrayList {}
pub struct FeedbackVector {}
pub struct DirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct Heap {}
pub struct Instruction {
  bits_: u32,
}

impl Instruction {
  pub fn InstructionBits(&self) -> u32 {
    self.bits_
  }
  pub fn SetInstructionBits(&mut self, bit: u32) {
    self.bits_ = bit;
  }
  pub fn S390OpcodeValue(&self) -> i32 {
    0
  }
  pub fn InstructionLength(&self) -> i32 {
    0
  }
  pub fn SvcValue(&self) -> i32 {
    0
  }
  pub fn Bits(&self, start: i32, end: i32) -> u32 {
    0
  }
}
pub struct FourByteInstr {}
pub struct SixByteInstr {}
pub struct RILInstruction {}
impl RILInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> i64 {
        0
    }
    pub fn I2UnsignedValue(&self) -> u32 {
        0
    }
}
pub struct RXYInstruction {}
impl RXYInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn X2Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
}
pub struct RXInstruction {}
impl RXInstruction {
    pub fn X2Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
}
pub struct RSInstruction {}
impl RSInstruction {
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
}
pub struct RSIInstruction {}
impl RSIInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> i32 {
        0
    }
}
pub struct SIInstruction {}
impl SIInstruction {
    pub fn B1Value(&self) -> i32 {
        0
    }
    pub fn D1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> u8 {
        0
    }
}
pub struct SILInstruction {}
impl SILInstruction {
    pub fn B1Value(&self) -> i32 {
        0
    }
    pub fn D1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> i16 {
        0
    }
}
pub struct SIYInstruction {}
impl SIYInstruction {
    pub fn B1Value(&self) -> i32 {
        0
    }
    pub fn D1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> u8 {
        0
    }
}
pub struct RREInstruction {}
impl RREInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn M3Value(&self) -> i32 {
        0
    }
}
pub struct RRDInstruction {}
impl RRDInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
}
pub struct RRFInstruction {}
impl RRFInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn M3Value(&self) -> i32 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
}
pub struct RRInstruction {}
impl RRInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
}
pub struct RIEInstruction {}
impl RIEInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn I6Value(&self) -> i32 {
        0
    }
    pub fn I3Value(&self) -> u32 {
        0
    }
    pub fn I4Value(&self) -> u32 {
        0
    }
    pub fn I5Value(&self) -> u32 {
        0
    }
}
pub struct RSYInstruction {}
impl RSYInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
}
pub struct RIInstruction {}
impl RIInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> i16 {
        0
    }
}

pub struct RXEInstruction {}
impl RXEInstruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn X2Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
}

pub struct VRR_A_Instruction {}
impl VRR_A_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn M5Value(&self) -> i32 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
    pub fn M3Value(&self) -> i32 {
        0
    }
}

pub struct VRR_B_Instruction {}
impl VRR_B_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn M5Value(&self) -> i32 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
}

pub struct VRR_C_Instruction {}
impl VRR_C_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn M6Value(&self) -> i32 {
        0
    }
    pub fn M5Value(&self) -> i32 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
}

pub struct VRR_E_Instruction {}
impl VRR_E_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn R4Value(&self) -> i32 {
        0
    }
    pub fn M6Value(&self) -> i32 {
        0
    }
    pub fn M5Value(&self) -> i32 {
        0
    }
}

pub struct VRR_F_Instruction {}
impl VRR_F_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R2Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
}

pub struct VRX_Instruction {}
impl VRX_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn X2Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
    pub fn M3Value(&self) -> i32 {
        0
    }
}

pub struct VRS_Instruction {}
impl VRS_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn B2Value(&self) -> i32 {
        0
    }
    pub fn D2Value(&self) -> i32 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
}

pub struct VRI_A_Instruction {}
impl VRI_A_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> i16 {
        0
    }
    pub fn M3Value(&self) -> i32 {
        0
    }
}

pub struct VRI_C_Instruction {}
impl VRI_C_Instruction {
    pub fn R1Value(&self) -> i32 {
        0
    }
    pub fn R3Value(&self) -> i32 {
        0
    }
    pub fn I2Value(&self) -> u16 {
        0
    }
    pub fn M4Value(&self) -> i32 {
        0
    }
}

pub struct SSInstruction {
  length_: i32,
  b1_: i32,
  b2_: i32,
  d1_: i32,
  d2_: i32,
}

impl SSInstruction {
  pub fn Length(&self) -> i32 {
    self.length_
  }
  pub fn B1Value(&self) -> i32 {
    self.b1_
  }
  pub fn B2Value(&self) -> i32 {
    self.b2_
  }
  pub fn D1Value(&self) -> i32 {
    self.d1_
  }
  pub fn D2Value(&self) -> i32 {
    self.d2_
  }
}
pub struct Address {}
pub struct v8 {}
pub struct ObjectPair {
    x: i64,
    y: i64
}


// Converted from V8 C++ source files:
// Header: frame-constants-arm.h
// Implementation: frame-constants-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::sync::atomic::AtomicU16;

pub struct Address {}
pub struct String {}
pub struct Object {}
pub struct StackFrame {}
pub struct Isolate {}
pub struct Simulator {}
pub struct Tagged {}
pub struct BuiltinCode {}
pub struct String_ExternalOneByteStringResource {}

const kSystemPointerSize: i32 = 4;
const kDoubleSize: i32 = 8;

const kNumDoubleCalleeSaved: i32 = 8;
const kNumCalleeSaved: i32 = 8;

macro_rules! TYPED_FRAME_PUSHED_VALUE_OFFSET {
    ($x:expr) => {
        -((3 + $x) * kSystemPointerSize)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
    cp,
    fp,
    lr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleRegister {
    d0,
    d1,
    d2,
    d3,
    d4,
    d5,
    d6,
    d7,
    d8,
    d9,
    d10,
    d11,
    d12,
    d13,
    d14,
    d15,
}

pub struct RegList {
    registers: Vec<Register>,
}

impl RegList {
    pub const fn new(registers: Vec<Register>) -> Self {
        Self { registers }
    }

    pub fn bits(&self) -> u32 {
        let mut bits: u32 = 0;
        for reg in &self.registers {
            bits |= 1 << *reg as usize;
        }
        bits
    }

    pub fn Count(&self) -> usize {
        self.registers.len()
    }
}

pub struct DoubleRegList {
    registers: Vec<DoubleRegister>,
}

impl DoubleRegList {
    pub const fn new(registers: Vec<DoubleRegister>) -> Self {
        Self { registers }
    }

    pub fn bits(&self) -> u32 {
        let mut bits: u32 = 0;
        for reg in &self.registers {
            bits |= 1 << *reg as usize;
        }
        bits
    }

    pub fn Count(&self) -> usize {
        self.registers.len()
    }
}

pub trait AllStatic {}

pub struct EntryFrameConstants {}
impl AllStatic for EntryFrameConstants {}

impl EntryFrameConstants {
    pub const kNextExitFrameFPOffset: i32 = -3 * kSystemPointerSize;
    pub const kNextFastCallFrameFPOffset: i32 =
        EntryFrameConstants::kNextExitFrameFPOffset - kSystemPointerSize;
    pub const kNextFastCallFramePCOffset: i32 =
        EntryFrameConstants::kNextFastCallFrameFPOffset - kSystemPointerSize;
    pub const kArgcOffset: i32 = +0 * kSystemPointerSize;
    pub const kArgvOffset: i32 = +1 * kSystemPointerSize;
    pub const kDirectCallerFPOffset: i32 = 0;
    pub const kDirectCallerPCOffset: i32 =
        EntryFrameConstants::kDirectCallerFPOffset + 1 * kSystemPointerSize;
    pub const kDirectCallerGeneralRegistersOffset: i32 =
        EntryFrameConstants::kDirectCallerPCOffset
            + kSystemPointerSize
            + kNumDoubleCalleeSaved * kDoubleSize;
    pub const kDirectCallerSPOffset: i32 = EntryFrameConstants::kDirectCallerGeneralRegistersOffset
        + (kNumCalleeSaved - 1) * kSystemPointerSize;
}

pub trait TypedFrameConstants {}

pub struct WasmLiftoffSetupFrameConstants {}
impl TypedFrameConstants for WasmLiftoffSetupFrameConstants {}

impl WasmLiftoffSetupFrameConstants {
    pub const kNumberOfSavedGpParamRegs: i32 = 3;
    pub const kNumberOfSavedFpParamRegs: i32 = 8;
    pub const kInstanceSpillOffset: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET!(2);
    pub const kParameterSpillsOffset: [i32; 3] = [
        TYPED_FRAME_PUSHED_VALUE_OFFSET!(4),
        TYPED_FRAME_PUSHED_VALUE_OFFSET!(3),
        TYPED_FRAME_PUSHED_VALUE_OFFSET!(1),
    ];
    pub const kWasmInstanceDataOffset: i32 = 2 * kSystemPointerSize;
    pub const kDeclaredFunctionIndexOffset: i32 = 1 * kSystemPointerSize;
    pub const kNativeModuleOffset: i32 = 0;
}

pub struct WasmLiftoffFrameConstants {}
impl TypedFrameConstants for WasmLiftoffFrameConstants {}

impl WasmLiftoffFrameConstants {
    pub const kFeedbackVectorOffset: i32 = 3 * kSystemPointerSize;
    pub const kInstanceDataOffset: i32 = 2 * kSystemPointerSize;
}

pub struct WasmDebugBreakFrameConstants {}
impl TypedFrameConstants for WasmDebugBreakFrameConstants {}

impl WasmDebugBreakFrameConstants {
    pub const kPushedGpRegs: RegList = RegList::new(vec![
        Register::r0,
        Register::r1,
        Register::r2,
        Register::r3,
        Register::r4,
        Register::r5,
        Register::r6,
        Register::r7,
        Register::r8,
        Register::r9,
    ]);
    pub const kPushedFpRegs: DoubleRegList = DoubleRegList::new(vec![
        DoubleRegister::d0,
        DoubleRegister::d1,
        DoubleRegister::d2,
        DoubleRegister::d3,
        DoubleRegister::d4,
        DoubleRegister::d5,
        DoubleRegister::d6,
        DoubleRegister::d7,
        DoubleRegister::d8,
        DoubleRegister::d9,
        DoubleRegister::d10,
        DoubleRegister::d11,
        DoubleRegister::d12,
    ]);

    pub const kNumPushedGpRegisters: usize = WasmDebugBreakFrameConstants::kPushedGpRegs.Count();
    pub const kNumPushedFpRegisters: usize = WasmDebugBreakFrameConstants::kPushedFpRegs.Count();

    pub const kLastPushedGpRegisterOffset: i32 =
        -Self::kFixedFrameSizeFromFp() - kSystemPointerSize * (Self::kNumPushedGpRegisters as i32);
    pub const kLastPushedFpRegisterOffset: i32 = Self::kLastPushedGpRegisterOffset
        - kDoubleSize * (Self::kNumPushedFpRegisters as i32);

    pub fn GetPushedGpRegisterOffset(reg_code: i32) -> i32 {
        if WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & (1 << reg_code) == 0 {
            panic!("DCHECK failed: reg_code not in kPushedGpRegs");
        }
        let lower_regs =
            WasmDebugBreakFrameConstants::kPushedGpRegs.bits() & ((1 << reg_code) - 1);
        Self::kLastPushedGpRegisterOffset
            + lower_regs.count_ones() as i32 * kSystemPointerSize
    }

    pub fn GetPushedFpRegisterOffset(reg_code: i32) -> i32 {
        if WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & (1 << reg_code) == 0 {
            panic!("DCHECK failed: reg_code not in kPushedFpRegs");
        }
        let lower_regs =
            WasmDebugBreakFrameConstants::kPushedFpRegs.bits() & ((1 << reg_code) - 1);
        Self::kLastPushedFpRegisterOffset + lower_regs.count_ones() as i32 * kDoubleSize
    }

    const fn kFixedFrameSizeFromFp() -> i32 {
        3 * kSystemPointerSize
    }
}

pub struct JavaScriptFrame {}

impl JavaScriptFrame {
    pub fn fp_register() -> Register {
        Register::fp
    }
    pub fn context_register() -> Register {
        Register::cp
    }
    pub fn constant_pool_pointer_register() -> Register {
        panic!("UNREACHABLE");
    }
}

pub struct UnoptimizedFrameConstants {}

impl UnoptimizedFrameConstants {
    pub fn RegisterStackSlotCount(register_count: i32) -> i32 {
        register_count
    }
}

pub struct BuiltinContinuationFrameConstants {}

impl BuiltinContinuationFrameConstants {
    pub fn PaddingSlotCount(register_count: i32) -> i32 {
        let _ = register_count;
        0
    }
}

pub struct MaglevFrame {}

impl MaglevFrame {
    pub fn StackGuardFrameSize(_register_input_count: i32) -> i32 {
        panic!("UNREACHABLE");
    }
}

// Converted from V8 C++ source files:
// Header: register-s390.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::sync::Arc;

  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  pub enum RegisterCode {
    kRegCode_r0,
    kRegCode_r1,
    kRegCode_r2,
    kRegCode_r3,
    kRegCode_r4,
    kRegCode_r5,
    kRegCode_r6,
    kRegCode_r7,
    kRegCode_r8,
    kRegCode_r9,
    kRegCode_r10,
    kRegCode_fp,
    kRegCode_ip,
    kRegCode_r13,
    kRegCode_r14,
    kRegCode_sp,
    kRegAfterLast,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  pub enum DoubleRegisterCode {
    kDoubleCode_d0,
    kDoubleCode_d1,
    kDoubleCode_d2,
    kDoubleCode_d3,
    kDoubleCode_d4,
    kDoubleCode_d5,
    kDoubleCode_d6,
    kDoubleCode_d7,
    kDoubleCode_d8,
    kDoubleCode_d9,
    kDoubleCode_d10,
    kDoubleCode_d11,
    kDoubleCode_d12,
    kDoubleCode_d13,
    kDoubleCode_d14,
    kDoubleCode_d15,
    kDoubleAfterLast,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  pub enum CRegisterCode {
    kCCode_cr0,
    kCCode_cr1,
    kCCode_cr2,
    kCCode_cr3,
    kCCode_cr4,
    kCCode_cr5,
    kCCode_cr6,
    kCCode_cr7,
    kCCode_cr8,
    kCCode_cr9,
    kCCode_cr10,
    kCCode_cr11,
    kCCode_cr12,
    kCCode_cr15,
    kCAfterLast,
  }

  pub const kNumRequiredStackFrameSlots: i32 = 20;
  pub const kStackFrameRASlot: i32 = 14;
  pub const kStackFrameSPSlot: i32 = 15;
  pub const kStackFrameExtraParamSlot: i32 = 20;
  pub const kCalleeRegisterSaveAreaSize: i32 = 160;
  pub const kRegisterPassedArguments: usize = 5;
  pub const kSimdMaskRegisters: bool = false;

  pub enum AliasingKind {
    kNoAlias,
    kMayAlias,
    kOverlap,
  }

  pub struct Register(i32);

  impl Register {
    pub const kMantissaOffset: i32 = 4;
    pub const kExponentOffset: i32 = 0;

    pub fn from_code(code: RegisterCode) -> Self {
      Register(code as i32)
    }

    pub fn no_reg() -> Self {
      Register(-1)
    }
  }

  pub struct DoubleRegister(i32);

  impl DoubleRegister {
    pub const kSizeInBytes: i32 = 8;

    pub fn from_code(code: DoubleRegisterCode) -> Self {
      DoubleRegister(code as i32)
    }

    pub fn no_reg() -> Self {
      DoubleRegister(-1)
    }
    pub fn SupportedRegisterCount() -> i32 {
        32
    }
  }

  pub struct CRegister(i32);

  impl CRegister {
    pub fn from_code(code: CRegisterCode) -> Self {
      CRegister(code as i32)
    }

    pub fn no_reg() -> Self {
      CRegister(-1)
    }
  }

  macro_rules! define_registers {
    ($define_register:ident, $register_type:ident, $($reg:ident),*) => {
      $(
        pub fn $reg(&self) -> bool {
          self.0 == RegisterCode::kRegCode_$reg as i32
        }
      )*
    };
  }

  impl Register {
    define_registers!(define_register_names, Register, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, fp, ip, r13, r14, sp);
  }
    impl DoubleRegister {
    define_registers!(define_register_names, DoubleRegister, d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15);
  }

  pub fn ReassignRegister(source: &mut Register) -> Register {
    let result = *source;
    *source = Register::no_reg();
    result
  }

  pub const r0: Register = Register(RegisterCode::kRegCode_r0 as i32);
  pub const r1: Register = Register(RegisterCode::kRegCode_r1 as i32);
  pub const r2: Register = Register(RegisterCode::kRegCode_r2 as i32);
  pub const r3: Register = Register(RegisterCode::kRegCode_r3 as i32);
  pub const r4: Register = Register(RegisterCode::kRegCode_r4 as i32);
  pub const r5: Register = Register(RegisterCode::kRegCode_r5 as i32);
  pub const r6: Register = Register(RegisterCode::kRegCode_r6 as i32);
  pub const r7: Register = Register(RegisterCode::kRegCode_r7 as i32);
  pub const r8: Register = Register(RegisterCode::kRegCode_r8 as i32);
  pub const r9: Register = Register(RegisterCode::kRegCode_r9 as i32);
  pub const r10: Register = Register(RegisterCode::kRegCode_r10 as i32);
  pub const fp: Register = Register(RegisterCode::kRegCode_fp as i32);
  pub const ip: Register = Register(RegisterCode::kRegCode_ip as i32);
  pub const r13: Register = Register(RegisterCode::kRegCode_r13 as i32);
  pub const r14: Register = Register(RegisterCode::kRegCode_r14 as i32);
  pub const sp: Register = Register(RegisterCode::kRegCode_sp as i32);
  pub const no_reg: Register = Register(-1);

  pub const kRootRegister: Register = r10;
  pub const kPtrComprCageBaseRegister: Register = r9;
  pub const cp: Register = r13;

  pub const kCArgRegs: [Register; 5] = [r2, r3, r4, r5, r6];

  pub const d0: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d0 as i32);
  pub const d1: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d1 as i32);
  pub const d2: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d2 as i32);
  pub const d3: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d3 as i32);
  pub const d4: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d4 as i32);
  pub const d5: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d5 as i32);
  pub const d6: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d6 as i32);
  pub const d7: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d7 as i32);
  pub const d8: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d8 as i32);
  pub const d9: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d9 as i32);
  pub const d10: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d10 as i32);
  pub const d11: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d11 as i32);
  pub const d12: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d12 as i32);
  pub const d13: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d13 as i32);
  pub const d14: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d14 as i32);
  pub const d15: DoubleRegister = DoubleRegister(DoubleRegisterCode::kDoubleCode_d15 as i32);
  pub const no_dreg: DoubleRegister = DoubleRegister(-1);

  pub const kDoubleRegZero: DoubleRegister = d14;
  pub const kScratchDoubleReg: DoubleRegister = d13;

  pub type FloatRegister = DoubleRegister;
  pub type Simd128Register = DoubleRegister;

    pub const cr0: CRegister = CRegister(CRegisterCode::kCCode_cr0 as i32);
    pub const cr1: CRegister = CRegister(CRegisterCode::kCCode_cr1 as i32);
    pub const cr2: CRegister = CRegister(CRegisterCode::kCCode_cr2 as i32);
    pub const cr3: CRegister = CRegister(CRegisterCode::kCCode_cr3 as i32);
    pub const cr4: CRegister = CRegister(CRegisterCode::kCCode_cr4 as i32);
    pub const cr5: CRegister = CRegister(CRegisterCode::kCCode_cr5 as i32);
    pub const cr6: CRegister = CRegister(CRegisterCode::kCCode_cr6 as i32);
    pub const cr7: CRegister = CRegister(CRegisterCode::kCCode_cr7 as i32);
    pub const cr8: CRegister = CRegister(CRegisterCode::kCCode_cr8 as i32);
    pub const cr9: CRegister = CRegister(CRegisterCode::kCCode_cr9 as i32);
    pub const cr10: CRegister = CRegister(CRegisterCode::kCCode_cr10 as i32);
    pub const cr11: CRegister = CRegister(CRegisterCode::kCCode_cr11 as i32);
    pub const cr12: CRegister = CRegister(CRegisterCode::kCCode_cr12 as i32);
    pub const cr15: CRegister = CRegister(CRegisterCode::kCCode_cr15 as i32);
    pub const no_creg: CRegister = CRegister(-1);


  pub fn ArgumentPaddingSlots(argument_count: i32) -> i32 {
    0
  }

  pub const kStackPointerRegister: Register = sp;
  pub const kReturnRegister0: Register = r2;
  pub const kReturnRegister1: Register = r3;
  pub const kReturnRegister2: Register = r4;
  pub const kJSFunctionRegister: Register = r3;
  pub const kContextRegister: Register = r13;
  pub const kAllocateSizeRegister: Register = r3;
  pub const kInterpreterAccumulatorRegister: Register = r2;
  pub const kInterpreterBytecodeOffsetRegister: Register = r6;
  pub const kInterpreterBytecodeArrayRegister: Register = r7;
  pub const kInterpreterDispatchTableRegister: Register = r8;
  pub const kJavaScriptCallArgCountRegister: Register = r2;
  pub const kJavaScriptCallCodeStartRegister: Register = r4;
  pub const kJavaScriptCallTargetRegister: Register = kJSFunctionRegister;
  pub const kJavaScriptCallNewTargetRegister: Register = r5;
  pub const kJavaScriptCallExtraArg1Register: Register = r4;
  pub const kJavaScriptCallDispatchHandleRegister: Register = no_reg;
  pub const kRuntimeCallFunctionRegister: Register = r3;
  pub const kRuntimeCallArgCountRegister: Register = r2;
  pub const kRuntimeCallArgvRegister: Register = r4;
  pub const kWasmImplicitArgRegister: Register = r6;
  pub const kWasmCompileLazyFuncIndexRegister: Register = r7;
  pub const kFPReturnRegister0: DoubleRegister = d0;

    pub fn ToRegister(num: i32) -> Register {
        match num {
            0 => r0,
            1 => r1,
            2 => r2,
            3 => r3,
            4 => r4,
            5 => r5,
            6 => r6,
            7 => r7,
            8 => r8,
            9 => r9,
            10 => r10,
            11 => fp,
            12 => ip,
            13 => r13,
            14 => r14,
            15 => sp,
            _ => no_reg,
        }
    }

}

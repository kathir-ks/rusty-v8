// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod liftoff_varstate {
    use crate::wasm::baseline::liftoff_register::LiftoffRegister;
    use crate::wasm::wasm_value::WasmValue;
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Location {
        Stack,
        Register,
        IntConst,
    }

    #[derive(Clone, Copy)]
    pub struct LiftoffVarState {
        loc: Location,
        kind: ValueKind,
        data: LiftoffVarStateData,
        spill_offset: i32,
    }

    #[derive(Clone, Copy)]
    union LiftoffVarStateData {
        reg: LiftoffRegister,
        i32_const: i32,
    }

    impl LiftoffVarState {
        pub fn new_stack(kind: ValueKind, offset: i32) -> Self {
            assert!(offset >= 0);
            LiftoffVarState {
                loc: Location::Stack,
                kind,
                data: LiftoffVarStateData { i32_const: 0 }, // Dummy value
                spill_offset: offset,
            }
        }

        pub fn new_register(kind: ValueKind, r: LiftoffRegister, offset: i32) -> Self {
            assert_eq!(r.reg_class(), reg_class_for(kind));
            assert!(offset >= 0);
            LiftoffVarState {
                loc: Location::Register,
                kind,
                data: LiftoffVarStateData { reg: r },
                spill_offset: offset,
            }
        }

        pub fn new_int_const(kind: ValueKind, i32_const: i32, offset: i32) -> Self {
            assert!(kind == ValueKind::I32 || kind == ValueKind::I64);
            assert!(offset >= 0);
            LiftoffVarState {
                loc: Location::IntConst,
                kind,
                data: LiftoffVarStateData { i32_const },
                spill_offset: offset,
            }
        }

        pub fn is_stack(&self) -> bool {
            self.loc == Location::Stack
        }

        pub fn is_gp_reg(&self) -> bool {
            self.loc == Location::Register && self.reg().is_gp()
        }

        pub fn is_fp_reg(&self) -> bool {
            self.loc == Location::Register && self.reg().is_fp()
        }

        pub fn is_gp_reg_pair(&self) -> bool {
            self.loc == Location::Register && self.reg().is_gp_pair()
        }

        pub fn is_fp_reg_pair(&self) -> bool {
            self.loc == Location::Register && self.reg().is_fp_pair()
        }

        pub fn is_reg(&self) -> bool {
            self.loc == Location::Register
        }

        pub fn is_const(&self) -> bool {
            self.loc == Location::IntConst
        }

        pub fn kind(&self) -> ValueKind {
            self.kind
        }

        pub fn loc(&self) -> Location {
            self.loc
        }

        pub fn i32_const(&self) -> i32 {
            assert_eq!(self.loc, Location::IntConst);
            unsafe { self.data.i32_const }
        }

        pub fn constant(&self) -> WasmValue {
            assert!(self.kind == ValueKind::I32 || self.kind == ValueKind::I64);
            assert_eq!(self.loc, Location::IntConst);
            if self.kind == ValueKind::I32 {
                WasmValue::I32(unsafe { self.data.i32_const })
            } else {
                WasmValue::I64(unsafe { self.data.i32_const as i64 })
            }
        }

        pub fn offset(&self) -> i32 {
            assert!(self.spill_offset >= 0);
            self.spill_offset
        }

        pub fn set_offset(&mut self, offset: i32) {
            assert!(self.spill_offset >= 0);
            self.spill_offset = offset;
        }

        pub fn gp_reg(&self) -> Register {
            self.reg().gp()
        }

        pub fn fp_reg(&self) -> DoubleRegister {
            self.reg().fp()
        }

        pub fn reg(&self) -> LiftoffRegister {
            assert_eq!(self.loc, Location::Register);
            unsafe { self.data.reg }
        }

        pub fn reg_class(&self) -> RegClass {
            self.reg().reg_class()
        }

        pub fn make_stack(&mut self) {
            self.loc = Location::Stack;
        }

        pub fn make_register(&mut self, r: LiftoffRegister) {
            self.loc = Location::Register;
            self.data.reg = r;
        }

        pub fn make_constant(&mut self, i32_const: i32) {
            assert!(self.kind == ValueKind::I32 || self.kind == ValueKind::I64);
            self.loc = Location::IntConst;
            self.data.i32_const = i32_const;
        }

        pub fn copy(&mut self, src: LiftoffVarState) {
            self.loc = src.loc();
            self.kind = src.kind();
            match self.loc {
                Location::Register => self.data.reg = src.reg(),
                Location::IntConst => self.data.i32_const = src.i32_const(),
                _ => {}
            }
        }
    }

    impl fmt::Display for LiftoffVarState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "LiftoffVarState {{ loc: {:?}, kind: {:?}, spill_offset: {} }}", self.loc, self.kind, self.spill_offset)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ValueKind {
        I32,
        I64,
        F32,
        F64,
        V128,
        Ref,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RegClass {
        NoReg, // Invalid RegClass value.
        GpReg, // General-purpose register.
        FpReg, // Floating-point register.
        GpRegPair, // Register pair for int64.
        FpRegPair, // Register pair for float64.
        Invalid,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Register {
        // Placeholder.  Implementation details would go here.
        code: u8
    }

    impl Register {
      pub fn from_code(code: u8) -> Self {
        Register { code }
      }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DoubleRegister {
      code: u8
    }
    impl DoubleRegister{
      pub fn from_code(code: u8) -> Self {
        DoubleRegister { code }
      }
    }

    // Stub functions for other modules.  These are needed to compile.
    pub fn reg_class_for(kind: ValueKind) -> RegClass {
        match kind {
            ValueKind::I32 | ValueKind::I64 => RegClass::GpReg,
            ValueKind::F32 | ValueKind::F64 => RegClass::FpReg,
            _ => RegClass::NoReg,
        }
    }
}

mod wasm {
  pub mod wasm_value {
      #[derive(Debug, Clone, Copy, PartialEq)]
      pub enum WasmValue {
          I32(i32),
          I64(i64),
          F32(f32),
          F64(f64),
      }
  }

  pub mod baseline {
    pub mod liftoff_register {
      use crate::liftoff_varstate::RegClass;
      use crate::liftoff_varstate::Register;
      use crate::liftoff_varstate::DoubleRegister;

      #[derive(Clone, Copy)]
      pub struct LiftoffRegister {
          reg_class: RegClass,
          gp_code: u8,
          fp_code: u8,
      }

      impl LiftoffRegister {
          pub fn new(reg_class: RegClass, gp_code: u8, fp_code: u8) -> Self {
              LiftoffRegister { reg_class, gp_code, fp_code }
          }

          pub fn reg_class(&self) -> RegClass {
              self.reg_class
          }

          pub fn gp(&self) -> Register {
            Register::from_code(self.gp_code)
          }

          pub fn fp(&self) -> DoubleRegister {
            DoubleRegister::from_code(self.fp_code)
          }

          pub fn is_gp(&self) -> bool {
              self.reg_class == RegClass::GpReg
          }

          pub fn is_fp(&self) -> bool {
              self.reg_class == RegClass::FpReg
          }

          pub fn is_gp_pair(&self) -> bool {
            self.reg_class == RegClass::GpRegPair
          }

          pub fn is_fp_pair(&self) -> bool {
            self.reg_class == RegClass::FpRegPair
          }
      }
    }
  }
}
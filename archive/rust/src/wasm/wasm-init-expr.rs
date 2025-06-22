// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

// TODO: Add a feature flag for enabling WebAssembly
// #[cfg(not(feature = "enable_webassembly"))]
// compile_error!("This module should only be included if WebAssembly is enabled.");

use std::mem;
use std::sync::Arc;

//use crate::wasm::value_type::ValueType;
//use crate::zone::zone::Zone;

pub mod value_type {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ValueTypeKind {
        I8,
        I16,
        I32,
        I64,
        F16,
        F32,
        F64,
        RefNull,
        S128,
        Void,
        Top,
        Bottom,
        Ref,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ValueType {
        kind: ValueTypeKind,
        heap_type: HeapType
    }

    impl ValueType {
        pub fn kind(&self) -> ValueTypeKind {
            self.kind
        }

        pub fn heap_type(&self) -> HeapType {
            self.heap_type
        }
    }

    impl ValueType {
      pub fn new(kind: ValueTypeKind, heap_type: HeapType) -> Self {
        ValueType { kind, heap_type }
      }
    }
}

pub mod zone {
  pub struct Zone {}
  impl Zone {
      pub fn new() -> Self {
          Zone {}
      }

      pub fn alloc<T>(&self, value: T) -> Box<T> {
          Box::new(value)
      }

      pub fn new_vec<T>(&self, values: Vec<T>) -> Box<Vec<T>> {
        Box::new(values)
      }
  }
}

pub mod wasm {
    use super::*;
    use std::array;
    use std::vec::Vec;
    use std::mem::MaybeUninit;
    use std::fmt;

    const kSimd128Size: usize = 16;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum HeapType {
      kHeapTypeI31 = 0,
      kHeapTypeAny = 1,
      kHeapTypeEq = 2,
      kHeapTypeStruct = 3,
      kHeapTypeArray = 4,
      kHeapTypeString = 5,
      kHeapTypeI8 = 6,
      kHeapTypeI16 = 7,
      kHeapTypeBottom = 8,
      kHeapTypeLast = 8,
    }

    impl HeapType {
      pub fn raw_bit_field(&self) -> u32 {
        *self as u32
      }

      pub fn from_bits(bits: u32) -> Self {
        match bits {
          0 => HeapType::kHeapTypeI31,
          1 => HeapType::kHeapTypeAny,
          2 => HeapType::kHeapTypeEq,
          3 => HeapType::kHeapTypeStruct,
          4 => HeapType::kHeapTypeArray,
          5 => HeapType::kHeapTypeString,
          6 => HeapType::kHeapTypeI8,
          7 => HeapType::kHeapTypeI16,
          8 => HeapType::kHeapTypeBottom,
          _ => panic!("Invalid heap type bits"),
        }
      }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleTypeIndex {
        pub index: u32,
    }

    // TODO: Define WasmModule struct

    #[derive(Debug, Copy, Clone)]
    pub enum Operator {
        kGlobalGet,
        kI32Const,
        kI64Const,
        kF32Const,
        kF64Const,
        kS128Const,
        kI32Add,
        kI32Sub,
        kI32Mul,
        kI64Add,
        kI64Sub,
        kI64Mul,
        kRefNullConst,
        kRefFuncConst,
        kStructNew,
        kStructNewDefault,
        kArrayNew,
        kArrayNewDefault,
        kArrayNewFixed,
        kRefI31,
        kStringConst,
        kAnyConvertExtern,
        kExternConvertAny,
    }

    #[derive(Debug, Copy, Clone)]
    pub union Immediate {
        i32_const: i32,
        i64_const: i64,
        f32_const: f32,
        f64_const: f64,
        s128_const: [u8; kSimd128Size],
        index: u32,
        heap_type_: u32, // Read with {heap_type()}.
    }

    impl Immediate {
        pub fn new() -> Immediate {
            Immediate { i32_const: 0 } // Initialize with a default value
        }
    }

    #[derive(Debug, Clone)]
    pub struct WasmInitExpr {
        kind_: Operator,
        operands_: Option<Arc<Vec<WasmInitExpr>>>,
        immediate_: Immediate,
    }

    impl WasmInitExpr {
        pub fn new_i32(v: i32) -> Self {
            let mut immediate = Immediate::new();
            unsafe {
                immediate.i32_const = v;
            }
            WasmInitExpr {
                kind_: Operator::kI32Const,
                operands_: None,
                immediate_: immediate,
            }
        }

        pub fn new_i64(v: i64) -> Self {
            let mut immediate = Immediate::new();
            unsafe {
                immediate.i64_const = v;
            }
            WasmInitExpr {
                kind_: Operator::kI64Const,
                operands_: None,
                immediate_: immediate,
            }
        }

        pub fn new_f32(v: f32) -> Self {
            let mut immediate = Immediate::new();
            unsafe {
                immediate.f32_const = v;
            }
            WasmInitExpr {
                kind_: Operator::kF32Const,
                operands_: None,
                immediate_: immediate,
            }
        }

        pub fn new_f64(v: f64) -> Self {
            let mut immediate = Immediate::new();
            unsafe {
                immediate.f64_const = v;
            }
            WasmInitExpr {
                kind_: Operator::kF64Const,
                operands_: None,
                immediate_: immediate,
            }
        }

        pub fn new_s128(v: [u8; kSimd128Size]) -> Self {
            let mut immediate = Immediate::new();
            unsafe {
                immediate.s128_const = v;
            }
            WasmInitExpr {
                kind_: Operator::kS128Const,
                operands_: None,
                immediate_: immediate,
            }
        }

        pub fn heap_type(&self) -> HeapType {
            unsafe {
                HeapType::from_bits(self.immediate_.heap_type_)
            }
        }

        pub fn binop(zone: &zone::Zone, op: Operator, lhs: WasmInitExpr, rhs: WasmInitExpr) -> Self {
            match op {
                Operator::kI32Add | Operator::kI32Sub | Operator::kI32Mul | Operator::kI64Add |
                Operator::kI64Sub | Operator::kI64Mul => {
                    WasmInitExpr::new_with_operands(zone, op, vec![lhs, rhs])
                }
                _ => panic!("Invalid binary operator"),
            }
        }

        pub fn global_get(index: u32) -> Self {
            let mut expr = WasmInitExpr::new(Operator::kGlobalGet);
            unsafe {
                expr.immediate_.index = index;
            }
            expr
        }

        pub fn ref_func_const(index: u32) -> Self {
            let mut expr = WasmInitExpr::new(Operator::kRefFuncConst);
            unsafe {
                expr.immediate_.index = index;
            }
            expr
        }

        pub fn ref_null_const(heap_type: HeapType) -> Self {
            let mut expr = WasmInitExpr::new(Operator::kRefNullConst);
            unsafe {
                expr.immediate_.heap_type_ = heap_type.raw_bit_field();
            }
            expr
        }

        pub fn struct_new(index: ModuleTypeIndex, elements: &mut Vec<WasmInitExpr>) -> Self {
            let mut expr = WasmInitExpr::new_with_operands_no_zone(Operator::kStructNew, elements.to_vec());
            unsafe {
                expr.immediate_.index = index.index;
            }
            expr
        }

        pub fn struct_new_default(index: ModuleTypeIndex) -> Self {
            let mut expr = WasmInitExpr::new(Operator::kStructNewDefault);
            unsafe {
                expr.immediate_.index = index.index;
            }
            expr
        }

        pub fn array_new(zone: &zone::Zone, index: ModuleTypeIndex, initial: WasmInitExpr, length: WasmInitExpr) -> Self {
            let mut expr = WasmInitExpr::new_with_operands(zone, Operator::kArrayNew, vec![initial, length]);
            unsafe {
                expr.immediate_.index = index.index;
            }
            expr
        }

        pub fn array_new_default(zone: &zone::Zone, index: ModuleTypeIndex, length: WasmInitExpr) -> Self {
            let mut expr = WasmInitExpr::new_with_operands(zone, Operator::kArrayNewDefault, vec![length]);
            unsafe {
                expr.immediate_.index = index.index;
            }
            expr
        }

        pub fn array_new_fixed(index: ModuleTypeIndex, elements: &mut Vec<WasmInitExpr>) -> Self {
            let mut expr = WasmInitExpr::new_with_operands_no_zone(Operator::kArrayNewFixed, elements.to_vec());
            unsafe {
                expr.immediate_.index = index.index;
            }
            expr
        }

        pub fn ref_i31(zone: &zone::Zone, value: WasmInitExpr) -> Self {
            WasmInitExpr::new_with_operands(zone, Operator::kRefI31, vec![value])
        }

        pub fn string_const(index: u32) -> Self {
            let mut expr = WasmInitExpr::new(Operator::kStringConst);
            unsafe {
                expr.immediate_.index = index;
            }
            expr
        }

        pub fn any_convert_extern(zone: &zone::Zone, arg: WasmInitExpr) -> Self {
            WasmInitExpr::new_with_operands(zone, Operator::kAnyConvertExtern, vec![arg])
        }

        pub fn extern_convert_any(zone: &zone::Zone, arg: WasmInitExpr) -> Self {
            WasmInitExpr::new_with_operands(zone, Operator::kExternConvertAny, vec![arg])
        }

        pub fn immediate(&self) -> Immediate {
            self.immediate_
        }

        pub fn kind(&self) -> Operator {
            self.kind_
        }

        pub fn operands(&self) -> Option<&Arc<Vec<WasmInitExpr>>> {
            self.operands_.as_ref()
        }

        fn new(kind: Operator) -> Self {
            WasmInitExpr {
                kind_: kind,
                operands_: None,
                immediate_: Immediate::new(),
            }
        }

        fn new_with_operands(zone: &zone::Zone, kind: Operator, operands: Vec<WasmInitExpr>) -> Self {
            WasmInitExpr {
                kind_: kind,
                operands_: Some(Arc::new(operands)),
                immediate_: Immediate::new(),
            }
        }

        fn new_with_operands_no_zone(kind: Operator, operands: Vec<WasmInitExpr>) -> Self {
          WasmInitExpr {
              kind_: kind,
              operands_: Some(Arc::new(operands)),
              immediate_: Immediate::new(),
          }
      }

        pub fn default_value(type_: value_type::ValueType) -> Self {
            match type_.kind() {
                value_type::ValueTypeKind::I8 | value_type::ValueTypeKind::I16 | value_type::ValueTypeKind::I32 => {
                    WasmInitExpr::new_i32(0)
                }
                value_type::ValueTypeKind::I64 => {
                    WasmInitExpr::new_i64(0)
                }
                value_type::ValueTypeKind::F16 | value_type::ValueTypeKind::F32 => {
                    WasmInitExpr::new_f32(0.0)
                }
                value_type::ValueTypeKind::F64 => {
                    WasmInitExpr::new_f64(0.0)
                }
                value_type::ValueTypeKind::RefNull => {
                    WasmInitExpr::ref_null_const(type_.heap_type())
                }
                value_type::ValueTypeKind::S128 => {
                    WasmInitExpr::new_s128([0; kSimd128Size])
                }
                value_type::ValueTypeKind::Void | value_type::ValueTypeKind::Top | value_type::ValueTypeKind::Bottom | value_type::ValueTypeKind::Ref => {
                    panic!("UNREACHABLE");
                }
            }
        }
    }

    impl PartialEq for WasmInitExpr {
        fn eq(&self, other: &Self) -> bool {
            if self.kind() != other.kind() {
                return false;
            }
            match self.kind() {
                Operator::kGlobalGet | Operator::kRefFuncConst | Operator::kStringConst => {
                    unsafe { self.immediate().index == other.immediate().index }
                }
                Operator::kI32Const => {
                    unsafe { self.immediate().i32_const == other.immediate().i32_const }
                }
                Operator::kI64Const => {
                    unsafe { self.immediate().i64_const == other.immediate().i64_const }
                }
                Operator::kF32Const => {
                    unsafe { self.immediate().f32_const == other.immediate().f32_const }
                }
                Operator::kF64Const => {
                    unsafe { self.immediate().f64_const == other.immediate().f64_const }
                }
                Operator::kI32Add | Operator::kI32Sub | Operator::kI32Mul | Operator::kI64Add |
                Operator::kI64Sub | Operator::kI64Mul => {
                    if let (Some(self_ops), Some(other_ops)) = (self.operands(), other.operands()) {
                        self_ops[0] == other_ops[0] && self_ops[1] == other_ops[1]
                    } else {
                        false
                    }
                }
                Operator::kS128Const => {
                    unsafe { self.immediate().s128_const == other.immediate().s128_const }
                }
                Operator::kRefNullConst => {
                    self.heap_type() == other.heap_type()
                }
                Operator::kStructNew | Operator::kStructNewDefault | Operator::kArrayNew |
                Operator::kArrayNewDefault => {
                    unsafe {
                        if self.immediate().index != other.immediate().index {
                            return false;
                        }
                    }

                    if let (Some(self_ops), Some(other_ops)) = (self.operands(), other.operands()) {
                        if self_ops.len() != other_ops.len() {
                            return false;
                        }
                        for i in 0..self_ops.len() {
                            if self_ops[i] != other_ops[i] {
                                return false;
                            }
                        }
                        true
                    } else {
                        false
                    }
                }
                Operator::kArrayNewFixed => {
                    unsafe {
                        if self.immediate().index != other.immediate().index {
                            return false;
                        }
                    }

                    if let (Some(self_ops), Some(other_ops)) = (self.operands(), other.operands()) {
                        if self_ops.len() != other_ops.len() {
                            return false;
                        }
                        for i in 0..self_ops.len() {
                            if self_ops[i] != other_ops[i] {
                                return false;
                            }
                        }
                        true
                    } else {
                        false
                    }
                }
                Operator::kRefI31 | Operator::kAnyConvertExtern | Operator::kExternConvertAny => {
                    if let (Some(self_ops), Some(other_ops)) = (self.operands(), other.operands()) {
                        self_ops[0] == other_ops[0]
                    } else {
                        false
                    }
                }
            }
        }
    }

    impl Eq for WasmInitExpr {}
}
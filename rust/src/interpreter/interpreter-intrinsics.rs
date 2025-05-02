// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/interpreter/interpreter_intrinsics.rs

// This file translates the C++ code from
// /home/kathirks_gc/v8_go/codebase/src/interpreter/interpreter-intrinsics.cc
// to Rust.

// The original C++ code uses a macro `INTRINSICS_LIST`. Since we don't have
// the definition of that macro, we'll define a dummy list here for
// demonstration purposes.  In a real conversion, you'd need to
// replicate the functionality of that macro.
macro_rules! intrinsics_list {
    ($callback:ident) => {
        $callback!(Add, add, 2);
        $callback!(Sub, sub, 2);
        $callback!(Mul, mul, 2);
    };
}

// Dummy Runtime::FunctionId enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionId {
    None,
    InlineAdd,
    InlineSub,
    InlineMul,
}

// Dummy implementation of Runtime module
pub mod runtime {
    use super::FunctionId;
    pub const K_INLINE_ADD: FunctionId = FunctionId::InlineAdd;
    pub const K_INLINE_SUB: FunctionId = FunctionId::InlineSub;
    pub const K_INLINE_MUL: FunctionId = FunctionId::InlineMul;
}

pub mod interpreter {

    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IntrinsicId {
        Add,
        Sub,
        Mul,
    }

    pub struct IntrinsicsHelper {}

    impl IntrinsicsHelper {
        pub fn is_supported(function_id: FunctionId) -> bool {
            match function_id {
                FunctionId::InlineAdd | FunctionId::InlineSub | FunctionId::InlineMul => true,
                _ => false,
            }
        }

        pub fn from_runtime_id(function_id: FunctionId) -> IntrinsicId {
            match function_id {
                FunctionId::InlineAdd => IntrinsicId::Add,
                FunctionId::InlineSub => IntrinsicId::Sub,
                FunctionId::InlineMul => IntrinsicId::Mul,
                _ => panic!("UNREACHABLE"),
            }
        }

        pub fn to_runtime_id(intrinsic_id: IntrinsicId) -> FunctionId {
            match intrinsic_id {
                IntrinsicId::Add => FunctionId::InlineAdd,
                IntrinsicId::Sub => FunctionId::InlineSub,
                IntrinsicId::Mul => FunctionId::InlineMul,
            }
        }
    }
}
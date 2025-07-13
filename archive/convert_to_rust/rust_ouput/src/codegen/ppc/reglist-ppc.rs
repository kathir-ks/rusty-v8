// Converted from V8 C++ source files:
// Header: reglist-ppc.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

use std::convert::TryInto;

use crate::codegen::register_arch::Register;
use crate::codegen::reglist_base::RegListBase;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DoubleRegister {
    code: i32,
}

impl DoubleRegister {
    pub fn new(code: i32) -> Self {
        DoubleRegister { code }
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Simd128Register {
    code: i32,
}

impl Simd128Register {
    pub fn new(code: i32) -> Self {
        Simd128Register { code }
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

pub type RegList = RegListBase<Register>;
pub type DoubleRegList = RegListBase<DoubleRegister>;
pub type Simd128RegList = RegListBase<Simd128Register>;

// Caller-saved/arguments registers
lazy_static::lazy_static! {
    pub static ref kJSCallerSaved: RegList = RegList::from_registers(&[
        Register { code: 3 },  // a1
        Register { code: 4 },  // a2
        Register { code: 5 },  // a3
        Register { code: 6 },  // a4
        Register { code: 7 },  // a5
        Register { code: 8 },  // a6
        Register { code: 9 },  // a7
        Register { code: 10 }, // a8
        Register { code: 11 },
    ]);

    pub static ref kCalleeSaved: RegList = RegList::from_registers(&[
        Register { code: 14 },
        Register { code: 15 },
        Register { code: 16 },
        Register { code: 17 },
        Register { code: 18 },
        Register { code: 19 },
        Register { code: 20 },
        Register { code: 21 },
        Register { code: 22 },
        Register { code: 23 },
        Register { code: 24 },
        Register { code: 25 },
        Register { code: 26 },
        Register { code: 27 },
        Register { code: 28 },
        Register { code: 29 },
        Register { code: 30 },
        Register { code: 31 },
    ]);

    pub static ref kCallerSavedDoubles: DoubleRegList = DoubleRegList::from_registers(&[
        DoubleRegister { code: 0 },
        DoubleRegister { code: 1 },
        DoubleRegister { code: 2 },
        DoubleRegister { code: 3 },
        DoubleRegister { code: 4 },
        DoubleRegister { code: 5 },
        DoubleRegister { code: 6 },
        DoubleRegister { code: 7 },
        DoubleRegister { code: 8 },
        DoubleRegister { code: 9 },
        DoubleRegister { code: 10 },
        DoubleRegister { code: 11 },
        DoubleRegister { code: 12 },
        DoubleRegister { code: 13 },
    ]);

    pub static ref kCallerSavedSimd128s: Simd128RegList = Simd128RegList::from_registers(&[
        Simd128Register { code: 0 },
        Simd128Register { code: 1 },
        Simd128Register { code: 2 },
        Simd128Register { code: 3 },
        Simd128Register { code: 4 },
        Simd128Register { code: 5 },
        Simd128Register { code: 6 },
        Simd128Register { code: 7 },
        Simd128Register { code: 8 },
        Simd128Register { code: 9 },
        Simd128Register { code: 10 },
        Simd128Register { code: 11 },
        Simd128Register { code: 12 },
        Simd128Register { code: 13 },
        Simd128Register { code: 14 },
        Simd128Register { code: 15 },
        Simd128Register { code: 16 },
        Simd128Register { code: 17 },
        Simd128Register { code: 18 },
        Simd128Register { code: 19 },
    ]);

    pub static ref kCalleeSavedDoubles: DoubleRegList = DoubleRegList::from_registers(&[
        DoubleRegister { code: 14 },
        DoubleRegister { code: 15 },
        DoubleRegister { code: 16 },
        DoubleRegister { code: 17 },
        DoubleRegister { code: 18 },
        DoubleRegister { code: 19 },
        DoubleRegister { code: 20 },
        DoubleRegister { code: 21 },
        DoubleRegister { code: 22 },
        DoubleRegister { code: 23 },
        DoubleRegister { code: 24 },
        DoubleRegister { code: 25 },
        DoubleRegister { code: 26 },
        DoubleRegister { code: 27 },
        DoubleRegister { code: 28 },
        DoubleRegister { code: 29 },
        DoubleRegister { code: 30 },
        DoubleRegister { code: 31 },
    ]);
}

pub const kNumJSCallerSaved: i32 = 9;
pub const kNumCallerSavedDoubles: i32 = 14;
pub const kNumCalleeSaved: i32 = 18;
pub const kNumCalleeSavedDoubles: i32 = 18;

// Return the code of the n-th caller-saved register available to JavaScript
// e.g. JSCallerSavedReg(0) returns r0.code() == 0
pub fn JSCallerSavedCode(n: i32) -> i32 {
    kJSCallerSaved.registers()[n as usize].code()
}

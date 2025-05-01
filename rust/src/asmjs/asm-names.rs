// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines constants and lists of names used in the asm.js parser.

macro_rules! define_stdlib_math_value_list {
    ($V:ident) => {
        $V!(E, 2.718281828459045);
        $V!(LN10, 2.302585092994046);
        $V!(LN2, 0.6931471805599453);
        $V!(LOG2E, 1.4426950408889634);
        $V!(LOG10E, 0.4342944819032518);
        $V!(PI, 3.141592653589793);
        $V!(SQRT1_2, 0.7071067811865476);
        $V!(SQRT2, 1.4142135623730951);
    };
}

macro_rules! define_stdlib_math_function_monomorphic_list {
    ($V:ident) => {
        $V!(acos, Acos, kExprF64Acos, dq2d);
        $V!(asin, Asin, kExprF64Asin, dq2d);
        $V!(atan, Atan, kExprF64Atan, dq2d);
        $V!(cos, Cos, kExprF64Cos, dq2d);
        $V!(sin, Sin, kExprF64Sin, dq2d);
        $V!(tan, Tan, kExprF64Tan, dq2d);
        $V!(exp, Exp, kExprF64Exp, dq2d);
        $V!(log, Log, kExprF64Log, dq2d);
        $V!(atan2, Atan2, kExprF64Atan2, dqdq2d);
        $V!(pow, Pow, kExprF64Pow, dqdq2d);
        $V!(imul, Imul, kExprI32Mul, ii2s);
        $V!(clz32, Clz32, kExprI32Clz, i2s);
    };
}

macro_rules! define_stdlib_math_function_ceil_like_list {
    ($V:ident) => {
        $V!(ceil, Ceil, x, ceil_like);
        $V!(floor, Floor, x, ceil_like);
        $V!(sqrt, Sqrt, x, ceil_like);
    };
}

macro_rules! define_stdlib_math_function_list {
    ($V:ident) => {
        $V!(min, Min, x, minmax);
        $V!(max, Max, x, minmax);
        $V!(abs, Abs, x, abs);
        $V!(fround, Fround, x, fround);
        define_stdlib_math_function_monomorphic_list!($V);
        define_stdlib_math_function_ceil_like_list!($V);
    };
}

macro_rules! define_stdlib_array_type_list {
    ($V:ident) => {
        $V!(Int8Array, Mem8S, Mem8, I32);
        $V!(Uint8Array, Mem8U, Mem8, I32);
        $V!(Int16Array, Mem16S, Mem16, I32);
        $V!(Uint16Array, Mem16U, Mem16, I32);
        $V!(Int32Array, Mem, Mem, I32);
        $V!(Uint32Array, Mem, Mem, I32);
        $V!(Float32Array, Mem, Mem, F32);
        $V!(Float64Array, Mem, Mem, F64);
    };
}

macro_rules! define_stdlib_other_list {
    ($V:ident) => {
        $V!(Infinity);
        $V!(NaN);
        $V!(Math);
    };
}

macro_rules! define_keyword_name_list {
    ($V:ident) => {
        $V!(arguments);
        $V!(break);
        $V!(case);
        $V!(const);
        $V!(continue);
        $V!(default);
        $V!(do);
        $V!(else);
        $V!(eval);
        $V!(for);
        $V!(function);
        $V!(if);
        $V!(new);
        $V!(return);
        $V!(switch);
        $V!(var);
        $V!(while);
    };
}

macro_rules! define_long_symbol_name_list {
    ($V:ident) => {
        $V!("<=", LE);
        $V!(">=", GE);
        $V!("==", EQ);
        $V!("!=", NE);
        $V!("<<", SHL);
        $V!(">>", SAR);
        $V!(">>>", SHR);
        $V!("'use asm'", UseAsm);
    };
}

macro_rules! define_simple_single_token_list {
    ($V:ident) => {
        $V!('+');
        $V!('-');
        $V!('*');
        $V!('%');
        $V!('~');
        $V!('^');
        $V!('&');
        $V!('|');
        $V!('(');
        $V!(')');
        $V!('[');
        $V!(']');
        $V!('{');
        $V!('}');
        $V!(':');
        $V!(';');
        $V!(',');
        $V!('?');
    };
}

macro_rules! define_special_token_list {
    ($V:ident) => {
        $V!(kUninitialized, 0, "{uninitialized}");
        $V!(kEndOfInput, -1, "{end of input}");
        $V!(kParseError, -2, "{parse error}");
        $V!(kUnsigned, -3, "{unsigned value}");
        $V!(kDouble, -4, "{double value}");
    };
}

// Dummy enums and structs as there is no direct translation for kExpr enums
// or the other types in the original C++ file. The goal here is to mirror
// the organization and structure as faithfully as possible given the information
// available from the header file.

#[derive(Debug, Copy, Clone)]
enum WasmOpcode {
    kExprF64Acos,
    kExprF64Asin,
    kExprF64Atan,
    kExprF64Cos,
    kExprF64Sin,
    kExprF64Tan,
    kExprF64Exp,
    kExprF64Log,
    kExprF64Atan2,
    kExprF64Pow,
    kExprI32Mul,
    kExprI32Clz,
}

#[derive(Debug, Copy, Clone)]
enum AsmJsType {
    dq2d,
    dqdq2d,
    ii2s,
    i2s,
    ceil_like,
    minmax,
    abs,
    fround,
}

#[derive(Debug, Copy, Clone)]
enum ArrayType {
    I32,
    F32,
    F64,
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    LE,
    GE,
    EQ,
    NE,
    SHL,
    SAR,
    SHR,
    UseAsm,
}

#[derive(Debug, Copy, Clone)]
enum SpecialToken {
    kUninitialized,
    kEndOfInput,
    kParseError,
    kUnsigned,
    kDouble,
}

impl SpecialToken {
    fn value(&self) -> i32 {
        match self {
            SpecialToken::kUninitialized => 0,
            SpecialToken::kEndOfInput => -1,
            SpecialToken::kParseError => -2,
            SpecialToken::kUnsigned => -3,
            SpecialToken::kDouble => -4,
        }
    }

    fn string_name(&self) -> &'static str {
        match self {
            SpecialToken::kUninitialized => "{uninitialized}",
            SpecialToken::kEndOfInput => "{end of input}",
            SpecialToken::kParseError => "{parse error}",
            SpecialToken::kUnsigned => "{unsigned value}",
            SpecialToken::kDouble => "{double value}",
        }
    }
}
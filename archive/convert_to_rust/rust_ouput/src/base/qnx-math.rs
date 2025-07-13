// Converted from V8 C++ source files:
// Header: qnx-math.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file aims to provide math functions that might be missing or behave
// differently on QNX systems.  It also undefines certain macros that might
// conflict with the standard C++ library.

use std::num::FpCategory;

pub fn fpclassify(x: f64) -> FpCategory {
    x.classify()
}

pub fn isfinite(x: f64) -> bool {
    x.is_finite()
}

pub fn isinf(x: f64) -> bool {
    x.is_infinite()
}

pub fn isnan(x: f64) -> bool {
    x.is_nan()
}

pub fn isnormal(x: f64) -> bool {
    x.is_normal()
}

pub fn signbit(x: f64) -> bool {
    x.is_sign_negative()
}

pub fn lrint(x: f64) -> i64 {
    x.round() as i64
}

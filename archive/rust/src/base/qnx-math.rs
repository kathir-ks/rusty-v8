// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides math functions for QNX, undefining potentially conflicting
// macros and re-exporting lrint from the standard library.

// Rust does not have a direct equivalent to C++'s #undef for macros.  However,
// in this context, it's used to avoid conflicts with system-defined macros.
// Since Rust manages namespaces and identifiers differently, this undef is generally
// not needed. If conflicts do arise, they can be handled on a case-by-case basis
// by renaming or fully qualifying the conflicting symbols.

pub use std::num::FpCategory;
pub use std::num::FpCategory::*;

#[inline]
pub fn fpclassify(f: f64) -> FpCategory {
    f.classify()
}

#[inline]
pub fn isfinite(f: f64) -> bool {
    f.is_finite()
}

#[inline]
pub fn isinf(f: f64) -> bool {
    f.is_infinite()
}

#[inline]
pub fn isnan(f: f64) -> bool {
    f.is_nan()
}

#[inline]
pub fn isnormal(f: f64) -> bool {
    f.is_normal()
}

#[inline]
pub fn signbit(f: f64) -> bool {
    f.is_sign_negative()
}

pub use std::convert::TryInto;

#[inline]
pub fn lrint<T>(f: f64) -> T
where
    T: TryInto<i64>,
{
    f.round() as i64
        .try_into()
        .unwrap() // TODO: Propagate error using Result
}
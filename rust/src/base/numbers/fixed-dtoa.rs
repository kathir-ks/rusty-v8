// Copyright 2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod fixed_dtoa {
    /// Produces digits necessary to print a given number with
    /// `fractional_count` digits after the decimal point.
    /// The buffer must be big enough to hold the result plus one terminating null
    /// character.
    ///
    /// The produced digits might be too short in which case the caller has to fill
    /// the gaps with '0's.
    /// Example: `fast_fixed_dtoa(0.001, 5, ...)` is allowed to return `buffer = "1"`, and
    /// `decimal_point = -2`.
    /// Halfway cases are rounded towards +/-Infinity (away from 0). The call
    /// `fast_fixed_dtoa(0.15, 2, ...)` thus returns `buffer = "2"`, `decimal_point = 0`.
    /// The returned buffer may contain digits that would be truncated from the
    /// shortest representation of the input.
    ///
    /// This method only works for some parameters. If it can't handle the input it
    /// returns `false`. The output is null-terminated when the function succeeds.
    pub fn fast_fixed_dtoa(
        v: f64,
        fractional_count: i32,
        buffer: &mut [u8],
        length: &mut i32,
        decimal_point: &mut i32,
    ) -> bool {
        // Placeholder implementation.  A full port would require a
        // significant amount of numerical code.
        // This stub always returns false.

        // This is the safest way to indicate failure without doing anything unsafe.
        *length = 0;
        *decimal_point = 0;
        false
    }
}
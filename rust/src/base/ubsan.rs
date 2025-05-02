// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(all(not(feature = "undefined_sanitizer"), target_arch = "x86"))]

/// Implements the __mulodi4 function for 32-bit UBSan builds.
///
/// This function is required when compiling with `-fsanitize=undefined` on 32-bit platforms.
/// It mimics the behavior of libcompiler_rt's implementation.
#[no_mangle]
pub extern "C" fn __mulodi4(a: i64, b: i64, overflow: *mut i32) -> i64 {
    // Multiply in 32-bit chunks.
    // For inputs [AH AL]*[BH BL], the result is:
    //
    //            [AL*BL]  // r_low
    //    +    [AL*BH]     // r_mid1
    //    +    [AH*BL]     // r_mid2
    //    + [AH*BH]        // r_high
    //    = [R4 R3 R2 R1]  // high = [R4 R3], low = [R2 R1]
    //
    // Where of course we must be careful with carries between the columns.
    let a_low = (a & 0xFFFFFFFF) as u64;
    let a_high = (a as u64) >> 32;
    let b_low = (b & 0xFFFFFFFF) as u64;
    let b_high = (b as u64) >> 32;

    let r_low = a_low * b_low;
    let r_mid1 = a_low * b_high;
    let r_mid2 = a_high * b_low;
    let r_high = a_high * b_high;

    let mut result1 = r_low + (r_mid1 << 32);
    if result1 < r_low {
        r_high += 1;
    }
    let mut result2 = result1 + (r_mid2 << 32);
    if result2 < result1 {
        r_high += 1;
    }
    let r_high = r_high + (r_mid1 >> 32) + (r_mid2 >> 32);
    let result = result2 as i64;
    let result_sign = (result >> 63) as u64;
    let expected_result_sign = ((a >> 63) ^ (b >> 63)) as u64;

    unsafe {
        *overflow = if r_high > 0 || result_sign != expected_result_sign {
            1
        } else {
            0
        };
    }
    result
}
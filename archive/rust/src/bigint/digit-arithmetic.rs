// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Helper functions that operate on individual digits.

pub mod digit_arithmetic {
    use crate::bigint::bigint::digit_t;
    use crate::bigint::util::CountLeadingZeros;

    pub const kHalfDigitBits: usize = crate::bigint::bigint::kDigitBits / 2;
    pub const kHalfDigitBase: digit_t = 1 << kHalfDigitBits;
    pub const kHalfDigitMask: digit_t = kHalfDigitBase - 1;

    #[inline]
    pub const fn digit_ismax(x: digit_t) -> bool {
        !x == 0
    }

    // {carry} will be set to 0 or 1.
    #[inline]
    pub fn digit_add2(a: digit_t, b: digit_t, carry: &mut digit_t) -> digit_t {
        #[cfg(feature = "HAVE_TWODIGIT_T")]
        {
            let result: u64 = a as u64 + b as u64;
            *carry = (result >> crate::bigint::bigint::kDigitBits) as digit_t;
            result as digit_t
        }
        #[cfg(not(feature = "HAVE_TWODIGIT_T"))]
        {
            let result = a.wrapping_add(b);
            *carry = if result < a { 1 } else { 0 };
            result
        }
    }

    // This compiles to slightly better machine code than repeated invocations
    // of {digit_add2}.
    #[inline]
    pub fn digit_add3(a: digit_t, b: digit_t, c: digit_t, carry: &mut digit_t) -> digit_t {
        #[cfg(feature = "HAVE_TWODIGIT_T")]
        {
            let result: u64 = a as u64 + b as u64 + c as u64;
            *carry = (result >> crate::bigint::bigint::kDigitBits) as digit_t;
            result as digit_t
        }
        #[cfg(not(feature = "HAVE_TWODIGIT_T"))]
        {
            let mut result = a.wrapping_add(b);
            *carry = if result < a { 1 } else { 0 };
            result = result.wrapping_add(c);
            if result < c {
                *carry += 1;
            }
            result
        }
    }

    // {borrow} will be set to 0 or 1.
    #[inline]
    pub fn digit_sub(a: digit_t, b: digit_t, borrow: &mut digit_t) -> digit_t {
        #[cfg(feature = "HAVE_TWODIGIT_T")]
        {
            let result: i64 = a as i64 - b as i64;
            *borrow = ((result >> crate::bigint::bigint::kDigitBits) & 1) as digit_t;
            result as digit_t
        }
        #[cfg(not(feature = "HAVE_TWODIGIT_T"))]
        {
            let result = a.wrapping_sub(b);
            *borrow = if result > a { 1 } else { 0 };
            result
        }
    }

    // {borrow_out} will be set to 0 or 1.
    #[inline]
    pub fn digit_sub2(a: digit_t, b: digit_t, borrow_in: digit_t, borrow_out: &mut digit_t) -> digit_t {
        #[cfg(feature = "HAVE_TWODIGIT_T")]
        {
            let subtrahend: i64 = b as i64 + borrow_in as i64;
            let result: i64 = a as i64 - subtrahend;
            *borrow_out = ((result >> crate::bigint::bigint::kDigitBits) & 1) as digit_t;
            result as digit_t
        }
        #[cfg(not(feature = "HAVE_TWODIGIT_T"))]
        {
            let mut result = a.wrapping_sub(b);
            *borrow_out = if result > a { 1 } else { 0 };
            if result < borrow_in {
                *borrow_out += 1;
            }
            result = result.wrapping_sub(borrow_in);
            result
        }
    }

    // Returns the low half of the result. High half is in {high}.
    #[inline]
    pub fn digit_mul(a: digit_t, b: digit_t, high: &mut digit_t) -> digit_t {
        #[cfg(feature = "HAVE_TWODIGIT_T")]
        {
            let result: u64 = a as u64 * b as u64;
            *high = (result >> crate::bigint::bigint::kDigitBits) as digit_t;
            result as digit_t
        }
        #[cfg(not(feature = "HAVE_TWODIGIT_T"))]
        {
            // Multiply in half-pointer-sized chunks.
            // For inputs [AH AL]*[BH BL], the result is:
            //
            //            [AL*BL]  // r_low
            //    +    [AL*BH]     // r_mid1
            //    +    [AH*BL]     // r_mid2
            //    + [AH*BH]        // r_high
            //    = [R4 R3 R2 R1]  // high = [R4 R3], low = [R2 R1]
            //
            // Where of course we must be careful with carries between the columns.
            let a_low = a & kHalfDigitMask;
            let a_high = a >> kHalfDigitBits;
            let b_low = b & kHalfDigitMask;
            let b_high = b >> kHalfDigitBits;

            let r_low = a_low * b_low;
            let r_mid1 = a_low * b_high;
            let r_mid2 = a_high * b_low;
            let r_high = a_high * b_high;

            let mut carry = 0;
            let low = digit_add3(
                r_low,
                r_mid1 << kHalfDigitBits,
                r_mid2 << kHalfDigitBits,
                &mut carry,
            );
            *high = (r_mid1 >> kHalfDigitBits)
                .wrapping_add(r_mid2 >> kHalfDigitBits)
                .wrapping_add(r_high)
                .wrapping_add(carry);
            low
        }
    }

    // Returns the quotient.
    // quotient = (high << kDigitBits + low - remainder) / divisor
    #[inline]
    pub fn digit_div(
        high: digit_t,
        low: digit_t,
        divisor: digit_t,
        remainder: &mut digit_t,
    ) -> digit_t {
        #[cfg(target_arch = "x86_64")]
        #[cfg(any(target_env = "gnu", target_env = "musl"))]
        {
            let quotient: digit_t;
            let rem: digit_t;

            unsafe {
                std::arch::asm!(
                    "divq  %{[divisor]}",
                    in("rax") low,
                    in("rdx") high,
                    divisor = in(reg) divisor,
                    out("rax") quotient,
                    out("rdx") rem,
                    options(att_syntax)
                );
            }

            *remainder = rem;
            return quotient;
        }

        #[cfg(target_arch = "x86")]
        #[cfg(any(target_env = "gnu", target_env = "musl"))]
        {
            let quotient: digit_t;
            let rem: digit_t;

            unsafe {
                std::arch::asm!(
                    "divl  %{[divisor]}",
                    in("eax") low,
                    in("edx") high,
                    divisor = in(reg) divisor,
                    out("eax") quotient,
                    out("edx") rem,
                    options(att_syntax)
                );
            }

            *remainder = rem;
            return quotient;
        }

        #[cfg(not(any(
            all(target_arch = "x86_64", any(target_env = "gnu", target_env = "musl")),
            all(target_arch = "x86", any(target_env = "gnu", target_env = "musl"))
        )))]
        {
            // Adapted from Warren, Hacker's Delight, p. 152.
            let s = CountLeadingZeros(divisor);

            let mut divisor = divisor << s;

            let vn1 = divisor >> kHalfDigitBits;
            let vn0 = divisor & kHalfDigitMask;
            // {s} can be 0. {low >> kDigitBits} would be undefined behavior, so
            // we mask the shift amount with {kShiftMask}, and the result with
            // {s_zero_mask} which is 0 if s == 0 and all 1-bits otherwise.
            let kShiftMask = crate::bigint::bigint::kDigitBits - 1;
            let s_zero_mask = ((-(s as isize)) >> (crate::bigint::bigint::kDigitBits - 1)) as digit_t;
            let un32 = (high << s) | ((low >> ((crate::bigint::bigint::kDigitBits - s) & kShiftMask)) & s_zero_mask);
            let un10 = low << s;
            let un1 = un10 >> kHalfDigitBits;
            let un0 = un10 & kHalfDigitMask;
            let mut q1 = un32 / vn1;
            let mut rhat = un32 - q1 * vn1;

            while q1 >= kHalfDigitBase || q1 * vn0 > rhat * kHalfDigitBase + un1 {
                q1 -= 1;
                rhat += vn1;
                if rhat >= kHalfDigitBase {
                    break;
                }
            }

            let un21 = un32 * kHalfDigitBase + un1 - q1 * divisor;
            let mut q0 = un21 / vn1;
            rhat = un21 - q0 * vn1;

            while q0 >= kHalfDigitBase || q0 * vn0 > rhat * kHalfDigitBase + un0 {
                q0 -= 1;
                rhat += vn1;
                if rhat >= kHalfDigitBase {
                    break;
                }
            }

            *remainder = (un21 * kHalfDigitBase + un0 - q0 * divisor) >> s;
            q1 * kHalfDigitBase + q0
        }
    }
}
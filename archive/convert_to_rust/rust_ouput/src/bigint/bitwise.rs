// Converted from V8 C++ source files:
// Header: N/A
// Implementation: bitwise.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bigint {
    pub mod bigint_internal {
        pub const kDigitBits: usize = 32;
    }
    use self::bigint_internal::kDigitBits;
    use crate::Result;

    pub type DigitT = u32;

    #[derive(Debug, Clone, Copy)]
    pub struct Digits<'a> {
        data: &'a [DigitT],
    }

    impl<'a> Digits<'a> {
        pub fn new(data: &'a [DigitT]) -> Self {
            Digits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get(&self, index: usize) -> Option<&DigitT> {
            self.data.get(index)
        }

        pub fn msd(&self) -> DigitT {
            if self.data.is_empty() {
                0
            } else {
                self.data[self.data.len() - 1]
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct RWDigits<'a> {
        data: &'a mut [DigitT],
    }

    impl<'a> RWDigits<'a> {
        pub fn new(data: &'a mut [DigitT]) -> Self {
            RWDigits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = DigitT;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    pub fn digit_sub(x: DigitT, y: DigitT, borrow: &mut DigitT) -> DigitT {
        let (diff, carry) = x.overflowing_sub(y + *borrow);
        *borrow = if carry { 1 } else { 0 };
        diff
    }

    pub fn digit_sub2(x: DigitT, y: DigitT, borrow_in: DigitT, borrow_out: &mut DigitT) -> DigitT {
        let (sub_result, carry1) = x.overflowing_sub(y);
        let (final_result, carry2) = sub_result.overflowing_sub(borrow_in);
        *borrow_out = if carry1 || carry2 { 1 } else { 0 };
        final_result
    }
    
    pub fn digit_ismax(digit: DigitT) -> bool {
        digit == DigitT::max_value()
    }

    pub fn add(z: RWDigits, val: DigitT) {
        let mut carry: DigitT = val;
        for i in 0..z.len() {
            let (sum, overflow) = z[i].overflowing_add(carry);
            z[i] = sum;
            carry = if overflow { 1 } else { 0 };
        }
        // If there's still a carry after adding to all digits, it means there's an overflow
        // which is not explicitly handled here.
        // In C++, BigInt::Add can reallocate to add one extra digit.
        // For now, we'll ignore it.
    }

    pub fn div_ceil(a: usize, b: usize) -> usize {
        (a + b - 1) / b
    }

    pub struct RightShiftState {
        pub must_round_down: bool,
    }

    pub fn bitwise_and_pos_pos(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        assert!(z.len() >= pairs);
        for i in 0..pairs {
            z[i] = x.get(i).copied().unwrap_or(0) & y.get(i).copied().unwrap_or(0);
        }
        for i in pairs..z.len() {
            z[i] = 0;
        }
    }

    pub fn bitwise_and_neg_neg(z: RWDigits, x: Digits, y: Digits) {
        // (-x) & (-y) == ~(x-1) & ~(y-1)
        //             == ~((x-1) | (y-1))
        //             == -(((x-1) | (y-1)) + 1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow = 1;
        let mut y_borrow = 1;
        for i in 0..pairs {
            z[i] = digit_sub(x.get(i).copied().unwrap_or(0), x_borrow, &mut x_borrow)
                | digit_sub(y.get(i).copied().unwrap_or(0), y_borrow, &mut y_borrow);
        }
        // (At least) one of the next two loops will perform zero iterations:
        for i in pairs..x.len() {
            if i < z.len() {
                z[i] = digit_sub(x.get(i).copied().unwrap_or(0), x_borrow, &mut x_borrow);
            }
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = digit_sub(y.get(i).copied().unwrap_or(0), y_borrow, &mut y_borrow);
            }
        }
        assert_eq!(x_borrow, 0);
        assert_eq!(y_borrow, 0);
        for i in x.len().max(y.len())..z.len() {
            z[i] = 0;
        }
        add(z, 1);
    }

    pub fn bitwise_and_pos_neg(z: RWDigits, x: Digits, y: Digits) {
        // x & (-y) == x & ~(y-1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow = 1;
        for i in 0..pairs {
            z[i] = x.get(i).copied().unwrap_or(0) & !digit_sub(y.get(i).copied().unwrap_or(0), borrow, &mut borrow);
        }
        for i in pairs..x.len() {
            if i < z.len() {
                z[i] = x.get(i).copied().unwrap_or(0);
            }
        }
        for i in x.len()..z.len() {
            z[i] = 0;
        }
    }

    pub fn bitwise_or_pos_pos(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        for i in 0..pairs {
            z[i] = x.get(i).copied().unwrap_or(0) | y.get(i).copied().unwrap_or(0);
        }
        // (At least) one of the next two loops will perform zero iterations:
        for i in pairs..x.len() {
            if i < z.len() {
                z[i] = x.get(i).copied().unwrap_or(0);
            }
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = y.get(i).copied().unwrap_or(0);
            }
        }
        for i in x.len().max(y.len())..z.len() {
            z[i] = 0;
        }
    }

    pub fn bitwise_or_neg_neg(z: RWDigits, x: Digits, y: Digits) {
        // (-x) | (-y) == ~(x-1) | ~(y-1)
        //             == ~((x-1) & (y-1))
        //             == -(((x-1) & (y-1)) + 1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow = 1;
        let mut y_borrow = 1;
        for i in 0..pairs {
            z[i] = digit_sub(x.get(i).copied().unwrap_or(0), x_borrow, &mut x_borrow)
                & digit_sub(y.get(i).copied().unwrap_or(0), y_borrow, &mut y_borrow);
        }
        // Any leftover borrows don't matter, the '&' would drop them anyway.
        for i in pairs..z.len() {
            z[i] = 0;
        }
        add(z, 1);
    }

    pub fn bitwise_or_pos_neg(z: RWDigits, x: Digits, y: Digits) {
        // x | (-y) == x | ~(y-1) == ~((y-1) &~ x) == -(((y-1) &~ x) + 1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow = 1;
        for i in 0..pairs {
            z[i] = digit_sub(y.get(i).copied().unwrap_or(0), borrow, &mut borrow) & !x.get(i).copied().unwrap_or(0);
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = digit_sub(y.get(i).copied().unwrap_or(0), borrow, &mut borrow);
            }
        }
        assert_eq!(borrow, 0);
        for i in y.len()..z.len() {
            z[i] = 0;
        }
        add(z, 1);
    }

    pub fn bitwise_xor_pos_pos(z: RWDigits, mut x: Digits, mut y: Digits) {
        let mut pairs = x.len();
        if y.len() < x.len() {
            std::mem::swap(&mut x, &mut y);
            pairs = x.len();
        }
        assert!(x.len() <= y.len());
        for i in 0..pairs {
            z[i] = x.get(i).copied().unwrap_or(0) ^ y.get(i).copied().unwrap_or(0);
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = y.get(i).copied().unwrap_or(0);
            }
        }
        for i in y.len()..z.len() {
            z[i] = 0;
        }
    }

    pub fn bitwise_xor_neg_neg(z: RWDigits, x: Digits, y: Digits) {
        // (-x) ^ (-y) == ~(x-1) ^ ~(y-1) == (x-1) ^ (y-1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow = 1;
        let mut y_borrow = 1;
        for i in 0..pairs {
            z[i] = digit_sub(x.get(i).copied().unwrap_or(0), x_borrow, &mut x_borrow)
                ^ digit_sub(y.get(i).copied().unwrap_or(0), y_borrow, &mut y_borrow);
        }
        // (At least) one of the next two loops will perform zero iterations:
        for i in pairs..x.len() {
            if i < z.len() {
                z[i] = digit_sub(x.get(i).copied().unwrap_or(0), x_borrow, &mut x_borrow);
            }
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = digit_sub(y.get(i).copied().unwrap_or(0), y_borrow, &mut y_borrow);
            }
        }
        assert_eq!(x_borrow, 0);
        assert_eq!(y_borrow, 0);
        for i in x.len().max(y.len())..z.len() {
            z[i] = 0;
        }
    }

    pub fn bitwise_xor_pos_neg(z: RWDigits, x: Digits, y: Digits) {
        // x ^ (-y) == x ^ ~(y-1) == ~(x ^ (y-1)) == -((x ^ (y-1)) + 1)
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow = 1;
        for i in 0..pairs {
            z[i] = x.get(i).copied().unwrap_or(0) ^ digit_sub(y.get(i).copied().unwrap_or(0), borrow, &mut borrow);
        }
        // (At least) one of the next two loops will perform zero iterations:
        for i in pairs..x.len() {
            if i < z.len() {
                z[i] = x.get(i).copied().unwrap_or(0);
            }
        }
        for i in pairs..y.len() {
            if i < z.len() {
                z[i] = digit_sub(y.get(i).copied().unwrap_or(0), borrow, &mut borrow);
            }
        }
        assert_eq!(borrow, 0);
        for i in x.len().max(y.len())..z.len() {
            z[i] = 0;
        }
        add(z, 1);
    }

    pub fn left_shift(z: RWDigits, x: Digits, shift: DigitT) {
        let digit_shift = (shift / kDigitBits as DigitT) as usize;
        let bits_shift = (shift % kDigitBits as DigitT) as usize;

        for i in 0..digit_shift {
            z[i] = 0;
        }
        if bits_shift == 0 {
            for i in digit_shift..(x.len() + digit_shift) {
                if i < z.len() {
                    z[i] = x.get(i - digit_shift).copied().unwrap_or(0);
                }
            }
            for i in (x.len() + digit_shift)..z.len() {
                z[i] = 0;
            }
        } else {
            let mut carry: DigitT = 0;
            for i in digit_shift..(x.len() + digit_shift) {
                if i < z.len() {
                    let d = x.get(i - digit_shift).copied().unwrap_or(0);
                    z[i] = (d << bits_shift) | carry;
                    carry = d >> (kDigitBits - bits_shift);
                }
            }
            if carry != 0 && (x.len() + digit_shift) < z.len() {
                z[x.len() + digit_shift] = carry;
                for i in (x.len() + digit_shift + 1)..z.len() {
                    z[i] = 0;
                }
            } else {
                for i in (x.len() + digit_shift)..z.len() {
                    z[i] = 0;
                }
            }
        }
    }

    pub fn right_shift_result_length(
        x: Digits,
        x_sign: bool,
        shift: DigitT,
        state: &mut Option<&mut RightShiftState>,
    ) -> usize {
        let digit_shift = (shift / kDigitBits as DigitT) as usize;
        let bits_shift = (shift % kDigitBits as DigitT) as usize;
        let mut result_length = x.len() - digit_shift;
        if result_length <= 0 {
            return 0;
        }

        // For negative numbers, round down if any bit was shifted out (so that e.g.
        // -5n >> 1n == -3n and not -2n). Check now whether this will happen and
        // whether it can cause overflow into a new digit.
        let mut must_round_down = false;
        if x_sign {
            let mask = (1 as DigitT << bits_shift) - 1;
            if (x.get(digit_shift).copied().unwrap_or(0) & mask) != 0 {
                must_round_down = true;
            } else {
                for i in 0..digit_shift {
                    if x.get(i).copied().unwrap_or(0) != 0 {
                        must_round_down = true;
                        break;
                    }
                }
            }
        }
        // If bits_shift is non-zero, it frees up bits, preventing overflow.
        if must_round_down && bits_shift == 0 {
            // Overflow cannot happen if the most significant digit has unset bits.
            let rounding_can_overflow = digit_ismax(x.msd());
            if rounding_can_overflow {
                result_length += 1;
            }
        }

        if let Some(s) = state {
            assert!(!must_round_down || x_sign);
            s.must_round_down = must_round_down;
        }
        result_length
    }

    pub fn right_shift(
        z: RWDigits,
        x: Digits,
        shift: DigitT,
        state: &RightShiftState,
    ) {
        let digit_shift = (shift / kDigitBits as DigitT) as usize;
        let bits_shift = (shift % kDigitBits as DigitT) as usize;

        let mut i = 0;
        if bits_shift == 0 {
            for i in 0..(x.len() - digit_shift) {
                z[i] = x.get(i + digit_shift).copied().unwrap_or(0);
            }
        } else {
            let mut carry = x.get(digit_shift).copied().unwrap_or(0) >> bits_shift;
            for i in 0..(x.len() - digit_shift - 1) {
                let d = x.get(i + digit_shift + 1).copied().unwrap_or(0);
                z[i] = (d << (kDigitBits - bits_shift)) | carry;
                carry = d >> bits_shift;
            }
            if i < z.len() {
                z[i] = carry;
                i += 1;
            }
        }
        for _ in i..z.len() {
            
        }

        if state.must_round_down {
            // Rounding down (a negative value) means adding one to
            // its absolute value. This cannot overflow.
            add(z, 1);
        }
    }

    // Z := (least significant n bits of X).
    fn truncate_to_n_bits(z: RWDigits, x: Digits, n: i32) {
        let digits = div_ceil(n as usize, kDigitBits);
        let bits = (n as usize) % kDigitBits;
        // Copy all digits except the MSD.
        let last = digits - 1;
        for i in 0..last {
            z[i] = x.get(i).copied().unwrap_or(0);
        }
        // The MSD might contain extra bits that we don't want.
        let mut msd = x.get(last).copied().unwrap_or(0);
        if bits != 0 {
            let drop = kDigitBits - bits;
            msd = (msd << drop) >> drop;
        }
        z[last] = msd;
    }

    // Z := 2**n - (least significant n bits of X).
    fn truncate_and_sub_from_power_of_two(z: RWDigits, x: Digits, n: i32) {
        let digits = div_ceil(n as usize, kDigitBits);
        let bits = (n as usize) % kDigitBits;
        // Process all digits except the MSD. Take X's digits, then simulate leading
        // zeroes.
        let last = digits - 1;
        let have_x = std::cmp::min(last, x.len());
        let mut borrow = 0;
        for i in 0..have_x {
            z[i] = digit_sub2(0, x.get(i).copied().unwrap_or(0), borrow, &mut borrow);
        }
        for i in have_x..last {
            z[i] = digit_sub(0, borrow, &mut borrow);
        }

        // The MSD might contain extra bits that we don't want.
        let mut msd = if last < x.len() { x.get(last).copied().unwrap_or(0) } else { 0 };
        if bits == 0 {
            z[last] = digit_sub2(0, msd, borrow, &mut borrow);
        } else {
            let drop = kDigitBits - bits;
            msd = (msd << drop) >> drop;
            let minuend_msd = 1 << bits;
            let result_msd = digit_sub2(minuend_msd, msd, borrow, &mut borrow);
            assert_eq!(borrow, 0); // result < 2^n.
                                     // If all subtracted bits were zero, we have to get rid of the
                                     // materialized minuend_msd again.
            z[last] = result_msd & (minuend_msd - 1);
        }
    }

    // Returns -1 when the operation would return X unchanged.
    pub fn as_int_n_result_length(x: Digits, x_negative: bool, n: i32) -> i32 {
        let needed_digits = div_ceil(n as usize, kDigitBits);
        // Generally: decide based on number of digits, and bits in the top digit.
        if x.len() < needed_digits {
            return -1;
        }
        if x.len() > needed_digits {
            return needed_digits as i32;
        }
        let top_digit = x.get(needed_digits - 1).copied().unwrap_or(0);
        let compare_digit = 1 << ((n - 1) % kDigitBits as i32);
        if top_digit < compare_digit {
            return -1;
        }
        if top_digit > compare_digit {
            return needed_digits as i32;
        }
        // Special case: if X == -2**(n-1), truncation is a no-op.
        if !x_negative {
            return needed_digits as i32;
        }
        for i in (0..(needed_digits - 1)).rev() {
            if x.get(i).copied().unwrap_or(0) != 0 {
                return needed_digits as i32;
            }
        }
        -1
    }

    pub fn as_int_n(z: RWDigits, x: Digits, x_negative: bool, n: i32) -> bool {
        assert!(x.len() > 0);
        assert!(n > 0);
        assert!(as_int_n_result_length(x, x_negative, n) > 0);
        let needed_digits = div_ceil(n as usize, kDigitBits);
        let top_digit = x.get(needed_digits - 1).copied().unwrap_or(0);
        let compare_digit = 1 << ((n - 1) % kDigitBits as i32);
        // The canonical algorithm would be: convert negative numbers to two's
        // complement representation, truncate, convert back to sign+magnitude. To
        // avoid the conversions, we predict what the result would be:
        // When the (n-1)th bit is not set:
        //  - truncate the absolute value
        //  - preserve the sign.
        // When the (n-1)th bit is set:
        //  - subtract the truncated absolute value from 2**n to simulate two's
        //    complement representation
        //  - flip the sign, unless it's the special case where the input is negative
        //    and the result is the minimum n-bit integer. E.g. asIntN(3, -12) => -4.
        let has_bit = (top_digit & compare_digit) == compare_digit;
        if !has_bit {
            truncate_to_n_bits(z, x, n);
            return x_negative;
        }
        truncate_and_sub_from_power_of_two(z, x, n);
        if !x_negative {
            return true; // Result is negative.
        }
        // Scan for the special case (see above): if all bits below the (n-1)th
        // digit are zero, the result is negative.
        if (top_digit & (compare_digit - 1)) != 0 {
            return false;
        }
        for i in (0..(needed_digits - 1)).rev() {
            if x.get(i).copied().unwrap_or(0) != 0 {
                return false;
            }
        }
        true
    }

    // Returns -1 when the operation would return X unchanged.
    pub fn as_uint_n_pos_result_length(x: Digits, n: i32) -> i32 {
        let needed_digits = div_ceil(n as usize, kDigitBits);
        if x.len() < needed_digits {
            return -1;
        }
        if x.len() > needed_digits {
            return needed_digits as i32;
        }
        let bits_in_top_digit = (n as usize) % kDigitBits;
        if bits_in_top_digit == 0 {
            return -1;
        }
        let top_digit = x.get(needed_digits - 1).copied().unwrap_or(0);
        if (top_digit >> bits_in_top_digit) == 0 {
            return -1;
        }
        needed_digits as i32
    }

    pub fn as_uint_n_pos(z: RWDigits, x: Digits, n: i32) {
        assert!(as_uint_n_pos_result_length(x, n) > 0);
        truncate_to_n_bits(z, x, n);
    }

    pub fn as_uint_n_neg(z: RWDigits, x: Digits, n: i32) {
        truncate_and_sub_from_power_of_two(z, x, n);
    }
}

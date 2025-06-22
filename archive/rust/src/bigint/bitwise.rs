pub mod bigint {
    pub mod bigint_internal {
        pub const kDigitBits: usize = 64; // Assuming 64-bit digits
        pub type digit_t = u64;
    }

    pub mod digit_arithmetic {
        use super::bigint_internal::digit_t;

        #[inline]
        pub fn digit_sub(a: digit_t, b: digit_t, borrow: &mut digit_t) -> digit_t {
            let (result, overflow) = a.overflowing_sub(b.wrapping_add(*borrow));
            if overflow {
                *borrow = 1;
            } else {
                *borrow = 0;
            }
            result
        }

        #[inline]
        pub fn digit_sub2(a: digit_t, b: digit_t, borrow_in: digit_t, borrow_out: &mut digit_t) -> digit_t {
            let (sub_result, sub_overflow) = a.overflowing_sub(b);
            let (result, add_overflow) = sub_result.overflowing_sub(borrow_in);

            if sub_overflow || add_overflow {
                *borrow_out = 1;
            } else {
                *borrow_out = 0;
            }

            result
        }

        #[inline]
        pub fn digit_ismax(digit: digit_t) -> bool {
            digit == digit_t::max_value()
        }

    }

    pub mod util {
        use super::bigint_internal::kDigitBits;

        #[inline]
        pub fn div_ceil(x: i32, y: usize) -> i32 {
            ((x as f64) / (y as f64)).ceil() as i32
        }
    }

    pub mod vector_arithmetic {
        use super::bigint_internal::digit_t;
        use super::bigint::Add;

        pub type Digits<'a> = &'a [digit_t];
        pub type RWDigits<'a> = &'a mut [digit_t];
    }

    use self::bigint_internal::{digit_t, kDigitBits};
    use self::digit_arithmetic::{digit_sub, digit_ismax, digit_sub2};
    use self::util::div_ceil;
    use self::vector_arithmetic::{Digits, RWDigits};

    pub struct RightShiftState {
        pub must_round_down: bool,
    }

    pub fn BitwiseAnd_PosPos(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        assert!(z.len() >= pairs);
        let mut i = 0;
        for i in 0..pairs {
            z[i] = x[i] & y[i];
        }
        for i in pairs..z.len() {
            z[i] = 0;
        }
    }

    pub fn BitwiseAnd_NegNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow: digit_t = 1;
        let mut y_borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = digit_sub(x[i], x_borrow, &mut x_borrow) |
                   digit_sub(y[i], y_borrow, &mut y_borrow);
        }
        for i in pairs..x.len() {
            z[i] = digit_sub(x[i], x_borrow, &mut x_borrow);
        }
        for i in pairs..y.len() {
            z[i] = digit_sub(y[i], y_borrow, &mut y_borrow);
        }
        assert_eq!(x_borrow, 0);
        assert_eq!(y_borrow, 0);
        for i in std::cmp::max(x.len(), y.len())..z.len() {
            z[i] = 0;
        }
        Add(z, 1);
    }

    pub fn BitwiseAnd_PosNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = x[i] & !digit_sub(y[i], borrow, &mut borrow);
        }
        for i in pairs..x.len() {
            z[i] = x[i];
        }
        for i in x.len()..z.len() {
            z[i] = 0;
        }
    }

    pub fn BitwiseOr_PosPos(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut i = 0;
        for i in 0..pairs {
            z[i] = x[i] | y[i];
        }
        for i in pairs..x.len() {
            z[i] = x[i];
        }
        for i in pairs..y.len() {
            z[i] = y[i];
        }
        for i in std::cmp::max(x.len(),y.len())..z.len() {
            z[i] = 0;
        }
    }

    pub fn BitwiseOr_NegNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow: digit_t = 1;
        let mut y_borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = digit_sub(x[i], x_borrow, &mut x_borrow) &
                   digit_sub(y[i], y_borrow, &mut y_borrow);
        }
        for i in pairs..z.len() {
            z[i] = 0;
        }
        Add(z, 1);
    }

    pub fn BitwiseOr_PosNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = digit_sub(y[i], borrow, &mut borrow) & !x[i];
        }
        for i in pairs..y.len() {
            z[i] = digit_sub(y[i], borrow, &mut borrow);
        }
        assert_eq!(borrow, 0);
        for i in y.len()..z.len() {
            z[i] = 0;
        }
        Add(z, 1);
    }

    pub fn BitwiseXor_PosPos(z: RWDigits, mut x: Digits, mut y: Digits) {
        let mut pairs = x.len();
        if y.len() < x.len() {
            std::mem::swap(&mut x, &mut y);
            pairs = x.len();
        }
        assert!(x.len() <= y.len());
        let mut i = 0;
        for i in 0..pairs {
            z[i] = x[i] ^ y[i];
        }
        for i in pairs..y.len() {
            z[i] = y[i];
        }
        for i in y.len()..z.len() {
            z[i] = 0;
        }
    }

    pub fn BitwiseXor_NegNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut x_borrow: digit_t = 1;
        let mut y_borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = digit_sub(x[i], x_borrow, &mut x_borrow) ^
                   digit_sub(y[i], y_borrow, &mut y_borrow);
        }
        for i in pairs..x.len() {
            z[i] = digit_sub(x[i], x_borrow, &mut x_borrow);
        }
        for i in pairs..y.len() {
            z[i] = digit_sub(y[i], y_borrow, &mut y_borrow);
        }
        assert_eq!(x_borrow, 0);
        assert_eq!(y_borrow, 0);
        for i in std::cmp::max(x.len(), y.len())..z.len() {
            z[i] = 0;
        }
    }

    pub fn BitwiseXor_PosNeg(z: RWDigits, x: Digits, y: Digits) {
        let pairs = std::cmp::min(x.len(), y.len());
        let mut borrow: digit_t = 1;
        let mut i = 0;
        for i in 0..pairs {
            z[i] = x[i] ^ digit_sub(y[i], borrow, &mut borrow);
        }
        for i in pairs..x.len() {
            z[i] = x[i];
        }
        for i in pairs..y.len() {
            z[i] = digit_sub(y[i], borrow, &mut borrow);
        }
        assert_eq!(borrow, 0);
        for i in std::cmp::max(x.len(), y.len())..z.len() {
            z[i] = 0;
        }
        Add(z, 1);
    }

    pub fn LeftShift(z: RWDigits, x: Digits, shift: digit_t) {
        let digit_shift = (shift / kDigitBits as digit_t) as usize;
        let bits_shift = (shift % kDigitBits as digit_t) as usize;

        let mut i = 0;
        for i in 0..digit_shift {
            z[i] = 0;
        }
        if bits_shift == 0 {
            for i in digit_shift..(x.len() + digit_shift) {
                z[i] = x[i - digit_shift];
            }
            for i in (x.len() + digit_shift)..z.len() {
                z[i] = 0;
            }
        } else {
            let mut carry: digit_t = 0;
            for i in digit_shift..(x.len() + digit_shift) {
                let d = x[i - digit_shift];
                z[i] = (d << bits_shift) | carry;
                carry = d >> (kDigitBits - bits_shift);
            }
            if carry != 0 {
                z[x.len() + digit_shift] = carry;
                i += 1;
            }

            for i in (x.len() + digit_shift + (if carry != 0 {1} else {0}))..z.len() {
                z[i] = 0;
            }
        }
    }

    pub fn RightShift_ResultLength(x: Digits, x_sign: bool, shift: digit_t, state: Option<&mut RightShiftState>) -> i32 {
        let digit_shift = (shift / kDigitBits as digit_t) as usize;
        let bits_shift = (shift % kDigitBits as digit_t) as usize;
        let mut result_length = x.len() as i32 - digit_shift as i32;
        if result_length <= 0 { return 0; }

        let mut must_round_down = false;
        if x_sign {
            let mask = (1 as digit_t << bits_shift) - 1;
            if (x[digit_shift] & mask) != 0 {
                must_round_down = true;
            } else {
                for i in 0..digit_shift {
                    if x[i] != 0 {
                        must_round_down = true;
                        break;
                    }
                }
            }
        }

        if must_round_down && bits_shift == 0 {
            let rounding_can_overflow = digit_ismax(x[x.len() -1]);
            if (rounding_can_overflow) { result_length += 1; }
        }

        if let Some(s) = state {
            assert!(!must_round_down || x_sign);
            s.must_round_down = must_round_down;
        }

        return result_length;
    }

    pub fn RightShift(z: RWDigits, x: Digits, shift: digit_t, state: &RightShiftState) {
        let digit_shift = (shift / kDigitBits as digit_t) as usize;
        let bits_shift = (shift % kDigitBits as digit_t) as usize;

        let mut i = 0;
        if bits_shift == 0 {
            for i in 0..(x.len() - digit_shift) {
                z[i] = x[i + digit_shift];
            }
        } else {
            let mut carry = x[digit_shift] >> bits_shift;
            for i in 0..(x.len() - digit_shift - 1) {
                let d = x[i + digit_shift + 1];
                z[i] = (d << (kDigitBits - bits_shift)) | carry;
                carry = d >> bits_shift;
            }
            z[x.len() - digit_shift - 1] = carry;
            i = x.len() - digit_shift;

        }

        for i in i..z.len() {
            z[i] = 0;
        }

        if state.must_round_down {
            Add(z, 1);
        }
    }

    // Z := (least significant n bits of X).
    fn TruncateToNBits(z: RWDigits, x: Digits, n: i32) {
        let digits = div_ceil(n, kDigitBits);
        let bits = n % kDigitBits;

        // Copy all digits except the MSD.
        let last = digits - 1;
        for i in 0..last {
            z[i as usize] = x[i as usize];
        }

        // The MSD might contain extra bits that we don't want.
        let mut msd = x[last as usize];
        if bits != 0 {
            let drop = kDigitBits as i32 - bits;
            msd = (msd << drop) >> drop;
        }
        z[last as usize] = msd;
    }

    // Z := 2**n - (least significant n bits of X).
    fn TruncateAndSubFromPowerOfTwo(z: RWDigits, x: Digits, n: i32) {
        let digits = div_ceil(n, kDigitBits);
        let bits = n % kDigitBits;

        // Process all digits except the MSD. Take X's digits, then simulate leading zeroes.
        let last = digits - 1;
        let have_x = std::cmp::min(last, x.len() as i32);
        let mut borrow: digit_t = 0;

        for i in 0..have_x {
            z[i as usize] = digit_sub2(0, x[i as usize], borrow, &mut borrow);
        }

        for i in have_x..last {
            z[i as usize] = digit_sub(0, borrow, &mut borrow);
        }

        // The MSD might contain extra bits that we don't want.
        let mut msd = if last < x.len() as i32 { x[last as usize] } else { 0 };

        if bits == 0 {
            z[last as usize] = digit_sub2(0, msd, borrow, &mut borrow);
        } else {
            let drop = kDigitBits as i32 - bits;
            msd = (msd << drop) >> drop;
            let minuend_msd: digit_t = 1 << bits;
            let result_msd = digit_sub2(minuend_msd, msd, borrow, &mut borrow);
            assert_eq!(borrow, 0); // result < 2^n.

            // If all subtracted bits were zero, we have to get rid of the
            // materialized minuend_msd again.
            z[last as usize] = result_msd & (minuend_msd - 1);
        }
    }

    // Returns -1 when the operation would return X unchanged.
    pub fn AsIntNResultLength(x: Digits, x_negative: bool, n: i32) -> i32 {
        let needed_digits = div_ceil(n, kDigitBits);

        // Generally: decide based on number of digits, and bits in the top digit.
        if x.len() < needed_digits as usize { return -1; }
        if x.len() > needed_digits as usize { return needed_digits; }

        let top_digit = x[needed_digits as usize - 1];
        let compare_digit: digit_t = 1 << ((n - 1) % kDigitBits as i32);

        if top_digit < compare_digit { return -1; }
        if top_digit > compare_digit { return needed_digits; }

        // Special case: if X == -2**(n-1), truncation is a no-op.
        if !x_negative { return needed_digits; }

        for i in (0..needed_digits - 1).rev() {
            if x[i as usize] != 0 { return needed_digits; }
        }

        return -1;
    }

    pub fn AsIntN(z: RWDigits, x: Digits, x_negative: bool, n: i32) -> bool {
        assert!(x.len() > 0);
        assert!(n > 0);
        assert!(AsIntNResultLength(x, x_negative, n) > 0);

        let needed_digits = div_ceil(n, kDigitBits);
        let top_digit = x[needed_digits as usize - 1];
        let compare_digit: digit_t = 1 << ((n - 1) % kDigitBits as i32);

        let has_bit = (top_digit & compare_digit) == compare_digit;
        if !has_bit {
            TruncateToNBits(z, x, n);
            return x_negative;
        }

        TruncateAndSubFromPowerOfTwo(z, x, n);
        if !x_negative { return true; }

        if (top_digit & (compare_digit - 1)) != 0 { return false; }
        for i in (0..needed_digits - 1).rev() {
            if x[i as usize] != 0 { return false; }
        }

        return true;
    }

    // Returns -1 when the operation would return X unchanged.
    pub fn AsUintN_Pos_ResultLength(x: Digits, n: i32) -> i32 {
        let needed_digits = div_ceil(n, kDigitBits);
        if x.len() < needed_digits as usize { return -1; }
        if x.len() > needed_digits as usize { return needed_digits; }

        let bits_in_top_digit = n % kDigitBits;
        if bits_in_top_digit == 0 { return -1; }

        let top_digit = x[needed_digits as usize - 1];
        if (top_digit >> bits_in_top_digit) == 0 { return -1; }

        return needed_digits;
    }

    pub fn AsUintN_Pos(z: RWDigits, x: Digits, n: i32) {
        assert!(AsUintN_Pos_ResultLength(x, n) > 0);
        TruncateToNBits(z, x, n);
    }

    pub fn AsUintN_Neg(z: RWDigits, x: Digits, n: i32) {
        TruncateAndSubFromPowerOfTwo(z, x, n);
    }

    // Placeholder for Add function.  Needs implementation according to BigInt
    // internal representation.
    pub fn Add(z: RWDigits, value: digit_t) {
        let mut carry: digit_t = value;
        for i in 0..z.len() {
            let (sum, overflow) = z[i].overflowing_add(carry);
            z[i] = sum;
            if overflow {
                carry = 1;
            } else {
                carry = 0;
                break;
            }
        }
        if carry != 0 {
            // Handle overflow if necessary
        }
    }

}
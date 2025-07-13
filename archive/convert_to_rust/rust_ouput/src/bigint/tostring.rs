// Converted from V8 C++ source files:
// Header: N/A
// Implementation: tostring.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bigint {
    pub mod bigint_internal {
        // Dummy implementations
        pub fn bit_length(_digits: usize) -> u32 {
            0
        }
    }
    pub mod digit_arithmetic {
        // Dummy implementations
        pub fn count_trailing_zeros(_x: i32) -> u32 {
            0
        }
        pub fn count_leading_zeros(_x: u64) -> u32 {
            0
        }
    }
    pub mod div_helpers {
        // Dummy implementations
        pub fn invert_scratch_space(_x: i32) -> i32 {
            0
        }
        pub fn divide_barrett_scratch_space(_x: i32) -> i32 {
            0
        }
    }
    pub mod util {
        // Dummy implementations
        pub fn is_power_of_two(_x: i32) -> bool {
            false
        }
    }
    pub mod vector_arithmetic {
        // Dummy implementations
        pub fn left_shift<T>(_dst: T, _src: T, _shift: i32) {}
        pub fn right_shift<T>(_dst: T, _src: T, _shift: i32) {}
        pub fn compare<T>(_x: T, _y: T) -> i32 {
            0
        }
    }
    use std::mem::MaybeUninit;

    const K_TO_STRING_FAST_THRESHOLD: i32 = 64;
    const K_DIGIT_BITS: i32 = 64;
    const K_HALF_DIGIT_BITS: i32 = 32;
    const K_HALF_DIGIT_MASK: u64 = 0xFFFFFFFF;
    const K_STRING_ZAP_VALUE: char = 'z';

    pub type DigitT = u64;

    #[derive(Debug, Clone)]
    pub struct Digits<'a> {
        digits: &'a [DigitT],
    }

    impl<'a> Digits<'a> {
        pub fn new(digits: &'a [DigitT]) -> Self {
            Digits { digits }
        }
        pub fn len(&self) -> usize {
            self.digits.len()
        }
        pub fn is_empty(&self) -> bool {
            self.digits.is_empty()
        }
        pub fn get(&self, index: usize) -> Option<&DigitT> {
            self.digits.get(index)
        }
        pub fn msd(&self) -> DigitT {
            if self.is_empty() {
                0
            } else {
                *self.digits.last().unwrap()
            }
        }
        pub fn normalize(&mut self) {}
        pub fn digits(&self) -> &[DigitT] {
            self.digits
        }
    }

    impl<'a> std::ops::Index<usize> for Digits<'a> {
        type Output = DigitT;

        fn index(&self, index: usize) -> &Self::Output {
            &self.digits[index]
        }
    }
    impl<'a> std::ops::Add<usize> for Digits<'a> {
        type Output = Digits<'a>;

        fn add(self, offset: usize) -> Self::Output {
            Digits {
                digits: &self.digits[offset..],
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct RWDigits<'a> {
        digits: &'a mut [DigitT],
    }

    impl<'a> RWDigits<'a> {
        pub fn new(digits: &'a mut [DigitT]) -> Self {
            RWDigits { digits }
        }
        pub fn len(&self) -> usize {
            self.digits.len()
        }
        pub fn is_empty(&self) -> bool {
            self.digits.is_empty()
        }
        pub fn get(&self, index: usize) -> Option<&DigitT> {
            self.digits.get(index)
        }
        pub fn msd(&self) -> DigitT {
            if self.is_empty() {
                0
            } else {
                *self.digits.last().unwrap()
            }
        }
        pub fn normalize(&mut self) {}
        pub fn trim_one(&mut self) {
            if !self.is_empty() {}
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = DigitT;

        fn index(&self, index: usize) -> &Self::Output {
            &self.digits[index]
        }
    }
    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.digits[index]
        }
    }
    pub struct ScratchDigits {
        digits: Vec<DigitT>,
    }

    impl ScratchDigits {
        pub fn new(len: usize) -> Self {
            ScratchDigits {
                digits: vec![0; len],
            }
        }
        pub fn len(&self) -> usize {
            self.digits.len()
        }
        pub fn is_empty(&self) -> bool {
            self.digits.is_empty()
        }
        pub fn get(&self, index: usize) -> Option<&DigitT> {
            self.digits.get(index)
        }
        pub fn msd(&self) -> DigitT {
            if self.is_empty() {
                0
            } else {
                *self.digits.last().unwrap()
            }
        }
        pub fn normalize(&mut self) {}
        pub fn digits(&mut self) -> &mut [DigitT] {
            &mut self.digits
        }
        pub fn resize(&mut self, new_len: usize) {
            self.digits.resize(new_len, 0);
        }
    }
    impl std::ops::Index<usize> for ScratchDigits {
        type Output = DigitT;

        fn index(&self, index: usize) -> &Self::Output {
            &self.digits[index]
        }
    }
    impl std::ops::IndexMut<usize> for ScratchDigits {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.digits[index]
        }
    }

    pub struct ShiftedDigits<'a> {
        digits: Digits<'a>,
        shift: i32,
        allow_inplace_modification: bool,
    }
    impl<'a> ShiftedDigits<'a> {
        pub fn new(
            digits: Digits<'a>,
            shift: i32,
            allow_inplace_modification: bool,
        ) -> Self {
            ShiftedDigits {
                digits,
                shift,
                allow_inplace_modification,
            }
        }
        pub fn reset(&mut self) {}
    }
    #[allow(dead_code)]
    struct Storage {
        data: Vec<DigitT>,
    }

    impl Storage {
        fn new(size: usize) -> Self {
            Storage { data: vec![0; size] }
        }

        fn get(&mut self) -> &mut [DigitT] {
            &mut self.data
        }
    }

    // Raises {base} to the power of {exponent}. Does not check for overflow.
    fn digit_pow(base: DigitT, exponent: DigitT) -> DigitT {
        let mut result = 1;
        let mut b = base;
        let mut e = exponent;
        while e > 0 {
            if e & 1 != 0 {
                result = result.wrapping_mul(b);
            }
            e >>= 1;
            b = b.wrapping_mul(b);
        }
        result
    }

    // Compile-time version of the above.
    const fn digit_pow_rec(base: DigitT, exponent: DigitT) -> DigitT {
        if exponent == 1 {
            base
        } else {
            base.wrapping_mul(digit_pow_rec(base, exponent - 1))
        }
    }
    // Lookup table for the maximum number of bits required per character of a
    // base-N string representation of a number. To increase accuracy, the array
    // value is the actual value multiplied by 32. To generate this table:
    // for (var i = 0; i <= 36; i++) { print(Math.ceil(Math.log2(i) * 32) + ","); }
    const K_MAX_BITS_PER_CHAR: [u8; 37] = [
        0, 0, 32, 51, 64, 75, 83, 90, 96, 102, 107, 111, 115, 119, 122, 126, 128, 131, 134, 136,
        139, 141, 143, 145, 147, 149, 151, 153, 154, 156, 158, 159, 160, 162, 163, 165, 166,
    ];

    const K_BITS_PER_CHAR_TABLE_SHIFT: i32 = 5;
    const K_BITS_PER_CHAR_TABLE_MULTIPLIER: u32 = 1 << K_BITS_PER_CHAR_TABLE_SHIFT;

    const K_CONVERSION_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

    // A variant of ToStringFormatter::BasecaseLast, specialized for a radix
    // known at compile-time.
    fn basecase_fixed_last<const RADIX: i32>(chunk: DigitT, out: &mut [u8]) -> usize {
        let mut current_chunk = chunk;
        let mut current_out_idx = out.len();
        while current_chunk != 0 {
            assert!(out[current_out_idx - 1] as char == K_STRING_ZAP_VALUE);
            if RADIX <= 10 {
                current_out_idx -= 1;
                out[current_out_idx] = b'0' + (current_chunk % RADIX as u64) as u8;
            } else {
                current_out_idx -= 1;
                out[current_out_idx] = K_CONVERSION_CHARS[(current_chunk % RADIX as u64) as usize];
            }
            current_chunk /= RADIX as u64;
        }
        current_out_idx
    }

    // By making {radix} a compile-time constant and computing {chunk_divisor}
    // as another compile-time constant from it, we allow the compiler to emit
    // an optimized instruction sequence based on multiplications with "magic"
    // numbers (modular multiplicative inverses) instead of actual divisions.
    // The price we pay is having to work on half digits; the technique doesn't
    // work with twodigit_t-by-digit_t divisions.
    // Includes an equivalent of ToStringFormatter::BasecaseMiddle, accordingly
    // specialized for a radix known at compile time.
    fn divide_by_magic<const RADIX: DigitT>(
        rest: &mut RWDigits,
        input: &Digits,
        output: &mut [u8],
    ) -> usize {
        let max_bits_per_char = K_MAX_BITS_PER_CHAR[RADIX as usize];
        let chunk_chars =
            K_HALF_DIGIT_BITS as u32 * K_BITS_PER_CHAR_TABLE_MULTIPLIER / max_bits_per_char as u32;
        let chunk_divisor = digit_pow_rec(RADIX, chunk_chars as u64);
        let mut remainder = 0;
        for i in (0..input.len()).rev() {
            let d = input[i];
            let upper = (remainder << K_HALF_DIGIT_BITS) | (d >> K_HALF_DIGIT_BITS);
            let u_result = upper / chunk_divisor;
            remainder = upper % chunk_divisor;
            let lower = (remainder << K_HALF_DIGIT_BITS) | (d & K_HALF_DIGIT_MASK);
            let l_result = lower / chunk_divisor;
            remainder = lower % chunk_divisor;
            rest[i] = (u_result << K_HALF_DIGIT_BITS) | l_result;
        }
        // {remainder} is now the current chunk to be written out.
        let mut output_idx = output.len();
        for _i in 0..chunk_chars {
            assert!(output[output_idx - 1] as char == K_STRING_ZAP_VALUE);
            if RADIX <= 10 {
                output_idx -= 1;
                output[output_idx] = b'0' + (remainder % RADIX as u64) as u8;
            } else {
                output_idx -= 1;
                output[output_idx] = K_CONVERSION_CHARS[(remainder % RADIX as u64) as usize];
            }
            remainder /= RADIX as u64;
        }
        assert!(remainder == 0);
        output_idx
    }
    struct RecursionLevel {}
    impl RecursionLevel {
        fn create_levels(
            _base_divisor: DigitT,
            _base_char_count: i32,
            _target_bit_length: i32,
            _processor: &mut ProcessorImpl,
        ) -> Option<Box<RecursionLevel>> {
            Some(Box::new(RecursionLevel {}))
        }
    }
    // The classic algorithm must check for interrupt requests if no faster
    // algorithm is available.
    // The classic algorithm must check for interrupt requests if no faster
    // algorithm is available.
    struct ToStringFormatter<'a> {
        digits: Digits<'a>,
        radix: i32,
        max_bits_per_char: i32,
        chunk_chars: i32,
        sign: bool,
        out_start: *mut u8,
        out_end: *mut u8,
        out: *mut u8,
        chunk_divisor: DigitT,
        processor: &'a mut ProcessorImpl,
    }
    impl<'a> ToStringFormatter<'a> {
        fn new(
            digits: Digits<'a>,
            radix: i32,
            sign: bool,
            out: *mut u8,
            chars_available: u32,
            processor: &'a mut ProcessorImpl,
        ) -> Self {
            unsafe {
                let mut formatter = ToStringFormatter {
                    digits,
                    radix,
                    max_bits_per_char: 0,
                    chunk_chars: 0,
                    sign,
                    out_start: out,
                    out_end: out.add(chars_available as usize),
                    out: out.add(chars_available as usize),
                    chunk_divisor: 0,
                    processor,
                };
                //formatter.digits.normalize(); // Fixme
                assert!(chars_available >= ToStringResultLength(&formatter.digits, radix, sign));
                formatter
            }
        }

        fn start(&mut self) {
            self.max_bits_per_char = K_MAX_BITS_PER_CHAR[self.radix as usize] as i32;
            self.chunk_chars =
                (K_DIGIT_BITS as u32 * K_BITS_PER_CHAR_TABLE_MULTIPLIER / self.max_bits_per_char as u32)
                    as i32;
            self.chunk_divisor = digit_pow(self.radix as u64, self.chunk_chars as u64);
            // By construction of chunk_chars_, there can't have been overflow.
            assert!(self.chunk_divisor != 0);
        }

        fn finish(&mut self) -> i32 {
            unsafe {
                assert!(self.out as *const u8 >= self.out_start as *const u8);
                assert!(self.out as *const u8 < self.out_end as *const u8); // At least one character was written.
                let mut current_out = self.out;
                while current_out < self.out_end && *current_out == b'0' {
                    current_out = current_out.add(1);
                }
                if self.sign {
                    self.out = self.out.sub(1);
                    *self.out = b'-';
                }
                let mut excess = 0;
                if self.out > self.out_start {
                    let actual_length = self.out_end as usize - self.out as usize;
                    excess = self.out as usize - self.out_start as usize;
                    std::ptr::copy(self.out, self.out_start, actual_length);
                }
                excess as i32
            }
        }
        fn classic(&mut self) {
            if self.digits.len() == 0 {
                unsafe {
                    self.out = self.out.sub(1);
                    *self.out = b'0';
                }
                return;
            }
            if self.digits.len() == 1 {
                unsafe {
                    self.out = self.basecase_last(self.digits[0], self.out);
                }
                return;
            }
            // {rest} holds the part of the BigInt that we haven't looked at yet.
            // Not to be confused with "remainder"!
            let mut rest = ScratchDigits::new(self.digits.len());
            // In the first round, divide the input, allocating a new BigInt for
            // the result == rest; from then on divide the rest in-place.
            let mut dividend = self.digits.clone();
            while rest.len() > 1 {
                unsafe {
                    if self.radix == 10 {
                        // Faster but costs binary size, so we optimize the most common case.
                        self.out = self.divide_by_magic_10(
                            &mut RWDigits::new(rest.digits()),
                            &dividend,
                            self.out,
                        );
                        self.processor.add_work_estimate(rest.len() * 2);
                    } else {
                        let mut chunk: DigitT = 0;
                        self.processor
                            .divide_single(&mut RWDigits::new(rest.digits()), &mut chunk, &dividend, self.chunk_divisor);
                        self.out = self.basecase_middle(chunk, self.out);
                        // Assume that a division is about ten times as expensive as a
                        // multiplication.
                        self.processor.add_work_estimate(rest.len() * 10);
                    }
                    if self.processor.should_terminate() {
                        return;
                    }
                    rest.normalize(); // Fixme
                    dividend = Digits::new(rest.digits());
                }
            }
            unsafe {
                self.out = self.basecase_last(rest[0], self.out);
            }
        }

        unsafe fn basecase_last(&mut self, digit: DigitT, out: *mut u8) -> *mut u8 {
            if self.radix == 10 {
                let out_idx = basecase_fixed_last::<10>(digit, std::slice::from_raw_parts_mut(self.out_start, self.out_end as usize - self.out_start as usize));
                self.out_start.add(out_idx)
            } else {
                let mut current_digit = digit;
                let mut current_out = out;
                loop {
                    assert!(*current_out.sub(1) as char == K_STRING_ZAP_VALUE);
                    current_out = current_out.sub(1);
                    *current_out = K_CONVERSION_CHARS[(current_digit % self.radix as u64) as usize];
                    current_digit /= self.radix as u64;
                    if current_digit == 0 {
                        break;
                    }
                }
                current_out
            }
        }
        unsafe fn divide_by_magic_10(&mut self, rest: &mut RWDigits, dividend: &Digits, out: *mut u8) -> *mut u8 {
            let out_idx = divide_by_magic::<10>(rest, dividend, std::slice::from_raw_parts_mut(self.out_start, self.out_end as usize - self.out_start as usize));
            self.out_start.add(out_idx)
        }

        // When processing a middle (non-most significant) digit, always write the
        // same number of characters (as many '0' as necessary).
        unsafe fn basecase_middle(&mut self, digit: DigitT, out: *mut u8) -> *mut u8 {
            let mut current_digit = digit;
            let mut current_out = out;

            for _i in 0..self.chunk_chars {
                assert!(*current_out.sub(1) as char == K_STRING_ZAP_VALUE);
                current_out = current_out.sub(1);
                *current_out = K_CONVERSION_CHARS[(current_digit % self.radix as u64) as usize];
                current_digit /= self.radix as u64;
            }

            assert!(current_digit == 0);
            current_out
        }
        fn base_power_of_two(&mut self) {
            let bits_per_char = digit_arithmetic::count_trailing_zeros(self.radix) as i32;
            let char_mask = self.radix - 1;
            let mut digit: DigitT = 0;
            // Keeps track of how many unprocessed bits there are in {digit}.
            let mut available_bits: i32 = 0;
            for i in 0..self.digits.len() - 1 {
                let new_digit = self.digits[i];
                // Take any leftover bits from the last iteration into account.
                let current = (digit | (new_digit << available_bits)) & char_mask as u64;
                unsafe {
                    self.out = self.out.sub(1);
                    *self.out = K_CONVERSION_CHARS[current as usize];
                }
                let consumed_bits = bits_per_char - available_bits;
                digit = new_digit >> consumed_bits;
                available_bits = K_DIGIT_BITS - consumed_bits;
                while available_bits >= bits_per_char {
                    unsafe {
                        self.out = self.out.sub(1);
                        *self.out = K_CONVERSION_CHARS[(digit & char_mask as u64) as usize];
                    }
                    digit >>= bits_per_char;
                    available_bits -= bits_per_char;
                }
            }
            // Take any leftover bits from the last iteration into account.
            let msd = self.digits.msd();
            let current = (digit | (msd << available_bits)) & char_mask as u64;
            unsafe {
                self.out = self.out.sub(1);
                *self.out = K_CONVERSION_CHARS[current as usize];
            }
            digit = msd >> (bits_per_char - available_bits);
            while digit != 0 {
                unsafe {
                    self.out = self.out.sub(1);
                    *self.out = K_CONVERSION_CHARS[(digit & char_mask as u64) as usize];
                }
                digit >>= bits_per_char;
            }
        }
        fn fast(&mut self) {
            // As a sandbox proofing measure, we round up here. Using {BitLength(digits_)}
            // would be technically optimal, but vulnerable to a malicious worker that
            // uses an in-sandbox corruption primitive to concurrently toggle the MSD bits
            // between the invocations of {CreateLevels} and {ProcessLevel}.
            let target_bit_length = self.digits.len() as i32 * K_DIGIT_BITS;
            let recursion_levels = RecursionLevel::create_levels(
                self.chunk_divisor,
                self.chunk_chars,
                target_bit_length,
                self.processor,
            );
            if self.processor.should_terminate() {
                return;
            }
            unsafe {
                self.out = self.process_level(
                    recursion_levels.as_deref().unwrap(),
                    &self.digits,
                    self.out,
                    true,
                );
            }
        }
        unsafe fn fill_with_zeros(
            &mut self,
            level: &RecursionLevel,
            right_boundary: *mut u8,
            out: *mut u8,
            is_last_on_level: bool,
        ) -> *mut u8 {
            // Fill up with zeros up to the character count expected to be generated
            // on this level; unless this is the left edge of the result.
            if is_last_on_level {
                return out;
            }
            let chunk_chars = if level as *const RecursionLevel as *const () == std::ptr::null() {
                self.chunk_chars
            } else {
                self.chunk_chars
            };
            let end = right_boundary.sub(chunk_chars as usize);
            let mut current_out = out;
            assert!(current_out >= end);
            while current_out > end {
                current_out = current_out.sub(1);
                *current_out = b'0';
            }
            current_out
        }

        unsafe fn process_level(
            &mut self,
            level: &RecursionLevel,
            chunk: &Digits,
            out: *mut u8,
            is_last_on_level: bool,
        ) -> *mut u8 {
            // Step 0: if only one digit is left, bail out to the base case.
            let mut normalized = chunk.clone();
            normalized.normalize(); // Fixme
            if normalized.len() <= 1 {
                let right_boundary = out;
                let mut current_out = out;
                if normalized.len() == 1 {
                    current_out = self.basecase_last(normalized[0], out);
                }
                return self.fill_with_zeros(level, right_boundary, current_out, is_last_on_level);
            }
            out
        }
    }

    #[derive(Debug)]
    pub struct ProcessorImpl {
        status: Status,
    }

    impl ProcessorImpl {
        pub fn new() -> Self {
            ProcessorImpl { status: Status::Ok }
        }

        pub fn add_work_estimate(&mut self, _estimate: usize) {}

        pub fn should_terminate(&self) -> bool {
            self.status != Status::Ok
        }

        pub fn get_and_clear_status(&mut self) -> Status {
            let status = self.status;
            self.status = Status::Ok;
            status
        }

        pub fn to_string(
            &mut self,
            out: *mut u8,
            out_length: &mut u32,
            x: Digits,
            radix: i32,
            sign: bool,
        ) {
            let use_fast_algorithm = x.len() >= K_TO_STRING_FAST_THRESHOLD as usize;
            self.to_string_impl(out, out_length, x, radix, sign, use_fast_algorithm);
        }
        // Factored out so that tests can call it.
        pub fn to_string_impl(
            &mut self,
            out: *mut u8,
            out_length: &mut u32,
            x: Digits,
            radix: i32,
            sign: bool,
            fast: bool,
        ) {
            unsafe {
                for i in 0..*out_length {
                    *out.add(i as usize) = K_STRING_ZAP_VALUE as u8;
                }
                let mut formatter = ToStringFormatter::new(x, radix, sign, out, *out_length, self);
                if util::is_power_of_two(radix) {
                    formatter.base_power_of_two();
                } else if fast {
                    formatter.start();
                    formatter.fast();
                    if self.should_terminate() {
                        return;
                    }
                } else {
                    formatter.start();
                    formatter.classic();
                }
                let excess = formatter.finish();
                *out_length -= excess as u32;

                for i in 0..excess {
                    *out.add(*out_length as usize + i as usize) = 0;
                }
            }
        }

        pub fn divide_single(
            &mut self,
            _rest: &mut RWDigits,
            _chunk: &mut DigitT,
            _dividend: &Digits,
            _chunk_divisor: DigitT,
        ) {
        }
        fn multiply(&mut self, _result: ScratchDigits, _x: Digits, _y: Digits) {}
        fn invert(&mut self, _inverse: RWDigits, _input: Digits, _scratch: ScratchDigits) {}
        fn divide_schoolbook(
            &mut self,
            _left: ScratchDigits,
            _right: ScratchDigits,
            _chunk: Digits,
            _divisor: ScratchDigits,
        ) {
        }
        fn divide_barrett(
            &mut self,
            _left: ScratchDigits,
            _right: ScratchDigits,
            _chunk: Digits,
            _divisor: ScratchDigits,
            _inverse: Digits,
            _scratch: ScratchDigits,
        ) {
        }
    }
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Status {
        Ok,
        Terminate,
    }
    pub struct Processor {}
    impl Processor {
        pub fn to_string(
            &mut self,
            out: *mut u8,
            out_length: &mut u32,
            x: Digits,
            radix: i32,
            sign: bool,
        ) -> Status {
            let impl_ = self as *mut Self as *mut ProcessorImpl;
            let impl_ref = unsafe { &mut *impl_ };
            impl_ref.to_string(out, out_length, x, radix, sign);
            impl_ref.get_and_clear_status()
        }
    }
    pub fn to_string_result_length(x: &Digits, radix: i32, sign: bool) -> u32 {
        let bit_length = bigint_internal::bit_length(x.len());
        let result: u32;
        if util::is_power_of_two(radix) {
            let bits_per_char = digit_arithmetic::count_trailing_zeros(radix) as u32;
            result = (bit_length + bits_per_char - 1) / bits_per_char + if sign { 1 } else { 0 };
        } else {
            // Maximum number of bits we can represent with one character.
            let max_bits_per_char = K_MAX_BITS_PER_CHAR[radix as usize];
            // For estimating the result length, we have to be pessimistic and work with
            // the minimum number of bits one character can represent.
            let min_bits_per_char = max_bits_per_char - 1;
            // Perform the following computation with u64 to avoid overflows.
            let mut chars_required: u64 = bit_length as u64;
            chars_required *= K_BITS_PER_CHAR_TABLE_MULTIPLIER as u64;
            chars_required = (chars_required + min_bits_per_char as u64 - 1) / min_bits_per_char as u64;
            assert!(chars_required < u32::MAX as u64);
            result = chars_required as u32;
        }
        result + if sign { 1 } else { 0 }
    }
} // namespace bigint
} // namespace v8

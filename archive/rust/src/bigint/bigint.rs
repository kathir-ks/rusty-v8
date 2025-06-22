// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod bigint {
    use std::{
        cmp::{max, min},
        mem,
        slice,
    };

    // To play nice with embedders' macros, we define our own DCHECK here.
    // It's only used in this file.
    #[cfg(debug_assertions)]
    macro_rules! bigint_dcheck {
        ($cond:expr) => {
            if !($cond) {
                eprintln!("{}:{}: Assertion failed: {}", file!(), line!(), stringify!($cond));
                std::process::abort();
            }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! bigint_dcheck {
        ($cond:expr) => {
            let _ = $cond;
        };
    }

    pub(crate) use bigint_dcheck;

    // The type of a digit: a register-width unsigned integer.
    pub type digit_t = usize;
    pub type signed_digit_t = isize;

    #[cfg(target_pointer_width = "32")]
    pub type twodigit_t = u64;

    #[cfg(target_pointer_width = "32")]
    pub const HAVE_TWODIGIT_T: bool = true;

    #[cfg(target_pointer_width = "32")]
    pub const kLog2DigitBits: i32 = 5;

    #[cfg(target_pointer_width = "64")]
    pub const kLog2DigitBits: i32 = 6;

    #[cfg(target_pointer_width = "64")]
    #[cfg(feature = "int128")]
    pub type twodigit_t = u128;

    #[cfg(target_pointer_width = "64")]
    #[cfg(feature = "int128")]
    pub const HAVE_TWODIGIT_T: bool = true;

    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    compile_error!("Unsupported platform.");

    pub const kDigitBits: i32 = 1 << kLog2DigitBits;

    const _: () = assert!(kDigitBits == 8 * mem::size_of::<digit_t>() as i32, "inconsistent type sizes");

    // Describes an array of digits, also known as a BigInt. Unsigned.
    // Does not own the memory it points at, and only gives read-only access to it.
    // Digits are stored in little-endian order.
    #[derive(Debug, Clone, Copy)]
    pub struct Digits<'a> {
        digits_: &'a [digit_t],
        len_: usize,
    }

    impl<'a> Digits<'a> {
        /// This is the constructor intended for public consumption.
        pub fn new(mem: &'a [digit_t]) -> Self {
            Digits {
                digits_: mem,
                len_: mem.len(),
            }
        }

        pub fn from_raw_ptr(mem: *mut digit_t, len: usize) -> Self {
            // Safety: The caller must ensure that the pointer is valid for reads for len * sizeof(digit_t) bytes.
            unsafe {
                Digits {
                    digits_: slice::from_raw_parts(mem, len),
                    len_: len,
                }
            }
        }

        // Provides a "slice" view into another Digits object.
        pub fn slice(&self, offset: usize, len: usize) -> Self {
            bigint_dcheck!(offset <= self.len_);
            let new_len = max(0, min(self.len_ - offset, len));
            Digits {
                digits_: &self.digits_[offset..(offset + new_len)],
                len_: new_len,
            }
        }

        pub fn empty() -> Self {
            Digits {
                digits_: &[],
                len_: 0,
            }
        }

        // Alternative way to get a "slice" view into another Digits object.
        pub fn add(&self, i: usize) -> Self {
            bigint_dcheck!(i <= self.len_);
            Digits {
                digits_: &self.digits_[i..],
                len_: self.len_ - i,
            }
        }

        // Provides access to individual digits.
        pub fn get(&self, i: usize) -> digit_t {
            bigint_dcheck!(i < self.len_);
            self.read_4byte_aligned(i)
        }

        // Convenience accessor for the most significant digit.
        pub fn msd(&self) -> digit_t {
            bigint_dcheck!(self.len_ > 0);
            self.read_4byte_aligned(self.len_ - 1)
        }

        // Checks "pointer equality" (does not compare digits contents).
        pub fn eq(&self, other: &Digits) -> bool {
            std::ptr::eq(self.digits_.as_ptr(), other.digits_.as_ptr()) && self.len_ == other.len_
        }

        // Decrements {len_} until there are no leading zero digits left.
        pub fn normalize(&mut self) {
            while self.len_ > 0 && self.msd() == 0 {
                self.len_ -= 1;
            }
        }

        // Unconditionally drops exactly one leading zero digit.
        pub fn trim_one(&mut self) {
            bigint_dcheck!(self.len_ > 0 && self.msd() == 0);
            self.len_ -= 1;
        }

        pub fn len(&self) -> usize {
            self.len_
        }

        pub fn digits(&self) -> &[digit_t] {
            self.digits_
        }

        // We require externally-provided digits arrays to be 4-byte aligned, but
        // not necessarily 8-byte aligned; so on 64-bit platforms we use memcpy
        // to allow unaligned reads.
        fn read_4byte_aligned(&self, i: usize) -> digit_t {
            if mem::size_of::<digit_t>() == 4 {
                self.digits_[i]
            } else {
                // Safety: digit_t is either u32 or usize, so this is safe
                unsafe { *self.digits_.as_ptr().add(i) }
            }
        }
    }

    // Writable version of a Digits array.
    // Does not own the memory it points at.
    #[derive(Debug)]
    pub struct RWDigits<'a> {
        digits_: &'a mut [digit_t],
        len_: usize,
    }

    impl<'a> RWDigits<'a> {
        pub fn new(mem: &'a mut [digit_t]) -> Self {
            RWDigits {
                digits_: mem,
                len_: mem.len(),
            }
        }

        pub fn slice(&mut self, offset: usize, len: usize) -> Self {
            bigint_dcheck!(offset <= self.len_);
            let new_len = max(0, min(self.len_ - offset, len));
            RWDigits {
                digits_: &mut self.digits_[offset..(offset + new_len)],
                len_: new_len,
            }
        }

        pub fn add(&mut self, i: usize) -> Self {
            bigint_dcheck!(i <= self.len_);
            RWDigits {
                digits_: &mut self.digits_[i..],
                len_: self.len_ - i,
            }
        }

        pub fn len(&self) -> usize {
            self.len_
        }

        pub fn digits(&mut self) -> &mut [digit_t] {
            self.digits_
        }

        pub fn set_len(&mut self, len: usize) {
            self.len_ = len;
        }

        pub fn clear(&mut self) {
            self.digits_.fill(0);
        }

        pub fn get(&self, i: usize) -> digit_t {
            bigint_dcheck!(i < self.len_);
            if mem::size_of::<digit_t>() == 4 {
                self.digits_[i]
            } else {
                unsafe { *self.digits_.as_ptr().add(i) }
            }
        }

        pub fn get_mut(&mut self, i: usize) -> &mut digit_t {
            bigint_dcheck!(i < self.len_);
            &mut self.digits_[i]
        }
    }

    pub trait Platform {
        // If you want the ability to interrupt long-running operations, implement
        // a Platform subclass that overrides this method. It will be queried
        // every now and then by long-running operations.
        fn interrupt_requested(&self) -> bool {
            false
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Status {
        kOk,
        kInterrupted,
    }

    pub struct Processor {
        platform: Box<dyn Platform>,
    }

    impl Processor {
        // Takes ownership of {platform}.
        pub fn new(platform: Box<dyn Platform>) -> Self {
            Processor { platform }
        }

        // Z := X * Y
        pub fn multiply(&mut self, _Z: &mut RWDigits, _X: &Digits, _Y: &Digits) -> Status {
            Status::kOk // Placeholder
        }

        // Q := A / B
        pub fn divide(&mut self, _Q: &mut RWDigits, _A: &Digits, _B: &Digits) -> Status {
            Status::kOk // Placeholder
        }

        // R := A % B
        pub fn modulo(&mut self, _R: &mut RWDigits, _A: &Digits, _B: &Digits) -> Status {
            Status::kOk // Placeholder
        }

        // {out_length} initially contains the allocated capacity of {out}, and
        // upon return will be set to the actual length of the result string.
        pub fn to_string(&mut self, _out: &mut [u8], _out_length: &mut u32, _X: &Digits, _radix: i32, _sign: bool) -> Status {
            Status::kOk // Placeholder
        }

        // Z := the contents of {accumulator}.
        // Assume that this leaves {accumulator} in unusable state.
        pub fn from_string(&mut self, _Z: &mut RWDigits, _accumulator: &mut FromStringAccumulator) -> Status {
            Status::kOk // Placeholder
        }
    }

    // Returns r such that r < 0 if A < B; r > 0 if A > B; r == 0 if A == B.
    // Defined here to be inlineable, which helps ia32 a lot (64-bit platforms
    // don't care).
    #[inline]
    pub fn compare(mut A: Digits, mut B: Digits) -> i32 {
        A.normalize();
        B.normalize();
        let diff = A.len() as i32 - B.len() as i32;
        if diff != 0 {
            return diff;
        }
        let mut i = A.len();
        while i > 0 {
            i -= 1;
            if A.get(i) != B.get(i) {
                return if A.get(i) > B.get(i) { 1 } else { -1 };
            }
        }
        0
    }

    // Z := X + Y
    pub fn add(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    // Addition of signed integers. Returns true if the result is negative.
    pub fn add_signed(Z: &mut RWDigits, X: &Digits, x_negative: bool, Y: &Digits, y_negative: bool) -> bool {
        false // Placeholder
    }

    // Z := X + 1
    pub fn add_one(Z: &mut RWDigits, X: &Digits) {
        // Placeholder
    }

    // Z := X - Y. Requires X >= Y.
    pub fn subtract(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    // Subtraction of signed integers. Returns true if the result is negative.
    pub fn subtract_signed(Z: &mut RWDigits, X: &Digits, x_negative: bool, Y: &Digits, y_negative: bool) -> bool {
        false // Placeholder
    }

    // Z := X - 1
    pub fn subtract_one(Z: &mut RWDigits, X: &Digits) {
        // Placeholder
    }

    // The bitwise operations assume that negative BigInts are represented as
    // sign+magnitude. Their behavior depends on the sign of the inputs: negative
    // inputs perform an implicit conversion to two's complement representation.
    // Z := X & Y
    pub fn bitwise_and_pos_pos(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    // Call this for a BigInt x = (magnitude=X, negative=true).
    pub fn bitwise_and_neg_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    // Positive X, negative Y. Callers must swap arguments as needed.
    pub fn bitwise_and_pos_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_or_pos_pos(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_or_neg_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_or_pos_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_xor_pos_pos(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_xor_neg_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn bitwise_xor_pos_neg(Z: &mut RWDigits, X: &Digits, Y: &Digits) {
        // Placeholder
    }

    pub fn left_shift(Z: &mut RWDigits, X: &Digits, shift: digit_t) {
        // Placeholder
    }

    // RightShiftState is provided by RightShift_ResultLength and used by the actual
    // RightShift to avoid some recomputation.
    #[derive(Default)]
    pub struct RightShiftState {
        pub must_round_down: bool,
    }

    pub fn right_shift(Z: &mut RWDigits, X: &Digits, shift: digit_t, state: &RightShiftState) {
        // Placeholder
    }

    // Z := (least significant n bits of X, interpreted as a signed n-bit integer).
    // Returns true if the result is negative; Z will hold the absolute value.
    pub fn as_intn(Z: &mut RWDigits, X: &Digits, x_negative: bool, n: i32) -> bool {
        false // Placeholder
    }

    // Z := (least significant n bits of X).
    pub fn as_uintn_pos(Z: &mut RWDigits, X: &Digits, n: i32) {
        // Placeholder
    }

    // Same, but X is the absolute value of a negative BigInt.
    pub fn as_uintn_neg(Z: &mut RWDigits, X: &Digits, n: i32) {
        // Placeholder
    }

    #[inline]
    pub fn add_result_length(x_length: i32, y_length: i32) -> i32 {
        max(x_length, y_length) + 1
    }

    #[inline]
    pub fn add_signed_result_length(x_length: i32, y_length: i32, same_sign: bool) -> i32 {
        if same_sign {
            add_result_length(x_length, y_length)
        } else {
            max(x_length, y_length)
        }
    }

    #[inline]
    pub fn subtract_result_length(x_length: i32, y_length: i32) -> i32 {
        x_length
    }

    #[inline]
    pub fn subtract_signed_result_length(x_length: i32, y_length: i32, same_sign: bool) -> i32 {
        if same_sign {
            max(x_length, y_length)
        } else {
            add_result_length(x_length, y_length)
        }
    }

    #[inline]
    pub fn multiply_result_length(X: &Digits, Y: &Digits) -> usize {
        X.len() + Y.len()
    }

    pub const kBarrettThreshold: i32 = 13310;

    #[inline]
    pub fn divide_result_length(A: &Digits, B: &Digits) -> usize {
        let kBarrettExtraScratch = 0;

        A.len() - B.len() + 1 + kBarrettExtraScratch
    }

    #[inline]
    pub fn modulo_result_length(B: &Digits) -> usize {
        B.len()
    }

    pub fn to_string_result_length(_X: &Digits, _radix: i32, _sign: bool) -> u32 {
        0 // Placeholder
    }

    // In DEBUG builds, the result of {ToString} will be initialized to this value.
    pub const kStringZapValue: char = '?';

    pub fn right_shift_result_length(_X: &Digits, _x_sign: bool, _shift: digit_t, _state: &mut RightShiftState) -> i32 {
        0 // Placeholder
    }

    // Returns -1 if this "asIntN" operation would be a no-op.
    pub fn as_intn_result_length(_X: &Digits, _x_negative: bool, _n: i32) -> i32 {
        -1 // Placeholder
    }

    // Returns -1 if this "asUintN" operation would be a no-op.
    pub fn as_uintn_pos_result_length(_X: &Digits, _n: i32) -> i32 {
        -1 // Placeholder
    }

    #[inline]
    pub fn as_uintn_neg_result_length(n: i32) -> i32 {
        ((n - 1) / kDigitBits as i32) + 1
    }

    // A container object for all metadata required for parsing a BigInt from
    // a string.
    // Aggressively optimized not to waste instructions for small cases, while
    // also scaling transparently to huge cases.
    // Defined here in the header so that it can be inlined.
    #[derive(Debug)]
    pub struct FromStringAccumulator {
        stack_parts_: [digit_t; kStackParts as usize],
        heap_parts_: Vec<digit_t>,
        max_multiplier_: digit_t,
        last_multiplier_: digit_t,
        max_digits_: i32,
        result_: Result,
        stack_parts_used_: i32,
        inline_everything_: bool,
        radix_: u8,
    }

    impl FromStringAccumulator {
        pub enum Result {
            kOk,
            kMaxSizeExceeded,
        }

        // Step 1: Create a FromStringAccumulator instance. For best performance,
        // stack allocation is recommended.
        // {max_digits} is only used for refusing to grow beyond a given size
        // (see "Step 2" below). It does not cause pre-allocation, so feel free to
        // specify a large maximum.
        // TODO(jkummerow): The limit applies to the number of intermediate chunks,
        // whereas the final result will be slightly smaller (depending on {radix}).
        // So for sufficiently large N, setting max_digits=N here will not actually
        // allow parsing BigInts with N digits. We can fix that if/when anyone cares.
        pub fn new(max_digits: i32) -> Self {
            FromStringAccumulator {
                stack_parts_: [0; kStackParts as usize],
                heap_parts_: Vec::new(),
                max_multiplier_: 0,
                last_multiplier_: 0,
                max_digits_: max(max_digits, kStackParts),
                result_: Result::kOk,
                stack_parts_used_: 0,
                inline_everything_: false,
                radix_: 0,
            }
        }

        // Step 2: Call this method to read all characters.
        // {CharIt} should be a forward iterator and
        // std::iterator_traits<CharIt>::value_type shall be a character type, such as
        // uint8_t or uint16_t. {end} should be one past the last character (i.e.
        // {start == end} would indicate an empty string). Returns the current
        // position when an invalid character is encountered.
        #[inline]
        pub fn parse<'a>(&mut self, start: &'a [u8], radix: digit_t) -> &'a [u8] {
            bigint_dcheck!(2 <= radix && radix <= 36);
            let mut current = start;
            if !self.inline_everything_ && (radix & (radix - 1)) == 0 {
                return self.parse_power_two(start, radix);
            }
            let mut done = false;
            while !done {
                let mut multiplier: digit_t = 1;
                let mut part: digit_t = 0;
                while true {
                    let d: digit_t;
                    if current.is_empty() {
                        done = true;
                        break;
                    }
                    let c = current[0];
                    if c > 127 || kCharValue[c as usize] >= radix as u8 {
                        done = true;
                        break;
                    } else {
                        d = kCharValue[c as usize] as digit_t;
                    }

                    match multiplier.overflowing_mul(radix) {
                        (new_multiplier, false) => {
                            multiplier = new_multiplier;
                            part = part.wrapping_mul(radix).wrapping_add(d);
                            current = &current[1..];
                        }
                        (_, true) => {
                            break;
                        }
                    }

                    if current.is_empty() {
                        done = true;
                        break;
                    }
                }
                if !self.add_part(multiplier, part, done) {
                    return current;
                }
            }
            current
        }

        // Step 3: Check if a result is available, and determine its required
        // allocation size (guaranteed to be <= max_digits passed to the constructor).
        pub fn result(&self) -> &Result {
            &self.result_
        }
        pub fn result_length(&self) -> i32 {
            max(self.stack_parts_used_, self.heap_parts_.len() as i32)
        }

        #[inline]
        fn parse_power_two<'a>(&mut self, start: &'a [u8], radix: digit_t) -> &'a [u8] {
            self.radix_ = radix as u8;
            let char_bits = kCharBits[(radix >> 2) as usize];
            let mut bits_left: i32;
            let mut done = false;
            let mut current = start;
            while !done {
                let mut part: digit_t = 0;
                bits_left = kDigitBits as i32;
                while true {
                    let d: digit_t;
                    if current.is_empty() {
                        done = true;
                        break;
                    }
                    let c = current[0];
                    if c > 127 || kCharValue[c as usize] >= radix as u8 {
                        done = true;
                        break;
                    } else {
                        d = kCharValue[c as usize] as digit_t;
                    }

                    if bits_left < char_bits as i32 {
                        break;
                    }
                    bits_left -= char_bits as i32;
                    part = (part << char_bits) | d;

                    current = &current[1..];
                    if current.is_empty() {
                        done = true;
                        break;
                    }
                }
                if !self.add_part(part) {
                    return current;
                }
            }
            // We use the unused {last_multiplier_} field to
            // communicate how many bits are unused in the last part.
            self.last_multiplier_ = bits_left as digit_t;
            current
        }

        #[inline]
        fn add_part(&mut self, multiplier: digit_t, part: digit_t, is_last: bool) -> bool {
            if is_last {
                self.last_multiplier_ = multiplier;
            } else {
                bigint_dcheck!(self.max_multiplier_ == 0 || self.max_multiplier_ == multiplier);
                self.max_multiplier_ = multiplier;
            }
            self.add_part(part)
        }

        #[inline]
        fn add_part(&mut self, part: digit_t) -> bool {
            if self.stack_parts_used_ < kStackParts {
                self.stack_parts_[self.stack_parts_used_ as usize] = part;
                self.stack_parts_used_ += 1;
                return true;
            }
            if self.heap_parts_.is_empty() {
                // Initialize heap storage. Copy the stack part to make things easier later.
                self.heap_parts_.reserve((kStackParts * 2) as usize);
                for i in 0..kStackParts {
                    self.heap_parts_.push(self.stack_parts_[i as usize]);
                }
            }
            if self.heap_parts_.len() as i32 >= self.max_digits_ {
                self.result_ = Result::kMaxSizeExceeded;
                return false;
            }
            self.heap_parts_.push(part);
            true
        }
    }

    // A space- and time-efficient way to map {2,4,8,16,32} to {1,2,3,4,5}.
    static kCharBits: [u8; 9] = [1, 2, 3, 0, 4, 0, 0, 0, 5];
    static kStackParts: i32 = 8;

    // Numerical value of the first 127 ASCII characters, using 255 as sentinel
    // for "invalid".
    static kCharValue: [u8; 128] = [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255,
    ];
}
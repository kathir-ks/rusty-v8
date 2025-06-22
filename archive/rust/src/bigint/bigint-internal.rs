// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod bigint_internal {
    use crate::bigint::bigint::*;
    use std::fmt;
    use std::process::abort;

    pub const KARATSUBA_THRESHOLD: i32 = 34;
    pub const TOOM_THRESHOLD: i32 = 193;
    pub const FFT_THRESHOLD: i32 = 1500;
    pub const FFT_INNER_THRESHOLD: i32 = 200;

    pub const BURNIKEL_THRESHOLD: i32 = 57;
    pub const NEWTON_INVERSION_THRESHOLD: i32 = 50;
    // kBarrettThreshold is defined in bigint.rs.

    pub const TO_STRING_FAST_THRESHOLD: i32 = 43;
    pub const FROM_STRING_LARGE_THRESHOLD: i32 = 300;

    pub struct ProcessorImpl<'a> {
        work_estimate: usize,
        status: Status,
        platform: &'a mut dyn Platform,
    }

    impl<'a> ProcessorImpl<'a> {
        pub fn new(platform: &'a mut dyn Platform) -> Self {
            ProcessorImpl {
                work_estimate: 0,
                status: Status::Ok,
                platform,
            }
        }

        pub fn get_and_clear_status(&mut self) -> Status {
            let status = self.status;
            self.status = Status::Ok;
            status
        }

        pub fn multiply(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }
        pub fn multiply_single(&mut self, z: RWDigits, x: Digits, y: digit_t) {
            todo!()
        }
        pub fn multiply_schoolbook(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }

        pub fn multiply_karatsuba(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }
        pub fn karatsuba_start(
            &mut self,
            z: RWDigits,
            x: Digits,
            y: Digits,
            scratch: RWDigits,
            k: i32,
        ) {
            todo!()
        }
        pub fn karatsuba_chunk(
            &mut self,
            z: RWDigits,
            x: Digits,
            y: Digits,
            scratch: RWDigits,
        ) {
            todo!()
        }
        pub fn karatsuba_main(
            &mut self,
            z: RWDigits,
            x: Digits,
            y: Digits,
            scratch: RWDigits,
            n: i32,
        ) {
            todo!()
        }

        pub fn divide(&mut self, q: RWDigits, a: Digits, b: Digits) {
            todo!()
        }
        pub fn divide_single(&mut self, q: RWDigits, remainder: &mut digit_t, a: Digits, b: digit_t) {
            todo!()
        }
        pub fn divide_schoolbook(&mut self, q: RWDigits, r: RWDigits, a: Digits, b: Digits) {
            todo!()
        }
        pub fn divide_burnikel_ziegler(&mut self, q: RWDigits, r: RWDigits, a: Digits, b: Digits) {
            todo!()
        }

        pub fn modulo(&mut self, r: RWDigits, a: Digits, b: Digits) {
            todo!()
        }

        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn multiply_toom_cook(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }
        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn toom3_main(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }

        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn multiply_fft(&mut self, z: RWDigits, x: Digits, y: Digits) {
            todo!()
        }

        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn divide_barrett(&mut self, q: RWDigits, r: RWDigits, a: Digits, b: Digits) {
            todo!()
        }
        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn divide_barrett_with_scratch(
            &mut self,
            q: RWDigits,
            r: RWDigits,
            a: Digits,
            b: Digits,
            i: Digits,
            scratch: RWDigits,
        ) {
            todo!()
        }

        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn invert(&mut self, z: RWDigits, v: Digits, scratch: RWDigits) {
            todo!()
        }
        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn invert_basecase(&mut self, z: RWDigits, v: Digits, scratch: RWDigits) {
            todo!()
        }
        #[cfg(feature = "advanced_bigint_algorithms")]
        pub fn invert_newton(&mut self, z: RWDigits, v: Digits, scratch: RWDigits) {
            todo!()
        }

        pub fn to_string(
            &mut self,
            out: &mut [char],
            out_length: &mut u32,
            x: Digits,
            radix: i32,
            sign: bool,
        ) {
            todo!()
        }
        pub fn to_string_impl(
            &mut self,
            out: &mut [char],
            out_length: &mut u32,
            x: Digits,
            radix: i32,
            sign: bool,
            use_fast_algorithm: bool,
        ) {
            todo!()
        }

        pub fn from_string(&mut self, z: RWDigits, accumulator: &mut FromStringAccumulator) {
            todo!()
        }
        pub fn from_string_classic(&mut self, z: RWDigits, accumulator: &mut FromStringAccumulator) {
            todo!()
        }
        pub fn from_string_large(&mut self, z: RWDigits, accumulator: &mut FromStringAccumulator) {
            todo!()
        }
        pub fn from_string_base_power_of_two(
            &mut self,
            z: RWDigits,
            accumulator: &mut FromStringAccumulator,
        ) {
            todo!()
        }

        pub fn should_terminate(&self) -> bool {
            self.status == Status::Interrupted
        }

        const WORK_ESTIMATE_THRESHOLD: usize = 5_000_000;

        pub fn add_work_estimate(&mut self, estimate: usize) {
            self.work_estimate += estimate;
            if self.work_estimate >= Self::WORK_ESTIMATE_THRESHOLD {
                self.work_estimate = 0;
                if self.platform.interrupt_requested() {
                    self.status = Status::Interrupted;
                }
            }
        }
    }

    impl<'a> Drop for ProcessorImpl<'a> {
        fn drop(&mut self) {
            // No resources to explicitly free in this example.
        }
    }

    pub trait Platform {
        fn interrupt_requested(&self) -> bool;
    }

    pub fn divide_barrett_scratch_space(n: i32) -> i32 {
        n + 2
    }

    const INVERT_NEWTON_EXTRA_SPACE: i32 = 5;
    pub fn invert_newton_scratch_space(n: i32) -> i32 {
        3 * n + 2 * INVERT_NEWTON_EXTRA_SPACE
    }
    pub fn invert_scratch_space(n: i32) -> i32 {
        if n < NEWTON_INVERSION_THRESHOLD {
            2 * n
        } else {
            invert_newton_scratch_space(n)
        }
    }

    #[macro_export]
    macro_rules! CHECK {
        ($cond:expr) => {
            if !($cond) {
                eprintln!("{}:{}: ", file!(), line!());
                eprintln!("Assertion failed: {}", stringify!($cond));
                abort();
            }
        };
    }

    #[macro_export]
    macro_rules! DCHECK {
        ($cond:expr) => {
            if cfg!(debug_assertions) {
                CHECK!($cond);
            } else {
                let _ = (stringify!($cond));
            }
        };
    }

    #[macro_export]
    macro_rules! USE {
        ($var:ident) => {
            let _ = $var;
        };
    }

    pub struct Storage {
        ptr: Vec<digit_t>,
    }

    impl Storage {
        pub fn new(count: usize) -> Self {
            Storage {
                ptr: vec![0; count],
            }
        }

        pub fn get(&mut self) -> *mut digit_t {
            self.ptr.as_mut_ptr()
        }
    }

    pub struct ScratchDigits {
        rw_digits: RWDigits,
        storage: Storage,
    }

    impl ScratchDigits {
        pub fn new(len: usize) -> Self {
            let mut storage = Storage::new(len);
            let digits_ptr = storage.get();
            ScratchDigits {
                rw_digits: RWDigits {
                    digits_: digits_ptr,
                    length_: len,
                },
                storage,
            }
        }
    }

    impl std::ops::Deref for ScratchDigits {
        type Target = RWDigits;

        fn deref(&self) -> &Self::Target {
            &self.rw_digits
        }
    }

    impl std::ops::DerefMut for ScratchDigits {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.rw_digits
        }
    }

} // module bigint_internal
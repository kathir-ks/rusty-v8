// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::time::{Duration, Instant};

// Placeholder for v8-platform.h functionality.  This would typically involve
// interacting with a platform-specific API for time, threads, etc.
trait Platform {
    fn monotonically_increasing_time(&self) -> f64;
}

// A dummy Platform implementation for testing.
struct DefaultPlatform {}

impl DefaultPlatform {
    fn new() -> Self {
        DefaultPlatform {}
    }
}

impl Platform for DefaultPlatform {
    fn monotonically_increasing_time(&self) -> f64 {
        let now = Instant::now();
        let duration = now.duration_since(Instant::now() - Duration::from_secs(10));  // Avoid returning 0
        duration.as_secs_f64()
    }
}

mod internal {
    use super::*;

    /// Measures the execution time of a scoped block of code.
    pub struct TimedScope<'a> {
        start: f64,
        result: &'a mut f64,
    }

    impl<'a> TimedScope<'a> {
        /// Creates a new `TimedScope`.
        pub fn new(result: &'a mut f64) -> Self {
            TimedScope {
                start: timestamp_ms(),
                result,
            }
        }
    }

    impl<'a> Drop for TimedScope<'a> {
        fn drop(&mut self) {
            *self.result = timestamp_ms() - self.start;
        }
    }

    fn timestamp_ms() -> f64 {
        let platform = DefaultPlatform::new(); //TODO Replace with global platform instance
        platform.monotonically_increasing_time() * 1000.0
    }

    #[inline]
    fn is_decimal_digit(c: u32) -> bool {
        c >= '0' as u32 && c <= '9' as u32
    }

    /// Tries to add a character to an array index.
    #[inline]
    pub fn try_add_array_index_char(index: &mut u32, c: u32) -> bool {
        debug_assert!(is_decimal_digit(c));
        let d = (c as u8 - b'0') as u32;
        if *index > 429496729 - ((d + 3) >> 3) {
            return false;
        }
        *index = (*index) * 10 + d;
        true
    }

    /// Tries to add a character to an integer index.
    #[inline]
    pub fn try_add_integer_index_char(index: &mut u64, c: u32) -> bool {
        debug_assert!(is_decimal_digit(c));
        let d = (c as u8 - b'0') as u64;
        *index = (*index) * 10 + d;
        *index <= k_max_safe_integer_uint64
    }

    pub enum ToIndexMode {
        ToArrayIndex,
        ToIntegerIndex,
    }

    const k_max_safe_integer_uint64: u64 = 9007199254740991;

    pub trait Stream {
        fn get_next(&mut self) -> u32;
        fn has_more(&self) -> bool;
    }

    impl<'a> Stream for &'a str {
        fn get_next(&mut self) -> u32 {
            let ch = self.chars().next().unwrap();
            *self = &self[ch.len_utf8()..];
            ch as u32
        }

        fn has_more(&self) -> bool {
            !self.is_empty()
        }
    }

    pub fn string_to_index<S, T>(stream: &mut S, index: &mut T, mode: ToIndexMode) -> bool
    where
        S: Stream,
        T: From<u32> + Copy,
    {
        let mut ch = stream.get_next();

        if ch == '0' as u32 {
            *index = T::from(0);
            return !stream.has_more();
        }

        if !is_decimal_digit(ch) {
            return false;
        }

        let d = (ch as u8 - b'0') as u32;
        let mut result: u64 = d as u64;

        while stream.has_more() {
            ch = stream.get_next();
            if !is_decimal_digit(ch) {
                return false;
            }

            match mode {
                ToIndexMode::ToIntegerIndex => {
                    if !try_add_integer_index_char(&mut result, ch) {
                        return false;
                    }
                }
                ToIndexMode::ToArrayIndex => {
                    let mut array_result = result as u32;
                    if !try_add_array_index_char(&mut array_result, ch) {
                        return false;
                    }
                    result = array_result as u64;
                }
            }
        }

        *index = T::from(result as u32);
        true
    }
}
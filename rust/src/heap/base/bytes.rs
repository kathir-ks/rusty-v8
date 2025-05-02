// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::{max, min};
use std::f64::consts::E;
use std::ops::{Add, AddAssign};

use v8::base::platform::time::TimeDelta;
use v8::base::ring_buffer::RingBuffer;

pub mod v8 {
    pub mod base {
        pub mod platform {
            pub mod time {
                #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
                pub struct TimeDelta {
                    millis: f64,
                }

                impl TimeDelta {
                    pub fn from_milliseconds(millis: f64) -> Self {
                        TimeDelta { millis }
                    }

                    pub fn as_milliseconds_f64(&self) -> f64 {
                        self.millis
                    }

                    pub fn is_zero(&self) -> bool {
                        self.millis == 0.0
                    }
                }

                impl Add for TimeDelta {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        TimeDelta {
                            millis: self.millis + other.millis,
                        }
                    }
                }

                impl AddAssign for TimeDelta {
                    fn add_assign(&mut self, other: Self) {
                        self.millis += other.millis;
                    }
                }
            }
        }

        pub mod ring_buffer {
            use std::collections::VecDeque;

            #[derive(Debug)]
            pub struct RingBuffer<T> {
                buffer: VecDeque<T>,
                capacity: usize,
            }

            impl<T: Copy> RingBuffer<T> {
                pub fn new(capacity: usize) -> Self {
                    RingBuffer {
                        buffer: VecDeque::with_capacity(capacity),
                        capacity,
                    }
                }

                pub fn push(&mut self, item: T) {
                    if self.buffer.len() == self.capacity {
                        self.buffer.pop_front();
                    }
                    self.buffer.push_back(item);
                }

                pub fn reduce<F>(&self, f: F, initial: T) -> T
                where
                    F: Fn(&T, &T) -> T,
                {
                    let mut accumulator = initial;
                    for item in &self.buffer {
                        accumulator = f(&accumulator, item);
                    }
                    accumulator
                }
            }
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct BytesAndDuration {
    pub bytes: usize,
    pub duration: v8::base::platform::time::TimeDelta,
}

impl BytesAndDuration {
    pub const fn new(bytes: usize, duration: v8::base::platform::time::TimeDelta) -> Self {
        BytesAndDuration { bytes, duration }
    }
}

pub type BytesAndDurationBuffer = v8::base::ring_buffer::RingBuffer<BytesAndDuration>;

/// Returns the average speed of events recorded in `buffer` including an
/// `initial` event in Bytes/ms. If provided, `selected_duration` will bound the
/// events considered (which uses the order of events in
/// `BytesAndDurationBuffer`). The bounds are in Bytes/ms and can be used to
/// bound non-zero speeds.
pub fn average_speed(
    buffer: &BytesAndDurationBuffer,
    initial: &BytesAndDuration,
    selected_duration: Option<v8::base::platform::time::TimeDelta>,
    min_non_empty_speed: usize,
    max_speed: usize,
) -> Option<f64> {
    let sum = buffer.reduce(
        |a, b| {
            if let Some(duration) = selected_duration {
                if a.duration >= duration {
                    return *a;
                }
            }
            BytesAndDuration {
                bytes: a.bytes + b.bytes,
                duration: a.duration + b.duration,
            }
        },
        *initial,
    );
    let duration = sum.duration;
    if duration.is_zero() {
        return None;
    }

    let speed = (sum.bytes as f64) / duration.as_milliseconds_f64();
    Some(max(
        min(speed, max_speed as f64),
        min_non_empty_speed as f64,
    ))
}

pub struct SmoothedBytesAndDuration {
    throughput_: f64,
    decay_: v8::base::platform::time::TimeDelta,
}

impl SmoothedBytesAndDuration {
    pub fn new(decay: v8::base::platform::time::TimeDelta) -> Self {
        SmoothedBytesAndDuration {
            throughput_: 0.0,
            decay_: decay,
        }
    }

    pub fn update(&mut self, bytes_and_duration: BytesAndDuration) {
        if bytes_and_duration.duration.is_zero() {
            return;
        }
        let new_throughput =
            bytes_and_duration.bytes as f64 / bytes_and_duration.duration.as_milliseconds_f64();
        self.throughput_ = new_throughput + self.decay(throughput_ - new_throughput, bytes_and_duration.duration);
    }

    /// Return throughput of memory (in bytes) over time (in millis).
    pub fn get_throughput(&self) -> f64 {
        self.throughput_
    }

    /// Returns throughput decayed as if `delay` passed.
    pub fn get_throughput_with_delay(&self, delay: v8::base::platform::time::TimeDelta) -> f64 {
        self.decay(self.throughput_, delay)
    }

    fn decay(&self, throughput: f64, delay: v8::base::platform::time::TimeDelta) -> f64 {
        throughput * (E).powf(-delay.as_milliseconds_f64() / self.decay_.as_milliseconds_f64())
    }
}
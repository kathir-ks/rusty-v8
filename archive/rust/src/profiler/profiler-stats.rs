// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Module `profiler_stats` mirroring the C++ namespace `v8::internal`.
pub mod profiler_stats {
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Enum `Reason` mirroring the C++ enum `v8::internal::ProfilerStats::Reason`.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Reason {
        TickBufferFull,
        IsolateNotLocked,
        SimulatorFillRegistersFailed,
        NoFrameRegion,
        InCallOrApply,
        NoSymbolizedFrames,
        NullPC,
        NumberOfReasons, // Should be last
    }

    impl Reason {
        const COUNT: usize = Reason::NumberOfReasons as usize;
    }

    /// Struct `ProfilerStats` mirroring the C++ class `v8::internal::ProfilerStats`.
    pub struct ProfilerStats {
        counts_: [AtomicUsize; Reason::COUNT],
    }

    impl ProfilerStats {
        /// Creates a new `ProfilerStats` instance with all counts initialized to 0.
        pub fn new() -> Self {
            ProfilerStats {
                counts_: [(); Reason::COUNT].map(|_| AtomicUsize::new(0)),
            }
        }

        /// Adds 1 to the count for the given `reason`.
        pub fn add_reason(&self, reason: Reason) {
            self.counts_[reason as usize].fetch_add(1, Ordering::Relaxed);
        }

        /// Clears all counts to 0.
        pub fn clear(&self) {
            for i in 0..Reason::COUNT {
                self.counts_[i].store(0, Ordering::Relaxed);
            }
        }

        /// Prints the profiler stats to stdout.
        pub fn print(&self) {
            println!("ProfilerStats:");
            for i in 0..Reason::COUNT {
                println!(
                    "  {:30}\t\t {}",
                    Self::reason_to_string(unsafe { std::mem::transmute(i as u8) }),
                    self.counts_[i].load(Ordering::Relaxed)
                );
            }
        }

        /// Converts a `Reason` to a string representation.
        pub fn reason_to_string(reason: Reason) -> &'static str {
            match reason {
                Reason::TickBufferFull => "kTickBufferFull",
                Reason::IsolateNotLocked => "kIsolateNotLocked",
                Reason::SimulatorFillRegistersFailed => "kSimulatorFillRegistersFailed",
                Reason::NoFrameRegion => "kNoFrameRegion",
                Reason::InCallOrApply => "kInCallOrApply",
                Reason::NoSymbolizedFrames => "kNoSymbolizedFrames",
                Reason::NullPC => "kNullPC",
                Reason::NumberOfReasons => "kNumberOfReasons",
            }
        }
    }
}
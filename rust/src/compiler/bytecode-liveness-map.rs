// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file corresponds to src/compiler/bytecode-liveness-map.h
// and src/compiler/bytecode-liveness-map.cc in the original V8 codebase.

/// Represents the liveness state of registers in a bytecode.
///
/// This is a placeholder struct; a real implementation would need
/// more details about how registers and the accumulator are tracked.
pub struct BytecodeLivenessState {
    register_count: usize,
    accumulator_live: bool,
    registers: Vec<bool>, // represents liveness of registers
}

impl BytecodeLivenessState {
    /// Creates a new `BytecodeLivenessState`.
    pub fn new(register_count: usize) -> Self {
        BytecodeLivenessState {
            register_count,
            accumulator_live: false,
            registers: vec![false; register_count],
        }
    }

    /// Returns the number of registers tracked.
    pub fn register_count(&self) -> usize {
        self.register_count
    }

    /// Returns whether the given register is live.
    pub fn register_is_live(&self, register_index: usize) -> bool {
        if register_index < self.register_count {
            self.registers[register_index]
        } else {
            false // or panic, depending on desired behavior
        }
    }

    /// Sets the liveness of the given register.
    pub fn set_register_live(&mut self, register_index: usize, is_live: bool) {
        if register_index < self.register_count {
            self.registers[register_index] = is_live;
        }
    }

    /// Returns whether the accumulator is live.
    pub fn accumulator_is_live(&self) -> bool {
        self.accumulator_live
    }

    /// Sets the liveness of the accumulator.
    pub fn set_accumulator_live(&mut self, is_live: bool) {
        self.accumulator_live = is_live;
    }
}

/// Converts a `BytecodeLivenessState` to a string representation.
pub fn to_string(liveness: &BytecodeLivenessState) -> String {
    let mut out = String::with_capacity(liveness.register_count() + 1);
    for i in 0..liveness.register_count() {
        if liveness.register_is_live(i) {
            out.push('L');
        } else {
            out.push('.');
        }
    }
    if liveness.accumulator_is_live() {
        out.push('L');
    } else {
        out.push('.');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let mut liveness = BytecodeLivenessState::new(3);
        liveness.set_register_live(0, true);
        liveness.set_accumulator_live(true);
        assert_eq!(to_string(&liveness), "L..L");

        let liveness2 = BytecodeLivenessState::new(2);
        assert_eq!(to_string(&liveness2), "..");

        let mut liveness3 = BytecodeLivenessState::new(1);
        liveness3.set_register_live(0, true);
        assert_eq!(to_string(&liveness3), "L.");
    }
}
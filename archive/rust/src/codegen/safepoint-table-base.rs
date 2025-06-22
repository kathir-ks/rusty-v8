// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod safepoint_table_base {
    /// The offset of the stack slots field in the safepoint table header.
    pub const K_SAFEPOINT_TABLE_STACK_SLOTS_OFFSET: i32 = 0;
    pub type SafepointTableStackSlotsFieldT = u32;

    /// Represents a base safepoint entry.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SafepointEntryBase {
        pc: i32,
        deopt_index: i32,
        trampoline_pc: i32,
    }

    impl SafepointEntryBase {
        pub const K_NO_DEOPT_INDEX: i32 = -1;
        pub const K_NO_TRAMPOLINE_PC: i32 = -1;

        /// Creates a new `SafepointEntryBase`.
        pub fn new(pc: i32, deopt_index: i32, trampoline_pc: i32) -> Self {
            let entry = SafepointEntryBase {
                pc,
                deopt_index,
                trampoline_pc,
            };
            debug_assert!(entry.is_initialized());
            entry
        }

        /// Creates a default `SafepointEntryBase`.
        pub fn default() -> Self {
            SafepointEntryBase {
                pc: 0,
                deopt_index: Self::K_NO_DEOPT_INDEX,
                trampoline_pc: Self::K_NO_TRAMPOLINE_PC,
            }
        }

        /// Checks if the safepoint entry is initialized.
        pub fn is_initialized(&self) -> bool {
            self.pc != 0
        }

        /// Returns the program counter (PC) of the safepoint.
        pub fn pc(&self) -> i32 {
            debug_assert!(self.is_initialized());
            self.pc
        }

        /// Returns the trampoline PC.
        pub fn trampoline_pc(&self) -> i32 {
            self.trampoline_pc
        }

        /// Checks if the safepoint entry has a deoptimization index.
        pub fn has_deoptimization_index(&self) -> bool {
            self.deopt_index != Self::K_NO_DEOPT_INDEX
        }

        /// Returns the deoptimization index of the safepoint entry.
        pub fn deoptimization_index(&self) -> i32 {
            debug_assert!(self.has_deoptimization_index());
            self.deopt_index
        }

        /// Resets the safepoint entry to an uninitialized state.
        pub fn reset(&mut self) {
            self.pc = 0;
        }

        // The `operator==` is automatically implemented by `#[derive(PartialEq)]`
    }

    /// Base class for safepoint table builders.
    pub struct SafepointTableBuilderBase {
        safepoint_table_offset: i32,
    }

    impl SafepointTableBuilderBase {
        const K_NO_SAFEPOINT_TABLE_OFFSET: i32 = -1;

        pub fn new() -> Self {
            SafepointTableBuilderBase {
                safepoint_table_offset: Self::K_NO_SAFEPOINT_TABLE_OFFSET,
            }
        }
        /// Checks if the safepoint table has been emitted.
        pub fn emitted(&self) -> bool {
            self.safepoint_table_offset != Self::K_NO_SAFEPOINT_TABLE_OFFSET
        }

        /// Returns the offset of the safepoint table.
        pub fn safepoint_table_offset(&self) -> i32 {
            debug_assert!(self.emitted());
            self.safepoint_table_offset
        }

        /// Sets the offset of the safepoint table.
        pub fn set_safepoint_table_offset(&mut self, offset: i32) {
            debug_assert!(!self.emitted());
            self.safepoint_table_offset = offset;
            debug_assert!(self.emitted());
        }
    }
}
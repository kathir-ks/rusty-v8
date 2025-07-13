// Converted from V8 C++ source files:
// Header: safepoint-table-base.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

//use crate::base::logging;

//use std::convert::TryInto;

pub mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! CHECK {
            ($x:expr) => {
                if !($x) {
                    panic!("Check failed: {}", stringify!($x));
                }
            };
        }
    }
}

pub mod v8 {
    pub mod internal {

        // Both Turbofan and Malgev safepoint tables store the stack slots as the first
        // data entry in the header.
        pub const K_SAFEPOINT_TABLE_STACK_SLOTS_OFFSET: i32 = 0;
        pub type SafepointTableStackSlotsField_t = u32;

        #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
        pub struct SafepointEntryBase {
            pc_: i32,
            deopt_index_: i32,
            trampoline_pc_: i32,
        }

        impl SafepointEntryBase {
            pub const K_NO_DEOPT_INDEX: i32 = -1;
            pub const K_NO_TRAMPOLINE_PC: i32 = -1;

            pub fn new(pc: i32, deopt_index: i32, trampoline_pc: i32) -> Self {
                let entry = SafepointEntryBase {
                    pc_: pc,
                    deopt_index_: deopt_index,
                    trampoline_pc_: trampoline_pc,
                };
                base::logging::CHECK!(entry.is_initialized());
                entry
            }

            pub fn is_initialized(&self) -> bool {
                self.pc_ != 0
            }

            pub fn pc(&self) -> i32 {
                base::logging::CHECK!(self.is_initialized());
                self.pc_
            }

            pub fn trampoline_pc(&self) -> i32 {
                self.trampoline_pc_
            }

            pub fn has_deoptimization_index(&self) -> bool {
                self.deopt_index_ != SafepointEntryBase::K_NO_DEOPT_INDEX
            }

            pub fn deoptimization_index(&self) -> i32 {
                base::logging::CHECK!(self.has_deoptimization_index());
                self.deopt_index_
            }

            pub fn reset(&mut self) {
                self.pc_ = 0;
            }
        }

        #[derive(Debug, Default)]
        pub struct SafepointTableBuilderBase {
            safepoint_table_offset_: i32,
        }

        impl SafepointTableBuilderBase {
            const K_NO_SAFEPOINT_TABLE_OFFSET: i32 = -1;

            pub fn emitted(&self) -> bool {
                self.safepoint_table_offset_ != SafepointTableBuilderBase::K_NO_SAFEPOINT_TABLE_OFFSET
            }

            pub fn safepoint_table_offset(&self) -> i32 {
                base::logging::CHECK!(self.emitted());
                self.safepoint_table_offset_
            }

            pub fn set_safepoint_table_offset(&mut self, offset: i32) {
                base::logging::CHECK!(!self.emitted());
                self.safepoint_table_offset_ = offset;
                base::logging::CHECK!(self.emitted());
            }
        }
    }
}

// Converted from V8 C++ source files:
// Header: constant-riscv-b.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_upper_case_globals)]

use std::fmt;

// use crate::codegen::riscv::base_constants_riscv::*;
// Mock the base_constants_riscv module
mod base_constants_riscv {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Opcode(pub u32);

    impl Opcode {
        pub const fn new(value: u32) -> Self {
            Opcode(value)
        }
    }

    impl std::ops::BitOr for Opcode {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self {
            Opcode(self.0 | rhs.0)
        }
    }

    impl std::ops::BitOr<u32> for Opcode {
        type Output = Self;

        fn bitor(self, rhs: u32) -> Self {
            Opcode(self.0 | rhs)
        }
    }

    pub const OP: Opcode = Opcode::new(0b0110011);
    pub const OP_32: Opcode = Opcode::new(0b0110011);
    pub const OP_IMM: Opcode = Opcode::new(0b0010011);
    pub const OP_IMM_32: Opcode = Opcode::new(0b0010011);
    pub const kFunct3Shift: u32 = 12;
    pub const kFunct7Shift: u32 = 25;
    pub const kFunct6Shift: u32 = 26;
    pub const kShamtShift: u32 = 20;
    pub const kImm12Shift: u32 = 0;
}

use base_constants_riscv::*;

mod internal {
    use super::*;

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_ADDUW: Opcode = Opcode::new(OP_32.0 | (0b000 << kFunct3Shift) | (0b0000100 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_SH1ADDUW: Opcode = Opcode::new(OP_32.0 | (0b010 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_SH2ADDUW: Opcode = Opcode::new(OP_32.0 | (0b100 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_SH3ADDUW: Opcode = Opcode::new(OP_32.0 | (0b110 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_SLLIUW: Opcode = Opcode::new(OP_IMM_32.0 | (0b001 << kFunct3Shift) | (0b000010 << kFunct6Shift));

    pub const RO_SH1ADD: Opcode = Opcode::new(OP.0 | (0b010 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    pub const RO_SH2ADD: Opcode = Opcode::new(OP.0 | (0b100 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    pub const RO_SH3ADD: Opcode = Opcode::new(OP.0 | (0b110 << kFunct3Shift) | (0b0010000 << kFunct7Shift));

    pub const RO_ANDN: Opcode = Opcode::new(OP.0 | (0b111 << kFunct3Shift) | (0b0100000 << kFunct7Shift));

    pub const RO_ORN: Opcode = Opcode::new(OP.0 | (0b110 << kFunct3Shift) | (0b0100000 << kFunct7Shift));

    pub const RO_XNOR: Opcode = Opcode::new(OP.0 | (0b100 << kFunct3Shift) | (0b0100000 << kFunct7Shift));

    pub const OP_COUNT: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    pub const RO_CLZ: Opcode = Opcode::new(OP_COUNT.0 | (0b00000 << kShamtShift));

    pub const RO_CTZ: Opcode = Opcode::new(OP_COUNT.0 | (0b00001 << kShamtShift));

    pub const RO_CPOP: Opcode = Opcode::new(OP_COUNT.0 | (0b00010 << kShamtShift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const OP_COUNTW: Opcode = Opcode::new(OP_IMM_32.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_CLZW: Opcode = Opcode::new(OP_COUNTW.0 | (0b00000 << kShamtShift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_CTZW: Opcode = Opcode::new(OP_COUNTW.0 | (0b00001 << kShamtShift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_CPOPW: Opcode = Opcode::new(OP_COUNTW.0 | (0b00010 << kShamtShift));

    pub const RO_MAX: Opcode = Opcode::new(OP.0 | (0b110 << kFunct3Shift) | (0b0000101 << kFunct7Shift));

    pub const RO_MAXU: Opcode = Opcode::new(OP.0 | (0b111 << kFunct3Shift) | (0b0000101 << kFunct7Shift));

    pub const RO_MIN: Opcode = Opcode::new(OP.0 | (0b100 << kFunct3Shift) | (0b0000101 << kFunct7Shift));

    pub const RO_MINU: Opcode = Opcode::new(OP.0 | (0b101 << kFunct3Shift) | (0b0000101 << kFunct7Shift));

    pub const RO_SEXTB: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift) | (0b00100 << kShamtShift));

    pub const RO_SEXTH: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift) | (0b00101 << kShamtShift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_ZEXTH: Opcode = Opcode::new(OP_32.0 | (0b100 << kFunct3Shift) | (0b0000100 << kFunct7Shift) | (0b00000 << kShamtShift));

    #[cfg(all(target_arch = "riscv32", target_pointer_width = "32"))]
    pub const RO_ZEXTH: Opcode = Opcode::new(OP.0 | (0b100 << kFunct3Shift) | (0b0000100 << kFunct7Shift) | (0b00000 << kShamtShift));

    pub const RO_ROL: Opcode = Opcode::new(OP.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    pub const RO_ROR: Opcode = Opcode::new(OP.0 | (0b101 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    pub const RO_ORCB: Opcode = Opcode::new(OP_IMM.0 | (0b101 << kFunct3Shift) | (0b001010000111 << kImm12Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_RORI: Opcode = Opcode::new(OP_IMM.0 | (0b101 << kFunct3Shift) | (0b011000 << kFunct6Shift));

    #[cfg(all(target_arch = "riscv32", target_pointer_width = "32"))]
    pub const RO_RORI: Opcode = Opcode::new(OP_IMM.0 | (0b101 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_ROLW: Opcode = Opcode::new(OP_32.0 | (0b001 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_RORIW: Opcode = Opcode::new(OP_IMM_32.0 | (0b101 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_RORW: Opcode = Opcode::new(OP_32.0 | (0b101 << kFunct3Shift) | (0b0110000 << kFunct7Shift));

    pub const RO_REV8: Opcode = Opcode::new(OP_IMM.0 | (0b101 << kFunct3Shift) | (0b011010 << kFunct6Shift));

    #[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
    pub const RO_REV8_IMM12: u32 = 0b011010111000;

    #[cfg(all(target_arch = "riscv32", target_pointer_width = "32"))]
    pub const RO_REV8_IMM12: u32 = 0b011010011000;

    pub const RO_BCLR: Opcode = Opcode::new(OP.0 | (0b001 << kFunct3Shift) | (0b0100100 << kFunct7Shift));

    pub const RO_BCLRI: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b010010 << kFunct6Shift));

    pub const RO_BEXT: Opcode = Opcode::new(OP.0 | (0b101 << kFunct3Shift) | (0b0100100 << kFunct7Shift));

    pub const RO_BEXTI: Opcode = Opcode::new(OP_IMM.0 | (0b101 << kFunct3Shift) | (0b010010 << kFunct6Shift));

    pub const RO_BINV: Opcode = Opcode::new(OP.0 | (0b001 << kFunct3Shift) | (0b0110100 << kFunct7Shift));

    pub const RO_BINVI: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b011010 << kFunct6Shift));

    pub const RO_BSET: Opcode = Opcode::new(OP.0 | (0b001 << kFunct3Shift) | (0b0010100 << kFunct7Shift));

    pub const RO_BSETI: Opcode = Opcode::new(OP_IMM.0 | (0b001 << kFunct3Shift) | (0b0010100 << kFunct7Shift));
}

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_zicsr {
    // RISCV CSR related bit mask and shift
    pub const K_FCSR_FLAGS_BITS: i32 = 5;
    pub const K_FCSR_FLAGS_MASK: u32 = (1 << K_FCSR_FLAGS_BITS) - 1;
    pub const K_FCSR_FRM_BITS: i32 = 3;
    pub const K_FCSR_FRM_SHIFT: i32 = K_FCSR_FLAGS_BITS;
    pub const K_FCSR_FRM_MASK: u32 = ((1 << K_FCSR_FRM_BITS) - 1) << K_FCSR_FRM_SHIFT;
    pub const K_FCSR_BITS: i32 = K_FCSR_FLAGS_BITS + K_FCSR_FRM_BITS;
    pub const K_FCSR_MASK: u32 = K_FCSR_FLAGS_MASK | K_FCSR_FRM_MASK;

    // RV32/RV64 Zicsr Standard Extension
    pub const RO_CSRRW: Opcode = Opcode::SYSTEM | (0b001 << K_FUNCT3_SHIFT);
    pub const RO_CSRRS: Opcode = Opcode::SYSTEM | (0b010 << K_FUNCT3_SHIFT);
    pub const RO_CSRRC: Opcode = Opcode::SYSTEM | (0b011 << K_FUNCT3_SHIFT);
    pub const RO_CSRRWI: Opcode = Opcode::SYSTEM | (0b101 << K_FUNCT3_SHIFT);
    pub const RO_CSRRSI: Opcode = Opcode::SYSTEM | (0b110 << K_FUNCT3_SHIFT);
    pub const RO_CSRRCI: Opcode = Opcode::SYSTEM | (0b111 << K_FUNCT3_SHIFT);

    // Assuming Opcode and K_FUNCT3_SHIFT are defined elsewhere, similar to base-constants-riscv.h
    // Placeholder definition:
    #[derive(Debug, Copy, Clone)]
    pub struct Opcode(pub u32);

    impl std::ops::BitOr<u32> for Opcode {
        type Output = Self;

        fn bitor(self, rhs: u32) -> Self {
            Opcode(self.0 | rhs)
        }
    }

    pub const SYSTEM: Opcode = Opcode(0b1110011); //Example value, check the correct one
    pub const K_FUNCT3_SHIFT: i32 = 12; // Example value, check the correct one
}
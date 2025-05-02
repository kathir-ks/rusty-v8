// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_a {
    use crate::codegen::riscv::base_constants_riscv::*;

    // RV32A Standard Extension
    pub const RO_LR_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b00010 << kFunct5Shift);
    pub const RO_SC_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b00011 << kFunct5Shift);
    pub const RO_AMOSWAP_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b00001 << kFunct5Shift);
    pub const RO_AMOADD_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b00000 << kFunct5Shift);
    pub const RO_AMOXOR_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b00100 << kFunct5Shift);
    pub const RO_AMOAND_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b01100 << kFunct5Shift);
    pub const RO_AMOOR_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b01000 << kFunct5Shift);
    pub const RO_AMOMIN_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b10000 << kFunct5Shift);
    pub const RO_AMOMAX_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b10100 << kFunct5Shift);
    pub const RO_AMOMINU_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b11000 << kFunct5Shift);
    pub const RO_AMOMAXU_W: Opcode = AMO | (0b010 << kFunct3Shift) | (0b11100 << kFunct5Shift);

    #[cfg(target_arch = "riscv64")]
    pub mod riscv64 {
        use crate::codegen::riscv::base_constants_riscv::*;
        pub const RO_LR_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b00010 << kFunct5Shift);
        pub const RO_SC_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b00011 << kFunct5Shift);
        pub const RO_AMOSWAP_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b00001 << kFunct5Shift);
        pub const RO_AMOADD_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b00000 << kFunct5Shift);
        pub const RO_AMOXOR_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b00100 << kFunct5Shift);
        pub const RO_AMOAND_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b01100 << kFunct5Shift);
        pub const RO_AMOOR_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b01000 << kFunct5Shift);
        pub const RO_AMOMIN_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b10000 << kFunct5Shift);
        pub const RO_AMOMAX_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b10100 << kFunct5Shift);
        pub const RO_AMOMINU_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b11000 << kFunct5Shift);
        pub const RO_AMOMAXU_D: Opcode = AMO | (0b011 << kFunct3Shift) | (0b11100 << kFunct5Shift);
    }
}
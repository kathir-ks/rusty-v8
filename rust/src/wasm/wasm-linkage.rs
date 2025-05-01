// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)] // Suppress warnings about unused code.

// This header should only be included if WebAssembly is enabled.
// This is handled by conditional compilation using features.
#[cfg(not(feature = "webassembly"))]
compile_error!("This code should only be included if WebAssembly is enabled.");

mod codegen {
    pub mod aligned_slot_allocator;
    pub mod assembler_arch;
    pub mod linkage_location;
    pub mod machine_type;
}

use crate::codegen::aligned_slot_allocator::AlignedSlotAllocator;
use crate::codegen::linkage_location::LinkageLocation;
use crate::codegen::machine_type::{ElementSizeInBytes, MachineRepresentation, MachineType, IsFloatingPoint};

// TODO(wasm): optimize calling conventions to be both closer to C++ (to
// reduce adapter costs for fast Wasm <-> C++ calls) and to be more efficient
// in general.

macro_rules! define_arch_registers {
    ($arch:ident, $register:ident, $double_register:ident) => {
        #[cfg(all(target_arch = $arch, feature = "webassembly"))]
        pub mod $arch {
            use super::*;
            use crate::codegen::assembler_arch::Register;
            use crate::codegen::assembler_arch::DoubleRegister;

            pub const kGpParamRegisters: &[Register] = &$register::kGpParamRegisters;
            pub const kGpReturnRegisters: &[Register] = &$register::kGpReturnRegisters;
            pub const kFpParamRegisters: &[DoubleRegister] = &$double_register::kFpParamRegisters;
            pub const kFpReturnRegisters: &[DoubleRegister] = &$double_register::kFpReturnRegisters;
        }
    };
}

macro_rules! define_arch_constants {
    ($arch:ident, $is_fp_always_double:expr, $is_big_endian:expr, $is_big_endian_on_sim:expr) => {
        #[cfg(all(target_arch = $arch, feature = "webassembly"))]
        pub mod $arch {
            pub const kIsFpAlwaysDouble: bool = $is_fp_always_double;
            pub const kIsBigEndian: bool = $is_big_endian;
            pub const kIsBigEndianOnSim: bool = $is_big_endian_on_sim;
        }
    };
}

// Define target architecture register sets using modules.
// TODO: replace with actual register definitions
pub mod ia32 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 4] = [Register(10), Register(11), Register(12), Register(13)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 6] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(x86, ia32, ia32);

pub mod x64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 6] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 6] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(x86_64, x64, x64);

pub mod arm {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 4] = [Register(10), Register(11), Register(12), Register(13)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(arm, arm, arm);

pub mod aarch64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(aarch64, aarch64, aarch64);

pub mod mips64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 7] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(mips64, mips64, mips64);

pub mod loongarch64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(loongarch64, loongarch64, loongarch64);

pub mod ppc64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(powerpc64, ppc64, ppc64);

pub mod s390x {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 4] = [Register(10), Register(11), Register(12), Register(13)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 4] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(s390x, s390x, s390x);

pub mod riscv64 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(riscv64, riscv64, riscv64);

pub mod riscv32 {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 7] = [Register(10), Register(11), Register(12), Register(13), Register(14), Register(15), Register(16)]; // Placeholder values
    pub const kGpReturnRegisters: [Register; 2] = [Register(10), Register(11)]; // Placeholder values
    pub const kFpParamRegisters: [DoubleRegister; 8] = [DoubleRegister(0), DoubleRegister(1), DoubleRegister(2), DoubleRegister(3), DoubleRegister(4), DoubleRegister(5), DoubleRegister(6), DoubleRegister(7)];
    pub const kFpReturnRegisters: [DoubleRegister; 2] = [DoubleRegister(0), DoubleRegister(1)];
}
define_arch_registers!(riscv32, riscv32, riscv32);

// Define unknown target architecture register sets.
pub mod unknown {
    use crate::codegen::assembler_arch::Register;
    use crate::codegen::assembler_arch::DoubleRegister;
    pub const kGpParamRegisters: [Register; 0] = [];
    pub const kGpReturnRegisters: [Register; 0] = [];
    pub const kFpParamRegisters: [DoubleRegister; 0] = [];
    pub const kFpReturnRegisters: [DoubleRegister; 0] = [];
}
define_arch_registers!(unknown, unknown, unknown);

// Define target architecture constants.

define_arch_constants!(powerpc64, true, false, false);
define_arch_constants!(s390x, false, false, false);
define_arch_constants!(s390x, false, false, true);
define_arch_constants!(x86, false, false, false);
define_arch_constants!(x86_64, false, false, false);
define_arch_constants!(arm, false, false, false);
define_arch_constants!(aarch64, false, false, false);
define_arch_constants!(mips64, false, false, false);
define_arch_constants!(loongarch64, false, false, false);
define_arch_constants!(riscv64, false, false, false);
define_arch_constants!(riscv32, false, false, false);

#[cfg(all(not(
    target_arch = "powerpc64"
    ),
    not(target_arch = "s390x"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    not(target_arch = "arm"),
    not(target_arch = "aarch64"),
    not(target_arch = "mips64"),
    not(target_arch = "loongarch64"),
    not(target_arch = "riscv64"),
    not(target_arch = "riscv32"),
    feature = "webassembly"))]
pub mod default_constants {
    pub const kIsFpAlwaysDouble: bool = false;
    pub const kIsBigEndian: bool = false;
    pub const kIsBigEndianOnSim: bool = false;
}

// The parameter index where the trusted instance data should be placed in wasm
// call descriptors. This is used by the Int64Lowering::LowerNode method.
pub const kWasmInstanceDataParameterIndex: i32 = 0;

// TODO: figure out what this register should be.
//static_assert(kWasmImplicitArgRegister ==
//              kGpParamRegisters[kWasmInstanceDataParameterIndex]);

pub struct LinkageAllocator {
    gp_count_: usize,
    gp_offset_: usize,
    gp_regs_: &'static [crate::codegen::assembler_arch::Register],

    fp_count_: usize,
    fp_offset_: usize, // Removed conditional compilation. Implementation will handle this if needed.
    fp_regs_: &'static [crate::codegen::assembler_arch::DoubleRegister],

    slot_allocator_: AlignedSlotAllocator,
}

impl LinkageAllocator {
    pub const fn new(gp: &'static [crate::codegen::assembler_arch::Register], fp: &'static [crate::codegen::assembler_arch::DoubleRegister]) -> Self {
        LinkageAllocator {
            gp_count_: gp.len(),
            gp_offset_: 0,
            gp_regs_: gp,

            fp_count_: fp.len(),
            fp_offset_: 0, // Removed conditional compilation. Implementation will handle this if needed.
            fp_regs_: fp,

            slot_allocator_: AlignedSlotAllocator::new(),
        }
    }

    pub fn can_allocate_gp(&self) -> bool {
        self.gp_offset_ < self.gp_count_
    }

    pub fn can_allocate_fp(&self, rep: MachineRepresentation) -> bool {
        self.fp_offset_ < self.fp_count_
    }

    pub fn next_gp_reg(&mut self) -> i32 {
        assert!(self.gp_offset_ < self.gp_count_);
        let reg_code = self.gp_regs_[self.gp_offset_].code();
        self.gp_offset_ += 1;
        reg_code
    }

    pub fn next_fp_reg(&mut self, rep: MachineRepresentation) -> i32 {
        assert!(self.can_allocate_fp(rep));

        let reg_code = self.fp_regs_[self.fp_offset_].code();
        self.fp_offset_ += 1;
        reg_code
    }

    pub fn next_stack_slot(&mut self, type_: MachineRepresentation) -> i32 {
        let num_slots = AlignedSlotAllocator::num_slots_for_width(ElementSizeInBytes(type_));
        let slot = self.slot_allocator_.allocate(num_slots);
        slot as i32
    }

    pub fn set_stack_offset(&mut self, offset: i32) {
        assert!(offset >= 0);
        assert_eq!(0, self.slot_allocator_.size());
        self.slot_allocator_.allocate_unaligned(offset as usize);
    }

    pub fn num_stack_slots(&self) -> usize {
        self.slot_allocator_.size()
    }

    pub fn end_slot_area(&mut self) {
        self.slot_allocator_.allocate_unaligned(0);
    }
}

pub struct LinkageLocationAllocator {
    allocator_: LinkageAllocator,
    slot_offset_: i32,
}

impl LinkageLocationAllocator {
    pub const fn new(gp: &'static [crate::codegen::assembler_arch::Register], fp: &'static [crate::codegen::assembler_arch::DoubleRegister], slot_offset: i32) -> Self {
        LinkageLocationAllocator {
            allocator_: LinkageAllocator::new(gp, fp),
            slot_offset_: slot_offset,
        }
    }

    pub fn next(&mut self, rep: MachineRepresentation) -> LinkageLocation {
        let type_ = MachineType::TypeForRepresentation(rep);
        if IsFloatingPoint(rep) {
            if self.allocator_.can_allocate_fp(rep) {
                let reg_code = self.allocator_.next_fp_reg(rep);
                return LinkageLocation::ForRegister(reg_code, type_);
            }
        } else if self.allocator_.can_allocate_gp() {
            let reg_code = self.allocator_.next_gp_reg();
            return LinkageLocation::ForRegister(reg_code, type_);
        }
        // Cannot use register; use stack slot.
        let index = -1 - (self.slot_offset_ + self.allocator_.next_stack_slot(rep)) ;
        LinkageLocation::ForCallerFrameSlot(index, type_)
    }

    pub fn num_stack_slots(&self) -> usize {
        self.allocator_.num_stack_slots()
    }

    pub fn end_slot_area(&mut self) {
        self.allocator_.end_slot_area();
    }
}
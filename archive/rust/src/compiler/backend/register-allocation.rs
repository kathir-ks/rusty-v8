// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::codegen::register_configuration::RegisterConfiguration;
//use crate::zone::zone::Zone; // Assuming a Zone-like memory management is needed.
// Assuming these definitions are in separate modules

/// Kind of register.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterKind {
    General,
    Double,
    Simd128,
}

// Define MachineRepresentation enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MachineRepresentation {
    Bit,
    Word8,
    Word16,
    Word32,
    Float16,
    Float32,
    SandboxedPointer,
    TaggedSigned,
    TaggedPointer,
    Tagged,
    CompressedPointer,
    Compressed,
    ProtectedPointer,
    Word64,
    Float64,
    Simd128,
    Simd256,
    None,
    MapWord,
    IndirectPointer,
    Float16RawBits,
}

const K_SYSTEM_POINTER_SIZE: i32 = 8; // Assuming 64-bit architecture
const K_DOUBLE_SIZE: i32 = 8;
const K_SIMD128_SIZE: i32 = 16;
const K_SIMD256_SIZE: i32 = 32;

pub struct RegisterConfiguration {
    num_general_registers: i32,
    num_double_registers: i32,
    num_simd128_registers: i32,
    num_allocatable_general_registers: i32,
    num_allocatable_double_registers: i32,
    num_allocatable_simd128_registers: i32,
    allocatable_general_codes: Vec<i32>,
    allocatable_double_codes: Vec<i32>,
    allocatable_simd128_codes: Vec<i32>,
}

impl RegisterConfiguration {
    pub fn new(
        num_general_registers: i32,
        num_double_registers: i32,
        num_simd128_registers: i32,
        num_allocatable_general_registers: i32,
        num_allocatable_double_registers: i32,
        num_allocatable_simd128_registers: i32,
        allocatable_general_codes: Vec<i32>,
        allocatable_double_codes: Vec<i32>,
        allocatable_simd128_codes: Vec<i32>,
    ) -> Self {
        Self {
            num_general_registers,
            num_double_registers,
            num_simd128_registers,
            num_allocatable_general_registers,
            num_allocatable_double_registers,
            num_allocatable_simd128_registers,
            allocatable_general_codes,
            allocatable_double_codes,
            allocatable_simd128_codes,
        }
    }

    pub fn num_general_registers(&self) -> i32 {
        self.num_general_registers
    }

    pub fn num_double_registers(&self) -> i32 {
        self.num_double_registers
    }

    pub fn num_simd128_registers(&self) -> i32 {
        self.num_simd128_registers
    }

    pub fn num_allocatable_general_registers(&self) -> i32 {
        self.num_allocatable_general_registers
    }

    pub fn num_allocatable_double_registers(&self) -> i32 {
        self.num_allocatable_double_registers
    }

    pub fn num_allocatable_simd128_registers(&self) -> i32 {
        self.num_allocatable_simd128_registers
    }

    pub fn allocatable_general_codes(&self) -> &Vec<i32> {
        &self.allocatable_general_codes
    }

    pub fn allocatable_double_codes(&self) -> &Vec<i32> {
        &self.allocatable_double_codes
    }

    pub fn allocatable_simd128_codes(&self) -> &Vec<i32> {
        &self.allocatable_simd128_codes
    }
}

/// Returns the number of registers for a given kind.
pub fn get_register_count(config: &RegisterConfiguration, kind: RegisterKind) -> i32 {
    match kind {
        RegisterKind::General => config.num_general_registers(),
        RegisterKind::Double => config.num_double_registers(),
        RegisterKind::Simd128 => config.num_simd128_registers(),
    }
}

/// Returns the number of allocatable registers for a given kind.
pub fn get_allocatable_register_count(config: &RegisterConfiguration, kind: RegisterKind) -> i32 {
    match kind {
        RegisterKind::General => config.num_allocatable_general_registers(),
        RegisterKind::Double => config.num_allocatable_double_registers(),
        RegisterKind::Simd128 => config.num_allocatable_simd128_registers(),
    }
}

/// Returns the list of allocatable register codes for a given kind.
pub fn get_allocatable_register_codes(config: &RegisterConfiguration, kind: RegisterKind) -> &Vec<i32> {
    match kind {
        RegisterKind::General => config.allocatable_general_codes(),
        RegisterKind::Double => config.allocatable_double_codes(),
        RegisterKind::Simd128 => config.allocatable_simd128_codes(),
    }
}

/// Returns the byte width for a stack slot based on the machine representation.
pub fn byte_width_for_stack_slot(rep: MachineRepresentation) -> i32 {
    match rep {
        MachineRepresentation::Bit
        | MachineRepresentation::Word8
        | MachineRepresentation::Word16
        | MachineRepresentation::Word32
        | MachineRepresentation::Float16
        | MachineRepresentation::Float32
        | MachineRepresentation::SandboxedPointer => K_SYSTEM_POINTER_SIZE,
        MachineRepresentation::TaggedSigned
        | MachineRepresentation::TaggedPointer
        | MachineRepresentation::Tagged
        | MachineRepresentation::CompressedPointer
        | MachineRepresentation::Compressed
        | MachineRepresentation::ProtectedPointer => {
            // TODO(ishell): kTaggedSize once half size locations are supported.
            K_SYSTEM_POINTER_SIZE
        }
        MachineRepresentation::Word64 | MachineRepresentation::Float64 => K_DOUBLE_SIZE,
        MachineRepresentation::Simd128 => K_SIMD128_SIZE,
        MachineRepresentation::Simd256 => K_SIMD256_SIZE,
        MachineRepresentation::None
        | MachineRepresentation::MapWord
        | MachineRepresentation::IndirectPointer
        | MachineRepresentation::Float16RawBits => {
            panic!("UNREACHABLE"); // Replace with proper error handling.
        }
    }
}
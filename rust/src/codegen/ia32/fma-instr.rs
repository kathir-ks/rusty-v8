// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines FMA instruction lists for IA32 architecture.
///
/// This module provides macro definitions to declare various FMA (Fused Multiply-Add)
/// instructions for different data types (single-precision, double-precision, packed single,
/// and packed double) on the IA32 architecture.  The macros generate lists of instructions
/// that can be further processed to create instruction tables or code generation logic.

macro_rules! fma_sd_instruction_list {
    ($V:ident) => {
        $V!(vfmadd132sd, L128, 0x66, 0x0F, 0x38, W1, 0x99);
        $V!(vfmadd213sd, L128, 0x66, 0x0F, 0x38, W1, 0xA9);
        $V!(vfmadd231sd, L128, 0x66, 0x0F, 0x38, W1, 0xB9);
        $V!(vfmsub132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9B);
        $V!(vfmsub213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAB);
        $V!(vfmsub231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBB);
        $V!(vfnmadd132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9D);
        $V!(vfnmadd213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAD);
        $V!(vfnmadd231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBD);
        $V!(vfnmsub132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9F);
        $V!(vfnmsub213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAF);
        $V!(vfnmsub231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBF);
    };
}

macro_rules! fma_ss_instruction_list {
    ($V:ident) => {
        $V!(vfmadd132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x99);
        $V!(vfmadd213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xA9);
        $V!(vfmadd231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xB9);
        $V!(vfmsub132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9B);
        $V!(vfmsub213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAB);
        $V!(vfmsub231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBB);
        $V!(vfnmadd132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9D);
        $V!(vfnmadd213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAD);
        $V!(vfnmadd231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBD);
        $V!(vfnmsub132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9F);
        $V!(vfnmsub213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAF);
        $V!(vfnmsub231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBF);
    };
}

macro_rules! fma_ps_instruction_list {
    ($V:ident) => {
        $V!(vfmadd132ps, L128, 0x66, 0x0F, 0x38, W0, 0x98);
        $V!(vfmadd213ps, L128, 0x66, 0x0F, 0x38, W0, 0xA8);
        $V!(vfmadd231ps, L128, 0x66, 0x0F, 0x38, W0, 0xB8);
        $V!(vfnmadd132ps, L128, 0x66, 0x0F, 0x38, W0, 0x9C);
        $V!(vfnmadd213ps, L128, 0x66, 0x0F, 0x38, W0, 0xAC);
        $V!(vfnmadd231ps, L128, 0x66, 0x0F, 0x38, W0, 0xBC);
    };
}

macro_rules! fma_pd_instruction_list {
    ($V:ident) => {
        $V!(vfmadd132pd, L128, 0x66, 0x0F, 0x38, W1, 0x98);
        $V!(vfmadd213pd, L128, 0x66, 0x0F, 0x38, W1, 0xA8);
        $V!(vfmadd231pd, L128, 0x66, 0x0F, 0x38, W1, 0xB8);
        $V!(vfnmadd132pd, L128, 0x66, 0x0F, 0x38, W1, 0x9C);
        $V!(vfnmadd213pd, L128, 0x66, 0x0F, 0x38, W1, 0xAC);
        $V!(vfnmadd231pd, L128, 0x66, 0x0F, 0x38, W1, 0xBC);
    };
}

macro_rules! fma_instruction_list {
    ($V:ident) => {
        fma_sd_instruction_list!($V);
        fma_ss_instruction_list!($V);
        fma_ps_instruction_list!($V);
        fma_pd_instruction_list!($V);
    };
}

// Example usage:
// macro_rules! print_instruction {
//     ($name:ident, $l:ident, $p1:expr, $p2:expr, $p3:expr, $w:ident, $p4:expr) => {
//         println!("Instruction: {}, L: {}, P1: {}, P2: {}, P3: {}, W: {}, P4: {}", stringify!($name), stringify!($l), $p1, $p2, $p3, stringify!($w), $p4);
//     };
// }
//
// fma_instruction_list!(print_instruction);

// Dummy constants for the instruction list to compile.
#[allow(non_upper_case_globals)]
const L128: i32 = 128;
#[allow(non_upper_case_globals)]
const LIG: i32 = 0;
#[allow(non_upper_case_globals)]
const W1: i32 = 1;
#[allow(non_upper_case_globals)]
const W0: i32 = 0;
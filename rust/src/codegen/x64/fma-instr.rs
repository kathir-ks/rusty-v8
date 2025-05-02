// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines FMA instructions for x64 architecture.
pub mod fma_instr {

    macro_rules! define_fma_instructions {
        ($macro:ident) => {
            $macro! {
                (vfmadd132sd, 0x66, 0x0F, 0x38, 0x01, 0x99),
                (vfmadd213sd, 0x66, 0x0F, 0x38, 0x01, 0xA9),
                (vfmadd231sd, 0x66, 0x0F, 0x38, 0x01, 0xB9),
                (vfmsub132sd, 0x66, 0x0F, 0x38, 0x01, 0x9B),
                (vfmsub213sd, 0x66, 0x0F, 0x38, 0x01, 0xAB),
                (vfmsub231sd, 0x66, 0x0F, 0x38, 0x01, 0xBB),
                (vfnmadd132sd, 0x66, 0x0F, 0x38, 0x01, 0x9D),
                (vfnmadd213sd, 0x66, 0x0F, 0x38, 0x01, 0xAD),
                (vfnmadd231sd, 0x66, 0x0F, 0x38, 0x01, 0xBD),
                (vfnmsub132sd, 0x66, 0x0F, 0x38, 0x01, 0x9F),
                (vfnmsub213sd, 0x66, 0x0F, 0x38, 0x01, 0xAF),
                (vfnmsub231sd, 0x66, 0x0F, 0x38, 0x01, 0xBF),
                (vfmadd132ss, 0x66, 0x0F, 0x38, 0x00, 0x99),
                (vfmadd213ss, 0x66, 0x0F, 0x38, 0x00, 0xA9),
                (vfmadd231ss, 0x66, 0x0F, 0x38, 0x00, 0xB9),
                (vfmsub132ss, 0x66, 0x0F, 0x38, 0x00, 0x9B),
                (vfmsub213ss, 0x66, 0x0F, 0x38, 0x00, 0xAB),
                (vfmsub231ss, 0x66, 0x0F, 0x38, 0x00, 0xBB),
                (vfnmadd132ss, 0x66, 0x0F, 0x38, 0x00, 0x9D),
                (vfnmadd213ss, 0x66, 0x0F, 0x38, 0x00, 0xAD),
                (vfnmadd231ss, 0x66, 0x0F, 0x38, 0x00, 0xBD),
                (vfnmsub132ss, 0x66, 0x0F, 0x38, 0x00, 0x9F),
                (vfnmsub213ss, 0x66, 0x0F, 0x38, 0x00, 0xAF),
                (vfnmsub231ss, 0x66, 0x0F, 0x38, 0x00, 0xBF),
                (vfmadd132ps, 0x66, 0x0F, 0x38, 0x00, 0x98),
                (vfmadd213ps, 0x66, 0x0F, 0x38, 0x00, 0xA8),
                (vfmadd231ps, 0x66, 0x0F, 0x38, 0x00, 0xB8),
                (vfnmadd132ps, 0x66, 0x0F, 0x38, 0x00, 0x9C),
                (vfnmadd213ps, 0x66, 0x0F, 0x38, 0x00, 0xAC),
                (vfnmadd231ps, 0x66, 0x0F, 0x38, 0x00, 0xBC),
                (vfmadd132pd, 0x66, 0x0F, 0x38, 0x01, 0x98),
                (vfmadd213pd, 0x66, 0x0F, 0x38, 0x01, 0xA8),
                (vfmadd231pd, 0x66, 0x0F, 0x38, 0x01, 0xB8),
                (vfnmadd132pd, 0x66, 0x0F, 0x38, 0x01, 0x9C),
                (vfnmadd213pd, 0x66, 0x0F, 0x38, 0x01, 0xAC),
                (vfnmadd231pd, 0x66, 0x0F, 0x38, 0x01, 0xBC)
            }
        };
    }

    // Example Usage.  Remove if not needed, or adapt to the real use case.
    // macro_rules! print_instruction {
    //     ($($name:ident, $p1:literal, $p2:literal, $p3:literal, $p4:literal, $p5:literal),*) => {
    //         $(
    //             println!("Instruction: {}, Params: {}, {}, {}, {}, {}", stringify!($name), $p1, $p2, $p3, $p4, $p5);
    //         )*
    //     };
    // }
    //
    // pub fn example_usage() {
    //     define_fma_instructions!(print_instruction);
    // }

    // The following is a placeholder for a data structure that could represent the
    // instruction information.  Adjust to fit actual use case.
    #[derive(Debug, Clone, Copy)]
    pub struct FmaInstruction {
        pub name: &'static str,
        pub p1: u8,
        pub p2: u8,
        pub p3: u8,
        pub p4: u8,
        pub p5: u8,
    }

    macro_rules! create_instruction_array {
        ($($name:ident, $p1:literal, $p2:literal, $p3:literal, $p4:literal, $p5:literal),*) => {
            pub const FMA_INSTRUCTIONS: &[FmaInstruction] = &[
                $(
                    FmaInstruction {
                        name: stringify!($name),
                        p1: $p1,
                        p2: $p2,
                        p3: $p3,
                        p4: $p4,
                        p5: $p5,
                    },
                )*
            ];
        };
    }

    define_fma_instructions!(create_instruction_array);
}
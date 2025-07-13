// Converted from V8 C++ source files:
// Header: register-configuration.h
// Implementation: register-configuration.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_configuration {
    use crate::codegen::machine_type::MachineRepresentation;
    use crate::codegen::reglist::RegList;
    use std::cmp::max;

    pub struct RegisterConfiguration {
        fp_aliasing_kind: AliasingKind,
        num_general_registers: i32,
        num_float_registers: i32,
        num_double_registers: i32,
        num_simd128_registers: i32,
        num_simd256_registers: i32,
        num_allocatable_general_registers: i32,
        num_allocatable_float_registers: i32,
        num_allocatable_double_registers: i32,
        num_allocatable_simd128_registers: i32,
        num_allocatable_simd256_registers: i32,
        allocatable_general_codes_mask: i32,
        allocatable_float_codes_mask: i32,
        allocatable_double_codes_mask: i32,
        allocatable_simd128_codes_mask: i32,
        allocatable_simd256_codes_mask: i32,
        allocatable_general_codes: Vec<i32>,
        allocatable_float_codes: Vec<i32>,
        allocatable_double_codes: Vec<i32>,
        allocatable_simd128_codes: Vec<i32>,
        allocatable_simd256_codes: Vec<i32>,
    }

    impl RegisterConfiguration {
        pub const K_MAX_GENERAL_REGISTERS: i32 = 32;
        pub const K_MAX_FP_REGISTERS: i32 = 32;
        pub const K_MAX_REGISTERS: i32 = max(
            RegisterConfiguration::K_MAX_FP_REGISTERS,
            RegisterConfiguration::K_MAX_GENERAL_REGISTERS,
        );

        pub fn default() -> &'static RegisterConfiguration {
            // NOTE: This is a simplified implementation. In a real scenario,
            // you'd likely need a platform-specific default configuration.
            lazy_static::lazy_static! {
                static ref DEFAULT_CONFIG: RegisterConfiguration = {
                    RegisterConfiguration {
                        fp_aliasing_kind: AliasingKind::KIndependent,
                        num_general_registers: 16,
                        num_float_registers: 16,
                        num_double_registers: 16,
                        num_simd128_registers: 16,
                        num_simd256_registers: 16,
                        num_allocatable_general_registers: 8,
                        num_allocatable_float_registers: 8,
                        num_allocatable_double_registers: 8,
                        num_allocatable_simd128_registers: 8,
                        num_allocatable_simd256_registers: 8,
                        allocatable_general_codes_mask: 0xFF,
                        allocatable_float_codes_mask: 0xFF,
                        allocatable_double_codes_mask: 0xFF,
                        allocatable_simd128_codes_mask: 0xFF,
                        allocatable_simd256_codes_mask: 0xFF,
                        allocatable_general_codes: (0..8).collect(),
                        allocatable_float_codes: (0..8).collect(),
                        allocatable_double_codes: (0..8).collect(),
                        allocatable_simd128_codes: (0..8).collect(),
                        allocatable_simd256_codes: (0..8).collect(),
                    }
                };
            }
            &DEFAULT_CONFIG
        }

        pub fn poisoning() -> &'static RegisterConfiguration {
            // NOTE: This is a simplified implementation.  A real poisoning
            // config would have specific reserved registers.
            lazy_static::lazy_static! {
                static ref POISONING_CONFIG: RegisterConfiguration = {
                    RegisterConfiguration {
                        fp_aliasing_kind: AliasingKind::KIndependent,
                        num_general_registers: 16,
                        num_float_registers: 16,
                        num_double_registers: 16,
                        num_simd128_registers: 16,
                        num_simd256_registers: 16,
                        num_allocatable_general_registers: 7,
                        num_allocatable_float_registers: 7,
                        num_allocatable_double_registers: 7,
                        num_allocatable_simd128_registers: 7,
                        num_allocatable_simd256_registers: 7,
                        allocatable_general_codes_mask: 0x7F,
                        allocatable_float_codes_mask: 0x7F,
                        allocatable_double_codes_mask: 0x7F,
                        allocatable_simd128_codes_mask: 0x7F,
                        allocatable_simd256_codes_mask: 0x7F,
                        allocatable_general_codes: (0..7).collect(),
                        allocatable_float_codes: (0..7).collect(),
                        allocatable_double_codes: (0..7).collect(),
                        allocatable_simd128_codes: (0..7).collect(),
                        allocatable_simd256_codes: (0..7).collect(),
                    }
                };
            }
            &POISONING_CONFIG
        }

        pub fn restrict_general_registers(registers: RegList) -> Box<RegisterConfiguration> {
            // NOTE: This is a placeholder.  A real implementation would
            // create a new configuration based on the registers specified.
            Box::new(RegisterConfiguration {
                fp_aliasing_kind: AliasingKind::KIndependent,
                num_general_registers: 16,
                num_float_registers: 16,
                num_double_registers: 16,
                num_simd128_registers: 16,
                num_simd256_registers: 16,
                num_allocatable_general_registers: 4,
                num_allocatable_float_registers: 4,
                num_allocatable_double_registers: 4,
                num_allocatable_simd128_registers: 4,
                num_allocatable_simd256_registers: 4,
                allocatable_general_codes_mask: 0x0F,
                allocatable_float_codes_mask: 0x0F,
                allocatable_double_codes_mask: 0x0F,
                allocatable_simd128_codes_mask: 0x0F,
                allocatable_simd256_codes_mask: 0x0F,
                allocatable_general_codes: (0..4).collect(),
                allocatable_float_codes: (0..4).collect(),
                allocatable_double_codes: (0..4).collect(),
                allocatable_simd128_codes: (0..4).collect(),
                allocatable_simd256_codes: (0..4).collect(),
            })
        }

        pub fn new(
            fp_aliasing_kind: AliasingKind,
            num_general_registers: i32,
            num_double_registers: i32,
            num_simd128_registers: i32,
            num_simd256_registers: i32,
            num_allocatable_general_registers: i32,
            num_allocatable_double_registers: i32,
            num_allocatable_simd128_registers: i32,
            num_allocatable_simd256_registers: i32,
            allocatable_general_codes: Vec<i32>,
            allocatable_double_codes: Vec<i32>,
            independent_allocatable_simd128_codes: Option<Vec<i32>>,
        ) -> Self {
            let mut config = RegisterConfiguration {
                fp_aliasing_kind,
                num_general_registers,
                num_float_registers: 0,
                num_double_registers,
                num_simd128_registers,
                num_simd256_registers,
                num_allocatable_general_registers,
                num_allocatable_float_registers: 0,
                num_allocatable_double_registers,
                num_allocatable_simd128_registers,
                num_allocatable_simd256_registers,
                allocatable_general_codes_mask: 0,
                allocatable_float_codes_mask: 0,
                allocatable_double_codes_mask: 0,
                allocatable_simd128_codes_mask: 0,
                allocatable_simd256_codes_mask: 0,
                allocatable_general_codes,
                allocatable_float_codes: Vec::new(),
                allocatable_double_codes,
                allocatable_simd128_codes: Vec::new(),
                allocatable_simd256_codes: Vec::new(),
            };

            config.allocatable_general_codes_mask = config
                .allocatable_general_codes
                .iter()
                .fold(0, |acc, &code| acc | (1 << code));
            config.allocatable_double_codes_mask = config
                .allocatable_double_codes
                .iter()
                .fold(0, |acc, &code| acc | (1 << code));

            if config.fp_aliasing_kind == AliasingKind::KCombine {
                config.num_float_registers =
                    if config.num_double_registers * 2 <= RegisterConfiguration::K_MAX_FP_REGISTERS {
                        config.num_double_registers * 2
                    } else {
                        RegisterConfiguration::K_MAX_FP_REGISTERS
                    };
                config.num_allocatable_float_registers = 0;
                for i in 0..config.num_allocatable_double_registers {
                    let base_code = config.allocatable_double_codes[i as usize] * 2;
                    if base_code >= RegisterConfiguration::K_MAX_FP_REGISTERS {
                        continue;
                    }
                    config.allocatable_float_codes.push(base_code);
                    config.allocatable_float_codes.push(base_code + 1);
                    config.allocatable_float_codes_mask |= (0x3 << base_code);
                }
                config.num_simd128_registers = config.num_double_registers / 2;
            } else if config.fp_aliasing_kind == AliasingKind::KOverlap {
                config.num_float_registers = config.num_simd128_registers = config.num_double_registers;
                config.num_allocatable_float_registers = config.num_allocatable_simd128_registers =
                    config.num_allocatable_double_registers;
                for i in 0..config.num_allocatable_float_registers {
                    config.allocatable_float_codes.push(config.allocatable_double_codes[i as usize]);
                    config.allocatable_simd128_codes.push(config.allocatable_double_codes[i as usize]);
                }
                config.allocatable_float_codes_mask = config.allocatable_simd128_codes_mask =
                    config.allocatable_double_codes_mask;
            } else {
                if let Some(independent_codes) = independent_allocatable_simd128_codes {
                   config.num_float_registers = config.num_double_registers;
                   config.num_allocatable_float_registers = config.num_allocatable_double_registers;

                    for i in 0..config.num_allocatable_float_registers {
                      config.allocatable_float_codes.push(config.allocatable_double_codes[i as usize]);
                    }
                    config.allocatable_float_codes_mask = config.allocatable_double_codes_mask;

                    config.allocatable_simd128_codes = independent_codes;
                    for i in 0..config.num_allocatable_simd128_registers {
                      config.allocatable_simd128_codes_mask |= (1 << config.allocatable_simd128_codes[i as usize]);
                    }
                }
            }
            config
        }

        pub fn num_general_registers(&self) -> i32 {
            self.num_general_registers
        }
        pub fn num_float_registers(&self) -> i32 {
            self.num_float_registers
        }
        pub fn num_double_registers(&self) -> i32 {
            self.num_double_registers
        }
        pub fn num_simd128_registers(&self) -> i32 {
            self.num_simd128_registers
        }
        pub fn num_simd256_registers(&self) -> i32 {
            self.num_simd256_registers
        }
        pub fn num_allocatable_general_registers(&self) -> i32 {
            self.num_allocatable_general_registers
        }
        pub fn num_allocatable_float_registers(&self) -> i32 {
            self.num_allocatable_float_registers
        }
        pub fn num_allocatable_double_registers(&self) -> i32 {
            self.num_allocatable_double_registers
        }
        pub fn num_allocatable_simd128_registers(&self) -> i32 {
            self.num_allocatable_simd128_registers
        }
        pub fn num_allocatable_simd256_registers(&self) -> i32 {
            self.num_allocatable_simd256_registers
        }

        pub fn fp_aliasing_kind(&self) -> AliasingKind {
            self.fp_aliasing_kind
        }
        pub fn allocatable_general_codes_mask(&self) -> i32 {
            self.allocatable_general_codes_mask
        }
        pub fn allocatable_double_codes_mask(&self) -> i32 {
            self.allocatable_double_codes_mask
        }
        pub fn allocatable_float_codes_mask(&self) -> i32 {
            self.allocatable_float_codes_mask
        }
        pub fn allocatable_simd128_codes_mask(&self) -> i32 {
            self.allocatable_simd128_codes_mask
        }

        pub fn get_allocatable_general_code(&self, index: i32) -> i32 {
            if index < 0 || index >= self.num_allocatable_general_registers() {
                panic!("Index out of bounds");
            }
            self.allocatable_general_codes[index as usize]
        }
        pub fn is_allocatable_general_code(&self, index: i32) -> bool {
            ((1 << index) & self.allocatable_general_codes_mask_) != 0
        }
        pub fn get_allocatable_float_code(&self, index: i32) -> i32 {
            if index < 0 || index >= self.num_allocatable_float_registers() {
                panic!("Index out of bounds");
            }
            self.allocatable_float_codes[index as usize]
        }
        pub fn is_allocatable_float_code(&self, index: i32) -> bool {
            ((1 << index) & self.allocatable_float_codes_mask_) != 0
        }
        pub fn get_allocatable_double_code(&self, index: i32) -> i32 {
            if index < 0 || index >= self.num_allocatable_double_registers() {
                panic!("Index out of bounds");
            }
            self.allocatable_double_codes[index as usize]
        }
        pub fn is_allocatable_double_code(&self, index: i32) -> bool {
            ((1 << index) & self.allocatable_double_codes_mask_) != 0
        }
        pub fn get_allocatable_simd128_code(&self, index: i32) -> i32 {
            if index < 0 || index >= self.num_allocatable_simd128_registers() {
                panic!("Index out of bounds");
            }
            self.allocatable_simd128_codes[index as usize]
        }
        pub fn is_allocatable_simd128_code(&self, index: i32) -> bool {
            ((1 << index) & self.allocatable_simd128_codes_mask_) != 0
        }
        pub fn get_allocatable_simd256_code(&self, index: i32) -> i32 {
            if index < 0 || index >= self.num_allocatable_simd256_registers() {
                panic!("Index out of bounds");
            }
            self.allocatable_simd256_codes[index as usize]
        }
        pub fn is_allocatable_simd256_code(&self, index: i32) -> bool {
            ((1 << index) & self.allocatable_simd256_codes_mask_) != 0
        }

        pub fn allocatable_general_codes(&self) -> &Vec<i32> {
            &self.allocatable_general_codes
        }
        pub fn allocatable_float_codes(&self) -> &Vec<i32> {
            &self.allocatable_float_codes
        }
        pub fn allocatable_double_codes(&self) -> &Vec<i32> {
            &self.allocatable_double_codes
        }
        pub fn allocatable_simd128_codes(&self) -> &Vec<i32> {
            &self.allocatable_simd128_codes
        }
        pub fn allocatable_simd256_codes(&self) -> &Vec<i32> {
            &self.allocatable_simd256_codes
        }

        pub fn get_aliases(
            &self,
            rep: MachineRepresentation,
            index: i32,
            other_rep: MachineRepresentation,
            alias_base_index: &mut i32,
        ) -> i32 {
            if self.fp_aliasing_kind_ != AliasingKind::KCombine {
                return 0;
            }

            if rep == other_rep {
                *alias_base_index = index;
                return 1;
            }

            let rep_int = rep as i32;
            let other_rep_int = other_rep as i32;

            if rep_int > other_rep_int {
                let shift = rep_int - other_rep_int;
                let base_index = index << shift;

                if base_index >= RegisterConfiguration::K_MAX_FP_REGISTERS {
                    return 0;
                }

                *alias_base_index = base_index;
                return 1 << shift;
            }

            let shift = other_rep_int - rep_int;
            *alias_base_index = index >> shift;
            1
        }

        pub fn are_aliases(
            &self,
            rep: MachineRepresentation,
            index: i32,
            other_rep: MachineRepresentation,
            other_index: i32,
        ) -> bool {
            if self.fp_aliasing_kind_ != AliasingKind::KCombine {
                return false;
            }

            if rep == other_rep {
                return index == other_index;
            }

            let rep_int = rep as i32;
            let other_rep_int = other_rep as i32;

            if rep_int > other_rep_int {
                let shift = rep_int - other_rep_int;
                return index == other_index >> shift;
            }

            let shift = other_rep_int - rep_int;
            index >> shift == other_index
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum AliasingKind {
        KIndependent,
        KCombine,
        KOverlap,
    }
}

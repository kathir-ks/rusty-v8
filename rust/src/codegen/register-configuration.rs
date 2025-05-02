// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/register-configuration.h - No direct equivalent in Rust

// src/base/lazy-instance.h - Replaced with std::sync::Once and lazy_static
use lazy_static::lazy_static;
use std::sync::Once;

// src/codegen/cpu-features.h - Placeholder for platform-specific features
// Assuming a simplified CPU features struct for demonstration
#[cfg(target_arch = "arm")]
mod cpu_features {
    #[derive(Default)]
    pub struct CpuFeatures {
        vfp32dregs: bool, // Example feature
    }

    static CPU_FEATURES: std::sync::OnceLock<CpuFeatures> = std::sync::OnceLock::new();

    impl CpuFeatures {
        pub fn initialize() {
            CPU_FEATURES.get_or_init(|| CpuFeatures { vfp32dregs: true }); //Example initialization
        }

        pub fn is_supported(feature: Feature) -> bool {
            if let Some(cpu_features) = CPU_FEATURES.get() {
                match feature {
                    Feature::VFP32DREGS => cpu_features.vfp32dregs,
                }
            } else {
                false
            }
        }
    }

    pub enum Feature {
        VFP32DREGS,
    }

    pub fn initialize() {
        CPU_FEATURES.get_or_init(|| CpuFeatures { vfp32dregs: true }); //Example initialization
    }

    pub fn is_supported(feature: Feature) -> bool {
        if let Some(cpu_features) = CPU_FEATURES.get() {
            match feature {
                Feature::VFP32DREGS => cpu_features.vfp32dregs,
            }
        } else {
            false
        }
    }
}

// src/codegen/register.h - Defining Register structs and enums
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    code: i32,
}

impl Register {
    pub const kNumRegisters: i32 = 32; // Example value

    pub fn from_code(code: i32) -> Self {
        Register { code }
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FloatRegister {
    code: i32,
}

impl FloatRegister {
    pub const kNumRegisters: i32 = 32; // Example value
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DoubleRegister {
    code: i32,
}

impl DoubleRegister {
    pub const kNumRegisters: i32 = 32; // Example value
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Simd128Register {
    code: i32,
}

impl Simd128Register {
    pub const kNumRegisters: i32 = 32; // Example value
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Simd256Register {
    code: i32,
}

impl Simd256Register {
    pub const kNumRegisters: i32 = 32; // Example value
}

// src/common/globals.h - Replaced with constants
const V8_TARGET_ARCH_IA32: bool = cfg!(target_arch = "x86");
const V8_TARGET_ARCH_X64: bool = cfg!(target_arch = "x86_64");
const V8_TARGET_ARCH_ARM: bool = cfg!(target_arch = "arm");
const V8_TARGET_ARCH_ARM64: bool = cfg!(target_arch = "aarch64");
const V8_TARGET_ARCH_MIPS: bool = false;
const V8_TARGET_ARCH_MIPS64: bool = false;
const V8_TARGET_ARCH_LOONG64: bool = false;
const V8_TARGET_ARCH_PPC64: bool = false;
const V8_TARGET_ARCH_S390X: bool = false;
const V8_TARGET_ARCH_RISCV64: bool = cfg!(target_arch = "riscv64");
const V8_TARGET_ARCH_RISCV32: bool = cfg!(target_arch = "riscv32");

macro_rules! register_count {
    ($r:ident) => {
        1 +
    };
}

macro_rules! register_code {
    ($r:ident) => {
        concat!("kRegCode_", stringify!($r))
    };
}

// Implementation of RegisterConfiguration
#[derive(Debug)]
pub struct RegisterConfiguration {
    num_general_registers_: i32,
    num_float_registers_: i32,
    num_double_registers_: i32,
    num_simd128_registers_: i32,
    num_simd256_registers_: i32,
    num_allocatable_general_registers_: i32,
    num_allocatable_float_registers_: i32,
    num_allocatable_double_registers_: i32,
    num_allocatable_simd128_registers_: i32,
    num_allocatable_simd256_registers_: i32,
    allocatable_general_codes_mask_: u32,
    allocatable_float_codes_mask_: u32,
    allocatable_double_codes_mask_: u32,
    allocatable_simd128_codes_mask_: u32,
    allocatable_simd256_codes_mask_: u32,
    allocatable_general_codes_: &'static [i32],
    allocatable_double_codes_: &'static [i32],
    fp_aliasing_kind_: AliasingKind,
    allocatable_float_codes_: [i32; RegisterConfiguration::kMaxFPRegisters as usize],
    allocatable_simd128_codes_: [i32; RegisterConfiguration::kMaxFPRegisters as usize],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AliasingKind {
    kFPAliasing,
    kCombine,
    kOverlap,
    kIndependent,
}

impl RegisterConfiguration {
    pub const kMaxGeneralRegisters: i32 = 64; // Example Value
    pub const kMaxFPRegisters: i32 = 64; // Example Value

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
        allocatable_general_codes: &'static [i32],
        allocatable_double_codes: &'static [i32],
        independent_allocatable_simd128_codes: &'static [i32],
    ) -> Self {
        let mut config = RegisterConfiguration {
            num_general_registers_: num_general_registers,
            num_float_registers_: 0,
            num_double_registers_: num_double_registers,
            num_simd128_registers_: num_simd128_registers,
            num_simd256_registers_: num_simd256_registers,
            num_allocatable_general_registers_: num_allocatable_general_registers,
            num_allocatable_float_registers_: 0,
            num_allocatable_double_registers_: num_allocatable_double_registers,
            num_allocatable_simd128_registers_: num_allocatable_simd128_registers,
            num_allocatable_simd256_registers_: num_allocatable_simd256_registers,
            allocatable_general_codes_mask_: 0,
            allocatable_float_codes_mask_: 0,
            allocatable_double_codes_mask_: 0,
            allocatable_simd128_codes_mask_: 0,
            allocatable_simd256_codes_mask_: 0,
            allocatable_general_codes_: allocatable_general_codes,
            allocatable_double_codes_: allocatable_double_codes,
            fp_aliasing_kind_: fp_aliasing_kind,
            allocatable_float_codes_: [0; RegisterConfiguration::kMaxFPRegisters as usize],
            allocatable_simd128_codes_: [0; RegisterConfiguration::kMaxFPRegisters as usize],
        };

        assert!(num_general_registers <= RegisterConfiguration::kMaxGeneralRegisters);
        assert!(num_double_registers <= RegisterConfiguration::kMaxFPRegisters);

        for i in 0..num_allocatable_general_registers {
            config.allocatable_general_codes_mask_ |= (1 << config.allocatable_general_codes_[i as usize]);
        }
        for i in 0..num_allocatable_double_registers {
            config.allocatable_double_codes_mask_ |= (1 << config.allocatable_double_codes_[i as usize]);
        }

        if config.fp_aliasing_kind_ == AliasingKind::kCombine {
            config.num_float_registers_ =
                if num_double_registers * 2 <= RegisterConfiguration::kMaxFPRegisters {
                    num_double_registers * 2
                } else {
                    RegisterConfiguration::kMaxFPRegisters
                };
            config.num_allocatable_float_registers_ = 0;

            let mut num_allocatable_float_registers = 0;
            for i in 0..num_allocatable_double_registers {
                let base_code = config.allocatable_double_codes_[i as usize] * 2;
                if base_code >= RegisterConfiguration::kMaxFPRegisters {
                    continue;
                }
                config.allocatable_float_codes_[num_allocatable_float_registers] = base_code;
                num_allocatable_float_registers += 1;
                config.allocatable_float_codes_[num_allocatable_float_registers] = base_code + 1;
                num_allocatable_float_registers += 1;

                config.allocatable_float_codes_mask_ |= (0x3 << base_code);
            }
            config.num_allocatable_float_registers_ = num_allocatable_float_registers;

            config.num_simd128_registers_ = num_double_registers / 2;
            config.num_allocatable_simd128_registers_ = 0;
            let mut last_simd128_code = config.allocatable_double_codes_[0] / 2;
            let mut num_allocatable_simd128_registers = 0;

            for i in 1..num_allocatable_double_registers {
                let next_simd128_code = config.allocatable_double_codes_[i as usize] / 2;
                assert!(next_simd128_code >= last_simd128_code);
                if last_simd128_code == next_simd128_code {
                    config.allocatable_simd128_codes_[num_allocatable_simd128_registers] = next_simd128_code;
                    num_allocatable_simd128_registers += 1;
                    config.allocatable_simd128_codes_mask_ |= (0x1 << next_simd128_code);
                }
                last_simd128_code = next_simd128_code;
            }
            config.num_allocatable_simd128_registers_ = num_allocatable_simd128_registers;
        } else if config.fp_aliasing_kind_ == AliasingKind::kOverlap {
            config.num_float_registers_ = num_simd128_registers;
            config.num_simd128_registers_ = num_double_registers;
            config.num_allocatable_float_registers_ = num_allocatable_double_registers;
            config.num_allocatable_simd128_registers_ = num_allocatable_double_registers;
            for i in 0..config.num_allocatable_float_registers_ as usize {
                config.allocatable_float_codes_[i] = config.allocatable_double_codes_[i];
                config.allocatable_simd128_codes_[i] = config.allocatable_double_codes_[i];
                #[cfg(V8_TARGET_ARCH_X64)]
                {
                    //config.allocatable_simd256_codes_[i] = config.allocatable_double_codes_[i];
                }
            }

            config.allocatable_float_codes_mask_ = config.allocatable_double_codes_mask_;
            config.allocatable_simd128_codes_mask_ = config.allocatable_double_codes_mask_;
            #[cfg(V8_TARGET_ARCH_X64)]
            {
                config.num_simd256_registers_ = num_double_registers;
                config.num_allocatable_simd256_registers_ = num_allocatable_double_registers;
                //config.allocatable_simd256_codes_mask_ = config.allocatable_double_codes_mask_;
            }
        } else {
            assert_eq!(config.fp_aliasing_kind_, AliasingKind::kIndependent);
            assert_ne!(independent_allocatable_simd128_codes, &[]);

            config.num_float_registers_ = num_double_registers;
            config.num_allocatable_float_registers_ = num_allocatable_double_registers;
            for i in 0..config.num_allocatable_float_registers_ as usize {
                config.allocatable_float_codes_[i] = config.allocatable_double_codes_[i];
            }
            config.allocatable_float_codes_mask_ = config.allocatable_double_codes_mask_;
            for i in 0..num_allocatable_simd128_registers as usize {
                config.allocatable_simd128_codes_[i] = independent_allocatable_simd128_codes[i];
            }
            for i in 0..config.num_allocatable_simd128_registers_ as usize {
                config.allocatable_simd128_codes_mask_ |= (1 << config.allocatable_simd128_codes_[i]);
            }
        }

        config
    }

    pub fn num_general_registers(&self) -> i32 {
        self.num_general_registers_
    }

    pub fn num_float_registers(&self) -> i32 {
        self.num_float_registers_
    }

    pub fn num_double_registers(&self) -> i32 {
        self.num_double_registers_
    }

    pub fn num_simd128_registers(&self) -> i32 {
        self.num_simd128_registers_
    }

    pub fn num_simd256_registers(&self) -> i32 {
        self.num_simd256_registers_
    }

    pub fn num_allocatable_general_registers(&self) -> i32 {
        self.num_allocatable_general_registers_
    }

    pub fn num_allocatable_float_registers(&self) -> i32 {
        self.num_allocatable_float_registers_
    }

    pub fn num_allocatable_double_registers(&self) -> i32 {
        self.num_allocatable_double_registers_
    }

    pub fn num_allocatable_simd128_registers(&self) -> i32 {
        self.num_allocatable_simd128_registers_
    }

    pub fn num_allocatable_simd256_registers(&self) -> i32 {
        self.num_allocatable_simd256_registers_
    }

    pub fn GetAllocatableGeneralCode(&self, i: i32) -> i32 {
        self.allocatable_general_codes_[i as usize]
    }

    pub fn GetAllocatableDoubleCode(&self, i: i32) -> i32 {
        self.allocatable_double_codes_[i as usize]
    }

    pub fn GetAllocatableSimd128Code(&self, i: i32) -> i32 {
        self.allocatable_simd128_codes_[i as usize]
    }

    pub fn fp_aliasing_kind(&self) -> AliasingKind {
        self.fp_aliasing_kind_
    }

    pub fn GetAliases(
        &self,
        rep: MachineRepresentation,
        index: i32,
        other_rep: MachineRepresentation,
        alias_base_index: &mut i32,
    ) -> i32 {
        assert_eq!(self.fp_aliasing_kind_, AliasingKind::kCombine);
        assert!(rep.is_floating_point() && other_rep.is_floating_point());
        if rep == other_rep {
            *alias_base_index = index;
            return 1;
        }
        let rep_int = rep as i32;
        let other_rep_int = other_rep as i32;
        if rep_int > other_rep_int {
            let shift = rep_int - other_rep_int;
            let base_index = index << shift;
            if base_index >= RegisterConfiguration::kMaxFPRegisters {
                return 0;
            }
            *alias_base_index = base_index;
            return 1 << shift;
        }
        let shift = other_rep_int - rep_int;
        *alias_base_index = index >> shift;
        return 1;
    }

    pub fn AreAliases(
        &self,
        rep: MachineRepresentation,
        index: i32,
        other_rep: MachineRepresentation,
        other_index: i32,
    ) -> bool {
        assert_eq!(self.fp_aliasing_kind_, AliasingKind::kCombine);
        assert!(rep.is_floating_point() && other_rep.is_floating_point());
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
        return index >> shift == other_index;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MachineRepresentation {
    kWord32, // Example
    kFloat32,
    kFloat64,
    kSimd128,
    kSimd256,
}

impl MachineRepresentation {
    pub fn is_floating_point(&self) -> bool {
        match self {
            MachineRepresentation::kFloat32 | MachineRepresentation::kFloat64 | MachineRepresentation::kSimd128 | MachineRepresentation::kSimd256 => true,
            _ => false,
        }
    }
}

// Assert that kFloat32, kFloat64, kSimd128 and kSimd256 are consecutive values.
const _ASSERT: () = {
    assert!(MachineRepresentation::kSimd256 as i32 == MachineRepresentation::kSimd128 as i32 + 1);
    assert!(MachineRepresentation::kSimd128 as i32 == MachineRepresentation::kFloat64 as i32 + 1);
    assert!(MachineRepresentation::kFloat64 as i32 == MachineRepresentation::kFloat32 as i32 + 1);
};

// Global Default Configuration
lazy_static! {
    static ref DEFAULT_REGISTER_CONFIGURATION: ArchDefaultRegisterConfiguration =
        ArchDefaultRegisterConfiguration::new();
}

pub fn default_register_configuration() -> &'static RegisterConfiguration {
    &DEFAULT_REGISTER_CONFIGURATION.config
}

// Restricted Register Configuration
pub struct RestrictedRegisterConfiguration {
    config: RegisterConfiguration,
    allocatable_general_register_codes_: Vec<i32>,
    allocatable_general_register_names_: Vec<&'static str>,
}

impl RestrictedRegisterConfiguration {
    pub fn new(
        num_allocatable_general_registers: i32,
        allocatable_general_register_codes: Vec<i32>,
        allocatable_general_register_names: Vec<&'static str>,
    ) -> Self {
        for i in 0..num_allocatable_general_registers {
            assert!(Self::is_allocatable_general_register(allocatable_general_register_codes[i as usize]));
        }

        let config = RegisterConfiguration::new(
            AliasingKind::kFPAliasing,
            Register::kNumRegisters,
            DoubleRegister::kNumRegisters,
            get_num_simd128_registers(),
            get_num_simd256_registers(),
            num_allocatable_general_registers,
            get_num_allocatable_double_registers(),
            get_num_allocatable_simd128_registers(),
            get_num_allocatable_simd256_registers(),
            &allocatable_general_register_codes,
            get_allocatable_double_codes(),
            get_allocatable_simd128_codes(),
        );

        RestrictedRegisterConfiguration {
            config,
            allocatable_general_register_codes_: allocatable_general_register_codes,
            allocatable_general_register_names_: allocatable_general_register_names,
        }
    }

    fn is_allocatable_general_register(code: i32) -> bool {
        for i in 0..kMaxAllocatableGeneralRegisterCount {
            if code == K_ALLOCATABLE_GENERAL_CODES[i as usize] {
                return true;
            }
        }
        false
    }

    pub fn config(&self) -> &RegisterConfiguration {
        &self.config
    }
}

pub fn restrict_general_registers(registers: RegList) -> Box<RestrictedRegisterConfiguration> {
    let num = registers.count();
    let mut codes = Vec::with_capacity(num as usize);
    let mut names = Vec::with_capacity(num as usize);
    let mut counter = 0;
    for i in 0..default_register_configuration().num_allocatable_general_registers() {
        let reg = Register::from_code(default_register_configuration().GetAllocatableGeneralCode(i));
        if registers.has(reg) {
            assert!(counter < num);
            codes.push(reg.code());
            names.push(register_name(Register::from_code(i)));
            counter += 1;
        }
    }

    Box::new(RestrictedRegisterConfiguration::new(
        num,
        codes,
        names,
    ))
}

// Arch Default Configuration
pub struct ArchDefaultRegisterConfiguration {
    config: RegisterConfiguration,
}

impl ArchDefaultRegisterConfiguration {
    fn new() -> Self {
        let config = RegisterConfiguration::new(
            AliasingKind::kFPAliasing,
            Register::kNumRegisters,
            DoubleRegister::kNumRegisters,
            get_num_simd128_registers(),
            get_num_simd256_registers(),
            K_MAX_ALLOCATABLE_GENERAL_REGISTER_COUNT,
            get_num_allocatable_double_registers(),
            get_num_allocatable_simd128_registers(),
            get_num_allocatable_simd256_registers(),
            &K_ALLOCATABLE_GENERAL_CODES,
            get_allocatable_double_codes(),
            get_allocatable_simd128_codes(),
        );
        ArchDefaultRegisterConfiguration { config }
    }
}

// Helper functions for register counts and codes
const K_MAX_ALLOCATABLE_GENERAL_REGISTER_COUNT: i32 = {
    let mut count = 0;
    #[cfg(target_arch = "x86_64")]
    {
        count = 15; // Example
    }
    #[cfg(target_arch = "arm64")]
    {
        count = 28;
    }
    count
};
const K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT: i32 = 16; // Example Value

const K_ALLOCATABLE_GENERAL_CODES: [i32; K_MAX_ALLOCATABLE_GENERAL_REGISTER_COUNT as usize] = {
    let mut codes = [0; K_MAX_ALLOCATABLE_GENERAL_REGISTER_COUNT as usize];
    #[cfg(target_arch = "x86_64")]
    {
        codes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    }
    #[cfg(target_arch = "arm64")]
    {
        codes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27];
    }
    codes
};

const K_ALLOCATABLE_DOUBLE_CODES: [i32; K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT as usize] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // Example Value
#[cfg(target_arch = "arm")]
const K_ALLOCATABLE_NO_VFP32_DOUBLE_CODES: [i32; K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT as usize] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // Example Value

const K_MAX_ALLOCATABLE_SIMD128_REGISTER_COUNT: i32 = 16; // Example Value
const K_ALLOCATABLE_SIMD128_CODES: [i32; K_MAX_ALLOCATABLE_SIMD128_REGISTER_COUNT as usize] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // Example Value

fn get_num_simd128_registers() -> i32 {
    #[cfg(any(
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "powerpc64"
    ))]
    {
        Simd128Register::kNumRegisters
    }
    #[cfg(not(any(
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "powerpc64"
    )))]
    {
        0
    }
}

fn get_num_simd256_registers() -> i32 {
    0
}

fn get_num_allocatable_double_registers() -> i32 {
    #[cfg(target_arch = "x86")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "x86_64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "arm")]
    {
        if cpu_features::CpuFeatures::is_supported(cpu_features::Feature::VFP32DREGS) {
            K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
        } else {
            16 // Example Value
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "mips")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "mips64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "loongarch64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "powerpc64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "s390x")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "riscv64")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(target_arch = "riscv32")]
    {
        K_MAX_ALLOCATABLE_DOUBLE_REGISTER_COUNT
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "loongarch64",
        target_arch = "powerpc64",
        target_arch = "s390x",
        target_arch = "riscv64",
        target_arch = "riscv32"
    )))]
    {
        compile_error!("Unsupported target architecture.");
    }
}

fn get_num_allocatable_simd128_registers() -> i32 {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "powerpc64"))]
    {
        K_MAX_ALLOCATABLE_SIMD128_REGISTER_COUNT
    }
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "powerpc64")))]
    {
        0
    }
}

fn get_num_allocatable_simd256_registers() -> i32 {
    0
}

fn get_allocatable_double_codes() -> &'static [i32] {
    #[cfg(target_arch = "arm")]
    {
        if cpu_features::CpuFeatures::is_supported(cpu_features::Feature::VFP32DREGS) {
            &K_ALLOCATABLE_DOUBLE_CODES
        } else {
            &K_ALLOCATABLE_NO_VFP32_DOUBLE_CODES
        }
    }
    #[cfg(not(target_arch = "arm"))]
    {
        &K_ALLOCATABLE_DOUBLE_CODES
    }
}

fn get_allocatable_simd128_codes() -> &'static [i32] {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "powerpc64"))]
    {
        &K_ALLOCATABLE_SIMD128_CODES
    }
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "powerpc64")))]
    {
        &K_ALLOCATABLE_DOUBLE_CODES
    }
}

// RegList struct (equivalent to C++ RegList)
#[derive(Clone, Copy, Debug, Default)]
pub struct RegList {
    bits: u64,
}

impl RegList {
    pub fn has(self, reg: Register) -> bool {
        (self.bits & (1 << reg.code())) != 0
    }

    pub fn count(self) -> i32 {
        self.bits.count_ones() as i32
    }
}

fn register_name(reg: Register) -> &'static str {
    match reg.code() {
        0 => "r0",
        1 => "r1",
        _ => "unknown",
    }
}
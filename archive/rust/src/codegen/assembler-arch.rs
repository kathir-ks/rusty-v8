// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file acts as a dispatch based on target architecture.
// In Rust, we use conditional compilation with `cfg!` attributes
// to achieve the same effect.

#[cfg(target_arch = "x86")]
pub mod ia32 {
    include!("codegen/ia32/assembler-ia32.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "x86_64")]
pub mod x64 {
    include!("codegen/x64/assembler-x64.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "aarch64")]
pub mod arm64 {
    include!("codegen/arm64/assembler-arm64.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "arm")]
pub mod arm {
    include!("codegen/arm/assembler-arm.rs"); // Assuming a Rust translation exists
}

// Note: ppc64 and mips64 might need a specific endianness check as well

#[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
pub mod ppc64 {
    include!("codegen/ppc/assembler-ppc.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "mips64")]
pub mod mips64 {
    include!("codegen/mips64/assembler-mips64.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "loongarch64")]
pub mod loong64 {
    include!("codegen/loong64/assembler-loong64.rs"); // Assuming a Rust translation exists
}

#[cfg(target_arch = "s390x")]
pub mod s390 {
    include!("codegen/s390/assembler-s390.rs"); // Assuming a Rust translation exists
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod riscv {
    include!("codegen/riscv/assembler-riscv.rs"); // Assuming a Rust translation exists
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm",
    all(target_arch = "powerpc64", target_endian = "big"),
    target_arch = "mips64",
    target_arch = "loongarch64",
    target_arch = "s390x",
    target_arch = "riscv32",
    target_arch = "riscv64"
)))]
compile_error!("Unknown architecture.");

pub mod assembler {
    // Placeholder for the common Assembler functionality.
    // This may need to be defined differently based on the
    // specific architecture being compiled for.  Consider using a trait
    // and implementing it for each architecture specific assembler.
    pub struct Assembler {}

    impl Assembler {
        pub fn new() -> Self {
            Assembler {}
        }
    }
}
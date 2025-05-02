// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_arch {
    #[cfg(target_arch = "x86")]
    pub mod ia32 {
        include!("codegen/ia32/register-ia32.rs");
    }

    #[cfg(target_arch = "x86_64")]
    pub mod x64 {
        include!("codegen/x64/register-x64.rs");
    }

    #[cfg(target_arch = "aarch64")]
    pub mod arm64 {
        include!("codegen/arm64/register-arm64.rs");
    }

    #[cfg(target_arch = "arm")]
    pub mod arm {
        include!("codegen/arm/register-arm.rs");
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod ppc {
        include!("codegen/ppc/register-ppc.rs");
    }

    #[cfg(target_arch = "mips64")]
    pub mod mips64 {
        include!("codegen/mips64/register-mips64.rs");
    }

    #[cfg(target_arch = "loongarch64")]
    pub mod loong64 {
        include!("codegen/loong64/register-loong64.rs");
    }

    #[cfg(target_arch = "s390x")]
    pub mod s390 {
        include!("codegen/s390/register-s390.rs");
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub mod riscv {
        include!("codegen/riscv/register-riscv.rs");
    }

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "powerpc64",
        target_arch = "mips64",
        target_arch = "loongarch64",
        target_arch = "s390x",
        target_arch = "riscv32",
        target_arch = "riscv64"
    )))]
    compile_error!("Unknown architecture.");

    pub mod register_base {
        // Placeholder for the content of "src/codegen/register-base.h"
        // which would be defined as a Rust module
        // For example:
        // #[derive(Debug, Clone)]
        // pub struct RegisterBase { ... }
    }

}
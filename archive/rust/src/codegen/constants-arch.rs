// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This Rust module emulates the C++ header file `src/codegen/constants-arch.h`.
// It includes architecture-specific constant definitions based on target architecture.

#[cfg(all(target_arch = "arm", feature = "v8_target_arch_arm"))]
pub mod constants_arch {
    include!("codegen/arm/constants-arm.rs");
}

#[cfg(all(target_arch = "aarch64", feature = "v8_target_arch_arm64"))]
pub mod constants_arch {
    include!("codegen/arm64/constants-arm64.rs");
}

#[cfg(all(target_arch = "x86", feature = "v8_target_arch_ia32"))]
pub mod constants_arch {
    include!("codegen/ia32/constants-ia32.rs");
}

#[cfg(all(target_arch = "mips64", target_endian = "big", feature = "v8_target_arch_mips64"))]
pub mod constants_arch {
    include!("codegen/mips64/constants-mips64.rs");
}

#[cfg(all(target_arch = "loongarch64", feature = "v8_target_arch_loong64"))]
pub mod constants_arch {
    include!("codegen/loong64/constants-loong64.rs");
}

#[cfg(all(target_arch = "powerpc64", feature = "v8_target_arch_ppc64"))]
pub mod constants_arch {
    include!("codegen/ppc/constants-ppc.rs");
}

#[cfg(all(target_arch = "s390x", feature = "v8_target_arch_s390x"))]
pub mod constants_arch {
    include!("codegen/s390/constants-s390.rs");
}

#[cfg(all(target_arch = "x86_64", feature = "v8_target_arch_x64"))]
pub mod constants_arch {
    include!("codegen/x64/constants-x64.rs");
}

#[cfg(any(all(target_arch = "riscv32", feature = "v8_target_arch_riscv32"), all(target_arch = "riscv64", feature = "v8_target_arch_riscv64")))]
pub mod constants_arch {
    include!("codegen/riscv/constants-riscv.rs");
}

#[cfg(not(any(
    all(target_arch = "arm", feature = "v8_target_arch_arm"),
    all(target_arch = "aarch64", feature = "v8_target_arch_arm64"),
    all(target_arch = "x86", feature = "v8_target_arch_ia32"),
    all(target_arch = "mips64", target_endian = "big", feature = "v8_target_arch_mips64"),
    all(target_arch = "loongarch64", feature = "v8_target_arch_loong64"),
    all(target_arch = "powerpc64", feature = "v8_target_arch_ppc64"),
    all(target_arch = "s390x", feature = "v8_target_arch_s390x"),
    all(target_arch = "x86_64", feature = "v8_target_arch_x64"),
    any(all(target_arch = "riscv32", feature = "v8_target_arch_riscv32"), all(target_arch = "riscv64", feature = "v8_target_arch_riscv64"))
)))]
compile_error!("Unsupported target architecture.  Please enable the appropriate feature (v8_target_arch_*) for your architecture.");
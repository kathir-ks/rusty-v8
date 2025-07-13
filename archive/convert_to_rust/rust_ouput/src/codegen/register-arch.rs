// Converted from V8 C++ source files:
// Header: register-arch.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

mod register_arch {
    #[cfg(target_arch = "x86")]
    pub mod ia32 {
        pub mod register_ia32;
        pub use register_ia32::*;
    }

    #[cfg(target_arch = "x86_64")]
    pub mod x64 {
        pub mod register_x64;
        pub use register_x64::*;
    }

    #[cfg(target_arch = "aarch64")]
    pub mod arm64 {
        pub mod register_arm64;
        pub use register_arm64::*;
    }

    #[cfg(target_arch = "arm")]
    pub mod arm {
        pub mod register_arm;
        pub use register_arm::*;
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod ppc {
        pub mod register_ppc;
        pub use register_ppc::*;
    }

    #[cfg(target_arch = "mips64")]
    pub mod mips64 {
        pub mod register_mips64;
        pub use register_mips64::*;
    }

    #[cfg(target_arch = "loongarch64")]
    pub mod loong64 {
        pub mod register_loong64;
        pub use register_loong64::*;
    }

    #[cfg(target_arch = "s390x")]
    pub mod s390 {
        pub mod register_s390;
        pub use register_s390::*;
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub mod riscv {
        pub mod register_riscv;
        pub use register_riscv::*;
    }

    pub mod register_base {
        pub trait Register {}
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct GeneralPurposeRegister {
            pub code: i32,
        }

        impl Register for GeneralPurposeRegister {}

        impl GeneralPurposeRegister {
            pub fn new(code: i32) -> Self {
                GeneralPurposeRegister { code }
            }

            pub fn is(&self, other: &Self) -> bool {
                self.code == other.code
            }
        }
    }
}

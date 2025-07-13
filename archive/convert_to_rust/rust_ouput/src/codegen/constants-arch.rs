// Converted from V8 C++ source files:
// Header: constants-arch.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[cfg(target_arch = "arm")]
mod constants_arch {
    include!("src/codegen/arm/constants-arm.rs");
}

#[cfg(target_arch = "aarch64")]
mod constants_arch {
    include!("src/codegen/arm64/constants-arm64.rs");
}

#[cfg(target_arch = "x86")]
mod constants_arch {
    include!("src/codegen/ia32/constants-ia32.rs");
}

#[cfg(target_arch = "mips64")]
mod constants_arch {
    include!("src/codegen/mips64/constants-mips64.rs");
}

#[cfg(target_arch = "loongarch64")]
mod constants_arch {
    include!("src/codegen/loong64/constants-loong64.rs");
}

#[cfg(target_arch = "powerpc64")]
mod constants_arch {
    include!("src/codegen/ppc/constants-ppc.rs");
}

#[cfg(target_arch = "s390x")]
mod constants_arch {
    include!("src/codegen/s390/constants-s390.rs");
}

#[cfg(target_arch = "x86_64")]
mod constants_arch {
    include!("src/codegen/x64/constants-x64.rs");
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod constants_arch {
    include!("src/codegen/riscv/constants-riscv.rs");
}

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64", target_arch = "x86", target_arch = "mips64", target_arch = "loongarch64", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86_64", target_arch = "riscv32", target_arch = "riscv64"))]
compile_error!("Unsupported target architecture.");

mod src {
    pub mod codegen {
        pub mod arm {
            pub mod constants_arm {
                // Mock implementation for arm constants
                pub const kRegisterSize: usize = 4;
            }
        }
        pub mod arm64 {
            pub mod constants_arm64 {
                // Mock implementation for arm64 constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod ia32 {
            pub mod constants_ia32 {
                // Mock implementation for ia32 constants
                pub const kRegisterSize: usize = 4;
            }
        }
        pub mod mips64 {
            pub mod constants_mips64 {
                // Mock implementation for mips64 constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod loong64 {
            pub mod constants_loong64 {
                // Mock implementation for loong64 constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod ppc {
            pub mod constants_ppc {
                // Mock implementation for ppc constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod s390 {
            pub mod constants_s390 {
                // Mock implementation for s390 constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod x64 {
            pub mod constants_x64 {
                // Mock implementation for x64 constants
                pub const kRegisterSize: usize = 8;
            }
        }
        pub mod riscv {
            pub mod constants_riscv {
                // Mock implementation for riscv constants
                pub const kRegisterSize: usize = 4;
            }
        }
    }
}

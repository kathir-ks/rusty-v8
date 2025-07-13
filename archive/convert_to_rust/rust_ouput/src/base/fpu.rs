// Converted from V8 C++ source files:
// Header: fpu.h
// Implementation: fpu.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[cfg(target_compiler = "gnu")]
    mod x64_gnu {
        use std::arch::asm;

        // Two bits on Intel CPUs, for FTZ (flush denormalized results to zero) and DAZ
        // (flush denormalized inputs to zero).
        const K_FLUSH_DENORM_TO_ZERO_BITS: i32 = 0x8040;

        fn get_csr() -> i32 {
            let result: i32;
            unsafe {
                asm!(
                    "stmxcsr {0}",
                    out("m")(result),
                );
            }
            result
        }

        fn set_csr(a: i32) {
            let temp = a;
            unsafe {
                asm!(
                    "ldmxcsr {0}",
                    in("m")(temp),
                );
            }
        }

        pub fn get_flush_denormals() -> bool {
            let csr = get_csr();
            csr & K_FLUSH_DENORM_TO_ZERO_BITS != 0
        }

        pub fn set_flush_denormals(value: bool) {
            let old_csr = get_csr();
            let new_csr = if value {
                old_csr | K_FLUSH_DENORM_TO_ZERO_BITS
            } else {
                old_csr & !K_FLUSH_DENORM_TO_ZERO_BITS
            };
            set_csr(new_csr);
        }
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[cfg(target_compiler = "msvc")]
    mod x64_msvc {
        use std::arch::x86_64::_controlfp_s;

        const _MCW_DN: u32 = 0x00030000;
        const _DN_FLUSH: u32 = 0x00030000;
        const _DN_SAVE: u32 = 0x00000000;

        pub fn get_flush_denormals() -> bool {
            let mut csr: u32 = 0;
            unsafe {
                _controlfp_s(&mut csr, 0, 0);
            }
            (csr & _MCW_DN) == _DN_FLUSH
        }

        pub fn set_flush_denormals(value: bool) {
            let mut csr: u32 = 0;
            let new_control = if value { _DN_FLUSH } else { _DN_SAVE };
            unsafe {
                _controlfp_s(&mut csr, new_control, _MCW_DN);
            }
        }
    }

    #[cfg(any(target_arch = "arm64", target_arch = "arm"))]
    mod arm {
        use std::arch::asm;

        // Bit 24 is the flush-to-zero mode control bit. Setting it to 1 flushes
        // denormals to 0.
        const K_FLUSH_DENORM_TO_ZERO_BIT: i32 = (1 << 24);

        fn get_status_word() -> i32 {
            let result: i32;
            unsafe {
                #[cfg(target_arch = "arm64")]
                asm!(
                    "mrs {0:x}, FPCR",
                    out(reg) result,
                );
                #[cfg(target_arch = "arm")]
                asm!(
                    "vmrs {0}, FPSCR",
                    out(reg) result,
                );
            }
            result
        }

        fn set_status_word(a: i32) {
            unsafe {
                #[cfg(target_arch = "arm64")]
                asm!(
                    "msr FPCR, {0:x}",
                    in(reg) a,
                );
                #[cfg(target_arch = "arm")]
                asm!(
                    "vmsr FPSCR, {0}",
                    in(reg) a,
                );
            }
        }

        pub fn get_flush_denormals() -> bool {
            let csr = get_status_word();
            csr & K_FLUSH_DENORM_TO_ZERO_BIT != 0
        }

        pub fn set_flush_denormals(value: bool) {
            let old_csr = get_status_word();
            let new_csr = if value {
                old_csr | K_FLUSH_DENORM_TO_ZERO_BIT
            } else {
                old_csr & !K_FLUSH_DENORM_TO_ZERO_BIT
            };
            set_status_word(new_csr);
        }
    }

    pub struct FPU {}

    impl FPU {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        pub fn get_flush_denormals() -> bool {
            #[cfg(target_compiler = "gnu")]
            return x64_gnu::get_flush_denormals();
            #[cfg(target_compiler = "msvc")]
            return x64_msvc::get_flush_denormals();
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        pub fn set_flush_denormals(value: bool) {
            #[cfg(target_compiler = "gnu")]
            return x64_gnu::set_flush_denormals(value);
            #[cfg(target_compiler = "msvc")]
            return x64_msvc::set_flush_denormals(value);
        }

        #[cfg(any(target_arch = "arm64", target_arch = "arm"))]
        pub fn get_flush_denormals() -> bool {
            arm::get_flush_denormals()
        }

        #[cfg(any(target_arch = "arm64", target_arch = "arm"))]
        pub fn set_flush_denormals(value: bool) {
            arm::set_flush_denormals(value)
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "arm64", target_arch = "arm")))]
        pub fn get_flush_denormals() -> bool {
            false
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "arm64", target_arch = "arm")))]
        pub fn set_flush_denormals(_value: bool) {}
    }

    pub struct FlushDenormalsScope {
        old_flush_state_: bool,
    }

    impl FlushDenormalsScope {
        pub fn new(value: bool) -> Self {
            let old_flush_state_ = FPU::get_flush_denormals();
            FPU::set_flush_denormals(value);
            FlushDenormalsScope { old_flush_state_ }
        }
    }

    impl Drop for FlushDenormalsScope {
        fn drop(&mut self) {
            FPU::set_flush_denormals(self.old_flush_state_);
        }
    }
}

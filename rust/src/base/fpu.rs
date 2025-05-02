// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/fpu.rs

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[cfg(target_env = "gnu")]
mod x86_gnu {
    // Two bits on Intel CPUs, for FTZ (flush denormalized results to zero) and DAZ
    // (flush denormalized inputs to zero).
    const K_FLUSH_DENORM_TO_ZERO_BITS: i32 = 0x8040;

    #[inline]
    fn get_csr() -> i32 {
        let result: i32;
        unsafe {
            asm!("stmxcsr %0" : "=m"(result));
        }
        result
    }

    #[inline]
    fn set_csr(a: i32) {
        let temp = a;
        unsafe {
            asm!("ldmxcsr %0" : : "m"(temp));
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
#[cfg(target_env = "msvc")]
mod x86_msvc {
    use std::arch::x86_64::_controlfp_s;

    pub fn get_flush_denormals() -> bool {
        let mut csr: u32 = 0;
        unsafe {
            _controlfp_s(&mut csr, 0, 0);
        }
        (csr & 0x00040000) == 0x00040000 // _MCW_DN, _DN_FLUSH
    }

    pub fn set_flush_denormals(value: bool) {
        let mut csr: u32 = 0;
        unsafe {
            _controlfp_s(&mut csr, if value { 0x00040000 } else { 0x00000000 }, 0x000C0000); // _DN_FLUSH, _DN_SAVE, _MCW_DN
        }
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
mod arm {
    // Bit 24 is the flush-to-zero mode control bit. Setting it to 1 flushes
    // denormals to 0.
    const K_FLUSH_DENORM_TO_ZERO_BIT: i32 = 1 << 24;

    #[cfg(target_arch = "aarch64")]
    #[inline]
    fn get_status_word() -> i32 {
        let result: i32;
        unsafe {
            asm!("mrs %x[result], FPCR" : [result] "=r"(result));
        }
        result
    }

    #[cfg(target_arch = "aarch64")]
    #[inline]
    fn set_status_word(a: i32) {
        unsafe {
            asm!("msr FPCR, %x[src]" : : [src] "r"(a));
        }
    }

    #[cfg(target_arch = "arm")]
    #[inline]
    fn get_status_word() -> i32 {
        let result: i32;
        unsafe {
            asm!("vmrs %[result], FPSCR" : [result] "=r"(result));
        }
        result
    }

    #[cfg(target_arch = "arm")]
    #[inline]
    fn set_status_word(a: i32) {
        unsafe {
            asm!("vmsr FPSCR, %[src]" : : [src] "r"(a));
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

pub struct FPU;

impl FPU {
    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_env = "gnu"))]
    pub fn get_flush_denormals() -> bool {
        x86_gnu::get_flush_denormals()
    }

    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_env = "gnu"))]
    pub fn set_flush_denormals(value: bool) {
        x86_gnu::set_flush_denormals(value);
    }

    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_env = "msvc"))]
    pub fn get_flush_denormals() -> bool {
        x86_msvc::get_flush_denormals()
    }

    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_env = "msvc"))]
    pub fn set_flush_denormals(value: bool) {
        x86_msvc::set_flush_denormals(value);
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
    pub fn get_flush_denormals() -> bool {
        arm::get_flush_denormals()
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
    pub fn set_flush_denormals(value: bool) {
        arm::set_flush_denormals(value);
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64", target_arch = "arm")))]
    pub fn get_flush_denormals() -> bool {
        false
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64", target_arch = "arm")))]
    pub fn set_flush_denormals(_value: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fpu_get_set_flush_denormals() {
        // Store the original value.
        let original_value = FPU::get_flush_denormals();

        // Set flush denormals to true.
        FPU::set_flush_denormals(true);
        assert_eq!(FPU::get_flush_denormals(), true);

        // Set flush denormals to false.
        FPU::set_flush_denormals(false);
        assert_eq!(FPU::get_flush_denormals(), false);

        // Restore the original value.
        FPU::set_flush_denormals(original_value);
    }
}
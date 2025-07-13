// Converted from V8 C++ source files:
// Header: immediate-crash.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        // x64-specific definitions
        #[cfg(target_os = "darwin")]
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("int3", options(att_syntax));
                    core::arch::asm!("ud2", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else if #[cfg(target_arch = "x86")] {
        // IA32-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("int3", options(att_syntax));
                    core::arch::asm!("ud2", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else if #[cfg(target_arch = "arm")] {
        // ARM-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("bkpt #0", options(att_syntax));
                    core::arch::asm!("udf #0", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else if #[cfg(target_arch = "aarch64")] {
        // ARM64-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("brk #0", options(att_syntax));
                    core::arch::asm!("hlt #0", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else if #[cfg(target_arch = "powerpc64")] {
        // PPC64-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    #[cfg(target_os = "aix")]
                    core::arch::asm!(".vbyte 4,0x7D821008", options(att_syntax));
                    #[cfg(not(target_os = "aix"))]
                    core::arch::asm!(".4byte 0x7D821008", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    }else if #[cfg(target_os = "zos")] {
        // zOS-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("trap", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else if #[cfg(target_arch = "s390x")] {
        // S390X-specific definitions
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!(".2byte 0x0001", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    } else {
        // Generic fallback
        macro_rules! trap_sequence {
            () => {
                unsafe {
                    core::arch::asm!("trap", options(att_syntax));
                    core::arch::asm!("", options(att_syntax)); // Ensure no folding
                }
            }
        }
    }
}

#[allow(unreachable_code)]
pub fn immediate_crash() -> ! {
    trap_sequence!();
    std::process::abort();
}

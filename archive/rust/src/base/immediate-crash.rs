// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This Rust code provides a best-effort equivalent
// to the C++ code, but direct translation isn't always possible
// due to differences in language features and system-level
// interactions.  The assembly snippets are placeholder and
// should be replaced with platform-specific, functionally
// equivalent Rust inline assembly, if needed.

#[cfg(target_arch = "x86_64")]
mod immediate_crash {
    #[cfg(target_os = "darwin")]
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("int3");
            }
        };
    }

    #[cfg(not(target_os = "darwin"))]
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("int3");
            }
        };
    }

    #[cfg(target_os = "darwin")]
    macro_rules! trap_sequence2 {
        () => {
             unsafe {
                core::arch::asm!("");
             }
        };
    }

    #[cfg(not(target_os = "darwin"))]
    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("ud2");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        // This is the best we can do to emulate __builtin_unreachable().
        // It may not have the same effects on optimization or code density.
        unreachable!()
    }
}

#[cfg(target_arch = "x86")]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("int3");
            }
        };
    }

    #[cfg(target_os = "darwin")]
    macro_rules! trap_sequence2 {
        () => {
             unsafe {
                core::arch::asm!("");
             }
        };
    }

    #[cfg(not(target_os = "darwin"))]
    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("ud2");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        // This is the best we can do to emulate __builtin_unreachable().
        // It may not have the same effects on optimization or code density.
        unreachable!()
    }
}

#[cfg(target_arch = "arm")]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("bkpt #0");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("udf #0");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(target_arch = "aarch64")]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("brk #0");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("hlt #0");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(target_arch = "powerpc64")]
mod immediate_crash {
    #[cfg(target_os = "aix")]
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!(".vbyte 4,0x7D821008");
            }
        };
    }

    #[cfg(not(target_os = "aix"))]
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!(".4byte 0x7D821008");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(target_os = "zos")]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("trap"); //Placeholder, needs investigation for a proper trap.
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(target_arch = "s390x")]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!(".2byte 0x0001");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64", target_os = "zos", target_arch = "s390x")))]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("trap"); // Placeholder for an architecture-neutral trap instruction
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(all(target_env = "msvc", not(target_feature = "llvm")))]
mod immediate_crash {
    #[inline(never)]
    pub fn immediate_crash() -> ! {
        unsafe {
            core::arch::asm!("__debugbreak");
        }
        unreachable!()
    }
}

#[cfg(all(target_env = "msvc", target_arch = "aarch64", target_feature = "llvm"))]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("brk #0xF000");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(all(target_env = "msvc", not(target_arch = "aarch64"), target_feature = "llvm"))]
mod immediate_crash {
    macro_rules! trap_sequence1 {
        () => {
            unsafe {
                core::arch::asm!("int3");
            }
        };
    }

    macro_rules! trap_sequence2 {
        () => {
            unsafe {
                core::arch::asm!("ud2");
            }
        };
    }

    macro_rules! trap_sequence {
        () => {
            trap_sequence1!();
            trap_sequence2!();
        };
    }

    macro_rules! wrapped_trap_sequence {
        () => {
            trap_sequence!();
        };
    }

    #[inline(never)]
    pub fn immediate_crash() -> ! {
        wrapped_trap_sequence!();
        unreachable!()
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64", target_os = "zos", target_arch = "s390x", all(target_env = "msvc", not(target_feature = "llvm")), all(target_env = "msvc", target_arch = "aarch64", target_feature = "llvm"), all(target_env = "msvc", not(target_arch = "aarch64"), target_feature = "llvm"))))]
mod immediate_crash {
    #[inline(never)]
    pub fn immediate_crash() -> ! {
        panic!("Unsupported architecture for immediate crash");
    }
}
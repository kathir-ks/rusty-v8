// Converted from V8 C++ source files:
// Header: N/A
// Implementation: handler-outside-simulator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[cfg(all(
    target_arch = "x86_64",
    any(target_os = "linux", target_os = "macos", target_os = "windows")
))]
mod probe_memory {
    #[cfg(target_os = "windows")]
    #[naked]
    pub unsafe extern "C" fn v8_internal_simulator_ProbeMemory() {
        asm!(
            ".att_syntax",
            ".globl v8_internal_simulator_ProbeMemory",
            "v8_internal_simulator_ProbeMemory:",
            "  movb (%rcx), %al",
            "  xorl %eax, %eax",
            "  ret",
            options(noreturn)
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[naked]
    pub unsafe extern "C" fn v8_internal_simulator_ProbeMemory() {
        asm!(
            ".att_syntax",
            ".globl v8_internal_simulator_ProbeMemory",
            "v8_internal_simulator_ProbeMemory:",
            "  movb (%rdi), %al",
            "  xorl %eax, %eax",
            "  ret",
            options(noreturn)
        );
    }

    #[naked]
    pub unsafe extern "C" fn v8_simulator_probe_memory_continuation() {
        asm!(
            ".att_syntax",
            ".globl v8_simulator_probe_memory_continuation",
            "v8_simulator_probe_memory_continuation:",
            "  ret",
            options(noreturn)
        );
    }
}

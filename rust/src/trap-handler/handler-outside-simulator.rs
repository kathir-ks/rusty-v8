#![cfg(all(target_arch = "x86_64", feature = "trap-handler"))]

#[cfg(target_os = "macos")]
macro_rules! symbol {
    ($name:ident) => {
        concat!("_", stringify!($name))
    };
}

#[cfg(not(target_os = "macos"))]
macro_rules! symbol {
    ($name:ident) => {
        stringify!($name)
    };
}

// This Rust code provides the equivalent assembly code using inline assembly.
// It's functionally equivalent to the original C++'s asm block.

#[cfg(target_os = "windows")]
#[rustfmt::skip]
#[naked]
pub unsafe extern "C" fn v8_internal_simulator_ProbeMemory() -> i32 {
    // First parameter (address) passed in %rdi on Linux/Mac, and %rcx on Windows.
    // The second parameter (pc) is unused here. It is read by the trap handler
    // instead.
    //
    // movb (%rcx), %al
    // xorl %eax, %eax
    // ret
    core::arch::asm!(
        ".att_syntax",
        ".globl {sym}",
        "{sym}:",
        "movb (%rcx), %al",
        "xorl %eax, %eax",
        "ret",
        sym = sym v8_internal_simulator_ProbeMemory,
        options(noreturn)
    )
}

#[cfg(not(target_os = "windows"))]
#[rustfmt::skip]
#[naked]
pub unsafe extern "C" fn v8_internal_simulator_ProbeMemory() -> i32 {
    // First parameter (address) passed in %rdi on Linux/Mac, and %rcx on Windows.
    // The second parameter (pc) is unused here. It is read by the trap handler
    // instead.
    //
    // movb (%rdi), %al
    // xorl %eax, %eax
    // ret
    core::arch::asm!(
        ".att_syntax",
        ".globl {sym}",
        "{sym}:",
        "movb (%rdi), %al",
        "xorl %eax, %eax",
        "ret",
        sym = sym v8_internal_simulator_ProbeMemory,
        options(noreturn)
    )
}

#[rustfmt::skip]
#[naked]
pub unsafe extern "C" fn v8_simulator_probe_memory_continuation() {
    // If the trap handler continues here, it wrote the landing pad in %rax.
    // ret
    core::arch::asm!(
        ".att_syntax",
        ".globl {sym}",
        "{sym}:",
        "ret",
        sym = sym v8_simulator_probe_memory_continuation,
        options(noreturn)
    )
}
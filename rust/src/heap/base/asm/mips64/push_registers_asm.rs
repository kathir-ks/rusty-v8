#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This Rust code represents a direct translation of the provided MIPS64 assembly
// into a Rust function that executes the same instructions inline.
// Due to the nature of assembly and its direct interaction with the hardware,
// a 100% safe Rust equivalent is generally not possible without relying on inline assembly.
// The following code provides the equivalent functionality using inline assembly.

use std::arch::asm;

/// Executes the assembly code that pushes all callee-saved registers,
/// calls a provided callback, and then restores the registers.
///
/// # Safety
///
/// This function uses inline assembly and is inherently unsafe. It relies
/// on the caller to ensure that the stack and registers are in a valid state
/// before and after the function call. The `callback` function must also
/// adhere to the expected calling conventions and stack frame layout.
///
/// # Arguments
///
/// * `stack`: A pointer to a `Stack` structure (opaque type).  It is assumed this is a pointer.
/// * `visitor`: A pointer to a `StackVisitor` structure (opaque type). It is assumed this is a pointer.
/// * `callback`: A function pointer to a callback function that accepts `stack`, `visitor`, and stack pointer as arguments.
pub unsafe fn push_all_registers_and_iterate_stack(
    stack: *mut core::ffi::c_void,
    visitor: *mut core::ffi::c_void,
    callback: fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
) {
    asm!(
        ".set noreorder",
        // Push all callee-saved registers and save return address.
        "daddiu $sp, $sp, -96",
        "sd $ra, 88($sp)",
        "sd $s8, 80($sp)",
        "sd $sp, 72($sp)",
        "sd $gp, 64($sp)",
        "sd $s7, 56($sp)",
        "sd $s6, 48($sp)",
        "sd $s5, 40($sp)",
        "sd $s4, 32($sp)",
        "sd $s3, 24($sp)",
        "sd $s2, 16($sp)",
        "sd $s1, 8($sp)",
        "sd $s0, 0($sp)",
        // Maintain frame pointer.
        "move $s8, $sp",
        // Pass 1st parameter (a0) unchanged (Stack*).
        // Pass 2nd parameter (a1) unchanged (StackVisitor*).
        // Save 3rd parameter (a2; IterateStackCallback).
        "move $t9, $a2",
        // Call the callback.
        "jalr $t9",
        // Delay slot: Pass 3rd parameter as sp (stack pointer).
        "move $a2, $sp",
        // Load return address.
        "ld $ra, 88($sp)",
        // Restore frame pointer.
        "ld $s8, 80($sp)",
        "jr $ra",
        // Delay slot: Discard all callee-saved registers.
        "daddiu $sp, $sp, 96",

        // Input registers:
        in("a0") stack,
        in("a1") visitor,
        in("a2") callback,
        //use_sym callback,

        // clobber_sym: This feature doesn't exist.

        // Clobbered registers:
        clobber_abi("C"),
        options(noreturn) //This is needed so no return will be generated.

    );
}
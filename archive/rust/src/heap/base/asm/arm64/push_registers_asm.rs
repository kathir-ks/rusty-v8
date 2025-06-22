#![allow(unused_unsafe)]
#![allow(non_snake_case)]

#[cfg(target_arch = "aarch64")]
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
#[cfg(not(feature = "v8_enable_control_flow_integrity"))]
#[cfg(not(test))] // Exclude from tests as they might not support inline assembly

mod push_registers_asm {
    use std::arch::asm;

    // This function pushes all callee-saved registers onto the stack and then
    // calls a callback function with the stack pointer. This is used for conservative
    // stack scanning.  The original C++ used inline assembly.  Rust doesn't
    // have a direct equivalent that is portable.  This is the most direct
    // translation given the contraints.  It relies on the `asm!` macro.
    //
    // Note:  This requires the `unsafe` keyword as inline assembly is inherently unsafe.
    // and relies on inline assembly.
    //
    // It mirrors the original C++ code as closely as possible, including
    // the use of x7 to temporarily store the callback function pointer.

    #[naked]
    pub unsafe extern "C" fn PushAllRegistersAndIterateStack(
        stack: *mut (),
        visitor: *mut (),
        callback: extern "C" fn(*mut (), *mut (), *mut ()) -> (),
    ) {
        asm!(
            // x19-x29 are callee-saved.
            "stp x19, x20, [sp, #-16]!",
            "stp x21, x22, [sp, #-16]!",
            "stp x23, x24, [sp, #-16]!",
            "stp x25, x26, [sp, #-16]!",
            "stp x27, x28, [sp, #-16]!",
            "stp fp, lr,   [sp, #-16]!",
            // Maintain frame pointer.
            "mov fp, sp",
            // Pass 1st parameter (x0) unchanged (Stack*).
            // Pass 2nd parameter (x1) unchanged (StackVisitor*).
            // Save 3rd parameter (x2; IterateStackCallback)
            "mov x7, x2",
            // Pass 3rd parameter as sp (stack pointer).
            "mov x2, sp",
            "blr x7",
            // Load return address and frame pointer.
            "ldp fp, lr, [sp], #16",
            // Drop all callee-saved registers.
            "add sp, sp, #80",
            "ret",
            options(noreturn)
        );
    }
}
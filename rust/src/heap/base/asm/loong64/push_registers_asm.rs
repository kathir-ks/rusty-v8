#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::arch::asm;

/// Pushes all callee-saved registers onto the stack and then calls a callback.
/// This function is architecture-specific (LoongArch64) and written in assembly.
///
/// # Safety
///
/// This function is inherently unsafe due to its direct manipulation of the stack
/// and registers. The caller must ensure that the `stack`, `visitor`, and `callback`
/// parameters are valid and that the stack is properly aligned.
///
/// # Arguments
///
/// * `stack`: A pointer to a `Stack` object (C++: `Stack*`).
/// * `visitor`: A pointer to a `StackVisitor` object (C++: `StackVisitor*`).
/// * `callback`: A function pointer to the stack iteration callback (C++: `IterateStackCallback`).
///
/// # Calling Convention (LoongArch64)
///
/// The function adheres to the standard LoongArch64 calling convention.
/// Callee-saved registers are pushed onto the stack, the callback is invoked,
/// and then the registers are restored before returning.
///
/// # Stack Frame Layout
///
/// The stack frame created by this function has the following layout:
///
/// ```text
/// +-----------------+  <-- sp (initial)
/// | Return Address  |
/// +-----------------+  +8
/// | Previous sp     |
/// +-----------------+  +16
/// | Previous fp     |
/// +-----------------+  +24
/// | s0              |
/// +-----------------+  +32
/// | s1              |
/// +-----------------+  +40
/// | s2              |
/// +-----------------+  +48
/// | s3              |
/// +-----------------+  +56
/// | s4              |
/// +-----------------+  +64
/// | s5              |
/// +-----------------+  +72
/// | s6              |
/// +-----------------+  +80
/// | s7              |
/// +-----------------+  +88
/// | s8              |
/// +-----------------+  <-- sp (after push)
/// ```
///
/// # Assembly Code Explanation
///
/// The assembly code performs the following steps:
///
/// 1.  Adjusts the stack pointer (`sp`) to allocate space for the callee-saved registers.
/// 2.  Saves the callee-saved registers (`s0` - `s8`, `fp`, `sp`, `ra`) onto the stack.
/// 3.  Maintains the frame pointer (`fp`).
/// 4.  Preserves the first two arguments (`a0`, `a1`).
/// 5.  Saves the third argument (`a2`) in `t7`.
/// 6.  Sets `a2` to `sp` and calls the callback function.
/// 7.  Restores the return address (`ra`) and frame pointer (`fp`).
/// 8.  Adjusts the stack pointer to deallocate the stack frame.
/// 9.  Returns to the caller.

#[cfg(target_arch = "loongarch64")]
#[naked]
pub unsafe extern "C" fn PushAllRegistersAndIterateStack(
    stack: *mut std::ffi::c_void,
    visitor: *mut std::ffi::c_void,
    callback: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void, *mut std::ffi::c_void),
) {
    asm!(
        ".text",
        ".global PushAllRegistersAndIterateStack",
        ".type PushAllRegistersAndIterateStack, %function",
        ".hidden PushAllRegistersAndIterateStack",
        "PushAllRegistersAndIterateStack:",
        // Push all callee-saved registers and save return address.
        "addi.d $sp, $sp, -96",
        "st.d $s8, $sp, 88",
        "st.d $s7, $sp, 80",
        "st.d $s6, $sp, 72",
        "st.d $s5, $sp, 64",
        "st.d $s4, $sp, 56",
        "st.d $s3, $sp, 48",
        "st.d $s2, $sp, 40",
        "st.d $s1, $sp, 32",
        "st.d $s0, $sp, 24",
        "st.d $fp, $sp, 16",
        "st.d $sp, $sp,  8",
        "st.d $ra, $sp,  0",
        // Maintain frame pointer.
        "addi.d $fp, $sp, 0",
        // Pass 1st parameter (a0) unchanged (Stack*).
        // Pass 2nd parameter (a1) unchanged (StackVisitor*).
        // Save 3rd parameter (a2; IterateStackCallback).
        "addi.d $t7, $a2, 0",
        // Call the callback.
        // Pass 3rd parameter as sp (stack pointer).
        "addi.d $a2, $sp, 0",
        "jirl $ra, $t7, 0",
        // Load return address.
        "ld.d $ra, $sp, 0",
        // Restore frame pointer.
        "ld.d $fp, $sp, 16",
        // Discard all callee-saved registers.
        "addi.d $sp, $sp, 96",
        "jirl $zero, $ra, 0",
        ".Lfunc_end0:",
        options(noreturn)
    )
}

#[cfg(not(target_arch = "loongarch64"))]
pub unsafe extern "C" fn PushAllRegistersAndIterateStack(
    _stack: *mut std::ffi::c_void,
    _visitor: *mut std::ffi::c_void,
    _callback: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void, *mut std::ffi::c_void),
) {
    panic!("PushAllRegistersAndIterateStack is only implemented for loongarch64 architecture.");
}
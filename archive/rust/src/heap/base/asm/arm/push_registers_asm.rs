#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This Rust code provides a function `push_all_registers_and_iterate_stack`
// that mirrors the functionality of the original C++ assembly code.
//
// The C++ code pushes all callee-saved registers (r4-r11) and the link register (lr)
// onto the stack, along with r3 to ensure 8-byte alignment.  It then calls a
// provided callback function with the stack pointer as an argument. Finally, it
// restores the stack and returns.
//
// The Rust implementation uses inline assembly to achieve the same effect.
//
// Note that direct equivalent register names and operations may not be portable
// across architectures. This is an ARM32 specific implementation.

// The function signature matches the expected behavior of the callback.
type IterateStackCallback = extern "C" fn(*mut Stack, *mut StackVisitor, *const u8) -> ();

// Placeholder structs for Stack and StackVisitor.  These need to be defined
// according to their actual structure in the V8 codebase.
#[repr(C)]
pub struct Stack {}

#[repr(C)]
pub struct StackVisitor {}

#[cfg(target_arch = "arm")]
#[naked]
pub unsafe extern "C" fn push_all_registers_and_iterate_stack(
    stack: *mut Stack,
    visitor: *mut StackVisitor,
    callback: IterateStackCallback,
) {
    // Assembly code equivalent to the original C++ assembly.
    //
    // Note: This uses inline assembly, which is architecture-specific.
    //       Ensure that the correct architecture is targeted when compiling.
    llvm_asm!(
        "push {r3-r11, lr}"
        "mov r3, r2"
        "mov r2, sp"
        "blx r3"
        "add sp, sp, #36"
        "pop {pc}"
        options(noreturn)
    )
}

#[cfg(not(target_arch = "arm"))]
pub unsafe extern "C" fn push_all_registers_and_iterate_stack(
    _stack: *mut Stack,
    _visitor: *mut StackVisitor,
    _callback: IterateStackCallback,
) {
    // Dummy implementation if not on ARM
    panic!("push_all_registers_and_iterate_stack only implemented for ARM");
}
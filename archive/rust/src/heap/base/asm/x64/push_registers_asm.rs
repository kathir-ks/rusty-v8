#![allow(unused_unsafe)]
#![allow(dead_code)]

#[cfg(target_os = "windows")]
compile_error!("The masm based version must be used for Windows");

#[cfg(target_os = "macos")]
#[link_section = ".text"]
extern "C" {
    #[link_name = "_PushAllRegistersAndIterateStack"]
    pub fn push_all_registers_and_iterate_stack(stack: *mut u8, visitor: *mut u8, callback: extern "C" fn(*mut u8, *mut u8, *mut u8));
}

#[cfg(not(target_os = "macos"))]
#[cfg(target_arch = "x86_64")]
#[link_section = ".text"]
extern "C" {
    #[link_name = "PushAllRegistersAndIterateStack"]
    pub fn push_all_registers_and_iterate_stack(stack: *mut u8, visitor: *mut u8, callback: extern "C" fn(*mut u8, *mut u8, *mut u8));
}

#[cfg(all(not(target_os = "macos"), target_arch = "x86_64"))]
#[macro_export]
macro_rules! push_all_registers_asm {
    () => {
        core::arch::global_asm!(
            ".global PushAllRegistersAndIterateStack",
            ".type PushAllRegistersAndIterateStack, %function",
            ".hidden PushAllRegistersAndIterateStack",
            "PushAllRegistersAndIterateStack:",
            // rbp is callee-saved. Maintain proper frame pointer for debugging.
            "  push %rbp",
            "  mov %rsp, %rbp",
            // Dummy for alignment.
            "  push $0xCDCDCD",
            "  push %rbx",
            "  push %r12",
            "  push %r13",
            "  push %r14",
            "  push %r15",
            // Pass 1st parameter (rdi) unchanged (Stack*).
            // Pass 2nd parameter (rsi) unchanged (StackVisitor*).
            // Save 3rd parameter (rdx; IterateStackCallback)
            "  mov %rdx, %r8",
            // Pass 3rd parameter as rsp (stack pointer).
            "  mov %rsp, %rdx",
            // Call the callback.
            "  call *%r8",
            // Pop the callee-saved registers.
            "  add $48, %rsp",
            // Restore rbp as it was used as frame pointer.
            "  pop %rbp",
            "  ret",
            ".Lfunc_end0:",
            ".size PushAllRegistersAndIterateStack, .Lfunc_end0-PushAllRegistersAndIterateStack",
            options(att_syntax)
        );
    };
}

#[cfg(all(not(target_os = "macos"), target_arch = "x86_64"))]
push_all_registers_asm!();

#[cfg(target_os = "macos")]
#[macro_export]
macro_rules! push_all_registers_asm {
    () => {
        core::arch::global_asm!(
            ".global _PushAllRegistersAndIterateStack",
            ".private_extern _PushAllRegistersAndIterateStack",
            "_PushAllRegistersAndIterateStack:",
            // rbp is callee-saved. Maintain proper frame pointer for debugging.
            "  push %rbp",
            "  mov %rsp, %rbp",
            // Dummy for alignment.
            "  push $0xCDCDCD",
            "  push %rbx",
            "  push %r12",
            "  push %r13",
            "  push %r14",
            "  push %r15",
            // Pass 1st parameter (rdi) unchanged (Stack*).
            // Pass 2nd parameter (rsi) unchanged (StackVisitor*).
            // Save 3rd parameter (rdx; IterateStackCallback)
            "  mov %rdx, %r8",
            // Pass 3rd parameter as rsp (stack pointer).
            "  mov %rsp, %rdx",
            // Call the callback.
            "  call *%r8",
            // Pop the callee-saved registers.
            "  add $48, %rsp",
            // Restore rbp as it was used as frame pointer.
            "  pop %rbp",
            "  ret",
            options(att_syntax)
        );
    };
}

#[cfg(target_os = "macos")]
push_all_registers_asm!();
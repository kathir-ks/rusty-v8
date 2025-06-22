#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

cfg_if::cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn PushAllRegistersAndIterateStack() {
            asm!(
                // Push all callee-saved registers and save return address.
                "addi sp, sp, -112",
                // Save return address.
                "sd ra, 104(sp)",
                // sp is callee-saved.
                "sd sp, 96(sp)",
                // s0-s11 are callee-saved.
                "sd s11, 88(sp)",
                "sd s10, 80(sp)",
                "sd s9, 72(sp)",
                "sd s8, 64(sp)",
                "sd s7, 56(sp)",
                "sd s6, 48(sp)",
                "sd s5, 40(sp)",
                "sd s4, 32(sp)",
                "sd s3, 24(sp)",
                "sd s2, 16(sp)",
                "sd s1,  8(sp)",
                "sd s0,  0(sp)",
                // Maintain frame pointer(fp is s0).
                "mv s0, sp",
                // Pass 1st parameter (a0) unchanged (Stack*).
                // Pass 2nd parameter (a1) unchanged (StackVisitor*).
                // Save 3rd parameter (a2; IterateStackCallback) to a3.
                "mv a3, a2",
                // Pass 3rd parameter as sp (stack pointer).
                "mv a2, sp",
                // Call the callback.
                "jalr a3",
                // Load return address.
                "ld ra, 104(sp)",
                // Restore frame pointer.
                "ld s0, 0(sp)",
                "addi sp, sp, 112",
                "jr ra",
                options(noreturn)
            )
        }
    } else if #[cfg(target_arch = "riscv32")] {
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn PushAllRegistersAndIterateStack() {
            asm!(
                // Push all callee-saved registers and save return address.
                "addi sp, sp, -56",
                // Save return address.
                "sw ra, 52(sp)",
                // sp is callee-saved.
                "sw sp, 48(sp)",
                // s0-s11 are callee-saved.
                "sw s11, 44(sp)",
                "sw s10, 40(sp)",
                "sw s9, 36(sp)",
                "sw s8, 32(sp)",
                "sw s7, 28(sp)",
                "sw s6, 24(sp)",
                "sw s5, 20(sp)",
                "sw s4, 16(sp)",
                "sw s3, 12(sp)",
                "sw s2, 8(sp)",
                "sw s1,  4(sp)",
                "sw s0,  0(sp)",
                // Maintain frame pointer(fp is s0).
                "mv s0, sp",
                // Pass 1st parameter (a0) unchanged (Stack*).
                // Pass 2nd parameter (a1) unchanged (StackVisitor*).
                // Save 3rd parameter (a2; IterateStackCallback) to a3.
                "mv a3, a2",
                // Pass 3rd parameter as sp (stack pointer).
                "mv a2, sp",
                // Call the callback.
                "jalr a3",
                // Load return address.
                "lw ra, 52(sp)",
                // Restore frame pointer.
                "lw s0, 0(sp)",
                "addi sp, sp, 56",
                "jr ra",
                options(noreturn)
            )
        }
    } else {
        compile_error!("Unsupported architecture");
    }
}
// Converted from V8 C++ source files:
// Header: reglist-ia32.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod codegen_ia32_reglist_ia32 {
    use crate::codegen::reglist_base::RegListBase;
    use crate::codegen::ia32::register_ia32::Register;
    use crate::codegen::ia32::register_ia32::DoubleRegister;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    // Caller-saved registers
    pub const K_JS_CALLER_SAVED: RegList = RegList {
        bits_: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 7), // eax, ecx, edx, ebx, edi
    };

    // Caller-saved registers according to the x86 ABI
    pub const K_CALLER_SAVED: RegList = RegList {
        bits_: (1 << 0) | (1 << 1) | (1 << 2), // eax, ecx, edx
    };

    pub const K_NUM_JS_CALLER_SAVED: i32 = 5;
}

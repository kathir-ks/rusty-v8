// Converted from V8 C++ source files:
// Header: reglist-s390.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod reglist_s390 {
    use crate::codegen::register_arch::*;
    use crate::codegen::reglist_base::*;
    use std::marker::Copy;
    use std::marker::Clone;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    //This is not strictly a compile time check, but it is close enough
    trait AssertTriviallyCopyable: Copy + Clone {}
    impl AssertTriviallyCopyable for RegList {}
    impl AssertTriviallyCopyable for DoubleRegList {}

    // Register list in load/store instructions
    // Note that the bit values must match those used in actual instruction encoding

    // Caller-saved/arguments registers
    pub const K_JS_CALLER_SAVED: RegList = RegList {
        registers: [
            Register::r1,
            Register::r2,
            Register::r3,
            Register::r4,
            Register::r5,
        ],
        count: 5,
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 5;

    // Callee-saved registers preserved when switching from C to JavaScript
    pub const K_CALLEE_SAVED: RegList = RegList {
        registers: [
            Register::r6,
            Register::r7,
            Register::r8,
            Register::r9,
            Register::r10,
            Register::fp,
            Register::ip,
            Register::r13,
        ],
        count: 8,
    };
    // r15;   // r15 (sp in Javascript)

    pub const K_NUM_CALLEE_SAVED: usize = 8;

    pub const K_CALLER_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: [
            DoubleRegister::d0,
            DoubleRegister::d1,
            DoubleRegister::d2,
            DoubleRegister::d3,
            DoubleRegister::d4,
            DoubleRegister::d5,
            DoubleRegister::d6,
            DoubleRegister::d7,
        ],
        count: 8,
    };

    pub const K_NUM_CALLER_SAVED_DOUBLES: usize = 8;

    pub const K_CALLEE_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: [
            DoubleRegister::d8,
            DoubleRegister::d9,
            DoubleRegister::d10,
            DoubleRegister::d11,
            DoubleRegister::d12,
            DoubleRegister::d13,
            DoubleRegister::d14,
            DoubleRegister::d15,
        ],
        count: 8,
    };

    pub const K_NUM_CALLEE_SAVED_DOUBLES: usize = 8;
}

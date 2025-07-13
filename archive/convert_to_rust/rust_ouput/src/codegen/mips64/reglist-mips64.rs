// Converted from V8 C++ source files:
// Header: reglist-mips64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod reglist_mips64 {
    use crate::codegen::mips64::constants_mips64::*;
    use crate::codegen::register_arch::*;
    use crate::codegen::reglist_base::*;
    use std::marker::Copy;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    const_assert!(std::mem::size_of::<RegList>() == std::mem::size_of::<RegList>());
    const_assert!(std::mem::size_of::<DoubleRegList>() == std::mem::size_of::<DoubleRegList>());

    lazy_static::lazy_static! {
        pub static ref K_JS_CALLER_SAVED: RegList = RegList::from_registers(&[
            v0, v1, a0, a1, a2, a3, a4,
            a5, a6, a7, t0, t1, t2, t3
        ]);
    }

    pub const K_NUM_JS_CALLER_SAVED: i32 = 14;

    lazy_static::lazy_static! {
        pub static ref K_CALLEE_SAVED: RegList = RegList::from_registers(&[
            s0, s1, s2, s3,
            s4, s5, s6, s7,
            fp
        ]);
    }

    pub const K_NUM_CALLEE_SAVED: i32 = 9;

    lazy_static::lazy_static! {
        pub static ref K_CALLEE_SAVED_FPU: DoubleRegList = DoubleRegList::from_registers(&[
            f20, f22, f24, f26, f28, f30
        ]);
    }

    pub const K_NUM_CALLEE_SAVED_FPU: i32 = 6;

    lazy_static::lazy_static! {
        pub static ref K_CALLER_SAVED_FPU: DoubleRegList = DoubleRegList::from_registers(&[
            f0,  f2,  f4,  f6,  f8,
            f10, f12, f14, f16, f18
        ]);
    }
}

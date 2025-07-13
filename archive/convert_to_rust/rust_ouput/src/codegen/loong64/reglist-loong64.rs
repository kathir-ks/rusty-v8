// Converted from V8 C++ source files:
// Header: reglist-loong64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod loong64 {
pub mod reglist_loong64 {
    use crate::codegen::reglist_base::RegListBase;
    use crate::codegen::register_arch::Register;
    use std::marker::Copy;
    use std::marker::Clone;

    #[derive(Clone, Copy)]
    pub struct RegList(RegListBase<Register>);
    #[derive(Clone, Copy)]
    pub struct DoubleRegList(RegListBase<DoubleRegister>);

    // Dummy definitions for Register and DoubleRegister
    #[derive(Clone, Copy)]
    pub struct DoubleRegister {}

    const A0: Register = Register {};
    const A1: Register = Register {};
    const A2: Register = Register {};
    const A3: Register = Register {};
    const A4: Register = Register {};
    const A5: Register = Register {};
    const A6: Register = Register {};
    const A7: Register = Register {};
    const T0: Register = Register {};
    const T1: Register = Register {};
    const T2: Register = Register {};
    const T3: Register = Register {};
    const T4: Register = Register {};
    const T5: Register = Register {};
    const T8: Register = Register {};
    const FP: Register = Register {};
    const S0: Register = Register {};
    const S1: Register = Register {};
    const S2: Register = Register {};
    const S3: Register = Register {};
    const S4: Register = Register {};
    const S5: Register = Register {};
    const S6: Register = Register {};
    const S7: Register = Register {};
    const S8: Register = Register {};

    const F0: DoubleRegister = DoubleRegister {};
    const F1: DoubleRegister = DoubleRegister {};
    const F2: DoubleRegister = DoubleRegister {};
    const F3: DoubleRegister = DoubleRegister {};
    const F4: DoubleRegister = DoubleRegister {};
    const F5: DoubleRegister = DoubleRegister {};
    const F6: DoubleRegister = DoubleRegister {};
    const F7: DoubleRegister = DoubleRegister {};
    const F8: DoubleRegister = DoubleRegister {};
    const F9: DoubleRegister = DoubleRegister {};
    const F10: DoubleRegister = DoubleRegister {};
    const F11: DoubleRegister = DoubleRegister {};
    const F12: DoubleRegister = DoubleRegister {};
    const F13: DoubleRegister = DoubleRegister {};
    const F14: DoubleRegister = DoubleRegister {};
    const F15: DoubleRegister = DoubleRegister {};
    const F16: DoubleRegister = DoubleRegister {};
    const F17: DoubleRegister = DoubleRegister {};
    const F18: DoubleRegister = DoubleRegister {};
    const F19: DoubleRegister = DoubleRegister {};
    const F20: DoubleRegister = DoubleRegister {};
    const F21: DoubleRegister = DoubleRegister {};
    const F22: DoubleRegister = DoubleRegister {};
    const F23: DoubleRegister = DoubleRegister {};
    const F24: DoubleRegister = DoubleRegister {};
    const F25: DoubleRegister = DoubleRegister {};
    const F26: DoubleRegister = DoubleRegister {};
    const F27: DoubleRegister = DoubleRegister {};
    const F28: DoubleRegister = DoubleRegister {};
    const F29: DoubleRegister = DoubleRegister {};
    const F30: DoubleRegister = DoubleRegister {};
    const F31: DoubleRegister = DoubleRegister {};

    pub const K_JS_CALLER_SAVED: RegList = RegList(RegListBase {
        registers: [
            Some(A0),
            Some(A1),
            Some(A2),
            Some(A3),
            Some(A4),
            Some(A5),
            Some(A6),
            Some(A7),
            Some(T0),
            Some(T1),
            Some(T2),
            Some(T3),
            Some(T4),
            Some(T5),
            Some(T8),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    });

    pub const K_NUM_JS_CALLER_SAVED: i32 = 15;

    pub const K_CALLEE_SAVED: RegList = RegList(RegListBase {
        registers: [
            Some(FP),
            Some(S0),
            Some(S1),
            Some(S2),
            Some(S3),
            Some(S4),
            Some(S5),
            Some(S6),
            Some(S7),
            Some(S8),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    });

    pub const K_NUM_CALLEE_SAVED: i32 = 10;

    pub const K_CALLEE_SAVED_FPU: DoubleRegList = DoubleRegList(RegListBase {
        registers: [
            Some(F24),
            Some(F25),
            Some(F26),
            Some(F27),
            Some(F28),
            Some(F29),
            Some(F30),
            Some(F31),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    });

    pub const K_NUM_CALLEE_SAVED_FPU: i32 = 8;

    pub const K_CALLER_SAVED_FPU: DoubleRegList = DoubleRegList(RegListBase {
        registers: [
            Some(F0),
            Some(F1),
            Some(F2),
            Some(F3),
            Some(F4),
            Some(F5),
            Some(F6),
            Some(F7),
            Some(F8),
            Some(F9),
            Some(F10),
            Some(F11),
            Some(F12),
            Some(F13),
            Some(F14),
            Some(F15),
            Some(F16),
            Some(F17),
            Some(F18),
            Some(F19),
            Some(F20),
            Some(F21),
            Some(F22),
            Some(F23),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    });
}
}

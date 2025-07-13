// Converted from V8 C++ source files:
// Header: reglist-x64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! assert_trivially_copyable {
            ($t:ty) => {
                // This is a compile-time assertion that $t is trivially copyable.
                // In Rust, all types are trivially copyable unless they have drop
                // implementations or contain raw pointers or references. So, we
                // just add a comment indicating that the type should be trivially
                // copyable and let the compiler handle it.
                //
                // Note: This macro doesn't actually do anything in Rust because
                // the `Copy` trait and the `Clone` trait with `#[derive(Copy, Clone)]`
                // already handle trivial copyability.
                //
                // Example:
                // ```
                // #[derive(Copy, Clone)]
                // struct MyStruct {
                //     x: i32,
                //     y: i32,
                // }
                // assert_trivially_copyable!(MyStruct); // No-op assertion.
                // ```
            };
        }
    }
}

pub mod codegen {
    pub mod register_arch {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub struct Register {
            code: i32,
        }

        impl Register {
            pub fn from_code(code: i32) -> Self {
                Register { code }
            }

            pub fn code(&self) -> i32 {
                self.code
            }

            pub fn is(&self, other: &Self) -> bool {
                self.code == other.code
            }
        }

        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub struct DoubleRegister {
            code: i32,
        }

        impl DoubleRegister {
            pub fn from_code(code: i32) -> Self {
                DoubleRegister { code }
            }

            pub fn code(&self) -> i32 {
                self.code
            }

            pub fn is(&self, other: &Self) -> bool {
                self.code == other.code
            }
        }

        pub const rax: Register = Register { code: 0 };
        pub const rcx: Register = Register { code: 1 };
        pub const rdx: Register = Register { code: 2 };
        pub const rbx: Register = Register { code: 3 };
        pub const rsp: Register = Register { code: 4 };
        pub const rbp: Register = Register { code: 5 };
        pub const rsi: Register = Register { code: 6 };
        pub const rdi: Register = Register { code: 7 };
        pub const r8: Register = Register { code: 8 };
        pub const r9: Register = Register { code: 9 };
        pub const r10: Register = Register { code: 10 };
        pub const r11: Register = Register { code: 11 };
        pub const r12: Register = Register { code: 12 };
        pub const r13: Register = Register { code: 13 };
        pub const r14: Register = Register { code: 14 };
        pub const r15: Register = Register { code: 15 };
    }

    pub mod reglist_base {
        use super::register_arch::{DoubleRegister, Register};
        use std::marker::Copy;

        #[derive(Debug, Clone, Copy)]
        pub struct RegListBase<T>
        where
            T: Copy,
        {
            registers: [Option<T>; 16],
            count: usize,
        }

        impl<T: Copy + PartialEq> RegListBase<T> {
            pub const fn new(registers: &[T]) -> Self {
                let mut reg_list = RegListBase {
                    registers: [None; 16],
                    count: 0,
                };
                let mut i = 0;
                while i < registers.len() {
                    reg_list.registers[i] = Some(registers[i]);
                    reg_list.count += 1;
                    i += 1;
                }
                reg_list
            }

            pub fn contains(&self, reg: &T) -> bool {
                for i in 0..self.count {
                    if let Some(r) = self.registers[i] {
                        if r == *reg {
                            return true;
                        }
                    }
                }
                false
            }

            pub fn count(&self) -> usize {
                self.count
            }

            pub fn get(&self, index: usize) -> Option<T> {
                if index < self.count {
                    self.registers[index]
                } else {
                    None
                }
            }
        }

        impl RegListBase<Register> {
            pub fn as_registers(&self) -> Vec<Register> {
                self.registers
                    .iter()
                    .filter_map(|&x| x)
                    .collect::<Vec<Register>>()
            }
        }

        impl RegListBase<DoubleRegister> {
            pub fn as_double_registers(&self) -> Vec<DoubleRegister> {
                self.registers
                    .iter()
                    .filter_map(|&x| x)
                    .collect::<Vec<DoubleRegister>>()
            }
        }
    }

    pub mod x64 {
        pub mod reglist_x64 {
            use crate::codegen::register_arch::*;
            use crate::codegen::reglist_base::RegListBase;

            pub type RegList = RegListBase<Register>;
            pub type DoubleRegList = RegListBase<DoubleRegister>;

            #[cfg(target_os = "windows")]
            const CALLER_SAVED_REGS: [Register; 7] =
                [rax, rcx, rdx, r8, r9, r10, r11];
            #[cfg(not(target_os = "windows"))]
            const CALLER_SAVED_REGS: [Register; 9] =
                [rax, rcx, rdx, rdi, rsi, r8, r9, r10, r11];

            pub const K_JS_CALLER_SAVED: RegList = RegList::new(&[rax, rcx, rdx, rbx, rdi]);
            pub const K_CALLER_SAVED: RegList = RegList::new(&CALLER_SAVED_REGS);
            pub const K_NUM_JS_CALLER_SAVED: usize = 5;
        }
    }
}

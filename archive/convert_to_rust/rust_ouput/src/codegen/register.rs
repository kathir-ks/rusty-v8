// Converted from V8 C++ source files:
// Header: register.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod codegen {
pub mod register {

pub use crate::codegen::register_arch::*;
pub use crate::codegen::reglist::*;

pub const fn add_argument_padding_slots(argument_count: i32) -> i32 {
    argument_count + argument_padding_slots(argument_count)
}

pub const fn should_pad_arguments(argument_count: i32) -> bool {
    argument_padding_slots(argument_count) != 0
}

pub trait RegisterTrait {}
impl RegisterTrait for Register {}
impl RegisterTrait for DoubleRegister {}
#[cfg(V8_TARGET_ARCH_X64)]
impl RegisterTrait for YMMRegister {}

pub fn are_aliased<T: RegisterTrait>(regs: &[T]) -> bool {
    if regs.is_empty() {
        return false;
    }

    let mut reg_list = RegListBase::<T>::new();
    for &reg in regs {
        reg_list.push(reg);
    }

    let num_different_regs = reg_list.count();
    let num_given_regs = regs.len();

    num_different_regs < num_given_regs
}

pub mod register_arch {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code: i16,
    }

    impl Register {
        pub fn valid(&self) -> bool {
            self.code != -1
        }

        pub fn is_valid(&self) -> bool {
            self.valid()
        }

        pub fn from_code(code: i16) -> Self {
            Register { code }
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister {
        code: i16,
    }

    impl DoubleRegister {
        pub fn valid(&self) -> bool {
            self.code != -1
        }

         pub fn is_valid(&self) -> bool {
            self.valid()
        }

        pub fn from_code(code: i16) -> Self {
            DoubleRegister { code }
        }
    }

    #[cfg(V8_TARGET_ARCH_X64)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct YMMRegister {
        code: i16,
    }

    #[cfg(V8_TARGET_ARCH_X64)]
    impl YMMRegister {
        pub fn valid(&self) -> bool {
            self.code != -1
        }

         pub fn is_valid(&self) -> bool {
            self.valid()
        }
        pub fn from_code(code: i16) -> Self {
            YMMRegister { code }
        }
    }

    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        0 // Default implementation
    }
}

pub mod reglist {
    use super::register_arch::Register;
    use super::register_arch::DoubleRegister;
    #[cfg(V8_TARGET_ARCH_X64)]
    use super::register_arch::YMMRegister;
    use std::marker::PhantomData;

    pub struct RegListBase<T> {
        regs: Vec<T>,
    }

    impl<T: Copy + PartialEq> RegListBase<T> {
        pub fn new() -> Self {
            RegListBase {
                regs: Vec::new(),
            }
        }

        pub fn push(&mut self, reg: T) {
           if !self.regs.contains(&reg) {
            self.regs.push(reg);
           }
        }

        pub fn count(&self) -> usize {
            self.regs.len()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.regs.iter()
        }

    }

    impl RegListBase<Register> {
        pub fn with_registers(registers: &[Register]) -> Self {
            let mut reg_list = RegListBase::new();
            for &reg in registers {
                reg_list.push(reg);
            }
            reg_list
        }
    }

    impl RegListBase<DoubleRegister> {
         pub fn with_registers(registers: &[DoubleRegister]) -> Self {
            let mut reg_list = RegListBase::new();
            for &reg in registers {
                reg_list.push(reg);
            }
            reg_list
        }
    }

    #[cfg(V8_TARGET_ARCH_X64)]
    impl RegListBase<YMMRegister> {
         pub fn with_registers(registers: &[YMMRegister]) -> Self {
            let mut reg_list = RegListBase::new();
            for &reg in registers {
                reg_list.push(reg);
            }
            reg_list
        }
    }
}

}  // namespace register
}  // namespace codegen

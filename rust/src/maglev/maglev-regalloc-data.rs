// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_regalloc_data {
    use std::marker::PhantomData;
    use std::mem::transmute;
    use std::ops::BitOr;

    use crate::codegen::register::{DoubleRegList, RegList, Register, DoubleRegister};
    use crate::compiler::backend::instruction::InstructionOperand;
    use crate::maglev::maglev_assembler::MaglevAssembler;

    pub trait ValueNode {}

    pub const K_ALLOCATABLE_GENERAL_REGISTER_COUNT: usize =
        MaglevAssembler::get_allocatable_registers().count();
    pub const K_ALLOCATABLE_DOUBLE_REGISTER_COUNT: usize =
        MaglevAssembler::get_allocatable_double_registers().count();

    pub struct AllocatableRegisters<T> {
        _phantom: PhantomData<T>,
    }

    impl AllocatableRegisters<Register> {
        pub const K_REGISTERS: RegList = MaglevAssembler::get_allocatable_registers();
    }

    impl AllocatableRegisters<DoubleRegister> {
        pub const K_REGISTERS: DoubleRegList = MaglevAssembler::get_allocatable_double_registers();
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RegisterStateFlags {
        pub is_initialized: bool,
        pub is_merge: bool,
    }

    impl RegisterStateFlags {
        pub const K_IS_MERGE_SHIFT: usize = 0;
        pub const K_IS_INITIALIZED_SHIFT: usize = 1;

        pub const fn new(is_initialized: bool, is_merge: bool) -> Self {
            RegisterStateFlags {
                is_initialized,
                is_merge,
            }
        }
    }

    impl From<RegisterStateFlags> for usize {
        fn from(flags: RegisterStateFlags) -> Self {
            (if flags.is_initialized {
                1 << RegisterStateFlags::K_IS_INITIALIZED_SHIFT
            } else {
                0
            }) | (if flags.is_merge {
                1 << RegisterStateFlags::K_IS_MERGE_SHIFT
            } else {
                0
            })
        }
    }

    impl From<usize> for RegisterStateFlags {
        fn from(state: usize) -> Self {
            RegisterStateFlags {
                is_initialized: (state & (1 << RegisterStateFlags::K_IS_INITIALIZED_SHIFT)) != 0,
                is_merge: (state & (1 << RegisterStateFlags::K_IS_MERGE_SHIFT)) != 0,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct RegisterState<T> {
        pointer: *mut T,
        payload: RegisterStateFlags,
    }

    impl<T> RegisterState<T> {
        pub fn new(pointer: *mut T, payload: RegisterStateFlags) -> Self {
            RegisterState { pointer, payload }
        }

        pub fn get_pointer(&self) -> *mut T {
            self.pointer
        }

        pub fn get_payload(&self) -> RegisterStateFlags {
            self.payload
        }
    }
    

    pub struct RegisterMerge {
        pub node: *mut dyn ValueNode, // Need to define ValueNode as a trait
        operands_ptr: *mut InstructionOperand,
        operand_count: usize
    }

    impl RegisterMerge {
        pub fn operands(&mut self) -> &mut [InstructionOperand] {
            unsafe { std::slice::from_raw_parts_mut(self.operands_ptr, self.operand_count) }
        }
    }

    pub fn load_merge_state<T: ValueNode>(
        state: RegisterState<T>,
        merge: &mut Option<*mut RegisterMerge>,
    ) -> bool {
        debug_assert!(state.payload.is_initialized);
        if state.payload.is_merge {
            *merge = Some(state.pointer as *mut RegisterMerge);
            true
        } else {
            *merge = None;
            false
        }
    }

    pub fn load_merge_state_with_node<T: ValueNode>(
        state: RegisterState<T>,
        node: &mut *mut dyn ValueNode,
        merge: &mut Option<*mut RegisterMerge>,
    ) -> bool {
        debug_assert!(state.payload.is_initialized);
        if load_merge_state(state, merge) {
            unsafe {
                *node = (*merge.unwrap()).node;
            }
            true
        } else {
            *node = state.pointer as *mut dyn ValueNode;
            false
        }
    }
}

pub mod codegen {
    pub mod register {
        use std::ops::BitOr;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Register(u32);

        impl Register {
            pub const fn from_code(code: u32) -> Self {
                Register(code)
            }

            pub fn code(&self) -> u32 {
                self.0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct DoubleRegister(u32);

        impl DoubleRegister {
            pub const fn from_code(code: u32) -> Self {
                DoubleRegister(code)
            }

            pub fn code(&self) -> u32 {
                self.0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct RegList(u64);

        impl RegList {
            pub const EMPTY: RegList = RegList(0);

            pub const fn from_code(code: u64) -> Self {
                RegList(code)
            }

            pub fn has(&self, reg: Register) -> bool {
                (self.0 & (1 << reg.code())) != 0
            }

            pub fn count(&self) -> usize {
                self.0.count_ones() as usize
            }
        }

        impl BitOr for RegList {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                RegList(self.0 | other.0)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct DoubleRegList(u64);

        impl DoubleRegList {
            pub const EMPTY: DoubleRegList = DoubleRegList(0);

            pub const fn from_code(code: u64) -> Self {
                DoubleRegList(code)
            }

            pub fn has(&self, reg: DoubleRegister) -> bool {
                (self.0 & (1 << reg.code())) != 0
            }

            pub fn count(&self) -> usize {
                self.0.count_ones() as usize
            }
        }

        impl BitOr for DoubleRegList {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                DoubleRegList(self.0 | other.0)
            }
        }
    }
}

pub mod compiler {
    pub mod backend {
        pub mod instruction {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct InstructionOperand {}
        }
    }
}

pub mod maglev {
    pub mod maglev_assembler {
        use crate::codegen::register::{DoubleRegList, RegList, Register, DoubleRegister};

        pub struct MaglevAssembler {}

        impl MaglevAssembler {
            pub const fn get_allocatable_registers() -> RegList {
                // Dummy implementation
                RegList::from_code(0x0F)
            }

            pub const fn get_allocatable_double_registers() -> DoubleRegList {
                // Dummy implementation
                DoubleRegList::from_code(0x0F)
            }
        }
    }
}
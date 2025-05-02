// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod interpreter {
    use std::cmp;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Register(pub i32);

    impl Register {
        pub fn index(&self) -> i32 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RegisterList {
        base_index: i32,
        count: i32,
    }

    impl RegisterList {
        pub fn new(base_index: i32, count: i32) -> Self {
            RegisterList {
                base_index,
                count,
            }
        }

        pub fn base_index(&self) -> i32 {
            self.base_index
        }

        pub fn count(&self) -> i32 {
            self.count
        }

        pub fn increment_register_count(&mut self) {
            self.count += 1;
        }

        pub fn last_register(&self) -> Register {
            Register(self.base_index + self.count - 1)
        }
    }

    pub trait Observer {
        fn register_allocate_event(&mut self, reg: Register);
        fn register_list_allocate_event(&mut self, reg_list: RegisterList);
        fn register_list_free_event(&mut self, reg_list: RegisterList);
        fn register_free_event(&mut self, reg: Register);
    }

    /// A class that allows the allocation of contiguous temporary registers.
    pub struct BytecodeRegisterAllocator<'a> {
        next_register_index_: i32,
        max_register_count_: i32,
        observer_: Option<&'a mut dyn Observer>,
    }

    impl<'a> BytecodeRegisterAllocator<'a> {
        pub fn new(start_index: i32) -> Self {
            BytecodeRegisterAllocator {
                next_register_index_: start_index,
                max_register_count_: start_index,
                observer_: None,
            }
        }

        /// Returns a new register.
        pub fn new_register(&mut self) -> Register {
            let reg = Register(self.next_register_index_);
            self.next_register_index_ += 1;
            self.max_register_count_ = cmp::max(self.next_register_index_, self.max_register_count_);
            if let Some(observer) = &mut self.observer_ {
                observer.register_allocate_event(reg);
            }
            reg
        }

        /// Returns a consecutive list of |count| new registers.
        pub fn new_register_list(&mut self, count: i32) -> RegisterList {
            let reg_list = RegisterList::new(self.next_register_index_, count);
            self.next_register_index_ += count;
            self.max_register_count_ = cmp::max(self.next_register_index_, self.max_register_count_);
            if let Some(observer) = &mut self.observer_ {
                observer.register_list_allocate_event(reg_list);
            }
            reg_list
        }

        /// Returns a growable register list.
        pub fn new_growable_register_list(&mut self) -> RegisterList {
            RegisterList::new(self.next_register_index_, 0)
        }

        /// Appends a new register to |reg_list| increasing it's count by one and
        /// returning the register added.
        ///
        /// Note: no other new registers must be currently allocated since the register
        /// list was originally allocated.
        pub fn grow_register_list(&mut self, reg_list: &mut RegisterList) -> Register {
            let reg = self.new_register();
            reg_list.increment_register_count();
            // If the following assert fails then a register was allocated (and not
            // freed) between the creation of the RegisterList and this call to add a
            // Register.
            debug_assert_eq!(reg.index(), reg_list.last_register().index());
            reg
        }

        /// Release all registers above |register_index|.
        pub fn release_registers(&mut self, register_index: i32) {
            let count = self.next_register_index_ - register_index;
            self.next_register_index_ = register_index;
            if let Some(observer) = &mut self.observer_ {
                observer.register_list_free_event(RegisterList::new(register_index, count));
            }
        }

        /// Release last allocated register
        pub fn release_register(&mut self, reg: Register) {
            debug_assert_eq!(self.next_register_index_ - 1, reg.index());
            if let Some(observer) = &mut self.observer_ {
                observer.register_free_event(reg);
            }
            self.next_register_index_ -= 1;
        }

        /// Returns true if the register |reg| is a live register.
        pub fn register_is_live(&self, reg: Register) -> bool {
            reg.index() < self.next_register_index_
        }

        /// Returns a register list for all currently live registers.
        pub fn all_live_registers(&self) -> RegisterList {
            RegisterList::new(0, self.next_register_index())
        }

        pub fn set_observer(&mut self, observer: &'a mut dyn Observer) {
            self.observer_ = Some(observer);
        }

        pub fn next_register_index(&self) -> i32 {
            self.next_register_index_
        }

        pub fn maximum_register_count(&self) -> i32 {
            self.max_register_count_
        }
    }
}
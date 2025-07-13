// Converted from V8 C++ source files:
// Header: bytecode-register-allocator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct BytecodeRegisterAllocator {
    next_register_index_: i32,
    max_register_count_: i32,
    observer_: Option<Box<dyn BytecodeRegisterAllocatorObserver>>,
}

pub trait BytecodeRegisterAllocatorObserver {
    fn register_allocate_event(&mut self, reg: Register);
    fn register_list_allocate_event(&mut self, reg_list: RegisterList);
    fn register_list_free_event(&mut self, reg_list: RegisterList);
    fn register_free_event(&mut self, reg: Register);
}

impl BytecodeRegisterAllocator {
    pub fn new(start_index: i32) -> Self {
        BytecodeRegisterAllocator {
            next_register_index_: start_index,
            max_register_count_: start_index,
            observer_: None,
        }
    }

    pub fn new_register(&mut self) -> Register {
        let reg = Register::new(self.next_register_index_);
        self.next_register_index_ += 1;
        self.max_register_count_ = std::cmp::max(self.next_register_index_, self.max_register_count_);
        if let Some(observer) = &mut self.observer_ {
            observer.register_allocate_event(reg);
        }
        reg
    }

    pub fn new_register_list(&mut self, count: i32) -> RegisterList {
        let reg_list = RegisterList::new(self.next_register_index_, count);
        self.next_register_index_ += count;
        self.max_register_count_ = std::cmp::max(self.next_register_index_, self.max_register_count_);
        if let Some(observer) = &mut self.observer_ {
            observer.register_list_allocate_event(reg_list);
        }
        reg_list
    }

    pub fn new_growable_register_list(&mut self) -> RegisterList {
        RegisterList::new(self.next_register_index_, 0)
    }

    pub fn grow_register_list(&mut self, reg_list: &mut RegisterList) -> Register {
        let reg = self.new_register();
        reg_list.increment_register_count();
        assert_eq!(reg.index(), reg_list.last_register().index());
        reg
    }

    pub fn release_registers(&mut self, register_index: i32) {
        let count = self.next_register_index_ - register_index;
        self.next_register_index_ = register_index;
        if let Some(observer) = &mut self.observer_ {
            observer.register_list_free_event(RegisterList::new(register_index, count));
        }
    }

    pub fn release_register(&mut self, reg: Register) {
        assert_eq!(self.next_register_index_ - 1, reg.index());
        if let Some(observer) = &mut self.observer_ {
            observer.register_free_event(reg);
        }
        self.next_register_index_ -= 1;
    }

    pub fn register_is_live(&self, reg: Register) -> bool {
        reg.index() < self.next_register_index_
    }

    pub fn all_live_registers(&self) -> RegisterList {
        RegisterList::new(0, self.next_register_index_)
    }

    pub fn set_observer(&mut self, observer: Option<Box<dyn BytecodeRegisterAllocatorObserver>>) {
        self.observer_ = observer;
    }

    pub fn next_register_index(&self) -> i32 {
        self.next_register_index_
    }

    pub fn maximum_register_count(&self) -> i32 {
        self.max_register_count_
    }
}

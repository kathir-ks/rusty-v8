// Converted from V8 C++ source files:
// Header: test-interface.h
// Implementation: test-interface.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_inspector {
    pub struct V8Inspector {
        // Opaque type, needs fields to satisfy the compiler
        _private: i32,
    }

    impl V8Inspector {
        pub fn new() -> Self {
            V8Inspector { _private: 0 }
        }
    }

    pub struct V8Debugger {
        max_async_task_stacks: i32,
        async_stacks_state: String,
    }

    impl V8Debugger {
        pub fn new() -> Self {
            V8Debugger {
                max_async_task_stacks: 0,
                async_stacks_state: String::new(),
            }
        }

        pub fn set_max_async_task_stacks_for_test(&mut self, limit: i32) {
            self.max_async_task_stacks = limit;
        }

        pub fn dump_async_task_stacks_state_for_test(&mut self) {
            self.async_stacks_state = format!("Max stacks: {}", self.max_async_task_stacks);
            println!("{}", self.async_stacks_state);
        }
    }

    pub struct V8InspectorImpl {
        debugger: V8Debugger,
    }

    impl V8InspectorImpl {
        pub fn new() -> Self {
            V8InspectorImpl {
                debugger: V8Debugger::new(),
            }
        }

        pub fn debugger(&mut self) -> &mut V8Debugger {
            &mut self.debugger
        }
    }

    pub fn set_max_async_task_stacks_for_test(inspector: &mut V8Inspector, limit: i32) {
        let inspector_impl: *mut V8InspectorImpl = unsafe { std::mem::transmute(inspector) };
        unsafe {
            (*inspector_impl).debugger().set_max_async_task_stacks_for_test(limit);
        }
    }

    pub fn dump_async_task_stacks_state_for_test(inspector: &mut V8Inspector) {
        let inspector_impl: *mut V8InspectorImpl = unsafe { std::mem::transmute(inspector) };
        unsafe {
            (*inspector_impl).debugger().dump_async_task_stacks_state_for_test();
        }
    }
}

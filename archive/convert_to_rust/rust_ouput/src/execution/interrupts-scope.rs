// Converted from V8 C++ source files:
// Header: interrupts-scope.h
// Implementation: interrupts-scope.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interrupts_scope {
    use std::cell::Cell;

    pub struct Isolate;

    pub struct StackGuard {
        interrupts_scope_list_head: Cell<Option<Box<InterruptsScope>>>,
    }

    impl StackGuard {
        pub fn new() -> StackGuard {
            StackGuard {
                interrupts_scope_list_head: Cell::new(None),
            }
        }

        pub const ALL_INTERRUPTS: u32 = 0xFFFFFFFF; // Assuming all 32 bits represent interrupts

        pub fn PushInterruptsScope(&self, scope: &mut InterruptsScope) {
            let mut scope_box = Box::new(InterruptsScope::from_scope(scope));
            let mut head = self.interrupts_scope_list_head.take();

            scope_box.prev_ = head;
            self.interrupts_scope_list_head.set(Some(scope_box));

        }

        pub fn PopInterruptsScope(&self) {
            let mut head = self.interrupts_scope_list_head.take();
            match head {
                Some(mut head_box) => {
                    self.interrupts_scope_list_head.set(head_box.prev_.take());
                }
                None => {}
            }
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum Mode {
        kPostponeInterrupts,
        kRunInterrupts,
        kNoop,
    }

    pub struct InterruptsScope {
        stack_guard_: *mut StackGuard,
        prev_: Option<Box<InterruptsScope>>,
        intercept_mask_: u32,
        intercepted_flags_: u32,
        mode_: Mode,
    }

    impl InterruptsScope {
        pub fn new(isolate: &mut Isolate, intercept_mask: u32, mode: Mode) -> InterruptsScope {
            let stack_guard = Isolate::stack_guard(isolate);

            let mut scope = InterruptsScope {
                stack_guard_: stack_guard,
                prev_: None,
                intercept_mask_: intercept_mask,
                intercepted_flags_: 0,
                mode_: mode,
            };

            if mode != Mode::kNoop {
                unsafe {
                    (&mut *stack_guard).PushInterruptsScope(&mut scope);
                }
            }

            scope
        }

        // Helper function to create an InterruptsScope from an existing scope
        fn from_scope(scope: &mut InterruptsScope) -> InterruptsScope {
            InterruptsScope {
                stack_guard_: scope.stack_guard_,
                prev_: scope.prev_.take(),
                intercept_mask_: scope.intercept_mask_,
                intercepted_flags_: scope.intercepted_flags_,
                mode_: scope.mode_,
            }
        }

        pub fn Intercept(&mut self, flag: u32) -> bool {
            let mut last_postpone_scope: Option<&mut InterruptsScope> = None;
            let mut current: *mut InterruptsScope = self;

            unsafe {
                while !current.is_null() {
                    let current_ref = &mut *current;

                    if (current_ref.intercept_mask_ & flag) == 0 {
                        current = current_ref.prev_.as_mut().map(|x| &mut **x as *mut InterruptsScope).unwrap_or(std::ptr::null_mut());
                        continue;
                    }

                    if current_ref.mode_ == Mode::kRunInterrupts {
                        break;
                    } else {
                        assert_eq!(current_ref.mode_, Mode::kPostponeInterrupts);
                        last_postpone_scope = Some(current_ref);
                    }

                    current = current_ref.prev_.as_mut().map(|x| &mut **x as *mut InterruptsScope).unwrap_or(std::ptr::null_mut());
                }
            }

            match last_postpone_scope {
                Some(scope) => {
                    scope.intercepted_flags_ |= flag;
                    true
                }
                None => false,
            }
        }
    }

    impl Drop for InterruptsScope {
        fn drop(&mut self) {
            if self.mode_ != Mode::kNoop {
                unsafe {
                    (&mut *self.stack_guard_).PopInterruptsScope();
                }
            }
        }
    }

    pub struct PostponeInterruptsScope {
        interrupts_scope: InterruptsScope,
    }

    impl PostponeInterruptsScope {
        pub fn new(isolate: &mut Isolate, intercept_mask: u32) -> PostponeInterruptsScope {
            PostponeInterruptsScope {
                interrupts_scope: InterruptsScope::new(isolate, intercept_mask, Mode::kPostponeInterrupts),
            }
        }
    }

    pub struct SafeForInterruptsScope {
        interrupts_scope: InterruptsScope,
    }

    impl SafeForInterruptsScope {
        pub fn new(isolate: &mut Isolate, intercept_mask: u32) -> SafeForInterruptsScope {
            SafeForInterruptsScope {
                interrupts_scope: InterruptsScope::new(isolate, intercept_mask, Mode::kRunInterrupts),
            }
        }
    }

    impl Isolate {
        fn stack_guard(&mut self) -> *mut StackGuard {
            // Assuming Isolate has a StackGuard field
            // Replace with the actual way to access the StackGuard
            unsafe {
                let isolate_ptr: *mut Isolate = self;
                let stack_guard_ptr: *mut StackGuard = isolate_ptr.cast::<StackGuard>();
                stack_guard_ptr
            }
        }
    }
}

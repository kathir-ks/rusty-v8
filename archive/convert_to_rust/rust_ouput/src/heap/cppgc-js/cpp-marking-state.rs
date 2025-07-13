// Converted from V8 C++ source files:
// Header: cpp-marking-state.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cpp_marking_state {
    use std::ptr::null_mut;
    use std::mem::MaybeUninit;
    use std::ops::Deref;

    use crate::heap::cppgc_js::cpp_heap::GCTracer;
    use crate::heap::CppMarkingState;
    use crate::objects::JSObject;
    //use crate::objects::EmbedderDataSlot;
    use crate::heap::marking_worklist::CppMarkingState as InternalCppMarkingState;
    pub struct EmbedderDataSlot {}

    pub struct CppMarkingState {
        owned_marking_state_: Option<Box<cppgc::internal::MarkingStateBase>>,
        marking_state_: *mut cppgc::internal::MarkingStateBase,
    }

    impl CppMarkingState {
        pub fn new(main_thread_marking_state: &mut cppgc::internal::MarkingStateBase) -> Self {
            CppMarkingState {
                owned_marking_state_: None,
                marking_state_: main_thread_marking_state,
            }
        }

        pub fn new_concurrent(concurrent_marking_state: Box<cppgc::internal::MarkingStateBase>) -> Self {
            CppMarkingState {
                owned_marking_state_: Some(concurrent_marking_state),
                marking_state_: unsafe { &mut *Box::into_raw(concurrent_marking_state) },
            }
        }

        pub fn publish(&mut self) {
            unsafe { (*self.marking_state_).Publish(); }
        }

        pub fn mark_and_push(&mut self, instance: *mut std::ffi::c_void) {
            unsafe { (*self.marking_state_).marking_worklist().push(instance); }
        }

        pub fn is_local_empty(&self) -> bool {
            unsafe { (*self.marking_state_).marking_worklist().IsLocalEmpty() }
        }
    }

    mod cppgc {
        pub mod internal {
            use std::ptr::null_mut;
            use std::mem::MaybeUninit;
            use std::ops::Deref;

            pub struct MarkingWorklist {
                local_queue: Vec<*mut std::ffi::c_void>,
            }

            impl MarkingWorklist {
                pub fn new() -> Self {
                    MarkingWorklist {
                        local_queue: Vec::new(),
                    }
                }
                pub fn push(&mut self, instance: *mut std::ffi::c_void) {
                    self.local_queue.push(instance);
                }

                pub fn IsLocalEmpty(&self) -> bool {
                    self.local_queue.is_empty()
                }
            }

            pub struct MarkingStateBase {
                worklist : MarkingWorklist,
            }

            impl MarkingStateBase {
                pub fn new() -> Self {
                    MarkingStateBase{
                        worklist: MarkingWorklist::new(),
                    }
                }
                pub fn Publish(&mut self) {}
                pub fn marking_worklist(&mut self) -> &mut MarkingWorklist {
                    &mut self.worklist
                }
            }
        }
    }
}

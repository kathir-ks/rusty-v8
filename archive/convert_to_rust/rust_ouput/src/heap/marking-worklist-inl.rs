// Converted from V8 C++ source files:
// Header: marking-worklist-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::collections::HashMap;

use crate::heap::MarkingWorklist;
use crate::objects::HeapObject;
use crate::objects::Tagged;
use crate::heap::cppgc_js::CppMarkingState;

pub struct MarkingWorklists {
    shared_: MarkingWorklist,
    on_hold_: MarkingWorklist,
    other_: MarkingWorklist,
    context_worklists_: Vec<ContextWorklistEntry>,
}

struct ContextWorklistEntry {
    worklist: Box<MarkingWorklist>,
}

impl MarkingWorklists {
    pub fn new() -> Self {
        MarkingWorklists {
            shared_: MarkingWorklist::new(),
            on_hold_: MarkingWorklist::new(),
            other_: MarkingWorklist::new(),
            context_worklists_: Vec::new(),
        }
    }

    pub fn update<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut HeapObject),
    {
        self.shared_.update(&mut callback);
        self.on_hold_.update(&mut callback);
        self.other_.update(&mut callback);
        for cw in &mut self.context_worklists_ {
            cw.worklist.update(&mut callback);
        }
    }

    pub struct Local<'a> {
        active_: *mut MarkingWorklist,
        active_context_: Address,
        on_hold_: MarkingWorklist,
        is_per_context_mode_: bool,
        cpp_marking_state_: Option<&'a mut CppMarkingState>,
        marking_worklists: &'a mut MarkingWorklists,
    }

    impl<'a> Local<'a> {
        pub fn new(marking_worklists: &'a mut MarkingWorklists) -> Self {
             let active = &mut marking_worklists.shared_ as *mut MarkingWorklist;
            Local {
                active_: active,
                active_context_: Address{},
                on_hold_: MarkingWorklist::new(),
                is_per_context_mode_: false, // Assuming default value
                cpp_marking_state_: None,
                marking_worklists,
            }
        }

        pub fn push(&mut self, object: Tagged<HeapObject>) {
            unsafe {
                (*self.active_).push(object);
            }
        }

        pub fn pop(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            unsafe {
                if (*self.active_).pop(object) {
                    return true;
                }
            }
            if !self.is_per_context_mode_ {
                return false;
            }
            // The active worklist is empty. Find any other non-empty worklist and
            // switch the active worklist to it.
            self.pop_context(object)
        }

        fn pop_context(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            // Iterate through all context worklists to find a non-empty one
            for cw in &mut self.marking_worklists.context_worklists_ {
                if !cw.worklist.is_empty() {
                    unsafe {
                        self.active_ = cw.worklist.as_mut() as *mut MarkingWorklist;
                    }
                    if unsafe { (*self.active_).pop(object) } {
                        return true;
                    }
                    // Worklist was empty after all.  Continue searching.
                }
            }
            false
        }

        pub fn push_on_hold(&mut self, object: Tagged<HeapObject>) {
            self.on_hold_.push(object);
        }

        pub fn pop_on_hold(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            self.on_hold_.pop(object)
        }

        pub fn switch_to_context(&mut self, context: Address) -> Address {
            if context == self.active_context_ {
                return context;
            }
            self.switch_to_context_slow(context)
        }

        fn switch_to_context_slow(&mut self, context: Address) -> Address {
           // Find the corresponding worklist for the given context.
            let mut found = false;
            for cw in &mut self.marking_worklists.context_worklists_ {
                // Assuming Address can be compared directly for equality
                // If not, adapt this logic as necessary.
                unsafe {
                     if (*self.active_).is_empty(){
                        self.active_ = cw.worklist.as_mut() as *mut MarkingWorklist;
                        self.active_context_ = context;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Create a new context worklist if one doesn't exist.
                let mut new_worklist = MarkingWorklist::new();
                unsafe {
                     self.active_ = &mut new_worklist as *mut MarkingWorklist;
                }
                self.active_context_ = context;

                self.marking_worklists.context_worklists_.push(ContextWorklistEntry {
                    worklist: Box::new(new_worklist),
                });
            }

             self.active_context_
        }

        fn switch_to_context_impl(&mut self, context: Address, worklist: &mut MarkingWorklist) {
            self.active_ = worklist as *mut MarkingWorklist;
            self.active_context_ = context;
        }

        pub fn publish_cpp_heap_objects(&mut self) {
            if let Some(cpp_marking_state) = &mut self.cpp_marking_state_ {
                cpp_marking_state.publish();
            }
        }
    }
}

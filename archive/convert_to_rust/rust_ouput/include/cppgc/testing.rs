// Converted from V8 C++ source files:
// Header: testing.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod testing {
    use crate::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct OverrideEmbedderStackStateScope<'a> {
        heap_handle: &'a HeapHandle,
        original_state: EmbedderStackState,
    }

    impl<'a> OverrideEmbedderStackStateScope<'a> {
        pub fn new(heap_handle: &'a HeapHandle, state: EmbedderStackState) -> Self {
            let original_state = heap_handle.override_stack_state.borrow().clone();
            *heap_handle.override_stack_state.borrow_mut() = state;
            OverrideEmbedderStackStateScope {
                heap_handle,
                original_state,
            }
        }
    }

    impl<'a> Drop for OverrideEmbedderStackStateScope<'a> {
        fn drop(&mut self) {
            *self.heap_handle.override_stack_state.borrow_mut() = self.original_state.clone();
        }
    }

    pub struct StandaloneTestingHeap<'a> {
        heap_handle: &'a HeapHandle,
    }

    impl<'a> StandaloneTestingHeap<'a> {
        pub fn new(heap_handle: &'a HeapHandle) -> Self {
            StandaloneTestingHeap { heap_handle }
        }

        pub fn start_garbage_collection(&self) {
            self.heap_handle.start_garbage_collection();
        }

        pub fn perform_marking_step(&self, stack_state: EmbedderStackState) -> bool {
            self.heap_handle.perform_marking_step(stack_state)
        }

        pub fn finalize_garbage_collection(&self, stack_state: EmbedderStackState) {
            self.heap_handle.finalize_garbage_collection(stack_state);
        }

        pub fn toggle_main_thread_marking(&self, should_mark: bool) {
            *self.heap_handle.main_thread_marking.borrow_mut() = should_mark;
        }

        pub fn force_compaction_for_next_garbage_collection(&self) {
            *self.heap_handle.force_compaction.borrow_mut() = true;
        }
    }

    pub fn is_heap_object_old(_ptr: *mut std::ffi::c_void) -> bool {
        true
    }
}

#[derive(Clone, Copy)]
pub enum EmbedderStackState {
    Safe,
    MayContainGarbageCollectedPointers,
}

pub struct HeapHandle {
    override_stack_state: Rc<RefCell<EmbedderStackState>>,
    main_thread_marking: Rc<RefCell<bool>>,
    force_compaction: Rc<RefCell<bool>>,
}

impl HeapHandle {
    pub fn new() -> Self {
        HeapHandle {
            override_stack_state: Rc::new(RefCell::new(EmbedderStackState::Safe)),
            main_thread_marking: Rc::new(RefCell::new(true)),
            force_compaction: Rc::new(RefCell::new(false)),
        }
    }

    fn start_garbage_collection(&self) {
        println!("Garbage collection started");
    }

    fn perform_marking_step(&self, _stack_state: EmbedderStackState) -> bool {
        println!("Performing marking step");
        true
    }

    fn finalize_garbage_collection(&self, _stack_state: EmbedderStackState) {
        println!("Finalizing garbage collection");
    }
}

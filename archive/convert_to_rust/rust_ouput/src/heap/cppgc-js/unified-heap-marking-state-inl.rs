// Converted from V8 C++ source files:
// Header: unified-heap-marking-state-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod unified_heap_marking_state_inl {
    use crate::heap::cppgc_js::unified_heap_marking_state::UnifiedHeapMarkingState;
    use crate::heap::heap::Heap;
    use crate::heap::marking_inl::MarkingHelper;
    use crate::heap::marking_state_inl::AtomicMarkingState;
    use crate::heap::marking_worklist_inl::LocalMarkingWorklist;
    use crate::objects::objects_inl::HeapObject;
    use crate::objects::objects_inl::Object;
    use crate::handles::traced_handles::TracedHandles;
    use crate::handles::traced_handles::TracedReferenceBase;
    use std::ptr::null_mut;

    pub struct BasicTracedReferenceExtractor {}

    impl BasicTracedReferenceExtractor {
        pub fn get_object_slot_for_marking(ref_: &TracedReferenceBase) -> *mut usize {
            ref_.get_slot_thread_safe() as *mut usize
        }
    }

    impl UnifiedHeapMarkingState {
        pub fn mark_and_push(&self, reference: &TracedReferenceBase) {
            let traced_handle_location =
                BasicTracedReferenceExtractor::get_object_slot_for_marking(reference);

            if traced_handle_location.is_null() {
                return;
            }

            let object = TracedHandles::mark(
                traced_handle_location,
                self.mark_mode_.clone(),
            );

            if !object.is_heap_object() {
                return;
            }

            let heap_object = object.unchecked_cast::<HeapObject>();

            let worklist_target = MarkingHelper::should_mark_object(self.heap_.unwrap(), heap_object);

            if let Some(target) = worklist_target {
                MarkingHelper::try_mark_and_push(
                    self.heap_.unwrap(),
                    &self.local_marking_worklist_,
                    &self.marking_state_,
                    target,
                    heap_object,
                );
            }
        }
    }
}

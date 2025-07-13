// Converted from V8 C++ source files:
// Header: cpp-marking-state-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

mod cpp_marking_state {
    use crate::heap::cppgc::cpp_marking_state::CppMarkingState;
    use crate::heap::cppgc::internal::heap_object_header::HeapObjectHeader;

    impl CppMarkingState {
        pub fn mark_and_push(&mut self, instance: *mut std::ffi::c_void) {
            let header = HeapObjectHeader::from_object(instance);
            self.marking_state_.mark_and_push(header);
        }
    }
}

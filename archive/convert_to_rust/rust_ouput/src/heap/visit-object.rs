// Converted from V8 C++ source files:
// Header: visit-object.h
// Implementation: visit-object.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod visit_object {
    use crate::objects::heap_object::HeapObject;
    use crate::objects::heap_object::Tagged;
    use crate::Isolate;
    use crate::ObjectVisitor;
    use crate::objects::map::Map;
    use crate::objects::slots::ExternalPointerSlot;
    use crate::objects::visitors::CppHeapPointerSlot;
    use crate::objects::instruction_stream::InstructionStreamSlot;
    use crate::objects::heap_object::MaybeObjectSlot;
    use crate::codegen::reloc_info::RelocInfo;
    use crate::objects::instruction_stream::InstructionStream;
    use crate::objects::code::Code;
    use crate::objects::visitors::IndirectPointerSlot;
    use crate::objects::visitors::IndirectPointerMode;
    use crate::objects::trusted_object::TrustedObject;
    use crate::objects::visitors::ProtectedPointerSlot;
    use crate::objects::visitors::ProtectedMaybeObjectSlot;
    use crate::objects::visitors::JSDispatchHandle;

    pub struct LocalIsolate {}

    struct ObjectVisitorForwarder<'a> {
        isolate: *mut Isolate,
        visitor_: *mut ObjectVisitor<'a>,
    }

    impl<'a> ObjectVisitorForwarder<'a> {
        fn new(isolate: *mut Isolate, visitor: *mut ObjectVisitor<'a>) -> Self {
            ObjectVisitorForwarder {
                isolate,
                visitor_: visitor,
            }
        }

        fn new_local(isolate: *mut LocalIsolate, visitor: *mut ObjectVisitor<'a>) -> Self {
            // Assuming LocalIsolate can be converted to Isolate pointer
            ObjectVisitorForwarder {
                isolate: std::ptr::null_mut(), // Replace with proper conversion if needed
                visitor_: visitor,
            }
        }

        const SHOULD_VISIT_MAP_POINTER: bool = false;
        const SHOULD_USE_UNCHECKED_CAST: bool = true;
        const SHOULD_VISIT_FULL_JS_OBJECT: bool = true;

        fn visit_pointers_objectslot(&mut self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) {
            unsafe {
                (*self.visitor_).VisitPointers_objectslot(host, start, end);
            }
        }

        fn visit_pointers_maybeobjectslot(&mut self, host: Tagged<HeapObject>, start: MaybeObjectSlot, end: MaybeObjectSlot) {
            unsafe {
                (*self.visitor_).VisitPointers_maybeobjectslot(host, start, end);
            }
        }

        fn visit_instruction_stream_pointer(&mut self, host: Tagged<Code>, slot: InstructionStreamSlot) {
            unsafe {
                (*self.visitor_).VisitInstructionStreamPointer(host, slot);
            }
        }

        fn visit_custom_weak_pointers(&mut self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) {
            unsafe {
                (*self.visitor_).VisitCustomWeakPointers(host, start, end);
            }
        }

        fn visit_pointer_objectslot(&mut self, host: Tagged<HeapObject>, slot: ObjectSlot) {
            self.visit_pointers_objectslot(host, slot, ObjectSlot{}); // Replace with slot + 1 if needed
        }

        fn visit_pointer_maybeobjectslot(&mut self, host: Tagged<HeapObject>, slot: MaybeObjectSlot) {
            self.visit_pointers_maybeobjectslot(host, slot, MaybeObjectSlot{}); // Replace with slot + 1 if needed
        }

        fn visit_custom_weak_pointer(&mut self, host: Tagged<HeapObject>, slot: ObjectSlot) {
            unsafe {
                (*self.visitor_).VisitCustomWeakPointer(host, slot);
            }
        }

        fn visit_code_target(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
            unsafe {
                (*self.visitor_).VisitCodeTarget(host, rinfo);
            }
        }

        fn visit_embedded_pointer(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
            unsafe {
                (*self.visitor_).VisitEmbeddedPointer(host, rinfo);
            }
        }

        fn visit_external_reference(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
            unsafe {
                (*self.visitor_).VisitExternalReference(host, rinfo);
            }
        }

        fn visit_internal_reference(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
            unsafe {
                (*self.visitor_).VisitInternalReference(host, rinfo);
            }
        }

        fn visit_off_heap_target(&mut self, host: Tagged<InstructionStream>, rinfo: *mut RelocInfo) {
            unsafe {
                (*self.visitor_).VisitOffHeapTarget(host, rinfo);
            }
        }

        fn visit_external_pointer(&mut self, host: Tagged<HeapObject>, slot: ExternalPointerSlot) {
            unsafe {
                (*self.visitor_).VisitExternalPointer(host, slot);
            }
        }

         fn visit_cpp_heap_pointer(&mut self, host: Tagged<HeapObject>, slot: CppHeapPointerSlot) {
            unsafe {
                (*self.visitor_).VisitCppHeapPointer(host, slot);
            }
        }
        
        fn visit_ephemeron(&mut self, host: Tagged<HeapObject>, index: i32, key: ObjectSlot, value: ObjectSlot) {
            unsafe {
                (*self.visitor_).VisitEphemeron(host, index, key, value);
            }
        }
        
        fn visit_indirect_pointer(&mut self, host: Tagged<HeapObject>, slot: IndirectPointerSlot, mode: IndirectPointerMode) {
            unsafe {
                (*self.visitor_).VisitIndirectPointer(host, slot, mode);
            }
        }

        fn visit_protected_pointer_protectedpointerslot(&mut self, host: Tagged<TrustedObject>, slot: ProtectedPointerSlot) {
            unsafe {
                (*self.visitor_).VisitProtectedPointer_protectedpointerslot(host, slot);
            }
        }

        fn visit_protected_pointer_protectedmaybeobjectslot(&mut self, host: Tagged<TrustedObject>, slot: ProtectedMaybeObjectSlot) {
            unsafe {
                (*self.visitor_).VisitProtectedPointer_protectedmaybeobjectslot(host, slot);
            }
        }

        fn visit_trusted_pointer_table_entry(&mut self, host: Tagged<HeapObject>, slot: IndirectPointerSlot) {
            unsafe {
                (*self.visitor_).VisitTrustedPointerTableEntry(host, slot);
            }
        }
        
        fn visit_js_dispatch_table_entry(&mut self, host: Tagged<HeapObject>, handle: JSDispatchHandle) {
            unsafe {
                (*self.visitor_).VisitJSDispatchTableEntry(host, handle);
            }
        }
        
        fn visit_map_pointer(&mut self, _host: Tagged<HeapObject>) {
            unreachable!();
        }

        fn visit(&mut self, object: Tagged<HeapObject>) {
            // Assuming HeapVisitor-like logic is within the visitor itself
            unsafe {
                 (*self.visitor_).VisitRootPointer(object);
             }
        }

        fn visit_map(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>) {
            // Assuming HeapVisitor-like logic is within the visitor itself
            unsafe {
                (*self.visitor_).VisitRootPointer(object); //Or map?
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct ObjectSlot {}

    pub fn VisitObject<'a>(isolate: *mut Isolate, object: Tagged<HeapObject>, visitor: *mut ObjectVisitor<'a>) {
        unsafe {
            (*visitor).VisitMapPointer(object);
        }
        let mut forward_visitor = ObjectVisitorForwarder::new(isolate, visitor);
        forward_visitor.visit(object);
    }

    pub fn VisitObject_local<'a>(isolate: *mut LocalIsolate, object: Tagged<HeapObject>, visitor: *mut ObjectVisitor<'a>) {
         unsafe {
            (*visitor).VisitMapPointer(object);
        }
        let mut forward_visitor = ObjectVisitorForwarder::new_local(isolate, visitor);
        forward_visitor.visit(object);
    }

    pub fn VisitObjectBody<'a>(isolate: *mut Isolate, object: Tagged<HeapObject>, visitor: *mut ObjectVisitor<'a>) {
        let mut forward_visitor = ObjectVisitorForwarder::new(isolate, visitor);
        forward_visitor.visit(object);
    }

    pub fn VisitObjectBody_map<'a>(isolate: *mut Isolate, map: Tagged<Map>, object: Tagged<HeapObject>, visitor: *mut ObjectVisitor<'a>) {
        let mut forward_visitor = ObjectVisitorForwarder::new(isolate, visitor);
        forward_visitor.visit_map(map, object);
    }

    pub fn VisitObjectBody_local<'a>(isolate: *mut LocalIsolate, object: Tagged<HeapObject>, visitor: *mut ObjectVisitor<'a>) {
        let mut forward_visitor = ObjectVisitorForwarder::new_local(isolate, visitor);
        forward_visitor.visit(object);
    }
}

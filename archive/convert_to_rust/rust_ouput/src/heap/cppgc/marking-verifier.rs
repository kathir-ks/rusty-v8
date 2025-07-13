// Converted from V8 C++ source files:
// Header: marking-verifier.h
// Implementation: marking-verifier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/cppgc/marking-verifier.rs
pub mod marking_verifier {
    use std::collections::HashSet;
    use std::optional::Option;
    use std::rc::Rc;
    use std::sync::Mutex;

    use crate::heap::base::stack::StackState;
    use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
    use crate::heap::cppgc::heap_page::{LargePage, NormalPage};
    use crate::heap::cppgc::visitor::Visitor;
    use crate::heap::cppgc::heap::HeapBase;
    use crate::heap::cppgc::marking_visitor::TraceDescriptor;
    use crate::heap::cppgc::object_view::ObjectView;
    use crate::heap::page_metadata::CollectionType;

    pub struct VerificationState {
        parent_: Option<*const HeapObjectHeader>,
    }

    impl VerificationState {
        pub fn new() -> Self {
            VerificationState { parent_: None }
        }

        pub fn verify_marked(&self, base_object_payload: *const std::ffi::c_void) {
            let child_header = unsafe { HeapObjectHeader::from_object(base_object_payload) };

            if !child_header.is_marked() {
                let parent_name = match self.parent_ {
                    Some(parent) => {
                        let header = unsafe {&*parent};
                        header.get_name(crate::heap::cppgc::heap_object_header::HeapObjectNameForUnnamedObject::KUseClassNameIfSupported).value
                    },
                    None => "Stack".to_string(),
                };
                let parent_ptr = match self.parent_ {
                    Some(parent) => {
                       let header = unsafe {&*parent};
                       header.object_start() as *const std::ffi::c_void
                    },
                    None => std::ptr::null(),
                };

                let child_name = child_header
                    .get_name(crate::heap::cppgc::heap_object_header::HeapObjectNameForUnnamedObject::KUseClassNameIfSupported)
                    .value;
                let child_ptr = child_header.object_start() as *const std::ffi::c_void;

               println!(
                    "MarkingVerifier: Encountered unmarked object.\n\
                    #\n\
                    # Hint:\n\
                    #   {} ({:p})\n\
                    #     \\-> {} ({:p})",
                    parent_name,
                    parent_ptr,
                    child_name,
                    child_ptr
                );

               panic!("MarkingVerifier: Encountered unmarked object.");
            }
        }

        pub fn set_current_parent(&mut self, header: Option<*const HeapObjectHeader>) {
            self.parent_ = header;
        }

        pub fn is_parent_on_stack(&self) -> bool {
            self.parent_.is_none()
        }
    }

    pub trait ConservativeTracingVisitor {
        fn trace_conservatively_if_needed(&mut self, address: *const std::ffi::c_void);
    }

    pub trait StackVisitor {
        fn iterate_pointers_until_marker(&mut self, marker: &mut dyn ConservativeTracingVisitor);
    }

    pub struct MarkingVerifierBase<'a> {
        heap_: &'a HeapBase,
        page_backend_: &'a dyn crate::heap::cppgc::heap_page::PageBackend,
        visitor_: Box<dyn Visitor>,
        verification_state_: &'a VerificationState,
        in_construction_objects_heap_: HashSet<*const HeapObjectHeader>,
        in_construction_objects_stack_: HashSet<*const HeapObjectHeader>,
        in_construction_objects_: *mut HashSet<*const HeapObjectHeader>,
        verifier_found_marked_bytes_: usize,
        verifier_found_marked_bytes_are_exact_: bool,
        collection_type_: CollectionType,
        verifier_found_marked_bytes_in_pages_: usize,
    }

    impl<'a> MarkingVerifierBase<'a> {
        pub fn new(
            heap: &'a HeapBase,
            page_backend: &'a dyn crate::heap::cppgc::heap_page::PageBackend,
            collection_type: CollectionType,
            verification_state: &'a VerificationState,
            visitor: Box<dyn Visitor>,
        ) -> Self {
            MarkingVerifierBase {
                heap_: heap,
                page_backend_: page_backend,
                visitor_: visitor,
                verification_state_: verification_state,
                in_construction_objects_heap_: HashSet::new(),
                in_construction_objects_stack_: HashSet::new(),
                in_construction_objects_: std::ptr::null_mut(),
                verifier_found_marked_bytes_: 0,
                verifier_found_marked_bytes_are_exact_: true,
                collection_type_: collection_type,
                verifier_found_marked_bytes_in_pages_: 0,
            }
        }

        pub fn run(&mut self, stack_state: StackState, expected_marked_bytes: Option<usize>) {
            self.traverse(self.heap_.raw_heap());

            if stack_state == StackState::kMayContainHeapPointers {
                self.in_construction_objects_ = &mut self.in_construction_objects_stack_ as *mut HashSet<*const HeapObjectHeader>;
                self.heap_.stack().iterate_pointers_until_marker(self);

                unsafe {
                    assert!(self.in_construction_objects_stack_.len() <= self.in_construction_objects_heap_.len());
                    for header in &self.in_construction_objects_stack_ {
                        assert!(self.in_construction_objects_heap_.contains(header));
                    }
                }
            }

            if let Some(expected_bytes) = expected_marked_bytes {
                if self.verifier_found_marked_bytes_are_exact_ {
                    if expected_bytes != self.verifier_found_marked_bytes_ && self.collection_type_ != CollectionType::kMinor {
                        self.report_differences(expected_bytes);
                    }
                    assert_eq!(expected_bytes, self.verifier_found_marked_bytes_);

                    if self.collection_type_ != CollectionType::kMinor {
                        assert_eq!(expected_bytes, self.verifier_found_marked_bytes_in_pages_);
                    }
                }
            }
        }

        fn visit_in_construction_conservatively(
            &mut self,
            header: *mut HeapObjectHeader,
            callback: fn(&mut Self, *mut HeapObjectHeader),
        ) {
            unsafe {
                let header_ref = &*header;
                if (*self.in_construction_objects_).contains(&(header as *const HeapObjectHeader)) {
                    return;
                }

                (*self.in_construction_objects_).insert(header as *const HeapObjectHeader);

                if self.verification_state_.is_parent_on_stack() {
                    self.verification_state_
                        .verify_marked(header_ref.object_start());
                    return;
                }

                assert!(header_ref.is_marked());
                callback(self, header);
            }
        }

        fn visit_pointer(&mut self, address: *const std::ffi::c_void) {
            self.trace_conservatively_if_needed(address);
        }

        fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
            self.verifier_found_marked_bytes_in_pages_ += page.marked_bytes();
            false
        }

        fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
            self.verifier_found_marked_bytes_in_pages_ += page.marked_bytes();
            false
        }

        fn visit_heap_object_header(&mut self, header: &mut HeapObjectHeader) -> bool {
            if !header.is_marked() {
                return true;
            }

            assert!(!header.is_free());

            #[cfg(feature = "cppgc_young_generation")]
            {
                if self.collection_type_ == CollectionType::kMinor {
                    let caged_heap = crate::heap::cppgc_js::caged_heap::CagedHeap::instance();
                    let age = crate::heap::cppgc_js::caged_heap::CagedHeapLocalData::get().age_table.get_age(
                        caged_heap.offset_from_address(header.object_start()),
                    );

                    if age == crate::heap::cppgc_js::caged_heap::AgeTable::Age::kOld {
                        return true;
                    } else if age == crate::heap::cppgc_js::caged_heap::AgeTable::Age::kMixed {
                        self.verifier_found_marked_bytes_are_exact_ = false;
                    }
                }
            }

            self.verification_state_.set_current_parent(Some(header as *const HeapObjectHeader));

            if !header.is_in_construction() {
                header.trace(self.visitor_.as_mut());
            } else {
                self.trace_conservatively_if_needed(header as *const _ as *const std::ffi::c_void);
            }

            self.verifier_found_marked_bytes_ +=
                ObjectView::new(header).size() + std::mem::size_of::<HeapObjectHeader>();

            self.verification_state_.set_current_parent(None);

            true
        }

        fn report_differences(&self, expected_marked_bytes: usize) {
             eprintln!("\n<--- Mismatch in marking verifier --->\n");
             eprintln!(
                "Marked bytes: expected {} vs. verifier found {}, difference {}\n",
                expected_marked_bytes,
                self.verifier_found_marked_bytes_,
                expected_marked_bytes as i64 - self.verifier_found_marked_bytes_ as i64
            );
            eprintln!(
                "A list of pages with possibly mismatched marked objects follows.\n"
            );

            for space in self.heap_.raw_heap() {
                for page in space {
                    let mut marked_bytes_on_page: usize = 0;
                    if page.is_large() {
                        let large_page = unsafe { &*LargePage::from(page) };
                        let header = large_page.object_header();
                        if header.is_marked() {
                            marked_bytes_on_page +=
                                ObjectView::new(header).size() + std::mem::size_of::<HeapObjectHeader>();
                        }
                        if marked_bytes_on_page == large_page.marked_bytes() {
                            continue;
                        }
                        self.report_large_page(*large_page, marked_bytes_on_page);
                        self.report_heap_object_header(*header);
                    } else {
                        let normal_page = unsafe { &*NormalPage::from(page) };
                        for header in normal_page {
                            if header.is_marked() {
                                marked_bytes_on_page +=
                                    ObjectView::new(header).size() + std::mem::size_of::<HeapObjectHeader>();
                            }
                        }
                        if marked_bytes_on_page == normal_page.marked_bytes() {
                            continue;
                        }
                        self.report_normal_page(*normal_page, marked_bytes_on_page);
                        for header in normal_page {
                            self.report_heap_object_header(*header);
                        }
                    }
                }
            }
        }

        fn report_normal_page(&self, page: NormalPage, marked_bytes_on_page: usize) {
            eprintln!(
                "\nNormal page in space {}:\nMarked bytes: expected {} vs. verifier found {}, difference {}\n",
                page.space().index(),
                page.marked_bytes(),
                marked_bytes_on_page,
                page.marked_bytes() as i64 - marked_bytes_on_page as i64
            );
        }

        fn report_large_page(&self, page: LargePage, marked_bytes_on_page: usize) {
            eprintln!(
                "\nLarge page in space {}:\nMarked bytes: expected {} vs. verifier found {}, difference {}\n",
                page.space().index(),
                page.marked_bytes(),
                marked_bytes_on_page,
                page.marked_bytes() as i64 - marked_bytes_on_page as i64
            );
        }

        fn report_heap_object_header(&self, header: HeapObjectHeader) {
            let name = if header.is_free() {
                "free space".to_string()
            } else {
                header.get_name(crate::heap::cppgc::heap_object_header::HeapObjectNameForUnnamedObject::KUseClassNameIfSupported).value
            };
            eprintln!(
                "- {} at {:p}, size {}, {}\n",
                name,
                header.object_start(),
                header.object_size(),
                if header.is_marked() { "marked" } else { "unmarked" }
            );
        }

        fn traverse(&mut self, raw_heap: &Vec<Box<dyn crate::heap::cppgc::heap_page::PageSpace>>) {
             for space in raw_heap {
                for page in space.iter() {
                    if page.is_large() {
                        let large_page = unsafe { &mut *LargePage::from(page.as_ref()) };
                        self.visit_large_page(large_page);
                    } else {
                        let normal_page = unsafe { &mut *NormalPage::from(page.as_ref()) };
                        self.visit_normal_page(normal_page);
                        for header in normal_page.iter_mut() {
                            self.visit_heap_object_header(header);
                        }
                    }
                }
            }
        }
    }

     impl<'a> ConservativeTracingVisitor for MarkingVerifierBase<'a> {
        fn trace_conservatively_if_needed(&mut self, address: *const std::ffi::c_void) {
            self.visitor_.trace_conservatively_if_needed(address, self.heap_.raw_heap(), self.page_backend_);
        }
    }

    pub struct MarkingVerifier<'a> {
        base_: MarkingVerifierBase<'a>,
        state_: VerificationState,
    }

    impl<'a> MarkingVerifier<'a> {
        pub fn new(heap_base: &'a HeapBase, page_backend: &'a dyn crate::heap::cppgc::heap_page::PageBackend, collection_type: CollectionType) -> Self {
            let state = VerificationState::new();
            let visitor = Box::new(VerificationVisitor::new(&state));
            let base_ = MarkingVerifierBase::new(heap_base, page_backend, collection_type, &state, visitor);
            MarkingVerifier { base_: base_, state_: state }
        }

        pub fn run(&mut self, stack_state: StackState, expected_marked_bytes: Option<usize>) {
            self.base_.run(stack_state, expected_marked_bytes);
        }
    }

    struct VerificationVisitor<'a> {
        state_: &'a VerificationState,
    }

    impl<'a> VerificationVisitor<'a> {
        fn new(state: &'a VerificationState) -> Self {
            VerificationVisitor { state_: state }
        }
    }

    impl<'a> Visitor for VerificationVisitor<'a> {
        fn visit(&mut self, object: *const std::ffi::c_void, desc: TraceDescriptor) {
            self.state_.verify_marked(desc.base_object_payload);
        }

        fn visit_weak(
            &mut self,
            object: *const std::ffi::c_void,
            desc: TraceDescriptor,
            _callback: crate::heap::cppgc::visitor::WeakCallback,
            _payload: *const std::ffi::c_void,
        ) {
            self.state_.verify_marked(desc.base_object_payload);
        }

        fn visit_weak_container(
            &mut self,
            object: *const std::ffi::c_void,
            _trace_descriptor: TraceDescriptor,
            weak_desc: TraceDescriptor,
            _callback: crate::heap::cppgc::visitor::WeakCallback,
            _payload: *const std::ffi::c_void,
        ) {
            if object.is_null() {
                return;
            }
            self.state_.verify_marked(weak_desc.base_object_payload);
        }

        fn trace_conservatively_if_needed(&mut self, address: *const std::ffi::c_void, raw_heap: &Vec<Box<dyn crate::heap::cppgc::heap_page::PageSpace>>, page_backend: &dyn crate::heap::cppgc::heap_page::PageBackend) {
            crate::heap::cppgc::marking_visitor::ConservativeTracingVisitor::trace_conservatively_if_needed(address, raw_heap, page_backend);
        }
    }
}

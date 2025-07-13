// Converted from V8 C++ source files:
// Header: prefinalizer-handler.h
// Implementation: prefinalizer-handler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/cppgc/prefinalizer-handler.h
use std::cell::RefCell;
use std::rc::Rc;

use crate::heap::cppgc::heap::HeapBase;

pub mod prefinalizer_handler {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type Callback = fn(liveness_broker: super::LivenessBroker, object: *mut std::ffi::c_void) -> bool;

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PreFinalizer {
        pub object: *mut std::ffi::c_void,
        pub callback: Callback,
    }

    impl PreFinalizer {
        pub fn new(object: *mut std::ffi::c_void, callback: Callback) -> Self {
            PreFinalizer { object, callback }
        }
    }

    pub struct PreFinalizerHandler {
        ordered_pre_finalizers_: RefCell<Vec<PreFinalizer>>,
        current_ordered_pre_finalizers_: RefCell<Vec<PreFinalizer>>,
        heap_: Rc<RefCell<super::HeapBase>>,
        is_invoking_: RefCell<bool>,
        bytes_allocated_in_prefinalizers: RefCell<usize>,
    }

    impl PreFinalizerHandler {
        pub fn new(heap: Rc<RefCell<super::HeapBase>>) -> Self {
            PreFinalizerHandler {
                ordered_pre_finalizers_: RefCell::new(Vec::new()),
                current_ordered_pre_finalizers_: RefCell::new(Vec::new()),
                heap_: heap,
                is_invoking_: RefCell::new(false),
                bytes_allocated_in_prefinalizers: RefCell::new(0),
            }
        }

        pub fn register_prefinalizer(&self, pre_finalizer: PreFinalizer) {
            if !self.current_thread_is_creation_thread() {
                return;
            }
            let mut ordered_pre_finalizers = self.ordered_pre_finalizers_.borrow_mut();
            if ordered_pre_finalizers.iter().any(|&x| x == pre_finalizer) {
                return;
            }

            let mut current_ordered_pre_finalizers = self.current_ordered_pre_finalizers_.borrow_mut();
             if current_ordered_pre_finalizers.iter().any(|&x| x == pre_finalizer) {
                return;
            }
            current_ordered_pre_finalizers.push(pre_finalizer);
        }

        pub fn invoke_pre_finalizers(&self) {
            use crate::heap::cppgc::stats_collector::StatsCollector;
            use crate::heap::cppgc::liveness_broker::LivenessBrokerFactory;

            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap_.borrow().stats_collector().clone(),
                StatsCollector::kAtomicSweep,
            );
            let nested_stats_scope = StatsCollector::EnabledScope::new(
                self.heap_.borrow().stats_collector().clone(),
                StatsCollector::kSweepInvokePreFinalizers,
            );

            if !self.current_thread_is_creation_thread() {
                return;
            }

            let liveness_broker = LivenessBrokerFactory::create();

            *self.is_invoking_.borrow_mut() = true;
            *self.bytes_allocated_in_prefinalizers.borrow_mut() = 0;

            self.heap_.borrow().object_allocator().reset_linear_allocation_buffers();

            let mut new_ordered_pre_finalizers: Vec<PreFinalizer> = Vec::new();
            *self.current_ordered_pre_finalizers_.borrow_mut() = new_ordered_pre_finalizers;

            let mut ordered_pre_finalizers = self.ordered_pre_finalizers_.borrow_mut();
            ordered_pre_finalizers.retain(|pf| !(pf.callback)(liveness_broker, pf.object));

            #[cfg(not(cppgc_allow_allocations_in_prefinalizers))]
            {
                assert!(self.current_ordered_pre_finalizers_.borrow().is_empty());
            }

            #[cfg(cppgc_allow_allocations_in_prefinalizers)]
            {
                let current_ordered_pre_finalizers = self.current_ordered_pre_finalizers_.borrow();
                ordered_pre_finalizers.extend_from_slice(&current_ordered_pre_finalizers);
            }

            *self.current_ordered_pre_finalizers_.borrow_mut() = ordered_pre_finalizers.clone();
            *self.is_invoking_.borrow_mut() = false;
            ordered_pre_finalizers.shrink_to_fit();
        }

        pub fn is_invoking_pre_finalizers(&self) -> bool {
            *self.is_invoking_.borrow()
        }

        pub fn notify_allocation_in_prefinalizer(&self, size: usize) {
            let mut bytes_allocated = self.bytes_allocated_in_prefinalizers.borrow_mut();
            assert!(*bytes_allocated + size > *bytes_allocated);
            *bytes_allocated += size;
        }

        pub fn extract_bytes_allocated_in_prefinalizers(&self) -> usize {
            std::mem::replace(&mut *self.bytes_allocated_in_prefinalizers.borrow_mut(), 0)
        }

        fn current_thread_is_creation_thread(&self) -> bool {
            #[cfg(debug_assertions)]
            {
                self.heap_.borrow().current_thread_is_heap_thread()
            }
            #[cfg(not(debug_assertions))]
            {
                true
            }
        }
    }
}

// src/heap/cppgc/prefinalizer-handler.cc
use prefinalizer_handler::*;
use crate::heap::cppgc::heap_page::BasePage;
use crate::heap::cppgc::heap::Heap;
use crate::heap::cppgc::liveness_broker::LivenessBroker;
use crate::heap::cppgc::liveness_broker::LivenessBrokerFactory;
use crate::heap::cppgc::stats_collector::StatsCollector;
use crate::heap::cppgc::object_allocator::ObjectAllocator;

pub struct PrefinalizerRegistration {
    object: *mut std::ffi::c_void,
    callback: Callback,
}

impl PrefinalizerRegistration {
    pub fn new(object: *mut std::ffi::c_void, callback: Callback) -> Self {
        let page = unsafe { BasePage::from_payload(object) };
        assert!(!page.space().is_compactable());
        page.heap().prefinalizer_handler().register_prefinalizer(PreFinalizer { object, callback });
        PrefinalizerRegistration { object, callback }
    }
}

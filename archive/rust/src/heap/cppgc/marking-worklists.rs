// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};

// Placeholder for cppgc::Visitor
pub struct TraceDescriptor {}

// Placeholder for heap::base::Worklist
pub struct Worklist<T, const N: usize> {
    items: Vec<T>, // Simulate a worklist with a Vec
}

impl<T, const N: usize> Worklist<T, N> {
    pub fn new() -> Self {
        Worklist { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

// Placeholder for HeapObjectHeader
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapObjectHeader {
    address: usize,
}

impl HeapObjectHeader {
    pub fn new(address: usize) -> Self {
        HeapObjectHeader { address }
    }
}

// Placeholder for cppgc::WeakCallback
pub type WeakCallback = fn();
// Placeholder for TraceCallback
pub type TraceCallback = fn(parameter: *const std::ffi::c_void);

pub mod internal {
    use super::*;

    pub struct MarkingWorklists {
        marking_worklist: MarkingWorklist,
        not_fully_constructed_worklist: NotFullyConstructedWorklist,
        previously_not_fully_constructed_worklist: PreviouslyNotFullyConstructedWorklist,
        write_barrier_worklist: WriteBarrierWorklist,
        weak_container_callback_worklist: WeakCallbackWorklist,
        weak_custom_callback_worklist: WeakCustomCallbackWorklist,
        parallel_weak_callback_worklist: WeakCallbackWorklist,
        concurrent_marking_bailout_worklist: ConcurrentMarkingBailoutWorklist,
        discovered_ephemeron_pairs_worklist: EphemeronPairsWorklist,
        ephemeron_pairs_for_processing_worklist: EphemeronPairsWorklist,
        weak_containers_worklist: WeakContainersWorklist,
        retrace_marked_objects_worklist: RetraceMarkedObjectsWorklist,
    }

    impl MarkingWorklists {
        pub const K_MUTATOR_THREAD_ID: i32 = 0;

        pub type MarkingItem = TraceDescriptor;

        #[derive(Clone, Copy)]
        pub struct WeakCallbackItem {
            pub callback: WeakCallback,
            pub parameter: *const std::ffi::c_void,
        }

        #[derive(Clone, Copy)]
        pub struct ConcurrentMarkingBailoutItem {
            pub parameter: *const std::ffi::c_void,
            pub callback: TraceCallback,
            pub bailedout_size: usize,
        }

        #[derive(Clone, Copy)]
        pub struct EphemeronPairItem {
            pub key: *const std::ffi::c_void,
            pub value: *const std::ffi::c_void,
            pub value_desc: TraceDescriptor,
        }

        pub fn new() -> Self {
            MarkingWorklists {
                marking_worklist: MarkingWorklist::new(),
                not_fully_constructed_worklist: NotFullyConstructedWorklist::new(),
                previously_not_fully_constructed_worklist:
                    PreviouslyNotFullyConstructedWorklist::new(),
                write_barrier_worklist: WriteBarrierWorklist::new(),
                weak_container_callback_worklist: WeakCallbackWorklist::new(),
                weak_custom_callback_worklist: WeakCustomCallbackWorklist::new(),
                parallel_weak_callback_worklist: WeakCallbackWorklist::new(),
                concurrent_marking_bailout_worklist: ConcurrentMarkingBailoutWorklist::new(),
                discovered_ephemeron_pairs_worklist: EphemeronPairsWorklist::new(),
                ephemeron_pairs_for_processing_worklist: EphemeronPairsWorklist::new(),
                weak_containers_worklist: WeakContainersWorklist::new(),
                retrace_marked_objects_worklist: RetraceMarkedObjectsWorklist::new(),
            }
        }

        pub fn marking_worklist(&mut self) -> &mut MarkingWorklist {
            &mut self.marking_worklist
        }

        pub fn not_fully_constructed_worklist(&mut self) -> &mut NotFullyConstructedWorklist {
            &mut self.not_fully_constructed_worklist
        }

        pub fn previously_not_fully_constructed_worklist(
            &mut self,
        ) -> &mut PreviouslyNotFullyConstructedWorklist {
            &mut self.previously_not_fully_constructed_worklist
        }

        pub fn write_barrier_worklist(&mut self) -> &mut WriteBarrierWorklist {
            &mut self.write_barrier_worklist
        }

        pub fn weak_container_callback_worklist(&mut self) -> &mut WeakCallbackWorklist {
            &mut self.weak_container_callback_worklist
        }

        pub fn parallel_weak_callback_worklist(&mut self) -> &mut WeakCallbackWorklist {
            &mut self.parallel_weak_callback_worklist
        }

        pub fn weak_custom_callback_worklist(&mut self) -> &mut WeakCustomCallbackWorklist {
            &mut self.weak_custom_callback_worklist
        }

        pub fn concurrent_marking_bailout_worklist(
            &self,
        ) -> &ConcurrentMarkingBailoutWorklist {
            &self.concurrent_marking_bailout_worklist
        }

        pub fn concurrent_marking_bailout_worklist_mut(
            &mut self,
        ) -> &mut ConcurrentMarkingBailoutWorklist {
            &mut self.concurrent_marking_bailout_worklist
        }

        pub fn discovered_ephemeron_pairs_worklist(&mut self) -> &mut EphemeronPairsWorklist {
            &mut self.discovered_ephemeron_pairs_worklist
        }

        pub fn ephemeron_pairs_for_processing_worklist(
            &mut self,
        ) -> &mut EphemeronPairsWorklist {
            &mut self.ephemeron_pairs_for_processing_worklist
        }

        pub fn weak_containers_worklist(&mut self) -> &mut WeakContainersWorklist {
            &mut self.weak_containers_worklist
        }

        pub fn retrace_marked_objects_worklist(&mut self) -> &mut RetraceMarkedObjectsWorklist {
            &mut self.retrace_marked_objects_worklist
        }

        pub fn clear_for_testing(&mut self) {
            self.marking_worklist.clear();
            self.not_fully_constructed_worklist.clear();
            self.previously_not_fully_constructed_worklist.clear();
            self.write_barrier_worklist.clear();
            self.weak_container_callback_worklist.clear();
            self.weak_custom_callback_worklist.clear();
            self.parallel_weak_callback_worklist.clear();
            self.concurrent_marking_bailout_worklist.clear();
            self.discovered_ephemeron_pairs_worklist.clear();
            self.ephemeron_pairs_for_processing_worklist.clear();
            self.weak_containers_worklist.clear();
            self.retrace_marked_objects_worklist.clear();
        }
    }

    pub type MarkingWorklist = Worklist<MarkingItem, 512>;
    pub type NotFullyConstructedWorklist = ExternalMarkingWorklist;
    pub type PreviouslyNotFullyConstructedWorklist = Worklist<HeapObjectHeader, 16>;
    pub type WeakCallbackWorklist = Worklist<WeakCallbackItem, 64>;
    pub type WeakCustomCallbackWorklist = Worklist<WeakCallbackItem, 16>;
    pub type WriteBarrierWorklist = Worklist<HeapObjectHeader, 64>;
    pub type ConcurrentMarkingBailoutWorklist = Worklist<ConcurrentMarkingBailoutItem, 64>;
    pub type EphemeronPairsWorklist = Worklist<EphemeronPairItem, 64>;
    pub type WeakContainersWorklist = ExternalMarkingWorklist;
    pub type RetraceMarkedObjectsWorklist = Worklist<HeapObjectHeader, 16>;

    pub struct ExternalMarkingWorklist {
        lock: Arc<Mutex<()>>,
        objects: Arc<Mutex<HashSet<HeapObjectHeader>>>,
    }

    impl ExternalMarkingWorklist {
        pub fn new() -> Self {
            ExternalMarkingWorklist {
                lock: Arc::new(Mutex::new(())),
                objects: Arc::new(Mutex::new(HashSet::new())),
            }
        }

        pub fn push(&self, object: HeapObjectHeader) {
            let _guard = self.lock.lock().unwrap();
            self.objects.lock().unwrap().insert(object);
        }

        pub fn contains(&self, object: HeapObjectHeader) -> bool {
            let _guard = self.lock.lock().unwrap();
            self.objects.lock().unwrap().contains(&object)
        }

        pub fn extract(&self) -> HashSet<HeapObjectHeader> {
            let _guard = self.lock.lock().unwrap();
            let mut extracted = HashSet::new();
            std::mem::swap(&mut extracted, &mut self.objects.lock().unwrap());
            extracted
        }

        pub fn clear(&self) {
            let _guard = self.lock.lock().unwrap();
            self.objects.lock().unwrap().clear();
        }

        pub fn is_empty(&self) -> bool {
            let _guard = self.lock.lock().unwrap();
            self.objects.lock().unwrap().is_empty()
        }
    }
}
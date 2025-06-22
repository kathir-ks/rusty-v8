// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK_NOT_NULL {
            ($arg:expr) => {
                if $arg.is_null() {
                    panic!("Argument cannot be null");
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK {
            ($arg:expr) => {
                if !$arg {
                    panic!("Assertion failed: {}", stringify!($arg));
                }
            };
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! V8_EXPORT_PRIVATE {
            () => {};
        }

        #[macro_export]
        macro_rules! V8_LIKELY {
            ($arg:expr) => {
                $arg
            };
        }

        #[macro_export]
        macro_rules! V8_UNLIKELY {
            ($arg:expr) => {
                $arg
            };
        }
    }

    pub mod stack {
        pub fn is_on_stack<T>(ptr: *const T) -> bool {
            false // Placeholder: actual stack check is platform-dependent
        }

        pub struct Stack {}

        impl Stack {
            pub fn is_on_stack<T>(_ptr: *const T) -> bool {
                false
            }
        }
    }
}

mod heap {
    pub mod base {
        use std::collections::hash_map::DefaultHasher;

        use std::hash::{Hash, Hasher};

        pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        pub struct CachedUnorderedMap<K, V, H> {
            map: std::collections::HashMap<K, V>,
            hasher: H,
        }

        impl<K: Eq + Hash + Clone, V: Clone, H: Fn(&K) -> u64> CachedUnorderedMap<K, V, H> {
            pub fn new(hasher: H) -> Self {
                CachedUnorderedMap {
                    map: std::collections::HashMap::new(),
                    hasher,
                }
            }

            pub fn insert(&mut self, key: K, value: V) {
                self.map.insert(key, value);
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.map.get(key)
            }

            pub fn hasher(&self) -> &H {
                &self.hasher
            }
        }
    }
}

mod include {
    pub mod cppgc {
        pub struct TraceDescriptor {
            pub base_object_payload: *mut std::ffi::c_void,
            pub callback: TraceCallback,
        }

        impl TraceDescriptor {
            pub fn new(base_object_payload: *mut std::ffi::c_void, callback: TraceCallback) -> Self {
                TraceDescriptor {
                    base_object_payload,
                    callback,
                }
            }
        }

        pub type TraceCallback = fn(&mut Visitor, *const std::ffi::c_void);
        pub trait TraceTrait {}
        pub trait Visitor {
            fn visit(&mut self, object: *mut std::ffi::c_void);
        }
    }
}

mod src {
    pub mod base {
        pub mod logging {}
        pub mod macros {}
    }
    pub mod heap {
        pub mod base {}
    }
}

pub mod cppgc {
    pub mod internal {
        use crate::base::logging::{DCHECK, DCHECK_NOT_NULL};
        use crate::base::macros::{V8_LIKELY, V8_UNLIKELY, V8_EXPORT_PRIVATE};
        use crate::heap::base::CachedUnorderedMap;
        use crate::heap::base::Stack;
        use crate::include::cppgc::{TraceCallback, TraceDescriptor, Visitor};
        use std::sync::{Arc, Mutex};
        use crate::heap::base;

        pub type Address = *mut u8;
        pub type ConstAddress = *const u8;

        pub type WeakCallback = fn(LivenessBroker, *const std::ffi::c_void);

        pub struct MarkingWorklists {
            marking_worklist: Arc<Mutex<MarkingWorklist>>,
            not_fully_constructed_worklist: Arc<Mutex<NotFullyConstructedWorklist>>,
            previously_not_fully_constructed_worklist: Arc<Mutex<PreviouslyNotFullyConstructedWorklist>>,
            weak_container_callback_worklist: Arc<Mutex<WeakCallbackWorklist>>,
            parallel_weak_callback_worklist: Arc<Mutex<WeakCallbackWorklist>>,
            weak_custom_callback_worklist: Arc<Mutex<WeakCustomCallbackWorklist>>,
            write_barrier_worklist: Arc<Mutex<WriteBarrierWorklist>>,
            concurrent_marking_bailout_worklist: Arc<Mutex<ConcurrentMarkingBailoutWorklist>>,
            ephemeron_pairs_worklist: Arc<Mutex<EphemeronPairsWorklist>>,
            retrace_marked_objects_worklist: Arc<Mutex<RetraceMarkedObjectsWorklist>>,
            weak_containers_worklist: WeakContainersWorklist,
        }

        impl MarkingWorklists {
            pub fn new() -> Self {
                MarkingWorklists {
                    marking_worklist: Arc::new(Mutex::new(MarkingWorklist::new())),
                    not_fully_constructed_worklist: Arc::new(Mutex::new(NotFullyConstructedWorklist::new())),
                    previously_not_fully_constructed_worklist: Arc::new(Mutex::new(PreviouslyNotFullyConstructedWorklist::new())),
                    weak_container_callback_worklist: Arc::new(Mutex::new(WeakCallbackWorklist::new())),
                    parallel_weak_callback_worklist: Arc::new(Mutex::new(WeakCallbackWorklist::new())),
                    weak_custom_callback_worklist: Arc::new(Mutex::new(WeakCustomCallbackWorklist::new())),
                    write_barrier_worklist: Arc::new(Mutex::new(WriteBarrierWorklist::new())),
                    concurrent_marking_bailout_worklist: Arc::new(Mutex::new(ConcurrentMarkingBailoutWorklist::new())),
                    ephemeron_pairs_worklist: Arc::new(Mutex::new(EphemeronPairsWorklist::new())),
                    retrace_marked_objects_worklist: Arc::new(Mutex::new(RetraceMarkedObjectsWorklist::new())),
                    weak_containers_worklist: WeakContainersWorklist::new(),
                }
            }

            pub fn marking_worklist(&self) -> Arc<Mutex<MarkingWorklist>> {
                self.marking_worklist.clone()
            }

            pub fn not_fully_constructed_worklist(&self) -> Arc<Mutex<NotFullyConstructedWorklist>> {
                self.not_fully_constructed_worklist.clone()
            }

            pub fn previously_not_fully_constructed_worklist(&self) -> Arc<Mutex<PreviouslyNotFullyConstructedWorklist>> {
                self.previously_not_fully_constructed_worklist.clone()
            }

            pub fn weak_container_callback_worklist(&self) -> Arc<Mutex<WeakCallbackWorklist>> {
                self.weak_container_callback_worklist.clone()
            }

            pub fn parallel_weak_callback_worklist(&self) -> Arc<Mutex<WeakCallbackWorklist>> {
                self.parallel_weak_callback_worklist.clone()
            }

            pub fn weak_custom_callback_worklist(&self) -> Arc<Mutex<WeakCustomCallbackWorklist>> {
                self.weak_custom_callback_worklist.clone()
            }

            pub fn write_barrier_worklist(&self) -> Arc<Mutex<WriteBarrierWorklist>> {
                self.write_barrier_worklist.clone()
            }

            pub fn concurrent_marking_bailout_worklist(&self) -> Arc<Mutex<ConcurrentMarkingBailoutWorklist>> {
                self.concurrent_marking_bailout_worklist.clone()
            }

            pub fn ephemeron_pairs_worklist(&self) -> Arc<Mutex<EphemeronPairsWorklist>> {
                self.ephemeron_pairs_worklist.clone()
            }

            pub fn retrace_marked_objects_worklist(&self) -> Arc<Mutex<RetraceMarkedObjectsWorklist>> {
                self.retrace_marked_objects_worklist.clone()
            }

            pub fn weak_containers_worklist(&self) -> &WeakContainersWorklist {
                &self.weak_containers_worklist
            }
        }

        // Dummy types, replace with actual implementations
        pub struct MarkingWorklist {
            items: Mutex<Vec<TraceDescriptor>>
        }

        impl MarkingWorklist {
            pub fn new() -> Self {
                MarkingWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, item: TraceDescriptor) {
                self.items.lock().unwrap().push(item);
            }

            pub fn pop(&self) -> Option<TraceDescriptor> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct NotFullyConstructedWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl NotFullyConstructedWorklist {
            pub fn new() -> Self {
                NotFullyConstructedWorklist {
                    items: Mutex::new(Vec::new())
                }
            }
            pub fn push<const MODE: AccessMode>(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }
        pub struct PreviouslyNotFullyConstructedWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl PreviouslyNotFullyConstructedWorklist {
            pub fn new() -> Self {
                PreviouslyNotFullyConstructedWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }
        pub struct WeakCallbackWorklist {
            items: Mutex<Vec<WeakCallbackData>>
        }

        impl WeakCallbackWorklist {
            pub fn new() -> Self {
                WeakCallbackWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, item: WeakCallbackData) {
                self.items.lock().unwrap().push(item);
            }

            pub fn pop(&self) -> Option<WeakCallbackData> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct WeakCustomCallbackWorklist {
            items: Mutex<Vec<WeakCallbackData>>
        }

        impl WeakCustomCallbackWorklist {
            pub fn new() -> Self {
                WeakCustomCallbackWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, item: WeakCallbackData) {
                self.items.lock().unwrap().push(item);
            }

            pub fn pop(&self) -> Option<WeakCallbackData> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct WriteBarrierWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl WriteBarrierWorklist {
            pub fn new() -> Self {
                WriteBarrierWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct ConcurrentMarkingBailoutWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl ConcurrentMarkingBailoutWorklist {
            pub fn new() -> Self {
                ConcurrentMarkingBailoutWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct EphemeronPairsWorklist {
            items: Mutex<Vec<EphemeronPairData>>
        }

        impl EphemeronPairsWorklist {
            pub fn new() -> Self {
                EphemeronPairsWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, item: EphemeronPairData) {
                self.items.lock().unwrap().push(item);
            }

            pub fn pop(&self) -> Option<EphemeronPairData> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct RetraceMarkedObjectsWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl RetraceMarkedObjectsWorklist {
             pub fn new() -> Self {
                RetraceMarkedObjectsWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }

        pub struct WeakContainersWorklist {
            items: Mutex<Vec<*mut HeapObjectHeader>>
        }

        impl WeakContainersWorklist {
            pub fn new() -> Self {
                WeakContainersWorklist {
                    items: Mutex::new(Vec::new())
                }
            }

            pub fn push<const MODE: AccessMode>(&self, header: *mut HeapObjectHeader) {
                self.items.lock().unwrap().push(header);
            }

            pub fn pop(&self) -> Option<*mut HeapObjectHeader> {
                self.items.lock().unwrap().pop()
            }
        }

        pub mod compaction_worklists {
            use std::sync::Mutex;

            pub struct MovableReferencesWorklist {
                items: Mutex<Vec<*const *const std::ffi::c_void>>
            }

            impl MovableReferencesWorklist {
                 pub fn new() -> Self {
                    MovableReferencesWorklist {
                        items: Mutex::new(Vec::new())
                    }
                }
                pub fn push(&self, slot: *const *const std::ffi::c_void) {
                    self.items.lock().unwrap().push(slot);
                }

                pub fn pop(&self) -> Option<*const *const std::ffi::c_void> {
                    self.items.lock().unwrap().pop()
                }
            }

            pub struct CompactionWorklists {
                pub movable_references_worklist: Arc<Mutex<MovableReferencesWorklist>>
            }

            impl CompactionWorklists {
                pub fn new() -> Self {
                    CompactionWorklists {
                        movable_references_worklist: Arc::new(Mutex::new(MovableReferencesWorklist::new()))
                    }
                }

                pub fn movable_references_worklist(&self) -> Arc<Mutex<MovableReferencesWorklist>> {
                    self.movable_references_worklist.clone()
                }
            }
        }
        use compaction_worklists::CompactionWorklists;

        pub struct WeakCallbackData {
            pub callback: WeakCallback,
            pub parameter: *const std::ffi::c_void,
        }

        pub struct EphemeronPairData {
            pub key: *const std::ffi::c_void,
            pub value: *const std::ffi::c_void,
            pub value_desc: TraceDescriptor,
        }

        pub struct GlobalGCInfoTable {}

        impl GlobalGCInfoTable {
            pub fn gc_info_from_index(_index: usize) -> GCInfo {
                GCInfo { trace: None } // Dummy value
            }
        }

        #[derive(Clone, Copy)]
        pub struct GCInfo {
            pub trace: Option<TraceCallback>,
        }

        pub struct HeapBase {}

        impl HeapBase {
            pub fn new() -> Self {
                HeapBase {}
            }
        }

        pub struct HeapPage {}

        pub struct LargePage {}

        impl LargePage {
            pub fn payload_size(&self) -> usize {
                0
            }
        }

        pub struct BasePage {}

        impl BasePage {
            pub fn from_payload<T>(_payload: *const T) -> *mut BasePage {
                ptr::null_mut()
            }

            pub fn object_header_from_inner_address(&self, _address: Address) -> HeapObjectHeader {
                HeapObjectHeader::new()
            }

            pub fn heap(&self) -> &HeapBase {
                unimplemented!()
            }
        }

        pub struct LivenessBrokerFactory {}

        impl LivenessBrokerFactory {
            pub fn create() -> LivenessBroker {
                LivenessBroker {}
            }
        }

        pub struct LivenessBroker {}

        #[derive(Debug)]
        pub struct HeapObjectHeader {
            marked: AtomicBool,
            in_construction: AtomicBool,
            gc_info_index: usize,
        }

        impl HeapObjectHeader {
            pub fn new() -> Self {
                HeapObjectHeader {
                    marked: AtomicBool::new(false),
                    in_construction: AtomicBool::new(false),
                    gc_info_index: 0,
                }
            }

            pub fn from_object<T>(object: *mut T) -> *mut HeapObjectHeader {
                object as *mut HeapObjectHeader // This is a simplification, the real implementation is more complex
            }

            pub fn is_in_construction<const MODE: AccessMode>(&self) -> bool {
                self.in_construction.load(Ordering::Relaxed)
            }

             pub fn set_in_construction<const MODE: AccessMode>(&self, value: bool) {
                self.in_construction.store(value, Ordering::Relaxed);
            }

            pub fn is_marked<const MODE: AccessMode>(&self) -> bool {
                self.marked.load(Ordering::Relaxed)
            }

            pub fn is_marked_relaxed(&self) -> bool {
                self.marked.load(Ordering::Relaxed)
            }

            pub fn is_free<const MODE: AccessMode>(&self) -> bool {
                false
            }

            pub fn try_mark_atomic(&self) -> bool {
                self.marked
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            }

            pub fn object_start(&self) -> *mut std::ffi::c_void {
                self as *const Self as *mut std::ffi::c_void
            }

            pub fn get_gc_info_index(&self) -> usize {
                self.gc_info_index
            }

            pub fn is_large_object<const MODE: AccessMode>(&self) -> bool {
                false
            }

            pub fn allocated_size<const MODE: AccessMode>(&self) -> usize {
                0
            }

            pub fn trace<const MODE: AccessMode>(&self, visitor: &mut dyn Visitor) {
                println!("traced");
            }
        }

        #[derive(Clone, Copy, Debug)]
        pub enum AccessMode {
            kNonAtomic,
            kAtomic,
        }

        impl AccessMode {
            pub const kAtomic: AccessMode = AccessMode::kAtomic;
            //NonAtomic: AccessMode = AccessMode::NonAtomic;
        }

        pub mod globals {
            pub struct Globals {}
        }

        ///Base class for marking state.
        pub struct MarkingStateBase {
            heap_: *mut HeapBase,
            marking_worklist_: Arc<Mutex<MarkingWorklist>>,
            not_fully_constructed_worklist_: Arc<Mutex<NotFullyConstructedWorklist>>,
        }

        impl MarkingStateBase {
            ///Constructor.
            pub fn new(heap: *mut HeapBase, marking_worklists: &MarkingWorklists) -> Self {
                MarkingStateBase {
                    heap_: heap,
                    marking_worklist_: marking_worklists.marking_worklist(),
                    not_fully_constructed_worklist_: marking_worklists.not_fully_constructed_worklist(),
                }
            }

            ///Marks and pushes an object.
            pub fn mark_and_push_ptr(&self, object: *const std::ffi::c_void, desc: TraceDescriptor) {
                unsafe {
                    DCHECK_NOT_NULL!(object);
                }
                self.mark_and_push_header(
                    unsafe {
                        &mut *HeapObjectHeader::from_object(desc.base_object_payload)
                    },
                    desc,
                );
            }

            ///Marks and pushes a header.
            pub fn mark_and_push_header(&self, header: &mut HeapObjectHeader, desc: TraceDescriptor) {
                unsafe {
                    DCHECK_NOT_NULL!(desc.callback);
                }

                if header.is_in_construction::<{ AccessMode::kAtomic }>() {
                    self.not_fully_constructed_worklist_.lock().unwrap().push::<{ AccessMode::kAtomic }>(header);
                } else if self.mark_no_push(header) {
                    self.push_marked(header, desc);
                }
            }

            ///Pushes a marked header.
            pub fn push_marked(&self, header: &mut HeapObjectHeader, desc: TraceDescriptor) {
                DCHECK!(header.is_marked::<{ AccessMode::kAtomic }>());
                DCHECK!(!header.is_in_construction::<{ AccessMode::kAtomic }>());
                unsafe {
                    DCHECK_NOT_NULL!(desc.callback);
                }

                self.marking_worklist_.lock().unwrap().push(desc);
            }

            ///Marks and pushes a header (overload).
            pub fn mark_and_push_header_simple(&self, header: &mut HeapObjectHeader) {
                self.mark_and_push_header(
                    header,
                    TraceDescriptor {
                        base_object_payload: header.object_start(),
                        callback: GlobalGCInfoTable::gc_info_from_index(header.get_gc_info_index()).trace.unwrap(),
                    },
                );
            }

            ///Marks without pushing.
            pub fn mark_no_push(&self, header: &mut HeapObjectHeader) -> bool {
                // A GC should only mark the objects that belong in its heap.
                // Never mark free space objects. This would e.g. hint to marking a promptly
                // freed backing store.
                DCHECK!(!header.is_free::<{ AccessMode::kAtomic }>());
                header.try_mark_atomic()
            }

            ///Publishes the marking state.
            #[V8_EXPORT_PRIVATE]
            pub fn publish(&self) {}

            pub fn marking_worklist(&self) -> Arc<Mutex<MarkingWorklist>> {
                self.marking_worklist_.clone()
            }

            pub fn not_fully_constructed_worklist(&self) -> Arc<Mutex<NotFullyConstructedWorklist>> {
                self.not_fully_constructed_worklist_.clone()
            }
        }

        pub struct BasicMarkingState {
            base: MarkingStateBase,
            previously_not_fully_constructed_worklist_: Arc<Mutex<PreviouslyNotFullyConstructedWorklist>>,
            weak_container_callback_worklist_: Arc<Mutex<WeakCallbackWorklist>>,
            parallel_weak_callback_worklist_: Arc<Mutex<WeakCallbackWorklist>>,
            weak_custom_callback_worklist_: Arc<Mutex<WeakCustomCallbackWorklist>>,
            write_barrier_worklist_: Arc<Mutex<WriteBarrierWorklist>>,
            concurrent_marking_bailout_worklist_: Arc<Mutex<ConcurrentMarkingBailoutWorklist>>,
            discovered_ephemeron_pairs_worklist_: Arc<Mutex<EphemeronPairsWorklist>>,
            ephemeron_pairs_for_processing_worklist_: Arc<Mutex<EphemeronPairsWorklist>>,
            weak_containers_worklist_: WeakContainersWorklist,
            movable_slots_worklist_: Option<Arc<Mutex<compaction_worklists::MovableReferencesWorklist>>>,
            marked_bytes_: usize,
            last_marked_bytes_: usize,
            in_ephemeron_processing_: bool,
            discovered_new_ephemeron_pairs_: bool,
            in_atomic_pause_: bool,
            marked_bytes_map_: Mutex<HashMap<*mut BasePage, i64>>,
        }

        impl BasicMarkingState {
            pub fn new(heap: *mut HeapBase, marking_worklists: &MarkingWorklists, compaction_worklists: Option<&CompactionWorklists>) -> Self {
                BasicMarkingState {
                    base: MarkingStateBase::new(heap, marking_worklists),
                    previously_not_fully_constructed_worklist_: marking_worklists.previously_not_fully_constructed_worklist(),
                    weak_container_callback_worklist_: marking_worklists.weak_container_callback_worklist(),
                    parallel_weak_callback_worklist_: marking_worklists.parallel_weak_callback_worklist(),
                    weak_custom_callback_worklist_: marking_worklists.weak_custom_callback_worklist(),
                    write_barrier_worklist_: marking_worklists.write_barrier_worklist(),
                    concurrent_marking_bailout_worklist_: marking_worklists.concurrent_marking_bailout_worklist(),
                    discovered_ephemeron_pairs_worklist_: marking_worklists.ephemeron_pairs_worklist(),
                    ephemeron_pairs_for_processing_worklist_: marking_worklists.ephemeron_pairs_worklist(),
                    weak_containers_worklist_: marking_worklists.weak_containers_worklist().clone(),
                    movable_slots_worklist_: compaction_worklists.map(|cw| cw.movable_references_worklist()),
                    marked_bytes_: 0,
                    last_marked_bytes_: 0,
                    in_ephemeron_processing_: false,
                    discovered_new_ephemeron_pairs_: false,
                    in_atomic_pause_: false,
                    marked_bytes_map_: Mutex::new(HashMap::new()),
                }
            }

            pub fn register_weak_reference_if_needed(
                &self,
                object: *const std::ffi::c_void,
                desc: TraceDescriptor,
                weak_callback: WeakCallback,
                parameter: *const std::ffi::c_void,
            ) {
                let header = unsafe {
                    &mut *HeapObjectHeader::from_object(desc.base_object_payload)
                };
                if !header.is_in_construction::<{ AccessMode::kAtomic }>() && header.is_marked::<{ AccessMode::kAtomic }>() {
                    return;
                }
                self.parallel_weak_callback_worklist_.lock().unwrap().push(WeakCallbackData {
                    callback: weak_callback,
                    parameter: parameter,
                });
            }

            pub fn register_weak_container_callback(&self, callback: WeakCallback, object: *const std::ffi::c_void) {
                unsafe {
                    DCHECK_NOT_NULL!(callback);
                }
                self.weak_container_callback_worklist_.lock().unwrap().push(WeakCallbackData {
                    callback: callback,
                    parameter: object,
                });
            }

            pub fn register_weak_custom_callback(&self, callback: WeakCallback, object: *const std::ffi::c_void) {
                unsafe {
                    DCHECK_NOT_NULL!(callback);
                }
                self.weak_custom_callback_worklist_.lock().unwrap().push(WeakCallbackData {
                    callback: callback,
                    parameter: object,
                });
            }

             pub fn register_movable_reference(&self, slot: *const *const std::ffi::c_void) {
                if V8_LIKELY!(self.movable_slots_worklist_.is_none()) {
                    return;
                }

                 unsafe {
                    if V8_UNLIKELY!(crate::CPPGC_CAGED_HEAP && !CagedHeapBase::is_within_cage(slot)) {
                        return;
                    }
                    if V8_UNLIKELY!(base::stack::Stack::is_on_stack(slot)) {
                        return;
                    }
                 }

                self.movable_slots_worklist_.as_ref().unwrap().lock().unwrap().push(slot);
            }

            pub fn process_weak_container(
                &self,
                object: *const std::ffi::c_void,
                desc: TraceDescriptor,
                callback: WeakCallback,
                data: *const std::ffi::c_void,
            ) {
                unsafe {
                    DCHECK_NOT_NULL!(object);
                }

                let header = unsafe {
                    &mut *HeapObjectHeader::from_object(object as *mut std::ffi::c_void)
                };

                if header.is_in_construction::<{ AccessMode::kAtomic }>() {
                    self.base.not_fully_constructed_worklist().lock().unwrap().push::<{ AccessMode::kAtomic }>(header);
                    return;
                }

                self.register_weak_container(header);

                if !self.base.mark_no_push(header) {
                    return;
                }

                self.register_weak_container_callback(callback, data);

                if let Some(trace_callback) = GlobalGCInfoTable::gc_info_from_index(header.get_gc_info_index()).trace {
                    self.base.push_marked(header, desc);
                } else {
                    self.account_marked_bytes_header(header);
                }
            }

            pub fn process_ephemeron(&self, key: *const std::ffi::c_void, value: *const std::ffi::c_void, value_desc: TraceDescriptor, visitor: &mut dyn Visitor) {
                DCHECK!(!self.in_ephemeron_processing_);
                self.in_ephemeron_processing_ = true;

                let key_in_construction = unsafe {
                    HeapObjectHeader::from_object(key as *mut std::ffi::c_void).is_in_construction::<{ AccessMode::kAtomic }>()
                };

                let key_considered_as_live =
                    if key_in_construction {
                        self.in_atomic_pause_
                    } else {
                        unsafe {
                            HeapObjectHeader::from_object(key as *mut std::ffi::c_void).is_marked::<{ AccessMode::kAtomic }>()
                        }
                    };

                DCHECK!(
                    !(key_in_construction && self.in_atomic_pause_)
                        || unsafe { HeapObjectHeader::from_object(key as *mut std::ffi::c_void).is_marked::<{ AccessMode::kAtomic }>() }
                );

                if key_considered_as_live {
                    if value_desc.base_object_payload.is_null() {
                       (value_desc.callback)(visitor, value);
                    } else {
                        self.base.mark_and_push_ptr(value_desc.base_object_payload, value_desc);
                    }
                } else {
                    self.discovered_ephemeron_pairs_worklist_.lock().unwrap().push(EphemeronPairData {
                        key: key,
                        value: value,
                        value_desc: value_desc,
                    });
                    self.discovered_new_ephemeron_pairs_ = true;
                }

                self.in_ephemeron_processing_ = false;
            }

            pub fn account_marked_bytes_header(&self, header: &HeapObjectHeader) {
                let marked_bytes =
                    if header.is_large_object::<{ AccessMode::kAtomic }>() {
                        unsafe {
                            (*(BasePage::from_payload(header as *const HeapObjectHeader)))
                                .payload_size()
                        }
                    } else {
                        header.allocated_size::<{ AccessMode::kAtomic }>()
                    };

                let base_page = unsafe {
                    BasePage::from_payload(&(header as *const HeapObjectHeader) as *const *const HeapObjectHeader as *mut HeapObjectHeader)
                };
                self.account_marked_bytes_base_page(base_page, marked_bytes);
            }

            pub fn account_marked_bytes_base_page(&self, base_page: *mut BasePage, marked_bytes: usize) {
                self.marked_bytes_ += marked_bytes;

                let mut marked_bytes_map = self.marked_bytes_map_.lock().unwrap();

                let entry = marked_bytes_map.entry(base_page).or_insert(0);
                *entry += marked_bytes as i64;
            }

            pub fn marked_bytes(&self) -> usize {
                self.marked_bytes_
            }

            pub fn recently_marked_bytes(&mut self) -> usize {
                let current_marked_bytes = self.marked_bytes_;
                let last_marked_bytes
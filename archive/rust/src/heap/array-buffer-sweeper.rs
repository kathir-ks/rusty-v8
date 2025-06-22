// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

// Placeholder for base::logging.  Using println! for now.
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            println!("DCHECK failed: {}", stringify!($condition));
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! DCHECK_NULL {
    ($ptr:expr) => {
        DCHECK!($ptr.is_none());
    };
}

macro_rules! DCHECK_NOT_NULL {
    ($ptr:expr) => {
        DCHECK!($ptr.is_some());
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        DCHECK!($left == $right);
    };
}

macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implication:expr) => {
        if $condition {
            DCHECK!($implication);
        }
    };
}

macro_rules! CHECK {
    ($condition:expr) => {
        if !$condition {
            println!("CHECK failed: {}", stringify!($condition));
            panic!("CHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! TRACE_GC_NOTE {
    ($message:expr) => {
        println!("TRACE_GC_NOTE: {}", $message);
    };
}

// Placeholder for GCTracer and related macros. Using println! for now.
macro_rules! TRACE_GC_WITH_FLOW {
    ($tracer:expr, $scope_id:expr, $trace_id:expr, $flags:expr) => {
        println!("TRACE_GC_WITH_FLOW: scope_id={}, trace_id={}, flags={}", stringify!($scope_id), $trace_id, $flags);
    };
}

macro_rules! TRACE_GC_EPOCH_WITH_FLOW {
    ($tracer:expr, $scope_id:expr, $thread_kind:expr, $trace_id:expr, $flags:expr) => {
        println!("TRACE_GC_EPOCH_WITH_FLOW: scope_id={}, thread_kind={}, trace_id={}, flags={}", stringify!($scope_id), stringify!($thread_kind), $trace_id, $flags);
    };
}

mod objects {
    pub struct JSArrayBuffer {}
}

mod heap {
    use super::*;
    use std::ptr::NonNull;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ExternalBackingStoreType {
        kArrayBuffer,
    }

    // Placeholder for ExternalMemoryAccounter
    #[derive(Default)]
    pub struct ExternalMemoryAccounter {}

    impl ExternalMemoryAccounter {
        pub fn increase(&self, isolate: *mut v8::Isolate, bytes: usize) {
            // Placeholder implementation
            println!(
                "ExternalMemoryAccounter::Increase(isolate={:?}, bytes={})",
                isolate, bytes
            );
        }
        pub fn decrease(&self, isolate: *mut v8::Isolate, bytes: usize) {
            // Placeholder implementation
            println!(
                "ExternalMemoryAccounter::Decrease(isolate={:?}, bytes={})",
                isolate, bytes
            );
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum SweepingType {
        kYoung,
        kFull,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum TreatAllYoungAsPromoted {
        kYes,
        kNo,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Age {
        kYoung,
        kOld,
    }

    #[derive(Debug)]
    pub struct ArrayBufferExtension {
        age: Age,
        accounting_length: usize,
        next: Option<NonNull<ArrayBufferExtension>>,
        marked: bool,
        young_marked: bool,
        young_promoted: bool,
    }

    impl ArrayBufferExtension {
        pub fn new(age: Age, accounting_length: usize) -> Self {
            ArrayBufferExtension {
                age,
                accounting_length,
                next: None,
                marked: false,
                young_marked: false,
                young_promoted: false,
            }
        }

        pub fn age(&self) -> Age {
            self.age
        }

        pub fn accounting_length(&self) -> usize {
            self.accounting_length
        }

        pub fn set_next(&mut self, next: *mut ArrayBufferExtension) {
            self.next = NonNull::new(next);
        }

        pub fn next(&self) -> Option<&ArrayBufferExtension> {
            unsafe { self.next.map(|ptr| ptr.as_ref()) }
        }

        pub fn next_mut(&mut self) -> Option<&mut ArrayBufferExtension> {
            unsafe { self.next.map(|ptr| ptr.as_mut()) }
        }

        pub fn is_marked(&self) -> bool {
            self.marked
        }

        pub fn mark(&mut self) {
            self.marked = true;
        }

        pub fn unmark(&mut self) {
            self.marked = false;
        }

        pub fn is_young_marked(&self) -> bool {
            self.young_marked
        }

        pub fn young_mark(&mut self) {
            self.young_marked = true;
        }

        pub fn young_unmark(&mut self) {
            self.young_marked = false;
        }

        pub fn is_young_promoted(&self) -> bool {
            self.young_promoted
        }

        pub fn young_promote(&mut self) {
            self.young_promoted = true;
        }

        pub fn set_old(&mut self) -> &mut Self {
            self.age = Age::kOld;
            self
        }

        pub fn set_young(&mut self) -> &mut Self {
            self.age = Age::kYoung;
            self
        }

        pub fn clear_accounting_length(&mut self) -> AccountingState {
            let previous_state = AccountingState {
                age: self.age,
                accounting_length: self.accounting_length,
            };
            self.accounting_length = 0;
            previous_state
        }

        pub fn update_accounting_length(&mut self, delta: i64) -> AccountingState {
            let previous_state = AccountingState {
                age: self.age,
                accounting_length: self.accounting_length,
            };
            let new_length = (self.accounting_length as i64) + delta;
            if new_length < 0 {
                self.accounting_length = 0;
            } else {
                self.accounting_length = new_length as usize;
            }
            previous_state
        }

        // Placeholder for ZapExternalPointerTableEntry, which seems V8 specific
        pub fn zap_external_pointer_table_entry(&self) {
            // Implementation specific to V8.  Placeholder for now.
            println!("ZapExternalPointerTableEntry called (placeholder)");
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct AccountingState {
        age: Age,
        accounting_length: usize,
    }

    #[derive(Debug)]
    pub struct ArrayBufferList {
        age: Age,
        head: Option<NonNull<ArrayBufferExtension>>,
        tail: Option<NonNull<ArrayBufferExtension>>,
        bytes: usize,
    }

    impl ArrayBufferList {
        pub fn new(age: Age) -> Self {
            ArrayBufferList {
                age,
                head: None,
                tail: None,
                bytes: 0,
            }
        }

        pub fn append(&mut self, extension: *mut ArrayBufferExtension) -> usize {
            unsafe {
                let extension_ref = &mut *extension;
                if self.head.is_none() {
                    DCHECK_NULL!(self.tail);
                    self.head = NonNull::new(extension);
                    self.tail = NonNull::new(extension);
                } else {
                    DCHECK_NOT_NULL!(self.tail);
                    if let Some(mut tail) = self.tail {
                        (&mut *tail.as_ptr()).set_next(extension);
                    }
                    self.tail = NonNull::new(extension);
                }

                let accounting_length = if self.age == Age::kOld {
                    extension_ref.set_old().accounting_length()
                } else {
                    extension_ref.set_young().accounting_length()
                };
                DCHECK!(self.bytes + accounting_length >= self.bytes);
                self.bytes += accounting_length;
                extension_ref.set_next(std::ptr::null_mut());
                accounting_length
            }
        }

        pub fn append_list(&mut self, mut list: ArrayBufferList) {
            DCHECK_EQ!(self.age, list.age);

            if self.head.is_none() {
                DCHECK_NULL!(self.tail);
                self.head = list.head;
                self.tail = list.tail;
            } else if list.head.is_some() {
                DCHECK_NOT_NULL!(list.tail);
                unsafe {
                    if let Some(mut tail) = self.tail {
                        (&mut *tail.as_ptr()).set_next(list.head.map(|p| p.as_ptr()).unwrap());
                    }
                }
                self.tail = list.tail;
            } else {
                DCHECK_NULL!(list.tail);
            }

            self.bytes += list.approximate_bytes();
            list = ArrayBufferList::new(self.age);
        }

        pub fn contains_slow(&self, extension: *mut ArrayBufferExtension) -> bool {
            let mut current = self.head;
            while let Some(ptr) = current {
                if ptr.as_ptr() == extension {
                    return true;
                }
                unsafe {
                    current = (&*ptr.as_ptr()).next.clone();
                }
            }
            false
        }

        pub fn bytes_slow(&self) -> usize {
            let mut current = self.head;
            let mut sum = 0;
            while let Some(ptr) = current {
                unsafe {
                    sum += (&*ptr.as_ptr()).accounting_length();
                    current = (&*ptr.as_ptr()).next.clone();
                }
            }
            DCHECK!(sum >= self.approximate_bytes());
            sum
        }

        pub fn is_empty(&self) -> bool {
            DCHECK_IMPLIES!(self.head.is_some(), self.tail.is_some());
            DCHECK_IMPLIES!(self.head.is_none(), self.bytes == 0);
            self.head.is_none()
        }

        pub fn approximate_bytes(&self) -> usize {
            self.bytes
        }
    }

    #[derive(Debug)]
    struct SweepingState {
        status: Arc<Mutex<Status>>,
        new_young: ArrayBufferList,
        new_old: ArrayBufferList,
        freed_bytes: usize,
        initial_young_bytes: usize,
        initial_old_bytes: usize,
        young_bytes_accounted: usize,
        old_bytes_accounted: usize,
        job_handle: Option<Arc<dyn JobHandleTrait>>,
    }

    #[derive(PartialEq, Eq, Debug)]
    enum Status {
        kInProgress,
        kDone,
    }

    impl SweepingState {
        fn new(
            heap: &mut Heap,
            young: ArrayBufferList,
            old: ArrayBufferList,
            type_: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
            trace_id: u64,
        ) -> Self {
            let initial_young_bytes = young.bytes;
            let initial_old_bytes = old.bytes;
            let status = Arc::new(Mutex::new(Status::kInProgress));
            let job_handle = V8::get_current_platform().create_job(
                TaskPriority::kUserVisible,
                Box::new(SweepingJob::new(
                    heap,
                    status.clone(),
                    young,
                    old,
                    type_,
                    treat_all_young_as_promoted,
                    trace_id,
                )),
            );
            SweepingState {
                status: status,
                new_young: ArrayBufferList::new(Age::kYoung),
                new_old: ArrayBufferList::new(Age::kOld),
                freed_bytes: 0,
                initial_young_bytes: initial_young_bytes,
                initial_old_bytes: initial_old_bytes,
                young_bytes_accounted: 0,
                old_bytes_accounted: 0,
                job_handle: Some(job_handle),
            }
        }

        fn set_done(&mut self) {
            let mut status = self.status.lock().unwrap();
            *status = Status::kDone;
        }

        fn is_done(&self) -> bool {
            let status = self.status.lock().unwrap();
            *status == Status::kDone
        }

        fn merge_to(&mut self, sweeper: &mut ArrayBufferSweeper) {
            // the worker may see a difference between `young/old_bytes_accounted_` and
            // `initial_young/old_bytes_` due to concurrent main thread adjustments
            // (resizing).
            sweeper.young_bytes_adjustment_while_sweeping +=
                (self.initial_young_bytes as i64) - (self.young_bytes_accounted as i64);
            sweeper.old_bytes_adjustment_while_sweeping +=
                (self.initial_old_bytes as i64) - (self.old_bytes_accounted as i64);
            DCHECK!(
                (self.new_young.bytes as i64)
                    + sweeper.young_bytes_adjustment_while_sweeping
                    + (sweeper.young.bytes as i64)
                    >= 0
            );
            DCHECK!(
                (self.new_old.bytes as i64)
                    + sweeper.old_bytes_adjustment_while_sweeping
                    + (sweeper.old.bytes as i64)
                    >= 0
            );
            sweeper.young.append_list(self.new_young);
            sweeper.old.append_list(self.new_old);
            // Apply pending adjustments from resizing and detaching.
            sweeper.young.bytes = ((sweeper.young.bytes as i64)
                + std::mem::replace(
                    &mut sweeper.young_bytes_adjustment_while_sweeping,
                    0,
                )) as usize;
            sweeper.old.bytes = ((sweeper.old.bytes as i64)
                + std::mem::replace(&mut sweeper.old_bytes_adjustment_while_sweeping, 0))
                as usize;
            sweeper.decrement_external_memory_counters(self.freed_bytes);
        }

        fn start_background_sweeping(&self) {
            if let Some(job_handle) = &self.job_handle {
                job_handle.notify_concurrency_increase();
            }
        }

        fn finish_sweeping(&mut self) {
            if let Some(job_handle) = self.job_handle.take() {
                job_handle.join();
            }
        }
    }

    trait JobHandleTrait: Send + Sync {
        fn notify_concurrency_increase(&self);
        fn join(&self);
        //fn is_valid(&self) -> bool;
    }

    // Placeholder for JobHandle and related traits/enums.
    #[derive(Debug)]
    struct JobHandle {
        valid: bool,
    }

    impl JobHandleTrait for JobHandle {
        fn notify_concurrency_increase(&self) {
            println!("JobHandle::NotifyConcurrencyIncrease (placeholder)");
        }
        fn join(&self) {
            println!("JobHandle::Join (placeholder)");
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum TaskPriority {
        kUserVisible,
    }

    #[derive(Debug)]
    struct SweepingJob {
        heap: *mut Heap, // *mut Heap since we need mutability
        state: Arc<Mutex<SweepingState>>,
        young: ArrayBufferList,
        old: ArrayBufferList,
        type_: SweepingType,
        treat_all_young_as_promoted: TreatAllYoungAsPromoted,
        trace_id: u64,
        local_sweeper: LocalSweeper,
    }

    impl SweepingJob {
        fn new(
            heap: &mut Heap, // &mut Heap since we need mutability
            state: Arc<Mutex<SweepingState>>,
            young: ArrayBufferList,
            old: ArrayBufferList,
            type_: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
            trace_id: u64,
        ) -> Self {
            SweepingJob {
                heap: heap,
                state: state,
                young: young,
                old: old,
                type_: type_,
                treat_all_young_as_promoted: treat_all_young_as_promoted,
                trace_id: trace_id,
                local_sweeper: LocalSweeper::new(heap.sweeper()),
            }
        }

        fn run(&mut self, delegate: &mut dyn JobDelegate) {
            let thread_kind = if delegate.is_joining_thread() {
                ThreadKind::kMain
            } else {
                ThreadKind::kBackground
            };

            let heap = unsafe { &mut *self.heap }; // Dereference the raw pointer
            if self.treat_all_young_as_promoted == TreatAllYoungAsPromoted::kNo {
                // Waiting for promoted page iteration is only needed when not all young
                // array buffers are promoted.
                let scope_id = match self.type_ {
                    SweepingType::kYoung => {
                        if thread_kind == ThreadKind::kMain {
                            GCTracerScope::MINOR_MS_SWEEP
                        } else {
                            GCTracerScope::MINOR_MS_BACKGROUND_SWEEPING
                        }
                    }
                    SweepingType::kFull => {
                        if thread_kind == ThreadKind::kMain {
                            GCTracerScope::MC_SWEEP
                        } else {
                            GCTracerScope::MC_BACKGROUND_SWEEPING
                        }
                    }
                };
                TRACE_GC_EPOCH_WITH_FLOW(
                    heap.tracer(),
                    scope_id,
                    thread_kind,
                    heap.sweeper().get_trace_id_for_flow_event(scope_id),
                    0x01 | 0x02,
                );
                let finished =
                    self.local_sweeper
                        .contribute_and_wait_for_promoted_pages_iteration(delegate);
                DCHECK_IMPLIES!(delegate.is_joining_thread(), finished);
                if !finished {
                    return;
                }
                DCHECK!(!heap.sweeper().is_iterating_promoted_pages());
            }
            let scope_id = match self.type_ {
                SweepingType::kYoung => {
                    if thread_kind == ThreadKind::kMain {
                        GCTracerScope::YOUNG_ARRAY_BUFFER_SWEEP
                    } else {
                        GCTracerScope::BACKGROUND_YOUNG_ARRAY_BUFFER_SWEEP
                    }
                }
                SweepingType::kFull => {
                    if thread_kind == ThreadKind::kMain {
                        GCTracerScope::FULL_ARRAY_BUFFER_SWEEP
                    } else {
                        GCTracerScope::BACKGROUND_FULL_ARRAY_BUFFER_SWEEP
                    }
                }
            };
            TRACE_GC_EPOCH_WITH_FLOW(heap.tracer(), scope_id, thread_kind, self.trace_id, 0x01);
            self.sweep(delegate);
        }

        fn get_max_concurrency(&self, worker_count: usize) -> usize {
            let state = self.state.lock().unwrap();
            if state.is_done() {
                0
            } else {
                1
            }
        }

        fn sweep(&mut self, delegate: &mut dyn JobDelegate) {
            let mut state = self.state.lock().unwrap();
            CHECK!(!state.is_done());
            let is_finished = match self.type_ {
                SweepingType::kYoung => self.sweep_young(delegate),
                SweepingType::kFull => self.sweep_full(delegate),
            };
            if is_finished {
                state.set_done();
            } else {
                TRACE_GC_NOTE!("ArrayBufferSweeper Preempted");
            }
        }

        fn sweep_full(&mut self, delegate: &mut dyn JobDelegate) -> bool {
            DCHECK_EQ!(SweepingType::kFull, self.type_);
            if !self.sweep_list_full(delegate, &mut self.young, Age::kYoung) {
                return false;
            }
            self.sweep_list_full(delegate, &mut self.old, Age::kOld)
        }

        fn sweep_list_full(
            &mut self,
            delegate: &mut dyn JobDelegate,
            list: &mut ArrayBufferList,
            age: Age,
        ) -> bool {
            const K_YIELD_CHECK_INTERVAL: usize = 256;
            assert!(K_YIELD_CHECK_INTERVAL.is_power_of_two());

            let mut current = list.head;

            let mut state = self.state.lock().unwrap();
            let new_old = &mut state.new_old;
            let mut freed_bytes = 0;
            let mut accounted_bytes = 0;
            let mut swept_extensions = 0;

            while let Some(mut current_ptr) = current {
                let mut current_ref;
                unsafe {
                    current_ref = &mut *current_ptr.as_ptr();
                }
                DCHECK_EQ!(list.age, current_ref.age());
                if (swept_extensions & (K_YIELD_CHECK_INTERVAL - 1)) == 0 {
                    if delegate.should_yield() {
                        break;
                    }
                }
                swept_extensions += 1;
                let next;
                unsafe {
                    next = current_ref.next.clone();
                }

                if !current_ref.is_marked() {
                    freed_bytes += current_ref.accounting_length();
                    let current_ptr = current_ptr.as_ptr();
                    unsafe {
                        self.finalize_and_delete(current_ptr);
                    }
                } else {
                    current_ref.unmark();
                    accounted_bytes += new_old.append(current_ptr.as_ptr());
                }

                current = next;
            }

            state.freed_bytes += freed_bytes;
            if age == Age::kYoung {
                state.young_bytes_accounted += freed_bytes + accounted_bytes;
            } else {
                state.old_bytes_accounted += freed_bytes + accounted_bytes;
            }

            list.head = current;
            current.is_none()
        }

        fn sweep_young(&mut self, delegate: &mut dyn JobDelegate) -> bool {
            const K_YIELD_CHECK_INTERVAL: usize = 256;
            assert!(K_YIELD_CHECK_INTERVAL.is_power_of_two());

            DCHECK_EQ!(SweepingType::kYoung, self.type_);
            let mut current = self.young.head;

            let mut state = self.state.lock().unwrap();
            let new_old = &mut state.new_old;
            let new_young = &mut state.new_young;
            let mut freed_bytes = 0;
            let mut accounted_bytes = 0;
            let mut swept_extensions = 0;

            while let Some(mut current_ptr) = current {
                let mut current_ref;
                unsafe {
                    current_ref = &mut *current_ptr.as_ptr();
                }
                DCHECK_EQ!(Age::kYoung, current_ref.age());
                if (swept_extensions & (K_YIELD_CHECK_INTERVAL - 1)) == 0 {
                    if delegate.should_yield() {
                        break;
                    }
                }
                swept_extensions += 1;

                let next;
                unsafe {
                    next = current_ref.next.clone();
                }

                if !current_ref.is_young_marked() {
                    let bytes = current_ref.accounting_length();
                    let current_ptr = current_ptr.as_ptr();
                    unsafe {
                        self.finalize_and_delete(current_ptr);
                    }
                    if bytes != 0 {
                        freed_bytes += bytes;
                    }
                } else {
                    if self.treat_all_young_as_promoted == TreatAllYoungAsPromoted::kYes
                        || current_ref.is_young_promoted()
                    {
                        current_ref.young_unmark();
                        accounted_bytes += new_old.append(current_ptr.as_ptr());
                    } else {
                        current_ref.young_unmark();
                        accounted_bytes += new_young.append(current_ptr.as_ptr());
                    }
                }

                current = next;
            }

            state.freed_bytes += freed_bytes;
            // Update young/old_bytes_accounted_; the worker may see a difference between
            // this and `initial_young/old_bytes_` due to concurrent main thread
            // adjustments.
            state.young_bytes_accounted += freed_bytes + accounted_bytes;

            self.young.head = current;
            current.is_none()
        }

        unsafe fn finalize_and_delete(&mut self, extension: *mut ArrayBufferExtension) {
            (&mut *extension).zap_external_pointer_table_entry();
            drop(Box::from_raw(extension));
        }
    }

    trait JobDelegate: Send + Sync {
        fn should_yield(&mut self) -> bool;
        fn is_joining_thread(&self) -> bool;
    }

    // Placeholder for JobDelegate trait and implementations.
    struct DummyJobDelegate {
        joining_thread: bool,
    }

    impl DummyJobDelegate {
        fn new(joining_thread: bool) -> Self {
            DummyJobDelegate { joining_thread }
        }
    }

    impl JobDelegate for DummyJobDelegate {
        fn should_yield(&mut self) -> bool {
            false // Placeholder implementation
        }
        fn is_joining_thread(&self) -> bool {
            self.joining_thread
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum ThreadKind {
        kMain,
        kBackground,
    }

    // Placeholder for GCTracer and related enums
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum GCTracerScope {
        MINOR_MS_SWEEP,
        MINOR_MS_BACKGROUND_SWEEPING,
        MC_SWEEP,
        MC_BACKGROUND_SWEEPING,
        YOUNG_ARRAY_BUFFER_SWEEP,
        BACKGROUND_YOUNG_ARRAY_BUFFER_SWEEP,
        FULL_ARRAY_BUFFER_SWEEP,
        BACKGROUND_FULL_ARRAY_BUFFER_SWEEP,
        MINOR_MS_FINISH_SWEEP_ARRAY_BUFFERS,
        SCAVENGER_SWEEP_ARRAY_BUFFERS,
        MC_FINISH_SWEEP_ARRAY_BUFFERS,
    }

    pub struct Heap {
        backing_store_bytes: AtomicU64,
        isolate: *mut v8::Isolate,
        sweeper: Mutex<ArrayBufferSweeper>,
        tracer: GCTracer,
        should_reduce_memory: bool,
        is_tearing_down: bool,
        should_use_background_threads: bool,
    }

    impl Heap {
        pub fn new(isolate: *mut v8::Isolate) -> Self {
            Heap {
                backing_store_bytes: AtomicU64::new(0),
                isolate,
                sweeper: Mutex::new(ArrayBufferSweeper::new(unsafe { &mut *std::ptr::null_mut() })), // Dummy heap reference
                tracer: GCTracer::new(),
                should_reduce_memory: false,
                is_tearing_down: false,
                should_use_background_threads: true,
            }
        }

        pub fn backing_store_bytes(&self) -> u64 {
            self.backing_store_bytes.load(Ordering::Relaxed)
        }

        pub fn increment_external_backing_store_bytes(
            &self,
            store_type: ExternalBackingStoreType,
            bytes: usize,
        ) {
            self.backing_store_bytes
                .fetch_add(bytes as u64, Ordering::Relaxed);
            println!(
                "Incremented backing store bytes for {:?} by {}",
                store_type, bytes
            );
        }

        pub fn decrement_external_backing_store_bytes(
            &self,
            store_type: ExternalBackingStoreType,
            bytes: usize,
        ) {
            self.backing_store_bytes
                .fetch_sub(bytes as u64, Ordering::Relaxed);
            println!(
                "Decremented backing store bytes for {:?} by {}",
                store_type, bytes
            );
        }

        pub fn sweeper(&mut self) -> &mut ArrayBufferSweeper {
            self.sweeper.get_mut().unwrap()
        }

        pub fn tracer(&self) -> &GCTracer {
            &self.tracer
        }

        pub fn should_reduce_memory(&self) -> bool {
            self.should_reduce_memory
        }

        pub fn is_tearing_down(&self) -> bool {
            self.is_tearing_down
        }

        pub fn should_use_background_threads(&self) -> bool {
            self.should_use_background_threads
        }
    }

    pub struct ArrayBufferSweeper {
        heap: *mut Heap, //Raw pointer to avoid lifetime issues
        young: ArrayBufferList,
        old: ArrayBufferList,
        state: Option<SweepingState>,
        young_bytes_adjustment_while_sweeping: i64,
        old_bytes_adjustment_while_sweeping: i64,
        external_memory_accounter: ExternalMemoryAccounter,
    }

    impl ArrayBufferSweeper {
        pub fn new(heap: &mut Heap) -> Self {
            ArrayBufferSweeper {
                heap: heap,
                young: ArrayBufferList::new(Age::kYoung),
                old: ArrayBufferList::new(Age::kOld),
                state: None,
                young_bytes_adjustment_while_sweeping: 0,
                old_bytes_adjustment_while_sweeping: 0,
                external_memory_accounter: ExternalMemoryAccounter::default(),
            }
        }

        pub fn ensure_finished(&mut self) {
            if !self.sweeping_in_progress() {
                return;
            }
            self.finish();
        }

        pub fn finish(&mut self) {
            if let Some(mut state) = self.state.take() {
                state.finish_sweeping();
                self.finalize(&mut state);
            }
            let heap = unsafe { &*self.heap }; //Dereference raw pointer
            DCHECK!(heap.backing_store_bytes() <= usize::MAX as u64);
            DCHECK!(!self.sweeping_in_progress());
        }

        pub fn finish_if_done(&mut self) {
            if self.sweeping_in_progress() {
                if let Some(state) = &self.state {
                    if state.is_done() {
                        self.finish();
                    }
                }
            }
        }

        pub fn request_sweep(
            &mut self,
            type_: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
        ) {
            if self.young.is_empty() && (self.old.is_empty() || type_ == SweepingType::kYoung) {
                return;
            }

            let scope_id = match type_ {
                SweepingType::kYoung => {
                    if V8::flags().minor_ms {
                        GCTracerScope::MINOR_MS_FINISH_SWEEP_ARRAY_BUFFERS
                    } else {
                        GCTracerScope::SCAVENGER_SWEEP_ARRAY_BUFFERS
                    }
                }
                SweepingType::kFull => GCTracerScope::MC_FINISH_SWEEP_ARRAY_BUFFERS,
            };

            let trace_id = self.get_trace_id_for_flow_event(scope_id);
            let heap = unsafe { &*self.heap }; //Dereference raw pointer
            TRACE_GC_WITH_FLOW!(heap.tracer(), scope_id, trace_id, 0x02);
            self.prepare(type_, treat_all_young_as_promoted, trace_id);

            DCHECK_IMPLIES!(
                V8::flags().minor_ms && type_ == SweepingType::kYoung,
                !heap.should_reduce_memory()
            );

            if !heap.is_tearing_down()
                && !heap.
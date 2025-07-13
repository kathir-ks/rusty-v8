// Converted from V8 C++ source files:
// Header: array-buffer-sweeper.h
// Implementation: array-buffer-sweeper.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod array_buffer_sweeper {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use std::{mem, ptr};

    use crate::heap::gc_tracer_inl::ThreadKind;
    use crate::heap::memory_chunk::Heap;
    use crate::heap::sweeper::Sweeper;
    use crate::heap::sweeper::Sweeper::LocalSweeper;
    use crate::objects::js_array_buffer::ArrayBufferExtension;
    use crate::objects::js_array_buffer::ArrayBufferExtension::Age;
    use crate::V8_EXPORT_PRIVATE;

    pub struct ArrayBufferList {
        head_: *mut ArrayBufferExtension,
        tail_: *mut ArrayBufferExtension,
        bytes_: usize,
        age_: Age,
    }

    impl ArrayBufferList {
        pub fn new(age: Age) -> Self {
            ArrayBufferList {
                head_: ptr::null_mut(),
                tail_: ptr::null_mut(),
                bytes_: 0,
                age_: age,
            }
        }
        pub fn is_empty(&self) -> bool {
            if self.head_.is_null() {
                self.tail_.is_null()
            } else {
                true
            }
        }

        pub fn approximate_bytes(&self) -> usize {
            self.bytes_
        }

        pub fn bytes_slow(&self) -> usize {
            let mut current = self.head_;
            let mut sum = 0;
            while !current.is_null() {
                unsafe {
                    sum += (*current).accounting_length();
                    current = (*current).next();
                }
            }
            if sum < self.approximate_bytes() {
                println!("sum {}, approx {}", sum, self.approximate_bytes());
            }
            assert!(sum >= self.approximate_bytes());
            sum
        }

        pub fn append(&mut self, extension: *mut ArrayBufferExtension) -> usize {
            unsafe {
                if self.head_.is_null() {
                    assert!(self.tail_.is_null());
                    self.head_ = extension;
                    self.tail_ = extension;
                } else {
                    assert!(!self.tail_.is_null());
                    (*self.tail_).set_next(extension);
                    self.tail_ = extension;
                }
                let accounting_length = if self.age_ == Age::kOld {
                    (*extension).set_old().accounting_length()
                } else {
                    (*extension).set_young().accounting_length()
                };
                assert!(self.bytes_ + accounting_length >= self.bytes_);
                self.bytes_ += accounting_length;
                (*extension).set_next(ptr::null_mut());
                accounting_length
            }
        }

        pub fn append_list(&mut self, mut list: ArrayBufferList) {
            assert_eq!(self.age_, list.age_);
            if self.head_.is_null() {
                assert!(self.tail_.is_null());
                self.head_ = list.head_;
                self.tail_ = list.tail_;
            } else if !list.head_.is_null() {
                assert!(!list.tail_.is_null());
                unsafe {
                    (*self.tail_).set_next(list.head_);
                }
                self.tail_ = list.tail_;
            } else {
                assert!(list.tail_.is_null());
            }
            self.bytes_ += list.approximate_bytes();
            list.head_ = ptr::null_mut();
            list.tail_ = ptr::null_mut();
            list.bytes_ = 0;
        }

        pub fn contains_slow(&self, extension: *mut ArrayBufferExtension) -> bool {
            let mut current = self.head_;
            while !current.is_null() {
                if current == extension {
                    return true;
                }
                unsafe {
                    current = (*current).next();
                }
            }
            false
        }
    }

    pub struct ArrayBufferSweeper {
        heap_: *mut Heap,
        state_: Arc<Mutex<Option<SweepingState>>>,
        young_: Arc<Mutex<ArrayBufferList>>,
        old_: Arc<Mutex<ArrayBufferList>>,
        young_bytes_adjustment_while_sweeping_: AtomicU64,
        old_bytes_adjustment_while_sweeping_: AtomicU64,
        external_memory_accounter_: ExternalMemoryAccounter,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum SweepingType {
        kYoung,
        kFull,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum TreatAllYoungAsPromoted {
        kNo,
        kYes,
    }

    impl ArrayBufferSweeper {
        pub fn new(heap: *mut Heap) -> Self {
            ArrayBufferSweeper {
                heap_: heap,
                state_: Arc::new(Mutex::new(None)),
                young_: Arc::new(Mutex::new(ArrayBufferList::new(Age::kYoung))),
                old_: Arc::new(Mutex::new(ArrayBufferList::new(Age::kOld))),
                young_bytes_adjustment_while_sweeping_: AtomicU64::new(0),
                old_bytes_adjustment_while_sweeping_: AtomicU64::new(0),
                external_memory_accounter_: ExternalMemoryAccounter::new(),
            }
        }

        pub fn request_sweep(
            &self,
            sweeping_type: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
        ) {
            let mut state = self.state_.lock().unwrap();
            if state.is_some() {
                return;
            }
            let young = self.young_.lock().unwrap();
            let old = self.old_.lock().unwrap();
            if young.is_empty() && (old.is_empty() || sweeping_type == SweepingType::kYoung) {
                return;
            }
            drop(young);
            drop(old);
            let scope_id = if sweeping_type == SweepingType::kYoung {
                todo!() //GCTracer::Scope::MINOR_MS_FINISH_SWEEP_ARRAY_BUFFERS
            } else {
                todo!() //GCTracer::Scope::MC_FINISH_SWEEP_ARRAY_BUFFERS
            };
            let trace_id = self.get_trace_id_for_flow_event(scope_id);
            self.prepare(sweeping_type, treat_all_young_as_promoted, trace_id);
            if !self.should_reduce_memory()
                && todo!()
                && self.should_use_background_threads()
            {
                let sweeping_state = self.state_.lock().unwrap();
                unsafe {
                    (*self.heap_).array_buffer_sweeper().state_.as_ref().unwrap()
                }.as_ref().unwrap().lock().unwrap().as_ref().unwrap().start_background_sweeping();
            } else {
                self.finish();
            }
        }

        fn should_reduce_memory(&self) -> bool {
            false
        }

        fn should_use_background_threads(&self) -> bool {
            false
        }

        pub fn ensure_finished(&self) {
            let state = self.state_.lock().unwrap();
            if state.is_none() {
                return;
            }
            self.finish();
        }

        fn finish(&self) {
            unsafe {
                (*self.heap_).array_buffer_sweeper().state_.as_ref().unwrap()
            }.as_ref().unwrap().lock().unwrap().as_ref().unwrap().finish_sweeping();
            self.finalize();
            assert!(self.backing_store_bytes() <= usize::MAX);
            assert!(!self.sweeping_in_progress());
        }

        fn finalize(&self) {
            assert!(self.sweeping_in_progress());
            todo!();
            //state_->MergeTo(this);
            //state_.reset();
            assert!(!self.sweeping_in_progress());
        }

        fn backing_store_bytes(&self) -> usize {
            0
        }

        pub fn sweeping_in_progress(&self) -> bool {
            self.state_.lock().unwrap().is_some()
        }

        fn prepare(
            &self,
            sweeping_type: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
            trace_id: u64,
        ) {
            assert!(!self.sweeping_in_progress());
            match sweeping_type {
                SweepingType::kYoung => {
                    let mut young = self.young_.lock().unwrap();
                    let state = SweepingState::new(
                        self.heap_,
                        mem::replace(&mut *young, ArrayBufferList::new(Age::kYoung)),
                        ArrayBufferList::new(Age::kOld),
                        sweeping_type,
                        treat_all_young_as_promoted,
                        trace_id,
                    );
                    let mut lock = self.state_.lock().unwrap();
                    *lock = Some(state);
                    drop(young);
                }
                SweepingType::kFull => {
                    let mut young = self.young_.lock().unwrap();
                    let mut old = self.old_.lock().unwrap();
                    let state = SweepingState::new(
                        self.heap_,
                        mem::replace(&mut *young, ArrayBufferList::new(Age::kYoung)),
                        mem::replace(&mut *old, ArrayBufferList::new(Age::kOld)),
                        sweeping_type,
                        treat_all_young_as_promoted,
                        trace_id,
                    );
                    let mut lock = self.state_.lock().unwrap();
                    *lock = Some(state);
                    drop(young);
                    drop(old);
                }
            }
            assert!(self.sweeping_in_progress());
        }

        fn release_all(&self, list: &mut ArrayBufferList) {
            let mut current = list.head_;
            while !current.is_null() {
                unsafe {
                    let next = (*current).next();
                    let bytes = (*current).clear_accounting_length().accounting_length();
                    self.decrement_external_memory_counters(bytes);
                    Self::finalize_and_delete(current);
                    current = next;
                }
            }
            list.head_ = ptr::null_mut();
            list.tail_ = ptr::null_mut();
            list.bytes_ = 0;
        }

        fn finalize_and_delete(extension: *mut ArrayBufferExtension) {
            todo!()
        }

        pub fn append(&self, extension: *mut ArrayBufferExtension) {
            let bytes;
            unsafe {
                bytes = (*extension).accounting_length();
            }

            self.ensure_finished();

            unsafe {
                match (*extension).age() {
                    Age::kYoung => {
                        let mut young = self.young_.lock().unwrap();
                        young.append(extension);
                        drop(young);
                    }
                    Age::kOld => {
                        let mut old = self.old_.lock().unwrap();
                        old.append(extension);
                        drop(old);
                    }
                }
            }

            self.increment_external_memory_counters(bytes);
        }

        fn increment_external_memory_counters(&self, bytes: usize) {
            if bytes == 0 {
                return;
            }
            todo!()
        }

        pub fn resize(&self, extension: *mut ArrayBufferExtension, delta: i64) {
            self.finish_if_done();

            let previous_value;
            unsafe {
                previous_value = (*extension).update_accounting_length(delta);
            }
            self.update_approximate_bytes(delta, previous_value.age());
            if delta > 0 {
                self.increment_external_memory_counters(delta as usize);
            } else {
                self.decrement_external_memory_counters((-delta) as usize);
            }
        }

        fn finish_if_done(&self) {
            let state = self.state_.lock().unwrap();
            if state.is_some() {
                self.finish();
            }
        }

        fn update_approximate_bytes(&self, delta: i64, age: Age) {
            match age {
                Age::kYoung => {
                    if !self.sweeping_in_progress() {
                        let mut young = self.young_.lock().unwrap();
                        if (young.bytes_ as i64) < -delta {
                            println!("{} {}", young.bytes_, delta);
                        }
                        assert!((young.bytes_ as i64) >= -delta);
                        young.bytes_ = (young.bytes_ as i64 + delta) as usize;
                        drop(young);
                    } else {
                        todo!()
                    }
                }
                Age::kOld => {
                    if !self.sweeping_in_progress() {
                        let mut old = self.old_.lock().unwrap();
                        if (old.bytes_ as i64) < -delta {
                            println!("{} {}", old.bytes_, delta);
                        }
                        assert!((old.bytes_ as i64) >= -delta);
                        old.bytes_ = (old.bytes_ as i64 + delta) as usize;
                        drop(old);
                    } else {
                        todo!()
                    }
                }
            }
        }

        pub fn detach(&self, extension: *mut ArrayBufferExtension) {
            self.finish_if_done();

            let previous_value;
            unsafe {
                previous_value = (*extension).clear_accounting_length();
            }

            self.update_approximate_bytes(
                -(previous_value.accounting_length() as i64),
                previous_value.age(),
            );
            self.decrement_external_memory_counters(previous_value.accounting_length());
        }

        fn decrement_external_memory_counters(&self, bytes: usize) {
            if bytes == 0 {
                return;
            }
            todo!()
        }

        fn get_trace_id_for_flow_event(&self, scope_id: i32) -> u64 {
            todo!()
        }
        pub fn young(&self) -> Arc<Mutex<ArrayBufferList>> {
            self.young_.clone()
        }
        pub fn old(&self) -> Arc<Mutex<ArrayBufferList>> {
            self.old_.clone()
        }
    }

    struct ExternalMemoryAccounter {}
    impl ExternalMemoryAccounter {
        fn new() -> Self {
            ExternalMemoryAccounter {}
        }
    }

    struct SweepingState {
        status_: Arc<Mutex<Status>>,
        new_young_: ArrayBufferList,
        new_old_: ArrayBufferList,
        freed_bytes_: usize,
        initial_young_bytes_: usize,
        initial_old_bytes_: usize,
        young_bytes_accounted_: usize,
        old_bytes_accounted_: usize,
        job_handle_: i32,
    }

    impl SweepingState {
        fn new(
            heap: *mut Heap,
            young: ArrayBufferList,
            old: ArrayBufferList,
            type_: SweepingType,
            treat_all_young_as_promoted: TreatAllYoungAsPromoted,
            trace_id: u64,
        ) -> SweepingState {
            let mut state = SweepingState {
                status_: Arc::new(Mutex::new(Status::kInProgress)),
                new_young_: ArrayBufferList::new(Age::kYoung),
                new_old_: ArrayBufferList::new(Age::kOld),
                freed_bytes_: 0,
                initial_young_bytes_: young.bytes_,
                initial_old_bytes_: old.bytes_,
                young_bytes_accounted_: 0,
                old_bytes_accounted_: 0,
                job_handle_: 0,
            };
            todo!();
            state
        }
        fn set_done(&self) {
            let mut status = self.status_.lock().unwrap();
            *status = Status::kDone;
            drop(status)
        }
        fn is_done(&self) -> bool {
            let status = self.status_.lock().unwrap();
            *status == Status::kDone
        }
        fn start_background_sweeping(&self) {
            todo!();
        }
        fn finish_sweeping(&self) {
            todo!();
        }
    }
    #[derive(PartialEq, Eq)]
    enum Status {
        kInProgress,
        kDone,
    }
}

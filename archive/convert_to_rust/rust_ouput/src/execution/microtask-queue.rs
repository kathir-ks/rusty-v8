// Converted from V8 C++ source files:
// Header: microtask-queue.h
// Implementation: microtask-queue.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod microtask_queue {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;
    use std::{mem, ptr};

    use crate::v8::internal::{Address, Isolate, Object};

    pub struct MicrotaskQueue {
        size_: i64,
        capacity_: i64,
        start_: i64,
        ring_buffer_: Vec<Address>,
        finished_microtask_count_: i64,
        next_: Option<Rc<RefCell<MicrotaskQueue>>>,
        prev_: Option<Rc<RefCell<MicrotaskQueue>>>,
        microtasks_depth_: i32,
        microtasks_suppressions_: i32,
        debug_microtasks_depth_: i32,
        microtasks_policy_: MicrotasksPolicy,
        is_running_microtasks_: AtomicBool,
        is_running_completed_callbacks_: AtomicBool,
        microtasks_completed_callbacks_:
            Vec<CallbackWithData>,
        microtasks_completed_callbacks_cow_:
            Option<Vec<CallbackWithData>>,
    }

    impl MicrotaskQueue {
        pub const KRING_BUFFER_OFFSET: usize = 0;
        pub const KCAPACITY_OFFSET: usize = 8;
        pub const KSIZE_OFFSET: usize = 16;
        pub const KSTART_OFFSET: usize = 24;
        pub const KFINISHED_MICROTASK_COUNT_OFFSET: usize = 32;
        pub const KMINIMUM_CAPACITY: i64 = 8;

        pub fn set_up_default_microtask_queue(isolate: &mut Isolate) {
            if isolate.default_microtask_queue.is_none() {
                let microtask_queue =
                    Rc::new(RefCell::new(MicrotaskQueue::new_internal()));
                microtask_queue.borrow_mut().next_ =
                    Some(microtask_queue.clone());
                microtask_queue.borrow_mut().prev_ =
                    Some(microtask_queue.clone());
                isolate.default_microtask_queue = Some(microtask_queue);
            }
        }

        pub fn new(isolate: &mut Isolate) -> Rc<RefCell<MicrotaskQueue>> {
            if isolate.default_microtask_queue.is_none() {
                panic!("Default microtask queue must be set up first");
            }

            let microtask_queue =
                Rc::new(RefCell::new(MicrotaskQueue::new_internal()));

            let last = isolate
                .default_microtask_queue
                .as_ref()
                .unwrap()
                .borrow()
                .prev_
                .clone();

            let next = last.as_ref().unwrap().borrow().next_.clone();

            microtask_queue.borrow_mut().next_ = next.clone();
            microtask_queue.borrow_mut().prev_ = last.clone();

            if let Some(next) = next {
                next.borrow_mut().prev_ = Some(microtask_queue.clone());
            }

            if let Some(last) = last {
                last.borrow_mut().next_ = Some(microtask_queue.clone());
            }

            microtask_queue
        }

        fn new_internal() -> MicrotaskQueue {
            MicrotaskQueue {
                size_: 0,
                capacity_: 0,
                start_: 0,
                ring_buffer_: Vec::new(),
                finished_microtask_count_: 0,
                next_: None,
                prev_: None,
                microtasks_depth_: 0,
                microtasks_suppressions_: 0,
                debug_microtasks_depth_: 0,
                microtasks_policy_: MicrotasksPolicy::KAuto,
                is_running_microtasks_: AtomicBool::new(false),
                is_running_completed_callbacks_: AtomicBool::new(false),
                microtasks_completed_callbacks_: Vec::new(),
                microtasks_completed_callbacks_cow_: None,
            }
        }

        pub fn enqueue_microtask(&mut self, microtask: Address) {
            if self.size_ == self.capacity_ {
                let new_capacity =
                    std::cmp::max(MicrotaskQueue::KMINIMUM_CAPACITY, self.capacity_ << 1);
                self.resize_buffer(new_capacity);
            }

            if self.size_ < self.capacity_ {
                if self.ring_buffer_.len() <= ((self.start_ + self.size_) % self.capacity_) as usize {
                    self.ring_buffer_.resize(
                        ((self.start_ + self.size_) % self.capacity_) as usize + 1,
                        Address {address_:0}
                    );
                }
                self.ring_buffer_[((self.start_ + self.size_) % self.capacity_) as usize] = microtask;
                self.size_ += 1;
            }
        }

        fn resize_buffer(&mut self, new_capacity: i64) {
            if self.size_ > new_capacity {
                panic!("Size must be less than or equal to new capacity");
            }

            let mut new_ring_buffer: Vec<Address> = Vec::new();
            new_ring_buffer.resize(new_capacity as usize, Address{address_: 0});

            for i in 0..self.size_ {
                new_ring_buffer[i as usize] =
                    self.ring_buffer_[((self.start_ + i) % self.capacity_) as usize];
            }

            self.ring_buffer_ = new_ring_buffer;
            self.capacity_ = new_capacity;
            self.start_ = 0;
        }

        pub fn call_enqueue_microtask(
            isolate: &mut Isolate,
            microtask_queue_pointer: i64,
            raw_microtask: Address,
        ) -> Address {
            unsafe {
                let microtask_queue =
                    &mut *(microtask_queue_pointer as *mut MicrotaskQueue);
                microtask_queue.enqueue_microtask(raw_microtask);
            }
            Address {address_:0}
        }

        pub fn perform_checkpoint(&mut self, isolate: &mut Isolate) {
            if !self.should_perfom_checkpoint() {
                return;
            }
            self.perform_checkpoint_internal(isolate);
        }

        pub fn should_perfom_checkpoint(&self) -> bool {
            !self.is_running_microtasks()
                && self.microtasks_depth_ == 0
                && !self.has_microtasks_suppressions()
        }

        pub fn increment_microtasks_scope_depth(&mut self) {
            self.microtasks_depth_ += 1;
        }

        pub fn decrement_microtasks_scope_depth(&mut self) {
            self.microtasks_depth_ -= 1;
        }

        pub fn get_microtasks_scope_depth(&self) -> i32 {
            self.microtasks_depth_
        }

        pub fn increment_microtasks_suppressions(&mut self) {
            self.microtasks_suppressions_ += 1;
        }

        pub fn decrement_microtasks_suppressions(&mut self) {
            self.microtasks_suppressions_ -= 1;
        }

        pub fn has_microtasks_suppressions(&self) -> bool {
            self.microtasks_suppressions_ != 0
        }

        pub fn increment_debug_microtasks_scope_depth(&mut self) {
            self.debug_microtasks_depth_ += 1;
        }

        pub fn decrement_debug_microtasks_scope_depth(&mut self) {
            self.debug_microtasks_depth_ -= 1;
        }

        #[cfg(debug_assertions)]
        pub fn debug_microtasks_scope_depth_is_zero(&self) -> bool {
            self.debug_microtasks_depth_ == 0
        }

        #[cfg(not(debug_assertions))]
        pub fn debug_microtasks_scope_depth_is_zero(&self) -> bool {
            true
        }

        pub fn set_microtasks_policy(&mut self, microtasks_policy: MicrotasksPolicy) {
            self.microtasks_policy_ = microtasks_policy;
        }

        pub fn microtasks_policy(&self) -> MicrotasksPolicy {
            self.microtasks_policy_
        }

        pub fn capacity(&self) -> i64 {
            self.capacity_
        }

        pub fn size(&self) -> i64 {
            self.size_
        }

        pub fn start(&self) -> i64 {
            self.start_
        }

        pub fn get(&self, index: i64) -> Address {
            if index < self.size_ {
                self.ring_buffer_[((index + self.start_) % self.capacity_) as usize]
            } else {
                Address {address_: 0}
            }
        }

        pub fn next(&self) -> Option<Rc<RefCell<MicrotaskQueue>>> {
            self.next_.clone()
        }

        pub fn prev(&self) -> Option<Rc<RefCell<MicrotaskQueue>>> {
            self.prev_.clone()
        }

        fn perform_checkpoint_internal(&mut self, isolate: &mut Isolate) {
            if !self.should_perfom_checkpoint() {
                return;
            }
            self.run_microtasks(isolate);
            isolate.clear_kept_objects();
        }

        fn run_microtasks(&mut self, isolate: &mut Isolate) -> i32 {
            let _scope = SetIsRunningMicrotasks {
                flag_: &self.is_running_microtasks_,
            };

            if self.size() == 0 {
                self.on_completed(isolate);
                return 0;
            }

            let base_count = self.finished_microtask_count_;
            let mut processed_microtask_count: i32 = 0;

            self.on_completed(isolate);

            processed_microtask_count
        }

        fn on_completed(&mut self, isolate: &mut Isolate) {
            self.is_running_completed_callbacks_
                .store(true, Ordering::SeqCst);

            for callback in &self.microtasks_completed_callbacks_ {
                (callback.callback)(isolate, callback.data);
            }

            self.is_running_completed_callbacks_
                .store(false, Ordering::SeqCst);

            if self.microtasks_completed_callbacks_cow_.is_some() {
                self.microtasks_completed_callbacks_ =
                    self.microtasks_completed_callbacks_cow_.take().unwrap();
            }
        }

        pub fn add_microtasks_completed_callback(
            &mut self,
            callback: MicrotasksCompletedCallbackWithData,
            data: *mut std::ffi::c_void,
        ) {
            let microtasks_completed_callbacks = if self.is_running_completed_callbacks_.load(Ordering::SeqCst) {
                if self.microtasks_completed_callbacks_cow_.is_none() {
                    self.microtasks_completed_callbacks_cow_ = Some(self.microtasks_completed_callbacks_.clone());
                }
                self.microtasks_completed_callbacks_cow_.as_mut().unwrap()
            } else {
                &mut self.microtasks_completed_callbacks_
            };
            
            let callback_with_data = CallbackWithData { callback, data };

            if !microtasks_completed_callbacks.contains(&callback_with_data) {
                microtasks_completed_callbacks.push(callback_with_data);
            }
        }

        pub fn remove_microtasks_completed_callback(
            &mut self,
            callback: MicrotasksCompletedCallbackWithData,
            data: *mut std::ffi::c_void,
        ) {
           let microtasks_completed_callbacks = if self.is_running_completed_callbacks_.load(Ordering::SeqCst) {
                if self.microtasks_completed_callbacks_cow_.is_none() {
                    self.microtasks_completed_callbacks_cow_ = Some(self.microtasks_completed_callbacks_.clone());
                }
                self.microtasks_completed_callbacks_cow_.as_mut().unwrap()
            } else {
                &mut self.microtasks_completed_callbacks_
            };

            let callback_with_data = CallbackWithData { callback, data };

            if let Some(pos) = microtasks_completed_callbacks.iter().position(|x| *x == callback_with_data) {
                microtasks_completed_callbacks.remove(pos);
            }
        }

        pub fn is_running_microtasks(&self) -> bool {
            self.is_running_microtasks_.load(Ordering::SeqCst)
        }
    }

    impl Drop for MicrotaskQueue {
        fn drop(&mut self) {
            if self.next_.is_some() && !Rc::ptr_eq(
                self.next_.as_ref().unwrap(),
                self.next_.as_ref().unwrap(),
            ) {
                if self.prev_.is_some() && !Rc::ptr_eq(
                    self.prev_.as_ref().unwrap(),
                    self.prev_.as_ref().unwrap(),
                ) {
                    self.next_.as_mut().unwrap().borrow_mut().prev_ =
                        self.prev_.clone();
                    self.prev_.as_mut().unwrap().borrow_mut().next_ =
                        self.next_.clone();
                }
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum MicrotasksPolicy {
        KAuto,
        KScoped,
    }

    struct SetIsRunningMicrotasks<'a> {
        flag_: &'a AtomicBool,
    }

    impl<'a> SetIsRunningMicrotasks<'a> {
        fn new(flag_: &'a mut AtomicBool) -> Self {
            if flag_.load(Ordering::SeqCst) {
                panic!("Flag should be false");
            }
            flag_.store(true, Ordering::SeqCst);
            SetIsRunningMicrotasks { flag_: flag_ }
        }
    }

    impl<'a> Drop for SetIsRunningMicrotasks<'a> {
        fn drop(&mut self) {
            if !self.flag_.load(Ordering::SeqCst) {
                panic!("Flag should be true");
            }
            self.flag_.store(false, Ordering::SeqCst);
        }
    }

    #[derive(Clone, PartialEq)]
    struct CallbackWithData {
        callback: MicrotasksCompletedCallbackWithData,
        data: *mut std::ffi::c_void,
    }

    unsafe impl Eq for CallbackWithData {}

    pub type MicrotasksCompletedCallbackWithData =
        fn(&mut Isolate, *mut std::ffi::c_void);
}

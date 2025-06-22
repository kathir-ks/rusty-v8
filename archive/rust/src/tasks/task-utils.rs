// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/tasks/task-utils.rs

pub mod task_utils {
    use std::boxed::Box;
    use std::ops::DerefMut;
    use std::sync::{Arc, Mutex};

    // Placeholder types.  Replace with actual definitions.
    pub struct Isolate {}
    pub struct CancelableTaskManager {}

    pub trait CancelableTask {
        fn run_internal(&mut self);
        fn cancel(&mut self);
    }

    pub trait CancelableIdleTask {
        fn run_internal(&mut self, deadline_in_seconds: f64);
        fn cancel(&mut self);
    }

    struct CancelableFuncTask {
        manager: Option<Arc<Mutex<CancelableTaskManager>>>,
        isolate: Option<Box<Isolate>>,
        func: Box<dyn FnMut() + Send + 'static>,
        is_cancelled: bool,
    }

    impl CancelableFuncTask {
        fn new_with_isolate(isolate: Box<Isolate>, func: Box<dyn FnMut() + Send + 'static>) -> Self {
            CancelableFuncTask {
                manager: None,
                isolate: Some(isolate),
                func,
                is_cancelled: false,
            }
        }

        fn new_with_manager(manager: Arc<Mutex<CancelableTaskManager>>, func: Box<dyn FnMut() + Send + 'static>) -> Self {
            CancelableFuncTask {
                manager: Some(manager),
                isolate: None,
                func,
                is_cancelled: false,
            }
        }
    }

    impl CancelableTask for CancelableFuncTask {
        fn run_internal(&mut self) {
            if !self.is_cancelled {
                (self.func)();
            }
        }

        fn cancel(&mut self) {
            self.is_cancelled = true;
        }
    }
    
    struct CancelableIdleFuncTask {
        manager: Option<Arc<Mutex<CancelableTaskManager>>>,
        isolate: Option<Box<Isolate>>,
        func: Box<dyn FnMut(f64) + Send + 'static>,
        is_cancelled: bool,
    }

    impl CancelableIdleFuncTask {
         fn new_with_isolate(isolate: Box<Isolate>, func: Box<dyn FnMut(f64) + Send + 'static>) -> Self {
            CancelableIdleFuncTask {
                manager: None,
                isolate: Some(isolate),
                func,
                is_cancelled: false,
            }
        }
        fn new_with_manager(manager: Arc<Mutex<CancelableTaskManager>>, func: Box<dyn FnMut(f64) + Send + 'static>) -> Self {
            CancelableIdleFuncTask {
                manager: Some(manager),
                isolate: None,
                func,
                is_cancelled: false,
            }
        }
    }

    impl CancelableIdleTask for CancelableIdleFuncTask {
        fn run_internal(&mut self, deadline_in_seconds: f64) {
            if !self.is_cancelled {
                (self.func)(deadline_in_seconds);
            }
        }

        fn cancel(&mut self) {
            self.is_cancelled = true;
        }
    }

    pub fn make_cancelable_task(
        isolate: Box<Isolate>,
        func: Box<dyn FnMut() + Send + 'static>,
    ) -> Box<dyn CancelableTask> {
        Box::new(CancelableFuncTask::new_with_isolate(isolate, func))
    }

    pub fn make_cancelable_task_with_manager(
        manager: Arc<Mutex<CancelableTaskManager>>,
        func: Box<dyn FnMut() + Send + 'static>,
    ) -> Box<dyn CancelableTask> {
        Box::new(CancelableFuncTask::new_with_manager(manager, func))
    }

    pub fn make_cancelable_idle_task(
        isolate: Box<Isolate>,
        func: Box<dyn FnMut(f64) + Send + 'static>,
    ) -> Box<dyn CancelableIdleTask> {
        Box::new(CancelableIdleFuncTask::new_with_isolate(isolate, func))
    }

    pub fn make_cancelable_idle_task_with_manager(
        manager: Arc<Mutex<CancelableTaskManager>>,
        func: Box<dyn FnMut(f64) + Send + 'static>,
    ) -> Box<dyn CancelableIdleTask> {
        Box::new(CancelableIdleFuncTask::new_with_manager(manager, func))
    }

}
// Converted from V8 C++ source files:
// Header: v8-microtask-queue.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ptr::null_mut;
use std::sync::{Mutex, MutexGuard};
use crate::Function;
use crate::Isolate;
use crate::MicrotaskQueue;
use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

#[derive(Debug)]
pub enum MicrotaskQueueError {
    QueueEmpty,
    IsolateError,
    GenericError(String),
}

pub type MicrotaskCallback = fn(data: Option<&mut dyn Any>);
pub type MicrotasksCompletedCallbackWithData = fn(data: Option<&mut dyn Any>);

pub struct MicrotaskQueueImpl {
    isolate: *mut Isolate,
    policy: MicrotasksPolicy,
    microtasks: Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>,
    completed_callbacks: Mutex<Vec<(MicrotasksCompletedCallbackWithData, *mut dyn Any)>>,
    is_running: RefCell<bool>,
    scope_depth: RefCell<i32>,
}

impl MicrotaskQueueImpl {
    pub fn new(isolate: *mut Isolate, policy: MicrotasksPolicy) -> Self {
        MicrotaskQueueImpl {
            isolate,
            policy,
            microtasks: Mutex::new(Vec::new()),
            completed_callbacks: Mutex::new(Vec::new()),
            is_running: RefCell::new(false),
            scope_depth: RefCell::new(0),
        }
    }

    pub fn enqueue_microtask<F>(&self, isolate: *mut Isolate, microtask: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut queue = self.microtasks.lock().unwrap();
        queue.push(Box::new(microtask));
    }

    pub fn enqueue_microtask_callback(&self, isolate: *mut Isolate, callback: MicrotaskCallback, data: *mut dyn Any) {
        let microtask = move || {
            if !data.is_null() {
                let mut_ref = unsafe { &mut *data };
                callback(Some(mut_ref));
            } else {
                callback(None);
            }
        };

        let mut queue = self.microtasks.lock().unwrap();
        queue.push(Box::new(microtask));
    }

    pub fn add_microtasks_completed_callback(&self, callback: MicrotasksCompletedCallbackWithData, data: *mut dyn Any) {
        let mut callbacks = self.completed_callbacks.lock().unwrap();
        callbacks.push((callback, data));
    }

    pub fn remove_microtasks_completed_callback(&self, callback: MicrotasksCompletedCallbackWithData, data: *mut dyn Any) {
        let mut callbacks = self.completed_callbacks.lock().unwrap();
        callbacks.retain(|&(cb, d)| cb as usize != callback as usize || d != data);
    }

    pub fn perform_checkpoint(&self, isolate: *mut Isolate) {
        if *self.is_running.borrow() {
            return;
        }

        *self.is_running.borrow_mut() = true;

        while let Some(microtask) = {
            let mut queue = self.microtasks.lock().unwrap();
            if queue.is_empty() {
                None
            } else {
                Some(queue.remove(0))
            }
        } {
            microtask();
        }

        *self.is_running.borrow_mut() = false;

        // Invoke completed callbacks
        let callbacks = self.completed_callbacks.lock().unwrap();
        for &(callback, data) in callbacks.iter() {
            if !data.is_null() {
                let mut_ref = unsafe { &mut *data };
                callback(Some(mut_ref));
            } else {
                callback(None);
            }
        }
    }

    pub fn is_running_microtasks(&self) -> bool {
        *self.is_running.borrow()
    }

    pub fn get_microtasks_scope_depth(&self) -> i32 {
        *self.scope_depth.borrow()
    }

    fn increment_scope_depth(&self) {
        let mut depth = self.scope_depth.borrow_mut();
        *depth += 1;
    }

    fn decrement_scope_depth(&self) {
        let mut depth = self.scope_depth.borrow_mut();
        *depth -= 1;
    }
}

pub struct MicrotaskQueue {
    impl_: Rc<MicrotaskQueueImpl>,
}

impl MicrotaskQueue {
    pub fn new(isolate: *mut Isolate, policy: MicrotasksPolicy) -> Self {
        MicrotaskQueue {
            impl_: Rc::new(MicrotaskQueueImpl::new(isolate, policy)),
        }
    }

    pub fn enqueue_microtask<F>(&self, isolate: *mut Isolate, microtask: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.impl_.enqueue_microtask(isolate, microtask);
    }

    pub fn enqueue_microtask_callback(&self, isolate: *mut Isolate, callback: MicrotaskCallback, data: *mut dyn Any) {
        self.impl_.enqueue_microtask_callback(isolate, callback, data);
    }

    pub fn add_microtasks_completed_callback(&self, callback: MicrotasksCompletedCallbackWithData, data: *mut dyn Any) {
        self.impl_.add_microtasks_completed_callback(callback, data);
    }

    pub fn remove_microtasks_completed_callback(&self, callback: MicrotasksCompletedCallbackWithData, data: *mut dyn Any) {
        self.impl_.remove_microtasks_completed_callback(callback, data);
    }

    pub fn perform_checkpoint(&self, isolate: *mut Isolate) {
        self.impl_.perform_checkpoint(isolate);
    }

    pub fn is_running_microtasks(&self) -> bool {
        self.impl_.is_running_microtasks()
    }

    pub fn get_microtasks_scope_depth(&self) -> i32 {
        self.impl_.get_microtasks_scope_depth()
    }
}

pub struct V8_NODISCARD MicrotasksScope<'a> {
    i_isolate_: *mut Isolate,
    microtask_queue_: Rc<MicrotaskQueueImpl>,
    run_: bool,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> MicrotasksScope<'a> {
    pub enum Type {
        kRunMicrotasks,
        kDoNotRunMicrotasks,
    }

    pub fn new_with_context(_context: *mut v8::Context, type_: Type) -> Self {
        // Assuming that the isolate can be retrieved from the context.  This will need to be fixed.
        let isolate = null_mut();
        let policy = MicrotasksPolicy::kAuto;
        let microtask_queue_impl = Rc::new(MicrotaskQueueImpl::new(isolate, policy));
        let run_ = match type_ {
            Type::kRunMicrotasks => true,
            Type::kDoNotRunMicrotasks => false,
        };
        if run_ {
            microtask_queue_impl.increment_scope_depth();
        }

        MicrotasksScope {
            i_isolate_: isolate,
            microtask_queue_: microtask_queue_impl,
            run_,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_with_isolate_and_queue(isolate: *mut Isolate, microtask_queue: &MicrotaskQueue, type_: Type) -> Self {
        let run_ = match type_ {
            Type::kRunMicrotasks => true,
            Type::kDoNotRunMicrotasks => false,
        };
        if run_ {
            microtask_queue.impl_.increment_scope_depth();
        }
        MicrotasksScope {
            i_isolate_: isolate,
            microtask_queue_: microtask_queue.impl_.clone(),
            run_,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn perform_checkpoint(isolate: *mut Isolate) {
        // Assuming there is a way to access the microtask queue from the isolate
        // This needs to be properly implemented based on the V8 structure
        // let microtask_queue = isolate.get_microtask_queue();
        // microtask_queue.perform_checkpoint(isolate);
    }

    pub fn get_current_depth(isolate: *mut Isolate) -> i32 {
        // Assuming there is a way to access the microtask queue from the isolate
        // This needs to be properly implemented based on the V8 structure
        // let microtask_queue = isolate.get_microtask_queue();
        // microtask_queue.get_microtasks_scope_depth()
        0
    }

    pub fn is_running_microtasks(isolate: *mut Isolate) -> bool {
        // Assuming there is a way to access the microtask queue from the isolate
        // This needs to be properly implemented based on the V8 structure
        // let microtask_queue = isolate.get_microtask_queue();
        // microtask_queue.is_running_microtasks()
        false
    }
}

impl<'a> Drop for MicrotasksScope<'a> {
    fn drop(&mut self) {
        if self.run_ {
            self.microtask_queue_.decrement_scope_depth();
            //if self.microtask_queue_.get_microtasks_scope_depth() == 0 {
                //MicrotasksScope::perform_checkpoint(self.i_isolate_);
            //}
        }
    }
}

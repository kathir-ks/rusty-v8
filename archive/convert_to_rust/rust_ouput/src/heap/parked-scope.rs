// Converted from V8 C++ source files:
// Header: parked-scope.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use std::optional::Optional;
use std::sync::{Mutex, MutexGuard, RecursiveMutex, Condvar};
use std::time::Duration;

use crate::v8::internal::LocalHeap;
use crate::v8::internal::V8_NODISCARD;

pub struct ParkedScope {
    local_heap_: *mut LocalHeap,
}

impl ParkedScope {
    fn new(local_heap: *mut LocalHeap) -> Self {
        unsafe {
            (*local_heap).nested_parked_scopes_ += 1;
            (*local_heap).Park();
        }
        ParkedScope {
            local_heap_: local_heap,
        }
    }
}

impl Drop for ParkedScope {
    fn drop(&mut self) {
        unsafe {
            assert!(0 < (*self.local_heap_).nested_parked_scopes_);
            (*self.local_heap_).nested_parked_scopes_ -= 1;
            (*self.local_heap_).Unpark();
        }
    }
}

pub struct UnparkedScope {
    local_heap_: *mut LocalHeap,
}

impl UnparkedScope {
    fn new(local_heap: *mut LocalHeap) -> Self {
        unsafe {
            (*local_heap).Unpark();
        }
        UnparkedScope {
            local_heap_: local_heap,
        }
    }
}

impl Drop for UnparkedScope {
    fn drop(&mut self) {
        unsafe {
            (*self.local_heap_).Park();
        }
    }
}

pub struct UnparkedScopeIfOnBackground {
    scope_: Option<UnparkedScope>,
}

impl UnparkedScopeIfOnBackground {
    fn new(local_heap: *mut LocalHeap) -> Self {
        let mut scope_: Option<UnparkedScope> = None;
        unsafe {
            if !(*local_heap).is_main_thread() {
                scope_ = Some(UnparkedScope::new(local_heap));
            }
        }
        UnparkedScopeIfOnBackground { scope_: scope_ }
    }
}

pub struct ParkedMutexGuard<'a> {
    mutex_: &'a Mutex<()>,
}

impl<'a> ParkedMutexGuard<'a> {
    pub fn new(local_heap: *mut LocalHeap, mutex: &'a Mutex<()>) -> Self {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            mutex.lock().unwrap();
        }
        ParkedMutexGuard { mutex_: mutex }
    }
}

impl<'a> Drop for ParkedMutexGuard<'a> {
    fn drop(&mut self) {
        self.mutex_.unlock().unwrap();
    }
}

pub struct ParkedRecursiveMutexGuard<'a> {
    mutex_: &'a RecursiveMutex,
}

impl<'a> ParkedRecursiveMutexGuard<'a> {
    pub fn new(local_heap: *mut LocalHeap, mutex: &'a RecursiveMutex) -> Self {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            mutex.lock().unwrap();
        }
        ParkedRecursiveMutexGuard { mutex_: mutex }
    }
}

impl<'a> Drop for ParkedRecursiveMutexGuard<'a> {
    fn drop(&mut self) {
        self.mutex_.unlock().unwrap();
    }
}

pub struct ParkedMutexGuardIf<'a> {
    mutex_: Option<&'a Mutex<()>>,
}

impl<'a> ParkedMutexGuardIf<'a> {
    pub fn new(local_heap: *mut LocalHeap, mutex: &'a Mutex<()>, enable_mutex: bool) -> Self {
        if enable_mutex {
            unsafe {
                let _parked_scope = ParkedScope::new(local_heap);
                mutex.lock().unwrap();
            }
            ParkedMutexGuardIf { mutex_: Some(mutex) }
        } else {
            ParkedMutexGuardIf { mutex_: None }
        }
    }
}

impl<'a> Drop for ParkedMutexGuardIf<'a> {
    fn drop(&mut self) {
        if let Some(mutex) = self.mutex_ {
            mutex.unlock().unwrap();
        }
    }
}

pub struct ParkingConditionVariable {
    condvar: Condvar,
}

impl ParkingConditionVariable {
    pub fn new() -> Self {
        ParkingConditionVariable {
            condvar: Condvar::new(),
        }
    }

    pub fn parked_wait(&self, local_heap: *mut LocalHeap, mutex: &Mutex<()>) {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            let _guard = mutex.lock().unwrap();
            self.condvar.wait(_guard).unwrap();
        }
    }

    pub fn parked_wait_for(&self, local_heap: *mut LocalHeap, mutex: &Mutex<()>, rel_time: Duration) -> bool {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            let _guard = mutex.lock().unwrap();
            let (guard, timeout_result) = self.condvar.wait_timeout(_guard, rel_time).unwrap();
            drop(guard);
            timeout_result.timed_out() == false
        }
    }
}

pub struct ParkingSemaphore {
    semaphore: std::sync::Semaphore,
}

impl ParkingSemaphore {
    pub fn new(count: i32) -> Self {
        ParkingSemaphore {
            semaphore: std::sync::Semaphore::new(count),
        }
    }

    pub fn parked_wait(&self, local_heap: *mut LocalHeap) {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            self.semaphore.acquire().unwrap();
        }
    }

    pub fn parked_wait_for(&self, local_heap: *mut LocalHeap, rel_time: Duration) -> bool {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            self.semaphore.try_acquire_for(rel_time).unwrap()
        }
    }
}

struct Options {}

impl Options {
    pub fn new() -> Self {
        Options {}
    }
}

pub struct ParkingThread {
    thread: std::thread::JoinHandle<()>,
}

impl ParkingThread {
    pub fn new(options: Options, f: impl FnOnce() + Send + 'static) -> Self {
        ParkingThread {
            thread: std::thread::spawn(f),
        }
    }

    pub fn parked_join(&mut self, local_heap: *mut LocalHeap) {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            self.thread.join().unwrap();
        }
    }

    pub fn parked_join_all<ThreadCollection>(local_heap: *mut LocalHeap, threads: &mut ThreadCollection)
        where ThreadCollection: Iterator<Item = &mut ParkingThread> {
        unsafe {
            let _parked_scope = ParkedScope::new(local_heap);
            for thread in threads {
                thread.thread.join().unwrap();
            }
        }
    }
}
trait Lockable {
    fn lock(&self);
    fn unlock(&self);
}

impl Lockable for Mutex<()> {
    fn lock(&self) {
        self.lock().unwrap();
    }
    fn unlock(&self) {
        self.unlock().unwrap();
    }
}

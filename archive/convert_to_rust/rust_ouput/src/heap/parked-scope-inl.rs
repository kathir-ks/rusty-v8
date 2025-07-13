// Converted from V8 C++ source files:
// Header: parked-scope-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::time::Duration;
use std::sync::{Mutex, Arc, Condvar};
use crate::v8::internal::AllowGarbageCollection;

struct LocalIsolate {}
struct LocalHeap {}
struct ParkedScope {}

impl LocalIsolate {
    fn heap(&self) -> LocalHeap {
        LocalHeap {}
    }
}

impl LocalHeap {
    fn ExecuteWhileParked<F>(&self, f: F)
    where
        F: FnOnce(ParkedScope),
    {
        let parked_scope = ParkedScope {};
        f(parked_scope);
    }
}

struct ParkedMutexGuard<'a> {
    mutex_: &'a Mutex<()>,
    locked: bool,
}

impl<'a> ParkedMutexGuard<'a> {
    fn new(local_isolate: &LocalIsolate, mutex: &'a Mutex<()>) -> Self {
        ParkedMutexGuard::from_local_heap(local_isolate.heap(), mutex)
    }

    fn from_local_heap(local_heap: LocalHeap, mutex_: &'a Mutex<()>) -> Self {
        assert!(AllowGarbageCollection::IsAllowed());
        if mutex_.try_lock().is_ok() {
            ParkedMutexGuard { mutex_, locked: true }
        } else {
            let mutex_ref = mutex_;
            local_heap.ExecuteWhileParked(|_| {
                mutex_ref.lock().unwrap();
            });
            ParkedMutexGuard { mutex_, locked: true }
        }
    }
}

impl<'a> Drop for ParkedMutexGuard<'a> {
    fn drop(&mut self) {
        if self.locked {
            self.mutex_.unlock().unwrap();
        }
    }
}

struct ParkedRecursiveMutexGuard<'a> {
    mutex_: &'a Mutex<()>, // Replace with RecursiveMutex if available
    locked: bool,
}

impl<'a> ParkedRecursiveMutexGuard<'a> {
    fn new(local_isolate: &LocalIsolate, mutex: &'a Mutex<()>) -> Self {
        ParkedRecursiveMutexGuard::from_local_heap(local_isolate.heap(), mutex)
    }

    fn from_local_heap(local_heap: LocalHeap, mutex_: &'a Mutex<()>) -> Self {
        assert!(AllowGarbageCollection::IsAllowed());
        if mutex_.try_lock().is_ok() {
            ParkedRecursiveMutexGuard { mutex_, locked: true }
        } else {
            let mutex_ref = mutex_;
            local_heap.ExecuteWhileParked(|_| {
                mutex_ref.lock().unwrap();
            });
            ParkedRecursiveMutexGuard { mutex_, locked: true }
        }
    }
}

impl<'a> Drop for ParkedRecursiveMutexGuard<'a> {
    fn drop(&mut self) {
        if self.locked {
            self.mutex_.unlock().unwrap();
        }
    }
}

struct ParkedMutexGuardIf<'a> {
    mutex_: Option<&'a Mutex<()>>,
    locked: bool,
}

impl<'a> ParkedMutexGuardIf<'a> {
    fn new(local_isolate: &LocalIsolate, mutex: &'a Mutex<()>, enable_mutex: bool) -> Self {
        ParkedMutexGuardIf::from_local_heap(local_isolate.heap(), mutex, enable_mutex)
    }

    fn from_local_heap(local_heap: LocalHeap, mutex_: &'a Mutex<()>, enable_mutex: bool) -> Self {
        assert!(AllowGarbageCollection::IsAllowed());
        if !enable_mutex {
            return ParkedMutexGuardIf { mutex_: None, locked: false };
        }

        if mutex_.try_lock().is_ok() {
            ParkedMutexGuardIf { mutex_: Some(mutex_), locked: true }
        } else {
            let mutex_ref = mutex_;
            local_heap.ExecuteWhileParked(|_| {
                mutex_ref.lock().unwrap();
            });
            ParkedMutexGuardIf { mutex_: Some(mutex_), locked: true }
        }
    }
}

impl<'a> Drop for ParkedMutexGuardIf<'a> {
    fn drop(&mut self) {
        if self.locked {
            if let Some(mutex) = self.mutex_ {
                mutex.unlock().unwrap();
            }
        }
    }
}

struct ParkingConditionVariable {
    condvar: Arc<Condvar>,
}

impl ParkingConditionVariable {
    fn new() -> Self {
        ParkingConditionVariable {
            condvar: Arc::new(Condvar::new()),
        }
    }

    fn ParkedWait(&self, local_isolate: &LocalIsolate, mutex: &Mutex<()>) {
        self.ParkedWait_local_heap(local_isolate.heap(), mutex);
    }

    fn ParkedWait_local_heap(&self, local_heap: LocalHeap, mutex: &Mutex<()>) {
        let condvar = self.condvar.clone();
        local_heap.ExecuteWhileParked(move |_| {
            let mut guard = mutex.lock().unwrap();
            guard = condvar.wait(guard).unwrap();
            drop(guard);
        });
    }

    fn ParkedWaitFor(&self, local_isolate: &LocalIsolate, mutex: &Mutex<()>, rel_time: Duration) -> bool {
        self.ParkedWaitFor_local_heap(local_isolate.heap(), mutex, rel_time)
    }

    fn ParkedWaitFor_local_heap(&self, local_heap: LocalHeap, mutex: &Mutex<()>, rel_time: Duration) -> bool {
        let condvar = self.condvar.clone();
        let mut result = false;
        local_heap.ExecuteWhileParked(move |_| {
            let mut guard = mutex.lock().unwrap();
            let wait_result = condvar.wait_timeout(guard, rel_time).unwrap();
            guard = wait_result.0;
            result = wait_result.1.timed_out();
            drop(guard);
        });
        !result
    }

    fn notify_one(&self) {
        self.condvar.notify_one();
    }
}

use std::sync::Semaphore;
struct ParkingSemaphore {
    semaphore: Arc<Semaphore>,
}

impl ParkingSemaphore {
    fn new(permits: usize) -> Self {
        ParkingSemaphore {
            semaphore: Arc::new(Semaphore::new(permits)),
        }
    }

    fn ParkedWait(&self, local_isolate: &LocalIsolate) {
        self.ParkedWait_local_heap(local_isolate.heap());
    }

    fn ParkedWait_local_heap(&self, local_heap: LocalHeap) {
        let semaphore = self.semaphore.clone();
        local_heap.ExecuteWhileParked(move |_| {
            let _permit = semaphore.acquire().unwrap();
        });
    }

    fn ParkedWaitFor(&self, local_isolate: &LocalIsolate, rel_time: Duration) -> bool {
        self.ParkedWaitFor_local_heap(local_isolate.heap(), rel_time)
    }

    fn ParkedWaitFor_local_heap(&self, local_heap: LocalHeap, rel_time: Duration) -> bool {
        let semaphore = self.semaphore.clone();
        let mut result = false;
        local_heap.ExecuteWhileParked(move |_| {
            result = semaphore.try_acquire_timeout(rel_time).is_ok();
        });
        result
    }

    fn release(&self) {
        self.semaphore.release();
    }
}

use std::thread::{self, JoinHandle};

struct ParkingThread {
    join_handle: Option<JoinHandle<()>>,
}

impl ParkingThread {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let join_handle = thread::spawn(f);
        ParkingThread {
            join_handle: Some(join_handle),
        }
    }

    fn ParkedJoin(&mut self, local_isolate: &LocalIsolate) {
        self.ParkedJoin_local_heap(local_isolate.heap());
    }

    fn ParkedJoin_local_heap(&mut self, local_heap: LocalHeap) {
        if let Some(join_handle) = self.join_handle.take() {
            local_heap.ExecuteWhileParked(move |_| {
                join_handle.join().unwrap();
            });
        }
    }

    fn ParkedJoinAll<ThreadCollection>(local_isolate: &LocalIsolate, threads: &ThreadCollection)
    where
        ThreadCollection: IntoIterator<Item = Self> + Send + Sync,
    {
        Self::ParkedJoinAll_local_heap(local_isolate.heap(), threads);
    }

    fn ParkedJoinAll_local_heap<ThreadCollection>(local_heap: LocalHeap, threads: &ThreadCollection)
    where
        ThreadCollection: IntoIterator<Item = Self> + Send + Sync,
    {
        let thread_handles: Vec<_> = threads.into_iter().map(|mut t| t.join_handle.take()).collect();
        local_heap.ExecuteWhileParked(move |_| {
            for handle in thread_handles {
                if let Some(h) = handle {
                    h.join().unwrap();
                }
            }
        });
    }
}

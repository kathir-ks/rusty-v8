// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod parked_scope {
    use std::ops::{Deref, DerefMut};
    use std::sync::{Mutex, MutexGuard, RecursiveMutex, RecursiveMutexGuard, Condvar, Semaphore, TryLockError, PoisonError};
    use std::time::Duration;
    use std::thread;
    use std::optional::Option;

    // Mock definitions for V8 specific types.  These should be replaced with
    // actual implementations if/when available.
    pub struct LocalIsolate {}
    impl LocalIsolate {
        pub fn heap(&self) -> &LocalHeap {
            unimplemented!()
        }
    }

    pub struct LocalHeap {
        nested_parked_scopes_: usize,
    }

    impl LocalHeap {
        pub fn new() -> Self {
            LocalHeap {
                nested_parked_scopes_: 0,
            }
        }

        pub fn Park(&mut self) {
            // Placeholder for park functionality.
        }

        pub fn Unpark(&mut self) {
            // Placeholder for unpark functionality.
        }

        pub fn is_main_thread(&self) -> bool {
          false
        }
    }

    macro_rules! USE {
        ($x:expr) => {
            let _ = $x;
        };
    }
    
    macro_rules! V8_NODISCARD {
        ($item:item) => {
            #[must_use]
            $item
        };
    }

    macro_rules! V8_INLINE {
        ($item:item) => {
            #[inline]
            $item
        };
    }

    macro_rules! V8_WARN_UNUSED_RESULT {
        ($item:item) => {
            #[must_use]
            $item
        };
    }
    
    macro_rules! DCHECK_LT {
        ($x:expr, $y:expr) => {
          debug_assert!($x < $y)
        }
    }

    /// Scope that explicitly parks a thread, prohibiting access to the heap and the
    /// creation of handles. Do not use this directly! Use the family of
    /// ExecuteWhileParked methods, instead.
    V8_NODISCARD! {
      pub struct ParkedScope<'a> {
          local_heap_: &'a mut LocalHeap,
      }
    }

    impl<'a> ParkedScope<'a> {
        fn new(local_isolate: &mut LocalIsolate) -> Self {
            ParkedScope::from_heap(local_isolate.heap())
        }

        fn from_heap(local_heap: &'a mut LocalHeap) -> Self {
            local_heap.nested_parked_scopes_ += 1;
            local_heap.Park();
            ParkedScope { local_heap_: local_heap }
        }
    }

    impl<'a> Drop for ParkedScope<'a> {
        fn drop(&mut self) {
            DCHECK_LT!(0, self.local_heap_.nested_parked_scopes_);
            self.local_heap_.nested_parked_scopes_ -= 1;
            self.local_heap_.Unpark();
        }
    }

    /// Scope that explicitly unparks a thread, allowing access to the heap and the
    /// creation of handles.
    V8_NODISCARD! {
        pub struct UnparkedScope<'a> {
            local_heap_: &'a mut LocalHeap,
        }
    }

    impl<'a> UnparkedScope<'a> {
        pub fn new(local_isolate: &mut LocalIsolate) -> Self {
            UnparkedScope::from_heap(local_isolate.heap())
        }

        pub fn from_heap(local_heap: &'a mut LocalHeap) -> Self {
            local_heap.Unpark();
            UnparkedScope { local_heap_: local_heap }
        }
    }

    impl<'a> Drop for UnparkedScope<'a> {
        fn drop(&mut self) {
            self.local_heap_.Park();
        }
    }

    /// Scope that explicitly unparks a background thread, allowing access to the
    /// heap and the creation of handles. It has no effect on the main thread.
    V8_NODISCARD! {
        pub struct UnparkedScopeIfOnBackground<'a> {
            scope_: Option<UnparkedScope<'a>>,
        }
    }

    impl<'a> UnparkedScopeIfOnBackground<'a> {
        pub fn new(local_isolate: &mut LocalIsolate) -> Self {
            UnparkedScopeIfOnBackground::from_heap(local_isolate.heap())
        }

        pub fn from_heap(local_heap: &'a mut LocalHeap) -> Self {
            let scope_ = if !local_heap.is_main_thread() {
                Some(UnparkedScope::from_heap(local_heap))
            } else {
                None
            };
            UnparkedScopeIfOnBackground { scope_ }
        }
    }

    /// Scope that automatically parks the thread while blocking on the given
    /// base::Mutex.
    V8_NODISCARD! {
      pub struct ParkedMutexGuard<'a> {
          mutex_: MutexGuard<'a, ()>,
          _parked_scope: ParkedScope<'a>,
      }
    }

    impl<'a> ParkedMutexGuard<'a> {
      V8_INLINE!{
        pub fn new(local_isolate: &mut LocalIsolate, mutex: &'a Mutex<()>) -> Self {
            ParkedMutexGuard::from_heap(local_isolate.heap(), mutex)
        }
      }

      V8_INLINE!{
        pub fn from_heap(local_heap: &'a mut LocalHeap, mutex: &'a Mutex<()>) -> Self {
            let parked_scope = ParkedScope::from_heap(local_heap);
            let mutex_guard = mutex.lock().unwrap();
            ParkedMutexGuard {
                mutex_: mutex_guard,
                _parked_scope: parked_scope,
            }
        }
      }
    }

    /// Scope that automatically parks the thread while blocking on the given
    /// base::RecursiveMutex.
    V8_NODISCARD! {
      pub struct ParkedRecursiveMutexGuard<'a> {
          mutex_: RecursiveMutexGuard<'a, ()>,
          _parked_scope: ParkedScope<'a>,
      }
    }

    impl<'a> ParkedRecursiveMutexGuard<'a> {
      V8_INLINE!{
        pub fn new(local_isolate: &mut LocalIsolate, mutex: &'a RecursiveMutex<()>) -> Self {
            ParkedRecursiveMutexGuard::from_heap(local_isolate.heap(), mutex)
        }
      }

      V8_INLINE!{
        pub fn from_heap(local_heap: &'a mut LocalHeap, mutex: &'a RecursiveMutex<()>) -> Self {
            let parked_scope = ParkedScope::from_heap(local_heap);
            let mutex_guard = mutex.lock().unwrap();
            ParkedRecursiveMutexGuard {
                mutex_: mutex_guard,
                _parked_scope: parked_scope,
            }
        }
      }
    }

    V8_NODISCARD! {
        pub struct ParkedMutexGuardIf<'a> {
            mutex_: Option<MutexGuard<'a, ()>>,
            _parked_scope: Option<ParkedScope<'a>>,
        }
    }

    impl<'a> ParkedMutexGuardIf<'a> {
      V8_INLINE!{
        pub fn new(local_isolate: &mut LocalIsolate, mutex: &'a Mutex<()>, enable_mutex: bool) -> Self {
            ParkedMutexGuardIf::from_heap(local_isolate.heap(), mutex, enable_mutex)
        }
      }

      V8_INLINE!{
        pub fn from_heap(local_heap: &'a mut LocalHeap, mutex: &'a Mutex<()>, enable_mutex: bool) -> Self {
            if enable_mutex {
                let parked_scope = ParkedScope::from_heap(local_heap);
                let mutex_guard = mutex.lock().unwrap();
                ParkedMutexGuardIf {
                    mutex_: Some(mutex_guard),
                    _parked_scope: Some(parked_scope),
                }
            } else {
                ParkedMutexGuardIf {
                    mutex_: None,
                    _parked_scope: None,
                }
            }
        }
      }
    }

    /// A subclass of base::ConditionVariable that automatically parks the thread
    /// while waiting.
    V8_NODISCARD! {
        pub struct ParkingConditionVariable {
            condvar: Condvar,
        }
    }

    impl ParkingConditionVariable {
        pub fn new() -> Self {
            ParkingConditionVariable {
                condvar: Condvar::new(),
            }
        }

      V8_INLINE!{
        pub fn parked_wait<'a>(&self, local_isolate: &mut LocalIsolate, mutex: &mut MutexGuard<'a, ()>) -> Result<(), PoisonError<MutexGuard<'a, ()>>> {
            self.parked_wait_heap(local_isolate.heap(), mutex)
        }
      }

      V8_INLINE!{
        pub fn parked_wait_heap<'a>(&self, local_heap: &mut LocalHeap, mutex: &mut MutexGuard<'a, ()>) -> Result<(), PoisonError<MutexGuard<'a, ()>>> {
            let parked_scope = ParkedScope::from_heap(local_heap);
            let _guard = USE!(parked_scope);
            let _result = self.condvar.wait(mutex)?;
            Ok(())
        }
      }

        pub fn parked_wait_scope<'a>(&self, scope: &ParkedScope, mutex: &mut MutexGuard<'a, ()>) -> Result<(), PoisonError<MutexGuard<'a, ()>>> {
            USE!(scope);
            let _result = self.condvar.wait(mutex)?;
            Ok(())
        }

      V8_INLINE!{
        V8_WARN_UNUSED_RESULT!{
          pub fn parked_wait_for<'a>(&self, local_isolate: &mut LocalIsolate, mutex: &mut MutexGuard<'a, ()>, rel_time: Duration) -> Result<bool, PoisonError<MutexGuard<'a, ()>>> {
              self.parked_wait_for_heap(local_isolate.heap(), mutex, rel_time)
          }
        }
      }

      V8_INLINE!{
        V8_WARN_UNUSED_RESULT!{
          pub fn parked_wait_for_heap<'a>(&self, local_heap: &mut LocalHeap, mutex: &mut MutexGuard<'a, ()>, rel_time: Duration) -> Result<bool, PoisonError<MutexGuard<'a, ()>>> {
              let parked_scope = ParkedScope::from_heap(local_heap);
              let _guard = USE!(parked_scope);
              let result = self.condvar.wait_timeout(mutex, rel_time)?;
              Ok(result.1.timed_out())
          }
        }
      }

        V8_WARN_UNUSED_RESULT!{
            pub fn parked_wait_for_scope<'a>(&self, scope: &ParkedScope, mutex: &mut MutexGuard<'a, ()>, rel_time: Duration) -> Result<bool, PoisonError<MutexGuard<'a, ()>>> {
                USE!(scope);
                let result = self.condvar.wait_timeout(mutex, rel_time)?;
                Ok(result.1.timed_out())
            }
        }

        pub fn notify_one(&self) {
            self.condvar.notify_one();
        }

        pub fn notify_all(&self) {
            self.condvar.notify_all();
        }
    }

    /// A subclass of base::Semaphore that automatically parks the thread while
    /// waiting.
    V8_NODISCARD! {
        pub struct ParkingSemaphore {
            semaphore: Semaphore,
        }
    }

    impl ParkingSemaphore {
        pub fn new(count: i32) -> Self {
            ParkingSemaphore {
                semaphore: Semaphore::new(count),
            }
        }

      V8_INLINE!{
        pub fn parked_wait(&self, local_isolate: &mut LocalIsolate) {
            self.parked_wait_heap(local_isolate.heap());
        }
      }

      V8_INLINE!{
        pub fn parked_wait_heap(&self, local_heap: &mut LocalHeap) {
            let parked_scope = ParkedScope::from_heap(local_heap);
            let _guard = USE!(parked_scope);
            self.semaphore.acquire();
        }
      }

        pub fn parked_wait_scope(&self, scope: &ParkedScope) {
            USE!(scope);
            self.semaphore.acquire();
        }

      V8_INLINE!{
        V8_WARN_UNUSED_RESULT!{
          pub fn parked_wait_for(&self, local_isolate: &mut LocalIsolate, rel_time: Duration) -> bool {
              self.parked_wait_for_heap(local_isolate.heap(), rel_time)
          }
        }
      }

      V8_INLINE!{
        V8_WARN_UNUSED_RESULT!{
          pub fn parked_wait_for_heap(&self, local_heap: &mut LocalHeap, rel_time: Duration) -> bool {
              let parked_scope = ParkedScope::from_heap(local_heap);
              let _guard = USE!(parked_scope);
              self.semaphore.try_acquire_for(rel_time).is_ok()
          }
        }
      }

        pub fn parked_wait_for_scope(&self, scope: &ParkedScope, rel_time: Duration) -> bool {
            USE!(scope);
            self.semaphore.try_acquire_for(rel_time).is_ok()
        }

        pub fn release(&self) {
            self.semaphore.release();
        }
    }

    pub struct ParkingThread {
      thread: thread::JoinHandle<()>, // Replace () with the actual return type if needed
    }
    
    pub struct Options {
        name: Option<String>,
    }

    impl Options {
        pub fn new() -> Self {
            Options {
                name: None,
            }
        }

        pub fn set_name(&mut self, name: String) -> &mut Self {
            self.name = Some(name);
            self
        }
    }

    impl ParkingThread {
        pub fn new(options: Options, f: impl FnOnce() + Send + 'static) -> Self {
            let builder = thread::Builder::new();
            let builder = if let Some(name) = options.name {
                builder.name(name)
            } else {
                builder
            };
            
            let thread = builder
                .spawn(f)
                .expect("Failed to spawn thread");
    
            ParkingThread { thread }
        }

      V8_INLINE!{
        pub fn parked_join(&mut self, local_isolate: &mut LocalIsolate) {
            self.parked_join_heap(local_isolate.heap());
        }
      }

      V8_INLINE!{
        pub fn parked_join_heap(&mut self, local_heap: &mut LocalHeap) {
            let parked_scope = ParkedScope::from_heap(local_heap);
            let _guard = USE!(parked_scope);
            self.thread.join().unwrap();
        }
      }

        pub fn parked_join_scope(&mut self, scope: &ParkedScope) {
            USE!(scope);
            self.thread.join().unwrap();
        }

        pub fn join(&mut self) {
            self.thread.join().unwrap();
        }

        // impl Drop for ParkingThread {
        //     fn drop(&mut self) {
        //         if thread::panicking() {
        //             // Avoid double-panicking.
        //             return;
        //         }
        //         // Join on drop to avoid detached threads.
        //         self.join();
        //     }
        // }
    }

  impl ParkingThread {
    V8_INLINE!{
      pub fn parked_join_all<ThreadCollection>(local_isolate: &mut LocalIsolate, threads: &mut ThreadCollection)
      where
        ThreadCollection: Iterator<Item = &mut ParkingThread>,
      {
          ParkingThread::parked_join_all_heap(local_isolate.heap(), threads);
      }
    }

    V8_INLINE!{
      pub fn parked_join_all_heap<ThreadCollection>(local_heap: &mut LocalHeap, threads: &mut ThreadCollection)
      where
        ThreadCollection: Iterator<Item = &mut ParkingThread>,
      {
          let parked_scope = ParkedScope::from_heap(local_heap);
          let _guard = USE!(parked_scope);
          for thread in threads {
              thread.join();
          }
      }
    }

    pub fn parked_join_all_scope<'a, ThreadCollection>(scope: &ParkedScope, threads: &mut ThreadCollection)
    where
      ThreadCollection: Iterator<Item = &'a mut ParkingThread>,
    {
        USE!(scope);
        for thread in threads {
            thread.join();
        }
    }
  }
}
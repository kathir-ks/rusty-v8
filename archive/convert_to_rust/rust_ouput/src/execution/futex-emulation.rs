// Converted from V8 C++ source files:
// Header: futex-emulation.h
// Implementation: futex-emulation.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod futex_emulation {
    // Copyright 2015 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    use std::any::Any;
    use std::cmp::max;
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::Debug;
    use std::future::Future;
    use std::ops::Deref;
    use std::pin::Pin;
    use std::ptr;
    use std::rc::{Rc, Weak};
    use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU32, Ordering};
    use std::sync::{Arc, Condvar, Mutex, MutexGuard, Weak};
    use std::thread;
    use std::time::{Duration, Instant};

    use crate::Isolate;
    use crate::{
        BackingStore, Context, Data, Global, Handle, HandleScope, JSArrayBuffer, Local, NativeContext, Object, Promise, String_ExternalOneByteStringResource, Value, V8,
    };

    pub enum AtomicsWaitEvent {
        kStartWait,
        kWokenUp,
        kNotEqual,
        kTimedOut,
        kAPIStopped,
        kTerminatedExecution,
    }

    pub struct AtomicsWaitWakeHandle {
        isolate_: *mut Isolate,
        stopped_: AtomicBool,
    }

    impl AtomicsWaitWakeHandle {
        pub fn new(isolate: *mut Isolate) -> Self {
            AtomicsWaitWakeHandle {
                isolate_: isolate,
                stopped_: AtomicBool::new(false),
            }
        }

        pub fn wake(&self) {
            unsafe {
                (*self.isolate_).futex_wait_list_node().notify_wake();
            }
            self.stopped_.store(true, Ordering::SeqCst);
        }

        #[inline]
        pub fn has_stopped(&self) -> bool {
            self.stopped_.load(Ordering::SeqCst)
        }
    }

    #[derive(Debug)]
    struct AsyncState {
        isolate_for_async_waiters: *mut Isolate,
        task_runner: Arc<TaskRunner>,
        backing_store: Weak<BackingStore>,
        promise: Global<Promise>,
        native_context: Global<Context>,
        timeout_time: Mutex<Option<Instant>>,
        timeout_task_id: AtomicU32,
    }

    impl AsyncState {
        fn new(
            isolate: *mut Isolate,
            task_runner: Arc<TaskRunner>,
            backing_store: Weak<BackingStore>,
            promise: Global<Promise>,
            native_context: Global<Context>,
        ) -> Self {
            AsyncState {
                isolate_for_async_waiters: isolate,
                task_runner,
                backing_store,
                promise,
                native_context,
                timeout_time: Mutex::new(None),
                timeout_task_id: AtomicU32::new(0),
            }
        }
    }

    impl Drop for AsyncState {
        fn drop(&mut self) {
            // Assert that the timeout task was cancelled.
            assert_eq!(0, self.timeout_task_id.load(Ordering::SeqCst));
        }
    }

    #[derive(Debug)]
    pub struct FutexWaitListNode {
        cond_: Condvar,
        prev_: Mutex<Option<*mut FutexWaitListNode>>,
        next_: Mutex<Option<*mut FutexWaitListNode>>,
        wait_location_: *mut std::ffi::c_void,
        waiting_: AtomicBool,
        interrupted_: AtomicBool,
        async_state_: Option<Box<AsyncState>>,
    }

    impl FutexWaitListNode {
        pub fn new_sync() -> Self {
            FutexWaitListNode {
                cond_: Condvar::new(),
                prev_: Mutex::new(None),
                next_: Mutex::new(None),
                wait_location_: ptr::null_mut(),
                waiting_: AtomicBool::new(false),
                interrupted_: AtomicBool::new(false),
                async_state_: None,
            }
        }

        pub fn new_async(
            backing_store: Weak<BackingStore>,
            wait_location: *mut std::ffi::c_void,
            promise_capability: Local<Object>,
            isolate: *mut Isolate,
        ) -> Self {
            let task_runner = unsafe {
                V8::GetCurrentPlatform()
                    .GetForegroundTaskRunner(isolate as *mut v8::Isolate)
            };
            let promise = unsafe {
                let v8_isolate = isolate as *mut v8::Isolate;
                let global = v8::Global::new(v8_isolate, promise_capability);
                Global::from(global)
            };
            let native_context = unsafe {
                let v8_isolate = isolate as *mut v8::Isolate;
                let global = v8::Global::new(
                    v8_isolate,
                    (*isolate).native_context() as *mut NativeContext,
                );
                Global::from(global)
            };
            FutexWaitListNode {
                cond_: Condvar::new(),
                prev_: Mutex::new(None),
                next_: Mutex::new(None),
                wait_location_: wait_location,
                waiting_: AtomicBool::new(true),
                interrupted_: AtomicBool::new(false),
                async_state_: Some(Box::new(AsyncState::new(
                    isolate,
                    task_runner,
                    backing_store,
                    promise,
                    native_context,
                ))),
            }
        }

        pub fn notify_wake(&self) {
            let wait_list = get_wait_list();
            let guard = wait_list.mutex.lock().unwrap();

            self.cond_.notify_one();
            self.interrupted_.store(true, Ordering::SeqCst);
        }

        pub fn is_async(&self) -> bool {
            self.async_state_.is_some()
        }

        pub fn cancel_timeout_task(&self) -> bool {
            if let Some(async_state) = &self.async_state_ {
                if async_state.timeout_task_id.load(Ordering::SeqCst) == 0 {
                    return true;
                }
                let cancelable_task_manager =
                    unsafe { (*async_state.isolate_for_async_waiters).cancelable_task_manager() };
                let return_value = cancelable_task_manager.try_abort(
                    async_state.timeout_task_id.load(Ordering::SeqCst),
                );
                async_state.timeout_task_id.store(0, Ordering::SeqCst);
                return return_value != TryAbortResult::kTaskRunning;
            }
            true
        }
    }

    // A {FutexWaitList} manages all contexts waiting (synchronously or
    // asynchronously) on any address.
    #[derive(Debug)]
    pub struct FutexWaitList {
        mutex: Mutex<()>,
        location_lists_: Mutex<HashMap<*mut std::ffi::c_void, HeadAndTail>>,
        isolate_promises_to_resolve_: Mutex<HashMap<*mut Isolate, HeadAndTail>>,
    }

    impl FutexWaitList {
        pub fn new() -> Self {
            FutexWaitList {
                mutex: Mutex::new(()),
                location_lists_: Mutex::new(HashMap::new()),
                isolate_promises_to_resolve_: Mutex::new(HashMap::new()),
            }
        }

        fn add_node(&self, node: *mut FutexWaitListNode) {
            let mut location_lists = self.location_lists_.lock().unwrap();
            let wait_location = unsafe { (*node).wait_location_ };

            let mut head_and_tail = HeadAndTail {
                head: node,
                tail: node,
            };

            if location_lists.contains_key(&wait_location) {
                let mut existing_head_and_tail = location_lists.get_mut(&wait_location).unwrap();
                unsafe {
                    (*existing_head_and_tail.tail).next_ = Mutex::new(Some(node));
                    (*node).prev_ = Mutex::new(Some(existing_head_and_tail.tail));
                }
                existing_head_and_tail.tail = node;
            } else {
                location_lists.insert(wait_location, head_and_tail);
            }

            self.verify();
        }

        fn remove_node(&self, node: *mut FutexWaitListNode) {
            let mut location_lists = self.location_lists_.lock().unwrap();
            let wait_location = unsafe { (*node).wait_location_ };

            if let Some(mut head_and_tail) = location_lists.get_mut(&wait_location) {
                let prev = unsafe { (*node).prev_.lock().unwrap().take() };
                let next = unsafe { (*node).next_.lock().unwrap().take() };

                match (prev, next) {
                    (None, None) => {
                        location_lists.remove(&wait_location);
                    }
                    (Some(prev_ptr), Some(next_ptr)) => unsafe {
                        (*prev_ptr).next_ = Mutex::new(Some(next_ptr));
                        (*next_ptr).prev_ = Mutex::new(Some(prev_ptr));
                    },
                    (Some(prev_ptr), None) => unsafe {
                        (*prev_ptr).next_ = Mutex::new(None);
                        head_and_tail.tail = prev_ptr;
                    },
                    (None, Some(next_ptr)) => unsafe {
                        (*next_ptr).prev_ = Mutex::new(None);
                        head_and_tail.head = next_ptr;
                    },
                }
            }

            self.verify();
        }

        pub fn to_wait_location(array_buffer: Tagged<JSArrayBuffer>, addr: usize) -> *mut std::ffi::c_void {
            assert!(addr < array_buffer.get_byte_length());
            assert_eq!(
                array_buffer.backing_store(),
                array_buffer.get_backing_store().buffer_start()
            );

            let backing_store = array_buffer.backing_store() as *mut u8;
            unsafe { backing_store.add(addr) as *mut std::ffi::c_void }
        }

        fn delete_async_waiter_node(node: *mut FutexWaitListNode) -> Option<*mut FutexWaitListNode> {
            unsafe {
                if let Some(async_state) = &(*node).async_state_ {
                    assert!(!async_state.isolate_for_async_waiters.is_null());
                    let next = (*node).next_.lock().unwrap().take();

                    if let Some(next_node) = next {
                        (*next_node).prev_ = (*node).prev_.lock().unwrap().take().into();
                    }
                    if let Some(prev_node) = (*node).prev_.lock().unwrap().take() {
                        (*prev_node).next_ = Mutex::new(next);
                    }

                    drop(Box::from_raw(node));
                    next
                } else {
                    None
                }
            }
        }

        fn delete_nodes_for_isolate(
            isolate: *mut Isolate,
            head: &mut *mut FutexWaitListNode,
            tail: &mut *mut FutexWaitListNode,
        ) {
            let mut current = *head;
            let mut new_head: Option<*mut FutexWaitListNode> = None;
            let mut new_tail: Option<*mut FutexWaitListNode> = None;

            while let Some(mut node) = NonNull::new(current) {
                let node_ref = unsafe { node.as_mut() };
                let next = unsafe { (*node_ref).next_.lock().unwrap().take() };

                if let Some(async_state) = &node_ref.async_state_ {
                    if async_state.isolate_for_async_waiters == isolate {
                        unsafe {
                            (*node).async_state_.as_ref().unwrap().timeout_task_id.store(0, Ordering::SeqCst);
                        }
                        FutexWaitList::delete_async_waiter_node(current);
                    } else {
                        if new_head.is_none() {
                            new_head = Some(current);
                        }
                        new_tail = Some(current);
                    }
                } else {
                    if new_head.is_none() {
                        new_head = Some(current);
                    }
                    new_tail = Some(current);
                }

                current = next.unwrap_or(ptr::null_mut());
            }

            *head = new_head.unwrap_or(ptr::null_mut());
            *tail = new_tail.unwrap_or(ptr::null_mut());
        }

        fn verify(&self) {
            #[cfg(debug_assertions)]
            {
                let location_lists = self.location_lists_.lock().unwrap();
                for (_addr, head_and_tail) in location_lists.iter() {
                    let head = head_and_tail.head;
                    let tail = head_and_tail.tail;
                    let mut current = head;
                    while let Some(node) = NonNull::new(current) {
                        let node_ref = unsafe { node.as_ref() };

                        if let Some(next_ptr) =
                            unsafe { (*node_ref).next_.lock().unwrap().as_ref() }
                        {
                            assert_ne!(current, tail);
                            assert_eq!(current, unsafe {
                                (*(*next_ptr)).prev_.lock().unwrap().unwrap()
                            });
                        } else {
                            assert_eq!(current, tail);
                        }

                        if let Some(prev_ptr) =
                            unsafe { (*node_ref).prev_.lock().unwrap().as_ref() }
                        {
                            assert_ne!(current, head);
                            assert_eq!(current, unsafe {
                                (*(*prev_ptr)).next_.lock().unwrap().unwrap()
                            });
                        } else {
                            assert_eq!(current, head);
                        }

                        assert!(FutexWaitList::node_is_on_list(node_ref, head));
                        current = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                    }
                }
            }
        }

        fn node_is_on_list(node: &FutexWaitListNode, head: *mut FutexWaitListNode) -> bool {
            let mut current = head;
            while let Some(n) = NonNull::new(current) {
                if n.as_ptr() as *const _ == node as *const _ {
                    return true;
                }
                current = unsafe { (*NonNull::from(n).as_ptr()).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
            }
            false
        }
    }

    unsafe impl Sync for FutexWaitList {}
    unsafe impl Send for FutexWaitList {}

    #[derive(Debug)]
    struct HeadAndTail {
        head: *mut FutexWaitListNode,
        tail: *mut FutexWaitListNode,
    }

    lazy_static::lazy_static! {
        static ref GLOBAL_WAIT_LIST: FutexWaitList = FutexWaitList::new();
    }

    fn get_wait_list() -> &'static FutexWaitList {
        &GLOBAL_WAIT_LIST
    }

    #[derive(Debug, PartialEq)]
    enum TryAbortResult {
        kTaskRunning,
        kAborted,
        kNotFound,
    }

    #[derive(Debug)]
    pub struct CancelableTaskManager {}

    impl CancelableTaskManager {
        pub fn new() -> Self {
            CancelableTaskManager {}
        }

        pub fn try_abort(&self, task_id: u32) -> TryAbortResult {
            TryAbortResult::kAborted // Placeholder implementation
        }
    }

    #[derive(Debug)]
    pub struct TaskRunner {}

    impl TaskRunner {
        pub fn new() -> Self {
            TaskRunner {}
        }

        pub fn post_non_nestable_task(&self, task: Box<dyn CancelableTask>) {}

        pub fn post_non_nestable_delayed_task(&self, task: Box<dyn CancelableTask>, delay_seconds: f64) {}
    }

    pub trait CancelableTask: Debug + Send {
        fn run_internal(&mut self);
        fn id(&self) -> u32;
    }

    pub mod futex_emulation_impl {
        use super::*;
        use std::sync::atomic::Ordering;

        const V8_INFINITY: f64 = f64::INFINITY;

        pub enum WaitMode {
            kSync = 0,
            kAsync,
        }

        pub enum CallType {
            kIsNotWasm = 0,
            kIsWasm,
        }

        pub struct AllStatic {}

        impl AllStatic {
            pub const KWAKEALL: u32 = u32::MAX;

            pub fn wait_js32(
                isolate: *mut Isolate,
                mode: WaitMode,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: i32,
                rel_timeout_ms: f64,
            ) -> Tagged<Object> {
                let res = Self::wait::<i32>(isolate, mode, array_buffer, addr, value, rel_timeout_ms);
                Self::wait_js_translate_return(isolate, res)
            }

            pub fn wait_js64(
                isolate: *mut Isolate,
                mode: WaitMode,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: i64,
                rel_timeout_ms: f64,
            ) -> Tagged<Object> {
                let res = Self::wait::<i64>(isolate, mode, array_buffer, addr, value, rel_timeout_ms);
                Self::wait_js_translate_return(isolate, res)
            }

            pub fn wait_wasm32(
                isolate: *mut Isolate,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: i32,
                rel_timeout_ns: i64,
            ) -> Tagged<Object> {
                Self::wait::<i32>(
                    isolate,
                    WaitMode::kSync,
                    array_buffer,
                    addr,
                    value,
                    rel_timeout_ns >= 0.0,
                    rel_timeout_ns,
                    CallType::kIsWasm,
                )
            }

            pub fn wait_wasm64(
                isolate: *mut Isolate,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: i64,
                rel_timeout_ns: i64,
            ) -> Tagged<Object> {
                Self::wait::<i64>(
                    isolate,
                    WaitMode::kSync,
                    array_buffer,
                    addr,
                    value,
                    rel_timeout_ns >= 0,
                    rel_timeout_ns,
                    CallType::kIsWasm,
                )
            }

            pub fn wake(array_buffer: Tagged<JSArrayBuffer>, addr: usize, num_waiters_to_wake: u32) -> i32 {
                let wait_location = FutexWaitList::to_wait_location(array_buffer, addr);
                Self::wake_internal(wait_location, num_waiters_to_wake)
            }

            pub fn wake_internal(addr: *mut std::ffi::c_void, num_waiters_to_wake: u32) -> i32 {
                let mut num_waiters_woken = 0;
                let wait_list = get_wait_list();
                let _guard = wait_list.mutex.lock().unwrap();

                let mut location_lists = wait_list.location_lists_.lock().unwrap();
                if !location_lists.contains_key(&addr) {
                    return num_waiters_woken;
                }

                let head_and_tail = location_lists.get_mut(&addr).unwrap();
                let mut node = head_and_tail.head;

                while let Some(mut node_ptr) = NonNull::new(node) {
                    let node_ref = unsafe { node_ptr.as_mut() };

                    if !node_ref.waiting_.load(Ordering::SeqCst) {
                        node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                        continue;
                    }

                    let matching_backing_store = if node_ref.is_async() {
                        if let Some(async_state) = &node_ref.async_state_ {
                            if let Some(backing_store) = async_state.backing_store.upgrade() {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        true
                    };

                    if matching_backing_store {
                        node_ref.waiting_.store(false, Ordering::SeqCst);
                        let next_node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };

                        if node_ref.is_async() {
                            Self::notify_async_waiter(node_ref);
                        } else {
                            node_ref.cond_.notify_one();
                        }

                        node = next_node;

                        if num_waiters_to_wake != AllStatic::KWAKEALL {
                            num_waiters_to_wake -= 1;
                        }

                        num_waiters_woken += 1;

                        continue;
                    }

                    let mut delete_this_node = false;

                    if node_ref.is_async() {
                        if let Some(async_state) = &node_ref.async_state_ {
                            if let Ok(timeout_time_guard) = async_state.timeout_time.try_lock() {
                                if timeout_time_guard.is_none() {
                                    if async_state.timeout_task_id.load(Ordering::SeqCst) == 0 {
                                        delete_this_node = true;
                                    }
                                }
                            }

                            if unsafe { (*async_state.isolate_for_async_waiters).is_dead() } {
                                if node_ref.cancel_timeout_task() {
                                    delete_this_node = true;
                                }
                            }
                        }
                    }

                    let next_node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };

                    if delete_this_node {
                        wait_list.remove_node(node_ref);
                        drop(unsafe { Box::from_raw(node_ref) });
                    }

                    node = next_node;
                }

                num_waiters_woken
            }

            pub fn isolate_deinit(isolate: *mut Isolate) {
                let wait_list = get_wait_list();
                let guard = wait_list.mutex.lock().unwrap();

                let mut location_lists = wait_list.location_lists_.lock().unwrap();
                let mut iter = location_lists.iter_mut();

                while let Some((&_addr, head_and_tail)) = iter.next() {
                    let head = &mut head_and_tail.head;
                    let tail = &mut head_and_tail.tail;
                    FutexWaitList::delete_nodes_for_isolate(isolate, head, tail);

                    if head.is_null() && tail.is_null() {
                        iter.remove();
                    }
                }

                let mut isolate_promises_to_resolve =
                    wait_list.isolate_promises_to_resolve_.lock().unwrap();
                if let Some(head_and_tail) = isolate_promises_to_resolve.remove(&isolate) {
                    let mut node = head_and_tail.head;
                    while let Some(mut node_ptr) = NonNull::new(node) {
                        let node_ref = unsafe { node_ptr.as_mut() };

                        if let Some(async_state) = &node_ref.async_state_ {
                            unsafe {
                                (*async_state).timeout_task_id.store(0, Ordering::SeqCst);
                            }
                        }

                        let next_node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };

                        FutexWaitList::delete_async_waiter_node(node);
                        node = next_node;
                    }
                }

                wait_list.verify();
            }

            pub fn num_waiters_for_testing(array_buffer: Tagged<JSArrayBuffer>, addr: usize) -> i32 {
                let wait_location = FutexWaitList::to_wait_location(array_buffer, addr);
                let wait_list = get_wait_list();
                let guard = wait_list.mutex.lock().unwrap();

                let mut num_waiters = 0;
                let location_lists = wait_list.location_lists_.lock().unwrap();
                if !location_lists.contains_key(&wait_location) {
                    return num_waiters;
                }

                let head_and_tail = location_lists.get(&wait_location).unwrap();
                let mut node = head_and_tail.head;

                while let Some(mut node_ptr) = NonNull::new(node) {
                    let node_ref = unsafe { node_ptr.as_mut() };

                    if !node_ref.waiting_.load(Ordering::SeqCst) {
                        node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                        continue;
                    }

                    if node_ref.is_async() {
                        if let Some(async_state) = &node_ref.async_state_ {
                            if async_state.backing_store.upgrade().is_none() {
                                node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                                continue;
                            }
                        }
                    }

                    num_waiters += 1;
                    node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                }

                num_waiters
            }

            pub fn num_unresolved_async_promises_for_testing(
                array_buffer: Tagged<JSArrayBuffer>,
                addr: usize,
            ) -> i32 {
                let wait_location = FutexWaitList::to_wait_location(array_buffer, addr);
                let wait_list = get_wait_list();
                let guard = wait_list.mutex.lock().unwrap();

                let mut num_waiters = 0;
                let isolate_promises_to_resolve =
                    wait_list.isolate_promises_to_resolve_.lock().unwrap();

                for (_isolate, head_and_tail) in isolate_promises_to_resolve.iter() {
                    let mut node = head_and_tail.head;

                    while let Some(mut node_ptr) = NonNull::new(node) {
                        let node_ref = unsafe { node_ptr.as_mut() };

                        if !node_ref.is_async() {
                            node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                            continue;
                        }

                        if node_ref.waiting_.load(Ordering::SeqCst) {
                            node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                            continue;
                        }

                        if node_ref.async_state_.as_ref().map_or(false, |async_state| {
                            async_state.backing_store.upgrade().is_none()
                        }) {
                            node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                            continue;
                        }

                        num_waiters += 1;
                        node = unsafe { (*node_ref).next_.lock().unwrap().unwrap_or(ptr::null_mut()) };
                    }
                }

                num_waiters
            }

            fn wait<T>(
                isolate: *mut Isolate,
                mode: WaitMode,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: T,
                rel_timeout_ms: f64,
            ) -> Tagged<Object>
            where
                T: PartialEq + Copy + Debug + Send + Sync,
            {
                assert!(addr < array_buffer.get_byte_length());

                let mut use_timeout = rel_timeout_ms != V8_INFINITY;
                let mut rel_timeout_ns = -1.0;

                if use_timeout {
                    let timeout_ns = rel_timeout_ms
                        * 1000.0
                        * 1000.0;

                    if timeout_ns > i64::MAX as f64 {
                        use_timeout = false;
                    } else {
                        rel_timeout_ns = timeout_ns;
                    }
                }

                Self::wait::<T>(
                    isolate,
                    mode,
                    array_buffer,
                    addr,
                    value,
                    use_timeout,
                    rel_timeout_ns as i64,
                    CallType::kIsNotWasm,
                )
            }

            fn wait<T>(
                isolate: *mut Isolate,
                mode: WaitMode,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: T,
                use_timeout: bool,
                rel_timeout_ns: i64,
                call_type: CallType,
            ) -> Tagged<Object>
            where
                T: PartialEq + Copy + Debug + Send + Sync,
            {
                match mode {
                    WaitMode::kSync => {
                        Self::wait_sync::<T>(isolate, array_buffer, addr, value, use_timeout, rel_timeout_ns, call_type)
                    }
                    WaitMode::kAsync => {
                        Self::wait_async::<T>(isolate, array_buffer, addr, value, use_timeout, rel_timeout_ns, call_type)
                    }
                }
            }

            fn wait_sync<T>(
                isolate: *mut Isolate,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: T,
                use_timeout: bool,
                rel_timeout_ns: i64,
                call_type: CallType,
            ) -> Tagged<Object>
            where
                T: PartialEq + Copy + Debug + Send + Sync,
            {
                let wait_list = get_wait_list();
                let node = unsafe { (*isolate).futex_wait_list_node() };
                let wait_location = FutexWaitList::to_wait_location(array_buffer, addr);

                let timeout_time = if use_timeout {
                    Some(Instant::now() + Duration::from_nanos(rel_timeout_ns as u64))
                } else {
                    None
                };

                loop {
                    let _guard = wait_list.mutex.lock().unwrap();

                    let p = wait_location as *mut AtomicI64;
                    let loaded_value = unsafe { (*p).load(Ordering::SeqCst) };

                    if loaded_value != value as i64 {
                        return Tagged::Smi(WaitReturnValue::kNotEqualValue as i32);
                    }

                    unsafe { (*node).wait_location_ = wait_location };
                    unsafe { (*node).waiting_.store(true, Ordering::SeqCst) };
                    wait_list.add_node(node);

                    loop {
                        if unsafe { (*node).interrupted_.load(Ordering::SeqCst) } {
                            unsafe { (*node).interrupted_.store(false, Ordering::SeqCst) };
                            continue;
                        }

                        if !unsafe { (*node).waiting_.load(Ordering::SeqCst) } {
                            return Tagged::Smi(WaitReturnValue::kOk as i32);
                        }

                        if let Some(time) = timeout_time {
                            if Instant::now() >= time {
                                return Tagged::Smi(WaitReturnValue::kTimedOut as i32);
                            }

                            let time_until_timeout = time.duration_since(Instant::now());
                            let (mutex_guard, wait_result) = unsafe {
                                (*node).cond_.wait_timeout(_guard, time_until_timeout).unwrap()
                            };
                        } else {
                            let mutex_guard = unsafe { (*node).cond_.wait(_guard).unwrap() };
                        }
                        break;
                    }

                    unsafe { (*node).waiting_.store(false, Ordering::SeqCst) };
                    wait_list.remove_node(node);

                    return Tagged::Smi(WaitReturnValue::kOk as i32);
                }
            }

            fn wait_async<T>(
                isolate: *mut Isolate,
                array_buffer: Local<JSArrayBuffer>,
                addr: usize,
                value: T,
                use_timeout: bool,
                rel_timeout_ns: i64,
                call_type: CallType,
            ) -> Tagged<Object>
            where
                T: PartialEq + Copy + Debug + Send + Sync,
            {
                Tagged::<Object>::null()
            }

            fn resolve_async_waiter_promises(isolate: *mut Isolate) {}

            fn resolve_async_waiter_promise(node: *mut FutexWaitListNode) {}

            fn handle_async_waiter_timeout(node: *mut FutexWaitListNode) {}

            fn notify_async_waiter(node: *mut FutexWaitListNode) {
                let wait_list = get_wait_list();
                let

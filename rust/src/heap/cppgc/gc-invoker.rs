// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};

pub mod cppgc {
    pub mod common {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum EmbedderStackState {
            kNoHeapPointers,
            kMayContainHeapPointers,
        }
    }

    pub mod platform {
        pub trait TaskRunner {
            fn post_non_nestable_task(&self, task: Box<dyn Task>);
            fn non_nestable_tasks_enabled(&self) -> bool;
        }

        pub trait Platform {
            fn get_foreground_task_runner(&self) -> Option<&dyn TaskRunner>;
        }
    }

    pub trait Task {
        fn run(self: Box<Self>);
    }

    pub mod internal {
        pub trait Finalizable {}
    }

    pub mod heap {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MarkingType {
            kAtomic,
            kIncremental,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum StackSupport {
            kSupportsConservativeStackScan,
            kRequiresPreciseStackScan,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum StackState {
            kNoHeapPointers,
            kMayContainHeapPointers,
        }
    }
}

pub struct GCConfig {
    pub marking_type: cppgc::heap::MarkingType,
    pub stack_state: cppgc::heap::StackState,
}

pub trait GarbageCollector {
    fn collect_garbage(&mut self, config: GCConfig);
    fn start_incremental_garbage_collection(&mut self, config: GCConfig);
    fn epoch(&self) -> usize;
    fn overridden_stack_state(&self) -> Option<cppgc::common::EmbedderStackState>;
    fn set_override_stack_state(&mut self, state: cppgc::common::EmbedderStackState);
    fn clear_overridden_stack_state(&mut self);
}

pub mod internal {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct GCTaskHandle {
        canceled: Rc<Cell<bool>>,
    }

    impl GCTaskHandle {
        fn new() -> Self {
            GCTaskHandle {
                canceled: Rc::new(Cell::new(false)),
            }
        }

        fn is_canceled(&self) -> bool {
            self.canceled.get()
        }

        fn cancel(&self) {
            self.canceled.set(true);
        }

        fn clone(&self) -> Self {
            GCTaskHandle {
                canceled: self.canceled.clone(),
            }
        }
    }

    struct GCTask {
        collector: *mut dyn GarbageCollector,
        config: GCConfig,
        handle: GCTaskHandle,
        saved_epoch: usize,
    }

    impl GCTask {
        fn new(collector: *mut dyn GarbageCollector, config: GCConfig) -> Self {
            // SAFETY: `collector` is a raw pointer that will be used in a single-threaded context.
            // Ensure that it is valid for the lifetime of the `GCTask`.
            unsafe {
                GCTask {
                    collector,
                    config,
                    handle: GCTaskHandle::new(),
                    saved_epoch: (*collector).epoch(),
                }
            }
        }

        fn get_handle(&self) -> GCTaskHandle {
            self.handle.clone()
        }
    }

    impl cppgc::Task for GCTask {
        fn run(mut self: Box<Self>) {
            // SAFETY: `self.collector` is a raw pointer that was valid when `GCTask` was created.
            // Check if the task was cancelled or epoch has changed before dereferencing.
            unsafe {
                if self.handle.is_canceled() || ((*self.collector).epoch() != self.saved_epoch) {
                    return;
                }

                (*self.collector).set_override_stack_state(cppgc::common::EmbedderStackState::kNoHeapPointers);
                (*self.collector).collect_garbage(self.config);
                (*self.collector).clear_overridden_stack_state();
                self.handle.cancel();
            }
        }
    }

    pub struct GCInvoker {
        impl_: Box<GCInvokerImpl>,
    }

    impl GCInvoker {
        pub fn new(
            collector: *mut dyn GarbageCollector,
            platform: *mut dyn cppgc::platform::Platform,
            stack_support: cppgc::heap::StackSupport,
        ) -> Self {
            GCInvoker {
                impl_: Box::new(GCInvokerImpl::new(collector, platform, stack_support)),
            }
        }

        pub fn collect_garbage(&mut self, config: GCConfig) {
            self.impl_.collect_garbage(config);
        }

        pub fn start_incremental_garbage_collection(&mut self, config: GCConfig) {
            self.impl_.start_incremental_garbage_collection(config);
        }

        pub fn epoch(&self) -> usize {
            self.impl_.epoch()
        }

        pub fn overridden_stack_state(&self) -> Option<cppgc::common::EmbedderStackState> {
            self.impl_.overridden_stack_state()
        }

        pub fn set_override_stack_state(&mut self, state: cppgc::common::EmbedderStackState) {
            self.impl_.set_override_stack_state(state)
        }

        pub fn clear_overridden_stack_state(&mut self) {
            self.impl_.clear_overridden_stack_state()
        }

        // The following code cannot be directly translated:
        // #ifdef V8_ENABLE_ALLOCATION_TIMEOUT
        // std::optional<int> GCInvoker::UpdateAllocationTimeout() {
        //   return impl_->UpdateAllocationTimeout();
        // }
        // #endif  // V8_ENABLE_ALLOCATION_TIMEOUT
        // This section requires conditional compilation based on a macro V8_ENABLE_ALLOCATION_TIMEOUT,
        // which is not available in the current context.
    }

    struct GCInvokerImpl {
        collector_: *mut dyn GarbageCollector,
        platform_: *mut dyn cppgc::platform::Platform,
        stack_support_: cppgc::heap::StackSupport,
        gc_task_handle_: Option<GCTaskHandle>,
    }

    impl GCInvokerImpl {
        fn new(
            collector: *mut dyn GarbageCollector,
            platform: *mut dyn cppgc::platform::Platform,
            stack_support: cppgc::heap::StackSupport,
        ) -> Self {
            GCInvokerImpl {
                collector_: collector,
                platform_: platform,
                stack_support_: stack_support,
                gc_task_handle_: None,
            }
        }

        fn collect_garbage(&mut self, config: GCConfig) {
            assert_eq!(config.marking_type, cppgc::heap::MarkingType::kAtomic);
            // SAFETY: The raw pointer `self.platform_` remains valid for the duration of this call.
            unsafe {
                if (config.stack_state == cppgc::heap::StackState::kNoHeapPointers)
                    || (self.stack_support_
                        == cppgc::heap::StackSupport::kSupportsConservativeStackScan)
                {
                    // SAFETY: `self.collector_` is a raw pointer that remains valid throughout this call.
                    (*self.collector_).collect_garbage(config);
                } else if let Some(task_runner) = (*self.platform_).get_foreground_task_runner() {
                    if task_runner.non_nestable_tasks_enabled() {
                        if self.gc_task_handle_.is_none() {
                            // Force a precise GC since it will run in a non-nestable task.
                            let mut config = config;
                            config.stack_state = cppgc::heap::StackState::kNoHeapPointers;
                            assert_ne!(
                                cppgc::heap::StackSupport::kSupportsConservativeStackScan,
                                self.stack_support_
                            );

                            let task = GCTask::new(self.collector_, config);
                            let handle = task.get_handle();
                            task_runner.post_non_nestable_task(Box::new(task));
                            self.gc_task_handle_ = Some(handle);
                        }
                    }
                }
            }
        }

        fn start_incremental_garbage_collection(&mut self, config: GCConfig) {
            assert_ne!(config.marking_type, cppgc::heap::MarkingType::kAtomic);
            // SAFETY: `self.platform_` is assumed to be valid for the duration of this function.
            unsafe {
                if (self.stack_support_
                    != cppgc::heap::StackSupport::kSupportsConservativeStackScan)
                    && ((*self.platform_).get_foreground_task_runner().is_none()
                        || !(*self.platform_)
                            .get_foreground_task_runner()
                            .unwrap()
                            .non_nestable_tasks_enabled())
                {
                    // In this configuration the GC finalization can only be triggered through
                    // ForceGarbageCollectionSlow. If incremental GC is started, there is no
                    // way to know how long it will remain enabled (and the write barrier with
                    // it). For that reason, we do not support running incremental GCs in this
                    // configuration.
                    return;
                }
                // No need to postpone starting incremental GC since the stack is not scanned
                // until GC finalization.
                // SAFETY: `self.collector_` is assumed to be valid for the duration of this function.
                (*self.collector_).start_incremental_garbage_collection(config);
            }
        }

        fn epoch(&self) -> usize {
            // SAFETY: `self.collector_` is assumed to be valid for the duration of this function.
            unsafe { (*self.collector_).epoch() }
        }

        fn overridden_stack_state(&self) -> Option<cppgc::common::EmbedderStackState> {
            // SAFETY: `self.collector_` is assumed to be valid for the duration of this function.
            unsafe { (*self.collector_).overridden_stack_state() }
        }

        fn set_override_stack_state(&mut self, state: cppgc::common::EmbedderStackState) {
            // SAFETY: `self.collector_` is assumed to be valid for the duration of this function.
            unsafe { (*self.collector_).set_override_stack_state(state) }
        }

        fn clear_overridden_stack_state(&mut self) {
            // SAFETY: `self.collector_` is assumed to be valid for the duration of this function.
            unsafe { (*self.collector_).clear_overridden_stack_state() }
        }
    }

    impl Drop for GCInvokerImpl {
        fn drop(&mut self) {
            if let Some(handle) = &self.gc_task_handle_ {
                handle.cancel();
            }
        }
    }
}
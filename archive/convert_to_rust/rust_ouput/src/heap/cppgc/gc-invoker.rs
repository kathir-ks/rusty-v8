// Converted from V8 C++ source files:
// Header: gc-invoker.h
// Implementation: gc-invoker.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/cppgc/gc-invoker.h
pub mod gc_invoker {
    use crate::heap::cppgc::garbage_collector::GCConfig;
    use crate::heap::cppgc::garbage_collector::GarbageCollector;
    use std::optional::Option;
    use crate::include::cppgc::heap::StackSupport;
    use crate::include::cppgc::common::EmbedderStackState;

    pub struct GCInvoker {
        impl_: Box<GCInvokerImpl>,
    }

    impl GCInvoker {
        pub fn new(collector: *mut dyn GarbageCollector, platform: *mut dyn Platform, stack_support: StackSupport) -> Self {
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

        pub fn overridden_stack_state(&self) -> Option<EmbedderStackState> {
            self.impl_.overridden_stack_state()
        }

        pub fn set_override_stack_state(&mut self, state: EmbedderStackState) {
            self.impl_.set_override_stack_state(state);
        }

        pub fn clear_overridden_stack_state(&mut self) {
            self.impl_.clear_overridden_stack_state();
        }

        #[cfg(feature = "v8_enable_allocation_timeout")]
        pub fn update_allocation_timeout(&mut self) -> Option<i32> {
            self.impl_.update_allocation_timeout()
        }
    }

    impl Drop for GCInvoker {
        fn drop(&mut self) {}
    }

    struct GCInvokerImpl {
        collector_: *mut dyn GarbageCollector,
        platform_: *mut dyn Platform,
        stack_support_: StackSupport,
        gc_task_handle_: Option<SingleThreadedHandle>,
    }

    impl GCInvokerImpl {
        fn new(collector: *mut dyn GarbageCollector, platform: *mut dyn Platform, stack_support: StackSupport) -> Self {
            GCInvokerImpl {
                collector_: collector,
                platform_: platform,
                stack_support_: stack_support,
                gc_task_handle_: None,
            }
        }

        fn collect_garbage(&mut self, config: GCConfig) {
            if config.stack_state == crate::heap::cppgc::marker::StackState::kNoHeapPointers ||
                self.stack_support_ == StackSupport::kSupportsConservativeStackScan {
                    unsafe { (*self.collector_).CollectGarbage(config); }
            } else {
                unsafe {
                if let Some(task_runner) = (*self.platform_).GetForegroundTaskRunner() {
                    if task_runner.NonNestableTasksEnabled() {
                        if self.gc_task_handle_.is_none() {
                            let mut config = config;
                            config.stack_state = crate::heap::cppgc::marker::StackState::kNoHeapPointers;
                            self.gc_task_handle_ = Some(GCTask::post(self.collector_, (*self.platform_).GetForegroundTaskRunner().unwrap(), config));
                        }
                    }
                }
                }
            }
        }

        fn start_incremental_garbage_collection(&mut self, config: GCConfig) {
            if self.stack_support_ != StackSupport::kSupportsConservativeStackScan &&
                unsafe {
                 if let Some(task_runner) = (*self.platform_).GetForegroundTaskRunner() {
                     !task_runner.NonNestableTasksEnabled()
                 } else {
                     true
                 }
                }{
                return;
            }
            unsafe { (*self.collector_).StartIncrementalGarbageCollection(config) };
        }

        fn epoch(&self) -> usize {
            unsafe { (*self.collector_).epoch() }
        }

        fn overridden_stack_state(&self) -> Option<EmbedderStackState> {
            unsafe { (*self.collector_).overridden_stack_state() }
        }

        fn set_override_stack_state(&mut self, state: EmbedderStackState) {
            unsafe { (*self.collector_).set_override_stack_state(state) };
        }

        fn clear_overridden_stack_state(&mut self) {
            unsafe { (*self.collector_).clear_overridden_stack_state() };
        }

        #[cfg(feature = "v8_enable_allocation_timeout")]
        fn update_allocation_timeout(&mut self) -> Option<i32> {
            None
        }
    }

    struct GCTask {
        collector_: *mut dyn GarbageCollector,
        config_: GCConfig,
        handle_: SingleThreadedHandle,
        saved_epoch_: usize,
    }

    impl GCTask {
        fn post(collector: *mut dyn GarbageCollector, runner: &dyn TaskRunner, config: GCConfig) -> SingleThreadedHandle {
            let mut task = Box::new(GCTask::new(collector, config));
            let handle = task.get_handle();
            runner.post_non_nestable_task(task);
            handle
        }

        fn new(collector: *mut dyn GarbageCollector, config: GCConfig) -> Self {
            let saved_epoch_ = unsafe { (*collector).epoch() };
            GCTask {
                collector_: collector,
                config_: config,
                handle_: SingleThreadedHandle::new(),
                saved_epoch_: saved_epoch_,
            }
        }

        fn run(&mut self) {
            unsafe {
            if self.handle_.is_canceled() || (*self.collector_).epoch() != self.saved_epoch_ {
                return;
            }

            (*self.collector_).set_override_stack_state(EmbedderStackState::kNoHeapPointers);
            (*self.collector_).CollectGarbage(self.config_);
            (*self.collector_).clear_overridden_stack_state();
            self.handle_.cancel();
            }
        }

        fn get_handle(&self) -> SingleThreadedHandle {
            self.handle_
        }
    }

    pub trait Platform {
        fn GetForegroundTaskRunner(&mut self) -> Option<&dyn TaskRunner>;
    }

    pub trait TaskRunner {
        fn NonNestableTasksEnabled(&self) -> bool;
        fn post_non_nestable_task(&self, task: Box<GCTask>);
    }

    #[derive(Clone, Copy)]
    pub struct SingleThreadedHandle {
        canceled: bool,
    }

    impl SingleThreadedHandle {
        fn new() -> Self {
            SingleThreadedHandle { canceled: false }
        }

        fn cancel(&mut self) {
            self.canceled = true;
        }

        fn is_canceled(&self) -> bool {
            self.canceled
        }
    }
}

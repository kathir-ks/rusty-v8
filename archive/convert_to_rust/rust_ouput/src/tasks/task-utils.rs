// Converted from V8 C++ source files:
// Header: task-utils.h
// Implementation: task-utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod task_utils {
    use std::boxed::Box;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct Isolate {}

    pub struct CancelableTaskManager {}

    pub trait Task {
        fn run(&self);
    }

    pub trait IdleTask {
        fn run(&self, deadline_in_seconds: f64);
    }

    pub struct CancelableTask {
        manager: Option<Rc<CancelableTaskManager>>,
        isolate: Option<Rc<Isolate>>,
    }

    impl CancelableTask {
        fn new(isolate: Option<Rc<Isolate>>, manager: Option<Rc<CancelableTaskManager>>) -> Self {
            CancelableTask {
                manager,
                isolate,
            }
        }

        fn from_isolate(isolate: Rc<Isolate>) -> Self {
            CancelableTask {
                manager: None,
                isolate: Some(isolate),
            }
        }

        fn from_manager(manager: Rc<CancelableTaskManager>) -> Self {
            CancelableTask {
                manager: Some(manager),
                isolate: None,
            }
        }

        fn run_internal(&self) {}
    }

    pub struct CancelableIdleTask {
        manager: Option<Rc<CancelableTaskManager>>,
        isolate: Option<Rc<Isolate>>,
    }

    impl CancelableIdleTask {
        fn new(isolate: Option<Rc<Isolate>>, manager: Option<Rc<CancelableTaskManager>>) -> Self {
            CancelableIdleTask {
                manager,
                isolate,
            }
        }

        fn from_isolate(isolate: Rc<Isolate>) -> Self {
            CancelableIdleTask {
                manager: None,
                isolate: Some(isolate),
            }
        }

        fn from_manager(manager: Rc<CancelableTaskManager>) -> Self {
            CancelableIdleTask {
                manager: Some(manager),
                isolate: None,
            }
        }

        fn run_internal(&self, deadline_in_seconds: f64) {}
    }

    struct CancelableFuncTask {
        base: CancelableTask,
        func: Box<dyn Fn()>,
    }

    impl CancelableFuncTask {
        fn new(isolate: Option<Rc<Isolate>>, manager: Option<Rc<CancelableTaskManager>>, func: Box<dyn Fn()>) -> Self {
            let base = match (isolate, manager) {
                (Some(isolate), None) => CancelableTask::new(Some(isolate), None),
                (None, Some(manager)) => CancelableTask::new(None, Some(manager)),
                _ => panic!("Must provide either an isolate or a manager"),
            };

            CancelableFuncTask {
                base,
                func,
            }
        }
    }

    impl Task for CancelableFuncTask {
        fn run(&self) {
            (self.func)();
        }
    }

    struct CancelableIdleFuncTask {
        base: CancelableIdleTask,
        func: Box<dyn Fn(f64)>,
    }

    impl CancelableIdleFuncTask {
        fn new(isolate: Option<Rc<Isolate>>, manager: Option<Rc<CancelableTaskManager>>, func: Box<dyn Fn(f64)>) -> Self {
            let base = match (isolate, manager) {
                (Some(isolate), None) => CancelableIdleTask::new(Some(isolate), None),
                (None, Some(manager)) => CancelableIdleTask::new(None, Some(manager)),
                _ => panic!("Must provide either an isolate or a manager"),
            };

            CancelableIdleFuncTask {
                base,
                func,
            }
        }
    }

    impl IdleTask for CancelableIdleFuncTask {
        fn run(&self, deadline_in_seconds: f64) {
            (self.func)(deadline_in_seconds);
        }
    }

    pub fn make_cancelable_task(
        isolate: &Rc<Isolate>,
        func: Box<dyn Fn()>,
    ) -> Box<dyn Task> {
        Box::new(CancelableFuncTask::new(Some(Rc::clone(isolate)), None, func))
    }

    pub fn make_cancelable_task_manager(
        manager: &Rc<CancelableTaskManager>,
        func: Box<dyn Fn()>,
    ) -> Box<dyn Task> {
        Box::new(CancelableFuncTask::new(None, Some(Rc::clone(manager)), func))
    }

    pub fn make_cancelable_idle_task(
        isolate: &Rc<Isolate>,
        func: Box<dyn Fn(f64)>,
    ) -> Box<dyn IdleTask> {
        Box::new(CancelableIdleFuncTask::new(Some(Rc::clone(isolate)), None, func))
    }

    pub fn make_cancelable_idle_task_manager(
        manager: &Rc<CancelableTaskManager>,
        func: Box<dyn Fn(f64)>,
    ) -> Box<dyn IdleTask> {
        Box::new(CancelableIdleFuncTask::new(None, Some(Rc::clone(manager)), func))
    }
}

// Converted from V8 C++ source files:
// Header: sampler.h
// Implementation: sampler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod sampler {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, RecursiveMutex, RecursiveMutexGuard,
    };
    use std::{
        collections::HashMap,
        sync::atomic::AtomicU32,
        thread::{self, ThreadId},
    };

    use crate::execution::isolate::Isolate;

    pub struct RegisterState {
        pub pc: *mut std::ffi::c_void,
        pub sp: *mut std::ffi::c_void,
        pub fp: *mut std::ffi::c_void,
        pub lr: *mut std::ffi::c_void,
    }

    pub trait SampleStack {
        fn sample_stack(&self, regs: &RegisterState);
    }

    pub struct Sampler {
        isolate_: *mut Isolate,
        active_: AtomicBool,
        record_sample_: AtomicBool,
        data_: Box<PlatformData>,
        is_counting_samples_: AtomicBool,
        js_sample_count_: AtomicU32,
        external_sample_count_: AtomicU32,
    }

    impl Sampler {
        pub const K_MAX_FRAMES_COUNT_LOG2: i32 = 8;
        pub const K_MAX_FRAMES_COUNT: u32 = (1u32 << Self::K_MAX_FRAMES_COUNT_LOG2) - 1;

        pub fn new(isolate: *mut Isolate) -> Self {
            Sampler {
                isolate_: isolate,
                active_: AtomicBool::new(false),
                record_sample_: AtomicBool::new(false),
                data_: PlatformData::new(),
                is_counting_samples_: AtomicBool::new(false),
                js_sample_count_: AtomicU32::new(0),
                external_sample_count_: AtomicU32::new(0),
            }
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn start(&self) {
            if self.active_.load(Ordering::Relaxed) {
                return;
            }
            self.set_active(true);
            #[cfg(feature = "use_signals")]
            {
                SignalHandler::increase_sampler_count();
                SamplerManager::instance().add_sampler(self);
            }
        }

        pub fn stop(&self) {
            #[cfg(feature = "use_signals")]
            {
                SamplerManager::instance().remove_sampler(self);
                SignalHandler::decrease_sampler_count();
            }
            if !self.active_.load(Ordering::Relaxed) {
                return;
            }
            self.set_active(false);
        }

        pub fn is_active(&self) -> bool {
            self.active_.load(Ordering::Relaxed)
        }

        pub fn should_record_sample(&self) -> bool {
            self.record_sample_
                .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        }

        pub fn do_sample(&self) {
            #[cfg(feature = "use_signals")]
            {
                if !SignalHandler::installed() {
                    return;
                }
                self.set_should_record_sample();
                let data = self.platform_data();
                if let Some(thread) = data.vm_tself() {
                   // pthread_kill(data.vm_tself(), SIGPROF);
                }

            }

        }

        pub fn js_sample_count(&self) -> u32 {
            self.js_sample_count_.load(Ordering::Relaxed)
        }

        pub fn external_sample_count(&self) -> u32 {
            self.external_sample_count_.load(Ordering::Relaxed)
        }

        pub fn start_counting_samples(&mut self) {
            self.js_sample_count_.store(0, Ordering::Relaxed);
            self.external_sample_count_.store(0, Ordering::Relaxed);
            self.is_counting_samples_.store(true, Ordering::Relaxed);
        }

        pub fn platform_data(&self) -> &PlatformData {
            &self.data_
        }

        fn set_active(&self, value: bool) {
            self.active_.store(value, Ordering::Relaxed);
        }

        fn set_should_record_sample(&self) {
            self.record_sample_.store(true, Ordering::Relaxed);
        }
    }

    impl Drop for Sampler {
        fn drop(&mut self) {
            assert!(!self.is_active());
        }
    }

    #[cfg(feature = "use_signals")]
    pub type AtomicMutex = AtomicBool;

    #[cfg(feature = "use_signals")]
    pub struct AtomicGuard<'a> {
        atomic_: &'a AtomicMutex,
        is_success_: bool,
    }

    #[cfg(feature = "use_signals")]
    impl<'a> AtomicGuard<'a> {
        pub fn new(atomic: &'a AtomicMutex, is_blocking: bool) -> Self {
            let mut guard = AtomicGuard {
                atomic_: atomic,
                is_success_: false,
            };
            loop {
                let expected = false;
                guard.is_success_ = guard
                    .atomic_
                    .compare_exchange_strong(expected, true, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok();
                if is_blocking || guard.is_success_ {
                    break;
                }
            }
            guard
        }

        pub fn is_success(&self) -> bool {
            self.is_success_
        }
    }

    #[cfg(feature = "use_signals")]
    impl<'a> Drop for AtomicGuard<'a> {
        fn drop(&mut self) {
            if !self.is_success_ {
                return;
            }
            self.atomic_.store(false, Ordering::SeqCst);
        }
    }

    #[cfg(feature = "use_signals")]
    pub struct SamplerManager {
        sampler_map_: Mutex<HashMap<ThreadId, Vec<*const Sampler>>>,
        samplers_access_counter_: AtomicMutex,
    }

    #[cfg(feature = "use_signals")]
    impl SamplerManager {
        fn new() -> Self {
            SamplerManager {
                sampler_map_: Mutex::new(HashMap::new()),
                samplers_access_counter_: AtomicBool::new(false),
            }
        }

        pub fn add_sampler(&self, sampler: *const Sampler) {
            let guard = AtomicGuard::new(&self.samplers_access_counter_, true);
            if !guard.is_success() {
                return;
            }
            unsafe {
                assert!((*sampler).is_active());
                let thread_id = (*sampler).platform_data().vm_tid();
                let mut map = self.sampler_map_.lock().unwrap();
                let samplers = map.entry(thread_id).or_insert(Vec::new());
                if !samplers.contains(&sampler) {
                    samplers.push(sampler);
                }
            }
        }

        pub fn remove_sampler(&self, sampler: *const Sampler) {
            let guard = AtomicGuard::new(&self.samplers_access_counter_, true);
            if !guard.is_success() {
                return;
            }

            unsafe {
                assert!((*sampler).is_active());
                let thread_id = (*sampler).platform_data().vm_tid();
                let mut map = self.sampler_map_.lock().unwrap();
                if let Some(samplers) = map.get_mut(&thread_id) {
                    samplers.retain(|&s| s != sampler);
                    if samplers.is_empty() {
                        map.remove(&thread_id);
                    }
                }
            }
        }

        pub fn do_sample(&self, state: &RegisterState) {
            let guard = AtomicGuard::new(&self.samplers_access_counter_, false);
            if !guard.is_success() {
                return;
            }

            let thread_id = thread::current().id();
            let map = self.sampler_map_.lock().unwrap();
            if let Some(samplers) = map.get(&thread_id) {
                for &sampler in samplers {
                    unsafe {
                        if !(*sampler).should_record_sample() {
                            continue;
                        }
                        let isolate = (*sampler).isolate();
                        if isolate == std::ptr::null_mut() || !(*isolate).is_in_use() {
                            continue;
                        }
                        //(*sampler).sample_stack(state);
                    }
                }
            }
        }

        pub fn instance() -> &'static Self {
            use std::sync::Once;
            static mut INSTANCE: *const SamplerManager = std::ptr::null();
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                let instance = SamplerManager::new();
                unsafe {
                    INSTANCE = std::mem::transmute(Box::new(instance));
                }
            });

            unsafe { &*INSTANCE }
        }
    }

    pub struct PlatformData {
        vm_tid_: ThreadId,
        vm_tself_: Option<thread::Thread>,
    }

    impl PlatformData {
        fn new() -> Box<Self> {
            Box::new(PlatformData {
                vm_tid_: thread::current().id(),
                vm_tself_: thread::current().thread().clone().into(),
            })
        }
        pub fn vm_tid(&self) -> ThreadId {
            self.vm_tid_.clone()
        }
        pub fn vm_tself(&self) -> Option<thread::Thread> {
            self.vm_tself_.clone()
        }
    }

    #[cfg(feature = "use_signals")]
    struct SignalHandler {
        mutex_: RecursiveMutex,
        client_count_: Mutex<i32>,
        signal_handler_installed_: AtomicBool,
    }

    #[cfg(feature = "use_signals")]
    impl SignalHandler {
        const fn new() -> Self {
            SignalHandler {
                mutex_: RecursiveMutex::new(),
                client_count_: Mutex::new(0),
                signal_handler_installed_: AtomicBool::new(false),
            }
        }

        fn increase_sampler_count() {
            let lock_guard = SignalHandler::mutex();
            let mut count = lock_guard.client_count_.lock().unwrap();
            *count += 1;
            if *count == 1 {
                //SignalHandler::install();
            }
        }

        fn decrease_sampler_count() {
            let lock_guard = SignalHandler::mutex();
            let mut count = lock_guard.client_count_.lock().unwrap();
            *count -= 1;
            if *count == 0 {
                //SignalHandler::restore();
            }
        }

        fn installed() -> bool {
            let _lock_guard = SignalHandler::mutex();
            SignalHandler::instance().signal_handler_installed_.load(Ordering::Relaxed)
        }

        fn mutex() -> &'static RecursiveMutex {
            &Self::instance().mutex_
        }

        fn instance() -> &'static SignalHandler {
            use std::sync::Once;
            static mut INSTANCE: *const SignalHandler = std::ptr::null();
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                let instance = SignalHandler::new();
                unsafe {
                    INSTANCE = std::mem::transmute(Box::new(instance));
                }
            });

            unsafe { &*INSTANCE }
        }

    }
}

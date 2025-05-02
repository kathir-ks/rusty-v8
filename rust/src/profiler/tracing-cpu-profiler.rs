// src/profiler/tracing_cpu_profiler.rs

use std::sync::{Arc, Mutex};
use v8::Isolate;
//use v8::Platform; // TODO: Add when v8 crate has Platform

#[cfg(feature = "v8_use_perfetto")]
use perfetto::DataSourceBase;
#[cfg(feature = "v8_use_perfetto")]
use perfetto::TrackEvent;

use v8::Task;
use v8::{GetCurrentPlatform, GetForegroundTaskRunner};

mod cpu_profiler;
use cpu_profiler::CpuProfiler;
use cpu_profiler::DebugNamingMode;

const K_DEBUG_NAMING: DebugNamingMode = DebugNamingMode::DebugNaming;
const K_LEAF_NODE_LINE_NUMBERS: i32 = 0x01; // Placeholder.  No direct translation without more context

// Placeholder for tracing facilities.  Replace with actual crate usage.
// #[cfg(not(feature = "v8_use_perfetto"))]
// mod tracing {
//     pub trait TraceStateObserver {
//         fn on_trace_enabled(&mut self);
//         fn on_trace_disabled(&mut self);
//     }

//     pub trait TracingController {
//         fn add_trace_state_observer(&mut self, observer: &mut dyn TraceStateObserver);
//         fn remove_trace_state_observer(&mut self, observer: &mut dyn TraceStateObserver);
//     }
// }

pub struct TracingCpuProfilerImpl {
    isolate_: *mut Isolate,
    profiling_enabled_: bool,
    profiler_: Mutex<Option<CpuProfiler>>,
    mutex_: Mutex<()>, // Replaces base::Mutex
}

impl TracingCpuProfilerImpl {
    pub fn new(isolate: *mut Isolate) -> TracingCpuProfilerImpl {
        let profiler = TracingCpuProfilerImpl {
            isolate_: isolate,
            profiling_enabled_: false,
            profiler_: Mutex::new(None),
            mutex_: Mutex::new(()),
        };

        #[cfg(feature = "v8_use_perfetto")]
        {
            // TODO: Convert the C++ AddSessionObserver to Rust equivalent
            // TrackEvent::add_session_observer(&profiler);
            if TrackEvent::is_enabled() {
                // TODO: Convert the C++ OnStart to Rust equivalent
                // profiler.on_start({});
            }
        }

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            // TODO: Convert the C++ GetCurrentPlatform()->GetTracingController()->AddTraceStateObserver(this); to Rust equivalent
            //if let Some(platform) = GetCurrentPlatform() {
            //     platform.get_tracing_controller().add_trace_state_observer(profiler);
            //}
        }

        profiler
    }

    pub fn drop(&mut self) {
        self.stop_profiling();

        #[cfg(feature = "v8_use_perfetto")]
        {
            // TODO: Convert the C++ RemoveSessionObserver to Rust equivalent
            // TrackEvent::remove_session_observer(self);
        }

        #[cfg(not(feature = "v8_use_perfetto"))]
        {
            // TODO: Convert the C++ GetCurrentPlatform()->GetTracingController()->RemoveTraceStateObserver(this); to Rust equivalent
            // if let Some(platform) = GetCurrentPlatform() {
            //    platform.get_tracing_controller().remove_trace_state_observer(self);
            // }
        }
    }

    #[cfg(feature = "v8_use_perfetto")]
    pub fn on_start(&mut self, _args: &DataSourceBase::StartArgs) {
        self.on_trace_enabled();
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn on_trace_enabled(&mut self) {
        let mut enabled = false;
        // TODO: Replace with actual tracing calls
        //TRACE_EVENT_CATEGORY_GROUP_ENABLED(TRACE_DISABLED_BY_DEFAULT("v8.cpu_profiler"), &enabled);
        if !enabled {
            return;
        }
        self.profiling_enabled_ = true;
        unsafe {
            let data = self as *mut Self as *mut std::ffi::c_void;
            let isolate = &mut *self.isolate_;
            isolate.request_interrupt(
                Some(Self::start_profiling_callback),
                data,
            );
        }
    }

    unsafe extern "C" fn start_profiling_callback(
        isolate: *mut Isolate,
        data: *mut std::ffi::c_void,
    ) {
        let this = data as *mut TracingCpuProfilerImpl;
        (*this).start_profiling();
    }

    #[cfg(feature = "v8_use_perfetto")]
    pub fn on_stop(&mut self, _args: &DataSourceBase::StopArgs) {
        self.on_trace_disabled();
    }

    #[cfg(not(feature = "v8_use_perfetto"))]
    pub fn on_trace_disabled(&mut self) {
        let _lock = self.mutex_.lock().unwrap();
        if !self.profiling_enabled_ {
            return;
        }
        self.profiling_enabled_ = false;
        unsafe {
            let data = self as *mut Self as *mut std::ffi::c_void;
            let isolate = &mut *self.isolate_;
            isolate.request_interrupt(
                Some(Self::stop_profiling_callback),
                data,
            );
        }

        // It could be a long time until the Isolate next runs any JS which could be
        // interrupted, and we'd rather not leave the sampler thread running during
        // that time, so also post a task to run any interrupts.
        unsafe {
            let isolate = &mut *self.isolate_;
            let task = RunInterruptsTask::new(self.isolate_);
            let foreground_task_runner = GetCurrentPlatform()
                .unwrap()
                .get_foreground_task_runner(isolate);
            foreground_task_runner.post_task(Box::new(task));
        }
    }

    unsafe extern "C" fn stop_profiling_callback(
        isolate: *mut Isolate,
        data: *mut std::ffi::c_void,
    ) {
        let this = data as *mut TracingCpuProfilerImpl;
        (*this).stop_profiling();
    }

    pub fn start_profiling(&mut self) {
        let _lock = self.mutex_.lock().unwrap();
        if !self.profiling_enabled_ {
            return;
        }

        let mut profiler_guard = self.profiler_.lock().unwrap();
        if profiler_guard.is_some() {
            return;
        }

        let sampling_interval_us = 100;
        let mut profiler = CpuProfiler::new(self.isolate_, K_DEBUG_NAMING);

        profiler.set_sampling_interval(sampling_interval_us);
        profiler.start_profiling("", K_LEAF_NODE_LINE_NUMBERS);

        *profiler_guard = Some(profiler);
    }

    pub fn stop_profiling(&mut self) {
        let _lock = self.mutex_.lock().unwrap();

        let mut profiler_guard = self.profiler_.lock().unwrap();
        if profiler_guard.is_none() {
            return;
        }

        if let Some(mut profiler) = profiler_guard.take() {
            profiler.stop_profiling("");
        }
    }
}

struct RunInterruptsTask {
    isolate_: *mut Isolate,
}

impl RunInterruptsTask {
    pub fn new(isolate: *mut Isolate) -> Self {
        RunInterruptsTask { isolate_: isolate }
    }
}

impl Task for RunInterruptsTask {
    fn run(&mut self) {
        unsafe {
            let isolate = &mut *self.isolate_;
            isolate.stack_guard().handle_interrupts();
        }
    }
}
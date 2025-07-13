// Converted from V8 C++ source files:
// Header: metric-recorder.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

pub struct StatsCollector {}

pub struct MetricRecorder {
}

impl MetricRecorder {
    pub fn new() -> Self {
        MetricRecorder{}
    }

    pub fn add_main_thread_event_gc_cycle(&mut self, _event: &GCCycle) {}
    pub fn add_main_thread_event_incremental_mark(&mut self, _event: &MainThreadIncrementalMark) {}
    pub fn add_main_thread_event_incremental_sweep(&mut self, _event: &MainThreadIncrementalSweep) {}
}

impl Drop for MetricRecorder {
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct GCCycle {
    pub cycle_type: GCCycleType,
    pub total: Phases,
    pub main_thread: Phases,
    pub main_thread_atomic: Phases,
    pub main_thread_incremental: IncrementalPhases,
    pub objects: Sizes,
    pub memory: Sizes,
    pub collection_rate_in_percent: f64,
    pub efficiency_in_bytes_per_us: f64,
    pub main_thread_efficiency_in_bytes_per_us: f64,
}

impl GCCycle {
    pub fn new() -> Self {
        GCCycle {
            cycle_type: GCCycleType::kMajor,
            total: Phases::new(),
            main_thread: Phases::new(),
            main_thread_atomic: Phases::new(),
            main_thread_incremental: IncrementalPhases::new(),
            objects: Sizes::new(),
            memory: Sizes::new(),
            collection_rate_in_percent: 0.0,
            efficiency_in_bytes_per_us: 0.0,
            main_thread_efficiency_in_bytes_per_us: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct IncrementalPhases {
    pub mark_duration_us: i64,
    pub sweep_duration_us: i64,
}

impl IncrementalPhases {
    pub fn new() -> Self {
        IncrementalPhases {
            mark_duration_us: -1,
            sweep_duration_us: -1,
        }
    }
}

#[derive(Debug)]
pub struct Phases {
    pub incremental_phases: IncrementalPhases,
    pub weak_duration_us: i64,
    pub compact_duration_us: i64,
}

impl Phases {
    pub fn new() -> Self {
        Phases {
            incremental_phases: IncrementalPhases::new(),
            weak_duration_us: -1,
            compact_duration_us: -1,
        }
    }
}

#[derive(Debug)]
pub struct Sizes {
    pub before_bytes: i64,
    pub after_bytes: i64,
    pub freed_bytes: i64,
}

impl Sizes {
    pub fn new() -> Self {
        Sizes {
            before_bytes: -1,
            after_bytes: -1,
            freed_bytes: -1,
        }
    }
}

#[derive(Debug)]
pub enum GCCycleType {
    kMinor,
    kMajor,
}

#[derive(Debug)]
pub struct MainThreadIncrementalMark {
    pub duration_us: i64,
}

impl MainThreadIncrementalMark {
    pub fn new() -> Self {
        MainThreadIncrementalMark {
            duration_us: -1,
        }
    }
}

#[derive(Debug)]
pub struct MainThreadIncrementalSweep {
    pub duration_us: i64,
}

impl MainThreadIncrementalSweep {
    pub fn new() -> Self {
        MainThreadIncrementalSweep {
            duration_us: -1,
        }
    }
}

}  // namespace internal
}  // namespace cppgc

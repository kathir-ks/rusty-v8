// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {

    /// Trait for collecting GC statistics.
    pub trait StatsCollector {}

    /// Base trait used for reporting GC statistics histograms. Embedders interested
    /// in collecting histograms should implement the add_main_thread_event
    /// methods below and pass an instance of the implementation during Heap
    /// creation.
    pub trait MetricRecorder {
        fn add_main_thread_event_gc_cycle(&self, _event: &GCCycle) {}
        fn add_main_thread_event_incremental_mark(&self, _event: &MainThreadIncrementalMark) {}
        fn add_main_thread_event_incremental_sweep(&self, _event: &MainThreadIncrementalSweep) {}
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct GCCycle {
        pub type_: GCCycleType,
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

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
    pub enum GCCycleType {
        #[default]
        Major,
        Minor,
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct IncrementalPhases {
        pub mark_duration_us: i64,
        pub sweep_duration_us: i64,
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Phases {
        pub mark_duration_us: i64,
        pub sweep_duration_us: i64,
        pub weak_duration_us: i64,
        pub compact_duration_us: i64,
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Sizes {
        pub before_bytes: i64,
        pub after_bytes: i64,
        pub freed_bytes: i64,
    }

    impl Default for IncrementalPhases {
        fn default() -> Self {
            IncrementalPhases {
                mark_duration_us: -1,
                sweep_duration_us: -1,
            }
        }
    }

    impl Default for Phases {
        fn default() -> Self {
            Phases {
                mark_duration_us: -1,
                sweep_duration_us: -1,
                weak_duration_us: -1,
                compact_duration_us: -1,
            }
        }
    }

    impl Default for Sizes {
        fn default() -> Self {
            Sizes {
                before_bytes: -1,
                after_bytes: -1,
                freed_bytes: -1,
            }
        }
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct MainThreadIncrementalMark {
        pub duration_us: i64,
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct MainThreadIncrementalSweep {
        pub duration_us: i64,
    }

    impl Default for MainThreadIncrementalMark {
        fn default() -> Self {
            MainThreadIncrementalMark { duration_us: -1 }
        }
    }

    impl Default for MainThreadIncrementalSweep {
        fn default() -> Self {
            MainThreadIncrementalSweep { duration_us: -1 }
        }
    }

    /// A basic implementation of the MetricRecorder trait that does nothing.
    pub struct DefaultMetricRecorder {}

    impl MetricRecorder for DefaultMetricRecorder {}
}
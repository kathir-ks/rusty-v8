// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::{fmt, mem, sync::Mutex};

// use v8_metrics; // Assuming a Rust crate equivalent to include/v8-metrics.h
use crate::base::{BoundedAverageSpeed, RingBuffer, SmoothedBytesAndDuration};
use crate::common::globals::*;
// use crate::execution::thread_id::ThreadId; // Assuming a Rust equivalent
// use crate::heap::cppgc_js::cpp_heap::CppHeap; // Assuming a Rust equivalent
// use crate::heap::cppgc::metric_recorder::MetricRecorder; // Assuming a Rust equivalent
// use crate::heap::gc_tracer_inl::*; // Assuming a Rust equivalent (probably inline functions)
// use crate::heap::heap_inl::*; // Assuming a Rust equivalent (probably inline functions)
use crate::heap::heap::Heap;
// use crate::heap::incremental_marking::IncrementalMarking; // Assuming a Rust equivalent
// use crate::heap::memory_balancer::MemoryBalancer; // Assuming a Rust equivalent
// use crate::heap::spaces::*; // Assuming a Rust equivalent
// use crate::logging::counters::Counters; // Assuming a Rust equivalent
// use crate::logging::metrics::*; // Assuming a Rust equivalent
// use crate::logging::tracing_flags::*; // Assuming a Rust equivalent
// use crate::tracing::tracing_category_observer::*; // Assuming a Rust equivalent

// Placeholder for tracing macros
macro_rules! trace_gc_note {
    ($message:expr) => {
        // Implement tracing functionality here if needed
        println!("TRACE_GC_NOTE: {}", $message);
    };
}

// Placeholder for tracing macros
macro_rules! trace_event_instant1 {
    ($category:expr, $name:expr, $scope:expr, $arg_name:expr, $arg_value:expr) => {
        // Implement tracing functionality here if needed
        println!(
            "TRACE_EVENT_INSTANT1: category={}, name={}, scope={}, arg_name={}, arg_value={}",
            $category, $name, $scope, $arg_name, $arg_value
        );
    };
}

macro_rules! trace_event_scope_thread {
    () => {
        // Placeholder for thread scope
    };
}

macro_rules! trace_str_copy {
    ($str:expr) => {
        $str // Placeholder; in C++, this would copy the string.  Handle appropriately in Rust.
    };
}

const GB: usize = 1024 * 1024 * 1024;

fn count_total_holes_size(heap: &Heap) -> usize {
    // This function needs access to the internals of `PagedSpaceIterator` and
    // `PagedSpace`, which are assumed to be translated to Rust data structures.
    // Since direct translation without these definitions is impossible,
    // the following is a placeholder implementation:

    // Assuming a method on Heap that provides the total waste and available memory
    heap.total_waste_and_available()
}

mod base {
    use std::{
        fmt,
        ops::{Add, Div},
        time::Duration,
    };

    #[derive(Clone, Copy, Debug, Default)]
    pub struct BytesAndDuration {
        pub bytes: usize,
        pub duration: Duration,
    }

    impl Add for BytesAndDuration {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                bytes: self.bytes + other.bytes,
                duration: self.duration + other.duration,
            }
        }
    }

    impl Div<usize> for BytesAndDuration {
        type Output = Self;

        fn div(self, rhs: usize) -> Self {
            Self {
                bytes: self.bytes / rhs,
                duration: self.duration / rhs as u32, // Duration doesn't implement div for usize, so convert
            }
        }
    }

    impl fmt::Display for BytesAndDuration {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Bytes: {}, Duration: {:?}",
                self.bytes, self.duration
            )
        }
    }

    // A simple smoothed value
    #[derive(Clone, Copy, Debug, Default)]
    pub struct SmoothedBytesAndDuration {
        pub bytes_per_ms: f64,
    }

    impl SmoothedBytesAndDuration {
        pub fn update(&mut self, bytes: usize, duration: Duration) {
            if duration.is_zero() {
                return;
            }
            let new_bytes_per_ms = bytes as f64 / duration.as_millis() as f64;
            self.bytes_per_ms = (self.bytes_per_ms + new_bytes_per_ms) / 2.0;
        }

        pub fn get_throughput(&self) -> f64 {
            self.bytes_per_ms
        }
    }

    // Ring buffer implementation with a fixed size
    #[derive(Debug)]
    pub struct RingBuffer<T: Copy + Default> {
        buffer: Vec<T>,
        capacity: usize,
        head: usize,
        size: usize,
    }

    impl<T: Copy + Default> RingBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            RingBuffer {
                buffer: vec![T::default(); capacity],
                capacity,
                head: 0,
                size: 0,
            }
        }

        pub fn push(&mut self, value: T) {
            self.buffer[self.head] = value;
            self.head = (self.head + 1) % self.capacity;
            if self.size < self.capacity {
                self.size += 1;
            }
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn reduce<F, A>(&self, op: F, initial_value: A) -> A
        where
            F: Fn(A, T) -> A,
            A: Copy,
        {
            let mut accumulator = initial_value;
            for i in 0..self.size {
                let index = (self.head + self.capacity - self.size + i) % self.capacity;
                accumulator = op(accumulator, self.buffer[index]);
            }
            accumulator
        }

        pub fn iter(&self) -> RingBufferIterator<'_, T> {
            RingBufferIterator {
                buffer: &self.buffer,
                head: self.head,
                size: self.size,
                capacity: self.capacity,
                index: 0,
            }
        }
    }

    pub struct RingBufferIterator<'a, T> {
        buffer: &'a Vec<T>,
        head: usize,
        size: usize,
        capacity: usize,
        index: usize,
    }

    impl<'a, T: Copy + Default> Iterator for RingBufferIterator<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.size {
                let index = (self.head + self.capacity - self.size + self.index) % self.capacity;
                self.index += 1;
                Some(self.buffer[index])
            } else {
                None
            }
        }
    }
    pub fn bounded_average_speed(
        buffer: &RingBuffer<BytesAndDuration>,
    ) -> Option<f64> {
        const K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS: usize = 1;
        const K_MAX_SPEED_IN_BYTES_PER_MS: usize = super::GB; // Assuming GB is defined in the parent scope

        let mut total_bytes: usize = 0;
        let mut total_duration: Duration = Duration::from_secs(0);

        for &bd in buffer.iter() {
            total_bytes += bd.bytes;
            total_duration += bd.duration;
        }

        if total_bytes == 0 || total_duration.is_zero() {
            return None;
        }

        let bytes_per_ms =
            total_bytes as f64 / total_duration.as_millis() as f64;

        if bytes_per_ms < K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS as f64 {
            return Some(K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS as f64);
        }

        if bytes_per_ms > K_MAX_SPEED_IN_BYTES_PER_MS as f64 {
            return Some(K_MAX_SPEED_IN_BYTES_PER_MS as f64);
        }
        Some(bytes_per_ms)
    }

    pub fn bounded_throughput(buffer: &SmoothedBytesAndDuration) -> f64 {
        const K_MAX_SPEED_IN_BYTES_PER_MS: f64 = super::GB as f64;
        buffer.get_throughput().min(K_MAX_SPEED_IN_BYTES_PER_MS)
    }
}

static GLOBAL_EPOCH: AtomicU64 = AtomicU64::new(0);

fn next_epoch() -> CollectionEpoch {
    GLOBAL_EPOCH.fetch_add(1, Ordering::Relaxed) + 1
}

pub type CollectionEpoch = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GarbageCollectionReason {
    kTesting,
    kMeasureMemory,
    kAllocationLimit,
    kLast ditch,
    kFirst,
    kDebugger,
    kLowMemory,
    // Add other reasons as needed
}

impl fmt::Display for GarbageCollectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GarbageCollector {
    SCAVENGER,
    MARK_COMPACTOR,
    MINOR_MARK_SWEEPER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkingType {
    kAtomic,
    kIncremental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Low,
}

#[derive(Debug)]
pub struct GCTracer {
    heap_: *mut Heap, // Assuming `Heap` struct is defined elsewhere
    current_: Event,
    previous_: Event,
    allocation_time_: Instant,
    previous_mark_compact_end_time_: Instant,
    new_space_allocation_counter_bytes_: usize,
    old_generation_allocation_counter_bytes_: usize,
    embedder_allocation_counter_bytes_: usize,
    new_generation_allocations_: SmoothedBytesAndDuration,
    old_generation_allocations_: SmoothedBytesAndDuration,
    embedder_generation_allocations_: SmoothedBytesAndDuration,
    recorded_minor_gc_atomic_pause_: RingBuffer<BytesAndDuration>,
    recorded_minor_gc_per_thread_: RingBuffer<BytesAndDuration>,
    recorded_mark_compacts_: RingBuffer<BytesAndDuration>,
    recorded_incremental_mark_compacts_: RingBuffer<BytesAndDuration>,
    recorded_compactions_: RingBuffer<BytesAndDuration>,
    recorded_embedder_marking_: RingBuffer<BytesAndDuration>,
    recorded_survival_ratios_: RingBuffer<f64>,
    epoch_young_: CollectionEpoch,
    epoch_full_: CollectionEpoch,
    recorded_major_totals_: RingBuffer<BytesAndDuration>,
    average_time_to_incremental_marking_task_: Option<Duration>,
    average_mark_compact_duration_: f64,
    average_mutator_duration_: f64,
    current_mark_compact_mutator_utilization_: f64,
    recorded_major_incremental_marking_speed_: f64,
    combined_mark_compact_speed_cache_: Option<f64>,
    start_of_observable_pause_: Option<Instant>,
    incremental_scopes_: [IncrementalInfos; Scope::NUMBER_OF_INCREMENTAL_SCOPES],
    background_scopes_: [Duration; Scope::NUMBER_OF_BACKGROUND_SCOPES],
    background_scopes_mutex_: Mutex<()>,
    code_flushing_increase_s_: u16,
    last_marking_start_time_for_code_flushing_: Option<Instant>,
    young_gc_while_full_gc_: bool,
    notified_full_sweeping_completed_: bool,
    notified_full_cppgc_completed_: bool,
    notified_young_sweeping_completed_: bool,
    notified_young_cppgc_running_: bool,
    notified_young_cppgc_completed_: bool,
    full_cppgc_completed_during_minor_gc_: bool,
    incremental_mark_batched_events_:
        v8::metrics::GarbageCollectionBatchedEvents<v8::metrics::GarbageCollectionIncrementalMark>,
    incremental_sweep_batched_events_:
        v8::metrics::GarbageCollectionBatchedEvents<v8::metrics::GarbageCollectionIncrementalSweep>,
    // parent_track_: Assuming a Rust equivalent for perfetto::ThreadTrack
}

impl fmt::Display for GCTracer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GCTracer")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IncrementalInfos {
    pub duration: Duration,
    pub steps: usize,
    pub longest_step: Duration,
}

impl IncrementalInfos {
    pub fn new() -> Self {
        IncrementalInfos {
            duration: Duration::from_secs(0),
            steps: 0,
            longest_step: Duration::from_secs(0),
        }
    }
}

impl Default for IncrementalInfos {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Event {
    type_: Event::Type,
    state: Event::State,
    gc_reason: GarbageCollectionReason,
    collector_reason: Option<&'static str>,
    priority: Option<Priority>,
    start_time: Instant,
    end_time: Instant,
    start_object_size: usize,
    end_object_size: usize,
    start_memory_size: usize,
    end_memory_size: usize,
    start_holes_size: usize,
    end_holes_size: usize,
    young_object_size: usize,
    survived_young_object_size: usize,
    start_atomic_pause_time: Instant,
    end_atomic_pause_time: Instant,
    incremental_marking_start_time: Instant,
    incremental_marking_bytes: usize,
    incremental_marking_duration: Duration,
    incremental_scopes: [IncrementalInfos; Scope::NUMBER_OF_INCREMENTAL_SCOPES],
    scopes: [Duration; Scope::NUMBER_OF_INCREMENTAL_SCOPES],
    concurrency_estimate: usize,
    reduce_memory: bool,
}

impl Event {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Type {
        SCAVENGER,
        MARK_COMPACTOR,
        INCREMENTAL_MARK_COMPACTOR,
        MINOR_MARK_SWEEPER,
        INCREMENTAL_MINOR_MARK_SWEEPER,
        START,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        NOT_RUNNING,
        MARKING,
        ATOMIC,
        SWEEPING,
    }

    const NUMBER_OF_STATES: usize = 4;

    fn new(
        type_: Event::Type,
        state: Event::State,
        gc_reason: GarbageCollectionReason,
        collector_reason: Option<&'static str>,
        priority: Option<Priority>,
    ) -> Self {
        Event {
            type_,
            state,
            gc_reason,
            collector_reason,
            priority,
            start_time: Instant::now(), // Initialized to current time, can be updated later
            end_time: Instant::now(),   // Initialized to current time, can be updated later
            start_object_size: 0,
            end_object_size: 0,
            start_memory_size: 0,
            end_memory_size: 0,
            start_holes_size: 0,
            end_holes_size: 0,
            young_object_size: 0,
            survived_young_object_size: 0,
            start_atomic_pause_time: Instant::now(), // Initialized to current time
            end_atomic_pause_time: Instant::now(),   // Initialized to current time
            incremental_marking_start_time: Instant::now(), // Initialized to current time
            incremental_marking_bytes: 0,
            incremental_marking_duration: Duration::from_secs(0),
            incremental_scopes: [IncrementalInfos::default(); Scope::NUMBER_OF_INCREMENTAL_SCOPES],
            scopes: [Duration::from_secs(0); Scope::NUMBER_OF_INCREMENTAL_SCOPES],
            concurrency_estimate: 1,
            reduce_memory: false,
        }
    }

    pub fn is_young_generation_event(type_: Event::Type) -> bool {
        type_ == Event::Type::SCAVENGER
            || type_ == Event::Type::MINOR_MARK_SWEEPER
            || type_ == Event::Type::INCREMENTAL_MINOR_MARK_SWEEPER
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event {{ type: {:?}, state: {:?}, gc_reason: {:?} }}",
            self.type_, self.state, self.gc_reason
        )
    }
}

pub fn to_string(type_: Event::Type, short_name: bool) -> &'static str {
    match type_ {
        Event::Type::SCAVENGER => {
            if short_name {
                "s"
            } else {
                "Scavenge"
            }
        }
        Event::Type::MARK_COMPACTOR | Event::Type::INCREMENTAL_MARK_COMPACTOR => {
            if short_name {
                "mc"
            } else {
                "Mark-Compact"
            }
        }
        Event::Type::MINOR_MARK_SWEEPER | Event::Type::INCREMENTAL_MINOR_MARK_SWEEPER => {
            if short_name {
                "mms"
            } else {
                "Minor Mark-Sweep"
            }
        }
        Event::Type::START => {
            if short_name {
                "st"
            } else {
                "Start"
            }
        }
    }
}

pub mod v8 {
    pub mod metrics {
        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionPhases {
            pub compact_wall_clock_duration_in_us: i64,
            pub mark_wall_clock_duration_in_us: i64,
            pub sweep_wall_clock_duration_in_us: i64,
            pub weak_wall_clock_duration_in_us: i64,
            pub total_wall_clock_duration_in_us: i64,
        }

        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionSizes {
            pub bytes_before: usize,
            pub bytes_after: usize,
            pub bytes_freed: usize,
        }

        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionFullCycle {
            pub reason: i32,
            pub priority: Option<super::Priority>,
            pub total_cpp: GarbageCollectionPhases,
            pub main_thread_cpp: GarbageCollectionPhases,
            pub main_thread_atomic_cpp: GarbageCollectionPhases,
            pub main_thread_incremental_cpp: GarbageCollectionPhases,
            pub objects_cpp: GarbageCollectionSizes,
            pub memory_cpp: GarbageCollectionSizes,
            pub collection_rate_cpp_in_percent: f64,
            pub efficiency_cpp_in_bytes_per_us: f64,
            pub main_thread_efficiency_cpp_in_bytes_per_us: f64,
            pub collection_weight_cpp_in_percent: f64,
            pub main_thread_collection_weight_cpp_in_percent: f64,
            pub total: GarbageCollectionPhases,
            pub main_thread: GarbageCollectionPhases,
            pub main_thread_atomic: GarbageCollectionPhases,
            pub objects: GarbageCollectionSizes,
            pub memory: GarbageCollectionSizes,
            pub collection_rate_in_percent: f64,
            pub efficiency_in_bytes_per_us: f64,
            pub main_thread_efficiency_in_bytes_per_us: f64,
            pub collection_weight_in_percent: f64,
            pub main_thread_collection_weight_in_percent: f64,
            pub main_thread_incremental: GarbageCollectionPhases,
            pub incremental_marking_start_stop_wall_clock_duration_in_us: i64,
        }
        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionYoungCycle {
            pub reason: i32,
            pub priority: Option<super::Priority>,
            pub total_cpp: GarbageCollectionPhases,
            pub objects_cpp: GarbageCollectionSizes,
            pub memory_cpp: GarbageCollectionSizes,
            pub collection_rate_cpp_in_percent: f64,
            pub efficiency_cpp_in_bytes_per_us: f64,
            pub main_thread_efficiency_cpp_in_bytes_per_us: f64,
            pub total_wall_clock_duration_in_us: i64,
            pub main_thread_wall_clock_duration_in_us: i64,
            pub collection_rate_in_percent: f64,
            pub efficiency_in_bytes_per_us: f64,
            pub main_thread_efficiency_in_bytes_per_us: f64,
        }
        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionIncrementalMark {
            pub wall_clock_duration_in_us: i64,
            pub cpp_wall_clock_duration_in_us: i64,
        }
        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionIncrementalSweep {
            pub wall_clock_duration_in_us: i64,
        }

        #[derive(Debug, Default, Clone)]
        pub struct GarbageCollectionBatchedEvents<T> {
            pub events: Vec<T>,
        }
        pub trait Recorder {
            fn add_main_thread_event<T: std::fmt::Debug + Clone>(
                &self,
                event: T,
                context_id: Recorder::ContextId,
            );
            fn has_embedder_recorder(&self) -> bool;
            type ContextId;
        }
        impl<T> GarbageCollectionBatchedEvents<T> {
            pub fn new() -> Self {
                Self { events: Vec::new() }
            }
        }

        impl Recorder for () {
            fn add_main_thread_event<T: std::fmt::Debug + Clone>(
                &self,
                event: T,
                context_id: <Self as Recorder>::ContextId,
            ) {
                println!("Adding event: {:?} with context {:?}", event, context_id);
            }

            fn has_embedder_recorder(&self) -> bool {
                false
            }

            type ContextId = u32;
        }

        impl Recorder {
            pub enum ContextId {
                Empty,
                Native(u32),
            }
        }
    }
}

impl GCTracer {
    pub fn new(
        heap: *mut Heap,
        startup_time: Instant,
        initial_gc_reason: GarbageCollectionReason,
    ) -> Self {
        GCTracer {
            heap_: heap,
            current_: Event::new(
                Event::Type::START,
                Event::State::NOT_RUNNING,
                initial_gc_reason,
                None,
                // FIXME: Find a good default for priority
                None,
            ),
            previous_: Event::new(
                Event::Type::START,
                Event::State::NOT_RUNNING,
                initial_gc_reason,
                None,
                // FIXME: Find a good default for priority
                None,
            ),
            allocation_time_: startup_time,
            previous_mark_compact_end_time_: startup_time,
            new_space_allocation_counter_bytes_: 0,
            old_generation_allocation_counter_bytes_: 0,
            embedder_allocation_counter_bytes_: 0,
            new_generation_allocations_: SmoothedBytesAndDuration::default(),
            old_generation_allocations_: SmoothedBytesAndDuration::default(),
            embedder_generation_allocations_: SmoothedBytesAndDuration::default(),
            recorded_minor_gc_atomic_pause_: RingBuffer::new(16), // Example size
            recorded_minor_gc_per_thread_: RingBuffer::new(16), // Example size
            recorded_mark_compacts_: RingBuffer::new(16),       // Example size
            recorded_incremental_mark_compacts_: RingBuffer::new(16), // Example size
            recorded_compactions_: RingBuffer::new(16),         // Example size
            recorded_embedder_marking_: RingBuffer::new(16),    // Example size
            recorded_survival_ratios_: RingBuffer::new(16),      // Example size
            epoch_young_: 0,
            epoch_full_: 0,
            recorded_major_totals_: RingBuffer::new(16),
            average_time_to_incremental_marking_task_: None,
            average_mark_compact_duration_: 0.0,
            average_mutator_duration_: 0.0,
            current_mark_compact_mutator_utilization_: 0.0,
            recorded_major_incremental_marking_speed_: 0.0,
            combined_mark_compact_speed_cache_: None,
            start_of_observable_pause_: None,
            incremental_scopes_: [IncrementalInfos::default(); Scope::NUMBER_OF_INCREMENTAL_SCOPES],
            background_scopes_: [Duration::from_secs(0); Scope::NUMBER_OF_BACKGROUND_SCOPES],
            background_scopes_mutex_: Mutex::new(()),
            code_flushing_increase_s_: 0,
            last_marking_start_time_for_code_flushing_: None,
            young_gc_while_full_gc_: false,
            notified_full_sweeping_completed_: false,
            notified_full_cppgc_completed_: false,
            notified_young_sweeping_completed_: false,
            notified_young_cppgc_running_: false,
            notified_young_cppgc_completed_: false,
            full_cppgc_completed_during_minor_gc_: false,
            incremental_mark_batched_events_:
                v8::metrics::GarbageCollectionBatchedEvents::default(),
            incremental_sweep_batched_events_:
                v8::metrics::GarbageCollectionBatchedEvents::default(),
            // parent_track_: Assuming a Rust equivalent for perfetto::ThreadTrack
        }
    }

    // ResetForTesting is not implemented because it uses placement new,
    // which is not directly supported in Rust. Instead, a new instance
    // should be created.

    pub fn start_observable_pause(&mut self, time: Instant) {
        assert!(self.start_of_observable_pause_.is_none());
        self.start_of_observable_pause_ = Some(time);
    }

    pub fn update_current_event(
        &mut self,
        gc_reason: GarbageCollectionReason,
        collector_reason: Option<&'static str>,
    ) {
        assert!(
            self.current_.type_ == Event::Type::INCREMENTAL_MARK_COMPACTOR
                || self.current_.type_ == Event::Type::INCREMENTAL_MINOR_MARK_SWEEPER
        );
        assert_eq!(self.current_.state, Event::State::ATOMIC);
        assert!(self.is_in_observable_pause());
        self.current_.gc_reason = gc_reason;
        self.current_.collector_reason = collector_reason;
        self.current_.start_time = self.start_of_observable_pause_.unwrap();
        self.current_.reduce_memory = self.should_reduce_memory();
    }

    pub fn start_cycle(
        &mut self,
        collector: GarbageCollector,
        gc_reason: GarbageCollectionReason,
        collector_reason: Option<&'static str>,
        marking: MarkingType,
    ) {
        assert_ne!(self.current_.state, Event::State::ATOMIC);
        assert!(!self.young_gc_while_full_gc_);

        self.young_gc_while_full_gc_ = self.current_.state != Event::State::NOT_RUNNING;
        if self.young_gc_while_full_gc_ {
            self.fetch_background_counters();
        }

        assert!(!self.young_gc_while_full_gc_ || Heap::is_young_generation_collector(collector));
        assert!(
            !self.young_gc_while_full_gc_
                || !matches!(
                    self.current_.type_,
                    Event::Type::SCAVENGER
                        | Event::Type::MINOR_MARK_SWEEPER
                        | Event::Type::INCREMENTAL_MINOR_MARK_SWEEPER
                )
        );

        assert!(!self.young_gc_while_full_gc_ || self.current_.state == Event::State::SWEEPING);
        assert!(!self.young_gc_while_full_gc_ || !Event::is_young_generation_event(self.current_.type_));

        assert!(!self.young_gc_while_full_gc_ || self.current_.state == Event::State::NOT_RUNNING);
        assert_eq!(self.previous_.state, Event::State::NOT_RUNNING);

        let type_ = match collector {
            GarbageCollector::SCAVENGER => Event::Type::SCAVENGER,
            GarbageCollector::MINOR_MARK_SWEEPER => {
                if marking == MarkingType::kIncremental {
                    Event::Type::INCREMENTAL_MINOR_MARK_SWEEPER
                } else {
                    Event::Type::MINOR_MARK_SWEEPER
                }
            }
            GarbageCollector::MARK_COMPACTOR => {
                if marking == MarkingType::kIncremental {
                    Event::Type::INCREMENTAL_MARK_COMPACTOR
                } else {
                    Event::Type::MARK_COMPACTOR
                }
            }
        };

        assert!(!self.young_gc_while_full_gc_ || self.current_.state == Event::State::NOT_RUNNING);
        assert_eq!(self.previous_.state, Event::State::NOT_RUNNING);

        self.previous_ = self.current_;
        self.current_ = Event::new(
            type_,
            Event::State::MARKING,
            gc_reason,
            collector_reason,
            self.isolate_priority(),
        );

        match marking {
            MarkingType::kAtomic => {
                assert!(self.is_in_observable_pause());
                self.current_.start_time = self.start_of_observable_pause_.unwrap();
                self.current_.reduce_memory = self.should_reduce_memory();
            }
            MarkingType::kIncremental => {
                assert!(!self.is_in_observable_pause());
            }
        }

        if Heap::is_young_generation_collector(collector) {
            self.epoch_young_ = next_epoch();
        } else {
            self.epoch_full_ = next_epoch();
        }
    }

    pub fn start_atomic_pause(&mut self) {
        assert_eq!(self.current_.state, Event::State::MARKING);
        self.current_.state = Event::State::ATOMIC;
    }

    pub fn start_in_safepoint(&mut self, time: Instant) {
        self.sample_allocation(
            self.current_.start_time,
            self.new_space_allocation_counter(),
            self.old_generation_allocation_counter(),
            self.embedder_allocation_counter(),
        );

        self.current_.start_object_size = self.size_of_objects();
        self.current_.start_memory_size = self.memory_allocator_size();
        self.current_.start_holes_size = count_total_holes_size(self.heap());
        let new_space_size = self.new_space().map_or(0, |space| space.size());
        let new_lo_space_size = self
            .new_lo_space()
            .map_or(0, |space| space.size_of_objects());
        self.current_.young_object_size = new_space_size + new_lo_space_size;
        self.current_.start_atomic_pause_time = time;
    }

    pub fn stop_in_safepoint(&mut self, time: Instant) {
        self.current_.end_object_size = self.size_of_objects();
        self.current_.end_memory_size = self.memory_allocator_size();
        self.current_.end_holes_size = count_total_holes_size(self.heap());
        self.current_.survived_young_object_size = self.survived_young_object_size();
        self.
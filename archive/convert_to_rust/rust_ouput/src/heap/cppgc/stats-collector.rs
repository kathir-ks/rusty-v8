// Converted from V8 C++ source files:
// Header: stats-collector.h
// Implementation: stats-collector.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/cppgc/stats-collector.h

use std::sync::{Arc, Mutex};

use crate::base::{TimeDelta, TimeTicks, AtomicWord};
use crate::heap::cppgc::{GarbageCollector, MetricRecorder};
use crate::heap::cppgc::trace_event::Platform;
use std::sync::atomic::{AtomicUsize, Ordering};

// Histogram scopes contribute to histogram as well as to traces and metrics.
// Other scopes contribute only to traces and metrics.
// Macro-based enum declaration in Rust
macro_rules! cppgc_for_all_histogram_scopes {
    ($callback:ident) => {
        $callback!(AtomicMark);
        $callback!(AtomicWeak);
        $callback!(AtomicCompact);
        $callback!(AtomicSweep);
        $callback!(IncrementalMark);
        $callback!(IncrementalSweep);
    };
}

macro_rules! cppgc_for_all_scopes {
    ($callback:ident) => {
        $callback!(Unmark);
        $callback!(MarkIncrementalStart);
        $callback!(MarkIncrementalFinalize);
        $callback!(MarkAtomicPrologue);
        $callback!(MarkAtomicEpilogue);
        $callback!(MarkTransitiveClosure);
        $callback!(MarkTransitiveClosureWithDeadline);
        $callback!(MarkFlushEphemerons);
        $callback!(MarkOnAllocation);
        $callback!(MarkProcessBailOutObjects);
        $callback!(MarkProcessMarkingWorklist);
        $callback!(MarkProcessRetraceWorklist);
        $callback!(MarkProcessWriteBarrierWorklist);
        $callback!(MarkProcessNotFullyconstructedWorklist);
        $callback!(MarkProcessEphemerons);
        $callback!(MarkVisitRoots);
        $callback!(MarkVisitNotFullyConstructedObjects);
        $callback!(MarkVisitPersistents);
        $callback!(MarkVisitCrossThreadPersistents);
        $callback!(MarkVisitStack);
        $callback!(MarkVisitRememberedSets);
        $callback!(WeakContainerCallbacksProcessing);
        $callback!(CustomCallbacksProcessing);
        $callback!(SweepEmptyPages);
        $callback!(SweepFinish);
        $callback!(SweepFinalizeEmptyPages);
        $callback!(SweepFinalizeSweptPages);
        $callback!(SweepFinishIfOutOfWork);
        $callback!(SweepInvokePreFinalizers);
        $callback!(SweepInLowPriorityTask);
        $callback!(SweepInTask);
        $callback!(SweepInTaskForStatistics);
        $callback!(SweepOnAllocation);
        $callback!(SweepPages);
    };
}

macro_rules! cppgc_for_all_histogram_concurrent_scopes {
    ($callback:ident) => {
        $callback!(ConcurrentMark);
        $callback!(ConcurrentSweep);
        $callback!(ConcurrentWeakCallback);
    };
}

macro_rules! cppgc_for_all_concurrent_scopes {
    ($callback:ident) => {
        $callback!(ConcurrentMarkProcessEphemeronWorklist);
        $callback!(ConcurrentMarkProcessMarkingWorklist);
        $callback!(ConcurrentMarkProcessNotFullyconstructedWorklist);
        $callback!(ConcurrentMarkProcessWriteBarrierWorklist);
    };
}
pub enum CollectionType {
    kMajor,
    kMinor,
}

pub enum MarkingType {
    kAtomic,
    kIncremental,
}

pub enum SweepingType {
    kAtomic,
    kIncremental,
}
pub enum IsForcedGC {
    kForced,
    kNotForced,
}
// Sink for various time and memory statistics.
#[derive(Debug)]
pub struct StatsCollector {
    allocated_bytes_since_end_of_marking_: AtomicI64,
    time_of_last_end_of_marking_: Mutex<TimeTicks>,
    allocated_bytes_since_safepoint_: AtomicI64,
    explicitly_freed_bytes_since_safepoint_: AtomicI64,
    tracked_live_bytes_: Mutex<usize>,
    marked_bytes_so_far_: AtomicUsize,
    memory_allocated_bytes_: AtomicI64,
    memory_freed_bytes_since_end_of_marking_: AtomicI64,
    discarded_bytes_: AtomicUsize,
    allocation_observers_: Mutex<Vec<Box<dyn AllocationObserver + Send + Sync>>>,
    gc_state_: Mutex<GarbageCollectionState>,
    current_: Mutex<Event>,
    previous_: Mutex<Event>,
    metric_recorder_: Mutex<Option<Box<dyn MetricRecorder + Send + Sync>>>,
    platform_: *mut Platform,
    allocation_observer_deleted_: Mutex<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GarbageCollectionState {
    kNotRunning,
    kUnmarking,
    kMarking,
    kSweeping,
}

// Macro to generate enum ScopeId
macro_rules! declare_scope_id {
    ($name:ident) => {
        $name,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeId {
    AtomicMark,
    AtomicWeak,
    AtomicCompact,
    AtomicSweep,
    IncrementalMark,
    IncrementalSweep,
    NumHistogramScopeIds,
    Unmark,
    MarkIncrementalStart,
    MarkIncrementalFinalize,
    MarkAtomicPrologue,
    MarkAtomicEpilogue,
    MarkTransitiveClosure,
    MarkTransitiveClosureWithDeadline,
    MarkFlushEphemerons,
    MarkOnAllocation,
    MarkProcessBailOutObjects,
    MarkProcessMarkingWorklist,
    MarkProcessRetraceWorklist,
    MarkProcessWriteBarrierWorklist,
    MarkProcessNotFullyconstructedWorklist,
    MarkProcessEphemerons,
    MarkVisitRoots,
    MarkVisitNotFullyConstructedObjects,
    MarkVisitPersistents,
    MarkVisitCrossThreadPersistents,
    MarkVisitStack,
    MarkVisitRememberedSets,
    WeakContainerCallbacksProcessing,
    CustomCallbacksProcessing,
    SweepEmptyPages,
    SweepFinish,
    SweepFinalizeEmptyPages,
    SweepFinalizeSweptPages,
    SweepFinishIfOutOfWork,
    SweepInvokePreFinalizers,
    SweepInLowPriorityTask,
    SweepInTask,
    SweepInTaskForStatistics,
    SweepOnAllocation,
    SweepPages,
    NumScopeIds,
}

// Macro to generate enum ConcurrentScopeId
macro_rules! declare_concurrent_scope_id {
    ($name:ident) => {
        $name,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConcurrentScopeId {
    ConcurrentMark,
    ConcurrentSweep,
    ConcurrentWeakCallback,
    NumHistogramConcurrentScopeIds,
    ConcurrentMarkProcessEphemeronWorklist,
    ConcurrentMarkProcessMarkingWorklist,
    ConcurrentMarkProcessNotFullyconstructedWorklist,
    ConcurrentMarkProcessWriteBarrierWorklist,
    NumConcurrentScopeIds,
}
impl StatsCollector {
    pub const K_ALLOCATION_THRESHOLD_BYTES: usize = 1024;

    pub fn new(platform: *mut Platform) -> StatsCollector {
        StatsCollector {
            allocated_bytes_since_end_of_marking_: AtomicI64::new(0),
            time_of_last_end_of_marking_: Mutex::new(TimeTicks::Now()),
            allocated_bytes_since_safepoint_: AtomicI64::new(0),
            explicitly_freed_bytes_since_safepoint_: AtomicI64::new(0),
            tracked_live_bytes_: Mutex::new(0),
            marked_bytes_so_far_: AtomicUsize::new(0),
            memory_allocated_bytes_: AtomicI64::new(0),
            memory_freed_bytes_since_end_of_marking_: AtomicI64::new(0),
            discarded_bytes_: AtomicUsize::new(0),
            allocation_observers_: Mutex::new(Vec::new()),
            gc_state_: Mutex::new(GarbageCollectionState::kNotRunning),
            current_: Mutex::new(Event::new()),
            previous_: Mutex::new(Event::new()),
            metric_recorder_: Mutex::new(None),
            platform_: platform,
            allocation_observer_deleted_: Mutex::new(false),
        }
    }

    pub fn register_observer(&self, observer: Box<dyn AllocationObserver + Send + Sync>) {
        let mut observers = self.allocation_observers_.lock().unwrap();
        observers.push(observer);
    }

    pub fn unregister_observer(&self, observer: &dyn AllocationObserver) {
        let mut observers = self.allocation_observers_.lock().unwrap();
        if let Some(index) = observers.iter().position(|o| o.as_ref() as *const dyn AllocationObserver == observer as *const dyn AllocationObserver) {
            observers.remove(index);
        }
    }

    pub fn notify_allocation(&self, bytes: usize) {
        self.allocated_bytes_since_safepoint_.fetch_add(bytes as i64, Ordering::Relaxed);
        let mut tracked_live_bytes = self.tracked_live_bytes_.lock().unwrap();
        *tracked_live_bytes += bytes;
    }

    pub fn notify_explicit_free(&self, bytes: usize) {
        self.explicitly_freed_bytes_since_safepoint_.fetch_add(bytes as i64, Ordering::Relaxed);
         let mut tracked_live_bytes = self.tracked_live_bytes_.lock().unwrap();
        *tracked_live_bytes -= bytes;
    }

    pub fn notify_safe_point_for_conservative_collection(&self) {
        let allocated = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed);
        let freed = self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);
        if (allocated - freed).abs() >= StatsCollector::K_ALLOCATION_THRESHOLD_BYTES as i64 {
            self.allocated_object_size_safepoint_impl();
        }
    }

    pub fn notify_safe_point_for_testing(&self) {
        self.allocated_object_size_safepoint_impl();
    }

    fn allocated_object_size_safepoint_impl(&self) {
        let allocated = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed);
        let freed = self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);
        self.allocated_bytes_since_end_of_marking_.fetch_add(allocated - freed, Ordering::Relaxed);

        let observers = self.allocation_observers_.lock().unwrap();
        for observer in observers.iter() {
            let delta = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed) -
                self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);

            if delta < 0 {
                observer.allocated_object_size_decreased((-delta) as usize);
            } else {
                observer.allocated_object_size_increased(delta as usize);
            }
        }

        self.allocated_bytes_since_safepoint_.store(0, Ordering::Relaxed);
        self.explicitly_freed_bytes_since_safepoint_.store(0, Ordering::Relaxed);
    }

    pub fn notify_unmarking_started(&self, collection_type: CollectionType) {
        let mut gc_state = self.gc_state_.lock().unwrap();
        *gc_state = GarbageCollectionState::kUnmarking;
    }

    pub fn notify_marking_started(&self, collection_type: CollectionType, marking_type: MarkingType, is_forced_gc: IsForcedGC) {
        let mut gc_state = self.gc_state_.lock().unwrap();
        let mut current = self.current_.lock().unwrap();
        current.collection_type = collection_type;
        current.is_forced_gc = is_forced_gc;
        current.marking_type = marking_type;
        *gc_state = GarbageCollectionState::kMarking;
    }

    pub fn notify_marking_completed(&self, marked_bytes: usize) {
        let mut gc_state = self.gc_state_.lock().unwrap();
        let mut current = self.current_.lock().unwrap();
        *gc_state = GarbageCollectionState::kSweeping;
        current.marked_bytes = marked_bytes;
        let marked_bytes_so_far = self.marked_bytes_so_far_.load(Ordering::Relaxed);
        let allocated_bytes_since_end_of_marking = self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed);
        let allocated_bytes_since_safepoint = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed);
        let explicitly_freed_bytes_since_safepoint = self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);

        current.object_size_before_sweep_bytes = (marked_bytes_so_far as i64 + allocated_bytes_since_end_of_marking + allocated_bytes_since_safepoint - explicitly_freed_bytes_since_safepoint) as usize;

        self.allocated_bytes_since_safepoint_.store(0, Ordering::Relaxed);
        self.explicitly_freed_bytes_since_safepoint_.store(0, Ordering::Relaxed);

        if current.collection_type == CollectionType::kMajor {
            self.marked_bytes_so_far_.store(0, Ordering::Relaxed);
        }
        self.marked_bytes_so_far_.fetch_add(marked_bytes, Ordering::Relaxed);
        let marked_bytes_so_far = self.marked_bytes_so_far_.load(Ordering::Relaxed);

        let mut observers = self.allocation_observers_.lock().unwrap();
        for observer in observers.iter() {
            observer.reset_allocated_object_size(marked_bytes_so_far);
        }
        self.allocated_bytes_since_end_of_marking_.store(0, Ordering::Relaxed);
        *self.time_of_last_end_of_marking_.lock().unwrap() = TimeTicks::Now();
    }

    pub fn notify_sweeping_completed(&self, sweeping_type: SweepingType) {
        let mut gc_state = self.gc_state_.lock().unwrap();
        *gc_state = GarbageCollectionState::kNotRunning;

        let mut current = self.current_.lock().unwrap();
        let mut previous = self.previous_.lock().unwrap();
        let mut metric_recorder = self.metric_recorder_.lock().unwrap();

        let current_val = std::mem::replace(&mut *current, Event::new());
        *previous = current_val;
        

        if let Some(recorder) = metric_recorder.as_mut() {
            let objects_before = previous.object_size_before_sweep_bytes as i64;
            let marked_bytes_so_far = self.marked_bytes_so_far_.load(Ordering::Relaxed) as i64;
            let objects_freed = objects_before - marked_bytes_so_far;
            let memory_before = previous.memory_size_before_sweep_bytes as i64;
            let memory_freed_bytes_since_end_of_marking = self.memory_freed_bytes_since_end_of_marking_.load(Ordering::Relaxed) as i64;
            let memory_after = memory_before - memory_freed_bytes_since_end_of_marking;
            let event = get_cycle_event_for_metric_recorder(
                previous.collection_type,
                previous.marking_type,
                sweeping_type,
                previous.scope_data[ScopeId::AtomicMark].in_microseconds(),
                previous.scope_data[ScopeId::AtomicWeak].in_microseconds(),
                previous.scope_data[ScopeId::AtomicCompact].in_microseconds(),
                previous.scope_data[ScopeId::AtomicSweep].in_microseconds(),
                previous.scope_data[ScopeId::IncrementalMark].in_microseconds(),
                previous.scope_data[ScopeId::IncrementalSweep].in_microseconds(),
                0, // Placeholder for concurrent mark
                0, // Placeholder for concurrent sweep
                previous.object_size_before_sweep_bytes as i64,
                self.marked_bytes_so_far_.load(Ordering::Relaxed) as i64,
                objects_freed,
                previous.memory_size_before_sweep_bytes as i64,
                memory_after,
                memory_freed_bytes_since_end_of_marking as i64,
            );

        }
    }

    pub fn allocated_memory_size(&self) -> usize {
        (self.memory_allocated_bytes_.load(Ordering::Relaxed) -
            self.memory_freed_bytes_since_end_of_marking_.load(Ordering::Relaxed)) as usize
    }

    pub fn allocated_object_size(&self) -> usize {
         (self.marked_bytes_so_far_.load(Ordering::Relaxed) as i64 +
            self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed)) as usize
    }

    pub fn marked_bytes(&self) -> usize {
        self.marked_bytes_so_far_.load(Ordering::Relaxed)
    }
    pub fn marked_bytes_on_current_cycle(&self) -> usize {
        self.current_.lock().unwrap().marked_bytes
    }
    pub fn marking_time(&self) -> TimeDelta {
       let current = self.current_.lock().unwrap();
        current.scope_data[ScopeId::AtomicMark] + current.scope_data[ScopeId::IncrementalMark]
    }
    pub fn get_recent_allocation_speed_in_bytes_per_ms(&self) -> f64 {
        let current_time = TimeTicks::Now();
        let time_of_last_end_of_marking = *self.time_of_last_end_of_marking_.lock().unwrap();

        if time_of_last_end_of_marking == current_time {
            return 0.0;
        }

        self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed) as f64 /
            (current_time - time_of_last_end_of_marking).in_milliseconds_f()
    }
    pub fn get_previous_event_for_testing(&self) -> Event {
        self.previous_.lock().unwrap().clone()
    }

    pub fn notify_allocated_memory(&self, size: i64) {
         self.memory_allocated_bytes_.fetch_add(size, Ordering::Relaxed);
        let observers = self.allocation_observers_.lock().unwrap();
        for observer in observers.iter() {
            observer.allocated_size_increased(size as usize);
        }
    }
    pub fn notify_freed_memory(&self, size: i64) {
        self.memory_freed_bytes_since_end_of_marking_.fetch_add(size, Ordering::Relaxed);
        let observers = self.allocation_observers_.lock().unwrap();
        for observer in observers.iter() {
            observer.allocated_size_decreased(size as usize);
        }
    }

    pub fn increment_discarded_memory(&self, value: usize) {
        self.discarded_bytes_.fetch_add(value, Ordering::Relaxed);
    }

    pub fn decrement_discarded_memory(&self, value: usize) {
        self.discarded_bytes_.fetch_sub(value, Ordering::Relaxed);
    }

    pub fn reset_discarded_memory(&self) {
        self.discarded_bytes_.store(0, Ordering::Relaxed);
    }
    pub fn discarded_memory_size(&self) -> usize {
        self.discarded_bytes_.load(Ordering::Relaxed)
    }
    pub fn resident_memory_size(&self) -> usize {
       let allocated = self.allocated_memory_size();
        let discarded = self.discarded_memory_size();
        allocated - discarded
    }
    pub fn set_metric_recorder(&self, histogram_recorder: Box<dyn MetricRecorder + Send + Sync>) {
        *self.metric_recorder_.lock().unwrap() = Some(histogram_recorder);
    }

    pub fn get_metric_recorder(&self) -> Option<&dyn MetricRecorder> {
        let metric_recorder = self.metric_recorder_.lock().unwrap();
        metric_recorder.as_ref().map(|recorder| recorder.as_ref())
    }
}
// POD to hold interesting data accumulated during a garbage collection cycle.
//
// The event is always fully populated when looking at previous events but
// may only be partially populated when looking at the current event.
#[derive(Debug, Clone)]
pub struct Event {
    scope_data: [TimeDelta; ScopeId::NumHistogramScopeIds as usize],
    concurrent_scope_data: [AtomicWord; ConcurrentScopeId::NumHistogramConcurrentScopeIds as usize],
    epoch: usize,
    collection_type: CollectionType,
    marking_type: MarkingType,
    sweeping_type: SweepingType,
    is_forced_gc: IsForcedGC,
    marked_bytes: usize,
    object_size_before_sweep_bytes: usize,
    memory_size_before_sweep_bytes: usize,
}

impl Event {
    fn new() -> Self {
        static EPOCH_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let epoch = EPOCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        Event {
            scope_data: [TimeDelta::FromMilliseconds(0); ScopeId::NumHistogramScopeIds as usize],
            concurrent_scope_data: [0; ConcurrentScopeId::NumHistogramConcurrentScopeIds as usize],
            epoch,
            collection_type: CollectionType::kMajor,
            marking_type: MarkingType::kAtomic,
            sweeping_type: SweepingType::kAtomic,
            is_forced_gc: IsForcedGC::kNotForced,
            marked_bytes: 0,
            object_size_before_sweep_bytes: 0,
            memory_size_before_sweep_bytes: 0,
        }
    }
}
trait AllocationObserver {
    // Called after observing at least
    // StatsCollector::kAllocationThresholdBytes changed bytes through
    // allocation or explicit free. Reports both, negative and positive
    // increments, to allow observer to decide whether absolute values or only
    // the deltas is interesting.
    //
    // May trigger GC.
    fn allocated_object_size_increased(&self, size: usize);
    fn allocated_object_size_decreased(&self, size: usize);

    // Called when the exact size of allocated object size is known. In
    // practice, this is after marking when marked bytes == allocated bytes.
    //
    // Must not trigger GC synchronously.
    fn reset_allocated_object_size(&self, size: usize);

    // Called upon allocating/releasing chunks of memory (e.g. pages) that can
    // contain objects.
    //
    // Must not trigger GC.
    fn allocated_size_increased(&self, size: usize);
    fn allocated_size_decreased(&self, size: usize);
}

// Mock implementation
struct MockAllocationObserver {}

impl AllocationObserver for MockAllocationObserver {
    fn allocated_object_size_increased(&self, size: usize) {
        println!("MockObserver: Allocated object size increased by {}", size);
    }
    fn allocated_object_size_decreased(&self, size: usize) {
        println!("MockObserver: Allocated object size decreased by {}", size);
    }
    fn reset_allocated_object_size(&self, size: usize) {
        println!("MockObserver: Allocated object size reset to {}", size);
    }
    fn allocated_size_increased(&self, size: usize) {
        println!("MockObserver: Allocated size increased by {}", size);
    }
    fn allocated_size_decreased(&self, size: usize) {
        println!("MockObserver: Allocated size decreased by {}", size);
    }
}

//MetricRecorder related structures
#[derive(Debug)]
struct MetricRecorderGCCyclePhases {
    mark_duration_us: i64,
    weak_duration_us: i64,
    compact_duration_us: i64,
    sweep_duration_us: i64,
}

#[derive(Debug)]
enum MetricRecorderGCCycleType {
    kMajor,
    kMinor,
}

#[derive(Debug)]
struct MetricRecorderGCCycle {
    event_type: MetricRecorderGCCycleType,
    main_thread_incremental: MetricRecorderGCCyclePhases,
    main_thread_atomic: MetricRecorderGCCyclePhases,
    main_thread: MetricRecorderGCCyclePhases,
    total: MetricRecorderGCCyclePhases,
    objects_before_bytes: i64,
    objects_after_bytes: i64,
    objects_freed_bytes: i64,
    memory_before_bytes: i64,
    memory_after_bytes: i64,
    memory_freed_bytes: i64,
    collection_rate_in_percent: f64,
    efficiency_in_bytes_per_us: f64,
    main_thread_efficiency_in_bytes_per_us: f64,
}
fn get_cycle_event_for_metric_recorder(
    collection_type: CollectionType,
    marking_type: MarkingType,
    sweeping_type: SweepingType,
    atomic_mark_us: i64,
    atomic_weak_us: i64,
    atomic_compact_us: i64,
    atomic_sweep_us: i64,
    incremental_mark_us: i64,
    incremental_sweep_us: i64,
    concurrent_mark_us: i64,
    concurrent_sweep_us: i64,
    objects_before: i64,
    objects_after: i64,
    objects_freed: i64,
    memory_before: i64,
    memory_after: i64,
    memory_freed: i64,
) -> MetricRecorderGCCycle {
    let event_type = match collection_type {
        CollectionType::kMajor => MetricRecorderGCCycleType::kMajor,
        CollectionType::kMinor => MetricRecorderGCCycleType::kMinor,
    };

    let main_thread_incremental = MetricRecorderGCCyclePhases {
        mark_duration_us: if marking_type != MarkingType::kAtomic {
            incremental_mark_us
        } else {
            -1
        },
        sweep_duration_us: if sweeping_type != SweepingType::kAtomic {
            incremental_sweep_us
        } else {
            -1
        },
        compact_duration_us: 0,
        weak_duration_us: 0,
    };

    let main_thread_atomic = MetricRecorderGCCyclePhases {
        mark_duration_us: atomic_mark_us,
        weak_duration_us: atomic_weak_us,
        compact_duration_us: atomic_compact_us,
        sweep_duration_us: atomic_sweep_us,
    };

    let main_thread = MetricRecorderGCCyclePhases {
        mark_duration_us: main_thread_atomic.mark_duration_us + incremental_mark_us,
        weak_duration_us: main_thread_atomic.weak_duration_us,
        compact_duration_us: main_thread_atomic.compact_duration_us,
        sweep_duration_us: main_thread_atomic.sweep_duration_us + incremental_sweep_us,
    };

    let total = MetricRecorderGCCyclePhases {
        mark_duration_us: main_thread.mark_duration_us + concurrent_mark_us,
        weak_duration_us: main_thread.weak_duration_us,
        compact_duration_us: main_thread.compact_duration_us,
        sweep_duration_us: main_thread.sweep_duration_us + concurrent_sweep_us,
    };

    let collection_rate_in_percent = if objects_before == 0 {
        0.0
    } else {
        objects_freed as f64 / objects_before as f64
    };

    let efficiency_in_bytes_per_us = if objects_freed == 0 {
        0.0
    } else {
        objects_freed as f64 / (total.mark_duration_us + total.weak_duration_us + total.compact_duration_us + total.sweep_duration_us) as f64
    };

    let main_thread_efficiency_in_bytes_per_us = if objects_freed == 0 {
        0.0
    } else {
        objects_freed as f64 / (main_thread.mark_duration_us + main_thread.weak_duration_us + main_thread.compact_duration_us + main_thread.sweep_duration_us) as f64
    };

    MetricRecorderGCCycle {
        event_type,
        main_thread_incremental,
        main_thread_atomic,
        main_thread,
        total,
        objects_before_bytes: objects_before,
        objects_after_bytes: objects_after,
        objects_freed_bytes: objects_freed,
        memory_before_bytes: memory_before,
        memory_after_bytes: memory_after,
        memory_freed_bytes: memory_freed,
        collection_rate_in_percent,
        efficiency_in_bytes_per_us,
        main_thread_efficiency_in_bytes_per_us,
    }
}

use std::sync::atomic::AtomicI64;

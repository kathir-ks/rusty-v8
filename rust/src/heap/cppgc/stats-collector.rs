// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use crate::heap::cppgc::garbage_collector::CollectionType;
use crate::heap::cppgc::gc_config::{GCConfig, IsForcedGC, MarkingType, SweepingType};
use crate::heap::cppgc::metric_recorder::MetricRecorder;
//use v8::base::logging; // No direct equivalent, using println! or logging crate
use crate::base::macros::*; // Adapting macros
//use v8::base::platform::time::TimeDelta; // Using std::time::Duration
use crate::heap::cppgc::trace_event; // No direct equivalent, using tracing or logging crate

mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! CPPGC_FOR_ALL_HISTOGRAM_SCOPES {
            ($V:ident) => {
                $V!(AtomicMark);
                $V!(AtomicWeak);
                $V!(AtomicCompact);
                $V!(AtomicSweep);
                $V!(IncrementalMark);
                $V!(IncrementalSweep);
            };
        }

        #[macro_export]
        macro_rules! CPPGC_FOR_ALL_SCOPES {
            ($V:ident) => {
                $V!(Unmark);
                $V!(MarkIncrementalStart);
                $V!(MarkIncrementalFinalize);
                $V!(MarkAtomicPrologue);
                $V!(MarkAtomicEpilogue);
                $V!(MarkTransitiveClosure);
                $V!(MarkTransitiveClosureWithDeadline);
                $V!(MarkFlushEphemerons);
                $V!(MarkOnAllocation);
                $V!(MarkProcessBailOutObjects);
                $V!(MarkProcessMarkingWorklist);
                $V!(MarkProcessRetraceWorklist);
                $V!(MarkProcessWriteBarrierWorklist);
                $V!(MarkProcessNotFullyconstructedWorklist);
                $V!(MarkProcessEphemerons);
                $V!(MarkVisitRoots);
                $V!(MarkVisitNotFullyConstructedObjects);
                $V!(MarkVisitPersistents);
                $V!(MarkVisitCrossThreadPersistents);
                $V!(MarkVisitStack);
                $V!(MarkVisitRememberedSets);
                $V!(WeakContainerCallbacksProcessing);
                $V!(CustomCallbacksProcessing);
                $V!(SweepEmptyPages);
                $V!(SweepFinish);
                $V!(SweepFinalizeEmptyPages);
                $V!(SweepFinalizeSweptPages);
                $V!(SweepFinishIfOutOfWork);
                $V!(SweepInvokePreFinalizers);
                $V!(SweepInLowPriorityTask);
                $V!(SweepInTask);
                $V!(SweepInTaskForStatistics);
                $V!(SweepOnAllocation);
                $V!(SweepPages);
            };
        }

        #[macro_export]
        macro_rules! CPPGC_FOR_ALL_HISTOGRAM_CONCURRENT_SCOPES {
            ($V:ident) => {
                $V!(ConcurrentMark);
                $V!(ConcurrentSweep);
                $V!(ConcurrentWeakCallback);
            };
        }

        #[macro_export]
        macro_rules! CPPGC_FOR_ALL_CONCURRENT_SCOPES {
            ($V:ident) => {
                $V!(ConcurrentMarkProcessEphemeronWorklist);
                $V!(ConcurrentMarkProcessMarkingWorklist);
                $V!(ConcurrentMarkProcessNotFullyconstructedWorklist);
                $V!(ConcurrentMarkProcessWriteBarrierWorklist);
            };
        }
    }
}

mod heap {
    pub mod cppgc {
        pub mod gc_config {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum IsForcedGC {
                kForced,
                kNotForced,
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum MarkingType {
                kAtomic,
                kIncremental,
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum SweepingType {
                kAtomic,
                kIncremental,
            }

            pub struct GCConfig {}
        }

        pub mod garbage_collector {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
            pub enum CollectionType {
                kMajor,
                #[default]
                kMinor,
            }
        }

        pub mod metric_recorder {
            pub struct MetricRecorder {}
        }

        pub mod trace_event {
            // Placeholders for trace events. Can be replaced with tracing crate or logging.
        }

        pub mod stats_collector {
            use super::*;
            use std::sync::{Arc, Mutex};

            macro_rules! declare_enums {
                ($macro:ident, $name:ident, $prefix:ident) => {
                    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
                    #[allow(non_camel_case_types)]
                    pub enum $name {
                        $macro!(declare_variant),
                    }
                };
            }

            macro_rules! declare_variant {
                ($name:ident) => {
                    $name,
                };
            }

            macro_rules! count_variants {
                ($macro:ident, $enum_name:ident) => {
                    {
                        let mut count = 0;
                        $macro!(count_variant);
                        count
                    }
                };
            }

            macro_rules! count_variant {
                ($name:ident) => {
                    const _: () = {
                        static mut COUNT: usize = 0;
                        unsafe { COUNT += 1; }
                    };
                }
            }

            macro_rules! define_enums {
                () => {
                    macro_rules! CPPGC_DECLARE_ENUM {
                        ($name:ident) => {
                            $name,
                        };
                    }

                    CPPGC_FOR_ALL_HISTOGRAM_SCOPES!(CPPGC_DECLARE_ENUM);
                    const kNumHistogramScopeIds: usize = {
                        let mut count = 0;
                        macro_rules! count {
                            ($name:ident) => {
                                const _: () = {
                                    static mut COUNT: usize = 0;
                                    unsafe { COUNT += 1; }
                                };
                            };
                        }
                        CPPGC_FOR_ALL_HISTOGRAM_SCOPES!(count);
                        unsafe {
                            count = crate::heap::cppgc::stats_collector::COUNT;
                            crate::heap::cppgc::stats_collector::COUNT = 0;
                        }
                        count
                    };

                    CPPGC_FOR_ALL_SCOPES!(CPPGC_DECLARE_ENUM);
                    const kNumScopeIds: usize = {
                        let mut count = 0;
                        macro_rules! count {
                            ($name:ident) => {
                                const _: () = {
                                    static mut COUNT: usize = 0;
                                    unsafe { COUNT += 1; }
                                };
                            };
                        }
                        CPPGC_FOR_ALL_SCOPES!(count);
                        unsafe {
                            count = crate::heap::cppgc::stats_collector::COUNT;
                            crate::heap::cppgc::stats_collector::COUNT = 0;
                        }
                        count
                    };

                    CPPGC_FOR_ALL_HISTOGRAM_CONCURRENT_SCOPES!(CPPGC_DECLARE_ENUM);
                    const kNumHistogramConcurrentScopeIds: usize = {
                        let mut count = 0;
                        macro_rules! count {
                            ($name:ident) => {
                                const _: () = {
                                    static mut COUNT: usize = 0;
                                    unsafe { COUNT += 1; }
                                };
                            };
                        }
                        CPPGC_FOR_ALL_HISTOGRAM_CONCURRENT_SCOPES!(count);
                        unsafe {
                            count = crate::heap::cppgc::stats_collector::COUNT;
                            crate::heap::cppgc::stats_collector::COUNT = 0;
                        }
                        count
                    };

                    CPPGC_FOR_ALL_CONCURRENT_SCOPES!(CPPGC_DECLARE_ENUM);
                    const kNumConcurrentScopeIds: usize = {
                        let mut count = 0;
                        macro_rules! count {
                            ($name:ident) => {
                                const _: () = {
                                    static mut COUNT: usize = 0;
                                    unsafe { COUNT += 1; }
                                };
                            };
                        }
                        CPPGC_FOR_ALL_CONCURRENT_SCOPES!(count);
                        unsafe {
                            count = crate::heap::cppgc::stats_collector::COUNT;
                            crate::heap::cppgc::stats_collector::COUNT = 0;
                        }
                        count
                    };
                }
            }

            define_enums!();

            static mut COUNT: usize = 0;

            #[allow(non_camel_case_types)]
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub enum ScopeId {
                AtomicMark,
                AtomicWeak,
                AtomicCompact,
                AtomicSweep,
                IncrementalMark,
                IncrementalSweep,
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
            }
            impl ScopeId {
                const COUNT: usize = 41;
            }

            #[allow(non_camel_case_types)]
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub enum ConcurrentScopeId {
                ConcurrentMark,
                ConcurrentSweep,
                ConcurrentWeakCallback,
                ConcurrentMarkProcessEphemeronWorklist,
                ConcurrentMarkProcessMarkingWorklist,
                ConcurrentMarkProcessNotFullyconstructedWorklist,
                ConcurrentMarkProcessWriteBarrierWorklist,
            }
            impl ConcurrentScopeId {
                const COUNT: usize = 7;
            }
            

            /// POD to hold interesting data accumulated during a garbage collection cycle.
            ///
            /// The event is always fully populated when looking at previous events but
            /// may only be partially populated when looking at the current event.
            #[derive(Default)]
            pub struct Event {
                pub scope_data: [Duration; 6], //kNumHistogramScopeIds
                pub concurrent_scope_data: [AtomicI64; 3], //kNumHistogramConcurrentScopeIds
                pub epoch: usize,
                pub collection_type: CollectionType,
                pub marking_type: MarkingType,
                pub sweeping_type: SweepingType,
                pub is_forced_gc: IsForcedGC,
                // Marked bytes collected during marking.
                pub marked_bytes: usize,
                pub object_size_before_sweep_bytes: usize,
                pub memory_size_before_sweep_bytes: usize,
            }

            impl Event {
                pub fn new() -> Self {
                    Event {
                        epoch: usize::MAX,
                        object_size_before_sweep_bytes: usize::MAX,
                        memory_size_before_sweep_bytes: usize::MAX,
                        ..Default::default()
                    }
                }
            }

            /// Sink for various time and memory statistics.
            pub struct StatsCollector {
                allocated_bytes_since_end_of_marking_: AtomicI64,
                time_of_last_end_of_marking_: Mutex<Instant>,
                allocated_bytes_since_safepoint_: AtomicI64,
                explicitly_freed_bytes_since_safepoint_: AtomicI64,
                #[cfg(feature = "cppgc_verify_heap")]
                tracked_live_bytes_: AtomicUsize,
                marked_bytes_so_far_: AtomicUsize,
                memory_allocated_bytes_: AtomicI64,
                memory_freed_bytes_since_end_of_marking_: AtomicI64,
                discarded_bytes_: AtomicUsize,
                allocation_observers_: Mutex<Vec<Box<dyn AllocationObserver>>>,
                allocation_observer_deleted_: Mutex<bool>,
                gc_state_: Mutex<GarbageCollectionState>,
                current_: Mutex<Event>,
                previous_: Mutex<Event>,
                metric_recorder_: Mutex<Option<MetricRecorder>>,
                platform_: (), //Platform*, // Assuming Platform is empty struct
            }

            impl StatsCollector {
                pub const K_ALLOCATION_THRESHOLD_BYTES: usize = 1024;

                pub fn new() -> Self {
                    StatsCollector {
                        allocated_bytes_since_end_of_marking_: AtomicI64::new(0),
                        time_of_last_end_of_marking_: Mutex::new(Instant::now()),
                        allocated_bytes_since_safepoint_: AtomicI64::new(0),
                        explicitly_freed_bytes_since_safepoint_: AtomicI64::new(0),
                        #[cfg(feature = "cppgc_verify_heap")]
                        tracked_live_bytes_: AtomicUsize::new(0),
                        marked_bytes_so_far_: AtomicUsize::new(0),
                        memory_allocated_bytes_: AtomicI64::new(0),
                        memory_freed_bytes_since_end_of_marking_: AtomicI64::new(0),
                        discarded_bytes_: AtomicUsize::new(0),
                        allocation_observers_: Mutex::new(Vec::new()),
                        allocation_observer_deleted_: Mutex::new(false),
                        gc_state_: Mutex::new(GarbageCollectionState::KNotRunning),
                        current_: Mutex::new(Event::new()),
                        previous_: Mutex::new(Event::new()),
                        metric_recorder_: Mutex::new(None),
                        platform_: (),
                    }
                }

                pub fn register_observer(&self, observer: Box<dyn AllocationObserver>) {
                    self.allocation_observers_.lock().unwrap().push(observer);
                }

                pub fn unregister_observer(&self, observer: &dyn AllocationObserver) {
                    let mut observers = self.allocation_observers_.lock().unwrap();
                    observers.retain(|x| !std::ptr::eq(x.as_ref(), observer));
                }

                pub fn notify_allocation(&self, size: usize) {
                    let mut allocated_bytes = self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed);
                    allocated_bytes += size as i64;
                    self.allocated_bytes_since_end_of_marking_.store(allocated_bytes, Ordering::Relaxed);

                    let mut bytes_since_safepoint = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed);
                    bytes_since_safepoint += size as i64;
                    self.allocated_bytes_since_safepoint_.store(bytes_since_safepoint, Ordering::Relaxed);
                    self.AllocatedObjectSizeSafepointImpl();
                }

                pub fn notify_explicit_free(&self, size: usize) {
                    let mut allocated_bytes = self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed);
                    allocated_bytes -= size as i64;
                    self.allocated_bytes_since_end_of_marking_.store(allocated_bytes, Ordering::Relaxed);

                    let mut freed_bytes = self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);
                    freed_bytes += size as i64;
                    self.explicitly_freed_bytes_since_safepoint_.store(freed_bytes, Ordering::Relaxed);
                    self.AllocatedObjectSizeSafepointImpl();
                }

                pub fn notify_safe_point_for_conservative_collection(&self) {
                    self.AllocatedObjectSizeSafepointImpl();
                }

                pub fn notify_safe_point_for_testing(&self) {
                    self.AllocatedObjectSizeSafepointImpl();
                }

                pub fn notify_unmarking_started(&self, collection_type: CollectionType) {
                    let mut gc_state = self.gc_state_.lock().unwrap();
                    *gc_state = GarbageCollectionState::KUnmarking;

                    let mut current = self.current_.lock().unwrap();
                    current.collection_type = collection_type;
                    current.epoch += 1;
                }

                pub fn notify_marking_started(&self, collection_type: CollectionType, marking_type: MarkingType, is_forced_gc: IsForcedGC) {
                    let mut gc_state = self.gc_state_.lock().unwrap();
                    *gc_state = GarbageCollectionState::KMarking;

                    let mut current = self.current_.lock().unwrap();
                    current.collection_type = collection_type;
                    current.marking_type = marking_type;
                    current.is_forced_gc = is_forced_gc;
                }

                pub fn notify_marking_completed(&self, marked_bytes: usize) {
                    let mut current = self.current_.lock().unwrap();
                    current.marked_bytes = marked_bytes;
                }

                pub fn notify_sweeping_completed(&self, sweeping_type: SweepingType) {
                    let mut gc_state = self.gc_state_.lock().unwrap();
                    *gc_state = GarbageCollectionState::KSweeping;

                    let mut current = self.current_.lock().unwrap();
                    current.sweeping_type = sweeping_type;
                    let mut previous = self.previous_.lock().unwrap();
                    *previous = std::mem::take(&mut *current);

                    *gc_state = GarbageCollectionState::KNotRunning;
                }

                pub fn allocated_memory_size(&self) -> usize {
                    self.memory_allocated_bytes_.load(Ordering::Relaxed) as usize
                }

                pub fn allocated_object_size(&self) -> usize {
                    let marked_bytes = self.marked_bytes();
                    let allocated_bytes = self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed);
                    (marked_bytes as i64 + allocated_bytes) as usize
                }

                pub fn marked_bytes(&self) -> usize {
                    self.marked_bytes_so_far_.load(Ordering::Relaxed)
                }

                pub fn marked_bytes_on_current_cycle(&self) -> usize {
                    let current = self.current_.lock().unwrap();
                    current.marked_bytes
                }

                pub fn marking_time(&self) -> Duration {
                    let previous = self.previous_.lock().unwrap();
                    previous.scope_data.iter().sum()
                }

                pub fn get_recent_allocation_speed_in_bytes_per_ms(&self) -> f64 {
                    let last_end_time = *self.time_of_last_end_of_marking_.lock().unwrap();
                    let elapsed = last_end_time.elapsed();
                    let allocated_bytes = self.allocated_bytes_since_end_of_marking_.load(Ordering::Relaxed);
                    if elapsed.as_millis() > 0 {
                        allocated_bytes as f64 / elapsed.as_millis() as f64
                    } else {
                        0.0
                    }
                }

                pub fn get_previous_event_for_testing(&self) -> Event {
                    self.previous_.lock().unwrap().clone()
                }

                pub fn notify_allocated_memory(&self, size: i64) {
                    self.memory_allocated_bytes_.fetch_add(size, Ordering::Relaxed);
                }

                pub fn notify_freed_memory(&self, size: i64) {
                    self.memory_freed_bytes_since_end_of_marking_.fetch_add(size, Ordering::Relaxed);
                }

                pub fn increment_discarded_memory(&self, size: usize) {
                    self.discarded_bytes_.fetch_add(size, Ordering::Relaxed);
                }

                pub fn decrement_discarded_memory(&self, size: usize) {
                    self.discarded_bytes_.fetch_sub(size, Ordering::Relaxed);
                }

                pub fn reset_discarded_memory(&self) {
                    self.discarded_bytes_.store(0, Ordering::Relaxed);
                }

                pub fn discarded_memory_size(&self) -> usize {
                    self.discarded_bytes_.load(Ordering::Relaxed)
                }

                pub fn resident_memory_size(&self) -> usize {
                    self.memory_allocated_bytes_.load(Ordering::Relaxed) as usize - self.memory_freed_bytes_since_end_of_marking_.load(Ordering::Relaxed) as usize - self.discarded_bytes_.load(Ordering::Relaxed)
                }

                pub fn set_metric_recorder(&self, metric_recorder: Option<MetricRecorder>) {
                    *self.metric_recorder_.lock().unwrap() = metric_recorder;
                }

                pub fn get_metric_recorder(&self) -> Option<&MetricRecorder> {
                    self.metric_recorder_.lock().unwrap().as_ref()
                }

                fn record_histogram_sample(&self, id: ScopeId, time: Duration) {
                    //println!("record_histogram_sample {:?} {:?}", id, time);
                }

                fn AllocatedObjectSizeSafepointImpl(&self) {
                    let allocated_bytes_since_safepoint = self.allocated_bytes_since_safepoint_.load(Ordering::Relaxed);
                    let explicitly_freed_bytes_since_safepoint = self.explicitly_freed_bytes_since_safepoint_.load(Ordering::Relaxed);
                    let delta = allocated_bytes_since_safepoint - explicitly_freed_bytes_since_safepoint;
                    if delta.abs() as usize >= StatsCollector::K_ALLOCATION_THRESHOLD_BYTES {
                        let observers = self.allocation_observers_.lock().unwrap();
                        for observer in observers.iter() {
                            if delta > 0 {
                                observer.AllocatedObjectSizeIncreased(delta as usize);
                            } else {
                                observer.AllocatedObjectSizeDecreased((-delta) as usize);
                            }
                        }
                        self.allocated_bytes_since_safepoint_.store(0, Ordering::Relaxed);
                        self.explicitly_freed_bytes_since_safepoint_.store(0, Ordering::Relaxed);
                    }
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            enum GarbageCollectionState {
                KNotRunning,
                KUnmarking,
                KMarking,
                KSweeping,
            }

            /// Observer for allocated object size.
            pub trait AllocationObserver {
                /// Called after observing at least
                /// StatsCollector::kAllocationThresholdBytes changed bytes through
                /// allocation or explicit free. Reports both, negative and positive
                /// increments, to allow observer to decide whether absolute values or only
                /// the deltas is interesting.
                ///
                /// May trigger GC.
                fn AllocatedObjectSizeIncreased(&self, size: usize) {}
                fn AllocatedObjectSizeDecreased(&self, size: usize) {}

                /// Called when the exact size of allocated object size is known. In
                /// practice, this is after marking when marked bytes == allocated bytes.
                ///
                /// Must not trigger GC synchronously.
                fn ResetAllocatedObjectSize(&self, size: usize) {}

                /// Called upon allocating/releasing chunks of memory (e.g. pages) that can
                /// contain objects.
                ///
                /// Must not trigger GC.
                fn AllocatedSizeIncreased(&self, size: usize) {}
                fn AllocatedSizeDecreased(&self, size: usize) {}
            }

            enum TraceCategory {
                kEnabled,
                kDisabled,
            }
            enum ScopeContext {
                kMutatorThread,
                kConcurrentThread,
            }
        }
    }
}
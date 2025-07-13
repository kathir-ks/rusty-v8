// Converted from V8 C++ source files:
// Header: heap-growing.h
// Implementation: heap-growing.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

use std::cell::RefCell;
use std::rc::Rc;

const kPageSize: usize = 4096;
pub struct GarbageCollector {}
pub struct StatsCollector {}

pub enum class HeapSweepingType {
    kSupportsConcurrentSweeping,
    kSupportsLazySweeping,
    kNoSweeping,
}

pub enum class HeapMarkingType {
    kAtomic,
    kIncremental,
}
    

pub struct Heap {
        }
        impl Heap {
            pub struct ResourceConstraints {
                pub initial_heap_size_bytes: usize,
            }
            pub enum class MarkingType {
                kAtomic,
                kIncremental,
            }
            pub enum class SweepingType {
                kSupportsConcurrentSweeping,
                kSupportsLazySweeping,
                kNoSweeping,
            }
        }
        
pub enum class CollectionType {
    kMajor,
    kMinor,
}

pub enum class StackState {
    kMayContainHeapPointers,
    kNoHeapPointers,
}

pub struct GCConfig {
    pub marking_type: GCConfigMarkingType,
}

pub enum class GCConfigMarkingType {
    kAtomic,
    kIncremental,
}

pub struct SingleThreadedHandle {
    _non_empty_tag: SingleThreadedHandleNonEmptyTag,
}
impl SingleThreadedHandle{
    pub struct SingleThreadedHandleNonEmptyTag{}
    
    pub fn NonEmptyTag() -> SingleThreadedHandleNonEmptyTag{
        SingleThreadedHandleNonEmptyTag{}
    }
}

const kMB: usize = 1024 * 1024;

// Growing strategy that invokes garbage collection using GarbageCollector based
// on allocation statistics provided by StatsCollector and ResourceConstraints.
//
// Implements a fixed-ratio growing strategy with an initial heap size that the
// GC can ignore to avoid excessive GCs for smaller heaps.
pub struct HeapGrowing {
    impl_: Box<HeapGrowingImpl>,
}

impl HeapGrowing {
    // Constant growing factor for growing the heap limit.
    pub const K_GROWING_FACTOR: f64 = 1.5;
    // For smaller heaps, allow allocating at least LAB in each regular space
    // before triggering GC again.
    pub const K_MIN_LIMIT_INCREASE: usize =
        kPageSize * RawHeap::K_NUMBER_OF_REGULAR_SPACES;

    pub fn new(
        collector: *mut GarbageCollector,
        stats_collector: *mut StatsCollector,
        constraints: Heap::ResourceConstraints,
        marking_support: Heap::MarkingType,
        sweeping_support: Heap::SweepingType,
    ) -> Self {
        HeapGrowing {
            impl_: HeapGrowingImpl::new(collector, stats_collector, constraints, marking_support, sweeping_support),
        }
    }

    pub fn limit_for_atomic_gc(&self) -> usize {
        self.impl_.limit_for_atomic_gc()
    }
    pub fn limit_for_incremental_gc(&self) -> usize {
        self.impl_.limit_for_incremental_gc()
    }

    pub fn disable_for_testing(&mut self) {
        self.impl_.disable_for_testing();
    }
}

struct HeapGrowingImpl {
    collector_: *mut GarbageCollector,
    stats_collector_: *mut StatsCollector,
    // Allow 1 MB heap by default;
    initial_heap_size_: usize,
    limit_for_atomic_gc_: usize,       // See ConfigureLimit().
    limit_for_incremental_gc_: usize,  // See ConfigureLimit().

    gc_task_handle_: SingleThreadedHandle,

    disabled_for_testing_: bool,

    marking_support_: Heap::MarkingType,
    sweeping_support_: Heap::SweepingType,
}

impl HeapGrowingImpl {
    fn new(
        collector: *mut GarbageCollector,
        stats_collector: *mut StatsCollector,
        constraints: Heap::ResourceConstraints,
        marking_support: Heap::MarkingType,
        sweeping_support: Heap::SweepingType,
    ) -> Box<Self> {
        let mut this = Box::new(HeapGrowingImpl {
            collector_: collector,
            stats_collector_: stats_collector,
            initial_heap_size_: if constraints.initial_heap_size_bytes > 0 {
                constraints.initial_heap_size_bytes
            } else {
                1 * kMB
            },
            limit_for_atomic_gc_: 0,
            limit_for_incremental_gc_: 0,
            gc_task_handle_: SingleThreadedHandle{ _non_empty_tag: SingleThreadedHandle::NonEmptyTag()},
            disabled_for_testing_: false,
            marking_support_: marking_support,
            sweeping_support_: sweeping_support,
        });
        let k_no_allocated_bytes: usize = 0;
        this.configure_limit(k_no_allocated_bytes);
        //stats_collector.register_observer(&*this);
        Box::new(*this)
    }
    

    fn allocated_object_size_increased(&mut self) {
        if self.disabled_for_testing_ {
            return;
        }
        //let allocated_object_size = self.stats_collector_.allocated_object_size();
        let allocated_object_size = 1000000; //stats_collector_->allocated_object_size(); //FIXME
        if allocated_object_size > self.limit_for_atomic_gc_ {
            unsafe {
                (*self.collector_).CollectGarbage({
                    CollectionType::kMajor,
                    StackState::kMayContainHeapPointers,
                    GCConfig::MarkingType::kAtomic,
                    self.sweeping_support_,
                });
            }
        } else if allocated_object_size > self.limit_for_incremental_gc_ {
            if self.marking_support_ == Heap::MarkingType::kAtomic {
                return;
            }
            unsafe {
                (*self.collector_).StartIncrementalGarbageCollection({
                    CollectionType::kMajor,
                    StackState::kMayContainHeapPointers,
                    self.marking_support_,
                    self.sweeping_support_,
                });
            }
        }
    }

    fn allocated_object_size_decreased(&mut self, _size: usize) {}
    fn reset_allocated_object_size(&mut self, allocated_object_size: usize) {
        self.configure_limit(allocated_object_size);
    }

    fn limit_for_atomic_gc(&self) -> usize {
        self.limit_for_atomic_gc_
    }
    fn limit_for_incremental_gc(&self) -> usize {
        self.limit_for_incremental_gc_
    }

    fn disable_for_testing(&mut self) {
        self.disabled_for_testing_ = true;
    }

    fn configure_limit(&mut self, allocated_object_size: usize) {
        let size = std::cmp::max(allocated_object_size, self.initial_heap_size_);
        self.limit_for_atomic_gc_ = std::cmp::max(
            (size as f64 * HeapGrowing::K_GROWING_FACTOR) as usize,
            size + HeapGrowing::K_MIN_LIMIT_INCREASE,
        );
        // Estimate when to start incremental GC based on current allocation speed.
        // Ideally we start incremental GC such that it is ready to finalize no
        // later than when we reach |limit_for_atomic_gc_|. However, we need to cap
        // |limit_for_incremental_gc_| within a range to prevent:
        // 1) |limit_for_incremental_gc_| being too close to |limit_for_atomic_gc_|
        //    such that incremental gc gets nothing done before reaching
        //    |limit_for_atomic_gc_| (in case where the allocation rate is very low).
        // 2) |limit_for_incremental_gc_| being too close to |size| such that GC is
        //    essentially always running and write barriers are always active (in
        //    case allocation rate is very high).
        let estimated_bytes_allocated_during_incremental_gc =
            (IncrementalMarkingSchedule::K_ESTIMATED_MARKING_TIME.in_milliseconds_f() * 10.0).ceil() as usize;
            //(IncrementalMarkingSchedule::K_ESTIMATED_MARKING_TIME.in_milliseconds_f() *
            //    self.stats_collector_.get_recent_allocation_speed_in_bytes_per_ms())
            //    .ceil() as usize;
        let limit_incremental_gc_based_on_allocation_rate =
            self.limit_for_atomic_gc_ - estimated_bytes_allocated_during_incremental_gc;
        let maximum_limit_incremental_gc =
            size + ((self.limit_for_atomic_gc_ - size) as f64 * K_MAXIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC) as usize;
        let minimum_limit_incremental_gc =
            size + ((self.limit_for_atomic_gc_ - size) as f64 * K_MINIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC) as usize;
        self.limit_for_incremental_gc_ = std::cmp::max(
            minimum_limit_incremental_gc,
            std::cmp::min(
                maximum_limit_incremental_gc,
                limit_incremental_gc_based_on_allocation_rate,
            ),
        );
    }
}

const K_MAXIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC: f64 = 0.9;
const K_MINIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC: f64 = 0.5;

struct IncrementalMarkingSchedule {}
impl IncrementalMarkingSchedule {
    const K_ESTIMATED_MARKING_TIME: Duration = Duration { milliseconds: 10 };
}

struct Duration {
    milliseconds: i32,
}

impl Duration {
    fn in_milliseconds_f(&self) -> f64 {
        self.milliseconds as f64
    }
}

pub trait AllocationObserver {
    fn allocated_object_size_increased(&mut self, size: usize);
    fn allocated_object_size_decreased(&mut self, size: usize);
    fn reset_allocated_object_size(&mut self, size: usize);
}

impl Drop for HeapGrowingImpl {
    fn drop(&mut self) {
        //stats_collector_->UnregisterObserver(self);
    }
}
}  // namespace internal
}  // namespace cppgc

pub mod base {
    pub mod macros {

    }
}
pub mod heap {
    pub mod base {
        pub struct IncrementalMarkingSchedule{}
        impl IncrementalMarkingSchedule{
            pub struct kEstimatedMarkingTime{}
        }
    }
}
pub mod include {
    pub mod cppgc {
        pub struct Platform{}
    }
}

pub mod src {
    pub mod heap {
        pub mod cppgc {
            pub mod task_handle {
                
            }
        }
    }
}

pub mod heap {
    pub mod cppgc {
        pub mod globals{

        }
    }
}

pub mod heap {
    pub mod cppgc {
        pub struct RawHeap{}
        impl RawHeap{
           pub const K_NUMBER_OF_REGULAR_SPACES: usize = 10;
        }
    }
}

pub trait CollectGarbage{
    fn CollectGarbage(&mut self, config: CollectParams);
    fn StartIncrementalGarbageCollection(&mut self, config: CollectParams);
}

pub struct CollectParams{
    collection_type: cppgc::internal::CollectionType,
    stack_state: cppgc::internal::StackState,
    marking_type: cppgc::internal::GCConfigMarkingType,
    sweeping_support : cppgc::Heap::SweepingType
}

impl GarbageCollector{
    pub fn CollectGarbage(&mut self, config: cppgc::internal::CollectParams){

    }
    pub fn StartIncrementalGarbageCollection(&mut self, config: cppgc::internal::CollectParams){

    }
}

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add equivalent Rust crates for any C++ libraries used

// pub mod flags;
// pub mod base;
// pub mod common;
// pub mod execution;
// pub mod heap;
// pub mod objects;

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::cmp;
// use std::memory;
use std::collections::HashSet;
// use crate::base::atomic_utils;
// use crate::common::globals::*;
// use crate::execution::vm_state::*;
// use crate::flags::flags::*;
// use crate::heap::base::active_system_pages::ActiveSystemPages;
// use crate::heap::ephemeron_remembered_set::EphemeronRememberedSet;
// use crate::heap::free_list::*;
// use crate::heap::gc_tracer::*;
// use crate::heap::heap_layout::*;
// use crate::heap::heap::*;
// use crate::heap::live_object_range::*;
// use crate::heap::mark_compact::*;
// use crate::heap::marking_state::*;
// use crate::heap::memory_allocator::*;
// use crate::heap::memory_chunk_layout::*;
// use crate::heap::mutable_page_metadata::*;
// use crate::heap::new_spaces::*;
// use crate::heap::page_metadata::*;
// use crate::heap::paged_spaces::*;
// use crate::heap::pretenuring_handler::*;
// use crate::heap::remembered_set::*;
// use crate::heap::slot_set::*;
// use crate::heap::zapping::*;
// use crate::objects::hash_table::*;
// use crate::objects::instance_type::*;
// use crate::objects::js_array_buffer::*;
// use crate::objects::map::*;
// use crate::objects::objects::*;

// Placeholder enums/structs to enable compilation

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AllocationSpace {
    NEW_SPACE,
    OLD_SPACE,
    CODE_SPACE,
	LO_SPACE,
    SHARED_SPACE,
    TRUSTED_SPACE,
    SHARED_TRUSTED_SPACE,
    FIRST_SWEEPABLE_SPACE,
    LAST_SWEEPABLE_SPACE,
}

// impl AllocationSpace {
//     fn as_usize(&self) -> usize {
//         match self {
//             AllocationSpace::NEW_SPACE => 0,
//             AllocationSpace::OLD_SPACE => 1,
//             AllocationSpace::CODE_SPACE => 2,
// 			AllocationSpace::LO_SPACE => 3,
//             AllocationSpace::SHARED_SPACE => 4,
//             AllocationSpace::TRUSTED_SPACE => 5,
//             AllocationSpace::SHARED_TRUSTED_SPACE => 6,
//             AllocationSpace::FIRST_SWEEPABLE_SPACE => 7,
//             AllocationSpace::LAST_SWEEPABLE_SPACE => 8,
//         }
//     }
// }

const NEW_SPACE: AllocationSpace = AllocationSpace::NEW_SPACE;
const OLD_SPACE: AllocationSpace = AllocationSpace::OLD_SPACE;
const CODE_SPACE: AllocationSpace = AllocationSpace::CODE_SPACE;
const LO_SPACE: AllocationSpace = AllocationSpace::LO_SPACE;
const SHARED_SPACE: AllocationSpace = AllocationSpace::SHARED_SPACE;
const TRUSTED_SPACE: AllocationSpace = AllocationSpace::TRUSTED_SPACE;
const SHARED_TRUSTED_SPACE: AllocationSpace = AllocationSpace::SHARED_TRUSTED_SPACE;
const FIRST_SWEEPABLE_SPACE: AllocationSpace = AllocationSpace::FIRST_SWEEPABLE_SPACE;
const LAST_SWEEPABLE_SPACE: AllocationSpace = AllocationSpace::LAST_SWEEPABLE_SPACE;

const kNumberOfSweepingSpaces: usize = 7;

fn IsValidSweepingSpace(identity: AllocationSpace) -> bool {
	match identity {
		AllocationSpace::NEW_SPACE | AllocationSpace::OLD_SPACE | AllocationSpace::CODE_SPACE | AllocationSpace::SHARED_SPACE | AllocationSpace::TRUSTED_SPACE | AllocationSpace::SHARED_TRUSTED_SPACE | AllocationSpace::LO_SPACE => true,
		_ => false,
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SweepingMode {
    kLazyOrConcurrent,
    kEagerDuringGC,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FreeSpaceTreatmentMode {
	kZapFreeSpace,
	kIgnoreFreeSpace,
}

struct Sweeper {
    heap_: *mut Heap, // Raw pointer to avoid lifetime issues with shared mutable state.  Consider using a smart pointer if safety can be guaranteed
    marking_state_: *mut NonAtomicMarkingState,
    main_thread_local_sweeper_: LocalSweeper,
	mutex_: Arc<Mutex<()>>,
	cv_page_swept_: Arc<Condvar>,
	promoted_pages_iteration_notification_mutex_: Mutex<()>,
	promoted_pages_iteration_notification_variable_: Condvar,
	promoted_page_iteration_in_progress_: AtomicBool,
	sweeping_list_: [Vec<*mut PageMetadata>; kNumberOfSweepingSpaces],
    sweeping_list_for_promoted_page_iteration_: Vec<*mut MutablePageMetadata>,
	has_sweeping_work_: [AtomicBool; kNumberOfSweepingSpaces],
	swept_list_: [Vec<*mut PageMetadata>; kNumberOfSweepingSpaces],
	has_swept_pages_: [AtomicBool; kNumberOfSweepingSpaces],
	minor_sweeping_state_: SweepingState<SweepingScope::kMinor>,
	major_sweeping_state_: SweepingState<SweepingScope::kMajor>,
    promoted_pages_for_iteration_count_: usize,
    iterated_promoted_pages_count_: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SweepingScope {
    kMajor,
    kMinor,
}

struct SweepingState<const scope: SweepingScope> {
    sweeper_: *mut Sweeper,
	in_progress_: bool,
	concurrent_sweepers_: Vec<ConcurrentSweeperTraitObject>,
	job_handle_: Option<Box<dyn JobHandleTrait>>,
	should_reduce_memory_: bool,
	trace_id_: u64,
	background_trace_id_: u64,
}

trait JobTaskTrait {
    fn run(&mut self, delegate: &mut dyn JobDelegate) -> ();
    fn get_max_concurrency(&self, worker_count: usize) -> usize;
}

trait JobDelegate {
    fn should_yield(&self) -> bool;
    fn get_task_id(&self) -> usize;
    fn is_joining_thread(&self) -> bool;
}

trait JobHandleTrait {
    fn is_valid(&self) -> bool;
    fn is_active(&self) -> bool;
    fn cancel(&mut self) -> ();
    fn join(&mut self) -> ();
}

trait PlatformTrait {
    fn number_of_worker_threads(&self) -> usize;
    fn post_job(&self, priority: TaskPriority, job: Box<dyn JobTaskTrait>) -> Box<dyn JobHandleTrait>;
}

enum TaskPriority {
    kUserVisible,
}

struct MajorSweeperJob {
    sweeper_: *mut Sweeper,
    concurrent_sweepers: Vec<ConcurrentMajorSweeper>,
    tracer_: *mut GCTracer,
    trace_id_: u64,
}

impl JobTaskTrait for MajorSweeperJob {
    fn run(&mut self, delegate: &mut dyn JobDelegate) -> () {
		unsafe {
			let sweeper = &mut *self.sweeper_;
        	self.run_impl(delegate, delegate.is_joining_thread());
		}
    }

    fn get_max_concurrency(&self, worker_count: usize) -> usize {
		unsafe {
			let sweeper = &mut *self.sweeper_;
        let kPagePerTask: usize = 2;
        cmp::min(
            self.concurrent_sweepers.len(),
            worker_count
                + (sweeper.ConcurrentMajorSweepingPageCount() + kPagePerTask - 1) / kPagePerTask,
        ) as usize
		}
    }
}

impl MajorSweeperJob {
    const kMaxTasks: usize = kNumberOfSweepingSpaces - 1;

    fn new(isolate: *mut Isolate, sweeper: *mut Sweeper) -> Self {
		unsafe {
			let sweeper_ref = &mut *sweeper;
        assert!(sweeper_ref.major_sweeping_state_.concurrent_sweepers_.len() <= Self::kMaxTasks);
        Self {
            sweeper: sweeper,
            concurrent_sweepers: sweeper_ref.major_sweeping_state_.concurrent_sweepers_.iter().map(|x| x.downcast_ref::<ConcurrentMajorSweeper>().unwrap().clone()).collect(),
            tracer_: (&mut *isolate).heap_.tracer(),
            trace_id_: sweeper_ref.major_sweeping_state_.background_trace_id_,
        }
		}
    }

    fn run_impl(&mut self, delegate: &mut dyn JobDelegate, is_joining_thread: bool) {
		unsafe {
			let sweeper = &mut *self.sweeper_;
			let heap = &mut *sweeper.heap_;
			let isolate = &mut *heap.isolate_;

        // In case multi-cage pointer compression mode is enabled ensure that
        // current thread's cage base values are properly initialized.
        let ptr_compr_cage_access_scope = PtrComprCageAccessScope::new(isolate);

        assert!(sweeper.major_sweeping_in_progress());
        let offset = delegate.get_task_id();
        assert!(offset < self.concurrent_sweepers.len());
        let concurrent_sweeper = &mut self.concurrent_sweepers[offset];
        // let concurrent_sweeper = &mut *self.concurrent_sweepers[offset];
        // TRACE_GC_EPOCH_WITH_FLOW(
        //     self.tracer_,
        //     sweeper.GetTracingScope(OLD_SPACE, is_joining_thread),
        //     if is_joining_thread {
        //         ThreadKind::kMain
        //     } else {
        //         ThreadKind::kBackground
        //     },
        //     self.trace_id_,
        //     TRACE_EVENT_FLAG_FLOW_IN,
        // );
        for i in 0..MajorSweeperJob::kMaxTasks {
            let space_id = unsafe {
				let offset_usize = offset as usize;
				let i_usize = i as usize;
				let kNumberOfMajorSweepingSpaces_usize = (kNumberOfSweepingSpaces - 1) as usize;
                let space_id_usize = (FIRST_SWEEPABLE_SPACE as usize + 1 + ((offset_usize + i_usize) % kNumberOfMajorSweepingSpaces_usize)) as usize;
                // std::mem::transmute::<usize, AllocationSpace>(space_id_usize)
				match space_id_usize {
					7 => AllocationSpace::FIRST_SWEEPABLE_SPACE,
					8 => AllocationSpace::LAST_SWEEPABLE_SPACE,
					_ => panic!("unexpected space id"),
				}
            };
            assert!(FIRST_SWEEPABLE_SPACE <= space_id);
            assert!(space_id <= LAST_SWEEPABLE_SPACE);
            assert!(space_id != NEW_SPACE);
            if !concurrent_sweeper.ConcurrentSweepSpace(space_id, delegate) {
                return;
            }
        }
		}
    }
}

struct MinorSweeperJob {
    sweeper_: *mut Sweeper,
    concurrent_sweepers: Vec<ConcurrentMinorSweeper>,
    tracer_: *mut GCTracer,
    trace_id_: u64,
}

impl JobTaskTrait for MinorSweeperJob {
    fn run(&mut self, delegate: &mut dyn JobDelegate) -> () {
		unsafe {
			let sweeper = &mut *self.sweeper_;
        	self.run_impl(delegate, delegate.is_joining_thread());
		}
    }

    fn get_max_concurrency(&self, worker_count: usize) -> usize {
		unsafe {
			let sweeper = &mut *self.sweeper_;
        let kPagePerTask: usize = 2;
        cmp::min(
            self.concurrent_sweepers.len(),
            worker_count
                + (sweeper.ConcurrentMinorSweepingPageCount() + kPagePerTask - 1) / kPagePerTask,
        ) as usize
		}
    }
}

impl MinorSweeperJob {
    const kMaxTasks: usize = 1;

    fn new(isolate: *mut Isolate, sweeper: *mut Sweeper) -> Self {
		unsafe {
			let sweeper_ref = &mut *sweeper;
        assert!(sweeper_ref.minor_sweeping_state_.concurrent_sweepers_.len() <= Self::kMaxTasks);
        Self {
            sweeper: sweeper,
            concurrent_sweepers: sweeper_ref.minor_sweeping_state_.concurrent_sweepers_.iter().map(|x| x.downcast_ref::<ConcurrentMinorSweeper>().unwrap().clone()).collect(),
            tracer_: (&mut *isolate).heap_.tracer(),
            trace_id_: sweeper_ref.minor_sweeping_state_.background_trace_id_,
        }
		}
    }

    fn run_impl(&mut self, delegate: &mut dyn JobDelegate, is_joining_thread: bool) {
		unsafe {
			let sweeper = &mut *self.sweeper_;
			let heap = &mut *sweeper.heap_;
			let isolate = &mut *heap.isolate_;

        assert!(sweeper.minor_sweeping_in_progress());
        let offset = delegate.get_task_id();
        assert!(offset < self.concurrent_sweepers.len());
        let concurrent_sweeper = &mut self.concurrent_sweepers[offset];
        // TRACE_GC_EPOCH_WITH_FLOW(
        //     self.tracer_,
        //     sweeper.GetTracingScope(NEW_SPACE, is_joining_thread),
        //     if is_joining_thread {
        //         ThreadKind::kMain
        //     } else {
        //         ThreadKind::kBackground
        //     },
        //     self.trace_id_,
        //     TRACE_EVENT_FLAG_FLOW_IN,
        // );

        // In case multi-cage pointer compression mode is enabled ensure that
        // current thread's cage base values are properly initialized.
        let ptr_compr_cage_access_scope = PtrComprCageAccessScope::new(isolate);

        if !concurrent_sweeper.ConcurrentSweepSpace(delegate) {
            return;
        }
        concurrent_sweeper.ConcurrentSweepPromotedPages(delegate);
		}
    }
}

// Generic implementation for SweepingState methods
impl<const scope: SweepingScope> SweepingState<scope> {
    fn new(sweeper: *mut Sweeper) -> Self {
        SweepingState {
            sweeper_: sweeper,
			in_progress_: false,
			concurrent_sweepers_: Vec::new(),
			job_handle_: None,
			should_reduce_memory_: false,
			trace_id_: 0,
			background_trace_id_: 0,
        }
    }

    fn has_valid_job(&self) -> bool {
        match &self.job_handle_ {
            Some(job_handle) => job_handle.is_valid(),
            None => false,
        }
    }

    fn has_active_job(&self) -> bool {
        match &self.job_handle_ {
            Some(job_handle) => job_handle.is_valid() && job_handle.is_active(),
            None => false,
        }
    }

    fn stop_concurrent_sweeping(&mut self) {
        if let Some(mut job_handle) = self.job_handle_.take() {
            job_handle.cancel();
        }
    }

    fn initialize_sweeping(&mut self) {
        assert!(!self.has_valid_job());
        assert!(!self.in_progress_);
        assert!(self.concurrent_sweepers_.is_empty());
        // TODO: Implement flags
        //DCHECK_IMPLIES(scope == Sweeper::SweepingScope::kMinor, v8_flags.minor_ms);
        //DCHECK_IMPLIES(scope == Sweeper::SweepingScope::kMinor,
        //             !sweeper_->heap_->ShouldReduceMemory());
        self.should_reduce_memory_ = (scope != SweepingScope::kMinor) && unsafe { (&mut *self.sweeper_).heap_->ShouldReduceMemory() };
        self.trace_id_ = unsafe {
            ((self.sweeper_ as u64) ^ (&mut *self.sweeper_).heap_->tracer().CurrentEpoch(
                if scope == SweepingScope::kMajor {
                    GCTracerScope::MC_SWEEP
                } else {
                    GCTracerScope::MINOR_MS_SWEEP
                },
            )) << 1
        };
        self.background_trace_id_ = self.trace_id_ + 1;
    }

    fn start_sweeping(&mut self) {
        assert!(!self.has_valid_job());
        assert!(!self.in_progress_);
        assert!(self.concurrent_sweepers_.is_empty());
        assert_ne!(0, self.trace_id_);
        assert_ne!(0, self.background_trace_id_);
        self.in_progress_ = true;
    }

    fn start_concurrent_sweeping(&mut self) {
		unsafe {
			let sweeper = &mut *self.sweeper_;
        assert!(!self.has_valid_job());
        assert!(self.in_progress_);
		let heap = &mut *sweeper.heap_;
		let isolate = &mut *heap.isolate_;
        // TODO: Implement flags
        //if (v8_flags.concurrent_sweeping &&
        //    !sweeper_->heap_->delay_sweeper_tasks_for_testing_) {
			// TODO:  Figure out what V8::GetCurrentPlatform() returns
        if true {
            let job = if scope == SweepingScope::kMinor {
				Box::new(MinorSweeperJob::new(isolate, self.sweeper_)) as Box<dyn JobTaskTrait>
            } else {
				Box::new(MajorSweeperJob::new(isolate, self.sweeper_)) as Box<dyn JobTaskTrait>
            };

            let scope_id = if scope == SweepingScope::kMinor {
                GCTracerScope::MINOR_MS_SWEEP_START_JOBS
            } else {
                GCTracerScope::MC_SWEEP_START_JOBS
            };
            // TRACE_GC_WITH_FLOW(sweeper_->heap_->tracer(), scope_id,
            //                   background_trace_id(), TRACE_EVENT_FLAG_FLOW_OUT);
            //DCHECK_IMPLIES(v8_flags.minor_ms, concurrent_sweepers_.empty());
            let mut max_concurrent_sweeper_count = 1; // TODO: std::cmp::min(SweeperJob::kMaxTasks, V8::GetCurrentPlatform()->NumberOfWorkerThreads() + 1);
			let concurrent_sweepers_len = self.concurrent_sweepers_.len();
            if concurrent_sweepers_len == 0 {
                for _i in 0..max_concurrent_sweeper_count {
                    self.concurrent_sweepers_.push(Box::new(ConcurrentMajorSweeper::new(self.sweeper_)) as ConcurrentSweeperTraitObject);
                }
            }
            assert_eq!(max_concurrent_sweeper_count, self.concurrent_sweepers_.len());
			
			// TODO: V8::GetCurrentPlatform()->PostJob needs to be implemented
            self.job_handle_ = Some(Box::new(MockJobHandle{})); //V8::GetCurrentPlatform()->PostJob(TaskPriority::kUserVisible, std::move(job));
        }
		}
    }

    fn join_sweeping(&mut self) {
        assert!(self.in_progress_);
        if let Some(mut job_handle) = self.job_handle_.take() {
            job_handle.join();
        }
    }

    fn finish_sweeping(&mut self) {
        assert!(self.in_progress_);
        // Sweeping jobs were already joined.
        assert!(self.job_handle_.is_none());

        self.concurrent_sweepers_.clear();
        self.in_progress_ = false;
    }

    fn pause(&mut self) {
        if self.job_handle_.is_none() {
            return;
        }

        //DCHECK(v8_flags.concurrent_sweeping);
        if let Some(mut job_handle) = self.job_handle_.take() {
            job_handle.cancel();
        }
    }

    fn resume(&mut self) {
        assert!(self.in_progress_);
		unsafe {
			let sweeper = &mut *self.sweeper_;
			let heap = &mut *sweeper.heap_;
			let isolate = &mut *heap.isolate_;
			
			// TODO: V8::GetCurrentPlatform()->PostJob needs to be implemented
			self.job_handle_ = Some(Box::new(MockJobHandle{}));
            // self.job_handle_ = V8::GetCurrentPlatform()->PostJob(
            //     TaskPriority::kUserVisible,
            //     std::make_unique<SweeperJob>(sweeper_->heap_->isolate(), sweeper_),
            // );
		}
    }
}

// Mock implementations for traits that rely on external V8 dependencies.
struct MockPlatform {}

impl PlatformTrait for MockPlatform {
    fn number_of_worker_threads(&self) -> usize {
        1 // Mock value
    }

	fn post_job(&self, _priority: TaskPriority, _job: Box<dyn JobTaskTrait>) -> Box<dyn JobHandleTrait> {
		Box::new(MockJobHandle{})
	}
}

struct MockJobHandle {}

impl JobHandleTrait for MockJobHandle {
    fn is_valid(&self) -> bool {
        true // Mock value
    }
    fn is_active(&self) -> bool {
        true // Mock value
    }
    fn cancel(&mut self) -> () {
        // Mock implementation
    }
    fn join(&mut self) -> () {
        // Mock implementation
    }
}

struct PtrComprCageAccessScope<'a> {
    isolate: *mut Isolate, // Assuming Isolate is defined elsewhere
    _phantom: std::marker::PhantomData<&'a Isolate>,
}

impl<'a> PtrComprCageAccessScope<'a> {
    fn new(isolate: *mut Isolate) -> Self {
        PtrComprCageAccessScope {
            isolate,
            _phantom: std::marker::PhantomData,
        }
    }
}

struct Heap {
	isolate_: *mut Isolate,
	tracer_: GCTracer,
    memory_allocator_: *mut MemoryAllocator,
    new_lo_space_: *mut NewLoSpace,
    new_space_: *mut NewSpace,
}

impl Heap {
	fn tracer(&mut self) -> &mut GCTracer {
		&mut self.tracer_
	}
	fn ShouldReduceMemory(&mut self) -> bool {
		false
	}
	fn sticky_space(&mut self) -> *mut PagedSpaceBase {
		std::ptr::null_mut()
	}
	fn paged_new_space(&mut self) -> *mut PagedNewSpace {
		std::ptr::null_mut()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GCTracerScope {
    MC_SWEEP,
    MINOR_MS_SWEEP,
    MINOR_MS_SWEEP_START_JOBS,
    MC_SWEEP_START_JOBS,
	MINOR_MS_COMPLETE_SWEEPING,
	MC_COMPLETE_SWEEPING,
}

impl GCTracerScope {
    fn NeedsYoungEpoch(scope_id: GCTracerScope) -> bool {
        match scope_id {
            GCTracerScope::MINOR_MS_SWEEP |
            GCTracerScope::MINOR_MS_SWEEP_START_JOBS |
			GCTracerScope::MINOR_MS_COMPLETE_SWEEPING => true,
            _ => false,
        }
    }
}

struct GCTracer {}

impl GCTracer {
    fn CurrentEpoch(&mut self, scope: GCTracerScope) -> u64 {
        // Mock implementation
        0
    }

	fn IsInAtomicPause(&mut self) -> bool {
		false
	}

	fn GetCurrentCollector(&mut self) -> GarbageCollector {
		GarbageCollector::MARK_COMPACTOR
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GarbageCollector {
    MARK_COMPACTOR,
    MINOR_MARK_SWEEPER,
}

struct NonAtomicMarkingState {}

struct Isolate {
    has_shared_space_: bool,
	heap_: Heap,
}

impl Isolate {
    fn has_shared_space(&self) -> bool {
        self.has_shared_space_
    }
}

// Placeholder functions for code that depends on external state

fn RoundUp(addr: Address, page_size: usize) -> Address {
    ((addr + page_size - 1) / page_size) * page_size
}

fn RoundDown(addr: Address, page_size: usize) -> Address {
    (addr / page_size) * page_size
}

fn heap_ShouldZapGarbage() -> bool {
    false
}

fn GetSweepSpaceIndex(space: AllocationSpace) -> usize {
    match space {
        AllocationSpace::NEW_SPACE => 0,
        AllocationSpace::OLD_SPACE => 1,
        AllocationSpace::CODE_SPACE => 2,
        AllocationSpace::LO_SPACE => 3,
        AllocationSpace::SHARED_SPACE => 4,
        AllocationSpace::TRUSTED_SPACE => 5,
        AllocationSpace::SHARED_TRUSTED_SPACE => 6,
        _ => panic!("Invalid AllocationSpace"),
    }
}

type Address = usize;
type SizeT = usize;
type TaggedT = usize;
const kTaggedSize: usize = 8;
const kZapValue: u8 = 0xAA;

fn IsAligned(addr: Address, alignment: usize) -> bool {
    addr % alignment == 0
}

struct WritableFreeSpace {
	addr_: Address,
	size_: SizeT,
}

impl WritableFreeSpace {
	fn ForNonExecutableMemory(addr: Address, size: SizeT) -> Self {
		Self {
			addr_: addr,
			size_: size,
		}
	}
}

// struct LiveObjectRange {
//     // Implement iterator
// }

// impl LiveObjectRange {
//     // Implement iterator methods
// }

impl Sweeper {
    pub fn new(heap: *mut Heap) -> Self {
        let mutex_ = Arc::new(Mutex::new(()));
        let cv_page_swept_ = Arc::new(Condvar::new());
        Sweeper {
            heap_: heap,
            marking_state_: unsafe { (&mut *heap).marking_state_ },
            main_thread_local_sweeper_: LocalSweeper { sweeper_: unsafe { &mut *heap}.memory_allocator_ as *mut MemoryAllocator, sweeper: unsafe { &mut *heap}.memory_allocator_ as *mut MemoryAllocator},
			mutex_: mutex_.clone(),
			cv_page_swept_: cv_page_swept_.clone(),
			promoted_pages_iteration_notification_mutex_: Mutex::new(()),
			promoted_pages_iteration_notification_variable_: Condvar::new(),
			promoted_page_iteration_in_progress_: AtomicBool::new(false),
			sweeping_list_: Default::default(),
            sweeping_list_for_promoted_page_iteration_: Vec::new(),
			has_sweeping_work_: [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)],
			swept_list_: Default::default(),
			has_swept_pages_: [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)],
			minor_sweeping_state_: SweepingState::new(heap as *mut Self),
			major_sweeping_state_: SweepingState::new(heap as *mut Self),
            promoted_pages_for_iteration_count_: 0,
            iterated_promoted_pages_count_: 0,
        }
    }

    fn TearDown(&mut self) {
        self.minor_sweeping_state_.stop_concurrent_sweeping();
        self.major_sweeping_state_.stop_concurrent_sweeping();
    }

    fn InitializeMajorSweeping(&mut self) {
        self.major_sweeping_state_.initialize_sweeping();
    }

    fn InitializeMinorSweeping(&mut self) {
        self.minor_sweeping_state_.initialize_sweeping();
    }

    fn StartMajorSweeping(&mut self) {
		unsafe {
			let heap = &mut *self.heap_;
        assert_eq!(GarbageCollector::MARK_COMPACTOR, heap.tracer().GetCurrentCollector());
        assert!(!self.minor_sweeping_in_progress());
        self.major_sweeping_state_.start_sweeping();
        self.ForAllSweepingSpaces(|space| {
            // Sorting is done in order to make compaction more efficient: by sweeping
            // pages with the most free bytes first, we make it more likely that when
            // evacuating a page, already swept pages will have enough free bytes to
            // hold the objects to move (and therefore, we won't need to wait for more
            // pages to be swept in order to move those objects).
            let space_index = GetSweepSpaceIndex(space);
            // TODO: Implement flags
            //DCHECK_IMPLIES(space == NEW_SPACE, sweeping_list_[space_index].empty());

            //self.sweeping_list_[space_index].sort_by(|a, b| ComparePagesForSweepingOrder(a, b));
        });
		}
    }

    fn StartMinorSweeping(&mut self) {
		unsafe {
			let heap = &mut *self.heap_;
        assert_eq!(GarbageCollector::MINOR_MARK_SWEEPER, heap.tracer().GetCurrentCollector());
        self.minor_sweeping_state_.start_sweeping();
        let new_space_index = GetSweepSpaceIndex(NEW_SPACE);
        // TODO: Implement ComparePagesForSweepingOrder
        //self.sweeping_list_[new_space_index].sort_by(|a, b| ComparePagesForSweepingOrder(a, b));
		}
    }

    fn StartMajorSweeperTasks(&mut self) {
		unsafe {
			let heap = &mut *self.heap_;
        //DCHECK_IMPLIES(v8_flags.minor_ms, GarbageCollector::MARK_COMPACTOR ==
        //                                    heap_->tracer()->GetCurrentCollector());
        assert!(!self.minor_sweeping_in_progress());
        assert!(!self.promoted_page_iteration_in_progress_.load(Ordering::Relaxed));
        assert_eq!(0, self.promoted_pages_for_iteration_count_);
        self.major_sweeping_state_.start_concurrent_sweeping();
		}
    }

    fn StartMinorSweeperTasks(&mut self) {
		unsafe {
			let heap = &mut *self.heap_;
        //DCHECK(v8_flags.minor_ms);
        assert_eq!(GarbageCollector::MINOR_MARK_SWEEPER, heap.tracer().GetCurrentCollector());
        assert!(!self.promoted_page_iteration_in_progress_.load(Ordering::Relaxed));
        let mut promoted_pages_for_clearing: Vec<*mut MutablePageMetadata> = Vec::new();
        if self.promoted_pages_for_iteration_count_ > 0 {
            // TODO: Implement flags and ShouldUpdateRememberedSets
            //if (ShouldUpdateRememberedSets(heap_)) {
            //    promoted_page_iteration_in_progress_.store(true,
            //                                              std::memory_order_release);
            //} else {
            //    promoted_pages_for_clearing.swap(
            //        sweeping_list_for_promoted_page_iteration_);
            //    DCHECK(sweeping_list_for_promoted_page_iteration_.empty());
            //    promoted_pages_for_iteration_count_ = 0;
            //}
        }
        self.minor_sweeping_state_.start_concurrent_sweeping();
        // TODO: Implement ClearPromotedPages
        //ClearPromotedPages(heap_, promoted_pages_for_clearing);
		}
    }

    fn GetSweptPageSafe(&mut self, space: *mut PagedSpaceBase) -> Option<*mut PageMetadata> {
        let mut guard = self.mutex_.lock().unwrap();
        let list = &mut self.swept_list_[GetSweepSpaceIndex(unsafe { (&mut *space).identity() })];
        if list.is_empty() {
            self.has_swept_pages_[GetSweepSpaceIndex(unsafe { (&mut *space).identity() })].store(false, Ordering::Release);
            return None;
        }
        let page = list.pop().unwrap();
        if list.is_empty() {
            self.has_swept_pages_[GetSweepSpaceIndex(unsafe { (&mut *space).identity() })].store(false, Ordering::Release);
        }
        Some(page)
    }

    fn GetAllSweptPagesSafe(&mut self, space: *mut PagedSpaceBase) -> Vec<*mut PageMetadata> {
        let mut
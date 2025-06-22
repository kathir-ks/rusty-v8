// TODO: Add appropriate crate dependencies in Cargo.toml

use std::sync::{Mutex, Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::cmp::{max, min};
use std::mem;
use std::ptr;
use std::vec::Vec;
use std::option::Option;

// Placeholder for cppgc::Platform and its related traits/structs
// You'll need to define these according to the C++ interface.
pub trait PageAllocatorTrait {
    fn commit_page_size(&self) -> usize;
    fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize);
}

pub trait PlatformTrait {
    type PageAllocator: PageAllocatorTrait;
    type TaskRunner: TaskRunnerTrait;
    type JobHandle;

    fn get_page_allocator(&self) -> &Self::PageAllocator;
    fn get_foreground_task_runner(&self, priority: TaskPriority) -> Arc<Self::TaskRunner>;
    fn post_job(&self, priority: TaskPriority, job: Box<dyn JobTask>) -> Box<Self::JobHandle>;
}

pub trait TaskRunnerTrait {
    fn post_task(&self, task: Box<dyn Task>);
    fn post_delayed_task(&self, task: Box<dyn Task>, delay_seconds: f64);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskPriority {
    kUserBlocking,
    kUserVisible,
}

pub trait Task {
    fn run(&mut self);
}

pub trait JobDelegate {
    fn should_yield(&self) -> bool;
}

pub trait JobTask: Send + Sync {
    fn run(&mut self, delegate: &mut dyn JobDelegate);
    fn get_max_concurrency(&self, active_worker_count: usize) -> usize;
}

pub trait JobHandle {
    fn is_valid(&self) -> bool;
    fn is_active(&self) -> bool;
    fn cancel(&mut self);
    fn join(&mut self);
    fn update_priority(&mut self, priority: TaskPriority);
    fn update_priority_enabled(&self) -> bool;
}

// Placeholder structures - replace with actual implementations
pub struct Platform;
impl PlatformTrait for Platform {
    type PageAllocator = PageAllocator;
    type TaskRunner = TaskRunner;
    type JobHandle = JobHandle;
    fn get_page_allocator(&self) -> &Self::PageAllocator {
        todo!()
    }
    fn get_foreground_task_runner(&self, priority: TaskPriority) -> Arc<Self::TaskRunner> {
        todo!()
    }
    fn post_job(&self, priority: TaskPriority, job: Box<dyn JobTask>) -> Box<Self::JobHandle> {
        todo!()
    }
}
pub struct PageAllocator;
impl PageAllocatorTrait for PageAllocator {
    fn commit_page_size(&self) -> usize {
        todo!()
    }
    fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize) {
        todo!()
    }
}

pub struct TaskRunner;
impl TaskRunnerTrait for TaskRunner {
    fn post_task(&self, task: Box<dyn Task>) {
        todo!()
    }
    fn post_delayed_task(&self, task: Box<dyn Task>, delay_seconds: f64) {
        todo!()
    }
}

pub struct JobHandle;
impl JobHandle for JobHandle {
    fn is_valid(&self) -> bool {
        todo!()
    }
    fn is_active(&self) -> bool {
        todo!()
    }
    fn cancel(&mut self) {
        todo!()
    }
    fn join(&mut self) {
        todo!()
    }
    fn update_priority(&mut self, priority: TaskPriority) {
        todo!()
    }
    fn update_priority_enabled(&self) -> bool {
        todo!()
    }
}

// End of placeholders

// Placeholder for v8::base::TimeTicks and v8::base::TimeDelta
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct TimeTicks {
    time: Instant,
}

impl TimeTicks {
    fn now() -> Self {
        TimeTicks { time: Instant::now() }
    }
}

use std::ops::{Add, Sub};

impl Add<Duration> for TimeTicks {
    type Output = Self;

    fn add(self, duration: Duration) -> Self {
        TimeTicks { time: self.time + duration }
    }
}

impl Sub<TimeTicks> for TimeTicks {
    type Output = Duration;

    fn sub(self, other: TimeTicks) -> Self::Output {
        self.time - other.time
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct TimeDelta {
    duration: Duration,
}

impl TimeDelta {
    fn from_milliseconds(ms: u64) -> Self {
        TimeDelta { duration: Duration::from_millis(ms) }
    }
    fn in_milliseconds_f(&self) -> f64 {
        self.duration.as_secs_f64() * 1000.0
    }
    fn in_seconds_f(&self) -> f64 {
        self.duration.as_secs_f64()
    }
}

impl Add<TimeDelta> for TimeTicks {
    type Output = Self;

    fn add(self, other: TimeDelta) -> Self {
        TimeTicks { time: self.time + other.duration }
    }
}

// End of placeholders

// Placeholder for HeapObjectHeader
#[derive(Debug)]
struct HeapObjectHeader {
    marked: AtomicBool,
    free: AtomicBool,
    finalizable: bool,
    // NOTE: The next unfinalized pointer is used in CAGED_HEAP. Because this is
    // difficult to implement without access to internal memory layout, this is
    // a placeholder.
    // next_unfinalized: *mut HeapObjectHeader,
}

impl HeapObjectHeader {
    fn is_marked<const A: AccessMode>(&self) -> bool {
        self.marked.load(A::ORDERING)
    }
    fn unmark<const A: AccessMode>(&self) {
        self.marked.store(false, A::ORDERING);
    }
    fn is_free<const A: AccessMode>(&self) -> bool {
        self.free.load(A::ORDERING)
    }
    fn finalize(&self) {
        //Placeholder
    }
    fn is_finalizable(&self) -> bool {
        self.finalizable
    }
    fn allocated_size(&self) -> usize {
        16 // Placeholder
    }
    fn set_next_unfinalized(&mut self, _header: *mut HeapObjectHeader) {
        todo!()
    }
    unsafe fn get_next_unfinalized(&self, _cage_base: u64) -> *mut HeapObjectHeader {
        todo!()
    }
    fn is_large_object(&self) -> bool {
        false // Placeholder
    }
}

// Placeholder for AccessMode
trait AccessMode {
    const ORDERING: Ordering;
}

struct AtomicAccess;
impl AccessMode for AtomicAccess {
    const ORDERING: Ordering = Ordering::Relaxed;
}

struct RegularAccess;
impl AccessMode for RegularAccess {
    const ORDERING: Ordering = Ordering::Relaxed; //Can be relaxed because it's single threaded
}

const K_FREE_LIST_ENTRY_SIZE: usize = 8;

// Placeholder for SetMemoryInaccessible/CheckMemoryIsInaccessible
fn set_memory_inaccessible(_header: *mut HeapObjectHeader, _size: usize) {
    // Placeholder
}

fn check_memory_is_inaccessible(_header: *mut HeapObjectHeader, _size: usize) {
    // Placeholder
}

fn can_discard_memory() -> bool {
    true // Placeholder
}

// Placeholder for StickyBits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StickyBits {
    kEnabled,
    kDisabled,
}

mod cppgc {
    pub mod internal {
        use super::*;

        const K_BACKGROUND_BOOSTED_PRIORITY: TaskPriority = TaskPriority::kUserBlocking;
        const K_BACKGROUND_REGULAR_PRIORITY: TaskPriority = TaskPriority::kUserVisible;
        const K_FOREGROUND_REGULAR_PRIORITY: TaskPriority = TaskPriority::kUserBlocking;
        const K_FOREGROUND_LOW_PRIORITY: TaskPriority = TaskPriority::kUserVisible;

        struct DeadlineChecker {
            end_: TimeTicks,
            count_: usize,
        }

        impl DeadlineChecker {
            const K_INTERVAL: usize = 4;

            fn new(end: TimeTicks) -> Self {
                DeadlineChecker { end_: end, count_: 0 }
            }

            fn check(&mut self) -> bool {
                if self.count_ % Self::K_INTERVAL == 0 {
                    self.end_ < TimeTicks::now()
                } else {
                    false
                }
            }
        }

        #[derive(Debug, PartialEq)]
        enum MutatorThreadSweepingMode {
            kOnlyFinalizers,
            kAll,
        }

        fn to_string(sweeping_mode: MutatorThreadSweepingMode) -> &'static str {
            match sweeping_mode {
                MutatorThreadSweepingMode::kAll => "all",
                MutatorThreadSweepingMode::kOnlyFinalizers => "only-finalizers",
            }
        }

        struct ObjectStartBitmapVerifier {}

        impl ObjectStartBitmapVerifier {
            fn new() -> Self {
                ObjectStartBitmapVerifier {}
            }

            fn verify(&self, heap: &mut RawHeap) {
                #[cfg(debug_assertions)]
                self.traverse(heap);
            }
            fn verify_page(&self, page: &mut NormalPage) {
                #[cfg(debug_assertions)]
                self.traverse_page(page);
            }
            fn traverse(&self, _heap: &mut RawHeap) {
                //Placeholder
            }
            fn traverse_page(&self, _page: &mut NormalPage) {
                //Placeholder
            }
        }

        struct FreeHandlerBase {}

        impl FreeHandlerBase {
            fn new() -> Self {
                FreeHandlerBase {}
            }
            fn free_free_list(&self, _unfinalized_free_list: &mut Vec<FreeListBlock>) {
                // Placeholder
            }
        }

        struct DiscardingFreeHandler<'a> {
            page_allocator_: &'a dyn PageAllocatorTrait,
            free_list_: &'a mut FreeList,
            page_: &'a mut BasePage,
        }

        impl<'a> DiscardingFreeHandler<'a> {
            fn new(
                page_allocator: &'a dyn PageAllocatorTrait,
                free_list: &'a mut FreeList,
                page: &'a mut BasePage,
            ) -> Self {
                DiscardingFreeHandler {
                    page_allocator_: page_allocator,
                    free_list_: free_list,
                    page_: page,
                }
            }

            fn free(&mut self, block: FreeListBlock) {
                let unused_range = self.free_list_.add_returning_unused_bounds(block);
                let aligned_begin_unused =
                    round_up(unused_range.0 as usize, self.page_allocator_.commit_page_size()) as *mut std::ffi::c_void;
                let aligned_end_unused =
                    round_down(unused_range.1 as usize, self.page_allocator_.commit_page_size());
                if aligned_begin_unused as usize < aligned_end_unused {
                    let discarded_size = aligned_end_unused - aligned_begin_unused as usize;
                    self.page_allocator_.discard_system_pages(
                        aligned_begin_unused,
                        aligned_end_unused - aligned_begin_unused as usize,
                    );
                    self.page_.increment_discarded_memory(discarded_size);
                    self.page_
                        .space()
                        .raw_heap()
                        .heap()
                        .stats_collector()
                        .increment_discarded_memory(discarded_size);
                }
            }

            fn free_free_list(&mut self, unfinalized_free_list: &mut Vec<FreeListBlock>) {
                for entry in unfinalized_free_list.drain(..) {
                    self.free(entry);
                }
            }
        }

        struct RegularFreeHandler<'a> {
            free_list_: &'a mut FreeList,
        }

        impl<'a> RegularFreeHandler<'a> {
            fn new(free_list: &'a mut FreeList) -> Self {
                RegularFreeHandler { free_list_: free_list }
            }

            fn free(&mut self, block: FreeListBlock) {
                self.free_list_.add(block);
            }

            fn free_free_list(&mut self, unfinalized_free_list: &mut Vec<FreeListBlock>) {
                for entry in unfinalized_free_list.drain(..) {
                    self.free(entry);
                }
            }
        }

        struct ThreadSafeStack<T> {
            mutex_: Mutex<Vec<T>>,
            is_empty_: AtomicBool,
        }

        impl<T> ThreadSafeStack<T> {
            fn new() -> Self {
                ThreadSafeStack {
                    mutex_: Mutex::new(Vec::new()),
                    is_empty_: AtomicBool::new(true),
                }
            }

            fn push(&self, t: T) {
                let mut vector = self.mutex_.lock().unwrap();
                vector.push(t);
                self.is_empty_.store(false, Ordering::Relaxed);
            }

            fn pop(&self) -> Option<T> {
                let mut vector = self.mutex_.lock().unwrap();
                if vector.is_empty() {
                    self.is_empty_.store(true, Ordering::Relaxed);
                    None
                } else {
                    let top = vector.pop().unwrap();
                    Some(top)
                }
            }

            fn insert<I>(&self, begin: I, end: I)
            where
                I: Iterator<Item = T>,
            {
                let mut vector = self.mutex_.lock().unwrap();
                for item in end {
                    vector.push(item);
                }
                self.is_empty_.store(false, Ordering::Relaxed);
            }

            fn is_empty(&self) -> bool {
                self.is_empty_.load(Ordering::Relaxed)
            }
        }

        struct SweepingState {
            swept_unfinalized_pages: ThreadSafeStack<SweptPageState>,
            unswept_pages: ThreadSafeStack<BasePagePointer>,
        }

        impl SweepingState {
            fn new() -> Self {
                SweepingState {
                    unswept_pages: ThreadSafeStack::new(),
                    swept_unfinalized_pages: ThreadSafeStack::new(),
                }
            }
        }

        type SpaceStates = Vec<SweepingState>;

        struct SweptPageState {
            page: *mut BasePage,
            unfinalized_objects_head: *mut HeapObjectHeader,
            cached_free_list: FreeList,
            unfinalized_free_list: Vec<FreeListBlock>,
            is_empty: bool,
            largest_new_free_list_entry: usize,
        }

        impl SweptPageState {
            fn new(page: *mut BasePage) -> Self {
                SweptPageState {
                    page,
                    unfinalized_objects_head: ptr::null_mut(),
                    cached_free_list: FreeList::new(),
                    unfinalized_free_list: Vec::new(),
                    is_empty: false,
                    largest_new_free_list_entry: 0,
                }
            }
        }

        fn sticky_unmark<const A: AccessMode>(header: &HeapObjectHeader, sticky_bits: StickyBits) {
            #[cfg(feature = "cppgc_young_generation")]
            {
                if sticky_bits == StickyBits::kDisabled {
                    header.unmark::<A>();
                }
            }
            #[cfg(not(feature = "cppgc_young_generation"))]
            {
                header.unmark::<A>();
            }
        }

        struct InlinedFinalizationBuilderBase {
            result_: InlinedFinalizationBuilderResult,
        }

        impl InlinedFinalizationBuilderBase {
            fn new() -> Self {
                InlinedFinalizationBuilderBase {
                    result_: InlinedFinalizationBuilderResult {
                        is_empty: false,
                        largest_new_free_list_entry: 0,
                    },
                }
            }
        }

        struct InlinedFinalizationBuilderResult {
            is_empty: bool,
            largest_new_free_list_entry: usize,
        }

        struct InlinedFinalizationBuilder<'a, F: FreeHandlerTrait> {
            free_handler: F,
            result_: InlinedFinalizationBuilderResult,
            page: &'a mut BasePage,
        }

        impl<'a, F: FreeHandlerTrait> InlinedFinalizationBuilder<'a, F> {
            fn new(free_handler: F, page: &'a mut BasePage) -> Self {
                InlinedFinalizationBuilder {
                    free_handler,
                    result_: InlinedFinalizationBuilderResult {
                        is_empty: false,
                        largest_new_free_list_entry: 0,
                    },
                    page
                }
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                unsafe {
                    (*header).finalize();
                    set_memory_inaccessible(header, size);
                }
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                self.free_handler.free(FreeListBlock { start, size });
                self.result_.largest_new_free_list_entry =
                    max(self.result_.largest_new_free_list_entry, size);
            }

            fn get_result(mut self, is_empty: bool) -> InlinedFinalizationBuilderResult {
                self.result_.is_empty = is_empty;
                self.result_
            }
        }

        trait FreeHandlerTrait {
            fn free(&mut self, block: FreeListBlock);
        }

        impl<'a> FreeHandlerTrait for DiscardingFreeHandler<'a> {
            fn free(&mut self, block: FreeListBlock) {
                self.free(block);
            }
        }

        impl<'a> FreeHandlerTrait for RegularFreeHandler<'a> {
            fn free(&mut self, block: FreeListBlock) {
                self.free(block);
            }
        }

        struct DeferredFinalizationBuilder<'a, F: FreeHandlerTrait> {
            free_handler: F,
            result_: SweptPageState,
            current_unfinalized_: *mut HeapObjectHeader,
            found_finalizer_: bool,
            page: &'a mut BasePage,
        }

        impl<'a, F: FreeHandlerTrait> DeferredFinalizationBuilder<'a, F> {
            fn new(free_handler: F, page: &'a mut BasePage) -> Self {
                DeferredFinalizationBuilder {
                    free_handler,
                    result_: SweptPageState::new(page),
                    current_unfinalized_: ptr::null_mut(),
                    found_finalizer_: false,
                    page,
                }
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                unsafe {
                    if (*header).is_finalizable() {
                        #[cfg(feature = "cppgc_caged_heap")]
                        {
                            if self.current_unfinalized_.is_null() {
                                // DCHECK_NULL(self.result_.unfinalized_objects_head);
                                self.current_unfinalized_ = header;
                                self.result_.unfinalized_objects_head = header;
                            } else {
                                (*self.current_unfinalized_).set_next_unfinalized(header);
                                self.current_unfinalized_ = header;
                            }
                        }
                        #[cfg(not(feature = "cppgc_caged_heap"))]
                        {
                            self.result_
                                .unfinalized_free_list
                                .push(FreeListBlock { start: 0, size: 0 });
                        }
                        self.found_finalizer_ = true;
                    } else {
                        set_memory_inaccessible(header, size);
                    }
                }
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                if self.found_finalizer_ {
                    self.result_.unfinalized_free_list.push(FreeListBlock { start, size });
                } else {
                    self.free_handler.free(FreeListBlock { start, size });
                }
                self.result_.largest_new_free_list_entry =
                    max(self.result_.largest_new_free_list_entry, size);
                self.found_finalizer_ = false;
            }

            fn get_result(mut self, is_empty: bool) -> SweptPageState {
                self.result_.is_empty = is_empty;
                self.result_
            }
        }

        fn sweep_normal_page<F>(
            page: &mut NormalPage,
            page_allocator: &dyn PageAllocatorTrait,
            sticky_bits: StickyBits,
        ) -> F::ResultType
        where
            F: NormalPageFreeListTrait
        {
            let k_atomic_access = AtomicAccess;
            let mut builder = F::new(page, page_allocator);

            let bitmap = page.object_start_bitmap();

            let mut live_bytes: usize = 0;

            let mut start_of_gap: Address = page.payload_start();

            let clear_bit_if_coalesced_entry =
                |bitmap: &PlatformAwareObjectStartBitmap, start_of_gap: Address, address: Address| {
                    if address != start_of_gap {
                        bitmap.clear_bit::<AtomicAccess>(address);
                    } else {
                        assert!(bitmap.check_bit::<AtomicAccess>(address));
                    }
                };

            let mut begin = page.payload_start();
            let end = page.payload_end();
            while begin != end {
                assert!(bitmap.check_bit::<AtomicAccess>(begin));
                let header = begin as *mut HeapObjectHeader;
                let size: usize = unsafe { (*header).allocated_size() };

                // Check if this is a free list entry.
                if unsafe { (*header).is_free::<k_atomic_access>() } {
                    set_memory_inaccessible(header, min(K_FREE_LIST_ENTRY_SIZE, size));
                    check_memory_is_inaccessible(header, size);
                    clear_bit_if_coalesced_entry(bitmap, start_of_gap, begin);
                    begin += size;
                    continue;
                }

                // Check if object is not marked (not reachable).
                if !unsafe { (*header).is_marked::<k_atomic_access>() } {
                    builder.add_finalizer(header, size);
                    clear_bit_if_coalesced_entry(bitmap, start_of_gap, begin);
                    begin += size;
                    continue;
                }

                // The object is alive.
                let header_address = header as Address;
                if start_of_gap != header_address {
                    let new_free_list_entry_size = header_address - start_of_gap;
                    builder.add_free_list_entry(start_of_gap, new_free_list_entry_size);
                    assert!(bitmap.check_bit::<AtomicAccess>(start_of_gap));
                }

                unsafe {
                    sticky_unmark::<AtomicAccess>(&*header, sticky_bits);
                }
                begin += size;
                start_of_gap = begin;
                live_bytes += size;
            }

            let is_empty = live_bytes == 0;
            assert_eq!(is_empty, page.marked_bytes() == 0);
            if is_empty {
                assert_eq!(start_of_gap, page.payload_start());
            }

            if !is_empty && start_of_gap != page.payload_end() {
                builder.add_free_list_entry(start_of_gap, page.payload_end() - start_of_gap);
                assert!(bitmap.check_bit::<AtomicAccess>(start_of_gap));
            }

            page.set_allocated_bytes_at_last_gc(live_bytes);
            page.reset_marked_bytes(if sticky_bits == StickyBits::kDisabled {
                0
            } else {
                live_bytes
            });
            builder.get_result(is_empty)
        }

        trait NormalPageFreeListTrait {
            type ResultType;
            fn new(page: &mut NormalPage, page_allocator: &dyn PageAllocatorTrait) -> Self;
            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize);
            fn add_free_list_entry(&mut self, start: Address, size: usize);
            fn get_result(self, is_empty: bool) -> Self::ResultType;
        }

        impl<'a> NormalPageFreeListTrait for InlinedFinalizationBuilder<'a, DiscardingFreeHandler<'a>> {
            type ResultType = InlinedFinalizationBuilderResult;
            fn new(page: &mut NormalPage, page_allocator: &dyn PageAllocatorTrait) -> Self {
                let free_list = NormalPageSpace::from(page.space_mut()).free_list();
                let free_handler = DiscardingFreeHandler::new(page_allocator, free_list, page.as_base_page_mut());
                InlinedFinalizationBuilder::new(free_handler, page.as_base_page_mut())
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                InlinedFinalizationBuilder::add_finalizer(self, header, size)
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                InlinedFinalizationBuilder::add_free_list_entry(self, start, size)
            }

            fn get_result(self, is_empty: bool) -> Self::ResultType {
                InlinedFinalizationBuilder::get_result(self, is_empty)
            }
        }

        impl<'a> NormalPageFreeListTrait for InlinedFinalizationBuilder<'a, RegularFreeHandler<'a>> {
            type ResultType = InlinedFinalizationBuilderResult;
            fn new(page: &mut NormalPage, page_allocator: &dyn PageAllocatorTrait) -> Self {
                let free_list = NormalPageSpace::from(page.space_mut()).free_list();
                let free_handler = RegularFreeHandler::new(free_list);
                InlinedFinalizationBuilder::new(free_handler, page.as_base_page_mut())
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                InlinedFinalizationBuilder::add_finalizer(self, header, size)
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                InlinedFinalizationBuilder::add_free_list_entry(self, start, size)
            }

            fn get_result(self, is_empty: bool) -> Self::ResultType {
                InlinedFinalizationBuilder::get_result(self, is_empty)
            }
        }

        impl<'a> NormalPageFreeListTrait for DeferredFinalizationBuilder<'a, DiscardingFreeHandler<'a>> {
            type ResultType = SweptPageState;
            fn new(page: &mut NormalPage, page_allocator: &dyn PageAllocatorTrait) -> Self {
                let mut state = SweptPageState::new(page.as_base_page_mut());
                let free_list = &mut state.cached_free_list;
                let free_handler = DiscardingFreeHandler::new(page_allocator, free_list, page.as_base_page_mut());
                DeferredFinalizationBuilder::new(free_handler, page.as_base_page_mut())
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                DeferredFinalizationBuilder::add_finalizer(self, header, size)
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                DeferredFinalizationBuilder::add_free_list_entry(self, start, size)
            }

            fn get_result(self, is_empty: bool) -> Self::ResultType {
                DeferredFinalizationBuilder::get_result(self, is_empty)
            }
        }

        impl<'a> NormalPageFreeListTrait for DeferredFinalizationBuilder<'a, RegularFreeHandler<'a>> {
            type ResultType = SweptPageState;
            fn new(page: &mut NormalPage, page_allocator: &dyn PageAllocatorTrait) -> Self {
                let mut state = SweptPageState::new(page.as_base_page_mut());
                let free_list = &mut state.cached_free_list;
                let free_handler = RegularFreeHandler::new(free_list);
                DeferredFinalizationBuilder::new(free_handler, page.as_base_page_mut())
            }

            fn add_finalizer(&mut self, header: *mut HeapObjectHeader, size: usize) {
                DeferredFinalizationBuilder::add_finalizer(self, header, size)
            }

            fn add_free_list_entry(&mut self, start: Address, size: usize) {
                DeferredFinalizationBuilder::add_free_list_entry(self, start, size)
            }

            fn get_result(self, is_empty: bool) -> Self::ResultType {
                DeferredFinalizationBuilder::get_result(self, is_empty)
            }
        }

        const K_SWEEP_WITHOUT_SPACE_ASSIGNMENT: *mut BaseSpace = ptr::null_mut();

        #[derive(Debug, PartialEq)]
        enum EmptyPageHandling {
            kDestroy,
            kReturn,
        }

        /// SweepFinalizer is responsible for heap/space/page finalization. Finalization
        /// is defined as a step following concurrent sweeping which:
        /// - calls finalizers;
        /// - returns (unmaps) empty pages;
        /// - merges freelists to the space's freelist.
        struct SweepFinalizer<'a> {
            platform_: &'a dyn PlatformTrait,
            stats_collector_: &'a mut StatsCollector,
            space_: *mut BaseSpace,
            unused_destroyed_normal_pages_: &'a mut usize,
            free_memory_handling_: FreeMemoryHandling,
            empty_page_handling_: EmptyPageHandling,
            largest_consecutive_block_: usize,
        }

        impl<'a> SweepFinalizer<'a> {
            fn new(
                platform: &'a dyn PlatformTrait,
                stats_collector: &'a mut StatsCollector,
                space: *mut BaseSpace,
                unused_destroyed_normal_pages: &'a mut usize,
                free_memory_handling: FreeMemoryHandling,
                empty_page_handling_type: EmptyPageHandling,
            ) -> Self {
                SweepFinalizer {
                    platform_: platform,
                    stats_collector_: stats_collector,
                    space_: space,
                    unused_destroyed_normal_pages_: unused_destroyed_normal_pages,
                    free_memory_handling_: free_memory_handling,
                    empty_page_handling_: empty_page_handling_type,
                    largest_consecutive_block_: 0,
                }
            }

            /// Finalizes all space states, irrespective of deadlines and sizes.
            fn finalize_all(&mut self, states: &mut SpaceStates) {
                for state in states.iter_mut() {
                    self.finalize(state);
                }
            }

            fn finalize(&mut self, state: &mut SweepingState) {
                while let Some(mut page_state) = state.swept_unfinalized_pages.pop() {
                    self.finalize_page(&mut page_state);
                }
            }

            /// Finalizes a given SweepingState with a deadline and size. Only returns
            /// true if a single memory block of at least `size` bytes was returned to the
            /// free list and false otherwise.
            fn finalize_with_deadline_and_size(
                &mut self,
                scope_id: StatsCollectorScopeId,
                state: &mut SweepingState,
                deadline: TimeTicks,
                size: usize,
            ) -> bool {
                if state.swept_unfinalized_pages.is_empty() {
                    return false;
                }
                let _finalize_scope = StatsCollectorDisabledScope::new(self.stats_collector_, scope_id);
                let mut deadline_check = DeadlineChecker::new(deadline);
                while let Some(mut page_state) = state.swept_unfinalized_pages.pop() {
                    self.finalize_page(&mut page_state);
                    if size <= self.largest_consecutive_block_ {
                        return true;
                    }
                    if deadline_check.check() {
                        break;
                    }
                }
                false
            }

            /// Finalizes a given SweepingState with a deadline. Returns false if the
            /// deadline exceeded and true if all pages are finalized.
            fn finalize_with_deadline(
                &mut self,
                scope_id: StatsCollectorScopeId,
                state: &mut SweepingState,
                deadline: TimeTicks,
            ) -> bool {
                if state.swept_unfinalized_pages.is_empty() {
                    return true;
                }
                let _finalize_scope = StatsCollectorDisabledScope::new(self.stats_collector_, scope_id);
                let mut deadline_check = DeadlineChecker::new(deadline);
                while let Some(mut page_state) = state.swe
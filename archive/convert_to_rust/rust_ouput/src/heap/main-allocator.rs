// Converted from V8 C++ source files:
// Header: main-allocator.h
// Implementation: main-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct MutexGuard<'a> {
        mutex: &'a Mutex,
    }

    impl<'a> MutexGuard<'a> {
        pub fn new(mutex: &'a Mutex) -> Self {
            mutex.lock();
            MutexGuard { mutex }
        }
    }

    impl<'a> Drop for MutexGuard<'a> {
        fn drop(&mut self) {
            self.mutex.unlock();
        }
    }

    #[derive(Debug)]
    pub struct Mutex {
        locked: std::cell::Cell<bool>,
    }

    impl Mutex {
        pub const fn new() -> Mutex {
            Mutex {
                locked: std::cell::Cell::new(false),
            }
        }

        pub fn lock(&self) {
            while self.locked.get() {}
            self.locked.set(true);
        }

        pub fn unlock(&self) {
            self.locked.set(false);
        }
    }
}

use std::sync::Arc;
use std::sync::Mutex;

pub struct Heap;
pub struct LocalHeap;
pub struct MainAllocator;
pub struct PagedNewSpace;
pub struct PagedSpaceBase;
pub struct SemiSpaceNewSpace;
pub struct SpaceWithLinearArea;
pub struct AllocationObserver;
pub struct Address(*mut u8);

const KB: i32 = 1024;

pub enum AllocationAlignment {
    kTaggedAligned,
}

pub enum AllocationOrigin {
    kRuntime,
}

pub struct AllocationResult {
    address: Address,
    is_failure: bool,
}

impl AllocationResult {
    pub fn Failure() -> Self {
        AllocationResult {
            address: Address(std::ptr::null_mut()),
            is_failure: true,
        }
    }

    pub fn IsFailure(&self) -> bool {
        self.is_failure
    }

   pub fn ToAddress(&self) -> Address {
        self.address
    }
}

pub trait CreateAllocatorPolicy {
    fn CreateAllocatorPolicy(&mut self, allocator: &mut MainAllocator) -> Box<dyn AllocatorPolicy>;
}

impl<T: SpacePolicy> CreateAllocatorPolicy for T {
    fn CreateAllocatorPolicy(&mut self, allocator: &mut MainAllocator) -> Box<dyn AllocatorPolicy> {
        space_policy(self, allocator)
    }
}

pub trait SpacePolicy {}

fn space_policy<T: SpacePolicy>(
    space: &mut T,
    allocator: &mut MainAllocator,
) -> Box<dyn AllocatorPolicy> {
    Box::new(DefaultAllocatorPolicy {
        allocator: unsafe { std::mem::transmute(allocator) },
        space: unsafe { std::mem::transmute(space) },
    })
}

pub trait AllocatorPolicy {
    fn EnsureAllocation(
        &mut self,
        size_in_bytes: i32,
        alignment: AllocationAlignment,
        origin: AllocationOrigin,
    ) -> bool;
    fn FreeLinearAllocationArea(&mut self);
    fn SupportsExtendingLAB(&self) -> bool {
        false
    }

    fn space_heap(&self) -> *mut Heap;
    fn isolate_heap(&self) -> *mut Heap;
}

pub struct DefaultAllocatorPolicy {
    allocator: *mut MainAllocator,
    space: *mut dyn SpacePolicy,
}

impl AllocatorPolicy for DefaultAllocatorPolicy {
    fn EnsureAllocation(
        &mut self,
        size_in_bytes: i32,
        alignment: AllocationAlignment,
        origin: AllocationOrigin,
    ) -> bool {
        true
    }
    fn FreeLinearAllocationArea(&mut self) {}
    fn space_heap(&self) -> *mut Heap {
       std::ptr::null_mut()
    }

    fn isolate_heap(&self) -> *mut Heap {
        std::ptr::null_mut()
    }
}

struct V8_WARN_UNUSED_RESULT {}

struct V8_INLINE {}

struct V8_EXPORT_PRIVATE {}

struct AllocationCounter {}

impl AllocationCounter {
    fn IsStepInProgress(&self) -> bool {
        false
    }
    fn AddAllocationObserver(&mut self, _observer: *mut AllocationObserver) {}
    fn RemoveAllocationObserver(&mut self, _observer: *mut AllocationObserver) {}
    fn NextBytes(&self) -> i32 {
        0
    }
    fn InvokeAllocationObservers(&mut self, _address: Address, _size: usize, _size1: usize) {}
     fn AdvanceAllocationObservers(&mut self, _size: isize) {}
}

pub struct LinearAllocationArea {
    start: Address,
    top: Address,
    limit: Address,
}

impl LinearAllocationArea {
    pub fn start(&self) -> Address {
        self.start
    }
    pub fn top(&self) -> Address {
        self.top
    }
    pub fn limit(&self) -> Address {
        self.limit
    }
    pub fn top_address(&self) -> *mut Address {
        std::ptr::null_mut()
    }
    pub fn limit_address(&self) -> *mut Address {
        std::ptr::null_mut()
    }
    pub fn IncrementTop(&mut self, _size: i32) {}
    pub fn Reset(&mut self, _address: Address, _address1: Address) {}
    pub fn ResetStart(&mut self) {}
    pub fn SetLimit(&mut self, _address: Address) {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tagged<T> {
    address: Address,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn is_null(&self) -> bool {
        self.address.0.is_null()
    }

    pub fn address(&self) -> Address {
        self.address
    }

}

const kNullAddress: Address = Address(std::ptr::null_mut());

pub struct LinearAreaOriginalData {
    pub linear_area_lock_: base::Mutex,
}

impl LinearAreaOriginalData {
    pub fn get_original_top_acquire(&self) -> Address {
        Address(std::ptr::null_mut())
    }
    pub fn get_original_limit_relaxed(&self) -> Address {
        Address(std::ptr::null_mut())
    }
    pub fn set_original_top_release(&mut self, _address: Address) {}
    pub fn set_original_limit_relaxed(&mut self, _address: Address) {}
    pub fn linear_area_lock(&mut self) -> &mut base::Mutex {
        &mut self.linear_area_lock_
    }

}

pub struct SpaceWithLinearArea {}

pub struct AllocationSpace {}

const CODE_SPACE: AllocationSpace = AllocationSpace {};
const SHARED_SPACE: AllocationSpace = AllocationSpace {};
const NEW_SPACE: AllocationSpace = AllocationSpace {};

const kCodeAlignment: i32 = 16;
const kObjectAlignment8GbHeap: i32 = 8;
const kTaggedSize: i32 = 8;

fn ALIGN_TO_ALLOCATION_ALIGNMENT(size: i32) -> i32 {
    size
}

pub enum ThreadKind {
    kMain,
    kBackground,
}

pub enum TraceEventFlag {
    TRACE_EVENT_FLAG_FLOW_IN,
    TRACE_EVENT_FLAG_FLOW_OUT,
}

pub struct GCTracer {}

impl GCTracer {
    fn IsMajorMarking(&self) -> bool {
        false
    }
}

pub struct Sweeper {}

impl Sweeper {
    fn IsSweepingDoneForSpace(&self, _space: AllocationSpace) -> bool {
        false
    }

    fn AreMinorSweeperTasksRunning(&self) -> bool {
        false
    }

    fn ShouldRefillFreelistForSpace(&self, _space: AllocationSpace) -> bool {
        false
    }
    fn ParallelSweepSpace(
        &mut self,
        _space: AllocationSpace,
        _mode: SweepingMode,
        _pages: u32,
    ) -> bool {
        false
    }
    fn GetTracingScope(&self, _space: AllocationSpace, _is_main_thread: bool) -> i32 {
        0
    }
    fn GetTraceIdForFlowEvent(&self, _sweeping_scope_id: i32) -> i32 {
        0
    }
    fn WaitForPageToBeSwept(&self, _page: *mut PageMetadata) {}
}

pub enum SweepingMode {
    kEagerDuringGC,
    kLazyOrConcurrent,
}

fn IsAligned(value: usize, alignment: i32) -> bool {
    value % alignment as usize == 0
}

pub struct PageMetadata {}

impl PageMetadata {
    fn FromAddress(_address: Address) -> *mut Self {
        std::ptr::null_mut()
    }
    fn FromHeapObject(_object: Tagged<FreeSpace>) -> *mut Self {
        std::ptr::null_mut()
    }
    fn FromAllocationAreaAddress(_address: Address) -> *mut Self {
        std::ptr::null_mut()
    }
    fn IncreaseAllocatedLabSize(&mut self, _size: Address) {}
    fn DecreaseAllocatedLabSize(&mut self, _size: Address) {}
    fn owner_identity(&self) -> AllocationSpace {
        AllocationSpace {}
    }
    fn Chunk(&self) -> *mut MemoryChunk {
        std::ptr::null_mut()
    }
    fn CreateBlackArea(&mut self, _top: Address, _limit: Address) {}
    fn DestroyBlackArea(&mut self, _top: Address, _limit: Address) {}
    fn SweepingDone(&self) -> bool {
        false
    }
}

pub struct HeapObject {}
pub struct FreeSpace {}
pub struct MarkCompactCollector {}

impl MarkCompactCollector {
    fn IsOnEvacuationCandidate(_node: Tagged<FreeSpace>) -> bool {
        false
    }
}

pub struct MemoryChunk {}

impl MemoryChunk {
    fn IsFlagSet(&self, _flag: MemoryChunkFlag) -> bool {
        false
    }
}

pub enum MemoryChunkFlag {
    BLACK_ALLOCATED,
}

fn RoundDown(value: i32, alignment: i32) -> i32 {
    value - value % alignment
}

impl SpaceWithLinearArea {
     fn AreaSize(&mut self) -> usize {
        0
    }
}

pub struct PagedSpace {}

impl PagedSpace {
    fn ResetFreeList(&mut self) {}
    fn AllocatePage(&mut self) -> bool {
        false
    }
    fn AddPage(&mut self, _page: *mut PageMetadata) {}
    fn UsableCapacity(&self) -> usize {
        0
    }
    fn TotalCapacity(&self) -> usize {
        0
    }
     fn RemovePageSafe(&mut self, _size: i32) -> *mut PageMetadata {
        std::ptr::null_mut()
    }
    fn AddRangeToActiveSystemPages(&mut self, _page: *mut PageMetadata, _address: Address, _limit: Address) {}
    fn RefillFreeList(&mut self) {}
    fn IncreaseAllocatedBytes(&mut self, _size: usize, _page: *mut PageMetadata) {}
    fn TryExpand(&mut self, _local_heap: *mut LocalHeap, _origin: AllocationOrigin) -> bool {
        false
    }
}

pub struct SemiSpaceNewSpace {}

impl SemiSpaceNewSpace {
    fn Allocate(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
    ) -> std::option::Option<std::pair<Address, Address>> {
        None
    }
    fn AllocateOnNewPageBeyondCapacity(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
    ) -> std::option::Option<std::pair<Address, Address>> {
        None
    }
    fn Free(&mut self, _address: Address, _limit: Address) {}
    fn AddRangeToActiveSystemPages(&mut self, _address: Address, _limit: Address) {}
     fn mutex(&mut self) -> &mut base::Mutex{
        todo!()
    }
    fn heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
    fn to_space(&self) -> &SemiSpace{}
     fn Free(&mut self, limit: Address, end: Address) {}
}

impl SpacePolicy for SemiSpaceNewSpace {}

pub struct PagedNewSpace {}
impl SpacePolicy for PagedNewSpace {}

impl PagedNewSpace {
    fn paged_space(&mut self) -> *mut PagedSpace {
        std::ptr::null_mut()
    }
    fn heap(&mut self) -> *mut Heap{
        std::ptr::null_mut()
    }
}

pub struct ConcurrentMarking {}

impl ConcurrentMarking {
     fn RescheduleJobIfNeeded(&mut self, _mark_sweeper: GarbageCollector) {}
}

pub enum GarbageCollector {
    MINOR_MARK_SWEEPER
}

pub struct MarkingState {}

impl MarkingState {
    fn IsUnmarked(&self, _object: HeapObject) -> bool {
        false
    }
}

impl Heap {
    fn GetFillToAlign(address: Address, alignment: AllocationAlignment) -> i32 {
        0
    }
    fn GetMaximumFillToAlign(alignment: AllocationAlignment) -> i32 {
        0
    }
    fn CreateFillerObjectAt(address: Address, size: i32) {}
     fn ShouldExpandYoungGenerationOnSlowAllocation(&mut self, _page_size: i32) -> bool{
        false
    }
    fn CanExpandOldGeneration(&mut self, _area_size: usize) -> bool {
        false
    }
    fn StartIncrementalMarkingIfAllocationLimitIsReached(
        &mut self,
        _local_heap: *mut LocalHeap,
        _gcflags: i32,
        _gc_callback_schedule_idle_garbage_collection: i32,
    ) {
    }
    fn StartMinorMSIncrementalMarkingIfNeeded(&mut self) {}
    fn incremental_marking(&mut self) -> &mut IncrementalMarking {
        todo!()
    }
    fn GCFlagsForIncrementalMarking(&self) -> i32 {
        0
    }
    fn paged_space(&self, _id: AllocationSpace) -> *mut PagedSpace {
        std::ptr::null_mut()
    }
    fn sweeping_in_progress(&self) -> bool {
        false
    }
    fn IsMainThread(&self) -> bool {
        false
    }
    fn sweeper(&mut self) -> &mut Sweeper {
        todo!()
    }
     fn force_oom(&self) -> bool {
        false
    }
     fn ShouldExpandOldGenerationOnSlowAllocation(
        &mut self,
        _local_heap: *mut LocalHeap,
        _origin: AllocationOrigin,
    ) -> bool {
        false
    }
    fn marking_state(&self) -> &MarkingState{
        todo!()
    }
    fn concurrent_marking(&mut self) -> &mut ConcurrentMarking{
        todo!()
    }
    fn tracer(&self) -> &GCTracer{
        todo!()
    }
    fn isolate(&self) -> *mut Isolate {
        std::ptr::null_mut()
    }
     fn IsAllocationObserverActive(&self) -> bool{
        false
    }
    fn IsInGC(&self) -> bool {
        false
    }
    fn incremental_marking(&self) -> &IncrementalMarking{
        todo!()
    }
}

pub struct IncrementalMarking {}

impl IncrementalMarking {
    fn black_allocation(&self) -> bool {
        false
    }
    fn IsMajorMarking(&self) -> bool {
        false
    }
     fn IsMinorMarking(&self) -> bool {
        false
    }
}

pub struct Isolate {}
pub struct SemiSpace {}

impl MainAllocator {
    pub struct InGCTag {}
    const kInGC: InGCTag = MainAllocator::InGCTag {};

    pub enum class IsNewGeneration {
        kNo,
        kYes,
    }

    pub fn new(
        local_heap: *mut LocalHeap,
        space: *mut SpaceWithLinearArea,
        is_new_generation: MainAllocator::IsNewGeneration,
        allocation_info: *mut LinearAllocationArea,
    ) -> Self {
        MainAllocator {
            local_heap: local_heap,
            isolate_heap: unsafe { (*local_heap).heap() },
            space: space,
            allocation_info: if allocation_info.is_null() {
                std::ptr::null_mut()
                // Box::into_raw(Box::new(LinearAllocationArea {
                //     start: kNullAddress,
                //     top: kNullAddress,
                //     limit: kNullAddress,
                // }))
            } else {
                allocation_info
            },
            owned_allocation_info_: LinearAllocationArea {
                start: kNullAddress,
                top: kNullAddress,
                limit: kNullAddress,
            },
            linear_area_original_data_: None,
            allocator_policy_:
            unsafe {
                if space.is_null(){
                   std::ptr::null_mut()
                }
                else {
                    (*space).CreateAllocatorPolicy(&mut MainAllocator {
                        local_heap: local_heap,
                        isolate_heap: unsafe { (*local_heap).heap() },
                        space: space,
                        allocation_info: if allocation_info.is_null() {
                            std::ptr::null_mut()
                        } else {
                            allocation_info
                        },
                        owned_allocation_info_: LinearAllocationArea {
                            start: kNullAddress,
                            top: kNullAddress,
                            limit: kNullAddress,
                        },
                        linear_area_original_data_: None,
                        allocator_policy_: std::ptr::null_mut(),
                        supports_extending_lab_: false,
                        black_allocation_: MainAllocator::BlackAllocation::kAlwaysDisabled,
                        allocation_counter_: None,
                    }) as *mut dyn AllocatorPolicy
                }
            } ,
            supports_extending_lab_: false,
            black_allocation_: MainAllocator::BlackAllocation::kAlwaysDisabled,
            allocation_counter_: None,
        }
    }

    pub fn new_in_gc(heap: *mut Heap, space: *mut SpaceWithLinearArea, _tag: MainAllocator::InGCTag) -> Self {
        MainAllocator {
            local_heap: std::ptr::null_mut(),
            isolate_heap: heap,
            space: space,
            allocation_info:std::ptr::null_mut() ,
            owned_allocation_info_: LinearAllocationArea {
                start: kNullAddress,
                top: kNullAddress,
                limit: kNullAddress,
            },
            linear_area_original_data_: None,
            allocator_policy_: unsafe {
                 if space.is_null(){
                   std::ptr::null_mut()
                }
                else {
                    (*space).CreateAllocatorPolicy(&mut MainAllocator {
                        local_heap: std::ptr::null_mut(),
                        isolate_heap: heap,
                        space: space,
                        allocation_info:std::ptr::null_mut() ,
                        owned_allocation_info_: LinearAllocationArea {
                            start: kNullAddress,
                            top: kNullAddress,
                            limit: kNullAddress,
                        },
                        linear_area_original_data_: None,
                        allocator_policy_: std::ptr::null_mut(),
                        supports_extending_lab_: false,
                        black_allocation_: MainAllocator::BlackAllocation::kAlwaysDisabled,
                        allocation_counter_: None,
                    }) as *mut dyn AllocatorPolicy
                 }
            },
            supports_extending_lab_: false,
            black_allocation_: MainAllocator::BlackAllocation::kAlwaysDisabled,
            allocation_counter_: None,
        }
    }
    pub fn start(&self) -> Address {
        Address(std::ptr::null_mut())
    }

    pub fn top(&self) -> Address {
        Address(std::ptr::null_mut())
    }

    pub fn limit(&self) -> Address {
        Address(std::ptr::null_mut())
    }
      pub fn allocation_top_address(&self) -> *mut Address {
        std::ptr::null_mut()
    }
     pub fn allocation_limit_address(&self) -> *mut Address {
        std::ptr::null_mut()
    }
    pub fn original_top_acquire(&self) -> Address {
        Address(std::ptr::null_mut())
    }

    pub fn original_limit_relaxed(&self) -> Address {
        Address(std::ptr::null_mut())
    }
    pub fn MoveOriginalTopForward(&mut self) {}
    pub fn ResetLab(&mut self, _start: Address, _end: Address, _extended_end: Address) {}
    pub fn IsPendingAllocation(&self, _object_address: Address) -> bool {
        false
    }
    pub fn allocation_info(&self) -> &LinearAllocationArea {
        &self.owned_allocation_info_
    }
     pub fn allocation_info_mut(&mut self) -> &mut LinearAllocationArea {
        &mut self.owned_allocation_info_
    }
    pub fn allocation_counter(&self) -> &AllocationCounter {
        todo!()
    }
    pub fn allocation_counter_mut(&mut self) -> &mut AllocationCounter{
        todo!()
    }
    pub fn AllocateRaw(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }
    pub fn AllocateRawForceAlignmentForTesting(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }
    pub fn AddAllocationObserver(&mut self, _observer: *mut AllocationObserver) {}
    pub fn RemoveAllocationObserver(&mut self, _observer: *mut AllocationObserver) {}
    pub fn PauseAllocationObservers(&mut self) {}
    pub fn ResumeAllocationObservers(&mut self) {}
    pub fn AdvanceAllocationObservers(&mut self) {}
    pub fn InvokeAllocationObservers(
        &mut self,
        _soon_object: Address,
        _size_in_bytes: usize,
        _aligned_size_in_bytes: usize,
        _allocation_size: usize,
    ) {
    }
    pub fn MakeLinearAllocationAreaIterable(&mut self) {}
    pub fn MarkLinearAllocationAreaBlack(&mut self) {}
    pub fn UnmarkLinearAllocationArea(&mut self) {}
    pub fn FreeLinearAllocationAreaAndResetFreeList(&mut self) {}
    pub fn AlignTopForTesting(&mut self, _alignment: AllocationAlignment, _offset: i32) -> Address {
        Address(std::ptr::null_mut())
    }
    pub fn TryFreeLast(&self, _object_address: Address, _object_size: i32) -> bool {
        false
    }
    pub fn IsLabValid(&self) -> bool {
        false
    }
    pub fn FreeLinearAllocationArea(&mut self) {}
    pub fn ExtendLAB(&mut self, _limit: Address) {}
    pub fn EnsureAllocationForTesting(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> bool {
        false
    }
    fn ComputeBlackAllocation(
        _is_new_generation: MainAllocator::IsNewGeneration,
    ) -> MainAllocator::BlackAllocation {
        MainAllocator::BlackAllocation::kAlwaysDisabled
    }
    pub fn AllocateFastUnaligned(
        &mut self,
        _size_in_bytes: i32,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }

    pub fn AllocateFastAligned(
        &mut self,
        _size_in_bytes: i32,
        _result_aligned_size_in_bytes: *mut i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }

    pub fn AllocateRawSlow(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }
    pub fn AllocateRawSlowUnaligned(
        &mut self,
        _size_in_bytes: i32,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }

    pub fn AllocateRawSlowAligned(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> AllocationResult {
        AllocationResult{address:Address(std::ptr::null_mut()), is_failure: false}
    }
    fn EnsureAllocation(
        &mut self,
        _size_in_bytes: i32,
        _alignment: AllocationAlignment,
        _origin: AllocationOrigin,
    ) -> bool {
        false
    }
    fn MarkLabStartInitialized(&mut self) {}
    fn IsBlackAllocationEnabled(&self) -> bool {
        false
    }
    fn linear_area_original_data_mut(&mut self) -> &mut LinearAreaOriginalData {
        todo!()
    }

    fn linear_area_original_data(&self) -> &LinearAreaOriginalData {
        todo!()
    }
    fn ObjectAlignment(&self) -> i32 {
        0
    }
    fn identity(&self) -> AllocationSpace {
        AllocationSpace {}
    }
    fn SupportsAllocationObserver(&self) -> bool {
        false
    }
    fn SupportsPendingAllocation(&self) -> bool {
        false
    }
    fn in_gc(&self) -> bool {
        false
    }
    fn in_gc_for_space(&self) -> bool {
        false
    }
    fn supports_extending_lab(&self) -> bool {
        false
    }
    fn is_main_thread(&self) -> bool {
        false
    }
     fn local_heap(&self) -> *mut LocalHeap {
        self.local_heap
    }
     fn isolate_heap(&self) -> *mut Heap {
        self.isolate_heap
    }
    pub fn space_heap(&self) -> *mut Heap {
        std::ptr::null_mut()
    }
}

impl Drop for MainAllocator {
    fn drop(&mut self) {
        unsafe {
            if !self.allocator_policy_.is_null() {
                drop(Box::from_raw(self.allocator_policy_));
            }
        }
    }
}

impl SemiSpaceNewSpaceAllocatorPolicy {
    pub fn new(space: *mut SemiSpaceNewSpace, allocator: *mut MainAllocator) -> Self {
        SemiSpaceNewSpaceAllocatorPolicy {
            space: space,
            allocator: allocator,
        }
    }
}

impl PagedNewSpaceAllocatorPolicy {
    pub fn new(space: *mut PagedNewSpace, allocator: *mut MainAllocator) -> Self {
        PagedNewSpaceAllocatorPolicy {
            space: space,
            allocator: allocator,
            paged_space_allocator_policy_: Box::new(PagedSpaceAllocatorPolicy {
                space: unsafe { (*space).paged_space() },
                allocator: allocator,
            }),
        }
    }
}

impl Default for LinearAreaOriginalData {
     fn default() -> Self {
        LinearAreaOriginalData {
            linear_area_lock_: base::Mutex::new(),
        }
    }
}
impl Default for MainAllocator {
     fn default() -> Self {
         let null_heap = std::ptr::null_mut();
        MainAllocator {
            local_heap: null_heap,
            isolate_heap: null_heap,
            space: null_heap,
            allocation_info:std::ptr::null_mut(),
            owned_allocation_info_: LinearAllocationArea {
                start: kNullAddress,
                top: kNullAddress,
                limit: kNullAddress,
            },
            linear_area_original_data_: None,
            allocator_policy_: std::ptr::null_mut(),
            supports_extending_lab_: false,
            black_allocation_: MainAllocator::BlackAllocation::kAlwaysDisabled,
            allocation_counter_: None,
        }
    }
}

pub struct SemiSpaceNewSpaceAllocatorPolicy {
    space: *mut SemiSpaceNewSpace,
    allocator: *mut MainAllocator,
}

impl AllocatorPolicy for SemiSpaceNewSpaceAllocatorPolicy {
    fn EnsureAllocation(
        &mut self,
        size_in_bytes: i32,
        alignment: AllocationAlignment,
        origin: AllocationOrigin,
    ) -> bool {
        unsafe {
            
        }
        true
    }
    fn FreeLinearAllocationArea(&mut self) {}

    fn space_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
     fn isolate_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
}

pub struct PagedNewSpaceAllocatorPolicy {
    space: *mut PagedNewSpace,
    allocator: *mut MainAllocator,
    paged_space_allocator_policy_: Box<PagedSpaceAllocatorPolicy>,
}
impl AllocatorPolicy for PagedNewSpaceAllocatorPolicy {
    fn EnsureAllocation(
        &mut self,
        size_in_bytes: i32,
        alignment: AllocationAlignment,
        origin: AllocationOrigin,
    ) -> bool {
        unsafe {
            
        }
        true
    }
    fn FreeLinearAllocationArea(&mut self) {}

     fn space_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
     fn isolate_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }

     fn SupportsExtendingLAB(&self) -> bool {
        true
    }
}
pub struct PagedSpaceAllocatorPolicy {
    space: *mut PagedSpace,
    allocator: *mut MainAllocator,
}

impl AllocatorPolicy for PagedSpaceAllocatorPolicy {
    fn EnsureAllocation(
        &mut self,
        size_in_bytes: i32,
        alignment: AllocationAlignment,
        origin: AllocationOrigin,
    ) -> bool {
        unsafe {
           
        }
        true
    }
    fn FreeLinearAllocationArea(&mut self) {}

     fn space_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
    fn isolate_heap(&self) -> *mut Heap{
        std::ptr::null_mut()
    }
}

pub enum RuntimeCallCounterId {
    kGC_Custom_SlowAllocateRaw,
}

pub struct RuntimeCallTimerScope {}

impl RuntimeCallTimerScope {
    pub fn new(_isolate: *mut Isolate, _id: RuntimeCallCounterId) -> Self {
        RuntimeCallTimerScope {}
    }
}

pub struct VMState<T> {
    phantom: std::marker::PhantomData<T>,
}

impl VMState<Heap>{

}

pub struct FreeListCategory {}

impl FreeListCategory {
    fn is_empty(&self) -> bool {
        false
    }
    fn is_linked(&self, _free_list: &FreeList) -> bool {
        false
    }
}

pub struct FreeList {}

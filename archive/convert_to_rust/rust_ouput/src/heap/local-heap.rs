// Converted from V8 C++ source files:
// Header: local-heap.h
// Implementation: local-heap.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]
use std::sync::atomic::{AtomicU8, Ordering};
use std::ptr::null_mut;
use std::marker::PhantomData;

use std::sync::{Mutex, RwLock};

use std::ops::Deref;
use std::ops::DerefMut;

use std::convert::TryInto;

pub struct PersistentHandles {}
pub struct ThreadKind {}
impl ThreadKind {
    const kMain: ThreadKind = ThreadKind {};
}
pub struct LocalHandles {}
pub struct HeapAllocator {}
pub struct Safepoint {}
pub struct Isolate {}
pub struct LinearAllocationArea {}
pub struct MarkingBarrier {}
pub struct ReadOnlySpace {}
pub struct DescriptorArray {}
pub struct AllocationResult {}
pub struct HeapObject {}
pub struct ClearRecordedSlots {}
pub struct AllocationSpace {}
pub struct PagedSpace {}
pub struct JSArrayBuffer {}
pub struct String {}
pub struct SharedFunctionInfo {}
pub struct Operand {}
pub enum Condition {}
pub enum GCType {}
pub struct CpuFeatures {}
pub enum RootIndex {}
pub struct Block {}
pub struct Operation {}
pub struct Register {}
pub enum AtomicMemoryOrder {}
pub struct UnalignedSlot<T> {dummy : i32, phantom : PhantomData<T>}
pub struct LookupIteratorState {}
pub struct JSDisposableStack {}
pub struct FixedArray {}
pub struct AbortReason {}
pub struct DisplayNamesInternal {}
pub struct Managed<T> {dummy : i32, phantom : PhantomData<T>}
pub struct MaybeObject {}
pub enum Mode {}
pub enum GCType {}
pub enum AllocationType {}
pub enum AllocationAlignment {}
impl AllocationAlignment {
    const kTaggedAligned: AllocationAlignment = AllocationAlignment {};
}
pub enum AllocationOrigin {}
impl AllocationOrigin {
    const kRuntime: AllocationOrigin = AllocationOrigin {};
}

#[repr(C)]
pub struct Address {}

struct Tagged<T> {dummy : i32, phantom : PhantomData<T>}

struct MaybeDirectHandle<T> {dummy : i32, phantom : PhantomData<T>}

impl<T> MaybeDirectHandle<T> {
    pub fn ToHandle(&self, handle: &mut DirectHandle<T>) -> bool {
        true
    }
}
struct DirectHandle<T> {dummy : i32, phantom : PhantomData<T>}

struct PtrComprCageAccessScope {}

impl<T> From<DirectHandle<T>> for IndirectHandle<T> {
    fn from(_handle: DirectHandle<T>) -> Self {
        IndirectHandle {dummy : 0, phantom : PhantomData}
    }
}

const kTaggedCanConvertToRawObjects : bool = true;
const kNullMaybeHandle : MaybeIndirectHandle<i32> = MaybeIndirectHandle{dummy : 0, phantom : PhantomData};

#[derive(Clone, Copy)]
struct AllowSafepoints {}
impl AllowSafepoints {
    fn IsAllowed() -> bool { true }
}

struct GlobalHandleVector<T> {dummy : i32, phantom : PhantomData<T>}

struct IgnoreLocalGCRequests {}

impl IgnoreLocalGCRequests {
    pub fn new(_heap: &Heap) -> Self { IgnoreLocalGCRequests{} }
}

struct GCCallbacksInSafepoint {}
impl GCCallbacksInSafepoint {
    fn Add(&mut self, _callback: *mut fn(void), _data: *mut void, _gc_type: GCCallbacksInSafepoint::GCType) {}
    fn Remove(&mut self, _callback: *mut fn(void), _data: *mut void) {}
    fn Invoke(&mut self, _gc_type: GCCallbacksInSafepoint::GCType) {}
    fn IsEmpty(&self) -> bool { true }

    pub enum GCType {
        kAll
    }
}

struct GCTracer {}
impl GCTracer {
    fn Scope(a: i32, b: i32) -> i32 { 0 }
}

struct Heap {
    gc_tracer_: GCTracer,
    safepoint_: Safepoint,
    incremental_marking_: IncrementalMarking,
    collection_barrier_: CollectionBarrier,
    isolate_: Isolate,
    deserialization_complete_: bool,
    read_only_space_: ReadOnlySpace
}

impl Heap {
    fn new() -> Self {
        Heap {
            gc_tracer_: GCTracer {},
            safepoint_: Safepoint {},
            incremental_marking_: IncrementalMarking {},
            collection_barrier_: CollectionBarrier {},
            isolate_: Isolate {},
            deserialization_complete_: false,
            read_only_space_: ReadOnlySpace {}
        }
    }
    fn isolate(&self) -> &Isolate { &self.isolate_ }
    fn safepoint(&self) -> &Safepoint { &self.safepoint_ }
    fn deserialization_complete(&self) -> bool { self.deserialization_complete_ }
    fn read_only_space(&self) -> &ReadOnlySpace { &self.read_only_space_ }
    fn incremental_marking(&self) -> &IncrementalMarking { &self.incremental_marking_ }
    fn ignore_local_gc_requests(&self) -> bool { false }
    fn gc_tracer(&self) -> &GCTracer { &self.gc_tracer_ }
    fn collection_barrier(&self) -> &CollectionBarrier { &self.collection_barrier_ }

    fn CollectGarbageForBackground(&self, local_heap: *mut LocalHeap) {}
    fn NotifyObjectSizeChange(&self, object: Tagged<HeapObject>, old_size: i32, new_size: i32, clear_recorded_slots: ClearRecordedSlots) {}
    fn WeakenDescriptorArrays(&self, strong_descriptor_arrays: GlobalHandleVector<DescriptorArray>) {}
}

struct IncrementalMarking {}
impl IncrementalMarking {
    fn IsMarking(&self) -> bool { false }
    fn IsCompacting(&self) -> bool { false }
    fn marking_mode(&self) -> i32 { 0 }
    fn IsMajorMarking(&self) -> bool { false }
}

struct IsolateSafepoint {}

struct IsolateSafepointScope {}

struct CollectionBarrier {}
impl CollectionBarrier {
    fn CancelCollectionAndResumeThreads(&self) {}
}

mod i {
    pub struct IsolateForSandbox {}
}

mod base {
    pub mod platform {
        pub struct Mutex {}
        impl Mutex {
            pub fn new() -> Self { Mutex {} }
        }
        pub struct ConditionVariable {}
        impl ConditionVariable {
            pub fn new() -> Self { ConditionVariable {} }
            pub fn NotifyOne(&self) {}
            pub fn Wait(&self, _mutex: &Mutex) {}
        }
    }
    pub struct BitField8<T, const OFFSET: usize, const SIZE: usize> {dummy : i32, phantom : PhantomData<T>}
    impl<T, const OFFSET: usize, const SIZE: usize> BitField8<T, OFFSET, SIZE> {
        const kMask : u8 = 0;
        fn decode(_raw_state_: u8) -> bool { false }
        type Next<U, const NEXT_OFFSET: usize> = BitField8<U, NEXT_OFFSET, 1>;
    }
}

struct HeapAllocator {
    local_heap_: *const LocalHeap,
    shared_space_allocator_: Option<SharedSpaceAllocator>
}

impl HeapAllocator {
    fn new() -> Self { HeapAllocator { local_heap_: null_mut(), shared_space_allocator_: None } }
    fn Setup(&mut self) {}
    fn FreeLinearAllocationAreas(&mut self) {}
    fn VerifyLinearAllocationAreas(&self) {}
    fn MakeLinearAllocationAreasIterable(&mut self) {}
    fn MarkLinearAllocationAreasBlack(&mut self) {}
    fn UnmarkLinearAllocationsArea(&mut self) {}
    fn FreeLinearAllocationAreasAndResetFreeLists(&mut self) {}
    fn shared_space_allocator(&self) -> &Option<SharedSpaceAllocator> { &self.shared_space_allocator_ }
    fn Setup(_new_allocation_info: &LinearAllocationArea, _old_allocation_info: &LinearAllocationArea) {}

    V8_WARN_UNUSED_RESULT inline fn AllocateRaw(
        &mut self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment
    ) -> AllocationResult {
        AllocationResult {}
    }
}

impl HeapAllocator {
    pub fn with<const mode: HeapAllocator::AllocationRetryMode>() -> Self {
        HeapAllocator { local_heap_: null_mut(), shared_space_allocator_: None }
    }
    pub enum AllocationRetryMode {}
}

struct SharedSpaceAllocator {

}

impl SharedSpaceAllocator {
    fn MarkLinearAllocationAreaBlack(&mut self) {}
    fn UnmarkLinearAllocationArea(&mut self) {}
    fn FreeLinearAllocationAreaAndResetFreeList(&mut self) {}
}

#[derive(PartialEq, Eq)]
struct ThreadState {
    raw_state_: u8,
}

impl ThreadState {
    const fn Parked() -> Self {
        ThreadState { raw_state_: 0 }
    }
    const fn Running() -> Self {
        ThreadState { raw_state_: 0 }
    }
    const fn IsRunning(&self) -> bool {
        self.raw_state_ == 0
    }
    const fn SetRunning(&self) -> Self {
        ThreadState { raw_state_: 0 }
    }
    const fn IsParked(&self) -> bool {
        self.raw_state_ != 0
    }
    const fn SetParked(&self) -> Self {
        ThreadState { raw_state_: 0 }
    }
    const fn IsSafepointRequested(&self) -> bool {
        false
    }
    const fn IsCollectionRequested(&self) -> bool {
        false
    }
    const fn IsRunningWithSlowPathFlag(&self) -> bool {
        false
    }
}

struct AtomicThreadState {
    raw_state_: AtomicU8,
}

impl AtomicThreadState {
    fn new(state: ThreadState) -> Self {
        AtomicThreadState { raw_state_: AtomicU8::new(state.raw_state_) }
    }

    fn CompareExchangeStrong(&self, expected: &mut ThreadState, updated: ThreadState) -> bool {
        self.raw_state_.compare_exchange(
            expected.raw_state_,
            updated.raw_state_,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ).is_ok()
    }

    fn CompareExchangeWeak(&self, expected: &mut ThreadState, updated: ThreadState) -> bool {
        self.raw_state_.compare_exchange_weak(
            expected.raw_state_,
            updated.raw_state_,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ).is_ok()
    }

    fn SetParked(&self) -> ThreadState {
        let _ = self.raw_state_.fetch_or(1, Ordering::SeqCst);
        ThreadState { raw_state_: 0 }
    }

    fn SetSafepointRequested(&self) -> ThreadState {
        ThreadState { raw_state_: 0 }
    }

    fn ClearSafepointRequested(&self) -> ThreadState {
        ThreadState { raw_state_: 0 }
    }

    fn SetCollectionRequested(&self) -> ThreadState {
        ThreadState { raw_state_: 0 }
    }

    fn ClearCollectionRequested(&self) -> ThreadState {
        ThreadState { raw_state_: 0 }
    }

    fn load_relaxed(&self) -> ThreadState {
        ThreadState { raw_state_: self.raw_state_.load(Ordering::Relaxed) }
    }
}

impl ThreadState {
    const fn raw(&self) -> u8 {
        self.raw_state_
    }
}

#[derive(Debug)]
struct LocalHeap {
    heap_: *mut Heap,
    ptr_compr_cage_access_scope_: PtrComprCageAccessScope,
    is_main_thread_: bool,
    state_: AtomicThreadState,
    allocation_failed_: bool,
    nested_parked_scopes_: i32,
    prev_: *mut LocalHeap,
    next_: *mut LocalHeap,
    handles_: Box<LocalHandles>,
    persistent_handles_: Option<Box<PersistentHandles>>,
    marking_barrier_: Option<Box<MarkingBarrier>>,
    gc_epilogue_callbacks_: GCCallbacksInSafepoint,
    heap_allocator_: HeapAllocator,
    saved_marking_barrier_: *mut MarkingBarrier,
    stack_: ::heap::base::Stack,
    saved_current_isolate_: *mut Isolate,
}

impl LocalHeap {
    fn new(heap: *mut Heap, kind: ThreadKind, persistent_handles: Option<Box<PersistentHandles>>) -> Self {
        let is_main_thread_ = kind.type_id() == ThreadKind::kMain.type_id();
        let mut local_heap = LocalHeap {
            heap_: heap,
            ptr_compr_cage_access_scope_: PtrComprCageAccessScope {},
            is_main_thread_: is_main_thread_,
            state_: AtomicThreadState::new(ThreadState::Parked()),
            allocation_failed_: false,
            nested_parked_scopes_: 0,
            prev_: null_mut(),
            next_: null_mut(),
            handles_: Box::new(LocalHandles {}),
            persistent_handles_: persistent_handles,
            marking_barrier_: None,
            gc_epilogue_callbacks_: GCCallbacksInSafepoint {},
            heap_allocator_: HeapAllocator::new(),
            saved_marking_barrier_: null_mut(),
            stack_: ::heap::base::Stack::new(),
            saved_current_isolate_: null_mut()
        };

        if !local_heap.is_main_thread() {
            local_heap.heap_allocator_.Setup();
            local_heap.SetUpMarkingBarrier();
        }
        unsafe {
            (*heap).safepoint().AddLocalHeap(&mut local_heap, || {
                if !local_heap.is_main_thread() {
                    local_heap.saved_marking_barrier_ =
                        WriteBarrier::SetForThread(local_heap.marking_barrier_.as_deref().unwrap() as *const MarkingBarrier as *mut MarkingBarrier);
                    if (*heap).incremental_marking().IsMarking() {
                        local_heap.marking_barrier_.as_mut().unwrap().Activate(
                            (*heap).incremental_marking().IsCompacting(),
                            (*heap).incremental_marking().marking_mode(),
                        );
                    }

                    local_heap.SetUpSharedMarking();
                }
            });
        }

        if local_heap.persistent_handles_.is_some() {
            local_heap.persistent_handles_.as_mut().unwrap().Attach(&mut local_heap);
        }

        if !local_heap.is_main_thread() {
            unsafe {
                local_heap.saved_current_isolate_ = Isolate::TryGetCurrent();
                Isolate::SetCurrent((*heap).isolate());
                LocalHeap::SetCurrent(&mut local_heap);
            }
        }

        return local_heap;
    }

    fn Safepoint(&self) {
        if !AllowSafepoints::IsAllowed() {
            return;
        }
        let current = self.state_.load_relaxed();
        if current.IsRunningWithSlowPathFlag() {
            self.SafepointSlowPath();
        }
    }

    fn handles(&mut self) -> &mut LocalHandles {
        &mut self.handles_
    }

    fn NewPersistentHandle<T>(&mut self, object: Tagged<T>) -> IndirectHandle<T> {
        if self.persistent_handles_.is_none() {
            self.EnsurePersistentHandles();
        }
        self.persistent_handles_.as_mut().unwrap().NewHandle(object)
    }

    fn NewPersistentHandle2<T>(&mut self, object: DirectHandle<T>) -> IndirectHandle<T> {
        if self.persistent_handles_.is_none() {
            self.EnsurePersistentHandles();
        }
        self.NewPersistentHandle(object)
    }

    fn NewPersistentHandle3<T>(&mut self, object: T) -> IndirectHandle<T> {
        if self.persistent_handles_.is_none() {
            self.EnsurePersistentHandles();
        }
        self.NewPersistentHandle(Tagged::<T> {dummy : 0, phantom : PhantomData})
    }

     fn NewPersistentMaybeHandle<T>(&mut self, maybe_handle: MaybeDirectHandle<T>) -> MaybeIndirectHandle<T> {
        let mut handle: DirectHandle<T> = DirectHandle{dummy : 0, phantom : PhantomData};
        if maybe_handle.ToHandle(&mut handle) {
            return self.NewPersistentHandle(handle).into();
        }
        kNullMaybeHandle
    }

    fn AttachPersistentHandles(&mut self, persistent_handles: Box<PersistentHandles>) {
        if self.persistent_handles_.is_some() {
            panic!("Persistent handles already attached");
        }
        self.persistent_handles_ = Some(persistent_handles);
        self.persistent_handles_.as_mut().unwrap().Attach(self);
    }

    fn DetachPersistentHandles(&mut self) -> Option<Box<PersistentHandles>> {
        if self.persistent_handles_.is_some() {
            self.persistent_handles_.as_mut().unwrap().Detach();
        }
        self.persistent_handles_.take()
    }

    fn IsParked(&self) -> bool {
        self.state_.load_relaxed().IsParked()
    }

    fn IsRunning(&self) -> bool {
        self.state_.load_relaxed().IsRunning()
    }

    fn IsRetryOfFailedAllocation(&self) -> bool {
        self.allocation_failed_
    }

    fn SetRetryOfFailedAllocation(&mut self, value: bool) {
        self.allocation_failed_ = value;
    }

    fn heap(&self) -> &Heap {
        unsafe { &*self.heap_ }
    }

    fn AsHeap(&self) -> &Heap {
        self.heap()
    }

    fn marking_barrier(&self) -> &MarkingBarrier {
        self.marking_barrier_.as_ref().unwrap()
    }

    fn FreeLinearAllocationAreas(&mut self) {
        self.heap_allocator_.FreeLinearAllocationAreas();
    }

    fn MakeLinearAllocationAreasIterable(&mut self) {
        self.heap_allocator_.MakeLinearAllocationAreasIterable();
    }

    fn MarkLinearAllocationAreasBlack(&mut self) {
        self.heap_allocator_.MarkLinearAllocationAreasBlack();
    }

    fn UnmarkLinearAllocationsArea(&mut self) {
        self.heap_allocator_.UnmarkLinearAllocationsArea();
    }

    fn MarkSharedLinearAllocationAreasBlack(&mut self) {
        if let Some(ssa) = &mut self.heap_allocator_.shared_space_allocator_ {
            ssa.MarkLinearAllocationAreaBlack();
        }
    }

    fn UnmarkSharedLinearAllocationsArea(&mut self) {
        if let Some(ssa) = &mut self.heap_allocator_.shared_space_allocator_ {
            ssa.UnmarkLinearAllocationArea();
        }
    }

    fn FreeLinearAllocationAreasAndResetFreeLists(&mut self) {
        self.heap_allocator_.FreeLinearAllocationAreasAndResetFreeLists();
    }

    fn FreeSharedLinearAllocationAreasAndResetFreeLists(&mut self) {
        if let Some(ssa) = &mut self.heap_allocator_.shared_space_allocator_ {
            ssa.FreeLinearAllocationAreaAndResetFreeList();
        }
    }

    fn AllocateRaw(
        &mut self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment
    ) -> AllocationResult {
        self.heap_allocator_.AllocateRaw(size_in_bytes, allocation, origin, alignment)
    }

    fn AllocateRawWith<const mode: HeapAllocator::AllocationRetryMode>(
        &mut self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment
    ) -> Tagged<HeapObject> {
       Tagged{dummy : 0, phantom : PhantomData}
    }

    fn AllocateRawOrFail(
        &mut self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment
    ) -> Address {
        Address {}
    }

    fn NotifyObjectSizeChange(
        &mut self,
        object: Tagged<HeapObject>,
        old_size: i32,
        new_size: i32,
        clear_recorded_slots: ClearRecordedSlots
    ) {
        unsafe {(*self.heap_).NotifyObjectSizeChange(object, old_size, new_size, clear_recorded_slots)};
    }

    fn is_main_thread(&self) -> bool {
        self.is_main_thread_
    }

    fn is_main_thread_for(&self, heap: *mut Heap) -> bool {
        self.is_main_thread() && self.heap_ as *mut Heap == heap
    }

    fn is_in_trampoline(&self) -> bool {
        false
    }

    fn deserialization_complete(&self) -> bool {
        unsafe { (*self.heap_).deserialization_complete() }
    }

    fn read_only_space(&self) -> &ReadOnlySpace {
        unsafe { (*self.heap_).read_only_space() }
    }

    fn AddGCEpilogueCallback(&mut self, callback: *mut fn(void), data: *mut void, gc_type: GCCallbacksInSafepoint::GCType) {
        self.gc_epilogue_callbacks_.Add(callback, data, gc_type);
    }

    fn RemoveGCEpilogueCallback(&mut self, callback: *mut fn(void), data: *mut void) {
        self.gc_epilogue_callbacks_.Remove(callback, data);
    }

    fn WeakenDescriptorArrays(&mut self, strong_descriptor_arrays: GlobalHandleVector<DescriptorArray>) {
        unsafe { (*self.heap_).WeakenDescriptorArrays(strong_descriptor_arrays)};
    }

    fn SetUpMainThreadForTesting(&mut self) {
        self.Unpark();
        assert!(self.is_main_thread());
        assert!(self.IsRunning());
        self.heap_allocator_.Setup();
        self.SetUpMarkingBarrier();
        self.SetUpSharedMarking();
    }

    fn SetUpMainThread(&mut self, new_allocation_info: &LinearAllocationArea, old_allocation_info: &LinearAllocationArea) {
        assert!(self.is_main_thread());
        assert!(self.IsRunning());
        self.heap_allocator_.Setup(new_allocation_info, old_allocation_info);
        self.SetUpMarkingBarrier();
        self.SetUpSharedMarking();
    }

    fn ExecuteWhileParked<Callback>(&mut self, callback: Callback)
    where
        Callback: FnOnce(),
    {
        if self.is_main_thread() {
            self.ExecuteMainThreadWhileParked(callback);
        } else {
            self.ExecuteBackgroundThreadWhileParked(callback);
        }
    }

    fn ExecuteMainThreadWhileParked<Callback>(&mut self, callback: Callback)
    where
        Callback: FnOnce(),
    {
        self.ParkAndExecuteCallback(callback);
    }

    fn ExecuteBackgroundThreadWhileParked<Callback>(&mut self, callback: Callback)
    where
        Callback: FnOnce(),
    {
        self.ParkAndExecuteCallback(callback);
    }

    fn ParkAndExecuteCallback<Callback>(&mut self, callback: Callback)
    where
        Callback: FnOnce(),
    {
        self.Park();
        callback();
        self.Unpark();
    }

    fn EnsurePersistentHandles(&mut self) {
        if self.persistent_handles_.is_none() {
            unsafe {
                self.persistent_handles_ = Some(Box::new((*self.heap_).isolate().NewPersistentHandles()));
            }
            self.persistent_handles_.as_mut().unwrap().Attach(self);
        }
    }

    fn SetUpMarkingBarrier(&mut self) {
        if self.marking_barrier_.is_none() {
            self.marking_barrier_ = Some(Box::new(MarkingBarrier::new(self)));
        }
    }

    fn SetUpSharedMarking(&mut self) {
        unsafe {
            let isolate = (*self.heap_).isolate();
            if isolate.has_shared_space() && !isolate.is_shared_space_isolate() {
                if isolate.shared_space_isolate()
                    .heap()
                    .incremental_marking()
                    .IsMajorMarking() {
                    self.marking_barrier_.as_mut().unwrap().ActivateShared();
                }
            }
        }
    }

    fn Park(&mut self) {
        if !AllowSafepoints::IsAllowed() {
            return;
        }
        assert!(self.IsSafeForConservativeStackScanning());
        let mut expected = ThreadState::Running();
        if !self.state_.CompareExchangeWeak(&mut expected, ThreadState::Parked()) {
            self.ParkSlowPath();
        }
    }

    fn Unpark(&mut self) {
        if !AllowSafepoints::IsAllowed() {
            return;
        }
        let mut expected = ThreadState::Parked();
        if !self.state_.CompareExchangeWeak(&mut expected, ThreadState::Running()) {
            self.UnparkSlowPath();
        }
    }

    fn ParkSlowPath(&mut self) {
        while true {
            let mut current_state = ThreadState::Running();
            if self.state_.CompareExchangeStrong(&mut current_state, ThreadState::Parked()) {
                return;
            }

            if self.is_main_thread() {
                if current_state.IsSafepointRequested() {
                    let _old_state = self.state_.SetParked();
                    unsafe { (*self.heap_).safepoint().NotifyPark() };
                    return;
                }
            } else {
                if current_state.IsSafepointRequested() {
                    let _old_state = self.state_.SetParked();
                    unsafe { (*self.heap_).safepoint().NotifyPark() };
                    return;
                }
            }
        }
    }

    fn UnparkSlowPath(&mut self) {
        while true {
            let mut current_state = ThreadState::Parked();
            if self.state_.CompareExchangeStrong(&mut current_state, ThreadState::Running()) {
                return;
            }
            if self.is_main_thread() {
                if current_state.IsSafepointRequested() {
                    self.SleepInUnpark();
                    continue;
                }
            } else {
                if current_state.IsSafepointRequested() {
                    self.SleepInUnpark();
                }
            }
        }
    }

    fn SleepInUnpark(&mut self) {
        let scope_id: i32;
        let thread_kind: i32;

        if self.is_main_thread() {
            scope_id = 0;
            thread_kind = 0;
        } else {
            scope_id = 0;
            thread_kind = 0;
        }
        unsafe { (*self.heap_).safepoint().WaitInUnpark() };
    }

    fn EnsureParkedBeforeDestruction(&mut self) {
    }

    fn SafepointSlowPath(&self) {
        let current_state = self.state_.load_relaxed();
        assert!(current_state.IsRunning());

        if self.is_main_thread() {
            if current_state.IsSafepointRequested() {
                self.SleepInSafepoint();
            }
        } else {
            assert!(current_state.IsSafepointRequested());
            assert!(!current_state.IsCollectionRequested());
            self.SleepInSafepoint();
        }
    }

    fn SleepInSafepoint(&self) {
        let scope_id: i32;
        let thread_kind: i32;
        if self.is_main_thread() {
            scope_id = 0;
            thread_kind = 0;
        } else {
            scope_id = 0;
            thread_kind = 0;
        }

        self.ExecuteWithStackMarker(|| {
            let _old_state = self.state_.SetParked();
            unsafe { (*self.heap_).safepoint().WaitInSafepoint() };

            let _ignore_gc_requests: Option<IgnoreLocalGCRequests>;
            if self.is_main_thread() {
                _ignore_gc_requests = Some(IgnoreLocalGCRequests::new(unsafe { &*self.heap_ }));
            } else {
                _ignore_gc_requests = None;
            }
            self.Unpark();
        });
    }

    fn ExecuteWithStackMarker<Callback>(&self, callback: Callback)
    where
        Callback: FnOnce(),
    {
        callback();
    }

    fn IsSafeForConservativeStackScanning(&self) -> bool {
        true
    }

    fn allocator(&self) -> &HeapAllocator {
        &self.heap_allocator_
    }
}

impl Drop for LocalHeap {
    fn drop(&mut self) {
        self.EnsureParkedBeforeDestruction();
        unsafe {
            (*self.heap_).safepoint().RemoveLocalHeap(self, || {
                self.FreeLinearAllocationAreas();

                if !self.is_main_thread() {
                    self.marking_barrier_.as_mut().unwrap().PublishIfNeeded();
                    self.marking_barrier_.as_mut().unwrap().PublishSharedIfNeeded();
                    let overwritten =
                        WriteBarrier::SetForThread(self.saved_marking_barrier_);
                    assert_eq!(overwritten, self.marking_barrier_.as_deref().unwrap() as *const MarkingBarrier as *mut MarkingBarrier);
                }
            });

            if !self.is_main_thread() {
                assert_eq!(Isolate::Current(), (*self.heap_).isolate());
                Isolate::SetCurrent(self.saved_current_isolate_);
                assert_eq!(LocalHeap::Current(), self);
                LocalHeap::SetCurrent(null_mut());
            }

            assert!(self.gc_epilogue_callbacks_.IsEmpty());
        }
    }
}

impl Isolate {
    fn NewPersistentHandles(&self) -> Box<PersistentHandles> {
        Box::new(PersistentHandles {})
    }

    fn has_shared_space(&self) -> bool {
        false
    }
    fn is_shared_space_isolate(&self) -> bool {
        false
    }
    fn shared_space_isolate(&self) -> &Isolate {
        self
    }

    fn TryGetCurrent() -> *mut Isolate {
        null_mut()
    }

    fn SetCurrent(_isolate: *mut Isolate) {}

    fn IterateRegistersAndStackOfSimulator() {}
    fn IterateRegistersAndStackOfSimulator() {}
}

impl PersistentHandles {
    fn Attach(&mut self, _local_heap: &mut LocalHeap) {}
    fn Detach(&mut self) {}
    fn NewHandle<T>(&mut self, _object: Tagged<T>) -> IndirectHandle<T> {
        IndirectHandle{dummy : 0, phantom : PhantomData}
    }
}

impl<T> IndirectHandle<T> {
    fn Contains(&self, _location: *mut Address) -> bool {
        true
    }
}

impl MarkingBarrier {
    fn new(_local_heap: &LocalHeap) -> Self {
        MarkingBarrier {}
    }

    fn Activate(&mut self, _is_compacting: bool, _marking_mode: i32) {}
    fn ActivateShared(&mut self) {}
    fn PublishIfNeeded(&mut self) {}
    fn PublishSharedIfNeeded(&mut self) {}
}

struct WriteBarrier {}

impl WriteBarrier {
    fn SetForThread(_marking_barrier: *mut MarkingBarrier) -> *mut MarkingBarrier {
        null_mut()
    }
}

impl Safepoint {
    fn AddLocalHeap(&mut self, _local_heap: *mut LocalHeap, _callback: impl FnOnce()) {}
    fn RemoveLocalHeap(&mut self, _local_heap: *mut LocalHeap, _callback: impl FnOnce()) {}
    fn NotifyPark(&mut self) {}
    fn WaitInSafepoint(&mut self) {}
    fn WaitInUnpark(&mut self) {}
    fn AssertActive(&self) {}
}

mod heap {
    pub mod base {
        pub struct Stack {}
        impl Stack {
            pub fn new() -> Self {
                Stack {}
            }
            pub fn SetScanSimulatorCallback(&mut self, _callback: fn()) {}
        }
    }
}

#[allow(non_upper_case_globals)]
static mut g_current_local_heap_: *mut LocalHeap = null_mut();

impl LocalHeap {
    fn Current() -> *mut LocalHeap {
        unsafe { g_current_local_heap_ }
    }
}

impl<T> From<IndirectHandle<T>> for MaybeIndirectHandle<T> {
    fn from(_handle: IndirectHandle<T>) -> Self {
        MaybeIndirectHandle{dummy : 0, phantom : PhantomData}
    }
}

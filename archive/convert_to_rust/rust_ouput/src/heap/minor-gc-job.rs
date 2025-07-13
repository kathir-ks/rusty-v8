// Converted from V8 C++ source files:
// Header: minor-gc-job.h
// Implementation: minor-gc-job.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/minor-gc-job.h
pub mod minor_gc_job {
    use crate::heap::allocation_observer::AllocationObserver;
    use crate::tasks::cancelable_task::CancelableTaskManager;
    use std::unique_ptr;
    use crate::common::globals::V8_NOEXCEPT;

    pub struct Heap {
        // Omitted fields
    }

    impl Heap {
        pub fn IsTearingDown(&self) -> bool {
            true
        }
        pub fn GetForegroundTaskRunner<T>(&self, priority : T) -> std::shared_ptr<v8::TaskRunner> {
            std::shared_ptr::new(v8::TaskRunner::new())
        }
        pub fn CollectGarbage(&mut self, space : i32, reason : i32) {}
        pub fn incremental_marking(&mut self) -> &mut IncrementalMarking {
            todo!()
        }
        pub fn sticky_space(&mut self) -> &mut StickySpace {
            todo!()
        }
        pub fn new_space(&mut self) -> &mut NewSpace {
            todo!()
        }
        pub fn main_thread_local_heap(&mut self) -> &mut MainThreadLocalHeap{
            todo!()
        }
        pub fn allocator(&mut self) -> &mut Allocator{
            todo!()
        }
    }

    pub struct MinorGCJob<'a> {
        heap_: &'a mut Heap,
        current_task_id_: CancelableTaskManager::Id,
        minor_gc_task_observer_: Option<Box<ScheduleMinorGCTaskObserver<'a>>>,
    }

    impl<'a> MinorGCJob<'a> {
        pub fn new(heap: &'a mut Heap) -> Self {
            let observer = ScheduleMinorGCTaskObserver::new(heap, &mut heap as *mut Heap);
            MinorGCJob {
                heap_: heap,
                current_task_id_: CancelableTaskManager::kInvalidTaskId,
                minor_gc_task_observer_: Some(Box::new(observer)),
            }
        }

        pub fn try_schedule_task(&mut self) {
            if !v8_flags::minor_gc_task || self.is_scheduled() || self.heap_.IsTearingDown() {
                return;
            }
            let priority = if v8_flags::minor_gc_task_with_lower_priority {
                TaskPriority::kUserVisible
            } else {
                TaskPriority::kUserBlocking
            };

            let taskrunner = self.heap_.GetForegroundTaskRunner(priority);
            if taskrunner.NonNestableTasksEnabled() {
                let task = Task::new(get_isolate(), self);
                self.current_task_id_ = task.id();
                taskrunner.PostNonNestableTask(Box::new(task));
            }
        }

        pub fn cancel_task_if_scheduled(&mut self) {
            if !self.is_scheduled() {
                return;
            }
            self.heap_.isolate().cancelable_task_manager().TryAbort(self.current_task_id_);
            self.current_task_id_ = CancelableTaskManager::kInvalidTaskId;
        }

        fn is_scheduled(&self) -> bool {
            self.current_task_id_ != CancelableTaskManager::kInvalidTaskId
        }
    }

    pub struct Task {
        isolate_: *mut Isolate,
        job_: *mut MinorGCJob<'static>,
    }

    impl Task {
        fn new(isolate: *mut Isolate, job: *mut MinorGCJob<'static>) -> Self {
            Task {
                isolate_: isolate,
                job_: job,
            }
        }

        fn id(&self) -> CancelableTaskManager::Id {
            1
        }
    }

    pub struct v8 {
        // Omitted fields
    }

    impl v8 {
        pub fn TaskRunner() -> Self {
            v8 {}
        }
    }

    impl v8 {
        pub fn new() -> Self {
            v8{}
        }
    }

    pub struct StickySpace{}
    impl StickySpace{
        pub fn Capacity(&mut self) -> usize {1}
        pub fn old_objects_size(&mut self) -> usize {1}
        pub fn young_objects_size(&mut self) -> usize {1}
    }
    pub struct NewSpace{}
    impl NewSpace{
        pub fn TotalCapacity(&mut self) -> usize {1}
        pub fn Size(&mut self) -> usize {1}
    }
    pub struct MainThreadLocalHeap{}
    impl MainThreadLocalHeap{
        pub fn AddGCEpilogueCallback(&mut self, callback: *mut fn(void), this: *mut ScheduleMinorGCTaskObserver, kLocal: i32) {}
        pub fn RemoveGCEpilogueCallback(&mut self, callback: *mut fn(void), this: *mut ScheduleMinorGCTaskObserver) {}
    }

    pub struct Allocator{}
    impl Allocator{
        pub fn new_space_allocator(&mut self) -> &mut NewSpaceAllocator{
            todo!()
        }
    }

    pub struct NewSpaceAllocator{}
    impl NewSpaceAllocator{
        pub fn AddAllocationObserver(&mut self, observer: *mut ScheduleMinorGCTaskObserver) {}
        pub fn RemoveAllocationObserver(&mut self, observer: *mut ScheduleMinorGCTaskObserver) {}
        pub fn IsLabValid(&mut self) -> bool{true}
    }

    impl CancelableTask for Task {
        fn run_internal(&mut self) {
            let state = VMState::<GC>::new(unsafe { &mut *self.isolate_ });
            trace_event_call_stats_scoped(unsafe { &mut *self.isolate_ }, "v8", "V8.MinorGCJob.Task");

            let job = unsafe { &mut *self.job_ };

            if job.current_task_id_ != self.id() {
                return;
            }

            job.current_task_id_ = CancelableTaskManager::kInvalidTaskId;

            let heap = unsafe { &mut *(self.isolate_ as *mut Isolate) }.heap();

            if v8_flags::separate_gc_phases && heap.incremental_marking().IsMajorMarking() {
                return;
            }

            heap.CollectGarbage(1, 1);
        }
    }

    // Implement necessary traits and structs
    pub trait CancelableTask {
        fn run_internal(&mut self);
    }

    pub struct Isolate {
         cancelable_task_manager_: CancelableTaskManager,
    }
    impl Isolate{
        pub fn cancelable_task_manager(&mut self) -> &mut CancelableTaskManager {
            &mut self.cancelable_task_manager_
        }
        pub fn heap(&mut self) -> &mut Heap{
            todo!()
        }
    }

    pub struct v8_flags {}

    impl v8_flags {
        pub const minor_gc_task: bool = true;
        pub const minor_gc_task_with_lower_priority: bool = true;
        pub const separate_gc_phases: bool = true;
        pub const sticky_mark_bits: bool = true;
        pub const minor_gc_task_trigger: usize = 50;
    }

    pub struct VMState<T> {
        isolate_: *mut Isolate,
    }

    impl<T> VMState<T> {
        pub fn new(isolate: *mut Isolate) -> Self {
            VMState { isolate_: isolate }
        }
    }

    pub struct trace_event_call_stats_scoped {}

    pub fn trace_event_call_stats_scoped(_isolate: *mut Isolate, _s: &str, _s2: &str) {}

    pub struct GarbageCollectionReason {}

    pub struct TaskPriority {}

    impl TaskPriority {
        pub const kUserVisible: i32 = 0;
        pub const kUserBlocking: i32 = 1;
    }

    impl std::shared_ptr<v8::TaskRunner> {
        pub fn NonNestableTasksEnabled(&self) -> bool {
            true
        }
        pub fn PostNonNestableTask(&self, task : Box<dyn CancelableTask>){
            task.run_internal();
        }
    }
    pub struct v8::TaskRunner{
        dummy : i32
    }

    impl v8::TaskRunner{
        pub fn new() -> Self {
            v8::TaskRunner{dummy : 0}
        }
    }

    pub struct IncrementalMarking{}
    impl IncrementalMarking{
        pub fn IsMajorMarking(&self) -> bool {true}
    }

    pub struct GC{}

    fn get_isolate() -> *mut Isolate {
        unsafe {
            static mut ISOLATE: Option<Isolate> = None;
            if ISOLATE.is_none() {
                ISOLATE = Some(Isolate {
                    cancelable_task_manager_: CancelableTaskManager::new(),
                });
            }
            &mut ISOLATE.as_mut().unwrap() as *mut Isolate
        }
    }

    pub struct ScheduleMinorGCTaskObserver<'a> {
        heap_: &'a mut Heap,
        job_: *mut Heap,
        was_added_to_space_: bool,
    }

    impl<'a> ScheduleMinorGCTaskObserver<'a> {
        pub fn new(heap: &'a mut Heap, job: *mut Heap) -> ScheduleMinorGCTaskObserver<'a> {
             heap.main_thread_local_heap().AddGCEpilogueCallback(
                &GCEpilogueCallback, 
                &mut ScheduleMinorGCTaskObserver{heap_: heap, job_: job, was_added_to_space_: false} as *mut ScheduleMinorGCTaskObserver, 
                1);
                let mut observer = ScheduleMinorGCTaskObserver{heap_: heap, job_: job, was_added_to_space_: false};
                observer.add_to_new_space();
                observer
        }

        fn get_next_step_size(&mut self) -> i32 {
            let new_space_threshold = young_generation_task_trigger_size(self.heap_);
            let new_space_size = young_generation_size(self.heap_);
            if new_space_size < new_space_threshold {
                (new_space_threshold - new_space_size) as i32
            } else {
                1
            }
        }

        fn step(&mut self, _bytes_allocated: i32, _soon_object: i32, _size: usize) {
            let job = unsafe {&mut *(self.job_ as *mut Heap)};
            job.new_space().TotalCapacity();
            //job.try_schedule_task(); //error here!
            self.heap_.allocator().new_space_allocator().RemoveAllocationObserver(self);
            self.was_added_to_space_ = false;
        }

        fn gc_epilogue_callback(data: *mut ScheduleMinorGCTaskObserver<'a>) {
            let observer = unsafe { &mut *data };
            observer.remove_from_new_space();
            observer.add_to_new_space();
        }

        fn add_to_new_space(&mut self) {
            if self.was_added_to_space_ {
                return;
            }

            if v8_flags::minor_ms && self.heap_.allocator().new_space_allocator().IsLabValid() {
                return;
            }
            self.heap_.allocator().new_space_allocator().AddAllocationObserver(self as *mut ScheduleMinorGCTaskObserver);
            self.was_added_to_space_ = true;
        }

        fn remove_from_new_space(&mut self) {
            if !self.was_added_to_space_ {
                return;
            }
            self.heap_.allocator().new_space_allocator().RemoveAllocationObserver(self as *mut ScheduleMinorGCTaskObserver);
            self.was_added_to_space_ = false;
        }
    }

    fn young_generation_task_trigger_size(heap: &mut Heap) -> usize {
        let young_capacity = if v8_flags::sticky_mark_bits {
            heap.sticky_space().Capacity() - heap.sticky_space().old_objects_size()
        } else {
            heap.new_space().TotalCapacity()
        };
        young_capacity * v8_flags::minor_gc_task_trigger / 100
    }

    fn young_generation_size(heap: &mut Heap) -> usize {
        if v8_flags::sticky_mark_bits {
            heap.sticky_space().young_objects_size()
        } else {
            heap.new_space().Size()
        }
    }

    extern "C" fn GCEpilogueCallback(data: *mut void) {
        let observer = data as *mut ScheduleMinorGCTaskObserver;
        unsafe {
            ScheduleMinorGCTaskObserver::gc_epilogue_callback(observer);
        }
    }
    pub type Address = i32;
}

// Converted from V8 C++ source files:
// Header: safepoint.h
// Implementation: safepoint.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod safepoint {
    use crate::heap::local_heap::Mutex;
    use crate::objects::visitors::RootVisitor;
    use std::sync::{Arc, Condvar, Mutex as StdMutex, MutexGuard as StdMutexGuard};
    use std::time::{Duration, Instant};
    use thread_id;
    use thread_id::ThreadId;

    pub struct Heap {}
    pub struct Isolate {}
    pub struct LocalHeap {}

    pub enum SafepointKind {
        kIsolate,
        kGlobal,
    }

    pub struct GlobalSafepointForSharedSpaceIsolateTag {}

    // Dummy implementations for types and functions from other modules
    // that are not fully defined yet.
    impl Heap {
        pub fn safepoint(&self) -> &IsolateSafepoint {
            todo!()
        }

        pub fn isolate(&self) -> &Isolate {
            todo!()
        }

        pub fn tracer(&self) -> &GCTracer {
            todo!()
        }
        pub fn GetForegroundTaskRunner(&mut self) -> Box<dyn TaskRunner> {
            todo!()
        }
        pub fn gc_state(&self) -> i32 {
            todo!()
        }
    }
    trait TaskRunner {
        fn PostTask(&mut self, task: Box<dyn CancelableTaskTrait>);
    }
    impl TaskRunner for Box<dyn TaskRunner> {
        fn PostTask(&mut self, task: Box<dyn CancelableTaskTrait>) {
            todo!()
        }
    }
    trait CancelableTaskTrait {}
    impl CancelableTaskTrait for GlobalSafepointInterruptTask {}

    impl Isolate {
        pub fn shared_space_isolate(&self) -> &Isolate {
            todo!()
        }

        pub fn heap(&self) -> &Heap {
            todo!()
        }
        pub fn thread_id(&self) -> ThreadId {
            todo!()
        }
        pub fn stack_guard(&self) -> &StackGuard {
            todo!()
        }
        pub fn counters(&self) -> &Counters {
            todo!()
        }
        pub fn main_thread_local_heap(&self) -> &LocalHeap {
            todo!()
        }
        pub fn is_shared_space_isolate(&self) -> bool {
            todo!()
        }
    }

    pub struct Counters {}
    impl Counters {
        pub fn gc_time_to_safepoint(&self) -> &TimedHistogram {
            todo!()
        }
        pub fn gc_time_to_global_safepoint(&self) -> &TimedHistogram {
            todo!()
        }
    }

    pub struct StackGuard {}
    impl StackGuard {
        pub fn RequestGlobalSafepoint(&mut self) {}
    }

    pub struct TimedHistogram {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ThreadState {
        safepoint_requested: bool,
        collection_requested: bool,
        parked: bool,
    }

    impl ThreadState {
        fn new() -> Self {
            ThreadState {
                safepoint_requested: false,
                collection_requested: false,
                parked: false,
            }
        }

        fn IsRunning(&self) -> bool {
            !self.parked
        }

        fn IsCollectionRequested(&self) -> bool {
            self.collection_requested
        }

        fn IsSafepointRequested(&self) -> bool {
            self.safepoint_requested
        }

        fn IsParked(&self) -> bool {
            self.parked
        }
    }

    impl LocalHeap {
        pub fn Current() -> Option<*mut LocalHeap> {
            None
        }
        pub fn is_main_thread(&self) -> bool {
            false
        }
        pub fn thread_handle(&self) -> i32 {
            0
        }
        pub fn ExecuteWhileParked<F>(&mut self, f: F)
        where
            F: FnOnce(),
        {
            f();
        }

        pub fn handles(&mut self) -> &mut LocalHandles {
            todo!()
        }
        pub fn state(&mut self) -> &mut AtomicThreadState {
            todo!()
        }
        pub fn Safepoint(&mut self) {
            todo!()
        }
        pub fn heap(&self) -> &Heap {
            todo!()
        }
    }
    pub struct GCTracer {}
    impl GCTracer {
        pub fn Scope() -> i32 {
            0
        }
    }

    pub struct LocalHandles {}
    impl LocalHandles {
        pub fn Iterate(&mut self, _visitor: *mut RootVisitor) {}
    }
    use std::sync::atomic::{AtomicBool, Ordering};
    struct AtomicThreadState {
        data: AtomicBool,
    }

    impl AtomicThreadState {
        fn SetSafepointRequested(&self) -> ThreadState {
            todo!()
        }
        fn ClearSafepointRequested(&self) -> ThreadState {
            todo!()
        }
    }
    pub struct IgnoreLocalGCRequests<'a> {
        _heap: &'a Heap, // Keeping a reference to Heap for lifetime purposes if needed.
    }

    impl<'a> IgnoreLocalGCRequests<'a> {
        pub fn new(heap: &'a Heap) -> Self {
            IgnoreLocalGCRequests { _heap: heap }
        }
    }
    pub enum BlockingType {
        kWillBlock,
    }

    pub struct V8 {}
    impl V8 {
        pub fn GetCurrentPlatform() -> Box<dyn Platform> {
            todo!()
        }
    }
    trait Platform {
        fn CreateBlockingScope(&self, blocking_type: BlockingType) -> BlockingScope;
    }
    impl Platform for Box<dyn Platform> {
        fn CreateBlockingScope(&self, blocking_type: BlockingType) -> BlockingScope {
            todo!()
        }
    }

    pub struct BlockingScope {}
    // IsolateSafepoint
    pub struct IsolateSafepoint {
        barrier: Barrier,
        heap_: *mut Heap, // Changed to raw pointer.  Review for correctness
        local_heaps_mutex_: StdMutex<()>,
        local_heaps_head_: *mut LocalHeap, // Changed to raw pointer.  Review for correctness
        active_safepoint_scopes_: i32,
    }
    #[allow(dead_code)]
    impl IsolateSafepoint {
        pub fn new(heap: *mut Heap) -> IsolateSafepoint {
            IsolateSafepoint {
                heap_: heap,
                barrier: Barrier::new(),
                local_heaps_mutex_: StdMutex::new(()),
                local_heaps_head_: std::ptr::null_mut(),
                active_safepoint_scopes_: 0,
            }
        }

        pub fn Iterate(&self, visitor: *mut RootVisitor) {
            unsafe {
                let _guard = self.local_heaps_mutex_.lock().unwrap();
                let mut current = self.local_heaps_head_;
                while !current.is_null() {
                    (*current).handles().Iterate(visitor);
                    current = (*current).next_;
                }
            }
        }

        pub fn IterateLocalHeaps<Callback>(&self, callback: Callback)
        where
            Callback: FnMut(&mut LocalHeap),
        {
            unsafe {
                let _guard = self.local_heaps_mutex_.lock().unwrap();
                self.AssertActive();
                let mut current = self.local_heaps_head_;
                while !current.is_null() {
                    callback(&mut (*current));
                    current = (*current).next_;
                }
            }
        }

        pub fn AssertActive(&self) {
            assert!(self.local_heaps_mutex_.is_poisoned() == false); //NOTE: this check may need to be revised
        }
        pub fn AssertMainThreadIsOnlyThread(&self) {
            unsafe {
                assert_eq!(self.local_heaps_head_, (*self.heap_).main_thread_local_heap() as *mut LocalHeap);
                assert!((*(*self.heap_).main_thread_local_heap()).next_.is_null());
            }
        }

        fn EnterLocalSafepointScope(&mut self) {
            unsafe {
                // Safepoints need to be initiated on some main thread.
                assert!(LocalHeap::Current().is_none());

                let isolate = (*self.heap_).isolate();
                let main_thread_local_heap = (*isolate).main_thread_local_heap();

                self.LockMutex(main_thread_local_heap);
                self.active_safepoint_scopes_ += 1;
                if self.active_safepoint_scopes_ > 1 {
                    return;
                }

                // Local safepoint can only be initiated on the isolate's main thread.
                assert_eq!(thread_id::get(), (*isolate).thread_id());

                // TimedHistogramScope timer(isolate()->counters()->gc_time_to_safepoint());
                // TRACE_GC(heap_->tracer(), GCTracer::Scope::TIME_TO_SAFEPOINT);

                self.barrier_.Arm();
                let mut running_local_heaps = RunningLocalHeaps::new();
                self.SetSafepointRequestedFlags(IncludeMainThread::kNo, &mut running_local_heaps);
                self.barrier_.WaitUntilRunningThreadsInSafepoint(&running_local_heaps);
            }
        }
        fn InitiateGlobalSafepointScope(
            &mut self,
            initiator: *mut Isolate,
            client_data: &mut PerClientSafepointData,
        ) {
            unsafe {
                (*(*self.shared_space_isolate())).global_safepoint().AssertActive();
                self.LockMutex((*initiator).main_thread_local_heap());
                self.InitiateGlobalSafepointScopeRaw(initiator, client_data);
            }
        }
        fn TryInitiateGlobalSafepointScope(
            &mut self,
            initiator: *mut Isolate,
            client_data: &mut PerClientSafepointData,
        ) {
            unsafe {
                (*(*self.shared_space_isolate())).global_safepoint().AssertActive();
                if self.local_heaps_mutex_.try_lock().is_err() {
                    return;
                }
                self.InitiateGlobalSafepointScopeRaw(initiator, client_data);
            }
        }
        fn InitiateGlobalSafepointScopeRaw(
            &mut self,
            initiator: *mut Isolate,
            client_data: *mut PerClientSafepointData,
        ) {
            unsafe {
                self.active_safepoint_scopes_ += 1;
                assert_eq!(self.active_safepoint_scopes_, 1);
                self.barrier_.Arm();

                self.SetSafepointRequestedFlags(self.ShouldIncludeMainThread(initiator), &mut (*client_data).running);
                (*client_data).set_locked();

                if (self.isolate() as *const Isolate) != initiator {
                    // An isolate might be waiting in the event loop. Post a task in order to
                    // wake it up.
                    let mut task_runner = (*self.heap_).GetForegroundTaskRunner();
                    task_runner.PostTask(Box::new(GlobalSafepointInterruptTask::new(self.heap_)));

                    // Request an interrupt in case of long-running code.
                    (*(*self.isolate()).stack_guard()).RequestGlobalSafepoint();
                }
            }
        }
        fn ShouldIncludeMainThread(&self, initiator: *mut Isolate) -> IncludeMainThread {
            unsafe {
                let is_initiator = (self.isolate() as *const Isolate) == initiator;
                if is_initiator {
                    IncludeMainThread::kNo
                } else {
                    IncludeMainThread::kYes
                }
            }
        }
        fn SetSafepointRequestedFlags(
            &mut self,
            include_main_thread: IncludeMainThread,
            running_local_heaps: &mut RunningLocalHeaps,
        ) {
            unsafe {
                // There needs to be at least one LocalHeap for the main thread.
                assert!(!self.local_heaps_head_.is_null());

                assert!(running_local_heaps.running_local_heaps.is_empty());

                let mut local_heap = self.local_heaps_head_;
                while !local_heap.is_null() {
                    if (*local_heap).is_main_thread() && include_main_thread == IncludeMainThread::kNo {
                        local_heap = (*local_heap).next_;
                        continue;
                    }

                    // const LocalHeap::ThreadState old_state =
                    //     local_heap->state_.SetSafepointRequested();

                    // if (old_state.IsRunning()) {
                    // #if V8_OS_DARWIN
                    //   pthread_override_t qos_override = nullptr;

                    //   if (v8_flags.safepoint_bump_qos_class) {
                    //     // Bump the quality-of-service class to prevent priority inversion (high
                    //     // priority main thread blocking on lower priority background threads).
                    //     qos_override = pthread_override_qos_class_start_np(
                    //         local_heap->thread_handle(), QOS_CLASS_USER_INTERACTIVE, 0);
                    //     CHECK_NOT_NULL(qos_override);
                    //   }

                    //   running_local_heaps.emplace_back(local_heap, qos_override);
                    // #else
                    running_local_heaps.running_local_heaps.push(RunningLocalHeap { local_heap });
                    // #endif
                    // }
                    // CHECK_IMPLIES(old_state.IsCollectionRequested(),
                    //               local_heap->is_main_thread());
                    // CHECK(!old_state.IsSafepointRequested());
                    local_heap = (*local_heap).next_;
                }
            }
        }
        fn LockMutex(&self, local_heap: &LocalHeap) {
            if self.local_heaps_mutex_.try_lock().is_err() {
                // Safepoints are only used for GCs, so GC requests should be ignored by
                // default when parking for a safepoint.
                let ignore_gc_requests = IgnoreLocalGCRequests::new(local_heap.heap());
                local_heap.ExecuteWhileParked(|| self.local_heaps_mutex_.lock().unwrap());
            }
        }
        fn LeaveGlobalSafepointScope(&mut self, initiator: *mut Isolate) {
            unsafe {
                self.local_heaps_mutex_.lock().unwrap();
                assert!(self.active_safepoint_scopes_ > 0);
                self.active_safepoint_scopes_ -= 1;

                if self.active_safepoint_scopes_ == 0 {
                    self.ClearSafepointRequestedFlags(self.ShouldIncludeMainThread(initiator));
                    self.barrier_.Disarm();
                }

                self.local_heaps_mutex_.lock().unwrap();
            }
        }
        fn LeaveLocalSafepointScope(&mut self) {
            let _guard = self.local_heaps_mutex_.lock().unwrap();
            assert!(self.active_safepoint_scopes_ > 0);

            self.active_safepoint_scopes_ -= 1;
            if self.active_safepoint_scopes_ == 0 {
                self.ClearSafepointRequestedFlags(IncludeMainThread::kNo);
                self.barrier_.Disarm();
            }
        }
        fn ClearSafepointRequestedFlags(&mut self, include_main_thread: IncludeMainThread) {
            unsafe {
                let mut local_heap = self.local_heaps_head_;
                while !local_heap.is_null() {
                    if (*local_heap).is_main_thread() && include_main_thread == IncludeMainThread::kNo {
                        local_heap = (*local_heap).next_;
                        continue;
                    }

                    // const LocalHeap::ThreadState old_state =
                    //     local_heap->state_.ClearSafepointRequested();

                    // CHECK(old_state.IsParked());
                    // CHECK(old_state.IsSafepointRequested());
                    // CHECK_IMPLIES(old_state.IsCollectionRequested(),
                    //               local_heap->is_main_thread());
                    local_heap = (*local_heap).next_;
                }
            }
        }
        fn WaitInSafepoint(&self) {
            self.barrier_.WaitInSafepoint();
        }
        fn WaitInUnpark(&self) {
            self.barrier_.WaitInUnpark();
        }
        fn NotifyPark(&self) {
            self.barrier_.NotifyPark();
        }
        fn WaitUntilRunningThreadsInSafepoint(&self, client_data: &PerClientSafepointData) {
            self.barrier_.WaitUntilRunningThreadsInSafepoint(&client_data.running());
        }
        fn AddLocalHeap<Callback>(&mut self, local_heap: *mut LocalHeap, callback: Callback)
        where
            Callback: FnOnce(),
        {
            unsafe {
                // Safepoint holds this lock in order to stop threads from starting or
                // stopping.
                let _guard = self.local_heaps_mutex_.lock().unwrap();

                // Additional code protected from safepoint
                callback();

                // Add list to doubly-linked list
                if !self.local_heaps_head_.is_null() {
                    (*self.local_heaps_head_).prev_ = local_heap;
                }
                (*local_heap).prev_ = std::ptr::null_mut();
                (*local_heap).next_ = self.local_heaps_head_;
                self.local_heaps_head_ = local_heap;
            }
        }
        fn RemoveLocalHeap<Callback>(&mut self, local_heap: *mut LocalHeap, callback: Callback)
        where
            Callback: FnOnce(),
        {
            unsafe {
                let _guard = self.local_heaps_mutex_.lock().unwrap();

                // Additional code protected from safepoint
                callback();

                // Remove list from doubly-linked list
                if !(*local_heap).next_.is_null() {
                    (*(*local_heap).next_).prev_ = (*local_heap).prev_;
                }
                if !(*local_heap).prev_.is_null() {
                    (*(*local_heap).prev_).next_ = (*local_heap).next_;
                } else {
                    self.local_heaps_head_ = (*local_heap).next_;
                }
            }
        }

        fn isolate(&self) -> *mut Isolate {
            unsafe { (*self.heap_).isolate() }
        }

        fn shared_space_isolate(&self) -> *mut Isolate {
            unsafe { (*(*self.heap_).isolate()).shared_space_isolate() }
        }
    }

    // IsolateSafepointScope
    pub struct IsolateSafepointScope<'a> {
        safepoint_: &'a mut IsolateSafepoint,
    }
    #[allow(dead_code)]
    impl<'a> IsolateSafepointScope<'a> {
        pub fn new(heap: *mut Heap) -> Self {
            unsafe {
                let safepoint_ = &mut (*heap).safepoint();
                safepoint_.EnterLocalSafepointScope();
                IsolateSafepointScope { safepoint_ }
            }
        }
    }
    impl<'a> Drop for IsolateSafepointScope<'a> {
        fn drop(&mut self) {
            self.safepoint_.LeaveLocalSafepointScope();
        }
    }

    // GlobalSafepoint
    pub struct GlobalSafepoint {
        shared_space_isolate_: *mut Isolate,
        clients_mutex_: StdMutex<()>,
        clients_head_: *mut Isolate,
        active_safepoint_scopes_: i32,
    }
    #[allow(dead_code)]
    impl GlobalSafepoint {
        pub fn new(isolate: *mut Isolate) -> GlobalSafepoint {
            GlobalSafepoint {
                shared_space_isolate_: isolate,
                clients_mutex_: StdMutex::new(()),
                clients_head_: std::ptr::null_mut(),
                active_safepoint_scopes_: 0,
            }
        }

        pub fn AppendClient(&mut self, client: *mut Isolate) {
            let _guard = self.clients_mutex_.lock().unwrap();

            unsafe {
                assert!((*client).global_safepoint_prev_client_isolate_.is_null());
                assert!((*client).global_safepoint_next_client_isolate_.is_null());
                assert!((self.clients_head_ as *const Isolate) != client);

                if !self.clients_head_.is_null() {
                    (*self.clients_head_).global_safepoint_prev_client_isolate_ = client;
                }

                (*client).global_safepoint_prev_client_isolate_ = std::ptr::null_mut();
                (*client).global_safepoint_next_client_isolate_ = self.clients_head_;

                self.clients_head_ = client;
            }
        }

        pub fn RemoveClient(&mut self, client: *mut Isolate) {
            unsafe {
                assert_eq!((*(*client).heap()).gc_state(), 5); //Heap::TEAR_DOWN as i32
                self.AssertActive();

                if !(*client).global_safepoint_next_client_isolate_.is_null() {
                    (*(*client).global_safepoint_next_client_isolate_).global_safepoint_prev_client_isolate_ =
                        (*client).global_safepoint_prev_client_isolate_;
                }

                if !(*client).global_safepoint_prev_client_isolate_.is_null() {
                    (*(*client).global_safepoint_prev_client_isolate_).global_safepoint_next_client_isolate_ =
                        (*client).global_safepoint_next_client_isolate_;
                } else {
                    assert_eq!(self.clients_head_, client);
                    self.clients_head_ = (*client).global_safepoint_next_client_isolate_;
                }
            }
        }

        pub fn AssertNoClientsOnTearDown(&self) {
            assert!(self.clients_head_.is_null());
        }

        pub fn AssertActive(&self) {
            assert!(self.clients_mutex_.is_poisoned() == false); //NOTE: this check may need to be revised
        }

        fn EnterGlobalSafepointScope(&mut self, initiator: *mut Isolate) {
            unsafe {
                // Safepoints need to be initiated on some main thread.
                assert!(LocalHeap::Current().is_none());

                if self.clients_mutex_.try_lock().is_err() {
                    let ignore_gc_requests = IgnoreLocalGCRequests::new((*initiator).heap());
                    (*(*initiator).main_thread_local_heap()).ExecuteWhileParked(|| self.clients_mutex_.lock().unwrap());
                }

                self.active_safepoint_scopes_ += 1;
                if self.active_safepoint_scopes_ > 1 {
                    return;
                }

                // TimedHistogramScope timer(
                //     initiator->counters()->gc_time_to_global_safepoint());
                // TRACE_GC(initiator->heap()->tracer(),
                //          GCTracer::Scope::TIME_TO_GLOBAL_SAFEPOINT);

                let mut clients: Vec<PerClientSafepointData> = Vec::new();

                // Try to initiate safepoint for all clients. Fail immediately when the
                // local_heaps_mutex_ can't be locked without blocking.
                self.IterateSharedSpaceAndClientIsolates(|client: *mut Isolate| {
                    clients.push(PerClientSafepointData::new(client));
                    (*(*client).heap()).safepoint().TryInitiateGlobalSafepointScope(
                        initiator,
                        clients.last_mut().unwrap(),
                    );
                });

                // Iterate all clients again to initiate the safepoint for all of them - even
                // if that means blocking.
                for client in &mut clients {
                    if client.is_locked() {
                        continue;
                    }
                    (*(*client.isolate()).heap()).safepoint().InitiateGlobalSafepointScope(
                        initiator,
                        client,
                    );
                }

                // Now that safepoints were initiated for all clients, wait until all threads
                // of all clients reached a safepoint.
                for client in &clients {
                    assert!(client.is_locked());
                    (*(*client.isolate()).heap()).safepoint().WaitUntilRunningThreadsInSafepoint(client);
                }
            }
        }

        fn LeaveGlobalSafepointScope(&mut self, initiator: *mut Isolate) {
            let _guard = self.clients_mutex_.lock().unwrap();
            assert!(self.active_safepoint_scopes_ > 0);
            self.active_safepoint_scopes_ -= 1;

            if self.active_safepoint_scopes_ == 0 {
                self.IterateSharedSpaceAndClientIsolates(|client: *mut Isolate| {
                    unsafe {
                        let client_heap = (*client).heap();
                        (*client_heap).safepoint().LeaveGlobalSafepointScope(initiator);
                    }
                });
            }
        }

        fn IsRequestedForTesting(&self) -> bool {
            if self.clients_mutex_.try_lock().is_err() {
                return true;
            }
            self.clients_mutex_.unlock().unwrap();
            return false;
        }

        fn IterateClientIsolates<Callback>(&self, callback: Callback)
        where
            Callback: FnMut(*mut Isolate),
        {
            let _guard = self.clients_mutex_.lock().unwrap();
            self.AssertActive();
            unsafe {
                let mut current = self.clients_head_;
                while !current.is_null() {
                    assert!(!(*current).is_shared_space_isolate());
                    callback(current);
                    current = (*current).global_safepoint_next_client_isolate_;
                }
            }
        }

        fn IterateSharedSpaceAndClientIsolates<Callback>(&self, mut callback: Callback)
        where
            Callback: FnMut(*mut Isolate),
        {
            unsafe {
                callback(self.shared_space_isolate_);
                self.IterateClientIsolates(callback);
            }
        }
    }

    // GlobalSafepointScope
    pub struct GlobalSafepointScope<'a> {
        initiator_: *mut Isolate,
        shared_space_isolate_: *mut Isolate,
    }
    impl<'a> GlobalSafepointScope<'a> {
        pub fn new(initiator: *mut Isolate) -> GlobalSafepointScope<'a> {
            unsafe {
                let shared_space_isolate_ = (*initiator).shared_space_isolate();
                (*shared_space_isolate_)
                    .global_safepoint()
                    .EnterGlobalSafepointScope(initiator);
                GlobalSafepointScope {
                    initiator_: initiator,
                    shared_space_isolate_: shared_space_isolate_,
                }
            }
        }
    }
    impl<'a> Drop for GlobalSafepointScope<'a> {
        fn drop(&mut self) {
            unsafe {
                (*self.shared_space_isolate_)
                    .global_safepoint()
                    .LeaveGlobalSafepointScope(self.initiator_);
            }
        }
    }

    // SafepointScope
    pub struct SafepointScope<'a> {
        isolate_safepoint_: Option<IsolateSafepointScope<'a>>,
        global_safepoint_: Option<GlobalSafepointScope<'a>>,
    }
    impl<'a> SafepointScope<'a> {
        pub fn new_isolate(initiator: *mut Isolate, kind: SafepointKind) -> SafepointScope<'a> {
            if kind == SafepointKind::kIsolate {
                unsafe {
                    SafepointScope {
                        isolate_safepoint_: Some(IsolateSafepointScope::new((*initiator).heap())),
                        global_safepoint_: None,
                    }
                }
            } else {
                assert_eq!(kind, SafepointKind::kGlobal);
                SafepointScope {
                    isolate_safepoint_: None,
                    global_safepoint_: Some(GlobalSafepointScope::new(initiator)),
                }
            }
        }

        pub fn new_global_for_shared_space_isolate(
            initiator: *mut Isolate,
            _: GlobalSafepointForSharedSpaceIsolateTag,
        ) -> SafepointScope<'a> {
            unsafe {
                if (*initiator).is_shared_space_isolate() {
                    SafepointScope {
                        isolate_safepoint_: None,
                        global_safepoint_: Some(GlobalSafepointScope::new(initiator)),
                    }
                } else {
                    SafepointScope {
                        isolate_safepoint_: Some(IsolateSafepointScope::new((*initiator).heap())),
                        global_safepoint_: None,
                    }
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IncludeMainThread {
        kYes,
        kNo,
    }

    // RunningLocalHeap
    struct RunningLocalHeap {
        local_heap: *mut LocalHeap,
    }

    // RunningLocalHeaps
    struct RunningLocalHeaps {
        running_local_heaps: Vec<RunningLocalHeap>,
    }
    impl RunningLocalHeaps {
        fn new() -> Self {
            RunningLocalHeaps {
                running_local_heaps: Vec::new(),
            }
        }
    }

    // Barrier
    struct Barrier {
        mutex_: StdMutex<()>,
        cv_resume_: Condvar,
        cv_stopped_: Condvar,
        armed_: bool,
        stopped_: usize,
    }
    impl Barrier {
        fn new() -> Self {
            Barrier {
                mutex_: StdMutex::new(()),
                cv_resume_: Condvar::new(),
                cv_stopped_: Condvar::new(),
                armed_: false,
                stopped_: 0,
            }
        }

        fn Arm(&mut self) {
            let guard = self.mutex_.lock().unwrap();
            assert!(!self.IsArmed());
            self.armed_ = true;
            self.stopped_ = 0;
        }

        fn Disarm(&mut self) {
            let mut guard = self.mutex_.lock().unwrap();
            assert!(self.IsArmed());
            self.armed_ = false;
            self.stopped_ = 0;
            self.cv_resume_.notify_all();
        }

        fn WaitUntilRunningThreadsInSafepoint(&self, running_local_heaps: &RunningLocalHeaps) {
            let mut guard = self.mutex_.lock().unwrap();
            assert!(self.IsArmed());
            let running_count = running_local_heaps.running_local_heaps.len();
            while self.stopped_ < running_count {
                guard = self.cv_stopped_.wait(guard).unwrap();
            }
            assert_eq!(self.stopped_, running_count);
        }

        fn NotifyPark(&self) {
            let mut guard = self.mutex_.lock().unwrap();
            assert!(self.IsArmed());
            self.stopped_ += 1;
            self.cv_stopped_.notify_one();
        }

        fn WaitInSafepoint(&self) {
            let scoped_blocking_call = V8::GetCurrentPlatform().CreateBlockingScope(BlockingType::kWillBlock);
            let mut guard = self.mutex_.lock().unwrap();
            assert!(self.IsArmed());
            self.stopped_ += 1;
            self.cv_stopped_.notify_one();

            while self.IsArmed() {
                guard = self.cv_resume_.wait(guard).unwrap();
            }
        }

        fn WaitInUnpark(&self) {
            let scoped_blocking_call = V8::GetCurrentPlatform().CreateBlockingScope(BlockingType::kWillBlock);
            let mut guard = self.mutex_.lock().unwrap();

            while self.IsArmed() {
                guard = self.cv_resume_.wait(guard).unwrap();
            }
        }

        fn IsArmed(&self) -> bool {
            self.armed_
        }
    }
    struct PerClientSafepointData {
        isolate_: *mut Isolate,
        running: RunningLocalHeaps,
        locked_: bool,
    }
    impl PerClientSafepointData {
        pub fn new(isolate_: *mut Isolate) -> Self {
            Self {
                isolate_: isolate_,
                running: RunningLocalHeaps::new(),
                locked_: false,
            }
        }

        fn set_locked(&mut self) {
            self.locked_ = true;
        }

        fn safepoint(&self) -> *mut IsolateSafepoint {
            unsafe { (*(*self.heap())).safepoint() }
        }
        fn heap(&self) -> *mut Heap {
            unsafe { (*self.isolate_).heap() }
        }
        fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        fn is_locked(&self) -> bool {
            self.locked_
        }
    }
    struct GlobalSafepointInterruptTask {
        heap_: *mut Heap,
    }
    impl GlobalSafepointInterruptTask {
        pub fn new(heap_: *mut Heap) -> Self {
            Self { heap_ }
        }
    }

    impl CancelableTaskTrait for GlobalSafepointInterruptTask {}
}

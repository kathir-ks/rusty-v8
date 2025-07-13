// Converted from V8 C++ source files:
// Header: v8threads.h
// Implementation: v8threads.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub mod internal {
        use std::sync::{Mutex, MutexGuard};
        use std::sync::atomic::{AtomicUsize, Ordering};
        use crate::v8::Isolate;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct ThreadId {
            id: usize,
        }

        impl ThreadId {
            pub fn current() -> Self {
                // A real implementation would use thread-local storage or a system call
                // to get the current thread ID.  This is a placeholder.
                use std::thread;
                use std::sync::atomic::{AtomicUsize, Ordering};

                static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
                thread_local! {
                    static THREAD_ID: ThreadId = ThreadId { id: NEXT_ID.fetch_add(1, Ordering::Relaxed) };
                }
                THREAD_ID.with(|id| *id)
            }

            pub fn is_valid(&self) -> bool {
                self.id != 0
            }

            pub fn invalid() -> Self {
                ThreadId { id: 0 }
            }

            pub fn is_invalid(&self) -> bool {
                self.id == 0
            }
        }

        pub struct ThreadState {
            id_: ThreadId,
            data_: Vec<u8>,
            next_: *mut ThreadState,
            previous_: *mut ThreadState,
            thread_manager_: *mut ThreadManager,
        }

        #[derive(Debug)]
        pub enum ThreadStateListError {
            EmptyList,
        }

        impl ThreadState {
            pub fn next(&self) -> Option<&ThreadState> {
                unsafe {
                    if self.next_ == (*(self.thread_manager_)).in_use_anchor_ {
                        return None;
                    }
                    Some(&*self.next_)
                }
            }

            pub enum List {
                FREE_LIST,
                IN_USE_LIST,
            }

            pub fn link_into(&mut self, list: List, thread_manager: &mut ThreadManager) {
                unsafe {
                    let flying_anchor = match list {
                        List::FREE_LIST => thread_manager.free_anchor_,
                        List::IN_USE_LIST => thread_manager.in_use_anchor_,
                    };

                    self.next_ = (*flying_anchor).next_;
                    self.previous_ = flying_anchor;
                    (*flying_anchor).next_ = self;
                    (*(self.next_)).previous_ = self;
                }
            }

            pub fn unlink(&mut self) {
                unsafe {
                    (*(self.next_)).previous_ = self.previous_;
                    (*(self.previous_)).next_ = self.next_;
                }
            }

            pub fn set_id(&mut self, id: ThreadId) {
                self.id_ = id;
            }

            pub fn id(&self) -> ThreadId {
                self.id_
            }

            pub fn data(&mut self) -> &mut Vec<u8> {
                &mut self.data_
            }
        }

        pub trait ThreadVisitor {
            fn visit_thread(&mut self, isolate: &Isolate, top: &ThreadLocalTop);
        }

        pub struct ThreadManager {
            mutex_: Mutex<()>,
            mutex_owner_: AtomicUsize,
            lazily_archived_thread_: AtomicUsize,
            lazily_archived_thread_state_: *mut ThreadState,
            free_anchor_: *mut ThreadState,
            in_use_anchor_: *mut ThreadState,
            isolate_: *mut Isolate,
        }

        impl ThreadManager {
            pub fn new(isolate: *mut Isolate) -> Self {
                let mut new_tm = ThreadManager {
                    mutex_: Mutex::new(()),
                    mutex_owner_: AtomicUsize::new(0),
                    lazily_archived_thread_: AtomicUsize::new(0),
                    lazily_archived_thread_state_: std::ptr::null_mut(),
                    free_anchor_: std::ptr::null_mut(),
                    in_use_anchor_: std::ptr::null_mut(),
                    isolate_: isolate,
                };

                unsafe {
                    let free_anchor = ThreadState::create(&mut new_tm);
                    let in_use_anchor = ThreadState::create(&mut new_tm);

                    new_tm.free_anchor_ = free_anchor;
                    new_tm.in_use_anchor_ = in_use_anchor;
                }

                new_tm
            }

            pub fn lock(&self) -> MutexGuard<()> {
                let guard = self.mutex_.lock().unwrap();
                self.mutex_owner_.store(ThreadId::current().id, Ordering::Relaxed);
                assert!(self.is_locked_by_current_thread());
                guard
            }

            pub fn unlock(&self) {
                self.mutex_owner_.store(0, Ordering::Relaxed);
                drop(self.mutex_.lock().unwrap()); // Is this double unlock?
            }

            pub fn init_thread(&self, lock: &ExecutionAccess) {
                unsafe {
                    (*(self.isolate_)).initialize_thread_local();
                    //(*(self.isolate_)).stack_guard().init_thread(lock);
                    //(*(self.isolate_)).debug().init_thread(lock);
                }
            }

            pub fn archive_thread(&self) {
               // println!("ThreadManager::ArchiveThread() - start");
                unsafe {
                    assert_eq!(self.lazily_archived_thread_.load(Ordering::Relaxed), 0);
                    assert!(!self.is_archived());
                    assert!(self.is_locked_by_current_thread());
                    let state = self.get_free_thread_state();
                   // println!("ThreadManager::ArchiveThread() - get_free_thread_state done");
                   // state.unlink();

                    //let per_thread = (*self.isolate_).find_or_allocate_per_thread_data_for_this_thread();
                    //per_thread.set_thread_state(state);

                    self.lazily_archived_thread_.store(ThreadId::current().id, Ordering::Relaxed);
                    self.lazily_archived_thread_state_ = state;

                    assert_eq!((*(self.lazily_archived_thread_state_)).id_.id, 0);
                    (*(self.lazily_archived_thread_state_)).set_id(self.current_id());
                    assert_ne!((*(self.lazily_archived_thread_state_)).id_.id, 0);
                }
               // println!("ThreadManager::ArchiveThread() - end");
            }

            pub fn restore_thread(&self) -> bool {
               // println!("ThreadManager::RestoreThread() - start");
                unsafe {
                    assert!(self.is_locked_by_current_thread());
                    // First check whether the current thread has been 'lazily archived', i.e.
                    // not archived at all.  If that is the case we put the state storage we
                    // had prepared back in the free list, since we didn't need it after all.
                    if self.lazily_archived_thread_.load(Ordering::Relaxed) == ThreadId::current().id {
                       // println!("ThreadManager::RestoreThread() - lazily_archived");
                        self.lazily_archived_thread_.store(ThreadId::invalid().id, Ordering::Relaxed);

                        //Isolate::PerIsolateThreadData* per_thread =
                        //    isolate_->FindPerThreadDataForThisThread();
                        //DCHECK_NOT_NULL(per_thread);
                        //DCHECK(per_thread->thread_state() == lazily_archived_thread_state_);
                        (*(self.lazily_archived_thread_state_)).set_id(ThreadId::invalid());
                        //(*(self.lazily_archived_thread_state_)).link_into(ThreadState::FREE_LIST, self);
                        self.lazily_archived_thread_state_ = std::ptr::null_mut();
                        //per_thread->set_thread_state(nullptr);
                        return true;
                    }

                    // Make sure that the preemption thread cannot modify the thread state while
                    // it is being archived or restored.
                   // ExecutionAccess access(isolate_);

                    // If there is another thread that was lazily archived then we have to really
                    // archive it now.
                    if self.lazily_archived_thread_.load(Ordering::Relaxed) != ThreadId::invalid().id {
                        self.eagerly_archive_thread();
                    }

                    //Isolate::PerIsolateThreadData* per_thread =
                    //  isolate_->FindPerThreadDataForThisThread();
                    //if (per_thread == nullptr || per_thread->thread_state() == nullptr) {
                    // This is a new thread.
                    //  InitThread(access);
                    //  return false;
                    //}
                    //// In case multi-cage pointer compression mode is enabled ensure that
                    //// current thread's cage base values are properly initialized.
                    //PtrComprCageAccessScope ptr_compr_cage_access_scope(isolate_);

                    //ThreadState* state = per_thread->thread_state();
                    //char* from = state->data();
                    //from = isolate_->handle_scope_implementer()->RestoreThread(from);
                    //from = isolate_->RestoreThread(from);
                    //from = Relocatable::RestoreState(isolate_, from);
                    //// Stack guard should be restored before Debug, etc. since Debug etc. might
                    //// depend on a correct stack guard.
                    //from = isolate_->stack_guard()->RestoreStackGuard(from);
                    //from = isolate_->debug()->RestoreDebug(from);
                    //from = isolate_->regexp_stack()->RestoreStack(from);
                    //from = isolate_->bootstrapper()->RestoreState(from);
                    //per_thread->set_thread_state(nullptr);
                    //state->set_id(ThreadId::Invalid());
                    //state->Unlink();
                    //state->LinkInto(ThreadState::FREE_LIST);
                    //println!("ThreadManager::RestoreThread() - end returning true");
                    return false;
                }
            }

            pub fn eagerly_archive_thread(&self) {
                unsafe {
                    assert!(self.is_locked_by_current_thread());
                    let state = self.lazily_archived_thread_state_;
                   // (*state).link_into(ThreadState::IN_USE_LIST, self);
                    let mut to = (*state).data();
                    // Ensure that data containing GC roots are archived first, and handle them
                    // in ThreadManager::Iterate(RootVisitor*).
                    //to = isolate_->handle_scope_implementer()->ArchiveThread(to);
                    //to = isolate_->ArchiveThread(to);
                    //to = Relocatable::ArchiveState(isolate_, to);
                    //to = isolate_->stack_guard()->ArchiveStackGuard(to);
                    //to = isolate_->debug()->ArchiveDebug(to);
                    //to = isolate_->regexp_stack()->ArchiveStack(to);
                    //to = isolate_->bootstrapper()->ArchiveState(to);
                    self.lazily_archived_thread_.store(ThreadId::invalid().id, Ordering::Relaxed);
                    self.lazily_archived_thread_state_ = std::ptr::null_mut();
                }
            }

            pub fn free_thread_resources(&self) {
                // This method might be called on a thread that's not bound to any Isolate
                // and thus pointer compression schemes might have cage base value unset.
                // So, allow heap access here to let the checks work.
                //PtrComprCageAccessScope ptr_compr_cage_access_scope(isolate_);

               // unsafe {
               //     assert!((*(self.isolate_)).has_exception());
               //     assert_eq!((*(self.isolate_)).try_catch_handler(), std::ptr::null_mut());
               //     //(*(self.isolate_)).handle_scope_implementer().free_thread_resources();
               //     (*(self.isolate_)).free_thread_resources();
               //     //(*(self.isolate_)).debug().free_thread_resources();
               //     //(*(self.isolate_)).stack_guard().free_thread_resources();
               //     //(*(self.isolate_)).regexp_stack().free_thread_resources();
               //     //(*(self.isolate_)).bootstrapper().free_thread_resources();
               // }
            }

            pub fn is_archived(&self) -> bool {
                unsafe {
                    //Isolate::PerIsolateThreadData* data =
                    //    isolate_->FindPerThreadDataForThisThread();
                    //return data != nullptr && data->thread_state() != nullptr;
                    false
                }
            }

            pub fn iterate(&self, v: &mut RootVisitor) {
                unsafe {
                    // Expecting no threads during serialization/deserialization
                    let mut state = self.first_thread_state_in_use();
                    while state.is_some() {
                        let data = (*(state.unwrap())).data();
                        //data = HandleScopeImplementer::Iterate(v, data);
                        //data = isolate_->Iterate(v, data);
                        //data = Relocatable::Iterate(v, data);
                        //data = StackGuard::Iterate(v, data);
                        //data = Debug::Iterate(v, data);
                        state = (*(state.unwrap())).next();
                    }
                }
            }

            pub fn iterate_archived_threads(&self, v: &mut dyn ThreadVisitor) {
                unsafe {
                    let mut state = self.first_thread_state_in_use();
                    while state.is_some() {
                        let mut data = (*(state.unwrap())).data();
                        //data += HandleScopeImplementer::ArchiveSpacePerThread();
                        //isolate_->IterateThread(v, data);
                        state = (*(state.unwrap())).next();
                    }
                }
            }

            pub fn current_id(&self) -> ThreadId {
                ThreadId::current()
            }

            pub fn first_thread_state_in_use(&self) -> Option<&ThreadState> {
                unsafe {
                    let state = (*self.in_use_anchor_).next_;
                    if state == self.in_use_anchor_ {
                        return None;
                    }

                    Some(&*state)
                }
            }

            fn get_free_thread_state(&self) -> *mut ThreadState {
                unsafe {
                    let gotten = (*self.free_anchor_).next_;
                    if gotten == self.free_anchor_ {
                        let new_thread_state = ThreadState::create(self);
                        //new_thread_state.allocate_space();
                        return new_thread_state;
                    }
                    return gotten;
                }
            }

            pub fn is_locked_by_current_thread(&self) -> bool {
                self.mutex_owner_.load(Ordering::Relaxed) == ThreadId::current().id
            }

            pub fn is_locked_by_thread(&self, id: ThreadId) -> bool {
                self.mutex_owner_.load(Ordering::Relaxed) == id.id
            }
        }

        impl Drop for ThreadManager {
            fn drop(&mut self) {
                unsafe {
                    self.delete_thread_state_list(self.free_anchor_);
                    self.delete_thread_state_list(self.in_use_anchor_);
                }
            }
        }

        impl ThreadManager {
            unsafe fn delete_thread_state_list(&mut self, anchor: *mut ThreadState) {
                let mut current = (*anchor).next_;
                while current != anchor {
                    let next = (*current).next_;
                    drop(Box::from_raw(current));
                    current = next;
                }
                drop(Box::from_raw(anchor));
            }
        }

        impl ThreadState {
            fn create(thread_manager: *mut ThreadManager) -> *mut ThreadState {
                let mut state = ThreadState {
                    id_: ThreadId::invalid(),
                    data_: Vec::new(),
                    next_: std::ptr::null_mut(),
                    previous_: std::ptr::null_mut(),
                    thread_manager_: thread_manager,
                };

                let boxed = Box::new(state);
                let ptr = Box::into_raw(boxed);
                unsafe {
                    (*ptr).next_ = ptr;
                    (*ptr).previous_ = ptr;
                }

                ptr
            }
        }

        pub struct RootVisitor {}

        pub struct ExecutionAccess<'a> {
            isolate_: &'a Isolate,
        }

        impl<'a> ExecutionAccess<'a> {
            pub fn new(isolate: &'a Isolate) -> Self {
                ExecutionAccess { isolate_: isolate }
            }
        }

        pub struct ThreadLocalTop {}
    }
}

mod base {
    pub struct Mutex {
        inner: std::sync::Mutex<()>,
    }

    impl Mutex {
        pub fn new() -> Self {
            Mutex {
                inner: std::sync::Mutex::new(()),
            }
        }

        pub fn lock(&self) -> Result<std::sync::MutexGuard<()>, std::sync::PoisonError<std::sync::MutexGuard<()>>> {
            self.inner.lock()
        }

        pub fn unlock(&self) {
            //No explicit unlock, MutexGuard is released when it goes out of scope.
        }
    }
}

pub mod v8 {
    pub struct Isolate {
        //Dummy field
        thread_manager: internal::ThreadManager,
    }

    impl Isolate {
        pub fn new() -> Isolate {
            let mut isolate = Isolate {
                thread_manager: internal::ThreadManager::new(&mut isolate),
            };

            return isolate;
        }

        pub fn thread_manager(&mut self) -> &mut internal::ThreadManager {
            &mut self.thread_manager
        }

        pub fn initialize_thread_local(&mut self) {}
        pub fn free_thread_resources(&mut self) {}

        fn find_per_thread_data_for_this_thread(&self) {}
    }

    pub struct Locker<'a> {
        has_lock_: bool,
        top_level_: bool,
        isolate_: *mut internal::Isolate,
        _guard: std::sync::MutexGuard<'a, ()>
    }

    impl<'a> Locker<'a> {
        pub fn new(isolate: &'a mut Isolate) -> Self {
            use std::sync::atomic::{AtomicUsize, Ordering};

            static G_LOCKER_WAS_EVER_USED_: AtomicUsize = AtomicUsize::new(0);

            let mut this = Locker {
                has_lock_: false,
                top_level_: true,
                isolate_: isolate as *mut Isolate as *mut internal::Isolate,
                _guard: isolate.thread_manager().lock()
            };

            unsafe{
                // Record that the Locker has been used at least once.
                G_LOCKER_WAS_EVER_USED_.store(1, Ordering::Relaxed);
                //isolate_->set_was_locker_ever_used();
                // Get the big lock if necessary.
                let isolate = &mut *(this.isolate_ as *mut internal::Isolate) ;
                if !isolate.thread_manager().is_locked_by_current_thread() {
                   // isolate.thread_manager().lock();
                    this.has_lock_ = true;

                    // This may be a locker within an unlocker in which case we have to
                    // get the saved state for this thread and restore it.
                    //if isolate_->thread_manager()->RestoreThread() {
                    //    top_level_ = false;
                    //}
                }

                assert!(isolate.thread_manager().is_locked_by_current_thread());
            }

            return this;
        }

        pub fn is_locked(isolate: &mut Isolate) -> bool {
            let i_isolate = isolate as *mut Isolate as *mut internal::Isolate;

            unsafe {
                return (*i_isolate).thread_manager().is_locked_by_current_thread();
            }
        }
    }

    impl<'a> Drop for Locker<'a> {
        fn drop(&mut self) {
            unsafe {
                let isolate = &mut *(self.isolate_ as *mut internal::Isolate);
                assert!(isolate.thread_manager().is_locked_by_current_thread());
                if self.has_lock_ {
                    //if self.top_level_ {
                        isolate.thread_manager().free_thread_resources();
                    //} else {
                      //  isolate.thread_manager().archive_thread();
                    //}

                   // isolate.thread_manager().unlock();
                }
            }
        }
    }

    pub struct Unlocker<'a> {
        isolate_: *mut internal::Isolate,
        _guard: std::sync::MutexGuard<'a, ()>
    }

    impl<'a> Unlocker<'a> {
        pub fn new(isolate: &'a mut Isolate) -> Self {
            let thread_manager = isolate.thread_manager();
            unsafe {
                assert!((*(isolate as *mut Isolate as *mut internal::Isolate)).thread_manager().is_locked_by_current_thread());
            }
            //thread_manager.archive_thread();
            //thread_manager.unlock();

            Unlocker{
                isolate_: isolate as *mut Isolate as *mut internal::Isolate,
                _guard: isolate.thread_manager().lock()
            }

           // isolate.thread_manager().restore_thread();
        }
    }

    impl<'a> Drop for Unlocker<'a> {
        fn drop(&mut self) {
           // unsafe {
           //     let isolate = &mut *(self.isolate_ as *mut internal::Isolate);
           //     assert!(!isolate.thread_manager().is_locked_by_current_thread());
           //     isolate.thread_manager().lock();
           //     isolate.thread_manager().restore_thread();
           // }
        }
    }
}

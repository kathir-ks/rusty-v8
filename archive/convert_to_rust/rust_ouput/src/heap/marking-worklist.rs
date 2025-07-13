// Converted from V8 C++ source files:
// Header: marking-worklist.h
// Implementation: marking-worklist.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap {
    pub mod base {
        pub struct Worklist<T, const SIZE: usize> {
            // items: Vec<T>,
        }

        impl<T, const SIZE: usize> Worklist<T, SIZE> {
            pub fn Clear(&mut self) {}
            pub fn Iterate<F>(&mut self, _callback: F)
            where
                F: Fn(T),
            {
            }
            pub fn IsLocalEmpty(&self) -> bool {
                true
            }
            pub fn IsGlobalEmpty(&self) -> bool {
                true
            }
            pub fn Publish(&mut self) {}
            pub fn Merge(&mut self, _other: Worklist<T, SIZE>) {}
            pub fn Pop(&mut self, _object: &mut T) -> bool {
                false
            }
        }

        pub mod internal {
            pub struct Tagged<T> {
                _phantom: std::marker::PhantomData<T>,
            }
        }
    }

    pub mod cppgc_js {
        pub struct CppHeap {}
        pub struct CppMarkingState {
             local_empty : bool,
        }

        impl CppMarkingState {
            pub fn IsLocalEmpty(&self) -> bool {
                self.local_empty
            }
        }
    }

    pub mod utils {
        pub struct AddressMap {}
    }

    pub mod objects {
        pub struct HeapObject {}
        pub mod heap_object_inl {
            pub struct HeapObjectInl {}
        }
        pub mod instance_type_inl {
            pub struct InstanceTypeInl {}
        }
        pub struct Map {
           instance_type : i32,
        }
        impl Map{
            pub fn instance_type(&self) -> i32{
                self.instance_type
            }
        }
        pub mod objects_definitions {
            pub struct ObjectsDefinitions {}
        }
    }

    use std::collections::HashMap;
    use std::ptr::null_mut;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::Address;
    use self::base::Worklist;
    use self::base::internal::Tagged;
    use self::cppgc_js::CppMarkingState;
    use self::objects::HeapObject;

    // The index of the main thread task used by concurrent/parallel GC.
    pub const K_MAIN_THREAD_TASK: i32 = 0;

    // We piggyback on marking to compute object sizes per native context that is
    // needed for the new memory measurement API. The algorithm works as follows:
    // 1) At the start of marking we create a marking worklist for each context.
    //    The existing shared, on_hold, and embedder worklists continue to work
    //    as they did before, but they hold objects that are not attributed to any
    //    context yet.
    // 2) Each marker has an active worklist where it pushes newly discovered
    //    objects. Initially the shared worklist is set as active for all markers.
    // 3) When a marker pops an object from the active worklist:
    //    a) It checks if the object has a known context (e.g. JSObjects, Maps,
    //       Contexts know the context they belong to). If that's the case, then
    //       the marker changes its active worklist to the worklist corresponding
    //       to the context of the object.
    //    b) It account the size of object to the active context.
    //    c) It visits all pointers in the object and pushes new objects onto the
    //       active worklist.
    // 4) When the active worklist becomes empty the marker selects any other
    //    non-empty worklist as the active worklist.
    // 5) The write barrier pushes onto the shared worklist.
    //
    // The main invariant for context worklists:
    //    If object X is in the worklist of context C, then either
    //    a) X has a context and that context is C.
    //    b) X is retained by object Y that has context C.
    //
    // The algorithm allows us to attribute context-independent objects such as
    // strings, numbers, FixedArrays to their retaining contexts. The algorithm is
    // not precise for context-independent objects that are shared between multiple
    // contexts. Such objects may be attributed to any retaining context.

    // Named pair of native context address and its marking worklist.
    // Since native contexts are allocated in the old generation, their addresses
    // a stable across Scavenges and stay valid throughout the marking phase.
    #[derive(Debug)]
    pub struct ContextWorklistPair {
        pub context: Address,
        pub worklist: Box<MarkingWorklist>,
    }

    // A helper class that owns all global marking worklists.
    #[derive(Debug)]
    pub struct MarkingWorklists {
        shared_: MarkingWorklist,
        on_hold_: MarkingWorklist,
        context_worklists_: Vec<ContextWorklistPair>,
        other_: MarkingWorklist,
    }

    impl MarkingWorklists {
        pub const K_SHARED_CONTEXT: Address = Address { address: 0 };
        pub const K_OTHER_CONTEXT: Address = Address { address: 8 };

        pub fn new() -> Self {
            MarkingWorklists {
                shared_: MarkingWorklist {},
                on_hold_: MarkingWorklist {},
                context_worklists_: Vec::new(),
                other_: MarkingWorklist {},
            }
        }

        // Calls the specified callback on each element of the deques and replaces
        // the element with the result of the callback. If the callback returns
        // nullptr then the element is removed from the deque.
        // The callback must accept HeapObject and return HeapObject.
        pub fn Update<Callback>(&mut self, _callback: Callback)
        where
            Callback: Fn(Tagged<HeapObject>) -> Tagged<HeapObject>,
        {
        }

        pub fn shared(&mut self) -> &mut MarkingWorklist {
            &mut self.shared_
        }
        pub fn on_hold(&mut self) -> &mut MarkingWorklist {
            &mut self.on_hold_
        }
        pub fn other(&mut self) -> &mut MarkingWorklist {
            &mut self.other_
        }

        // A list of (context, worklist) pairs that was set up at the start of
        // marking by CreateContextWorklists.
        pub fn context_worklists(&self) -> &Vec<ContextWorklistPair> {
            &self.context_worklists_
        }

        // This should be invoked at the start of marking with the list of contexts
        // that require object size accounting.
        pub fn CreateContextWorklists(&mut self, contexts: &Vec<Address>) {
            assert!(self.context_worklists_.is_empty());
            if contexts.is_empty() {
                return;
            }

            self.context_worklists_.reserve(contexts.len());
            for &context in contexts {
                self.context_worklists_.push(ContextWorklistPair {
                    context,
                    worklist: Box::new(MarkingWorklist {}),
                });
            }
        }

        // This should be invoked at the end of marking. All worklists must be
        // empty at that point.
        pub fn ReleaseContextWorklists(&mut self) {
            self.context_worklists_.clear();
        }

        pub fn IsUsingContextWorklists(&self) -> bool {
            !self.context_worklists_.is_empty()
        }

        pub fn Clear(&mut self) {
            self.shared_.Clear();
            self.on_hold_.Clear();
            self.other_.Clear();
            for cw in &mut self.context_worklists_ {
                cw.worklist.Clear();
            }
            self.ReleaseContextWorklists();
        }

        pub fn Print(&mut self) {}

        // Prints the stats about the global pool of the worklist.
        fn PrintWorklist(&mut self, _worklist_name: &str, _worklist: &mut MarkingWorklist) {}
    }

    #[derive(Debug)]
    pub struct AddressToIndexHashMap {
        map: Mutex<HashMap<Address, i32>>,
    }

    impl AddressToIndexHashMap {
        pub fn new() -> Self {
            AddressToIndexHashMap {
                map: Mutex::new(HashMap::new()),
            }
        }
        pub fn Set(&self, key: Address, value: i32) {
            let mut map = self.map.lock().unwrap();
            map.insert(key, value);
        }

        pub fn Get(&self, key: Address) -> Maybe<i32> {
            let map = self.map.lock().unwrap();
            match map.get(&key) {
                Some(&value) => Maybe::Just(value),
                None => Maybe::Nothing,
            }
        }

        pub fn Start(&self) -> *mut HashMapEntry {
            let map = self.map.lock().unwrap();
            if let Some((key, value)) = map.iter().next() {
                let entry = Box::new(HashMapEntry {
                    key: *key,
                    value: *value,
                });
                Box::into_raw(entry)
            } else {
                null_mut()
            }
        }

        pub fn Next(&self, entry: *mut HashMapEntry) -> *mut HashMapEntry {
            unsafe {
                if entry.is_null() {
                    return null_mut();
                }
                let entry_ref = &*entry;
                let map = self.map.lock().unwrap();
                let mut found = false;
                for (key, value) in map.iter() {
                    if found {
                        let next_entry = Box::new(HashMapEntry {
                            key: *key,
                            value: *value,
                        });
                        return Box::into_raw(next_entry);
                    }
                    if key == &entry_ref.key && value == &entry_ref.value {
                        found = true;
                    }
                }
                null_mut()
            }
        }
    }

    #[derive(Debug)]
    pub struct HashMapEntry {
        pub key: Address,
        pub value: i32,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Maybe<T> {
        Just(T),
        Nothing,
    }

    impl<T> Maybe<T> {
        pub fn IsNothing(&self) -> bool {
            match self {
                Maybe::Nothing => true,
                _ => false,
            }
        }

        pub fn FromJust(self) -> T {
            match self {
                Maybe::Just(value) => value,
                Maybe::Nothing => panic!("Trying to get value from Maybe::Nothing"),
            }
        }
    }
    #[derive(Debug)]
    pub struct MarkingWorklist {
        is_empty: AtomicBool,
    }
    impl MarkingWorklist {
        pub fn Clear(&mut self) {
            self.is_empty.store(true, Ordering::Relaxed);
        }
        pub fn Iterate<F>(&mut self, _callback: F)
        where
            F: Fn(Tagged<HeapObject>),
        {
        }
        pub fn IsLocalEmpty(&self) -> bool {
            self.is_empty.load(Ordering::Relaxed)
        }
        pub fn IsGlobalEmpty(&self) -> bool {
            self.is_empty.load(Ordering::Relaxed)
        }
        pub fn Publish(&mut self) {}
        pub fn Merge(&mut self, _other: MarkingWorklist) {}
        pub fn Pop(&mut self, _object: &mut Tagged<HeapObject>) -> bool {
            false
        }
    }

    // A thread-local view of the marking worklists. It owns all local marking
    // worklists and keeps track of the currently active local marking worklist
    // for per-context marking. In order to avoid additional indirections for
    // pushing and popping entries, the active_ worklist is not a pointer to
    // Local but an actual instance of Local with the following invariants:
    // - active_owner == worlist_by_context[active_context_].get()
    // - *active_owner is empty (all fields are null) because its content has
    //   been moved to active_.
    #[derive(Debug)]
    pub struct MarkingWorklistsLocal<'a> {
        active_: MarkingWorklistLocalImpl<'a>,
        shared_: MarkingWorklistLocalImpl<'a>,
        on_hold_: MarkingWorklistLocalImpl<'a>,
        active_context_: Address,
        is_per_context_mode_: bool,
        context_worklists_: Vec<MarkingWorklistLocalImpl<'a>>,
        worklist_by_context_: AddressToIndexHashMap,
        other_: MarkingWorklistLocalImpl<'a>,
        cpp_marking_state_: Option<Box<CppMarkingState>>,
    }

    impl<'a> MarkingWorklistsLocal<'a> {
        pub const K_SHARED_CONTEXT: Address = Address { address: 0 };
        pub const K_OTHER_CONTEXT: Address = Address { address: 8 };
        pub const K_NO_CPP_MARKING_STATE: *mut CppMarkingState = std::ptr::null_mut();

        pub fn new(
            global: &'a mut MarkingWorklists,
            cpp_marking_state: Option<Box<CppMarkingState>>,
        ) -> Self {
            let is_per_context_mode_ = !global.context_worklists().is_empty();
            let mut local = Self {
                active_: MarkingWorklistLocalImpl::new(&mut global.shared_),
                shared_: MarkingWorklistLocalImpl::new(&mut global.shared_),
                on_hold_: MarkingWorklistLocalImpl::new(&mut global.on_hold_),
                active_context_: Self::K_SHARED_CONTEXT,
                is_per_context_mode_,
                context_worklists_: Vec::new(),
                worklist_by_context_: AddressToIndexHashMap::new(),
                other_: MarkingWorklistLocalImpl::new(&mut global.other_),
                cpp_marking_state_: cpp_marking_state,
            };
            if local.is_per_context_mode_ {
                local.context_worklists_.reserve(global.context_worklists().len());
                let mut index = 0;
                for cw in global.context_worklists() {
                    local.context_worklists_.push(MarkingWorklistLocalImpl::new(&mut cw.worklist));
                    local.worklist_by_context_.Set(cw.context, index as i32);
                    index += 1;
                }
            }
            local
        }

        pub fn Push(&mut self, object: Tagged<HeapObject>) {
            self.active_.Push(object);
        }
        pub fn Pop(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            self.active_.Pop(object)
        }

        pub fn PushOnHold(&mut self, object: Tagged<HeapObject>) {
            self.on_hold_.Push(object);
        }
        pub fn PopOnHold(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            self.on_hold_.Pop(object)
        }

        pub fn Publish(&mut self) {
            self.shared_.Publish();
            self.on_hold_.Publish();
            self.other_.Publish();
            if self.is_per_context_mode_ {
                for entry in 0..self.context_worklists_.len() {
                    self.context_worklists_[entry].Publish();
                }
            }
            self.PublishCppHeapObjects();
        }

        pub fn IsEmpty(&mut self) -> bool {
            // This function checks the on_hold_ worklist, so it works only for the main
            // thread.
            if !self.active_.IsLocalEmpty()
                || !self.on_hold_.IsLocalEmpty()
                || !self.active_.IsGlobalEmpty()
                || !self.on_hold_.IsGlobalEmpty()
            {
                return false;
            }
            if !self.is_per_context_mode_ {
                return true;
            }
            if !self.shared_.IsLocalEmpty()
                || !self.other_.IsLocalEmpty()
                || !self.shared_.IsGlobalEmpty()
                || !self.other_.IsGlobalEmpty()
            {
                return false;
            }
            for entry in 0..self.context_worklists_.len() {
                let worklist = &self.context_worklists_[entry];
                let address = self.worklist_by_context_.map.lock().unwrap().iter().find_map(|(addr, &index)| if index as usize == entry { Some(*addr) } else { None });
                if let Some(address) = address {
                    if address != self.active_context_ &&
                        !(worklist.IsLocalEmpty() && worklist.IsGlobalEmpty()) {
                        self.SwitchToContextImpl(address, &self.context_worklists_[entry]);
                        return false;
                    }
                }

            }
            true
        }

        pub fn IsWrapperEmpty(&self) -> bool {
            self.cpp_marking_state_.is_none() || self.cpp_marking_state_.as_ref().unwrap().IsLocalEmpty()
        }

        pub fn ShareWork(&mut self) {
            if !self.active_.IsLocalEmpty() && self.active_.IsGlobalEmpty() {
                self.active_.Publish();
            }
            if self.is_per_context_mode_ && self.active_context_ != Self::K_SHARED_CONTEXT {
                if !self.shared_.IsLocalEmpty() && self.shared_.IsGlobalEmpty() {
                    self.shared_.Publish();
                }
            }
        }

        pub fn PublishWork(&mut self) {
            assert!(!self.is_per_context_mode_);
            self.shared_.Publish();
        }

        pub fn MergeOnHold(&mut self) {
            self.shared_.Merge(&mut self.on_hold_);
        }

        // Publishes CppHeap objects.
        pub fn PublishCppHeapObjects(&mut self) {}

        // Returns the context of the active worklist.
        pub fn Context(&self) -> Address {
            self.active_context_
        }
        pub fn SwitchToContext(&mut self, context: Address) -> Address {
            if self.is_per_context_mode_ {
                self.SwitchToContextSlow(context)
            } else {
                self.active_context_ = context;
                context
            }

        }
        pub fn IsPerContextMode(&self) -> bool {
            self.is_per_context_mode_
        }

        pub fn cpp_marking_state(&self) -> Option<&CppMarkingState> {
            self.cpp_marking_state_.as_ref().map(|s| &**s)
        }

        pub fn SwitchToSharedForTesting(&mut self) -> Address {
            self.SwitchToContext(Self::K_SHARED_CONTEXT)
        }

        fn SwitchToContextImpl(&mut self, context: Address, worklist: &mut MarkingWorklistLocalImpl<'a>) {
            self.active_ = MarkingWorklistLocalImpl::new(worklist.global);
            self.active_context_ = context;
        }

        fn PopContext(&mut self, _object: &mut Tagged<HeapObject>) -> bool {
            assert!(self.is_per_context_mode_);
            // As an optimization we first check only the local segments to avoid locks.
            for entry in 0..self.context_worklists_.len() {

                let address = self.worklist_by_context_.map.lock().unwrap().iter().find_map(|(addr, &index)| if index as usize == entry { Some(*addr) } else { None });
                if let Some(address) = address {
                    if address != self.active_context_ && !self.context_worklists_[entry].IsLocalEmpty() {
                        self.SwitchToContextImpl(address, &mut self.context_worklists_[entry]);
                        return self.active_.Pop(_object);
                    }
                }


            }
            // All local segments are empty. Check global segments.
            for entry in 0..self.context_worklists_.len() {

                let address = self.worklist_by_context_.map.lock().unwrap().iter().find_map(|(addr, &index)| if index as usize == entry { Some(*addr) } else { None });
                if let Some(address) = address {
                    if address != self.active_context_ && self.context_worklists_[entry].Pop(_object) {
                        self.SwitchToContextImpl(address, &mut self.context_worklists_[entry]);
                        return true;
                    }
                }

            }
            // All worklists are empty. Switch to the default shared worklist.
            self.SwitchToContext(Self::K_SHARED_CONTEXT);
            false
        }

        fn SwitchToContextSlow(&mut self, context: Address) -> Address {
            match self.worklist_by_context_.Get(context) {
                Maybe::Just(index) => {
                    self.SwitchToContextImpl(context, &mut self.context_worklists_[index as usize]);
                }
                Maybe::Nothing => {
                    // The context passed is not an actual context:
                    // - Shared context that should use the explicit worklist.
                    // - This context was created during marking and should use the other
                    // bucket.
                    if context == Self::K_SHARED_CONTEXT {
                        self.SwitchToContextImpl(Self::K_SHARED_CONTEXT, &mut self.shared_);
                    } else {
                        self.SwitchToContextImpl(Self::K_OTHER_CONTEXT, &mut self.other_);
                    }
                }
            }
            self.active_context_
        }
    }

    #[derive(Debug)]
    struct MarkingWorklistLocalImpl<'a> {
        global: &'a mut MarkingWorklist,
    }
    impl<'a> MarkingWorklistLocalImpl<'a> {
        fn new(global: &'a mut MarkingWorklist) -> Self {
            MarkingWorklistLocalImpl { global }
        }
        fn Push(&mut self, object: Tagged<HeapObject>) {}
        fn Pop(&mut self, object: &mut Tagged<HeapObject>) -> bool {
            self.global.Pop(object)
        }
        fn Publish(&mut self) {
            self.global.Publish();
        }
        fn Merge(&mut self, other: &mut MarkingWorklistLocalImpl<'a>) {
            self.global.Merge(MarkingWorklist{});
        }

        fn IsLocalEmpty(&self) -> bool {
            self.global.IsLocalEmpty()
        }
        fn IsGlobalEmpty(&self) -> bool {
            self.global.IsGlobalEmpty()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Address {
        address: usize,
    }
}

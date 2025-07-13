// Converted from V8 C++ source files:
// Header: cpp-snapshot.h
// Implementation: cpp-snapshot.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cpp_snapshot {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::error::Error;
    use std::fmt;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;
    use std::ptr;
    use std::rc::Rc;
    use std::sync::{Mutex, RwLock};

    use crate::heap::cppgc_js::cpp_heap::*;
    use crate::heap::cppgc_js::cpp_heap::{CppHeap, CppHeapCreateParams, Isolate};
    use crate::heap::heap_allocator::*;
    use crate::profiler::heap_profiler::*;
    use crate::objects::objects::*;
    use crate::objects::objects_inl::*;
    use crate::codegen::interface_descriptors::*;
    use crate::heap::factory_base::*;
    use crate::heap::safepoint::*;

    pub struct EmbedderGraph;
    impl EmbedderGraph {
        pub fn AddNode(_node: std::unique_ptr<Node>) -> *mut Node {
            todo!()
        }
        pub fn AddNativeSize(_size: usize){
            todo!()
        }
        pub fn AddEdge(_node: *mut Node, _node2: *mut Node){
            todo!()
        }
        pub fn AddEdge(_node: *mut Node, _node2: *mut Node, _edge_name: *const char){
            todo!()
        }
        pub fn V8Node(_data: v8::Local<v8::Data>) -> *mut Node{
            todo!()
        }
    }

    pub mod v8 {
        pub struct Local<T> {
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> Local<T> {
            pub fn IsEmpty(&self) -> bool {
                true
            }
            pub fn As<U>(&self) -> Local<U> {
                Local::<U> {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
        pub mod v8 {
            pub struct Data {}
            pub struct Value {}
            pub struct Object {}
        }
        pub use v8::*;
        pub enum Detachedness {
            kUnknown,
        }
    }

    pub struct Node;
    impl Node {
        pub fn InternalizeEdgeName(&self, _edge_name: std::string::String) -> *const char {
            todo!()
        }
        pub fn SetWrapperNode(&self, _wrapper_node: *mut Node) {
            todo!()
        }
        pub fn SizeInBytes(&self) -> usize {
            todo!()
        }
        pub fn GetDetachedness(&self) -> v8::Detachedness{
            todo!()
        }
    }

    pub trait NodeTrait {
        fn Name(&self) -> *const char;
        fn SizeInBytes(&self) -> usize;
        fn SetWrapperNode(&mut self, wrapper_node: *mut dyn NodeTrait);
        fn WrapperNode(&self) -> *mut dyn NodeTrait;
        fn SetDetachedness(&mut self, detachedness: v8::Detachedness);
        fn GetDetachedness(&self) -> v8::Detachedness;
        fn IsRootNode(&self) -> bool;
        fn GetAddress(&self) -> *const std::ffi::c_void;
        //fn InternalizeEdgeName(&self, edge_name: std::string::String) -> *const char;
    }

    pub mod cppgc {
        pub mod internal {
            pub struct HeapObjectName {
                pub value: *const char,
                pub name_was_hidden: bool,
            }
        }
        pub mod subtle {
            pub struct DisallowGarbageCollectionScope {
                _heap_handle: i32,
            }
            impl DisallowGarbageCollectionScope {
                pub fn new(_heap_handle: i32) -> Self {
                    DisallowGarbageCollectionScope{_heap_handle}
                }
            }
        }
        pub struct SourceLocation;
        impl SourceLocation {
            pub fn ToString(&self) -> String {
                todo!()
            }
        }
        pub struct TraceDescriptor{
            pub base_object_payload: *mut std::ffi::c_void,
            pub callback: TraceCallback,
        }
        pub type TraceCallback = fn(&dyn Visitor, *const std::ffi::c_void);

        pub trait Visitor {
            fn Visit(&mut self, obj: *const std::ffi::c_void, desc: TraceDescriptor);
            fn VisitWeakContainer(&mut self, object: *const std::ffi::c_void, strong_desc: TraceDescriptor, weak_desc: TraceDescriptor, weak_callback: WeakCallback, user_data: *const std::ffi::c_void);
            fn VisitEphemeron(&mut self, key: *const std::ffi::c_void, value: *const std::ffi::c_void, value_desc: TraceDescriptor);
        }
        pub type WeakCallback = fn();
        pub mod internal{
            pub struct PersistentRegionLock;
        }
    }

    pub mod heap{
        pub mod base{
            pub struct StackVisitor;
        }
    }

    pub mod i {
        pub struct IsolateForSandbox{}
    }

    pub mod Utils {
        pub fn OpenDirectHandle<T>(_value: *const T) -> *mut T {
            todo!()
        }
    }

    pub mod execution{
        pub struct Isolate{}
    }

    pub struct TracedReferenceBase;
    impl TracedReferenceBase{
        pub fn Get(&self, _isolate: *mut Isolate) -> v8::Local<v8::Data> {
            v8::Local::<v8::Data> {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub trait Visitor {
        fn visit(&mut self, address: *const std::ffi::c_void);
    }

    pub struct CppGraphBuilder;

    impl CppGraphBuilder {
        pub fn Run(isolate: *mut Isolate, graph: *mut EmbedderGraph, data: *mut std::ffi::c_void) {
            let cpp_heap = data as *mut CppHeap;
            assert!(!cpp_heap.is_null());
            let graph = graph as *mut EmbedderGraph;
            assert!(!graph.is_null());

            let mut graph_builder = CppGraphBuilderImpl::new(unsafe { &mut *cpp_heap }, unsafe { &mut *graph });
            graph_builder.run();
        }
    }

    // Node representing a C++ object on the heap.
    pub struct EmbedderNode {
        header_address_: *const std::ffi::c_void,
        name_: *const char,
        size_: usize,
        wrapper_node_: *mut Node,
        detachedness_: v8::Detachedness,
        named_edges_: Vec<std::unique_ptr<[std::ffi::c_char]>>,
    }

    impl EmbedderNode {
        fn new(header_address: *const std::ffi::c_void, name: cppgc::internal::HeapObjectName, size: usize) -> Self {
            EmbedderNode {
                header_address_: header_address,
                name_: name.value,
                size_: if name.name_was_hidden { 0 } else { size },
                wrapper_node_: ptr::null_mut(),
                detachedness_: v8::Detachedness::kUnknown,
                named_edges_: Vec::new(),
            }
        }
        fn InternalizeEdgeName(&mut self, edge_name: std::string::String) -> *const char {
            let edge_name_len = edge_name.len();
            let mut named_edge = std::unique_ptr::<[std::ffi::c_char]>::new_uninit(edge_name_len + 1);
            unsafe {
                let named_edge_ptr = named_edge.as_mut_ptr() as *mut std::ffi::c_char;
                snprintf(named_edge_ptr, edge_name_len + 1, "%s", edge_name.as_ptr() as *const i8);
                self.named_edges_.push(named_edge.assume_init());
                self.named_edges_.last().unwrap().as_ptr() as *const char
            }
        }
    }

    impl NodeTrait for EmbedderNode {
        fn Name(&self) -> *const char {
            self.name_
        }
        fn SizeInBytes(&self) -> usize {
            self.size_
        }
        fn SetWrapperNode(&mut self, wrapper_node: *mut dyn NodeTrait) {
            self.wrapper_node_ = wrapper_node as *mut Node;
        }
        fn WrapperNode(&self) -> *mut dyn NodeTrait {
            self.wrapper_node_ as *mut dyn NodeTrait
        }
        fn SetDetachedness(&mut self, detachedness: v8::Detachedness) {
            self.detachedness_ = detachedness;
        }
        fn GetDetachedness(&self) -> v8::Detachedness {
            self.detachedness_
        }
        fn IsRootNode(&self) -> bool {
            false
        }
        fn GetAddress(&self) -> *const std::ffi::c_void {
            self.header_address_
        }
    }

    // Node representing an artificial root group, e.g., set of Persistent handles.
    pub struct EmbedderRootNode {
        base: EmbedderNode,
    }

    impl EmbedderRootNode {
        fn new(name: *const char) -> Self {
            EmbedderRootNode {
                base: EmbedderNode::new(kNoNativeAddress, cppgc::internal::HeapObjectName { value: name, name_was_hidden: false }, 0),
            }
        }
    }

    impl NodeTrait for EmbedderRootNode {
        fn Name(&self) -> *const char {
            self.base.name_
        }
        fn SizeInBytes(&self) -> usize {
            self.base.size_
        }
        fn SetWrapperNode(&mut self, _wrapper_node: *mut dyn NodeTrait) {}
        fn WrapperNode(&self) -> *mut dyn NodeTrait {
            std::ptr::null_mut()
        }
        fn SetDetachedness(&mut self, detachedness: v8::Detachedness) {
            self.base.detachedness_ = detachedness;
        }
        fn GetDetachedness(&self) -> v8::Detachedness {
            self.base.detachedness_
        }
        fn IsRootNode(&self) -> bool {
            true
        }
        fn GetAddress(&self) -> *const std::ffi::c_void {
            self.base.header_address_
        }
    }

    const kNoNativeAddress: *const std::ffi::c_void = std::ptr::null();

    // Canonical state representing real and artificial (e.g. root) objects.
    #[derive(Debug)]
    enum Visibility {
        kHidden,
        kDependentVisibility,
        kVisible,
    }

    struct StateBase {
        key_: *const std::ffi::c_void,
        state_count_: usize,
        visibility_: Visibility,
        visibility_dependency_: *mut StateBase,
        node_: *mut Node,
        visited_: bool,
        pending_: bool,
    }

    impl StateBase {
        fn new(key: *const std::ffi::c_void, state_count: usize, visibility: Visibility, node: *mut Node, visited: bool) -> Self {
            assert_ne!(Visibility::kDependentVisibility, visibility);
            StateBase {
                key_: key,
                state_count_: state_count,
                visibility_: visibility,
                visibility_dependency_: ptr::null_mut(),
                node_: node,
                visited_: visited,
                pending_: false,
            }
        }
        fn is_visited(&self) -> bool {
            self.visited_
        }
        fn is_pending(&self) -> bool {
            self.pending_
        }
        fn is_visible_not_dependent(&self) -> bool {
            let v = self.get_visibility();
            assert_ne!(Visibility::kDependentVisibility, v);
            v == Visibility::kVisible
        }
        fn set_node(&mut self, node: *mut Node) {
            assert_eq!(Visibility::kVisible, self.get_visibility());
            assert!(self.node_.is_null());
            self.node_ = node;
        }
        fn get_node(&mut self) -> *mut Node {
            assert_eq!(Visibility::kVisible, self.get_visibility());
            self.node_
        }

        fn get_visibility(&self) -> Visibility {
            self.follow_dependencies();
            self.visibility_
        }

        fn follow_dependencies(&self) -> *const StateBase {
            unsafe{
                todo!()
            }
        }
    }

    impl Drop for StateBase {
        fn drop(&mut self) {}
    }

    struct State {
        base: StateBase,
        is_weak_container_: bool,
        was_visited_from_stack_: bool,
        ephemeron_keys_: HashSet<*const std::ffi::c_void>,
        ephemeron_edges_: HashSet<*const std::ffi::c_void>,
        eager_ephemeron_edges_: HashMap<*const std::ffi::c_void, cppgc::TraceCallback>,
    }

    impl State {
        fn new(header: &HeapObjectHeader, state_count: usize) -> Self {
            State {
                base: StateBase::new(header as *const HeapObjectHeader as *const std::ffi::c_void, state_count, Visibility::kHidden, ptr::null_mut(), false),
                is_weak_container_: false,
                was_visited_from_stack_: false,
                ephemeron_keys_: HashSet::new(),
                ephemeron_edges_: HashSet::new(),
                eager_ephemeron_edges_: HashMap::new(),
            }
        }

        fn header(&self) -> &HeapObjectHeader {
            unsafe { &*(self.base.key_ as *const HeapObjectHeader) }
        }

        fn mark_visited(&mut self) {
            self.base.visited_ = true;
        }

        fn mark_pending(&mut self) {
            self.base.pending_ = true;
        }
        fn unmark_pending(&mut self) {
            self.base.pending_ = false;
        }

        fn mark_visible(&mut self) {
            self.base.visibility_ = Visibility::kVisible;
            self.base.visibility_dependency_ = ptr::null_mut();
        }

        fn mark_dependent_visibility(&mut self, dependency: &mut StateBase) {
            todo!()
        }

        fn mark_as_weak_container(&mut self) {
            self.is_weak_container_ = true;
        }
        fn is_weak_container(&self) -> bool {
            self.is_weak_container_
        }

        fn mark_visited_from_stack(&mut self) {
            self.was_visited_from_stack_ = true;
        }
        fn was_visited_from_stack(&self) -> bool {
            self.was_visited_from_stack_
        }

        fn record_ephemeron_key(&mut self, key: &HeapObjectHeader) {
            self.ephemeron_keys_.insert(key as *const HeapObjectHeader as *const std::ffi::c_void);
        }

        fn add_ephemeron_edge(&mut self, value: &HeapObjectHeader) {
            self.ephemeron_edges_.insert(value as *const HeapObjectHeader as *const std::ffi::c_void);
        }

        fn add_eager_ephemeron_edge(&mut self, value: *const std::ffi::c_void, callback: cppgc::TraceCallback) {
            self.eager_ephemeron_edges_.insert((value, callback));
        }

        fn for_all_ephemeron_keys<Callback>(&self, callback: Callback)
        where
            Callback: Fn(*const HeapObjectHeader),
        {
            for value in &self.ephemeron_keys_ {
                unsafe {
                    callback(*(value as *const std::ffi::c_void as *const HeapObjectHeader));
                }
            }
        }

        fn for_all_ephemeron_edges<Callback>(&self, callback: Callback)
        where
            Callback: Fn(*const HeapObjectHeader),
        {
            for value in &self.ephemeron_edges_ {
                unsafe {
                    callback(*(value as *const std::ffi::c_void as *const HeapObjectHeader));
                }
            }
        }

        fn for_all_eager_ephemeron_edges<Callback>(&self, callback: Callback)
        where
            Callback: Fn(*const std::ffi::c_void, cppgc::TraceCallback),
        {
            for pair in &self.eager_ephemeron_edges_ {
                callback(pair.0, pair.1);
            }
        }
    }

    // Root states are similar to regular states with the difference that they are
    // always visible.
    struct RootState {
        base: StateBase,
    }

    impl RootState {
        fn new(node: *mut EmbedderRootNode, state_count: usize) -> Self {
            RootState {
                base: StateBase::new(node as *const EmbedderRootNode as *const std::ffi::c_void, state_count, Visibility::kVisible, node as *mut Node, true),
            }
        }
    }

    // Abstraction for storing states. Storage allows for creation and lookup of
    // different state objects.
    struct StateStorage {
        states_: HashMap<*const std::ffi::c_void, Box<StateBase>>,
        state_count_: usize,
    }

    impl StateStorage {
        fn new() -> Self {
            StateStorage {
                states_: HashMap::new(),
                state_count_: 0,
            }
        }

        fn state_exists(&self, key: *const std::ffi::c_void) -> bool {
            self.states_.contains_key(&key)
        }

        fn get_existing_state(&self, key: *const std::ffi::c_void) -> &StateBase {
            assert!(self.state_exists(key));
            self.states_.get(&key).unwrap()
        }

        fn get_existing_state_mut(&mut self, key: *const std::ffi::c_void) -> &mut StateBase {
            assert!(self.state_exists(key));
            self.states_.get_mut(&key).unwrap()
        }

        fn get_or_create_state(&mut self, header: &HeapObjectHeader) -> &mut State {
            let key = header as *const HeapObjectHeader as *const std::ffi::c_void;
            if !self.state_exists(key) {
                self.state_count_ += 1;
                let state = State::new(header, self.state_count_);
                self.states_.insert(key, Box::new(StateBase::new(key, self.state_count_, Visibility::kHidden, ptr::null_mut(), false)));
            }
            unsafe {
                &mut *(self.states_.get_mut(&key).unwrap().as_mut() as *mut StateBase as *mut State)
            }
        }

        fn create_root_state(&mut self, root_node: *mut EmbedderRootNode) -> &mut RootState {
            let key = root_node as *const EmbedderRootNode as *const std::ffi::c_void;
            assert!(!self.state_exists(key));
            self.state_count_ += 1;
            let root_state = RootState::new(root_node, self.state_count_);
            self.states_.insert(key, Box::new(StateBase::new(key, self.state_count_, Visibility::kVisible, root_node as *mut Node, true)));
            unsafe {
                &mut *(self.states_.get_mut(&key).unwrap().as_mut() as *mut StateBase as *mut RootState)
            }
        }

        fn for_all_states<Callback>(&mut self, callback: Callback)
        where
            Callback: FnMut(&mut StateBase),
        {
            for state in self.states_.values_mut() {
                callback(state.as_mut());
            }
        }
    }

    extern "C" {
        fn snprintf(buf: *mut std::ffi::c_char, n: usize, format: *const i8, ...) -> i32;
    }

    fn extract_embedder_data_backref(isolate: *mut Isolate, cpp_heap: &CppHeap, v8_value: v8::Local<v8::Data>) -> *mut std::ffi::c_void {
        todo!()
    }

    // The following implements a snapshotting algorithm for C++ objects that also
    // filters strongly-connected components (SCCs) of only "hidden" objects that
    // are not (transitively) referencing any non-hidden objects.
    //
    // C++ objects come in two versions.
    // a. Named objects that have been assigned a name through NameProvider.
    // b. Unnamed objects, that are potentially hidden if the build configuration
    //    requires Oilpan to hide such names. Hidden objects have their name
    //    set to NameProvider::kHiddenName.
    //
    // The main challenge for the algorithm is to avoid blowing up the final object
    // graph with hidden nodes that do not carry information. For that reason, the
    // algorithm filters SCCs of only hidden objects, e.g.:
    //   ... -> (object) -> (object) -> (hidden) -> (hidden)
    // In this case the (hidden) objects are filtered from the graph. The trickiest
    // part is maintaining visibility state for objects referencing other objects
    // that are currently being processed.
    //
    // Main algorithm idea (two passes):
    // 1. First pass marks all non-hidden objects and those that transitively reach
    //    non-hidden objects as visible. Details:
    //    - Iterate over all objects.
    //    - If object is non-hidden mark it as visible and also mark parent as
    //      visible if needed.
    //    - If object is hidden, traverse children as DFS to find non-hidden
    //      objects. Post-order process the objects and mark those objects as
    //      visible that have child nodes that are visible themselves.
    //    - Maintain an epoch counter (StateStorage::state_count_) to allow
    //      deferring the visibility decision to other objects in the same SCC. This
    //      is similar to the "lowlink" value in Tarjan's algorithm for SCC.
    //    - After the first pass it is guaranteed that all deferred visibility
    //      decisions can be resolved.
    // 2. Second pass adds nodes and edges for all visible objects.
    //    - Upon first checking the visibility state of an object, all deferred
    //      visibility states are resolved.
    //
    // For practical reasons, the recursion is transformed into an iteration. We do
    // do not use plain Tarjan's algorithm to avoid another pass over all nodes to
    // create SCCs.
    struct CppGraphBuilderImpl<'a> {
        cpp_heap_: &'a mut CppHeap,
        graph_: &'a mut EmbedderGraph,
        states_: StateStorage,
        workstack_: Vec<Box<WorkstackItemBase>>,
    }

    impl<'a> CppGraphBuilderImpl<'a> {
        fn new(cpp_heap: &'a mut CppHeap, graph: &'a mut EmbedderGraph) -> Self {
            CppGraphBuilderImpl {
                cpp_heap_: cpp_heap,
                graph_: graph,
                states_: StateStorage::new(),
                workstack_: Vec::new(),
            }
        }

        fn run(&mut self) {
            // Sweeping from a previous GC might still be running, in which case not all
            // pages have been returned to spaces yet.
            self.cpp_heap_.sweeper().FinishIfRunning();
            //let _no_gc = cppgc::subtle::DisallowGarbageCollectionScope::new(self.cpp_heap_.GetHeapHandle()); //TODO

            // First pass: Figure out which objects should be included in the graph -- see
            // class-level comment on CppGraphBuilder.
            let mut visitor = LiveObjectsForVisibilityIterator::new(self);
            visitor.traverse(self.cpp_heap_.raw_heap());

            // Second pass: Add graph nodes for objects that must be shown.
            let mut self_mut = self;
            self_mut.states_.for_all_states(|state_base| {
                let state = unsafe {&mut *(state_base as *mut StateBase as *mut State)};

                if !state.base.is_visible_not_dependent() {
                    //self.graph_.AddNativeSize(state.header().AllocatedSize());  //TODO
                    return;
                }

                // Emit no edges for the contents of the weak containers. For both, fully
                // weak and ephemeron containers, the contents should be retained from
                // somewhere else.
                if state.is_weak_container() {
                    return;
                }

                //let parent_scope = ParentScope::new(state);
                //let mut object_visitor = GraphBuildingVisitor::new(self_mut, parent_scope);
                todo!()
            });
            todo!()
        }

        fn visit_for_visibility(&mut self, parent: *mut State, header: &HeapObjectHeader) {
            let current = self.states_.get_or_create_state(header);

            if current.base.is_visited() {
                // Avoid traversing into already visited subgraphs and just update the state
                // based on a previous result.
                if !parent.is_null() {
                    //self.mark_dependent_visibility(unsafe{&mut *parent}, current); //TODO
                    todo!()
                }
                return;
            }

            current.mark_visited();
            if header.GetName().name_was_hidden {
                current.mark_pending();
                todo!()
            } else {
                // No need to mark/unmark pending as the node is immediately processed.
                current.mark_visible();
                // In case the names are visible, the graph is not traversed in this phase.
                // Explicitly trace one level to handle weak containers.
                todo!()
                if !parent.is_null() {
                    // Eagerly update a parent object as its visibility state is now fixed.
                    //unsafe {&mut *parent}.mark_visible(); //TODO
                }
            }
        }

        fn add_root_node(&mut self, name: *const char) -> *mut EmbedderRootNode{
            let node = EmbedderRootNode::new(name);
            self.graph_.AddNode(std::unique_ptr::<Node>::new(EmbedderNode::new(kNoNativeAddress, cppgc::internal::HeapObjectName{value: name, name_was_hidden:false}, 0))) as *mut EmbedderRootNode;
            todo!()
        }

        fn add_node(&mut self, header: &HeapObjectHeader) -> *mut EmbedderNode{
            let size = header.AllocatedSize();
            let node = self.graph_.AddNode(std::unique_ptr::<Node>::new(EmbedderNode::new(header as *const HeapObjectHeader as *const std::ffi::c_void, header.GetName(), size))) as *mut EmbedderNode;
            let node_size = unsafe {&*node}.SizeInBytes();
            if size > node_size {
                self.graph_.AddNativeSize(size - node_size);
            }
            node
        }
    }

    // Iterating live objects to mark them as visible if needed.
    struct LiveObjectsForVisibilityIterator<'a> {
        graph_builder_: &'a mut CppGraphBuilderImpl<'a>,
    }

    impl<'a> LiveObjectsForVisibilityIterator<'a> {
        fn new(graph_builder: &'a mut CppGraphBuilderImpl<'a>) -> Self {
            LiveObjectsForVisibilityIterator {
                graph_builder_: graph_builder,
            }
        }

        fn traverse(&mut self, raw_heap: i32){
            todo!()
        }
    }

    // This visitor can be used stand-alone to handle fully weak and ephemeron
    // containers or as part of the VisibilityVisitor that recursively traverses
    // the object graph.
    struct WeakVisitor<'a> {
        graph_builder_: &'a mut CppGraphBuilderImpl<'a>,
        current_weak_container_header_: *const HeapObjectHeader,
    }

    impl<'a> WeakVisitor<'a> {
        fn new(graph_builder: &'a mut CppGraphBuilderImpl<'a>) -> Self {
            WeakVisitor {
                graph_builder_: graph_builder,
                current_weak_container_header_: ptr::null(),
            }
        }
    }
}


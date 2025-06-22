// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add Rust-equivalent crates for V8 internal functionalities.
// This is a placeholder.
mod v8_internal {
    pub struct Isolate {}
}

// TODO: Add Rust-equivalent crates for cppgc functionalities.
// This is a placeholder.
mod cppgc {
    pub mod internal {
        pub struct HeapObjectName {
            pub value: *const i8,
            pub name_was_hidden: bool,
        }
        pub struct HeapObjectHeader {}
    }
    pub struct TraceDescriptor {
        pub base_object_payload: *const u8,
        pub callback: TraceCallback,
    }
    pub type TraceCallback = extern "C" fn(&dyn Visitor, *const std::ffi::c_void);
    pub trait Visitor {
        // Define required methods here
    }
    pub struct SourceLocation {}
    impl SourceLocation {
      pub fn to_string(&self) -> String {
        String::new()
      }
    }
}

// TODO: Add Rust-equivalent crates for heap functionalities.
// This is a placeholder.
mod heap {
    pub mod base {
        pub trait StackVisitor {
          fn visit_pointer(&mut self, address: *const std::ffi::c_void);
        }
    }
}

// TODO: Add Rust-equivalent crates for v8-profiler functionalities.
// This is a placeholder.
mod v8_profiler {
  pub trait HeapProfiler {
    fn has_get_detachedness_callback(&self) -> bool;
    fn get_detachedness(&self, value: &v8::Value, index: usize) -> Detachedness;
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum Detachedness {
      kUnknown,
      kDetached,
      kNotDetached
  }
}

// TODO: Add Rust-equivalent crates for v8 api functionalities.
// This is a placeholder.
mod v8 {
    use std::any::Any;
    pub struct Value {}
    impl Value {
        pub fn is_object(&self) -> bool {
            false
        }
    }
    pub struct Data {}
    impl Data {
        pub fn is_empty(&self) -> bool {
            true
        }
        pub fn is_value(&self) -> bool {
            false
        }
        pub fn as_value(&self) -> &Value {
            unimplemented!()
        }
    }

    pub struct Isolate {}
    pub trait EmbedderGraph {
        type Node;
        fn add_node(&mut self, node: Box<Self::Node>) -> *mut Self::Node;
        fn add_edge(&mut self, from: *mut Self::Node, to: *mut Self::Node);
        fn add_edge_with_name(&mut self, from: *mut Self::Node, to: *mut Self::Node, name: &str);
        fn v8_node(&mut self, data: &Data) -> *mut Self::Node;
        fn add_native_size(&mut self, size: usize);
    }
}

// TODO: Add Rust-equivalent crates for base functionalities.
// This is a placeholder.
mod base {
    pub fn log(message: &str) {
        println!("{}", message);
    }
}

// TODO: Add Rust-equivalent crates for execution functionalities.
// This is a placeholder.
mod execution {
    pub struct Isolate {}
}

// TODO: Add Rust-equivalent crates for heap cppgc functionalities.
// This is a placeholder.
mod heap_cppgc {
    pub struct HeapObjectHeader {}
}

// TODO: Add Rust-equivalent crates for objects functionalities.
// This is a placeholder.
mod objects {
    pub struct JSObject {}
}

// TODO: Add Rust-equivalent crates for mark_compact functionalities.
// This is a placeholder.
mod mark_compact {}

// TODO: Add Rust-equivalent crates for profiler functionalities.
// This is a placeholder.
mod profiler {}

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::c_char;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Detachedness {
    kUnknown,
    kDetached,
    kNotDetached
}

/// Node representing a C++ object on the heap.
struct EmbedderNode {
    header_address_: *const std::ffi::c_void,
    name_: *const i8,
    size_: usize,
    wrapper_node_: RefCell<Option<*mut dyn v8::EmbedderGraph::Node>>,
    detachedness_: RefCell<Detachedness>,
    named_edges_: RefCell<Vec<CString>>,
}

impl fmt::Debug for EmbedderNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmbedderNode")
         .field("header_address_", &self.header_address_)
         .field("size_", &self.size_)
         .field("wrapper_node_", &self.wrapper_node_.borrow())
         .field("detachedness_", &self.detachedness_.borrow())
         .finish()
    }
}

impl EmbedderNode {
    fn new(
        header_address: *const std::ffi::c_void,
        name: cppgc::internal::HeapObjectName,
        size: usize,
    ) -> Self {
        EmbedderNode {
            header_address_: header_address,
            name_: name.value,
            size_: if name.name_was_hidden { 0 } else { size },
            wrapper_node_: RefCell::new(None),
            detachedness_: RefCell::new(Detachedness::kUnknown),
            named_edges_: RefCell::new(Vec::new()),
        }
    }

    fn name(&self) -> *const i8 {
        self.name_
    }
    fn size_in_bytes(&self) -> usize {
        self.size_
    }

    fn set_wrapper_node(&self, wrapper_node: *mut dyn v8::EmbedderGraph::Node) {
        // An embedder node may only be merged with a single wrapper node, as
        // consumers of the graph may merge a node and its wrapper node.
        //
        // TODO(chromium:1218404): Add a DCHECK() to avoid overriding an already
        // set `wrapper_node_`. This can currently happen with global proxies that
        // are rewired (and still kept alive) after reloading a page, see
        // `AddEdge`. We accept overriding the wrapper node in such cases,
        // leading to a random merged node and separated nodes for all other
        // proxies.
        *self.wrapper_node_.borrow_mut() = Some(wrapper_node);
    }
    fn wrapper_node(&self) -> Option<*mut dyn v8::EmbedderGraph::Node> {
        *self.wrapper_node_.borrow()
    }

    fn set_detachedness(&self, detachedness: Detachedness) {
        *self.detachedness_.borrow_mut() = detachedness;
    }
    fn get_detachedness(&self) -> Detachedness {
        *self.detachedness_.borrow()
    }

    // Edge names are passed to V8 but are required to be held alive from the
    // embedder until the snapshot is compiled.
    fn internalize_edge_name(&self, edge_name: String) -> *const i8 {
        let c_string = CString::new(edge_name).unwrap();
        let ptr = c_string.as_ptr();
        self.named_edges_.borrow_mut().push(c_string);
        ptr
    }

    fn get_address(&self) -> *const std::ffi::c_void {
        self.header_address_
    }
}

// Assuming the existence of the `v8::EmbedderGraph::Node` trait.
trait V8EmbedderGraphNode {
    fn name(&self) -> *const i8;
    fn size_in_bytes(&self) -> usize;
    fn wrapper_node(&self) -> Option<*mut dyn v8::EmbedderGraph::Node>;
    fn set_wrapper_node(&self, wrapper_node: *mut dyn v8::EmbedderGraph::Node);
    fn get_detachedness(&self) -> Detachedness;
    fn set_detachedness(&self, detachedness: Detachedness);
    fn get_address(&self) -> *const std::ffi::c_void;
}

impl V8EmbedderGraphNode for EmbedderNode {
    fn name(&self) -> *const i8 {
        self.name()
    }
    fn size_in_bytes(&self) -> usize {
        self.size_in_bytes()
    }
    fn wrapper_node(&self) -> Option<*mut dyn v8::EmbedderGraph::Node> {
        self.wrapper_node()
    }
    fn set_wrapper_node(&self, wrapper_node: *mut dyn v8::EmbedderGraph::Node) {
        self.set_wrapper_node(wrapper_node)
    }
    fn get_detachedness(&self) -> Detachedness {
        self.get_detachedness()
    }
    fn set_detachedness(&self, detachedness: Detachedness) {
        self.set_detachedness(detachedness)
    }
    fn get_address(&self) -> *const std::ffi::c_void {
        self.get_address()
    }
}

const K_NO_NATIVE_ADDRESS: *const std::ffi::c_void = std::ptr::null();

/// Node representing an artificial root group, e.g., set of Persistent handles.
struct EmbedderRootNode {
    node: EmbedderNode,
}

impl EmbedderRootNode {
    fn new(name: &str) -> Self {
        let c_string = CString::new(name).unwrap();
        let name_ptr = c_string.as_ptr();
        EmbedderRootNode {
            node: EmbedderNode::new(
                K_NO_NATIVE_ADDRESS,
                cppgc::internal::HeapObjectName {
                    value: name_ptr,
                    name_was_hidden: false,
                },
                0,
            ),
        }
    }

    fn is_root_node(&self) -> bool {
        true
    }
}

impl V8EmbedderGraphNode for EmbedderRootNode {
    fn name(&self) -> *const i8 {
        self.node.name()
    }
    fn size_in_bytes(&self) -> usize {
        self.node.size_in_bytes()
    }
    fn wrapper_node(&self) -> Option<*mut dyn v8::EmbedderGraph::Node> {
        self.node.wrapper_node()
    }
    fn set_wrapper_node(&self, wrapper_node: *mut dyn v8::EmbedderGraph::Node) {
        self.node.set_wrapper_node(wrapper_node)
    }
    fn get_detachedness(&self) -> Detachedness {
        self.node.get_detachedness()
    }
    fn set_detachedness(&self, detachedness: Detachedness) {
        self.node.set_detachedness(detachedness)
    }
    fn get_address(&self) -> *const std::ffi::c_void {
        self.node.get_address()
    }
}

// Canonical state representing real and artificial (e.g. root) objects.
#[derive(Debug)]
struct StateBase {
    key_: *const std::ffi::c_void,
    state_count_: usize,
    visibility_: RefCell<Visibility>,
    visibility_dependency_: RefCell<Option<*mut StateBase>>,
    node_: RefCell<Option<*mut EmbedderNode>>,
    visited_: bool,
    pending_: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Visibility {
    kHidden,
    kDependentVisibility,
    kVisible,
}

impl StateBase {
    fn new(
        key: *const std::ffi::c_void,
        state_count: usize,
        visibility: Visibility,
        node: Option<*mut EmbedderNode>,
        visited: bool,
    ) -> Self {
        assert_ne!(visibility, Visibility::kDependentVisibility);
        StateBase {
            key_: key,
            state_count_: state_count,
            visibility_: RefCell::new(visibility),
            visibility_dependency_: RefCell::new(None),
            node_: RefCell::new(node),
            visited_: visited,
            pending_: false,
        }
    }

    // Visited objects have already been processed or are currently being
    // processed, see also IsPending() below.
    fn is_visited(&self) -> bool {
        self.visited_
    }

    // Pending objects are currently being processed as part of the same SCC.
    fn is_pending(&self) -> bool {
        self.pending_
    }

    fn is_visible_not_dependent(&self) -> bool {
        let v = self.get_visibility();
        assert_ne!(v, Visibility::kDependentVisibility);
        v == Visibility::kVisible
    }

    fn set_node(&self, node: *mut EmbedderNode) {
        assert_eq!(*self.visibility_.borrow(), Visibility::kVisible);
        assert!(self.node_.borrow().is_none());
        *self.node_.borrow_mut() = Some(node);
    }

    fn get_node(&self) -> Option<*mut EmbedderNode> {
        *self.node_.borrow()
    }

    fn get_visibility(&self) -> Visibility {
        self.follow_dependencies();
        *self.visibility_.borrow()
    }

    fn follow_dependencies(&self) -> *mut StateBase {
        if *self.visibility_.borrow() != Visibility::kDependentVisibility {
            assert!(self.visibility_dependency_.borrow().is_none());
            return self as *const Self as *mut Self;
        }
        let mut current = self as *const Self as *mut Self;
        let mut dependencies: Vec<*mut StateBase> = Vec::new();

        // This is unsafe code because it is dealing with raw pointers.
        // This could be converted to safe code by using Rc<RefCell<StateBase>> but doing so would require
        // large architectural changes.
        unsafe {
            while let Some(dependency) = (*current).visibility_dependency_.borrow().map(|ptr| ptr) {
                if dependency == current {
                    break;
                }
                assert_eq!(*(*current).visibility_.borrow(), Visibility::kDependentVisibility);
                dependencies.push(current);
                current = dependency;
            }
            let mut new_visibility = Visibility::kDependentVisibility;
            let mut new_visibility_dependency: Option<*mut StateBase> = (*current).visibility_dependency_.borrow().map(|ptr| ptr);
            if *(*current).visibility_.borrow() == Visibility::kVisible {
                new_visibility = Visibility::kVisible;
                new_visibility_dependency = None;
            } else if !(*current).is_pending() {
                assert!((*current).is_visited());
                // The object was not visible (above case). Having a dependency on itself
                // or null means no visible object was found.
                new_visibility = Visibility::kHidden;
                new_visibility_dependency = None;
            }
            *(*current).visibility_.borrow_mut() = new_visibility;
            *(*current).visibility_dependency_.borrow_mut() = new_visibility_dependency;

            for state in dependencies {
                *(*state).visibility_.borrow_mut() = new_visibility;
                *(*state).visibility_dependency_.borrow_mut() = new_visibility_dependency;
            }
        }

        current
    }
}

/// State representing a C++ heap object.
#[derive(Debug)]
struct State {
    base: StateBase,
    is_weak_container_: RefCell<bool>,
    was_visited_from_stack_: RefCell<bool>,
    ephemeron_keys_: RefCell<HashSet<*const cppgc::internal::HeapObjectHeader>>,
    ephemeron_edges_: RefCell<HashSet<*const cppgc::internal::HeapObjectHeader>>,
    eager_ephemeron_edges_: RefCell<HashMap<*const std::ffi::c_void, cppgc::TraceCallback>>,
}

impl State {
    fn new(header: &cppgc::internal::HeapObjectHeader, state_count: usize) -> Self {
        State {
            base: StateBase::new(
                header as *const cppgc::internal::HeapObjectHeader as *const std::ffi::c_void,
                state_count,
                Visibility::kHidden,
                None,
                false,
            ),
            is_weak_container_: RefCell::new(false),
            was_visited_from_stack_: RefCell::new(false),
            ephemeron_keys_: RefCell::new(HashSet::new()),
            ephemeron_edges_: RefCell::new(HashSet::new()),
            eager_ephemeron_edges_: RefCell::new(HashMap::new()),
        }
    }

    fn header(&self) -> *const cppgc::internal::HeapObjectHeader {
        self.base.key_ as *const cppgc::internal::HeapObjectHeader
    }

    fn mark_visited(&self) {
        self.base.visited_ = true;
    }

    fn mark_pending(&self) {
        self.base.pending_ = true;
    }
    fn unmark_pending(&self) {
        self.base.pending_ = false;
    }

    fn mark_visible(&self) {
        *self.base.visibility_.borrow_mut() = Visibility::kVisible;
        *self.base.visibility_dependency_.borrow_mut() = None;
    }

    fn mark_dependent_visibility(&self, dependency: *mut StateBase) {
        // This is unsafe code because it is dealing with raw pointers.
        // This could be converted to safe code by using Rc<RefCell<State>> but doing so would require
        // large architectural changes.
        unsafe {
            let dependency = (*dependency).follow_dependencies();
            assert!((*dependency).is_visited());

            if *self.base.visibility_.borrow() == Visibility::kVisible {
                // Already visible, no dependency needed.
                assert!(self.base.visibility_dependency_.borrow().is_none());
                return;
            }
            if *(*dependency).visibility_.borrow() == Visibility::kVisible {
                // Simple case: Dependency is visible.
                *self.base.visibility_.borrow_mut() = Visibility::kVisible;
                *self.base.visibility_dependency_.borrow_mut() = None;
                return;
            }

            if (self.base.visibility_dependency_.borrow().is_some() &&
                (*self.base.visibility_dependency_.borrow().unwrap()).state_count_ > (*dependency).state_count_) ||
                (self.base.visibility_dependency_.borrow().is_none() &&
                 self.base.state_count_ > (*dependency).state_count_) {
                // Only update when new state_count_ < original state_count_. This
                // ensures that we pick an ancestor as dependency and not a child which
                // is guaranteed to converge to an answer.
                //
                // Dependency is now
                // a) either pending with unknown visibility (same call chain), or
                // b) not pending and has defined visibility.
                //
                // It's not possible to point to a state that is not pending but has
                // dependent visibility because dependencies are updated to the top-most
                // dependency at the beginning of method.
                if (*dependency).is_pending() {
                    *self.base.visibility_.borrow_mut() = Visibility::kDependentVisibility;
                    *self.base.visibility_dependency_.borrow_mut() = Some(dependency);
                } else {
                    assert_ne!(*(*dependency).visibility_.borrow(), Visibility::kDependentVisibility);
                    if *(*dependency).visibility_.borrow() == Visibility::kVisible {
                        *self.base.visibility_.borrow_mut() = Visibility::kVisible;
                        *self.base.visibility_dependency_.borrow_mut() = None;
                    }
                }
            }
        }
    }

    fn mark_as_weak_container(&self) {
        *self.is_weak_container_.borrow_mut() = true;
    }
    fn is_weak_container(&self) -> bool {
        *self.is_weak_container_.borrow()
    }

    fn mark_visited_from_stack(&self) {
        *self.was_visited_from_stack_.borrow_mut() = true;
    }
    fn was_visited_from_stack(&self) -> bool {
        *self.was_visited_from_stack_.borrow()
    }

    fn record_ephemeron_key(&self, key: *const cppgc::internal::HeapObjectHeader) {
        // This ignores duplicate entries (in different containers) for the same
        // Key->Value pairs. Only one edge will be emitted in this case.
        self.ephemeron_keys_.borrow_mut().insert(key);
    }

    fn add_ephemeron_edge(&self, value: *const cppgc::internal::HeapObjectHeader) {
        // This ignores duplicate entries (in different containers) for the same
        // Key->Value pairs. Only one edge will be emitted in this case.
        self.ephemeron_edges_.borrow_mut().insert(value);
    }

    fn add_eager_ephemeron_edge(&self, value: *const std::ffi::c_void, callback: cppgc::TraceCallback) {
        self.eager_ephemeron_edges_.borrow_mut().insert((value, callback));
    }

    fn for_all_ephemeron_keys<Callback>(&self, mut callback: Callback)
    where
        Callback: FnMut(*const cppgc::internal::HeapObjectHeader),
    {
        for value in self.ephemeron_keys_.borrow().iter() {
            callback(*value);
        }
    }

    fn for_all_ephemeron_edges<Callback>(&self, mut callback: Callback)
    where
        Callback: FnMut(*const cppgc::internal::HeapObjectHeader),
    {
        for value in self.ephemeron_edges_.borrow().iter() {
            callback(*value);
        }
    }

    fn for_all_eager_ephemeron_edges<Callback>(&self, mut callback: Callback)
    where
        Callback: FnMut(*const std::ffi::c_void, cppgc::TraceCallback),
    {
        for (value, cb) in self.eager_ephemeron_edges_.borrow().iter() {
            callback(*value, *cb);
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
        // Root states are always visited, visible, and have a node attached.
        RootState {
            base: StateBase::new(
                node as *const EmbedderRootNode as *const std::ffi::c_void,
                state_count,
                Visibility::kVisible,
                Some(unsafe { &mut (*node).node }),
                true,
            ),
        }
    }
}

/// Abstraction for storing states. Storage allows for creation and lookup of
/// different state objects.
struct StateStorage {
    states_: RefCell<HashMap<*const std::ffi::c_void, Box<StateBase>>>,
    state_count_: AtomicUsize,
}

impl StateStorage {
    fn new() -> Self {
        StateStorage {
            states_: RefCell::new(HashMap::new()),
            state_count_: AtomicUsize::new(0),
        }
    }

    fn state_exists(&self, key: *const std::ffi::c_void) -> bool {
        self.states_.borrow().contains_key(&key)
    }

    fn get_existing_state(&self, key: *const std::ffi::c_void) -> &StateBase {
        assert!(self.state_exists(key));
        self.states_.borrow().get(&key).unwrap().as_ref()
    }

    fn get_existing_state_header(&self, header: &cppgc::internal::HeapObjectHeader) -> &State {
        let key = header as *const cppgc::internal::HeapObjectHeader as *const std::ffi::c_void;
        let state_base = self.get_existing_state(key);
        // convert state_base to state
        unsafe {
            &*(state_base as *const StateBase as *const State)
        }
    }

    fn get_or_create_state(&self, header: &cppgc::internal::HeapObjectHeader) -> &State {
        let key = header as *const cppgc::internal::HeapObjectHeader as *const std::ffi::c_void;
        if !self.state_exists(key) {
            let state_count = self.state_count_.fetch_add(1, Ordering::SeqCst) + 1;
            let state = State::new(header, state_count);
            self.states_.borrow_mut().insert(key, Box::new(state.base));
            // Need to return a reference. Must lookup again.
            return self.get_existing_state_header(header);
        }
        self.get_existing_state_header(header)
    }

    fn create_root_state(&self, root_node: *mut EmbedderRootNode) -> &RootState {
        assert!(!self.state_exists(root_node as *const EmbedderRootNode as *const std::ffi::c_void));
        let state_count = self.state_count_.fetch_add(1, Ordering::SeqCst) + 1;
        let root_state = RootState::new(root_node, state_count);
        let key = root_node as *const EmbedderRootNode as *const std::ffi::c_void;
        self.states_.borrow_mut().insert(key, Box::new(root_state.base));
        // Need to return a reference. Must lookup again.
        unsafe {
            &*(self.states_.borrow().get(&key).unwrap().as_ref() as *const StateBase as *const RootState)
        }
    }

    fn for_all_states<Callback>(&self, mut callback: Callback)
    where
        Callback: FnMut(&StateBase),
    {
        for state in self.states_.borrow().values() {
            callback(state.as_ref());
        }
    }
}

fn extract_embedder_data_backref(
    isolate: *mut v8_internal::Isolate,
    cpp_heap: &CppHeap,
    v8_value: &v8::Data,
) -> *mut std::ffi::c_void {
    // TODO: Implement this function.
    std::ptr::null_mut()
}

/// The following implements a snapshotting algorithm for C++ objects that also
/// filters strongly-connected components (SCCs) of only "hidden" objects that
/// are not (transitively) referencing any non-hidden objects.
///
/// C++ objects come in two versions.
/// a. Named objects that have been assigned a name through NameProvider.
/// b. Unnamed objects, that are potentially hidden if the build configuration
///    requires Oilpan to hide such names. Hidden objects have their name
///    set to NameProvider::kHiddenName.
///
/// The main challenge for the algorithm is to avoid blowing up the final object
/// graph with hidden nodes that do not carry information. For that reason, the
/// algorithm filters SCCs of only hidden objects, e.g.:
///   ... -> (object) -> (object) -> (hidden) -> (hidden)
/// In this case the (hidden) objects are filtered from the graph. The trickiest
/// part is maintaining visibility state for objects referencing other objects
/// that are currently being processed.
///
/// Main algorithm idea (two passes):
/// 1. First pass marks all non-hidden objects and those that transitively reach
///    non-hidden objects as visible. Details:
///    - Iterate over all objects.
///    - If object is non-hidden mark it as visible and also mark parent as
///      visible if needed.
///    - If object is hidden, traverse children as DFS to find non-hidden
///      objects. Post-order process the objects and mark those objects as
///      visible that have child nodes that are visible themselves.
///    - Maintain an epoch counter (StateStorage::state_count_) to allow
///      deferring the visibility decision to other objects in the same SCC. This
///      is similar to the "lowlink" value in Tarjan's algorithm for SCC.
///    - After the first pass it is guaranteed that all deferred visibility
///      decisions can be resolved.
/// 2. Second pass adds nodes and edges for all visible objects.
///    - Upon first checking the visibility state of an object, all deferred
///      visibility states are resolved.
///
/// For practical reasons, the recursion is transformed into an iteration. We do
/// do not use plain Tarjan's algorithm to avoid another pass over all nodes to
/// create SCCs.
struct CppGraphBuilderImpl<'a, T: v8::EmbedderGraph> {
    cpp_heap_: &'a CppHeap,
    graph_: &'a mut T,
    states_: StateStorage,
    workstack_: RefCell<Vec<Box<dyn WorkstackItemBase>>>,
}

impl<'a, T: v8::EmbedderGraph> CppGraphBuilderImpl<'a, T> {
    fn new(cpp_heap: &'a CppHeap, graph: &'a mut T) -> Self {
        CppGraphBuilderImpl {
            cpp_heap_: cpp_heap,
            graph_: graph,
            states_: StateStorage::new(),
            workstack_: RefCell::new(Vec::new()),
        }
    }

    fn run(&self) {
        // Sweeping from a previous GC might still be running, in which case not all
        // pages have been returned to spaces yet.
        self.cpp_heap_.sweeper().finish_if_running();
        // let no_gc = cppgc::subtle::DisallowGarbageCollectionScope::new(self.cpp_heap_.get_heap_handle()); // TODO: Implement DisallowGarbageCollectionScope
        // First pass: Figure out which objects should be included in the graph -- see
        // class-level comment on CppGraphBuilder.
        let visitor = LiveObjectsForVisibilityIterator::new(self);
        // visitor.traverse(self.cpp_heap_.raw_heap()); // TODO: Implement traverse
                                                        // Second pass: Add graph nodes for objects that must be shown.
        self.states_.for_all_states(|state_base| {
            // No roots have been created so far, so all StateBase objects are State.
            let state = unsafe {
                &*(state_base as *const StateBase as *const State)
            };

            if !state.base.is_visible_not_dependent() {
                // self.graph_.add_native_size(state.header().allocated_size()); // TODO: Implement add_native_size
                return;
            }

            // Emit no edges for the contents of the weak containers. For both, fully
            // weak and ephemeron containers, the contents should be retained from
            // somewhere else.
            if state.is_weak_container() {
                return;
            }

            let parent_scope = ParentScope::new(&state.base);
            let object_visitor = GraphBuildingVisitor::new(self, &parent_scope);
            // if !state.header().is_in_construction() { // TODO: Implement is_in_construction
            //     // TODO(mlippautz): Handle in-construction objects.
            //     state.header().trace(&object_visitor); // TODO: Implement trace
            // }
            state.for_all_ephemeron_edges(|value| {
                self.add_edge_header(&state.base, value, "part of key -> value pair in ephemeron table".to_string());
            });
            // object_visitor.set_edge_name("part of key -> value pair in ephemeron table".to_string());
            state.for_all_eager_ephemeron_edges(|value, callback| {
                // callback(&object_visitor, value); // TODO: Implement callback
            });
        });
        // Add roots.
        {
            let parent_scope = ParentScope::new(&self.states_.create_root_state(self.add_root_node("C++ Persistent roots")));
            let root_object_visitor = GraphBuildingRootVisitor::new(self, &parent_scope);
            // self.cpp_heap_.get_strong_persistent_region().iterate(root_object_visitor); // TODO: Implement iterate
        }
        {
            let parent_scope = ParentScope::new(&self.states_.create_root_state(self.add_root_node("C++ CrossThreadPersistent roots")));
            let root_object_visitor = GraphBuildingRootVisitor::new(self, &parent_scope);
            // let guard = cppgc::internal::PersistentRegionLock::new(); // TODO: Implement PersistentRegionLock
            // self.cpp_heap_.get_strong_cross_thread_persistent_region().iterate(root_object_visitor); // TODO: Implement iterate
        }
        // Only add stack roots in case the callback is not run from generating a
        // snapshot without stack. This avoids adding false-positive edges when
        // conservatively scanning the stack.
        // if self.cpp_heap_.isolate().heap().is_gc_with_main_thread_stack() { // TODO: Implement is_gc_with_main_thread_stack
        //     let parent_scope = ParentScope::new(&self.states_.create_root_state(self.add_root_node("C++ native stack roots")));
        //     let root_object_visitor = GraphBuildingRootVisitor::new(self, &parent_scope);
        //     let stack_visitor = GraphBuildingStackVisitor::new(self, self.cpp_heap_, root_object_visitor);
        //     // self.cpp_heap
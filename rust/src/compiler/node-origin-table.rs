// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod node_origin_table {
    use std::{
        fmt,
        fmt::Display,
        i64,
        //io::Write,
    };

    // Placeholder for TFGraph and Node (since they're not defined in the header)
    pub struct TFGraph {}
    pub struct Node {
        id: NodeId,
    }
    impl Node {
        pub fn id(&self) -> NodeId {
            self.id
        }
    }

    pub type NodeId = i64;

    /// Represents the origin of a node.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NodeOrigin {
        phase_name: &'static str,
        reducer_name: &'static str,
        origin_kind: OriginKind,
        created_from: i64,
    }

    impl NodeOrigin {
        /// The kind of origin.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OriginKind {
            WasmBytecode,
            GraphNode,
            JSBytecode,
        }

        /// Creates a new NodeOrigin with GraphNode origin.
        pub fn new_graph_node(phase_name: &'static str, reducer_name: &'static str, created_from: NodeId) -> Self {
            NodeOrigin {
                phase_name,
                reducer_name,
                origin_kind: OriginKind::GraphNode,
                created_from,
            }
        }

        /// Creates a new NodeOrigin.
        pub fn new(phase_name: &'static str, reducer_name: &'static str, origin_kind: OriginKind, created_from: u64) -> Self {
            NodeOrigin {
                phase_name,
                reducer_name,
                origin_kind,
                created_from: created_from as i64,
            }
        }

        /// Creates an unknown NodeOrigin.
        pub fn unknown() -> Self {
            NodeOrigin {
                phase_name: "",
                reducer_name: "",
                origin_kind: OriginKind::GraphNode, // Default value
                created_from: i64::MIN,
            }
        }

        /// Checks if the NodeOrigin is known.
        pub fn is_known(&self) -> bool {
            self.created_from >= 0
        }

        /// Returns the id of the node this origin was created from.
        pub fn created_from(&self) -> i64 {
            self.created_from
        }

        /// Returns the name of the reducer.
        pub fn reducer_name(&self) -> &'static str {
            self.reducer_name
        }

        /// Returns the name of the phase.
        pub fn phase_name(&self) -> &'static str {
            self.phase_name
        }

        pub fn origin_kind(&self) -> OriginKind {
            self.origin_kind
        }

        // Missing PrintJson implementation, as it requires std::ostream which does not cleanly map to Rust

        /// Prints the NodeOrigin as JSON to the given formatter.
        pub fn print_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{{\"phase_name\": \"{}\", \"reducer_name\": \"{}\", \"origin_kind\": \"{:?}\", \"created_from\": {}}}",
                self.phase_name, self.reducer_name, self.origin_kind, self.created_from
            )
        }
    }

    impl Display for NodeOrigin {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.print_json(f)
        }
    }

    /// Table to store node origins.
    pub struct NodeOriginTable {
        graph_: *mut TFGraph, // raw pointer
        decorator_: *mut Decorator, //raw pointer
        current_origin_: NodeOrigin,
        current_bytecode_position_: i32,
        current_phase_name_: &'static str,
        table_: NodeAuxData<NodeOrigin>,
    }

    impl NodeOriginTable {
        /// Creates a new NodeOriginTable.
        pub fn new(graph: *mut TFGraph) -> Self {
            NodeOriginTable {
                graph_: graph,
                decorator_: std::ptr::null_mut(), //nullptr
                current_origin_: NodeOrigin::unknown(),
                current_bytecode_position_: 0,
                current_phase_name_: "",
                table_: NodeAuxData::new(),
            }
        }

        pub fn new_zone(zone: *mut Zone) -> Self {
            NodeOriginTable {
                graph_: std::ptr::null_mut(), //nullptr
                decorator_: std::ptr::null_mut(), //nullptr
                current_origin_: NodeOrigin::unknown(),
                current_bytecode_position_: 0,
                current_phase_name_: "",
                table_: NodeAuxData::new(),
            }
        }

        /// Adds a decorator.
        pub fn add_decorator(&mut self) {
            //TODO implement decorator
        }

        /// Removes a decorator.
        pub fn remove_decorator(&mut self) {
            //TODO implement decorator
        }

        /// Gets the NodeOrigin for a node.
        pub fn get_node_origin(&self, node: *mut Node) -> NodeOrigin {
            unsafe {
                if node.is_null() {
                    NodeOrigin::unknown()
                } else {
                    let node = &*node;
                    self.table_.get(node.id())
                }
            }
        }

        /// Gets the NodeOrigin for a node id.
        pub fn get_node_origin_by_id(&self, id: NodeId) -> NodeOrigin {
            self.table_.get(id)
        }

        /// Sets the NodeOrigin for a node.
        pub fn set_node_origin(&mut self, node: *mut Node, no: NodeOrigin) {
            unsafe {
                if !node.is_null() {
                    let node = &*node;
                    self.table_.set(node.id(), no);
                }
            }
        }

        /// Sets the NodeOrigin for a node id using another node id.
        pub fn set_node_origin_by_id(&mut self, id: NodeId, origin: NodeId) {
            //TODO: what to do with origin?
            self.table_.set(id, NodeOrigin::new_graph_node("", "", origin)); //TODO "" "" correct?
        }

        /// Sets the NodeOrigin for a node id.
        pub fn set_node_origin_with_kind(
            &mut self,
            id: NodeId,
            kind: NodeOrigin::OriginKind,
            origin: NodeId,
        ) {
            self.table_.set(id, NodeOrigin::new("", "", kind, origin as u64)); //TODO "" correct?
        }

        /// Sets the current position.
        pub fn set_current_position(&mut self, no: NodeOrigin) {
            self.current_origin_ = no;
        }

        /// Sets the current bytecode position.
        pub fn set_current_bytecode_position(&mut self, offset: i32) {
            self.current_bytecode_position_ = offset;
        }

        /// Gets the current bytecode position.
        pub fn get_current_bytecode_position(&self) -> i32 {
            self.current_bytecode_position_
        }

        // Missing PrintJson implementation, as it requires std::ostream which does not cleanly map to Rust
        /// Prints the NodeOriginTable as JSON to the given formatter.
        pub fn print_json(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "{{\"current_origin\": \"{}\"}}", self.current_origin_)
        }
    }

    //Missing Decorator
    struct Decorator {}

    impl Display for NodeOriginTable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.print_json(f)
        }
    }

    /// A scope for NodeOriginTable.
    pub struct Scope<'a> {
        origins_: *mut NodeOriginTable, //raw pointer
        prev_origin_: NodeOrigin,
        _marker: std::marker::PhantomData<&'a mut NodeOriginTable>,
    }

    impl<'a> Scope<'a> {
        /// Creates a new Scope.
        pub fn new(origins: *mut NodeOriginTable, reducer_name: &'static str, node: *mut Node) -> Self {
            let mut prev_origin_ = NodeOrigin::unknown();
            unsafe {
                if !origins.is_null() {
                    let origins_ref = &mut *origins;
                    prev_origin_ = origins_ref.current_origin_;
                    if !node.is_null() {
                         let node_ref = &*node;
                         origins_ref.current_origin_ = NodeOrigin::new_graph_node(origins_ref.current_phase_name_, reducer_name, node_ref.id());
                    }
                   
                }
            }
            Scope {
                origins_,
                prev_origin_,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<'a> Drop for Scope<'a> {
        fn drop(&mut self) {
            unsafe {
                if !self.origins_.is_null() {
                    let origins_ref = &mut *self.origins_;
                    origins_ref.current_origin_ = self.prev_origin_;
                }
            }
        }
    }

    /// A scope for phases in NodeOriginTable.
    pub struct PhaseScope<'a> {
        origins_: *mut NodeOriginTable, //raw pointer
        prev_phase_name_: &'static str,
        _marker: std::marker::PhantomData<&'a mut NodeOriginTable>,
    }

    impl<'a> PhaseScope<'a> {
        /// Creates a new PhaseScope.
        pub fn new(origins: *mut NodeOriginTable, phase_name: &'static str) -> Self {
            let mut prev_phase_name_: &'static str = "";
            unsafe {
                if !origins.is_null() {
                    let origins_ref = &mut *origins;
                    prev_phase_name_ = origins_ref.current_phase_name_;
                    origins_ref.current_phase_name_ = if phase_name.is_empty() { "unnamed" } else { phase_name };
                }
            }

            PhaseScope {
                origins_,
                prev_phase_name_,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<'a> Drop for PhaseScope<'a> {
        fn drop(&mut self) {
            unsafe {
                if !self.origins_.is_null() {
                    let origins_ref = &mut *self.origins_;
                    origins_ref.current_phase_name_ = self.prev_phase_name_;
                }
            }
        }
    }

    // Placeholder for Zone and ZoneObject
    pub struct Zone {}

    pub struct NodeAuxData<T> {
        data: std::collections::HashMap<NodeId, T>,
    }

    impl<T: Copy> NodeAuxData<T> {
        pub fn new() -> Self {
            NodeAuxData {
                data: std::collections::HashMap::new(),
            }
        }

        pub fn get(&self, id: NodeId) -> T
        where
            T: Copy,
        {
            *self.data.get(&id).unwrap_or_else(|| panic!("Unknown NodeId {}", id))
        }

        pub fn set(&mut self, id: NodeId, value: T) {
            self.data.insert(id, value);
        }
    }
}
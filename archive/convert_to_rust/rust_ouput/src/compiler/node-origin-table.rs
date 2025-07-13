// Converted from V8 C++ source files:
// Header: node-origin-table.h
// Implementation: node-origin-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod compiler_specific {
        #[macro_export]
        macro_rules! V8_NOEXCEPT {
            () => {
                #[inline(always)]
                fn noexcept(&self) {}
            };
        }
    }
}

pub mod compiler {
    use crate::base::compiler_specific::V8_NOEXCEPT;
    use std::fmt;
    use std::fmt::Display;
    use std::i64;
    use std::collections::HashMap;
    use std::cell::RefCell;

    pub struct NodeId(pub i64);

    impl Display for NodeId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub struct NodeAuxData<T, F> {
        data: RefCell<HashMap<NodeId, T>>,
        default_fn: F,
        zone: Zone,
    }

    impl<T: Clone, F: Fn(&Zone) -> T> NodeAuxData<T, F> {
        pub fn new(zone: Zone, default_fn: F) -> Self {
            NodeAuxData {
                data: RefCell::new(HashMap::new()),
                default_fn,
                zone,
            }
        }

        pub fn get(&self, node: &Node) -> T {
            self.get_by_id(node.id())
        }

        pub fn get_by_id(&self, id: NodeId) -> T {
            self.data.borrow().get(&id).cloned().unwrap_or_else(|| (self.default_fn)(&self.zone))
        }

        pub fn set(&self, node: &Node, value: T) {
            self.set_by_id(node.id(), value);
        }

        pub fn set_by_id(&self, id: NodeId, value: T) {
            self.data.borrow_mut().insert(id, value);
        }
    }

    pub struct TFGraph {
        zone: Zone,
    }

    impl TFGraph {
        pub fn new(zone: Zone) -> Self {
            TFGraph { zone }
        }
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
        pub fn AddDecorator(&mut self, decorator: *mut Decorator) {}
        pub fn RemoveDecorator(&mut self, decorator: *mut Decorator) {}
    }

    pub struct Node {
        id: NodeId,
    }

    impl Node {
        pub fn new(id: NodeId) -> Self {
            Node { id }
        }
        pub fn id(&self) -> NodeId {
            self.id
        }
    }

    pub struct Zone {
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }

    pub trait GraphDecorator {
        fn Decorate(&mut self, node: &Node);
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NodeOrigin {
        phase_name_: &'static str,
        reducer_name_: &'static str,
        origin_kind_: OriginKind,
        created_from_: i64,
    }

    impl NodeOrigin {
        pub fn new(phase_name: &'static str, reducer_name: &'static str, created_from: NodeId) -> Self {
            NodeOrigin {
                phase_name_: phase_name,
                reducer_name_: reducer_name,
                origin_kind_: OriginKind::kGraphNode,
                created_from_: created_from.0,
            }
        }

        pub fn new_with_kind(phase_name: &'static str, reducer_name: &'static str, origin_kind: OriginKind, created_from: u64) -> Self {
            NodeOrigin {
                phase_name_: phase_name,
                reducer_name_: reducer_name,
                origin_kind_: origin_kind,
                created_from_: created_from as i64,
            }
        }

        pub fn unknown() -> Self {
            NodeOrigin {
                phase_name_: "",
                reducer_name_: "",
                origin_kind_: OriginKind::kGraphNode,
                created_from_: i64::min_value(),
            }
        }

        pub fn is_known(&self) -> bool {
            self.created_from_ >= 0
        }

        pub fn created_from(&self) -> i64 {
            self.created_from_
        }

        pub fn reducer_name(&self) -> &'static str {
            self.reducer_name_
        }

        pub fn phase_name(&self) -> &'static str {
            self.phase_name_
        }

        pub fn origin_kind(&self) -> OriginKind {
            self.origin_kind_
        }
        pub fn PrintJson(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(out, "{{ ")?;
            match self.origin_kind_ {
                OriginKind::kWasmBytecode => {
                    write!(out, "\"bytecodePosition\" : ")?;
                }
                OriginKind::kGraphNode => {
                    write!(out, "\"nodeId\" : ")?;
                }
                OriginKind::kJSBytecode => {
                    write!(out, "\"bytecodePosition\" : ")?;
                }
            }
            write!(out, "{}", self.created_from())?;
            write!(out, ", \"reducer\" : \"{}\"", self.reducer_name())?;
            write!(out, ", \"phase\" : \"{}\"", self.phase_name())?;
            write!(out, "}}")?;
            Ok(())
        }
    }

    impl fmt::Debug for NodeOrigin {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NodeOrigin")
                .field("phase_name_", &self.phase_name_)
                .field("reducer_name_", &self.reducer_name_)
                .field("origin_kind_", &self.origin_kind_)
                .field("created_from_", &self.created_from_)
                .finish()
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum OriginKind {
        kWasmBytecode,
        kGraphNode,
        kJSBytecode,
    }

    pub struct NodeOriginTable {
        graph_: *mut TFGraph,
        decorator_: *mut Decorator,
        current_origin_: NodeOrigin,
        current_bytecode_position_: i32,
        current_phase_name_: &'static str,
        table_: NodeAuxData<NodeOrigin, fn(&Zone) -> NodeOrigin>,
        zone_: Zone,
    }

    impl NodeOriginTable {
        pub fn new(graph: *mut TFGraph) -> Self {
            let zone = unsafe { (*graph).zone().clone() };
            NodeOriginTable {
                graph_: graph,
                decorator_: std::ptr::null_mut(),
                current_origin_: NodeOrigin::unknown(),
                current_bytecode_position_: 0,
                current_phase_name_: "unknown",
                table_: NodeAuxData::new(zone, NodeOriginTable::unknown_node_origin),
                zone_: Zone::new(),
            }
        }

        pub fn new_with_zone(zone: Zone) -> Self {
            NodeOriginTable {
                graph_: std::ptr::null_mut(),
                decorator_: std::ptr::null_mut(),
                current_origin_: NodeOrigin::unknown(),
                current_bytecode_position_: 0,
                current_phase_name_: "unknown",
                table_: NodeAuxData::new(zone.clone(), NodeOriginTable::unknown_node_origin),
                zone_: zone,
            }
        }

        fn unknown_node_origin(zone: &Zone) -> NodeOrigin {
            NodeOrigin::unknown()
        }

        pub fn add_decorator(&mut self) {
             if self.graph_.is_null() {
                eprintln!("graph_ is null");
                return;
            }
            if !self.decorator_.is_null() {
                eprintln!("decorator_ is not null");
                return;
            }

            let decorator = Box::into_raw(Box::new(Decorator { origins_: self as *mut NodeOriginTable }));
            self.decorator_ = decorator;
            unsafe {
                if !self.graph_.is_null(){
                  (*self.graph_).AddDecorator(decorator);
                }
            }
        }

        pub fn remove_decorator(&mut self) {
            if self.graph_.is_null() {
                eprintln!("graph_ is null");
                return;
            }
            if self.decorator_.is_null() {
                eprintln!("decorator_ is null");
                return;
            }
            unsafe {
              if !self.graph_.is_null(){
                (*self.graph_).RemoveDecorator(self.decorator_);
              }
               let _ = Box::from_raw(self.decorator_); // deallocate
            }
            self.decorator_ = std::ptr::null_mut();
        }

        pub fn get_node_origin(&self, node: &Node) -> NodeOrigin {
            self.table_.get(node)
        }

        pub fn get_node_origin_by_id(&self, id: NodeId) -> NodeOrigin {
            self.table_.get_by_id(id)
        }

        pub fn set_node_origin(&self, node: &Node, no: NodeOrigin) {
            self.table_.set(node, no);
        }

        pub fn set_node_origin_by_id(&self, id: NodeId, origin: NodeId) {
            self.table_.set_by_id(id, NodeOrigin::new(self.current_phase_name_, "", origin));
        }

        pub fn set_node_origin_by_id_with_kind(&self, id: NodeId, kind: OriginKind, origin: NodeId) {
            self.table_.set_by_id(id, NodeOrigin::new_with_kind(self.current_phase_name_, "", kind, origin.0 as u64));
        }
        pub fn set_current_position(&mut self, no: NodeOrigin) {
            self.current_origin_ = no;
        }

        pub fn set_current_bytecode_position(&mut self, offset: i32) {
            self.current_bytecode_position_ = offset;
        }

        pub fn get_current_bytecode_position(&self) -> i32 {
            self.current_bytecode_position_
        }

         pub fn PrintJson(&self, os: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(os, "{{")?;
            let mut needs_comma = false;
            for (node_id, no) in self.table_.data.borrow().iter() {
                if no.is_known() {
                    if needs_comma {
                        write!(os, ",")?;
                    }
                    write!(os, "\"{}\": ", node_id)?;
                    no.PrintJson(os)?;
                    needs_comma = true;
                }
            }
            write!(os, "}}")?;
            Ok(())
        }
    }

    struct Decorator {
        origins_: *mut NodeOriginTable,
    }

    impl GraphDecorator for Decorator {
        fn Decorate(&mut self, node: &Node) {
            unsafe {
                if !self.origins_.is_null() {
                    let origins = &mut *self.origins_;
                    origins.set_node_origin(node, origins.current_origin_);
                }
            }
        }
    }
    pub struct Scope<'a> {
        origins_: Option<&'a mut NodeOriginTable>,
        prev_origin_: NodeOrigin,
    }

    impl<'a> Scope<'a> {
        pub fn new(origins: Option<&'a mut NodeOriginTable>, reducer_name: &'static str, node: &Node) -> Self {
            let mut prev_origin_ = NodeOrigin::unknown();
            if let Some(origins) = &mut origins.as_mut() {
                prev_origin_ = origins.current_origin_;
                origins.current_origin_ = NodeOrigin::new(origins.current_phase_name_, reducer_name, node.id());
            }

            Scope {
                origins_: origins.as_mut().map(|o| o),
                prev_origin_,
            }
        }
    }

    impl<'a> Drop for Scope<'a> {
        fn drop(&mut self) {
            if let Some(origins) = &mut self.origins_ {
                origins.current_origin_ = self.prev_origin_;
            }
        }
    }
    pub struct PhaseScope<'a> {
        origins_: Option<&'a mut NodeOriginTable>,
        prev_phase_name_: &'static str,
    }

    impl<'a> PhaseScope<'a> {
        pub fn new(origins: Option<&'a mut NodeOriginTable>, phase_name: &'static str) -> Self {
            let mut prev_phase_name_ = "unknown";
            if let Some(origins) = &mut origins.as_mut() {
                prev_phase_name_ = origins.current_phase_name_;
                origins.current_phase_name_ = if phase_name.is_null() { "unnamed" } else { phase_name };
            }

            PhaseScope {
                origins_: origins.as_mut().map(|o| o),
                prev_phase_name_,
            }
        }
    }

    impl<'a> Drop for PhaseScope<'a> {
        fn drop(&mut self) {
            if let Some(origins) = &mut self.origins_ {
                origins.current_phase_name_ = self.prev_phase_name_;
            }
        }
    }
}

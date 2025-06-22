// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod compiler_source_position_table {
    use std::collections::HashMap;
    use std::fmt;

    use crate::compiler::node_aux_data::NodeAuxData;
    use crate::compiler::turbofan_graph::TFGraph;
    use crate::compiler::node::Node;
    use crate::compiler::node::NodeId;
    use crate::compiler::graph_decorator::GraphDecorator;
    use crate::base::zone::Zone;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SourcePosition {
        line: i32,
        column: i32,
    }

    impl SourcePosition {
        pub fn new(line: i32, column: i32) -> Self {
            SourcePosition { line, column }
        }

        pub fn unknown() -> Self {
            SourcePosition { line: -1, column: -1 }
        }

        pub fn is_known(&self) -> bool {
            self.line >= 0 && self.column >= 0
        }

        pub fn print_json(&self, os: &mut dyn fmt::Write) -> fmt::Result {
            write!(os, "{{\"line\":{}, \"column\":{}}}", self.line, self.column)
        }
    }

    pub struct SourcePositionTable {
        graph_: *mut TFGraph, // Raw pointer to avoid ownership issues with TFGraph's Zone
        decorator_: Option<Box<Decorator>>,
        current_position_: SourcePosition,
        table_: NodeAuxData<SourcePosition>,
        enabled_: bool, // Added field to track enabled state
    }

    impl SourcePositionTable {
        pub fn new(graph: *mut TFGraph) -> Self {
            unsafe {
                let zone = (*graph).zone();
                SourcePositionTable {
                    graph_: graph,
                    decorator_: None,
                    current_position_: SourcePosition::unknown(),
                    table_: NodeAuxData::new(zone),
                    enabled_: true,
                }
            }
        }

        pub fn enable(&mut self) {
            self.enabled_ = true;
        }

        pub fn disable(&mut self) {
            self.enabled_ = false;
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled_
        }

        pub fn add_decorator(&mut self) {
            if !self.enabled_ {
                return;
            }

            if self.decorator_.is_none() {
                unsafe {
                    let zone = (*self.graph_).zone();
                    let decorator = Box::new(Decorator::new(self));
                    self.decorator_ = Some(decorator);

                    if let Some(decorator) = &self.decorator_ {
                        (*self.graph_).add_decorator(decorator.as_ref() as &dyn GraphDecorator);
                    }
                }
            }
        }

        pub fn remove_decorator(&mut self) {
            if !self.enabled_ {
                if self.decorator_.is_some() {
                    panic!("Decorator should be None when not enabled");
                }
                return;
            }
            if self.decorator_.is_none() {
                panic!("Decorator should not be None when removing");
            }
            unsafe {
                if let Some(decorator) = &self.decorator_ {
                    (*self.graph_).remove_decorator(decorator.as_ref() as &dyn GraphDecorator);
                }
            }
            self.decorator_ = None;
        }

        pub fn get_source_position(&self, node: *mut Node) -> SourcePosition {
            self.table_.get(node)
        }

        pub fn get_source_position_by_id(&self, id: NodeId) -> SourcePosition {
            self.table_.get_by_id(id)
        }

        pub fn set_source_position(&mut self, node: *mut Node, position: SourcePosition) {
            if self.is_enabled() {
                self.table_.set(node, position);
            }
        }

        pub fn print_json(&self, os: &mut dyn fmt::Write) -> fmt::Result {
            write!(os, "{{")?;
            let mut needs_comma = false;
            for (node_ptr, &pos) in self.table_.iter() {
                if pos.is_known() {
                    if needs_comma {
                        write!(os, ",")?;
                    }
                    unsafe {
                        write!(os, "\"{:?}\" : ", node_ptr)?;
                    }
                    pos.print_json(os)?;
                    needs_comma = true;
                }
            }
            write!(os, "}}")
        }

        pub fn set_current_position(&mut self, position: SourcePosition) {
            self.current_position_ = position;
        }

        pub fn current_position(&self) -> SourcePosition {
            self.current_position_
        }
    }

    pub struct Decorator {
        source_positions_: *mut SourcePositionTable,
    }

    impl Decorator {
        pub fn new(source_positions: *mut SourcePositionTable) -> Self {
            Decorator {
                source_positions_: source_positions,
            }
        }
    }

    impl GraphDecorator for Decorator {
        fn decorate(&self, node: *mut Node) {
            unsafe {
                (*self.source_positions_).set_source_position(
                    node,
                    (*self.source_positions_).current_position_,
                );
            }
        }
    }
}

pub mod compiler {
    pub mod node {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct NodeId {
            id: usize,
        }

        impl NodeId {
            pub fn new(id: usize) -> Self {
                NodeId { id }
            }

            pub fn invalid() -> Self {
                NodeId { id: 0 }
            }

            pub fn is_valid(&self) -> bool {
                self.id != 0
            }
        }

        #[derive(Debug)]
        pub struct Node {
            id: NodeId,
            // other fields
        }

        impl Node {
            pub fn new(id: NodeId) -> Self {
                Node { id }
            }
        }
    }

    pub mod turbofan_graph {
        use crate::base::zone::Zone;
        use crate::compiler::graph_decorator::GraphDecorator;
        use std::any::Any;

        pub struct TFGraph {
            zone_: Zone,
            decorators: Vec<Box<dyn GraphDecorator>>,
        }

        impl TFGraph {
            pub fn new(zone: Zone) -> Self {
                TFGraph {
                    zone_: zone,
                    decorators: Vec::new(),
                }
            }

            pub fn zone(&mut self) -> &mut Zone {
                &mut self.zone_
            }

            pub fn add_decorator(&mut self, decorator: &dyn GraphDecorator) {
                self.decorators.push(dyn_clone::clone_box(decorator));
            }

            pub fn remove_decorator(&mut self, decorator: &dyn GraphDecorator) {
                self.decorators.retain(|d| !eq(d.as_any(), decorator.as_any()));
            }
        }

        fn eq(a: &dyn Any, b: &dyn Any) -> bool {
             a.downcast_ref::<Box<dyn GraphDecorator>>().map_or(false, |a| {
                b.downcast_ref::<Box<dyn GraphDecorator>>().map_or(false, |b| {
                    a.type_id() == b.type_id() && (a as *const _ == b as *const _)
                })
             })
        }
    }

    pub mod node_aux_data {
        use std::collections::HashMap;
        use crate::base::zone::Zone;
        use crate::compiler::node::Node;
        use crate::compiler::node::NodeId;

        pub struct NodeAuxData<T> {
            table: HashMap<*mut Node, T>,
            zone: *mut Zone, // Zone lifetime management
        }

        impl<T: Copy> NodeAuxData<T> {
            pub fn new(zone: *mut Zone) -> Self {
                NodeAuxData {
                    table: HashMap::new(),
                    zone: zone,
                }
            }

            pub fn get(&self, node: *mut Node) -> T
                where T: Default + Copy
            {
                match self.table.get(&node) {
                    Some(&value) => value,
                    None => T::default(),
                }
            }

            pub fn get_by_id(&self, id: NodeId) -> T
                where T: Default + Copy
            {
                 //Since the table does not store NodeId's directly, you need to iterate and compare.
                 for (node_ptr, &value) in self.table.iter() {
                    unsafe {
                        if !node_ptr.is_null() && (*node_ptr).id == id {
                            return value;
                        }
                    }
                 }
                 T::default()
            }

            pub fn set(&mut self, node: *mut Node, value: T) {
                self.table.insert(node, value);
            }

            pub fn iter(&self) -> std::collections::hash_map::Iter<'_, *mut Node, T> {
                self.table.iter()
            }
        }

    }

    pub mod graph_decorator {
        use crate::compiler::node::Node;
        use std::any::Any;

        pub trait GraphDecorator {
            fn decorate(&self, node: *mut Node);

            fn as_any(&self) -> &dyn Any;
        }

        impl dyn GraphDecorator {
            pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
                self.as_any().downcast_ref::<T>()
            }
        }

        dyn_clone::clone_trait_object!(GraphDecorator);
    }
}

pub mod base {
    pub mod zone {
        pub struct Zone {
            // Simplified Zone for demonstration purposes
        }

        impl Zone {
            pub fn new() -> Self {
                Zone {}
            }

            pub fn allocate<T>(&mut self, value: T) -> Box<T> {
                Box::new(value)
            }

            // Additional allocation methods can be added as needed.
        }
    }
}
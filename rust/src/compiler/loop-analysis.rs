// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod loop_analysis {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;
    use std::ops::Range;

    pub const K_ASSUMED_LOOP_ENTRY_INDEX: usize = 0;

    pub struct LoopTree {
        zone: Rc<RefCell<Zone>>,
        outer_loops: Vec<Loop>,
        all_loops: Vec<Loop>,
        node_to_loop_num: Vec<i32>,
        loop_nodes: Vec<*mut Node>,
    }

    impl LoopTree {
        pub fn new(num_nodes: usize, zone: Rc<RefCell<Zone>>) -> Self {
            LoopTree {
                zone,
                outer_loops: Vec::new(),
                all_loops: Vec::new(),
                node_to_loop_num: vec![-1; num_nodes],
                loop_nodes: Vec::new(),
            }
        }

        pub fn containing_loop(&self, node: *mut Node) -> Option<&Loop> {
            unsafe {
                if (*node).id() >= self.node_to_loop_num.len() {
                    return None;
                }
                let num = self.node_to_loop_num[(*node).id()];
                if num > 0 {
                    Some(&self.all_loops[(num - 1) as usize])
                } else {
                    None
                }
            }
        }

        pub fn contains(&self, loop_: &Loop, node: *mut Node) -> bool {
            let mut c = self.containing_loop(node);
            while let Some(current_loop) = c {
                if current_loop as *const Loop == loop_ as *const Loop {
                    return true;
                }
                c = current_loop.parent();
            }
            false
        }

        pub fn outer_loops(&self) -> &Vec<Loop> {
            &self.outer_loops
        }

        pub fn inner_loops(&self) -> Vec<&Loop> {
            let mut inner_loops = Vec::new();
            for loop_ in &self.all_loops {
                if loop_.children().is_empty() {
                    inner_loops.push(loop_);
                }
            }
            inner_loops
        }

        pub fn loop_num(&self, loop_: &Loop) -> usize {
            1 + (loop_ as *const Loop as usize - &self.all_loops[0] as *const Loop as usize) / std::mem::size_of::<Loop>()
        }

        pub fn header_nodes(&self, loop_: &Loop) -> Range<usize> {
            loop_.header_start_ as usize..loop_.body_start_ as usize
        }

        pub fn header_node(&self, loop_: &Loop) -> *mut Node {
            for i in self.header_nodes(loop_) {
                let node = self.loop_nodes[i];
                unsafe {
                    if (*node).opcode() == IrOpcode::kLoop {
                        return node;
                    }
                }
            }
            unreachable!()
        }

        pub fn body_nodes(&self, loop_: &Loop) -> Range<usize> {
            loop_.body_start_ as usize..loop_.exits_start_ as usize
        }

        pub fn exit_nodes(&self, loop_: &Loop) -> Range<usize> {
            loop_.exits_start_ as usize..loop_.exits_end_ as usize
        }

        pub fn loop_nodes_range(&self, loop_: &Loop) -> Range<usize> {
            loop_.header_start_ as usize..loop_.exits_end_ as usize
        }

        pub fn get_loop_control(&self, loop_: &Loop) -> *mut Node {
            for i in self.header_nodes(loop_) {
                let node = self.loop_nodes[i];
                unsafe {
                    if (*node).opcode() == IrOpcode::kLoop {
                        return node;
                    }
                }
            }
            unreachable!()
        }

        pub fn zone(&self) -> Rc<RefCell<Zone>> {
            self.zone.clone()
        }

        fn new_loop(&mut self) -> &mut Loop {
            self.all_loops.push(Loop::new(self.zone.clone()));
            self.all_loops.last_mut().unwrap()
        }

        fn set_parent(&mut self, parent: Option<&mut Loop>, child: &mut Loop) {
            if let Some(p) = parent {
                p.children_.push(child);
                child.parent_ = Some(p);
                child.depth_ = p.depth_ + 1;
            } else {
                self.outer_loops.push(child);
            }
        }
    }

    #[derive(Default)]
    pub struct Loop {
        parent_: Option<&'static mut Loop>,
        depth_: i32,
        children_: Vec<&'static mut Loop>,
        header_start_: i32,
        body_start_: i32,
        exits_start_: i32,
        exits_end_: i32,
        zone: Rc<RefCell<Zone>>,
    }

    impl Loop {
        fn new(zone: Rc<RefCell<Zone>>) -> Self {
            Loop {
                parent_: None,
                depth_: 0,
                children_: Vec::new(),
                header_start_: -1,
                body_start_: -1,
                exits_start_: -1,
                exits_end_: -1,
                zone,
            }
        }

        pub fn parent(&self) -> Option<&Loop> {
            unsafe{
                self.parent_.map(|l| &*l)
            }
        }

        pub fn children(&self) -> &Vec<&Loop> {
            unsafe{
                &(*(self.children_.as_ptr() as *const Vec<&Loop>))
            }
        }

        pub fn header_size(&self) -> i32 {
            self.body_start_ - self.header_start_
        }

        pub fn body_size(&self) -> i32 {
            self.exits_start_ - self.body_start_
        }

        pub fn exits_size(&self) -> i32 {
            self.exits_end_ - self.exits_start_
        }

        pub fn total_size(&self) -> i32 {
            self.exits_end_ - self.header_start_
        }

        pub fn depth(&self) -> i32 {
            self.depth_
        }
    }

    pub struct LoopFinder {}

    impl LoopFinder {
        //This requires TFGraph and TickCounter which are not implemented
        //pub fn build_loop_tree(graph: &TFGraph, tick_counter: &TickCounter, temp_zone: &Zone) -> LoopTree {}
        pub fn has_marked_exits(loop_tree: &LoopTree, loop_: &Loop) -> bool {
            false
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn find_small_innermost_loop_from_header(
            loop_header: *mut Node,
            all_nodes: &mut AllNodes,
            zone: &mut Zone,
            max_size: usize,
            purpose: Purpose,
        ) -> Option<HashSet<*mut Node>> {
            None
        }
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub enum Purpose {
        kLoopPeeling,
        kLoopUnrolling,
    }

    //Dummy implementations for types that were not translated.
    pub struct TFGraph {}
    pub struct TickCounter {}
    pub struct Zone {
        pub data: Vec<u8>
    }

    impl Zone {
        pub fn new() -> Self {
            Zone{ data: Vec::new() }
        }
    }

    pub struct Node {
        id_: usize,
        opcode_: IrOpcode,
    }

    impl Node {
        pub fn id(&self) -> usize {
            self.id_
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode_
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum IrOpcode {
        kLoop,
        kOther
    }

    pub struct AllNodes {}

    pub struct NodeVector {}

    // Copies a range of nodes any number of times.
    pub struct NodeCopier {
        node_map_: NodeMarker<usize>,
        copies_: *mut NodeVector,
        copy_count_: u32,
    }

    impl NodeCopier {
        pub fn new(graph: &TFGraph, max: u32, p: *mut NodeVector, copy_count: u32) -> Self {
            assert!(copy_count > 0);
            NodeCopier {
                node_map_: NodeMarker::new(max as usize),
                copies_: p,
                copy_count_: copy_count,
            }
        }

        pub fn map(&self, node: *mut Node, copy_index: u32) -> *mut Node {
            if let Some(index) = self.node_map_.get(node) {
                unsafe {
                    // Assuming NodeVector is just a Vec<*mut Node>
                    let copies = &*(self.copies_ as *mut Vec<*mut Node>);
                    copies[*index as usize]
                }
            } else {
                node
            }
        }

        #[inline]
        pub fn map_single(&self, node: *mut Node) -> *mut Node {
            self.map(node, 0)
        }

        pub fn insert(&mut self, original: *mut Node, new_copies: &NodeVector) {
            unsafe{
                let copies = &mut *(self.copies_ as *mut Vec<*mut Node>);
                self.node_map_.set(original, copies.len() + 1);

                // Assuming NodeVector is just a Vec<*mut Node>
                //let copy_vec = &*(new_copies as *const NodeVector as *const Vec<*mut Node>);
                //copies.extend_from_slice(copy_vec.as_slice());
            }
        }

        pub fn insert_single(&mut self, original: *mut Node, copy: *mut Node) {
            unsafe {
                let copies = &mut *(self.copies_ as *mut Vec<*mut Node>);
                self.node_map_.set(original, copies.len() + 1);
                copies.push(copy);
            }
        }

        pub fn copy_nodes<InputIterator>(
            &mut self,
            graph: &mut TFGraph,
            tmp_zone_: &mut Zone,
            dead: *mut Node,
            nodes: &[InputIterator], //base::iterator_range<InputIterator> nodes,
            source_positions: &mut SourcePositionTable,
            node_origins: &mut NodeOriginTable,
        ) {
            // Copy all the nodes first.
            for original in nodes {
                //SourcePositionTable::Scope position(
                //    source_positions,
                //    source_positions.GetSourcePosition(original),
                //);
                //NodeOriginTable::Scope origin_scope(node_origins, "copy nodes", original);
                //self.node_map_.Set(original, self.copies_.size() + 1);
                //self.copies_.push_back(original);
                //for (uint32_t copy_index = 0; copy_index < copy_count_; copy_index++) {
                //    Node* copy = graph->CloneNode(original);
                //    self.copies_.push_back(copy);
                //}
            }

            // Fix inputs of the copies.
            //for (Node* original : nodes) {
            //    for (uint32_t copy_index = 0; copy_index < copy_count_; copy_index++) {
            //        Node* copy = map(original, copy_index);
            //        for (int i = 0; i < copy->InputCount(); i++) {
            //            copy->ReplaceInput(i, map(original->InputAt(i), copy_index));
            //        }
            //    }
            //}
        }

        pub fn marked(&self, node: *mut Node) -> bool {
            self.node_map_.get(node).is_some()
        }
    }

    struct NodeMarker<T> {
        markers: Vec<Option<T>>,
    }

    impl<T: Copy> NodeMarker<T> {
        pub fn new(max_nodes: usize) -> Self {
            NodeMarker {
                markers: vec![None; max_nodes],
            }
        }

        pub fn set(&mut self, node: *mut Node, value: T) {
            unsafe {
                let id = (*node).id();
                if id >= self.markers.len() {
                    self.markers.resize(id + 1, None);
                }
                self.markers[id] = Some(value);
            }
        }

        pub fn get(&self, node: *mut Node) -> Option<&T> {
            unsafe {
                let id = (*node).id();
                if id < self.markers.len() {
                    self.markers[id].as_ref()
                } else {
                    None
                }
            }
        }
    }

    pub struct SourcePositionTable {}
    impl SourcePositionTable {
        pub fn new() -> Self {
            SourcePositionTable{}
        }
        pub fn GetSourcePosition(&self, node: *mut Node) -> u32 {0}
        pub struct Scope<'a> {
            _table: &'a SourcePositionTable,
            _pos: u32,
        }
        pub fn Scope<'a>(_table: &'a SourcePositionTable, _pos: u32) -> Self::Scope<'a> {
            Self::Scope{_table, _pos}
        }
    }
    pub struct NodeOriginTable {}
    impl NodeOriginTable {
        pub fn new() -> Self {
            NodeOriginTable{}
        }
        pub struct Scope<'a> {
            _table: &'a NodeOriginTable,
            _desc: &'static str,
            _node: *mut Node,
        }
        pub fn Scope<'a>(_table: &'a NodeOriginTable, _desc: &'static str, _node: *mut Node) -> Self::Scope<'a> {
            Self::Scope{_table, _desc, _node}
        }
    }
}
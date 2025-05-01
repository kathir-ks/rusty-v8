// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod compiler {
    use std::fmt;
    use std::ops::{Deref, DerefMut};
    use std::vec::Vec;

    pub trait ZoneObject {}

    // Placeholder for AssemblerDebugInfo.  Since its definition is not available
    // we use an empty struct.
    #[derive(Debug, Copy, Clone, Default)]
    pub struct AssemblerDebugInfo {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Id {
        index: usize,
    }

    impl Id {
        pub fn to_int(&self) -> i32 {
            self.index as i32
        }
        pub fn to_size(&self) -> usize {
            self.index
        }
        pub fn from_size(index: usize) -> Self {
            Id { index }
        }
        pub fn from_int(index: i32) -> Self {
            Id {
                index: index as usize,
            }
        }
    }

    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.index)
        }
    }

    pub type NodeId = usize;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Control {
        None,
        Goto,
        Call,
        Branch,
        Switch,
        Deoptimize,
        TailCall,
        Return,
        Throw,
    }

    impl fmt::Display for Control {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Control::None => write!(f, "None"),
                Control::Goto => write!(f, "Goto"),
                Control::Call => write!(f, "Call"),
                Control::Branch => write!(f, "Branch"),
                Control::Switch => write!(f, "Switch"),
                Control::Deoptimize => write!(f, "Deoptimize"),
                Control::TailCall => write!(f, "TailCall"),
                Control::Return => write!(f, "Return"),
                Control::Throw => write!(f, "Throw"),
            }
        }
    }

    pub struct BasicBlock {
        id: Id,
        #[cfg(debug_assertions)]
        debug_info: AssemblerDebugInfo,
        predecessors: Vec<*mut BasicBlock>,
        successors: Vec<*mut BasicBlock>,
        nodes: Vec<NodeId>,
        control: Control,
        control_input: Option<NodeId>,
        deferred: bool,
        dominator_depth: i32,
        dominator: Option<*mut BasicBlock>,
        rpo_next: Option<*mut BasicBlock>,
        loop_header: Option<*mut BasicBlock>,
        loop_end: Option<*mut BasicBlock>,
        loop_depth: i32,
        loop_number: i32,
        rpo_number: i32,
        #[cfg(feature = "log_builtin_block_count")]
        pgo_execution_count: u64,
    }

    impl BasicBlock {
        pub fn new(id: Id) -> Self {
            BasicBlock {
                id,
                #[cfg(debug_assertions)]
                debug_info: AssemblerDebugInfo::default(),
                predecessors: Vec::new(),
                successors: Vec::new(),
                nodes: Vec::new(),
                control: Control::None,
                control_input: None,
                deferred: false,
                dominator_depth: 0,
                dominator: None,
                rpo_next: None,
                loop_header: None,
                loop_end: None,
                loop_depth: 0,
                loop_number: 0,
                rpo_number: 0,
                #[cfg(feature = "log_builtin_block_count")]
                pgo_execution_count: 0,
            }
        }

        pub fn id(&self) -> Id {
            self.id
        }

        #[cfg(debug_assertions)]
        pub fn set_debug_info(&mut self, debug_info: AssemblerDebugInfo) {
            self.debug_info = debug_info;
        }

        #[cfg(debug_assertions)]
        pub fn debug_info(&self) -> AssemblerDebugInfo {
            self.debug_info
        }

        pub fn print(&self) {
            println!("{}", self);
        }

        pub fn predecessors(&mut self) -> &mut Vec<*mut BasicBlock> {
            &mut self.predecessors
        }

        pub fn predecessors_const(&self) -> &Vec<*mut BasicBlock> {
            &self.predecessors
        }

        pub fn predecessor_count(&self) -> usize {
            self.predecessors.len()
        }

        pub fn predecessor_at(&self, index: usize) -> *mut BasicBlock {
            self.predecessors[index]
        }

        pub fn clear_predecessors(&mut self) {
            self.predecessors.clear();
        }

        pub fn add_predecessor(&mut self, predecessor: *mut BasicBlock) {
            self.predecessors.push(predecessor);
        }

        pub fn remove_predecessor(&mut self, index: usize) {
            self.predecessors.remove(index);
        }

        pub fn successors(&mut self) -> &mut Vec<*mut BasicBlock> {
            &mut self.successors
        }

        pub fn successors_const(&self) -> &Vec<*mut BasicBlock> {
            &self.successors
        }

        pub fn successor_count(&self) -> usize {
            self.successors.len()
        }

        pub fn successor_at(&self, index: usize) -> *mut BasicBlock {
            self.successors[index]
        }

        pub fn clear_successors(&mut self) {
            self.successors.clear();
        }

        pub fn add_successor(&mut self, successor: *mut BasicBlock) {
            self.successors.push(successor);
        }

        pub fn empty(&self) -> bool {
            self.nodes.is_empty()
        }

        pub fn size(&self) -> usize {
            self.nodes.len()
        }

        pub fn node_at(&self, index: usize) -> NodeId {
            self.nodes[index]
        }

        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }

        pub fn front(&self) -> &NodeId {
            &self.nodes.first().unwrap()
        }

        pub fn front_mut(&mut self) -> &mut NodeId {
            self.nodes.first_mut().unwrap()
        }

        pub fn remove_node(&mut self, it: usize) {
            self.nodes.remove(it);
        }

        pub fn add_node(&mut self, node: NodeId) {
            self.nodes.push(node);
        }

        pub fn insert_nodes<I>(&mut self, insertion_point: usize, insertion_start: I, insertion_end: I)
        where
            I: IntoIterator<Item = NodeId> + Clone,
        {
            let mut insertion = insertion_start.into_iter();
            let mut new_nodes = Vec::new();
            while let Some(node) = insertion.next() {
               new_nodes.push(node);
            }
            self.nodes.splice(insertion_point..insertion_point, new_nodes);
        }

        pub fn trim_nodes(&mut self, new_end: usize) {
            self.nodes.truncate(new_end);
        }

        pub fn reset_rpo_info(&mut self) {
            self.rpo_number = 0;
            self.rpo_next = None;
        }

        pub fn control(&self) -> Control {
            self.control
        }

        pub fn set_control(&mut self, control: Control) {
            self.control = control;
        }

        pub fn control_input(&self) -> Option<NodeId> {
            self.control_input
        }

        pub fn set_control_input(&mut self, control_input: Option<NodeId>) {
            self.control_input = control_input;
        }

        pub fn deferred(&self) -> bool {
            self.deferred
        }

        pub fn set_deferred(&mut self, deferred: bool) {
            self.deferred = deferred;
        }

        pub fn dominator_depth(&self) -> i32 {
            self.dominator_depth
        }

        pub fn set_dominator_depth(&mut self, depth: i32) {
            self.dominator_depth = depth;
        }

        pub fn dominator(&self) -> Option<*mut BasicBlock> {
            self.dominator
        }

        pub fn set_dominator(&mut self, dominator: Option<*mut BasicBlock>) {
            self.dominator = dominator;
        }

        pub fn rpo_next(&self) -> Option<*mut BasicBlock> {
            self.rpo_next
        }

        pub fn set_rpo_next(&mut self, rpo_next: Option<*mut BasicBlock>) {
            self.rpo_next = rpo_next;
        }

        pub fn loop_header(&self) -> Option<*mut BasicBlock> {
            self.loop_header
        }

        pub fn set_loop_header(&mut self, loop_header: Option<*mut BasicBlock>) {
            self.loop_header = loop_header;
        }

        pub fn loop_end(&self) -> Option<*mut BasicBlock> {
            self.loop_end
        }

        pub fn set_loop_end(&mut self, loop_end: Option<*mut BasicBlock>) {
            self.loop_end = loop_end;
        }

        pub fn loop_depth(&self) -> i32 {
            self.loop_depth
        }

        pub fn set_loop_depth(&mut self, loop_depth: i32) {
            self.loop_depth = loop_depth;
        }

        pub fn loop_number(&self) -> i32 {
            self.loop_number
        }

        pub fn set_loop_number(&mut self, loop_number: i32) {
            self.loop_number = loop_number;
        }

        pub fn rpo_number(&self) -> i32 {
            self.rpo_number
        }

        pub fn set_rpo_number(&mut self, rpo_number: i32) {
            self.rpo_number = rpo_number;
        }

        pub fn nodes(&mut self) -> &mut Vec<NodeId> {
            &mut self.nodes
        }

        #[cfg(feature = "log_builtin_block_count")]
        pub fn pgo_execution_count(&self) -> u64 {
            self.pgo_execution_count
        }

        #[cfg(feature = "log_builtin_block_count")]
        pub fn set_pgo_execution_count(&mut self, count: u64) {
            self.pgo_execution_count = count;
        }

        pub fn is_loop_header(&self) -> bool {
            self.loop_end.is_some()
        }

        pub fn loop_contains(&self, block: *mut BasicBlock) -> bool {
            // Placeholder implementation, as the logic depends on the dominator tree structure
            // which is not fully represented here.
            false
        }

        pub fn get_common_dominator(b1: *mut BasicBlock, b2: *mut BasicBlock) -> *mut BasicBlock {
            // Placeholder implementation. A real implementation would traverse the dominator
            // tree to find the common dominator.
            b1
        }
    }

    impl fmt::Display for BasicBlock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "BasicBlock {{ id: {}, control: {}, nodes: {:?} }}",
                self.id, self.control, self.nodes
            )
        }
    }

    pub struct Schedule {
        zone: Zone,
        all_blocks: Vec<*mut BasicBlock>,
        nodeid_to_block: Vec<Option<*mut BasicBlock>>, // map from node to containing block.
        rpo_order: Vec<*mut BasicBlock>,          // Reverse-post-order block list.
        start: *mut BasicBlock,
        end: *mut BasicBlock,
    }

    impl Schedule {
        pub fn new(zone: Zone, node_count_hint: usize) -> Self {
            Schedule {
                zone,
                all_blocks: Vec::new(),
                nodeid_to_block: vec![None; node_count_hint],
                rpo_order: Vec::new(),
                start: std::ptr::null_mut(),
                end: std::ptr::null_mut(),
            }
        }

        pub fn block(&self, node: NodeId) -> Option<*mut BasicBlock> {
            self.nodeid_to_block[node]
        }

        pub fn is_scheduled(&self, node: NodeId) -> bool {
            self.nodeid_to_block[node].is_some()
        }

        pub fn get_block_by_id(&self, block_id: Id) -> Option<*mut BasicBlock> {
            self.all_blocks
                .iter()
                .find(|&&block| unsafe { (*block).id() == block_id })
                .map(|&block| block)
        }

        pub fn clear_block_by_id(&mut self, block_id: Id) {
            for block in &mut self.all_blocks {
                if unsafe { (*(*block)).id() == block_id } {
                    *block = std::ptr::null_mut();
                }
            }
            self.all_blocks.retain(|&block| !block.is_null());
        }

        pub fn basic_block_count(&self) -> usize {
            self.all_blocks.len()
        }

        pub fn rpo_block_count(&self) -> usize {
            self.rpo_order.len()
        }

        pub fn same_basic_block(&self, a: NodeId, b: NodeId) -> bool {
            self.nodeid_to_block[a] == self.nodeid_to_block[b]
        }

        pub fn new_basic_block(&mut self) -> *mut BasicBlock {
            let id = Id::from_size(self.all_blocks.len());
            let block = Box::into_raw(Box::new(BasicBlock::new(id)));
            self.all_blocks.push(block);
            block
        }

        pub fn plan_node(&mut self, block: *mut BasicBlock, node: NodeId) {
            self.set_block_for_node(block, node);
        }

        pub fn add_node(&mut self, block: *mut BasicBlock, node: NodeId) {
            unsafe {
                (*block).add_node(node);
            }
            self.set_block_for_node(block, node);
        }

        pub fn add_goto(&mut self, block: *mut BasicBlock, succ: *mut BasicBlock) {
            unsafe {
                (*block).set_control(Control::Goto);
                self.add_successor(block, succ);
            }
        }

        pub fn add_call(
            &mut self,
            block: *mut BasicBlock,
            call: NodeId,
            success_block: *mut BasicBlock,
            exception_block: *mut BasicBlock,
        ) {
            unsafe {
                (*block).set_control(Control::Call);
                (*block).set_control_input(Some(call));
                self.add_successor(block, success_block);
                self.add_successor(block, exception_block);
            }
        }

        pub fn add_branch(
            &mut self,
            block: *mut BasicBlock,
            branch: NodeId,
            tblock: *mut BasicBlock,
            fblock: *mut BasicBlock,
        ) {
            unsafe {
                (*block).set_control(Control::Branch);
                (*block).set_control_input(Some(branch));
                self.add_successor(block, tblock);
                self.add_successor(block, fblock);
            }
        }

        pub fn add_switch(
            &mut self,
            block: *mut BasicBlock,
            sw: NodeId,
            succ_blocks: &mut [*mut BasicBlock],
            succ_count: usize,
        ) {
            unsafe {
                (*block).set_control(Control::Switch);
                (*block).set_control_input(Some(sw));
                for i in 0..succ_count {
                    self.add_successor(block, succ_blocks[i]);
                }
            }
        }

        pub fn add_deoptimize(&mut self, block: *mut BasicBlock, input: NodeId) {
            unsafe {
                (*block).set_control(Control::Deoptimize);
                (*block).set_control_input(Some(input));
            }
        }

        pub fn add_tail_call(&mut self, block: *mut BasicBlock, input: NodeId) {
            unsafe {
                (*block).set_control(Control::TailCall);
                (*block).set_control_input(Some(input));
            }
        }

        pub fn add_return(&mut self, block: *mut BasicBlock, input: NodeId) {
            unsafe {
                (*block).set_control(Control::Return);
                (*block).set_control_input(Some(input));
            }
        }

        pub fn add_throw(&mut self, block: *mut BasicBlock, input: NodeId) {
            unsafe {
                (*block).set_control(Control::Throw);
                (*block).set_control_input(Some(input));
            }
        }

        pub fn insert_branch(
            &mut self,
            block: *mut BasicBlock,
            end: *mut BasicBlock,
            branch: NodeId,
            tblock: *mut BasicBlock,
            fblock: *mut BasicBlock,
        ) {
            unsafe {
                (*block).set_control(Control::Branch);
                (*block).set_control_input(Some(branch));
                self.add_successor(block, tblock);
                self.add_successor(block, fblock);
            }
        }

        pub fn insert_switch(
            &mut self,
            block: *mut BasicBlock,
            end: *mut BasicBlock,
            sw: NodeId,
            succ_blocks: &mut [*mut BasicBlock],
            succ_count: usize,
        ) {
            unsafe {
                (*block).set_control(Control::Switch);
                (*block).set_control_input(Some(sw));
                for i in 0..succ_count {
                    self.add_successor(block, succ_blocks[i]);
                }
            }
        }

        pub fn add_successor_for_testing(&mut self, block: *mut BasicBlock, succ: *mut BasicBlock) {
            self.add_successor(block, succ);
        }

        pub fn all_blocks(&self) -> &Vec<*mut BasicBlock> {
            &self.all_blocks
        }

        pub fn rpo_order(&mut self) -> &mut Vec<*mut BasicBlock> {
            &mut self.rpo_order
        }

        pub fn rpo_order_const(&self) -> &Vec<*mut BasicBlock> {
            &self.rpo_order
        }

        pub fn start(&self) -> *mut BasicBlock {
            self.start
        }

        pub fn end(&self) -> *mut BasicBlock {
            self.end
        }

        pub fn zone(&self) -> &Zone {
            &self.zone
        }

        fn ensure_cfg_well_formedness(&mut self) {
            // Placeholder implementation
        }

        fn eliminate_redundant_phi_nodes(&mut self) {
            // Placeholder implementation
        }

        fn ensure_split_edge_form(&mut self, block: *mut BasicBlock) {
            // Placeholder implementation
        }

        fn move_phis(&mut self, from: *mut BasicBlock, to: *mut BasicBlock) {
            // Placeholder implementation
        }

        fn propagate_deferred_mark(&mut self) {
            // Placeholder implementation
        }

        fn add_successor(&mut self, block: *mut BasicBlock, succ: *mut BasicBlock) {
            unsafe {
                (*block).add_successor(succ);
            }
        }

        fn move_successors(&mut self, from: *mut BasicBlock, to: *mut BasicBlock) {
            // Placeholder implementation
        }

        fn set_control_input(&mut self, block: *mut BasicBlock, node: NodeId) {
            unsafe {
                (*block).set_control_input(Some(node));
            }
        }

        fn set_block_for_node(&mut self, block: *mut BasicBlock, node: NodeId) {
            if node >= self.nodeid_to_block.len() {
                self.nodeid_to_block.resize(node + 1, None);
            }
            self.nodeid_to_block[node] = Some(block);
        }
    }

    impl fmt::Display for Schedule {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Schedule {{ all_blocks: {:?}, rpo_order: {:?} }}",
                self.all_blocks, self.rpo_order
            )
        }
    }

    #[derive(Debug, Default)]
    pub struct Zone {} // Dummy Zone object.
}
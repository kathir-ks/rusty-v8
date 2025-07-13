// Converted from V8 C++ source files:
// Header: scheduler.h
// Implementation: scheduler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Flags<T> {
        flags: u32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Flags<T> {
        pub fn new(flags: u32) -> Self {
            Self {
                flags,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn empty() -> Self {
            Self::new(0)
        }

        pub fn contains(&self, flag: T) -> bool
        where
            T: Into<u32> + Copy,
        {
            (self.flags & flag.into()) != 0
        }

        pub fn insert(&mut self, flag: T)
        where
            T: Into<u32> + Copy,
        {
            self.flags |= flag.into();
        }

        pub fn remove(&mut self, flag: T)
        where
            T: Into<u32> + Copy,
        {
            self.flags &= !(flag.into());
        }
    }

    impl<T> std::ops::BitOr for Flags<T> {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            Self::new(self.flags | rhs.flags)
        }
    }

    impl<T> std::ops::BitAnd for Flags<T> {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self::new(self.flags & rhs.flags)
        }
    }

    impl<T> std::ops::BitOrAssign for Flags<T> {
        fn bitor_assign(&mut self, rhs: Self) {
            self.flags |= rhs.flags;
        }
    }
}

pub mod codegen {
    pub struct TickCounter {}

    impl TickCounter {
        pub fn tick_and_maybe_enter_safepoint(&mut self) {}
    }
}

pub mod utils {
    pub struct BitVector {
        bits: Vec<bool>,
    }

    impl BitVector {
        pub fn new(size: usize) -> Self {
            Self { bits: vec![false; size] }
        }

        pub fn resize(&mut self, size: usize) {
            self.bits.resize(size, false);
        }

        pub fn add(&mut self, index: usize) {
            if index < self.bits.len() {
                self.bits[index] = true;
            }
        }

        pub fn contains(&self, index: usize) -> bool {
            index < self.bits.len() && self.bits[index]
        }

        pub fn clear(&mut self) {
            self.bits.iter_mut().for_each(|b| *b = false);
        }
    }
}

pub mod zone {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub struct Zone {
        name: String,
    }

    impl Zone {
        pub fn new(name: String) -> Self {
            Zone { name }
        }

        pub fn allocate<T>(&self, value: T) -> Box<T> {
            Box::new(value)
        }

        pub fn allocate_rc<T>(&self, value: T) -> Rc<RefCell<T>> {
            Rc::new(RefCell::new(value))
        }

        pub fn allocate_slice<T>(&self, size: usize, default: T) -> Box<[T]>
        where
            T: Clone,
        {
            vec![default; size].into_boxed_slice()
        }
    }

    pub struct ZoneObject {}

    pub struct ZoneQueue<T> {
        queue: std::collections::VecDeque<T>,
        zone: Zone,
    }

    impl<T> ZoneQueue<T> {
        pub fn new(zone: &Zone) -> Self {
            ZoneQueue {
                queue: std::collections::VecDeque::new(),
                zone: zone.clone(),
            }
        }

        pub fn push(&mut self, value: T) {
            self.queue.push_back(value);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.queue.pop_front()
        }

        pub fn front(&self) -> &T {
            self.queue.front().unwrap()
        }

        pub fn is_empty(&self) -> bool {
            self.queue.is_empty()
        }
    }

    pub struct ZoneStack<T> {
        stack: Vec<T>,
        zone: Zone,
    }

    impl<T> ZoneStack<T> {
        pub fn new(zone: &Zone) -> Self {
            ZoneStack {
                stack: Vec::new(),
                zone: zone.clone(),
            }
        }

        pub fn push(&mut self, value: T) {
            self.stack.push(value);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.stack.pop()
        }

        pub fn top(&self) -> &T {
            self.stack.last().unwrap()
        }

        pub fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }
    }
}

pub mod builtins {
    pub struct ProfileDataFromFile {}

    impl ProfileDataFromFile {
        pub fn get_hint(&self, _arg0: usize, _arg1: usize) -> super::compiler::BranchHint {
            super::compiler::BranchHint::kNone
        }

        pub fn get_executed_count(&self, _arg0: usize) -> u64 {
            0
        }
    }
}

pub mod compiler {
    use super::base::Flags;
    use super::codegen::TickCounter;
    use super::utils::BitVector;
    use super::zone::{Zone, ZoneObject, ZoneQueue, ZoneStack};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IrOpcode {
        kStart,
        kEnd,
        kLoop,
        kMerge,
        kBranch,
        kSwitch,
        kReturn,
        kThrow,
        kDeoptimize,
        kTailCall,
        kParameter,
        kOsrValue,
        kPhi,
        kEffectPhi,
	    kFinishRegion,
	    kBeginRegion,
        kCall,
        kFastApiCall,
        kTerminate,
        // Add more opcodes as needed
    }

    impl IrOpcode {
        pub fn is_phi_opcode(self) -> bool {
            self == IrOpcode::kPhi || self == IrOpcode::kEffectPhi
        }

        pub fn is_merge_opcode(self) -> bool {
            self == IrOpcode::kMerge || self == IrOpcode::kLoop
        }
    }

    pub struct Operator {
        properties: i32,
    }

    impl Operator {
        pub const kPure: i32 = 1;

        pub fn has_property(&self, property: i32) -> bool {
            (self.properties & property) != 0
        }
        pub fn value_output_count(&self) -> i32 {
            0
        }
		pub fn effect_input_count(&self) -> i32 {
            0
        }
		pub fn effect_output_count(&self) -> i32 {
            0
        }
        pub fn control_output_count(&self) -> usize {
            0
        }
        pub fn value_outputs(&self) -> i32 {
            0
        }
    }

    pub struct Node {
        id: NodeId,
        opcode: IrOpcode,
        inputs: Vec<*mut Node>,
        uses: Vec<*mut Node>,
        op: Rc<Operator>,
    }

    impl Node {
        pub fn id(&self) -> NodeId {
            self.id
        }
        pub fn input_at(&self, index: usize) -> *mut Node {
            self.inputs[index]
        }
        pub fn inputs(&self) -> &Vec<*mut Node> {
            &self.inputs
        }
        pub fn uses(&self) -> &Vec<*mut Node> {
            &self.uses
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
        pub fn op(&self) -> &Operator {
            self.op.as_ref()
        }
        pub fn input_count(&self) -> usize {
            self.inputs.len()
        }
        pub fn new(id: NodeId, opcode: IrOpcode, inputs: Vec<*mut Node>, op: Rc<Operator>) -> Self {
            Node {
                id,
                opcode,
                inputs,
                uses: Vec::new(),
                op,
            }
        }

        pub fn input_edges(&self) -> Vec<Edge> {
            let mut edges = Vec::new();
            for (index, input) in self.inputs.iter().enumerate() {
                edges.push(Edge {
                    from: self,
                    to: unsafe { &**input },
                    index,
                });
            }
            edges
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct NodeId {
        id: usize,
    }

    impl NodeId {
        pub fn from_int(id: i32) -> Self {
            NodeId { id: id as usize }
        }

        pub fn to_int(&self) -> i32 {
            self.id as i32
        }

        pub fn to_size(&self) -> usize {
            self.id
        }
    }

    pub struct TFGraph {
        nodes: Vec<Node>,
		end_node: Node,
		start_node: Node,
        zone: Zone,
    }

    impl TFGraph {
        pub fn new(zone: Zone, end_node: Node, start_node: Node) -> Self {
            TFGraph {
                nodes: Vec::new(),
				end_node: end_node,
				start_node: start_node,
                zone,
            }
        }
		pub fn end(&mut self) -> *mut Node {
			&mut self.end_node
		}
		pub fn start(&mut self) -> *mut Node {
			&mut self.start_node
		}

        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }
		pub fn clone_node(&mut self, node: *mut Node) -> *mut Node {
			let node_ref = unsafe { &*node };
            let cloned_node = Node {
                id: NodeId::from_int(self.nodes.len() as i32),
                opcode: node_ref.opcode,
                inputs: node_ref.inputs.clone(),
                uses: Vec::new(),
                op: Rc::clone(&node_ref.op),
            };

            self.nodes.push(cloned_node);
            &mut self.nodes.last_mut().unwrap() as *mut Node
		}
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
    }

    pub struct Edge<'a> {
        pub from: &'a Node,
        pub to: &'a Node,
        pub index: usize,
    }

    impl <'a> Edge<'a> {
        pub fn update_to(&self, _arg0: *mut Node){

        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum BranchHint {
        kNone,
        kTrue,
        kFalse,
    }

    pub fn branch_hint_of(_arg0: *const Operator) -> BranchHint {
        BranchHint::kNone
    }

    pub struct BasicBlock {
        id: BasicBlockId,
        rpo_number: i32,
        loop_number: i32,
        loop_depth: i32,
        loop_header: *mut BasicBlock,
        loop_end: *mut BasicBlock,
        dominator: *mut BasicBlock,
        dominator_depth: i32,
        deferred: bool,
        predecessors: Vec<*mut BasicBlock>,
        successors: Vec<*mut BasicBlock>,
        nodes: Vec<*mut Node>,
    }

    impl BasicBlock {
        pub fn new(id: BasicBlockId) -> Self {
            BasicBlock {
                id,
                rpo_number: -1,
                loop_number: -1,
                loop_depth: 0,
                loop_header: std::ptr::null_mut(),
                loop_end: std::ptr::null_mut(),
                dominator: std::ptr::null_mut(),
                dominator_depth: 0,
                deferred: false,
                predecessors: Vec::new(),
                successors: Vec::new(),
                nodes: Vec::new(),
            }
        }

        pub fn id(&self) -> BasicBlockId {
            self.id
        }

        pub fn rpo_number(&self) -> i32 {
            self.rpo_number
        }

        pub fn set_rpo_number(&mut self, number: i32) {
            self.rpo_number = number;
        }

        pub fn loop_number(&self) -> i32 {
            self.loop_number
        }

        pub fn set_loop_number(&mut self, number: i32) {
            self.loop_number = number;
        }

        pub fn loop_depth(&self) -> i32 {
            self.loop_depth
        }

        pub fn set_loop_depth(&mut self, depth: i32) {
            self.loop_depth = depth;
        }

        pub fn loop_header(&self) -> *mut BasicBlock {
            self.loop_header
        }

        pub fn set_loop_header(&mut self, header: *mut BasicBlock) {
            self.loop_header = header;
        }

        pub fn loop_end(&self) -> *mut BasicBlock {
            self.loop_end
        }

        pub fn set_loop_end(&mut self, end: *mut BasicBlock) {
            self.loop_end = end;
        }

        pub fn dominator(&self) -> *mut BasicBlock {
            self.dominator
        }

        pub fn set_dominator(&mut self, dominator: *mut BasicBlock) {
            self.dominator = dominator;
        }

        pub fn dominator_depth(&self) -> i32 {
            self.dominator_depth
        }

        pub fn set_dominator_depth(&mut self, depth: i32) {
            self.dominator_depth = depth;
        }

        pub fn deferred(&self) -> bool {
            self.deferred
        }

        pub fn set_deferred(&mut self, deferred: bool) {
            self.deferred = deferred;
        }

        pub fn predecessors(&self) -> &Vec<*mut BasicBlock> {
            &self.predecessors
        }

        pub fn successors(&self) -> &Vec<*mut BasicBlock> {
            &self.successors
        }

        pub fn successor_at(&self, index: usize) -> *mut BasicBlock {
            self.successors[index]
        }

        pub fn predecessor_at(&self, index: usize) -> *mut BasicBlock {
            self.predecessors[index]
        }

        pub fn front(&self) -> *mut Node {
            if let Some(node) = self.nodes.first() {
                *node
            } else {
                std::ptr::null_mut()
            }
        }

        pub fn add_node(&mut self, node: *mut Node) {
            self.nodes.push(node);
        }

        pub fn is_loop_header(&self) -> bool {
            self.loop_number >= 0
        }

        pub fn loop_contains(&self, block: *mut BasicBlock) -> bool {
            let loop_end = self.loop_end();
            if loop_end.is_null() {
                return false;
            }

            let self_rpo = self.rpo_number();
            let block_ptr = unsafe { &*block };
            let block_rpo = block_ptr.rpo_number();
            let loop_end_ptr = unsafe { &*loop_end };
            let loop_end_rpo = loop_end_ptr.rpo_number();

            self_rpo <= block_rpo && block_rpo < loop_end_rpo
        }

        pub fn get_common_dominator(b1: *mut BasicBlock, b2: *mut BasicBlock) -> *mut BasicBlock {
            if b1.is_null() || b2.is_null() {
                return std::ptr::null_mut();
            }
        
            let mut current_b1 = unsafe { &*b1 };
            let mut current_b2 = unsafe { &*b2 };
        
            while current_b1.dominator_depth() > current_b2.dominator_depth() {
                if current_b1.dominator().is_null() {
                    return std::ptr::null_mut();
                }
                current_b1 = unsafe { &*current_b1.dominator() };
            }
        
            while current_b2.dominator_depth() > current_b1.dominator_depth() {
                if current_b2.dominator().is_null() {
                    return std::ptr::null_mut();
                }
                current_b2 = unsafe { &*current_b2.dominator() };
            }
        
            while !std::ptr::eq(current_b1 as *const BasicBlock, current_b2 as *const BasicBlock) {
                if current_b1.dominator().is_null() || current_b2.dominator().is_null() {
                    return std::ptr::null_mut();
                }
                current_b1 = unsafe { &*current_b1.dominator() };
                current_b2 = unsafe { &*current_b2.dominator() };
            }
        
            unsafe { &mut *(current_b1 as *const BasicBlock as *mut BasicBlock) }
        }

        pub fn predecessor_count(&self) -> usize {
            self.predecessors.len()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BasicBlockId {
        id: i32,
    }

    impl BasicBlockId {
        pub fn from_int(id: i32) -> Self {
            BasicBlockId { id }
        }
        pub fn to_int(&self) -> i32 {
            self.id
        }
    }
    
    pub type BasicBlockVector = Vec<*mut BasicBlock>;
    pub type NodeVector = Vec<*mut Node>;

    pub struct Schedule {
        start_block: BasicBlock,
        end_block: BasicBlock,
        blocks: Vec<BasicBlock>,
        rpo_order: BasicBlockVector,
        zone: Zone,
    }

    impl Schedule {
        pub fn new(zone: &Zone, node_count_hint: usize) -> Self {
            Schedule {
                start_block: BasicBlock::new(BasicBlockId::from_int(0)),
                end_block: BasicBlock::new(BasicBlockId::from_int(1)),
                blocks: Vec::new(),
                rpo_order: Vec::new(),
                zone: zone.clone(),
            }
        }
        pub fn start(&mut self) -> *mut BasicBlock {
			&mut self.start_block
		}
		pub fn end(&mut self) -> *mut BasicBlock {
			&mut self.end_block
		}

        pub fn basic_block_count(&self) -> usize {
            self.blocks.len() + 2 // account for start and end blocks
        }
        pub fn new_basic_block(&mut self) -> *mut BasicBlock {
            let id = BasicBlockId::from_int((self.blocks.len() + 2) as i32);
            let block = BasicBlock::new(id);
            self.blocks.push(block);
            &mut self.blocks.last_mut().unwrap() as *mut BasicBlock
        }
        pub fn block(&self, _arg0: *mut Node) -> *mut BasicBlock {
            std::ptr::null_mut()
        }
		pub fn set_block_for_node(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
        pub fn rpo_order(&mut self) -> &mut BasicBlockVector {
            &mut self.rpo_order
        }
        pub fn is_scheduled(&self, _arg0: *mut Node) -> bool {
            false
        }
        pub fn add_node(&mut self, block: *mut BasicBlock, node: *mut Node) {
            unsafe {
                (*block).add_node(node);
            }
        }
        pub fn plan_node(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
        pub fn get_block_by_id(&self, _arg0: BasicBlockId) -> *mut BasicBlock {
            std::ptr::null_mut()
        }
        pub fn all_blocks(&mut self) -> &mut Vec<BasicBlock> {
            &mut self.blocks
        }
		pub fn insert_branch(&mut self, _arg0: *mut BasicBlock, _arg1: *mut BasicBlock, _arg2: *mut Node, _arg3: *mut BasicBlock, _arg4: *mut BasicBlock){}
		pub fn add_branch(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node, _arg2: *mut BasicBlock, _arg3: *mut BasicBlock){}
		pub fn insert_switch(&mut self, _arg0: *mut BasicBlock, _arg1: *mut BasicBlock, _arg2: *mut Node, _arg3: *mut *mut BasicBlock, _arg4: usize){}
		pub fn add_switch(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node, _arg2: *mut *mut BasicBlock, _arg3: usize){}
		pub fn add_goto(&mut self, _arg0: *mut BasicBlock, _arg1: *mut BasicBlock){}
		pub fn add_tail_call(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
		pub fn add_return(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
		pub fn add_deoptimize(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
		pub fn add_throw(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node){}
		pub fn add_call(&mut self, _arg0: *mut BasicBlock, _arg1: *mut Node, _arg2: *mut BasicBlock, _arg3: *mut BasicBlock){}
    }

    pub struct ControlEquivalence {
        zone: Zone,
        graph: *mut TFGraph,
    }

    impl ControlEquivalence {
        pub fn new(zone: &Zone, graph: *mut TFGraph) -> Self {
            ControlEquivalence {
                zone: zone.clone(),
                graph,
            }
        }

        pub fn run(&mut self, _arg0: *mut Node) {}
        pub fn class_of(&self, _arg0: *mut Node) -> usize {
            0
        }
    }

    pub struct NodeProperties {}
    impl NodeProperties {
        pub fn get_control_input(_arg0: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn get_control_input_usize(_arg0: *mut Node, _arg1: usize) -> *mut Node {
            std::ptr::null_mut()
        }
		pub fn first_control_index(_arg0: *mut Node) -> std::option::Option<usize> {
			Some(0)
		}
		pub fn collect_control_projections(_arg0: *mut Node, _arg1: *mut *mut Node, _arg2: usize){

		}
		pub fn past_control_index(_arg0: *mut Node) -> usize {
			0
		}
        pub fn is_exceptional_call(_arg0: *mut Node) -> bool {
            false
        }
        pub fn is_phi(_arg0: *mut Node) -> bool {
            false
        }
    }

    pub struct NodeMarker<T> {
        values: Vec<T>,
        default: T,
    }

    impl<T: Copy> NodeMarker<T> {
        pub fn new(size: usize, default: T) -> Self {
            NodeMarker {
                values: vec![default; size],
                default,
            }
        }

        pub fn get(&self, node: *mut Node) -> T {
            let node_ptr = unsafe { &*node };
            self.values[node_ptr.id().to_int() as usize]
        }

        pub fn set(&mut self, node: *mut Node, value: T) {
            let node_ptr = unsafe { &*node };
            self.values[node_ptr.id().to_int() as usize] = value;
        }
    }

    pub struct Scheduler {
        zone: Zone,
        graph: *mut TFGraph,
        schedule: *mut Schedule,
        flags: Flags<SchedulerFlag>,
        scheduled_nodes: Vec<*mut NodeVector>,
        schedule_root_nodes: NodeVector,
        schedule_queue: ZoneQueue<*mut Node>,
        node_data: Vec<SchedulerData>,
        control_flow_builder: *mut ControlFlowBuilder,
        special_rpo: *mut SpecialRPONumberer,
        equivalence: *mut ControlEquivalence,
        tick_counter: *mut TickCounter,
        profile_data: *const super::builtins::ProfileDataFromFile,
        common_dominator_cache: HashMap<i32, HashMap<i32, *mut BasicBlock>>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum SchedulerFlag {
        kNoFlags = 0,
        kSplitNodes = 1 << 1,
        kTempSchedule = 1 << 2,
    }

    impl From<SchedulerFlag> for u32 {
        fn from(flag: SchedulerFlag) -> Self {
            flag as u32
        }
    }

    #[derive(Clone, Copy)]
    struct SchedulerData {
        minimum_block: *mut BasicBlock,
        unscheduled_count: i32,
        placement: Placement,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Placement {
        kUnknown,
        kSchedulable,
        kFixed,
        kCoupled,
        kScheduled,
    }

    impl Scheduler {
        fn new(
            zone: Zone,
            graph: *mut TFGraph,
            schedule: *mut Schedule,
            flags: Flags<SchedulerFlag>,
            node_count_hint: usize,
            tick_counter: *mut TickCounter,
            profile_data: *const super::builtins::ProfileDataFromFile,
        ) -> Self {
            let mut node_data = Vec::new();
            node_data.resize(unsafe { (*graph).node_count() }, Self::default_scheduler_data());
            Scheduler {
                zone: zone.clone(),
                graph,
                schedule,
                flags,
                scheduled_nodes: Vec::new(),
                schedule_root_nodes: Vec::new(),
                schedule_queue: ZoneQueue::new(&zone),
                node_data,
                control_flow_builder: std::ptr::null_mut(),
                special_rpo: std::ptr::null_mut(),
                equivalence: std::ptr::null_mut(),
                tick_counter,
                profile_data,
                common_dominator_cache: HashMap::new(),
            }
        }

        fn default_scheduler_data() -> SchedulerData {
            SchedulerData {
                minimum_block: std::ptr::null_mut(),
                unscheduled_count: 0,
                placement: Placement::kUnknown,
            }
        }

        fn get_data(&mut self, node: *mut Node) -> &mut SchedulerData {
            let node_ptr = unsafe { &*node };
            &mut self.node_data[node_ptr.id().to_int() as usize]
        }

        fn initialize_placement(&mut self, node: *mut Node) -> Placement {
            let data = self.get_data(node);
            if data.placement == Placement::kFixed {
                return data.placement;
            }
            assert_eq!(Placement::kUnknown, data.placement);
            let opcode = unsafe { (*node).opcode() };
            match opcode {
                IrOpcode::kParameter | IrOpcode::kOsrValue => {
                    data.placement = Placement::kFixed;
                }
                IrOpcode::kPhi | IrOpcode::kEffectPhi => {
                    let p = self.get_placement(unsafe { NodeProperties::get_control_input(node) });
                    data.placement = if p == Placement::kFixed {
                        Placement::kFixed
                    } else {
                        Placement::kCoupled
                    };
                }
                _ => {
                    data.placement = Placement::kSchedulable;
                }
            }
            data.placement
        }

        fn get_placement(&mut self, node: *mut Node) -> Placement {
            self.get_data(node).placement
        }

        fn update_placement(&mut self, node: *mut Node, placement: Placement) {
            let data = self.get_data(node);
            if data.placement == Placement::kUnknown {
                assert_eq!(Placement::kFixed, placement);
                data.placement = placement;
                return;
            }

            let opcode = unsafe { (*node).opcode() };
            match opcode {
                IrOpcode::kParameter => unreachable!(),
                IrOpcode::kPhi | IrOpcode::kEffectPhi => {
                    assert_eq!(Placement::kCoupled, data.placement);
                    assert_eq!(Placement::kFixed, placement);
                    let control = unsafe { NodeProperties::get_control_input(node) };
                    let block = unsafe { (*self.schedule).block(control) };
					unsafe {(*self.schedule).add_node(block,node)};
                }
                IrOpcode::kBranch | IrOpcode::kSwitch | IrOpcode::kDeoptimize | IrOpcode::kTailCall | IrOpcode::kReturn | IrOpcode::kThrow =>
                {
                    for use in unsafe {(*node).uses()} {
                        if self.get_placement(*use) == Placement::kCoupled {
                            assert_eq!(node, unsafe { NodeProperties::get_control_input(*use) });
                            self.update_placement(*use, placement);
                        }
                    }
                }
                _ => {
                    assert_eq!(Placement::kSchedulable, data.placement);
                    assert_eq!(Placement::kScheduled, placement);
                }
            }

            let coupled_control_edge = self.get_coupled_control_edge(node);
            for edge in unsafe { (*node).input_edges() } {
                assert_eq!(node, edge.from);
                if Some(edge.index) != coupled_control_edge {
                    self.decrement_unscheduled_use_count(edge.to as *const Node as *mut Node, node);
                }
            }
            data.placement = placement;
        }

        fn get_coupled_control_edge(&self, node: *mut Node) -> Option<usize> {
            if self.get_placement(node) == Placement::kCoupled {
				Some(unsafe { NodeProperties::first_control_index(node).unwrap() })
            } else {
                None
            }
        }

        fn increment_unscheduled_use_count(&mut self, node: *mut Node, from: *mut Node) {
            if self.get_placement(node) == Placement::kFixed {
                return;
            }

            let mut current_node = node;
            if self.get_placement(node) == Placement::kCoupled {
                current_node = unsafe { NodeProperties::get_control_input(node) };
                assert_ne!(self.get_placement(current_node), Placement::kFixed);
                assert_ne!(self.get_placement(current_node), Placement::kCoupled);
            }

            self.get_data(current_node).unscheduled_count += 1;
        }

        fn decrement_unscheduled_use_count(&mut self, node: *mut Node, from: *mut Node) {
            if self.get_placement(node) == Placement::kFixed {
                return;
            }

            let mut current_node = node;
            if self.get_placement(node) == Placement::kCoupled {
                current_node = unsafe { NodeProperties::get_control_input(node) };
                assert_ne!(self.get_placement(current_node), Placement::kFixed);
                assert_ne!(self.get_placement(current_node), Placement::kCoupled);
            }

            self.get_data(current_node).unscheduled_count -= 1;

            if self.get_data(current_node).unscheduled_count == 0 {
				self.schedule_queue.push(current_node);
            }
        }

        pub fn compute_schedule(
            zone: &Zone,
            graph: *mut TFGraph,
            flags: Flags<SchedulerFlag>,
            tick_counter: *mut TickCounter,
            profile_data: *

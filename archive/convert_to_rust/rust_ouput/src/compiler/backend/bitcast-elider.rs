// Converted from V8 C++ source files:
// Header: bitcast-elider.h
// Implementation: bitcast-elider.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
    pub use crate::v8::internal::compiler::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    pub struct TFGraph {}

    // Assuming Node is a struct representing a node in the graph.
    #[derive(Clone, Debug)]
    pub struct Node {
        id: usize,
        opcode: IrOpcode,
        inputs: Vec<Rc<RefCell<Node>>>,
        uses: Vec<Edge>, // Use edges to track dependents
        killed: bool,
    }

    impl Node {
        pub fn new(id: usize, opcode: IrOpcode) -> Self {
            Node {
                id,
                opcode,
                inputs: Vec::new(),
                uses: Vec::new(),
                killed: false,
            }
        }

        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }

        pub fn input_count(&self) -> usize {
            self.inputs.len()
        }

        pub fn input_at(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
            self.inputs.get(index).map(|node| node.clone())
        }

        pub fn add_input(&mut self, input: Rc<RefCell<Node>>) {
            self.inputs.push(input);
        }

        pub fn use_edges(&self) -> &Vec<Edge> {
            &self.uses
        }

        pub fn kill(&mut self) {
            self.killed = true;
        }

        pub fn is_killed(&self) -> bool {
            self.killed
        }
    }

    #[derive(Clone, Debug)]
    pub struct Edge {
        user: Rc<RefCell<Node>>,
        index: usize, // index of the input in the user node
    }

    impl Edge {
        pub fn new(user: Rc<RefCell<Node>>, index: usize) -> Self {
            Edge { user, index }
        }

        pub fn update_to(&self, replacement: Rc<RefCell<Node>>) {
            let mut user = self.user.borrow_mut();
            user.inputs[self.index] = replacement;
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum IrOpcode {
        Invalid,
        Parameter,
        Constant,
        BitcastTaggedToWordForTagAndSmiBits,
        BitcastWordToTaggedSigned,
        TruncateInt64ToInt32,
        Word32Equal,
        Int32LessThan,
        Int32LessThanOrEqual,
        Uint32LessThan,
        Uint32LessThanOrEqual,
        ChangeInt32ToInt64,
        // Add more opcodes as needed based on MACHINE_BINOP_32_LIST
        Word32And,
        Word32Or,
        Word32Xor,
        Word32Add,
        Word32Sub,
        Word32Mul,
        End,
    }

    // Assuming Zone is an allocator.  Using a simple Vec for now.
    pub struct Zone {
        nodes: RefCell<Vec<Rc<RefCell<Node>>>>,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {
                nodes: RefCell::new(Vec::new()),
            }
        }

        pub fn create_node(&self, opcode: IrOpcode) -> Rc<RefCell<Node>> {
            let mut nodes = self.nodes.borrow_mut();
            let id = nodes.len();
            let node = Rc::new(RefCell::new(Node::new(id, opcode)));
            nodes.push(node.clone());
            node
        }
    }

    // A simple queue using VecDeque
    pub struct ZoneQueue<T> {
        queue: RefCell<VecDeque<T>>,
    }

    impl<T> ZoneQueue<T> {
        pub fn new(_zone: &Zone) -> Self {
            ZoneQueue {
                queue: RefCell::new(VecDeque::new()),
            }
        }

        pub fn push(&self, value: T) {
            self.queue.borrow_mut().push_back(value);
        }

        pub fn pop(&self) -> Option<T> {
            self.queue.borrow_mut().pop_front()
        }

        pub fn is_empty(&self) -> bool {
            self.queue.borrow().is_empty()
        }
    }

    // A simple node marker using a HashMap
    pub struct NodeMarker<T: Copy> {
        graph: *const TFGraph, // Not used, but kept for consistency with C++
        markers: RefCell<std::collections::HashMap<usize, T>>,
        default_value: T,
    }

    impl<T: Copy> NodeMarker<T> {
        pub fn new(_graph: &TFGraph, default_value: T) -> Self {
            NodeMarker {
                graph: std::ptr::null(),
                markers: RefCell::new(std::collections::HashMap::new()),
                default_value,
            }
        }

        pub fn get(&self, node: &Node) -> T {
            *self
                .markers
                .borrow()
                .get(&node.id)
                .unwrap_or(&self.default_value)
        }

        pub fn set(&self, node: &Node, value: T) {
            self.markers.borrow_mut().insert(node.id, value);
        }
    }

    pub struct BitcastElider {
        graph_: *mut TFGraph, // Raw pointer kept for consistency
        to_visit_: ZoneQueue<*mut Node>,
        seen_: NodeMarker<bool>,
        is_builtin_: bool,
    }

    impl BitcastElider {
        pub fn new(zone: &Zone, graph: *mut TFGraph, is_builtin: bool) -> Self {
            BitcastElider {
                graph_: graph,
                to_visit_: ZoneQueue::new(zone),
                seen_: NodeMarker::new(&TFGraph {}, false),
                is_builtin_: is_builtin,
            }
        }

        fn is_bitcast(node: &Node) -> bool {
            node.opcode() == IrOpcode::BitcastTaggedToWordForTagAndSmiBits
                || node.opcode() == IrOpcode::BitcastWordToTaggedSigned
        }

        #[cfg(any(
            target_arch = "loong64",
            target_arch = "mips64",
            target_arch = "riscv64"
        ))]
        fn owned_by_word32_op(_node: &Node) -> bool {
            false
        }

        #[cfg(not(any(
            target_arch = "loong64",
            target_arch = "mips64",
            target_arch = "riscv64"
        )))]
        fn owned_by_word32_op(node: &Node) -> bool {
            for edge in node.use_edges() {
                let use_node = edge.user.borrow();
                match use_node.opcode() {
                    IrOpcode::Word32Equal
                    | IrOpcode::Int32LessThan
                    | IrOpcode::Int32LessThanOrEqual
                    | IrOpcode::Uint32LessThan
                    | IrOpcode::Uint32LessThanOrEqual
                    | IrOpcode::ChangeInt32ToInt64
                    | IrOpcode::Word32And
                    | IrOpcode::Word32Or
                    | IrOpcode::Word32Xor
                    | IrOpcode::Word32Add
                    | IrOpcode::Word32Sub
                    | IrOpcode::Word32Mul => {}
                    _ => return false,
                }
            }
            true
        }

        fn replace(node: &mut Node, replacement: Rc<RefCell<Node>>) {
            let use_edges = node.use_edges().clone();

            for edge in use_edges {
                edge.update_to(replacement.clone());
            }
            node.kill();
        }

        pub fn enqueue(&self, node: *mut Node) {
            unsafe {
                let node = &*node;
                if self.seen_.get(node) {
                    return;
                }
                self.seen_.set(node, true);
                self.to_visit_.push(node as *mut Node);
            }
        }

        pub fn revisit(&self, node: *mut Node) {
            self.to_visit_.push(node);
        }

        pub fn visit_node(&self, node_ptr: *mut Node) {
            unsafe {
                let node = &mut *node_ptr;
                for i in 0..node.input_count() {
                    if let Some(input_rc) = node.input_at(i) {
                        let mut input = input_rc.borrow_mut();
                        if input.opcode() == IrOpcode::TruncateInt64ToInt32
                            && Self::owned_by_word32_op(&input)
                        {
                            if let Some(first_input_rc) = input.input_at(0) {
                                let first_input = first_input_rc.borrow();
                                Self::replace(&mut input, first_input_rc.clone());
                                self.revisit(node_ptr);
                            }
                        } else if self.is_builtin_ && Self::is_bitcast(&input) {
                            if let Some(first_input_rc) = input.input_at(0) {
                                let first_input = first_input_rc.borrow();
                                Self::replace(&mut input, first_input_rc.clone());
                                self.revisit(node_ptr);
                            }
                        } else {
                            self.enqueue(&mut *input as *mut Node);
                        }
                    }
                }
            }
        }

        pub fn process_graph(&self) {
            // Assuming graph_->end() returns a *mut Node representing the end node.
            // We'll need a way to access the end node of the graph.
            // For now, let's assume we have a function get_end_node() that does this.
            unsafe {
                // Assuming graph_ is a valid pointer to a TFGraph object
                // and TFGraph has a method get_end_node() that returns *mut Node.
                // let end_node = (*self.graph_).get_end_node();

                //Creating dummy end node
                let end_node = &mut Node::new(0, IrOpcode::End) as *mut Node;

                self.enqueue(end_node);
                while !self.to_visit_.is_empty() {
                    if let Some(node_ptr) = self.to_visit_.pop() {
                        self.visit_node(node_ptr);
                    }
                }
            }
        }

        pub fn reduce(&self) {
            self.process_graph();
        }
    }
}

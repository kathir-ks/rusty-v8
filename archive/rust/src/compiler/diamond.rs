pub mod compiler {
    use std::rc::Rc;

    /// Representation of machine types for values in the graph.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MachineRepresentation {
        None,
        Bit,
        Word8,
        Word16,
        Word32,
        Word64,
        Float32,
        Float64,
        Simd128,
        Pointer,
        TaggedSigned,
        TaggedPointer,
        Tagged,
        ExternalPointer,
    }

    /// Hints for branch prediction.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BranchHint {
        kNone,
        kTrue,
        kFalse,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BranchSemantics {
        kUnspecified,
        kTouched,
        kUntouched
    }


    pub struct Node {
        id: usize, //dummy value, replace with actual node identifier if needed.
        operator: Operator,
        inputs: Vec<Rc<Node>>,
    }

    impl Node {
        pub fn new(operator: Operator, inputs: Vec<Rc<Node>>) -> Self {
            Node {
                id: 0,
                operator,
                inputs
            }
        }

        pub fn replace_input(&mut self, index: usize, new_input: Rc<Node>) {
            self.inputs[index] = new_input;
        }
    }

    pub struct TFGraph {
        nodes: Vec<Rc<Node>>,
        start_node: Rc<Node>,
        next_node_id: usize,
    }

    impl TFGraph {
        pub fn new() -> Self {
            let start_operator = Operator::Start;
            let start_node = Rc::new(Node::new(start_operator, Vec::new()));
            TFGraph {
                nodes: vec![start_node.clone()],
                start_node: start_node.clone(),
                next_node_id: 1,
            }
        }

        pub fn start(&self) -> Rc<Node> {
            self.start_node.clone()
        }

        pub fn new_node(&mut self, operator: Operator, inputs: Vec<Rc<Node>>) -> Rc<Node> {
            let node = Rc::new(Node {
                id: self.next_node_id,
                operator,
                inputs,
            });
            self.nodes.push(node.clone());
            self.next_node_id += 1;
            node
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Operator {
        Branch(BranchHint, BranchSemantics),
        IfTrue,
        IfFalse,
        Merge(usize),
        Phi(MachineRepresentation, usize),
        EffectPhi(usize),
        Start,
    }

    pub struct CommonOperatorBuilder {}

    impl CommonOperatorBuilder {
        pub fn new() -> Self {
            CommonOperatorBuilder {}
        }

        pub fn branch(&self, hint: BranchHint, semantics: BranchSemantics) -> Operator {
            Operator::Branch(hint, semantics)
        }

        pub fn if_true(&self) -> Operator {
            Operator::IfTrue
        }

        pub fn if_false(&self) -> Operator {
            Operator::IfFalse
        }

        pub fn merge(&self, count: usize) -> Operator {
            Operator::Merge(count)
        }

        pub fn phi(&self, rep: MachineRepresentation, count: usize) -> Operator {
            Operator::Phi(rep, count)
        }

        pub fn effect_phi(&self, count: usize) -> Operator {
            Operator::EffectPhi(count)
        }
    }

    pub struct Diamond {
        pub graph: TFGraph,
        pub common: CommonOperatorBuilder,
        pub branch: Rc<Node>,
        pub if_true: Rc<Node>,
        pub if_false: Rc<Node>,
        pub merge: Rc<Node>,
    }

    impl Diamond {
        pub fn new(
            mut graph: TFGraph,
            common: CommonOperatorBuilder,
            cond: Rc<Node>,
            hint: BranchHint,
            semantics: BranchSemantics,
        ) -> Self {
            let branch_op = common.branch(hint, semantics);
            let branch = graph.new_node(branch_op, vec![cond.clone(), graph.start()]);

            let if_true = graph.new_node(common.if_true(), vec![branch.clone()]);
            let if_false = graph.new_node(common.if_false(), vec![branch.clone()]);
            let merge = graph.new_node(common.merge(2), vec![if_true.clone(), if_false.clone()]);

            Diamond {
                graph,
                common,
                branch,
                if_true,
                if_false,
                merge,
            }
        }

        pub fn chain(&mut self, that: &Diamond) {
            let mut_branch = Rc::get_mut(&mut self.branch).unwrap(); // Ensure no other references
            mut_branch.replace_input(1, that.merge.clone());
        }

        pub fn chain_node(&mut self, that: Rc<Node>) {
            let mut_branch = Rc::get_mut(&mut self.branch).unwrap(); // Ensure no other references
            mut_branch.replace_input(1, that);
        }

        pub fn nest(&mut self, that: &mut Diamond, cond: bool) {
            if cond {
                let mut_branch = Rc::get_mut(&mut self.branch).unwrap(); // Ensure no other references
                mut_branch.replace_input(1, that.if_true.clone());

                let mut_merge = Rc::get_mut(&mut that.merge).unwrap(); // Ensure no other references
                mut_merge.replace_input(0, self.merge.clone());
            } else {
                 let mut_branch = Rc::get_mut(&mut self.branch).unwrap(); // Ensure no other references
                mut_branch.replace_input(1, that.if_false.clone());
                let mut_merge = Rc::get_mut(&mut that.merge).unwrap(); // Ensure no other references
                mut_merge.replace_input(1, self.merge.clone());
            }
        }

        pub fn phi(&mut self, rep: MachineRepresentation, tv: Rc<Node>, fv: Rc<Node>) -> Rc<Node> {
            self.graph.new_node(
                self.common.phi(rep, 2),
                vec![tv, fv, self.merge.clone()],
            )
        }

        pub fn effect_phi(&mut self, tv: Rc<Node>, fv: Rc<Node>) -> Rc<Node> {
            self.graph.new_node(
                self.common.effect_phi(2),
                vec![tv, fv, self.merge.clone()],
            )
        }
    }
}
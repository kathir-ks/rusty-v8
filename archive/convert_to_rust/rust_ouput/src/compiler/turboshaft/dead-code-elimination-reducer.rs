// Converted from V8 C++ source files:
// Header: dead-code-elimination-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    use std::{
        fmt,
        fmt::{Debug, Display, Formatter},
        marker::PhantomData,
        mem,
        ops::{Deref, DerefMut, Index, IndexMut},
        optional::Option,
        ptr::NonNull,
    };

    pub use crate::compiler::dead_code_elimination::DeadCodeElimination;
    use crate::compiler::turboshaft::csa_optimize_phase::V8;
    use crate::compiler::turboshaft::loop_peeling_reducer::AbortReason;
    use crate::compiler::turboshaft::operations::Any;
    use crate::compiler::turboshaft::wasm_lowering_reducer::Kind;
    use crate::{
        base,
        compiler::{
            representation_change::Representation,
            wasm_gc_operator_reducer::If,
        },
        execution::isolate::Isolate,
    };

    pub struct ControlState {
        kind: ControlStateKind,
        block: BlockIndex,
    }

    impl ControlState {
        pub fn not_eliminatable() -> Self {
            ControlState {
                kind: ControlStateKind::NotEliminatable,
                block: BlockIndex::invalid(),
            }
        }

        pub fn block(block: BlockIndex) -> Self {
            ControlState {
                kind: ControlStateKind::Block,
                block,
            }
        }

        pub fn unreachable() -> Self {
            ControlState {
                kind: ControlStateKind::Unreachable,
                block: BlockIndex::invalid(),
            }
        }

        pub fn new(kind: ControlStateKind, block: BlockIndex) -> Self {
            ControlState { kind, block }
        }

        pub fn least_upper_bound(lhs: &ControlState, rhs: &ControlState) -> Self {
            match lhs.kind {
                ControlStateKind::Unreachable => rhs.clone(),
                ControlStateKind::Block => {
                    if rhs.kind == ControlStateKind::Unreachable {
                        lhs.clone()
                    } else if rhs.kind == ControlStateKind::NotEliminatable {
                        rhs.clone()
                    } else if lhs.block == rhs.block {
                        lhs.clone()
                    } else {
                        ControlState::not_eliminatable()
                    }
                }
                ControlStateKind::NotEliminatable => lhs.clone(),
            }
        }
    }

    impl Clone for ControlState {
        fn clone(&self) -> Self {
            ControlState {
                kind: self.kind.clone(),
                block: self.block.clone(),
            }
        }
    }

    impl PartialEq for ControlState {
        fn eq(&self, other: &Self) -> bool {
            if self.kind != other.kind {
                return false;
            }
            if self.kind == ControlStateKind::Block {
                return self.block == other.block;
            }
            true
        }
    }

    impl fmt::Display for ControlState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.kind {
                ControlStateKind::NotEliminatable => write!(f, "NotEliminatable"),
                ControlStateKind::Block => write!(f, "Block({})", self.block),
                ControlStateKind::Unreachable => write!(f, "Unreachable"),
            }
        }
    }

    impl Debug for ControlState {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "ControlState({})", self)
        }
    }

    impl ControlState {
        pub fn kind(&self) -> &ControlStateKind {
            &self.kind
        }

        pub fn block(&self) -> &BlockIndex {
            &self.block
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum ControlStateKind {
        Unreachable,
        Block,
        NotEliminatable,
    }

    pub struct OperationState {
        liveness: OperationStateLiveness,
    }

    impl OperationState {
        pub fn least_upper_bound(lhs: OperationStateLiveness, rhs: OperationStateLiveness) -> OperationStateLiveness {
            if lhs == OperationStateLiveness::Live || rhs == OperationStateLiveness::Live {
                OperationStateLiveness::Live
            } else {
                OperationStateLiveness::Dead
            }
        }
    }

    impl fmt::Display for OperationState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.liveness {
                OperationStateLiveness::Dead => write!(f, "Dead"),
                OperationStateLiveness::Live => write!(f, "Live"),
            }
        }
    }

    impl OperationState {
        pub fn liveness(&self) -> &OperationStateLiveness {
            &self.liveness
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum OperationStateLiveness {
        Dead,
        Live,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OpIndex {
        id: usize,
    }

    impl OpIndex {
        pub fn new(id: usize) -> Self {
            OpIndex { id }
        }
        pub fn id(&self) -> usize {
            self.id
        }
        pub fn invalid() -> Self {
            OpIndex { id: usize::MAX }
        }
        pub fn is_valid(&self) -> bool {
            self.id != usize::MAX
        }
    }

    impl fmt::Display for OpIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OpIndex({})", self.id)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BlockIndex {
        id: usize,
    }

    impl BlockIndex {
        pub fn new(id: usize) -> Self {
            BlockIndex { id }
        }

        pub fn id(&self) -> usize {
            self.id
        }

        pub fn invalid() -> Self {
            BlockIndex { id: usize::MAX }
        }
        pub fn is_valid(&self) -> bool {
            self.id != usize::MAX
        }
    }

    impl fmt::Display for BlockIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BlockIndex({})", self.id)
        }
    }

    pub struct Graph {
        blocks: Vec<Block>,
        operations: Vec<Operation>,
        op_id_count: usize,
        block_count: usize,
    }

    impl Graph {
        pub fn new(phase_zone: &Zone) -> Self {
            Graph {
                blocks: Vec::new(),
                operations: Vec::new(),
                op_id_count: 0,
                block_count: 0,
            }
        }

        pub fn add_block(&mut self, block: Block) -> BlockIndex {
            let index = BlockIndex::new(self.blocks.len());
            self.blocks.push(block);
            self.block_count += 1;
            index
        }

        pub fn add_operation(&mut self, operation: Operation) -> OpIndex {
            let index = OpIndex::new(self.operations.len());
            self.operations.push(operation);
            self.op_id_count += 1;
            index
        }

        pub fn get(&self, index: BlockIndex) -> &Block {
            &self.blocks[index.id]
        }
        pub fn get_mut(&mut self, index: BlockIndex) -> &mut Block {
            &mut self.blocks[index.id]
        }

        pub fn get(&self, index: OpIndex) -> &Operation {
            &self.operations[index.id]
        }
        pub fn get_mut(&mut self, index: OpIndex) -> &mut Operation {
            &mut self.operations[index.id]
        }

        pub fn block_count(&self) -> usize {
            self.block_count
        }

        pub fn op_id_count(&self) -> usize {
            self.op_id_count
        }

        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn operation_indices(&self, block: &Block) -> std::ops::Range<OpIndex> {
            OpIndex::new(0)..OpIndex::new(self.op_id_count)
        }
    }

    pub struct Block {
        index: BlockIndex,
    }

    impl Block {
        pub fn index(&self) -> BlockIndex {
            self.index
        }

        pub fn is_loop(&self) -> bool {
            false
        }

        pub fn is_merge(&self) -> bool {
            false
        }

        pub fn last_operation(&self, graph: &Graph) -> &Operation {
            let last_op_index = OpIndex::new(0);
            graph.get(last_op_index)
        }

        pub fn last_predecessor(&self) -> *mut Block {
            std::ptr::null_mut()
        }
    }

    pub struct Operation {
        inputs: Vec<OpIndex>,
        kind: OperationKind,
    }

    impl Operation {
        pub fn inputs(&self) -> &[OpIndex] {
            &self.inputs
        }

        pub fn is<T>(&self) -> bool {
            false
        }

        pub fn is_required_when_unused(&self) -> bool {
            false
        }
    }

    impl fmt::Display for Operation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Operation")
        }
    }

    #[derive(Debug)]
    pub enum OperationKind {}

    pub struct CallOp {}
    pub struct BranchOp {}
    pub struct GotoOp {}
    pub struct PhiOp {}
    pub struct DeadOp {}

    #[derive(Debug)]
    pub struct Zone {
        data: Vec<u8>,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone { data: Vec::new() }
        }
    }

    pub struct FixedOpIndexSidetable<T> {
        data: Vec<T>,
        graph: *const Graph,
    }

    impl<T: Clone> FixedOpIndexSidetable<T> {
        pub fn new(size: usize, default_value: T, _zone: &Zone, graph: *const Graph) -> Self {
            FixedOpIndexSidetable {
                data: vec![default_value; size],
                graph,
            }
        }
    }

    impl<T> Index<OpIndex> for FixedOpIndexSidetable<T> {
        type Output = T;

        fn index(&self, index: OpIndex) -> &Self::Output {
            &self.data[index.id()]
        }
    }

    impl<T> IndexMut<OpIndex> for FixedOpIndexSidetable<T> {
        fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
            &mut self.data[index.id()]
        }
    }

    pub struct FixedBlockSidetable<T> {
        data: Vec<T>,
    }

    impl<T: Clone> FixedBlockSidetable<T> {
        pub fn new(size: usize, default_value: T, _zone: &Zone) -> Self {
            FixedBlockSidetable {
                data: vec![default_value; size],
            }
        }
    }

    impl<T> Index<BlockIndex> for FixedBlockSidetable<T> {
        type Output = T;

        fn index(&self, index: BlockIndex) -> &Self::Output {
            &self.data[index.id()]
        }
    }

    impl<T> IndexMut<BlockIndex> for FixedBlockSidetable<T> {
        fn index_mut(&mut self, index: BlockIndex) -> &mut Self::Output {
            &mut self.data[index.id()]
        }
    }

    #[derive(Debug)]
    pub struct SparseOpIndexSideTable<T> {
        data: Vec<(OpIndex, T)>,
        graph: *const Graph,
        phantom: PhantomData<T>,
    }

    impl<T> SparseOpIndexSideTable<T> {
        pub fn new(_zone: &Zone, graph: *const Graph) -> Self {
            SparseOpIndexSideTable {
                data: Vec::new(),
                graph,
                phantom: PhantomData,
            }
        }

        pub fn contains(&self, index: OpIndex, value: &mut *mut BlockIndex) -> bool {
            for (op_index, target) in &self.data {
                if *op_index == index {
                    return true;
                }
            }
            false
        }

        pub fn remove(&mut self, _index: OpIndex) {}
    }

    impl<K, V> Index<(OpIndex, K)> for SparseOpIndexSideTable<V> {
        type Output = V;

        fn index(&self, _index: (OpIndex, K)) -> &Self::Output {
            todo!()
        }
    }

    impl<K, V> IndexMut<(OpIndex, K)> for SparseOpIndexSideTable<V> {
        fn index_mut(&mut self, _index: (OpIndex, K)) -> &mut Self::Output {
            todo!()
        }
    }

    impl<V> SparseOpIndexSideTable<V> {
        fn insert(&mut self, index: OpIndex, value: V) {
            self.data.push((index, value));
        }
    }

    impl<V> SparseOpIndexSideTable<V> {
        fn get(&self, index: OpIndex) -> Option<&V> {
            self.data
                .iter()
                .find(|(op_index, _)| *op_index == index)
                .map(|(_, value)| value)
        }

        fn get_mut(&mut self, index: OpIndex) -> Option<&mut V> {
            self.data
                .iter_mut()
                .find(|(op_index, _)| *op_index == index)
                .map(|(_, value)| value)
        }
    }

    impl<V> Deref for SparseOpIndexSideTable<V> {
        type Target = Vec<(OpIndex, V)>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<V> DerefMut for SparseOpIndexSideTable<V> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    pub fn successor_blocks(operation: &Operation) -> Vec<*mut Block> {
        Vec::new()
    }

    #[derive(Debug)]
    pub struct PrintAsBlockHeader {
        block: Block,
    }

    impl fmt::Display for PrintAsBlockHeader {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BlockHeader")
        }
    }

    impl PrintAsBlockHeader {
        pub fn new(block: Block) -> Self {
            PrintAsBlockHeader { block }
        }
    }

    pub struct DeadCodeAnalysis {
        graph_: Graph,
        liveness_: FixedOpIndexSidetable<OperationStateLiveness>,
        entry_control_state_: FixedBlockSidetable<ControlState>,
        rewritable_branch_targets_: SparseOpIndexSideTable<BlockIndex>,
        is_leaf_function_: bool,
        phase_zone: *const Zone,
    }

    impl DeadCodeAnalysis {
        pub fn new(graph: Graph, phase_zone: *const Zone) -> Self {
            let liveness_ = FixedOpIndexSidetable::new(
                graph.op_id_count(),
                OperationStateLiveness::Dead,
                unsafe { &*phase_zone },
                &graph,
            );
            let entry_control_state_ = FixedBlockSidetable::new(
                graph.block_count(),
                ControlState::unreachable(),
                unsafe { &*phase_zone },
            );
            let rewritable_branch_targets_ = SparseOpIndexSideTable::new(unsafe { &*phase_zone }, &graph);
            DeadCodeAnalysis {
                graph_: graph,
                liveness_,
                entry_control_state_,
                rewritable_branch_targets_,
                is_leaf_function_: true,
                phase_zone,
            }
        }

        pub fn run(
            &mut self,
        ) -> (
            FixedOpIndexSidetable<OperationStateLiveness>,
            SparseOpIndexSideTable<BlockIndex>,
        ) {
            for unprocessed_count in (0..self.graph_.block_count()).rev() {
                let block_index = BlockIndex::new(unprocessed_count);

                let block = self.graph_.get(block_index);
                self.process_block(block, &mut (self.graph_.block_count() - 1));
            }
            (
                self.liveness_.clone(),
                SparseOpIndexSideTable::new(unsafe { &*self.phase_zone }, &self.graph_),
            )
        }

        fn process_block(&mut self, block: &Block, unprocessed_count: &mut usize) {
            let successors = successor_blocks(block.last_operation(&self.graph_));
            let mut control_state = ControlState::unreachable();
            for successor in &successors {
                let r = self.entry_control_state_[BlockIndex::new(0)];
                control_state = ControlState::least_upper_bound(&control_state, &r);
            }

            let op_range = self.graph_.operation_indices(block);
            let mut has_live_phis = false;
            for index in op_range.start.id..op_range.end.id {
                let index = OpIndex::new(index);
                let op = self.graph_.get(index);
                let mut op_state = self.liveness_[index];

                if op.is::<DeadOp>() {
                    assert_eq!(op_state, OperationStateLiveness::Dead);
                } else if op.is::<CallOp>() {
                    self.is_leaf_function_ = false;
                } else if op.is::<BranchOp>() || op.is::<GotoOp>() {
                    if control_state != ControlState::not_eliminatable() {
                        assert_eq!(op_state, OperationStateLiveness::Dead);

                        if control_state.kind() == &ControlStateKind::Block {
                            let target = control_state.block();
                            assert!(target.is_valid());
                        }
                    } else {
                        op_state = OperationStateLiveness::Live;
                        self.rewritable_branch_targets_.remove(index);
                    }
                } else if op.is_required_when_unused() {
                    op_state = OperationStateLiveness::Live;
                } else if op.is::<PhiOp>() {
                    has_live_phis = has_live_phis || (op_state == OperationStateLiveness::Live);

                    if block.is_loop() {
                        if self.liveness_[OpIndex::new(0)] < op_state {
                            let backedge = block.last_predecessor();
                            *unprocessed_count =
                                std::cmp::max(*unprocessed_count, 0);
                        }
                    }
                }

                assert!(self.liveness_[index] <= op_state);
                if op_state == OperationStateLiveness::Dead {
                    continue;
                }

                self.liveness_[index] = op_state;

                for input in op.inputs() {
                    let old_input_state = self.liveness_[*input];
                    let new_input_state = OperationState::least_upper_bound(old_input_state, op_state);
                    self.liveness_[*input] = new_input_state;
                }

                if op_state == OperationStateLiveness::Live
                    && control_state != ControlState::not_eliminatable()
                {
                    control_state = ControlState::not_eliminatable();
                }
            }

            if block.is_merge() {
                if !has_live_phis {
                    if control_state.kind() != &ControlStateKind::Block {
                        control_state = ControlState::block(block.index());
                    }
                }
            } else if block.is_loop() {
                control_state = ControlState::not_eliminatable();

                if self.entry_control_state_[block.index()] != control_state {
                    let backedge = block.last_predecessor();
                    assert!(!backedge.is_null());
                    *unprocessed_count = std::cmp::max(*unprocessed_count, 0);
                }
            }

            self.entry_control_state_[block.index()] = control_state;
        }

        pub fn is_leaf_function(&self) -> bool {
            self.is_leaf_function_
        }
    }

    pub struct Assembler {
        phase_zone: *const Zone,
        input_graph: Graph,
    }

    impl Assembler {
        pub fn phase_zone(&self) -> *const Zone {
            self.phase_zone
        }
        pub fn input_graph(&self) -> &Graph {
            &self.input_graph
        }
        pub fn modifiable_input_graph(&mut self) -> &mut Graph {
            &mut self.input_graph
        }

        pub fn goto(&mut self, block: *mut Block) {}
        pub fn map_to_new_graph(&self, block: &Block) -> *mut Block {
            std::ptr::null_mut()
        }
    }

    pub struct UniformReducerAdapter<T, Next> {
        next: Next,
        _phantom: PhantomData<T>,
    }

    impl<T, Next> UniformReducerAdapter<T, Next> {
        pub fn new(next: Next) -> Self {
            UniformReducerAdapter {
                next,
                _phantom: PhantomData,
            }
        }
        pub fn asm(&mut self) -> &mut Assembler {
            todo!()
        }
        pub fn asm_ref(&self) -> &Assembler {
            todo!()
        }
        pub fn analyze(&mut self) {}

        pub fn reduce_input_graph_branch(
            &mut self,
            ig_index: V<None>,
            branch: &BranchOp,
        ) -> V<None> {
            V::Invalid()
        }
        pub fn reduce_input_graph_goto(&mut self, ig_index: V<None>, gto: &GotoOp) -> V<None> {
            V::Invalid()
        }
    }

    #[derive(Debug)]
    pub struct V<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> V<T> {
        pub fn invalid() -> Self {
            V { _phantom: PhantomData }
        }
    }

    impl<T> Clone for V<T> {
        fn clone(&self) -> Self {
            V { _phantom: PhantomData }
        }
    }

    impl<T> Copy for V<T> {}

    impl<T> V<T> {
        pub fn is_valid(&self) -> bool {
            true
        }

        pub fn valid() -> Self {
            Self { _phantom: PhantomData }
        }
    }

    pub enum None {}

    pub struct DeadCodeEliminationReducer<Next> {
        adapter: UniformReducerAdapter<DeadCodeEliminationReducer<Next>, Next>,
        liveness_: Option<FixedOpIndexSidetable<OperationStateLiveness>>,
        branch_rewrite_targets_: SparseOpIndexSideTable<BlockIndex>,
        analyzer_: DeadCodeAnalysis,
        asm_: Assembler,
    }

    impl<Next> DeadCodeEliminationReducer<Next> {
        pub fn new(next: Next, graph: Graph, phase_zone: *const Zone) -> Self {
            let mut asm_ = Assembler {
                phase_zone: phase_zone,
                input_graph: graph,
            };
            let analyzer_ = DeadCodeAnalysis::new(asm_.input_graph, phase_zone);
            let branch_rewrite_targets_ = SparseOpIndexSideTable::new(unsafe { &*phase_zone }, &asm_.input_graph);
            DeadCodeEliminationReducer {
                adapter: UniformReducerAdapter::new(next),
                liveness_: None,
                branch_rewrite_targets_: branch_rewrite_targets_,
                analyzer_: analyzer_,
                asm_: asm_,
            }
        }

        pub fn can_auto_inline_blocks_with_single_predecessor(&self) -> bool {
            false
        }

        pub fn analyze(&mut self) {
            let (liveness, branch_rewrite_targets) = self.analyzer_.run();
            self.liveness_ = Some(liveness);
            self.branch_rewrite_targets_ = branch_rewrite_targets;
            self.adapter.analyze();
        }

        pub fn reduce_input_graph_branch(
            &mut self,
            ig_index: V<None>,
            branch: &BranchOp,
        ) -> V<None> {
            if self.try_rewrite_branch(ig_index) {
                return V::invalid();
            }
            self.adapter.reduce_input_graph_branch(ig_index, branch)
        }

        pub fn reduce_input_graph_goto(&mut self, ig_index: V<None>, gto: &GotoOp) -> V<None> {
            if self.try_rewrite_branch(ig_index) {
                return V::invalid();
            }
            self.adapter.reduce_input_graph_goto(ig_index, gto)
        }

        pub fn reduce_input_graph_operation<Op, Continuation>(
            &mut self,
            ig_index: OpIndex,
            op: &Op,
        ) -> OpIndex {
            if self.liveness_.as_ref().unwrap()[ig_index] == OperationStateLiveness::Dead {
                return OpIndex::invalid();
            }
            todo!()
        }

        fn is_leaf_function(&self) -> bool {
            self.analyzer_.is_leaf_function()
        }

        fn try_rewrite_branch(&mut self, index: V<None>) -> bool {
            todo!()
        }
    }

}

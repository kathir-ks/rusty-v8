// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    marker::PhantomData,
    ops::{Index, IndexMut},
    option::Option,
};

// use crate::common::globals::*;  // Assuming globals are defined elsewhere
// use crate::compiler::backend::instruction_codes::*; // Assuming instruction_codes are defined elsewhere
// use crate::compiler::turboshaft::assembler::*; // Assuming assembler is defined elsewhere
// use crate::compiler::turboshaft::graph::*; // Assuming graph is defined elsewhere
// use crate::compiler::turboshaft::index::*; // Assuming index is defined elsewhere
// use crate::compiler::turboshaft::operations::*; // Assuming operations is defined elsewhere
// use crate::compiler::turboshaft::uniform_reducer_adapter::*; // Assuming uniform_reducer_adapter is defined elsewhere

pub mod turboshaft {
    use super::*;

    macro_rules! CHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!("CHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
            }
        };
    }

    macro_rules! DCHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                eprintln!("DCHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
            }
        };
    }

    macro_rules! DCHECK {
        ($cond:expr) => {
            if !$cond {
                eprintln!("DCHECK failed: {}", stringify!($cond));
            }
        };
    }

    macro_rules! DCHECK_NOT_NULL {
        ($ptr:expr) => {
            if $ptr.is_none() {
                eprintln!("DCHECK_NOT_NULL failed: {}", stringify!($ptr));
            }
        };
    }

    macro_rules! UNREACHABLE {
        () => {
            panic!("UNREACHABLE");
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct BlockIndex(u32);

    impl BlockIndex {
        pub const fn new(id: u32) -> Self {
            BlockIndex(id)
        }

        pub fn id(&self) -> u32 {
            self.0
        }
        pub const fn invalid() -> Self {
            BlockIndex(u32::MAX)
        }

        pub fn valid(&self) -> bool {
            self.0 != u32::MAX
        }
    }

    impl fmt::Display for BlockIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BlockIndex({})", self.0)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct OpIndex(u32);

    impl OpIndex {
        pub const fn new(id: u32) -> Self {
            OpIndex(id)
        }

        pub fn id(&self) -> u32 {
            self.0
        }

        pub const fn invalid() -> Self {
            OpIndex(u32::MAX)
        }
        pub fn valid(&self) -> bool {
            self.0 != u32::MAX
        }
    }

    impl fmt::Display for OpIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OpIndex({})", self.0)
        }
    }

    // Dummy structs representing V8's Operations and Graph structures
    pub trait OperationTrait {
        fn is<T: 'static>(&self) -> bool;
        fn is_required_when_unused(&self) -> bool;
        fn inputs(&self) -> Vec<OpIndex>;
        fn input_count(&self) -> usize;
        fn cast<T: 'static>(&self) -> &T;
        fn try_cast<T: 'static>(&self) -> Option<&T>;
    }

    #[derive(Debug, Clone)]
    pub struct Operation {
        is_dead: bool,
        is_required: bool,
        input_indices: Vec<OpIndex>,
    }

    impl Operation {
        pub fn new(is_dead: bool, is_required: bool, input_indices: Vec<OpIndex>) -> Self {
            Operation {
                is_dead,
                is_required,
                input_indices,
            }
        }
        pub fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        pub fn try_cast<T: 'static>(&self) -> Option<&T> {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                Some(unsafe { &*(self as *const Self as *const T) })
            } else {
                None
            }
        }
    }

    impl OperationTrait for Operation {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            self.is_required
        }
        fn inputs(&self) -> Vec<OpIndex> {
            self.input_indices.clone()
        }
        fn input_count(&self) -> usize {
            self.input_indices.len()
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }
    impl fmt::Display for Operation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Operation {{ is_dead: {} }}", self.is_dead)
        }
    }
    #[derive(Debug, Clone)]
    pub struct Graph {
        operations: Vec<Operation>,
        blocks: Vec<Block>,
        block_operation_indices: Vec<Vec<OpIndex>>,
        op_id_count: usize,
        block_count: usize,
    }

    impl Graph {
        pub fn new(
            operations: Vec<Operation>,
            blocks: Vec<Block>,
            block_operation_indices: Vec<Vec<OpIndex>>,
            op_id_count: usize,
            block_count: usize,
        ) -> Self {
            Graph {
                operations,
                blocks,
                block_operation_indices,
                op_id_count,
                block_count,
            }
        }
        pub fn get(&self, index: OpIndex) -> &Operation {
            &self.operations[index.0 as usize]
        }

        pub fn get_block(&self, index: BlockIndex) -> &Block {
            &self.blocks[index.0 as usize]
        }

        pub fn get_mut(&mut self, index: OpIndex) -> &mut Operation {
            &mut self.operations[index.0 as usize]
        }

        pub fn op_id_count(&self) -> usize {
            self.op_id_count
        }

        pub fn block_count(&self) -> usize {
            self.block_count
        }

        pub fn operation_indices(&self, block: &Block) -> std::ops::Range<usize> {
            let block_index = block.index();
            let start = self.block_operation_indices[block_index.0 as usize][0].0 as usize;
            let end = self.block_operation_indices[block_index.0 as usize].last().unwrap().0 as usize + 1;
            start..end
        }

        pub fn operation_indices_vector(&self, block: &Block) -> &Vec<OpIndex> {
            let block_index = block.index();
            &self.block_operation_indices[block_index.0 as usize]
        }

        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }
    }

    #[derive(Debug, Clone)]
    pub struct Block {
        index: BlockIndex,
        is_loop: bool,
        is_merge: bool,
        last_operation_index: OpIndex,
        last_predecessor: Option<BlockIndex>, // Option to handle potentially missing predecessors
    }

    impl Block {
        pub fn new(
            index: BlockIndex,
            is_loop: bool,
            is_merge: bool,
            last_operation_index: OpIndex,
            last_predecessor: Option<BlockIndex>, // Option to handle potentially missing predecessors
        ) -> Self {
            Block {
                index,
                is_loop,
                is_merge,
                last_operation_index,
                last_predecessor,
            }
        }
        pub fn index(&self) -> BlockIndex {
            self.index
        }
        pub fn is_loop(&self) -> bool {
            self.is_loop
        }
        pub fn is_merge(&self) -> bool {
            self.is_merge
        }
        pub fn is_loop_header(&self) -> bool {
            self.is_loop
        }
        pub fn last_operation(&self, graph: &Graph) -> &Operation {
            graph.get(self.last_operation_index)
        }
        pub fn last_predecessor(&self) -> Option<BlockIndex> {
            self.last_predecessor
        }
    }

    //Dummy structs for Operations
    #[derive(Debug, Clone)]
    pub struct DeadOp {}
    impl DeadOp {
        pub fn new() -> Self {
            DeadOp {}
        }
    }
    impl OperationTrait for DeadOp {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            false
        }
        fn inputs(&self) -> Vec<OpIndex> {
            Vec::new()
        }
        fn input_count(&self) -> usize {
            0
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CallOp {}
    impl CallOp {
        pub fn new() -> Self {
            CallOp {}
        }
    }
    impl OperationTrait for CallOp {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            false
        }
        fn inputs(&self) -> Vec<OpIndex> {
            Vec::new()
        }
        fn input_count(&self) -> usize {
            0
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BranchOp {}
    impl BranchOp {
        pub fn new() -> Self {
            BranchOp {}
        }
    }
    impl OperationTrait for BranchOp {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            false
        }
        fn inputs(&self) -> Vec<OpIndex> {
            Vec::new()
        }
        fn input_count(&self) -> usize {
            0
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct GotoOp {}
    impl GotoOp {
        pub fn new() -> Self {
            GotoOp {}
        }
    }
    impl OperationTrait for GotoOp {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            false
        }
        fn inputs(&self) -> Vec<OpIndex> {
            Vec::new()
        }
        fn input_count(&self) -> usize {
            0
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct PhiOp {
        inputs: Vec<OpIndex>,
    }
    impl PhiOp {
        pub const K_LOOP_PHI_BACK_EDGE_INDEX: usize = 1;
        pub fn new(inputs: Vec<OpIndex>) -> Self {
            PhiOp { inputs }
        }
        pub fn inputs(&self) -> &Vec<OpIndex> {
            &self.inputs
        }
    }
    impl OperationTrait for PhiOp {
        fn is<T: 'static>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>()
        }
        fn is_required_when_unused(&self) -> bool {
            false
        }
        fn inputs(&self) -> Vec<OpIndex> {
            self.inputs.clone()
        }
        fn input_count(&self) -> usize {
            self.inputs.len()
        }
        fn cast<T: 'static>(&self) -> &T {
            if self.is::<T>() {
                //SAFETY: We already checked the types match.
                unsafe { &*(self as *const Self as *const T) }
            } else {
                panic!("Invalid cast")
            }
        }
    }

    pub struct PrintAsBlockHeader<'a> {
        block: &'a Block,
    }

    impl<'a> PrintAsBlockHeader<'a> {
        pub fn new(block: &'a Block) -> Self {
            PrintAsBlockHeader { block }
        }
    }

    impl<'a> fmt::Display for PrintAsBlockHeader<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Block({})", self.block.index().0)
        }
    }

    /// Represents a fixed-size table indexed by OpIndex, storing elements of type T.
    #[derive(Debug, Clone)]
    pub struct FixedOpIndexSidetable<T: Copy> {
        data: Vec<T>,
        graph: *const Graph, //PhantomData<Graph>,
                              // PhantomData used to tell the compiler that FixedOpIndexSidetable<T> logically contains
                              // a reference to a Graph instance, therefore it will know that it cannot outlive this graph.
    }

    impl<T: Copy> FixedOpIndexSidetable<T> {
        /// Creates a new FixedOpIndexSidetable with a default value for all entries.
        pub fn new(size: usize, default_value: T, graph: &Graph) -> Self {
            FixedOpIndexSidetable {
                data: vec![default_value; size],
                graph: graph as *const Graph, //PhantomData,
            }
        }
    }

    impl<T: Copy> Index<OpIndex> for FixedOpIndexSidetable<T> {
        type Output = T;

        fn index(&self, index: OpIndex) -> &Self::Output {
            &self.data[index.0 as usize]
        }
    }

    impl<T: Copy> IndexMut<OpIndex> for FixedOpIndexSidetable<T> {
        fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
            &mut self.data[index.0 as usize]
        }
    }

    /// Represents a sparse table indexed by OpIndex, storing elements of type T.
    #[derive(Debug, Clone)]
    pub struct SparseOpIndexSideTable<T> {
        data: std::collections::HashMap<OpIndex, T>,
        phase_zone: *const usize, //PhantomData<usize>, // Replace Zone with a placeholder type
        graph: *const Graph,      //PhantomData<Graph>,
    }

    impl<T> SparseOpIndexSideTable<T> {
        /// Creates a new SparseOpIndexSideTable.
        pub fn new(phase_zone: *const usize, graph: &Graph) -> Self {
            SparseOpIndexSideTable {
                data: std::collections::HashMap::new(),
                phase_zone: phase_zone, //PhantomData,
                graph: graph as *const Graph, //PhantomData,
            }
        }

        /// Inserts a new value into the table.
        pub fn insert(&mut self, index: OpIndex, value: T) {
            self.data.insert(index, value);
        }

        /// Removes a value from the table.
        pub fn remove(&mut self, index: OpIndex) {
            self.data.remove(&index);
        }

        /// Checks if the table contains a value for the given index.
        pub fn contains(&self, index: OpIndex, value: *mut BlockIndex) -> bool
        where
            T: PartialEq,
        {
            match self.data.get(&index) {
                Some(_) => true,
                None => false,
            }
        }
        pub fn get(&self, index: OpIndex) -> Option<&T> {
            self.data.get(&index)
        }
    }

    impl<T> IntoIterator for SparseOpIndexSideTable<T> {
        type Item = (OpIndex, T);
        type IntoIter = std::collections::hash_map::IntoIter<OpIndex, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    // ControlState struct
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

        pub fn least_upper_bound(lhs: &ControlState, rhs: &ControlState) -> Self {
            match lhs.kind {
                ControlStateKind::Unreachable => *rhs,
                ControlStateKind::Block => {
                    if rhs.kind == ControlStateKind::Unreachable {
                        *lhs
                    } else if rhs.kind == ControlStateKind::NotEliminatable {
                        *rhs
                    } else if lhs.block == rhs.block {
                        *lhs
                    } else {
                        ControlState::not_eliminatable()
                    }
                }
                ControlStateKind::NotEliminatable => *lhs,
            }
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

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ControlStateKind {
        Unreachable,
        Block,
        NotEliminatable,
    }

    // OperationState struct
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct OperationState {
        liveness: OperationLiveness,
    }

    impl OperationState {
        pub fn least_upper_bound(lhs: OperationLiveness, rhs: OperationLiveness) -> OperationLiveness {
            match (lhs, rhs) {
                (OperationLiveness::Dead, OperationLiveness::Dead) => OperationLiveness::Dead,
                _ => OperationLiveness::Live,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperationLiveness {
        Dead,
        Live,
    }

    impl fmt::Display for OperationLiveness {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                OperationLiveness::Dead => write!(f, "Dead"),
                OperationLiveness::Live => write!(f, "Live"),
            }
        }
    }

    /// Performs dead code analysis on a Turboshaft graph.
    pub struct DeadCodeAnalysis {
        graph: *mut Graph,
        liveness: FixedOpIndexSidetable<OperationLiveness>,
        entry_control_state: FixedBlockSidetable<ControlState>,
        rewritable_branch_targets: SparseOpIndexSideTable<BlockIndex>,
        is_leaf_function: bool,
        phase_zone: *const usize,
    }

    impl DeadCodeAnalysis {
        /// Creates a new DeadCodeAnalysis instance.
        pub fn new(graph: &mut Graph, phase_zone: *const usize) -> Self {
            let liveness = FixedOpIndexSidetable::new(
                graph.op_id_count(),
                OperationLiveness::Dead,
                graph,
            );
            let entry_control_state =
                FixedBlockSidetable::new(graph.block_count(), ControlState::unreachable());
            let rewritable_branch_targets = SparseOpIndexSideTable::new(phase_zone, graph);
            DeadCodeAnalysis {
                graph: graph,
                liveness,
                entry_control_state,
                rewritable_branch_targets,
                is_leaf_function: true,
                phase_zone,
            }
        }

        /// Runs the dead code analysis.
        pub fn run<const TRACE_ANALYSIS: bool>(
            &mut self,
        ) -> (
            FixedOpIndexSidetable<OperationLiveness>,
            SparseOpIndexSideTable<BlockIndex>,
        ) {
            if TRACE_ANALYSIS {
                println!("===== Running Dead Code Analysis =====\n");
            }
            let graph = unsafe { &*self.graph };

            let mut unprocessed_count = graph.block_count();
            while unprocessed_count > 0 {
                let block_index = BlockIndex::new((unprocessed_count - 1) as u32);
                unprocessed_count -= 1;

                let block = graph.get_block(block_index);
                self.process_block::<TRACE_ANALYSIS>(block, &mut unprocessed_count);
            }

            if TRACE_ANALYSIS {
                println!("===== Results =====\n== Operation State ==\n");
                for block in graph.blocks() {
                    println!("{}:\n", PrintAsBlockHeader::new(block));
                    for index in graph.operation_indices_vector(block) {
                        let index_val = *index;
                        println!(
                            " {:8?} {:3}: {}",
                            self.liveness[index_val],
                            index_val.id(),
                            graph.get(index_val)
                        );
                    }
                }

                println!("== Rewritable Branches ==\n");
                for (branch_id, target) in self.rewritable_branch_targets.data.iter() {
                    DCHECK!(target.valid());
                    println!(
                        " {:3}: Branch ==> Goto {}",
                        branch_id.id(),
                        target.id()
                    );
                }
                println!("==========\n");
            }

            let liveness = unsafe {
                std::ptr::read(&self.liveness as *const FixedOpIndexSidetable<OperationLiveness>)
            };
            let branch_rewrite_targets = unsafe {
                std::ptr::read(&self.rewritable_branch_targets as *const SparseOpIndexSideTable<BlockIndex>)
            };

            (liveness, branch_rewrite_targets)
        }

        /// Processes a single block in the graph.
        fn process_block<const TRACE_ANALYSIS: bool>(
            &mut self,
            block: &Block,
            unprocessed_count: &mut u32,
        ) {
            if TRACE_ANALYSIS {
                println!(
                    "\n==========\n=== Processing {}:\n==========\nEXIT CONTROL STATE\n",
                    PrintAsBlockHeader::new(block)
                );
            }
            let graph = unsafe { &*self.graph };
            let mut successors = self.successor_blocks(block.last_operation(graph));
            let mut control_state = ControlState::unreachable();
            for i in 0..successors.len() {
                let r = self.entry_control_state[successors[i].index()];
                if TRACE_ANALYSIS {
                    println!(" Successor {}: {}\n", successors[i].index(), r);
                }
                control_state = ControlState::least_upper_bound(&control_state, &r);
            }
            if TRACE_ANALYSIS {
                println!("Combined: {}\n", control_state);
            }

            if TRACE_ANALYSIS {
                println!("OPERATION STATE\n");
            }

            let mut has_live_phis = false;
            for index_usize in graph.operation_indices_vector(block).iter().rev() {
                let index = *index_usize;
                let op = graph.get(index);
                if TRACE_ANALYSIS {
                    println!("{}:{:?}", index, op);
                }
                let mut op_state = self.liveness[index];

                if op.is::<DeadOp>() {
                    // Operation is already recognized as dead by a previous analysis.
                    DCHECK_EQ!(op_state, OperationLiveness::Dead);
                } else if op.is::<CallOp>() {
                    // The function contains a call, so it's not a leaf function.
                    self.is_leaf_function = false;
                } else if op.is::<BranchOp>() || op.is::<GotoOp>() {
                    if control_state != ControlState::not_eliminatable() {
                        // Branch is still dead.
                        DCHECK_EQ!(op_state, OperationLiveness::Dead);
                        // If we know a target block we can rewrite into a goto.
                        if control_state.kind == ControlStateKind::Block {
                            let target = control_state.block;
                            DCHECK!(target.valid());
                            self.rewritable_branch_targets.insert(index, target);
                        }
                    } else {
                        // Branch is live. We cannot rewrite it.
                        op_state = OperationLiveness::Live;
                        self.rewritable_branch_targets.remove(index);
                    }
                } else if op.is_required_when_unused() {
                    op_state = OperationLiveness::Live;
                } else if op.is::<PhiOp>() {
                    has_live_phis =
                        has_live_phis || (op_state == OperationLiveness::Live);

                    if block.is_loop() {
                        let phi: &PhiOp = op.cast::<PhiOp>();
                        // Check if the operation state of the input coming from the backedge
                        // changes the liveness of the phi. In that case, trigger a revisit of
                        // the loop.
                        if self.liveness[phi.inputs()[PhiOp::K_LOOP_PHI_BACK_EDGE_INDEX]]
                            < op_state
                        {
                            if TRACE_ANALYSIS {
                                println!(
                                    "Operation state has changed. Need to revisit loop.\n"
                                );
                            }
                            let backedge = block.last_predecessor();
                            // Revisit the loop by increasing the {unprocessed_count} to include
                            // all blocks of the loop.
                            match backedge {
                                Some(backedge) => *unprocessed_count =
                                    std::cmp::max(*unprocessed_count, backedge.id() + 1),
                                None => println!("No backedge!"),
                            };
                        }
                    }
                }

                DCHECK!(self.liveness[index] <= op_state);
                // If everything is still dead. We don't need to update anything.
                if op_state == OperationLiveness::Dead {
                    continue;
                }

                // We have a live operation.
                if TRACE_ANALYSIS {
                    println!(" {} <== {}\n", op_state, self.liveness[index]);
                }
                self.liveness[index] = op_state;

                if TRACE_ANALYSIS {
                    if op.input_count() > 0 {
                        println!(" Updating inputs:\n");
                    }
                }
                for input in op.inputs() {
                    let old_input_state = self.liveness[input];
                    let new_input_state = OperationState::least_upper_bound(old_input_state, op_state);
                    if TRACE_ANALYSIS {
                        println!(
                            "  {}: {:?} <== {:?} || {:?}\n",
                            input, new_input_state, old_input_state, op_state
                        );
                    }
                    self.liveness[input] = new_input_state;
                }

                if op_state == OperationLiveness::Live && control_state != ControlState::not_eliminatable() {
                    // This block has live operations, which means that we can't skip it.
                    // Reset the ControlState to NotEliminatable.
                    if TRACE_ANALYSIS {
                        println!(
                            "Block has live operations. New control state: {}\n",
                            ControlState::not_eliminatable()
                        );
                    }
                    control_state = ControlState::not_eliminatable();
                }
            }

            if TRACE_ANALYSIS {
                println!(
                    "ENTRY CONTROL STATE\nAfter operations: {}\n",
                    control_state
                );
            }

            // If this block is a merge and we don't have any live phis, it is a
            // potential target for branch redirection.
            if block.is_merge() {
                if !has_live_phis {
                    if control_state.kind != ControlStateKind::Block {
                        control_state = ControlState::block(block.index());
                        if TRACE_ANALYSIS {
                            println!(
                                "Block is loop or merge and has no live phi operations.\n"
                            );
                        }
                    } else if TRACE_ANALYSIS {
                        println!("Block is loop or merge and has no live phi operations.\nControl state already has a goto block: {}\n", control_state);
                    }
                }
            } else if block.is_loop() {
                // If this is a loop, we reset the control state to avoid jumps into the
                // middle of the loop. In particular, this is required to prevent
                // introducing new backedges when blocks towards the end of the loop body
                // want to jump to a block at the beginning (past the header).
                control_state = ControlState::not_eliminatable();
                if TRACE_ANALYSIS {
                    println!(
                        "Block is loop header. Resetting control state: {}\n",
                        control_state
                    );
                }

                if self.entry_control_state[block.index()] != control_state {
                    if TRACE_ANALYSIS {
                        println!(
                            "Control state has changed. Need to revisit loop.\n"
                        );
                    }
                    let backedge = block.last_predecessor();
                    match backedge {
                        Some(backedge) => {
                            // Revisit the loop by increasing the {unprocessed_count} to include
                            // all blocks of the loop.
                            *unprocessed_count =
                                std::cmp::max(*unprocessed_count, backedge.id() + 1);
                        }
                        None => println!("No backedge for loop header!"),
                    }
                }
            }

            if TRACE_ANALYSIS {
                println!("Final: {}\n", control_state);
            }
            self.entry_control_state[block.index()] = control_state;
        }

        /// Determines the successor blocks for a given operation.
        fn successor_blocks(&self, operation: &Operation) -> Vec<&Block> {
            // This is a simplified version.  In a real implementation, this would
            // analyze the operation to determine the actual successor blocks based on
            // the graph structure.
            //
            // For example, a BranchOp would have two successors, while a GotoOp would
            // have one.  A ReturnOp would have no successors.

            // Returning an empty vector for now as a placeholder.
            unsafe {
                let graph = &*self.graph;
                let mut successors = Vec::new();

                if operation.is::<BranchOp>() {
                    if graph.blocks().len() > 1 {
                        successors.push(&graph.blocks()[0]);
                        successors.push(&graph.blocks()[1]);
                    }
                } else if operation.is::<Goto
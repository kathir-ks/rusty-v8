// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod loop_unrolling_reducer {
    use std::{
        collections::{HashMap, HashSet},
        fmt,
        marker::PhantomData,
        ops::{BitAnd, BitOr, BitXor},
    };

    //use crate::base::logging;  // Assuming this is custom logging
    //use crate::compiler::globals; // Assuming these are global constants/flags
    //use crate::compiler::turboshaft::assembler; // Assuming this is a module for assembling code
    //use crate::compiler::turboshaft::copying_phase; // Assuming this handles copying phases
    //use crate::compiler::turboshaft::index; // Assuming this defines an index type
    //use crate::compiler::turboshaft::loop_finder; // Assuming this module finds loops
    //use crate::compiler::turboshaft::machine_optimization_reducer; // Assuming this defines a reducer trait
    //use crate::compiler::turboshaft::operations; // Assuming this defines operation structs
    //use crate::compiler::turboshaft::phase; // Assuming this defines phase-related types

    // Placeholder types/modules
    pub type OpIndex = usize;
    pub type Block = usize;
    pub type Graph = usize;
    pub type Zone = usize;
    pub type AnyOrNone = usize;
    pub type None = usize;
    pub type Word = usize;
    pub type JSHeapBroker = usize;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Kind {
        Exact,
        Approx,
        Unknown,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct IterationCount {
        kind_: Kind,
        count_: usize,
    }

    impl IterationCount {
        // Loops with an exact number of iteration could be unrolled.
        pub fn exact(count: usize) -> Self {
            IterationCount {
                kind_: Kind::Exact,
                count_: count,
            }
        }
        // We can remove stack checks from loops with a small number of iterations.
        pub fn approx(count: usize) -> Self {
            IterationCount {
                kind_: Kind::Approx,
                count_: count,
            }
        }
        pub fn unknown() -> Self {
            IterationCount { kind_: Kind::Unknown, count_: 0 }
        }

        pub fn new() -> Self {
            IterationCount { kind_: Kind::Unknown, count_: 0 }
        }
        pub fn with_kind(kind: Kind) -> Self {
            assert_ne!(kind, Kind::Exact);
            IterationCount { kind_: kind, count_: 0 }
        }
        pub fn with_kind_and_count(kind: Kind, count: usize) -> Self {
            assert!(kind == Kind::Exact || kind == Kind::Approx);
            IterationCount {
                kind_: kind,
                count_: count,
            }
        }

        pub fn exact_count(&self) -> usize {
            assert_eq!(self.kind_, Kind::Exact);
            self.count_
        }
        pub fn approx_count(&self) -> usize {
            assert_eq!(self.kind_, Kind::Approx);
            self.count_
        }

        pub fn is_exact(&self) -> bool {
            self.kind_ == Kind::Exact
        }
        pub fn is_approx(&self) -> bool {
            self.kind_ == Kind::Approx
        }
        pub fn is_unknown(&self) -> bool {
            self.kind_ == Kind::Unknown
        }

        pub fn is_smaller_than(&self, max: usize) -> bool {
            (self.is_exact() || self.is_approx()) && self.count_ < max
        }
    }

    impl fmt::Display for IterationCount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.kind_ {
                Kind::Exact => write!(f, "Exact({})", self.count_),
                Kind::Approx => write!(f, "Approx({})", self.count_),
                Kind::Unknown => write!(f, "Unknown"),
            }
        }
    }

    // Dummy types for operations
    pub struct ComparisonOp {
        pub kind: ComparisonOpKind,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ComparisonOpKind {
        Equal,
        SignedLessThan,
        SignedLessThanOrEqual,
        UnsignedLessThan,
        UnsignedLessThanOrEqual,
        SignedGreaterThan,
        SignedGreaterThanOrEqual,
        UnsignedGreaterThan,
        UnsignedGreaterThanOrEqual,
    }

    pub struct WordBinopOp {
        pub kind: WordBinopOpKind,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum WordBinopOpKind {
        Add,
        Mul,
        Sub,
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
    }

    pub struct OverflowCheckedBinopOp {
        pub kind: OverflowCheckedBinopOpKind,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum OverflowCheckedBinopOpKind {
        Add,
        Mul,
        Sub,
    }

    pub struct PhiOp {}

    pub struct GotoOp {
        pub destination: Block,
        pub is_backedge: bool,
    }
    pub struct BranchOp {
        pub if_true: Block,
        pub if_false: Block,
    }
    pub struct CallOp {}
    pub struct JSStackCheckOp {
        pub kind: JSStackCheckOpKind,
    }
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum JSStackCheckOpKind {
        Loop,
    }
    pub struct WasmStackCheckOp {
        pub kind: WasmStackCheckOpKind,
    }
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum WasmStackCheckOpKind {
        Loop,
    }

    // Dummy types for representation
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum WordRepresentation {
        Word32,
        Word64,
    }

    pub struct OperationMatcher {}

    impl OperationMatcher {
        pub fn new() -> Self {
            OperationMatcher {}
        }
        pub fn is_constant(&self, _idx: OpIndex) -> bool {
            false
        }
        pub fn constant_value(&self, _idx: OpIndex) -> u64 {
            0
        }

        pub fn get_operation(&self, _idx: OpIndex) -> Operation {
            Operation::Unknown
        }
    }

    // Added a generic operation enum to cover possible operation types
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Operation {
        Comparison(ComparisonOp),
        WordBinop(WordBinopOp),
        OverflowCheckedBinop(OverflowCheckedBinopOp),
        Phi(PhiOp),
        Goto(GotoOp),
        Branch(BranchOp),
        Call(CallOp),
        JSStackCheck(JSStackCheckOp),
        WasmStackCheck(WasmStackCheckOp),
        Unknown,
    }

    impl Operation {
        pub fn try_cast<T>(&self) -> Option<&T> {
            match self {
                Operation::Comparison(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<ComparisonOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::WordBinop(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<WordBinopOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::OverflowCheckedBinop(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<OverflowCheckedBinopOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::Phi(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<PhiOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::Goto(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<GotoOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::Branch(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<BranchOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::Call(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<CallOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::JSStackCheck(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<JSStackCheckOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                Operation::WasmStackCheck(op) => {
                    if let Some(x) = any_to_any::downcast_ref::<WasmStackCheckOp, T>(op) {
                        Some(x)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }

    mod any_to_any {
        use std::any::Any;

        pub fn downcast_ref<T: Any, U: Any>(any: &T) -> Option<&U> {
            let any: &dyn Any = any;
            any.downcast_ref::<U>()
        }
    }

    pub struct StaticCanonicalForLoopMatcher {
        matcher_: OperationMatcher,
    }

    impl StaticCanonicalForLoopMatcher {
        pub fn new(matcher: &OperationMatcher) -> Self {
            StaticCanonicalForLoopMatcher {
                matcher_: matcher.clone(),
            }
        }

        pub fn get_iter_count_if_static_canonical_for_loop(
            &self,
            _header: Block,
            cond_idx: OpIndex,
            loop_if_cond_is: bool,
        ) -> IterationCount {
            // Placeholder implementation, needs to be properly implemented.
            // Should actually analyze the loop and calculate iterations
            if cond_idx == 0 {
                IterationCount::unknown()
            } else {
                IterationCount::approx(5) // some default
            }
        }

        pub const fn comparison_kind_to_cmp_op(kind: ComparisonOpKind) -> CmpOp {
            match kind {
                ComparisonOpKind::Equal => CmpOp::Equal,
                ComparisonOpKind::SignedLessThan => CmpOp::SignedLessThan,
                ComparisonOpKind::SignedLessThanOrEqual => CmpOp::SignedLessThanOrEqual,
                ComparisonOpKind::UnsignedLessThan => CmpOp::UnsignedLessThan,
                ComparisonOpKind::UnsignedLessThanOrEqual => CmpOp::UnsignedLessThanOrEqual,
                ComparisonOpKind::SignedGreaterThan => CmpOp::SignedGreaterThan,
                ComparisonOpKind::SignedGreaterThanOrEqual => CmpOp::SignedGreaterThanOrEqual,
                ComparisonOpKind::UnsignedGreaterThan => CmpOp::UnsignedGreaterThan,
                ComparisonOpKind::UnsignedGreaterThanOrEqual => CmpOp::UnsignedGreaterThanOrEqual,
            }
        }

        pub const fn invert_comparison_op(op: CmpOp) -> CmpOp {
            match op {
                CmpOp::Equal => CmpOp::Equal,
                CmpOp::SignedLessThan => CmpOp::SignedGreaterThanOrEqual,
                CmpOp::SignedLessThanOrEqual => CmpOp::SignedGreaterThan,
                CmpOp::UnsignedLessThan => CmpOp::UnsignedGreaterThanOrEqual,
                CmpOp::UnsignedLessThanOrEqual => CmpOp::UnsignedGreaterThan,
                CmpOp::SignedGreaterThan => CmpOp::SignedLessThanOrEqual,
                CmpOp::SignedGreaterThanOrEqual => CmpOp::SignedLessThan,
                CmpOp::UnsignedGreaterThan => CmpOp::UnsignedLessThanOrEqual,
                CmpOp::UnsignedGreaterThanOrEqual => CmpOp::UnsignedLessThan,
            }
        }

        pub const fn binop_from_word_binop_kind(kind: WordBinopOpKind) -> BinOp {
            match kind {
                WordBinopOpKind::Add => BinOp::Add,
                WordBinopOpKind::Mul => BinOp::Mul,
                WordBinopOpKind::Sub => BinOp::Sub,
                WordBinopOpKind::BitwiseAnd => BinOp::BitwiseAnd,
                WordBinopOpKind::BitwiseOr => BinOp::BitwiseOr,
                WordBinopOpKind::BitwiseXor => BinOp::BitwiseXor,
            }
        }

        pub const fn binop_from_overflow_checked_binop_kind(
            kind: OverflowCheckedBinopOpKind,
        ) -> BinOp {
            match kind {
                OverflowCheckedBinopOpKind::Add => BinOp::OverflowCheckedAdd,
                OverflowCheckedBinopOpKind::Mul => BinOp::OverflowCheckedMul,
                OverflowCheckedBinopOpKind::Sub => BinOp::OverflowCheckedSub,
            }
        }

        pub const fn binop_kind_is_supported(binop_kind: WordBinopOpKind) -> bool {
            match binop_kind {
                WordBinopOpKind::Add
                | WordBinopOpKind::Mul
                | WordBinopOpKind::Sub
                | WordBinopOpKind::BitwiseAnd
                | WordBinopOpKind::BitwiseOr
                | WordBinopOpKind::BitwiseXor => true,
            }
        }

        fn match_phi_compare_cst(
            &self,
            _cond_idx: OpIndex,
            _cmp_op: &mut CmpOp,
            _phi: &mut OpIndex,
            _cst: &mut u64,
        ) -> bool {
            // Placeholder implementation, needs to be properly implemented.
            false
        }

        fn match_checked_overflow_binop(
            &self,
            _idx: OpIndex,
            _left: &mut usize,
            _right: &mut usize,
            _binop_op: &mut BinOp,
            _binop_rep: &mut WordRepresentation,
        ) -> bool {
            // Placeholder implementation, needs to be properly implemented.
            false
        }

        fn match_word_binop(
            &self,
            _idx: OpIndex,
            _left: &mut usize,
            _right: &mut usize,
            _binop_op: &mut BinOp,
            _binop_rep: &mut WordRepresentation,
        ) -> bool {
            // Placeholder implementation, needs to be properly implemented.
            false
        }

        fn count_iterations(
            &self,
            _equal_cst: u64,
            _cmp_op: CmpOp,
            _initial_input: u64,
            _binop_cst: u64,
            _binop_op: BinOp,
            _binop_rep: WordRepresentation,
            _loop_if_cond_is: bool,
        ) -> IterationCount {
            // Placeholder implementation, needs to be properly implemented.
            IterationCount::unknown()
        }

        fn count_iterations_impl<Int: std::ops::Add + std::ops::Sub + std::ops::Mul + std::cmp::PartialOrd + Copy>(
            &self,
            init: Int,
            max: Int,
            cmp_op: CmpOp,
            binop_cst: Int,
            binop_op: BinOp,
            binop_rep: WordRepresentation,
            loop_if_cond_is: bool,
        ) -> IterationCount {
            // Placeholder implementation, needs to be properly implemented.
            IterationCount::unknown()
        }

        const K_MAX_EXACT_ITER: usize = 5;
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum CmpOp {
        Equal,
        SignedLessThan,
        SignedLessThanOrEqual,
        UnsignedLessThan,
        UnsignedLessThanOrEqual,
        SignedGreaterThan,
        SignedGreaterThanOrEqual,
        UnsignedGreaterThan,
        UnsignedGreaterThanOrEqual,
    }

    impl fmt::Display for CmpOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CmpOp::Equal => write!(f, "Equal"),
                CmpOp::SignedLessThan => write!(f, "SignedLessThan"),
                CmpOp::SignedLessThanOrEqual => write!(f, "SignedLessThanOrEqual"),
                CmpOp::UnsignedLessThan => write!(f, "UnsignedLessThan"),
                CmpOp::UnsignedLessThanOrEqual => write!(f, "UnsignedLessThanOrEqual"),
                CmpOp::SignedGreaterThan => write!(f, "SignedGreaterThan"),
                CmpOp::SignedGreaterThanOrEqual => write!(f, "SignedGreaterThanOrEqual"),
                CmpOp::UnsignedGreaterThan => write!(f, "UnsignedGreaterThan"),
                CmpOp::UnsignedGreaterThanOrEqual => write!(f, "UnsignedGreaterThanOrEqual"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum BinOp {
        Add,
        Mul,
        Sub,
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
        OverflowCheckedAdd,
        OverflowCheckedMul,
        OverflowCheckedSub,
    }

    impl fmt::Display for BinOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                BinOp::Add => write!(f, "Add"),
                BinOp::Mul => write!(f, "Mul"),
                BinOp::Sub => write!(f, "Sub"),
                BinOp::BitwiseAnd => write!(f, "BitwiseAnd"),
                BinOp::BitwiseOr => write!(f, "BitwiseOr"),
                BinOp::BitwiseXor => write!(f, "BitwiseXor"),
                BinOp::OverflowCheckedAdd => write!(f, "OverflowCheckedAdd"),
                BinOp::OverflowCheckedMul => write!(f, "OverflowCheckedMul"),
                BinOp::OverflowCheckedSub => write!(f, "OverflowCheckedSub"),
            }
        }
    }

    pub struct LoopUnrollingAnalyzer {
        input_graph_: Graph,
        matcher_: OperationMatcher,
        loop_finder_: LoopFinder,
        loop_iteration_count_: HashMap<Block, IterationCount>,
        canonical_loop_matcher_: StaticCanonicalForLoopMatcher,
        is_wasm_: bool,
        stack_checks_to_remove_: HashSet<u32>,
        k_max_loop_size_for_partial_unrolling: usize,
        can_unroll_at_least_one_loop_: bool,
    }

    impl LoopUnrollingAnalyzer {
        pub fn new(phase_zone: Zone, input_graph: Graph, is_wasm: bool) -> Self {
            let matcher_ = OperationMatcher::new();
            let canonical_loop_matcher_ = StaticCanonicalForLoopMatcher::new(&matcher_);
            let mut analyzer = LoopUnrollingAnalyzer {
                input_graph_: input_graph,
                matcher_: matcher_,
                loop_finder_: LoopFinder::new(phase_zone, input_graph),
                loop_iteration_count_: HashMap::new(),
                canonical_loop_matcher_: canonical_loop_matcher_,
                is_wasm_: is_wasm,
                stack_checks_to_remove_: HashSet::new(),
                k_max_loop_size_for_partial_unrolling: if is_wasm {
                    Self::K_WASM_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING
                } else {
                    Self::K_JS_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING
                },
                can_unroll_at_least_one_loop_: false,
            };
            analyzer.detect_unrollable_loops();
            analyzer
        }

        pub fn should_fully_unroll_loop(&self, loop_header: Block) -> bool {
            //DCHECK(loop_header.is_loop());
            let header_info = self.loop_finder_.get_loop_info(loop_header);
            if header_info.has_inner_loops {
                return false;
            }
            if header_info.op_count > Self::K_MAX_LOOP_SIZE_FOR_FULL_UNROLLING {
                return false;
            }

            let iter_count = self.get_iteration_count(loop_header);
            iter_count.is_exact()
                && iter_count.exact_count() < Self::K_MAX_LOOP_ITERATIONS_FOR_FULL_UNROLLING
        }

        pub fn should_partially_unroll_loop(&self, loop_header: Block) -> bool {
            //DCHECK(loop_header.is_loop());
            let info = self.loop_finder_.get_loop_info(loop_header);
            !info.has_inner_loops && info.op_count < self.k_max_loop_size_for_partial_unrolling
        }

        // The returned unroll count is the total number of copies of the loop body
        // in the resulting graph, i.e., an unroll count of N means N-1 copies of the
        // body which were partially unrolled, and 1 for the original/remaining body.
        pub fn get_partial_unroll_count(&self, _loop_header: Block) -> usize {
            // Don't unroll if the function is already huge.
            // Otherwise we have run into pathological runtimes or large memory usage,
            // e.g., in register allocation in the past, see https://crbug.com/383661627
            // for an example / reproducer.
            // Even though we return an unroll count of one (i.e., don't unroll at all
            // really), running this phase can speed up subsequent optimizations,
            // probably because it produces loops in a "compact"/good block order for
            // analyses, namely <loop header>, <loop body>, <loop exit>, <rest of code>.
            // In principle, we should fix complexity problems in analyses, make sure
            // loops are already produced in this order, and not rely on the "unrolling"
            // here for the order alone, but this is a longer standing issue.
            // Placeholder implementation, needs to be properly implemented.
            1
        }

        pub fn should_remove_loop(&self, loop_header: Block) -> bool {
            let iter_count = self.get_iteration_count(loop_header);
            iter_count.is_exact() && iter_count.exact_count() == 0
        }

        pub fn get_iteration_count(&self, loop_header: Block) -> IterationCount {
            //DCHECK(loop_header.is_loop());
            self.loop_iteration_count_
                .get(&loop_header)
                .copied()
                .unwrap_or(IterationCount::unknown())
        }

        pub fn get_loop_body(&self, loop_header: Block) -> HashSet<Block> {
            self.loop_finder_.get_loop_body(loop_header)
        }

        pub fn get_loop_header(&self, block: Block) -> Block {
            self.loop_finder_.get_loop_header(block)
        }

        pub fn can_unroll_at_least_one_loop(&self) -> bool {
            self.can_unroll_at_least_one_loop_
        }

        const K_MAX_LOOP_SIZE_FOR_FULL_UNROLLING: usize = 150;
        // This function size limit is quite arbitrary. It is large enough that we
        // probably never hit it in JavaScript and it is lower than the operation
        // count we have seen in some huge Wasm functions in the past, e.g., function
        // #21937 of https://crbug.com/383661627 (1.7M operations, 2.7MB wire bytes).
        const K_MAX_FUNCTION_SIZE_FOR_PARTIAL_UNROLLING: usize = 1_000_000;
        const K_JS_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING: usize = 50;
        const K_WASM_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING: usize = 80;
        const K_WASM_MAX_UNROLLED_LOOP_SIZE: usize = 240;
        const K_MAX_LOOP_ITERATIONS_FOR_FULL_UNROLLING: usize = 4;
        const K_MAX_PARTIAL_UNROLLING_COUNT: usize = 4;
        const K_MAX_ITER_FOR_STACK_CHECK_REMOVAL: usize = 5000;

        fn detect_unrollable_loops(&mut self) {
            // Placeholder implementation, needs to be properly implemented.
            self.can_unroll_at_least_one_loop_ = false; // Example
        }
        fn get_loop_iteration_count(&self, _info: &LoopFinderLoopInfo) -> IterationCount {
            // Placeholder implementation, needs to be properly implemented.
            IterationCount::unknown()
        }
    }

    #[derive(Clone)]
    pub struct LoopFinder {
        // Placeholder fields, add the actual fields needed for the implementation
    }

    impl LoopFinder {
        pub fn new(_phase_zone: Zone, _input_graph: Graph) -> Self {
            // Placeholder implementation, needs to be properly implemented.
            LoopFinder {}
        }

        pub fn get_loop_info(&self, _loop_header: Block) -> LoopFinderLoopInfo {
            // Placeholder implementation, needs to be properly implemented.
            LoopFinderLoopInfo {
                has_inner_loops: false,
                op_count: 0,
            }
        }

        pub fn get_loop_body(&self, _loop_header: Block) -> HashSet<Block> {
            // Placeholder implementation, needs to be properly implemented.
            HashSet::new()
        }

        pub fn get_loop_header(&self, _block: Block) -> Block {
            // Placeholder implementation, needs to be properly implemented.
            0
        }
    }

    #[derive(Clone)]
    pub struct LoopFinderLoopInfo {
        pub has_inner_loops: bool,
        pub op_count: usize,
    }

    pub struct ZoneUnorderedMap<K, V> {
        data: HashMap<K, V>,
    }

    impl<K: Eq + std::hash::Hash + Copy, V: Copy> ZoneUnorderedMap<K, V> {
        pub fn new(_zone: Zone) -> Self {
            ZoneUnorderedMap { data: HashMap::new() }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.data.insert(key, value);
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }
    }

    pub struct ZoneAbslFlatHashSet<T> {
        data: HashSet<T>,
    }

    impl<T: Eq + std::hash::Hash + Copy> ZoneAbslFlatHashSet<T> {
        pub fn new(_zone: Zone) -> Self {
            ZoneAbslFlatHashSet { data: HashSet::new() }
        }

        pub fn insert(&mut self, value: T) {
            self.data.insert(value);
        }

        pub fn contains(&self, value: &T) -> bool {
            self.data.contains(value)
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    // -------- Reducer Templates --------

    // Boilerplate macro for reducer
    macro_rules! turboshaft_reducer_boilerplate {
        ($name:ident) => {
            pub struct $name<Next> {
                next: Next,
                phantom: PhantomData<Next>,
            }

            impl<Next> $name<Next> {
                pub fn new(next: Next) -> Self {
                    $name {
                        next,
                        phantom: PhantomData,
                    }
                }
            }
        };
    }

    // Trait to represent the next reducer in the chain
    pub trait ReducerNext<T> {
        fn reduce_input_graph_goto(&mut self, ig_idx: T, gto: &GotoOp) -> T;
        fn reduce_input_graph_branch(&mut self, ig_idx: T, branch: &BranchOp) -> T;
        fn reduce_input_graph_call(&mut self, ig_idx: T, call: &CallOp) -> T;
        fn reduce_input_graph_jsstackcheck(&mut self, ig_idx: T, check: &JSStackCheckOp) -> T;
        fn reduce_input_graph_wasmstackcheck(&mut self, ig_idx: T, check: &WasmStackCheckOp) -> T;
    }

    // Default implementation for the ReducerNext trait
    impl<T> ReducerNext<T> for () {
        fn reduce_input_graph_goto(&mut self, ig_idx: T, gto: &GotoOp) -> T {
            ig_idx
        }
        fn reduce_input_graph_branch(&mut self, ig_idx: T, branch: &BranchOp) -> T {
            ig_idx
        }
        fn reduce_input_graph_call(&mut self, ig_idx: T, call: &CallOp) -> T {
            ig_idx
        }
        fn reduce_input_graph_jsstackcheck(&mut self, ig_idx: T, check: &JSStackCheckOp) -> T {
            ig_idx
        }
        fn reduce_input_graph_wasmstackcheck(&mut self, ig_idx: T, check: &WasmStackCheckOp) -> T {
            ig_idx
        }
    }

    // Dummy functions for macros
    fn is_running_builtin_pipeline() -> bool {
        false
    }

    fn should_skip_optimization_step() -> bool {
        false
    }

    fn is_stack_check(_call: &CallOp, _ig: Graph, _broker: JSHeapBroker, _kind: StackCheckKind) -> bool {
        false
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum StackCheckKind {
        kJSIterationBody,
    }

    //Reducer that removes stack checks from loops with a small number of iterations.
    turboshaft_reducer_boilerplate!(LoopStackCheckElisionReducer);

    impl<Next: ReducerNext<OpIndex>> LoopStackCheckElisionReducer<Next> {
        pub fn bind(&mut self, _new_block: Block) {
            // Placeholder implementation
        }

        pub fn reduce_input_graph_call(&mut self, ig_idx: OpIndex, call: &CallOp) -> OpIndex {
            if should_skip_optimization_step() {
                return self.next.reduce_input_graph_call(ig_idx, call);
            }

            //            if self.skip_next_stack_check_ &&
            //                call.is_stack_check(__ input_graph(), self.broker_, StackCheckKind::kJSIterationBody) {
            //                self.skip_next_stack_check_ = false;
            //                return OpIndex::Invalid();
            //            }
            self.next.reduce_input_graph_call(ig_idx, call)
        }

        pub fn reduce_input_graph_jsstackcheck(&mut self, ig_idx: OpIndex, check: &JSStackCheckOp) -> OpIndex {
            // if self.skip_next_stack_check_ && check.kind == JSStackCheckOpKind::Loop {
            //     self.skip_next_stack_check_ = false;
            //     return OpIndex::Invalid();
            // }
            self.next.reduce_input_graph_jsstackcheck(ig_idx, check)
        }

        pub fn reduce_input_graph_wasmstackcheck(&mut self, ig_idx: OpIndex, check: &WasmStackCheckOp) -> OpIndex {
            // if self.skip_next_stack_check_ && check.kind == WasmStackCheckOpKind::Loop {
            //     self.skip_next_stack_check_ = false;
            //     return OpIndex::Invalid();
            // }
            self.next.reduce_input_graph_wasmstackcheck(ig_idx, check)
        }
    }

    impl<Next: ReducerNext<None>> ReducerNext<None> for LoopStackCheckElisionReducer<Next> {
        fn reduce_input_graph_goto(&mut self, ig_idx: None, gto: &GotoOp) -> None {
            self.next.reduce_input_graph_goto(ig_idx, gto)
        }

        fn reduce_input_graph_branch(&mut self, ig_idx: None, branch: &BranchOp) -> None {
            self.next.reduce_input_graph_branch(ig_idx, branch)
        }
        fn reduce_input_graph_call(&mut self, ig_idx: None, call: &CallOp) -> None {
            self.next.reduce_input_graph_call(ig_idx, call)
        }
        fn reduce_input_graph_jsstackcheck(&mut self, ig_idx: None, check: &JSStackCheckOp) -> None {
            self.next.reduce_input_graph_jsstackcheck(ig_idx, check)
        }

        fn reduce_input_graph_wasmstackcheck(&mut self, ig_idx: None, check: &WasmStackCheckOp) -> None {
            self.next.reduce_input_graph_wasmstackcheck(ig_idx, check)
        }
    }

    //The LoopUnrollingReducer.
    turboshaft_reducer_boilerplate!(LoopUnrolling
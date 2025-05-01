// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    ops::{BitAnd, BitOr, BitXor},
    option,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::compiler::turboshaft::index::OpIndex;
use crate::compiler::turboshaft::loop_finder::LoopFinder;

const TURBOSHAFT_TRACE_UNROLLING: AtomicBool = AtomicBool::new(false); // Equivalent of v8_flags.turboshaft_trace_unrolling

macro_rules! trace {
    ($($arg:tt)*) => {
        if TURBOSHAFT_TRACE_UNROLLING.load(Ordering::Relaxed) {
            println!($($arg)*);
        }
    };
}

mod base {
    pub mod bits {
        pub fn signed_add_overflow_32(x: i32, y: i32, result: &mut i32) -> bool {
            match x.overflowing_add(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }

        pub fn signed_mul_overflow_32(x: i32, y: i32, result: &mut i32) -> bool {
            match x.overflowing_mul(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }

        pub fn signed_sub_overflow_32(x: i32, y: i32, result: &mut i32) -> bool {
            match x.overflowing_sub(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }

        pub fn signed_add_overflow_64(x: i64, y: i64, result: &mut i64) -> bool {
            match x.overflowing_add(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }

        pub fn signed_mul_overflow_64(x: i64, y: i64, result: &mut i64) -> bool {
            match x.overflowing_mul(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }

        pub fn signed_sub_overflow_64(x: i64, y: i64, result: &mut i64) -> bool {
            match x.overflowing_sub(y) {
                (res, overflow) => {
                    *result = res;
                    overflow
                }
            }
        }
    }
}

mod compiler {
    pub mod turboshaft {
        pub mod index {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct OpIndex {
                id: usize, // Or other appropriate type
            }

            impl OpIndex {
                pub fn new(id: usize) -> Self {
                    OpIndex { id }
                }
                pub fn id(&self) -> usize {
                    self.id
                }
            }
        }

        pub mod loop_finder {
            use std::collections::HashMap;

            use super::index::OpIndex;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct BlockId(pub usize);

            #[derive(Debug)]
            pub struct Block {
                id: BlockId,
                is_loop: bool,
            }

            impl Block {
                pub fn new(id: BlockId, is_loop: bool) -> Self {
                    Block { id, is_loop }
                }

                pub fn id(&self) -> BlockId {
                    self.id
                }

                pub fn is_loop(&self) -> bool {
                    self.is_loop
                }

                pub fn contains(&self, _op_index: OpIndex) -> bool {
                    // Simplified placeholder
                    true
                }

                pub fn last_operation(&self, _input_graph: &InputGraph) -> LastOperationResult {
                    // Placeholder for demonstration
                    LastOperationResult::Branch(BranchOp {
                        if_true: BlockId(1),
                        if_false: BlockId(2),
                        condition: OpIndex::new(10),
                    })
                }
            }

            pub struct LoopFinder {
                loop_headers: HashMap<Block, LoopInfo>,
                // ... other fields
            }

            impl LoopFinder {
                pub fn new() -> Self {
                    LoopFinder {
                        loop_headers: HashMap::new(),
                    }
                }

                pub fn loop_headers(&self) -> &HashMap<Block, LoopInfo> {
                    &self.loop_headers
                }

                pub fn get_loop_header(&self, block_id: BlockId) -> &Block {
                    // Placeholder implementation.  Needs proper lookup.
                    static DUMMY_BLOCK: Block = Block {
                        id: BlockId(0),
                        is_loop: true,
                    };
                    &DUMMY_BLOCK
                }
            }

            #[derive(Debug)]
            pub struct LoopInfo {
                pub start: Block,
            }

            #[derive(Debug)]
            pub enum LastOperationResult {
                Branch(BranchOp),
                None,
            }

            #[derive(Debug)]
            pub struct BranchOp {
                pub if_true: BlockId,
                pub if_false: BlockId,
                pub condition: OpIndex,
            }

            impl BranchOp {
                pub fn try_cast<T>(&self) -> &Self {
                    self
                }
            }

            pub struct InputGraph {} // Placeholder, define as needed

        }

        pub mod loop_unrolling_reducer {
            use std::{
                collections::{HashMap, HashSet},
                fmt,
                option,
            };

            use super::{
                index::OpIndex,
                loop_finder::{Block, InputGraph, LoopFinder, LoopFinder::LoopInfo},
            };

            const K_MAX_ITER_FOR_STACK_CHECK_REMOVAL: usize = 100;
            const K_MAX_EXACT_ITER: usize = 10;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct IterationCount {
                kind: IterationCountKind,
                count: u64,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            enum IterationCountKind {
                Unknown,
                Exact,
                Approx,
            }

            impl IterationCount {
                pub fn unknown() -> Self {
                    IterationCount {
                        kind: IterationCountKind::Unknown,
                        count: 0,
                    }
                }

                pub fn exact(count: u64) -> Self {
                    IterationCount {
                        kind: IterationCountKind::Exact,
                        count,
                    }
                }

                pub fn approx(count: u64) -> Self {
                    IterationCount {
                        kind: IterationCountKind::Approx,
                        count,
                    }
                }

                pub fn is_unknown(&self) -> bool {
                    self.kind == IterationCountKind::Unknown
                }

                pub fn is_exact(&self) -> bool {
                    self.kind == IterationCountKind::Exact
                }

                pub fn is_approx(&self) -> bool {
                    self.kind == IterationCountKind::Approx
                }

                pub fn exact_count(&self) -> u64 {
                    assert_eq!(self.kind, IterationCountKind::Exact);
                    self.count
                }

                pub fn approx_count(&self) -> u64 {
                    assert_eq!(self.kind, IterationCountKind::Approx);
                    self.count
                }

                pub fn is_smaller_than(&self, max: usize) -> bool {
                    match self.kind {
                        IterationCountKind::Exact => self.count as usize <= max,
                        _ => false, // Consider only Exact for this check
                    }
                }
            }

            impl fmt::Display for IterationCount {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self.kind {
                        IterationCountKind::Exact => write!(f, "Exact[{}]", self.count),
                        IterationCountKind::Approx => write!(f, "Approx[{}]", self.count),
                        IterationCountKind::Unknown => write!(f, "Unknown"),
                    }
                }
            }

            pub struct LoopUnrollingAnalyzer {
                loop_finder_: LoopFinder,
                loop_iteration_count_: HashMap<Block, IterationCount>,
                stack_checks_to_remove_: HashSet<usize>,
                can_unroll_at_least_one_loop_: bool,
                canonical_loop_matcher_: StaticCanonicalForLoopMatcher,
            }

            impl LoopUnrollingAnalyzer {
                pub fn new(loop_finder: LoopFinder, matcher: StaticCanonicalForLoopMatcher) -> Self {
                    LoopUnrollingAnalyzer {
                        loop_finder_: loop_finder,
                        loop_iteration_count_: HashMap::new(),
                        stack_checks_to_remove_: HashSet::new(),
                        can_unroll_at_least_one_loop_: false,
                        canonical_loop_matcher_: matcher,
                    }
                }

                pub fn detect_unrollable_loops(&mut self) {
                    for (start, info) in self.loop_finder_.loop_headers() {
                        let iter_count = self.get_loop_iteration_count(info);
                        trace!(
                            "LoopUnrollingAnalyzer: loop at {:?} ==> iter_count={}",
                            start.id(),
                            iter_count
                        );
                        self.loop_iteration_count_.insert(*start, iter_count);

                        if self.should_fully_unroll_loop(start) || self.should_partially_unroll_loop(start) {
                            self.can_unroll_at_least_one_loop_ = true;
                        }

                        if iter_count.is_smaller_than(K_MAX_ITER_FOR_STACK_CHECK_REMOVAL) {
                            self.stack_checks_to_remove_.insert(start.id().0);
                        }
                    }
                }

                fn get_loop_iteration_count(
                    &self,
                    info: &LoopFinder::LoopInfo,
                ) -> IterationCount {
                    let start = &info.start;
                    assert!(start.is_loop());

                    // Checking that the condition for the loop can be computed statically, and
                    // that the loop contains no more than kMaxLoopIterationsForFullUnrolling
                    // iterations.
                    let branch = match start.last_operation(&InputGraph {}) {
                        crate::compiler::turboshaft::loop_finder::LastOperationResult::Branch(b) => b,
                        crate::compiler::turboshaft::loop_finder::LastOperationResult::None => {
                            return IterationCount::unknown();
                        }
                    };

                    // Checking that one of the successor of the loop header is indeed not in the
                    // loop (otherwise, the Branch that ends the loop header is not the Branch
                    // that decides to exit the loop).
                    let if_true_header = self.loop_finder_.get_loop_header(branch.if_true);
                    let if_false_header = self.loop_finder_.get_loop_header(branch.if_false);
                    if if_true_header.id() == if_false_header.id() {
                        return IterationCount::unknown();
                    }

                    // If {if_true} is in the loop, then we're looping if the condition is true,
                    // but if {if_false} is in the loop, then we're looping if the condition is
                    // false.
                    let loop_if_cond_is = if_true_header.id() == start.id();

                    self.canonical_loop_matcher_.get_iter_count_if_static_canonical_for_loop(
                        start,
                        branch.condition,
                        loop_if_cond_is,
                    )
                }

                fn should_fully_unroll_loop(&self, _start: &Block) -> bool {
                    // Placeholder
                    false
                }

                fn should_partially_unroll_loop(&self, _start: &Block) -> bool {
                    // Placeholder
                    false
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                        CmpOp::Equal => write!(f, "=="),
                        CmpOp::SignedLessThan => write!(f, "<ˢ"),
                        CmpOp::SignedLessThanOrEqual => write!(f, "<=ˢ"),
                        CmpOp::UnsignedLessThan => write!(f, "<ᵘ"),
                        CmpOp::UnsignedLessThanOrEqual => write!(f, "<=ᵘ"),
                        CmpOp::SignedGreaterThan => write!(f, ">ˢ"),
                        CmpOp::SignedGreaterThanOrEqual => write!(f, ">=ˢ"),
                        CmpOp::UnsignedGreaterThan => write!(f, ">ᵘ"),
                        CmpOp::UnsignedGreaterThanOrEqual => write!(f, ">=ᵘ"),
                    }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                        BinOp::Add => write!(f, "+"),
                        BinOp::Mul => write!(f, "*"),
                        BinOp::Sub => write!(f, "-"),
                        BinOp::BitwiseAnd => write!(f, "&"),
                        BinOp::BitwiseOr => write!(f, "|"),
                        BinOp::BitwiseXor => write!(f, "^"),
                        BinOp::OverflowCheckedAdd => write!(f, "+ᵒ"),
                        BinOp::OverflowCheckedMul => write!(f, "*ᵒ"),
                        BinOp::OverflowCheckedSub => write!(f, "-ᵒ"),
                    }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum WordRepresentation {
                Word32,
                Word64,
            }

            pub struct StaticCanonicalForLoopMatcher {
                matcher_: Matcher,
            }

            impl StaticCanonicalForLoopMatcher {
                pub fn new(matcher: Matcher) -> Self {
                    StaticCanonicalForLoopMatcher { matcher_: matcher }
                }

                // Tries to match `phi cmp cst` (or `cst cmp phi`).
                fn match_phi_compare_cst(
                    &self,
                    cond_idx: OpIndex,
                    cmp_op: &mut CmpOp,
                    phi: &mut OpIndex,
                    cst: &mut u64,
                ) -> bool {
                    let cond = self.matcher_.get(cond_idx);

                    if let Some(cmp) = cond.try_cast::<ComparisonOp>() {
                        *cmp_op = Self::comparison_kind_to_cmp_op(cmp.kind);
                    } else {
                        return false;
                    }

                    let left = cond.input(0);
                    let right = cond.input(1);

                    if self.matcher_.match_phi(left, 2) {
                        if self.matcher_.match_unsigned_integral_constant(right, cst) {
                            *phi = left;
                            return true;
                        }
                    } else if self.matcher_.match_phi(right, 2) {
                        if self.matcher_.match_unsigned_integral_constant(left, cst) {
                            *cmp_op = Self::invert_comparison_op(*cmp_op);
                            *phi = right;
                            return true;
                        }
                    }
                    false
                }

                fn match_checked_overflow_binop(
                    &self,
                    idx: OpIndex,
                    left: &mut V<Word>,
                    right: &mut V<Word>,
                    binop_op: &mut BinOp,
                    binop_rep: &mut WordRepresentation,
                ) -> bool {
                    if let Some(proj) = self.matcher_.try_cast::<ProjectionOp>(idx) {
                        if proj.index != OverflowCheckedBinopOp::kValueIndex {
                            return false;
                        }
                        if let Some(binop) =
                            self.matcher_.try_cast::<OverflowCheckedBinopOp>(proj.input())
                        {
                            *left = binop.left();
                            *right = binop.right();
                            *binop_op = Self::binop_from_overflow_checked_binop_kind(binop.kind);
                            *binop_rep = binop.rep;
                            return true;
                        }
                    }
                    false
                }

                fn match_word_binop(
                    &self,
                    idx: OpIndex,
                    left: &mut V<Word>,
                    right: &mut V<Word>,
                    binop_op: &mut BinOp,
                    binop_rep: &mut WordRepresentation,
                ) -> bool {
                    let mut kind = WordBinopOp::Kind::kAdd; // Dummy init. Value will be overwritten if successful match
                    if self.matcher_.match_word_binop::<Word>(
                        idx,
                        left,
                        right,
                        &mut kind,
                        binop_rep,
                    ) && Self::binop_kind_is_supported(kind)
                    {
                        *binop_op = Self::binop_from_word_binop_kind(kind);
                        return true;
                    }
                    false
                }

                pub fn get_iter_count_if_static_canonical_for_loop(
                    &self,
                    header: &Block,
                    cond_idx: OpIndex,
                    loop_if_cond_is: bool,
                ) -> IterationCount {
                    let mut cmp_op = CmpOp::Equal; // Dummy init
                    let mut phi_idx = OpIndex::new(0); // Dummy init
                    let mut cmp_cst: u64 = 0; // Dummy init
                    if !self.match_phi_compare_cst(cond_idx, &mut cmp_op, &mut phi_idx, &mut cmp_cst) {
                        return IterationCount::unknown();
                    }
                    if !header.contains(phi_idx) {
                        // The termination condition for this loop is based on a Phi that is defined
                        // in another loop.
                        return IterationCount::unknown();
                    }

                    let phi = self.matcher_.cast::<PhiOp>(phi_idx);

                    // We have: phi(..., ...) cmp_op cmp_cst
                    // eg, for (i = ...; i < 42; ...)
                    let mut phi_cst: u64 = 0; // Dummy init
                    if self.matcher_.match_unsigned_integral_constant(phi.input(0), &mut phi_cst) {
                        // We have: phi(phi_cst, ...) cmp_op cmp_cst
                        // eg, for (i = 0; i < 42; ...)
                        let mut left = V::new(); // Dummy init
                        let mut right = V::new(); // Dummy init
                        let mut binop_op = BinOp::Add; // Dummy init
                        let mut binop_rep = WordRepresentation::Word32; // Dummy init
                        if self.match_word_binop(
                            phi.input(1),
                            &mut left,
                            &mut right,
                            &mut binop_op,
                            &mut binop_rep,
                        ) || self.match_checked_overflow_binop(
                            phi.input(1),
                            &mut left,
                            &mut right,
                            &mut binop_op,
                            &mut binop_rep,
                        ) {
                            // We have: phi(phi_cst, ... binop_op ...) cmp_op cmp_cst
                            // eg, for (i = 0; i < 42; i = ... + ...)
                            if left == V::new() { // FIX
                                // We have: phi(phi_cst, phi binop_op ...) cmp_op cmp_cst
                                // eg, for (i = 0; i < 42; i = i + ...)
                                let mut binop_cst: u64 = 0; // Dummy init
                                if self.matcher_.match_unsigned_integral_constant(right, &mut binop_cst) {
                                    // We have: phi(phi_cst, phi binop_op binop_cst) cmp_op cmp_cst
                                    // eg, for (i = 0; i < 42; i = i + 2)
                                    return self.count_iterations(
                                        cmp_cst,
                                        cmp_op,
                                        phi_cst,
                                        binop_cst,
                                        binop_op,
                                        binop_rep,
                                        loop_if_cond_is,
                                    );
                                }
                            } else if right == V::new() { // FIX
                                // We have: phi(phi_cst, ... binop_op phi) cmp_op cmp_cst
                                // eg, for (i = 0; i < 42; i = ... + i)
                                let mut binop_cst: u64 = 0; // Dummy init
                                if self.matcher_.match_unsigned_integral_constant(left, &mut binop_cst) {
                                    // We have: phi(phi_cst, binop_cst binop_op phi) cmp_op cmp_cst
                                    // eg, for (i = 0; i < 42; i = 2 + i)
                                    return self.count_iterations(
                                        cmp_cst,
                                        cmp_op,
                                        phi_cst,
                                        binop_cst,
                                        binop_op,
                                        binop_rep,
                                        loop_if_cond_is,
                                    );
                                }
                            }
                        }
                    }

                    // The condition is not an operation that we support.
                    IterationCount::unknown()
                }

                const fn binop_kind_is_supported(binop_kind: WordBinopOp::Kind) -> bool {
                    match binop_kind {
                        // This list needs to be kept in sync with the `Next` function that follows.
                        WordBinopOp::Kind::kAdd => true,
                        WordBinopOp::Kind::kMul => true,
                        WordBinopOp::Kind::kSub => true,
                        WordBinopOp::Kind::kBitwiseAnd => true,
                        WordBinopOp::Kind::kBitwiseOr => true,
                        WordBinopOp::Kind::kBitwiseXor => true,
                        _ => false,
                    }
                }

                const fn binop_from_word_binop_kind(kind: WordBinopOp::Kind) -> BinOp {
                    assert!(Self::binop_kind_is_supported(kind));
                    match kind {
                        WordBinopOp::Kind::kAdd => BinOp::Add,
                        WordBinopOp::Kind::kMul => BinOp::Mul,
                        WordBinopOp::Kind::kSub => BinOp::Sub,
                        WordBinopOp::Kind::kBitwiseAnd => BinOp::BitwiseAnd,
                        WordBinopOp::Kind::kBitwiseOr => BinOp::BitwiseOr,
                        WordBinopOp::Kind::kBitwiseXor => BinOp::BitwiseXor,
                    }
                }

                const fn binop_from_overflow_checked_binop_kind(kind: OverflowCheckedBinopOp::Kind) -> BinOp {
                    match kind {
                        OverflowCheckedBinopOp::Kind::kSignedAdd => BinOp::OverflowCheckedAdd,
                        OverflowCheckedBinopOp::Kind::kSignedMul => BinOp::OverflowCheckedMul,
                        OverflowCheckedBinopOp::Kind::kSignedSub => BinOp::OverflowCheckedSub,
                    }
                }

                // Returns true if the loop
                // `for (i = initial_input, i cmp_op cmp_cst; i = i binop_op binop_cst)` has
                // fewer than `max_iter_` iterations.
                fn count_iterations(
                    &self,
                    cmp_cst: u64,
                    cmp_op: CmpOp,
                    initial_input: u64,
                    binop_cst: u64,
                    binop_op: BinOp,
                    binop_rep: WordRepresentation,
                    loop_if_cond_is: bool,
                ) -> IterationCount {
                    match cmp_op {
                        CmpOp::SignedLessThan
                        | CmpOp::SignedLessThanOrEqual
                        | CmpOp::SignedGreaterThan
                        | CmpOp::SignedGreaterThanOrEqual
                        | CmpOp::Equal => {
                            if binop_rep == WordRepresentation::Word32 {
                                self.count_iterations_impl::<i32>(
                                    initial_input as i32,
                                    cmp_cst as i32,
                                    cmp_op,
                                    binop_cst as i32,
                                    binop_op,
                                    binop_rep,
                                    loop_if_cond_is,
                                )
                            } else {
                                assert_eq!(binop_rep, WordRepresentation::Word64);
                                self.count_iterations_impl::<i64>(
                                    initial_input as i64,
                                    cmp_cst as i64,
                                    cmp_op,
                                    binop_cst as i64,
                                    binop_op,
                                    binop_rep,
                                    loop_if_cond_is,
                                )
                            }
                        }
                        CmpOp::UnsignedLessThan
                        | CmpOp::UnsignedLessThanOrEqual
                        | CmpOp::UnsignedGreaterThan
                        | CmpOp::UnsignedGreaterThanOrEqual => {
                            if binop_rep == WordRepresentation::Word32 {
                                self.count_iterations_impl::<u32>(
                                    initial_input as u32,
                                    cmp_cst as u32,
                                    cmp_op,
                                    binop_cst as u32,
                                    binop_op,
                                    binop_rep,
                                    loop_if_cond_is,
                                )
                            } else {
                                assert_eq!(binop_rep, WordRepresentation::Word64);
                                self.count_iterations_impl::<u64>(
                                    initial_input,
                                    cmp_cst,
                                    cmp_op,
                                    binop_cst,
                                    binop_op,
                                    binop_rep,
                                    loop_if_cond_is,
                                )
                            }
                        }
                    }
                }

                // Returns true if the loop
                // `for (i = init, i cmp_op max; i = i binop_op binop_cst)` has fewer than
                // `max_iter_` iterations.
                fn count_iterations_impl<Int: Integer>(
                    &self,
                    init: Int,
                    max: Int,
                    cmp_op: CmpOp,
                    binop_cst: Int,
                    binop_op: BinOp,
                    binop_rep: WordRepresentation,
                    loop_if_cond_is: bool,
                ) -> IterationCount {
                    use std::cmp::Ordering;

                    // It's a bit hard to compute the number of iterations without some kind of
                    // (simple) SMT solver, especially when taking overflows into account. Thus,
                    // we just simulate the evolution of the loop counter: we repeatedly compute
                    // `init binop_op binop_cst`, and compare the result with `max`. This is
                    // somewhat inefficient, so it should only be done if `kMaxExactIter` is
                    // small.
                    assert!(K_MAX_EXACT_ITER <= 10);

                    let mut curr = init;
                    let mut iter_count: usize = 0;
                    while iter_count < K_MAX_EXACT_ITER {
                        if cmp(&curr, &max, cmp_op) != loop_if_cond_is {
                            return IterationCount::exact(iter_count as u64);
                        }

                        if let Some(next) = next(curr, binop_cst, binop_op, binop_rep) {
                            curr = next;
                        } else {
                            // There was an overflow, bailing out.
                            break;
                        }
                        iter_count += 1;
                    }

                    if binop_cst == Int::zero() {
                        // If {binop_cst} is 0, the loop should either execute a single time or loop
                        // infinitely (since the increment is in the form of "i = i op binop_cst"
                        // with op being an arithmetic or bitwise binop). If we didn't detect above
                        // that it executes a single time, then we are in the latter case.
                        return IterationCount::unknown();
                    }

                    // Trying to figure out an approximate number of iterations
                    if binop_op == BinOp::Add {
                        if matches!(cmp_op, CmpOp::UnsignedLessThan | CmpOp::UnsignedLessThanOrEqual | CmpOp::SignedLessThan | CmpOp::SignedLessThanOrEqual)
                            && curr < max
                            && !sub_will_overflow(max, init)
                            && loop_if_cond_is
                        {
                            // eg, for (int i = 0; i < 42; i += 2)
                            if binop_cst < Int::zero() {
                                // Will either loop forever or rely on underflow wrap-around to
                                // eventually stop.
                                return IterationCount::unknown();
                            }
                            if div_will_overflow(max - init, binop_cst) {
                                return IterationCount::unknown();
                            }
                            let quotient = (max - init) / binop_cst;
                            assert!(quotient >= Int::zero());
                            return IterationCount::approx(quotient.to_u64().unwrap());
                        }
                        if matches!(cmp_op, CmpOp::UnsignedGreaterThan | CmpOp::UnsignedGreaterThanOrEqual | CmpOp::SignedGreaterThan | CmpOp::SignedGreaterThanOrEqual)
                            && init > max
                            && !sub_will_overflow(max, init)
                            && loop_if_cond_is
                        {
                            // eg, for (int i = 42; i > 0; i += -2)
                            if binop_cst > Int::zero() {
                                // Will either loop forever or rely on overflow wrap-around to
                                // eventually stop.
                                return IterationCount::unknown();
                            }
                            if div_will_overflow(max - init, binop_cst) {
                                return IterationCount::unknown();
                            }
                            let quotient = (max - init) / binop_cst;
                            assert!(quotient >= Int::zero());
                            return IterationCount::approx(quotient.to_u64().unwrap());
                        }
                        if cmp_op == CmpOp::Equal && !sub_will_overflow(max, init) && !loop_if_cond_is {
                            // eg, for (int i = 0;  i != 42; i += 2)
                            // or, for (int i = 42; i != 0;  i += -2)
                            if init < max && binop_cst < Int::zero() {
                                // Will either loop forever or rely on underflow wrap-around to
                                // eventually stop.
                                return IterationCount::unknown();
                            }
                            if init > max && binop_cst > Int::zero() {
                                // Will either loop forever or rely on overflow wrap-around to
                                // eventually stop.
                                return IterationCount::unknown();
                            }

                            let remainder = (max - init) % binop_cst;
                            if remainder != Int::zero() {
                                // Will loop forever or rely on over/underflow wrap-around to eventually
                                // stop.
                                return IterationCount::unknown();
                            }

                            let quotient = (max - init) / binop_cst;
                            assert!(quotient >= Int::zero());
                            return IterationCount::approx(quotient.to_u64().unwrap());
                        }
                    }

                    IterationCount::unknown()
                }

                const fn comparison_kind_to_cmp_op(kind: ComparisonOp::Kind) -> CmpOp {
                    match kind {
                        ComparisonOp::Kind::kEqual => CmpOp::Equal,
                        ComparisonOp::Kind::kSignedLessThan => CmpOp::SignedLessThan,
                        ComparisonOp::Kind::kSignedLessThanOrEqual => CmpOp::SignedLessThanOrEqual,
                        ComparisonOp::Kind::kUnsignedLessThan => CmpOp::UnsignedLessThan,
                        ComparisonOp::Kind::kUnsignedLessThanOrEqual => CmpOp::UnsignedLessThanOrEqual,
                    }
                }

                const fn invert_comparison_op(op: CmpOp) -> CmpOp {
                    match op {
                        CmpOp::Equal => CmpOp::Equal,
                        CmpOp::SignedLessThan => CmpOp::SignedGreaterThan,
                        CmpOp::SignedLessThanOrEqual => CmpOp::SignedGreaterThanOrEqual,
                        CmpOp::UnsignedLessThan => CmpOp::UnsignedGreaterThan,
                        CmpOp::UnsignedLessThanOrEqual => CmpOp::UnsignedGreaterThanOrEqual,
                        CmpOp::SignedGreaterThan => CmpOp::SignedLessThan
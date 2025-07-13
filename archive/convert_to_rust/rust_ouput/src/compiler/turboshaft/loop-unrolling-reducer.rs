// Converted from V8 C++ source files:
// Header: loop-unrolling-reducer.h
// Implementation: loop-unrolling-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod loop_unrolling_reducer {
use std::any::Any;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub,
};
use std::ptr::null_mut;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex, RwLock};

use crate::base::bits;
use crate::compiler::turboshaft::analyzer_iterator::Block;
use crate::compiler::turboshaft::assembler::TSAssembler;
use crate::compiler::turboshaft::index::OpIndex;
use crate::compiler::turboshaft::loop_finder::LoopFinder;
use crate::compiler::turboshaft::machine_optimization_reducer::AdvancedReducerWithControlPathState;
use crate::compiler::turboshaft::operations::*;
use crate::compiler::turboshaft::phase::PipelineData;
use crate::execution::isolate::Isolate;
use crate::execution::isolate::Wasm;
use crate::execution::simulator_riscv::Simulator;
use crate::heap::factory::New;
use crate::objects::js_array::JSArrayRef;
use crate::objects::map::MapRef;
use crate::objects::template_objects::FunctionTemplateInfoRef;
use crate::zone::zone::Zone;
use crate::zone::zone::ZoneSnapshot;

pub struct V8_EXPORT_PRIVATE {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TurboshaftPipelineKind {
    kGeneral,
    kCSA,
}

pub struct JSHeapBroker {}

impl JSHeapBroker {
    pub fn source(&self, _broker: &JSHeapBroker) -> Node {
        Node {}
    }

    pub fn code(&self) -> HeapObjectRef {
        HeapObjectRef {}
    }
}

pub struct HeapObjectRef {}

pub struct Node {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Address {
    address: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineRepresentation {
    Word32,
    Word64,
    Float32,
    Float64,
    Simd128,
    Tagged,
    Bit,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicMemoryOrder {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackCheckKind {
    kJSIterationBody,
    kJSFunctionEntry,
}

pub struct DirectHandle<T> {
    value: T,
}

pub struct ArrayList {}

impl ArrayList {
    fn Add(arg0:&Isolate, arg1: &DirectHandle<ArrayList>, arg2: DirectHandle<FeedbackVector>) -> DirectHandle<ArrayList>{
        DirectHandle{value: ArrayList{}}
    }
}

pub struct FeedbackVector {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    kGeneral,
    kUninitialized,
    kArguments,
    kRest,
    kWith,
}

pub struct CodeAssemblerLabel {}

pub struct ZoneAbslFlatHashSet<T> {
    set: std::collections::HashSet<T>,
}

impl<T: Eq + Hash + Copy> ZoneAbslFlatHashSet<T> {
    pub fn new() -> Self {
        ZoneAbslFlatHashSet {
            set: std::collections::HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn empty(&self) -> bool {
        self.set.is_empty()
    }
}

pub struct ZoneUnorderedMap<K, V> {
    map: std::collections::HashMap<K, V>,
}

impl<K: Eq + Hash + Copy, V: Copy> ZoneUnorderedMap<K, V> {
    pub fn new() -> Self {
        ZoneUnorderedMap {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn end(&self) -> bool {
        self.map.is_empty()
    }

}

pub struct Graph {
    stack_checks_to_remove_: ZoneAbslFlatHashSet<u32>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            stack_checks_to_remove_: ZoneAbslFlatHashSet::new(),
        }
    }

    pub fn stack_checks_to_remove(&mut self) -> &mut ZoneAbslFlatHashSet<u32> {
        &mut self.stack_checks_to_remove_
    }

    pub fn op_id_count(&self) -> usize{
        1000
    }

    pub fn operations(&self, begin: usize, end: usize) -> Vec<Operation> {
        vec![]
    }

    pub fn Index(&self, phi: PhiOp) -> OpIndex{
        OpIndex{id:0}
    }

    pub fn loop_unrolling_analyzer(&mut self) -> *mut LoopUnrollingAnalyzer {
        Box::into_raw(Box::new(LoopUnrollingAnalyzer::new(&mut Zone::new(0), self, false)))
    }
}

#[macro_export]
macro_rules! any_of {
    ($first:expr, $($rest:expr),+) => {
        $first $(|| $rest)+
    };
}

#[macro_export]
macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("DCHECK_NE failed: {} == {}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("DCHECK_LE failed: {} > {}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! DCHECK_GT {
    ($left:expr, $right:expr) => {
        if $left <= $right {
            panic!("DCHECK_GT failed: {} <= {}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! DCHECK_GE {
    ($left:expr, $right:expr) => {
        if $left < $right {
            panic!("DCHECK_GE failed: {} < {}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

#[macro_export]
macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IterationCount {
    kind_: IterationCountKind,
    count_: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IterationCountKind {
    kExact,
    kApprox,
    kUnknown,
}

impl IterationCount {
    // Loops with an exact number of iteration could be unrolled.
    pub fn exact(count: usize) -> Self {
        IterationCount {
            kind_: IterationCountKind::kExact,
            count_: count,
        }
    }
    // We can remove stack checks from loops with a small number of iterations.
    pub fn approx(count: usize) -> Self {
        IterationCount {
            kind_: IterationCountKind::kApprox,
            count_: count,
        }
    }
    pub fn unknown() -> Self {
        IterationCount {
            kind_: IterationCountKind::kUnknown,
            count_: 0,
        }
    }

    pub fn new() -> Self {
        IterationCount {
            kind_: IterationCountKind::kUnknown,
            count_: 0,
        }
    }
    pub fn with_kind(kind: IterationCountKind) -> Self {
        DCHECK_NE!(kind, IterationCountKind::kExact);
        IterationCount { kind_: kind, count_: 0 }
    }
    pub fn with_kind_and_count(kind: IterationCountKind, count: usize) -> Self {
        DCHECK_EQ!(kind, any_of!(IterationCountKind::kExact, IterationCountKind::kApprox));
        IterationCount { kind_: kind, count_: count }
    }

    pub fn exact_count(&self) -> usize {
        DCHECK_EQ!(self.kind_, IterationCountKind::kExact);
        self.count_
    }
    pub fn approx_count(&self) -> usize {
        DCHECK_EQ!(self.kind_, IterationCountKind::kApprox);
        self.count_
    }

    pub fn is_exact(&self) -> bool {
        self.kind_ == IterationCountKind::kExact
    }
    pub fn is_approx(&self) -> bool {
        self.kind_ == IterationCountKind::kApprox
    }
    pub fn is_unknown(&self) -> bool {
        self.kind_ == IterationCountKind::kUnknown
    }

    pub fn is_smaller_than(&self, max: usize) -> bool {
        (self.is_exact() || self.is_approx()) && self.count_ < max
    }
}

impl fmt::Display for IterationCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_exact() {
            write!(f, "Exact[{}]", self.exact_count())
        } else if self.is_approx() {
            write!(f, "Approx[{}]", self.approx_count())
        } else {
            DCHECK!(self.is_unknown());
            write!(f, "Unknown")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    kEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual,
    kUnsignedLessThan,
    kUnsignedLessThanOrEqual,
    kSignedGreaterThan,
    kSignedGreaterThanOrEqual,
    kUnsignedGreaterThan,
    kUnsignedGreaterThanOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    kAdd,
    kMul,
    kSub,
    kBitwiseAnd,
    kBitwiseOr,
    kBitwiseXor,
    kOverflowCheckedAdd,
    kOverflowCheckedMul,
    kOverflowCheckedSub,
}

#[derive(Debug, Clone)]
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
        header: &Block,
        cond_idx: OpIndex,
        loop_if_cond_is: bool,
    ) -> IterationCount {
        self.match_loop(header, cond_idx, loop_if_cond_is)
    }

    fn match_loop(
        &self,
        header: &Block,
        cond_idx: OpIndex,
        loop_if_cond_is: bool,
    ) -> IterationCount {
        let mut cmp_op = CmpOp::kEqual;
        let mut phi_idx = OpIndex { id: 0 };
        let mut cmp_cst: u64 = 0;

        if !self.match_phi_compare_cst(cond_idx, &mut cmp_op, &mut phi_idx, &mut cmp_cst) {
            return IterationCount::unknown();
        }

        if !header.contains(phi_idx) {
            return IterationCount::unknown();
        }

        let phi = self.matcher_.cast::<PhiOp>(phi_idx);

        let mut phi_cst: u64 = 0;
        if self.matcher_.match_unsigned_integral_constant(phi.input(0), &mut phi_cst) {
            let mut left = V::<Word> { phantom: PhantomData };
            let mut right = V::<Word> { phantom: PhantomData };
            let mut binop_op = BinOp::kAdd;
            let mut binop_rep = WordRepresentation::Word32;

            if self.match_word_binop(phi.input(1), &mut left, &mut right, &mut binop_op, &mut binop_rep) ||
                self.match_checked_overflow_binop(phi.input(1), &mut left, &mut right, &mut binop_op, &mut binop_rep) {

                if left == V::<Word>::from(phi_idx) {
                    let mut binop_cst: u64 = 0;
                    if self.matcher_.match_unsigned_integral_constant(right.into_op_index(), &mut binop_cst) {
                        return self.count_iterations(cmp_cst, cmp_op, phi_cst, binop_cst, binop_op, binop_rep, loop_if_cond_is);
                    }
                } else if right == V::<Word>::from(phi_idx) {
                    let mut binop_cst: u64 = 0;
                    if self.matcher_.match_unsigned_integral_constant(left.into_op_index(), &mut binop_cst) {
                        return self.count_iterations(cmp_cst, cmp_op, phi_cst, binop_cst, binop_op, binop_rep, loop_if_cond_is);
                    }
                }
            }
        }

        IterationCount::unknown()
    }

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
            if let Some(binop) = self.matcher_.try_cast::<OverflowCheckedBinopOp>(proj.input()) {
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
        let mut kind = WordBinopOp::Kind::kAdd;
        if self.matcher_.match_word_binop::<Word>(idx, left, right, &mut kind, binop_rep)
            && Self::binop_kind_is_supported(kind)
        {
            *binop_op = Self::binop_from_word_binop_kind(kind);
            return true;
        }
        false
    }

    fn count_iterations(
        &self,
        equal_cst: u64,
        cmp_op: CmpOp,
        initial_input: u64,
        binop_cst: u64,
        binop_op: BinOp,
        binop_rep: WordRepresentation,
        loop_if_cond_is: bool,
    ) -> IterationCount {
        match binop_rep {
            WordRepresentation::Word32 => {
                self.count_iterations_impl::<i32>(
                    initial_input as i32,
                    equal_cst as i32,
                    cmp_op,
                    binop_cst as i32,
                    binop_op,
                    binop_rep,
                    loop_if_cond_is,
                )
            }
            WordRepresentation::Word64 => {
                self.count_iterations_impl::<i64>(
                    initial_input as i64,
                    equal_cst as i64,
                    cmp_op,
                    binop_cst as i64,
                    binop_op,
                    binop_rep,
                    loop_if_cond_is,
                )
            }
            _ => IterationCount::unknown(),
        }
    }

    fn count_iterations_impl<Int: Copy +
                                    PartialOrd +
                                    Add<Output = Int> +
                                    Sub<Output = Int> +
                                    Mul<Output = Int> +
                                    Div<Output = Int> +
                                    Rem<Output = Int> +
                                    BitAnd<Output = Int> +
                                    BitOr<Output = Int> +
                                    BitXor<Output = Int> +
                                    From<i8> +
                                    std::fmt::Debug>
    (
        &self,
        init: Int,
        max: Int,
        cmp_op: CmpOp,
        binop_cst: Int,
        binop_op: BinOp,
        binop_rep: WordRepresentation,
        loop_if_cond_is: bool,
    ) -> IterationCount {
        use std::ops::*;
        use crate::loop_unrolling_reducer::*;
        use crate::loop_unrolling_reducer::BinOp::*;

        DCHECK!(std::any::TypeId::of::<Int>() == std::any::TypeId::of::<i32>() ||
                std::any::TypeId::of::<Int>() == std::any::TypeId::of::<u32>() ||
                std::any::TypeId::of::<Int>() == std::any::TypeId::of::<i64>() ||
                std::any::TypeId::of::<Int>() == std::any::TypeId::of::<u64>());

        let mut curr = init;
        let mut iter_count = 0;

        for _ in 0..Self::K_MAX_EXACT_ITER {
            if Self::cmp(curr, max, cmp_op) != loop_if_cond_is {
                return IterationCount::exact(iter_count);
            }

            match Self::next(curr, binop_cst, binop_op, binop_rep) {
                Some(next) => curr = next,
                None => break,
            }

            iter_count += 1;
        }

        if binop_cst == Int::from(0) {
            return IterationCount::unknown();
        }

        if binop_op == kAdd {
            if matches!(cmp_op, CmpOp::kUnsignedLessThan | CmpOp::kUnsignedLessThanOrEqual |
                                 CmpOp::kSignedLessThan | CmpOp::kSignedLessThanOrEqual) &&
                init < max &&
                !Self::sub_will_overflow(max, init) &&
                loop_if_cond_is {
                if binop_cst < Int::from(0) {
                    return IterationCount::unknown();
                }
                if Self::div_will_overflow(max - init, binop_cst) {
                  return IterationCount::unknown();
                }
                let quotient = (max - init) / binop_cst;
                if std::any::TypeId::of::<Int>() == std::any::TypeId::of::<i32>(){
                    if quotient < Int::from(0){
                        return IterationCount::unknown()
                    }
                }
                return IterationCount::approx(Self::to_usize(quotient));
            }
            if matches!(cmp_op, CmpOp::kUnsignedGreaterThan | CmpOp::kUnsignedGreaterThanOrEqual |
                                 CmpOp::kSignedGreaterThan | CmpOp::kSignedGreaterThanOrEqual) &&
                init > max &&
                !Self::sub_will_overflow(max, init) &&
                loop_if_cond_is {
                if binop_cst > Int::from(0) {
                    return IterationCount::unknown();
                }
                if Self::div_will_overflow(max - init, binop_cst){
                    return IterationCount::unknown();
                }
                let quotient = (max - init) / binop_cst;
                if std::any::TypeId::of::<Int>() == std::any::TypeId::of::<i32>(){
                    if quotient < Int::from(0){
                        return IterationCount::unknown()
                    }
                }
                return IterationCount::approx(Self::to_usize(quotient));
            }
            if cmp_op == CmpOp::kEqual && !Self::sub_will_overflow(max, init) && !loop_if_cond_is {
                if init < max && binop_cst < Int::from(0) {
                    return IterationCount::unknown();
                }
                if init > max && binop_cst > Int::from(0) {
                    return IterationCount::unknown();
                }

                let remainder = (max - init) % binop_cst;
                if remainder != Int::from(0) {
                    return IterationCount::unknown();
                }

                let quotient = (max - init) / binop_cst;
                if std::any::TypeId::of::<Int>() == std::any::TypeId::of::<i32>(){
                    if quotient < Int::from(0){
                        return IterationCount::unknown()
                    }
                }
                return IterationCount::approx(Self::to_usize(quotient));
            }
        }

        IterationCount::unknown()
    }

    const K_MAX_EXACT_ITER: usize = 5;

    const K_MAX_EXACT_ITER1: usize = 5;

    fn binop_kind_is_supported(binop_kind: WordBinopOp::Kind) -> bool {
        match binop_kind {
            WordBinopOp::Kind::kAdd => true,
            WordBinopOp::Kind::kMul => true,
            WordBinopOp::Kind::kSub => true,
            WordBinopOp::Kind::kBitwiseAnd => true,
            WordBinopOp::Kind::kBitwiseOr => true,
            WordBinopOp::Kind::kBitwiseXor => true,
            _ => false,
        }
    }

    fn binop_from_word_binop_kind(kind: WordBinopOp::Kind) -> BinOp {
        DCHECK!(Self::binop_kind_is_supported(kind));
        match kind {
            WordBinopOp::Kind::kAdd => BinOp::kAdd,
            WordBinopOp::Kind::kMul => BinOp::kMul,
            WordBinopOp::Kind::kSub => BinOp::kSub,
            WordBinopOp::Kind::kBitwiseAnd => BinOp::kBitwiseAnd,
            WordBinopOp::Kind::kBitwiseOr => BinOp::kBitwiseOr,
            WordBinopOp::Kind::kBitwiseXor => BinOp::kBitwiseXor,
            _ => panic!("Unreachable"),
        }
    }

    fn binop_from_overflow_checked_binop_kind(kind: OverflowCheckedBinopOp::Kind) -> BinOp {
        match kind {
            OverflowCheckedBinopOp::Kind::kSignedAdd => BinOp::kOverflowCheckedAdd,
            OverflowCheckedBinopOp::Kind::kSignedMul => BinOp::kOverflowCheckedMul,
            OverflowCheckedBinopOp::Kind::kSignedSub => BinOp::kOverflowCheckedSub,
        }
    }

    fn comparison_kind_to_cmp_op(kind: ComparisonOp::Kind) -> CmpOp {
        match kind {
            ComparisonOp::Kind::kEqual => CmpOp::kEqual,
            ComparisonOp::Kind::kSignedLessThan => CmpOp::kSignedLessThan,
            ComparisonOp::Kind::kSignedLessThanOrEqual => CmpOp::kSignedLessThanOrEqual,
            ComparisonOp::Kind::kUnsignedLessThan => CmpOp::kUnsignedLessThan,
            ComparisonOp::Kind::kUnsignedLessThanOrEqual => CmpOp::kUnsignedLessThanOrEqual,
        }
    }

    fn invert_comparison_op(op: CmpOp) -> CmpOp {
        match op {
            CmpOp::kEqual => CmpOp::kEqual,
            CmpOp::kSignedLessThan => CmpOp::kSignedGreaterThan,
            CmpOp::kSignedLessThanOrEqual => CmpOp::kSignedGreaterThanOrEqual,
            CmpOp::kUnsignedLessThan => CmpOp::kUnsignedGreaterThan,
            CmpOp::kUnsignedLessThanOrEqual => CmpOp::kUnsignedGreaterThanOrEqual,
            CmpOp::kSignedGreaterThan => CmpOp::kSignedLessThan,
            CmpOp::kSignedGreaterThanOrEqual => CmpOp::kSignedLessThanOrEqual,
            CmpOp::kUnsignedGreaterThan => CmpOp::kUnsignedLessThan,
            CmpOp::kUnsignedGreaterThanOrEqual => CmpOp::kUnsignedLessThanOrEqual,
        }
    }

    fn next<Int: Copy +
                Add<Output = Int> +
                Sub<Output = Int> +
                Mul<Output = Int> +
                BitAnd<Output = Int> +
                BitOr<Output = Int> +
                BitXor<Output = Int>>
    (
        val: Int,
        incr: Int,
        binop_op: BinOp,
        binop_rep: WordRepresentation,
    ) -> Option<Int> {
        use crate::loop_unrolling_reducer::BinOp::*;

        match binop_op {
            kBitwiseAnd => Some(val & incr),
            kBitwiseOr => Some(val | incr),
            kBitwiseXor => Some(val ^ incr),
            kAdd | kOverflowCheckedAdd => {
                if binop_rep == WordRepresentation::Word32() {
                    let res = val + incr;
                    Some(res)
                } else {
                    Some(val + incr)
                }
            }
            kMul | kOverflowCheckedMul => {
                if binop_rep == WordRepresentation::Word32() {
                    let res = val * incr;
                    Some(res)
                } else {
                    Some(val * incr)
                }
            }
            kSub | kOverflowCheckedSub => {
                if binop_rep == WordRepresentation::Word32() {
                    let res = val - incr;
                    Some(res)
                } else {
                    Some(val - incr)
                }
            }
        }
    }

    fn cmp<Int: PartialOrd>(val: Int, max: Int, cmp_op: CmpOp) -> bool {
        match cmp_op {
            CmpOp::kSignedLessThan | CmpOp::kUnsignedLessThan => val < max,
            CmpOp::kSignedLessThanOrEqual | CmpOp::kUnsignedLessThanOrEqual => val <= max,
            CmpOp::kSignedGreaterThan | CmpOp::kUnsignedGreaterThan => val > max,
            CmpOp::kSignedGreaterThanOrEqual | CmpOp::kUnsignedGreaterThanOrEqual => val >= max,
            CmpOp::kEqual => val == max,
        }
    }

    fn sub_will_overflow<Int: Sub + std::ops::Add<Output = Int> + From<i8>>(lhs: Int, rhs: Int) -> bool {
        false
    }

    fn div_will_overflow<Int>(dividend: Int, divisor: Int) -> bool {
        false
    }

    fn to_usize<Int>(value: Int) -> usize {
        value as usize
    }
}

#[derive(Debug)]
pub struct LoopUnrollingAnalyzer {
    input_graph_: *mut Graph,
    matcher_: OperationMatcher,
    loop_finder_: LoopFinder,
    loop_iteration_count_: ZoneUnorderedMap<*const Block, IterationCount>,
    canonical_loop_matcher_: StaticCanonicalForLoopMatcher,
    is_wasm_: bool,
    k_max_loop_size_for_partial_unrolling: usize,
    can_unroll_at_least_one_loop_: bool,
    stack_checks_to_remove_: *mut ZoneAbslFlatHashSet<u32>,
}

impl LoopUnrollingAnalyzer {
    pub fn new(phase_zone: &mut Zone, input_graph: *mut Graph, is_wasm: bool) -> Self {
        unsafe {
            let matcher = OperationMatcher::new(&(*input_graph));
            let canonical_loop_matcher = StaticCanonicalForLoopMatcher::new(&matcher);

            let k_max_loop_size_for_partial_unrolling =
                if is_wasm {
                    LoopUnrollingAnalyzer::K_WASM_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING
                } else {
                    LoopUnrollingAnalyzer::K_JS_MAX_LOOP_SIZE_FOR_PARTIAL_UNROLLING
                };

            let mut new_analyzer = LoopUnrollingAnalyzer {
                input_graph_: input_graph,
                matcher_: matcher.clone(),
                loop_finder_: LoopFinder::new(phase_zone, &(*input_graph)),
                loop_iteration_count_: ZoneUnorderedMap::new(),
                canonical_loop_matcher_: canonical_loop_matcher.clone(),
                is_wasm_: is_wasm,
                k_max_loop_size_for_partial_unrolling: k_max_loop_size_for_partial_unrolling,
                can_unroll_at_least_one_loop_: false,
                stack_checks_to_remove_: (*input_graph).stack_checks_to_remove()
            };

            new_analyzer.detect_unrollable_loops();

            new_analyzer
        }
    }

    pub fn should_fully_unroll_loop(&self, loop_header: &Block) -> bool {
        DCHECK!(loop_header.is_loop());

        let header_info = self.loop_finder_.get_loop_info(loop_header);
        if header_info.has_inner_loops {
            return false;
        }
        if header_info.op_count > Self::K_MAX_LOOP_SIZE_FOR_FULL_UNROLLING {
            return false;
        }

        let iter_count = self.get_iteration_count(loop_header);
        iter_count.is_exact() && iter_count.exact_count() < Self::K_MAX_LOOP_ITERATIONS_FOR_FULL_UNROLLING
    }

    pub fn should_partially_unroll_loop(&self, loop_header: &Block) -> bool {
        DCHECK!(loop_header.is_loop());
        let info = self.loop_finder_.get_loop_info(loop_header);
        !info.has_inner_loops && info.op_count < self.k_max_loop_size_for_partial_unrolling
    }

    // The returned unroll count is the total number of copies of the loop body
    // in the resulting graph, i.e., an unroll count of N means N-1 copies of the
    // body which were partially unrolled, and 1 for the original/remaining body.
    pub fn get_partial_unroll_count(&self, loop_header: &Block) -> usize {
        unsafe {
            if (*self.input_graph_).op_id_count() > Self::K_MAX_FUNCTION_SIZE_FOR_PARTIAL_UNROLLING {
                return 1;
            }
        }

        if self.is_wasm_ {
            let info = self.loop_finder_.get_loop_

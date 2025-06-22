// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// Placeholder for src/base/logging.h (consider using log crate)
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implication:expr) => {
        if $condition {
            DCHECK!($implication);
        }
    };
}

// Placeholder for src/base/vector.h (use Vec or slices)
type Vector<T> = Vec<T>;

// Placeholder for common-operator.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Nop, // Example
}

// Placeholder for turboshaft/assembler.h
pub struct Assembler {}
impl Assembler {
    pub fn input_graph(&self) -> &Graph {
        unimplemented!()
    }
    pub fn output_graph(&self) -> &Graph {
        unimplemented!()
    }
    pub fn graph_zone(&self) -> &Zone {
        unimplemented!()
    }
    pub fn modifiable_input_graph(&self) -> &mut Graph {
        unimplemented!()
    }
    pub fn phase_zone(&self) -> &Zone {
        unimplemented!()
    }
    pub fn current_block(&self) -> &Block {
        unimplemented!()
    }
}

// Placeholder for turboshaft/copying-phase.h
// (Likely not needed directly, functionality integrated into other parts)

// Placeholder for turboshaft/operations.h
#[derive(Debug, Clone)]
pub struct Operation {
    opcode: Opcode,
}

impl Operation {
    pub fn outputs_rep(&self) -> Vec<RegisterRepresentation> {
        unimplemented!()
    }
    pub fn try_cast<T: TurboshaftOp>(&self) -> Option<&T> {
        T::try_from(self)
    }

    pub fn is<T: TurboshaftOp>(&self) -> bool {
        T::try_from(self).is_some()
    }

    pub fn cast<T: TurboshaftOp>(&self) -> &T {
        T::try_from(self).unwrap()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

trait TurboshaftOp {
    fn try_from(op: &Operation) -> Option<&Self>;
}

// Example Op
#[derive(Debug, Clone)]
pub struct BranchOp {
    pub if_true: *const Block,
    pub if_false: *const Block,
    pub condition: OpIndex,
}

impl TurboshaftOp for BranchOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        // Placeholder Implementation, adjust based on actual BranchOp structure
        None //Some(&BranchOp{ if_true: std::ptr::null(), if_false: std::ptr::null(), condition: OpIndex::invalid()})
    }
}

#[derive(Debug, Clone)]
pub struct TupleOp {}

impl TurboshaftOp for TupleOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl TupleOp {
    pub fn inputs(&self) -> Vec<OpIndex> {
        unimplemented!()
    }

    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

// Example Op
#[derive(Debug, Clone)]
pub struct ConstantOp {
    pub kind: ConstantOpKind,
    pub value: ConstantOpStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstantOpKind {
    Int32,
    Float64,
}

#[derive(Debug, Clone)]
pub enum ConstantOpStorage {
    Int32(i32),
    Float64(f64),
}

impl TurboshaftOp for ConstantOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct WordBinopOp {}

impl TurboshaftOp for WordBinopOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl WordBinopOp {
    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct OverflowCheckedBinopOp {}

impl TurboshaftOp for OverflowCheckedBinopOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl OverflowCheckedBinopOp {
    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct FloatBinopOp {}

impl TurboshaftOp for FloatBinopOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl FloatBinopOp {
    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonOp {}

impl TurboshaftOp for ComparisonOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl ComparisonOp {
    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct ProjectionOp {}

impl TurboshaftOp for ProjectionOp {
    fn try_from(op: &Operation) -> Option<&Self> {
        None
    }
}

impl ProjectionOp {
    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }
}

// Placeholder for turboshaft/representations.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterRepresentation {
    Any,
    Word32,
    Word64,
    Float32,
    Float64,
    Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordRepresentation {
    Word32,
    Word64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatRepresentation {
    Float32,
    Float64,
}

// Placeholder for turboshaft/sidetable.h
pub struct GrowingOpIndexSidetable<T> {
    data: RefCell<Vec<T>>,
    zone: *const Zone,
    graph: *const Graph,
}

impl<T: Clone> GrowingOpIndexSidetable<T> {
    pub fn new(zone: *const Zone, graph: *const Graph, default: T) -> Self {
        GrowingOpIndexSidetable {
            data: RefCell::new(Vec::new()),
            zone,
            graph,
        }
    }
}

impl<T: Clone> std::ops::Index<OpIndex> for GrowingOpIndexSidetable<T> {
    type Output = T;

    fn index(&self, index: OpIndex) -> &Self::Output {
        let data = self.data.borrow();
        if (index.id() as usize) >= data.len() {
            panic!("Index out of bounds");
        }
        &data[index.id() as usize]
    }
}

impl<T: Clone> std::ops::IndexMut<OpIndex> for GrowingOpIndexSidetable<T> {
    fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
        let mut data = self.data.borrow_mut();
        let id = index.id() as usize;
        if id >= data.len() {
            data.resize(id + 1, (unsafe { (*self.zone).default_type() }));
        }
        &mut data[id]
    }
}

pub struct GrowingBlockSidetable<T> {
    data: RefCell<Vec<T>>,
    default_value: T,
    zone: *const Zone,
}

impl<T: Clone> GrowingBlockSidetable<T> {
    pub fn new(block_count: usize, default_value: T, zone: *const Zone) -> Self {
        GrowingBlockSidetable {
            data: RefCell::new(vec![default_value.clone(); block_count]),
            default_value,
            zone,
        }
    }
}

impl<T: Clone> std::ops::Index<BlockIndex> for GrowingBlockSidetable<T> {
    type Output = T;

    fn index(&self, index: BlockIndex) -> &Self::Output {
        let data = self.data.borrow();
        &data[index.id() as usize]
    }
}

impl<T: Clone> std::ops::IndexMut<BlockIndex> for GrowingBlockSidetable<T> {
    fn index_mut(&mut self, index: BlockIndex) -> &mut Self::Output {
        let mut data = self.data.borrow_mut();
        &mut data[index.id() as usize]
    }
}

// Placeholder for turboshaft/snapshot-table.h
pub struct SnapshotTable<T> {
    zone: *const Zone, // Assuming Zone is a valid type
                       // Placeholder implementation
}

impl<T: Clone + Debug> SnapshotTable<T> {
    pub fn new(zone: *const Zone) -> Self {
        SnapshotTable { zone }
    }
    pub fn new_key(&self, default_value: T) -> Key {
        unimplemented!()
    }
    pub fn get(&self, key: Key) -> T {
        unimplemented!()
    }
    pub fn set(&self, key: Key, value: T) {
        unimplemented!()
    }
    pub fn start_new_snapshot<F>(&mut self, predecessors: Vec<Snapshot>, merge_types: F)
    where
        F: Fn(Key, Vector<&T>) -> T,
    {
        unimplemented!()
    }

    pub fn seal(&self) -> Snapshot {
        unimplemented!()
    }

    pub fn is_sealed(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {}

// Placeholder for turboshaft/tracing.h (replace with logging)
mod tracing {
    pub struct Tracing {}
    impl Tracing {
        pub fn get() -> &'static Tracing {
            &TRACING
        }
        pub fn print_per_operation_data<F, G>(&self, name: &str, graph: &super::Graph, printer: F)
        where
            F: Fn(&mut std::fmt::Formatter, &super::Graph, super::OpIndex) -> bool,
            G: Fn(super::OpIndex) -> bool,
        {
            // Placeholder implementation
        }
        pub fn print_per_block_data<F>(&self, name: &str, graph: &super::Graph, printer: F)
        where
            F: Fn(&mut std::fmt::Formatter, &super::Graph, super::BlockIndex) -> bool,
        {
            // Placeholder implementation
        }
    }
    static TRACING: Tracing = Tracing {};

    #[macro_export]
    macro_rules! TURBOSHAFT_TRACE_TYPING_OK {
        ($fmt:expr, $($arg:tt)*) => {
            //println!("TRACE_OK: {}", format!($fmt, $($arg)*)); // Use println or a logging crate
        };
    }
    #[macro_export]
    macro_rules! TURBOSHAFT_TRACE_TYPING_FAIL {
        ($fmt:expr, $($arg:tt)*) => {
            //eprintln!("TRACE_FAIL: {}", format!($fmt, $($arg)*)); // Use eprintln or a logging crate
        };
    }
}

// Placeholder for turboshaft/type-inference-analysis.h
pub struct TypeInferenceAnalysis {
    modifiable_input_graph: *mut Graph,
    phase_zone: *const Zone,
}

impl TypeInferenceAnalysis {
    pub fn new(modifiable_input_graph: *mut Graph, phase_zone: *const Zone) -> Self {
        TypeInferenceAnalysis {
            modifiable_input_graph,
            phase_zone,
        }
    }
    pub fn run<T>(&mut self, refinements: *mut T) -> GrowingOpIndexSidetable<Type> {
        unimplemented!()
    }
}

// Placeholder for turboshaft/typer.h
pub mod typer {
    use super::*;
    pub struct BranchRefinements<F, G>
    where
        F: Fn(OpIndex) -> Type,
        G: Fn(OpIndex, Type),
    {
        getter: F,
        setter: G,
    }

    impl<F, G> BranchRefinements<F, G>
    where
        F: Fn(OpIndex) -> Type,
        G: Fn(OpIndex, Type),
    {
        pub fn new(getter: F, setter: G) -> Self {
            BranchRefinements { getter, setter }
        }

        pub fn refine_types(&mut self, condition: &Operation, then_branch: bool, graph_zone: &Zone) {
            // Implement the type refinement logic based on the condition.
            // This is a placeholder.
            // Example: if the condition is a comparison, we can refine the types of the inputs
            // based on the comparison result.
            // if let Some(comparison) = condition.try_cast::<ComparisonOp>() {
            //     let left_type = (self.getter)(comparison.left);
            //     let right_type = (self.getter)(comparison.right);
            //     // Refine types based on the comparison and the branch taken
            //     // (self.setter)(comparison.left, refined_type_left);
            //     // (self.setter)(comparison.right, refined_type_right);
            // }
        }
    }

    pub fn type_for_representation(rep: Vec<RegisterRepresentation>, zone: &Zone) -> Type {
        // Placeholder implementation - return a default type based on the representation
        match rep.get(0) {
            Some(RegisterRepresentation::Word32) => Type::SignedSmall(),
            Some(RegisterRepresentation::Float64) => Type::Number(),
            _ => Type::Any(),
        }
    }

    pub fn type_constant(kind: ConstantOpKind, value: ConstantOpStorage) -> Type {
        match kind {
            ConstantOpKind::Int32 => Type::SignedSmall(),
            ConstantOpKind::Float64 => Type::Number(),
        }
    }

    pub fn type_comparison(
        left_type: Type,
        right_type: Type,
        rep: RegisterRepresentation,
        kind: ComparisonOpKind,
        graph_zone: &Zone,
    ) -> Type {
        Type::Boolean()
    }

    pub fn type_projection(input_type: Type, idx: u16) -> Type {
        Type::Any()
    }

    pub fn type_word_binop(
        left_type: Type,
        right_type: Type,
        kind: WordBinopKind,
        rep: WordRepresentation,
        graph_zone: &Zone,
    ) -> Type {
        Type::SignedSmall()
    }

    pub fn type_overflow_checked_binop(
        left_type: Type,
        right_type: Type,
        kind: OverflowCheckedBinopKind,
        rep: WordRepresentation,
        graph_zone: &Zone,
    ) -> Type {
        Type::SignedSmall()
    }

    pub fn type_float_binop(
        left_type: Type,
        right_type: Type,
        kind: FloatBinopKind,
        rep: FloatRepresentation,
        graph_zone: &Zone,
    ) -> Type {
        Type::Number()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOpKind {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordBinopKind {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowCheckedBinopKind {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatBinopKind {
    Add,
    Sub,
}

// Placeholder for turboshaft/types.h
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Invalid(),
    None(),
    Any(),
    SignedSmall(),
    Number(),
    Boolean(),
    Tuple(TupleType),
}

impl Type {
    pub fn is_invalid(&self) -> bool {
        match self {
            Type::Invalid() => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Type::None() => true,
            _ => false,
        }
    }
    pub fn is_subtype_of(&self, other: &Type) -> bool {
        // Placeholder implementation
        self == other || *self == Type::None() || *other == Type::Any()
    }
    pub fn least_upper_bound(&self, other: &Type, zone: &Zone) -> Type {
        // Placeholder implementation
        if self == other {
            self.clone()
        } else {
            Type::Any()
        }
    }
    pub fn print_to(&self, stream: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(stream, "{:?}", self)
    }

    pub fn tuple(types: Vec<Type>, zone: &Zone) -> Type {
        Type::Tuple(TupleType { types })
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Type::Boolean() => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Invalid() => write!(f, "Invalid"),
            Type::None() => write!(f, "None"),
            Type::Any() => write!(f, "Any"),
            Type::SignedSmall() => write!(f, "SignedSmall"),
            Type::Number() => write!(f, "Number"),
            Type::Boolean() => write!(f, "Boolean"),
            Type::Tuple(t) => write!(f, "Tuple: {:?}", t.types),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType {
    pub types: Vec<Type>,
}

impl TupleType {
    pub fn tuple(types: Vector<Type>, zone: &Zone) -> Type {
        Type::Tuple(TupleType { types })
    }
}

// Placeholder for turboshaft/uniform-reducer-adapter.h
pub trait Reducer<T> {
    fn reduce(&mut self, arg: T) -> OpIndex;
}

pub struct UniformReducerAdapter<R, N> {
    next: N,
    _reducer: PhantomData<R>,
}

impl<R, N> UniformReducerAdapter<R, N> {
    pub fn next(&self) -> &N {
        &self.next
    }
    pub fn next_mut(&mut self) -> &mut N {
        &mut self.next
    }
}

// Placeholder for V (Value)
#[derive(Debug, Clone, Copy)]
pub struct V<T> {
    index: OpIndex,
    _phantom: PhantomData<T>,
}

impl<T> V<T> {
    pub fn new(index: OpIndex) -> Self {
        V {
            index,
            _phantom: PhantomData,
        }
    }
    pub fn index(&self) -> OpIndex {
        self.index
    }
}

impl<T> Deref for V<T> {
    type Target = OpIndex;
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl<T> DerefMut for V<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.index
    }
}

// Define assembler macros (placeholders)
macro_rules! V {
    ($type:ident) => {
        V<$type>
    };
}

// Define assembler macros (placeholders)
macro_rules! REDUCE {
    ($name:ident) => {
        fn reduce_##$name(&mut self, arg: i32) -> OpIndex {
            // Default implementation - replace with actual logic
            println!("REDUCE({}) with arg: {}", stringify!($name), arg);
            OpIndex::invalid()
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpIndex {
    id: u32,
}

impl OpIndex {
    pub fn new(id: u32) -> Self {
        OpIndex { id }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn valid(&self) -> bool {
        self.id != std::u32::MAX
    }
    pub fn invalid() -> Self {
        OpIndex { id: std::u32::MAX }
    }
}

impl Display for OpIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpIndex({})", self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockIndex {
    id: u32,
}

impl BlockIndex {
    pub fn new(id: u32) -> Self {
        BlockIndex { id }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn valid(&self) -> bool {
        self.id != std::u32::MAX
    }
    pub fn invalid() -> Self {
        BlockIndex { id: std::u32::MAX }
    }
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockIndex({})", self.id)
    }
}

// Placeholder for Graph
pub struct Graph {
    operation_types: GrowingOpIndexSidetable<Type>,
}

impl Graph {
    pub fn get(&self, index: OpIndex) -> &Operation {
        unimplemented!()
    }

    pub fn index(&self, op: &BranchOp) -> OpIndex {
        unimplemented!()
    }

    pub fn block_count(&self) -> usize {
        unimplemented!()
    }

    pub fn block_type_refinement(&mut self) -> &mut GrowingBlockSidetable<std::vec::Vec<(OpIndex, Type)>> {
        unimplemented!()
    }

    pub fn operation_types(&mut self) -> &mut GrowingOpIndexSidetable<Type> {
        &mut self.operation_types
    }
}

// Placeholder for Block
pub struct Block {
    index: BlockIndex,
}

impl Block {
    pub fn index(&self) -> BlockIndex {
        self.index
    }
    pub fn predecessors_iterable(&self) -> Vec<&Block> {
        unimplemented!()
    }
    pub fn predecessor_count(&self) -> usize {
        unimplemented!()
    }
    pub fn last_predecessor(&self) -> &Block {
        unimplemented!()
    }
    pub fn last_operation(&self, graph: &Graph) -> &Operation {
        unimplemented!()
    }
}

// Placeholder for Zone
pub struct Zone {}

impl Zone {
    pub fn default_type(&self) -> Type {
        Type::Invalid()
    }
}

pub struct TypeInferenceReducerArgs {
    pub input_graph_typing: InputGraphTyping,
    pub output_graph_typing: OutputGraphTyping,
}

impl TypeInferenceReducerArgs {
    pub fn new(input_graph_typing: InputGraphTyping, output_graph_typing: OutputGraphTyping) -> Self {
        TypeInferenceReducerArgs {
            input_graph_typing,
            output_graph_typing,
        }
    }

    pub fn get() -> &'static TypeInferenceReducerArgs {
        &DEFAULT_TYPE_INFERENCE_REDUCER_ARGS
    }
}

static DEFAULT_TYPE_INFERENCE_REDUCER_ARGS: TypeInferenceReducerArgs = TypeInferenceReducerArgs {
    input_graph_typing: InputGraphTyping::KNone,
    output_graph_typing: OutputGraphTyping::KNone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputGraphTyping {
    KNone,
    KPrecise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputGraphTyping {
    KNone,
    KPreserveFromInputGraph,
    KRefineFromInputGraph,
}

struct NextIsBottomOfAssemblerStack {
    value: bool,
}

impl NextIsBottomOfAssemblerStack {
    const fn new(value: bool) -> Self {
        Self { value }
    }
}

impl NextIsBottomOfAssemblerStack {
    const VALUE: Self = Self::new(true);
}

trait TurboshaftReducerBoilerplate {
    fn turboshaft_reducer_boilerplate() -> Self;
}

macro_rules! impl_turboshaft_reducer_boilerplate {
    ($struct_name:ident) => {
        impl TurboshaftReducerBoilerplate for $struct_name {
            fn turboshaft_reducer_boilerplate() -> Self {
                Self {}
            }
        }
    };
}

// Example implementation for a bottom-level reducer
#[derive(Default)]
struct BottomReducer {}

impl BottomReducer {
    fn reduce_pending_loop_phi(&mut self, first: OpIndex, rep: RegisterRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_phi(&mut self, inputs: Vec<OpIndex>, rep: RegisterRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_constant(&mut self, kind: ConstantOpKind, value: ConstantOpStorage) -> OpIndex {
        unimplemented!()
    }
    fn reduce_comparison(&mut self, left: OpIndex, right: OpIndex, kind: ComparisonOpKind, rep: RegisterRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_projection(&mut self, input: OpIndex, idx: u16, rep: RegisterRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_word_binop(&mut self, left: OpIndex, right: OpIndex, kind: WordBinopKind, rep: WordRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_overflow_checked_binop(&mut self, left: OpIndex, right: OpIndex, kind: OverflowCheckedBinopKind, rep: WordRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_float_binop(&mut self, left: OpIndex, right: OpIndex, kind: FloatBinopKind, rep: FloatRepresentation) -> OpIndex {
        unimplemented!()
    }
    fn reduce_check_turboshaft_type_of(&mut self, input: OpIndex, rep: RegisterRepresentation, type_: Type, successful: bool) -> OpIndex {
        unimplemented!()
    }

    fn remove_last(&mut self, index_of_last_operation: OpIndex) {
        unimplemented!()
    }
}

struct DefaultContinuation<'a, T> {
    reducer: &'a mut T,
}

impl<'a, T> DefaultContinuation<'a, T> {
    pub fn new(reducer: &'a mut T) -> Self {
        DefaultContinuation { reducer }
    }
}

impl_turboshaft_reducer_boilerplate!(BottomReducer);

const next_is_bottom_of_assembler_stack: NextIsBottomOfAssemblerStack = NextIsBottomOfAssemblerStack::VALUE;

macro_rules! TURBOSHAFT_REDUCER_BOILERPLATE {
    ($name:ident) => {
        fn turboshaft_reducer_name() -> String {
            String::from(stringify!($name))
        }
    };
}

// NOTE: The TypeInferenceReducer has to be the last reducer in the stack!
pub struct TypeInferenceReducer<Next> {
    next: Next,
    args_: &'static TypeInferenceReducerArgs,
    input_graph_types_: GrowingOpIndexSidetable<Type>,
    output_graph_types_: GrowingOpIndexSidetable<Type>,
    table_: SnapshotTable<Type>,
    current_block_: *const Block,
    op_to_key_mapping_: GrowingOpIndexSidetable<Option<Key>>,
    block_to_snapshot_mapping_: GrowingBlockSidetable<Option<Snapshot>>,
    predecessors_: Vec<Snapshot>,
    analyzer_: TypeInferenceAnalysis,
    assembler: Rc<Assembler>, // Added Assembler
}

impl<Next> TypeInferenceReducer<Next> {
    pub fn new(next: Next, assembler: Rc<Assembler>) -> Self {
        let args_ = TypeInferenceReducerArgs::get();

        DCHECK_IMPLIES(
            args_.output_graph_typing == OutputGraphTyping::KPreserveFromInputGraph,
            args_.input_graph_typing != InputGraphTyping::KNone,
        );

        let mut output_graph = assembler.output_graph();
        let input_graph_types_ = GrowingOpIndexSidetable::new(
            assembler.phase_zone(),
            assembler.input_graph(),
            Type::Invalid(),
        );
        let output_graph_types_ = GrowingOpIndexSidetable::new(
            assembler.phase_zone(),
            assembler.output_graph(),
            Type::Invalid(),
        );

        let op_to_key_mapping_ = GrowingOpIndexSidetable::new(
            assembler.phase_zone(),
            assembler.output_graph(),
            None,
        );

        let block_to_snapshot_mapping_ = GrowingBlockSidetable::new(
            assembler.input_graph().block_count(),
            None,
            assembler.phase_zone(),
        );

        let predecessors_: Vec<Snapshot> = Vec::new();
        let analyzer_ = TypeInferenceAnalysis::new(
            assembler.modifiable_input_graph(),
            assembler.phase_zone(),
        );

        TypeInferenceReducer {
            next,
            args_,
            input_graph_types_,
            output_graph_types_,
            table_: SnapshotTable::new(assembler.phase_zone()),
            current_block_: std::ptr::null(),
            op_to_key_mapping_,
            block_to_snapshot_mapping_,
            predecessors_,
            analyzer_,
            assembler,
        }
    }

    pub fn asm(&self) -> &Assembler {
        &self.assembler
    }

    pub fn next(&self) -> &Next {
        &self.next
    }
}

impl<Next> TypeInferenceReducer<Next> {
    pub fn analyze(&mut self) {
        if self.args_.input_graph_typing == InputGraphTyping::KPrecise {
            let input_graph_types_ = if cfg!(debug_assertions) {
                let mut block_refinements: GrowingBlockSidetable<Vec<(OpIndex, Type)>> =
                    GrowingBlockSidetable::new(
                        self.asm().input_graph().block_count(),
                        Vec::new(),
                        self.asm().phase_zone(),
                    );
                let input_graph_types_ = self.analyzer_.run(&mut block_refinements);

                tracing::Tracing::get().print_per_block_data(
                    "Type Refinements",
                    self.asm().input_graph(),
                    |stream, graph, index| -> bool {
                        let refinements = &block_refinements[index];
                        if refinements.is_empty() {
                            return false;
                        }
                        //stream.push_str("\\n");
                        for (op, type_) in refinements {
                            //stream.push_str(format!("{} : {}\\n", op, type_).as_str());
                        }
                        true
                    },
                );
                input_graph_types_
            } else {
                self.analyzer_.run::<()>(std::ptr::null_mut())
            };

            tracing::Tracing::get().print_per_operation_data(
                "Types",
                self.asm().input_graph(),
                |stream, graph, index| -> bool {
                    let type_ = self.input_graph_types_[index];
                    if !type_.is_invalid() && !type_.is_none() {
                        //type_.print_to(stream);
                        return true;
                    }
                    false
                },
            );
        }

        //self.next.analyze();
    }

    pub fn get_input_graph_type(&self, ig_index: OpIndex) -> Type {
        self.input_graph_types_[ig_index]
    }

    pub fn get_output_graph_type(&self, og_index: OpIndex) -> Type {
        self.get_type(og_index)
    }
}

// Example implementation for a generic reducer method
impl<Next> TypeInferenceReducer<Next>
where
    Next: BottomTrait,
{
    fn reduce_operation<Continuation, Ts, Opcode>(&mut self, args: Ts) -> OpIndex
    where
        Continuation: FnOnce(&mut Self, Ts) -> OpIndex,
    {
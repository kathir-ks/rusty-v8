// Converted from V8 C++ source files:
// Header: type-inference-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod define_assembler_macros {
}
pub mod undef_assembler_macros {
}
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crate::V8;
use crate::AbortReason;
use crate::TypeCache;
use crate::Args;
use crate::If;
use crate::OpIndex;
use crate::RegisterRepresentation;
use crate::WordRepresentation;
use crate::FloatRepresentation;
use crate::Type;
use crate::internal::compiler::turboshaft::csa_optimize_phase::MaybeIndirectHandle;
use crate::internal::compiler::turboshaft::operations::Code;
use crate::internal::execution::isolate::Isolate;
use crate::internal::compiler::turboshaft::loop_finder::Node;
use crate::internal::compiler::js_heap_broker::JSHeapBroker;
use crate::internal::compiler::turboshaft::phase::PipelineData;
use crate::internal::compiler::turboshaft::wasm_dead_code_elimination_phase::WasmDeadCodeEliminationPhaseError;
use crate::internal::compiler::turboshaft::loop_finder::BlockIndex;
use crate::internal::compiler::turboshaft::wasm_revec_reducer::NodeGroup;
use crate::internal::compiler::control_equivalence::Operator;
use crate::internal::compiler::turboshaft::operations::Block;
use crate::internal::zone::Zone;
use crate::internal::zone::ZoneSnapshot;
use crate::internal::compiler::code_assembler::Context;
use crate::internal::compiler::code_assembler::Local;
use crate::internal::compiler::code_assembler::MaybeLocal;
use crate::internal::compiler::code_assembler::Value;
use crate::internal::compiler::machine_operator::AtomicMemoryOrder;
use crate::internal::compiler::turboshaft::operations::Operation;
use crate::internal::compiler::turboshaft::graph_visualizer::V;
use crate::internal::compiler::turboshaft::operations::ConstantOp;
use crate::internal::compiler::turboshaft::operations::ComparisonOp;
use crate::internal::compiler::turboshaft::operations::WordBinopOp;
use crate::internal::compiler::turboshaft::operations::OverflowCheckedBinopOp;
use crate::internal::compiler::turboshaft::operations::FloatBinopOp;
use crate::internal::compiler::turboshaft::operations::TupleOp;
use crate::internal::compiler::script_origin::ScriptOriginOptions;
use crate::internal::compiler::fast_api_calls::Int64Representation;
use crate::internal::compiler::fast_api_calls::CFunction;
use crate::internal::asmjs::asm_js::StandardMember;
use crate::internal::compiler::loop_unrolling::TupleType;
pub struct GrowingOpIndexSidetable<T> {
    table: RefCell<HashMap<OpIndex, T>>,
    zone: *mut Zone,
    graph: *const crate::internal::compiler::turboshaft::graph::Graph,
}

impl<T> GrowingOpIndexSidetable<T>
where
    T: Clone,
{
    pub fn new(zone: *mut Zone, graph: *const crate::internal::compiler::turboshaft::graph::Graph) -> Self {
        GrowingOpIndexSidetable {
            table: RefCell::new(HashMap::new()),
            zone,
            graph,
        }
    }

    pub fn resize(&mut self, new_size: usize, default_value: T) {
        let mut table = self.table.borrow_mut();
        for i in table.len()..new_size {
            let op_index = OpIndex { id: i as u32 };
            table.insert(op_index, default_value.clone());
        }
    }

    pub fn get(&self, index: OpIndex) -> Option<T> {
        let table = self.table.borrow();
        table.get(&index).cloned()
    }

    pub fn set(&self, index: OpIndex, value: T) {
        let mut table = self.table.borrow_mut();
        table.insert(index, value);
    }

    pub fn len(&self) -> usize {
        self.table.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.borrow().is_empty()
    }
}

impl<T> std::ops::Index<OpIndex> for GrowingOpIndexSidetable<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: OpIndex) -> &Self::Output {
        let table = self.table.borrow();
        table.get(&index).unwrap()
    }
}

impl<T> std::ops::IndexMut<OpIndex> for GrowingOpIndexSidetable<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
        let mut table = self.table.borrow_mut();
        table.get_mut(&index).unwrap()
    }
}

pub struct GrowingBlockSidetable<T> {
    table: RefCell<HashMap<BlockIndex, T>>,
    default_value: T,
    zone: *mut Zone,
}

impl<T> GrowingBlockSidetable<T>
where
    T: Clone,
{
    pub fn new(block_count: usize, default_value: T, zone: *mut Zone) -> Self {
        GrowingBlockSidetable {
            table: RefCell::new(HashMap::new()),
            default_value,
            zone,
        }
    }
}

impl<T> GrowingBlockSidetable<T>
where
    T: Clone,
{
    pub fn with_default(default_value: T, zone: *mut Zone) -> Self {
        GrowingBlockSidetable {
            table: RefCell::new(HashMap::new()),
            default_value,
            zone,
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        let mut table = self.table.borrow_mut();
        for i in table.len()..new_size {
            let block_index = BlockIndex { id: i as u32 };
            table.insert(block_index, self.default_value.clone());
        }
    }

    pub fn get(&self, index: BlockIndex) -> T {
        let table = self.table.borrow();
        table.get(&index).cloned().unwrap_or(self.default_value.clone())
    }

    pub fn set(&self, index: BlockIndex, value: T) {
        let mut table = self.table.borrow_mut();
        table.insert(index, value);
    }
}

impl<T> std::ops::Index<BlockIndex> for GrowingBlockSidetable<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: BlockIndex) -> &Self::Output {
        let table = self.table.borrow();
        if let Some(value) = table.get(&index) {
            value
        } else {
            panic!("BlockIndex out of bounds");
        }
    }
}

impl<T> std::ops::IndexMut<BlockIndex> for GrowingBlockSidetable<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: BlockIndex) -> &mut Self::Output {
        let mut table = self.table.borrow_mut();
        table.entry(index).or_insert(self.default_value.clone())
    }
}

pub fn fast_hash_combine<T, Ts>(v: &T, vs: Ts) -> usize {
    42
}

#[derive(Debug, Clone)]
pub enum TypeInferenceReducerArgsError {
    InvalidInput,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TypeInferenceReducerArgs {
    pub input_graph_typing: TypeInferenceReducerArgsInputGraphTyping,
    pub output_graph_typing: TypeInferenceReducerArgsOutputGraphTyping,
}

impl TypeInferenceReducerArgs {
    pub fn new(
        input_graph_typing: TypeInferenceReducerArgsInputGraphTyping,
        output_graph_typing: TypeInferenceReducerArgsOutputGraphTyping,
    ) -> Self {
        TypeInferenceReducerArgs {
            input_graph_typing,
            output_graph_typing,
        }
    }
    pub fn Get() -> Self {
        TypeInferenceReducerArgs {
            input_graph_typing: TypeInferenceReducerArgsInputGraphTyping::kNone,
            output_graph_typing: TypeInferenceReducerArgsOutputGraphTyping::kNone,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeInferenceReducerArgsInputGraphTyping {
    kNone,
    kPrecise,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeInferenceReducerArgsOutputGraphTyping {
    kNone,
    kPreserveFromInputGraph,
    kRefineFromInputGraph,
}

pub trait NextTrait {
    fn Analyze(&mut self) {}
    fn ReducePendingLoopPhi(&mut self, first: OpIndex, rep: RegisterRepresentation) -> OpIndex;
    fn ReducePhi(&mut self, inputs: &[OpIndex], rep: RegisterRepresentation) -> OpIndex;
    fn ReduceConstant(&mut self, kind: ConstantOp::Kind, value: ConstantOp::Storage) -> OpIndex;
    fn ReduceComparison(
        &mut self,
        left: V<Any>,
        right: V<Any>,
        kind: ComparisonOp::Kind,
        rep: RegisterRepresentation,
    ) -> OpIndex;
    fn ReduceProjection(
        &mut self,
        input: V<Any>,
        idx: u16,
        rep: RegisterRepresentation,
    ) -> V<Any>;
    fn ReduceWordBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: WordBinopOp::Kind,
        rep: WordRepresentation,
    ) -> V<Word>;
    fn ReduceOverflowCheckedBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: OverflowCheckedBinopOp::Kind,
        rep: WordRepresentation,
    ) -> OpIndex;
    fn ReduceFloatBinop(
        &mut self,
        left: V<Float>,
        right: V<Float>,
        kind: FloatBinopOp::Kind,
        rep: FloatRepresentation,
    ) -> V<Float>;
    fn ReduceCheckTurboshaftTypeOf(
        &mut self,
        input: OpIndex,
        rep: RegisterRepresentation,
        type_: Type,
        successful: bool,
    ) -> OpIndex;
    fn RemoveLast(&mut self, index_of_last_operation: OpIndex);
    fn Bind(&mut self, new_block: *mut Block);
    fn ReduceInputGraph(&mut self, ig_index: OpIndex, operation: &Operation) -> OpIndex;
    fn current_block(&self) -> *mut Block;
    fn output_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph;
    fn phase_zone(&self) -> *mut Zone;
    fn modifiable_input_graph(&mut self) -> &mut crate::internal::compiler::turboshaft::graph::Graph;
    fn input_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph;
    fn graph_zone(&self) -> *mut Zone;
    fn output_graph_operation_types(&mut self) -> &mut GrowingOpIndexSidetable<Type>;
}

struct BottomOfAssemblerStack {}

impl NextTrait for BottomOfAssemblerStack {
    fn ReducePendingLoopPhi(&mut self, _first: OpIndex, _rep: RegisterRepresentation) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn ReducePhi(&mut self, _inputs: &[OpIndex], _rep: RegisterRepresentation) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn ReduceConstant(&mut self, _kind: ConstantOp::Kind, _value: ConstantOp::Storage) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn ReduceComparison(
        &mut self,
        _left: V<Any>,
        _right: V<Any>,
        _kind: ComparisonOp::Kind,
        _rep: RegisterRepresentation,
    ) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn ReduceProjection(
        &mut self,
        _input: V<Any>,
        _idx: u16,
        _rep: RegisterRepresentation,
    ) -> V<Any> {
        V { id: 0 }
    }
    fn ReduceWordBinop(
        &mut self,
        _left: V<Word>,
        _right: V<Word>,
        _kind: WordBinopOp::Kind,
        _rep: WordRepresentation,
    ) -> V<Word> {
        V { id: 0 }
    }
    fn ReduceOverflowCheckedBinop(
        &mut self,
        _left: V<Word>,
        _right: V<Word>,
        _kind: OverflowCheckedBinopOp::Kind,
        _rep: WordRepresentation,
    ) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn ReduceFloatBinop(
        &mut self,
        _left: V<Float>,
        _right: V<Float>,
        _kind: FloatBinopOp::Kind,
        _rep: FloatRepresentation,
    ) -> V<Float> {
        V { id: 0 }
    }
    fn ReduceCheckTurboshaftTypeOf(
        &mut self,
        _input: OpIndex,
        _rep: RegisterRepresentation,
        _type_: Type,
        _successful: bool,
    ) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn RemoveLast(&mut self, _index_of_last_operation: OpIndex) {}
    fn Bind(&mut self, _new_block: *mut Block) {}
    fn ReduceInputGraph(&mut self, _ig_index: OpIndex, _operation: &Operation) -> OpIndex {
        OpIndex { id: 0 }
    }
    fn current_block(&self) -> *mut Block {
        std::ptr::null_mut()
    }
    fn output_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph {
        panic!("output_graph() called on BottomOfAssemblerStack");
    }
    fn phase_zone(&self) -> *mut Zone {
        std::ptr::null_mut()
    }
    fn modifiable_input_graph(&mut self) -> &mut crate::internal::compiler::turboshaft::graph::Graph {
        panic!("modifiable_input_graph() called on BottomOfAssemblerStack");
    }
    fn input_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph {
        panic!("input_graph() called on BottomOfAssemblerStack");
    }
    fn graph_zone(&self) -> *mut Zone {
        std::ptr::null_mut()
    }
    fn output_graph_operation_types(&mut self) -> &mut GrowingOpIndexSidetable<Type> {
        panic!("output_graph_operation_types() called on BottomOfAssemblerStack");
    }
}

struct UniformReducerAdapter<R, N> {
    reducer: R,
    next: N,
}

impl<R, N> UniformReducerAdapter<R, N> {
    fn new(reducer: R, next: N) -> Self {
        UniformReducerAdapter { reducer, next }
    }
}

trait ReducerTrait {
    fn Analyze(&mut self) {}
    fn ReducePendingLoopPhi(&mut self, first: OpIndex, rep: RegisterRepresentation) -> OpIndex;
    fn ReducePhi(&mut self, inputs: &[OpIndex], rep: RegisterRepresentation) -> OpIndex;
    fn ReduceConstant(&mut self, kind: ConstantOp::Kind, value: ConstantOp::Storage) -> OpIndex;
    fn ReduceComparison(
        &mut self,
        left: V<Any>,
        right: V<Any>,
        kind: ComparisonOp::Kind,
        rep: RegisterRepresentation,
    ) -> OpIndex;
    fn ReduceProjection(
        &mut self,
        input: V<Any>,
        idx: u16,
        rep: RegisterRepresentation,
    ) -> V<Any>;
    fn ReduceWordBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: WordBinopOp::Kind,
        rep: WordRepresentation,
    ) -> V<Word>;
    fn ReduceOverflowCheckedBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: OverflowCheckedBinopOp::Kind,
        rep: WordRepresentation,
    ) -> OpIndex;
    fn ReduceFloatBinop(
        &mut self,
        left: V<Float>,
        right: V<Float>,
        kind: FloatBinopOp::Kind,
        rep: FloatRepresentation,
    ) -> V<Float>;
    fn ReduceCheckTurboshaftTypeOf(
        &mut self,
        input: OpIndex,
        rep: RegisterRepresentation,
        type_: Type,
        successful: bool,
    ) -> OpIndex;
    fn RemoveLast(&mut self, index_of_last_operation: OpIndex);
    fn Bind(&mut self, new_block: *mut Block);
    fn ReduceInputGraph(&mut self, ig_index: OpIndex, operation: &Operation) -> OpIndex;
}

impl<R, N> NextTrait for UniformReducerAdapter<R, N>
where
    R: ReducerTrait,
    N: NextTrait,
{
    fn Analyze(&mut self) {
        self.reducer.Analyze();
        self.next.Analyze();
    }
    fn ReducePendingLoopPhi(&mut self, first: OpIndex, rep: RegisterRepresentation) -> OpIndex {
        self.reducer.ReducePendingLoopPhi(first, rep)
    }
    fn ReducePhi(&mut self, inputs: &[OpIndex], rep: RegisterRepresentation) -> OpIndex {
        self.reducer.ReducePhi(inputs, rep)
    }
    fn ReduceConstant(&mut self, kind: ConstantOp::Kind, value: ConstantOp::Storage) -> OpIndex {
        self.reducer.ReduceConstant(kind, value)
    }
    fn ReduceComparison(
        &mut self,
        left: V<Any>,
        right: V<Any>,
        kind: ComparisonOp::Kind,
        rep: RegisterRepresentation,
    ) -> OpIndex {
        self.reducer.ReduceComparison(left, right, kind, rep)
    }
    fn ReduceProjection(
        &mut self,
        input: V<Any>,
        idx: u16,
        rep: RegisterRepresentation,
    ) -> V<Any> {
        self.reducer.ReduceProjection(input, idx, rep)
    }
    fn ReduceWordBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: WordBinopOp::Kind,
        rep: WordRepresentation,
    ) -> V<Word> {
        self.reducer.ReduceWordBinop(left, right, kind, rep)
    }
    fn ReduceOverflowCheckedBinop(
        &mut self,
        left: V<Word>,
        right: V<Word>,
        kind: OverflowCheckedBinopOp::Kind,
        rep: WordRepresentation,
    ) -> OpIndex {
        self.reducer.ReduceOverflowCheckedBinop(left, right, kind, rep)
    }
    fn ReduceFloatBinop(
        &mut self,
        left: V<Float>,
        right: V<Float>,
        kind: FloatBinopOp::Kind,
        rep: FloatRepresentation,
    ) -> V<Float> {
        self.reducer.ReduceFloatBinop(left, right, kind, rep)
    }
    fn ReduceCheckTurboshaftTypeOf(
        &mut self,
        input: OpIndex,
        rep: RegisterRepresentation,
        type_: Type,
        successful: bool,
    ) -> OpIndex {
        self.reducer.ReduceCheckTurboshaftTypeOf(input, rep, type_, successful)
    }
    fn RemoveLast(&mut self, index_of_last_operation: OpIndex) {
        self.reducer.RemoveLast(index_of_last_operation)
    }
    fn Bind(&mut self, new_block: *mut Block) {
        self.reducer.Bind(new_block)
    }
    fn ReduceInputGraph(&mut self, ig_index: OpIndex, operation: &Operation) -> OpIndex {
        self.reducer.ReduceInputGraph(ig_index, operation)
    }
    fn current_block(&self) -> *mut Block {
        self.next.current_block()
    }
    fn output_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph {
        self.next.output_graph()
    }
    fn phase_zone(&self) -> *mut Zone {
        self.next.phase_zone()
    }
    fn modifiable_input_graph(&mut self) -> &mut crate::internal::compiler::turboshaft::graph::Graph {
        self.next.modifiable_input_graph()
    }
    fn input_graph(&self) -> &crate::internal::compiler::turboshaft::graph::Graph {
        self.next.input_graph()
    }
    fn graph_zone(&self) -> *mut Zone {
        self.next.graph_zone()
    }
    fn output_graph_operation_types(&mut self) -> &mut GrowingOpIndexSidetable<Type> {
        self.next.output_graph_operation_types()
    }
}

struct TypeInferenceReducer<Next> {
    adapter: UniformReducerAdapter<Self, Next>,
    args_: TypeInferenceReducerArgs,
    input_graph_types_: GrowingOpIndexSidetable<Type>,
    table_: SnapshotTable<Type>,
    current_block_: *const Block,
    op_to_key_mapping_: GrowingOpIndexSidetable<std::option::Option<SnapshotTableKey>>,
    block_to_snapshot_mapping_: GrowingBlockSidetable<std::option::Option<SnapshotTableSnapshot>>,
    predecessors_: ZoneVector<SnapshotTableSnapshot>,
    analyzer_: TypeInferenceAnalysis,
}

impl<Next> TypeInferenceReducer<Next>
where
    Next: NextTrait,
{
    pub fn new(next: Next) -> Self {
        let args_ = TypeInferenceReducerArgs::Get();
        let asm = Asm {};
        let graph_zone = asm.graph_zone();
        let phase_zone = asm.phase_zone();
        let input_graph = asm.input_graph();
        let output_graph = asm.output_graph();
        let modifiable_input_graph = asm.modifiable_input_graph();
        let input_graph_types_ = GrowingOpIndexSidetable::new(phase_zone, input_graph);
        let op_to_key_mapping_ = GrowingOpIndexSidetable::new(phase_zone, output_graph);
        let output_graph_types_ = asm.output_graph_operation_types();
        let block_to_snapshot_mapping_ = GrowingBlockSidetable::new(input_graph.block_count(), None, phase_zone);
        let predecessors_ = ZoneVector::new(phase_zone);
        let analyzer_ = TypeInferenceAnalysis::new(modifiable_input_graph, phase_zone);

        let reducer = Self {
            adapter: UniformReducerAdapter::new(Self{}, next),
            args_,
            input_graph_types_,
            table_: SnapshotTable::new(phase_zone),
            current_block_: std::ptr::null(),
            op_to_key_mapping_,
            block_to_snapshot_mapping_,
            predecessors_,
            analyzer_,
        };

        if reducer.args_.output_graph_typing == TypeInferenceReducerArgsOutputGraphTyping::kPreserveFromInputGraph {
            assert_ne!(reducer.args_.input_graph_typing, TypeInferenceReducerArgsInputGraphTyping::kNone);
        }
        reducer
    }

    fn Analyze(&mut self) {
        if self.args_.input_graph_typing == TypeInferenceReducerArgsInputGraphTyping::kPrecise {
            let mut block_refinements = GrowingBlockSidetable::new(
                self.Asm().input_graph().block_count(),
                Vec::<(OpIndex, Type)>::new(),
                self.Asm().phase_zone(),
            );

            self.input_graph_types_ = self.analyzer_.Run(&mut block_refinements);

            Tracing::Get().PrintPerBlockData(
                "Type Refinements",
                self.Asm().input_graph(),
                |stream: &mut std::fmt::Write,
                 graph: &turboshaft::Graph,
                 index: turboshaft::BlockIndex|
                 -> bool {
                    let refinements = &block_refinements[index];
                    if refinements.is_empty() {
                        return false;
                    }
                    stream.write_str("\\n").unwrap();
                    for (op, type_) in refinements {
                        stream.write_fmt(format_args!("{} : {}\\n", op, type_)).unwrap();
                    }
                    true
                },
            );

            Tracing::Get().PrintPerOperationData(
                "Types",
                self.Asm().input_graph(),
                |stream: &mut std::fmt::Write,
                 graph: &turboshaft::Graph,
                 index: turboshaft::OpIndex|
                 -> bool {
                    let type_ = self.input_graph_types_[index];
                    if !type_.IsInvalid() && !type_.IsNone() {
                        type_.PrintTo(stream);
                        return true;
                    }
                    false
                },
            );
        }
        self.adapter.next.Analyze();
    }

    fn GetInputGraphType(&self, ig_index: OpIndex) -> Type {
        self.input_graph_types_[ig_index]
    }

    fn GetOutputGraphType(&self, og_index: OpIndex) -> Type {
        self.GetType(og_index)
    }

    fn ReduceOperation<const OPCODE: u32, C, Ts>(
        &mut self,
        args: Ts,
    ) -> OpIndex
    where
        C: FnOnce(Ts) -> OpIndex,
    {
        let index = C(args);
        if !self.NeedsTyping(index) {
            return index;
        }

        let op = self.Asm().output_graph().Get(index);
        if CanBeTyped(op) {
            let type_ = Typer::TypeForRepresentation(
                self.Asm().output_graph().Get(index).outputs_rep(),
                self.Asm().graph_zone(),
            );
            self.SetType(index, type_, true);
        }
        index
    }

    fn ReduceInputGraphOperation<Op>(
        &mut self,
        ig_index: OpIndex,
        operation: &Op,
    ) -> OpIndex
    where
        Op: OperationTrait,
    {
        let og_index = self.adapter.next.ReduceInputGraph(ig_index, operation.as_operation());
        if !og_index.valid() {
            return og_index;
        }

        if self.args_.output_graph_typing == TypeInferenceReducerArgsOutputGraphTyping::kNone {
            return og_index;
        }
        if !CanBeTyped(operation) {
            return og_index;
        }

        let ig_type = self.GetInputGraphType(ig_index);
        assert_eq!(self.args_.input_graph_typing != TypeInferenceReducerArgsInputGraphTyping::kNone, !ig_type.IsInvalid());

        if !ig_type.IsInvalid() {
            let og_type = self.GetType(og_index);
            if og_type.IsInvalid() || (ig_type.IsSubtypeOf(og_type) && !og_type.IsSubtypeOf(ig_type)) {
                self.RefineTypeFromInputGraph(og_index, og_type, ig_type);
            }
        }

        og_index
    }

    fn Bind(&mut self, new_block: *mut Block) {
        self.adapter.next.Bind(new_block);

        if self.table_.IsSealed() {
            assert!(self.current_block_.is_null());
        } else {
            assert!(!self.current_block_.is_null());
            let current_block = unsafe { &*self.current_block_ };
            assert!(current_block.index().valid());
            self.block_to_snapshot_mapping_[current_block.index()] = Some(self.table_.Seal());
            self.current_block_ = std::ptr::null();
        }

        self.predecessors_.clear();
        let new_block_ref = unsafe { &*new_block };
        for pred in new_block_ref.PredecessorsIterable() {
            let pred_snapshot = self.block_to_snapshot_mapping_[pred.index()].unwrap();
            self.predecessors_.push(pred_snapshot);
        }
        self.predecessors_.reverse();

        let predecessors_slice = self.predecessors_.as_slice();

        let MergeTypes =
            |key: SnapshotTableKey, predecessors: &[Type]| -> Type {
                assert!(!predecessors.is_empty());
                let mut result_type = predecessors[0];
                for i in 1..predecessors.len() {
                    result_type = Type::LeastUpperBound(
                        result_type,
                        predecessors[i],
                        self.Asm().graph_zone(),
                    );
                }
                result_type
            };

        self.table_.StartNewSnapshot(
            predecessors_slice,
            MergeTypes,
        );

        if self.args_.output_graph_typing == TypeInferenceReducerArgsOutputGraphTyping::kRefineFromInputGraph {
            if new_block_ref.PredecessorCount() == 1 {
                let predecessor = new_block_ref.LastPredecessor();
                let terminator = predecessor.LastOperation(self.Asm().output_graph());

                if let Some(branch) = terminator.TryCast::<BranchOp>() {
                    assert!(branch.if_true == new_block_ref || branch.if_false == new_block_ref);
                    self.RefineTypesAfterBranch(
                        branch,
                        new_block,
                        branch.if_true == new_block_ref,
                    );
                }
            }
        }

        self.current_block_ = new_block;
    }

    fn RefineTypesAfterBranch(
        &mut self,
        branch: &BranchOp,
        new_block: *mut Block,
        then_branch: bool,
    ) {
        let branch_str = branch.ToString().chars().take(40).collect::<String>();
        println!("Br   {:3}:{:40}", self.Asm().output_graph().Index(branch).id(), branch_str);

        let mut refinements = Typer::BranchRefinements::new(
            |index: OpIndex| self.GetType(index),
            |index: OpIndex, refined_type: &Type| {
                self.RefineOperationType(
                    new_block,
                    index,
                    refined_type,
                    if then_branch { 'T' } else { 'F' },
                );
            },
        );

        let condition = self.Asm().output_graph().Get(branch.condition());
        refinements.RefineTypes(&condition, then_branch, self.Asm().graph_zone());
    }

    fn RefineOperationType(
        &mut self,
        new_block: *mut Block,
        op: OpIndex,
        type_: &Type,
        case_for_tracing: char,
    ) {
        assert!(op.valid());
        assert!(!type_.IsInvalid());

        println!(
            "  {}: {:3}:{:40} ~~> {}",
            case_for_tracing,
            op.id(),
            self.Asm().output_graph().Get(op).ToString().chars().take(40).collect::<String>(),
            type_.ToString()
        );

        if let Some(key_opt) = self.op_to_key_mapping_[op] {
            self.table_.Set(key_opt, *type_);

            let refinement = &mut self.Asm().output_graph().block_type_refinement()[unsafe { &*new_block }.index()];

            refinement.push((op, *type_));
        }
    }

    fn ReducePendingLoopPhi(&mut self, first: OpIndex, rep: RegisterRepresentation) -> OpIndex {
        let index = self.adapter.next.ReducePendingLoopPhi(first, rep);
        if !self.NeedsTyping(index) {
            return index;
        }

        self.SetType(index, Typer::TypeForRepresentation(rep), false);
        index
    }

    fn ReducePhi(&mut self, inputs: &[OpIndex], rep: RegisterRepresentation) -> OpIndex {
        let index = self.adapter.next.ReducePhi(inputs, rep);
        if !self.NeedsTyping(index) {
            return index;
        }

        let mut type_ = Type::None();
        for input in inputs {
            type_ = Type::LeastUpperBound(type_, self.GetType(*input), self.Asm().graph_zone());
        }
        self.SetType(index, type_, false);
        index
    }

    fn ReduceConstant(&mut self, kind: ConstantOp

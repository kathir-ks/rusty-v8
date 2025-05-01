// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.rs

use std::collections::HashMap;
use std::fmt;

// Assuming base, compiler, and wasm value type definitions exist
// and are accessible through these modules.  These would need to
// be defined based on the C++ code.
//
// Note: It's impossible to provide full Rust conversions without
// knowing the definitions for these V8 specific structs and enums.
// Placeholders are used, and these must be replaced with real
// definitions.

mod base {
    // Placeholder for base utilities.
    pub type Vector<T> = Vec<T>;
    pub type SmallVector<T, const N: usize> = Vec<T>;

    pub trait VectorOf<T> {
        fn of(data: &[T]) -> Self;
    }

    impl<T: Copy> VectorOf<T> for Vector<T> {
        fn of(data: &[T]) -> Self {
            data.to_vec()
        }
    }
}

mod compiler {
    use super::wasm;

    // Placeholder for compiler related structures.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OpIndex {
        id: u32,
    }

    impl OpIndex {
        pub fn id(&self) -> u32 {
            self.id
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BlockIndex {
        id: u32,
    }

    impl BlockIndex {
        pub fn id(&self) -> u32 {
            self.id
        }
    }

    #[derive(Debug)]
    pub enum Opcode {
        kWasmTypeCast,
        kWasmTypeCheck,
        kAssertNotNull,
        kNull,
        kIsNull,
        kParameter,
        kStructGet,
        kStructSet,
        kArrayGet,
        kArrayLength,
        kGlobalGet,
        kWasmRefFunc,
        kWasmAllocateArray,
        kWasmAllocateStruct,
        kPhi,
        kWasmTypeAnnotation,
        kBranch,
        kRttCanon,
        // Add other opcodes as needed
    }

    pub fn opcode_name(opcode: &Opcode) -> &'static str {
        match opcode {
            Opcode::kWasmTypeCast => "kWasmTypeCast",
            Opcode::kWasmTypeCheck => "kWasmTypeCheck",
            Opcode::kAssertNotNull => "kAssertNotNull",
            Opcode::kNull => "kNull",
            Opcode::kIsNull => "kIsNull",
            Opcode::kParameter => "kParameter",
            Opcode::kStructGet => "kStructGet",
            Opcode::kStructSet => "kStructSet",
            Opcode::kArrayGet => "kArrayGet",
            Opcode::kArrayLength => "kArrayLength",
            Opcode::kGlobalGet => "kGlobalGet",
            Opcode::kWasmRefFunc => "kWasmRefFunc",
            Opcode::kWasmAllocateArray => "kWasmAllocateArray",
            Opcode::kWasmAllocateStruct => "kWasmAllocateStruct",
            Opcode::kPhi => "kPhi",
            Opcode::kWasmTypeAnnotation => "kWasmTypeAnnotation",
            Opcode::kBranch => "kBranch",
            Opcode::kRttCanon => "kRttCanon",
        }
    }

    pub trait OperationTrait {
        fn opcode(&self) -> Opcode;
    }

    #[derive(Debug)]
    pub struct Operation {
        pub opcode: Opcode,
        // Add common operation fields here.
    }

    impl OperationTrait for Operation {
        fn opcode(&self) -> Opcode {
            self.opcode.clone()
        }
    }

    #[derive(Debug)]
    pub struct WasmTypeCastOp {
        pub base: Operation,
        pub config: TypeCastConfig,
        object: OpIndex,
    }

    impl WasmTypeCastOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct WasmTypeCheckOp {
        pub base: Operation,
        pub config: TypeCheckConfig,
        object: OpIndex,
    }

    impl WasmTypeCheckOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct AssertNotNullOp {
        pub base: Operation,
        pub type_: wasm::ValueType,
        object: OpIndex,
    }

    impl AssertNotNullOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct IsNullOp {
        pub base: Operation,
        pub type_: wasm::ValueType,
        object: OpIndex,
    }

    impl IsNullOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct ParameterOp {
        pub base: Operation,
        pub parameter_index: usize,
    }

    #[derive(Debug)]
    pub struct StructGetOp {
        pub base: Operation,
        object: OpIndex,
        pub type_: Box<StructType>,
        pub field_index: usize,
    }

    impl StructGetOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct StructSetOp {
        pub base: Operation,
        object: OpIndex,
    }

    impl StructSetOp {
        pub fn object(&self) -> OpIndex {
            self.object
        }
    }

    #[derive(Debug)]
    pub struct ArrayGetOp {
        pub base: Operation,
        array: OpIndex,
        pub array_type: Box<ArrayType>,
    }

    impl ArrayGetOp {
        pub fn array(&self) -> OpIndex {
            self.array
        }
    }

    #[derive(Debug)]
    pub struct ArrayLengthOp {
        pub base: Operation,
        array: OpIndex,
    }

    impl ArrayLengthOp {
        pub fn array(&self) -> OpIndex {
            self.array
        }
    }

    #[derive(Debug)]
    pub struct GlobalGetOp {
        pub base: Operation,
        pub global: Box<Global>,
    }

    #[derive(Debug)]
    pub struct WasmRefFuncOp {
        pub base: Operation,
        pub function_index: usize,
    }

    #[derive(Debug)]
    pub struct WasmAllocateArrayOp {
        pub base: Operation,
        rtt: OpIndex,
    }

    impl WasmAllocateArrayOp {
        pub fn rtt(&self) -> OpIndex {
            self.rtt
        }
    }

    #[derive(Debug)]
    pub struct WasmAllocateStructOp {
        pub base: Operation,
        rtt: OpIndex,
    }

    impl WasmAllocateStructOp {
        pub fn rtt(&self) -> OpIndex {
            self.rtt
        }
    }

    #[derive(Debug)]
    pub struct PhiOp {
        pub base: Operation,
        inputs: Vec<OpIndex>,
        pub input_count: usize,
    }

    impl PhiOp {
        pub fn input(&self, index: usize) -> OpIndex {
            self.inputs[index]
        }
    }

    #[derive(Debug)]
    pub struct WasmTypeAnnotationOp {
        pub base: Operation,
        value: OpIndex,
        pub type_: wasm::ValueType,
    }

    impl WasmTypeAnnotationOp {
        pub fn value(&self) -> OpIndex {
            self.value
        }
    }

    #[derive(Debug)]
    pub struct BranchOp {
        pub base: Operation,
        condition: OpIndex,
        pub if_true: *const Block,
        pub if_false: *const Block,
    }

    impl BranchOp {
        pub fn condition(&self) -> OpIndex {
            self.condition
        }
    }

    #[derive(Debug)]
    pub struct NullOp {
        pub base: Operation,
        pub type_: wasm::ValueType,
    }

    #[derive(Debug)]
    pub struct GotoOp {
        pub base: Operation,
        pub destination: *const Block,
    }

    #[derive(Debug)]
    pub struct RttCanonOp {
        pub base: Operation,
        pub type_index: wasm::ModuleTypeIndex,
    }

    // Placeholder cast functions. These would ideally use a Rust
    // enum dispatch or similar to avoid unsafe casts.
    impl Operation {
        pub fn cast<T>(&self) -> &T {
            unsafe { &*(self as *const Operation as *const T) }
        }

        pub fn try_cast<T>(&self) -> Option<&T> {
            // This is a placeholder. In a real implementation, you'd
            // need to check the opcode and only cast if it's the correct type.
            // This example always returns Some for demonstration.
            Some(unsafe { &*(self as *const Operation as *const T) })
        }
    }

    // Example implementations for cast functions
    impl WasmTypeCastOp {
        pub fn cast_from_operation(op: &Operation) -> Option<&WasmTypeCastOp> {
            if let Opcode::kWasmTypeCast = op.opcode {
                Some(unsafe { &*(op as *const Operation as *const WasmTypeCastOp) })
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TypeCastConfig {
        pub to: wasm::ValueType,
    }

    #[derive(Debug, Clone)]
    pub struct TypeCheckConfig {
        pub to: wasm::ValueType,
    }

    #[derive(Debug)]
    pub struct Block {
        index: BlockIndex,
        kind: BlockKind,
        predecessors: Vec<*const Block>, // Vec of raw pointers, needs careful handling
        // other block fields
    }

    impl Block {
        pub fn index(&self) -> BlockIndex {
            self.index
        }

        pub fn kind(&self) -> BlockKind {
            self.kind.clone()
        }

        pub fn is_loop(&self) -> bool {
            self.kind == BlockKind::kLoop
        }

        pub fn is_branch_target(&self) -> bool {
            self.kind == BlockKind::kBranchTarget
        }

        pub fn is_merge(&self) -> bool {
            self.kind == BlockKind::kMerge
        }

        pub fn has_predecessors(&self) -> usize {
            self.predecessors.len()
        }

        pub fn PredecessorCount(&self) -> usize {
            self.predecessors.len()
        }

        pub fn Predecessors(&self) -> &Vec<*const Block> {
            &self.predecessors
        }

        pub fn PredecessorsIterable(&self) -> BlockPredecessorIterator {
            BlockPredecessorIterator {
                block: self,
                index: 0,
            }
        }

        // Note: requires graph to be passed to access operation.
        pub fn LastOperation<'a>(&self, graph: &'a Graph) -> &'a Operation {
            // Dummy implementation
            graph.operations.last().unwrap()
        }

        pub fn LastPredecessor(&self) -> BlockPredecessorIterator {
            let len = self.predecessors.len();
            BlockPredecessorIterator {
                block: self,
                index: len - 1,
            }
        }

        pub fn begin(&self) -> OpIndex {
            OpIndex{ id: 0} //dummy
        }
    }

    pub struct BlockPredecessorIterator<'a> {
        block: &'a Block,
        index: usize,
    }

    impl<'a> BlockPredecessorIterator<'a> {
        pub fn NeighboringPredecessor(&self) -> Self {
            BlockPredecessorIterator {
                block: self.block,
                index: self.index,
            }
        }

        pub fn index(&self) -> BlockIndex {
            BlockIndex{ id: 0 } //dummy
        }
    }

    impl<'a> Iterator for BlockPredecessorIterator<'a> {
        type Item = &'a Block;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.block.predecessors.len() {
                let ptr = self.block.predecessors[self.index];
                self.index += 1;
                unsafe { ptr.as_ref() }
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BlockKind {
        kNormal,
        kLoop,
        kBranchTarget,
        kMerge,
    }

    pub struct Graph {
        operations: Vec<Operation>,
        // Other graph fields
    }

    impl Graph {
        pub fn Get(&self, op_index: OpIndex) -> &Operation {
            &self.operations[op_index.id as usize]
        }

        pub fn Index(&self, operation: &Operation) -> OpIndex {
            // dummy
            OpIndex{ id: 0 }
        }

        pub fn OperationIndices(&self, block: &Block) -> OperationIndexIterator {
            OperationIndexIterator{
                block: block,
                graph: self,
                index: 0, // dummy
            }
        }
    }

    pub struct OperationIndexIterator<'a> {
        block: &'a Block,
        graph: &'a Graph,
        index: usize, // Dummy
    }

    impl<'a> Iterator for OperationIndexIterator<'a> {
        type Item = OpIndex;

        fn next(&mut self) -> Option<Self::Item> {
            // Dummy
            if self.index < 1 {
                self.index += 1;
                Some(OpIndex {id: 0})
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    pub struct Global {
        pub type_: wasm::ValueType,
    }

    #[derive(Debug)]
    pub struct StructType {
        // Define fields here
    }

    impl StructType {
        pub fn field(&self, _index: usize) -> wasm::ValueType {
            wasm::ValueType::I32 // Dummy
        }
    }

    #[derive(Debug)]
    pub struct ArrayType {
        // Define fields here
    }
}

mod turboshaft {
    // Placeholder for turboshaft related structures.
    pub struct LoopFinder {}

    impl LoopFinder {
        pub fn new(_zone: &Zone, _graph: &compiler::Graph) -> Self {
            LoopFinder {}
        }
    }

    pub struct AnalyzerIterator<'a> {
        zone: &'a Zone,
        graph: &'a compiler::Graph,
        loop_finder: LoopFinder,
        current_block_index: usize,
        blocks: Vec<&'a compiler::Block>,
        loop_revisit_skip_header: bool,
    }

    impl<'a> AnalyzerIterator<'a> {
        pub fn new(zone: &'a Zone, graph: &'a compiler::Graph, loop_finder: LoopFinder) -> Self {
            AnalyzerIterator {
                zone: zone,
                graph: graph,
                loop_finder: loop_finder,
                current_block_index: 0,
                blocks: Vec::new(), // Initialize with the blocks to process
                loop_revisit_skip_header: false,
            }
        }

        pub fn HasNext(&self) -> bool {
            self.current_block_index < self.blocks.len()
        }

        pub fn Next(&mut self) -> Option<&'a compiler::Block> {
            if self.HasNext() {
                let block = self.blocks[self.current_block_index];
                self.current_block_index += 1;
                Some(block)
            } else {
                None
            }
        }

        pub fn MarkLoopForRevisitSkipHeader(&mut self) {
            self.loop_revisit_skip_header = true;
        }

        pub fn MarkLoopForRevisit(&mut self) {
            // Dummy implementation
        }
    }
}

mod wasm {
    use super::compiler;

    // Placeholder for wasm value type definitions.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ValueType {
        kind: ValueTypeKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ValueTypeKind {
        I32,
        I64,
        F32,
        F64,
        Ref,
        NullRef,
        Bottom,
        Uninhabited,
        AnyRef,
        FuncRef,
        // Add other value types
    }

    impl ValueType {
        pub fn name(&self) -> String {
            match self.kind {
                ValueTypeKind::I32 => "i32".to_string(),
                ValueTypeKind::I64 => "i64".to_string(),
                ValueTypeKind::F32 => "f32".to_string(),
                ValueTypeKind::F64 => "f64".to_string(),
                ValueTypeKind::Ref => "ref".to_string(),
                ValueTypeKind::NullRef => "nullref".to_string(),
                ValueTypeKind::Bottom => "bottom".to_string(),
                ValueTypeKind::Uninhabited => "uninhabited".to_string(),
                ValueTypeKind::AnyRef => "anyref".to_string(),
                ValueTypeKind::FuncRef => "funcref".to_string(),
            }
        }

        pub fn is_non_nullable(&self) -> bool {
            // Dummy
            false
        }

        pub fn AsNonNull(&self) -> Self {
            // Dummy
            ValueType { kind: ValueTypeKind::I32 }
        }

        pub fn is_uninhabited(&self) -> bool {
            self.kind == ValueTypeKind::Uninhabited
        }

        pub fn Ref(heap_type: ModuleTypeIndex) -> Self {
            //Dummy
            ValueType {kind: ValueTypeKind::Ref}
        }
    }

    impl Default for ValueType {
        fn default() -> Self {
            ValueType { kind: ValueTypeKind::Bottom }
        }
    }

    pub const kWasmBottom: ValueType = ValueType { kind: ValueTypeKind::Bottom };

    pub fn IsSubtypeOf(t1: ValueType, t2: ValueType, _module: &Module) -> bool {
        // Dummy implementation
        t1 == t2
    }

    pub fn Union(t1: ValueType, t2: ValueType, _module: &Module, _module2: &Module) -> UnionResult {
        // Dummy implementation
        UnionResult{ type_: t1 }
    }

    pub fn Intersection(t1: ValueType, t2: ValueType, _module: &Module, _module2: &Module) -> IntersectionResult {
        // Dummy implementation
        IntersectionResult{ type_: t1 }
    }

    pub fn ToNullSentinel(params: NullSentinelParams) -> ValueType {
        //Dummy
        ValueType {kind: ValueTypeKind::NullRef}
    }

    #[derive(Debug)]
    pub struct NullSentinelParams<'a> {
        pub type_: ValueType,
        pub module_: &'a Module,
    }

    #[derive(Debug)]
    pub struct Module {
        // Dummy
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ModuleTypeIndex {
        id: u32
    }

    pub struct UnionResult {
        pub type_: ValueType
    }

    pub struct IntersectionResult {
        pub type_: ValueType
    }
}

mod v8_flags {
    // Placeholder for V8 flags.
    pub const trace_wasm_typer: bool = true;
}

macro_rules! TRACE {
    ($($arg:tt)*) => {
        if v8_flags::trace_wasm_typer {
            println!($($arg)*);
        }
    };
}

mod zone {
    // Placeholder for zone.
    pub struct Zone {}
}

use base::Vector;
use base::VectorOf;
use compiler::Block;
use compiler::BlockIndex;
use compiler::BlockKind;
use compiler::BranchOp;
use compiler::Global;
use compiler::Graph;
use compiler::IsNullOp;
use compiler::NullOp;
use compiler::OpIndex;
use compiler::Opcode;
use compiler::ParameterOp;
use compiler::PhiOp;
use compiler::RttCanonOp;
use compiler::StructGetOp;
use compiler::StructSetOp;
use compiler::ArrayGetOp;
use compiler::ArrayLengthOp;
use compiler::GlobalGetOp;
use compiler::WasmAllocateArrayOp;
use compiler::WasmAllocateStructOp;
use compiler::WasmRefFuncOp;
use compiler::WasmTypeAnnotationOp;
use compiler::WasmTypeCastOp;
use compiler::WasmTypeCheckOp;
use compiler::AssertNotNullOp;
use compiler::Operation;
use compiler::OperationTrait;
use std::cell::RefCell;
use std::rc::Rc;
use turboshaft::AnalyzerIterator;
use turboshaft::LoopFinder;
use wasm::ValueType;
use wasm::Module;
use zone::Zone;

#[derive(Clone, Copy)]
struct MaybeSnapshot(Snapshot);

impl MaybeSnapshot {
    fn new(snapshot: Snapshot) -> Self {
        MaybeSnapshot(snapshot)
    }

    fn has_value(&self) -> bool {
        true // Dummy
    }

    fn value(&self) -> Snapshot {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Copy)]
struct Snapshot {}

#[derive(Debug)]
struct TypeSnapshotTable {
    // Simplified implementation for demonstration purposes.
    types: HashMap<compiler::OpIndex, wasm::ValueType>,
    predecessor_values: HashMap<(compiler::OpIndex, usize), wasm::ValueType>,
}

impl TypeSnapshotTable {
    fn new() -> Self {
        TypeSnapshotTable {
            types: HashMap::new(),
            predecessor_values: HashMap::new(),
        }
    }

    fn Get(&self, op_index: compiler::OpIndex) -> wasm::ValueType {
        self.types.get(&op_index).copied().unwrap_or_default()
    }

    fn Set(&mut self, op_index: compiler::OpIndex, value: wasm::ValueType) {
        self.types.insert(op_index, value);
    }

    fn GetPredecessorValue(&self, op_index: compiler::OpIndex, input_index: usize) -> wasm::ValueType {
        self.predecessor_values.get(&(op_index, input_index)).copied().unwrap_or_default()
    }

    fn StartNewSnapshot(&mut self) {
        // Reset the snapshot data. In a real implementation, you might want
        // to copy the previous snapshot or use a more sophisticated approach.
        self.types.clear();
        self.predecessor_values.clear();
    }

    fn Seal(&self) -> Snapshot {
        // Creates a copy of the current snapshot.
        Snapshot {}
    }

    fn StartNewSnapshotWithPredecessors<F>(&mut self, predecessors: base::Vector<const Snapshot>, merge_function: F)
    where
        F: Fn(Key, base::Vector<const wasm::ValueType>) -> wasm::ValueType,
    {
        //Dummy
    }

    fn StartNewSnapshot(&mut self, snapshot: Snapshot) {
        //Dummy
    }
}

type Key = usize; // Dummy key

#[derive(Debug)]
struct WasmGCTypeAnalyzer<'a> {
    phase_zone_: &'a Zone,
    graph_: &'a Graph,
    types_table_: TypeSnapshotTable,
    input_type_map_: HashMap<compiler::OpIndex, wasm::ValueType>,
    block_to_snapshot_: HashMap<compiler::BlockIndex, MaybeSnapshot>,
    block_is_unreachable_: UnreachableBlocks,
    signature_: &'a compiler::Operation, // Signature
    module_: &'a Module,
    current_block_: Option<&'a compiler::Block>,
    is_first_loop_header_evaluation_: bool,
}

impl<'a> WasmGCTypeAnalyzer<'a> {
    fn new(
        phase_zone_: &'a Zone,
        graph_: &'a Graph,
        signature_: &'a compiler::Operation,
        module_: &'a Module,
    ) -> Self {
        WasmGCTypeAnalyzer {
            phase_zone_,
            graph_,
            types_table_: TypeSnapshotTable::new(),
            input_type_map_: HashMap::new(),
            block_to_snapshot_: HashMap::new(),
            block_is_unreachable_: UnreachableBlocks::new(),
            signature_: signature_,
            module_: module_,
            current_block_: None,
            is_first_loop_header_evaluation_: false,
        }
    }

    fn Run(&mut self) {
        let loop_finder = LoopFinder::new(self.phase_zone_, self.graph_);
        let mut iterator = AnalyzerIterator::new(self.phase_zone_, self.graph_, loop_finder);
        while iterator.HasNext() {
            let block = iterator.Next().unwrap();
            self.ProcessBlock(block);

            // Finish snapshot.
            self.block_to_snapshot_.insert(
                block.index(),
                MaybeSnapshot::new(self.types_table_.Seal()),
            );

            // Consider re-processing for loops.
            if let Some(last) = block.LastOperation(self.graph_).try_cast::<compiler::GotoOp>() {
                unsafe {
                if self.IsReachable(block) && (*last).destination.is_null() == false && (*last).destination.as_ref().unwrap().is_loop()
                    && (*(*last).destination).LastPredecessor().block == block {
                    TRACE!(
                        "[b{}] Reprocessing loop header b{} at backedge #{}\n",
                        block.index().id(),
                        (*(*last).destination).index().id(),
                        self.graph_.Index(block.LastOperation(self.graph_)).id()
                    );
                    let loop_header = unsafe { (*(*last).destination).as_ref().unwrap() };
                    // Create a merged snapshot state for the forward- and backedge and
                    // process all operations inside the loop header.
                    self.ProcessBlock(loop_header);
                    let old_snapshot = self.block_to_snapshot_[&loop_header.index()].value();
                    let snapshot = self.types_table_.Seal();
                    // TODO(14108): The snapshot isn't needed at all, we only care about the
                    // information if two snapshots are equivalent. Unfortunately, currently
                    // this can only be answered by creating a merge snapshot.
                    let needs_revisit = self.CreateMergeSnapshot(
                        base::Vector::of(&[old_snapshot, snapshot]),
                        base::Vector::of(&[true, true]),
                    );
                    self.types_table_.Seal(); // Discard the snapshot.

                    TRACE!(
                        "[b{}] Loop header b{} reprocessed at backedge #{}: {}\n",
                        block.index().id(),
                        (*(*last).destination).index().id(),
                        self.graph_.Index(block.LastOperation(self.graph_)).id(),
                        if needs_revisit {
                            "Scheduling loop body revisitation"
                        } else {
                            "No revisit of loop body needed"
                        }
                    );

                    // TODO(14108): This currently encodes a fixed point analysis where the
                    // analysis is finished once the backedge doesn't provide updated type
                    // information any more compared to the previous iteration. This could
                    // be stopped in cases where the backedge only refines types (i.e. only
                    // defines more precise types than the previous iteration).
                    if needs_revisit {
                        self.block_to_snapshot_
                            .insert(loop_header.index(), MaybeSnapshot::new(snapshot));
                        if block.index() != loop_header.index() {
                            // This will push the successors of the loop header to the iterator
                            // stack, so the loop body will be visited in the next iteration.
                            iterator.MarkLoopForRevisitSkipHeader();
                        } else {
                            // A single-block loop doesn't have any successors which would be
                            // re-evaluated and which might trigger another re-evaluation of the
                            // loop header.
                            // TODO(mliedtke): This is not a great design: We don't just
                            // schedule revisiting the loop header but afterwards we revisit it
                            // once again to evaluate whether we need to revisit it more times,
                            // so for single block loops the revisitation count will always be a
                            // multiple of 2. While this is inefficient, single-block loops are
                            // rare and are either endless loops or need to trigger an exception
                            // (e.g. a wasm trap) to terminate.
                            iterator.MarkLoopForRevisit();
                        }
                    }
                }
            }
            }
        }
    }

    fn ProcessBlock(&mut self, block: &'a compiler::Block) {
        debug_assert!(self.current_block_.is_none());
        self.current_block_ = Some(block);
        self.StartNewSnapshotFor(block);
        self.ProcessOperations(block);
        self.current_block_ = None;
    }

    fn StartNewSnapshotFor(&mut self, block: &'a compiler::Block) {
        self.is_first_loop_header_evaluation_ = false;
        // Reset reachability information. This can be outdated in case of loop
        // revisits. Below the reachability is calculated again and potentially
        // re-added.
        let block_was_previously_reachable = self.IsReachable(block);
        if !block_was_previously_reachable {
            TRACE!(
                "[b{}] Removing unreachable flag as block is re-evaluated\n",
                block.index().id()
            );
        }
        self.block_is_unreachable_.Remove(block.index());
        // Start new snapshot based on predecessor information.
        if block.has_predecessors() == 0 {
            // The first block just starts with an empty snapshot.
            debug_assert_eq!(block.index().id(), 0);
            self.types_table_.StartNewSnapshot();
        } else if block.is_loop() {
            let forward_predecessor = unsafe {
                (*block.LastPredecessor().NeighboringPredecessor().block.predecessors.last().unwrap()).as_ref().unwrap()
            };
            if !self.IsReachable(forward_predecessor) {
                // If a loop isn't reachable through its forward edge, it can't possibly
                // become reachable via the backedge.
                TRACE!(
                    "[b{}u] Loop unreachable as forward predecessor b{} is unreachable\n",
                    block.index().id(),
                    forward_predecessor.index().id()
                );
                self.block_is_unreachable_.Add(block.index());
            }
            let back_edge_snap =
                self.block_to_snapshot_.get(&block.LastPredecessor().index()).copied();
            if back_edge_snap.is_some() && block_was_previously_reachable {
                // The loop was already visited at least once. In this case use the
                // available information from the backedge.
                // Note that we only do this if the loop wasn't marked as unreachable
                // before. This solves an issue where a single block loop would think the
                // backedge is reachable as we just removed the unreachable information
                // above. Once the analyzer hits the backedge, it will re-evaluate if the
                // backedge changes any analysis results and then potentially revisit
                // this loop with forward edge and backedge.
                self.CreateMergeSnapshot(block);
            } else {
                // The loop wasn't visited yet. There isn't any type information available
                // for the backedge.
                TRACE!(
                    "[b{}{}] First loop header evaluation: Ignoring all backedges on phis\n",
                    block.index().id(),
                    if !self.IsReachable(*self.current_block_.unwrap()) {
                        "u"
                    } else {
                        ""
                    }
                );
                self.is_first_loop_header_evaluation_ = true;
                let forward_edge_snap = self.block_to_snapshot_[&forward_predecessor.index()].value();
                self.types_table_.StartNewSnapshot(forward_edge_snap);
            }
        } else if block.is_branch_target() {
            debug_assert_eq!(block.PredecessorCount(), 1);
            let predecessor = unsafe{ block.Predecessors()[0].as_ref().unwrap() };
            self.types_table_.StartNewSnapshot(
                self.block_to_snapshot_[&predecessor.index()].value(),
            );
            if self.IsReachable(predecessor) {
                let branch = block.Predecessors()[0].as_ref().unwrap().LastOperation(self.graph_).try_cast::<BranchOp>();
                if branch.is_some() {
                    self.ProcessBranchOnTarget(branch.unwrap(), block);
                }
            } else {
                TRACE!(
                    "[b{}u] Block unreachable as sole predecessor b{} is unreachable\n",
                    block.index().id(),
                    predecessor.index().id()
                );
                self.block_is_unreachable_.Add(block.index());
            }
        } else {
            debug_assert_eq!(block.kind(), BlockKind::kMerge);
            self.CreateMergeSnapshot(
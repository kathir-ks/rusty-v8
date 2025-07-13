// Converted from V8 C++ source files:
// Header: copying-phase.h
// Implementation: copying-phase.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub struct Vector<T> {
        data: Vec<T>,
    }
    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector { data: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }
        pub fn size(&self) -> usize {
            self.data.len()
        }
        pub fn get(&self, index: usize) -> &T {
            &self.data[index]
        }
        pub fn data(&self) -> &Vec<T> {
            &self.data
        }
        pub fn from_vec(vec: Vec<T>) -> Self {
            Vector { data: vec }
        }
    }
    pub struct SmallVector<T, const N: usize> {
        data: Vec<T>,
    }
    impl<T, const N: usize> SmallVector<T, const N> {
        pub fn new() -> Self {
            SmallVector { data: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }
        pub fn size(&self) -> usize {
            self.data.len()
        }
        pub fn get(&self, index: usize) -> &T {
            &self.data[index]
        }
    }
}
pub mod codegen {
    pub struct OptimizedCompilationInfo {}
    impl OptimizedCompilationInfo {
        pub fn tick_counter(&self) -> TickCounter {
            TickCounter {}
        }
    }
    pub struct SourcePosition {}
    impl SourcePosition {
        pub fn Unknown() -> Self {
            SourcePosition {}
        }
    }
}
pub mod compiler {
    pub mod turboshaft {
        use std::{
            cell::RefCell,
            fmt,
            hash::Hash,
            ops::BitAnd,
            rc::Rc,
        };
        use super::super::codegen::{OptimizedCompilationInfo, SourcePosition};
        use crate::base::Vector;
        pub struct PipelineData {}
        pub struct Zone {}
        pub struct Graph {
            companion: Graph,
            op_id_count: usize,
            block_count: usize,
        }
        impl Graph {
            pub fn new() -> Self {
                Graph {
                    companion: Graph::new(),
                    op_id_count: 0,
                    block_count: 0,
                }
            }
            pub fn GetOrCreateCompanion(&mut self) -> &mut Graph {
                &mut self.companion
            }
            pub fn Reset(&mut self) {
                // Dummy impl
            }
            pub fn blocks(&mut self) -> &mut Vec<Block> {
                todo!()
            }
            pub fn AllOperationIndices(&self) -> Vec<OpIndex> {
                // Dummy impl
                Vec::new()
            }
            pub fn operation_origins(&self) -> Vec<OpIndex> {
                // Dummy impl
                Vec::new()
            }
            pub fn source_positions(&self) -> Vec<SourcePosition> {
                // Dummy impl
                Vec::new()
            }
            pub fn modifiable_input_graph(&mut self) -> &mut Graph {
                self
            }
            pub fn op_id_count(&self) -> usize {
                self.op_id_count
            }
            pub fn block_count(&self) -> usize {
                self.block_count
            }
            pub fn Index(&self, _op: &dyn OperationTrait) -> OpIndex {
                OpIndex {}
            }
            pub fn Get(&self, _index: OpIndex) -> &dyn OperationTrait {
                todo!()
            }
            pub fn StartBlock(&self) -> Block {
                Block {}
            }
            pub fn GetRef(&self, _index: OpIndex) -> Rc<RefCell<dyn OperationTrait>> {
                todo!()
            }
            pub fn SwapWithCompanion(&mut self) {
                std::mem::swap(self, &mut self.companion);
            }
            pub fn AllOperationIndicesRef(&self) -> Vec<Rc<RefCell<dyn OperationTrait>>> {
                todo!()
            }
            pub fn BelongsToThisGraph(&self, _index: OpIndex) -> bool {
                true
            }
            pub fn zone(&self) -> *mut Zone {
                todo!()
            }
            pub fn operations<T>(&self, _first_output_index: OpIndex, _next_operation_index: OpIndex) -> Vec<&T> {
                todo!()
            }
            pub fn IsCreatedFromTurbofan(&self) -> bool {
                todo!()
            }
            pub fn IndexOp<T>(&self, op: &T) -> OpIndex
            where
                T: OperationTrait,
            {
                OpIndex {}
            }
            pub fn new_block(&mut self, kind: Block::Kind) -> Block {
                self.block_count += 1;
                Block {
                    index: BlockIndex {},
                    kind,
                    origin_: None
                }
            }
            pub fn is_loop(&self) -> bool {
                todo!()
            }
        }
        pub struct Block {
            index: BlockIndex,
            kind: Block::Kind,
            origin_: Option<Box<Block>>
        }
        impl Block {
            pub fn new(index: BlockIndex) -> Self {
                Block {
                    index,
                    kind: Block::Kind::kMerge,
                    origin_: None,
                }
            }

            pub fn index(&self) -> BlockIndex {
                self.index
            }
            pub fn SetOrigin(&mut self, _current_input_block: &Block) {}
            pub fn IsLoop(&self) -> bool {
                false
            }
            pub fn GetPredecessorIndex(&self, _origin_for_block_end: &Block) -> i32 {
                0
            }
            pub fn LastOperation(&self, _input_graph: &Graph) -> &dyn OperationTrait {
                todo!()
            }
            pub fn LastChild(&self) -> *mut Block {
                std::ptr::null_mut()
            }
            pub fn NeighboringChild(&self) -> *mut Block {
                std::ptr::null_mut()
            }
            pub fn PredecessorCount(&self) -> i32 {
                0
            }
            pub fn LastPredecessor(&self) -> *mut Block {
                std::ptr::null_mut()
            }
            pub fn OriginForBlockEnd(&self) -> *mut Block {
                std::ptr::null_mut()
            }
            pub fn NeighboringPredecessor(&self) -> *mut Block {
                std::ptr::null_mut()
            }
            pub fn clear_custom_data(&mut self) {}
            pub fn set_custom_data(&mut self, _pos: u32, _kind: Block::CustomDataKind) {}
            pub fn get_custom_data(&self, _kind: Block::CustomDataKind) -> i32 {
                0
            }
            pub fn begin(&self) -> OpIndex {
                OpIndex {}
            }
            pub fn end(&self) -> OpIndex {
                OpIndex {}
            }
            pub fn Contains(&self, _phi_index: OpIndex) -> bool {
                true
            }
            pub fn SetHasPeeledIteration(&mut self) {}
            pub fn has_peeled_iteration(&self) -> bool {
                false
            }
            pub fn kind(&self) -> &Block::Kind {
                &self.kind
            }
            pub fn set_kind(&mut self, kind: Block::Kind) {
                self.kind = kind;
            }
            pub fn origin_(&self) -> &Option<Box<Block>> {
                &self.origin_
            }
            pub fn origin(&self) -> *const Block {
                todo!()
            }
            pub fn set_origin(&mut self, origin: Block) {
                self.origin_ = Some(Box::new(origin));
            }
        
        }

        impl Block {
            pub enum Kind {
                kMerge,
                kLoopHeader,
            }
            pub enum CustomDataKind {
                kPhiInputIndex,
            }
            pub fn IsMerge(&self) -> bool {
                match self.kind {
                    Block::Kind::kMerge => true,
                    _ => false,
                }
            }
            pub fn IsLoopHeader(&self) -> bool {
                match self.kind {
                    Block::Kind::kLoopHeader => true,
                    _ => false,
                }
            }
        }

        pub struct BlockIndex {}
        impl BlockIndex {
            pub fn id(&self) -> u32 {
                0
            }
        }

        pub struct FixedOpIndexSidetable<T> {
            data: Vec<T>,
            graph: *const Graph,
        }
        impl<T: Copy> FixedOpIndexSidetable<T> {
            pub fn new(size: usize, default_value: T, zone: *mut Zone, graph: *const Graph) -> Self {
                FixedOpIndexSidetable {
                    data: vec![default_value; size],
                    graph,
                }
            }
        }
        impl<T: Copy> std::ops::Index<OpIndex> for FixedOpIndexSidetable<T> {
            type Output = T;
            fn index(&self, index: OpIndex) -> &Self::Output {
                &self.data[0]
            }
        }
        impl<T: Copy> std::ops::IndexMut<OpIndex> for FixedOpIndexSidetable<T> {
            fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
                &mut self.data[0]
            }
        }

        pub struct FixedBlockSidetable<T> {
            data: Vec<T>,
        }
        impl<T> FixedBlockSidetable<T> {
            pub fn new(size: usize, default_value: T, zone: *mut Zone) -> Self {
                FixedBlockSidetable {
                    data: vec![default_value; size],
                }
            }
        }
        impl<T> std::ops::Index<BlockIndex> for FixedBlockSidetable<T> {
            type Output = T;
            fn index(&self, index: BlockIndex) -> &Self::Output {
                &self.data[0]
            }
        }
        impl<T> std::ops::IndexMut<BlockIndex> for FixedBlockSidetable<T> {
            fn index_mut(&mut self, index: BlockIndex) -> &mut Self::Output {
                &mut self.data[0]
            }
        }

        pub struct BitVector {
            data: Vec<bool>,
        }
        impl BitVector {
            pub fn new(size: usize, zone: *mut Zone) -> Self {
                BitVector {
                    data: vec![false; size],
                }
            }
            pub fn Add(&mut self, index: u32) {
                self.data[index as usize] = true;
            }
            pub fn Contains(&self, index: u32) -> bool {
                self.data[index as usize]
            }
        }

        pub struct OpIndex {}
        impl OpIndex {
            pub fn valid(&self) -> bool {
                true
            }
            pub fn id(&self) -> u32 {
                0
            }
        
        }
        impl From<V<None>> for OpIndex {
            fn from(_: V<None>) -> Self {
                OpIndex {}
            }
        }
        pub struct OptionalOpIndex {}
        impl OptionalOpIndex {
            pub fn Nullopt() -> Self {
                OptionalOpIndex {}
            }
            pub fn has_value(&self) -> bool {
                false
            }
            pub fn value(&self) -> OpIndex {
                OpIndex {}
            }
        }
        pub trait OperationTrait {
            fn opcode(&self) -> Opcode;
            fn inputs(&self) -> Vector<OpIndex> {
                Vector { data: Vec::new() }
            }
            fn outputs_rep(&self) -> Vector<RegisterRepresentation> {
                Vector { data: Vec::new() }
            }
            fn is<T: OperationTrait>(&self) -> bool {
                std::any::Any::type_id(self) == std::any::Any::type_id(&T::default())
            }
            fn as_any(&self) -> &dyn std::any::Any;
            fn input(&self, _index: i32) -> OpIndex {
                OpIndex {}
            }
        
        }
        impl dyn OperationTrait {
            fn TryCast<T: OperationTrait>(&self) -> Option<&T> {
                if self.is::<T>() {
                    Some(self.as_any().downcast_ref::<T>().unwrap())
                } else {
                    None
                }
            }
        }
        pub struct Operation {
            pub opcode: Opcode,
        }
        impl Operation {
            pub fn Is<T: OperationTrait>(&self) -> bool {
                match self.opcode {
                    Opcode::kGoto => {
                        let goto_op = GotoOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&goto_op)
                    }
                    Opcode::kBranch => {
                        let branch_op = BranchOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&branch_op)
                    }
                    Opcode::kSwitch => {
                        let switch_op = SwitchOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&switch_op)
                    }
                    Opcode::kPhi => {
                        let phi_op = PhiOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&phi_op)
                    }
                    Opcode::kPendingLoopPhi => {
                        let pending_loop_phi_op = PendingLoopPhiOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&pending_loop_phi_op)
                    }
                    Opcode::kFrameState => {
                        let frame_state_op = FrameStateOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&frame_state_op)
                    }
                    Opcode::kCall => {
                        let call_op = CallOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&call_op)
                    }
                    Opcode::kDidntThrow => {
                        let didnt_throw_op = DidntThrowOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&didnt_throw_op)
                    }
                    Opcode::kCheckException => {
                        let check_exception_op = CheckExceptionOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&check_exception_op)
                    }
                    Opcode::kParameter => {
                        let parameter_op = ParameterOp {};
                        std::any::Any::type_id(self) == std::any::Any::type_id(&parameter_op)
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            pub fn is_block_terminator(&self) -> bool {
                true
            }
            pub fn Cast<T: OperationTrait>(&self) -> &T {
                todo!()
            }
        }
        pub trait OpInterface {
            fn Explode<F, R>(&self, _f: F, _arg: R) -> OpIndex
            where
                F: FnOnce() -> OpIndex,
            {
                todo!()
            }
        }
        #[derive(Clone)]
        pub enum Opcode {
            kGoto,
            kBranch,
            kSwitch,
            kPhi,
            kPendingLoopPhi,
            kFrameState,
            kCall,
            kDidntThrow,
            kCheckException,
            kParameter,
            kTuple,
            kNumberConstant,
        }
        pub struct GotoOp {
            pub destination: *mut Block,
            pub is_backedge: bool,
        }
        impl GotoOp {
            pub fn new(destination: *mut Block, is_backedge: bool) -> Self {
                GotoOp {
                    destination,
                    is_backedge,
                }
            }
        }
        impl OperationTrait for GotoOp {
            fn opcode(&self) -> Opcode {
                Opcode::kGoto
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for GotoOp {}
        pub struct BranchOp {
            pub condition: OpIndex,
            pub if_true: *mut Block,
            pub if_false: *mut Block,
            pub hint: bool,
        }
        impl OperationTrait for BranchOp {
            fn opcode(&self) -> Opcode {
                Opcode::kBranch
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for BranchOp {}
        pub struct SwitchOp {
            pub input: OpIndex,
            pub cases: Vec<SwitchOp::Case>,
            pub default_case: *mut Block,
            pub default_hint: bool,
        }
        impl SwitchOp {
            pub struct Case {
                pub value: i32,
                pub destination: *mut Block,
                pub hint: bool,
            }
        }
        impl OperationTrait for SwitchOp {
            fn opcode(&self) -> Opcode {
                Opcode::kSwitch
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for SwitchOp {}
        pub struct PhiOp {
            pub input_count: usize,
            pub rep: RegisterRepresentation,
        }
        impl PhiOp {
            pub const kLoopPhiBackEdgeIndex: usize = 1;
        }
        impl OperationTrait for PhiOp {
            fn opcode(&self) -> Opcode {
                Opcode::kPhi
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn input(&self, index: i32) -> OpIndex {
                OpIndex {}
            }
            fn inputs(&self) -> Vector<OpIndex> {
                Vector {data: Vec::new()}
            }
        }
        impl OpInterface for PhiOp {}
        pub struct PendingLoopPhiOp {}
        impl OperationTrait for PendingLoopPhiOp {
            fn opcode(&self) -> Opcode {
                Opcode::kPendingLoopPhi
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for PendingLoopPhiOp {}
        pub struct FrameStateOp {
            pub inlined: bool,
            pub data: i32,
        }
        impl OperationTrait for FrameStateOp {
            fn opcode(&self) -> Opcode {
                Opcode::kFrameState
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn inputs(&self) -> Vector<OpIndex> {
                Vector{ data: Vec::new() }
            }
        }
        impl OpInterface for FrameStateOp {}
        pub struct CallOp {
            pub descriptor: i32,
        }
        impl CallOp {
            pub fn callee(&self) -> V<CallTarget> {
                V { index: OpIndex {} }
            }
            pub fn frame_state(&self) -> OptionalV<FrameState> {
                OptionalV { index: OptionalOpIndex::Nullopt() }
            }
            pub fn arguments(&self) -> Vector<OpIndex> {
                Vector { data: Vec::new() }
            }
            pub fn Effects(&self) -> i32 {
                0
            }
        }
        impl OperationTrait for CallOp {
            fn opcode(&self) -> Opcode {
                Opcode::kCall
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for CallOp {}
        pub struct DidntThrowOp {
            pub throwing_operation: OpIndex,
            pub didnt_throw_block: *mut Block,
        }
        impl OperationTrait for DidntThrowOp {
            fn opcode(&self) -> Opcode {
                Opcode::kDidntThrow
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for DidntThrowOp {}
        pub struct CheckExceptionOp {
            pub catch_block: *mut Block,
            pub didnt_throw_block: *mut Block,
        }
        impl OperationTrait for CheckExceptionOp {
            fn opcode(&self) -> Opcode {
                Opcode::kCheckException
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for CheckExceptionOp {}
        pub struct ParameterOp {
            pub parameter_index: i32,
            pub rep: RegisterRepresentation,
            pub debug_name: i32,
        }
        impl OperationTrait for ParameterOp {
            fn opcode(&self) -> Opcode {
                Opcode::kParameter
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl OpInterface for ParameterOp {}
        impl dyn OpInterface {
            fn Explode<F, R>(&self, _f: F, _arg: R) -> OpIndex
            where
                F: FnOnce() -> OpIndex,
            {
                todo!()
            }
        }
        pub struct Assembler<ReducerList> {
            reducer_list: ReducerList,
            current_block: *mut Block,
            current_origin: OpIndex,
            output_graph: Graph,
            input_graph: Graph,
            phase_zone: *mut Zone,
            data: *mut PipelineData,
            can_auto_inline_blocks_with_single_predecessor: bool,
            variable_reducer: VariableReducer<i32>,
        }
        impl<ReducerList> Assembler<ReducerList> {
            pub fn current_block(&self) -> *mut Block {
                self.current_block
            }
            pub fn ReduceGoto(&mut self, _destination: *mut Block, _is_backedge: bool) {}
            pub fn ReduceBranch(&mut self, _condition: OpIndex, _if_true: *mut Block, _if_false: *mut Block, _hint: bool) -> OpIndex {
                OpIndex {}
            }
            pub fn ReduceSwitch(&mut self, _input: OpIndex, _cases: *mut Vec<SwitchOp::Case>, _default_case: *mut Block, _default_hint: bool) -> OpIndex {
                OpIndex {}
            }
            pub fn ReducePhi(&mut self, _inputs: Vector<OpIndex>, _rep: RegisterRepresentation) -> OpIndex {
                OpIndex {}
            }
            pub fn Parameter(&mut self, _parameter_index: i32, _rep: RegisterRepresentation, _debug_name: i32) -> OpIndex {
                OpIndex {}
            }
            pub fn CanAutoInlineBlocksWithSinglePredecessor(&self) -> bool {
                self.can_auto_inline_blocks_with_single_predecessor
            }
            pub fn modifiable_input_graph(&mut self) -> &mut Graph {
                &mut self.input_graph
            }
            pub fn SetCurrentOrigin(&mut self, index: OpIndex) {
                self.current_origin = index;
            }
            pub fn generating_unreachable_operations(&self) -> bool {
                false
            }
            pub fn input_graph(&self) -> &Graph {
                &self.input_graph
            }
            pub fn output_graph(&mut self) -> &mut Graph {
                &mut self.output_graph
            }
            pub fn phase_zone(&self) -> *mut Zone {
                self.phase_zone
            }
            pub fn data(&self) -> *mut PipelineData {
                self.data
            }
            pub fn FixLoopPhi(&self, _input_phi: PhiOp, _phi_index: OpIndex, _output_graph_loop: *mut Block) {}
            pub fn GetVariable(&self, _var: Variable) -> OpIndex {
                OpIndex {}
            }
            pub fn GetPredecessorValue(&self, _var: Variable, _predecessor_index: i32) -> OpIndex {
                OpIndex {}
            }
            pub fn NewLoopInvariantVariable(&self, _rep: MaybeRegisterRepresentation) -> Variable {
                Variable {}
            }
            pub fn SetVariable(&self, _var: Variable, _new_index: OpIndex) {}
            pub fn Bind(&mut self, block: *mut Block) -> bool {
                self.current_block = block;
                true
            }
            pub fn BindReachable(&mut self, _output_block: *mut Block) {}
            pub fn FinalizeLoop(&mut self, _destination: *mut Block) {}
            pub fn PendingLoopPhi(&mut self, _og_index: OpIndex, _rep: RegisterRepresentation) -> OpIndex {
                OpIndex {}
            }
            pub fn Verify(&self, _index: OpIndex, _new_index: OpIndex) {}
            pub fn ReduceFrameState(&mut self, _vector_of: Vector<OpIndex>, _inlined: bool, _data: i32) -> OpIndex {
                OpIndex {}
            }
            pub fn ReduceCall(&mut self, _callee: V<CallTarget>, _frame_state: OptionalV<FrameState>, _vector_of: Vector<OpIndex>, _descriptor: i32, _effects: i32) -> OpIndex {
                OpIndex {}
            }
            pub fn InlineOp(&mut self, _index: OpIndex, _didnt_throw_block: *mut Block) -> bool {
                true
            }
        }
        pub struct ReducerList {}
        pub struct Variable {}
        pub struct V<T> {
            index: OpIndex,
        }
        impl<T> V<T> {
            pub fn Cast(_map_to_new_graph: OpIndex) -> Self {
                V { index: OpIndex {} }
            }
            pub fn Invalid() -> Self {
                V { index: OpIndex {} }
            }
        }
        pub struct OptionalV<T> {
            index: OptionalOpIndex,
        }
        pub struct CallTarget {}
        pub struct FrameState {}
        pub struct RegisterRepresentation {}
        impl RegisterRepresentation {
            pub fn AllowImplicitRepresentationChangeTo(&self, _other: RegisterRepresentation, _b: bool) -> bool {
                true
            }
        }
        pub struct MaybeRegisterRepresentation {}
        impl MaybeRegisterRepresentation {
            pub fn None() -> Self {
                MaybeRegisterRepresentation {}
            }
        }
        pub struct TupleOp {}
        impl OperationTrait for TupleOp {
            fn opcode(&self) -> Opcode {
                Opcode::kTuple
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        pub struct NumberConstantOp {}
        impl OperationTrait for NumberConstantOp {
            fn opcode(&self) -> Opcode {
                Opcode::kNumberConstant
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        pub struct TickCounter {}
        impl TickCounter {
            pub fn TickAndMaybeEnterSafepoint(&self) {}
        }
        pub struct NodeOriginTable {}
        impl NodeOriginTable {
            pub fn SetNodeOrigin(&self, _id: u32, _id2: u32) {}
        }
        pub struct SnapshotTable {}
        pub struct VariableReducer<AfterNext> {
            next: AfterNext,
        }
        impl<AfterNext> VariableReducer<AfterNext> {
            pub fn Bind(&mut self, _block: *mut Block) {}
        }
        pub struct ReducerBaseForwarder<Next> {
            next: Next,
        }
        pub struct WasmRevecReducer<Next> {
            next: Next,
        }
        pub struct TSAssembler<GraphVisitor, Reducers> {
            graph_visitor: GraphVisitor,
            reducers: Reducers,
            assembler: Assembler<ReducerList>,
        }
        impl<GraphVisitor, Reducers> TSAssembler<GraphVisitor, Reducers> {
            pub fn new(data: *mut PipelineData, input_graph: Graph, output_graph: Graph, phase_zone: *mut Zone) -> Self {
                TSAssembler {
                    graph_visitor: GraphVisitor::new(),
                    reducers: Reducers::new(),
                    assembler: Assembler {
                        reducer_list: ReducerList {},
                        current_block: std::ptr::null_mut(),
                        current_origin: OpIndex {},
                        output_graph,
                        input_graph,
                        phase_zone,
                        data,
                        can_auto_inline_blocks_with_single_predecessor: false,
                        variable_reducer: VariableReducer { next: 0 },
                    },
                }
            }
            pub fn template_VisitGraph<const TRACE_REDUCTION: bool>(&mut self) {
                self.graph_visitor.template_VisitGraph::<TRACE_REDUCTION>(&mut self.assembler);
            }
        }
        pub struct CopyingPhaseImpl {}
        impl CopyingPhaseImpl {
            pub fn Run(_data: *mut PipelineData, _input_graph: Graph, _phase_zone: *mut Zone, _trace_reductions: bool) {}
        }
        pub struct CopyingPhase {}
        impl CopyingPhase {
            pub fn Run(_data: *mut PipelineData, _phase_zone: *mut Zone) {}
        }
        pub struct GraphVisitor {
            input_graph_: Graph,
            current_input_block_: *const Block,
            op_mapping_: FixedOpIndexSidetable<OpIndex>,
            block_mapping_: FixedBlockSidetable<*mut Block>,
            blocks_needing_variables_: BitVector,
            old_opindex_to_variables: FixedOpIndexSidetable<MaybeVariable>,
            blocks_to_clone_: Vec<BlockToClone>,
            turn_loop_without_backedge_into_merge_: bool,
            current_block_needs_variables_: bool,
        }
        impl GraphVisitor {
            pub fn new() -> Self {
                let mut graph = Graph::new();
                GraphVisitor {
                    input_graph_: graph,
                    current_input_block_: std::ptr::null(),
                    op_mapping_: FixedOpIndexSidetable {
                        data: Vec::new(),
                        graph: std::ptr::null(),
                    },
                    block_mapping_: FixedBlockSidetable { data: Vec::new() },
                    blocks_needing_variables_: BitVector { data: Vec::new() },
                    old_opindex_to_variables: FixedOpIndexSidetable {
                        data: Vec::new(),
                        graph: std::ptr::null(),
                    },
                    blocks_to_clone_: Vec::new(),
                    turn_loop_without_backedge_into_merge_: true,
                    current_block_needs_variables_: false,
                }
            }
            pub fn current_input_block(&self) -> *const Block {
                self.current_input_block_
            }
            pub fn turn_loop_without_backedge_into_merge(&mut self) -> *mut bool {
                &mut self.turn_loop_without_backedge_into_merge_ as *mut bool
            }
            pub fn Finalize(&mut self) {}
            fn MapToNewGraph(&self, _block: *const Block) -> *mut Block {
                todo!()
            }
            pub fn InlineOp(&mut self, _index: OpIndex, _input_block: *const Block) -> bool {
                true
            }
            fn OriginForBlockStart(&self, _block: *mut Block) -> *const Block {
                todo!()
            }
            pub fn GetVariableFor(&self, _old_index: OpIndex) -> MaybeVariable {
                MaybeVariable {}
            }
            pub fn SetVariableFor(&self, _old_index: OpIndex, _var: MaybeVariable) {}
            pub fn CloneBlockAndGoto(&mut self, _input_block: *const Block) {}
            pub fn CloneAndInlineBlock(&mut self, _input_block: *const Block) {}
            pub fn template_VisitGraph<const TRACE_REDUCTION: bool>(&mut self, asm: &mut Assembler<ReducerList>) {}
        }
        pub struct BlockToClone {
            input_block: *const Block,
            added_block_phi_input: i32,
            new_output_block: *mut Block,
        }
        pub struct PaddingSpace {
            spaces: i32,
        }
        impl fmt::Display for PaddingSpace {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.spaces > 10000 {
                    return Ok(());
                }
                for _ in 0..self.spaces {
                    write!(f, " ")?;
                }
                Ok(())
            }
        }
        pub struct OperationPrintStyle<T, const S: &'static str> {
            operation: T,
        }
        pub struct PrintAsBlockHeader<T> {
            block: T,
        }
        pub struct None {}
        
        impl OperationTrait for None {
            fn opcode(&self) -> Opcode {
                Opcode::kTuple
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        impl Default for dyn OperationTrait {
            fn default() -> Self {
                todo!()
            }
        }
        impl Default for Goto

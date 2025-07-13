// Converted from V8 C++ source files:
// Header: variable-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::base;
use crate::codegen::machine_type::MachineRepresentation;
use crate::compiler::turboshaft::operations::FrameStateOp;
use crate::compiler::turboshaft::operations::LoadRootRegisterOp;
use crate::compiler::turboshaft::operations::PhiOp;
use crate::compiler::turboshaft::operations::RegisterRepresentation;
use crate::compiler::turboshaft::operations::V;
use crate::compiler::turboshaft::operations::MaybeRegisterRepresentation;
use crate::compiler::turboshaft::operations::AbortReason;
use crate::compiler::turboshaft::operations::PendingLoopPhiOp;
use crate::compiler::turboshaft::optimize_phase::VariableReducer;
use crate::execution::isolate::Isolate;
use crate::objects::code::Code;
use crate::objects::map::MapRef;
use crate::objects::tagged::Tagged;
use crate::objects::object::Object;
use crate::wasm::memory_tracing::Zone;

//use crate::compiler::turboshaft::csa_optimize_phase::V8;
//use crate::compiler::turboshaft::graph_visualizer::Block;
//use crate::v8_container::Set;
//use crate::compiler::wasm_gc_operator_reducer::If;
//use crate::compiler::turboshaft::graph_visualizer::OpIndex;
//use crate::compiler::turboshaft::graph_visualizer::Operation;
//use crate::compiler::turboshaft::optimize_phase::VariableReducer;
//use crate::compiler::turboshaft::operations::AbortReason;
//use crate::v8::Snapshot;
//use crate::compiler::wasm_gc_operator_reducer::AdvancedReducerWithControlPathState;
//use crate::compiler::turboshaft::access_builder::TypeCache;
//use crate::compiler::simplified_lowering_verifier::void;
//use crate::compiler::turboshaft::operations::MaybeRegisterRepresentation;
//use crate::compiler::turboshaft::operations::RegisterRepresentation;
//use crate::compiler::turboshaft::operations::Frame;
//use crate::compiler::turboshaft::operations::Map;
//use crate::compiler::turboshaft::loop_unrolling_reducer::source;
//use crate::compiler::turboshaft::phase::code;
use crate::compiler::turboshaft::fast_hash::FastHashSeed;
//use crate::compiler::wasm_simd_phase::graph;
use crate::compiler::node::Node;
use crate::compiler::basic_block::BasicBlock;
use crate::libsampler::sampler::SignalHandler;
//use crate::compiler::redundancy_elimination::EffectPathChecks;
use crate::compiler::machine_operator::AtomicMemoryOrder;
//use crate::compiler::js_intrinsic_lowering::Reduction;
//use crate::v8_template::then;
use crate::execution::simulator::Simulator;
//use crate::compiler::turboshaft::type_parser::TypeParserError;
use crate::execution::local_isolate_inl::RootIndex;
//use crate::tasks::cancelable_task::Cancelable;
use crate::compiler::code_assembler::Builtin;
use crate::compiler::representation_change::RepresentationChange;
use crate::inspector::v8_inspector::state;
use crate::compiler::branch_elimination::Reduction;
use crate::compiler::loop_analysis::Loop;
use crate::compiler::js_inlining::FrameState;
use crate::compiler::turboshaft::typer::Type;
use crate::asmjs::asm_scanner::AsmScanner;
//use crate::compiler::js_create_lowering::Map;
use crate::compiler::turboshaft::operations::Operation;
use crate::compiler::turboshaft::graph::Graph;

// Turboshaft specific
pub struct Assembler {
    // incomplete...
}

impl Assembler {
    pub fn load_root_register(&self) -> OpIndex {
        OpIndex { value: 0 }
    }
    pub fn phi(&self, inputs: base::Vector<OpIndex>, rep: RegisterRepresentation) -> OpIndex {
        OpIndex { value: 0 }
    }
    pub fn frame_state(&self, inputs: base::Vector<OpIndex>, inlined: bool, data: *const std::ffi::c_void) -> OpIndex {
        OpIndex { value: 0 }
    }
    pub fn pending_loop_phi(&self, var: OpIndex, rep: RegisterRepresentation) -> V<Any> {
        V {
            op_index: OpIndex { value: 0 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockIndex {
    value: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpIndex {
    value: usize,
}

impl OpIndex {
    pub fn valid(&self) -> bool {
        self.value != usize::MAX
    }

    pub fn invalid() -> Self {
        OpIndex { value: usize::MAX }
    }
}

pub struct ZoneVector<T> {
    elements: Vec<T>,
    //zone: Rc<RefCell<Zone>>, // incomplete
}

impl<T> ZoneVector<T> {
    pub fn new(zone: &Zone) -> Self {
        ZoneVector { elements: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }
}

pub struct ZoneAbslFlatHashMap<K, V> {
    map: HashMap<K, V>,
    //zone: Rc<RefCell<Zone>>, // incomplete
}

impl<K: Eq + std::hash::Hash, V> ZoneAbslFlatHashMap<K, V> {
    pub fn new(zone: &Zone) -> Self {
        ZoneAbslFlatHashMap { map: HashMap::new() }
    }
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

pub struct GrowingBlockSidetable<T> {
    table: Vec<T>,
    default_value: T,
    //zone: Rc<RefCell<Zone>>, // incomplete
}

impl<T: Clone> GrowingBlockSidetable<T> {
    pub fn new(block_count: usize, default_value: T, zone: &Zone) -> Self {
        GrowingBlockSidetable {
            table: vec![default_value.clone(); block_count],
            default_value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    id: usize,
    data: *mut VariableData,
}

impl Variable {
    fn data(&self) -> &VariableData {
        unsafe { &*self.data }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IntrusiveSetIndex {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariableData {
    rep: MaybeRegisterRepresentation,
    loop_invariant: bool,
    pub active_loop_variables_index: IntrusiveSetIndex,
}

struct SnapshotTable<K, V> {
    // incomplete
    _phantom: std::marker::PhantomData<(K, V)>,
}
impl<K, V> SnapshotTable<K, V> {
    fn new() -> Self {
        SnapshotTable { _phantom: std::marker::PhantomData }
    }
}

struct ChangeTrackingSnapshotTable<Table, K, V> {
    // incomplete
    _phantom: std::marker::PhantomData<(Table, K, V)>,
}
impl<Table, K, V> ChangeTrackingSnapshotTable<Table, K, V> {
    fn new() -> Self {
        ChangeTrackingSnapshotTable { _phantom: std::marker::PhantomData }
    }
}

pub struct Block {
    index: BlockIndex,
    predecessors: Vec<Block>,
    is_loop: bool,
    is_bound: bool,
    predecessor_count: usize,
}

impl Block {
    pub fn index(&self) -> BlockIndex {
        self.index
    }
    pub fn PredecessorsIterable(&self) -> std::slice::Iter<Block> {
        self.predecessors.iter()
    }
    pub fn IsLoop(&self) -> bool {
        self.is_loop
    }
    pub fn IsBound(&self) -> bool {
        self.is_bound
    }
    pub fn PredecessorCount(&self) -> usize {
        self.predecessor_count
    }
}

pub struct JSHeapBroker {}

impl JSHeapBroker {
    pub fn allocation_zone(&self) -> &Zone {
        todo!()
    }
}

pub trait TurboshaftReducer {
    fn reduce_goto(&mut self, destination: *mut Block, is_backedge: bool) -> V<void>;
}

impl<AfterNext> VariableReducer<AfterNext> {
    pub fn new(after_next: AfterNext, _isolate: *mut Isolate, _broker: &JSHeapBroker) -> Self {
        VariableReducer {
            next: RequiredOptimizationReducer { next: after_next },
            table_: VariableTable::new(__phase_zone()),
            current_block_: None,
            block_to_snapshot_mapping_: GrowingBlockSidetable::new(0, None, __phase_zone()),
            is_temporary_: false,
            predecessors_: ZoneVector::new(__phase_zone()),
            loop_pending_phis_: ZoneAbslFlatHashMap::new(__phase_zone()),
        }
    }
}

impl<AfterNext: TurboshaftReducer> VariableReducer<AfterNext> {
    fn reduce_goto(&mut self, destination: *mut Block, is_backedge: bool) -> V<void> {
        let result = self.next.reduce_goto(destination, is_backedge);
        let destination = unsafe { &*destination };

        if !destination.IsBound() {
            return result;
        }

        if destination.IsLoop() && destination.PredecessorCount() == 2 {
            if self.loop_pending_phis_.contains(&destination.index()) {
                if let Some(pending_phis) = self.loop_pending_phis_.get(&destination.index()) {
                    for (var, pending_phi_idx) in &pending_phis.elements {
                        let pending_phi_idx = *pending_phi_idx;
                        //let pending_phi = self.__get(pending_phi_idx).downcast_ref::<PendingLoopPhiOp>().unwrap();
                        let pending_phi = unsafe {
                            let op = self.__output_graph().get(pending_phi_idx);
                            &*(op as *const dyn Any as *const PendingLoopPhiOp)
                        };
                        let var_data = unsafe { &*var.data };
                        self.__output_graph().replace::<PhiOp>(
                            pending_phi_idx,
                            base::Vector::from_vec(vec![pending_phi.first(), self.get_variable(*var)]),
                            pending_phi.rep,
                        );
                    }
                }
            }
        }
        result
    }

    fn get_variable(&self, var: Variable) -> OpIndex {
        self.table_.get(var)
    }

    fn get_predecessor_value(&self, var: Variable, predecessor_index: usize) -> OpIndex {
        self.table_.get_predecessor_value(var, predecessor_index)
    }

    fn set_variable(&mut self, var: Variable, new_index: OpIndex) {
        if self.__generating_unreachable_operations() {
            return;
        }
        self.table_.set(var, new_index);
    }
    fn set<Rep>(&mut self, var: Variable, value: V<Rep>) {
        if self.__generating_unreachable_operations() {
            return;
        }
        if !V::<Rep>::allows_representation(RegisterRepresentation(var.data().rep)) {
            panic!("Invalid representation");
        }
        self.table_.set(var, value.op_index);
    }

    fn new_loop_invariant_variable(&mut self, rep: MaybeRegisterRepresentation) -> Variable {
        self.table_.new_key(VariableData { rep, loop_invariant: true, active_loop_variables_index: IntrusiveSetIndex {} }, OpIndex::invalid())
    }
    fn new_variable(&mut self, rep: MaybeRegisterRepresentation) -> Variable {
        self.table_.new_key(VariableData { rep, loop_invariant: false, active_loop_variables_index: IntrusiveSetIndex {} }, OpIndex::invalid())
    }

    fn seal_and_save_variable_snapshot(&mut self) {
        if self.table_.is_sealed() {
            assert!(self.current_block_.is_none());
            return;
        }

        assert!(self.current_block_.is_some());
        let current_block_index = self.current_block_.unwrap().index();
        self.block_to_snapshot_mapping_.table[current_block_index.value] = Some(self.table_.seal());
        self.current_block_ = None;
    }

    fn merge_op_indices(&mut self, inputs: base::Vector<OpIndex>, maybe_rep: MaybeRegisterRepresentation) -> OpIndex {
        if maybe_rep != MaybeRegisterRepresentation::None() {
            return self.__phi(inputs, RegisterRepresentation(maybe_rep));
        } else if self.__output_graph().get(inputs[0]).is::<FrameStateOp>() {
            return self.merge_frame_state(inputs);
        } else {
            return OpIndex::invalid();
        }
    }

    fn merge_frame_state(&mut self, frame_states_indices: base::Vector<OpIndex>) -> OpIndex {
        let mut frame_states: Vec<&FrameStateOp> = Vec::new();
        for idx in frame_states_indices.iter() {
            let op = self.__output_graph().get(*idx);
            frame_states.push(unsafe { &*(op as *const dyn Any as *const FrameStateOp) });
        }
        let first_frame = frame_states[0];

        #[cfg(debug_assertions)]
        {
            for frame_state in &frame_states {
                assert_eq!(first_frame.input_count, frame_state.input_count);
                assert_eq!(first_frame.inlined, frame_state.inlined);
                //assert_eq!(*first_frame.data, *frame_state.data);
            }
        }

        let mut new_inputs: Vec<OpIndex> = Vec::new();

        if first_frame.inlined {
            let mut indices_to_merge: ZoneVector<OpIndex> = ZoneVector::new(__phase_zone());
            let mut all_parent_frame_states_are_the_same = true;
            for frame_state in &frame_states {
                indices_to_merge.push(frame_state.parent_frame_state());
                all_parent_frame_states_are_the_same =
                    all_parent_frame_states_are_the_same &&
                        first_frame.parent_frame_state() == frame_state.parent_frame_state();
            }
            if all_parent_frame_states_are_the_same {
                new_inputs.push(first_frame.parent_frame_state());
            } else {
                let merged_parent_frame_state =
                    self.merge_frame_state(base::Vector::from_vec(indices_to_merge.elements));
                new_inputs.push(merged_parent_frame_state);
            }
        }

        for i in 0..first_frame.state_values_count() {
            let mut indices_to_merge: ZoneVector<OpIndex> = ZoneVector::new(__phase_zone());
            let mut all_inputs_are_the_same = true;
            for frame_state in &frame_states {
                indices_to_merge.push(frame_state.state_value(i));
                all_inputs_are_the_same =
                    all_inputs_are_the_same &&
                        first_frame.state_value(i) == frame_state.state_value(i);
            }
            if all_inputs_are_the_same {
                new_inputs.push(first_frame.state_value(i));
            } else {
                let rep = first_frame.state_value_rep(i);
                let new_input =
                    self.merge_op_indices(base::Vector::from_vec(indices_to_merge.elements), rep);
                new_inputs.push(new_input);
            }
        }

        self.__frame_state(base::Vector::from_vec(new_inputs), first_frame.inlined, first_frame.data)
    }

    fn bind(&mut self, new_block: *mut Block) {
        let new_block = unsafe { &mut *new_block };
        self.next.bind(new_block);

        self.seal_and_save_variable_snapshot();

        self.predecessors_.elements.clear();
        for pred in new_block.PredecessorsIterable() {
            let pred_snapshot = self.block_to_snapshot_mapping_.table[pred.index().value].clone();
            assert!(pred_snapshot.is_some());
            self.predecessors_.elements.push(pred_snapshot.unwrap());
        }
        self.predecessors_.elements.reverse();

        let merge_variables = |var: Variable, predecessors: base::Vector<OpIndex>| -> OpIndex {
            for idx in predecessors.iter() {
                if !idx.valid() {
                    return OpIndex::invalid();
                } else if self.__output_graph().get(*idx).is::<LoadRootRegisterOp>() {
                    return self.__load_root_register();
                }
            }
            self.merge_op_indices(predecessors, var.data().rep)
        };

        self.table_.start_new_snapshot(base::Vector::from_vec(self.predecessors_.elements.clone()), merge_variables);
        self.current_block_ = Some(new_block);
        if new_block.IsLoop() {
            let mut pending_phis: ZoneVector<std::pair<Variable, OpIndex>> = ZoneVector::new(__phase_zone());
            for var in self.table_.active_loop_variables.elements.clone() {
                let rep = var.data().rep;
                assert_ne!(rep, MaybeRegisterRepresentation::None());
                let pending_loop_phi = self.__pending_loop_phi(self.table_.get(var), RegisterRepresentation(rep));
                self.set_variable(var, pending_loop_phi.op_index);
                pending_phis.push(std::pair { first: var, second: pending_loop_phi.op_index });
            }
            self.loop_pending_phis_.insert(new_block.index(), Some(pending_phis));
        }
    }

    fn restore_temporary_variable_snapshot_after(&mut self, block: *mut Block) {
        let block = unsafe { &mut *block };
        assert!(self.table_.is_sealed());
        assert!(self.block_to_snapshot_mapping_.table[block.index().value].is_some());
        self.table_.start_new_snapshot(self.block_to_snapshot_mapping_.table[block.index().value].clone().unwrap());
        self.is_temporary_ = true;
    }

    fn close_temporary_variable_snapshot(&mut self) {
        assert!(self.is_temporary_);
        self.table_.seal();
        self.is_temporary_ = false;
    }

    fn __output_graph(&self) -> &Graph {
        todo!()
    }

    fn __generating_unreachable_operations(&self) -> bool {
        false
    }

    fn __load_root_register(&self) -> OpIndex {
        OpIndex { value: 0 }
    }

    fn __phi(&self, inputs: base::Vector<OpIndex>, rep: RegisterRepresentation) -> OpIndex {
        OpIndex { value: 0 }
    }

    fn __frame_state(&self, inputs: base::Vector<OpIndex>, inlined: bool, data: *const std::ffi::c_void) -> OpIndex {
        OpIndex { value: 0 }
    }

    fn __pending_loop_phi(&self, var: OpIndex, rep: RegisterRepresentation) -> V<Any> {
        V {
            op_index: OpIndex { value: 0 },
        }
    }
}

impl<AfterNext: TurboshaftReducer> TurboshaftReducer for VariableReducer<AfterNext> {
    fn reduce_goto(&mut self, destination: *mut Block, is_backedge: bool) -> V<void> {
        self.reduce_goto(destination, is_backedge)
    }
}

// Supporting structs/enums
pub struct RequiredOptimizationReducer<T> {
    next: T,
}

impl<T> RequiredOptimizationReducer<T> {
    pub fn bind(&mut self, _block: *mut Block) {}
}

impl<T: TurboshaftReducer> RequiredOptimizationReducer<T> {
    fn reduce_goto(&mut self, destination: *mut Block, is_backedge: bool) -> V<void> {
        self.next.reduce_goto(destination, is_backedge)
    }
}

struct VariableTable {
    active_loop_variables: ZoneIntrusiveSet<Variable>,
    table: ChangeTrackingSnapshotTable<VariableTable, OpIndex, VariableData>,
    zone: *mut Zone,
    variable_map: HashMap<Variable, OpIndex>,
    // incomplete
}

impl VariableTable {
    fn new(zone: *mut Zone) -> Self {
        VariableTable {
            active_loop_variables: ZoneIntrusiveSet::new(unsafe { &*zone }),
            table: ChangeTrackingSnapshotTable::new(),
            zone,
            variable_map: HashMap::new(),
        }
    }
    fn start_new_snapshot<F>(&mut self, predecessors: base::Vector<Snapshot>, merge_variables: F) where F: Fn(Variable, base::Vector<OpIndex>) -> OpIndex {
        // incomplete
    }
    fn get(&self, var: Variable) -> OpIndex {
        *self.variable_map.get(&var).unwrap_or(&OpIndex::invalid())
    }

    fn get_predecessor_value(&self, var: Variable, predecessor_index: usize) -> OpIndex {
        OpIndex::invalid()
    }
    fn set(&mut self, var: Variable, new_index: OpIndex) {
        self.variable_map.insert(var, new_index);
    }
    fn new_key(&mut self, data: VariableData, invalid: OpIndex) -> Variable {
        static mut NEXT_ID: usize = 0;
        unsafe {
            NEXT_ID += 1;
            let variable = Variable { id: NEXT_ID, data: Box::into_raw(Box::new(data)) };
            self.variable_map.insert(variable, invalid);
            variable
        }
    }
    fn seal(&mut self) -> Option<Snapshot> {
        Some(SnapshotTable::new())
    }
    fn is_sealed(&self) -> bool {
        false
    }
}

struct ZoneIntrusiveSet<T> {
    elements: Vec<T>,
    zone: *mut Zone,
}

impl<T: Copy + Clone> ZoneIntrusiveSet<T> {
    fn new(zone: *mut Zone) -> Self {
        ZoneIntrusiveSet { elements: Vec::new(), zone }
    }
    fn remove(&mut self, var: T) {
        self.elements.retain(|&x| x != var);
    }
    fn add(&mut self, var: T) {
        self.elements.push(var);
    }
    fn begin(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }
    fn end(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }
}

pub struct std {
    // incomplete
}

impl std {
    pub struct pair<T, U> {
        first: T,
        second: U,
    }
}

unsafe fn __phase_zone() -> *mut Zone {
    todo!()
}

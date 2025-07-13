// Converted from V8 C++ source files:
// Header: wasm-gc-typed-optimization-reducer.h
// Implementation: wasm-gc-typed-optimization-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod wasm_gc_typed_optimization_reducer {
#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::compiler::turboshaft::assembler::*;
use crate::compiler::turboshaft::operations::*;
use crate::compiler::turboshaft::phase::*;
use crate::compiler::turboshaft::snapshot_table_opindex::*;
use crate::compiler::wasm_graph_assembler::*;
use crate::wasm::wasm_subtyping::*;
use crate::wasm::wasm_module::*;
use crate::wasm::value_type::*;
use crate::base::vector::*;
use crate::compiler::turboshaft::loop_finder::*;
use crate::compiler::turboshaft::analyzer_iterator::*;
use crate::compiler::turboshaft::define_assembler_macros::*;
use crate::compiler::turboshaft::undef_assembler_macros::*;
use crate::base::logging::*;
use crate::compiler::turboshaft::*;
use crate::zone::zone::*;
use crate::flags::*;

pub struct WasmGCTypeAnalyzer<'a> {
    data_: &'a mut PipelineData,
    graph_: &'a mut Graph,
    phase_zone_: &'a Zone,
    module_: *const WasmModule,
    signature_: *const FunctionSig,
    types_table_: TypeSnapshotTable,
    block_to_snapshot_: FixedBlockSidetable<MaybeSnapshot>,
    block_is_unreachable_: BitVector,
    current_block_: Option<Rc<RefCell<Block>>>,
    input_type_map_: HashMap<OpIndex, ValueType>,
    is_first_loop_header_evaluation_: bool,
}

impl<'a> WasmGCTypeAnalyzer<'a> {
    pub fn new(data: &'a mut PipelineData, graph: &'a mut Graph, zone: &'a Zone) -> Self {
        let signature_ = data.wasm_module_sig();
        let module_ = data.wasm_module();

        WasmGCTypeAnalyzer {
            data_: data,
            graph_: graph,
            phase_zone_: zone,
            module_: module_,
            signature_: signature_,
            types_table_: TypeSnapshotTable::new(zone),
            block_to_snapshot_: FixedBlockSidetable::new(graph.block_count(), zone),
            block_is_unreachable_: BitVector::new(graph.block_count() as i32, zone),
            current_block_: None,
            input_type_map_: HashMap::new(),
            is_first_loop_header_evaluation_: false,
        }
    }

    pub fn run(&mut self) {
        let mut loop_finder = LoopFinder::new(self.phase_zone_, &self.graph_);
        let mut iterator = AnalyzerIterator::new(self.phase_zone_, &self.graph_, &mut loop_finder);

        while iterator.has_next() {
            let block = iterator.next();
            self.process_block(&block);

            self.block_to_snapshot_[block.index()] = MaybeSnapshot::new(self.types_table_.seal());

            if let Some(last) = block.last_operation(&self.graph_).try_cast::<GotoOp>() {
                if self.is_reachable(&block) && last.destination.is_loop() &&
                   last.destination.last_predecessor() == &block
                {
                   let last_op_index = self.graph_.index(block.last_operation(&self.graph_));

                    let loop_header = last.destination;
                    self.process_block(loop_header);

                    let old_snapshot = self.block_to_snapshot_[loop_header.index()].value();
                    let snapshot = self.types_table_.seal();

                    let needs_revisit = self.create_merge_snapshot(
                        Vector::from_vec(vec![old_snapshot, snapshot]),
                        Vector::from_vec(vec![true, true]),
                    );
                    self.types_table_.seal();

                    if needs_revisit {
                        self.block_to_snapshot_[loop_header.index()] = MaybeSnapshot::new(snapshot);
                        if block.index() != loop_header.index() {
                            iterator.mark_loop_for_revisit_skip_header();
                        } else {
                            iterator.mark_loop_for_revisit();
                        }
                    }
                }
            }
        }
    }

    fn start_new_snapshot_for(&mut self, block: &Block) {
        self.is_first_loop_header_evaluation_ = false;

        let block_was_previously_reachable = self.is_reachable(block);
        if !block_was_previously_reachable {
        }
        self.block_is_unreachable_.remove(block.index().id());

        if block.predecessor_count() == 0 {
            self.types_table_.start_new_snapshot();
        } else if block.is_loop() {
            let forward_predecessor =
                block.last_predecessor().neighboring_predecessor();
            if !self.is_reachable(forward_predecessor) {
                self.block_is_unreachable_.add(block.index().id());
            }
            let back_edge_snap =
                self.block_to_snapshot_[block.last_predecessor().index()];
            if back_edge_snap.has_value() && block_was_previously_reachable {
                self.create_merge_snapshot(block);
            } else {
                self.is_first_loop_header_evaluation_ = true;
                let forward_edge_snap =
                    self.block_to_snapshot_[forward_predecessor.index()].value();
                self.types_table_.start_new_snapshot_from(forward_edge_snap);
            }
        } else if block.is_branch_target() {
            let predecessor = block.last_predecessor();
            self.types_table_.start_new_snapshot_from(
                self.block_to_snapshot_[predecessor.index()].value(),
            );
            if self.is_reachable(predecessor) {
                if let Some(branch) =
                    block.predecessors()[0].last_operation(&self.graph_).try_cast::<BranchOp>()
                {
                   self.process_branch_on_target(&branch, block);
                }
            } else {
                self.block_is_unreachable_.add(block.index().id());
            }
        } else {
            self.create_merge_snapshot(block);
        }
    }

    fn process_operations(&mut self, block: &Block) {
        for op_idx in self.graph_.operation_indices(block) {
            let mut op = self.graph_.get_mut(op_idx);
            match op.opcode {
                Opcode::kWasmTypeCast => {
                    let type_cast = op.cast::<WasmTypeCastOp>();
                    self.process_type_cast(type_cast, op_idx);
                }
                Opcode::kWasmTypeCheck => {
                    let type_check = op.cast::<WasmTypeCheckOp>();
                    self.process_type_check(type_check, op_idx);
                }
                Opcode::kAssertNotNull => {
                    let assert_not_null = op.cast::<AssertNotNullOp>();
                    self.process_assert_not_null(assert_not_null, op_idx);
                }
                Opcode::kNull => {
                    let null_op = op.cast::<NullOp>();
                    self.process_null(null_op, op_idx);
                }
                Opcode::kIsNull => {
                    let is_null = op.cast::<IsNullOp>();
                    self.process_is_null(is_null, op_idx);
                }
                Opcode::kParameter => {
                    let parameter = op.cast::<ParameterOp>();
                    self.process_parameter(parameter, op_idx);
                }
                Opcode::kStructGet => {
                    let struct_get = op.cast::<StructGetOp>();
                    self.process_struct_get(struct_get, op_idx);
                }
                Opcode::kStructSet => {
                    let struct_set = op.cast::<StructSetOp>();
                    self.process_struct_set(struct_set, op_idx);
                }
                Opcode::kArrayGet => {
                    let array_get = op.cast::<ArrayGetOp>();
                    self.process_array_get(array_get, op_idx);
                }
                Opcode::kArrayLength => {
                    let array_length = op.cast::<ArrayLengthOp>();
                    self.process_array_length(array_length, op_idx);
                }
                Opcode::kGlobalGet => {
                    let global_get = op.cast::<GlobalGetOp>();
                    self.process_global_get(global_get, op_idx);
                }
                Opcode::kWasmRefFunc => {
                    let ref_func = op.cast::<WasmRefFuncOp>();
                    self.process_ref_func(ref_func, op_idx);
                }
                Opcode::kWasmAllocateArray => {
                    let allocate_array = op.cast::<WasmAllocateArrayOp>();
                    self.process_allocate_array(allocate_array, op_idx);
                }
                Opcode::kWasmAllocateStruct => {
                    let allocate_struct = op.cast::<WasmAllocateStructOp>();
                    self.process_allocate_struct(allocate_struct, op_idx);
                }
                Opcode::kPhi => {
                    let phi = op.cast::<PhiOp>();
                    self.process_phi(phi, op_idx);
                }
                Opcode::kWasmTypeAnnotation => {
                    let type_annotation = op.cast::<WasmTypeAnnotationOp>();
                    self.process_type_annotation(type_annotation, op_idx);
                }
                Opcode::kBranch => {}
                _ => {}
            }
        }
    }

    fn process_block(&mut self, block: &Block) {
        self.current_block_ = Some(Rc::new(RefCell::new(block.clone())));
        self.start_new_snapshot_for(block);
        self.process_operations(block);
        self.current_block_ = None;
    }

    fn process_type_cast(&mut self, type_cast: &WasmTypeCastOp, op_idx: OpIndex) {
        let object = type_cast.object();
        let target_type = type_cast.config.to;
        let known_input_type =
            self.refine_type_knowledge(object, target_type, &Operation::from(type_cast.clone()));
        self.input_type_map_.insert(op_idx, known_input_type);
    }

    fn process_type_check(&mut self, type_check: &WasmTypeCheckOp, op_idx: OpIndex) {
        let type_ = self.get_resolved_type(type_check.object());
        self.input_type_map_.insert(op_idx, type_);
    }

    fn process_assert_not_null(&mut self, assert_not_null: &AssertNotNullOp, op_idx: OpIndex) {
        let object = assert_not_null.object();
        let new_type = assert_not_null.type_.as_non_null();
        let known_input_type =
            self.refine_type_knowledge(object, new_type, &Operation::from(assert_not_null.clone()));
        self.input_type_map_.insert(op_idx, known_input_type);
    }

    fn process_is_null(&mut self, is_null: &IsNullOp, op_idx: OpIndex) {
        self.input_type_map_.insert(op_idx, self.get_resolved_type(is_null.object()));
    }

    fn process_parameter(&mut self, parameter: &ParameterOp, op_idx: OpIndex) {
        if parameter.parameter_index != kWasmInstanceDataParameterIndex {
            let param_type = unsafe { (*self.signature_).params[parameter.parameter_index as usize - 1]};

            self.refine_type_knowledge(
                self.graph_.index(parameter),
                param_type,
                &Operation::from(parameter.clone()),
            );
        }
    }

    fn process_struct_get(&mut self, struct_get: &StructGetOp, op_idx: OpIndex) {
        let type_ = self.refine_type_knowledge_not_null(struct_get.object(), &Operation::from(struct_get.clone()));
        self.input_type_map_.insert(op_idx, type_);
        let field_type = struct_get.type.field(struct_get.field_index).unpacked();
        self.refine_type_knowledge(self.graph_.index(struct_get), field_type, &Operation::from(struct_get.clone()));
    }

    fn process_struct_set(&mut self, struct_set: &StructSetOp, op_idx: OpIndex) {
        let type_ = self.refine_type_knowledge_not_null(struct_set.object(), &Operation::from(struct_set.clone()));
        self.input_type_map_.insert(op_idx, type_);
    }

    fn process_array_get(&mut self, array_get: &ArrayGetOp, op_idx: OpIndex) {
        self.refine_type_knowledge_not_null(array_get.array(), &Operation::from(array_get.clone()));

        let element_type = array_get.array_type.element_type().unpacked();
        self.refine_type_knowledge(self.graph_.index(array_get), element_type, &Operation::from(array_get.clone()));
    }

    fn process_array_length(&mut self, array_length: &ArrayLengthOp, op_idx: OpIndex) {
        let type_ = self.refine_type_knowledge_not_null(array_length.array(), &Operation::from(array_length.clone()));
        self.input_type_map_.insert(op_idx, type_);
    }

    fn process_global_get(&mut self, global_get: &GlobalGetOp, op_idx: OpIndex) {
        self.refine_type_knowledge(
            self.graph_.index(global_get),
            global_get.global.type,
            &Operation::from(global_get.clone()),
        );
    }

    fn process_ref_func(&mut self, ref_func: &WasmRefFuncOp, op_idx: OpIndex) {
        let sig_index = unsafe { (*self.module_).functions[ref_func.function_index as usize].sig_index };
        let heap_type = unsafe { (*self.module_).heap_type(sig_index) };
        let ref_type = ValueType::ref_(heap_type);
        self.refine_type_knowledge(
            self.graph_.index(ref_func),
            ref_type,
            &Operation::from(ref_func.clone()),
        );
    }

    fn process_allocate_array(&mut self, allocate_array: &WasmAllocateArrayOp, op_idx: OpIndex) {
        let type_index = self
            .graph_
            .get(allocate_array.rtt())
            .cast::<RttCanonOp>()
            .type_index;
        let heap_type = unsafe { (*self.module_).heap_type(type_index) };
        let ref_type = ValueType::ref_(heap_type);
        self.refine_type_knowledge(
            self.graph_.index(allocate_array),
            ref_type,
            &Operation::from(allocate_array.clone()),
        );
    }

    fn process_allocate_struct(&mut self, allocate_struct: &WasmAllocateStructOp, op_idx: OpIndex) {
        let type_index = self
            .graph_
            .get(allocate_struct.rtt())
            .cast::<RttCanonOp>()
            .type_index;
        let heap_type = unsafe { (*self.module_).heap_type(type_index) };
        let ref_type = ValueType::ref_(heap_type);
        self.refine_type_knowledge(
            self.graph_.index(allocate_struct),
            ref_type,
            &Operation::from(allocate_struct.clone()),
        );
    }

    fn get_type_for_phi_input(&mut self, phi: &PhiOp, input_index: i32) -> ValueType {
        let phi_id = self.graph_.index(phi);
        let input = self.resolve_aliases(phi.input(input_index as usize));

        if self.current_block_.as_ref().unwrap().borrow().begin().id() <= input.id() &&
           input.id() < phi_id.id()
        {
            return self.types_table_.get(input);
        }

        self.types_table_.get_predecessor_value(input, input_index as usize)
    }

    fn process_phi(&mut self, phi: &PhiOp, op_idx: OpIndex) {
        if self.is_first_loop_header_evaluation_ {
            let input_type = self.get_resolved_type(phi.input(0));
            self.refine_type_knowledge(self.graph_.index(phi), input_type, &Operation::from(phi.clone()));
            return;
        }

        let mut union_type = self.get_type_for_phi_input(phi, 0);
        if union_type == ValueType::none() {
            return;
        }

        for i in 1..phi.input_count as i32 {
            let input_type = self.get_type_for_phi_input(phi, i);
            if input_type == ValueType::none() {
                return;
            }

            if input_type.is_uninhabited() {
                continue;
            }

            if union_type.is_uninhabited() {
                union_type = input_type;
            } else {
                union_type = unsafe { Union(union_type, input_type, self.module_, self.module_).type_ };
            }
        }
        self.refine_type_knowledge(self.graph_.index(phi), union_type, &Operation::from(phi.clone()));
    }

    fn process_type_annotation(&mut self, type_annotation: &WasmTypeAnnotationOp, op_idx: OpIndex) {
        self.refine_type_knowledge(
            type_annotation.value(),
            type_annotation.type,
            &Operation::from(type_annotation.clone()),
        );
    }

    fn process_branch_on_target(&mut self, branch: &BranchOp, target: &Block) {
        let condition = self.graph_.get(branch.condition());
        match condition.opcode {
            Opcode::kWasmTypeCheck => {
                let check = condition.cast::<WasmTypeCheckOp>();
                if branch.if_true == target {
                    self.refine_type_knowledge(check.object(), check.config.to, &Operation::from(branch.clone()));
                } else {
                    if is_subtype_of(self.get_resolved_type(check.object()), check.config.to, self.module_) {
                        self.block_is_unreachable_.add(target.index().id());
                    }
                }
            }
            Opcode::kIsNull => {
                let is_null = condition.cast::<IsNullOp>();
                if branch.if_true == target {
                    if self.get_resolved_type(is_null.object()).is_non_nullable() {
                        self.block_is_unreachable_.add(target.index().id());
                        return;
                    }
                    self.refine_type_knowledge(
                        is_null.object(),
                        to_null_sentinel(is_null.type, self.module_),
                        &Operation::from(branch.clone()),
                    );
                } else {
                    self.refine_type_knowledge(is_null.object(), is_null.type.as_non_null(), &Operation::from(branch.clone()));
                }
            }
            _ => {}
        }
    }

    fn process_null(&mut self, null: &NullOp, op_idx: OpIndex) {
        let null_type = to_null_sentinel(null.type, self.module_);
        self.refine_type_knowledge(self.graph_.index(null), null_type, &Operation::from(null.clone()));
    }

    fn create_merge_snapshot(&mut self, block: &Block) -> bool {
        let mut snapshots: Vec<Snapshot> = Vec::new();
        let mut reachable: Vec<bool> = Vec::new();
        let mut all_predecessors_unreachable = true;

        for predecessor in block.predecessors_iterable() {
            snapshots.push(self.block_to_snapshot_[predecessor.index()].value());
            let predecessor_reachable = self.is_reachable(predecessor);
            reachable.push(predecessor_reachable);
            all_predecessors_unreachable &= !predecessor_reachable;
        }

        if all_predecessors_unreachable {
            self.block_is_unreachable_.add(block.index().id());
        }
        snapshots.reverse();
        reachable.reverse();
        self.create_merge_snapshot(
            Vector::from_vec(snapshots),
            Vector::from_vec(reachable),
        )
    }

    fn create_merge_snapshot(
        &mut self,
        predecessors: Vector<Snapshot>,
        reachable: Vector<bool>,
    ) -> bool {
        let mut types_are_equivalent = true;
        let module_ = self.module_;
        let mut closure = |key: TypeSnapshotTable::Key,
                           predecessors_types: Vector<ValueType>|
         -> ValueType {
            let mut i = 0;
            let mut first = ValueType::bottom();
            while i < reachable.len() {
                if reachable[i] && !predecessors_types[i].is_uninhabited() {
                    first = predecessors_types[i];
                    i += 1;
                    break;
                }
                i += 1;
            }

            let mut res = first;
            while i < reachable.len() {
                if !reachable[i] {
                    i += 1;
                    continue;
                }
                let type_ = predecessors_types[i];
                if type_.is_uninhabited() {
                    i += 1;
                    continue;
                }
                types_are_equivalent &= first == type_;
                if res == ValueType::none() || type_ == ValueType::none() {
                    res = ValueType::none();
                } else {
                    res = unsafe { Union(res, type_, module_, module_).type_ };
                }
                i += 1;
            }
            res
        };
        self.types_table_.start_new_snapshot_with_merge(predecessors, &mut closure);

        !types_are_equivalent
    }

    fn refine_type_knowledge(
        &mut self,
        object: OpIndex,
        new_type: ValueType,
        op: &Operation,
    ) -> ValueType {
        let current_block_borrowed = self.current_block_.as_ref().unwrap().borrow();
        let object = self.resolve_aliases(object);
        let previous_value = self.types_table_.get(object);
        let intersection_type = if previous_value == ValueType::none() {
            new_type
        } else {
            unsafe { Intersection(previous_value, new_type, self.module_, self.module_).type_ }
        };

        if intersection_type == previous_value {
            return previous_value;
        }

        self.types_table_.set(object, intersection_type);
        if intersection_type.is_uninhabited() {
            self.block_is_unreachable_.add(current_block_borrowed.index().id());
            return ValueType::bottom();
        }
        previous_value
    }

    fn refine_type_knowledge_not_null(&mut self, object: OpIndex, op: &Operation) -> ValueType {
        let current_block_borrowed = self.current_block_.as_ref().unwrap().borrow();
        let object = self.resolve_aliases(object);
        let previous_value = self.types_table_.get(object);
        if previous_value.is_non_nullable() {
            return previous_value;
        }

        let not_null_type = previous_value.as_non_null();

        self.types_table_.set(object, not_null_type);
        if not_null_type.is_uninhabited() {
            self.block_is_unreachable_.add(current_block_borrowed.index().id());
            return ValueType::bottom();
        }
        previous_value
    }

    fn resolve_aliases(&self, object: OpIndex) -> OpIndex {
        let mut current = object;
        loop {
            let op = self.graph_.get(current);
            match op.opcode {
                Opcode::kWasmTypeCast => {
                    current = op.cast::<WasmTypeCastOp>().object();
                }
                Opcode::kAssertNotNull => {
                    current = op.cast::<AssertNotNullOp>().object();
                }
                Opcode::kWasmTypeAnnotation => {
                    current = op.cast::<WasmTypeAnnotationOp>().value();
                }
                _ => {
                    return current;
                }
            }
        }
    }

    fn is_reachable(&self, block: &Block) -> bool {
        !self.block_is_unreachable_.contains(block.index().id())
    }

    fn get_resolved_type(&self, object: OpIndex) -> ValueType {
        self.types_table_.get(self.resolve_aliases(object))
    }

    pub fn get_input_type_or_sentinel_type(&self, op: OpIndex) -> ValueType {
        *self.input_type_map_.get(&op).unwrap()
    }
}

pub struct WasmGCTypedOptimizationReducer<'a, Next: TurboshaftReducer> {
    next: Next,
    graph_: &'a mut Graph,
    module_: *const WasmModule,
    analyzer_: WasmGCTypeAnalyzer<'a>,
}

impl<'a, Next: TurboshaftReducer> WasmGCTypedOptimizationReducer<'a, Next> {
    pub fn new(next: Next, data: &'a mut PipelineData, graph: &'a mut Graph, phase_zone: &'a Zone) -> Self {
        let module_ = data.wasm_module();
        let analyzer_ = WasmGCTypeAnalyzer::new(data, graph, phase_zone);

        WasmGCTypedOptimizationReducer {
            next: next,
            graph_: graph,
            module_: module_,
            analyzer_: analyzer_,
        }
    }
}

impl<'a, Next: TurboshaftReducer> TurboshaftReducer for WasmGCTypedOptimizationReducer<'a, Next> {
    fn analyze(&mut self) {
        self.analyzer_.run();
        self.next.analyze();
    }

    fn reduce_input_graph_wasm_type_cast(&mut self, op_idx: OpIndex, cast_op: &WasmTypeCastOp) -> OpIndex {
        let no_change = || self.next.reduce_input_graph_wasm_type_cast(op_idx, cast_op);

        if self.next.should_skip_optimization_step() {
            return no_change();
        }

        let type_ = self.analyzer_.get_input_type_or_sentinel_type(op_idx);

        if type_.is_uninhabited() {
           let trap_id = TrapId::kTrapIllegalCast;
            let trap_if = self.next.assembler().trap_if(1, trap_id);
            let unreachable = self.next.assembler().unreachable();
            return OpIndex::invalid();
        }

        if type_ != ValueType::none() {
            let module_ = self.module_;
            let cast_op_config_to_heap_type = cast_op.config.to.heap_type();
            let cast_op_config_to_is_nullable = cast_op.config.to.is_nullable();

            if is_same_type_hierarchy(type_.heap_type(), cast_op_config_to_heap_type, module_) {
               if is_heap_subtype_of(type_.heap_type(), cast_op_config_to_heap_type, module_, module_) {
                   if cast_op_config_to_is_nullable || type_.is_non_nullable() {
                       let mapped_op = self.next.map_to_new_graph(cast_op.object());
                       return mapped_op;
                   } else {
                       let mapped_op = self.next.map_to_new_graph(cast_op.object());
                       let assert_not_null = self.next.assembler().assert_not_null(mapped_op, type_, TrapId::kTrapIllegalCast);
                       return assert_not_null;
                   }
               }

               if heap_types_unrelated(type_.heap_type(), cast_op_config_to_heap_type, module_, module_) {
                  let non_trapping_condition = if type_.is_nullable() && cast_op_config_to_is_nullable {
                      let mapped_op = self.next.map_to_new_graph(cast_op.object());
                      self.next.assembler().is_null(mapped_op, type_)
                  } else {
                      self.next.assembler().word32_constant(0)
                  };
                   self.next.assembler().trap_if_not(non_trapping_condition, TrapId::kTrapIllegalCast);
                   let mapped_op = self.next.map_to_new_graph(cast_op.object());
                   return mapped_op;
               }
                let intersection_type_result = unsafe {Intersection(type_, cast_op.config.to, module_, module_).type_};
                if intersection_type_result.is_uninhabited() {
                    unreachable!("intersection type should not be uninhabited")
                }

               let intersection_type = unsafe {Intersection(type_, cast_op.config.from, module_, module_).type_};
               if intersection_type.is_uninhabited() {
                   unreachable!("intersection type should not be uninhabited")
               }

               let config = WasmTypeCheckConfig{
                   from: intersection_type,
                   to: cast_op.config.to
               };
               let mapped_object = self.next.map_to_new_graph(cast_op.object());
               let mapped_rtt = self.next.map_to_new_graph(cast_op.rtt());
               let wasm_type_cast = self.next.assembler().wasm_type_cast(mapped_object, mapped_rtt, config);
                return wasm_type_cast;
            }
        }
        no_change()
    }

   fn reduce_input_graph_wasm_type_check(&mut self, op_idx: OpIndex, type_check: &WasmTypeCheckOp) -> OpIndex {
        let no_change = || self.next.reduce_input_graph_wasm_type_check(op_idx, type_check);

        if self.next.should_skip_optimization_step() {
            return no_change();
        }

        let type_ = self.analyzer_.get_input_type_or_sentinel_type(op_idx);

        if type_.is_uninhabited() {
            let unreachable = self.next.assembler().unreachable();
            return OpIndex::invalid();
        }

        if type_ != ValueType::none() {
            let module_ = self.module_;
            let type_check_config_to_heap_type = type_check.config.to.heap_type();
            let type_check_config_to_is_nullable = type_check.config.to.is_nullable();

            if is_same_type_hierarchy(type_.heap_type(), type_check_config_to_heap_type, module_) {
               if is_heap_subtype_of(type_.heap_type(), type_check_config_to_heap_type, module_, module_) {
                   if type_check_config_to_is_nullable || type_.is_non_nullable() {
                       let word32_constant = self.next.assembler().word32_constant(1);
                       return word32_constant;
                   } else {
                       let mapped_op = self.next.map_to_new_graph(type_check.object());
                       let is_null = self.next.assembler().is_null(mapped_op, type_);
                       let word32_equal = self.next.assembler().word32_equal(is_null, 0);
                       return word32_equal;
                   }
               }

               if heap_types_unrelated(type_.heap_type(), type_check_config_to_heap_type, module_, module_) {
                   if type_check_config_to_is_nullable && type_.is_nullable() {
                       let mapped_op = self.next.map_to_new_graph(type_check.object());
                       let is_null = self.next.assembler().is_null(mapped_op, type_);
                       return is_null;
                   } else {
                       let word32_constant = self.next.assembler().word32_constant(0);
                       return word32_constant;
                   }
               }
                let intersection_type_result = unsafe {Intersection(type_, type_check.config

// Converted from V8 C++ source files:
// Header: raw-machine-assembler.h
// Implementation: raw-machine-assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
    use std::fmt;
    use std::mem;
    use std::ptr;

    pub use crate::compiler::access_builder::*;
    pub use crate::compiler::common_operator::*;
    pub use crate::compiler::linkage::*;
    pub use crate::compiler::machine_operator::*;
    pub use crate::compiler::node_matchers::*;
    pub use crate::compiler::operator::*;
    pub use crate::compiler::simplified_operator::*;
    pub use crate::compiler::turbofan_graph::*;
    pub use crate::compiler::write_barrier_kind::*;
    pub use crate::execution::isolate::*;
    pub use crate::heap::factory::*;
    pub use crate::objects::string::*;

    pub struct BasicBlock;
    pub struct RawMachineLabel;
    pub struct Schedule;
    pub struct SourcePositionTable;

    // Represents a file and line number. The file name is a C-style string,
    // so it's represented as a raw pointer.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FileAndLine {
        pub file: *const i8,
        pub line: i32,
    }

    impl FileAndLine {
        pub fn new(file: *const i8, line: i32) -> Self {
            FileAndLine { file, line }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum StackCheckKind {
        kCodeStubAssembler,
    }

    pub struct RawMachineAssembler {
        isolate_: *mut Isolate,
        graph_: *mut TFGraph,
        schedule_: *mut Schedule,
        source_positions_: *mut SourcePositionTable,
        machine_: MachineOperatorBuilder,
        common_: CommonOperatorBuilder,
        simplified_: SimplifiedOperatorBuilder,
        call_descriptor_: *mut CallDescriptor,
        dynamic_js_parameter_count_: *mut Node,
        target_parameter_: *mut Node,
        parameters_: Vec<*mut Node>,
        current_block_: *mut BasicBlock,
    }

    impl RawMachineAssembler {
        pub fn new(
            isolate: *mut Isolate,
            graph: *mut TFGraph,
            call_descriptor: *mut CallDescriptor,
            word: MachineRepresentation,
            flags: MachineOperatorBuilder::Flags,
            alignment_requirements: MachineOperatorBuilder::AlignmentRequirements,
        ) -> RawMachineAssembler {
            let zone = unsafe { (*graph).zone() };
            let schedule = unsafe {
                let layout = std::alloc::Layout::new::<Schedule>();
                let schedule_ptr = std::alloc::alloc(layout) as *mut Schedule;
                if schedule_ptr.is_null() {
                    panic!("Failed to allocate Schedule");
                }
                schedule_ptr.write(Schedule::new(zone));
                schedule_ptr
            };
            let source_positions = unsafe {
                let layout = std::alloc::Layout::new::<SourcePositionTable>();
                let source_positions_ptr = std::alloc::alloc(layout) as *mut SourcePositionTable;
                if source_positions_ptr.is_null() {
                    panic!("Failed to allocate SourcePositionTable");
                }
                source_positions_ptr.write(SourcePositionTable::new(graph));
                source_positions_ptr
            };
            let mut assembler = RawMachineAssembler {
                isolate_: isolate,
                graph_: graph,
                schedule_: schedule,
                source_positions_: source_positions,
                machine_: MachineOperatorBuilder::new(zone, word, flags, alignment_requirements),
                common_: CommonOperatorBuilder::new(zone),
                simplified_: SimplifiedOperatorBuilder::new(zone),
                call_descriptor_: call_descriptor,
                dynamic_js_parameter_count_: ptr::null_mut(),
                target_parameter_: ptr::null_mut(),
                parameters_: Vec::new(),
                current_block_: unsafe { (*schedule).start() },
            };

            let param_count = unsafe { (*call_descriptor).ParameterCount() };
            assembler.parameters_ = vec![ptr::null_mut(); param_count];

            unsafe {
                let param_count_i32 = param_count as i32;
                (*graph).SetStart((*graph).NewNode(assembler.common_.Start(param_count_i32 + 1)));
            }

            if unsafe { (*call_descriptor).IsJSFunctionCall() } {
                assembler.target_parameter_ = assembler.AddNode(
                    assembler.common_.Parameter(Linkage::kJSCallClosureParamIndex),
                    unsafe { (*graph).start() },
                );
            }

            for i in 0..param_count {
                assembler.parameters_[i] = assembler.AddNode(
                    assembler.common_.Parameter(i as i32),
                    unsafe { (*graph).start() },
                );
            }

            unsafe {
                (*graph).SetEnd((*graph).NewNode(assembler.common_.End(0)));
            }
            unsafe {
                (*assembler.source_positions_).AddDecorator();
            }
            assembler
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn graph(&self) -> *mut TFGraph {
            self.graph_
        }

        pub unsafe fn zone(&self) -> *mut Zone {
            (*self.graph_).zone()
        }

        pub fn machine(&mut self) -> &mut MachineOperatorBuilder {
            &mut self.machine_
        }

        pub fn common(&mut self) -> &mut CommonOperatorBuilder {
            &mut self.common_
        }

        pub fn simplified(&mut self) -> &mut SimplifiedOperatorBuilder {
            &mut self.simplified_
        }

        pub fn call_descriptor(&self) -> *mut CallDescriptor {
            self.call_descriptor_
        }

        pub unsafe fn ExportForTest(&mut self) -> *mut Schedule {
            // Compute the correct codegen order.
            assert!((*self.schedule_).rpo_order().is_empty());
            if V8_FLAGS.trace_turbo_scheduler {
                println!("--- RAW SCHEDULE -------------------------------------------");
                print!("{}", *self.schedule_); // Assuming Schedule implements Display
            }
            (*self.schedule_).EnsureCFGWellFormedness();
            Scheduler::ComputeSpecialRPO(self.zone(), self.schedule_);
            Scheduler::GenerateDominatorTree(self.schedule_);
            (*self.schedule_).PropagateDeferredMark();

            if V8_FLAGS.trace_turbo_scheduler {
                println!("--- EDGE SPLIT AND PROPAGATED DEFERRED SCHEDULE ------------");
                print!("{}", *self.schedule_);
            }

            (*self.source_positions_).RemoveDecorator();

            let schedule = self.schedule_;
            self.schedule_ = ptr::null_mut();
            schedule
        }

        pub unsafe fn ExportForOptimization(&mut self) -> *mut TFGraph {
            // Compute the correct codegen order.
            assert!((*self.schedule_).rpo_order().is_empty());

            if V8_FLAGS.trace_turbo_scheduler {
                println!("--- RAW SCHEDULE -------------------------------------------");
                print!("{}", *self.schedule_);
            }
            (*self.schedule_).EnsureCFGWellFormedness();
            Self::OptimizeControlFlow(self.schedule_, self.graph_, &mut self.common_);

            Scheduler::ComputeSpecialRPO(self.zone(), self.schedule_);

            if V8_FLAGS.trace_turbo_scheduler {
                println!("--- SCHEDULE BEFORE GRAPH CREATION -------------------------");
                print!("{}", *self.schedule_);
            }

            self.MakeReschedulable();

            self.schedule_ = ptr::null_mut();
            self.graph_
        }

        unsafe fn OptimizeControlFlow(
            schedule_: *mut Schedule,
            graph_: *mut TFGraph,
            common_: &mut CommonOperatorBuilder,
        ) {
            loop {
                let mut changed = false;
                let all_blocks = &(*schedule_).all_blocks_;
                for i in 0..all_blocks.len() {
                    let block = all_blocks[i];
                    if block.is_null() {
                        continue;
                    }

                    if (*block).control() == BasicBlockControl::kGoto {
                        assert_eq!((*block).SuccessorCount(), 1);
                        let successor = (*block).SuccessorAt(0);
                        if (*successor).PredecessorCount() == 1 {
                            assert_eq!((*successor).PredecessorAt(0), block);

                            for node_ptr in (*successor).iter() {
                                let node = *node_ptr;
                                (*schedule_).SetBlockForNode(ptr::null_mut(), node);
                                (*schedule_).AddNode(block, node);
                            }
                            (*block).set_control((*successor).control());
                            (*block).set_control_input((*successor).control_input());

                            if (*successor).control_input().is_null() == false {
                                (*schedule_).SetBlockForNode(block, (*successor).control_input());
                            }
                            if (*successor).deferred() {
                                (*block).set_deferred(true);
                            }
                            (*block).ClearSuccessors();
                            (*schedule_).MoveSuccessors(successor, block);
                            (*schedule_).ClearBlockById((*successor).id());
                            changed = true;
                            //Decrement i, because we changed the vector all_blocks
                            continue;
                        }
                    }

                    if (*block).control() == BasicBlockControl::kBranch
                        && (*block).NodeCount() == 1
                    {
                        let phi = (*block).NodeAt(0);
                        if (*(*phi)).opcode() != IrOpcode::kPhi {
                            continue;
                        }
                        let branch = (*block).control_input();
                        assert_eq!((*branch).opcode(), IrOpcode::kBranch);
                        if NodeProperties::GetValueInput(branch, 0) != phi {
                            continue;
                        }
                        if (*phi).UseCount() != 1 {
                            continue;
                        }
                        assert_eq!((*(*phi).op()).ValueInputCount(), (*block).PredecessorCount());

                        assert_eq!((*block).SuccessorCount(), 2);
                        let true_block = (*block).SuccessorAt(0);
                        let false_block = (*block).SuccessorAt(1);
                        assert_eq!((*(*true_block).NodeAt(0)).opcode(), IrOpcode::kIfTrue);
                        assert_eq!((*(*false_block).NodeAt(0)).opcode(), IrOpcode::kIfFalse);
                        (**(*true_block).begin()).Kill();
                        (*true_block).RemoveNode((*true_block).begin());
                        (**(*false_block).begin()).Kill();
                        (*false_block).RemoveNode((*false_block).begin());
                        (*true_block).ClearPredecessors();
                        (*false_block).ClearPredecessors();

                        let arity = (*block).PredecessorCount();
                        for j in 0..arity {
                            let predecessor = (*block).PredecessorAt(j);
                            (*predecessor).ClearSuccessors();
                            if (*block).deferred() {
                                (*predecessor).set_deferred(true);
                            }
                            let branch_clone = (*graph_).CloneNode(branch);
                            let phi_input = j as i32;
                            NodeProperties::ReplaceValueInput(
                                branch_clone,
                                NodeProperties::GetValueInput(phi, phi_input),
                                0,
                            );
                            let new_true_block = (*schedule_).NewBasicBlock();
                            let new_false_block = (*schedule_).NewBasicBlock();
                            (*new_true_block).AddNode((*graph_).NewNode(
                                common_.IfTrue(),
                                branch_clone,
                            ));
                            (*new_false_block).AddNode((*graph_).NewNode(
                                common_.IfFalse(),
                                branch_clone,
                            ));
                            (*schedule_).AddGoto(new_true_block, true_block);
                            (*schedule_).AddGoto(new_false_block, false_block);

                            assert_eq!((*predecessor).control(), BasicBlockControl::kGoto);
                            (*predecessor).set_control(BasicBlockControl::kNone);
                            (*schedule_).AddBranch(
                                predecessor,
                                branch_clone,
                                new_true_block,
                                new_false_block,
                            );
                        }
                        (*branch).Kill();
                        (*schedule_).ClearBlockById((*block).id());
                        changed = true;
                        continue;
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        fn MakeReschedulable(&mut self) {
            let mut block_final_control: Vec<*mut Node> =
                vec![ptr::null_mut(); unsafe { (*self.schedule_).all_blocks_.len() }];
            let mut block_final_effect: Vec<*mut Node> =
                vec![ptr::null_mut(); unsafe { (*self.schedule_).all_blocks_.len() }];

            struct LoopHeader {
                block: *mut BasicBlock,
                loop_node: *mut Node,
                effect_phi: *mut Node,
            }
            let mut loop_headers: Vec<LoopHeader> = Vec::new();

            let mut merge_inputs: Vec<*mut Node> = Vec::new();
            let mut effect_phi_inputs: Vec<*mut Node> = Vec::new();

            unsafe {
                for block in (*self.schedule_).rpo_order().iter() {
                    let mut current_control: *mut Node;
                    let mut current_effect: *mut Node;

                    if *block == (*self.schedule_).start() {
                        current_control = (*self.graph_).start();
                        current_effect = (*self.graph_).start();
                    } else if *block == (*self.schedule_).end() {
                        for i in 0..(*(*block)).PredecessorCount() {
                            NodeProperties::MergeControlToEnd(
                                (*self.graph_),
                                &mut self.common_,
                                (*(*(*block)).PredecessorAt(i)).control_input(),
                            );
                        }
                        continue;
                    } else if (*(*block)).IsLoopHeader() {
                        current_control = (*self.graph_).NewNode(
                            self.common_.Loop(2),
                            (*self.graph_).start(),
                            (*self.graph_).start(),
                        );
                        current_effect = (*self.graph_).NewNode(
                            self.common_.EffectPhi(2),
                            (*self.graph_).start(),
                            (*self.graph_).start(),
                            current_control,
                        );

                        let terminate = (*self.graph_).NewNode(
                            self.common_.Terminate(),
                            current_effect,
                            current_control,
                        );

                        NodeProperties::MergeControlToEnd((*self.graph_), &mut self.common_, terminate);
                        loop_headers.push(LoopHeader {
                            block: *block,
                            loop_node: current_control,
                            effect_phi: current_effect,
                        });
                    } else if (*(*block)).PredecessorCount() == 1 {
                        let predecessor = (*(*block)).PredecessorAt(0);
                        assert!((*predecessor).rpo_number() < (*(*block)).rpo_number());
                        current_effect = block_final_effect[(*predecessor).id().ToSize()];
                        current_control = block_final_control[(*predecessor).id().ToSize()];
                    } else {
                        merge_inputs.clear();
                        effect_phi_inputs.clear();
                        let predecessor_count = (*(*block)).PredecessorCount() as i32;
                        for i in 0..predecessor_count {
                            let predecessor = (*(*block)).PredecessorAt(i as usize);
                            assert!((*predecessor).rpo_number() < (*(*block)).rpo_number());
                            merge_inputs.push(
                                block_final_control[(*predecessor).id().ToSize()],
                            );
                            effect_phi_inputs.push(
                                block_final_effect[(*predecessor).id().ToSize()],
                            );
                        }
                        current_control = (*self.graph_).NewNode(
                            self.common_.Merge(predecessor_count as usize),
                            merge_inputs.len() as i32,
                            merge_inputs.as_ptr(),
                        );
                        effect_phi_inputs.push(current_control);
                        current_effect = (*self.graph_).NewNode(
                            self.common_.EffectPhi(predecessor_count as usize),
                            effect_phi_inputs.len() as i32,
                            effect_phi_inputs.as_ptr(),
                        );
                    }

                    let update_current_control_and_effect = |node: *mut Node| {
                        let existing_effect_and_control =
                            IrOpcode::IsIfProjectionOpcode((*node).opcode())
                            || IrOpcode::IsPhiOpcode((*node).opcode());

                        if (*node).op().EffectInputCount() > 0 {
                            assert_eq!(1, (*node).op().EffectInputCount());
                            if existing_effect_and_control {
                                NodeProperties::ReplaceEffectInput(node, current_effect);
                            } else {
                                (*node).AppendInput((*self.graph_).zone(), current_effect);
                            }
                        }
                        if (*node).op().ControlInputCount() > 0 {
                            assert_eq!(1, (*node).op().ControlInputCount());
                            if existing_effect_and_control {
                                NodeProperties::ReplaceControlInput(node, current_control);
                            } else {
                                (*node).AppendInput((*self.graph_).zone(), current_control);
                            }
                        }

                        if (*node).op().EffectOutputCount() > 0 {
                            assert_eq!(1, (*node).op().EffectOutputCount());
                            current_effect = node;
                        }
                        if (*node).op().ControlOutputCount() > 0 {
                            current_control = node;
                        }
                    };

                    for node_ptr in (*(*block)).iter() {
                        update_current_control_and_effect(*node_ptr);
                    }
                    if (*(*block)).deferred() {
                        self.MarkControlDeferred(current_control);
                    }

                    if let Some(block_terminator) =
                        Some((*(*block)).control_input()).filter(|p| !p.is_null())
                    {
                        update_current_control_and_effect(block_terminator);
                    }

                    block_final_effect[(*(*block)).id().ToSize()] = current_effect;
                    block_final_control[(*(*block)).id().ToSize()] = current_control;
                }

                for loop_header in loop_headers.iter() {
                    let block = loop_header.block;

                    let mut loop_entries: Vec<*mut BasicBlock> = Vec::new();
                    let mut loop_backedges: Vec<*mut BasicBlock> = Vec::new();

                    for i in 0..(*(*block)).PredecessorCount() {
                        let predecessor = (*(*block)).PredecessorAt(i as usize);
                        if (*(*block)).LoopContains(predecessor) {
                            loop_backedges.push(predecessor);
                        } else {
                            assert!(loop_backedges.is_empty());
                            loop_entries.push(predecessor);
                        }
                    }
                    assert!(!loop_entries.is_empty());
                    assert!(!loop_backedges.is_empty());

                    let entrance_count = loop_entries.len();
                    let backedge_count = loop_backedges.len();

                    let control_loop_entry = self.CreateNodeFromPredecessors(
                        &loop_entries,
                        &block_final_control,
                        self.common_.Merge(entrance_count),
                        &[],
                    );

                    let control_backedge = self.CreateNodeFromPredecessors(
                        &loop_backedges,
                        &block_final_control,
                        self.common_.Merge(backedge_count),
                        &[],
                    );

                    let effect_loop_entry = self.CreateNodeFromPredecessors(
                        &loop_entries,
                        &block_final_effect,
                        self.common_.EffectPhi(entrance_count),
                        &[control_loop_entry],
                    );
                    let effect_backedge = self.CreateNodeFromPredecessors(
                        &loop_backedges,
                        &block_final_effect,
                        self.common_.EffectPhi(backedge_count),
                        &[control_backedge],
                    );

                    (*loop_header.loop_node).ReplaceInput(0, control_loop_entry);
                    (*loop_header.loop_node).ReplaceInput(1, control_backedge);
                    (*loop_header.effect_phi).ReplaceInput(0, effect_loop_entry);
                    (*loop_header.effect_phi).ReplaceInput(1, effect_backedge);

                    for node_ptr in (*(*block)).iter() {
                        if (*node_ptr).opcode() == IrOpcode::kPhi {
                            self.MakePhiBinary(
                                *node_ptr,
                                loop_entries.len() as i32,
                                control_loop_entry,
                                control_backedge,
                            );
                        }
                    }
                }
            }
        }

        fn CreateNodeFromPredecessors(
            &mut self,
            predecessors: &Vec<*mut BasicBlock>,
            sidetable: &Vec<*mut Node>,
            op: &Operator,
            additional_inputs: &[*mut Node],
        ) -> *mut Node {
            unsafe {
                if predecessors.len() == 1 {
                    return sidetable[(*predecessors.first().unwrap()).id().ToSize()];
                }

                let mut inputs: Vec<*mut Node> = Vec::new();
                inputs.reserve(predecessors.len() + additional_inputs.len());

                for predecessor in predecessors {
                    inputs.push(sidetable[(*(*predecessor)).id().ToSize()]);
                }
                for additional_input in additional_inputs {
                    inputs.push(*additional_input);
                }

                (*self.graph_).NewNode(op, inputs.len() as i32, inputs.as_ptr())
            }
        }

        fn MakePhiBinary(&mut self, phi: *mut Node, split_point: i32, left_control: *mut Node, right_control: *mut Node) {
            unsafe {
                let value_count = (*phi).op().ValueInputCount();
                if value_count == 2 {
                    return;
                }
                assert!(split_point < value_count as i32);
                assert!(split_point > 0);

                let rep = PhiRepresentationOf((*phi).op());

                let left_input_count = split_point;
                let right_input_count = value_count as i32 - split_point;

                let mut left_input: *mut Node;
                if left_input_count == 1 {
                    left_input = NodeProperties::GetValueInput(phi, 0);
                } else {
                    let mut inputs: Vec<*mut Node> = Vec::new();
                    inputs.reserve(left_input_count as usize);
                    for i in 0..left_input_count {
                        inputs.push(NodeProperties::GetValueInput(phi, i));
                    }
                    inputs.push(left_control);

                    left_input = (*self.graph_).NewNode(
                        self.common_.Phi(rep, left_input_count as usize),
                        inputs.len() as i32,
                        inputs.as_ptr(),
                    );
                }

                let mut right_input: *mut Node;
                if right_input_count == 1 {
                    right_input = NodeProperties::GetValueInput(phi, split_point as i32);
                } else {
                    let mut inputs: Vec<*mut Node> = Vec::new();
                    for i in split_point..value_count as i32 {
                        inputs.push(NodeProperties::GetValueInput(phi, i));
                    }
                    inputs.push(right_control);

                    right_input = (*self.graph_).NewNode(
                        self.common_.Phi(rep, right_input_count as usize),
                        inputs.len() as i32,
                        inputs.as_ptr(),
                    );
                }
                let control = NodeProperties::GetControlInput(phi);
                (*phi).TrimInputCount(3);
                (*phi).ReplaceInput(0, left_input);
                (*phi).ReplaceInput(1, right_input);
                (*phi).ReplaceInput(2, control);
                NodeProperties::ChangeOp(phi, self.common_.Phi(rep, 2));
            }
        }

        fn MarkControlDeferred(&mut self, control_node: *mut Node) {
            unsafe {
                let mut responsible_branch: *mut Node = ptr::null_mut();
                let mut new_branch_hint: BranchHint = BranchHint::kNone;
                let mut current_control = control_node;
                while responsible_branch.is_null() {
                    match (*current_control).opcode() {
                        IrOpcode::kIfException => {
                            return;
                        }
                        IrOpcode::kIfSuccess => {
                            current_control = NodeProperties::GetControlInput(current_control);
                        }
                        IrOpcode::kIfValue => {
                            let parameters = IfValueParametersOf((*current_control).op());
                            if parameters.hint() != BranchHint::kFalse {
                                NodeProperties::ChangeOp(
                                    current_control,
                                    self.common_.IfValue(
                                        parameters.value(),
                                        parameters.comparison_order(),
                                        BranchHint::kFalse,
                                    ),
                                );
                            }
                            return;
                        }
                        IrOpcode::kIfDefault => {
                            if BranchHintOf((*current_control).op()) != BranchHint::kFalse {
                                NodeProperties::ChangeOp(
                                    current_control,
                                    self.common_.IfDefault(BranchHint::kFalse),
                                );
                            }
                            return;
                        }
                        IrOpcode::kIfTrue => {
                            let branch = NodeProperties::GetControlInput(current_control);
                            let hint = BranchHintOf((*branch).op());
                            if hint == BranchHint::kTrue {
                                current_control = NodeProperties::GetControlInput(branch);
                                continue;
                            }
                            new_branch_hint = BranchHint::kFalse;
                            responsible_branch = branch;
                            break;
                        }
                        IrOpcode::kIfFalse => {
                            let branch = NodeProperties::GetControlInput(current_control);
                            let hint = BranchHintOf((*branch).op());
                            if hint == BranchHint::kFalse {
                                current_control = NodeProperties::GetControlInput(branch);
                                continue;
                            }
                            new_branch_hint = BranchHint::kTrue;
                            responsible_branch = branch;
                            break;
                        }
                        IrOpcode::kMerge => {
                            for i in 0..(*current_control).op().ControlInputCount() {
                                self.MarkControlDeferred(
                                    NodeProperties::GetControlInput(current_control, i),
                                );
                            }
                            return;
                        }
                        IrOpcode::kLoop => {
                            current_control = NodeProperties::GetControlInput(current_control, 0);
                            continue;
                        }
                        IrOpcode::kBranch | IrOpcode::kSwitch => unreachable!(),
                        IrOpcode::kStart => {
                            return;
                        }
                        _ => {
                            assert_eq!(1, (*current_control).op().ControlInputCount());
                            current_control = NodeProperties::GetControlInput(current_control);
                            continue;
                        }
                    }
                }
                let hint = BranchHintOf((*responsible_branch).op());
                if hint == new_branch_hint {
                    return;
                }
                NodeProperties::ChangeOp(responsible_branch, self.common_.Branch(new_branch_hint));
            }
        }

        pub fn TargetParameter(&self) -> *mut Node {
            assert!(!self.target_parameter_.is_null());
            self.target_parameter_
        }

        pub fn Parameter(&self, index: usize) -> *mut Node {
            assert!(index < self.parameter_count());
            self.parameters_[index]
        }

        pub fn SetCurrentExternalSourcePosition(
            &mut self,
            file_and_line: FileAndLine,
        ) {
            unsafe {
                let file_id = (*self.isolate_)
                    .LookupOrAddExternallyCompiledFilename(file_and_line.file);
                let p = SourcePosition::External(file_and_line.line, file_id);
                assert_eq!(p.ExternalLine(), file_and_line.line);
                (*self.source_positions_).SetCurrentPosition(p);
            }
        }

        pub fn GetCurrentExternalSourcePosition(&self) -> FileAndLine {
            unsafe {
                let p = (*self.source_positions_).GetCurrentPosition();
                if !p.IsKnown() {
                    return FileAndLine::new(ptr::null(), -1);
                }
                let file_id = p.ExternalFileId();
                let file_name = (*self.isolate_).GetExternallyCompiledFilename(file_id);
                let line = p.ExternalLine();
                FileAndLine::new(file_name, line)
            }
        }

        pub fn source_positions(&mut self) -> *mut SourcePositionTable {
            self.source_positions_
        }

        pub fn parameter_count(&self) -> usize {
            unsafe { (*self.call_descriptor_).ParameterCount() }
        }

        pub fn dynamic_js_parameter_count(&self) -> *mut Node {
            self.dynamic_js_parameter_count_
        }

        pub fn set_dynamic_js_parameter_count(&mut self, parameter_count: *mut Node) {
            self.dynamic_js_parameter_count_ = parameter_count;
        }

        pub fn NullConstant(&mut self) -> *mut Node {
            unsafe {
                let null_value = (*(*self.isolate()).factory()).null_value();
                self.HeapConstant(null_value)
            }
        }

        pub fn UndefinedConstant(&mut self) -> *mut Node {
            unsafe {
                let undefined_value = (*(*self.isolate()).factory()).undefined_value();
                self.HeapConstant(undefined_value)
            }
        }

        pub fn PointerConstant(&mut self, value: *mut u8) -> *mut Node {
            self.IntPtrConstant(value as intptr_t)
        }

        pub fn IntPtrConstant(&mut self, value: intptr_t) -> *mut Node {
            if kSystemPointerSize == 8 {
                self.Int64Constant(value)
            } else {
                self.Int32Constant(value as i32)
            }
        }

        pub fn RelocatableIntPtrConstant(&mut self, value: intptr_t, rmode: RelocInfo::Mode) -> *mut Node {
            if kSystemPointerSize == 8 {
                self.RelocatableInt64Constant(value, rmode)
            } else {
                self.RelocatableInt32Constant(value as i32, rmode)
            }
        }

        pub fn Int32Constant(&mut self, value: i32) -> *mut Node {
            self.AddNode(self.common_.Int32Constant(value))
        }

        pub fn StackSlot(&mut self, rep: MachineRepresentation, alignment: i32) -> *mut Node {
            self.AddNode(self.machine_.StackSlot(rep, alignment))
        }

        pub fn StackSlot2(&mut self, size: i32, alignment: i32) -> *mut Node {
            self.AddNode(self.machine_.StackSlot2(size, alignment))
        }

        pub fn Int64Constant(&mut self, value: i64) -> *mut Node {
            self.AddNode(self.common_.Int64Constant(value))
        }

        pub fn NumberConstant(&mut self, value: f64) -> *mut Node {
            self.AddNode(self.common_.NumberConstant(value))
        }

        pub fn Float32Constant(&mut self, value: f32) -> *mut Node {
            self.AddNode(self.common_.Float32Constant(value))
        }

        pub fn Float64Constant(&mut self, value: f64) -> *mut Node {
            self.AddNode(self.common_.Float64Constant(value))
        }

        pub unsafe fn HeapConstant(&mut self, object: Tagged<HeapObject>) -> *mut Node {
            self.AddNode(self.common_.HeapConstant(object))
        }

        pub fn ExternalConstant(&mut self, address: ExternalReference) -> *mut Node {
            self.AddNode(self.common_.ExternalConstant(address))
        }

        pub fn RelocatableInt32Constant(&mut self, value: i32, rmode: RelocInfo::Mode) -> *mut Node {
            self.AddNode(self.common_.RelocatableInt32Constant(value, rmode))
        }

        pub fn RelocatableInt64Constant(&mut self, value: i64, rmode: RelocInfo::Mode) -> *mut Node {
            self.AddNode(self.common_.RelocatableInt64Constant(value, rmode))
        }

        pub fn Projection(&mut self, index: i32, a: *mut Node) -> *mut Node {
            self.AddNode(self.common_.Projection(index), 1, &a)
        }

        pub fn Load(&mut self, type_: MachineType, base: *mut Node) -> *mut Node {
            self.Load2(type_, base, self.IntPtrConstant(0))
        }

        pub fn Load2(&mut self, type_: MachineType, base: *mut Node, index: *mut Node) -> *mut Node {
            let op = self.machine_.Load(type_);
            let load = self.AddNode(op, 2, &[base, index]);
            load
        }

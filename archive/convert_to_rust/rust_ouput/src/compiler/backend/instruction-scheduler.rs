// Converted from V8 C++ source files:
// Header: instruction-scheduler.h
// Implementation: instruction-scheduler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod utils {
        pub struct RandomNumberGenerator {
            seed: u32,
        }

        impl RandomNumberGenerator {
            pub fn new(seed: u32) -> Self {
                RandomNumberGenerator { seed }
            }

            pub fn NextInt(&mut self, max: i32) -> i32 {
                self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
                (self.seed as i32).abs() % max
            }
        }
    }

    pub mod iterator {
      pub struct Reversed<T> {
        data: Vec<T>
      }
    
      impl<T> Reversed<T> {
        pub fn new(data: Vec<T>) -> Self {
          Reversed { data }
        }
    
        pub fn into_iter(self) -> std::vec::IntoIter<T> {
            self.data.into_iter().rev().collect::<Vec<_>>().into_iter()
        }
      }

      pub fn Reversed<T>(vec: &Vec<T>) -> Reversed<&T> where T: Copy {
        let mut reversed_vec: Vec<&T> = vec.iter().rev().collect();
        Reversed{data: reversed_vec}
      }
    }
}

pub mod compiler {
    pub mod backend {
        use crate::base::utils::RandomNumberGenerator;
        use std::collections::{HashMap, LinkedList};
        use std::ops::Deref;
        use std::{
            cell::{RefCell, RefMut},
            optional::Option,
        };

        pub struct Zone {
            name: String,
        }

        impl Zone {
            pub fn new(name: String) -> Self {
                Zone { name }
            }

            pub fn New<T>(&self, value: T) -> Box<T> {
                Box::new(value)
            }
        }

        pub struct ZoneObject {}

        pub type RpoNumber = i32;

        pub struct InstructionSequence {
            blocks: Vec<BasicBlock>,
        }

        impl InstructionSequence {
            pub fn new() -> Self {
                InstructionSequence { blocks: Vec::new() }
            }

            pub fn StartBlock(&mut self, rpo: RpoNumber) {
                self.blocks.push(BasicBlock::new(rpo));
            }

            pub fn EndBlock(&mut self, _rpo: RpoNumber) {}

            pub fn AddInstruction(&mut self, _instr: *mut Instruction) {}
        }

        struct BasicBlock {
            rpo: RpoNumber,
        }

        impl BasicBlock {
            fn new(rpo: RpoNumber) -> Self {
                BasicBlock { rpo }
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum ArchOpcodeFlags {
            kNoOpcodeFlags = 0,
            kHasSideEffect = 1,
            kIsLoadOperation = 2,
            kMayNeedDeoptOrTrapCheck = 4,
            kIsBarrier = 8,
        }

        #[derive(Debug)]
        pub struct Instruction {
            arch_opcode: ArchOpcode,
            flags_mode: FlagsMode,
            memory_access_mode: MemoryAccessMode,
            inputs: Vec<InstructionOperand>,
            outputs: Vec<InstructionOperand>,
            is_trap: bool,
            id: i32,
        }

        #[derive(Debug, PartialEq, Copy, Clone)]
        pub enum ArchOpcode {
            kArchNop,
            kArchStackCheckOffset,
            kArchFramePointer,
            kArchParentFramePointer,
            kArchStackSlot,
            kArchComment,
            kArchDeoptimize,
            kArchJmp,
            kArchBinarySearchSwitch,
            kArchRet,
            kArchTableSwitch,
            kArchThrowTerminator,
            kArchTruncateDoubleToI,
            kIeee754Float64Acos,
            kIeee754Float64Acosh,
            kIeee754Float64Asin,
            kIeee754Float64Asinh,
            kIeee754Float64Atan,
            kIeee754Float64Atanh,
            kIeee754Float64Atan2,
            kIeee754Float64Cbrt,
            kIeee754Float64Cos,
            kIeee754Float64Cosh,
            kIeee754Float64Exp,
            kIeee754Float64Expm1,
            kIeee754Float64Log,
            kIeee754Float64Log1p,
            kIeee754Float64Log10,
            kIeee754Float64Log2,
            kIeee754Float64Pow,
            kIeee754Float64Sin,
            kIeee754Float64Sinh,
            kIeee754Float64Tan,
            kIeee754Float64Tanh,
            kArchStackPointerGreaterThan,
            kArchStackPointer,
            kArchSetStackPointer,
            kArchPrepareCallCFunction,
            kArchPrepareTailCall,
            kArchTailCallCodeObject,
            kArchTailCallAddress,
            kArchTailCallWasm,
            kArchTailCallWasmIndirect,
            kArchAbortCSADcheck,
            kArchDebugBreak,
            kArchSaveCallerRegisters,
            kArchRestoreCallerRegisters,
            kArchCallCFunction,
            kArchCallCFunctionWithFrameState,
            kArchCallCodeObject,
            kArchCallJSFunction,
            kArchCallWasmFunction,
            kArchCallWasmFunctionIndirect,
            kArchCallBuiltinPointer,
            kArchStoreWithWriteBarrier,
            kArchAtomicStoreWithWriteBarrier,
            kArchStoreIndirectWithWriteBarrier,
            kAtomicLoadInt8,
            kAtomicLoadUint8,
            kAtomicLoadInt16,
            kAtomicLoadUint16,
            kAtomicLoadWord32,
            kAtomicStoreWord8,
            kAtomicStoreWord16,
            kAtomicStoreWord32,
            kAtomicExchangeInt8,
            kAtomicExchangeUint8,
            kAtomicExchangeInt16,
            kAtomicExchangeUint16,
            kAtomicExchangeWord32,
            kAtomicCompareExchangeInt8,
            kAtomicCompareExchangeUint8,
            kAtomicCompareExchangeInt16,
            kAtomicCompareExchangeUint16,
            kAtomicCompareExchangeWord32,
            kAtomicAddInt8,
            kAtomicAddUint8,
            kAtomicAddInt16,
            kAtomicAddUint16,
            kAtomicAddWord32,
            kAtomicSubInt8,
            kAtomicSubUint8,
            kAtomicSubInt16,
            kAtomicSubUint16,
            kAtomicSubWord32,
            kAtomicAndInt8,
            kAtomicAndUint8,
            kAtomicAndInt16,
            kAtomicAndUint16,
            kAtomicAndWord32,
            kAtomicOrInt8,
            kAtomicOrUint8,
            kAtomicOrInt16,
            kAtomicOrUint16,
            kAtomicOrWord32,
            kAtomicXorInt8,
            kAtomicXorUint8,
            kAtomicXorInt16,
            kAtomicXorUint16,
            kAtomicXorWord32,
            kTargetArchOpcodePlaceholder,
        }

        impl Instruction {
            pub fn new(arch_opcode: ArchOpcode) -> Self {
                Instruction {
                    arch_opcode,
                    flags_mode: FlagsMode::kNone,
                    memory_access_mode: MemoryAccessMode::kMemoryAccessNone,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    is_trap: false,
                    id: 0,
                }
            }

            pub fn arch_opcode(&self) -> ArchOpcode {
                self.arch_opcode
            }

            pub fn flags_mode(&self) -> FlagsMode {
                self.flags_mode
            }

            pub fn memory_access_mode(&self) -> MemoryAccessMode {
                self.memory_access_mode
            }

            pub fn IsTrap(&self) -> bool {
                self.is_trap
            }

            pub fn HasMemoryAccessMode(&self) -> bool {
                self.memory_access_mode != MemoryAccessMode::kMemoryAccessNone
            }

            pub fn InputCount(&self) -> usize {
                self.inputs.len()
            }

            pub fn OutputCount(&self) -> usize {
                self.outputs.len()
            }

            pub fn InputAt(&self, index: usize) -> &InstructionOperand {
                &self.inputs[index]
            }

            pub fn OutputAt(&self, index: usize) -> &InstructionOperand {
                &self.outputs[index]
            }

            pub fn memory_access_mode(&self) -> MemoryAccessMode {
                self.memory_access_mode
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum FlagsMode {
            kNone,
            kFlags_branch,
        }

        #[derive(Debug, PartialEq)]
        pub enum MemoryAccessMode {
            kMemoryAccessNone,
            kMemoryAccessDirect,
        }

        #[derive(Debug)]
        pub enum InstructionOperand {
            Unallocated(UnallocatedOperand),
            Constant(ConstantOperand),
        }

        impl InstructionOperand {
            pub fn IsUnallocated(&self) -> bool {
                match self {
                    InstructionOperand::Unallocated(_) => true,
                    _ => false,
                }
            }

            pub fn IsConstant(&self) -> bool {
                match self {
                    InstructionOperand::Constant(_) => true,
                    _ => false,
                }
            }
        }

        #[derive(Debug)]
        pub struct UnallocatedOperand {
            virtual_register: i32,
            fixed_register_policy: bool,
            fixed_fp_register_policy: bool,
        }

        impl UnallocatedOperand {
            pub fn cast(operand: &InstructionOperand) -> &UnallocatedOperand {
                match operand {
                    InstructionOperand::Unallocated(unallocated) => unallocated,
                    _ => panic!("Cannot cast to UnallocatedOperand"),
                }
            }
            pub fn virtual_register(&self) -> i32 {
                self.virtual_register
            }

            pub fn HasFixedRegisterPolicy(&self) -> bool {
                self.fixed_register_policy
            }

            pub fn HasFixedFPRegisterPolicy(&self) -> bool {
                self.fixed_fp_register_policy
            }
        }

        #[derive(Debug)]
        pub struct ConstantOperand {
            virtual_register: i32,
        }
        impl ConstantOperand {
            pub fn cast(operand: &InstructionOperand) -> &ConstantOperand {
                match operand {
                    InstructionOperand::Constant(constant) => constant,
                    _ => panic!("Cannot cast to ConstantOperand"),
                }
            }
            pub fn virtual_register(&self) -> i32 {
                self.virtual_register
            }
        }

        pub struct InstructionScheduler {
            zone_: Zone,
            sequence_: InstructionSequence,
            graph_: Vec<Box<ScheduleGraphNode>>,
            last_side_effect_instr_: Option<Box<ScheduleGraphNode>>,
            pending_loads_: Vec<Box<ScheduleGraphNode>>,
            last_live_in_reg_marker_: Option<Box<ScheduleGraphNode>>,
            last_deopt_or_trap_: Option<Box<ScheduleGraphNode>>,
            operands_map_: HashMap<i32, Box<ScheduleGraphNode>>,
            random_number_generator_: Option<RandomNumberGenerator>,
        }

        impl InstructionScheduler {
            pub fn new(zone: Zone, sequence: InstructionSequence) -> Self {
                InstructionScheduler {
                    zone_: zone,
                    sequence_: sequence,
                    graph_: Vec::new(),
                    last_side_effect_instr_: None,
                    pending_loads_: Vec::new(),
                    last_live_in_reg_marker_: None,
                    last_deopt_or_trap_: None,
                    operands_map_: HashMap::new(),
                    random_number_generator_: if v8_flags::turbo_stress_instruction_scheduling {
                        Some(RandomNumberGenerator::new(v8_flags::random_seed))
                    } else {
                        None
                    },
                }
            }

            pub fn StartBlock(&mut self, rpo: RpoNumber) {
                assert!(self.graph_.is_empty());
                assert!(self.last_side_effect_instr_.is_none());
                assert!(self.pending_loads_.is_empty());
                assert!(self.last_live_in_reg_marker_.is_none());
                assert!(self.last_deopt_or_trap_.is_none());
                assert!(self.operands_map_.is_empty());
                self.sequence_.StartBlock(rpo);
            }

            pub fn EndBlock(&mut self, rpo: RpoNumber) {
                if v8_flags::turbo_stress_instruction_scheduling {
                    self.Schedule::<StressSchedulerQueue>();
                } else {
                    self.Schedule::<CriticalPathFirstQueue>();
                }
                self.sequence_.EndBlock(rpo);
            }

            pub fn AddTerminator(&mut self, instr: *mut Instruction) {
                let new_node = self.zone_.New(ScheduleGraphNode::new(&self.zone_, instr));
                let new_node_ptr = unsafe { new_node.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };

                // Make sure that basic block terminators are not moved by adding them
                // as successor of every instruction.
                for node in &self.graph_ {
                    node.AddSuccessor(unsafe { &mut *new_node_ptr });
                }
                self.graph_.push(new_node);
            }

            pub fn AddInstruction(&mut self, instr: *mut Instruction) {
                let instr_ref = unsafe { instr.as_ref().unwrap() };
                if self.IsBarrier(instr_ref) {
                    if v8_flags::turbo_stress_instruction_scheduling {
                        self.Schedule::<StressSchedulerQueue>();
                    } else {
                        self.Schedule::<CriticalPathFirstQueue>();
                    }
                    self.sequence_.AddInstruction(instr);
                    return;
                }

                let new_node = self.zone_.New(ScheduleGraphNode::new(&self.zone_, instr));
                let new_node_ptr = unsafe { new_node.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };

                // We should not have branches in the middle of a block.
                assert_ne!(instr_ref.flags_mode(), FlagsMode::kFlags_branch);

                if self.IsFixedRegisterParameter(instr_ref) {
                    if let Some(last_live_in_reg_marker) = &self.last_live_in_reg_marker_ {
                        let last_live_in_reg_marker_ptr = unsafe { last_live_in_reg_marker_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                        unsafe { &mut *last_live_in_reg_marker_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                    }
                    self.last_live_in_reg_marker_ = Some(new_node.clone());
                } else {
                    if let Some(last_live_in_reg_marker) = &self.last_live_in_reg_marker_ {
                        let last_live_in_reg_marker_ptr = unsafe { last_live_in_reg_marker_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                        unsafe { &mut *last_live_in_reg_marker_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                    }

                    // Make sure that instructions are not scheduled before the last
                    // deoptimization or trap point when they depend on it.
                    if let Some(last_deopt_or_trap) = &self.last_deopt_or_trap_ {
                        if self.DependsOnDeoptOrTrap(instr_ref) {
                            let last_deopt_or_trap_ptr = unsafe { last_deopt_or_trap_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *last_deopt_or_trap_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                        }
                    }

                    // Instructions with side effects and memory operations can't be
                    // reordered with respect to each other.
                    if self.HasSideEffect(instr_ref) {
                        if let Some(last_side_effect_instr) = &self.last_side_effect_instr_ {
                            let last_side_effect_instr_ptr = unsafe { last_side_effect_instr_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *last_side_effect_instr_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                        }
                        for load in &self.pending_loads_ {
                            let load_ptr = unsafe { load.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *load_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                        }
                        self.pending_loads_.clear();
                        self.last_side_effect_instr_ = Some(new_node.clone());
                    } else if self.IsLoadOperation(instr_ref) {
                        // Load operations can't be reordered with side effects instructions but
                        // independent loads can be reordered with respect to each other.
                        if let Some(last_side_effect_instr) = &self.last_side_effect_instr_ {
                            let last_side_effect_instr_ptr = unsafe { last_side_effect_instr_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *last_side_effect_instr_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                        }
                        self.pending_loads_.push(new_node.clone());
                    } else if instr_ref.IsDeoptimizeCall() || self.CanTrap(instr_ref) {
                        // Ensure that deopts or traps are not reordered with respect to
                        // side-effect instructions.
                        if let Some(last_side_effect_instr) = &self.last_side_effect_instr_ {
                            let last_side_effect_instr_ptr = unsafe { last_side_effect_instr_.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *last_side_effect_instr_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                        }
                    }

                    // Update last deoptimization or trap point.
                    if instr_ref.IsDeoptimizeCall() || self.CanTrap(instr_ref) {
                        self.last_deopt_or_trap_ = Some(new_node.clone());
                    }

                    // Look for operand dependencies.
                    for i in 0..instr_ref.InputCount() {
                        let input = instr_ref.InputAt(i);
                        if input.IsUnallocated() {
                            let vreg = UnallocatedOperand::cast(input).virtual_register();
                            if let Some(it) = self.operands_map_.get(&vreg) {
                                let it_ptr = unsafe { it.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                                unsafe { &mut *it_ptr }.AddSuccessor(unsafe {&mut *new_node_ptr} );
                            }
                        }
                    }

                    // Record the virtual registers defined by this instruction.
                    for i in 0..instr_ref.OutputCount() {
                        let output = instr_ref.OutputAt(i);
                        if output.IsUnallocated() {
                            self.operands_map_.insert(
                                UnallocatedOperand::cast(output).virtual_register(),
                                new_node.clone(),
                            );
                        } else if output.IsConstant() {
                            self.operands_map_.insert(
                                ConstantOperand::cast(output).virtual_register(),
                                new_node.clone(),
                            );
                        }
                    }
                }

                self.graph_.push(new_node);
            }

            fn Schedule<QueueType: SchedulingQueueTrait>(&mut self) {
                let mut ready_list = QueueType::new(self);

                // Compute total latencies so that we can schedule the critical path first.
                self.ComputeTotalLatencies();

                // Add nodes which don't have dependencies to the ready list.
                for node in &self.graph_ {
                    if !node.HasUnscheduledPredecessor() {
                        ready_list.AddNode(node.clone());
                    }
                }

                // Go through the ready list and schedule the instructions.
                let mut cycle = 0;
                while !ready_list.IsEmpty() {
                    let candidate = ready_list.PopBestCandidate(cycle);

                    if let Some(candidate) = candidate {
                        let instr_ptr = unsafe { candidate.instruction() };
                        self.sequence_.AddInstruction(instr_ptr);

                        for successor in candidate.successors() {
                          let successor_ptr = unsafe { successor.as_ref().unwrap() as *const ScheduleGraphNode as *mut ScheduleGraphNode };
                            unsafe { &mut *successor_ptr }.DropUnscheduledPredecessor();
                            unsafe { &mut *successor_ptr }.set_start_cycle(std::cmp::max(
                                unsafe { &*successor_ptr }.start_cycle(),
                                cycle + candidate.latency(),
                            ));

                            if !unsafe { &*successor_ptr }.HasUnscheduledPredecessor() {
                                ready_list.AddNode(unsafe { &*successor_ptr }.clone());
                            }
                        }
                    }

                    cycle += 1;
                }

                // Reset own state.
                self.graph_.clear();
                self.operands_map_.clear();
                self.pending_loads_.clear();
                self.last_deopt_or_trap_ = None;
                self.last_live_in_reg_marker_ = None;
                self.last_side_effect_instr_ = None;
            }

            pub fn GetInstructionFlags(&self, instr: &Instruction) -> i32 {
                match instr.arch_opcode() {
                    ArchOpcode::kArchNop
                    | ArchOpcode::kArchStackCheckOffset
                    | ArchOpcode::kArchFramePointer
                    | ArchOpcode::kArchParentFramePointer
                    | ArchOpcode::kArchStackSlot
                    | ArchOpcode::kArchComment
                    | ArchOpcode::kArchDeoptimize
                    | ArchOpcode::kArchJmp
                    | ArchOpcode::kArchBinarySearchSwitch
                    | ArchOpcode::kArchRet
                    | ArchOpcode::kArchTableSwitch
                    | ArchOpcode::kArchThrowTerminator => ArchOpcodeFlags::kNoOpcodeFlags as i32,
                    ArchOpcode::kArchTruncateDoubleToI
                    | ArchOpcode::kIeee754Float64Acos
                    | ArchOpcode::kIeee754Float64Acosh
                    | ArchOpcode::kIeee754Float64Asin
                    | ArchOpcode::kIeee754Float64Asinh
                    | ArchOpcode::kIeee754Float64Atan
                    | ArchOpcode::kIeee754Float64Atanh
                    | ArchOpcode::kIeee754Float64Atan2
                    | ArchOpcode::kIeee754Float64Cbrt
                    | ArchOpcode::kIeee754Float64Cos
                    | ArchOpcode::kIeee754Float64Cosh
                    | ArchOpcode::kIeee754Float64Exp
                    | ArchOpcode::kIeee754Float64Expm1
                    | ArchOpcode::kIeee754Float64Log
                    | ArchOpcode::kIeee754Float64Log1p
                    | ArchOpcode::kIeee754Float64Log10
                    | ArchOpcode::kIeee754Float64Log2
                    | ArchOpcode::kIeee754Float64Pow
                    | ArchOpcode::kIeee754Float64Sin
                    | ArchOpcode::kIeee754Float64Sinh
                    | ArchOpcode::kIeee754Float64Tan
                    | ArchOpcode::kIeee754Float64Tanh => ArchOpcodeFlags::kNoOpcodeFlags as i32,
                    ArchOpcode::kArchStackPointerGreaterThan => {
                        // The ArchStackPointerGreaterThan instruction loads the current stack
                        // pointer value and must not be reordered with instructions with side
                        // effects.
                        ArchOpcodeFlags::kIsLoadOperation as i32
                    }
                    ArchOpcode::kArchStackPointer | ArchOpcode::kArchSetStackPointer => {
                        // Instructions that load or set the stack pointer must not be reordered
                        // with instructions with side effects or with each other.
                        ArchOpcodeFlags::kHasSideEffect as i32
                    }
                    ArchOpcode::kArchPrepareCallCFunction
                    | ArchOpcode::kArchPrepareTailCall
                    | ArchOpcode::kArchTailCallCodeObject
                    | ArchOpcode::kArchTailCallAddress
                    | ArchOpcode::kArchTailCallWasm
                    | ArchOpcode::kArchTailCallWasmIndirect
                    | ArchOpcode::kArchAbortCSADcheck => ArchOpcodeFlags::kHasSideEffect as i32,
                    ArchOpcode::kArchDebugBreak => ArchOpcodeFlags::kIsBarrier as i32,
                    ArchOpcode::kArchSaveCallerRegisters | ArchOpcode::kArchRestoreCallerRegisters => {
                        ArchOpcodeFlags::kIsBarrier as i32
                    }
                    ArchOpcode::kArchCallCFunction
                    | ArchOpcode::kArchCallCFunctionWithFrameState
                    | ArchOpcode::kArchCallCodeObject
                    | ArchOpcode::kArchCallJSFunction
                    | ArchOpcode::kArchCallWasmFunction
                    | ArchOpcode::kArchCallWasmFunctionIndirect
                    | ArchOpcode::kArchCallBuiltinPointer => {
                        // Calls can cause GC and GC may relocate objects. If a pure instruction
                        // operates on a tagged pointer that was cast to a word then it may be
                        // incorrect to move the instruction across the call. Hence we mark all
                        // (non-tail-)calls as barriers.
                        ArchOpcodeFlags::kIsBarrier as i32
                    }
                    ArchOpcode::kArchStoreWithWriteBarrier
                    | ArchOpcode::kArchAtomicStoreWithWriteBarrier
                    | ArchOpcode::kArchStoreIndirectWithWriteBarrier => ArchOpcodeFlags::kHasSideEffect as i32,
                    ArchOpcode::kAtomicLoadInt8
                    | ArchOpcode::kAtomicLoadUint8
                    | ArchOpcode::kAtomicLoadInt16
                    | ArchOpcode::kAtomicLoadUint16
                    | ArchOpcode::kAtomicLoadWord32 => ArchOpcodeFlags::kIsLoadOperation as i32,
                    ArchOpcode::kAtomicStoreWord8
                    | ArchOpcode::kAtomicStoreWord16
                    | ArchOpcode::kAtomicStoreWord32 => ArchOpcodeFlags::kHasSideEffect as i32,
                    ArchOpcode::kAtomicExchangeInt8
                    | ArchOpcode::kAtomicExchangeUint8
                    | ArchOpcode::kAtomicExchangeInt16
                    | ArchOpcode::kAtomicExchangeUint16
                    | ArchOpcode::kAtomicExchangeWord32
                    | ArchOpcode::kAtomicCompareExchangeInt8
                    | ArchOpcode::kAtomicCompareExchangeUint8
                    | ArchOpcode::kAtomicCompareExchangeInt16
                    | ArchOpcode::kAtomicCompareExchangeUint16
                    | ArchOpcode::kAtomicCompareExchangeWord32
                    | ArchOpcode::kAtomicAddInt8
                    | ArchOpcode::kAtomicAddUint8
                    | ArchOpcode::kAtomicAddInt16
                    | ArchOpcode::kAtomicAddUint16
                    | ArchOpcode::kAtomicAddWord32
                    | ArchOpcode::kAtomicSubInt8
                    | ArchOpcode::kAtomicSubUint8
                    | ArchOpcode::kAtomicSubInt16
                    | ArchOpcode::kAtomicSubUint16
                    | ArchOpcode::kAtomicSubWord32
                    | ArchOpcode::kAtomicAndInt8
                    | ArchOpcode::kAtomicAndUint8
                    | ArchOpcode::kAtomicAndInt16
                    | ArchOpcode::kAtomicAndUint16
                    | ArchOpcode::kAtomicAndWord32
                    | ArchOpcode::kAtomicOrInt8
                    | ArchOpcode::kAtomicOrUint8
                    | ArchOpcode::kAtomicOrInt16
                    | ArchOpcode::kAtomicOrUint16
                    | ArchOpcode::kAtomicOrWord32
                    | ArchOpcode::kAtomicXorInt8
                    | ArchOpcode::kAtomicXorUint8
                    | ArchOpcode::kAtomicXorInt16
                    | ArchOpcode::kAtomicXorUint16
                    | ArchOpcode::kAtomicXorWord32 => ArchOpcodeFlags::kHasSideEffect as i32,
                    _ => self.GetTargetInstructionFlags(instr),
                }
            }

            fn GetTargetInstructionFlags(&self, _instr: &Instruction) -> i32 {
                ArchOpcodeFlags::kNoOpcodeFlags as i32
            }

            fn IsBarrier(&self, instr: &Instruction) -> bool {
                (self.GetInstructionFlags(instr) & ArchOpcodeFlags::kIsBarrier as i32) != 0
            }

            // Check whether the given instruction has side effects (e.g. function call,
            // memory store).
            fn HasSideEffect(&self, instr: &Instruction) -> bool {
                (self.GetInstructionFlags(instr) & ArchOpcodeFlags::kHasSideEffect as i32) != 0
            }

            // Return true if the instruction is a memory load.
            fn IsLoadOperation(&self, instr: &Instruction) -> bool {
                (self.GetInstructionFlags(instr) & ArchOpcodeFlags::kIsLoadOperation as i32) != 0
            }

            fn CanTrap(&self, instr: &Instruction) -> bool {
                instr.IsTrap()
                    || (instr.HasMemoryAccessMode()
                        && instr.memory_access_mode() != MemoryAccessMode::kMemoryAccessDirect)
            }

            // The scheduler will not move the following instructions before the last
            // deopt/trap check:
            //  * loads (this is conservative)
            //  * instructions with side effect
            //  * other deopts/traps
            // Any other instruction can be moved, apart from those that raise exceptions
            // on specific inputs - these are filtered out by the deopt/trap check.
            fn MayNeedDeoptOrTrapCheck(&self, instr: &Instruction) -> bool {
                (self.GetInstructionFlags(instr) & ArchOpcodeFlags::kMayNeedDeoptOrTrapCheck as i32) != 0
            }

            // Return true if the instruction cannot be moved before the last deopt or
            // trap point we encountered.
            fn DependsOnDeoptOrTrap(&self, instr: &Instruction) -> bool {
                self.MayNeedDeoptOrTrapCheck(instr)
                    || instr.IsDeoptimizeCall()
                    || self.CanTrap(instr)
                    || self.HasSideEffect(instr)
                    || self.IsLoadOperation(instr)
            }

            // Identify nops used as a definition point for live-in registers at
            // function entry.
            fn IsFixedRegisterParameter(&self, instr: &Instruction) -> bool {
                (instr.arch_opcode() == ArchOpcode::kArchNop)
                    && (instr.OutputCount() == 1)
                    && (instr.OutputAt(0).IsUnallocated())
                    && (UnallocatedOperand::cast(instr.OutputAt(0)).HasFixedRegisterPolicy()
                        || UnallocatedOperand::cast(instr.OutputAt(0)).HasFixedFPRegisterPolicy())
            }

            fn ComputeTotalLatencies(&mut self) {
              use crate::base::iterator;
              use crate::base::iterator::Reversed;

              for node in iterator::Reversed(&self.graph_).into_iter() {
                let mut max_latency = 0;

                for successor in node.successors() {
                    assert_ne!(-1, unsafe {&*successor}.total_latency());
                    if unsafe {&*successor}.total_latency() > max_latency {
                        max_latency = unsafe {&*successor}.total_latency();
                    }
                }

                unsafe { node.set_total_latency(max_latency + node.latency()) };
              }
            }

            fn zone(&self) -> &Zone {
                &self.zone_
            }

            fn sequence(&self) -> &InstructionSequence {
                &self.sequence_
            }

            fn random_number_generator(&mut self) -> &mut RandomNumberGenerator {
                self.random_number_generator_.as_mut().unwrap()
            }

            pub fn SchedulerSupported() -> bool {
                true
            }
        }

        pub trait SchedulingQueueTrait {
            fn new(scheduler: &mut InstructionScheduler) -> Self;
            fn AddNode(&mut self, node: Box<ScheduleGraphNode>);
            fn IsEmpty(&self) -> bool;
            fn PopBestCandidate(&mut self, cycle: i32) -> Option<Box<ScheduleGraphNode>>;
        }

        // A scheduling graph node.
        // Represent an instruction and their dependencies.
        #[derive(Debug, Clone)]
        pub struct ScheduleGraphNode {
            instr_: *mut Instruction,
            successors_: ZoneDeque<*mut ScheduleGraphNode>,
            unscheduled_predecessors_count_:

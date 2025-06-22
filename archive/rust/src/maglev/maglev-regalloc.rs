// TODO: Add necessary crate imports based on the C++ headers used.
// For example:
// use std::collections::{HashMap, HashSet};
// use std::sync::{Arc, Mutex};
// use v8_rs::*; // Assuming a v8_rs crate that mirrors V8's API.

// Placeholder for missing V8-related crates.
mod v8_rs {
    pub mod base {
        pub mod bits {
            #[inline]
            pub fn RoundUpToPowerOfTwo32(x: u32) -> u32 {
                let mut v = x;
                v -= 1;
                v |= v >> 1;
                v |= v >> 2;
                v |= v >> 4;
                v |= v >> 8;
                v |= v >> 16;
                v += 1;
                v
            }
        }
    }
    pub mod codegen {
        pub mod register {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct Register(u16);

            impl Register {
                pub const NO_REG: Register = Register(0);

                pub fn from_code(code: usize) -> Self {
                    Register(code as u16)
                }

                pub fn code(&self) -> usize {
                    self.0 as usize
                }

                pub fn is_valid(&self) -> bool {
                    self != &Register::NO_REG
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct DoubleRegister(u16);

            impl DoubleRegister {
                pub const NO_REG: DoubleRegister = DoubleRegister(0);

                pub fn from_code(code: usize) -> Self {
                    DoubleRegister(code as u16)
                }

                pub fn code(&self) -> usize {
                    self.0 as usize
                }

                pub fn is_valid(&self) -> bool {
                    self != &DoubleRegister::NO_REG
                }
            }
        }

        pub mod machine_type {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum MachineRepresentation {
                kTagged,
                kWord32,
                kWord64,
                kFloat64,
                // Add other representations as needed.
            }
        }
    }

    pub mod compiler {
        pub mod backend {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum LocationOperand {
                REGISTER,
                STACK_SLOT
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum AllocatedOperandType {
                REGISTER,
                STACK_SLOT,
                IMMEDIATE,
                CONSTANT
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct AllocatedOperand {
                operand_type: AllocatedOperandType,
                representation: MachineRepresentation,
                index: i32,
            }

            impl AllocatedOperand {
                pub const STACK_SLOT: LocationOperand = LocationOperand::STACK_SLOT;
                pub const REGISTER: LocationOperand = LocationOperand::REGISTER;

                pub fn new(operand_type: AllocatedOperandType, representation: MachineRepresentation, index: i32) -> Self {
                    AllocatedOperand {
                        operand_type,
                        representation,
                        index,
                    }
                }

                pub fn stack_slot(representation: MachineRepresentation, index: i32) -> Self {
                    AllocatedOperand {
                        operand_type: AllocatedOperandType::STACK_SLOT,
                        representation,
                        index,
                    }
                }

                pub fn register(representation: MachineRepresentation, index: i32) -> Self {
                    AllocatedOperand {
                        operand_type: AllocatedOperandType::REGISTER,
                        representation,
                        index,
                    }
                }

                pub fn constant(representation: MachineRepresentation, index: i32) -> Self {
                    AllocatedOperand {
                        operand_type: AllocatedOperandType::CONSTANT,
                        representation,
                        index,
                    }
                }

                pub fn is_register(&self) -> bool {
                    self.operand_type == AllocatedOperandType::REGISTER
                }

                pub fn is_stack_slot(&self) -> bool {
                    self.operand_type == AllocatedOperandType::STACK_SLOT
                }

                pub fn index(&self) -> i32 {
                    self.index
                }

                pub fn representation(&self) -> MachineRepresentation {
                    self.representation
                }

                pub fn cast(source:InstructionOperand) -> Self {
                    // TODO: Implement correctly
                    AllocatedOperand {
                        operand_type: AllocatedOperandType::REGISTER,
                        representation: MachineRepresentation::kTagged,
                        index: 0,
                    }
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum UnallocatedOperandExtendedPolicy {
                FIXED_REGISTER,
                MUST_HAVE_REGISTER,
                SAME_AS_INPUT,
                FIXED_FP_REGISTER,
                NONE,
                MUST_HAVE_SLOT,
                REGISTER_OR_SLOT,
                REGISTER_OR_SLOT_OR_CONSTANT,
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum UnallocatedOperandBasicPolicy {
                FIXED_SLOT,
                ANY,
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct UnallocatedOperand {
                basic_policy: UnallocatedOperandBasicPolicy,
                extended_policy: UnallocatedOperandExtendedPolicy,
                fixed_register_index: usize,
                input_index: usize,
            }

            impl UnallocatedOperand {
                pub const FIXED_SLOT: UnallocatedOperandBasicPolicy = UnallocatedOperandBasicPolicy::FIXED_SLOT;

                pub const FIXED_REGISTER: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::FIXED_REGISTER;
                pub const MUST_HAVE_REGISTER: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::MUST_HAVE_REGISTER;
                pub const SAME_AS_INPUT: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::SAME_AS_INPUT;
                pub const FIXED_FP_REGISTER: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::FIXED_FP_REGISTER;
                pub const NONE: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::NONE;
                pub const MUST_HAVE_SLOT: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::MUST_HAVE_SLOT;
                pub const REGISTER_OR_SLOT: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::REGISTER_OR_SLOT;
                pub const REGISTER_OR_SLOT_OR_CONSTANT: UnallocatedOperandExtendedPolicy = UnallocatedOperandExtendedPolicy::REGISTER_OR_SLOT_OR_CONSTANT;

                pub fn new(basic_policy: UnallocatedOperandBasicPolicy, extended_policy: UnallocatedOperandExtendedPolicy, fixed_register_index: usize, input_index: usize) -> Self {
                    UnallocatedOperand {
                        basic_policy,
                        extended_policy,
                        fixed_register_index,
                        input_index,
                    }
                }

                pub fn basic_policy(&self) -> UnallocatedOperandBasicPolicy {
                    self.basic_policy
                }

                pub fn extended_policy(&self) -> UnallocatedOperandExtendedPolicy {
                    self.extended_policy
                }

                pub fn fixed_register_index(&self) -> usize {
                    self.fixed_register_index
                }

                pub fn input_index(&self) -> usize {
                    self.input_index
                }

                pub fn cast(source: InstructionOperand) -> Self {
                    // TODO: Implement correctly
                    UnallocatedOperand {
                        basic_policy: UnallocatedOperandBasicPolicy::ANY,
                        extended_policy: UnallocatedOperandExtendedPolicy::NONE,
                        fixed_register_index: 0,
                        input_index: 0,
                    }
                }

                pub fn has_same_as_input_policy(&self) -> bool {
                    self.extended_policy == UnallocatedOperandExtendedPolicy::SAME_AS_INPUT
                }
            }

            #[derive(Debug, Clone, PartialEq)]
            pub enum InstructionOperand {
                Allocated(AllocatedOperand),
                Unallocated(UnallocatedOperand),
                Constant(i32),
                Invalid,
            }

            impl InstructionOperand {
                pub fn is_any_register(&self) -> bool {
                    match self {
                        InstructionOperand::Allocated(op) => op.is_register(),
                        _ => false,
                    }
                }

                pub fn is_unallocated(&self) -> bool {
                    match self {
                        InstructionOperand::Unallocated(_) => true,
                        _ => false,
                    }
                }

                pub fn is_constant(&self) -> bool {
                    match self {
                        InstructionOperand::Constant(_) => true,
                        _ => false,
                    }
                }

                pub fn is_invalid(&self) -> bool {
                    match self {
                        InstructionOperand::Invalid => true,
                        _ => false,
                    }
                }

                pub fn cast(source: InstructionOperand) -> Self {
                    // TODO: Implement correctly
                    InstructionOperand::Unallocated(UnallocatedOperand::new(UnallocatedOperandBasicPolicy::ANY, UnallocatedOperandExtendedPolicy::NONE, 0, 0))
                }
            }

            impl From<AllocatedOperand> for InstructionOperand {
                fn from(operand: AllocatedOperand) -> Self {
                    InstructionOperand::Allocated(operand)
                }
            }

            impl From<UnallocatedOperand> for InstructionOperand {
                fn from(operand: UnallocatedOperand) -> Self {
                    InstructionOperand::Unallocated(operand)
                }
            }
        }
    }

    pub mod heap {
        pub mod parked_scope {} // Placeholder.
    }

    pub mod interpreter {
        pub mod register {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct Register(i32);

            impl Register {
                pub fn virtual_accumulator() -> Self {
                    Register(0) // Placeholder. Adjust the actual value.
                }

                pub fn receiver() -> Self {
                    Register(1) // Placeholder. Adjust the actual value.
                }

                pub fn is_parameter(&self) -> bool {
                    self.0 > 0
                }

                pub fn is_receiver(&self) -> bool {
                    self.0 == 1
                }

                pub fn index(&self) -> i32 {
                    self.0
                }
            }
        }
    }
    pub mod zone {
        pub mod zone_containers {
            pub type ZoneVec<T> = Vec<T>;
            pub type ZoneUnorderedSet<T> = std::collections::HashSet<T>;
        }
    }
}

// Placeholder for V8 flags.
mod v8_flags {
    pub static trace_maglev_regalloc: bool = false;
    pub static maglev_reuse_stack_slots: bool = true;
}

mod base {
    pub mod bits {
        pub fn countr_zero32(x: u32) -> u32 {
            x.trailing_zeros()
        }
    }
}

use v8_rs::base::bits;
use v8_rs::codegen::machine_type::MachineRepresentation;
use v8_rs::codegen::register::{DoubleRegister, Register};
use v8_rs::compiler::backend::{AllocatedOperand, InstructionOperand, UnallocatedOperand};

/// Flags representing the register state of a node.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RegisterStateFlags {
    is_initialized: bool,
    is_merge: bool,
}

const INITIALIZED_NODE: RegisterStateFlags = RegisterStateFlags {
    is_initialized: true,
    is_merge: false,
};
const INITIALIZED_MERGE: RegisterStateFlags = RegisterStateFlags {
    is_initialized: true,
    is_merge: true,
};

type BlockReverseIterator = std::vec::IntoIter<BasicBlock>; // Assuming BasicBlock is defined elsewhere

/// Determines if a target block is a fallthrough of a control node.
///
/// A target is considered a fallthrough if its ID immediately follows the
/// control node's ID. This is a common pattern in basic block arrangements.
fn is_target_of_node_fallthrough(node: &ControlNode, target: &BasicBlock) -> bool {
    node.id() + 1 == target.first_id()
}

/// Finds the nearest post-dominating hole for a given control node.
///
/// This function navigates the control flow graph to locate the next
/// control node that represents a "hole" in the post-dominance tree.
fn nearest_post_dominating_hole(node: &ControlNode) -> &ControlNode {
    // Conditional control nodes don't cause holes themselves. So, the nearest
    // post-dominating hole is the conditional control node's next post-dominating
    // hole.
    if node.is::<BranchControlNode>() {
        return node.next_post_dominating_hole();
    }

    // If the node is a Jump, it may be a hole, but only if it is not a
    // fallthrough (jump to the immediately next block). Otherwise, it will point
    // to the nearest post-dominating hole in its own "next" field.
    if node.is::<Jump>() || node.is::<CheckpointedJump>() {
        let target: &BasicBlock;
        if let Some(jmp) = node.try_cast::<Jump>() {
            target = jmp.target();
        } else {
            target = node.cast::<CheckpointedJump>().target();
        }
        if is_target_of_node_fallthrough(node, target) {
            return node.next_post_dominating_hole();
        }
    }

    // If the node is a Switch, it can only have a hole if there is no
    // fallthrough.
    if let Some(_switch) = node.try_cast::<Switch>() {
        if _switch.has_fallthrough() {
            return _switch.next_post_dominating_hole();
        }
    }

    node
}

/// Finds the highest post-dominating hole between two control nodes.
///
/// This function determines the merge-point of two branches or the highest
/// reachable control-node of the longest branch after the last node of the
/// shortest branch.
fn highest_post_dominating_hole<'a>(first: &'a ControlNode, second: &'a ControlNode) -> &'a ControlNode {
    let mut first = first;
    let mut second = second;

    // As long as there's no merge-point.
    while first != second {
        // Walk the highest branch to find where it goes.
        if first.id() > second.id() {
            std::mem::swap(&mut first, &mut second);
        }

        // If the first branch terminates or jumps back, we've found highest
        // reachable control-node of the longest branch (the second control
        // node).
        if first.is::<TerminalControlNode>() || first.is::<JumpLoop>() {
            return second;
        }

        // Continue one step along the highest branch. This may cross over the
        // lowest branch in case it returns or loops. If labelled blocks are
        // involved such swapping of which branch is the highest branch can
        // occur multiple times until a return/jumploop/merge is discovered.
        first = first.next_post_dominating_hole();
    }

    // Once the branches merged, we've found the gap-chain that's relevant
    // for the control node.
    first
}

/// Finds the highest post-dominating hole from a vector of control nodes.
///
/// This function sorts the control nodes by ID and iteratively finds the
/// highest post-dominating hole.
fn highest_post_dominating_hole_vec<'a>(holes: &mut Vec<&'a ControlNode>) -> &'a ControlNode {
    // Sort them from highest to shortest.
    holes.sort_by(|a, b| b.id().cmp(&a.id()));
    assert!(holes.len() > 1);

    // Find the highest post dominating hole.
    let mut post_dominating_hole = holes.pop().unwrap();
    while !holes.is_empty() {
        let next_hole = holes.pop().unwrap();
        post_dominating_hole = highest_post_dominating_hole(post_dominating_hole, next_hole);
    }
    post_dominating_hole
}

/// Checks if a value node is live at a specific target basic block.
///
/// This function determines if a value node is still in use at the beginning
/// of a target basic block, considering loop conditions and resumable loops.
fn is_live_at_target(node: &ValueNode, source: &ControlNode, target: &BasicBlock) -> bool {
    assert!(node.has_no_more_uses() == false);

    // If we're looping, a value can only be live if it was live before the loop.
    if target.control_node().id() <= source.id() {
        // Gap moves may already be inserted in the target, so skip over those.
        return node.id() < target.first_non_gap_move_id();
    }

    // Drop all values on resumable loop headers.
    if target.is_loop() && target.state().is_resumable_loop() {
        return false;
    }

    // TODO(verwaest): This should be true but isn't because we don't yet
    // eliminate dead code.
    // assert!(node.next_use > source.id());
    // TODO(verwaest): Since we don't support deopt yet we can only deal with
    // direct branches. Add support for holes.
    node.live_range().end >= target.first_id()
}

/// Checks if a node is a dead node that should be skipped during allocation.
///
/// A node is considered dead if it's a ValueNode with no more uses and isn't
/// required when unused.
fn is_dead_node_to_skip(node: &Node) -> bool {
    if !node.is::<ValueNode>() {
        return false;
    }
    let value = node.cast::<ValueNode>();
    value.has_no_more_uses() && !value.properties().is_required_when_unused()
}

/// Manages register allocation in a straightforward manner.
///
/// This struct performs register allocation for a given Maglev compilation unit,
/// considering liveness, control flow, and register constraints.
pub struct StraightForwardRegisterAllocator {
    compilation_info: Box<MaglevCompilationInfo>, // Assuming MaglevCompilationInfo is defined elsewhere
    graph: Box<Graph>,                         // Assuming Graph is defined elsewhere
    patches: Vec<Patch>,
    block_it_: std::vec::IntoIter<BasicBlock>,
    node_it_: std::slice::Iter<'static, Node>,
    general_registers_: RegisterFrameState<Register>,
    double_registers_: RegisterFrameState<DoubleRegister>,
    tagged_: SpillSlots,
    untagged_: SpillSlots,
    current_node_: *mut Node, //raw pointer
    printing_visitor_: Option<MaglevPrintingVisitor> // Assuming MaglevPrintingVisitor is defined elsewhere
}

struct Patch {
    diff: usize,
    new_node: *mut Node, //raw pointer
}

impl StraightForwardRegisterAllocator {
    /// Applies patches to a basic block by inserting new nodes.
    fn apply_patches(&mut self, block: &mut BasicBlock) {
        // TODO(verwaest): Perhaps don't actually merge these in but let the code
        // generator pick up the gap moves from a separate list.
        let diff = self.patches.len();
        if diff == 0 {
            return;
        }
        block.nodes().resize(block.nodes().len() + diff, unsafe { std::mem::zeroed() });
        let mut patches_it = self.patches.iter().rev();
        let mut node_it = block.nodes().iter_mut().rev().skip(diff);

        for _ in 0..block.nodes().len()- diff {
            unsafe {
                let node = node_it.next().unwrap();
                let diff = patches_it.len();
                *node = std::ptr::read(node);
                let mut patches_it2 = self.patches.iter().rev();
                for _ in 0..diff {
                   let patch = patches_it2.next().unwrap();
                   std::ptr::write(node, patch.new_node as Node);
                }
            }
           
        }

    }

    /// Creates a new register allocator for the given compilation info and graph.
    pub fn new(compilation_info: Box<MaglevCompilationInfo>, graph: Box<Graph>) -> Self {
        let patches = Vec::new();
        let block_it_ = graph.blocks().clone().into_iter();
        let node_it_: std::slice::Iter<'static, Node> = [].iter();
        let general_registers_: RegisterFrameState<Register> = RegisterFrameState::new();
        let double_registers_: RegisterFrameState<DoubleRegister> = RegisterFrameState::new();
        let tagged_: SpillSlots = SpillSlots::new();
        let untagged_: SpillSlots = SpillSlots::new();
        let current_node_: *mut Node = std::ptr::null_mut();
        let printing_visitor_: Option<MaglevPrintingVisitor> = None;

        let mut allocator = Self {
            compilation_info,
            graph,
            patches,
            block_it_,
            node_it_,
            general_registers_,
            double_registers_,
            tagged_,
            untagged_,
            current_node_,
            printing_visitor_,
        };

        allocator.compute_post_dominating_holes();
        allocator.allocate_registers();
        let tagged_stack_slots = allocator.tagged_.top;
        let untagged_stack_slots = allocator.untagged_.top;
        if allocator.graph.is_osr() {
            // Fix our stack frame to be compatible with the source stack frame of this
            // OSR transition:
            // 1) Ensure the section with tagged slots is big enough to receive all
            //    live OSR-in values.
            for val in allocator.graph.osr_values() {
                if val.result().operand().is_allocated()
                    && val.stack_slot() >= tagged_stack_slots
                {
                    //tagged_stack_slots = val.stack_slot() + 1; // TODO: Check +1
                }
            }
            // 2) Ensure we never have to shrink stack frames when OSR'ing into Maglev.
            //    We don't grow tagged slots or they might end up being uninitialized.
            let source_frame_size =
                allocator.graph.min_maglev_stackslots_for_unoptimized_frame_size();
            let target_frame_size = tagged_stack_slots + untagged_stack_slots;
            if source_frame_size > target_frame_size {
                //untagged_stack_slots += source_frame_size - target_frame_size;
            }
        }
        // Due to alignment constraints, we add one untagged slot if
        // stack_slots + fixed_slot_count is odd.
        static_assertions::const_assert!(STANDARD_FRAME_CONSTANTS_K_FIXED_SLOT_COUNT % 2 == 1);
        if (tagged_stack_slots + untagged_stack_slots) % 2 == 0 {
            //untagged_stack_slots += 1;
        }
        //allocator.graph.set_tagged_stack_slots(tagged_stack_slots);
        //allocator.graph.set_untagged_stack_slots(untagged_stack_slots);
        allocator
    }

    /// Computes the post-dominating holes for all forward control nodes.
    fn compute_post_dominating_holes(&mut self) {
        // For all blocks, find the list of jumps that jump over code unreachable from
        // the block. Such a list of jumps terminates in return or jumploop.
        for block in self.graph.blocks().iter().rev() {
            let control = block.control_node();
            if let Some(unconditional_control) = control.try_cast::<UnconditionalControlNode>() {
                // If the current control node is a jump, prepend it to the list of jumps
                // at the target.
                control.set_next_post_dominating_hole(nearest_post_dominating_hole(
                    unconditional_control.target().control_node(),
                ));
            } else if let Some(branch) = control.try_cast::<BranchControlNode>() {
                let first = nearest_post_dominating_hole(branch.if_true().control_node());
                let second = nearest_post_dominating_hole(branch.if_false().control_node());
                control.set_next_post_dominating_hole(highest_post_dominating_hole(first, second));
            } else if let Some(switch_node) = control.try_cast::<Switch>() {
                let num_targets =
                    switch_node.size() + (if switch_node.has_fallthrough() { 1 } else { 0 });
                if num_targets == 1 {
                    // If we have a single target, the next post dominating hole
                    // is the same one as the target.
                    assert!(!switch_node.has_fallthrough());
                    control.set_next_post_dominating_hole(nearest_post_dominating_hole(
                        switch_node.targets()[0].block_ptr().control_node(),
                    ));
                    continue;
                }
                // Calculate the post dominating hole for each target.
                let mut holes: Vec<&ControlNode> = Vec::with_capacity(num_targets);
                for i in 0..switch_node.size() {
                    holes.push(nearest_post_dominating_hole(
                        switch_node.targets()[i].block_ptr().control_node(),
                    ));
                }
                if switch_node.has_fallthrough() {
                    holes.push(nearest_post_dominating_hole(switch_node.fallthrough().control_node()));
                }
                control.set_next_post_dominating_hole(highest_post_dominating_hole_vec(&mut holes));
            }
        }
    }

    /// Prints the currently live registers for debugging purposes.
    fn print_live_regs(&self) {
        let mut first = true;
        let mut print = |reg: Register, node: &ValueNode| {
            if first {
                first = false;
            } else {
                //self.printing_visitor_.as_mut().unwrap().os() << ", ";
            }
           // self.printing_visitor_.as_mut().unwrap().os() << reg << "=v" << node.id();
        };
        self.general_registers_.for_each_used_register(print);

        let mut print_double = |reg: DoubleRegister, node: &ValueNode| {
            if first {
                first = false;
            } else {
                //self.printing_visitor_.as_mut().unwrap().os() << ", ";
            }
            //self.printing_visitor_.as_mut().unwrap().os() << reg << "=v" << node.id();
        };

        self.double_registers_.for_each_used_register(print_double);
    }

    /// Allocates registers for all nodes in the graph.
    fn allocate_registers(&mut self) {
        if v8_flags::trace_maglev_regalloc {
           // self.printing_visitor_ = Some(MaglevPrintingVisitor::new(
           //     self.compilation_info.graph_labeller(),
           //     std::io::stdout(),
           // ));
           // self.printing_visitor_.as_mut().unwrap().pre_process_graph(&self.graph);
        }

        for (ref_, constant) in self.graph.constants() {
            constant.set_constant_location();
            std::mem::drop(ref_);
        }
        for (index, constant) in self.graph.root() {
            constant.set_constant_location();
            std::mem::drop(index);
        }
        for (value, constant) in self.graph.smi() {
            constant.set_constant_location();
            std::mem::drop(value);
        }
        for (value, constant) in self.graph.tagged_index() {
            constant.set_constant_location();
            std::mem::drop(value);
        }
        for (value, constant) in self.graph.int32() {
            constant.set_constant_location();
            std::mem::drop(value);
        }
        for (value, constant) in self.graph.uint32() {
            constant.set_constant_location();
            std::mem::drop(value);
        }
        for (value, constant) in self.graph.float64() {
            constant.set_constant_location();
            std::mem::drop(value);
        }
        for (address, constant) in self.graph.external_references() {
            constant.set_constant_location();
            std::mem::drop(address);
        }
        for (ref_, constant) in self.graph.trusted_constants() {
            constant.set_constant_location();
            std::mem::drop(ref_);
        }

        for block in self.graph.blocks() {
            //self.block_it_ = self.graph.blocks().clone().into_iter();

            // Restore mergepoint state.
            if block.has_state() {
                if block.state().is_exception_handler() || block.state().is_unreachable_by_forward_edge() {
                    // Exceptions and loops only reachable from a JumpLoop (i.e., resumable
                    // loops with no fall-through edge) start with a blank state of register
                    // values.
                    self.clear_register_values();
                } else {
                    self.initialize_register_values(&block.state().register_state());
                }
            } else if block.is_edge_split_block() {
                self.initialize_register_values(block.edge_split_block_register_state());
            }

            if v8_flags::trace_maglev_regalloc {
                // TODO: Restore
                //self.printing_visitor_.as_mut().unwrap().pre_process_basic_block(block);
                //self.printing_visitor_.as_mut().unwrap().os() << "live regs: ";
                self.print_live_regs();

                let control = nearest_post_dominating_hole(block.control_node());
                if !control.is::<JumpLoop>() {
                    //self.printing_visitor_.as_mut().unwrap().os() << "\n[holes:";
                    loop {
                        if control.is::<JumpLoop>() {
                            //self.printing_visitor_.as_mut().unwrap().os() << " " << control.id() << "↰";
                            break;
                        } else if control.is::<UnconditionalControlNode>() {
                            let target = control.cast::<UnconditionalControlNode>().target();
                            //self.printing_visitor_.as_mut().unwrap().os()
                            //    << " " << control.id() << "-" << target.first_id();
                            //control = control.next_post_dominating_hole();
                            assert!(!control.next_post_dominating_hole().is_null());
                            continue;
                        } else if control.is::<Switch>() {
                            let _switch = control.cast::<Switch>();
                            assert!(!_switch.has_fallthrough());
                            assert!(switch.size() >= 1);
                            let first_target = _switch.targets()[0].block_ptr();
                            //self.printing_visitor_.as_mut().unwrap().os()
                            //    << " " << control.id() << "-" << first_target.first_id();
                            //control = control.next_post_dominating_hole();
                            assert!(!control.next_post_dominating_hole().is_null());
                            continue;
                        } else if control.is::<Return>() {
                            //self.printing_visitor_.as_mut().unwrap().os() << " " << control.id() << ".";
                            break;
                        } else if control.is::<Deopt>() || control.is::<Abort>() {
                            //self.printing_visitor_.as_mut().unwrap().os() << " " << control.id() << "✖️";
                            break;
                        }
                        panic!("UNREACHABLE");
                    }
                   // self.printing_visitor_.as_mut().unwrap().os() << "]";
                }
                //self.printing_visitor_.as_mut().unwrap().os() << std::endl;
            }

            // Activate phis.
            if block.has_phi() {
                let phis = block.phis();
                // Firstly, make the phi live, and try to assign it to an input
                // location.
                let mut phi_it = phis.begin();
                while phi_it != phis.end() {
                    let phi = *phi_it;
                    if !phi.has_valid_live_range() {
                        // We might still have left over dead Phis, due to phis being kept
                        // alive by deopts that the representation analysis dropped. Clear
                        // them out now.
                       // phi_it = phis.remove_at(phi_it); // TODO: Fix RemoveAt
                    } else {
                        assert!(phi.has_valid_live_range());
                        phi.set_no_spill();
                        self.try_allocate_to_input(phi);
                        //phi_it = phis.begin();// TODO: Fix ++phi_it;
                    }
                    //phi_it = phis.begin();
                }
                if block.is_exception_handler_block() {
                    // If we are in exception handler block, then we find the ExceptionPhi
                    // (the first one by default) that is marked with the
                    // virtual_accumulator and force kReturnRegister0. This corresponds to
                    // the exception message object.
                   // for phi in phis {
                   //     assert_eq!(phi.input_count(), 0);
                   //     assert!(phi.is_exception_phi());
                   //     if phi.owner() == interpreter::Register::virtual_accumulator() {
                   //         if !phi.has_no_more_uses() {
                   //             phi.result().set_allocated(self.force_allocate(kReturnRegister0, phi
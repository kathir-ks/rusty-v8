// Converted from V8 C++ source files:
// Header: instruction-selection-phase.h
// Implementation: instruction-selection-phase.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instruction_selection_phase {
use std::cell::RefCell;
use std::rc::Rc;
use crate::compiler::turboshaft::phase::V8_EXPORT_PRIVATE;
use crate::base;
use crate::compiler::turboshaft::operations::BranchHint;
use crate::execution::isolate::Isolate;
use crate::compiler::backend::instruction_selector::InstructionSelector;
use crate::compiler::pipeline::PipelineData;
use crate::compiler::turboshaft::sidetable::FixedBlockSidetable;
use crate::diagnostics::code_tracer::CodeTracer;
use crate::utils::sparse_bit_vector::SparseBitVector;
use crate::objects::code::Code;
use crate::compiler::turboshaft::graph::Graph;
use crate::compiler::turboshaft::block::Block;
use crate::compiler::turboshaft::block::BlockIndex;
use crate::compiler::call_descriptor::CallDescriptor;
use crate::codegen::linkage::Linkage;
use crate::compiler::js_heap_broker::JSHeapBroker;
use crate::compiler::turbofan_graph_visualizer::TurboJsonFile;
use crate::compiler::turboshaft::phase::AllowHandleDereference;
use crate::compiler::turboshaft::wasm_dead_code_elimination_phase::UnparkedScopeIfNeeded;
use crate::compiler::backend::instruction_selector_impl::InstructionSequenceAsJSON;
use crate::compiler::turboshaft::operations::BranchOp;
use crate::codegen::bailout_reason::BailoutReason;
use std::optional::Option;
use crate::handles::maybe_indirect_handle::MaybeIndirectHandle;
use crate::compiler::frame::Frame;
use crate::zone::Zone;
use std::vec::Vec;
use std::cmp;

pub struct TurboshaftSpecialRPONumberer {
    graph_: *const Graph,
    block_data_: FixedBlockSidetable<BlockData>,
    loops_: ZoneVector<LoopInfo>,
}

impl TurboshaftSpecialRPONumberer {
    pub const kBlockOnStack: i32 = -2;
    pub const kBlockVisited1: i32 = -3;
    pub const kBlockVisited2: i32 = -4;
    pub const kBlockUnvisited: i32 = -1;

    pub fn new(graph: &Graph, zone: &Zone) -> TurboshaftSpecialRPONumberer {
        TurboshaftSpecialRPONumberer {
            graph_: graph,
            block_data_: FixedBlockSidetable::new(graph.block_count(), zone),
            loops_: ZoneVector::new(zone),
        }
    }

    pub fn compute_special_rpo(&mut self) -> ZoneVector<u32> {
        let mut stack: ZoneVector<SpecialRPOStackFrame> = ZoneVector::new(self.loops_.zone());
        let mut backedges: ZoneVector<Backedge> = ZoneVector::new(self.loops_.zone());
        stack.reserve(64);
        backedges.reserve(32);
        let mut num_loops: usize = 0;

        let graph = unsafe { &*self.graph_ };

        let push = |stack: &mut ZoneVector<SpecialRPOStackFrame>, block: &Block, block_data: &mut FixedBlockSidetable<BlockData>| {
            let succs = Self::successor_blocks(block, graph);
            stack.push(SpecialRPOStackFrame::new(block, 0, succs));
            block_data[block.index()].rpo_number = Self::kBlockOnStack;
        };

        let entry: &Block = &graph.StartBlock();
        let mut order: *const Block = std::ptr::null();

        push(&mut stack, &graph.StartBlock(), &mut self.block_data_);

        while !stack.is_empty() {
            let mut frame: &mut SpecialRPOStackFrame = stack.last_mut().unwrap();

            if frame.index < frame.successors.len() {
                let succ: &Block = &frame.successors[frame.index];
                frame.index += 1;

                if self.rpo_number(succ) == Self::kBlockVisited1 {
                    continue;
                }
                if self.rpo_number(succ) == Self::kBlockOnStack {
                    backedges.push(Backedge {
                        block: frame.block,
                        index: frame.index - 1,
                    });
                    if !self.has_loop_number(succ) {
                        self.set_loop_number(succ, num_loops);
                        num_loops += 1;
                    }
                } else {
                    if self.rpo_number(succ) == Self::kBlockUnvisited {
                        push(&mut stack, succ, &mut self.block_data_);
                    }
                }
            } else {
                unsafe {
                  order = self.push_front(order, frame.block);
                }
                self.set_rpo_number(frame.block, Self::kBlockVisited1);
                stack.pop();
            }
        }

        if num_loops == 0 {
            return self.compute_block_permutation(entry);
        }

        self.compute_loop_info(num_loops, &mut backedges);

        let mut loop_info: *mut LoopInfo = std::ptr::null_mut();
        order = std::ptr::null();

        push(&mut stack, &graph.StartBlock(), &mut self.block_data_);

        while !stack.is_empty() {
            let mut frame: &mut SpecialRPOStackFrame = stack.last_mut().unwrap();
            let block: &Block = frame.block;
            let mut succ: *const Block = std::ptr::null();

            if frame.index < frame.successors.len() {
                unsafe {
                  succ = &frame.successors[frame.index] as *const Block;
                }
                frame.index += 1;
            } else if self.has_loop_number(block) {
                if self.rpo_number(block) == Self::kBlockOnStack {
                    let loop_num = self.loop_number(block);
                    unsafe {
                      if !(*loop_info).header.is_null() {
                         if (*loop_info).header != block {
                          println!("loop info header not equal to block");
                        }
                      }
                      (*loop_info).start = self.push_front(order, block);
                      order = (*loop_info).end;
                    }
                    self.set_rpo_number(block, Self::kBlockVisited2);
                    
                    unsafe {
                      loop_info = (*loop_info).prev;
                    }
                }

                let outgoing_index: usize = frame.index - frame.successors.len();
                let info: &LoopInfo = &self.loops_[self.loop_number(block)];
                unsafe {
                  if loop_info != info as *const LoopInfo as *mut LoopInfo {
                    println!("loop info does not match info");
                  }
                }

                if block != entry && outgoing_index < info.outgoing.len() {
                    unsafe {
                      succ = &info.outgoing[outgoing_index] as *const Block;
                    }
                    frame.index += 1;
                }
            }

            if succ != std::ptr::null() {
                unsafe {
                  if self.rpo_number(&*succ) == Self::kBlockOnStack {
                      continue;
                  }
                  if self.rpo_number(&*succ) == Self::kBlockVisited2 {
                      continue;
                  }
                  if Self::kBlockVisited1 != self.rpo_number(&*succ) {
                    println!("rpo number not equal");
                  }
                  if !(*loop_info).members.is_null() && !(*(*loop_info).members).contains((&*succ).index().id() as usize) {
                      self.add_outgoing(self.loops_.zone(), succ);
                  } else {
                      push(&mut stack, &*succ, &mut self.block_data_);
                      if self.has_loop_number(&*succ) {
                        let next_loop_num = self.loop_number(&*succ);
                        if next_loop_num >= num_loops {
                          println!("loop number out of bounds");
                        }
                        let next: &mut LoopInfo = &mut self.loops_[next_loop_num];
                        (*next).end = order;
                        (*next).prev = loop_info;
                        loop_info = next as *mut LoopInfo;
                      }
                  }
                }
            } else {
                if self.has_loop_number(block) {
                    let info: &LoopInfo = &self.loops_[self.loop_number(block)];
                    let mut b: *const Block = info.start;
                    unsafe {
                      loop {
                        if self.block_data_[(*b).index()].rpo_next == info.end {
                            self.push_front(order, b);
                            (*info).end = order;
                            break;
                        }
                        b = self.block_data_[(*b).index()].rpo_next;
                      }
                    }
                    order = info.start;
                } else {
                    unsafe {
                      order = self.push_front(order, block);
                    }
                    self.set_rpo_number(block, Self::kBlockVisited2);
                }
                stack.pop();
            }
        }

        return self.compute_block_permutation(entry);
    }

    fn compute_loop_info(&mut self, num_loops: usize, backedges: &mut ZoneVector<Backedge>) {
        let mut stack: ZoneVector<*const Block> = ZoneVector::new(self.loops_.zone());

        self.loops_.resize(num_loops, LoopInfo::default());

        for backedge in backedges.iter() {
            unsafe {
              let header: &Block = &Self::successor_blocks(&*backedge.block, &*self.graph_)[backedge.index];

              if !header.IsLoop() {
                println!("is not a loop");
              }

              let loop_num: usize = self.loop_number(header);

              if !self.loops_[loop_num].header.is_null() {
                println!("loop num header is not null");
              }

              self.loops_[loop_num].header = header;
              self.loops_[loop_num].members = self.loops_.zone().New(SparseBitVector::new());

              if backedge.block != header {
                  if !(*self.loops_[loop_num].members).contains((&*backedge.block).index().id() as usize) {
                    (*self.loops_[loop_num].members).add((&*backedge.block).index().id() as usize);
                    stack.push(backedge.block);
                  }
              }

              while !stack.is_empty() {
                  let block: *const Block = stack.last().unwrap();
                  stack.pop();

                  for pred in (&*block).PredecessorsIterable() {
                      if pred != header {
                          unsafe {
                            if !(*self.loops_[loop_num].members).contains((&*pred).index().id() as usize) {
                                (*self.loops_[loop_num].members).add((&*pred).index().id() as usize);
                                stack.push(pred);
                            }
                          }
                      }
                  }
              }
            }
        }
    }

    fn compute_block_permutation(&self, entry: &Block) -> ZoneVector<u32> {
        let graph = unsafe { &*self.graph_ };
        let mut result: ZoneVector<u32> = ZoneVector::new(self.loops_.zone());
        result.resize(graph.block_count(), 0);
        let mut i: usize = 0;
        let mut b: *const Block = entry;
        unsafe {
          while !b.is_null() {
            result[i] = (&*b).index().id();
            i += 1;
            b = self.block_data_[(&*b).index()].rpo_next;
          }
        }

        if i != graph.block_count() {
          println!("i not equal to graph block count");
        }

        return result;
    }

    fn rpo_number(&self, block: &Block) -> i32 {
        self.block_data_[block.index()].rpo_number
    }

    fn set_rpo_number(&mut self, block: &Block, rpo_number: i32) {
        self.block_data_[block.index()].rpo_number = rpo_number;
    }

    fn has_loop_number(&self, block: &Block) -> bool {
        self.block_data_[block.index()].loop_number != BlockData::kNoLoopNumber
    }

    fn loop_number(&self, block: &Block) -> usize {
        if !self.has_loop_number(block) {
          println!("does not have loop number");
        }
        self.block_data_[block.index()].loop_number
    }

    fn set_loop_number(&mut self, block: &Block, loop_number: usize) {
        self.block_data_[block.index()].loop_number = loop_number;
    }

    unsafe fn push_front(&mut self, head: *const Block, block: &Block) -> *const Block {
        self.block_data_[block.index()].rpo_next = head;
        block
    }

    fn zone(&self) -> &Zone {
        self.loops_.zone()
    }

    fn successor_blocks(block: &Block, graph: &Graph) -> Vec<Block> {
        block.successors().iter().map(|&block_ptr| {
            unsafe { (*block_ptr).clone() }
        }).collect()
    }

    fn add_outgoing(&mut self, zone: &Zone, block: *const Block) {
      unsafe {
        let loop_num = self.loop_number(&*block);
        self.loops_[loop_num].outgoing.push(block);
      }
    }
}

#[derive(Clone)]
pub struct SpecialRPOStackFrame {
    pub block: *const Block,
    pub index: usize,
    pub successors: Vec<Block>,
}

impl SpecialRPOStackFrame {
    pub fn new(block: *const Block, index: usize, successors: Vec<Block>) -> SpecialRPOStackFrame {
        SpecialRPOStackFrame {
            block,
            index,
            successors,
        }
    }
}

pub type Backedge = std::pair<*const Block, usize>;

#[derive(Default)]
pub struct LoopInfo {
    pub header: *const Block,
    pub outgoing: Vec<*const Block>,
    pub members: *mut SparseBitVector,
    pub prev: *mut LoopInfo,
    pub end: *const Block,
    pub start: *const Block,
}

impl LoopInfo {
    pub fn add_outgoing(&mut self, zone: &Zone, block: *const Block) {
        self.outgoing.push(block);
    }
}

#[derive(Clone, Copy)]
pub struct BlockData {
    pub rpo_number: i32,
    pub loop_number: usize,
    pub rpo_next: *const Block,
}

impl BlockData {
    pub const kNoLoopNumber: usize = usize::max_value();
}

impl Default for BlockData {
    fn default() -> Self {
        BlockData {
            rpo_number: TurboshaftSpecialRPONumberer::kBlockUnvisited,
            loop_number: BlockData::kNoLoopNumber,
            rpo_next: std::ptr::null(),
        }
    }
}

pub fn propagate_deferred(graph: &mut Graph) {
    graph.StartBlock().set_custom_data(
        0,
        Block::CustomDataKind::kDeferredInSchedule,
    );
    for block in graph.blocks_mut() {
        let predecessor: *const Block = block.LastPredecessor();
        if predecessor.is_null() {
            continue;
        } else if block.IsLoop() {
            unsafe {
              let mut predecessor = (&*predecessor).NeighboringPredecessor();

              if predecessor.is_null() {
                println!("predecessor is null");
              }

              if !(&*predecessor).NeighboringPredecessor().is_null() {
                println!("neighboring predecessor is not null");
              }

              block.set_custom_data(
                  (&*predecessor).get_custom_data(Block::CustomDataKind::kDeferredInSchedule),
                  Block::CustomDataKind::kDeferredInSchedule,
              );
            }
        } else {
            unsafe {
              if (&*predecessor).NeighboringPredecessor().is_null() {
                  let is_deferred: bool =
                      (&*predecessor).get_custom_data(Block::CustomDataKind::kDeferredInSchedule)
                          || is_unlikely_successor(predecessor, block, graph);
                  block.set_custom_data(
                      is_deferred,
                      Block::CustomDataKind::kDeferredInSchedule,
                  );
              } else {
                  block.set_custom_data(
                      true,
                      Block::CustomDataKind::kDeferredInSchedule,
                  );
                  let mut predecessor_iter = predecessor;
                  loop {
                      if predecessor_iter.is_null() {
                          break;
                      }
                      if !(&*predecessor_iter).get_custom_data(Block::CustomDataKind::kDeferredInSchedule) {
                          block.set_custom_data(
                              false,
                              Block::CustomDataKind::kDeferredInSchedule,
                          );
                          break;
                      }
                      predecessor_iter = (&*predecessor_iter).NeighboringPredecessor();
                  }
              }
            }
        }
    }
}

fn is_unlikely_successor(predecessor: *const Block, block: &Block, graph: &Graph) -> bool {
    unsafe {
        if let Some(branch) = (&*predecessor).operation().and_then(|op| op.TryCast::<BranchOp>()) {
            if branch.if_true == block && branch.hint == BranchHint::kFalse {
                return true;
            }
            if branch.if_false == block && branch.hint == BranchHint::kTrue {
                return true;
            }
        }
    }
    false
}

pub struct ProfileApplicationPhase {}

impl ProfileApplicationPhase {
    pub const kPhaseName: &'static str = "ProfileApplication";

    pub fn run(&self, data: &mut PipelineData, temp_zone: &Zone, profile: *const ProfileDataFromFile) {
        let graph = &mut data.graph();
        for op in graph.AllOperations_mut() {
            if let Some(branch) = op.TryCastMut::<BranchOp>() {
                let true_block_id = branch.if_true.index().id();
                let false_block_id = branch.if_false.index().id();
                unsafe {
                  let hint: BranchHint = (&*profile).GetHint(true_block_id, false_block_id);
                  if hint != BranchHint::kNone {
                      branch.hint = hint;
                  }
                }
            }
        }
    }
}

pub struct SpecialRPOSchedulingPhase {}

impl SpecialRPOSchedulingPhase {
    pub const kPhaseName: &'static str = "SpecialRPOScheduling";

    pub fn run(&self, data: &mut PipelineData, temp_zone: &Zone) {
        let graph: &mut Graph = &mut data.graph();

        let mut numberer: TurboshaftSpecialRPONumberer =
            TurboshaftSpecialRPONumberer::new(graph, temp_zone);
        if !data.graph_has_special_rpo() {
            let schedule: ZoneVector<u32> = numberer.compute_special_rpo();
            graph.ReorderBlocks(base::VectorOf(schedule));
            data.set_graph_has_special_rpo();
        }

        propagate_deferred(graph);
    }
}

pub struct InstructionSelectionPhase {}

impl InstructionSelectionPhase {
    pub const kPhaseName: &'static str = "InstructionSelection";
    pub const kOutputIsTraceableGraph: bool = false;

    pub fn run(
        &self,
        data: &mut PipelineData,
        temp_zone: &Zone,
        call_descriptor: *const CallDescriptor,
        linkage: *mut Linkage,
        code_tracer: *mut CodeTracer,
    ) -> Option<BailoutReason> {
        let graph: &mut Graph = &mut data.graph();

        data.InitializeInstructionComponent(unsafe { &*call_descriptor });

        let selector: InstructionSelector = InstructionSelector::ForTurboshaft(
            temp_zone,
            graph.op_id_count(),
            unsafe { &*linkage },
            data.sequence(),
            graph,
            data.frame(),
            if data.info().switch_jump_table() {
                InstructionSelector::kEnableSwitchJumpTable
            } else {
                InstructionSelector::kDisableSwitchJumpTable
            },
            &data.info().tick_counter(),
            data.broker(),
            &data.max_unoptimized_frame_height(),
            &data.max_pushed_argument_count(),
            if data.info().source_positions() {
                InstructionSelector::kAllSourcePositions
            } else {
                InstructionSelector::kCallSourcePositions
            },
            InstructionSelector::SupportedFeatures(),
            if v8_flags.turbo_instruction_scheduling {
                InstructionSelector::kEnableScheduling
            } else {
                InstructionSelector::kDisableScheduling
            },
            if data.assembler_options().enable_root_relative_access {
                InstructionSelector::kEnableRootsRelativeAddressing
            } else {
                InstructionSelector::kDisableRootsRelativeAddressing
            },
            if data.info().trace_turbo_json() {
                InstructionSelector::kEnableTraceTurboJson
            } else {
                InstructionSelector::kDisableTraceTurboJson
            },
        );

        if let Some(bailout) = selector.SelectInstructions() {
            return Some(bailout);
        }
        unsafe {
          Self::trace_sequence(
              data.info(),
              data.sequence(),
              data.broker(),
              code_tracer,
              "after instruction selection",
          );
        }

        None
    }

    unsafe fn trace_sequence(
        info: *mut crate::codegen::optimized_compilation_info::OptimizedCompilationInfo,
        sequence: *mut crate::compiler::backend::instruction::InstructionSequence,
        broker: *mut JSHeapBroker,
        code_tracer: *mut CodeTracer,
        phase_name: &str,
    ) {
        if (&*info).trace_turbo_json() {
            let scope = UnparkedScopeIfNeeded::new(broker);
            let allow_deref = AllowHandleDereference {};
            let mut json_of = TurboJsonFile::new(info, std::ios_base::app);
            json_of.stream() << "{\"name\":\"" << phase_name << "\",\"type\":\"sequence\""
                    << ",\"blocks\":" << InstructionSequenceAsJSON{sequence}
                    << ",\"register_allocation\":{"
                    << "\"fixed_double_live_ranges\": {}"
                    << ",\"fixed_live_ranges\": {}"
                    << ",\"live_ranges\": {}"
                    << "}},\n";
        }
        if (&*info).trace_turbo_graph() {
            let scope = UnparkedScopeIfNeeded::new(broker);
            let allow_deref = AllowHandleDereference {};
            let mut tracing_scope = CodeTracer::StreamScope::new(code_tracer);
            tracing_scope.stream() << "----- Instruction sequence " << phase_name
                               << " -----\n"
                               << (&*sequence);
        }
    }
}

pub mod v8_flags {
  pub static turbo_instruction_scheduling: bool = false;
}

pub struct ZoneVector<T> {
  elements: Vec<T>,
  zone: *mut Zone,
}

impl<T> ZoneVector<T> {
  pub fn new(zone: *mut Zone) -> Self {
      ZoneVector {
          elements: Vec::new(),
          zone,
      }
  }

  pub fn reserve(&mut self, additional: usize) {
      self.elements.reserve(additional);
  }

  pub fn push(&mut self, value: T) {
      self.elements.push(value);
  }

  pub fn pop(&mut self) -> Option<T> {
      self.elements.pop()
  }

  pub fn is_empty(&self) -> bool {
      self.elements.is_empty()
  }

  pub fn last_mut(&mut self) -> Option<&mut T> {
      self.elements.last_mut()
  }

  pub fn resize(&mut self, new_len: usize, value: T)
      where T: Clone
  {
      self.elements.resize(new_len, value);
  }

  pub fn zone(&self) -> &Zone {
      unsafe { &*self.zone }
  }

  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.elements.iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.elements.iter_mut()
  }

  pub fn get(&self, index: usize) -> Option<&T> {
    self.elements.get(index)
  }

  pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    self.elements.get_mut(index)
  }

  pub fn len(&self) -> usize {
    self.elements.len()
  }
}

impl<T> std::ops::Index<usize> for ZoneVector<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
      &self.elements[index]
  }
}

impl<T> std::ops::IndexMut<usize> for ZoneVector<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      &mut self.elements[index]
  }
}
}

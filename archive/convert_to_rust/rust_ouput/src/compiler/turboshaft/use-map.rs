// Converted from V8 C++ source files:
// Header: use-map.h
// Implementation: use-map.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/turboshaft/use-map.h

pub type FunctionType = fn(op: &Operation, zone: &mut Zone) -> bool;

// UseMap computes uses of all operations of the given turboshaft graph. It
// provides a mapping from `OpIndex` to its `uses`.
pub struct UseMap {
    table_: FixedOpIndexSidetable<PerOperationUses>,
    uses_: Vec<OpIndex>,
    saturated_uses_: Vec<Vec<OpIndex>>,
}

struct PerOperationUses {
    // We encode offsets as follows:
    // offset < 0: -offset-1 indexes into {saturated_uses_}.
    // offset = 0: definition not visited yet.
    // offset > 0: offset indexes into {uses_}.
    offset: i32,
    count: u32,
}

impl PerOperationUses {
    fn new() -> Self {
        PerOperationUses {
            offset: 0,
            count: 0,
        }
    }
}

impl UseMap {
    pub fn new(graph: &Graph, zone: &mut Zone, filter: FunctionType) -> Self {
        let mut table_: FixedOpIndexSidetable<PerOperationUses> =
            FixedOpIndexSidetable::new(graph.op_id_count(), zone, graph);
        let mut uses_: Vec<OpIndex> = Vec::new();
        let mut saturated_uses_: Vec<Vec<OpIndex>> = Vec::new();

        let mut delayed_phi_uses: Vec<(OpIndex, OpIndex)> = Vec::new();

        // We preallocate for 2 uses per operation.
        uses_.reserve(graph.op_id_count() * 2);

        // We skip {offset:0} to use {offset == 0} as uninitialized.
        let mut offset: u32 = 1;
        for index in 0..graph.block_count() {
            let block_index = BlockIndex { index };
            let block = graph.get(block_index).unwrap();

            let block_ops = graph.operation_indices(block);
            for op_index in block_ops {
                let op = graph.get(op_index).unwrap();
                // When we see a definition, we allocate space in the {uses_}.
                assert_eq!(table_.get(op_index).offset, 0);
                assert_eq!(table_.get(op_index).count, 0);

                if op.saturated_use_count.is_saturated() {
                    table_.get_mut(op_index).offset =
                        -(saturated_uses_.len() as i32) - 1;
                    saturated_uses_.push(Vec::new());
                    saturated_uses_
                        .last_mut()
                        .unwrap()
                        .reserve(u8::MAX as usize);
                } else {
                    table_.get_mut(op_index).offset = offset as i32;
                    offset += op.saturated_use_count.get() as u32;
                    uses_.resize(offset as usize, OpIndex { index: 0 });
                }

                if filter(op, zone) {
                    continue;
                }

                if block.is_loop() {
                    if op.is::<PhiOp>() {
                        assert_eq!(op.input_count, 2);
                        assert_eq!(PhiOp::K_LOOP_PHI_BACK_EDGE_INDEX, 1);
                        UseMap::add_use(&mut table_, &mut uses_, &mut saturated_uses_, graph, op.input(0), op_index);
                        // Delay back edge of loop Phis.
                        delayed_phi_uses.push((op.input(1), op_index));
                        continue;
                    }
                }

                // Add uses.
                for input_index in op.inputs() {
                    UseMap::add_use(&mut table_, &mut uses_, &mut saturated_uses_, graph, input_index, op_index);
                }
            }
        }

        for (input_index, op_index) in delayed_phi_uses {
            UseMap::add_use(&mut table_, &mut uses_, &mut saturated_uses_, graph, input_index, op_index);
        }

        UseMap {
            table_: table_,
            uses_: uses_,
            saturated_uses_: saturated_uses_,
        }
    }

    pub fn uses(&self, index: OpIndex) -> Vec<OpIndex> {
        assert!(index.valid());
        let offset = self.table_.get(index).offset;
        let count = self.table_.get(index).count;
        assert_ne!(offset, 0);
        if offset > 0 {
            let start = offset as usize - 1;
            self.uses_[start..(start + count as usize)].to_vec()
        } else {
            let saturated_uses_index = (-offset - 1) as usize;
            assert_eq!(count as usize, self.saturated_uses_[saturated_uses_index].len());
            self.saturated_uses_[saturated_uses_index].clone()
        }
    }

    fn add_use(
        table_: &mut FixedOpIndexSidetable<PerOperationUses>,
        uses_: &mut Vec<OpIndex>,
        saturated_uses_: &mut Vec<Vec<OpIndex>>,
        graph: &Graph,
        node: OpIndex,
        use_: OpIndex,
    ) {
        let input_offset = table_.get(node).offset;
        let input_count = &mut table_.get_mut(node).count;
        assert_ne!(input_offset, 0);
        if input_offset > 0 {
            assert!((*input_count as usize) < graph.get(node).unwrap().saturated_use_count.get() as usize);
            let index = input_offset as usize + *input_count as usize - 1;
            while index >= uses_.len() {
              uses_.push(OpIndex{index: 0});
            }
            assert!(!uses_[index].valid());
            uses_[index] = use_;
        } else {
            let uses = &mut saturated_uses_[(-input_offset - 1) as usize];
            assert_eq!(uses.len(), *input_count as usize);
            uses.push(use_);
        }
        *input_count += 1;
    }
    pub fn new_default(graph: &Graph, zone: &mut Zone) -> Self {
        UseMap::new(graph, zone, |_: &Operation, _: &mut Zone| false)
    }
}

// SimdUseMap computes uses of SIMD operations of the given turboshaft graph and
// skip other operations.
pub struct SimdUseMap {
    use_map: UseMap,
}

impl SimdUseMap {
    pub fn new(graph: &Graph, zone: &mut Zone) -> Self {
        SimdUseMap {
            use_map: UseMap::new(
                graph,
                zone,
                |op: &Operation, zone: &mut Zone| {
                    if op.outputs_rep().len() == 1
                        && op.outputs_rep()[0] == RegisterRepresentation::Simd128
                    {
                        return false;
                    }

                    let mut storage: Vec<MaybeRegisterRepresentation> = Vec::new();
                    for rep in op.inputs_rep(&mut storage) {
                        if rep == MaybeRegisterRepresentation::Simd128 {
                            return false;
                        }
                    }
                    return true;
                },
            ),
        }
    }
    pub fn uses(&self, index: OpIndex) -> Vec<OpIndex> {
      self.use_map.uses(index)
    }
}

use crate::compiler::turboshaft::sidetable::FixedOpIndexSidetable;
use crate::compiler::turboshaft::graph::Graph;
use crate::compiler::turboshaft::graph::BlockIndex;
use crate::compiler::turboshaft::graph::Block;
use crate::compiler::turboshaft::graph::Operation;
use crate::compiler::turboshaft::graph::OpIndex;
use crate::compiler::turboshaft::representations::RegisterRepresentation;
use crate::compiler::turboshaft::representations::MaybeRegisterRepresentation;
use crate::zone::zone::Zone;
use crate::compiler::turboshaft::op::PhiOp;
use std::vec;
use std::assert_eq;
use std::assert_ne;
use std::assert;

trait IsSaturated {
    fn is_saturated(&self) -> bool;
}

impl IsSaturated for crate::compiler::turboshaft::graph::SaturatedUseCount {
    fn is_saturated(&self) -> bool {
        self.saturated
    }
}

trait Get {
    fn get(&self) -> i32;
}

impl Get for crate::compiler::turboshaft::graph::SaturatedUseCount {
    fn get(&self) -> i32 {
        self.count
    }
}

trait Valid {
    fn valid(&self) -> bool;
}
impl Valid for OpIndex {
  fn valid(&self) -> bool {
    self.index != 0
  }
}

trait Is<T> {
    fn is::<T>(&self) -> bool;
}

impl Is<PhiOp> for Operation {
    fn is::<PhiOp>(&self) -> bool {
        self.kind == "PhiOp"
    }
}

trait Inputs {
  fn inputs(&self) -> std::vec::IntoIter<OpIndex>;
}

impl Inputs for Operation {
  fn inputs(&self) -> std::vec::IntoIter<OpIndex> {
    self.inputs.clone().into_iter()
  }
}

trait GetGraph {
  fn get(&self, op_index: OpIndex) -> Result<&Operation, String>;
  fn get(&self, block_index: BlockIndex) -> Result<&Block, String>;
}

impl GetGraph for Graph {
  fn get(&self, op_index: OpIndex) -> Result<&Operation, String> {
    self.operations.get(op_index.index as usize).ok_or("Operation not found".to_string())
  }
    fn get(&self, block_index: BlockIndex) -> Result<&Block, String> {
        self.blocks.get(block_index.index as usize).ok_or("Block not found".to_string())
    }
}
trait OperationIndices {
    fn operation_indices(&self, block: &Block) -> std::vec::IntoIter<OpIndex>;
}
impl OperationIndices for Graph {
    fn operation_indices(&self, block: &Block) -> std::vec::IntoIter<OpIndex> {
        block.operations.clone().into_iter()
    }
}
trait OpInputs {
  fn input(&self, index: usize) -> OpIndex;
}
impl OpInputs for Operation {
  fn input(&self, index: usize) -> OpIndex {
    self.inputs[index]
  }
}
trait BlockType {
  fn is_loop(&self) -> bool;
}
impl BlockType for Block {
    fn is_loop(&self) -> bool {
        self.kind == "Loop"
    }
}
trait InputRep {
  fn inputs_rep(&self, storage: &mut Vec<MaybeRegisterRepresentation>) -> &Vec<MaybeRegisterRepresentation>;
  fn outputs_rep(&self) -> &Vec<RegisterRepresentation>;
}
impl InputRep for Operation {
  fn inputs_rep(&self, _storage: &mut Vec<MaybeRegisterRepresentation>) -> &Vec<MaybeRegisterRepresentation> {
    &self.input_reps
  }
  fn outputs_rep(&self) -> &Vec<RegisterRepresentation> {
    &self.output_reps
  }
}

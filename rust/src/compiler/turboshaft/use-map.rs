// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod use_map {
    use crate::compiler::turboshaft::graph::{Block, BlockIndex, Graph, OpIndex, Operation, PhiOp};
    use crate::base::vector::Vector;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type FunctionType = fn(&Operation, &Zone) -> bool;

    #[derive(Debug, Default)]
    struct TableEntry {
        offset: i32,
        count: u32,
    }

    pub struct UseMap {
        table_: Vec<TableEntry>,
        uses_: Vec<OpIndex>,
        saturated_uses_: Vec<ZoneVector<OpIndex>>,
    }

    pub struct Zone {
        // Placeholder - replace with actual Zone implementation if needed
        // Consider using a memory arena crate like `bumpalo`
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }

    type ZoneVector<T> = Vec<T>;

    impl UseMap {
        pub fn new(graph: &Graph, zone: &Zone, filter: FunctionType) -> Self {
            let mut table_ = vec![TableEntry::default(); graph.op_id_count()];
            let mut uses_ = Vec::new();
            let mut saturated_uses_ = Vec::new();
            let mut delayed_phi_uses: Vec<(OpIndex, OpIndex)> = Vec::new();

            uses_.reserve(graph.op_id_count() * 2);

            let mut offset: u32 = 1;
            for index in 0..graph.block_count() {
                let block_index = BlockIndex(index);
                let block = graph.get(block_index);

                let block_ops = graph.operation_indices(block);
                for op_index in block_ops {
                    let op = graph.get(op_index);

                    assert_eq!(table_[op_index.0 as usize].offset, 0);
                    assert_eq!(table_[op_index.0 as usize].count, 0);

                    if op.saturated_use_count.is_saturated() {
                        table_[op_index.0 as usize].offset =
                            -(saturated_uses_.len() as i32) - 1;
                        saturated_uses_.push(ZoneVector::new());
                        saturated_uses_.last_mut().unwrap().reserve(u8::MAX as usize);
                    } else {
                        table_[op_index.0 as usize].offset = offset as i32;
                        offset += op.saturated_use_count.get() as u32;
                        uses_.resize(offset as usize, OpIndex(0)); //Initialize with a default OpIndex(0)
                    }

                    if filter(op, zone) {
                        continue;
                    }

                    if block.is_loop() {
                        if op.is::<PhiOp>() {
                            assert_eq!(op.input_count, 2);
                            assert_eq!(PhiOp::k_loop_phi_back_edge_index(), 1);
                            Self::add_use(graph, op.input(0), op_index, &mut table_, &mut uses_, &mut saturated_uses_);

                            delayed_phi_uses.push((op.input(1), op_index));
                            continue;
                        }
                    }

                    for input_index in op.inputs() {
                        Self::add_use(graph, input_index, op_index, &mut table_, &mut uses_, &mut saturated_uses_);
                    }
                }
            }

            for (input_index, op_index) in delayed_phi_uses {
                Self::add_use(graph, input_index, op_index, &mut table_, &mut uses_, &mut saturated_uses_);
            }

            UseMap {
                table_,
                uses_,
                saturated_uses_,
            }
        }

        pub fn uses(&self, index: OpIndex) -> Vector<OpIndex> {
            assert!(index.valid());
            let offset = self.table_[index.0 as usize].offset;
            let count = self.table_[index.0 as usize].count;
            assert_ne!(offset, 0);

            if offset > 0 {
                let start = offset as usize;
                Vector::from_slice(&self.uses_[start..start + count as usize])
            } else {
                let vec = &self.saturated_uses_[-offset as usize - 1];
                assert_eq!(count as usize, vec.len());
                Vector::from_slice(vec.as_slice())
            }
        }

        fn add_use(graph: &Graph, node: OpIndex, use_: OpIndex, table_: &mut Vec<TableEntry>, uses_: &mut Vec<OpIndex>, saturated_uses_: &mut Vec<ZoneVector<OpIndex>>) {
            let input_offset = table_[node.0 as usize].offset;
            let input_count = &mut table_[node.0 as usize].count;
            assert_ne!(input_offset, 0);

            if input_offset > 0 {
                assert!(*input_count < graph.get(node).saturated_use_count.get() as u32);
                assert!(!uses_.get(input_offset as usize + *input_count as usize).is_some_and(|x| x.valid()));
                uses_[input_offset as usize + *input_count as usize] = use_;
            } else {
                let uses = &mut saturated_uses_[-input_offset as usize - 1];
                assert_eq!(uses.len(), *input_count as usize);
                uses.push(use_);
            }
            *input_count += 1;
        }
    }
}

pub mod base {
    pub mod vector {
        #[derive(Debug)]
        pub struct Vector<'a, T> {
            data: &'a [T],
        }

        impl<'a, T> Vector<'a, T> {
            pub fn from_slice(slice: &'a [T]) -> Self {
                Vector { data: slice }
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }
            
            pub fn data(&self) -> &'a [T] {
                self.data
            }
        }
    }
}

pub mod compiler {
    pub mod turboshaft {
        pub mod graph {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct OpIndex(pub u32);

            impl OpIndex {
                pub fn valid(&self) -> bool {
                    self.0 != 0
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct BlockIndex(pub u32);

            #[derive(Debug)]
            pub struct Graph {
                op_id_count_: usize,
                block_count_: usize,
                operations_: Vec<Operation>, // Simplified: Store operations directly.
                blocks_: Vec<Block>
            }

            impl Graph {
                pub fn new(op_id_count: usize, block_count: usize, operations: Vec<Operation>, blocks: Vec<Block>) -> Self {
                    Graph {
                        op_id_count_: op_id_count,
                        block_count_: block_count,
                        operations_: operations,
                        blocks_: blocks
                    }
                }

                pub fn op_id_count(&self) -> usize {
                    self.op_id_count_
                }

                pub fn block_count(&self) -> usize {
                    self.block_count_
                }

                pub fn get(&self, index: OpIndex) -> &Operation {
                    &self.operations_[index.0 as usize]
                }
                
                 pub fn get_block(&self, index: BlockIndex) -> &Block {
                    &self.blocks_[index.0 as usize]
                }
                
                pub fn operation_indices(&self, block: &Block) -> Vec<OpIndex> {
                     block.operations().to_vec()
                }
            }

            #[derive(Debug)]
            pub struct Operation {
                pub input_count: u32,
                pub saturated_use_count: SaturatedUseCount,
                pub inputs_: Vec<OpIndex>,
                // Added type field for Op
                pub op_type: OperationType
            }

            impl Operation {
                pub fn inputs(&self) -> &[OpIndex] {
                    &self.inputs_
                }

                pub fn input(&self, index: usize) -> OpIndex {
                    self.inputs_[index]
                }

                pub fn is<T>(&self) -> bool {
                    match self.op_type {
                        OperationType::PhiOp => {
                            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<PhiOp>() {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct SaturatedUseCount {
                count: u8,
                saturated: bool
            }

            impl SaturatedUseCount {
                pub fn new(count: u8, saturated: bool) -> Self {
                    SaturatedUseCount {
                        count: count,
                        saturated: saturated
                    }
                }

                pub fn get(&self) -> u8 {
                    self.count
                }

                pub fn is_saturated(&self) -> bool {
                    self.saturated
                }
            }

            #[derive(Debug)]
            pub struct Block {
               operations: Vec<OpIndex>,
               is_loop: bool
            }

            impl Block {
                pub fn new(operations: Vec<OpIndex>, is_loop: bool) -> Self {
                    Block {
                        operations: operations,
                        is_loop: is_loop
                    }
                }

                pub fn operations(&self) -> &Vec<OpIndex> {
                    &self.operations
                }

                pub fn is_loop(&self) -> bool {
                    self.is_loop
                }
            }

            #[derive(Debug)]
            pub struct PhiOp;

            impl PhiOp {
                pub fn k_loop_phi_back_edge_index() -> usize {
                    1
                }
            }

            #[derive(Debug)]
            pub enum OperationType {
                PhiOp,
                Other
            }
        }
    }
}
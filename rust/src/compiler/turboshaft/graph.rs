// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::fmt::Display;
use std::vec;

// Placeholder for base::logging.  In V8, this is a logging system.
// For this minimal translation, we will just use println!.
macro_rules! PrintF {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Index {
    id: usize,
}

impl Index {
    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockIndex {
    index: Index,
}

impl BlockIndex {
    pub fn new(id: usize) -> Self {
        BlockIndex { index: Index { id } }
    }

    pub fn id(&self) -> usize {
        self.index.id()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OperationIndex {
    index: Index,
}

impl OperationIndex {
    pub fn new(id: usize) -> Self {
        OperationIndex { index: Index { id } }
    }

    pub fn id(&self) -> usize {
        self.index.id()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockKind {
    LoopHeader,
    Merge,
    BranchTarget,
}

impl Display for BlockKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockKind::LoopHeader => write!(f, "LOOP"),
            BlockKind::Merge => write!(f, "MERGE"),
            BlockKind::BranchTarget => write!(f, "BLOCK"),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    kind: BlockKind,
    index: BlockIndex,
    predecessors: Vec<*const Block>, // Using raw pointers here, needs careful management in Graph
    children: Vec<*mut Block>,      // Using raw pointers here, needs careful management in Graph
}

impl Block {
    pub fn new(kind: BlockKind, index: BlockIndex) -> Self {
        Block {
            kind,
            index,
            predecessors: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn kind(&self) -> BlockKind {
        self.kind
    }

    pub fn index(&self) -> BlockIndex {
        self.index
    }

    pub fn predecessors(&self) -> &Vec<*const Block> {
        &self.predecessors
    }

    pub fn children(&self) -> Vec<&mut Block> {
        self.children.iter().map(|&child| unsafe { &mut *child }).collect()
    }

    pub fn add_child(&mut self, child: *mut Block) {
        self.children.push(child);
    }

    pub fn add_predecessor(&mut self, predecessor: *const Block) {
        self.predecessors.push(predecessor);
    }

    // PrintDominatorTree prints the dominator tree.
    pub fn print_dominator_tree(&self, mut tree_symbols: Vec<&str>, has_next: bool) {
        // Printing the current node.
        if tree_symbols.is_empty() {
            // This node is the root of the tree.
            PrintF!("B{}\n", self.index().id());
            tree_symbols.push("");
        } else {
            // This node is not the root of the tree; we start by printing the
            // connectors of the previous levels.
            for s in &tree_symbols {
                PrintF!("{}", s);
            }
            // Then, we print the node id, preceeded by a ╠ or ╚ connector.
            let tree_connector_symbol = if has_next { "╠" } else { "╚" };
            PrintF!("{} B{}\n", tree_connector_symbol, self.index().id());
            // And we add to the stack a connector to continue this path (if needed)
            // while printing the current node's children.
            let tree_cont_symbol = if has_next { "║ " } else { "  " };
            tree_symbols.push(tree_cont_symbol);
        }
        // Recursively printing the children of this node.
        let children = self.children();
        for child in &children {
            child.print_dominator_tree(tree_symbols.clone(), child != children.last().unwrap());
        }
        // Removing from the stack the "║" or " " corresponding to this node.
        tree_symbols.pop();
    }
}

pub struct PrintAsBlockHeader<'a> {
    pub block: &'a Block,
    pub block_id: BlockIndex,
}

impl<'a> Display for PrintAsBlockHeader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.block.kind(), self.block_id.id())?;
        if !self.block.predecessors().is_empty() {
            write!(f, " <- ")?;
            let mut first = true;
            for pred_ptr in self.block.predecessors() {
                let pred = unsafe { &**pred_ptr }; // Dereference raw pointer
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", pred.index().id())?;
                first = false;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Operation {
    // Placeholder for Operation data.  For this translation, we only need to be able to print.
    name: String,
}

impl Operation {
    pub fn new(name: String) -> Self {
        Operation { name }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Graph {
    blocks: Vec<Box<Block>>,
    operations: Vec<Vec<Operation>>, // Operations are stored per block
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            blocks: Vec::new(),
            operations: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: BlockKind) -> BlockIndex {
        let index = BlockIndex::new(self.blocks.len());
        let block = Block::new(block, index);
        self.blocks.push(Box::new(block));
        self.operations.push(Vec::new()); //Initialize operations vector for the new block
        index
    }

    pub fn get_block_mut(&mut self, index: BlockIndex) -> Option<&mut Block> {
        self.blocks.get_mut(index.id()).map(|block| &mut **block)
    }

    pub fn get_block(&self, index: BlockIndex) -> Option<&Block> {
        self.blocks.get(index.id()).map(|block| &**block)
    }

    pub fn add_operation(&mut self, block_index: BlockIndex, operation: Operation) -> OperationIndex {
        let block_ops = &mut self.operations[block_index.id()];
        let index = OperationIndex::new(block_ops.len());
        block_ops.push(operation);
        index
    }

    pub fn blocks(&self) -> &Vec<Box<Block>> {
        &self.blocks
    }

    pub fn operations(&self, block: &Block) -> &Vec<Operation> {
        &self.operations[block.index().id()]
    }

    pub fn index(&self, op: &Operation) -> OperationIndex {
        // This is inefficient, but given the lack of an Operation index,
        // we need to iterate through all blocks to find which one contains the operation
        for (block_idx, block_ops) in self.operations.iter().enumerate() {
          if block_ops.contains(op) {
            for (op_idx, block_op) in block_ops.iter().enumerate() {
              if block_op as *const Operation == op as *const Operation {
                return OperationIndex::new(op_idx);
              }
            }
          }
        }

        panic!("Operation not found in any block's operations vector");
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            writeln!(f, "\n{}", PrintAsBlockHeader { block: &**block, block_id: block.index() })?;
            for (op_idx, op) in self.operations(block).iter().enumerate() {
                writeln!(f, "{:5}: {}", op_idx, op)?;
            }
        }
        Ok(())
    }
}
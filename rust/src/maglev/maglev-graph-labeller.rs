// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::fmt;

/// Placeholder for MaglevCompilationUnit.  Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MaglevCompilationUnit {}

/// Placeholder for BytecodeOffset. Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BytecodeOffset {}

impl BytecodeOffset {
    pub fn None() -> Self {
        BytecodeOffset {}
    }
}

/// Placeholder for SourcePosition.  Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SourcePosition {}

impl SourcePosition {
    pub fn Unknown() -> Self {
        SourcePosition {}
    }
}

/// Placeholder for NodeBase.  Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeBase {
    id: Option<i32>,
}

impl NodeBase {
    pub fn new() -> Self {
        NodeBase { id: None }
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(-1)
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn is<T>(&self) -> bool
    where
        T: 'static, // Placeholder for the type check.  Needs proper implementation
    {
        std::any::TypeId::of::<Self>() == std::any::TypeId::of::<T>()
    }

    pub fn cast<T>(&self) -> &T
    where
        T: 'static, // Placeholder for the type check.  Needs proper implementation
    {
        assert!(self.is::<T>());
        unsafe { &*(self as *const Self as *const T) }
    }
}

/// Placeholder for BasicBlock. Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BasicBlock {}

/// Placeholder for Input. Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Input {
    node: *const NodeBase,
    operand: i32,
}

impl Input {
    pub fn new(node: *const NodeBase, operand: i32) -> Self {
        Input { node, operand }
    }

    pub fn node(&self) -> *const NodeBase {
        self.node
    }

    pub fn operand(&self) -> i32 {
        self.operand
    }
}

/// Placeholder for VirtualObject. Needs proper definition based on v8 codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VirtualObject {
    id: i32,
    allocation: *const NodeBase,
}

impl VirtualObject {
    pub fn new(id: i32, allocation: *const NodeBase) -> Self {
        VirtualObject { id, allocation }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn allocation(&self) -> *const NodeBase {
        self.allocation
    }
}

impl NodeBase {
    // Placeholder for Cast Implementation since Rust doesn't have implicit casting like C++
    // Requires more context on how types are being used.
    //
    // pub fn cast<T>(&self) -> &T {
    //     unimplemented!("Needs proper implementation for type casting from NodeBase to T")
    // }
}

pub struct MaglevGraphLabeller {
    block_ids_: HashMap<*const BasicBlock, i32>,
    nodes_: HashMap<*const NodeBase, NodeInfo>,
    next_block_label_: i32,
    next_node_label_: i32,
}

impl MaglevGraphLabeller {
    #[derive(Debug, Copy, Clone)]
    pub struct Provenance {
        pub unit: *const MaglevCompilationUnit,
        pub bytecode_offset: BytecodeOffset,
        pub position: SourcePosition,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct NodeInfo {
        pub label: i32,
        pub provenance: Provenance,
    }

    pub fn new() -> Self {
        MaglevGraphLabeller {
            block_ids_: HashMap::new(),
            nodes_: HashMap::new(),
            next_block_label_: 1,
            next_node_label_: 1,
        }
    }

    pub fn register_node(
        &mut self,
        node: *const NodeBase,
        unit: *const MaglevCompilationUnit,
        bytecode_offset: BytecodeOffset,
        position: SourcePosition,
    ) {
        if self
            .nodes_
            .insert(
                node,
                NodeInfo {
                    label: self.next_node_label_,
                    provenance: Provenance {
                        unit,
                        bytecode_offset,
                        position,
                    },
                },
            )
            .is_none()
        {
            self.next_node_label_ += 1;
        }
    }

    pub fn register_node_simple(&mut self, node: *const NodeBase) {
        self.register_node(
            node,
            std::ptr::null(),
            BytecodeOffset::None(),
            SourcePosition::Unknown(),
        );
    }

    pub fn register_basic_block(&mut self, block: *const BasicBlock) {
        self.block_ids_.insert(block, self.next_block_label_);
        self.next_block_label_ += 1;
    }

    pub fn block_id(&self, block: *const BasicBlock) -> Option<i32> {
        self.block_ids_.get(&block).copied()
    }

    pub fn node_id(&self, node: *const NodeBase) -> Option<i32> {
        self.nodes_.get(&node).map(|node_info| node_info.label)
    }

    pub fn get_node_provenance(&self, node: *const NodeBase) -> Option<Provenance> {
        self.nodes_.get(&node).map(|node_info| node_info.provenance)
    }

    pub fn max_node_id(&self) -> i32 {
        self.next_node_label_ - 1
    }

    pub fn print_node_label(&self, os: &mut dyn fmt::Write, node: *const NodeBase) -> fmt::Result {
        unsafe {
            let node_ref = node.as_ref();

            if let Some(node_ref) = node_ref {
                if node_ref.is::<VirtualObject>() {
                    let vo: &VirtualObject = node_ref.cast::<VirtualObject>();
                    write!(os, "VO{{{}}}:", vo.id())?;
                    self.print_node_label(os, vo.allocation())?;
                    return Ok(());
                }
            }
        }

        match self.nodes_.get(&node) {
            Some(node_info) => {
                unsafe {
                    if let Some(node_ref) = node.as_ref() {
                        if node_ref.has_id() {
                            write!(os, "v{}/", node_ref.id())?;
                        }
                    }
                }
                write!(os, "n{}", node_info.label)
            }
            None => write!(os, "<unregistered node {:?}>", node),
        }
    }

    pub fn print_input(&self, os: &mut dyn fmt::Write, input: &Input) -> fmt::Result {
        self.print_node_label(os, input.node())?;
        write!(os, ":{}", input.operand())
    }
}
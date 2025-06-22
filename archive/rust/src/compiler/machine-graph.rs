// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/machine-graph.h

use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

// Placeholder for RelocInfo::Mode and Builtin.  These need to be defined for
// the Relocatable*Constant functions to be complete.
#[derive(Debug, Clone, Copy)]
enum RelocInfoMode {
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum Builtin {
    Unknown,
}

// Placeholder for ExternalReference and Runtime::FunctionId.  These need to be
// defined for the ExternalConstant functions to be complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExternalReference {}

#[derive(Debug, Clone, Copy)]
enum FunctionId {}


// Placeholder for NodeId.
type NodeId = usize;

// Placeholder for TFGraph, CommonOperatorBuilder, MachineOperatorBuilder, Node
// These will need appropriate definitions for the MachineGraph struct to be
// fully functional.
struct TFGraph {}
struct CommonOperatorBuilder {}
struct MachineOperatorBuilder {}
struct Node {}

trait ZoneObject {}

struct CommonNodeCache {}

// A map with specialized functionality for NodeId keys and int values.
// Offers O(1) put and get operations.
struct NodeAuxDataMap<T: Copy, const DEFAULT: i32> {
    map: HashMap<NodeId, T>,
}

impl<T: Copy, const DEFAULT: i32> NodeAuxDataMap<T, DEFAULT> {
    fn new() -> Self {
        NodeAuxDataMap { map: HashMap::new() }
    }

    fn put(&mut self, key: NodeId, value: T) {
        self.map.insert(key, value);
    }

    fn get(&self, key: NodeId) -> T
    where T: From<i32>
    {
        *self.map.get(&key).unwrap_or(&T::from(DEFAULT))
    }

    fn reserve(&mut self, _num_call_instructions: usize) {
        // No-op in the Rust version.
    }
}

/// Implements a facade on a TFGraph, enhancing the graph with
/// machine-specific notions, including a builder for common and
/// machine operators, as well as caching primitive constants.
pub struct MachineGraph {
    graph: *mut TFGraph,
    common: *mut CommonOperatorBuilder,
    machine: *mut MachineOperatorBuilder,
    cache: CommonNodeCache,
    call_counts: NodeAuxDataMap<i32, -1>,
    dead: *mut Node,
}

impl MachineGraph {
    pub fn new(
        graph: *mut TFGraph,
        common: *mut CommonOperatorBuilder,
        machine: *mut MachineOperatorBuilder,
    ) -> Self {
        MachineGraph {
            graph,
            common,
            machine,
            cache: CommonNodeCache {},
            call_counts: NodeAuxDataMap::new(),
            dead: std::ptr::null_mut(),
        }
    }

    /// Creates a new (unique) Int32Constant node.
    pub fn unique_int32_constant(&mut self, _value: i32) -> *mut Node {
        // TODO: Implement the unique node creation logic.
        std::ptr::null_mut()
    }

    pub fn unique_int64_constant(&mut self, _value: i64) -> *mut Node {
        // TODO: Implement the unique node creation logic.
        std::ptr::null_mut()
    }

    /// Creates an Int32Constant node, usually canonicalized.
    pub fn int32_constant(&mut self, _value: i32) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn uint32_constant(&mut self, value: u32) -> *mut Node {
        self.int32_constant(value as i32)
    }

    /// Creates a Int64Constant node, usually canonicalized.
    pub fn int64_constant(&mut self, _value: i64) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn uint64_constant(&mut self, value: u64) -> *mut Node {
        self.int64_constant(value as i64)
    }

    /// Creates an Int32Constant/Int64Constant node, depending on the word size
    /// of the target machine.
    /// TODO(turbofan): Code using Int32Constant/Int64Constant to store pointer
    /// constants is probably not serializable.
    pub fn int_ptr_constant(&mut self, _value: isize) -> *mut Node {
        // TODO: Implement node creation logic based on word size.
        std::ptr::null_mut()
    }

    pub fn uint_ptr_constant(&mut self, value: usize) -> *mut Node {
        self.int_ptr_constant(value as isize)
    }

    pub fn unique_int_ptr_constant(&mut self, _value: isize) -> *mut Node {
         // TODO: Implement the unique node creation logic.
        std::ptr::null_mut()
    }

    pub fn tagged_index_constant(&mut self, _value: isize) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn relocatable_int32_constant(&mut self, _value: i32, _rmode: RelocInfoMode) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn relocatable_int64_constant(&mut self, _value: i64, _rmode: RelocInfoMode) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn relocatable_int_ptr_constant(&mut self, _value: isize, _rmode: RelocInfoMode) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn relocatable_wasm_builtin_call_target(&mut self, _builtin: Builtin) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    /// Creates a Float32Constant node, usually canonicalized.
    pub fn float32_constant(&mut self, _value: f32) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    /// Creates a Float64Constant node, usually canonicalized.
    pub fn float64_constant(&mut self, _value: f64) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    /// Creates a PointerConstant node.
    pub fn pointer_constant(&mut self, value: usize) -> *mut Node {
        // TODO: Implement node creation logic.
        // Requires memory safety considerations (lifetime).
        std::ptr::null_mut()
    }

    pub fn pointer_constant_t<T>(&mut self, value: *mut T) -> *mut Node {
        self.pointer_constant(value as usize)
    }

    /// Creates an ExternalConstant node, usually canonicalized.
    pub fn external_constant(&mut self, _ref: ExternalReference) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    pub fn external_constant_function_id(&mut self, _function_id: FunctionId) -> *mut Node {
        // TODO: Implement node creation logic.
        std::ptr::null_mut()
    }

    /// Global cache of the dead node.
    pub fn dead(&mut self) -> *mut Node {
        if self.dead.is_null() {
            // TODO: Implement graph_->NewNode(common_->Dead()).
            self.dead = std::ptr::null_mut();
            // self.dead = unsafe { (*self.graph).new_node((*self.common).dead()) };
        }
        self.dead
    }

    /// Store and retrieve call count information.
    pub fn store_call_count(&mut self, call_id: NodeId, count: i32) {
        self.call_counts.put(call_id, count);
    }
    pub fn get_call_count(&self, call_id: NodeId) -> i32 {
        self.call_counts.get(call_id)
    }
    /// Use this to keep the number of map rehashings to a minimum.
    pub fn reserve_call_counts(&mut self, num_call_instructions: usize) {
        self.call_counts.reserve(num_call_instructions);
    }

    pub fn common(&self) -> *mut CommonOperatorBuilder {
        self.common
    }
    pub fn machine(&self) -> *mut MachineOperatorBuilder {
        self.machine
    }
    pub fn graph(&self) -> *mut TFGraph {
        self.graph
    }

    //TODO: Find correct type for Zone and implement its retrieval.
    // pub fn zone(&self) -> &Zone {
    //    self.graph.zone()
    // }
}
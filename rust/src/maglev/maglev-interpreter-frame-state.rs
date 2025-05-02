// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/maglev/maglev-interpreter-frame-state.rs

// use v8::internal;  // Placeholder for v8 crate
// use v8::internal::handles;  // Placeholder for v8 crate
// use v8::internal::interpreter::bytecode_register;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_basic_block;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_compilation_info;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_compilation_unit;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_graph_builder;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_graph_printer;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_graph;  // Placeholder for v8 crate
// use v8::internal::maglev::maglev_ir;  // Placeholder for v8 crate
// use v8::internal::objects::function_kind;  // Placeholder for v8 crate

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt;

// Placeholder types and enums.  Real definitions needed from v8 crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    kUnknown,
    kSmi,
    kString,
    kInternalizedString,
    kNumber,
    kBoolean,
    kObject,
    kArray,
    // ... other node types
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueRepresentation {
    kTagged,
    kInt32,
    kUint32,
    kIntPtr,
    kFloat64,
    kHoleyFloat64,
}

pub trait Node {
    fn properties(&self) -> NodeProperties;
    fn opcode(&self) -> Opcode;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Int32Constant,
    UnsafeSmiTagInt32,
    Int32ToNumber,
    Uint32ToNumber,
    IntPtrToNumber,
    Float64ToTagged,
    HoleyFloat64ToTagged,
    StringLength,
    BuiltinStringPrototypeCharCodeOrCodePointAt,
    LoadField, // Example
    // ... other opcodes
}

#[derive(Debug, Clone)]
pub struct NodeProperties {
    value_representation: ValueRepresentation,
    is_conversion: bool,
}

impl NodeProperties {
    pub fn new(value_representation: ValueRepresentation, is_conversion: bool) -> Self {
        NodeProperties {
            value_representation,
            is_conversion,
        }
    }

    pub fn value_representation(&self) -> ValueRepresentation {
        self.value_representation
    }

    pub fn is_conversion(&self) -> bool {
        self.is_conversion
    }
}

pub trait ValueNode: Node {
    fn id(&self) -> usize;
}

pub trait ControlNode: Node {
    fn predecessor_id(&self) -> usize;
    fn set_predecessor_id(&mut self, id: usize);
}

pub trait UnconditionalControlNode: ControlNode {}

pub trait MaglevCompilationUnit {
    fn zone(&self) -> &Zone;
    fn RegisterNodeInGraphLabeller<T: Node>(&self, node: &T);
    fn info(&self) -> &CompilationInfo;
    fn graph_labeller(&self) -> &GraphLabeller;
}

pub trait LocalIsolate {}

pub trait JSHeapBroker {
    //fn can_read_field_owner(&self, object: ValueNode, field: ValueNode) -> bool;
}

#[derive(Debug, Clone)]
pub struct Zone {
    // Simple arena allocator
    data: Vec<u8>,
    allocated: usize,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            data: Vec::new(),
            allocated: 0,
        }
    }

    pub fn allocate_array<T>(&mut self, count: usize) -> *mut T {
        let size = std::mem::size_of::<T>() * count;
        let align = std::mem::align_of::<T>();

        // Ensure proper alignment
        let aligned_allocated = (self.allocated + align - 1) & !(align - 1);
        if self.data.len() < aligned_allocated + size {
            self.data.resize(aligned_allocated + size, 0);
        }

        let ptr = self.data[aligned_allocated..].as_mut_ptr() as *mut T;
        self.allocated = aligned_allocated + size;
        ptr
    }

    pub fn allocate<T>(&mut self) -> *mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();

        // Ensure proper alignment
        let aligned_allocated = (self.allocated + align - 1) & !(align - 1);
        if self.data.len() < aligned_allocated + size {
            self.data.resize(aligned_allocated + size, 0);
        }

        let ptr = self.data[aligned_allocated..].as_mut_ptr() as *mut T;
        self.allocated = aligned_allocated + size;
        ptr
    }

    pub fn new_box<T>(&mut self, value: T) -> Box<T> {
        let ptr = self.allocate::<T>();
        unsafe {
            ptr.write(value);
            Box::from_raw(ptr)
        }
    }

    pub fn new_uninit_box<T>(&mut self) -> Box<std::mem::MaybeUninit<T>> {
        let ptr = self.allocate::<std::mem::MaybeUninit<T>>();
        unsafe {
            Box::from_raw(ptr)
        }
    }
}

// Example implementation, requires actual implementations from v8 crate.
impl Default for Zone {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($arg:tt)+) => {
        if !$condition {
            panic!("DCHECK failed: {}: {}", stringify!($condition), format!($($arg)+));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}: {}", stringify!($left), stringify!($right), format!($($arg)+));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("DCHECK_NE failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if $left == $right {
            panic!("DCHECK_NE failed: {} == {}: {}", stringify!($left), stringify!($right), format!($($arg)+));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implies:expr) => {
        if $condition && !$implies {
            panic!("DCHECK_IMPLIES failed: {} implies {}", stringify!($condition), stringify!($implies));
        }
    };
    ($condition:expr, $implies:expr, $($arg:tt)+) => {
        if $condition && !$implies {
            panic!("DCHECK_IMPLIES failed: {} implies {}: {}", stringify!($condition), stringify!($implies), format!($($arg)+));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_NOT_NULL {
    ($ptr:expr) => {
        if $ptr.is_null() {
            panic!("DCHECK_NOT_NULL failed: pointer is null");
        }
    };
}

#[macro_export]
macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
    ($($arg:tt)+) => {
        panic!("UNREACHABLE: {}", format!($($arg)+));
    };
}

// Example flag implementation
pub struct Flags {
    pub trace_maglev_graph_building: bool,
    pub trace_maglev_loop_speeling: bool,
    pub maglev_optimistic_peeled_loops: bool,
}

lazy_static::lazy_static! {
    pub static ref v8_flags: Flags = Flags {
        trace_maglev_graph_building: false,
        trace_maglev_loop_speeling: false,
        maglev_optimistic_peeled_loops: false,
    };
}

// Function to check NodeType.  Requires correct v8::internal::NodeType enum.
fn NodeTypeIs(a: NodeType, b: NodeType) -> bool {
    // This is a placeholder and needs proper implementation based on v8's type system.
    a == b
}

fn NodeTypeIsSmi(node_type: NodeType) -> bool {
    node_type == NodeType::kSmi
}

// Example implementation, requires actual implementations from v8 crate.
fn StaticTypeForNode(_broker: &dyn JSHeapBroker, _isolate: &dyn LocalIsolate, _node: &dyn ValueNode) -> NodeType {
    // Needs a proper implementation based on node info and static types.
    NodeType::kUnknown
}

fn IntersectType(a: NodeType, b: NodeType) -> NodeType {
    if a == b {
        a
    } else {
        NodeType::kUnknown
    }
}

// Placeholder for StringLength node. Requires actual implementations from v8 crate.
pub trait StringLengthNode: ValueNode {}

// Placeholder for UnsafeSmiTagInt32 node. Requires actual implementations from v8 crate.
pub trait UnsafeSmiTagInt32Node: ValueNode {}

// Placeholder for Int32ToNumber node. Requires actual implementations from v8 crate.
pub trait Int32ToNumberNode: ValueNode {}

// Placeholder for Phi node. Requires actual implementations from v8 crate.
pub trait PhiNode: ValueNode {}

impl dyn ValueNode {
    fn TryCast<T: ValueNode>(&self) -> Option<&T> {
        // This is a placeholder and needs proper implementation for casting.
        None
    }

    fn Is<T: ValueNode>(&self) -> bool {
        // This is a placeholder and needs proper implementation for checking.
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AvailableExpression {
    node: *const dyn ValueNode,
    effect_epoch: usize,
}

const kEffectEpochOverflow: usize = usize::MAX;
const kEffectEpochForPureInstructions: usize = 0;

impl AvailableExpression {
    pub fn new(node: *const dyn ValueNode, effect_epoch: usize) -> Self {
        AvailableExpression { node, effect_epoch }
    }
}

#[derive(Clone, Debug)]
pub struct NodeInfo {
    node_type: NodeType,
    possible_maps: Option<HashSet<usize>>, // usize represents map id
    any_map_is_unstable: bool,
    tagged_alternative: Option<*mut dyn ValueNode>, // Example: a pointer to a Tagged version of the node.
}

impl NodeInfo {
    pub fn new(node_type: NodeType) -> Self {
        NodeInfo {
            node_type,
            possible_maps: None,
            any_map_is_unstable: false,
            tagged_alternative: None,
        }
    }

    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

    pub fn possible_maps_are_known(&self) -> bool {
        self.possible_maps.is_some()
    }

    pub fn possible_maps(&self) -> &HashSet<usize> {
        self.possible_maps.as_ref().unwrap()
    }

    pub fn any_map_is_unstable(&self) -> bool {
        self.any_map_is_unstable
    }

    pub fn alternative(&self) -> &AlternativeNodes {
        unimplemented!();
    }

    fn no_info_available(&self) -> bool {
        self.node_type() == NodeType::kUnknown && !self.possible_maps_are_known()
    }

    fn MergeWith(&mut self, rhs: &Self, _zone: &Zone, any_merged_map_is_unstable: &mut bool) {
        // Placeholder implementation.  Needs detailed logic for merging NodeInfo.
        if self.node_type != rhs.node_type {
            self.node_type = NodeType::kUnknown;
        }
    }

    fn IntersectType(&mut self, unmerged_type: NodeType) {
        if self.node_type != unmerged_type {
            self.node_type = NodeType::kUnknown;
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlternativeNodes {
    tagged: Option<*mut dyn ValueNode>,
}

impl AlternativeNodes {
    pub fn tagged(&self) -> Option<*mut dyn ValueNode> {
        self.tagged
    }
}

impl NodeInfo {
    // This is a helper struct for clear unstable maps. It's C++ NodeInfo::ClearUnstableMapsOnCopy
    pub struct ClearUnstableMapsOnCopy {
        node_info: NodeInfo,
    }
}

impl From<NodeInfo::ClearUnstableMapsOnCopy> for NodeInfo {
    fn from(clear_unstable_maps_on_copy: NodeInfo::ClearUnstableMapsOnCopy) -> Self {
        let mut node_info = clear_unstable_maps_on_copy.node_info;
        node_info.any_map_is_unstable = false;
        node_info.possible_maps = None;
        node_info
    }
}

impl<'a> From<&'a NodeInfo> for NodeInfo::ClearUnstableMapsOnCopy {
    fn from(node_info: &'a NodeInfo) -> Self {
        NodeInfo::ClearUnstableMapsOnCopy {
            node_info: node_info.clone(),
        }
    }
}

impl<'a> From<&'a NodeInfo> for NodeInfo {
    fn from(node_info: &'a NodeInfo) -> Self {
        node_info.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Alternatives {
    node_info: Option<NodeInfo>,
}

impl Alternatives {
    pub fn new(node_info: Option<&NodeInfo>) -> Self {
        Alternatives {
            node_info: node_info.map(|info| info.clone()),
        }
    }

    pub fn node_type(&self) -> NodeType {
        self.node_info.as_ref().map_or(NodeType::kUnknown, |info| info.node_type())
    }
}

impl Alternatives {
    pub struct List {
        alternatives: Vec<Alternatives>,
    }

    impl List {
        pub fn new() -> Self {
            List {
                alternatives: Vec::new(),
            }
        }

        pub fn Add(&mut self, alternatives: &Alternatives) {
            self.alternatives.push(alternatives.clone());
        }

        pub fn first(&self) -> Option<&Alternatives> {
            self.alternatives.first()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, Alternatives> {
            self.alternatives.iter()
        }

        #[cfg(test)]
        pub fn LengthForTest(&self) -> usize {
            self.alternatives.len()
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnownNodeAspects {
    node_infos: HashMap<*const dyn ValueNode, NodeInfo>,
    available_expressions: HashSet<AvailableExpression>,
    loaded_constant_properties: HashMap<*const dyn ValueNode, HashMap<*const dyn ValueNode, *const dyn ValueNode>>,
    loaded_properties: HashMap<*const dyn ValueNode, HashMap<*const dyn ValueNode, *const dyn ValueNode>>,
    loaded_context_constants: HashMap<usize, *const dyn ValueNode>,
    loaded_context_slots: HashMap<usize, *const dyn ValueNode>,
    may_have_aliasing_contexts_: ContextSlotLoadsAlias,
    effect_epoch_: usize,
    any_map_for_any_node_is_unstable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextSlotLoadsAlias {
    None,
    Yes,
}

impl KnownNodeAspects {
    pub fn new(zone: &Zone) -> Self {
        KnownNodeAspects {
            node_infos: HashMap::new(),
            available_expressions: HashSet::new(),
            loaded_constant_properties: HashMap::new(),
            loaded_properties: HashMap::new(),
            loaded_context_constants: HashMap::new(),
            loaded_context_slots: HashMap::new(),
            may_have_aliasing_contexts_: ContextSlotLoadsAlias::None,
            effect_epoch_: 0,
            any_map_for_any_node_is_unstable: false,
        }
    }

    pub fn Merge(&mut self, other: &KnownNodeAspects, zone: &Zone) {
        let mut any_merged_map_is_unstable = false;

        // DestructivelyIntersect for node_infos
        let mut to_remove = Vec::new();
        for (&lhs_node, lhs) in self.node_infos.iter_mut() {
            if let Some(rhs) = other.node_infos.get(&lhs_node) {
                let mut lhs_clone = lhs.clone();
                lhs_clone.MergeWith(rhs, zone, &mut any_merged_map_is_unstable);
                if lhs_clone.no_info_available() {
                    to_remove.push(lhs_node);
                } else {
                    *lhs = lhs_clone;
                }
            } else {
                to_remove.push(lhs_node);
            }
        }
        for node in to_remove {
            self.node_infos.remove(&node);
        }

        if self.effect_epoch_ != other.effect_epoch_ {
            self.effect_epoch_ = cmp::max(self.effect_epoch_, other.effect_epoch_) + 1;
        }

        // DestructivelyIntersect for available_expressions
        self.available_expressions.retain(|lhs| {
            other.available_expressions.contains(lhs) && lhs.effect_epoch >= self.effect_epoch_
        });

        self.any_map_for_any_node_is_unstable = any_merged_map_is_unstable;

        // DestructivelyIntersect for loaded_constant_properties
        let mut to_remove_loaded_constant_properties = Vec::new();
        for (&lhs_key, lhs) in self.loaded_constant_properties.iter_mut() {
            if let Some(rhs) = other.loaded_constant_properties.get(&lhs_key) {
                lhs.retain(|k, _| rhs.contains_key(k));
                if lhs.is_empty() {
                    to_remove_loaded_constant_properties.push(lhs_key);
                }
            } else {
                to_remove_loaded_constant_properties.push(lhs_key);
            }
        }
        for key in to_remove_loaded_constant_properties {
            self.loaded_constant_properties.remove(&key);
        }

        // DestructivelyIntersect for loaded_properties
        let mut to_remove_loaded_properties = Vec::new();
        for (&lhs_key, lhs) in self.loaded_properties.iter_mut() {
            if let Some(rhs) = other.loaded_properties.get(&lhs_key) {
                lhs.retain(|k, _| rhs.contains_key(k));
                if lhs.is_empty() {
                    to_remove_loaded_properties.push(lhs_key);
                }
            } else {
                to_remove_loaded_properties.push(lhs_key);
            }
        }
        for key in to_remove_loaded_properties {
            self.loaded_properties.remove(&key);
        }

        // DestructivelyIntersect for loaded_context_constants
        self.loaded_context_constants.retain(|k, _| other.loaded_context_constants.contains_key(k));

        // DestructivelyIntersect for loaded_context_slots
        self.loaded_context_slots.retain(|k, _| other.loaded_context_slots.contains_key(k));

        if self.may_have_aliasing_contexts() != other.may_have_aliasing_contexts() {
            if self.may_have_aliasing_contexts() == ContextSlotLoadsAlias::None {
                self.may_have_aliasing_contexts_ = other.may_have_aliasing_contexts_;
            } else if other.may_have_aliasing_contexts() != ContextSlotLoadsAlias::None {
                self.may_have_aliasing_contexts_ = ContextSlotLoadsAlias::Yes;
            }
        }
    }

    fn FindInfo(&self, node: *const dyn ValueNode) -> Option<std::collections::hash_map::Iter<'_, *const dyn ValueNode, NodeInfo>> {
        if self.node_infos.contains_key(&node) {
            Some(self.node_infos.iter())
        } else {
            None
        }
    }

    fn IsValid(&self, _it: Option<std::collections::hash_map::Iter<'_, *const dyn ValueNode, NodeInfo>>) -> bool {
        true
    }

    pub fn NodeTypeFor(&self, node: *const dyn ValueNode) -> NodeType {
        if let Some(node_info) = self.node_infos.get(&node) {
            node_info.node_type()
        } else {
            NodeType::kUnknown
        }
    }

    pub fn TryGetInfoFor(&self, node: *const dyn ValueNode) -> Option<&NodeInfo> {
        self.node_infos.get(&node)
    }

    pub fn Clone(
        &self,
        zone: &Zone,
    ) -> Box<KnownNodeAspects> {
        let mut cloned = KnownNodeAspects::new(zone);
        cloned.node_infos = self.node_infos.clone();
        cloned.available_expressions = self.available_expressions.clone();
        cloned.loaded_constant_properties = self.loaded_constant_properties.clone();
        cloned.loaded_properties = self.loaded_properties.clone();
        cloned.loaded_context_constants = self.loaded_context_constants.clone();
        cloned.loaded_context_slots = self.loaded_context_slots.clone();
        cloned.may_have_aliasing_contexts_ = self.may_have_aliasing_contexts_;
        cloned.effect_epoch_ = self.effect_epoch_;
        cloned.any_map_for_any_node_is_unstable = self.any_map_for_any_node_is_unstable;
        Box::new(cloned)
    }

    pub fn CloneForLoopHeader(
        &self,
        optimistic: bool,
        loop_effects: &LoopEffects,
        zone: &Zone,
    ) -> Box<KnownNodeAspects> {
        Box::new(KnownNodeAspects::new_for_loop_header(self, optimistic, loop_effects, zone))
    }

    fn new_for_loop_header(
        other: &KnownNodeAspects,
        optimistic_initial_state: bool,
        loop_effects: &LoopEffects,
        zone: &Zone,
    ) -> Self {
        let mut new = KnownNodeAspects {
            any_map_for_any_node_is_unstable: false,
            loaded_constant_properties: other.loaded_constant_properties.clone(),
            loaded_properties: HashMap::new(),
            loaded_context_constants: other.loaded_context_constants.clone(),
            loaded_context_slots: HashMap::new(),
            available_expressions: HashSet::new(),
            may_have_aliasing_contexts_: ContextSlotLoadsAlias::None,
            effect_epoch_: other.effect_epoch_,
            node_infos: HashMap::new(),
        };

        if !other.any_map_for_any_node_is_unstable {
            new.node_infos = other.node_infos.clone();
            #[cfg(debug_assertions)]
            for it in new.node_infos.values() {
                DCHECK!(!it.any_map_is_unstable());
            }
        } else if optimistic_initial_state && !loop_effects.unstable_aspects_cleared {
            new.node_infos = other.node_infos.clone();
            new.any_map_for_any_node_is_unstable = other.any_map_for_any_node_is_unstable;
        } else {
            for (key, it) in &other.node_infos {
                new.node_infos.insert(*key, NodeInfo::from(NodeInfo::ClearUnstableMapsOnCopy::from(it)));
            }
        }

        if optimistic_initial_state && !loop_effects.unstable_aspects_cleared {
            // IMPORTANT: Whatever we clone here needs to be checked for consistency
            // in when we try to terminate the loop in `IsCompatibleWithLoopHeader`.
            if loop_effects.objects_written.is_empty() && loop_effects.keys_cleared.is_empty() {
                new.loaded_properties = other.loaded_properties.clone();
            } else {
                let mut cleared_key = loop_effects.keys_cleared.iter();
                let cleared_keys_end = &mut loop_effects.keys_cleared.end();
                let mut cleared_obj = loop_effects.objects_written.iter();
                let cleared_objs_end = &mut loop_effects.objects_written.end();
                for (loaded_key, value) in &other.loaded_properties {
                    if next_in_ignore_list(&mut cleared_key, cleared_keys_end, loaded_key) {
                        continue;
                    }
                    let mut props_for_key: HashMap<*const dyn ValueNode, *const dyn ValueNode> = HashMap::new();
                    for (loaded_obj_key, loaded_obj_value) in value {
                        if !next_in_ignore_list(&mut cleared_obj, cleared_objs_end, loaded_obj_key) {
                            props_for_key.insert(*loaded_obj_key, *loaded_obj_value);
                        }
                    }
                    new.loaded_properties.insert(*loaded_key, props_for_key);
                }
            }
            if loop_effects.context_slot_written.is_empty() {
                new.loaded_context_slots = other.loaded_context_slots.clone();
            } else {
                let mut slot_written = loop_effects.context_slot_written.iter();
                let slot_written_end = &mut loop_effects.context_slot_written.end();
                for (loaded_key, loaded_value) in &other.loaded_context_slots {
                    if !next_in_ignore_list(&mut slot_written, slot_written_end, loaded_key) {
                        new.loaded_context_slots.insert(*loaded_key, *loaded_value);
                    }
                }
            }
            if !new.loaded_context_slots.is_empty() {
                if loop_effects.may_have_aliasing_contexts {
                    new.may_have_aliasing_contexts_ = ContextSlotLoadsAlias::Yes;
                } else {
                    new.may_have_aliasing_contexts_ = other.may_have_aliasing_contexts();
                }
            }
        }

        // To account for the back-jump we must not allow effects to be reshuffled
        // across loop headers.
        // TODO(olivf): Only do this if the loop contains write effects.
        new.increment_effect_epoch();
        for e in other.available_expressions.iter() {
            if e.effect_epoch >= new.effect_epoch() {
                new.available_expressions.insert(*e);
            }
        }

        new
    }

    pub fn ClearUnstableNodeAspects(&mut self) {
        if v8_flags.trace_maglev_graph_building {
            println!("  ! Clearing unstable node aspects");
        }
        self.ClearUnstableMaps();
        // Side-effects can change object contents, so we have to clear
        // our known loaded properties -- however, constant properties are known
        // to not change (and we added a dependency on this), so we don't have to
        // clear those.
        self.loaded_properties.clear();
        self.loaded_context_slots.clear();
        self.may_have_aliasing_contexts_ = KnownNodeAspects::ContextSlotLoadsAlias::None;
    }

    pub fn ClearUnstableMaps(&mut self) {
        self.any_map_for_any_node_is_unstable = true;

        for entry in self.node_infos.values_mut() {
            entry.any_map_is_unstable = true;
        }

        self.node_infos.retain(|_, v| !v.any_map_is_unstable());
    }

    pub fn may_have_aliasing_contexts(&self) -> ContextSlotLoadsAlias {
        self.may_have_aliasing_contexts_
    }

    pub fn effect_epoch(&self) -> usize {
        self.effect_epoch_
    }

    pub fn increment_effect_epoch(&mut self) {
        self.effect_epoch_ += 1;
    }

    pub fn IsCompatibleWithLoopHeader(&self, loop_header: &KnownNodeAspects) -> bool {
        // Needs to be in sync with `CloneForLoopHeader(zone, true)`.

        // Analysis state can change with loads.
        if !loop_header.loaded_context_slots.is_empty()
            && loop_header.may_have_aliasing_contexts() != ContextSlotLoadsAlias::Yes
            && loop_header.may_have_aliasing_contexts() != self.may_have_aliasing_contexts()
            && self.may_have_aliasing_contexts() != ContextSlotLoadsAlias::None
        {
            if v8_flags.trace_maglev_loop_speeling {
                println!("KNA after loop has incompatible loop_header.may_have_aliasing_contexts");
            }
            return false;
        }

        let had_effects = self.effect_epoch() != loop_header.effect_epoch();

        if !had_effects {
            if !aspect_includes(&loop_header.node_infos, &self.node_infos, NodeInfoTypeIs, NodeInfoIsEmpty) {
                if v8_flags.trace_maglev_loop_speeling {
                    println!("KNA after effectless loop has incompatible node_infos");
                }
                return false;
            }
            // In debug builds we do a full comparison to ensure that without an effect
            // epoch change all unstable properties still hold.
            #[cfg(not(debug_assertions))]
            return true;
        }

        if !aspect_includes(&loop_header.node_infos, &self.node_infos, NodeInfoIncludes, NodeInfoIsEmpty) {
            if v8_flags.trace_maglev_loop_speeling {
                println!("KNA after loop has incompatible node_infos");
            }
            DCHECK!(had_effects);
            return false;
        }

        if !maybe_empty_aspect_includes(&loop_header.loaded_properties, &self.loaded_properties,
                                          |a, b| aspect_includes(a, b, SameValue, None::<fn(_)->bool>)) {
            if v8_flags.trace_maglev_loop_speeling {
                println!("KNA after loop has incompatible loaded_properties");
            }
            DCHECK!(had_effects);
            return false;
        }

        if !maybe_null_aspect_includes(&loop_header.loaded_context_slots, &self.loaded_context_slots, SameValue) {
            if v8_flags.trace_maglev_loop_speeling {
                println!("KNA after loop has incompatible loaded_context_slots");
            }
            DCHECK!(had_effects);
            return false;
        }

        true
    }
}

fn next_in_ignore_list<T: Eq + Ord, I>(ignore: &mut I, ignore_end: &mut dyn std::iter::Iterator<Item = &T>, cur: &T) -> bool where
    I: Iterator<Item = &T>
{
    if let Some(next_ignore) = ignore.next() {
        while next_ignore < cur {
            if let None = ignore.next() {
                return false;
            }
        }
        return next_ignore == cur;
    }
    false
}

// Takes two ordered maps and ensures that every element in `as` is
//  * also present in `bs` and
//  * `Compare(a, b)` holds for each value.
fn aspect_includes<As, Bs, CompareFunction, IsEmptyFunction>(
    as_: &As,
    bs: &Bs,
    compare: CompareFunction,
    is_empty:
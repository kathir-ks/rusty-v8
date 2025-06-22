// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/csa-load-elimination.h
// This file would define the public interface for the CsaLoadElimination module.
// Since we are translating directly to Rust, this part is integrated into the implementation.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::{Hash, Hasher},
};

// Placeholder for external crates and definitions.  Replace with actual crate imports.
// mod common;
// mod node_matchers;
// mod node_properties;
// mod simplified_operator;

// v8::internal::compiler namespace equivalent
pub mod compiler {

    use super::*;

    // Placeholder: Replace with actual flags access.
    pub mod v8_flags {
        pub static trace_turbo_load_elimination: bool = false;
    }

    pub struct CsaLoadElimination<'a> {
        jsgraph: &'a Jsgraph,
        node_states_: HashMap<NodeId, AbstractState>,
        zone_: Zone,
    }

    impl<'a> CsaLoadElimination<'a> {
        pub fn new(jsgraph: &'a Jsgraph) -> Self {
            CsaLoadElimination {
                jsgraph,
                node_states_: HashMap::new(),
                zone_: Zone::new(),
            }
        }

        fn empty_state(&self) -> &'static AbstractState {
            // Using static lifetime as the empty state is constant throughout the execution.
            static EMPTY_STATE: AbstractState = AbstractState {
                mutable_state: HalfState {
                    fresh_entries_: OuterMap::new(),
                    constant_entries_: OuterMap::new(),
                    arbitrary_entries_: OuterMap::new(),
                    fresh_unknown_entries_: UnknownOffsetInfos::new(),
                    constant_unknown_entries_: UnknownOffsetInfos::new(),
                    arbitrary_unknown_entries_: UnknownOffsetInfos::new(),
                    zone_: &Zone::empty(),
                },
                immutable_state: HalfState {
                    fresh_entries_: OuterMap::new(),
                    constant_entries_: OuterMap::new(),
                    arbitrary_entries_: OuterMap::new(),
                    fresh_unknown_entries_: UnknownOffsetInfos::new(),
                    constant_unknown_entries_: UnknownOffsetInfos::new(),
                    arbitrary_unknown_entries_: UnknownOffsetInfos::new(),
                    zone_: &Zone::empty(),
                },
            };
            &EMPTY_STATE
        }

        pub fn reduce(&mut self, node: &mut Node) -> Reduction {
            if v8_flags::trace_turbo_load_elimination {
                if node.op.effect_input_count() > 0 {
                    println!(" visit #{}:{}", node.id, node.op.mnemonic());
                    if node.op.value_input_count() > 0 {
                        print!("(");
                        for i in 0..node.op.value_input_count() {
                            if i > 0 {
                                print!(", ");
                            }
                            let value = self.get_value_input(node, i);
                            print!("#{}:{}", value.id, value.op.mnemonic());
                        }
                        println!(")");
                    }
                    println!();
                    for i in 0..node.op.effect_input_count() {
                        let effect = self.get_effect_input(node, i);
                        if let Some(state) = self.node_states_.get(&effect.id) {
                            println!("  state[{}]: #{}:{}", i, effect.id, effect.op.mnemonic());
                            state.mutable_state.print();
                            state.immutable_state.print();
                        } else {
                            println!("  no state[{}]: #{}:{}", i, effect.id, effect.op.mnemonic());
                        }
                    }
                }
            }

            match node.op.opcode {
                IrOpcode::kLoadFromObject | IrOpcode::kLoadImmutableFromObject => {
                    self.reduce_load_from_object(node, ObjectAccessOf { machine_type: node.op.machine_type })
                }
                IrOpcode::kStoreToObject | IrOpcode::kInitializeImmutableInObject => {
                    self.reduce_store_to_object(node, ObjectAccessOf { machine_type: node.op.machine_type })
                }
                IrOpcode::kDebugBreak | IrOpcode::kAbortCSADcheck => {
                    // Avoid changing optimizations in the presence of debug instructions.
                    self.propagate_input_state(node)
                }
                IrOpcode::kCall => self.reduce_call(node),
                IrOpcode::kEffectPhi => self.reduce_effect_phi(node),
                IrOpcode::kDead => Reduction::NoChange,
                IrOpcode::kStart => self.reduce_start(node),
                _ => self.reduce_other_node(node),
            }
        }

        fn reduce_load_from_object(
            &mut self,
            node: &mut Node,
            access: ObjectAccessOf,
        ) -> Reduction {
            debug_assert!(
                node.op.opcode == IrOpcode::kLoadFromObject
                    || node.op.opcode == IrOpcode::kLoadImmutableFromObject
            );
            let object = self.get_value_input(node, 0);
            let offset = self.get_value_input(node, 1);
            let effect = self.get_effect_input(node, 0); // Using 0, assuming there is one effect input
            let state = self.node_states_.get(&effect.id);
            if state.is_none() {
                return Reduction::NoChange;
            }

            let state = state.unwrap();
            let is_mutable = node.op.opcode == IrOpcode::kLoadFromObject;

            if !(if is_mutable {
                &state.immutable_state
            } else {
                &state.mutable_state
            })
            .lookup(object, offset)
            .is_empty()
            {
                let control = self.get_control_input(node);
                let unreachable = Node::new(
                    self.jsgraph.common().unreachable(),
                    effect.id,
                    control.id,
                    access.machine_type
                ); // Placeholders
                let rep = access.machine_type.representation();
                let dead_value = Node::new(self.jsgraph.common().dead_value(rep), unreachable.id, unreachable.id, access.machine_type);
                self.replace_with_value(node, dead_value, unreachable, control);
                return Reduction::Replace(dead_value);
            }

            let half_state = if is_mutable {
                &state.mutable_state
            } else {
                &state.immutable_state
            };

            let representation = access.machine_type.representation();
            let lookup_result = half_state.lookup(object, offset);

            if !lookup_result.is_empty() {
                let from = lookup_result.representation;
                if Helpers::subsumes(from, representation) && !lookup_result.value.is_dead() {
                    let replacement = self.truncate_and_extend(
                        &lookup_result.value,
                        from,
                        access.machine_type,
                    );
                    self.replace_with_value(node, replacement, effect.clone());
                    self.revisit(object);
                    return Reduction::Replace(replacement);
                }
            }

            let half_state = half_state.add_field(object, offset, node, representation);

            let new_state = if is_mutable {
                AbstractState::new(half_state.clone(), state.immutable_state.clone(), self.zone())
            } else {
                AbstractState::new(state.mutable_state.clone(), half_state.clone(), self.zone())
            };

            self.update_state(node, &new_state)
        }

        fn reduce_store_to_object(
            &mut self,
            node: &mut Node,
            access: ObjectAccessOf,
        ) -> Reduction {
            debug_assert!(
                node.op.opcode == IrOpcode::kStoreToObject
                    || node.op.opcode == IrOpcode::kInitializeImmutableInObject
            );
            let object = self.get_value_input(node, 0);
            let offset = self.get_value_input(node, 1);
            let value = self.get_value_input(node, 2);
            let effect = self.get_effect_input(node, 0);
            let state = self.node_states_.get(&effect.id);
            if state.is_none() {
                return Reduction::NoChange;
            }

            let state = state.unwrap();

            let repr = access.machine_type.representation();

            if node.op.opcode == IrOpcode::kStoreToObject {
                if !state.immutable_state.lookup(object, offset).is_empty() {
                    return self.assert_unreachable(node);
                }
                let mutable_state = state.mutable_state.kill_field(object, offset, repr);
                let mutable_state = mutable_state.add_field(object, offset, value, repr);
                let new_state =
                    AbstractState::new(mutable_state.clone(), state.immutable_state.clone(), self.zone());
                return self.update_state(node, &new_state);
            } else {
                if !state.mutable_state.lookup(object, offset).is_empty() {
                    return self.assert_unreachable(node);
                }
                debug_assert!(state.immutable_state.lookup(object, offset).is_empty());
                let immutable_state = state.immutable_state.add_field(object, offset, value, repr);
                let new_state =
                    AbstractState::new(state.mutable_state.clone(), immutable_state.clone(), self.zone());
                return self.update_state(node, &new_state);
            }
        }

        fn reduce_effect_phi(&mut self, node: &mut Node) -> Reduction {
            let effect0 = self.get_effect_input(node, 0);
            let control = self.get_control_input(node);

            let state0 = self.node_states_.get(&effect0.id);
            if state0.is_none() {
                return Reduction::NoChange;
            }
            let state0 = state0.unwrap();

            if control.op.opcode == IrOpcode::kLoop {
                let state = self.compute_loop_state(node, state0);
                return self.update_state(node, state);
            }

            debug_assert_eq!(control.op.opcode, IrOpcode::kMerge);

            let input_count = node.op.effect_input_count();
            for i in 1..input_count {
                let effect = self.get_effect_input(node, i);
                if self.node_states_.get(&effect.id).is_none() {
                    return Reduction::NoChange;
                }
            }

            let mut state = AbstractState::new(state0.mutable_state.clone(), state0.immutable_state.clone(), self.zone());
            for i in 1..input_count {
                let input = self.get_effect_input(node, i);
                let input_state = self.node_states_.get(&input.id).unwrap();
                state.intersect_with(input_state);
            }
            self.update_state(node, &state)
        }

        fn reduce_start(&mut self, node: &mut Node) -> Reduction {
            self.update_state(node, self.empty_state())
        }

        fn reduce_call(&mut self, node: &mut Node) -> Reduction {
            let value = self.get_value_input(node, 0);
            if let Some(_ext_ref) = ExternalReferenceMatcher::match_external_reference(value) {
                return self.propagate_input_state(node);
            }
            self.reduce_other_node(node)
        }

        fn reduce_other_node(&mut self, node: &mut Node) -> Reduction {
            if node.op.effect_input_count() == 1 && node.op.effect_output_count() == 1 {
                let effect = self.get_effect_input(node, 0);
                let state = self.node_states_.get(&effect.id);

                if state.is_none() {
                    return Reduction::NoChange;
                }
                let state = state.unwrap();

                let new_state = if node.op.has_property(OperatorProperties::kNoWrite) {
                    state
                } else {
                    AbstractState::new(HalfState::new(self.zone()), state.immutable_state.clone(), self.zone())
                };

                return self.update_state(node, new_state);
            }
            debug_assert_eq!(0, node.op.effect_output_count());
            Reduction::NoChange
        }

        fn update_state(&mut self, node: &mut Node, state: &AbstractState) -> Reduction {
            let original = self.node_states_.get(&node.id);
            if original.is_none() || !state.equals(original.unwrap()) {
                self.node_states_.insert(node.id, state.clone());
                return Reduction::Changed(node.id);
            }
            Reduction::NoChange
        }

        fn propagate_input_state(&mut self, node: &mut Node) -> Reduction {
            let effect = self.get_effect_input(node, 0);
            let state = self.node_states_.get(&effect.id);

            if state.is_none() {
                return Reduction::NoChange;
            }

            self.update_state(node, state.unwrap())
        }

        fn assert_unreachable(&mut self, node: &mut Node) -> Reduction {
            let effect = self.get_effect_input(node, 0);
            let control = self.get_control_input(node);

            let unreachable = Node::new(self.jsgraph.common().unreachable(), effect.id, control.id, node.op.machine_type);
            Reduction::Replace(unreachable)
        }

        fn compute_loop_state(&self, node: &mut Node, state: &AbstractState) -> &AbstractState {
            debug_assert_eq!(node.op.opcode, IrOpcode::kEffectPhi);

            let mut queue: VecDeque<Node> = VecDeque::new();
            let mut visited: HashSet<NodeId> = HashSet::new();
            visited.insert(node.id);

            for i in 1..(node.inputs.len() - 1) {
                let input_node = &node.inputs[i];
                let input = self.jsgraph.graph.nodes.get(&input_node).unwrap();
                queue.push_back(input.clone());
            }

            let mut current_state = state.clone(); // Start with a copy of the incoming state.

            while let Some(mut current) = queue.pop_front() {
                if visited.insert(current.id) {
                    match current.op.opcode {
                        IrOpcode::kStoreToObject => {
                            let object = self.get_value_input(&mut current, 0);
                            let offset = self.get_value_input(&mut current, 1);
                            let repr = current.op.machine_type.representation();
                            let new_mutable_state = current_state.mutable_state.kill_field(object, offset, repr);
                            current_state = AbstractState::new(new_mutable_state.clone(), current_state.immutable_state.clone(), self.zone());
                        }
                        IrOpcode::kInitializeImmutableInObject => {
                            // Placeholder.  Add code for Initializing Immutable in object,
                            //let object = self.get_value_input(&mut current, 0);
                            //let offset = self.get_value_input(&mut current, 1);
                            //debug_assert!(current_state.immutable_state.lookup(object, offset).is_empty());
                        }
                        _ => {
                            if !current.op.has_property(OperatorProperties::kNoWrite) {
                                return AbstractState::new(HalfState::new(self.zone()), current_state.immutable_state.clone(), self.zone());
                            }
                        }
                    }

                    for i in 0..current.op.effect_input_count() {
                        let effect_input = self.get_effect_input(&mut current, i);
                        let node = self.jsgraph.graph.nodes.get(&effect_input.id).unwrap().clone(); //Clone the effect input
                        queue.push_back(node);
                    }
                }
            }

            self.zone().alloc(current_state);
            self.node_states_.entry(node.id).or_insert(current_state)
        }

        fn truncate_and_extend(&self, node: &Node, from: MachineRepresentation, to: MachineType) -> Node {
            debug_assert!(Helpers::subsumes(from, to.representation()));
            debug_assert!(ElementSizeInBytes(from) >= ElementSizeInBytes(to.representation()));

            if to == MachineType::Int8() || to == MachineType::Int16() {
                debug_assert_eq!(to.semantic(), MachineSemantic::kInt32);
                let mut truncated_node = node.clone();
                if from == MachineRepresentation::kWord64 {
                    truncated_node = Node::new(self.machine().truncate_int64to_int32(), truncated_node.id, truncated_node.id, truncated_node.op.machine_type);
                }
                let shift = 32 - 8 * ElementSizeInBytes(to.representation());
                let shifted_node = Node::new(self.machine().word32_shl(), truncated_node.id, self.jsgraph.int32_constant(shift).id, truncated_node.op.machine_type);

                return Node::new(
                    self.machine().word32_sar(),
                    shifted_node.id,
                    self.jsgraph.int32_constant(shift).id,
                    to,
                );
            } else if to == MachineType::Uint8() || to == MachineType::Uint16() {
                let mut truncated_node = node.clone();
                if from == MachineRepresentation::kWord64 {
                    truncated_node = Node::new(self.machine().truncate_int64to_int32(), truncated_node.id, truncated_node.id, truncated_node.op.machine_type);
                }

                let mask = (1 << 8 * ElementSizeInBytes(to.representation())) - 1;
                return Node::new(self.machine().word32_and(), truncated_node.id, self.jsgraph.int32_constant(mask).id, to);
            } else if from == MachineRepresentation::kWord64 && to.representation() == MachineRepresentation::kWord32 {
                return Node::new(self.machine().truncate_int64to_int32(), node.id, node.id, to);
            } else {
                debug_assert!(
                    (from == to.representation() && (from == MachineRepresentation::kWord32 || from == MachineRepresentation::kWord64 || !IsIntegral(from)))
                        || (IsAnyTagged(from) && IsAnyTagged(to.representation()))
                );
                return node.clone();
            }
        }

        fn get_value_input(&self, node: &Node, index: usize) -> &Node {
            let input_id = node.inputs[index];
            self.jsgraph.graph.nodes.get(&input_id).expect("Input node should exist")
        }

        fn get_effect_input(&self, node: &Node, index: usize) -> &Node {
             let input_id = node.effect_inputs[index];
            self.jsgraph.graph.nodes.get(&input_id).expect("Effect input node should exist")
        }

        fn get_control_input(&self, node: &Node) -> &Node {
             let input_id = node.control_inputs[0];
            self.jsgraph.graph.nodes.get(&input_id).expect("Control input node should exist")
        }

        fn common(&self) -> &CommonOperatorBuilder {
            self.jsgraph.common()
        }

        fn machine(&self) -> &MachineOperatorBuilder {
            self.jsgraph.machine()
        }

        fn graph(&self) -> &TFGraph {
            self.jsgraph.graph()
        }

        fn zone(&self) -> &Zone {
            &self.zone_
        }

        fn isolate(&self) -> &Isolate {
            self.jsgraph.isolate()
        }

        fn replace_with_value(&mut self, node: &mut Node, replacement: Node, effect: &Node) {
            // Update node references in other nodes
            for other_node in self.jsgraph.graph.nodes.values_mut() {
                for input_id in other_node.inputs.iter_mut() {
                    if *input_id == node.id {
                        *input_id = replacement.id;
                    }
                }
                for input_id in other_node.effect_inputs.iter_mut() {
                    if *input_id == node.id {
                        *input_id = replacement.id;
                    }
                }
            }

            // Update the effect input of the replacement node, if it exists
            //replacement.effect_inputs[0] = effect.id;

            // Add the replacement node to the graph
            self.jsgraph.graph.nodes.insert(replacement.id, replacement);

            // Remove the original node from the graph
            self.jsgraph.graph.nodes.remove(&node.id);
        }

        fn replace_with_value(&mut self, node: &mut Node, replacement: Node, effect: Node, control: Node) {
            // Update node references in other nodes
            for other_node in self.jsgraph.graph.nodes.values_mut() {
                for input_id in other_node.inputs.iter_mut() {
                    if *input_id == node.id {
                        *input_id = replacement.id;
                    }
                }
                for input_id in other_node.effect_inputs.iter_mut() {
                    if *input_id == node.id {
                        *input_id = replacement.id;
                    }
                }
            }

            // Update the effect input of the replacement node, if it exists
           // replacement.effect_inputs[0] = effect.id;
           // replacement.control_inputs[0] = control.id;

            // Add the replacement node to the graph
            self.jsgraph.graph.nodes.insert(replacement.id, replacement);

            // Remove the original node from the graph
            self.jsgraph.graph.nodes.remove(&node.id);
        }

        fn revisit(&mut self, _node: &Node) {
            // Implement Revisit logic, which involves potentially re-enqueueing
            // nodes for further analysis.  Currently a no-op.
            // TODO: Implement Revisit functionality.
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Reduction {
        Changed(NodeId),
        NoChange,
        Replace(Node),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FieldInfo {
        pub value: Node,
        pub representation: MachineRepresentation,
    }

    impl FieldInfo {
        pub fn new(value: Node, representation: MachineRepresentation) -> Self {
            FieldInfo {
                value,
                representation,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.representation == MachineRepresentation::kNone
        }
    }

    impl Default for FieldInfo {
        fn default() -> Self {
            FieldInfo {
                value: Node::default(),
                representation: MachineRepresentation::kNone,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HalfState {
        fresh_entries_: OuterMap<u32>,
        constant_entries_: OuterMap<u32>,
        arbitrary_entries_: OuterMap<u32>,
        fresh_unknown_entries_: UnknownOffsetInfos,
        constant_unknown_entries_: UnknownOffsetInfos,
        arbitrary_unknown_entries_: UnknownOffsetInfos,
        zone_: &'static Zone,
    }

    impl HalfState {
        pub fn new(zone: &Zone) -> Self {
            HalfState {
                fresh_entries_: OuterMap::new(),
                constant_entries_: OuterMap::new(),
                arbitrary_entries_: OuterMap::new(),
                fresh_unknown_entries_: UnknownOffsetInfos::new(),
                constant_unknown_entries_: UnknownOffsetInfos::new(),
                arbitrary_unknown_entries_: UnknownOffsetInfos::new(),
                zone_: zone,
            }
        }

        pub fn intersect_with(&mut self, that: &HalfState) {
            Self::intersect_with_outer_map(&mut self.fresh_entries_, &that.fresh_entries_);
            Self::intersect_with_outer_map(&mut self.constant_entries_, &that.constant_entries_);
            Self::intersect_with_outer_map(&mut self.arbitrary_entries_, &that.arbitrary_entries_);
            Self::intersect_with_unknown_offset_infos(&mut self.fresh_unknown_entries_, &that.fresh_unknown_entries_);
            Self::intersect_with_unknown_offset_infos(&mut self.constant_unknown_entries_, &that.constant_unknown_entries_);
            Self::intersect_with_unknown_offset_infos(&mut self.arbitrary_unknown_entries_, &that.arbitrary_unknown_entries_);
        }

        fn intersect_with_outer_map<OuterKey: Eq + Hash + Copy>(
            to: &mut OuterMap<OuterKey>,
            from: &OuterMap<OuterKey>,
        ) {
            let empty_info = FieldInfo::default();
            for (key, to_map) in to.entries.iter_mut() {
                if let Some(from_map) = from.entries.get(key) {
                    let to_map_copy = to_map.clone();
                    for (node_ptr, info) in to_map.entries.iter() {
                        if from_map.entries.get(node_ptr) != Some(info) {
                            to_map_copy.entries.insert(*node_ptr, empty_info.clone());
                        }
                    }
                    *to_map = to_map_copy;
                } else {
                    to.entries.remove(key);
                }
            }
        }

        fn intersect_with_unknown_offset_infos(
            to: &mut UnknownOffsetInfos,
            from: &UnknownOffsetInfos,
        ) {
            for (key, to_map) in to.entries.iter_mut() {
                if let Some(from_map) = from.entries.get(key) {
                    let to_map_copy = to_map.clone();
                    for (node_ptr, info) in to_map.entries.iter() {
                        if from_map.entries.get(node_ptr) != Some(info) {
                            to_map_copy.entries.insert(*node_ptr, FieldInfo::default());
                        }
                    }
                    *to_map = to_map_copy;
                } else {
                    to.entries.remove(key);
                }
            }
        }

        pub fn kill_field(
            &self,
            object: &Node,
            offset: &Node,
            repr: MachineRepresentation,
        ) -> Self {
            let mut result = self.clone();

            if let Some(offset_value) = IntPtrMatcher::resolve_value(offset) {
                let num_offset = offset_value as u32;
                if Helpers::is_fresh_object(object) {
                    result.kill_offset_in_fresh(object, num_offset, repr);
                    result.kill_offset(&mut result.arbitrary_entries_, num_offset, repr);
                    result.fresh_unknown_entries_.entries.insert(object.clone(), InnerMap::new());
                    result.arbitrary_unknown_entries_.entries.clear();
                } else if Helpers::is_constant_object(object) {
                    result.kill_offset(&mut result.constant_entries_, num_offset, repr);
                    result.kill_offset(&mut result.arbitrary_entries_, num_offset, repr);
                    result.constant_unknown_entries_.entries.clear();
                    result.arbitrary_unknown_entries_.entries.clear();
                } else {
                    result.kill_offset(&mut result.fresh_entries_, num_offset, repr);
                    result.kill_offset(&mut result.constant_entries_, num_offset, repr);
                    result.kill_offset(&mut result.arbitrary_entries_, num_offset, repr);
                    result.fresh_unknown_entries_.entries.clear();
                    result.constant_unknown_entries_.entries.clear();
                    result.arbitrary_unknown_entries_.entries.clear();
                }
            } else {
                if Helpers::is_fresh_object(object) {
                    for map in result.fresh_entries_.entries.values_mut() {
                        map.entries.insert(object.clone(), FieldInfo::default());
                    }
                    result.fresh_unknown_entries_.entries.insert(object.clone(), InnerMap::new());
                    result.arbitrary_entries_.entries.clear();
                    result.arbitrary_unknown_entries_.entries.clear();
                } else if Helpers::is_constant_object(object) {
                    result.constant_entries_.entries.clear();
                    result.constant_unknown_entries_.entries.clear();
                    result.arbitrary_entries_.entries.clear();
                    result.arbitrary_unknown_entries_.entries.clear();
                } else {
                    return HalfState::new(self.zone_);
                }
            }
            result
        }

        pub fn add_field(
            &self,
            object: &Node,
            offset: &Node,
            value: &Node,
            repr: MachineRepresentation,
        ) -> Self {
            let mut new_state = self.clone();

            if let Some(offset_num) = IntPtrMatcher::resolve_value(offset) {
                let offset_num = offset_num as u32;
                let infos = if Helpers::is_fresh_object(object) {
                    &mut new_state.fresh_entries_
                } else if Helpers::is_constant_object(object) {
                    &mut new_state.constant_entries_
                } else {
                    &mut new_state.arbitrary_entries_
                };
                Self::update_outer_map(infos, offset_num, object.clone(), FieldInfo::new(value.clone(), repr));
            } else {
                let infos = if Helpers::is_fresh_object(object) {
                    &mut new_state.fresh_unknown_entries_
                } else if Helpers::is_constant_object(object) {
                    &mut new_state.constant_unknown_entries_
                } else {
                    &mut new_state.arbitrary_unknown_entries_
                };
                Self::update_unknown_offset_infos(infos, object.clone(), offset.clone(), FieldInfo::new(value.clone(), repr));
            }
            new_state
        }

        pub fn lookup(&self, object: &Node, offset: &Node) -> FieldInfo {
            if let Some(num_offset) = IntPtrMatcher::resolve_value(offset) {
                let num_offset = num_offset as u32;
                let infos = if Helpers::is_fresh_object(object) {
                    &self.fresh_entries_
                } else if Helpers::is_constant_object(object) {
                    &self.constant_entries_
                } else {
                    &self.arbitrary_entries_
                };
                return infos.get(num_offset, object);
            } else {
                let infos = if Helpers::is_fresh_object(object) {
                    &self.fresh_unknown_entries_
                } else if Helpers::is_constant_object(object) {
                    &self.constant_unknown_entries_
                } else {
                    &self.arbitrary_unknown_entries_
                };
                return infos.get(object, offset);
            }
        }

        fn update_outer_map<OuterKey: Eq + Hash + Copy>(
            infos: &mut OuterMap<OuterKey>,
            outer_key: OuterKey,
            object: Node,
            field_info: FieldInfo,
        ) {
            let inner_map = infos.entries.entry(outer_key).or_insert(InnerMap::new());
            inner_map.entries.insert(object, field_info);
        }

        fn update_unknown_offset_infos(
            infos: &mut UnknownOffsetInfos,
            object: Node,
            offset: Node,
            field_info: FieldInfo,
        ) {
            let inner_map = infos.entries.entry(object).or_insert(InnerMap::new());
            inner_map.entries.insert(offset, field_info);
        }

        fn kill_offset(&mut self, infos: &mut OuterMap<u32>, offset: u32, repr: MachineRepresentation) {
            for i in 0..ElementSizeInBytes(repr) {
                infos.entries.remove(&(offset + i));
            }

            let initial_offset = if offset >= kMaximumReprSizeInBytes as u32 - 1 {
                offset - (kMaximumReprSizeInBytes as u32 - 1)
            } else {
                0
            };

            for i in initial_offset..offset {
                if let Some(map) = infos.entries.get_mut(&i) {
                    let mut map_copy = map.clone();
                    for (node, info) in map.entries.iter() {
                        if info.representation != MachineRepresentation::kNone
                            && ElementSizeInBytes(info.representation) as u32 > offset - i
                        {
                            map_copy.entries.remove(node);
                        
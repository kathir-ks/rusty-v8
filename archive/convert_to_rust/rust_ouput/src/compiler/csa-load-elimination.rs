// Converted from V8 C++ source files:
// Header: csa-load-elimination.h
// Implementation: csa-load-elimination.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod base;
mod codegen;
mod compiler;
mod execution;

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::Mutex;

use crate::base::Flags;
use crate::codegen::machine_type::MachineRepresentation;
use crate::compiler::graph_reducer::AdvancedReducer;
use crate::compiler::graph_reducer::Reduction;
use crate::compiler::graph_reducer::V8;
use crate::compiler::js_graph::JSGraph;
use crate::compiler::node_aux_data::NodeAuxData;
use crate::compiler::operator::Operator;
use crate::compiler::turbofan_types::v8;
use crate::execution::isolate::Isolate;

// Forward declarations.
struct CommonOperatorBuilder {}
struct ObjectAccess {}
struct TFGraph {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MachineType {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Representation {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Smi {}
#[derive(Clone, Debug)]
pub struct Handle<T> {
    _phantom: std::marker::PhantomData<T>,
}
#[derive(Clone, Debug)]
pub struct MaybeIndirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}
#[derive(Clone, Debug)]
pub struct OptimizedCompilationInfo {}
#[derive(Clone, Debug)]
pub struct Code {}
#[derive(Clone, Debug)]
pub struct HeapObject {}
#[derive(Clone, Debug)]
pub struct String_ExternalOneByteStringResource {}
#[derive(Clone, Debug)]
pub struct Local<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}
#[derive(Clone, Debug)]
pub struct Value {}
#[derive(Clone, Debug)]
pub struct WasmMemoryMapDescriptor {}

#[derive(Clone, Debug)]
pub struct JsFunction {}
#[derive(Clone, Debug)]
pub struct Debug {}

pub struct CsaLoadElimination {
    editor: Box<dyn Editor>,
    empty_state_: AbstractState,
    node_states_: NodeAuxData<AbstractState>,
    jsgraph_: *mut JSGraph,
    zone_: *mut Zone,
}

impl CsaLoadElimination {
    pub fn new(editor: Box<dyn Editor>, jsgraph: *mut JSGraph, zone: *mut Zone) -> Self {
        let graph = unsafe { (*jsgraph).graph };
        let node_count = unsafe { (*graph).NodeCount() };

        CsaLoadElimination {
            editor,
            empty_state_: AbstractState::new(unsafe { &mut *zone }),
            node_states_: NodeAuxData::new(node_count, unsafe { &mut *zone }),
            jsgraph_,
            zone_,
        }
    }

    fn reducer_name(&self) -> &'static str {
        "CsaLoadElimination"
    }

    fn reduce(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if v8_flags.trace_turbo_load_elimination {
                if (*node).op.EffectInputCount() > 0 {
                    println!(" visit #{}:{}", (*node).id, (*node).op.mnemonic);
                    if (*node).op.ValueInputCount() > 0 {
                        print!("(");
                        for i in 0..(*node).op.ValueInputCount() {
                            if i > 0 {
                                print!(", ");
                            }
                            let value = (*node).inputs[i as usize];
                            println!("#{}:{}", (*value).id, (*value).op.mnemonic);
                        }
                        println!(")");
                    }
                    println!("");
                    for i in 0..(*node).op.EffectInputCount() {
                        let effect = (*node).effects[i as usize];
                        if let Some(state) = self.node_states_.get(effect) {
                            println!(
                                "  state[{}]: #{}:{}",
                                i, (*effect).id, (*effect).op.mnemonic
                            );
                            state.mutable_state.print();
                            state.immutable_state.print();
                        } else {
                            println!(
                                "  no state[{}]: #{}:{}",
                                i, (*effect).id, (*effect).op.mnemonic
                            );
                        }
                    }
                }
            }
            match (*node).opcode {
                IrOpcode::kLoadFromObject | IrOpcode::kLoadImmutableFromObject => {
                    self.reduce_load_from_object(node, ObjectAccessOf((*node).op))
                }
                IrOpcode::kStoreToObject | IrOpcode::kInitializeImmutableInObject => {
                    self.reduce_store_to_object(node, ObjectAccessOf((*node).op))
                }
                IrOpcode::kDebugBreak | IrOpcode::kAbortCSADcheck => {
                    self.propagate_input_state(node)
                }
                IrOpcode::kCall => self.reduce_call(node),
                IrOpcode::kEffectPhi => self.reduce_effect_phi(node),
                IrOpcode::kDead => Reduction::no_change(),
                IrOpcode::kStart => self.reduce_start(node),
                _ => self.reduce_other_node(node),
            }
        }
    }

    fn reduce_load_from_object(&mut self, node: *mut Node, access: ObjectAccess) -> Reduction {
        unsafe {
            let object = (*node).inputs[0];
            let offset = (*node).inputs[1];
            let effect = (*node).effects[0];

            let state = self.node_states_.get(effect);
            if state.is_none() {
                return Reduction::no_change();
            }
            let state = state.unwrap();

            let is_mutable = (*node).opcode == IrOpcode::kLoadFromObject;

            if !(if is_mutable {
                &state.immutable_state
            } else {
                &state.mutable_state
            })
            .lookup(object, offset)
            .is_empty()
            {
                let control = (*node).control;
                let unreachable = (*(*self.jsgraph_).common).Unreachable(effect, control);
                let rep = ObjectAccessOf((*node).op).machine_type.representation();
                let dead_value = (*(*self.jsgraph_).common).DeadValue(rep, unreachable);
                self.replace_with_value(node, dead_value, unreachable, control);
                (*node).Kill();
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
                if CsaLoadEliminationHelpers::subsumes(from, representation)
                    && !(*lookup_result.value).IsDead()
                {
                    let replacement = self.truncate_and_extend(
                        lookup_result.value,
                        from,
                        access.machine_type,
                    );
                    self.replace_with_value(node, replacement, effect);
                    self.revisit(object);
                    return Reduction::Replace(replacement);
                }
            }

            let half_state =
                half_state.add_field(object, offset, node, representation, &mut *self.zone_);

            let new_state = if is_mutable {
                let new_state = AbstractState {
                    mutable_state: half_state.clone(),
                    immutable_state: state.immutable_state.clone(),
                };
                self.zone()
                    .allocate(new_state)
            } else {
                let new_state = AbstractState {
                    mutable_state: state.mutable_state.clone(),
                    immutable_state: half_state.clone(),
                };
                self.zone()
                    .allocate(new_state)
            };

            self.update_state(node, new_state)
        }
    }

    fn reduce_store_to_object(&mut self, node: *mut Node, access: ObjectAccess) -> Reduction {
        unsafe {
            let object = (*node).inputs[0];
            let offset = (*node).inputs[1];
            let value = (*node).inputs[2];
            let effect = (*node).effects[0];

            let state = self.node_states_.get(effect);
            if state.is_none() {
                return Reduction::no_change();
            }
            let state = state.unwrap();

            let repr = access.machine_type.representation();

            if (*node).opcode == IrOpcode::kStoreToObject {
                if !state.immutable_state.lookup(object, offset).is_empty() {
                    return self.assert_unreachable(node);
                }

                let mutable_state = state.mutable_state.kill_field(object, offset, repr, &mut *self.zone_);
                let mutable_state =
                    mutable_state.add_field(object, offset, value, repr, &mut *self.zone_);

                let new_state = AbstractState {
                    mutable_state: mutable_state.clone(),
                    immutable_state: state.immutable_state.clone(),
                };
                let new_state_ptr = self.zone().allocate(new_state);
                return self.update_state(node, new_state_ptr);
            } else {
                if !state.mutable_state.lookup(object, offset).is_empty() {
                    return self.assert_unreachable(node);
                }

                assert!(state.immutable_state.lookup(object, offset).is_empty());

                let immutable_state =
                    state.immutable_state.add_field(object, offset, value, repr, &mut *self.zone_);
                let new_state = AbstractState {
                    mutable_state: state.mutable_state.clone(),
                    immutable_state: immutable_state.clone(),
                };
                let new_state_ptr = self.zone().allocate(new_state);
                return self.update_state(node, new_state_ptr);
            }
        }
    }

    fn reduce_effect_phi(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            let effect0 = (*node).inputs[0];
            let control = (*node).control;

            let state0 = self.node_states_.get(effect0);
            if state0.is_none() {
                return Reduction::no_change();
            }
            let state0 = state0.unwrap();

            if (*control).opcode == IrOpcode::kLoop {
                let state = self.compute_loop_state(node, state0);
                return self.update_state(node, state);
            }

            assert_eq!((*control).opcode, IrOpcode::kMerge);

            let input_count = (*node).op.EffectInputCount();
            for i in 1..input_count {
                let effect = (*node).effects[i as usize];
                if self.node_states_.get(effect).is_none() {
                    return Reduction::no_change();
                }
            }

            let mut state = state0.clone();
            for i in 1..input_count {
                let input = (*node).effects[i as usize];
                let input_state = self.node_states_.get(input).unwrap();
                state.intersect_with(input_state);
            }

            let state_ptr = self.zone().allocate(state);
            return self.update_state(node, state_ptr);
        }
    }

    fn reduce_start(&mut self, node: *mut Node) -> Reduction {
        self.update_state(node, self.empty_state())
    }

    fn reduce_call(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            let value = (*node).inputs[0];
            let m = ExternalReferenceMatcher { value };
            if m.is(ExternalReference::check_object_type()) {
                return self.propagate_input_state(node);
            }
            self.reduce_other_node(node)
        }
    }

    fn reduce_other_node(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if (*node).op.EffectInputCount() == 1 && (*node).op.EffectOutputCount() == 1 {
                let effect = (*node).effects[0];
                let state = self.node_states_.get(effect);

                if state.is_none() {
                    return Reduction::no_change();
                }
                let state = state.unwrap();

                let new_state = if (*node).op.HasProperty(Operator::kNoWrite) {
                    state
                } else {
                    let new_state = AbstractState {
                        mutable_state: HalfState::new(self.zone()),
                        immutable_state: state.immutable_state.clone(),
                    };
                    self.zone()
                        .allocate(new_state)
                };
                return self.update_state(node, new_state);
            }
            assert_eq!((*node).op.EffectOutputCount(), 0);
            Reduction::no_change()
        }
    }

    fn update_state(&mut self, node: *mut Node, state: *const AbstractState) -> Reduction {
        unsafe {
            let original = self.node_states_.get(node);

            if original.is_none() || !(*state).equals(original.unwrap()) {
                self.node_states_.set(node, state);
                return Reduction::changed(node);
            }

            Reduction::no_change()
        }
    }

    fn propagate_input_state(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            let effect = (*node).effects[0];
            let state = self.node_states_.get(effect);
            if state.is_none() {
                return Reduction::no_change();
            }
            let state = state.unwrap();
            self.update_state(node, state)
        }
    }

    fn compute_loop_state(
        &mut self,
        node: *mut Node,
        state: *const AbstractState,
    ) -> *const AbstractState {
        unsafe {
            assert_eq!((*node).opcode, IrOpcode::kEffectPhi);

            let mut queue: VecDeque<*mut Node> = VecDeque::new();
            let mut visited: HashSet<*mut Node> = HashSet::new();
            visited.insert(node);

            for i in 1..(*node).InputCount() - 1 {
                queue.push_back((*node).inputs[i as usize]);
            }

            while let Some(current) = queue.pop_front() {
                if visited.insert(current) {
                    if (*current).opcode == IrOpcode::kStoreToObject {
                        let object = (*current).inputs[0];
                        let offset = (*current).inputs[1];
                        let repr = ObjectAccessOf((*current).op).machine_type.representation();
                        let new_mutable_state =
                            (*state).mutable_state.kill_field(object, offset, repr, &mut *self.zone_);
                        let abstract_state = AbstractState {
                            mutable_state: new_mutable_state.clone(),
                            immutable_state: (*state).immutable_state.clone(),
                        };
                        let new_state = self.zone().allocate(abstract_state);
                        return new_state;
                    } else if (*current).opcode == IrOpcode::kInitializeImmutableInObject {
                        // We are not allowed to reset an immutable (object, offset) pair.
                        let object = (*current).inputs[0];
                        let offset = (*current).inputs[1];
                        assert!((*state).immutable_state.lookup(object, offset).is_empty());
                    } else if !(*current).op.HasProperty(Operator::kNoWrite) {
                        let new_state = AbstractState {
                            mutable_state: HalfState::new(self.zone()),
                            immutable_state: (*state).immutable_state.clone(),
                        };
                        return self.zone().allocate(new_state);
                    }

                    for i in 0..(*current).op.EffectInputCount() {
                        queue.push_back((*current).effects[i as usize]);
                    }
                }
            }

            state
        }
    }

    fn truncate_and_extend(
        &mut self,
        node: *mut Node,
        from: MachineRepresentation,
        to: MachineType,
    ) -> *mut Node {
        unsafe {
            assert!(CsaLoadEliminationHelpers::subsumes(from, to.representation()));
            assert!(ElementSizeInBytes(from) >= ElementSizeInBytes(to.representation()));

            if to == MachineType::Int8() || to == MachineType::Int16() {
                assert_eq!(to.semantic(), MachineSemantic::kInt32);
                let mut node = node;
                if from == MachineRepresentation::kWord64 {
                    node = (*(*self.jsgraph_).machine).TruncateInt64ToInt32(node);
                }
                let shift = 32 - 8 * ElementSizeInBytes(to.representation()) as i32;
                let shift_node = (*self.jsgraph_).Int32Constant(shift);
                let shl_node = (*(*self.jsgraph_).machine).Word32Shl(node, shift_node);
                let sar_node = (*(*self.jsgraph_).machine).Word32Sar(shl_node, shift_node);
                sar_node
            } else if to == MachineType::Uint8() || to == MachineType::Uint16() {
                let mut node = node;
                if from == MachineRepresentation::kWord64 {
                    node = (*(*self.jsgraph_).machine).TruncateInt64ToInt32(node);
                }
                let mask = (1 << (8 * ElementSizeInBytes(to.representation()))) - 1;
                let mask_node = (*self.jsgraph_).Int32Constant(mask as i32);
                let and_node = (*(*self.jsgraph_).machine).Word32And(node, mask_node);
                and_node
            } else if from == MachineRepresentation::kWord64
                && to.representation() == MachineRepresentation::kWord32
            {
                (*(*self.jsgraph_).machine).TruncateInt64ToInt32(node)
            } else {
                assert!(
                    (from == to.representation()
                        && (from == MachineRepresentation::kWord32
                            || from == MachineRepresentation::kWord64
                            || !IsIntegral(from)))
                        || (IsAnyTagged(from) && IsAnyTagged(to.representation()))
                );
                node
            }
        }
    }

    fn assert_unreachable(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            let effect = (*node).effects[0];
            let control = (*node).control;
            let unreachable = (*(*self.jsgraph_).common).Unreachable(effect, control);
            Reduction::Replace(unreachable)
        }
    }

    fn common(&self) -> &CommonOperatorBuilder {
        unsafe { &*(*self.jsgraph_).common }
    }

    fn machine(&self) -> &MachineOperatorBuilder {
        unsafe { &*(*self.jsgraph_).machine }
    }

    fn graph(&self) -> &TFGraph {
        unsafe { &*(*self.jsgraph_).graph }
    }

    fn jsgraph(&self) -> &JSGraph {
        unsafe { &*self.jsgraph_ }
    }

    fn zone(&mut self) -> &mut Zone {
        unsafe { &mut *self.zone_ }
    }

    fn empty_state(&self) -> *const AbstractState {
        &self.empty_state_
    }

    fn replace_with_value(&mut self, node: *mut Node, replacement: *mut Node, effect: *mut Node) {
        unsafe {
            (*node).replacement = Some(replacement);
            (*node).effect = effect;
            (*node).control = (*effect).control; // Assuming control should be inherited from effect
            self.editor.replace_node(node, replacement);
        }
    }
    fn revisit(&mut self, node: *mut Node) {
        unsafe {
            self.editor.mark_as_needs_revisit(node);
        }
    }
}

impl AdvancedReducer for CsaLoadElimination {
    fn reduce(&mut self, node: *mut Node) -> Reduction {
        self.reduce(node)
    }
}

mod CsaLoadEliminationHelpers {
    use super::{ElementSizeInBytes, MachineRepresentation, Node, NodeProperties, IrOpcode};

    pub fn subsumes(from: MachineRepresentation, to: MachineRepresentation) -> bool {
        if from == to {
            return true;
        }
        if is_any_tagged(from) {
            return is_any_tagged(to);
        }
        if is_integral(from) {
            return is_integral(to) && ElementSizeInBytes(from) >= ElementSizeInBytes(to);
        }
        false
    }

    fn is_any_tagged(repr: MachineRepresentation) -> bool {
        match repr {
            MachineRepresentation::kTagged
            | MachineRepresentation::kTaggedSigned
            | MachineRepresentation::kTaggedPointer => true,
            _ => false,
        }
    }

    fn is_integral(repr: MachineRepresentation) -> bool {
        match repr {
            MachineRepresentation::kBit
            | MachineRepresentation::kWord8
            | MachineRepresentation::kWord16
            | MachineRepresentation::kWord32
            | MachineRepresentation::kWord64 => true,
            _ => false,
        }
    }

    pub fn is_constant_object(object: *mut Node) -> bool {
        unsafe {
            (*object).opcode == IrOpcode::kParameter
                || (*object).opcode == IrOpcode::kLoadImmutable
                || NodeProperties::is_constant(object)
        }
    }

    pub fn is_fresh_object(object: *mut Node) -> bool {
        unsafe { (*object).opcode == IrOpcode::kAllocate || (*object).opcode == IrOpcode::kAllocateRaw }
    }
}

#[derive(Clone)]
struct HalfState {
    zone_: *mut Zone,
    fresh_entries_: PersistentMap<u32, PersistentMap<*mut Node, FieldInfo>>,
    constant_entries_: PersistentMap<u32, PersistentMap<*mut Node, FieldInfo>>,
    arbitrary_entries_: PersistentMap<u32, PersistentMap<*mut Node, FieldInfo>>,
    fresh_unknown_entries_: PersistentMap<*mut Node, PersistentMap<*mut Node, FieldInfo>>,
    constant_unknown_entries_: PersistentMap<*mut Node, PersistentMap<*mut Node, FieldInfo>>,
    arbitrary_unknown_entries_: PersistentMap<*mut Node, PersistentMap<*mut Node, FieldInfo>>,
}

impl HalfState {
    pub fn new(zone_: *mut Zone) -> Self {
        HalfState {
            zone_,
            fresh_entries_: PersistentMap::new(),
            constant_entries_: PersistentMap::new(),
            arbitrary_entries_: PersistentMap::new(),
            fresh_unknown_entries_: PersistentMap::new(),
            constant_unknown_entries_: PersistentMap::new(),
            arbitrary_unknown_entries_: PersistentMap::new(),
        }
    }

    pub fn equals(&self, that: &HalfState) -> bool {
        self.fresh_entries_ == that.fresh_entries_
            && self.constant_entries_ == that.constant_entries_
            && self.arbitrary_entries_ == that.arbitrary_entries_
            && self.fresh_unknown_entries_ == that.fresh_unknown_entries_
            && self.constant_unknown_entries_ == that.constant_unknown_entries_
            && self.arbitrary_unknown_entries_ == that.arbitrary_unknown_entries_
    }

    fn intersect_with(&mut self, that: &HalfState) {
        Self::intersect_with_outer(
            &mut self.fresh_entries_,
            &that.fresh_entries_,
            unsafe { &mut *self.zone_ },
        );
        Self::intersect_with_outer(
            &mut self.constant_entries_,
            &that.constant_entries_,
            unsafe { &mut *self.zone_ },
        );
        Self::intersect_with_outer(
            &mut self.arbitrary_entries_,
            &that.arbitrary_entries_,
            unsafe { &mut *self.zone_ },
        );
        Self::intersect_with_outer(
            &mut self.fresh_unknown_entries_,
            &that.fresh_unknown_entries_,
            unsafe { &mut *self.zone_ },
        );
        Self::intersect_with_outer(
            &mut self.constant_unknown_entries_,
            &that.constant_unknown_entries_,
            unsafe { &mut *self.zone_ },
        );
        Self::intersect_with_outer(
            &mut self.arbitrary_unknown_entries_,
            &that.arbitrary_unknown_entries_,
            unsafe { &mut *self.zone_ },
        );
    }

    fn kill_field(&self, object: *mut Node, offset: *mut Node, repr: MachineRepresentation, zone: &mut Zone) -> Self {
        let mut result = self.clone();
        let empty_unknown = PersistentMap::<*mut Node, FieldInfo>::new();
        let m = IntPtrMatcher { offset };
        unsafe {
            if m.has_resolved_value() {
                let num_offset = m.resolved_value() as u32;
                if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    result.kill_offset_in_fresh(object, num_offset, repr);
                    Self::kill_offset(&mut result.arbitrary_entries_, num_offset, repr, zone);
                    result.fresh_unknown_entries_.set(object, PersistentMap::new());
                    result.arbitrary_unknown_entries_ = PersistentMap::new();
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    Self::kill_offset(&mut result.constant_entries_, num_offset, repr, zone);
                    Self::kill_offset(&mut result.arbitrary_entries_, num_offset, repr, zone);
                    result.constant_unknown_entries_ = PersistentMap::new();
                    result.arbitrary_unknown_entries_ = PersistentMap::new();
                } else {
                    Self::kill_offset(&mut result.fresh_entries_, num_offset, repr, zone);
                    Self::kill_offset(&mut result.constant_entries_, num_offset, repr, zone);
                    Self::kill_offset(&mut result.arbitrary_entries_, num_offset, repr, zone);
                    result.fresh_unknown_entries_ = PersistentMap::new();
                    result.constant_unknown_entries_ = PersistentMap::new();
                    result.arbitrary_unknown_entries_ = PersistentMap::new();
                }
            } else {
                let empty_constant = PersistentMap::<u32, PersistentMap<*mut Node, FieldInfo>>::new();
                if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    for (&outer_key, inner_map) in result.fresh_entries_.map.iter() {
                        let mut map_copy = inner_map.clone();
                        map_copy.set(object, FieldInfo::default());
                        result.fresh_entries_.set(outer_key, map_copy);
                    }
                    result.fresh_unknown_entries_.set(object, PersistentMap::new());
                    result.arbitrary_entries_ = PersistentMap::new();
                    result.arbitrary_unknown_entries_ = PersistentMap::new();
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    result.constant_entries_ = PersistentMap::new();
                    result.constant_unknown_entries_ = PersistentMap::new();
                    result.arbitrary_entries_ = PersistentMap::new();
                    result.arbitrary_unknown_entries_ = PersistentMap::new();
                } else {
                    return HalfState::new(self.zone_);
                }
            }
        }

        result
    }

    fn add_field(&self, object: *mut Node, offset: *mut Node, value: *mut Node, repr: MachineRepresentation, zone: &mut Zone) -> Self {
        let mut new_state = self.clone();
        let m = IntPtrMatcher { offset };

        unsafe {
            if m.has_resolved_value() {
                let offset_num = m.resolved_value() as u32;

                let infos = if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    &mut new_state.fresh_entries_
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    &mut new_state.constant_entries_
                } else {
                    &mut new_state.arbitrary_entries_
                };
                Self::update_inner(infos, offset_num, object, FieldInfo { value, representation: repr });
            } else {
                let infos = if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    &mut new_state.fresh_unknown_entries_
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    &mut new_state.constant_unknown_entries_
                } else {
                    &mut new_state.arbitrary_unknown_entries_
                };
                Self::update_inner(infos, object, offset, FieldInfo { value, representation: repr });
            }
        }
        new_state
    }

    fn lookup(&self, object: *mut Node, offset: *mut Node) -> FieldInfo {
        let m = IntPtrMatcher { offset };
        unsafe {
            if m.has_resolved_value() {
                let num_offset = m.resolved_value() as u32;

                let infos = if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    &self.fresh_entries_
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    &self.constant_entries_
                } else {
                    &self.arbitrary_entries_
                };

                infos.get(num_offset).get(object).clone()
            } else {
                let infos = if CsaLoadEliminationHelpers::is_fresh_object(object) {
                    &self.fresh_unknown_entries_
                } else if CsaLoadEliminationHelpers::is_constant_object(object) {
                    &self.constant_unknown_entries_
                } else {
                    &self.arbitrary_unknown_entries_
                };

                infos.get(object).get(offset).clone()
            }
        }
    }

    fn kill_offset(
        infos: &mut PersistentMap<u32, PersistentMap<*mut Node, FieldInfo>>,
        offset: u32,
        repr: MachineRepresentation,
        zone: &mut Zone,
    ) {
        for i in 0..ElementSizeInBytes(repr) as u32 {
            infos.set(offset + i, PersistentMap::new());
        }

        let initial_offset = if offset >= (kMaximumReprSizeInBytes - 1) as u32 {
            offset - ((kMaximumReprSizeInBytes - 1) as u32)
        } else {
            0
        };

        for i in initial_offset..offset {
            if let Some(inner_map) = infos.map.get(&i) {
                let mut map_copy = inner_map.clone();
                for (&node, info) in inner_map.map.iter() {
                    if info.representation != MachineRepresentation::kNone
                        && ElementSizeInBytes(info.representation) as u32 > offset - i
                    {
                        map_copy.set(node, FieldInfo::default());
                    }
                }
                infos.set(i, map_copy);
            }
        }
    }

    fn kill_offset_in_fresh(&mut self, object: *mut Node, offset: u32, repr: MachineRepresentation) {
        for i in 0..ElementSizeInBytes(repr) as u32 {
            Self::update_inner(&mut self.fresh_entries_, offset + i, object, FieldInfo::default());
        }
        let initial_offset = if offset >= (kMaximumReprSizeInBytes - 1) as u32 {
            offset - ((kMaximumReprSizeInBytes - 1) as u32)
        } else {
            0
        };

        for i in initial_offset..offset {
            let info = self.fresh_entries_.get(i).get(object).clone();
            if info.representation != MachineRepresentation::kNone
                && ElementSizeInBytes(info.representation) as u32 > offset - i
            {
                Self::update_inner(&mut self.fresh_entries_, i, object, FieldInfo::default());
            }
        }
    }

    fn print(&self) const {
        println!("  fresh_entries_:");
        Self::print_inner(&self.fresh_entries_);
        println!("  constant_entries_:");
        Self::print_inner(&self.constant_entries_);
        println!("  arbitrary_entries_:");
        Self::print_inner(&self.arbitrary_entries_);
        println!("  fresh_unknown_entries_:");
        Self::print_inner(&self.fresh_unknown_entries_);
        println!("  constant_unknown_entries_:");
        Self::print_inner(&self.constant_unknown_entries_);
        println!("  arbitrary_unknown_entries_:");
        Self::print_inner(&self.arbitrary_unknown_

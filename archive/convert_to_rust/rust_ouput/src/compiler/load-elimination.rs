// Converted from V8 C++ source files:
// Header: load-elimination.h
// Implementation: load-elimination.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::{f32, f64};

use crate::base::compiler_specific::*;
use crate::codegen::machine_type::*;
use crate::common::globals::*;
use crate::compiler::access_builder::*;
use crate::compiler::common_operator::*;
use crate::compiler::graph_reducer::*;
use crate::compiler::js_graph::*;
use crate::compiler::js_heap_broker::*;
use crate::compiler::node::*;
use crate::compiler::node_properties::*;
use crate::compiler::simplified_operator::*;
use crate::compiler::turbofan::*;
use crate::handles::maybe_handles::*;
use crate::heap::factory::*;
use crate::objects::objects_inl::*;
use crate::zone::zone::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LoadEliminationError {
    LookupFailed,
    MergeConflict,
    UnsupportedOperator,
    IncompatibleRepresentation,
    IllegalDoubleConstStore,
}

pub struct LoadElimination<'a> {
    editor: &'a mut Editor<'a>,
    broker_: &'a mut JSHeapBroker,
    node_states_: AbstractStateForEffectNodes,
    jsgraph_: &'a mut JSGraph,
}

impl<'a> LoadElimination<'a> {
    pub fn new(
        editor: &'a mut Editor<'a>,
        broker_: &'a mut JSHeapBroker,
        jsgraph: &'a mut JSGraph,
        zone: &'a Zone,
    ) -> Self {
        LoadElimination {
            editor,
            broker_: broker_,
            node_states_: AbstractStateForEffectNodes::new(zone),
            jsgraph_: jsgraph,
        }
    }

    pub fn reducer_name(&self) -> &'static str {
        "LoadElimination"
    }

    pub fn reduce(&mut self, node: &mut Node) -> Reduction {
        if v8_flags.trace_turbo_load_elimination {
            if node.op().effect_input_count() > 0 {
                print!(" visit #{}:{}", node.id(), node.op().mnemonic());
                if node.op().value_input_count() > 0 {
                    print!("(");
                    for i in 0..node.op().value_input_count() {
                        if i > 0 {
                            print!(", ");
                        }
                        let value = NodeProperties::get_value_input(node, i as i32);
                        print!("#{}:{}", value.id(), value.op().mnemonic());
                    }
                    print!(")");
                }
                println!();
                for i in 0..node.op().effect_input_count() {
                    let effect = NodeProperties::get_effect_input(node, i as i32);
                    if let Some(state) = self.node_states_.get(effect) {
                        println!(
                            "  state[{}]: #{}:{}",
                            i,
                            effect.id(),
                            effect.op().mnemonic()
                        );
                        state.print();
                    } else {
                        println!(
                            "  no state[{}]: #{}:{}",
                            i,
                            effect.id(),
                            effect.op().mnemonic()
                        );
                    }
                }
            }
        }

        match node.opcode() {
            IrOpcode::kMapGuard => self.reduce_map_guard(node),
            IrOpcode::kCheckMaps => self.reduce_check_maps(node),
            IrOpcode::kCompareMaps => self.reduce_compare_maps(node),
            IrOpcode::kEnsureWritableFastElements => self.reduce_ensure_writable_fast_elements(node),
            IrOpcode::kMaybeGrowFastElements => self.reduce_maybe_grow_fast_elements(node),
            IrOpcode::kTransitionElementsKind => self.reduce_transition_elements_kind(node),
            IrOpcode::kTransitionElementsKindOrCheckMap => {
                self.reduce_transition_elements_kind_or_check_map(node)
            }
            IrOpcode::kLoadField => self.reduce_load_field(node, FieldAccessOf(node.op())),
            IrOpcode::kStoreField => self.reduce_store_field(node, FieldAccessOf(node.op())),
            IrOpcode::kLoadElement => self.reduce_load_element(node),
            IrOpcode::kStoreElement => self.reduce_store_element(node),
            IrOpcode::kTransitionAndStoreElement => self.reduce_transition_and_store_element(node),
            IrOpcode::kStoreTypedElement => self.reduce_store_typed_element(node),
            IrOpcode::kEffectPhi => self.reduce_effect_phi(node),
            IrOpcode::kDead => Reduction::NoChange,
            IrOpcode::kStart => self.reduce_start(node),
            _ => self.reduce_other_node(node),
        }
    }

    fn reduce_check_maps(&mut self, node: &mut Node) -> Reduction {
        let maps = CheckMapsParametersOf(node.op()).maps();
        let object = NodeProperties::get_value_input(node, 0);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    if maps.contains(&object_maps) {
                        return self.replace(effect);
                    }
                }
                let state = state.set_maps(object, maps.clone(), self.zone());
                self.update_state(node, state)
            }
        }
    }

    fn reduce_compare_maps(&mut self, node: &mut Node) -> Reduction {
        let maps = CompareMapsParametersOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    if maps.contains(&object_maps) {
                        let value = self.jsgraph_.true_constant();
                        self.replace_with_value(node, value, effect);
                        return self.replace(value);
                    }
                }
                self.update_state(node, state)
            }
        }
    }

    fn reduce_map_guard(&mut self, node: &mut Node) -> Reduction {
        let maps = MapGuardMapsOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    if maps.contains(&object_maps) {
                        return self.replace(effect);
                    }
                }
                let state = state.set_maps(object, maps.clone(), self.zone());
                self.update_state(node, state)
            }
        }
    }
    fn reduce_ensure_writable_fast_elements(&mut self, node: &mut Node) -> Reduction {
        let object = NodeProperties::get_value_input(node, 0);
        let elements = NodeProperties::get_value_input(node, 1);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut elements_maps = ZoneRefSet::<Map>::new(self.zone());
                let fixed_array_maps = ZoneRefSet::<Map>::new_from_slice(
                    self.broker_.fixed_array_map(),
                );
                if state.lookup_maps(elements, &mut elements_maps)
                    && fixed_array_maps.contains(&elements_maps)
                {
                    self.replace_with_value(node, elements, effect);
                    return self.replace(elements);
                }
                let state = state.set_maps(node, fixed_array_maps, self.zone());
                let state = state.kill_field(
                    object,
                    FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                    MaybeHandle::<Name>::empty(),
                    self.zone(),
                );
                let state = state.add_field(
                    object,
                    FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                    FieldInfo {
                        value: node,
                        representation: MachineRepresentation::kTaggedPointer,
                        name: MaybeHandle::<Name>::empty(),
                        const_field_info: ConstFieldInfo::None(),
                    },
                    self.zone(),
                );
                self.update_state(node, state)
            }
        }
    }

    fn reduce_maybe_grow_fast_elements(&mut self, node: &mut Node) -> Reduction {
        let params = GrowFastElementsParametersOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let state = if params.mode() == GrowFastElementsMode::kDoubleElements {
                    let fixed_double_array_map = ZoneRefSet::<Map>::new_from_slice(
                        self.broker_.fixed_double_array_map(),
                    );
                    state.set_maps(node, fixed_double_array_map, self.zone())
                } else {
                    let fixed_array_maps = ZoneRefSet::<Map>::new_from_slice(&[
                        self.broker_.fixed_array_map(),
                        self.broker_.fixed_cow_array_map(),
                    ]);
                    state.set_maps(node, fixed_array_maps, self.zone())
                };
                let state = state.kill_field(
                    object,
                    FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                    MaybeHandle::<Name>::empty(),
                    self.zone(),
                );
                let state = state.add_field(
                    object,
                    FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                    FieldInfo {
                        value: node,
                        representation: MachineRepresentation::kTaggedPointer,
                        name: MaybeHandle::<Name>::empty(),
                        const_field_info: ConstFieldInfo::None(),
                    },
                    self.zone(),
                );
                self.update_state(node, state)
            }
        }
    }

    fn reduce_transition_elements_kind(&mut self, node: &mut Node) -> Reduction {
        let transition = ElementsTransitionOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let source_map = MapRef(transition.source());
        let target_map = MapRef(transition.target());
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let state = match transition.mode() {
                    ElementsTransition::kFastTransition => state,
                    ElementsTransition::kSlowTransition => state.kill_field(
                        object,
                        FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                        MaybeHandle::<Name>::empty(),
                        self.zone(),
                    ),
                };
                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    if ZoneRefSet::<Map>::new_from_slice(target_map).contains(&object_maps) {
                        return self.replace(effect);
                    }
                    if object_maps.contains(&ZoneRefSet::<Map>::new_from_slice(source_map)) {
                        object_maps.remove(source_map, self.zone());
                        object_maps.insert(target_map, self.zone());
                        let alias_info = AliasStateInfo::new(state, object, Some(source_map));
                        let state = state.kill_maps(&alias_info, self.zone());
                        let state = state.set_maps(object, object_maps, self.zone());
                        return self.update_state(node, state);
                    }
                }
                let alias_info = AliasStateInfo::new(state, object, Some(source_map));
                let state = state.kill_maps(&alias_info, self.zone());
                self.update_state(node, state)
            }
        }
    }

    fn reduce_transition_elements_kind_or_check_map(&mut self, node: &mut Node) -> Reduction {
        let transition = ElementsTransitionWithMultipleSourcesOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let source_maps = transition.sources();
        let target_map = MapRef(transition.target());
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut state = state;
                for source_map in source_maps.iter() {
                    if !IsSimpleMapChangeTransition(
                        source_map.elements_kind(),
                        target_map.elements_kind(),
                    ) {
                        let alias_info = AliasStateInfo::new(state, object, Some(*source_map));
                        state = state.kill_field(
                            &alias_info,
                            FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                            MaybeHandle::<Name>::empty(),
                            self.zone(),
                        );
                    }
                }

                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    if ZoneRefSet::<Map>::new_from_slice(target_map).contains(&object_maps) {
                        return self.replace(effect);
                    }
                    for source_map in source_maps.iter() {
                        if object_maps.contains(&ZoneRefSet::<Map>::new_from_slice(*source_map)) {
                            object_maps.remove(*source_map, self.zone());
                            object_maps.insert(target_map, self.zone());
                            let alias_info = AliasStateInfo::new(state, object, Some(*source_map));
                            state = state.kill_maps(&alias_info, self.zone());
                            state = state.set_maps(object, object_maps, self.zone());
                        }
                    }
                } else {
                    for source_map in source_maps.iter() {
                        let alias_info = AliasStateInfo::new(state, object, Some(*source_map));
                        state = state.kill_maps(&alias_info, self.zone());
                    }
                }
                let state = state.set_maps(object, ZoneRefSet::<Map>::new_from_slice(target_map), self.zone());
                self.update_state(node, state)
            }
        }
    }

    fn reduce_load_field(&mut self, node: &mut Node, access: FieldAccess) -> Reduction {
        let object = NodeProperties::get_value_input(node, 0);
        let effect = NodeProperties::get_effect_input(node);
        let control = NodeProperties::get_control_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                if access.offset == HeapObject::kMapOffset && access.base_is_tagged == kTaggedBase {
                    if let MachineRepresentation::kTagged
                    | MachineRepresentation::kTaggedSigned
                    | MachineRepresentation::kTaggedPointer =
                        access.machine_type.representation()
                    {
                        let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                        if state.lookup_maps(object, &mut object_maps) && object_maps.size() == 1 {
                            let value = self
                                .jsgraph_
                                .heap_constant_no_hole(object_maps.at(0).object());
                            NodeProperties::set_type(value, Type::other_internal());
                            self.replace_with_value(node, value, effect);
                            return self.replace(value);
                        }
                    }
                } else {
                    let field_index = FieldIndexOf(access);
                    if field_index != IndexRange::invalid() {
                        let representation = access.machine_type.representation();
                        let lookup_result =
                            state.lookup_field(object, field_index, access.const_field_info);

                        let lookup_result = if lookup_result.is_none()
                            && access.const_field_info.is_const()
                        {
                            state.lookup_field(object, field_index, ConstFieldInfo::None())
                        } else {
                            lookup_result
                        };

                        if let Some(lookup_result) = lookup_result {
                            let replacement = lookup_result.value;
                            if is_compatible(representation, lookup_result.representation)
                                && !replacement.is_dead()
                            {
                                let replacement_type = NodeProperties::get_type(replacement);
                                let node_type = NodeProperties::get_type(node);
                                if !replacement_type.is(node_type) {
                                    let replacement_type =
                                        Type::intersect(node_type, replacement_type, self.graph().zone());
                                    let type_guard = self
                                        .common()
                                        .type_guard(replacement_type);
                                    let guarded_replacement = Node::new(
                                        self.graph().zone(),
                                        type_guard,
                                        &[replacement, effect, control],
                                        false,
                                    );
                                    NodeProperties::set_type(
                                        &guarded_replacement,
                                        replacement_type,
                                    );
                                    self.replace_with_value(
                                        node,
                                        &guarded_replacement,
                                        effect,
                                    );
                                    return self.replace(&guarded_replacement);
                                }
                                self.replace_with_value(node, replacement, effect);
                                return self.replace(replacement);
                            }
                        }

                        let info = FieldInfo {
                            value: node,
                            representation,
                            name: access.name,
                            const_field_info: access.const_field_info,
                        };
                        let state = state.add_field(object, field_index, info, self.zone());
                        return self.update_state(node, state);
                    }
                }
                if let Some(map) = access.map {
                    let state = state.set_maps(node, ZoneRefSet::<Map>::new_from_slice(map), self.zone());
                    return self.update_state(node, state);
                }
                self.update_state(node, state)
            }
        }
    }

    fn reduce_store_field(&mut self, node: &mut Node, access: FieldAccess) -> Reduction {
        let object = NodeProperties::get_value_input(node, 0);
        let new_value = NodeProperties::get_value_input(node, 1);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);

        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut state = state;
                if access.offset == HeapObject::kMapOffset && access.base_is_tagged == kTaggedBase
                {
                    state = state.kill_maps(object, self.zone());
                    let new_value_type = NodeProperties::get_type(new_value);
                    if new_value_type.is_heap_constant() {
                        let object_maps = ZoneRefSet::<Map>::new_from_slice(
                            new_value_type.as_heap_constant().ref_().as_map(),
                        );
                        state = state.set_maps(object, object_maps, self.zone());
                    }
                } else {
                    let field_index = FieldIndexOf(access);
                    if field_index != IndexRange::invalid() {
                        let is_const_store = access.const_field_info.is_const();
                        let representation = access.machine_type.representation();
                        let lookup_result =
                            state.lookup_field(object, field_index, access.const_field_info);

                        if lookup_result.is_some()
                            && (!is_const_store || V8_ENABLE_DOUBLE_CONST_STORE_CHECK_BOOL)
                        {
                            let lookup_result = lookup_result.unwrap();
                            let incompatible_representation =
                                !lookup_result.name.is_null()
                                    && !is_compatible(representation, lookup_result.representation);
                            let illegal_double_const_store =
                                is_const_store && !access.is_store_in_literal;

                            if incompatible_representation || illegal_double_const_store {
                                let control = NodeProperties::get_control_input(node);
                                let unreachable = Node::new(
                                    self.graph().zone(),
                                    self.common().unreachable(),
                                    &[effect, control],
                                    false,
                                );
                                return self.replace(&unreachable);
                            }

                            if lookup_result.value == new_value {
                                return self.replace(effect);
                            }
                        }

                        let mut new_info = FieldInfo {
                            value: new_value,
                            representation,
                            name: access.name,
                            const_field_info: access.const_field_info,
                        };

                        if is_const_store && access.is_store_in_literal {
                            state = state.kill_const_field(object, field_index, self.zone());
                        }

                        state = state.kill_field(object, field_index, access.name, self.zone());
                        state = state.add_field(object, field_index, new_info, self.zone());

                        if is_const_store {
                            new_info.const_field_info = ConstFieldInfo::None();
                            state = state.add_field(object, field_index, new_info, self.zone());
                        }
                    } else {
                        state = state.kill_fields(object, access.name, self.zone());
                    }
                }
                self.update_state(node, state)
            }
        }
    }
    fn reduce_load_element(&mut self, node: &mut Node) -> Reduction {
        let object = NodeProperties::get_value_input(node, 0);
        let index = NodeProperties::get_value_input(node, 1);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let access = ElementAccessOf(node.op());
                match access.machine_type.representation() {
                    MachineRepresentation::kNone
                    | MachineRepresentation::kBit
                    | MachineRepresentation::kWord8
                    | MachineRepresentation::kWord16
                    | MachineRepresentation::kWord32
                    | MachineRepresentation::kWord64
                    | MachineRepresentation::kFloat16RawBits
                    | MachineRepresentation::kFloat16
                    | MachineRepresentation::kFloat32
                    | MachineRepresentation::kCompressedPointer
                    | MachineRepresentation::kCompressed
                    | MachineRepresentation::kProtectedPointer
                    | MachineRepresentation::kIndirectPointer
                    | MachineRepresentation::kSandboxedPointer => {
                        Reduction::NoChange
                    }
                    MachineRepresentation::kFloat64
                    | MachineRepresentation::kSimd128
                    | MachineRepresentation::kSimd256
                    | MachineRepresentation::kTaggedSigned
                    | MachineRepresentation::kTaggedPointer
                    | MachineRepresentation::kTagged
                    | MachineRepresentation::kMapWord => {
                        if let Some(replacement) = state.lookup_element(
                            object,
                            index,
                            access.machine_type.representation(),
                        ) {
                            if !replacement.is_dead()
                                && NodeProperties::get_type(replacement)
                                    .is(NodeProperties::get_type(node))
                            {
                                self.replace_with_value(node, replacement, effect);
                                return self.replace(replacement);
                            }
                        }
                        let state = state.add_element(
                            object,
                            index,
                            node,
                            access.machine_type.representation(),
                            self.zone(),
                        );
                        self.update_state(node, state)
                    }
                }
            }
        }
    }

    fn reduce_store_element(&mut self, node: &mut Node) -> Reduction {
        let access = ElementAccessOf(node.op());
        let object = NodeProperties::get_value_input(node, 0);
        let index = NodeProperties::get_value_input(node, 1);
        let new_value = NodeProperties::get_value_input(node, 2);
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                if let Some(old_value) =
                    state.lookup_element(object, index, access.machine_type.representation())
                {
                    if old_value == new_value {
                        return self.replace(effect);
                    }
                }
                let state = state.kill_element(object, index, self.zone());
                let state = match access.machine_type.representation() {
                    MachineRepresentation::kNone
                    | MachineRepresentation::kBit
                    | MachineRepresentation::kWord8
                    | MachineRepresentation::kWord16
                    | MachineRepresentation::kWord32
                    | MachineRepresentation::kWord64
                    | MachineRepresentation::kFloat16RawBits
                    | MachineRepresentation::kFloat16
                    | MachineRepresentation::kFloat32
                    | MachineRepresentation::kCompressedPointer
                    | MachineRepresentation::kCompressed
                    | MachineRepresentation::kSandboxedPointer
                    | MachineRepresentation::kProtectedPointer
                    | MachineRepresentation::kIndirectPointer => state,
                    MachineRepresentation::kFloat64
                    | MachineRepresentation::kSimd128
                    | MachineRepresentation::kSimd256
                    | MachineRepresentation::kTaggedSigned
                    | MachineRepresentation::kTaggedPointer
                    | MachineRepresentation::kTagged
                    | MachineRepresentation::kMapWord => state.add_element(
                        object,
                        index,
                        new_value,
                        access.machine_type.representation(),
                        self.zone(),
                    ),
                };
                self.update_state(node, state)
            }
        }
    }
    fn reduce_transition_and_store_element(&mut self, node: &mut Node) -> Reduction {
        let object = NodeProperties::get_value_input(node, 0);
        let double_map = MapRef(DoubleMapParameterOf(node.op()));
        let fast_map = MapRef(FastMapParameterOf(node.op()));
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => {
                let mut object_maps = ZoneRefSet::<Map>::new(self.zone());
                if state.lookup_maps(object, &mut object_maps) {
                    object_maps.insert(double_map, self.zone());
                    object_maps.insert(fast_map, self.zone());
                    let state = state.kill_maps(object, self.zone());
                    let state = state.set_maps(object, object_maps, self.zone());
                    let state = state.kill_field(
                        object,
                        FieldIndexOf(JSObject::kElementsOffset, kTaggedSize),
                        MaybeHandle::<Name>::empty(),
                        self.zone(),
                    );
                    self.update_state(node, state)
                } else {
                    Reduction::NoChange
                }
            }
        }
    }

    fn reduce_store_typed_element(&mut self, node: &mut Node) -> Reduction {
        let effect = NodeProperties::get_effect_input(node);
        let state = self.node_states_.get(effect);
        match state {
            None => Reduction::NoChange,
            Some(state) => self.update_state(node, state),
        }
    }

    fn reduce_effect_phi(&mut self, node: &mut Node) -> Reduction {
        let effect0 = NodeProperties::get_effect_input(node, 0);
        let control = NodeProperties::get_control_input(node);
        let state0 = self.node_states_.get(effect0);

        match state0 {
            None => Reduction::NoChange,
            Some(state0) => {
                if control.opcode() == IrOpcode::kLoop {
                    let state = self.compute_loop_state(node, state0);
                    self.update_state(node, state)
                } else if control.opcode() == IrOpcode::kMerge {
                    let input_count = node.op().effect_input_count();
                    for i in 1..input_count {
                        let effect = NodeProperties::get_effect_input(node, i as i32);
                        if self.node_states_.get(effect).is_none() {
                            return Reduction::NoChange;
                        }
                    }

                    let mut state = AbstractState::clone(state0);
                    for i in 1..input_count {
                        let input = NodeProperties::get_effect_input(node, i as i32);
                        let input_state = self.node_states_.get(input).unwrap();
                        state.merge(input_state, self.zone());
                    }

                    let mut state_with_phis = state;
                    for use in control.uses() {
                        if use.opcode() == IrOpcode::kPhi {
                            state_with_phis = self.update_state_for_phi(state_with_phis, node, use);
                        }
                    }
                    self.update_state(node, state_with_phis)
                } else {
                    Reduction::NoChange
                }
            }
        }
    }

    fn reduce_start(&mut self, node: &mut Node) -> Reduction {
        self.update_state(node, AbstractState::empty_state())
    }

    fn reduce_other_node(&mut self, node: &mut Node) -> Reduction {
        if node.op().effect_input_count() == 1 {
            if node.op().effect_output_count() == 1 {
                let effect = NodeProperties::get_effect_input(node);
                let state = self.node_states_.get(effect);

                match state {
                    None => Reduction::NoChange,
                    Some(state) => {
                        let mut state = state;
                        if !node.op().has_property(Operator::kNoWrite) {
                            state = state.kill_all(self.zone());
                        }
                        self.update_state(node, state)
                    }
                }
            } else {
                Reduction::NoChange
            }
        } else {
            Reduction::NoChange
        }
    }

    fn update_state(&mut self, node: &mut Node, state: &AbstractState) -> Reduction {
        let original = self.node_states_.get(node);

        if original.is_none() || !state.equals(&original.unwrap()) {
            self.node_states_.set(node, state);
            return self.changed(node);
        }

        Reduction::NoChange
    }
    fn compute_loop_state_for_store_field(
        &mut self,
        current: &mut Node,
        state: &AbstractState,
        access: FieldAccess,
    ) -> &AbstractState {
        let object = NodeProperties::get_value_input(current, 0);
        let mut state = state;

        if access.offset == HeapObject::kMapOffset {
            state = state.kill_maps(object, self.zone());
        } else {
            let field_index = FieldIndexOf(access);
            if field_index == IndexRange::invalid() {
                state = state.kill_fields(object, access.name, self.zone());
            } else {
                state = state.kill_field(object, field_index, access.name, self.zone());
            }
        }

        state
    }
    fn compute_loop_state(&mut self, node: &mut Node, state: &AbstractState) -> &AbstractState {
        let control = NodeProperties::get_control_input(node);

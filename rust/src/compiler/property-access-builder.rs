// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/property-access-builder.h (Module definition - not directly translatable, represented by file structure)

// src/compiler/property-access-builder.cc

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

//use crate::compiler::access_builder::*; // Assuming access_builder.h is converted to access_builder.rs
use crate::compiler::access_info::*;
use crate::compiler::compilation_dependencies::*; // Assuming compilation_dependencies.h is converted to compilation_dependencies.rs
use crate::compiler::js_graph::*; // Assuming js_graph.h is converted to js_graph.rs
use crate::compiler::node_matchers::*; // Assuming node_matchers.h is converted to node_matchers.rs
use crate::compiler::simplified_operator::*; // Assuming simplified_operator.h is converted to simplified_operator.rs
//use crate::objects::heap_number::*; // Assuming heap_number.h is converted to heap_number.rs
//use crate::objects::internal_index::*; // Assuming internal_index.h is converted to internal_index.rs
//use crate::objects::js_function::*; // Assuming js_function.h is converted to js_function.rs
//use crate::objects::map_inl::*; // Assuming map_inl.h is converted to map_inl.rs
//use crate::objects::property_details::*; // Assuming property_details.h is converted to property_details.rs

// Mock definitions for types that are too complex or not fully specified
// struct MapRef;
// struct JSHeapBroker;
// struct CompilationDependencies;
// struct FeedbackSource;
// struct ObjectRef;
// struct HeapObjectMatcher;
// struct Map;
// struct OptionalJSObjectRef;
// struct OptionalObjectRef;
// struct FieldAccess;
// struct NameRef;
// struct HeapObject;
// struct Float64;
// enum MachineRepresentation;
// enum MachineType;
// enum Type;
// enum CheckMapsFlag;
// enum DeoptimizeReason;
// enum PropertyKind;
// struct DirectHandle<T>;
// enum IrOpcode;
// type Control = NodeId;
// type Effect = NodeId;
// type NodeId = u32;

pub struct PropertyAccessBuilder<'a> {
    jsgraph: &'a JSGraph,
    dependencies: Option<&'a CompilationDependencies>, // Changed to Option<&>
}

impl<'a> PropertyAccessBuilder<'a> {
    pub fn new(jsgraph: &'a JSGraph, dependencies: Option<&'a CompilationDependencies>) -> Self {
        PropertyAccessBuilder { jsgraph, dependencies }
    }

    pub fn graph(&self) -> &TFGraph {
        self.jsgraph.graph()
    }

    // pub fn isolate(&self) -> &Isolate {
    //     self.jsgraph.isolate()
    // }

    pub fn common(&self) -> &CommonOperatorBuilder {
        self.jsgraph.common()
    }

    pub fn simplified(&self) -> &SimplifiedOperatorBuilder {
        self.jsgraph.simplified()
    }

    fn dependencies(&self) -> Option<&CompilationDependencies> {
        self.dependencies
    }

    // fn broker(&self) -> &JSHeapBroker {
    //     self.jsgraph.broker()
    // }

    fn jsgraph(&self) -> &JSGraph {
        self.jsgraph
    }

    // bool HasOnlyStringMaps(JSHeapBroker* broker, ZoneVector<MapRef> const& maps)
    fn has_only_string_maps(_broker: &JSHeapBroker, maps: &Vec<MapRef>) -> bool {
        for map in maps {
            if !map.is_string_map() {
                return false;
            }
        }
        true
    }

    // bool HasOnlyStringWrapperMaps(JSHeapBroker* broker,ZoneVector<MapRef> const& maps)
    fn has_only_string_wrapper_maps(_broker: &JSHeapBroker, maps: &Vec<MapRef>) -> bool {
        for map in maps {
            if !map.is_js_primitive_wrapper_map() {
                return false;
            }
            let elements_kind = map.elements_kind();
            if elements_kind != ElementsKind::FAST_STRING_WRAPPER_ELEMENTS
                && elements_kind != ElementsKind::SLOW_STRING_WRAPPER_ELEMENTS
            {
                return false;
            }
        }
        true
    }

    // bool HasOnlyNonResizableTypedArrayMaps(JSHeapBroker* broker, ZoneVector<MapRef> const& maps)
    fn has_only_non_resizable_typed_array_maps(_broker: &JSHeapBroker, maps: &Vec<MapRef>) -> bool {
        for map in maps {
            if !map.is_js_typed_array_map() {
                return false;
            }
            if is_rab_gsab_typed_array_elements_kind(map.elements_kind()) {
                return false;
            }
        }
        true
    }

    // bool HasOnlyNumberMaps(JSHeapBroker* broker, ZoneVector<MapRef> const& maps)
    fn has_only_number_maps(_broker: &JSHeapBroker, maps: &Vec<MapRef>) -> bool {
        for map in maps {
            if map.instance_type() != InstanceType::HEAP_NUMBER_TYPE {
                return false;
            }
        }
        true
    }

    // bool TryBuildStringCheck(JSHeapBroker* broker,ZoneVector<MapRef> const& maps,Node** receiver, Effect* effect,Control control)
    pub fn try_build_string_check(
        &self,
        broker: &JSHeapBroker,
        maps: &Vec<MapRef>,
        receiver: &mut NodeId,
        effect: &mut Effect,
        control: Control,
    ) -> bool {
        if PropertyAccessBuilder::has_only_string_maps(broker, maps) {
            // Monormorphic string access (ignoring the fact that there are multiple
            // String maps).
            *receiver = self.graph().new_node(
                self.simplified().check_string(FeedbackSource {}),
                *receiver,
                *effect,
                control,
            );
            *effect = *receiver;
            return true;
        }
        false
    }

    // bool TryBuildNumberCheck(JSHeapBroker* broker,ZoneVector<MapRef> const& maps,Node** receiver, Effect* effect,Control control)
    pub fn try_build_number_check(
        &self,
        broker: &JSHeapBroker,
        maps: &Vec<MapRef>,
        receiver: &mut NodeId,
        effect: &mut Effect,
        control: Control,
    ) -> bool {
        if PropertyAccessBuilder::has_only_number_maps(broker, maps) {
            // Monomorphic number access (we also deal with Smis here).
            *receiver = self.graph().new_node(
                self.simplified().check_number(FeedbackSource {}),
                *receiver,
                *effect,
                control,
            );
            *effect = *receiver;
            return true;
        }
        false
    }

    // void BuildCheckMaps(Node* object, Effect* effect, Control control,ZoneVector<MapRef> const& maps,bool has_deprecated_map_without_migration_target)
    pub fn build_check_maps(
        &self,
        object: NodeId,
        effect: &mut Effect,
        control: Control,
        maps: &Vec<MapRef>,
        has_deprecated_map_without_migration_target: bool,
    ) {
        let m = HeapObjectMatcher::new(object);
        if m.has_resolved_value() {
            let object_map = m.reference(self.jsgraph().broker()).map(self.jsgraph().broker());
            if object_map.is_stable() {
                for map in maps {
                    if map.equals(&object_map) {
                        if let Some(dependencies) = self.dependencies() {
                            dependencies.depend_on_stable_map(&object_map);
                        }
                        return;
                    }
                }
            }
        }

        let mut map_set: Vec<MapRef> = Vec::new(); //ZoneRefSet<Map>
        let mut has_migration_target = false;
        for map in maps {
            map_set.push(map.clone()); //map_set.insert(map, graph()->zone());
            if map.is_migration_target() {
                has_migration_target = true;
            }
        }

        let mut flags = CheckMapsFlags::kNone;
        if has_migration_target {
            flags = CheckMapsFlags::kTryMigrateInstance;
        } else if has_deprecated_map_without_migration_target {
            flags = CheckMapsFlags::kTryMigrateInstanceAndDeopt;
        }

        *effect = self.graph().new_node(
            self.simplified().check_maps(flags, map_set),
            object,
            *effect,
            control,
        );
    }

    // Node* BuildCheckValue(Node* receiver, Effect* effect,Control control, ObjectRef value)
    pub fn build_check_value(
        &self,
        receiver: NodeId,
        effect: &mut Effect,
        control: Control,
        value: &ObjectRef,
    ) -> NodeId {
        if value.is_heap_object() {
            let m = HeapObjectMatcher::new(receiver);
            if m.is(value.as_heap_object().object()) {
                return receiver;
            }
        }
        //let expected = self.jsgraph().constant_no_hole(value, self.jsgraph().broker());
        let expected = self.jsgraph().constant_no_hole(value); //modified for testing

        let check = self
            .graph()
            .new_node(self.simplified().reference_equal(), receiver, expected);
        *effect = self.graph().new_node(
            self.simplified()
                .check_if(DeoptimizeReason::kWrongValue),
            check,
            *effect,
            control,
        );
        expected
    }

    // Node* BuildCheckSmi(Node* value, Effect* effect, Control control, FeedbackSource feedback_source)
    pub fn build_check_smi(
        &self,
        value: NodeId,
        effect: &mut Effect,
        control: Control,
        feedback_source: FeedbackSource,
    ) -> NodeId {
        let smi_value = *effect = self.graph().new_node(
            self.simplified().check_smi(feedback_source),
            value,
            *effect,
            control,
        );
        smi_value
    }

    // Node* BuildCheckNumber(Node* value, Effect* effect, Control control,FeedbackSource feedback_source)
    pub fn build_check_number(
        &self,
        value: NodeId,
        effect: &mut Effect,
        control: Control,
        feedback_source: FeedbackSource,
    ) -> NodeId {
        let number = *effect = self.graph().new_node(
            self.simplified().check_number(feedback_source),
            value,
            *effect,
            control,
        );
        number
    }

    // Node* BuildCheckNumberFitsInt32(Node* value, Effect* effect, Control control,FeedbackSource feedback_source)
    pub fn build_check_number_fits_int32(
        &self,
        value: NodeId,
        effect: &mut Effect,
        control: Control,
        feedback_source: FeedbackSource,
    ) -> NodeId {
        let number = *effect = self.graph().new_node(
            self.simplified().check_number_fits_int32(feedback_source),
            value,
            *effect,
            control,
        );
        number
    }

    // Node* ResolveHolder(PropertyAccessInfo const& access_info, Node* lookup_start_object)
    pub fn resolve_holder(
        &self,
        access_info: &PropertyAccessInfo,
        lookup_start_object: NodeId,
    ) -> NodeId {
        if let Some(holder) = access_info.holder() {
            //return self.jsgraph().constant_no_hole(holder.value(), self.jsgraph().broker());
            return self.jsgraph().constant_no_hole(&holder.value()); //modified for testing
        }
        lookup_start_object
    }

    // MachineRepresentation ConvertRepresentation(Representation representation)
    pub fn convert_representation(representation: Representation) -> MachineRepresentation {
        match representation.kind() {
            RepresentationKind::kSmi => MachineRepresentation::kTaggedSigned,
            RepresentationKind::kDouble => MachineRepresentation::kFloat64,
            RepresentationKind::kHeapObject => MachineRepresentation::kTaggedPointer,
            RepresentationKind::kTagged => MachineRepresentation::kTagged,
            _ => unreachable!(),
        }
    }

    // std::optional<Node*> FoldLoadDictPrototypeConstant(PropertyAccessInfo const& access_info)
    pub fn fold_load_dict_prototype_constant(
        &self,
        access_info: &PropertyAccessInfo,
    ) -> Option<NodeId> {
        if !V8_DICT_PROPERTY_CONST_TRACKING_BOOL {
            return None;
        }
        if !access_info.is_dictionary_proto_data_constant() {
            return None;
        }

        let index = access_info.dictionary_index();
        let value = access_info
            .holder()
            .as_ref()
            .and_then(|holder| {
                holder.get_own_dictionary_property(
                    self.jsgraph().broker(),
                    index,
                    self.dependencies().unwrap(),
                )
            });

        if value.is_none() {
            return None;
        }

        //     for (MapRef map : access_info.lookup_start_object_maps()) {
        //       DirectHandle<Map> map_handle = map.object();
        //       // Non-JSReceivers that passed AccessInfoFactory::ComputePropertyAccessInfo
        //       // must have different lookup start map.
        //       if (!IsJSReceiverMap(*map_handle)) {
        //         // Perform the implicit ToObject for primitives here.
        //         // Implemented according to ES6 section 7.3.2 GetV (V, P).
        //         Tagged<JSFunction> constructor =
        //             Map::GetConstructorFunction(
        //                 *map_handle, *broker()->target_native_context().object())
        //                 .value();
        //         // {constructor.initial_map()} is loaded/stored with acquire-release
        //         // semantics for constructors.
        //         map = MakeRefAssumeMemoryFence(broker(), constructor->initial_map());
        //         DCHECK(IsJSObjectMap(*map.object()));
        //       }
        //       dependencies()->DependOnConstantInDictionaryPrototypeChain(
        //           map, access_info.name(), value.value(), PropertyKind::kData);
        //     }

        for map in access_info.lookup_start_object_maps() {
            // DirectHandle<Map> map_handle = map.object();
            // Non-JSReceivers that passed AccessInfoFactory::ComputePropertyAccessInfo
            // must have different lookup start map.

            // if (!IsJSReceiverMap(*map_handle)) { // TODO: need a substitute for this condition
            //   // Perform the implicit ToObject for primitives here.
            //   // Implemented according to ES6 section 7.3.2 GetV (V, P).
            //   Tagged<JSFunction> constructor =
            //       Map::GetConstructorFunction(
            //           *map_handle, *broker()->target_native_context().object())
            //           .value();
            //   // {constructor.initial_map()} is loaded/stored with acquire-release
            //   // semantics for constructors.
            //   map = MakeRefAssumeMemoryFence(broker(), constructor->initial_map());
            //   DCHECK(IsJSObjectMap(*map.object()));
            // }

            if let Some(dependencies) = self.dependencies() {
                dependencies.depend_on_constant_in_dictionary_prototype_chain(
                    &map,
                    access_info.name(),
                    value.as_ref().unwrap(),
                    PropertyKind::kData,
                );
            }
        }
        //return self.jsgraph().constant_no_hole(value.value(), self.jsgraph().broker());
        return Some(self.jsgraph().constant_no_hole(value.as_ref().unwrap())); //modified for testing
    }

    // Node* TryFoldLoadConstantDataField(NameRef name, PropertyAccessInfo const& access_info, Node* lookup_start_object)
    pub fn try_fold_load_constant_data_field(
        &self,
        name: &NameRef,
        access_info: &PropertyAccessInfo,
        lookup_start_object: NodeId,
    ) -> Option<NodeId> {
        if !access_info.is_fast_data_constant() {
            return None;
        }

        // First, determine if we have a constant holder to load from.
        let holder = access_info.holder();

        // If {access_info} has a holder, just use it.
        if holder.is_none() {
            // Otherwise, try to match the {lookup_start_object} as a constant.
            let mut adjusted_lookup_start_object = lookup_start_object;
            let opcode = self.graph().node_opcode(lookup_start_object);
            if opcode == IrOpcode::kCheckString || opcode == IrOpcode::kCheckStringOrStringWrapper {
                // Bypassing Check inputs in order to allow constant folding.
                adjusted_lookup_start_object = self.graph().node_input_at(lookup_start_object, 0);
            }

            let m = HeapObjectMatcher::new(adjusted_lookup_start_object);
            if !m.has_resolved_value() || !m.reference(self.jsgraph().broker()).is_js_object() {
                return None;
            }

            // Let us make sure the actual map of the constant lookup_start_object is
            // among the maps in {access_info}.
            let lookup_start_object_map = m.reference(self.jsgraph().broker()).map(self.jsgraph().broker());

            if !access_info
                .lookup_start_object_maps()
                .iter()
                .any(|map| map.equals(&lookup_start_object_map))
            {
                // The map of the lookup_start_object is not in the feedback, let us bail
                // out.
                return None;
            }
            // holder = m.Ref(broker()).AsJSObject();
            // TODO: verify if this is valid way to convert m to OptionalJSObjectRef
            // let ref_obj = m.reference(self.jsgraph().broker());
            // holder = Some(ref_obj.as_js_object());
        }

        if access_info.field_representation().is_double() {
            let value = holder.as_ref().and_then(|h| {
                h.get_own_fast_constant_double_property(
                    self.jsgraph().broker(),
                    access_info.field_index(),
                    self.dependencies().unwrap(),
                )
            });
            return value.map(|v| {
                //self.jsgraph().constant_no_hole(v.get_scalar())
                self.jsgraph().constant_no_hole(&ObjectRef::from(v.get_scalar())) //modified for testing
            });
        }
        let value = holder.as_ref().and_then(|h| {
            h.get_own_fast_constant_data_property(
                self.jsgraph().broker(),
                access_info.field_representation(),
                access_info.field_index(),
                self.dependencies().unwrap(),
            )
        });

        value.map(|v| {
            //self.jsgraph().constant_no_hole(*v, self.jsgraph().broker())
            self.jsgraph().constant_no_hole(v) //modified for testing
        })
    }

    // Node* BuildLoadDataField(NameRef name, Node* holder,FieldAccess&& field_access,bool is_inobject, Node** effect,Node** control)
    pub fn build_load_data_field(
        &self,
        name: &NameRef,
        holder: NodeId,
        field_access: FieldAccess,
        is_inobject: bool,
        effect: &mut Effect,
        control: &mut Control,
    ) -> NodeId {
        let mut storage = holder;
        if !is_inobject {
            let access = AccessBuilder::for_js_object_properties_or_hash_known_pointer();
            storage = *effect = self.graph().new_node(
                self.simplified().load_field(access),
                storage,
                *effect,
                *control,
            );
        }
        let mut mutable_field_access = field_access;

        if mutable_field_access.machine_type.representation() == MachineRepresentation::kFloat64 {
            if self.dependencies().is_none() {
                let storage_access = FieldAccess {
                    base: kTaggedBase,
                    offset: mutable_field_access.offset,
                    name: name.object(),
                    map: None, //OptionalMapRef::default(), //default
                    field_type: Type::Any(),
                    machine_type: MachineType::AnyTagged(),
                    write_barrier_kind: WriteBarrierKind::kPointerWriteBarrier,
                    debug_name: "BuildLoadDataField",
                    const_field_info: mutable_field_access.const_field_info.clone(), //default(),
                };

                storage = *effect = self.graph().new_node(
                    self.simplified().load_field(storage_access),
                    storage,
                    *effect,
                    *control,
                );

                storage = *effect = self.graph().new_node(
                    self.simplified().check_heap_object(),
                    storage,
                    *effect,
                    *control,
                );
                let map = *effect = self.graph().new_node(
                    self.simplified().load_field(AccessBuilder::for_map()),
                    storage,
                    *effect,
                    *control,
                );
                let is_heap_number = self.graph().new_node(
                    self.simplified().reference_equal(),
                    map,
                    self.jsgraph().heap_number_map_constant(),
                );
                *effect = self.graph().new_node(
                    self.simplified()
                        .check_if(DeoptimizeReason::kNotAHeapNumber),
                    is_heap_number,
                    *effect,
                    *control,
                );
            } else {
                let storage_access = FieldAccess {
                    base: kTaggedBase,
                    offset: mutable_field_access.offset,
                    name: name.object(),
                    map: None, //OptionalMapRef::default(), //default
                    field_type: Type::OtherInternal(),
                    machine_type: MachineType::TaggedPointer(),
                    write_barrier_kind: WriteBarrierKind::kPointerWriteBarrier,
                    debug_name: "BuildLoadDataField",
                    const_field_info: mutable_field_access.const_field_info.clone(), //default(),
                };

                storage = *effect = self.graph().new_node(
                    self.simplified().load_field(storage_access),
                    storage,
                    *effect,
                    *control,
                );
            }

            let mut value_field_access = AccessBuilder::for_heap_number_value();
            value_field_access.const_field_info = mutable_field_access.const_field_info.clone();
            mutable_field_access = value_field_access;
        }

        let value = *effect = self.graph().new_node(
            self.simplified().load_field(mutable_field_access),
            storage,
            *effect,
            *control,
        );
        value
    }

    // Node* BuildLoadDataField(NameRef name, PropertyAccessInfo const& access_info,Node* lookup_start_object, Node** effect, Node** control)
    pub fn build_load_data_field2(
        &self,
        name: &NameRef,
        access_info: &PropertyAccessInfo,
        lookup_start_object: NodeId,
        effect: &mut Effect,
        control: &mut Control,
    ) -> NodeId {
        assert!(
            access_info.is_data_field() || access_info.is_fast_data_constant(),
            "expected data field or constant"
        );

        if let Some(value) =
            self.try_fold_load_constant_data_field(name, access_info, lookup_start_object)
        {
            return value;
        }

        let field_representation =
            PropertyAccessBuilder::convert_representation(access_info.field_representation());
        let storage = self.resolve_holder(access_info, lookup_start_object);

        let mut field_access = FieldAccess {
            base: kTaggedBase,
            offset: access_info.field_index().offset(),
            name: name.object(),
            map: None, //OptionalMapRef::default(), //default
            field_type: access_info.field_type(),
            machine_type: MachineType::TypeForRepresentation(field_representation),
            write_barrier_kind: WriteBarrierKind::kFullWriteBarrier,
            debug_name: "BuildLoadDataField",
            const_field_info: access_info.get_const_field_info(),
        };

        if field_representation == MachineRepresentation::kTaggedPointer
            || field_representation == MachineRepresentation::kCompressedPointer
        {
            // Remember the map of the field value, if its map is stable. This is
            // used by the LoadElimination to eliminate map checks on the result.
            if let Some(field_map) = access_info.field_map() {
                if field_map.is_stable() {
                    if let Some(dependencies) = self.dependencies() {
                        dependencies.depend_on_stable_map(field_map);
                    }
                    field_access.map = Some(field_map.clone()); //Some(field_map);
                    field_access.field_type = Type::For(field_map, self.jsgraph().broker());
                }
            }
        }

        self.build_load_data_field(
            name,
            storage,
            field_access,
            access_info.field_index().is_inobject(),
            effect,
            control,
        )
    }
}

// Mock Enums and Structs for compilation:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceType {
    HEAP_NUMBER_TYPE,
    JS_OBJECT_TYPE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementsKind {
    FAST_STRING_WRAPPER_ELEMENTS,
    SLOW_STRING_WRAPPER_ELEMENTS,
    // Add other element kinds as needed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrOpcode {
    kCheckString,
    kCheckStringOrStringWrapper,
    // Add other opcodes as needed
}

#[derive(Debug, Clone)]
pub struct PropertyAccessInfo {
    is_data_field: bool,
    is_fast_data_constant: bool,
    dictionary_index: InternalIndex,
    holder: Option<JSObjectRef>,
    field_representation: Representation,
    field_index: InternalIndex,
    field_type: Type,
    field_map: Option<MapRef>,
    lookup_start_object_maps: Vec<MapRef>,
    name: NameRef,
}

impl PropertyAccessInfo {
    fn is_data_field(&self) -> bool {
        self.is_data_field
    }
    fn is_fast_data_constant(&self) -> bool {
        self.is_fast_data_constant
    }
    fn dictionary_index(&self) -> InternalIndex {
        self.dictionary_index
    }
    fn holder(&self) -> &Option<JSObjectRef> {
        &self.holder
    }
    fn field_representation(&self) -> Representation {
        self.field_representation
    }
    fn field_index(&self) -> InternalIndex {
        self.field_index
    }

    fn field_type(&self) -> Type {
        self.field_type
    }
    fn field_map(&self) -> &Option<MapRef> {
        &self.field_map
    }

    fn lookup_start_object_maps(&self) -> &Vec<MapRef> {
        &self.lookup_start_object_maps
    }

    fn name(&self) -> NameRef {
        self.name
    }
    fn get_const_field_info(&self) -> ConstFieldInfo {
        ConstFieldInfo {}
    }

    fn is_dictionary_proto_data_constant(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FeedbackSource {}

#[derive(Debug, Clone, Copy)]
pub enum CheckMapsFlags {
    kNone,
    kTryMigrateInstance,
    kTryMigrateInstanceAndDeopt,
}

#[derive(Debug, Clone, Copy)]
pub enum DeoptimizeReason {
    kWrongValue,
    kNotAHeapNumber,
    // Add other reasons as needed
}

#[derive(Debug, Clone, Copy)]
pub enum MachineRepresentation {
    kSmi,
    kDouble,
    kHeapObject,
    kTagged,
    kTaggedSigned,
    kFloat64,
    kTaggedPointer,
    kCompressedPointer,
    // Add other representations as needed
}

#[derive(Debug, Clone, Copy)]
pub enum MachineType {
    AnyTagged(),
    TypeForRepresentation(MachineRepresentation),
    TaggedPointer(),
    Any(),
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Any(),
    OtherInternal(),
}

impl Type {
    pub fn For(_field_map: &MapRef, _broker: &JSHeapBroker) -> Self {
        Type::Any()
    }
}

#[derive(Debug, Clone)]
pub struct MapRef {}

impl MapRef {
    pub fn is_string_map(&self) -> bool {
        false
    }
    pub fn is_js_primitive_wrapper_map(&self) -> bool {
        false
    }
    pub fn elements_kind(&self) -> ElementsKind {
        ElementsKind::FAST_STRING_WRAPPER_ELEMENTS
    }
    pub fn is_js_typed_array_map(&self) -> bool {
        false
    }
    pub fn instance_type(&self) -> InstanceType {
        InstanceType::HEAP_NUMBER_TYPE
    }

    pub fn is_stable(&self) -> bool {
        false
    }

    pub fn equals(&self, _other: &MapRef) -> bool {
        false
    }

    pub fn is_migration_target(&self) -> bool {
        false
    }
    // added for testing
    pub fn object(&self) -> DirectHandle<Map> {
        DirectHandle { inner: Map {} }
    }
}

#[derive(Debug, Clone)]
pub struct JSObjectRef {
    value: ObjectRef,
}

impl JSObjectRef {
    fn get_own_dictionary_property(
        &self,
        _broker: &JSHeapBroker,
        _index: InternalIndex,
        _dependencies: &CompilationDependencies,
    ) -> Option<&ObjectRef> {
        Some(&self.value)
    }

    fn get_own_fast_constant_double_property(
        &self,
        _broker: &JSHeapBroker,
        _index: InternalIndex,
        _dependencies: &CompilationDependencies,
    ) -> Option<Float64> {
        None
    }

    fn get_own_fast_constant_data_property(
        &self,
        _broker: &JSHeapBroker,
        _field_representation: Representation,
        _index: InternalIndex,
        _dependencies: &CompilationDependencies,
    ) -> Option<&ObjectRef> {
        Some(&self.value)
    }

    pub fn value(&self) -> ObjectRef {
        self.value.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InternalIndex {}

impl InternalIndex {
    pub fn offset(&self) -> i32 {
        0
    }
    pub fn is_inobject(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PropertyKind {
    kData,
}

#[derive(Debug, Clone)]
pub struct NameRef {}

impl NameRef {
    pub fn object(&self) -> *const NameRef {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Float64 {
    scalar: f64,
}

impl Float64 {
    pub fn get_scalar(&self) -> f64 {
        self.scalar
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RepresentationKind {
    kSmi,
    kDouble,
    kHeapObject,
    kTagged,
}

#[derive(Debug, Clone, Copy)]
pub struct Representation {
    kind: RepresentationKind,
}

impl Representation {
    pub fn kind(&self) -> RepresentationKind {
        self.kind
    }
    pub fn is_double(&self) -> bool {
        self.kind == RepresentationKind::kDouble
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WriteBarrierKind {
    kNoWriteBarrier,
    kPointerWriteBarrier,
    kFullWriteBarrier,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstFieldInfo {}

#[derive(Debug, Clone)]
pub struct FieldAccess {
    base: Base,
    offset: i32,
    name: *const NameRef,
    map: Option<MapRef>,
    field_type: Type,
    machine
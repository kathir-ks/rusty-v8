// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add equivalent Rust crates for the following C++ includes:
// - "src/compiler/compiler-source-position-table.h"
// - "src/compiler/node-properties.h"
// - "src/compiler/simplified-operator.h"
// - "src/compiler/wasm-compiler-definitions.h"
// - "src/wasm/wasm-subtyping.h"

// use v8::internal::compiler::*; // Assuming v8 is an external crate
// use v8::wasm::*; // Assuming v8 is an external crate
// use v8::base::*; // Assuming v8 is an external crate

mod compiler {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    // Placeholder types, replace with actual definitions
    pub struct Node {
        opcode_: IrOpcode,
        inputs_: Vec<*mut Node>,
        // Add other fields as needed
    }

    impl Node {
        pub fn opcode(&self) -> IrOpcode {
            self.opcode_
        }

        pub fn inputs(&self) -> &Vec<*mut Node> {
            &self.inputs_
        }

        pub fn remove_input(&mut self, index: usize) {
            self.inputs_.remove(index);
        }

        // Add other methods as needed
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IrOpcode {
        Start,
        WasmStructGet,
        WasmStructSet,
        WasmArrayLength,
        AssertNotNull,
        IsNull,
        IsNotNull,
        WasmTypeCheck,
        WasmTypeCheckAbstract,
        WasmTypeCast,
        WasmTypeCastAbstract,
        TypeGuard,
        WasmAnyConvertExtern,
        Merge,
        IfTrue,
        IfFalse,
        Dead,
        Loop,
        Branch,
        DeadValue,
    }

    pub struct Editor {}
    pub struct Zone {}
    pub struct MachineGraph {}
    pub struct SourcePositionTable {}

    impl MachineGraph {
        pub fn graph(&self) -> &Graph {
            // Placeholder implementation
            unimplemented!("MachineGraph::graph")
        }

        pub fn zone(&self) -> &Zone {
            // Placeholder implementation
            unimplemented!("MachineGraph::zone")
        }
    }

    pub struct Graph {}

    pub mod wasm {
        use super::*;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ValueType {
            kind: ValueTypeKind,
        }

        impl ValueType {
            pub fn is_nullable(&self) -> bool {
                self.kind == ValueTypeKind::NullableRef
            }

            pub fn as_non_null(&self) -> ValueType {
                // Placeholder implementation
                ValueType { kind: ValueTypeKind::Ref }
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum ValueTypeKind {
            I32,
            Ref,
            NullableRef,
            ExternRef,
            AnyRef,
            // Add other value type kinds as needed
        }

        pub const kWasmI32: ValueType = ValueType { kind: ValueTypeKind::I32 };
        pub const kWasmAnyRef: ValueType = ValueType { kind: ValueTypeKind::AnyRef };
        pub const kWasmExternRef: ValueType = ValueType { kind: ValueTypeKind::ExternRef };

        pub fn to_null_sentinel(t: TypeInModule) -> ValueType {
            // Placeholder implementation
            ValueType { kind: ValueTypeKind::NullableRef }
        }

        pub fn is_heap_subtype_of(
            a: HeapType,
            b: HeapType,
            module_a: Option<&WasmModule>,
            module_b: Option<&WasmModule>,
        ) -> bool {
            // Placeholder implementation
            true
        }

        pub fn heap_types_unrelated(
            a: HeapType,
            b: HeapType,
            module_a: Option<&WasmModule>,
            module_b: Option<&WasmModule>,
        ) -> bool {
            // Placeholder implementation
            false
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct TypeInModule<'a> {
            pub type_: ValueType,
            pub module: Option<&'a WasmModule>,
        }

        impl<'a> TypeInModule<'a> {
            pub fn new(type_: ValueType, module: Option<&'a WasmModule>) -> Self {
                TypeInModule { type_, module }
            }
        }

        pub fn intersection<'a>(a: TypeInModule<'a>, b: TypeInModule<'a>) -> TypeInModule<'a> {
            // Placeholder implementation
            a
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum HeapType {
            Any,
            String,
            // Add other heap types as needed
        }
    }

    pub mod simplified {
        use super::*;

        pub struct Simplified {
            zone_: Zone,
        }

        impl Simplified {
            pub fn wasm_struct_get(
                &self,
                op_params: ValueType,
                field_index: usize,
                is_signed: bool,
                null_check: NullCheck,
            ) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_struct_get")
            }
            pub fn wasm_struct_set(
                &self,
                op_params: ValueType,
                field_index: usize,
                null_check: NullCheck,
            ) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_struct_set")
            }
            pub fn wasm_array_length(&self, null_check: NullCheck) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_array_length")
            }
            pub fn wasm_type_cast(config: WasmTypeCheckConfig) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_type_cast")
            }
            pub fn wasm_type_check(config: WasmTypeCheckConfig) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_type_check")
            }
            pub fn wasm_type_cast_abstract(config: WasmTypeCheckConfig) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_type_cast_abstract")
            }
            pub fn wasm_type_check_abstract(config: WasmTypeCheckConfig) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Simplified::wasm_type_check_abstract")
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NullCheck {
        kWithNullCheck,
        kWithoutNullCheck,
    }

    pub mod common {
        use super::*;
        pub struct Common {
            zone_: Zone,
        }

        impl Common {
            pub fn type_guard(&self, t: Type) -> *const Operator {
                // Placeholder implementation
                unimplemented!("Common::type_guard")
            }
        }
    }

    pub struct NodeProperties {}

    impl NodeProperties {
        pub fn get_control_input(node: *mut Node) -> *mut Node {
            // Placeholder implementation
            unsafe { (*node).inputs_[0] }
        }
        pub fn get_value_input(node: *mut Node, index: usize) -> *mut Node {
            // Placeholder implementation
            unsafe { (*node).inputs_[index] }
        }

        pub fn change_op(node: *mut Node, op: *const Operator) {
            // Placeholder implementation
        }

        pub fn set_type(node: *mut Node, t: Type) {
            // Placeholder implementation
        }

        pub fn get_type(node: *mut Node) -> Type {
            // Placeholder implementation
            Type::Any
        }

        pub fn is_typed(node: *mut Node) -> bool {
            // Placeholder implementation
            true
        }
    }

    pub struct Type {
        // Placeholder fields
        is_wasm: bool,
        wasm_type: wasm::TypeInModule<'static>, // Assuming static lifetime for simplicity
    }

    impl Type {
        pub const Any: Self = Type { is_wasm: false, wasm_type: wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None } };

        pub fn is_wasm(&self) -> bool {
            self.is_wasm
        }

        pub fn as_wasm(&self) -> wasm::TypeInModule<'static> {
            self.wasm_type
        }
    }

    pub struct WasmModule {}

    pub struct WasmFieldInfo {
        // Placeholder fields
    }

    pub struct OpParameter {}

    impl OpParameter {
        pub fn new(_op: *const Operator) -> Self {
            // Placeholder implementation
            OpParameter {}
        }
    }

    pub struct WasmTypeCheckConfig {
        pub from: wasm::ValueType,
        pub to: wasm::ValueType,
    }

    pub struct Operator {}

    // Assuming gasm_ is a helper struct for generating nodes
    pub struct GraphAssembler<'a> {
        mcgraph_: &'a MachineGraph,
        zone_: &'a Zone,
    }

    impl<'a> GraphAssembler<'a> {
        pub fn new(mcgraph: &'a MachineGraph, zone: &'a Zone) -> Self {
            GraphAssembler {
                mcgraph_: mcgraph,
                zone_: zone,
            }
        }

        pub fn int32_constant(&self, value: i32) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::int32_constant")
        }

        pub fn null(&self, _type: wasm::ValueType) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::null")
        }

        pub fn is_null(&self, _object: *mut Node, _object_type: wasm::ValueType) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::is_null")
        }

        pub fn is_not_null(&self, _object: *mut Node, _object_type: wasm::ValueType) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::is_not_null")
        }

        pub fn assert_not_null(&mut self, object: *mut Node, object_type: wasm::ValueType, trap_id: TrapId) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::assert_not_null")
        }

        pub fn trap_unless(&mut self, condition: *mut Node, trap_id: TrapId) {
            // Placeholder implementation
            unimplemented!("GraphAssembler::trap_unless")
        }

        pub fn effect(&mut self) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::effect")
        }

        pub fn control(&mut self) -> *mut Node {
            // Placeholder implementation
            unimplemented!("GraphAssembler::control")
        }

        pub fn initialize_effect_control(&mut self, _effect: *mut Node, _control: *mut Node) {
            // Placeholder implementation
            unimplemented!("GraphAssembler::initialize_effect_control")
        }

        pub fn simplified(&self) -> &simplified::Simplified {
            // Placeholder implementation
            unimplemented!("GraphAssembler::simplified")
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TrapId {
        kTrapIllegalCast,
    }

    // Placeholder for kNoSourcePosition
    pub const kNoSourcePosition: i32 = -1;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SourcePosition {
        script_offset: i32,
    }

    impl SourcePosition {
        pub fn script_offset(&self) -> i32 {
            self.script_offset
        }
    }

    // Placeholder implementations for SourcePositionTable methods
    impl SourcePositionTable {
        pub fn get_source_position(&self, _node: *mut Node) -> SourcePosition {
            SourcePosition {
                script_offset: kNoSourcePosition,
            }
        }
        pub fn set_source_position(&mut self, _node: *mut Node, _position: SourcePosition) {}
    }

    // ControlPathTypes and NodeWithType structs for holding state information
    #[derive(Debug, Clone)]
    pub struct NodeWithType<'a> {
        pub node: *mut Node,
        pub type_: wasm::TypeInModule<'a>,
        is_set: bool,
    }

    impl<'a> NodeWithType<'a> {
        pub fn new(node: *mut Node, type_: wasm::TypeInModule<'a>) -> Self {
            NodeWithType {
                node,
                type_,
                is_set: true,
            }
        }

        pub fn is_set(&self) -> bool {
            self.is_set
        }
    }

    #[derive(Debug, Clone)]
    pub struct ControlPathTypes<'a> {
        types: HashMap<*mut Node, NodeWithType<'a>>,
        zone_: &'a Zone,
    }

    impl<'a> ControlPathTypes<'a> {
        pub fn new(zone: &'a Zone) -> Self {
            ControlPathTypes {
                types: HashMap::new(),
                zone_: zone,
            }
        }

        pub fn lookup_state(&self, node: *mut Node) -> NodeWithType<'a> {
            self.types.get(&node).cloned().unwrap_or(NodeWithType { node: std::ptr::null_mut(), type_: wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None }, is_set: false })
        }

        pub fn insert_state(&mut self, node: *mut Node, info: NodeWithType<'a>) {
            self.types.insert(node, info);
        }

        pub fn reset_to_common_ancestor(&mut self, other: &ControlPathTypes<'a>) {
            // Placeholder implementation for finding the common ancestor
            let mut common_types: HashMap<*mut Node, NodeWithType<'a>> = HashMap::new();

            for (node, node_with_type) in self.types.iter() {
                if let Some(other_node_with_type) = other.types.get(node) {
                    if node_with_type.type_ == other_node_with_type.type_ {
                        common_types.insert(*node, node_with_type.clone());
                    }
                }
            }

            self.types = common_types;
        }

        pub fn is_empty(&self) -> bool {
            self.types.is_empty()
        }
    }

    // Abstract reducer with control path state
    pub struct AdvancedReducerWithControlPathState<'a> {
        editor: *mut Editor, // Assuming raw pointer is safe here, as in original C++
        temp_zone_: &'a Zone,
        graph_: &'a Graph,
        states: RefCell<HashMap<*mut Node, ControlPathTypes<'a>>>,
    }

    impl<'a> AdvancedReducerWithControlPathState<'a> {
        pub fn new(editor: *mut Editor, temp_zone_: &'a Zone, graph_: &'a Graph) -> Self {
            AdvancedReducerWithControlPathState {
                editor,
                temp_zone_: temp_zone_,
                graph_: graph_,
                states: RefCell::new(HashMap::new()),
            }
        }

        pub fn get_state(&self, node: *mut Node) -> ControlPathTypes<'a> {
            self.states
                .borrow()
                .get(&node)
                .cloned()
                .unwrap_or(ControlPathTypes::new(self.temp_zone_))
        }

        pub fn update_states(
            &self,
            state_owner: *mut Node,
            parent_state: ControlPathTypes<'a>,
            node: *mut Node,
            node_with_type: NodeWithType<'a>,
            in_new_block: bool,
        ) -> Reduction {
            let mut current_state = self.get_state(state_owner);
            if !in_new_block {
                for (n, t) in parent_state.types.iter() {
                    current_state.types.insert(*n, t.clone());
                }
            }

            current_state.insert_state(node, node_with_type);
            self.states.borrow_mut().insert(state_owner, current_state);
            Reduction::Changed(state_owner)
        }

        pub fn update_states_initial(&self, node: *mut Node, types: ControlPathTypes<'a>) -> Reduction {
            self.states.borrow_mut().insert(node, types);
            Reduction::Changed(node)
        }

        pub fn take_states_from_first_control(&self, node: *mut Node) -> Reduction {
            let control = unsafe { (*node).inputs()[0] };
            let state = self.get_state(control);
            self.states.borrow_mut().insert(node, state);
            Reduction::NoChange()
        }

        pub fn is_reduced(&self, node: *mut Node) -> bool {
            self.states.borrow().contains_key(&node)
        }
    }

    // Reducer struct and methods
    pub struct WasmGCOperatorReducer<'a> {
        advanced_reducer: AdvancedReducerWithControlPathState<'a>,
        mcgraph_: *mut MachineGraph, // Assuming raw pointer is safe here, as in original C++
        gasm_: GraphAssembler<'a>,
        module_: *const WasmModule,   // Assuming raw pointer is safe here, as in original C++
        source_position_table_: *mut SourcePositionTable, // Assuming raw pointer is safe here, as in original C++
    }

    impl<'a> WasmGCOperatorReducer<'a> {
        pub fn new(
            editor: *mut Editor,
            temp_zone_: &'a Zone,
            mcgraph: *mut MachineGraph,
            module: *const WasmModule,
            source_position_table: *mut SourcePositionTable,
        ) -> Self {
            let mcgraph_ref = unsafe { &*mcgraph };
            let gasm_ = GraphAssembler::new(mcgraph_ref, temp_zone_);
            WasmGCOperatorReducer {
                advanced_reducer: AdvancedReducerWithControlPathState::new(editor, temp_zone_, mcgraph_ref.graph()),
                mcgraph_: mcgraph,
                gasm_: gasm_,
                module_: module,
                source_position_table_: source_position_table,
            }
        }

        pub fn reduce(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                match (*node).opcode() {
                    IrOpcode::Start => self.reduce_start(node),
                    IrOpcode::WasmStructGet | IrOpcode::WasmStructSet => {
                        self.reduce_wasm_struct_operation(node)
                    }
                    IrOpcode::WasmArrayLength => self.reduce_wasm_array_length(node),
                    IrOpcode::AssertNotNull => self.reduce_assert_not_null(node),
                    IrOpcode::IsNull | IrOpcode::IsNotNull => self.reduce_check_null(node),
                    IrOpcode::WasmTypeCheck => self.reduce_wasm_type_check(node),
                    IrOpcode::WasmTypeCheckAbstract => self.reduce_wasm_type_check_abstract(node),
                    IrOpcode::WasmTypeCast => self.reduce_wasm_type_cast(node),
                    IrOpcode::WasmTypeCastAbstract => self.reduce_wasm_type_cast_abstract(node),
                    IrOpcode::TypeGuard => self.reduce_type_guard(node),
                    IrOpcode::WasmAnyConvertExtern => self.reduce_wasm_any_convert_extern(node),
                    IrOpcode::Merge => self.reduce_merge(node),
                    IrOpcode::IfTrue => self.reduce_if(node, true),
                    IrOpcode::IfFalse => self.reduce_if(node, false),
                    IrOpcode::Dead => Reduction::NoChange(),
                    IrOpcode::Loop => self.advanced_reducer.take_states_from_first_control(node),
                    _ => {
                        if (*node).opcode().control_output_count() > 0 {
                            assert_eq!((*node).opcode().control_input_count(), 1);
                            self.advanced_reducer.take_states_from_first_control(node)
                        } else {
                            Reduction::NoChange()
                        }
                    }
                }
            }
        }

        fn reduce_start(&mut self, node: *mut Node) -> Reduction {
            let zone_ = unsafe { (&*self.mcgraph_).zone() };
            self.advanced_reducer.update_states_initial(node, ControlPathTypes::new(zone_))
        }

        fn object_type_from_context(
            &self,
            object: *mut Node,
            control: *mut Node,
            allow_non_wasm: bool,
        ) -> wasm::TypeInModule<'a> {
            unsafe {
                if (*object).opcode() == IrOpcode::Dead || (*object).opcode() == IrOpcode::DeadValue {
                    return wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None };
                }
                if !self.advanced_reducer.is_reduced(control) {
                    return wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None };
                }
                if allow_non_wasm && !NodeProperties::is_typed(object) {
                    return wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None };
                }
                let raw_type = NodeProperties::get_type(object);
                if allow_non_wasm && !raw_type.is_wasm() {
                    return wasm::TypeInModule { type_: wasm::ValueType { kind: wasm::ValueTypeKind::AnyRef }, module: None };
                }
                let type_from_node = raw_type.as_wasm();
                let state = self.advanced_reducer.get_state(control);
                let mut type_from_state = state.lookup_state(object);
                // We manually resolve TypeGuard aliases in the state.
                let mut current_object = object;
                while (*current_object).opcode() == IrOpcode::TypeGuard && !type_from_state.is_set() {
                    current_object = NodeProperties::get_value_input(current_object, 0);
                    type_from_state = state.lookup_state(current_object);
                }
                if !type_from_state.is_set() {
                    return type_from_node;
                }
                return wasm::intersection(type_from_node, type_from_state.type_);
            }
        }

        fn reduce_wasm_struct_operation(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                assert!((*node).opcode() == IrOpcode::WasmStructGet || (*node).opcode() == IrOpcode::WasmStructSet);
                let control = NodeProperties::get_control_input(node);
                if !self.advanced_reducer.is_reduced(control) {
                    return Reduction::NoChange();
                }
                let object = NodeProperties::get_value_input(node, 0);

                let object_type = self.object_type_from_context(object, control, false);
                if object_type.type_.is_uninhabited() {
                    return Reduction::NoChange();
                }

                if object_type.type_.is_non_nullable() {
                    // If the object is known to be non-nullable in the context, remove the null
                    // check.
                    // TODO: OpParameter equivalent in rust
                    // let op_params = OpParameter::<WasmFieldInfo>::new((*node).op());
                    // let new_op = if (*node).opcode() == IrOpcode::WasmStructGet {
                    //     self.simplified_.wasm_struct_get(
                    //         op_params.type,
                    //         op_params.field_index,
                    //         op_params.is_signed,
                    //         NullCheck::kWithoutNullCheck,
                    //     )
                    // } else {
                    //     self.simplified_.wasm_struct_set(
                    //         op_params.type,
                    //         op_params.field_index,
                    //         NullCheck::kWithoutNullCheck,
                    //     )
                    // };
                    // NodeProperties::change_op(node, new_op);
                }

                let mut object_type_mut = object_type.clone(); // Create a mutable copy
                object_type_mut.type_ = object_type.type_.as_non_null();

                let advanced_reducer = &self.advanced_reducer;
                let parent_state = advanced_reducer.get_state(control);
                advanced_reducer.update_states(
                    node,
                    parent_state,
                    object,
                    NodeWithType::new(object, object_type_mut),
                    false,
                )
            }
        }

        fn reduce_wasm_array_length(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                assert_eq!((*node).opcode(), IrOpcode::WasmArrayLength);
                let control = NodeProperties::get_control_input(node);
                if !self.advanced_reducer.is_reduced(control) {
                    return Reduction::NoChange();
                }
                let object = NodeProperties::get_value_input(node, 0);

                let object_type = self.object_type_from_context(object, control, false);
                if object_type.type_.is_uninhabited() {
                    return Reduction::NoChange();
                }

                if object_type.type_.is_non_nullable() {
                    // If the object is known to be non-nullable in the context, remove the null
                    // check.
                    // let new_op = self.simplified_.wasm_array_length(NullCheck::kWithoutNullCheck);
                    // NodeProperties::change_op(node, new_op);
                }

                let mut object_type_mut = object_type.clone(); // Create a mutable copy
                object_type_mut.type_ = object_type.type_.as_non_null();

                let advanced_reducer = &self.advanced_reducer;
                let parent_state = advanced_reducer.get_state(control);
                advanced_reducer.update_states(
                    node,
                    parent_state,
                    object,
                    NodeWithType::new(object, object_type_mut),
                    false,
                )
            }
        }

        // If the condition of this node's branch is a type check or a null check,
        // add the additional information about the type-checked node to the path
        // state.
        fn reduce_if(&mut self, node: *mut Node, condition: bool) -> Reduction {
            unsafe {
                assert!((*node).opcode() == IrOpcode::IfTrue || (*node).opcode() == IrOpcode::IfFalse);
                let branch = NodeProperties::get_control_input(node);
                if (*branch).opcode() == IrOpcode::Dead {
                    return Reduction::NoChange();
                }
                assert_eq!((*branch).opcode(), IrOpcode::Branch);
                if !self.advanced_reducer.is_reduced(branch) {
                    return Reduction::NoChange();
                }
                let parent_state = self.advanced_reducer.get_state(branch);
                let condition_node = NodeProperties::get_value_input(branch, 0);
                match (*condition_node).opcode() {
                    IrOpcode::WasmTypeCheck | IrOpcode::WasmTypeCheckAbstract => {
                        if !condition {
                            return self.advanced_reducer.take_states_from_first_control(node);
                        }
                        let object = NodeProperties::get_value_input(condition_node, 0);
                        let object_type = self.object_type_from_context(object, branch, false);
                        if object_type.type_.is_uninhabited() {
                            return Reduction::NoChange();
                        }

                        // TODO(12166): Think about {module_} below if we have cross-module
                        // inlining.
                        // TODO: OpParameter and WasmTypeCheckConfig equivalents in rust
                        // let to_type = OpParameter::<WasmTypeCheckConfig>::new((*condition_node).op()).to;
                        // let new_type = wasm::intersection(object_type, {to_type, self.module_});
                        // self.update_node_and_aliases_types(node, parent_state, object, new_type, true)
                        Reduction::NoChange() // Placeholder to avoid compilation error
                    }
                    IrOpcode::IsNull | IrOpcode::IsNotNull => {
                        let object = NodeProperties::get_value_input(condition_node, 0);
                        let control = NodeProperties::get_control_input(condition_node);
                        let object_type = self.object_type_from_context(object, control, false);
                        if object_type.type_.is_uninhabited() {
                            return Reduction::NoChange();
                        }
                        // If the checked value is null, narrow the type to the corresponding
                        // null type, otherwise to a non-null reference.
                        let is_null =
                            condition == ((*condition_node).opcode() == IrOpcode::IsNull);

                        let mut object_type_mut = object_type.clone(); // Create a mutable copy
                        object_type_mut.type_ = if is_null {
                            wasm::to_null_sentinel(object_type_mut)
                        } else {
                            object_type.type_.as_non_null()
                        };
                        let advanced_reducer = &self.advanced_reducer;
                        advanced_reducer.update_states(
                            node,
                            parent_state,
                            object,
                            NodeWithType::new(object, object_type_mut),
                            true,
                        )
                    }
                    _ => self.advanced_reducer.take_states_from_first_control(node),
                }
            }
        }

        fn reduce_merge(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                // Shortcut for the case when we do not know anything about some
                // input.
                let inputs = (*node).inputs();
                for input in inputs.iter() {
                    if !self.advanced_reducer.is_reduced(*input) {
                        return Reduction::NoChange();
                    }
                }

                let mut input_it = inputs.iter();

                assert!(inputs.len() > 0);

                let zone_ = unsafe { (&*self.mcgraph_).zone() };
                let mut types = self.advanced_reducer.get_state(*input_it.next().unwrap());

                for input in input_it {
                    // Change the current type block list to a longest common prefix of this
                    // state list and the other list. (The common prefix should correspond to
                    // the state of the common dominator.)
                    // TODO(manoskouk): Consider computing unions for some types.
                    types.reset_to_common_ancestor(&self.advanced_reducer.get_state(*input));
                }

                let advanced_reducer = &self.advanced_reducer;
                advanced_reducer.update_states_initial(node, types)
            }
        }

        fn reduce_assert_not_null(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                assert_eq!((*node).opcode(), IrOpcode::AssertNotNull);
                let object = NodeProperties::get_value_input(node, 0);
                let control = NodeProperties::get_control_input(node);

                let object_type = self.object_type_from_context(object, control, false);
                if object_type.type_.is_uninhabited() {
                    return Reduction::NoChange();
                }

                // Optimize the check away if the argument is known to be non-null.
                if object_type.type_.is_non_nullable() {
                    // First, relax control.
                    self.replace_with_value(node, node, node, control);
                    // Use a TypeGuard node to not lose any type information.
                    // TODO: Common::type_guard and NodeProperties::get_type equivalents in rust
                    // NodeProperties::change_op(
                    //     node,
                    //     self.common_.type_guard(NodeProperties::get_type(node)),
                    // );
                    return Reduction::Changed(node);
                }

                let mut object_type_mut = object_type.clone(); // Create a mutable copy
                object_type_mut.type_ = object_type.type_.as_non_null();

                let advanced_reducer = &self.advanced_reducer;
                let parent_state = advanced_reducer.get_state(control);
                advanced_reducer.update_states(
                    node,
                    parent_state,
                    node,
                    NodeWithType::new(node, object_type_mut),
                    false,
                )
            }
        }

        fn reduce_check_null(&mut self, node: *mut Node) -> Reduction {
            unsafe {
                assert!((*node).opcode() == IrOpcode::IsNull || (*node).opcode() == IrOpcode::IsNotNull);
                let object = NodeProperties::get_value_input(node, 0);
                let control = NodeProperties::get_control_input(node);

                let object_type = self.object_type_from_context(object, control, false);
                if object_type.type_.is_uninhabited() {
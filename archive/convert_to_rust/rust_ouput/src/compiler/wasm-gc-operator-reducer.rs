// Converted from V8 C++ source files:
// Header: wasm-gc-operator-reducer.h
// Implementation: wasm-gc-operator-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

//use crate::v8::wasm;

pub struct V8 {}
pub struct v8 {}
pub enum Reduction {
    kNoChange,
    kChange,
}
pub struct MachineGraph;
pub struct SourcePositionTable;

struct NodeProperties {}

pub enum If {}
pub enum wasm {}

pub struct WasmGraphAssembler {}
pub enum Type {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IrOpcode {
    kStart,
    kWasmStructGet,
    kWasmStructSet,
    kWasmArrayLength,
    kAssertNotNull,
    kIsNull,
    kIsNotNull,
    kWasmTypeCheck,
    kWasmTypeCheckAbstract,
    kWasmTypeCast,
    kWasmTypeCastAbstract,
    kTypeGuard,
    kWasmAnyConvertExtern,
    kMerge,
    kIfTrue,
    kIfFalse,
    kDead,
    kLoop,
    kBranch,
    kDeadValue,
    kNull,
    kWasmExternConvertAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WasmFieldInfo {
    pub type_: i32,
    pub field_index: i32,
    pub is_signed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrapId {
    kTrapIllegalCast,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentifyZeros {
    kIdentifyZeros,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GraphAssemblerLabelType {
    kControl,
    kExit,
    kInput,
}

pub struct Control {
    label: GraphAssemblerLabelType,
}

pub struct Inputs {}
pub struct FeedbackVector {}
pub struct IndirectHandle<T> {
    value: T,
}
impl<T> IndirectHandle<T> {
    pub fn new(value: T) -> Self {
        IndirectHandle { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

pub struct BasicBlock {}
pub struct CommonOperatorBuilder {}
pub struct JSHeapBroker {}

impl CommonOperatorBuilder {
    pub fn TypeGuard(&mut self, _type: Type) -> *mut Operator {
        Box::into_raw(Box::new(Operator {}))
    }
}

pub struct FocusedTree<Key, Value, Hasher> {
    _key: std::marker::PhantomData<Key>,
    _value: std::marker::PhantomData<Value>,
    _hasher: std::marker::PhantomData<Hasher>,
}
pub trait OperatorTrait {
    type Output;
    fn not(self) -> Self::Output;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SourcePosition {
    script_offset: i32,
}

const kNoSourcePosition: i32 = -1;

impl SourcePosition {
    pub fn ScriptOffset(&self) -> i32 {
        self.script_offset
    }
}

pub struct WasmTypeCheckConfig {
    pub from: wasm::ValueType,
    pub to: wasm::ValueType,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Operator {};
pub struct TypeGuard {}
pub struct Isolate {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    kWasmI32,
    kWasmAnyRef,
    kWasmExternRef,
    kWasmVoid,
}

impl ValueType {
    pub fn is_nullable(&self) -> bool {
        true
    }
}

impl wasm {
    pub fn Intersection(a: TypeInModule, b: TypeInModule) -> TypeInModule {
        a // Implement intersection logic here
    }

    pub fn IsHeapSubtypeOf(
        a: i32,
        b: i32,
        _module_a: *const wasm::WasmModule,
        _module_b: *const wasm::WasmModule,
    ) -> bool {
        a == b // Replace with actual subtype checking logic
    }
    pub fn HeapTypesUnrelated(
        a: i32,
        b: i32,
        _module_a: *const wasm::WasmModule,
        _module_b: *const wasm::WasmModule,
    ) -> bool {
        a != b // Replace with actual unrelated types checking logic
    }

    pub fn ToNullSentinel(_object_type: TypeInModule) -> ValueType {
        ValueType::kWasmAnyRef // Implement logic here
    }
}

pub type NodeId = usize;

pub struct Node {
    opcode: IrOpcode,
    id: NodeId,
    inputs: Vec<NodeId>,
    control_input: Option<NodeId>,
    effect_input: Option<NodeId>,
    value_type: Option<ValueType>,
    op: *const Operator,
}

impl Node {
    pub fn new(opcode: IrOpcode, id: NodeId, op: *const Operator) -> Self {
        Node {
            opcode,
            id,
            inputs: Vec::new(),
            control_input: None,
            effect_input: None,
            value_type: None,
            op,
        }
    }

    pub fn opcode(&self) -> IrOpcode {
        self.opcode
    }
    pub fn inputs(&self) -> Inputs {
        Inputs {}
    }

    pub fn op(&self) -> &Operator {
        unsafe { &*self.op }
    }
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn value_type(&self) -> Option<ValueType> {
        self.value_type
    }

    pub fn set_value_type(&mut self, value_type: ValueType) {
        self.value_type = Some(value_type);
    }

    pub fn add_input(&mut self, input: NodeId) {
        self.inputs.push(input);
    }

    pub fn set_control_input(&mut self, control_input: NodeId) {
        self.control_input = Some(control_input);
    }

    pub fn set_effect_input(&mut self, effect_input: NodeId) {
        self.effect_input = Some(effect_input);
    }

    pub fn control_input(&self) -> Option<NodeId> {
        self.control_input
    }

    pub fn effect_input(&self) -> Option<NodeId> {
        self.effect_input
    }

    pub fn remove_input(&mut self, _index: usize) {}
}

pub struct TFGraph {
    nodes: RefCell<HashMap<NodeId, Node>>,
    next_node_id: RefCell<NodeId>,
    zone: Zone,
}

impl TFGraph {
    pub fn new(zone: Zone) -> Self {
        TFGraph {
            nodes: RefCell::new(HashMap::new()),
            next_node_id: RefCell::new(0),
            zone,
        }
    }

    pub fn CreateNode(&self, opcode: IrOpcode, op: *const Operator) -> NodeId {
        let mut next_node_id = self.next_node_id.borrow_mut();
        let node_id = *next_node_id;
        *next_node_id += 1;
        let node = Node::new(opcode, node_id, op);
        self.nodes.borrow_mut().insert(node_id, node);
        node_id
    }

    pub fn node(&self, node_id: NodeId) -> Option<Node> {
        self.nodes.borrow().get(&node_id).cloned()
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }
}

pub struct Editor {
    graph: *mut TFGraph,
    replacements: RefCell<HashMap<NodeId, NodeId>>,
}

impl Editor {
    pub fn new(graph: *mut TFGraph) -> Self {
        Editor {
            graph,
            replacements: RefCell::new(HashMap::new()),
        }
    }

    pub fn ReplaceWithValue(
        &self,
        node_id: NodeId,
        new_node_id: NodeId,
        _effect: NodeId,
        _control: NodeId,
    ) {
        self.replacements.borrow_mut().insert(node_id, new_node_id);
    }

    pub fn Replace(&self, node_id: NodeId, new_node_id: NodeId) {
        self.replacements.borrow_mut().insert(node_id, new_node_id);
    }

    pub fn graph(&self) -> *mut TFGraph {
        self.graph
    }
}

pub struct Zone {
    // A simple zone allocator
}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeInModule {
    pub type_: ValueType,
    pub module: *const wasm::WasmModule,
}

impl TypeInModule {
    pub fn new(type_: ValueType, module: *const wasm::WasmModule) -> Self {
        TypeInModule { type_, module }
    }

    pub fn is_uninhabited(&self) -> bool {
        self.type_ == ValueType::kWasmVoid // Adjust condition based on actual type
    }
    pub fn is_non_nullable(&self) -> bool {
        true // Adjust condition based on actual type
    }

    pub fn AsNonNull(&self) -> ValueType {
        self.type_ // Adjust based on actual logic
    }
}

pub struct MachineType {}

pub struct SimplifiedOperatorBuilder {
    zone: *mut Zone,
}

impl SimplifiedOperatorBuilder {
    pub fn WasmStructGet(
        &self,
        _type: i32,
        _field_index: i32,
        _is_signed: bool,
        _null_check: i32,
    ) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }

    pub fn WasmStructSet(&self, _type: i32, _field_index: i32, _null_check: i32) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }

    pub fn WasmArrayLength(&self, _null_check: i32) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }

    pub fn WasmTypeCast(&self, _config: WasmTypeCheckConfig) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }
    pub fn WasmTypeCheck(&self, _config: WasmTypeCheckConfig) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }
    pub fn WasmTypeCastAbstract(&self, _config: WasmTypeCheckConfig) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }
    pub fn WasmTypeCheckAbstract(&self, _config: WasmTypeCheckConfig) -> *const Operator {
        Box::into_raw(Box::new(Operator {}))
    }
}

pub struct WasmModule {}

pub struct NodeWithType {
    pub node: *mut Node,
    pub type_: TypeInModule,
}

impl NodeWithType {
    pub fn new() -> Self {
        NodeWithType {
            node: std::ptr::null_mut(),
            type_: TypeInModule::new(ValueType::kWasmVoid, std::ptr::null()),
        }
    }

    pub fn with_node_and_type(node: *mut Node, type_: TypeInModule) -> Self {
        NodeWithType { node, type_ }
    }

    pub fn operator_equals(&self, other: &NodeWithType) -> bool {
        self.node == other.node && self.type_ == other.type_
    }

    pub fn operator_not_equals(&self, other: &NodeWithType) -> bool {
        !self.operator_equals(other)
    }

    pub fn IsSet(&self) -> bool {
        self.node != std::ptr::null_mut()
    }
}

#[derive(Clone)]
struct ControlPathState<T, const N: usize> {
    state: HashMap<NodeId, T>,
}

impl<T: Clone, const N: usize> ControlPathState<T, N> {
    fn new(zone: &Zone) -> Self {
        ControlPathState {
            state: HashMap::new(),
        }
    }
    fn ResetToCommonAncestor(&mut self, other: ControlPathState<T, N>) {
        let mut common_state = HashMap::new();
        for (node_id, value) in &self.state {
            if let Some(other_value) = other.state.get(node_id) {
                if *value == *other_value {
                    common_state.insert(*node_id, value.clone());
                }
            }
        }
        self.state = common_state;
    }

    fn LookupState(&self, node: *mut Node) -> T
    where
        T: Default + Clone,
    {
        if let Some(node) = unsafe { node.as_ref() } {
            if let Some(state) = self.state.get(&node.id()) {
                return state.clone();
            }
        }
        T::default()
    }

    fn IsEmpty(&self) -> bool {
        self.state.is_empty()
    }
}

impl<T: Clone + Default, const N: usize> Default for ControlPathState<T, N> {
    fn default() -> Self {
        ControlPathState {
            state: HashMap::new(),
        }
    }
}

const kMultipleInstances: usize = 2;

struct AdvancedReducerWithControlPathState<T, const N: usize> {
    editor: *mut Editor,
    _temp_zone: *mut Zone,
    graph: *mut TFGraph,
    states: RefCell<HashMap<NodeId, ControlPathState<T, N>>>,
}

impl<T: Clone + Default, const N: usize> AdvancedReducerWithControlPathState<T, N> {
    fn new(editor: *mut Editor, temp_zone: *mut Zone, graph: *mut TFGraph) -> Self {
        AdvancedReducerWithControlPathState {
            editor,
            _temp_zone: temp_zone,
            graph,
            states: RefCell::new(HashMap::new()),
        }
    }

    fn GetState(&self, node: *mut Node) -> ControlPathState<T, N> {
        if let Some(node) = unsafe { node.as_ref() } {
            if let Some(state) = self.states.borrow().get(&node.id()) {
                return state.clone();
            }
        }
        ControlPathState::default()
    }

    fn UpdateStates(
        &self,
        state_owner: *mut Node,
        mut parent_state: ControlPathState<T, N>,
        node: *mut Node,
        value: T,
        in_new_block: bool,
    ) -> Reduction {
        if let Some(state_owner) = unsafe { state_owner.as_ref() } {
            if let Some(node) = unsafe { node.as_ref() } {
                if !in_new_block {
                    if let Some(existing_state) = self.states.borrow().get(&state_owner.id()) {
                        if let Some(existing_value) = existing_state.state.get(&node.id()) {
                            if *existing_value == value {
                                return Reduction::kNoChange;
                            }
                        }
                    }
                }

                parent_state.state.insert(node.id(), value);
                self.states.borrow_mut().insert(state_owner.id(), parent_state);
                return Reduction::kChange;
            }
        }
        Reduction::kNoChange
    }

    fn UpdateStates2(
        &self,
        state_owner: *mut Node,
        new_state: ControlPathState<T, N>,
    ) -> Reduction {
        if let Some(state_owner) = unsafe { state_owner.as_ref() } {
            self.states.borrow_mut().insert(state_owner.id(), new_state);
            return Reduction::kChange;
        }
        Reduction::kNoChange
    }

    fn IsReduced(&self, node: *mut Node) -> bool {
        if let Some(node) = unsafe { node.as_ref() } {
            self.states.borrow().contains_key(&node.id())
        } else {
            false
        }
    }

    fn TakeStatesFromFirstControl(&self, node: *mut Node) -> Reduction {
        if let Some(node_ref) = unsafe { node.as_ref() } {
            if node_ref.control_input().is_some() {
                let control_input_id = node_ref.control_input().unwrap();
                if let Some(graph) = unsafe { self.graph.as_ref() } {
                    if let Some(control_input_node) = graph.node(control_input_id) {
                        let state = self.GetState(
                            control_input_node.as_ptr() as *mut Node
                        );
                        return self.UpdateStates2(node, state);
                    }
                }
            }
        }
        Reduction::kNoChange
    }
    fn NoChange(&self) -> Reduction {
        Reduction::kNoChange
    }

    fn Changed(&self, _node: *mut Node) -> Reduction {
        Reduction::kChange
    }

    fn ReplaceWithValue(&self, node: *mut Node, new_node: *mut Node, effect: *mut Node, control: *mut Node) {
        if let Some(editor) = unsafe { self.editor.as_mut() } {
            if let Some(node_ref) = unsafe { node.as_ref() } {
                if let Some(new_node_ref) = unsafe { new_node.as_ref() } {
                     editor.ReplaceWithValue(node_ref.id(), new_node_ref.id(),
                     if let Some(effect_ref) = unsafe { effect.as_ref() } { effect_ref.id() } else { 0 },
                     if let Some(control_ref) = unsafe { control.as_ref() } { control_ref.id() } else { 0 });
                }
            }
        }
    }

    fn Replace(&self, node: *mut Node, new_node: *mut Node) {
        if let Some(editor) = unsafe { self.editor.as_mut() } {
            if let Some(node_ref) = unsafe { node.as_ref() } {
                if let Some(new_node_ref) = unsafe { new_node.as_ref() } {
                    editor.Replace(node_ref.id(), new_node_ref.id());
                }
            }
        }
    }
}

struct WasmGCOperatorReducer {
    base: AdvancedReducerWithControlPathState<NodeWithType, kMultipleInstances>,
    mcgraph_: *mut MachineGraph,
    gasm_: WasmGraphAssembler,
    module_: *const wasm::WasmModule,
    source_position_table_: *mut SourcePositionTable,
}

impl WasmGCOperatorReducer {
    fn new(
        editor: *mut Editor,
        temp_zone_: *mut Zone,
        mcgraph: *mut MachineGraph,
        module: *const wasm::WasmModule,
        source_position_table: *mut SourcePositionTable,
    ) -> Self {
        let graph_ptr = unsafe { (*mcgraph).graph() };
        let zone_ptr = unsafe { (*mcgraph).zone() };
        let gasm_ = WasmGraphAssembler::new(mcgraph, zone_ptr);

        WasmGCOperatorReducer {
            base: AdvancedReducerWithControlPathState::new(editor, temp_zone_, graph_ptr),
            mcgraph_: mcgraph,
            gasm_: gasm_,
            module_: module,
            source_position_table_: source_position_table,
        }
    }

    fn reducer_name(&self) -> &str {
        "WasmGCOperatorReducer"
    }

    fn Reduce(&mut self, node_id: NodeId) -> Reduction {
        let graph = unsafe { (*self.mcgraph_).graph() };
        let node = graph.node(node_id).unwrap();
        match node.opcode() {
            IrOpcode::kStart => self.ReduceStart(node.as_ptr() as *mut Node),
            IrOpcode::kWasmStructGet | IrOpcode::kWasmStructSet => {
                self.ReduceWasmStructOperation(node.as_ptr() as *mut Node)
            }
            IrOpcode::kWasmArrayLength => {
                self.ReduceWasmArrayLength(node.as_ptr() as *mut Node)
            }
            IrOpcode::kAssertNotNull => self.ReduceAssertNotNull(node.as_ptr() as *mut Node),
            IrOpcode::kIsNull | IrOpcode::kIsNotNull => {
                self.ReduceCheckNull(node.as_ptr() as *mut Node)
            }
            IrOpcode::kWasmTypeCheck => self.ReduceWasmTypeCheck(node.as_ptr() as *mut Node),
            IrOpcode::kWasmTypeCheckAbstract => {
                self.ReduceWasmTypeCheckAbstract(node.as_ptr() as *mut Node)
            }
            IrOpcode::kWasmTypeCast => self.ReduceWasmTypeCast(node.as_ptr() as *mut Node),
            IrOpcode::kWasmTypeCastAbstract => {
                self.ReduceWasmTypeCastAbstract(node.as_ptr() as *mut Node)
            }
            IrOpcode::kTypeGuard => self.ReduceTypeGuard(node.as_ptr() as *mut Node),
            IrOpcode::kWasmAnyConvertExtern => {
                self.ReduceWasmAnyConvertExtern(node.as_ptr() as *mut Node)
            }
            IrOpcode::kMerge => self.ReduceMerge(node.as_ptr() as *mut Node),
            IrOpcode::kIfTrue => self.ReduceIf(node.as_ptr() as *mut Node, true),
            IrOpcode::kIfFalse => self.ReduceIf(node.as_ptr() as *mut Node, false),
            IrOpcode::kDead => self.base.NoChange(),
            IrOpcode::kLoop => self
                .base
                .TakeStatesFromFirstControl(node.as_ptr() as *mut Node),
            _ => {
                if true {
                    //node.op().ControlOutputCount() > 0
                    //DCHECK_EQ(1, node.op().ControlInputCount());
                    self.base
                        .TakeStatesFromFirstControl(node.as_ptr() as *mut Node)
                } else {
                    self.base.NoChange()
                }
            }
        }
    }

    fn ReduceWasmStructOperation(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if (*node).opcode() != IrOpcode::kWasmStructGet && (*node).opcode() != IrOpcode::kWasmStructSet {
                return self.base.NoChange();
            }

            let control_id = (*node).control_input().unwrap();
            let graph = (*self.mcgraph_).graph();
            let control = graph.node(control_id).unwrap().as_ptr() as *mut Node;
            if !self.base.IsReduced(control) {
                return self.base.NoChange();
            }

            let object_id = (*node).inputs.get(0).unwrap();
            let object = graph.node(*object_id).unwrap().as_ptr() as *mut Node;

            let mut object_type = self.ObjectTypeFromContext(object, control, false);
            if object_type.type_.is_uninhabited() {
                return self.base.NoChange();
            }

            if object_type.type_.is_non_nullable() {
                // If the object is known to be non-nullable in the context, remove the null
                // check.
                let op_params = WasmFieldInfo {
                    type_: 0,
                    field_index: 0,
                    is_signed: false,
                };
                let simplified = self.gasm_.simplified();
                let new_op = if (*node).opcode() == IrOpcode::kWasmStructGet {
                   simplified.WasmStructGet(
                        op_params.type_,
                        op_params.field_index,
                        op_params.is_signed,
                        0,
                    )
                } else {
                    simplified.WasmStructSet(op_params.type_, op_params.field_index, 0)
                };
                (*node).op = new_op;
            }

            object_type.type_ = object_type.type_.AsNonNull();

            self.UpdateNodeAndAliasesTypes(node, self.base.GetState(control), object, object_type, false)
        }
    }

    fn ReduceWasmArrayLength(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if (*node).opcode() != IrOpcode::kWasmArrayLength {
                return self.base.NoChange();
            }

            let control_id = (*node).control_input().unwrap();
            let graph = (*self.mcgraph_).graph();
            let control = graph.node(control_id).unwrap().as_ptr() as *mut Node;

            if !self.base.IsReduced(control) {
                return self.base.NoChange();
            }

             let object_id = (*node).inputs.get(0).unwrap();
            let object = graph.node(*object_id).unwrap().as_ptr() as *mut Node;

            let mut object_type = self.ObjectTypeFromContext(object, control, false);
            if object_type.type_.is_uninhabited() {
                return self.base.NoChange();
            }

            if object_type.type_.is_non_nullable() {
                // If the object is known to be non-nullable in the context, remove the null
                // check.
                let new_op = self.gasm_.simplified().WasmArrayLength(0);
                (*node).op = new_op;
            }

            object_type.type_ = object_type.type_.AsNonNull();

            self.UpdateNodeAndAliasesTypes(node, self.base.GetState(control), object, object_type, false)
        }
    }

    // If the condition of this node's branch is a type check or a null check,
    // add the additional information about the type-checked node to the path
    // state.
    fn ReduceIf(&mut self, node: *mut Node, condition: bool) -> Reduction {
        unsafe {
            if (*node).opcode() != IrOpcode::kIfTrue && (*node).opcode() != IrOpcode::kIfFalse {
                return self.base.NoChange();
            }

            let branch_id = (*node).control_input().unwrap();
            let graph = (*self.mcgraph_).graph();
            let branch = graph.node(branch_id).unwrap().as_ptr() as *mut Node;

            if (*branch).opcode() == IrOpcode::kDead {
                return self.base.NoChange();
            }
            if (*branch).opcode() != IrOpcode::kBranch {
                return self.base.NoChange();
            }

            if !self.base.IsReduced(branch) {
                return self.base.NoChange();
            }

            let parent_state = self.base.GetState(branch);
            let condition_node_id = (*branch).inputs.get(0).unwrap();
            let condition_node = graph.node(*condition_node_id).unwrap().as_ptr() as *mut Node;
            match (*condition_node).opcode() {
                IrOpcode::kWasmTypeCheck | IrOpcode::kWasmTypeCheckAbstract => {
                    if !condition {
                        return self.base.TakeStatesFromFirstControl(node);
                    }

                    let object_id = (*condition_node).inputs.get(0).unwrap();
                    let object = graph.node(*object_id).unwrap().as_ptr() as *mut Node;
                    let object_type = self.ObjectTypeFromContext(object, branch, false);
                    if object_type.type_.is_uninhabited() {
                        return self.base.NoChange();
                    }

                    let to_type = WasmTypeCheckConfig {
                        from: ValueType::kWasmI32,
                        to: ValueType::kWasmAnyRef,
                    };

                    let mut new_type = wasm::Intersection(
                        object_type,
                        TypeInModule {
                            type_: to_type.to,
                            module: self.module_,
                        },
                    );
                    self.UpdateNodeAndAliasesTypes(node, parent_state, object, new_type, true)
                }
                IrOpcode::kIsNull | IrOpcode::kIsNotNull => {
                    let object_id = (*condition_node).inputs.get(0).unwrap();
                    let object = graph.node(*object_id).unwrap().as_ptr() as *mut Node;

                    let control_id = (*condition_node).control_input().unwrap();
                    let control = graph.node(control_id).unwrap().as_ptr() as *mut Node;
                    let mut object_type = self.ObjectTypeFromContext(object, control, false);
                    if object_type.type_.is_uninhabited() {
                        return self.base.NoChange();
                    }

                    // If the checked value is null, narrow the type to the corresponding
                    // null type, otherwise to a non-null reference.
                    let is_null =
                        condition == ((*condition_node).opcode() == IrOpcode::kIsNull);
                    object_type.type_ = if is_null {
                        wasm::ToNullSentinel(object_type)
                    } else {
                        object_type.type_.AsNonNull()
                    };
                    self.UpdateNodeAndAliasesTypes(node, parent_state, object, object_type, true)
                }
                _ => self.base.TakeStatesFromFirstControl(node),
            }
        }
    }

    fn ReduceMerge(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            let graph = (*self.mcgraph_).graph();
            // Shortcut for the case when we do not know anything about some
            // input.
             let inputs = (*node).inputs.clone();

            for input_id in &inputs {
               let input_node = graph.node(*input_id).unwrap().as_ptr() as *mut Node;
                if !self.base.IsReduced(input_node) {
                    return self.base.NoChange();
                }
            }

            if inputs.is_empty() {
                return self.base.NoChange();
            }

            let mut input_iter = inputs.iter();
             let first_input_id = input_iter.next().unwrap();
            let first_input_node = graph.node(*first_input_id).unwrap().as_ptr() as *mut Node;

            let mut types = self.base.GetState(first_input_node);
             for input_id in input_iter {
                let input_node = graph.node(*input_id).unwrap().as_ptr() as *mut Node;
                types.ResetToCommonAncestor(self.base.GetState(input_node));
            }
            self.UpdateStates(node, types)
        }
    }

    fn ReduceAssertNotNull(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if (*node).opcode() != IrOpcode::kAssertNotNull {
                return self.base.NoChange();
            }

             let graph = (*self.mcgraph_).graph();

            let object_id = (*node).inputs.get(0).unwrap();
            let object = graph.node(*object_id).unwrap().as_ptr() as *mut Node;

            let control_id = (*node).control_input().unwrap();
            let control = graph.node(control_id).unwrap().as_ptr() as *mut Node;

            let mut object_type = self.ObjectTypeFromContext(object, control, false);
            if object_type.type_.is_uninhabited() {
                return self.base.NoChange();
            }

            // Optimize the check away if the argument is known to be non-null.
            if object_type.type_.is_non_nullable() {
                // First, relax control.
                let editor = unsafe { (*self.base.editor).graph() };
                let graph = unsafe { editor.as_ref().unwrap() };
                let control_node = graph.node(control_id).unwrap().as_ptr() as *mut Node;
                self.base.ReplaceWithValue(node, node, node, control_node);
                // Use a TypeGuard node to not lose any type information.
                (*node).op = unsafe {
                    (*self.mcgraph_).common().TypeGuard(Type {})
                };
                return self.base.Changed(node);
            }

            object_type.type_ = object_type.type_.AsNonNull();
            self.UpdateNodeAndAliasesTypes(node, self.base.GetState(control), node, object_type, false)
        }
    }

    fn ReduceCheckNull(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            if (*node).opcode() != IrOpcode::kIsNull && (*node).opcode() != IrOpcode::kIsNotNull {
                return self.base.NoChange();
            }

            let graph = (*self.mcgraph_).graph();



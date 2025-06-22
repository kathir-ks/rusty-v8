// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/graph-assembler.h

use std::{
    any::Any,
    borrow::Borrow,
    cell::{Cell, RefCell},
    collections::HashSet,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// Placeholder for base::SmallVector
mod base {
    pub struct SmallVector<T, const N: usize> {
        data: Vec<T>,
    }

    impl<T, const N: usize> SmallVector<T, const N: usize> {
        pub fn new() -> Self {
            SmallVector { data: Vec::new() }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            SmallVector {
                data: Vec::with_capacity(capacity),
            }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }

        pub fn data(&self) -> &Vec<T> {
            &self.data
        }

        pub fn data_mut(&mut self) -> &mut Vec<T> {
            &mut self.data
        }
    }

    impl<T, const N: usize> Deref for SmallVector<T, N> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T, const N: usize> DerefMut for SmallVector<T, N> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }
}

// Placeholder for src/codegen/tnode.h
mod codegen {
    pub struct TNode<T> {
        // Placeholder, add fields as needed
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> TNode<T> {
        pub fn unchecked_cast(node: *mut crate::compiler::Node) -> Self {
            // Implement the unchecked cast logic here, if needed
            TNode {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub trait MachineTypeTrait {
        const REPRESENTATION: MachineRepresentation;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MachineRepresentation {
        Tagged,
        Word32,
        Word64,
        Float64,
        // Add other representations as needed
    }

    pub type Word32T = i32;
    pub type Uint32T = u32;
    pub type BoolT = bool;
    pub type Uint64T = u64;
    pub type Int32T = i32;
    pub type Int64T = i64;
    pub type UintPtrT = usize;
    pub type RawPtrT = *mut u8;
}

// Placeholder for src/common/globals.h
mod common {
    pub const kSystemPointerSize: usize = 8; // Or 4, depending on the target architecture
}

// Placeholder for src/compiler/feedback-source.h
mod compiler {
    #[derive(Clone, Copy)]
    pub struct FeedbackSource {}
}

// Placeholder for src/compiler/js-graph.h
mod compiler_jsgraph {
    use crate::compiler::{SimplifiedOperatorBuilder, TFGraph};

    pub struct JSGraph {
        // Placeholder, add fields as needed
        simplified: SimplifiedOperatorBuilder,
        graph: TFGraph,
    }

    impl JSGraph {
        pub fn simplified(&self) -> &SimplifiedOperatorBuilder {
            &self.simplified
        }
        pub fn graph(&self) -> &TFGraph {
            &self.graph
        }

        pub fn isolate(&self) -> *mut Isolate {
            std::ptr::null_mut() // Placeholder, replace with the correct value
        }
    }

    // Placeholder for Isolate
    pub struct Isolate {}

    pub struct JSHeapBroker {}
}

// Placeholder for src/compiler/node.h
mod compiler_node {
    use crate::{
        codegen::MachineRepresentation,
        compiler::{TFGraph, Zone},
    };
    use std::{any::Any, cell::RefCell, rc::Rc};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IrOpcode {
        Start,
        End,
        Loop,
        Merge,
        Branch,
        IfTrue,
        IfFalse,
        EffectPhi,
        Phi,
        NumberConstant,
        Int32Constant,
        // Add other opcodes as needed
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Type {
        Any,
        Number,
        Boolean,
        // Add more types as needed
    }

    pub struct Node {
        id: usize, //added for easier debugging
        opcode: IrOpcode,
        inputs: Vec<*mut Node>,
        // Placeholder, add fields as needed
        op: *const Operator, //raw pointer because it's borrowed
        repr: MachineRepresentation,
        node_properties: NodeProperties,
    }

    impl Node {
        pub fn new(op: *const Operator, inputs: Vec<*mut Node>, repr: MachineRepresentation) -> Self {
            Node {
                id: unsafe { NEXT_NODE_ID += 1; NEXT_NODE_ID },
                opcode: IrOpcode::NumberConstant, // Placeholder, replace with actual opcode
                inputs,
                op,
                repr,
                node_properties: NodeProperties::new(),
            }
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
        pub fn op(&self) -> &Operator {
            unsafe { &*self.op }
        }

        pub fn effect_output_count(&self) -> usize {
            0
        }
        pub fn control_output_count(&self) -> usize {
            0
        }

        pub fn replace_input(&mut self, index: usize, new_input: *mut Node) {
            if index < self.inputs.len() {
                self.inputs[index] = new_input;
            }
        }

        pub fn append_input(&mut self, _zone: &mut Zone, new_input: *mut Node) {
            self.inputs.push(new_input);
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct NodeProperties {
        typed: bool,
        node_type: Type,
    }

    impl NodeProperties {
        pub fn new() -> Self {
            NodeProperties {
                typed: false,
                node_type: Type::Any,
            }
        }
    }

    pub struct NodePropertiesMethods;
    impl NodePropertiesMethods {
        pub fn is_typed(node: *mut Node) -> bool {
            unsafe {
                (*node).node_properties.typed
            }
        }
        pub fn change_op(node: *mut Node, op: *const Operator) {
            unsafe {
                (*node).op = op;
            }
        }
        pub fn set_type(node: *mut Node, new_type: Type) {
            unsafe {
                (*node).node_properties.typed = true;
                (*node).node_properties.node_type = new_type;
            }
        }

        pub fn get_type(node: *mut Node) -> Type {
            unsafe {
                (*node).node_properties.node_type
            }
        }
    }
    static mut NEXT_NODE_ID: usize = 0;
}

// Placeholder for src/compiler/simplified-operator.h
mod compiler_simplified_operator {
    pub struct SimplifiedOperatorBuilder {}

    impl SimplifiedOperatorBuilder {
        // Add methods as needed
    }
}

// Placeholder for src/objects/hole.h
mod objects {
    pub struct Hole {}
    pub struct Oddball {}
    pub struct HeapObject {}
    pub struct Map {}
    pub struct String {}
    pub struct Boolean {}
    pub struct Number {}
    pub struct Null {}
    pub struct Undefined {}
    pub struct FixedArrayMap {}
    pub struct FixedDoubleArrayMap {}
    pub struct WeakFixedArrayMap {}
    pub struct InstructionStream {}
    pub struct ExternalObjectMap {}
    pub struct BigIntMap {}
    pub struct Context {}
    pub struct JSArray {}
    pub struct FixedArrayBase {}
    pub struct JSArrayBufferView {}
    pub struct JSTypedArray {}
}

//--------------------------------------- ACTUAL IMPLEMENTATION ---------------------------------------

use crate::{
    base::SmallVector,
    codegen::{MachineRepresentation, TNode, Uint32T, UintPtrT, Word32T, BoolT, Uint64T, Int32T, Int64T, RawPtrT},
    common::kSystemPointerSize,
    compiler::{
        compiler_jsgraph::{JSGraph, JSHeapBroker, Isolate},
        compiler_node::{IrOpcode, Node, NodePropertiesMethods, Type},
        compiler_simplified_operator::SimplifiedOperatorBuilder,
        FeedbackSource,
    },
    compiler_jsgraph::JSGraph,
    objects::*,
};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashSet,
    mem,
    ops::{Deref, DerefMut},
    rc::Rc,
};

macro_rules! pure_assembler_mach_unop_list {
    ($V:ident) => {
        $V!(BitcastFloat32ToInt32);
        $V!(BitcastFloat64ToInt64);
        $V!(BitcastInt32ToFloat32);
        $V!(BitcastWord32ToWord64);
        $V!(BitcastInt64ToFloat64);
        $V!(ChangeFloat32ToFloat64);
        $V!(ChangeFloat64ToInt32);
        $V!(ChangeFloat64ToInt64);
        $V!(ChangeFloat64ToUint32);
        $V!(ChangeFloat64ToUint64);
        $V!(ChangeInt32ToFloat64);
        $V!(ChangeInt32ToInt64);
        $V!(ChangeInt64ToFloat64);
        $V!(ChangeUint32ToFloat64);
        $V!(ChangeUint32ToUint64);
        $V!(Float64Abs);
        $V!(Float64ExtractHighWord32);
        $V!(Float64ExtractLowWord32);
        $V!(Float64SilenceNaN);
        $V!(RoundFloat64ToInt32);
        $V!(RoundInt32ToFloat32);
        $V!(TruncateFloat64ToFloat32);
        $V!(TruncateFloat64ToWord32);
        $V!(TruncateInt64ToInt32);
        $V!(TryTruncateFloat64ToInt64);
        $V!(TryTruncateFloat64ToUint64);
        $V!(TryTruncateFloat64ToInt32);
        $V!(TryTruncateFloat64ToUint32);
        $V!(Word32ReverseBytes);
        $V!(Word64ReverseBytes);
    };
}

macro_rules! pure_assembler_mach_binop_list {
    ($V:ident, $T:ident) => {
        $V!(Float64Add);
        $V!(Float64Div);
        $V!(Float64Equal);
        $V!(Float64InsertHighWord32);
        $V!(Float64InsertLowWord32);
        $V!(Float64LessThan);
        $V!(Float64LessThanOrEqual);
        $V!(Float64Max);
        $V!(Float64Min);
        $V!(Float64Mod);
        $V!(Float64Sub);
        $V!(Int32Add);
        $V!(Int32LessThan);
        $T!(Int32LessThanOrEqual, BoolT, Int32T, Int32T);
        $V!(Int32Mul);
        $V!(Int32Sub);
        $V!(Int64Add);
        $V!(Int64Sub);
        $V!(IntAdd);
        $V!(IntLessThan);
        $V!(IntMul);
        $V!(IntSub);
        $T!(Uint32LessThan, BoolT, Uint32T, Uint32T);
        $T!(Uint32LessThanOrEqual, BoolT, Uint32T, Uint32T);
        $T!(Uint64LessThan, BoolT, Uint64T, Uint64T);
        $T!(Uint64LessThanOrEqual, BoolT, Uint64T, Uint64T);
        $V!(UintLessThan);
        $T!(Word32And, Word32T, Word32T, Word32T);
        $T!(Word32Equal, BoolT, Word32T, Word32T);
        $T!(Word32Or, Word32T, Word32T, Word32T);
        $V!(Word32Sar);
        $V!(Word32SarShiftOutZeros);
        $V!(Word32Shl);
        $T!(Word32Shr, Word32T, Word32T, Word32T);
        $V!(Word32Xor);
        $V!(Word64And);
        $V!(Word64Equal);
        $V!(Word64Or);
        $V!(Word64Sar);
        $V!(Word64SarShiftOutZeros);
        $V!(Word64Shl);
        $V!(Word64Shr);
        $V!(Word64Xor);
        $V!(WordAnd);
        $V!(WordEqual);
        $V!(WordOr);
        $V!(WordSar);
        $V!(WordSarShiftOutZeros);
        $V!(WordShl);
        $V!(WordShr);
        $V!(WordXor);
    };
}

macro_rules! checked_assembler_mach_binop_list {
    ($V:ident) => {
        $V!(Int32AddWithOverflow);
        $V!(Int64AddWithOverflow);
        $V!(Int32Div);
        $V!(Int32Mod);
        $V!(Int32MulWithOverflow);
        $V!(Int64MulWithOverflow);
        $V!(Int32SubWithOverflow);
        $V!(Int64SubWithOverflow);
        $V!(Int64Div);
        $V!(Int64Mod);
        $V!(Uint32Div);
        $V!(Uint32Mod);
        $V!(Uint64Div);
        $V!(Uint64Mod);
    };
}

macro_rules! jsgraph_singleton_constant_list {
    ($V:ident) => {
        $V!(AllocateInOldGenerationStub, InstructionStream);
        $V!(AllocateInYoungGenerationStub, InstructionStream);
        $V!(BigIntMap, Map);
        $V!(BooleanMap, Map);
        $V!(EmptyString, String);
        $V!(ExternalObjectMap, Map);
        $V!(False, Boolean);
        $V!(FixedArrayMap, Map);
        $V!(FixedDoubleArrayMap, Map);
        $V!(WeakFixedArrayMap, Map);
        $V!(HeapNumberMap, Map);
        $V!(MinusOne, Number);
        $V!(NaN, Number);
        $V!(NoContext, Object);
        $V!(Null, Null);
        $V!(One, Number);
        $V!(TheHole, Hole);
        $V!(ToNumberBuiltin, InstructionStream);
        $V!(PlainPrimitiveToNumberBuiltin, InstructionStream);
        $V!(True, Boolean);
        $V!(Undefined, Undefined);
        $V!(Zero, Number);
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphAssemblerLabelType {
    kDeferred,
    kNonDeferred,
    kLoop,
}

mod detail {
    use crate::{
        base::SmallVector,
        codegen::MachineRepresentation,
        compiler::Node,
    };
    use std::array;

    pub const kGraphAssemblerLabelDynamicCount: usize = usize::MAX - 1;

    pub struct GraphAssemblerHelper<const VarCount: usize>;

    impl GraphAssemblerHelper< { usize::MAX - 1 }> {
        // TODO(leszeks): We could allow other sizes of small vector here, by encoding
        // the size in the negative VarCount.
        pub type Array<T> = SmallVector<T, 4>;
        pub const kIsDynamic: bool = true;

        pub fn init_node_array(reps: &SmallVector<MachineRepresentation, 4>) -> SmallVector<*mut Node, 4> {
            SmallVector::with_capacity(reps.len()) //Array<Node*>
        }
    }
}

/// Label with statically known count of incoming branches and phis.
pub struct GraphAssemblerLabel<const VarCount: usize> {
    // type Helper = detail::GraphAssemblerHelper<VarCount>;
    // type Array<T> = <Helper as GraphAssemblerHelper<VarCount>>::Array<T>;
    // static constexpr bool kIsDynamic = Helper::kIsDynamic;

    type Array<T> = Vec<T>; // Placeholder
    pub representations_: Self::Array<MachineRepresentation>,
    bindings_: Self::Array<*mut Node>, //Node*
    pub type_: GraphAssemblerLabelType,
    pub loop_nesting_level_: i32,
    pub merged_count_: usize,
    pub is_bound_: bool,
    pub effect_: *mut Node,   //Node*
    pub control_: *mut Node,  //Node*
}

impl GraphAssemblerLabel< { usize::MAX - 1 }> {
    //TODO: Check this
    pub fn new(
        type_: GraphAssemblerLabelType,
        loop_nesting_level_: i32,
        reps: SmallVector<MachineRepresentation, 4>,
    ) -> Self {
        let bindings_ = detail::GraphAssemblerHelper::<{ usize::MAX - 1 }>::init_node_array(&reps);
        GraphAssemblerLabel {
            type_,
            loop_nesting_level_: loop_nesting_level_,
            bindings_: bindings_,
            representations_: reps,
            merged_count_: 0,
            is_bound_: false,
            effect_: std::ptr::null_mut(),
            control_: std::ptr::null_mut(),
        }
    }
}

impl<const VarCount: usize> GraphAssemblerLabel<VarCount> {
    //type Array<T> = Vec<T>;
    pub fn count(&self) -> usize {
        self.representations_.len()
    }

    pub fn phi_at(&self, index: usize) -> *mut Node {
        //Node*
        assert!(self.is_bound_);
        assert!(index < self.count());
        self.bindings_[index]
    }

    pub fn phi_at_typed<T>(&self, index: usize) -> TNode<T> {
        // TODO(jgruber): Investigate issues on ptr compression bots and enable.
        // DCHECK(IsMachineRepresentationOf<T>(representations_[index]));
        TNode::<T>::unchecked_cast(self.phi_at(index))
    }

    pub fn is_used(&self) -> bool {
        self.merged_count_ > 0
    }

    pub fn is_bound(&self) -> bool {
        self.is_bound_
    }
    pub fn is_deferred(&self) -> bool {
        self.type_ == GraphAssemblerLabelType::kDeferred
    }
    pub fn is_loop(&self) -> bool {
        self.type_ == GraphAssemblerLabelType::kLoop
    }

    // private:
    // friend class GraphAssembler;

    pub fn set_bound(&mut self) {
        assert!(!self.is_bound());
        self.is_bound_ = true;
    }
}

impl<const VarCount: usize> Drop for GraphAssemblerLabel<VarCount> {
    fn drop(&mut self) {
        assert!(self.is_bound() || self.merged_count_ == 0);
    }
}

//TODO: Fix Dynamic Label
pub type GraphAssemblerDynamicLabel = GraphAssemblerLabel< { usize::MAX - 1 }>;

mod detail_graphassembler {
    use crate::{
        base::SmallVector,
        codegen::MachineRepresentation,
        compiler::{GraphAssemblerDynamicLabel, GraphAssemblerLabel, Node},
    };
    use std::{marker::PhantomData, any::Any};

    pub struct GraphAssemblerLabelForXHelper<T, Enable, Us> {
        _phantom: PhantomData<(T, Enable, Us)>,
    }

    impl<T, Us> GraphAssemblerLabelForXHelper<T, (), Us> {
        pub type Type = GraphAssemblerLabel<0>; // Placeholder
    }

    pub struct GraphAssemblerLabelForVars<Vars> {
        _phantom: PhantomData<Vars>,
    }
    impl GraphAssemblerLabelForVars<()> {
        pub type Type = GraphAssemblerLabel<0>;
    }
    impl GraphAssemblerLabelForVars<(*mut Node,)> {
        pub type Type = GraphAssemblerLabel<1>;
    }
    impl GraphAssemblerLabelForVars<(*mut Node, *mut Node)> {
        pub type Type = GraphAssemblerLabel<2>;
    }
    impl GraphAssemblerLabelForVars<(*mut Node, *mut Node, *mut Node)> {
        pub type Type = GraphAssemblerLabel<3>;
    }
    impl GraphAssemblerLabelForVars<(*mut Node, *mut Node, *mut Node, *mut Node)> {
        pub type Type = GraphAssemblerLabel<4>;
    }

    pub struct GraphAssemblerLabelForReps<Reps> {
        _phantom: PhantomData<Reps>,
    }

    impl GraphAssemblerLabelForReps<()> {
        pub type Type = GraphAssemblerLabel<0>;
    }

    impl GraphAssemblerLabelForReps<(MachineRepresentation,)> {
        pub type Type = GraphAssemblerLabel<1>;
    }

    impl GraphAssemblerLabelForReps<(MachineRepresentation, MachineRepresentation)> {
        pub type Type = GraphAssemblerLabel<2>;
    }

    impl GraphAssemblerLabelForReps<(MachineRepresentation, MachineRepresentation, MachineRepresentation)> {
        pub type Type = GraphAssemblerLabel<3>;
    }

    impl GraphAssemblerLabelForReps<(MachineRepresentation, MachineRepresentation, MachineRepresentation, MachineRepresentation)> {
        pub type Type = GraphAssemblerLabel<4>;
    }
}

pub type NodeChangedCallback = Box<dyn FnMut(*mut Node)>; //std::function<void(Node*)>

#[derive(Debug, Clone, Copy)]
pub enum BranchSemantics {
    kJS,
    kMachine,
    kNone, //Added to represent the null branch semantics in C++
}

#[derive(Debug, Clone, Copy)]
pub enum BranchHint {
    kTrue,
    kFalse,
    kNone,
}

pub struct GraphAssembler {
    mcgraph_: *mut MachineGraph, //MachineGraph*
    default_branch_semantics_: BranchSemantics,
    effect_: *mut Node,         //Node*
    control_: *mut Node,        //Node*
    // {node_changed_callback_} should be called when a node outside the
    // subgraph created by the graph assembler changes.
    node_changed_callback_: Option<NodeChangedCallback>,
    // Inline reducers enable reductions to be performed to nodes as they are
    // added to the graph with the graph assembler.
    inline_reducers_: Vec<*mut Reducer>, //ZoneVector<Reducer*>
    inline_reductions_blocked_: bool,
    // Track loop information in order to properly mark loop exits with
    // {LoopExit,LoopExitEffect,LoopExitValue} nodes. The outermost level has
    // a nesting level of 0. See also GraphAssembler::LoopScope.
    loop_nesting_level_: i32,
    loop_headers_: Vec<*mut *mut Node>, //ZoneVector<Node**>
    // Feature configuration. As more features are added, this should be turned
    // into a bitfield.
    mark_loop_exits_: bool,
    temp_zone_: Zone,
}

impl GraphAssembler {
    /// Constructs a GraphAssembler. If {schedule} is not null, the graph assembler
    /// will maintain the schedule as it updates blocks.
    pub fn new(
        jsgraph: *mut MachineGraph, //MachineGraph*
        zone: *mut Zone,            //Zone*
        default_branch_semantics: BranchSemantics,
        node_changed_callback: Option<NodeChangedCallback>,
        mark_loop_exits: bool,
    ) -> Self {
        let mut temp_zone_ = Zone::new();
        let mut gasm = GraphAssembler {
            mcgraph_: jsgraph,
            default_branch_semantics_: default_branch_semantics,
            effect_: std::ptr::null_mut(),
            control_: std::ptr::null_mut(),
            node_changed_callback_: node_changed_callback,
            inline_reducers_: Vec::new(),
            inline_reductions_blocked_: false,
            loop_nesting_level_: 0,
            loop_headers_: Vec::new(),
            mark_loop_exits_: mark_loop_exits,
            temp_zone_,
        };
        gasm
    }

    pub fn reset(&mut self) {
        todo!()
    }
    pub fn initialize_effect_control(&mut self, effect: *mut Node, control: *mut Node) {
        self.effect_ = effect;
        self.control_ = control;
    }

    pub fn make_label_for<const N: usize>(
        &mut self,
        type_: GraphAssemblerLabelType,
        reps: [MachineRepresentation; N],
    ) -> GraphAssemblerLabel<N> {
        let reps_array: Vec<MachineRepresentation> = reps.to_vec();
        GraphAssemblerLabel {
            type_: type_,
            loop_nesting_level_: self.loop_nesting_level_,
            bindings_: vec![std::ptr::null_mut(); reps_array.len()],
            representations_: reps_array,
            merged_count_: 0,
            is_bound_: false,
            effect_: std::ptr::null_mut(),
            control_: std::ptr::null_mut(),
        }
    }

    pub fn make_label_for_dynamic(
        &mut self,
        type_: GraphAssemblerLabelType,
        reps: SmallVector<MachineRepresentation, 4>,
    ) -> GraphAssemblerDynamicLabel {
        GraphAssemblerDynamicLabel::new(type_, self.loop_nesting_level_, reps)
    }

    // Convenience wrapper for creating non-deferred labels.
    pub fn make_label<const N: usize>(
        &mut self,
        reps: [MachineRepresentation; N],
    ) -> GraphAssemblerLabel<N> {
        self.make_label_for(GraphAssemblerLabelType::kNonDeferred, reps)
    }

    // Convenience wrapper for creating loop labels.
    pub fn make_loop_label<const N: usize>(
        &mut self,
        reps: [MachineRepresentation; N],
    ) -> GraphAssemblerLabel<N> {
        self.make_label_for(GraphAssemblerLabelType::kLoop, reps)
    }

    // Convenience wrapper for creating deferred labels.
    pub fn make_deferred_label<const N: usize>(
        &mut self,
        reps: [MachineRepresentation; N],
    ) -> GraphAssemblerLabel<N> {
        self.make_label_for(GraphAssemblerLabelType::kDeferred, reps)
    }

    // Value creation.
    pub fn intptr_constant(&mut self, value: isize) -> *mut Node {
        todo!()
    }
    pub fn uintptr_constant(&mut self, value: usize) -> TNode<UintPtrT> {
        todo!()
    }
    pub fn int32_constant(&mut self, value: i32) -> *mut Node {
        todo!()
    }
    pub fn uint32_constant(&mut self, value: u32) -> TNode<Uint32T> {
        todo!()
    }
    pub fn int64_constant(&mut self, value: i64) -> *mut Node {
        todo!()
    }
    pub fn uint64_constant(&mut self, value: u64) -> *mut Node {
        todo!()
    }
    pub fn unique_intptr_constant(&mut self, value: isize) -> *mut Node {
        todo!()
    }
    pub fn float64_constant(&mut self, value: f64) -> *mut Node {
        todo!()
    }
    pub fn external_constant(&mut self, ref_: ExternalReference) -> *mut Node {
        todo!()
    }
    pub fn isolate_field(&mut self, id: IsolateFieldId) -> *mut Node {
        todo!()
    }

    pub fn projection(&mut self, index: i32, value: *mut Node, ctrl: *mut Node) -> *mut Node {
        todo!()
    }

    pub fn parameter(&mut self, index: i32) -> *mut Node {
        todo!()
    }

    pub fn load_frame_pointer(&mut self) -> *mut Node {
        todo!()
    }

    pub fn load_root_register(&mut self) -> *mut Node {
        todo!()
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn load_stack_pointer(&mut self) -> *mut Node {
        todo!()
    }
    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn set_stack_pointer(&mut self, sp: *mut Node) -> *mut Node {
        todo!()
    }

    pub fn load_heap_number_value(&mut self, heap_number: *mut Node) -> *mut Node {
        todo!()
    }

    macro_rules! declare_pure_unop {
        ($Name:ident) => {
            pub fn $Name(&mut self, input: *mut Node) -> *mut Node {
                todo!()
            }
        };
    }
    pure_assembler_mach_unop_list!(declare_pure_unop);

    macro_rules! declare_binop {
        ($Name:ident) => {
            pub fn $Name(&mut self, left: *mut Node, right: *mut Node) -> *mut Node {
                todo!()
            }
        };
    }

    macro_rules! declare_binop_tnode {
        ($Name:ident, $Result:ident, $Left:ident, $Right:ident) => {
            pub fn $Name(
                &mut self,
                left: TNode<$Left>,
                right: TNode<$Right>,
            ) -> TNode<$Result> {
                todo!()
            }
        };
    }

    pure_assembler_mach_binop_list!(declare_binop, declare_binop_tnode);
    checked_assembler_mach_binop_list!(declare_binop);

    pub fn uintptr_less_than(&mut self, left: TNode<UintPtrT>, right: TNode<UintPtrT>) -> TNode<BoolT> {
        todo!()
    }
    pub fn uintptr_less_than_or_equal(
        &mut self,
        left: TNode<UintPtrT>,
        right: TNode<UintPtrT>,
    ) -> TNode<BoolT> {
        todo!()
    }
    pub fn uintptr_add(&mut self, left: TNode<UintPtrT>, right: TNode<UintPtrT>) -> TNode<UintPtrT> {
        todo!()
    }
    pub fn uintptr_sub(&mut self, left: TNode<UintPtrT>, right: TNode<UintPtrT>) -> TNode<UintPtrT> {
        todo!()
    }
    pub fn uintptr_div(&mut self, left: TNode<UintPtrT>, right: TNode<UintPtrT>) -> TNode<UintPtrT> {
        todo!()
    }
    pub fn change_uint32_to_uintptr(&mut self, value: TNode<Uint32T>) -> TNode<UintPtrT> {
        todo!()
    }

    #[cfg(V8_MAP_PACKING)]
    pub fn pack_map_word(&mut self, map: TNode<Map>) -> *mut Node {
        todo!()
    }
    #[cfg(V8_MAP_PACKING)]
    pub fn unpack_map_word(&mut self, map_word: *mut Node) -> TNode<Map> {
        todo!()
    }
    pub fn load_map(&mut self, object: *mut Node) -> TNode<Map> {
        todo!()
    }

    pub fn debug_break(&mut self) -> *mut Node {
        todo!()
    }

    pub fn unreachable(&mut self) -> *mut Node {
        todo!()
    }
    // This special variant doesn't connect the Unreachable node to end, and does
    // not reset current effect/control. Intended only for special use-cases like
    // lowering DeadValue.
    pub fn unreachable_without_connect_to_end(&mut self) -> *mut Node {
        todo!()
    }

    pub fn intptr_equal(&mut self, left: *mut Node, right: *mut Node) -> *mut Node {
        todo!()
    }
    pub fn tagged_equal(&mut self, left: *mut Node, right: *mut Node) -> *mut Node {
        todo!()
    }

    pub fn smi_sub(&mut self, left
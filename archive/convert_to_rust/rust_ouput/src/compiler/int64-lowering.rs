// Converted from V8 C++ source files:
// Header: int64-lowering.h
// Implementation: int64-lowering.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/int64-lowering.h
pub mod int64_lowering {
use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::common_operator::CommonOperatorBuilder;
use crate::compiler::machine_operator::MachineOperatorBuilder;
use crate::compiler::simplified_operator::SimplifiedOperatorBuilder;
use crate::compiler::turbofan_graph::TFGraph;
use crate::wasm::wasm_engine::GetI32WasmCallDescriptor;
use crate::wasm::wasm_engine::WasmCallDescriptors;
use crate::wasm::wasm_engine::WasmEngine;
use crate::wasm::wasm_linkage::GetWasmEngine;
use crate::zone::zone::Zone;
use crate::compiler::linkage::MachineRepresentation;
use crate::compiler::linkage::MachineType;
use crate::compiler::node::Node;
use crate::compiler::node_properties::NodeProperties;
use crate::compiler::operator::{ IrOpcode, Operator };
use crate::compiler::wasm_compiler::ObjectAccess;
use crate::compiler::wasm_compiler::AtomicLoadParameters;
use crate::compiler::wasm_compiler::AtomicStoreParameters;
use crate::compiler::wasm_compiler::AtomicOpType;
use crate::compiler::operator::OpParameter;
use crate::compiler::call_descriptor::CallDescriptor;
use crate::compiler::node_matchers::Int32Matcher;
use crate::compiler::wasm_compiler::LoadRepresentationOf;
use crate::compiler::wasm_compiler::StoreRepresentation;
use crate::compiler::wasm_compiler::UnalignedStoreRepresentationOf;
use crate::compiler::diamond::Diamond;
use crate::compiler::node_properties::ProjectionIndexOf;
use crate::compiler::operator::PhiRepresentationOf;
use crate::compiler::operator::LoopExitValueRepresentationOf;
use crate::compiler::operator::ParameterIndexOf;
use std::collections::VecDeque;

    pub struct Signature<T> {
        params: Vec<T>,
        returns: Vec<T>,
    }

    impl<T: Copy + Clone> Signature<T> {
        pub fn new(params: Vec<T>, returns: Vec<T>) -> Self {
            Signature { params, returns }
        }

        pub fn parameter_count(&self) -> usize {
            self.params.len()
        }

        pub fn return_count(&self) -> usize {
            self.returns.len()
        }

        pub fn GetParam(&self, index: usize) -> T {
            self.params[index]
        }

        pub fn GetReturn(&self, index: usize) -> T {
            self.returns[index]
        }
    }

#[cfg(not(target_arch = "x86"))]
pub struct Int64Lowering {
    graph: *mut TFGraph,
    machine: *mut MachineOperatorBuilder,
    common: *mut CommonOperatorBuilder,
    simplified_: *mut SimplifiedOperatorBuilder,
    zone: *mut Zone,
    signature: *mut Signature<MachineRepresentation>,
}

#[cfg(not(target_arch = "x86"))]
impl Int64Lowering {
    pub fn new(
        graph: *mut TFGraph,
        machine: *mut MachineOperatorBuilder,
        common: *mut CommonOperatorBuilder,
        simplified_: *mut SimplifiedOperatorBuilder,
        zone: *mut Zone,
        signature: *mut Signature<MachineRepresentation>,
    ) -> Self {
        Int64Lowering {
            graph,
            machine,
            common,
            simplified_,
            zone,
            signature,
        }
    }

    pub fn lower_graph(&mut self) {}
}

#[cfg(target_arch = "x86")]
pub struct Int64Lowering<'a> {
    graph_: *mut TFGraph,
    machine_: *mut MachineOperatorBuilder,
    common_: *mut CommonOperatorBuilder,
    simplified_: *mut SimplifiedOperatorBuilder,
    zone_: &'a Zone,
    signature_: *mut Signature<MachineRepresentation>,
    state_: Vec<State>,
    stack_: VecDeque<NodeState>,
    replacements_: Vec<Replacement>,
    placeholder_: *mut Node,
}

#[cfg(target_arch = "x86")]
impl<'a> Int64Lowering<'a> {
    pub fn new(
        graph_: *mut TFGraph,
        machine_: *mut MachineOperatorBuilder,
        common_: *mut CommonOperatorBuilder,
        simplified_: *mut SimplifiedOperatorBuilder,
        zone_: &'a Zone,
        signature_: *mut Signature<MachineRepresentation>,
    ) -> Self {
        let graph = unsafe { &mut *graph_ };
        let node_count = graph.NodeCount();

        Int64Lowering {
            graph_: graph_,
            machine_: machine_,
            common_: common_,
            simplified_: simplified_,
            zone_: zone_,
            signature_: signature_,
            state_: vec![State::kUnvisited; node_count],
            stack_: VecDeque::new(),
            replacements_: vec![Replacement::default(); node_count],
            placeholder_: unsafe { (&mut *graph_).NewNode(unsafe { (&mut *common_).Dead() }) },
        }
    }

    fn zone(&self) -> &Zone {
        self.zone_
    }

    fn graph(&self) -> *mut TFGraph {
        self.graph_
    }

    fn machine(&self) -> *mut MachineOperatorBuilder {
        self.machine_
    }

    fn common(&self) -> *mut CommonOperatorBuilder {
        self.common_
    }

    fn simplified(&self) -> *mut SimplifiedOperatorBuilder {
        self.simplified_
    }

    fn signature(&self) -> *mut Signature<MachineRepresentation> {
        self.signature_
    }

    fn push_node(&mut self, _node: *mut Node) {
       
    }

    fn lower_node(&mut self, node: *mut Node) {
        unsafe {
            match (&mut *node).opcode() {
                IrOpcode::kInt64Constant => {
                    let value = OpParameter::<i64>((&mut *node).op());
                    let low_node = (&mut *self.graph_).NewNode((&mut *self.common_).Int32Constant(value as i32));
                    let high_node = (&mut *self.graph_).NewNode((&mut *self.common_).Int32Constant((value >> 32) as i32));
                    self.replace_node(node, low_node, high_node);
                }
                IrOpcode::kLoad => {
                     let rep =
                        LoadRepresentationOf((&mut *node).op()).representation();
                    self.lower_load_operator(node, rep, (&mut *self.machine_).Load(MachineType::Int32()));
                }
                IrOpcode::kUnalignedLoad => {
                     let rep =
                        LoadRepresentationOf((&mut *node).op()).representation();
                    self.lower_load_operator(node, rep, (&mut *self.machine_).UnalignedLoad(MachineType::Int32()));
                }
                IrOpcode::kLoadImmutable => {
                     let rep =
                        LoadRepresentationOf((&mut *node).op()).representation();
                    self.lower_load_operator(node, rep, (&mut *self.machine_).LoadImmutable(MachineType::Int32()));
                }
                IrOpcode::kLoadFromObject => {
                    let access = ObjectAccessOf((&mut *node).op());
                    self.lower_load_operator(node, access.machine_type.representation(),
                        (&mut *self.simplified_).LoadFromObject(ObjectAccess(MachineType::Int32(), access.write_barrier_kind)));
                }
                 IrOpcode::kLoadImmutableFromObject => {
                    let access = ObjectAccessOf((&mut *node).op());
                    self.lower_load_operator(node, access.machine_type.representation(),
                        (&mut *self.simplified_).LoadImmutableFromObject(ObjectAccess(MachineType::Int32(), access.write_barrier_kind)));
                }
                IrOpcode::kStore => {
                    let store_rep = StoreRepresentationOf((&mut *node).op());
                    self.lower_store_operator(
                        node,
                        store_rep.representation(),
                        (&mut *self.machine_).Store(StoreRepresentation(
                            MachineRepresentation::kWord32,
                            store_rep.write_barrier_kind(),
                        )),
                    );
                }
                IrOpcode::kUnalignedStore => {
                    let store_rep = UnalignedStoreRepresentationOf((&mut *node).op());
                    self.lower_store_operator(
                        node,
                        store_rep,
                        (&mut *self.machine_).UnalignedStore(MachineRepresentation::kWord32),
                    );
                }
                IrOpcode::kStoreToObject => {
                    let access = ObjectAccessOf((&mut *node).op());
                    self.lower_store_operator(
                        node,
                        access.machine_type.representation(),
                        (&mut *self.simplified_).StoreToObject(ObjectAccess(
                            MachineType::Int32(),
                            access.write_barrier_kind(),
                        )),
                    );
                }
                IrOpcode::kInitializeImmutableInObject => {
                    let access = ObjectAccessOf((&mut *node).op());
                    self.lower_store_operator(
                        node,
                        access.machine_type.representation(),
                        (&mut *self.simplified_).InitializeImmutableInObject(ObjectAccess(
                            MachineType::Int32(),
                            access.write_barrier_kind(),
                        )),
                    );
                }
                IrOpcode::kStart => {
                    let signature = &mut *self.signature_;
                    let parameter_count = Self::get_parameter_count_after_lowering(signature);
                    if parameter_count != signature.parameter_count() as i32 {
                        let delta = parameter_count - signature.parameter_count() as i32;
                        let new_output_count = (&mut *node).op().ValueOutputCount() as i32 + delta;
                        NodeProperties::ChangeOp(node, (&mut *self.common_).Start(new_output_count as usize));
                    }
                }
                IrOpcode::kParameter => {
                     let signature = &mut *self.signature_;
                    if (&mut *node).InputCount() != 1 {
                        eprintln!("{:?}", node);
                    }

                    assert_eq!((&mut *node).InputCount(), 1);

                    let param_count = signature.parameter_count() as i32;
                    if Self::get_parameter_count_after_lowering(signature) != param_count {
                        let old_index = ParameterIndexOf((&mut *node).op()) as i32;
                        let mut old_index_adj = old_index - 1;
                        let new_index = Self::get_parameter_index_after_lowering(signature, old_index_adj) as i32;
                        let mut new_index_adj = new_index + 1;

                        NodeProperties::ChangeOp(node, (&mut *self.common_).Parameter(new_index_adj as usize));

                        if old_index_adj < 0 || old_index_adj >= param_count {
                            break;
                        }

                        if signature.GetParam(old_index_adj as usize) == MachineRepresentation::kWord64 {
                            let high_node = (&mut *self.graph_).NewNode((&mut *self.common_).Parameter(new_index_adj as usize + 1), (&mut *self.graph_).start());
                            self.replace_node(node, node, high_node);
                        }
                    }
                }
                IrOpcode::kReturn => {
                    let input_count = (&mut *node).InputCount() as i32;
                    self.default_lowering(node, false);
                    if input_count != (&mut *node).InputCount() as i32 {
                         let signature = &mut *self.signature_;
                        let new_return_count = Self::get_return_count_after_lowering(signature);
                        if signature.return_count() as i32 != new_return_count {
                            NodeProperties::ChangeOp(node, (&mut *self.common_).Return(new_return_count as usize));
                        }
                    }
                }
                 IrOpcode::kTailCall => {
                    let call_descriptor =
                        CallDescriptorOf((&mut *node).op()) as *const CallDescriptor;
                    let mut_call_descriptor = call_descriptor as *mut CallDescriptor;
                    let signature = &mut *self.signature_;

                    let returns_require_lowering = Self::get_return_count_after_lowering_call_descriptor(unsafe{&mut *mut_call_descriptor}) !=
                                                   (&mut *mut_call_descriptor).ReturnCount() as i32;
                    if self.default_lowering(node, false) || returns_require_lowering {
                         let call_descriptor =
                        CallDescriptorOf((&mut *node).op()) as *const CallDescriptor;
                    let mut_call_descriptor = call_descriptor as *mut CallDescriptor;
                        NodeProperties::ChangeOp(
                            node,
                            (&mut *self.common_).TailCall(self.lower_call_descriptor(unsafe{&mut *mut_call_descriptor})),
                        );
                    }
                }
                IrOpcode::kCall => {
                    let call_descriptor =
                        CallDescriptorOf((&mut *node).op()) as *const CallDescriptor;
                    let mut_call_descriptor = call_descriptor as *mut CallDescriptor;

                    let returns_require_lowering = Self::get_return_count_after_lowering_call_descriptor(unsafe{&mut *mut_call_descriptor}) !=
                                                   (&mut *mut_call_descriptor).ReturnCount() as i32;

                    if self.default_lowering(node, false) || returns_require_lowering {
                        NodeProperties::ChangeOp(
                            node,
                            (&mut *self.common_).Call(self.lower_call_descriptor(unsafe{&mut *mut_call_descriptor})),
                        );
                    }
                    if returns_require_lowering {
                        let return_arity = (&mut *mut_call_descriptor).ReturnCount();
                        if return_arity == 1 {
                            self.replace_node_with_projections(node);
                        } else {
                            let mut projections: Vec<*mut Node> = vec![std::ptr::null_mut(); return_arity];
                            NodeProperties::CollectValueProjections(node, projections.as_mut_ptr(), return_arity);
                            let mut new_index: usize = 0;
                            for old_index in 0..return_arity {
                                let use_node = projections[old_index];
                                assert_eq!(ProjectionIndexOf((&mut *use_node).op()), old_index);

                                assert_eq!(
                                    Self::get_return_index_after_lowering(unsafe{&mut *mut_call_descriptor}, old_index as i32),
                                    new_index as i32
                                );

                                if new_index != old_index {
                                    NodeProperties::ChangeOp(use_node, (&mut *self.common_).Projection(new_index));
                                }

                                if (&mut *mut_call_descriptor).GetReturnType(old_index).representation() == MachineRepresentation::kWord64 {
                                    let high_node = (&mut *self.graph_).NewNode((&mut *self.common_).Projection(new_index + 1), node, (&mut *self.graph_).start());
                                    self.replace_node(use_node, use_node, high_node);
                                    new_index += 1;
                                }

                                new_index += 1;
                            }
                        }
                    }
                }
                IrOpcode::kWord64And => {
                    assert_eq!((&mut *node).InputCount(), 2);
                    let left = (&mut *node).InputAt(0);
                    let right = (&mut *node).InputAt(1);

                    let low_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32And(), self.get_replacement_low(left), self.get_replacement_low(right));
                    let high_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32And(), self.get_replacement_high(left), self.get_replacement_high(right));
                    self.replace_node(node, low_node, high_node);
                }
                 IrOpcode::kTruncateInt64ToInt32 => {
                    assert_eq!((&mut *node).InputCount(), 1);
                    let input = (&mut *node).InputAt(0);
                    self.replace_node(node, self.get_replacement_low(input), std::ptr::null_mut());
                    (&mut *node).NullAllInputs();
                }
                 IrOpcode::kInt64Add => {
                    assert_eq!((&mut *node).InputCount(), 2);

                    let right = (&mut *node).InputAt(1);
                    (&mut *node).ReplaceInput(1, self.get_replacement_low(right));
                    (&mut *node).AppendInput(self.zone_, self.get_replacement_high(right));

                    let left = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(left));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(left));

                    NodeProperties::ChangeOp(node, (&mut *self.machine_).Int32PairAdd());
                    self.replace_node_with_projections(node);
                }
                IrOpcode::kInt64Sub => {
                    assert_eq!((&mut *node).InputCount(), 2);

                    let right = (&mut *node).InputAt(1);
                    (&mut *node).ReplaceInput(1, self.get_replacement_low(right));
                    (&mut *node).AppendInput(self.zone_, self.get_replacement_high(right));

                    let left = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(left));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(left));

                    NodeProperties::ChangeOp(node, (&mut *self.machine_).Int32PairSub());
                    self.replace_node_with_projections(node);
                }
                 IrOpcode::kInt64Mul => {
                    assert_eq!((&mut *node).InputCount(), 2);

                    let right = (&mut *node).InputAt(1);
                    (&mut *node).ReplaceInput(1, self.get_replacement_low(right));
                    (&mut *node).AppendInput(self.zone_, self.get_replacement_high(right));

                    let left = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(left));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(left));

                    NodeProperties::ChangeOp(node, (&mut *self.machine_).Int32PairMul());
                    self.replace_node_with_projections(node);
                }
                 IrOpcode::kWord64Or => {
                    assert_eq!((&mut *node).InputCount(), 2);
                    let left = (&mut *node).InputAt(0);
                    let right = (&mut *node).InputAt(1);

                    let low_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32Or(), self.get_replacement_low(left), self.get_replacement_low(right));
                    let high_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32Or(), self.get_replacement_high(left), self.get_replacement_high(right));
                    self.replace_node(node, low_node, high_node);
                }
                IrOpcode::kWord64Xor => {
                    assert_eq!((&mut *node).InputCount(), 2);
                    let left = (&mut *node).InputAt(0);
                    let right = (&mut *node).InputAt(1);

                    let low_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32Xor(), self.get_replacement_low(left), self.get_replacement_low(right));
                    let high_node = (&mut *self.graph_).NewNode((&mut *self.machine_).Word32Xor(), self.get_replacement_high(left), self.get_replacement_high(right));
                    self.replace_node(node, low_node, high_node);
                }
                IrOpcode::kWord64Shl => {
                     let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 2);
                    let shift = (&mut *node).InputAt(1);
                    if self.has_replacement_low(shift) {
                        (&mut *node).ReplaceInput(1, self.get_replacement_low(shift));
                    }

                    let value = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(value));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(value));

                    NodeProperties::ChangeOp(node, machine.Word32PairShl());
                    self.replace_node_with_projections(node);
                }
                IrOpcode::kWord64Shr => {
                     let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 2);
                    let shift = (&mut *node).InputAt(1);
                    if self.has_replacement_low(shift) {
                        (&mut *node).ReplaceInput(1, self.get_replacement_low(shift));
                    }

                    let value = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(value));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(value));

                    NodeProperties::ChangeOp(node, machine.Word32PairShr());
                    self.replace_node_with_projections(node);
                }
                IrOpcode::kWord64Sar => {
                     let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 2);
                    let shift = (&mut *node).InputAt(1);
                    if self.has_replacement_low(shift) {
                        (&mut *node).ReplaceInput(1, self.get_replacement_low(shift));
                    }

                    let value = (&mut *node).InputAt(0);
                    (&mut *node).ReplaceInput(0, self.get_replacement_low(value));
                    (&mut *node).InsertInput(self.zone_, 1, self.get_replacement_high(value));

                    NodeProperties::ChangeOp(node, machine.Word32PairSar());
                    self.replace_node_with_projections(node);
                }
                 IrOpcode::kWord64Equal => {
                    let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 2);
                    let left = (&mut *node).InputAt(0);
                    let right = (&mut *node).InputAt(1);

                    let replacement = (&mut *self.graph_).NewNode(
                        machine.Word32Equal(),
                        (&mut *self.graph_).NewNode(
                            machine.Word32Or(),
                            (&mut *self.graph_).NewNode(machine.Word32Xor(), self.get_replacement_low(left), self.get_replacement_low(right)),
                            (&mut *self.graph_).NewNode(machine.Word32Xor(), self.get_replacement_high(left), self.get_replacement_high(right)),
                        ),
                        (&mut *self.graph_).NewNode(common.Int32Constant(0)),
                    );
                    self.replace_node(node, replacement, std::ptr::null_mut());
                }
                IrOpcode::kInt64LessThan => {
                     let machine = &mut *self.machine_;
                    self.lower_comparison(node, machine.Int32LessThan(), machine.Uint32LessThan());
                }
                 IrOpcode::kInt64LessThanOrEqual => {
                      let machine = &mut *self.machine_;
                    self.lower_comparison(node, machine.Int32LessThan(), machine.Uint32LessThanOrEqual());
                }
                 IrOpcode::kUint64LessThan => {
                      let machine = &mut *self.machine_;
                    self.lower_comparison(node, machine.Uint32LessThan(), machine.Uint32LessThan());
                }
                IrOpcode::kUint64LessThanOrEqual => {
                    let machine = &mut *self.machine_;
                    self.lower_comparison(node, machine.Uint32LessThan(), machine.Uint32LessThanOrEqual());
                }
                IrOpcode::kSignExtendWord32ToInt64 |
                IrOpcode::kChangeInt32ToInt64 => {
                     let machine = &mut *self.machine_;
                     let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 1);
                    let input = (&mut *node).InputAt(0);
                    let input = if self.has_replacement_low(input) {
                        self.get_replacement_low(input)
                    } else {
                        input
                    };

                    let high_node = (&mut *self.graph_).NewNode(machine.Word32Sar(), input, (&mut *self.graph_).NewNode(common.Int32Constant(31)));
                    self.replace_node(node, input, high_node);
                    (&mut *node).NullAllInputs();
                }
                IrOpcode::kChangeUint32ToUint64 => {
                     let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 1);
                    let input = (&mut *node).InputAt(0);
                    let input = if self.has_replacement_low(input) {
                        self.get_replacement_low(input)
                    } else {
                        input
                    };
                    self.replace_node(node, input, (&mut *self.graph_).NewNode(common.Int32Constant(0)));
                    (&mut *node).NullAllInputs();
                }
                IrOpcode::kBitcastInt64ToFloat64 => {
                     let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert_eq!((&mut *node).InputCount(), 1);
                    let input = (&mut *node).InputAt(0);

                    let high_half = (&mut *self.graph_).NewNode(
                        machine.Float64InsertHighWord32(),
                        (&mut *self.graph_).NewNode(common.Float64Constant(0.0)),
                        self.get_replacement_high(input),
                    );
                    let result = (&mut *self.graph_).NewNode(machine.Float64InsertLowWord32(), high_half, self.get_replacement_low(input));
                    self.replace_node(node, result, std::ptr::null_mut());
                }
                IrOpcode::kBitcastFloat64ToInt64 => {
                     let machine = &mut *self.machine_;
                    assert_eq!((&mut *node).InputCount(), 1);
                    let input = (&mut *node).InputAt(0);
                    let input = if self.has_replacement_low(input) {
                        self.get_replacement_low(input)
                    } else {
                        input
                    };

                    let low_node = (&mut *self.graph_).NewNode(machine.Float64ExtractLowWord32(), input);
                    let high_node = (&mut *self.graph_).NewNode(machine.Float64ExtractHighWord32(), input);
                    self.replace_node(node, low_node, high_node);
                }
                IrOpcode::kWord64RolLowerable => {
                    let machine = &mut *self.machine_;
                    let common = &mut *self.common_;
                    assert!(machine.Word32Rol().IsSupported());
                }
                IrOpcode::kWord64RorLowerable => {
                     let machine = &mut *self.machine_;
                    assert_eq!((&mut *node).InputCount(), 3);
                    let input = (&mut *node).InputAt(0);
                    let shift = if self.has_replacement_low((&mut *node).InputAt(1)) {
                        self.get_replacement_low((&mut *node).InputAt(1))
                    } else {
                        (&mut *node).InputAt(1)
                    };
                    let mut m = Int32Matcher::new(shift);
                    if m.HasResolvedValue() {
                        let shift_value = m.ResolvedValue() & 0x3F;
                        if shift_value == 0 {
                            self.replace_node(node, self.get_replacement_low(input), self.get_replacement_high(input));
                        } else if shift_value == 32 {
                            self.replace_node(node, self.get_replacement_high(input), self.get_replacement_low(input));
                        } else {
                            let (low_input, high_input) = if shift_value < 32 {
                                (self.get_replacement_low(input), self.get_replacement_high(input))
                            } else {
                                (self.get_replacement_high(input), self.get_replacement_low(input))
                            };
                            let masked_shift_value = shift_value & 0x1F;
                            let masked_shift = (&mut *self.graph_).NewNode(common.Int32Constant(masked_shift_value));
                            let inv_shift = (&mut *self.graph_).NewNode(common.Int32Constant(32 - masked_shift_value));

                            let (op1, op2) = {
                                let machine = &mut *self.machine_;
                                if (&mut *node).opcode() == IrOpcode::kWord64RorLowerable {
                                    (machine.Word32Shr(), machine.Word32Shl())
                                } else {
                                    (machine.Word32Shl(), machine.Word32Shr())
                                }
                            };
                            let low_node = (&mut *self.graph_).NewNode(
                                machine.Word32Or(),
                                (&mut *self.graph_).NewNode(op1, low_input, masked_shift),
                                (&mut *self.graph_).NewNode(op2, high_input, inv_shift),
                            );
                            let high_node = (&mut *self.graph_).NewNode(
                                machine.Word32Or(),
                                (&mut *self.graph_).NewNode(op1, high_input, masked_shift),
                                (&mut *self.graph_).NewNode(op2, low_input, inv_shift),
                            );
                            self.replace_node(node, low_node, high_node);
                        }
                    } else {
                        let mut safe_shift = shift;
                        if !machine.Word32ShiftIsSafe() {
                            safe_shift = (&mut *self.graph_).NewNode(
                                machine.Word32And(),
                                shift,
                                (&mut *self.graph_).NewNode(common.Int32Constant(0x1F)),
                            );
                        }
                        let is_ror = (&mut *node).opcode() == IrOpcode::kWord64RorLowerable;
                        let inv_mask = if is_ror {
                            (&mut *self.graph_).NewNode(
                                machine.Word32Xor(),
                                (&mut *self.graph_).NewNode(
                                    machine.Word32Shr(),
                                    (&mut *self.graph_).NewNode(common.Int32Constant(-1)),
                                    safe_shift,
                                ),
                                (&mut *self.graph_).NewNode(common.Int32Constant(-1)),
                            )
                        } else {
                            (&mut *self.graph_).NewNode(
                                machine.Word32Shl(),
                                (&mut *self.graph_).NewNode(common.Int32Constant(-1)),
                                safe_shift,
                            )
                        };
                        let bit_mask = (&mut *self.graph_).NewNode(
                            machine.Word32Xor(),
                            inv_mask,
                            (&mut *self.graph_).NewNode(common.Int32Constant(-1)),
                        );
                        let masked_shift6 = if machine.Word32ShiftIsSafe() {
                            (&mut *self.graph_).NewNode(
                                machine.Word32And(),
                                shift,
                                (&mut *self.graph_).NewNode(common.Int32Constant(0x3F)),
                            )
                        } else {
                            shift
                        };
                         let control = NodeProperties::GetControlInput(node);
                         let diamond = Diamond::new((&mut *self.graph_), (&mut *self.common_),
                                                (&mut *self.graph_).NewNode(machine.Int32LessThan(), masked_shift6, (&mut *self.graph_).NewNode(common.Int32Constant(32))));

                       // diamond.Chain(NodeProperties::GetControlInput(node));
                         let graph = &mut *self.graph_;
                        let common = &mut *self.common_;

                         let _ = diamond.Chain(if control.is_null() {
                            // Handle the case where the control input is

// Converted from V8 C++ source files:
// Header: wasm-gc-lowering.h
// Implementation: wasm-gc-lowering.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::compiler::wasm_compiler_definitions::NullCheckStrategy;
use crate::compiler::wasm_compiler_definitions::WasmTypeCheckConfig;
use crate::compiler::wasm_graph_assembler::WasmGraphAssembler;
use crate::wasm::wasm_objects::WasmArray;
use crate::wasm::wasm_objects::WasmStruct;
use crate::wasm::wasm_subtyping;
use crate::wasm::wasm_engine;
use crate::wasm::wasm_linkage;
use crate::wasm::wasm_objects;
use crate::wasm::wasm_objects::WasmInternalFunction;
use crate::wasm::object_access::ObjectAccess;
use crate::compiler::access_builder::AccessBuilder;
use crate::compiler::graph_reducer::AdvancedReducer;
use crate::compiler::graph_reducer::Reduction;
use crate::compiler::node_properties::NodeProperties;
use crate::compiler::operator::OpParameter;
use crate::objects::string::String;
use crate::compiler::opcodes::IrOpcode;
use crate::compiler::source_position_table::SourcePositionTable;
use crate::compiler::source_position_table::SourcePosition;
use crate::compiler::source_position_table::kNoSourcePosition;
use crate::compiler::machine_type::MachineType;
use crate::compiler::machine_graph::MachineGraph;
use crate::compiler::node::Node;
use crate::wasm::wasm_module::WasmModule;
use crate::objects::heap_number::HeapNumber;
use crate::execution::isolate::IsolateData;
use crate::compiler::js_call_reducer::RootIndex;
use crate::wasm::wasm_compiler_definitions::ExternalPointerTag;
use crate::compiler::wasm_compiler_definitions::WasmFieldInfo;
use crate::compiler::wasm_compiler_definitions::WasmElementInfo;
use crate::wasm::value_type::ValueType;
use crate::compiler::common_operator::Operator;
use crate::compiler::access_builder;
use crate::compiler::simplified_operator_reducer::SimplifiedOperatorReducer;
use crate::compiler::js_call_reducer::Builtin;
use crate::compiler::node::Type;
use crate::compiler::wasm_compiler_definitions::AssertNotNullParameters;
use crate::compiler::graph_reducer::Editor;

const kHeapObjectTag: usize = 0;
const kTaggedSize: usize = 8;
const WASM_ARRAY_TYPE: usize = 1;
const WASM_STRUCT_TYPE: usize = 2;
const FIRST_NONSTRING_TYPE: u32 = 3;
const kStringRepresentationMask: i32 = 0;
const kSeqStringTag: i32 = 1;
const kCharWidthBailoutSentinel: i32 = 2;
const kExternalStringResourceDataTag: ExternalPointerTag = ExternalPointerTag::kFinalizationRegistryCleanup;
const kStringEncodingMask: i32 = 1 << 3;
const kIsIndirectStringTag: i32 = 0;
const kIsDirectStringMask: i32 = 1;
const kThinStringTag: i32 = 2;
const kConsStringTag: i32 = 3;
const kExternalStringTag: i32 = 4;
const kUncachedExternalStringMask: i32 = 5;
const kFullWriteBarrier: i32 = 1;
const kNoWriteBarrier: i32 = 0;

struct Flags {
    experimental_wasm_skip_null_checks: bool,
}

static mut v8_flags: Flags = Flags {
    experimental_wasm_skip_null_checks: false,
};

pub struct WasmGCLowering<'a> {
    editor: *mut Editor,
    null_check_strategy_: NullCheckStrategy,
    gasm_: WasmGraphAssembler<'a>,
    module_: *const WasmModule,
    dead_: *mut Node,
    mcgraph_: *mut MachineGraph,
    source_position_table_: *mut SourcePositionTable,
}

impl<'a> WasmGCLowering<'a> {
    pub fn new(mcgraph: *mut MachineGraph) -> Self {
        let zone = unsafe { (*mcgraph).zone() };
        WasmGCLowering {
            editor: std::ptr::null_mut(),
            null_check_strategy_: NullCheckStrategy::kExplicit,
            gasm_: WasmGraphAssembler::new(mcgraph, zone),
            module_: std::ptr::null(),
            dead_: unsafe { (*mcgraph).Dead() },
            mcgraph_: mcgraph,
            source_position_table_: std::ptr::null_mut(),
        }
    }

    fn tagged_offset(access: access_builder::FieldAccess) -> i32 {
        assert!(access.base_is_tagged);
        wasm::ObjectAccess::to_tagged(access.offset)
    }

    pub fn new_gc(
        editor: *mut Editor,
        mcgraph: *mut MachineGraph,
        module: *const WasmModule,
        disable_trap_handler: bool,
        source_position_table: *mut SourcePositionTable,
    ) -> Self {
        let null_check_strategy = if wasm_linkage::is_trap_handler_enabled()
            && true // V8_STATIC_ROOTS_BOOL (assuming true for now)
            && !disable_trap_handler
        {
            NullCheckStrategy::kTrapHandler
        } else {
            NullCheckStrategy::kExplicit
        };
        let zone = unsafe { (*mcgraph).zone() };

        WasmGCLowering {
            editor: editor,
            null_check_strategy_: null_check_strategy,
            gasm_: WasmGraphAssembler::new(mcgraph, zone),
            module_: module,
            dead_: unsafe { (*mcgraph).Dead() },
            mcgraph_: mcgraph,
            source_position_table_: source_position_table,
        }
    }

    pub fn reducer_name(&self) -> &'static str {
        "WasmGCLowering"
    }

    pub fn reduce(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            match (*node).opcode() {
                IrOpcode::kWasmTypeCheck => self.reduce_wasm_type_check(node),
                IrOpcode::kWasmTypeCheckAbstract => self.reduce_wasm_type_check_abstract(node),
                IrOpcode::kWasmTypeCast => self.reduce_wasm_type_cast(node),
                IrOpcode::kWasmTypeCastAbstract => self.reduce_wasm_type_cast_abstract(node),
                IrOpcode::kAssertNotNull => self.reduce_assert_not_null(node),
                IrOpcode::kNull => self.reduce_null(node),
                IrOpcode::kIsNull => self.reduce_is_null(node),
                IrOpcode::kIsNotNull => self.reduce_is_not_null(node),
                IrOpcode::kRttCanon => self.reduce_rtt_canon(node),
                IrOpcode::kTypeGuard => self.reduce_type_guard(node),
                IrOpcode::kWasmAnyConvertExtern => self.reduce_wasm_any_convert_extern(node),
                IrOpcode::kWasmExternConvertAny => self.reduce_wasm_extern_convert_any(node),
                IrOpcode::kWasmStructGet => self.reduce_wasm_struct_get(node),
                IrOpcode::kWasmStructSet => self.reduce_wasm_struct_set(node),
                IrOpcode::kWasmArrayGet => self.reduce_wasm_array_get(node),
                IrOpcode::kWasmArraySet => self.reduce_wasm_array_set(node),
                IrOpcode::kWasmArrayLength => self.reduce_wasm_array_length(node),
                IrOpcode::kWasmArrayInitializeLength => self.reduce_wasm_array_initialize_length(node),
                IrOpcode::kStringAsWtf16 => self.reduce_string_as_wtf16(node),
                IrOpcode::kStringPrepareForGetCodeunit => self.reduce_string_prepare_for_get_codeunit(node),
                _ => Reduction::New(), // Assuming NoChange() is equivalent to an empty Reduction
            }
        }
    }

    fn reduce_wasm_type_check(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kWasmTypeCheck);

            let object = (*node).InputAt(0);
            let rtt = (*node).InputAt(1);
            let effect_input = NodeProperties::get_effect_input(node);
            let control_input = NodeProperties::get_control_input(node);
            let config = OpParameter::<WasmTypeCheckConfig>::new((*node).op());
            let rtt_depth = wasm_subtyping::get_subtyping_depth(self.module_, config.to.ref_index());
            let object_can_be_null = config.from.is_nullable();
            let object_can_be_i31 = wasm_subtyping::is_subtype_of(
                ValueType::wasm_i31ref().as_non_null(),
                config.from,
                self.module_,
            );

            self.gasm_.initialize_effect_control(effect_input, control_input);

            let end_label = self.gasm_.make_label(MachineType::word32().representation());
            let is_cast_from_any = config.from.is_reference_to(wasm::HeapType::kAny);

            if object_can_be_null && (!is_cast_from_any || config.to.is_nullable()) {
                let k_result = if config.to.is_nullable() { 1 } else { 0 };
                self.gasm_.goto_if(
                    self.is_null(object, ValueType::wasm_anyref()),
                    &end_label,
                    SimplifiedOperatorReducer::BranchHint::kFalse,
                    self.gasm_.int32_constant(k_result),
                );
            }

            if object_can_be_i31 {
                self.gasm_.goto_if(
                    self.gasm_.is_smi(object),
                    &end_label,
                    SimplifiedOperatorReducer::BranchHint::kFalse,
                    self.gasm_.int32_constant(0),
                );
            }

            let map = self.gasm_.load_map(object);

            if (*self.module_).type_(config.to.ref_index()).is_final {
                self.gasm_.goto(&end_label, self.gasm_.tagged_equal(map, rtt));
            } else {
                self.gasm_.goto_if(
                    self.gasm_.tagged_equal(map, rtt),
                    &end_label,
                    SimplifiedOperatorReducer::BranchHint::kTrue,
                    self.gasm_.int32_constant(1),
                );

                if is_cast_from_any {
                    let is_wasm_obj = self.gasm_.is_data_ref_map(map);
                    self.gasm_.goto_if_not(
                        is_wasm_obj,
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kTrue,
                        self.gasm_.int32_constant(0),
                    );
                }

                let type_info = self.gasm_.load_wasm_type_info(map);
                assert!(rtt_depth >= 0);

                if rtt_depth as u32 >= wasm_subtyping::k_minimum_supertype_array_size as u32 {
                    let supertypes_length = self.gasm_.build_change_smi_to_intptr(
                        self.gasm_.load_immutable_from_object(
                            MachineType::tagged_signed(),
                            type_info,
                            wasm::ObjectAccess::to_tagged(WasmTypeInfo::k_supertypes_length_offset),
                        ),
                    );
                    self.gasm_.goto_if_not(
                        self.gasm_.uint_less_than(
                            self.gasm_.intptr_constant(rtt_depth),
                            supertypes_length,
                        ),
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kTrue,
                        self.gasm_.int32_constant(0),
                    );
                }

                let maybe_match = self.gasm_.load_immutable_from_object(
                    MachineType::tagged_pointer(),
                    type_info,
                    wasm::ObjectAccess::to_tagged(
                        WasmTypeInfo::k_supertypes_offset + (kTaggedSize * rtt_depth),
                    ),
                );
                self.gasm_.goto(&end_label, self.gasm_.tagged_equal(maybe_match, rtt));
            }

            self.gasm_.bind(&end_label);

            self.replace_with_value(
                node,
                end_label.phi_at(0),
                self.gasm_.effect(),
                self.gasm_.control(),
            );
            (*node).kill();
            Reduction::Replace(end_label.phi_at(0))
        }
    }

    fn reduce_wasm_type_check_abstract(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kWasmTypeCheckAbstract);

            let object = (*node).InputAt(0);
            let effect_input = NodeProperties::get_effect_input(node);
            let control_input = NodeProperties::get_control_input(node);
            let config = OpParameter::<WasmTypeCheckConfig>::new((*node).op());
            let object_can_be_null = config.from.is_nullable();
            let null_succeeds = config.to.is_nullable();
            let object_can_be_i31 = wasm_subtyping::is_subtype_of(
                ValueType::wasm_i31ref().as_non_null(),
                config.from,
                self.module_,
            ) || config.from.heap_representation() == wasm::HeapType::kExtern;

            self.gasm_.initialize_effect_control(effect_input, control_input);

            let mut result: *mut Node = std::ptr::null_mut();
            let end_label = self.gasm_.make_label(MachineType::word32().representation());

            let to_rep = config.to.heap_representation();
            loop {
                if to_rep == wasm::HeapType::kNone
                    || to_rep == wasm::HeapType::kNoExtern
                    || to_rep == wasm::HeapType::kNoFunc
                    || to_rep == wasm::HeapType::kNoExn
                {
                    result = self.is_null(object, config.from);
                    break;
                }

                if object_can_be_null && null_succeeds {
                    let k_result = if null_succeeds { 1 } else { 0 };
                    self.gasm_.goto_if(
                        self.is_null(object, ValueType::wasm_anyref()),
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kFalse,
                        self.gasm_.int32_constant(k_result),
                    );
                }

                if to_rep == wasm::HeapType::kI31 {
                    result = if object_can_be_i31 {
                        self.gasm_.is_smi(object)
                    } else {
                        self.gasm_.int32_constant(0)
                    };
                    break;
                }

                if to_rep == wasm::HeapType::kEq {
                    if object_can_be_i31 {
                        self.gasm_.goto_if(
                            self.gasm_.is_smi(object),
                            &end_label,
                            SimplifiedOperatorReducer::BranchHint::kFalse,
                            self.gasm_.int32_constant(1),
                        );
                    }
                    result = self.gasm_.is_data_ref_map(self.gasm_.load_map(object));
                    break;
                }

                if object_can_be_i31 {
                    self.gasm_.goto_if(
                        self.gasm_.is_smi(object),
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kFalse,
                        self.gasm_.int32_constant(0),
                    );
                }

                if to_rep == wasm::HeapType::kArray {
                    result = self.gasm_.has_instance_type(object, WASM_ARRAY_TYPE);
                    break;
                }

                if to_rep == wasm::HeapType::kStruct {
                    result = self.gasm_.has_instance_type(object, WASM_STRUCT_TYPE);
                    break;
                }

                if to_rep == wasm::HeapType::kString || to_rep == wasm::HeapType::kExternString {
                    let instance_type = self.gasm_.load_instance_type(self.gasm_.load_map(object));
                    result = self.gasm_.uint32_less_than(
                        instance_type,
                        self.gasm_.uint32_constant(FIRST_NONSTRING_TYPE as u32),
                    );
                    break;
                }
                unreachable!();
            }

            assert!(!result.is_null());

            if end_label.is_used() {
                self.gasm_.goto(&end_label, result);
                self.gasm_.bind(&end_label);
                result = end_label.phi_at(0);
            }

            self.replace_with_value(
                node,
                result,
                self.gasm_.effect(),
                self.gasm_.control(),
            );
            (*node).kill();
            Reduction::Replace(result)
        }
    }

    fn reduce_wasm_type_cast(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kWasmTypeCast);

            let object = (*node).InputAt(0);
            let rtt = (*node).InputAt(1);
            let effect_input = NodeProperties::get_effect_input(node);
            let control_input = NodeProperties::get_control_input(node);
            let config = OpParameter::<WasmTypeCheckConfig>::new((*node).op());
            let rtt_depth = wasm_subtyping::get_subtyping_depth(self.module_, config.to.ref_index());
            let object_can_be_null = config.from.is_nullable();
            let object_can_be_i31 = wasm_subtyping::is_subtype_of(
                ValueType::wasm_i31ref().as_non_null(),
                config.from,
                self.module_,
            );

            self.gasm_.initialize_effect_control(effect_input, control_input);

            let end_label = self.gasm_.make_label();
            let is_cast_from_any = config.from.is_reference_to(wasm::HeapType::kAny);

            if object_can_be_null && (!is_cast_from_any || config.to.is_nullable()) {
                let is_null = self.is_null(object, ValueType::wasm_anyref());
                if config.to.is_nullable() {
                    self.gasm_.goto_if(
                        is_null,
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kFalse,
                    );
                } else if !v8_flags.experimental_wasm_skip_null_checks {
                    self.gasm_.trap_if(is_null, wasm_linkage::TrapId::kTrapIllegalCast);
                    self.update_source_position(self.gasm_.effect(), node);
                }
            }

            if object_can_be_i31 {
                self.gasm_.trap_if(self.gasm_.is_smi(object), wasm_linkage::TrapId::kTrapIllegalCast);
                self.update_source_position(self.gasm_.effect(), node);
            }

            let map = self.gasm_.load_map(object);

            if (*self.module_).type_(config.to.ref_index()).is_final {
                self.gasm_.trap_unless(
                    self.gasm_.tagged_equal(map, rtt),
                    wasm_linkage::TrapId::kTrapIllegalCast,
                );
                self.update_source_position(self.gasm_.effect(), node);
                self.gasm_.goto(&end_label);
            } else {
                self.gasm_.goto_if(
                    self.gasm_.tagged_equal(map, rtt),
                    &end_label,
                    SimplifiedOperatorReducer::BranchHint::kTrue,
                );

                if is_cast_from_any {
                    let is_wasm_obj = self.gasm_.is_data_ref_map(map);
                    self.gasm_.trap_unless(
                        is_wasm_obj,
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                }

                let type_info = self.gasm_.load_wasm_type_info(map);
                assert!(rtt_depth >= 0);

                if rtt_depth as u32 >= wasm_subtyping::k_minimum_supertype_array_size as u32 {
                    let supertypes_length = self.gasm_.build_change_smi_to_intptr(
                        self.gasm_.load_immutable_from_object(
                            MachineType::tagged_signed(),
                            type_info,
                            wasm::ObjectAccess::to_tagged(WasmTypeInfo::k_supertypes_length_offset),
                        ),
                    );
                    self.gasm_.trap_unless(
                        self.gasm_.uint_less_than(
                            self.gasm_.intptr_constant(rtt_depth),
                            supertypes_length,
                        ),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                }

                let maybe_match = self.gasm_.load_immutable_from_object(
                    MachineType::tagged_pointer(),
                    type_info,
                    wasm::ObjectAccess::to_tagged(
                        WasmTypeInfo::k_supertypes_offset + (kTaggedSize * rtt_depth),
                    ),
                );

                self.gasm_.trap_unless(
                    self.gasm_.tagged_equal(maybe_match, rtt),
                    wasm_linkage::TrapId::kTrapIllegalCast,
                );
                self.update_source_position(self.gasm_.effect(), node);
                self.gasm_.goto(&end_label);
            }

            self.gasm_.bind(&end_label);

            self.replace_with_value(
                node,
                object,
                self.gasm_.effect(),
                self.gasm_.control(),
            );
            (*node).kill();
            Reduction::Replace(object)
        }
    }

    fn reduce_wasm_type_cast_abstract(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kWasmTypeCastAbstract);

            let object = (*node).InputAt(0);
            let effect_input = NodeProperties::get_effect_input(node);
            let control_input = NodeProperties::get_control_input(node);
            let config = OpParameter::<WasmTypeCheckConfig>::new((*node).op());
            let object_can_be_null = config.from.is_nullable();
            let null_succeeds = config.to.is_nullable();
            let object_can_be_i31 = wasm_subtyping::is_subtype_of(
                ValueType::wasm_i31ref().as_non_null(),
                config.from,
                self.module_,
            ) || config.from.heap_representation() == wasm::HeapType::kExtern;

            self.gasm_.initialize_effect_control(effect_input, control_input);

            let end_label = self.gasm_.make_label();

            let to_rep = config.to.heap_representation();

            loop {
                if to_rep == wasm::HeapType::kNone
                    || to_rep == wasm::HeapType::kNoExtern
                    || to_rep == wasm::HeapType::kNoFunc
                    || to_rep == wasm::HeapType::kNoExn
                {
                    self.gasm_.trap_unless(
                        self.is_null(object, config.from),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }

                if object_can_be_null && null_succeeds
                    && !v8_flags.experimental_wasm_skip_null_checks
                {
                    self.gasm_.goto_if(
                        self.is_null(object, config.from),
                        &end_label,
                        SimplifiedOperatorReducer::BranchHint::kFalse,
                    );
                }

                if to_rep == wasm::HeapType::kI31 {
                    let success = if object_can_be_i31 {
                        self.gasm_.is_smi(object)
                    } else {
                        self.gasm_.int32_constant(0)
                    };
                    self.gasm_.trap_unless(
                        success,
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }

                if to_rep == wasm::HeapType::kEq {
                    if object_can_be_i31 {
                        self.gasm_.goto_if(
                            self.gasm_.is_smi(object),
                            &end_label,
                            SimplifiedOperatorReducer::BranchHint::kFalse,
                        );
                    }
                    self.gasm_.trap_unless(
                        self.gasm_.is_data_ref_map(self.gasm_.load_map(object)),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }

                if object_can_be_i31 {
                    self.gasm_.trap_if(self.gasm_.is_smi(object), wasm_linkage::TrapId::kTrapIllegalCast);
                    self.update_source_position(self.gasm_.effect(), node);
                }

                if to_rep == wasm::HeapType::kArray {
                    self.gasm_.trap_unless(
                        self.gasm_.has_instance_type(object, WASM_ARRAY_TYPE),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }

                if to_rep == wasm::HeapType::kStruct {
                    self.gasm_.trap_unless(
                        self.gasm_.has_instance_type(object, WASM_STRUCT_TYPE),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }

                if to_rep == wasm::HeapType::kString || to_rep == wasm::HeapType::kExternString {
                    let instance_type = self.gasm_.load_instance_type(self.gasm_.load_map(object));
                    self.gasm_.trap_unless(
                        self.gasm_.uint32_less_than(
                            instance_type,
                            self.gasm_.uint32_constant(FIRST_NONSTRING_TYPE as u32),
                        ),
                        wasm_linkage::TrapId::kTrapIllegalCast,
                    );
                    self.update_source_position(self.gasm_.effect(), node);
                    break;
                }
                unreachable!();
            }

            if end_label.is_used() {
                self.gasm_.goto(&end_label);
                self.gasm_.bind(&end_label);
            }

            self.replace_with_value(
                node,
                object,
                self.gasm_.effect(),
                self.gasm_.control(),
            );
            (*node).kill();
            Reduction::Replace(object)
        }
    }

    fn reduce_assert_not_null(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kAssertNotNull);
            let effect = NodeProperties::get_effect_input(node);
            let control = NodeProperties::get_control_input(node);
            let object = NodeProperties::get_value_input(node, 0);
            self.gasm_.initialize_effect_control(effect, control);
            let op_parameter = OpParameter::<AssertNotNullParameters>::new((*node).op());

            if op_parameter.trap_id == wasm_linkage::TrapId::kTrapNullDereference {
                if !v8_flags.experimental_wasm_skip_null_checks {
                    if self.null_check_strategy_ == NullCheckStrategy::kExplicit
                        || wasm_subtyping::is_subtype_of(
                            ValueType::wasm_i31ref().as_non_null(),
                            op_parameter.type,
                            self.module_,
                        )
                        || !op_parameter.type.use_wasm_null()
                    {
                        self.gasm_.trap_if(self.is_null(object, op_parameter.type), op_parameter.trap_id);
                        self.update_source_position(self.gasm_.effect(), node);
                    } else {
                        static_assert!(WasmStruct::kHeaderSize > kTaggedSize);
                        static_assert!(WasmArray::kHeaderSize > kTaggedSize);
                        static_assert!(WasmInternalFunction::kHeaderSize > kTaggedSize);

                        let trap_null = self.gasm_.load_trap_on_null(
                            MachineType::int32(),
                            object,
                            self.gasm_.intptr_constant(wasm::ObjectAccess::to_tagged(kTaggedSize)),
                        );
                        self.update_source_position(trap_null, node);
                    }
                }
            } else {
                self.gasm_.trap_if(self.is_null(object, op_parameter.type), op_parameter.trap_id);
                self.update_source_position(self.gasm_.effect(), node);
            }

            self.replace_with_value(
                node,
                object,
                self.gasm_.effect(),
                self.gasm_.control(),
            );
            (*node).kill();
            Reduction::Replace(object)
        }
    }

    fn reduce_null(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kNull);
            let type_ = OpParameter::<ValueType>::new((*node).op());
            Reduction::Replace(self.null(type_))
        }
    }

    fn reduce_is_null(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kIsNull);
            let object = NodeProperties::get_value_input(node, 0);
            let type_ = OpParameter::<ValueType>::new((*node).op());
            Reduction::Replace(self.is_null(object, type_))
        }
    }

    fn reduce_is_not_null(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kIsNotNull);
            let object = NodeProperties::get_value_input(node, 0);
            let type_ = OpParameter::<ValueType>::new((*node).op());
            Reduction::Replace(self.gasm_.word32_equal(
                self.is_null(object, type_),
                self.gasm_.int32_constant(0),
            ))
        }
    }

    fn reduce_rtt_canon(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kRttCanon);
            let type_index = OpParameter::<i32>::new((*node).op());
            let instance_node = (*node).InputAt(0);
            let maps_list = self.gasm_.load_immutable(
                MachineType::tagged_pointer(),
                instance_node,
                WasmTrustedInstanceData::kManagedObjectMapsOffset - kHeapObjectTag,
            );
            Reduction::Replace(self.gasm_.load_immutable(
                MachineType::tagged_pointer(),
                maps_list,
                wasm::ObjectAccess::element_offset_in_tagged_fixed_array(type_index),
            ))
        }
    }

    fn reduce_type_guard(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), IrOpcode::kTypeGuard);
            let alias = NodeProperties::get_value_input(node, 0);
            self.replace_with_value(node, alias);
            (*node).kill();
            Reduction::Replace(alias)
        }
    }

    fn reduce_wasm_any_convert_extern(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            assert_eq!((*node).opcode(), Ir

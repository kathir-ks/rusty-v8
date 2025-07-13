// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code, unused_variables)]
use std::{
    convert::TryInto,
    mem::size_of,
    sync::{Arc, Mutex, RwLock},
};

use crate::base::bits::IsPowerOfTwo;
use crate::codegen::assembler_inl::RootRegisterOffsetForExternalReference;
use crate::codegen::{
    assembler_inl,
    machine_type::{self, MachineType},
};
use crate::common::globals::kTaggedSize;
use crate::compiler::backend::{
    instruction::{AddressingMode, Instruction, InstructionCode, InstructionSequence},
    instruction_codes::{
        AccessModeField, AtomicWidthField, FlagsCondition, LaneSizeField,
        MemoryAccessKind, MiscField, RecordWriteModeField,
    },
    instruction_selector::{InstructionSelector, TurboshaftAdapter},
    instruction_selector_impl::InstructionSelectorT,
    register_allocator::{
        Frame, InstructionOperand, MachineRepresentation, OperandGenerator,
        RegisterRepresentation,
    },
    spill_placer::TopLevelLiveRange,
};
use crate::compiler::common_operator::DeoptimizeKind;
use crate::compiler::code_assembler::RootIndex;
use crate::compiler::machine_operator::{
    AlignmentRequirements, AtomicMemoryOrder, MachineOperatorBuilder,
};
use crate::compiler::turboshaft::{
    opmasks, operations::*, representations::WordRepresentation, OpIndex,
};
use crate::compiler::{backend::instruction::ArchOpcode, turboshaft};
use crate::flags::{
    kFloat16, COMPRESS_POINTERS_BOOL, DOTPROD, FP16, LSE, PMULL1Q,
    V8_COMPRESS_POINTERS, V8_STATIC_ROOTS_BOOL,
};
use crate::objects::{Tagged, HeapObject, RootsTable, RootsTableEntry, Map};
use crate::roots::RootsTable;
use crate::{
    objects::Tagged, zone::Zone, compiler::wasm_graph_assembler::*
};

enum ImmediateMode {
    kArithmeticImm, // 12 bit unsigned immediate shifted left 0 or 12 bits
    kShift32Imm,    // 0 - 31
    kShift64Imm,    // 0 - 63
    kLogical32Imm,
    kLogical64Imm,
    kLoadStoreImm8, // signed 8 bit or 12 bit unsigned scaled by access size
    kLoadStoreImm16,
    kLoadStoreImm32,
    kLoadStoreImm64,
    kConditionalCompareImm,
    kNoImmediate,
}

struct Arm64OperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT<'a>,
}

impl<'a> Arm64OperandGeneratorT<'a> {
    pub fn new(selector: &'a mut InstructionSelectorT<'a>) -> Self {
        Arm64OperandGeneratorT { selector }
    }

    fn use_operand(&mut self, node: OpIndex, mode: ImmediateMode) -> InstructionOperand {
        if self.can_be_immediate(node, mode) {
            return self.use_immediate(node);
        }
        self.use_register(node)
    }

    fn is_immediate_zero(&mut self, node: OpIndex) -> bool {
        if let Some(constant) = self.selector.get(node).try_cast::<ConstantOp>() {
            if constant.is_integral() && constant.integral() == 0 {
                return true;
            }
            if constant.kind == ConstantOp::Kind::kFloat32 {
                return constant.float32().get_bits() == 0;
            }
            if constant.kind == ConstantOp::Kind::kFloat64 {
                return constant.float64().get_bits() == 0;
            }
        }
        false
    }

    fn use_register_or_immediate_zero(&mut self, node: OpIndex) -> InstructionOperand {
        if self.is_immediate_zero(node) {
            self.use_immediate(node)
        } else {
            self.use_register(node)
        }
    }

    fn use_register_at_end_or_immediate_zero(&mut self, node: OpIndex) -> InstructionOperand {
        if self.is_immediate_zero(node) {
            self.use_immediate(node)
        } else {
            self.use_register_at_end(node)
        }
    }

    fn use_immediate_or_temp(&mut self, node: OpIndex, value: i32) -> InstructionOperand {
        if self.selector.get(node).cast::<ConstantOp>().signed_integral() == value as i64 {
            self.use_immediate(node)
        } else {
            self.temp_immediate(value)
        }
    }

    fn is_integer_constant(&self, node: OpIndex) -> bool {
        let mut unused: i64 = 0;
        self.selector.match_signed_integral_constant(node, &mut unused)
    }

    fn get_optional_integer_constant(&self, operation: OpIndex) -> Option<i64> {
        let mut constant: i64 = 0;
        if self.selector.match_signed_integral_constant(operation, &mut constant) {
            Some(constant)
        } else {
            None
        }
    }

    fn can_be_immediate(&mut self, node: OpIndex, mode: ImmediateMode) -> bool {
        let constant = self.selector.get(node).try_cast::<ConstantOp>();
        if constant.is_none() {
            return false;
        }
        let constant = constant.unwrap();

        if constant.kind == ConstantOp::Kind::kCompressedHeapObject {
            if !COMPRESS_POINTERS_BOOL {
                return false;
            }
            if self.selector.isolate().bootstrapper().is_some() && !V8_STATIC_ROOTS_BOOL {
                return false;
            }
            let roots_table = self.selector.isolate().roots_table();
            let mut root_index = RootIndex {};
            let value: Handle<HeapObject> = Handle::new(constant.handle());
            if roots_table.is_root_handle(value.clone(), &mut root_index) {
                if !RootsTable::is_read_only(root_index) {
                    return false;
                }
                return self.can_be_immediate(
                    assembler_inl::ReadOnlyRootPtr(root_index, self.selector.isolate()),
                    mode,
                );
            }
            return false;
        }

        let mut value: i64 = 0;
        self.selector.match_signed_integral_constant(node, &mut value) &&
            self.can_be_immediate_value(value, mode)
    }

    fn can_be_immediate_value(&mut self, value: i64, mode: ImmediateMode) -> bool {
        let mut ignored: u32 = 0;
        match mode {
            ImmediateMode::kLogical32Imm => internal::Assembler::is_imm_logical(
                value as u32,
                32,
                &mut ignored,
                &mut ignored,
                &mut ignored,
            ),
            ImmediateMode::kLogical64Imm => internal::Assembler::is_imm_logical(
                value as u64,
                64,
                &mut ignored,
                &mut ignored,
                &mut ignored,
            ),
            ImmediateMode::kArithmeticImm => internal::Assembler::is_imm_add_sub(value),
            ImmediateMode::kLoadStoreImm8 => self.is_load_store_immediate(value, 0),
            ImmediateMode::kLoadStoreImm16 => self.is_load_store_immediate(value, 1),
            ImmediateMode::kLoadStoreImm32 => self.is_load_store_immediate(value, 2),
            ImmediateMode::kLoadStoreImm64 => self.is_load_store_immediate(value, 3),
            ImmediateMode::kNoImmediate => false,
            ImmediateMode::kConditionalCompareImm => internal::Assembler::is_imm_conditional_compare(value),
            ImmediateMode::kShift32Imm | ImmediateMode::kShift64Imm => {
                // Shift operations only observe the bottom 5 or 6 bits of the value.
                true
            }
        }
    }

    fn can_be_load_store_shift_immediate(
        &mut self,
        node: OpIndex,
        rep: MachineRepresentation,
    ) -> bool {
        let mut constant: u64 = 0;
        self.selector.match_unsigned_integral_constant(node, &mut constant) &&
            constant == element_size_log2_of(rep) as u64
    }

    fn is_load_store_immediate(&mut self, value: i64, size: u32) -> bool {
        internal::Assembler::is_imm_ls_scaled(value, size) ||
            internal::Assembler::is_imm_ls_unscaled(value)
    }
}

mod arm64_instruction_selector {
    use super::*;

    pub fn visit_rr(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex) {
        let mut g = Arm64OperandGeneratorT::new(selector);
        selector.emit(
            opcode,
            g.define_as_register(node),
            g.use_register(selector.input_at(node, 0)),
        );
    }

    pub fn visit_rrr(selector: &mut InstructionSelectorT, opcode: InstructionCode, node: OpIndex) {
        let mut g = Arm64OperandGeneratorT::new(selector);
        selector.emit(
            opcode,
            g.define_as_register(node),
            g.use_register(selector.input_at(node, 0)),
            g.use_register(selector.input_at(node, 1)),
        );
    }
}

pub trait InstructionSelectorTMethods {
}

impl InstructionSelectorTMethods for InstructionSelectorT<'_> {
}

struct Handle<T> {
   _phantom: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
   fn new(_obj: *mut T) -> Self {
       Handle {
           _phantom: std::marker::PhantomData,
       }
   }
   fn is_null(&self) -> bool {
       false
   }
}
impl Clone for Handle<HeapObject> {
    fn clone(&self) -> Self {
        Handle { _phantom: std::marker::PhantomData }
    }
}

impl Copy for Handle<HeapObject> {}


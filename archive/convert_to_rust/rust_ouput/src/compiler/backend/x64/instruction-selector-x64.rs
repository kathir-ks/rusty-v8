// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::Mutex;

use crate::base;
use crate::codegen::cpu_features::CpuFeatures;
use crate::codegen::machine_type::MachineType;
use crate::compiler::backend::instruction_codes::AddressingMode;
use crate::compiler::backend::instruction_codes::AddressingMode::*;
use crate::compiler::backend::instruction_codes::ArchOpcode;
use crate::compiler::backend::instruction_codes::AtomicWidth;
use crate::compiler::backend::instruction_codes::AtomicWidth::*;
use crate::compiler::backend::instruction_codes::InstructionCode;
use crate::compiler::backend::instruction_codes::MemoryAccessKind;
use crate::compiler::backend::instruction_codes::RecordWriteMode;
use crate::compiler::backend::instruction_selector_adapter::LoadRepresentation;
use crate::compiler::backend::instruction_selector_adapter::OpIndex;
use crate::compiler::backend::instruction_selector_adapter::OptionalOpIndex;
use crate::compiler::backend::instruction_selector_adapter::RegisterRepresentation;
use crate::compiler::backend::instruction_selector_adapter::StoreRepresentation;
use crate::compiler::backend::instruction_selector_impl::FlagsContinuation;
use crate::compiler::backend::instruction::InstructionOperand;
use crate::compiler::backend::instruction_selector_impl::*;
use crate::compiler::machine_operator::MemoryRepresentation;
use crate::compiler::machine_operator::StoreRepresentation::*;
use crate::compiler::turboshaft::load_store_simplification_reducer::DisplacementMode;
use crate::compiler::turboshaft::load_store_simplification_reducer::DisplacementMode::*;
use crate::compiler::turboshaft::operations::*;
use crate::compiler::backend::move_optimizer::RegisterUseKind;
use crate::compiler::machine_operator::AtomicMemoryOrder;
use crate::compiler::machine_operator::AtomicMemoryOrder::*;
use crate::compiler::wasm_gc_operator_reducer::Simd128ConstantOp;
use crate::wasm::simd_shuffle::*;
use crate::handles::handles_inl::*;
use crate::objects::slots_inl::*;
use crate::roots::roots_inl::*;
use crate::compiler::code_assembler::RootIndex;

mod opmasks;
mod turboshaft;

pub struct X64InstructionSelector {}

impl X64InstructionSelector {
    pub fn new() -> Self {
        X64InstructionSelector {}
    }
}

fn is_compressed(selector: &InstructionSelectorT, node: OpIndex) -> bool {
    if !node.valid() {
        return false;
    }
    if selector.is_load(node) {
        let load = selector.load_view(node);
        return load.loaded_rep().is_compressed();
    } else if selector.IsPhi(node) {
        let phi_rep = selector.phi_representation_of(node);
        return phi_rep == MachineRepresentation::kCompressed ||
            phi_rep == MachineRepresentation::kCompressedPointer;
    }
    false
}

#[cfg(debug_assertions)]
fn lhs_is_not_only_constant(graph: &Graph, left_idx: OpIndex, right_idx: OpIndex) -> bool {
    let left = graph.get(left_idx);
    let right = graph.get(right_idx);

    if right.is::<ConstantOp>() {
        return true;
    }
    if left.is::<ConstantOp>() {
        return false;
    }

    true
}

fn value_fits_into_immediate(value: i64) -> bool {
    const IMMEDIATE_MIN: i64 = i32::MIN as i64 + 1;
    const IMMEDIATE_MAX: i64 = i32::MAX as i64;

    IMMEDIATE_MIN <= value && value <= IMMEDIATE_MAX
}

fn can_be_immediate(selector: &InstructionSelectorT, node: OpIndex) -> bool {
    let op = selector.get(node);
    if !op.is::<ConstantOp>() {
        return false;
    }

    let constant = op.cast::<ConstantOp>();
    match constant.kind {
        ConstantOp::Kind::kCompressedHeapObject => {
            if !crate::common::globals::COMPRESS_POINTERS_BOOL {
                return false;
            }
            if selector.isolate().bootstrapper() && !crate::common::globals::V8_STATIC_ROOTS_BOOL {
                return false;
            }

            let roots_table = selector.isolate().roots_table();
            let mut root_index = RootIndex {};
            let value = constant.handle();
            if roots_table.IsRootHandle(value, &mut root_index) {
                return RootsTable::IsReadOnly(root_index);
            }
            false
        }
        ConstantOp::Kind::kWord32 => {
            let value = constant.word32();
            value != i32::MIN
        }
        ConstantOp::Kind::kWord64 => {
            let value = constant.word64();
            value_fits_into_immediate(value)
        }
        ConstantOp::Kind::kSmi => {
            if crate::common::globals::Is64() {
                let value = constant.smi().ptr() as i64;
                value_fits_into_immediate(value)
            } else {
                let value = constant.smi().ptr() as i32;
                value != i32::MIN
            }
        }
        ConstantOp::Kind::kNumber => {
            constant.number().get_bits() == 0
        }
        _ => false,
    }
}

fn get_immediate_integer_value(selector: &InstructionSelectorT, node: OpIndex) -> i32 {
    debug_assert!(can_be_immediate(selector, node));
    let constant = selector.get(node).cast::<ConstantOp>();
    match constant.kind {
        ConstantOp::Kind::kWord32 => constant.word32(),
        ConstantOp::Kind::kWord64 => constant.word64() as i32,
        ConstantOp::Kind::kSmi => constant.smi().ptr() as i32,
        ConstantOp::Kind::kNumber => {
            debug_assert_eq!(constant.number().get_bits(), 0);
            0
        }
        _ => unreachable!(),
    }
}

struct ScaledIndexMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
}

fn match_scaled_index(selector: &InstructionSelectorT, node: OpIndex, index: &mut OpIndex, scale: &mut i32, power_of_two_plus_one: Option<&mut bool>) -> bool {
    let match_scale_constant = |op: &Operation, scale: &mut i32, plus_one: Option<&mut bool>| -> bool {
        if let Some(constant) = op.try_cast::<ConstantOp>() {
            if constant.kind != ConstantOp::Kind::kWord32 && constant.kind != ConstantOp::Kind::kWord64 {
                return false;
            }

            let value = constant.integral();
            if let Some(plus_one) = plus_one.as_deref_mut() {
                *plus_one = false;
            }
            if value == 1 { *scale = 0; return true; }
            if value == 2 { *scale = 1; return true; }
            if value == 4 { *scale = 2; return true; }
            if value == 8 { *scale = 3; return true; }
            if let Some(plus_one) = plus_one.as_deref_mut() {
                *plus_one = true;
                if value == 3 { *scale = 1; return true; }
                if value == 5 { *scale = 2; return true; }
                if value == 9 { *scale = 3; return true; }
                return false;
            } else {
                return false;
            }
        }
        false
    };

    let op = selector.get(node);
    if let Some(binop) = op.try_cast::<WordBinopOp>() {
        if binop.kind != WordBinopOp::Kind::kMul {
            return false;
        }
        if match_scale_constant(selector.get(binop.right()), scale, power_of_two_plus_one) {
            *index = binop.left();
            return true;
        }
        if match_scale_constant(selector.get(binop.left()), scale, power_of_two_plus_one) {
            *index = binop.right();
            return true;
        }
        return false;
    } else if let Some(shift) = op.try_cast::<ShiftOp>() {
        if shift.kind != ShiftOp::Kind::kShiftLeft {
            return false;
        }
        if let Some(scale_value) = selector.match_signed_integral_constant(shift.right()) {
            if scale_value < 0 || scale_value > 3 {
                return false;
            }
            *index = shift.left();
            *scale = scale_value as i32;
            if let Some(plus_one) = power_of_two_plus_one.as_deref_mut() {
                *plus_one = false;
            }
            return true;
        }
    }
    false
}

fn try_match_scaled_index(selector: &InstructionSelectorT, node: OpIndex, allow_power_of_two_plus_one: bool) -> Option<ScaledIndexMatch> {
    let mut match_obj = ScaledIndexMatch {
        base: OpIndex::invalid(),
        index: OpIndex::invalid(),
        scale: 0,
    };
    let plus_one: Option<&mut bool> = if allow_power_of_two_plus_one { Some(&mut false) } else { None };
    if match_scaled_index(selector, node, &mut match_obj.index, &mut match_obj.scale, plus_one.as_deref_mut()) {
        if let Some(plus_one_val) = plus_one {
            if plus_one_val {
                match_obj.base = match_obj.index;
            }
        }
        return Some(match_obj);
    }
    None
}

fn try_match_scaled_index32(selector: &InstructionSelectorT, node: OpIndex, allow_power_of_two_plus_one: bool) -> Option<ScaledIndexMatch> {
    try_match_scaled_index(selector, node, allow_power_of_two_plus_one)
}

fn try_match_scaled_index64(selector: &InstructionSelectorT, node: OpIndex, allow_power_of_two_plus_one: bool) -> Option<ScaledIndexMatch> {
    try_match_scaled_index(selector, node, allow_power_of_two_plus_one)
}

struct BaseWithScaledIndexAndDisplacementMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
    displacement: i64,
    displacement_mode: DisplacementMode,
}

impl BaseWithScaledIndexAndDisplacementMatch {
    fn new() -> Self {
        BaseWithScaledIndexAndDisplacementMatch {
            base: OpIndex::invalid(),
            index: OpIndex::invalid(),
            scale: 0,
            displacement: 0,
            displacement_mode: DisplacementMode::kPositiveDisplacement,
        }
    }
}

fn try_match_base_with_scaled_index_and_displacement64_for_word_binop(
    selector: &InstructionSelectorT, left: OpIndex, right: OpIndex,
    is_commutative: bool) -> Option<BaseWithScaledIndexAndDisplacementMatch> {
    let match_s_plus = |left: OpIndex, right: OpIndex| -> Option<BaseWithScaledIndexAndDisplacementMatch> {
        let mut result = BaseWithScaledIndexAndDisplacementMatch::new();
        result.displacement_mode = DisplacementMode::kPositiveDisplacement;

        if match_scaled_index(selector, left, &mut result.index, &mut result.scale, None) {
            result.displacement_mode = DisplacementMode::kPositiveDisplacement;

            if let Some(right_binop) = selector.get(right).try_cast::<WordBinopOp>() {
                if right_binop.kind == WordBinopOp::Kind::kSub {
                    if let Some(displacement) = selector.match_signed_integral_constant(right_binop.right()) {
                        result.displacement = displacement;
                        result.base = right_binop.left();
                        result.displacement_mode = DisplacementMode::kNegativeDisplacement;
                        return Some(result);
                    } else {
                        return None;
                    }
                }
                if right_binop.kind == WordBinopOp::Kind::kAdd {
                    if let Some(displacement) = selector.match_signed_integral_constant(right_binop.right()) {
                        result.base = right_binop.left();
                    } else if let Some(displacement) = selector.match_signed_integral_constant(right_binop.left()) {
                        result.base = right_binop.right();
                    } else {
                        result.base = right;
                        result.displacement = 0;
                    }
                    return Some(result);
                }
            }

            if let Some(displacement) = selector.match_signed_integral_constant(right) {
                result.base = OpIndex::invalid();
                return Some(result);
            }

            result.base = right;
            result.displacement = 0;
            return Some(result);
        }

        None
    };

    let match_s_plus_plus = |left: OpIndex, right: OpIndex, left_add_left: OpIndex, left_add_right: OpIndex| -> Option<BaseWithScaledIndexAndDisplacementMatch> {
        debug_assert_eq!(selector.get(left).cast::<WordBinopOp>().kind, WordBinopOp::Kind::kAdd);

        let mut result = BaseWithScaledIndexAndDisplacementMatch::new();
        result.displacement_mode = DisplacementMode::kPositiveDisplacement;

        if match_scaled_index(selector, left_add_left, &mut result.index, &mut result.scale, None) {
            result.displacement_mode = DisplacementMode::kPositiveDisplacement;
            if let Some(displacement) = selector.match_signed_integral_constant(left_add_right) {
                result.base = right;
                return Some(result);
            }

            if let Some(displacement) = selector.match_signed_integral_constant(right) {
                result.base = left_add_right;
                return Some(result);
            }

            result.base = left;
            result.index = right;
            result.scale = 0;
            result.displacement = 0;
            return Some(result);
        }

        None
    };

    let match_plus_plus = |left: OpIndex, right: OpIndex| -> Option<BaseWithScaledIndexAndDisplacementMatch> {
        if let Some(left_add) = selector.get(left).try_cast::<WordBinopOp>() {
            if left_add.kind == WordBinopOp::Kind::kAdd {
                if let Some(res) = match_s_plus_plus(left, right, left_add.left(), left_add.right()) {
                    return Some(res);
                }
                if let Some(res) = match_s_plus_plus(left, right, left_add.right(), left_add.left()) {
                    return Some(res);
                }
            }
        }

        None
    };

    if let Some(res) = match_s_plus(left, right) {
        return Some(res);
    }

    if is_commutative {
        if let Some(res) = match_s_plus(right, left) {
            return Some(res);
        }
    }

    if let Some(res) = match_plus_plus(left, right) {
        return Some(res);
    }

    if is_commutative {
        if let Some(res) = match_plus_plus(right, left) {
            return Some(res);
        }
    }

    let mut result = BaseWithScaledIndexAndDisplacementMatch::new();
    result.displacement_mode = DisplacementMode::kPositiveDisplacement;

    if let Some(displacement) = selector.match_signed_integral_constant(right) {
        result.base = left;
        return Some(result);
    }

    result.index = left;
    result.base = right;
    Some(result)
}

fn try_match_base_with_scaled_index_and_displacement64(selector: &InstructionSelectorT, node: OpIndex) -> Option<BaseWithScaledIndexAndDisplacementMatch> {
    let op = selector.get(node);
    if let Some(load) = op.try_cast::<LoadOp>() {
        let mut result = BaseWithScaledIndexAndDisplacementMatch::new();
        result.base = load.base();
        result.index = load.index().unwrap_or(OpIndex::invalid());
        result.scale = load.element_size_log2;
        result.displacement = load.offset;
        if load.kind.tagged_base {
            result.displacement -= kHeapObjectTag as i32;
        }
        return Some(result);
    } else if let Some(store) = op.try_cast::<StoreOp>() {
        let mut result = BaseWithScaledIndexAndDisplacementMatch::new();
        result.base = store.base();
        result.index = store.index().unwrap_or(OpIndex::invalid());
        result.scale = store.element_size_log2;
        result.displacement = store.offset;
        if store.kind.tagged_base {
            result.displacement -= kHeapObjectTag as i32;
        }
        return Some(result);
    }
    
        let binop = op.cast::<WordBinopOp>();
        let left = binop.left();
        let right = binop.right();
        try_match_base_with_scaled_index_and_displacement64_for_word_binop(selector, left, right, binop.IsCommutative())
    
}

fn try_match_base_with_scaled_index_and_displacement32(selector: &InstructionSelectorT, node: OpIndex) -> Option<BaseWithScaledIndexAndDisplacementMatch> {
    try_match_base_with_scaled_index_and_displacement64(selector, node)
}

struct X64OperandGenerator<'a> {
    selector: &'a InstructionSelectorT<'a>,
}

impl<'a> X64OperandGenerator<'a> {
    fn new(selector: &'a InstructionSelectorT) -> Self {
        X64OperandGenerator { selector }
    }

    fn can_be_immediate(&self, node: OpIndex) -> bool {
        can_be_immediate(self.selector, node)
    }

    fn get_immediate_integer_value(&self, node: OpIndex) -> i32 {
        get_immediate_integer_value(self.selector, node)
    }

    fn can_be_memory_operand(&self, opcode: InstructionCode, node: OpIndex, input: OpIndex, effect_level: i32) -> bool {
        if !self.is_load_or_load_immutable(input) {
            return false;
        }
        if !self.selector().can_cover(node, input) {
            return false;
        }

        if effect_level != self.selector().GetEffectLevel(input) {
            return false;
        }

        let rep = self.load_view(input).loaded_rep().representation();
        match opcode {
            kX64And | kX64Or | kX64Xor | kX64Add | kX64Sub | kX64Push | kX64Cmp | kX64Test => {
                rep == MachineRepresentation::kWord64 ||
                    (!crate::common::globals::COMPRESS_POINTERS_BOOL && self.is_any_tagged(rep))
            }
            kX64And32 | kX64Or32 | kX64Xor32 | kX64Add32 | kX64Sub32 | kX64Cmp32 | kX64Test32 => {
                rep == MachineRepresentation::kWord32 ||
                    (crate::common::globals::COMPRESS_POINTERS_BOOL &&
                        (self.is_any_tagged(rep) || self.is_any_compressed(rep)))
            }
            kAVXFloat64Add | kAVXFloat64Sub | kAVXFloat64Mul => {
                rep == MachineRepresentation::kFloat64
            }
            kAVXFloat32Add | kAVXFloat32Sub | kAVXFloat32Mul => {
                rep == MachineRepresentation::kFloat32
            }
            kX64Cmp16 | kX64Test16 => {
                rep == MachineRepresentation::kWord16
            }
            kX64Cmp8 | kX64Test8 => {
                rep == MachineRepresentation::kWord8
            }
            _ => false,
        }
    }

    fn is_zero_int_constant(&self, node: OpIndex) -> bool {
        if let Some(constant) = self.turboshaft_graph().get(node).try_cast::<ConstantOp>() {
            match constant.kind {
                ConstantOp::Kind::kWord32 => return constant.word32() == 0,
                ConstantOp::Kind::kWord64 => return constant.word64() == 0,
                _ => return false,
            }
        }
        false
    }

    fn generate_memory_operand_inputs(
        &self, index: OptionalOpIndex, scale_exponent: i32, base: OpIndex,
        displacement: i64, displacement_mode: DisplacementMode,
        inputs: &mut [InstructionOperand], input_count: &mut usize,
        reg_kind: RegisterUseKind,
    ) -> AddressingMode {
        let mut mode = kMode_MRI;
        let base_before_folding = base;
        let mut fold_base_into_displacement = false;
        let mut fold_value = 0;
        if base.valid() && (index.valid() || displacement != 0) {
            if self.can_be_immediate(base) && index.valid() && value_fits_into_immediate(displacement) {
                fold_value = self.get_immediate_integer_value(base) as i64;
                if displacement_mode == DisplacementMode::kNegativeDisplacement {
                    fold_value -= displacement;
                } else {
                    fold_value += displacement;
                }

                if fold_value == 0 {
                    let _ = base;
                    let _ = displacement;
                } else if value_fits_into_immediate(fold_value) {
                   
                    fold_base_into_displacement = true;
                }
            } else if self.is_zero_int_constant(base) {
                let _ = base;
            }
        }

        if base.valid() {
            inputs[*input_count] = self.UseRegister(base, reg_kind);
            *input_count += 1;
            if index.valid() {
                debug_assert!(scale_exponent >= 0 && scale_exponent <= 3);
                inputs[*input_count] = self.UseRegister(self.value(index), reg_kind);
                *input_count += 1;

                if displacement != 0 {
                    inputs[*input_count] = self.UseImmediate64(if displacement_mode == DisplacementMode::kNegativeDisplacement { -displacement } else { displacement });
                    *input_count += 1;
                    static K_MRNI_MODES: [AddressingMode; 4] = [kMode_MR1I, kMode_MR2I, kMode_MR4I, kMode_MR8I];
                    mode = K_MRNI_MODES[scale_exponent as usize];
                } else {
                    static K_MRN_MODES: [AddressingMode; 4] = [kMode_MR1, kMode_MR2, kMode_MR4, kMode_MR8];
                    mode = K_MRN_MODES[scale_exponent as usize];
                }
            } else {
                if displacement == 0 {
                    mode = kMode_MR;
                } else {
                    inputs[*input_count] = self.UseImmediate64(if displacement_mode == DisplacementMode::kNegativeDisplacement { -displacement } else { displacement });
                    *input_count += 1;
                    mode = kMode_MRI;
                }
            }
        } else {
            debug_assert!(scale_exponent >= 0 && scale_exponent <= 3);
            if fold_base_into_displacement {
                debug_assert!(!base.valid());
                debug_assert!(index.valid());
                inputs[*input_count] = self.UseRegister(self.value(index), reg_kind);
                *input_count += 1;
                inputs[*input_count] = self.UseImmediate(fold_value as i32);
                *input_count += 1;
                static K_MNI_MODES: [AddressingMode; 4] = [kMode_MRI, kMode_M2I, kMode_M4I, kMode_M8I];
                mode = K_MNI_MODES[scale_exponent as usize];
            } else if displacement != 0 {
                if !index.valid() {
                    debug_assert!(self.is_zero_int_constant(base_before_folding));
                    inputs[*input_count] = self.UseRegister(base_before_folding, reg_kind);
                    *input_count += 1;
                    inputs[*input_count] = self.UseImmediate64(if displacement_mode == DisplacementMode::kNegativeDisplacement { -displacement } else { displacement });
                    *input_count += 1;
                    mode = kMode_MRI;
                } else {
                   
                }
            }
        }

        mode
    }

    fn get_effective_address_memory_operand(
        &self, operand: OpIndex, inputs: &mut [InstructionOperand],
        input_count: &mut usize, reg_kind: RegisterUseKind,
    ) -> AddressingMode {
         let op = self.selector.get(operand);
            if let Some(load) = op.try_cast::<LoadOp>() {
                
            }
            let m = try_match_base_with_scaled_index_and_displacement64(self.selector, operand);
            if m.is_none() {
              panic!("No match for {operand:?}");
            }
            let m = m.unwrap();
            if self.is_compressed(m.base) {
              return kMode_MRI;
            }
    
        self.generate_memory_operand_inputs(
        OptionalOpIndex::from(m.index), m.scale, m.base,
        m.displacement, m.displacement_mode,
        inputs, input_count,
        reg_kind,
    )
    }

    fn get_effective_index_operand(&self, index: OpIndex, mode: &mut AddressingMode) -> InstructionOperand {
        if self.can_be_immediate(index) {
            *mode = kMode_MRI;
            self.UseImmediate(index)
        } else {
            *mode = kMode_MR1;
            self.UseUniqueRegister(index)
        }
    }

    fn can_be_better_left_operand(&self, node: OpIndex) -> bool {
        !self.selector().IsReallyLive(node)
    }

    fn turboshaft_graph(&self) -> &Graph {
        self.selector().turboshaft_graph()
    }

    fn is_load_or_load_immutable(&self, node: OpIndex) -> bool {
        self.selector().IsLoadOrLoadImmutable(node)
    }

    fn load_view(&self, node: OpIndex) -> TurboshaftAdapter::LoadView {
        self.selector().load_view(node)
    }

   
    fn UseRegister(&self, node: OpIndex, reg_kind: RegisterUseKind) -> InstructionOperand {
        self.selector().UseRegister(node, reg_kind)
    }

    fn UseImmediate(&self, node: OpIndex) -> InstructionOperand {
        self.selector().UseImmediate(node)
    }

    fn UseUniqueRegister(&self, node: OpIndex) -> InstructionOperand {
        self.selector().UseUniqueRegister(node)
    }

    fn UseAny(&self, node: OpIndex) -> InstructionOperand {
        self.selector().UseAny(node)
    }

    fn UseFixed(&self, node: OpIndex, fixed_register: crate::compiler::backend::register_allocator::Register) -> InstructionOperand {
        self.selector().UseFixed(node, fixed_register)
    }

    fn DefineAsRegister(&self, node: OpIndex) -> InstructionOperand {
        self.selector().DefineAsRegister(node)
    }

     fn DefineSameAsInput(&self, node: OpIndex, index: usize) -> InstructionOperand {
        self.selector().DefineSameAsInput(node, index)
    }

    fn TempRegister(&self) -> InstructionOperand {
        self.selector().TempRegister()
    }

    fn TempSimd128Register(&self) -> InstructionOperand {
        self.selector().TempSimd128Register()
    }

     fn IsProtectedLoad(&self, input: OpIndex) -> bool {
      if let Some(load) = self.selector.get(input).try_cast::<LoadOp>() {
       return load.kind.with_trap_handler;
      }
      return false;
    }
}

impl<'a> std::ops::Deref for X64OperandGenerator<'a> {
    type Target = OperandGeneratorT<'a>;

    fn deref(&self) -> &Self::Target {
        &self.selector.operand_generator_t
    }
}

struct LoadStoreView {
    base: OpIndex,
    index: OptionalOpIndex,
    offset: i32,
}

impl LoadStoreView {
    fn new(op: &Operation) -> Self {
        debug_assert!(op.is::<LoadOp>() || op.is::<StoreOp>());
        if let Some(load) = op.try_cast::<LoadOp>() {
            LoadStoreView {
                base: load.base(),
                index: load.index(),
                offset: load.offset,
            }
        } else {
            let store = op.cast::<StoreOp>();
            LoadStoreView {
                base: store.base(),
                index: store.index(),
                offset: store.offset,
            }
        }
    }
}

fn get_load_opcode(loaded_rep: MemoryRepresentation, result_rep: RegisterRepresentation) -> ArchOpcode {
    match loaded_rep {
        MemoryRepresentation::Int8() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word32());
            ArchOpcode::kX64Movsxbl
        }
        MemoryRepresentation::Uint8() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word32());
            ArchOpcode::kX64Movzxbl
        }
        MemoryRepresentation::Int16() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word32());
            ArchOpcode::kX64Movsxwl
        }
        MemoryRepresentation::Uint16() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word32());
            ArchOpcode::kX64Movzxwl
        }
        MemoryRepresentation::Int32() | MemoryRepresentation::Uint32() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word32());
            ArchOpcode::kX64Movl
        }
        MemoryRepresentation::Int64() | MemoryRepresentation::Uint64() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Word64());
            ArchOpcode::kX64Movq
        }
        MemoryRepresentation::Float16() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Float32());
            ArchOpcode::kX64Movsh
        }
        MemoryRepresentation::Float32() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Float32());
            ArchOpcode::kX64Movss
        }
        MemoryRepresentation::Float64() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Float64());
            ArchOpcode::kX64Movsd
        }
        MemoryRepresentation::AnyTagged() | MemoryRepresentation::TaggedPointer() => {
            if result_rep == RegisterRepresentation::Compressed() {
                ArchOpcode::kX64Movl
            } else {
                debug_assert_eq!(result_rep, RegisterRepresentation::Tagged());
                ArchOpcode::kX64MovqDecompressTagged
            }
        }
        MemoryRepresentation::TaggedSigned() => {
            if result_rep == RegisterRepresentation::Compressed() {
                ArchOpcode::kX64Movl
            } else {
                debug_assert_eq!(result_rep, RegisterRepresentation::Tagged());
                ArchOpcode::kX64MovqDecompressTaggedSigned
            }
        }
        MemoryRepresentation::AnyUncompressedTagged() | MemoryRepresentation::UncompressedTaggedPointer() | MemoryRepresentation::UncompressedTaggedSigned() => {
            debug_assert_eq!(result_rep, RegisterRepresentation::Tagged());
            ArchOpcode::kX64Movq
        }
        MemoryRepresentation::ProtectedPointer() => {
            debug_assert!(crate::common::globals::V8_ENABLE_SANDBOX_BOOL);
            ArchOpcode::kX64MovqDecompressProtected
        }
        MemoryRepresentation::IndirectPointer() => {
            unreachable!()
        }
        MemoryRepresentation::SandboxedPointer() => {
            ArchOpcode::kX64Movq

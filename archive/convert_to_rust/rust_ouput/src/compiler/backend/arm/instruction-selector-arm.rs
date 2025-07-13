// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::convert::TryInto;
use std::i64;
use std::mem;
use std::ops::BitOr;

use crate::base;
use crate::codegen::arm::assembler_arm as assembler;
use crate::codegen::machine_type::*;
use crate::compiler::backend::instruction_selector as instr_sel;
use crate::compiler::backend::instruction_selector_adapter as adapter;
use crate::compiler::instruction_selector::*;
use crate::compiler::turboshaft;
use crate::compiler::turboshaft::opmasks as opmask;
use crate::compiler::turboshaft::operations as ops;

struct ArmOperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT,
}

impl<'a> ArmOperandGeneratorT<'a> {
    pub fn new(selector: &'a mut InstructionSelectorT) -> Self {
        ArmOperandGeneratorT { selector }
    }

    fn CanBeImmediate(&self, value: i32) -> bool {
        assembler::ImmediateFitsAddrMode1Instruction(value)
    }

    fn CanBeImmediate_u32(&self, value: u32) -> bool {
        self.CanBeImmediate(unsafe { mem::transmute::<u32, i32>(value) })
    }

    fn CanBeImmediate_OpIndex(&self, node: OpIndex, opcode: InstructionCode) -> bool {
        let mut value64: i64 = 0;
        if !self.selector.MatchSignedIntegralConstant(node, &mut value64) {
            return false;
        }
        if value64 < i32::MIN.into() || value64 > i32::MAX.into() {
            return false;
        }

        let value = value64 as i32;

        match ArchOpcodeField::decode(opcode) {
            ArchOpcode::kArmAnd | ArchOpcode::kArmMov | ArchOpcode::kArmMvn | ArchOpcode::kArmBic => {
                self.CanBeImmediate(value) || self.CanBeImmediate(!value)
            }
            ArchOpcode::kArmAdd | ArchOpcode::kArmSub | ArchOpcode::kArmCmp | ArchOpcode::kArmCmn => {
                self.CanBeImmediate(value) || self.CanBeImmediate(-value)
            }
            ArchOpcode::kArmTst | ArchOpcode::kArmTeq | ArchOpcode::kArmOrr | ArchOpcode::kArmEor
            | ArchOpcode::kArmRsb => self.CanBeImmediate(value),
            ArchOpcode::kArmVldrF32
            | ArchOpcode::kArmVstrF32
            | ArchOpcode::kArmVldrF64
            | ArchOpcode::kArmVstrF64 => value >= -1020 && value <= 1020 && (value % 4) == 0,
            ArchOpcode::kArmLdrb
            | ArchOpcode::kArmLdrsb
            | ArchOpcode::kArmStrb
            | ArchOpcode::kArmLdr
            | ArchOpcode::kArmStr => value >= -4095 && value <= 4095,
            ArchOpcode::kArmLdrh | ArchOpcode::kArmLdrsh | ArchOpcode::kArmStrh => {
                value >= -255 && value <= 255
            }
            _ => false,
        }
    }
}

fn VisitRR(selector: &mut InstructionSelectorT, opcode: InstructionCode, node: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    selector.Emit(opcode, g.DefineAsRegister(node), selector.input_at(node, 0).UseRegister());
}

fn VisitRRR(selector: &mut InstructionSelectorT, opcode: InstructionCode, node: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    selector.Emit(
        opcode,
        g.DefineAsRegister(node),
        selector.input_at(node, 0).UseRegister(),
        selector.input_at(node, 1).UseRegister(),
    );
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
fn VisitSimdShiftRRR(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex, width: i32) {
    let g = ArmOperandGeneratorT::new(selector);
    let op = selector.Get(node).Cast::<ops::Simd128ShiftOp>();
    let mut shift_by: i32 = 0;
    if selector.MatchIntegralWord32Constant(op.shift(), &mut shift_by) {
        if shift_by % width == 0 {
            selector.EmitIdentity(node);
        } else {
            selector.Emit(opcode as InstructionCode, g.DefineAsRegister(node), op.input().UseRegister(), g.UseImmediate(op.shift()));
        }
    } else {
        VisitRRR(selector, opcode as InstructionCode, node);
    }
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
fn VisitRRRShuffle(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex, input0: OpIndex, input1: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    let mut input0_mut = input0;
    let mut input1_mut = input1;
    // Swap inputs to save an instruction in the CodeGenerator for High ops.
    match opcode {
        ArchOpcode::kArmS32x4ZipRight
        | ArchOpcode::kArmS32x4UnzipRight
        | ArchOpcode::kArmS32x4TransposeRight
        | ArchOpcode::kArmS16x8ZipRight
        | ArchOpcode::kArmS16x8UnzipRight
        | ArchOpcode::kArmS16x8TransposeRight
        | ArchOpcode::kArmS8x16ZipRight
        | ArchOpcode::kArmS8x16UnzipRight
        | ArchOpcode::kArmS8x16TransposeRight => {
            mem::swap(&mut input0_mut, &mut input1_mut);
        }
        _ => {}
    }

    selector.Emit(
        opcode as InstructionCode,
        g.DefineSameAsFirst(node),
        input0_mut.UseRegister(),
        input1_mut.UseRegister(),
    );
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
fn VisitRRI(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    let op = selector.Get(node);
    let imm = op.Cast::<ops::Simd128ExtractLaneOp>().lane;
    selector.Emit(opcode as InstructionCode, g.DefineAsRegister(node), op.input(0).UseRegister(), g.UseImmediate(imm));
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
fn VisitRRIR(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    let op = selector.Get(node).Cast::<ops::Simd128ReplaceLaneOp>();
    selector.Emit(
        opcode as InstructionCode,
        g.DefineAsRegister(node),
        op.into().UseRegister(),
        g.UseImmediate(op.lane),
        op.new_lane().UseUniqueRegister(),
    );
}

fn TryMatchShift<OpmaskT: Copy, const kImmMin: i32, const kImmMax: i32, const kImmMode: AddressingMode, const kRegMode: AddressingMode>(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool
where
    opmask::Opmask: opmask::MatchOp<OpmaskT>,
{
    let g = ArmOperandGeneratorT::new(selector);
    let op = selector.Get(node);
    if op.Is::<OpmaskT>() {
        let shift = op.Cast::<ops::ShiftOp>();
        *value_return = shift.left().UseRegister();
        let mut shift_by: i32 = 0;
        if selector.MatchIntegralWord32Constant(shift.right(), &mut shift_by)
            && shift_by >= kImmMin && shift_by <= kImmMax
        {
            *opcode_return = AddressingModeField::encode(*opcode_return, kImmMode);
            *shift_return = shift.right().UseImmediate();
        } else {
            *opcode_return = AddressingModeField::encode(*opcode_return, kRegMode);
            *shift_return = shift.right().UseRegister();
        }
        return true;
    }
    false
}

fn TryMatchShiftImmediate<OpmaskT: Copy, const kImmMin: i32, const kImmMax: i32, const kImmMode: AddressingMode>(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool
where
    opmask::Opmask: opmask::MatchOp<OpmaskT>,
{
    let g = ArmOperandGeneratorT::new(selector);
    let op = selector.Get(node);
    if op.Is::<OpmaskT>() {
        let shift = op.Cast::<ops::ShiftOp>();
        let mut shift_by: i32 = 0;
        if selector.MatchIntegralWord32Constant(shift.right(), &mut shift_by)
            && shift_by >= kImmMin && shift_by <= kImmMax
        {
            *opcode_return = AddressingModeField::encode(*opcode_return, kImmMode);
            *value_return = shift.left().UseRegister();
            *shift_return = shift.right().UseImmediate();
            return true;
        }
    }
    false
}

fn TryMatchROR(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchShift::<opmask::OpmaskkWord32RotateRight, 1, 31, AddressingMode::kMode_Operand2_R_ROR_I, AddressingMode::kMode_Operand2_R_ROR_R>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    )
}

fn TryMatchASR(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchShift::<opmask::OpmaskkWord32ShiftRightArithmetic, 1, 32, AddressingMode::kMode_Operand2_R_ASR_I, AddressingMode::kMode_Operand2_R_ASR_R>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    ) || TryMatchShift::<opmask::OpmaskkWord32ShiftRightArithmeticShiftOutZeros, 1, 32, AddressingMode::kMode_Operand2_R_ASR_I, AddressingMode::kMode_Operand2_R_ASR_R>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    )
}

fn TryMatchLSL(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchShift::<opmask::OpmaskkWord32ShiftLeft, 0, 31, AddressingMode::kMode_Operand2_R_LSL_I, AddressingMode::kMode_Operand2_R_LSL_R>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    )
}

fn TryMatchLSLImmediate(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchShiftImmediate::<opmask::OpmaskkWord32ShiftLeft, 0, 31, AddressingMode::kMode_Operand2_R_LSL_I>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    )
}

fn TryMatchLSR(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchShift::<opmask::OpmaskkWord32ShiftRightLogical, 1, 32, AddressingMode::kMode_Operand2_R_LSR_I, AddressingMode::kMode_Operand2_R_LSR_R>(
        selector,
        opcode_return,
        node,
        value_return,
        shift_return,
    )
}

fn TryMatchShift(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    value_return: &mut InstructionOperand,
    shift_return: &mut InstructionOperand,
) -> bool {
    TryMatchASR(selector, opcode_return, node, value_return, shift_return)
        || TryMatchLSL(selector, opcode_return, node, value_return, shift_return)
        || TryMatchLSR(selector, opcode_return, node, value_return, shift_return)
        || TryMatchROR(selector, opcode_return, node, value_return, shift_return)
}

fn TryMatchImmediateOrShift(
    selector: &mut InstructionSelectorT,
    opcode_return: &mut InstructionCode,
    node: OpIndex,
    input_count_return: &mut usize,
    inputs: &mut [InstructionOperand],
) -> bool {
    let g = ArmOperandGeneratorT::new(selector);
    if g.CanBeImmediate_OpIndex(node, *opcode_return) {
        *opcode_return = AddressingModeField::encode(*opcode_return, AddressingMode::kMode_Operand2_I);
        inputs[0] = node.UseImmediate();
        *input_count_return = 1;
        return true;
    }
    if TryMatchShift(selector, opcode_return, node, &mut inputs[0], &mut inputs[1]) {
        *input_count_return = 2;
        return true;
    }
    false
}

fn VisitBinop(
    selector: &mut InstructionSelectorT,
    node: OpIndex,
    opcode: InstructionCode,
    reverse_opcode: InstructionCode,
    cont: &mut FlagsContinuationT,
) {
    let g = ArmOperandGeneratorT::new(selector);
    let lhs = selector.input_at(node, 0);
    let rhs = selector.input_at(node, 1);
    let mut inputs: [InstructionOperand; 3] = [InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID];
    let mut input_count: usize = 0;
    let mut outputs: [InstructionOperand; 1] = [InstructionOperand::INVALID];
    let mut output_count: usize = 0;

    if lhs == rhs {
        let input = lhs.UseRegister();
        *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Operand2_R);
        inputs[input_count] = input;
        input_count += 1;
        inputs[input_count] = input;
        input_count += 1;
    } else if TryMatchImmediateOrShift(selector, &mut opcode, rhs, &mut input_count, &mut inputs[1..])) {
        inputs[0] = lhs.UseRegister();
        input_count += 1;
    } else if TryMatchImmediateOrShift(selector, &mut reverse_opcode, lhs, &mut input_count, &mut inputs[1..]) {
        inputs[0] = rhs.UseRegister();
        opcode = reverse_opcode;
        input_count += 1;
    } else {
        *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Operand2_R);
        inputs[input_count] = lhs.UseRegister();
        input_count += 1;
        inputs[input_count] = rhs.UseRegister();
        input_count += 1;
    }

    outputs[output_count] = g.DefineAsRegister(node);
    output_count += 1;

    selector.EmitWithContinuation(
        opcode,
        output_count as usize,
        &mut outputs,
        input_count as usize,
        &mut inputs,
        cont,
    );
}

fn VisitBinop(selector: &mut InstructionSelectorT, node: OpIndex, opcode: InstructionCode, reverse_opcode: InstructionCode) {
    let mut cont = FlagsContinuationT::default();
    VisitBinop(selector, node, opcode, reverse_opcode, &mut cont);
}

fn EmitDiv(selector: &mut InstructionSelectorT, div_opcode: ArchOpcode, f64i32_opcode: ArchOpcode, i32f64_opcode: ArchOpcode, result_operand: InstructionOperand, left_operand: InstructionOperand, right_operand: InstructionOperand) {
    let g = ArmOperandGeneratorT::new(selector);
    if selector.IsSupported(SUDIV) {
        selector.Emit(div_opcode as InstructionCode, result_operand, left_operand, right_operand);
        return;
    }

    let left_double_operand = g.TempDoubleRegister();
    let right_double_operand = g.TempDoubleRegister();
    let result_double_operand = g.TempDoubleRegister();
    selector.Emit(f64i32_opcode as InstructionCode, left_double_operand, left_operand);
    selector.Emit(f64i32_opcode as InstructionCode, right_double_operand, right_operand);
    selector.Emit(ArchOpcode::kArmVdivF64 as InstructionCode, result_double_operand, left_double_operand, right_double_operand);
    selector.Emit(i32f64_opcode as InstructionCode, result_operand, result_double_operand);
}

fn VisitDiv(selector: &mut InstructionSelectorT, node: OpIndex, div_opcode: ArchOpcode, f64i32_opcode: ArchOpcode, i32f64_opcode: ArchOpcode) {
    let g = ArmOperandGeneratorT::new(selector);
    EmitDiv(selector, div_opcode, f64i32_opcode, i32f64_opcode, g.DefineAsRegister(node), selector.input_at(node, 0).UseRegister(), selector.input_at(node, 1).UseRegister());
}

fn VisitMod(selector: &mut InstructionSelectorT, node: OpIndex, div_opcode: ArchOpcode, f64i32_opcode: ArchOpcode, i32f64_opcode: ArchOpcode) {
    let g = ArmOperandGeneratorT::new(selector);
    let div_operand = g.TempRegister();
    let result_operand = g.DefineAsRegister(node);
    let left_operand = selector.input_at(node, 0).UseRegister();
    let right_operand = selector.input_at(node, 1).UseRegister();

    EmitDiv(selector, div_opcode, f64i32_opcode, i32f64_opcode, div_operand, left_operand, right_operand);

    if selector.IsSupported(ARMv7) {
        selector.Emit(ArchOpcode::kArmMls as InstructionCode, result_operand, div_operand, right_operand, left_operand);
    } else {
        let mul_operand = g.TempRegister();
        selector.Emit(ArchOpcode::kArmMul as InstructionCode, mul_operand, div_operand, right_operand);
        selector.Emit(ArchOpcode::kArmSub as InstructionCode | AddressingModeField::encode(AddressingMode::kMode_Operand2_R), result_operand, left_operand, mul_operand);
    }
}

fn EmitAddBeforeS128LoadStore(selector: &mut InstructionSelectorT, opcode_return: &mut InstructionCode, input_count_return: &mut usize, inputs: &mut [InstructionOperand]) {
    let g = ArmOperandGeneratorT::new(selector);
    let addr = g.TempRegister();
    let op = ArchOpcode::kArmAdd as InstructionCode;
    let op = AddressingModeField::encode(op, AddressingMode::kMode_Operand2_R);
    selector.Emit(op, 1, &addr, 2, inputs);
    *opcode_return = AddressingModeField::encode(*opcode_return, AddressingMode::kMode_Operand2_R);
    *input_count_return -= 1;
    inputs[0] = addr;
}

fn EmitLoad(selector: &mut InstructionSelectorT, opcode: InstructionCode, output: &mut InstructionOperand, base: OpIndex, index: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    let mut inputs: [InstructionOperand; 3] = [InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID];
    let mut input_count: usize = 2;

    let base_op = selector.Get(base);
    let mut index_constant: i64 = 0;

    if base_op.Is::<opmask::OpmaskkExternalConstant>() && selector.MatchSignedIntegralConstant(index, &mut index_constant) {
        let constant_base = base_op.Cast::<ops::ConstantOp>();

        if selector.CanAddressRelativeToRootsRegister(constant_base.external_reference()) {
            let delta: i64 = index_constant
                + MacroAssemblerBase::RootRegisterOffsetForExternalReference(
                    selector.isolate(),
                    constant_base.external_reference(),
                );
            input_count = 1;
            inputs[0] = delta.UseImmediate(delta as i32);
            *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Root);
            selector.Emit(opcode, 1, &mut *output, input_count, &inputs);
            return;
        }
    }

    if selector.is_load_root_register(base) {
        input_count = 1;
        inputs[0] = index.UseImmediate();
        *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Root);
        selector.Emit(opcode, 1, &mut *output, input_count, &inputs);
        return;
    }

    inputs[0] = base.UseRegister();
    if g.CanBeImmediate_OpIndex(index, opcode) {
        inputs[1] = index.UseImmediate();
        *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Offset_RI);
    } else if *opcode == ArchOpcode::kArmLdr as InstructionCode && TryMatchLSLImmediate(selector, opcode, index, &mut inputs[1], &mut inputs[2])) {
        input_count = 3;
    } else {
        inputs[1] = index.UseRegister();
        if *opcode == ArchOpcode::kArmVld1S128 as InstructionCode {
            EmitAddBeforeS128LoadStore(selector, opcode, &mut input_count, &mut inputs[0..]);
        } else {
            *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Offset_RR);
        }
    }
    selector.Emit(opcode, 1, &mut *output, input_count, &inputs);
}

fn EmitStore(selector: &mut InstructionSelectorT, opcode: InstructionCode, input_count: usize, inputs: &mut [InstructionOperand], index: OpIndex) {
    let g = ArmOperandGeneratorT::new(selector);
    let mut input_count_mut = input_count;
    let arch_opcode = ArchOpcodeField::decode(opcode);

    if g.CanBeImmediate_OpIndex(index, opcode) {
        inputs[input_count_mut] = index.UseImmediate();
        *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Offset_RI);
        
    } else if (arch_opcode == ArchOpcode::kArmStr || arch_opcode == ArchOpcode::kAtomicStoreWord32)
        && TryMatchLSLImmediate(selector, opcode, index, &mut inputs[2], &mut inputs[3]) {
        *&mut input_count_mut = 4;
    } else {
        inputs[input_count_mut] = index.UseRegister();
        
        if arch_opcode == ArchOpcode::kArmVst1S128 {
            EmitAddBeforeS128LoadStore(selector, opcode, &mut input_count_mut, &mut inputs[1..]);
        } else {
             *opcode = AddressingModeField::encode(*opcode, AddressingMode::kMode_Offset_RR);
        }
    }
     selector.Emit(opcode, 0, &mut [], input_count_mut, inputs);
}

fn VisitPairAtomicBinOp(selector: &mut InstructionSelectorT, node: OpIndex, opcode: ArchOpcode) {
    let g = ArmOperandGeneratorT::new(selector);
    let base = selector.input_at(node, 0);
    let index = selector.input_at(node, 1);
    let value = selector.input_at(node, 2);
    let value_high = selector.input_at(node, 3);
    let addressing_mode = AddressingMode::kMode_Offset_RR;
    let code = opcode as InstructionCode | AddressingModeField::encode(addressing_mode);
    let inputs = [
        value.UseUniqueRegister(),
        value_high.UseUniqueRegister(),
        base.UseUniqueRegister(),
        index.UseUniqueRegister(),
    ];
    let mut outputs: [InstructionOperand; 2] = [InstructionOperand::INVALID, InstructionOperand::INVALID];
    let mut output_count: usize = 0;
    let mut temps: [InstructionOperand; 6] = [InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID];
    let mut temp_count: usize = 0;
    temps[temp_count] = g.TempRegister();
    temp_count += 1;
    temps[temp_count] = r6.TempRegister();
    temp_count += 1;
    temps[temp_count] = r7.TempRegister();
    temp_count += 1;
    temps[temp_count] = g.TempRegister();
    temp_count += 1;

    let projection0 = selector.FindProjection(node, 0);
    let projection1 = selector.FindProjection(node, 1);
    if projection0.valid() {
        outputs[output_count] = projection0.value().DefineAsFixed(r2);
        output_count += 1;
    } else {
        temps[temp_count] = r2.TempRegister();
        temp_count += 1;
    }
    if projection1.valid() {
        outputs[output_count] = projection1.value().DefineAsFixed(r3);
        output_count += 1;
    } else {
        temps[temp_count] = r3.TempRegister();
        temp_count += 1;
    }
    selector.Emit(code, output_count, &mut outputs, inputs.len(), &inputs, temp_count, &temps);
}

impl InstructionSelectorT {
    pub fn VisitStackSlot(&mut self, node: OpIndex) {
        let stack_slot = self.Get(node).Cast::<ops::StackSlotOp>();
        let slot = self.frame_.AllocateSpillSlot(stack_slot.size, stack_slot.alignment, stack_slot.is_tagged);
        let g = ArmOperandGeneratorT::new(self);
        self.Emit(ArchOpcode::kArchStackSlot as InstructionCode, g.DefineAsRegister(node), self.sequence().AddImmediate(Constant(slot)), 0, &mut []);
    }

    pub fn VisitAbortCSADcheck(&mut self, node: OpIndex) {
        let g = ArmOperandGeneratorT::new(self);
        self.Emit(ArchOpcode::kArchAbortCSADcheck as InstructionCode, g.NoOutput(), self.input_at(node, 0).UseFixed(r1));
    }
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
mod wasm_utils {
    use crate::codegen::machine_type::*;
    use crate::compiler::turboshaft::operations as ops;
    pub(crate) fn MachineRepresentationOf(lane_kind: ops::Simd128LaneMemoryOpLaneKind) -> MachineRepresentation {
        match lane_kind {
            ops::Simd128LaneMemoryOpLaneKind::k8 => MachineRepresentation::kWord8,
            ops::Simd128LaneMemoryOpLaneKind::k16 => MachineRepresentation::kWord16,
            ops::Simd128LaneMemoryOpLaneKind::k32 => MachineRepresentation::kWord32,
            ops::Simd128LaneMemoryOpLaneKind::k64 => MachineRepresentation::kWord64,
        }
    }
}

#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
impl InstructionSelectorT {
    fn VisitStoreLane(&mut self, node: OpIndex) {
        use crate::compiler::backend::arm::instruction_selector_arm::wasm_utils::*;
        let store = self.Get(node).Cast::<ops::Simd128LaneMemoryOp>();
        let f = LoadStoreLaneParams::new(MachineRepresentationOf(store.lane_kind), store.lane);
        let mut opcode = if f.low_op {
            ArchOpcode::kArmS128StoreLaneLow as InstructionCode
        } else {
            ArchOpcode::kArmS128StoreLaneHigh as InstructionCode
        };
        opcode |= MiscField::encode(f.sz);

        let g = ArmOperandGeneratorT::new(self);
        let mut inputs: [InstructionOperand; 4] = [InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID];
        let mut input_count: usize = 4;
        inputs[0] = store.value().UseRegister();
        inputs[1] = f.laneidx.UseImmediate();
        inputs[2] = store.base().UseRegister();
        inputs[3] = store.index().UseRegister();
        EmitAddBeforeS128LoadStore(self, &mut opcode, &mut input_count, &mut inputs[2..]);
        self.Emit(opcode, 0, &mut [], input_count, &inputs);
    }

    fn VisitLoadLane(&mut self, node: OpIndex) {
        use crate::compiler::backend::arm::instruction_selector_arm::wasm_utils::*;
        let load = self.Get(node).Cast::<ops::Simd128LaneMemoryOp>();
        let f = LoadStoreLaneParams::new(MachineRepresentationOf(load.lane_kind), load.lane);
        let mut opcode = if f.low_op {
            ArchOpcode::kArmS128LoadLaneLow as InstructionCode
        } else {
            ArchOpcode::kArmS128LoadLaneHigh as InstructionCode
        };
        opcode |= MiscField::encode(f.sz);

        let g = ArmOperandGeneratorT::new(self);
        let mut output = g.DefineSameAsFirst(node);
        let mut inputs: [InstructionOperand; 4] = [InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID, InstructionOperand::INVALID];
        let mut input_count: usize = 4;
        inputs[0] = load.value().UseRegister();
        inputs[1] = f.laneidx.UseImmediate();
        inputs[2] = load.base().UseRegister();
        inputs[3] = load.index().UseRegister();
        EmitAddBeforeS128LoadStore(self, &mut opcode, &mut input_count, &mut inputs[2..]);
        self.Emit(opcode, 1, &mut output, input_count, &inputs);
    }

    fn VisitLoadTransform(&mut self, node: OpIndex) {
        let op = self.Get(node).Cast::<ops::Simd128LoadTransformOp>();
        let opcode: InstructionCode = match op.transform_kind {
            ops::Simd128LoadTransformOpTransformKind::k8Splat => ArchOpcode::kArmS128Load8Splat as InstructionCode,
            ops::Simd128LoadTransformOpTransformKind::k16Splat => ArchOpcode::kArmS128Load16Splat as InstructionCode,
            ops::Simd128LoadTransformOpTransformKind::k32Splat => ArchOpcode::kArmS128Load32Splat as InstructionCode,
            ops::Simd128LoadTransformOpTransformKind::k64Splat => ArchOpcode::kArmS128Load64Splat as InstructionCode,
            ops::Simd128LoadTransformOpTransformKind::k8x8S => ArchOpcode::kArmS128Load8x8S as InstructionCode,
            ops::Simd128LoadTransformOpTransformKind::k8x8U => ArchOpcode::kArmS128Load8x8U as InstructionCode,
            ops::Simd

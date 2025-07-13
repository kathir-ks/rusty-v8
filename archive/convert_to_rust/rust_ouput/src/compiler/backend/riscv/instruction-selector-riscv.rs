// Converted from V8 C++ source files:
// Header: instruction-selector-riscv.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::mem;
use std::ops::{BitAnd, BitOr, BitXor};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use crate::base;
use crate::codegen::machine_type::MachineType;
use crate::common::globals::COMPRESS_POINTERS_BOOL;
use crate::compiler::backend::instruction_codes::InstructionCode;
use crate::compiler::backend::instruction_selector_impl::OperandGeneratorT;
use crate::compiler::backend::instruction_selector_impl::InstructionSelectorT;
use crate::compiler::machine_operator::kArchNop;
use crate::compiler::turboshaft::operation_matcher::MatchZero;
use crate::compiler::turboshaft::operations::Operation;
use crate::compiler::turboshaft::opmasks::OpIndex;
use crate::compiler::turboshaft::representations::RegisterRepresentation;
use crate::flags::flags::V8_STATIC_ROOTS_BOOL;
use crate::handles::HeapObject;
use crate::handles::Handle;
use crate::instruction_sequence::InstructionSequence;
use crate::instruction::Instruction;
use crate::machine_type::MachineType;
use crate::simulator::a0;
use crate::wasm::wasm_code_manager::kSimd128ScratchReg;

pub struct RootsTable {}
pub struct RootIndex {}
pub trait RootsTableTrait {
    fn IsRootHandle(&self, value: Handle<HeapObject>, root_index: &mut RootIndex) -> bool;
    fn IsReadOnly(root_index: RootIndex) -> bool;
}

impl RootsTableTrait for RootsTable {
    fn IsRootHandle(&self, value: Handle<HeapObject>, root_index: &mut RootIndex) -> bool {
        false
    }
    fn IsReadOnly(root_index: RootIndex) -> bool {
        false
    }
}
pub struct Isolate {
    bootstrapper_: bool,
    roots_table_: RootsTable,
}

impl Isolate {
    pub fn bootstrapper(&self) -> bool {
        self.bootstrapper_
    }
    pub fn roots_table(&self) -> &RootsTable {
        &self.roots_table_
    }
}
#[derive(Debug, Copy, Clone)]
pub struct ConstantOp {
    pub kind: ConstantOpKind,
    pub integral: i64,
}

impl ConstantOp {
    pub fn IsIntegral(&self) -> bool {
        match self.kind {
            ConstantOpKind::kInteger => true,
            _ => false,
        }
    }

    pub fn integral(&self) -> i64 {
        self.integral
    }

    pub fn float32(&self) -> Float32 {
        Float32 { bits: 0 }
    }

    pub fn float64(&self) -> Float64 {
        Float64 { bits: 0 }
    }
    pub fn handle(&self) -> Handle<HeapObject>{
        Handle::new()
    }
    pub fn is_compressed_heap_object(&self) -> bool {
        match self.kind {
            ConstantOpKind::kCompressedHeapObject => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ConstantOpKind {
    kInteger,
    kFloat32,
    kFloat64,
    kCompressedHeapObject,
}

#[derive(Debug, Copy, Clone)]
pub struct Float32 {
    bits: u32,
}
impl Float32 {
    pub fn get_bits(&self) -> u32 {
        self.bits
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Float64 {
    bits: u64,
}
impl Float64 {
    pub fn get_bits(&self) -> u64 {
        self.bits
    }
}
pub struct StackSlotOp {
    pub size: i32,
    pub alignment: i32,
    pub is_tagged: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum VSew{
    E8,
    E16,
    E32,
    E64,
}

#[derive(Debug, Copy, Clone)]
pub enum Vlmul{
    m1,
    mf2,
}
pub enum MaskType{
    Mask,
}
pub struct OperandGenerator {}
pub type OptionalOpIndex = Option<OpIndex>;
pub struct RiscvOperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT,
}

impl<'a> RiscvOperandGeneratorT<'a> {
    pub fn new(selector: &'a mut InstructionSelectorT) -> Self {
        RiscvOperandGeneratorT { selector }
    }

    pub fn UseOperand(&mut self, node: OpIndex, opcode: InstructionCode) -> InstructionOperand {
        if self.CanBeImmediate(node, opcode) {
            return self.UseImmediate(node);
        }
        self.UseRegister(node)
    }

    // Use the zero register if the node has the immediate value zero, otherwise
    // assign a register.
    pub fn UseRegisterOrImmediateZero(&mut self, node: OpIndex) -> InstructionOperand {
        if let Some(constant) = self.selector.Get(node).TryCast::<ConstantOp>() {
            if (constant.IsIntegral() && constant.integral() == 0)
                || (constant.kind == ConstantOpKind::kFloat32 && constant.float32().get_bits() == 0)
                || (constant.kind == ConstantOpKind::kFloat64 && constant.float64().get_bits() == 0)
            {
                return self.UseImmediate(node);
            }
        }
        self.UseRegister(node)
    }

    pub fn IsIntegerConstant(&mut self, node: OpIndex) -> bool {
        let mut unused = 0;
        self.selector.MatchSignedIntegralConstant(node, &mut unused)
    }

    pub fn IsIntegerConstantOptional(&mut self, node: OptionalOpIndex) -> bool {
        node.is_some() && self.IsIntegerConstant(node.unwrap())
    }

    pub fn GetOptionalIntegerConstant(&mut self, operation: OpIndex) -> Option<i64> {
        let mut constant: i64 = 0;
        if self.selector.MatchSignedIntegralConstant(operation, &mut constant) {
            return Some(constant);
        }
        None
    }

    pub fn CanBeZero(&mut self, node: OpIndex) -> bool {
        MatchZero(node, &self.selector.Get(node))
    }

    pub fn CanBeImmediate(&mut self, node: OpIndex, mode: InstructionCode) -> bool {
        let constant = self.selector.Get(node).TryCast::<ConstantOp>();
        if constant.is_none() {
            return false;
        }
        let constant = constant.unwrap();
        if constant.kind == ConstantOpKind::kCompressedHeapObject {
            if !COMPRESS_POINTERS_BOOL {
                return false;
            }
            // For builtin code we need static roots
            if self.selector.isolate.bootstrapper() && !V8_STATIC_ROOTS_BOOL {
                return false;
            }
            let roots_table = self.selector.isolate.roots_table();
            let mut root_index: RootIndex = RootIndex {};
            let value: Handle<HeapObject> = constant.handle();
            if roots_table.IsRootHandle(value, &mut root_index) {
                if !<dyn RootsTableTrait>::IsReadOnly(root_index) {
                    return false;
                }
                return self.CanBeImmediate(
                    0,
                    mode,
                ); //MacroAssemblerBase::ReadOnlyRootPtr(root_index, selector().isolate()), mode);
            }
            return false;
        }

        let mut value: i64 = 0;
        self.selector.MatchSignedIntegralConstant(node, &mut value)
            && self.CanBeImmediateI64(value, mode)
    }

    pub fn CanBeImmediateI64(&mut self, value: i64, opcode: InstructionCode) -> bool {
        // Placeholder implementation
        true
    }

    fn ImmediateFitsAddrMode1Instruction(&self, imm: i32) -> bool {
        false
    }

    fn UseImmediate(&mut self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn UseRegister(&mut self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn DefineAsRegister(&mut self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn DefineSameAsFirst(&mut self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn UseUniqueRegister(&mut self, input_at: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
    fn UseFixed(&mut self, input_at: OpIndex, a0: i32) -> InstructionOperand{
        InstructionOperand {}
    }
    fn NoOutput(&mut self) -> InstructionOperand{
        InstructionOperand {}
    }
    fn TempRegister(&mut self) -> InstructionOperand{
        InstructionOperand {}
    }
    fn TempImmediate(&mut self, value: i32) -> InstructionOperand{
        InstructionOperand {}
    }
    fn UseImmediate64(&mut self, value: i64) -> InstructionOperand{
        InstructionOperand {}
    }
    fn TempFpRegister(&mut self, v0: i32) -> InstructionOperand{
        InstructionOperand {}
    }
    fn Use(&mut self, hi: OpIndex) -> InstructionOperand{
        InstructionOperand {}
    }
    fn TempDoubleRegister(&mut self) -> InstructionOperand{
        InstructionOperand {}
    }
}

// Add implementations for the methods of InstructionSelectorT here

impl InstructionSelectorT {
    pub fn new() -> Self {
        InstructionSelectorT {}
    }
    fn MatchSignedIntegralConstant(&mut self, node: OpIndex, value: &mut i64) -> bool {
        true
    }
    fn Get(&mut self, node: OpIndex) -> &dyn OperationTrait {
        todo!()
    }
    fn value_input_count(&self, node: OpIndex) -> i32 {
        2
    }
    fn input_at(&self, node: OpIndex, i: i32) -> OpIndex {
        OpIndex {}
    }
    fn is_protected(&self, traps_on_null: &mut bool) -> bool{
        false
    }
    fn sequence(&mut self) -> &mut InstructionSequence {
        todo!()
    }
    fn isolate(&self) -> &Isolate {
        todo!()
    }
    fn EmitWithContinuation(&mut self, opcode: InstructionCode, output_count: usize, outputs: &[InstructionOperand], input_count: usize, inputs: &[InstructionOperand], cont: &FlagsContinuationT) -> &mut Self{
        self
    }
    fn EmitWithContinuation(&mut self, opcode: InstructionCode, output_count: usize, outputs: &[InstructionOperand], input_count: usize, inputs: &[InstructionOperand], temps_count: usize, temps: &[InstructionOperand], cont: &FlagsContinuationT) -> &mut Self{
        self
    }
    fn EmitWithContinuation(&mut self, opcode: InstructionCode, left: InstructionOperand, right: InstructionOperand, cont: &FlagsContinuationT) -> &mut Instruction{
        todo!()
    }
    fn Emit(&mut self, opcode: InstructionCode, dst: InstructionOperand) -> &mut Self{
        self
    }
    fn Emit(&mut self, opcode: InstructionCode, output: InstructionOperand, input1: InstructionOperand, input2: InstructionOperand) -> &mut Self{
        self
    }
    fn Emit(&mut self, opcode: InstructionCode, output: InstructionOperand, input1: InstructionOperand, input2: InstructionOperand, input3: InstructionOperand) -> &mut Self{
        self
    }
    fn Emit(&mut self, opcode: InstructionCode, output: InstructionOperand, input1: InstructionOperand, input2: InstructionOperand, input3: InstructionOperand, input4: InstructionOperand) -> &mut Self{
        self
    }
    fn AddImmediate(&self, constant: ConstantOp) -> InstructionOperand {
        InstructionOperand {}
    }
    fn frame(&self) -> &Frame {
        todo!()
    }
    fn simd_shuffle_view(&self, node: OpIndex) -> SimdShuffleView{
        SimdShuffleView {}
    }
    fn MarkAsFloat32(&mut self, node: OpIndex){}
    fn MarkAsFloat64(&mut self, node: OpIndex){}
    fn MarkAsSimd128(&mut self, node: OpIndex){}
    fn DefineSameAsFirst(&mut self, node: OpIndex) -> InstructionOperand{
        InstructionOperand {}
    }
}
struct Frame {}
impl Frame {
    fn AllocateSpillSlot(&self, size: i32, alignment: i32, is_tagged: bool) -> i32{
        0
    }
}

pub struct FlagsContinuationT {}

impl FlagsContinuationT {
    fn IsDeoptimize(&self) -> bool {
        false
    }
    fn Commute(&mut self) {}
    fn condition(&self) -> i32{
        0
    }
    fn IsSet(&self) -> bool {
        false
    }
    fn ForSet(equal: i32, node: OpIndex) -> Self {
        FlagsContinuationT {}
    }
}

pub trait OperationTrait {
    fn TryCast<T>() -> Option<T>
    where
        T: OperationTrait + Copy,
    {
        None
    }
    fn Is<T>() -> bool
    where
        T: OperationTrait + Copy,
    {
        false
    }
    fn Cast<T>(&self) -> T
    where
        T: OperationTrait + Copy,
    {
        todo!()
    }
    fn input(&self, i: i32) -> OpIndex {
        OpIndex{}
    }
    fn is(&self, opcode: ArchOpcode) -> bool {
        false
    }
}

impl OperationTrait for ConstantOp {
    fn TryCast<ConstantOp>() -> Option<ConstantOp> {
        todo!()
    }
}

impl OperationTrait for StackSlotOp {
    fn TryCast<StackSlotOp>() -> Option<StackSlotOp> {
        todo!()
    }
}

pub struct Simd128ExtractLaneOp {
    pub lane: i32,
}

impl OperationTrait for Simd128ExtractLaneOp {
    fn Cast<Simd128ExtractLaneOp>(&self) -> Simd128ExtractLaneOp {
        Simd128ExtractLaneOp { lane: 0 }
    }
    fn input(&self, i: i32) -> OpIndex {
        OpIndex {}
    }
}

pub struct Simd128ReplaceLaneOp {
    pub lane: i32,
}

impl OperationTrait for Simd128ReplaceLaneOp {
    fn Cast<Simd128ReplaceLaneOp>(&self) -> Simd128ReplaceLaneOp {
        Simd128ReplaceLaneOp { lane: 0 }
    }
    fn input(&self, i: i32) -> OpIndex {
        OpIndex {}
    }
}

pub struct ComparisonOp {}

impl OperationTrait for ComparisonOp {
    fn Cast<ComparisonOp>(&self) -> ComparisonOp {
        ComparisonOp {}
    }
    fn input(&self, i: i32) -> OpIndex {
        OpIndex {}
    }
    fn left(&self) -> OpIndex{
        OpIndex {}
    }
    fn right(&self) -> OpIndex{
        OpIndex {}
    }
}

pub struct SwitchInfo {}
#[derive(Copy, Clone)]
pub enum ArchOpcode{

}

pub struct LinkageLocation {

}
pub struct PushParameter {
    pub location: LinkageLocation,
    pub node: OpIndex,
}
pub struct CallDescriptor {}

impl CallDescriptor {
    fn IsCFunctionCall(&self) -> bool {
        false
    }
    fn GetOffsetToReturns(&self) -> i32{
        0
    }
}

pub struct Simd128LoadTransformOp {
    pub load_kind: Simd128LoadTransformOpLoadKind,
    pub transform_kind: Simd128LoadTransformOpTransformKind,
}
impl Simd128LoadTransformOp{
    pub enum TransformKind{
        k8Splat,
        k16Splat,
        k32Splat,
        k64Splat,
        k8x8S,
        k8x8U,
        k16x4S,
        k16x4U,
        k32x2S,
        k32x2U,
        k32Zero,
        k64Zero,
    }
    pub enum LoadKind{
        with_trap_handler,
    }
}
pub struct Simd128LoadTransformOpLoadKind{
    pub with_trap_handler: bool,
}

impl OperationTrait for Simd128LoadTransformOp {
    fn Cast<Simd128LoadTransformOp>(&self) -> Simd128LoadTransformOp {
        Simd128LoadTransformOp{
            load_kind: Simd128LoadTransformOpLoadKind{with_trap_handler: false},
            transform_kind: Simd128LoadTransformOp::TransformKind::k8Splat,
        }
    }
}

pub trait InstructionSelector {
    fn AlignmentRequirements() -> MachineOperatorBuilder::AlignmentRequirements;
}

impl InstructionSelector for InstructionSelectorT{
    fn AlignmentRequirements() -> MachineOperatorBuilder::AlignmentRequirements{
        MachineOperatorBuilder::AlignmentRequirements::FullUnalignedAccessSupport()
    }
}

mod test{
    #[test]
    fn dummy_test(){
        assert_eq!(1,1);
    }
}

pub enum FPURoundingMode{
    RNE,
}

pub struct BitcastWord32PairToFloat64Op{
}
impl OperationTrait for BitcastWord32PairToFloat64Op{
    fn Cast<BitcastWord32PairToFloat64Op>(&self) -> BitcastWord32PairToFloat64Op{
        BitcastWord32PairToFloat64Op{}
    }
    fn low_word32(&self) -> OpIndex{
        OpIndex{}
    }
    fn high_word32(&self) -> OpIndex{
        OpIndex{}
    }
}

pub struct Simd128ConstantOp{
    pub value: [u8; 16],
}
impl OperationTrait for Simd128ConstantOp {
    fn Cast<Simd128ConstantOp>(&self) -> Simd128ConstantOp{
        Simd128ConstantOp{value: [0; 16]}
    }
}

pub struct SimdShuffleView{}

impl SimdShuffleView{
    fn input(&self, i: i32) -> OpIndex{
        OpIndex{}
    }
}

pub mod wasm{
    pub mod SimdShuffle{
        pub fn Pack4Lanes(shuffle: &[u8]) -> i32{
            0
        }
    }
}
pub enum AccessModeField{
    kMemoryAccessProtectedMemOutOfBounds
}
impl AccessModeField{
    pub fn encode(mode: AccessModeField) -> i32{
        0
    }
}

pub mod turboshaft{
    pub mod RegisterRepresentation{
        pub enum RegisterRepresentation{
            Word32,
            Float64,
        }
    }
    pub struct ChangeOp{
        pub kind: ChangeOpKind,
        pub from: RegisterRepresentation::RegisterRepresentation,
        pub to: RegisterRepresentation::RegisterRepresentation,
    }
    pub enum ChangeOpKind{
        kSignedToFloat,
    }
    impl super::OperationTrait for ChangeOp{
        fn Cast<ChangeOp>(&self) -> ChangeOp {
            ChangeOp{
                kind: ChangeOpKind::kSignedToFloat,
                from: RegisterRepresentation::RegisterRepresentation::Word32,
                to: RegisterRepresentation::RegisterRepresentation::Float64,
            }
        }
        fn Is<ChangeOp>() -> bool{
            false
        }
    }
}

fn VisitRR(selector: &mut InstructionSelectorT, opcode: InstructionCode, node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(opcode, g.DefineAsRegister(node), g.UseRegister(selector.input_at(node, 0)));
}

fn VisitRRArchOpcode(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node), g.UseRegister(selector.input_at(node, 0)));
}

static fn VisitRRI(selector: &mut InstructionSelectorT, opcode: ArchOpcode, node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    let op = selector.Get(node);
    let imm = op.Cast::<Simd128ExtractLaneOp>().lane;
    selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node), g.UseRegister(op.input(0)),
                 g.UseImmediate(imm));
}

static fn VisitSimdShift(selector: &mut InstructionSelectorT, opcode: ArchOpcode,
                           node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    let rhs = selector.input_at(node, 1);
    if selector.Get(rhs).TryCast::<ConstantOp>().is_some() {
        selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node),
                       g.UseRegister(selector.input_at(node, 0)),
                       g.UseImmediate(selector.input_at(node, 1)));
    } else {
        selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node),
                       g.UseRegister(selector.input_at(node, 0)),
                       g.UseRegister(selector.input_at(node, 1)));
    }
}

static fn VisitRRIR(selector: &mut InstructionSelectorT, opcode: ArchOpcode,
                      node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    let op = selector.Get(node).Cast::<Simd128ReplaceLaneOp>();
    selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node), g.UseRegister(op.input(0)),
                 g.UseImmediate(op.lane), g.UseUniqueRegister(op.input(1)));
}

fn VisitRRR(selector: &mut InstructionSelectorT, opcode: InstructionCode,
              node: OpIndex,
              kind: i32 /*OperandGeneratorT::RegisterUseKind*/ = 0 //OperandGeneratorT::RegisterUseKind::kUseRegister)
            ) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(opcode, g.DefineAsRegister(node),
                 g.UseRegister(selector.input_at(node, 0)),
                 g.UseRegister(selector.input_at(node, 1)));
}

static fn VisitUniqueRRR(selector: &mut InstructionSelectorT, opcode: ArchOpcode,
                           node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node),
                 g.UseUniqueRegister(selector.input_at(node, 0)),
                 g.UseUniqueRegister(selector.input_at(node, 1)));
}

fn VisitRRRR(selector: &mut InstructionSelectorT, opcode: ArchOpcode,
               node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(InstructionCode::from(0), g.DefineSameAsFirst(node),
                 g.UseRegister(selector.input_at(node, 0)),
                 g.UseRegister(selector.input_at(node, 1)),
                 g.UseRegister(selector.input_at(node, 2)));
}

static fn VisitRRO(selector: &mut InstructionSelectorT, opcode: ArchOpcode,
                     node: OpIndex) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    selector.Emit(InstructionCode::from(0), g.DefineAsRegister(node),
                 g.UseRegister(selector.input_at(node, 0)),
                 g.UseOperand(selector.input_at(node, 1), InstructionCode::from(0)));
}

fn TryMatchImmediate(selector: &mut InstructionSelectorT,
                       opcode_return: &mut InstructionCode, node: OpIndex,
                       input_count_return: &mut usize, inputs: &mut [InstructionOperand]) -> bool {
    let mut g = RiscvOperandGeneratorT::new(selector);
    if g.CanBeImmediate(node, *opcode_return) {
        //*opcode_return |= AddressingModeField::encode(kMode_MRI);
        inputs[0] = g.UseImmediate(node);
        *input_count_return = 1;
        return true;
    }
    return false;
}

// Shared routine for multiple binary operations.
//template <typename Matcher>
static fn VisitBinop(selector: &mut InstructionSelectorT, node: OpIndex,
                       opcode: InstructionCode, has_reverse_opcode: bool,
                       reverse_opcode: InstructionCode,
                       cont: &FlagsContinuationT) {
    let mut g = RiscvOperandGeneratorT::new(selector);
    let mut inputs: [InstructionOperand; 2] = [InstructionOperand {}, InstructionOperand {}];
    let mut input_count: usize = 0;
    let mut outputs: [InstructionOperand; 1] = [InstructionOperand {}];
    let mut output_count: usize = 0;

    let binop = selector.Get(node);
    let left_node = binop.input(0);
    let right_node = binop.input(1);

    if TryMatchImmediate(selector, &mut opcode, right_node, &mut input_count,
                          &mut inputs[1]) {
        inputs[0] = g.UseRegisterOrImmediateZero(left_node);
        input_count += 1;
    } else if has_reverse_opcode &&
               TryMatchImmediate(selector, &mut reverse_opcode, left_node,
                                 &mut input_count, &mut inputs[1]) {
        inputs[0] = g.UseRegisterOrImmediateZero(right_node);
        opcode = reverse_opcode;
        input_count += 1;
    } else {
        inputs[input_count] = g.UseRegister(left_node);
        input_count += 1;
        inputs[input_count] = g.UseOperand(right_node, opcode);
        input_count += 1;
    }

    if cont.IsDeoptimize() {
        // If we can deoptimize as a result of the binop, we need to make sure that
        // the deopt inputs are not overwritten by the binop result. One way
        // to achieve that is to declare the output register as same-as-first.
        outputs[output_count] = g.DefineSameAsFirst(node);
        output_count += 1;
    } else {
        outputs[output_count] = g.DefineAsRegister(node);
        output_count += 1;
    }

    //DCHECK_NE(0u, input_count);
    //DCHECK_EQ(1u, output_count);
    //DCHECK_GE(arraysize(inputs), input_count);
    //DCHECK_GE(arraysize(outputs), output_count);

    selector.EmitWithContinuation(opcode, output_count, &outputs, input_count,
                                   &inputs, cont);
}

//template <typename Matcher>
static fn VisitBinopSimple(selector: &mut InstructionSelectorT, node: OpIndex,
                       opcode: InstructionCode, has_reverse_opcode: bool,
                       reverse_opcode: InstructionCode) {
    let cont = FlagsContinuationT {};
    VisitBinop(selector, node, opcode, has_reverse_opcode,
                      reverse_opcode, &cont);
}

//template <typename Matcher>
static fn VisitBinopContinuation(selector: &mut InstructionSelectorT, node: OpIndex,
                       opcode: InstructionCode, cont: &FlagsContinuationT) {
    VisitBinop(selector, node, opcode, false, InstructionCode::from(0), cont);
}

//template <typename Matcher>
static fn VisitBinopSimple2(selector: &mut InstructionSelectorT, node: OpIndex,
                       opcode: InstructionCode) {
    VisitBinopSimple(selector, node, opcode, false, InstructionCode::from(0));
}

impl InstructionSelectorT {
    fn VisitStackSlot(&mut self, node: OpIndex) {
        let stack_slot = self.Get(node).Cast::<StackSlotOp>();
        let slot = self.frame().AllocateSpillSlot(stack_slot.size, stack_slot.alignment,
                                            stack_slot.is_tagged);
        let mut g = RiscvOperandGeneratorT::new(self);

        self.Emit(InstructionCode::from(0), g.DefineAsRegister(node),
            self.sequence().AddImmediate(ConstantOp{
                kind: ConstantOpKind::kInteger,
                integral: slot as i64,
            }), InstructionOperand{}, InstructionOperand{});
    }

    fn VisitAbortCSADcheck(&mut self, node: OpIndex) {
        let mut g = RiscvOperandGeneratorT::new(self);
        self.Emit(InstructionCode::from(0), g.NoOutput(),
            g.UseFixed(self.input_at(node, 0), a0));
    }

    fn VisitLoadTransform(&mut self, node: OpIndex) {
        let op = self.Get(node).Cast::<Simd128LoadTransformOp>();
        let is_protected = (op.load_kind.with_trap_handler);
        let mut opcode = InstructionCode::from(0);
        match op.transform_kind {
            Simd128LoadTransformOp::TransformKind::k8Splat => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E8, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k16Splat => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E16, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k32Splat => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E32, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k64Splat => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E64, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k8x8S => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E16, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k8x8U => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E16, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k16x4S => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E32, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k16x4U => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E32, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k32x2S => {
                opcode = InstructionCode::from(0);
                if is_protected {
                    //opcode |= AccessModeField::encode(kMemoryAccessProtectedMemOutOfBounds);
                }
                self.EmitS128Load(node, opcode, VSew::E64, Vlmul::m1);
            }
            Simd128LoadTransformOp::TransformKind::k32x2U => {


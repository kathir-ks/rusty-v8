// src/maglev/s390/maglev-ir-s390.rs

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::base::logging;  // Assuming logging is handled differently in Rust
//use crate::codegen::s390::assembler_s390::Assembler;  // Assuming an Assembler struct exists
//use crate::codegen::s390::register_s390::Register; // Assuming a Register enum/struct exists
//use crate::maglev::maglev_assembler::MaglevAssembler; // Assuming a MaglevAssembler struct exists
//use crate::maglev::maglev_graph_processor::ProcessingState; // Assuming a ProcessingState struct exists
//use crate::maglev::maglev_graph::MaglevGraph; // Assuming a MaglevGraph struct exists
//use crate::maglev::maglev_ir::MaglevNode; // Assuming a MaglevNode trait/struct exists
//use crate::maglev::maglev_ir::ValueInput; // Assuming a ValueInput struct exists
//use crate::maglev::maglev_assembler_inl; // Assuming inline assembler functionality
//use crate::maglev::maglev_ir_inl; // Assuming inline IR functionality

//mod assembler_s390;

use std::convert::TryInto;
use std::ops::{BitAnd, BitOr, BitXor};
//use std::marker::PhantomData;

// Assuming some definitions from the V8 codebase
type Register = u16; // Placeholder
type DoubleRegister = u16; // Placeholder
//const kReturnRegister0: Register = 0; // Placeholder
const kDoubleRegZero: DoubleRegister = 0; // Placeholder
const fp: Register = 0; // Placeholder
const sp: Register = 0; // Placeholder
const r0: Register = 0; // Placeholder
const r2: Register = 2; // Placeholder
const r3: Register = 3; // Placeholder
const r4: Register = 4; // Placeholder
const r5: Register = 5; // Placeholder
const r6: Register = 6; // Placeholder
const kContextRegister: Register = 0; //Placeholder
const kMinInt: i32 = std::i32::MIN;
const overflow: u16 = 0; //Placeholder
const ne: u16 = 1; //Placeholder
const eq: u16 = 2; //Placeholder
const lt: u16 = 3; //Placeholder
const le: u16 = 4; //Placeholder
const ge: u16 = 5; //Placeholder

struct Operand {
    value: i64,
}

impl Operand {
    fn new(value: i64) -> Self {
        Operand { value }
    }
    fn Zero() -> Operand {
        Operand { value: 0 }
    }
}

struct MemOperand {
    reg: Register,
    offset: i64,
}

impl MemOperand {
    fn new(reg: Register, offset: i64) -> Self {
        MemOperand { reg, offset }
    }
}

struct FieldMemOperand {
    reg: Register,
    offset: i64,
}

impl FieldMemOperand {
    fn new(reg: Register, offset: i64) -> Self {
        FieldMemOperand { reg, offset }
    }
}

mod v8_flags {
    pub const debug_code: bool = true;
}

mod compiler {
    pub fn ExternalArrayElementSize(element_type_: i32) -> i32 {
        1 //Placeholder
    }
}

mod standard_frame_constants {
    pub const kArgCOffset: i64 = 0; // Placeholder
    pub const kFunctionOffset: i64 = 8; // Placeholder
}
mod runtime {
    pub const kBytecodeBudgetInterruptWithStackCheck_Maglev: i32 = 0; // Placeholder
    pub const kBytecodeBudgetInterrupt_Maglev: i32 = 1; // Placeholder
}

mod abort_reason {
    pub const kUnexpectedValue: i32 = 0; // Placeholder
}

mod deoptimize_reason {
    pub const kOverflow: i32 = 0; // Placeholder
    pub const kNotInt32: i32 = 1; // Placeholder
    pub const kDivisionByZero: i32 = 2; // Placeholder
    pub const kOutOfBounds: i32 = 3; // Placeholder
}

mod stack_limit_kind {
    pub const kInterruptStackLimit: i32 = 0; // Placeholder
}

mod feedback_cell {
    pub const kInterruptBudgetOffset: i64 = 0; // Placeholder
}

mod string {
    pub const kMaxOneByteCharCode: i32 = 256;
}

mod seq_two_byte_string {
  pub const OFFSET_OF_DATA_START: i64 = 0;
}

mod js_typed_array {
  pub const kRawByteLengthOffset: i64 = 0;
}

mod js_data_view {
  pub const kRawByteLengthOffset: i64 = 0;
}

mod cpu_features {
  pub fn IsSupported(feature: i32) -> bool {
    true
  }
  pub const MISC_INSTR_EXT2: i32 = 0;
}

mod stack_frame {
  pub enum StackFrame {
    MANUAL,
    MAGLEV
  }
}

mod elements_kind {
    pub fn ElementsKindToShiftSize(elements_kind: i32) -> i32 {
        0
    }
}

// Mock MaglevAssembler and related structs/enums
#[allow(dead_code)]
#[allow(unused_variables)]
mod maglev {
    use super::*;
    use std::cell::RefCell;

    pub struct MaglevAssembler<'a> {
        compilation_info: &'a CompilationInfo<'a>,
        native_context: NativeContext,
        label_id: RefCell<u32>,
        temporary_register_scope: Option<TemporaryRegisterScope<'a>>
    }

    impl <'a> MaglevAssembler<'a>{
      pub fn new(compilation_info: &'a CompilationInfo<'a>, native_context: NativeContext) -> MaglevAssembler<'a> {
          MaglevAssembler{
            compilation_info,
            native_context,
            label_id: RefCell::new(0),
            temporary_register_scope: None
          }
      }

      fn get_next_label_id(&self) -> u32 {
        let mut id = self.label_id.borrow_mut();
        *id += 1;
        *id
      }

      pub fn TemporaryRegisterScope(&mut self, assembler: *mut MaglevAssembler) -> TemporaryRegisterScope<'a> {
          let scope = TemporaryRegisterScope {
              assembler: unsafe{&mut *assembler},
              acquired_scratch: Vec::new(),
              acquired_scratch_double: Vec::new()
          };
          self.temporary_register_scope = Some(scope);

          self.temporary_register_scope.take().unwrap()
      }

      pub fn native_context(&self) -> &NativeContext {
          &self.native_context
      }

      pub fn compilation_info(&self) -> &CompilationInfo<'a> {
          &self.compilation_info
      }

        fn LoadSingleCharacterString(&mut self, result_string: Register, char_code: i32) {}
        fn AllocateTwoByteString(&mut self, register_snapshot: u32, result_string: Register, len: i32) {}
        fn Move(&mut self, dest: Register, src: i32) {}
        fn StringFromCharCode(&mut self, register_snapshot: u32, p1: *const (), result_string: Register, code_input: Register, scratch: Register, kMustApplyMask: CharCodeMaskMode) {}
        fn GetDeoptLabel(&mut self, node: &dyn MaglevNode, reason: i32) -> Label {
            Label { id: self.get_next_label_id()}
        }
        fn JumpToDeferredIf(&mut self, condition: u16, f: fn(&mut MaglevAssembler, ZoneLabelRef,Register, Register, &dyn MaglevNode), done: ZoneLabelRef, r1: Register, r2: Register, node: &dyn MaglevNode) {}
        fn JumpToDeferredIf2(&mut self, condition: u16, f: fn(&mut MaglevAssembler, ZoneLabelRef,Register, Register, Register, &dyn MaglevNode), done: ZoneLabelRef, r1: Register, r2: Register, r3: Register, node: &dyn MaglevNode) {}

        fn EmitEagerDeoptIf(&mut self, condition: u16, reason: i32, node: &dyn MaglevNode) {}
        fn EmitEagerDeopt(&mut self, node: &dyn MaglevNode, reason: i32) {}
        fn LoadStackLimit(&mut self, stack_limit: Register, interrupt_stack_limit: i32) {}
        fn CallRuntime(&mut self, runtime_id: i32, args_count: i32) {}
        fn Jump(&mut self, label: Label) {}
        fn LoadBoundedSizeFromObject(&mut self, byte_length: Register, object: Register, raw_byte_length_offset: i64) {}
        fn JumpToDeopt(&mut self, deopt: Label) {}
        fn Push(&mut self, r2: Register, r3: Register, r4: Register, r5: Register) {}
        fn Pop(&mut self, r2: Register, r3: Register, r4: Register, r5: Register) {}
        fn PrepareCallCFunction(&mut self, p1: i32, p2: i32) {}
        fn CallCFunction(&mut self, ieee754_pow_function: ExternalReference, p1: i32, p2: i32) {}
        fn AssertObjectType(&mut self, object: Register, js_typed_array_type: i32, kUnexpectedValue: i32) {}

        fn LeaveFrame(&mut self, maglev: stack_frame::StackFrame) {}
        fn DropArguments(&mut self, actual_params_size: Register) {}

        fn NearestIntF64(&mut self, out: DoubleRegister, input: DoubleRegister) {}
        fn SubF64(&mut self, out: DoubleRegister, left: DoubleRegister, right: DoubleRegister) {}
        fn MoveDouble(&mut self, temp: DoubleRegister, input: DoubleRegister) {}
        fn AddF64(&mut self, out: DoubleRegister, left: DoubleRegister, right: DoubleRegister) {}
        fn CmpF64(&mut self, temp: DoubleRegister, temp2: DoubleRegister) {}
        fn CeilF64(&mut self, out: DoubleRegister, input: DoubleRegister) {}
        fn FloorF64(&mut self, out: DoubleRegister, input: DoubleRegister) {}

        // S390 instructions (placeholder implementations)
        fn AddS32(&mut self, dest: Register, src1: Register, src2: Operand) {}
        fn SubS32(&mut self, dest: Register, src1: Register, src2: Operand) {}
        fn SubS64(&mut self, dest: Register, src1: Operand) {}
        fn MulS32(&mut self, dest: Register, src1: Register, src2: Register) {}
        fn MulS64(&mut self, dest: Register, src: Register) {}
        fn DivS32(&mut self, dest: Register, src1: Register, src2: Register) {}
        fn ModU32(&mut self, dest: Register, src1: Register, src2: Register) {}
        fn ShiftLeftU32(&mut self, dest: Register, src: Register, shift: Operand) {}
        fn ShiftRightS32(&mut self, dest: Register, src: Register, shift: Operand) {}
        fn ShiftRightU32(&mut self, dest: Register, src: Register, shift: Operand) {}

        fn CmpS32(&mut self, reg: Register, operand: Operand) {}
        fn CmpS64(&mut self, reg: Register, operand: Operand) {}
        fn CmpU64(&mut self, reg: Register, operand: Register) {}

        fn LoadS32(&mut self, dest: Register, src: Register) {}
        fn LoadU32(&mut self, dest: Register, field: FieldMemOperand) {}
        fn LoadU64(&mut self, dest: Register, mem: MemOperand) {}

        fn StoreU16(&mut self, src: Register, field: FieldMemOperand) {}
        fn StoreU32(&mut self, src: Register, field: FieldMemOperand) {}

        fn And(&mut self, dest: Register, src1: Register, src2: Operand) {}
        fn Or(&mut self, dest: Register, src1: Register, src2: Register) {}
        fn Xor(&mut self, dest: Register, src1: Register, src2: Register) {}
        fn Not32(&mut self, dest: Register, src: Register) {}

        fn lcr(&mut self, dest: Register, src: Register) {}
        fn lpr(&mut self, dest: Register, dest2: Register) {}
        fn lgfr(&mut self, dest: Register, src: Register) {}
        fn lay(&mut self, dest: Register, mem: MemOperand) {}
        fn UncheckedSmiTagInt32(&mut self, length: Register) {}
        fn lcdbr(&mut self, out: DoubleRegister, value: DoubleRegister) {}
        fn lpdbr(&mut self, out: DoubleRegister, input: DoubleRegister) {}
        fn lzdr(&mut self, kDoubleRegZero: DoubleRegister) {}

        fn Ret(&mut self) {}
        fn bge(&mut self, done: &Label) {}
        fn bgt(&mut self, next: &Label) {}
        fn blt(&mut self, deopt: &Label) {}
        fn bne(&mut self, end: &Label) {}
        fn bne_2(&mut self, done: &Label) {}
        fn beq(&mut self, deopt: &Label) {}
        fn b(&mut self, done: &Label) {}
        fn ShiftRightU64(&mut self, result_register: Register, result_register2: Register, operand: Operand) {}
    }

    pub struct Label {
      id: u32,
    }

    #[derive(Clone, Copy)]
    pub struct ZoneLabelRef<'a> {
        assembler: *mut MaglevAssembler<'a>,
    }

    impl <'a> ZoneLabelRef<'a> {
      pub fn new(assembler: *mut MaglevAssembler<'a>) -> ZoneLabelRef<'a>{
        ZoneLabelRef{
          assembler
        }
      }
    }

    pub enum CharCodeMaskMode {
        kMustApplyMask,
    }

    pub struct TemporaryRegisterScope<'a> {
      assembler: &'a mut MaglevAssembler<'a>,
      acquired_scratch: Vec<Register>,
      acquired_scratch_double: Vec<DoubleRegister>
    }

    impl <'a> TemporaryRegisterScope<'a> {
      pub fn AcquireScratch(&mut self) -> Register {
        let scratch = self.get_free_register();
        self.acquired_scratch.push(scratch);
        scratch
      }

      pub fn AcquireScratchDouble(&mut self) -> DoubleRegister {
        let scratch = self.get_free_double_register();
        self.acquired_scratch_double.push(scratch);
        scratch
      }

      fn get_free_register(&self) -> Register {
        0 //Placeholder
      }

      fn get_free_double_register(&self) -> DoubleRegister {
        0 //Placeholder
      }

      pub fn IncludeScratch(&mut self, mask: Register) {}
    }

    pub struct SaveRegisterStateForCall<'a> {
      assembler: &'a mut MaglevAssembler<'a>,
      register_snapshot: u32,
      lazy_deopt_info: Option<u32>
    }

    impl <'a> SaveRegisterStateForCall<'a> {
      pub fn new(assembler: &'a mut MaglevAssembler<'a>, register_snapshot: u32) -> SaveRegisterStateForCall<'a> {
        SaveRegisterStateForCall{
          assembler,
          register_snapshot,
          lazy_deopt_info: None
        }
      }
      pub fn DefineSafepointWithLazyDeopt(&mut self, lazy_deopt_info: u32) {}
      pub fn DefineSafepoint(&mut self) {}
    }

    pub trait MaglevNode {
      fn value_input(&self) -> ValueInput;
      fn result(&self) -> ValueInput;
      fn eager_deopt_info(&self) -> u32;
      fn lazy_deopt_info(&self) -> u32;
      fn register_snapshot(&self) -> u32;
    }

    pub struct ValueInput {
        node: Box<dyn MaglevNode>
    }

    impl ValueInput {
        fn node(&self) -> &dyn MaglevNode {
            self.node.as_ref()
        }
    }

    pub trait ProcessingState {}
}

struct CompilationInfo<'a> {
    toplevel_compilation_unit: &'a TopLevelCompilationUnit,
}
impl <'a> CompilationInfo<'a>{
    pub fn toplevel_compilation_unit(&self) -> &TopLevelCompilationUnit {
        &self.toplevel_compilation_unit
    }
}

struct TopLevelCompilationUnit {
    parameter_count: i32,
}

impl TopLevelCompilationUnit {
    fn parameter_count(&self) -> i32 {
        self.parameter_count
    }
}

#[allow(dead_code)]
struct NativeContext {
    object: u32,
}

#[allow(dead_code)]
struct ExternalReference {}

impl ExternalReference {
    fn mod_two_doubles_operation() -> ExternalReference {
        ExternalReference {}
    }
    fn ieee754_pow_function() -> ExternalReference {
        ExternalReference {}
    }
}

// Mock FrameScope
#[allow(dead_code)]
struct FrameScope<'a> {
    masm: &'a mut maglev::MaglevAssembler<'a>,
    frame_type: stack_frame::StackFrame,
}

impl <'a> FrameScope<'a> {
    fn new(masm: &'a mut maglev::MaglevAssembler<'a>, frame_type: stack_frame::StackFrame) -> Self {
        FrameScope { masm, frame_type }
    }
}

// Trait for Maglev Nodes
trait MaglevNode {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {}
    fn value_input(&self) -> ValueInput;
    fn code_input(&self) -> ValueInput;
    fn left_input(&self) -> ValueInput;
    fn right_input(&self) -> ValueInput;
    fn input(&self) -> ValueInput;
    fn receiver_input(&self) -> ValueInput;
    fn index_input(&self) -> ValueInput;
    fn result(&self) -> ValueInput;
    fn offset(&self) -> i32;
    fn allocation_block_input(&self) -> ValueInput;
    fn formal_parameter_count(&self) -> i32;
    fn feedback_cell(&self) -> ValueInput;
    fn amount(&self) -> i32;
    fn eager_deopt_info(&self) -> u32;
    fn lazy_deopt_info(&self) -> u32;
    fn register_snapshot(&self) -> u32;
    fn try_cast<T: 'static>(&self) -> Option<&T> {
      None
    }
}

// Placeholder Implementations for Maglev Nodes
#[allow(dead_code)]
#[allow(unused_variables)]
struct Int32NegateWithOverflow {
    value: ValueInput,
    result: ValueInput,
    eager_deopt_info: u32
}

impl Int32NegateWithOverflow {
    fn new(value: ValueInput, result: ValueInput, eager_deopt_info: u32) -> Self {
        Int32NegateWithOverflow { value, result, eager_deopt_info }
    }
}

impl MaglevNode for Int32NegateWithOverflow {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {
        let value = 0; //ToRegister(self.value_input());
        let out = 0; //ToRegister(self.result());

        // Deopt when result would be -0.
        masm.CmpS32(value, Operand::new(0));
        masm.EmitEagerDeoptIf(eq, deoptimize_reason::kOverflow, self);

        masm.lcr(out, value);
        masm.LoadS32(out, out);

        // Output register must not be a register input into the eager deopt info.
        //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
        masm.EmitEagerDeoptIf(overflow, deoptimize_reason::kOverflow, self);
    }
    fn value_input(&self) -> ValueInput {
      self.value.clone()
    }
    fn code_input(&self) -> ValueInput {
        todo!()
    }
    fn left_input(&self) -> ValueInput {
        todo!()
    }
    fn right_input(&self) -> ValueInput {
        todo!()
    }
    fn input(&self) -> ValueInput {
        todo!()
    }
    fn receiver_input(&self) -> ValueInput {
        todo!()
    }
    fn index_input(&self) -> ValueInput {
        todo!()
    }
    fn result(&self) -> ValueInput {
      self.result.clone()
    }
    fn offset(&self) -> i32 {
        todo!()
    }
    fn allocation_block_input(&self) -> ValueInput {
        todo!()
    }
    fn formal_parameter_count(&self) -> i32 {
        todo!()
    }
    fn feedback_cell(&self) -> ValueInput {
        todo!()
    }
    fn amount(&self) -> i32 {
        todo!()
    }
    fn eager_deopt_info(&self) -> u32 {
      self.eager_deopt_info
    }
    fn lazy_deopt_info(&self) -> u32 {
        todo!()
    }
    fn register_snapshot(&self) -> u32 {
        todo!()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct Int32AbsWithOverflow {
    result: ValueInput,
    input: ValueInput,
    eager_deopt_info: u32,
}

impl Int32AbsWithOverflow {
    fn new(result: ValueInput, input: ValueInput, eager_deopt_info: u32) -> Self {
        Int32AbsWithOverflow { result, input, eager_deopt_info }
    }
}

impl MaglevNode for Int32AbsWithOverflow {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {
        let out = 0; //ToRegister(self.result());
        masm.lpr(out, out);
        // Output register must not be a register input into the eager deopt info.
        //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
        masm.EmitEagerDeoptIf(overflow, deoptimize_reason::kOverflow, self);
        masm.lgfr(out, out);
    }
    fn value_input(&self) -> ValueInput {
        todo!()
    }
    fn code_input(&self) -> ValueInput {
        todo!()
    }
    fn left_input(&self) -> ValueInput {
        todo!()
    }
    fn right_input(&self) -> ValueInput {
        todo!()
    }
    fn input(&self) -> ValueInput {
      self.input.clone()
    }
    fn receiver_input(&self) -> ValueInput {
        todo!()
    }
    fn index_input(&self) -> ValueInput {
        todo!()
    }
    fn result(&self) -> ValueInput {
      self.result.clone()
    }
    fn offset(&self) -> i32 {
        todo!()
    }
    fn allocation_block_input(&self) -> ValueInput {
        todo!()
    }
    fn formal_parameter_count(&self) -> i32 {
        todo!()
    }
    fn feedback_cell(&self) -> ValueInput {
        todo!()
    }
    fn amount(&self) -> i32 {
        todo!()
    }
    fn eager_deopt_info(&self) -> u32 {
      self.eager_deopt_info
    }
    fn lazy_deopt_info(&self) -> u32 {
        todo!()
    }
    fn register_snapshot(&self) -> u32 {
        todo!()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct Int32IncrementWithOverflow {
    value: ValueInput,
    result: ValueInput,
    eager_deopt_info: u32,
}

impl Int32IncrementWithOverflow {
    fn new(value: ValueInput, result: ValueInput, eager_deopt_info: u32) -> Self {
        Int32IncrementWithOverflow { value, result, eager_deopt_info }
    }
}

impl MaglevNode for Int32IncrementWithOverflow {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {
        let value = 0; //ToRegister(self.value_input());
        let out = 0; //ToRegister(self.result());
        masm.AddS32(out, value, Operand::new(1));
        masm.LoadS32(out, out);

        // Output register must not be a register input into the eager deopt info.
        //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
        masm.EmitEagerDeoptIf(overflow, deoptimize_reason::kOverflow, self);
    }
    fn value_input(&self) -> ValueInput {
      self.value.clone()
    }
    fn code_input(&self) -> ValueInput {
        todo!()
    }
    fn left_input(&self) -> ValueInput {
        todo!()
    }
    fn right_input(&self) -> ValueInput {
        todo!()
    }
    fn input(&self) -> ValueInput {
        todo!()
    }
    fn receiver_input(&self) -> ValueInput {
        todo!()
    }
    fn index_input(&self) -> ValueInput {
        todo!()
    }
    fn result(&self) -> ValueInput {
      self.result.clone()
    }
    fn offset(&self) -> i32 {
        todo!()
    }
    fn allocation_block_input(&self) -> ValueInput {
        todo!()
    }
    fn formal_parameter_count(&self) -> i32 {
        todo!()
    }
    fn feedback_cell(&self) -> ValueInput {
        todo!()
    }
    fn amount(&self) -> i32 {
        todo!()
    }
    fn eager_deopt_info(&self) -> u32 {
      self.eager_deopt_info
    }
    fn lazy_deopt_info(&self) -> u32 {
        todo!()
    }
    fn register_snapshot(&self) -> u32 {
        todo!()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct Int32DecrementWithOverflow {
    value: ValueInput,
    result: ValueInput,
    eager_deopt_info: u32,
}

impl Int32DecrementWithOverflow {
    fn new(value: ValueInput, result: ValueInput, eager_deopt_info: u32) -> Self {
        Int32DecrementWithOverflow { value, result, eager_deopt_info }
    }
}

impl MaglevNode for Int32DecrementWithOverflow {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {
        let value = 0; //ToRegister(self.value_input());
        let out = 0; //ToRegister(self.result());
        masm.AddS32(out, value, Operand::new(-1));
        masm.LoadS32(out, out);

        // Output register must not be a register input into the eager deopt info.
        //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
        masm.EmitEagerDeoptIf(overflow, deoptimize_reason::kOverflow, self);
    }
    fn value_input(&self) -> ValueInput {
      self.value.clone()
    }
    fn code_input(&self) -> ValueInput {
        todo!()
    }
    fn left_input(&self) -> ValueInput {
        todo!()
    }
    fn right_input(&self) -> ValueInput {
        todo!()
    }
    fn input(&self) -> ValueInput {
        todo!()
    }
    fn receiver_input(&self) -> ValueInput {
        todo!()
    }
    fn index_input(&self) -> ValueInput {
        todo!()
    }
    fn result(&self) -> ValueInput {
      self.result.clone()
    }
    fn offset(&self) -> i32 {
        todo!()
    }
    fn allocation_block_input(&self) -> ValueInput {
        todo!()
    }
    fn formal_parameter_count(&self) -> i32 {
        todo!()
    }
    fn feedback_cell(&self) -> ValueInput {
        todo!()
    }
    fn amount(&self) -> i32 {
        todo!()
    }
    fn eager_deopt_info(&self) -> u32 {
      self.eager_deopt_info
    }
    fn lazy_deopt_info(&self) -> u32 {
        todo!()
    }
    fn register_snapshot(&self) -> u32 {
        todo!()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct BuiltinStringFromCharCode {
    code_input: ValueInput,
    result: ValueInput,
    temporaries_needed: i32,
}

impl BuiltinStringFromCharCode {
    fn new(code_input: ValueInput, result: ValueInput) -> Self {
        BuiltinStringFromCharCode {
            code_input,
            result,
            temporaries_needed: 0,
        }
    }

    fn MaxCallStackArgs() -> i32 {
        0
    }
}

impl MaglevNode for BuiltinStringFromCharCode {
    fn set_value_location_constraints(&mut self) {}
    fn generate_code(&self, masm: &mut maglev::MaglevAssembler, state: &dyn maglev::ProcessingState) {
        let mut temps = masm.TemporaryRegisterScope(masm);
        let scratch = temps.AcquireScratch();
        let result_string = 0; //ToRegister(self.result());

        if let Some(constant) = self.code_input().node().try_cast::<Int32Constant>() {
            let char_code = (constant.value() & 0xFFFF) as i32;
            if 0 <= char_code && char_code < string::kMaxOneByteCharCode {
                masm.LoadSingleCharacterString(result_string, char_code);
            } else {
                // Ensure that {result_string} never aliases {scratch}, otherwise the
                // store will fail.
                let reallocate_result = scratch == result_string;
                let result_string = if reallocate_result {
                    temps.AcquireScratch()
                } else {
                    result_string
                };
                assert_ne!(scratch, result_string);

                masm.AllocateTwoByteString(self.register_snapshot(), result_string, 1);
                masm.Move(scratch, char_code);
                masm.StoreU16(scratch, FieldMemOperand::new(result_string, seq_two_byte_string::OFFSET_OF_DATA_START));

                if reallocate_result {
                    masm.Move(0, result_string); //ToRegister(self.result())
                }
            }
        } else {
            masm.StringFromCharCode(
                self.register_snapshot(),
                std::ptr::null(),
                result_string,
                0, //ToRegister(self.code_input()),
                scratch,
                maglev::CharCodeMaskMode::kMustApplyMask,
            );
        }
    }
    fn value_input(&self) -> ValueInput {
        todo!()
    }
    fn code_input(&self) -> ValueInput {
      self.code_input.clone()
    }
    fn left_input(&self) -> ValueInput {
        todo!()
    }
    fn right_input(&self) -> ValueInput {
        todo!()
    }
    fn input(&self) ->
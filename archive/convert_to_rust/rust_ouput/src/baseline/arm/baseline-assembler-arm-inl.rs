// Converted from V8 C++ source files:
// Header: baseline-assembler-arm-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};

// use crate::v8::internal::wasm::std::SharedRef;

use crate::Register;
use crate::UseScratchRegisterScope;
use crate::v8::internal::v8;

pub struct BaselineAssembler {
    masm_: *mut MacroAssembler,
    scratch_register_scope_: *mut ScratchRegisterScope,
}

impl BaselineAssembler {
    pub fn masm(&mut self) -> &mut MacroAssembler {
        unsafe { &mut *self.masm_ }
    }
    pub fn new(masm: *mut MacroAssembler) -> Self {
        Self {
            masm_: masm,
            scratch_register_scope_: std::ptr::null_mut(),
        }
    }
}

pub struct Operand {}

impl Operand {
    pub fn new(value: i32) -> Self {
        Operand {}
    }
    pub fn new_register(reg: Register) -> Self {
        Operand {}
    }
}

pub struct Label {}

impl Label {
    pub fn new() -> Self {
        Label {}
    }
}

pub enum RootIndex {}

pub enum Condition {}

pub struct TaggedIndex {}

impl TaggedIndex {
    pub fn ptr(&self) -> i64 {
        0
    }
}

pub struct ExternalReference {}

pub enum StackFrame {}

impl StackFrame {
    pub const BASELINE: Self = StackFrame {};
}

pub struct FieldMemOperand {}

impl FieldMemOperand {
    pub fn new(base: Register, offset: i32) -> Self {
        FieldMemOperand {}
    }
}

pub struct FeedbackSlot {}

impl FeedbackSlot {
    pub fn ToInt(&self) -> i32 {
        0
    }
}

pub struct Handle<T> {}

pub struct InterpreterRegister {}

impl InterpreterRegister {
    pub fn ToOperand(&self) -> i32 {
        0
    }
}

pub struct RegisterList {}

impl RegisterList {
    pub fn register_count(&self) -> i32 {
        0
    }
}

impl std::ops::Index<i32> for RegisterList {
    type Output = InterpreterRegister;
    fn index(&self, index: i32) -> &Self::Output {
        todo!()
    }
}

pub struct MacroAssembler {}

impl MacroAssembler {
    pub fn Push(&mut self, reg: Register) {}
    pub fn Pop(&mut self, reg: Register) {}
    pub fn LeaveFrame(&mut self, stack_frame: StackFrame) {}
    pub fn DropArguments(&mut self, size: Register) {}
    pub fn Ret(&mut self) {}
    pub fn bind(&mut self, label: *mut Label) {}
    pub fn SmiTag(&mut self, reg: Register) {}
    pub fn SmiUntag(&mut self, reg: Register) {}
    pub fn Assert(&mut self, condition: Condition, reason: AbortReason) {}
    pub fn cmp(&mut self, lhs: Register, rhs: Register) {}
    pub fn mov(&mut self, dest: Register, src: Register, cc: Condition, kind: i32) {}
    pub fn Switch(
        _reg1: Register,
        reg: Register,
        case_value_base: i32,
        labels: &mut [*mut Label],
        num_labels: i32,
    ) {
    }
}

pub enum AbortReason {}

pub struct BaselineFrameConstants {}

impl BaselineFrameConstants {
    pub const kFeedbackVectorFromFp: i32 = 0;
    pub const kFeedbackCellFromFp: i32 = 0;
}

pub enum SaveFPRegsMode {}

impl SaveFPRegsMode {
    pub const kIgnore: Self = SaveFPRegsMode {};
}

pub enum TypeofMode {}

pub enum Tagged {}

impl Tagged {
    pub fn ptr(&self) -> i64 {
        0
    }
}

impl Tagged {
    pub fn from_int(value: i32) -> Self {
        Tagged {}
    }
}

pub struct Smi {}

impl Smi {
    pub fn FromInt(value: i32) -> Self {
        Smi {}
    }
}

pub enum InstanceType {}

pub struct Context {}

impl Context {
    pub const kPreviousOffset: i32 = 0;
    pub fn OffsetOfElementAt(index: u32) -> i32 {
        0
    }
    pub const kExtensionOffset: i32 = 0;
}

pub struct SourceTextModule {}

impl SourceTextModule {
    pub const kRegularExportsOffset: i32 = 0;
    pub const kRegularImportsOffset: i32 = 0;
}

pub struct Cell {}

impl Cell {
    pub const kValueOffset: i32 = 0;
}

pub struct FeedbackVector {}

impl FeedbackVector {
    pub fn OffsetOfElementAt(index: i32) -> i32 {
        0
    }
}

pub struct CodeWrapper {}

impl CodeWrapper {
    pub const kCodeOffset: i32 = 0;
}

pub struct FeedbackCell {}

impl FeedbackCell {
    pub const kInterruptBudgetOffset: i32 = 0;
}

pub struct StandardFrameConstants {}

impl StandardFrameConstants {
    pub const kArgCOffset: i32 = 0;
}

pub struct Runtime {}

impl Runtime {
    pub const kBytecodeBudgetInterrupt_Sparkplug: Self = Runtime {};
}

pub struct BaselineLeaveFrameDescriptor {}

impl BaselineLeaveFrameDescriptor {
    pub fn WeightRegister() -> Register {
        Register {}
    }
    pub fn ParamsSizeRegister() -> Register {
        Register {}
    }
}

pub enum CompressionMode {}

const kSystemPointerSize: i32 = 4;

const kInterpreterBytecodeOffsetRegister: Register = Register {};
const kInterpreterAccumulatorRegister: Register = Register {};
const kContextRegister: Register = Register {};
const kJSFunctionRegister: Register = Register {};

struct v8_flags {
    pub debug_code: bool,
}

static v8_flags: v8_flags = v8_flags { debug_code: true };

impl BaselineAssembler {
    pub struct ScratchRegisterScope {
        assembler_: *mut BaselineAssembler,
        prev_scope_: *mut BaselineAssembler::ScratchRegisterScope,
        wrapped_scope_: UseScratchRegisterScope,
    }

    impl ScratchRegisterScope {
        pub fn new(assembler: *mut BaselineAssembler) -> Self {
            let mut basm = unsafe { &mut *assembler };
            let prev_scope_ = basm.scratch_register_scope_;
            let mut wrapped_scope_ = UseScratchRegisterScope {};

            if basm.scratch_register_scope_.is_null() {
                wrapped_scope_.Include(Register {});
                wrapped_scope_.Include(Register {});
                wrapped_scope_.Include(kInterpreterBytecodeOffsetRegister);
            }
            basm.scratch_register_scope_ = unsafe { std::mem::transmute(Box::new(Self {
                assembler_: assembler,
                prev_scope_: prev_scope_,
                wrapped_scope_: wrapped_scope_,
            }))};

            unsafe { *std::mem::transmute::<*mut BaselineAssembler::ScratchRegisterScope, &mut Self>(basm.scratch_register_scope_) }
        }

        pub fn AcquireScratch(&mut self) -> Register {
            self.wrapped_scope_.Acquire()
        }
    }
    impl Drop for ScratchRegisterScope {
        fn drop(&mut self) {
            let mut basm = unsafe { &mut *self.assembler_ };
            basm.scratch_register_scope_ = self.prev_scope_;
        }
    }
}

mod detail {
    use super::*;

    #[cfg(debug_assertions)]
    pub fn Clobbers(target: Register, op: MemOperand) -> bool {
        todo!()
    }
}

impl BaselineAssembler {
    pub fn RegisterFrameOperand(&mut self, interpreter_register: InterpreterRegister) -> MemOperand {
        MemOperand {}
    }

    pub fn RegisterFrameAddress(&mut self, interpreter_register: InterpreterRegister, rscratch: Register) {
        let size: i32 = interpreter_register.ToOperand() * kSystemPointerSize;
        self.add(rscratch, Register {}, Operand::new(size));
    }

    pub fn FeedbackVectorOperand(&mut self) -> MemOperand {
        MemOperand {}
    }

    pub fn FeedbackCellOperand(&mut self) -> MemOperand {
        MemOperand {}
    }

    pub fn Bind(&mut self, label: *mut Label) {
        unsafe { (&mut *self.masm_).bind(label) };
    }

    pub fn JumpTarget(&mut self) {}

    pub fn Jump(&mut self, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).b(target) };
    }

    pub fn JumpIfRoot(&mut self, value: Register, index: RootIndex, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).JumpIfRoot(value, index, target) };
    }

    pub fn JumpIfNotRoot(&mut self, value: Register, index: RootIndex, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).JumpIfNotRoot(value, index, target) };
    }

    pub fn JumpIfSmi(&mut self, value: Register, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).JumpIfSmi(value, target) };
    }

    pub fn JumpIfImmediate(
        &mut self,
        cc: Condition,
        left: Register,
        right: i32,
        target: *mut Label,
        distance: Label::Distance,
    ) {
        self.JumpIf(cc, left, Operand::new(right), target, distance);
    }

    pub fn JumpIfNotSmi(&mut self, value: Register, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).JumpIfNotSmi(value, target) };
    }

    pub fn TestAndBranch(&mut self, value: Register, mask: i32, cc: Condition, target: *mut Label, distance: Label::Distance) {
        unsafe {
            (&mut *self.masm_).tst(value, Operand::new(mask));
            (&mut *self.masm_).b(cc, target);
        };
    }

    pub fn JumpIf(&mut self, cc: Condition, lhs: Register, rhs: Operand, target: *mut Label, distance: Label::Distance) {
        unsafe {
            (&mut *self.masm_).cmp(lhs, rhs);
            (&mut *self.masm_).b(cc, target);
        };
    }

    pub fn JumpIfObjectTypeFast(
        &mut self,
        cc: Condition,
        object: Register,
        instance_type: InstanceType,
        target: *mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let scratch = temps.AcquireScratch();
        self.JumpIfObjectType(cc, object, instance_type, scratch, target, distance);
    }

    pub fn JumpIfObjectType(
        &mut self,
        cc: Condition,
        object: Register,
        instance_type: InstanceType,
        map: Register,
        target: *mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let type_ = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).LoadMap(map, object) };
        unsafe { (&mut *self.masm_).ldrh(type_, FieldMemOperand::new(map, Map::kInstanceTypeOffset)) };
        self.JumpIf(cc, type_, Operand::new(instance_type as i32), target, distance);
    }

    pub fn JumpIfInstanceType(
        &mut self,
        cc: Condition,
        map: Register,
        instance_type: InstanceType,
        target: *mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let type_ = temps.AcquireScratch();
        if v8_flags.debug_code {
            unsafe { (&mut *self.masm_).AssertNotSmi(map) };
            unsafe { (&mut *self.masm_).CompareObjectType(map, type_, type_, MAP_TYPE) };
            unsafe { (&mut *self.masm_).Assert(eq, AbortReason::kUnexpectedValue) };
        }
        unsafe { (&mut *self.masm_).ldrh(type_, FieldMemOperand::new(map, Map::kInstanceTypeOffset)) };
        self.JumpIf(cc, type_, Operand::new(instance_type as i32), target, distance);
    }

    pub fn JumpIfPointer(&mut self, cc: Condition, value: Register, operand: MemOperand, target: *mut Label, distance: Label::Distance) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let tmp = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(tmp, operand) };
        self.JumpIf(cc, value, Operand::new_register(tmp), target, distance);
    }

    pub fn JumpIfSmi_smi(&mut self, cc: Condition, value: Register, smi: Tagged<Smi>, target: *mut Label, distance: Label::Distance) {
        unsafe { (&mut *self.masm_).AssertSmi(value) };
        self.JumpIf(cc, value, Operand::new(smi.ptr() as i32), target, distance);
    }

    pub fn JumpIfSmi(&mut self, cc: Condition, lhs: Register, rhs: Register, target: *mut Label, distance: Label::Distance) {
        unsafe {
            (&mut *self.masm_).AssertSmi(lhs);
            (&mut *self.masm_).AssertSmi(rhs);
        };
        self.JumpIf(cc, lhs, Operand::new_register(rhs), target, distance);
    }

    pub fn JumpIfTagged(&mut self, cc: Condition, value: Register, operand: MemOperand, target: *mut Label, distance: Label::Distance) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let tmp = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(tmp, operand) };
        self.JumpIf(cc, value, Operand::new_register(tmp), target, distance);
    }

    pub fn JumpIfTagged_mem(&mut self, cc: Condition, operand: MemOperand, value: Register, target: *mut Label, distance: Label::Distance) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let tmp = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(tmp, operand) };
        self.JumpIf(cc, tmp, Operand::new_register(value), target, distance);
    }

    pub fn JumpIfByte(&mut self, cc: Condition, value: Register, byte: i32, target: *mut Label, distance: Label::Distance) {
        self.JumpIf(cc, value, Operand::new(byte), target, distance);
    }

    pub fn Move(&mut self, output: InterpreterRegister, source: Register) {
        self.Move_mem(self.RegisterFrameOperand(output), source);
    }

    pub fn Move_register(&mut self, output: Register, value: Tagged<TaggedIndex>) {
        unsafe { (&mut *self.masm_).mov(output, Operand::new(value.ptr() as i32)) };
    }

    pub fn Move_mem(&mut self, output: MemOperand, source: Register) {
        unsafe { (&mut *self.masm_).str(source, output) };
    }

    pub fn Move_external(&mut self, output: Register, reference: ExternalReference) {
        unsafe { (&mut *self.masm_).Move32BitImmediate(output, Operand::new(0)) };
    }

    pub fn Move_handle(&mut self, output: Register, value: Handle<HeapObject>) {
        unsafe { (&mut *self.masm_).Move32BitImmediate(output, Operand::new(0)) };
    }

    pub fn Move_int(&mut self, output: Register, value: i32) {
        unsafe { (&mut *self.masm_).mov(output, Operand::new(value)) };
    }

    pub fn MoveMaybeSmi(&mut self, output: Register, source: Register) {
        unsafe { (&mut *self.masm_).mov(output, source) };
    }

    pub fn MoveSmi(&mut self, output: Register, source: Register) {
        unsafe { (&mut *self.masm_).mov(output, source) };
    }

    pub fn Push(&mut self, vals: &[Register]) {}
    pub fn PushReverse(&mut self, vals: &[Register]) {}
    pub fn Pop(&mut self, registers: &[Register]) {}

    pub fn LoadTaggedField(&mut self, output: Register, source: Register, offset: i32) {
        unsafe { (&mut *self.masm_).ldr(output, FieldMemOperand::new(source, offset)) };
    }

    pub fn LoadTaggedSignedField(&mut self, output: Register, source: Register, offset: i32) {
        unsafe { (&mut *self.masm_).ldr(output, FieldMemOperand::new(source, offset)) };
    }

    pub fn LoadTaggedSignedFieldAndUntag(&mut self, output: Register, source: Register, offset: i32) {
        self.LoadTaggedSignedField(output, source, offset);
        self.SmiUntag(output);
    }

    pub fn LoadWord16FieldZeroExtend(&mut self, output: Register, source: Register, offset: i32) {
        unsafe { (&mut *self.masm_).ldrh(output, FieldMemOperand::new(source, offset)) };
    }

    pub fn LoadWord8Field(&mut self, output: Register, source: Register, offset: i32) {
        unsafe { (&mut *self.masm_).ldrb(output, FieldMemOperand::new(source, offset)) };
    }

    pub fn StoreTaggedSignedField(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let tmp = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).mov(tmp, Operand::new(value.ptr() as i32)) };
        unsafe { (&mut *self.masm_).str(tmp, FieldMemOperand::new(target, offset)) };
    }

    pub fn StoreTaggedFieldWithWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
        unsafe {
            (&mut *self.masm_).str(value, FieldMemOperand::new(target, offset));
            (&mut *self.masm_).RecordWriteField(target, offset, value, 0, SaveFPRegsMode::kIgnore);
        };
    }

    pub fn StoreTaggedFieldNoWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
        unsafe { (&mut *self.masm_).str(value, FieldMemOperand::new(target, offset)) };
    }

    pub fn TryLoadOptimizedOsrCode(&mut self, scratch_and_result: Register, feedback_vector: Register, slot: FeedbackSlot, on_result: *mut Label, distance: Label::Distance) {
        let mut fallthrough = Label::new();
        self.LoadTaggedField(scratch_and_result, feedback_vector, FeedbackVector::OffsetOfElementAt(slot.ToInt()));
        unsafe { (&mut *self.masm_).LoadWeakValue(scratch_and_result, scratch_and_result, &mut fallthrough) };

        {
            let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));

            unsafe { (&mut *self.masm_).ldr(scratch_and_result, FieldMemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset)) };

            let scratch = temps.AcquireScratch();
            unsafe { (&mut *self.masm_).TestCodeIsMarkedForDeoptimization(scratch_and_result, scratch) };
            unsafe { (&mut *self.masm_).b(eq, on_result) };
            unsafe { (&mut *self.masm_).mov(scratch, __ClearedValue()) };
            self.StoreTaggedFieldNoWriteBarrier(feedback_vector, FeedbackVector::OffsetOfElementAt(slot.ToInt()), scratch);
        }

        self.Bind(&mut fallthrough);
        self.Move_int(scratch_and_result, 0);
    }

    pub fn AddToInterruptBudgetAndJumpIfNotExceeded(&mut self, weight: i32, skip_interrupt_label: *mut Label) {
        let mut scratch_scope = ScratchRegisterScope::new(std::mem::transmute(self));
        let feedback_cell = scratch_scope.AcquireScratch();
        self.LoadFeedbackCell(feedback_cell);

        let interrupt_budget = scratch_scope.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset)) };
        unsafe { (&mut *self.masm_).add(interrupt_budget, interrupt_budget, Operand::new(weight), 1) };
        unsafe { (&mut *self.masm_).str(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset)) };
        if !skip_interrupt_label.is_null() {
            unsafe { (&mut *self.masm_).b(ge, skip_interrupt_label) };
        }
    }

    pub fn AddToInterruptBudgetAndJumpIfNotExceeded_register(&mut self, weight: Register, skip_interrupt_label: *mut Label) {
        let mut scratch_scope = ScratchRegisterScope::new(std::mem::transmute(self));
        let feedback_cell = scratch_scope.AcquireScratch();
        self.LoadFeedbackCell(feedback_cell);

        let interrupt_budget = scratch_scope.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset)) };
        unsafe { (&mut *self.masm_).add(interrupt_budget, interrupt_budget, weight, 1) };
        unsafe { (&mut *self.masm_).str(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset)) };
        if !skip_interrupt_label.is_null() {
            unsafe { (&mut *self.masm_).b(ge, skip_interrupt_label) };
        }
    }

    pub fn LdaContextSlot(&mut self, context: Register, index: u32, depth: u32, compression_mode: CompressionMode) {
        let mut context_ = context;
        for _ in 0..depth {
            self.LoadTaggedField(context_, context_, Context::kPreviousOffset);
        }
        self.LoadTaggedField(kInterpreterAccumulatorRegister, context_, Context::OffsetOfElementAt(index));
    }

    pub fn StaContextSlot(&mut self, context: Register, value: Register, index: u32, depth: u32) {
        let mut context_ = context;
        for _ in 0..depth {
            self.LoadTaggedField(context_, context_, Context::kPreviousOffset);
        }
        self.StoreTaggedFieldWithWriteBarrier(context_, Context::OffsetOfElementAt(index), value);
    }

    pub fn LdaModuleVariable(&mut self, context: Register, cell_index: i32, depth: u32) {
        let mut context_ = context;
        for _ in 0..depth {
            self.LoadTaggedField(context_, context_, Context::kPreviousOffset);
        }
        self.LoadTaggedField(context_, context_, Context::kExtensionOffset);
        let mut cell_index_ = cell_index;
        if cell_index > 0 {
            self.LoadTaggedField(context_, context_, SourceTextModule::kRegularExportsOffset);
            cell_index_ -= 1;
        } else {
            self.LoadTaggedField(context_, context_, SourceTextModule::kRegularImportsOffset);
            cell_index_ = -cell_index - 1;
        }
        self.LoadFixedArrayElement(context_, context_, cell_index_);
        self.LoadTaggedField(kInterpreterAccumulatorRegister, context_, Cell::kValueOffset);
    }

    pub fn StaModuleVariable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
        let mut context_ = context;
        for _ in 0..depth {
            self.LoadTaggedField(context_, context_, Context::kPreviousOffset);
        }
        self.LoadTaggedField(context_, context_, Context::kExtensionOffset);
        self.LoadTaggedField(context_, context_, SourceTextModule::kRegularExportsOffset);

        let cell_index_ = cell_index - 1;
        self.LoadFixedArrayElement(context_, context_, cell_index_);
        self.StoreTaggedFieldWithWriteBarrier(context_, Cell::kValueOffset, value);
    }

    pub fn IncrementSmi(&mut self, lhs: MemOperand) {
        let mut temps = ScratchRegisterScope::new(std::mem::transmute(self));
        let tmp = temps.AcquireScratch();
        unsafe { (&mut *self.masm_).ldr(tmp, lhs) };
        unsafe { (&mut *self.masm_).add(tmp, tmp, Operand::new(Smi::FromInt(1).ptr() as i32), 0) };
        unsafe { (&mut *self.masm_).str(tmp, lhs) };
    }

    pub fn Word32And(&mut self, output: Register, lhs: Register, rhs: i32) {
        unsafe { (&mut *self.masm_).and_(output, lhs, Operand::new(rhs)) };
    }

    pub fn Switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [*mut Label], num_labels: i32) {
        unsafe {
            MacroAssembler::Switch(Register::no_reg(), reg, case_value_base, labels, num_labels);
        };
    }

    fn add(&mut self, scratch: Register, sp: Register, operand: Operand) {}
    fn AssertNotSmi(&mut self, map: Register) {}
    fn CompareObjectType(&mut self, map: Register, type_: Register, type_1: Register, map_type: i32) {}
    fn ldr(&mut self, tmp: Register, operand: MemOperand) {}
    fn LoadMap(&mut self, map: Register, object: Register) {}
    fn ldrh(&mut self, type_: Register, fieldMemOperand: FieldMemOperand) {}
    fn AssertSmi(&mut self, value: Register) {}
    fn RecordWriteField(&mut self, target: Register, offset: i32, value: Register, i: i32, kIgnore: SaveFPRegsMode) {}
    fn TestCodeIsMarkedForDeoptimization(&mut self, scratch_and_result: Register, scratch: Register) {}
    fn str(&mut self, value: Register, fieldMemOperand: FieldMemOperand) {}
    fn Move32BitImmediate(&mut self, output: Register, operand: Operand) {}
    fn b(cc: Condition, target: *mut Label) {}
    fn tst(&mut self, value: Register, operand: Operand) {}
    fn JumpIfRoot(&mut self, value: Register, index: RootIndex, target: *mut Label) {}
    fn JumpIfNotRoot(&mut self, value: Register, index: RootIndex, target: *mut Label) {}
    fn LoadWeakValue(&mut self, scratch_and_result: Register, scratch_and_result1: Register, fallthrough: *mut Label) {}
    fn LoadFixedArrayElement(&mut self, context_: Register, context_1: Register, cell_index_: i32) {}
    fn SmiUntag(&mut self, output: Register) {}
    fn ldrb(&mut self, output: Register, fieldMemOperand: FieldMemOperand) {}
    fn and_(&mut self, output: Register, lhs: Register, operand: Operand) {}
}

impl MacroAssembler {
    fn b(&mut self, target: *mut Label) {}
    fn and_(&mut self, output: Register, lhs: Register, operand: Operand) {}
    fn str(&mut self, source: Register, output: MemOperand) {}
    fn AssertNotSmi(&mut self, map: Register) {}
    fn CompareObjectType(&mut self, map: Register, type_: Register, type_1: Register, map_type: i32) {}
    fn ldr(&mut self, tmp: Register, operand: MemOperand) {}
    fn LoadMap(&mut self, map: Register, object: Register) {}
    fn ldrh(&mut self, type_: Register, fieldMemOperand: FieldMemOperand) {}
    fn AssertSmi(&mut self, value: Register) {}
    fn RecordWriteField(&mut self, target: Register, offset: i32, value: Register, i: i32, kIgnore: SaveFPRegsMode) {}
    fn TestCodeIsMarkedForDeoptimization(&mut self, scratch_and_result: Register, scratch: Register) {}
    fn Move32BitImmediate(&mut self, output: Register, operand: Operand) {}
    fn tst(&mut self, value: Register, operand: Operand) {}
    fn JumpIfRoot(&mut self, value: Register, index: RootIndex, target: *mut Label) {}
    fn JumpIfNotRoot(&mut self, value: Register, index: RootIndex, target: *mut Label) {}
    fn JumpIfSmi(&mut self, value: Register, target: *mut Label) {}
    fn LoadWeakValue(&mut self, scratch_and_result: Register, scratch_and_result1: Register, fallthrough: *mut Label) {}
    fn add(&mut self, interrupt_budget: Register, interrupt_budget1: Register, operand: Operand, i: i32) {}
}

fn __ClearedValue() -> i32 {
    0
}

#[derive(PartialEq)]
pub enum LeaveCC {
    kLessThan,
}

const eq: Condition = Condition {};
const ge: Condition = Condition {};
const MAP_TYPE: i32 = 0;

pub struct Map {}

impl Map {
    pub const kInstanceTypeOffset: i32 = 0;
}

impl Register {
    pub fn no_reg() -> Register {
        Register {}
    }
}

impl Operand {
    pub fn new(value: i32) -> Operand {
        Operand {}
    }
}

pub mod baseline {
    use super::*;

    pub fn EmitReturn(masm: *mut MacroAssembler) {
        let mut basm = BaselineAssembler::new(masm);
        let weight = BaselineLeaveFrameDescriptor::WeightRegister();
        let params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();

        {
            let mut skip_interrupt_label = Label::new();
            basm.AddToInterruptBudgetAndJumpIfNotExceeded(0, std::ptr::null_mut());
            {
                unsafe { (&mut *basm.masm_).SmiTag(params_size) };
                basm.Push(&[params_size, kInterpreterAccumulatorRegister]);

                unsafe { (&mut *basm.masm_).LoadContext(kContextRegister) };
                unsafe { (&mut *basm.masm_).LoadFunction(kJSFunctionRegister) };
                basm.Push(&[kJSFunctionRegister]);
                unsafe { (&mut *basm.masm_).CallRuntime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1) };

                basm.Pop(&[kInterpreterAccumulatorRegister, params_size]);
                unsafe { (&mut *basm.masm_).SmiUntag(params_size) };
            }

           basm.Bind(&mut skip_interrupt_label);
        }

        let mut temps = ScratchRegisterScope::new(std::mem::transmute(&mut basm));
        let actual_params_size = temps.AcquireScratch();
        unsafe { (&mut *basm.masm_).Move(actual_params_size, MemOperand::new(Register {}, StandardFrameConstants::kArgCOffset)) };
        unsafe { (&mut *basm.masm_).cmp(params_size, actual_params_size) };
        unsafe { (&mut *basm.masm_).mov(params_size, actual_params_size, eq, LeaveCC::kLessThan as i32) };
        unsafe { (&mut *basm.masm_).LeaveFrame(StackFrame::BASELINE) };
        unsafe { (&mut *basm.masm_).DropArguments(params_size) };
        unsafe { (&mut *basm.masm_).Ret() };
    }
    impl BaselineAssembler {
        pub fn Push(&mut self, registers: &[Register]) {}
        pub fn Pop(&mut self, registers: &[Register]) {}
    }

    impl MacroAssembler {
        pub fn LoadContext(&mut self, contextRegister: Register) {}
        pub fn LoadFunction(&mut self, jsFunctionRegister: Register) {}
        pub fn CallRuntime(&mut self, bytecodeBudgetInterruptSparkplug: Runtime, i: i32) {}
        pub fn Move(&mut self, actual_params_size: Register, memOperand: MemOperand) {}
    }

    pub struct EnsureAccum

// Converted from V8 C++ source files:
// Header: baseline-assembler-riscv-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod riscv_baseline_assembler_riscv_inl {
use crate::baseline::baseline_assembler::BaselineAssembler;
use crate::codegen::assembler_riscv::Operand;
use crate::codegen::interface_descriptors::BaselineLeaveFrameDescriptor;
use crate::init::bootstrapper::If;
use crate::objects::literal_objects_inl::SmiValuesAre31Bits;
use crate::Register;
use std::sync::Arc;

use crate::init::v8::V8;
use crate::init::bootstrapper::v8;
use crate::snapshot::references::SnapshotSpace;
use crate::baseline::ppc::baseline_assembler_ppc_inl::Register;
use crate::compiler::backend::arm::code_generator_arm::UseScratchRegisterScope;
use crate::baseline::s390::baseline_assembler_s390_inl::detail;
use crate::baseline::ia32::baseline_assembler_ia32_inl::MemOperand;
use crate::baseline::loong64::baseline_compiler_loong64_inl::RootIndex;
use crate::baseline::ppc::baseline_assembler_ppc_inl::Condition;
use crate::baseline::ppc::baseline_assembler_ppc_inl::ScratchRegisterScope;
use crate::compiler::turboshaft::build_graph_phase::ZoneWithNamePointer;
use crate::baseline::x64::baseline_assembler_x64_inl::InstanceType;
use crate::baseline::ppc::baseline_assembler_ppc_inl::ExternalReference;
use crate::include::v8_fast_api_calls::CFunctionBuilderWithFunction;
use crate::compiler::backend::mips64::code_generator_mips64::Local;
use crate::baseline::mips64::baseline_compiler_mips64_inl::BaselineAssemblerMIPS64;
use crate::torque::utils::EraseIf;
use crate::compiler::simplified_operator_reducer::Is;
use crate::compiler::backend::loong64::code_generator_loong64::Label;
use crate::flags::flags_impl::set;
use crate::flags::flags_impl::flags;
use crate::compiler::control_path_state::depth;
use crate::baseline::s390::baseline_compiler_s390_inl::index;
use crate::init::bootstrapper::to;
use crate::deoptimizer::frame_description::free;
use crate::compiler::backend::arm64::code_generator_arm64::stack;
use crate::tasks::cancelable_task::Cancelable;
use crate::compiler::js_inlining::receiver;
use crate::baseline::ppc::baseline_assembler_ppc_inl::FeedbackSlot;
use crate::baseline::ia32::baseline_assembler_ia32_inl::CodeWrapper;
use crate::compiler::backend::ppc::unwinding_info_writer_ppc::flags;
use crate::baseline::ppc::baseline_assembler_ppc_inl::CompressionMode;
use crate::baseline::ppc::baseline_assembler_ppc_inl::BaselineAssembler as PPCBaselineAssembler;
use crate::execution::isolate::this;
use crate::torque::ls::message::source;
use crate::torque::ls::message::code;
use crate::compiler::backend::riscv::instruction_selector_riscv64::Use;
use crate::compiler::backend::riscv::instruction_selector_riscv::is;
use crate::compiler::persistent_map::first;
use crate::baseline::ia32::baseline_assembler_ia32_inl::add;
use crate::compiler::representation_change::bool;
use crate::baseline::arm64::baseline_compiler_arm64_inl::ScratchRegisterScope as Arm64ScratchRegisterScope;
use crate::torque::torque::SourceFileMapScope;
use crate::init::v8::size;
use crate::torque::declaration_visitor::parameters;
use crate::compiler::backend::riscv::instruction_selector_riscv::use;
use crate::compiler::backend::riscv::instruction_selector_riscv::frame;

use crate::codegen::macro_assembler::MacroAssembler;
use crate::codegen::register::RegList;
use crate::codegen::register::RegisterConfiguration;
use crate::codegen::riscv64::register::kScratchReg;
use crate::codegen::riscv64::register::kScratchReg2;
use crate::interpreter;
use crate::objects::contexts::Context;
use crate::objects::contexts::Context::kPreviousOffset;
use crate::objects::contexts::Context::kExtensionOffset;
use crate::objects::contexts::Context::OffsetOfElementAt;
use crate::objects::fixed_array::FixedArray::kLengthOffset;
use crate::objects::heap_object::HeapObject;
use crate::objects::map::Map::kInstanceTypeOffset;
use crate::objects::source_text_module::SourceTextModule;
use crate::objects::smi::Smi;
use crate::codegen::riscv64::assembler::Assembler;
use crate::codegen::riscv64::register::zero_reg;
use crate::codegen::riscv64::assembler::kUnsignedGreaterThanEqual;
use crate::codegen::riscv64::register::t6;
use crate::codegen::riscv64::assembler::ASM_CODE_COMMENT;
use crate::codegen::register::kInterpreterAccumulatorRegister;
use crate::codegen::riscv64::assembler::AbortReason;
use crate::codegen::register::kContextRegister;
use crate::codegen::register::kJSFunctionRegister;
use crate::runtime::runtime::Runtime;
use crate::codegen::riscv64::assembler::StackFrame;
use crate::codegen::riscv64::assembler::ge;
use crate::codegen::riscv64::assembler::Operand::*;
use crate::codegen::riscv64::assembler::SaveFPRegsMode;
use crate::codegen::riscv64::assembler::kRAHasNotBeenSaved;
use crate::utils::PtrComprCageBase;
use crate::objects::feedback_vector::FeedbackVector;
use crate::objects::feedback_cell::FeedbackCell;
use crate::codegen::riscv64::assembler::eq;
use std::mem::size_of;
use crate::objects::code::Code;

pub struct EnsureAccumulatorPreservedScope<'a> {
    assembler_: &'a mut BaselineAssembler,
}

impl<'a> EnsureAccumulatorPreservedScope<'a> {
    pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
        EnsureAccumulatorPreservedScope { assembler_: assembler }
    }

    fn AssertEqualToAccumulator(&self, reg: Register) {
        self.assembler_.masm().Assert(eq, AbortReason::kAccumulatorClobbered, reg,
                                     Operand::from(kInterpreterAccumulatorRegister as i64));
    }
}

impl BaselineAssembler {
    pub struct ScratchRegisterScope<'a> {
        assembler_: &'a mut BaselineAssembler,
        prev_scope_: Option<Box<ScratchRegisterScope<'a>>>,
        wrapped_scope_: UseScratchRegisterScope,
    }

    impl<'a> ScratchRegisterScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
            let mut wrapped_scope_ = UseScratchRegisterScope::new(assembler.masm());
            let prev_scope_ = assembler.scratch_register_scope_.take();

            if prev_scope_.is_none() {
                wrapped_scope_.Include(kScratchReg, kScratchReg2);
            }

            let scope = ScratchRegisterScope {
                assembler_: assembler,
                prev_scope_: prev_scope_,
                wrapped_scope_: wrapped_scope_,
            };
            assembler.scratch_register_scope_ = Some(Box::new(scope));
            
            // This returns a mutable reference to `scope`, which is now part of the `assembler`.
            let ptr = unsafe {
                assembler.scratch_register_scope_.as_mut().unwrap().as_mut()
            };

            unsafe { std::ptr::read(ptr) }
        }

        pub fn AcquireScratch(&mut self) -> Register {
            self.wrapped_scope_.Acquire()
        }
    }
    
    pub fn scratch_register_scope_mut(&mut self) -> Option<&mut ScratchRegisterScope> {
        self.scratch_register_scope_.as_mut().map(|boxed_scope| boxed_scope.as_mut())
    }
}

impl Drop for BaselineAssembler::ScratchRegisterScope<'_> {
    fn drop(&mut self) {
        if let Some(prev_scope) = self.prev_scope_.take() {
            self.assembler_.scratch_register_scope_ = Some(prev_scope);
        } else {
            self.assembler_.scratch_register_scope_ = None;
        }
    }
}

pub mod detail {
    use crate::Register;
    use crate::baseline::ia32::baseline_assembler_ia32_inl::MemOperand;

    #[cfg(debug_assertions)]
    pub fn Clobbers(target: Register, op: MemOperand) -> bool {
        op.is_reg() && op.rm() == target
    }
}

impl BaselineAssembler {
    pub fn RegisterFrameOperand(
        &mut self,
        interpreter_register: interpreter::Register,
    ) -> MemOperand {
        MemOperand::new(self.fp, interpreter_register.ToOperand() * size_of::<*mut std::ffi::c_void>() as i32)
    }

    pub fn RegisterFrameAddress(
        &mut self,
        interpreter_register: interpreter::Register,
        rscratch: Register,
    ) {
        self.AddWord(rscratch, self.fp, interpreter_register.ToOperand() * size_of::<*mut std::ffi::c_void>() as i32);
    }

    pub fn FeedbackVectorOperand(&mut self) -> MemOperand {
        MemOperand::new(self.fp, BaselineFrameConstants::kFeedbackVectorFromFp as i32)
    }

    pub fn FeedbackCellOperand(&mut self) -> MemOperand {
        MemOperand::new(self.fp, BaselineFrameConstants::kFeedbackCellFromFp as i32)
    }

    pub fn Bind(&mut self, label: &mut Label) {
        self.masm().bind(label);
    }

    pub fn JumpTarget(&mut self) {
        // Nop
    }

    pub fn Jump(&mut self, target: &mut Label, distance: Label::Distance) {
        self.masm().jmp(target, distance);
    }

    pub fn JumpIfRoot(
        &mut self,
        value: Register,
        index: RootIndex,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.masm().JumpIfRoot(value, index, target, distance);
    }

    pub fn JumpIfNotRoot(
        &mut self,
        value: Register,
        index: RootIndex,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.masm().JumpIfNotRoot(value, index, target, distance);
    }

    pub fn JumpIfSmi(&mut self, value: Register, target: &mut Label, distance: Label::Distance) {
        self.masm().JumpIfSmi(value, target, distance);
    }

    pub fn JumpIfNotSmi(&mut self, value: Register, target: &mut Label) {
        self.masm().JumpIfNotSmi(value, target);
    }

    pub fn JumpIfImmediate(
        &mut self,
        cc: Condition,
        left: Register,
        right: i32,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.JumpIf(cc, left, Operand::from(right), target, distance);
    }

    pub fn TestAndBranch(
        &mut self,
        value: Register,
        mask: i32,
        cc: Condition,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let tmp = temps.AcquireScratch();
        self.And(tmp, value, Operand::from(mask));
        self.Branch(target, cc, tmp, Operand::from(zero_reg as i64), distance);
    }

    pub fn JumpIf(
        &mut self,
        cc: Condition,
        lhs: Register,
        rhs: Operand,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.Branch(target, cc, lhs, rhs, distance);
    }

    #[cfg(V8_STATIC_ROOTS_BOOL)]
    pub fn JumpIfJSAnyIsPrimitive(
        &mut self,
        heap_object: Register,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.AssertNotSmi(heap_object);
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let scratch = temps.AcquireScratch();
        self.masm().JumpIfJSAnyIsPrimitive(heap_object, scratch, target, distance);
    }

    pub fn JumpIfObjectTypeFast(
        &mut self,
        cc: Condition,
        object: Register,
        instance_type: InstanceType,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let scratch = temps.AcquireScratch();
        if cc == Condition::eq || cc == Condition::ne {
            self.masm().JumpIfObjectType(target, cc, object, instance_type, scratch);
            return;
        }
        self.JumpIfObjectType(
            cc,
            object,
            instance_type,
            scratch,
            target,
            distance,
        );
    }

    pub fn JumpIfObjectType(
        &mut self,
        cc: Condition,
        object: Register,
        instance_type: InstanceType,
        map: Register,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let type_reg = temps.AcquireScratch();
        self.GetObjectType(object, map, type_reg);
        self.Branch(target, cc, type_reg, Operand::from(instance_type as i64), distance);
    }

    pub fn JumpIfInstanceType(
        &mut self,
        cc: Condition,
        map: Register,
        instance_type: InstanceType,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let type_reg = temps.AcquireScratch();

        if self.v8_flags.debug_code {
            self.AssertNotSmi(map);
            self.GetObjectType(map, type_reg, type_reg);
            self.Assert(eq, AbortReason::kUnexpectedValue, type_reg, Operand::from(1));
        }

        self.LoadWord(type_reg, MemOperand::new(map, Map::kInstanceTypeOffset as i32));
        self.Branch(target, cc, type_reg, Operand::from(instance_type as i64), distance);
    }

    pub fn JumpIfPointer(
        &mut self,
        cc: Condition,
        value: Register,
        operand: MemOperand,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let temp = temps.AcquireScratch();
        self.LoadWord(temp, operand);
        self.Branch(target, cc, value, Operand::from(temp as i64), distance); //TODO: verify that converting Register to i64 is sound
    }

    pub fn JumpIfSmi1(
        &mut self,
        cc: Condition,
        value: Register,
        smi: Tagged<Smi>,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.masm().CompareTaggedAndBranch(target, cc, value, Operand::from(smi.ptr()));
    }

    pub fn JumpIfSmi2(
        &mut self,
        cc: Condition,
        lhs: Register,
        rhs: Register,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.AssertSmi(lhs);
        self.AssertSmi(rhs);
        self.masm().CompareTaggedAndBranch(target, cc, lhs, Operand::from(rhs as i64), distance);
    }

    pub fn JumpIfTagged(
        &mut self,
        cc: Condition,
        value: Register,
        operand: MemOperand,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let scratch = temps.AcquireScratch();
        self.LoadWord(scratch, operand);
        self.masm().CompareTaggedAndBranch(target, cc, value, Operand::from(scratch as i64), distance); //TODO: verify that converting Register to i64 is sound
    }

    pub fn JumpIfTagged2(
        &mut self,
        cc: Condition,
        operand: MemOperand,
        value: Register,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let scratch = temps.AcquireScratch();
        self.LoadWord(scratch, operand);
        self.masm().CompareTaggedAndBranch(target, cc, scratch, Operand::from(value as i64), distance); //TODO: verify that converting Register to i64 is sound
    }

    pub fn JumpIfByte(
        &mut self,
        cc: Condition,
        value: Register,
        byte: i32,
        target: &mut Label,
        distance: Label::Distance,
    ) {
        self.Branch(target, cc, value, Operand::from(byte), distance);
    }

    pub fn Move(&mut self, output: interpreter::Register, source: Register) {
        self.Move1(self.RegisterFrameOperand(output), source);
    }

    pub fn Move1(&mut self, output: MemOperand, source: Register) {
        self.StoreWord(source, output);
    }

    pub fn Move2(&mut self, output: Register, reference: ExternalReference) {
        self.li(output, Operand::from(reference.ptr() as i64));
    }

    pub fn Move3(&mut self, output: Register, value: Handle<HeapObject>) {
        self.li(output, Operand::from(value.location() as i64));
    }

    pub fn Move4(&mut self, output: Register, value: i32) {
        self.li(output, Operand::from(value));
    }

    pub fn Move5(&mut self, output: Register, value: Tagged<TaggedIndex>) {
        self.li(output, Operand::from(value.ptr()));
    }

    pub fn MoveMaybeSmi(&mut self, output: Register, source: Register) {
        self.Move6(output, source);
    }

    pub fn MoveSmi(&mut self, output: Register, source: Register) {
        self.Move6(output, source);
    }

    pub fn Move6(&mut self, output: Register, source: Register) {
        self.masm().Move(output, source);
    }

    pub fn LoadTaggedField(&mut self, output: Register, source: Register, offset: i32) {
        self.masm().LoadTaggedField(output, MemOperand::new(source, offset));
    }

    pub fn LoadTaggedSignedField(&mut self, output: Register, source: Register, offset: i32) {
        self.masm().LoadTaggedSignedField(output, MemOperand::new(source, offset));
    }

    pub fn LoadTaggedSignedFieldAndUntag(&mut self, output: Register, source: Register, offset: i32) {
        self.LoadTaggedSignedField(output, source, offset);
        self.SmiUntag(output);
    }

    pub fn LoadWord16FieldZeroExtend(&mut self, output: Register, source: Register, offset: i32) {
        self.masm().Lhu(output, MemOperand::new(source, offset));
    }

    pub fn LoadWord8Field(&mut self, output: Register, source: Register, offset: i32) {
        self.masm().Lb(output, MemOperand::new(source, offset));
    }

    pub fn StoreTaggedSignedField(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
        ASM_CODE_COMMENT(self.masm());
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let tmp = temps.AcquireScratch();
        self.li(tmp, Operand::from(value.ptr()));
        self.masm().StoreTaggedField(tmp, MemOperand::new(target, offset));
    }

    pub fn StoreTaggedFieldWithWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
        ASM_CODE_COMMENT(self.masm());
        self.masm().StoreTaggedField(value, MemOperand::new(target, offset));
        self.masm().RecordWriteField(target, offset, value, kRAHasNotBeenSaved,
                            SaveFPRegsMode::kIgnore);
    }

    pub fn StoreTaggedFieldNoWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
        self.masm().StoreTaggedField(value, MemOperand::new(target, offset));
    }

    pub fn TryLoadOptimizedOsrCode(
        &mut self,
        scratch_and_result: Register,
        feedback_vector: Register,
        slot: FeedbackSlot,
        on_result: &mut Label,
        distance: Label::Distance,
    ) {
        let mut fallthrough = Label::new();
        let mut clear_slot = Label::new();
        self.LoadTaggedField(
            scratch_and_result,
            feedback_vector,
            FeedbackVector::OffsetOfElementAt(slot.ToInt()) as i32,
        );
        self.masm().LoadWeakValue(scratch_and_result, scratch_and_result, &mut fallthrough);

        {
            let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
            self.masm().LoadCodePointerField(
                scratch_and_result,
                MemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset as i32),
            );

            self.masm().JumpIfCodeIsMarkedForDeoptimization(
                scratch_and_result,
                temps.AcquireScratch(),
                &mut clear_slot,
            );
            self.Jump(on_result, distance);
        }

        self.Bind(&mut clear_slot);
        self.li(scratch_and_result, Operand::from(self.ClearedValue()));
        self.StoreTaggedFieldNoWriteBarrier(
            feedback_vector,
            FeedbackVector::OffsetOfElementAt(slot.ToInt()) as i32,
            scratch_and_result,
        );

        self.Bind(&mut fallthrough);
        self.Move4(scratch_and_result, 0);
    }

    pub fn AddToInterruptBudgetAndJumpIfNotExceeded(
        &mut self,
        weight: i32,
        skip_interrupt_label: Option<&mut Label>,
    ) {
        ASM_CODE_COMMENT(self.masm());
        let mut scratch_scope = BaselineAssembler::ScratchRegisterScope::new(self);
        let feedback_cell = scratch_scope.AcquireScratch();
        self.LoadFeedbackCell(feedback_cell);

        let interrupt_budget = scratch_scope.AcquireScratch();
        self.masm().Lw(
            interrupt_budget,
            MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
        );

        self.Add32(interrupt_budget, interrupt_budget, Operand::from(weight));

        self.masm().Sw(
            interrupt_budget,
            MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
        );

        if let Some(label) = skip_interrupt_label {
            assert!(weight < 0);
            self.Branch(label, ge, interrupt_budget, Operand::from(zero_reg as i64), Label::Distance::kNear);
        }
    }

    pub fn AddToInterruptBudgetAndJumpIfNotExceeded1(
        &mut self,
        weight: Register,
        skip_interrupt_label: Option<&mut Label>,
    ) {
        ASM_CODE_COMMENT(self.masm());
        let mut scratch_scope = BaselineAssembler::ScratchRegisterScope::new(self);
        let feedback_cell = scratch_scope.AcquireScratch();
        self.LoadFeedbackCell(feedback_cell);

        let interrupt_budget = scratch_scope.AcquireScratch();
        self.masm().Lw(
            interrupt_budget,
            MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
        );

        self.Add32(interrupt_budget, interrupt_budget, Operand::from(weight as i64));

        self.masm().Sw(
            interrupt_budget,
            MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
        );

        if let Some(label) = skip_interrupt_label {
            self.Branch(label, ge, interrupt_budget, Operand::from(zero_reg as i64), Label::Distance::kNear);
        }
    }

    pub fn LdaContextSlot(
        &mut self,
        context: Register,
        index: u32,
        depth: u32,
        compression_mode: CompressionMode,
    ) {
        let mut current_context = context;
        for _ in 0..depth {
            self.LoadTaggedField(current_context, current_context, Context::kPreviousOffset as i32);
        }
        self.LoadTaggedField(
            kInterpreterAccumulatorRegister,
            current_context,
            Context::OffsetOfElementAt(index) as i32,
        );
    }

    pub fn StaContextSlot(
        &mut self,
        context: Register,
        value: Register,
        index: u32,
        depth: u32,
    ) {
        let mut current_context = context;
        for _ in 0..depth {
            self.LoadTaggedField(current_context, current_context, Context::kPreviousOffset as i32);
        }
        self.StoreTaggedFieldWithWriteBarrier(
            current_context,
            Context::OffsetOfElementAt(index) as i32,
            value,
        );
    }

    pub fn LdaModuleVariable(&mut self, context: Register, cell_index: i32, depth: u32) {
        let mut current_context = context;
        for _ in 0..depth {
            self.LoadTaggedField(current_context, current_context, Context::kPreviousOffset as i32);
        }
        self.LoadTaggedField(current_context, current_context, Context::kExtensionOffset as i32);
        let mut actual_cell_index = cell_index;
        if cell_index > 0 {
            self.LoadTaggedField(
                current_context,
                current_context,
                SourceTextModule::kRegularExportsOffset as i32,
            );
            actual_cell_index -= 1;
        } else {
            self.LoadTaggedField(
                current_context,
                current_context,
                SourceTextModule::kRegularImportsOffset as i32,
            );
            actual_cell_index = -cell_index - 1;
        }
        self.LoadFixedArrayElement(current_context, current_context, actual_cell_index);
        self.LoadTaggedField(kInterpreterAccumulatorRegister, current_context, Cell::kValueOffset as i32);
    }

    pub fn StaModuleVariable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
        let mut current_context = context;
        for _ in 0..depth {
            self.LoadTaggedField(current_context, current_context, Context::kPreviousOffset as i32);
        }
        self.LoadTaggedField(current_context, current_context, Context::kExtensionOffset as i32);
        self.LoadTaggedField(
            current_context,
            current_context,
            SourceTextModule::kRegularExportsOffset as i32,
        );

        let actual_cell_index = cell_index - 1;
        self.LoadFixedArrayElement(current_context, current_context, actual_cell_index);
        self.StoreTaggedFieldWithWriteBarrier(current_context, Cell::kValueOffset as i32, value);
    }

    pub fn IncrementSmi(&mut self, lhs: MemOperand) {
        let mut temps = BaselineAssembler::ScratchRegisterScope::new(self);
        let tmp = temps.AcquireScratch();
        ASM_CODE_COMMENT(self.masm());
        if SmiValuesAre31Bits() {
            self.masm().Lw(tmp, lhs);
            self.Add32(tmp, tmp, Operand::from(Smi::FromInt(1)));
            self.masm().Sw(tmp, lhs);
        } else {
            self.LoadWord(tmp, lhs);
            self.AddWord(tmp, tmp, Operand::from(Smi::FromInt(1)));
            self.StoreWord(tmp, lhs);
        }
    }

    pub fn Word32And(&mut self, output: Register, lhs: Register, rhs: i32) {
        self.And(output, lhs, Operand::from(rhs));
    }

    pub fn Switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: i32) {
        ASM_CODE_COMMENT(self.masm());
        let mut fallthrough = Label::new();
        if case_value_base != 0 {
            self.SubWord(reg, reg, Operand::from(case_value_base));
        }

        let mut scope = BaselineAssembler::ScratchRegisterScope::new(self);
        let mut table = Label::new();
        self.Branch(&mut fallthrough, kUnsignedGreaterThanEqual, reg, Operand::from(num_labels as i64), Label::Distance::kNear);
        let imm64 = self.masm().branch_long_offset(&table);
        assert!((imm64 + 0x800) >= i32::MIN as i64 && (imm64 + 0x800) <= i32::MAX as i64);
        let hi20 = (((imm64 + 0x800) as i32) >> 12);
        let lo12 = ((imm64 << 20) >> 20) as i32;
        self.BlockTrampolinePoolFor(2);
        self.masm().auipc(t6, hi20);
        self.masm().addi(t6, t6, lo12);

        let entry_size_log2 = 3;
        self.BlockTrampolinePoolFor(num_labels as usize * 2 + 5);
        self.CalcScaledAddress(t6, t6, reg, entry_size_log2);
        self.masm().Jump(t6);
        {
            self.Bind(&mut table);
            for i in 0..num_labels {
                self.masm().BranchLong(labels[i as usize]);
            }
            assert_eq!(num_labels as usize * 2, self.masm().InstructionsGeneratedSince(&table) as usize);
        }
        self.Bind(&mut fallthrough);
    }
}

impl BaselineAssembler {
    pub fn EmitReturn(masm: &mut MacroAssembler) {
        ASM_CODE_COMMENT(masm);
        let mut basm = BaselineAssembler::new_for_macro_assembler(masm);

        let weight = BaselineLeaveFrameDescriptor::WeightRegister();
        let params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();

        {
            ASM_CODE_COMMENT_STRING(masm, "Update Interrupt Budget");

            let mut skip_interrupt_label = Label::new();
            basm.AddToInterruptBudgetAndJumpIfNotExceeded(0, Some(&mut skip_interrupt_label));
            masm.SmiTag(params_size);
            masm.Push(params_size, kInterpreterAccumulatorRegister);

            basm.LoadContext(kContextRegister);
            basm.LoadFunction(kJSFunctionRegister);
            masm.Push(kJSFunctionRegister);
            masm.CallRuntime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

            masm.Pop(params_size, kInterpreterAccumulatorRegister);
            masm.SmiUntag(params_size);

            basm.Bind(&mut skip_interrupt_label);
        }

        let mut temps = BaselineAssembler::ScratchRegisterScope::new(&mut basm);
        let actual_params_size = temps.AcquireScratch();
        basm.Move6(actual_params_size, actual_params_size); //dummy to avoid usage warning.
        masm.Move(actual_params_size,
                MemOperand::new(masm.fp(), StandardFrameConstants::kArgCOffset as i32));

        let mut corrected_args_count = Label::new

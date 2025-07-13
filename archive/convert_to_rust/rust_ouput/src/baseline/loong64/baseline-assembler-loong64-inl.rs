// Converted from V8 C++ source files:
// Header: baseline-assembler-loong64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod loong64_baseline_assembler_loong64_inl {
    use crate::baseline::baseline_assembler::*;
    use crate::codegen::interface_descriptors::*;
    use crate::codegen::loong64::assembler_loong64_inl::*;
    use crate::objects::literal_objects_inl::*;
    use crate::execution::isolate::Isolate;
    use std::sync::{Arc, Mutex};

    pub struct BaselineAssembler {
        masm_: Box<MacroAssembler>, // Assuming MacroAssembler is defined elsewhere
        scratch_register_scope_: *mut ScratchRegisterScope,
    }

    impl BaselineAssembler {
        pub fn masm(&mut self) -> &mut MacroAssembler {
            &mut self.masm_
        }
    }

    impl BaselineAssembler {
        pub fn new(masm: *mut MacroAssembler) -> Self {
            BaselineAssembler {
                masm_: unsafe { Box::from_raw(masm) },
                scratch_register_scope_: std::ptr::null_mut(),
            }
        }
    }

    impl Drop for BaselineAssembler {
        fn drop(&mut self) {
            // Handle dropping of resources if needed, such as ensuring
            // the MacroAssembler is properly finalized or deallocated.
        }
    }

    impl BaselineAssembler {
        pub fn RegisterFrameOperand(
            &mut self,
            interpreter_register: interpreter::Register,
        ) -> MemOperand {
            MemOperand::new(
                fp,
                interpreter_register.to_operand() * kSystemPointerSize as i32,
            )
        }

        pub fn RegisterFrameAddress(
            &mut self,
            interpreter_register: interpreter::Register,
            rscratch: Register,
        ) {
            self.masm_
                .Add_d(rscratch, fp, interpreter_register.to_operand() * kSystemPointerSize as i32);
        }

        pub fn FeedbackVectorOperand(&mut self) -> MemOperand {
            MemOperand::new(fp, BaselineFrameConstants::kFeedbackVectorFromFp)
        }

        pub fn FeedbackCellOperand(&mut self) -> MemOperand {
            MemOperand::new(fp, BaselineFrameConstants::kFeedbackCellFromFp)
        }

        pub fn Bind(&mut self, label: &mut Label) {
            self.masm_.bind(label);
        }

        pub fn JumpTarget(&mut self) {}

        pub fn Jump(&mut self, target: &mut Label, distance: Label::Distance) {
            self.masm_.Branch(target);
        }

        pub fn JumpIfRoot(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.JumpIfRoot(value, index, target);
        }

        pub fn JumpIfNotRoot(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.JumpIfNotRoot(value, index, target);
        }

        pub fn JumpIfSmi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.JumpIfSmi(value, target);
        }

        pub fn JumpIfNotSmi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.JumpIfNotSmi(value, target);
        }

        pub fn JumpIfImmediate(
            &mut self,
            cc: Condition,
            left: Register,
            right: i32,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            self.JumpIf(cc, left, Operand::Immediate(right), target, distance);
        }

        pub fn TestAndBranch(
            &mut self,
            value: Register,
            mask: i32,
            cc: Condition,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_.And(scratch, value, Operand::Immediate(mask));
            self.masm_
                .Branch(target, cc, scratch, Operand::Register(zero_reg));
        }

        pub fn JumpIf(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Operand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.Branch(target, cc, lhs, rhs);
        }

        #[cfg(V8_STATIC_ROOTS_BOOL)]
        pub fn JumpIfJSAnyIsPrimitive(
            &mut self,
            heap_object: Register,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            self.masm_.AssertNotSmi(heap_object);
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_
                .JumpIfJSAnyIsPrimitive(heap_object, scratch, target, distance);
        }

        pub fn JumpIfObjectTypeFast(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            if cc == Condition::eq || cc == Condition::ne {
                self.masm_
                    .JumpIfObjectType(target, cc, object, instance_type, scratch);
                return;
            }
            self.JumpIfObjectType(cc, object, instance_type, scratch, target, distance);
        }

        pub fn JumpIfObjectType(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            map: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            self.masm_.GetObjectType(object, map, type_reg);
            self.masm_.Branch(target, cc, type_reg, Operand::Immediate(instance_type as i32));
        }

        pub fn JumpIfInstanceType(
            &mut self,
            cc: Condition,
            map: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            if v8_flags.debug_code {
                self.masm_.AssertNotSmi(map);
                self.masm_.GetObjectType(map, type_reg, type_reg);
                self.masm_
                    .Assert(Condition::eq, AbortReason::kUnexpectedValue, type_reg, Operand::Immediate(MAP_TYPE as i32));
            }
            self.masm_.Ld_hu(type_reg, FieldMemOperand::new(map, Map::kInstanceTypeOffset as i32));
            self.masm_
                .Branch(target, cc, type_reg, Operand::Immediate(instance_type as i32));
        }

        pub fn JumpIfSmi1(
            &mut self,
            cc: Condition,
            value: Register,
            smi: Tagged<Smi>,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.CompareTaggedAndBranch(target, cc, value, Operand::Immediate(smi.ptr() as i32));
        }

        pub fn JumpIfSmi2(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.AssertSmi(lhs);
            self.masm_.AssertSmi(rhs);
            self.masm_.CompareTaggedAndBranch(target, cc, lhs, Operand::Register(rhs));
        }

        pub fn JumpIfTagged(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_.Ld_d(scratch, operand);
            self.masm_.CompareTaggedAndBranch(target, cc, value, Operand::Register(scratch));
        }

        pub fn JumpIfTagged2(
            &mut self,
            cc: Condition,
            operand: MemOperand,
            value: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_.Ld_d(scratch, operand);
            self.masm_.CompareTaggedAndBranch(target, cc, scratch, Operand::Register(value));
        }

        pub fn JumpIfByte(
            &mut self,
            cc: Condition,
            value: Register,
            byte: i32,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_.Branch(target, cc, value, Operand::Immediate(byte));
        }

        pub fn Move(&mut self, output: interpreter::Register, source: Register) {
            self.Move1(self.RegisterFrameOperand(output), source);
        }

        pub fn Move2(&mut self, output: Register, value: Tagged<TaggedIndex>) {
            self.masm_.li(output, Operand::Immediate(value.ptr() as i32));
        }

        pub fn Move1(&mut self, output: MemOperand, source: Register) {
            self.masm_.St_d(source, output);
        }

        pub fn Move3(&mut self, output: Register, reference: ExternalReference) {
            self.masm_.li(output, Operand::ExternalReference(reference));
        }

        pub fn Move4(&mut self, output: Register, value: Handle<HeapObject>) {
            self.masm_.li(output, Operand::Handle(value));
        }

        pub fn Move5(&mut self, output: Register, value: i32) {
            self.masm_.li(output, Operand::Immediate(value));
        }

        pub fn MoveMaybeSmi(&mut self, output: Register, source: Register) {
            self.masm_.Move(output, source);
        }

        pub fn MoveSmi(&mut self, output: Register, source: Register) {
            self.masm_.Move(output, source);
        }

        pub fn LoadTaggedField(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedField(output, FieldMemOperand::new(source, offset));
        }

        pub fn LoadTaggedSignedField(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedSignedField(output, FieldMemOperand::new(source, offset));
        }

        pub fn LoadTaggedSignedFieldAndUntag(&mut self, output: Register, source: Register, offset: i32) {
            self.LoadTaggedSignedField(output, source, offset);
            self.SmiUntag(output);
        }

        pub fn LoadWord16FieldZeroExtend(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.Ld_hu(output, FieldMemOperand::new(source, offset));
        }

        pub fn LoadWord8Field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.Ld_b(output, FieldMemOperand::new(source, offset));
        }

        pub fn StoreTaggedSignedField(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
            self.ASM_CODE_COMMENT();
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_.li(scratch, Operand::Immediate(value.ptr() as i32));
            self.masm_.StoreTaggedField(scratch, FieldMemOperand::new(target, offset));
        }

        pub fn StoreTaggedFieldWithWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
            self.ASM_CODE_COMMENT();
            self.masm_.StoreTaggedField(value, FieldMemOperand::new(target, offset));
            let mut temps = ScratchRegisterScope::new(self);
            self.masm_.RecordWriteField(
                target,
                offset,
                value,
                kRAHasNotBeenSaved,
                SaveFPRegsMode::kIgnore,
            );
        }

        pub fn StoreTaggedFieldNoWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_.StoreTaggedField(value, FieldMemOperand::new(target, offset));
        }

        pub fn TryLoadOptimizedOsrCode(
            &mut self,
            scratch_and_result: Register,
            feedback_vector: Register,
            slot: FeedbackSlot,
            on_result: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut fallthrough = Label::new();
            self.LoadTaggedField(
                scratch_and_result,
                feedback_vector,
                FeedbackVector::OffsetOfElementAt(slot.to_int()),
            );
            self.masm_.LoadWeakValue(
                scratch_and_result,
                scratch_and_result,
                &mut fallthrough,
            );

            {
                let mut temps = ScratchRegisterScope::new(self);

                self.masm_.LoadCodePointerField(
                    scratch_and_result,
                    FieldMemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset),
                );

                let scratch = temps.acquire_scratch();
                self.masm_.TestCodeIsMarkedForDeoptimizationAndJump(
                    scratch_and_result,
                    scratch,
                    Condition::eq,
                    on_result,
                );
                self.masm_.li(scratch, Operand::Immediate(self.masm_.ClearedValue() as i32));
                self.StoreTaggedFieldNoWriteBarrier(
                    feedback_vector,
                    FeedbackVector::OffsetOfElementAt(slot.to_int()),
                    scratch,
                );
            }
            self.masm_.bind(&mut fallthrough);
            self.Move5(scratch_and_result, 0);
        }

        pub fn AddToInterruptBudgetAndJumpIfNotExceeded(
            &mut self,
            weight: i32,
            skip_interrupt_label: &mut Label,
        ) {
            self.ASM_CODE_COMMENT();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.LoadFeedbackCell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_.Ld_w(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset),
            );
            self.masm_
                .Add_w(interrupt_budget, interrupt_budget, Operand::Immediate(weight));
            self.masm_.St_w(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset),
            );
            if skip_interrupt_label.is_linked() {
                self.masm_.Branch(
                    skip_interrupt_label,
                    Condition::ge,
                    interrupt_budget,
                    Operand::Register(zero_reg),
                );
            }
        }

        pub fn AddToInterruptBudgetAndJumpIfNotExceeded2(
            &mut self,
            weight: Register,
            skip_interrupt_label: &mut Label,
        ) {
            self.ASM_CODE_COMMENT();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.LoadFeedbackCell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_.Ld_w(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset),
            );
            self.masm_.Add_w(interrupt_budget, interrupt_budget, Operand::Register(weight));
            self.masm_.St_w(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset),
            );
            if skip_interrupt_label.is_linked() {
                self.masm_.Branch(
                    skip_interrupt_label,
                    Condition::ge,
                    interrupt_budget,
                    Operand::Register(zero_reg),
                );
            }
        }

        pub fn LdaContextSlot(
            &mut self,
            context: Register,
            index: u32,
            depth: u32,
            compression_mode: CompressionMode,
        ) {
            for _ in 0..depth {
                self.LoadTaggedField(context, context, Context::kPreviousOffset);
            }
            self.LoadTaggedField(
                kInterpreterAccumulatorRegister,
                context,
                Context::OffsetOfElementAt(index),
            );
        }

        pub fn StaContextSlot(
            &mut self,
            context: Register,
            value: Register,
            index: u32,
            depth: u32,
        ) {
            for _ in 0..depth {
                self.LoadTaggedField(context, context, Context::kPreviousOffset);
            }
            self.StoreTaggedFieldWithWriteBarrier(
                context,
                Context::OffsetOfElementAt(index),
                value,
            );
        }

        pub fn LdaModuleVariable(&mut self, context: Register, cell_index: i32, depth: u32) {
            for _ in 0..depth {
                self.LoadTaggedField(context, context, Context::kPreviousOffset);
            }
            self.LoadTaggedField(context, context, Context::kExtensionOffset);
            if cell_index > 0 {
                self.LoadTaggedField(
                    context,
                    context,
                    SourceTextModule::kRegularExportsOffset,
                );
                // The actual array index is (cell_index - 1).
                let cell_index = cell_index - 1;
            } else {
                self.LoadTaggedField(
                    context,
                    context,
                    SourceTextModule::kRegularImportsOffset,
                );
                // The actual array index is (-cell_index - 1).
                let cell_index = -cell_index - 1;
            }
            self.LoadFixedArrayElement(context, context, cell_index);
            self.LoadTaggedField(
                kInterpreterAccumulatorRegister,
                context,
                Cell::kValueOffset,
            );
        }

        pub fn StaModuleVariable(
            &mut self,
            context: Register,
            value: Register,
            cell_index: i32,
            depth: u32,
        ) {
            for _ in 0..depth {
                self.LoadTaggedField(context, context, Context::kPreviousOffset);
            }
            self.LoadTaggedField(context, context, Context::kExtensionOffset);
            self.LoadTaggedField(
                context,
                context,
                SourceTextModule::kRegularExportsOffset,
            );

            // The actual array index is (cell_index - 1).
            let cell_index = cell_index - 1;
            self.LoadFixedArrayElement(context, context, cell_index);
            self.StoreTaggedFieldWithWriteBarrier(context, Cell::kValueOffset, value);
        }

        pub fn IncrementSmi(&mut self, lhs: MemOperand) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            if self.SmiValuesAre31Bits() {
                self.masm_.Ld_w(tmp, lhs);
                self.masm_
                    .Add_w(tmp, tmp, Operand::Immediate(Smi::from_int(1).value()));
                self.masm_.St_w(tmp, lhs);
            } else {
                self.masm_.Ld_d(tmp, lhs);
                self.masm_
                    .Add_d(tmp, tmp, Operand::Immediate(Smi::from_int(1).value()));
                self.masm_.St_d(tmp, lhs);
            }
        }

        pub fn Word32And(&mut self, output: Register, lhs: Register, rhs: i32) {
            self.masm_.And(output, lhs, Operand::Immediate(rhs));
        }

        pub fn Switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: i32) {
            self.ASM_CODE_COMMENT();
            let mut fallthrough = Label::new();
            if case_value_base != 0 {
                self.masm_
                    .Sub_d(reg, reg, Operand::Immediate(case_value_base));
            }

            self.masm_.Branch(
                &mut fallthrough,
                Condition::kUnsignedGreaterThanEqual,
                reg,
                Operand::Immediate(num_labels),
            );

            let label_ptrs: Vec<*mut Label> = labels.iter_mut().map(|label| *label as *mut Label).collect();
            let label_slice: &[*mut Label] = label_ptrs.as_slice();

            self.masm_.GenerateSwitchTable(
                reg,
                num_labels,
                |i| {
                    let label_ptr = label_slice[i as usize];
                    unsafe { &mut *label_ptr }
                },
            );

            self.masm_.bind(&mut fallthrough);
        }

        fn ASM_CODE_COMMENT(&mut self) {}

        fn SmiUntag(&mut self, output: Register) {}
        fn SmiValuesAre31Bits(&mut self) -> bool {
            true
        }
        fn LoadFixedArrayElement(&mut self, context: Register, context1: Register, cell_index: i32) {}
        fn LoadFeedbackCell(&mut self, feedback_cell: Register) {}
        fn ClearedValue(&mut self) -> Tagged<Smi> {
            Tagged::<Smi> {
                ptr: 0 as *mut std::ffi::c_void,
            }
        }
    }

    pub struct ScratchRegisterScope<'a> {
        assembler_: &'a mut BaselineAssembler,
        prev_scope_: *mut ScratchRegisterScope<'a>,
        wrapped_scope_: UseScratchRegisterScope,
        acquired_registers: Vec<Register>,
    }

    impl<'a> ScratchRegisterScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
            let mut scope = ScratchRegisterScope {
                assembler_: assembler,
                prev_scope_: assembler.scratch_register_scope_,
                wrapped_scope_: UseScratchRegisterScope::new(assembler.masm()),
                acquired_registers: Vec::new(),
            };

            if assembler.scratch_register_scope_.is_null() {
                // If we haven't opened a scratch scope yet, for the first one add a
                // couple of extra registers.
                scope.wrapped_scope_.include(vec![t0, t1, t2, t3]);
            }

            assembler.scratch_register_scope_ = &mut scope;
            scope
        }

        pub fn acquire_scratch(&mut self) -> Register {
            let reg = self.wrapped_scope_.acquire();
            self.acquired_registers.push(reg);
            reg
        }
    }

    impl<'a> Drop for ScratchRegisterScope<'a> {
        fn drop(&mut self) {
            self.assembler_.scratch_register_scope_ = self.prev_scope_;
        }
    }

    pub mod detail {
        use super::*;

        #[cfg(debug_assertions)]
        pub fn Clobbers(target: Register, op: MemOperand) -> bool {
            op.base() == target || op.index() == target
        }

        pub fn ToRegister(basm: &mut BaselineAssembler, scope: &mut ScratchRegisterScope, reg: Register) -> Register {
            reg
        }
    }

    impl MacroAssembler {
        fn GenerateSwitchTable(
            &mut self,
            reg: Register,
            num_labels: i32,
            label_callback: impl Fn(i32) -> &mut Label,
        ) {
            for i in 0..num_labels {
                let label = label_callback(i);
                self.bind(label);
            }
        }
        fn ClearedValue(&mut self) -> Tagged<Smi>{
            Tagged::<Smi> {
                ptr: 0 as *mut std::ffi::c_void,
            }
        }
    }

    impl BaselineAssembler {
        fn EmitReturn(&mut self, masm: &mut MacroAssembler) {
            self.ASM_CODE_COMMENT();

            let weight = BaselineLeaveFrameDescriptor::WeightRegister();
            let params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();

            {
                self.ASM_CODE_COMMENT();

                let mut skip_interrupt_label = Label::new();
                self.AddToInterruptBudgetAndJumpIfNotExceeded(
                    weight,
                    &mut skip_interrupt_label,
                );
                masm.SmiTag(params_size);
                masm.Push(params_size, kInterpreterAccumulatorRegister);

                self.LoadContext(kContextRegister);
                self.LoadFunction(kJSFunctionRegister);
                masm.Push(kJSFunctionRegister);
                masm.CallRuntime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

                masm.Pop(params_size, kInterpreterAccumulatorRegister);
                masm.SmiUntag(params_size);
                self.Bind(&mut skip_interrupt_label);
            }

            let mut temps = ScratchRegisterScope::new(self);
            let actual_params_size = temps.acquire_scratch();
            masm.Move(
                actual_params_size,
                MemOperand::new(fp, StandardFrameConstants::kArgCOffset),
            );

            let mut corrected_args_count = Label::new();
            masm.Branch(
                &mut corrected_args_count,
                Condition::ge,
                params_size,
                Operand::Register(actual_params_size),
            );
            masm.Move(params_size, actual_params_size);
            self.Bind(&mut corrected_args_count);

            masm.LeaveFrame(StackFrame::BASELINE);
            masm.DropArguments(params_size);
            masm.Ret();
        }

        fn LoadContext(&mut self, context_register: Register) {}
        fn LoadFunction(&mut self, js_function_register: Register) {}
    }

    struct EnsureAccumulatorPreservedScope<'a> {
        assembler_: &'a mut BaselineAssembler,
    }

    impl<'a> EnsureAccumulatorPreservedScope<'a> {
        fn AssertEqualToAccumulator(&self, reg: Register) {
            self.assembler_.masm().Assert(
                Condition::eq,
                AbortReason::kAccumulatorClobbered,
                reg,
                Operand::Register(kInterpreterAccumulatorRegister),
            );
        }
    }
}

pub use loong64_baseline_assembler_loong64_inl::*;

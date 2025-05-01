// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod baseline_assembler_arm {
    use crate::baseline::baseline_assembler::BaselineAssembler;
    use crate::codegen::arm::assembler_arm::Assembler;
    use crate::codegen::interface_descriptors::BaselineLeaveFrameDescriptor;
    use crate::objects::literal_objects::Smi;
    use crate::objects::{
        FeedbackCell, FeedbackVector, HeapObject, Map, SourceTextModule, Tagged, TaggedIndex,
    };
    use crate::{
        interpreter, Address, Code, CodeWrapper, Condition, ExternalReference, Handle, InstanceType,
        Label, Register, RootIndex, SaveFPRegsMode, StackFrame, StandardFrameConstants,
    };
    use std::marker::PhantomData;

    impl BaselineAssembler {
        pub struct ScratchRegisterScope<'a> {
            assembler_: &'a mut BaselineAssembler,
            prev_scope_: Option<Box<ScratchRegisterScope<'a>>>,
            wrapped_scope_: UseScratchRegisterScope<'a>,
        }

        impl<'a> ScratchRegisterScope<'a> {
            pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
                let mut wrapped_scope_ = UseScratchRegisterScope::new(assembler.masm_mut());
                if assembler.scratch_register_scope_.is_none() {
                    // If we haven't opened a scratch scope yet, for the first one add a
                    // couple of extra registers.
                    assert!(wrapped_scope_.can_acquire());
                    wrapped_scope_.include(Register::r8, Register::r9);
                    wrapped_scope_.include(Register::kInterpreterBytecodeOffsetRegister, Register::kInterpreterBytecodeOffsetRegister);
                }
                let prev_scope_ = assembler.scratch_register_scope_.take();
                let mut scope = ScratchRegisterScope {
                    assembler_: assembler,
                    prev_scope_: prev_scope_.map(|x| Box::new(*x)), // box to allow recursive struct
                    wrapped_scope_: wrapped_scope_,
                };
                assembler.scratch_register_scope_ = Some(Box::new(scope));

                let scope_ptr = assembler.scratch_register_scope_.as_mut().unwrap() as *mut Box<ScratchRegisterScope<'a>>;
                unsafe { &mut (**scope_ptr) }
                scope
            }

            pub fn acquire_scratch(&mut self) -> Register {
                self.wrapped_scope_.acquire()
            }
        }

        impl<'a> Drop for ScratchRegisterScope<'a> {
            fn drop(&mut self) {
                self.assembler_.scratch_register_scope_ = self.prev_scope_.take().map(|x| Box::new(*x));
            }
        }

        mod detail {
            use crate::baseline::baseline_assembler::BaselineAssembler;
            use crate::Register;
            use crate::MemOperand;

            #[cfg(debug_assertions)]
            pub fn clobbers(target: Register, op: MemOperand) -> bool {
                op.rn() == target || op.rm() == target
            }
        }

        pub fn register_frame_operand(
            &mut self,
            interpreter_register: interpreter::Register,
        ) -> MemOperand {
            MemOperand::new(
                Register::fp,
                interpreter_register.to_operand() * crate::kSystemPointerSize,
            )
        }

        pub fn register_frame_address(
            &mut self,
            interpreter_register: interpreter::Register,
            rscratch: Register,
        ) {
            self.masm_mut().add(
                rscratch,
                Register::fp,
                Operand::new(interpreter_register.to_operand() * crate::kSystemPointerSize),
            );
        }

        pub fn feedback_vector_operand(&self) -> MemOperand {
            MemOperand::new(
                Register::fp,
                crate::baseline::baseline_assembler::BaselineFrameConstants::kFeedbackVectorFromFp,
            )
        }

        pub fn feedback_cell_operand(&self) -> MemOperand {
            MemOperand::new(
                Register::fp,
                crate::baseline::baseline_assembler::BaselineFrameConstants::kFeedbackCellFromFp,
            )
        }

        pub fn bind(&mut self, label: &mut Label) {
            self.masm_mut().bind(label);
        }

        pub fn jump_target(&self) {
            // NOP on arm.
        }

        pub fn jump(&mut self, target: &mut Label, _distance: Label::Distance) {
            self.masm_mut().b(target);
        }

        pub fn jump_if_root(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_mut().jump_if_root(value, index, target);
        }

        pub fn jump_if_not_root(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_mut().jump_if_not_root(value, index, target);
        }

        pub fn jump_if_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_mut().jump_if_smi(value, target);
        }

        pub fn jump_if_immediate(
            &mut self,
            cc: Condition,
            left: Register,
            right: i32,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            self.jump(cc, left, Operand::new(right), target, distance);
        }

        pub fn jump_if_not_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_mut().jump_if_not_smi(value, target);
        }

        pub fn test_and_branch(
            &mut self,
            value: Register,
            mask: i32,
            cc: Condition,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_mut().tst(value, Operand::new(mask));
            self.masm_mut().b(cc, target);
        }

        pub fn jump(&mut self, cc: Condition, lhs: Register, rhs: Operand, target: &mut Label, _distance: Label::Distance) {
            self.masm_mut().cmp(lhs, rhs);
            self.masm_mut().b(cc, target);
        }

        pub fn jump_if_object_type_fast(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.jump_if_object_type(cc, object, instance_type, scratch, target, distance);
        }

        pub fn jump_if_object_type(
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
            self.masm_mut().load_map(map, object);
            self.masm_mut().ldrh(
                type_reg,
                MemOperand::new(map, Map::kInstanceTypeOffset as i32),
            );
            self.jump(cc, type_reg, Operand::new(instance_type as i32), target);
        }

        pub fn jump_if_instance_type(
            &mut self,
            cc: Condition,
            map: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            if cfg!(debug_assertions) {
                self.masm_mut().assert_not_smi(map);
                self.masm_mut().compare_object_type(map, type_reg, type_reg, crate::MAP_TYPE);
                self.masm_mut().assert(
                    Condition::Eq,
                    crate::AbortReason::kUnexpectedValue,
                );
            }
            self.masm_mut().ldrh(
                type_reg,
                MemOperand::new(map, Map::kInstanceTypeOffset as i32),
            );
            self.jump(cc, type_reg, Operand::new(instance_type as i32), target);
        }

        pub fn jump_if_pointer(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_mut().ldr(tmp, operand);
            self.jump(cc, value, Operand::new(tmp), target);
        }

        pub fn jump_if_smi_smi(
            &mut self,
            cc: Condition,
            value: Register,
            smi: Tagged<Smi>,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_mut().assert_smi(value);
            self.jump(cc, value, Operand::new(smi.ptr()), target);
        }

        pub fn jump_if_smi_reg(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.masm_mut().assert_smi(lhs);
            self.masm_mut().assert_smi(rhs);
            self.jump(cc, lhs, Operand::new(rhs), target);
        }

        pub fn jump_if_tagged(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_mut().ldr(tmp, operand);
            self.jump(cc, value, Operand::new(tmp), target);
        }

        pub fn jump_if_tagged_mem(
            &mut self,
            cc: Condition,
            operand: MemOperand,
            value: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_mut().ldr(tmp, operand);
            self.jump(cc, tmp, Operand::new(value), target);
        }

        pub fn jump_if_byte(
            &mut self,
            cc: Condition,
            value: Register,
            byte: i32,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.jump(cc, value, Operand::new(byte), target);
        }

        pub fn move_reg_to_reg(
            &mut self,
            output: interpreter::Register,
            source: Register,
        ) {
            self.move_mem_reg(self.register_frame_operand(output), source);
        }

        pub fn move_reg_tagged_index(
            &mut self,
            output: Register,
            value: Tagged<TaggedIndex>,
        ) {
            self.masm_mut().mov(output, Operand::new(value.ptr()));
        }

        pub fn move_mem_reg(&mut self, output: MemOperand, source: Register) {
            self.masm_mut().str(source, output);
        }

        pub fn move_reg_external_reference(
            &mut self,
            output: Register,
            reference: ExternalReference,
        ) {
            self.masm_mut().move32bit_immediate(output, Operand::new(reference));
        }

        pub fn move_reg_heap_object(&mut self, output: Register, value: Handle<HeapObject>) {
            self.masm_mut().move32bit_immediate(output, Operand::new(value));
        }

        pub fn move_reg_i32(&mut self, output: Register, value: i32) {
            self.masm_mut().mov(output, Operand::new(value));
        }

        pub fn move_maybe_smi(&mut self, output: Register, source: Register) {
            self.masm_mut().mov(output, source);
        }

        pub fn move_smi(&mut self, output: Register, source: Register) {
            self.masm_mut().mov(output, source);
        }

        mod detail_impl {
            use crate::baseline::baseline_assembler::BaselineAssembler;
            use crate::baseline::baseline_assembler::BaselineAssembler::ScratchRegisterScope;
            use crate::Register;
            use crate::Tagged;
            use crate::TaggedIndex;

            pub fn to_register_tagged_index(
                basm: &mut BaselineAssembler,
                scope: &mut ScratchRegisterScope,
                arg: Tagged<TaggedIndex>,
            ) -> Register {
                let reg = scope.acquire_scratch();
                basm.move_reg_tagged_index(reg, arg);
                reg
            }

            pub fn to_register_reg(
                _basm: &mut BaselineAssembler,
                _scope: &mut ScratchRegisterScope,
                reg: Register,
            ) -> Register {
                reg
            }
        }

        mod push_all_helper {
            use crate::baseline::baseline_assembler::BaselineAssembler;
            use crate::baseline::baseline_assembler::BaselineAssembler::ScratchRegisterScope;
            use crate::baseline::baseline_assembler::detail_impl;
            use crate::interpreter;
            use crate::Register;
            use crate::Tagged;
            use crate::TaggedIndex;

            pub trait PushAll {
                fn push(&mut self, basm: &mut BaselineAssembler) -> i32;
                fn push_reverse(&mut self, basm: &mut BaselineAssembler) -> i32;
            }

            impl PushAll for () {
                fn push(&mut self, _basm: &mut BaselineAssembler) -> i32 {
                    0
                }
                fn push_reverse(&mut self, _basm: &mut BaselineAssembler) -> i32 {
                    0
                }
            }

            impl PushAll for Register {
                fn push(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let mut scope = ScratchRegisterScope::new(basm);
                    basm.masm_mut().push(*self);
                    1
                }
                fn push_reverse(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    self.push(basm)
                }
            }

            impl PushAll for Tagged<TaggedIndex> {
                fn push(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let mut scope = ScratchRegisterScope::new(basm);
                    basm.masm_mut().push(detail_impl::to_register_tagged_index(basm, &mut scope, *self));
                    1
                }
                fn push_reverse(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    self.push(basm)
                }
            }

            impl<Arg, Args> PushAll for (Arg, Args)
            where
                Arg: PushAll,
                Args: PushAll,
            {
                fn push(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let nargs = self.0.push(basm);
                    nargs + self.1.push(basm)
                }
                fn push_reverse(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let nargs = self.1.push_reverse(basm);
                    nargs + self.0.push(basm)
                }
            }

            impl PushAll for interpreter::RegisterList {
                fn push(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let mut count = 0;
                    for reg_index in 0..self.register_count() {
                        count += interpreter::Register::from_index(reg_index).push(basm);
                    }
                    count
                }
                fn push_reverse(&mut self, basm: &mut BaselineAssembler) -> i32 {
                    let mut count = 0;
                    for reg_index in (0..self.register_count()).rev() {
                        count += interpreter::Register::from_index(reg_index).push(basm);
                    }
                    count
                }
            }
        }

        mod pop_all_helper {
            use crate::baseline::baseline_assembler::BaselineAssembler;
            use crate::Register;

            pub trait PopAll {
                fn pop(&mut self, basm: &mut BaselineAssembler);
            }

            impl PopAll for () {
                fn pop(&mut self, _basm: &mut BaselineAssembler) {}
            }

            impl PopAll for Register {
                fn pop(&mut self, basm: &mut BaselineAssembler) {
                    basm.masm_mut().pop(*self);
                }
            }

            impl<T, Tail> PopAll for (T, Tail)
            where
                T: PopAll,
                Tail: PopAll,
            {
                fn pop(&mut self, basm: &mut BaselineAssembler) {
                    self.0.pop(basm);
                    self.1.pop(basm);
                }
            }
        }

        pub fn push<T: push_all_helper::PushAll>(&mut self, vals: T) -> i32 {
            vals.push(self)
        }

        pub fn push_reverse<T: push_all_helper::PushAll>(&mut self, vals: T) {
            vals.push_reverse(self);
        }

        pub fn pop<T: pop_all_helper::PopAll>(&mut self, registers: T) {
            registers.pop(self);
        }

        pub fn load_tagged_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_mut().ldr(
                output,
                MemOperand::new(source, offset),
            );
        }

        pub fn load_tagged_signed_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_mut().ldr(
                output,
                MemOperand::new(source, offset),
            );
        }

        pub fn load_tagged_signed_field_and_untag(
            &mut self,
            output: Register,
            source: Register,
            offset: i32,
        ) {
            self.load_tagged_signed_field(output, source, offset);
            self.smi_untag(output);
        }

        pub fn load_word16_field_zero_extend(
            &mut self,
            output: Register,
            source: Register,
            offset: i32,
        ) {
            self.masm_mut().ldrh(
                output,
                MemOperand::new(source, offset),
            );
        }

        pub fn load_word8_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_mut().ldrb(
                output,
                MemOperand::new(source, offset),
            );
        }

        pub fn store_tagged_signed_field(
            &mut self,
            target: Register,
            offset: i32,
            value: Tagged<Smi>,
        ) {
            self.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_mut().mov(tmp, Operand::new(value));
            self.masm_mut().str(tmp, MemOperand::new(target, offset));
        }

        pub fn store_tagged_field_with_write_barrier(
            &mut self,
            target: Register,
            offset: i32,
            value: Register,
        ) {
            self.asm_code_comment();
            assert!(!are_aliased(target, value));
            self.masm_mut().str(value, MemOperand::new(target, offset));
            self.record_write_field(
                target,
                offset,
                value,
                crate::kLRHasNotBeenSaved,
                SaveFPRegsMode::kIgnore,
            );
        }

        pub fn store_tagged_field_no_write_barrier(
            &mut self,
            target: Register,
            offset: i32,
            value: Register,
        ) {
            self.masm_mut().str(value, MemOperand::new(target, offset));
        }

        pub fn try_load_optimized_osr_code(
            &mut self,
            scratch_and_result: Register,
            feedback_vector: Register,
            slot: crate::FeedbackSlot,
            on_result: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut fallthrough = Label::new();
            self.load_tagged_field(
                scratch_and_result,
                feedback_vector,
                FeedbackVector::OffsetOfElementAt(slot.to_int()) as i32,
            );
            self.masm_mut().load_weak_value(
                scratch_and_result,
                scratch_and_result,
                &mut fallthrough,
            );

            // Is it marked_for_deoptimization? If yes, clear the slot.
            {
                let mut temps = ScratchRegisterScope::new(self);

                // The entry references a CodeWrapper object. Unwrap it now.
                self.masm_mut().ldr(
                    scratch_and_result,
                    MemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset as i32),
                );

                let scratch = temps.acquire_scratch();
                self.masm_mut().test_code_is_marked_for_deoptimization(scratch_and_result, scratch);
                self.masm_mut().b(Condition::Eq, on_result);
                self.masm_mut().mov(scratch, Operand::new(self.cleared_value()));
                self.store_tagged_field_no_write_barrier(
                    feedback_vector,
                    FeedbackVector::OffsetOfElementAt(slot.to_int()) as i32,
                    scratch,
                );
            }

            self.bind(&mut fallthrough);
            self.move_reg_i32(scratch_and_result, 0);
        }

        pub fn add_to_interrupt_budget_and_jump_if_not_exceeded(
            &mut self,
            weight: i32,
            skip_interrupt_label: &mut Label,
        ) {
            self.asm_code_comment();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_mut().ldr(
                interrupt_budget,
                MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
            );
            // Remember to set flags as part of the add!
            self.masm_mut().add(
                interrupt_budget,
                interrupt_budget,
                Operand::new(weight),
                crate::SetCC::SetCC,
            );
            self.masm_mut().str(
                interrupt_budget,
                MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
            );
            if skip_interrupt_label.is_linked() {
                // Use compare flags set by add
                assert!(weight < 0);
                self.masm_mut().b(Condition::Ge, skip_interrupt_label);
            }
        }

        pub fn add_to_interrupt_budget_and_jump_if_not_exceeded_register(
            &mut self,
            weight: Register,
            skip_interrupt_label: &mut Label,
        ) {
            self.asm_code_comment();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_mut().ldr(
                interrupt_budget,
                MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
            );
            // Remember to set flags as part of the add!
            self.masm_mut().add(
                interrupt_budget,
                interrupt_budget,
                weight,
                crate::SetCC::SetCC,
            );
            self.masm_mut().str(
                interrupt_budget,
                MemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32),
            );
            self.masm_mut().b(Condition::Ge, skip_interrupt_label);
        }

        pub fn lda_context_slot(
            &mut self,
            context: Register,
            index: u32,
            depth: u32,
            _compression_mode: crate::CompressionMode,
        ) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset as i32);
            }
            self.load_tagged_field(
                Register::kInterpreterAccumulatorRegister,
                context,
                Context::OffsetOfElementAt(index) as i32,
            );
        }

        pub fn sta_context_slot(
            &mut self,
            context: Register,
            value: Register,
            index: u32,
            depth: u32,
        ) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset as i32);
            }
            self.store_tagged_field_with_write_barrier(
                context,
                Context::OffsetOfElementAt(index) as i32,
                value,
            );
        }

        pub fn lda_module_variable(
            &mut self,
            context: Register,
            cell_index: i32,
            depth: u32,
        ) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset as i32);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset as i32);
            if cell_index > 0 {
                self.load_tagged_field(
                    context,
                    context,
                    SourceTextModule::kRegularExportsOffset as i32,
                );
                // The actual array index is (cell_index - 1).
                let cell_index = cell_index - 1;
                self.load_fixed_array_element(context, context, cell_index as usize);
                self.load_tagged_field(
                    Register::kInterpreterAccumulatorRegister,
                    context,
                    Cell::kValueOffset as i32,
                );
            } else {
                self.load_tagged_field(
                    context,
                    context,
                    SourceTextModule::kRegularImportsOffset as i32,
                );
                // The actual array index is (-cell_index - 1).
                let cell_index = -cell_index - 1;
                self.load_fixed_array_element(context, context, cell_index as usize);
                self.load_tagged_field(
                    Register::kInterpreterAccumulatorRegister,
                    context,
                    Cell::kValueOffset as i32,
                );
            }
        }

        pub fn sta_module_variable(
            &mut self,
            context: Register,
            value: Register,
            cell_index: i32,
            depth: u32,
        ) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset as i32);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset as i32);
            self.load_tagged_field(
                context,
                context,
                SourceTextModule::kRegularExportsOffset as i32,
            );

            // The actual array index is (cell_index - 1).
            let cell_index = cell_index - 1;
            self.load_fixed_array_element(context, context, cell_index as usize);
            self.store_tagged_field_with_write_barrier(context, Cell::kValueOffset as i32, value);
        }

        pub fn increment_smi(&mut self, lhs: MemOperand) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_mut().ldr(tmp, lhs);
            self.masm_mut().add(tmp, tmp, Operand::new(Smi::from_int(1)));
            self.masm_mut().str(tmp, lhs);
        }

        pub fn word32_and(&mut self, output: Register, lhs: Register, rhs: i32) {
            self.masm_mut().and_(output, lhs, Operand::new(rhs));
        }

        pub fn switch_instr(
            &mut self,
            reg: Register,
            case_value_base: i32,
            labels: &mut [&mut Label],
            num_labels: usize,
        ) {
            self.masm_mut().switch_instr(
                Register::no_reg(),
                reg,
                case_value_base,
                labels,
                num_labels,
            );
        }
    }

    impl BaselineAssembler {
        pub fn emit_return(masm: &mut Assembler) {
            let mut basm = BaselineAssembler::new(masm);

            let weight = BaselineLeaveFrameDescriptor::WeightRegister();
            let params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();

            {
                basm.asm_code_comment_string("Update Interrupt Budget");

                let mut skip_interrupt_label = Label::new();
                basm.add_to_interrupt_budget_and_jump_if_not_exceeded(
                    BaselineLeaveFrameDescriptor::WeightRegister() as i32,
                    &mut skip_interrupt_label,
                );
                {
                    basm.masm_mut().smi_tag(params_size);
                    basm.push((params_size, Register::kInterpreterAccumulatorRegister));

                    basm.load_context(Register::kContextRegister);
                    basm.load_function(Register::kJSFunctionRegister);
                    basm.push(Register::kJSFunctionRegister);
                    basm.call_runtime(crate::Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

                    basm.pop((Register::kInterpreterAccumulatorRegister, params_size));
                    basm.masm_mut().smi_untag(params_size);
                }

                basm.bind(&mut skip_interrupt_label);
            }

            let mut temps = ScratchRegisterScope::new(&mut basm);
            let actual_params_size = temps.acquire_scratch();

            // Compute the size of the actual parameters + receiver.
            basm.move_reg_i32(actual_params_size,
                              masm.load_constant(StandardFrameConstants::kArgCOffset as i32));

            // If actual is bigger than formal, then we should use it to free up the stack
            // arguments.
            masm.cmp(params_size, actual_params_size);
            masm.mov(params_size, actual_params_size, Condition::Lt, None);

            // Leave the frame (also dropping the register file).
            masm.leave_frame(StackFrame::BASELINE);

            // Drop receiver + arguments.
            masm.drop_arguments(params_size);
            masm.ret();
        }
    }

    impl EnsureAccumulatorPreservedScope {
        pub fn assert_equal_to_accumulator(&self, reg: Register) {
            self.assembler_.masm().cmp(reg, Register::kInterpreterAccumulatorRegister);
            self.assembler_.masm().assert(
                Condition::Eq,
                crate::AbortReason::kAccumulatorClobbered,
            );
        }
    }

    #[derive(Debug)]
    pub struct Operand {
        imm: i32,
    }

    impl Operand {
        pub fn new(imm: i32) -> Self {
            Operand { imm }
        }
    }

    #[derive(Debug)]
    pub struct MemOperand {
        rn: Register,
        imm: i32,
        rm: Register,
    }

    impl MemOperand {
        pub fn new(rn: Register, imm: i32) -> Self {
            MemOperand { rn, imm, rm: Register::no_reg() }
        
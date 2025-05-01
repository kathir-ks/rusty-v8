// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod baseline_assembler_ppc_inl {
    use crate::baseline::baseline_assembler::*;
    use crate::codegen::interface_descriptors::*;
    use crate::codegen::ppc::assembler_ppc::*;
    use crate::codegen::ppc::register_ppc::*;
    use crate::objects::literal_objects::*;
    use crate::interpreter;
    use std::mem;

    pub mod detail {
        use super::*;

        pub const K_SCRATCH_REGISTERS: [Register; 3] = [r9, r10, ip];
        pub const K_NUM_SCRATCH_REGISTERS: usize = K_SCRATCH_REGISTERS.len();

        #[cfg(debug_assertions)]
        pub fn clobbers(target: Register, op: MemOperand) -> bool {
            op.rb() == target || op.ra() == target
        }
    }

    pub struct ScratchRegisterScope<'a> {
        assembler_: &'a mut BaselineAssembler<'a>,
        prev_scope_: Option<&'a ScratchRegisterScope<'a>>,
        registers_used_: usize,
    }

    impl<'a> ScratchRegisterScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler<'a>) -> Self {
            let prev_scope_ = assembler.scratch_register_scope_.take();
            let registers_used_ = match &prev_scope_ {
                Some(scope) => scope.registers_used_,
                None => 0,
            };

            let mut scope = ScratchRegisterScope {
                assembler_: assembler,
                prev_scope_: prev_scope_,
                registers_used_: registers_used_,
            };
            assembler.scratch_register_scope_ = Some(&mut scope);
            scope
        }

        pub fn acquire_scratch(&mut self) -> Register {
            debug_assert!(self.registers_used_ < detail::K_NUM_SCRATCH_REGISTERS);
            let reg = detail::K_SCRATCH_REGISTERS[self.registers_used_];
            self.registers_used_ += 1;
            reg
        }
    }

    impl<'a> Drop for ScratchRegisterScope<'a> {
        fn drop(&mut self) {
            self.assembler_.scratch_register_scope_ = self.prev_scope_.take();
        }
    }
    
    pub fn jump_if_helper(assm: &mut MacroAssembler, cc: Condition, lhs: Register, rhs: Register, target: &mut Label) {
        if mem::size_of::<usize>() == 8 {
            if is_signed(cc) {
                assm.cmps64(lhs, rhs);
            } else {
                assm.cmpu64(lhs, rhs);
            }
        } else {
            if is_signed(cc) {
                assm.cmps32(lhs, rhs);
            } else {
                assm.cmpu32(lhs, rhs);
            }
        }
        assm.b(to_condition(cc), target);
    }

    impl<'a> BaselineAssembler<'a> {
        pub fn register_frame_operand(&mut self, interpreter_register: interpreter::Register) -> MemOperand {
            MemOperand::new(fp, interpreter_register.to_operand() * kSystemPointerSize)
        }

        pub fn register_frame_address(&mut self, interpreter_register: interpreter::Register, rscratch: Register) {
            self.masm_.adds64(
                rscratch,
                fp,
                Operand::new(interpreter_register.to_operand() * kSystemPointerSize),
            );
        }

        pub fn feedback_vector_operand(&mut self) -> MemOperand {
            MemOperand::new(fp, BaselineFrameConstants::kFeedbackVectorFromFp)
        }

        pub fn feedback_cell_operand(&mut self) -> MemOperand {
            MemOperand::new(fp, BaselineFrameConstants::kFeedbackCellFromFp)
        }

        pub fn bind(&mut self, label: &mut Label) {
            self.masm_.bind(label);
        }

        pub fn jump_target(&mut self) {
            // NOP on arm.
        }

        pub fn jump(&mut self, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.b(target);
        }

        pub fn jump_if_root(&mut self, value: Register, index: RootIndex, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.jump_if_root(value, index, target);
        }

        pub fn jump_if_not_root(&mut self, value: Register, index: RootIndex, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.jump_if_not_root(value, index, target);
        }

        pub fn jump_if_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.jump_if_smi(value, target);
        }

        pub fn jump_if_immediate(&mut self, cc: Condition, left: Register, right: i32, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.jump_if(cc, left, Operand::new(right), target, _distance);
        }

        pub fn jump_if_not_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.jump_if_not_smi(value, target);
        }

        pub fn test_and_branch(&mut self, value: Register, mask: i32, cc: Condition, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.and_u64(r0, value, Operand::new(mask), ip, SetRC);
            self.masm_.b(to_condition(cc), target, cr0);
        }

        pub fn jump_if(&mut self, cc: Condition, lhs: Register, rhs: Operand, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            if is_signed(cc) {
                self.masm_.cmps64(lhs, rhs, r0);
            } else {
                self.masm_.cmpu64(lhs, rhs, r0);
            }
            self.masm_.b(to_condition(cc), target);
        }

        #[cfg(V8_STATIC_ROOTS_BOOL)]
        pub fn jump_if_js_any_is_primitive(&mut self, heap_object: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.assert_not_smi(heap_object);
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.masm_.jump_if_js_any_is_primitive(heap_object, scratch, target, _distance);
        }

        pub fn jump_if_object_type_fast(&mut self, cc: Condition, object: Register, instance_type: InstanceType, target: &mut Label, _distance: Label::Distance) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            if cc == eq || cc == ne {
                let scratch2 = temps.acquire_scratch();
                self.masm_.is_object_type(object, scratch, scratch2, instance_type);
                self.masm_.b(to_condition(cc), target);
                return;
            }
            self.jump_if_object_type(cc, object, instance_type, scratch, target, _distance);
        }

        pub fn jump_if_object_type(&mut self, cc: Condition, object: Register, instance_type: InstanceType, map: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            self.masm_.load_map(map, object);
            self.masm_.loadu16(type_reg, FieldMemOperand::new(map, Map::kInstanceTypeOffset as i32), r0);
            self.jump_if(cc, type_reg, Operand::new(instance_type as i32), target);
        }

        pub fn jump_if_instance_type(&mut self, cc: Condition, map: Register, instance_type: InstanceType, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            if v8_flags::debug_code {
                self.masm_.assert_not_smi(map);
                self.masm_.compare_object_type(map, type_reg, type_reg, MAP_TYPE);
                self.masm_.assert(eq, AbortReason::kUnexpectedValue);
            }
            self.masm_.loadu16(type_reg, FieldMemOperand::new(map, Map::kInstanceTypeOffset as i32), r0);
            self.jump_if(cc, type_reg, Operand::new(instance_type as i32), target);
        }

        pub fn jump_if_pointer(&mut self, cc: Condition, value: Register, operand: MemOperand, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_.loadu64(tmp, operand, r0);
            jump_if_helper(self.masm_, cc, value, tmp, target);
        }

        pub fn jump_if_smi_tagged(&mut self, cc: Condition, value: Register, smi: Tagged<Smi>, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.assert_smi(value);
            self.masm_.load_smi_literal(r0, smi);
            jump_if_helper(self.masm_, cc, value, r0, target);
        }

        pub fn jump_if_smi_registers(&mut self, cc: Condition, lhs: Register, rhs: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.assert_smi(lhs);
            self.masm_.assert_smi(rhs);
            jump_if_helper(self.masm_, cc, lhs, rhs, target);
        }

        pub fn jump_if_tagged(&mut self, cc: Condition, value: Register, operand: MemOperand, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.load_tagged_field(ip, operand, r0);
            jump_if_helper::<{ if COMPRESS_POINTERS_BOOL { 32 } else { 64 } }>(self.masm_, cc, value, ip, target);
        }

        pub fn jump_if_tagged_mem_reg(&mut self, cc: Condition, operand: MemOperand, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.masm_.load_tagged_field(ip, operand, r0);
            jump_if_helper::<{ if COMPRESS_POINTERS_BOOL { 32 } else { 64 } }>(self.masm_, cc, value, ip, target);
        }

        pub fn jump_if_byte(&mut self, cc: Condition, value: Register, byte: i32, target: &mut Label, _distance: Label::Distance) {
            self.masm_.asm_code_comment();
            self.jump_if(cc, value, Operand::new(byte), target);
        }

        pub fn move_register(&mut self, output: interpreter::Register, source: Register) {
            self.masm_.asm_code_comment();
            self.move_mem_reg(self.register_frame_operand(output), source);
        }

        pub fn move_reg_tagged_index(&mut self, output: Register, value: Tagged<TaggedIndex>) {
            self.masm_.asm_code_comment();
            self.masm_.mov(output, Operand::new(value.ptr() as i64));
        }

        pub fn move_mem_reg(&mut self, output: MemOperand, source: Register) {
            self.masm_.asm_code_comment();
            self.masm_.storeu64(source, output, r0);
        }

        pub fn move_reg_external_reference(&mut self, output: Register, reference: ExternalReference) {
            self.masm_.asm_code_comment();
            self.masm_.move_reg_external_reference(output, reference);
        }

        pub fn move_reg_handle(&mut self, output: Register, value: Handle<HeapObject>) {
            self.masm_.asm_code_comment();
            self.masm_.move_reg_handle(output, value);
        }

        pub fn move_reg_i32(&mut self, output: Register, value: i32) {
            self.masm_.asm_code_comment();
            self.masm_.mov(output, Operand::new(value as i64));
        }

        pub fn move_maybe_smi(&mut self, output: Register, source: Register) {
            self.masm_.asm_code_comment();
            self.masm_.mr(output, source);
        }

        pub fn move_smi(&mut self, output: Register, source: Register) {
            self.masm_.asm_code_comment();
            self.masm_.mr(output, source);
        }

        pub fn load_tagged_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.asm_code_comment();
            self.masm_.load_tagged_field(output, FieldMemOperand::new(source, offset), r0);
        }

        pub fn load_tagged_signed_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.asm_code_comment();
            self.masm_.load_tagged_signed_field(output, FieldMemOperand::new(source, offset), r0);
        }

        pub fn load_tagged_signed_field_and_untag(&mut self, output: Register, source: Register, offset: i32) {
            self.load_tagged_signed_field(output, source, offset);
            self.smi_untag(output);
        }

        pub fn load_word16_field_zero_extend(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.asm_code_comment();
            self.masm_.loadu16(output, FieldMemOperand::new(source, offset), r0);
        }

        pub fn load_word8_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.asm_code_comment();
            self.masm_.loadu8(output, FieldMemOperand::new(source, offset), r0);
        }

        pub fn store_tagged_signed_field(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
            self.masm_.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_.load_smi_literal(tmp, value);
            self.masm_.store_tagged_field(tmp, FieldMemOperand::new(target, offset), r0);
        }

        pub fn store_tagged_field_with_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_.asm_code_comment();
            let scratch = WriteBarrierDescriptor::slot_address_register();
            debug_assert!(!are_aliased(target, value, scratch));
            self.masm_.store_tagged_field(value, FieldMemOperand::new(target, offset), r0);
            self.masm_.record_write_field(target, offset, value, scratch, kLRHasNotBeenSaved, SaveFPRegsMode::kIgnore);
        }

        pub fn store_tagged_field_no_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_.asm_code_comment();
            self.masm_.store_tagged_field(value, FieldMemOperand::new(target, offset), r0);
        }

        pub fn try_load_optimized_osr_code(&mut self, scratch_and_result: Register, feedback_vector: Register, slot: FeedbackSlot, on_result: &mut Label, _distance: Label::Distance) {
            let mut fallthrough = Label::new();

            self.load_tagged_field(scratch_and_result, feedback_vector,
                                    FeedbackVector::offset_of_element_at(slot.to_int()));
            self.masm_.load_weak_value(scratch_and_result, scratch_and_result, &mut fallthrough);

            // Is it marked_for_deoptimization? If yes, clear the slot.
            {
                let mut temps = ScratchRegisterScope::new(self);

                // The entry references a CodeWrapper object. Unwrap it now.
                self.masm_.load_code_pointer_field(
                    scratch_and_result,
                    FieldMemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset as i32), r0);

                let scratch = temps.acquire_scratch();
                self.masm_.test_code_is_marked_for_deoptimization(scratch_and_result, scratch, r0);
                self.masm_.beq(on_result, cr0);
                self.masm_.mov(scratch, self.masm_.cleared_value());
                self.store_tagged_field_no_write_barrier(
                    feedback_vector, FeedbackVector::offset_of_element_at(slot.to_int()),
                    scratch);
            }

            self.bind(&mut fallthrough);
            self.move_reg_i32(scratch_and_result, 0);
        }

        pub fn add_to_interrupt_budget_and_jump_if_not_exceeded(&mut self, weight: i32, skip_interrupt_label: &mut Label) {
            self.masm_.asm_code_comment();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_.loadu32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32), r0);
            // Remember to set flags as part of the add!
            self.masm_.adds32(interrupt_budget, interrupt_budget, Operand::new(weight as i64), r0, SetRC);
            self.masm_.storeu32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32), r0);
            if skip_interrupt_label.is_unused() {
                // Use compare flags set by add
                debug_assert!(weight < 0);
                self.masm_.bge(skip_interrupt_label, cr0);
            }
        }

        pub fn add_to_interrupt_budget_and_jump_if_not_exceeded_register(&mut self, weight: Register, skip_interrupt_label: &mut Label) {
            self.masm_.asm_code_comment();
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.acquire_scratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.acquire_scratch();
            self.masm_.loadu32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32), r0);
            // Remember to set flags as part of the add!
            self.masm_.adds32(interrupt_budget, interrupt_budget, weight, SetRC);
            self.masm_.storeu32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset as i32), r0);
            if skip_interrupt_label.is_unused() {
              self.masm_.bge(skip_interrupt_label, cr0);
            }
        }

        pub fn lda_context_slot(&mut self, context: Register, index: u32, depth: u32, _compression_mode: CompressionMode) {
            self.masm_.asm_code_comment();
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(kInterpreterAccumulatorRegister, context,
                                    Context::offset_of_element_at(index));
        }

        pub fn sta_context_slot(&mut self, context: Register, value: Register, index: u32, depth: u32) {
            self.masm_.asm_code_comment();
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.store_tagged_field_with_write_barrier(context, Context::offset_of_element_at(index),
                                                        value);
        }

        pub fn lda_module_variable(&mut self, context: Register, cell_index: i32, depth: u32) {
            self.masm_.asm_code_comment();
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset);
            if cell_index > 0 {
                self.load_tagged_field(context, context, SourceTextModule::kRegularExportsOffset);
                // The actual array index is (cell_index - 1).
                //cell_index -= 1;
                self.load_fixed_array_element(context, context, cell_index - 1);
            } else {
                self.load_tagged_field(context, context, SourceTextModule::kRegularImportsOffset);
                // The actual array index is (-cell_index - 1).
                //cell_index = -cell_index - 1;
                self.load_fixed_array_element(context, context, -cell_index - 1);
            }
            self.load_tagged_field(kInterpreterAccumulatorRegister, context, Cell::kValueOffset);
        }

        pub fn sta_module_variable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
            self.masm_.asm_code_comment();
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset);
            self.load_tagged_field(context, context, SourceTextModule::kRegularExportsOffset);

            // The actual array index is (cell_index - 1).
            //cell_index -= 1;
            self.load_fixed_array_element(context, context, cell_index - 1);
            self.store_tagged_field_with_write_barrier(context, Cell::kValueOffset, value);
        }

        pub fn increment_smi(&mut self, lhs: MemOperand) {
            let scratch = ip;
            if smi_values_are_31_bits() {
                self.masm_.load_s32(scratch, lhs, r0);
                self.masm_.adds64(scratch, scratch, Operand::new(Smi::from_int(1).value() as i64));
                self.masm_.storeu32(scratch, lhs, r0);
            } else {
                self.masm_.smi_untag(scratch, lhs, LeaveRC, r0);
                self.masm_.adds64(scratch, scratch, Operand::new(1));
                self.masm_.smi_tag(scratch);
                self.masm_.storeu64(scratch, lhs, r0);
            }
        }

        pub fn switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: i32) {
            self.masm_.asm_code_comment();
            let mut fallthrough = Label::new();
            let mut jump_table = Label::new();
            if case_value_base != 0 {
                self.masm_.adds64(reg, reg, Operand::new(-case_value_base as i64));
            }

            // Mostly copied from code-generator-arm.cc
            self.jump_if(kUnsignedGreaterThanEqual, reg, Operand::new(num_labels as i64), &mut fallthrough, Label::Distance::Near);
            // Ensure to emit the constant pool first if necessary.
            let entry_size_log2 = 3;
            self.masm_.shift_left_u32(reg, reg, Operand::new(entry_size_log2));
            self.masm_.mov_label_addr(ip, &mut jump_table);
            self.masm_.adds64(reg, reg, ip);
            self.masm_.jump(reg, Label::Distance::Near);
            self.masm_.b(&mut fallthrough);
            self.masm_.bind(&mut jump_table);
            let block_trampoline_pool = Assembler::block_trampoline_pool_scope(self.masm_);
            for i in 0..num_labels as usize {
                self.masm_.b(labels[i]);
                self.masm_.nop();
            }
            self.masm_.bind(&mut fallthrough);
        }

        pub fn word32_and(&mut self, output: Register, lhs: Register, rhs: i32) {
            self.masm_.and_u32(output, lhs, Operand::new(rhs as i64));
        }
    }

    impl<'a> EnsureAccumulatorPreservedScope<'a> {
        pub fn assert_equal_to_accumulator(&self, reg: Register) {
            if COMPRESS_POINTERS_BOOL {
                self.assembler_.masm().cmpu32(reg, kInterpreterAccumulatorRegister);
            } else {
                self.assembler_.masm().cmpu64(reg, kInterpreterAccumulatorRegister);
            }
            self.assembler_.masm().assert(eq, AbortReason::kAccumulatorClobbered);
        }
    }

    pub fn emit_return(masm: &mut MacroAssembler) {
        masm.asm_code_comment();
        let mut basm = BaselineAssembler::new(masm);

        let weight = BaselineLeaveFrameDescriptor::weight_register();
        let params_size = BaselineLeaveFrameDescriptor::params_size_register();

        {
            masm.asm_code_comment_string("Update Interrupt Budget");

            let mut skip_interrupt_label = Label::new();
            basm.add_to_interrupt_budget_and_jump_if_not_exceeded(weight as i32, &mut skip_interrupt_label);
            {
                masm.smi_tag(params_size);
                basm.push(params_size, kInterpreterAccumulatorRegister);

                masm.load_context(kContextRegister);
                masm.load_function(kJSFunctionRegister);
                basm.push(kJSFunctionRegister);
                masm.call_runtime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

                basm.pop(kInterpreterAccumulatorRegister, params_size);
                masm.smi_untag(params_size);
            }

            basm.bind(&mut skip_interrupt_label);
        }

        let mut temps = ScratchRegisterScope::new(&mut basm);
        let actual_params_size = temps.acquire_scratch();
        // Compute the size of the actual parameters + receiver.
        basm.move_reg_i32(actual_params_size,
                        MemOperand::new(fp, StandardFrameConstants::kArgCOffset).offset() as i32);

        // If actual is bigger than formal, then we should use it to free up the stack
        // arguments.
        let mut corrected_args_count = Label::new();
        jump_if_helper(basm.masm(), kGreaterThanEqual, params_size, actual_params_size, &mut corrected_args_count);
        masm.mr(params_size, actual_params_size);
        basm.bind(&mut corrected_args_count);

        // Leave the frame (also dropping the register file).
        masm.leave_frame(StackFrame::BASELINE);

        // Drop receiver + arguments.
        masm.drop_arguments(params_size);
        masm.ret();
    }
}
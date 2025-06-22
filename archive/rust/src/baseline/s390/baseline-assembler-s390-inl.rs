// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod baseline_assembler_s390 {
    use crate::baseline::baseline_assembler::*;
    use crate::codegen::interface_descriptors::*;
    use crate::codegen::s390::assembler_s390::*;
    use crate::objects::literal_objects::*;
    use crate::objects::map::Map;
    use crate::objects::feedback_vector::FeedbackVector;
    use crate::objects::code_wrapper::CodeWrapper;
    use crate::objects::cell::Cell;
    use crate::objects::source_text_module::SourceTextModule;
    use crate::interpreter::register::*;
    use crate::codegen::register::Register;
    use crate::codegen::macro_assembler::Label;
    use crate::codegen::macro_assembler::Operand;
    use crate::codegen::macro_assembler::Condition;
    use crate::codegen::macro_assembler::MemOperand;
    use crate::codegen::code_factory::StackFrame;
    use crate::codegen::macro_assembler::AbortReason;
    use crate::roots::roots::RootIndex;
    use crate::isolate::isolate::CompressionMode;
    use crate::tagged::tagged_impl::Tagged;
    use crate::smi::smi::Smi;
    use crate::codegen::macro_assembler::SaveFPRegsMode;
    use crate::runtime::runtime::Runtime;
    use std::mem::size_of;

    pub mod detail {
        use crate::codegen::register::Register;

        pub const K_SCRATCH_REGISTERS: [Register; 3] = [
            Register(8),  // r8
            Register(13), // ip
            Register(1),  // r1
        ];
        pub const K_NUM_SCRATCH_REGISTERS: usize = K_SCRATCH_REGISTERS.len();

        // #[cfg(debug_assertions)] // Equivalent of #ifdef DEBUG
        // pub fn clobbers(target: Register, op: &MemOperand) -> bool {
        //     op.rb() == target || op.rx() == target
        // }
    }

    pub struct ScratchRegisterScope<'a> {
        assembler_: &'a mut BaselineAssembler,
        prev_scope_: Option<Box<ScratchRegisterScope<'a>>>,
        registers_used_: usize,
    }

    impl<'a> ScratchRegisterScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
            let prev_scope_ = assembler.scratch_register_scope_.take();
            let registers_used_ = match &prev_scope_ {
                Some(scope) => scope.registers_used_,
                None => 0,
            };
            let mut scope = Self {
                assembler_: assembler,
                prev_scope_: prev_scope_,
                registers_used_: registers_used_,
            };
            assembler.scratch_register_scope_ = Some(Box::new(scope));
            let raw_ptr = assembler.scratch_register_scope_.as_mut().map(|x| &mut **x).unwrap() as *mut ScratchRegisterScope;
            unsafe {
                let assembler_mut = &mut *(assembler as *mut BaselineAssembler);
                assembler_mut.scratch_register_scope_raw = Some(raw_ptr);

            }


            let scratch_register_scope_ptr = assembler.scratch_register_scope_raw.unwrap() as *mut ScratchRegisterScope<'a>;
            let scratch_register_scope = unsafe { &mut *scratch_register_scope_ptr };

            scratch_register_scope
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
            if let Some(scope_ptr) = self.assembler_.scratch_register_scope_raw {
                unsafe {
                    let _ = *scope_ptr;
                }
                self.assembler_.scratch_register_scope_raw = None;

            }
        }
    }

    // Helper function equivalent to JumpIfHelper in C++
    fn jump_if_helper(
        assm: &mut MacroAssembler,
        cc: Condition,
        lhs: Register,
        rhs: Register,
        target: &mut Label,
        width: usize,
    ) {
        assert!(width == 64 || width == 32, "only support 64 and 32 bit compare");
        if width == 64 {
            if is_signed(cc) {
                assm.cmp_s64(lhs, rhs);
            } else {
                assm.cmp_u64(lhs, rhs);
            }
        } else {
            if is_signed(cc) {
                assm.cmp_s32(lhs, rhs);
            } else {
                assm.cmp_u32(lhs, rhs);
            }
        }
        assm.b(to_condition(cc), target);
    }

    impl BaselineAssembler {
        pub fn register_frame_operand(
            &mut self,
            interpreter_register: interpreter::Register,
        ) -> MemOperand {
            MemOperand::new(
                Register(15), // fp
                interpreter_register.to_operand() * size_of::<usize>(),
            )
        }

        pub fn register_frame_address(
            &mut self,
            interpreter_register: interpreter::Register,
            rscratch: Register,
        ) {
            self.masm_().add_s64(
                rscratch,
                Register(15), // fp
                interpreter_register.to_operand() * size_of::<usize>(),
            );
        }

        pub fn feedback_vector_operand(&mut self) -> MemOperand {
            MemOperand::new(Register(15), BaselineFrameConstants::K_FEEDBACK_VECTOR_FROM_FP)
        }

        pub fn feedback_cell_operand(&mut self) -> MemOperand {
            MemOperand::new(Register(15), BaselineFrameConstants::K_FEEDBACK_CELL_FROM_FP)
        }

        pub fn bind(&mut self, label: &mut Label) {
            self.masm_().bind(label);
        }

        pub fn jump_target(&mut self) {
            // NOP on arm.
        }

        pub fn jump(&mut self, target: &mut Label, _distance: Label::Distance) {
            self.asm_code_comment();
            self.masm_().b(target);
        }

        pub fn jump_if_root(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.masm_().jump_if_root(value, index, target);
        }

        pub fn jump_if_not_root(
            &mut self,
            value: Register,
            index: RootIndex,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.masm_().jump_if_not_root(value, index, target);
        }

        pub fn jump_if_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.asm_code_comment();
            self.masm_().jump_if_smi(value, target);
        }

        pub fn jump_if_immediate(
            &mut self,
            cc: Condition,
            left: Register,
            right: i32,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.jump(cc, left, Operand::Imm(right), target, _distance);
        }

        pub fn jump_if_not_smi(&mut self, value: Register, target: &mut Label, _distance: Label::Distance) {
            self.asm_code_comment();
            self.masm_().jump_if_not_smi(value, target);
        }

        pub fn test_and_branch(
            &mut self,
            value: Register,
            mask: i32,
            cc: Condition,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.masm_().and_p(Register(0), value, Operand::Imm(mask));
            self.masm_().b(to_condition(cc), target);
        }

        pub fn jump(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Operand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            if is_signed(cc) {
                self.masm_().cmp_s64(lhs, rhs);
            } else {
                self.masm_().cmp_u64(lhs, rhs);
            }
            self.masm_().b(to_condition(cc), target);
        }

        pub fn jump_if_object_type_fast(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.jump_if_object_type(cc, object, instance_type, scratch, target, _distance);
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
            self.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            self.masm_().load_map(map, object);
            self.masm_().load_u16(type_reg, FieldMemOperand::new(map, Map::K_INSTANCE_TYPE_OFFSET));
            self.jump(cc, type_reg, Operand::Imm(instance_type as i32), target);
        }

        pub fn jump_if_instance_type(
            &mut self,
            cc: Condition,
            map: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.acquire_scratch();
            if v8_flags::debug_code {
                self.masm_().assert_not_smi(map);
                self.masm_().compare_object_type(map, type_reg, type_reg, Map::K_MAP_TYPE);
                self.masm_().assert(Condition::kEqual, AbortReason::K_UNEXPECTED_VALUE);
            }
            self.masm_().load_u16(type_reg, FieldMemOperand::new(map, Map::K_INSTANCE_TYPE_OFFSET));
            self.jump(cc, type_reg, Operand::Imm(instance_type as i32), target);
        }

        pub fn jump_if_pointer(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_().load_u64(tmp, operand);
            jump_if_helper(self.masm_(), cc, value, tmp, target, 64);
        }

        pub fn jump_if_smi_smi(
            &mut self,
            cc: Condition,
            smi: Register,
            other_smi: Tagged<Smi>,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.masm_().assert_smi(smi);
            self.masm_().load_smi_literal(Register(0), other_smi);
            jump_if_helper(self.masm_(), cc, smi, Register(0), target, 64);
        }

        pub fn jump_if_smi_reg(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.masm_().assert_smi(lhs);
            self.masm_().assert_smi(rhs);
            jump_if_helper(self.masm_(), cc, lhs, rhs, target, 64);
        }

        #[cfg(target_endian = "big")]
        const STACK_BIAS: i32 = 4;
        #[cfg(target_endian = "little")]
        const STACK_BIAS: i32 = 0;

        pub fn jump_if_tagged_mem(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            debug_assert!(operand.rb() == Register(15) || operand.rx() == Register(15));

            if crate::flags::COMPRESS_POINTERS_BOOL {
                let addr = MemOperand::new(
                    operand.rx(),
                    operand.rb(),
                    operand.offset() + Self::STACK_BIAS,
                );
                self.masm_().load_tagged_field(Register(13), addr, Register(0));
                jump_if_helper(self.masm_(), cc, value, Register(13), target, 32);
            } else {
                self.masm_().load_tagged_field(Register(13), operand, Register(0));
                jump_if_helper(self.masm_(), cc, value, Register(13), target, 64);
            }
        }

        pub fn jump_if_tagged_reg(
            &mut self,
            cc: Condition,
            operand: MemOperand,
            value: Register,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            debug_assert!(operand.rb() == Register(15) || operand.rx() == Register(15));

            if crate::flags::COMPRESS_POINTERS_BOOL {
                let addr = MemOperand::new(
                    operand.rx(),
                    operand.rb(),
                    operand.offset() + Self::STACK_BIAS,
                );
                self.masm_().load_tagged_field(Register(13), addr, Register(0));
                jump_if_helper(self.masm_(), cc, Register(13), value, target, 32);
            } else {
                self.masm_().load_tagged_field(Register(13), operand, Register(0));
                jump_if_helper(self.masm_(), cc, Register(13), value, target, 64);
            }
        }

        pub fn jump_if_byte(
            &mut self,
            cc: Condition,
            value: Register,
            byte: i32,
            target: &mut Label,
            _distance: Label::Distance,
        ) {
            self.asm_code_comment();
            self.jump(cc, value, Operand::Imm(byte), target);
        }

        pub fn move_register_interpreter(&mut self, output: interpreter::Register, source: Register) {
            self.move_mem_register(self.register_frame_operand(output), source);
        }

        pub fn move_register_tagged(&mut self, output: Register, value: Tagged<TaggedIndex>) {
            self.asm_code_comment();
            self.masm_().mov(output, Operand::Imm(value.ptr() as i64));
        }

        pub fn move_mem_register(&mut self, output: MemOperand, source: Register) {
            self.asm_code_comment();
            self.masm_().store_u64(source, output);
        }

        pub fn move_register_external_reference(&mut self, output: Register, reference: ExternalReference) {
            self.asm_code_comment();
            self.masm_().move_external_reference(output, reference);
        }

        pub fn move_register_handle(&mut self, output: Register, value: Handle<HeapObject>) {
            self.asm_code_comment();
            self.masm_().move_handle(output, value);
        }

        pub fn move_register_i32(&mut self, output: Register, value: i32) {
            self.asm_code_comment();
            self.masm_().mov(output, Operand::Imm(value as i64));
        }

        pub fn move_maybe_smi(&mut self, output: Register, source: Register) {
            self.asm_code_comment();
            self.masm_().mov(output, source);
        }

        pub fn move_smi(&mut self, output: Register, source: Register) {
            self.asm_code_comment();
            self.masm_().mov(output, source);
        }

        pub fn load_tagged_field(&mut self, output: Register, source: Register, offset: i32) {
            self.asm_code_comment();
            self.masm_().load_tagged_field(output, FieldMemOperand::new(source, offset), Register(0));
        }

        pub fn load_tagged_signed_field(&mut self, output: Register, source: Register, offset: i32) {
            self.asm_code_comment();
            self.masm_().load_tagged_signed_field(output, FieldMemOperand::new(source, offset));
        }

        pub fn load_tagged_signed_field_and_untag(&mut self, output: Register, source: Register, offset: i32) {
            self.load_tagged_signed_field(output, source, offset);
            self.smi_untag(output);
        }

        pub fn load_word16_field_zero_extend(&mut self, output: Register, source: Register, offset: i32) {
            self.asm_code_comment();
            self.masm_().load_u16(output, FieldMemOperand::new(source, offset));
        }

        pub fn load_word8_field(&mut self, output: Register, source: Register, offset: i32) {
            self.asm_code_comment();
            self.masm_().load_u8(output, FieldMemOperand::new(source, offset));
        }

        pub fn store_tagged_signed_field(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
            self.asm_code_comment();
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.acquire_scratch();
            self.masm_().load_smi_literal(tmp, value);
            self.masm_().store_tagged_field(tmp, FieldMemOperand::new(target, offset), Register(0));
        }

        pub fn store_tagged_field_with_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            self.asm_code_comment();
            let scratch = WriteBarrierDescriptor::slot_address_register();
            debug_assert!(!are_aliased(target, value, scratch));
            self.masm_().store_tagged_field(value, FieldMemOperand::new(target, offset), Register(0));
            self.masm_().record_write_field(target, offset, value, scratch, false, SaveFPRegsMode::K_IGNORE);
        }

        pub fn store_tagged_field_no_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_().store_tagged_field(value, FieldMemOperand::new(target, offset), Register(0));
        }

        pub fn try_load_optimized_osr_code(
            &mut self,
            scratch_and_result: Register,
            feedback_vector: Register,
            slot: FeedbackSlot,
            on_result: &mut Label,
            _distance: Label::Distance,
        ) {
            let mut fallthrough = Label::new();
            self.load_tagged_field(
                scratch_and_result,
                feedback_vector,
                FeedbackVector::offset_of_element_at(slot.to_int()),
            );
            self.masm_().load_weak_value(
                scratch_and_result,
                scratch_and_result,
                &mut fallthrough,
            );

            // Is it marked_for_deoptimization? If yes, clear the slot.
            {
                let mut temps = ScratchRegisterScope::new(self);

                // The entry references a CodeWrapper object. Unwrap it now.
                self.load_tagged_field(
                    scratch_and_result,
                    FieldMemOperand::new(scratch_and_result, CodeWrapper::K_CODE_OFFSET),
                );

                let scratch = temps.acquire_scratch();
                self.masm_().test_code_is_marked_for_deoptimization(scratch_and_result, scratch);
                self.masm_().b(Condition::kEqual, on_result);
                self.masm_().mov(scratch, self.masm_().cleared_value());
                self.store_tagged_field_no_write_barrier(
                    feedback_vector,
                    FeedbackVector::offset_of_element_at(slot.to_int()),
                    scratch,
                );
            }

            self.bind(&mut fallthrough);
            self.move_register_i32(scratch_and_result, 0);
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
            self.masm_().load_u32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::K_INTERRUPT_BUDGET_OFFSET),
            );
            // Remember to set flags as part of the add!
            self.masm_().add_s32(interrupt_budget, Operand::Imm(weight));
            self.masm_().store_u32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::K_INTERRUPT_BUDGET_OFFSET),
                Register(0),
            );
            if skip_interrupt_label.is_unused() {
                // Use compare flags set by add
                debug_assert!(weight < 0);
                self.masm_().b(Condition::kGreaterEqual, skip_interrupt_label);
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
            self.masm_().load_u32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::K_INTERRUPT_BUDGET_OFFSET),
            );
            // Remember to set flags as part of the add!
            self.masm_()
                .add_s32(interrupt_budget, interrupt_budget, weight);
            self.masm_().store_u32(
                interrupt_budget,
                FieldMemOperand::new(feedback_cell, FeedbackCell::K_INTERRUPT_BUDGET_OFFSET),
            );
            if skip_interrupt_label.is_unused() {
                self.masm_().b(Condition::kGreaterEqual, skip_interrupt_label);
            }
        }

        pub fn lda_context_slot(
            &mut self,
            context: Register,
            index: u32,
            depth: u32,
            _compression_mode: CompressionMode,
        ) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::K_PREVIOUS_OFFSET);
            }
            self.load_tagged_field(
                Register(16), // kInterpreterAccumulatorRegister
                context,
                Context::offset_of_element_at(index),
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
                self.load_tagged_field(context, context, Context::K_PREVIOUS_OFFSET);
            }
            self.store_tagged_field_with_write_barrier(
                context,
                Context::offset_of_element_at(index),
                value,
            );
        }

        pub fn lda_module_variable(&mut self, context: Register, cell_index: i32, depth: u32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::K_PREVIOUS_OFFSET);
            }
            self.load_tagged_field(context, context, Context::K_EXTENSION_OFFSET);
            if cell_index > 0 {
                self.load_tagged_field(
                    context,
                    context,
                    SourceTextModule::K_REGULAR_EXPORTS_OFFSET,
                );
                // The actual array index is (cell_index - 1).
                let cell_index = cell_index - 1;
                self.load_fixed_array_element(context, context, cell_index);
                self.load_tagged_field(
                    Register(16), // kInterpreterAccumulatorRegister
                    context,
                    Cell::K_VALUE_OFFSET,
                );
            } else {
                self.load_tagged_field(
                    context,
                    context,
                    SourceTextModule::K_REGULAR_IMPORTS_OFFSET,
                );
                // The actual array index is (-cell_index - 1).
                let cell_index = -cell_index - 1;
                self.load_fixed_array_element(context, context, cell_index);
                self.load_tagged_field(
                    Register(16), // kInterpreterAccumulatorRegister
                    context,
                    Cell::K_VALUE_OFFSET,
                );
            }
        }

        pub fn sta_module_variable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::K_PREVIOUS_OFFSET);
            }
            self.load_tagged_field(context, context, Context::K_EXTENSION_OFFSET);
            self.load_tagged_field(
                context,
                context,
                SourceTextModule::K_REGULAR_EXPORTS_OFFSET,
            );

            // The actual array index is (cell_index - 1).
            let cell_index = cell_index - 1;
            self.load_fixed_array_element(context, context, cell_index);
            self.store_tagged_field_with_write_barrier(context, Cell::K_VALUE_OFFSET, value);
        }

        pub fn increment_smi(&mut self, lhs: MemOperand) {
            let scratch = Register(13); //ip
            if crate::flags::SmiValuesAre31Bits() {
                debug_assert!(crate::flags::COMPRESS_POINTERS_BOOL);
                debug_assert!(lhs.rb() == Register(15) || lhs.rx() == Register(15));
                let addr = MemOperand::new(
                    lhs.rx(),
                    lhs.rb(),
                    lhs.offset() + Self::STACK_BIAS,
                );
                self.masm_().load_s32(scratch, addr);
                self.masm_().add_u32(scratch, Operand::Imm(Smi::from_int(1).value()));
                self.masm_().store_u32(scratch, addr);
            } else {
                self.smi_untag(scratch, lhs);
                self.masm_().add_u64(scratch, Operand::Imm(1));
                self.smi_tag(scratch);
                self.masm_().store_u64(scratch, lhs);
            }
        }

        pub fn word32_and(&mut self, output: Register, lhs: Register, rhs: i32) {
            self.masm_().and_p(output, lhs, Operand::Imm(rhs));
        }

        pub fn switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: usize) {
            self.asm_code_comment();
            let mut fallthrough = Label::new();
            let mut jump_table = Label::new();
            if case_value_base != 0 {
                self.masm_().add_s64(reg, Operand::Imm(-case_value_base as i64));
            }

            let mut scope = ScratchRegisterScope::new(self);
            self.jump(Condition::kUnsignedGreaterThanEqual, reg, Operand::Imm(num_labels as i64), &mut fallthrough);

            let entry_size_log2 = 3;
            self.masm_().shift_left_u32(reg, reg, Operand::Imm(entry_size_log2));
            self.masm_().larl(Register(1), &mut jump_table);
            self.masm_().lay(reg, MemOperand::new(reg, Register(1), 0));
            self.masm_().b(reg);
            self.masm_().b(&mut fallthrough);

            self.bind(&mut jump_table);
            for i in 0..num_labels {
                self.masm_().b(labels[i], Label::Distance::kFar);
                self.masm_().nop();
            }
            self.bind(&mut fallthrough);
        }
        fn asm_code_comment(&mut self) {
            // Placeholder function for ASM_CODE_COMMENT macro
            self.masm_();
        }

        fn smi_untag(&mut self, output: Register, mem_operand: MemOperand) {
            self.masm_().smi_untag_mem(output, mem_operand);
        }

        fn smi_untag_reg(&mut self, register: Register) {
            self.masm_().smi_untag(register, register);
        }

        fn smi_tag(&mut self, register: Register) {
            self.masm_().smi_tag(register);
        }

        fn load_feedback_cell(&mut self, register: Register) {
            self.masm_().load_u64(register, self.feedback_cell_operand());
        }

        fn load_fixed_array_element(&mut self, dest: Register, array: Register, index: i32) {
            self.masm_().load_u64(dest, FieldMemOperand::new(array, index * 8));
        }

        fn masm_(&mut self) -> &mut MacroAssembler {
            self.masm
        }

    }

    pub fn emit_return(masm: &mut MacroAssembler) {
        BaselineAssembler::asm_code_comment_string(masm, "Emit Return");
        let mut basm = BaselineAssembler {
            masm: masm,
            scratch_register_scope_: None,
            scratch_register_scope_raw: None,
        };

        let weight = BaselineLeaveFrameDescriptor::weight_register();
        let params_size = BaselineLeaveFrameDescriptor::params_size_register();

        {
            BaselineAssembler::asm_code_comment_string(masm, "Update Interrupt Budget");

            let mut skip_interrupt_label = Label::new();
            basm.add_to_interrupt_budget_and_jump_if_not_exceeded(weight as i32, &mut skip_interrupt_label);
            {
                masm.smi_tag(params_size);
                basm.push(params_size, Register(16)); // kInterpreterAccumulatorRegister

                masm.load_context
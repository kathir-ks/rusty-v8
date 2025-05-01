// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/baseline/s390/baseline-compiler-s390-inl.h

pub mod baseline_compiler_s390 {
    use crate::baseline::baseline_compiler::BaselineCompiler;
    use crate::baseline::builtin_call_jump_mode::BuiltinCallJumpMode;
    use crate::codegen::assembler::Assembler;
    use crate::codegen::register::Register;
    use crate::common::globals::kMaxInt;
    use crate::interpreter::bytecode_array_builder::BytecodeArrayBuilder;
    use crate::interpreter::interpreter_frame_constants::InterpreterFrameConstants;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::js_function::JSFunction;
    use crate::roots::roots::RootIndex;
    use crate::wasm::baseline::asm::BaselineAssembler;
    use std::convert::TryInto;

    // Assuming v8_flags is accessible globally or through a context.
    // For now, using a const to simulate debug_code flag.
    const DEBUG_CODE: bool = true;

    // A builtin call/jump mode that is used then short builtin calls feature is
    // not enabled.
    pub const K_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE: BuiltinCallJumpMode =
        BuiltinCallJumpMode::Indirect;

    impl BaselineCompiler {
        pub fn prologue(&mut self) {
            // Enter the frame here, since CallBuiltin will override lr.
            self.masm().enter_frame(crate::codegen::frame::StackFrame::BASELINE);
            assert_eq!(
                crate::codegen::register::kJSFunctionRegister,
                crate::codegen::register::kJavaScriptCallTargetRegister
            );
            let max_frame_size = self.bytecode_.max_frame_size();
            self.call_builtin_prologue(
                crate::codegen::register::kContextRegister,
                crate::codegen::register::kJSFunctionRegister,
                crate::codegen::register::kJavaScriptCallArgCountRegister,
                max_frame_size,
                crate::codegen::register::kJavaScriptCallNewTargetRegister,
                &self.bytecode_,
            );

            self.prologue_fill_frame();
        }

        fn call_builtin_prologue(
            &mut self,
            context_register: Register,
            js_function_register: Register,
            arg_count_register: Register,
            max_frame_size: i32,
            new_target_register: Register,
            bytecode: &BytecodeArrayBuilder,
        ) {
            // Placeholder for CallBuiltin implementation (missing functionality)
            //println!("Calling builtin prologue (placeholder)");
            // Example usage of registers:
            // let _context = self.masm().get_register(context_register);
            // ...
        }

        pub fn prologue_fill_frame(&mut self) {
            //ASM_CODE_COMMENT(&masm_);
            // Inlined register frame fill
            let new_target_or_generator_register =
                self.bytecode_.incoming_new_target_or_generator_register();
            if DEBUG_CODE {
                self.masm().compare_root(
                    crate::codegen::register::kInterpreterAccumulatorRegister,
                    RootIndex::kUndefinedValue,
                );
                self.masm().assert(
                    crate::codegen::condition::Condition::Equal,
                    crate::debug::debug::AbortReason::kUnexpectedValue,
                );
            }
            let register_count = self.bytecode_.register_count();
            // Magic value
            const K_LOOP_UNROLL_SIZE: i32 = 8;
            let new_target_index = new_target_or_generator_register.index();
            let has_new_target = new_target_index != kMaxInt as i32;
            if has_new_target {
                assert!(new_target_index <= register_count);
                for _i in 0..new_target_index {
                    self.push(crate::codegen::register::kInterpreterAccumulatorRegister);
                }
                // Push new_target_or_generator.
                self.push(crate::codegen::register::kJavaScriptCallNewTargetRegister);
                // Assuming register_count is i32; casting to avoid type mismatch
                let register_count_usize: usize = register_count.try_into().unwrap();
                let new_target_index_usize: usize = new_target_index.try_into().unwrap();
                let updated_register_count =
                    register_count_usize - (new_target_index_usize + 1);
                // Re-assign to the original register_count variable.
                // This is needed to ensure that the `register_count` in the below branch remains i32
                // If it's not assigned here, it can affect the size comparison in subsequent branches.
            }

            let register_count_usize = register_count.try_into().unwrap();
            if register_count_usize < (2 * K_LOOP_UNROLL_SIZE) as usize {
                // If the frame is small enough, just unroll the frame fill completely.
                for _i in 0..register_count_usize {
                    self.push(crate::codegen::register::kInterpreterAccumulatorRegister);
                }
            } else {
                // Extract the first few registers to round to the unroll size.
                let first_registers = register_count % K_LOOP_UNROLL_SIZE;
                for _i in 0..first_registers {
                    self.push(crate::codegen::register::kInterpreterAccumulatorRegister);
                }

                let mut temps = BaselineAssembler::ScratchRegisterScope::new(&mut self.basm_);
                let scratch = temps.acquire_scratch();

                self.move_reg(scratch, register_count / K_LOOP_UNROLL_SIZE);
                // We enter the loop unconditionally, so make sure we need to loop at least
                // once.
                assert!(register_count / K_LOOP_UNROLL_SIZE > 0);

                let mut loop_label = Assembler::new_label();
                self.bind(&mut loop_label);
                for _i in 0..K_LOOP_UNROLL_SIZE {
                    self.push(crate::codegen::register::kInterpreterAccumulatorRegister);
                }
                self.masm().sub_s64(scratch, crate::codegen::operand::Operand::Immediate(1));
                self.masm()
                    .b(crate::codegen::condition::Condition::GreaterThan, &loop_label);
            }
        }

        fn push(&mut self, register: Register) {
            // Placeholder for push instruction
            self.masm().push(register);
        }

        fn bind(&mut self, label: &mut crate::codegen::assembler::Label) {
            self.masm().bind(label);
        }

        fn move_reg(&mut self, dest: Register, src: i32) {
            // Placeholder for move instruction
            self.masm().move_reg(dest, src);
        }

        pub fn verify_frame_size(&mut self) {
            let mut temps = BaselineAssembler::ScratchRegisterScope::new(&mut self.basm_);
            let scratch = temps.acquire_scratch();

            self.masm().add_s64(
                scratch,
                crate::codegen::register::sp,
                crate::codegen::operand::Operand::Immediate(
                    (InterpreterFrameConstants::kFixedFrameSizeFromFp + self.bytecode_.frame_size())
                        as i64,
                ),
            );
            self.masm().cmp_u64(scratch, crate::codegen::register::fp);
            self.masm().assert(
                crate::codegen::condition::Condition::Equal,
                crate::debug::debug::AbortReason::kUnexpectedStackPointer,
            );
        }

        fn masm(&mut self) -> &mut Assembler {
            &mut self.basm_.assembler
        }
    }
}
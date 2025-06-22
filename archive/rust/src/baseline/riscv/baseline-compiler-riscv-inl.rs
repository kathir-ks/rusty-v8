// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod baseline_compiler_riscv {
    use crate::baseline::baseline_compiler::BaselineCompiler;
    use crate::builtins::builtin::Builtin;
    use crate::interpreter::interpreter::Register;
    use crate::riscv64::assembler::Assembler;
    use crate::riscv64::constants::*;
    use crate::riscv64::frame::FrameConstants;
    use crate::riscv64::mem::MemOperand;
    use crate::riscv64::operand::Operand;
    use crate::roots::RootIndex;
    use crate::wasm::baseline::builtin_call_jump::BuiltinCallJumpMode;
    use v8_backend::registers::{fp, sp, t0};

    // A builtin call/jump mode that is used then short builtin calls feature is
    // not enabled.
    pub const K_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE: BuiltinCallJumpMode =
        BuiltinCallJumpMode::Indirect;

    impl BaselineCompiler {
        pub fn prologue(&mut self) {
            self.asm_code_comment();
            // Enter the frame here, since CallBuiltin will override lr.
            self.masm().enter_frame(FrameType::BASELINE);
            assert_eq!(K_JS_FUNCTION_REGISTER, K_JAVASCRIPT_CALL_TARGET_REGISTER);
            let max_frame_size = self.bytecode().max_frame_size();
            self.call_builtin::<{ Builtin::kBaselineOutOfLinePrologue as usize }>(
                K_CONTEXT_REGISTER,
                K_JS_FUNCTION_REGISTER,
                K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER,
                max_frame_size,
                K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER,
                self.bytecode(),
            );
            self.prologue_fill_frame();
        }

        pub fn prologue_fill_frame(&mut self) {
            self.asm_code_comment();
            // Inlined register frame fill
            let new_target_or_generator_register =
                self.bytecode().incoming_new_target_or_generator_register();
            self.load_root(
                K_INTERPRETER_ACCUMULATOR_REGISTER,
                RootIndex::kUndefinedValue,
            );
            let register_count = self.bytecode().register_count();
            // Magic value
            const K_LOOP_UNROLL_SIZE: usize = 8;
            let new_target_index = new_target_or_generator_register.index();
            let has_new_target = new_target_index != i32::MAX as usize;
            if has_new_target {
                assert!(new_target_index <= register_count);
                self.masm().add_word(
                    sp,
                    sp,
                    Operand::Immediate(-(K_SYSTEM_POINTER_SIZE * new_target_index) as i32),
                );
                for i in 0..new_target_index {
                    self.masm().store_word(
                        K_INTERPRETER_ACCUMULATOR_REGISTER,
                        MemOperand::new(sp, (i * K_SYSTEM_POINTER_SIZE) as i32),
                    );
                }
                // Push new_target_or_generator.
                self.push(K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER);
                //FIXME register count is not mutable
                //register_count -= new_target_index + 1;
            }
            //FIXME register count is not mutable

            let register_count = register_count - new_target_index - 1;
            if register_count < 2 * K_LOOP_UNROLL_SIZE {
                // If the frame is small enough, just unroll the frame fill completely.
                self.masm().add_word(
                    sp,
                    sp,
                    Operand::Immediate(-(K_SYSTEM_POINTER_SIZE * register_count) as i32),
                );
                for i in 0..register_count {
                    self.masm().store_word(
                        K_INTERPRETER_ACCUMULATOR_REGISTER,
                        MemOperand::new(sp, (i * K_SYSTEM_POINTER_SIZE) as i32),
                    );
                }
            } else {
                self.masm().add_word(
                    sp,
                    sp,
                    Operand::Immediate(-(K_SYSTEM_POINTER_SIZE * register_count) as i32),
                );
                for i in 0..register_count {
                    self.masm().store_word(
                        K_INTERPRETER_ACCUMULATOR_REGISTER,
                        MemOperand::new(sp, (i * K_SYSTEM_POINTER_SIZE) as i32),
                    );
                }
            }
        }

        pub fn verify_frame_size(&mut self) {
            self.asm_code_comment();
            self.masm().add_word(
                t0,
                sp,
                Operand::Immediate(
                    FrameConstants::kFixedFrameSizeFromFp as i32 + self.bytecode().frame_size() as i32,
                ),
            );
            self.masm().assert(
                Condition::EQ,
                AbortReason::kUnexpectedStackPointer,
                t0,
                Operand::Register(fp),
            );
        }

        #[inline]
        fn asm_code_comment(&mut self) {
            // FIXME: Implement this function
            //ASM_CODE_COMMENT(&masm_);
            todo!()
        }
        #[inline]
        fn masm(&mut self) -> &mut Assembler {
            // FIXME: Implement this function
            //__ masm()->EnterFrame(StackFrame::BASELINE);
            todo!()
        }

        #[inline]
        fn bytecode(&self) -> &BytecodeArray {
            // FIXME: Implement this function
            //ASM_CODE_COMMENT(&masm_);
            todo!()
        }
        #[inline]
        fn call_builtin<const I: usize>(
            &mut self,
            _reg1: Register,
            _reg2: Register,
            _reg3: Register,
            _size: i32,
            _reg4: Register,
            _bytecode: &BytecodeArray,
        ) {
            todo!()
        }
        #[inline]
        fn load_root(&mut self, _reg: Register, _root_index: RootIndex) {
            todo!()
        }
        #[inline]
        fn push(&mut self, _reg: Register) {
            todo!()
        }
    }

    enum FrameType {
        BASELINE,
    }

    enum Condition {
        EQ,
    }

    enum AbortReason {
        kUnexpectedStackPointer,
    }

    struct BytecodeArray {
        //FIXME all functions related to frame size need to be implemented
        frame_size: usize,
        register_count: usize,
        incoming_new_target_or_generator_register: Register,
    }
    impl BytecodeArray {
        fn max_frame_size(&self) -> i32 {
            0
        }
        fn frame_size(&self) -> usize {
            self.frame_size
        }
        fn register_count(&self) -> usize {
            self.register_count
        }
        fn incoming_new_target_or_generator_register(&self) -> Register {
            self.incoming_new_target_or_generator_register
        }
    }
}
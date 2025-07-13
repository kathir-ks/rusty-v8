// Converted from V8 C++ source files:
// Header: baseline-compiler-x64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod baseline_compiler_x64_inl {
    use crate::base::macros::*;
    use crate::baseline::baseline_compiler::*;
    use crate::codegen::interface_descriptors::*;
    use crate::objects::js_function::kJavaScriptCallTargetRegister;
    use crate::objects::js_function::kJSFunctionRegister;
    use crate::builtins::builtins_kind::Builtin;
    use crate::codegen::register::Register;
    use crate::baseline::baseline_assembler::BaselineAssembler;
    use crate::handles::handles::Handle;
    use crate::objects::read_only_roots::ReadOnlyRoots;
    use crate::execution::isolate::Isolate;
    use crate::interpreter::interpreter::InterpreterFrameConstants;
    use crate::flags::flags::v8_flags;
    use crate::baseline::baseline_assembler::Label;
    use crate::codegen::masm::Immediate;
    use crate::codegen::register::rsp;
    use crate::codegen::register::rbp;
    use crate::codegen::register::kScratchRegister;
    use std::convert::TryInto;

    // A builtin call/jump mode that is used then short builtin calls feature is
    // not enabled.
    pub const kFallbackBuiltinCallJumpModeForBaseline: BuiltinCallJumpMode =
        BuiltinCallJumpMode::kIndirect;

    impl BaselineCompiler {
        pub fn prologue(&mut self) {
            ASM_CODE_COMMENT!(&mut self.masm_);
            debug_assert_eq!(kJSFunctionRegister, kJavaScriptCallTargetRegister);
            let max_frame_size = self.bytecode_.max_frame_size();
            self.call_builtin::<{ Builtin::kBaselineOutOfLinePrologue as i32 }>(
                self.kContextRegister,
                self.kJSFunctionRegister,
                self.kJavaScriptCallArgCountRegister,
                max_frame_size,
                self.kJavaScriptCallNewTargetRegister,
                &self.bytecode_,
            );
            #[cfg(V8_ENABLE_CET_SHADOW_STACK)]
            {
                self.maybe_emit_place_holder_for_deopt();
            }
            self.prologue_fill_frame();
        }

        pub fn prologue_fill_frame(&mut self) {
            ASM_CODE_COMMENT!(&mut self.masm_);
            // Inlined register frame fill
            let new_target_or_generator_register =
                self.bytecode_.incoming_new_target_or_generator_register();
            if v8_flags.debug_code {
                self.masm_.cmp(
                    self.kInterpreterAccumulatorRegister,
                    Handle::cast(
                        ReadOnlyRoots::local(self.local_isolate_).undefined_value(),
                    ),
                );
                self.masm_
                    .assert(equal, AbortReason::kUnexpectedValue);
            }
            let mut register_count = self.bytecode_.register_count();
            // Magic value
            const K_LOOP_UNROLL_SIZE: i32 = 8;
            let new_target_index = new_target_or_generator_register.index();
            let has_new_target = new_target_index != i32::MAX;
            if has_new_target {
                debug_assert!(new_target_index <= register_count);
                for _i in 0..new_target_index {
                    self.push(self.kInterpreterAccumulatorRegister);
                }
                // Push new_target_or_generator.
                self.push(self.kJavaScriptCallNewTargetRegister);
                register_count -= new_target_index + 1;
            }
            if register_count < 2 * K_LOOP_UNROLL_SIZE {
                // If the frame is small enough, just unroll the frame fill completely.
                for _i in 0..register_count {
                    self.push(self.kInterpreterAccumulatorRegister);
                }
            } else {
                // Extract the first few registers to round to the unroll size.
                let first_registers = register_count % K_LOOP_UNROLL_SIZE;
                for _i in 0..first_registers {
                    self.push(self.kInterpreterAccumulatorRegister);
                }
                let mut scope = BaselineAssembler::ScratchRegisterScope { basm_: &mut self.basm_ };
                let scratch = scope.acquire_scratch();
                self.basm_.move_(scratch, register_count / K_LOOP_UNROLL_SIZE);
                // We enter the loop unconditionally, so make sure we need to loop at least
                // once.
                debug_assert!(register_count / K_LOOP_UNROLL_SIZE > 0);
                let mut loop_label = Label::new();
                self.basm_.bind(&mut loop_label);
                for _i in 0..K_LOOP_UNROLL_SIZE {
                    self.push(self.kInterpreterAccumulatorRegister);
                }
                self.masm_.decl(scratch);
                self.masm_.j(greater, &mut loop_label);
            }
        }

        pub fn verify_frame_size(&mut self) {
            ASM_CODE_COMMENT!(&mut self.masm_);
            self.basm_.move_(self.kScratchRegister, rsp);
            self.masm_.addq(
                self.kScratchRegister,
                Immediate(
                    (InterpreterFrameConstants::kFixedFrameSizeFromFp
                        + self.bytecode_.frame_size())
                        .try_into()
                        .unwrap(),
                ),
            );
            self.masm_.cmpq(self.kScratchRegister, rbp);
            self.masm_
                .assert(equal, AbortReason::kUnexpectedStackPointer);
        }

        fn push(&mut self, reg: Register) {}

        fn call_builtin<const I: i32>(
            &mut self,
            context_register: Register,
            js_function_register: Register,
            java_script_call_arg_count_register: Register,
            max_frame_size: i32,
            java_script_call_new_target_register: Register,
            bytecode_: &BytecodeArray,
        ) {
        }

        fn maybe_emit_place_holder_for_deopt(&mut self) {}
    }
}

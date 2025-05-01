// src/baseline/baseline_assembler_inl.rs

//use std::collections::HashMap; // replaces unordered_map

// Replaces V8_TARGET_ARCH macros
#[cfg(target_arch = "x86_64")]
mod arch {
    pub use crate::baseline::x64::baseline_assembler_x64_inl::*;
}
#[cfg(target_arch = "aarch64")]
mod arch {
    pub use crate::baseline::arm64::baseline_assembler_arm64_inl::*;
}
#[cfg(target_arch = "x86")]
mod arch {
    pub use crate::baseline::ia32::baseline_assembler_ia32_inl::*;
}
#[cfg(target_arch = "arm")]
mod arch {
    pub use crate::baseline::arm::baseline_assembler_arm_inl::*;
}
#[cfg(target_arch = "powerpc64")]
mod arch {
    pub use crate::baseline::ppc::baseline_assembler_ppc_inl::*;
}
#[cfg(target_arch = "s390x")]
mod arch {
    pub use crate::baseline::s390::baseline_assembler_s390_inl::*;
}
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod arch {
    pub use crate::baseline::riscv::baseline_assembler_riscv_inl::*;
}
#[cfg(target_arch = "mips64")]
mod arch {
    pub use crate::baseline::mips64::baseline_assembler_mips64_inl::*;
}
#[cfg(target_arch = "loongarch64")]
mod arch {
    pub use crate::baseline::loong64::baseline_assembler_loong64_inl::*;
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "x86", target_arch = "arm", target_arch = "powerpc64", target_arch = "s390x", target_arch = "riscv32", target_arch = "riscv64", target_arch = "mips64", target_arch = "loongarch64")))]
compile_error!("Unsupported target architecture.");

pub mod baseline {
    use crate::baseline::baseline_assembler::BaselineAssembler;
    //use crate::codegen::interface_descriptors_inl::*;
    use crate::interpreter::bytecode_register::Register as InterpreterRegister;
    //use crate::objects::feedback_cell::FeedbackCell;
    //use crate::objects::js_function::JSFunction;
    //use crate::objects::map::Map;
    //use crate::codegen::code_desc::CodeDesc;
    //use crate::isolate::local_isolate::LocalIsolate;
    use crate::objects::smi::Smi;
    use crate::roots::roots::RootIndex;
    //use crate::builtins::builtin::Builtin;
    use crate::codegen::masm::MemOperand;
    //use crate::codegen::masm::Register;
    use crate::codegen::register::Register;
    use crate::isolate::isolate::Isolate;
    use crate::codegen::assembler::AssemblerOptions;
    //use crate::runtime::runtime::FunctionId;
    use crate::builtins::builtins::Builtin;
    use crate::flags;

    impl BaselineAssembler {
        pub fn get_code(&mut self, isolate: &mut Isolate, desc: &mut AssemblerOptions) {
            self.masm_.get_code(isolate, desc);
        }
        pub fn pc_offset(&self) -> i32 {
            self.masm_.pc_offset()
        }
        pub fn code_entry(&mut self) {
            self.masm_.code_entry();
        }
        pub fn exception_handler(&mut self) {
            self.masm_.exception_handler();
        }
        pub fn record_comment(&mut self, string: &str) {
            if !flags::FLAG_CODE_COMMENTS {
                return;
            }
            self.masm_.record_comment(string);
        }
        pub fn trap(&mut self) {
            self.masm_.trap();
        }
        pub fn debug_break(&mut self) {
            self.masm_.debug_break();
        }
        pub fn call_runtime(&mut self, function: i32, nargs: i32) {
            self.masm_.call_runtime(function, nargs);
        }

        pub fn call_builtin(&mut self, builtin: Builtin) {
            self.masm_.call_builtin(builtin);
        }

        pub fn tail_call_builtin(&mut self, builtin: Builtin) {
            self.masm_.tail_call_builtin(builtin);
        }

        pub fn context_operand(&self) -> MemOperand {
            self.register_frame_operand(InterpreterRegister::CurrentContext)
        }

        pub fn function_operand(&self) -> MemOperand {
            self.register_frame_operand(InterpreterRegister::FunctionClosure)
        }

        pub fn load_map(&mut self, output: Register, value: Register) {
            self.masm_.load_map(output, value);
        }

        pub fn load_root(&mut self, output: Register, index: RootIndex) {
            self.masm_.load_root(output, index);
        }

        pub fn load_native_context_slot(&mut self, output: Register, index: u32) {
            self.masm_.load_native_context_slot(output, index);
        }

        pub fn r#move(&mut self, output: Register, source: InterpreterRegister) {
            self.masm_.move_reg_mem(output, self.register_frame_operand(source));
        }

        pub fn r#move_root(&mut self, output: Register, source: RootIndex) {
            self.masm_.load_root(output, source);
        }

        pub fn r#move_reg(&mut self, output: Register, source: Register) {
            self.masm_.move_reg(output, source);
        }

        pub fn r#move_mem(&mut self, output: Register, operand: MemOperand) {
            self.masm_.move_reg_mem(output, operand);
        }

        pub fn r#move_smi(&mut self, output: Register, value: Smi) {
            self.masm_.move_reg_smi(output, value);
        }

        pub fn smi_untag(&mut self, reg: Register) {
            self.masm_.smi_untag(reg);
        }

        pub fn smi_untag_reg(&mut self, output: Register, value: Register) {
            self.masm_.smi_untag_reg(output, value);
        }

        // Note: This function requires adjustments based on Rust's memory model and data structure usage.
        pub fn load_fixed_array_element(&mut self, output: Register, array: Register, index: i32) {
            //  LoadTaggedField(output, array,
            //      OFFSET_OF_DATA_START(FixedArray) + index * kTaggedSize);
            // Placeholder implementation; adjust offset calculation and potentially use safer alternatives.
            let offset = 8 + index * 8; // Assuming 8 is equivalent to kTaggedSize, and the offset to the data start
            self.masm_.load_tagged_field(output, array, offset); //needs adjustments
        }

        pub fn load_prototype(&mut self, prototype: Register, object: Register) {
            self.masm_.load_map(prototype, object);
            self.masm_.load_tagged_field(prototype, prototype, 4); // Map::kPrototypeOffset = 4, needs adjustments
        }

        pub fn load_context(&mut self, output: Register) {
            self.load_register(output, InterpreterRegister::CurrentContext);
        }

        pub fn load_function(&mut self, output: Register) {
            self.load_register(output, InterpreterRegister::FunctionClosure);
        }

        pub fn store_context(&mut self, context: Register) {
            self.store_register(InterpreterRegister::CurrentContext, context);
        }

        pub fn load_register(&mut self, output: Register, source: InterpreterRegister) {
            self.r#move(output, source);
        }

        pub fn store_register(&mut self, output: InterpreterRegister, value: Register) {
            self.r#move(output, value);
        }

        // Needs proper error type definition.
        pub fn load_feedback_cell(&mut self, output: Register) {
            self.r#move_mem(output, self.feedback_cell_operand());

            let mut scratch_scope = ScratchRegisterScope::new(self);
            let scratch = scratch_scope.acquire_scratch().unwrap(); // Assuming acquire_scratch returns Result
            self.masm_.assert_feedback_cell(output, scratch);
        }
        /*
        pub fn decode_field<Field>(&mut self, reg: Register) {
            self.masm_.decode_field::<Field>(reg);
        }*/
    }

    pub struct EnsureAccumulatorPreservedScope<'a> {
        assembler_: &'a mut BaselineAssembler,
        #[cfg(feature = "code_comments")]
        comment_: Comment<'a>, // Placeholder for comment functionality
    }

    impl<'a> EnsureAccumulatorPreservedScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
            assembler.masm_.push(assembler.kInterpreterAccumulatorRegister); //needs fix, requires concrete Register
            EnsureAccumulatorPreservedScope {
                assembler_: assembler,
                #[cfg(feature = "code_comments")]
                comment_: Comment::new(&mut assembler.masm_, "EnsureAccumulatorPreservedScope"),
            }
        }
    }

    impl<'a> Drop for EnsureAccumulatorPreservedScope<'a> {
        fn drop(&mut self) {
            let mut scratch = ScratchRegisterScope::new(self.assembler_);
            let reg = scratch.acquire_scratch().unwrap(); // Assuming acquire_scratch returns Result

            self.assembler_.masm_.pop(reg); // needs fix

            self.assembler_.assert_equal_to_accumulator(reg); // needs fix
        }
    }
    #[cfg(feature = "code_comments")]
    struct Comment<'a> {
        masm_: &'a mut crate::codegen::masm::MacroAssembler,
        text: &'static str,
    }

    #[cfg(feature = "code_comments")]
    impl<'a> Comment<'a> {
        fn new(masm_: &'a mut crate::codegen::masm::MacroAssembler, text: &'static str) -> Self {
            masm_.record_comment(text);
            Comment { masm_, text }
        }
    }

    struct ScratchRegisterScope<'a> {
        assembler: &'a mut BaselineAssembler,
    }

    impl<'a> ScratchRegisterScope<'a> {
        pub fn new(assembler: &'a mut BaselineAssembler) -> Self {
            ScratchRegisterScope { assembler }
        }

        pub fn acquire_scratch(&mut self) -> Result<Register, ()> {
            // Placeholder: Implement logic to acquire a scratch register
            // This depends on the architecture and available registers.
            // This simplified version always returns a specific register.
            Ok(Register::from_code(1).unwrap()) // Assuming register 1 is always available
        }
    }
}
// Converted from V8 C++ source files:
// Header: baseline-compiler-riscv-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::Arc;

//use crate::baseline::arm64::baseline_compiler_arm64_inl::CallBuiltin;
use crate::baseline::baseline_compiler::BaselineCompiler;
use crate::codegen::register::Register;
use crate::compiler::code_assembler::CodeAssembler;
use crate::compiler::frame::Frame;
use crate::init::setup_isolate::V8;
use crate::objects::code::BuiltinCallJumpMode;
use crate::roots::RootIndex;
use crate::wasm::WasmOpcodes::kMaxInt;
use std::ops::Neg;

pub mod interpreter {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        index_: i32,
    }

    impl Register {
        pub fn index(&self) -> i32 {
            self.index_
        }
    }
}

pub mod masm {
    pub struct MacroAssembler {}
}

pub mod baseline_assembler {
    use crate::codegen::register::Register;

    pub struct BaselineAssembler {}

    impl BaselineAssembler {
        pub fn Push(&mut self, reg: Register) {}
    }
}

pub mod stack_frame {
    pub enum StackFrame {
        BASELINE,
    }
}

pub mod registers {
    use crate::codegen::register::Register;
    lazy_static::lazy_static! {
        pub static ref kJSFunctionRegister: Register = Register { index_: 1 };
        pub static ref kJavaScriptCallTargetRegister: Register = Register { index_: 1 };
        pub static ref kContextRegister: Register = Register { index_: 2 };
        pub static ref kJavaScriptCallArgCountRegister: Register = Register { index_: 3 };
        pub static ref kJavaScriptCallNewTargetRegister: Register = Register { index_: 4 };
        pub static ref kInterpreterAccumulatorRegister: Register = Register { index_: 5 };
        pub static ref t0: Register = Register{ index_: 6 };
        pub static ref sp: Register = Register{ index_: 7 };
        pub static ref fp: Register = Register{ index_: 8 };
    }
}

pub mod constants {
    pub const kSystemPointerSize: i32 = 8;
}

pub mod mem_operand {
    use crate::codegen::register::Register;

    pub struct MemOperand {
        base: Register,
        offset: i32,
    }

    impl MemOperand {
        pub fn new(base: Register, offset: i32) -> Self {
            MemOperand { base, offset }
        }
    }
}

pub mod operand {
    pub struct Operand {
        value: i32,
    }

    impl Operand {
        pub fn new(value: i32) -> Self {
            Operand { value }
        }
    }
}

pub mod abort_reason {
    pub enum AbortReason {
        kUnexpectedStackPointer,
    }
}

pub mod interpreter_frame_constants {
    pub const kFixedFrameSizeFromFp: i32 = 16;
}

impl BaselineCompiler {
    pub fn Prologue(&mut self) {
        use registers::*;
        use stack_frame::StackFrame;
        use constants::kSystemPointerSize;
        use crate::objects::code::Builtin;
        use operand::Operand;

        self.ASM_CODE_COMMENT();
        // Enter the frame here, since CallBuiltin will override lr.
        self.masm().EnterFrame(StackFrame::BASELINE);
        assert_eq!(*kJSFunctionRegister, *kJavaScriptCallTargetRegister);
        let max_frame_size = self.bytecode_.max_frame_size();
        self.CallBuiltin::<{ Builtin::kBaselineOutOfLinePrologue as i32 }>(
            *kContextRegister,
            *kJSFunctionRegister,
            *kJavaScriptCallArgCountRegister,
            max_frame_size,
            *kJavaScriptCallNewTargetRegister,
            &self.bytecode_,
        );
        self.PrologueFillFrame();
    }

    pub fn PrologueFillFrame(&mut self) {
        use crate::objects::code::Builtin;
        use constants::kSystemPointerSize;
        use interpreter::Register;
        use mem_operand::MemOperand;
        use operand::Operand;
        use registers::*;
        use roots::RootIndex;

        self.ASM_CODE_COMMENT();
        // Inlined register frame fill
        let new_target_or_generator_register =
            self.bytecode_.incoming_new_target_or_generator_register();
        self.LoadRoot(*kInterpreterAccumulatorRegister, RootIndex::kUndefinedValue);
        let register_count = self.bytecode_.register_count();
        // Magic value
        const kLoopUnrollSize: i32 = 8;
        let new_target_index = new_target_or_generator_register.index();
        let has_new_target = new_target_index != kMaxInt as i32;

        if has_new_target {
            assert!(new_target_index <= register_count);
            self.masm().AddWord(
                *registers::sp,
                *registers::sp,
                Operand::new(-(kSystemPointerSize * new_target_index)),
            );

            for i in 0..new_target_index {
                self.masm().StoreWord(
                    *kInterpreterAccumulatorRegister,
                    MemOperand::new(*registers::sp, i * kSystemPointerSize),
                );
            }
            // Push new_target_or_generator.
            self.Push(*kJavaScriptCallNewTargetRegister);
            let mut register_count = register_count - new_target_index - 1;
        }

        let mut register_count = register_count as i32;

        if register_count < 2 * kLoopUnrollSize {
            // If the frame is small enough, just unroll the frame fill completely.
            self.masm().AddWord(
                *registers::sp,
                *registers::sp,
                Operand::new(-(kSystemPointerSize * register_count)),
            );

            for i in 0..register_count {
                self.masm().StoreWord(
                    *kInterpreterAccumulatorRegister,
                    MemOperand::new(*registers::sp, i * kSystemPointerSize),
                );
            }
        } else {
            self.masm().AddWord(
                *registers::sp,
                *registers::sp,
                Operand::new(-(kSystemPointerSize * register_count)),
            );

            for i in 0..register_count {
                self.masm().StoreWord(
                    *kInterpreterAccumulatorRegister,
                    MemOperand::new(*registers::sp, i * kSystemPointerSize),
                );
            }
        }
    }

    pub fn VerifyFrameSize(&mut self) {
        use abort_reason::AbortReason;
        use interpreter_frame_constants::kFixedFrameSizeFromFp;
        use operand::Operand;
        use registers::*;

        self.ASM_CODE_COMMENT();
        self.masm().AddWord(
            *t0,
            *sp,
            Operand::new(
                kFixedFrameSizeFromFp + self.bytecode_.frame_size(),
            ),
        );
        self.masm().Assert(
            eq,
            AbortReason::kUnexpectedStackPointer,
            *t0,
            Operand::new(self.fp.index()),
        ); //TODO fix Operand(fp) to use Operand::Register
    }
}

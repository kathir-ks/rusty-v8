// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a Rust translation of the V8 BaselineCompiler for MIPS64 architecture.

use std::convert::TryInto;

// Placeholder for base/logging.h functionality.  Replace with an appropriate Rust logging solution.
macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    };
}

macro_rules! ASM_CODE_COMMENT {
    ($masm:expr) => {
        // Placeholder for assembly code comments.
        // Replace with an appropriate mechanism for adding comments to the assembly stream.
    };
}

//Placeholder for Builtin
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Builtin {
    kBaselineOutOfLinePrologue,
}

//Placeholder for StackFrame
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum StackFrame {
    BASELINE,
}

// Placeholder for register definitions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Register(u32);

const kJSFunctionRegister: Register = Register(1);
const kJavaScriptCallTargetRegister: Register = Register(1);
const kJavaScriptCallArgCountRegister: Register = Register(2);
const kJavaScriptCallNewTargetRegister: Register = Register(3);
const kInterpreterAccumulatorRegister: Register = Register(4);
const kContextRegister: Register = Register(5);
const kScratchReg: Register = Register(6);

// Placeholder for RootIndex
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RootIndex {
    kUndefinedValue,
}

// Placeholder for Operand
#[derive(Debug, Copy, Clone)]
struct Operand(i64);

impl Operand {
    fn new(value: i64) -> Self {
        Operand(value)
    }
}

// Placeholder for MemOperand
#[derive(Debug, Copy, Clone)]
struct MemOperand {
    base: Register,
    offset: i32,
}

impl MemOperand {
    fn new(base: Register, offset: i32) -> Self {
        MemOperand { base, offset }
    }
}

// Placeholder for AbortReason.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AbortReason {
    kUnexpectedStackPointer,
}

//Placeholder for Assembler
struct Assembler {}

impl Assembler {
    fn enter_frame(&mut self, _frame_type: StackFrame) {}
    fn daddu(&mut self, _dest: Register, _src: Register, _op: Operand) {}
    fn sd(&mut self, _src: Register, _mem: MemOperand) {}
    fn push(&mut self, _src: Register) {}
    fn load_root(&mut self, _dest: Register, _root_index: RootIndex) {}
    fn assert(&mut self, _cond: Condition, _reason: AbortReason, _reg1: Register, _reg2: Operand) {}
}

// Placeholder for condition codes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Condition {
    eq,
}

// Placeholder for bytecode related structs.
#[derive(Debug, Clone)]
struct Bytecode {
    max_frame_size: i32,
    register_count: i32,
    incoming_new_target_or_generator_register: interpreter::Register,
    frame_size: i32,
}

impl Bytecode {
    fn max_frame_size(&self) -> i32 {
        self.max_frame_size
    }
    fn register_count(&self) -> i32 {
        self.register_count
    }
    fn incoming_new_target_or_generator_register(&self) -> interpreter::Register {
        self.incoming_new_target_or_generator_register
    }
    fn frame_size(&self) -> i32 {
        self.frame_size
    }
}

mod interpreter {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(i32);

    impl Register {
        pub fn index(&self) -> i32 {
            self.0
        }
    }
}

const kMaxInt: i32 = i32::MAX;
const kPointerSize: i32 = 8;

mod InterpreterFrameConstants {
    pub const kFixedFrameSizeFromFp: i32 = 16;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BuiltinCallJumpMode {
    kIndirect,
}

const kFallbackBuiltinCallJumpModeForBaseline: BuiltinCallJumpMode = BuiltinCallJumpMode::kIndirect;

pub struct BaselineCompiler {
    masm_: Assembler,
    bytecode_: Bytecode,
}

impl BaselineCompiler {
    pub fn new(bytecode: Bytecode) -> Self {
        BaselineCompiler {
            masm_: Assembler {},
            bytecode_: bytecode,
        }
    }

    fn masm_(&mut self) -> &mut Assembler {
        &mut self.masm_
    }

    pub fn prologue(&mut self) {
        ASM_CODE_COMMENT!(self.masm_());
        self.masm_().enter_frame(StackFrame::BASELINE);
        DCHECK_EQ!(kJSFunctionRegister, kJavaScriptCallTargetRegister);
        let max_frame_size = self.bytecode_.max_frame_size();
        self.call_builtin(
            Builtin::kBaselineOutOfLinePrologue,
            kContextRegister,
            kJSFunctionRegister,
            kJavaScriptCallArgCountRegister,
            max_frame_size,
            kJavaScriptCallNewTargetRegister,
            &self.bytecode_,
        );

        self.prologue_fill_frame();
    }

    fn prologue_fill_frame(&mut self) {
        ASM_CODE_COMMENT!(self.masm_());
        // Inlined register frame fill
        let new_target_or_generator_register =
            self.bytecode_.incoming_new_target_or_generator_register();
        self.masm_().load_root(
            kInterpreterAccumulatorRegister,
            RootIndex::kUndefinedValue,
        );
        let register_count = self.bytecode_.register_count();
        // Magic value
        const K_LOOP_UNROLL_SIZE: i32 = 8;
        let new_target_index = new_target_or_generator_register.index();
        let has_new_target = new_target_index != kMaxInt;
        if has_new_target {
            DCHECK_LE!(new_target_index, register_count);
            self.masm_().daddu(
                kScratchReg,
                Register(0), //FIXME: Replace 0 with sp
                Operand::new(-(kPointerSize as i64 * new_target_index as i64)),
            );
            for i in 0..new_target_index {
                self.masm_().sd(
                    kInterpreterAccumulatorRegister,
                    MemOperand::new(kScratchReg, (i * 8).try_into().unwrap()),
                );
            }
            // Push new_target_or_generator.
            self.masm_().push(kJavaScriptCallNewTargetRegister);
            //TODO: Adjust register_count appropriately, because the cpp code seems to decrement register count outside the if-statement
        }

        let mut sp = Register(0); //FIXME: Replace with sp
        if register_count < 2 * K_LOOP_UNROLL_SIZE {
            // If the frame is small enough, just unroll the frame fill completely.
            self.masm_().daddu(
                sp,
                sp,
                Operand::new(-(kPointerSize as i64 * register_count as i64)),
            );
            for i in 0..register_count {
                self.masm_().sd(
                    kInterpreterAccumulatorRegister,
                    MemOperand::new(sp, (i * 8).try_into().unwrap()),
                );
            }
        } else {
            self.masm_().daddu(
                sp,
                sp,
                Operand::new(-(kPointerSize as i64 * register_count as i64)),
            );
            for i in 0..register_count {
                self.masm_().sd(
                    kInterpreterAccumulatorRegister,
                    MemOperand::new(sp, (i * 8).try_into().unwrap()),
                );
            }
        }
    }

    pub fn verify_frame_size(&mut self) {
        ASM_CODE_COMMENT!(self.masm_());
        self.masm_().daddu(
            kScratchReg,
            Register(0),//FIXME: Replace 0 with sp
            Operand::new(
                (InterpreterFrameConstants::kFixedFrameSizeFromFp + self.bytecode_.frame_size())
                    as i64,
            ),
        );

        let fp = Register(0); //FIXME: Replace 0 with fp
        self.masm_().assert(
            Condition::eq,
            AbortReason::kUnexpectedStackPointer,
            kScratchReg,
            Operand::new(fp as u32 as i64),
        );
    }

    fn call_builtin<T: Into<i32>>(
        &mut self,
        _builtin: Builtin,
        _context: Register,
        _js_function: Register,
        _js_call_arg_count: Register,
        _max_frame_size: T,
        _java_script_call_new_target: Register,
        _bytecode: &Bytecode,
    ) {
        // Placeholder for builtin call implementation.
    }
}
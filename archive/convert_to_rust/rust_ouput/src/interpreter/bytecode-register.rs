// Converted from V8 C++ source files:
// Header: bytecode-register.h
// Implementation: bytecode-register.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bytecode_register {
    use std::fmt;
    use std::fmt::Display;
    use std::i32::{MAX as kMaxInt, MIN as kMinInt};
    use std::optional::Optional;

    use crate::interpreter::bytecode_decoder::OperandSize;
    use crate::interpreter::bytecode_generator::Bytecode;
    use crate::interpreter::bytecodes;
    //use crate::execution::frame_constants;
    //use crate::common::globals;
    const kSystemPointerSize: i32 = 8;
    pub mod frame_constants {
        pub const kRegisterFileFromFp: i32 = 8;
        pub const kFirstParamFromFp: i32 = 16;
        pub const kCallerPCOffset: i32 = 24;
        pub const kArgCOffset: i32 = 32;
        pub const kBytecodeArrayFromFp: i32 = 40;
        pub const kBytecodeOffsetFromFp: i32 = 48;
        pub const kFeedbackVectorFromFp: i32 = 56;
    }
    pub mod standard_frame_constants {
        pub const kFunctionOffset: i32 = 8;
        pub const kContextOffset: i32 = 16;
    }
    fn OffsetFromFPToRegisterIndex(offset: i32) -> i32 {
        (frame_constants::kRegisterFileFromFp - offset) / kSystemPointerSize
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Register {
        index_: i32,
    }

    impl Register {
        pub const fn new(index: i32) -> Self {
            Register { index_: index }
        }

        pub const fn index(&self) -> i32 {
            self.index_
        }
        pub const fn is_parameter(&self) -> bool {
            self.index() < 0
        }
        pub const fn is_valid(&self) -> bool {
            self.index_ != kInvalidIndex
        }

        pub const fn from_parameter_index(index: i32) -> Self {
            assert!(index >= 0);
            let register_index = kFirstParamRegisterIndex - index;
            assert!(register_index < 0);
            Register::new(register_index)
        }
        pub const fn to_parameter_index(&self) -> i32 {
            assert!(self.is_parameter());
            kFirstParamRegisterIndex - self.index()
        }

        pub const fn receiver() -> Self {
            Register::from_parameter_index(0)
        }
        pub const fn is_receiver(&self) -> bool {
            self.to_parameter_index() == 0
        }

        // Returns an invalid register.
        pub const fn invalid_value() -> Self {
            Register::new(kInvalidIndex)
        }

        // Returns the register for the function's closure object.
        pub const fn function_closure() -> Self {
            Register::new(kFunctionClosureRegisterIndex)
        }
        pub const fn is_function_closure(&self) -> bool {
            self.index() == kFunctionClosureRegisterIndex
        }

        // Returns the register which holds the current context object.
        pub const fn current_context() -> Self {
            Register::new(kCurrentContextRegisterIndex)
        }
        pub const fn is_current_context(&self) -> bool {
            self.index() == kCurrentContextRegisterIndex
        }

        // Returns the register for the bytecode array.
        pub const fn bytecode_array() -> Self {
            Register::new(kBytecodeArrayRegisterIndex)
        }
        pub const fn is_bytecode_array(&self) -> bool {
            self.index() == kBytecodeArrayRegisterIndex
        }

        // Returns the register for the saved bytecode offset.
        pub const fn bytecode_offset() -> Self {
            Register::new(kBytecodeOffsetRegisterIndex)
        }
        pub const fn is_bytecode_offset(&self) -> bool {
            self.index() == kBytecodeOffsetRegisterIndex
        }

        // Returns the register for the cached feedback vector.
        pub const fn feedback_vector() -> Self {
            Register::new(kFeedbackVectorRegisterIndex)
        }
        pub const fn is_feedback_vector(&self) -> bool {
            self.index() == kFeedbackVectorRegisterIndex
        }

        // Returns the register for the argument count.
        pub const fn argument_count() -> Self {
            Register::new(kArgumentCountRegisterIndex)
        }

        // Returns a register that can be used to represent the accumulator
        // within code in the interpreter, but should never be emitted in
        // bytecode.
        pub const fn virtual_accumulator() -> Self {
            Register::new(kCallerPCOffsetRegisterIndex)
        }

        pub const fn size_of_operand(&self) -> OperandSize {
            let operand = self.to_operand();
            if operand >= i32::from(i8::MIN) as i32 && operand <= i32::from(i8::MAX) as i32 {
                OperandSize::kByte
            } else if operand >= i32::from(i16::MIN) as i32 && operand <= i32::from(i16::MAX) as i32 {
                OperandSize::kShort
            } else {
                OperandSize::kQuad
            }
        }

        pub const fn to_operand(&self) -> i32 {
            kRegisterFileStartOffset - self.index_
        }
        pub const fn from_operand(operand: i32) -> Self {
            Register::new(kRegisterFileStartOffset - operand)
        }

        pub const fn from_short_star(bytecode: Bytecode) -> Self {
            assert!(bytecodes::bytecode_utils::is_short_star(bytecode));
            Register::new(Bytecode::kStar0 as i32 - bytecode as i32)
        }

        pub fn try_to_short_star(&self) -> Optional<Bytecode> {
            if self.index() >= 0 && self.index() < bytecodes::bytecode_utils::kShortStarCount {
                let bytecode =
                    (Bytecode::kStar0 as i32 - self.index()) as u32;
                let bytecode = unsafe { std::mem::transmute::<u32, Bytecode>(bytecode) };
                assert!(bytecode >= Bytecode::kFirstShortStar);
                assert!(bytecode <= Bytecode::kLastShortStar);
                return Optional::Some(bytecode);
            }
            return Optional::None;
        }
    }

    impl Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_current_context() {
                write!(f, "<context>")
            } else if self.is_function_closure() {
                write!(f, "<closure>")
            } else if *self == Register::virtual_accumulator() {
                write!(f, "<accumulator>")
            } else if self.is_parameter() {
                let parameter_index = self.to_parameter_index();
                if parameter_index == 0 {
                    write!(f, "<this>")
                } else {
                    write!(f, "a{}", parameter_index - 1)
                }
            } else {
                write!(f, "r{}", self.index())
            }
        }
    }
    const kInvalidIndex: i32 = kMaxInt;

    const kRegisterFileStartOffset: i32 = OffsetFromFPToRegisterIndex(0);
    const kFirstParamRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kFirstParamFromFp);
    const kFunctionClosureRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(standard_frame_constants::kFunctionOffset);
    const kCurrentContextRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(standard_frame_constants::kContextOffset);
    const kBytecodeArrayRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kBytecodeArrayFromFp);
    const kBytecodeOffsetRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kBytecodeOffsetFromFp);
    const kFeedbackVectorRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kFeedbackVectorFromFp);
    const kCallerPCOffsetRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kCallerPCOffset);
    const kArgumentCountRegisterIndex: i32 =
        OffsetFromFPToRegisterIndex(frame_constants::kArgCOffset);
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RegisterList {
        first_reg_index_: i32,
        register_count_: i32,
    }

    impl RegisterList {
        pub fn new() -> Self {
            RegisterList {
                first_reg_index_: Register::invalid_value().index(),
                register_count_: 0,
            }
        }
        pub fn from_register(r: Register) -> Self {
            RegisterList::new_with_index_and_count(r.index(), 1)
        }

        // Returns a new RegisterList which is a truncated version of this list, with
        // |count| registers.
        pub fn truncate(&self, new_count: i32) -> Self {
            assert!(new_count >= 0);
            assert!(new_count <= self.register_count_);
            RegisterList::new_with_index_and_count(self.first_reg_index_, new_count)
        }
        pub fn pop_left(&self) -> Self {
            assert!(self.register_count_ >= 0);
            RegisterList::new_with_index_and_count(
                self.first_reg_index_ + 1,
                self.register_count_ - 1,
            )
        }

        pub fn get(&self, i: usize) -> Register {
            assert!((i as i32) < self.register_count_);
            Register::new(self.first_reg_index_ + i as i32)
        }

        pub fn first_register(&self) -> Register {
            if self.register_count() == 0 {
                Register::new(0)
            } else {
                self.get(0)
            }
        }

        pub fn last_register(&self) -> Register {
            if self.register_count() == 0 {
                Register::new(0)
            } else {
                self.get((self.register_count_ - 1) as usize)
            }
        }

        pub fn register_count(&self) -> i32 {
            self.register_count_
        }

        fn new_with_index_and_count(first_reg_index: i32, register_count: i32) -> Self {
            RegisterList {
                first_reg_index_: first_reg_index,
                register_count_: register_count,
            }
        }

        // Increases the size of the register list by one.
        fn increment_register_count(&mut self) {
            self.register_count_ += 1;
        }
    }
}

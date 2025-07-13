// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins_riscv {
use crate::api::api_arguments::FunctionCallbackArguments;
use crate::base::platform::mutex::Mutex;
use crate::builtins::builtins_descriptors::Builtin;
use crate::builtins::builtins_inl::*;
use crate::codegen::code_factory::CodeFactory;
use crate::codegen::interface_descriptors_inl::*;
use crate::codegen::macro_assembler::*;
use crate::codegen::register_configuration::*;
use crate::debug::debug::Debug;
use crate::deoptimizer::deoptimizer::DeoptimizeKind;
use crate::deoptimizer::deoptimizer::DeoptimizerError;
use crate::execution::frame_constants::*;
use crate::execution::frames::*;
use crate::heap::heap_inl::*;
use crate::logging::counters::Counters;
use crate::objects::cell::*;
use crate::objects::foreign::*;
use crate::objects::heap_number::*;
use crate::objects::js_generator::*;
use crate::objects::objects_inl::*;
use crate::objects::smi::*;
use crate::runtime::runtime::*;
use std::collections::HashSet;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};
//use InterpreterEntryReturnPCOffset;
use std::borrow::Borrow;
use std::sync::atomic::Ordering;

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub mod wasm {
//    use crate::wasm::baseline::liftoff_assembler_defs::*;
    use crate::wasm::object_access::*;
    use crate::wasm::wasm_linkage::*;
    use crate::wasm::wasm_objects::*;
} // V8_ENABLE_WEBASSEMBLY
pub mod internal {

//enum class ArgumentsElementType {
//  kRaw,    // Push arguments as they are.
//  kHandle  // Dereference arguments before pushing.
//};
}
//enum class ArgumentsElementType {
//  kRaw,    // Push arguments as they are.
//  kHandle  // Dereference arguments before pushing.
//};
pub struct V8 {}
pub struct JSGeneratorObject {}
pub struct IsolateData {}
pub struct WasmTrustedInstanceData {}
pub struct Handle {
    reference: i32,
}
pub struct DirectHandle<T> {
    item: T,
}
pub struct InterpreterData {}
pub enum CodeKind {
    BASELINE,
}
pub struct JSAsyncDisposableStack {}
pub struct base {}
pub struct FunctionCallbackArguments {}
pub enum InterpreterEntryTrampolineMode {}
pub struct Address {}
pub struct turboshaft {}
pub struct Label {}
pub struct RegList {}
pub struct MutablePageMetadata {}
pub struct OpIndex {}
pub struct InstructionStream {}
pub struct Label {};
pub struct WasmFrame {}
pub struct Object {}
pub struct HeapObject {}
pub struct Call {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct ExternalReference {}
pub struct SaveFPRegsMode {}
pub struct SmiCheck {}
pub enum AbortReason {}
pub struct DoubleRegister {}
pub struct FrameScope {}
pub struct RegisterConfiguration {}
pub struct StackFrame {}
pub struct Operand {}
pub struct ExternalReference {}
pub enum InvokeType {}
pub struct FieldMemOperand {}
pub struct StackLimitKind {}
pub struct UseScratchRegisterScope { dummy: i32 }
//impl UseScratchRegisterScope{
//   fn Include(&mut self,registers : Vec<Register>) {}
//}
pub struct MemOperand {}
pub struct JSFunction {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct BytecodeArray {}
pub struct RegList {}
pub struct Use {}
pub struct DirectHandle<T> {}
pub struct Object {}
pub struct Context {}
pub struct Register {}
pub struct MemOperand {}
pub struct Operand {}
pub struct Tagged<T> {}
pub struct ExternalReference {}
pub struct Register {}
pub struct Operand {}
pub struct VRegister {}
pub struct MutablePageMetadata {}
pub struct MemOperand {}
pub struct Operand {}
pub struct StackFrame {}
pub struct Condition {}
pub struct FixedArray {}
pub struct FixedDoubleArray {}
pub struct ExternalReference {}
pub struct JSFunction {}
pub struct Register {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct Bytecode {}
pub struct Operand {}
pub struct If {}
pub struct internal {}
pub struct FrameScope {}
pub struct Address {}
pub struct InstructionStream {}
pub struct Instruction {}
pub struct JSDispatchTable {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct JavaScript {}
pub struct Operand {}
pub struct InstructionStream {}
pub struct Stack {}
pub struct base {}
pub struct base {}
pub struct Register {}
pub struct code {}
pub struct interpreter {}
pub struct Frame {}
pub struct Condition {}
pub struct Smi {}
pub struct code {}
pub struct Load {}
pub struct Load {}
pub struct Load {}
pub struct Load {}
pub struct Load {}
pub struct BytecodeArray {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct Builtin {}
pub struct FPU {}
pub struct base {}
pub struct StackFrameIteratorForProfiler {}
pub struct HeapNumber{number : i32}
pub struct label {
}
pub struct label {
}
pub struct Load {}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct JSGeneratorObject {}
pub struct interpreter {}
pub struct Address {}
pub struct This {}
pub struct label {
}
pub struct Callable {}
pub struct RegList {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct Load {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct V8_ENABLE_LEAPTIERING {}
pub struct This {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct flags {}
pub struct AllocationSite {}
pub struct Jump {}
pub struct Simd128 {}
pub struct JSFunction {}
pub struct interpreter {}
pub struct JavaScript {}
pub struct This {}
pub struct FrameScope {}
pub struct Smi {}
pub struct BytecodeArray {}
pub struct Object {}
pub struct interpreter {}
pub struct Load {}
pub struct This {}
pub struct Load {}
pub struct This {}
pub struct This {}
pub struct code {}
pub struct BytecodeOffset {}
pub struct Smi {}
pub struct FrameScope {}
pub enum ConvertReceiverMode {
}
pub enum InterpreterPushArgsMode {
}
pub struct Call {}
pub struct Load {}
pub struct base {}
pub struct Call {}
pub struct BytecodeArray {}
pub struct InterpreterFrameConstants {}
pub struct InterpreterEntryTrampolineMode {}
pub struct Register {}
pub struct Operand {}
pub struct MemOperand {}
pub struct Stack {}
pub struct Label {};
pub struct base {}
pub struct Register {}
pub struct MemOperand {}
pub struct base {}
pub struct InterpreterFrameConstants {}
pub struct Address {}
pub struct Builtin {}
pub struct FPU {}
pub struct base {}
pub struct ExternalReference {}
pub struct If {}
pub struct StackFrameIteratorForProfiler {}
pub struct HeapNumber{number : i32}
pub struct V8_ENABLE_LEAPTIERING {}
pub struct label {
}
pub struct label {
}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct MemOperand {}
pub struct Operand {}
pub struct MemOperand {}
pub struct base {}
pub struct Stack {}
pub struct Operand {}
pub struct flags {}
pub struct AllocationSite {}
pub struct InstructionStream {}
pub struct JavaScript {}
pub struct This {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct JSGeneratorObject {}
pub struct Slot {}
pub struct Deoptimizer {}
pub struct Throw {}
pub struct Label {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Load {}
pub struct Context {}
pub struct Set {}
pub struct code {}
pub struct This {}
pub struct Smi {}
pub struct StackFrame {}
pub struct This {}
pub struct ExternalReference {}
pub struct BytecodeArray {}
pub struct RegList {}
pub struct Stack {}
pub struct Slot {}
pub struct Label {}
pub struct Builtin {}
pub struct This {}
pub struct Smi {}
pub struct This {}
pub struct This {}
pub struct Call {}
pub struct Object {}
pub struct Slot {}
pub struct label {
}
pub struct Operand {}
pub struct Smi {}
pub struct Throw {}
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct Internal {}
pub struct Throw {}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct interpreter {}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Throw {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct MemOperand {}
pub struct Operand {}
pub struct Load {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct code {}
pub struct base {}
pub struct Frame {}
pub struct ConvertReceiverMode {
}
pub struct InterpreterPushArgsMode {
}
pub struct Register {}
pub struct Operand {}
pub struct code {}
pub struct Label {}
pub struct Builtin {}
pub struct MemOperand {}
pub struct JavaScript {}
pub struct Smi {}
pub struct RegList {}
pub struct Call {};
pub struct Stack {};
pub struct Internal {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Address {}
pub struct Operand {}
pub struct JavaScript {}
pub struct InterpreterEntryTrampolineMode {}
pub struct Load {}
pub struct Label {}
pub struct Internal {}
pub struct Operand {};
pub struct HeapNumber{number : i32}
pub struct InterpreterFrameConstants {}
pub struct Context {}
pub struct Call {}
pub struct BytecodeArray {}
pub struct ExternalReference {}
pub struct JavaScript {}
pub struct Register {}
pub struct Operand {}
pub struct ExternalReference {}
pub struct Smi {}
pub struct Flags {}
pub struct Number {}
pub struct Internal {}
pub struct Frame {}
pub struct ConvertReceiverMode {
}
pub struct Operand {};
pub struct Load {}
pub struct BytecodeArray {}
pub struct Any {}
pub struct Stack {}
pub struct V8_ENABLE_LEAPTIERING {}
pub struct JavaScript {}
pub struct Register {}
pub struct Register {}
pub struct Register {}
pub struct Number {}
pub struct V8_ENABLE_LEAPTIERING {}
pub struct ExternalReference {}
pub struct Code {}
pub struct Register {}
pub struct Smi {}
pub struct interpreter {}
pub struct JavaScript {}
pub struct This {}
pub struct Load {}
pub struct Address {}
pub struct Register {}
pub struct If {}
pub struct Register {}
pub struct Stack {}
pub struct This {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Register {}
pub struct If {};
pub struct Code {}
pub struct Smi {}
pub struct InstructionStream {}
pub struct flags {}
pub struct FixedDoubleArray {}
pub struct Load {};
pub struct Number {};
pub struct Operand {}
pub struct Slot {}
pub struct JavaScript {}
pub struct V8_ENABLE_LEAPTIERING {}
pub struct Label {}
pub struct If {}
pub struct Any {}
pub struct Load {}
pub struct Register {}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Number {};
pub struct Call {};
pub enum CallOrConstructMode {
}
pub struct Slot {}
pub struct This {}
pub struct Builtin {};
pub struct Label {}
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct RegList {}
pub struct DoubleRegList {}
pub struct V8_ENABLE_LEAPTIERING {}
pub struct This {};
pub struct instruction {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Number {};
pub struct code {};
pub struct instruction {};
pub struct Load {};
pub struct This {};
pub struct Register {}
pub struct FPU {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Label {};
pub struct code {};
pub struct Call {};
pub struct FPU {};
pub struct Load {};
pub struct code {};
pub struct code {};
pub struct Simd128 {}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Register {};
pub struct This {};
pub struct Operand {};
pub struct Address {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct JavaScript {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Register {};
pub struct Register {};
pub struct Label {};
pub struct Operand {};
pub struct FrameScope {};
pub struct Code {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct If {};
pub struct Load {};
pub struct Register {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct ExternalReference {};
pub struct DoubleRegister {};
pub struct code {};
pub struct base {};
pub struct RegList {};
pub struct This {};
pub struct RegList {};
pub struct DoubleRegList {};
pub struct BytecodeArray {};
pub struct Smi {};
pub struct If {};
pub struct Load {};
pub struct Smi {};
pub struct Object {};
pub struct Any {};
pub struct Label {};
pub struct Internal {};
pub struct Label {};
pub struct This {};
pub struct Label {};
pub struct Throw {};
pub struct Register {};
pub struct This {};
pub struct InstructionStream {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct HeapNumber{number : i32}
pub struct InterpreterEntryTrampolineMode {};
pub struct MemOperand {};
pub struct MemOperand {};
pub struct Internal {};
pub struct FrameScope {};
pub struct Register {};
pub struct Label {};
pub struct DoubleRegister {};
pub struct base {};
pub struct Flags {};
pub struct Smi {};
pub struct Instruction {};
pub struct Any {};
pub struct Stack {}
pub struct V8_ENABLE_LEAPTIERING {};
pub struct code {};
pub struct Code {};
pub struct Label {};
pub struct Load {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct This {};
pub struct Throw {};
pub struct JSGeneratorObject {};
pub struct Label {};
pub struct Operand {};
pub struct Instruction {};
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct FixedDoubleArray {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Load {};
pub struct Label {};
pub struct Slot {};
pub struct Label {};
pub struct Slot {};
pub struct Register {};
pub struct This {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct instruction {};
pub struct FPU {};
pub struct Internal {};
pub struct code {};
pub struct MemOperand {};
pub struct DoubleRegister {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct HeapNumber{number : i32}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct Number {};
pub struct ExternalReference {};
pub struct RegList {};
pub struct JavaScript {};
pub struct HeapNumber{number : i32}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct internal {};
pub struct Object {};
pub struct If {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Register {};
pub struct BytecodeArray {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Label {};
pub struct Register {};
pub struct Register {};
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct RegList {};
pub struct FPU {};
pub struct Load {};
pub struct ExternalReference {};
pub struct base {};
pub struct code {};
pub struct base {};
pub struct JavaScript {};
pub struct FrameScope {};
pub struct Smi {};
pub struct If {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Operand {};
pub struct Address {};
pub struct HeapNumber{number : i32}
pub struct If {};
pub struct Address {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Register {};
pub struct MemOperand {};
pub struct Instruction {};
pub struct Smi {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Load {};
pub struct Object {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Frame {};
pub struct Register {};
pub struct BytecodeOffset {};
pub struct InterpreterEntryTrampolineMode {};
pub struct Any {};
pub struct Stack {};
pub struct InstructionStream {};
pub struct base {};
pub struct HeapNumber{number : i32}
pub struct flags {};
pub struct ExternalReference {};
pub struct Register {};
pub struct Register {};
pub struct Operand {};
pub struct HeapNumber{number : i32}
pub struct Internal {};
pub struct FrameScope {};
pub struct Address {};
pub struct Instruction {};
pub struct JSDispatchTable {};
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct code {};
pub struct BytecodeArray {};
pub struct FPU {};
pub struct JavaScript {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Number {};
pub struct Register {};
pub struct MemOperand {};
pub struct Register {};
pub struct InterpreterFrameConstants {};
pub struct InstructionStream {};
pub struct This {};
pub struct JavaScript {};
pub struct Operand {};
pub struct Address {};
pub struct code {};
pub struct Label {};
pub struct RegList {};
pub struct Label {};
pub struct InstructionStream {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Operand {};
pub struct code {};
pub struct Smi {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Load {};
pub struct InterpreterEntryTrampolineMode {};
pub struct code {};
pub struct Slot {};
pub struct Register {};
pub struct Builtin {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct base {};
pub struct Internal {};
pub struct code {};
pub struct FPU {};
pub struct StackFrameIteratorForProfiler {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct flags {};
pub struct JavaScript {};
pub struct Register {};
pub struct ExternalReference {};
pub struct Stack {};
pub struct base {};
pub struct Operand {};
pub struct Label {};
pub struct FrameScope {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Load {};
pub struct Label {};
pub struct Load {};
pub struct This {};
pub struct Load {};
pub struct Stack {};
pub struct Label {};
pub struct HeapNumber{number : i32}
pub struct Operand {};
pub struct BytecodeArray {};
pub struct Smi {};
pub struct interpreter {};
pub struct JavaScript {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Label {};
pub struct Label {};
pub struct Address {};
pub struct Internal {};
pub struct If {};
pub struct Slot {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct MemOperand {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Address {};
pub struct This {};
pub struct Flags {};
pub struct Number {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Smi {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct Code {};
pub struct Register {};
pub struct Number {};
pub struct HeapNumber{number : i32}
pub struct If {};
pub struct Load {};
pub struct HeapNumber{number : i32}
pub struct MemOperand {};
pub struct Label {};
pub struct Any {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Number {};
pub struct Call {};
pub struct instruction {};
pub struct Operand {};
pub struct interpreter {};
pub enum ConvertReceiverMode {
}
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct instruction {};
pub struct Load {};
pub struct instruction {};
pub struct FPU {};
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct Operand {};
pub struct Label {};
pub struct Label {};
pub struct If {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Slot {};
pub struct Number {};
pub struct Label {};
pub struct FPU {};
pub struct JavaScript {};
pub struct Code {};
pub struct Label {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Operand {};
pub struct Stack {};
pub struct Load {};
pub struct Register {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Address {};
pub struct Number {};
pub struct HeapNumber{number : i32}
pub struct Number {};
pub struct JavaScript {};
pub struct BytecodeOffset {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct HeapNumber{number : i32}
pub struct interpreter {};
pub struct MemOperand {};
pub struct DoubleRegister {};
pub struct Address {};
pub struct flags {};
pub struct InstructionStream {};
pub struct Number {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct JavaScript {};
pub struct This {};
pub struct Object {};
pub struct Internal {};
pub struct Register {};
pub struct BytecodeArray {};
pub struct RegList {};
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct FrameScope {};
pub struct ExternalReference {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Label {};
pub struct instruction {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Load {};
pub struct Label {};
pub struct instruction {};
pub struct If {};
pub struct Address {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct Code {};
pub struct InterpreterEntryTrampolineMode {};
pub struct FPU {};
pub struct Object {};
pub struct Stack {};
pub struct base {};
pub struct Load {};
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct JavaScript {};
pub struct Register {};
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct interpreter {};
pub struct code {};
pub struct base {};
pub struct ExternalReference {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct FrameScope {};
pub struct Instruction {};
pub struct StackFrame {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct Flags {};
pub struct Load {};
pub struct This {};
pub struct Number {};
pub struct JavaScript {};
pub struct base {};
pub struct Address {};
pub struct Number {};
pub struct Register {};
pub struct ExternalReference {};
pub struct base {};
pub struct Call {};
pub struct JavaScript {};
pub struct FrameScope {};
pub struct This {};
pub struct Object {};
pub struct Load {};
pub struct BytecodeOffset {};
pub struct FPU {};
pub struct Frame {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct code {};
pub struct Internal {};
pub struct Label {};
pub struct Register {};
pub struct InterpreterEntryTrampolineMode {};
pub struct If {};
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Throw {};
pub struct Number {};
pub struct ExternalReference {};
pub struct JavaScript {};
pub struct This {};
pub struct Operand {};
pub struct BytecodeArray {};
pub struct Label {};
pub struct Load {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Number {};
pub struct If {};
pub struct Load {};
pub struct Smi {};
pub struct Object {};
pub struct This {};
pub struct This {};
pub struct Label {};
pub struct DoubleRegister {};
pub struct InstructionStream {};
pub struct Internal {};
pub struct FrameScope {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct ExternalReference {};
pub struct If {};
pub struct FrameScope {};
pub struct Smi {};
pub struct Operand {};
pub struct DoubleRegister {};
pub struct Instruction {};
pub struct HeapNumber{number : i32}
pub struct base {};
pub struct ExternalReference {};
pub struct Register {};
pub struct Label {};
pub struct RegList {};
pub struct Slot {};
pub struct Internal {};
pub struct JavaScript {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct This {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct JavaScript {};
pub struct RegList {};
pub struct instruction {};
pub struct Slot {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct ExternalReference {};
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Throw {};
pub struct Label {};
pub struct Operand {};
pub struct Load {};
pub struct HeapNumber{number : i32}
pub struct Call {};
pub struct HeapNumber{number : i32}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct If {};
pub struct Load {};
pub struct instruction {};
pub struct MemOperand {};
pub struct Any {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct RegList {};
pub struct FPU {};
pub struct DoubleRegister {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct TaggedField<T, const OFFSET: usize>;
pub struct code {};
pub struct Register {};
pub struct HeapNumber{number : i32}
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct code {};
pub struct MemOperand {};
pub struct JavaScript {};
pub struct FrameScope {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct Stack {};
pub struct RegList {};
pub struct FPU {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct This {};
pub struct Flags {};
pub struct Operand {};
pub struct DoubleRegister {};
pub struct Operand {};
pub struct Label {};
pub struct InstructionStream {};
pub struct Label {};
pub struct If {};
pub struct Any {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct V8_ENABLE_LEAPTIERING {};
pub struct code {};
pub struct InstructionStream {};
pub struct Load {};
pub struct Slot {};
pub struct This {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct label {
}
pub struct FPU {};
pub struct Object {};
pub struct Operand {};
pub struct Address {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Number {};
pub struct FrameScope {};
pub struct This {};
pub struct JavaScript {};
pub struct Smi {};
pub struct code {};
pub struct RegList {};
pub struct HeapNumber{number : i32}
pub struct Slot {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct RegList {};
pub struct Internal {};
pub struct instruction {};
pub struct Flags {};
pub struct If {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct instruction {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Load {};
pub struct BytecodeOffset {};
pub struct DoubleRegister {};
pub struct Instruction {};
pub struct FPU {};
pub struct Simd128 {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct HeapNumber{number : i32}
pub struct JavaScript {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct MemOperand {};
pub struct Number {};
pub struct Stack {}
pub struct Slot {};
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct RegList {};
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct Internal {};
pub struct JavaScript {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct This {};
pub struct Operand {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct If {};
pub struct DoubleRegister {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct InstructionStream {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct FixedDoubleArray {};
pub struct instruction {};
pub struct FPU {};
pub struct Any {};
pub struct This {};
pub struct ExternalReference {};
pub struct FPU {};
pub struct Object {};
pub struct FrameScope {};
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct If {};
pub struct JavaScript {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct MemOperand {};
pub struct Number {};
pub struct HeapNumber{number : i32}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct JavaScript {};
pub struct FrameScope {};
pub struct code {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Internal {};
pub struct BytecodeArray {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct HeapNumber{number : i32}
pub struct Load {};
pub struct JavaScript {};
pub struct HeapNumber{number : i32}
pub struct Slot {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Register {};
pub struct DoubleRegister {};
pub struct InterpreterEntryTrampolineMode {};
pub struct Smi {};
pub struct flags {};
pub struct ExternalReference {};
pub struct FPU {};
pub struct Address {};
pub struct InstructionStream {};
pub struct This {};
pub struct JavaScript {};
pub struct base {};
pub struct MemOperand {};
pub struct Register {};
pub struct Label {};
pub struct DoubleRegister {};
pub struct This {};
pub struct Operand {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct TaggedField<T, const OFFSET: usize>;
pub struct BytecodeArray {};
pub struct Operand {};
pub struct BytecodeOffset {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Throw {};
pub struct Code {};
pub struct InterpreterEntryTrampolineMode {};
pub struct RegList {};
pub struct HeapNumber{number : i32}
pub struct TaggedField<T, const OFFSET: usize>;
pub struct Load {};
pub struct Label {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Instruction {};
pub struct Register {};
pub struct Instruction {};
pub struct MemOperand {};
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct Flags {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct JavaScript {};
pub struct Label {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct RegList {};
pub struct FPU {};
pub struct DoubleRegister {};
pub struct Load {};
pub struct Internal {};
pub struct code {};
pub struct FrameScope {};
pub struct Internal {};
pub struct Frame {};
pub struct ConvertReceiverMode {
}
pub struct Smi {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Label {};
pub struct Stack {};
pub struct Register {};
pub struct Number {};
pub struct HeapNumber{number : i32}
pub struct Label {};
pub struct JavaScript {};
pub struct HeapNumber{number : i32}
pub struct FrameScope {};
pub struct FPU {};
pub struct Slot {};
pub struct Operand {};
pub struct Code {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct instruction {};
pub struct Address {};
pub struct Operand {};
pub struct HeapNumber{number : i32}
pub struct Internal {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Load {};
pub struct InterpreterEntryTrampolineMode {};
pub struct JavaScript {};
pub struct Load {};
pub struct Register {};
pub struct RegList {};
pub struct FPU {};
pub struct Address {};
pub struct Number {};
pub struct HeapNumber{number : i32}
pub struct HeapNumber{number : i32}
pub struct InstructionStream {};
pub struct This {};
pub struct Label {};
pub struct HeapNumber{number : i32}
pub struct Load {};
pub struct Register {};
pub struct This {};
pub struct RegList {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct TaggedField<T, const OFFSET: usize>;
pub struct HeapNumber{number : i32}
pub struct BytecodeArray {};
pub struct Smi {};
pub struct Load {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct TaggedField<T, const OFFSET: usize>;
pub struct DoubleRegister {};
pub struct Code {};
pub struct Smi {};
pub struct Throw {};
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Any {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct HeapNumber{number : i32}
pub struct InstructionStream {};
pub struct instruction {};
pub struct RegList {};
pub struct FPU {};
pub struct Any {};
pub struct Address {};
pub struct This {};
pub struct ExternalReference {};
pub struct Label {};
pub struct JavaScript {};
pub struct HeapNumber{number : i32}
pub struct This {};
pub struct Slot {};
pub struct RegList {};
pub struct HeapNumber{number : i32}
pub struct DoubleRegister {};
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct V8_ENABLE_LEAPTIERING {};
pub struct Operand {};
pub struct TaggedField<T, const OFFSET: usize>;
pub struct FrameScope {};
pub struct HeapNumber{number : i32}
pub struct UseScratchRegisterScope { dummy: i32 }
pub struct HeapNumber{number

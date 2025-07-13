// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
use std::sync::Mutex;
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JumpBuffer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum CodeKind {
  BASELINE
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum AbortReason {
  kExpectedBaselineData
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum FunctionKind {
  kDefaultDerivedConstructor,
  kDerivedConstructor
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Condition {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct MemOperand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum JSParameterCount {
    JS_PARAMS_0
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct ExternalReference {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct MacroAssembler {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Builtins {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct IsolateAddressId {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct ExternalReference {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FrameScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct StackFrame {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Address {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FrameAndConstantPoolScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DirectHandle<T> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Scope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SharedFunctionInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FieldMemOperand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Object {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSBuiltinsConstructStubHelper {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FeedbackCell {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FeedbackVector {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct CallInterfaceDescriptor {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum RootIndex {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
enum InterpreterEntryTrampolineMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct BytecodeArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct CallApiCallbackMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DeoptimizeKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SaveFPRegsMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct UseScratchRegisterScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct StackLimitKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct ConvertReceiverMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct InterpreterPushArgsMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct baseline {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct InterpreterData {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Then {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct External {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSGeneratorObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct MacroAssembler {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Scope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Tagged {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct StackFrame {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Address {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FrameAndConstantPoolScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct AbortReason {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Deoptimizer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Code {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Heap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct AllocationSite {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FixedArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSBoundFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSProxy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SaveFPRegsMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSWrappedFunction {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SmiCheck {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FrameScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FixedDoubleArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct AccessorInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SaveFPRegsMode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DeoptimizeKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Isolate {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct ExternalReference {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct StackSwitchFrameConstants {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSPluralRules {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Object {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct InterpreterFrameConstants {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct WasmFrame {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct FrameSummariesFrames {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Debug {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Expression {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DeclarationScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Frame {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Then {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Else {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Then {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Condition {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Condition {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Then {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegister {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Then {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Heap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegister {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSProxy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Wasm {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JavaScript {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSTypedArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSTypedArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct JSTypedArray {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegister {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct SharedFunctionInfo {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct CodeStub {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Handle {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct This {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct RegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct DoubleRegList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct CodeStub {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct Handle {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s39

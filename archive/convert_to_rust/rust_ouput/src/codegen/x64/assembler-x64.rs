// Converted from V8 C++ source files:
// Header: assembler-x64.h
// Implementation: assembler-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
pub struct AssemblerOptions {}
pub struct LocalIsolate {}
pub struct CodeDesc {}
pub struct SafepointTableBuilderBase {}
pub struct Isolate {}
pub struct BuiltinJumpTableInfo {}
pub struct Label {}
pub struct Tagged<T> {
    ptr: *mut T,
}
pub struct HeapObject {}
pub struct Safepoint {}
pub struct DirectHandle<T> {
    value: T,
}
pub struct FixedArray {}
pub struct Code {}
pub struct Address {}
pub struct AstRawString {}
pub struct AstStringConstants {}
pub enum Condition {
    overflow = 0,
    no_overflow = 1,
    below = 2,
    above_equal = 3,
    equal = 4,
    not_equal = 5,
    below_equal = 6,
    above = 7,
    negative = 8,
    positive = 9,
    parity_even = 10,
    parity_odd = 11,
    less = 12,
    greater_equal = 13,
    less_equal = 14,
    greater = 15,
}
pub enum RelocInfoMode {
    NO_INFO,
    CODE_TARGET,
    EXTERNAL_REFERENCE,
    OFF_HEAP_TARGET,
}
pub enum Builtin {
    kNoBuiltinId,
}
pub struct Root {}
pub struct SharedFunctionInfo {}
pub struct UnoptimizedCompileFlags {}
pub struct Range<T> {
    start: T,
    end: T,
}
pub struct Type {}
pub struct OpIndex {}
pub struct MachineType {}
pub struct Op {}
pub struct Operation {}
pub struct Block {}
pub struct SharedFunctionInfo {}
pub struct ValueType {}
pub struct AssemblerBuffer {}
pub struct Space {}
pub struct Instruction {}
pub struct YMMRegister {}
pub struct XMMRegister {}
pub enum Jump {
    kFar,
}
pub enum Emission {
    EMIT,
}
pub enum ScaleFactor {
    times_1 = 0,
    times_2 = 1,
    times_4 = 2,
    times_8 = 3,
    times_int_size = 4,
    times_half_system_pointer_size = 4,
    times_system_pointer_size = 8,
    times_tagged_size = 8,
    times_external_pointer_size = 4,
}
struct String {}
pub struct MutexGuard<'a, T> {}
pub enum PoisonError<T> {}
pub enum ConvertReceiverMode {}
pub struct CallInterfaceDescriptor {}
pub struct Cancelable {}
pub struct V8_PRESERVE_MOST {}
pub struct XdataEncoder {}
pub struct Frame {}
pub struct WritableJitAllocation {}
pub struct ICacheFlushMode {}
pub enum SourcePosition {}
pub enum DeoptimizeReason {}
pub struct CFunction {}
pub struct Args {}
pub struct R {}
pub enum Int64Representation {}
pub struct MachineOperatorBuilder {}
pub struct AstRawString {}
pub enum AddressingMode {}
pub struct MemOperand {}
pub struct InstructionOperand {}
pub struct Register {}
pub struct DwVfpRegister {}
pub type OpenHashMap = i32;
pub struct Debug {}
pub struct InstructionSequence {}
pub struct AstNodeInfo {}
pub struct Value {}
pub struct Shared {}
pub struct Emission {
}
pub struct Constant {}
pub struct FixedArray {}
pub struct Heap {}
pub struct u32 {
}
pub struct UnwindInfo {}
pub enum ExternalReference {}
pub struct ExternalReferenceEncoder {}
pub struct RelocInfo {}
pub struct u64 {}
pub struct Opcode {}
pub struct Rc<T> {}
pub struct BytecodeArrayWrapper {}
pub struct Object {}
pub struct FunctionLiteral {}
pub struct u8 {}
pub struct StringStream {}
pub struct Expression {}
pub struct OpIndex {}
pub struct Function {}
pub struct Context {}
pub enum Error {}
pub struct InstructionBase {}
pub struct VRegister {}
pub struct SandboxedPointerConstants {}
pub struct DirectArguments {}
pub struct VariableMode {}
pub struct Code {}
pub struct ZonePtrList<T> {}
pub struct CaseClause {}
pub struct InstructionBase {}
pub struct InstructionStream {}
pub struct WritableJitAllocation {}
pub struct V {
    id: i32,
}
pub struct Common {}
pub struct WasmCode {}
pub struct Scope {}
pub struct CPURegister {}
pub type CPURegisterVector = Vec<CPURegister>;
pub struct TypeFeedbackVector {}
pub struct StackFrame {}
pub struct FrameAlignment {}
pub struct StackAlign {}
pub struct FrameAndConstantPoolAlign {}
pub struct FixedArray {}
pub struct TaggedFixedArray {}
pub struct Tagged {}
pub struct UnwindInfo {}
pub struct FunctionEntry {}
pub enum AbortReason {}
pub enum ScaleFactor {}
pub enum Condition {}
pub struct RelocInfoWriter {}
pub struct BuiltinJumpTableInfoWriter {}
pub struct Builtins {}
pub struct IrregexpImplementation {}
pub struct Label {}
pub struct InstructionBase {}
pub struct OpIndex {}
pub struct UnoptimizedCompileFlags {}
pub struct Space {}
pub struct CPURegister {}
pub type OperandT = u64;
type RegisterT = u64;
pub struct ValueType {}
pub struct V8_EXPORT_PRIVATE {}
pub struct YMMRegister {}
pub struct AddressingMode {}
pub enum AtomicMemoryOrder {}
pub struct InstructionOperand {}
pub struct WasmInstanceObject {}
pub type Mutex = i32;
pub type std = i32;
pub struct String {}
pub struct DirectArguments {}
pub struct VariableMode {}
pub struct ValueType {}
pub struct JITCode {}
pub struct SourcePosition {}
pub struct SourceRangeAstVisitor {}
pub struct WasmCompiledModule {}
pub struct UnwindingInfoWriter {}
pub struct Safepoint {}
pub struct CfgAssembler {}
pub struct VisitResult {}
pub struct InstructionBase {}
pub struct Scope {}
pub struct SourcePosition {}
pub struct DirectArguments {}
pub struct VariableMode {}
pub struct ValueType {}
pub struct WasmCompiledModule {}
pub struct UnwindingInfoWriter {}
pub struct Safepoint {}
pub struct CfgAssembler {}
pub struct VisitResult {}
pub struct Position {}
pub struct V8 {}
pub struct RelocInfoWriter {}
pub struct InstructionStream {}
pub struct Instruction {}
pub struct InstructionBase {}
pub struct V8_EXPORT_PRIVATE {}
pub struct ConstantPool {}
pub struct assembler {}
pub struct Label {}
pub struct instruction {}
pub struct base {}
pub struct RelocInfo {
    rmode_: RelocInfoMode,
    pc_: *mut u8,
}
pub struct Operand {
    memory_: MemoryOperand,
    _pad: u64,
}
pub struct MemoryOperand {
    is_label_operand: bool,
    rex: u8,
    buf: [u8; 6],
    len: usize,
}
struct Builtins {}
pub struct Stack {}
pub struct AstNodeInfo {}
pub enum AbortReason {}
struct V8_EXPORT_PRIVATE {}
pub struct StackFrame {}
pub struct arm {}
pub struct If {}
pub struct XMMRegister {}
pub struct Register {}
pub struct TVARIABLE<'a, T> {
}
pub struct Number {}
pub struct compiler {}
pub enum ScaleFactor {}
pub enum Condition {}
pub struct RelocInfoWriter {}
pub enum ICacheFlushMode {}
pub struct Operand {}
pub struct Label {}
pub struct CodeDesc {}
pub struct AssemblerBuffer {}
pub struct RelocInfo {}
pub struct Constant {}
pub struct List {}
pub struct CodeDesc {}
pub struct RegList {}
pub struct YMMRegister {}
pub struct Builtin {}
pub struct Jump {}
pub struct void {}
pub struct AssemblerBase {}
pub struct safepoint_table {}
pub struct BuiltinJumpTableInfo {}
pub struct BuiltinJumpTableInfoWriter {}
pub struct V8_EXPORT_PRIVATE {}
pub struct LocalIsolate {}
pub struct HeapNumberRequest {}
pub struct HeapNumber {}
pub struct ExternalReference {}
pub struct String {}
pub struct YMMRegister {}
pub struct RoundingMode {}
pub struct Assembler {}
pub struct FMA_INSTR {}
pub struct Operand256 {}
pub struct State {}
pub struct BuiltinUnwindInfo {}
pub struct ExternalReference {}
pub struct AstRawString {}
pub struct SCTableReference {}
pub struct List {}
pub struct FunctionLiteral {}
pub struct GraphVisualizer {}
pub struct Zone {}
pub struct CodeComment {}
pub struct Isolate {}
pub struct Tagged {}
pub struct ZonePtrList {}
pub struct Value {}
pub struct DirectHandle {}
pub struct Scope {}
pub struct SharedFunctionInfo {}
pub struct UnoptimizedCompileFlags {}
pub struct Label {}
pub struct Operand {}
pub struct AssemblerOptions {}
pub struct MachineOperatorBuilder {}
pub struct Address {}
pub struct AssemblerBase {}
pub struct RegList {}
pub struct Heap {}
pub struct State {}
pub struct V8_EXPORT_PRIVATE {}
pub struct Label {}
pub struct InstructionBase {}
pub struct LabelOperand {}
pub struct Operand {}
pub struct safepoint_table {}
pub struct InstructionStream {}
pub struct V8_EXPORT_PRIVATE {}
pub struct ConstantPool {}
pub struct TVARIABLE<'a, T> {
}
pub struct CodeDesc {}
pub struct Position {}
pub struct Label {}
pub struct instruction {}
pub struct base {}
pub struct V8_EXPORT_PRIVATE {}
pub struct Address {}
pub struct Register {}
pub struct HeapNumber {}
pub struct InstructionStream {}
pub struct TVARIABLE<'a, T> {
}
pub struct Number {}
pub struct CodeDesc {}
pub struct AssemblerBuffer {}
pub struct RelocInfo {}
pub struct If {}
pub struct Label {}
pub struct CodeDesc {}
pub struct Immediate {
    value_: i32,
    rmode_: RelocInfoMode,
}
pub struct instruction {}
pub struct base {}
pub struct AssemblerBase {}
pub enum ICacheFlushMode {}
pub struct CodeDesc {}
pub enum RelocInfoMode {}
pub struct TVARIABLE<'a, T> {
}
pub struct V8_EXPORT_PRIVATE {}
pub enum void {}
pub struct safepoint_table {}
pub struct Label {}
pub struct InstructionBase {}
pub struct Operand {}
pub struct CodeDesc {}
pub struct V8_EXPORT_PRIVATE {}
pub struct Number {}
pub struct Compiler {}
pub struct ExternalReference {}
pub struct String {}
pub struct YMMRegister {}
pub struct RoundingMode {}
pub struct Assembler {
    buffer: AssemblerBuffer,
}
pub struct YMMRegister {}
pub struct SourcePosition {}
pub enum ICacheFlushMode {}
pub struct Operand {}
pub struct arm {}
pub struct code {}
pub struct x64 {}
pub struct Register {}
pub struct arm64 {}
pub struct AssemblerOptions {}
pub struct FMA_INSTR {}
pub enum ScaleFactor {}
pub struct arm64 {}
pub struct XMMRegister {}
pub struct Register {}
pub struct AssemblerOptions {}
pub struct TVARIABLE<'a, T> {
}
pub struct If {}
pub struct FMA_INSTR {}
pub struct Number {}
pub struct Compiler {}
pub enum ScaleFactor {}
pub struct code {}
pub struct StackFrame {}
pub struct State {}
pub struct List {}
pub struct safepoint_table {}
pub struct WritableJitAllocation {}
pub struct BuiltinUnwindInfo {}
pub struct CodeComment {}
pub struct Register {}
pub struct CodeDesc {}
pub struct BuiltinJumpTableInfo {}
pub struct arm64 {}
pub struct Value {}
pub struct DirectHandle {}
pub struct Operand256 {}
pub struct List {}
pub struct FunctionLiteral {}
pub struct GraphVisualizer {}
pub struct Zone {}
pub struct CodeComment {}
pub struct ExternalReference {}
pub struct AssemblerOptions {}
pub struct MachineOperatorBuilder {}
pub struct State {}
pub struct Label {}
pub struct Value {}
pub struct DirectHandle {}
pub struct FixedArray {}
pub struct Heap {}
pub struct u32 {}
pub struct Code {}
pub struct Code {
}
pub struct DirectHandle<T> {
    value: T,
}
pub struct Assembler {}
pub struct CPURegister {}
pub struct Code {}
pub struct Assembler {}
pub struct safepoint_table {}
pub struct InstructionStream {}
pub struct SafepointTableBuilderBase {}
pub struct RegList {}
pub struct XMMRegister {}
pub struct Register {}
pub struct Value {}
pub struct CPURegister {}
pub struct CallInterfaceDescriptor {}
pub struct CodeDesc {}
pub struct Frame {}
pub struct SafepointTableBuilderBase {}
pub struct AssemblerBase {}
pub struct DirectHandle<T> {
    value: T,
}
pub struct CodeDesc {}
pub struct safepoint_table {}
pub struct WritableJitAllocation {}
pub struct SafepointTableBuilder {}
pub struct BuiltinUnwindInfo {}
pub struct State {}
pub struct AssemblerOptions {}
pub struct MachineOperatorBuilder {}
pub struct ExternalReference {}
pub struct Value {}
pub struct CodeDesc {}
pub struct RelocInfo {}
pub struct safepoint_table {}
pub struct CodeDesc {}
pub struct safepoint_table {}
pub struct Label {}
pub struct Register {}
pub struct RelocInfoWriter {}
pub struct ConstantPool {}
pub struct CodeDesc {}
pub struct AssemblerBase {}
pub struct StackFrame {}
pub struct If {}
pub struct CodeDesc {}
pub struct XMMRegister {}
pub struct TVARIABLE<'a, T> {
}
pub struct Compiler {}
pub struct DirectHandle<T> {
    value: T,
}
pub struct SourcePosition {}
pub struct arm {}
pub struct assembler {}
pub struct RelocInfo {}
pub struct Operand {}
pub struct Label {}
pub struct Position {}
pub struct Register {}
pub struct Instruction {}
pub struct base {}
pub struct V8_EXPORT_PRIVATE {}
pub enum void {}
pub enum RoundingMode {}
pub enum ICacheFlushMode {}
pub struct TVARIABLE<'a, T> {
}
pub struct XMMRegister {}
pub struct V8 {}
pub struct CodeDesc {}
pub struct Isolate {}
pub struct arm64 {}
pub struct Label {}
pub struct Register {}
pub struct instruction {}
pub struct base {}
pub struct AssemblerBase {}
pub enum ICacheFlushMode {}
pub struct safepoint_table {}
pub struct AssemblerOptions {}
pub struct FMA_INSTR {}
pub enum ScaleFactor {}
pub struct arm64 {}
pub struct CodeDesc {}
pub struct DirectHandle<T> {
    value: T,
}
pub struct SourcePosition {}
pub struct Constant {}
pub struct List {}
pub struct ConstantPool {}
pub struct StackFrame {}
pub struct base {}
pub enum RelocInfoMode {}
pub struct Operand256 {}
pub struct Position {}
pub struct CodeDesc {}
pub struct YMMRegister {}
pub struct Builtin {}
pub struct Jump {}
pub enum void {}
pub struct CFunction {}
pub enum Int64Representation {}
pub struct InstructionBase {}
pub struct Code {}
pub struct DirectHandle<T> {
    value: T,
}
pub struct Frame {
}
impl Assembler {
    pub fn new(options: &AssemblerOptions) -> Self {
        Self {
            buffer: AssemblerBuffer {
                offset_: 0,
                size_: 0,
            }
        }
    }
}
impl RelocInfo {
    pub fn wasm_call_tag(&self) -> u32 {
        0
    }
}
impl AssemblerBase {
    pub fn new(options: &AssemblerOptions) -> Self {
        AssemblerBase {}
    }
}
pub struct AssemblerBase {
    offset_: i32,
    size_: i32,
}
pub struct AssemblerBuffer {
    offset_: i32,
    size_: i32,
}

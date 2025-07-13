// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
use std::ptr;

use crate::builtins::builtins_descriptors::Builtins;
use crate::codegen::code_factory::CodeFactory;
use crate::codegen::register_configuration::RegisterConfiguration;
use crate::debug::debug::Debug;
use crate::deoptimizer::deoptimizer::Deoptimizer;
use crate::execution::frame_constants::EntryFrameConstants;
use crate::execution::frames::StackFrame;
use crate::heap::heap_inl::*;
use crate::logging::counters::Counters;
use crate::objects::cell::*;
use crate::objects::foreign::*;
use crate::objects::heap_number::*;
use crate::objects::js_generator::*;
use crate::objects::objects_inl::*;
use crate::objects::smi::*;
use crate::runtime::runtime::*;

pub struct V8 {}

pub struct internal {}

pub struct Address {}

pub struct MacroAssembler {}

pub struct Operand {}

pub struct ExternalReference {}

pub struct Label {}

pub struct Register {}

pub struct ThreadLocalTop {}

pub struct HeapNumber {}

pub struct FrameScope {}

pub struct RootIndex {}

pub struct RegisterConfiguration {}

pub struct Heap {}

pub struct ConstantPoolUnavailableScope {}

pub struct UseScratchRegisterScope { dummy: i32 }

pub struct Code {};

pub struct RegList {}

pub struct FixedArray {}

pub struct FrameAndConstantPoolScope {}

pub struct v8_flags {}

pub struct Operand {}

pub struct DwVfpRegister {}

pub struct AssemblerBase {}

pub struct SaveFPRegsMode {}

pub struct Condition {}

pub struct JSAsyncGeneratorObject {}

pub struct Isolate {
    isolate_address : Address
}

pub struct AbortReason {}

pub struct SharedFunctionInfo {
 kFlagsOffset: i32,
 kTrustedFunctionDataOffset: i32,
 kFormalParameterCountOffset : i32,
 kAgeOffset : i32
}

pub struct JSFunction {
 kSharedFunctionInfoOffset: i32,
 kContextOffset : i32,
 kFeedbackCellOffset : i32,
}

pub struct Tagged<T> {}

pub struct BytecodeArray {
  kFrameSizeOffset : i32,
 kHeaderSize : i32,
  kParameterSizeOffset : i32,
  kIncomingNewTargetOrGeneratorRegisterOffset: i32
}

pub struct FeedbackCell {
 kValueOffset: i32,
}

pub struct FeedbackVector {
 kInvocationCountOffset : i32,
 kOsrStateOffset : i32
}

pub struct InterpreterFrameConstants {
    kBytecodeOffsetFromFp : i32,
 kBytecodeArrayFromFp : i32,
 kBytecodeOffset : i32
}

pub struct SharedFunctionInfo {}

pub struct InterpreterEntryTrampolineMode {}

pub struct StackFrame {}

pub struct Builtins { dummy: i32 }

pub struct SharedFunctionInfo {}

pub struct Map { kBitFieldOffset: i32,
   kInstanceTypeOffset : i32}

pub struct InterpreterData { kBytecodeArrayOffset: i32 }

pub struct FrameDescription {
 registers_offset: i32,
  simd128_registers_offset: i32,
 frame_size_offset: i32,
 continuation_offset: i32,
 frame_content_offset: i32,
 pc_offset: i32,
}

pub struct CallInterfaceDescriptor {}

pub struct Object {}

pub struct Immediate {}

pub struct RegList {}

pub enum Type {}

pub enum InvokeType {}

pub enum JSParameterCount {}

pub enum ExternalReference {}

pub struct base {}

pub struct CodeKind {}

pub struct FeedbackSlot {}

pub struct InstructionStream {}

pub struct Operand {}

pub struct Jump {}

pub struct v8 {

}

pub struct FrameAndConstantPoolScope {}

pub struct Stack {}

pub struct CodeStub {}

pub struct Condition {}

pub struct Label {}

pub struct roots {}

pub struct List {}

pub enum Condition {}

pub enum RegList {}

pub struct AbortReason {}

pub struct SmallVectorError {}

pub struct FixedDoubleArray {}

pub struct OpIndex {}

pub struct Expression {}

pub struct Call {}

pub struct ConstantPoolUnavailableScope {}

pub struct TaggedField<T, const OFFSET: usize>;

pub struct DeoptimizeKind {}

pub struct Object {}

pub struct String {}

pub struct DirectHandle<T> {}

pub struct CallApiCallbackMode {}

pub struct StackFrameIteratorForProfiler {}

pub enum GCCallbacksInSafepoint {}

pub struct AccessorPair {}

pub struct base {}

pub struct TaggedField<T, const OFFSET: usize>;

pub struct Object {}

pub enum class GCConfigMarkingType {
}

impl Builtins {
  pub fn Generate_Adaptor(masm: &MacroAssembler, formal_parameter_count: i32, address: Address) {
        todo!()
  }
}

pub enum ArgumentsElementType {
    kRaw,
    kHandle
}

pub struct RootIndex {}

impl MacroAssembler {
 fn StackOverflowCheck(&mut self, r0: Register, scratch: Register, stack_overflow: &Label) {}
  fn Jump(&mut self, label: &Label){}
    fn EnforceStackAlignment(&mut self) {}
   fn Move(&mut self, root_register: Register, isolate_address: ExternalReference) {}
  fn Ret(&mut self) {}
   fn Push(&mut self, closure: Register){}
  fn ldr(&mut self, value: Register, mem_operand: crate::builtins::arm::builtins_arm::MemOperand){}
   fn Pop(&mut self, scratch: Register){}
    fn add(&mut self, rsp: Register, scratch: Register, arg: Operand){}
   fn PrepareCallCFunction(&mut self, i: i32) {}
  fn CallCFunction(&mut self, create: ExternalReference, i: i32, set_isolate_data_slots: SetIsolateDataSlots) {}
    fn sub(&mut self, dst: Register, src1: Register, src2: Operand){}
   fn LoadCodeInstructionStart(&mut self, reg: Register, reg1: Register){}
    fn ldrb(&mut self, r4: Register, mem_operand: crate::builtins::arm::builtins_arm::MemOperand){}
  fn PushRoot(&mut self, kUndefinedValue: RootIndex){}
  fn TailCallRuntime(&mut self, runtime_kthrowstackoverflow: Runtime){}
  fn Trap(&mut self){}
  fn JumpIfEqual(&mut self, dst: Register, state: wasm::JumpBuffer::StackState, label: &Label){}
    fn Jump(&mut self, reg: Register){}
    fn str(&mut self, dst: Register, mem_operand: crate::builtins::arm::builtins_arm::MemOperand){}
}

pub enum SetIsolateDataSlots{
    kNo
}

pub enum AbortReason {}

pub enum StackLimitKind {}

pub struct WasmFrame {}

pub struct WasmImplicitArgs {}

pub enum SaveFPRegsMode {}

pub struct SmiCheck {}

pub struct Operand {}

pub struct NativeContextSlot {}

pub struct CodeObject {}

pub enum InvokeType {}

pub struct MemOperand{
}
impl MemOperand{
    fn new(a : Register, v: i32) -> Self{ MemOperand{}}
}

pub struct MemOperand {}

pub struct FieldMemOperand {}
impl FieldMemOperand {
    fn new(reg: Register, offset : i32) -> Self{ FieldMemOperand{} }
}

pub struct SaveOptions {}

pub struct SlotSet {}

pub struct base {}

pub struct CallApiCallbackDescriptor {}

pub struct base {}

pub struct TaggedField<T, const OFFSET: usize>;

pub struct JSFunction {}

pub struct SharedFunctionInfo {}

pub struct CallOrConstructMode {}

pub struct ConvertReceiverMode {}

pub struct Builtin {}

pub struct interpreter {}

pub struct interpreter {}

pub struct interpreter {}

pub struct Stack {}

pub struct interpreter {}

impl MacroAssembler {
    fn Check(eq: Condition, kExpectedBaselineData: AbortReason){}
    fn stop(&mut self){}
    fn b(&mut self, eq: Condition, not_baseline: &Label){}
    fn CheckConstPool(&mut self, b: bool, b1: bool){}
}

pub struct JavaScript {}

pub struct FrameScope {}

pub struct interpreter {}

pub struct internal {}

pub struct WasmContinuationObject {}

pub struct Label {}

pub struct Wasm {
    fn all() -> Vec <OpIndex>{}
}

pub struct InstructionOperand {}

pub struct Tagged {}

pub struct StackSwitchFrameConstants {}

pub struct MemOperand {}

pub struct WasmSuspendObject {}

pub struct InstructionBase {}

pub struct JSGeneratorObject {}

pub struct JSProxy {}

pub struct Load {}

pub struct Internal {}

pub struct FrameDescription {}

pub struct JSBoundFunction {}

pub struct RegList {}

pub struct String {}

pub struct MacroAssembler {}

pub struct Load {}

pub struct MemOperand {}

pub struct This {}

pub struct base {}

pub struct Bytecode {}

pub struct FixedArray {}

pub struct Label {}

impl base {
  pub fn bits(hi: Tagged<Smi>, lo: Tagged<Smi>) -> Result<(), String> {
      Ok(())
  }
}

pub struct InterpreterFrameConstants {}

impl Tagged {
  pub fn size() -> usize {
      todo!()
  }
  pub fn FieldSizeLog2() -> i32 {
      todo!()
  }
    pub fn SmiCheck(&self){}
   pub fn SmiUntag(&mut self){}
   pub fn field_offset(i: i32) -> i32 {todo!()}
  pub fn cast<T>() -> Tagged<T> {
    todo!()
  }
}

impl ExternalReference {
 fn Create(address: Address) -> ExternalReference{
        ExternalReference{}
 }
  fn Create(thread_in_wasm_flag_address: IsolateAddressId, isolate: &Isolate) -> Self{ ExternalReference{} }
   fn address_of_interpreter_entry_trampoline_instruction_start(isolate: &Isolate) -> Self{ExternalReference{}}
 pub fn invoke_accessor_getter_callback() -> Self {ExternalReference{}}
  fn wasm_grow_stack() -> Self { ExternalReference{} }
  fn isolate_address() -> Self{ExternalReference{}}
      fn invoke_function_callback(mode: CallApiCallbackMode) -> ExternalReference {todo!()}
 pub fn new_deoptimizer_function() -> Self {todo!()}
        fn bytecode_size_table_address() -> Self{ todo!() }
      fn debug_hook_on_function_call_address(isolate: &Isolate) -> Self{ ExternalReference{} }
     fn debug_suspended_generator_address(isolate: &Isolate) -> Self{ExternalReference{}}
       fn Create(runtime_kdebugonfunctioncall: Runtime, isolate: &Isolate) -> Self{ ExternalReference{} }
      fn kUnwindAndFindExceptionHandler() -> Self {ExternalReference{}}
     fn kIsOnCentralStackFlagAddress() -> Self{ ExternalReference{} }
     fn wasm_switch_to_the_central_stack() -> Self{ ExternalReference{} }
     fn wasm_switch_from_the_central_stack() -> Self{ ExternalReference{} }
       fn address_of_profiler_array_sample_address() -> ExternalReference { ExternalReference{} }
}

enum IsolateAddressId {
    kContextAddress,
    kCEntryFPAddress,
    kJSEntrySPAddress,
  kIsOnCentralStackFlagAddress,
          kExceptionAddress,
        kPendingHandlerContextAddress,
        kPendingHandlerEntrypointAddress,
        kPendingHandlerFPAddress,
        kPendingHandlerSPAddress,

        kFastCCallCallerFP,
        kFastCCallCallerPC
}

enum DeoptimizeKind {}
enum CodeKind {}

impl JSProxy {
  pub fn kDataOffset() -> i32{ 1 }
}

impl MacroAssembler {
    fn JumpIfNotSmi(&mut self, r1: Register, non_callable: &Label){}
    fn Ldr(&mut self, r1: Register, mem_operand: MemOperand){}
   fn TailCallBuiltin(&mut self, call_function: Builtin){}
   fn Cmp(&mut self, scratch: Register, i: i32){}
}

impl Load{
    fn CodeInstructionStart(_reg: Register, _r: Register){}
}
pub struct Load{}
impl MacroAssembler {
 fn LoadCodeInstructionStart(_reg: Register, _r: Register){}
    fn LoadObjectType(&mut self, scratch: Register, target: Register, scratch1: Register, js_proxy_type: Type) {}
   fn TailCallBuiltin(&mut self, kCallProxy: Builtin, eq: Condition){}
    fn CompareInstanceTypeRange(&mut self, map: Register, instance_type: Register, scratch: Register, first_callable_js_function_type: Type, last_callable_js_function_type: Type) {}
     fn Cmp(&mut self, scratch: Register, js_proxy_type: Type){}
     fn CallRuntime(&mut self, runtime_kthrowcallednoncallable: Runtime){}
 fn EnterFrame(&mut self, construct: StackFrame){}
 fn LoadGlobalProxy(&mut self, r3: Register){}
  fn TestIfSmi(&mut self, tmp: Register){}
   fn BranchOnSmi(&mut self, smivalue: Register, labels: &Label){}
    fn cmp(&mut self, reg: Register, target: Register, cond: Condition){}
     fn mov(&mut self, dst: Register, value: Operand, leave_cc: Condition, ge: Condition){}
  fn CmpObjectType(r3: Register, r31: Register, r4: Register, first_js_receiver_type: Type) {}
    fn JumpIfSmi(r0: Register, use_receiver: &Label){}
        fn GetLabelAddress(&mut self, label_code: Register, lab: &Label){}
         fn Branch(&mut self, branch: &Condition, label: &Label){}
 fn Assert(&mut self, eq: Condition, missing_bytecode_array: AbortReason){}
 fn PushStandardFrame(&mut self, closure: Register){}
  fn LeaveFrame(&mut self, frame: StackFrame) {}
         fn ComputeCodeStartAddress(_new_target: Register, _data: Register, _r0: Register, number: u32, reg3: Operand){}
        fn Ret(&mut self) {}
   fn JumpCodeObject(_r: Register){}
 fn StoreReturnAddressAndCall(_target_fun: Register){}
    fn LoadNativeContextSlot(dst: Register, slot: i32){}
       fn pop(&mut self, _r0: Register) {}
        fn Push(js_entry_fp: Register, jslr: Register, jssp: Register) {}
        fn Sub(&mut self, stack_pointer: Register, kNumCalleeSaved: i32){}
   fn SmiTag(&mut self, reg: Register, registers3: Register){}
        fn LoadIsolateField(tmp: Register, top_of_stack_address: IsolateAddressId){}
    fn Throw(&mut self) {}
}

impl JsFunction {
  pub fn Size() -> i32{
        8
  }
}

impl Tagged{
 pub fn from_smi(value:Smi) -> Self { todo!() }
}

impl Operand {
    fn Zero() -> Operand {Operand{}}
    fn HashSeedFromRelocInfo(has_position: i32, i: i32) -> Self {Operand{}}
  fn from(immediate: Immediate) -> Self{ Operand{}}
  fn Register(r5: Register) -> Operand{Operand{}}
   fn SmiUntag(name: Register) -> Operand{ Operand{}}
   fn UntagSmi() -> Operand {Operand{}}
}

pub struct Immediate {}
impl Immediate {
    fn New(hash: u32) -> Self{ Immediate{}}
}

impl Builtins {
 pub fn CallFunction(mode: ConvertReceiverMode) -> Builtin{ Builtin{dummy:0}}
 pub fn kContinueToJavaScriptBuiltin() -> Builtin{ Builtin{dummy:0}}
  fn kConstruct() -> Builtin{ Builtin{dummy:0}}
   fn Call() -> Builtin{ Builtin{dummy:0}}
 fn CallFunction(mode:ConvertReceiverMode) -> Builtin{ Builtin{dummy:0}}
 pub fn kCallConstructedNonConstructable() -> Builtin{Builtin{dummy:0}}
  fn kCallProxy() -> Builtin{ Builtin{dummy:0}}
 pub fn kJSBuiltinsConstructStub() -> Builtin{ Builtin{dummy:0}}
     fn kStackOverflow() -> Builtin{ Builtin{dummy:0}}
       fn CallFunction(mode: ConvertReceiverMode) -> Builtin {Builtin {dummy: 0}}
     fn kCall() -> Builtin{Builtin{dummy : 1}}
     fn kConstructForwardVarargs() -> Builtin{Builtin{dummy : 1}}
 fn kJSConstructStubGeneric() -> Builtin{ Builtin{dummy:0}}
    fn kCallBoundFunction()-> Builtin { todo!() }
        fn CallWithSpread() -> Builtin{todo!()}
 fn kJSConstructStub()-> Builtin {Builtin{dummy : 0}}
      fn kNotifyDeoptimized()-> Builtin {Builtin{dummy: 0}}
         fn kInstallBaselineCode()-> Builtin {Builtin{dummy: 0}}
     fn kV8Serialize() -> Builtin{ Builtin{dummy: 0}}
}

struct External {}

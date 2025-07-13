// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code, unused_variables)]

use std::sync::Mutex;
pub struct AbortReason {}
pub struct Frame {}
pub struct Isolate {
    stack_is_iterable_:bool,
    heap_:Heap,
}
pub struct Heap {
    interpreter_entry_return_pc_offset:i32
}
pub struct Handle<T> {}
pub struct SharedFunctionInfo {}
pub struct Address {}
pub struct CallInterfaceDescriptor {}
pub struct Builtins {}
pub struct SealHandleScope {}
pub struct RootIndex {}
pub struct CodeKind {}
pub struct StackFrame {}
pub struct JavaScriptFrame {}
pub struct ExternalReference {}
pub struct Operand {}
pub struct Deoptimizer {}
pub struct CodeObject {}
pub struct BytecodeArray {}
pub struct RegisterConfiguration {}
pub struct FeedbackVector {}
pub struct FixedArray {}
pub struct FieldType {}
pub struct JSFunction {}
pub struct Map {}
pub struct AbortReason {}
pub struct DeoptimizationData {}
pub struct JSToWasmWrapperFrameConstants {}
pub struct List {}
pub struct ExternalPointerHandle {}
pub struct ExternalPointerHandleScope {}
pub struct Vector {}
pub struct ThreadLocalTop {}
pub struct TurboshaftGraph {}
pub struct Instruction {}
pub struct Bytecode {}
pub struct AbortReason {}
pub struct TaggedField {dummy:i32}
pub struct GetResult{}
pub struct CPURegister {}
pub struct FieldMemOperand {}
pub struct Jump {}
pub struct Immediate{value:i32}
pub struct Operand{value:i32}

// From /home/kathirks_gc/v8_go/archive/codebase/src/execution/ppc/frame-constants-ppc.h
pub struct Simd128RegList {
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/builtins-regexp.cc
pub struct Arguments {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct JavaScript {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Use {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct JavaScript {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/stress-scavenge-observer.h
pub struct code {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct StackGuard {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct UseScratchRegisterScope { dummy: i32 }
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
struct TaggedField<T, const OFFSET: usize>;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/ia32/builtins-ia32.cc
pub struct ExternalReference;
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct FrameScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/module.h
pub struct Set {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/live-object-range.h
pub struct iterator {
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct UseScratchRegisterScope { dummy: i32 }
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/cppgc/heap-growing.h
pub enum class GCConfigMarkingType {
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}

pub enum CallOrConstructMode {
    kCall,
    kConstruct,
}

pub enum InvokeType {
    kCall,
}

pub enum ConvertReceiverMode {
    kNullOrUndefined,
    kNotNullOrUndefined,
    kAny,
}

pub enum Ordering {
    SeqCst,
}

pub enum SaveFPRegsMode {
    kIgnore,
    kSave,
}

pub enum InterpreterPushArgsMode {
    kArrayFunction,
    kWithFinalSpread,
    kOther,
}

pub enum DeoptimizeKind {
    kEager,
    kLazy,
}

pub enum SmiCheck {
    kNotNeeded,
}

struct StackSwitchFrameConstants {
    const kImplicitArgOffset: i32,
    const kResultArrayOffset: i32,
    const kNumSpillSlots: i32,
}

impl StackSwitchFrameConstants{
    const kImplicitArgOffset: i32 = 0;
    const kResultArrayOffset: i32 = 0;
    const kNumSpillSlots: i32 = 0;

}
struct JSToWasmWrapperFrameConstants {
    const kNumParameters: i32,
    const kWrapperBufferOffset: i32,
    const kResultArrayParamOffset:i32,
     const kImplicitArgOffset: i32,

    const kWrapperBufferParamStart: i32,
    const kWrapperBufferParamEnd: i32,
    const kWrapperBufferStackReturnBufferSize: i32,
    const kWrapperBufferStackReturnBufferStart: i32,
     const kWrapperBufferReturnCount: i32,

    const kWrapperBufferRefReturnCount: i32,

    const kWrapperBufferCallTarget: i32,
    const kWrapperBufferSigRepresentationArray :i32,

}

impl JSToWasmWrapperFrameConstants{
        const kNumParameters: i32 = 0;
        const kWrapperBufferOffset: i32=0;
        const kResultArrayParamOffset: i32=0;
        const kImplicitArgOffset: i32 = 0;
        const kWrapperBufferParamStart: i32=0;
        const kWrapperBufferParamEnd: i32=0;
        const kWrapperBufferStackReturnBufferSize: i32=0;
        const kWrapperBufferStackReturnBufferStart: i32=0;
        const kWrapperBufferReturnCount:i32 = 0;

        const kWrapperBufferRefReturnCount: i32 = 0;
        const kWrapperBufferCallTarget: i32=0;
          const kWrapperBufferSigRepresentationArray :i32=0;

}
pub struct DirectHandle<T> {}
pub struct Managed<T> {}
pub struct FrameAndConstantPoolScope {}
pub struct ConstantPoolUnavailableScope {}

//From src/codegen/ppc/macro-assembler-ppc.h
pub struct MacroAssembler {}

impl MacroAssembler {
    fn isolate(&mut self) -> &Isolate{ todo!()}
}
pub struct DoubleRegister {
   code_: i32,
}

impl DoubleRegister {
    fn from_code(code: i32) -> Self {
        DoubleRegister { code_: code }
    }
}

pub const kDoubleRegZero: DoubleRegister = DoubleRegister { code_: 0 };

pub struct Condition {}
pub enum ComparisonResult {}

pub struct String {}
pub struct Object {}
pub struct Load {}
pub struct AbortReason {}
pub struct HeapObject {}
pub struct Smi {}
pub struct Operand {}
pub struct Operand{}
pub struct Label{}
pub struct MemOperand{offset : i32}
pub struct AbortReason{}
pub struct StackLimitKind {}
pub struct SaveOptions {}
pub struct Type {}
pub struct StringSet {}
pub enum IterationKind {}
pub struct JSArray {}
pub struct DeoptimizeKind {}
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/module.h
pub struct Set {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct interpreter {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct interpreter {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Load {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Load {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct base {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Context {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/page-metadata.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/s390/builtins-s390.cc
struct HeapNumber{number : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/x64/builtins-x64.cc
pub struct internal {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct FrameScope {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Call {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Context {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct If {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Label {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct If {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Throw {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct If {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct If {};
// From /home/kathirks_gc/v8_go/archive/codebase/src/builtins/loong64/builtins-loong64.cc
pub struct Object {}

#[derive(PartialEq, Copy, Clone)]
pub struct Register {
    code_: i32,
}

impl Register {
    const no_reg: Self = Register{ code_: -1 };

    fn from_code(code: i32) -> Self {
        Register{ code_: code }
    }
    fn code(&self) -> i32 {
        self.code_
    }
}

struct WasmHandleStackOverflowDescriptor {}

impl WasmHandleStackOverflowDescriptor {
    fn FrameBaseRegister() -> Register {
        todo!()
    }
    fn GapRegister() -> Register {
        todo!()
    }
}

impl Isolate {
    fn builtins(&mut self) -> &mut Builtins {
        todo!()
    }
}

impl Heap {
        fn interpreter_entry_return_pc_offset(&mut self) -> Smi {
        todo!()
    }
    fn SetConstructStubInvokeDeoptPCOffset(&mut self, v:i32){
        todo!()
    }
     fn SetInterpreterEntryReturnPCOffset(&mut self, v:i32){
        self.interpreter_entry_return_pc_offset = v;
    }
}

impl MacroAssembler {
    fn new() -> Self {
        MacroAssembler {}
    }
    fn isolate(&mut self) -> &mut Isolate {
        todo!()
    }
    fn pc_offset(&mut self) -> i32 {
        todo!()
    }
    fn SetCodeTargetAddressAt(&mut self, arg1: Register){
        todo!()
    }
    fn Abort(&mut self, reason: AbortReason){}
    fn PrepareCallCFunction(&mut self, arg1:i32, arg2:Register){}
     fn RestoreFPAndRet(){}
     fn StackOverflowCheck(&mut self, r6: Register, ip: Register, stack_overflow: &Label) {todo!()}
     fn CallRuntime(&mut self, arg1:Runtime, arg2: i32){}
      fn LoadStackLimit(&mut self, ip: Register, stack_limit: StackLimitKind,reg:Register){}
    fn AddS64(&mut self, r6: Register, ip: Register, op: i32,r:Register){}
    fn CallCFunction(&mut self, r7: ExternalReference, arg1: i32){}
     fn LoadU64(&mut self, r7: Register, arg1: MemOperand,reg:Register){}
    fn Jump(&mut self, arg1:Register){}
    fn SmiTag(&mut self, r0: Register, kInterpreterBytecodeOffsetRegister: Register){}
    fn AddS64(&mut self, r1: Register, r5: Register, op: Operand) {}
    fn Trap(&mut self){}
    fn Push(&mut self,r3: Register){}
    fn LoadTaggedField(&mut self, reg:Register,field_mem_operand: MemOperand, scratch:Register){}
    fn LeaveFrame(&mut self, state: StackFrame){}
    fn LoadTaggedField(
        &mut self,
        feedback_cell: Register,
        field_mem_operand: FieldMemOperand,
        Register: Register,
    ) {
    }
    fn Pop(&mut self, reg:Register){}
     fn Pop(
        &mut self,
        reg1: Register,
        reg2: Register
    ) {
    }
      fn mflr(&mut self, r0: Register) {
    }
    fn mtlr(&mut self, r0: Register) {
    }
      fn LoadRootRelative(
        &mut self,
        dst: Register,
        src: i32
    ) {
    }
         fn LoadU32(&mut self,dst: Register,src: MemOperand,reg:Register){}
         fn subi(&mut self,reg: Register, reg2:Register,op:Operand){}
         fn andi(&mut self, r0: Register, r4: Register, op: Operand){}
         fn extsb(&mut self, reg: Register, reg2:Register){}
         fn lbzx(&mut self, dst:Register, reg:MemOperand){}
         fn mtctr(&mut self, reg: Register){}
         fn shift_left_u64(&mut self, reg: Register, reg1: Register, op: Operand){}
         fn add(&mut self, dst:Register, reg1: Register, reg2: Register){}
          fn LoadCodeInstructionStart(&mut self, scratch: Register,kJavaScriptCallCodeStartRegister:Register) {}
           fn mtctr_with_update(&mut self, scratch: Register) {}
    fn AssertFunction(&mut self, r4: Register){}
    fn StoreU64(&mut self, reg: Register, MemOperand: MemOperand){
        todo!()
    }
        fn cmpi(&mut self, reg: Register,  immediate: Operand) {}
    fn LoadU64WithUpdate(&mut self, ip: Register, memop: MemOperand){
        todo!()
    }
    fn Assert( eq: Condition, reason: AbortReason){}
    fn li(&mut self,r: Register,op: Operand){}
         fn PrepareCallCFunction(&mut self, c_args: i32, r0:Register){}
          fn LoadGlobalProxy(&mut self, reg: Register){}
         fn LoadIsolateField(&mut self, ip: Register, kStackIsIterable: IsolateFieldId){}
         fn bne(&mut self, target: &Label,cr:Condition) {}
    fn cmpi(&mut self,scratch_1: Register,p1: Operand,reg:Register){}
          fn IsObjectType(&mut self, data: Register, scratch: Register,Register1:Register, code_type: Type){}
       
}

pub enum StackFrame {
    INTERNAL,
    JAVASCRIPT,
    WASM_DEBUG_BREAK,
    ENTRY,
    CONSTRUCT,
    API_CALLBACK_EXIT,
       BUILTIN_EXIT,
            FAST_CONSTRUCT, INTERPRETED,
             BASELINE,
               STACK_SWITCH,

               WASM,

                  OUTERMOST_JSENTRY_FRAME,
                   INNER_JSENTRY_FRAME,

}

impl StackFrame {
   const BASELINE : Self = StackFrame::BASELINE;
   const INTERPRETED : Self = StackFrame::INTERPRETED;

    fn MANUAL() -> Self { StackFrame::INTERNAL }
    fn API_CALLBACK_EXIT() -> Self {StackFrame::API_CALLBACK_EXIT}
    fn TypeToMarker(arg:Self) -> Tagged<Object>{ Tagged{}}
     const EXIT: Self = StackFrame::EXIT;
   const CONSTRUCT_ENTRY: Self = StackFrame::CONSTRUCT_ENTRY;
     const ENTRY: Self = StackFrame::ENTRY;
}
pub struct Operand {};
impl Operand {
    fn Zero() -> Self {
        Operand{value : 0}
    }
    fn new(value : i32) -> Self {
        Operand{value : value}
    }
}

pub struct AbortReason {}
pub struct Load{}

pub struct ExternalReference{}

impl ExternalReference {
    fn Create(address: Address) -> Self {
        ExternalReference{}
    }
    fn Create(address: IsolateAddressId, isolate: &Isolate) -> Self {
        ExternalReference{}
    }
      fn Create(address: Runtime, isolate: &Isolate) -> Self {
        ExternalReference{}
    }
        fn Create(reason: AbortReason) -> Self {
        ExternalReference{}
    }

        fn baseline_pc_for_next_executed_bytecode() -> Self {
        ExternalReference {}
    }
    fn debug_hook_on_function_

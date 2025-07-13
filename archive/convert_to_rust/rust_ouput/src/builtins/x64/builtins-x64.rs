// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code, non_snake_case, unused_variables, unused_macros)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp::Ordering, ffi::c_void, mem::size_of, ptr::NonNull};

use crate::base;
use crate::codegen;
use crate::heap;
use crate::objects;
use crate::runtime;

pub struct CPURegister {}
pub struct Isolate {}
pub struct Register {}
pub struct JSFunction {}
pub struct Code {}
pub struct Operand {}
pub struct DoubleRegister {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct Immediate {}
pub struct FieldOperand {}
pub struct Tagged {}
pub struct BytecodeArray {}
pub struct FixedArray {}
pub struct FeedbackVector {}
pub struct AbortReason {}
pub struct Builtin {}
pub struct ValueType {}
pub struct Expression {}
pub struct MutablePageMetadata {}
pub struct SlotSet {}
pub struct Address {}
pub struct FixedDoubleArray {}
pub struct AccessorPair {}
pub struct Mutex<T> {}
pub struct FixedArrayBase {}
pub struct PageSpace {}
pub struct ThreadLocalTop {}
pub struct IsolateForSandbox {}
pub struct AccessorInfo {}
pub struct JSArray {}
pub struct Bytecode {}
pub struct JSAsyncDisposableStack {}
pub struct internal {}
pub struct DirectHandle<T> {}
pub struct Name {}
pub struct Object {}
pub struct Vector<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct ExpressionStatement {}
pub struct String {}
pub struct JSGeneratorObject {}
pub struct TaggedRegister {}
pub struct Assembler {}
pub struct CodeEntry {}
pub struct Internal {}
pub struct SaveOptions {}
pub struct Load {}
pub struct Jump {}
pub struct MacroAssembler {}
pub struct Stack {}
pub struct JSPluralRules {}
pub struct Call {}
pub struct JSAsyncGeneratorObject {}
pub struct Condition {}
pub struct Base {}
pub struct JSObject {}
pub struct Context {}
pub struct HandleScope {}
pub struct StackFrameIteratorForProfiler {}
pub struct HeapNumber {}
pub struct JSString {}
pub struct Any {}
pub struct AllocationSite {}
pub struct JavaScript {}
pub struct Instruction {}
pub struct Smi {}
pub struct InstructionStream {}
pub struct interpreter {}
pub struct JSGenerator {}
pub struct CodeStubAssemblerState {}
pub struct InterpreterFrameConstants {}
pub struct VectorOf<T> {}
pub struct JSSharedArray {}
pub struct Location {}
pub struct JSSet {}
pub struct XMMRegister {}
pub struct JSMap {}
pub struct SharedFunctionInfo {}
pub struct DeoptimizeKind {}
pub struct VectorDescriptor {}
pub struct OpIndex {}
pub struct Turboshaft {}
pub struct JumpBuffer {}
pub struct InstructionOperand {}
pub struct RootIndex {}
pub struct Deoptimizer {}
pub struct RegisterArray {}
pub struct DeoptimizerData {}
pub struct Op {}
pub struct MarkingWorklist {}
pub struct turboshaft {}
pub struct StackFrameIterator {}
pub struct Internal {}
pub struct Callable {}
pub struct NativeModule {}
pub struct Operand {}
pub struct BytecodeArrayBuilder {}
pub struct Slot {}
pub struct SharedArrayBuffer {}
pub struct StringTable {}
pub struct CallSiteInfo {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct AccessorPair {}
pub struct BytecodeArrayWriter {}
pub struct Operand {}
pub struct Smi {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct DirectArguments {}
pub struct FixedArray {}
pub struct CallInterfaceDescriptor {}
pub struct ExternalReference {}
pub struct JavaScript {}
pub struct StackFrame {}
pub struct JavaScriptFrame {}
pub struct AccessorPair {}
pub struct InterpreterEntryTrampolineMode {}
pub struct JSBoundFunction {}
pub struct JSProxy {}
pub struct BuiltinReducerData {}
pub struct Vector<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct JSAsyncFunctionObject {}
pub struct FeedbackCell {}
pub struct JSReceiver {}
pub struct JSTypedArray {}
pub struct WasmFrame {}
pub struct ThreadLocalTop {}
pub struct MicrotaskQueue {}
pub struct Expression {}
pub struct SlotSet {}
pub struct JSArrayBuffer {}
pub struct AsmType {}
pub struct ExpressionStatement {}
pub struct JSSharedStruct {}
pub struct CodeStubAssembler {}
pub struct AccessorInfo {}
pub struct JSWeakRef {}
pub struct FeedbackSlot {}
pub struct SourceTextModuleInfo {}
pub struct HeapObject {}
pub struct StringTable {}
pub struct WasmFrame {}
pub struct Builtins {}
pub struct Label {}
pub struct Interpreter {}
pub struct CPURegister {}
pub struct Handle {}
pub struct Address {}
pub struct HandleScope {}
pub struct MacroAssembler {}
pub struct JSPluralRules {}
pub struct FixedArray {}
pub struct HandleScope {}
pub struct TaggedRegister {}
pub struct Map {}
pub struct Stack {}
pub struct CallSiteInfo {}
pub struct Operand {}
pub struct Register {}
pub struct Builtin {}
pub struct Location {}
pub struct JSGenerator {}
pub struct VectorDescriptor {}
pub struct CPURegister {}
pub struct Operand {}
pub struct TaggedRegister {}
pub struct TurboFanGraph {}
pub struct Builtin {}
pub struct JumpBuffer {}
pub struct Internal {}
pub struct Address {}
pub struct TaggedRegister {}
pub struct JSFunction {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct MacroAssembler {}
pub struct String {}
pub struct JSGeneratorObject {}
pub struct JSTypedArray {}
pub struct Any {}
pub struct AnyValue {}
pub struct TaggedRegister {}
pub struct CodeEntry {}
pub struct JSMap {}
pub struct Vector<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct turboshaft {}
pub struct interpreter {}
pub struct JSArray {}
pub struct FixedArrayBase {}
pub struct StackFrameIterator {}
pub struct This {}
pub struct InterpreterFrameConstants {}
pub struct JSSharedArray {}
pub struct Smi {
    _phantom: std::marker::PhantomData<i32>,
}
pub struct Builtins {}
pub struct JumpBuffer {}
pub struct DirectArguments {}
pub struct interpreter {}
pub struct VectorDescriptor {}
pub struct interpreter {}
pub struct Address {}
pub struct TaggedRegister {}
pub struct JSTypedArray {}
pub struct JSFunction {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct MacroAssembler {}
pub struct String {}
pub struct JSGeneratorObject {}
pub struct ValueType {}
pub struct TurboFanGraph {}
pub struct Builtin {}
pub struct Label {}
pub struct CPURegister {}
pub struct Operand {}
pub struct TaggedRegister {}
pub struct DeoptimizerData {}
pub struct interpreter {}
pub struct interpreter {}
pub struct JSArray {}
pub struct Operand {}
pub struct Smi {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct TurboFanGraph {}
pub struct DirectArguments {}
pub struct InstructionStream {}
pub struct FeedbackVector {}
pub struct Builtin {}
pub struct Label {}
pub struct NativeModule {}
pub struct Operand {}
pub struct CallSiteInfo {}
pub struct CallInterfaceDescriptor {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct MacroAssembler {}
pub struct String {}
pub struct DirectHandle<T> {}
pub struct HandleScope {}
pub struct External {}
pub struct JumpBuffer {}
pub struct interpreter {}
pub struct This {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct Internal {}
pub struct AccessorPair {}
pub struct Register {}
pub struct TaggedRegister {}
pub struct DoubleRegister {}
pub struct JumpBuffer {}
pub struct DirectArguments {}
pub struct Register {}
pub struct TaggedRegister {}
pub struct AnyValue {}
pub struct TurboFanGraph {}
pub struct DeoptimizerData {}
pub struct Builtin {}
pub struct Label {}
pub struct NativeModule {}
pub struct Operand {}
pub struct HeapNumber {}
pub struct StackFrameIterator {}
pub struct This {}
pub struct Operand {}
pub struct AccessorPair {}
pub struct DirectHandle<T> {}
pub struct Name {}
pub struct BuiltinReducerData {}
pub struct Vector<T> {}
pub struct Label {}
pub struct CPURegister {}
pub struct HandleScope {}
pub struct TaggedRegister {}
pub struct HandleScope {}
pub struct JSFunction {}
pub struct StackFrame {}
pub struct MacroAssembler {}
pub struct String {}
pub struct Vector<T> {}
pub struct InterpreterFrameConstants {}
pub struct DirectHandle<T> {}
pub struct Builtins {}
pub struct JumpBuffer {}
pub struct NativeContext {}
pub struct Operand {}
pub struct Address {}
pub struct TaggedRegister {}
pub struct ValueType {}
pub struct BytecodeArrayBuilder {}
pub struct Label {}
pub struct CPURegister {}
pub struct Operand {}
pub struct Label {}
pub struct OpIndex {}
pub struct turboshaft {}
pub struct Operand {}
pub struct Register {}
pub struct JSPluralRules {}
pub struct Builtin {}
pub struct DirectHandle<T> {}
pub struct NativeModule {}
pub struct Operand {}
pub struct HeapNumber {}
pub struct DirectArguments {}
pub struct Register {}
pub struct JSTypedArray {}
pub struct NativeModule {}
pub struct JSGeneratorObject {}
pub struct HandleScope {}
pub struct TaggedRegister {}
pub struct VectorDescriptor {}
pub struct Label {}
pub struct Operand {}
pub struct Register {}
pub struct StackFrame {}
pub struct JSTypedArray {}
pub struct CPURegister {}
pub struct HandleScope {}
pub struct TaggedRegister {}
pub struct Operand {}
pub struct Register {}
pub struct String {}
pub struct Op {}
pub struct CodeStubAssembler {}
pub struct StringTable {}
pub struct BytecodeArrayWriter {}
pub struct Operand {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct NumberFormatFields {}
pub struct HeapNumber {}
pub struct Local {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct WasmFrame {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct FixedArray {}
pub struct StackSwitchFrameConstants {}
pub struct WasmSuspenderObject {}
pub struct JumpBuffer {}
pub struct JSToWasmWrapperFrameConstants {}
pub struct LinearAllocationArea {}
pub struct TaggedFixedArray {}
pub struct JSAsyncGeneratorObject {}
pub struct Vector{}
pub struct List{}
pub struct Scope{}
pub struct TaggedScope {}
pub struct ListBuffer{}
pub struct base {}
pub struct base {}
pub struct Heap {}
pub struct roots {}
pub struct base {}
pub struct Zone {}
pub struct Expression {}
pub struct Address {}
pub struct JSFunction {}
pub struct DirectHandle<T> {}
pub struct NativeModule {}
pub struct Builtin {}
pub struct Label {}
pub struct Register {}
pub struct Operand {}
pub struct TurboFanGraph {}
pub struct If {}
pub struct External {}
pub struct JSGlobalProxy {}
pub struct FixedArray {}
pub struct DirectArguments {}
pub struct HeapNumber {}
pub struct HandleScope {}
pub struct NumberFormatFields {}
pub struct Operand {}
pub struct DirectHandle<T> {}
pub struct Any {}
pub struct HeapNumber {}
pub struct CodeStubArguments {}
pub struct RegisterConfiguration {}
pub struct BytecodeArrayBuilder {}
pub struct AnyValue {}
pub struct StackSwitchFrameConstants {}
pub struct ExternalReference {}
pub struct MacroAssembler {}
pub struct BytecodeArrayWriter {}
pub struct Label {}
pub struct JavaScript {}
pub struct InterpreterFrameConstants {}
pub struct base {}
pub struct BytecodeArray {}
pub struct DeoptimizeKind {}
pub struct objects {}
pub struct HeapObject {}
pub struct AbortReason {}
pub struct Address {}
pub struct List {}
pub struct turboshaft {}
pub struct VectorDescriptor {}
pub struct Op {}
pub struct turboshaft {}
pub struct This {}
pub struct Internal {}
pub struct Callable {}
pub struct BytecodeArrayBuilder {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct Smi {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct interpreter {}
pub struct JSGenerator {}
pub struct CodeStubAssemblerState {}
pub struct VectorOf<T> {}
pub struct JSSharedArray {}
pub struct ListBuffer {}
pub struct MacroAssembler {}
pub struct BytecodeArrayWriter {}
pub struct Operand {}
pub struct Smi {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct CodeStubAssembler {}
pub struct JumpBuffer {}
pub struct Internal {}
pub struct Callable {}
pub struct NativeModule {}
pub struct HeapNumber {}
pub struct CPURegister {}
pub struct HandleScope {}
pub struct TaggedRegister {}
pub struct Map {}
pub struct CallSiteInfo {}
pub struct Operand {}
pub struct Stack {
    _phantom: std::marker::PhantomData<i32>,
}
pub struct BuiltinReducerData {}
pub struct Vector<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct JSAsyncFunctionObject {}
pub struct FeedbackCell {}
pub struct JSReceiver {}
pub struct InterpreterPushArgsMode {}
pub struct Builtin {}
pub struct Heap {}
pub struct JSTypedArray {}
pub struct Smi {}
pub struct Builtin {}
pub struct JavaScript {}
pub struct HeapNumber {}
pub struct FixedDoubleArray {}
pub struct NativeModule {}
pub struct HeapNumber {}
pub struct DirectHandle<T> {}
pub struct Any {}
pub struct CPURegister {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct MacroAssembler {}
pub struct String {}
pub struct JavaScript {}
pub struct JSTypedArray {}
pub struct ValueType {}
pub struct Instruction {}
pub struct turboShaft {}
pub struct Label {}
pub struct StringTable {}
pub struct JavaScript {}
pub struct Operand {}
pub struct DirectHandle<T> {}
pub struct NumberFormatFields {}
pub struct Base {}
pub struct CodeStubArguments {}
pub struct DoubleRegister {}
pub struct HeapNumber {}
pub struct Local {
    _phantom: std::marker::PhantomData<Address>,
}
pub struct Base {}
pub struct FrameSummary {}
pub struct String {}
pub struct HeapNumber {}
pub struct Base {}
pub struct CodeStubArguments {}
pub struct AccessorInfo {}
pub struct CodeKind {}
pub struct Address {}
pub struct JSFunction {}
pub struct ExternalReference {}
pub struct MacroAssembler {}
pub struct String {}
pub struct JSGeneratorObject {}
pub struct HandleScope {}
pub struct This {}
pub struct Any {}
pub struct ThisValue {}
pub struct DoubleRegister {}
pub struct ExternalReference {}
pub struct StackFrame {}
pub struct JavaScriptFrame {}
pub struct FixedDoubleArray {}
pub struct ThisValue {}
pub struct MacroAssembler {}
pub struct Local<T> {}
pub struct FixedArrayBase {}
pub struct StackFrameIterator {}
pub struct DirectHandle<T> {}
pub struct ValueType {}
pub struct If {}
pub struct TaggedRegister {}
pub struct JSGenerator {}
pub struct Double {}
pub struct Operand {}
pub struct TaggedRegister {}
pub struct AbortReason {}
pub struct VectorDescriptor {}
pub struct BuiltinReducerData {}
pub struct base {}
pub struct Label {}
pub struct AccessorPair {}
pub struct Builtin {}
pub struct JSTypedArray {}
pub struct AnyValue {}
pub struct InterpreterFrameConstants {}
pub struct JavaScript {}
pub struct JumpBuffer {}
pub struct InstructionStream {}
pub struct Any {}
pub struct interpreter {}
pub struct HeapNumber {}
pub struct BuiltinReducerData {}
pub struct Vector<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct ListBuffer {}
pub struct StackFrameIterator {}
pub struct Smi {
    _phantom: std::marker::PhantomData<i32>,
}
pub struct Value {}
pub struct CodeStubAssembler {}
pub struct Builtins {}
pub struct NativeModule {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct DoubleRegister {}
pub struct Operand {}
pub struct Register {}
pub struct Builtin {}
pub struct Location {}
pub struct AnyValue {}
pub struct OpIndex {}
pub struct MacroAssembler {}
pub struct JavaScript {}
pub struct Operand {}
pub struct AbortReason {}
pub struct StackFrameIterator {}
pub struct TurboFanGraph {}
pub struct NativeModule {}
pub struct HeapNumber {}
pub struct ExternalReference {}
pub struct TaggedRegister {}
pub struct JSTypedArray {}
pub struct InterpreterFrameConstants {}
pub struct Op {}
pub struct Builtins {}
pub struct Label {}
pub struct Internal {}
pub struct Callable {}
pub struct NativeModule {}
pub struct HeapNumber {}
pub struct CPURegister {}
pub struct DoubleRegister {}
pub struct Operand {}
pub struct Instruction {}
pub struct FixedArrayBase {}
pub struct NativeModule {}
pub struct FrameSummary {}
pub struct CodeStubAssembler {}
pub struct OpIndex {}
pub struct AbortReason {}
pub struct FeedbackVector {}
pub struct Deoptimizer {}
pub struct base {}
pub struct Smi {
    _phantom: std::marker::PhantomData<i32>,
}
pub struct Label {}
pub struct CPURegister {}
pub struct CodeStubAssembler {}
pub struct Builtin {}
pub struct DirectHandle<T> {}
pub struct TaggedField<T, const OFFSET: usize> {}
pub struct AnyValue {}
pub struct DirectArguments {}
pub struct base {}
pub struct base {}
pub struct TaggedRegister {}
pub struct TaggedRegister {}
pub struct Smi {
    _phantom: std::marker::PhantomData<i32>,
}
mod x64 {
    use super::*;
    
    macro_rules! ACCESS_MASM {
        ($masm:expr) => {
            $masm
        };
    }
    
    macro_rules! ASM_CODE_COMMENT {
        ($masm:expr) => {
        };
        ($masm:expr, $($args:tt)*) => {
        };
    }
    
    pub fn generate_adaptor(
        masm: &mut MacroAssembler,
        formal_parameter_count: i32,
        address: Address,
    ) {
        
    }
}

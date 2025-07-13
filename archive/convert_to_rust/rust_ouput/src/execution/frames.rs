// Converted from V8 C++ source files:
// Header: frames.h
// Implementation: frames.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::thread::ThreadId;

use crate::execution::futex_emulation::AllStatic;
use crate::execution::isolate::Address;
use crate::execution::isolate::BytecodeArray;
use crate::execution::isolate::Code;
use crate::execution::isolate::Context;
use crate::execution::isolate::GCType;
use crate::execution::isolate::HandlerTable;
use crate::execution::isolate::HeapObject;
use crate::execution::isolate::Isolate;
use crate::execution::isolate::JSFunction;
use crate::execution::isolate::MaybeObject;
use crate::execution::isolate::NoExtension;
use crate::execution::isolate::PtrComprCageBase;
use crate::execution::isolate::RootIndex;
use crate::execution::isolate::Safepoint;
use crate::execution::isolate::Tagged_t;
use crate::execution::isolate::Wasm;
use crate::objects::objects::SharedFunctionInfo;
use crate::v8::Error;
use crate::v8::Function;
use crate::v8::Local;
use crate::v8::Value;
use crate::zone::zone_chunk_list::iterator;

pub struct V8 {}

pub enum Type {
    I32,
}

pub struct String_ExternalOneByteStringResource {}

pub struct ThreadState {}

pub struct HeapObject {}

pub struct RootVisitor {}

pub struct SourceLocation {}

pub struct ThreadManager {}

pub struct SafepointTable {}

pub struct InstructionStream {}

pub struct TraceDescriptor {}

pub struct NativeContext {}

pub struct Error {}

pub struct JSFunction {}

pub struct Context {}

pub struct HeapObject {}

pub struct NativeContext {}

pub struct FunctionTemplateInfo {}

pub struct PtrComprCageBase {}

pub struct StackFrame {}

pub struct CodeKind {}

pub enum CpuProfilingMode {}

pub enum LazyDeoptimizeReason {}

pub struct SourcePosition {}

pub struct Address {}

pub struct HeapObject {}

pub enum ThreadId {}

pub struct HeapObject {}

pub struct NativeContext {}

pub enum Builtin {}

pub struct HeapObject {}

pub struct JSFunction {}

pub struct HeapObject {}

pub struct NativeContext {}

pub struct Context {}

pub struct String {}

pub struct SharedFunctionInfo {}

pub struct HeapObject {}

pub struct Script {}

pub enum V8 {}

pub struct ThreadId {}

pub struct StackFrame {}

pub struct Code {}

pub struct HandlerTable {}

pub enum MaybeObject {}

pub struct ThreadId {}

pub struct HeapObject {}

pub struct String {}

pub struct SharedFunctionInfo {}

pub struct HeapObject {}

pub struct Script {}

pub enum V8 {}

pub struct ThreadId {}

pub struct Address {}

pub struct HeapObject {}

pub struct HandlerTable {}

pub struct StackHandlerConstants {
    pub const kNextOffset: i32 = 0;
    pub const kPaddingOffset: i32 = 1;

    pub const kSize: i32 = Self::kPaddingOffset + 1;
    pub const kSlotCount: i32 = Self::kSize >> 1;
}

pub struct StackHandler {
}

impl StackHandler {
  #[inline]
  pub fn address(&self) -> Address {
    Address {}
  }

  #[inline]
  pub fn next(&self) -> Option<&StackHandler> {
      None
  }

  #[inline]
  pub fn next_address(&self) -> Address {
    Address {}
  }

  pub fn FromAddress(address: Address) -> Option<&'static StackHandler> {
      None
  }
}

impl AllStatic for StackHandler {}

pub enum StackFrameType {
    NO_FRAME_TYPE = 0,
    ENTRY,
    CONSTRUCT_ENTRY,
    EXIT,
    WASM,
    WASM_TO_JS,
    WASM_TO_JS_FUNCTION,
    JS_TO_WASM,
    STACK_SWITCH,
    WASM_INTERPRETER_ENTRY,
    WASM_DEBUG_BREAK,
    C_WASM_ENTRY,
    WASM_EXIT,
    WASM_LIFTOFF_SETUP,
    WASM_SEGMENT_START,
    INTERPRETED,
    BASELINE,
    MAGLEV,
    TURBOFAN_JS,
    STUB,
    TURBOFAN_STUB_WITH_CONTEXT,
    BUILTIN_CONTINUATION,
    JAVASCRIPT_BUILTIN_CONTINUATION,
    JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH,
    INTERNAL,
    CONSTRUCT,
    FAST_CONSTRUCT,
    BUILTIN,
    BUILTIN_EXIT,
    API_CALLBACK_EXIT,
    API_ACCESSOR_EXIT,
    NATIVE,
    IRREGEXP,
    NUMBER_OF_TYPES,
    MANUAL
}

pub struct State {
    pub sp: Address,
    pub fp: Address,
    pub pc_address: *mut Address,
    pub callee_fp: Address,
    pub callee_pc: Address,
    pub constant_pool_address: *mut Address,
    pub is_profiler_entry_frame: bool,
    pub is_stack_exit_frame: bool,
}

impl State {
  fn default() -> Self {
      State {
          sp: Address {},
          fp: Address {},
          pc_address: std::ptr::null_mut(),
          callee_fp: Address {},
          callee_pc: Address {},
          constant_pool_address: std::ptr::null_mut(),
          is_profiler_entry_frame: false,
          is_stack_exit_frame: false,
      }
  }
}

pub struct StackFrameIteratorBase {}

impl StackFrameIteratorBase {
    pub fn isolate(&self) -> &Isolate {
        todo!()
    }

    pub fn done(&self) -> bool {
        todo!()
    }

    
}

pub trait StackFrameTrait {
    fn get_caller_stack_pointer(&self) -> Address;
    fn get_caller_state(&self, state: &mut State);
}
// Abstract base class for all stack frames.
pub struct StackFrame;

impl StackFrame{
    pub fn TypeToMarker(ty: StackFrameType) -> i32 {
        ty as i32
    }
    pub fn MarkerToType(marker: i64) -> StackFrameType {
        if marker == StackFrameType::ENTRY as i64 {
            return StackFrameType::ENTRY
        } else {
            return StackFrameType::EXIT
        }
    }
    pub fn IsTypeMarker(function_or_marker: u64) -> bool {
        if function_or_marker == StackFrameType::ENTRY as u64 {
            return true;
        } else {
            return false;
        }
    }

    fn GetCallerStackPointer(&self) -> Address {
        Address{}
    }
}

pub enum StackFrameType {
    NO_FRAME_TYPE = 0,
    ENTRY,
    CONSTRUCT_ENTRY,
    EXIT,
    WASM,
    WASM_TO_JS,
    JS_TO_WASM,
    INTERPRETED,
    BASELINE,
    MAGLEV,
    TURBOFAN_JS,
    STUB,
    INTERNAL,
    NUMBER_OF_TYPES,
    MANUAL
}
pub enum FrameSummaryEnum{
    JAVASCRIPT,
    BUILTIN,
    WASM,
    WASM_INTERPRETED,
    WASM_INLINED
}
pub struct FrameSummary {
    pub receiver: HeapObject{},
    pub is_constructor: bool,
}
impl FrameSummary{
    pub fn GetBottom() -> Self{
        FrameSummary{
            receiver: HeapObject {},
            is_constructor: true,
        }
    }
}
pub struct CommonFrame {}
pub struct TypedFrame {}
pub struct CommonFrameWithJSLinkage {}
pub struct JavaScriptFrame {}
pub struct UnoptimizedJSFrame {}
pub struct InterpretedFrame {}
pub struct BaselineFrame {}
pub struct OptimizedJSFrame {}
pub struct BuiltinContinuationFrame {}
pub struct TurbofanStubWithContextFrame {}

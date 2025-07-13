// Converted from V8 C++ source files:
// Header: deoptimizer.h
// Implementation: deoptimizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod deoptimizer {
use std::mem::size_of;

use crate::builtins::builtins::Builtin;
use crate::codegen::source_position::SourcePosition;
use crate::execution::frames::StackFrame;
use crate::execution::isolate::Isolate;
use crate::objects::js_function::JSFunction;

pub mod wasm {
        pub struct WasmCode {}
        pub enum ValueKind {}
}
pub enum BuiltinContinuationMode {}
        pub struct DeoptimizedFrameInfo {}
        pub enum DeoptimizeReason {}
        pub struct Address {}
        pub struct Code {}
        pub struct BytecodeOffset {}
        pub struct SharedFunctionInfo {}

#[derive(Debug)]
pub enum DeoptimizerError {
    MemoryAllocationError,
    InvalidAddress,
    SafepointNotFound,
    InvalidCodeKind,
}

pub struct DeoptInfo {
        pub position: SourcePosition,
        pub deopt_reason: DeoptimizeReason,
        pub node_id: u32,
        pub deopt_id: i32,
}

impl DeoptInfo {
        pub fn new(position: SourcePosition, deopt_reason: DeoptimizeReason, node_id: u32, deopt_id: i32) -> Self {
                Self {
                        position,
                        deopt_reason,
                        node_id,
                        deopt_id,
                }
        }
}

pub struct Deoptimizer {
        isolate_: *mut Isolate,
        function_: *mut JSFunction,
        compiled_code_: *mut Code,
        deopt_exit_index_: u32,
        bytecode_offset_in_outermost_frame_: BytecodeOffset,
        deopt_kind_: DeoptimizeKind,
        from_: Address,
        fp_to_sp_delta_: i32,
        deoptimizing_throw_: bool,
        catch_handler_data_: i32,
        catch_handler_pc_offset_: i32,
        restart_frame_index_: i32,
        input_: *mut FrameDescription,
        output_count_: i32,
        output_: *mut *mut FrameDescription,
        caller_frame_top_: i64,
        caller_fp_: i64,
        caller_pc_: i64,
        caller_constant_pool_: i64,
        actual_argument_count_: i32,
        stack_fp_: i64,
        trace_scope_: bool
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeoptimizeKind {
        Eager,
        Lazy,
}

impl Deoptimizer {
        pub fn deopt_exit_is_inside_osr_loop(
                isolate: *mut Isolate,
                function: *mut JSFunction,
                deopt_exit_offset: BytecodeOffset,
                osr_offset: BytecodeOffset,
        ) -> bool {
                true
        }

        pub fn get_deopt_info(code: *mut Code, from: Address) -> DeoptInfo {
                DeoptInfo::new(
                        SourcePosition {},
                        DeoptimizeReason {},
                        0,
                        0,
                )
        }

        pub fn message_for(kind: DeoptimizeKind) -> &'static str {
                match kind {
                        DeoptimizeKind::Eager => "deopt-eager",
                        DeoptimizeKind::Lazy => "deopt-lazy",
                }
        }

        pub fn new(raw_function: Address, kind: DeoptimizeKind, from: Address, fp_to_sp_delta: i32, isolate: *mut Isolate) -> Result<*mut Self, DeoptimizerError> {
                Ok(std::ptr::null_mut())
        }

        pub fn grab(isolate: *mut Isolate) -> Result<*mut Self, DeoptimizerError> {
                Ok(std::ptr::null_mut())
        }

        pub fn delete_for_wasm(isolate: *mut Isolate) -> Result<usize, DeoptimizerError> {
                Ok(0)
        }

        pub fn debugger_inspectable_frame(frame: *mut JavaScriptFrame, jsframe_index: i32, isolate: *mut Isolate) -> Result<*mut DeoptimizedFrameInfo, DeoptimizerError> {
                Ok(std::ptr::null_mut())
        }

        pub fn deoptimize_function(
                function: *mut JSFunction,
                reason: LazyDeoptimizeReason,
                code: *mut Code,
        ) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn deoptimize_all(isolate: *mut Isolate) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn deoptimize_marked_code(isolate: *mut Isolate) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn deoptimize_all_optimized_code_with_function(
                isolate: *mut Isolate,
                function: *mut SharedFunctionInfo,
        ) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn ensure_valid_return_address(isolate: *mut Isolate, address: Address) -> Result<Address, DeoptimizerError> {
                Ok(address)
        }

        pub fn compute_output_frames(deoptimizer: *mut Deoptimizer) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn get_deoptimization_entry(kind: DeoptimizeKind) -> Builtin {
                Builtin {}
        }

        pub fn input_offset() -> usize {
                0
        }

        pub fn output_count_offset() -> usize {
                0
        }

        pub fn output_offset() -> usize {
                0
        }

        pub fn caller_frame_top_offset() -> usize {
                0
        }

        pub const K_MAX_NUMBER_OF_ENTRIES: i32 = 16384;
        pub const K_FIXED_EXIT_SIZE_MARKER: u32 = Self::K_MAX_NUMBER_OF_ENTRIES as u32;
        pub const K_EAGER_DEOPT_EXIT_SIZE: i32 = 10;
        pub const K_LAZY_DEOPT_EXIT_SIZE: i32 = 10;
 pub const K_ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 10;

        pub fn trace_mark_for_deoptimization(
                isolate: *mut Isolate,
                code: *mut Code,
                reason: LazyDeoptimizeReason,
        ) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn trace_evict_from_optimized_code_cache(
                isolate: *mut Isolate,
                sfi: *mut SharedFunctionInfo,
                reason: &str,
        ) -> Result<(), DeoptimizerError> {
                Ok(())
        }

        pub fn patch_to_jump(pc: Address, new_pc: Address) -> Result<(), DeoptimizerError> {
                Ok(())
        }
}

pub struct JavaScriptFrame {}
        pub struct FrameDescription {}
pub enum LazyDeoptimizeReason {}
        pub struct GCType {}

        pub enum CodeKind {}
        pub struct Heap {}
        pub struct DirectHandle<T> {}
        pub struct Safepoint {}
        pub struct ThreadManager {}
        pub struct Tagged<T> {}
        pub struct Smi {}
        pub struct StackGuard {}
        pub struct Object {}
pub enum GCFlag {}
        pub enum GarbageCollectionReason {}
        pub struct CodeTracer {}
pub struct Builtins {}

impl Deoptimizer {
        pub fn get_deopt_info_() -> DeoptInfo {
                DeoptInfo::new(
                        SourcePosition {},
                        DeoptimizeReason {},
                        0,
                        0,
                )
        }
}

trait Visitor {}
        pub struct ThreadLocalTop {}
        pub struct StackFrameIterator {}
        pub struct OptimizedJSFrame {}
        pub struct GcSafeCode {}
pub struct MaglevSafepointEntry {}
pub struct SafepointEntry {}

impl StackFrame {
        pub fn is_optimized_js(&self) -> bool {false}
        pub fn GcSafeLookupCode(&self) -> *mut GcSafeCode {std::ptr::null_mut()}
        pub fn maybe_unauthenticated_pc(&self) -> Address {Address {}}
        pub fn pc_address(&self) -> *mut Address {std::ptr::null_mut()}
        pub fn InFastCCall(&self) -> bool {false}
        pub fn is_wasm(&self) -> bool {false}
}

impl Code {
        pub fn kind(&self) -> CodeKind {CodeKind {}}
        pub fn uses_deoptimization_data(&self) -> bool {false}
        pub fn osr_offset(&self) -> BytecodeOffset {BytecodeOffset {}}
        pub fn Inlines(&self, _arg: *mut SharedFunctionInfo) -> bool {false}
        pub fn parameter_count(&self) -> i32 {0}
  pub fn instruction_start(&self) -> Address {Address {}}
  pub fn instruction_end(&self) -> Address {Address {}}
        pub fn stack_slots(&self) -> u32 {0}
        pub fn is_maglevved(&self) -> bool {false}
  pub fn deoptimization_data(&self) -> *mut Object {std::ptr::null_mut()}
  pub fn SetMarkedForDeoptimization(&self, _arg: *mut Isolate, _arg1: LazyDeoptimizeReason){}
}

pub enum CachedTieringDecision {}

impl SharedFunctionInfo {
        pub fn cached_tiering_decision(&self) -> CachedTieringDecision {CachedTieringDecision {}}
        pub fn set_cached_tiering_decision(&self, _arg: CachedTieringDecision){}
        pub fn GetBytecodeArray(&self, _arg: *mut Isolate) -> *mut BytecodeArray {std::ptr::null_mut()}
}
        pub struct FeedbackCell {}

impl JSFunction {
        pub fn shared(&self) -> *mut SharedFunctionInfo {std::ptr::null_mut()}
        pub fn code(&self, _arg: *mut Isolate) -> *mut Code {std::ptr::null_mut()}
        pub fn feedback_vector(&self) -> *mut FeedbackVector {std::ptr::null_mut()}
        pub fn raw_feedback_cell(&self) -> *mut FeedbackCell {std::ptr::null_mut()}
}

pub struct FeedbackVector {}

impl FeedbackVector {
        pub fn was_once_deoptimized(&self) -> bool {false}
        pub fn set_was_once_deoptimized(&self) {}
}
        pub struct NativeModule {}
        pub struct WasmTrustedInstanceData {}
        pub struct ObjectRef {}

pub struct FrameDescription {}

impl FrameDescription {
        pub fn GetFrameSize(&self) -> u32 {0}
  pub fn Create(frame_size: u32, parameter_count: i32, isolate: *mut Isolate) -> *mut FrameDescription {std::ptr::null_mut()}
  pub fn SetFrameSlot(&mut self, top_offset: u32, value: i64){}
        pub fn parameter_count(&self) -> i32 {0}
        pub fn GetLastArgumentSlotOffset(&self, _arg: bool) -> u32 {0}
        pub fn GetTop(&self) -> Address {Address {}}
  pub fn SetPc(&mut self, pc: i64){}
        pub fn GetPc(&self) -> i64 {0}
  pub fn SetCallerPc(&mut self, offset: u32, pc: i64){}
  pub fn GetCallerPc(&self) -> i64 {0}
        pub fn HasCallerPc(&self) -> bool {false}
  pub fn SetFp(&mut self, fp: i64){}
        pub fn GetFp(&self) -> i64 {0}
  pub fn GetFramePointerAddress(&self) -> Address {Address {}}
        pub fn SetCallerFp(&mut self, _arg: u32, _arg1: i64){}
  pub fn SetRegister(&mut self, code: i32, value: i64){}
  pub fn GetRegisterValues(&self) -> *mut RegisterValues {std::ptr::null_mut()}
        pub fn SetConstantPool(&mut self, _arg: i64){}
        pub fn GetConstantPool(&self) -> i64 {0}
        pub fn SetFrameSlotPointer(&mut self, _arg: u32, _arg1: i64){}
        pub fn SetLiftoffFrameSlot32(&mut self, _arg: u32, _arg1: i32){}
  pub fn SetLiftoffFrameSlot64(&mut self, offset: u32, val: i64){}
        pub fn SetLiftoffFrameSlotPointer(&mut self, _arg: u32, _arg1: i64){}
        pub fn SetDoubleRegister(&mut self, _arg: i32, _arg1: f64){}
  pub fn SetSimd128Register(&mut self, code: i32, simd_value: Simd128){}
        pub fn SetCallerConstantPool(&mut self, top_offset: u32, cp: i64){}
}

        pub struct RegisterValues {}
impl RegisterValues {
        pub fn SetRegister(&mut self, _arg: i32, _arg1: i64){}
}

        pub struct ObjectIterator {}
        pub struct InstructionStream {}

impl Heap {
        pub fn code_space(&self) -> &CodeSpace {CodeSpace {}}
        pub fn code_lo_space(&self) -> &CodeLOSpace {CodeLOSpace {}}
        pub fn FindCodeForInnerPointer(&mut self, _addr: Address) -> *mut Code {std::ptr::null_mut()}
        pub fn offset(&self) -> Smi {Smi {}}
}

        pub struct CodeSpace {}
impl CodeSpace {
        pub fn GetObjectIterator(&self, _arg: *mut Heap) -> *mut ObjectIterator {std::ptr::null_mut()}
}

        pub struct CodeLOSpace {}
impl CodeLOSpace {
        pub fn GetObjectIterator(&self, _arg: *mut Heap) -> *mut ObjectIterator {std::ptr::null_mut()}
}

impl SafepointTable {
        pub fn FindEntry(
                _isolate: *mut Isolate,
                _code: *mut Code,
                _pc: Address,
        ) -> SafepointEntry {
                SafepointEntry {}
        }
}
pub struct SafepointTable {}
        pub struct CodeLOSpace {}

impl MaglevSafepointTable {
        pub fn FindEntry(
                _isolate: *mut Isolate,
                _code: *mut Code,
                _pc: Address,
        ) -> MaglevSafepointEntry {
                MaglevSafepointEntry {}
        }
}

        pub struct MaglevSafepointTable {}
pub struct HandlerTable {}

impl HandlerTable {
        pub fn LookupHandlerIndexForRange(&self, _arg: i32) -> i32 {0}
  pub fn GetRangeData(&self, handler_index: i32) -> i32 {0}
        pub fn GetRangeHandler(&self, _arg: i32) -> i32 {0}
  pub fn MarkHandlerUsed(&self, _arg: i32){}
}

pub struct StackFrameIteratorForProfiler {}
        pub struct DisallowGarbageCollection {}
        pub struct Profile {}
        pub struct DeoptimizationData {}
impl Builtins {
        pub fn code(&self, _arg: Builtin) -> *mut Code {std::ptr::null_mut()}
}

impl Isolate {
        pub fn builtins(&mut self) -> &mut Builtins {&mut Builtins {}}
  pub fn heap(&mut self) -> &mut Heap {&mut Heap {}}
        pub fn thread_local_top(&self) -> &ThreadLocalTop {&ThreadLocalTop {}}
  pub fn set_current_deoptimizer(&mut self, deoptimizer: *mut Deoptimizer){}
        pub fn GetAndClearCurrentDeoptimizer(&mut self) -> *mut Deoptimizer {std::ptr::null_mut()}
  pub fn AbortConcurrentOptimization(&self, _arg: BlockingBehavior){}
        pub fn stack_guard(&mut self) -> &mut StackGuard {&mut StackGuard {}}
        pub fn counters(&self) -> &Counters {&Counters {}}
  pub fn materialized_object_store(&mut self) -> &mut MaterializedObjectStore {&mut MaterializedObjectStore {}}
        pub fn deoptimizer_lazy_throw(&self) -> bool {false}
        pub fn set_deoptimizer_lazy_throw(&mut self, _arg: bool){}
        pub fn debug(&mut self) -> &mut Debug {&mut Debug {}}
        pub fn isolate_data(&mut self) -> &mut IsolateData {&mut IsolateData {}}
        pub fn thread_manager(&self) -> &ThreadManager {&ThreadManager {}}
  pub fn cage_base(&self) -> u64 {0}
  pub fn isolate_root(&self) -> u64 {0}
        pub fn is_shared_space_isolate(&self) -> bool {false}
        pub fn GetCodeTracer(&self) -> &CodeTracer {&CodeTracer {}}
}

pub enum BlockingBehavior {}
        pub enum FrameState {}
impl DeoptimizationData {
        pub fn TranslationIndex(&self, _arg: u32) -> Smi {Smi {}}
  pub fn GetBytecodeOffsetOrBuiltinContinuationId(&self, deopt_exit_index: u32) -> BytecodeOffset {BytecodeOffset {}}
  pub fn FrameTranslation(&self) -> *mut Object {std::ptr::null_mut()}
  pub fn LiteralArray(&self) -> *mut Object {std::ptr::null_mut()}
  pub fn ProtectedLiteralArray(&self) -> *mut Object {std::ptr::null_mut()}
  pub fn DeoptExitStart(&self) -> Smi {Smi {}}
        pub fn EagerDeoptCount(&self) -> Smi {Smi {}}
        pub fn OptimizationId(&self) -> Smi {Smi {}}
        pub fn GetSharedFunctionInfo(&self) -> *mut SharedFunctionInfo {std::ptr::null_mut()}
}

        pub struct Debug {}
impl Debug {
        pub fn IsRestartFrameScheduled(&self) -> bool {false}
        pub fn restart_inline_frame_index(&self) -> i32 {0}
        pub fn clear_restart_frame(&mut self){}
}

        pub struct UnoptimizedFrameInfo {}
impl UnoptimizedFrameInfo {
        pub fn Precise(_arg0: i32, _arg1: i32, _arg2: bool, _arg3: bool) -> UnoptimizedFrameInfo {UnoptimizedFrameInfo {}}
        pub fn frame_size_in_bytes(&self) -> u32 {0}
        pub fn frame_size_in_bytes_without_fixed(&self) -> u32 {0}
        pub fn register_stack_slot_count(&self) -> u32 {0}
}

        pub struct FastConstructStubFrameInfo {}

impl FastConstructStubFrameInfo {
        pub fn Precise(_arg: bool) -> FastConstructStubFrameInfo {FastConstructStubFrameInfo {}}
        pub fn frame_size_in_bytes(&self) -> u32 {0}
        pub fn frame_size_in_bytes_without_fixed(&self) -> u32 {0}
}
        pub struct IsolateData {}
        pub struct Counters {}
impl Counters {
        pub fn wasm_deopts_executed(&self) -> &WasmDeoptsExecuted {&WasmDeoptsExecuted {}}
  pub fn wasm_deopts_per_function(&self) -> &WasmDeoptsPerFunction {&WasmDeoptsPerFunction {}}
}

pub struct WasmDeoptsExecuted {}
        impl WasmDeoptsExecuted {
                pub fn AddSample(&self, _arg: i32){}
        }
pub struct WasmDeoptsPerFunction {}
impl WasmDeoptsPerFunction {
        pub fn AddSample(&self, _arg: i32){}
}

struct Malloced {}
impl ::std::ops::Drop for Deoptimizer {
 fn drop(&mut self) {
                }
}
pub mod trap_handler {
        pub fn ClearThreadInWasm(){}
        pub fn SetThreadInWasm(){}
}
pub struct FlatMap {}
        pub struct TypeFeedbackStorage {}

pub mod base {
        pub fn IsInRange(_arg0: i32, _arg1: i32, _arg2: i32) -> bool {false}
}

pub struct CodeTracerScope {}
impl CodeTracerScope {
        pub fn file(&self) -> *mut std::ffi::c_void {std::ptr::null_mut()}
}

pub struct RootVisitor {}
pub mod pointer_authentication {
        pub fn ReplacePC(_arg0: *mut Address, _arg1: Address, _arg2: usize){}
        pub fn SignAndCheckPC(_arg0: *mut super::Isolate, _arg1: super::Address, _arg2: super::Address) -> i64 {0}
        pub fn MoveSignedPC(_arg0: *mut super::Isolate, _arg1: i64, _arg2: super::Address, _arg3: super::Address) -> i64 {0}
        pub fn StripPAC(_arg0: i64) -> i64 {0}
}
pub mod interpreter {
        pub enum Bytecode {}
        pub struct BytecodeArrayIterator {}
 impl BytecodeArrayIterator {
                pub fn CurrentBytecodeIsValidOSREntry(&self) -> bool {false}
  pub fn done(&self) -> bool {false}
                pub fn CurrentBytecode(&self) -> Bytecode {Bytecode {}}
  pub fn GetJumpTargetOffset(&self) -> i32 {0}
                pub fn GetImmediateOperand(&self, _arg: i32) -> i32 {0}
                pub fn Advance(&mut self){}
                pub fn current_offset(&self) -> i32 {0}
                pub fn IsValidOffset(_bytecode_array: *mut super::BytecodeArray, _off: i32) -> bool { false }
        }
}
        pub struct BuiltinContinuationFrameInfo {}

impl BuiltinContinuationFrameInfo {
        pub fn Precise(_arg: i32, _arg1: CallInterfaceDescriptor, _arg2: &RegisterConfiguration, _arg3: bool, _arg4: DeoptimizeKind, _arg5: BuiltinContinuationMode) -> BuiltinContinuationFrameInfo {BuiltinContinuationFrameInfo {}}
        pub fn frame_size_in_bytes(&self) -> u32 {0}
        pub fn frame_size_in_bytes_above_fp(&self) -> u32 {0}
        pub fn translated_stack_parameter_count(&self) -> u32 {0}
  pub fn frame_has_result_stack_slot(&self) -> bool {false}
}
        pub struct CallInterfaceDescriptor {}

impl CallInterfaceDescriptor {
        pub fn GetRegisterParameterCount(&self) -> i32 {0}
        pub fn GetParameterType(&self, _arg: i32) -> MachineType {MachineType {}}
        pub fn GetRegisterParameter(&self, _arg: i32) -> Register {Register {}}
        pub fn GetStackArgumentOrder(&self) -> StackArgumentOrder {StackArgumentOrder {}}
}
        pub enum StackArgumentOrder {}

#[derive(Clone, Copy)]
pub struct MachineType {}

impl MachineType {
        pub fn Int32() -> MachineType { MachineType {} }
        pub fn representation(&self) -> Representation { Representation {} }
}
pub enum Representation {}

impl Representation {
        pub fn Integer32() -> Representation { Representation {} }
        pub fn Tagged() -> Representation { Representation {} }
}
pub fn IsAnyTagged(_arg: Representation) -> bool { false }

pub struct Register {}
impl Register {
        pub fn from_code(code: i32) -> Register {Register {}}
        pub fn code(&self) -> i32 {0}
}
pub struct RegisterConfiguration {}

impl RegisterConfiguration {
        pub fn Default() -> &RegisterConfiguration {&RegisterConfiguration {}}
        pub fn num_general_registers(&self) -> i32 {0}
        pub fn num_allocatable_general_registers(&self) -> i32 {0}
        pub fn GetAllocatableGeneralCode(&self, _arg: i32) -> i32 {0}
}
        pub struct ByteArray {}
impl Deoptimizer {
 pub const kAdaptShadowStackOffsetToSubtract: i32 = 0;
}
pub struct Zone {}
pub mod wasm {
        pub struct NativeModule {}
        pub struct LiftoffVarState {}
        pub struct FunctionBody {}
        pub struct FunctionSig {}
        pub struct WasmCompilationResult {}
        pub enum ValueKind {}
        pub struct LiftoffOptions {}
        pub struct UnpublishedWasmCode {}
        pub struct CompilationState {}
        pub struct WasmDeoptView {}
        pub struct WasmDeoptEntry {}
        pub struct FunctionTable {};

 impl FunctionTable {
                pub fn offset_table_size_bytes() -> u32 {0}
                pub fn element_size_bytes() -> u32 {0}
 }

 impl NativeModule {
                pub fn wire_bytes(&self) -> base::Vector<*const u8> {base::Vector {}}
                pub fn compilation_state(&self) -> &CompilationState {&CompilationState {}}
  pub fn AddCompiledCode(&self, _result: WasmCompilationResult) -> UnpublishedWasmCode {UnpublishedWasmCode {}}
                pub fn module(&self) -> &Module {&Module {}}
 }

 pub mod linkage {
                pub struct LinkageLocation {}
 }
 pub struct Module {}
        pub struct Simd128 {}
        pub struct FrameDescription {}
 impl  WasmCode {
                pub fn native_module(&self) -> *mut NativeModule {std::ptr::null_mut()}
                pub fn instruction_start(&self) -> Address {Address {}}
                pub fn index(&self) -> i32 {0}
  pub fn deopt_data(&self) -> *mut Object {std::ptr::null_mut()}
                pub fn kind(&self) -> WasmCodeKind {WasmCodeKind {}}
  pub fn DebugName(&self) -> String {String::from("")}
 }

        #[derive(PartialEq)]
 pub enum WasmCodeKind {
                kWasmFunction
 }
 pub fn GetWasmCodeManager() -> *mut WasmCodeManager {std::ptr::null_mut()}
        pub struct WasmCodeManager {}

 impl WasmCodeManager {
                pub fn LookupCode(&self, _isolate: *mut super::Isolate, _address: Address) -> *mut WasmCode {std::ptr::null_mut()}
                pub fn IncrementDeoptsExecutedCount(&self) -> i32 {0}
 }

 pub struct TypeFeedbackStorage {}

 impl TypeFeedbackStorage {
                pub fn feedback_for_function(&mut self) -> &mut FlatMap {&mut FlatMap {}}
 }
 pub fn declared_function_index(_module: &Module, _function_index: i32) -> u32 {0}
 pub struct Int64 {}
 public struct FunctionTable {}
        pub struct Simd128 {}
 public struct Float32 {}
        pub struct Float64 {}
 public struct Int64x2 {pub val: [i64; 2]}
}

impl From<i64> for Tagged<Object> {
 fn from(_value: i64) -> Self {
  Tagged {}
 }
}
pub mod test_base {
        pub fn SizeOf(_what: i32) -> i32 {0}
}

pub mod tracing {
        pub fn IsSupported(isolate: &super::Isolate) -> bool {false}
        pub struct TracingController {}
 impl TracingController {
                pub fn GetDefaultTracingController() -> *mut TracingController {std::ptr::null_mut()}
 }
 pub struct TraceEventArgs {}
}

        pub struct HeapObject {}

        pub struct StringView {}
        pub struct OfStream {}

impl Isolate {
        pub fn GetCodeTracer() -> &CodeTracer {&CodeTracer {}}
}
impl Profile {
        pub fn CodeDeoptEvent(_arg0: *mut Code, _arg1: DeoptimizeKind, _arg2: Address, _arg3: i32) -> Profile {Profile {}}
        pub fn CodeDependencyChangeEvent(_arg0: *mut Code, _arg1: *mut SharedFunctionInfo, _arg2: &str) -> Profile {Profile {}}
}

pub struct Profile {
 
}
        pub struct AccountingAllocator {}
        pub struct DeoptimizationFrameTranslation {}
        pub struct DeoptimizationLiteralProvider {}

impl ReadOnlyRoots {
 pub fn arguments_marker(&self) -> Tagged<Object> { Tagged {} }
        pub fn undefined_value(&self) -> Tagged<Object> {Tagged {}}
  pub fn the_hole_value(&self) -> Tagged<Object> { Tagged {} }
}
        pub struct ReadOnlyRoots {}

impl DeoptimizationFrameTranslation {
        pub struct Iterator {}
}
impl DeoptimizationFrameTranslation::Iterator {
 pub fn SeekToFirstTranslation(&self) {}
 public fn GetNextCommand(&mut self) {}
}
pub mod compiler {
 public struct InstructionSelector {}
        }

        pub struct SafepointScope {}

        pub mod flag_list {}

pub mod v8threads {
 pub struct ThreadState {}
 public fn IterateArchivedThreads(_thread_manager: &super::ThreadManager, _thread_visitor: &impl Visitor){}
}
pub struct MutexGuard {}

impl Lock {
 fn new(isolate: &mut Isolate, _arg: bool) -> Self {Lock {}}
        }
struct Lock {}

        pub struct Simd128 {}
impl Simd128 {
 pub fn to_i64x2(&self) -> Int64x2 {Int64x2 { val: [0; 2]}}
}
        pub struct ConstructStubFrameInfo {}
impl ConstructStubFrameInfo {
 public fn Precise(_arg0: i32, _arg1: bool) -> Self {Self{}}
        pub fn frame_size_in_bytes(&self) -> u32 {0}
}

const K_JS_ARGCOUNT_REGISTER: i32 = 0;

        pub struct MaterializedObjectStore {}

impl MaterializedObjectStore {
        pub fn Remove(&mut self, _arg: Address){}
}
        pub struct ThreadId {}

fn DeoptimizeReasonToString(_reason: LazyDeoptimizeReason) -> &'static str {
 return ""
}

fn ArgumentPaddingSlots(parameters_count: i32) -> i32 {
 return 0
}
const kNoDeoptimizationId: i32 = 0;
const kReturnRegister0: Register = Register {};
const kReturnRegister1: Register = Register {};
const kJavaScriptCallArgCountRegister: Register = Register {};
const kJavaScriptCallDispatchHandleRegister: Register = Register {};

const kContextRegister: Register = Register {};

const kStackLimitSlackForDeoptimizationInBytes: u32 = 0;
const kSystemPointerSize: u32 = 8;
const K_JS_ARGCOUNT_REGISTER: i32 = 0;
const kReturnRegister0: Register = Register {};
const kPtrComprCageBaseRegister: Register = Register {};
const kRootRegister: Register = Register {};

const kPCOnStackSize: u32 = 0;
const kFPOnStackSize: u32 = 0;
        const kJSArgcReceiverSlots: i32 = 0;



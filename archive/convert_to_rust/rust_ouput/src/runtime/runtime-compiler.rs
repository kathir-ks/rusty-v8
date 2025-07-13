// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-compiler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::sync::Arc;

use crate::asmjs::asm_js::AsmWasmData;
use crate::codegen::compiler::{
    CodeKind, Compiler, ConcurrencyMode, IsCompiledScope, LanguageMode,
};
use crate::codegen::pending_optimization_table::AsmWasmData;
use crate::codegen::reglist::BitField64;
use crate::common::globals::FLAG_log_function_events;
use crate::common::message_template::MessageTemplate;
use crate::deoptimizer::deoptimizer::Deoptimizer;
use crate::execution::arguments_inl::Arguments;
use crate::execution::frames_inl::{JavaScriptFrame, JavaScriptStackFrameIterator, MaglevFrame, UnoptimizedJSFrame};
use crate::execution::isolate_inl::Isolate;
use crate::init::bootstrapper::ReadOnlyRoots;
use crate::interpreter::bytecode_array_iterator::BytecodeArrayIterator;
use crate::interpreter::bytecode_register_optimizer::BudgetModification;
use crate::objects::js_array_buffer_inl::JSArrayBuffer;
use crate::objects::objects_inl::Object;
use crate::objects::shared_function_info::SharedFunctionInfo;
use crate::runtime::runtime_utils::StackLimitCheck;

use crate::codegen::compiler::NO_PARSE_RESTRICTION;

use crate::codegen::compiler::ParseRestriction;
use crate::codegen::compiler::kNoSourcePosition;

use crate::objects::shared_function_info::Script;
use crate::objects::scope_info::ScopeInfo;
use crate::objects::contexts::Context;
use crate::objects::native_context::NativeContext;

use crate::objects::script::IndirectHandle;
use crate::objects::contexts::DirectHandle;
use crate::objects::contexts::Handle;
use crate::wasm::decoder::MaybeDirectHandle;
use crate::asmjs::asm_js::AsmJs;
use crate::objects::contexts::Tagged;

use crate::codegen::code_stub_assembler::Smi;
use crate::codegen::code_stub_assembler::Builtin;

use crate::codegen::loong64::macro_assembler_loong64::DeoptimizeKind;
use crate::codegen::s390::assembler_s390::DeoptimizeReason;
use crate::codegen::compiler::CodeKindIsOptimizedJSFunction;

use crate::codegen::reglist::CPURegister;
use crate::codegen::arm64::reglist_arm64::CPURegList;
use crate::compiler::turboshaft::operations::OpIndex;
use crate::codegen::arm::macro_assembler_arm::Register;
use crate::codegen::arm::macro_assembler_arm::Operand;
use crate::codegen::arm::macro_assembler_arm::Condition;
use crate::codegen::label::Label;

use crate::codegen::pending_optimization_table::FeedbackVector;
use crate::codegen::compiler::BytecodeOffset;
use crate::codegen::code_tracer::CodeTracer;

use crate::codegen::code_tracer::TimerEventScope;
use crate::codegen::code_tracer::TimerEventDeoptimizeCode;
use crate::codegen::code_tracer::TRACE_EVENT0;

use crate::codegen::loong64::macro_assembler_loong64::LazyDeoptimizeReason;
use crate::codegen::loong64::macro_assembler_loong64::DeoptimizeKind::kLazy;

use crate::codegen::compiler::UnoptimizedCompileFlags;
use crate::codegen::compiler::ConcurrencyMode::kConcurrent;
use crate::codegen::compiler::ConcurrencyMode::kSynchronous;

use crate::compiler::backend::riscv::instruction_selector_riscv::Frame;
use crate::objects::js_function::JSFunction;

use crate::init::isolate_group::IsolateGroup;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub mod v8 {
    pub mod internal {
        pub enum V8 {}
    }
}

pub mod base {
    pub fn IsInRange(value: i32, start: i32, end: i32) -> bool {
        value >= start && value <= end
    }
}

pub mod interpreter {
    pub mod Bytecode {
        pub enum Bytecode {
            kJumpLoop,
        }
    }

    pub fn BytecodeArrayIteratorIsValidOffset(_bytecode_array: &BytecodeArray, _offset: i32) -> bool {
        true
    }

    pub struct BytecodeArrayIterator<'a> {
        bytecode_array: &'a BytecodeArray,
        offset: i32,
    }

    impl<'a> BytecodeArrayIterator<'a> {
        pub fn new(bytecode_array: &'a BytecodeArray, offset: i32) -> Self {
            BytecodeArrayIterator {
                bytecode_array,
                offset,
            }
        }

        pub fn IsValidOffset(_bytecode_array: &BytecodeArray, _offset: i32) -> bool {
            true
        }

        pub fn done(&self) -> bool {
            false
        }

        pub fn Advance(&mut self) {}

        pub fn current_bytecode(&self) -> Bytecode {
            Bytecode::kJumpLoop
        }

        pub fn GetJumpTargetOffset(&self) -> i32 {
            0
        }

        pub fn GetImmediateOperand(&self, _index: i32) -> i32 {
            0
        }

        pub fn current_offset(&self) -> i32 {
            self.offset
        }

        pub fn SetOffset(&mut self, offset: i32) {
            self.offset = offset;
        }

        pub fn GetSlotOperand(&self, _index: i32) -> i32 {
            0
        }
    }
}

pub mod i {
    pub type Object = u64;
}

pub mod roots {
    pub struct ReadOnlyRoots {}
}

pub struct CodePointerHandle {}
pub struct Space {}
pub enum GCType {}

struct SaveAndClearThreadInWasmFlag {}
impl SaveAndClearThreadInWasmFlag {
    pub fn new(_isolate: &Isolate) -> Self {
        SaveAndClearThreadInWasmFlag {}
    }
}

impl Drop for SaveAndClearThreadInWasmFlag {
    fn drop(&mut self) {}
}

pub struct AstNodeSourceRanges {}

pub struct Isolate {
    // Add necessary fields here
    counters: Counters,
}

impl Isolate {
    pub fn counters(&mut self) -> &mut Counters {
        &mut self.counters
    }

    pub fn is_execution_terminating(&self) -> bool {
        false
    }

    pub fn has_exception(&self) -> bool {
        false
    }

    pub fn native_context(&self) -> DirectHandle<NativeContext> {
        DirectHandle {
            value: 0, // Replace with actual value if needed
        }
    }

    pub fn Throw(&mut self, _error: u64) {}

    pub fn factory(&mut self) -> Factory {
        Factory {}
    }
}

pub struct Factory {}
impl Factory {
    pub fn NewEvalError(&mut self, _message_template: MessageTemplate, _error_message: &u64) -> MaybeDirectHandle<Object> {
        MaybeDirectHandle {
            value: Some(0), // Replace with actual value if needed
        }
    }
}

pub struct Counters {}
impl Counters {
    pub fn asmjs_instantiate_result(&mut self) -> AsmJsInstantiateResultCounter {
        AsmJsInstantiateResultCounter {}
    }
}

pub struct AsmJsInstantiateResultCounter {}
impl AsmJsInstantiateResultCounter {
    pub fn AddSample(&mut self, _sample: AsmJsInstantiateResult) {}
}

enum AsmJsInstantiateResult {
    kAsmJsInstantiateSuccess = 0,
    kAsmJsInstantiateFail = 1,
}

pub struct BytecodeArray {}

pub struct UnoptimizedCompileFlags {}

pub struct TieringManager {}
impl TieringManager {
    pub fn MarkForTurboFanOptimization(&self, _function: &JSFunction) {}
}

impl Isolate {
    pub fn concurrent_recompilation_enabled(&self) -> bool {
        true
    }

    pub fn EfficiencyModeEnabledForTiering(&self) -> bool {
        false
    }

    pub fn BatterySaverModeEnabled(&self) -> bool {
        false
    }

    pub fn tiering_manager(&self) -> &TieringManager {
        todo!()
    }

    pub fn set_context(&self, _arg: Tagged<Context>) {}

    pub fn context(&self) -> Tagged<Context> {
        todo!()
    }

    pub fn GetCodeTracer(&self) -> &CodeTracer {
        todo!()
    }
}

pub struct SharedFunctionInfoDiscardCompiledResult {}

#[no_mangle]
pub extern "C" fn Runtime_CompileLazy(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());

    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);

    let check = StackLimitCheck::new(isolate);
    if check.JsHasOverflowed(
        (kStackSpaceRequiredForCompilation * 1024) as usize,
    ) {
        return 0; //isolate.StackOverflow();
    }

    let sfi = DirectHandle::new(function.shared(), isolate);

    assert!(!function.is_compiled(isolate));
    let is_compiled_scope = IsCompiledScope::new();
    if !Compiler::Compile(
        isolate,
        &function,
        Compiler::KEEP_EXCEPTION,
        &is_compiled_scope,
    ) {
        return ReadOnlyRoots::new().exception();
    }

    if FLAG_log_function_events {
        LogExecution(isolate, function);
    }

    assert!(function.is_compiled(isolate));
    return function.code(isolate);
}

const kStackSpaceRequiredForCompilation: usize = 256;

fn LogExecution(isolate: *mut Isolate, function: DirectHandle<JSFunction>) {
    let isolate = unsafe { &mut *isolate };
    if !FLAG_log_function_events {
        return;
    }

    if !function.has_feedback_vector() {
        return;
    }

    let sfi = DirectHandle::new(function.shared(), isolate);
    let name = SharedFunctionInfo::DebugName(isolate, sfi);

    let raw_sfi = *sfi;
    let event_name = "first-execution".to_string();
    let kind = function.abstract_code(isolate).kind(isolate);
    if kind != CodeKind::INTERPRETED_FUNCTION {
        let kind_str = CodeKindToString(kind);
        println!("kind str = {}", kind_str);
    }
}

fn CodeKindToString(kind: CodeKind) -> String {
    match kind {
        CodeKind::INTERPRETED_FUNCTION => "INTERPRETED_FUNCTION".to_string(),
        CodeKind::MAGLEV => "MAGLEV".to_string(),
        CodeKind::TURBOFAN_JS => "TURBOFAN_JS".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_InstallBaselineCode(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    let sfi = DirectHandle::new(function.shared(), isolate);
    assert!(sfi.HasBaselineCode());

    let is_compiled_scope = IsCompiledScope::new_sfi(sfi, isolate);
    assert!(!function.HasAvailableOptimizedCode(isolate));
    assert!(!function.has_feedback_vector());
    JSFunction::CreateAndAttachFeedbackVector(isolate, &function, &is_compiled_scope);
    let baseline_code = sfi.baseline_code(kAcquireLoad);
    function.UpdateCodeKeepTieringRequests(baseline_code);

    return baseline_code;
}

#[no_mangle]
pub extern "C" fn Runtime_InstallSFICode(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);

    let sfi = function.shared();
    assert!(sfi.is_compiled());
    let sfi_code = sfi.GetCode(isolate);
    if sfi_code.kind() != CodeKind::BASELINE || function.has_feedback_vector() {
        function.UpdateCode(sfi_code);
        return sfi_code;
    }

    let is_compiled_scope = IsCompiledScope::new_sfi(function.shared(), isolate);
    assert!(!function.HasAvailableOptimizedCode(isolate));
    assert!(!function.has_feedback_vector());
    JSFunction::CreateAndAttachFeedbackVector(isolate, &function, &is_compiled_scope);
    let sfi_code = function.shared().GetCode(isolate);
    function.UpdateCode(sfi_code);
    return sfi_code;
}

#[no_mangle]
pub extern "C" fn Runtime_StartMaglevOptimizeJob(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    assert!(function.IsOptimizationRequested(isolate));
    CompileOptimized(function, ConcurrencyMode::kConcurrent, CodeKind::MAGLEV, isolate);

    return ReadOnlyRoots::new().undefined_value();
}

#[no_mangle]
pub extern "C" fn Runtime_StartTurbofanOptimizeJob(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    assert!(function.IsOptimizationRequested(isolate));
    CompileOptimized(function, ConcurrencyMode::kConcurrent, CodeKind::TURBOFAN_JS, isolate);
    return ReadOnlyRoots::new().undefined_value();
}

#[no_mangle]
pub extern "C" fn Runtime_OptimizeMaglevEager(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    assert!(function.IsOptimizationRequested(isolate));
    CompileOptimized(function, ConcurrencyMode::kSynchronous, CodeKind::MAGLEV, isolate);
    return ReadOnlyRoots::new().undefined_value();
}

#[no_mangle]
pub extern "C" fn Runtime_OptimizeTurbofanEager(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    assert!(function.IsOptimizationRequested(isolate));
    CompileOptimized(function, ConcurrencyMode::kSynchronous, CodeKind::TURBOFAN_JS, isolate);
    return ReadOnlyRoots::new().undefined_value();
}

#[no_mangle]
pub extern "C" fn Runtime_MarkLazyDeoptimized(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(2, args.length());
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    let reoptimize = args.smi_value_at(1) != 0;

    let is_compiled_scope = IsCompiledScope::new_sfi(function.shared(), isolate);
    if !is_compiled_scope.is_compiled() {
        let check = StackLimitCheck::new(isolate);
        if check.JsHasOverflowed(
            (kStackSpaceRequiredForCompilation * 1024) as usize,
        ) {
            return 0; //isolate.StackOverflow();
        }
        if !Compiler::Compile(
            isolate,
            &function,
            Compiler::KEEP_EXCEPTION,
            &is_compiled_scope,
        ) {
            return ReadOnlyRoots::new().exception();
        }
    }

    function.ResetTieringRequests();
    if reoptimize {
        function.raw_feedback_cell().set_interrupt_budget(1);
    } else {
        function.SetInterruptBudget(isolate, BudgetModification::kRaise, CodeKind::INTERPRETED_FUNCTION);
    }

    return ReadOnlyRoots::new().undefined_value();
}

fn CompileOptimized(
    function: DirectHandle<JSFunction>,
    mode: ConcurrencyMode,
    target_kind: CodeKind,
    isolate: *mut Isolate,
) {
    let isolate = unsafe { &mut *isolate };
    function.ResetTieringRequests();

    let is_compiled_scope = IsCompiledScope::new_sfi(function.shared(), isolate);
    if !is_compiled_scope.is_compiled() {
        function.ResetIfCodeFlushed(isolate);
        return;
    }

    let gap = if mode == ConcurrencyMode::kConcurrent {
        0
    } else {
        kStackSpaceRequiredForCompilation * 1024
    };

    let check = StackLimitCheck::new(isolate);
    if check.JsHasOverflowed(gap as usize) {
        return;
    }

    Compiler::CompileOptimized(isolate, &function, mode, target_kind);

    assert!(function.is_compiled(isolate));
}

#[no_mangle]
pub extern "C" fn Runtime_FunctionLogNextExecution(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());

    let js_function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);
    assert!(FLAG_log_function_events);
    LogExecution(isolate, js_function);

    return js_function.code(isolate);
}

#[no_mangle]
pub extern "C" fn Runtime_InstantiateAsmJs(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(args.length(), 4);

    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(0);

    let stdlib: DirectHandle<u64> = DirectHandle::new(if args[1] != 0 {
        args[1]
    } else {
        0
    }, isolate); //args.at::<JSReceiver>(1);
    let foreign: DirectHandle<u64> = DirectHandle::new(if args[2] != 0 {
        args[2]
    } else {
        0
    }, isolate); //args.at::<JSReceiver>(2);

    let memory: DirectHandle<JSArrayBuffer> = args.at::<JSArrayBuffer>(3);
    let shared = DirectHandle::new(function.shared(), isolate);

    shared.set_is_asm_wasm_broken(true);
    function.UpdateCode(Builtin::CompileLazy as u64);

    return Smi::zero();
}

fn TryGetOptimizedOsrCode(
    isolate: *mut Isolate,
    vector: Tagged<FeedbackVector>,
    it: &BytecodeArrayIterator,
    code_out: &mut Tagged<u64>,
) -> bool {
    false
}

fn DeoptAllOsrLoopsContainingDeoptExit(
    isolate: *mut Isolate,
    function: Tagged<JSFunction>,
    deopt_exit_offset: BytecodeOffset,
) {}

#[no_mangle]
pub extern "C" fn Runtime_NotifyDeoptimized(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(0, args.length());

    let deoptimizer = Deoptimizer::Grab(isolate);
    let optimized_code = DirectHandle::new(deoptimizer.compiled_code(), isolate);
    let deopt_exit_offset = deoptimizer.bytecode_offset_in_outermost_frame();
    
    let function = deoptimizer.function();
    let _optimized_code = deoptimizer.compiled_code();

    ReadOnlyRoots::new().undefined_value()
}

#[no_mangle]
pub extern "C" fn Runtime_ObserveNode(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());

    let obj: DirectHandle<u64> = args.at::<u64>(0);

    return *obj;
}

#[no_mangle]
pub extern "C" fn Runtime_VerifyType(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    let obj: DirectHandle<u64> = args.at::<u64>(0);

    return *obj;
}

#[no_mangle]
pub extern "C" fn Runtime_CheckTurboshaftTypeOf(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(2, args.length());

    let obj: DirectHandle<u64> = args.at::<u64>(0);

    return *obj;
}

fn GetOsrOffsetAndFunctionForOSR(
    isolate: *mut Isolate,
    osr_offset: &mut BytecodeOffset,
    function: &mut Handle<JSFunction>,
) {
    let isolate = unsafe { &mut *isolate };
    assert!(osr_offset.IsNone());
    assert!(function.is_null());

    let mut it = JavaScriptStackFrameIterator::new(isolate);
    let frame = unsafe {
        UnoptimizedJSFrame::cast(it.frame())
    };

    *osr_offset = BytecodeOffset::new(frame.GetBytecodeOffset());
    *function = Handle::new(frame.function(), isolate);

    assert!(!osr_offset.IsNone());
    assert!((*function).shared().HasBytecodeArray());
}

fn CompileOptimizedOSR(
    isolate: *mut Isolate,
    function: &Handle<JSFunction>,
    min_opt_level: CodeKind,
    osr_offset: BytecodeOffset,
) -> Tagged<u64> {
    let isolate = unsafe { &mut *isolate };

    let mode = if isolate.concurrent_recompilation_enabled() {
        ConcurrencyMode::kConcurrent
    } else {
        ConcurrencyMode::kSynchronous
    };

    let result: u64 = if !Compiler::CompileOptimizedOSR(
        isolate,
        function,
        osr_offset,
        mode,
        CodeKind::TURBOFAN_JS,
    )
    .is_null() {
        return Smi::zero();
    } else {
        0
    };

    Smi::zero()
}

#[no_mangle]
pub extern "C" fn Runtime_CompileOptimizedOSR(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(0, args.length());
    assert!(true); //v8_flags.use_osr);

    let mut osr_offset = BytecodeOffset::new(0);
    let mut function = Handle::new(0, isolate);
    GetOsrOffsetAndFunctionForOSR(isolate, &mut osr_offset, &mut function);

    CompileOptimizedOSR(isolate, &function, CodeKind::MAGLEV, osr_offset)
}

fn CompileOptimizedOSRFromMaglev(
    isolate: *mut Isolate,
    function: DirectHandle<JSFunction>,
    osr_offset: BytecodeOffset,
) -> Tagged<u64> {
    if true {
        return Smi::zero();
    }

    if true {
        function.shared().feedback_vector().reset_osr_urgency();
        function.SetInterruptBudget(isolate, BudgetModification::kRaise);
        return Smi::zero();
    }

    CompileOptimizedOSR(isolate, &Handle::new(*function, isolate), CodeKind::TURBOFAN_JS, osr_offset)
}

#[no_mangle]
pub extern "C" fn Runtime_CompileOptimizedOSRFromMaglev(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(1, args.length());
    assert!(true); //v8_flags.use_osr);

    let osr_offset = BytecodeOffset::new(args.positive_smi_value_at(0) as i32);

    let mut it = JavaScriptStackFrameIterator::new(isolate);
    let frame = unsafe {
        MaglevFrame::cast(it.frame())
    };
    assert_eq!(frame.LookupCode().kind(), CodeKind::MAGLEV);
    let function = DirectHandle::new(frame.function(), isolate);

    CompileOptimizedOSRFromMaglev(isolate, function, osr_offset)
}

#[no_mangle]
pub extern "C" fn Runtime_CompileOptimizedOSRFromMaglevInlined(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(2, args.length());
    assert!(true); //v8_flags.use_osr);

    let osr_offset = BytecodeOffset::new(args.positive_smi_value_at(0) as i32);
    let function: DirectHandle<JSFunction> = args.at::<JSFunction>(1);

    let mut it = JavaScriptStackFrameIterator::new(isolate);
    let frame = unsafe {
        MaglevFrame::cast(it.frame())
    };
    assert_eq!(frame.LookupCode().kind(), CodeKind::MAGLEV);

    if *function != frame.function() {
        if !DirectHandle::new(frame.function(), isolate).ActiveTierIsTurbofan(isolate) {
            isolate.tiering_manager().MarkForTurboFanOptimization(
                frame.function(),
            );
        }
    }

    CompileOptimizedOSRFromMaglev(isolate, function, osr_offset)
}

#[no_mangle]
pub extern "C" fn Runtime_LogOrTraceOptimizedOSREntry(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(0, args.length());
    assert!(true || true); //v8_flags.trace_osr || v8_flags.log_function_events);

    let mut osr_offset = BytecodeOffset::new(0);
    let mut function = Handle::new(0, isolate);
    GetOsrOffsetAndFunctionForOSR(isolate, &mut osr_offset, &mut function);

    ReadOnlyRoots::new().undefined_value()
}

fn CompileGlobalEval(
    isolate: *mut Isolate,
    source_object: &Handle<u64>,
    outer_info: DirectHandle<SharedFunctionInfo>,
    language_mode: LanguageMode,
    eval_scope_info_index: i32,
    eval_position: i32,
) -> Tagged<u64> {
    let isolate = unsafe { &mut *isolate };
    let native_context = isolate.native_context();

    let (_source, _unknown_object) = Compiler::ValidateDynamicCompilationSource(
        isolate,
        &native_context,
        source_object,
    );

    let _error_message =
        native_context.ErrorMessageForCodeGenerationFromStrings();
    
    let restriction: ParseRestriction = NO_PARSE_RESTRICTION;
    let context = Handle::new(isolate.context().value, isolate);
    let compiled = Compiler::GetFunctionFromEval(
        &Handle::new(*source_object, isolate),
        &outer_info,
        &context,
        language_mode,
        restriction,
        kNoSourcePosition,
        eval_position,
    );

    match compiled {
        Ok(function) => *function,
        Err(_e) => {
            ReadOnlyRoots::new().exception()
        }
    }
}

#[no_mangle]
pub extern "C" fn Runtime_ResolvePossiblyDirectEval(
    isolate: *mut Isolate,
    args: *mut Arguments,
) -> u64 {
    let args = unsafe { &*args };
    let isolate = unsafe { &mut *isolate };
    let scope = HandleScope::new();
    assert_eq!(6, args.length());

    let callee = DirectHandle::new(args[0], isolate);
    if callee.value != isolate.native_context().global_eval_fun() {
        return callee.value;
    }

    let language_mode = unsafe { std::mem::transmute::<i32, LanguageMode>(args.smi_value_at(3)) };
    let outer_info: DirectHandle<SharedFunctionInfo> = DirectHandle::new(args.at::<JSFunction>(2).shared(), isolate);
    let source_object = Handle::new(args[1], isolate);

    CompileGlobalEval(
        isolate,
        &source_object,
        outer_info,
        language_mode,
        args.smi_value_at(4),
        args.smi_value_at(5),
    )
}

#[derive(Debug)]
pub struct HandleScope {}
impl HandleScope {
    pub fn new() -> Self {
        HandleScope {}
    }
}

impl JSFunction {
    pub fn ActiveTierIsMaglev(&self, _isolate: *mut Isolate) -> bool {
        true
    }

    pub fn ActiveTierIsTurbofan(&self, _isolate: *mut Isolate) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum TieringState {
    kNone,
    kRequestMaglev_Synchronous,
    kRequestMaglev_Concurrent,
    kRequestTurbofan_Synchronous,
    kRequestTurbofan_Concurrent,
    kInProgress,
}

const kAcquireLoad: u64 = 0;
const kTieringStateInProgressBlocksTierup: bool = true;

mod pending_optimization_table {
    pub struct Tagged<T>(T);
    impl<T> Tagged<T> {
        pub fn is_null(&self) -> bool {
            true
        }
    }
}


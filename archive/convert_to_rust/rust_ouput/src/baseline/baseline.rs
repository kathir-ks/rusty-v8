// Converted from V8 C++ source files:
// Header: baseline.h
// Implementation: baseline.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod baseline {
    // Empty module. Implemented in .cc file to allow conditional compilation.
}

use std::ptr::null_mut;

pub use baseline::*;

#[cfg(feature = "sparkplug")]
mod baseline_impl {
    use super::*;
    use crate::debug::debug::Debug;
    use crate::execution::isolate::Isolate;
    use crate::handles::{Handle, MaybeDirectHandle};
    use crate::heap::factory::Factory;
    use crate::init::setup_isolate::Code;
    use crate::logging::runtime_call_stats_scope::RuntimeCallCounterId;
    use crate::objects::script::Script;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::objects::shared_function_info_inl::SharedFunctionInfoInl;
    use crate::objects::tagged::Tagged;
    use crate::snapshot::snapshot_data::DisallowGarbageCollection;
    use v8_flags;
    use crate::objects::bytecode_array::BytecodeArray;
    use crate::baseline::baseline_assembler_inl::BaselineAssembler;
    use crate::baseline::baseline_compiler::BaselineCompiler;
    use crate::init::setup_isolate::V8;
    use crate::local_isolate::LocalIsolate;
    use crate::handles::handle::ToHandle;
    use crate::compiler::code_printer::PrintCode;
    use crate::compiler::macro_assembler::MacroAssembler;

    #[derive(Debug)]
    pub enum BaselineError {
        SparkplugDisabled,
        ShortBuiltinsDisabled,
        NoBytecode,
        DebuggerNeedsCheck,
        HasBreakInfo,
        HasInstrumentedBytecode,
        FilterFailed,
        CompilationFailed,
    }

    pub fn can_compile_with_baseline(
        isolate: *mut Isolate,
        shared: Tagged<SharedFunctionInfo>,
    ) -> bool {
        let no_gc = DisallowGarbageCollection {};

        // Check that baseline compiler is enabled.
        if !unsafe { v8_flags::sparkplug } {
            return false;
        }

        // Check that short builtin calls are enabled if needed.
        if unsafe { v8_flags::sparkplug_needs_short_builtins }
            && !unsafe { isolate.as_ref().unwrap().is_short_builtin_calls_enabled() }
        {
            return false;
        }

        // Check if we actually have bytecode.
        if !shared.has_bytecode_array() {
            return false;
        }

        // Do not optimize when debugger needs to hook into every call.
        if unsafe { isolate.as_ref().unwrap().debug().needs_check_on_function_call() } {
            return false;
        }

        if let Some(debug_info) = shared.try_get_debug_info(unsafe { isolate.as_mut().unwrap() }) {
            // Functions with breakpoints have to stay interpreted.
            if debug_info.has_break_info() {
                return false;
            }

            // Functions with instrumented bytecode can't be baseline compiled since the
            // baseline code's bytecode array pointer is immutable.
            if debug_info.has_instrumented_bytecode_array() {
                return false;
            }
        }

        // Do not baseline compile if function doesn't pass sparkplug_filter.
        if !shared.passes_filter(unsafe { v8_flags::sparkplug_filter }) {
            return false;
        }

        true
    }

    pub fn generate_baseline_code(
        isolate: *mut Isolate,
        shared: Handle<SharedFunctionInfo>,
    ) -> MaybeDirectHandle<Code> {
        unsafe {
            let _rcs_scope =
                RuntimeCallStatsScope::new(isolate, RuntimeCallCounterId::kCompileBaseline);
            let bytecode_array = shared.get_bytecode_array(isolate);
            let bytecode = Handle::new(bytecode_array, isolate);
            let local_isolate = isolate.as_mut().unwrap().main_thread_local_isolate();
            let mut compiler = BaselineCompiler::new(local_isolate, shared, bytecode);
            compiler.generate_code();
            let code = compiler.build();
            if v8_flags::print_code && !code.is_null() {
                PrintCode(code.to_handle_checked().into_raw());
            }
            code
        }
    }

    pub fn emit_return_baseline(masm: *mut MacroAssembler) {
        unsafe {
            BaselineAssembler::emit_return(masm);
        }
    }
}

#[cfg(not(feature = "sparkplug"))]
mod baseline_impl {
    use super::*;
    use crate::handles::{Handle, MaybeDirectHandle};
    use crate::init::setup_isolate::Code;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::objects::tagged::Tagged;
    use crate::execution::isolate::Isolate;
    use crate::compiler::macro_assembler::MacroAssembler;

    pub fn can_compile_with_baseline(
        _isolate: *mut Isolate,
        _shared: Tagged<SharedFunctionInfo>,
    ) -> bool {
        false
    }

    pub fn generate_baseline_code(
        _isolate: *mut Isolate,
        _shared: Handle<SharedFunctionInfo>,
    ) -> MaybeDirectHandle<Code> {
        MaybeDirectHandle::null()
    }

   pub fn emit_return_baseline(_masm: *mut MacroAssembler) {
        panic!("UNREACHABLE");
    }
}

pub use baseline_impl::*;

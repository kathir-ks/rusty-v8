// Converted from V8 C++ source files:
// Header: shared-function-info-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod shared_function_info_inl {
use std::mem;
use crate::v8::internal::Builtin;
use crate::v8::internal::IsolateForSandbox;
use crate::v8::internal::LanguageMode;
use crate::v8::internal::PtrComprCageBase;
use crate::v8::internal::Smi;
use crate::v8::internal::WriteBarrierMode;
use crate::v8::internal::BailoutReason;
use crate::v8::internal::FunctionKind;
use crate::v8::internal::ScopeInfo;
use crate::v8::internal::Tagged;
use crate::v8::internal::Object;
use crate::v8::internal::HeapObject;
use crate::v8::internal::String;
use crate::v8::internal::Code;
use crate::v8::internal::ObjectSlot;
use crate::v8::internal::Address;
use crate::v8::internal::Heap;
use crate::v8::internal::Script;
use crate::v8::internal::UncompiledData;
use crate::v8::internal::ExposedTrustedObject;
use crate::v8::internal::IndirectPointerTag;
use crate::v8::internal::IndirectPointerHandle;

    pub struct PreparseData {}

    impl PreparseData {
        pub fn inner_start_offset(&self) -> i32 {
            self.data_length()
        }

        pub fn inner_data_start(&self) -> ObjectSlot {
            ObjectSlot {}
        }

        pub fn clear_padding(&self) {}

        pub fn get(&self, index: i32) -> u8 {
            0
        }

        pub fn set(&self, index: i32, value: u8) {}

        pub fn copy_in(&self, index: i32, buffer: *const u8, length: i32) {}

        pub fn get_child(&self, index: i32) -> Tagged<PreparseData> {
            Tagged {  }
        }

        pub fn get_child_raw(&self, index: i32) -> Tagged<Object> {
            Tagged {  }
        }

        pub fn set_child(&self, index: i32, value: Tagged<PreparseData>, mode: WriteBarrierMode) {}
    
        fn data_length(&self) -> i32 {
            0
        }
    
        fn address(&self) -> usize {
            0
        }
    }

    pub struct UncompiledData {}

    impl UncompiledData {
        pub fn init_after_bytecode_flush(&self, isolate: *mut Isolate, inferred_name: Tagged<String>, start_position: i32, end_position: i32, gc_notify_updated_slot: impl Fn(Tagged<HeapObject>, ObjectSlot, Tagged<HeapObject>)) {}
    
        fn inferred_name(&self, cage_base: PtrComprCageBase) -> Tagged<String> {
            Tagged {  }
        }
        
        fn set_inferred_name(&self, inferred_name: Tagged<String>) {}

        fn set_start_position(&self, start_position: i32) {}

        fn set_end_position(&self, end_position: i32) {}
    }

    pub struct UncompiledDataWithoutPreparseData {}

    pub struct UncompiledDataWithPreparseData {}

    pub struct UncompiledDataWithoutPreparseDataWithJob {}

    pub struct UncompiledDataWithPreparseDataAndJob {}

    pub struct InterpreterData {}

    impl InterpreterData {
        pub fn set_bytecode_array(&self, bytecode: Tagged<crate::v8::internal::BytecodeArray>) {}
    }

    pub struct SharedFunctionInfo {}

    impl SharedFunctionInfo {
        pub fn SetTrustedData(&self, value: Tagged<ExposedTrustedObject>, mode: WriteBarrierMode) {}

        pub fn SetUntrustedData(&self, value: Tagged<Object>, mode: WriteBarrierMode) {}

        pub fn HasTrustedData(&self) -> bool {
            true
        }

        pub fn HasUntrustedData(&self) -> bool {
            true
        }

        pub fn GetTrustedData(&self, isolate: IsolateForSandbox) -> Tagged<Object> {
            Tagged {  }
        }

        pub fn GetUntrustedData(&self) -> Tagged<Object> {
            Tagged {  }
        }

        pub fn script(&self) -> Tagged<HeapObject> {
            Tagged {  }
        }

        pub fn raw_outer_scope_info_or_feedback_metadata(&self) -> Tagged<HeapObject> {
            Tagged {  }
        }

        pub fn internal_formal_parameter_count_with_receiver(&self) -> u16 {
            0
        }

        pub fn internal_formal_parameter_count_without_receiver(&self) -> u16 {
            0
        }

        pub fn set_internal_formal_parameter_count(&self, value: i32) {}

        pub fn raw_function_token_offset(&self) -> u16 {
            0
        }

        pub fn flags(&self, mode: crate::v8::internal::LoadStoreMode) -> i32 {
            0
        }

        pub fn relaxed_flags(&self) -> i32 {
            0
        }

        pub fn set_relaxed_flags(&self, flags: i32) {}

        pub fn set_flags(&self, flags: i32, mode: crate::v8::internal::LoadStoreMode) {}

        pub fn flags2(&self) -> u8 {
            0
        }

        pub fn HasSharedName(&self) -> bool {
            true
        }

        pub fn Name(&self) -> Tagged<String> {
            Tagged {  }
        }

        pub fn SetName(&self, name: Tagged<String>) {}

        pub fn is_script(&self) -> bool {
            true
        }

        pub fn needs_script_context(&self) -> bool {
            true
        }

        pub fn abstract_code(&self, isolate: *mut Isolate) -> Tagged<crate::v8::internal::AbstractCode> {
            Tagged {  }
        }

        pub fn function_token_position(&self) -> i32 {
            0
        }

        pub fn AreSourcePositionsAvailable<T>(&self, isolate: *mut T) -> bool {
            true
        }

        pub fn GetInlineability<T>(&self, isolate: *mut T) -> Inlineability {
            Inlineability::kIsInlineable
        }

        pub fn set_class_scope_has_private_brand(&self, value: bool) {}

        pub fn has_static_private_methods_or_accessors(&self) -> bool {
            true
        }

        pub fn set_has_static_private_methods_or_accessors(&self, value: bool) {}

        pub fn is_sparkplug_compiling(&self) -> bool {
            true
        }

        pub fn set_is_sparkplug_compiling(&self, value: bool) {}

        pub fn maglev_compilation_failed(&self) -> bool {
            true
        }

        pub fn set_maglev_compilation_failed(&self, value: bool) {}

        pub fn function_context_independent_compiled(&self) -> bool {
            true
        }

        pub fn set_function_context_independent_compiled(&self, value: bool) {}

        pub fn syntax_kind(&self) -> FunctionSyntaxKind {
            FunctionSyntaxKind::kNormal
        }

        pub fn set_syntax_kind(&self, value: FunctionSyntaxKind) {}

        pub fn allows_lazy_compilation(&self) -> bool {
            true
        }

        pub fn set_allows_lazy_compilation(&self, value: bool) {}

        pub fn has_duplicate_parameters(&self) -> bool {
            true
        }

        pub fn set_has_duplicate_parameters(&self, value: bool) {}

        pub fn native(&self) -> bool {
            true
        }

        pub fn set_native(&self, value: bool) {}

        pub fn is_asm_wasm_broken(&self) -> bool {
            true
        }

        pub fn set_is_asm_wasm_broken(&self, value: bool) {}

        pub fn requires_instance_members_initializer(&self) -> bool {
            true
        }

        pub fn set_requires_instance_members_initializer(&self, value: bool) {}

        pub fn name_should_print_as_anonymous(&self) -> bool {
            true
        }

        pub fn set_name_should_print_as_anonymous(&self, value: bool) {}

        pub fn has_reported_binary_coverage(&self) -> bool {
            true
        }

        pub fn set_has_reported_binary_coverage(&self, value: bool) {}

        pub fn is_toplevel(&self) -> bool {
            true
        }

        pub fn set_is_toplevel(&self, value: bool) {}

        pub fn properties_are_final(&self) -> bool {
            true
        }

        pub fn set_properties_are_final(&self, value: bool) {}

        pub fn private_name_lookup_skips_outer_class(&self) -> bool {
            true
        }

        pub fn set_private_name_lookup_skips_outer_class(&self, value: bool) {}

        pub fn optimization_disabled(&self) -> bool {
            true
        }

        pub fn disabled_optimization_reason(&self) -> BailoutReason {
            BailoutReason::kNoReason
        }

        pub fn language_mode(&self) -> LanguageMode {
            LanguageMode::kSloppy
        }

        pub fn set_language_mode(&self, language_mode: LanguageMode) {}

        pub fn kind(&self) -> FunctionKind {
            FunctionKind::kNormalFunction
        }

        pub fn set_kind(&self, kind: FunctionKind) {}

        pub fn is_wrapped(&self) -> bool {
            true
        }

        pub fn construct_as_builtin(&self) -> bool {
            true
        }

        pub fn CalculateConstructAsBuiltin(&self) {}

        pub fn age(&self) -> u16 {
            0
        }

        pub fn set_age(&self, value: u16) {}

        pub fn CompareExchangeAge(&self, expected_age: u16, new_age: u16) -> u16 {
            0
        }

        pub fn function_map_index(&self) -> i32 {
            0
        }

        pub fn set_function_map_index(&self, index: i32) {}

        pub fn clear_padding(&self) {}

        pub fn UpdateFunctionMapIndex(&self) {}

        pub fn DontAdaptArguments(&self) {}

        pub fn IsDontAdaptArguments(&self) -> bool {
            true
        }

        pub fn scope_info(&self, cage_base: PtrComprCageBase, mode: crate::v8::internal::AcquireLoadTag) -> Tagged<ScopeInfo> {
            Tagged {  }
        }

        pub fn EarlyScopeInfo(&self, mode: crate::v8::internal::AcquireLoadTag) -> Tagged<ScopeInfo> {
            Tagged {  }
        }

        pub fn SetScopeInfo(&self, scope_info: Tagged<ScopeInfo>, mode: WriteBarrierMode) {}

        pub fn set_raw_scope_info(&self, scope_info: Tagged<ScopeInfo>, mode: WriteBarrierMode) {}

        pub fn outer_scope_info(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
            Tagged {  }
        }

        pub fn HasOuterScopeInfo(&self) -> bool {
            true
        }

        pub fn GetOuterScopeInfo(&self) -> Tagged<ScopeInfo> {
            Tagged {  }
        }

        pub fn set_outer_scope_info(&self, value: Tagged<HeapObject>, mode: WriteBarrierMode) {}

        pub fn HasFeedbackMetadata(&self) -> bool {
            true
        }

        pub fn feedback_metadata(&self, cage_base: PtrComprCageBase) -> Tagged<crate::v8::internal::FeedbackMetadata> {
            Tagged {  }
        }

        pub fn is_compiled(&self) -> bool {
            true
        }

        pub fn is_compiled_scope<T>(&self, isolate: *mut T) -> IsCompiledScope {
            IsCompiledScope { is_compiled_: true, retain_code_: crate::v8::internal::MaybeHandle {  } }
        }

        pub fn has_simple_parameters(&self) -> bool {
            true
        }

        pub fn CanCollectSourcePosition(&self, isolate: *mut Isolate) -> bool {
            true
        }

        pub fn IsApiFunction(&self) -> bool {
            true
        }

        pub fn api_func_data(&self) -> Tagged<crate::v8::internal::FunctionTemplateInfo> {
            Tagged {  }
        }

        pub fn HasBytecodeArray(&self, cage_base: PtrComprCageBase) -> bool {
            true
        }

        pub fn GetBytecodeArray<T>(&self, isolate: *mut T) -> Tagged<crate::v8::internal::BytecodeArray> {
            Tagged {  }
        }

        pub fn GetActiveBytecodeArray(&self, isolate: IsolateForSandbox) -> Tagged<crate::v8::internal::BytecodeArray> {
            Tagged {  }
        }

        pub fn SetActiveBytecodeArray(&self, bytecode: Tagged<crate::v8::internal::BytecodeArray>, isolate: IsolateForSandbox) {}

        pub fn set_bytecode_array(&self, bytecode: Tagged<crate::v8::internal::BytecodeArray>) {}

        pub fn overwrite_bytecode_array(&self, bytecode: Tagged<crate::v8::internal::BytecodeArray>) {}

        pub fn InterpreterTrampoline(&self, isolate: IsolateForSandbox) -> Tagged<Code> {
            Tagged {  }
        }

        pub fn HasInterpreterData(&self, isolate: IsolateForSandbox) -> bool {
            true
        }

        pub fn interpreter_data(&self, isolate: IsolateForSandbox) -> Tagged<InterpreterData> {
            Tagged {  }
        }

        pub fn set_interpreter_data(&self, isolate: *mut Isolate, interpreter_data: Tagged<InterpreterData>, mode: WriteBarrierMode) {}

        pub fn HasBaselineCode(&self, cage_base: PtrComprCageBase) -> bool {
            true
        }

        pub fn baseline_code(&self, cage_base: PtrComprCageBase) -> Tagged<Code> {
            Tagged {  }
        }

        pub fn set_baseline_code(&self, baseline_code: Tagged<Code>, tag: crate::v8::internal::ReleaseStoreTag, mode: WriteBarrierMode) {}

        pub fn FlushBaselineCode(&self) {}

        pub fn HasAsmWasmData(&self) -> bool {
            true
        }

        pub fn HasWasmFunctionData(&self) -> bool {
            true
        }

        pub fn HasWasmExportedFunctionData(&self) -> bool {
            true
        }

        pub fn HasWasmJSFunctionData(&self) -> bool {
            true
        }

        pub fn HasWasmCapiFunctionData(&self) -> bool {
            true
        }

        pub fn HasWasmResumeData(&self) -> bool {
            true
        }

        pub fn asm_wasm_data(&self) -> Tagged<crate::v8::internal::AsmWasmData> {
            Tagged {  }
        }

        pub fn set_asm_wasm_data(&self, data: Tagged<crate::v8::internal::AsmWasmData>, mode: WriteBarrierMode) {}

        pub fn wasm_function_data(&self) -> Tagged<crate::v8::internal::WasmFunctionData> {
            Tagged {  }
        }

        pub fn wasm_exported_function_data(&self) -> Tagged<crate::v8::internal::WasmExportedFunctionData> {
            Tagged {  }
        }

        pub fn wasm_js_function_data(&self) -> Tagged<crate::v8::internal::WasmJSFunctionData> {
            Tagged {  }
        }

        pub fn wasm_capi_function_data(&self) -> Tagged<crate::v8::internal::WasmCapiFunctionData> {
            Tagged {  }
        }

        pub fn wasm_resume_data(&self) -> Tagged<crate::v8::internal::WasmResumeData> {
            Tagged {  }
        }

        pub fn HasBuiltinId(&self) -> bool {
            true
        }

        pub fn builtin_id(&self) -> Builtin {
            Builtin::kAbort
        }

        pub fn set_builtin_id(&self, builtin: Builtin) {}

        pub fn HasUncompiledData(&self) -> bool {
            true
        }

        pub fn uncompiled_data(&self, isolate: IsolateForSandbox) -> Tagged<UncompiledData> {
            Tagged {  }
        }

        pub fn set_uncompiled_data(&self, uncompiled_data: Tagged<UncompiledData>, mode: WriteBarrierMode) {}

        pub fn HasUncompiledDataWithPreparseData(&self) -> bool {
            true
        }

        pub fn uncompiled_data_with_preparse_data(&self, isolate: IsolateForSandbox) -> Tagged<UncompiledDataWithPreparseData> {
            Tagged {  }
        }

        pub fn set_uncompiled_data_with_preparse_data(&self, uncompiled_data_with_preparse_data: Tagged<UncompiledDataWithPreparseData>, mode: WriteBarrierMode) {}

        pub fn HasUncompiledDataWithoutPreparseData(&self) -> bool {
            true
        }

        pub fn ClearUncompiledDataJobPointer(&self, isolate: IsolateForSandbox) {}

        pub fn ClearPreparseData(&self, isolate: IsolateForSandbox) {}

        pub fn is_repl_mode(&self) -> bool {
            true
        }

        pub fn HasInferredName(&self) -> bool {
            true
        }

        pub fn inferred_name(&self) -> Tagged<String> {
            Tagged {  }
        }

        pub fn IsUserJavaScript(&self) -> bool {
            true
        }

        pub fn IsSubjectToDebugging(&self) -> bool {
            true
        }

        pub fn CanDiscardCompiled(&self) -> bool {
            true
        }

        pub fn is_class_constructor(&self) -> bool {
            true
        }

        pub fn set_are_properties_final(&self, value: bool) {}

        pub fn are_properties_final(&self) -> bool {
            true
        }

        fn set_name_or_scope_info(&self, scope_info: Tagged<ScopeInfo>, mode: crate::v8::internal::ReleaseStoreTag, wb_mode: WriteBarrierMode) {}

        fn set_raw_outer_scope_info_or_feedback_metadata(&self, value: Tagged<HeapObject>, mode: WriteBarrierMode) {}

        fn outer_scope_info(&self) -> Tagged<Object> {
            Tagged {  }
        }

        fn raw_outer_scope_info_or_feedback_metadata(&self, tag: crate::v8::internal::AcquireLoadTag) -> Tagged<HeapObject> {
            Tagged {  }
        }

        fn set_properties_are_final(&self, value: bool) {}

        fn name_or_scope_info(&self, tag: crate::v8::internal::AcquireLoadTag) -> Tagged<Object> {
            Tagged {  }
        }
    }

    pub struct IsCompiledScope {
        is_compiled_: bool,
        retain_code_: crate::v8::internal::MaybeHandle<HeapObject>,
    }

    pub enum Inlineability {
        kHasNoScript,
        kNeedsBinaryCoverage,
        kIsBuiltin,
        kIsNotUserCode,
        kHasNoBytecode,
        kExceedsBytecodeLimit,
        kMayContainBreakPoints,
        kHasOptimizationDisabled,
        kIsInlineable,
    }

    pub enum FunctionSyntaxKind {
        kNormal,
        kWrapped,
    }

    pub struct SharedFunctionInfoWrapper {}

    impl SharedFunctionInfoWrapper {
        pub fn shared_info(&self) -> Tagged<SharedFunctionInfo> {
            Tagged {  }
        }
    }

    pub struct Isolate {}
    pub struct LocalIsolate {}
}

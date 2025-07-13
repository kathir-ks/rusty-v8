// Converted from V8 C++ source files:
// Header: N/A
// Implementation: setup-builtins-internal.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::Write,
    mem::MaybeUninit,
    rc::Rc,
    sync::{Arc, Mutex},
};

// use crate::builtins::ProfileDataFromFile; // Assuming this is the correct path
use crate::codegen::{AssemblerOptions, CodeDesc};
use crate::compiler::pipeline::CodeAssemblerState;
// use crate::compiler::turboshaft::PipelineData; // Assuming this is the correct path
use crate::heap::{DisallowGarbageCollection, Heap, WritableJitAllocation};
use crate::interpreter;
use crate::objects::CodeKind;
use crate::sandbox::SandboxedPointerConstants;
use crate::wasm;

// use crate::builtins::Builtins; // Assuming this is the correct path
// use crate::codegen::MacroAssembler; // Assuming this is the correct path
// use crate::codegen::RelocInfo; // Assuming this is the correct path
// use crate::compiler::Pipeline; // Assuming this is the correct path
// use crate::execution::Isolate; // Assuming this is the correct path
// use crate::handles::Handles; // Assuming this is the correct path
// use crate::heap::Factory; // Assuming this is the correct path
// use crate::interpreter::Interpreter; // Assuming this is the correct path
// use crate::objects::Objects; // Assuming this is the correct path

const KB: usize = 1024;
const MB: usize = 1024 * KB;

const kBufferSize: usize = 128 * KB;
const kMaxPCRelativeCodeRangeInMB: f32 = 2.0;

pub struct SetupIsolateDelegate {}

impl SetupIsolateDelegate {
    pub fn add_builtin(builtins: &mut Builtins, builtin: Builtin, code: Tagged<Code>) {
        assert_eq!(builtin, code.builtin_id());
        builtins.set_code(builtin, code);
    }

    pub fn populate_with_placeholders(isolate: &mut Isolate) {
        let builtins = &mut isolate.builtins;
        let mut scope = HandleScope::new(isolate);

        for builtin in Builtin::kFirst..=Builtin::kLast {
            let placeholder = Self::build_placeholder(isolate, builtin);
            Self::add_builtin(builtins, builtin, *placeholder);
        }
    }

    fn build_placeholder(isolate: &mut Isolate, builtin: Builtin) -> DirectHandle<Code> {
        let mut scope = HandleScope::new(isolate);
        let mut buffer: [u8; kBufferSize] = [0; kBufferSize];
        let mut masm = MacroAssembler::new(
            isolate,
            CodeObjectRequired::kYes,
            ExternalAssemblerBuffer {
                buffer: &mut buffer,
                size: kBufferSize,
            },
        );

        assert!(!masm.has_frame());
        {
            let mut frame_scope = FrameScope::new(&mut masm, StackFrame::NO_FRAME_TYPE);

            masm.move_(kJavaScriptCallCodeStartRegister, Smi::zero());
            masm.call(kJavaScriptCallCodeStartRegister);
        }

        let mut desc = CodeDesc::default();
        masm.get_code(isolate, &mut desc);

        let code = Factory::code_builder(isolate, desc, CodeKind::BUILTIN)
            .set_self_reference(masm.code_object())
            .set_builtin(builtin)
            .build();
        scope.close_and_escape(code)
    }

    pub fn replace_placeholders(isolate: &mut Isolate) {
        let builtins = &mut isolate.builtins;
        let _no_gc = DisallowGarbageCollection::new(); // RAII guard

        const kRelocMask: i32 = RelocInfo::mode_mask(RelocInfo::CODE_TARGET)
            | RelocInfo::mode_mask(RelocInfo::FULL_EMBEDDED_OBJECT)
            | RelocInfo::mode_mask(RelocInfo::COMPRESSED_EMBEDDED_OBJECT)
            | RelocInfo::mode_mask(RelocInfo::RELATIVE_CODE_TARGET);

        let cage_base = PtrComprCageBase::new(isolate);

        for builtin in Builtin::kFirst..=Builtin::kLast {
            let code = builtins.code(builtin);
            let istream = code.instruction_stream();

            let jit_allocation = ThreadIsolation::lookup_jit_allocation(
                istream.address(),
                istream.size(),
                ThreadIsolation::JitAllocationType::kInstructionStream,
                true,
            );

            let mut flush_icache = false;
            let mut it = WritableRelocIterator::new(jit_allocation, istream, code.constant_pool(), kRelocMask);

            while !it.done() {
                let rinfo = it.rinfo();

                if RelocInfo::is_code_target_mode(rinfo.rmode()) {
                    let target_code = Code::from_target_address(rinfo.target_address());

                    if !target_code.is_builtin() {
                        continue;
                    }

                    let new_target = builtins.code(target_code.builtin_id());
                    rinfo.set_target_address(
                        istream,
                        new_target.instruction_start(),
                        UPDATE_WRITE_BARRIER,
                        SKIP_ICACHE_FLUSH,
                    );
                } else {
                    assert!(RelocInfo::is_embedded_object_mode(rinfo.rmode()));

                    let object = rinfo.target_object(cage_base);
                    if !is_code(object, cage_base) {
                        continue;
                    }

                    let target = cast::<Code>(object);

                    if !target.is_builtin() {
                        continue;
                    }

                    let new_target = builtins.code(target.builtin_id());
                    rinfo.set_target_object(istream, new_target, UPDATE_WRITE_BARRIER, SKIP_ICACHE_FLUSH);
                }
                flush_icache = true;
                it.next();
            }

            if flush_icache {
                flush_instruction_cache(code.instruction_start(), code.instruction_size());
            }
        }
    }

    pub fn setup_builtins_internal(isolate: &mut Isolate) {
        let builtins = &mut isolate.builtins;
        assert!(!builtins.initialized_);

        if isolate.v8_flags.dump_builtins_hashes_to_file.is_some() {
            let filename = isolate.v8_flags.dump_builtins_hashes_to_file.as_ref().unwrap();
            if let Ok(file) = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(filename)
            {
                drop(file); // Create empty file
            }
        }

        Self::populate_with_placeholders(isolate);

        let mut scope = HandleScope::new(isolate);

        let mut generated_builtins: Vec<Handle<Code>> = Vec::with_capacity(Builtins::kBuiltinCount);
        for _ in 0..Builtins::kBuiltinCount {
            generated_builtins.push(Handle::empty());
        }

        let install_generated_builtin =
            |generated_builtins: &mut Vec<Handle<Code>>, isolate: &mut Isolate, builtin: Builtin, code: Handle<Code>| {
                assert_eq!(ThreadId::current(), isolate.thread_id());
                generated_builtins[Builtins::to_int(builtin)] = code;
            };

        let mut builtins_built_without_job_count = 0;
        let mut job_creation_order = 0;
        let mut scheduler = BuiltinCompilationScheduler::new();

        let mut code: Tagged<Code>;

        macro_rules! build_cpp_without_job {
            ($Name:ident, $Argc:expr) => {{
                code = Self::build_adaptor(
                    isolate,
                    Builtin::k##$Name,
                    function_addr(Builtin_##$Name),
                    stringify!($Name),
                );
                generated_builtins[Builtins::to_int(Builtin::k##$Name)] =
                    Handle::new(code, isolate);
                builtins_built_without_job_count += 1;
            }};
        }

        macro_rules! build_tsj_without_job {
            ($Name:ident, $Argc:expr, $($rest:tt)*) => {{
                code = Self::build_with_turboshaft_assembler_js(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    $Argc,
                    stringify!($Name),
                );
                Self::add_builtin(builtins, Builtin::k##$Name, code);
                builtins_built_without_job_count += 1;
            }};
        }

        macro_rules! build_tfj_with_job {
            ($Name:ident, $Argc:expr, $($rest:tt)*) => {{
                Self::compile_js_linkage_code_stub_builtin(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    &install_generated_builtin,
                    $Argc,
                    stringify!($Name),
                    job_creation_order,
                    &mut scheduler,
                );
                job_creation_order += 1;
            }};
        }

        macro_rules! build_tsc_without_job {
            ($Name:ident, $InterfaceDescriptor:ident) => {{
                code = Self::build_with_turboshaft_assembler_cs(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    CallDescriptors::$InterfaceDescriptor,
                    stringify!($Name),
                );
                generated_builtins[Builtins::to_int(Builtin::k##$Name)] =
                    Handle::new(code, isolate);
                builtins_built_without_job_count += 1;
            }};
        }

        macro_rules! build_tfc_with_job {
            ($Name:ident, $InterfaceDescriptor:ident) => {{
                Self::compile_cs_linkage_code_stub_builtin(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    &install_generated_builtin,
                    CallDescriptors::$InterfaceDescriptor,
                    stringify!($Name),
                    job_creation_order,
                    &mut scheduler,
                );
                job_creation_order += 1;
            }};
        }

        macro_rules! build_tfs_with_job {
            ($Name:ident, $($rest:tt)*) => {{
                Self::compile_cs_linkage_code_stub_builtin(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    &install_generated_builtin,
                    CallDescriptors::$Name,
                    stringify!($Name),
                    job_creation_order,
                    &mut scheduler,
                );
                job_creation_order += 1;
            }};
        }

        macro_rules! build_tfh_with_job {
            ($Name:ident, $InterfaceDescriptor:ident) => {{
                Self::compile_cs_linkage_code_stub_builtin(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    &install_generated_builtin,
                    CallDescriptors::$InterfaceDescriptor,
                    stringify!($Name),
                    job_creation_order,
                    &mut scheduler,
                );
                job_creation_order += 1;
            }};
        }

        macro_rules! build_bch_with_job {
            ($Name:ident, $OperandScale:ident, $Bytecode:ident) => {{
                Self::compile_bytecode_handler(
                    isolate,
                    Builtin::k##$Name,
                    interpreter::OperandScale::$OperandScale,
                    interpreter::Bytecode::$Bytecode,
                    &install_generated_builtin,
                    job_creation_order,
                    &mut scheduler,
                );
                job_creation_order += 1;
            }};
        }

        macro_rules! build_asm_without_job {
            ($Name:ident, $InterfaceDescriptor:ident) => {{
                code = Self::build_with_macro_assembler(
                    isolate,
                    Builtin::k##$Name,
                    &Builtins::generate_##$Name,
                    stringify!($Name),
                );
                generated_builtins[Builtins::to_int(Builtin::k##$Name)] =
                    Handle::new(code, isolate);
                builtins_built_without_job_count += 1;
            }};
        }

        macro_rules! nop {
            ($($rest:tt)*) => {};
        }

        crate::builtins::builtin_list!(
            build_cpp_without_job,
            build_tsj_without_job,
            nop,
            build_tsc_without_job,
            nop,
            nop,
            nop,
            nop,
            build_asm_without_job
        );

        crate::builtins::builtin_list!(
            nop,
            nop,
            build_tfj_with_job,
            nop,
            build_tfc_with_job,
            build_tfs_with_job,
            build_tfh_with_job,
            build_bch_with_job,
            nop
        );

        scheduler.await_and_finalize_current_batch(isolate);
        assert_eq!(
            Builtins::kBuiltinCount,
            builtins_built_without_job_count + scheduler.builtins_installed_count()
        );

        for builtin in Builtin::kFirst..=Builtin::kLast {
            Self::add_builtin(
                builtins,
                builtin,
                *generated_builtins[Builtins::to_int(builtin)].clone(),
            );
        }

        Self::replace_placeholders(isolate);

        builtins.mark_initialized();
    }

    fn build_adaptor(
        isolate: &mut Isolate,
        builtin: Builtin,
        builtin_address: Address,
        name: &str,
    ) -> Tagged<Code> {
        let mut scope = HandleScope::new(isolate);
        let mut buffer: [u8; kBufferSize] = [0; kBufferSize];
        let mut masm = MacroAssembler::new(
            isolate,
            Self::builtin_assembler_options(isolate, builtin),
            CodeObjectRequired::kYes,
            ExternalAssemblerBuffer {
                buffer: &mut buffer,
                size: kBufferSize,
            },
        );
        masm.set_builtin(builtin);
        assert!(!masm.has_frame());

        let formal_parameter_count = Builtins::get_formal_parameter_count(builtin);
        Builtins::generate_adaptor(&mut masm, formal_parameter_count, builtin_address);

        let mut desc = CodeDesc::default();
        masm.get_code(isolate, &mut desc);

        let code = Factory::code_builder(isolate, desc, CodeKind::BUILTIN)
            .set_self_reference(masm.code_object())
            .set_builtin(builtin)
            .set_parameter_count(formal_parameter_count)
            .build();
        *code
    }

    fn build_with_turboshaft_assembler_js(
        isolate: &mut Isolate,
        builtin: Builtin,
        generator: &dyn Fn(&mut PipelineData, &mut Isolate, &mut Graph, &mut Zone), // TurboshaftAssemblerGenerator,
        argc: i32,
        name: &str,
    ) -> Tagged<Code> {
        let mut scope = HandleScope::new(isolate);
        let code = compiler::turboshaft::build_with_turboshaft_assembler_impl(
            isolate,
            builtin,
            generator,
            |zone: &mut Zone| {
                compiler::Linkage::get_js_call_descriptor(
                    zone,
                    false,
                    argc,
                    compiler::CallDescriptor::kCanUseRoots,
                )
            },
            name,
            Self::builtin_assembler_options(isolate, builtin),
        );
        *code
    }

    fn compile_js_linkage_code_stub_builtin(
        isolate: &mut Isolate,
        builtin: Builtin,
        generator: &dyn Fn(&mut CodeAssemblerState), // CodeAssemblerGenerator,
        installer: &dyn Fn(&mut Vec<Handle<Code>>, &mut Isolate, Builtin, Handle<Code>), // CodeAssemblerInstaller,
        argc: i32,
        name: &str,
        finalize_order: i32,
        scheduler: &mut BuiltinCompilationScheduler,
    ) {
        let mut job = compiler::Pipeline::new_js_linkage_code_stub_builtin_compilation_job(
            isolate,
            builtin,
            generator,
            Self::builtin_assembler_options(isolate, builtin),
            argc,
            name,
            ProfileDataFromFile::try_read(name),
            finalize_order,
        );
        scheduler.compile_code(isolate, job);
    }

    fn build_with_turboshaft_assembler_cs(
        isolate: &mut Isolate,
        builtin: Builtin,
        generator: &dyn Fn(&mut PipelineData, &mut Isolate, &mut Graph, &mut Zone), // TurboshaftAssemblerGenerator,
        interface_descriptor: CallDescriptors::Key,
        name: &str,
    ) -> Tagged<Code> {
        let mut scope = HandleScope::new(isolate);
        let code = compiler::turboshaft::build_with_turboshaft_assembler_impl(
            isolate,
            builtin,
            generator,
            |zone: &mut Zone| {
                let descriptor = CallInterfaceDescriptor(interface_descriptor);
                assert!(descriptor.get_register_parameter_count() >= 0);
                compiler::Linkage::get_stub_call_descriptor(
                    zone,
                    descriptor,
                    descriptor.get_stack_parameter_count(),
                    compiler::CallDescriptor::kNoFlags,
                    compiler::Operator::kNoProperties,
                )
            },
            name,
            Self::builtin_assembler_options(isolate, builtin),
        );
        *code
    }

    fn compile_cs_linkage_code_stub_builtin(
        isolate: &mut Isolate,
        builtin: Builtin,
        generator: &dyn Fn(&mut CodeAssemblerState), // CodeAssemblerGenerator,
        installer: &dyn Fn(&mut Vec<Handle<Code>>, &mut Isolate, Builtin, Handle<Code>), // CodeAssemblerInstaller,
        interface_descriptor: CallDescriptors::Key,
        name: &str,
        finalize_order: i32,
        scheduler: &mut BuiltinCompilationScheduler,
    ) {
        let mut job = compiler::Pipeline::new_cs_linkage_code_stub_builtin_compilation_job(
            isolate,
            builtin,
            generator,
            Self::builtin_assembler_options(isolate, builtin),
            interface_descriptor,
            name,
            ProfileDataFromFile::try_read(name),
            finalize_order,
        );
        scheduler.compile_code(isolate, job);
    }

    fn compile_bytecode_handler(
        isolate: &mut Isolate,
        builtin: Builtin,
        operand_scale: interpreter::OperandScale,
        bytecode: interpreter::Bytecode,
        installer: &dyn Fn(&mut Vec<Handle<Code>>, &mut Isolate, Builtin, Handle<Code>), // CodeAssemblerInstaller,
        finalize_order: i32,
        scheduler: &mut BuiltinCompilationScheduler,
    ) {
        assert!(interpreter::Bytecodes::bytecode_has_handler(bytecode, operand_scale));
        let name = Builtins::name(builtin);

        let generator = |state: &mut CodeAssemblerState| {
            interpreter::generate_bytecode_handler(state, bytecode, operand_scale);
        };

        let mut job = compiler::Pipeline::new_bytecode_handler_compilation_job(
            isolate,
            builtin,
            &generator,
            Self::builtin_assembler_options(isolate, builtin),
            name,
            ProfileDataFromFile::try_read(name),
            finalize_order,
        );
        scheduler.compile_code(isolate, job);
    }

    fn builtin_assembler_options(isolate: &mut Isolate, builtin: Builtin) -> AssemblerOptions {
        let mut options = AssemblerOptions::default(isolate);
        assert!(!options.isolate_independent_code);
        assert!(!options.collect_win64_unwind_info);

        if wasm::BuiltinLookup::is_wasm_builtin_id(builtin)
            || builtin == Builtin::kJSToWasmWrapper
            || builtin == Builtin::kJSToWasmHandleReturns
            || builtin == Builtin::kWasmToJsWrapperCSA
        {
            options.is_wasm = true;
        }

        if !isolate.is_generating_embedded_builtins() {
            return options;
        }

        let code_region = isolate.heap.code_region();
        let pc_relative_calls_fit_in_code_range = !code_region.is_empty()
            && (code_region.size() as f32 / MB as f32).ceil() <= kMaxPCRelativeCodeRangeInMB;

        options.use_pc_relative_calls_and_jumps_for_mksnapshot = pc_relative_calls_fit_in_code_range;
        options.builtin_call_jump_mode = BuiltinCallJumpMode::kForMksnapshot;
        options.isolate_independent_code = true;
        options.collect_win64_unwind_info = true;

        if builtin == Builtin::kInterpreterEntryTrampolineForProfiling {
            options.builtin_call_jump_mode = BuiltinCallJumpMode::kIndirect;
        }
        options
    }

    fn build_with_macro_assembler(
        isolate: &mut Isolate,
        builtin: Builtin,
        generator: &dyn Fn(&mut MacroAssembler),
        s_name: &str,
    ) -> Tagged<Code> {
        let mut scope = HandleScope::new(isolate);
        let mut buffer: [u8; kBufferSize] = [0; kBufferSize];

        let mut masm = MacroAssembler::new(
            isolate,
            Self::builtin_assembler_options(isolate, builtin),
            CodeObjectRequired::kYes,
            ExternalAssemblerBuffer {
                buffer: &mut buffer,
                size: kBufferSize,
            },
        );

        masm.set_builtin(builtin);
        assert!(!masm.has_frame());
        masm.code_entry();
        generator(&mut masm);

        let mut handler_table_offset = 0;

        assert_eq!(Builtins::kind_of(Builtin::kJSEntry), Builtins::ASM);
        assert_eq!(Builtins::kind_of(Builtin::kJSConstructEntry), Builtins::ASM);
        assert_eq!(
            Builtins::kind_of(Builtin::kJSRunMicrotasksEntry),
            Builtins::ASM
        );
        if Builtins::is_js_entry_variant(builtin) {
            handler_table_offset = HandlerTable::emit_return_table_start(&mut masm);
            HandlerTable::emit_return_entry(
                &mut masm,
                0,
                isolate.builtins.js_entry_handler_offset(),
            );
        }

        let mut desc = CodeDesc::default();
        masm.get_code(
            isolate.main_thread_local_isolate(),
            &mut desc,
            MacroAssembler::kNoSafepointTable,
            handler_table_offset,
        );

        let code = Factory::code_builder(isolate, desc, CodeKind::BUILTIN)
            .set_self_reference(masm.code_object())
            .set_builtin(builtin)
            .build();
        isolate.set_builtin_unwind_data(builtin, masm.get_unwind_info());
        *code
    }
}

// Implementations for types/functions used in the code

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    kFirst,
    kArrayPrototypePush,
    kArrayPrototypePop,
    kArrayPrototypeShift,
    kArrayPrototypeUnshift,
    kArrayPrototypeSplice,
    kArrayPrototypeSlice,
    kArrayPrototypeSort,
    kArrayPrototypeReverse,
    kArrayPrototypeConcat,
    kArrayPrototypeJoin,
    kArrayPrototypeIndexOf,
    kArrayPrototypeLastIndexOf,
    kArrayPrototypeEvery,
    kArrayPrototypeSome,
    kArrayPrototypeForEach,
    kArrayPrototypeMap,
    kArrayPrototypeFilter,
    kArrayPrototypeReduce,
    kArrayPrototypeReduceRight,
    kArrayPrototypeFind,
    kArrayPrototypeFindIndex,
    kArrayPrototypeFill,
    kArrayPrototypeCopyWithin,
    kArrayPrototypeFlat,
    kArrayPrototypeFlatMap,
    kStringPrototypeCharCodeAt,
    kStringPrototypeCodePointAt,
    kStringPrototypeConcat,
    kStringPrototypeSlice,
    kStringPrototypeSubstring,
    kStringPrototypeSubstr,
    kStringPrototypeSplit,
    kStringPrototypeToLowerCase,
    kStringPrototypeToUpperCase,
    kStringPrototypeTrim,
    kStringPrototypeTrimStart,
    kStringPrototypeTrimEnd,
    kStringPrototypeStartsWith,
    kStringPrototypeEndsWith,
    kStringPrototypeIncludes,
    kStringPrototypePadStart,
    kStringPrototypePadEnd,
    kStringPrototypeRepeat,
    kStringPrototypeReplace,
    kStringPrototypeReplaceAll,
    kStringPrototypeSearch,
    kStringPrototypeMatch,
    kStringPrototypeMatchAll,
    kStringPrototypeValueOf,
    kStringPrototypeToString,
    kNumberPrototypeValueOf,
    kNumberPrototypeToString,
    kBooleanPrototypeValueOf,
    kBooleanPrototypeToString,
    kSymbolPrototypeValueOf,
    kSymbolPrototypeToString,
    kBigIntPrototypeValueOf,
    kBigIntPrototypeToString,
    kObjectPrototypeValueOf,
    kObjectPrototypeToString,
    kObjectPrototypeHasOwnProperty,
    kObjectPrototypeIsPrototypeOf,
    kObjectPrototypePropertyIsEnumerable,
    kFunctionPrototypeApply,
    kFunctionPrototypeCall,
    kFunctionPrototypeBind,
    kReflectApply,
    kReflectConstruct,
    kReflectGetPrototypeOf,
    kReflectSetPrototypeOf,
    kReflectGetOwnPropertyDescriptor,
    kReflectDefineProperty,
    kReflectHas,
    kReflectGet,
    kReflectSet,
    kReflectDeleteProperty,
    kReflectOwnKeys,
    kProxyGet,
    kProxySet,
    kProxyHas,
    kProxyDeleteProperty,
    kProxyDefineProperty,
    kProxyGetOwnPropertyDescriptor,
    kProxyOwnKeys,
    kGeneratorPrototypeNext,
    kGeneratorPrototypeReturn,
    kGeneratorPrototypeThrow,
    kAsyncGeneratorPrototypeNext,
    kAsyncGeneratorPrototypeReturn,
    kAsyncGeneratorPrototypeThrow,
    kIteratorPrototypeNext,
    kAsyncIteratorPrototypeNext,
    kMapPrototypeGet,
    kMapPrototypeSet,
    kMapPrototypeHas,
    kMapPrototypeDelete,
    kMapPrototypeClear,
    kMapPrototypeForEach,
    kSetPrototypeAdd,
    kSetPrototypeHas,
    kSetPrototypeDelete,
    kSetPrototypeClear,
    kSetPrototypeForEach,
    kWeakMapPrototypeGet,
    kWeakMapPrototypeSet,
    kWeakMapPrototypeHas,
    kWeakMapPrototypeDelete,
    kWeakSetPrototypeAdd,
    kWeakSetPrototypeHas,
    kWeakSetPrototypeDelete,
    kDataViewPrototypeGetInt8,
    kDataViewPrototypeGetUint8,
    kDataViewPrototypeGetInt16,
    kDataViewPrototypeGetUint16,
    kDataViewPrototypeGetInt32,
    kDataViewPrototypeGetUint32,
    kDataViewPrototypeGetFloat32,
    kDataViewPrototypeGetFloat64,
    kDataViewPrototypeSetInt8,
    kDataViewPrototypeSetUint8,
    kDataViewPrototypeSetInt16,
    kDataViewPrototypeSetUint16,
    kDataViewPrototypeSetInt32,
    kDataViewPrototypeSetUint32,
    kDataViewPrototypeSetFloat32,
    kDataViewPrototypeSetFloat64,
    kArrayBufferIsView,
    kTypedArrayPrototypeSet,
    kTypedArrayPrototypeSlice,
    kTypedArrayPrototypeSubarray,
    kTypedArrayPrototypeCopyWithin,
    kTypedArrayPrototypeFill,
    kBigInt64ArrayPrototypeSet,
    kBigUint64ArrayPrototypeSet,
    kRegExpPrototypeExec,
    kRegExpPrototypeTest,
    kRegExpPrototypeToString,
    kRegExpPrototypeCompile,
    kRegExpPrototypeFlagsGetter,
    kDatePrototypeGetTime,
    kDatePrototypeGetFullYear,
    kDatePrototypeGetMonth,
    kDatePrototypeGetDate,
    kDatePrototypeGetHours,
    kDatePrototypeGetMinutes,
    kDatePrototypeGetSeconds,
    kDatePrototypeGetMilliseconds,
    kDatePrototypeGetUTCFullYear,
    kDatePrototypeGetUTCMonth,
    kDatePrototypeGetUTCDate,
    kDatePrototypeGetUTCHours,
    kDatePrototypeGetUTCMinutes,
    kDatePrototypeGetUTCSeconds,
    kDatePrototypeGetUTCMilliseconds,
    kDatePrototypeGetDay,
    kDatePrototypeGetUTCDay,
    kDatePrototypeSetTime,
    kDatePrototypeSetFullYear,
    kDatePrototypeSetMonth,
    kDatePrototypeSetDate,
    kDatePrototypeSetHours,
    kDatePrototypeSetMinutes,
    kDatePrototypeSetSeconds,
    kDatePrototypeSetMilliseconds,
    kDatePrototypeSetUTCFullYear,
    kDatePrototypeSetUTCMonth,
    kDatePrototypeSetUTCDate,
    kDatePrototypeSetUTCHours,
    kDatePrototypeSetUTCMinutes,
    kDatePrototypeSetUTCSeconds,
    kDatePrototypeSetUTCMilliseconds,
    kDatePrototypeToISOString,
    kDatePrototypeToJson,
    kDatePrototypeValueOf,
    kDatePrototypeToString,
    kDatePrototypeToDateString,
    kDatePrototypeToTimeString,
    kDatePrototypeToUTCString,
    kDatePrototypeToGMTString,
    kErrorPrototypeToString,
    kEvalErrorPrototypeToString,
    kRangeErrorPrototypeToString,
    kReferenceErrorPrototypeToString,
    kSyntaxErrorPrototypeToString,
    kTypeErrorPrototypeToString,
    kURIErrorPrototypeToString,
    kPromisePrototypeThen,
    kPromisePrototypeCatch,
    kPromisePrototypeFinally,
    kPromiseResolve,
    kPromiseReject,
    kPromiseAll,
    kPromiseRace,
    kPromiseAllSettled,
    kPromiseAny,
    kAsyncFunctionAwaitCaught,
    kAsyncFunctionAwaitUncaught,
    kAsyncFunctionReject,
    kAsyncFunctionResolve,
    kAsyncGeneratorAwaitReturn,
    kAsyncGeneratorAwaitYield,
    kAsyncGeneratorReject,
    kAsyncGeneratorResolve,
    kSharedArrayBufferPrototypeGrow,
    kAtomicsAdd,
    kAtomicsSub,
    kAtomicsAnd,
    kAtomicsOr,
    kAtomicsXor,
    kAtomicsExchange,
    kAtomicsCompareExchange,
    kAtomicsLoad,
    kAtomicsStore,
    kAtomicsWait,
    kAtomicsNotify,
    kAtomicsIsLockFree,
    kJSONStringify,
    kJSONParse,
    kMapIteratorPrototypeNext,
    kSetIteratorPrototypeNext,
    kStringIteratorPrototypeNext,
    kArrayIteratorPrototypeNext,
    kRegExpStringIteratorPrototypeNext,
    kGeneratorResume,
    kAsyncGeneratorResume,
    kAsyncGeneratorResolveTrampoline,
    kAsyncGeneratorRejectTrampoline,
    kWasmCompile,
    kWasmInstantiate,
    kWasmValidate,
    kWasmMemoryGrow,
    kWasmMemorySize,
    kWasmTableGet,
    kWasmTableSet,
    kWasmTableSize,
    kWasmTableGrow,
    kWasmTableCopy,
    kWasmTableFill,
    kWasmTableInit,
    kWasmElementDrop,
    kWasmFunctionCall,
    kWasmRunInterpreter,
    kWasmGetOwnProperty,
    kWasmHasProperty,
    kWasmGetProperty,
    kWasmSetProperty,
    kWasmDeleteProperty,
    kWasmOwnKeys,
    kWasmIsWasmCode,
    kWasmIsWasmInstance,
    kWasmIsWasmMemory,
    kWasmIsWasmTable,
    kWasmIsWasmGlobal,
    kWasmIsWasmFunction,
    kWasmIsWasmModule,
    kWasmIsWasmExternal,
    kWasmIsWasmRef,
    kWasmGetGlobal,
    kWasmSetGlobal,
    kWasmGlobalNew,
    kWasmNew,
    kWasmToJsWrapper,
    kJSToWasmWrapper,
    kJSToWasmHandleReturns,
    kWasmToJsWrapperCSA,
    kWasmInterpreterCWasmEntry,
    kWasmReturnPromiseOnSuspendAsm,
    kDebugPrint,
    kDebugBreak,
    kDebugGetLoadedScripts,
    kDebugSetBreakpoint,
    kDebugClearBreakpoint,
    kDebugChangeBreakpoint,
    kDebugListBreakpoints,
    kDebugEvaluate,
    kDebugGetFrameCount,
    kDebugGetFrameDetails,
    kDebugGetScopeCount,
    kDebugGetScopeDetails,
    kDebugSetVariableValue,
    kDebugGetPossibleBreakpoints,
    kDebugNext,
    kDebugStepIn,
    kDebugStepOut,
    kDebugStepOver,
    kDebugPrepareStepInIfStepping,
    kDebugAsyncFunctionResumed,
    k

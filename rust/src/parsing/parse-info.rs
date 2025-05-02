// src/parsing/parse_info.rs

pub mod ast;
pub mod base;
pub mod common;
pub mod compiler_dispatcher;
pub mod heap;
pub mod logging;
pub mod numbers;
pub mod objects;
pub mod zone;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{
    ast_source_ranges::SourceRangeMap, ast_value_factory::AstValueFactory, AstNode, FunctionLiteral,
};
use crate::base::logging::DCHECK_EQ;
use crate::base::logging::DCHECK_IMPLIES;
use crate::common::globals::ScriptEventType;
use crate::objects::{
    script::Script, shared_function_info::SharedFunctionInfo, scope_info::ScopeInfo,
};
use crate::zone::zone::Zone;

// Placeholder for Isolate and LocalIsolate.  Need more context to fully implement.
pub struct Isolate {}

impl Isolate {
    pub fn is_best_effort_code_coverage(&self) -> bool {
        false
    }
    pub fn is_block_code_coverage(&self) -> bool {
        false
    }
    pub fn GetNextScriptId(&self) -> i32 {
        0
    }
    pub fn NeedsDetailedOptimizedCodeLineInfo(&self) -> bool {
        false
    }
    pub fn lazy_compile_dispatcher(&self) -> &LazyCompileDispatcher {
        &LazyCompileDispatcher {}
    }
    pub fn ast_string_constants(&self) -> &AstStringConstants {
        &AstStringConstants {}
    }
    pub fn factory(&self) -> Factory {
        Factory {}
    }
    pub fn stack_guard(&self) -> StackGuard {
        StackGuard {}
    }
    pub fn counters(&self) -> Counters {
        Counters {}
    }
}

// Placeholder structs/enums/functions that need further implementation or dependency injection
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionKind {
    kNormalFunction,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionSyntaxKind {
    kDeclaration,
    kWrapped,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParsingWhileDebugging {
    kNo,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LanguageMode {
    Normal,
    Strict,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum REPLMode {
    kNo,
    kYes,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScriptType {
    kClassic,
    kModule,
}

pub struct LazyCompileDispatcher {}

impl LazyCompileDispatcher {
    // Add necessary methods and fields if required.
}

pub struct AstStringConstants {}

impl AstStringConstants {
    // Add necessary methods and fields if required.
}

pub struct ScriptOriginOptions {
    is_module: bool,
}

impl ScriptOriginOptions {
    pub fn IsModule(&self) -> bool {
        self.is_module
    }
}

pub struct StackGuard {}

impl StackGuard {
    pub fn real_climit(&self) -> usize {
        0
    }
}

pub struct Counters {}

impl Counters {
    pub fn runtime_call_stats(&self) -> &RuntimeCallStats {
        &RuntimeCallStats {}
    }
}

pub struct RuntimeCallStats {}

// Flags holder (v8_flags)
struct V8Flags {
    always_turbofan: bool,
    prepare_always_turbofan: bool,
    allow_natives_syntax: bool,
    enable_lazy_source_positions: bool,
    parallel_compile_tasks_for_eager_toplevel: bool,
    parallel_compile_tasks_for_lazy: bool,
    lazy: bool,
}

static mut V8_FLAGS: V8Flags = V8Flags {
    always_turbofan: false,
    prepare_always_turbofan: false,
    allow_natives_syntax: false,
    enable_lazy_source_positions: false,
    parallel_compile_tasks_for_eager_toplevel: false,
    parallel_compile_tasks_for_lazy: false,
    lazy: true,
};

// Utility function to access v8_flags.  Marked unsafe due to global mutable state.
unsafe fn v8_flags() -> &'static V8Flags {
    &V8_FLAGS
}

#[derive(Clone, Copy, Debug)]
pub enum NativesFlag {
    EXTENSION_CODE,
    INSPECTOR_CODE,
    NOT_NATIVES_CODE,
}

pub struct Factory {}

impl Factory {
    pub fn NewScriptWithId(
        &self,
        source: DirectHandle<String>,
        script_id: i32,
        event: ScriptEventType,
    ) -> Handle<Script> {
        Handle::new(Script {
            id: script_id,
            compilation_type: Script::CompilationType::kUnknown,
            origin_options: ScriptOriginOptions { is_module: false },
            is_repl_mode: false,
            wrapped_arguments: FixedArray {},
            r#type: Script::Type::kNormal,
        })
    }
}

pub struct FixedArray {}

pub struct String {}

pub struct Handle<T> {
    value: T,
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { value }
    }
}

pub struct DirectHandle<T> {
    value: T,
}

impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle { value }
    }
}

impl<T> std::ops::Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct NoGarbageCollection {}

#[macro_export]
macro_rules! DisallowGarbageCollection {
    () => {
        let _no_gc = NoGarbageCollection {};
    };
}

/// Compile Flags
#[derive(Debug, Clone, Copy)]
pub struct UnoptimizedCompileFlags {
    flags_: u32,
    script_id_: i32,
    function_kind_: FunctionKind,
    function_syntax_kind_: FunctionSyntaxKind,
    parsing_while_debugging_: ParsingWhileDebugging,
}

impl UnoptimizedCompileFlags {
    pub fn new(isolate: &Isolate, script_id: i32) -> Self {
        let mut flags = Self {
            flags_: 0,
            script_id_: script_id,
            function_kind_: FunctionKind::kNormalFunction,
            function_syntax_kind_: FunctionSyntaxKind::kDeclaration,
            parsing_while_debugging_: ParsingWhileDebugging::kNo,
        };

        flags.set_coverage_enabled(!isolate.is_best_effort_code_coverage());
        flags.set_block_coverage_enabled(isolate.is_block_code_coverage());

        unsafe {
            flags.set_might_always_turbofan(
                v8_flags().always_turbofan || v8_flags().prepare_always_turbofan,
            );
            flags.set_allow_natives_syntax(v8_flags().allow_natives_syntax);
            flags.set_allow_lazy_compile(true);
            flags.set_collect_source_positions(
                !v8_flags().enable_lazy_source_positions
                    || isolate.NeedsDetailedOptimizedCodeLineInfo(),
            );
            flags.set_post_parallel_compile_tasks_for_eager_toplevel(
                v8_flags().parallel_compile_tasks_for_eager_toplevel,
            );
            flags.set_post_parallel_compile_tasks_for_lazy(
                v8_flags().parallel_compile_tasks_for_lazy,
            );
        }
        flags
    }

    pub fn for_function_compile(
        isolate: &Isolate,
        shared: &SharedFunctionInfo,
    ) -> UnoptimizedCompileFlags {
        let script = &shared.script();

        let mut flags = UnoptimizedCompileFlags::new(isolate, script.id);

        flags.SetFlagsForFunctionFromScript(script);
        flags.SetFlagsFromFunction(shared);
        flags.set_allow_lazy_parsing(true);
        flags.set_is_lazy_compile(true);

        // #[cfg(V8_ENABLE_WEBASSEMBLY)]
        flags.set_is_asm_wasm_broken(shared.is_asm_wasm_broken());

        flags.set_is_repl_mode(script.is_repl_mode());

        // Do not support re-parsing top-level function of a wrapped script.
        DCHECK_IMPLIES(flags.is_toplevel(), !script.is_wrapped());

        flags
    }

    pub fn for_script_compile(isolate: &Isolate, script: &Script) -> UnoptimizedCompileFlags {
        let mut flags = UnoptimizedCompileFlags::new(isolate, script.id);

        flags.SetFlagsForFunctionFromScript(script);
        flags.SetFlagsForToplevelCompile(
            script.IsUserJavaScript(),
            flags.outer_language_mode(),
            construct_repl_mode(script.is_repl_mode()),
            if script.origin_options().IsModule() {
                ScriptType::kModule
            } else {
                ScriptType::kClassic
            },
            unsafe { v8_flags().lazy },
        );
        if script.is_wrapped() {
            flags.set_function_syntax_kind(FunctionSyntaxKind::kWrapped);
        }

        flags
    }

    pub fn for_toplevel_compile(
        isolate: &Isolate,
        is_user_javascript: bool,
        language_mode: LanguageMode,
        repl_mode: REPLMode,
        type_: ScriptType,
        lazy: bool,
    ) -> UnoptimizedCompileFlags {
        let mut flags = UnoptimizedCompileFlags::new(isolate, isolate.GetNextScriptId());
        flags.SetFlagsForToplevelCompile(is_user_javascript, language_mode, repl_mode, type_, lazy);
        // LOG(isolate, ScriptEvent(ScriptEventType::kReserveId, flags.script_id())); //No direct translation
        flags
    }

    pub fn for_toplevel_function(
        toplevel_flags: UnoptimizedCompileFlags,
        literal: &FunctionLiteral,
    ) -> UnoptimizedCompileFlags {
        DCHECK_EQ(toplevel_flags.is_toplevel(), true);
        //DCHECK(!literal->is_toplevel());

        // Replicate the toplevel flags, then setup the function-specific flags.
        let mut flags = toplevel_flags;
        flags.SetFlagsFromFunction(literal);

        flags
    }

    pub fn for_test(isolate: &Isolate) -> UnoptimizedCompileFlags {
        UnoptimizedCompileFlags::new(isolate, Script::kTemporaryScriptId)
    }

    fn SetFlagsFromFunction<T>(&mut self, function: &T)
    where
        T: FunctionLike,
    {
        self.set_outer_language_mode(function.language_mode());
        self.set_function_kind(function.kind());
        self.set_function_syntax_kind(function.syntax_kind());
        self.set_requires_instance_members_initializer(
            function.requires_instance_members_initializer(),
        );
        self.set_class_scope_has_private_brand(function.class_scope_has_private_brand());
        self.set_has_static_private_methods_or_accessors(
            function.has_static_private_methods_or_accessors(),
        );
        self.set_private_name_lookup_skips_outer_class(
            function.private_name_lookup_skips_outer_class(),
        );
        self.set_is_toplevel(function.is_toplevel());
    }

    fn SetFlagsForToplevelCompile(
        &mut self,
        is_user_javascript: bool,
        language_mode: LanguageMode,
        repl_mode: REPLMode,
        type_: ScriptType,
        lazy: bool,
    ) {
        self.set_is_toplevel(true);
        self.set_allow_lazy_parsing(lazy);
        self.set_allow_lazy_compile(lazy);
        self.set_outer_language_mode(stricter_language_mode(
            self.outer_language_mode(),
            language_mode,
        ));
        self.set_is_repl_mode((repl_mode == REPLMode::kYes));
        self.set_is_module(type_ == ScriptType::kModule);
        DCHECK_IMPLIES(self.is_eval(), !self.is_module());

        self.set_block_coverage_enabled(self.block_coverage_enabled() && is_user_javascript);
    }

    fn SetFlagsForFunctionFromScript(&mut self, script: &Script) {
        DCHECK_EQ(self.script_id(), script.id());

        self.set_is_eval(script.compilation_type() == Script::CompilationType::kEval);
        self.set_is_module(script.origin_options().IsModule());
        DCHECK_IMPLIES(self.is_eval(), !self.is_module());

        self.set_block_coverage_enabled(self.block_coverage_enabled() && script.IsUserJavaScript());
    }

    // Setters
    fn set_coverage_enabled(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 0;
    }

    fn set_block_coverage_enabled(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 1;
    }

    fn set_might_always_turbofan(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 2;
    }

    fn set_allow_natives_syntax(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 3;
    }

    fn set_allow_lazy_compile(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 4;
    }

    fn set_collect_source_positions(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 5;
    }

    fn set_post_parallel_compile_tasks_for_eager_toplevel(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 6;
    }

    fn set_post_parallel_compile_tasks_for_lazy(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 7;
    }

    fn set_outer_language_mode(&mut self, mode: LanguageMode) {
        // Assuming LanguageMode can be represented as u32
        self.flags_ |= (mode as u32) << 8;
    }

    fn set_function_kind(&mut self, kind: FunctionKind) {
        self.flags_ |= (kind as u32) << 9;
    }

    fn set_function_syntax_kind(&mut self, kind: FunctionSyntaxKind) {
        self.flags_ |= (kind as u32) << 10;
    }

    fn set_requires_instance_members_initializer(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 11;
    }

    fn set_class_scope_has_private_brand(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 12;
    }

    fn set_has_static_private_methods_or_accessors(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 13;
    }

    fn set_private_name_lookup_skips_outer_class(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 14;
    }

    fn set_is_toplevel(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 15;
    }

    fn set_allow_lazy_parsing(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 16;
    }

    fn set_is_lazy_compile(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 17;
    }

    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    fn set_is_asm_wasm_broken(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 18;
    }

    fn set_is_repl_mode(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 19;
    }

    fn set_is_module(&mut self, value: bool) {
        self.flags_ |= (value as u32) << 20;
    }

    // Getters
    fn script_id(&self) -> i32 {
        self.script_id_
    }

    fn outer_language_mode(&self) -> LanguageMode {
        unsafe { std::mem::transmute((self.flags_ >> 8) as u8) }
    }

    fn function_syntax_kind(&self) -> FunctionSyntaxKind {
        unsafe { std::mem::transmute((self.flags_ >> 10) as u8) }
    }

    fn is_toplevel(&self) -> bool {
        (self.flags_ >> 15 & 1) != 0
    }

    fn is_eval(&self) -> bool {
        (self.flags_ >> 21 & 1) != 0 // Assuming you've added an eval flag
    }

    fn block_coverage_enabled(&self) -> bool {
        (self.flags_ >> 1 & 1) != 0
    }
}

// Trait to represent FunctionLiteral and SharedFunctionInfo since they share some methods
trait FunctionLike {
    fn language_mode(&self) -> LanguageMode;
    fn kind(&self) -> FunctionKind;
    fn syntax_kind(&self) -> FunctionSyntaxKind;
    fn requires_instance_members_initializer(&self) -> bool;
    fn class_scope_has_private_brand(&self) -> bool;
    fn has_static_private_methods_or_accessors(&self) -> bool;
    fn private_name_lookup_skips_outer_class(&self) -> bool;
    fn is_toplevel(&self) -> bool;
}

impl FunctionLike for FunctionLiteral {
    fn language_mode(&self) -> LanguageMode {
        LanguageMode::Normal // Placeholder
    }
    fn kind(&self) -> FunctionKind {
        FunctionKind::kNormalFunction // Placeholder
    }
    fn syntax_kind(&self) -> FunctionSyntaxKind {
        FunctionSyntaxKind::kDeclaration // Placeholder
    }
    fn requires_instance_members_initializer(&self) -> bool {
        false // Placeholder
    }
    fn class_scope_has_private_brand(&self) -> bool {
        false // Placeholder
    }
    fn has_static_private_methods_or_accessors(&self) -> bool {
        false // Placeholder
    }
    fn private_name_lookup_skips_outer_class(&self) -> bool {
        false // Placeholder
    }
    fn is_toplevel(&self) -> bool {
        false // Placeholder
    }
}

impl FunctionLike for SharedFunctionInfo {
    fn language_mode(&self) -> LanguageMode {
        LanguageMode::Normal // Placeholder
    }
    fn kind(&self) -> FunctionKind {
        FunctionKind::kNormalFunction // Placeholder
    }
    fn syntax_kind(&self) -> FunctionSyntaxKind {
        FunctionSyntaxKind::kDeclaration // Placeholder
    }
    fn requires_instance_members_initializer(&self) -> bool {
        false // Placeholder
    }
    fn class_scope_has_private_brand(&self) -> bool {
        false // Placeholder
    }
    fn has_static_private_methods_or_accessors(&self) -> bool {
        false // Placeholder
    }
    fn private_name_lookup_skips_outer_class(&self) -> bool {
        false // Placeholder
    }
    fn is_toplevel(&self) -> bool {
        false // Placeholder
    }
}

fn stricter_language_mode(mode1: LanguageMode, mode2: LanguageMode) -> LanguageMode {
    if mode2 == LanguageMode::Strict {
        mode2
    } else {
        mode1
    }
}

fn construct_repl_mode(is_repl_mode: bool) -> REPLMode {
    if is_repl_mode {
        REPLMode::kYes
    } else {
        REPLMode::kNo
    }
}

// Hash Seed struct
pub struct HashSeed(u32);

impl HashSeed {
    pub fn new(isolate: &Isolate) -> HashSeed {
        HashSeed(0) // Replace 0 with appropriate hash seed generation logic.
    }
}

//Allocator placeholder.
pub struct Allocator {}

impl Allocator {
    //Add necessary methods and fields if required.
}

//V8FileLogger placeholder.
pub struct V8FileLogger {}

impl V8FileLogger {
    //Add necessary methods and fields if required.
}

pub struct LocalIsolate {}

impl LocalIsolate {
    pub fn allocator(&self) -> &Allocator {
        &Allocator {}
    }
    pub fn main_thread_logger(&self) -> &V8FileLogger {
        &V8FileLogger {}
    }
    pub fn lazy_compile_dispatcher(&self) -> &LazyCompileDispatcher {
        &LazyCompileDispatcher {}
    }
    pub fn ast_string_constants(&self) -> &AstStringConstants {
        &AstStringConstants {}
    }
    pub fn runtime_call_stats(&self) -> &RuntimeCallStats {
        &RuntimeCallStats {}
    }
}

// Reusable Unoptimized Compile State
pub struct ReusableUnoptimizedCompileState {
    hash_seed_: HashSeed,
    allocator_: Allocator,
    v8_file_logger_: V8FileLogger,
    dispatcher_: LazyCompileDispatcher,
    ast_string_constants_: AstStringConstants,
    ast_raw_string_zone_: Zone,
    single_parse_zone_: Zone,
    ast_value_factory_: Box<AstValueFactory>,
}

impl ReusableUnoptimizedCompileState {
    pub fn new(isolate: &Isolate) -> Self {
        let allocator = Allocator {};
        let ast_raw_string_zone = Zone::new(&allocator, "unoptimized-compile-ast-raw-string-zone");
        let single_parse_zone = Zone::new(&allocator, "unoptimized-compile-parse-zone");
        ReusableUnoptimizedCompileState {
            hash_seed_: HashSeed::new(isolate),
            allocator_: allocator,
            v8_file_logger_: V8FileLogger {},
            dispatcher_: LazyCompileDispatcher {},
            ast_string_constants_: AstStringConstants {},
            ast_raw_string_zone_: ast_raw_string_zone,
            single_parse_zone_: single_parse_zone,
            ast_value_factory_: Box::new(AstValueFactory::new(
                Zone::new(&Allocator {}, "AstRawStringZone"),
                Zone::new(&Allocator {}, "SingleParseZone"),
                AstStringConstants {},
                HashSeed::new(isolate),
            )),
        }
    }
    pub fn new_local(isolate: &LocalIsolate) -> Self {
        let allocator = Allocator {};
        let ast_raw_string_zone = Zone::new(&allocator, "unoptimized-compile-ast-raw-string-zone");
        let single_parse_zone = Zone::new(&allocator, "unoptimized-compile-parse-zone");
        ReusableUnoptimizedCompileState {
            hash_seed_: HashSeed::new(&Isolate {}), //Use placeholder as it needs an isolate
            allocator_: allocator,
            v8_file_logger_: V8FileLogger {},
            dispatcher_: LazyCompileDispatcher {},
            ast_string_constants_: AstStringConstants {},
            ast_raw_string_zone_: ast_raw_string_zone,
            single_parse_zone_: single_parse_zone,
            ast_value_factory_: Box::new(AstValueFactory::new(
                Zone::new(&Allocator {}, "AstRawStringZone"),
                Zone::new(&Allocator {}, "SingleParseZone"),
                AstStringConstants {},
                HashSeed::new(&Isolate {}), //Use placeholder as it needs an isolate
            )),
        }
    }

    fn NotifySingleParseCompleted(&self) {}
}

//Unoptimized Compile State placeholder.
pub struct UnoptimizedCompileState {}

// Constants
const kNoSourcePosition: i32 = -1;
const kInvalidInfoId: i32 = -1;
pub struct Utf16CharacterStream {}

impl Utf16CharacterStream {
    // Add necessary methods and fields if required.
}

// Parse Info
pub struct ParseInfo<'a> {
    flags_: UnoptimizedCompileFlags,
    state_: &'a UnoptimizedCompileState,
    reusable_state_: &'a ReusableUnoptimizedCompileState,
    extension_: *mut u8, // Placeholder
    script_scope_: *mut u8, // Placeholder
    stack_limit_: usize,
    parameters_end_pos_: i32,
    max_info_id_: i32,
    character_stream_: Option<std::boxed::Box<Utf16CharacterStream>>,
    function_name_: *mut u8, // Placeholder
    runtime_call_stats_: &'a RuntimeCallStats,
    source_range_map_: *mut SourceRangeMap, // Needs to be Zone-allocated
    literal_: *mut FunctionLiteral,         // Placeholder
    allow_eval_cache_: bool,
    contains_asm_module_: bool, // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    language_mode_: LanguageMode,
    is_background_compilation_: bool,
    is_streaming_compilation_: bool,
    has_module_in_scope_chain_: bool,
}

impl<'a> ParseInfo<'a> {
    pub fn new(
        flags: UnoptimizedCompileFlags,
        state: &'a UnoptimizedCompileState,
        reusable_state: &'a ReusableUnoptimizedCompileState,
        stack_limit: usize,
        runtime_call_stats: &'a RuntimeCallStats,
    ) -> Self {
        ParseInfo {
            flags_: flags,
            state_: state,
            reusable_state_: reusable_state,
            extension_: std::ptr::null_mut(),
            script_scope_: std::ptr::null_mut(),
            stack_limit_: stack_limit,
            parameters_end_pos_: kNoSourcePosition,
            max_info_id_: kInvalidInfoId,
            character_stream_: None,
            function_name_: std::ptr::null_mut(),
            runtime_call_stats_: runtime_call_stats,
            source_range_map_: std::ptr::null_mut(),
            literal_: std::ptr::null_mut(),
            allow_eval_cache_: false,
            contains_asm_module_: false, // #[cfg(V8_ENABLE_WEBASSEMBLY)]
            language_mode_: flags.outer_language_mode(),
            is_background_compilation_: false,
            is_streaming_compilation_: false,
            has_module_in_scope_chain_: flags.is_module(),
        }
    }

    pub fn new_isolate(
        isolate: &Isolate,
        flags: UnoptimizedCompileFlags,
        state: &'a UnoptimizedCompileState,
        reusable_state: &'a ReusableUnoptimizedCompileState,
    ) -> Self {
        ParseInfo::new(
            flags,
            state,
            reusable_state,
            isolate.stack_guard().real_climit(),
            isolate.counters().runtime_call_stats(),
        )
    }

    pub fn new_local_isolate(
        isolate: &LocalIsolate,
        flags: UnoptimizedCompileFlags,
        state: &'a UnoptimizedCompileState,
        reusable_state: &'a ReusableUnoptimizedCompileState,
        stack_limit: usize,
    ) -> Self {
        ParseInfo::new(
            flags,
            state,
            reusable_state,
            stack_limit,
            isolate.runtime_call_stats(),
        )
    }
    pub fn is_wrapped_as_function(&self) -> bool {
        false
    }
    pub fn flags(&self) -> UnoptimizedCompileFlags {
        self.flags_
    }
    pub fn language_mode(&self) -> LanguageMode {
        self.language_mode_
    }

    pub fn is_streaming_compilation(&self) -> bool {
        self.is_streaming_compilation_
    }

    pub fn is_background_compilation(&self) -> bool {
        self.is_background_compilation_
    }

    pub fn CreateScript<T>(
        &mut self,
        isolate: &T,
        source: DirectHandle<String>,
        maybe_wrapped_arguments: Option<DirectHandle<FixedArray>>,
        origin_options: ScriptOriginOptions,
        natives: NativesFlag,
    ) -> Handle<Script>
    where
        T: CreateScriptTrait,
    {
        // Create a script object describing the script to be compiled.
        DCHECK_EQ(
            self.flags().script_id() >= 0 || self.flags().script_id() == Script::kTemporaryScriptId,
            true,
        );
        let mut event = ScriptEventType::kCreate;
        if self.is_streaming_compilation() {
            event = if self.is_background_compilation() {
                ScriptEventType::kStreamingCompileBackground
            } else {
                ScriptEventType::kStreamingCompileForeground
            };
        } else if self.is_background_compilation() {
            event = ScriptEventType::kBackgroundCompile;
        }

        let script = isolate.create_script_with_id(source, self.flags().script_id(), event);
        // let script =
        //     isolate.factory().NewScriptWithId(source, self.flags().script_id(), event);
        DisallowGarbageCollection!();
        //Tagged<Script> raw_script = *script;
        match natives {
            NativesFlag::EXTENSION_CODE => {
                script.value.set_type(Script::Type::kExtension);
            }
            NativesFlag::INSPECTOR_CODE => {
                script.value.set_type(Script::Type::kInspector);
            }
            NativesFlag::NOT_NATIVES_CODE => {}
        }
        script.value.set_origin_options(origin_options);
        script.value.set_is_repl_mode(self.flags().is_repl_mode());

        DCHECK_EQ(
            self.is_wrapped_as_function(),
            maybe_wrapped_arguments.is_some(),
        );
        if self.is_wrapped_as_function() {
            //raw_script.set_wrapped_arguments(*maybe_wrapped_arguments.ToHandleChecked());
            script.value.set_wrapped_arguments(maybe_wrapped_arguments.unwrap().value);
        } else if self.flags().is_eval() {
            script.value.set_compilation_type(Script::CompilationType::kEval);
        }
        self.CheckFlagsForToplevelCompileFromScript(&script.value);

        script
    }
    fn CheckFlagsForToplevelCompileFromScript(&self, script: &Script) {
        self.CheckFlagsForFunctionFromScript(script);
        DCHECK_EQ(self.flags().is_toplevel(), true);
        DCHECK_EQ(self.flags().is_repl_mode(), script.is_repl_mode());

        if script.is_wrapped() {
            DCHECK_EQ(
                self.flags().function_syntax_kind(),
                FunctionSyntaxKind::kWrapped,
            );
        }
    }
    fn CheckFlagsForFunctionFromScript(&self, script: &Script) {
        DCHECK_EQ(self.flags().script_id(), script.id());
        // We set "is_eval" for wrapped scripts to get an outer declaration scope.
        // This is a bit hacky, but ok since we can't be both eval and wrapped.
        DCHECK_EQ(
            self.flags().is_eval() && !script.is_wrapped(),
            script.compilation_type() == Script::CompilationType::kEval,
        );
        DCHECK_EQ(self.flags().is_module(), script.origin_options().IsModule());
        DCHECK_IMPLIES(
            self.flags().block_coverage_enabled() && script.IsUserJavaScript(),
            !self.source_range_map_.is_null(),
        );
    }

    fn AllocateSourceRangeMap(&mut self) {
        DCHECK_EQ(self.flags().block_coverage_enabled(), true);
        DCHECK_EQ(self.source_range_map_.is_null(), true);

        // Create a SourceRangeMap within the zone
        let source_range_map = self.zone().New(SourceRangeMap::new(self.zone()));
        self.set_source_range_map(source_range_map);
    }

    fn ResetCharacterStream(&mut self) {
        self.character_stream_.take();
    }

    fn set_character_stream(&mut self, character_stream: std::boxed::Box<Utf16CharacterStream>) {
        DCHECK_EQ(self.character_stream_.is_none(), true);
        self.character_stream_ = Some(character_stream);
    }

    fn set_source_range_map(&mut self, source_range_map: *mut SourceRangeMap) {
        self.source_range_map_ = source_range_map;
    }

    fn source_range_map(&self) -> *mut SourceRangeMap {
        self.source_range_map_
    }

    fn zone(&self
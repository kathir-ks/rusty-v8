// Converted from V8 C++ source files:
// Header: parse-info.h
// Implementation: parse-info.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub struct Extension;
}

pub mod internal {
    pub struct AccountingAllocator;
    pub struct AstRawString;
    pub struct AstStringConstants;
    pub struct AstValueFactory;
    pub struct LazyCompileDispatcher;
    pub struct DeclarationScope;
    pub struct FunctionLiteral;
    pub struct RuntimeCallStats;
    pub struct V8FileLogger;
    pub struct SourceRangeMap;
    pub struct Utf16CharacterStream;
    pub struct Zone;

    use std::rc::Rc;

    pub enum LanguageMode {
        Normal,
        Strict,
    }

    pub enum ParseRestriction {
        NoRestriction,
        MaybeEval,
        OnlyEval,
    }

    #[derive(Clone, Copy)]
    pub enum FunctionKind {
        kNormalFunction,
        kGetter,
        kSetter,
        kConstructor,
        kAsyncFunction,
        kAsyncGeneratorFunction,
        kGeneratorFunction,
    }

    #[derive(Clone, Copy)]
    pub enum FunctionSyntaxKind {
        kDeclaration,
        kExpression,
        kWrapped,
    }

    pub enum REPLMode {
        kNo,
        kYes,
    }

    pub enum ScriptType {
        kClassic,
        kModule,
    }

    pub enum ParsingWhileDebugging {
        kNo,
        kYes,
    }

    macro_rules! define_bit_fields {
        ($($name:ident, $type:ty, $size:literal, _);*) => {
            pub struct BitFields {
                $(
                    pub $name: u32,
                )*
            }
        }
    }

    macro_rules! flag_get_set {
        ($flags:ident, $name:ident, $type:ty) => {
            pub fn $name(&self) -> $type {
                self.$flags.$name as $type
            }
            pub fn set_$name(&mut self, value: $type) -> &mut Self {
                self.$flags.$name = value as u32;
                self
            }
        };
    }

    pub struct UnoptimizedCompileFlags {
        flags_: BitFieldsStruct,
        script_id_: i32,
        function_kind_: FunctionKind,
        function_syntax_kind_: FunctionSyntaxKind,
        parsing_while_debugging_: ParsingWhileDebugging,
    }

    impl UnoptimizedCompileFlags {
        pub fn ForToplevelCompile(
            isolate: &mut Isolate,
            is_user_javascript: bool,
            language_mode: LanguageMode,
            repl_mode: REPLMode,
            type_: ScriptType,
            lazy: bool,
        ) -> Self {
            let mut flags = UnoptimizedCompileFlags::new(isolate, isolate.get_next_script_id());
            flags.SetFlagsForToplevelCompile(is_user_javascript, language_mode, repl_mode, type_, lazy);
            flags
        }

        pub fn ForFunctionCompile(
            isolate: &mut Isolate,
            shared: Tagged<SharedFunctionInfo>,
        ) -> Self {
            let script = shared.script();
            let mut flags = UnoptimizedCompileFlags::new(isolate, script.id());

            flags.SetFlagsForFunctionFromScript(script);
            flags.SetFlagsFromFunction(shared);
            flags.set_allow_lazy_parsing(true);
            flags.set_is_lazy_compile(true);
            flags.set_is_repl_mode(script.is_repl_mode());
            flags
        }

        pub fn ForScriptCompile(
            isolate: &mut Isolate,
            script: Tagged<Script>,
        ) -> Self {
            let mut flags = UnoptimizedCompileFlags::new(isolate, script.id());

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
                false,
            );
            if script.is_wrapped() {
                flags.set_function_syntax_kind(FunctionSyntaxKind::kWrapped);
            }

            flags
        }
        pub fn ForToplevelFunction(
            toplevel_flags: UnoptimizedCompileFlags,
            literal: &FunctionLiteral,
        ) -> Self {
            assert!(toplevel_flags.is_toplevel());

            let mut flags = toplevel_flags;
            flags.SetFlagsFromFunction(literal);

            flags
        }
        pub fn ForTest(isolate: &mut Isolate) -> Self {
            UnoptimizedCompileFlags::new(isolate, Script::kTemporaryScriptId)
        }
        fn new(isolate: &mut Isolate, script_id: i32) -> Self {
            let mut flags = UnoptimizedCompileFlags {
                flags_: BitFieldsStruct {
                    is_toplevel: 0,
                    is_eager: 0,
                    is_eval: 0,
                    is_reparse: 0,
                    outer_language_mode: 0,
                    parse_restriction: 0,
                    is_module: 0,
                    allow_lazy_parsing: 0,
                    is_lazy_compile: 0,
                    coverage_enabled: 0,
                    block_coverage_enabled: 0,
                    is_asm_wasm_broken: 0,
                    class_scope_has_private_brand: 0,
                    private_name_lookup_skips_outer_class: 0,
                    requires_instance_members_initializer: 0,
                    has_static_private_methods_or_accessors: 0,
                    might_always_turbofan: 0,
                    allow_natives_syntax: 0,
                    allow_lazy_compile: 0,
                    post_parallel_compile_tasks_for_eager_toplevel: 0,
                    post_parallel_compile_tasks_for_lazy: 0,
                    collect_source_positions: 0,
                    is_repl_mode: 0,
                    produce_compile_hints: 0,
                    compile_hints_magic_enabled: 0,
                    compile_hints_per_function_magic_enabled: 0,
                },
                script_id_: script_id,
                function_kind_: FunctionKind::kNormalFunction,
                function_syntax_kind_: FunctionSyntaxKind::kDeclaration,
                parsing_while_debugging_: ParsingWhileDebugging::kNo,
            };

            flags.set_coverage_enabled(!isolate.is_best_effort_code_coverage());
            flags.set_block_coverage_enabled(isolate.is_block_code_coverage());
            flags.set_might_always_turbofan(true);
            flags.set_allow_natives_syntax(true);
            flags.set_allow_lazy_compile(true);
            flags.set_collect_source_positions(true);
            flags.set_post_parallel_compile_tasks_for_eager_toplevel(false);
            flags.set_post_parallel_compile_tasks_for_lazy(false);

            flags
        }

        fn SetFlagsForFunctionFromScript(&mut self, script: Tagged<Script>) {
            assert_eq!(self.script_id(), script.id());

            self.set_is_eval(script.compilation_type() == Script::CompilationType::kEval);
            self.set_is_module(script.origin_options().IsModule());
            assert!(!(self.is_eval() && self.is_module()));

            self.set_block_coverage_enabled(self.block_coverage_enabled() && script.IsUserJavaScript());
        }

        fn SetFlagsForToplevelCompile(&mut self, is_user_javascript: bool, language_mode: LanguageMode, repl_mode: REPLMode, type_: ScriptType, lazy: bool) {
            self.set_is_toplevel(true);
            self.set_allow_lazy_parsing(lazy);
            self.set_allow_lazy_compile(lazy);
            self.set_outer_language_mode(language_mode);
            self.set_is_repl_mode(repl_mode == REPLMode::kYes);
            self.set_is_module(type_ == ScriptType::kModule);
            assert!(!(self.is_eval() && self.is_module()));

            self.set_block_coverage_enabled(self.block_coverage_enabled() && is_user_javascript);
        }

        fn SetFlagsFromFunction<T>(&mut self, function: T)
            where T: Functionlike
        {
            self.set_outer_language_mode(function.language_mode());
            self.set_function_kind(function.kind());
            self.set_function_syntax_kind(function.syntax_kind());
            self.set_requires_instance_members_initializer(function.requires_instance_members_initializer());
            self.set_class_scope_has_private_brand(function.class_scope_has_private_brand());
            self.set_has_static_private_methods_or_accessors(function.has_static_private_methods_or_accessors());
            self.set_private_name_lookup_skips_outer_class(function.private_name_lookup_skips_outer_class());
            self.set_is_toplevel(function.is_toplevel());
        }

        flag_get_set!(flags_, is_toplevel, bool);
        flag_get_set!(flags_, is_eager, bool);
        flag_get_set!(flags_, is_eval, bool);
        flag_get_set!(flags_, is_reparse, bool);
        flag_get_set!(flags_, outer_language_mode, LanguageMode);
        flag_get_set!(flags_, parse_restriction, ParseRestriction);
        flag_get_set!(flags_, is_module, bool);
        flag_get_set!(flags_, allow_lazy_parsing, bool);
        flag_get_set!(flags_, is_lazy_compile, bool);
        flag_get_set!(flags_, coverage_enabled, bool);
        flag_get_set!(flags_, block_coverage_enabled, bool);
        flag_get_set!(flags_, is_asm_wasm_broken, bool);
        flag_get_set!(flags_, class_scope_has_private_brand, bool);
        flag_get_set!(flags_, private_name_lookup_skips_outer_class, bool);
        flag_get_set!(flags_, requires_instance_members_initializer, bool);
        flag_get_set!(flags_, has_static_private_methods_or_accessors, bool);
        flag_get_set!(flags_, might_always_turbofan, bool);
        flag_get_set!(flags_, allow_natives_syntax, bool);
        flag_get_set!(flags_, allow_lazy_compile, bool);
        flag_get_set!(flags_, post_parallel_compile_tasks_for_eager_toplevel, bool);
        flag_get_set!(flags_, post_parallel_compile_tasks_for_lazy, bool);
        flag_get_set!(flags_, collect_source_positions, bool);
        flag_get_set!(flags_, is_repl_mode, bool);
        flag_get_set!(flags_, produce_compile_hints, bool);
        flag_get_set!(flags_, compile_hints_magic_enabled, bool);
        flag_get_set!(flags_, compile_hints_per_function_magic_enabled, bool);

        pub fn script_id(&self) -> i32 {
            self.script_id_
        }

        pub fn set_script_id(&mut self, value: i32) -> &mut Self {
            self.script_id_ = value;
            self
        }

        pub fn function_kind(&self) -> FunctionKind {
            self.function_kind_
        }

        pub fn set_function_kind(&mut self, value: FunctionKind) -> &mut Self {
            self.function_kind_ = value;
            self
        }

        pub fn function_syntax_kind(&self) -> FunctionSyntaxKind {
            self.function_syntax_kind_
        }

        pub fn set_function_syntax_kind(&mut self, value: FunctionSyntaxKind) -> &mut Self {
            self.function_syntax_kind_ = value;
            self
        }

        pub fn parsing_while_debugging(&self) -> ParsingWhileDebugging {
            self.parsing_while_debugging_
        }

        pub fn set_parsing_while_debugging(&mut self, value: ParsingWhileDebugging) -> &mut Self {
            self.parsing_while_debugging_ = value;
            self
        }
    }

    pub trait Functionlike {
        fn language_mode(&self) -> LanguageMode;
        fn kind(&self) -> FunctionKind;
        fn syntax_kind(&self) -> FunctionSyntaxKind;
        fn requires_instance_members_initializer(&self) -> bool;
        fn class_scope_has_private_brand(&self) -> bool;
        fn has_static_private_methods_or_accessors(&self) -> bool;
        fn private_name_lookup_skips_outer_class(&self) -> bool;
        fn is_toplevel(&self) -> bool;
    }

    struct BitFieldsStruct {
        is_toplevel: u32,
        is_eager: u32,
        is_eval: u32,
        is_reparse: u32,
        outer_language_mode: u32,
        parse_restriction: u32,
        is_module: u32,
        allow_lazy_parsing: u32,
        is_lazy_compile: u32,
        coverage_enabled: u32,
        block_coverage_enabled: u32,
        is_asm_wasm_broken: u32,
        class_scope_has_private_brand: u32,
        private_name_lookup_skips_outer_class: u32,
        requires_instance_members_initializer: u32,
        has_static_private_methods_or_accessors: u32,
        might_always_turbofan: u32,
        allow_natives_syntax: u32,
        allow_lazy_compile: u32,
        post_parallel_compile_tasks_for_eager_toplevel: u32,
        post_parallel_compile_tasks_for_lazy: u32,
        collect_source_positions: u32,
        is_repl_mode: u32,
        produce_compile_hints: u32,
        compile_hints_magic_enabled: u32,
        compile_hints_per_function_magic_enabled: u32,
    }

    pub struct UnoptimizedCompileState {
        pending_error_handler_: PendingCompilationErrorHandler,
    }

    impl UnoptimizedCompileState {
        pub fn new() -> Self {
            UnoptimizedCompileState {
                pending_error_handler_: PendingCompilationErrorHandler::new(),
            }
        }
        pub fn pending_error_handler(&self) -> &PendingCompilationErrorHandler {
            &self.pending_error_handler_
        }
        pub fn pending_error_handler_mut(&mut self) -> &mut PendingCompilationErrorHandler {
            &mut self.pending_error_handler_
        }
    }

    pub struct ReusableUnoptimizedCompileState {
        hash_seed_: u64,
        allocator_: *mut AccountingAllocator,
        v8_file_logger_: *mut V8FileLogger,
        dispatcher_: *mut LazyCompileDispatcher,
        ast_string_constants_: *mut AstStringConstants,
        ast_raw_string_zone_: Zone,
        single_parse_zone_: Zone,
        ast_value_factory_: Box<AstValueFactory>,
    }

    impl ReusableUnoptimizedCompileState {
        pub fn new(isolate: &mut Isolate) -> Self {
            let hash_seed_ = HashSeed(isolate);
            let allocator_ = isolate.allocator();
            let v8_file_logger_ = isolate.v8_file_logger();
            let dispatcher_ = isolate.lazy_compile_dispatcher();
            let ast_string_constants_ = isolate.ast_string_constants();
            let ast_raw_string_zone_ = Zone::new(allocator_, "unoptimized-compile-ast-raw-string-zone");
            let single_parse_zone_ = Zone::new(allocator_, "unoptimized-compile-parse-zone");
            let ast_value_factory_ = Box::new(AstValueFactory::new(
                &ast_raw_string_zone_,
                &single_parse_zone_,
                ast_string_constants_,
                hash_seed_,
            ));

            ReusableUnoptimizedCompileState {
                hash_seed_: hash_seed_,
                allocator_: allocator_,
                v8_file_logger_: v8_file_logger_,
                dispatcher_: dispatcher_,
                ast_string_constants_: ast_string_constants_,
                ast_raw_string_zone_: ast_raw_string_zone_,
                single_parse_zone_: single_parse_zone_,
                ast_value_factory_: ast_value_factory_,
            }
        }

        pub fn ast_raw_string_zone(&mut self) -> &mut Zone {
            &mut self.ast_raw_string_zone_
        }

        pub fn single_parse_zone(&mut self) -> &mut Zone {
            &mut self.single_parse_zone_
        }

        pub fn NotifySingleParseCompleted(&mut self) {
            self.single_parse_zone_.Reset();
        }

        pub fn ast_value_factory(&self) -> &AstValueFactory {
            &self.ast_value_factory_
        }

        pub fn hash_seed(&self) -> u64 {
            self.hash_seed_
        }

        pub fn allocator(&self) -> *mut AccountingAllocator {
            self.allocator_
        }

        pub fn ast_string_constants(&self) -> *mut AstStringConstants {
            self.ast_string_constants_
        }

        pub fn v8_file_logger(&self) -> *mut V8FileLogger {
            self.v8_file_logger_
        }

        pub fn dispatcher(&self) -> *mut LazyCompileDispatcher {
            self.dispatcher_
        }
    }

    impl Drop for ReusableUnoptimizedCompileState {
        fn drop(&mut self) {}
    }

    pub struct ParseInfo {
        flags_: UnoptimizedCompileFlags,
        state_: *mut UnoptimizedCompileState,
        reusable_state_: *mut ReusableUnoptimizedCompileState,
        extension_: *mut v8::Extension,
        script_scope_: *mut DeclarationScope,
        stack_limit_: usize,
        parameters_end_pos_: i32,
        max_info_id_: i32,
        character_stream_: Option<Box<Utf16CharacterStream>>,
        function_name_: *const AstRawString,
        runtime_call_stats_: *mut RuntimeCallStats,
        source_range_map_: *mut SourceRangeMap,
        literal_: *mut FunctionLiteral,
        allow_eval_cache_: bool,
        language_mode_: LanguageMode,
        is_background_compilation_: bool,
        is_streaming_compilation_: bool,
        has_module_in_scope_chain_: bool,
        compile_hint_callback_: Option<CompileHintCallback>,
        compile_hint_callback_data_: *mut std::ffi::c_void,
    }

    type CompileHintCallback = extern "C" fn(*mut std::ffi::c_void);

    const kNoSourcePosition: i32 = -1;
    const kInvalidInfoId: i32 = -1;

    impl ParseInfo {
        pub fn new(
            isolate: &mut Isolate,
            flags: UnoptimizedCompileFlags,
            state: *mut UnoptimizedCompileState,
            reusable_state: *mut ReusableUnoptimizedCompileState,
        ) -> Self {
            let stack_limit = isolate.stack_guard().real_climit();
            let runtime_call_stats = isolate.counters().runtime_call_stats();
            ParseInfo::new_internal(flags, state, reusable_state, stack_limit, runtime_call_stats)
        }

        fn new_internal(
            flags: UnoptimizedCompileFlags,
            state: *mut UnoptimizedCompileState,
            reusable_state: *mut ReusableUnoptimizedCompileState,
            stack_limit: usize,
            runtime_call_stats: *mut RuntimeCallStats,
        ) -> Self {
            let mut result = ParseInfo {
                flags_: flags,
                state_: state,
                reusable_state_: reusable_state,
                extension_: std::ptr::null_mut(),
                script_scope_: std::ptr::null_mut(),
                stack_limit_: stack_limit,
                parameters_end_pos_: kNoSourcePosition,
                max_info_id_: kInvalidInfoId,
                character_stream_: None,
                function_name_: std::ptr::null(),
                runtime_call_stats_: runtime_call_stats,
                source_range_map_: std::ptr::null_mut(),
                literal_: std::ptr::null_mut(),
                allow_eval_cache_: false,
                language_mode_: LanguageMode::Normal,
                is_background_compilation_: false,
                is_streaming_compilation_: false,
                has_module_in_scope_chain_: false,
                compile_hint_callback_: None,
                compile_hint_callback_data_: std::ptr::null_mut(),
            };

            if result.flags_.block_coverage_enabled() {
                result.AllocateSourceRangeMap();
            }

            result
        }

        pub fn zone(&self) -> &Zone {
            unsafe { &(*self.reusable_state_).single_parse_zone() }
        }

        pub fn flags(&self) -> &UnoptimizedCompileFlags {
            &self.flags_
        }

        pub fn hash_seed(&self) -> u64 {
            unsafe { (*self.reusable_state_).hash_seed() }
        }

        pub fn allocator(&self) -> *mut AccountingAllocator {
            unsafe { (*self.reusable_state_).allocator() }
        }

        pub fn ast_string_constants(&self) -> *mut AstStringConstants {
            unsafe { (*self.reusable_state_).ast_string_constants() }
        }

        pub fn v8_file_logger(&self) -> *mut V8FileLogger {
            unsafe { (*self.reusable_state_).v8_file_logger() }
        }

        pub fn dispatcher(&self) -> *mut LazyCompileDispatcher {
            unsafe { (*self.reusable_state_).dispatcher() }
        }

        pub fn state(&self) -> *mut UnoptimizedCompileState {
            self.state_
        }

        pub fn pending_error_handler(&mut self) -> &mut PendingCompilationErrorHandler {
            unsafe { &mut (*self.state_).pending_error_handler_ }
        }

        pub fn stack_limit(&self) -> usize {
            self.stack_limit_
        }

        pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
            self.runtime_call_stats_
        }

        pub fn allow_eval_cache(&self) -> bool {
            self.allow_eval_cache_
        }

        pub fn set_allow_eval_cache(&mut self, value: bool) {
            self.allow_eval_cache_ = value;
        }

        pub fn language_mode(&self) -> LanguageMode {
            self.language_mode_
        }

        pub fn set_language_mode(&mut self, value: LanguageMode) {
            self.language_mode_ = value;
        }

        pub fn character_stream(&self) -> Option<&Utf16CharacterStream> {
            self.character_stream_.as_ref().map(|stream| stream.as_ref())
        }

        pub fn set_character_stream(&mut self, character_stream: Utf16CharacterStream) {
            self.character_stream_ = Some(Box::new(character_stream));
        }

        pub fn ResetCharacterStream(&mut self) {
            self.character_stream_ = None;
        }

        pub fn extension(&self) -> *mut v8::Extension {
            self.extension_
        }

        pub fn set_extension(&mut self, extension: *mut v8::Extension) {
            self.extension_ = extension;
        }

        pub fn script_scope(&self) -> *mut DeclarationScope {
            self.script_scope_
        }

        pub fn set_script_scope(&mut self, script_scope: *mut DeclarationScope) {
            self.script_scope_ = script_scope;
        }

        pub fn ast_value_factory(&self) -> *mut AstValueFactory {
            unsafe { (*self.reusable_state_).ast_value_factory_.as_mut() as *mut AstValueFactory }
        }

        pub fn function_name(&self) -> *const AstRawString {
            self.function_name_
        }

        pub fn set_function_name(&mut self, function_name: *const AstRawString) {
            self.function_name_ = function_name;
        }

        pub fn literal(&self) -> *mut FunctionLiteral {
            self.literal_
        }

        pub fn set_literal(&mut self, literal: *mut FunctionLiteral) {
            self.literal_ = literal;
        }

        pub fn scope(&self) -> *mut DeclarationScope {
            std::ptr::null_mut()
        }

        pub fn parameters_end_pos(&self) -> i32 {
            self.parameters_end_pos_
        }

        pub fn set_parameters_end_pos(&mut self, parameters_end_pos: i32) {
            self.parameters_end_pos_ = parameters_end_pos;
        }

        pub fn is_wrapped_as_function(&self) -> bool {
            self.flags_.function_syntax_kind() == FunctionSyntaxKind::kWrapped
        }

        pub fn max_info_id(&self) -> i32 {
            self.max_info_id_
        }

        pub fn set_max_info_id(&mut self, max_info_id: i32) {
            self.max_info_id_ = max_info_id;
        }

        pub fn AllocateSourceRangeMap(&mut self) {
            assert!(self.flags().block_coverage_enabled());
            assert!(self.source_range_map_.is_null());
            unsafe {
                let source_range_map = (*self.reusable_state_).single_parse_zone().New::<SourceRangeMap>();
                self.set_source_range_map(source_range_map);
            }
        }

        pub fn source_range_map(&self) -> *mut SourceRangeMap {
            self.source_range_map_
        }

        pub fn set_source_range_map(&mut self, source_range_map: *mut SourceRangeMap) {
            self.source_range_map_ = source_range_map;
        }

        pub fn CheckFlagsForFunctionFromScript(&self, script: Tagged<Script>) {
            assert_eq!(self.flags().script_id(), script.id());
            assert_eq!(self.flags().is_eval() && !script.is_wrapped(), script.compilation_type() == Script::CompilationType::kEval);
            assert_eq!(self.flags().is_module(), script.origin_options().IsModule());
            assert!(!(self.flags().block_coverage_enabled() && script.IsUserJavaScript()) || !self.source_range_map().is_null());
        }

        pub fn is_background_compilation(&self) -> bool {
            self.is_background_compilation_
        }

        pub fn set_is_background_compilation(&mut self) {
            self.is_background_compilation_ = true;
        }

        pub fn is_streaming_compilation(&self) -> bool {
            self.is_streaming_compilation_
        }

        pub fn set_is_streaming_compilation(&mut self) {
            self.is_streaming_compilation_ = true;
        }

        pub fn has_module_in_scope_chain(&self) -> bool {
            self.has_module_in_scope_chain_
        }

        pub fn set_has_module_in_scope_chain(&mut self) {
            self.has_module_in_scope_chain_ = true;
        }
        
        pub fn SetCompileHintCallbackAndData(&mut self, callback: CompileHintCallback, data: *mut std::ffi::c_void) {
            assert!(self.compile_hint_callback_.is_none());
            assert!(self.compile_hint_callback_data_.is_null());
            self.compile_hint_callback_ = Some(callback);
            self.compile_hint_callback_data_ = data;
        }
        
        pub fn compile_hint_callback(&self) -> Option<CompileHintCallback> {
            self.compile_hint_callback_
        }
        
        pub fn compile_hint_callback_data(&self) -> *mut std::ffi::c_void {
            self.compile_hint_callback_data_
        }

        pub fn CreateScript(
            &mut self,
            isolate: &mut Isolate,
            source: DirectHandle<String>,
            maybe_wrapped_arguments: Option<DirectHandle<FixedArray>>,
            origin_options: ScriptOriginOptions,
            natives: NativesFlag,
        ) -> Handle<Script> {
            let event = if self.is_streaming_compilation() {
                if self.is_background_compilation() {
                    ScriptEventType::kStreamingCompileBackground
                } else {
                    ScriptEventType::kStreamingCompileForeground
                }
            } else if self.is_background_compilation() {
                ScriptEventType::kBackgroundCompile
            } else {
                ScriptEventType::kCreate
            };

            let mut script = isolate.factory().NewScriptWithId(source, self.flags().script_id(), event);
            let raw_script = script.get_mut();

            match natives {
                NativesFlag::EXTENSION_CODE => {
                    raw_script.set_type(ScriptTypeEnum::kExtension);
                }
                NativesFlag::INSPECTOR_CODE => {
                    raw_script.set_type(ScriptTypeEnum::kInspector);
                }
                NativesFlag::NOT_NATIVES_CODE => {}
            }

            raw_script.set_origin_options(origin_options);
            raw_script.set_is_repl_mode(self.flags().is_repl_mode());

            if self.is_wrapped_as_function() {
                raw_script.set_wrapped_arguments(maybe_wrapped_arguments.unwrap());
            } else if self.flags().is_eval() {
                raw_script.set_compilation_type(Script::CompilationType::kEval);
            }

            self.CheckFlagsForToplevelCompileFromScript(script.get());
            script
        }

        fn CheckFlagsForToplevelCompileFromScript(&self, script: Tagged<Script>) {
            self.CheckFlagsForFunctionFromScript(script);
            assert!(self.flags().is_toplevel());
            assert_eq!(self.flags().is_repl_mode(), script.is_repl_mode());

            if script.is_wrapped() {
                assert_eq!(self.flags().function_syntax_kind(), FunctionSyntaxKind::kWrapped);
            }
        }

    }

    impl Drop for ParseInfo {
        fn drop(&mut self) {
            unsafe {
                (*self.reusable_state_).NotifySingleParseCompleted();
            }
        }
    }

    pub struct PendingCompilationErrorHandler {}
    impl PendingCompilationErrorHandler {
        pub fn new() -> Self {
            PendingCompilationErrorHandler{}
        }
    }
    pub fn stricter_language_mode(mode1: LanguageMode, mode2: LanguageMode) -> LanguageMode {
        match (mode1, mode2) {
            (LanguageMode::Strict, _) | (_, LanguageMode::Strict) => LanguageMode::Strict,
            _ => LanguageMode::Normal,
        }
    }
    pub fn construct_repl_mode(is_repl_mode: bool) -> REPLMode {
        if is_repl_mode {
            REPLMode::kYes
        } else {
            REPLMode::kNo
        }
    }

    pub struct Isolate {
        next_script_id: i32,
        best_effort_code_coverage: bool,
        block_code_coverage: bool,
        allocator: *mut AccountingAllocator,
        v8_file_logger: *mut V8FileLogger,
        lazy_compile_dispatcher: *mut LazyCompileDispatcher,
        ast_string_constants: *mut AstStringConstants,
        counters: Counters,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                next_script_id: 0,
                best_effort_code_coverage: false,
                block_code_coverage: false,
                allocator: std::ptr::null_mut(),
                v8_file_logger: std::ptr::null_mut(),
                lazy_compile_dispatcher: std::ptr::null_mut(),
                ast_string_constants: std::ptr::null_mut(),
                counters: Counters::new(),
            }
        }
        fn get_next_script_id(&mut self) -> i32 {
            self.next_script_id += 1;
            self.next_script_id
        }
        fn is_best_effort_code_coverage(&self) -> bool {
            self.best_effort_code_coverage
        }
        fn is_block_code_coverage(&self) -> bool {
            self.block_code_coverage
        }
        fn allocator(&mut self) -> *mut AccountingAllocator {
            self.allocator
        }
        fn v8_file_logger(&mut self) -> *mut V8FileLogger {
            self.v8_file_logger
        }
        fn lazy_compile_

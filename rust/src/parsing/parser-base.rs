// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a partial translation, and some parts are still represented with placeholders.

use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr};
use std::rc::Rc;
use std::vec::Vec;

// Placeholder for base/flags.h, assuming it defines a Flags struct/trait
pub mod base {
    use std::marker::PhantomData;
    use std::ops::{BitAnd, BitOr};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags<T: Copy + Clone + PartialEq + Eq + Hash + 'static> {
        bits: u8,
        _phantom: PhantomData<T>,
    }

    impl<T: Copy + Clone + PartialEq + Eq + Hash + 'static> Flags<T> {
        pub const fn new(bits: u8) -> Self {
            Flags {
                bits,
                _phantom: PhantomData,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.bits == 0
        }
    }

    impl<T: Copy + Clone + PartialEq + Eq + Hash + 'static> BitOr for Flags<T> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags {
                bits: self.bits | other.bits,
                _phantom: PhantomData,
            }
        }
    }

    impl<T: Copy + Clone + PartialEq + Eq + Hash + 'static> BitAnd for Flags<T> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Flags {
                bits: self.bits & other.bits,
                _phantom: PhantomData,
            }
        }
    }

    impl<T: Copy + Clone + PartialEq + Eq + Hash + 'static> Flags<T> {
        pub fn contains(&self, other: &Self) -> bool {
            (self.bits & other.bits) == other.bits
        }

        pub fn insert(&mut self, other: Self) {
            self.bits |= other.bits;
        }

        pub fn remove(&mut self, other: Self) {
            self.bits &= !other.bits;
        }
    }
}

// Placeholder for base/hashmap.h
pub mod base_hashmap {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub struct HashMapWrapper<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Eq + Hash, V> HashMapWrapper<K, V> {
        pub fn new() -> Self {
            HashMapWrapper {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.map.insert(key, value);
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key)
        }

        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.map.remove(key)
        }

        pub fn contains_key(&self, key: &K) -> bool {
            self.map.contains_key(key)
        }
    }
}

pub mod internal {
    pub use super::base;
    use super::base::Flags;
    use super::common::{LanguageMode, MessageTemplate};
    use super::parsing::Scanner;
    use super::regexp::RegExpFlags;
    use std::cell::RefCell;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use std::vec::Vec;

    //Placeholder type definitions
    pub type Identifier = Rc<String>;
    pub type Expression = Rc<String>;
    pub type FunctionLiteral = Rc<String>;
    pub type ObjectLiteralProperty = Rc<String>;
    pub type ClassLiteralProperty = Rc<String>;
    pub type ExpressionList = Rc<Vec<Expression>>;
    pub type ObjectPropertyList = Rc<Vec<ObjectLiteralProperty>>;
    pub type ClassPropertyList = Rc<Vec<ClassLiteralProperty>>;
    pub type ClassStaticElementList = Rc<Vec<ClassLiteralProperty>>;
    pub type FormalParameters = Rc<String>;
    pub type Statement = Rc<String>;
    pub type StatementList = Rc<Vec<Statement>>;
    pub type Block = Rc<String>;
    pub type BreakableStatement = Rc<String>;
    pub type ForStatement = Rc<String>;
    pub type IterationStatement = Rc<String>;
    pub type SourceRange = Rc<String>;
    pub type FuncNameInferrer = Rc<String>;
    pub type Factory = Rc<String>;
    pub type Suspend = Rc<String>;
    pub type AstRawString = String;

    pub const kNoSourcePosition: i32 = -1;

    pub enum FunctionNameValidity {
        kFunctionNameIsStrictReserved,
        kSkipFunctionNameCheck,
        kFunctionNameValidityUnknown,
    }

    pub enum AllowLabelledFunctionStatement {
        kAllowLabelledFunctionStatement,
        kDisallowLabelledFunctionStatement,
    }

    pub enum ParsingArrowHeadFlag {
        kCertainlyNotArrowHead,
        kMaybeArrowHead,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ParseFunctionFlag {
        kIsNormal = 0,
        kIsGenerator = 1 << 0,
        kIsAsync = 1 << 1,
    }

    impl ParseFunctionFlag {
        pub fn from_bits(bits: u8) -> Self {
            match bits {
                1 => ParseFunctionFlag::kIsGenerator,
                2 => ParseFunctionFlag::kIsAsync,
                _ => ParseFunctionFlag::kIsNormal,
            }
        }
    }

    pub type ParseFunctionFlags = Flags<ParseFunctionFlag>;

    #[derive(Debug)]
    pub struct FormalParametersBase {
        pub scope: *mut DeclarationScope, // Mutable pointer
        pub has_rest: bool,
        pub is_simple: bool,
        pub function_length: i32,
        pub arity: i32,
    }

    impl FormalParametersBase {
        pub fn new(scope: *mut DeclarationScope) -> Self {
            FormalParametersBase {
                scope,
                has_rest: false,
                is_simple: true,
                function_length: 0,
                arity: 0,
            }
        }

        pub fn num_parameters(&self) -> i32 {
            self.arity - (self.has_rest as i32)
        }

        pub fn update_arity_and_function_length(&mut self, is_optional: bool, is_rest: bool) {
            if !is_optional && !is_rest && self.function_length == self.arity {
                self.function_length += 1;
            }
            self.arity += 1;
        }
    }

    // Stack-allocated scope to collect source ranges from the parser.
    pub struct SourceRangeScope<'a> {
        scanner_: &'a Scanner,
        range_: &'a mut SourceRange,
    }

    impl<'a> SourceRangeScope<'a> {
        pub fn new(scanner: &'a Scanner, range: &'a mut SourceRange) -> Self {
            // range_.start = scanner.peek_location().beg_pos;
            // DCHECK_NE(range_.start, kNoSourcePosition);
            // DCHECK_EQ(range_.end, kNoSourcePosition);
            SourceRangeScope {
                scanner_: scanner,
                range_: range,
            }
        }
    }

    impl<'a> Drop for SourceRangeScope<'a> {
        fn drop(&mut self) {
            //DCHECK_EQ(kNoSourcePosition, range_.end);
            //range_.end = scanner_.location().end_pos;
            //DCHECK_NE(range_.end, kNoSourcePosition);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ParsePropertyKind {
        kAutoAccessorClassField,
        kAccessorGetter,
        kAccessorSetter,
        kValue,
        kShorthand,
        kAssign,
        kMethod,
        kClassField,
        kShorthandOrClassField,
        kSpread,
        kNotSet,
    }

    pub struct ParserBase<Impl> {
        scope_: *mut Scope,        // Mutable pointer to Scope
        original_scope_: *mut Scope, // Mutable pointer to Scope
        function_state_: *mut FunctionState, //Mutable pointer to FunctionState
        fni_: FuncNameInferrer,
        ast_value_factory_: *mut AstValueFactory, // Mutable pointer to AstValueFactory
        ast_node_factory_: Factory,
        runtime_call_stats_: *mut RuntimeCallStats, // Mutable pointer to RuntimeCallStats
        v8_file_logger_: *mut V8FileLogger, // Mutable pointer to V8FileLogger
        parsing_on_main_thread_: bool,
        stack_limit_: usize,
        pending_error_handler_: *mut PendingCompilationErrorHandler, // Mutable pointer to PendingCompilationErrorHandler
        zone_: *mut Zone,          // Mutable pointer to Zone
        expression_scope_: *mut ExpressionScope<Types<Impl>>, // Mutable pointer to ExpressionScope
        scanner_: *mut Scanner,      // Mutable pointer to Scanner
        flags_: UnoptimizedCompileFlags,
        info_id_: i32,
        has_module_in_scope_chain_: bool,
        default_eager_compile_hint_: FunctionLiteralEagerCompileHint,
        compile_hints_magic_enabled_: bool,
        compile_hints_per_function_magic_enabled_: bool,
        pointer_buffer_: RefCell<Vec<*mut std::ffi::c_void>>,
        variable_buffer_: RefCell<Vec<(*mut VariableProxy, i32)>>,
        accept_IN_: bool,
        allow_eval_cache_: bool,
        parameters_: *mut FormalParameters,
        next_arrow_function_info_: NextArrowFunctionInfo,
        position_after_last_primary_expression_open_parenthesis_: i32,

        _phantom: PhantomData<Impl>,
    }

    impl<Impl> ParserBase<Impl> {
        pub fn new(
            zone: *mut Zone,
            scanner: *mut Scanner,
            stack_limit: usize,
            ast_value_factory: *mut AstValueFactory,
            pending_error_handler: *mut PendingCompilationErrorHandler,
            runtime_call_stats: *mut RuntimeCallStats,
            v8_file_logger: *mut V8FileLogger,
            flags: UnoptimizedCompileFlags,
            parsing_on_main_thread: bool,
            compile_hints_magic_enabled: bool,
            compile_hints_per_function_magic_enabled: bool,
        ) -> Self {
            let mut parser = ParserBase {
                scope_: std::ptr::null_mut(),
                original_scope_: std::ptr::null_mut(),
                function_state_: std::ptr::null_mut(),
                fni_: "FuncNameInferrer".to_string(), //Placeholder value
                ast_value_factory_: ast_value_factory,
                ast_node_factory_: "AstNodeFactory".to_string(), //Placeholder value
                runtime_call_stats_: runtime_call_stats,
                v8_file_logger_: v8_file_logger,
                parsing_on_main_thread_: parsing_on_main_thread,
                stack_limit_: stack_limit,
                pending_error_handler_: pending_error_handler,
                zone_: zone,
                expression_scope_: std::ptr::null_mut(),
                scanner_: scanner,
                flags_: flags,
                info_id_: 0,
                has_module_in_scope_chain_: flags.is_module,
                default_eager_compile_hint_: FunctionLiteralEagerCompileHint::kShouldLazyCompile,
                compile_hints_magic_enabled_: compile_hints_magic_enabled,
                compile_hints_per_function_magic_enabled_: compile_hints_per_function_magic_enabled,
                pointer_buffer_: RefCell::new(Vec::new()),
                variable_buffer_: RefCell::new(Vec::new()),
                accept_IN_: true,
                allow_eval_cache_: true,
                parameters_: std::ptr::null_mut(),
                next_arrow_function_info_: NextArrowFunctionInfo::new(),
                position_after_last_primary_expression_open_parenthesis_: -1,
                _phantom: PhantomData,
            };
            parser
        }

        // All implementation-specific methods must be called through this.
        #[allow(clippy::borrowed_box)]
        fn impl_mut(&mut self) -> &mut Impl {
            unsafe { &mut *(self as *mut Self as *mut Impl) }
        }

        fn impl_ref(&self) -> &Impl {
            unsafe { &*(self as *const Self as *const Impl) }
        }

        pub fn flags(&self) -> &UnoptimizedCompileFlags {
            &self.flags_
        }

        pub fn has_module_in_scope_chain(&self) -> bool {
            self.has_module_in_scope_chain_
        }

        pub fn is_parsing_while_debugging(&self) -> bool {
            self.flags_.parsing_while_debugging == ParsingWhileDebugging::kYes
        }

        pub fn allow_eval_cache(&self) -> bool {
            self.allow_eval_cache_
        }

        pub fn set_allow_eval_cache(&mut self, allow: bool) {
            self.allow_eval_cache_ = allow;
        }

        pub fn has_error(&self) -> bool {
            unsafe { (*self.scanner_).has_parser_error() }
        }

        pub fn stack_limit(&self) -> usize {
            self.stack_limit_
        }

        pub fn set_stack_limit(&mut self, stack_limit: usize) {
            self.stack_limit_ = stack_limit;
        }

        pub fn set_default_eager_compile_hint(&mut self, eager_compile_hint: FunctionLiteralEagerCompileHint) {
            self.default_eager_compile_hint_ = eager_compile_hint;
        }

        pub fn default_eager_compile_hint(&self) -> FunctionLiteralEagerCompileHint {
            self.default_eager_compile_hint_
        }

        pub fn loop_nesting_depth(&self) -> i32 {
            unsafe { (*self.function_state_).loop_nesting_depth() }
        }

        pub fn peek_next_info_id(&self) -> i32 {
            self.info_id_ + 1
        }

        pub fn get_next_info_id(&mut self) -> i32 {
            self.info_id_ += 1;
            self.info_id_
        }

        pub fn get_last_info_id(&self) -> i32 {
            self.info_id_
        }

        pub fn skip_infos(&mut self, delta: i32) {
            self.info_id_ += delta;
        }

        pub fn reset_info_id(&mut self) {
            self.info_id_ = 0;
        }

        pub fn main_zone(&self) -> *mut Zone {
            unsafe { (*self.ast_value_factory_).single_parse_zone() }
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }

        // V8_INLINE bool IsExtraordinaryPrivateNameAccessAllowed() const;
        //TODO - IMPLEMENT

        // enum VariableDeclarationContext {
        //     kStatementListItem,
        //     kStatement,
        //     kForStatement
        // };
        //TODO - IMPLEMENT

        // class ClassLiteralChecker;
        //TODO - IMPLEMENT

        // class BlockState
        //TODO - IMPLEMENT

        // class Target
        //TODO - IMPLEMENT

        // class FunctionState
        //TODO - IMPLEMENT

        // struct DeclarationDescriptor
        //TODO - IMPLEMENT

        // struct DeclarationParsingResult
        //TODO - IMPLEMENT

        // struct CatchInfo
        //TODO - IMPLEMENT

        // struct ForInfo
        //TODO - IMPLEMENT

        // struct ClassInfo
        //TODO - IMPLEMENT

        // enum class PropertyPosition { kObjectLiteral, kClassLiteral };
        // struct ParsePropertyInfo
        //TODO - IMPLEMENT

        // void DeclareLabel(ZonePtrList<const AstRawString>** labels,
        //                     ZonePtrList<const AstRawString>** own_labels,
        //                     const AstRawString* label)
        //TODO - IMPLEMENT

        // bool ContainsLabel(const ZonePtrList<const AstRawString>* labels,
        //                     const AstRawString* label)
        //TODO - IMPLEMENT

        // bool TargetStackContainsLabel(const AstRawString* label)
        //TODO - IMPLEMENT

        // ClassLiteralProperty::Kind ClassPropertyKindFor(ParsePropertyKind kind)
        //TODO - IMPLEMENT

        // VariableMode GetVariableMode(ClassLiteralProperty::Kind kind)
        //TODO - IMPLEMENT

        // const AstRawString* ClassFieldVariableName(AstValueFactory* ast_value_factory,
        //                                              int index)
        //TODO - IMPLEMENT

        // const AstRawString* AutoAccessorVariableName(
        //     AstValueFactory* ast_value_factory, int index)
        //TODO - IMPLEMENT

        // DeclarationScope* NewScriptScope(REPLMode repl_mode) const
        //TODO - IMPLEMENT

        // DeclarationScope* NewVarblockScope() const
        //TODO - IMPLEMENT

        // ModuleScope* NewModuleScope(DeclarationScope* parent) const
        //TODO - IMPLEMENT

        // DeclarationScope* NewEvalScope(Scope* parent) const
        //TODO - IMPLEMENT

        // ClassScope* NewClassScope(Scope* parent, bool is_anonymous) const
        //TODO - IMPLEMENT

        // Scope* NewBlockScopeForObjectLiteral()
        //TODO - IMPLEMENT

        // Scope* NewScope(ScopeType scope_type) const
        //TODO - IMPLEMENT

        // Scope* NewScopeWithParent(Scope* parent, ScopeType scope_type) const
        //TODO - IMPLEMENT

        // DeclarationScope* NewFunctionScope(FunctionKind kind,
        //                                     Zone* parse_zone = nullptr) const
        //TODO - IMPLEMENT

        // V8_INLINE DeclarationScope* GetDeclarationScope() const
        //TODO - IMPLEMENT

        // V8_INLINE DeclarationScope* GetClosureScope() const
        //TODO - IMPLEMENT

        // VariableProxy* NewRawVariable(const AstRawString* name, int pos)
        //TODO - IMPLEMENT

        // VariableProxy* NewUnresolved(const AstRawString* name)
        //TODO - IMPLEMENT

        // VariableProxy* NewUnresolved(const AstRawString* name, int begin_pos,
        //                                VariableKind kind = NORMAL_VARIABLE)
        //TODO - IMPLEMENT

        pub fn scanner(&self) -> *mut Scanner {
            self.scanner_
        }

        pub fn ast_value_factory(&self) -> *mut AstValueFactory {
            self.ast_value_factory_
        }

        pub fn position(&self) -> i32 {
            unsafe { (*self.scanner_).location().beg_pos }
        }

        pub fn peek_position(&self) -> i32 {
            unsafe { (*self.scanner_).peek_location().beg_pos }
        }

        pub fn end_position(&self) -> i32 {
            unsafe { (*self.scanner_).location().end_pos }
        }

        pub fn peek_end_position(&self) -> i32 {
            unsafe { (*self.scanner_).peek_location().end_pos }
        }

        pub fn stack_overflow(&self) -> bool {
            unsafe { (*self.pending_error_handler_).stack_overflow() }
        }

        pub fn set_stack_overflow(&mut self) {
            unsafe {
                (*self.scanner_).set_parser_error();
                (*self.pending_error_handler_).set_stack_overflow();
            }
        }

        pub fn check_stack_overflow(&mut self) {
            // Any further calls to Next or peek will return the illegal token.
            if self.get_current_stack_position() < self.stack_limit_ {
                self.set_stack_overflow();
            }
        }

        fn get_current_stack_position(&self) -> usize {
            // Placeholder implementation
            0
        }

        pub fn peek(&self) -> TokenValue {
            unsafe { (*self.scanner_).peek() }
        }

        // Returns the position past the following semicolon (if it exists), and the
        // position past the end of the current token otherwise.
        pub fn position_after_semicolon(&self) -> i32 {
            if self.peek() == TokenValue::kSemicolon {
                self.peek_end_position()
            } else {
                self.end_position()
            }
        }

        pub fn peek_ahead_ahead(&self) -> TokenValue {
            unsafe { (*self.scanner_).peek_ahead_ahead() }
        }

        pub fn peek_ahead(&self) -> TokenValue {
            unsafe { (*self.scanner_).peek_ahead() }
        }

        pub fn next(&mut self) -> TokenValue {
            unsafe { (*self.scanner_).next() }
        }

        pub fn consume(&mut self, token: TokenValue) {
            let next = self.next();
            if !self.has_error() {
                assert_eq!(next, token);
            }
        }

        pub fn check(&mut self, token: TokenValue) -> bool {
            let next = self.peek();
            if next == token {
                self.consume(next);
                true
            } else {
                false
            }
        }

        pub fn expect(&mut self, token: TokenValue) {
            let next = self.next();
            if next != token {
                self.report_unexpected_token(next);
            }
        }

        pub fn expect_semicolon(&mut self) {
            // Check for automatic semicolon insertion according to
            // the rules given in ECMA-262, section 7.9, page 21.
            let tok = self.peek();
            if tok == TokenValue::kSemicolon {
                self.next();
                return;
            }
            if unsafe { (*self.scanner_).has_line_terminator_before_next() }
                || TokenValue::is_auto_semicolon(tok)
            {
                return;
            }

            if unsafe { (*self.scanner_).current_token() } == TokenValue::kAwait
                && !self.is_async_function()
            {
                if self.is_parsing_while_debugging() {
                    self.report_message_at(
                        unsafe { (*self.scanner_).location() },
                        MessageTemplate::kAwaitNotInDebugEvaluate,
                    );
                } else {
                    self.report_message_at(
                        unsafe { (*self.scanner_).location() },
                        MessageTemplate::kAwaitNotInAsyncContext,
                    );
                }
                return;
            }

            self.report_unexpected_token(self.next());
        }

        pub fn peek_any_identifier(&self) -> bool {
            TokenValue::is_any_identifier(self.peek())
        }

        pub fn peek_contextual_keyword(&self, name: &AstRawString) -> bool {
            self.peek() == TokenValue::kIdentifier
                && unsafe { !(*self.scanner_).next_literal_contains_escapes() }
                && unsafe { (*self.scanner_).next_symbol(self.ast_value_factory_) } == *name
        }

        pub fn peek_contextual_keyword_token(&self, token: TokenValue) -> bool {
            self.peek() == token && unsafe { !(*self.scanner_).next_literal_contains_escapes() }
        }

        pub fn check_contextual_keyword(&mut self, name: &AstRawString) -> bool {
            if self.peek_contextual_keyword(name) {
                self.consume(TokenValue::kIdentifier);
                true
            } else {
                false
            }
        }

        pub fn check_contextual_keyword_token(&mut self, token: TokenValue) -> bool {
            if self.peek_contextual_keyword_token(token) {
                self.consume(token);
                true
            } else {
                false
            }
        }

        pub fn expect_contextual_keyword(&mut self, name: &AstRawString, fullname: Option<&str>, pos: Option<i32>) {
            self.expect(TokenValue::kIdentifier);
            if unsafe { (*self.scanner_).current_symbol(self.ast_value_factory_) } != *name {
                self.report_unexpected_token(unsafe { (*self.scanner_).current_token() });
            }
            if unsafe { (*self.scanner_).literal_contains_escapes() } {
                let full = fullname.unwrap_or(name.as_str());
                let start = pos.unwrap_or(self.position());
                self.impl_mut().report_message_at(
                    ScannerLocation {
                        beg_pos: start,
                        end_pos: self.end_position(),
                    },
                    MessageTemplate::kInvalidEscapedMetaProperty,
                    &full.to_string(),
                );
            }
        }

        pub fn expect_contextual_keyword_token(&mut self, token: TokenValue) {
            // Token Should be in range of Token::kIdentifier + 1 to Token::kAsync
            assert!((TokenValue::kGet as u8..=TokenValue::kAsync as u8).contains(&(token as u8)));
            let next = self.next();
            if next != token {
                self.report_unexpected_token(next);
            }
            if unsafe { (*self.scanner_).literal_contains_escapes() } {
                self.impl_mut().report_unexpected_token(TokenValue::kEscapedKeyword);
            }
        }

        pub fn check_in_or_of(&mut self, visit_mode: &mut ForEachStatementVisitMode) -> bool {
            if self.check(TokenValue::kIn) {
                *visit_mode = ForEachStatementVisitMode::ENUMERATE;
                true
            } else if self.check_contextual_keyword_token(TokenValue::kOf) {
                *visit_mode = ForEachStatementVisitMode::ITERATE;
                true
            } else {
                false
            }
        }

        pub fn peek_in_or_of(&self) -> bool {
            self.peek() == TokenValue::kIn || self.peek_contextual_keyword_token(TokenValue::kOf)
        }

        // Checks whether an octal literal was last seen between beg_pos and end_pos.
        // Only called for strict mode strings.
        pub fn check_strict_octal_literal(&mut self, beg_pos: i32, end_pos: i32) {
            let octal = unsafe { (*self.scanner_).octal_position() };
            if octal.is_valid() && beg_pos <= octal.beg_pos && octal.end_pos <= end_pos {
                let message = unsafe { (*self.scanner_).octal_message() };
                assert_ne!(message, MessageTemplate::kNone);
                self.impl_mut().report_message_at(octal, message, &"octal".to_string()); // Placeholder argument
                unsafe { (*self.scanner_).clear_octal_position() };
                if message == MessageTemplate::kStrictDecimalWithLeadingZero {
                    // TODO: Add CountUsage call
                    // self.impl_mut().CountUsage(v8::Isolate::kDecimalWithLeadingZeroInStrictMode);
                }
            }
        }

        // Checks if an octal literal or an invalid hex or unicode escape sequence
        // appears in the current template literal token. In the presence of such,
        // either returns false or reports an error, depending on should_throw.
        // Otherwise returns true.
        pub fn check_template_escapes(&mut self, should_throw: bool) -> bool {
            assert!(TokenValue::is_template(unsafe { (*self.scanner_).current_token() }));
            if unsafe { !(*self.scanner_).has_invalid_template_escape() } {
                return true;
            }

            // Handle error case(s)
            if should_throw {
                self.impl_mut().report_message_at(
                    unsafe { (*self.scanner_).invalid_template_escape_location() },
                    unsafe { (*self.scanner_).invalid_template_escape_message() },
                    &"template escape".to_string(), // Placeholder
                );
            }
            unsafe { (*self.scanner_).clear_invalid_template_escape_message() };
            should_throw
        }

        // ExpressionT ParsePossibleDestructuringSubPattern(AccumulationScope* scope);
        //TODO - IMPLEMENT

        // void ClassifyParameter(IdentifierT parameter, int beg_pos, int end_pos);
        //TODO - IMPLEMENT

        // void ClassifyArrowParameter(AccumulationScope* accumulation_scope,
        //                               int position, ExpressionT parameter);
        //TODO - IMPLEMENT

        // Checking the name of a function literal. This has to be done after parsing
        // the function, since the function can declare itself strict.
        pub fn check_function_name(
            &mut self,
            language_mode: LanguageMode,
            function_name: Identifier,
            function_name_validity: FunctionNameValidity,
            function_name_loc: &ScannerLocation,
        ) {
            if self.impl_mut().is_null_identifier(&function_name) {
                return;
            }
            if function_name_validity == FunctionNameValidity::kSkipFunctionNameCheck {
                return;
            }
            // The function name needs to be checked in strict mode.
            if self.impl_mut().is_sloppy(language_mode) {
                return;
            }

            if self.impl_mut().is_eval_or_arguments(&function_name) {
                self.impl_mut().report_message_at(
                    function_name_loc.clone(),
                    MessageTemplate::kStrictEvalArguments,
                    &"check_function_name".to_string(), //Placeholder
                );
                return;
            }
            if function_name_validity == FunctionNameValidity::kFunctionNameIsStrictReserved {
                self.impl_mut().report_message_at(
                    function_name_loc.clone(),
                    MessageTemplate::kUnexpectedStrictReserved,
                    &"check_function_name".to_string(), //Placeholder
                );
                return;
            }
        }

        // Types::Factory* factory() { return &ast_node_factory_; }
        //TODO - IMPLEMENT

        // DeclarationScope* GetReceiverScope() const
        //TODO - IMPLEMENT

        pub fn language_mode(&self) -> LanguageMode {
            unsafe { (*self.scope_).language_mode() }
        }

        // void RaiseLanguageMode(LanguageMode mode)
        //TODO - IMPLEMENT

        pub fn is_generator(&self) -> bool {
            Self::is_generator_function(unsafe { (*self.function_state_).kind() })
        }

        pub fn is_async_function(&self) -> bool {
            Self::is_async_function_kind(unsafe { (*self.function_state_).kind() })
        }

        pub fn is_async_generator(&self) -> bool {
            Self::is_async_generator_function(unsafe { (*self.function_state_).kind() })
        }

        pub fn is_resumable(&self) -> bool {
            Self::is_resumable_function(unsafe { (*self.function_state_).kind() })
        }

        pub fn is_await_allowed(&self) -> bool {
            self.is_async_function() || Self::is_module_kind(unsafe { (*self.function_state_).kind() })
        }

        pub fn is_await_as_identifier_disallowed(&self) -> bool {
            self.flags_.is_module
                || self.is_await_as_identifier_disallowed_function_kind(unsafe { (*self.function_state_).kind() })
        }

        fn is_await_as_identifier_disallowed_function_kind(&self, kind: FunctionKind) -> bool {
            // 'await' is always disallowed as an identifier in module contexts. Callers
            // should short-circuit the module case instead of calling this.
            //
            // There is one special case: direct eval inside a module. In that case,
            // even though the eval script itself is parsed as a Script (not a Module,
            // i.e. flags().is_module() is false), thus allowing await as an identifier
            // by default, the immediate outer scope is a module scope.
            assert!(
                !Self::is_module_kind(kind)
                    || (self.flags_.is_eval
                        && unsafe { (*self.function_state_).scope() } == self.original_scope_
                        && Self::is_module_kind(unsafe { (*self.function_state_).kind() }))
            );
            Self::is_async_function_kind(kind) || kind == FunctionKind::kClassStaticInitializerFunction
        }

        pub fn is_using_allowed(&self) -> bool {
            // UsingDeclaration and AwaitUsingDeclaration are Syntax Errors if the goal
            // symbol is Script. UsingDeclaration and AwaitUsingDeclaration are not
            // contained, either directly or indirectly, within a Block, CaseBlock,
            // ForStatement, ForInOfStatement, FunctionBody, GeneratorBody,
            // AsyncGeneratorBody, AsyncFunctionBody, ClassStaticBlockBody, or
            // ClassBody. Unless the current scope's ScopeType is ScriptScope, the
            // current position is directly or indirectly within one of the productions
            // listed above since they open a new scope.
            let scope_type = unsafe { (*self.scope_).scope_type() };
            (scope_type != ScopeType::SCRIPT_SCOPE && scope_
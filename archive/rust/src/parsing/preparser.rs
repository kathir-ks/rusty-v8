// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod ast {
    pub mod ast_value_factory {}
    pub mod ast {}
    pub mod scopes {}
}

mod parsing {
    pub mod parse_info {}
    pub mod parser_base {}
    pub mod pending_compilation_error_handler {}
    pub mod preparser_logger {}
}

mod base {
    pub fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
        value >= lower && value <= upper
    }

    pub struct BitField<T, const START: usize, const LENGTH: usize>;

    impl<T, const START: usize, const LENGTH: usize> BitField<T, START, LENGTH> {
        pub fn encode(_value: T) -> u32 {
            0 // Dummy implementation
        }
        pub fn decode(_code: u32) -> T {
            // Dummy implementation
            unimplemented!()
        }

        pub fn next<U, const NEXT_LENGTH: usize>() -> BitField<U, START + LENGTH, NEXT_LENGTH> {
            BitField::<U, START + LENGTH, NEXT_LENGTH>
        }

        pub fn update(_code: u32, _value: T) -> u32 {
            0 // Dummy implementation
        }
    }
}

pub mod internal {
    use crate::ast::ast_value_factory::*;
    use crate::ast::ast::*;
    use crate::ast::scopes::*;
    use crate::parsing::parse_info::*;
    use crate::parsing::parser_base::*;
    use crate::parsing::pending_compilation_error_handler::*;
    use crate::parsing::preparser_logger::*;
    use std::marker::PhantomData;
    use std::ptr::null;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PreParserIdentifier {
        string_: *const AstRawString,
        type_: PreParserIdentifierType,
    }

    impl PreParserIdentifier {
        pub fn new() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::UnknownIdentifier,
            }
        }

        pub fn default() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::UnknownIdentifier,
            }
        }
        pub fn null_() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::NullIdentifier,
            }
        }
        pub fn eval() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::EvalIdentifier,
            }
        }
        pub fn arguments() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::ArgumentsIdentifier,
            }
        }
        pub fn constructor() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::ConstructorIdentifier,
            }
        }
        pub fn async_() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::AsyncIdentifier,
            }
        }
        pub fn private_name() -> Self {
            PreParserIdentifier {
                string_: null(),
                type_: PreParserIdentifierType::PrivateNameIdentifier,
            }
        }
        pub fn is_null(&self) -> bool {
            self.type_ == PreParserIdentifierType::NullIdentifier
        }
        pub fn is_eval(&self) -> bool {
            self.type_ == PreParserIdentifierType::EvalIdentifier
        }
        pub fn is_async(&self) -> bool {
            self.type_ == PreParserIdentifierType::AsyncIdentifier
        }
        pub fn is_arguments(&self) -> bool {
            self.type_ == PreParserIdentifierType::ArgumentsIdentifier
        }
        pub fn is_eval_or_arguments(&self) -> bool {
            if PreParserIdentifierType::EvalIdentifier as u8 + 1 != PreParserIdentifierType::ArgumentsIdentifier as u8 {
                panic!("kEvalIdentifier + 1 != kArgumentsIdentifier");
            }
            crate::base::is_in_range(self.type_ as u8, PreParserIdentifierType::EvalIdentifier as u8, PreParserIdentifierType::ArgumentsIdentifier as u8)
        }
        pub fn is_constructor(&self) -> bool {
            self.type_ == PreParserIdentifierType::ConstructorIdentifier
        }
        pub fn is_private_name(&self) -> bool {
            self.type_ == PreParserIdentifierType::PrivateNameIdentifier
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PreParserIdentifierType {
        NullIdentifier,
        UnknownIdentifier,
        EvalIdentifier,
        ArgumentsIdentifier,
        ConstructorIdentifier,
        AsyncIdentifier,
        PrivateNameIdentifier,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PreParserExpression {
        code_: u32,
    }

    impl PreParserExpression {
        pub fn new() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Null),
            }
        }

        pub fn null_() -> Self {
            PreParserExpression::new()
        }
        pub fn failure() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Failure),
            }
        }

        pub fn default() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression),
            }
        }

        pub fn from_identifier(id: &PreParserIdentifier) -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::IdentifierExpression) | IdentifierTypeField::encode(id.type_),
            }
        }

        pub fn assignment() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::Assignment),
            }
        }

        pub fn object_literal() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::ArrayOrObjectLiteralExpression),
            }
        }

        pub fn array_literal() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::ArrayOrObjectLiteralExpression),
            }
        }

        pub fn string_literal() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::StringLiteralExpression),
            }
        }

        pub fn this() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::ThisExpression),
            }
        }

        pub fn this_private_reference() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::ThisPrivateReferenceExpression),
            }
        }

        pub fn this_property() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::ThisPropertyExpression),
            }
        }

        pub fn property() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::PropertyExpression),
            }
        }

        pub fn private_reference() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::PrivateReferenceExpression),
            }
        }

        pub fn call() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::CallExpression),
            }
        }

        pub fn call_eval() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::CallEvalExpression),
            }
        }

        pub fn import_call() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::ImportCallExpression),
            }
        }

        pub fn super_call_reference() -> Self {
            PreParserExpression {
                code_: TypeField::encode(ExpressionType::Expression) | ExpressionTypeField::encode(InnerExpressionType::SuperCallReference),
            }
        }

        pub fn is_null(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Null
        }
        pub fn is_failure_expression(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Failure
        }

        pub fn is_identifier(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::IdentifierExpression
        }

        pub fn as_identifier(&self) -> PreParserIdentifier {
            if !self.is_identifier() {
                panic!("DCHECK(IsIdentifier()) failed");
            }
            PreParserIdentifier {
                string_: null(), //This is problematic, requires proper implementation
                type_: IdentifierTypeField::decode(self.code_),
            }
        }

        pub fn is_assignment(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression && ExpressionTypeField::decode(self.code_) == InnerExpressionType::Assignment
        }

        pub fn is_pattern(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::ArrayOrObjectLiteralExpression
        }

        pub fn is_string_literal(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::StringLiteralExpression
        }

        pub fn is_this(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression && ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisExpression
        }

        pub fn is_this_property(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression
                && (ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisPropertyExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisPrivateReferenceExpression)
        }

        pub fn is_property(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression
                && (ExpressionTypeField::decode(self.code_) == InnerExpressionType::PropertyExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisPropertyExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::PrivateReferenceExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisPrivateReferenceExpression)
        }

        pub fn is_private_reference(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression
                && (ExpressionTypeField::decode(self.code_) == InnerExpressionType::PrivateReferenceExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::ThisPrivateReferenceExpression)
        }

        pub fn is_call(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression
                && (ExpressionTypeField::decode(self.code_) == InnerExpressionType::CallExpression
                || ExpressionTypeField::decode(self.code_) == InnerExpressionType::CallEvalExpression)
        }

        pub fn is_super_call_reference(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression && ExpressionTypeField::decode(self.code_) == InnerExpressionType::SuperCallReference
        }

        pub fn is_import_call_expression(&self) -> bool {
            TypeField::decode(self.code_) == ExpressionType::Expression && ExpressionTypeField::decode(self.code_) == InnerExpressionType::ImportCallExpression
        }

        // At the moment PreParser doesn't track these expression types.
        pub fn is_function_literal(&self) -> bool {
            false
        }
        pub fn is_call_new(&self) -> bool {
            false
        }
        pub fn is_tagged_template(&self) -> bool {
            false
        }

        pub fn is_parenthesized(&self) -> bool {
            IsParenthesizedField::decode(self.code_)
        }

        pub fn mark_parenthesized(&mut self) {
            self.code_ = IsParenthesizedField::update(self.code_, true);
        }

        pub fn clear_parenthesized(&mut self) {
            self.code_ = IsParenthesizedField::update(self.code_, false);
        }

        pub fn as_call(&mut self) -> &mut Self {
            self
        }
        pub fn as_function_literal(&mut self) -> &mut Self {
            self
        }

        // Dummy implementation for making expression->somefunc() work in both Parser
        // and PreParser.
        pub fn operator_arrow(&mut self) -> &mut Self {
            self
        }

        // More dummy implementations of things PreParser doesn't need to track:
        pub fn set_should_eager_compile(&mut self) {}

        pub fn position(&self) -> i32 {
            kNoSourcePosition
        }
        pub fn set_function_token_position(&mut self, _position: i32) {}
        pub fn set_suspend_count(&mut self, _suspend_count: i32) {}
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum ExpressionType {
        Null,
        Failure,
        Expression,
        IdentifierExpression,
        StringLiteralExpression,
        ArrayOrObjectLiteralExpression,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum InnerExpressionType {
        ThisExpression,
        ThisPropertyExpression,
        ThisPrivateReferenceExpression,
        PropertyExpression,
        PrivateReferenceExpression,
        CallExpression,
        CallEvalExpression,
        SuperCallReference,
        Assignment,
        ImportCallExpression,
    }

    type TypeField = crate::base::BitField<ExpressionType, 0, 3>;
    type IsParenthesizedField = TypeField::next::<bool, 1>;
    type ExpressionTypeField = IsParenthesizedField::next::<InnerExpressionType, 4>;
    type IdentifierTypeField = IsParenthesizedField::next::<PreParserIdentifierType, 8>;
    type HasCoverInitializedNameField = IsParenthesizedField::next::<bool, 1>;

    pub struct PreParserStatement;

    impl PreParserStatement {
        pub fn default() -> Self {
            PreParserStatement
        }
    }

    pub struct PreParserScopedStatementList {
        _buffer: *mut Vec<*mut std::ffi::c_void>,
        // _marker: PhantomData<T>
    }

    impl PreParserScopedStatementList {
        pub fn new(_buffer: *mut Vec<*mut std::ffi::c_void>) -> Self {
            PreParserScopedStatementList {
                _buffer,
                // _marker: PhantomData,
            }
        }

        pub fn rewind(&mut self) {}
        pub fn merge_into(&mut self, _other: &PreParserScopedStatementList) {}
        pub fn add(&mut self, _element: &PreParserStatement) {}
        pub fn length(&self) -> i32 {
            0
        }
    }

    pub struct PreParserExpressionList {
        length_: i32,
    }

    impl PreParserExpressionList {
        pub fn new(_buffer: *mut Vec<*mut std::ffi::c_void>) -> Self {
            PreParserExpressionList { length_: 0 }
        }

        pub fn length(&self) -> i32 {
            self.length_
        }

        pub fn add(&mut self, _expression: &PreParserExpression) {
            self.length_ += 1;
        }
    }

    pub struct AstRawString {}

    pub struct PreparseDataBuilder {}

    pub struct V8FileLogger {}

    pub const kNoSourcePosition: i32 = -1;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TokenValue {
        String,
        Assign,
        Init,
        RightBrace,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ForEachStatementVisitMode {
        kNormal,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IteratorType {
        kSync,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ModuleImportPhase {
        kSync,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FunctionKind {
        kNormalFunction,
        kGetterFunction,
        kSetterFunction,
        kStaticGetterFunction,
        kStaticSetterFunction,
        kDefaultDerivedConstructor,
        kDefaultBaseConstructor,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FunctionSyntaxKind {
        kNormal,
        kNamedExpression,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum LanguageMode {
        kStrict,
        kSloppy,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SuspendOnAbruptResume {
        kTrue,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SourceRange {}

    pub struct DeclarationParsingResult {
        pub descriptor: Descriptor,
    }

    pub struct Descriptor {
        pub mode: VariableMode,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum VariableMode {
        kVar,
        kLet,
        kConst,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum VariableKind {
        NORMAL_VARIABLE,
        SLOPPY_BLOCK_FUNCTION_VARIABLE,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum InitializationFlag {
        kNeedsInitialization,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MessageTemplate {
        kVarRedeclaration,
        kUnexpectedToken,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IsStaticFlag {
        kStatic,
        kNotStatic,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum InferName {
        kYes,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct UnoptimizedCompileFlags {}

    pub struct ProducedPreparseData {}

    pub struct ClassInfo {
        pub has_static_elements: bool,
        pub has_seen_constructor: bool,
        pub extends: PreParserExpression,
        pub computed_field_count: i32,
        pub autoaccessor_count: i32,
    }

    impl ClassInfo {
        pub fn new() -> Self {
            ClassInfo {
                has_static_elements: false,
                has_seen_constructor: false,
                extends: PreParserExpression::null_(),
                computed_field_count: 0,
                autoaccessor_count: 0,
            }
        }
    }

    pub struct ScannerLocation {}

    impl ScannerLocation {
        pub fn is_valid(&self) -> bool {
            false
        }
    }

    pub struct Scanner {
        parser_error: bool,
    }

    impl Scanner {
        pub fn new() -> Self {
            Scanner { parser_error: false }
        }

        pub fn set_parser_error(&mut self) {
            self.parser_error = true;
        }
    }

    pub struct PendingCompilationErrorHandler {
        unidentifiable_error: bool,
    }

    impl PendingCompilationErrorHandler {
        pub fn new() -> Self {
            PendingCompilationErrorHandler {
                unidentifiable_error: false,
            }
        }

        pub fn set_unidentifiable_error(&mut self) {
            self.unidentifiable_error = true;
        }
    }

    pub struct RuntimeCallStats {}

    pub struct ClassLiteralProperty {
        kind: ClassLiteralPropertyKind,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ClassLiteralPropertyKind {
        AUTO_ACCESSOR,
    }

    pub struct ObjectLiteralProperty {
        kind: ObjectLiteralPropertyKind,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ObjectLiteralPropertyKind {
        NORMAL,
    }

    pub struct ForInfo {
        pub parsing_result: DeclarationParsingResult,
        pub bound_names: Vec<*const AstRawString>,
    }

    pub struct CatchInfo {}

    pub struct PrivateNameScopeIterator {}

    pub struct ProducedPreparseDataBuilder {}

    pub struct FormalParametersBase {
        scope: *mut DeclarationScope,
    }

    impl FormalParametersBase {
        pub fn new(scope: *mut DeclarationScope) -> Self {
            FormalParametersBase { scope }
        }
    }

    pub struct FunctionLiteral {
        eager_compile_hint: EagerCompileHint,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EagerCompileHint {
        kLazy,
    }

    pub struct Declaration {}

    pub struct AstNodeFactory {}

    impl AstNodeFactory {
        pub fn new() -> Self {
            AstNodeFactory {}
        }

        pub fn new_nested_variable_declaration(
            &self,
            _scope: *mut Scope,
            _position: i32,
        ) -> *mut Declaration {
            std::ptr::null_mut()
        }
    }

    pub struct SloppyBlockFunctionStatement {}

    impl AstValueFactory {
        pub fn new() -> Self {
            AstValueFactory {}
        }

        pub fn empty_string(&self) -> *const AstRawString {
            std::ptr::null()
        }
        pub fn this_function_string(&self) -> *const AstRawString {
            std::ptr::null()
        }
        pub fn new_target_string(&self) -> *const AstRawString {
            std::ptr::null()
        }
    }

    pub struct PreParserFormalParameters {
        scope: *mut DeclarationScope,
        has_duplicate_: bool,
        strict_parameter_error_: bool,
        is_simple: bool,
        parameter_count: i32,
        function_length: i32,
    }

    impl PreParserFormalParameters {
        pub fn new(scope: *mut DeclarationScope) -> Self {
            PreParserFormalParameters {
                scope,
                has_duplicate_: false,
                strict_parameter_error_: false,
                is_simple: false,
                parameter_count: 0,
                function_length: 0,
            }
        }

        pub fn set_has_duplicate(&mut self) {
            self.has_duplicate_ = true;
        }
        pub fn has_duplicate(&self) -> bool {
            self.has_duplicate_
        }
        pub fn validate_duplicate(&self, _preparser: &PreParser) {}

        pub fn set_strict_parameter_error(&mut self, _loc: &ScannerLocation, _message: MessageTemplate) {
            self.strict_parameter_error_ = _loc.is_valid();
        }
        pub fn validate_strict_mode(&self, _preparser: &PreParser) {}

        pub fn update_arity_and_function_length(&mut self, has_initializer: bool, is_rest: bool) {
            if is_rest {
                self.function_length = -1;
            } else {
                self.parameter_count += 1;
                if !has_initializer && self.function_length != -1 {
                    self.function_length += 1;
                } else {
                    self.function_length = -1;
                }
            }
        }
    }

    pub struct PreParserFuncNameInferrer {}

    impl PreParserFuncNameInferrer {
        pub fn new(_avf: *mut AstValueFactory) -> Self {
            PreParserFuncNameInferrer {}
        }
    }

    pub struct PreParserFactory {
        ast_node_factory_: AstNodeFactory,
    }

    impl PreParserFactory {
        pub fn new(_ast_value_factory: *mut AstValueFactory, _zone: *mut std::ffi::c_void) -> Self {
            PreParserFactory {
                ast_node_factory_: AstNodeFactory::new(),
            }
        }

        pub fn ast_node_factory(&mut self) -> &mut AstNodeFactory {
            &mut self.ast_node_factory_
        }

        pub fn new_string_literal(&self, _identifier: &PreParserIdentifier, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_number_literal(&self, _number: f64, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_undefined_literal(&self, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_the_hole_literal(&self) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_reg_exp_literal(&self, _js_pattern: *const AstRawString, _js_flags: i32, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_array_literal(&self, _values: &PreParserExpressionList, _first_spread_index: i32, _pos: i32) -> PreParserExpression {
            PreParserExpression::array_literal()
        }
        pub fn new_class_literal_property(
            &self,
            _key: &PreParserExpression,
            _value: &PreParserExpression,
            _kind: ClassLiteralPropertyKind,
            _is_static: bool,
            _is_computed_name: bool,
            _is_private: bool,
        ) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_object_literal_property(
            &self,
            _key: &PreParserExpression,
            _value: &PreParserExpression,
            _kind: ObjectLiteralPropertyKind,
            _is_computed_name: bool,
        ) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_object_literal_property1(
            &self,
            _key: &PreParserExpression,
            _value: &PreParserExpression,
            _is_computed_name: bool,
        ) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_object_literal(
            &self,
            _properties: &PreParserExpressionList,
            _boilerplate_properties: i32,
            _pos: i32,
            _has_rest_property: bool,
            _home_object: *mut Variable,
        ) -> PreParserExpression {
            PreParserExpression::object_literal()
        }
        pub fn new_variable_proxy(&self, _variable: *mut std::ffi::c_void) -> PreParserExpression {
            PreParserExpression::default()
        }

        pub fn new_optional_chain(&self, expr: &PreParserExpression) -> PreParserExpression {
            // Needed to track `delete a?.#b` early errors
            if expr.is_private_reference() {
                return PreParserExpression::private_reference();
            }
            PreParserExpression::default()
        }

        pub fn new_property(&self, obj: &PreParserExpression, key: &PreParserExpression, _pos: i32, _optional_chain: bool) -> PreParserExpression {
            if key.is_identifier() && key.as_identifier().is_private_name() {
                if obj.is_this() {
                    return PreParserExpression::this_private_reference();
                }
                return PreParserExpression::private_reference();
            }

            if obj.is_this() {
                return PreParserExpression::this_property();
            }
            PreParserExpression::property()
        }
        pub fn new_unary_operation(&self, _op: TokenValue, _expression: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_binary_operation(&self, _op: TokenValue, _left: &PreParserExpression, _right: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_compare_operation(&self, _op: TokenValue, _left: &PreParserExpression, _right: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_assignment(&self, _op: TokenValue, _left: &PreParserExpression, _right: &PreParserExpression, _pos: i32) -> PreParserExpression {
            // Identifiers need to be tracked since this might be a parameter with a
            // default value inside an arrow function parameter list.
            PreParserExpression::assignment()
        }
        pub fn new_yield(&self, _expression: &PreParserExpression, _pos: i32, _on_abrupt_resume: SuspendOnAbruptResume) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_await(&self, _expression: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_yield_star(&self, _iterable: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_conditional_chain(&self, _initial_size: usize, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_conditional(
            &self,
            _condition: &PreParserExpression,
            _then_expression: &PreParserExpression,
            _else_expression: &PreParserExpression,
            _pos: i32,
        ) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_count_operation(&self, _op: TokenValue, _is_prefix: bool, _expression: &PreParserExpression, _pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_call(
            &self,
            expression: PreParserExpression,
            _arguments: &PreParserExpressionList,
            _pos: i32,
            _has_spread: bool,
            eval_scope_info_index: i32,
            _optional_chain: bool,
        ) -> PreParserExpression {
            if eval_scope_info_index > 0 {
                if !expression.is_identifier() || !expression.as_identifier().is_eval() {
                    panic!("DCHECK(expression.IsIdentifier() && expression.AsIdentifier().IsEval()) failed");
                }
                if _optional_chain {
                    panic!("DCHECK(!optional_chain) failed");
                }
                return PreParserExpression::call_eval();
            }
            PreParserExpression::call()
        }
        pub fn new_call_new(&self, _expression: &PreParserExpression, _arguments: &PreParserExpressionList, _pos: i32, _has_spread: bool) -> PreParserExpression {
            PreParserExpression::default()
        }
        pub fn new_return_statement(&self, _expression: &PreParserExpression, _pos: i32, _continuation_pos: i32) -> PreParserStatement {
            PreParserStatement::default()
        }
        pub fn new_async_return_statement(&self, _expression: &PreParserExpression, _pos: i32, _continuation_pos: i32) -> PreParserStatement {
            PreParserStatement::default()
        }
        pub fn new_function_literal(
            &self,
            _name: &PreParserIdentifier,
            _scope: *mut Scope,
            _body: &PreParserScopedStatementList,
            _expected_property_count: i32,
            _parameter_count: i32,
            _function_length: i32,
            _has_duplicate_parameters: FunctionLiteral::ParameterFlag,
            _function_syntax_kind: FunctionSyntaxKind,
            _eager_compile_hint: EagerCompileHint,
            _position: i32,
            _has_braces: bool,
            _function_literal_id: i32,
            _produced_preparse_data: *mut *mut ProducedPreparseData,
        ) -> PreParserExpression {
            PreParserExpression::default()
        }

        pub fn new_spread(&self, _expression: &PreParserExpression, _pos: i32, _expr_pos: i32) -> PreParserExpression {
            PreParserExpression::default()
        }

        pub fn new_empty_parentheses(&self, _pos: i32) -> PreParserExpression {
            let mut result = PreParserExpression::default();
            result.mark_parenthesized();
            result
        }

        pub fn empty_statement(&self) -> PreParserStatement {
            PreParserStatement::default()
        }

        pub fn new_block(&self, _capacity: i32, _ignore_completion_value: bool) -> PreParserStatement {
            PreParserStatement::default()
        }

        pub fn new_block1(&self, _ignore_completion_value: bool, _is_breakable: bool) -> PreParserStatement {
            PreParserStatement::default()
        }

        pub fn new_block2(&self, _ignore_completion_value: bool, _list: &PreParserScopedStatementList) -> PreParserStatement {
            PreParserStatement::default()
        }

        pub fn new_debugger_statement(&self, _pos: i32) -> PreParserStatement {
            PreParserStatement::default()
        }

        pub fn new_expression_statement(&self, expr: &PreParserExpression, _pos: i32) -> PreParserStatement {
            //PreParserStatement::ExpressionStatement(expr);
            PreParserStatement::default()
        }

        pub fn new_if_statement
// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![allow(dead_code)] // Allow unused code during conversion
// #![allow(unused_variables)] // Allow unused variables during conversion
// #![allow(unused_imports)] // Allow unused imports during conversion

// use std::any::Any;
// use std::collections::{HashMap, HashSet};
// use std::fmt;
// use std::rc::Rc;
// use std::sync::Arc;

// use crate::flags; // Assuming a flags module exists

// pub mod ast;
// pub mod constants;
// pub mod declarations;
// pub mod earley_parser;
// pub mod global_context;
// pub mod utils;

// Placeholder enums and structs
#[derive(Debug, Clone)]
pub struct SourcePosition;

impl SourcePosition {
    pub fn invalid() -> Self {
        SourcePosition {}
    }
}

#[derive(Debug, Clone)]
pub struct InputPosition;

impl InputPosition {
    pub fn new() -> Self {
        InputPosition {}
    }
}

impl std::ops::AddAssign for InputPosition {
    fn add_assign(&mut self, _rhs: Self::Output) {
    }
}

impl InputPosition {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        InputPosition {}
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
    pub pos: SourcePosition,
}

#[derive(Debug, Clone)]
pub struct TypeExpression;

#[derive(Debug, Clone)]
pub struct Statement;

#[derive(Debug, Clone)]
pub struct Expression;

#[derive(Debug, Clone)]
pub struct Declaration;

#[derive(Debug, Clone)]
pub struct TryHandler;

impl TryHandler {
    pub fn new() -> Self {
        TryHandler {}
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HandlerKind {
      kCatch,
      kLabel,
    }

  pub handler_kind: TryHandler::HandlerKind,
}

#[derive(Debug, Clone)]
pub struct ParameterList;

impl ParameterList {
    pub fn empty() -> Self {
        ParameterList {}
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression;

impl CallExpression {
    pub fn cast(_expression: &Expression) -> &Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct CallMethodExpression;

#[derive(Debug, Clone)]
pub struct FieldAccessExpression;

#[derive(Debug, Clone)]
pub struct ElementAccessExpression;

#[derive(Debug, Clone)]
pub struct DereferenceExpression;

#[derive(Debug, Clone)]
pub struct StructExpression;

#[derive(Debug, Clone)]
pub struct AssignmentExpression;

#[derive(Debug, Clone)]
pub struct FloatingPointLiteralExpression;

#[derive(Debug, Clone)]
pub struct IntegerLiteralExpression;

#[derive(Debug, Clone)]
pub struct StringLiteralExpression;

#[derive(Debug, Clone)]
pub struct IncrementDecrementExpression;

#[derive(Debug, Clone)]
pub struct LogicalOrExpression;

#[derive(Debug, Clone)]
pub struct LogicalAndExpression;

#[derive(Debug, Clone)]
pub struct ConditionalExpression;

#[derive(Debug, Clone)]
pub struct GenericParameter;

#[derive(Debug, Clone)]
pub struct TorqueMacroDeclaration;

#[derive(Debug, Clone)]
pub struct SpecializationDeclaration;

#[derive(Debug, Clone)]
pub struct StatementExpression;

#[derive(Debug, Clone)]
pub struct AbstractTypeDeclaration;

#[derive(Debug, Clone)]
pub struct CppIncludeDeclaration;

#[derive(Debug, Clone)]
pub struct ExternalBuiltinDeclaration;

#[derive(Debug, Clone)]
pub struct ExternalRuntimeDeclaration;

#[derive(Debug, Clone)]
pub struct BasicTypeExpression;

impl BasicTypeExpression {
    pub fn dynamic_cast(_type: &TypeExpression) -> Option<&Self> {
      None
    }

    pub fn new(_namespace_qualification: std::vec::Vec<String>, _name: Identifier, _generic_arguments: std::vec::Vec<TypeExpression>) -> Self {
        BasicTypeExpression {}
    }

    pub fn new_from_identifier(_name: Identifier) -> Self {
      BasicTypeExpression {}
    }
}

#[derive(Debug, Clone)]
pub struct FunctionTypeExpression;

#[derive(Debug, Clone)]
pub struct UnionTypeExpression;

#[derive(Debug, Clone)]
pub struct VarDeclarationStatement;

#[derive(Debug, Clone)]
pub struct BreakStatement;

#[derive(Debug, Clone)]
pub struct ContinueStatement;

#[derive(Debug, Clone)]
pub struct GotoStatement;

#[derive(Debug, Clone)]
pub struct ForLoopStatement;

#[derive(Debug, Clone)]
pub struct EnumDescription {
  pos: SourcePosition,
  name: String,
  constexpr_generates: String,
  is_open: bool,
  entries: Vec<EnumEntryDescription>
}

#[derive(Debug, Clone)]
pub struct EnumEntryDescription {
  name: String,
  alias: String
}

#[derive(Debug, Clone)]
pub struct IntrinsicCallExpression;

#[derive(Debug, Clone)]
pub struct NewExpression;

#[derive(Debug, Clone)]
pub struct SpreadExpression;

#[derive(Debug, Clone)]
pub struct TorqueBuiltinDeclaration;

#[derive(Debug, Clone)]
pub struct ClassBody;

#[derive(Debug, Clone)]
pub struct ClassDeclaration;

#[derive(Debug, Clone)]
pub struct NamespaceDeclaration;

#[derive(Debug, Clone)]
pub struct StructDeclaration;

#[derive(Debug, Clone)]
pub struct BitFieldStructDeclaration;

#[derive(Debug, Clone)]
pub struct ExternConstDeclaration;

#[derive(Debug, Clone)]
pub struct TypeAliasDeclaration;

#[derive(Debug, Clone)]
pub struct GenericCallableDeclaration;

#[derive(Debug, Clone)]
pub struct GenericTypeDeclaration;

#[derive(Debug, Clone)]
pub struct ExternalMacroDeclaration;

#[derive(Debug, Clone)]
pub struct IntrinsicDeclaration;

#[derive(Debug, Clone)]
pub struct AssertStatement;

#[derive(Debug, Clone)]
pub struct DebugStatement;

#[derive(Debug, Clone)]
pub struct ExpressionStatement;

#[derive(Debug, Clone)]
pub struct IfStatement;

#[derive(Debug, Clone)]
pub struct WhileStatement;

#[derive(Debug, Clone)]
pub struct ReturnStatement;

#[derive(Debug, Clone)]
pub struct TailCallStatement;

#[derive(Debug, Clone)]
pub struct BlockStatement {
  pub deferred: bool,
  pub statements: Vec<Statement*>
}

impl BlockStatement {
  pub fn dynamic_cast(statement: &Statement) -> Option<&Self> {
    None
  }
}

#[derive(Debug, Clone)]
pub struct TryLabelExpression;

#[derive(Debug, Clone)]
pub struct AssumeTypeImpossibleExpression;

#[derive(Debug, Clone)]
pub struct ClassFieldExpression {
  pub conditions: Vec<ConditionalAnnotation>
}

#[derive(Debug, Clone)]
pub struct ClassFieldIndexInfo;

#[derive(Debug, Clone)]
pub struct StructFieldExpression;

#[derive(Debug, Clone)]
pub struct BitFieldDeclaration;

#[derive(Debug, Clone)]
pub struct ImplicitParameters {
    pub kind: Identifier*,
    pub parameters: std::vec::Vec<NameAndTypeExpression>
}

#[derive(Debug, Clone)]
pub struct NameAndTypeExpression;

#[derive(Debug, Clone)]
pub struct NameAndExpression;

#[derive(Debug, Clone)]
pub struct Annotation;

#[derive(Debug, Clone)]
pub struct AnnotationParameter {
    pub string_value: String,
    pub int_value: i32,
    pub is_int: bool
}

#[derive(Debug, Clone)]
pub struct LabelAndTypes;

#[derive(Debug, Clone)]
pub struct IntegerLiteral;

impl IntegerLiteral {
    pub fn new(value: i32) -> Self {
        IntegerLiteral {}
    }
    pub fn new_64(_negative: bool, _absolute_value: u64) -> Self {
        IntegerLiteral {}
    }
}

#[derive(Debug, Clone)]
pub struct GenericParameters {
    pub params: Vec<GenericParameter>
}

#[derive(Debug, Clone)]
pub struct LabelAndTypesVector;

#[derive(Debug, Clone)]
pub struct TypeList;

#[derive(Debug, Clone)]
pub struct TryHandlerPtr;

#[derive(Debug, Clone)]
pub struct StatementPtr;

#[derive(Debug, Clone)]
pub struct DeclarationPtr;

#[derive(Debug, Clone)]
pub struct IdentifierPtr;

#[derive(Debug, Clone)]
pub struct OptionalIdentifierPtr;

#[derive(Debug, Clone)]
pub struct ExpressionPtr;

#[derive(Debug, Clone)]
pub struct NameAndTypeExpressionVector;

#[derive(Debug, Clone)]
pub struct ExpressionWithSource;

#[derive(Debug, Clone)]
pub struct EnumEntry;

#[derive(Debug, Clone)]
pub struct NameAndExpressionVector;

#[derive(Debug, Clone)]
pub struct AnnotationVector;

#[derive(Debug, Clone)]
pub struct AnnotationParameterVector;

#[derive(Debug, Clone)]
pub struct ClassFieldExpressionVector;

#[derive(Debug, Clone)]
pub struct StructFieldExpressionVector;

#[derive(Debug, Clone)]
pub struct BitFieldDeclarationVector;

#[derive(Debug, Clone)]
pub struct IncrementDecrementOperator;

#[derive(Debug, Clone)]
pub struct StatementPtrVector;

#[derive(Debug, Clone)]
pub struct DeclarationPtrVector;

#[derive(Debug, Clone)]
pub struct ExpressionPtrVector;

#[derive(Debug, Clone)]
pub struct LabelAndTypesVectorVector;

#[derive(Debug, Clone)]
pub struct OptionalStatementPtr;

#[derive(Debug, Clone)]
pub struct OptionalExpressionPtr;

#[derive(Debug, Clone)]
pub struct TypeswitchCase;

#[derive(Debug, Clone)]
pub struct TypeswitchCaseVector;

#[derive(Debug, Clone)]
pub struct IdentifierPtrVector;

#[derive(Debug, Clone)]
pub struct OptionalClassBody;

#[derive(Debug, Clone)]
pub struct GenericParameterType;

#[derive(Debug, Clone)]
pub struct GenericParametersType;

#[derive(Debug, Clone)]
pub struct ImplicitKind;

#[derive(Debug, Clone)]
pub struct AbstractTypeFlags;

impl AbstractTypeFlags {
    pub fn new() -> Self {
      AbstractTypeFlags {}
    }
}

#[derive(Debug, Clone)]
pub struct ClassFlags;

impl ClassFlags {
    pub fn new() -> Self {
      ClassFlags {}
    }
}

#[derive(Debug, Clone)]
pub struct StructFlags;

impl StructFlags {
    pub fn new() -> Self {
      StructFlags {}
    }
}

#[derive(Debug, Clone)]
pub struct FieldSynchronization;

#[derive(Debug, Clone)]
pub struct ConditionalAnnotation;

#[derive(Debug, Clone)]
pub struct ConditionalAnnotationType;

#[derive(Debug, Clone)]
pub struct InstanceTypeConstraints;

#[derive(Debug, Clone)]
pub enum ParseResultTypeId {
  kStdString,
  kBool,
  kInt32,
  kDouble,
  kIntegerLiteral,
  kStdVectorOfString,
  kDeclarationPtr,
  kTypeExpressionPtr,
  kOptionalTypeExpressionPtr,
  kTryHandlerPtr,
  kExpressionPtr,
  kIdentifierPtr,
  kOptionalIdentifierPtr,
  kStatementPtr,
  kNameAndTypeExpression,
  kEnumEntry,
  kStdVectorOfEnumEntry,
  kNameAndExpression,
  kAnnotation,
  kVectorOfAnnotation,
  kAnnotationParameter,
  kOptionalAnnotationParameter,
  kClassFieldExpression,
  kStructFieldExpression,
  kBitFieldDeclaration,
  kStdVectorOfNameAndTypeExpression,
  kImplicitParameters,
  kOptionalImplicitParameters,
  kStdVectorOfNameAndExpression,
  kStdVectorOfClassFieldExpression,
  kStdVectorOfStructFieldExpression,
  kStdVectorOfBitFieldDeclaration,
  kIncrementDecrementOperator,
  kOptionalStdString,
  kStdVectorOfStatementPtr,
  kStdVectorOfDeclarationPtr,
  kStdVectorOfStdVectorOfDeclarationPtr,
  kStdVectorOfExpressionPtr,
  kExpressionWithSource,
  kParameterList,
  kTypeList,
  kOptionalTypeList,
  kLabelAndTypes,
  kStdVectorOfLabelAndTypes,
  kStdVectorOfTryHandlerPtr,
  kOptionalStatementPtr,
  kOptionalExpressionPtr,
  kTypeswitchCase,
  kStdVectorOfTypeswitchCase,
  kStdVectorOfIdentifierPtr,
  kOptionalClassBody,
  kGenericParameter,
  kGenericParameters,
}

// Placeholder struct for ParseResult
#[derive(Debug)]
pub struct ParseResult<T> {
  pub result: T
}

impl<T> ParseResult<Vec<T>> {
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.result.into_iter()
    }
}

#[derive(Debug)]
pub struct ParseResultNone {}

impl ParseResultNone {
  pub fn new() -> Self {
    ParseResultNone {}
  }
}

// Placeholder struct for ParseResultHolder
pub struct ParseResultHolder<T> {
    data: T,
    id: ParseResultTypeId
}

impl<T> ParseResultHolder<T> {
    pub fn new(data: T, id: ParseResultTypeId) -> Self {
        ParseResultHolder { data, id }
    }
}

// Placeholder struct for ParseResultIterator
pub struct ParseResultIterator<'a> {
    results: Vec<&'a ParseResultHolder<dyn std::any::Any>>,
    index: usize,
    pub matched_input: MatchedInput
}

impl<'a> ParseResultIterator<'a> {
  pub fn new() -> Self {
    ParseResultIterator {
      results: Vec::new(),
      index: 0,
      matched_input: MatchedInput {
        pos: SourcePosition {}
      }
    }
  }

    pub fn next_as<T: 'static>(&mut self) -> T
    where T: Default + Clone {
        if self.index >= self.results.len() {
            panic!("ParseResultIterator::NextAs out of bounds");
        }
        let result = &self.results[self.index];
        self.index += 1;
        let any = &result.data;

        if let Some(data) = any.downcast_ref::<T>() {
            data.clone()
        } else {
            println!("Expected TypeId: {:?}", std::any::TypeId::of::<T>());
            panic!(
                "ParseResultIterator::NextAs failed to downcast to {}",
                std::any::type_name::<T>()
            );
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.results.len()
    }
}

// Placeholder struct for Grammar
pub struct Grammar {
    file: *mut Symbol, // Placeholder type
    whitespace: fn(&mut InputPosition) -> bool,
}

impl Grammar {
    pub fn new(_file: *mut Symbol) -> Self {
        Grammar {
            file: _file,
            whitespace: |_| true,
        }
    }
    pub fn set_whitespace(&mut self, whitespace: fn(&mut InputPosition) -> bool) {
        self.whitespace = whitespace;
    }
}

// Placeholder struct for Rule
pub struct Rule {
    symbols: Vec<Symbol>, // Placeholder type
    action: fn(&mut ParseResultIterator) -> Option<ParseResult<dyn std::any::Any>>,
}

impl Rule {
    pub fn new(_symbols: Vec<Symbol>, _action: fn(&mut ParseResultIterator) -> Option<ParseResult<dyn std::any::Any>>) -> Self {
        Rule {
            symbols: _symbols,
            action: _action,
        }
    }
}

// Placeholder struct for Symbol
#[derive(Clone)]
pub struct Symbol {
    rules: Vec<Rule>, // Placeholder type
}

impl Symbol {
    pub fn new() -> Self {
        Symbol {
            rules: Vec::new(),
        }
    }
}

// Placeholder functions

pub fn match_char<F: Fn(char) -> bool>(predicate: F, pos: &mut InputPosition) -> bool {
    // Placeholder implementation
    true
}

pub fn match_string(s: &str, pos: &mut InputPosition) -> bool {
    // Placeholder implementation
    true
}

pub fn match_any_char(pos: &mut InputPosition) -> bool {
    // Placeholder implementation
    true
}

pub struct MatchedInput {
  pub pos: SourcePosition
}

impl MatchedInput {
  pub fn to_string(&self) -> String {
    String::from("")
  }
}

pub fn yield_matched_input(child_results: &mut ParseResultIterator) -> Option<ParseResult<String>> {
    // Placeholder implementation
    Some(ParseResult { result: String::from("") })
}

pub fn new_symbol() -> *mut Symbol {
    // Placeholder implementation
    std::ptr::null_mut()
}

pub fn sequence<T>(symbol: T) -> T {
    symbol
}

pub fn optional<T>(symbol: T) -> T {
    symbol
}

pub fn one_of<T>(symbol: T) -> T {
    symbol
}

pub fn try_or_default<T>(symbol: T) -> T {
    symbol
}

pub fn check_if<T>(symbol: T) -> T {
    symbol
}

pub fn list<T>(element: &Symbol, separator: &Symbol) -> &Symbol {
    element
}

// Torque code implementations
type TypeList = Vec<TypeExpression*>;

struct ExpressionWithSource {
  expression: Expression*,
  source: String,
}

struct TypeswitchCase {
  pos: SourcePosition,
  name: Option<Identifier*>,
  type_: TypeExpression*,
  block: Statement*,
}

struct EnumEntry {
  name: Identifier*,
  type_: Option<TypeExpression*>,
  alias_entry: Option<String>,
}

struct BuildFlags {
  build_flags_: std::collections::HashMap<String, bool>,
}

impl BuildFlags {
  fn new() -> Self {
    let mut build_flags_ = std::collections::HashMap::new();
    build_flags_.insert("V8_EXTERNAL_CODE_SPACE".to_string(), false); // Replace with actual value
    build_flags_.insert("TAGGED_SIZE_8_BYTES".to_string(), std::mem::size_of::<usize>() == 8);
    build_flags_.insert("V8_ENABLE_EXPERIMENTAL_UNDEFINED_DOUBLE".to_string(), false); // Replace with actual value
    build_flags_.insert("V8_INTL_SUPPORT".to_string(), false); // conditionally based on feature
    build_flags_.insert("V8_ENABLE_SWISS_NAME_DICTIONARY".to_string(), false); // Replace with actual value
    build_flags_.insert("V8_ENABLE_JAVASCRIPT_PROMISE_HOOKS".to_string(), false); // conditionally based on feature
    build_flags_.insert("V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA".to_string(), false); // conditionally based on feature
    build_flags_.insert("TRUE_FOR_TESTING".to_string(), true);
    build_flags_.insert("FALSE_FOR_TESTING".to_string(), false);
    build_flags_.insert("V8_SCRIPTORMODULE_LEGACY_LIFETIME".to_string(), false);
    build_flags_.insert("V8_ENABLE_WEBASSEMBLY".to_string(), false);
    build_flags_.insert("WASM_CODE_POINTER_NEEDS_PADDING".to_string(), std::mem::size_of::<usize>() == 8);
    build_flags_.insert("V8_ENABLE_SANDBOX".to_string(), false);
    build_flags_.insert("V8_ENABLE_LEAPTIERING".to_string(), false);
    build_flags_.insert("V8_ENABLE_LEAPTIERING_TAGGED_SIZE_8_BYTES".to_string(), false);
    build_flags_.insert("DEBUG".to_string(), false);
    build_flags_.insert("V8_ENABLE_DRUMBRAKE".to_string(), false);

    BuildFlags { build_flags_ }
  }

  fn get_flag(&self, name: &str, production: &str) -> bool {
    match self.build_flags_.get(name) {
      Some(&value) => value,
      None => {
        report_error(&format!("Unknown flag used in {}: {}. Please add it to the list in BuildFlags.", production, name));
        false // Default to false, but ideally, this should panic/return a Result
      }
    }
  }
}

// Dummy report_error function
fn report_error(message: &str) {
  eprintln!("{}", message);
  // Or panic!(), or return a Result::Err, depending on desired behavior
}

// Dummy Lint function
struct LintMessage {
  message: String,
  position: Option<SourcePosition>,
}

impl LintMessage {
  fn new(message: String) -> Self {
    LintMessage {
      message,
      position: None,
    }
  }

  fn position(mut self, position: SourcePosition) -> Self {
    self.position = Some(position);
    self
  }
}

fn lint(args: &str) -> LintMessage {
    LintMessage::new(args.to_string())
}

// fn lint(args: impl fmt::Display) -> LintMessage {
//   LintMessage::new(format!("{}", args))
// }

// Dummy Error function
#[derive(Debug)]
struct TorqueError {
  message: String,
}

impl TorqueError {
    fn new(message: &str) -> Self {
        TorqueError { message: message.to_string() }
    }

    fn throw(&self) -> ! {
        panic!("{}", self.message);
    }

    fn position(self, _pos: SourcePosition) -> Self {
        self
    }
}

fn error(message: &str) -> TorqueError {
    TorqueError::new(message)
}

// Dummy is_upper_camel_case and is_snake_case functions
fn is_upper_camel_case(_name: &str) -> bool {
    true
}

fn is_snake_case(_name: &str) -> bool {
    true
}

fn is_lower_camel_case(_name: &str) -> bool {
    true
}

fn is_valid_type_name(_name: &str) -> bool {
    true
}

fn is_valid_namespace_const_name(_name: &str) -> bool {
    true
}

// Dummy CurrentAst struct and impl
struct CurrentAst {}

impl CurrentAst {
  fn new() -> Self {
    CurrentAst {}
  }

  fn get() -> Self {
    CurrentAst {}
  }

  fn declare_import_for_current_file(&self, _import_id: u32) {}

  fn add_enum_description(&self, _enum_description: EnumDescription) {}

  fn declarations(&self) -> Vec<Declaration*> {
    Vec::new()
  }
}

// Dummy SourceFileMap struct and functions
struct SourceFileMap {}

impl SourceFileMap {
  fn file_relative_to_v8_root_exists(_path: &str) -> bool {
    true
  }

  fn get_source_id(_path: &str) -> u32 {
    0
  }
}

// Dummy CurrentSourcePosition struct and function
struct CurrentSourcePosition {}

impl CurrentSourcePosition {
  fn get() -> SourcePosition {
    SourcePosition {}
  }

  struct Scope {
    pos: SourcePosition,
  }

  impl Scope {
    fn new(_pos: SourcePosition) -> Self {
        Scope {
            pos: _pos,
        }
    }
  }
}

// Dummy UnwrapTNodeTypeName function
fn unwrap_t_node_type_name(_name: String) -> String {
  _name
}

// Dummy get_constexpr_name function
fn get_constexpr_name(_name: String) -> String {
  _name
}

// Template function implementations
fn concat_list(child_results: &mut ParseResultIterator) -> Option<ParseResult<Vec<Declaration*>>> {
  let list_of_lists = child_results.next_as::<Vec<Vec<Declaration*>>>();
  let mut result = Vec::new();
  for list in list_of_lists {
    result.extend(list);
  }
  Some(ParseResult { result })
}

fn check_not_deferred_statement(statement: Statement*) {
  // let source_position = CurrentSourcePosition::Scope::new(statement.pos);
  // if let Some(block) = BlockStatement::DynamicCast(statement) {
  //   if block.deferred {
  //     lint(
  //       "cannot use deferred with a statement block here, it will have no effect"
  //     ).position(statement.pos);
  //   }
  // }
}

fn add_constexpr(type_: TypeExpression*) -> TypeExpression* {
  // if let Some(basic) = BasicTypeExpression::DynamicCast(type_) {
  //   return MakeNode<BasicTypeExpression>(
  //     basic.namespace_qualification,
  //     MakeNode<Identifier>(CONSTEXPR_TYPE_PREFIX + basic.name.value),
  //     basic.generic_arguments
  //   );
  // } else {
  //   Error("Unsupported extends clause.").Throw();
  // }
  todo!()
}

fn make_call(callee: IdentifierExpression*, target: Option<Expression*>, arguments: Vec<Expression*>, otherwise: Vec<Statement*>) -> Expression* {
  todo!()
}

fn make_method_call(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let this_arg = child_results.next_as::<Expression*>();
  let callee = child_results.next_as::<Identifier*>();
  let args = child_results.next_as::<Vec<Expression*>>();
  let otherwise = child_results.next_as::<Vec<Statement*>>();
  // return ParseResult{MakeCall(MakeNode<IdentifierExpression>(callee), this_arg,
  //                             std::move(args), otherwise)};
  todo!()
}

fn make_new_expression(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let pretenured = child_results.next_as::<bool>();
  let clear_padding = child_results.next_as::<bool>();

  let type_ = child_results.next_as::<TypeExpression*>();
  let initializers = child_results.next_as::<Vec<NameAndExpression>>();

  // Expression* result = MakeNode<NewExpression>(type, std::move(initializers),
  //                                              pretenured, clear_padding);
  // return ParseResult{result};
  todo!()
}

fn make_binary_operator(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let left = child_results.next_as::<Expression*>();
  let op = child_results.next_as::<Identifier*>();
  let right = child_results.next_as::<Expression*>();
  // return ParseResult{MakeCall(op, TypeList{},
  //                             std::vector<Expression*>{left, right},
  //                             std::vector<Statement*>{})};
  todo!()
}

fn make_intrinsic_call_expression(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let callee = child_results.next_as::<Identifier*>();
  let generic_arguments =
    child_results.next_as::<Vec<TypeExpression*>>();
  let args = child_results.next_as::<Vec<Expression*>>();
  // Expression* result = MakeNode<IntrinsicCallExpression>(
  //   callee, std::move(generic_arguments), std::move(args));
  // return ParseResult{result};
  todo!()
}

fn make_unary_operator(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let op = child_results.next_as::<Identifier*>();
  let e = child_results.next_as::<Expression*>();
  // return ParseResult{MakeCall(op, TypeList{}, std::vector<Expression*>{e},
  //                             std::vector<Statement*>{})};
  todo!()
}

fn make_spread_expression(child_results: &mut ParseResultIterator) -> Option<ParseResult<Expression*>> {
  let spreadee = child_results.next_as::<Expression*>();
  // Expression* result = MakeNode<SpreadExpression>(spreadee);
  // return ParseResult{result};
  todo!()
}

fn make_implicit_parameter_list(child_results: &mut ParseResultIterator) -> Option<ParseResult<ImplicitParameters>> {
  let kind = child_results.next_as::<Identifier*>();
  let parameters = child_results.next_as::<Vec<NameAndTypeExpression>>();
  // return ParseResult{ImplicitParameters{kind, std::move(parameters)}};
  todo!()
}

fn add_parameter(parameter_list: ParameterList*, param: &NameAndTypeExpression) {
  // if !IsLowerCamelCase(param.name.value) {
  //   NamingConventionError("Parameter", param.name, "lowerCamelCase");
  // }
  // parameter_list.names.push_back(param.name);
  // parameter_list.types.push_back(param.type);
}

fn make_parameter_list<const HAS_VARARGS: bool, const HAS_EXPLICIT_PARAMETER_NAMES: bool>(
  child_results: &mut ParseResultIterator
) -> Option<ParseResult<ParameterList>> {
  let implicit_params =
    child_results.next_as::<Option<ImplicitParameters>>();
  let mut result = ParameterList {};
  // result.has_varargs = has_varargs;
  // result.implicit_count = 0;
  // result.implicit_kind = ImplicitKind::kNoImplicit;
  // if implicit_params {
  //   result.implicit_count = implicit_params.parameters.size();
  //   if implicit_params.kind.value == "implicit" {
  //     result.implicit_kind = ImplicitKind::kImplicit;
  //   } else {
  //     DCHECK_EQ(implicit_params.kind.value, "js-implicit");
  //     result.implicit_kind = ImplicitKind::kJSImplicit;
  //   }
  //   result.implicit_kind_pos = implicit_params.kind.pos;
  //   for implicit_param in implicit_params.parameters {
  //     AddParameter(&result, implicit_param);
  //   }
  // }
  // if has_explicit_parameter_names {
  //   let explicit_params =
  //     child_results.NextAs::<Vec<NameAndTypeExpression>>();
  //   let mut arguments_variable = String::new();
  //   if has_varargs {
  //     arguments_variable = child_results.NextAs::<String>();
  //   }
  //   for param in explicit_params {
  //     AddParameter(&result, param);
  //   }
  //   result.arguments_variable = arguments_variable;
  // } else {
  //   let explicit_types = child_results.NextAs::<TypeList>();
  //   for explicit_type in explicit_types {
  //     result.types.push_back(explicit_type);
  //   }
  // }
  // return ParseResult{std::move(result)};
  todo!()
}

fn make_assert_statement(child_results: &mut ParseResultIterator) -> Option<ParseResult<Statement*>> {
  let kind_string = child_results.next_as::<Identifier*>().value;
  let expr_with_source = child_results.next_as::<ExpressionWithSource>();
  // let kind = match kind_string.as_str() {
  //   "dcheck" => AssertStatement::AssertKind::kDcheck,
  //   "check" => AssertStatement::AssertKind::kCheck,
  //   "sbxcheck" => {
  //     #[cfg(V8_ENABLE_SANDBOX)]
  //     { AssertStatement::AssertKind::kSbxCheck }
  //     #[cfg(not(V8_ENABLE_SANDBOX))]
  //     { AssertStatement::AssertKind::kDcheck }
  //   }
  //   "static_assert" => AssertStatement::AssertKind::kStaticAssert,
  //   _ => unreachable!(),
  // };
  // Statement* result = MakeNode<AssertStatement>(
  //   kind, expr_with_source.expression, expr_with_source.source);
  // return ParseResult{result};
  todo!()
}

fn make_debug_statement(child_results: &mut ParseResultIterator) -> Option<ParseResult<Statement*>> {
  let kind = child_results.next_as::<Identifier*>().value;
  // assert!(kind == "unreachable" || kind == "debug");
  // Statement* result = MakeNode<DebugStatement>(
  //   if kind == "unreachable" { DebugStatement::Kind::kUnreachable }
  //   else { DebugStatement::Kind::kDebug });
  // return ParseResult{result};
  todo!()
}

fn deprecated_make_void_type(child_results: &mut ParseResultIterator) -> Option<ParseResult<TypeExpression*>> {
  // Error("Default void return types are deprecated. Add `: void`.");
  // TypeExpression* result = MakeNode<BasicTypeExpression>(
  //   std::vector<std::string>{}, MakeNode<Identifier>("void"),
  //   std::vector<TypeExpression*>{});
  // return ParseResult{result};
  todo!()
}

fn make_external_macro(child_results: &mut ParseResultIterator) -> Option<ParseResult<Vec<Declaration*>>> {
  let enabled = process_if_annotation(child_results);
  let transitioning = child_results.next_as::<bool>();
  let operator_name = child_results.next_as::<Option<String>>();
  let external_assembler_name =
    child_results.next_as::<Option<String>>();
  let
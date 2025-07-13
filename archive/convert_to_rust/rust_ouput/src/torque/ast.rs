// Converted from V8 C++ source files:
// Header: ast.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

use crate::numbers::integer_literal::IntegerLiteral;
use crate::torque::constants::*;
use crate::torque::source_positions::*;
use crate::torque::utils::*;

#[macro_export]
macro_rules! DEFINE_AST_NODE_LEAF_BOILERPLATE {
    ($struct_name:ident) => {
        impl $struct_name {
            pub const KIND: AstNode::Kind = AstNode::Kind::k##$struct_name;
            pub fn cast(node: &AstNode) -> Result<&$struct_name, String> {
                if node.kind == Self::KIND {
                    Ok(unsafe { &*(node as *const AstNode as *const $struct_name) })
                } else {
                    Err(format!("Invalid cast to {}", stringify!($struct_name)))
                }
            }
            pub fn dynamic_cast(node: &AstNode) -> Option<&$struct_name> {
                if node.kind == Self::KIND {
                    Some(unsafe { &*(node as *const AstNode as *const $struct_name) })
                } else {
                    None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! DEFINE_AST_NODE_INNER_BOILERPLATE {
    ($trait_name:ident) => {
        impl dyn $trait_name {
            pub fn cast(node: &AstNode) -> Result<&dyn $trait_name, String> {
                if AstNodeClassCheck::is_instance_of::<$trait_name>(node) {
                    Ok(unsafe { &*(node as *const AstNode as *const dyn $trait_name) })
                } else {
                    Err(format!("Invalid cast to {}", stringify!($trait_name)))
                }
            }

            pub fn dynamic_cast(node: &AstNode) -> Option<&dyn $trait_name> {
                if node.kind == AstNode::Kind::kIdentifier {
                    return None;
                }
                if AstNodeClassCheck::is_instance_of::<$trait_name>(node) {
                    Some(unsafe { &*(node as *const AstNode as *const dyn $trait_name) })
                } else {
                    None
                }
            }
        }
    };
}

pub struct AstNode {
    pub kind: AstNode::Kind,
    pub pos: SourcePosition,
}

impl AstNode {
    pub fn new(kind: AstNode::Kind, pos: SourcePosition) -> Self {
        AstNode { kind, pos }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Kind {
        kCallExpression,
        kCallMethodExpression,
        kIntrinsicCallExpression,
        kStructExpression,
        kLogicalOrExpression,
        kLogicalAndExpression,
        kSpreadExpression,
        kConditionalExpression,
        kIdentifierExpression,
        kStringLiteralExpression,
        kIntegerLiteralExpression,
        kFloatingPointLiteralExpression,
        kFieldAccessExpression,
        kElementAccessExpression,
        kDereferenceExpression,
        kAssignmentExpression,
        kIncrementDecrementExpression,
        kNewExpression,
        kAssumeTypeImpossibleExpression,
        kStatementExpression,
        kTryLabelExpression,
        kBasicTypeExpression,
        kFunctionTypeExpression,
        kPrecomputedTypeExpression,
        kUnionTypeExpression,
        kBlockStatement,
        kExpressionStatement,
        kIfStatement,
        kWhileStatement,
        kForLoopStatement,
        kBreakStatement,
        kContinueStatement,
        kReturnStatement,
        kDebugStatement,
        kAssertStatement,
        kTailCallStatement,
        kVarDeclarationStatement,
        kGotoStatement,
        kAbstractTypeDeclaration,
        kTypeAliasDeclaration,
        kBitFieldStructDeclaration,
        kClassDeclaration,
        kStructDeclaration,
        kGenericCallableDeclaration,
        kGenericTypeDeclaration,
        kSpecializationDeclaration,
        kExternConstDeclaration,
        kNamespaceDeclaration,
        kConstDeclaration,
        kCppIncludeDeclaration,
        kTorqueMacroDeclaration,
        kTorqueBuiltinDeclaration,
        kExternalMacroDeclaration,
        kExternalBuiltinDeclaration,
        kExternalRuntimeDeclaration,
        kIntrinsicDeclaration,
        kIdentifier,
        kTryHandler,
        kClassBody,
    }
}

struct AstNodeClassCheck {}

impl AstNodeClassCheck {
    pub fn is_instance_of<T: AstNodeInterface>(node: &AstNode) -> bool {
        match node.kind {
            AstNode::Kind::kCallExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<CallExpression>(),
            AstNode::Kind::kCallMethodExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<CallMethodExpression>(),
            AstNode::Kind::kIntrinsicCallExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IntrinsicCallExpression>(),
            AstNode::Kind::kStructExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<StructExpression>(),
            AstNode::Kind::kLogicalOrExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<LogicalOrExpression>(),
            AstNode::Kind::kLogicalAndExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<LogicalAndExpression>(),
            AstNode::Kind::kSpreadExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<SpreadExpression>(),
            AstNode::Kind::kConditionalExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ConditionalExpression>(),
            AstNode::Kind::kIdentifierExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IdentifierExpression>(),
            AstNode::Kind::kStringLiteralExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<StringLiteralExpression>(),
            AstNode::Kind::kIntegerLiteralExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IntegerLiteralExpression>(),
            AstNode::Kind::kFloatingPointLiteralExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<FloatingPointLiteralExpression>(),
            AstNode::Kind::kFieldAccessExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<FieldAccessExpression>(),
            AstNode::Kind::kElementAccessExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ElementAccessExpression>(),
            AstNode::Kind::kDereferenceExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<DereferenceExpression>(),
            AstNode::Kind::kAssignmentExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<AssignmentExpression>(),
            AstNode::Kind::kIncrementDecrementExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IncrementDecrementExpression>(),
            AstNode::Kind::kNewExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<NewExpression>(),
            AstNode::Kind::kAssumeTypeImpossibleExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<AssumeTypeImpossibleExpression>(),
            AstNode::Kind::kStatementExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<StatementExpression>(),
            AstNode::Kind::kTryLabelExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TryLabelExpression>(),
            AstNode::Kind::kBasicTypeExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<BasicTypeExpression>(),
            AstNode::Kind::kFunctionTypeExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<FunctionTypeExpression>(),
            AstNode::Kind::kPrecomputedTypeExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<PrecomputedTypeExpression>(),
            AstNode::Kind::kUnionTypeExpression => std::any::TypeId::of::<T>() == std::any::TypeId::of::<UnionTypeExpression>(),
            AstNode::Kind::kBlockStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<BlockStatement>(),
            AstNode::Kind::kExpressionStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ExpressionStatement>(),
            AstNode::Kind::kIfStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IfStatement>(),
            AstNode::Kind::kWhileStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<WhileStatement>(),
            AstNode::Kind::kForLoopStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ForLoopStatement>(),
            AstNode::Kind::kBreakStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<BreakStatement>(),
            AstNode::Kind::kContinueStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ContinueStatement>(),
            AstNode::Kind::kReturnStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ReturnStatement>(),
            AstNode::Kind::kDebugStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<DebugStatement>(),
            AstNode::Kind::kAssertStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<AssertStatement>(),
            AstNode::Kind::kTailCallStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TailCallStatement>(),
            AstNode::Kind::kVarDeclarationStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<VarDeclarationStatement>(),
            AstNode::Kind::kGotoStatement => std::any::TypeId::of::<T>() == std::any::TypeId::of::<GotoStatement>(),
            AstNode::Kind::kAbstractTypeDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<AbstractTypeDeclaration>(),
            AstNode::Kind::kTypeAliasDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TypeAliasDeclaration>(),
            AstNode::Kind::kBitFieldStructDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<BitFieldStructDeclaration>(),
            AstNode::Kind::kClassDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ClassDeclaration>(),
            AstNode::Kind::kStructDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<StructDeclaration>(),
            AstNode::Kind::kGenericCallableDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<GenericCallableDeclaration>(),
            AstNode::Kind::kGenericTypeDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<GenericTypeDeclaration>(),
            AstNode::Kind::kSpecializationDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<SpecializationDeclaration>(),
            AstNode::Kind::kExternConstDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ExternConstDeclaration>(),
            AstNode::Kind::kNamespaceDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<NamespaceDeclaration>(),
            AstNode::Kind::kConstDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ConstDeclaration>(),
            AstNode::Kind::kCppIncludeDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<CppIncludeDeclaration>(),
            AstNode::Kind::kTorqueMacroDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TorqueMacroDeclaration>(),
            AstNode::Kind::kTorqueBuiltinDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TorqueBuiltinDeclaration>(),
            AstNode::Kind::kExternalMacroDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ExternalMacroDeclaration>(),
            AstNode::Kind::kExternalBuiltinDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ExternalBuiltinDeclaration>(),
            AstNode::Kind::kExternalRuntimeDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ExternalRuntimeDeclaration>(),
            AstNode::Kind::kIntrinsicDeclaration => std::any::TypeId::of::<T>() == std::any::TypeId::of::<IntrinsicDeclaration>(),
            AstNode::Kind::kIdentifier => std::any::TypeId::of::<T>() == std::any::TypeId::of::<Identifier>(),
            AstNode::Kind::kTryHandler => std::any::TypeId::of::<T>() == std::any::TypeId::of::<TryHandler>(),
            AstNode::Kind::kClassBody => std::any::TypeId::of::<T>() == std::any::TypeId::of::<ClassBody>(),
            _ => false,
        }
    }
}

trait AstNodeInterface {
    fn visit_all_sub_expressions(&self, _callback: &mut dyn FnMut(&dyn Expression)) {}
}

pub trait Expression: AstNodeInterface {
    fn as_any(&self) -> &dyn Any;
}
DEFINE_AST_NODE_INNER_BOILERPLATE!(Expression);

pub trait LocationExpression: Expression {}
DEFINE_AST_NODE_INNER_BOILERPLATE!(LocationExpression);

pub trait TypeExpression: AstNodeInterface {}
DEFINE_AST_NODE_INNER_BOILERPLATE!(TypeExpression);

pub trait Declaration: AstNodeInterface {}
DEFINE_AST_NODE_INNER_BOILERPLATE!(Declaration);

pub trait Statement: AstNodeInterface {}
DEFINE_AST_NODE_INNER_BOILERPLATE!(Statement);

pub struct NamespaceDeclaration {
    pub declarations: Vec<Box<dyn Declaration>>,
    pub name: String,
    pub base: AstNode,
}

impl NamespaceDeclaration {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(NamespaceDeclaration);
    pub fn new(
        pos: SourcePosition,
        name: String,
        declarations: Vec<Box<dyn Declaration>>,
    ) -> Self {
        NamespaceDeclaration {
            base: AstNode::new(AstNode::Kind::kNamespaceDeclaration, pos),
            declarations,
            name,
        }
    }
}

pub struct EnumDescription {
    pub pos: SourcePosition,
    pub name: String,
    pub constexpr_generates: String,
    pub is_open: bool,
    pub entries: Vec<EnumDescriptionEntry>,
}

pub struct EnumDescriptionEntry {
    pub name: String,
    pub alias_entry: String,
}

impl EnumDescriptionEntry {
    pub fn new(name: String, alias_entry: String) -> Self {
        EnumDescriptionEntry { name, alias_entry }
    }
}

pub struct Ast {
    declarations_: Vec<Box<dyn Declaration>>,
    nodes_: Vec<Box<dyn AstNodeInterface>>,
    declared_imports_: HashMap<SourceId, HashSet<SourceId>>,
    enum_descriptions_: Vec<EnumDescription>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            declarations_: Vec::new(),
            nodes_: Vec::new(),
            declared_imports_: HashMap::new(),
            enum_descriptions_: Vec::new(),
        }
    }

    pub fn declarations(&mut self) -> &mut Vec<Box<dyn Declaration>> {
        &mut self.declarations_
    }

    pub fn get_declarations(&self) -> &Vec<Box<dyn Declaration>> {
        &self.declarations_
    }

    pub fn add_node<T: AstNodeInterface + 'static>(&mut self, node: T) -> &T {
        let node = Box::new(node);
        let result = unsafe { &*(Box::into_raw(node) as *const T) };
        self.nodes_.push(Box::new(result));
        result
    }

    pub fn declare_import_for_current_file(&mut self, import_id: SourceId) {
        let source_id = CurrentSourcePosition::get().source;
        self.declared_imports_
            .entry(source_id)
            .or_insert_with(HashSet::new)
            .insert(import_id);
    }

    pub fn add_enum_description(&mut self, description: EnumDescription) {
        let name = &description.name;
        assert!(!name.is_empty());
        if self
            .enum_descriptions_
            .iter()
            .any(|existing| existing.name == *name)
        {
            panic!("Enum description with name {} already exists", name);
        }
        self.enum_descriptions_.push(description);
    }

    pub fn enum_descriptions(&mut self) -> &mut Vec<EnumDescription> {
        &mut self.enum_descriptions_
    }
}

thread_local! {
    pub static CURRENT_AST: CurrentAst = CurrentAst::new();
}

pub struct CurrentAst {
    ast: Option<Ast>,
}

impl CurrentAst {
    const fn new() -> Self {
        CurrentAst { ast: None }
    }

    pub fn set(mut self, ast: Ast) {
        self.ast = Some(ast);
    }

    pub fn get() -> Ast {
        CURRENT_AST.with(|ca| {
            if let Some(ref ast) = ca.ast {
                let value =  unsafe { std::ptr::read(ast) };
                return value;
            }
             panic!("No CurrentAst set");
        })
    }
}

const K_THIS_PARAMETER_NAME: &str = "this";

// A Identifier is a string with a SourcePosition attached.
pub struct Identifier {
    pub value: String,
    pub base: AstNode,
}

impl Identifier {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(Identifier);
    pub fn new(pos: SourcePosition, identifier: String) -> Self {
        Identifier {
            base: AstNode::new(AstNode::Kind::kIdentifier, pos),
            value: identifier,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct IdentifierPtrValueEq {}

impl IdentifierPtrValueEq {
    fn operator(a: &Identifier, b: &Identifier) -> bool {
        a.value < b.value
    }
}

pub struct IdentifierExpression {
    pub namespace_qualification: Vec<String>,
    pub name: Box<Identifier>,
    pub generic_arguments: Vec<Box<dyn TypeExpression>>,
    base: AstNode,
}

impl IdentifierExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(IdentifierExpression);
    pub fn new(
        pos: SourcePosition,
        namespace_qualification: Vec<String>,
        name: Box<Identifier>,
        generic_arguments: Vec<Box<dyn TypeExpression>>,
    ) -> Self {
        IdentifierExpression {
            base: AstNode::new(AstNode::Kind::kIdentifierExpression, pos),
            namespace_qualification,
            name,
            generic_arguments,
        }
    }

    pub fn new_simple(
        pos: SourcePosition,
        name: Box<Identifier>,
        generic_arguments: Vec<Box<dyn TypeExpression>>,
    ) -> Self {
        IdentifierExpression {
            base: AstNode::new(AstNode::Kind::kIdentifierExpression, pos),
            namespace_qualification: Vec::new(),
            name,
            generic_arguments,
        }
    }

    pub fn is_this(&self) -> bool {
        self.name.value == K_THIS_PARAMETER_NAME
    }
}

impl AstNodeInterface for IdentifierExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //callback(self);
    }
}

impl Expression for IdentifierExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LocationExpression for IdentifierExpression {}

pub struct IntrinsicCallExpression {
    pub name: Box<Identifier>,
    pub generic_arguments: Vec<Box<dyn TypeExpression>>,
    pub arguments: Vec<Box<dyn Expression>>,
    base: AstNode,
}

impl IntrinsicCallExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(IntrinsicCallExpression);
    pub fn new(
        pos: SourcePosition,
        name: Box<Identifier>,
        generic_arguments: Vec<Box<dyn TypeExpression>>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        IntrinsicCallExpression {
            base: AstNode::new(AstNode::Kind::kIntrinsicCallExpression, pos),
            name,
            generic_arguments,
            arguments,
        }
    }
}

impl AstNodeInterface for IntrinsicCallExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        for argument in &self.arguments {
           // argument.visit_all_sub_expressions(callback);
        }
        //callback(self);
    }
}

impl Expression for IntrinsicCallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallMethodExpression {
    pub target: Box<dyn Expression>,
    pub method: Box<IdentifierExpression>,
    pub arguments: Vec<Box<dyn Expression>>,
    pub labels: Vec<Box<Identifier>>,
    base: AstNode,
}

impl CallMethodExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(CallMethodExpression);
    pub fn new(
        pos: SourcePosition,
        target: Box<dyn Expression>,
        method: Box<IdentifierExpression>,
        arguments: Vec<Box<dyn Expression>>,
        labels: Vec<Box<Identifier>>,
    ) -> Self {
        CallMethodExpression {
            base: AstNode::new(AstNode::Kind::kCallMethodExpression, pos),
            target,
            method,
            arguments,
            labels,
        }
    }
}

impl AstNodeInterface for CallMethodExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.target.visit_all_sub_expressions(callback);
        //self.method.visit_all_sub_expressions(callback);
        for argument in &self.arguments {
            //argument.visit_all_sub_expressions(callback);
        }
        //callback(self);
    }
}

impl Expression for CallMethodExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    pub callee: Box<IdentifierExpression>,
    pub arguments: Vec<Box<dyn Expression>>,
    pub labels: Vec<Box<Identifier>>,
    base: AstNode,
}

impl CallExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(CallExpression);
    pub fn new(
        pos: SourcePosition,
        callee: Box<IdentifierExpression>,
        arguments: Vec<Box<dyn Expression>>,
        labels: Vec<Box<Identifier>>,
    ) -> Self {
        CallExpression {
            base: AstNode::new(AstNode::Kind::kCallExpression, pos),
            callee,
            arguments,
            labels,
        }
    }
}

impl AstNodeInterface for CallExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.callee.visit_all_sub_expressions(callback);
        for argument in &self.arguments {
           // argument.visit_all_sub_expressions(callback);
        }
        //callback(self);
    }
}

impl Expression for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NameAndExpression {
    pub name: Box<Identifier>,
    pub expression: Box<dyn Expression>,
}

pub struct StructExpression {
    pub type_: Box<dyn TypeExpression>,
    pub initializers: Vec<NameAndExpression>,
    base: AstNode,
}

impl StructExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(StructExpression);
    pub fn new(
        pos: SourcePosition,
        type_: Box<dyn TypeExpression>,
        initializers: Vec<NameAndExpression>,
    ) -> Self {
        StructExpression {
            base: AstNode::new(AstNode::Kind::kStructExpression, pos),
            type_,
            initializers,
        }
    }
}

impl AstNodeInterface for StructExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        for initializer in &self.initializers {
            //initializer.expression.visit_all_sub_expressions(callback);
        }
        //callback(self);
    }
}

impl Expression for StructExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LogicalOrExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    base: AstNode,
}

impl LogicalOrExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(LogicalOrExpression);
    pub fn new(
        pos: SourcePosition,
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
    ) -> Self {
        LogicalOrExpression {
            base: AstNode::new(AstNode::Kind::kLogicalOrExpression, pos),
            left,
            right,
        }
    }
}

impl AstNodeInterface for LogicalOrExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.left.visit_all_sub_expressions(callback);
        //self.right.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for LogicalOrExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LogicalAndExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    base: AstNode,
}

impl LogicalAndExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(LogicalAndExpression);
    pub fn new(
        pos: SourcePosition,
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
    ) -> Self {
        LogicalAndExpression {
            base: AstNode::new(AstNode::Kind::kLogicalAndExpression, pos),
            left,
            right,
        }
    }
}

impl AstNodeInterface for LogicalAndExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.left.visit_all_sub_expressions(callback);
        //self.right.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for LogicalAndExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SpreadExpression {
    pub spreadee: Box<dyn Expression>,
    base: AstNode,
}

impl SpreadExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(SpreadExpression);
    pub fn new(pos: SourcePosition, spreadee: Box<dyn Expression>) -> Self {
        SpreadExpression {
            base: AstNode::new(AstNode::Kind::kSpreadExpression, pos),
            spreadee,
        }
    }
}

impl AstNodeInterface for SpreadExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.spreadee.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for SpreadExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConditionalExpression {
    pub condition: Box<dyn Expression>,
    pub if_true: Box<dyn Expression>,
    pub if_false: Box<dyn Expression>,
    base: AstNode,
}

impl ConditionalExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(ConditionalExpression);
    pub fn new(
        pos: SourcePosition,
        condition: Box<dyn Expression>,
        if_true: Box<dyn Expression>,
        if_false: Box<dyn Expression>,
    ) -> Self {
        ConditionalExpression {
            base: AstNode::new(AstNode::Kind::kConditionalExpression, pos),
            condition,
            if_true,
            if_false,
        }
    }
}

impl AstNodeInterface for ConditionalExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.condition.visit_all_sub_expressions(callback);
        //self.if_true.visit_all_sub_expressions(callback);
        //self.if_false.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for ConditionalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StringLiteralExpression {
    pub literal: String,
    base: AstNode,
}

impl StringLiteralExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(StringLiteralExpression);
    pub fn new(pos: SourcePosition, literal: String) -> Self {
        StringLiteralExpression {
            base: AstNode::new(AstNode::Kind::kStringLiteralExpression, pos),
            literal,
        }
    }
}

impl AstNodeInterface for StringLiteralExpression {
    fn visit_all_sub_expressions(&self, _callback: &mut dyn FnMut(&dyn Expression)) {
        //callback(self);
    }
}

impl Expression for StringLiteralExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IntegerLiteralExpression {
    pub value: IntegerLiteral,
    base: AstNode,
}

impl IntegerLiteralExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(IntegerLiteralExpression);
    pub fn new(pos: SourcePosition, value: IntegerLiteral) -> Self {
        IntegerLiteralExpression {
            base: AstNode::new(AstNode::Kind::kIntegerLiteralExpression, pos),
            value,
        }
    }
}

impl AstNodeInterface for IntegerLiteralExpression {
    fn visit_all_sub_expressions(&self, _callback: &mut dyn FnMut(&dyn Expression)) {
        //callback(self);
    }
}

impl Expression for IntegerLiteralExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct FloatingPointLiteralExpression {
    pub value: f64,
    base: AstNode,
}

impl FloatingPointLiteralExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(FloatingPointLiteralExpression);
    pub fn new(pos: SourcePosition, value: f64) -> Self {
        FloatingPointLiteralExpression {
            base: AstNode::new(AstNode::Kind::kFloatingPointLiteralExpression, pos),
            value,
        }
    }
}

impl AstNodeInterface for FloatingPointLiteralExpression {
    fn visit_all_sub_expressions(&self, _callback: &mut dyn FnMut(&dyn Expression)) {
        //callback(self);
    }
}

impl Expression for FloatingPointLiteralExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ElementAccessExpression {
    pub array: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
    base: AstNode,
}

impl ElementAccessExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(ElementAccessExpression);
    pub fn new(
        pos: SourcePosition,
        array: Box<dyn Expression>,
        index: Box<dyn Expression>,
    ) -> Self {
        ElementAccessExpression {
            base: AstNode::new(AstNode::Kind::kElementAccessExpression, pos),
            array,
            index,
        }
    }
}

impl AstNodeInterface for ElementAccessExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.array.visit_all_sub_expressions(callback);
        //self.index.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for ElementAccessExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LocationExpression for ElementAccessExpression {}

pub struct FieldAccessExpression {
    pub object: Box<dyn Expression>,
    pub field: Box<Identifier>,
    base: AstNode,
}

impl FieldAccessExpression {
    DEFINE_AST_NODE_LEAF_BOILERPLATE!(FieldAccessExpression);
    pub fn new(pos: SourcePosition, object: Box<dyn Expression>, field: Box<Identifier>) -> Self {
        FieldAccessExpression {
            base: AstNode::new(AstNode::Kind::kFieldAccessExpression, pos),
            object,
            field,
        }
    }
}

impl AstNodeInterface for FieldAccessExpression {
    fn visit_all_sub_expressions(&self, callback: &mut dyn FnMut(&dyn Expression)) {
        //self.object.visit_all_sub_expressions(callback);
        //callback(self);
    }
}

impl Expression for FieldAccessExpression {
    fn as_any(&self)

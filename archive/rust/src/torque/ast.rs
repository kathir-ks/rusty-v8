// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ast {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::fmt;
    use std::rc::Rc;

    use crate::numbers::integer_literal::IntegerLiteral;
    use crate::torque::constants::IsConstexprName;
    use crate::torque::source_positions::{SourceId, SourcePosition, CurrentSourcePosition};
    use crate::torque::utils::Error;

    macro_rules! define_ast_node_kind_enum {
        ($($name:ident,)*) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub enum Kind {
                $(
                    $name,
                )*
            }
        }
    }

    define_ast_node_kind_enum! {
        CallExpression,
        CallMethodExpression,
        IntrinsicCallExpression,
        StructExpression,
        LogicalOrExpression,
        LogicalAndExpression,
        SpreadExpression,
        ConditionalExpression,
        IdentifierExpression,
        StringLiteralExpression,
        IntegerLiteralExpression,
        FloatingPointLiteralExpression,
        FieldAccessExpression,
        ElementAccessExpression,
        DereferenceExpression,
        AssignmentExpression,
        IncrementDecrementExpression,
        NewExpression,
        AssumeTypeImpossibleExpression,
        StatementExpression,
        TryLabelExpression,
        BasicTypeExpression,
        FunctionTypeExpression,
        PrecomputedTypeExpression,
        UnionTypeExpression,
        BlockStatement,
        ExpressionStatement,
        IfStatement,
        WhileStatement,
        ForLoopStatement,
        BreakStatement,
        ContinueStatement,
        ReturnStatement,
        DebugStatement,
        AssertStatement,
        TailCallStatement,
        VarDeclarationStatement,
        GotoStatement,
        AbstractTypeDeclaration,
        TypeAliasDeclaration,
        BitFieldStructDeclaration,
        ClassDeclaration,
        GenericCallableDeclaration,
        GenericTypeDeclaration,
        SpecializationDeclaration,
        ExternConstDeclaration,
        NamespaceDeclaration,
        ConstDeclaration,
        CppIncludeDeclaration,
        TorqueMacroDeclaration,
        TorqueBuiltinDeclaration,
        ExternalMacroDeclaration,
        ExternalBuiltinDeclaration,
        ExternalRuntimeDeclaration,
        IntrinsicDeclaration,
        Identifier,
        TryHandler,
        ClassBody,
    }

    pub struct AstNode {
        pub kind: Kind,
        pub pos: SourcePosition,
    }

    impl AstNode {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            AstNode { kind, pos }
        }
    }

    pub trait AstNodeTrait {
        fn as_node(&self) -> &AstNode;
        fn as_node_mut(&mut self) -> &mut AstNode;
    }

    macro_rules! impl_ast_node_trait {
        ($struct_name:ident) => {
            impl AstNodeTrait for $struct_name {
                fn as_node(&self) -> &AstNode {
                    &self.node
                }
                fn as_node_mut(&mut self) -> &mut AstNode {
                    &mut self.node
                }
            }
        };
    }

    pub struct Expression {
        pub node: AstNode,
    }

    impl Expression {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            Expression {
                node: AstNode::new(kind, pos),
            }
        }

        pub type VisitCallback = Rc<dyn Fn(&Expression)>;

        pub fn visit_all_sub_expressions(&self, _callback: VisitCallback) {
            // TODO(szuend): Hoist this up to AstNode and make it a
            //               general Ast visitor.
        }
    }

    impl_ast_node_trait!(Expression);

    pub struct LocationExpression {
        pub node: AstNode,
    }

    impl LocationExpression {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            LocationExpression {
                node: AstNode::new(kind, pos),
            }
        }
    }

    impl_ast_node_trait!(LocationExpression);

    pub struct TypeExpression {
        pub node: AstNode,
    }

    impl TypeExpression {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            TypeExpression {
                node: AstNode::new(kind, pos),
            }
        }
    }

    impl_ast_node_trait!(TypeExpression);

    pub struct Declaration {
        pub node: AstNode,
    }

    impl Declaration {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            Declaration {
                node: AstNode::new(kind, pos),
            }
        }
    }

    impl_ast_node_trait!(Declaration);

    pub struct Statement {
        pub node: AstNode,
    }

    impl Statement {
        pub fn new(kind: Kind, pos: SourcePosition) -> Self {
            Statement {
                node: AstNode::new(kind, pos),
            }
        }
    }

    impl_ast_node_trait!(Statement);

    pub struct NamespaceDeclaration {
        pub node: AstNode,
        pub declarations: Vec<Rc<RefCell<Declaration>>>,
        pub name: String,
    }

    impl NamespaceDeclaration {
        pub fn new(
            pos: SourcePosition,
            name: String,
            declarations: Vec<Rc<RefCell<Declaration>>>,
        ) -> Self {
            NamespaceDeclaration {
                node: AstNode::new(Kind::NamespaceDeclaration, pos),
                declarations,
                name,
            }
        }
    }

    impl_ast_node_trait!(NamespaceDeclaration);

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

    impl EnumDescription {
        pub fn new(
            pos: SourcePosition,
            name: String,
            constexpr_generates: String,
            is_open: bool,
            entries: Vec<EnumDescriptionEntry>,
        ) -> Self {
            EnumDescription {
                pos,
                name,
                constexpr_generates,
                is_open,
                entries,
            }
        }
    }

    pub struct Ast {
        declarations_: Vec<Rc<RefCell<Declaration>>>,
        nodes_: Vec<Rc<RefCell<AstNode>>>,
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

        pub fn declarations(&mut self) -> &mut Vec<Rc<RefCell<Declaration>>> {
            &mut self.declarations_
        }
        pub fn get_declarations(&self) -> &Vec<Rc<RefCell<Declaration>>> {
            &self.declarations_
        }

        pub fn add_node<T: 'static>(&mut self, node: T) -> Rc<RefCell<dyn AstNodeTrait>> {
            let rc_node = Rc::new(RefCell::new(node));
            self.nodes_.push(rc_node.clone());
            rc_node
        }

        pub fn declare_import_for_current_file(&mut self, import_id: SourceId) {
            let current_source = CurrentSourcePosition::get().source;
            self.declared_imports_
                .entry(current_source)
                .or_insert_with(HashSet::new)
                .insert(import_id);
        }

        pub fn add_enum_description(&mut self, description: EnumDescription) {
            let name = description.name.clone();
            assert!(!name.is_empty());
            #[cfg(debug_assertions)]
            {
                let f = |d: &EnumDescription| d.name == name;
                assert_eq!(
                    self.enum_descriptions_.iter().find(|&d| f(d)),
                    None
                );
            }
            self.enum_descriptions_.push(description);
        }

        pub fn enum_descriptions(&mut self) -> &mut Vec<EnumDescription> {
            &mut self.enum_descriptions_
        }
    }

    pub const K_THIS_PARAMETER_NAME: &str = "this";

    // A Identifier is a string with a SourcePosition attached.
    pub struct Identifier {
        pub node: AstNode,
        pub value: String,
    }

    impl Identifier {
        pub fn new(pos: SourcePosition, identifier: String) -> Self {
            Identifier {
                node: AstNode::new(Kind::Identifier, pos),
                value: identifier,
            }
        }
    }

    impl_ast_node_trait!(Identifier);

    impl fmt::Display for Identifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct IdentifierPtrValueEq;

    impl IdentifierPtrValueEq {
        pub fn compare(a: &Identifier, b: &Identifier) -> bool {
            a.value < b.value
        }
    }

    pub struct IdentifierExpression {
        pub node: AstNode,
        pub namespace_qualification: Vec<String>,
        pub name: Rc<RefCell<Identifier>>,
        pub generic_arguments: Vec<Rc<RefCell<TypeExpression>>>,
    }

    impl IdentifierExpression {
        pub fn new_with_namespace(
            pos: SourcePosition,
            namespace_qualification: Vec<String>,
            name: Rc<RefCell<Identifier>>,
            args: Vec<Rc<RefCell<TypeExpression>>>,
        ) -> Self {
            IdentifierExpression {
                node: AstNode::new(Kind::IdentifierExpression, pos),
                namespace_qualification,
                name,
                generic_arguments: args,
            }
        }

        pub fn new(
            pos: SourcePosition,
            name: Rc<RefCell<Identifier>>,
            args: Vec<Rc<RefCell<TypeExpression>>>,
        ) -> Self {
            IdentifierExpression::new_with_namespace(pos, vec![], name, args)
        }

        pub fn is_this(&self) -> bool {
            self.name.borrow().value == K_THIS_PARAMETER_NAME
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(IdentifierExpression);

    pub struct IntrinsicCallExpression {
        pub node: AstNode,
        pub name: Rc<RefCell<Identifier>>,
        pub generic_arguments: Vec<Rc<RefCell<TypeExpression>>>,
        pub arguments: Vec<Rc<RefCell<Expression>>>,
    }

    impl IntrinsicCallExpression {
        pub fn new(
            pos: SourcePosition,
            name: Rc<RefCell<Identifier>>,
            generic_arguments: Vec<Rc<RefCell<TypeExpression>>>,
            arguments: Vec<Rc<RefCell<Expression>>>,
        ) -> Self {
            IntrinsicCallExpression {
                node: AstNode::new(Kind::IntrinsicCallExpression, pos),
                name,
                generic_arguments,
                arguments,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            for argument in &self.arguments {
                argument.borrow().visit_all_sub_expressions(callback.clone());
            }
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(IntrinsicCallExpression);

    pub struct CallMethodExpression {
        pub node: AstNode,
        pub target: Rc<RefCell<Expression>>,
        pub method: Rc<RefCell<IdentifierExpression>>,
        pub arguments: Vec<Rc<RefCell<Expression>>>,
        pub labels: Vec<Rc<RefCell<Identifier>>>,
    }

    impl CallMethodExpression {
        pub fn new(
            pos: SourcePosition,
            target: Rc<RefCell<Expression>>,
            method: Rc<RefCell<IdentifierExpression>>,
            arguments: Vec<Rc<RefCell<Expression>>>,
            labels: Vec<Rc<RefCell<Identifier>>>,
        ) -> Self {
            CallMethodExpression {
                node: AstNode::new(Kind::CallMethodExpression, pos),
                target,
                method,
                arguments,
                labels,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.target.borrow().visit_all_sub_expressions(callback.clone());
            self.method.borrow().visit_all_sub_expressions(callback.clone());
            for argument in &self.arguments {
                argument.borrow().visit_all_sub_expressions(callback.clone());
            }
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(CallMethodExpression);

    pub struct CallExpression {
        pub node: AstNode,
        pub callee: Rc<RefCell<IdentifierExpression>>,
        pub arguments: Vec<Rc<RefCell<Expression>>>,
        pub labels: Vec<Rc<RefCell<Identifier>>>,
    }

    impl CallExpression {
        pub fn new(
            pos: SourcePosition,
            callee: Rc<RefCell<IdentifierExpression>>,
            arguments: Vec<Rc<RefCell<Expression>>>,
            labels: Vec<Rc<RefCell<Identifier>>>,
        ) -> Self {
            CallExpression {
                node: AstNode::new(Kind::CallExpression, pos),
                callee,
                arguments,
                labels,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.callee.borrow().visit_all_sub_expressions(callback.clone());
            for argument in &self.arguments {
                argument.borrow().visit_all_sub_expressions(callback.clone());
            }
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(CallExpression);

    pub struct NameAndExpression {
        pub name: Rc<RefCell<Identifier>>,
        pub expression: Rc<RefCell<Expression>>,
    }

    impl NameAndExpression {
        pub fn new(name: Rc<RefCell<Identifier>>, expression: Rc<RefCell<Expression>>) -> Self {
            NameAndExpression { name, expression }
        }
    }

    pub struct StructExpression {
        pub node: AstNode,
        pub type_: Rc<RefCell<TypeExpression>>,
        pub initializers: Vec<NameAndExpression>,
    }

    impl StructExpression {
        pub fn new(
            pos: SourcePosition,
            type_: Rc<RefCell<TypeExpression>>,
            initializers: Vec<NameAndExpression>,
        ) -> Self {
            StructExpression {
                node: AstNode::new(Kind::StructExpression, pos),
                type_,
                initializers,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            for initializer in &self.initializers {
                initializer
                    .expression
                    .borrow()
                    .visit_all_sub_expressions(callback.clone());
            }
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(StructExpression);

    pub struct LogicalOrExpression {
        pub node: AstNode,
        pub left: Rc<RefCell<Expression>>,
        pub right: Rc<RefCell<Expression>>,
    }

    impl LogicalOrExpression {
        pub fn new(
            pos: SourcePosition,
            left: Rc<RefCell<Expression>>,
            right: Rc<RefCell<Expression>>,
        ) -> Self {
            LogicalOrExpression {
                node: AstNode::new(Kind::LogicalOrExpression, pos),
                left,
                right,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.left.borrow().visit_all_sub_expressions(callback.clone());
            self.right.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(LogicalOrExpression);

    pub struct LogicalAndExpression {
        pub node: AstNode,
        pub left: Rc<RefCell<Expression>>,
        pub right: Rc<RefCell<Expression>>,
    }

    impl LogicalAndExpression {
        pub fn new(
            pos: SourcePosition,
            left: Rc<RefCell<Expression>>,
            right: Rc<RefCell<Expression>>,
        ) -> Self {
            LogicalAndExpression {
                node: AstNode::new(Kind::LogicalAndExpression, pos),
                left,
                right,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.left.borrow().visit_all_sub_expressions(callback.clone());
            self.right.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(LogicalAndExpression);

    pub struct SpreadExpression {
        pub node: AstNode,
        pub spreadee: Rc<RefCell<Expression>>,
    }

    impl SpreadExpression {
        pub fn new(pos: SourcePosition, spreadee: Rc<RefCell<Expression>>) -> Self {
            SpreadExpression {
                node: AstNode::new(Kind::SpreadExpression, pos),
                spreadee,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.spreadee.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(SpreadExpression);

    pub struct ConditionalExpression {
        pub node: AstNode,
        pub condition: Rc<RefCell<Expression>>,
        pub if_true: Rc<RefCell<Expression>>,
        pub if_false: Rc<RefCell<Expression>>,
    }

    impl ConditionalExpression {
        pub fn new(
            pos: SourcePosition,
            condition: Rc<RefCell<Expression>>,
            if_true: Rc<RefCell<Expression>>,
            if_false: Rc<RefCell<Expression>>,
        ) -> Self {
            ConditionalExpression {
                node: AstNode::new(Kind::ConditionalExpression, pos),
                condition,
                if_true,
                if_false,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.condition.borrow().visit_all_sub_expressions(callback.clone());
            self.if_true.borrow().visit_all_sub_expressions(callback.clone());
            self.if_false.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(ConditionalExpression);

    pub struct StringLiteralExpression {
        pub node: AstNode,
        pub literal: String,
    }

    impl StringLiteralExpression {
        pub fn new(pos: SourcePosition, literal: String) -> Self {
            StringLiteralExpression {
                node: AstNode::new(Kind::StringLiteralExpression, pos),
                literal,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(StringLiteralExpression);

    pub struct IntegerLiteralExpression {
        pub node: AstNode,
        pub value: IntegerLiteral,
    }

    impl IntegerLiteralExpression {
        pub fn new(pos: SourcePosition, value: IntegerLiteral) -> Self {
            IntegerLiteralExpression {
                node: AstNode::new(Kind::IntegerLiteralExpression, pos),
                value,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(IntegerLiteralExpression);

    pub struct FloatingPointLiteralExpression {
        pub node: AstNode,
        pub value: f64,
    }

    impl FloatingPointLiteralExpression {
        pub fn new(pos: SourcePosition, value: f64) -> Self {
            FloatingPointLiteralExpression {
                node: AstNode::new(Kind::FloatingPointLiteralExpression, pos),
                value,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(FloatingPointLiteralExpression);

    pub struct ElementAccessExpression {
        pub node: AstNode,
        pub array: Rc<RefCell<Expression>>,
        pub index: Rc<RefCell<Expression>>,
    }

    impl ElementAccessExpression {
        pub fn new(
            pos: SourcePosition,
            array: Rc<RefCell<Expression>>,
            index: Rc<RefCell<Expression>>,
        ) -> Self {
            ElementAccessExpression {
                node: AstNode::new(Kind::ElementAccessExpression, pos),
                array,
                index,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.array.borrow().visit_all_sub_expressions(callback.clone());
            self.index.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(ElementAccessExpression);

    pub struct FieldAccessExpression {
        pub node: AstNode,
        pub object: Rc<RefCell<Expression>>,
        pub field: Rc<RefCell<Identifier>>,
    }

    impl FieldAccessExpression {
        pub fn new(
            pos: SourcePosition,
            object: Rc<RefCell<Expression>>,
            field: Rc<RefCell<Identifier>>,
        ) -> Self {
            FieldAccessExpression {
                node: AstNode::new(Kind::FieldAccessExpression, pos),
                object,
                field,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.object.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(FieldAccessExpression);

    pub struct DereferenceExpression {
        pub node: AstNode,
        pub reference: Rc<RefCell<Expression>>,
    }

    impl DereferenceExpression {
        pub fn new(pos: SourcePosition, reference: Rc<RefCell<Expression>>) -> Self {
            DereferenceExpression {
                node: AstNode::new(Kind::DereferenceExpression, pos),
                reference,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.reference.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(DereferenceExpression);

    pub struct AssignmentExpression {
        pub node: AstNode,
        pub location: Rc<RefCell<Expression>>,
        pub op: Option<String>,
        pub value: Rc<RefCell<Expression>>,
    }

    impl AssignmentExpression {
        pub fn new(
            pos: SourcePosition,
            location: Rc<RefCell<Expression>>,
            op: Option<String>,
            value: Rc<RefCell<Expression>>,
        ) -> Self {
            AssignmentExpression {
                node: AstNode::new(Kind::AssignmentExpression, pos),
                location,
                op,
                value,
            }
        }

        pub fn new_simple(
            pos: SourcePosition,
            location: Rc<RefCell<Expression>>,
            value: Rc<RefCell<Expression>>,
        ) -> Self {
            AssignmentExpression::new(pos, location, None, value)
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.location.borrow().visit_all_sub_expressions(callback.clone());
            self.value.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(AssignmentExpression);

    #[derive(Debug, Copy, Clone)]
    pub enum IncrementDecrementOperator {
        Increment,
        Decrement,
    }

    pub struct IncrementDecrementExpression {
        pub node: AstNode,
        pub location: Rc<RefCell<Expression>>,
        pub op: IncrementDecrementOperator,
        pub postfix: bool,
    }

    impl IncrementDecrementExpression {
        pub fn new(
            pos: SourcePosition,
            location: Rc<RefCell<Expression>>,
            op: IncrementDecrementOperator,
            postfix: bool,
        ) -> Self {
            IncrementDecrementExpression {
                node: AstNode::new(Kind::IncrementDecrementExpression, pos),
                location,
                op,
                postfix,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.location.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(IncrementDecrementExpression);

    // This expression is only used in the desugaring of typeswitch, and it allows
    // to bake in the static information that certain types are impossible at a
    // certain position in the control flow.
    // The result type is the type of {expression} minus the provided type.
    pub struct AssumeTypeImpossibleExpression {
        pub node: AstNode,
        pub excluded_type: Rc<RefCell<TypeExpression>>,
        pub expression: Rc<RefCell<Expression>>,
    }

    impl AssumeTypeImpossibleExpression {
        pub fn new(
            pos: SourcePosition,
            excluded_type: Rc<RefCell<TypeExpression>>,
            expression: Rc<RefCell<Expression>>,
        ) -> Self {
            AssumeTypeImpossibleExpression {
                node: AstNode::new(Kind::AssumeTypeImpossibleExpression, pos),
                excluded_type,
                expression,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            self.expression.borrow().visit_all_sub_expressions(callback.clone());
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(AssumeTypeImpossibleExpression);

    pub struct NewExpression {
        pub node: AstNode,
        pub type_: Rc<RefCell<TypeExpression>>,
        pub initializers: Vec<NameAndExpression>,
        pub pretenured: bool,
        pub clear_padding: bool,
    }

    impl NewExpression {
        pub fn new(
            pos: SourcePosition,
            type_: Rc<RefCell<TypeExpression>>,
            initializers: Vec<NameAndExpression>,
            pretenured: bool,
            clear_padding: bool,
        ) -> Self {
            NewExpression {
                node: AstNode::new(Kind::NewExpression, pos),
                type_,
                initializers,
                pretenured,
                clear_padding,
            }
        }

        pub fn visit_all_sub_expressions(&self, callback: Expression::VisitCallback) {
            for initializer in &self.initializers {
                initializer
                    .expression
                    .borrow()
                    .visit_all_sub_expressions(callback.clone());
            }
            callback(&Expression { node: self.node });
        }
    }

    impl_ast_node_trait!(NewExpression);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ImplicitKind {
        NoImplicit,
        JSImplicit,
        Implicit,
    }

    pub struct ParameterList {
        pub names: Vec<Rc<RefCell<Identifier>>>,
        pub types: Vec<Rc<RefCell<TypeExpression>>>,
        pub implicit_kind: ImplicitKind,
        pub implicit_kind_pos: SourcePosition,
        pub implicit_count: usize,
        pub has_varargs: bool,
        pub arguments_variable: String,
    }

    impl ParameterList {
        pub fn empty() -> Self {
            ParameterList {
                names: Vec::new(),
                types: Vec::new(),
                implicit_kind: ImplicitKind::NoImplicit,
                implicit_kind_pos: SourcePosition::Invalid(),
                implicit_count: 0,
                has_varargs: false,
                arguments_variable: String::new(),
            }
        }

        pub fn get_implicit_types(&self) -> Vec<Rc<RefCell<TypeExpression>>> {
            self.types[..self.implicit_count].to_vec()
        }
        pub fn get_explicit_types(&self) -> Vec<Rc<RefCell<TypeExpression>>> {
            self.types[self.implicit_count..].to_vec()
        }
    }

    pub struct BasicTypeExpression {
        pub node: AstNode,
        pub namespace_qualification: Vec<String>,
        pub is_constexpr: bool,
        pub name: Rc<RefCell<Identifier>>,
        pub generic_arguments: Vec<Rc<RefCell<TypeExpression>>>,
    }

    impl BasicTypeExpression {
        pub fn new_with_namespace(
            pos: SourcePosition,
            namespace_qualification: Vec<String>,
            name: Rc<RefCell<Identifier>>,
            generic_arguments: Vec<Rc<RefCell<TypeExpression>>>,
        ) -> Self {
            BasicTypeExpression {
                node: AstNode::new(Kind::BasicTypeExpression, pos),
                namespace_qualification,
                is_constexpr: IsConstexprName(&name.borrow().value),
                name,
                generic_arguments,
            }
        }

        pub fn new(pos: SourcePosition, name: Rc<RefCell<Identifier>>) -> Self {
            BasicTypeExpression::new_with_namespace(pos, vec![], name, vec![])
        }
    }

    impl_ast_node_trait!(BasicTypeExpression);

    pub struct FunctionTypeExpression {
        pub node: AstNode,
        pub parameters: Vec<Rc<RefCell<TypeExpression>>>,
        pub return_type: Rc<RefCell<TypeExpression>>,
    }

    impl FunctionTypeExpression {
        pub fn new(
            pos: SourcePosition,
            parameters: Vec<Rc<RefCell<TypeExpression>>>,
            return_type: Rc<RefCell<TypeExpression>>,
        ) -> Self {
            FunctionTypeExpression {
                node: AstNode::new(Kind::FunctionTypeExpression, pos),
                parameters,
                return_type,
            }
        }
    }

    impl_ast_node_trait!(FunctionTypeExpression);

    // A PrecomputedTypeExpression is never created directly by the parser. Later
    // stages can use this to insert AST snippets where the type has already been
    // resolved.
    pub struct PrecomputedTypeExpression {
        pub node: AstNode,
        pub type_: Rc<RefCell<Type>>, // Assuming Type is defined elsewhere
    }

    impl PrecomputedTypeExpression {
        pub fn new(pos: SourcePosition, type_: Rc<RefCell<Type>>) -> Self {
            PrecomputedTypeExpression {
                node: AstNode::new(Kind::PrecomputedTypeExpression, pos),
                type_,
            }
        }
    }

    impl_ast_node_trait!(PrecomputedTypeExpression);

    pub struct UnionTypeExpression {
        pub node: AstNode,
        pub a: Rc<RefCell<TypeExpression>>,
        pub b: Rc<RefCell<TypeExpression>>,
    }

    impl UnionTypeExpression {
        pub fn new(
            pos: SourcePosition,
            a: Rc<RefCell<TypeExpression>>,
            b: Rc<RefCell<TypeExpression>>,
        ) -> Self {
            UnionTypeExpression {
                node: AstNode::new(Kind::UnionTypeExpression, pos),
                a,
                b,
            }
        }
    }

    impl_ast_node_trait!(UnionTypeExpression);

    pub struct ExpressionStatement {
        pub node: AstNode,
        pub expression: Rc<RefCell<Expression>>,
    }

    impl ExpressionStatement {
        pub fn new(pos: SourcePosition, expression: Rc<RefCell<Expression>>) -> Self {
            ExpressionStatement {
                node: AstNode::new(Kind::ExpressionStatement, pos),
                expression,
            }
        }
    }

    impl_ast_node_trait!(ExpressionStatement);

    pub struct IfStatement {
        pub node: AstNode,
        pub condition: Rc<RefCell<Expression>>,
        pub is_constexpr: bool,
        pub if_true: Rc<RefCell<Statement>>,
        pub if_false: Option<Rc<RefCell<Statement>>>,
    }

    impl IfStatement {
        pub fn new(
            pos: SourcePosition,
            is_constexpr: bool,
            condition: Rc<RefCell<Expression>>,
            if_true: Rc<RefCell<Statement>>,
            if_false: Option<Rc<RefCell<Statement>>>,
        ) -> Self {
            IfStatement {
                node: AstNode::new(Kind::IfStatement, pos),
                condition,
                is_constexpr,
                if_true,
                if_false,
            }
        }
    }

    impl_ast_node_trait!(IfStatement);

    pub struct WhileStatement
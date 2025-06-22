// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO(you): Add the necessary crate dependencies.

// mod ast;
// mod kythe_data;
// mod server_data;
// mod type_inference;
// mod type_visitor;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
// use ast::*;
// use kythe_data::*;
// use server_data::*;
// use type_inference::*;
// use type_visitor::*;

pub mod declarations {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    
    //use super::ast::*;

    thread_local! {
        static ALL_DECLARABLES: RefCell<Vec<Rc<Declarable>>> = RefCell::new(Vec::new());
        static NAMESPACES: RefCell<HashMap<String, Rc<Namespace>>> = RefCell::new(HashMap::new());
    }

    pub fn with_all_declarables<F, R>(f: F) -> R
    where
        F: FnOnce(&Vec<Rc<Declarable>>) -> R,
    {
        ALL_DECLARABLES.with(|declarables| f(&declarables.borrow()))
    }

    pub fn with_all_declarables_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Vec<Rc<Declarable>>) -> R,
    {
        ALL_DECLARABLES.with(|declarables| f(&mut declarables.borrow_mut()))
    }

    pub fn declare_namespace(name: String) -> Rc<Namespace> {
        NAMESPACES.with(|namespaces| {
            let mut namespaces_mut = namespaces.borrow_mut();
            if let Some(ns) = namespaces_mut.get(&name) {
                return Rc::clone(ns);
            }
            let namespace = Rc::new(Namespace {
                name: name.clone(),
            });
            namespaces_mut.insert(name, Rc::clone(&namespace));
            ALL_DECLARABLES.with(|declarables| {
                declarables.borrow_mut().push(Rc::clone(&namespace) as Rc<Declarable>);
            });
            namespace
        })
    }

    pub fn try_lookup_shallow(qualified_name: QualifiedName) -> Vec<Rc<Declarable>> {
        ALL_DECLARABLES.with(|declarables| {
            declarables.borrow().iter()
                .filter(|declarable| {
                    // Assuming Declarable has a method to get its name
                    // TODO(you): Implement name retrieval on Declarable
                    //declarable.name() == qualified_name.name
                    true
                })
                .map(|declarable| Rc::clone(declarable))
                .collect()
        })
    }

    // Assuming QualifiedName is a struct with a 'name' field.
    #[derive(Debug, Clone)]
    pub struct QualifiedName {
        pub name: String,
    }

    pub trait Declarable {
        // Define common methods for Declarable
    }

    pub struct Namespace {
        pub name: String,
    }

    impl Declarable for Namespace {}
}

pub mod current_source_position {
    use std::cell::RefCell;

    thread_local! {
        static CURRENT_POSITION: RefCell<Option<SourcePosition>> = RefCell::new(None);
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SourcePosition {
        pub line: usize,
        pub column: usize,
    }

    pub struct Scope {
        previous_position: Option<SourcePosition>,
    }

    impl Scope {
        pub fn new(position: SourcePosition) -> Self {
            let previous_position = CURRENT_POSITION.with(|pos| pos.borrow_mut().replace(position));
            Scope { previous_position }
        }
    }

    impl Drop for Scope {
        fn drop(&mut self) {
            CURRENT_POSITION.with(|pos| *pos.borrow_mut() = self.previous_position);
        }
    }

    impl SourcePosition {
        pub fn new(line: usize, column: usize) -> Self {
            SourcePosition { line, column }
        }
    }

    pub fn get() -> SourcePosition {
        CURRENT_POSITION.with(|pos| {
            pos.borrow().unwrap_or(SourcePosition { line: 0, column: 0 })
        })
    }

    pub fn set(position: SourcePosition) {
        CURRENT_POSITION.with(|pos| {
            *pos.borrow_mut() = Some(position);
        });
    }
}

pub mod global_context {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;

    thread_local! {
        static CPP_INCLUDES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
        static COLLECT_KYTHE_DATA_FLAG: RefCell<bool> = RefCell::new(false);
    }

    pub fn add_cpp_include(include_path: String) {
        CPP_INCLUDES.with(|includes| {
            includes.borrow_mut().insert(include_path);
        });
    }

    pub fn with_cpp_includes<F, R>(f: F) -> R
    where
        F: FnOnce(&HashSet<String>) -> R,
    {
        CPP_INCLUDES.with(|includes| f(&includes.borrow()))
    }

    pub fn collect_kythe_data() -> bool {
        COLLECT_KYTHE_DATA_FLAG.with(|flag| *flag.borrow())
    }

    pub fn set_collect_kythe_data(value: bool) {
        COLLECT_KYTHE_DATA_FLAG.with(|flag| *flag.borrow_mut() = value);
    }

    pub fn all_declarables() -> Vec<Rc<declarations::Declarable>> {
        declarations::with_all_declarables(|declarables| declarables.clone())
    }

    pub fn make_unique_name(base_name: String) -> String {
        // TODO(you): Implement unique name generation logic.
        format!("{}_unique", base_name)
    }

    pub fn collect_language_server_data() -> bool {
        //TODO Implement this when language server data is implemented
        false
    }
}

pub mod reporting {
    use super::current_source_position;
    use std::fmt;

    pub fn report_error<T: fmt::Display>(message: T) {
        let position = current_source_position::get();
        eprintln!("Error at line {}, column {}: {}", position.line, position.column, message);
    }
}

pub mod type_oracle {
    pub fn get_js_any_type() -> &'static Type {
        static JS_ANY_TYPE: Type = Type {
            name: "JSAny",
        };
        &JS_ANY_TYPE
    }

    pub fn get_void_type() -> &'static Type {
        static VOID_TYPE: Type = Type {
            name: "Void",
        };
        &VOID_TYPE
    }

    pub fn get_never_type() -> &'static Type {
        static NEVER_TYPE: Type = Type {
            name: "Never",
        };
        &NEVER_TYPE
    }

    pub fn get_context_type() -> &'static Type {
        static CONTEXT_TYPE: Type = Type {
            name: "Context",
        };
        &CONTEXT_TYPE
    }

    pub fn get_no_context_type() -> &'static Type {
        static NO_CONTEXT_TYPE: Type = Type {
            name: "NoContext",
        };
        &NO_CONTEXT_TYPE
    }

    pub fn get_strong_tagged_type() -> &'static Type {
        static STRONG_TAGGED_TYPE: Type = Type {
            name: "StrongTagged",
        };
        &STRONG_TAGGED_TYPE
    }

    pub fn get_native_context_type() -> &'static Type {
        static NATIVE_CONTEXT_TYPE: Type = Type {
            name: "NativeContext",
        };
        &NATIVE_CONTEXT_TYPE
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Type {
        name: &'static str,
    }

    impl Type {
        pub fn is_subtype_of(&self, other: &Type) -> bool {
            // TODO(you): Implement subtype checking logic.
            self.name == other.name || other.name == "JSAny"
        }
        pub fn is_constexpr(&self) -> bool {
            //TODO Implement constexpr type
            true
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }
}

pub mod current_scope {
    use std::cell::RefCell;
    use std::rc::Rc;

    thread_local! {
        static CURRENT_SCOPE: RefCell<Option<Rc<Scope>>> = RefCell::new(None);
    }

    pub struct Scope {
        previous_scope: Option<Rc<Scope>>,
        namespace: Rc<super::declarations::Namespace>, // Assuming Namespace is defined elsewhere
    }

    impl Scope {
        pub fn new(namespace: &Rc<super::declarations::Namespace>) -> Self {
            let previous_scope = CURRENT_SCOPE.with(|scope| scope.borrow_mut().take());
            Scope {
                previous_scope,
                namespace: Rc::clone(namespace),
            }
        }

        pub fn namespace(&self) -> &Rc<super::declarations::Namespace> {
            &self.namespace
        }
    }

    impl Drop for Scope {
        fn drop(&mut self) {
            CURRENT_SCOPE.with(|scope| *scope.borrow_mut() = self.previous_scope.take());
        }
    }

    pub fn get() -> Rc<Scope> {
        CURRENT_SCOPE.with(|scope| {
            scope.borrow().clone().expect("No current scope").clone()
        })
    }

    pub fn set(scope: Rc<Scope>) {
        CURRENT_SCOPE.with(|s| {
            *s.borrow_mut() = Some(scope);
        });
    }
}

pub mod ast {
    use super::current_source_position::SourcePosition;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum AstNodeKind {
        NamespaceDeclaration,
        GenericCallableDeclaration,
        GenericTypeDeclaration,

        // Add other node kinds as needed
        TorqueBuiltinDeclaration,
        ExternalBuiltinDeclaration,
        ExternalRuntimeDeclaration,
        ExternalMacroDeclaration,
        TorqueMacroDeclaration,
        IntrinsicDeclaration,
        ConstDeclaration,
        SpecializationDeclaration,
        ExternConstDeclaration,
        CppIncludeDeclaration,
    }

    pub trait AstNode {
        fn kind(&self) -> AstNodeKind;
    }

    #[derive(Debug, Clone)]
    pub struct NamespaceDeclaration {
        pub pos: SourcePosition,
        // Add other fields as needed
    }

    impl AstNode for NamespaceDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::NamespaceDeclaration
        }
    }

    impl NamespaceDeclaration {
        pub fn cast(decl: &Declaration) -> &NamespaceDeclaration {
            match decl {
                Declaration::Namespace(ns) => ns,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct GenericCallableDeclaration {
        pub pos: SourcePosition,
        // Add other fields as needed
    }

    impl AstNode for GenericCallableDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::GenericCallableDeclaration
        }
    }

    impl GenericCallableDeclaration {
        pub fn cast(decl: &Declaration) -> &GenericCallableDeclaration {
            match decl {
                Declaration::GenericCallable(gc) => gc,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct GenericTypeDeclaration {
        pub pos: SourcePosition,
        // Add other fields as needed
    }

    impl AstNode for GenericTypeDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::GenericTypeDeclaration
        }
    }

    impl GenericTypeDeclaration {
        pub fn cast(decl: &Declaration) -> &GenericTypeDeclaration {
            match decl {
                Declaration::GenericType(gt) => gt,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TorqueBuiltinDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
        pub javascript_linkage: bool,
        pub has_custom_interface_descriptor: bool,
        pub use_counter_name: Option<String>,
        pub body: Option<Statement>,
        pub parameters: ParameterList,
    }

    impl AstNode for TorqueBuiltinDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::TorqueBuiltinDeclaration
        }
    }

    impl TorqueBuiltinDeclaration {
        pub fn cast(decl: &Declaration) -> &TorqueBuiltinDeclaration {
            match decl {
                Declaration::TorqueBuiltin(tb) => tb,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExternalBuiltinDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
    }

    impl AstNode for ExternalBuiltinDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::ExternalBuiltinDeclaration
        }
    }

    impl ExternalBuiltinDeclaration {
        pub fn cast(decl: &Declaration) -> &ExternalBuiltinDeclaration {
            match decl {
                Declaration::ExternalBuiltin(eb) => eb,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExternalRuntimeDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
    }

    impl AstNode for ExternalRuntimeDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::ExternalRuntimeDeclaration
        }
    }

    impl ExternalRuntimeDeclaration {
        pub fn cast(decl: &Declaration) -> &ExternalRuntimeDeclaration {
            match decl {
                Declaration::ExternalRuntime(er) => er,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExternalMacroDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
        pub external_assembler_name: String,
        pub op: Option<String>,
    }

    impl AstNode for ExternalMacroDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::ExternalMacroDeclaration
        }
    }

    impl ExternalMacroDeclaration {
        pub fn cast(decl: &Declaration) -> &ExternalMacroDeclaration {
            match decl {
                Declaration::ExternalMacro(em) => em,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TorqueMacroDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
        pub export_to_csa: bool,
        pub body: Option<Statement>,
        pub op: Option<String>,
    }

    impl AstNode for TorqueMacroDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::TorqueMacroDeclaration
        }
    }

    impl TorqueMacroDeclaration {
        pub fn cast(decl: &Declaration) -> &TorqueMacroDeclaration {
            match decl {
                Declaration::TorqueMacro(tm) => tm,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct IntrinsicDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
    }

    impl AstNode for IntrinsicDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::IntrinsicDeclaration
        }
    }

    impl IntrinsicDeclaration {
        pub fn cast(decl: &Declaration) -> &IntrinsicDeclaration {
            match decl {
                Declaration::Intrinsic(i) => i,
                _ => panic!("invalid cast"),
            }
        }
        pub fn dynamic_cast(decl: &Declaration) -> Option<&IntrinsicDeclaration> {
          match decl {
              Declaration::Intrinsic(i) => Some(i),
              _ => None,
          }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ConstDeclaration {
        pub name: String,
        pub pos: SourcePosition,
        //TODO Implement type
        pub type_: TypeExpression,
        pub expression: Expression,
    }

    impl AstNode for ConstDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::ConstDeclaration
        }
    }

    impl ConstDeclaration {
        pub fn cast(decl: &Declaration) -> &ConstDeclaration {
            match decl {
                Declaration::Const(c) => c,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct SpecializationDeclaration {
        pub name: Identifier,
        pub pos: SourcePosition,
        pub generic_parameters: Vec<TypeExpression>,
        pub body: Option<Statement>,
    }

    impl AstNode for SpecializationDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::SpecializationDeclaration
        }
    }

    impl SpecializationDeclaration {
        pub fn cast(decl: &Declaration) -> &SpecializationDeclaration {
            match decl {
                Declaration::Specialization(s) => s,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExternConstDeclaration {
        pub name: String,
        pub pos: SourcePosition,
        //TODO Implement type
        pub type_: TypeExpression,
        pub literal: String,
    }

    impl AstNode for ExternConstDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::ExternConstDeclaration
        }
    }

    impl ExternConstDeclaration {
        pub fn cast(decl: &Declaration) -> &ExternConstDeclaration {
            match decl {
                Declaration::ExternConst(ec) => ec,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CppIncludeDeclaration {
        pub include_path: String,
        pub pos: SourcePosition,
    }

    impl AstNode for CppIncludeDeclaration {
        fn kind(&self) -> AstNodeKind {
            AstNodeKind::CppIncludeDeclaration
        }
    }

    impl CppIncludeDeclaration {
        pub fn cast(decl: &Declaration) -> &CppIncludeDeclaration {
            match decl {
                Declaration::CppInclude(ci) => ci,
                _ => panic!("invalid cast"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Identifier {
        pub value: String,
        pub pos: SourcePosition,
    }
    
    impl fmt::Display for Identifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    #[derive(Debug, Clone)]
    pub enum Declaration {
        Namespace(NamespaceDeclaration),
        GenericCallable(GenericCallableDeclaration),
        GenericType(GenericTypeDeclaration),
        TorqueBuiltin(TorqueBuiltinDeclaration),
        ExternalBuiltin(ExternalBuiltinDeclaration),
        ExternalRuntime(ExternalRuntimeDeclaration),
        ExternalMacro(ExternalMacroDeclaration),
        TorqueMacro(TorqueMacroDeclaration),
        Intrinsic(IntrinsicDeclaration),
        Const(ConstDeclaration),
        Specialization(SpecializationDeclaration),
        ExternConst(ExternConstDeclaration),
        CppInclude(CppIncludeDeclaration),
    }

    impl Declaration {
        pub fn kind(&self) -> AstNodeKind {
            match self {
                Declaration::Namespace(_) => AstNodeKind::NamespaceDeclaration,
                Declaration::GenericCallable(_) => AstNodeKind::GenericCallableDeclaration,
                Declaration::GenericType(_) => AstNodeKind::GenericTypeDeclaration,
                Declaration::TorqueBuiltin(_) => AstNodeKind::TorqueBuiltinDeclaration,
                Declaration::ExternalBuiltin(_) => AstNodeKind::ExternalBuiltinDeclaration,
                Declaration::ExternalRuntime(_) => AstNodeKind::ExternalRuntimeDeclaration,
                Declaration::ExternalMacro(_) => AstNodeKind::ExternalMacroDeclaration,
                Declaration::TorqueMacro(_) => AstNodeKind::TorqueMacroDeclaration,
                Declaration::Intrinsic(_) => AstNodeKind::IntrinsicDeclaration,
                Declaration::Const(_) => AstNodeKind::ConstDeclaration,
                Declaration::Specialization(_) => AstNodeKind::SpecializationDeclaration,
                Declaration::ExternConst(_) => AstNodeKind::ExternConstDeclaration,
                Declaration::CppInclude(_) => AstNodeKind::CppIncludeDeclaration,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Statement {
        //TODO Implement
    }

    #[derive(Debug, Clone)]
    pub struct Expression {
        //TODO Implement
    }
    
    #[derive(Debug, Clone)]
    pub struct ParameterList {
        pub types: Vec<TypeExpression>,
        pub has_varargs: bool,
    }
    
    #[derive(Debug, Clone)]
    pub struct TypeExpression {
      //TODO Implement
    }
}

pub mod type_visitor {
    use super::ast::*;
    use super::type_oracle::*;

    #[derive(Debug, Clone)]
    pub struct Signature {
        pub parameter_types: ParameterTypes,
        pub return_type: &'static Type,
        pub implicit_count: usize,
        pub parameter_names: Vec<String>,
    }

    impl Signature {
        pub fn has_same_types_as(&self, other: &Signature, mode: ParameterMode) -> bool {
          // TODO Implement this
          true
        }
        pub fn types(&self) -> &Vec<&'static Type> {
          &self.parameter_types.types
        }
    }

    impl fmt::Display for Signature {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature")
      }
    }

    #[derive(Debug, Clone)]
    pub struct ParameterTypes {
        pub types: Vec<&'static Type>,
    }

    pub enum ParameterMode {
        kIgnoreImplicit
    }

    pub fn make_signature(decl: &ExternalBuiltinDeclaration) -> Signature {
        // TODO(you): Implement signature creation logic.
        Signature {
            parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
            return_type: get_js_any_type(),
            implicit_count: 0,
            parameter_names: vec![],
        }
    }

    pub fn make_signature(decl: &ExternalRuntimeDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn make_signature(decl: &ExternalMacroDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn make_signature(decl: &TorqueBuiltinDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn make_signature(decl: &TorqueMacroDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn make_signature(decl: &IntrinsicDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn make_signature(decl: &SpecializationDeclaration) -> Signature {
      // TODO(you): Implement signature creation logic.
      Signature {
          parameter_types: ParameterTypes { types: vec![get_js_any_type()] },
          return_type: get_js_any_type(),
          implicit_count: 0,
          parameter_names: vec![],
      }
    }

    pub fn compute_type(type_expression: &TypeExpression) -> &'static Type {
        //TODO Implement
        get_js_any_type()
    }

    pub fn compute_type_vector(generic_parameters: &Vec<TypeExpression>) -> Vec<&'static Type> {
        //TODO Implement
        vec![]
    }
}

pub mod declaration_visitor {
    use super::ast::*;
    use super::current_scope;
    use super::current_source_position;
    use super::declarations;
    use super::global_context;
    use super::reporting;
    use super::type_oracle;
    use super::type_visitor;
    use std::borrow::Cow;
    use std::option::Option;
    use std::rc::Rc;

    /// This struct represents a request to specialize a generic. It's used
    /// to propagate the name of the requester (the outer function which caused
    /// the specialization). This name is then used in various error messages,
    /// for example when a missing specialization is encountered during code
    /// generation.
    #[derive(Debug, Clone)]
    pub struct SpecializationRequester {
      pub position: current_source_position::SourcePosition,
      pub scope: Rc<current_scope::Scope>,
      pub name: String,
    }

    pub struct PredeclarationVisitor {}

    impl PredeclarationVisitor {
        pub fn predeclare(&mut self, decl: &Declaration) {
            let scope = current_source_position::Scope::new(match decl {
                Declaration::Namespace(ns) => ns.pos,
                Declaration::GenericCallable(gc) => gc.pos,
                Declaration::GenericType(gt) => gt.pos,
                Declaration::TorqueBuiltin(tb) => tb.pos,
                Declaration::ExternalBuiltin(eb) => eb.pos,
                Declaration::ExternalRuntime(er) => er.pos,
                Declaration::ExternalMacro(em) => em.pos,
                Declaration::TorqueMacro(tm) => tm.pos,
                Declaration::Intrinsic(i) => i.pos,
                Declaration::Const(c) => c.pos,
                Declaration::Specialization(s) => s.pos,
                Declaration::ExternConst(ec) => ec.pos,
                Declaration::CppInclude(ci) => ci.pos,
            });
            match decl {
                Declaration::Namespace(_) => self.predeclare_namespace(NamespaceDeclaration::cast(decl)),
                Declaration::GenericCallable(_) => self.predeclare_generic_callable(GenericCallableDeclaration::cast(decl)),
                Declaration::GenericType(_) => self.predeclare_generic_type(GenericTypeDeclaration::cast(decl)),
                _ => {
                    // Only processes type declaration nodes, namespaces and generics.
                }
            }
        }

        fn predeclare_namespace(&mut self, decl: &NamespaceDeclaration) {
            //TODO implement
        }

        fn predeclare_generic_callable(&mut self, decl: &GenericCallableDeclaration) {
            //TODO implement
        }

        fn predeclare_generic_type(&mut self, decl: &GenericTypeDeclaration) {
            //TODO implement
        }

        pub fn resolve_predeclarations(&mut self) {
            let all_declarables = global_context::all_declarables();
            for declarable in all_declarables {
              //TODO Implement this
              //  if let Some(alias) = TypeAlias::DynamicCast(declarable) {
              //      CurrentScope::Scope scope_activator(alias->ParentScope());
              //      CurrentSourcePosition::Scope position_activator(alias->Position());
              //      alias->Resolve();
              //  }
            }
        }
    }

    pub struct DeclarationVisitor {}

    impl DeclarationVisitor {
        pub fn visit(&mut self, decl: &Declaration) {
            let scope = current_source_position::Scope::new(match decl {
                Declaration::Namespace(ns) => ns.pos,
                Declaration::GenericCallable(gc) => gc.pos,
                Declaration::GenericType(gt) => gt.pos,
                Declaration::TorqueBuiltin(tb) => tb.pos,
                Declaration::ExternalBuiltin(eb) => eb.pos,
                Declaration::ExternalRuntime(er) => er.pos,
                Declaration::ExternalMacro(em) => em.pos,
                Declaration::TorqueMacro(tm) => tm.pos,
                Declaration::Intrinsic(i) => i.pos,
                Declaration::Const(c) => c.pos,
                Declaration::Specialization(s) => s.pos,
                Declaration::ExternConst(ec) => ec.pos,
                Declaration::CppInclude(ci) => ci.pos,
            });
            match decl {
                Declaration::Namespace(_) => self.visit_namespace(NamespaceDeclaration::cast(decl)),
                Declaration::GenericCallable(_) => self.visit_generic_callable(GenericCallableDeclaration::cast(decl)),
                Declaration::GenericType(_) => self.visit_generic_type(GenericTypeDeclaration::cast(decl)),
                Declaration::TorqueBuiltin(_) => self.visit_torque_builtin(TorqueBuiltinDeclaration::cast(decl)),
                Declaration::ExternalBuiltin(_) => self.visit_external_builtin(ExternalBuiltinDeclaration::cast(decl)),
                Declaration::ExternalRuntime(_) => self.visit_external_runtime(ExternalRuntimeDeclaration::cast(decl)),
                Declaration::ExternalMacro(_) => self.visit_external_macro(ExternalMacroDeclaration::cast(decl)),
                Declaration::TorqueMacro(_) => self.visit_torque_macro(TorqueMacroDeclaration::cast(decl)),
                Declaration::Intrinsic(_) => self.visit_intrinsic(IntrinsicDeclaration::cast(decl)),
                Declaration::Const(_) => self.visit_const(ConstDeclaration::cast(decl)),
                Declaration::Specialization(_) => self.visit_specialization(SpecializationDeclaration::cast(decl)),
                Declaration::ExternConst(_) => self.visit_extern_const(ExternConstDeclaration::cast(decl)),
                Declaration::CppInclude(_) => self.visit_cpp_include(CppIncludeDeclaration::cast(decl)),
            }
        }

        fn visit_namespace(&mut self, decl: &NamespaceDeclaration) {
            //TODO implement
        }

        fn visit_generic_callable(&mut self, decl: &GenericCallableDeclaration) {
            //TODO implement
        }

        fn visit_generic_type(&mut self, decl: &GenericTypeDeclaration) {
            //TODO implement
        }

        fn create_builtin<'a>(
            &mut self,
            decl: &TorqueBuiltinDeclaration,
            external_name: String,
            readable_name: String,
            signature: type_visitor::Signature,
            use_counter_name: Option<String>,
            body: Option<&'a Statement>,
        ) -> Builtin {
            let javascript = decl.javascript_linkage;
            let varargs = decl.parameters.has_varargs;
            let kind = if !javascript {
                BuiltinKind::Stub
            } else if varargs {
                BuiltinKind::VarArgsJavaScript
            } else {
                BuiltinKind::FixedArgsJavaScript
            };
            let mut has_custom_interface_descriptor = false;
            if decl.kind() == AstNodeKind::TorqueBuiltinDeclaration {
                has_custom_interface_descriptor =
                    TorqueBuiltinDeclaration::cast(&Declaration::TorqueBuiltin(decl.clone()))
                        .has_custom_interface_descriptor;
            }

            if varargs && !javascript {
                reporting::report_error(format!("Rest parameters require {} to be a JavaScript builtin", decl.name));
            }

            if javascript {
                if !signature.return_type.is_subtype_of(type_oracle::get_js_any_type()) {
                    reporting::report_error(format!(
                        "Return type of JavaScript-linkage builtins has to be JSAny.",
                    ));
                }
                // Validate the parameter types. In general, for JS builtins the parameters
                // must all be tagged values (JSAny). However, we currently allow declaring
                // "extern javascript" builtins with any parameter types. The reason is
                // that those are typically used for tailcalls, in which case we typically
                // need to supply the implicit parameters of the JS calling convention
                // (target, receiver, argc, etc.). It would probablu be nicer if we could
                // instead declare these parameters as js-implicit (like we do for
                // torque-defined javascript builtins) and then allow explicitly supplying
                // the implicit arguments during tailscalls. It's unclear though if that's
                // worth the effort. In particular, calls and tailcalls to javascript
                // builtins will emit CSA::CallJSBuiltin and CSA::TailCallJSBuiltin calls
                // which will validate the parameter types at C++ compile time.
                if decl.kind() != AstNodeKind::ExternalBuiltinDeclaration {
                  //TODO
                  //  for (size_t i = signature.implicit_count;
                  //       i < signature.parameter_types.types.size(); ++i) {
                  //    const Type* parameter_type = signature.parameter_types.types[i];
                  //    if
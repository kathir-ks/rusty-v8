// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

mod global_context;
mod server_data;
mod type_oracle;
mod declarable;
mod scope;

use declarable::*;
use global_context::*;
use scope::*;
use type_oracle::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedName {
    pub name: String,
}

impl<T: Into<String>> From<T> for QualifiedName {
    fn from(s: T) -> Self {
        QualifiedName { name: s.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub value: String,
    pub pos: usize, // Placeholder for position information
}

impl Identifier {
    pub fn new(value: String, pos: usize) -> Self {
        Identifier { value, pos }
    }
}

// Placeholder for Statement type
#[derive(Debug, Clone)]
pub struct Statement {}

// Placeholder for Expression type
#[derive(Debug, Clone)]
pub struct Expression {}

// Placeholder for BuiltinPointerType type
#[derive(Debug, Clone)]
pub struct BuiltinPointerType {}

impl BuiltinPointerType {
    pub fn return_type(&self) -> &Type {
        todo!() // Placeholder
    }
    pub fn parameter_types(&self) -> &TypeVector {
        todo!() // Placeholder
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub return_type: Rc<Type>,
    pub parameter_types: TypeVector,
}

impl Signature {
    pub fn get_explicit_types(&self) -> TypeVector {
        self.parameter_types.types.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVector {
    pub types: Vec<Rc<Type>>,
    pub var_args: bool,
}

impl TypeVector {
    pub fn new(types: Vec<Rc<Type>>, var_args: bool) -> Self {
        TypeVector { types, var_args }
    }
}

// Placeholder for Builtin::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinKind {
    kStub,
}

// Placeholder for Builtin::Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinFlags {}

// Placeholder for TypeDeclaration
#[derive(Debug, Clone)]
pub struct TypeDeclaration {}

pub struct Declarations {}

impl Declarations {
    pub fn lookup_global_scope(name: &QualifiedName) -> Vec<DeclarableRef> {
        let mut d = GlobalContext::get_default_namespace().lookup(name);
        if d.is_empty() {
            Self::report_error(format!("cannot find \"{}\" in global scope", name.name));
        }
        d
    }

    pub fn lookup_type_alias(name: &QualifiedName) -> Rc<TypeAlias> {
        let declaration = Self::ensure_unique(
            Self::filter_declarables::<TypeAlias>(Self::lookup(name)),
            &name.name,
            "type",
        );
        declaration
    }

    pub fn lookup_type(name: &QualifiedName) -> Rc<Type> {
        Self::lookup_type_alias(name).borrow().type_.clone()
    }

    pub fn lookup_type(name: &Identifier) -> Rc<Type> {
        let alias = Self::lookup_type_alias(QualifiedName::from(name.value.clone()));
        if GlobalContext::collect_language_server_data() {
            //LanguageServerData::add_definition(name.pos, alias.get_declaration_position()); // TODO: Implement LanguageServerData
        }
        alias.borrow().type_.clone()
    }

    pub fn try_lookup_type(name: &QualifiedName) -> Option<Rc<Type>> {
        let decls = Self::filter_declarables::<TypeAlias>(Self::try_lookup(name));
        if decls.is_empty() {
            return None;
        }
        Some(Self::ensure_unique(decls, &name.name, "type").borrow().type_.clone())
    }

    pub fn lookup_global_type(name: &QualifiedName) -> Rc<Type> {
        let declaration = Self::ensure_unique(
            Self::filter_declarables::<TypeAlias>(Self::lookup_global_scope(name)),
            &name.name,
            "type",
        );
        declaration.borrow().type_.clone()
    }

    pub fn find_some_internal_builtin_with_type(type_: &BuiltinPointerType) -> Option<Rc<Builtin>> {
        for declarable in GlobalContext::all_declarables() {
            if let Some(builtin) = Declarable::dynamic_cast::<Builtin>(&declarable) {
                if !builtin.borrow().is_external()
                    && builtin.borrow().kind() == BuiltinKind::kStub
                    && builtin.borrow().signature().return_type == type_.return_type()
                    && builtin.borrow().signature().parameter_types.types == type_.parameter_types().types
                {
                    return Some(builtin);
                }
            }
        }
        None
    }

    pub fn lookup_value(name: &QualifiedName) -> Rc<Value> {
        Self::ensure_unique(Self::filter_declarables::<Value>(Self::lookup(name)), &name.name, "value")
    }

    pub fn try_lookup_macro(name: &str, types: &TypeVector) -> Option<Rc<Macro>> {
        let macros = Self::try_lookup::<Macro>(QualifiedName::from(name.to_string()));
        for m in &macros {
            let signature_types = m.borrow().signature().get_explicit_types();
            if signature_types == types.types && !m.borrow().signature().parameter_types.var_args {
                return Some(m.clone());
            }
        }
        None
    }

    pub fn try_lookup_builtin(name: &QualifiedName) -> Option<Rc<Builtin>> {
        let builtins = Self::try_lookup::<Builtin>(name.clone());
        if builtins.is_empty() {
            return None;
        }
        Some(Self::ensure_unique(builtins, &name.name, "builtin"))
    }

    pub fn lookup_generic(name: &str) -> Vec<Rc<GenericCallable>> {
        Self::ensure_nonempty(
            Self::filter_declarables::<GenericCallable>(Self::lookup(&QualifiedName::from(name.to_string()))),
            name.to_string(),
            "generic callable",
        )
    }

    pub fn lookup_unique_generic(name: &QualifiedName) -> Rc<GenericCallable> {
        Self::ensure_unique(
            Self::filter_declarables::<GenericCallable>(Self::lookup(name)),
            &name.name,
            "generic callable",
        )
    }

    pub fn lookup_unique_generic_type(name: &QualifiedName) -> Rc<GenericType> {
        Self::ensure_unique(
            Self::filter_declarables::<GenericType>(Self::lookup(name)),
            &name.name,
            "generic type",
        )
    }

    pub fn lookup_global_unique_generic_type(name: &str) -> Rc<GenericType> {
        Self::ensure_unique(
            Self::filter_declarables::<GenericType>(Self::lookup_global_scope(&QualifiedName::from(name.to_string()))),
            name.to_string(),
            "generic type",
        )
    }

    pub fn try_lookup_generic_type(name: &QualifiedName) -> Option<Rc<GenericType>> {
        let results = Self::try_lookup::<GenericType>(name.clone());
        if results.is_empty() {
            return None;
        }
        Some(Self::ensure_unique(results, &name.name, "generic type"))
    }

    pub fn declare_namespace(name: String) -> Rc<Namespace> {
        Self::declare(name, Rc::new(Namespace::new(name.clone())))
    }

    pub fn declare_type(name: &Identifier, type_: Rc<Type>) -> Rc<TypeAlias> {
        Self::check_already_declared::<TypeAlias>(&name.value, "type");
        Self::declare(
            name.value.clone(),
            Rc::new(TypeAlias::new(type_, true, name.pos)),
        )
    }

    pub fn predeclare_type_alias(
        name: &Identifier,
        type_: &TypeDeclaration,
        redeclaration: bool,
    ) -> Rc<TypeAlias> {
        Self::check_already_declared::<TypeAlias>(&name.value, "type");
        Self::declare(
            name.value.clone(),
            Rc::new(TypeAlias::new(Rc::new(Type::Declaration(type_.clone())), redeclaration, name.pos)), //FIXME: Use Type::Declaration as Rc<Type>
        )
    }

    pub fn create_torque_macro(
        external_name: String,
        readable_name: String,
        exported_to_csa: bool,
        signature: Signature,
        body: Option<&Statement>,
        is_user_defined: bool,
    ) -> Rc<TorqueMacro> {
        let external_name = GlobalContext::make_unique_name(external_name);
        GlobalContext::register_declarable(Rc::new(TorqueMacro::new(
            external_name,
            readable_name,
            signature,
            body.map(|b| b.clone()),
            is_user_defined,
            exported_to_csa,
        )))
    }

    pub fn create_extern_macro(
        name: String,
        external_assembler_name: String,
        signature: Signature,
    ) -> Rc<ExternMacro> {
        GlobalContext::register_declarable(Rc::new(ExternMacro::new(
            name,
            external_assembler_name,
            signature,
        )))
    }

    pub fn declare_macro(
        name: &str,
        accessible_from_csa: bool,
        external_assembler_name: Option<String>,
        signature: &Signature,
        body: Option<&Statement>,
        op: Option<String>,
        is_user_defined: bool,
    ) -> Rc<Macro> {
        if let Some(existing_macro) = Self::try_lookup_macro(name, &signature.parameter_types) {
            if existing_macro.borrow().parent_scope() == CurrentScope::get() {
                Self::report_error(format!(
                    "cannot redeclare macro {} with identical explicit parameters",
                    name
                ));
            }
        }
        let macro_: Rc<Macro> = match external_assembler_name {
            Some(external_assembler_name) => Self::create_extern_macro(
                name.to_string(),
                external_assembler_name,
                signature.clone(),
            ),
            None => Self::create_torque_macro(
                name.to_string(),
                name.to_string(),
                accessible_from_csa,
                signature.clone(),
                body,
                is_user_defined,
            ),
        };

        Self::declare(name.to_string(), macro_.clone());
        if let Some(op) = op {
            if Self::try_lookup_macro(&op, &signature.parameter_types).is_some() {
                Self::report_error(format!(
                    "cannot redeclare operator {} with identical explicit parameters",
                    name
                ));
            }
            Self::declare_operator(op, &macro_);
        }
        macro_
    }

    pub fn create_method(
        container_type: Rc<AggregateType>,
        name: &str,
        signature: Signature,
        body: &Statement,
    ) -> Rc<Method> {
        let generated_name =
            GlobalContext::make_unique_name(format!("Method_{}_{}", container_type.simple_name(), name));
        let result = GlobalContext::register_declarable(Rc::new(Method::new(
            container_type.clone(),
            generated_name,
            name.to_string(),
            signature,
            body.clone(),
        )));
        container_type.register_method(result.clone());
        result
    }

    pub fn create_intrinsic(name: String, signature: Signature) -> Rc<Intrinsic> {
        GlobalContext::register_declarable(Rc::new(Intrinsic::new(name, signature)))
    }

    pub fn declare_intrinsic(name: String, signature: Signature) -> Rc<Intrinsic> {
        let result = Self::create_intrinsic(name.clone(), signature);
        Self::declare(name, result.clone());
        result
    }

    pub fn create_builtin(
        external_name: String,
        readable_name: String,
        kind: BuiltinKind,
        flags: BuiltinFlags,
        signature: Signature,
        use_counter_name: Option<String>,
        body: Option<&Statement>,
    ) -> Rc<Builtin> {
        GlobalContext::register_declarable(Rc::new(Builtin::new(
            external_name,
            readable_name,
            kind,
            flags,
            signature,
            use_counter_name,
            body.map(|b| b.clone()),
        )))
    }

    pub fn declare_runtime_function(name: String, signature: Signature) -> Rc<RuntimeFunction> {
        Self::check_already_declared::<RuntimeFunction>(&name, "runtime function");
        Self::declare(
            name.clone(),
            GlobalContext::register_declarable(Rc::new(RuntimeFunction::new(name, signature))),
        )
    }

    pub fn declare_extern_constant(name: &Identifier, type_: Rc<Type>, value: String) -> Rc<ExternConstant> {
        Self::check_already_declared::<Value>(&name.value, "constant");
        Self::declare(
            name.value.clone(),
            Rc::new(ExternConstant::new(name, type_, value)),
        )
    }

    pub fn declare_namespace_constant(
        name: &Identifier,
        type_: Rc<Type>,
        body: &Expression,
    ) -> Rc<NamespaceConstant> {
        Self::check_already_declared::<Value>(&name.value, "constant");
        let external_name = GlobalContext::make_unique_name(name.value.clone());
        let namespace_constant = Rc::new(NamespaceConstant::new(name, external_name, type_, body.clone()));
        let result = namespace_constant.clone();
        Self::declare(name.value.clone(), namespace_constant);
        result
    }

    pub fn declare_generic_callable(
        name: String,
        ast_node: &GenericCallableDeclaration,
    ) -> Rc<GenericCallable> {
        Self::declare(
            name.clone(),
            Rc::new(GenericCallable::new(name, ast_node)),
        )
    }

    pub fn declare_generic_type(name: String, ast_node: &GenericTypeDeclaration) -> Rc<GenericType> {
        Self::declare(
            name.clone(),
            Rc::new(GenericType::new(name, ast_node)),
        )
    }

    pub fn get_generated_callable_name(name: &str, specialized_types: &[Rc<Type>]) -> String {
        let mut result = name.to_string();
        for type_ in specialized_types {
            result += "_" + &type_.simple_name();
        }
        result
    }

    pub fn declare_operator(name: String, m: &Rc<Macro>) -> Rc<Macro> {
        GlobalContext::get_default_namespace().add_declarable(name, m.clone());
        m.clone()
    }

    fn ensure_nonempty<T>(
        list: Vec<Rc<T>>,
        name: String,
        kind: &str,
    ) -> Vec<Rc<T>> {
        if list.is_empty() {
            Self::report_error(format!("there is no {} named {}", kind, name));
        }
        list
    }

    fn ensure_unique<T>(
        list: Vec<Rc<T>>,
        name: &str,
        kind: &str,
    ) -> Rc<T> {
        if list.is_empty() {
            Self::report_error(format!("there is no {} named {}", kind, name));
        }
        if list.len() >= 2 {
            Self::report_error(format!("ambiguous reference to {} {}", kind, name));
        }
        list.first().unwrap().clone()
    }

    fn check_already_declared<T: Declarable>(name: &str, new_type: &str) {
        let declarations = Self::filter_declarables::<T>(Self::try_lookup_shallow(QualifiedName::from(name.to_string())));
        if !declarations.is_empty() {
            let scope = CurrentScope::get();
            Self::report_error(format!("cannot redeclare {} (type {} {:?})", name, new_type, scope));
        }
    }

    fn declare<T: Declarable>(name: String, declarable: Rc<T>) -> Rc<T> {
        CurrentScope::add_declarable(name, &declarable);
        declarable
    }

    fn lookup(name: &QualifiedName) -> Vec<DeclarableRef> {
        CurrentScope::lookup(name)
    }

    fn try_lookup<T: Declarable>(name: QualifiedName) -> Vec<Rc<T>> {
        let mut result = Vec::new();
        for d in Self::lookup(&name) {
            if let Some(t) = Declarable::dynamic_cast::<T>(&d) {
                result.push(t);
            }
        }
        result
    }

    fn try_lookup_shallow(name: QualifiedName) -> Vec<DeclarableRef> {
        CurrentScope::try_lookup(name)
    }

    fn filter_declarables<T: Declarable>(declarables: Vec<DeclarableRef>) -> Vec<Rc<T>> {
        let mut result = Vec::new();
        for d in declarables {
            if let Some(t) = Declarable::dynamic_cast::<T>(&d) {
                result.push(t);
            }
        }
        result
    }

    fn report_error(message: String) -> ! {
        panic!("{}", message);
    }
}

impl Declarations {
    //This function is just a placeholder and should be replaced with actual declarations implementation
    pub fn new() -> Declarations {
        Declarations {}
    }
}
// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::*;
use crate::global_context::*;
use crate::type_inference::*;
use crate::type_visitor::*;

pub struct QualifiedName {
    namespace_qualification: Vec<String>,
    name: String,
}

impl QualifiedName {
    pub fn parse(qualified_name: String) -> Self {
        let mut qualifications = Vec::new();
        let mut current_name = qualified_name;
        while let Some(namespace_delimiter_index) = current_name.find("::") {
            qualifications.push(current_name[0..namespace_delimiter_index].to_string());
            current_name = current_name[namespace_delimiter_index + 2..].to_string();
        }
        QualifiedName {
            namespace_qualification: qualifications,
            name: current_name,
        }
    }

    pub fn drop_first_namespace_qualification(&self) -> Self {
        let mut new_qualifications = self.namespace_qualification.clone();
        new_qualifications.remove(0);
        QualifiedName {
            namespace_qualification: new_qualifications,
            name: self.name.clone(),
        }
    }
}

impl Display for QualifiedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for qualifier in &self.namespace_qualification {
            write!(f, "{}::", qualifier)?;
        }
        write!(f, "{}", self.name)
    }
}

pub trait Declarable {
    fn readable_name(&self) -> String;
}

pub struct CallableSignature {
    pub parameter_types: TypeVector,
    pub return_type: Rc<Type>,
    pub implicit_count: usize,
}

pub trait Callable: Declarable {
    fn signature(&self) -> &CallableSignature;
}

impl dyn Callable {
    fn readable_name(&self) -> String {
        Declarable::readable_name(self)
    }
}

impl Display for dyn Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "callable {} (", self.readable_name())?;
        if self.signature().implicit_count != 0 {
            write!(f, "implicit ")?;
            let implicit_parameter_types: Vec<Rc<Type>> = self.signature().parameter_types.types[..self.signature().implicit_count].to_vec();
            write!(f, "{}) (", TypeVector { types: implicit_parameter_types })?;
            let explicit_parameter_types: Vec<Rc<Type>> = self.signature().parameter_types.types[self.signature().implicit_count..].to_vec();
            write!(f, "{}", TypeVector { types: explicit_parameter_types })?;
        } else {
            write!(f, "{}", self.signature().parameter_types)?;
        }
        write!(f, "): {}", *self.signature().return_type)
    }
}

pub struct Builtin {
    signature: CallableSignature,
    readable_name: String
}

impl Builtin {
    pub fn new(signature: CallableSignature, readable_name: String) -> Self {
        Builtin { signature, readable_name }
    }
}

impl Callable for Builtin {
    fn signature(&self) -> &CallableSignature {
        &self.signature
    }
}

impl Declarable for Builtin {
    fn readable_name(&self) -> String {
        self.readable_name.clone()
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "builtin {} {} {}", *self.signature().return_type, self.readable_name(), self.signature().parameter_types)
    }
}

pub struct RuntimeFunction {
    signature: CallableSignature,
    readable_name: String
}

impl RuntimeFunction {
    pub fn new(signature: CallableSignature, readable_name: String) -> Self {
        RuntimeFunction { signature, readable_name }
    }
}

impl Callable for RuntimeFunction {
    fn signature(&self) -> &CallableSignature {
        &self.signature
    }
}

impl Declarable for RuntimeFunction {
    fn readable_name(&self) -> String {
        self.readable_name.clone()
    }
}

impl Display for RuntimeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "runtime function {} {} {}", *self.signature().return_type, self.readable_name(), self.signature().parameter_types)
    }
}

pub struct GenericCallable {
    name: String,
    generic_parameters: Vec<GenericParameter>,
    constraints: Vec<TypeConstraint>,
    parent_scope: Option<Rc<Scope>>,
    declaration: Rc<TorqueMacroDeclaration> // Assuming TorqueMacroDeclaration holds the body.  Needs adjustment if not
}

impl GenericCallable {
    pub fn new(name: String, generic_parameters: Vec<GenericParameter>, constraints: Vec<TypeConstraint>, parent_scope: Option<Rc<Scope>>, declaration: Rc<TorqueMacroDeclaration>) -> Self {
        GenericCallable { name, generic_parameters, constraints, parent_scope, declaration }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn generic_parameters(&self) -> &Vec<GenericParameter> {
        &self.generic_parameters
    }

    pub fn constraints(&self) -> &Vec<TypeConstraint> {
        &self.constraints
    }

    pub fn declaration(&self) -> &TorqueMacroDeclaration {
        &self.declaration
    }

    pub fn infer_specialization_types(
        &self,
        explicit_specialization_types: &TypeVector,
        arguments: &[Option<Rc<Type>>],
    ) -> TypeArgumentInference {
        let parameters = &self.declaration().parameters.types;
        // Assuming CurrentScope and CurrentSourcePosition are handled elsewhere.
        // CurrentScope::Scope generic_scope(self.parent_scope.as_ref().map(|s| s.deref()));
        let inference = TypeArgumentInference::new(
            &self.generic_parameters(),
            explicit_specialization_types,
            parameters,
            arguments,
        );
        if !inference.has_failed() {
            if let Some(violation) =
                find_constraint_violation(inference.get_result(), self.constraints())
            {
                return inference.fail(format!("Could not instantiate generic, {}.", violation));
            }
        }
        inference
    }

    pub fn callable_body(&self) -> Option<&Statement> {
        TorqueMacroDeclaration::dynamic_cast(self.declaration()).map(|macro_decl| &macro_decl.body)
    }
}

impl Display for GenericCallable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "generic {}<", self.name())?;
        print_comma_separated_list(f, self.generic_parameters(), |identifier| {
            identifier.name.value.clone()
        })?;
        write!(f, ">")
    }
}

pub struct SpecializationRequester {
    position: SourcePosition,
    name: String,
    scope: Option<Rc<Scope>>,
}

impl SpecializationRequester {
    pub fn new(position: SourcePosition, s: Option<Rc<Scope>>, name: String) -> Self {
        let mut current_scope = s.clone();
        // Skip scopes that are not related to template specializations, they might be
        // stack-allocated and not live for long enough.
        while let Some(scope) = current_scope {
            if scope.get_specialization_requester().is_some() {
                break;
            }
            current_scope = scope.parent_scope().cloned();
        }
        SpecializationRequester {
            position,
            name,
            scope: current_scope,
        }
    }

    pub fn is_none(&self) -> bool {
        self.scope.is_none()
    }
}

#[derive(Clone)]
pub struct Scope {
    declarables: Vec<Box<dyn Declarable>>,
    parent_scope: Option<Rc<Scope>>,
    specialization_requester: Option<SpecializationRequester>
}

impl Scope {
    pub fn new(parent_scope: Option<Rc<Scope>>, specialization_requester: Option<SpecializationRequester>) -> Self {
        Scope {
            declarables: Vec::new(),
            parent_scope,
            specialization_requester
        }
    }
    pub fn lookup(&self, name: &QualifiedName) -> Vec<&dyn Declarable> {
        if !name.namespace_qualification.is_empty() && name.namespace_qualification[0].is_empty() {
            if let Some(default_namespace) = GlobalContext::get_default_namespace() {
                return default_namespace.lookup(&name.drop_first_namespace_qualification());
            } else {
                return Vec::new();
            }
        }

        let mut result: Vec<&dyn Declarable> = if let Some(parent) = &self.parent_scope {
            parent.lookup(name)
        } else {
            Vec::new()
        };

        for declarable in self.lookup_shallow(name) {
            result.push(declarable);
        }
        result
    }

    fn lookup_shallow(&self, name: &QualifiedName) -> Vec<&dyn Declarable> {
        self.declarables
            .iter()
            .filter(|d| d.readable_name() == name.name) //TODO: Add namespace checks later.
            .map(|d| d.as_ref() as &dyn Declarable)
            .collect()
    }

    pub fn parent_scope(&self) -> &Option<Rc<Scope>> {
        &self.parent_scope
    }

    pub fn get_specialization_requester(&self) -> &Option<SpecializationRequester> {
        &self.specialization_requester
    }
}

#[derive(Clone)]
pub struct TypeConstraint {
    upper_bound: Option<Rc<Type>>,
}

impl TypeConstraint {
    pub fn subtype_constraint(upper_bound: Rc<Type>) -> Self {
        TypeConstraint {
            upper_bound: Some(upper_bound),
        }
    }

    pub fn unconstrained() -> Self {
        TypeConstraint { upper_bound: None }
    }

    pub fn is_violated(&self, type_: &Type) -> Option<String> {
        if let Some(upper_bound) = &self.upper_bound {
            if !type_.is_subtype_of(upper_bound) {
                if type_.is_top_type() {
                    return TopType::cast(type).map(|top_type| top_type.reason().clone());
                } else {
                    return Some(format!(
                        "expected {} to be a subtype of {}",
                        type_, **upper_bound
                    ));
                }
            }
        }
        None
    }
}

fn find_constraint_violation(
    types: &[Rc<Type>],
    constraints: &[TypeConstraint],
) -> Option<String> {
    assert_eq!(constraints.len(), types.len());
    for i in 0..types.len() {
        if let Some(violation) = constraints[i].is_violated(&types[i]) {
            return Some(format!("Could not instantiate generic, {}.", violation));
        }
    }
    None
}

pub fn compute_constraints(
    scope: &Rc<Scope>,
    parameters: &[GenericParameter],
) -> Vec<TypeConstraint> {
    // CurrentScope::Scope scope_scope(scope); // Assuming CurrentScope is handled elsewhere.
    let mut result = Vec::new();
    for parameter in parameters {
        if let Some(constraint) = &parameter.constraint {
            // CurrentSourcePosition::Scope position_activator(Position()); // Assuming CurrentSourcePosition is handled elsewhere.
            result.push(TypeConstraint::subtype_constraint(
                TypeVisitor::compute_type(constraint),
            ));
        } else {
            result.push(TypeConstraint::unconstrained());
        }
    }
    result
}

pub struct TypeArgumentInference {
    result: Vec<Rc<Type>>,
    failed: bool,
    failure_reason: Option<String>,
}

impl TypeArgumentInference {
    pub fn new(
        generic_parameters: &[GenericParameter],
        explicit_specialization_types: &TypeVector,
        parameters: &[TypeExpression],
        arguments: &[Option<Rc<Type>>>
    ) -> Self {
        TypeArgumentInference {
            result: vec![],
            failed: false,
            failure_reason: None,
        }
    }

    pub fn has_failed(&self) -> bool {
        self.failed
    }

    pub fn get_result(&self) -> &[Rc<Type>] {
        &self.result
    }

    pub fn fail(mut self, reason: String) -> Self {
        self.failed = true;
        self.failure_reason = Some(reason);
        self
    }
}

#[derive(Clone)]
pub struct Namespace {
    name: String,
    declarables: Vec<Box<dyn Declarable>>,
    parent_scope: Option<Rc<Scope>>
}

impl Namespace {
    pub fn new(name: String, parent_scope: Option<Rc<Scope>>) -> Self {
        Namespace {
            name,
            declarables: Vec::new(),
            parent_scope
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_default_namespace(&self) -> bool {
        if let Some(default_namespace) = GlobalContext::get_default_namespace() {
            self as *const _ == default_namespace as *const _
        } else {
            false
        }
    }

    pub fn is_test_namespace(&self) -> bool {
        self.name() == kTestNamespaceName
    }

    pub fn lookup(&self, name: &QualifiedName) -> Vec<&dyn Declarable> {
        self.declarables
            .iter()
            .filter(|d| d.readable_name() == name.name) //TODO: Add namespace checks later.
            .map(|d| d.as_ref() as &dyn Declarable)
            .collect()
    }
}

pub struct TypeAlias {
    name: String,
    delayed_: Rc<DelayedType>,
    type_: Option<Rc<Type>>,
    parent_scope: Option<Rc<Scope>>,
    being_resolved_: bool
}

impl TypeAlias {
    pub fn new(name: String, delayed_: Rc<DelayedType>, parent_scope: Option<Rc<Scope>>) -> Self {
        TypeAlias {
            name,
            delayed_,
            type_: None,
            parent_scope,
            being_resolved_: false
        }
    }
    pub fn resolve(&mut self) -> Rc<Type> {
        if self.type_.is_none() {
            //Assuming CurrentScope and CurrentSourcePosition are handled globally
            //CurrentScope::Scope scope_activator(self.parent_scope.as_ref().map(|s| s.deref()));
            //CurrentSourcePosition::Scope position_activator(self.Position());

            if self.being_resolved_ {
                //TODO:Implement ReportError, requires global access
                //ReportError(format!("Cannot create type {} due to circular dependencies.", self.delayed_.decl.name.value));
                println!("Cannot create type {} due to circular dependencies.", self.delayed_.decl.name.value);
            }
            self.being_resolved_ = true;
            let decl = self.delayed_.decl.clone();
            self.type_ = Some(TypeVisitor::compute_type(&decl));
        }
        self.type_.clone().unwrap()
    }
}

// Dummy implementations for types that are not fully converted
pub struct TopType {}
impl TopType {
    pub fn cast(_type: &Type) -> Option<&Self> {
        None
    }

    pub fn reason(&self) -> &String {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct TypeVector {
    pub types: Vec<Rc<Type>>,
}

impl Display for TypeVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, type_) in self.types.iter().enumerate() {
            write!(f, "{}", type_)?;
            if i < self.types.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

// Dummy implementations
impl Type {
    pub fn is_subtype_of(&self, _other: &Type) -> bool {
        false
    }
    pub fn is_top_type(&self) -> bool {
        false
    }
}

pub struct DelayedType {
    decl: Rc<TypeDeclaration>
}

impl DelayedType {
    pub fn new(decl: Rc<TypeDeclaration>) -> Self {
        DelayedType {
            decl
        }
    }
}

pub struct GenericParameter {
    pub name: Rc<Identifier>,
    pub constraint: Option<Rc<TypeExpression>>
}

fn print_comma_separated_list<T, F>(f: &mut Formatter<'_>, list: &[T], printer: F) -> fmt::Result
    where
        F: Fn(&T) -> String,
{
    for (i, item) in list.iter().enumerate() {
        write!(f, "{}", printer(item))?;
        if i < list.len() - 1 {
            write!(f, ", ")?;
        }
    }
    Ok(())
}

const kTestNamespaceName: &str = "test";
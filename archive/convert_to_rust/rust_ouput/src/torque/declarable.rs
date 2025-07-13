// Converted from V8 C++ source files:
// Header: declarable.h
// Implementation: declarable.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod declarable {
    use crate::torque::ast::{
        Error, Expression, GenericParameter, GenericParameters, Identifier,
        SourcePosition, Statement, TypeExpression,
    };
    use crate::torque::type_visitor::TypeVisitor;
    use crate::torque::types::{AggregateType, NameVector, Signature, Type, TypeVector, TopType};
    use crate::torque::utils::PrintCommaSeparatedList;
    use std::cell::RefCell;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::mem;
    use std::optional::Option;
    use std::rc::Rc;
    thread_local! {
        pub static CURRENT_SCOPE: RefCell<Option<*mut Scope>> = RefCell::new(None);
        pub static CURRENT_SOURCE_POSITION: RefCell<SourcePosition> = RefCell::new(SourcePosition::Invalid());
    }
    #[macro_export]
    macro_rules! DECLARE_CONTEXTUAL_VARIABLE {
        ($name:ident, $type:ty) => {
            pub mod $name {
                use std::cell::RefCell;
                thread_local! {
                    static VALUE: RefCell<Option<$type>> = RefCell::new(None);
                }
                pub fn Get() -> $type {
                    VALUE.with(|f| *f.borrow().as_ref().unwrap())
                }
                pub fn Set(value: $type) {
                    VALUE.with(|f| {
                        *f.borrow_mut() = Some(value);
                    });
                }
                pub fn Clear() {
                    VALUE.with(|f| {
                        f.borrow_mut().take();
                    });
                }
                pub struct Scope {
                    old_value: Option<$type>,
                }
                impl Scope {
                    pub fn new(value: $type) -> Self {
                        let old_value = VALUE.with(|f| f.borrow_mut().take());
                        Set(value);
                        Scope { old_value }
                    }
                }
                impl Drop for Scope {
                    fn drop(&mut self) {
                        VALUE.with(|f| {
                            *f.borrow_mut() = self.old_value.take();
                        });
                    }
                }
            }
        };
    }
    DECLARE_CONTEXTUAL_VARIABLE!(CurrentScope, *mut Scope);
    DECLARE_CONTEXTUAL_VARIABLE!(CurrentSourcePosition, SourcePosition);
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct QualifiedName {
        pub namespace_qualification: Vec<String>,
        pub name: String,
    }
    impl QualifiedName {
        pub fn new(namespace_qualification: Vec<String>, name: String) -> Self {
            QualifiedName {
                namespace_qualification,
                name,
            }
        }
        pub fn simple(name: String) -> Self {
            QualifiedName {
                namespace_qualification: Vec::new(),
                name,
            }
        }
        pub fn parse(qualified_name: String) -> Self {
            let mut qualifications: Vec<String> = Vec::new();
            let mut current_name = qualified_name;
            while let Some(index) = current_name.find("::") {
                qualifications.push(current_name[..index].to_string());
                current_name = current_name[index + 2..].to_string();
            }
            QualifiedName {
                namespace_qualification: qualifications,
                name: current_name,
            }
        }
        pub fn has_namespace_qualification(&self) -> bool {
            !self.namespace_qualification.is_empty()
        }
        pub fn drop_first_namespace_qualification(&self) -> Self {
            if self.namespace_qualification.is_empty() {
                return self.clone();
            }
            QualifiedName {
                namespace_qualification: self.namespace_qualification[1..].to_vec(),
                name: self.name.clone(),
            }
        }
    }
    impl fmt::Display for QualifiedName {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for qualifier in &self.namespace_qualification {
                write!(f, "{}::", qualifier)?;
            }
            write!(f, "{}", self.name)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct VisitResult {
        pub type_: *const Type,
        pub value: String,
    }
    impl VisitResult {
        pub fn new(type_: *const Type, value: String) -> Self {
            VisitResult { type_, value }
        }
    }
    #[derive(Debug, PartialEq)]
    pub enum DeclarableKind {
        Namespace,
        TorqueMacro,
        ExternMacro,
        Method,
        Builtin,
        RuntimeFunction,
        Intrinsic,
        GenericCallable,
        GenericType,
        TypeAlias,
        ExternConstant,
        NamespaceConstant,
    }
    pub trait DeclarableTrait {
        fn kind(&self) -> DeclarableKind;
        fn is_namespace(&self) -> bool {
            self.kind() == DeclarableKind::Namespace
        }
        fn is_macro(&self) -> bool {
            self.is_torque_macro() || self.is_extern_macro()
        }
        fn is_torque_macro(&self) -> bool {
            self.kind() == DeclarableKind::TorqueMacro || self.is_method()
        }
        fn is_method(&self) -> bool {
            self.kind() == DeclarableKind::Method
        }
        fn is_extern_macro(&self) -> bool {
            self.kind() == DeclarableKind::ExternMacro
        }
        fn is_intrinsic(&self) -> bool {
            self.kind() == DeclarableKind::Intrinsic
        }
        fn is_builtin(&self) -> bool {
            self.kind() == DeclarableKind::Builtin
        }
        fn is_runtime_function(&self) -> bool {
            self.kind() == DeclarableKind::RuntimeFunction
        }
        fn is_generic_callable(&self) -> bool {
            self.kind() == DeclarableKind::GenericCallable
        }
        fn is_generic_type(&self) -> bool {
            self.kind() == DeclarableKind::GenericType
        }
        fn is_type_alias(&self) -> bool {
            self.kind() == DeclarableKind::TypeAlias
        }
        fn is_extern_constant(&self) -> bool {
            self.kind() == DeclarableKind::ExternConstant
        }
        fn is_namespace_constant(&self) -> bool {
            self.kind() == DeclarableKind::NamespaceConstant
        }
        fn is_value(&self) -> bool {
            self.is_extern_constant() || self.is_namespace_constant()
        }
        fn is_scope(&self) -> bool {
            self.is_namespace() || self.is_callable()
        }
        fn is_callable(&self) -> bool {
            self.is_macro()
                || self.is_builtin()
                || self.is_runtime_function()
                || self.is_intrinsic()
                || self.is_method()
        }
        fn type_name(&self) -> &'static str {
            "<<unknown>>"
        }
        fn parent_scope(&self) -> Option<&Scope>;
        fn position(&self) -> SourcePosition;
        fn set_position(&mut self, position: SourcePosition);
        fn identifier_position(&self) -> SourcePosition;
        fn set_identifier_position(&mut self, position: SourcePosition);
        fn is_user_defined(&self) -> bool;
        fn set_is_user_defined(&mut self, is_user_defined: bool);
    }
    pub struct Declarable {
        kind_: DeclarableKind,
        parent_scope_: Option<Box<Scope>>,
        position_: SourcePosition,
        identifier_position_: SourcePosition,
        is_user_defined_: bool,
    }
    impl Declarable {
        pub fn new(kind: DeclarableKind) -> Self {
            let parent_scope_ = CURRENT_SCOPE.with(|s| {
                s.borrow()
                    .map(|ptr| unsafe { Box::new((*ptr).clone()) })
            });
            let position_ = CURRENT_SOURCE_POSITION.with(|pos| *pos.borrow());
            Declarable {
                kind_: kind,
                parent_scope_: parent_scope_,
                position_: position_,
                identifier_position_: SourcePosition::Invalid(),
                is_user_defined_: true,
            }
        }
    }
    impl DeclarableTrait for Declarable {
        fn kind(&self) -> DeclarableKind {
            self.kind_.clone()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.parent_scope_.as_ref().map(|s| &**s)
        }
        fn position(&self) -> SourcePosition {
            self.position_
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.position_ = position;
        }
        fn identifier_position(&self) -> SourcePosition {
            if self.identifier_position_.source.is_valid() {
                self.identifier_position_
            } else {
                self.position_
            }
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.identifier_position_ = position;
        }
        fn is_user_defined(&self) -> bool {
            self.is_user_defined_
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.is_user_defined_ = is_user_defined;
        }
    }
    #[macro_export]
    macro_rules! DECLARE_DECLARABLE_BOILERPLATE {
        ($x:ident, $y:ident) => {
            impl $x {
                pub fn cast(declarable: &Declarable) -> &Self {
                    assert!(declarable.is_$x());
                    unsafe { std::mem::transmute(declarable) }
                }
                pub fn dynamic_cast(declarable: &Declarable) -> Option<&Self> {
                    if !declarable.is_$x() {
                        return None;
                    }
                    Some(unsafe { std::mem::transmute(declarable) })
                }
            }
            impl DeclarableTrait for $x {
                fn type_name(&self) -> &'static str {
                    stringify!($y)
                }
                fn kind(&self) -> DeclarableKind {
                    Self::KIND
                }
                fn parent_scope(&self) -> Option<&Scope> {
                    self.declarable.parent_scope()
                }
                fn position(&self) -> SourcePosition {
                    self.declarable.position()
                }
                fn set_position(&mut self, position: SourcePosition) {
                    self.declarable.set_position(position);
                }
                fn identifier_position(&self) -> SourcePosition {
                    self.declarable.identifier_position()
                }
                fn set_identifier_position(&mut self, position: SourcePosition) {
                    self.declarable.set_identifier_position(position);
                }
                fn is_user_defined(&self) -> bool {
                    self.declarable.is_user_defined()
                }
                fn set_is_user_defined(&mut self, is_user_defined: bool) {
                    self.declarable.set_is_user_defined(is_user_defined);
                }
            }
        };
    }
    #[derive(Debug, Clone)]
    pub struct SpecializationRequester {
        pub position: SourcePosition,
        pub scope: Option<Box<Scope>>,
        pub name: String,
    }
    impl SpecializationRequester {
        pub fn none() -> Self {
            SpecializationRequester {
                position: SourcePosition::Invalid(),
                scope: None,
                name: String::new(),
            }
        }
        pub fn is_none(&self) -> bool {
            self.position == SourcePosition::Invalid() && self.scope.is_none() && self.name.is_empty()
        }
        pub fn new(position: SourcePosition, scope: Option<&Scope>, name: String) -> Self {
            let mut s = scope;
            while let Some(inner_scope) = s {
                if !inner_scope.requester_.is_none() {
                    break;
                }
                s = inner_scope.declarable.parent_scope().as_ref();
            }
            SpecializationRequester {
                position,
                scope: s.map(|s| Box::new(s.clone())),
                name,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Scope {
        declarable: Declarable,
        declarations_: HashMap<String, Vec<Box<Declarable>>>,
        requester_: SpecializationRequester,
    }
    impl Scope {
        pub const KIND: DeclarableKind = DeclarableKind::Namespace;
        pub fn new(kind: DeclarableKind) -> Self {
            Scope {
                declarable: Declarable::new(kind),
                declarations_: HashMap::new(),
                requester_: SpecializationRequester::none(),
            }
        }
        pub fn lookup_shallow(&self, name: &QualifiedName) -> Vec<&Declarable> {
            if name.has_namespace_qualification() {
                if let Some(declarables) = self.declarations_.get(&name.namespace_qualification[0]) {
                    let mut child: Option<&Scope> = None;
                    for declarable in declarables {
                        if let Some(scope) = Scope::dynamic_cast(&*declarable.declarable) {
                            if child.is_some() {
                                //ReportError("ambiguous reference to scope ", name.namespace_qualification.front());
                                println!("ambiguous reference to scope {}", name.namespace_qualification[0]);
                            }
                            child = Some(scope);
                        }
                    }
                    if let Some(child) = child {
                        return child.lookup_shallow(&name.drop_first_namespace_qualification());
                    }
                }
                Vec::new()
            } else {
                self.declarations_
                    .get(&name.name)
                    .map(|vec| vec.iter().map(|d| &*d.declarable).collect())
                    .unwrap_or_default()
            }
        }
        pub fn add_declarable(&mut self, name: String, declarable: Declarable) {
            self.declarations_
                .entry(name)
                .or_insert_with(Vec::new)
                .push(Box::new(Scope {
                    declarable,
                    declarations_: HashMap::new(),
                    requester_: SpecializationRequester::none(),
                }));
        }
        pub fn get_specialization_requester(&self) -> &SpecializationRequester {
            &self.requester_
        }
        pub fn set_specialization_requester(&mut self, requester: SpecializationRequester) {
            self.requester_ = requester;
        }
        pub fn lookup(&self, name: &QualifiedName) -> Vec<&Declarable> {
            if !name.namespace_qualification.is_empty() && name.namespace_qualification[0].is_empty() {
                if let Some(default_namespace) = get_default_namespace() {
                    return default_namespace.lookup(&name.drop_first_namespace_qualification());
                }
            }
            let mut result = Vec::new();
            if let Some(parent_scope) = self.declarable.parent_scope() {
                result.extend(parent_scope.lookup(name));
            }
            result.extend(self.lookup_shallow(name));
            result
        }
    }
    impl DeclarableTrait for Scope {
        fn kind(&self) -> DeclarableKind {
            self.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "Scope"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(Scope, scope);
    #[derive(Debug, Clone, PartialEq)]
    pub struct Namespace {
        scope: Scope,
        name_: String,
    }
    impl Namespace {
        pub const KIND: DeclarableKind = DeclarableKind::Namespace;
        pub fn new(name: String) -> Self {
            Namespace {
                scope: Scope::new(DeclarableKind::Namespace),
                name_: name,
            }
        }
        pub fn name(&self) -> &String {
            &self.name_
        }
        pub fn is_default_namespace(&self) -> bool {
            if let Some(default_namespace) = get_default_namespace() {
                self == default_namespace
            } else {
                false
            }
        }
        pub fn is_test_namespace(&self) -> bool {
            self.name() == kTestNamespaceName
        }
    }
    impl DeclarableTrait for Namespace {
        fn kind(&self) -> DeclarableKind {
            self.scope.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.scope.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.scope.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.scope.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.scope.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.scope.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.scope.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.scope.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "Namespace"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(Namespace, namespace);
    thread_local! {
        static DEFAULT_NAMESPACE: RefCell<Option<Namespace>> = RefCell::new(None);
    }
    fn get_default_namespace() -> Option<&'static Namespace> {
        DEFAULT_NAMESPACE.with(|dn| unsafe { dn.borrow().as_ref() })
    }
    pub fn current_namespace() -> Option<&'static Namespace> {
        CURRENT_SCOPE.with(|cs| {
            let mut scope = cs.borrow();
            let mut current_scope: Option<&Scope> = scope.map(|s| unsafe { &*s });
            while let Some(s) = current_scope {
                if let Some(n) = Namespace::dynamic_cast(s.declarable()) {
                    return Some(unsafe { mem::transmute(n) });
                }
                current_scope = s.declarable().parent_scope();
            }
            None
        })
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Value {
        declarable: Declarable,
        type_: *const Type,
        name_: *const Identifier,
        value_: Option<VisitResult>,
    }
    impl Value {
        pub const KIND: DeclarableKind = DeclarableKind::ExternConstant;
        pub fn new(kind: DeclarableKind, type_: *const Type, name: *const Identifier) -> Self {
            Value {
                declarable: Declarable::new(kind),
                type_: type_,
                name_: name,
                value_: None,
            }
        }
        pub fn name(&self) -> *const Identifier {
            self.name_
        }
        pub fn is_const(&self) -> bool {
            true
        }
        pub fn value(&self) -> &Option<VisitResult> {
            &self.value_
        }
        pub fn type_(&self) -> *const Type {
            self.type_
        }
        pub fn set_value(&mut self, value: VisitResult) {
            assert!(self.value_.is_none());
            self.value_ = Some(value);
        }
    }
    impl DeclarableTrait for Value {
        fn kind(&self) -> DeclarableKind {
            self.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "Value"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(Value, value);
    #[derive(Debug, Clone, PartialEq)]
    pub struct NamespaceConstant {
        value: Value,
        external_name_: String,
        body_: *mut Expression,
    }
    impl NamespaceConstant {
        pub const KIND: DeclarableKind = DeclarableKind::NamespaceConstant;
        pub fn new(
            constant_name: *mut Identifier,
            external_name: String,
            type_: *const Type,
            body: *mut Expression,
        ) -> Self {
            NamespaceConstant {
                value: Value::new(DeclarableKind::NamespaceConstant, type_, constant_name),
                external_name_: external_name,
                body_: body,
            }
        }
        pub fn external_name(&self) -> &String {
            &self.external_name_
        }
        pub fn body(&self) -> *mut Expression {
            self.body_
        }
    }
    impl DeclarableTrait for NamespaceConstant {
        fn kind(&self) -> DeclarableKind {
            self.value.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.value.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.value.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.value.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.value.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.value.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.value.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.value.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "NamespaceConstant"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(NamespaceConstant, constant);
    #[derive(Debug, Clone, PartialEq)]
    pub struct ExternConstant {
        value: Value,
        value_str: String,
    }
    impl ExternConstant {
        pub const KIND: DeclarableKind = DeclarableKind::ExternConstant;
        pub fn new(name: *mut Identifier, type_: *const Type, value: String) -> Self {
            let mut extern_constant = ExternConstant {
                value: Value::new(DeclarableKind::ExternConstant, type_, name),
                value_str: value.clone(),
            };
            extern_constant.value.set_value(VisitResult {
                type_: type_,
                value: value,
            });
            extern_constant
        }
    }
    impl DeclarableTrait for ExternConstant {
        fn kind(&self) -> DeclarableKind {
            self.value.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.value.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.value.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.value.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.value.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.value.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.value.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.value.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "ExternConstant"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(ExternConstant, constant);
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum OutputType {
        CSA,
        CC,
        CCDebug,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Callable {
        scope: Scope,
        external_name_: String,
        readable_name_: String,
        signature_: Signature,
        returns_: usize,
        body_: Option<*mut Statement>,
    }
    impl Callable {
        pub const KIND: DeclarableKind = DeclarableKind::Builtin;
        pub fn new(
            kind: DeclarableKind,
            external_name: String,
            readable_name: String,
            signature: Signature,
            body: Option<*mut Statement>,
        ) -> Self {
            Callable {
                scope: Scope::new(kind),
                external_name_: external_name,
                readable_name_: readable_name,
                signature_: signature,
                returns_: 0,
                body_: body,
            }
        }
        pub fn external_name(&self) -> &String {
            &self.external_name_
        }
        pub fn readable_name(&self) -> &String {
            &self.readable_name_
        }
        pub fn signature(&self) -> &Signature {
            &self.signature_
        }
        pub fn is_transitioning(&self) -> bool {
            self.signature().transitioning
        }
        pub fn parameter_names(&self) -> &NameVector {
            &self.signature_.parameter_names
        }
        pub fn has_return_value(&self) -> bool {
            !self.signature_.return_type.is_void_or_never()
        }
        pub fn increment_returns(&mut self) {
            self.returns_ += 1;
        }
        pub fn has_returns(&self) -> bool {
            self.returns_ > 0
        }
        pub fn body(&self) -> &Option<*mut Statement> {
            &self.body_
        }
        pub fn is_external(&self) -> bool {
            self.body_.is_none()
        }
        pub fn should_be_inlined(&self, output_type: &OutputType) -> bool {
            output_type == &OutputType::CC && !self.signature().labels.is_empty()
        }
        pub fn should_generate_external_code(&self, output_type: &OutputType) -> bool {
            !self.should_be_inlined(output_type)
        }
        pub fn prefix_name_for_cc_output(name: &String) -> String {
            "TqRuntime".to_string() + name
        }
        pub fn prefix_name_for_cc_debug_output(name: &String) -> String {
            "TqDebug".to_string() + name
        }
        pub fn cc_name(&self) -> String {
            Callable::prefix_name_for_cc_output(self.external_name())
        }
        pub fn cc_debug_name(&self) -> String {
            Callable::prefix_name_for_cc_debug_output(self.external_name())
        }
    }
    impl DeclarableTrait for Callable {
        fn kind(&self) -> DeclarableKind {
            self.scope.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.scope.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.scope.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.scope.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.scope.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.scope.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.scope.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.scope.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "Callable"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(Callable, callable);
    #[derive(Debug, Clone, PartialEq)]
    pub struct Macro {
        callable: Callable,
        used_: bool,
    }
    impl Macro {
        pub const KIND: DeclarableKind = DeclarableKind::TorqueMacro;
        pub fn new(
            kind: DeclarableKind,
            external_name: String,
            readable_name: String,
            signature: Signature,
            body: Option<*mut Statement>,
        ) -> Self {
            if signature.parameter_types.var_args {
                //ReportError("Varargs are not supported for macros.");
                println!("Varargs are not supported for macros.");
            }
            Macro {
                callable: Callable::new(kind, external_name, readable_name, signature, body),
                used_: false,
            }
        }
        pub fn should_be_inlined(&self, output_type: &OutputType) -> bool {
            for label in &self.callable.signature().labels {
                for type_ in &label.types {
                    if type_.struct_supertype() {
                        return true;
                    }
                }
            }
            if !self.callable.readable_name().is_empty() && self.callable.readable_name().as_bytes()[0] == b'%' {
                return true;
            }
            self.callable.should_be_inlined(output_type)
        }
        pub fn set_used(&mut self) {
            self.used_ = true;
        }
        pub fn is_used(&self) -> bool {
            self.used_
        }
    }
    impl DeclarableTrait for Macro {
        fn kind(&self) -> DeclarableKind {
            self.callable.scope.declarable.kind()
        }
        fn parent_scope(&self) -> Option<&Scope> {
            self.callable.scope.declarable.parent_scope()
        }
        fn position(&self) -> SourcePosition {
            self.callable.scope.declarable.position()
        }
        fn set_position(&mut self, position: SourcePosition) {
            self.callable.scope.declarable.set_position(position);
        }
        fn identifier_position(&self) -> SourcePosition {
            self.callable.scope.declarable.identifier_position()
        }
        fn set_identifier_position(&mut self, position: SourcePosition) {
            self.callable.scope.declarable.set_identifier_position(position);
        }
        fn is_user_defined(&self) -> bool {
            self.callable.scope.declarable.is_user_defined()
        }
        fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.callable.scope.declarable.set_is_user_defined(is_user_defined);
        }
        fn type_name(&self) -> &'static str {
            "Macro"
        }
    }
    DECLARE_DECLARABLE_BOILERPLATE!(Macro, macro);
    #[derive(Debug, Clone, PartialEq)]
    pub struct Extern

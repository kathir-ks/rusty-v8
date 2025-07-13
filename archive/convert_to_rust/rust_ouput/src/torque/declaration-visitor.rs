// Converted from V8 C++ source files:
// Header: declaration-visitor.h
// Implementation: declaration-visitor.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod declaration_visitor {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::string::String;

    use crate::base::macros::*;
    use crate::torque::ast::*;
    use crate::torque::declarable::*;
    use crate::torque::declarations::*;
    use crate::torque::global_context::*;
    use crate::torque::kythe_data::*;
    use crate::torque::server_data::*;
    use crate::torque::type_inference::*;
    use crate::torque::type_visitor::*;
    use crate::torque::types::*;
    use crate::torque::utils::*;

    thread_local! {
        static CURRENT_SOURCE_POSITION: RefCell<SourcePosition> = RefCell::new(SourcePosition { line: 0, column: 0 });
    }

    pub struct CurrentSourcePosition {}

    impl CurrentSourcePosition {
        pub fn get() -> SourcePosition {
            CURRENT_SOURCE_POSITION.with(|pos| *pos.borrow())
        }

        pub fn set(pos: SourcePosition) {
            CURRENT_SOURCE_POSITION.with(|p| *p.borrow_mut() = pos);
        }

        pub struct Scope {
            old_pos: SourcePosition,
        }

        impl Scope {
            pub fn new(new_pos: SourcePosition) -> Self {
                let old_pos = CurrentSourcePosition::get();
                CurrentSourcePosition::set(new_pos);
                Scope { old_pos }
            }
        }

        impl Drop for Scope {
            fn drop(&mut self) {
                CurrentSourcePosition::set(self.old_pos);
            }
        }
    }

    pub fn get_or_create_namespace(name: &str) -> Rc<Namespace> {
        let existing_namespaces = filter_declarables::<Namespace>(
            Declarations::try_lookup_shallow(QualifiedName(name.to_string())),
        );
        if existing_namespaces.is_empty() {
            Declarations::declare_namespace(name.to_string())
        } else {
            assert_eq!(1, existing_namespaces.len());
            existing_namespaces.first().unwrap().clone()
        }
    }

    pub struct PredeclarationVisitor {}

    impl PredeclarationVisitor {
        pub fn predeclare(ast: &Ast) {
            let default_namespace = GlobalContext::get_default_namespace();
            CurrentScope::set_scope(default_namespace.clone());
            for child in &ast.declarations {
                PredeclarationVisitor::predeclare(child);
            }
            CurrentScope::clear_scope();
        }
        pub fn resolve_predeclarations() {
            let all_declarables = GlobalContext::all_declarables();
            for declarable in &all_declarables {
                if let Some(alias) = declarable.downcast_ref::<TypeAlias>() {
                    CurrentScope::set_scope(alias.parent_scope().clone());
                    CurrentSourcePosition::set(alias.position());
                    alias.resolve();
                    CurrentScope::clear_scope();
                }
            }
        }

        fn predeclare(decl: &Rc<dyn Declaration>) {
            CURRENT_SOURCE_POSITION.with(|pos| {
                let current_pos = *pos.borrow();
                CurrentSourcePosition::set(decl.pos());
                match decl.kind() {
                    AstNodeKind::kTypeDeclaration => {
                        let decl = decl.downcast_ref::<TypeDeclaration>().unwrap();
                        PredeclarationVisitor::predeclare_type_declaration(decl);
                    }
                    AstNodeKind::kNamespaceDeclaration => {
                        let decl = decl.downcast_ref::<NamespaceDeclaration>().unwrap();
                        PredeclarationVisitor::predeclare_namespace_declaration(decl);
                    }
                    AstNodeKind::kGenericCallableDeclaration => {
                        let decl = decl.downcast_ref::<GenericCallableDeclaration>().unwrap();
                        PredeclarationVisitor::predeclare_generic_callable_declaration(decl);
                    }
                    AstNodeKind::kGenericTypeDeclaration => {
                        let decl = decl.downcast_ref::<GenericTypeDeclaration>().unwrap();
                        PredeclarationVisitor::predeclare_generic_type_declaration(decl);
                    }
                    _ => {}
                }
                CurrentSourcePosition::set(current_pos);
            });
        }

        fn predeclare_namespace_declaration(decl: &NamespaceDeclaration) {
            let namespace = get_or_create_namespace(&decl.name);
            CurrentScope::set_scope(namespace);
            for child in &decl.declarations {
                PredeclarationVisitor::predeclare(child);
            }
            CurrentScope::clear_scope();
        }

        fn predeclare_type_declaration(decl: &TypeDeclaration) {
            let alias = Declarations::predeclare_type_alias(decl.name.clone(), Rc::new(decl.clone()), false);
            alias.set_position(decl.pos);
            alias.set_identifier_position(decl.name.pos);
            if GlobalContext::collect_kythe_data() {
                KytheData::add_type_definition(&alias);
            }
        }

        fn predeclare_struct_declaration(decl: &StructDeclaration) {
            let alias = Declarations::predeclare_type_alias(decl.name.clone(), Rc::new(decl.clone()), false);
            alias.set_position(decl.pos);
            alias.set_identifier_position(decl.name.pos);
            if GlobalContext::collect_kythe_data() {
                KytheData::add_type_definition(&alias);
            }
        }

        fn predeclare_generic_type_declaration(generic_decl: &GenericTypeDeclaration) {
            Declarations::declare_generic_type(
                generic_decl.declaration.name.value.clone(),
                Rc::new(generic_decl.clone()),
            );
        }

        fn predeclare_generic_callable_declaration(generic_decl: &GenericCallableDeclaration) {
            Declarations::declare_generic_callable(
                generic_decl.declaration.name.value.clone(),
                Rc::new(generic_decl.clone()),
            );
        }
    }

    pub struct DeclarationVisitor {}

    impl DeclarationVisitor {
        pub fn visit(ast: &Ast) {
            let default_namespace = GlobalContext::get_default_namespace();
            CurrentScope::set_scope(default_namespace.clone());
            for child in &ast.declarations {
                DeclarationVisitor::visit(child);
            }
            CurrentScope::clear_scope();
        }

        fn visit(decl: &Rc<dyn Declaration>) {
            CURRENT_SOURCE_POSITION.with(|pos| {
                let current_pos = *pos.borrow();
                CurrentSourcePosition::set(decl.pos());
                match decl.kind() {
                    AstNodeKind::kNamespaceDeclaration => {
                        let decl = decl.downcast_ref::<NamespaceDeclaration>().unwrap();
                        DeclarationVisitor::visit_namespace_declaration(decl);
                    }
                    AstNodeKind::kTypeDeclaration => {
                        let decl = decl.downcast_ref::<TypeDeclaration>().unwrap();
                        DeclarationVisitor::visit_type_declaration(decl);
                    }
                    AstNodeKind::kStructDeclaration => {
                        let decl = decl.downcast_ref::<StructDeclaration>().unwrap();
                        DeclarationVisitor::visit_struct_declaration(decl);
                    }
                    AstNodeKind::kExternalBuiltinDeclaration => {
                        let decl = decl.downcast_ref::<ExternalBuiltinDeclaration>().unwrap();
                        DeclarationVisitor::visit_external_builtin_declaration(decl);
                    }
                    AstNodeKind::kExternalRuntimeDeclaration => {
                        let decl = decl.downcast_ref::<ExternalRuntimeDeclaration>().unwrap();
                        DeclarationVisitor::visit_external_runtime_declaration(decl);
                    }
                    AstNodeKind::kExternalMacroDeclaration => {
                        let decl = decl.downcast_ref::<ExternalMacroDeclaration>().unwrap();
                        DeclarationVisitor::visit_external_macro_declaration(decl);
                    }
                    AstNodeKind::kTorqueBuiltinDeclaration => {
                        let decl = decl.downcast_ref::<TorqueBuiltinDeclaration>().unwrap();
                        DeclarationVisitor::visit_torque_builtin_declaration(decl);
                    }
                    AstNodeKind::kTorqueMacroDeclaration => {
                        let decl = decl.downcast_ref::<TorqueMacroDeclaration>().unwrap();
                        DeclarationVisitor::visit_torque_macro_declaration(decl);
                    }
                    AstNodeKind::kIntrinsicDeclaration => {
                        let decl = decl.downcast_ref::<IntrinsicDeclaration>().unwrap();
                        DeclarationVisitor::visit_intrinsic_declaration(decl);
                    }
                    AstNodeKind::kConstDeclaration => {
                        let decl = decl.downcast_ref::<ConstDeclaration>().unwrap();
                        DeclarationVisitor::visit_const_declaration(decl);
                    }
                    AstNodeKind::kGenericCallableDeclaration => {
                        let decl = decl.downcast_ref::<GenericCallableDeclaration>().unwrap();
                        DeclarationVisitor::visit_generic_callable_declaration(decl);
                    }
                    AstNodeKind::kGenericTypeDeclaration => {
                        let decl = decl.downcast_ref::<GenericTypeDeclaration>().unwrap();
                        DeclarationVisitor::visit_generic_type_declaration(decl);
                    }
                    AstNodeKind::kSpecializationDeclaration => {
                        let decl = decl.downcast_ref::<SpecializationDeclaration>().unwrap();
                        DeclarationVisitor::visit_specialization_declaration(decl);
                    }
                    AstNodeKind::kExternConstDeclaration => {
                        let decl = decl.downcast_ref::<ExternConstDeclaration>().unwrap();
                        DeclarationVisitor::visit_extern_const_declaration(decl);
                    }
                    AstNodeKind::kCppIncludeDeclaration => {
                        let decl = decl.downcast_ref::<CppIncludeDeclaration>().unwrap();
                        DeclarationVisitor::visit_cpp_include_declaration(decl);
                    }
                    _ => panic!("UNIMPLEMENTED"),
                }
                CurrentSourcePosition::set(current_pos);
            });
        }

        fn visit_namespace_declaration(decl: &NamespaceDeclaration) {
            let namespace = get_or_create_namespace(&decl.name);
            CurrentScope::set_scope(namespace);
            for child in &decl.declarations {
                DeclarationVisitor::visit(child);
            }
            CurrentScope::clear_scope();
        }

        fn visit_type_declaration(decl: &TypeDeclaration) {
            Declarations::lookup_type(&decl.name);
        }

        fn visit_struct_declaration(decl: &StructDeclaration) {
            Declarations::lookup_type(&decl.name);
        }

        pub fn create_builtin(
            decl: &dyn BuiltinDeclarationBase,
            external_name: String,
            readable_name: String,
            signature: Signature,
            use_counter_name: Option<String>,
            body: Option<Rc<Statement>>,
        ) -> Rc<Builtin> {
            let javascript = decl.javascript_linkage();
            let varargs = decl.parameters().has_varargs;
            let kind = if !javascript {
                BuiltinKind::kStub
            } else if varargs {
                BuiltinKind::kVarArgsJavaScript
            } else {
                BuiltinKind::kFixedArgsJavaScript
            };
            let mut has_custom_interface_descriptor = false;
            if let Some(decl) = decl.as_any().downcast_ref::<TorqueBuiltinDeclaration>() {
                has_custom_interface_descriptor = decl.has_custom_interface_descriptor;
            }

            if varargs && !javascript {
                report_error(
                    "Rest parameters require ".to_string()
                        + &decl.name().value
                        + " to be a JavaScript builtin",
                );
            }

            if javascript {
                if !signature.return_type.is_subtype_of(TypeOracle::get_js_any_type()) {
                    report_error("Return type of JavaScript-linkage builtins has to be JSAny.")
                        .position(signature.return_type.pos());
                }

                if !decl.as_any().is::<ExternalBuiltinDeclaration>() {
                    for i in signature.implicit_count..signature.parameter_types.types.len() {
                        let parameter_type = &signature.parameter_types.types[i];
                        if !TypeOracle::get_js_any_type().is_subtype_of(parameter_type) {
                            report_error(
                                "Parameters of JavaScript-linkage builtins have to be a supertype of JSAny."
                                    .to_string(),
                            )
                            .position(decl.parameters().types[i].pos());
                        }
                    }
                }
            }

            for i in 0..signature.types().len() {
                let parameter_type = &signature.types()[i];
                if parameter_type.struct_supertype() {
                    report_error(
                        "Builtin do not support structs as arguments, but argument ".to_string()
                            + &signature.parameter_names[i]
                            + " has type "
                            + &signature.types()[i].to_string()
                            + ".",
                    );
                }
                if parameter_type.is_float32() || parameter_type.is_float64() {
                    if !has_custom_interface_descriptor {
                        report_error(
                            "Builtin ".to_string()
                                + &external_name
                                + " needs a custom interface descriptor, because it uses type "
                                + &parameter_type.to_string()
                                + " for argument "
                                + &signature.parameter_names[i]
                                + ". One reason being that the default descriptor defines xmm0 to be the first floating point argument register, which is current used as scratch on ia32 and cannot be allocated.",
                        );
                    }
                }
            }

            if signature.return_type.struct_supertype() && javascript {
                report_error(
                    "Builtins with JS linkage cannot return structs, but the return type is "
                        .to_string()
                        + &signature.return_type.to_string()
                        + ".",
                );
            }

            if signature.return_type == TypeOracle::get_void_type() {
                report_error("Builtins cannot have return type void.".to_string());
            }

            let mut flags = BuiltinFlag::kNone;
            if has_custom_interface_descriptor {
                flags |= BuiltinFlag::kCustomInterfaceDescriptor;
            }
            let builtin = Declarations::create_builtin(
                external_name.clone(),
                readable_name.clone(),
                kind,
                flags,
                signature,
                use_counter_name,
                body,
            );
            builtin.set_identifier_position(decl.name().pos);
            builtin
        }

        fn visit_external_builtin_declaration(decl: &ExternalBuiltinDeclaration) {
            let signature = TypeVisitor::make_signature(decl);
            let builtin = DeclarationVisitor::create_builtin(
                decl,
                decl.name.value.clone(),
                decl.name.value.clone(),
                signature,
                None,
                None,
            );
            builtin.set_identifier_position(decl.name.pos);
            Declarations::declare(decl.name.value.clone(), builtin);
        }

        fn visit_external_runtime_declaration(decl: &ExternalRuntimeDeclaration) {
            let signature = TypeVisitor::make_signature(decl);
            if signature.parameter_types.types.is_empty() {
                report_error(
                    "Missing parameters for runtime function, at least the context parameter is required."
                        .to_string(),
                );
            }
            if !(signature.parameter_types.types[0] == TypeOracle::get_context_type()
                || signature.parameter_types.types[0] == TypeOracle::get_no_context_type())
            {
                report_error(
                    "first parameter to runtime functions has to be the context and have type Context or NoContext, but found type ".to_string() + &signature.parameter_types.types[0].to_string(),
                );
            }
            if !signature
                .return_type
                .is_subtype_of(TypeOracle::get_strong_tagged_type())
                && signature.return_type != TypeOracle::get_void_type()
                && signature.return_type != TypeOracle::get_never_type()
            {
                report_error(
                    "runtime functions can only return strong tagged values, but found type "
                        .to_string()
                        + &signature.return_type.to_string(),
                );
            }
            for parameter_type in &signature.parameter_types.types {
                if !parameter_type.is_subtype_of(TypeOracle::get_strong_tagged_type()) {
                    report_error(
                        "runtime functions can only take strong tagged parameters, but found type "
                            .to_string()
                            + &parameter_type.to_string(),
                    );
                }
            }

            let function = Declarations::declare_runtime_function(decl.name.value.clone(), signature);
            function.set_identifier_position(decl.name.pos);
            function.set_position(decl.pos);
            if GlobalContext::collect_kythe_data() {
                KytheData::add_function_definition(&function);
            }
        }

        fn visit_external_macro_declaration(decl: &ExternalMacroDeclaration) {
            let macro_ = Declarations::declare_macro(
                decl.name.value.clone(),
                true,
                decl.external_assembler_name.clone(),
                TypeVisitor::make_signature(decl),
                None,
                decl.op.clone(),
            );
            macro_.set_identifier_position(decl.name.pos);
            macro_.set_position(decl.pos);
            if GlobalContext::collect_kythe_data() {
                KytheData::add_function_definition(&macro_);
            }
        }

        fn visit_torque_builtin_declaration(decl: &TorqueBuiltinDeclaration) {
            let signature = TypeVisitor::make_signature(decl);
            if let Some(use_counter_name) = &decl.use_counter_name {
                if signature.types().is_empty()
                    || (signature.types()[0] != TypeOracle::get_native_context_type()
                        && signature.types()[0] != TypeOracle::get_context_type())
                {
                    report_error(
                        "@incrementUseCounter requires the builtin's first parameter to be of type Context or NativeContext, but found type "
                            .to_string()
                            + &signature.types()[0].to_string(),
                    );
                }
            }
            let builtin = DeclarationVisitor::create_builtin(
                decl,
                decl.name.value.clone(),
                decl.name.value.clone(),
                signature,
                decl.use_counter_name.clone(),
                decl.body.clone(),
            );
            builtin.set_identifier_position(decl.name.pos);
            builtin.set_position(decl.pos);
            Declarations::declare(decl.name.value.clone(), builtin);
        }

        fn visit_torque_macro_declaration(decl: &TorqueMacroDeclaration) {
            let macro_ = Declarations::declare_macro(
                decl.name.value.clone(),
                decl.export_to_csa,
                None,
                TypeVisitor::make_signature(decl),
                decl.body.clone(),
                decl.op.clone(),
            );
            macro_.set_identifier_position(decl.name.pos);
            macro_.set_position(decl.pos);
            if GlobalContext::collect_kythe_data() {
                KytheData::add_function_definition(&macro_);
            }
        }

        fn visit_intrinsic_declaration(decl: &IntrinsicDeclaration) {
            Declarations::declare_intrinsic(decl.name.value.clone(), TypeVisitor::make_signature(decl));
        }

        fn visit_const_declaration(decl: &ConstDeclaration) {
            let constant = Declarations::declare_namespace_constant(
                decl.name.clone(),
                TypeVisitor::compute_type(&decl.type_),
                decl.expression.clone(),
            );
            if GlobalContext::collect_kythe_data() {
                KytheData::add_constant_definition(&constant);
            }
        }

        fn visit_generic_callable_declaration(decl: &GenericCallableDeclaration) {}

        fn visit_generic_type_declaration(decl: &GenericTypeDeclaration) {}

        fn visit_specialization_declaration(decl: &SpecializationDeclaration) {
            let generic_list = Declarations::lookup_generic(&decl.name.value);
            let mut matching_generic: Option<Rc<GenericCallable>> = None;
            let signature_with_types = TypeVisitor::make_signature(decl);
            for generic in &generic_list {
                let inference = generic.infer_specialization_types(
                    TypeVisitor::compute_type_vector(&decl.generic_parameters),
                    vec![],
                );
                if inference.has_failed() {
                    continue;
                }
                let generic_signature_with_types =
                    DeclarationVisitor::make_specialized_signature(SpecializationKey {
                        generic: generic.clone(),
                        specialized_types: TypeVisitor::compute_type_vector(&decl.generic_parameters),
                    });
                if signature_with_types.has_same_types_as(&generic_signature_with_types, ParameterMode::kIgnoreImplicit) {
                    if matching_generic.is_some() {
                        let mut stream = std::string::String::new();
                        stream.push_str(&("specialization of ".to_string() + &decl.name.to_string()));
                        stream.push_str(" is ambigous, it matches more than one generic declaration (");
                        stream.push_str(&matching_generic.as_ref().unwrap().to_string());
                        stream.push_str(" and ");
                        stream.push_str(&generic.to_string());
                        stream.push_str(")");
                        report_error(stream);
                    }
                    matching_generic = Some(generic.clone());
                }
            }

            if matching_generic.is_none() {
                let mut stream = std::string::String::new();
                if generic_list.is_empty() {
                    stream.push_str(&("no generic defined with the name ".to_string() + &decl.name.to_string()));
                    report_error(stream.clone());
                }
                stream.push_str(&("specialization of ".to_string() + &decl.name.to_string()));
                stream.push_str(" doesn't match any generic declaration\n");
                stream.push_str("specialization signature:");
                stream.push_str(&("\n  ".to_string() + &signature_with_types.to_string()));
                stream.push_str("\ncandidates are:");
                for generic in &generic_list {
                    stream.push_str(&("\n  ".to_string()
                        + &DeclarationVisitor::make_specialized_signature(SpecializationKey {
                            generic: generic.clone(),
                            specialized_types: TypeVisitor::compute_type_vector(&decl.generic_parameters),
                        })
                        .to_string()));
                }
                report_error(stream);
            }

            if GlobalContext::collect_language_server_data() {
                LanguageServerData::add_definition(
                    decl.name.pos,
                    matching_generic.as_ref().unwrap().identifier_position(),
                );
            }

            let generic_declaration = matching_generic.as_ref().unwrap().declaration();

            DeclarationVisitor::specialize(
                SpecializationKey {
                    generic: matching_generic.unwrap(),
                    specialized_types: TypeVisitor::compute_type_vector(&decl.generic_parameters),
                },
                generic_declaration,
                Some(decl),
                decl.body.clone(),
                decl.pos,
            );
        }

        fn visit_extern_const_declaration(decl: &ExternConstDeclaration) {
            let type_ = TypeVisitor::compute_type(&decl.type_);
            if !type_.is_constexpr() {
                let mut stream = std::string::String::new();
                stream.push_str("extern constants must have constexpr type, but found: \"");
                stream.push_str(&type_.to_string());
                stream.push_str("\"\n");
                report_error(stream);
            }

            let constant = Declarations::declare_extern_constant(decl.name.clone(), type_, decl.literal.clone());
            if GlobalContext::collect_kythe_data() {
                KytheData::add_constant_definition(&constant);
            }
        }

        fn visit_cpp_include_declaration(decl: &CppIncludeDeclaration) {
            GlobalContext::add_cpp_include(decl.include_path.clone());
        }

        fn declare_specialized_types(key: &SpecializationKey) {
            let mut i = 0;
            let generic_parameter_count = key.generic.generic_parameters().len();
            if generic_parameter_count != key.specialized_types.len() {
                let mut stream = std::string::String::new();
                stream.push_str(&("Wrong generic argument count for specialization of \"".to_string() + &key.generic.name()));
                stream.push_str("\", expected: ");
                stream.push_str(&generic_parameter_count.to_string());
                stream.push_str(", actual: ");
                stream.push_str(&key.specialized_types.len().to_string());
                report_error(stream);
            }

            for type_ in &key.specialized_types {
                let generic_type_name = &key.generic.generic_parameters()[i].name;
                let alias = Declarations::declare_type(generic_type_name.clone(), type_.clone());
                alias.set_is_user_defined(false);
                i += 1;
            }
        }

        fn make_specialized_signature(key: SpecializationKey) -> Signature {
            CurrentScope::set_scope(key.generic.parent_scope().clone());
            let tmp_namespace = Namespace {
                name: "_tmp".to_string(),
                declarations: vec![],
            };

            CurrentScope::set_scope(Rc::new(tmp_namespace));
            DeclarationVisitor::declare_specialized_types(&key);
            let signature = TypeVisitor::make_signature(key.generic.declaration());
            CurrentScope::clear_scope();
            signature
        }

        fn specialize_implicit(key: SpecializationKey) -> Rc<dyn Callable> {
            if key.generic.callable_body().is_none()
                && IntrinsicDeclaration::dynamic_cast(key.generic.declaration()).is_none()
            {
                report_error(
                    "missing specialization of ".to_string()
                        + &key.generic.name()
                        + " with types <"
                        + &key.specialized_types.to_string()
                        + "> declared at "
                        + &key.generic.position().to_string(),
                );
            }
            let requester = SpecializationRequester {
                source_position: CurrentSourcePosition::get(),
                current_scope: CurrentScope::get().clone(),
                name: "".to_string(),
            };
            CurrentScope::set_scope(key.generic.parent_scope().clone());
            let result = DeclarationVisitor::specialize(
                key.clone(),
                key.generic.declaration(),
                None,
                key.generic.callable_body(),
                CurrentSourcePosition::get(),
            );
            result.set_is_user_defined(false);
            let mut requester_mut = SpecializationRequester {
                source_position: CurrentSourcePosition::get(),
                current_scope: CurrentScope::get().clone(),
                name: "".to_string(),
            };
            requester_mut.name = result.readable_name();
            result.set_specialization_requester(requester_mut);
            CurrentScope::set_scope(result.clone());
            DeclarationVisitor::declare_specialized_types(&key);
            result
        }

        fn specialize(
            key: SpecializationKey,
            declaration: &dyn CallableDeclarationBase,
            explicit_specialization: Option<&SpecializationDeclaration>,
            body: Option<Rc<Statement>>,
            position: SourcePosition,
        ) -> Rc<dyn Callable> {
            CurrentSourcePosition::set(position);
            let generic_parameter_count = key.generic.generic_parameters().len();
            if generic_parameter_count != key.specialized_types.len() {
                let mut stream = std::string::String::new();
                stream.push_str(&("number of template parameters (".to_string() + &key.specialized_types.len().to_string()));
                stream.push_str(") to intantiation of generic ");
                stream.push_str(&declaration.name().to_string());
                stream.push_str(" doesnt match the generic's declaration (");
                stream.push_str(&generic_parameter_count.to_string());
                stream.push_str(")");
                report_error(stream);
            }
            if key.generic.get_specialization(&key.specialized_types).is_some() {
                report_error(
                    "cannot redeclare specialization of ".to_string()
                        + &key.generic.name()
                        + " with types <"
                        + &key.specialized_types.to_string()
                        + ">",
                );
            }

            let type_signature = if let Some(explicit_specialization) = explicit_specialization {
                TypeVisitor::make_signature(explicit_specialization)
            } else {
                DeclarationVisitor::make_specialized_signature(key.clone())
            };

            let generated_name = Declarations::get_generated_callable_name(
                declaration.name().value.clone(),
                &key.specialized_types,
            );
            let mut readable_name = std::string::String::new();
            readable_name.push_str(&(declaration.name().value.clone() + "<"));
            let mut first = true;
            for t in &key.specialized_types {
                if !first {
                    readable_name.push_str(", ");
                }
                readable_name.push_str(&t.to_string());
                first = false;
            }
            readable_name.push_str(">");
            let callable: Rc<dyn Callable> = if let Some(_macro_declaration) = declaration.as_any().downcast_ref::<MacroDeclaration>() {
                Declarations::create_torque_macro(
                    generated_name.clone(),
                    readable_name.clone(),
                    false,
                    type_signature.clone(),
                    body.clone().unwrap(),
                    true,
                )
            } else if let Some(_intrinsic_declaration) = declaration.as_any().downcast_ref::<IntrinsicDeclaration>() {
                Declarations::create_intrinsic(declaration.name().value.clone(), type_signature.clone())
            } else {
                let builtin = declaration.as_any().downcast_ref::<BuiltinDeclaration>().unwrap();
                let mut use_counter_name: Option<String> = None;
                if let Some(torque_builtin) = declaration.as_any().downcast_ref::<TorqueBuiltinDeclaration>() {
                    use_counter_name = torque_builtin.use_counter_name.clone();
                } else {
                    use_counter_name = None;
                }
                DeclarationVisitor::create_builtin(
                    builtin,
                    GlobalContext::make_unique_name(generated_name.clone()),
                    readable_name.clone(),
                    type_signature.clone(),
                    use_counter_name,
                    body.clone(),
                )
            };
            key.generic.add_specialization(key.specialized_types.clone(), callable.clone());
            callable
        }
    }

    #[derive(Clone, Debug)]
    pub struct SpecializationKey {
        pub generic: Rc<GenericCallable>,
        pub specialized_types: Vec<Rc<Type>>,
    }

    pub trait BuiltinDeclarationBase: Declaration {
        fn javascript_linkage(&self) -> bool;
        fn parameters(&self) -> &ParameterList;
        fn name(&self) -> &Identifier;
        fn as_any(&self) -> &dyn std::any::Any;
    }

    impl BuiltinDeclarationBase for ExternalBuiltinDeclaration {
        fn javascript_linkage(&self) -> bool {
            self.javascript_linkage
        }
        fn parameters(&self) -> &ParameterList {
            &self.parameters
        }
        fn name(&self) -> &Identifier {
            &self.name
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl BuiltinDeclarationBase for TorqueBuiltinDeclaration {
        fn javascript_linkage(&self) -> bool {
            self.javascript_linkage
        }
        fn parameters(&self) -> &ParameterList {
            &self.parameters
        }
        fn name(&self) -> &Identifier {
            &self.name
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    pub trait CallableDeclarationBase {
        fn name(&self) -> &Identifier;
        fn as_any(&self) -> &dyn std::any::Any;
    }

    impl CallableDeclarationBase for MacroDeclaration {
        fn name(&self) -> &Identifier {
            &self.name
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl CallableDeclarationBase for IntrinsicDeclaration {
        fn name(&self) -> &Identifier {
            &self.name
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl CallableDeclarationBase for BuiltinDeclaration {
        fn name(&self) -> &Identifier {
            &self.name
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn filter_declarables<T: 'static>(
        declarables: Option<Vec<Rc<dyn Declarable>>>,
    ) -> Vec<Rc<T>> {
        let mut result = Vec::new();
        if let Some(decls) = declarables {
            for decl in decls {
                if let Some(t) = decl.downcast_rc::<T>() {
                    result.push(t);
                }
            }
        }
        result


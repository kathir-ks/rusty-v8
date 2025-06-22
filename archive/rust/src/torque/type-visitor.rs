// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod type_visitor {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::option::Option;

    pub type MaybeSpecializationKey = Option<usize>;

    pub struct TypeVector(pub Vec<Rc<Type>>);

    pub trait TypeExpression {}

    pub struct Type;

    pub struct AbstractType;

    pub struct StructType;

    pub struct BitFieldStructType;

    pub struct ClassType;

    pub struct ClassDeclaration;

    pub struct StructDeclaration;

    pub struct CallableDeclaration;

    pub struct TypeDeclaration;

    pub struct AbstractTypeDeclaration;

    pub struct TypeAliasDeclaration;

    pub struct BitFieldStructDeclaration;

    pub struct Scope;

    pub struct Signature;

    pub struct TypeVisitor;

    impl TypeVisitor {
        pub fn compute_type_vector(v: &Vec<Box<dyn TypeExpression>>) -> TypeVector {
            let mut result = TypeVector(Vec::new());
            for t in v {
                result.0.push(Self::compute_type(t.as_ref()));
            }
            result
        }

        pub fn compute_type(type_expression: &dyn TypeExpression) -> Rc<Type> {
            // TODO: Implement the actual computation of the type based on the type expression
            Rc::new(Type)
        }

        pub fn visit_class_fields_and_methods(
            class_type: &mut ClassType,
            class_declaration: &ClassDeclaration,
        ) {
            // TODO: Implement visiting class fields and methods
        }

        pub fn visit_struct_methods(
            struct_type: &mut StructType,
            struct_declaration: &StructDeclaration,
        ) {
            // TODO: Implement visiting struct methods
        }

        pub fn make_signature(declaration: &CallableDeclaration) -> Signature {
            // TODO: Implement making a signature
            Signature
        }

        // Can return either StructType or BitFieldStructType, since they can both be
        // used in struct expressions like `MyStruct{ a: 0, b: foo }`
        pub fn compute_type_for_struct_expression(
            type_expression: &dyn TypeExpression,
            term_argument_types: &Vec<&Type>,
        ) -> Rc<Type> {
            // TODO: Implement the actual computation of the type based on the type expression
            Rc::new(Type)
        }

        fn compute_type_type_declaration(
            decl: &TypeDeclaration,
            specialized_from: MaybeSpecializationKey,
            specialization_requester: Option<&Scope>,
        ) -> Rc<Type> {
            // TODO: Implement
            Rc::new(Type)
        }

        fn compute_type_abstract_type_declaration(
            decl: &AbstractTypeDeclaration,
            specialized_from: MaybeSpecializationKey,
        ) -> Rc<AbstractType> {
            // TODO: Implement
            Rc::new(AbstractType)
        }

        fn compute_type_type_alias_declaration(
            decl: &TypeAliasDeclaration,
            specialized_from: MaybeSpecializationKey,
        ) -> Rc<Type> {
            // TODO: Implement
            Rc::new(Type)
        }

        fn compute_type_bit_field_struct_declaration(
            decl: &BitFieldStructDeclaration,
            specialized_from: MaybeSpecializationKey,
        ) -> Rc<BitFieldStructType> {
            // TODO: Implement
            Rc::new(BitFieldStructType)
        }

        fn compute_type_struct_declaration(
            decl: &StructDeclaration,
            specialized_from: MaybeSpecializationKey,
        ) -> Rc<StructType> {
            // TODO: Implement
            Rc::new(StructType)
        }

        fn compute_type_class_declaration(
            decl: &ClassDeclaration,
            specialized_from: MaybeSpecializationKey,
        ) -> Rc<ClassType> {
            // TODO: Implement
            Rc::new(ClassType)
        }
    }
}
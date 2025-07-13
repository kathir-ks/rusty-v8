// Converted from V8 C++ source files:
// Header: type-visitor.h
// Implementation: type-visitor.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ast {
    pub enum Kind {
        kTypeDeclaration,
    }
}
pub mod types {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub enum Type {
        AbstractType(AbstractType),
        StructType(StructType),
        BitFieldStructType(BitFieldStructType),
        ClassType(ClassType),
        UnionType(UnionType),
        BuiltinPointerType(BuiltinPointerType),
        StrongTaggedType,
        ObjectType,
        BoolType,
        AnyType,
        UnsignedInteger(UnsignedIntegerType),
    }

    impl Type {
        pub fn add_alias(&self, _alias: &str) {}
        pub fn is_union_type(&self) -> bool {
            match self {
                Type::UnionType(_) => true,
                _ => false,
            }
        }

        pub fn compute_name(name: &str, _specialized_from: &MaybeSpecializationKey) -> String {
            name.to_string()
        }

        pub fn is_subtype_of(&self, other: &Type) -> bool {
            match (self, other) {
                (_, Type::AnyType) => true,
                (Type::BoolType, Type::AbstractType(at)) if at.name == "bool" => true,
                (Type::StructType(_), Type::ObjectType) => true,
                (Type::ClassType(_), Type::ObjectType) => true,
                (Type::ClassType(ct), other_type) => {
                    if let Some(super_class) = ct.super_class.as_ref() {
                        super_class.is_subtype_of(other_type)
                    } else {
                        false
                    }
                }
                (Type::ObjectType, _) => false,
                _ => std::mem::discriminant(self) == std::mem::discriminant(other),
            }
        }
        pub fn is_struct_type(&self) -> bool {
            match self {
                Type::StructType(_) => true,
                _ => false,
            }
        }

        pub fn is_bitfield_struct_type(&self) -> bool {
            match self {
                Type::BitFieldStructType(_) => true,
                _ => false,
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                Type::AbstractType(at) => at.name.clone(),
                Type::StructType(st) => st.name.clone(),
                Type::BitFieldStructType(bfst) => bfst.name.clone(),
                Type::ClassType(ct) => ct.name.clone(),
                Type::UnionType(_) => "UnionType".to_string(),
                Type::BuiltinPointerType(_) => "BuiltinPointerType".to_string(),
                Type::StrongTaggedType => "StrongTaggedType".to_string(),
                Type::ObjectType => "ObjectType".to_string(),
                Type::BoolType => "BoolType".to_string(),
                Type::AnyType => "AnyType".to_string(),
                Type::UnsignedInteger(uit) => uit.name.clone(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct AbstractType {
        pub parent: Option<Box<Type>>,
        pub name: String,
        pub flags: ClassFlags,
        pub generates: String,
        pub non_constexpr_version: Option<Box<Type>>,
        pub specialized_from: Option<MaybeSpecializationKey>,
    }

    impl AbstractType {
        pub fn new(
            parent: Option<Type>,
            name: String,
            flags: ClassFlags,
            generates: String,
            non_constexpr_version: Option<Type>,
            specialized_from: Option<MaybeSpecializationKey>,
        ) -> Self {
            AbstractType {
                parent: parent.map(Box::new),
                name,
                flags,
                generates,
                non_constexpr_version: non_constexpr_version.map(Box::new),
                specialized_from,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct StructType {
        pub name: String,
        pub nspace: Scope,
        pub fields: Vec<Field>,
        pub declaration: *mut StructDeclaration,
        pub specialized_from: Option<MaybeSpecializationKey>,
    }
    impl StructType {
        pub fn new(name: String, nspace: Scope, declaration: *mut StructDeclaration, specialized_from: Option<MaybeSpecializationKey>) -> Self {
            StructType {
                name,
                nspace,
                fields: Vec::new(),
                declaration,
                specialized_from
            }
        }
        pub fn register_field(&mut self, field: Field) {
            self.fields.push(field);
        }
    }

    #[derive(Debug, Clone)]
    pub struct BitFieldStructType {
        pub name: String,
        pub parent: Box<Type>,
        pub fields: Vec<BitField>,
    }
    impl BitFieldStructType {
        pub fn new(name: String, parent: Type) -> Self {
            BitFieldStructType {
                name,
                parent: Box::new(parent),
                fields: Vec::new(),
            }
        }

        pub fn register_field(&mut self, field: BitField) {
            self.fields.push(field);
        }
    }

    #[derive(Debug, Clone)]
    pub struct ClassType {
        pub name: String,
        pub flags: ClassFlags,
        pub generates: String,
        pub declaration: *mut ClassDeclaration,
        pub super_class: Option<Box<ClassType>>,
        pub alias: *mut TypeAlias,
        pub header_size_: usize,
        pub size_: ResidueClass,
        pub fields: Vec<Field>,
    }
    impl ClassType {
        pub fn new(name: String, flags: ClassFlags, generates: String, declaration: *mut ClassDeclaration, super_class: Option<ClassType>, alias: *mut TypeAlias) -> Self {
            ClassType {
                name,
                flags,
                generates,
                declaration,
                super_class: super_class.map(Box::new),
                alias,
                header_size_: 0,
                size_: ResidueClass::Known(0),
                fields: Vec::new(),
            }
        }

        pub fn get_super_class(&self) -> Option<&ClassType> {
            self.super_class.as_deref()
        }
        pub fn is_shape(&self) -> bool {
            self.flags.bits & ClassFlag::kIsShape as u32 != 0
        }

        pub fn has_undefined_layout(&self) -> bool {
            self.flags.bits & ClassFlag::kUndefinedLayout as u32 != 0
        }

        pub fn should_export(&self) -> bool {
            self.flags.bits & ClassFlag::kExport as u32 != 0
        }

        pub fn is_extern(&self) -> bool {
            self.flags.bits & ClassFlag::kExtern as u32 != 0
        }
        pub fn register_field(&mut self, field: Field) -> Field {
            self.fields.push(field.clone());
            field
        }

        pub fn generate_accessors(&mut self) {}

        pub fn header_size(&self) -> usize {
            self.header_size_
        }

        pub fn size(&self) -> ResidueClass {
            self.size_
        }

        pub fn name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct UnionType {
        pub a: Box<Type>,
        pub b: Box<Type>,
    }
    impl UnionType {
        pub fn new(a: Type, b: Type) -> Self {
            UnionType {
                a: Box::new(a),
                b: Box::new(b),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BuiltinPointerType {
        pub argument_types: Vec<Type>,
        pub return_type: Box<Type>,
    }
    impl BuiltinPointerType {
        pub fn new(argument_types: Vec<Type>, return_type: Type) -> Self {
            BuiltinPointerType {
                argument_types,
                return_type: Box::new(return_type),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Field {
        pub pos: SourcePosition,
        pub class_type: *mut ClassType,
        pub index: Option<ClassFieldIndexInfo>,
        pub name_and_type: NameAndType,
        pub offset: Option<usize>,
        pub custom_weak_marking: bool,
        pub const_qualified: bool,
        pub synchronization: FieldSynchronization,
    }
    impl Field {
        pub fn get_field_size_information(&self) -> (ResidueClass, bool) {
            (ResidueClass::Known(8), false)
        }

        pub fn validate_alignment(&self, _class_offset: ResidueClass) {}
    }

    #[derive(Debug, Clone)]
    pub struct BitField {
        pub pos: SourcePosition,
        pub name_and_type: NameAndType,
        pub offset: i32,
        pub num_bits: i32,
    }

    #[derive(Debug, Clone)]
    pub struct NameAndType {
        pub name: String,
        pub the_type: Type,
    }

    #[derive(Debug, Clone)]
    pub struct ClassFieldIndexInfo {
        pub expr: *mut IntegerLiteralExpression,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ClassFlag {
        kIsShape = 1 << 0,
        kExtern = 1 << 1,
        kUndefinedLayout = 1 << 2,
        kExport = 1 << 3,
        kGenerateBodyDescriptor = 1 << 4,
        kGenerateUniqueMap = 1 << 5,
        kAbstract = 1 << 6,
        kGenerateFactoryFunction = 1 << 7,
        kHasSameInstanceTypeAsParent = 1 << 8,
        kDoNotGenerateCast = 1 << 9,
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct ClassFlags: u32 {
            const kNone = 0;
            const kIsShape = 1 << 0;
            const kExtern = 1 << 1;
            const kUndefinedLayout = 1 << 2;
            const kExport = 1 << 3;
            const kGenerateBodyDescriptor = 1 << 4;
            const kGenerateUniqueMap = 1 << 5;
            const kAbstract = 1 << 6;
            const kGenerateFactoryFunction = 1 << 7;
            const kHasSameInstanceTypeAsParent = 1 << 8;
            const kDoNotGenerateCast = 1 << 9;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FieldSynchronization {
        kNone,
    }
    #[derive(Debug, Clone)]
    pub struct UnsignedIntegerType {
        pub name: String,
    }
}

pub mod torque {
    use std::any::Any;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt;
    use std::rc::Rc;

    pub use super::ast;
    pub use super::types;
    use types::{
        AbstractType, BitFieldStructType, BuiltinPointerType, ClassFlags, ClassType, Field,
        NameAndType, StructType, Type, UnionType, UnsignedIntegerType,
    };

    pub type TypeVector = Vec<Type>;
    pub type LabelDeclarationVector = Vec<LabelDeclaration>;
    pub type MaybeSpecializationKey = Option<SpecializationKey>;

    thread_local! {
        pub static CURRENT_SOURCE_POSITION: RefCell<SourcePosition> = RefCell::new(SourcePosition::new());
        pub static CURRENT_SCOPE: RefCell<*mut Scope> = RefCell::new(std::ptr::null_mut());
    }

    pub struct CurrentSourcePosition {}

    impl CurrentSourcePosition {
        pub fn get() -> SourcePosition {
            CURRENT_SOURCE_POSITION.with(|pos| *pos.borrow())
        }

        pub struct Scope {
            pos: SourcePosition,
        }

        impl Scope {
            pub fn new(pos: SourcePosition) -> Self {
                CURRENT_SOURCE_POSITION.with(|current_pos| {
                    *current_pos.borrow_mut() = pos;
                });
                Scope { pos }
            }
        }
    }
    pub struct CurrentScope {}

    impl CurrentScope {
        pub fn get() -> *mut Scope {
            CURRENT_SCOPE.with(|scope| *scope.borrow())
        }

        pub struct Scope<'a> {
            scope: &'a mut *mut Scope,
        }
        impl<'a> Scope<'a> {
            pub fn new(scope: &'a mut *mut Scope, new_scope: *mut Scope) -> Self {
                CURRENT_SCOPE.with(|current_scope| {
                    *current_scope.borrow_mut() = new_scope;
                });
                Scope { scope }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SourcePosition {
        line: i32,
        column: i32,
    }

    impl SourcePosition {
        pub fn new() -> Self {
            SourcePosition { line: 0, column: 0 }
        }

        pub fn invalid() -> Self {
            SourcePosition {
                line: -1,
                column: -1,
            }
        }

        pub fn get(&self) -> SourcePosition {
            *self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SpecializationKey {
        pub generic: *mut GenericType,
        pub specialized_types: TypeVector,
    }

    #[derive(Debug, Clone)]
    pub struct Signature {
        pub parameter_names: Vec<String>,
        pub arguments_variable: Option<String>,
        pub parameter_types: ParameterTypes,
        pub implicit_count: i32,
        pub return_type: Type,
        pub labels: LabelDeclarationVector,
        pub transitioning: bool,
    }

    #[derive(Debug, Clone)]
    pub struct ParameterTypes {
        pub types: TypeVector,
        pub has_varargs: bool,
    }
    impl ParameterTypes {
        pub fn new(types: TypeVector, has_varargs: bool) -> Self {
            ParameterTypes { types, has_varargs }
        }
    }

    #[derive(Debug, Clone)]
    pub struct LabelDeclaration {
        pub name: String,
        pub types: TypeVector,
    }

    #[derive(Debug, Clone)]
    pub struct Scope {
        specialization_requester: Option<SpecializationRequester>,
    }
    impl Scope {
        pub fn new() -> Self {
            Scope {
                specialization_requester: None,
            }
        }

        pub fn set_specialization_requester(&mut self, requester: SpecializationRequester) {
            self.specialization_requester = Some(requester);
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SpecializationRequester {
        pub position: SourcePosition,
        pub specialization_requester: *mut Scope,
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct Declarations {}

    impl Declarations {
        pub fn declare_type(name: &str, the_type: &Type) -> TypeAlias {
            TypeAlias::new(name.to_string(), the_type.clone())
        }

        pub fn lookup_type_alias(qualified_name: QualifiedName) -> TypeAlias {
            TypeAlias::new(qualified_name.name, Type::AnyType)
        }

        pub fn try_lookup_generic_type(_qualified_name: QualifiedName) -> Option<*mut GenericType> {
            None
        }
        pub fn lookup_unique_generic_type(_qualified_name: QualifiedName) -> *mut GenericType {
            let mut gt = GenericType::new("".to_string());
            &mut gt
        }

        pub fn create_method(
            _container_type: &Type,
            method_name: String,
            signature: Signature,
            _body: *mut Statement,
        ) -> Method {
            Method::new(method_name, signature)
        }
    }

    #[derive(Debug, Clone)]
    pub struct TypeAlias {
        pub name: String,
        pub the_type: Type,
        pub is_user_defined: bool,
        pub delayed_: Option<*mut ClassDeclaration>,
    }
    impl TypeAlias {
        pub fn new(name: String, the_type: Type) -> Self {
            TypeAlias {
                name,
                the_type,
                is_user_defined: true,
                delayed_: None,
            }
        }

        pub fn the_type(&self) -> &Type {
            &self.the_type
        }

        pub fn set_is_user_defined(&mut self, is_user_defined: bool) {
            self.is_user_defined = is_user_defined;
        }

        pub fn get_declaration_position(&self) -> SourcePosition {
            SourcePosition::new()
        }
    }

    #[derive(Debug, Clone)]
    pub struct GenericType {
        pub name: String,
    }
    impl GenericType {
        pub fn new(name: String) -> Self {
            GenericType { name }
        }
        pub fn parent_scope(&self) -> Scope {
            Scope::new()
        }
        pub fn declaration(&self) -> *mut StructDeclaration {
            std::ptr::null_mut()
        }
        pub fn generic_parameters(&self) -> Vec<Identifier> {
            Vec::new()
        }
    }

    #[derive(Debug, Clone)]
    pub struct Method {
        pub name: String,
        pub signature: Signature,
    }
    impl Method {
        pub fn new(name: String, signature: Signature) -> Self {
            Method { name, signature }
        }
        pub fn set_position(&mut self, _pos: SourcePosition) {}
        pub fn set_identifier_position(&mut self, _pos: SourcePosition) {}
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct QualifiedName {
        pub namespace_qualification: Vec<String>,
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct TypeArgumentInference {
        has_failed: bool,
        failure_reason: String,
    }
    impl TypeArgumentInference {
        pub fn new(
            _generic_parameters: Vec<Identifier>,
            _explicit_type_arguments: TypeVector,
            _term_parameters: Vec<*mut TypeExpression>,
            _term_argument_types: Vec<Option<&Type>>,
        ) -> Self {
            TypeArgumentInference {
                has_failed: false,
                failure_reason: String::new(),
            }
        }

        pub fn has_failed(&self) -> bool {
            self.has_failed
        }

        pub fn get_failure_reason(&self) -> &str {
            &self.failure_reason
        }

        pub fn get_result(&self) -> TypeVector {
            Vec::new()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TypeOracle {}
    impl TypeOracle {
        pub fn get_abstract_type(
            parent_type: Option<&Type>,
            name: String,
            flags: ClassFlags,
            generates: String,
            non_constexpr_version: Option<&Type>,
            specialized_from: MaybeSpecializationKey,
        ) -> *mut Type {
            let abs_type = AbstractType::new(
                parent_type.cloned(),
                name,
                flags,
                generates,
                non_constexpr_version.cloned(),
                specialized_from,
            );
            let the_type = Type::AbstractType(abs_type);
            Box::into_raw(Box::new(the_type))
        }

        pub fn get_struct_type(decl: *mut StructDeclaration, specialized_from: MaybeSpecializationKey) -> *mut StructType {
            let struct_type = StructType::new("".to_string(), Scope::new(), decl, specialized_from);
            Box::into_raw(Box::new(struct_type)) as *mut StructType
        }

        pub fn get_bitfield_struct_type(parent: &Type, decl: *mut BitFieldStructDeclaration) -> *mut BitFieldStructType {
            let name = "bitfield".to_string();
            let bfst = BitFieldStructType::new(name, parent.clone());
            Box::into_raw(Box::new(bfst))
        }

        pub fn get_class_type(
            super_type: &Type,
            name: String,
            flags: ClassFlags,
            generates: String,
            decl: *mut ClassDeclaration,
            alias: *mut TypeAlias,
        ) -> *mut ClassType {
            let super_class = if let Type::ClassType(ct) = super_type {
                Some(ct.clone())
            } else {
                None
            };
            let class_type = ClassType::new(name, flags, generates, decl, super_class, alias);
            Box::into_raw(Box::new(class_type))
        }

        pub fn get_union_type(a: &Type, b: &Type) -> *mut Type {
            let union_type = UnionType::new(a.clone(), b.clone());
            Box::into_raw(Box::new(Type::UnionType(union_type)))
        }

        pub fn get_builtin_pointer_type(argument_types: Vec<Type>, return_type: &Type) -> *mut Type {
            let builtin_pointer_type = BuiltinPointerType::new(argument_types, return_type.clone());
            Box::into_raw(Box::new(Type::BuiltinPointerType(builtin_pointer_type)))
        }
        pub fn get_generic_type_instance(_generic_type: *mut GenericType, _compute_type_vector: Vec<Type>) -> *mut Type {
             Box::into_raw(Box::new(Type::AnyType))
        }

        pub fn create_generic_type_instantiation_namespace() -> *mut Scope {
            Box::into_raw(Box::new(Scope::new()))
        }
        pub fn get_strong_tagged_type() -> Type {
            Type::StrongTaggedType
        }
        pub fn get_object_type() -> Type {
            Type::ObjectType
        }
        pub fn get_bool_type() -> Type {
            Type::BoolType
        }
        pub fn get_any_type() -> Type {
            Type::AnyType
        }
    }

    #[derive(Debug, Clone)]
    pub struct Identifier {
        pub name: String,
    }
    impl Identifier {
        pub fn new(name: String) -> Self {
            Identifier { name }
        }
    }
    pub fn make_node<T>(name: String) -> Identifier {
        Identifier::new(name)
    }
    pub const KTHIS_PARAMETER_NAME: &str = "this";

    #[derive(Debug, Clone)]
    pub struct TypeExpression {}
    #[derive(Debug, Clone)]
    pub struct BasicTypeExpression {
        pub namespace_qualification: Vec<String>,
        pub name: Identifier,
        pub generic_arguments: Vec<*mut TypeExpression>,
    }
    impl BasicTypeExpression {
        pub fn dynamic_cast(type_expression: *mut TypeExpression) -> Option<*mut BasicTypeExpression> {
            if type_expression.is_null() {
                return None;
            }
            Some(type_expression as *mut BasicTypeExpression)
        }
    }

    #[derive(Debug, Clone)]
    pub struct UnionTypeExpression {
        pub a: *mut TypeExpression,
        pub b: *mut TypeExpression,
    }
    impl UnionTypeExpression {
        pub fn dynamic_cast(type_expression: *mut TypeExpression) -> Option<*mut UnionTypeExpression> {
            if type_expression.is_null() {
                return None;
            }
            Some(type_expression as *mut UnionTypeExpression)
        }
    }

    #[derive(Debug, Clone)]
    pub struct FunctionTypeExpression {
        pub parameters: Vec<*mut TypeExpression>,
        pub return_type: *mut TypeExpression,
    }
    impl FunctionTypeExpression {
        pub fn dynamic_cast(type_expression: *mut TypeExpression) -> Option<*mut FunctionTypeExpression> {
            if type_expression.is_null() {
                return None;
            }
            Some(type_expression as *mut FunctionTypeExpression)
        }
    }

    #[derive(Debug, Clone)]
    pub struct PrecomputedTypeExpression {
        pub the_type: Type,
    }
    impl PrecomputedTypeExpression {
        pub fn cast(type_expression: *mut TypeExpression) -> *mut PrecomputedTypeExpression {
            type_expression as *mut PrecomputedTypeExpression
        }
    }

    #[derive(Debug, Clone)]
    pub struct Declaration {
        pub pos: SourcePosition,
    }

    #[derive(Debug, Clone)]
    pub struct TypeDeclaration {
        pub base: Declaration,
        pub name: Identifier,
        pub kind: ast::Kind,
    }

    impl TypeDeclaration {
        pub fn get_pos(&self) -> SourcePosition {
            self.base.pos
        }
    }

    #[derive(Debug, Clone)]
    pub struct AbstractTypeDeclaration {
        pub base: TypeDeclaration,
        pub extends: Option<*mut TypeExpression>,
        pub flags: ClassFlags,
        pub generates: Option<String>,
    }
    impl AbstractTypeDeclaration {
        pub fn is_constexpr(&self) -> bool {
            false
        }
        pub fn is_transient(&self) -> bool {
            false
        }

        pub fn dynamic_cast(decl: *mut Declaration) -> Option<*mut AbstractTypeDeclaration> {
            if decl.is_null() {
                return None;
            }
            Some(decl as *mut AbstractTypeDeclaration)
        }
    }

    #[derive(Debug, Clone)]
    pub struct TypeAliasDeclaration {
        pub base: TypeDeclaration,
        pub type_: *mut TypeExpression,
    }
    impl TypeAliasDeclaration {
        pub fn dynamic_cast(decl: *mut Declaration) -> Option<*mut TypeAliasDeclaration> {
            if decl.is_null() {
                return None;
            }
            Some(decl as *mut TypeAliasDeclaration)
        }
    }

    #[derive(Debug, Clone)]
    pub struct BitFieldStructDeclaration {
        pub base: TypeDeclaration,
        pub parent: *mut TypeExpression,
        pub fields: Vec<BitFieldStructField>,
    }
    impl BitFieldStructDeclaration {
        pub fn dynamic_cast(decl: *mut Declaration) -> Option<*mut BitFieldStructDeclaration> {
            if decl.is_null() {
                return None;
            }
            Some(decl as *mut BitFieldStructDeclaration)
        }
    }

    #[derive(Debug, Clone)]
    pub struct StructDeclaration {
        pub base: TypeDeclaration,
        pub fields: Vec<StructField>,
        pub methods: Vec<*mut Declaration>,
    }
    impl StructDeclaration {
        pub fn dynamic_cast(decl: *mut Declaration) -> Option<*mut StructDeclaration> {
            if decl.is_null() {
                return None;
            }
            Some(decl as *mut StructDeclaration)
        }
    }

    #[derive(Debug, Clone)]
    pub struct ClassDeclaration {
        pub base: TypeDeclaration,
        pub super_: *mut TypeExpression,
        pub fields: Vec<ClassFieldExpression>,
        pub flags: ClassFlags,
        pub generates: Option<String>,
        pub methods: Vec<*mut Declaration>,
    }
    impl ClassDeclaration {
        pub fn dynamic_cast(decl: *mut Declaration) -> Option<*mut ClassDeclaration> {
            if decl.is_null() {
                return None;
            }
            Some(decl as *mut ClassDeclaration)
        }
    }
    #[derive(Debug, Clone)]
    pub struct BitFieldStructField {
        pub name_and_type: NameAndTypeExpression,
        pub num_bits: i32,
    }
    #[derive(Debug, Clone)]
    pub struct StructField {
        pub name_and_type: NameAndTypeExpression,
        pub const_qualified: bool,
    }
    #[derive(Debug, Clone)]
    pub struct ClassFieldExpression {
        pub name_and_type: NameAndTypeExpression,
        pub custom_weak_marking: bool,
        pub index: Option<ClassFieldIndexInfo>,
        pub const_qualified: bool,
        pub synchronization: FieldSynchronization,
    }

    #[derive(Debug, Clone)]
    pub struct NameAndTypeExpression {
        pub name: Identifier,
        pub type_: *mut TypeExpression,
    }

    #[derive(Debug, Clone)]
    pub struct CallableDeclaration {
        pub parameters: ParameterList,
        pub return_type: *mut TypeExpression,
        pub labels: Vec<Label>,
        pub transitioning: bool,
    }

    #[derive(Debug, Clone)]
    pub struct ParameterList {
        pub names: Vec<String>,
        pub types: Vec<*mut TypeExpression>,
        pub has_varargs: bool,
        pub arguments_variable: Option<String>,
        pub implicit_count: i32,
    }

    #[derive(Debug, Clone)]
    pub struct Label {
        pub name: String,
        pub types: Vec<*mut TypeExpression>,
    }

    #[derive(Debug, Clone)]
    pub struct TorqueMacroDeclaration {
        pub base: Declaration,
        pub name: Identifier,
        pub body: *mut Statement,
    }
    impl TorqueMacroDeclaration {
        pub fn dynamic_cast(declaration: *mut Declaration) -> Option<*mut TorqueMacroDeclaration> {
            if declaration.is_null() {
                return None;
            }
            Some(declaration as *mut TorqueMacroDeclaration)
        }
    }
    #[derive(Debug, Clone)]
    pub struct Statement {}
    #[derive(Debug, Clone)]
    pub struct IntegerLiteralExpression {
        pub value: i64,
    }
    impl IntegerLiteralExpression {
        pub fn dynamic_cast(expr: *mut ClassFieldIndexInfo) -> Option<*mut IntegerLiteralExpression> {
            if expr.is_null() {
                return None;
            }
            Some(expr as *mut IntegerLiteralExpression)
        }
    }

    pub fn transform_vector<T, F>(vec: Vec<T>, _transform: F) -> Vec<Option<&'static Type>>
    where
        F: Fn(T) -> Option<&'static Type>,
    {
        Vec::new()
    }

    #[derive(Debug, Clone)]
    pub enum ResidueClass {
        Known(usize),
        Unknown(),
    }

    impl ResidueClass {
        pub fn single_value(&self) -> Option<usize> {
            match self {
                ResidueClass::Known(value) => Some(*value),
                ResidueClass::Unknown() => None,
            }
        }
        pub fn unknown() -> Self {
            ResidueClass::Unknown()
        }
    }
    impl std::ops::Add<usize> for ResidueClass {
        type Output = Self;

        fn add(self, other: usize) -> Self {
            match self {
                ResidueClass::Known(value) => ResidueClass::Known(value + other),
                ResidueClass::Unknown() => ResidueClass::Unknown(),
            }
        }
    }
    impl std::ops::Mul<ResidueClass> for ResidueClass {
        type Output = Self;

        fn mul(self, other: ResidueClass) -> Self {
            match (self, other) {
                (ResidueClass::Known(a), ResidueClass::Known(b)) => ResidueClass::Known(a * b),
                _ => ResidueClass::Unknown(),
            }
        }
    }
    impl std::ops::Add for ResidueClass {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            match (self, other) {
                (ResidueClass::Known(a), ResidueClass::Known(b)) => ResidueClass::Known(a + b),
                _ => ResidueClass::Unknown(),
            }
        }
    }
}

pub mod global_context {
    pub fn collect_kythe_data() -> bool {
        false
    }
    pub fn collect_language_server_data() -> bool {
        false
    }
}

pub mod kythe_data {
    use super::torque::{Identifier, SourcePosition, TypeAlias};
    pub fn add_type_use(_pos: SourcePosition, _id: &Identifier) {}
    pub fn add_type_use(_pos: SourcePosition, alias: &TypeAlias) {}
}

pub mod language_server_data {
    use super::torque::SourcePosition;
    pub fn add_definition(_pos1: SourcePosition, _pos2: SourcePosition) {}
}
pub mod error_report {
    use super::torque::SourcePosition;
    #[derive(Debug, Clone)]
    pub struct Error {
        message: String,
        position: SourcePosition,
    }
    impl Error {
        pub fn new(message: String) -> Self {
            Error {
                message,
                position: SourcePosition::invalid(),
            }
        }

        pub fn position(mut self, pos: SourcePosition) ->

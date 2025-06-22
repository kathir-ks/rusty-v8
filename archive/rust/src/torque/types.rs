// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod types {
    use std::any::Any;
    use std::collections::{HashSet, HashMap};
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;
    use std::ops::{BitAnd, BitOr};

    use crate::ast::*;
    use crate::constants::*;
    use crate::source_positions::*;
    use crate::utils::*;

    pub trait TypeBaseTrait {
        fn kind(&self) -> Kind;
        fn is_top_type(&self) -> bool { self.kind() == Kind::TopType }
        fn is_abstract_type(&self) -> bool { self.kind() == Kind::AbstractType }
        fn is_builtin_pointer_type(&self) -> bool { self.kind() == Kind::BuiltinPointerType }
        fn is_union_type(&self) -> bool { self.kind() == Kind::UnionType }
        fn is_bit_field_struct_type(&self) -> bool { self.kind() == Kind::BitFieldStructType }
        fn is_struct_type(&self) -> bool { self.kind() == Kind::StructType }
        fn is_class_type(&self) -> bool { self.kind() == Kind::ClassType }
        fn is_aggregate_type(&self) -> bool { self.is_struct_type() || self.is_class_type() }
    }

    macro_rules! declare_type_boilerplate {
        ($x:ident) => {
            impl $x {
                #[allow(dead_code)]
                pub fn cast<'a>(declarable: &'a dyn TypeBaseTrait) -> &'a $x {
                    assert!(declarable.is_$x());
                    declarable.as_any().downcast_ref::<$x>().unwrap()
                }
                #[allow(dead_code)]
                pub fn cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> &'a mut $x {
                    assert!(declarable.is_$x());
                    declarable.as_any_mut().downcast_mut::<$x>().unwrap()
                }
                #[allow(dead_code)]
                pub fn dynamic_cast<'a>(declarable: &'a dyn TypeBaseTrait) -> Option<&'a $x> {
                    if !declarable.is_$x() { return None; }
                    declarable.as_any().downcast_ref::<$x>()
                }
                #[allow(dead_code)]
                pub fn dynamic_cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> Option<&'a mut $x> {
                    if !declarable.is_$x() { return None; }
                    declarable.as_any_mut().downcast_mut::<$x>()
                }
            }
            impl dyn TypeBaseTrait + '_ {
                #[allow(dead_code)]
                pub fn is_$x(&self) -> bool {
                    self.kind() == Kind::$x
                }
                #[allow(dead_code)]
                pub fn as_any(&self) -> &dyn Any {
                    self
                }
                #[allow(dead_code)]
                pub fn as_any_mut(&mut self) -> &mut dyn Any {
                    self
                }
            }
        };
    }

    pub(crate) use declare_type_boilerplate;

    pub type TypeVector<'a> = Vec<&'a Type>;

    #[derive(Debug, PartialEq, Eq)]
    pub struct SpecializationKey<'a> {
        pub generic: &'a GenericType,
        pub specialized_types: TypeVector<'a>,
    }

    pub type MaybeSpecializationKey<'a> = Option<SpecializationKey<'a>>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TypeChecker {
        // The type of the object. This string is not guaranteed to correspond to a
        // C++ class, but just to a type checker function: for any type "Foo" here,
        // the function IsFoo must exist.
        pub type_: String,
        // If {type} is "MaybeObject", then {weak_ref_to} indicates the corresponding
        // strong object type. Otherwise, {weak_ref_to} is empty.
        pub weak_ref_to: String,
    }

    pub struct Type {
        kind: Kind,
        parent: Option<Box<Type>>,
        aliases: HashSet<String>,
        id: usize,
        specialized_from: MaybeSpecializationKey<'static>, // FIXME: Lifetime
        constexpr_version: Option<Box<Type>>,
    }

    impl Type {
        pub fn new(kind: Kind, parent: Option<Box<Type>>, specialized_from: MaybeSpecializationKey<'static>) -> Self { // FIXME: Lifetime
            static mut NEXT_ID: usize = 0;
            unsafe {
                NEXT_ID += 1;
                Type {
                    kind,
                    parent,
                    aliases: HashSet::new(),
                    id: NEXT_ID,
                    specialized_from,
                    constexpr_version: None,
                }
            }
        }

        pub fn is_subtype_of(&self, supertype: &Type) -> bool {
            if self as *const _ == supertype as *const _ {
                return true;
            }
            if let Some(ref parent) = self.parent {
                parent.is_subtype_of(supertype)
            } else {
                false
            }
        }

        // Default rendering for error messages etc.
        pub fn to_string(&self) -> String {
            // TODO: Implement this properly
            "Type::ToString".to_string()
        }

        // This name is not unique, but short and somewhat descriptive.
        // Used for naming generated code.
        pub fn simple_name(&self) -> String {
            // TODO: Implement this properly
            "Type::SimpleName".to_string()
        }

        pub fn get_handle_type_name(&self, kind: HandleKind, type_name: &str) -> String {
            // TODO: Implement this properly
            format!("Type::GetHandleTypeName({:?}, {})", kind, type_name)
        }

        pub fn tagglified_cpp_type_name(&self) -> String {
            // TODO: Implement this properly
            "Type::TagglifiedCppTypeName".to_string()
        }

        pub fn handlified_cpp_type_name(&self, kind: HandleKind) -> String {
            // TODO: Implement this properly
            format!("Type::HandlifiedCppTypeName({:?})", kind)
        }

        pub fn parent(&self) -> Option<&Type> {
            self.parent.as_ref().map(|p| &**p)
        }

        pub fn is_void(&self) -> bool {
            // TODO: Replace VOID_TYPE_STRING with its actual value
            self.is_abstract_name(VOID_TYPE_STRING)
        }

        pub fn is_never(&self) -> bool {
            // TODO: Replace NEVER_TYPE_STRING with its actual value
            self.is_abstract_name(NEVER_TYPE_STRING)
        }

        pub fn is_bool(&self) -> bool {
            // TODO: Replace BOOL_TYPE_STRING with its actual value
            self.is_abstract_name(BOOL_TYPE_STRING)
        }

        pub fn is_constexpr_bool(&self) -> bool {
            // TODO: Replace CONSTEXPR_BOOL_TYPE_STRING with its actual value
            self.is_abstract_name(CONSTEXPR_BOOL_TYPE_STRING)
        }

        pub fn is_void_or_never(&self) -> bool {
            self.is_void() || self.is_never()
        }

        pub fn is_float32(&self) -> bool {
            // TODO: Replace FLOAT32_TYPE_STRING with its actual value
            self.is_abstract_name(FLOAT32_TYPE_STRING)
        }

        pub fn is_float64(&self) -> bool {
            // TODO: Replace FLOAT64_TYPE_STRING with its actual value
            self.is_abstract_name(FLOAT64_TYPE_STRING)
        }

        pub fn get_generated_type_name(&self) -> String {
            // TODO: Implement this properly
            "Type::GetGeneratedTypeName".to_string()
        }

        pub fn get_generated_tnode_type_name(&self) -> String {
            // TODO: Implement this properly
            "Type::GetGeneratedTNodeTypeName".to_string()
        }

        pub fn is_constexpr(&self) -> bool {
            false // TODO: Implement this properly
        }

        pub fn is_transient(&self) -> bool {
            false // TODO: Implement this properly
        }

        pub fn non_constexpr_version(&self) -> &Type {
            self // TODO: Implement this properly
        }

        pub fn get_constexpr_generated_type_name(&self) -> String {
            // TODO: Implement this properly
            "Type::GetConstexprGeneratedTypeName".to_string()
        }

        pub fn class_supertype(&self) -> Option<&ClassType> {
            None // TODO: Implement this properly
        }

        pub fn struct_supertype(&self) -> Option<&StructType> {
            None // TODO: Implement this properly
        }

        pub fn aggregate_supertype(&self) -> Option<&AggregateType> {
            None // TODO: Implement this properly
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            Vec::new() // TODO: Implement this properly
        }

        pub fn get_runtime_type(&self) -> String {
            // TODO: Implement this properly
            "Type::GetRuntimeType".to_string()
        }

        pub fn get_debug_type(&self) -> String {
            // TODO: Implement this properly
            "Type::GetDebugType".to_string()
        }

        pub fn common_supertype(a: &Type, b: &Type) -> &Type {
            // TODO: Implement this properly
            a
        }

        pub fn add_alias(&mut self, alias: String) {
            self.aliases.insert(alias);
        }

        pub fn id(&self) -> usize {
            self.id
        }

        pub fn get_specialized_from(&self) -> &MaybeSpecializationKey<'static> { // FIXME: Lifetime
            &self.specialized_from
        }

        pub fn match_unary_generic(type_: &Type, generic: &GenericType) -> Option<&Type> {
            // TODO: Implement this properly
            None
        }

        pub fn compute_name(basename: &str, specialized_from: MaybeSpecializationKey) -> String {
            // TODO: Implement this properly
            format!("Type::ComputeName({}, {:?})", basename, specialized_from)
        }

        pub fn set_constexpr_version(&self, type_: &Type) {
            // TODO: Implement this properly
        }

        pub fn constexpr_version(&self) -> Option<&Type> {
            None // TODO: Implement this properly
        }

        pub fn alignment_log2(&self) -> usize {
            0 // TODO: Implement this properly
        }

        fn is_abstract_name(&self, name: &str) -> bool {
            // TODO: Implement this properly
            false
        }

        fn depth(&self) -> i32 {
            // TODO: Implement this properly
            0
        }
    }

    impl PartialEq for Type {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for Type {}

    impl fmt::Debug for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Type")
                .field("kind", &self.kind)
                // .field("parent", &self.parent) // Avoid potential recursion
                .field("aliases", &self.aliases)
                .field("id", &self.id)
                .field("specialized_from", &self.specialized_from)
                // .field("constexpr_version", &self.constexpr_version) // Avoid potential recursion
                .finish()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NameAndType<'a> {
        pub name: String,
        pub type_: &'a Type,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Field<'a> {
        pub pos: SourcePosition,
        pub aggregate: &'a AggregateType,
        pub index: Option<ClassFieldIndexInfo>,
        pub name_and_type: NameAndType<'a>,
        // The byte offset of this field from the beginning of the containing class or
        // struct. Most structs are never packed together in memory, and are only used
        // to hold a batch of related CSA TNode values, in which case |offset| is
        // irrelevant.
        // The offset may be unknown because the field is after an indexed field or
        // because we don't support the struct field for on-heap layouts.
        pub offset: Option<usize>,
        pub custom_weak_marking: bool,
        pub const_qualified: bool,
        pub synchronization: FieldSynchronization,
    }

    impl Field<'_> {
        pub fn get_field_size_information(&self) -> (usize, String) {
            // TODO(danno): This likely should be refactored, the handling of the types
            // using the universal grab-bag utility with std::tie, as well as the
            // reliance of string types is quite clunky.
            (0, String::new()) // TODO: Implement this properly
        }

        pub fn validate_alignment(&self, at_offset: ResidueClass) {
            // TODO: Implement this properly
        }
    }

    impl fmt::Display for Field<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Field {{ name: {}, type: {} }}", self.name_and_type.name, self.name_and_type.type_.to_string())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TopType {
        reason: String,
        source_type: Box<Type>,
    }

    impl TopType {
        #[allow(dead_code)]
        pub fn cast<'a>(declarable: &'a dyn TypeBaseTrait) -> &'a TopType {
            assert!(declarable.is_top_type());
            declarable.as_any().downcast_ref::<TopType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> &'a mut TopType {
            assert!(declarable.is_top_type());
            declarable.as_any_mut().downcast_mut::<TopType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'a>(declarable: &'a dyn TypeBaseTrait) -> Option<&'a TopType> {
            if !declarable.is_top_type() { return None; }
            declarable.as_any().downcast_ref::<TopType>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> Option<&'a mut TopType> {
            if !declarable.is_top_type() { return None; }
            declarable.as_any_mut().downcast_mut::<TopType>()
        }

        pub fn source_type(&self) -> &Type {
            &self.source_type
        }
        pub fn reason(&self) -> String {
            self.reason.clone()
        }
    }

    impl TypeBaseTrait for TopType {
        fn kind(&self) -> Kind {
            Kind::TopType
        }
    }

    impl dyn TypeBaseTrait + '_ {
        #[allow(dead_code)]
        pub fn is_top_type(&self) -> bool {
            self.kind() == Kind::TopType
        }
        #[allow(dead_code)]
        pub fn as_any(&self) -> &dyn Any {
            self
        }
        #[allow(dead_code)]
        pub fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AbstractType {
        flags: AbstractTypeFlags,
        name: String,
        generated_type: String,
        non_constexpr_version: Option<Box<AbstractType>>,
    }

    impl AbstractType {
        #[allow(dead_code)]
        pub fn cast<'a>(declarable: &'a dyn TypeBaseTrait) -> &'a AbstractType {
            assert!(declarable.is_abstract_type());
            declarable.as_any().downcast_ref::<AbstractType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> &'a mut AbstractType {
            assert!(declarable.is_abstract_type());
            declarable.as_any_mut().downcast_mut::<AbstractType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'a>(declarable: &'a dyn TypeBaseTrait) -> Option<&'a AbstractType> {
            if !declarable.is_abstract_type() { return None; }
            declarable.as_any().downcast_ref::<AbstractType>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> Option<&'a mut AbstractType> {
            if !declarable.is_abstract_type() { return None; }
            declarable.as_any_mut().downcast_mut::<AbstractType>()
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn is_constexpr(&self) -> bool {
            self.flags.contains(AbstractTypeFlag::Constexpr)
        }

        pub fn non_constexpr_version(&self) -> Option<&AbstractType> {
            self.non_constexpr_version.as_ref().map(|t| &**t)
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            vec![] // TODO: Implement this properly
        }

        pub fn alignment_log2(&self) -> usize {
            0 // TODO: Implement this properly
        }

        fn use_parent_type_checker(&self) -> bool {
            self.flags.contains(AbstractTypeFlag::UseParentTypeChecker)
        }

        pub fn is_transient(&self) -> bool {
            self.flags.contains(AbstractTypeFlag::Transient)
        }
    }

    impl TypeBaseTrait for AbstractType {
        fn kind(&self) -> Kind {
            Kind::AbstractType
        }
    }

    impl dyn TypeBaseTrait + '_ {
        #[allow(dead_code)]
        pub fn is_abstract_type(&self) -> bool {
            self.kind() == Kind::AbstractType
        }
        #[allow(dead_code)]
        pub fn as_any(&self) -> &dyn Any {
            self
        }
        #[allow(dead_code)]
        pub fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AbstractTypeFlags: u32 {
            const Constexpr = 1 << 0;
            const Transient = 1 << 1;
            const UseParentTypeChecker = 1 << 2;
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct BuiltinPointerType<'a> {
        pub parameter_types: TypeVector<'a>,
        pub return_type: &'a Type,
        pub function_pointer_type_id: usize,
    }

    impl BuiltinPointerType<'_> {
        #[allow(dead_code)]
        pub fn cast<'a>(declarable: &'a dyn TypeBaseTrait) -> &'a BuiltinPointerType<'a> {
            assert!(declarable.is_builtin_pointer_type());
            declarable.as_any().downcast_ref::<BuiltinPointerType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> &'a mut BuiltinPointerType<'a> {
            assert!(declarable.is_builtin_pointer_type());
            declarable.as_any_mut().downcast_mut::<BuiltinPointerType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'a>(declarable: &'a dyn TypeBaseTrait) -> Option<&'a BuiltinPointerType<'a>> {
            if !declarable.is_builtin_pointer_type() { return None; }
            declarable.as_any().downcast_ref::<BuiltinPointerType>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> Option<&'a mut BuiltinPointerType<'a>> {
            if !declarable.is_builtin_pointer_type() { return None; }
            declarable.as_any_mut().downcast_mut::<BuiltinPointerType>()
        }

        pub fn has_context_parameter(&self) -> bool {
            // TODO: Implement this properly
            false
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            vec![TypeChecker { type_: "Smi".to_string(), weak_ref_to: "".to_string() }]
        }
    }

    impl TypeBaseTrait for BuiltinPointerType<'_> {
        fn kind(&self) -> Kind {
            Kind::BuiltinPointerType
        }
    }

    impl dyn TypeBaseTrait + '_ {
        #[allow(dead_code)]
        pub fn is_builtin_pointer_type(&self) -> bool {
            self.kind() == Kind::BuiltinPointerType
        }
        #[allow(dead_code)]
        pub fn as_any(&self) -> &dyn Any {
            self
        }
        #[allow(dead_code)]
        pub fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UnionType<'a> {
        types: HashSet<&'a Type>,
    }

    impl<'a> UnionType<'a> {
        #[allow(dead_code)]
        pub fn cast<'b>(declarable: &'b dyn TypeBaseTrait) -> &'b UnionType<'a> {
            assert!(declarable.is_union_type());
            declarable.as_any().downcast_ref::<UnionType<'a>>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'b>(declarable: &'b mut dyn TypeBaseTrait) -> &'b mut UnionType<'a> {
            assert!(declarable.is_union_type());
            declarable.as_any_mut().downcast_mut::<UnionType<'a>>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'b>(declarable: &'b dyn TypeBaseTrait) -> Option<&'b UnionType<'a>> {
            if !declarable.is_union_type() { return None; }
            declarable.as_any().downcast_ref::<UnionType<'a>>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'b>(declarable: &'b mut dyn TypeBaseTrait) -> Option<&'b mut UnionType<'a>> {
            if !declarable.is_union_type() { return None; }
            declarable.as_any_mut().downcast_mut::<UnionType<'a>>()
        }

        pub fn get_single_member(&self) -> Option<&&Type> {
            if self.types.len() == 1 {
                return self.types.iter().next();
            }
            None
        }

        pub fn is_subtype_of(&self, other: &Type) -> bool {
            for member in &self.types {
                if !member.is_subtype_of(other) {
                    return false;
                }
            }
            true
        }

        pub fn is_supertype_of(&self, other: &Type) -> bool {
            for member in &self.types {
                if other.is_subtype_of(member) {
                    return true;
                }
            }
            false
        }

        pub fn is_transient(&self) -> bool {
            self.types.iter().any(|member| member.is_transient())
        }

        pub fn extend(&mut self, t: &'a Type) {
            // TODO: Implement the rest of this function
            self.types.insert(t);
        }

        pub fn subtract(&mut self, t: &Type) {
            // TODO: Implement this function
        }

        pub fn from_type(t: &'a Type) -> Self {
            // TODO: Implement the rest of this function
            UnionType { types: HashSet::new() }
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            let mut result = Vec::new();
            for member in &self.types {
                let sub_result = member.get_type_checkers();
                result.extend(sub_result);
            }
            result
        }
    }

    impl TypeBaseTrait for UnionType<'_> {
        fn kind(&self) -> Kind {
            Kind::UnionType
        }
    }

    impl dyn TypeBaseTrait + '_ {
        #[allow(dead_code)]
        pub fn is_union_type(&self) -> bool {
            self.kind() == Kind::UnionType
        }
        #[allow(dead_code)]
        pub fn as_any(&self) -> &dyn Any {
            self
        }
        #[allow(dead_code)]
        pub fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BitField {
        pub pos: SourcePosition,
        pub name_and_type: NameAndType<'static>, // FIXME: Lifetime
        pub offset: i32,
        pub num_bits: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BitFieldStructType<'a> {
        namespace_: *mut Namespace, // raw pointer
        decl_: *const BitFieldStructDeclaration, // raw pointer
        fields_: Vec<BitField>,
        phantom: PhantomData<&'a Type>,
    }

    impl<'a> BitFieldStructType<'a> {
        #[allow(dead_code)]
        pub fn cast<'b>(declarable: &'b dyn TypeBaseTrait) -> &'b BitFieldStructType<'a> {
            assert!(declarable.is_bit_field_struct_type());
            declarable.as_any().downcast_ref::<BitFieldStructType<'a>>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'b>(declarable: &'b mut dyn TypeBaseTrait) -> &'b mut BitFieldStructType<'a> {
            assert!(declarable.is_bit_field_struct_type());
            declarable.as_any_mut().downcast_mut::<BitFieldStructType<'a>>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'b>(declarable: &'b dyn TypeBaseTrait) -> Option<&'b BitFieldStructType<'a>> {
            if !declarable.is_bit_field_struct_type() { return None; }
            declarable.as_any().downcast_ref::<BitFieldStructType<'a>>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'b>(declarable: &'b mut dyn TypeBaseTrait) -> Option<&'b mut BitFieldStructType<'a>> {
            if !declarable.is_bit_field_struct_type() { return None; }
            declarable.as_any_mut().downcast_mut::<BitFieldStructType<'a>>()
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            // TODO: Implement this properly
            vec![]
        }

        pub fn name(&self) -> String {
            // TODO: Implement this properly
            "BitFieldStructType::name".to_string()
        }

        pub fn fields(&self) -> &Vec<BitField> {
            &self.fields_
        }

        pub fn lookup_field(&self, name: &str) -> &BitField {
            // TODO: Implement this properly
            &self.fields_[0]
        }

        pub fn get_position(&self) -> SourcePosition {
            // TODO: Implement this properly
            SourcePosition::new()
        }

        pub fn register_field(&mut self, field: BitField) {
            self.fields_.push(field);
        }
    }

    impl<'a> TypeBaseTrait for BitFieldStructType<'a> {
        fn kind(&self) -> Kind {
            Kind::BitFieldStructType
        }
    }

    impl dyn TypeBaseTrait + '_ {
        #[allow(dead_code)]
        pub fn is_bit_field_struct_type(&self) -> bool {
            self.kind() == Kind::BitFieldStructType
        }
        #[allow(dead_code)]
        pub fn as_any(&self) -> &dyn Any {
            self
        }
        #[allow(dead_code)]
        pub fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AggregateType {
        is_finalized_: bool,
        fields_: Vec<Field<'static>>, // FIXME: Lifetime
        namespace_: *mut Namespace, // raw pointer
        name_: String,
        methods_: Vec<*mut Method>, // raw pointer
    }

    impl AggregateType {
        #[allow(dead_code)]
        pub fn cast<'a>(declarable: &'a dyn TypeBaseTrait) -> &'a AggregateType {
            assert!(declarable.is_aggregate_type());
            declarable.as_any().downcast_ref::<AggregateType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> &'a mut AggregateType {
            assert!(declarable.is_aggregate_type());
            declarable.as_any_mut().downcast_mut::<AggregateType>().unwrap()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast<'a>(declarable: &'a dyn TypeBaseTrait) -> Option<&'a AggregateType> {
            if !declarable.is_aggregate_type() { return None; }
            declarable.as_any().downcast_ref::<AggregateType>()
        }
        #[allow(dead_code)]
        pub fn dynamic_cast_mut<'a>(declarable: &'a mut dyn TypeBaseTrait) -> Option<&'a mut AggregateType> {
            if !declarable.is_aggregate_type() { return None; }
            declarable.as_any_mut().downcast_mut::<AggregateType>()
        }

        pub fn finalize(&self) {
            // TODO: Implement this properly
        }

        pub fn set_fields(&mut self, fields: Vec<Field<'static>>) { // FIXME: Lifetime
            self.fields_ = fields;
        }

        pub fn fields(&self) -> &Vec<Field<'static>> { // FIXME: Lifetime
            if !self.is_finalized_ {
                self.finalize();
            }
            &self.fields_
        }

        pub fn has_field(&self, name: &str) -> bool {
            // TODO: Implement this properly
            false
        }

        pub fn lookup_field(&self, name: &str) -> &Field<'static> { // FIXME: Lifetime
            // TODO: Implement this properly
            &self.fields_[0]
        }

        pub fn name(&self) -> &String {
            &self.name_
        }

        pub fn nspace(&self) -> *mut Namespace {
            self.namespace_
        }

        pub fn register_field(&mut self, field: Field<'static>) -> &Field<'static> { // FIXME: Lifetime
            self.fields_.push(field);
            self.fields_.last().unwrap()
        }

        pub fn register_method(&mut self, method: *mut Method) { // raw pointer
            self.methods_.push(method);
        }

        pub fn methods(&self) -> &Vec<*mut Method> { // raw pointer
            if !self.is_finalized_ {
                self.finalize();
            }
            &self.methods_
        }

        pub fn methods_by_name(&self, name: &str) -> Vec<*mut Method> { // raw pointer
            // TODO: Implement this properly
            vec![]
        }

        pub fn get_hierarchy(&self) -> Vec<&AggregateType> {
            // TODO: Implement this properly
            vec![]
        }

        pub fn get_type_checkers(&self) -> Vec<TypeChecker> {
            vec![TypeChecker { type_: self.name_.clone(), weak_ref_to: "".to_string() }]
        }

        pub fn last_field(&self) -> &Field<'static> { // FIXME: Lifetime
            // TODO: Implement this properly
            &self.fields_[0]
        }

        fn check_for_duplicate_fields(&self) {
            // TODO: Implement this properly
        }

        // Use this lookup if you do not want to trigger finalization on this type.
        fn lookup_field_internal(&self, name: &str) -> &Field<'static> { // FIXME: Lifetime
            // TODO: Implement this properly
            &self.fields_[0
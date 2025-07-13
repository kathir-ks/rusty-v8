// Converted from V8 C++ source files:
// Header: types.h
// Implementation: types.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod torque {
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

//use crate::base;
//use crate::zone;
//use crate::ast;
//use crate::constants;
//use crate::source_positions;
//use crate::utils;

pub struct Identifier {
    pub value: String,
}

pub struct BitFieldStructDeclaration {
    pub name: Box<Identifier>,
    pub pos: SourcePosition,
}

pub struct StructDeclaration {
    pub name: Box<Identifier>,
    pub flags: StructFlag,
    pub pos: SourcePosition,
}

#[derive(Default, Copy, Clone)]
pub struct StructFlag(u8);

impl StructFlag {
    pub const K_EXPORT: Self = Self(1 << 0);

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for StructFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for StructFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

pub struct ClassDeclaration {
    pub name: Box<Identifier>,
    pub flags: ClassFlags,
    pub pos: SourcePosition,
    pub instance_type_constraints: InstanceTypeConstraints,
}

#[derive(Default, Copy, Clone)]
pub struct ClassFlags(u32);

impl ClassFlags {
    pub const K_EXTERN: Self = Self(1 << 0);
    pub const K_CPP_OBJECT_DEFINITION: Self = Self(1 << 1);
    pub const K_CPP_OBJECT_LAYOUT_DEFINITION: Self = Self(1 << 2);
    pub const K_GENERATE_BODY_DESCRIPTOR: Self = Self(1 << 3);
    pub const K_DO_NOT_GENERATE_CAST: Self = Self(1 << 4);
    pub const K_TRANSIENT: Self = Self(1 << 5);
    pub const K_ABSTRACT: Self = Self(1 << 6);
    pub const K_GENERATE_CPP_CLASS_DEFINITIONS: Self = Self(1 << 7);
    pub const K_GENERATE_UNIQUE_MAP: Self = Self(1 << 8);
    pub const K_GENERATE_FACTORY_FUNCTION: Self = Self(1 << 9);
    pub const K_EXPORT: Self = Self(1 << 10);
    pub const K_IS_SHAPE: Self = Self(1 << 11);
    pub const K_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT: Self = Self(1 << 12);
    pub const K_LOWEST_INSTANCE_TYPE_WITHIN_PARENT: Self = Self(1 << 13);
    pub const K_UNDEFINED_LAYOUT: Self = Self(1 << 14);
    pub const K_HAS_SAME_INSTANCE_TYPE_AS_PARENT: Self = Self(1 << 15);

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for ClassFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ClassFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

pub struct InstanceTypeConstraints {}

pub struct SourcePosition {}
pub mod base {
    pub fn hash_combine<T: std::hash::Hash>(seed: usize, value: &T) -> usize {
        use std::hash::{Hasher, BuildHasherDefault};
        let mut hasher = BuildHasherDefault::default().build_hasher();
        seed.hash(&mut hasher);
        value.hash(&mut hasher);
        hasher.finish() as usize
    }
}

pub trait Declarable {}
pub struct Zone {}
pub struct StringLiteral {}
pub struct IntegerLiteral {}
pub struct Expression {}

pub struct Scope {}
impl Scope {
    pub fn new() -> Self {
        Scope {}
    }
}
pub mod ast {
    use super::{Declarable, Identifier};
    pub struct TypeDeclaration {}
    impl Declarable for TypeDeclaration {}
    pub struct Function {}
    impl Declarable for Function {}
    pub struct External {}
    impl Declarable for External {}
    pub struct Macro {}
    impl Declarable for Macro {}

    pub enum Node {
        Identifier(Identifier),
    }
}

pub struct QualifiedName {}
pub struct Location {}
pub type VisitResult = i32;
pub type StackRange = i32;
pub struct Label {}
pub struct Int {}
pub struct Float {}

pub struct Macro {
    pub pos: SourcePosition,
}

impl Macro {
    pub fn Position(&self) -> &SourcePosition {
        &self.pos
    }
}

pub mod constants {
    pub enum FieldSynchronization {
        kNone,
        kWeak,
    }
}

pub mod utils {
    use std::fmt;
    pub fn PrintCommaSeparatedList<T: fmt::Display>(
        os: &mut fmt::Formatter,
        list: &std::vec::Vec<T>,
    ) -> fmt::Result {
        for (i, item) in list.iter().enumerate() {
            if i > 0 {
                write!(os, ", ")?;
            }
            write!(os, "{}", item)?;
        }
        Ok(())
    }
}

pub mod torque {
    pub fn ToString<T: std::fmt::Display>(arg: &str, t: T) -> String {
        format!("{} {}", arg, t)
    }
}

pub mod implementation_visitor {
    pub struct CurrentSourcePosition {}
    impl CurrentSourcePosition {
        pub struct Scope {}
        impl Scope {
            pub fn new(_location: i32) -> Self {
                Scope {}
            }
        }
    }
}

pub struct AbstractTypeFlags(u32);

impl AbstractTypeFlags {
    pub const K_CONSTEXPR: Self = Self(1 << 0);
    pub const K_TRANSIENT: Self = Self(1 << 1);
    pub const K_USE_PARENT_TYPE_CHECKER: Self = Self(1 << 2);
}

pub fn StringStartsWith(string: String, prefix: &str) -> bool {
    string.starts_with(prefix)
}
pub fn CamelifyString(s: String) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
            } else {
                result.push(c);
            }
            capitalize_next = false;
        }
    }
    result
}

pub mod turboshaft {
    pub struct Graph {}
    pub struct Block {}
}

pub enum DeoptimizeReason {}

pub mod compilation_flags {
    pub fn Flags() -> i32 {
        0
    }
}
}
use std::any::Any;
use std::fmt;

use self::torque::{
    base::hash_combine, constants::FieldSynchronization, utils::PrintCommaSeparatedList, ClassDeclaration, ClassFlags, CompilationFlags,
    Identifier, StructDeclaration, StructFlag, LabelDeclaration,
    LabelDefinition, Macro, NameAndType, NameVector, ParameterMode,
    ParameterTypes, QualifiedName, Scope, SourceId, SourcePosition,
    StringLiteral, BitFieldStructDeclaration, IntegerLiteral, ImplementationVisitor, Ast, ImplementationVisitor::CurrentSourcePosition
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResidueClass {
    Known(usize),
    Unknown,
}

impl ResidueClass {
    pub fn SingleValue(&self) -> Option<usize> {
        match self {
            ResidueClass::Known(v) => Some(*v),
            _ => None,
        }
    }
}

impl std::ops::Add for ResidueClass {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ResidueClass::Known(a), ResidueClass::Known(b)) => ResidueClass::Known(a + b),
            _ => ResidueClass::Unknown,
        }
    }
}
impl std::fmt::Display for ResidueClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResidueClass::Known(v) => write!(f, "{}", v),
            ResidueClass::Unknown => write!(f, "unknown"),
        }
    }
}
impl ResidueClass {
    pub fn AlignmentLog2(&self) -> usize {
        match self {
            ResidueClass::Known(value) => {
                // Check if the value is a power of 2
                if (value != &0) && ((value & (value - 1)) == 0) {
                    // Calculate log2 using the count_ones method
                    value.trailing_zeros() as usize
                } else {
                    panic!("not align")
                }
            }
            ResidueClass::Unknown => 0,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct Tagged_t {}

pub mod execution {
    pub struct Isolate {}
    impl Isolate {
        pub fn AssertNotOOM(&self) {}
    }
}

pub enum RootIndex {}
pub mod compiler {
    pub struct Common {}
    pub mod turboshaft {
        pub struct Schedule {}
    }
}
pub mod i64 {}
pub struct Map {}
pub mod objects {
    pub struct String {}
}
pub mod wasm {
    pub struct ModuleTypeIndex {}
}

pub mod zone {
    pub struct Zone {}
}
pub mod heap {
    pub enum HeapObject {}
}
pub mod register_configuration {
    pub struct RegisterConfiguration {}
}

pub mod thread_local {
    pub struct ThreadLocal<T> {
        value: T,
    }
    impl<T> ThreadLocal<T> {
        pub fn set(&mut self, value: T) {
            self.value = value;
        }
    }
}
pub mod flags {
    #[derive(Copy, Clone)]
    pub struct Flags<T>(u32);
}

mod internal {
    pub mod torque {
        use super::*;
        pub fn MakeUniqueName(name: String) -> String {
            name
        }
    }
}
pub mod compiler {
    pub mod turboshaft {
        pub struct Graph {}
        pub struct Block {}
    }
}
pub mod turboshaft {
    pub struct Value {}
}

pub mod representation_change {
    pub enum RepresentationChange {}
}
pub mod machine_type {
    pub enum MachineType {}
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArchOpcode {}

pub mod instructions {
    pub enum InstructionCode {}
    pub enum InstructionKind {}
}

pub mod turboshaft {
    pub struct OpIndex {}
}
pub struct InstructionOperand {}

pub mod cc_outputs {
    pub struct cc_output_list {}
}

pub enum Nullability {}

pub struct ValueType {
    kind: ValueTypeKind,
    nullable: bool,
    representation: HeapTypeRepresentation,
}

impl ValueType {
    pub fn new(kind: ValueTypeKind, nullable: bool, representation: HeapTypeRepresentation) -> Self {
        ValueType { kind, nullable, representation }
    }
}

pub enum ValueTypeKind {
    kI32,
}

pub enum HeapTypeRepresentation {
    kNone,
}

pub mod simplified_lowering {
    pub struct Simple {
        pub a: i32,
    }
}

pub struct Script {}

pub mod js_create_lowering {
    pub enum GCType {}
}

pub mod turboshaft {
    pub mod wasm_assembler_helpers {
        pub enum LoadOpKind {}
    }
}

pub mod memory_representation {
    pub enum MemoryRepresentation {}
    pub enum WriteBarrierKind {}
}

pub mod store_representation {
    pub enum StoreRepresentation {}
}

pub mod backend {
    pub mod loong64 {
        pub struct CodeGeneratorLoong64 {}
    }
}

pub mod ast {
    pub enum BinaryOperation {}
}

pub struct V<T>(T);

pub mod compiler {
    pub mod backend {
        pub struct InstructionSequence {}
    }
}
pub mod compiler {
    pub mod backend {
        pub enum ArchOpcode {}
    }
}
pub mod zone {
    pub struct AccountingAllocator {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GenericKind {}

pub mod internal {
    pub mod compiler {
        pub mod backend {
            pub struct CodeGenerator {
                pub a: i32,
            }
        }
    }
}

pub mod memory {
    pub struct MemoryChunk {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpIndex {}
pub struct Node {}
pub struct InstructionOperand {}

pub mod code_reference {
    pub struct CodeReference {}
}

pub mod compilation_flags {
    pub fn Flags() -> i32 {
        0
    }
}
pub mod source_file_map {
    pub fn PathFromV8Root(source: SourceId) -> String {
        String::from("test/")
    }
    pub fn GetSourceId(test_torque_defined_classes_tq: &str) -> SourceId {
        SourceId {}
    }
}
pub mod backend {
    pub struct InstructionSequence {}
}
pub struct DirectHandle<T> {
    t: T,
}
impl<T> DirectHandle<T> {
    pub fn new(t: T) -> Self {
        DirectHandle { t: t }
    }
}

pub struct Type {
    kind: TypeKind,
    parent: Option<Box<Type>>,
    aliases: RefCell<std::collections::HashSet<String>>,
    id: usize,
    specialized_from: Option<SpecializationKey<GenericType>>,
    constexpr_version: RefCell<Option<Box<Type>>>,
}

impl Type {
    fn new(kind: TypeKind, parent: Option<Box<Type>>, specialized_from: Option<SpecializationKey<GenericType>>) -> Self {
        Type {
            kind,
            parent,
            aliases: RefCell::new(HashSet::new()),
            id: Self::fresh_type_id(),
            specialized_from,
            constexpr_version: RefCell::new(None),
        }
    }

    fn fresh_type_id() -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    fn is_top_type(&self) -> bool {
        self.kind == TypeKind::kTopType
    }

    fn is_abstract_type(&self) -> bool {
        self.kind == TypeKind::kAbstractType
    }

    fn is_builtin_pointer_type(&self) -> bool {
        self.kind == TypeKind::kBuiltinPointerType
    }

    fn is_union_type(&self) -> bool {
        self.kind == TypeKind::kUnionType
    }

    fn is_bit_field_struct_type(&self) -> bool {
        self.kind == TypeKind::kBitFieldStructType
    }

    fn is_struct_type(&self) -> bool {
        self.kind == TypeKind::kStructType
    }

    fn is_class_type(&self) -> bool {
        self.kind == TypeKind::kClassType
    }

    fn is_aggregate_type(&self) -> bool {
        self.is_struct_type() || self.is_class_type()
    }

    fn is_void(&self) -> bool {
        self.is_abstract_name("Void".to_string())
    }
    fn is_never(&self) -> bool {
        self.is_abstract_name("Never".to_string())
    }
    fn is_bool(&self) -> bool {
        self.is_abstract_name("Bool".to_string())
    }
    fn is_constexpr_bool(&self) -> bool {
        self.is_abstract_name("ConstexprBool".to_string())
    }
    fn is_void_or_never(&self) -> bool {
        self.is_void() || self.is_never()
    }
    fn is_float32(&self) -> bool {
        self.is_abstract_name("Float32".to_string())
    }
    fn is_float64(&self) -> bool {
        self.is_abstract_name("Float64".to_string())
    }

    pub fn is_constexpr(&self) -> bool {
        match self.parent() {
            Some(parent) => {
                if parent.borrow().is_constexpr() {
                    false
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_abstract_name(&self, name: String) -> bool {
        if let Some(abstract_type) = AbstractType::dynamic_cast(self) {
            abstract_type.name() == name
        } else {
            false
        }
    }
    fn get_generated_type_name(&self) -> String {
        todo!()
    }
    fn const_expr_version(&self) -> Option<Box<Type>> {
        todo!()
    }
    fn get_simple_name(&self) -> String {
        todo!()
    }
    fn add_alias(&self, alias: String) {
        self.aliases.borrow_mut().insert(alias);
    }
    fn id(&self) -> usize {
        self.id
    }

    fn parent(&self) -> Option<std::cell::Ref<'_, Type>> {
        match &self.parent {
            Some(boxed_type) => Some(RefCell::new(Type {
                kind: boxed_type.kind,
                parent: boxed_type.parent.clone(),
                aliases: boxed_type.aliases.clone(),
                id: boxed_type.id,
                specialized_from: boxed_type.specialized_from.clone(),
                constexpr_version: boxed_type.constexpr_version.clone(),
            }).borrow()),
            None => None,
        }
    }
    fn class_supertype(&self) -> Option<&ClassType> {
        let mut t = self;
        while let Some(parent) = &t.parent {
             let t = &parent;
            if let Some(class_type) = ClassType::dynamic_cast(t) {
                return Some(class_type);
            }
        }
        None
    }
    pub fn is_subtype_of(&self, supertype: &Type) -> bool {
        if supertype.is_top_type() {
            return true;
        }
        if self.is_never() {
            return true;
        }
        if let Some(union_type) = UnionType::dynamic_cast(supertype) {
            return union_type.is_supertype_of(self);
        }
        let mut subtype = self;
        while let Some(parent) = &subtype.parent {
             let subtype = &parent;
            if subtype == supertype {
                return true;
            }
        }
        false
    }
    fn specialized_from(&self) -> &Option<SpecializationKey<GenericType>> {
        &self.specialized_from
    }
    fn get_handle_type_name(&self, kind: HandleKind, type_name: &String) -> String {
        match kind {
            HandleKind::Indirect => format!("Handle<{}>", type_name),
            HandleKind::Direct => format!("DirectHandle<{}>", type_name),
        }
    }
    pub fn handlified_cpp_type_name(&self, kind: HandleKind) -> String {
        todo!()
    }
    pub fn get_constexpr_generated_type_name(&self) -> String {
        todo!()
    }
    pub fn match_unary_generic(type_: &Type, generic: *mut GenericType) -> Option<&Type> {
        todo!()
    }
    pub fn simple_name(&self) -> String {
        todo!()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Type {
    fn to_string(&self) -> String {
        if self.aliases.borrow().is_empty() {
            return self.compute_name(self.to_explicit_string(), self.specialized_from.clone());
        }
        let aliases = self.aliases.borrow();
        if aliases.len() == 1 {
            return aliases.iter().next().unwrap().clone();
        }
        let mut result = String::new();
        let mut i = 0;
        for alias in aliases.iter() {
            if i == 0 {
                result.push_str(&format!("{} (aka. ", alias));
            } else if i == 1 {
                result.push_str(alias);
            } else {
                result.push_str(&format!(", {}", alias));
            }
            i += 1;
        }
        result.push_str(")");
        result.clone()
    }

    fn to_explicit_string(&self) -> String {
        match self.kind {
            TypeKind::kTopType => "TopType".to_string(),
            TypeKind::kAbstractType => "AbstractType".to_string(),
            TypeKind::kBuiltinPointerType => "BuiltinPointerType".to_string(),
            TypeKind::kUnionType => "UnionType".to_string(),
            TypeKind::kBitFieldStructType => "BitFieldStructType".to_string(),
            TypeKind::kStructType => "StructType".to_string(),
            TypeKind::kClassType => "ClassType".to_string(),
        }
    }
    fn compute_name(&self, basename: String, specialized_from: Option<SpecializationKey<GenericType>>) -> String {
        match specialized_from {
            Some(key) => format!("{}<{}>", basename, key.specialized_types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")),
            None => basename,
        }
    }
    fn dynamic_cast(type_: &Type) -> Option<&Type> {
         Some(type_)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandleKind {
    Indirect,
    Direct,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeKind {
    kTopType,
    kAbstractType,
    kBuiltinPointerType,
    kUnionType,
    kBitFieldStructType,
    kStructType,
    kClassType,
}

pub struct TopType {
    base: Type,
    reason: String,
    source_type: Box<Type>,
}
impl TopType {
    fn reason(&self) -> String {
        self.reason.clone()
    }
    fn source_type(&self) -> &Type {
        &self.source_type
    }
    fn dynamic_cast(base: &Type) -> Option<&TopType> {
        if base.kind == TypeKind::kTopType {
            unsafe { Some(&*(base as *const Type as *const TopType)) }
        } else {
            None
        }
    }
}

pub struct AbstractType {
    base: Type,
    flags: AbstractTypeFlags,
    name: String,
    generated_type: String,
    non_constexpr_version: Option<Box<Type>>,
}

impl AbstractType {
    pub fn dynamic_cast(base: &Type) -> Option<&AbstractType> {
        if base.kind == TypeKind::kAbstractType {
            unsafe { Some(&*(base as *const Type as *const AbstractType)) }
        } else {
            None
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct BuiltinPointerType {
    base: Type,
    parameter_types: Vec<Box<Type>>,
    return_type: Box<Type>,
    function_pointer_type_id: usize,
}
impl BuiltinPointerType {
    pub fn dynamic_cast(base: &Type) -> Option<&BuiltinPointerType> {
        if base.kind == TypeKind::kBuiltinPointerType {
            unsafe { Some(&*(base as *const Type as *const BuiltinPointerType)) }
        } else {
            None
        }
    }
}

pub struct UnionType {
    base: Type,
    types: Vec<Box<Type>>,
}
impl UnionType {
    pub fn dynamic_cast(base: &Type) -> Option<&UnionType> {
        if base.kind == TypeKind::kUnionType {
            unsafe { Some(&*(base as *const Type as *const UnionType)) }
        } else {
            None
        }
    }
    fn is_supertype_of(&self, other: &Type) -> bool {
        todo!()
    }
}

pub struct BitFieldStructType {
    base: Type,
    namespace_: *mut Namespace,
    decl_: *mut BitFieldStructDeclaration,
    fields_: Vec<BitField>,
}
impl BitFieldStructType {
    pub fn dynamic_cast(base: &Type) -> Option<&BitFieldStructType> {
        if base.kind == TypeKind::kBitFieldStructType {
            unsafe { Some(&*(base as *const Type as *const BitFieldStructType)) }
        } else {
            None
        }
    }
}

pub struct AggregateType {
    base: Type,
    is_finalized: bool,
    fields: Vec<Field>,
    namespace_: *mut Namespace,
    name: String,
    methods: Vec<*mut Macro>,
}

impl AggregateType {
    pub fn dynamic_cast(base: &Type) -> Option<&AggregateType> {
        if base.kind == TypeKind::kStructType || base.kind == TypeKind::kClassType {
            unsafe { Some(&*(base as *const Type as *const AggregateType)) }
        } else {
            None
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn methods(&self) -> Vec<*mut Macro> {
        self.methods.clone()
    }
}

pub struct StructType {
    base: AggregateType,
    decl_: *mut StructDeclaration,
    generated_type_name: String,
}
impl StructType {
    pub fn dynamic_cast(base: &Type) -> Option<&StructType> {
        if base.kind == TypeKind::kStructType {
            unsafe { Some(&*(base as *const Type as *const StructType)) }
        } else {
            None
        }
    }
    pub fn fields(&self) -> Vec<Field> {
        self.base.fields.clone()
    }
}

pub struct ClassType {
    base: AggregateType,
    header_size: usize,
    size: ResidueClass,
    flags_: ClassFlags,
    generates_: String,
    decl_: *mut ClassDeclaration,
    alias_: *mut TypeAlias,
    own_instance_type_: std::option::Option<i32>,
    instance_type_range_: std::option::Option<(i32, i32)>,
}

impl ClassType {
    pub fn dynamic_cast(base: &Type) -> Option<&ClassType> {
        if base.kind == TypeKind::kClassType {
            unsafe { Some(&*(base as *const Type as *const ClassType)) }
        } else {
            None
        }
    }
    pub fn generates_(&self) -> String {
        self.generates_.clone()
    }
}

impl Clone for ClassType {
    fn clone(&self) -> Self {
        ClassType {
            base: AggregateType {
                base: Type {
                    kind: TypeKind::kClassType,
                    parent: None,
                    aliases: RefCell::new(std::collections::HashSet::new()),
                    id: 0,
                    specialized_from: None,
                    constexpr_version: RefCell::new(None),
                },
                is_finalized: self.base.is_finalized,
                fields: self.base.fields.clone(),
                namespace_: self.base.namespace_,
                name: self.base.name.clone(),
                methods: self.base.methods.clone(),
            },
            header_size: self.header_size,
            size: self.size,
            flags_: self.flags_,
            generates_: self.generates_.clone(),
            decl_: self.decl_,
            alias_: self.alias_,
            own_instance_type_: self.own_instance_type_,
            instance_type_range_: self.instance_type_range_,
        }
    }
}

impl fmt::Display for AbstractTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "AbstractTypeFlags(0)")
        } else {
            write!(f, "AbstractTypeFlags(?)")
        }
    }
}

impl AbstractTypeFlags {
    pub fn new() -> Self {
        AbstractTypeFlags(0)
    }
}
pub fn MakeIdentifierExpression(str: &str) -> Box<Expression> {
    Box::new(Expression {})
}

pub fn MakeNode<T>(_arg: T) -> Box<T> {
    Box::new(_arg)
}

pub fn StringLiteralQuote(str: String) -> String {
    str
}
pub fn MakeFieldAccessExpression(str: Box<Expression>, name: String) -> Box<Expression> {
    Box::new(Expression {})
}
pub struct AssignmentExpression {}
pub struct ExpressionStatement {}
pub fn MakeNode_Ex<T>(_arg: T) -> T {
    _arg
}
pub struct PrecomputedTypeExpression {}
pub struct ElementAccessExpression {}
pub struct ReturnStatement {}

pub struct TypeVector {}
impl IntoIterator for TypeVector {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![].into_iter()
    }
}

pub fn MakeCallExpression(str: &str, types: Vec<Box<Expression>>) -> Box<Expression> {
    Box::new(Expression {})
}

pub struct IntegerLiteralExpression {
    literal: IntegerLiteral,
}
pub struct CodeGenerator {}

pub fn MakeConstDeclarationStatement(
    name: &str,
    expression: Box<Expression>,
) -> Box<Statement> {
    Box::new(Statement {})
}

pub struct BlockStatement {
    pub deferred: bool,
    pub statements: std::vec::Vec<Box<Statement>>,
}
impl BlockStatement {
    pub fn new() -> Self {
        BlockStatement {
            deferred: false,
            statements: std::vec::Vec::new(),
        }
    }
}

pub struct Statement {}
pub struct Declarations {}

impl Declarations {
    pub fn DeclareMacro(macro_name: String, b: bool, d: std::option::Option<i32>, l: Signature, block: Box<Statement>, d2: std::option::Option<i32>, b2: bool) -> *mut Macro {
        let m = Macro {
            pos: SourcePosition {},
        };
        Box::into_raw(Box::new(m))
    }
}

pub struct TypeAlias {}
pub struct Namespace {}
impl Namespace {
    pub fn IsDefaultNamespace(&self) -> bool {
        true
    }
}
pub struct GenericType {}

pub struct SpecializationKey<T> {
    generic: *mut T,
    specialized_types: Vec<*const Type>,
}
impl<T> Clone for SpecializationKey<T> {
    fn clone(&self) -> Self {
        SpecializationKey {
            generic: self.generic,
            specialized_types: self.specialized_types.clone(),
        }
    }
}
impl<T> Copy for SpecializationKey<T> {}

pub enum ObjectSlotKind {
    kNoPointer,
    kStrongPointer,
    kMaybeObjectPointer,
    kCustomWeakPointer,
}

impl std::cmp::PartialOrd for ObjectSlotKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for ObjectSlotKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use ObjectSlotKind::*;
        match (self, other) {
            (kNoPointer, _) => std::cmp::Ordering::Less,
            (_, kNoPointer) => std::cmp::Ordering::Greater,
            (kStrongPointer, kStrongPointer) => std::cmp::Ordering::Equal,
            (kMaybeObjectPointer, kMaybeObjectPointer) => std::cmp::Ordering::Equal,
            (kCustomWeakPointer, kCustomWeakPointer) => std::cmp::Ordering::Equal,
            (kStrongPointer, kMaybeObjectPointer) => std::cmp::Ordering::Less,
            (kMaybeObjectPointer, kStrongPointer) => std::cmp::Ordering::Greater,
            (kStrongPointer, kCustomWeakPointer) => std::cmp::Ordering::Less,
            (kCustomWeakPointer, kStrongPointer) => std::cmp::Ordering::Greater,
            (

// Converted from V8 C++ source files:
// Header: torque-parser.h
// Implementation: torque-parser.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ast {
    pub struct EnumDescription {
 pub pos: SourcePosition,
 pub name: String,
 pub constexpr_generates: String,
 pub is_open: bool,
 pub entries: Vec<String>,
 }
}
pub mod utils {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SourcePosition {
        offset: usize,
    }

    impl SourcePosition {
        pub fn from(offset: usize) -> Self {
            SourcePosition { offset }
        }

        pub fn invalid() -> Self {
            SourcePosition { offset: 0 }
        }
    }

    pub struct V8_EXPORT_PRIVATE {}
}

pub mod earley_parser {
    pub struct TypeList {}
    pub struct TypeswitchCase {}
    pub struct EnumEntry {}
    pub struct ParameterList {}
    pub struct AnnotationParameter {}
    pub struct Annotation {}
    pub struct GenericParameter {}
    pub struct GenericParameters {}
    pub struct LabelAndTypes {}
    pub struct NameAndTypeExpression {}
    pub struct ClassFieldExpression {}
    pub struct StructFieldExpression {}
    pub struct BitFieldDeclaration {}
    pub struct IncrementDecrementOperator {}
    pub struct NameAndExpression {}
}

pub mod torque_parser {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fmt;
    use std::fmt::Display;
    use std::optional::Option;

    use crate::ast::EnumDescription;
    use crate::earley_parser::{
        Annotation, AnnotationParameter, BitFieldDeclaration, ClassFieldExpression, EnumEntry,
        GenericParameter, GenericParameters, LabelAndTypes, NameAndExpression,
        NameAndTypeExpression, ParameterList, StructFieldExpression, TypeList, TypeswitchCase,
    };
    use crate::utils::SourcePosition;

    pub struct V8 {}

    pub trait Declaration {}

    pub struct Identifier {
        pub value: String,
        pub pos: SourcePosition,
    }

    pub struct BasicTypeExpression {
        pub namespace_qualification: Vec<String>,
        pub name: Identifier,
        pub generic_arguments: Vec<Box<dyn TypeExpression>>,
    }

    impl TypeExpression for BasicTypeExpression {}

    pub struct FunctionTypeExpression {
        pub parameters: Vec<Box<dyn TypeExpression>>,
        pub return_type: Box<dyn TypeExpression>,
    }

    impl TypeExpression for FunctionTypeExpression {}

    pub struct UnionTypeExpression {
        pub a: Box<dyn TypeExpression>,
        pub b: Box<dyn TypeExpression>,
    }

    impl TypeExpression for UnionTypeExpression {}

    pub trait TypeExpression {}

    pub struct ExpressionStatement {
        pub expression: Box<dyn Expression>,
    }

    pub struct IfStatement {
        pub is_constexpr: bool,
        pub condition: Box<dyn Expression>,
        pub if_true: Box<dyn Statement>,
        pub if_false: Option<Box<dyn Statement>>,
    }

    pub struct WhileStatement {
        pub condition: Box<dyn Expression>,
        pub body: Box<dyn Statement>,
    }

    pub struct ReturnStatement {
        pub value: Option<Box<dyn Expression>>,
    }

    pub struct TailCallStatement {
        pub value: Box<CallExpression>,
    }

    pub struct VarDeclarationStatement {
        pub const_qualified: bool,
        pub name: Identifier,
        pub type_: Option<Box<dyn TypeExpression>>,
        pub initializer: Option<Box<dyn Expression>>,
    }

    pub struct BreakStatement {}

    pub struct ContinueStatement {}

    pub struct GotoStatement {
        pub label: Identifier,
        pub arguments: Vec<Box<dyn Expression>>,
    }

    pub struct BlockStatement {
        pub deferred: bool,
        pub statements: Vec<Box<dyn Statement>>,
    }

    pub struct TryLabelExpression {
        pub try_block: Box<dyn Statement>,
        pub handler: Box<TryHandler>,
    }

    pub struct ForLoopStatement {
        pub var_decl: Option<Box<dyn Statement>>,
        pub test: Option<Box<dyn Expression>>,
        pub action: Option<Box<dyn Statement>>,
        pub body: Box<dyn Statement>,
    }

    pub struct StatementExpression {
        pub statement: Box<dyn Statement>,
    }

    pub struct ClassDeclaration {
        pub name: Identifier,
        pub flags: ClassFlags,
        pub extends: Box<dyn TypeExpression>,
        pub generates: Option<String>,
        pub methods: Vec<Box<dyn Declaration>>,
        pub fields: Vec<ClassFieldExpression>,
        pub instance_type_constraints: InstanceTypeConstraints,
    }

    pub struct StructDeclaration {
        pub flags: StructFlags,
        pub name: Identifier,
        pub methods: Vec<Box<dyn Declaration>>,
        pub fields: Vec<StructFieldExpression>,
    }

    pub struct BitFieldStructDeclaration {
        pub name: Identifier,
        pub extends: Box<dyn TypeExpression>,
        pub fields: Vec<BitFieldDeclaration>,
    }

    pub struct NamespaceDeclaration {
        pub name: String,
        pub declarations: Vec<Box<dyn Declaration>>,
    }

    pub struct SpecializationDeclaration {
        pub transitioning: bool,
        pub name: Identifier,
        pub generic_parameters: Vec<Box<dyn TypeExpression>>,
        pub parameters: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
        pub labels: LabelAndTypesVector,
        pub body: Box<dyn Statement>,
    }

    pub struct CppIncludeDeclaration {
        pub include_path: String,
    }

    pub struct ExternalBuiltinDeclaration {
        pub transitioning: bool,
        pub js_linkage: bool,
        pub name: Identifier,
        pub args: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
    }

    pub struct ExternalRuntimeDeclaration {
        pub transitioning: bool,
        pub name: Identifier,
        pub args: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
    }

    pub struct CallExpression {
        pub callee: Box<IdentifierExpression>,
        pub arguments: Vec<Box<dyn Expression>>,
        pub labels: Vec<Identifier>,
    }

    pub struct CallMethodExpression {
        pub target: Box<dyn Expression>,
        pub callee: Box<IdentifierExpression>,
        pub arguments: Vec<Box<dyn Expression>>,
        pub labels: Vec<Identifier>,
    }

    pub struct IntrinsicCallExpression {
        pub callee: Identifier,
        pub generic_arguments: Vec<Box<dyn TypeExpression>>,
        pub args: Vec<Box<dyn Expression>>,
    }

    pub struct NewExpression {
        pub type_: Box<dyn TypeExpression>,
        pub initializers: Vec<NameAndExpression>,
        pub pretenured: bool,
        pub clear_padding: bool,
    }

    pub struct FieldAccessExpression {
        pub object: Box<dyn Expression>,
        pub field: Identifier,
    }

    pub struct ElementAccessExpression {
        pub object: Box<dyn Expression>,
        pub field: Box<dyn Expression>,
    }

    pub struct DereferenceExpression {
        pub reference: Box<dyn Expression>,
    }

    pub struct StructExpression {
        pub type_: Box<dyn TypeExpression>,
        pub initializers: Vec<NameAndExpression>,
    }

    pub struct AssignmentExpression {
        pub location: Box<dyn Expression>,
        pub op: Option<String>,
        pub value: Box<dyn Expression>,
    }

    pub struct FloatingPointLiteralExpression {
        pub value: f64,
    }

    pub struct IntegerLiteralExpression {
        pub value: IntegerLiteral,
    }

    pub struct StringLiteralExpression {
        pub literal: String,
    }

    pub struct IncrementDecrementExpression {
        pub location: Box<dyn Expression>,
        pub op: IncrementDecrementOperator,
        pub postfix: bool,
    }

    pub struct LogicalOrExpression {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }

    pub struct LogicalAndExpression {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }

    pub struct ConditionalExpression {
        pub condition: Box<dyn Expression>,
        pub if_true: Box<dyn Expression>,
        pub if_false: Box<dyn Expression>,
    }

    pub struct IdentifierExpression {
        pub namespace_qualification: Vec<String>,
        pub name: Identifier,
        pub generic_arguments: Vec<Box<dyn TypeExpression>>,
    }

    pub struct AssumeTypeImpossibleExpression {
        pub accumulated_types: Box<dyn TypeExpression>,
        pub value: Box<dyn Expression>,
    }

    pub struct ConstDeclaration {
        pub name: Identifier,
        pub type_: Box<dyn TypeExpression>,
        pub expression: Box<dyn Expression>,
    }

    pub struct ExternConstDeclaration {
        pub name: Identifier,
        pub type_: Box<dyn TypeExpression>,
        pub literal: String,
    }

    pub struct AbstractTypeDeclaration {
        pub name: Identifier,
        pub flags: AbstractTypeFlags,
        pub extends: Option<Box<dyn TypeExpression>>,
        pub generates: Option<String>,
    }

    pub struct GenericTypeDeclaration {
        pub generic_parameters: GenericParameters,
        pub type_decl: Box<dyn TypeDeclaration>,
    }

    pub struct TorqueMacroDeclaration {
        pub transitioning: bool,
        pub name: Identifier,
        pub operator_name: Option<String>,
        pub args: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
        pub labels: LabelAndTypesVector,
        pub export_to_csa: bool,
        pub body: Option<Box<dyn Statement>>,
    }

    pub struct GenericCallableDeclaration {
        pub generic_parameters: GenericParameters,
        pub declaration: Box<dyn CallableDeclaration>,
    }

    pub struct IntrinsicDeclaration {
        pub name: Identifier,
        pub args: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
    }

    pub struct TorqueBuiltinDeclaration {
        pub transitioning: bool,
        pub javascript_linkage: bool,
        pub name: Identifier,
        pub args: ParameterList,
        pub return_type: Box<dyn TypeExpression>,
        pub has_custom_interface_descriptor: bool,
        pub use_counter_name: Option<String>,
        pub body: Option<Box<dyn Statement>>,
    }

    pub struct DebugStatement {
        pub kind: DebugStatementKind,
    }

    pub struct AssertStatement {
        pub kind: AssertStatementKind,
        pub expression: Box<dyn Expression>,
        pub source: String,
    }

    pub struct SpreadExpression {
        pub spreadee: Box<dyn Expression>,
    }

    pub struct ClassBody {
        pub methods: Vec<Box<dyn Declaration>>,
        pub fields: Vec<ClassFieldExpression>,
    }

    pub struct ImplicitParameters {
        pub kind: Identifier,
        pub parameters: Vec<NameAndTypeExpression>,
    }

    pub struct TryHandler {
        pub handler_kind: TryHandlerKind,
        pub label: Identifier,
        pub parameters: ParameterList,
        pub block: Box<dyn Statement>,
    }

    pub enum TryHandlerKind {
        kLabel,
        kCatch,
    }

    pub enum DebugStatementKind {
        kUnreachable,
        kDebug,
    }

    pub enum AssertStatementKind {
        kDcheck,
        kCheck,
        kSbxCheck,
        kStaticAssert,
    }

    pub struct IntegerLiteral {
        pub negative: bool,
        pub absolute_value: u64,
    }

    pub struct ClassFieldIndexInfo {
        pub index: Box<dyn Expression>,
        pub optional: bool,
    }

    pub type LabelAndTypesVector = Vec<LabelAndTypes>;

    pub struct ConditionalAnnotation {
        pub condition: String,
        pub type_: ConditionalAnnotationType,
    }

    pub enum ConditionalAnnotationType {
        kPositive,
        kNegative,
    }

    pub enum FieldSynchronization {
        kNone,
        kRelaxed,
        kAcquireRelease,
    }

    pub struct InstanceTypeConstraints {
        pub value: i32,
        pub num_flags_bits: i32,
    }

    pub struct ClassFlags(u32);

    impl ClassFlags {
        const kNone: ClassFlags = ClassFlags(0);
        const kAbstract: ClassFlags = ClassFlags(1 << 0);
        const kHasSameInstanceTypeAsParent: ClassFlags = ClassFlags(1 << 1);
        const kDoNotGenerateCppClass: ClassFlags = ClassFlags(1 << 2);
        const kGenerateBodyDescriptor: ClassFlags = ClassFlags(1 << 3);
        const kGenerateUniqueMap: ClassFlags = ClassFlags(1 << 4);
        const kGenerateFactoryFunction: ClassFlags = ClassFlags(1 << 5);
        const kExport: ClassFlags = ClassFlags(1 << 6);
        const kDoNotGenerateCast: ClassFlags = ClassFlags(1 << 7);
        const kHighestInstanceTypeWithinParent: ClassFlags = ClassFlags(1 << 8);
        const kLowestInstanceTypeWithinParent: ClassFlags = ClassFlags(1 << 9);
        const kCppObjectDefinition: ClassFlags = ClassFlags(1 << 10);
        const kCppObjectLayoutDefinition: ClassFlags = ClassFlags(1 << 11);
        const kExtern: ClassFlags = ClassFlags(1 << 12);
        const kTransient: ClassFlags = ClassFlags(1 << 13);
        const kIsShape: ClassFlags = ClassFlags(1 << 14);
        const kUndefinedLayout: ClassFlags = ClassFlags(1 << 15);
        const kGenerateCppClassDefinitions: ClassFlags = ClassFlags(1 << 16);
    }

    impl std::ops::BitOr for ClassFlags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            ClassFlags(self.0 | other.0)
        }
    }

    pub struct StructFlags(u32);

    impl StructFlags {
        const kNone: StructFlags = StructFlags(0);
        const kExport: StructFlags = StructFlags(1 << 0);
    }

    impl std::ops::BitOr for StructFlags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            StructFlags(self.0 | other.0)
        }
    }

    pub struct AbstractTypeFlags(u32);

    impl AbstractTypeFlags {
        const kNone: AbstractTypeFlags = AbstractTypeFlags(0);
        const kTransient: AbstractTypeFlags = AbstractTypeFlags(1 << 0);
        const kConstexpr: AbstractTypeFlags = AbstractTypeFlags(1 << 1);
        const kUseParentTypeChecker: AbstractTypeFlags = AbstractTypeFlags(1 << 2);
    }

    impl std::ops::BitOr for AbstractTypeFlags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            AbstractTypeFlags(self.0 | other.0)
        }
    }

    pub trait TypeDeclaration: Declaration {}
    pub trait CallableDeclaration: Declaration {}
    pub trait Expression {}
    pub trait Statement {}

    pub type ParseResult = Result<ParseResultValue, ParseError>;

    pub enum ParseResultValue {
        StdString(String),
        Bool(bool),
        Int32(i32),
        Double(f64),
        IntegerLiteral(IntegerLiteral),
        StdVectorOfString(Vec<String>),
        DeclarationPtr(Box<dyn Declaration>),
        TypeExpressionPtr(Box<dyn TypeExpression>),
        OptionalTypeExpressionPtr(Option<Box<dyn TypeExpression>>),
        TryHandlerPtr(Box<TryHandler>),
        ExpressionPtr(Box<dyn Expression>),
        IdentifierPtr(Box<Identifier>),
        OptionalIdentifierPtr(Option<Box<Identifier>>),
        StatementPtr(Box<dyn Statement>),
        NameAndTypeExpression(NameAndTypeExpression),
        EnumEntry(EnumEntry),
        StdVectorOfEnumEntry(Vec<EnumEntry>),
        NameAndExpression(NameAndExpression),
        Annotation(Annotation),
        VectorOfAnnotation(Vec<Annotation>),
        AnnotationParameter(AnnotationParameter),
        OptionalAnnotationParameter(Option<AnnotationParameter>),
        ClassFieldExpression(ClassFieldExpression),
        StructFieldExpression(StructFieldExpression),
        BitFieldDeclaration(BitFieldDeclaration),
        StdVectorOfNameAndTypeExpression(Vec<NameAndTypeExpression>),
        ImplicitParameters(ImplicitParameters),
        OptionalImplicitParameters(Option<ImplicitParameters>),
        StdVectorOfNameAndExpression(Vec<NameAndExpression>),
        StdVectorOfClassFieldExpression(Vec<ClassFieldExpression>),
        StdVectorOfStructFieldExpression(Vec<StructFieldExpression>),
        StdVectorOfBitFieldDeclaration(Vec<BitFieldDeclaration>),
        IncrementDecrementOperator(IncrementDecrementOperator),
        OptionalStdString(Option<String>),
        StdVectorOfStatementPtr(Vec<Box<dyn Statement>>),
        StdVectorOfDeclarationPtr(Vec<Box<dyn Declaration>>),
        StdVectorOfStdVectorOfDeclarationPtr(Vec<Vec<Box<dyn Declaration>>>),
        StdVectorOfExpressionPtr(Vec<Box<dyn Expression>>),
        ExpressionWithSource(ExpressionWithSource),
        ParameterList(ParameterList),
        TypeList(TypeList),
        OptionalTypeList(Option<TypeList>),
        LabelAndTypes(LabelAndTypes),
        StdVectorOfLabelAndTypes(Vec<LabelAndTypes>),
        StdVectorOfTryHandlerPtr(Vec<Box<TryHandler>>),
        OptionalStatementPtr(Option<Box<dyn Statement>>),
        OptionalExpressionPtr(Option<Box<dyn Expression>>),
        TypeswitchCase(TypeswitchCase),
        StdVectorOfTypeswitchCase(Vec<TypeswitchCase>),
        StdVectorOfIdentifierPtr(Vec<Box<Identifier>>),
        OptionalClassBody(Option<Box<ClassBody>>),
        GenericParameter(GenericParameter),
        GenericParameters(GenericParameters),
    }

    #[derive(Debug)]
    pub enum ParseError {
        GenericError(String),
    }

    impl Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::GenericError(msg) => write!(f, "ParseError: {}", msg),
            }
        }
    }

    impl Error for ParseError {}

    pub struct ParseResultIterator {}

    impl ParseResultIterator {
        pub fn next_as<T>(&mut self) -> T {
            todo!()
        }
        pub fn matched_input(&mut self) -> InputPosition {
            todo!()
        }
        pub fn has_next(&mut self) -> bool {
            todo!()
        }
    }

    pub struct InputPosition {
        pub pos: SourcePosition,
    }

    pub struct BuildFlags {}

    impl BuildFlags {
        pub fn get_flag(condition: &str, if_str: &str) -> bool {
            todo!()
        }
    }

    pub struct CurrentAst {}

    impl CurrentAst {
        pub fn get() -> CurrentAst {
            todo!()
        }
        pub fn declarations(&mut self) -> &mut Vec<Box<dyn Declaration>> {
            todo!()
        }
        pub fn declare_import_for_current_file(&mut self, import_id: i32) {
            todo!()
        }
        pub fn add_enum_description(&mut self, description: EnumDescription) {
            todo!()
        }
    }

    pub struct ExpressionWithSource {
        pub expression: Box<dyn Expression>,
        pub source: String,
    }

    pub type ImplicitKind = i32;
    pub const CONSTEXPR_TYPE_PREFIX: &str = "constexpr ";
    pub const TORQUE_INTERNAL_NAMESPACE_STRING: &str = "torque_internal";
    pub const CONST_REFERENCE_TYPE_STRING: &str = "ConstReference";
    pub const MUTABLE_REFERENCE_TYPE_STRING: &str = "MutableReference";
    pub const ANNOTATION_EXPORT: &str = "export";
    pub const ANNOTATION_IF: &str = "if";
    pub const ANNOTATION_IFNOT: &str = "ifnot";
    pub const ANNOTATION_USE_PARENT_TYPE_CHECKER: &str = "useParentTypeChecker";
    pub const ANNOTATION_DO_NOT_GENERATE_CPP_CLASS: &str = "doNotGenerateCppClass";
    pub const ANNOTATION_CUSTOM_CPP_CLASS: &str = "customCppClass";
    pub const ANNOTATION_CUSTOM_MAP: &str = "customMap";
    pub const ANNOTATION_GENERATE_BODY_DESCRIPTOR: &str = "generateBodyDescriptor";
    pub const ANNOTATION_DO_NOT_GENERATE_CAST: &str = "doNotGenerateCast";
    pub const ANNOTATION_GENERATE_UNIQUE_MAP: &str = "generateUniqueMap";
    pub const ANNOTATION_GENERATE_FACTORY_FUNCTION: &str = "generateFactoryFunction";
    pub const ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "highestInstanceTypeWithinParent";
    pub const ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "lowestInstanceTypeWithinParent";
    pub const ANNOTATION_CPP_OBJECT_DEFINITION: &str = "cppObjectDefinition";
    pub const ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION: &str = "cppObjectLayoutDefinition";
    pub const ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE: &str =
        "reserveBitsInInstanceType";
    pub const ANNOTATION_INSTANCE_TYPE_VALUE: &str = "instanceTypeValue";
    pub const ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR: &str =
        "customInterfaceDescriptor";
    pub const ANNOTATION_INCREMENT_USE_COUNTER: &str = "incrementUseCounter";
    pub const ANNOTATION_SAME_ENUM_VALUE_AS: &str = "sameEnumValueAs";
    pub const ANNOTATION_CPP_RELAXED_STORE: &str = "cppRelaxedStore";
    pub const ANNOTATION_CPP_RELAXED_LOAD: &str = "cppRelaxedLoad";
    pub const ANNOTATION_CPP_RELEASE_STORE: &str = "cppReleaseStore";
    pub const ANNOTATION_CPP_ACQUIRE_LOAD: &str = "cppAcquireLoad";
    pub const ANNOTATION_CUSTOM_WEAK_MARKING: &str = "customWeakMarking";
    pub const kCatchLabelName: &str = "__catch";
    pub const kNextCaseLabelName: &str = "__next_case";

    pub struct SourceFileMap {}

    impl SourceFileMap {
        pub fn file_relative_to_v8_root_exists(import_path: &String) -> bool {
            todo!()
        }
        pub fn get_source_id(import_path: String) -> i32 {
            todo!()
        }
    }

    pub struct CurrentSourcePosition {}

    impl CurrentSourcePosition {
        pub fn get() -> SourcePosition {
            todo!()
        }
    }

    fn string_literal_unquote(input: String) -> String {
        todo!()
    }

    fn is_upper_camel_case(s: &str) -> bool {
        todo!()
    }

    fn is_lower_camel_case(s: &str) -> bool {
        todo!()
    }

    fn is_valid_namespace_const_name(name: &str) -> bool {
        todo!()
    }

    fn is_valid_type_name(name: &str) -> bool {
        todo!()
    }

    fn is_snake_case(name: &String) -> bool {
        todo!()
    }

    fn unwrap_t_node_type_name(generates: String) -> Option<String> {
        todo!()
    }

    fn get_constexpr_name(name: String) -> String {
        todo!()
    }

    fn process_if_annotation(annotations: &AnnotationSet) -> bool {
        if let Some(condition) = annotations.get_string_param(ANNOTATION_IF) {
            if !BuildFlags::get_flag(&condition, ANNOTATION_IF) {
                return false;
            }
        }
        if let Some(condition) = annotations.get_string_param(ANNOTATION_IFNOT) {
            if BuildFlags::get_flag(&condition, ANNOTATION_IFNOT) {
                return false;
            }
        }
        true
    }

    struct BuildFlagsScope {}

    impl BuildFlagsScope {
    }

    pub fn parse_torque(input: String) {
        BuildFlagsScope {};
        TorqueGrammar {}.parse(input);
    }

    struct TorqueGrammar {}

    impl TorqueGrammar {
        fn parse(&self, input: String) {
            todo!()
        }
    }

    #[derive(Debug, Default)]
    pub struct AnnotationSet {
        set_: HashSet<String>,
        map_: HashMap<String, (AnnotationParameter, SourcePosition)>,
    }

    impl AnnotationSet {
        fn new(
            iter: &mut ParseResultIterator,
            allowed_without_param: HashSet<String>,
            allowed_with_param: HashSet<String>,
        ) -> Self {
            todo!()
        }

        fn contains(&self, s: &str) -> bool {
            self.set_.contains(s)
        }

        fn get_string_param(&self, s: &str) -> Option<String> {
            self.map_.get(s).map(|(ap, _)| {
                if ap.is_int {
                    todo!()
                } else {
                    ap.string_value.clone()
                }
            })
        }

        fn get_int_param(&self, s: &str) -> Option<i32> {
            self.map_.get(s).map(|(ap, _)| {
                if !ap.is_int {
                    todo!()
                } else {
                    ap.int_value
                }
            })
        }
    }
}

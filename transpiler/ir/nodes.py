"""Intermediate representation (IR) node types for the C++ to Rust transpiler.

These dataclasses form the bridge between C++ AST extraction and Rust code generation.
Every C++ construct is first converted to an IR node, then the IR is mapped/transformed,
and finally emitted as Rust source code.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any, Dict, List, Optional, Union


# ─── Enums ───────────────────────────────────────────────────────────────────

class AccessSpecifier(Enum):
    PUBLIC = "pub"
    PROTECTED = "pub(crate)"
    PRIVATE = ""


class TypeKind(Enum):
    """What kind of type this is at the C++ level."""
    PRIMITIVE = auto()
    CLASS = auto()
    STRUCT = auto()
    ENUM = auto()
    TYPEDEF = auto()
    TEMPLATE_PARAM = auto()
    POINTER = auto()
    REFERENCE = auto()
    ARRAY = auto()
    FUNCTION_POINTER = auto()
    VOID = auto()
    AUTO = auto()
    UNKNOWN = auto()


class CastKind(Enum):
    AS = "as"                        # static_cast of primitives
    INTO = "into"                    # static_cast with conversion
    FROM = "from"                    # constructor-style
    UNSAFE_TRANSMUTE = "transmute"   # reinterpret_cast
    UNSAFE_PTR_CAST = "ptr_cast"     # pointer casts


class UnaryOp(Enum):
    NEG = "-"
    NOT = "!"
    BITNOT = "!"      # C++ ~ maps to ! in Rust for integers
    DEREF = "*"
    ADDR_OF = "&"
    ADDR_OF_MUT = "&mut"
    PRE_INC = "++"
    PRE_DEC = "--"
    POST_INC = "post++"
    POST_DEC = "post--"


class BinaryOp(Enum):
    ADD = "+"
    SUB = "-"
    MUL = "*"
    DIV = "/"
    MOD = "%"
    AND = "&&"
    OR = "||"
    BIT_AND = "&"
    BIT_OR = "|"
    BIT_XOR = "^"
    SHL = "<<"
    SHR = ">>"
    EQ = "=="
    NE = "!="
    LT = "<"
    LE = "<="
    GT = ">"
    GE = ">="
    ASSIGN = "="
    ADD_ASSIGN = "+="
    SUB_ASSIGN = "-="
    MUL_ASSIGN = "*="
    DIV_ASSIGN = "/="
    MOD_ASSIGN = "%="
    BIT_AND_ASSIGN = "&="
    BIT_OR_ASSIGN = "|="
    BIT_XOR_ASSIGN = "^="
    SHL_ASSIGN = "<<="
    SHR_ASSIGN = ">>="
    COMMA = ","


class LiteralKind(Enum):
    INT = "int"
    FLOAT = "float"
    STRING = "string"
    CHAR = "char"
    BOOL = "bool"
    NULL = "null"


# ─── Type IR ─────────────────────────────────────────────────────────────────

@dataclass
class IRType:
    """Represents a type in the IR (can be C++ original or mapped Rust type)."""
    name: str                                  # Type name (e.g., "Vec<i32>", "Handle<T>")
    cpp_name: str = ""                         # Original C++ type name
    kind: TypeKind = TypeKind.UNKNOWN
    is_const: bool = False
    is_reference: bool = False                 # Rust &T
    is_mut_reference: bool = False             # Rust &mut T
    is_pointer: bool = False                   # Raw *const T / *mut T
    is_mut_pointer: bool = False               # *mut T (vs *const T)
    is_optional: bool = False                  # Option<T> (nullable pointer)
    generic_args: List[IRType] = field(default_factory=list)
    array_size: Optional[int] = None           # For fixed-size arrays [T; N]
    lifetime: Optional[str] = None             # 'a, 'static, etc.

    def __str__(self) -> str:
        return self.name


# ─── Template / Generic Parameters ──────────────────────────────────────────

@dataclass
class IRTemplateParam:
    """A template/generic parameter."""
    name: str                                  # e.g., "T"
    is_type_param: bool = True                 # typename T vs int N
    constraint: Optional[str] = None           # Trait bound (e.g., "Clone + Debug")
    default: Optional[IRType] = None           # Default type/value
    # For non-type params (const generics):
    value_type: Optional[IRType] = None        # Type of non-type param (e.g., usize)


# ─── Field / Parameter ──────────────────────────────────────────────────────

@dataclass
class IRField:
    """A struct/class field."""
    name: str
    type: IRType
    access: AccessSpecifier = AccessSpecifier.PUBLIC
    is_static: bool = False
    is_const: bool = False
    default_value: Optional[str] = None        # Literal default value
    cpp_name: str = ""                         # Original C++ name
    comment: str = ""


@dataclass
class IRParam:
    """A function parameter."""
    name: str
    type: IRType
    default_value: Optional[str] = None
    cpp_name: str = ""


# ─── Enum ────────────────────────────────────────────────────────────────────

@dataclass
class IREnumVariant:
    """A single enum variant."""
    name: str
    value: Optional[str] = None                # Explicit discriminant value
    cpp_name: str = ""


@dataclass
class IREnum:
    """An enum definition (C++ enum/enum class → Rust enum)."""
    name: str
    qualified_name: str = ""
    variants: List[IREnumVariant] = field(default_factory=list)
    underlying_type: Optional[IRType] = None   # e.g., i32 for `enum class Foo : int`
    is_scoped: bool = True                     # enum class (true) vs enum (false)
    visibility: AccessSpecifier = AccessSpecifier.PUBLIC
    derives: List[str] = field(default_factory=lambda: [
        "Debug", "Clone", "Copy", "PartialEq", "Eq",
    ])
    cpp_name: str = ""
    source_file: str = ""
    source_line: int = 0
    comment: str = ""


# ─── Expression IR Nodes ─────────────────────────────────────────────────────

@dataclass
class IRLiteral:
    """A literal value."""
    value: str
    kind: LiteralKind = LiteralKind.INT


@dataclass
class IRNameRef:
    """Reference to a named entity (variable, function, type)."""
    name: str
    qualified_name: str = ""


@dataclass
class IRBinaryExpr:
    """Binary operator expression: left op right."""
    op: BinaryOp
    left: IRExpr = None
    right: IRExpr = None


@dataclass
class IRUnaryExpr:
    """Unary operator expression: op operand."""
    op: UnaryOp
    operand: IRExpr = None


@dataclass
class IRCallExpr:
    """Function/method call expression."""
    function: IRExpr = None                    # What's being called
    args: List[IRExpr] = field(default_factory=list)
    is_method_call: bool = False
    generic_args: List[IRType] = field(default_factory=list)  # Turbofish ::<T>


@dataclass
class IRMemberExpr:
    """Field/method access: obj.member or obj->member."""
    object: IRExpr = None
    member: str = ""
    is_arrow: bool = False                     # -> in C++, always . in Rust


@dataclass
class IRIndexExpr:
    """Array/subscript access: obj[index]."""
    object: IRExpr = None
    index: IRExpr = None


@dataclass
class IRCastExpr:
    """Type cast expression."""
    expr: IRExpr = None
    target_type: Optional[IRType] = None
    kind: CastKind = CastKind.AS


@dataclass
class IRTernaryExpr:
    """C++ ternary a ? b : c → Rust if a { b } else { c }."""
    condition: IRExpr = None
    then_expr: IRExpr = None
    else_expr: IRExpr = None


@dataclass
class IRNewExpr:
    """C++ new expression → Box::new(T::new(args))."""
    type: Optional[IRType] = None
    args: List[IRExpr] = field(default_factory=list)
    is_array: bool = False                     # new T[] → Vec<T>
    array_size: Optional[IRExpr] = None


@dataclass
class IRDeleteExpr:
    """C++ delete expression → drop()."""
    expr: IRExpr = None
    is_array: bool = False


@dataclass
class IRSizeofExpr:
    """sizeof / alignof expression."""
    target_type: Optional[IRType] = None
    target_expr: Optional[IRExpr] = None       # sizeof(expr)
    is_alignof: bool = False


@dataclass
class IRLambdaExpr:
    """C++ lambda → Rust closure."""
    params: List[IRParam] = field(default_factory=list)
    body: Optional[IRBlock] = None
    capture_mode: str = ""                     # "move", "ref", or ""
    return_type: Optional[IRType] = None


@dataclass
class IRThisExpr:
    """C++ `this` → Rust `self`."""
    is_mut: bool = False


@dataclass
class IRInitListExpr:
    """Aggregate initialization {a, b, c}."""
    elements: List[IRExpr] = field(default_factory=list)


@dataclass
class IRRawExpr:
    """Fallback: unparsed C++ expression as raw source text."""
    cpp_source: str = ""
    comment: str = "TODO: manually translate"


# Union type for all expressions
IRExpr = Union[
    IRLiteral, IRNameRef, IRBinaryExpr, IRUnaryExpr, IRCallExpr,
    IRMemberExpr, IRIndexExpr, IRCastExpr, IRTernaryExpr,
    IRNewExpr, IRDeleteExpr, IRSizeofExpr, IRLambdaExpr,
    IRThisExpr, IRInitListExpr, IRRawExpr,
]


# ─── Statement IR Nodes ─────────────────────────────────────────────────────

@dataclass
class IRBlock:
    """A block of statements: { ... }."""
    statements: List[IRStmt] = field(default_factory=list)


@dataclass
class IRVarDecl:
    """Variable declaration: let [mut] name: Type = init;"""
    name: str
    type: Optional[IRType] = None
    initializer: Optional[IRExpr] = None
    is_mutable: bool = True                    # C++ vars are mutable by default
    is_static: bool = False
    cpp_name: str = ""


@dataclass
class IRExprStmt:
    """Expression as a statement."""
    expr: Optional[IRExpr] = None


@dataclass
class IRReturnStmt:
    """return [value];"""
    value: Optional[IRExpr] = None


@dataclass
class IRIfStmt:
    """if condition { then } [else { else }]."""
    condition: Optional[IRExpr] = None
    then_block: Optional[IRBlock] = None
    else_block: Optional[IRBlock] = None       # Can be another IRIfStmt for else-if


@dataclass
class IRMatchStmt:
    """match expr { arms } (from C++ switch)."""
    expr: Optional[IRExpr] = None
    arms: List[IRMatchArm] = field(default_factory=list)


@dataclass
class IRMatchArm:
    """A single match arm: pattern => block."""
    patterns: List[str] = field(default_factory=list)  # Pattern strings
    body: Optional[IRBlock] = None
    is_default: bool = False                   # _ => ...


@dataclass
class IRForLoop:
    """C-style for loop: for (init; cond; incr) { body }."""
    init: Optional[IRStmt] = None
    condition: Optional[IRExpr] = None
    increment: Optional[IRExpr] = None
    body: Optional[IRBlock] = None


@dataclass
class IRForRangeLoop:
    """Range-based for: for item in iterable { body }."""
    variable: str = ""
    var_type: Optional[IRType] = None
    iterable: Optional[IRExpr] = None
    body: Optional[IRBlock] = None
    is_ref: bool = False                       # for (auto& x : ...) → for x in &...
    is_mut_ref: bool = False


@dataclass
class IRWhileLoop:
    """while condition { body }."""
    condition: Optional[IRExpr] = None
    body: Optional[IRBlock] = None
    is_do_while: bool = False                  # do-while → loop { body; if !cond break }


@dataclass
class IRBreakStmt:
    """break;"""
    pass


@dataclass
class IRContinueStmt:
    """continue;"""
    pass


@dataclass
class IRThrowStmt:
    """C++ throw → return Err(...)."""
    expr: Optional[IRExpr] = None


@dataclass
class IRTryCatchStmt:
    """C++ try/catch → Rust match on Result."""
    try_block: Optional[IRBlock] = None
    catch_blocks: List[IRCatchBlock] = field(default_factory=list)


@dataclass
class IRCatchBlock:
    """A single catch handler."""
    exception_type: Optional[IRType] = None
    variable_name: str = ""
    body: Optional[IRBlock] = None


@dataclass
class IRUnsafeBlock:
    """unsafe { ... }."""
    body: Optional[IRBlock] = None


@dataclass
class IRRawStmt:
    """Fallback: unparsed C++ statement as raw source text."""
    cpp_source: str = ""
    comment: str = "TODO: manually translate"


# Union type for all statements
IRStmt = Union[
    IRBlock, IRVarDecl, IRExprStmt, IRReturnStmt,
    IRIfStmt, IRMatchStmt,
    IRForLoop, IRForRangeLoop, IRWhileLoop,
    IRBreakStmt, IRContinueStmt,
    IRThrowStmt, IRTryCatchStmt, IRUnsafeBlock,
    IRRawStmt,
]


# ─── Function IR ─────────────────────────────────────────────────────────────

@dataclass
class IRFunction:
    """A function or method definition."""
    name: str
    qualified_name: str = ""
    params: List[IRParam] = field(default_factory=list)
    return_type: Optional[IRType] = None
    body: Optional[IRBlock] = None             # None = declaration only

    # Method-specific
    is_method: bool = False
    self_param: str = ""                       # "&self", "&mut self", "self", ""
    is_static: bool = False

    # Qualifiers
    is_virtual: bool = False
    is_pure_virtual: bool = False
    is_override: bool = False
    is_const: bool = False                     # const method → &self (not &mut self)
    is_inline: bool = False
    is_constexpr: bool = False
    is_unsafe: bool = False                    # Needs unsafe in Rust

    # Generic parameters (from templates)
    template_params: List[IRTemplateParam] = field(default_factory=list)

    # Visibility
    access: AccessSpecifier = AccessSpecifier.PUBLIC

    # Metadata
    cpp_name: str = ""
    source_file: str = ""
    source_line: int = 0
    comment: str = ""

    # Attributes
    attributes: List[str] = field(default_factory=list)  # #[inline], #[must_use], etc.


# ─── Struct / Class IR ───────────────────────────────────────────────────────

@dataclass
class IRBaseClass:
    """A base class in an inheritance relationship."""
    name: str
    qualified_name: str = ""
    access: AccessSpecifier = AccessSpecifier.PUBLIC
    is_virtual: bool = False                   # virtual inheritance


@dataclass
class IRStruct:
    """A struct/class definition (C++ class → Rust struct + impl + traits)."""
    name: str
    qualified_name: str = ""
    fields: List[IRField] = field(default_factory=list)
    methods: List[IRFunction] = field(default_factory=list)
    base_classes: List[IRBaseClass] = field(default_factory=list)
    nested_types: List[Union[IRStruct, IREnum, IRTypeAlias]] = field(default_factory=list)

    # Template/generic
    template_params: List[IRTemplateParam] = field(default_factory=list)

    # Classification
    is_abstract: bool = False                  # Has pure virtual methods
    is_class: bool = True                      # class vs struct in C++

    # Visibility
    visibility: AccessSpecifier = AccessSpecifier.PUBLIC
    derives: List[str] = field(default_factory=list)

    # Metadata
    cpp_name: str = ""
    source_file: str = ""
    source_line: int = 0
    comment: str = ""


# ─── Trait IR ────────────────────────────────────────────────────────────────

@dataclass
class IRTrait:
    """A trait definition (from C++ abstract class or interface)."""
    name: str
    qualified_name: str = ""
    methods: List[IRFunction] = field(default_factory=list)
    supertraits: List[str] = field(default_factory=list)
    template_params: List[IRTemplateParam] = field(default_factory=list)
    visibility: AccessSpecifier = AccessSpecifier.PUBLIC
    comment: str = ""


# ─── Impl Block IR ──────────────────────────────────────────────────────────

@dataclass
class IRImplBlock:
    """An impl block (inherent or trait impl)."""
    struct_name: str                           # What type we're implementing for
    trait_name: Optional[str] = None           # None = inherent impl, str = trait impl
    methods: List[IRFunction] = field(default_factory=list)
    template_params: List[IRTemplateParam] = field(default_factory=list)


# ─── Type Alias IR ───────────────────────────────────────────────────────────

@dataclass
class IRTypeAlias:
    """A type alias (C++ typedef/using → Rust type alias)."""
    name: str
    qualified_name: str = ""
    target: Optional[IRType] = None
    template_params: List[IRTemplateParam] = field(default_factory=list)
    visibility: AccessSpecifier = AccessSpecifier.PUBLIC
    cpp_name: str = ""
    source_file: str = ""
    source_line: int = 0


# ─── Constant / Static IR ───────────────────────────────────────────────────

@dataclass
class IRConst:
    """A const or static variable."""
    name: str
    qualified_name: str = ""
    type: Optional[IRType] = None
    value: Optional[str] = None
    is_static: bool = False                    # static vs const
    is_mutable: bool = False                   # static mut (rare, unsafe)
    visibility: AccessSpecifier = AccessSpecifier.PUBLIC
    cpp_name: str = ""
    source_file: str = ""
    source_line: int = 0


# ─── Use / Import IR ────────────────────────────────────────────────────────

@dataclass
class IRUseDecl:
    """A use declaration (from C++ #include → Rust use)."""
    path: str                                  # e.g., "v8_base::bits"
    alias: Optional[str] = None                # use ... as Alias
    is_glob: bool = False                      # use module::*


# ─── Macro IR ────────────────────────────────────────────────────────────────

@dataclass
class IRMacro:
    """A macro definition (C++ #define → Rust macro_rules! or const)."""
    name: str
    params: List[str] = field(default_factory=list)  # Empty = object-like macro
    body: str = ""                             # Rust macro body or const value
    is_function_like: bool = False
    is_const: bool = False                     # Simple value macro → const
    type: Optional[IRType] = None              # For const macros


# ─── Top-level item union ────────────────────────────────────────────────────

IRItem = Union[
    IRStruct, IREnum, IRTrait, IRImplBlock, IRTypeAlias,
    IRConst, IRFunction, IRUseDecl, IRMacro,
]


# ─── File and Module IR ─────────────────────────────────────────────────────

@dataclass
class IRFile:
    """A single Rust source file."""
    path: str                                  # Relative path in output (e.g., "src/bits.rs")
    cpp_source_files: List[str] = field(default_factory=list)  # Original C++ files
    uses: List[IRUseDecl] = field(default_factory=list)
    items: List[IRItem] = field(default_factory=list)
    module_doc: str = ""                       # //! module doc comment
    cfg_attrs: List[str] = field(default_factory=list)  # #![cfg(...)]


@dataclass
class IRModule:
    """A Rust crate (one per V8 module)."""
    name: str                                  # V8 module name (e.g., "base")
    crate_name: str                            # Rust crate name (e.g., "v8-base")
    files: List[IRFile] = field(default_factory=list)
    dependencies: List[str] = field(default_factory=list)  # Crate names
    features: List[str] = field(default_factory=list)
    external_deps: Dict[str, str] = field(default_factory=dict)  # name → version


# ─── Serialization helpers ───────────────────────────────────────────────────

def ir_to_dict(node) -> Any:
    """Recursively convert an IR node to a JSON-serializable dict."""
    if node is None:
        return None
    if isinstance(node, (str, int, float, bool)):
        return node
    if isinstance(node, Enum):
        return node.value
    if isinstance(node, list):
        return [ir_to_dict(item) for item in node]
    if isinstance(node, dict):
        return {k: ir_to_dict(v) for k, v in node.items()}
    if hasattr(node, "__dataclass_fields__"):
        result = {"_type": type(node).__name__}
        for fname in node.__dataclass_fields__:
            result[fname] = ir_to_dict(getattr(node, fname))
        return result
    return str(node)


def ir_to_json(node, indent: int = 2) -> str:
    """Serialize an IR node to JSON string."""
    return json.dumps(ir_to_dict(node), indent=indent)


# ─── Deserialization ────────────────────────────────────────────────────────

# Registry of all IR dataclass types by name (populated at module load time).
_IR_TYPE_REGISTRY: Dict[str, type] = {}


def _build_registry():
    """Auto-populate _IR_TYPE_REGISTRY from this module's globals."""
    import inspect
    for name, obj in globals().items():
        if (inspect.isclass(obj)
                and hasattr(obj, "__dataclass_fields__")
                and name.startswith("IR")):
            _IR_TYPE_REGISTRY[name] = obj


# Enum value→member reverse maps (populated lazily).
_ENUM_REVERSE: Dict[type, Dict[Any, Any]] = {}

_IR_ENUM_TYPES = {
    "AccessSpecifier": AccessSpecifier,
    "TypeKind": TypeKind,
    "CastKind": CastKind,
    "UnaryOp": UnaryOp,
    "BinaryOp": BinaryOp,
    "LiteralKind": LiteralKind,
}


def _enum_from_value(enum_cls, value):
    """Convert a serialized enum value back to the Enum member."""
    if enum_cls not in _ENUM_REVERSE:
        _ENUM_REVERSE[enum_cls] = {m.value: m for m in enum_cls}
    return _ENUM_REVERSE[enum_cls].get(value, value)


def ir_from_dict(data) -> Any:
    """Reconstruct an IR node tree from the dict produced by ir_to_dict.

    Handles nested IR nodes, enum values, and plain types transparently.
    Returns the original Python value (str, int, None, etc.) for non-IR data.
    """
    if not _IR_TYPE_REGISTRY:
        _build_registry()

    if data is None:
        return None
    if isinstance(data, (str, int, float, bool)):
        return data
    if isinstance(data, list):
        return [ir_from_dict(item) for item in data]
    if not isinstance(data, dict):
        return data

    type_name = data.get("_type")
    if type_name is None:
        # Plain dict (not an IR node).
        return {k: ir_from_dict(v) for k, v in data.items()}

    cls = _IR_TYPE_REGISTRY.get(type_name)
    if cls is None:
        return data  # Unknown type — return as-is

    import dataclasses
    fields = {f.name: f for f in dataclasses.fields(cls)}
    kwargs: Dict[str, Any] = {}
    for fname, fld in fields.items():
        raw = data.get(fname, dataclasses.MISSING)
        if raw is dataclasses.MISSING:
            continue  # will use the dataclass default
        kwargs[fname] = _coerce_field(fld, raw)

    return cls(**kwargs)


def _coerce_field(fld, raw):
    """Convert a raw deserialized value to the correct Python type for *fld*."""
    import dataclasses
    # Retrieve the type annotation as a string.
    type_str = str(fld.type) if not isinstance(fld.type, str) else fld.type

    # Check if the field is an Enum type we know about.
    for enum_name, enum_cls in _IR_ENUM_TYPES.items():
        if enum_name in type_str and isinstance(raw, (str, int)):
            result = _enum_from_value(enum_cls, raw)
            if result != raw:
                return result
            break

    # Recurse for nested IR nodes / lists.
    return ir_from_dict(raw)

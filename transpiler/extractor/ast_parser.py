"""Core Clang AST parser using Python libclang bindings.

Parses C++ translation units and extracts structured information about
all declarations: classes, structs, enums, functions, typedefs, templates,
namespaces, and variables.
"""

from __future__ import annotations

import os
import sys
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple

from clang.cindex import (
    Index,
    TranslationUnit,
    Cursor,
    CursorKind,
    TypeKind as ClangTypeKind,
    AccessSpecifier as ClangAccess,
    TokenKind,
)

# Add parent dir to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent))
from config import TranspilerConfig


# ─── Extracted Data Structures ───────────────────────────────────────────────
# These are plain dicts/dataclasses that serialize to JSON.
# They are NOT the IR nodes — they are the raw extraction output
# that will later be converted to IR.

@dataclass
class ExtractedParam:
    name: str
    type: str                    # Type spelling from Clang
    canonical_type: str = ""     # Canonical (resolved) type
    default_value: str = ""
    is_const: bool = False

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedField:
    name: str
    type: str
    canonical_type: str = ""
    access: str = "public"       # public, protected, private
    is_static: bool = False
    is_const: bool = False
    is_mutable: bool = False     # C++ mutable keyword
    offset: int = -1             # Bit offset in struct

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedEnumVariant:
    name: str
    value: Optional[int] = None
    value_str: str = ""

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedEnum:
    name: str
    qualified_name: str = ""
    variants: List[ExtractedEnumVariant] = field(default_factory=list)
    underlying_type: str = ""
    is_scoped: bool = False      # enum class
    access: str = "public"
    file: str = ""
    line: int = 0

    def to_dict(self) -> dict:
        d = asdict(self)
        d["variants"] = [v.to_dict() for v in self.variants]
        return d


@dataclass
class ExtractedFunction:
    name: str
    qualified_name: str = ""
    return_type: str = ""
    canonical_return_type: str = ""
    params: List[ExtractedParam] = field(default_factory=list)
    access: str = "public"
    is_virtual: bool = False
    is_pure_virtual: bool = False
    is_override: bool = False
    is_static: bool = False
    is_const: bool = False       # const method
    is_inline: bool = False
    is_constexpr: bool = False
    is_deleted: bool = False
    is_defaulted: bool = False
    is_constructor: bool = False
    is_destructor: bool = False
    is_copy_constructor: bool = False
    is_move_constructor: bool = False
    is_operator: bool = False
    operator_name: str = ""      # e.g., "operator+", "operator=="
    template_params: List[ExtractedTemplateParam] = field(default_factory=list)
    has_body: bool = False
    body_source: str = ""        # Raw C++ source of the body
    body_start_line: int = 0
    body_end_line: int = 0
    body_ir: Any = None          # Serialized IR body dict (from StmtConverter)
    file: str = ""
    line: int = 0

    def to_dict(self) -> dict:
        d = asdict(self)
        d["params"] = [p.to_dict() for p in self.params]
        d["template_params"] = [t.to_dict() for t in self.template_params]
        # body_ir is already a dict from ir_to_dict — include as-is.
        if self.body_ir is not None:
            d["body_ir"] = self.body_ir
        return d


@dataclass
class ExtractedTemplateParam:
    name: str
    is_type_param: bool = True   # typename T vs non-type
    default_value: str = ""
    value_type: str = ""         # For non-type params

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedBaseClass:
    name: str
    qualified_name: str = ""
    access: str = "public"
    is_virtual: bool = False

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedClass:
    name: str
    qualified_name: str = ""
    is_struct: bool = False
    fields: List[ExtractedField] = field(default_factory=list)
    methods: List[ExtractedFunction] = field(default_factory=list)
    base_classes: List[ExtractedBaseClass] = field(default_factory=list)
    nested_enums: List[ExtractedEnum] = field(default_factory=list)
    nested_classes: List[ExtractedClass] = field(default_factory=list)
    typedefs: List[ExtractedTypedef] = field(default_factory=list)
    template_params: List[ExtractedTemplateParam] = field(default_factory=list)
    is_abstract: bool = False
    is_forward_decl: bool = False
    access: str = "public"
    file: str = ""
    line: int = 0

    def to_dict(self) -> dict:
        d = {
            "name": self.name,
            "qualified_name": self.qualified_name,
            "is_struct": self.is_struct,
            "fields": [f.to_dict() for f in self.fields],
            "methods": [m.to_dict() for m in self.methods],
            "base_classes": [b.to_dict() for b in self.base_classes],
            "nested_enums": [e.to_dict() for e in self.nested_enums],
            "nested_classes": [c.to_dict() for c in self.nested_classes],
            "typedefs": [t.to_dict() for t in self.typedefs],
            "template_params": [t.to_dict() for t in self.template_params],
            "is_abstract": self.is_abstract,
            "is_forward_decl": self.is_forward_decl,
            "access": self.access,
            "file": self.file,
            "line": self.line,
        }
        return d


@dataclass
class ExtractedTypedef:
    name: str
    qualified_name: str = ""
    underlying_type: str = ""
    canonical_type: str = ""
    template_params: List[ExtractedTemplateParam] = field(default_factory=list)
    file: str = ""
    line: int = 0

    def to_dict(self) -> dict:
        d = asdict(self)
        d["template_params"] = [t.to_dict() for t in self.template_params]
        return d


@dataclass
class ExtractedVariable:
    name: str
    qualified_name: str = ""
    type: str = ""
    canonical_type: str = ""
    is_const: bool = False
    is_constexpr: bool = False
    is_static: bool = False
    value: str = ""              # Initializer value if simple
    file: str = ""
    line: int = 0

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ExtractedNamespace:
    name: str
    qualified_name: str = ""
    classes: List[ExtractedClass] = field(default_factory=list)
    functions: List[ExtractedFunction] = field(default_factory=list)
    enums: List[ExtractedEnum] = field(default_factory=list)
    typedefs: List[ExtractedTypedef] = field(default_factory=list)
    variables: List[ExtractedVariable] = field(default_factory=list)
    namespaces: List[ExtractedNamespace] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "qualified_name": self.qualified_name,
            "classes": [c.to_dict() for c in self.classes],
            "functions": [f.to_dict() for f in self.functions],
            "enums": [e.to_dict() for e in self.enums],
            "typedefs": [t.to_dict() for t in self.typedefs],
            "variables": [v.to_dict() for v in self.variables],
            "namespaces": [n.to_dict() for n in self.namespaces],
        }


@dataclass
class ExtractedFile:
    path: str
    includes: List[str] = field(default_factory=list)
    namespaces: List[ExtractedNamespace] = field(default_factory=list)
    # Top-level declarations (outside any namespace)
    classes: List[ExtractedClass] = field(default_factory=list)
    functions: List[ExtractedFunction] = field(default_factory=list)
    enums: List[ExtractedEnum] = field(default_factory=list)
    typedefs: List[ExtractedTypedef] = field(default_factory=list)
    variables: List[ExtractedVariable] = field(default_factory=list)
    parse_errors: List[str] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "path": self.path,
            "includes": self.includes,
            "namespaces": [n.to_dict() for n in self.namespaces],
            "classes": [c.to_dict() for c in self.classes],
            "functions": [f.to_dict() for f in self.functions],
            "enums": [e.to_dict() for e in self.enums],
            "typedefs": [t.to_dict() for t in self.typedefs],
            "variables": [v.to_dict() for v in self.variables],
            "parse_errors": self.parse_errors,
        }


# ─── Main AST Parser ────────────────────────────────────────────────────────

class ASTParser:
    """Parses C++ files using libclang and extracts structured information."""

    def __init__(self, config: TranspilerConfig):
        self.config = config
        self.index = Index.create()
        self._processed_files: Set[str] = set()
        self._stmt_converter = None  # Lazy-init to avoid import if unused

    def _get_stmt_converter(self):
        """Lazy-initialise the StmtConverter."""
        if self._stmt_converter is None:
            from extractor.stmt_converter import StmtConverter
            self._stmt_converter = StmtConverter()
        return self._stmt_converter

    def parse_module(self, module_name: str, files: List[Path],
                     extract_bodies: bool = True) -> dict:
        """Parse all files in a module and return extraction results.

        When *extract_bodies* is True, function bodies are parsed and
        converted to IR using the StmtConverter.
        """
        results = {
            "module": module_name,
            "files": [],
            "stats": {
                "total_files": len(files),
                "parsed_files": 0,
                "failed_files": 0,
                "classes": 0,
                "functions": 0,
                "enums": 0,
            }
        }

        for fpath in files:
            try:
                if extract_bodies:
                    extracted = self.parse_file_with_bodies(fpath)
                else:
                    extracted = self.parse_file(fpath)
                results["files"].append(extracted.to_dict())
                results["stats"]["parsed_files"] += 1
                results["stats"]["classes"] += self._count_classes(extracted)
                results["stats"]["functions"] += self._count_functions(extracted)
                results["stats"]["enums"] += self._count_enums(extracted)
            except Exception as e:
                results["files"].append({
                    "path": str(fpath),
                    "parse_errors": [str(e)],
                })
                results["stats"]["failed_files"] += 1

        return results

    def parse_file(self, file_path: Path) -> ExtractedFile:
        """Parse a single C++ file and extract all declarations."""
        file_str = str(file_path)
        result = ExtractedFile(path=file_str)

        # Parse with Clang
        try:
            tu = self.index.parse(
                file_str,
                args=self.config.clang_args,
                options=(
                    TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD |
                    TranslationUnit.PARSE_SKIP_FUNCTION_BODIES  # First pass: structure only
                ),
            )
        except Exception as e:
            result.parse_errors.append(f"Clang parse failed: {e}")
            return result

        # Collect diagnostics (errors/warnings)
        for diag in tu.diagnostics:
            if diag.severity >= 3:  # Error or Fatal
                result.parse_errors.append(
                    f"{diag.location.file}:{diag.location.line}: {diag.spelling}"
                )

        # Extract includes
        result.includes = self._extract_includes(tu, file_str)

        # Walk the AST
        self._visit_children(tu.cursor, result, file_str)

        return result

    def parse_file_with_bodies(self, file_path: Path) -> ExtractedFile:
        """Parse a C++ file including function bodies (slower, more detail)."""
        file_str = str(file_path)
        result = ExtractedFile(path=file_str)

        try:
            tu = self.index.parse(
                file_str,
                args=self.config.clang_args,
                options=TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD,
            )
        except Exception as e:
            result.parse_errors.append(f"Clang parse failed: {e}")
            return result

        for diag in tu.diagnostics:
            if diag.severity >= 3:
                result.parse_errors.append(
                    f"{diag.location.file}:{diag.location.line}: {diag.spelling}"
                )

        result.includes = self._extract_includes(tu, file_str)
        self._visit_children(tu.cursor, result, file_str, extract_bodies=True)

        return result

    # ─── Include extraction ──────────────────────────────────────────────

    def _extract_includes(self, tu: TranslationUnit, source_file: str) -> List[str]:
        """Extract #include directives from the file."""
        includes = []
        for inc in tu.get_includes():
            # Only get direct includes from this file
            if inc.depth == 1:
                include_path = inc.include.name
                includes.append(include_path)
        return includes

    # ─── AST traversal ───────────────────────────────────────────────────

    def _is_from_file(self, cursor: Cursor, target_file: str) -> bool:
        """Check if a cursor originates from the target file."""
        loc = cursor.location
        if loc.file is None:
            return False
        return os.path.abspath(loc.file.name) == os.path.abspath(target_file)

    def _visit_children(self, cursor: Cursor, result: ExtractedFile,
                        target_file: str, extract_bodies: bool = False):
        """Recursively visit AST children and extract declarations."""
        for child in cursor.get_children():
            if not self._is_from_file(child, target_file):
                continue

            kind = child.kind

            if kind == CursorKind.NAMESPACE:
                ns = self._extract_namespace(child, target_file, extract_bodies)
                if ns:
                    result.namespaces.append(ns)

            elif kind in (CursorKind.CLASS_DECL, CursorKind.STRUCT_DECL,
                          CursorKind.CLASS_TEMPLATE):
                cls = self._extract_class(child, target_file, extract_bodies)
                if cls and not cls.is_forward_decl:
                    result.classes.append(cls)

            elif kind in (CursorKind.FUNCTION_DECL, CursorKind.FUNCTION_TEMPLATE):
                func = self._extract_function(child, extract_bodies)
                if func:
                    result.functions.append(func)

            elif kind in (CursorKind.ENUM_DECL,):
                enum = self._extract_enum(child)
                if enum:
                    result.enums.append(enum)

            elif kind in (CursorKind.TYPEDEF_DECL, CursorKind.TYPE_ALIAS_DECL,
                          CursorKind.TYPE_ALIAS_TEMPLATE_DECL):
                td = self._extract_typedef(child)
                if td:
                    result.typedefs.append(td)

            elif kind == CursorKind.VAR_DECL:
                var = self._extract_variable(child)
                if var:
                    result.variables.append(var)

    # ─── Namespace extraction ────────────────────────────────────────────

    def _extract_namespace(self, cursor: Cursor, target_file: str,
                           extract_bodies: bool = False) -> Optional[ExtractedNamespace]:
        """Extract a namespace and its contents."""
        name = cursor.spelling or "(anonymous)"
        ns = ExtractedNamespace(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
        )

        for child in cursor.get_children():
            if not self._is_from_file(child, target_file):
                continue

            kind = child.kind

            if kind == CursorKind.NAMESPACE:
                sub_ns = self._extract_namespace(child, target_file, extract_bodies)
                if sub_ns:
                    ns.namespaces.append(sub_ns)

            elif kind in (CursorKind.CLASS_DECL, CursorKind.STRUCT_DECL,
                          CursorKind.CLASS_TEMPLATE):
                cls = self._extract_class(child, target_file, extract_bodies)
                if cls and not cls.is_forward_decl:
                    ns.classes.append(cls)

            elif kind in (CursorKind.FUNCTION_DECL, CursorKind.FUNCTION_TEMPLATE):
                func = self._extract_function(child, extract_bodies)
                if func:
                    ns.functions.append(func)

            elif kind == CursorKind.ENUM_DECL:
                enum = self._extract_enum(child)
                if enum:
                    ns.enums.append(enum)

            elif kind in (CursorKind.TYPEDEF_DECL, CursorKind.TYPE_ALIAS_DECL,
                          CursorKind.TYPE_ALIAS_TEMPLATE_DECL):
                td = self._extract_typedef(child)
                if td:
                    ns.typedefs.append(td)

            elif kind == CursorKind.VAR_DECL:
                var = self._extract_variable(child)
                if var:
                    ns.variables.append(var)

        return ns

    # ─── Class/Struct extraction ─────────────────────────────────────────

    def _extract_class(self, cursor: Cursor, target_file: str,
                       extract_bodies: bool = False) -> Optional[ExtractedClass]:
        """Extract a class or struct declaration."""
        name = cursor.spelling
        if not name:
            return None

        is_template = cursor.kind == CursorKind.CLASS_TEMPLATE
        is_struct = cursor.kind == CursorKind.STRUCT_DECL

        # Check if forward declaration
        is_definition = cursor.is_definition()

        cls = ExtractedClass(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
            is_struct=is_struct,
            is_forward_decl=not is_definition,
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        if not is_definition:
            return cls

        # Extract template parameters
        if is_template:
            cls.template_params = self._extract_template_params(cursor)

        # Walk children
        for child in cursor.get_children():
            kind = child.kind

            # Base classes
            if kind == CursorKind.CXX_BASE_SPECIFIER:
                base = self._extract_base_class(child)
                if base:
                    cls.base_classes.append(base)

            # Fields
            elif kind == CursorKind.FIELD_DECL:
                fld = self._extract_field(child)
                if fld:
                    cls.fields.append(fld)

            # Methods (including constructors and destructors)
            elif kind in (CursorKind.CXX_METHOD, CursorKind.FUNCTION_TEMPLATE):
                method = self._extract_function(child, extract_bodies)
                if method:
                    method.is_method = True  # not in ExtractedFunction, but we tag it
                    cls.methods.append(method)

            elif kind == CursorKind.CONSTRUCTOR:
                ctor = self._extract_constructor(child, extract_bodies)
                if ctor:
                    cls.methods.append(ctor)

            elif kind == CursorKind.DESTRUCTOR:
                dtor = self._extract_destructor(child, extract_bodies)
                if dtor:
                    cls.methods.append(dtor)

            # Nested enums
            elif kind == CursorKind.ENUM_DECL:
                enum = self._extract_enum(child)
                if enum:
                    cls.nested_enums.append(enum)

            # Nested classes/structs
            elif kind in (CursorKind.CLASS_DECL, CursorKind.STRUCT_DECL,
                          CursorKind.CLASS_TEMPLATE):
                nested = self._extract_class(child, target_file, extract_bodies)
                if nested and not nested.is_forward_decl:
                    cls.nested_classes.append(nested)

            # Typedefs inside the class
            elif kind in (CursorKind.TYPEDEF_DECL, CursorKind.TYPE_ALIAS_DECL):
                td = self._extract_typedef(child)
                if td:
                    cls.typedefs.append(td)

            # Static member variables
            elif kind == CursorKind.VAR_DECL:
                fld = ExtractedField(
                    name=child.spelling,
                    type=child.type.spelling,
                    canonical_type=child.type.get_canonical().spelling,
                    access=self._get_access(child),
                    is_static=True,
                    is_const="const" in child.type.spelling,
                )
                cls.fields.append(fld)

        # Determine if abstract (has any pure virtual methods)
        cls.is_abstract = any(m.is_pure_virtual for m in cls.methods)

        return cls

    def _extract_base_class(self, cursor: Cursor) -> Optional[ExtractedBaseClass]:
        """Extract a base class specifier."""
        # The type of the base specifier gives us the base class
        base_type = cursor.type
        if not base_type:
            return None

        # Get the referenced declaration
        ref_cursor = cursor.get_definition()
        name = base_type.spelling
        qualified = name

        if ref_cursor:
            qualified = self._get_qualified_name(ref_cursor)

        return ExtractedBaseClass(
            name=name,
            qualified_name=qualified,
            access=self._get_access(cursor),
            is_virtual=any(
                c.kind == CursorKind.CXX_VIRTUAL_BASE_SPECIFIER
                for c in cursor.get_children()
            ),
        )

    def _extract_field(self, cursor: Cursor) -> Optional[ExtractedField]:
        """Extract a class/struct field."""
        return ExtractedField(
            name=cursor.spelling,
            type=cursor.type.spelling,
            canonical_type=cursor.type.get_canonical().spelling,
            access=self._get_access(cursor),
            is_static=False,
            is_const="const" in cursor.type.spelling,
            is_mutable=any(
                t.spelling == "mutable"
                for t in cursor.get_tokens()
            ),
            offset=cursor.get_field_offsetof() if hasattr(cursor, 'get_field_offsetof') else -1,
        )

    # ─── Function extraction ─────────────────────────────────────────────

    def _extract_function(self, cursor: Cursor,
                          extract_bodies: bool = False) -> Optional[ExtractedFunction]:
        """Extract a function or method declaration."""
        name = cursor.spelling
        if not name:
            return None

        is_template = cursor.kind in (CursorKind.FUNCTION_TEMPLATE,)

        func = ExtractedFunction(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
            return_type=cursor.result_type.spelling if cursor.result_type else "void",
            canonical_return_type=(
                cursor.result_type.get_canonical().spelling
                if cursor.result_type else "void"
            ),
            access=self._get_access(cursor),
            is_virtual=cursor.is_virtual_method(),
            is_pure_virtual=cursor.is_pure_virtual_method(),
            is_static=cursor.is_static_method() if hasattr(cursor, 'is_static_method') else False,
            is_const=cursor.is_const_method() if hasattr(cursor, 'is_const_method') else False,
            has_body=bool(cursor.is_definition()),
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        # Check for operator overloading
        if name.startswith("operator"):
            func.is_operator = True
            func.operator_name = name

        # Check for inline/constexpr via tokens
        self._check_function_qualifiers(cursor, func)

        # Extract parameters
        func.params = self._extract_params(cursor)

        # Extract template parameters
        if is_template:
            func.template_params = self._extract_template_params(cursor)

        # Check for override
        for child in cursor.get_children():
            if child.kind == CursorKind.CXX_OVERRIDE_ATTR:
                func.is_override = True

        # Extract body source and IR if requested
        if extract_bodies and cursor.is_definition():
            func.body_source = self._extract_body_source(cursor)
            func.body_ir = self._convert_body_to_ir(cursor)
            if cursor.extent:
                func.body_start_line = cursor.extent.start.line
                func.body_end_line = cursor.extent.end.line

        return func

    def _extract_constructor(self, cursor: Cursor,
                             extract_bodies: bool = False) -> Optional[ExtractedFunction]:
        """Extract a constructor."""
        func = ExtractedFunction(
            name=cursor.spelling,
            qualified_name=self._get_qualified_name(cursor),
            return_type="",
            access=self._get_access(cursor),
            is_constructor=True,
            has_body=bool(cursor.is_definition()),
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        func.params = self._extract_params(cursor)

        # Check if copy or move constructor
        if len(func.params) == 1:
            ptype = func.params[0].type
            if "const" in ptype and "&" in ptype:
                func.is_copy_constructor = True
            elif "&&" in ptype:
                func.is_move_constructor = True

        # Check for = delete / = default
        self._check_function_qualifiers(cursor, func)

        if extract_bodies and cursor.is_definition():
            func.body_source = self._extract_body_source(cursor)
            func.body_ir = self._convert_body_to_ir(cursor)

        return func

    def _extract_destructor(self, cursor: Cursor,
                            extract_bodies: bool = False) -> Optional[ExtractedFunction]:
        """Extract a destructor."""
        func = ExtractedFunction(
            name=cursor.spelling,
            qualified_name=self._get_qualified_name(cursor),
            return_type="void",
            access=self._get_access(cursor),
            is_destructor=True,
            is_virtual=cursor.is_virtual_method(),
            has_body=bool(cursor.is_definition()),
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        if extract_bodies and cursor.is_definition():
            func.body_source = self._extract_body_source(cursor)
            func.body_ir = self._convert_body_to_ir(cursor)

        return func

    def _extract_params(self, cursor: Cursor) -> List[ExtractedParam]:
        """Extract function parameters."""
        params = []
        for child in cursor.get_children():
            if child.kind == CursorKind.PARM_DECL:
                param = ExtractedParam(
                    name=child.spelling or f"arg{len(params)}",
                    type=child.type.spelling,
                    canonical_type=child.type.get_canonical().spelling,
                    is_const="const" in child.type.spelling,
                )
                # Try to get default value
                for token in child.get_tokens():
                    pass  # TODO: extract default value from tokens
                params.append(param)
        return params

    # ─── Enum extraction ─────────────────────────────────────────────────

    def _extract_enum(self, cursor: Cursor) -> Optional[ExtractedEnum]:
        """Extract an enum declaration."""
        name = cursor.spelling
        if not name:
            # Anonymous enum — skip or generate name
            return None

        enum = ExtractedEnum(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
            is_scoped=cursor.is_scoped_enum() if hasattr(cursor, 'is_scoped_enum') else False,
            access=self._get_access(cursor),
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        # Get underlying type
        if cursor.enum_type:
            enum.underlying_type = cursor.enum_type.spelling

        # Extract variants
        for child in cursor.get_children():
            if child.kind == CursorKind.ENUM_CONSTANT_DECL:
                variant = ExtractedEnumVariant(
                    name=child.spelling,
                    value=child.enum_value if hasattr(child, 'enum_value') else None,
                    value_str=str(child.enum_value) if hasattr(child, 'enum_value') else "",
                )
                enum.variants.append(variant)

        return enum

    # ─── Typedef extraction ──────────────────────────────────────────────

    def _extract_typedef(self, cursor: Cursor) -> Optional[ExtractedTypedef]:
        """Extract a typedef or using declaration."""
        name = cursor.spelling
        if not name:
            return None

        underlying = ""
        canonical = ""

        if cursor.kind == CursorKind.TYPEDEF_DECL:
            underlying = cursor.underlying_typedef_type.spelling if hasattr(cursor, 'underlying_typedef_type') else ""
            canonical = cursor.underlying_typedef_type.get_canonical().spelling if hasattr(cursor, 'underlying_typedef_type') else ""
        elif cursor.kind in (CursorKind.TYPE_ALIAS_DECL, CursorKind.TYPE_ALIAS_TEMPLATE_DECL):
            # For 'using' aliases, get the underlying type from children
            for child in cursor.get_children():
                if child.kind == CursorKind.TYPE_REF:
                    underlying = child.type.spelling
                    canonical = child.type.get_canonical().spelling
                    break
            if not underlying:
                underlying = cursor.type.spelling

        td = ExtractedTypedef(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
            underlying_type=underlying,
            canonical_type=canonical,
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        # Template params for using alias templates
        if cursor.kind == CursorKind.TYPE_ALIAS_TEMPLATE_DECL:
            td.template_params = self._extract_template_params(cursor)

        return td

    # ─── Variable extraction ─────────────────────────────────────────────

    def _extract_variable(self, cursor: Cursor) -> Optional[ExtractedVariable]:
        """Extract a global/namespace-scope variable or constant."""
        name = cursor.spelling
        if not name:
            return None

        var = ExtractedVariable(
            name=name,
            qualified_name=self._get_qualified_name(cursor),
            type=cursor.type.spelling,
            canonical_type=cursor.type.get_canonical().spelling,
            is_const="const" in cursor.type.spelling,
            file=cursor.location.file.name if cursor.location.file else "",
            line=cursor.location.line,
        )

        # Check for constexpr
        self._check_var_qualifiers(cursor, var)

        # Try to extract simple initializer value
        for child in cursor.get_children():
            if child.kind in (CursorKind.INTEGER_LITERAL, CursorKind.FLOATING_LITERAL,
                              CursorKind.STRING_LITERAL, CursorKind.CXX_BOOL_LITERAL_EXPR):
                tokens = list(child.get_tokens())
                if tokens:
                    var.value = tokens[0].spelling

        return var

    # ─── Template parameter extraction ───────────────────────────────────

    def _extract_template_params(self, cursor: Cursor) -> List[ExtractedTemplateParam]:
        """Extract template parameters from a template declaration."""
        params = []
        for child in cursor.get_children():
            if child.kind == CursorKind.TEMPLATE_TYPE_PARAMETER:
                param = ExtractedTemplateParam(
                    name=child.spelling or f"T{len(params)}",
                    is_type_param=True,
                )
                # Check for default type
                for grandchild in child.get_children():
                    if grandchild.kind == CursorKind.TYPE_REF:
                        param.default_value = grandchild.type.spelling
                params.append(param)

            elif child.kind == CursorKind.TEMPLATE_NON_TYPE_PARAMETER:
                param = ExtractedTemplateParam(
                    name=child.spelling or f"N{len(params)}",
                    is_type_param=False,
                    value_type=child.type.spelling,
                )
                params.append(param)

            elif child.kind == CursorKind.TEMPLATE_TEMPLATE_PARAMETER:
                param = ExtractedTemplateParam(
                    name=child.spelling or f"TT{len(params)}",
                    is_type_param=True,  # Treat as type param for Rust generics
                )
                params.append(param)

        return params

    # ─── Helper methods ──────────────────────────────────────────────────

    def _get_qualified_name(self, cursor: Cursor) -> str:
        """Get the fully qualified name of a declaration."""
        parts = []
        c = cursor
        while c and c.kind != CursorKind.TRANSLATION_UNIT:
            if c.spelling:
                parts.append(c.spelling)
            c = c.semantic_parent
        return "::".join(reversed(parts))

    def _get_access(self, cursor: Cursor) -> str:
        """Get the access specifier of a declaration."""
        access = cursor.access_specifier
        if access == ClangAccess.PUBLIC:
            return "public"
        elif access == ClangAccess.PROTECTED:
            return "protected"
        elif access == ClangAccess.PRIVATE:
            return "private"
        return "public"  # Default for top-level declarations

    def _check_function_qualifiers(self, cursor: Cursor, func: ExtractedFunction):
        """Check for inline, constexpr, deleted, defaulted keywords via tokens."""
        try:
            tokens = list(cursor.get_tokens())
            token_strs = [t.spelling for t in tokens]

            if "inline" in token_strs:
                func.is_inline = True
            if "constexpr" in token_strs:
                func.is_constexpr = True
            if "delete" in token_strs:
                func.is_deleted = True
            if "default" in token_strs:
                func.is_defaulted = True
            if "static" in token_strs:
                func.is_static = True
        except Exception:
            pass

    def _check_var_qualifiers(self, cursor: Cursor, var: ExtractedVariable):
        """Check for constexpr, static keywords on variables."""
        try:
            tokens = list(cursor.get_tokens())
            token_strs = [t.spelling for t in tokens]

            if "constexpr" in token_strs:
                var.is_constexpr = True
            if "static" in token_strs:
                var.is_static = True
        except Exception:
            pass

    def _convert_body_to_ir(self, cursor: Cursor) -> Optional[dict]:
        """Convert the COMPOUND_STMT child of *cursor* to a serialised IR dict.

        Returns None if no body is found or conversion fails.
        """
        try:
            from ir.nodes import ir_to_dict
            converter = self._get_stmt_converter()
            # Find the COMPOUND_STMT child (the function body).
            for child in cursor.get_children():
                if child.kind == CursorKind.COMPOUND_STMT:
                    ir_block = converter.convert_stmt(child)
                    return ir_to_dict(ir_block)
        except Exception:
            pass
        return None

    def _extract_body_source(self, cursor: Cursor) -> str:
        """Extract the raw C++ source code of a function body."""
        try:
            extent = cursor.extent
            if not extent:
                return ""

            file_path = extent.start.file.name
            with open(file_path, "r", errors="replace") as f:
                lines = f.readlines()

            start_line = extent.start.line - 1  # 0-indexed
            end_line = extent.end.line
            body_lines = lines[start_line:end_line]

            # Find the opening brace of the body
            source = "".join(body_lines)
            brace_idx = source.find("{")
            if brace_idx >= 0:
                # Return from opening brace to closing brace
                depth = 0
                for i in range(brace_idx, len(source)):
                    if source[i] == "{":
                        depth += 1
                    elif source[i] == "}":
                        depth -= 1
                        if depth == 0:
                            return source[brace_idx:i + 1]
            return source
        except Exception:
            return ""

    def _count_classes(self, extracted: ExtractedFile) -> int:
        count = len(extracted.classes)
        for ns in extracted.namespaces:
            count += self._count_classes_in_ns(ns)
        return count

    def _count_classes_in_ns(self, ns: ExtractedNamespace) -> int:
        count = len(ns.classes)
        for sub in ns.namespaces:
            count += self._count_classes_in_ns(sub)
        return count

    def _count_functions(self, extracted: ExtractedFile) -> int:
        count = len(extracted.functions)
        for ns in extracted.namespaces:
            count += self._count_functions_in_ns(ns)
        for cls in extracted.classes:
            count += len(cls.methods)
        return count

    def _count_functions_in_ns(self, ns: ExtractedNamespace) -> int:
        count = len(ns.functions)
        for cls in ns.classes:
            count += len(cls.methods)
        for sub in ns.namespaces:
            count += self._count_functions_in_ns(sub)
        return count

    def _count_enums(self, extracted: ExtractedFile) -> int:
        count = len(extracted.enums)
        for ns in extracted.namespaces:
            count += self._count_enums_in_ns(ns)
        return count

    def _count_enums_in_ns(self, ns: ExtractedNamespace) -> int:
        count = len(ns.enums)
        for sub in ns.namespaces:
            count += self._count_enums_in_ns(sub)
        return count

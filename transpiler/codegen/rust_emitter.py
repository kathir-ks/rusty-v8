"""Rust source code emitter for the C++ to Rust transpiler.

Converts IR nodes into properly formatted Rust source text.  The main entry
point is ``RustEmitter.emit_file`` which returns a complete ``*.rs`` file as a
string.  Individual helpers (``emit_struct``, ``emit_enum``, etc.) can also be
called directly when only a fragment is needed.
"""

from __future__ import annotations

import re
from typing import List, Optional

from ir.nodes import (
    # Enums
    AccessSpecifier,
    BinaryOp,
    CastKind,
    LiteralKind,
    UnaryOp,
    # Type IR
    IRType,
    IRTemplateParam,
    # Fields / params
    IRField,
    IRParam,
    # Enum
    IREnumVariant,
    IREnum,
    # Expressions
    IRLiteral,
    IRNameRef,
    IRBinaryExpr,
    IRUnaryExpr,
    IRCallExpr,
    IRMemberExpr,
    IRIndexExpr,
    IRCastExpr,
    IRTernaryExpr,
    IRNewExpr,
    IRDeleteExpr,
    IRSizeofExpr,
    IRLambdaExpr,
    IRThisExpr,
    IRInitListExpr,
    IRRawExpr,
    # Statements
    IRBlock,
    IRVarDecl,
    IRExprStmt,
    IRReturnStmt,
    IRIfStmt,
    IRMatchStmt,
    IRMatchArm,
    IRForLoop,
    IRForRangeLoop,
    IRWhileLoop,
    IRBreakStmt,
    IRContinueStmt,
    IRThrowStmt,
    IRTryCatchStmt,
    IRCatchBlock,
    IRUnsafeBlock,
    IRRawStmt,
    # Top-level items
    IRFunction,
    IRBaseClass,
    IRStruct,
    IRTrait,
    IRImplBlock,
    IRTypeAlias,
    IRConst,
    IRUseDecl,
    IRMacro,
    # File / module
    IRFile,
)

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

_INDENT = "    "  # 4 spaces per indentation level


def _vis(access: AccessSpecifier) -> str:
    """Return the Rust visibility keyword followed by a space, or empty."""
    val = access.value
    if val:
        return val + " "
    return ""


def _is_assignment_op(op: BinaryOp) -> bool:
    return op in {
        BinaryOp.ASSIGN,
        BinaryOp.ADD_ASSIGN,
        BinaryOp.SUB_ASSIGN,
        BinaryOp.MUL_ASSIGN,
        BinaryOp.DIV_ASSIGN,
        BinaryOp.MOD_ASSIGN,
        BinaryOp.BIT_AND_ASSIGN,
        BinaryOp.BIT_OR_ASSIGN,
        BinaryOp.BIT_XOR_ASSIGN,
        BinaryOp.SHL_ASSIGN,
        BinaryOp.SHR_ASSIGN,
    }


# ---------------------------------------------------------------------------
# RustEmitter
# ---------------------------------------------------------------------------

class RustEmitter:
    """Converts IR nodes into formatted Rust source text.

    Usage::

        emitter = RustEmitter()
        source = emitter.emit_file(ir_file)
    """

    def __init__(self) -> None:
        self._indent: int = 0  # current indentation level

    # -- indentation helpers -------------------------------------------------

    def _i(self) -> str:
        """Return the current indentation string."""
        return _INDENT * self._indent

    def _indent_inc(self) -> None:
        self._indent += 1

    def _indent_dec(self) -> None:
        self._indent = max(0, self._indent - 1)

    # -----------------------------------------------------------------------
    # Public entry points
    # -----------------------------------------------------------------------

    def emit_file(self, ir_file: IRFile) -> str:
        """Emit a full Rust source file and return it as a string."""
        self._indent = 0
        parts: List[str] = []

        # File-level cfg attributes
        for cfg in ir_file.cfg_attrs:
            parts.append(f"#![cfg({cfg})]")
        if ir_file.cfg_attrs:
            parts.append("")

        # Module-level doc comment
        if ir_file.module_doc:
            for line in ir_file.module_doc.splitlines():
                parts.append(f"//! {line}" if line.strip() else "//!")
            parts.append("")

        # Source provenance comment
        if ir_file.cpp_source_files:
            parts.append("// Auto-generated from:")
            for src in ir_file.cpp_source_files:
                parts.append(f"//   {src}")
            parts.append("")

        # Import crate-level types (prelude stubs from lib.rs)
        parts.append("use crate::*;")
        parts.append("")

        # Use declarations
        if ir_file.uses:
            for use in ir_file.uses:
                parts.append(self.emit_use(use))
            parts.append("")

        # Items
        for idx, item in enumerate(ir_file.items):
            text = self.emit_item(item)
            if text:
                parts.append(text)
                # blank line between top-level items
                if idx < len(ir_file.items) - 1:
                    parts.append("")

        source = "\n".join(parts) + "\n"
        return self._post_process(source)

    @staticmethod
    def _post_process(source: str) -> str:
        """Clean up known problematic patterns in the emitted Rust source."""
        # 1. Remove standalone _msg references (from expanded DCHECK macros).
        #    Patterns: "_msg;", "_msg();", "drop(_msg);", "let _msg = ..."
        source = re.sub(r'^\s*_msg\s*;\s*$', '', source, flags=re.MULTILINE)
        source = re.sub(r'^\s*_msg\(\)\s*;\s*$', '', source, flags=re.MULTILINE)
        source = re.sub(r'^\s*drop\(_msg\)\s*;\s*$', '', source, flags=re.MULTILINE)
        source = re.sub(r'^\s*let\s+(?:mut\s+)?_msg\b[^;]*;\s*$', '', source, flags=re.MULTILINE)

        # 2. Remove lines with raw DCHECK/CHECK tuple patterns from macro expansion.
        #    Pattern: ("DCHECK_...", DCHECK_..., _msg());
        source = re.sub(
            r'^\s*\("(?:DCHECK|CHECK|SLOW_DCHECK)[^"]*"[^;]*;\s*$',
            '',
            source,
            flags=re.MULTILINE,
        )

        # 3. Remove empty loop blocks that were DCHECK expansion wrappers.
        #    Pattern: loop { <only whitespace/empty lines> }
        source = re.sub(
            r'^\s*loop\s*\{\s*\n(?:\s*\n)*\s*\}\s*$',
            '',
            source,
            flags=re.MULTILINE,
        )

        # 3c. Replace raw dcheck/check function calls in raw expressions
        #     These survive as raw text from DCHECK macro expansion
        source = re.sub(r'\bdcheck_le\(([^,]+),\s*([^)]+)\)', r'debug_assert!(\1 <= \2)', source)
        source = re.sub(r'\bdcheck_lt\(([^,]+),\s*([^)]+)\)', r'debug_assert!(\1 < \2)', source)
        source = re.sub(r'\bdcheck_gt\(([^,]+),\s*([^)]+)\)', r'debug_assert!(\1 > \2)', source)
        source = re.sub(r'\bdcheck_ge\(([^,]+),\s*([^)]+)\)', r'debug_assert!(\1 >= \2)', source)
        source = re.sub(r'\bdcheck_eq\(([^,]+),\s*([^)]+)\)', r'debug_assert_eq!(\1, \2)', source)
        source = re.sub(r'\bdcheck_ne\(([^,]+),\s*([^)]+)\)', r'debug_assert_ne!(\1, \2)', source)
        source = re.sub(r'\bdcheck\(([^)]+)\)', r'debug_assert!(\1)', source)
        source = re.sub(r'\bcheck_le\(([^,]+),\s*([^)]+)\)', r'assert!(\1 <= \2)', source)
        source = re.sub(r'\bcheck_lt\(([^,]+),\s*([^)]+)\)', r'assert!(\1 < \2)', source)
        source = re.sub(r'\bcheck_gt\(([^,]+),\s*([^)]+)\)', r'assert!(\1 > \2)', source)
        source = re.sub(r'\bcheck_ge\(([^,]+),\s*([^)]+)\)', r'assert!(\1 >= \2)', source)
        source = re.sub(r'\bcheck_eq\(([^,]+),\s*([^)]+)\)', r'assert_eq!(\1, \2)', source)
        source = re.sub(r'\bcheck_ne\(([^,]+),\s*([^)]+)\)', r'assert_ne!(\1, \2)', source)
        source = re.sub(r'\bslow_dcheck\(([^)]+)\)', r'debug_assert!(\1)', source)

        # 3d. Remove `if !(debug_assert!(...)) { break; }` wrapper pattern
        #     from DCHECK macro expansion. The assertion is self-contained.
        source = re.sub(
            r'^\s*if\s+!\((?:debug_assert(?:_eq|_ne)?!|assert(?:_eq|_ne)?!)\([^)]*\)\)\s*\{\s*\n\s*break;\s*\n\s*\}',
            '',
            source,
            flags=re.MULTILINE,
        )

        # 4. Fix debug_assert!/assert! with string literal + expression pattern
        #    from DCHECK expansion: debug_assert!("...", EXPR, "") → debug_assert!(EXPR)
        #    Broadened to match any string-prefixed assert pattern.
        source = re.sub(
            r'(debug_assert(?:_eq|_ne)?!)\s*\("[^"]*",\s*([^,]+),\s*""\)',
            r'\1(\2)',
            source,
        )
        # Also handle: assert!("...", EXPR, "")
        source = re.sub(
            r'(assert(?:_eq|_ne)?!)\s*\("[^"]*",\s*([^,]+),\s*""\)',
            r'\1(\2)',
            source,
        )

        # 4b. Fix empty if/while conditions from DCHECK/CHECK macro expansion.
        #    Pattern: `if /* TODO: ... */ {` → `if true /* TODO: ... */ {`
        source = re.sub(
            r'\bif\s+(/\*[^*]*\*/)\s*\{',
            r'if true \1 {',
            source,
        )
        source = re.sub(
            r'\bwhile\s+(/\*[^*]*\*/)\s*\{',
            r'while true \1 {',
            source,
        )

        # 5. Convert remaining C++ `->` to Rust `.` (member access), but NOT
        #    in function return type position (`fn foo() -> Type`).
        #    Return type arrows follow `)` at the end of a function signature,
        #    identified by the `fn` keyword earlier on the same logical context.
        #    We protect lines containing `fn ... ) ->` and replace all others.
        _RET_ARROW = '\x00RET_ARROW\x00'
        # Protect return-type arrows: `fn name(...) -> RetType`
        source = re.sub(
            r'(fn\s+\w+\s*(?:<[^>]*>)?\s*\([^)]*\))\s*->',
            lambda m: m.group().replace('->', _RET_ARROW),
            source,
        )
        source = source.replace('->', '.')
        source = source.replace(_RET_ARROW, '->')

        # 5. Convert postfix `i++` / `i--` to `i += 1` / `i -= 1`
        #    Only handle simple identifier++ at statement level (line starts
        #    with whitespace, identifier, ++, semicolon).
        source = re.sub(
            r'^(\s*)(\w+)\+\+\s*;',
            r'\1\2 += 1;',
            source,
            flags=re.MULTILINE,
        )
        source = re.sub(
            r'^(\s*)(\w+)--\s*;',
            r'\1\2 -= 1;',
            source,
            flags=re.MULTILINE,
        )

        # 8. Fix Rust keywords used as identifiers
        #    Common C++ identifiers that are Rust keywords
        # Note: don't include "crate", "self", "super" here — they are
        # valid in paths like `use crate::*;` and `self.field`.
        for kw in ("type", "match", "ref", "mod", "move", "use", "in",
                    "where", "yield", "async", "await", "dyn", "fn",
                    "loop", "unsafe", "impl"):
            # In field access: .type → .r#type
            source = re.sub(rf'\.{kw}\b', f'.r#{kw}', source)
            # In let bindings: let type = → let r#type =
            source = re.sub(rf'\blet\s+(mut\s+)?{kw}\b', rf'let \1r#{kw}', source)
            # In function parameters: (type: → (r#type:  (only after ( or ,)
            source = re.sub(rf'(?<=[,(])\s*{kw}\s*:', f' r#{kw}:', source)

        # 9. Replace C++ constants with Rust equivalents
        source = re.sub(r'\bINT_MIN\b', 'i32::MIN', source)
        source = re.sub(r'\bINT_MAX\b', 'i32::MAX', source)
        source = re.sub(r'\bUINT_MAX\b', 'u32::MAX', source)
        source = re.sub(r'\bINT64_MIN\b', 'i64::MIN', source)
        source = re.sub(r'\bINT64_MAX\b', 'i64::MAX', source)
        source = re.sub(r'\bUINT64_MAX\b', 'u64::MAX', source)
        source = re.sub(r'\bSIZE_MAX\b', 'usize::MAX', source)
        source = re.sub(r'\bnullptr\b', 'std::ptr::null()', source)

        # 10. Fix operator names leaked into call arguments
        #     Pattern: func(operator++, ) → { func += 1; func }
        source = re.sub(r'(\w+)\(operator\+\+,?\s*\)', r'{ \1 += 1; \1 }', source)
        source = re.sub(r'(\w+)\(operator--,?\s*\)', r'{ \1 -= 1; \1 }', source)
        #     Pattern: x(operator=, expr) → x = expr
        source = re.sub(r'(\w+)\(operator=,\s*(.+?)\)', r'\1 = \2', source)
        #     Pattern: x(operator., ...) → x.method(...)  (keep as todo for now)
        source = re.sub(r'(\w+)\(operator\.,\s*', r'\1.todo_op(', source)

        # 11. Fix leading/trailing commas in function call arguments
        #     Pattern: func(, arg) → func(arg)
        source = re.sub(r'\(\s*,\s*', '(', source)
        #     Pattern: func(arg, ) → func(arg)
        source = re.sub(r',\s*\)', ')', source)

        # 12. Fix remaining postfix i++ in expression context
        #     Pattern: expr++ at end of statement → expr += 1
        source = re.sub(r'(\w+)\+\+\s*;', r'\1 += 1;', source)
        source = re.sub(r'(\w+)--\s*;', r'\1 -= 1;', source)

        # 13. Fix remaining raw DCHECK/CHECK/RUNTIME_FUNCTION calls
        source = re.sub(r'\bDCHECK\(([^)]+)\)', r'debug_assert!(\1)', source)
        source = re.sub(r'\bCHECK\(([^)]+)\)', r'assert!(\1)', source)
        source = re.sub(r'\bRUNTIME_FUNCTION\(([^)]+)\)', r'/* RUNTIME_FUNCTION(\1) */', source)

        # 14. Fix C++ stream output: cerr/cout
        source = re.sub(r'\bcerr\b', 'eprintln!', source)
        source = re.sub(r'\bcout\b', 'println!', source)

        # 13. Collapse multiple blank lines into at most two.
        source = re.sub(r'\n{4,}', '\n\n\n', source)

        return source

    # -----------------------------------------------------------------------
    # Item dispatch
    # -----------------------------------------------------------------------

    def emit_item(self, item) -> str:
        """Emit any top-level IR item."""
        if isinstance(item, IRStruct):
            return self.emit_struct(item)
        if isinstance(item, IREnum):
            return self.emit_enum(item)
        if isinstance(item, IRTrait):
            return self.emit_trait(item)
        if isinstance(item, IRImplBlock):
            return self.emit_impl(item)
        if isinstance(item, IRTypeAlias):
            return self.emit_type_alias(item)
        if isinstance(item, IRConst):
            return self.emit_const(item)
        if isinstance(item, IRFunction):
            return self.emit_function(item)
        if isinstance(item, IRUseDecl):
            return self.emit_use(item)
        if isinstance(item, IRMacro):
            return self.emit_macro(item)
        return f"// TODO: unhandled IR item {type(item).__name__}"

    # -----------------------------------------------------------------------
    # Use declarations
    # -----------------------------------------------------------------------

    def emit_use(self, u: IRUseDecl) -> str:
        i = self._i()
        path = u.path
        if u.is_glob:
            path += "::*"
        if u.alias:
            return f"{i}use {path} as {u.alias};"
        return f"{i}use {path};"

    # -----------------------------------------------------------------------
    # Struct
    # -----------------------------------------------------------------------

    def emit_struct(self, s: IRStruct) -> str:
        parts: List[str] = []
        i = self._i()

        # Doc comment
        if s.comment:
            for line in s.comment.splitlines():
                parts.append(f"{i}/// {line}" if line.strip() else f"{i}///")

        # Derive attribute
        if s.derives:
            parts.append(f"{i}#[derive({', '.join(s.derives)})]")

        # Struct head
        vis = _vis(s.visibility)
        generics = self._emit_generic_params(s.template_params)
        where_clause = self._emit_where_clause(s.template_params)

        if not s.fields:
            parts.append(f"{i}{vis}struct {s.name}{generics}{where_clause};")
        else:
            parts.append(f"{i}{vis}struct {s.name}{generics}{where_clause} {{")
            self._indent_inc()
            for fld in s.fields:
                parts.append(self._emit_field(fld))
            self._indent_dec()
            parts.append(f"{i}}}")

        # Nested types
        for nested in s.nested_types:
            parts.append("")
            parts.append(self.emit_item(nested))

        return "\n".join(parts)

    def _emit_field(self, fld: IRField) -> str:
        i = self._i()
        comment = ""
        if fld.comment:
            comment = f"  // {fld.comment}"
        vis = _vis(fld.access)
        type_str = self.emit_type(fld.type)
        return f"{i}{vis}{fld.name}: {type_str},{comment}"

    # -----------------------------------------------------------------------
    # Enum
    # -----------------------------------------------------------------------

    def emit_enum(self, e: IREnum) -> str:
        parts: List[str] = []
        i = self._i()

        # Doc comment
        if e.comment:
            for line in e.comment.splitlines():
                parts.append(f"{i}/// {line}" if line.strip() else f"{i}///")

        # Repr attribute for enums with an underlying type
        if e.underlying_type:
            parts.append(f"{i}#[repr({self.emit_type(e.underlying_type)})]")

        # Derive attribute
        if e.derives:
            parts.append(f"{i}#[derive({', '.join(e.derives)})]")

        vis = _vis(e.visibility)
        parts.append(f"{i}{vis}enum {e.name} {{")
        self._indent_inc()
        # Track used discriminant values to avoid E0081 (duplicate values).
        seen_values: set = set()
        for variant in e.variants:
            if variant.value is not None and variant.value in seen_values:
                # Duplicate discriminant — emit without explicit value
                vi = self._i()
                parts.append(f"{vi}{variant.name}, // = {variant.value} (duplicate)")
            else:
                if variant.value is not None:
                    seen_values.add(variant.value)
                parts.append(self._emit_enum_variant(variant))
        self._indent_dec()
        parts.append(f"{i}}}")
        return "\n".join(parts)

    def _emit_enum_variant(self, v: IREnumVariant) -> str:
        i = self._i()
        if v.value is not None:
            return f"{i}{v.name} = {v.value},"
        return f"{i}{v.name},"

    # -----------------------------------------------------------------------
    # Trait
    # -----------------------------------------------------------------------

    def emit_trait(self, t: IRTrait) -> str:
        parts: List[str] = []
        i = self._i()

        # Doc comment
        if t.comment:
            for line in t.comment.splitlines():
                parts.append(f"{i}/// {line}" if line.strip() else f"{i}///")

        vis = _vis(t.visibility)
        generics = self._emit_generic_params(t.template_params)
        where_clause = self._emit_where_clause(t.template_params)

        supers = ""
        if t.supertraits:
            supers = ": " + " + ".join(t.supertraits)

        parts.append(f"{i}{vis}trait {t.name}{generics}{supers}{where_clause} {{")
        self._indent_inc()
        for method in t.methods:
            parts.append(self._emit_trait_method(method))
        self._indent_dec()
        parts.append(f"{i}}}")
        return "\n".join(parts)

    def _emit_trait_method(self, f: IRFunction) -> str:
        """Emit a method signature inside a trait block.

        If the method has a body it is emitted as a default implementation,
        otherwise it is emitted as a required method (signature only).
        """
        i = self._i()
        sig = self._function_signature(f, in_trait=True)
        if f.body is not None and f.body.statements:
            body = self._emit_block_body(f.body)
            return f"{i}{sig} {body}"
        return f"{i}{sig};"

    # -----------------------------------------------------------------------
    # Impl block
    # -----------------------------------------------------------------------

    def emit_impl(self, impl: IRImplBlock) -> str:
        parts: List[str] = []
        i = self._i()
        generics = self._emit_generic_params(impl.template_params)
        where_clause = self._emit_where_clause(impl.template_params)

        if impl.trait_name:
            parts.append(
                f"{i}impl{generics} {impl.trait_name} for "
                f"{impl.struct_name}{where_clause} {{"
            )
        else:
            parts.append(
                f"{i}impl{generics} {impl.struct_name}{where_clause} {{"
            )

        self._indent_inc()
        is_trait_impl = bool(impl.trait_name)
        for idx, method in enumerate(impl.methods):
            parts.append(self.emit_function(method, in_trait_impl=is_trait_impl))
            if idx < len(impl.methods) - 1:
                parts.append("")
        self._indent_dec()
        parts.append(f"{i}}}")
        return "\n".join(parts)

    # -----------------------------------------------------------------------
    # Function
    # -----------------------------------------------------------------------

    def emit_function(self, f: IRFunction, *, in_trait_impl: bool = False) -> str:
        parts: List[str] = []
        i = self._i()

        # Doc comment
        if f.comment:
            for line in f.comment.splitlines():
                parts.append(f"{i}/// {line}" if line.strip() else f"{i}///")

        # Attributes
        for attr in f.attributes:
            # Normalise: if the attr already has #[...] wrapping, use as-is
            if attr.startswith("#[") and attr.endswith("]"):
                parts.append(f"{i}{attr}")
            else:
                parts.append(f"{i}#[{attr}]")

        sig = self._function_signature(f, in_trait=in_trait_impl)

        if f.body is not None:
            # For constructors (fn new(...) -> Self), ensure the body
            # ends with a Self return if it doesn't already.
            is_constructor = (
                f.name == "new"
                and f.return_type is not None
                and f.return_type.name == "Self"
            )
            if is_constructor:
                body = self._emit_constructor_body(f)
            else:
                body = self._emit_block_body(f.body)
            parts.append(f"{i}{sig} {body}")
        elif f.is_pure_virtual:
            # Pure virtual methods have no body; they belong in traits.
            parts.append(f"{i}{sig};")
        else:
            # Declaration-only: emit a stub with todo!()
            parts.append(f"{i}{sig} {{")
            self._indent_inc()
            parts.append(f"{self._i()}todo!()")
            self._indent_dec()
            parts.append(f"{i}}}")

        return "\n".join(parts)

    def _function_signature(self, f: IRFunction, *, in_trait: bool = False) -> str:
        """Build the ``fn name(params) -> Ret`` signature string."""
        vis = "" if in_trait else _vis(f.access)
        unsafe_kw = "unsafe " if f.is_unsafe else ""
        generics = self._emit_generic_params(f.template_params)
        where_clause = self._emit_where_clause(f.template_params)

        # Parameters
        param_parts: List[str] = []
        if f.self_param:
            param_parts.append(f.self_param)
        for p in f.params:
            param_parts.append(f"{p.name}: {self.emit_type(p.type)}")
        params_str = ", ".join(param_parts)

        # Return type
        ret = ""
        if f.return_type and f.return_type.name not in ("()", "void", ""):
            ret = f" -> {self.emit_type(f.return_type)}"

        return f"{vis}{unsafe_kw}fn {f.name}{generics}({params_str}){ret}{where_clause}"

    # -----------------------------------------------------------------------
    # Type alias
    # -----------------------------------------------------------------------

    def emit_type_alias(self, ta: IRTypeAlias) -> str:
        i = self._i()
        vis = _vis(ta.visibility)
        generics = self._emit_generic_params(ta.template_params)
        target = self.emit_type(ta.target) if ta.target else "()"
        return f"{i}{vis}type {ta.name}{generics} = {target};"

    # -----------------------------------------------------------------------
    # Const / static
    # -----------------------------------------------------------------------

    def emit_const(self, c: IRConst) -> str:
        i = self._i()
        vis = _vis(c.visibility)
        type_str = self.emit_type(c.type) if c.type else "_"

        if c.value is not None:
            value = c.value
        else:
            # Use a zero/default value instead of todo!() which panics in
            # const/static context at compile time (E0080).
            value = self._const_default_value(type_str)

        if c.is_static:
            mut = "mut " if c.is_mutable else ""
            return f"{i}{vis}static {mut}{c.name}: {type_str} = {value};"
        return f"{i}{vis}const {c.name}: {type_str} = {value};"

    @staticmethod
    def _const_default_value(type_str: str) -> str:
        """Return a compile-time default value for a Rust type."""
        # Numeric types
        if type_str in ("i8", "i16", "i32", "i64", "i128", "isize",
                         "u8", "u16", "u32", "u64", "u128", "usize"):
            return "0"
        if type_str in ("f32", "f64"):
            return "0.0"
        if type_str == "bool":
            return "false"
        if type_str == "()":
            return "()"
        # Pointer types
        if type_str.startswith("*const "):
            return "std::ptr::null()"
        if type_str.startswith("*mut "):
            return "std::ptr::null_mut()"
        # Function pointer — can't have a const default easily
        if type_str.startswith("fn("):
            return "{ fn _stub() { panic!(\"uninitialized\") } _stub }"
        # Fallback: use 0 as a numeric default (most common case for
        # unresolved V8 constants), or comment out
        return "0 /* TODO: provide value */"

    # -----------------------------------------------------------------------
    # Macro
    # -----------------------------------------------------------------------

    def emit_macro(self, m: IRMacro) -> str:
        i = self._i()
        if m.is_const:
            # Simple value macro emitted as a constant
            type_str = self.emit_type(m.type) if m.type else "_"
            return f"{i}pub const {m.name}: {type_str} = {m.body};"

        if m.is_function_like:
            params = ", ".join(f"${p}:expr" for p in m.params)
            return (
                f"{i}macro_rules! {m.name} {{\n"
                f"{i}    ({params}) => {{\n"
                f"{i}        {m.body}\n"
                f"{i}    }};\n"
                f"{i}}}"
            )

        # Object-like macro without params
        return (
            f"{i}macro_rules! {m.name} {{\n"
            f"{i}    () => {{\n"
            f"{i}        {m.body}\n"
            f"{i}    }};\n"
            f"{i}}}"
        )

    # -----------------------------------------------------------------------
    # Type emission
    # -----------------------------------------------------------------------

    def emit_type(self, t: Optional[IRType]) -> str:
        """Render an IRType as a Rust type string."""
        if t is None:
            return "()"

        base = t.name

        # C++ `auto` → Rust type inference `_`
        if base == "auto":
            return "_"

        # Fixed-size array — handle before generic args to avoid [T<T>; N]
        if t.array_size is not None:
            # For arrays, don't append generic_args to the element type
            # since the element type name already contains the right info.
            return f"[{base}; {t.array_size}]"

        # Append generic arguments if not already baked into the name
        if t.generic_args and "<" not in base:
            args = ", ".join(self.emit_type(a) for a in t.generic_args)
            base = f"{base}<{args}>"

        # Wrap in Option if nullable
        if t.is_optional:
            base = f"Option<{base}>"

        # Reference / pointer wrappers
        lt = f"'{t.lifetime} " if t.lifetime else ""
        if t.is_mut_reference:
            base = f"&{lt}mut {base}"
        elif t.is_reference:
            base = f"&{lt}{base}"
        elif t.is_mut_pointer:
            base = f"*mut {base}"
        elif t.is_pointer:
            base = f"*const {base}"

        return base

    # -----------------------------------------------------------------------
    # Generics / where clauses
    # -----------------------------------------------------------------------

    def _emit_generic_params(self, params: List[IRTemplateParam]) -> str:
        if not params:
            return ""
        parts: List[str] = []
        for p in params:
            if p.is_type_param:
                parts.append(p.name)
            else:
                # const generic: const N: usize
                vt = self.emit_type(p.value_type) if p.value_type else "usize"
                parts.append(f"const {p.name}: {vt}")
        return "<" + ", ".join(parts) + ">"

    def _emit_where_clause(self, params: List[IRTemplateParam]) -> str:
        """Emit a ``where`` clause for constrained generic parameters."""
        clauses: List[str] = []
        for p in params:
            if p.is_type_param and p.constraint:
                clauses.append(f"{p.name}: {p.constraint}")
        if not clauses:
            return ""
        return "\nwhere\n    " + ",\n    ".join(clauses)

    # -----------------------------------------------------------------------
    # Statement emission
    # -----------------------------------------------------------------------

    def emit_stmt(self, stmt) -> str:
        """Emit a single IR statement."""
        if isinstance(stmt, IRBlock):
            return self._emit_block_stmt(stmt)
        if isinstance(stmt, IRVarDecl):
            return self._emit_var_decl(stmt)
        if isinstance(stmt, IRExprStmt):
            return self._emit_expr_stmt(stmt)
        if isinstance(stmt, IRReturnStmt):
            return self._emit_return_stmt(stmt)
        if isinstance(stmt, IRIfStmt):
            return self._emit_if_stmt(stmt)
        if isinstance(stmt, IRMatchStmt):
            return self._emit_match_stmt(stmt)
        if isinstance(stmt, IRForLoop):
            return self._emit_for_loop(stmt)
        if isinstance(stmt, IRForRangeLoop):
            return self._emit_for_range_loop(stmt)
        if isinstance(stmt, IRWhileLoop):
            return self._emit_while_loop(stmt)
        if isinstance(stmt, IRBreakStmt):
            return f"{self._i()}break;"
        if isinstance(stmt, IRContinueStmt):
            return f"{self._i()}continue;"
        if isinstance(stmt, IRThrowStmt):
            return self._emit_throw_stmt(stmt)
        if isinstance(stmt, IRTryCatchStmt):
            return self._emit_try_catch(stmt)
        if isinstance(stmt, IRUnsafeBlock):
            return self._emit_unsafe_block(stmt)
        if isinstance(stmt, IRRawStmt):
            return self._emit_raw_stmt(stmt)
        return f"{self._i()}// TODO: unhandled statement {type(stmt).__name__}"

    # -- constructor body helper ---------------------------------------------

    def _emit_constructor_body(self, f: IRFunction) -> str:
        """Emit a constructor body that returns Self.

        If the body already ends with a return statement, use it as-is.
        Otherwise, append ``Self { ..Default::default() }`` or ``todo!()``
        to ensure the function returns Self.
        """
        if not f.body or not f.body.statements:
            return "{\n        todo!(\"constructor\")\n    }"

        # Check if body already ends with a return
        last_stmt = f.body.statements[-1]
        has_return = isinstance(last_stmt, IRReturnStmt)

        if has_return:
            return self._emit_block_body(f.body)

        # Emit body statements, then add a Self return
        lines = ["{"]
        self._indent_inc()
        for stmt in f.body.statements:
            lines.append(self.emit_stmt(stmt))
        # Add default Self return
        lines.append(f"{self._i()}todo!(\"constructor: return Self\")")
        self._indent_dec()
        lines.append(f"{self._i()}}}")
        return "\n".join(lines)

    # -- block ---------------------------------------------------------------

    def _emit_block_body(self, block: IRBlock) -> str:
        """Emit the *contents* of a block including braces, at the current indent."""
        if not block.statements:
            return "{}"
        lines: List[str] = ["{"]
        self._indent_inc()
        for stmt in block.statements:
            lines.append(self.emit_stmt(stmt))
        self._indent_dec()
        lines.append(f"{self._i()}}}")
        return "\n".join(lines)

    def _emit_block_stmt(self, block: IRBlock) -> str:
        """Emit a block that appears as a statement."""
        i = self._i()
        if not block.statements:
            return f"{i}{{}}"
        parts: List[str] = [f"{i}{{"]
        self._indent_inc()
        for stmt in block.statements:
            parts.append(self.emit_stmt(stmt))
        self._indent_dec()
        parts.append(f"{self._i()}}}")
        return "\n".join(parts)

    # -- variable declaration ------------------------------------------------

    def _emit_var_decl(self, v: IRVarDecl) -> str:
        i = self._i()
        mut = "mut " if v.is_mutable else ""

        type_ann = ""
        if v.type and v.type.name:
            type_ann = f": {self.emit_type(v.type)}"

        if v.is_static:
            # static variable inside a function — rare in Rust, emit a comment
            type_str = self.emit_type(v.type) if v.type else "_"
            val = self.emit_expr(v.initializer) if v.initializer else "todo!()"
            return f"{i}static {mut}{v.name}: {type_str} = {val};"

        if v.initializer is not None:
            init = self.emit_expr(v.initializer)
            return f"{i}let {mut}{v.name}{type_ann} = {init};"
        return f"{i}let {mut}{v.name}{type_ann};"

    # -- expression statement ------------------------------------------------

    def _emit_expr_stmt(self, es: IRExprStmt) -> str:
        if es.expr is None:
            return ""
        i = self._i()
        expr_text = self.emit_expr(es.expr)
        return f"{i}{expr_text};"

    # -- return --------------------------------------------------------------

    def _emit_return_stmt(self, r: IRReturnStmt) -> str:
        i = self._i()
        if r.value is not None:
            return f"{i}return {self.emit_expr(r.value)};"
        return f"{i}return;"

    # -- if / else -----------------------------------------------------------

    def _emit_if_stmt(self, s: IRIfStmt) -> str:
        i = self._i()
        cond = self.emit_expr(s.condition) if s.condition else "true"
        then_body = self._emit_block_body(s.then_block) if s.then_block else "{}"

        parts: List[str] = [f"{i}if {cond} {then_body}"]

        if s.else_block is not None:
            # else-if chain: the else_block contains a single IRIfStmt
            if (
                isinstance(s.else_block, IRBlock)
                and len(s.else_block.statements) == 1
                and isinstance(s.else_block.statements[0], IRIfStmt)
            ):
                inner = s.else_block.statements[0]
                # Recursively emit the else-if, then splice
                else_if_text = self._emit_if_stmt(inner)
                # Strip the leading indentation from the recursive call so it
                # flows after ``} else ``
                else_if_text = else_if_text.lstrip()
                parts[-1] += f" else {else_if_text}"
            else:
                else_body = self._emit_block_body(s.else_block)
                parts[-1] += f" else {else_body}"

        return "\n".join(parts)

    # -- match ---------------------------------------------------------------

    def _emit_match_stmt(self, m: IRMatchStmt) -> str:
        i = self._i()
        expr = self.emit_expr(m.expr) if m.expr else "_"
        parts: List[str] = [f"{i}match {expr} {{"]
        self._indent_inc()
        for arm in m.arms:
            parts.append(self._emit_match_arm(arm))
        self._indent_dec()
        parts.append(f"{i}}}")
        return "\n".join(parts)

    def _emit_match_arm(self, arm: IRMatchArm) -> str:
        i = self._i()
        if arm.is_default:
            pattern = "_"
        elif arm.patterns:
            pattern = " | ".join(arm.patterns)
        else:
            pattern = "_"

        if arm.body and arm.body.statements:
            body = self._emit_block_body(arm.body)
            return f"{i}{pattern} => {body}"
        return f"{i}{pattern} => {{}}"

    # -- for loop (C-style) --------------------------------------------------

    def _emit_for_loop(self, f: IRForLoop) -> str:
        """Emit a C-style for loop.

        We attempt to detect simple ``for (int i = 0; i < n; i++)`` patterns
        and emit a Rust ``for i in 0..n`` range.  Otherwise we fall back to a
        ``while`` loop with the init before it and the increment at the end of
        the body.
        """
        i = self._i()
        range_loop = self._try_range_pattern(f)
        if range_loop is not None:
            return range_loop

        # Fallback: init; while cond { body; incr }
        parts: List[str] = []
        if f.init:
            parts.append(self.emit_stmt(f.init))

        cond = self.emit_expr(f.condition) if f.condition else "true"
        body_lines: List[str] = []
        if f.body:
            self._indent_inc()
            for stmt in f.body.statements:
                body_lines.append(self.emit_stmt(stmt))
            if f.increment:
                body_lines.append(f"{self._i()}{self.emit_expr(f.increment)};")
            self._indent_dec()

        parts.append(f"{i}while {cond} {{")
        parts.extend(body_lines)
        parts.append(f"{i}}}")
        return "\n".join(parts)

    def _try_range_pattern(self, f: IRForLoop) -> Optional[str]:
        """Try to detect ``for i in start..end`` from a C-style for loop."""
        # We need init (var decl), condition (< or <=), and simple increment
        if not (
            isinstance(f.init, IRVarDecl)
            and f.init.initializer is not None
            and isinstance(f.condition, IRBinaryExpr)
            and f.condition.op in (BinaryOp.LT, BinaryOp.LE)
        ):
            return None

        var_name = f.init.name
        start = self.emit_expr(f.init.initializer)
        end = self.emit_expr(f.condition.right) if f.condition.right else "0"
        range_op = ".." if f.condition.op == BinaryOp.LT else "..="
        i = self._i()
        body = self._emit_block_body(f.body) if f.body else "{}"
        return f"{i}for {var_name} in {start}{range_op}{end} {body}"

    # -- for-range loop ------------------------------------------------------

    def _emit_for_range_loop(self, f: IRForRangeLoop) -> str:
        i = self._i()
        iter_expr = self.emit_expr(f.iterable) if f.iterable else "todo!()"
        if f.is_mut_ref:
            iter_expr = f"{iter_expr}.iter_mut()"
        elif f.is_ref:
            iter_expr = f"{iter_expr}.iter()"
        body = self._emit_block_body(f.body) if f.body else "{}"
        return f"{i}for {f.variable} in {iter_expr} {body}"

    # -- while / loop --------------------------------------------------------

    def _emit_while_loop(self, w: IRWhileLoop) -> str:
        i = self._i()
        body_block = w.body if w.body else IRBlock(statements=[])

        if w.is_do_while:
            # loop { body; if !cond { break; } }
            parts: List[str] = [f"{i}loop {{"]
            self._indent_inc()
            for stmt in body_block.statements:
                parts.append(self.emit_stmt(stmt))
            cond = self.emit_expr(w.condition) if w.condition else "false"
            parts.append(f"{self._i()}if !({cond}) {{")
            self._indent_inc()
            parts.append(f"{self._i()}break;")
            self._indent_dec()
            parts.append(f"{self._i()}}}")
            self._indent_dec()
            parts.append(f"{i}}}")
            return "\n".join(parts)

        if w.condition is None:
            # Infinite loop
            body = self._emit_block_body(body_block)
            return f"{i}loop {body}"

        cond = self.emit_expr(w.condition)
        body = self._emit_block_body(body_block)
        return f"{i}while {cond} {body}"

    # -- throw ---------------------------------------------------------------

    def _emit_throw_stmt(self, t: IRThrowStmt) -> str:
        i = self._i()
        if t.expr is not None:
            return f"{i}return Err({self.emit_expr(t.expr)});"
        return f"{i}return Err(());"

    # -- try / catch ---------------------------------------------------------

    def _emit_try_catch(self, tc: IRTryCatchStmt) -> str:
        """Emit try/catch as a match on the result of a closure.

        This is a best-effort mapping; real code usually needs manual review.
        """
        i = self._i()
        parts: List[str] = []
        parts.append(f"{i}// TODO: review try/catch translation")
        parts.append(f"{i}match (|| -> Result<_, _> {{")
        if tc.try_block:
            self._indent_inc()
            for stmt in tc.try_block.statements:
                parts.append(self.emit_stmt(stmt))
            parts.append(f"{self._i()}Ok(())")
            self._indent_dec()
        parts.append(f"{i}}})() {{")
        self._indent_inc()
        parts.append(f"{self._i()}Ok(_) => {{}}")
        for cb in tc.catch_blocks:
            parts.append(self._emit_catch_block(cb))
        self._indent_dec()
        parts.append(f"{i}}}")
        return "\n".join(parts)

    def _emit_catch_block(self, cb: IRCatchBlock) -> str:
        i = self._i()
        if cb.exception_type:
            type_str = self.emit_type(cb.exception_type)
            var = cb.variable_name or "_"
            pattern = f"Err({var})"
            comment = f"  // {type_str}"
        else:
            pattern = "Err(_)"
            comment = ""
        body = self._emit_block_body(cb.body) if cb.body else "{}"
        return f"{i}{pattern} => {body}{comment}"

    # -- unsafe block --------------------------------------------------------

    def _emit_unsafe_block(self, u: IRUnsafeBlock) -> str:
        i = self._i()
        body = self._emit_block_body(u.body) if u.body else "{}"
        return f"{i}unsafe {body}"

    # -- raw statement -------------------------------------------------------

    def _emit_raw_stmt(self, r: IRRawStmt) -> str:
        i = self._i()
        # Emit raw C++ as a comment followed by todo!() so the code compiles
        source = (r.cpp_source or "").replace('"', r'\"').replace('\n', ' ')
        if len(source) > 100:
            source = source[:100] + "..."
        comment = r.comment or "raw C++"
        return f'{i}todo!("{comment}: {source}"); // raw C++ statement'

    # -----------------------------------------------------------------------
    # Expression emission
    # -----------------------------------------------------------------------

    def emit_expr(self, expr) -> str:
        """Emit a single IR expression."""
        if expr is None:
            return "()"

        if isinstance(expr, IRLiteral):
            return self._emit_literal(expr)
        if isinstance(expr, IRNameRef):
            return expr.name
        if isinstance(expr, IRBinaryExpr):
            return self._emit_binary_expr(expr)
        if isinstance(expr, IRUnaryExpr):
            return self._emit_unary_expr(expr)
        if isinstance(expr, IRCallExpr):
            return self._emit_call_expr(expr)
        if isinstance(expr, IRMemberExpr):
            return self._emit_member_expr(expr)
        if isinstance(expr, IRIndexExpr):
            return self._emit_index_expr(expr)
        if isinstance(expr, IRCastExpr):
            return self._emit_cast_expr(expr)
        if isinstance(expr, IRTernaryExpr):
            return self._emit_ternary_expr(expr)
        if isinstance(expr, IRNewExpr):
            return self._emit_new_expr(expr)
        if isinstance(expr, IRDeleteExpr):
            return self._emit_delete_expr(expr)
        if isinstance(expr, IRSizeofExpr):
            return self._emit_sizeof_expr(expr)
        if isinstance(expr, IRLambdaExpr):
            return self._emit_lambda_expr(expr)
        if isinstance(expr, IRThisExpr):
            return "self"
        if isinstance(expr, IRInitListExpr):
            return self._emit_init_list_expr(expr)
        if isinstance(expr, IRRawExpr):
            return self._emit_raw_expr(expr)
        return f"/* TODO: unhandled expr {type(expr).__name__} */"

    # -- literal -------------------------------------------------------------

    def _emit_literal(self, lit: IRLiteral) -> str:
        if lit.kind == LiteralKind.NULL:
            return "None"
        if lit.kind == LiteralKind.BOOL:
            return lit.value.lower()
        if lit.kind == LiteralKind.STRING:
            # Ensure the string is properly quoted
            val = lit.value
            if not (val.startswith('"') and val.endswith('"')):
                val = f'"{val}"'
            return val
        if lit.kind == LiteralKind.CHAR:
            val = lit.value
            if not (val.startswith("'") and val.endswith("'")):
                val = f"'{val}'"
            return val
        # INT / FLOAT — strip C++ literal suffixes (ULL, LL, UL, U, u, L, f, F)
        val = lit.value
        if lit.kind == LiteralKind.INT:
            val = re.sub(r'(?i)(?:ULL|LLU|LL|UL|LU|L|U)$', '', val)
        elif lit.kind == LiteralKind.FLOAT:
            val = re.sub(r'(?i)[fFlL]$', '', val)
        return val

    # -- binary expression ---------------------------------------------------

    def _emit_binary_expr(self, b: IRBinaryExpr) -> str:
        left = self.emit_expr(b.left)
        right = self.emit_expr(b.right)

        if b.op == BinaryOp.COMMA:
            # C++ comma operator: emit both as separate expressions in a block
            return f"{{ {left}; {right} }}"

        op = b.op.value
        return f"{left} {op} {right}"

    # -- unary expression ----------------------------------------------------

    def _emit_unary_expr(self, u: IRUnaryExpr) -> str:
        operand = self.emit_expr(u.operand)

        if u.op == UnaryOp.PRE_INC:
            return f"{{ {operand} += 1; {operand} }}"
        if u.op == UnaryOp.PRE_DEC:
            return f"{{ {operand} -= 1; {operand} }}"
        if u.op == UnaryOp.POST_INC:
            return f"{{ let _old = {operand}; {operand} += 1; _old }}"
        if u.op == UnaryOp.POST_DEC:
            return f"{{ let _old = {operand}; {operand} -= 1; _old }}"
        if u.op == UnaryOp.ADDR_OF_MUT:
            return f"&mut {operand}"

        op = u.op.value
        return f"{op}{operand}"

    # -- call expression -----------------------------------------------------

    # V8 macros → Rust equivalents (applied at emit time)
    _V8_MACRO_MAP = {
        # Assertions (original case)
        "DCHECK": "debug_assert!",
        "DCHECK_EQ": "debug_assert_eq!",
        "DCHECK_NE": "debug_assert_ne!",
        "CHECK": "assert!",
        "CHECK_EQ": "assert_eq!",
        "CHECK_NE": "assert_ne!",
        "SLOW_DCHECK": "debug_assert!",
        "FATAL": "panic!",
        "V8_Fatal": "panic!",
        "V8_Dcheck": "debug_assert!",
        # snake_case variants (from name converter)
        "dcheck": "debug_assert!",
        "dcheck_eq": "debug_assert_eq!",
        "dcheck_ne": "debug_assert_ne!",
        "check": "assert!",
        "check_eq": "assert_eq!",
        "check_ne": "assert_ne!",
        "slow_dcheck": "debug_assert!",
        "fatal": "panic!",
        "v8_fatal": "panic!",
        "v8_dcheck": "debug_assert!",
        # Control flow
        "UNREACHABLE": "unreachable!",
        "UNIMPLEMENTED": "unimplemented!",
        "unreachable": "unreachable!",
        "unimplemented": "unimplemented!",
        # Variable usage
        "USE": None,  # special handling
        "STATIC_ASSERT": None,  # special handling
        # Printing
        "PrintF": "print!",
        "SNPrintF": "write!",
        "print_f": "print!",
        "s_n_print_f": "write!",
    }
    # Comparison macros: DCHECK_LT(a,b) → debug_assert!(a < b)
    _V8_CMP_MACRO_MAP = {
        "DCHECK_LT": ("<", "debug_assert!"),
        "DCHECK_LE": ("<=", "debug_assert!"),
        "DCHECK_GT": (">", "debug_assert!"),
        "DCHECK_GE": (">=", "debug_assert!"),
        "DCHECK_NOT_NULL": None,  # special
        "DCHECK_NULL": None,  # special
        "DCHECK_IMPLIES": None,  # special
        "CHECK_LT": ("<", "assert!"),
        "CHECK_LE": ("<=", "assert!"),
        "CHECK_GT": (">", "assert!"),
        "CHECK_GE": (">=", "assert!"),
        "CHECK_NOT_NULL": None,  # special
        # snake_case variants
        "dcheck_lt": ("<", "debug_assert!"),
        "dcheck_le": ("<=", "debug_assert!"),
        "dcheck_gt": (">", "debug_assert!"),
        "dcheck_ge": (">=", "debug_assert!"),
        "dcheck_not_null": None,
        "dcheck_null": None,
        "dcheck_implies": None,
        "check_lt": ("<", "assert!"),
        "check_le": ("<=", "assert!"),
        "check_gt": (">", "assert!"),
        "check_ge": (">=", "assert!"),
        "check_not_null": None,
    }

    def _emit_call_expr(self, c: IRCallExpr) -> str:
        func = self.emit_expr(c.function)
        args_exprs = [self.emit_expr(a) for a in c.args]

        # Handle V8 macros
        if func in self._V8_MACRO_MAP:
            rust_macro = self._V8_MACRO_MAP[func]
            if func == "USE":
                if args_exprs:
                    return f"let _ = {args_exprs[0]}"
                return "/* USE() */"
            if func == "STATIC_ASSERT":
                args = ", ".join(args_exprs)
                return f"const _: () = assert!({args})"
            if func in ("UNREACHABLE", "UNIMPLEMENTED") and not args_exprs:
                return f"{rust_macro}()"
            args = ", ".join(args_exprs)
            return f"{rust_macro}({args})"

        # Handle V8 comparison macros
        if func in self._V8_CMP_MACRO_MAP:
            entry = self._V8_CMP_MACRO_MAP[func]
            if entry is not None:
                op, macro = entry
                if len(args_exprs) >= 2:
                    return f"{macro}({args_exprs[0]} {op} {args_exprs[1]})"
            else:
                # Special comparison macros
                func_upper = func.upper()
                if func_upper in ("DCHECK_NOT_NULL", "CHECK_NOT_NULL") and args_exprs:
                    macro = "debug_assert!" if "DCHECK" in func_upper else "assert!"
                    return f"{macro}(!{args_exprs[0]}.is_null())"
                if func_upper == "DCHECK_NULL" and args_exprs:
                    return f"debug_assert!({args_exprs[0]}.is_null())"
                if func_upper == "DCHECK_IMPLIES" and len(args_exprs) >= 2:
                    return f"debug_assert!(!{args_exprs[0]} || {args_exprs[1]})"
            args = ", ".join(args_exprs)
            return f"debug_assert!({args})"

        # Handle C++ math / utility functions
        _MATH_MAP = {
            "isnan": "f64::is_nan",
            "isfinite": "f64::is_finite",
            "isinf": "f64::is_infinite",
            "quiet_NaN": "f64::NAN",  # no-arg → constant
            "infinity": "f64::INFINITY",
            "std::isnan": "f64::is_nan",
            "std::isfinite": "f64::is_finite",
            "std::isinf": "f64::is_infinite",
            "abs": "i32::abs",
            "fabs": "f64::abs",
            "floor": "f64::floor",
            "ceil": "f64::ceil",
            "sqrt": "f64::sqrt",
            "pow": "f64::powf",
            "log": "f64::ln",
            "log2": "f64::log2",
            "log10": "f64::log10",
            "exp": "f64::exp",
            "round": "f64::round",
            "trunc": "f64::trunc",
            "max": "std::cmp::max",
            "min": "std::cmp::min",
            "std::max": "std::cmp::max",
            "std::min": "std::cmp::min",
        }
        if func in _MATH_MAP:
            mapped = _MATH_MAP[func]
            if not args_exprs and func in ("quiet_NaN", "infinity"):
                return mapped  # constant, not a function call
            if args_exprs and mapped.startswith("f64::") and mapped != "f64::powf":
                # Method-style: f64::is_nan(x) → x.is_nan()
                method = mapped.split("::")[1]
                return f"{args_exprs[0]}.{method}({', '.join(args_exprs[1:])})"
            args = ", ".join(args_exprs)
            return f"{mapped}({args})"

        # Handle __builtin_expect(expr, val) → just expr
        if func == "__builtin_expect":
            if args_exprs:
                return args_exprs[0]
            return "false"

        # Handle V8_LIKELY / V8_UNLIKELY → just the inner expression
        if func in ("V8_LIKELY", "V8_UNLIKELY", "LIKELY", "UNLIKELY"):
            if args_exprs:
                return args_exprs[0]
            return "true"

        # Handle C++ operator function names that leak through
        if func.startswith("operator"):
            op = func[len("operator"):].strip()
            if op == "++" and args_exprs:
                return f"{{ {args_exprs[0]} += 1; {args_exprs[0]} }}"
            if op == "--" and args_exprs:
                return f"{{ {args_exprs[0]} -= 1; {args_exprs[0]} }}"
            if op == "()" and args_exprs:
                # operator() → Fn call
                rest = ", ".join(args_exprs[1:]) if len(args_exprs) > 1 else ""
                return f"{args_exprs[0]}({rest})"

        # Turbofish generic arguments
        if c.generic_args:
            args_str = ", ".join(self.emit_type(a) for a in c.generic_args)
            func = f"{func}::<{args_str}>"

        args = ", ".join(args_exprs)
        return f"{func}({args})"

    # -- member expression ---------------------------------------------------

    def _emit_member_expr(self, m: IRMemberExpr) -> str:
        member = m.member or ""
        if not member:
            # Empty member name — emit object alone to avoid `obj.;` syntax errors
            if m.object is not None:
                return self.emit_expr(m.object)
            return "self"
        if m.object is None:
            # Implicit `this->member` in C++ → `self.member` in Rust
            return f"self.{member}"
        obj = self.emit_expr(m.object)
        # If the object resolved to "()" it was likely an implicit this
        if obj == "()":
            return f"self.{member}"
        # In Rust, both . and -> become .
        return f"{obj}.{member}"

    # -- index expression ----------------------------------------------------

    def _emit_index_expr(self, ix: IRIndexExpr) -> str:
        obj = self.emit_expr(ix.object)
        idx = self.emit_expr(ix.index)
        return f"{obj}[{idx}]"

    # -- cast expression -----------------------------------------------------

    def _emit_cast_expr(self, c: IRCastExpr) -> str:
        expr = self.emit_expr(c.expr)
        target = self.emit_type(c.target_type) if c.target_type else "_"

        if c.kind == CastKind.AS:
            return f"{expr} as {target}"
        if c.kind == CastKind.INTO:
            return f"{expr}.into()"
        if c.kind == CastKind.FROM:
            return f"{target}::from({expr})"
        if c.kind == CastKind.UNSAFE_TRANSMUTE:
            return f"unsafe {{ std::mem::transmute::<_, {target}>({expr}) }}"
        if c.kind == CastKind.UNSAFE_PTR_CAST:
            return f"{expr} as {target}"

        return f"{expr} as {target}"

    # -- ternary → if expression ---------------------------------------------

    def _emit_ternary_expr(self, t: IRTernaryExpr) -> str:
        cond = self.emit_expr(t.condition)
        then = self.emit_expr(t.then_expr)
        els = self.emit_expr(t.else_expr)
        return f"if {cond} {{ {then} }} else {{ {els} }}"

    # -- new → Box::new / Vec ------------------------------------------------

    def _emit_new_expr(self, n: IRNewExpr) -> str:
        type_str = self.emit_type(n.type) if n.type else "_"
        if n.is_array:
            size = self.emit_expr(n.array_size) if n.array_size else "0"
            return f"vec![{type_str}::default(); {size}]"
        args = ", ".join(self.emit_expr(a) for a in n.args)
        if args:
            return f"Box::new({type_str}::new({args}))"
        return f"Box::new({type_str}::new())"

    # -- delete → drop -------------------------------------------------------

    def _emit_delete_expr(self, d: IRDeleteExpr) -> str:
        expr = self.emit_expr(d.expr)
        return f"drop({expr})"

    # -- sizeof / alignof ----------------------------------------------------

    def _emit_sizeof_expr(self, s: IRSizeofExpr) -> str:
        if s.is_alignof:
            if s.target_type:
                return f"std::mem::align_of::<{self.emit_type(s.target_type)}>()"
            if s.target_expr:
                return f"std::mem::align_of_val(&{self.emit_expr(s.target_expr)})"
            return "std::mem::align_of::<()>()"

        if s.target_type:
            return f"std::mem::size_of::<{self.emit_type(s.target_type)}>()"
        if s.target_expr:
            return f"std::mem::size_of_val(&{self.emit_expr(s.target_expr)})"
        return "std::mem::size_of::<()>()"

    # -- lambda → closure ----------------------------------------------------

    def _emit_lambda_expr(self, lam: IRLambdaExpr) -> str:
        params = ", ".join(
            f"{p.name}: {self.emit_type(p.type)}" for p in lam.params
        )

        ret = ""
        if lam.return_type and lam.return_type.name not in ("()", "void", ""):
            ret = f" -> {self.emit_type(lam.return_type)}"

        capture = "move " if lam.capture_mode == "move" else ""

        if lam.body and lam.body.statements:
            # Multi-statement closure
            body_text = self._emit_block_body(lam.body)
            return f"{capture}|{params}|{ret} {body_text}"

        return f"{capture}|{params}|{ret} {{}}"

    # -- init list -----------------------------------------------------------

    def _emit_init_list_expr(self, il: IRInitListExpr) -> str:
        elements = ", ".join(self.emit_expr(e) for e in il.elements)
        return f"[{elements}]"

    # -- raw expression ------------------------------------------------------

    def _emit_raw_expr(self, r: IRRawExpr) -> str:
        # Emit as todo!() — always valid Rust syntax, unlike /* comments */
        # which break when inside call arguments or expressions.
        source = (r.cpp_source or "").replace('"', r'\"').replace('\n', ' ')
        if len(source) > 100:
            source = source[:100] + "..."
        comment = r.comment or "manually translate"
        return f'todo!("{comment}: {source}")'

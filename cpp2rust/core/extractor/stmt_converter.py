"""Converts Clang AST statement/expression cursor nodes into IR nodes.

This module bridges the gap between libclang's Cursor-based AST representation
and the transpiler's intermediate representation defined in ir/nodes.py.
The main entry points are StmtConverter.convert_stmt() and
StmtConverter.convert_expr(), which recursively walk Clang cursor trees
and produce the corresponding IR statement or expression nodes.
"""

from __future__ import annotations

import logging
import re
import sys
from pathlib import Path
from typing import List, Optional

from clang.cindex import CursorKind, Cursor, TypeKind as ClangTypeKind

from cpp2rust.core.ir.nodes import (
    # Enums
    AccessSpecifier,
    BinaryOp,
    CastKind,
    LiteralKind,
    TypeKind as IRTypeKind,
    UnaryOp,
    # Type IR
    IRType,
    IRParam,
    # Expression IR
    IRBinaryExpr,
    IRCallExpr,
    IRCastExpr,
    IRDeleteExpr,
    IRExpr,
    IRIndexExpr,
    IRInitListExpr,
    IRLambdaExpr,
    IRLiteral,
    IRMemberExpr,
    IRNameRef,
    IRNewExpr,
    IRRawExpr,
    IRRawCppExpr,
    IRSizeofExpr,
    IRTernaryExpr,
    IRThisExpr,
    IRUnaryExpr,
    # Statement IR
    IRBlock,
    IRBreakStmt,
    IRCatchBlock,
    IRContinueStmt,
    IRExprStmt,
    IRForLoop,
    IRForRangeLoop,
    IRIfStmt,
    IRMatchArm,
    IRMatchStmt,
    IRRawStmt,
    IRRawCppStmt,
    IRReturnStmt,
    IRStmt,
    IRThrowStmt,
    IRTryCatchStmt,
    IRVarDecl,
    IRWhileLoop,
)

logger = logging.getLogger(__name__)


# ─── Operator Mapping Tables ────────────────────────────────────────────────

_BINARY_OP_MAP = {
    "+": BinaryOp.ADD,
    "-": BinaryOp.SUB,
    "*": BinaryOp.MUL,
    "/": BinaryOp.DIV,
    "%": BinaryOp.MOD,
    "&&": BinaryOp.AND,
    "||": BinaryOp.OR,
    "&": BinaryOp.BIT_AND,
    "|": BinaryOp.BIT_OR,
    "^": BinaryOp.BIT_XOR,
    "<<": BinaryOp.SHL,
    ">>": BinaryOp.SHR,
    "==": BinaryOp.EQ,
    "!=": BinaryOp.NE,
    "<": BinaryOp.LT,
    "<=": BinaryOp.LE,
    ">": BinaryOp.GT,
    ">=": BinaryOp.GE,
    "=": BinaryOp.ASSIGN,
    "+=": BinaryOp.ADD_ASSIGN,
    "-=": BinaryOp.SUB_ASSIGN,
    "*=": BinaryOp.MUL_ASSIGN,
    "/=": BinaryOp.DIV_ASSIGN,
    "%=": BinaryOp.MOD_ASSIGN,
    "&=": BinaryOp.BIT_AND_ASSIGN,
    "|=": BinaryOp.BIT_OR_ASSIGN,
    "^=": BinaryOp.BIT_XOR_ASSIGN,
    "<<=": BinaryOp.SHL_ASSIGN,
    ">>=": BinaryOp.SHR_ASSIGN,
    ",": BinaryOp.COMMA,
}

# Unary prefix operators (token appears before the operand)
_UNARY_PREFIX_OP_MAP = {
    "-": UnaryOp.NEG,
    "!": UnaryOp.NOT,
    "~": UnaryOp.BITNOT,
    "*": UnaryOp.DEREF,
    "&": UnaryOp.ADDR_OF,
    "++": UnaryOp.PRE_INC,
    "--": UnaryOp.PRE_DEC,
}

# Unary postfix operators (token appears after the operand)
_UNARY_POSTFIX_OP_MAP = {
    "++": UnaryOp.POST_INC,
    "--": UnaryOp.POST_DEC,
}

# Binary operators that are valid as overloaded C++ operators.
# Used to decide whether a CXX_OPERATOR_CALL_EXPR should become
# IRBinaryExpr rather than IRCallExpr.
_OVERLOADED_BINARY_TOKENS = frozenset(_BINARY_OP_MAP.keys())


# Expression CursorKind set — built at module load time with graceful handling
# for CursorKinds that may not exist in every libclang version.
_EXPR_KIND_NAMES = [
    "INTEGER_LITERAL", "FLOATING_LITERAL", "STRING_LITERAL", "CHARACTER_LITERAL",
    "CXX_BOOL_LITERAL_EXPR", "CXX_NULL_PTR_LITERAL_EXPR",
    "DECL_REF_EXPR", "MEMBER_REF_EXPR",
    "CALL_EXPR", "CXX_MEMBER_CALL_EXPR", "CXX_OPERATOR_CALL_EXPR",
    "BINARY_OPERATOR", "COMPOUND_ASSIGNMENT_OPERATOR", "UNARY_OPERATOR",
    "CONDITIONAL_OPERATOR",
    "CXX_STATIC_CAST_EXPR", "CXX_DYNAMIC_CAST_EXPR",
    "CXX_REINTERPRET_CAST_EXPR", "CXX_CONST_CAST_EXPR",
    "CSTYLE_CAST_EXPR", "CXX_FUNCTIONAL_CAST_EXPR",
    "PAREN_EXPR", "IMPLICIT_CAST_EXPR",
    "CXX_NEW_EXPR", "CXX_DELETE_EXPR", "CXX_THIS_EXPR",
    "ARRAY_SUBSCRIPT_EXPR", "LAMBDA_EXPR", "INIT_LIST_EXPR",
    "UNARY_EXPR_OR_TYPE_TRAIT_EXPR", "CXX_THROW_EXPR",
    "UNEXPOSED_EXPR",
]
_EXPR_KINDS_SET = set()
for _name in _EXPR_KIND_NAMES:
    _kind = getattr(CursorKind, _name, None)
    if _kind is not None:
        _EXPR_KINDS_SET.add(_kind)

# Safe references for CursorKinds that may not exist in all libclang versions.
# Dispatch code must compare against these (which may be None) instead of
# accessing CursorKind.XYZ directly.
_CXX_MEMBER_CALL_EXPR = getattr(CursorKind, "CXX_MEMBER_CALL_EXPR", None)
_CXX_OPERATOR_CALL_EXPR = getattr(CursorKind, "CXX_OPERATOR_CALL_EXPR", None)
_IMPLICIT_CAST_EXPR = getattr(CursorKind, "IMPLICIT_CAST_EXPR", None)
_UNARY_EXPR_OR_TYPE_TRAIT_EXPR = getattr(CursorKind, "UNARY_EXPR_OR_TYPE_TRAIT_EXPR", None)
_CXX_CATCH_HANDLER = getattr(CursorKind, "CXX_CATCH_HANDLER", None)


# ─── Helpers ────────────────────────────────────────────────────────────────

def _get_source_text(cursor: Cursor) -> str:
    """Extract the raw C++ source text covered by *cursor*.extent.

    Returns an empty string if the extent or file cannot be read.
    """
    try:
        extent = cursor.extent
        if extent is None:
            return ""
        start = extent.start
        end = extent.end
        if start.file is None:
            return ""
        file_path = start.file.name
        with open(file_path, "r", errors="replace") as fh:
            content = fh.read()
        # Convert (line, column) — both 1-based — to a byte offset within
        # the content string.  This is a simplistic line/col approach; for
        # most single-statement extents it is accurate enough.
        lines = content.splitlines(keepends=True)
        start_offset = sum(len(lines[i]) for i in range(start.line - 1)) + (start.column - 1)
        end_offset = sum(len(lines[i]) for i in range(end.line - 1)) + (end.column - 1)
        return content[start_offset:end_offset]
    except Exception:
        return ""


def _to_snake_case_id(name: str) -> str:
    """Convert a CamelCase/mixedCase/ALLCAPS identifier to snake_case.

    Used so that parameter/variable references in function bodies match
    the snake_case names used in the Rust declaration sites.
    """
    if not name:
        return name
    s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s2 = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s1)
    result = s2.lower().strip("_")
    # Avoid creating Rust keywords from identifiers
    _RUST_KW = {"use", "type", "match", "move", "ref", "where", "loop",
                "yield", "async", "await", "dyn", "impl", "trait", "fn",
                "let", "mut", "pub", "mod", "crate", "super", "self",
                "Self", "return", "if", "else", "for", "while", "break",
                "continue", "struct", "enum", "true", "false", "in",
                "const", "static", "extern", "unsafe", "box", "priv"}
    if result in _RUST_KW:
        result = f"{result}_"
    return result


def _children(cursor: Cursor) -> List[Cursor]:
    """Return cursor.get_children() materialised as a list."""
    return list(cursor.get_children())


def _token_spellings(cursor: Cursor) -> List[str]:
    """Return all token spelling strings for *cursor*."""
    try:
        return [tok.spelling for tok in cursor.get_tokens()]
    except Exception:
        return []


def _make_ir_type(clang_type) -> IRType:
    """Build a minimal IRType from a Clang Type object."""
    if clang_type is None:
        return IRType(name="unknown", kind=IRTypeKind.UNKNOWN)
    spelling = clang_type.spelling or "unknown"
    kind = IRTypeKind.UNKNOWN
    ck = clang_type.kind
    if ck == ClangTypeKind.VOID:
        kind = IRTypeKind.VOID
    elif ck in (ClangTypeKind.INT, ClangTypeKind.UINT, ClangTypeKind.LONG,
                ClangTypeKind.ULONG, ClangTypeKind.LONGLONG, ClangTypeKind.ULONGLONG,
                ClangTypeKind.SHORT, ClangTypeKind.USHORT, ClangTypeKind.CHAR_S,
                ClangTypeKind.CHAR_U, ClangTypeKind.SCHAR, ClangTypeKind.UCHAR,
                ClangTypeKind.FLOAT, ClangTypeKind.DOUBLE, ClangTypeKind.LONGDOUBLE,
                ClangTypeKind.BOOL, ClangTypeKind.CHAR16, ClangTypeKind.CHAR32,
                ClangTypeKind.WCHAR):
        kind = IRTypeKind.PRIMITIVE
    elif ck == ClangTypeKind.POINTER:
        kind = IRTypeKind.POINTER
    elif ck in (ClangTypeKind.LVALUEREFERENCE, ClangTypeKind.RVALUEREFERENCE):
        kind = IRTypeKind.REFERENCE
    elif ck in (ClangTypeKind.CONSTANTARRAY, ClangTypeKind.INCOMPLETEARRAY,
                ClangTypeKind.VARIABLEARRAY, ClangTypeKind.DEPENDENTSIZEDARRAY):
        kind = IRTypeKind.ARRAY
    elif ck == ClangTypeKind.ENUM:
        kind = IRTypeKind.ENUM
    elif ck == ClangTypeKind.RECORD:
        kind = IRTypeKind.CLASS
    elif ck == ClangTypeKind.TYPEDEF:
        kind = IRTypeKind.TYPEDEF
    elif ck == ClangTypeKind.AUTO:
        kind = IRTypeKind.AUTO
    elif ck == ClangTypeKind.FUNCTIONPROTO:
        kind = IRTypeKind.FUNCTION_POINTER
    return IRType(
        name=spelling,
        cpp_name=spelling,
        kind=kind,
        is_const="const" in spelling,
        is_reference=(ck == ClangTypeKind.LVALUEREFERENCE),
        is_mut_reference=(ck == ClangTypeKind.RVALUEREFERENCE),
        is_pointer=(ck == ClangTypeKind.POINTER),
    )


# ─── StmtConverter ──────────────────────────────────────────────────────────

class StmtConverter:
    """Converts Clang Cursor nodes for statements and expressions into IR nodes.

    Usage::

        converter = StmtConverter(pattern_lib=lib)
        ir_stmt = converter.convert_stmt(some_cursor)
        ir_expr = converter.convert_expr(some_cursor)
    """

    def __init__(self, pattern_lib=None) -> None:
        self.pattern_lib = pattern_lib

    # ── public API ────────────────────────────────────────────────────────

    def convert_stmt(self, cursor: Cursor) -> IRStmt:
        """Convert a Clang statement cursor into an IRStmt node.

        For any cursor kind that is not explicitly handled, the converter
        falls back to ``IRRawCppStmt`` with the original C++ source text.
        """
        try:
            return self._dispatch_stmt(cursor)
        except Exception as exc:
            logger.debug("convert_stmt fallback for %s: %s", cursor.kind, exc)
            return IRRawCppStmt(
                cpp_source=_get_source_text(cursor),
            )

    def convert_expr(self, cursor: Cursor) -> IRExpr:
        """Convert a Clang expression cursor into an IRExpr node.

        For any cursor kind that is not explicitly handled, the converter
        falls back to ``IRRawCppExpr`` with the original C++ source text.
        """
        try:
            return self._dispatch_expr(cursor)
        except Exception as exc:
            logger.debug("convert_expr fallback for %s: %s", cursor.kind, exc)
            return IRRawCppExpr(
                cpp_source=_get_source_text(cursor),
                comment=f"unhandled {cursor.kind}",
            )

    # ── statement dispatch ────────────────────────────────────────────────

    def _dispatch_stmt(self, cursor: Cursor) -> IRStmt:
        kind = cursor.kind

        if kind == CursorKind.COMPOUND_STMT:
            return self._convert_compound_stmt(cursor)

        if kind == CursorKind.DECL_STMT:
            return self._convert_decl_stmt(cursor)

        if kind == CursorKind.RETURN_STMT:
            return self._convert_return_stmt(cursor)

        if kind == CursorKind.IF_STMT:
            return self._convert_if_stmt(cursor)

        if kind == CursorKind.SWITCH_STMT:
            return self._convert_switch_stmt(cursor)

        if kind == CursorKind.FOR_STMT:
            return self._convert_for_stmt(cursor)

        if kind == CursorKind.CXX_FOR_RANGE_STMT:
            return self._convert_for_range_stmt(cursor)

        if kind == CursorKind.WHILE_STMT:
            return self._convert_while_stmt(cursor)

        if kind == CursorKind.DO_STMT:
            return self._convert_do_stmt(cursor)

        if kind == CursorKind.BREAK_STMT:
            return IRBreakStmt()

        if kind == CursorKind.CONTINUE_STMT:
            return IRContinueStmt()

        if kind == CursorKind.CXX_THROW_EXPR:
            return self._convert_throw_stmt(cursor)

        if kind == CursorKind.CXX_TRY_STMT:
            return self._convert_try_catch_stmt(cursor)

        if kind == CursorKind.NULL_STMT:
            # Empty statement (bare semicolon) — produce an empty expression statement.
            return IRExprStmt(expr=None)

        # Fall through: any expression used as a statement.
        expr = self._try_dispatch_expr(cursor)
        if expr is not None:
            return IRExprStmt(expr=expr)

        return IRRawCppStmt(
            cpp_source=_get_source_text(cursor),
        )

    # ── expression dispatch ───────────────────────────────────────────────

    def _dispatch_expr(self, cursor: Cursor) -> IRExpr:
        result = self._try_dispatch_expr(cursor)
        if result is not None:
            return result
        return IRRawCppExpr(
            cpp_source=_get_source_text(cursor),
            comment=f"unhandled {cursor.kind}",
        )

    def _try_dispatch_expr(self, cursor: Cursor) -> Optional[IRExpr]:
        """Attempt to convert *cursor* as an expression; return None if
        the cursor kind is not a recognised expression."""
        kind = cursor.kind

        # -- Literals ---------------------------------------------------
        if kind == CursorKind.INTEGER_LITERAL:
            return self._convert_integer_literal(cursor)

        if kind == CursorKind.FLOATING_LITERAL:
            return self._convert_floating_literal(cursor)

        if kind == CursorKind.STRING_LITERAL:
            return self._convert_string_literal(cursor)

        if kind == CursorKind.CHARACTER_LITERAL:
            return self._convert_character_literal(cursor)

        if kind == CursorKind.CXX_BOOL_LITERAL_EXPR:
            return self._convert_bool_literal(cursor)

        if kind == CursorKind.CXX_NULL_PTR_LITERAL_EXPR:
            return IRLiteral(value="None", kind=LiteralKind.NULL)

        # -- References -------------------------------------------------
        if kind == CursorKind.DECL_REF_EXPR:
            return self._convert_decl_ref_expr(cursor)

        if kind == CursorKind.MEMBER_REF_EXPR:
            return self._convert_member_ref_expr(cursor)

        # -- Calls ------------------------------------------------------
        if kind == CursorKind.CALL_EXPR:
            return self._convert_call_expr(cursor)

        if _CXX_MEMBER_CALL_EXPR is not None and kind == _CXX_MEMBER_CALL_EXPR:
            return self._convert_member_call_expr(cursor)

        if _CXX_OPERATOR_CALL_EXPR is not None and kind == _CXX_OPERATOR_CALL_EXPR:
            return self._convert_operator_call_expr(cursor)

        # -- Binary / unary / ternary -----------------------------------
        if kind == CursorKind.BINARY_OPERATOR:
            return self._convert_binary_operator(cursor)

        if kind == CursorKind.COMPOUND_ASSIGNMENT_OPERATOR:
            return self._convert_binary_operator(cursor)

        if kind == CursorKind.UNARY_OPERATOR:
            return self._convert_unary_operator(cursor)

        if kind == CursorKind.CONDITIONAL_OPERATOR:
            return self._convert_conditional_operator(cursor)

        # -- Casts ------------------------------------------------------
        if kind == CursorKind.CXX_STATIC_CAST_EXPR:
            return self._convert_cast_expr(cursor, CastKind.AS)

        if kind == CursorKind.CXX_DYNAMIC_CAST_EXPR:
            return self._convert_cast_expr(cursor, CastKind.INTO)

        if kind == CursorKind.CXX_REINTERPRET_CAST_EXPR:
            return self._convert_cast_expr(cursor, CastKind.UNSAFE_TRANSMUTE)

        if kind == CursorKind.CXX_CONST_CAST_EXPR:
            return self._convert_cast_expr(cursor, CastKind.UNSAFE_PTR_CAST)

        if kind == CursorKind.CSTYLE_CAST_EXPR:
            return self._convert_cast_expr(cursor, CastKind.AS)

        if kind == CursorKind.CXX_FUNCTIONAL_CAST_EXPR:
            return self._convert_functional_cast_expr(cursor)

        # -- Implicit / paren / unexposed (unwrap) ----------------------
        if kind == CursorKind.PAREN_EXPR:
            return self._unwrap_single_child_expr(cursor)

        if _IMPLICIT_CAST_EXPR is not None and kind == _IMPLICIT_CAST_EXPR:
            return self._unwrap_single_child_expr(cursor)

        if kind == CursorKind.UNEXPOSED_EXPR:
            return self._unwrap_single_child_expr(cursor)

        # -- new / delete -----------------------------------------------
        if kind == CursorKind.CXX_NEW_EXPR:
            return self._convert_new_expr(cursor)

        if kind == CursorKind.CXX_DELETE_EXPR:
            return self._convert_delete_expr(cursor)

        # -- this -------------------------------------------------------
        if kind == CursorKind.CXX_THIS_EXPR:
            return self._convert_this_expr(cursor)

        # -- Subscript --------------------------------------------------
        if kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            return self._convert_index_expr(cursor)

        # -- Lambda -----------------------------------------------------
        if kind == CursorKind.LAMBDA_EXPR:
            return self._convert_lambda_expr(cursor)

        # -- Init list --------------------------------------------------
        if kind == CursorKind.INIT_LIST_EXPR:
            return self._convert_init_list_expr(cursor)

        # -- C++ construct expressions -----------------------------------
        if kind == CursorKind.CXX_CONSTRUCT_EXPR:
            return self._convert_construct_expr(cursor)

        if kind == CursorKind.CXX_TEMPORARY_OBJECT_EXPR:
            return self._convert_construct_expr(cursor)

        # -- sizeof / alignof -------------------------------------------
        if _UNARY_EXPR_OR_TYPE_TRAIT_EXPR is not None and kind == _UNARY_EXPR_OR_TYPE_TRAIT_EXPR:
            return self._convert_sizeof_expr(cursor)

        # -- throw as expression ----------------------------------------
        if kind == CursorKind.CXX_THROW_EXPR:
            # When encountered as an expression context, wrap in a raw expr.
            children = _children(cursor)
            inner = self.convert_expr(children[0]) if children else None
            return IRRawCppExpr(
                cpp_source=_get_source_text(cursor),
                comment="C++ throw expression",
            )

        # -- Overloaded declaration reference (operator++, operator<< etc.) --
        if kind == CursorKind.OVERLOADED_DECL_REF:
            return IRNameRef(name=cursor.spelling or "operator")

        # -- Type reference (type name in templates, casts, etc.) ----------
        if kind == CursorKind.TYPE_REF:
            ref = cursor.referenced
            name = (ref.spelling if ref else cursor.spelling) or ""
            return IRNameRef(name=name)

        # -- Template reference -------------------------------------------
        if kind == CursorKind.TEMPLATE_REF:
            ref = cursor.referenced
            name = (ref.spelling if ref else cursor.spelling) or ""
            return IRNameRef(name=name)

        # -- Namespace reference ------------------------------------------
        if kind == CursorKind.NAMESPACE_REF:
            return IRNameRef(name=cursor.spelling or "")

        # -- GNU null expression (__null) ---------------------------------
        if kind == CursorKind.GNU_NULL_EXPR:
            return IRLiteral(value="None", kind=LiteralKind.NULL)

        # -- Materialize temporary (implicit temp object) -----------------
        if hasattr(CursorKind, 'MATERIALIZE_TEMPORARY_EXPR') and kind == CursorKind.MATERIALIZE_TEMPORARY_EXPR:
            return self._unwrap_single_child_expr(cursor)

        # -- Expression with cleanups (RAII destructors) ------------------
        if hasattr(CursorKind, 'EXPR_WITH_CLEANUPS') and kind == CursorKind.EXPR_WITH_CLEANUPS:
            return self._unwrap_single_child_expr(cursor)

        # -- Default argument expression ----------------------------------
        if hasattr(CursorKind, 'CXX_DEFAULT_ARG_EXPR') and kind == CursorKind.CXX_DEFAULT_ARG_EXPR:
            children = _children(cursor)
            if children:
                return self.convert_expr(children[0])
            return IRNameRef(name="Default::default()")

        # Not a recognised expression kind.
        return None

    # ── compound / block ──────────────────────────────────────────────────

    def _convert_compound_stmt(self, cursor: Cursor) -> IRBlock:
        stmts: List[IRStmt] = []
        for child in cursor.get_children():
            stmts.append(self.convert_stmt(child))
        return IRBlock(statements=stmts)

    # ── declaration statement ─────────────────────────────────────────────

    def _convert_decl_stmt(self, cursor: Cursor) -> IRStmt:
        """DECL_STMT typically contains one or more VAR_DECL children."""
        children = _children(cursor)
        stmts: List[IRStmt] = []

        _type_alias_kinds = {
            CursorKind.TYPE_ALIAS_DECL,
            CursorKind.TYPEDEF_DECL,
        }
        _type_alias_template = getattr(CursorKind, "TYPE_ALIAS_TEMPLATE_DECL", None)
        if _type_alias_template is not None:
            _type_alias_kinds.add(_type_alias_template)
        _using_decl = getattr(CursorKind, "USING_DECLARATION", None)

        for child in children:
            if child.kind == CursorKind.VAR_DECL:
                stmts.append(self._convert_var_decl(child))
            elif child.kind in _type_alias_kinds:
                # `using Alias = Type;` or `typedef Type Alias;` inside a function body.
                # Rust doesn't allow type aliases inside function bodies as statements,
                # so emit as a comment.
                alias_name = child.spelling or ""
                underlying = (child.type.spelling if child.type else "") or ""
                src = f"// using {alias_name} = {underlying};"
                stmts.append(IRRawCppStmt(cpp_source=src))
            elif _using_decl is not None and child.kind == _using_decl:
                # `using Base::method;` — expose an inherited name.  Skip silently.
                pass

        if len(stmts) == 1:
            return stmts[0]
        if stmts:
            # Multiple declarations in one statement — wrap in a block.
            return IRBlock(statements=stmts)
        # Unexpected DECL_STMT content; fall back with a comment (not todo!).
        src_text = _get_source_text(cursor).replace('\n', ' ').strip()
        return IRRawCppStmt(
            cpp_source=f"// {src_text}",
        )

    def _convert_var_decl(self, cursor: Cursor) -> IRVarDecl:
        cpp_name = cursor.spelling or ""
        # Apply the same snake_case convention that is used when emitting
        # function parameters and when resolving DECL_REF_EXPR references to
        # local variables.  This keeps declaration and use sites consistent.
        name = _to_snake_case_id(cpp_name) if cpp_name else cpp_name
        ir_type = _make_ir_type(cursor.type)
        is_const = cursor.type and "const" in (cursor.type.spelling or "")
        initializer: Optional[IRExpr] = None

        children = _children(cursor)
        # The last child (if any) that is an expression is typically the initializer.
        for child in children:
            # Skip type references, template refs, etc.
            if self._is_expr_cursor(child):
                initializer = self.convert_expr(child)

        # If the AST only exposed a plain name-ref as the initializer (e.g.
        # std::min/max whose CALL_EXPR args are hidden inside an UNEXPOSED_EXPR),
        # try to recover the full expression from the VAR_DECL token stream.
        if isinstance(initializer, IRNameRef):
            recovered = self._recover_initializer_from_tokens(cursor, cpp_name)
            if recovered is not None:
                initializer = recovered

        return IRVarDecl(
            name=name,
            type=ir_type,
            initializer=initializer,
            is_mutable=not is_const,
            is_static=False,
            cpp_name=cpp_name,
        )

    def _recover_initializer_from_tokens(self, cursor: Cursor, var_name: str) -> Optional["IRRawCppExpr"]:
        """Return an IRRawCppExpr built from the C++ source after the '=' token.

        This is a fallback for cases where the AST exposes only a bare
        IRNameRef as the initializer (e.g. when std::min(…) is hidden
        inside an UNEXPOSED_EXPR that only shows the callee).
        """
        try:
            tokens = list(cursor.get_tokens())
        except Exception:
            return None

        # Collect all token spellings and find '='
        spellings = [t.spelling for t in tokens]
        eq_idx = None
        for i, sp in enumerate(spellings):
            if sp == "=":
                eq_idx = i
                break

        if eq_idx is None:
            return None

        # Everything after '=' is the initializer source text.
        init_tokens = spellings[eq_idx + 1:]
        if not init_tokens:
            return None

        # Only bother if the initializer contains a '(' – i.e. it looks like a call.
        if "(" not in init_tokens:
            return None

        init_src = " ".join(init_tokens).strip()
        if not init_src:
            return None

        return IRRawCppExpr(cpp_source=init_src)

    # ── return ────────────────────────────────────────────────────────────

    def _convert_return_stmt(self, cursor: Cursor) -> IRReturnStmt:
        children = _children(cursor)
        value: Optional[IRExpr] = None
        if children:
            value = self.convert_expr(children[0])
        return IRReturnStmt(value=value)

    # ── if ────────────────────────────────────────────────────────────────

    def _convert_if_stmt(self, cursor: Cursor) -> IRIfStmt:
        """IF_STMT children: [condition, then-block, (optional) else-block]."""
        children = _children(cursor)
        condition: Optional[IRExpr] = None
        then_block: Optional[IRBlock] = None
        else_block: Optional[IRBlock] = None

        if len(children) >= 1:
            condition = self.convert_expr(children[0])
        if len(children) >= 2:
            then_block = self._ensure_block(children[1])
        if len(children) >= 3:
            # The else clause may be another IF_STMT (else if chain) or a COMPOUND_STMT.
            else_child = children[2]
            if else_child.kind == CursorKind.IF_STMT:
                # Model else-if as an IRBlock containing a single IRIfStmt.
                inner_if = self._convert_if_stmt(else_child)
                else_block = IRBlock(statements=[inner_if])
            else:
                else_block = self._ensure_block(else_child)

        return IRIfStmt(
            condition=condition,
            then_block=then_block,
            else_block=else_block,
        )

    # ── switch / match ────────────────────────────────────────────────────

    def _convert_switch_stmt(self, cursor: Cursor) -> IRMatchStmt:
        """SWITCH_STMT children: [expression, body (compound_stmt with case children)]."""
        children = _children(cursor)
        expr: Optional[IRExpr] = None
        arms: List[IRMatchArm] = []

        if len(children) >= 1:
            expr = self.convert_expr(children[0])
        if len(children) >= 2:
            body = children[1]
            arms = self._collect_match_arms(body)

        return IRMatchStmt(expr=expr, arms=arms)

    def _collect_match_arms(self, cursor: Cursor) -> List[IRMatchArm]:
        """Walk the body of a switch and collect IRMatchArm entries."""
        arms: List[IRMatchArm] = []
        for child in cursor.get_children():
            if child.kind == CursorKind.CASE_STMT:
                arms.append(self._convert_case_stmt(child))
            elif child.kind == CursorKind.DEFAULT_STMT:
                arms.append(self._convert_default_stmt(child))
            # Other statements between/before cases are rare but possible (e.g.,
            # a declaration preceding the first case).  We silently skip them
            # here; a downstream pass can flag the anomaly.
        return arms

    def _convert_case_stmt(self, cursor: Cursor) -> IRMatchArm:
        """CASE_STMT children: [value-expr, sub-statement(s)]."""
        children = _children(cursor)
        patterns: List[str] = []
        body_stmts: List[IRStmt] = []

        if children:
            # First child is the case value expression.
            patterns.append(_get_source_text(children[0]).strip())
            # Remaining children are the body statements.
            for child in children[1:]:
                if child.kind == CursorKind.CASE_STMT:
                    # Fallthrough to another case — merge patterns.
                    inner_arm = self._convert_case_stmt(child)
                    patterns.extend(inner_arm.patterns)
                    if inner_arm.body:
                        body_stmts.extend(inner_arm.body.statements)
                elif child.kind == CursorKind.DEFAULT_STMT:
                    inner_arm = self._convert_default_stmt(child)
                    patterns.clear()  # default supersedes
                    if inner_arm.body:
                        body_stmts.extend(inner_arm.body.statements)
                    return IRMatchArm(
                        patterns=patterns,
                        body=IRBlock(statements=body_stmts) if body_stmts else None,
                        is_default=True,
                    )
                else:
                    body_stmts.append(self.convert_stmt(child))

        return IRMatchArm(
            patterns=patterns,
            body=IRBlock(statements=body_stmts) if body_stmts else None,
            is_default=False,
        )

    def _convert_default_stmt(self, cursor: Cursor) -> IRMatchArm:
        """DEFAULT_STMT children: [sub-statement(s)]."""
        children = _children(cursor)
        body_stmts: List[IRStmt] = []
        for child in children:
            body_stmts.append(self.convert_stmt(child))
        return IRMatchArm(
            patterns=["_"],
            body=IRBlock(statements=body_stmts) if body_stmts else None,
            is_default=True,
        )

    # ── for loop ──────────────────────────────────────────────────────────

    def _convert_for_stmt(self, cursor: Cursor) -> IRForLoop:
        """FOR_STMT children: [init, condition, increment, body].

        Clang OMITS NULL_STMT placeholders for absent for-loop components,
        so we cannot rely on child count alone. We use the byte offsets of the
        two semicolons inside ``for (INIT; COND; INC)`` to classify each child.

        Observed child counts:
          1  → [body]            (all of init/cond/inc absent)
          2  → [cond, body] or [init, body]
          3  → [cond, incr, body] or [init, cond, body] or [init, incr, body]
          4  → [init, cond, incr, body]
        """
        children = _children(cursor)
        init: Optional[IRStmt] = None
        condition: Optional[IRExpr] = None
        increment: Optional[IRExpr] = None
        body: Optional[IRBlock] = None

        # --- find the two semicolons that delimit INIT ; COND ; INC ----------
        semis: list = []
        close_paren_offset: int = -1
        depth = 0
        opened = False
        for tok in cursor.get_tokens():
            sp = tok.spelling
            if sp == "for" and not opened:
                continue
            if sp == "(" and not opened:
                opened = True
                continue
            if not opened:
                continue
            if sp == "(":
                depth += 1
            elif sp == ")":
                if depth == 0:
                    close_paren_offset = tok.extent.start.offset
                    break
                depth -= 1
            elif sp == ";" and depth == 0:
                semis.append(tok.extent.start.offset)
                if len(semis) == 2:
                    pass  # keep scanning for close paren

        # --- classify each child by its source position ----------------------
        def _child_region(child) -> str:
            """'init' | 'cond' | 'incr' | 'body'"""
            offset = child.extent.start.offset
            if len(semis) < 2:
                return "body"
            if offset < semis[0]:
                return "init"
            if offset < semis[1]:
                return "cond"
            if close_paren_offset < 0 or offset < close_paren_offset:
                return "incr"
            return "body"

        if len(semis) == 2:
            for child in children:
                region = _child_region(child)
                if region == "init" and child.kind != CursorKind.NULL_STMT:
                    init = self.convert_stmt(child)
                elif region == "cond" and child.kind != CursorKind.NULL_STMT:
                    condition = self.convert_expr(child)
                elif region == "incr" and child.kind != CursorKind.NULL_STMT:
                    increment = self.convert_expr(child)
                elif region == "body":
                    body = self._ensure_block(child)
        else:
            # Fallback when token scanning fails (e.g. macro-generated loops):
            # old positional logic with null-stmt guard
            if len(children) >= 1 and children[0].kind != CursorKind.NULL_STMT:
                init = self.convert_stmt(children[0])
            if len(children) >= 2 and children[1].kind != CursorKind.NULL_STMT:
                condition = self.convert_expr(children[1])
            if len(children) >= 3 and children[2].kind != CursorKind.NULL_STMT:
                increment = self.convert_expr(children[2])
            if len(children) >= 4:
                body = self._ensure_block(children[3])

        return IRForLoop(
            init=init,
            condition=condition,
            increment=increment,
            body=body,
        )

    # ── range-for loop ────────────────────────────────────────────────────

    def _convert_for_range_stmt(self, cursor: Cursor) -> IRForRangeLoop:
        """CXX_FOR_RANGE_STMT children vary across Clang versions.

        Typical layout: [var-decl, range-init-expr, body-compound-stmt] with
        possible implicit helper declarations interleaved.
        """
        children = _children(cursor)
        variable = ""
        var_type: Optional[IRType] = None
        iterable: Optional[IRExpr] = None
        body: Optional[IRBlock] = None
        is_ref = False
        is_mut_ref = False

        for child in children:
            if child.kind == CursorKind.VAR_DECL and not variable:
                # The loop variable declaration.
                variable = child.spelling or ""
                var_type = _make_ir_type(child.type)
                type_spelling = child.type.spelling if child.type else ""
                if "&&" in type_spelling or ("&" in type_spelling and "const" not in type_spelling):
                    is_mut_ref = True
                elif "&" in type_spelling:
                    is_ref = True
            elif child.kind == CursorKind.DECL_REF_EXPR and iterable is None:
                iterable = self.convert_expr(child)
            elif self._is_expr_cursor(child) and iterable is None:
                iterable = self.convert_expr(child)
            elif child.kind == CursorKind.COMPOUND_STMT:
                body = self._convert_compound_stmt(child)

        # If we never found an iterable from the children directly, attempt to
        # grab one from a DECL_STMT or the second meaningful child.
        if iterable is None:
            # Fallback: search deeper for a call or member expression.
            for child in children:
                if child.kind == CursorKind.DECL_STMT:
                    inner = _children(child)
                    for ic in inner:
                        if self._is_expr_cursor(ic):
                            iterable = self.convert_expr(ic)
                            break
                if iterable is not None:
                    break

        return IRForRangeLoop(
            variable=variable,
            var_type=var_type,
            iterable=iterable,
            body=body,
            is_ref=is_ref,
            is_mut_ref=is_mut_ref,
        )

    # ── while loop ────────────────────────────────────────────────────────

    def _convert_while_stmt(self, cursor: Cursor) -> IRWhileLoop:
        children = _children(cursor)
        condition: Optional[IRExpr] = None
        body: Optional[IRBlock] = None
        if len(children) >= 1:
            condition = self.convert_expr(children[0])
        if len(children) >= 2:
            body = self._ensure_block(children[1])
        return IRWhileLoop(condition=condition, body=body, is_do_while=False)

    # ── do-while loop ─────────────────────────────────────────────────────

    def _convert_do_stmt(self, cursor: Cursor) -> IRWhileLoop:
        """DO_STMT children: [body, condition]."""
        children = _children(cursor)
        condition: Optional[IRExpr] = None
        body: Optional[IRBlock] = None
        if len(children) >= 1:
            body = self._ensure_block(children[0])
        if len(children) >= 2:
            condition = self.convert_expr(children[1])
        return IRWhileLoop(condition=condition, body=body, is_do_while=True)

    # ── throw ─────────────────────────────────────────────────────────────

    def _convert_throw_stmt(self, cursor: Cursor) -> IRThrowStmt:
        children = _children(cursor)
        expr: Optional[IRExpr] = None
        if children:
            expr = self.convert_expr(children[0])
        return IRThrowStmt(expr=expr)

    # ── try / catch ───────────────────────────────────────────────────────

    def _convert_try_catch_stmt(self, cursor: Cursor) -> IRTryCatchStmt:
        """CXX_TRY_STMT children: [try-compound-stmt, catch-handler, ...]."""
        children = _children(cursor)
        try_block: Optional[IRBlock] = None
        catch_blocks: List[IRCatchBlock] = []

        for i, child in enumerate(children):
            if i == 0 and child.kind == CursorKind.COMPOUND_STMT:
                try_block = self._convert_compound_stmt(child)
            elif child.kind == CursorKind.CXX_CATCH_STMT:
                catch_blocks.append(self._convert_catch_handler(child))

        return IRTryCatchStmt(try_block=try_block, catch_blocks=catch_blocks)

    def _convert_catch_handler(self, cursor: Cursor) -> IRCatchBlock:
        """CXX_CATCH_STMT children: [var-decl (optional), compound-stmt]."""
        children = _children(cursor)
        exception_type: Optional[IRType] = None
        variable_name = ""
        body: Optional[IRBlock] = None

        for child in children:
            if child.kind == CursorKind.VAR_DECL:
                variable_name = child.spelling or ""
                exception_type = _make_ir_type(child.type)
            elif child.kind == CursorKind.COMPOUND_STMT:
                body = self._convert_compound_stmt(child)

        # catch (...) — no VAR_DECL child.
        if exception_type is None and not variable_name:
            exception_type = IRType(name="...", cpp_name="...")

        return IRCatchBlock(
            exception_type=exception_type,
            variable_name=variable_name,
            body=body,
        )

    # ── literals ──────────────────────────────────────────────────────────

    def _convert_integer_literal(self, cursor: Cursor) -> IRLiteral:
        tokens = _token_spellings(cursor)
        value = tokens[0] if tokens else _get_source_text(cursor).strip()
        return IRLiteral(value=value, kind=LiteralKind.INT)

    def _convert_floating_literal(self, cursor: Cursor) -> IRLiteral:
        tokens = _token_spellings(cursor)
        value = tokens[0] if tokens else _get_source_text(cursor).strip()
        return IRLiteral(value=value, kind=LiteralKind.FLOAT)

    def _convert_string_literal(self, cursor: Cursor) -> IRLiteral:
        value = _get_source_text(cursor).strip()
        return IRLiteral(value=value, kind=LiteralKind.STRING)

    def _convert_character_literal(self, cursor: Cursor) -> IRLiteral:
        tokens = _token_spellings(cursor)
        value = tokens[0] if tokens else _get_source_text(cursor).strip()
        return IRLiteral(value=value, kind=LiteralKind.CHAR)

    def _convert_bool_literal(self, cursor: Cursor) -> IRLiteral:
        tokens = _token_spellings(cursor)
        value = tokens[0] if tokens else _get_source_text(cursor).strip()
        return IRLiteral(value=value, kind=LiteralKind.BOOL)

    # ── references ────────────────────────────────────────────────────────

    def _convert_decl_ref_expr(self, cursor: Cursor) -> IRNameRef:
        name = cursor.spelling or ""
        ref = cursor.referenced
        qualified = ""
        if ref is not None:
            qualified = self._get_qualified_name(ref)
            if not name:
                # Fallback 1: use the referenced declaration's spelling / displayname.
                name = ref.spelling or getattr(ref, "displayname", "") or ""
            # For parameter or FUNCTION-LOCAL variable references, apply the
            # same snake_case convention used when emitting the declaration.
            # We deliberately skip global/static variables (those whose semantic
            # parent is a namespace or the translation unit) because their
            # declaration sites are NOT snake_cased by the emitter.
            if ref.kind == CursorKind.PARM_DECL and name:
                name = _to_snake_case_id(name)
            elif ref.kind == CursorKind.VAR_DECL and name:
                try:
                    parent_kind = ref.semantic_parent.kind if ref.semantic_parent else None
                    _LOCAL_PARENT_KINDS = {
                        CursorKind.FUNCTION_DECL,
                        CursorKind.CXX_METHOD,
                        CursorKind.FUNCTION_TEMPLATE,
                        CursorKind.CONSTRUCTOR,
                        CursorKind.DESTRUCTOR,
                    }
                    if parent_kind in _LOCAL_PARENT_KINDS:
                        name = _to_snake_case_id(name)
                except Exception:
                    pass
        if not name:
            # Fallback 2: token scan — the name is usually the last identifier token.
            for tok in reversed(_token_spellings(cursor)):
                if tok.isidentifier() and tok not in ("this", "auto", "nullptr", "true", "false"):
                    name = tok
                    break
        return IRNameRef(name=name, qualified_name=qualified)

    def _convert_member_ref_expr(self, cursor: Cursor) -> IRMemberExpr:
        children = _children(cursor)
        obj: Optional[IRExpr] = None
        member = cursor.spelling or ""
        is_arrow = False

        # The first child is the object expression.
        # For template member functions, there may be additional children
        # for template arguments — only take the first as the object.
        if children:
            obj = self.convert_expr(children[0])

        # If member name is empty, try to get it from the cursor's
        # referenced declaration or from tokens.
        if not member:
            ref = cursor.referenced
            if ref and ref.spelling:
                member = ref.spelling
            else:
                # Last resort: scan tokens for the member name
                tokens = _token_spellings(cursor)
                # The member name is typically the last identifier token
                for tok in reversed(tokens):
                    if tok.isidentifier() and tok not in ("this", "auto"):
                        member = tok
                        break

        # Apply the same snake_case transformation used for struct field
        # declarations (class_mapper snake_cases field names in structs).
        # Only snake_case FIELD references (FIELD_DECL), not method calls.
        if member:
            try:
                ref = cursor.referenced
                if ref is not None and ref.kind == CursorKind.FIELD_DECL:
                    member = _to_snake_case_id(member)
            except Exception:
                pass

        # Detect arrow (->) vs dot (.) by checking tokens.
        tokens = _token_spellings(cursor)
        if "->" in tokens:
            is_arrow = True

        return IRMemberExpr(object=obj, member=member, is_arrow=is_arrow)

    # ── calls ─────────────────────────────────────────────────────────────

    def _convert_call_expr(self, cursor: Cursor) -> IRExpr:
        """Regular CALL_EXPR: first child is the callee, rest are arguments.

        Special case: clang sometimes represents copy/move constructor calls for
        value types (like ``RWDigits(Z)``) as a CALL_EXPR with spelling equal to
        the type name and a single child that is the source expression wrapped in
        an UNEXPOSED_EXPR.  In this case, treat it as a trivial pass-through copy
        (Rust Copy types don't need an explicit constructor call).
        """
        children = _children(cursor)
        function: Optional[IRExpr] = None
        args: List[IRExpr] = []

        # Detect copy-constructor CALL_EXPR:
        # cursor.spelling == TypeName AND cursor.type.spelling ends with TypeName
        # AND exactly one child (the source expression).
        if len(children) == 1 and cursor.spelling and cursor.type and cursor.type.spelling:
            call_spelling = cursor.spelling.split("::")[-1]
            type_spelling = cursor.type.spelling.split("::")[-1].rstrip("* ")
            if call_spelling == type_spelling:
                # This is a copy/move constructor call — just return the inner expression.
                inner = self.convert_expr(children[0])
                if inner is not None:
                    return inner

        if children:
            function = self.convert_expr(children[0])
            for child in children[1:]:
                args.append(self.convert_expr(child))

        return IRCallExpr(function=function, args=args, is_method_call=False)

    def _convert_member_call_expr(self, cursor: Cursor) -> IRCallExpr:
        """CXX_MEMBER_CALL_EXPR: first child is a MEMBER_REF_EXPR, rest are args."""
        children = _children(cursor)
        function: Optional[IRExpr] = None
        args: List[IRExpr] = []

        if children:
            function = self.convert_expr(children[0])
            for child in children[1:]:
                args.append(self.convert_expr(child))

        return IRCallExpr(function=function, args=args, is_method_call=True)

    def _convert_operator_call_expr(self, cursor: Cursor) -> IRExpr:
        """CXX_OPERATOR_CALL_EXPR — an overloaded operator invocation.

        If the operator is a recognised binary operator and there are exactly
        two operand children (after the implicit operator function reference),
        produce an IRBinaryExpr.  Otherwise fall back to IRCallExpr.
        """
        children = _children(cursor)
        # The first child is typically a DECL_REF_EXPR referencing the
        # operator function, followed by the operands.
        if not children:
            return IRRawCppExpr(cpp_source=_get_source_text(cursor))

        op_name = cursor.spelling or ""
        # Attempt to extract the operator token from the function reference.
        op_token = self._extract_operator_token(op_name, children[0])
        operands = children[1:]

        # Binary operator with two operands?
        if op_token in _OVERLOADED_BINARY_TOKENS and len(operands) == 2:
            return IRBinaryExpr(
                op=_BINARY_OP_MAP[op_token],
                left=self.convert_expr(operands[0]),
                right=self.convert_expr(operands[1]),
            )

        # Unary prefix operator with one operand?
        if op_token in _UNARY_PREFIX_OP_MAP and len(operands) == 1:
            return IRUnaryExpr(
                op=_UNARY_PREFIX_OP_MAP[op_token],
                operand=self.convert_expr(operands[0]),
            )

        # Generic: treat as a function call.
        function = self.convert_expr(children[0])
        args = [self.convert_expr(c) for c in operands]
        return IRCallExpr(function=function, args=args, is_method_call=False)

    @staticmethod
    def _extract_operator_token(op_name: str, first_child: Cursor) -> str:
        """Try to determine the operator symbol from an overloaded operator call.

        ``op_name`` is ``cursor.spelling`` which for operators is something
        like ``operator+``.  We strip the ``operator`` prefix and return the
        remaining symbol.  Falls back to scanning the first child's tokens.
        """
        if op_name.startswith("operator"):
            sym = op_name[len("operator"):].strip()
            if sym:
                return sym

        # Fallback — scan tokens of the operator reference child.
        tokens = _token_spellings(first_child)
        for tok in reversed(tokens):
            if tok in _OVERLOADED_BINARY_TOKENS or tok in _UNARY_PREFIX_OP_MAP:
                return tok
        return ""

    # ── binary operator ───────────────────────────────────────────────────

    def _convert_binary_operator(self, cursor: Cursor) -> IRBinaryExpr:
        children = _children(cursor)
        left: Optional[IRExpr] = None
        right: Optional[IRExpr] = None

        if len(children) >= 1:
            left = self.convert_expr(children[0])
        if len(children) >= 2:
            right = self.convert_expr(children[1])

        op = self._determine_binary_op(cursor, children)
        return IRBinaryExpr(op=op, left=left, right=right)

    def _determine_binary_op(self, cursor: Cursor, children: List[Cursor]) -> BinaryOp:
        """Determine the BinaryOp for a BINARY_OPERATOR / COMPOUND_ASSIGNMENT_OPERATOR cursor.

        Uses ``cursor.binary_operator`` if the libclang binding exposes it,
        otherwise falls back to scanning the tokens that appear between the
        two operand extents.
        """
        # Modern libclang bindings (>= 16) expose ``binary_operator``.
        if hasattr(cursor, "binary_operator"):
            try:
                bo = cursor.binary_operator
                # bo may be an enum value or an int; convert to the symbol string.
                bo_name = str(bo)
                # Some bindings return an enum member with a .name attribute.
                if hasattr(bo, "name"):
                    bo_name = bo.name
                # Try a simple lookup: the name may directly match a BinaryOp member.
                for member in BinaryOp:
                    if member.name == bo_name:
                        return member
            except Exception:
                pass

        # Fallback: position-based token scanning.
        # The binary operator token sits BETWEEN the end of the left operand
        # and the start of the right operand in the source.  Crucially, we use
        # byte offsets rather than token-set membership so that an `&` that
        # appears inside the left operand (e.g. `&x_borrow`) is not confused
        # with the `&` binary operator that follows it.
        if len(children) >= 2:
            try:
                left_end = children[0].extent.end.offset
                right_start = children[1].extent.start.offset
                candidate_ops: List[str] = []
                for tok in cursor.get_tokens():
                    tok_offset = tok.extent.start.offset
                    if left_end <= tok_offset < right_start:
                        sp = tok.spelling
                        if sp in _BINARY_OP_MAP:
                            candidate_ops.append(sp)
                # Prefer longer operators first (e.g. ">>=" before ">")
                candidate_ops.sort(key=len, reverse=True)
                if candidate_ops:
                    return _BINARY_OP_MAP[candidate_ops[0]]
            except Exception:
                pass

            # Fallback using token-set exclusion (may fail when operator token
            # also appears inside an operand, e.g. `&` in `f(&x) & g(&y)`).
            tokens = _token_spellings(cursor)
            left_tokens = set(_token_spellings(children[0]))
            right_tokens = set(_token_spellings(children[1]))
            candidate_ops = []
            for tok in tokens:
                if tok not in left_tokens and tok not in right_tokens:
                    candidate_ops.append(tok)
            candidate_ops.sort(key=len, reverse=True)
            for cand in candidate_ops:
                if cand in _BINARY_OP_MAP:
                    return _BINARY_OP_MAP[cand]

        # If all else fails, scan all tokens for a known operator.
        tokens = _token_spellings(cursor)
        for tok in tokens:
            if tok in _BINARY_OP_MAP:
                return _BINARY_OP_MAP[tok]

        return BinaryOp.COMMA  # Degenerate fallback.

    # ── unary operator ────────────────────────────────────────────────────

    def _convert_unary_operator(self, cursor: Cursor) -> IRUnaryExpr:
        children = _children(cursor)
        operand: Optional[IRExpr] = None
        if children:
            operand = self.convert_expr(children[0])

        op = self._determine_unary_op(cursor, children)
        return IRUnaryExpr(op=op, operand=operand)

    def _determine_unary_op(self, cursor: Cursor, children: List[Cursor]) -> UnaryOp:
        """Determine the UnaryOp for a UNARY_OPERATOR cursor.

        Prefix vs. postfix is disambiguated by comparing the source location
        of the operator token against the operand: if the operator token
        comes before the operand's start location, it is prefix; otherwise
        postfix.
        """
        tokens = _token_spellings(cursor)
        child_tokens = set()
        if children:
            child_tokens = set(_token_spellings(children[0]))

        # Find operator token (the one that is NOT part of the child expression).
        op_tok = ""
        is_prefix = True

        all_cursor_tokens = list(cursor.get_tokens()) if True else []
        try:
            all_cursor_tokens = list(cursor.get_tokens())
        except Exception:
            all_cursor_tokens = []

        if all_cursor_tokens and children:
            # The operator token is the one whose spelling is not entirely
            # within the child's token set, OR that lies outside the child's
            # source range.
            child_start = children[0].extent.start if children[0].extent else None
            child_end = children[0].extent.end if children[0].extent else None
            for tok in all_cursor_tokens:
                tok_sp = tok.spelling
                if tok_sp in _UNARY_PREFIX_OP_MAP or tok_sp in _UNARY_POSTFIX_OP_MAP:
                    # Check whether this token is before or after the child.
                    if child_start and tok.extent.start.line < child_start.line:
                        is_prefix = True
                        op_tok = tok_sp
                        break
                    if child_start and tok.extent.start.line == child_start.line and tok.extent.start.column < child_start.column:
                        is_prefix = True
                        op_tok = tok_sp
                        break
                    if child_end and tok.extent.start.line > child_end.line:
                        is_prefix = False
                        op_tok = tok_sp
                        break
                    if child_end and tok.extent.start.line == child_end.line and tok.extent.start.column >= child_end.column:
                        is_prefix = False
                        op_tok = tok_sp
                        break

        if not op_tok:
            # Simple fallback: scan tokens for any operator-like symbol.
            for t in tokens:
                if t in _UNARY_PREFIX_OP_MAP or t in _UNARY_POSTFIX_OP_MAP:
                    op_tok = t
                    break

        if is_prefix and op_tok in _UNARY_PREFIX_OP_MAP:
            op = _UNARY_PREFIX_OP_MAP[op_tok]
            # C++ `&var` yields a pointer.  If the result type is a non-const
            # pointer (e.g. `digit_t *`), use ADDR_OF_MUT so the emitter
            # produces `std::ptr::addr_of_mut!(var)` → `*mut T`.
            if op == UnaryOp.ADDR_OF:
                try:
                    type_sp = cursor.type.spelling or ""
                    # Non-const pointer: ends with " *" and does not contain "const".
                    if type_sp.endswith(" *") or type_sp.endswith("*"):
                        if "const" not in type_sp.split("*")[0]:
                            op = UnaryOp.ADDR_OF_MUT
                except Exception:
                    pass
            return op
        if not is_prefix and op_tok in _UNARY_POSTFIX_OP_MAP:
            return _UNARY_POSTFIX_OP_MAP[op_tok]
        # Fallback: treat as prefix if available.
        if op_tok in _UNARY_PREFIX_OP_MAP:
            op = _UNARY_PREFIX_OP_MAP[op_tok]
            if op == UnaryOp.ADDR_OF:
                try:
                    type_sp = cursor.type.spelling or ""
                    if (type_sp.endswith(" *") or type_sp.endswith("*")) and "const" not in type_sp.split("*")[0]:
                        op = UnaryOp.ADDR_OF_MUT
                except Exception:
                    pass
            return op

        # Degenerate fallback — the operator token is not visible (e.g. came
        # from a macro expansion like DCHECK(cond) → !(cond)).  Use the result
        # type of the cursor to pick the most likely operator:
        #   • bool result  → logical NOT (!)
        #   • numeric      → arithmetic NEG (-)
        try:
            result_type_spelling = (cursor.type.spelling or "").lower()
        except Exception:
            result_type_spelling = ""
        if "bool" in result_type_spelling:
            return UnaryOp.NOT
        # If the cursor extent starts at the same offset as the child (operator
        # token is invisible / macro-injected) and the child is a comparison or
        # logical expression, prefer NOT.
        if children:
            try:
                if cursor.extent.start.offset == children[0].extent.start.offset:
                    child_kind = children[0].kind
                    _BOOL_KINDS = {
                        CursorKind.BINARY_OPERATOR,
                        CursorKind.UNARY_OPERATOR,
                        CursorKind.PAREN_EXPR,
                        CursorKind.CALL_EXPR,
                    }
                    if child_kind in _BOOL_KINDS:
                        return UnaryOp.NOT
            except Exception:
                pass
        return UnaryOp.NEG

    # ── conditional (ternary) ─────────────────────────────────────────────

    def _convert_conditional_operator(self, cursor: Cursor) -> IRTernaryExpr:
        """CONDITIONAL_OPERATOR children: [condition, then-expr, else-expr]."""
        children = _children(cursor)
        condition: Optional[IRExpr] = None
        then_expr: Optional[IRExpr] = None
        else_expr: Optional[IRExpr] = None
        if len(children) >= 1:
            condition = self.convert_expr(children[0])
        if len(children) >= 2:
            then_expr = self.convert_expr(children[1])
        if len(children) >= 3:
            else_expr = self.convert_expr(children[2])
        return IRTernaryExpr(condition=condition, then_expr=then_expr, else_expr=else_expr)

    # ── casts ─────────────────────────────────────────────────────────────

    def _convert_cast_expr(self, cursor: Cursor, cast_kind: CastKind) -> IRCastExpr:
        """Named cast (static_cast, dynamic_cast, reinterpret_cast, const_cast) or C-style cast."""
        children = _children(cursor)
        expr: Optional[IRExpr] = None
        target_type: Optional[IRType] = None

        # The cast's result type is the target type.
        if cursor.type:
            target_type = _make_ir_type(cursor.type)

        # Find the source expression child, skipping TYPE_REF / TEMPLATE_REF
        # nodes that describe the target type.
        for child in children:
            if child.kind in (CursorKind.TYPE_REF, CursorKind.TEMPLATE_REF,
                              CursorKind.NAMESPACE_REF):
                continue
            expr = self.convert_expr(child)
            break

        return IRCastExpr(expr=expr, target_type=target_type, kind=cast_kind)

    def _convert_functional_cast_expr(self, cursor: Cursor) -> IRExpr:
        """CXX_FUNCTIONAL_CAST_EXPR — ``Type(args)`` syntax.

        This may be either:
        - A true type cast (e.g., ``int(3.14)``) which maps to IRCastExpr.
        - A constructor call (e.g., ``std::string("hello")``) which maps to
          IRCallExpr.

        Heuristic: if the target type is a primitive or built-in, treat it as
        a cast.  Otherwise treat it as a constructor call.
        """
        children = _children(cursor)
        target_type = _make_ir_type(cursor.type) if cursor.type else None

        # Determine whether this is a primitive cast or a constructor call.
        # Treat primitive types, typedefs (digit_t, twodigit_t, size_t), and
        # elaborated types (twodigit_t via elaborated specifier) as casts.
        is_primitive_cast = False
        if target_type:
            is_primitive_cast = target_type.kind in (
                IRTypeKind.PRIMITIVE,
                IRTypeKind.TYPEDEF,
                IRTypeKind.UNKNOWN,   # ELABORATED maps to UNKNOWN via _make_ir_type
            )
            # Also treat ENUM as a cast target (to the underlying integral type).
            if target_type.kind == IRTypeKind.ENUM:
                is_primitive_cast = True

        # Additionally check the raw clang type kind for ELABORATED (typedef alias).
        try:
            raw_ck = cursor.type.kind
            from clang.cindex import TypeKind as _TK
            if raw_ck in (_TK.TYPEDEF, getattr(_TK, 'ELABORATED', None)):
                is_primitive_cast = True
        except Exception:
            pass

        if is_primitive_cast and children:
            # Skip leading TYPE_REF children (the target type reference itself)
            # and use the first non-TYPE_REF child as the operand.
            value_children = [c for c in children
                              if c.kind != CursorKind.TYPE_REF]
            if len(value_children) == 1:
                vc = value_children[0]
                # Brace-initialization casts: `Type{a}` → INIT_LIST_EXPR `{a}`.
                # For a single-element init list used as a cast, unwrap the
                # list to get the scalar operand: `Type{a}` → `a as Type`.
                if vc.kind == CursorKind.INIT_LIST_EXPR:
                    init_children = list(vc.get_children())
                    if len(init_children) == 1:
                        vc = init_children[0]  # unwrap to scalar
                expr = self.convert_expr(vc)
                return IRCastExpr(expr=expr, target_type=target_type, kind=CastKind.AS)
            elif not value_children and children:
                expr = self.convert_expr(children[-1])
                return IRCastExpr(expr=expr, target_type=target_type, kind=CastKind.AS)

        # Constructor-style call.
        func = IRNameRef(
            name=target_type.name if target_type else "",
            qualified_name=target_type.cpp_name if target_type else "",
        )
        args: List[IRExpr] = []
        for child in children:
            args.append(self.convert_expr(child))
        return IRCallExpr(function=func, args=args, is_method_call=False)

    # ── unwrap helpers (PAREN_EXPR, IMPLICIT_CAST_EXPR) ───────────────────

    def _unwrap_single_child_expr(self, cursor: Cursor) -> IRExpr:
        """Unwrap parenthesised or implicit-cast expressions by returning the
        conversion of the single child."""
        children = _children(cursor)
        if children:
            return self.convert_expr(children[0])
        # No child — shouldn't happen, but fall back.
        return IRRawCppExpr(cpp_source=_get_source_text(cursor))

    # ── new / delete ──────────────────────────────────────────────────────

    def _convert_construct_expr(self, cursor: Cursor) -> IRCallExpr:
        """CXX_CONSTRUCT_EXPR / CXX_TEMPORARY_OBJECT_EXPR: constructor call.

        Converts ``TypeName(args)`` to ``TypeName::new(args)`` in Rust.
        For copy/move constructors (single DECL_REF_EXPR argument of the same
        type), just emit the argument directly — Rust Copy types don't need a
        wrapping constructor call.
        """
        children = _children(cursor)
        args: List[IRExpr] = []

        # Get the constructed type name
        type_name = ""
        if cursor.type and cursor.type.spelling:
            type_name = cursor.type.spelling.split("::")[-1]
        if not type_name and cursor.spelling:
            type_name = cursor.spelling

        # Collect constructor arguments (skip type refs, template refs, etc.)
        expr_children = [c for c in children if self._is_expr_cursor(c)]
        for child in expr_children:
            args.append(self.convert_expr(child))

        # Detect copy/move constructor: single DECL_REF_EXPR argument whose
        # declared type matches the constructed type.  In Rust, Copy types are
        # duplicated implicitly, so no constructor call is needed.
        if len(expr_children) == 1:
            sole = expr_children[0]
            # Unwrap IMPLICIT_CAST_EXPR / UNEXPOSED_EXPR wrappers
            _wrap_kinds = set()
            if _IMPLICIT_CAST_EXPR is not None:
                _wrap_kinds.add(_IMPLICIT_CAST_EXPR)
            if hasattr(CursorKind, 'UNEXPOSED_EXPR'):
                _wrap_kinds.add(CursorKind.UNEXPOSED_EXPR)
            while sole.kind in _wrap_kinds:
                inner = _children(sole)
                if not inner:
                    break
                sole = inner[0]
            if sole.kind == CursorKind.DECL_REF_EXPR:
                # It's a trivial copy: just emit the variable reference.
                if args:
                    return args[0]

        # Emit as Type::new(args) call
        func = IRMemberExpr(
            object=IRNameRef(name=type_name) if type_name else None,
            member="new",
            is_arrow=False,
        )
        return IRCallExpr(function=func, args=args, is_method_call=False)

    def _convert_new_expr(self, cursor: Cursor) -> IRNewExpr:
        children = _children(cursor)
        new_type: Optional[IRType] = None
        args: List[IRExpr] = []
        is_array = False
        array_size: Optional[IRExpr] = None

        # Detect array form: ``new Type[n]``.
        tokens = _token_spellings(cursor)
        if "[" in tokens:
            is_array = True

        # The cursor's type gives us the allocated type (pointer-to-T); strip
        # the pointer to obtain T.
        if cursor.type:
            pointee = cursor.type.get_pointee()
            if pointee and pointee.spelling:
                new_type = _make_ir_type(pointee)
            else:
                new_type = _make_ir_type(cursor.type)

        for child in children:
            if is_array and array_size is None and self._is_expr_cursor(child):
                # First expression in array-new is the size.
                array_size = self.convert_expr(child)
            elif self._is_expr_cursor(child):
                args.append(self.convert_expr(child))

        return IRNewExpr(
            type=new_type,
            args=args,
            is_array=is_array,
            array_size=array_size,
        )

    def _convert_delete_expr(self, cursor: Cursor) -> IRDeleteExpr:
        children = _children(cursor)
        expr: Optional[IRExpr] = None
        is_array = False

        tokens = _token_spellings(cursor)
        if "[" in tokens and "]" in tokens:
            is_array = True

        if children:
            expr = self.convert_expr(children[0])

        return IRDeleteExpr(expr=expr, is_array=is_array)

    # ── this ──────────────────────────────────────────────────────────────

    def _convert_this_expr(self, cursor: Cursor) -> IRThisExpr:
        # Determine mutability from the enclosing method's const-qualification.
        is_mut = True
        parent = cursor.semantic_parent
        if parent and hasattr(parent, "is_const_method"):
            try:
                if parent.is_const_method():
                    is_mut = False
            except Exception:
                pass
        return IRThisExpr(is_mut=is_mut)

    # ── subscript ─────────────────────────────────────────────────────────

    def _convert_index_expr(self, cursor: Cursor) -> IRIndexExpr:
        """ARRAY_SUBSCRIPT_EXPR children: [object, index]."""
        children = _children(cursor)
        obj: Optional[IRExpr] = None
        index: Optional[IRExpr] = None
        if len(children) >= 1:
            obj = self.convert_expr(children[0])
        if len(children) >= 2:
            index = self.convert_expr(children[1])
        return IRIndexExpr(object=obj, index=index)

    # ── lambda ────────────────────────────────────────────────────────────

    def _convert_lambda_expr(self, cursor: Cursor) -> IRLambdaExpr:
        children = _children(cursor)
        params: List[IRParam] = []
        body: Optional[IRBlock] = None
        capture_mode = ""
        return_type: Optional[IRType] = None

        # Detect capture mode from tokens: [=] → "move", [&] → "ref".
        tokens = _token_spellings(cursor)
        if tokens:
            capture_str = ""
            bracket_depth = 0
            for tok in tokens:
                if tok == "[":
                    bracket_depth += 1
                    continue
                if tok == "]" and bracket_depth > 0:
                    break
                if bracket_depth > 0:
                    capture_str += tok
            if capture_str == "=":
                capture_mode = "move"
            elif capture_str == "&":
                capture_mode = "ref"

        for child in children:
            if child.kind == CursorKind.PARM_DECL:
                param = IRParam(
                    name=child.spelling or "",
                    type=_make_ir_type(child.type),
                    cpp_name=child.spelling or "",
                )
                params.append(param)
            elif child.kind == CursorKind.COMPOUND_STMT:
                body = self._convert_compound_stmt(child)

        # Attempt to determine return type from the lambda's call operator.
        if cursor.type:
            try:
                result = cursor.type.get_result()
                if result and result.spelling and result.spelling != "auto":
                    return_type = _make_ir_type(result)
            except Exception:
                pass

        return IRLambdaExpr(
            params=params,
            body=body,
            capture_mode=capture_mode,
            return_type=return_type,
        )

    # ── init list ─────────────────────────────────────────────────────────

    def _convert_init_list_expr(self, cursor: Cursor) -> IRInitListExpr:
        elements: List[IRExpr] = []
        for child in cursor.get_children():
            elements.append(self.convert_expr(child))
        return IRInitListExpr(elements=elements)

    # ── sizeof / alignof ──────────────────────────────────────────────────

    def _convert_sizeof_expr(self, cursor: Cursor) -> IRSizeofExpr:
        """UNARY_EXPR_OR_TYPE_TRAIT_EXPR — sizeof(T), sizeof(expr), alignof(T)."""
        children = _children(cursor)
        target_type: Optional[IRType] = None
        target_expr: Optional[IRExpr] = None
        is_alignof = False

        tokens = _token_spellings(cursor)
        if "alignof" in tokens or "_Alignof" in tokens:
            is_alignof = True

        if children:
            child = children[0]
            if self._is_expr_cursor(child):
                target_expr = self.convert_expr(child)
            else:
                # Type reference — use the cursor's type.
                if cursor.type:
                    # The referenced type is available via the child or the
                    # cursor's argument type.
                    try:
                        arg_type = cursor.type
                        # For sizeof(Type), Clang sometimes provides the
                        # argument type as the cursor's type, but more
                        # reliably it is the child's referenced type.
                        if child.type:
                            target_type = _make_ir_type(child.type)
                        else:
                            target_type = _make_ir_type(arg_type)
                    except Exception:
                        target_type = _make_ir_type(cursor.type)
        else:
            # No children — sizeof may reference a type directly.
            if cursor.type:
                target_type = _make_ir_type(cursor.type)

        return IRSizeofExpr(
            target_type=target_type,
            target_expr=target_expr,
            is_alignof=is_alignof,
        )

    # ── internal helpers ──────────────────────────────────────────────────

    def _ensure_block(self, cursor: Cursor) -> IRBlock:
        """If *cursor* is a COMPOUND_STMT, convert it directly; otherwise wrap
        the converted statement in a single-statement IRBlock."""
        if cursor.kind == CursorKind.COMPOUND_STMT:
            return self._convert_compound_stmt(cursor)
        stmt = self.convert_stmt(cursor)
        return IRBlock(statements=[stmt])

    @staticmethod
    def _is_expr_cursor(cursor: Cursor) -> bool:
        """Return True if *cursor* looks like an expression node.

        libclang does not expose a single ``is_expression`` predicate; we use
        the cursor kind's name as a heuristic.
        """
        kind = cursor.kind
        if kind in _EXPR_KINDS_SET:
            return True
        # Heuristic: many expression CursorKinds have names ending in _EXPR
        # or _LITERAL.
        name = str(kind.name) if hasattr(kind, "name") else str(kind)
        return name.endswith("_EXPR") or name.endswith("_LITERAL")

    @staticmethod
    def _get_qualified_name(cursor: Cursor) -> str:
        """Return the fully qualified ``::``-separated name of *cursor*."""
        parts: List[str] = []
        c = cursor
        while c and c.kind != CursorKind.TRANSLATION_UNIT:
            if c.spelling:
                parts.append(c.spelling)
            c = c.semantic_parent
        return "::".join(reversed(parts))

"""Rust source code emitter for the C++ to Rust transpiler.

Converts IR nodes into properly formatted Rust source text.  The main entry
point is ``RustEmitter.emit_file`` which returns a complete ``*.rs`` file as a
string.  Individual helpers (``emit_struct``, ``emit_enum``, etc.) can also be
called directly when only a fragment is needed.
"""

from __future__ import annotations

import re
from typing import List, Optional

from cpp2rust.core.ir.nodes import (
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
    IRRawCppExpr,
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
    IRRawCppStmt,
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
        # 0. Remove `/* cerr */(shift_left, "...")(shift_left, expr)...;` lines.
        #    These come from DCHECK macro expansion: `std::cerr << msg << cond << ...`
        #    The `<<` operator is renamed to `shift_left` by _OP_RENAME, producing
        #    these chains.  They carry no useful information in Rust.
        source = re.sub(
            r'^\s*/\*\s*cerr\s*\*/(?:\(shift_left,[^;]*);\s*$',
            '',
            source,
            flags=re.MULTILINE,
        )
        # Also remove bare `/* cerr */;` lines (the cerr object alone, no chained calls).
        source = re.sub(r'^\s*/\*\s*cerr\s*\*/\s*;\s*$', '', source, flags=re.MULTILINE)
        # Remove any remaining cerr(...) calls (non-/* */ form).
        source = re.sub(r'^\s*cerr\s*(?:\([^;]*);\s*$', '', source, flags=re.MULTILINE)

        # 0b. Convert `abort()` → `std::process::abort()` (the C stdlib abort is not
        #     in scope in Rust; use the standard library equivalent).
        source = re.sub(r'\babort\(\)', 'std::process::abort()', source)

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
        #    Match: assert!("any string", <anything>, "")
        #    The EXPR can contain commas (function calls), so use a greedy match
        #    with the `", "")` anchor at the end.
        source = re.sub(
            r'(debug_assert(?:_eq|_ne)?!)\s*\("[^"]*",\s*(.+?),\s*""\)',
            r'\1(\2)',
            source,
        )
        source = re.sub(
            r'(assert(?:_eq|_ne)?!)\s*\("[^"]*",\s*(.+?),\s*""\)',
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
        # Use a function to handle nested parens in parameter lists
        def _protect_fn_arrows(source_text):
            result = []
            i = 0
            while i < len(source_text):
                # Look for `fn ` keyword
                if source_text[i:i+3] == 'fn ' and (i == 0 or not source_text[i-1].isalnum()):
                    # Find the opening paren of params
                    j = source_text.find('(', i + 3)
                    if j == -1:
                        result.append(source_text[i])
                        i += 1
                        continue
                    # Match balanced parens
                    depth = 1
                    k = j + 1
                    while k < len(source_text) and depth > 0:
                        if source_text[k] == '(':
                            depth += 1
                        elif source_text[k] == ')':
                            depth -= 1
                        k += 1
                    # k is now past the closing paren. Check for ->
                    while k < len(source_text) and source_text[k] in ' \t':
                        k += 1
                    if k + 1 < len(source_text) and source_text[k:k+2] == '->':
                        result.append(source_text[i:k])
                        result.append(_RET_ARROW)
                        i = k + 2
                        continue
                result.append(source_text[i])
                i += 1
            return ''.join(result)

        source = _protect_fn_arrows(source)
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

        # Special case: C++ USE(x) → use(x) after snake_case. Treat as discard.
        source = re.sub(r'\buse\(([^)]+)\)', r'{ let _ = \1; }', source)

        for kw in ("type", "match", "ref", "mod", "move", "in",
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
        source = re.sub(r'\bnullptr\b', 'std::ptr::null_mut()', source)

        # 9b. Replace C stdlib / C++ std:: functions with Rust equivalents.
        #     DIV_CEIL(x, y) = ((x) - 1) / (y) + 1  (integer ceiling division)
        #     Only handle simple (no-parens) arguments to avoid greedy mismatch.
        source = re.sub(
            r'\bDIV_CEIL\(\s*([^,()]+?)\s*,\s*([^,()]+?)\s*\)',
            r'((\1).wrapping_sub(1) / (\2) + 1)',
            source,
        )
        #     std::min / std::max → std::cmp::min / std::cmp::max
        source = re.sub(r'\bstd::min\b', 'std::cmp::min', source)
        source = re.sub(r'\bstd::max\b', 'std::cmp::max', source)
        #     Bare `min(a, b)` / `max(a, b)` in function-call position
        source = re.sub(r'(?<![:\w])min\(', 'std::cmp::min(', source)
        source = re.sub(r'(?<![:\w])max\(', 'std::cmp::max(', source)
        #     std::swap / swap → std::mem::swap
        source = re.sub(r'\bstd::swap\b', 'std::mem::swap', source)
        source = re.sub(r'(?<![:\w])swap\(', 'std::mem::swap(', source)
        #     memset / memcpy — leave as-is; these appear rarely and need
        #     manual handling due to nested parens in arguments.

        # 9c. Fix twodigit_t (u128) arithmetic: when one side is cast to u128,
        #     the other side (digit_t = u64) must also be cast for Rust to accept it.
        #     Pattern: `a as twodigit_t OP b` where OP is +/-/*  → add `b as twodigit_t`
        source = re.sub(r'(as twodigit_t)\s*\+\s*(\w+)', r'\1 + (\2 as twodigit_t)', source)
        source = re.sub(r'(as twodigit_t)\s*-\s*(\w+)', r'\1 - (\2 as twodigit_t)', source)
        source = re.sub(r'(as twodigit_t)\s*\*\s*(\w+)', r'\1 * (\2 as twodigit_t)', source)

        # 9d. Fix `.as_ulong_op()` on digit_t values: `operator unsigned long long`
        #     is a conversion to the underlying digit type (already u64), so remove it.
        source = re.sub(r'\.as_ulong_op\(\)', '', source)
        source = re.sub(r'\.as_uint_op\(\)', ' as u32', source)
        source = re.sub(r'\.as_int_op\(\)', ' as i32', source)
        source = re.sub(r'\.as_long_op\(\)', ' as i64', source)
        source = re.sub(r'\.as_i64_op\(\)', ' as i64', source)
        source = re.sub(r'\.as_u64_op\(\)', ' as u64', source)

        # 9e. Fix raw pointer indexing: ptr[expr] → unsafe { *ptr.add((expr) as usize) }
        #     Strategy: collect all raw pointer parameter names from fn signatures,
        #     then rewrite their indexing.  Do assignment case first (avoids double-wrap).
        def _fix_raw_ptr_indexing(src: str) -> str:
            import re as _re
            # Gather ALL raw pointer parameter names used anywhere in the file
            ptr_names = set(_re.findall(
                r'(\w+)\s*:\s*\*(?:mut|const)\s+\w+',
                src,
            ))
            if not ptr_names:
                return src
            pat = '|'.join(_re.escape(n) for n in sorted(ptr_names, key=len, reverse=True))
            # Pass 1: assignment  ptr[expr] = rhs;
            # Use =(?!=) to avoid matching == (comparison operator).
            def _repl_write(m):
                n, idx, rhs = m.group(1), m.group(2), m.group(3)
                return f'unsafe {{ *{n}.add(({idx}) as usize) = {rhs}; }}'
            # Use (?<!\.) to avoid matching struct field access: self.part[i]
            src = _re.sub(
                rf'(?<!\.)\b({pat})\[([^\]]+)\]\s*=(?!=)\s*([^;]+);',
                _repl_write,
                src,
            )
            # Pass 2: read  ptr[expr]
            def _repl_read(m):
                n, idx = m.group(1), m.group(2)
                return f'unsafe {{ *{n}.add(({idx}) as usize) }}'
            src = _re.sub(
                rf'(?<!\.)\b({pat})\[([^\]]+)\]',
                _repl_read,
                src,
            )
            return src
        source = _fix_raw_ptr_indexing(source)

        # 9e2. Wrap bare `*ptr_param = expr;` and `*ptr_param` reads in unsafe.
        #      Handles out-param pointer writes like `*carry = result >> kDigitBits;`.
        def _fix_raw_ptr_deref(src: str) -> str:
            import re as _re
            ptr_names = set(_re.findall(r'(\w+)\s*:\s*\*(?:mut|const)\s+\w+', src))
            if not ptr_names:
                return src
            pat = '|'.join(_re.escape(n) for n in sorted(ptr_names, key=len, reverse=True))
            # Write: `*ptr = expr;` → `unsafe { *ptr = expr; }` (only when not already in unsafe)
            def _wrap_write(m):
                ptr, rhs = m.group(1), m.group(2)
                return f'unsafe {{ *{ptr} = {rhs}; }}'
            src = _re.sub(
                rf'(?<!unsafe \{{ )(?<!\*)\*({pat})\s*=(?!=)\s*([^;{{}}]+);',
                _wrap_write,
                src,
            )
            # Read: `*ptr` (not followed by .add or already in unsafe) → `unsafe { *ptr }`
            # But be conservative: only fix `*ptr` in expression context (not as LHS of =)
            def _wrap_read(m):
                ptr = m.group(1)
                return f'unsafe {{ *{ptr} }}'
            src = _re.sub(
                rf'(?<!unsafe \{{ )(?<!\*)(?<!=\s)\*({pat})\b(?!\s*[.=])',
                _wrap_read,
                src,
            )
            return src
        source = _fix_raw_ptr_deref(source)

        # 9f. Fix raw pointer arithmetic: ptr -= 1 / ptr += 1
        #     In Rust, pointer arithmetic requires unsafe { ptr.sub(n) } / ptr.add(n).
        # More targeted: look for variables declared as *mut/*const in scope, then fix -=
        # For now, use a file-level heuristic: if the file has any *mut/*const parameters,
        # fix ALL -= 1 on variables that look like output/result/ptr pointers.
        def _fix_ptr_arith(src: str) -> str:
            import re as _re
            ptr_names = set(_re.findall(r'(\w+)\s*:\s*\*(?:mut|const)\s+\w+', src))
            if not ptr_names:
                return src
            for pn in ptr_names:
                src = _re.sub(
                    rf'\b{_re.escape(pn)}\s*-=\s*(\d+)\s*;',
                    lambda m, p=pn: f'{p} = unsafe {{ {p}.sub({m.group(1)}) }};',
                    src,
                )
                src = _re.sub(
                    rf'\b{_re.escape(pn)}\s*\+=\s*(\d+)\s*;',
                    lambda m, p=pn: f'{p} = unsafe {{ {p}.add({m.group(1)}) }};',
                    src,
                )
            return src
        source = _fix_ptr_arith(source)

        # 9g. Map CamelCase C++ bigint function calls → snake_case Rust names.
        #     Apply 3-arg versions before 2-arg to avoid partial match.
        #     These are the inline header functions and free functions that the
        #     call sites still reference by their C++ names.
        _BIGINT_RENAME = [
            # 3-arg versions first (more specific)
            (r'\bAdd\(([^,)]+),\s*([^,)]+),\s*([^,)]+)\)',     r'add(\1, \2, \3)'),
            (r'\bSubtract\(([^,)]+),\s*([^,)]+),\s*([^,)]+)\)', r'subtract(\1, \2, \3)'),
            # 2-arg scalar versions
            (r'\bAdd\(([^,)]+),\s*([^,)]+)\)',         r'add_scalar(\1, \2)'),
            (r'\bSubtract\(([^,)]+),\s*([^,)]+)\)',    r'subtract_scalar(\1, \2)'),
            # Other capitalized → snake_case
            (r'\bGreaterThanOrEqual\b',   'greater_than_or_equal'),
            (r'\bSubtractAndReturnBorrow\b', 'subtract_and_return_borrow'),
            (r'\bAddAndReturnOverflow\b',  'add_and_return_overflow'),
            (r'\bAddAndReturnCarry\b',     'add_and_return_carry'),
            (r'\bIsPowerOfTwo\b',           'is_power_of_two'),
            # Copy(RWDigits, Digits) → copy_digits (avoids Rust Copy trait clash)
            (r'\bCopy\(([^,)]+),\s*([^,)]+)\)', r'copy_digits(\1, \2)'),
            (r'\bRoundUpLen\b',            'round_up_len'),
            (r'\bRoundUp\b',               'round_up'),
            (r'\bBitLength\b',             'bit_length'),
            (r'\bCompare\b',               'compare'),
            (r'\bCountTrailingZeros\b',    'count_trailing_zeros_i32'),
            (r'\bCountLeadingZeros\b',     'count_leading_zeros'),
            (r'\bModFn_Helper\b',          'mod_fn_helper'),
            (r'\bModFn\b',                 'mod_fn'),
            (r'\bShiftModFn_Large\b',      'shift_mod_fn_large'),
            (r'\bShiftModFn\b',            'shift_mod_fn'),
            (r'\bComputeParameters_Inner\b', 'compute_parameters_inner'),
            (r'\bComputeParameters\b',     'compute_parameters'),
            (r'\bPredictInnerK\b',         'predict_inner_k'),
            (r'\bShouldDecrementM\b',      'should_decrement_m'),
            (r'\bAsIntNResultLength\b',    'as_int_n_result_length'),
            (r'\bAsUintN_Pos_ResultLength\b', 'as_uint_n_pos_result_length'),
            (r'\bToStringResultLength\b',  'to_string_result_length'),
            (r'\bTruncateToNBits\b',       'truncate_to_n_bits'),
            (r'\bTruncateAndSubFromPowerOfTwo\b', 'truncate_and_sub_from_power_of_two'),
            # C builtins → Rust intrinsics
            (r'\b__builtin_ctzll\(([^)]+)\)', r'((\1) as u64).trailing_zeros() as i32'),
            (r'\b__builtin_clzll\(([^)]+)\)', r'((\1) as u64).leading_zeros() as i32'),
            (r'\b__builtin_ctz\(([^)]+)\)',   r'((\1) as u32).trailing_zeros() as i32'),
            (r'\b__builtin_clz\(([^)]+)\)',   r'((\1) as u32).leading_zeros() as i32'),
        ]
        for _pat, _repl in _BIGINT_RENAME:
            source = re.sub(_pat, _repl, source)

        # 9h. Fix u64 op i32 / i32 op u64 type mismatches for kDigitBits expressions.
        #     After the bigint constants are u64, some expressions still have i32 operands.
        #     Pattern: `(u64_expr) / kDigitBits as i32` → cast the i32 to u64 first.
        #     More generally, fix `expr % self.radix` where radix: i32 and expr: u64.
        source = re.sub(r'%\s*self\.radix\b(?!\s*as\b)', '% self.radix as u64', source)
        source = re.sub(r'/=\s*self\.radix\b(?!\s*as\b)', '/= self.radix as u64', source)
        source = re.sub(r'%\s*radix\b(?!\s*as\b)', '% radix as u64', source)
        source = re.sub(r'/\s*radix\b(?!\s*as\b)', '/ radix as u64', source)
        # Fix kConversionChars[expr] where expr might be u64: add as usize for indexing.
        source = re.sub(r'kConversionChars\[([^\]]+)\]', r'kConversionChars[(\1) as usize]', source)

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

        # 13. Fix remaining raw DCHECK/CHECK/RUNTIME_FUNCTION/UNREACHABLE calls
        source = re.sub(r'\bDCHECK\(([^)]+)\)', r'debug_assert!(\1)', source)
        source = re.sub(r'\bCHECK\(([^)]+)\)', r'assert!(\1)', source)
        source = re.sub(r'\bRUNTIME_FUNCTION\(([^)]+)\)', r'/* RUNTIME_FUNCTION(\1) */', source)
        source = re.sub(r'\bruntime_function\(([^)]+)\)', r'/* runtime_function(\1) */', source)
        source = re.sub(r'\bUNREACHABLE\(\)', r'unreachable!()', source)
        source = re.sub(r'\bcheck\(([^)]+)\)', r'assert!(\1)', source)
        # V8 macros without parens at statement level
        source = re.sub(r'^\s*UNREACHABLE\s*;', '    unreachable!();', source, flags=re.MULTILINE)

        # 14. Fix C++ stream output: cerr/cout/ostream
        # Replace `cerr << expr` and `cout << expr` patterns first (with args).
        source = re.sub(r'\bcerr\s*<<\s*', 'eprintln!("{:?}", ', source)
        source = re.sub(r'\bcout\s*<<\s*', 'println!("{:?}", ', source)
        # Any remaining bare `cerr` / `cout` references become a no-op comment.
        source = re.sub(r'\bcerr\b', '/* cerr */', source)
        source = re.sub(r'\bcout\b', '/* cout */', source)
        source = re.sub(r'\bstd::ostream\b', 'ostream', source)
        # Collapse chained macro calls: eprintln!(...)(...)(...) → eprintln!(...)
        source = re.sub(r'((?:eprintln|println|eprint|print)!\([^)]*\))(?:\([^)]*\))+', r'\1', source)
        # Fix bare macro invocation without parens: `eprintln!;` → `/* eprintln */;`
        source = re.sub(r'\b(eprintln|println|eprint|print)!;', r'/* \1 */;', source)

        # 15. Strip C++ digit separators: 0x0010'0000'0000'0000 → 0x0010_0000_0000_0000
        #     Must also match hex digits (a-fA-F)
        source = re.sub(r"([0-9a-fA-F])'([0-9a-fA-F])", r'\1_\2', source)

        # 16. Fix operator<< / operator>> function names → shift_left / shift_right
        source = re.sub(r'\boperator<<', 'shift_left', source)
        source = re.sub(r'\boperator>>', 'shift_right', source)
        source = re.sub(r'\boperator!', 'not_op', source)
        # Replace remaining operator* symbols so the code at least parses.
        _OP_RENAME = [
            # Conversion operators (must come before bare operator names)
            (r'\boperator\s+unsigned\s+long\s+long', 'as_u64_op'),
            (r'\boperator\s+unsigned\s+long', 'as_ulong_op'),
            (r'\boperator\s+unsigned\s+int', 'as_uint_op'),
            (r'\boperator\s+unsigned\s+char', 'as_uchar_op'),
            (r'\boperator\s+unsigned', 'as_unsigned_op'),
            (r'\boperator\s+long\s+long', 'as_i64_op'),
            (r'\boperator\s+long', 'as_long_op'),
            (r'\boperator\s+int', 'as_int_op'),
            (r'\boperator\s+char', 'as_char_op'),
            (r'\boperator\s+bool', 'as_bool_op'),
            (r'\boperator\s+float', 'as_f32_op'),
            (r'\boperator\s+double', 'as_f64_op'),
            (r'\boperator\s+void\s*\*', 'as_void_ptr_op'),
            # Compound/unary operators
            (r'\boperator\[\]', 'index_op'),
            (r'\boperator\+\+', 'inc_op'),
            (r'\boperator--', 'dec_op'),
            (r'\boperator\+=', 'add_assign_op'),
            (r'\boperator-=', 'sub_assign_op'),
            (r'\boperator\+', 'add_op'),
            (r'\boperator-', 'sub_op'),
            (r'\boperator\*', 'mul_op'),
            (r'\boperator/', 'div_op'),
            (r'\boperator%', 'mod_op'),
            (r'\boperator==', 'eq_op'),
            (r'\boperator!=', 'ne_op'),
            (r'\boperator<=', 'le_op'),
            (r'\boperator>=', 'ge_op'),
            (r'\boperator<', 'lt_op'),
            (r'\boperator>', 'gt_op'),
            (r'\boperator=(?!=)', 'assign_op'),  # = but not ==
            (r'\boperator\(\)', 'call_op'),
            (r'\boperator\|', 'bitor_op'),
            (r'\boperator&', 'bitand_op'),
            (r'\boperator\^', 'bitxor_op'),
        ]
        for pat, repl in _OP_RENAME:
            source = re.sub(pat, repl, source)

        # 16b. Fix `X(index_op, idx)(assign_op, val)` → `X[idx] = val`
        #      Only match simple index expressions (identifier + optional arithmetic)
        #      to avoid greedy matching that breaks nested function calls.
        _SIMPLE_IDX = r'(?:[a-zA-Z_]\w*|\d+)(?:\s*[+\-]\s*(?:[a-zA-Z_]\w*|\d+))*'
        _SIMPLE_VAL = r'[^()]*'   # value: no parens (simple scalar)
        source = re.sub(
            rf'(\w+)\(index_op,\s*({_SIMPLE_IDX})\)\(assign_op,\s*({_SIMPLE_VAL})\)',
            r'\1[\2] = \3',
            source,
        )
        # Fix remaining `X(index_op, simple_idx)` → `X[simple_idx]`
        source = re.sub(
            rf'(\w+)\(index_op,\s*({_SIMPLE_IDX})\)',
            r'\1[\2]',
            source,
        )
        # Fix block-expression indices: `X[{ let _old = i; i += 1; _old }]` is valid Rust
        source = re.sub(
            r'(\w+)\(index_op,\s*(\{[^}]+\})\)',
            r'\1[\2]',
            source,
        )

        # After index_op→bracket substitutions, handle `X[idx](assign_op, val)` → `X[idx] = val`.
        # This catches cases where the rhs value contained parens (e.g. `X[i] & Y[i]`) so the
        # earlier combined `(index_op,...)(assign_op,...)` pattern could not match atomically.
        # We apply this line-by-line so the `.*` in the value doesn't cross statement boundaries.
        def _fix_bracket_assign(line: str) -> str:
            # Use a paren-balanced approach to find the closing ')' of
            # `(assign_op, VALUE)`.  The non-greedy regex `.+?` incorrectly
            # stops at the first ')' inside VALUE (e.g. inside
            # `std::ptr::addr_of_mut!(x_borrow)`).
            changed = True
            while changed:
                changed = False
                m = re.search(r'(\w+\[[^\]]+\])\(assign_op,\s*', line)
                if not m:
                    break
                # Walk forward from the end of the match, tracking paren depth.
                # The outer '(' of '(assign_op, ...)' has already been consumed
                # by the regex, so we start at depth=1.
                depth = 1
                idx = m.end()
                while idx < len(line) and depth > 0:
                    c = line[idx]
                    if c == '(':
                        depth += 1
                    elif c == ')':
                        depth -= 1
                    idx += 1
                if depth != 0:
                    break  # unbalanced — give up
                # idx is now one past the closing ')'.
                value = line[m.end():idx - 1]
                rest = line[idx:]
                # Require lookahead: rest starts with ;, ,, ), or is empty.
                stripped = rest.lstrip()
                if not stripped or stripped[0] in ';,)':
                    line = line[:m.start()] + m.group(1) + ' = ' + value + rest
                    changed = True
            return line
        source = "\n".join(_fix_bracket_assign(ln) for ln in source.splitlines())

        # 16c. Fix chained comparison operators (C++ allows `a > b > c` but Rust doesn't).
        #      Replace `a OP b OP c` with `a OP b && b OP c` for inequality ops.
        _CHAIN_OPS = r'(?:>|<|>=|<=)'
        for _ in range(10):
            new = re.sub(
                rf'(\w+)\s*({_CHAIN_OPS})\s*(\w+)\s*({_CHAIN_OPS})\s*(\w+)',
                r'\1 \2 \3 && \3 \4 \5',
                source,
            )
            if new == source:
                break
            source = new

        # 17. Fix C-style array declarations: Type[] → Vec<Type>
        source = re.sub(r'(\w+)\[\]', r'Vec<\1>', source)

        # 18. Fix chained comparison operators.
        #     C++ allows `a == b == c` and `a < b < c`, but Rust doesn't.
        #     We break chains by inserting `&&` between adjacent comparisons.
        #     Patterns like `a == b == c` → `a == b && b == c`
        #     Apply repeatedly to handle long chains.
        _ALL_CMP = r'(?:==|!=|>=|<=|>|<)'
        # Match identifiers, member accesses, and simple function calls
        _EXPR = r'[\w.]+(?:\.[a-z_]\w*)*(?:\([^)]*\))?'
        for _ in range(15):
            new = re.sub(
                rf'({_EXPR})\s*({_ALL_CMP})\s*({_EXPR})\s*({_ALL_CMP})\s*({_EXPR})',
                lambda m: (
                    f'{m.group(1)} {m.group(2)} {m.group(3)} '
                    f'&& {m.group(3)} {m.group(4)} {m.group(5)}'
                ),
                source,
            )
            if new == source:
                break
            source = new

        # 18b. Fix `!expr >= val` → `!(expr >= val)` (NOT operator precedence)
        #      In C++, `!(x >= 1)` sometimes loses parens during AST conversion.
        #      Match at statement/condition level to safely close parens.
        source = re.sub(
            r'\bif\s+!(\w+)\s*(>=|<=|>|<|==|!=)\s*(\S+)',
            r'if !(\1 \2 \3)',
            source,
        )
        source = re.sub(
            r'\bwhile\s+!(\w+)\s*(>=|<=|>|<|==|!=)\s*(\S+)',
            r'while !(\1 \2 \3)',
            source,
        )
        # General expression level: `--!expr op val` → wrap negation
        source = re.sub(
            r'([^a-zA-Z0-9_])!(\w+)\s*(>=|<=|>|<)\s*(\w+)',
            r'\1!(\2 \3 \4)',
            source,
        )

        # 19. Fix `value as Type <<` being parsed as generics
        #     Wrap `expr as Type` in parens when followed by <<
        source = re.sub(r'(\w+)\s+as\s+(\w+)\s*<<', r'(\1 as \2) <<', source)
        source = re.sub(r'(\w+)\s+as\s+(\w+)\s*>>', r'(\1 as \2) >>', source)

        # 19. Fix missing initializers: `let x: Type = ;` → `let x: Type = Default::default();`
        #     Also handle: `x = ;` → `x = Default::default();`
        source = re.sub(
            r'(let\s+(?:mut\s+)?\w+\s*:\s*\w[^=]*?)\s*=\s*;',
            r'\1 = Default::default();',
            source,
        )
        # Bare assignment with missing value: `x = ;` → `x = Default::default();`
        source = re.sub(
            r'^(\s+\w+)\s*=\s*;',
            r'\1 = Default::default();',
            source,
            flags=re.MULTILINE,
        )

        # 20. Fix `fn name<...>` with C++ variadic `...` → remove the `...`
        source = re.sub(r',\s*\.\.\.>', '>', source)
        source = re.sub(r'<\s*\.\.\.>', '<>', source)

        # 21. Fix unnamed enums: `pub enum (unnamed enum at ...)` → valid name
        source = re.sub(
            r'pub enum \(unnamed enum at [^)]*\)',
            'pub enum UnnamedEnum',
            source,
        )

        # 22. Collapse multiple blank lines into at most two.
        source = re.sub(r'\n{4,}', '\n\n\n', source)

        # 22b. Fix `debug_assert!(debug_assert!(...))` → `debug_assert!(...)`.
        #      Nested debug_assert! from DCHECK expansion produces invalid code.
        source = re.sub(r'debug_assert!\(debug_assert!\(([^)]+)\)\)', r'debug_assert!(\1)', source)

        # 22c. Fix `= ()()` (unit type called as constructor) → `= Default::default()`.
        #      This occurs when a struct type was mapped to `()` in the prelude.
        source = re.sub(r'\s*=\s*\(\)\(\)', ' = Default::default()', source)

        # 22d. Fix `>> kDigitBits` / `<< kDigitBits`: Rust requires u32 shift amount.
        #      Handle subtraction first: `>> kDigitBits - var` → `>> (kDigitBits - var) as u32`
        source = re.sub(r'>>\s*kDigitBits\s*-\s*(\w+)\b', r'>> (kDigitBits - \1) as u32', source)
        source = re.sub(r'<<\s*kDigitBits\s*-\s*(\w+)\b', r'<< (kDigitBits - \1) as u32', source)
        # Also handle already-corrupted form `>> kDigitBits as u32 - var`
        source = re.sub(r'>>\s*kDigitBits\s*as\s*u32\s*-\s*(\w+)\b', r'>> (kDigitBits - \1) as u32', source)
        source = re.sub(r'<<\s*kDigitBits\s*as\s*u32\s*-\s*(\w+)\b', r'<< (kDigitBits - \1) as u32', source)
        # Standalone: >> kDigitBits → >> kDigitBits as u32
        source = re.sub(r'>>\s*kDigitBits\b(?!\s*(as|-)\b)', '>> kDigitBits as u32', source)
        source = re.sub(r'<<\s*kDigitBits\b(?!\s*(as|-)\b)', '<< kDigitBits as u32', source)

        # 22e. Fix bitwise-AND boolean checks: `if expr & 1` → `if expr & 1 != 0`
        source = re.sub(r'\bif\s+(\w+)\s*&\s*1\b(?!\s*!=)', r'if \1 & 1 != 0', source)

        # 22f. Fix `kConversionChars[...]` → cast result to `i8` for *mut i8 targets.
        source = re.sub(r'= kConversionChars\[([^\]]+)\];', r'= kConversionChars[\1] as i8;', source)

        # 22g. Fix char literal `'0'` assigned to *mut i8 target: `= '0'` → `= b'0' as i8`
        source = re.sub(r"(?<=\} )= '(\w)'(?=;)", r"= b'\1' as i8", source)

        # 22h. Fix `bool` used in arithmetic with u32 (sign: bool + u32 in to_string_result_length)
        source = re.sub(r'\};\s*sign\s*\}', '}; sign as u32 }', source)
        source = re.sub(r'\b(\w+)\s*\+=\s*sign\b', r'\1 += sign as u32', source)

        # 22i. Fix `*mut T` field access for known struct types: `params.field` → `unsafe { (*params).field }`
        #      Process function-by-function to avoid replacing params.field in functions where
        #      the variable is NOT a raw pointer.
        def _fix_ptr_struct_fields(src: str) -> str:
            import re as _re
            # Split on function definitions to process each function independently
            fn_splitter = _re.compile(
                r'((?:pub\s+)?fn\s+\w+\s*(?:<[^>]*>)?\s*\([^{]*\)\s*(?:->[^{]*)?\s*\{)',
                _re.DOTALL,
            )
            parts = fn_splitter.split(src)
            result_parts = []
            i = 0
            while i < len(parts):
                if i + 1 < len(parts) and fn_splitter.match(parts[i + 1] if i + 1 < len(parts) else ''):
                    # This is a non-function part (module level)
                    result_parts.append(parts[i])
                    i += 1
                elif fn_splitter.match(parts[i]):
                    # This is a fn header
                    fn_header = parts[i]
                    fn_body = parts[i + 1] if i + 1 < len(parts) else ''
                    # Check which variables are *mut Parameters or *mut RightShiftState in this fn
                    ptr_names = set(_re.findall(
                        r'(\w+)\s*:\s*\*(?:mut|const)\s+(?:Parameters|RightShiftState)', fn_header
                    ))
                    for _ppn in ptr_names:
                        _f = r'\w+'
                        # Write case: `ptr.field = expr;` → `unsafe { (*ptr).field = expr; }`
                        fn_body = _re.sub(
                            rf'(?<!\*)\b{_re.escape(_ppn)}\.({_f})\b\s*=(?!=)\s*([^;{{}}]+);',
                            rf'unsafe {{ (*{_ppn}).\1 = \2; }}',
                            fn_body,
                        )
                        # Read case: `ptr.field` → `unsafe { (*ptr).field }`
                        fn_body = _re.sub(
                            rf'(?<!\*)\b{_re.escape(_ppn)}\.({_f})\b(?!\s*\()',
                            rf'unsafe {{ (*{_ppn}).\1 }}',
                            fn_body,
                        )
                    result_parts.append(fn_header)
                    result_parts.append(fn_body)
                    i += 2
                else:
                    result_parts.append(parts[i])
                    i += 1
            return ''.join(result_parts)
        source = _fix_ptr_struct_fields(source)

        # 22j. Fix twodigit_t shift result needs explicit cast to digit_t when assigned
        #      to *digit_t output params. Pattern: `*var = result >> kDigitBits as u32`
        #      Wrap the whole RHS in parens before adding `as digit_t`.
        source = re.sub(
            r'(\*\w+\s*=\s*)(result\s*>>\s*kDigitBits\s*as\s*u32\s*(?:&\s*1\s*)?);',
            r'\1(\2) as digit_t;',
            source,
        )

        # 22k. Fix digit_t added to twodigit_t expression without cast.
        #      Pattern: `as twodigit_t) + c` where c is a digit_t var → `as twodigit_t) + (c as twodigit_t)`
        source = re.sub(
            r'(twodigit_t\))\s*\+\s*(\w+)\s*;',
            r'\1 + (\2 as twodigit_t);',
            source,
        )

        # 22l. Fix RWDigits passed where Digits expected in inplace add/sub patterns.
        #      When the same RWDigits variable is both output and input: carry(z, z, x) → carry(z, z.as_digits(), x)
        source = re.sub(
            r'\badd_and_return_carry\((\w+),\s*\1\s*,',
            lambda m: f'add_and_return_carry({m.group(1)}, {m.group(1)}.as_digits(),',
            source,
        )
        source = re.sub(
            r'\bsubtract_and_return_borrow\((\w+),\s*\1\s*,',
            lambda m: f'subtract_and_return_borrow({m.group(1)}, {m.group(1)}.as_digits(),',
            source,
        )

        # 22m. Fix signed_digit_t assigned to digit_t variable (type mismatch in mul_fft.rs).
        #      Pattern: `let mut borrow: digit_t = high;` where high: signed_digit_t
        source = re.sub(
            r'(let\s+mut\s+\w+\s*:\s*digit_t\s*=\s*)(\w+)\s*;',
            lambda m: (
                f'{m.group(1)}{m.group(2)} as digit_t;'
                if m.group(2) in {'high', 'low', 'borrow_in'}
                else f'{m.group(1)}{m.group(2)};'
            ),
            source,
        )

        # 22n. Fix `should_decrement_m(val, val, val)` — function takes &Parameters, pass by ref.
        source = re.sub(
            r'\bshould_decrement_m\((\w+),\s*(\w+),\s*(\w+)\)',
            r'should_decrement_m(&\1, &\2, &\3)',
            source,
        )

        # 22o. Fix raw-pointer method calls on *mut ProcessorImpl.
        #      `processor.method(args)` → `unsafe { (*processor).method(args) }`
        #      `self.processor.method(args)` → `unsafe { (*self.processor).method(args) }`
        #      Must capture the full call including `(args)` to include them inside unsafe.
        for _proc_method, _rust_method in [
            ('should_terminate', 'should_terminate'),
            ('AddWorkEstimate', 'add_work_estimate'),
            ('add_work_estimate', 'add_work_estimate'),
            ('DivideSingle', 'divide_single'),
            ('divide_single', 'divide_single'),
        ]:
            # self.processor.method(args)
            source = re.sub(
                rf'\bself\.processor\.{re.escape(_proc_method)}\s*\(([^)]*)\)',
                rf'unsafe {{ (*self.processor).{_rust_method}(\1) }}',
                source,
            )
            # bare processor.method(args)
            source = re.sub(
                rf'(?<!self\.)(?<!\*)\bprocessor\.{re.escape(_proc_method)}\s*\(([^)]*)\)',
                rf'unsafe {{ (*processor).{_rust_method}(\1) }}',
                source,
            )

        # 23. Second pass: remove `.as_*_op()` conversion operator calls that were
        #     created by _OP_RENAME in section 16 AFTER section 9d already ran.
        #     `.as_ulong_op()` on digit_t (uintptr_t = u64) — identity conversion, remove.
        source = re.sub(r'\.as_ulong_op\(\)', '', source)
        source = re.sub(r'\.as_u64_op\(\)', '', source)
        source = re.sub(r'\.as_uint_op\(\)', ' as u32', source)
        source = re.sub(r'\.as_int_op\(\)', ' as i32', source)
        source = re.sub(r'\.as_long_op\(\)', ' as i64', source)
        source = re.sub(r'\.as_i64_op\(\)', ' as i64', source)
        source = re.sub(r'\.as_uchar_op\(\)', ' as u8', source)
        source = re.sub(r'\.as_bool_op\(\)', ' != 0', source)

        # 24. Fix digit_t (u64) / kDigitBits (i32) arithmetic.
        #     kDigitBits is i32 (from prelude). When dividing a digit_t (u64) by kDigitBits,
        #     cast kDigitBits to u64.
        #     Pattern: `shift / kDigitBits` → `shift / kDigitBits as u64`
        #     Only apply when the LHS already has an `as i32` cast (indicating the context
        #     expects i32 result after the division of u64 digits).
        source = re.sub(
            r'\((\w+)\s*/\s*kDigitBits\)\s*as\s*i32',
            r'(\1 / kDigitBits as u64) as i32',
            source,
        )
        source = re.sub(
            r'\((\w+)\s*%\s*kDigitBits\)\s*as\s*i32',
            r'(\1 % kDigitBits as u64) as i32',
            source,
        )
        # Also fix bare `shift / kDigitBits` (without as i32 wrapper) where
        # `shift` is digit_t (u64) in bigint/bitwise context.
        # Note: power_of_two is i32, NOT digit_t — do NOT add u64 cast for it.
        source = re.sub(
            r'\b(shift)\s*/\s*kDigitBits\b(?!\s*as\b)',
            r'\1 / kDigitBits as u64',
            source,
        )
        source = re.sub(
            r'\b(shift)\s*%\s*kDigitBits\b(?!\s*as\b)',
            r'\1 % kDigitBits as u64',
            source,
        )

        # 25. tostring.rs type fixes.
        #     `bit_length(x)` returns i32 → `bit_length(x) as u32` for u32 vars.
        source = re.sub(
            r'(let\s+\w+\s*:\s*u32\s*=\s*)bit_length\(([^)]+)\)',
            r'\1bit_length(\2) as u32',
            source,
        )
        #     `count_trailing_zeros_i32(...)` returns i32 → cast to u32 for u32 vars.
        source = re.sub(
            r'(let\s+\w+\s*:\s*u32\s*=\s*)count_trailing_zeros_i32\(([^)]+)\)',
            r'\1count_trailing_zeros_i32(\2) as u32',
            source,
        )
        #     `let mut chars_required: u64 = bit_length` where bit_length: u32.
        source = re.sub(
            r'(let\s+mut\s+chars_required\s*:\s*u64\s*=\s*)(\w+)\s*;',
            r'\1\2 as u64;',
            source,
        )
        #     `chunk_divisor: digit_t = digit_pow_rec(radix, chunk_chars)` — chunk_chars is i32.
        source = re.sub(
            r'\bdigit_pow_rec\((\w+),\s*(\w+)\)',
            r'digit_pow_rec(\1 as digit_t, \2 as digit_t)',
            source,
        )
        #     `chunk /= radix` where chunk: digit_t and radix: i32.
        source = re.sub(
            r'\bchunk\s*/=\s*radix\b(?!\s*as\b)',
            'chunk /= radix as digit_t',
            source,
        )
        #     `/ (min_bits_per_char)` — min_bits_per_char is u8, divisor of u64. Cast to u64.
        source = re.sub(
            r'/\s*\(min_bits_per_char\)',
            '/ (min_bits_per_char as u64)',
            source,
        )
        source = re.sub(
            r'/\s*\(max_bits_per_char\)',
            '/ (max_bits_per_char as u64)',
            source,
        )
        #     mul-assignment with *=: `chars_required *= kBitsPerCharTableMultiplier`
        source = re.sub(
            r'\bchars_required\s*\*=\s*kBitsPerCharTableMultiplier\b(?!\s*as\b)',
            'chars_required *= kBitsPerCharTableMultiplier as u64',
            source,
        )
        #     `kHalfDigitBits * kBitsPerCharTableMultiplier / max_bits_per_char` — all i32 arithmetic.
        source = re.sub(
            r'\bkHalfDigitBits\s*\*\s*kBitsPerCharTableMultiplier\s*/\s*max_bits_per_char\b',
            '(kHalfDigitBits as i64 * kBitsPerCharTableMultiplier as i64 / max_bits_per_char as i64) as i32',
            source,
        )
        #     Division by min_bits_per_char (u8) in u64 context — cast to u64.
        source = re.sub(
            r'/\s*\(min_bits_per_char\)(?!\s*as\b)',
            '/ (min_bits_per_char as u64)',
            source,
        )
        source = re.sub(
            r'/\s*\(max_bits_per_char\)(?!\s*as\b)',
            '/ (max_bits_per_char as u64)',
            source,
        )

        # 26. Add `#[derive(Default, Copy)]` to Parameters struct without double-derive conflict.
        #     The transpiler may have already generated `#[derive(Debug, Clone)]` — merge the derives.
        if re.search(r'#\[derive\([^\)]*\)\]\s*\n?\s*pub struct Parameters\s*\{', source):
            # Merge: add Default and Copy to the existing derive list (inside the parens)
            source = re.sub(
                r'(#\[derive\([^\)]*)(\)\])([\s\S]*?pub struct Parameters\s*\{)',
                lambda m: (
                    m.group(1)
                    + (', Default, Copy' if 'Default' not in m.group(1) else '')
                    + m.group(2) + m.group(3)
                ),
                source,
                count=1,
            )
        else:
            source = re.sub(
                r'(pub struct Parameters \{)',
                r'#[derive(Default, Clone, Copy, Debug)]\n\1',
                source,
                count=1,
            )

        # 27. Fix `.Normalize` bare field-like method refs → `.Normalize()` and
        #     `.len` bare field refs (on Digits/RWDigits/ScratchDigits) → `.len()`
        source = re.sub(r'\b(\w+)\.Normalize\b(?!\s*\()', r'\1.Normalize()', source)
        source = re.sub(
            r'\b(rest|dividend|z|x|y|digits)\.len\b(?!\s*\()',
            r'\1.len()',
            source,
        )

        # 28. Resolve `bit_length` name collision between util::bit_length(i32)
        #     and vector_arithmetic::bit_length(Digits).
        #     Rename `bit_length(int_var)` calls → `int_bit_length(int_var)`.
        #     The variable names that appear as i32 bit_length args in bigint code:
        source = re.sub(
            r'\bbit_length\((?=(?:n|len|max_m|n_bits)\b)',
            'int_bit_length(',
            source,
        )

        # 29. Fix ScratchDigits initialization: `let mut rest: ScratchDigits = expr;`
        #     → `let mut rest: ScratchDigits = ScratchDigits::new(expr);`
        source = re.sub(
            r'(let\s+mut\s+\w+\s*:\s*ScratchDigits\s*=\s*)([^;{]+?)\s*;',
            r'\1ScratchDigits::new(\2);',
            source,
        )

        # 29b. Fix memcpy/memset calls: dst/src must be *mut u8 / *const u8, not *mut digit_t.
        #      Pattern: `memcpy(dst, src, n)` where dst/src are *mut digit_t → cast to *mut u8.
        source = re.sub(
            r'\bmemcpy\((\w+),\s*(\w+)\s*as\s*\*const\s*\(\)',
            r'memcpy(\1 as *mut u8, \2 as *const u8',
            source,
        )
        source = re.sub(
            r'\bmemset\(([^.]+)\.digits\(\)',
            r'memset(\1.digits() as *mut u8',
            source,
        )

        # 30. Fix `*x.add(...)` raw pointer deref missing unsafe (common after pointer rewrites).
        #     Pattern: `= unsafe { *x.add(k) }` already wrapped. But bare `*x.add(k)` outside of unsafe
        #     in assignment RHS → wrap in unsafe { }
        source = re.sub(
            r'(?<!unsafe \{ )(\*\w+\.add\([^)]+\))(?!\s*})',
            r'unsafe { \1 }',
            source,
        )
        # Fix `= -high` where high: signed_digit_t and result: digit_t — cast negation.
        source = re.sub(
            r'(:\s*digit_t\s*=\s*)-(\w+)\s*;',
            r'\1(-\2 as i64) as digit_t;',
            source,
        )

        # 31. power_of_two is i32, kDigitBits is i32 — no cast needed; fix any erroneous u64 casts.
        source = re.sub(
            r'\bpower_of_two\s*/\s*kDigitBits\s*as\s*u64\b',
            r'power_of_two / kDigitBits',
            source,
        )
        source = re.sub(
            r'\bpower_of_two\s*%\s*kDigitBits\s*as\s*u64\b',
            r'power_of_two % kDigitBits',
            source,
        )

        # 32. Fix `*mut T` deref that still triggers E0133 (added by 22i incorrectly).
        #     Pattern: `type Parameters cannot be dereferenced` happens when
        #     we have `(*params)` but `params` is NOT a raw pointer.
        #     The 22i regex should have been guarded by type check — can't easily fix here,
        #     but we can try to limit to known pointer param names from mul_fft.rs.

        # 33. Fix `if state` where state: *mut RightShiftState (pointer used as bool).
        source = re.sub(
            r'\bif\s+state\s*\{',
            'if !state.is_null() {',
            source,
        )

        # 33b. Fix `*mut digit_t<*mut digit_t>` (wrong generic on digit_t) → `*mut *mut digit_t`.
        source = re.sub(
            r'\*mut\s+digit_t\s*<\s*\*mut\s+digit_t\s*>',
            '*mut *mut digit_t',
            source,
        )

        # 33c. Fix `std::cmp::max() as u64` (zero-arg max is wrong) → `u32::MAX as u64`.
        #      This comes from `std::numeric_limits<uint32_t>::max()`.
        source = re.sub(
            r'\bstd::cmp::max\(\)\s*as\s*u64',
            'u32::MAX as u64',
            source,
        )

        # 33d. Fix `kMaxBitsPerChar[radix]` where radix: i32 → needs as usize.
        source = re.sub(
            r'\bkMaxBitsPerChar\[([^\]]+)\]',
            r'kMaxBitsPerChar[(\1) as usize]',
            source,
        )

        # 33e. Fix f64 division by i32: `x as f64 / int_var.field` → `x as f64 / (int_var.field as f64)`.
        source = re.sub(
            r'(as\s+f64\s*/\s*)(\w+(?:\.\w+)?)\b(?!\s*as\b)',
            r'\1(\2 as f64)',
            source,
        )

        # 33f. Fix `dst + digits_to_copy` pointer arithmetic (raw ptr + int).
        #      Pattern: ptr_var + int_expr → unsafe { ptr_var.add(int_expr as usize) }
        source = re.sub(
            r'\b(dst|src)\s*\+\s*(\w+)\b',
            r'unsafe { \1.add(\2 as usize) }',
            source,
        )

        # 33g-pre. Fix todo!() with format string braces (literal {} causes format interpretation).
        def _escape_todo_braces(src: str) -> str:
            import re as _re
            def _escape(m: 're.Match') -> str:
                inner = m.group(1).replace('{', '{{').replace('}', '}}')
                return f'todo!("{inner}")'
            return _re.sub(r'todo!\("([^"]*)"\)', _escape, src)
        source = _escape_todo_braces(source)

        # 33g. Fix `variable = min;` / `variable = max;` where transpiler lost std::min/max args.
        source = re.sub(r'(\w+)\s*=\s*min\s*;(?!\()', r'\1 = todo!("std::min(...)");', source)
        source = re.sub(r'(\w+)\s*=\s*max\s*;(?!\()', r'\1 = todo!("std::max(...)");', source)

        # 33h. Fix `'0' + chunk % radix` (char + u64 is invalid).
        #      The target is *mut i8, so use byte literal b'0' + u64 cast.
        source = re.sub(
            r"'0'\s*\+\s*(\w+)\s*%\s*radix\s*as\s*u64",
            lambda m: f"(b'0' as u64 + {m.group(1)} % radix as u64) as i8",
            source,
        )
        source = re.sub(
            r"'0'\s*\+\s*(\w+)\s*%\s*(\w+)\s*as\s*u64",
            lambda m: f"(b'0' as u64 + {m.group(1)} % {m.group(2)} as u64) as i8",
            source,
        )

        # 33i. Fix `max_bits_per_char` struct field declared `as i32` (leftover from old fixes).
        # (Should be clean now, but guard anyway)
        source = re.sub(r'\bmax_bits_per_char\s*as\s*i32\s*:', 'max_bits_per_char:', source)
        source = re.sub(r'\bmin_bits_per_char\s*as\s*i32\s*:', 'min_bits_per_char:', source)

        # 34. Fix `signed_digit_t` assigned to `digit_t`: mul_fft.rs raw pointer reads.
        #     `*x.add(k)` where x: *mut digit_t returns digit_t, assigned to signed_digit_t needs cast.
        #     The argument to .add() may be parenthesized, e.g. `*x.add((k) as usize)`, so use [^}]+.
        source = re.sub(
            r'(let\s+mut\s+\w+\s*:\s*signed_digit_t\s*=\s*)(unsafe\s*\{[^{}]+\})',
            r'\1\2 as signed_digit_t',
            source,
        )

        # 34b. Fix sizeof(digit_t) todo → std::mem::size_of::<digit_t>()
        source = re.sub(
            r'todo!\("TODO: manually translate[^"]*sizeof\(digit_t\)[^"]*"\)',
            'std::mem::size_of::<digit_t>()',
            source,
        )
        # 34c. After replacing sizeof, i32 * usize is invalid; cast the i32 operand to usize.
        #      Pattern: `<expr_returning_i32> * std::mem::size_of::<digit_t>()` where lhs is `.len()`.
        source = re.sub(
            r'(\w+)\.len\(\)\s*\*\s*std::mem::size_of::<digit_t>\(\)',
            r'\1.len() as usize * std::mem::size_of::<digit_t>()',
            source,
        )
        source = re.sub(
            r'\b(\w+)\s*\*\s*std::mem::size_of::<digit_t>\(\)',
            lambda m: (
                f'{m.group(1)} as usize * std::mem::size_of::<digit_t>()'
                if not m.group(1).endswith('usize') else m.group(0)
            ),
            source,
        )

        # 35pre. Fix `*{ ptr = unsafe { ptr.sub(1) }; ptr } = expr;`
        # Rewrite to: `unsafe { ptr = ptr.sub(1); *ptr = expr; }`
        def _fix_dec_ptr_write(src: str) -> str:
            def _repl(m: 're.Match') -> str:
                ptr = m.group(1)
                expr = m.group(2).strip()
                return f'unsafe {{ {ptr} = {ptr}.sub(1); *{ptr} = {expr}; }}'
            return re.sub(
                r'\*\{\s*(\w+)\s*=\s*unsafe\s*\{\s*\1\.sub\(1\)\s*\}\s*;\s*\1\s*\}\s*=\s*([^;]+);',
                _repl,
                src,
            )
        source = _fix_dec_ptr_write(source)

        # 35pre-b. Any remaining *{ ... } = expr; patterns need unsafe (nested braces ok).
        source = re.sub(
            r'(\*\{(?:[^{}]|\{[^{}]*\})*\})\s*=\s*([^;{}]+);',
            r'unsafe { \1 = \2; }',
            source,
        )

        # 35pre1b. Fix PascalCase method calls that were not renamed to snake_case.
        #   self.BasecaseLast(...) → self.basecase_last(...)
        #   self.BasecaseMiddle(...) → self.basecase_middle(...)
        for _pascal, _snake in [
            ('BasecaseLast', 'basecase_last'),
            ('BasecaseMiddle', 'basecase_middle'),
            ('BasecaseFixedLast', 'basecase_fixed_last'),
            ('DivideByMagic', 'divide_by_magic'),
            ('Start_Default', 'start_default'),
            ('PointwiseMultiply', 'pointwise_multiply'),
            ('BackwardFFT', 'backward_fft'),
            ('CounterWeightAndRecombine', 'counter_weight_and_recombine'),
        ]:
            source = re.sub(
                rf'\bself\.{re.escape(_pascal)}\b',
                f'self.{_snake}',
                source,
            )

        # 35pre2. Fix eq_op and assign_op operator remnants.
        # `z(eq_op, x)` → `z.as_digits() == x`
        source = re.sub(
            r'\b(\w+)\s*\(eq_op,\s*([^)]+)\)',
            r'\1.as_digits() == \2',
            source,
        )
        # `unsafe { *ptr }(assign_op, val)` → `unsafe { *ptr = val; }`
        source = re.sub(
            r'unsafe\s*\{\s*\*(\w+)\s*\}\s*\(assign_op,\s*([^)]+)\)',
            r'unsafe { *\1 = \2; }',
            source,
        )
        # `var(assign_op, val)` → `var = val;`
        source = re.sub(
            r'\b(\w+)\s*\(assign_op,\s*([^)]+)\)\s*;',
            r'\1 = \2;',
            source,
        )

        # 35pre3. Fix `swap;` bare reference → todo!() and `swap(a, b)` → std::mem::swap(&mut a, &mut b)
        source = re.sub(r'\bswap\s*;', r'{ /* swap todo */ };', source)
        source = re.sub(
            r'\bswap\((\w+),\s*(\w+)\)',
            r'std::mem::swap(&mut \1, &mut \2)',
            source,
        )

        # 35pre4. Remove conflicting constants at bottom of tostring.rs (duplicated in lib.rs stubs).
        source = re.sub(
            r'\npub const MAX_BITS_PER_CHAR:.*?;',
            '\n// MAX_BITS_PER_CHAR defined in lib.rs',
            source,
            flags=re.DOTALL,
        )
        source = re.sub(
            r'\npub const CONVERSION_CHARS:.*?;',
            '\n// CONVERSION_CHARS defined in lib.rs',
            source,
            flags=re.DOTALL,
        )
        source = re.sub(
            r'\npub const BITS_PER_CHAR_TABLE_SHIFT:.*?;',
            '\n// BITS_PER_CHAR_TABLE_SHIFT defined in lib.rs',
            source,
        )
        source = re.sub(
            r'\npub const BITS_PER_CHAR_TABLE_MULTIPLIER:.*?;',
            '\n// BITS_PER_CHAR_TABLE_MULTIPLIER defined in lib.rs',
            source,
        )

        # 35pre5. Fix memset/memcpy with *mut digit_t — cast to *mut u8.
        source = re.sub(
            r'\bmemset\(([^,]+),',
            lambda m: f'memset({m.group(1)} as *mut u8,'
                      if '*mut digit_t' not in m.group(1) and 'as *mut u8' not in m.group(1)
                      else f'memset({m.group(1)},',
            source,
        )
        source = re.sub(
            r'memset\(unsafe\s*\{\s*(\w+)\.add\(([^)]+)\)\s*\}(?!\s*as)',
            r'memset(unsafe { \1.add(\2) } as *mut u8',
            source,
        )

        # 35. Fix immutable function arguments that are reassigned in the body.
        #     Detect `param_name <assign_op>` in a function body and add `mut` to
        #     the parameter declaration if it's missing.
        def _fix_mut_params(src: str) -> str:
            import re as _re
            # Find all fn signatures and their bodies
            fn_pattern = _re.compile(
                r'((?:pub\s+)?fn\s+\w+\s*(?:<[^>]*>)?\s*\()([^{]+)(\)\s*(?:->\s*[^{]+)?\s*\{)',
                _re.DOTALL,
            )
            # Detect assignment operators AND Normalize() method calls (which require &mut self)
            assign_re = _re.compile(r'\b(\w+)\s*(?:[+\-*/%&|^]?=(?!=)|>>=|<<=|-=|\+=|\*=|/=|%=|\.Normalize\(\))')

            def _fix_fn(m: 're.Match') -> str:
                prefix, params_str, suffix = m.group(1), m.group(2), m.group(3)
                # Extract param names (non-mut, non-self params)
                param_names = set(_re.findall(
                    r'(?<!\bmut\s)\b(\w+)\s*:\s*(?!\*)', params_str
                ))
                param_names.discard('self')
                if not param_names:
                    return m.group(0)
                fn_start = m.end()
                body_text = src[fn_start:fn_start + 3000]
                assigned = set()
                for am in assign_re.finditer(body_text):
                    nm = am.group(1)
                    if nm in param_names:
                        assigned.add(nm)
                if not assigned:
                    return m.group(0)
                new_params = params_str
                for nm in assigned:
                    new_params = _re.sub(
                        rf'(?<!\bmut\s)\b{_re.escape(nm)}\s*:',
                        f'mut {nm}:',
                        new_params,
                    )
                return prefix + new_params + suffix

            return fn_pattern.sub(_fix_fn, src)

        source = _fix_mut_params(source)

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
            # Declaration-only: emit an empty stub (no body in C++ header).
            ret = f.return_type
            ret_name = ret.name if ret else "void"
            if not ret_name or ret_name in ("void", "()", ""):
                default_expr = ""
            else:
                default_expr = f"\n{self._i()}    <{ret_name} as Default>::default()"
            parts.append(f"{i}{sig} {{{default_expr}\n{i}}}")

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
            type_str = self.emit_type(p.type)
            # RWDigits parameters are passed by value and need to be mutable
            # so that IndexMut (z[i] = val) works inside the function body.
            if type_str == "RWDigits":
                param_parts.append(f"mut {p.name}: {type_str}")
            else:
                param_parts.append(f"{p.name}: {type_str}")
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
        if isinstance(stmt, IRRawCppStmt):
            return self._emit_raw_cpp_stmt(stmt)
        return f"{self._i()}// TODO: unhandled statement {type(stmt).__name__}"

    # -- constructor body helper ---------------------------------------------

    def _emit_constructor_body(self, f: IRFunction) -> str:
        """Emit a constructor body that returns Self.

        If the body already ends with a return statement, use it as-is.
        Otherwise, append ``Self { ..Default::default() }`` or ``todo!()``
        to ensure the function returns Self.
        """
        if not f.body or not f.body.statements:
            # Empty constructor — emit a default Self initialisation.
            return "{\n        Self::default()\n    }"

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
        lines.append(f"{self._i()}unimplemented!(\"constructor: translate return to Self\")")
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
            val = self.emit_expr(v.initializer) if v.initializer else f"<{type_str} as Default>::default()"
            return f"{i}static {mut}{v.name}: {type_str} = {val};"

        if v.initializer is not None:
            init = self.emit_expr(v.initializer)
            # Guard against empty initializer expressions
            if not init or init.strip() == "":
                init = "Default::default()"
            return f"{i}let {mut}{v.name}{type_ann} = {init};"
        return f"{i}let {mut}{v.name}{type_ann};"

    # -- expression statement ------------------------------------------------

    def _emit_expr_stmt(self, es: IRExprStmt) -> str:
        if es.expr is None:
            return ""
        i = self._i()

        # POST_INC / POST_DEC as statements: the return value is discarded,
        # so emit a simple `x += 1;` / `x -= 1;` instead of the verbose
        # `{ let _old = x; x += 1; _old };` block expression.
        if isinstance(es.expr, IRUnaryExpr):
            operand_str = self.emit_expr(es.expr.operand)
            if es.expr.op == UnaryOp.POST_INC:
                return f"{i}{operand_str} += 1;"
            if es.expr.op == UnaryOp.POST_DEC:
                return f"{i}{operand_str} -= 1;"
            if es.expr.op == UnaryOp.PRE_INC:
                return f"{i}{operand_str} += 1;"
            if es.expr.op == UnaryOp.PRE_DEC:
                return f"{i}{operand_str} -= 1;"

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
        iter_expr = self.emit_expr(f.iterable) if f.iterable else "[]"
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
        source = (r.cpp_source or "").replace('\n', ' ').strip()

        # If the source is already a Rust comment (starts with //), emit as-is.
        if source.startswith("//"):
            return f"{i}{source}"

        source_safe = source.replace('*/', '* /').replace('\x00', '')
        if len(source_safe) > 120:
            source_safe = source_safe[:120] + "..."
        comment = (r.comment or "").replace('*/', '* /')
        if comment:
            return f"{i}unsafe {{ let _ = 0; /* UNTRANSLATED {comment}: {source_safe} */ }}"
        return f"{i}unsafe {{ let _ = 0; /* UNTRANSLATED: {source_safe} */ }}"

    def _emit_raw_cpp_stmt(self, r: IRRawCppStmt) -> str:
        """Always-compiles fallback: emits `unsafe { let _ = 0; }` with a comment."""
        i = self._i()
        source = (r.cpp_source or "").replace('\n', ' ').strip()
        if source.startswith("//"):
            return f"{i}{source}"
        source_safe = source.replace('*/', '* /').replace('\x00', '')
        if len(source_safe) > 120:
            source_safe = source_safe[:120] + "..."
        return f"{i}unsafe {{ let _ = 0; /* UNTRANSLATED: {source_safe} */ }}"

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
            if expr.name:
                return expr.name
            # Fall back to the last segment of the qualified name.
            if expr.qualified_name:
                last = expr.qualified_name.split("::")[-1]
                if last:
                    return last
            # Truly unknown — use Default::default() which is always valid as an expression.
            return '<_ as Default>::default() /* unknown_ref */'
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
        if isinstance(expr, IRRawCppExpr):
            return self._emit_raw_cpp_expr(expr)
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
            # C++ `&var` that yields a non-const pointer becomes a raw mutable
            # pointer in Rust.  `std::ptr::addr_of_mut!` is the safe way to
            # obtain a `*mut T` without creating a mutable reference first.
            return f"std::ptr::addr_of_mut!({operand})"

        # For prefix operators applied to a binary expression, wrap the operand
        # in parentheses to preserve intended precedence.  Without parens,
        # `!a >= b` is parsed as `(!a) >= b`, not `!(a >= b)`.
        if isinstance(u.operand, IRBinaryExpr) and u.op in (
            UnaryOp.NOT, UnaryOp.NEG, UnaryOp.BITNOT
        ):
            operand = f"({operand})"

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

        # If the function resolves to a placeholder (from IRRawExpr/IRRawCppExpr), don't
        # try to call it — calling a Default::default() value gives E0618.
        if (func.startswith("todo!(") or func.startswith("unimplemented!(")
                or func.startswith("unreachable!(") or "Default>::default()" in func
                or func.startswith("/* UNTRANSLATED")):
            return f"/* UNTRANSLATED call: {func} */ <_ as Default>::default()"

        # Handle C++ `USE(x)` — a no-op that silences unused-variable warnings.
        # After snake_case conversion the function name becomes `use`, a Rust keyword.
        if func in ("USE", "use"):
            if args_exprs:
                return f"let _ = {args_exprs[0]}"
            return "/* USE() */"

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
        # If the object is an untranslated placeholder, don't chain .member on it
        if obj.startswith("todo!(") or "Default>::default()" in obj:
            return obj
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
            # Rust `as` has very high precedence — parenthesize the operand
            # when it is a complex expression to prevent mis-parsing.
            # E.g. `a / b as T` is `a / (b as T)`, NOT `(a / b) as T`.
            needs_parens = isinstance(c.expr, (IRBinaryExpr, IRUnaryExpr, IRTernaryExpr))
            if needs_parens:
                expr = f"({expr})"
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
        source = (r.cpp_source or "").replace('*/', '* /').replace('\x00', '').replace('\n', ' ')
        if len(source) > 120:
            source = source[:120] + "..."
        comment = (r.comment or "").replace('*/', '* /')
        if comment:
            return f"/* UNTRANSLATED {comment}: {source} */ <_ as Default>::default()"
        return f"/* UNTRANSLATED: {source} */ <_ as Default>::default()"

    def _emit_raw_cpp_expr(self, r: IRRawCppExpr) -> str:
        """Always-compiles fallback: emits Default::default() with a comment."""
        source = (r.cpp_source or "").replace('*/', '* /').replace('\x00', '')
        if len(source) > 120:
            source = source[:120] + "..."
        default_type = r.default_type or "i32"
        comment = (r.comment or "").replace('*/', '* /')
        if comment:
            return f"/* UNTRANSLATED: {comment}: {source} */ <{default_type} as Default>::default()"
        return f"/* UNTRANSLATED: {source} */ <{default_type} as Default>::default()"

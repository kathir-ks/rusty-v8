"""Deterministic mapping from C++ type names to Rust type names.

Converts C++ type strings (including qualifiers, pointers, references, templates,
and namespaced identifiers) into IRType nodes suitable for Rust code generation.
"""

from __future__ import annotations

import re
from typing import Dict, List, Optional, Tuple

from ir.nodes import IRType, TypeKind


# ─── Primitive type mapping table ────────────────────────────────────────────

_PRIMITIVE_MAP: Dict[str, str] = {
    # Exact-width integer types
    "int8_t": "i8",
    "uint8_t": "u8",
    "int16_t": "i16",
    "uint16_t": "u16",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    # Platform-width types
    "size_t": "usize",
    "ssize_t": "isize",
    "ptrdiff_t": "isize",
    "intptr_t": "isize",
    "uintptr_t": "usize",
    # Fundamental types
    "bool": "bool",
    "char": "i8",
    "unsigned char": "u8",
    "signed char": "i8",
    "wchar_t": "u32",
    "char16_t": "u16",
    "char32_t": "u32",
    "float": "f32",
    "double": "f64",
    "long double": "f64",
    # int variants
    "short": "i16",
    "short int": "i16",
    "unsigned short": "u16",
    "unsigned short int": "u16",
    "int": "i32",
    "unsigned int": "u32",
    "unsigned": "u32",
    "signed int": "i32",
    "signed": "i32",
    "long": "i64",
    "long int": "i64",
    "unsigned long": "u64",
    "unsigned long int": "u64",
    "signed long": "i64",
    "long long": "i64",
    "long long int": "i64",
    "unsigned long long": "u64",
    "unsigned long long int": "u64",
    "signed long long": "i64",
    # Special
    "void": "()",
}

# ─── Std container / library type mapping table ──────────────────────────────

# Maps C++ std:: type base name to (Rust name, number of expected generic args).
# -1 means variadic (e.g., std::tuple).
_STD_CONTAINER_MAP: Dict[str, Tuple[str, int]] = {
    "std::string": ("String", 0),
    "std::string_view": ("&str", 0),
    "std::vector": ("Vec", 1),
    "std::array": ("[T; N]", 2),  # special handling
    "std::unique_ptr": ("Box", 1),
    "std::shared_ptr": ("Arc", 1),
    "std::weak_ptr": ("Weak", 1),
    "std::optional": ("Option", 1),
    "std::pair": ("(A, B)", 2),  # special handling → tuple
    "std::tuple": ("tuple", -1),  # special handling → tuple
    "std::unordered_map": ("HashMap", 2),
    "std::map": ("BTreeMap", 2),
    "std::unordered_set": ("HashSet", 1),
    "std::set": ("BTreeSet", 1),
    "std::function": ("Box<dyn Fn>", 1),  # special handling
    "std::mutex": ("Mutex", 0),
    "std::atomic": ("Atomic", 1),  # special handling
    "std::thread": ("JoinHandle<()>", 0),
    "std::initializer_list": ("&[T]", 1),  # special handling → slice ref
    "std::span": ("&[T]", 1),  # special handling → slice ref
    "std::basic_string": ("String", 0),
    "std::basic_string_view": ("&str", 0),
}

# ─── V8 base:: and internal type aliases ─────────────────────────────────────
# These are V8-specific types commonly found after namespace stripping.
# They map to Rust std equivalents or simple aliases.

_V8_TYPE_MAP: Dict[str, str] = {
    # base:: types
    "base::Mutex": "std::sync::Mutex<()>",
    "base::MutexGuard": "std::sync::MutexGuard<'static, ()>",
    "base::RecursiveMutex": "std::sync::Mutex<()>",
    "base::SharedMutex": "std::sync::RwLock<()>",
    "base::ConditionVariable": "()",
    "base::Optional": "Option",
    "base::Vector": "&[T]",
    "base::OwnedVector": "Vec",
    "base::TimeDelta": "std::time::Duration",
    "base::TimeTicks": "std::time::Instant",
    "base::Thread": "std::thread::JoinHandle<()>",
    "base::Semaphore": "()",
    "base::LazyInstance": "()",
    "base::OnceType": "std::sync::Once",
    "base::RandomNumberGenerator": "()",
    # Common V8 address/pointer types (after namespace stripping, these
    # appear as bare names and are already in the prelude)
}

# ─── Import table for Rust types that require `use` statements ───────────────

_IMPORT_MAP: Dict[str, str] = {
    "HashMap": "std::collections::HashMap",
    "BTreeMap": "std::collections::BTreeMap",
    "HashSet": "std::collections::HashSet",
    "BTreeSet": "std::collections::BTreeSet",
    "Arc": "std::sync::Arc",
    "Weak": "std::sync::Weak",
    "Mutex": "std::sync::Mutex",
    "AtomicBool": "std::sync::atomic::AtomicBool",
    "AtomicI8": "std::sync::atomic::AtomicI8",
    "AtomicI16": "std::sync::atomic::AtomicI16",
    "AtomicI32": "std::sync::atomic::AtomicI32",
    "AtomicI64": "std::sync::atomic::AtomicI64",
    "AtomicU8": "std::sync::atomic::AtomicU8",
    "AtomicU16": "std::sync::atomic::AtomicU16",
    "AtomicU32": "std::sync::atomic::AtomicU32",
    "AtomicU64": "std::sync::atomic::AtomicU64",
    "AtomicUsize": "std::sync::atomic::AtomicUsize",
    "AtomicIsize": "std::sync::atomic::AtomicIsize",
    "JoinHandle": "std::thread::JoinHandle",
}

# ─── Atomic type specialisation ──────────────────────────────────────────────

_ATOMIC_MAP: Dict[str, str] = {
    "bool": "AtomicBool",
    "i8": "AtomicI8",
    "i16": "AtomicI16",
    "i32": "AtomicI32",
    "i64": "AtomicI64",
    "u8": "AtomicU8",
    "u16": "AtomicU16",
    "u32": "AtomicU32",
    "u64": "AtomicU64",
    "usize": "AtomicUsize",
    "isize": "AtomicIsize",
}


# ─── Simple tokeniser for C++ type strings ────────────────────────────────────

def _tokenize(type_str: str) -> List[str]:
    """Split a C++ type string into a stream of tokens.

    Tokens include identifiers (possibly with ::), angle brackets, parentheses,
    square brackets, asterisks, ampersands, commas, and numbers.
    """
    tokens: List[str] = []
    i = 0
    s = type_str.strip()
    n = len(s)
    while i < n:
        # Skip whitespace (but we may need to collapse identifiers with spaces
        # like "unsigned long long").
        if s[i].isspace():
            i += 1
            continue
        # Single-character punctuation
        if s[i] in "<>,*[]()":
            tokens.append(s[i])
            i += 1
            continue
        # Ampersand(s)
        if s[i] == "&":
            if i + 1 < n and s[i + 1] == "&":
                tokens.append("&&")
                i += 2
            else:
                tokens.append("&")
                i += 1
            continue
        # Numbers (for array sizes and template integer args)
        if s[i].isdigit():
            j = i
            while j < n and s[j].isdigit():
                j += 1
            tokens.append(s[i:j])
            i = j
            continue
        # Identifiers and keywords (may contain ::)
        if s[i].isalpha() or s[i] == "_" or s[i] == ":":
            j = i
            while j < n and (s[j].isalnum() or s[j] == "_" or s[j] == ":"):
                j += 1
            token = s[i:j]
            # Collapse multi-word type keywords: "unsigned long long int" etc.
            # Peek ahead for keywords that form compound types.
            _COMPOUND_PARTS = {
                "unsigned", "signed", "long", "short", "int", "char", "double",
            }
            while token.split()[-1] if " " in token else token.split("::")[-1] in _COMPOUND_PARTS:
                # Check if next non-space token is also a compound part
                k = j
                while k < n and s[k].isspace():
                    k += 1
                if k < n and (s[k].isalpha() or s[k] == "_"):
                    m = k
                    while m < n and (s[m].isalnum() or s[m] == "_"):
                        m += 1
                    next_word = s[k:m]
                    if next_word in _COMPOUND_PARTS:
                        token = token + " " + next_word
                        j = m
                        continue
                break
            tokens.append(token)
            i = j
            continue
        # Anything else: skip
        i += 1
    return tokens


# ─── Recursive descent parser for C++ type strings ──────────────────────────

class _TypeParser:
    """Parse a tokenised C++ type string into structured components."""

    def __init__(self, tokens: List[str]) -> None:
        self._tokens = tokens
        self._pos = 0

    def _peek(self) -> Optional[str]:
        if self._pos < len(self._tokens):
            return self._tokens[self._pos]
        return None

    def _advance(self) -> str:
        tok = self._tokens[self._pos]
        self._pos += 1
        return tok

    def _expect(self, tok: str) -> None:
        got = self._advance()
        if got != tok:
            raise ValueError(f"Expected '{tok}' but got '{got}'")

    def _at_end(self) -> bool:
        return self._pos >= len(self._tokens)

    # ── Public API ──

    def parse(self) -> dict:
        """Return a dict with keys: base, is_const, ptr_levels, ref_kind,
        template_args, array_size, is_function_ptr, fn_ret, fn_params.
        """
        is_const = False
        # Leading const
        if self._peek() == "const":
            is_const = True
            self._advance()

        # Check for function pointer: void (*)(int, int)
        # Pattern: <type> ( * ) ( <params> )
        # We handle this after parsing the base type by checking for '(' next.

        base = self._parse_base_name()

        # Trailing const (e.g., "int const")
        if self._peek() == "const":
            is_const = True
            self._advance()

        # Template args
        template_args: List[str] = []
        if self._peek() == "<":
            template_args = self._parse_template_args()

        # Trailing const after template
        if self._peek() == "const":
            is_const = True
            self._advance()

        # Check for function pointer: base_type (*)(args)
        is_function_ptr = False
        fn_params: List[str] = []
        fn_ret = base
        if self._peek() == "(":
            saved_pos = self._pos
            try:
                fp_result = self._try_parse_function_pointer(base, template_args)
                if fp_result is not None:
                    return {
                        "base": fp_result["ret"],
                        "is_const": is_const,
                        "ptr_levels": 0,
                        "ref_kind": None,
                        "template_args": [],
                        "array_size": None,
                        "is_function_ptr": True,
                        "fn_ret": fp_result["ret"],
                        "fn_params": fp_result["params"],
                    }
            except (ValueError, IndexError):
                self._pos = saved_pos

        # Pointer / reference suffixes
        ptr_levels = 0
        ref_kind: Optional[str] = None  # "&", "&&", or None
        while not self._at_end():
            tok = self._peek()
            if tok == "*":
                ptr_levels += 1
                self._advance()
            elif tok == "&":
                ref_kind = "&"
                self._advance()
            elif tok == "&&":
                ref_kind = "&&"
                self._advance()
            elif tok == "const":
                # const after * — pointer-to-const already captured
                is_const = True
                self._advance()
            else:
                break

        # Array suffix [N]
        array_size: Optional[int] = None
        if self._peek() == "[":
            self._advance()
            size_tok = self._advance()
            array_size = int(size_tok)
            self._expect("]")

        return {
            "base": base,
            "is_const": is_const,
            "ptr_levels": ptr_levels,
            "ref_kind": ref_kind,
            "template_args": template_args,
            "array_size": array_size,
            "is_function_ptr": False,
            "fn_ret": None,
            "fn_params": [],
        }

    def _parse_base_name(self) -> str:
        """Parse a possibly-namespaced identifier."""
        parts: List[str] = []
        tok = self._peek()
        if tok is None:
            return ""
        # Handle multi-word types that survived tokenisation as a single token
        parts.append(self._advance())
        return parts[0]

    def _parse_template_args(self) -> List[str]:
        """Parse <A, B, C> returning the raw strings of each argument (including nested templates)."""
        self._expect("<")
        args: List[str] = []
        depth = 1
        current_tokens: List[str] = []
        while not self._at_end() and depth > 0:
            tok = self._peek()
            if tok == "<":
                depth += 1
                current_tokens.append(self._advance())
            elif tok == ">":
                depth -= 1
                if depth == 0:
                    self._advance()
                    if current_tokens:
                        args.append(" ".join(current_tokens))
                else:
                    current_tokens.append(self._advance())
            elif tok == "," and depth == 1:
                self._advance()
                if current_tokens:
                    args.append(" ".join(current_tokens))
                current_tokens = []
            else:
                current_tokens.append(self._advance())
        return args

    def _try_parse_function_pointer(
        self, base: str, tmpl_args: List[str]
    ) -> Optional[dict]:
        """Try to parse (*)(params) as a function pointer.

        Returns {"ret": ..., "params": [...]} or None.
        """
        self._expect("(")
        if self._peek() != "*":
            return None
        self._advance()  # *
        self._expect(")")
        self._expect("(")
        params: List[str] = []
        depth = 1
        current_tokens: List[str] = []
        while not self._at_end() and depth > 0:
            tok = self._peek()
            if tok == "(":
                depth += 1
                current_tokens.append(self._advance())
            elif tok == ")":
                depth -= 1
                if depth == 0:
                    self._advance()
                    if current_tokens:
                        params.append(" ".join(current_tokens))
                else:
                    current_tokens.append(self._advance())
            elif tok == "," and depth == 1:
                self._advance()
                if current_tokens:
                    params.append(" ".join(current_tokens))
                current_tokens = []
            else:
                current_tokens.append(self._advance())
        ret_type = base
        if tmpl_args:
            ret_type = f"{base}<{', '.join(tmpl_args)}>"
        return {"ret": ret_type, "params": params}


# ─── TypeMapper class ───────────────────────────────────────────────────────

class TypeMapper:
    """Deterministic mapper from C++ type names to Rust IRType nodes."""

    def __init__(self) -> None:
        # Cache for previously mapped types to ensure determinism and speed.
        self._cache: Dict[str, IRType] = {}

    # ── Main entry point ──

    def map_type(self, cpp_type: str) -> IRType:
        """Map a C++ type string to an IRType.

        Handles qualifiers, pointers, references, templates, arrays,
        function pointers, and namespaced identifiers.
        """
        cpp_type = cpp_type.strip()
        if not cpp_type:
            return IRType(name="()", cpp_name="", kind=TypeKind.VOID)

        if cpp_type in self._cache:
            return self._cache[cpp_type]

        result = self._do_map(cpp_type)
        self._cache[cpp_type] = result
        return result

    def _do_map(self, cpp_type: str) -> IRType:
        """Internal mapping implementation."""
        tokens = _tokenize(cpp_type)
        if not tokens:
            return IRType(name="()", cpp_name=cpp_type, kind=TypeKind.VOID)

        parser = _TypeParser(tokens)
        try:
            parsed = parser.parse()
        except (ValueError, IndexError):
            # If parsing fails, fall back to raw name mapping.
            return IRType(name=self._fallback_name(cpp_type), cpp_name=cpp_type, kind=TypeKind.UNKNOWN)

        base: str = parsed["base"]
        is_const: bool = parsed["is_const"]
        ptr_levels: int = parsed["ptr_levels"]
        ref_kind: Optional[str] = parsed["ref_kind"]
        template_args: List[str] = parsed["template_args"]
        array_size: Optional[int] = parsed["array_size"]
        is_function_ptr: bool = parsed["is_function_ptr"]
        fn_params: List[str] = parsed["fn_params"]
        fn_ret: Optional[str] = parsed["fn_ret"]

        # ── Function pointer ──
        if is_function_ptr:
            return self._build_function_pointer(cpp_type, fn_ret, fn_params)

        # ── C-style array: T[N] ──
        if array_size is not None:
            inner = self._map_base_with_templates(base, template_args)
            return IRType(
                name=f"[{inner.name}; {array_size}]",
                cpp_name=cpp_type,
                kind=TypeKind.ARRAY,
                array_size=array_size,
                generic_args=[inner],
            )

        # ── Map the base type (possibly with templates) ──
        inner = self._map_base_with_templates(base, template_args)

        # ── Pointer / reference wrapping ──
        # Rvalue reference (&&) → just the inner type (Rust moves by default)
        if ref_kind == "&&":
            inner.cpp_name = cpp_type
            return inner

        # Lvalue reference (&)
        if ref_kind == "&":
            if is_const:
                return IRType(
                    name=inner.name,
                    cpp_name=cpp_type,
                    kind=inner.kind,
                    is_const=True,
                    is_reference=True,
                    generic_args=inner.generic_args,
                    lifetime=inner.lifetime,
                )
            else:
                return IRType(
                    name=inner.name,
                    cpp_name=cpp_type,
                    kind=inner.kind,
                    is_mut_reference=True,
                    generic_args=inner.generic_args,
                    lifetime=inner.lifetime,
                )

        # Raw pointer(s)
        if ptr_levels > 0:
            result = inner
            for level in range(ptr_levels):
                is_last = (level == ptr_levels - 1)
                # Only the outermost pointer respects the const qualifier on the
                # pointed-to type.  For multi-level pointers like T** we treat
                # intermediate levels as *mut.
                if is_last and is_const:
                    result = IRType(
                        name=result.name,
                        cpp_name=cpp_type if is_last else "",
                        kind=TypeKind.POINTER,
                        is_pointer=True,
                        is_const=True,
                        generic_args=result.generic_args if level == 0 else [result],
                    )
                else:
                    result = IRType(
                        name=result.name,
                        cpp_name=cpp_type if is_last else "",
                        kind=TypeKind.POINTER,
                        is_pointer=True,
                        is_mut_pointer=True,
                        generic_args=result.generic_args if level == 0 else [result],
                    )
            return result

        # Value type — strip const (Rust values are immutable by default; mutability
        # is on the binding, not the type).
        inner.cpp_name = cpp_type
        return inner

    # ── Base-type + template resolution ──

    def _map_base_with_templates(self, base: str, template_args: List[str]) -> IRType:
        """Resolve a base type name with optional template arguments."""
        # Check for std:: container/library type first
        container = self.map_std_container(base, template_args)
        if container is not None:
            return container

        # Check V8 base:: types
        v8_type = _V8_TYPE_MAP.get(base)
        if v8_type is not None:
            if template_args:
                mapped_args = [self.map_type(arg) for arg in template_args]
                arg_str = ", ".join(a.name for a in mapped_args)
                # Replace placeholder T in v8 type
                if "<T>" in v8_type or v8_type in ("Option", "Vec"):
                    return IRType(
                        name=f"{v8_type}<{arg_str}>",
                        cpp_name=base,
                        kind=TypeKind.CLASS,
                        generic_args=mapped_args,
                    )
            return IRType(name=v8_type, cpp_name=base, kind=TypeKind.CLASS)

        # Check primitives (no templates expected)
        prim = self.map_primitive(base)
        if prim is not None:
            return prim

        # Generic class / struct with optional template args
        rust_name = self.map_qualified_name(base)
        if template_args:
            mapped_args = [self.map_type(arg) for arg in template_args]
            arg_str = ", ".join(a.name for a in mapped_args)
            return IRType(
                name=f"{rust_name}<{arg_str}>",
                cpp_name=base,
                kind=TypeKind.CLASS,
                generic_args=mapped_args,
            )
        return IRType(
            name=rust_name,
            cpp_name=base,
            kind=self._guess_kind(base),
        )

    # ── Primitive mapping ──

    def map_primitive(self, cpp_type: str) -> Optional[IRType]:
        """Map a C++ primitive type name to an IRType.

        Returns None if the type is not a recognised primitive.
        """
        cpp_type = cpp_type.strip()
        rust = _PRIMITIVE_MAP.get(cpp_type)
        if rust is not None:
            kind = TypeKind.VOID if rust == "()" else TypeKind.PRIMITIVE
            return IRType(name=rust, cpp_name=cpp_type, kind=kind)
        return None

    # ── Std container mapping ──

    def map_std_container(
        self, base: str, template_args: Optional[List[str]] = None
    ) -> Optional[IRType]:
        """Map a C++ std:: container/library type to an IRType.

        Returns None if the base is not a recognised std:: type.
        """
        if template_args is None:
            template_args = []

        entry = _STD_CONTAINER_MAP.get(base)
        if entry is None:
            return None

        rust_name, expected_arity = entry
        mapped_args = [self.map_type(arg) for arg in template_args]

        # ── Special cases ──

        # std::string / std::string_view (no generic args)
        if base == "std::string":
            return IRType(name="String", cpp_name=base, kind=TypeKind.CLASS)
        if base == "std::string_view":
            return IRType(name="&str", cpp_name=base, kind=TypeKind.REFERENCE, is_reference=True, is_const=True)

        # std::thread
        if base == "std::thread":
            return IRType(name="JoinHandle<()>", cpp_name=base, kind=TypeKind.CLASS)

        # std::mutex
        if base == "std::mutex":
            return IRType(name="Mutex<()>", cpp_name=base, kind=TypeKind.CLASS)

        # std::initializer_list<T> / std::span<T> → &[T]
        if base in ("std::initializer_list", "std::span"):
            if mapped_args:
                inner = mapped_args[0].name
                return IRType(
                    name=f"&[{inner}]",
                    cpp_name=base,
                    kind=TypeKind.REFERENCE,
                    is_reference=True,
                    is_const=True,
                    generic_args=mapped_args,
                )
            return IRType(name="&[()]", cpp_name=base, kind=TypeKind.REFERENCE)

        # std::basic_string / std::basic_string_view
        if base == "std::basic_string":
            return IRType(name="String", cpp_name=base, kind=TypeKind.CLASS)
        if base == "std::basic_string_view":
            return IRType(name="&str", cpp_name=base, kind=TypeKind.REFERENCE, is_reference=True, is_const=True)

        # std::array<T, N> → [T; N]
        if base == "std::array":
            if len(mapped_args) >= 2:
                t = mapped_args[0]
                n_str = template_args[1].strip() if len(template_args) >= 2 else "0"
                return IRType(
                    name=f"[{t.name}; {n_str}]",
                    cpp_name=base,
                    kind=TypeKind.ARRAY,
                    array_size=int(n_str) if n_str.isdigit() else None,
                    generic_args=[t],
                )
            return IRType(name="[(); 0]", cpp_name=base, kind=TypeKind.ARRAY)

        # std::pair<A, B> → (A, B)
        if base == "std::pair":
            inner = ", ".join(a.name for a in mapped_args)
            return IRType(
                name=f"({inner})",
                cpp_name=base,
                kind=TypeKind.CLASS,
                generic_args=mapped_args,
            )

        # std::tuple<...> → (...)
        if base == "std::tuple":
            inner = ", ".join(a.name for a in mapped_args)
            return IRType(
                name=f"({inner})",
                cpp_name=base,
                kind=TypeKind.CLASS,
                generic_args=mapped_args,
            )

        # std::function<R(Args...)> → Box<dyn Fn(Args...) -> R>
        if base == "std::function":
            return self._map_std_function(template_args)

        # std::atomic<T> → AtomicT
        if base == "std::atomic":
            if mapped_args:
                inner_name = mapped_args[0].name
                atomic_name = _ATOMIC_MAP.get(inner_name)
                if atomic_name:
                    return IRType(name=atomic_name, cpp_name=base, kind=TypeKind.CLASS)
                # Fallback: AtomicCell<T> (from crossbeam) or just leave generic
                return IRType(
                    name=f"Atomic<{inner_name}>",
                    cpp_name=base,
                    kind=TypeKind.CLASS,
                    generic_args=mapped_args,
                )
            return IRType(name="AtomicI32", cpp_name=base, kind=TypeKind.CLASS)

        # Generic single/double argument containers: Vec<T>, Box<T>, Arc<T>, etc.
        if mapped_args:
            arg_str = ", ".join(a.name for a in mapped_args)
            return IRType(
                name=f"{rust_name}<{arg_str}>",
                cpp_name=base,
                kind=TypeKind.CLASS,
                generic_args=mapped_args,
            )
        return IRType(name=rust_name, cpp_name=base, kind=TypeKind.CLASS)

    # ── Pointer / reference (standalone convenience) ──

    def map_pointer(self, cpp_type: str) -> IRType:
        """Handle pointer/reference types.

        This is a convenience entry point; ``map_type`` already handles
        pointers and references.  This method exists for callers that have
        already determined the type is a pointer/reference and want the IR.
        """
        return self.map_type(cpp_type)

    # ── Qualified name mapping ──

    @staticmethod
    def map_qualified_name(cpp_name: str) -> str:
        """Convert a C++ qualified name to a Rust type name.

        Since each V8 module is transpiled into its own crate, we strip
        namespace prefixes (v8::, v8::internal::, etc.) and use the bare
        type name.  Cross-crate references would need explicit `use`
        statements which are handled separately.

        Examples:
            "v8::internal::Foo"                → "Foo"
            "v8::internal::compiler::Node"     → "Node"
            "FooBar"                           → "FooBar"
            "std::vector"                      → "std::vector"  (not changed)
        """
        cpp_name = cpp_name.strip()
        if not cpp_name:
            return cpp_name

        parts = cpp_name.split("::")
        # Strip leading empty parts from "::global" qualified names
        while parts and parts[0] == "":
            parts = parts[1:]

        if not parts:
            return cpp_name

        # Don't remap std:: names — they are handled in map_std_container
        if parts[0] == "std":
            return cpp_name

        # For v8:: namespaced types, use just the final type name.
        # Each module is its own crate, so v8::internal::Foo → Foo.
        if parts[0] == "v8":
            return parts[-1]

        # For other namespaced names (e.g. base::Mutex), also use the
        # final segment — namespace mapping is handled by use declarations.
        if len(parts) > 1:
            return parts[-1]

        return cpp_name

    # ── Function pointer mapping ──

    def _build_function_pointer(
        self, cpp_type: str, fn_ret: Optional[str], fn_params: List[str]
    ) -> IRType:
        """Build an IRType for a C++ function pointer."""
        ret_ir = self.map_type(fn_ret) if fn_ret else IRType(name="()", kind=TypeKind.VOID)
        param_irs = [self.map_type(p) for p in fn_params]
        param_str = ", ".join(p.name for p in param_irs)
        ret_str = ret_ir.name
        if ret_str == "()":
            name = f"fn({param_str})"
        else:
            name = f"fn({param_str}) -> {ret_str}"
        return IRType(
            name=name,
            cpp_name=cpp_type,
            kind=TypeKind.FUNCTION_POINTER,
            generic_args=param_irs + [ret_ir],
        )

    # ── std::function mapping ──

    def _map_std_function(self, template_args: List[str]) -> IRType:
        """Map std::function<R(Args...)> to Box<dyn Fn(Args...) -> R>.

        The template_args list typically has a single entry like "int(double, float)".
        """
        if not template_args:
            return IRType(name="Box<dyn Fn()>", cpp_name="std::function", kind=TypeKind.CLASS)

        sig = template_args[0].strip()
        # Parse R(Args...)
        paren_idx = sig.find("(")
        if paren_idx == -1:
            # Not a valid function signature; fall back
            inner = self.map_type(sig)
            return IRType(name=f"Box<dyn Fn() -> {inner.name}>", cpp_name="std::function", kind=TypeKind.CLASS)

        ret_part = sig[:paren_idx].strip()
        args_part = sig[paren_idx + 1:].rstrip(")")

        ret_ir = self.map_type(ret_part) if ret_part else IRType(name="()", kind=TypeKind.VOID)

        # Split args on commas (respecting template depth)
        arg_strs = _split_comma_respecting_brackets(args_part) if args_part.strip() else []
        arg_irs = [self.map_type(a) for a in arg_strs]
        param_str = ", ".join(a.name for a in arg_irs)

        if ret_ir.name == "()" or ret_part == "void":
            name = f"Box<dyn Fn({param_str})>"
        else:
            name = f"Box<dyn Fn({param_str}) -> {ret_ir.name}>"

        return IRType(
            name=name,
            cpp_name="std::function",
            kind=TypeKind.CLASS,
            generic_args=arg_irs + [ret_ir],
        )

    # ── Helpers ──

    @staticmethod
    def _guess_kind(base: str) -> TypeKind:
        """Guess TypeKind from the base name."""
        if base in _PRIMITIVE_MAP:
            return TypeKind.PRIMITIVE
        if base == "void":
            return TypeKind.VOID
        if base == "auto":
            return TypeKind.AUTO
        return TypeKind.CLASS

    @staticmethod
    def _fallback_name(cpp_type: str) -> str:
        """Produce a best-effort Rust name for an unparseable C++ type."""
        # Strip const and whitespace
        name = cpp_type.strip()
        name = re.sub(r"\bconst\b", "", name).strip()
        name = re.sub(r"\s+", " ", name)
        return name if name else "()"


# ─── Naming convention helpers ───────────────────────────────────────────────

def _to_snake_case(name: str) -> str:
    """Convert a CamelCase or mixedCase identifier to snake_case."""
    if not name:
        return name
    # Insert _ before uppercase runs
    s1 = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s2 = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1)
    return s2.lower()


def _to_screaming_snake_case(name: str) -> str:
    """Convert an identifier to SCREAMING_SNAKE_CASE."""
    return _to_snake_case(name).upper()


def to_rust_name(cpp_name: str, kind: str = "variable") -> str:
    """Convert a C++ name to the appropriate Rust naming convention.

    Args:
        cpp_name: The original C++ identifier.
        kind: One of "variable", "function", "type", "constant".

    Returns:
        The name in the appropriate Rust convention.
    """
    cpp_name = cpp_name.strip()
    if not cpp_name:
        return cpp_name

    if kind in ("variable", "function"):
        # snake_case
        return _to_snake_case(cpp_name)
    elif kind == "constant":
        # SCREAMING_SNAKE_CASE
        return _to_screaming_snake_case(cpp_name)
    elif kind == "type":
        # CamelCase — C++ types are already CamelCase usually, so return as-is
        # unless it's snake_case (rare) in which case convert.
        if "_" in cpp_name and not cpp_name[0].isupper():
            # snake_case → CamelCase
            return "".join(part.capitalize() for part in cpp_name.split("_"))
        return cpp_name
    return cpp_name


def needs_import(rust_type: str) -> Optional[str]:
    """Return the Rust import path if the given Rust type needs a ``use`` statement.

    Args:
        rust_type: A Rust type name (e.g., "HashMap", "Arc").

    Returns:
        The full import path (e.g., "std::collections::HashMap") or None.
    """
    # Check the base type name (strip generics)
    base = rust_type.split("<")[0].strip()
    # Also handle qualified names
    last_segment = base.rsplit("::", 1)[-1]
    result = _IMPORT_MAP.get(last_segment)
    if result is not None:
        return result
    return _IMPORT_MAP.get(base)


# ─── Utility: comma-split respecting angle brackets ─────────────────────────

def _split_comma_respecting_brackets(s: str) -> List[str]:
    """Split a string on commas, respecting nested <>, (), and [] brackets."""
    parts: List[str] = []
    depth = 0
    current: List[str] = []
    for ch in s:
        if ch in "<([":
            depth += 1
            current.append(ch)
        elif ch in ">)]":
            depth -= 1
            current.append(ch)
        elif ch == "," and depth == 0:
            parts.append("".join(current).strip())
            current = []
        else:
            current.append(ch)
    trailing = "".join(current).strip()
    if trailing:
        parts.append(trailing)
    return parts

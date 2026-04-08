"""C++ template → Rust generic mapper.

Maps C++ template declarations and instantiations to Rust generics,
const generics, and trait bounds.
"""

from __future__ import annotations

import re
from typing import Dict, List, Optional, Tuple

from ir.nodes import IRTemplateParam, IRType, TypeKind


class TemplateMapper:
    """Maps C++ templates to Rust generics."""

    def __init__(self, type_mapper=None):
        self.type_mapper = type_mapper

        # CRTP pattern detection: class Derived : public Base<Derived>
        self._crtp_cache: Dict[str, str] = {}

        # Known trait bound mappings for SFINAE / enable_if patterns
        self._sfinae_to_bounds: Dict[str, str] = {
            "std::is_integral": "num_traits::PrimInt",
            "std::is_floating_point": "num_traits::Float",
            "std::is_arithmetic": "num_traits::Num",
            "std::is_pointer": "",  # Raw pointers, no good Rust bound
            "std::is_same": "",  # Equality constraint
            "std::is_base_of": "",  # Inheritance check → trait bound
            "std::is_convertible": "Into",
            "std::is_trivially_copyable": "Copy",
            "std::is_trivially_destructible": "",
            "std::is_default_constructible": "Default",
            "std::is_copy_constructible": "Clone",
            "std::is_move_constructible": "",  # All Rust types are movable
            "std::is_enum": "",
            "std::is_class": "",
            "std::is_void": "",
        }

    def map_template_params(self, params: List[dict]) -> List[IRTemplateParam]:
        """Map C++ template parameters to Rust generic parameters.

        C++ template<typename T, typename U = int, int N = 10>
        → Rust <T, U = i32, const N: usize = 10>
        """
        result = []
        for p in params:
            if p.get("is_type_param", True):
                ir_param = IRTemplateParam(
                    name=p.get("name", "T"),
                    is_type_param=True,
                    constraint=self._infer_trait_bound(p),
                    default=IRType(name=self._map_default_type(p.get("default_value", "")))
                    if p.get("default_value") else None,
                )
            else:
                # Non-type template parameter → const generic
                value_type = p.get("value_type", "int")
                ir_param = IRTemplateParam(
                    name=p.get("name", "N"),
                    is_type_param=False,
                    value_type=IRType(
                        name=self._map_nontype_param_type(value_type),
                        kind=TypeKind.PRIMITIVE,
                    ),
                    default=IRType(name=p["default_value"]) if p.get("default_value") else None,
                )
            result.append(ir_param)

        return result

    def map_template_args(self, args: List[str]) -> List[IRType]:
        """Map C++ template arguments to Rust generic arguments.

        e.g., <int, std::string, 42> → <i32, String, 42>
        """
        result = []
        for arg in args:
            arg = arg.strip()
            if self.type_mapper:
                result.append(self.type_mapper.map_type(arg))
            else:
                result.append(IRType(name=arg, cpp_name=arg))
        return result

    def detect_crtp(self, class_name: str, base_classes: List[dict]) -> Optional[str]:
        """Detect CRTP pattern: class Derived : public Base<Derived>.

        Returns the base class name if CRTP is detected, None otherwise.
        CRTP in Rust becomes a trait with Self bound.
        """
        for base in base_classes:
            base_name = base.get("name", "")
            # Check if base class is templated with the derived class as argument
            pattern = rf"^(\w+)<{re.escape(class_name)}>$"
            m = re.match(pattern, base_name)
            if m:
                crtp_base = m.group(1)
                self._crtp_cache[class_name] = crtp_base
                return crtp_base
        return None

    def map_crtp_to_trait(self, base_name: str) -> str:
        """Convert a CRTP base class to a Rust trait pattern.

        C++:  template<typename Derived> class Base { ... }
              class Foo : public Base<Foo> { ... }
        Rust: trait Base { fn method(&self) -> Self; }
              impl Base for Foo { ... }
        """
        return base_name  # The trait name is the base class name

    def map_template_specialization(self, class_name: str,
                                     specialization_args: List[str]) -> str:
        """Map C++ template specialization.

        Full specialization: template<> class Foo<int> → separate struct FooInt
        Partial specialization: template<typename T> class Foo<T*> → conditional impl
        """
        if all(self._is_concrete_type(a) for a in specialization_args):
            # Full specialization → generate a separate type
            suffix = "_".join(self._type_to_suffix(a) for a in specialization_args)
            return f"{class_name}_{suffix}"
        else:
            # Partial specialization → will need conditional impl with where clause
            return class_name

    def map_sfinae(self, enable_if_condition: str) -> Optional[str]:
        """Map std::enable_if / SFINAE patterns to Rust trait bounds.

        C++: template<typename T, typename = std::enable_if_t<std::is_integral_v<T>>>
        Rust: fn foo<T: num_traits::PrimInt>(...)
        """
        # Extract the type trait from the condition
        for cpp_trait, rust_bound in self._sfinae_to_bounds.items():
            if cpp_trait in enable_if_condition:
                return rust_bound if rust_bound else None

        return None

    def map_variadic_template(self, param_name: str) -> str:
        """Map C++ variadic templates (typename... Args).

        Rust doesn't have variadic generics, so we use macro_rules! or tuples.
        Returns a comment indicating manual translation is needed.
        """
        return f"/* TODO: variadic template {param_name}... - use macro_rules! or tuple */"

    # ─── Helper Methods ──────────────────────────────────────────────────

    def _infer_trait_bound(self, param: dict) -> Optional[str]:
        """Try to infer a Rust trait bound for a template type parameter."""
        # Look at the default type for hints
        default = param.get("default_value", "")
        if default:
            if default in ("int", "size_t", "uint32_t"):
                return None  # Numeric default doesn't imply a bound

        # No constraint by default — C++ templates are duck-typed
        return None

    def _map_default_type(self, cpp_type: str) -> str:
        """Map a C++ default template argument type to Rust."""
        if not cpp_type:
            return ""
        if self.type_mapper:
            return self.type_mapper.map_type(cpp_type).name
        return cpp_type

    def _map_nontype_param_type(self, cpp_type: str) -> str:
        """Map a non-type template parameter's type to Rust const generic type."""
        mapping = {
            "int": "i32",
            "unsigned": "u32",
            "unsigned int": "u32",
            "size_t": "usize",
            "bool": "bool",
            "char": "u8",
            "long": "i64",
            "unsigned long": "u64",
            "int64_t": "i64",
            "uint64_t": "u64",
            "int32_t": "i32",
            "uint32_t": "u32",
        }
        return mapping.get(cpp_type.strip(), "usize")

    def _is_concrete_type(self, type_str: str) -> bool:
        """Check if a template argument is a concrete type (not a type parameter)."""
        type_str = type_str.strip()
        # If it's a single uppercase letter, it's likely a type parameter
        if len(type_str) == 1 and type_str.isupper():
            return False
        # If it contains *, &, it's a parameterized form
        if "*" in type_str or "&" in type_str:
            return False
        return True

    def _type_to_suffix(self, type_str: str) -> str:
        """Convert a type to a suffix for specialized type names."""
        mapping = {
            "int": "I32",
            "unsigned int": "U32",
            "float": "F32",
            "double": "F64",
            "bool": "Bool",
            "char": "Char",
            "int32_t": "I32",
            "uint32_t": "U32",
            "int64_t": "I64",
            "uint64_t": "U64",
            "size_t": "Usize",
        }
        clean = type_str.strip()
        return mapping.get(clean, clean.replace("::", "_").replace("<", "_").replace(">", ""))

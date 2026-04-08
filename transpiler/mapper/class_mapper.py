"""C++ class hierarchy → Rust struct/trait mapper.

Maps C++ classes with inheritance to Rust structs with composition and traits.
Handles single inheritance, multiple inheritance, abstract classes,
virtual dispatch, constructors, destructors, and operator overloading.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Set

sys.path.insert(0, str(Path(__file__).parent.parent))

from ir.nodes import (
    IRModule, IRFile, IRStruct, IRTrait, IRImplBlock, IREnum, IRTypeAlias,
    IRFunction, IRField, IRParam, IRBaseClass, IRConst, IRType, IRBlock,
    IRTemplateParam, IREnumVariant, IRUseDecl, IRItem,
    AccessSpecifier, TypeKind,
)
from config import TranspilerConfig


class ClassMapper:
    """Maps C++ class hierarchies to Rust structs and traits."""

    def __init__(self, type_mapper=None):
        self.type_mapper = type_mapper
        self._operator_trait_map = {
            "operator+": ("Add", "add", "Output"),
            "operator-": ("Sub", "sub", "Output"),
            "operator*": ("Mul", "mul", "Output"),
            "operator/": ("Div", "div", "Output"),
            "operator%": ("Rem", "rem", "Output"),
            "operator&": ("BitAnd", "bitand", "Output"),
            "operator|": ("BitOr", "bitor", "Output"),
            "operator^": ("BitXor", "bitxor", "Output"),
            "operator<<": ("Shl", "shl", "Output"),
            "operator>>": ("Shr", "shr", "Output"),
            "operator==": ("PartialEq", "eq", None),
            "operator!=": ("PartialEq", "ne", None),
            "operator<": ("PartialOrd", "lt", None),
            "operator<=": ("PartialOrd", "le", None),
            "operator>": ("PartialOrd", "gt", None),
            "operator>=": ("PartialOrd", "ge", None),
            "operator[]": ("Index", "index", "Output"),
            "operator()": ("Fn", "call", "Output"),
            "operator+=": ("AddAssign", "add_assign", None),
            "operator-=": ("SubAssign", "sub_assign", None),
            "operator*=": ("MulAssign", "mul_assign", None),
            "operator/=": ("DivAssign", "div_assign", None),
            "operator!": ("Not", "not", "Output"),
            "operator-" : ("Neg", "neg", "Output"),  # unary minus
        }

    def map_module(self, module_name: str, parsed_data: dict,
                   config: TranspilerConfig) -> IRModule:
        """Map an entire parsed module to an IRModule."""
        crate_name = config.module_to_crate_name(module_name)
        ir_module = IRModule(
            name=module_name,
            crate_name=crate_name,
            files=[],
            dependencies=[],
        )

        for file_data in parsed_data.get("files", []):
            ir_file = self._map_file(file_data, module_name, config)
            if ir_file and ir_file.items:
                ir_module.files.append(ir_file)

        return ir_module

    def _map_file(self, file_data: dict, module_name: str,
                  config: TranspilerConfig) -> Optional[IRFile]:
        """Map a single parsed file to an IRFile."""
        cpp_path = file_data.get("path", "")
        if not cpp_path:
            return None

        # Determine Rust file path
        rust_path = self._cpp_to_rust_path(cpp_path, module_name)

        ir_file = IRFile(
            path=rust_path,
            cpp_source_files=[cpp_path],
            uses=[],
            items=[],
        )

        # Process top-level declarations
        self._process_declarations(file_data, ir_file)

        # Process namespace contents
        for ns_data in file_data.get("namespaces", []):
            self._process_namespace(ns_data, ir_file)

        # Deduplicate items within the file (e.g. template specializations
        # producing multiple structs with the same name).
        self._deduplicate_items(ir_file)

        # Disambiguate overloaded functions / methods
        self._disambiguate_overloads(ir_file)

        return ir_file

    def _process_declarations(self, data: dict, ir_file: IRFile):
        """Process top-level declarations from parsed data."""
        # Classes
        for cls_data in data.get("classes", []):
            items = self.map_class(cls_data)
            ir_file.items.extend(items)

        # Enums
        for enum_data in data.get("enums", []):
            ir_enum = self.map_enum(enum_data)
            if ir_enum:
                ir_file.items.append(ir_enum)

        # Free functions
        for func_data in data.get("functions", []):
            ir_func = self.map_function(func_data)
            if ir_func:
                ir_file.items.append(ir_func)

        # Typedefs
        for td_data in data.get("typedefs", []):
            ir_alias = self.map_typedef(td_data)
            if ir_alias:
                ir_file.items.append(ir_alias)

        # Variables/constants
        for var_data in data.get("variables", []):
            ir_const = self.map_variable(var_data)
            if ir_const:
                ir_file.items.append(ir_const)

    def _process_namespace(self, ns_data: dict, ir_file: IRFile):
        """Process declarations within a namespace."""
        # Recursively process all declarations in the namespace
        self._process_declarations(ns_data, ir_file)

        # Process nested namespaces
        for sub_ns in ns_data.get("namespaces", []):
            self._process_namespace(sub_ns, ir_file)

    # ─── Class Mapping ───────────────────────────────────────────────────

    def map_class(self, cls_data: dict) -> List[IRItem]:
        """Map a C++ class to Rust items (struct + trait + impl blocks).

        Returns a list of items because one C++ class may produce:
        - A struct definition
        - A trait (if abstract)
        - Inherent impl block
        - Trait impl blocks (for base class traits)
        - Operator trait impls
        """
        items: List[IRItem] = []
        name = cls_data.get("name", "")
        if not name:
            return items

        is_abstract = cls_data.get("is_abstract", False)
        base_classes = cls_data.get("base_classes", [])

        # 1. If abstract, generate a trait
        if is_abstract:
            trait = self._make_trait(cls_data)
            if trait:
                items.append(trait)

        # 2. Generate the struct
        struct = self._make_struct(cls_data)
        if struct:
            items.append(struct)

        # 3. Generate inherent impl block (non-virtual methods, constructors)
        impl_block = self._make_inherent_impl(cls_data)
        if impl_block and impl_block.methods:
            items.append(impl_block)

        # 4. Generate trait impls for base classes that are abstract
        for base in base_classes:
            if base.get("access", "public") == "public":
                trait_impl = self._make_base_trait_impl(cls_data, base)
                if trait_impl and trait_impl.methods:
                    items.append(trait_impl)

        # 5. Generate operator trait impls
        operator_impls = self._make_operator_impls(cls_data)
        items.extend(operator_impls)

        # 6. Generate Drop impl if non-trivial destructor
        drop_impl = self._make_drop_impl(cls_data)
        if drop_impl:
            items.append(drop_impl)

        # 7. Process nested types
        for nested_enum in cls_data.get("nested_enums", []):
            ir_enum = self.map_enum(nested_enum)
            if ir_enum:
                items.append(ir_enum)

        for nested_cls in cls_data.get("nested_classes", []):
            nested_items = self.map_class(nested_cls)
            items.extend(nested_items)

        return items

    def _make_struct(self, cls_data: dict) -> IRStruct:
        """Create an IRStruct from class data."""
        name = cls_data["name"]
        fields = []

        # Add base class as first field (composition for single inheritance)
        for i, base in enumerate(cls_data.get("base_classes", [])):
            if i == 0 and base.get("access", "public") == "public":
                # Primary base → embedded field
                base_name = base.get("name", "").split("::")[-1]
                fields.append(IRField(
                    name="base",
                    type=IRType(name=base_name, kind=TypeKind.CLASS),
                    access=AccessSpecifier.PUBLIC,
                    comment=f"Base class: {base.get('name', '')}",
                ))

        # Add own fields
        for field_data in cls_data.get("fields", []):
            if field_data.get("is_static", False):
                continue  # Static fields go to associated consts
            ir_field = self._map_field(field_data)
            if ir_field:
                fields.append(ir_field)

        # Determine derives
        derives = self._determine_derives(cls_data)

        # Template params
        template_params = self._map_template_params(cls_data.get("template_params", []))

        # Base classes for metadata
        ir_bases = [
            IRBaseClass(
                name=b.get("name", "").split("::")[-1],
                qualified_name=b.get("qualified_name", b.get("name", "")),
                access=self._map_access(b.get("access", "public")),
                is_virtual=b.get("is_virtual", False),
            )
            for b in cls_data.get("base_classes", [])
        ]

        return IRStruct(
            name=name,
            qualified_name=cls_data.get("qualified_name", name),
            fields=fields,
            methods=[],  # Methods go in impl blocks
            base_classes=ir_bases,
            template_params=template_params,
            is_abstract=cls_data.get("is_abstract", False),
            is_class=not cls_data.get("is_struct", False),
            visibility=AccessSpecifier.PUBLIC,
            derives=derives,
            cpp_name=name,
            source_file=cls_data.get("file", ""),
            source_line=cls_data.get("line", 0),
        )

    def _make_trait(self, cls_data: dict) -> Optional[IRTrait]:
        """Create a trait from an abstract class (pure virtual methods)."""
        methods = []
        for m in cls_data.get("methods", []):
            if m.get("is_pure_virtual", False):
                ir_func = self.map_function(m, is_trait_method=True, is_class_method=True)
                if ir_func:
                    ir_func.body = None  # No default impl
                    methods.append(ir_func)
            elif m.get("is_virtual", False) and not m.get("is_destructor", False):
                # Virtual but not pure → default impl in trait
                ir_func = self.map_function(m, is_trait_method=True, is_class_method=True)
                if ir_func:
                    methods.append(ir_func)

        if not methods:
            return None

        name = cls_data["name"]

        # Supertraits from base classes
        supertraits = []
        for base in cls_data.get("base_classes", []):
            base_name = base.get("name", "").split("::")[-1]
            if base_name:
                supertraits.append(base_name)

        return IRTrait(
            name=name,
            qualified_name=cls_data.get("qualified_name", name),
            methods=methods,
            supertraits=supertraits,
            visibility=AccessSpecifier.PUBLIC,
        )

    def _make_inherent_impl(self, cls_data: dict) -> Optional[IRImplBlock]:
        """Create an inherent impl block for non-virtual methods and constructors."""
        methods = []
        name = cls_data["name"]

        for m in cls_data.get("methods", []):
            # Skip pure virtual (they go in the trait)
            if m.get("is_pure_virtual", False):
                continue
            # Skip virtual methods if the class is abstract (they go in trait)
            if m.get("is_virtual", False) and cls_data.get("is_abstract", False):
                continue
            # Skip operators (they get their own trait impls)
            if m.get("is_operator", False):
                continue
            # Skip destructors (they go in Drop impl)
            if m.get("is_destructor", False):
                continue
            # Skip deleted/defaulted copy/move
            if m.get("is_deleted", False):
                continue

            ir_func = self.map_function(m, is_class_method=True)
            if ir_func:
                methods.append(ir_func)

        # Add static fields as associated constants
        for field_data in cls_data.get("fields", []):
            if field_data.get("is_static", False) and field_data.get("is_const", False):
                const_fn = IRFunction(
                    name=self._to_screaming_snake(field_data["name"]),
                    return_type=self._map_type_str(field_data.get("type", "")),
                    is_static=True,
                    access=self._map_access(field_data.get("access", "public")),
                    cpp_name=field_data["name"],
                )
                # We'll represent this as an associated const, not a method
                # But for now, include it in the impl block

        return IRImplBlock(
            struct_name=name,
            methods=methods,
        )

    def _make_base_trait_impl(self, cls_data: dict, base: dict) -> Optional[IRImplBlock]:
        """Create a trait impl block for implementing a base class's trait."""
        name = cls_data["name"]
        base_name = base.get("name", "").split("::")[-1]

        # Find override methods
        methods = []
        for m in cls_data.get("methods", []):
            if m.get("is_override", False) or m.get("is_virtual", False):
                if not m.get("is_pure_virtual", False):
                    ir_func = self.map_function(m, is_trait_method=True, is_class_method=True)
                    if ir_func:
                        methods.append(ir_func)

        if not methods:
            return None

        return IRImplBlock(
            struct_name=name,
            trait_name=base_name,
            methods=methods,
        )

    def _make_operator_impls(self, cls_data: dict) -> List[IRImplBlock]:
        """Create trait impl blocks for operator overloading."""
        impls = []
        name = cls_data["name"]

        for m in cls_data.get("methods", []):
            if not m.get("is_operator", False):
                continue

            op_name = m.get("operator_name", m.get("name", ""))
            if op_name in self._operator_trait_map:
                trait_name, method_name, output = self._operator_trait_map[op_name]

                ir_func = self.map_function(m, is_class_method=True)
                if ir_func:
                    ir_func.name = method_name
                    impls.append(IRImplBlock(
                        struct_name=name,
                        trait_name=f"std::ops::{trait_name}",
                        methods=[ir_func],
                    ))

        return impls

    def _make_drop_impl(self, cls_data: dict) -> Optional[IRImplBlock]:
        """Create a Drop impl if the class has a non-trivial destructor."""
        for m in cls_data.get("methods", []):
            if m.get("is_destructor", False) and m.get("has_body", False):
                drop_fn = IRFunction(
                    name="drop",
                    self_param="&mut self",
                    is_method=True,
                    return_type=IRType(name="()", kind=TypeKind.VOID),
                    body=IRBlock(),  # Body will be filled by stmt_mapper
                    cpp_name=m.get("name", ""),
                )
                return IRImplBlock(
                    struct_name=cls_data["name"],
                    trait_name="Drop",
                    methods=[drop_fn],
                )
        return None

    # ─── Function Mapping ────────────────────────────────────────────────

    def map_function(self, func_data: dict,
                     is_trait_method: bool = False,
                     is_class_method: bool = False) -> Optional[IRFunction]:
        """Map a C++ function/method to an IRFunction."""
        name = func_data.get("name", "")
        if not name:
            return None

        # Convert name to snake_case for methods/functions
        rust_name = name
        if func_data.get("is_constructor", False):
            rust_name = "new"
        elif func_data.get("is_destructor", False):
            rust_name = "drop"
        elif not func_data.get("is_operator", False):
            rust_name = self._to_snake_case(name)

        # Determine self parameter (only for class methods, not free functions)
        self_param = ""
        if (is_class_method or is_trait_method) and \
           not func_data.get("is_static", False) and \
           not func_data.get("is_constructor", False):
            if func_data.get("is_const", False):
                self_param = "&self"
            else:
                self_param = "&mut self"

        # Map parameters
        params = []
        for p in func_data.get("params", []):
            ir_param = IRParam(
                name=self._to_snake_case(p.get("name", "arg")),
                type=self._map_type_str(p.get("type", "")),
                cpp_name=p.get("name", ""),
            )
            params.append(ir_param)

        # Map return type
        ret_type_str = func_data.get("return_type", "void")
        if func_data.get("is_constructor", False):
            ret_type = IRType(name="Self", kind=TypeKind.CLASS)
        elif ret_type_str in ("void", ""):
            ret_type = IRType(name="()", kind=TypeKind.VOID)
        else:
            ret_type = self._map_type_str(ret_type_str)

        # Template params
        template_params = self._map_template_params(func_data.get("template_params", []))

        # Attributes
        attributes = []
        if func_data.get("is_inline", False):
            attributes.append("inline")

        # Reconstruct body from IR dict if available, else empty IRBlock
        body = None
        if func_data.get("has_body", False):
            body_ir = func_data.get("body_ir")
            if body_ir is not None:
                from ir.nodes import ir_from_dict
                body = ir_from_dict(body_ir)
                if not isinstance(body, IRBlock):
                    body = IRBlock(statements=[body] if body else [])
                # Remap C++ types in body to Rust types
                self._remap_body_types(body)
            else:
                body = IRBlock()

        return IRFunction(
            name=rust_name,
            qualified_name=func_data.get("qualified_name", name),
            params=params,
            return_type=ret_type,
            body=body,
            is_method=bool(self_param),
            self_param=self_param,
            is_static=func_data.get("is_static", False),
            is_virtual=func_data.get("is_virtual", False),
            is_pure_virtual=func_data.get("is_pure_virtual", False),
            is_override=func_data.get("is_override", False),
            is_const=func_data.get("is_const", False),
            is_inline=func_data.get("is_inline", False),
            is_constexpr=func_data.get("is_constexpr", False),
            template_params=template_params,
            access=self._map_access(func_data.get("access", "public")),
            cpp_name=name,
            source_file=func_data.get("file", ""),
            source_line=func_data.get("line", 0),
            attributes=attributes,
        )

    # ─── Enum Mapping ────────────────────────────────────────────────────

    def map_enum(self, enum_data: dict) -> Optional[IREnum]:
        """Map a C++ enum to an IREnum."""
        name = enum_data.get("name", "")
        if not name:
            return None

        variants = []
        for v in enum_data.get("variants", []):
            variant = IREnumVariant(
                name=self._to_screaming_snake(v.get("name", "")),
                value=v.get("value_str", None) or (str(v["value"]) if v.get("value") is not None else None),
                cpp_name=v.get("name", ""),
            )
            variants.append(variant)

        # Map underlying type
        underlying = None
        if enum_data.get("underlying_type"):
            underlying = self._map_type_str(enum_data["underlying_type"])

        return IREnum(
            name=name,
            qualified_name=enum_data.get("qualified_name", name),
            variants=variants,
            underlying_type=underlying,
            is_scoped=enum_data.get("is_scoped", True),
            visibility=self._map_access(enum_data.get("access", "public")),
            cpp_name=name,
            source_file=enum_data.get("file", ""),
            source_line=enum_data.get("line", 0),
        )

    # ─── Typedef Mapping ─────────────────────────────────────────────────

    def map_typedef(self, td_data: dict) -> Optional[IRTypeAlias]:
        """Map a C++ typedef/using to an IRTypeAlias."""
        name = td_data.get("name", "")
        if not name:
            return None

        target_type = self._map_type_str(
            td_data.get("underlying_type", "") or td_data.get("canonical_type", "")
        )

        template_params = self._map_template_params(td_data.get("template_params", []))

        return IRTypeAlias(
            name=name,
            qualified_name=td_data.get("qualified_name", name),
            target=target_type,
            template_params=template_params,
            visibility=AccessSpecifier.PUBLIC,
            cpp_name=name,
            source_file=td_data.get("file", ""),
            source_line=td_data.get("line", 0),
        )

    # ─── Variable Mapping ────────────────────────────────────────────────

    def map_variable(self, var_data: dict) -> Optional[IRConst]:
        """Map a C++ global/namespace variable to an IRConst."""
        name = var_data.get("name", "")
        if not name:
            return None

        is_const = var_data.get("is_const", False) or var_data.get("is_constexpr", False)

        return IRConst(
            name=self._to_screaming_snake(name) if is_const else self._to_snake_case(name),
            qualified_name=var_data.get("qualified_name", name),
            type=self._map_type_str(var_data.get("type", "")),
            value=var_data.get("value", None) or None,
            is_static=not is_const,
            visibility=AccessSpecifier.PUBLIC,
            cpp_name=name,
            source_file=var_data.get("file", ""),
            source_line=var_data.get("line", 0),
        )

    # ─── Body Type Remapping ─────────────────────────────────────────────

    def _remap_body_types(self, node):
        """Recursively walk an IR node tree and remap all IRType instances
        through TypeMapper so C++ types (uint64_t, etc.) become Rust types."""
        if node is None or self.type_mapper is None:
            return

        import dataclasses
        if not dataclasses.is_dataclass(node):
            return

        for f in dataclasses.fields(node):
            value = getattr(node, f.name)
            if value is None:
                continue

            if isinstance(value, IRType):
                cpp = value.cpp_name or value.name
                if cpp and cpp not in ("()", ""):
                    mapped = self.type_mapper.map_type(cpp)
                    setattr(node, f.name, mapped)
            elif isinstance(value, list):
                for i, item in enumerate(value):
                    if isinstance(item, IRType):
                        cpp = item.cpp_name or item.name
                        if cpp and cpp not in ("()", ""):
                            mapped = self.type_mapper.map_type(cpp)
                            value[i] = mapped
                    elif dataclasses.is_dataclass(item):
                        self._remap_body_types(item)
            elif dataclasses.is_dataclass(value):
                self._remap_body_types(value)

    # ─── Item Deduplication ────────────────────────────────────────────────

    @staticmethod
    def _deduplicate_items(ir_file: IRFile):
        """Remove duplicate items within a file.

        Template specializations and forward declarations can produce
        multiple structs/enums/type-aliases/consts with the same name.
        Keep the first occurrence of each (type, name) pair.
        """
        seen: Set[str] = set()
        deduped: list = []
        for item in ir_file.items:
            type_name = type(item).__name__
            name = getattr(item, "name", None)
            if name:
                if isinstance(item, IRImplBlock):
                    key = f"{type_name}:{item.struct_name}:{item.trait_name or ''}"
                else:
                    key = f"{type_name}:{name}"
                if key in seen:
                    continue
                seen.add(key)
            deduped.append(item)
        ir_file.items = deduped

    # ─── Overload Disambiguation ─────────────────────────────────────────

    def _disambiguate_overloads(self, ir_file: IRFile):
        """Rename duplicate function/method names to avoid Rust conflicts.

        For overloaded functions, appends a suffix based on parameter count
        and types.  Also handles duplicate names within impl blocks.
        """
        # Disambiguate top-level free functions
        self._disambiguate_item_list(ir_file.items)

        # Disambiguate methods inside impl blocks
        for item in ir_file.items:
            if isinstance(item, IRImplBlock):
                self._disambiguate_fn_list(item.methods)

    @staticmethod
    def _disambiguate_item_list(items: list):
        """Disambiguate IRFunction items in a mixed item list."""
        from collections import Counter
        # Count function names
        fn_names = Counter()
        for item in items:
            if isinstance(item, IRFunction):
                fn_names[item.name] += 1

        # First pass: rename with type-based suffixes
        seen: Dict[str, int] = {}
        for item in items:
            if isinstance(item, IRFunction) and fn_names[item.name] > 1:
                key = item.name
                idx = seen.get(key, 0)
                seen[key] = idx + 1
                if idx > 0:
                    suffix = ClassMapper._overload_suffix(item, idx)
                    item.name = f"{key}{suffix}"

        # Second pass: if type-based suffixes still collide, add numeric index
        fn_names2 = Counter()
        for item in items:
            if isinstance(item, IRFunction):
                fn_names2[item.name] += 1
        seen2: Dict[str, int] = {}
        for item in items:
            if isinstance(item, IRFunction) and fn_names2[item.name] > 1:
                key = item.name
                idx = seen2.get(key, 0)
                seen2[key] = idx + 1
                if idx > 0:
                    item.name = f"{key}_{idx}"

    @staticmethod
    def _disambiguate_fn_list(methods: list):
        """Disambiguate IRFunction names within a method list."""
        from collections import Counter
        fn_names = Counter(m.name for m in methods if isinstance(m, IRFunction))
        seen: Dict[str, int] = {}
        for m in methods:
            if isinstance(m, IRFunction) and fn_names[m.name] > 1:
                key = m.name
                idx = seen.get(key, 0)
                seen[key] = idx + 1
                if idx > 0:
                    suffix = ClassMapper._overload_suffix(m, idx)
                    m.name = f"{key}{suffix}"
        # Second pass for remaining collisions
        fn_names2 = Counter(m.name for m in methods if isinstance(m, IRFunction))
        seen2: Dict[str, int] = {}
        for m in methods:
            if isinstance(m, IRFunction) and fn_names2[m.name] > 1:
                key = m.name
                idx = seen2.get(key, 0)
                seen2[key] = idx + 1
                if idx > 0:
                    m.name = f"{key}_{idx}"

    @staticmethod
    def _overload_suffix(func: IRFunction, idx: int) -> str:
        """Generate a disambiguation suffix for an overloaded function.

        Tries to use parameter type info for clarity; falls back to a numeric
        index if types are too complex.
        """
        if func.params:
            # Use first param type name for readability
            first_type = func.params[0].type.name if func.params[0].type else ""
            # Simplify: take last component, remove pointers/refs
            simple = first_type.split("::")[-1].rstrip("*& ").replace(" ", "_")
            simple = re.sub(r'[^a-zA-Z0-9_]', '', simple)
            if simple and simple != "Unknown":
                if len(func.params) > 1:
                    return f"_{simple}_{len(func.params)}"
                return f"_{simple}"
        return f"_{idx}"

    # ─── Helper Methods ──────────────────────────────────────────────────

    def _map_field(self, field_data: dict) -> Optional[IRField]:
        """Map a C++ field to an IRField."""
        name = field_data.get("name", "")
        if not name:
            return None

        # Strip trailing underscore (V8 convention for private fields)
        rust_name = self._to_snake_case(name.rstrip("_"))

        return IRField(
            name=rust_name,
            type=self._map_type_str(field_data.get("type", "")),
            access=self._map_access(field_data.get("access", "public")),
            is_static=field_data.get("is_static", False),
            is_const=field_data.get("is_const", False),
            cpp_name=name,
        )

    def _map_type_str(self, cpp_type: str) -> IRType:
        """Map a C++ type string to an IRType using the type_mapper if available."""
        if self.type_mapper:
            return self.type_mapper.map_type(cpp_type)
        # Fallback: minimal mapping
        return IRType(name=cpp_type or "Unknown", cpp_name=cpp_type)

    def _map_template_params(self, params: list) -> List[IRTemplateParam]:
        """Map extracted template params to IR template params."""
        result = []
        for p in params:
            # Map default and value_type through TypeMapper if available
            default_type = None
            if p.get("default_value"):
                if self.type_mapper:
                    default_type = self.type_mapper.map_type(p["default_value"])
                else:
                    default_type = IRType(name=p["default_value"])

            value_type = None
            if p.get("value_type"):
                if self.type_mapper:
                    value_type = self.type_mapper.map_type(p["value_type"])
                else:
                    value_type = IRType(name=p["value_type"])

            result.append(IRTemplateParam(
                name=p.get("name", "T"),
                is_type_param=p.get("is_type_param", True),
                default=default_type,
                value_type=value_type,
            ))
        return result

    def _map_access(self, access: str) -> AccessSpecifier:
        """Map C++ access specifier string to IR enum."""
        mapping = {
            "public": AccessSpecifier.PUBLIC,
            "protected": AccessSpecifier.PROTECTED,
            "private": AccessSpecifier.PRIVATE,
        }
        return mapping.get(access, AccessSpecifier.PUBLIC)

    def _determine_derives(self, cls_data: dict) -> List[str]:
        """Determine which #[derive(...)] traits to add to a struct.

        Avoids deriving traits that would conflict with manually-generated
        operator trait impls (E0119).
        """
        derives = []

        has_complex_fields = False
        for f in cls_data.get("fields", []):
            ftype = f.get("type", "")
            if any(x in ftype for x in ["*", "unique_ptr", "shared_ptr", "vector",
                                         "string", "map", "set"]):
                has_complex_fields = True
                break

        # Check if the class defines operators that map to Debug or Clone.
        methods = cls_data.get("methods", [])
        operator_names = {m.get("name", "") for m in methods if m.get("is_operator")}
        has_copy_ctor = any(m.get("is_copy_constructor", False) for m in methods)

        # operator<< often maps to Display/Debug impl — skip derive(Debug)
        has_debug_conflict = "operator<<" in operator_names
        # Copy constructor → Clone impl
        has_clone_conflict = has_copy_ctor or "operator=" in operator_names
        # operator== → PartialEq
        has_eq_operators = "operator==" in operator_names or "operator!=" in operator_names

        if not has_debug_conflict:
            derives.append("Debug")

        if not has_complex_fields and not cls_data.get("is_abstract", False) \
                and not has_clone_conflict:
            derives.append("Clone")

        return derives

    def _cpp_to_rust_path(self, cpp_path: str, module_name: str) -> str:
        """Convert a C++ file path to a Rust file path."""
        p = Path(cpp_path)
        stem = p.stem
        # Remove common suffixes
        for suffix in ["-inl"]:
            if stem.endswith(suffix):
                stem = stem[:-len(suffix)]

        # Convert to snake_case
        rust_name = self._to_snake_case(stem.replace("-", "_"))
        return f"src/{rust_name}.rs"

    # Rust keywords that cannot be used as identifiers without r# prefix.
    _RUST_KEYWORDS = frozenset({
        "as", "async", "await", "break", "const", "continue", "crate", "dyn",
        "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
        "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "self", "Self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while", "yield",
    })

    @staticmethod
    def _escape_rust_keyword(name: str) -> str:
        """Escape a name that is a Rust keyword using r# prefix."""
        if name in ClassMapper._RUST_KEYWORDS:
            return f"r#{name}"
        return name

    @staticmethod
    def _to_snake_case(name: str) -> str:
        """Convert CamelCase or mixedCase to snake_case."""
        if not name:
            return name
        # Insert underscore before uppercase letters
        s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
        s2 = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s1)
        result = s2.lower().strip("_")
        return ClassMapper._escape_rust_keyword(result)

    @staticmethod
    def _to_screaming_snake(name: str) -> str:
        """Convert to SCREAMING_SNAKE_CASE for constants."""
        if not name:
            return name
        # Check if already screaming snake
        if name == name.upper() and "_" in name:
            return name
        # Convert from camelCase/PascalCase
        s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
        s2 = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s1)
        result = s2.upper().strip("_")
        # Remove leading k prefix (V8 convention: kFoo → FOO)
        if result.startswith("K_") and len(result) > 2:
            result = result[2:]
        elif name.startswith("k") and name[1:2].isupper():
            result = ClassMapper._to_screaming_snake(name[1:])
        return result

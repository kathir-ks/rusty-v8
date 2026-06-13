"""Central type registry for the transpiler.

Stores all types found during C++ AST extraction and provides lookup,
resolution, and analysis capabilities. This is the single source of truth
for type information across the entire codebase.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple


@dataclass
class TypeInfo:
    """Information about a single type."""
    name: str
    qualified_name: str
    kind: str                          # "class", "struct", "enum", "typedef", "template"
    module: str                        # Which V8 module this belongs to
    source_file: str = ""
    source_line: int = 0
    base_classes: List[str] = field(default_factory=list)   # Qualified names
    is_abstract: bool = False
    is_template: bool = False
    template_params: List[str] = field(default_factory=list)
    fields: List[Dict[str, str]] = field(default_factory=list)  # name, type, access
    virtual_methods: List[str] = field(default_factory=list)
    pure_virtual_methods: List[str] = field(default_factory=list)
    nested_types: List[str] = field(default_factory=list)
    typedef_target: str = ""           # For typedefs: what they resolve to

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "qualified_name": self.qualified_name,
            "kind": self.kind,
            "module": self.module,
            "source_file": self.source_file,
            "source_line": self.source_line,
            "base_classes": self.base_classes,
            "is_abstract": self.is_abstract,
            "is_template": self.is_template,
            "template_params": self.template_params,
            "virtual_methods": self.virtual_methods,
            "pure_virtual_methods": self.pure_virtual_methods,
            "nested_types": self.nested_types,
            "typedef_target": self.typedef_target,
        }


class TypeRegistry:
    """Central registry of all types in the codebase."""

    def __init__(self):
        # qualified_name → TypeInfo
        self._types: Dict[str, TypeInfo] = {}

        # short name → list of qualified names (for ambiguous lookups)
        self._name_index: Dict[str, List[str]] = {}

        # module → list of qualified names in that module
        self._module_index: Dict[str, List[str]] = {}

        # Inheritance graph: child → [parents]
        self._parents: Dict[str, List[str]] = {}
        # Reverse: parent → [children]
        self._children: Dict[str, List[str]] = {}

        # Cached analysis results
        self._abstract_classes: Optional[Set[str]] = None
        self._v8_gc_types: Optional[Set[str]] = None

    # ─── Loading ─────────────────────────────────────────────────────────

    def load_module(self, module_name: str, parsed_data: dict):
        """Load types from a parsed module's AST data."""
        for file_data in parsed_data.get("files", []):
            source_file = file_data.get("path", "")

            # Top-level types
            self._load_classes(file_data.get("classes", []), module_name, source_file)
            self._load_enums(file_data.get("enums", []), module_name, source_file)
            self._load_typedefs(file_data.get("typedefs", []), module_name, source_file)

            # Namespace-scoped types
            for ns in file_data.get("namespaces", []):
                self._load_namespace(ns, module_name, source_file)

        # Invalidate caches
        self._abstract_classes = None
        self._v8_gc_types = None

    def _load_namespace(self, ns_data: dict, module: str, source_file: str):
        """Recursively load types from a namespace."""
        self._load_classes(ns_data.get("classes", []), module, source_file)
        self._load_enums(ns_data.get("enums", []), module, source_file)
        self._load_typedefs(ns_data.get("typedefs", []), module, source_file)

        for sub_ns in ns_data.get("namespaces", []):
            self._load_namespace(sub_ns, module, source_file)

    def _load_classes(self, classes: list, module: str, source_file: str):
        """Load class/struct type information."""
        for cls in classes:
            name = cls.get("name", "")
            if not name:
                continue

            qualified = cls.get("qualified_name", name)
            kind = "struct" if cls.get("is_struct", False) else "class"

            # Collect base classes
            bases = []
            for b in cls.get("base_classes", []):
                base_qname = b.get("qualified_name", b.get("name", ""))
                if base_qname:
                    bases.append(base_qname)

            # Collect virtual methods
            virtual_methods = []
            pure_virtual = []
            for m in cls.get("methods", []):
                if m.get("is_pure_virtual"):
                    pure_virtual.append(m.get("name", ""))
                elif m.get("is_virtual"):
                    virtual_methods.append(m.get("name", ""))

            # Template params
            template_params = [p.get("name", "") for p in cls.get("template_params", [])]

            info = TypeInfo(
                name=name,
                qualified_name=qualified,
                kind=kind,
                module=module,
                source_file=source_file,
                source_line=cls.get("line", 0),
                base_classes=bases,
                is_abstract=bool(pure_virtual),
                is_template=bool(template_params),
                template_params=template_params,
                virtual_methods=virtual_methods,
                pure_virtual_methods=pure_virtual,
            )

            self._register(info)

            # Record inheritance
            for base in bases:
                self._parents.setdefault(qualified, []).append(base)
                self._children.setdefault(base, []).append(qualified)

            # Nested types
            for nested in cls.get("nested_classes", []):
                self._load_classes([nested], module, source_file)
            for nested_enum in cls.get("nested_enums", []):
                self._load_enums([nested_enum], module, source_file)

    def _load_enums(self, enums: list, module: str, source_file: str):
        """Load enum type information."""
        for e in enums:
            name = e.get("name", "")
            if not name:
                continue

            info = TypeInfo(
                name=name,
                qualified_name=e.get("qualified_name", name),
                kind="enum",
                module=module,
                source_file=source_file,
                source_line=e.get("line", 0),
            )
            self._register(info)

    def _load_typedefs(self, typedefs: list, module: str, source_file: str):
        """Load typedef/using alias information."""
        for td in typedefs:
            name = td.get("name", "")
            if not name:
                continue

            template_params = [p.get("name", "") for p in td.get("template_params", [])]

            info = TypeInfo(
                name=name,
                qualified_name=td.get("qualified_name", name),
                kind="typedef",
                module=module,
                source_file=source_file,
                source_line=td.get("line", 0),
                typedef_target=td.get("underlying_type", "") or td.get("canonical_type", ""),
                is_template=bool(template_params),
                template_params=template_params,
            )
            self._register(info)

    def _register(self, info: TypeInfo):
        """Register a type in the registry."""
        self._types[info.qualified_name] = info

        # Index by short name
        self._name_index.setdefault(info.name, []).append(info.qualified_name)

        # Index by module
        self._module_index.setdefault(info.module, []).append(info.qualified_name)

    # ─── Lookup ──────────────────────────────────────────────────────────

    def get(self, qualified_name: str) -> Optional[TypeInfo]:
        """Look up a type by its fully qualified name."""
        return self._types.get(qualified_name)

    def find(self, name: str) -> List[TypeInfo]:
        """Find types by short name (may return multiple if ambiguous)."""
        qualified_names = self._name_index.get(name, [])
        return [self._types[qn] for qn in qualified_names if qn in self._types]

    def find_in_module(self, name: str, module: str) -> Optional[TypeInfo]:
        """Find a type by name within a specific module."""
        for qn in self._name_index.get(name, []):
            info = self._types.get(qn)
            if info and info.module == module:
                return info
        return None

    def get_module_types(self, module: str) -> List[TypeInfo]:
        """Get all types in a specific module."""
        return [
            self._types[qn]
            for qn in self._module_index.get(module, [])
            if qn in self._types
        ]

    # ─── Inheritance Analysis ────────────────────────────────────────────

    def get_bases(self, qualified_name: str) -> List[str]:
        """Get direct base classes of a type."""
        return self._parents.get(qualified_name, [])

    def get_all_bases(self, qualified_name: str) -> Set[str]:
        """Get all ancestor classes (transitive closure)."""
        result = set()
        stack = list(self._parents.get(qualified_name, []))
        while stack:
            base = stack.pop()
            if base not in result:
                result.add(base)
                stack.extend(self._parents.get(base, []))
        return result

    def get_derived(self, qualified_name: str) -> List[str]:
        """Get direct subclasses of a type."""
        return self._children.get(qualified_name, [])

    def get_all_derived(self, qualified_name: str) -> Set[str]:
        """Get all descendant classes (transitive closure)."""
        result = set()
        stack = list(self._children.get(qualified_name, []))
        while stack:
            child = stack.pop()
            if child not in result:
                result.add(child)
                stack.extend(self._children.get(child, []))
        return result

    def is_abstract(self, qualified_name: str) -> bool:
        """Check if a type is abstract (has pure virtual methods)."""
        info = self._types.get(qualified_name)
        return info.is_abstract if info else False

    def get_abstract_classes(self) -> Set[str]:
        """Get all abstract classes in the registry."""
        if self._abstract_classes is None:
            self._abstract_classes = {
                qn for qn, info in self._types.items()
                if info.is_abstract
            }
        return self._abstract_classes

    def get_virtual_methods(self, qualified_name: str) -> List[str]:
        """Get all virtual methods of a type (including inherited)."""
        info = self._types.get(qualified_name)
        if not info:
            return []

        methods = set(info.virtual_methods + info.pure_virtual_methods)

        # Add inherited virtual methods
        for base in self.get_all_bases(qualified_name):
            base_info = self._types.get(base)
            if base_info:
                methods.update(base_info.virtual_methods)
                methods.update(base_info.pure_virtual_methods)

        return sorted(methods)

    # ─── Type Resolution ─────────────────────────────────────────────────

    def resolve_typedef(self, qualified_name: str) -> str:
        """Resolve a typedef chain to the final concrete type."""
        visited = set()
        current = qualified_name

        while current in self._types:
            if current in visited:
                break  # Cycle detected
            visited.add(current)

            info = self._types[current]
            if info.kind != "typedef" or not info.typedef_target:
                return current
            current = info.typedef_target

        return current

    def get_module(self, qualified_name: str) -> Optional[str]:
        """Get which module a type belongs to."""
        info = self._types.get(qualified_name)
        return info.module if info else None

    # ─── V8-Specific Analysis ────────────────────────────────────────────

    def get_v8_gc_types(self) -> Set[str]:
        """Get all types that are V8 garbage-collected (derive from HeapObject)."""
        if self._v8_gc_types is None:
            # Find HeapObject and all its descendants
            heap_obj_variants = [
                qn for qn, info in self._types.items()
                if info.name == "HeapObject"
            ]
            self._v8_gc_types = set()
            for ho in heap_obj_variants:
                self._v8_gc_types.add(ho)
                self._v8_gc_types.update(self.get_all_derived(ho))
        return self._v8_gc_types

    def is_v8_gc_type(self, qualified_name: str) -> bool:
        """Check if a type is a V8 garbage-collected heap object."""
        return qualified_name in self.get_v8_gc_types()

    # ─── Statistics ──────────────────────────────────────────────────────

    def type_count(self) -> int:
        return len(self._types)

    def module_count(self) -> int:
        return len(self._module_index)

    def stats(self) -> Dict[str, Any]:
        """Return summary statistics."""
        kind_counts: Dict[str, int] = {}
        for info in self._types.values():
            kind_counts[info.kind] = kind_counts.get(info.kind, 0) + 1

        return {
            "total_types": len(self._types),
            "modules": len(self._module_index),
            "by_kind": kind_counts,
            "abstract_classes": len(self.get_abstract_classes()),
            "template_types": sum(1 for i in self._types.values() if i.is_template),
            "inheritance_edges": sum(len(v) for v in self._parents.values()),
            "gc_types": len(self.get_v8_gc_types()),
        }

    # ─── Serialization ───────────────────────────────────────────────────

    def save(self, path: Path):
        """Save the registry to a JSON file."""
        data = {
            "types": {qn: info.to_dict() for qn, info in self._types.items()},
            "inheritance": {
                "parents": dict(self._parents),
                "children": dict(self._children),
            },
            "stats": self.stats(),
        }
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w") as f:
            json.dump(data, f, indent=2)

    def load(self, path: Path):
        """Load the registry from a JSON file."""
        with open(path) as f:
            data = json.load(f)

        for qn, info_dict in data.get("types", {}).items():
            info = TypeInfo(
                name=info_dict["name"],
                qualified_name=info_dict["qualified_name"],
                kind=info_dict["kind"],
                module=info_dict["module"],
                source_file=info_dict.get("source_file", ""),
                source_line=info_dict.get("source_line", 0),
                base_classes=info_dict.get("base_classes", []),
                is_abstract=info_dict.get("is_abstract", False),
                is_template=info_dict.get("is_template", False),
                template_params=info_dict.get("template_params", []),
                virtual_methods=info_dict.get("virtual_methods", []),
                pure_virtual_methods=info_dict.get("pure_virtual_methods", []),
                nested_types=info_dict.get("nested_types", []),
                typedef_target=info_dict.get("typedef_target", ""),
            )
            self._register(info)

        inh = data.get("inheritance", {})
        self._parents = {k: list(v) for k, v in inh.get("parents", {}).items()}
        self._children = {k: list(v) for k, v in inh.get("children", {}).items()}

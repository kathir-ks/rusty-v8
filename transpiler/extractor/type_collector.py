"""Global type inventory collector for the V8 codebase.

Traverses V8 header files using the ASTParser and builds a comprehensive
inventory of all types: classes, enums, typedefs, template specializations,
and inheritance relationships. This inventory is used by downstream stages
(IR construction, dependency analysis, Rust code generation) to resolve
type references across module boundaries.
"""

from __future__ import annotations

import json
import logging
import re
import sys
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple

sys.path.insert(0, str(Path(__file__).parent.parent))
from config import TranspilerConfig
from extractor.ast_parser import (
    ASTParser,
    ExtractedClass,
    ExtractedEnum,
    ExtractedFile,
    ExtractedNamespace,
    ExtractedTypedef,
)

logger = logging.getLogger(__name__)

# Regex patterns for detecting V8-specific handle/tagged usage in type strings.
_HANDLE_PATTERN = re.compile(r"\bHandle\s*<\s*([^>]+)\s*>")
_MAYBE_HANDLE_PATTERN = re.compile(r"\bMaybeHandle\s*<\s*([^>]+)\s*>")
_DIRECT_HANDLE_PATTERN = re.compile(r"\bDirectHandle\s*<\s*([^>]+)\s*>")
_TAGGED_PATTERN = re.compile(r"\bTagged\s*<\s*([^>]+)\s*>")
_TAGGED_MEMBER_PATTERN = re.compile(r"\bTaggedMember\s*<\s*([^>]+)\s*>")

# Well-known V8 GC root types.
_HEAP_OBJECT_BASES = frozenset({
    "HeapObject",
    "v8::internal::HeapObject",
    "TorqueGeneratedHeapObject",
})


# ---- Data classes -----------------------------------------------------------

@dataclass
class ClassInfo:
    """Information about a collected class or struct."""

    name: str
    qualified_name: str
    module: str = ""
    file: str = ""
    line: int = 0
    is_struct: bool = False
    is_abstract: bool = False
    is_forward_decl: bool = False
    base_classes: List[str] = field(default_factory=list)
    fields: List[Dict[str, Any]] = field(default_factory=list)
    methods: List[Dict[str, Any]] = field(default_factory=list)
    template_params: List[str] = field(default_factory=list)
    nested_class_names: List[str] = field(default_factory=list)
    nested_enum_names: List[str] = field(default_factory=list)
    typedef_names: List[str] = field(default_factory=list)
    is_heap_object: bool = False

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class EnumInfo:
    """Information about a collected enum."""

    name: str
    qualified_name: str
    module: str = ""
    file: str = ""
    line: int = 0
    is_scoped: bool = False
    underlying_type: str = ""
    variants: List[Dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class TypedefInfo:
    """Information about a collected typedef or using alias."""

    name: str
    qualified_name: str
    module: str = ""
    file: str = ""
    line: int = 0
    underlying_type: str = ""
    canonical_type: str = ""
    template_params: List[str] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class TypeInventory:
    """Complete type inventory for one or more V8 modules.

    Every type is keyed by its C++ qualified name (e.g.
    ``v8::internal::JSObject``).  The inventory is fully JSON-serializable
    for caching and inspection.
    """

    classes: Dict[str, ClassInfo] = field(default_factory=dict)
    enums: Dict[str, EnumInfo] = field(default_factory=dict)
    typedefs: Dict[str, TypedefInfo] = field(default_factory=dict)
    inheritance_tree: Dict[str, List[str]] = field(default_factory=dict)
    abstract_classes: Set[str] = field(default_factory=set)
    template_classes: Dict[str, List[str]] = field(default_factory=dict)
    v8_handle_types: Set[str] = field(default_factory=set)
    module_map: Dict[str, str] = field(default_factory=dict)

    # ------------------------------------------------------------------
    # Merge
    # ------------------------------------------------------------------

    def merge(self, other: TypeInventory) -> None:
        """Merge *other* into this inventory (mutates self)."""
        self.classes.update(other.classes)
        self.enums.update(other.enums)
        self.typedefs.update(other.typedefs)
        for cls_name, bases in other.inheritance_tree.items():
            self.inheritance_tree[cls_name] = bases
        self.abstract_classes.update(other.abstract_classes)
        for cls_name, params in other.template_classes.items():
            self.template_classes[cls_name] = params
        self.v8_handle_types.update(other.v8_handle_types)
        self.module_map.update(other.module_map)

    # ------------------------------------------------------------------
    # Queries
    # ------------------------------------------------------------------

    def is_subclass_of(self, qualified_name: str, ancestor: str) -> bool:
        """Return True if *qualified_name* transitively inherits from *ancestor*."""
        visited: Set[str] = set()
        stack = [qualified_name]
        while stack:
            current = stack.pop()
            if current in visited:
                continue
            visited.add(current)
            bases = self.inheritance_tree.get(current, [])
            for base in bases:
                if base == ancestor:
                    return True
                stack.append(base)
        return False

    def get_heap_object_classes(self) -> List[str]:
        """Return qualified names of all classes deriving from HeapObject."""
        return [
            qname for qname in self.classes
            if self.classes[qname].is_heap_object
        ]

    # ------------------------------------------------------------------
    # Serialisation
    # ------------------------------------------------------------------

    def to_dict(self) -> Dict[str, Any]:
        return {
            "classes": {k: v.to_dict() for k, v in self.classes.items()},
            "enums": {k: v.to_dict() for k, v in self.enums.items()},
            "typedefs": {k: v.to_dict() for k, v in self.typedefs.items()},
            "inheritance_tree": self.inheritance_tree,
            "abstract_classes": sorted(self.abstract_classes),
            "template_classes": self.template_classes,
            "v8_handle_types": sorted(self.v8_handle_types),
            "module_map": self.module_map,
        }

    def to_json(self, indent: int = 2) -> str:
        return json.dumps(self.to_dict(), indent=indent)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> TypeInventory:
        """Reconstruct a TypeInventory from its dict representation."""
        inv = cls()
        for qname, cinfo in data.get("classes", {}).items():
            inv.classes[qname] = ClassInfo(**cinfo)
        for qname, einfo in data.get("enums", {}).items():
            inv.enums[qname] = EnumInfo(**einfo)
        for qname, tinfo in data.get("typedefs", {}).items():
            inv.typedefs[qname] = TypedefInfo(**tinfo)
        inv.inheritance_tree = data.get("inheritance_tree", {})
        inv.abstract_classes = set(data.get("abstract_classes", []))
        inv.template_classes = data.get("template_classes", {})
        inv.v8_handle_types = set(data.get("v8_handle_types", []))
        inv.module_map = data.get("module_map", {})
        return inv


# ---- Collector --------------------------------------------------------------

class TypeCollector:
    """Traverses V8 header files and builds a global :class:`TypeInventory`.

    Usage::

        config = TranspilerConfig()
        parser = ASTParser(config)
        collector = TypeCollector(config, parser)
        inventory = collector.collect_all()
    """

    def __init__(self, config: TranspilerConfig, parser: ASTParser) -> None:
        self.config = config
        self.parser = parser

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def collect_all(self) -> TypeInventory:
        """Parse every module's headers and return a merged inventory."""
        inventory = TypeInventory()

        # Collect from src/ modules
        for module in self.config.get_v8_modules():
            logger.info("Collecting types from module: %s", module)
            module_inv = self.collect_module(module)
            inventory.merge(module_inv)

        # Collect from include/ (public API headers)
        include_inv = self._collect_include_dir()
        inventory.merge(include_inv)

        # Post-processing: propagate heap-object flags through the tree
        self._propagate_heap_object_flags(inventory)

        logger.info(
            "Type collection complete: %d classes, %d enums, %d typedefs",
            len(inventory.classes),
            len(inventory.enums),
            len(inventory.typedefs),
        )

        return inventory

    def collect_module(self, module: str) -> TypeInventory:
        """Parse one module's header files and return its inventory."""
        header_files = self._get_module_headers(module)
        if not header_files:
            logger.debug("No header files found for module: %s", module)
            return TypeInventory()

        logger.debug(
            "Parsing %d headers for module %s", len(header_files), module
        )

        parsed = self.parser.parse_module(module, header_files)
        return self._inventory_from_parsed(parsed, module)

    # ------------------------------------------------------------------
    # Internal: file discovery
    # ------------------------------------------------------------------

    def _get_module_headers(self, module: str) -> List[Path]:
        """Return header files for *module*, respecting skip patterns."""
        module_dir = self.config.v8_src_dir / module
        if not module_dir.exists():
            return []

        headers: List[Path] = []
        for ext in self.config.header_extensions:
            for fpath in module_dir.rglob(f"*{ext}"):
                if self.config.should_skip(fpath):
                    continue
                if fpath.stat().st_size > self.config.max_file_size:
                    continue
                headers.append(fpath)

        return sorted(headers)

    def _collect_include_dir(self) -> TypeInventory:
        """Collect types from the public ``include/`` API headers."""
        files = self.config.get_include_files()
        if not files:
            return TypeInventory()

        parsed = self.parser.parse_module("include", files)
        return self._inventory_from_parsed(parsed, "include")

    # ------------------------------------------------------------------
    # Internal: build inventory from parsed data
    # ------------------------------------------------------------------

    def _inventory_from_parsed(
        self, parsed: dict, module: str
    ) -> TypeInventory:
        """Convert ASTParser ``parse_module`` output into a TypeInventory."""
        inventory = TypeInventory()

        for file_data in parsed.get("files", []):
            # Skip files that only contain parse errors.
            if "namespaces" not in file_data and "classes" not in file_data:
                continue

            file_path = file_data.get("path", "")

            # Process top-level declarations.
            self._collect_classes(
                file_data.get("classes", []),
                module, file_path, inventory,
            )
            self._collect_enums(
                file_data.get("enums", []),
                module, file_path, inventory,
            )
            self._collect_typedefs(
                file_data.get("typedefs", []),
                module, file_path, inventory,
            )

            # Process namespace-scoped declarations.
            for ns_data in file_data.get("namespaces", []):
                self._collect_from_namespace(
                    ns_data, module, file_path, inventory,
                )

            # Scan all type strings for Handle<T> / Tagged<T> usage.
            self._scan_handle_types(file_data, inventory)

        return inventory

    def _collect_from_namespace(
        self,
        ns_data: dict,
        module: str,
        file_path: str,
        inventory: TypeInventory,
    ) -> None:
        """Recursively collect types from a namespace dict."""
        self._collect_classes(
            ns_data.get("classes", []), module, file_path, inventory,
        )
        self._collect_enums(
            ns_data.get("enums", []), module, file_path, inventory,
        )
        self._collect_typedefs(
            ns_data.get("typedefs", []), module, file_path, inventory,
        )
        for sub_ns in ns_data.get("namespaces", []):
            self._collect_from_namespace(sub_ns, module, file_path, inventory)

    # ------------------------------------------------------------------
    # Class collection
    # ------------------------------------------------------------------

    def _collect_classes(
        self,
        classes: List[dict],
        module: str,
        file_path: str,
        inventory: TypeInventory,
    ) -> None:
        for cls_data in classes:
            qname = cls_data.get("qualified_name", cls_data.get("name", ""))
            if not qname:
                continue

            # Skip forward declarations -- they carry no useful info.
            if cls_data.get("is_forward_decl", False):
                continue

            # Build base class list (qualified names).
            base_names = [
                b.get("qualified_name") or b.get("name", "")
                for b in cls_data.get("base_classes", [])
            ]

            # Template parameters (just names).
            tmpl_params = [
                tp.get("name", "")
                for tp in cls_data.get("template_params", [])
            ]

            # Determine if abstract (has pure virtual methods).
            is_abstract = cls_data.get("is_abstract", False)
            if not is_abstract:
                is_abstract = any(
                    m.get("is_pure_virtual", False)
                    for m in cls_data.get("methods", [])
                )

            # Check whether this class directly inherits from HeapObject.
            is_heap_object = bool(
                _HEAP_OBJECT_BASES.intersection(base_names)
            )

            info = ClassInfo(
                name=cls_data.get("name", ""),
                qualified_name=qname,
                module=module,
                file=file_path,
                line=cls_data.get("line", 0),
                is_struct=cls_data.get("is_struct", False),
                is_abstract=is_abstract,
                is_forward_decl=False,
                base_classes=base_names,
                fields=[
                    {
                        "name": f.get("name", ""),
                        "type": f.get("type", ""),
                        "access": f.get("access", "public"),
                        "is_static": f.get("is_static", False),
                    }
                    for f in cls_data.get("fields", [])
                ],
                methods=[
                    {
                        "name": m.get("name", ""),
                        "return_type": m.get("return_type", ""),
                        "is_virtual": m.get("is_virtual", False),
                        "is_pure_virtual": m.get("is_pure_virtual", False),
                        "is_static": m.get("is_static", False),
                        "access": m.get("access", "public"),
                    }
                    for m in cls_data.get("methods", [])
                ],
                template_params=tmpl_params,
                nested_class_names=[
                    nc.get("qualified_name", nc.get("name", ""))
                    for nc in cls_data.get("nested_classes", [])
                ],
                nested_enum_names=[
                    ne.get("qualified_name", ne.get("name", ""))
                    for ne in cls_data.get("nested_enums", [])
                ],
                typedef_names=[
                    td.get("qualified_name", td.get("name", ""))
                    for td in cls_data.get("typedefs", [])
                ],
                is_heap_object=is_heap_object,
            )

            inventory.classes[qname] = info
            inventory.module_map[qname] = module

            # Inheritance tree.
            if base_names:
                inventory.inheritance_tree[qname] = base_names

            # Abstract set.
            if is_abstract:
                inventory.abstract_classes.add(qname)

            # Template classes.
            if tmpl_params:
                inventory.template_classes[qname] = tmpl_params

            # Recursively collect nested classes.
            self._collect_classes(
                cls_data.get("nested_classes", []),
                module, file_path, inventory,
            )

            # Collect nested enums.
            self._collect_enums(
                cls_data.get("nested_enums", []),
                module, file_path, inventory,
            )

    # ------------------------------------------------------------------
    # Enum collection
    # ------------------------------------------------------------------

    def _collect_enums(
        self,
        enums: List[dict],
        module: str,
        file_path: str,
        inventory: TypeInventory,
    ) -> None:
        for enum_data in enums:
            qname = enum_data.get(
                "qualified_name", enum_data.get("name", "")
            )
            if not qname:
                continue

            info = EnumInfo(
                name=enum_data.get("name", ""),
                qualified_name=qname,
                module=module,
                file=file_path,
                line=enum_data.get("line", 0),
                is_scoped=enum_data.get("is_scoped", False),
                underlying_type=enum_data.get("underlying_type", ""),
                variants=[
                    {
                        "name": v.get("name", ""),
                        "value": v.get("value"),
                        "value_str": v.get("value_str", ""),
                    }
                    for v in enum_data.get("variants", [])
                ],
            )

            inventory.enums[qname] = info
            inventory.module_map[qname] = module

    # ------------------------------------------------------------------
    # Typedef collection
    # ------------------------------------------------------------------

    def _collect_typedefs(
        self,
        typedefs: List[dict],
        module: str,
        file_path: str,
        inventory: TypeInventory,
    ) -> None:
        for td_data in typedefs:
            qname = td_data.get("qualified_name", td_data.get("name", ""))
            if not qname:
                continue

            tmpl_params = [
                tp.get("name", "")
                for tp in td_data.get("template_params", [])
            ]

            info = TypedefInfo(
                name=td_data.get("name", ""),
                qualified_name=qname,
                module=module,
                file=file_path,
                line=td_data.get("line", 0),
                underlying_type=td_data.get("underlying_type", ""),
                canonical_type=td_data.get("canonical_type", ""),
                template_params=tmpl_params,
            )

            inventory.typedefs[qname] = info
            inventory.module_map[qname] = module

    # ------------------------------------------------------------------
    # V8-specific Handle<T> / Tagged<T> scanning
    # ------------------------------------------------------------------

    def _scan_handle_types(
        self, file_data: dict, inventory: TypeInventory
    ) -> None:
        """Scan type strings in *file_data* for Handle<T> and Tagged<T> usage."""
        type_strings = self._gather_type_strings(file_data)
        for ts in type_strings:
            for pattern in (
                _HANDLE_PATTERN,
                _MAYBE_HANDLE_PATTERN,
                _DIRECT_HANDLE_PATTERN,
                _TAGGED_PATTERN,
                _TAGGED_MEMBER_PATTERN,
            ):
                for match in pattern.finditer(ts):
                    inner = match.group(1).strip()
                    if inner:
                        inventory.v8_handle_types.add(inner)

    def _gather_type_strings(self, data: dict) -> List[str]:
        """Recursively collect all type-spelling strings from parsed data."""
        strings: List[str] = []

        # Fields
        for f in data.get("fields", []):
            strings.append(f.get("type", ""))
            strings.append(f.get("canonical_type", ""))

        # Methods / functions
        for m in data.get("methods", []) + data.get("functions", []):
            strings.append(m.get("return_type", ""))
            strings.append(m.get("canonical_return_type", ""))
            for p in m.get("params", []):
                strings.append(p.get("type", ""))
                strings.append(p.get("canonical_type", ""))

        # Typedefs
        for td in data.get("typedefs", []):
            strings.append(td.get("underlying_type", ""))
            strings.append(td.get("canonical_type", ""))

        # Base classes
        for bc in data.get("base_classes", []):
            strings.append(bc.get("name", ""))
            strings.append(bc.get("qualified_name", ""))

        # Variables
        for v in data.get("variables", []):
            strings.append(v.get("type", ""))
            strings.append(v.get("canonical_type", ""))

        # Classes (recurse)
        for cls in data.get("classes", []) + data.get("nested_classes", []):
            strings.extend(self._gather_type_strings(cls))

        # Namespaces (recurse)
        for ns in data.get("namespaces", []):
            strings.extend(self._gather_type_strings(ns))

        return [s for s in strings if s]

    # ------------------------------------------------------------------
    # Post-processing
    # ------------------------------------------------------------------

    def _propagate_heap_object_flags(self, inventory: TypeInventory) -> None:
        """Mark every class that transitively inherits from HeapObject."""
        # Seed: classes already flagged by direct base check.
        heap_classes: Set[str] = {
            qname
            for qname, info in inventory.classes.items()
            if info.is_heap_object
        }
        # Also include the canonical HeapObject names themselves.
        for hb in _HEAP_OBJECT_BASES:
            if hb in inventory.classes:
                heap_classes.add(hb)
                inventory.classes[hb].is_heap_object = True

        # Build child -> parent and parent -> children maps.
        children_of: Dict[str, List[str]] = {}
        for cls_name, bases in inventory.inheritance_tree.items():
            for base in bases:
                children_of.setdefault(base, []).append(cls_name)

        # BFS from known heap objects downward.
        queue = list(heap_classes)
        visited: Set[str] = set(queue)
        while queue:
            current = queue.pop(0)
            for child in children_of.get(current, []):
                if child not in visited:
                    visited.add(child)
                    if child in inventory.classes:
                        inventory.classes[child].is_heap_object = True
                    queue.append(child)

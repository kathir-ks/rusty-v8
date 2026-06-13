"""Include-directive resolver and module dependency graph builder.

Analyses ``#include`` directives across the V8 codebase to determine
inter-file and inter-module dependencies.  The output is a
:class:`ModuleDependencyGraph` that contains a topological ordering of
modules (when acyclic) and explicit cycle reporting.

The resolver understands V8's include conventions:

- ``#include "src/objects/tagged.h"``  -> module **objects**
- ``#include "src/base/bits.h"``       -> module **base**
- ``#include "include/v8.h"``          -> module **include** (public API)
"""

from __future__ import annotations

import json
import logging
import os
import re
import sys
from collections import defaultdict, deque
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Deque, Dict, List, Optional, Set, Tuple

sys.path.insert(0, str(Path(__file__).parent.parent))
from config import TranspilerConfig

logger = logging.getLogger(__name__)

# Pre-compiled pattern for V8 src-rooted includes.
_SRC_INCLUDE_RE = re.compile(r"^src/([^/]+)/")
_INCLUDE_DIR_RE = re.compile(r"^include/")


# ---- Data classes -----------------------------------------------------------

@dataclass
class ModuleInfo:
    """Metadata about a single V8 module."""

    name: str
    crate_name: str = ""
    file_count: int = 0
    depends_on: Set[str] = field(default_factory=set)

    def to_dict(self) -> Dict[str, Any]:
        d = asdict(self)
        d["depends_on"] = sorted(self.depends_on)
        return d

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> ModuleInfo:
        info = cls(
            name=data["name"],
            crate_name=data.get("crate_name", ""),
            file_count=data.get("file_count", 0),
            depends_on=set(data.get("depends_on", [])),
        )
        return info


@dataclass
class ModuleDependencies:
    """Per-module dependency information (file-level detail)."""

    module: str
    file_includes: Dict[str, List[str]] = field(default_factory=dict)
    # file_path -> list of included paths
    internal_deps: Set[str] = field(default_factory=set)
    # files within the same module
    external_module_deps: Set[str] = field(default_factory=set)
    # other V8 modules this module depends on
    unresolved_includes: List[str] = field(default_factory=list)
    # includes that could not be mapped to any module

    def to_dict(self) -> Dict[str, Any]:
        return {
            "module": self.module,
            "file_includes": self.file_includes,
            "internal_deps": sorted(self.internal_deps),
            "external_module_deps": sorted(self.external_module_deps),
            "unresolved_includes": self.unresolved_includes,
        }


@dataclass
class ModuleDependencyGraph:
    """Complete inter-module dependency graph for the V8 codebase.

    ``topological_order`` is empty when cycles prevent a full ordering.
    In that case ``cycles`` contains the detected strongly-connected
    components of size > 1.
    """

    modules: Dict[str, ModuleInfo] = field(default_factory=dict)
    edges: List[Tuple[str, str]] = field(default_factory=list)
    topological_order: List[str] = field(default_factory=list)
    cycles: List[List[str]] = field(default_factory=list)

    # ------------------------------------------------------------------
    # Serialisation
    # ------------------------------------------------------------------

    def to_dict(self) -> Dict[str, Any]:
        return {
            "modules": {k: v.to_dict() for k, v in self.modules.items()},
            "edges": self.edges,
            "topological_order": self.topological_order,
            "cycles": self.cycles,
        }

    def to_json(self, indent: int = 2) -> str:
        return json.dumps(self.to_dict(), indent=indent)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> ModuleDependencyGraph:
        graph = cls()
        for name, minfo in data.get("modules", {}).items():
            graph.modules[name] = ModuleInfo.from_dict(minfo)
        graph.edges = [tuple(e) for e in data.get("edges", [])]
        graph.topological_order = data.get("topological_order", [])
        graph.cycles = data.get("cycles", [])
        return graph


# ---- Resolver ---------------------------------------------------------------

class IncludeResolver:
    """Resolves ``#include`` directives to module dependencies.

    Usage::

        config = TranspilerConfig()
        resolver = IncludeResolver(config)
        graph = resolver.resolve_all()
        print(graph.topological_order)
    """

    def __init__(self, config: TranspilerConfig) -> None:
        self.config = config
        # Cache: include path string -> resolved module name (or None).
        self._resolve_cache: Dict[str, Optional[str]] = {}

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def resolve_all(self) -> ModuleDependencyGraph:
        """Analyse every module and the public include/ directory.

        Returns a :class:`ModuleDependencyGraph` with topological ordering
        and cycle information.
        """
        graph = ModuleDependencyGraph()

        all_modules = self.config.get_v8_modules()

        # Add the "include" pseudo-module for public API headers.
        if self.config.v8_include_dir.exists():
            all_modules = list(all_modules) + ["include"]

        # First pass: register every module and count its files.
        for module in all_modules:
            info = ModuleInfo(
                name=module,
                crate_name=self.config.module_to_crate_name(module),
            )
            if module == "include":
                info.file_count = len(self.config.get_include_files())
            else:
                info.file_count = len(self.config.get_module_files(module))
            graph.modules[module] = info

        # Second pass: resolve includes for each module.
        for module in all_modules:
            deps = self._resolve_module_includes(module)
            if module in graph.modules:
                graph.modules[module].depends_on = deps.external_module_deps

        # Build edge list (deduplicated, no self-edges).
        edge_set: Set[Tuple[str, str]] = set()
        for module, info in graph.modules.items():
            for dep in info.depends_on:
                if dep != module and dep in graph.modules:
                    edge_set.add((module, dep))
        graph.edges = sorted(edge_set)

        # Detect cycles via Tarjan's SCC algorithm.
        graph.cycles = self._detect_cycles(graph)

        # Compute topological ordering via Kahn's algorithm.
        graph.topological_order = self._topological_sort(graph)

        logger.info(
            "Dependency graph: %d modules, %d edges, %d cycles",
            len(graph.modules),
            len(graph.edges),
            len(graph.cycles),
        )

        return graph

    def resolve_module(
        self, module: str, parsed_data: dict
    ) -> ModuleDependencies:
        """Resolve includes for a single module given its parsed data.

        *parsed_data* is the dict returned by ``ASTParser.parse_module``.
        """
        deps = ModuleDependencies(module=module)

        for file_data in parsed_data.get("files", []):
            file_path = file_data.get("path", "")
            includes = file_data.get("includes", [])
            if not includes:
                continue

            deps.file_includes[file_path] = includes

            for inc in includes:
                resolved_module = self._resolve_include_to_module(inc)
                if resolved_module is None:
                    deps.unresolved_includes.append(inc)
                elif resolved_module == module:
                    deps.internal_deps.add(inc)
                else:
                    deps.external_module_deps.add(resolved_module)

        return deps

    # ------------------------------------------------------------------
    # Internal: include resolution
    # ------------------------------------------------------------------

    def _resolve_module_includes(self, module: str) -> ModuleDependencies:
        """Scan header and source files for *module* and resolve includes."""
        deps = ModuleDependencies(module=module)

        if module == "include":
            files = self.config.get_include_files()
        else:
            files = self.config.get_module_files(module)

        for fpath in files:
            includes = self._extract_includes_from_file(fpath)
            if not includes:
                continue

            fpath_str = str(fpath)
            deps.file_includes[fpath_str] = includes

            for inc in includes:
                resolved = self._resolve_include_to_module(inc)
                if resolved is None:
                    deps.unresolved_includes.append(inc)
                elif resolved == module:
                    deps.internal_deps.add(inc)
                else:
                    deps.external_module_deps.add(resolved)

        return deps

    def _extract_includes_from_file(self, file_path: Path) -> List[str]:
        """Read a file and extract ``#include "..."`` paths.

        We use a simple regex scan rather than Clang here because we only
        need the textual include paths, not semantic resolution.  This is
        much faster and avoids parsing the entire translation unit.
        """
        includes: List[str] = []
        try:
            with open(file_path, "r", errors="replace") as fh:
                for line in fh:
                    line = line.strip()
                    if not line.startswith("#include"):
                        continue
                    # Match #include "path" (V8 uses quoted includes for
                    # internal headers, not angle-bracket system includes).
                    m = re.match(r'#include\s+"([^"]+)"', line)
                    if m:
                        includes.append(m.group(1))
        except OSError as exc:
            logger.debug("Could not read %s: %s", file_path, exc)
        return includes

    def _resolve_include_to_module(self, include_path: str) -> Optional[str]:
        """Map a quoted include path to its owning V8 module name.

        Returns ``None`` for system headers or includes that cannot be
        mapped to a known module.

        Examples::

            "src/objects/tagged.h"  -> "objects"
            "src/base/bits.h"       -> "base"
            "include/v8.h"          -> "include"
            "v8-internal.h"         -> "include"
        """
        if include_path in self._resolve_cache:
            return self._resolve_cache[include_path]

        result: Optional[str] = None

        # Pattern 1: src/<module>/...
        m = _SRC_INCLUDE_RE.match(include_path)
        if m:
            result = m.group(1)
        else:
            # Pattern 2: include/... (public API headers)
            if _INCLUDE_DIR_RE.match(include_path):
                result = "include"
            else:
                # Pattern 3: bare filename like "v8.h", "v8-internal.h"
                # These live in include/ in the V8 repo.
                resolved_path = self.config.v8_include_dir / include_path
                if resolved_path.exists():
                    result = "include"
                else:
                    # Pattern 4: try relative to codebase root
                    resolved_path = self.config.codebase_root / include_path
                    if resolved_path.exists():
                        # Re-check if it matches src/<module>/...
                        rel = str(
                            resolved_path.relative_to(self.config.codebase_root)
                        )
                        m2 = _SRC_INCLUDE_RE.match(rel)
                        if m2:
                            result = m2.group(1)

        self._resolve_cache[include_path] = result
        return result

    # ------------------------------------------------------------------
    # Cycle detection: Tarjan's SCC
    # ------------------------------------------------------------------

    def _detect_cycles(
        self, graph: ModuleDependencyGraph
    ) -> List[List[str]]:
        """Return strongly-connected components of size > 1 (cycles)."""
        # Build adjacency list.
        adj: Dict[str, List[str]] = {m: [] for m in graph.modules}
        for src, dst in graph.edges:
            adj.setdefault(src, []).append(dst)

        index_counter = [0]
        stack: List[str] = []
        on_stack: Set[str] = set()
        indices: Dict[str, int] = {}
        lowlinks: Dict[str, int] = {}
        sccs: List[List[str]] = []

        def strongconnect(v: str) -> None:
            indices[v] = index_counter[0]
            lowlinks[v] = index_counter[0]
            index_counter[0] += 1
            stack.append(v)
            on_stack.add(v)

            for w in adj.get(v, []):
                if w not in indices:
                    strongconnect(w)
                    lowlinks[v] = min(lowlinks[v], lowlinks[w])
                elif w in on_stack:
                    lowlinks[v] = min(lowlinks[v], indices[w])

            # Root of an SCC.
            if lowlinks[v] == indices[v]:
                scc: List[str] = []
                while True:
                    w = stack.pop()
                    on_stack.discard(w)
                    scc.append(w)
                    if w == v:
                        break
                if len(scc) > 1:
                    sccs.append(sorted(scc))

        for node in sorted(graph.modules.keys()):
            if node not in indices:
                strongconnect(node)

        return sccs

    # ------------------------------------------------------------------
    # Topological sort: Kahn's algorithm
    # ------------------------------------------------------------------

    def _topological_sort(
        self, graph: ModuleDependencyGraph
    ) -> List[str]:
        """Compute a topological ordering using Kahn's algorithm.

        Returns a partial ordering if cycles exist (nodes in cycles are
        omitted from the result).
        """
        # In-degree map.
        in_degree: Dict[str, int] = {m: 0 for m in graph.modules}
        adj: Dict[str, List[str]] = {m: [] for m in graph.modules}

        for src, dst in graph.edges:
            adj.setdefault(src, []).append(dst)
            in_degree[dst] = in_degree.get(dst, 0) + 1

        # Seed the queue with zero-in-degree nodes.
        queue: Deque[str] = deque(
            sorted(m for m, d in in_degree.items() if d == 0)
        )

        order: List[str] = []
        while queue:
            node = queue.popleft()
            order.append(node)
            for neighbour in sorted(adj.get(node, [])):
                in_degree[neighbour] -= 1
                if in_degree[neighbour] == 0:
                    queue.append(neighbour)

        # If not all modules are in the order, cycles prevented it.
        if len(order) < len(graph.modules):
            remaining = sorted(
                set(graph.modules.keys()) - set(order)
            )
            logger.warning(
                "Topological sort incomplete: %d modules in cycles: %s",
                len(remaining),
                ", ".join(remaining[:10]),
            )

        # The order computed by Kahn's gives "dependencies first":
        # a module with no deps appears first. This is the natural
        # build order (leaves first).
        return order

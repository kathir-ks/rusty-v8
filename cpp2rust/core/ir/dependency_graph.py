"""Module-level dependency graph for determining conversion order.

Builds a directed graph of module dependencies from #include information,
detects cycles, and computes topological ordering.
"""

from __future__ import annotations

import json
from collections import defaultdict, deque
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple


@dataclass
class ModuleInfo:
    """Information about a single V8 module."""
    name: str
    crate_name: str = ""
    file_count: int = 0
    depends_on: Set[str] = field(default_factory=set)
    depended_by: Set[str] = field(default_factory=set)

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "crate_name": self.crate_name,
            "file_count": self.file_count,
            "depends_on": sorted(self.depends_on),
            "depended_by": sorted(self.depended_by),
        }


class DependencyGraph:
    """Module-level dependency graph."""

    def __init__(self):
        self._modules: Dict[str, ModuleInfo] = {}
        self._edges: Set[Tuple[str, str]] = set()  # (from_module, to_module)
        self._cached_order: Optional[List[str]] = None
        self._cached_cycles: Optional[List[List[str]]] = None

    def add_module(self, module_name: str, parsed_data: dict):
        """Add a module and its dependencies from parsed AST data."""
        files = parsed_data.get("files", [])

        if module_name not in self._modules:
            self._modules[module_name] = ModuleInfo(
                name=module_name,
                file_count=len(files),
            )
        else:
            self._modules[module_name].file_count = len(files)

        # Extract dependencies from includes
        for file_data in files:
            includes = file_data.get("includes", [])
            for inc in includes:
                dep_module = self._include_to_module(inc)
                if dep_module and dep_module != module_name:
                    self._add_dependency(module_name, dep_module)

        # Invalidate caches
        self._cached_order = None
        self._cached_cycles = None

    def _add_dependency(self, from_module: str, to_module: str):
        """Add a dependency edge."""
        if to_module not in self._modules:
            self._modules[to_module] = ModuleInfo(name=to_module)

        self._modules[from_module].depends_on.add(to_module)
        self._modules[to_module].depended_by.add(from_module)
        self._edges.add((from_module, to_module))

    def _include_to_module(self, include_path: str) -> Optional[str]:
        """Determine which module an included file belongs to.

        V8 include patterns:
        - "src/base/bits.h" → module "base"
        - "src/objects/tagged.h" → module "objects"
        - "include/v8.h" → module "include" (API)
        - "src/compiler/node.h" → module "compiler"
        """
        path = include_path.strip().strip('"').strip("'")

        # Handle src/ prefix
        if path.startswith("src/"):
            parts = path[4:].split("/")
            if parts:
                return parts[0]

        # Handle include/ prefix
        if path.startswith("include/"):
            return "include"

        # Handle relative includes within a module
        parts = path.split("/")
        if len(parts) >= 2:
            return parts[0]

        return None

    # ─── Analysis ────────────────────────────────────────────────────────

    def topological_order(self) -> List[str]:
        """Compute topological ordering of modules (dependencies first).

        Uses Kahn's algorithm. If cycles exist, modules in cycles are
        placed based on their dependency count (fewer deps first).
        """
        if self._cached_order is not None:
            return self._cached_order

        # Build in-degree map
        in_degree: Dict[str, int] = {m: 0 for m in self._modules}
        for from_m, to_m in self._edges:
            if to_m in in_degree:
                in_degree[from_m] = in_degree.get(from_m, 0)  # ensure exists

        # Recalculate proper in-degree
        in_degree = {m: 0 for m in self._modules}
        for from_m, to_m in self._edges:
            if from_m in in_degree and to_m in in_degree:
                in_degree[from_m] = in_degree.get(from_m, 0)

        # Actually, in_degree counts how many modules depend on each module
        # For topological sort, in_degree[m] = number of dependencies of m that haven't been processed
        in_degree = {m: 0 for m in self._modules}
        adj: Dict[str, List[str]] = defaultdict(list)

        for from_m, to_m in self._edges:
            if from_m in self._modules and to_m in self._modules:
                adj[to_m].append(from_m)  # to_m must come before from_m
                in_degree[from_m] += 1

        # Kahn's algorithm
        queue = deque([m for m, d in in_degree.items() if d == 0])
        order = []

        while queue:
            node = queue.popleft()
            order.append(node)

            for neighbor in adj.get(node, []):
                in_degree[neighbor] -= 1
                if in_degree[neighbor] == 0:
                    queue.append(neighbor)

        # Handle remaining nodes (in cycles)
        remaining = [m for m in self._modules if m not in order]
        if remaining:
            # Sort by number of dependencies (fewer deps first)
            remaining.sort(key=lambda m: len(self._modules[m].depends_on))
            order.extend(remaining)

        self._cached_order = order
        return order

    def detect_cycles(self) -> List[List[str]]:
        """Detect all cycles in the dependency graph using DFS."""
        if self._cached_cycles is not None:
            return self._cached_cycles

        cycles = []
        visited = set()
        rec_stack = set()
        path = []

        def dfs(node: str):
            visited.add(node)
            rec_stack.add(node)
            path.append(node)

            for dep in self._modules.get(node, ModuleInfo(name="")).depends_on:
                if dep not in self._modules:
                    continue
                if dep not in visited:
                    dfs(dep)
                elif dep in rec_stack:
                    # Found a cycle
                    cycle_start = path.index(dep)
                    cycle = path[cycle_start:] + [dep]
                    cycles.append(cycle)

            path.pop()
            rec_stack.discard(node)

        for module in self._modules:
            if module not in visited:
                dfs(module)

        self._cached_cycles = cycles
        return cycles

    def get_dependencies(self, module: str) -> Set[str]:
        """Get direct dependencies of a module."""
        info = self._modules.get(module)
        return info.depends_on if info else set()

    def get_all_dependencies(self, module: str) -> Set[str]:
        """Get all transitive dependencies of a module."""
        result = set()
        stack = list(self.get_dependencies(module))
        while stack:
            dep = stack.pop()
            if dep not in result:
                result.add(dep)
                stack.extend(self.get_dependencies(dep))
        return result

    def get_dependents(self, module: str) -> Set[str]:
        """Get modules that directly depend on this module."""
        info = self._modules.get(module)
        return info.depended_by if info else set()

    def get_leaf_modules(self) -> List[str]:
        """Get modules with no dependencies (good starting points)."""
        return sorted([
            m for m, info in self._modules.items()
            if not info.depends_on
        ])

    # ─── Statistics ──────────────────────────────────────────────────────

    def module_count(self) -> int:
        return len(self._modules)

    def edge_count(self) -> int:
        return len(self._edges)

    def stats(self) -> Dict[str, Any]:
        return {
            "modules": self.module_count(),
            "edges": self.edge_count(),
            "cycles": len(self.detect_cycles()),
            "leaf_modules": self.get_leaf_modules(),
            "conversion_order": self.topological_order(),
        }

    # ─── Serialization ───────────────────────────────────────────────────

    def save(self, path: Path):
        """Save the dependency graph to JSON."""
        data = {
            "modules": {name: info.to_dict() for name, info in self._modules.items()},
            "edges": [list(e) for e in sorted(self._edges)],
            "topological_order": self.topological_order(),
            "cycles": self.detect_cycles(),
            "stats": self.stats(),
        }
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w") as f:
            json.dump(data, f, indent=2)

    def load(self, path: Path):
        """Load the dependency graph from JSON."""
        with open(path) as f:
            data = json.load(f)

        for name, info_dict in data.get("modules", {}).items():
            self._modules[name] = ModuleInfo(
                name=name,
                crate_name=info_dict.get("crate_name", ""),
                file_count=info_dict.get("file_count", 0),
                depends_on=set(info_dict.get("depends_on", [])),
                depended_by=set(info_dict.get("depended_by", [])),
            )

        self._edges = {tuple(e) for e in data.get("edges", [])}

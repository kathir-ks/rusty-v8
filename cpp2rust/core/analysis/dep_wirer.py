"""Dependency wirer for cpp2rust.

Builds a crate dependency graph from the #include relationships between
extracted C++ files.  The graph drives Cargo.toml generation so that
cross-crate references compile without E0425 errors.
"""

from __future__ import annotations

import logging
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Optional, Set

logger = logging.getLogger(__name__)


class DepGraph:
    """Simple directed graph of crate dependencies."""

    def __init__(self) -> None:
        self._deps: Dict[str, Set[str]] = defaultdict(set)

    def add_dep(self, from_crate: str, to_crate: str) -> None:
        if from_crate != to_crate:
            self._deps[from_crate].add(to_crate)

    def get_deps(self, crate_name: str) -> List[str]:
        return sorted(self._deps.get(crate_name, set()))

    def all_crates(self) -> List[str]:
        crates: Set[str] = set()
        for k, vs in self._deps.items():
            crates.add(k)
            crates.update(vs)
        return sorted(crates)


class DepWirer:
    """Builds a DepGraph from extracted file information.

    For each extracted file, inspects include_paths to determine which
    other crates are referenced, then adds edges to the graph.
    """

    def __init__(self, plugin=None) -> None:
        self._plugin = plugin

    def build_graph(self, extracted_files) -> DepGraph:
        """Build a dependency graph from a list of extracted file objects."""
        graph = DepGraph()

        for ef in extracted_files:
            src_crate = self._crate_for(ef.path)
            if src_crate is None:
                continue

            for inc_path in getattr(ef, "include_paths", []):
                dst_crate = self._crate_for(inc_path)
                if dst_crate and dst_crate != src_crate:
                    graph.add_dep(src_crate, dst_crate)

        crate_count = len(graph.all_crates())
        logger.info("DepWirer: %d crates, graph built", crate_count)
        return graph

    def _crate_for(self, file_path: str) -> Optional[str]:
        if self._plugin is not None:
            crate = self._plugin.crate_for_file(file_path)
            if crate:
                return crate
        p = Path(file_path)
        for part in reversed(p.parts):
            if part not in ("src", "include", ".", ".."):
                return part.replace("_", "-")
        return None

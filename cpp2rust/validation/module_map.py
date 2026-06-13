"""Authoritative V8 module -> layer mapping (v8-layer-taxonomy capability).

Reads ``v8-layers/module_map.yaml``. Every targeted source path resolves to
exactly one layer or to the explicit sentinel ``UNASSIGNED`` — never a silent
default. Resolution is by longest directory-prefix match, so the more specific
prefix wins (e.g. ``src/parsing/scanner`` over ``src/parsing``).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional

import yaml

from . import LAYERS_ROOT

UNASSIGNED = "UNASSIGNED"


@dataclass(frozen=True)
class Mapping:
    prefix: str
    layer: str


@dataclass(frozen=True)
class ModuleMap:
    mappings: List[Mapping]

    def layer_for(self, source_path: str) -> str:
        """Return the owning layer id, or UNASSIGNED if no prefix matches.

        Normalizes to a path relative to a ``codebase/`` root if present so both
        absolute compile_commands paths and repo-relative paths resolve.
        """
        norm = _normalize(source_path)
        best: Optional[Mapping] = None
        for m in self.mappings:
            if norm.startswith(m.prefix) and (
                best is None or len(m.prefix) > len(best.prefix)
            ):
                best = m
        return best.layer if best else UNASSIGNED

    def is_mapped(self, source_path: str) -> bool:
        return self.layer_for(source_path) != UNASSIGNED


def _normalize(source_path: str) -> str:
    p = source_path.replace("\\", "/")
    if "codebase/" in p:
        p = p.split("codebase/", 1)[1]
    return p.lstrip("./")


def default_module_map_path() -> Path:
    return LAYERS_ROOT / "module_map.yaml"


def load_module_map(path: Optional[Path] = None) -> ModuleMap:
    path = path or default_module_map_path()
    data = yaml.safe_load(path.read_text()) or {}
    mappings = [
        Mapping(prefix=_normalize(e["prefix"]), layer=e["layer"])
        for e in (data.get("mappings") or [])
    ]
    return ModuleMap(mappings=mappings)

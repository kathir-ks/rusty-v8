"""Canonical V8 validation-layer taxonomy.

Loads and validates ``v8-layers/taxonomy.yaml`` against the
``v8-layer-taxonomy`` capability:

  - between 5 and 10 layers
  - unique kebab-case ids
  - unique, contiguous order indices (no gaps, no duplicates)
  - pipeline coverage includes scanner, parser, ast, and a bytecode stage
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional

import yaml

from . import LAYERS_ROOT

_KEBAB = re.compile(r"^[a-z][a-z0-9]*(-[a-z0-9]+)*$")

# Stages the pipeline must cover (substring match against layer ids), so the
# taxonomy can never silently drop a core compilation stage.
_REQUIRED_STAGES = ("scanner", "parser", "ast", "bytecode")


class TaxonomyError(ValueError):
    """Raised when the taxonomy violates the v8-layer-taxonomy contract."""


@dataclass(frozen=True)
class Layer:
    id: str
    order: int
    description: str


@dataclass(frozen=True)
class Taxonomy:
    layers: List[Layer]
    v8_revision: Optional[str] = None

    def ordered(self) -> List[Layer]:
        return sorted(self.layers, key=lambda layer: layer.order)

    def ids(self) -> List[str]:
        return [layer.id for layer in self.ordered()]

    def get(self, layer_id: str) -> Optional[Layer]:
        for layer in self.layers:
            if layer.id == layer_id:
                return layer
        return None


def default_taxonomy_path() -> Path:
    return LAYERS_ROOT / "taxonomy.yaml"


def load_taxonomy(path: Optional[Path] = None) -> Taxonomy:
    """Load and fully validate the taxonomy. Raises TaxonomyError on violation."""
    path = path or default_taxonomy_path()
    if not path.exists():
        raise TaxonomyError(f"taxonomy file not found: {path}")

    data = yaml.safe_load(path.read_text()) or {}
    raw_layers = data.get("layers")
    if not isinstance(raw_layers, list) or not raw_layers:
        raise TaxonomyError("taxonomy must declare a non-empty 'layers' list")

    layers: List[Layer] = []
    for i, entry in enumerate(raw_layers):
        if not isinstance(entry, dict):
            raise TaxonomyError(f"layer #{i} must be a mapping")
        lid = entry.get("id")
        order = entry.get("order")
        desc = (entry.get("description") or "").strip()
        if not lid or not _KEBAB.match(str(lid)):
            raise TaxonomyError(f"layer #{i} id {lid!r} is not kebab-case")
        if not isinstance(order, int):
            raise TaxonomyError(f"layer {lid!r} order must be an integer")
        if not desc:
            raise TaxonomyError(f"layer {lid!r} must have a description")
        layers.append(Layer(id=str(lid), order=order, description=desc))

    _validate_invariants(layers)
    return Taxonomy(layers=layers, v8_revision=data.get("v8_revision"))


def _validate_invariants(layers: List[Layer]) -> None:
    if not (5 <= len(layers) <= 10):
        raise TaxonomyError(
            f"taxonomy must declare between 5 and 10 layers, got {len(layers)}"
        )

    ids = [layer.id for layer in layers]
    if len(set(ids)) != len(ids):
        raise TaxonomyError(f"duplicate layer ids: {ids}")

    orders = [layer.order for layer in layers]
    if len(set(orders)) != len(orders):
        raise TaxonomyError(f"duplicate order indices: {orders}")
    expected = list(range(1, len(layers) + 1))
    if sorted(orders) != expected:
        raise TaxonomyError(
            f"order indices must be contiguous {expected}, got {sorted(orders)}"
        )

    joined = " ".join(ids)
    missing = [stage for stage in _REQUIRED_STAGES if stage not in joined]
    if missing:
        raise TaxonomyError(
            f"pipeline omits required stage(s): {missing} (layer ids: {ids})"
        )

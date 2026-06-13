"""Fixture provenance tracking (golden-fixture-corpus capability).

Every fixture records whether it was lifted from a real V8 test/example or
authored for this suite. Provenance lives in a sibling ``<name>.provenance.yaml``;
when absent, the fixture is treated as ``authored`` with no source — but the
loader can flag missing provenance so coverage is auditable.

    source: v8 | authored
    ref: cctest/test-parsing.cc:1234   # required when source == v8
    note: optional free text
    v8_revision: <pinned rev>          # optional per-fixture override
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional

import yaml

SOURCE_V8 = "v8"
SOURCE_AUTHORED = "authored"


class ProvenanceError(ValueError):
    pass


@dataclass(frozen=True)
class Provenance:
    fixture: str
    source: str
    ref: Optional[str] = None
    note: Optional[str] = None
    v8_revision: Optional[str] = None
    explicit: bool = True   # False when defaulted (no sidecar file present)

    @property
    def is_v8_derived(self) -> bool:
        return self.source == SOURCE_V8


def provenance_path(fixture_in: Path) -> Path:
    # foo.in -> foo.provenance.yaml
    return fixture_in.with_suffix(".provenance.yaml")


def load_provenance(fixture_in: Path) -> Provenance:
    """Load provenance for a `<name>.in` fixture, defaulting to authored."""
    p = provenance_path(fixture_in)
    if not p.exists():
        return Provenance(fixture=fixture_in.stem, source=SOURCE_AUTHORED, explicit=False)
    data = yaml.safe_load(p.read_text()) or {}
    source = data.get("source", SOURCE_AUTHORED)
    if source not in (SOURCE_V8, SOURCE_AUTHORED):
        raise ProvenanceError(f"{p}: source must be '{SOURCE_V8}' or '{SOURCE_AUTHORED}'")
    if source == SOURCE_V8 and not data.get("ref"):
        raise ProvenanceError(f"{p}: V8-derived fixture must cite a 'ref'")
    return Provenance(
        fixture=fixture_in.stem,
        source=source,
        ref=data.get("ref"),
        note=data.get("note"),
        v8_revision=data.get("v8_revision"),
        explicit=True,
    )


def layer_provenance(fixtures_dir: Path) -> List[Provenance]:
    return [load_provenance(fx) for fx in sorted(fixtures_dir.glob("*.in"))]

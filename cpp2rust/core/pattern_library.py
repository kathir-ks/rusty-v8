"""Pattern library for cpp2rust.

A PatternLibrary holds declarative mappings from C++ idioms (macros, function
calls, type expressions) to idiomatic Rust equivalents.  Patterns are loaded
from YAML files and can be queried at extraction and emission time.

Pattern schema (YAML):
    - cpp: "DCHECK($expr)"          # C++ call / macro pattern
      rust: "debug_assert!($expr)"  # Rust replacement
      kind: macro                   # macro | call | type | stmt
"""

from __future__ import annotations

import logging
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional

logger = logging.getLogger(__name__)


@dataclass
class PatternEntry:
    cpp: str
    rust: str
    kind: str = "call"  # macro | call | type | stmt


class PatternLibrary:
    """Registry of C++ → Rust pattern substitutions."""

    def __init__(self) -> None:
        self._by_kind: Dict[str, List[PatternEntry]] = {}
        self._by_cpp: Dict[str, PatternEntry] = {}

    # ── Loading ────────────────────────────────────────────────────────────

    @classmethod
    def load_builtin(cls) -> "PatternLibrary":
        """Load patterns from the built-in patterns directory."""
        lib = cls()
        builtin_dir = Path(__file__).parent.parent / "patterns"
        if builtin_dir.exists():
            for yaml_file in sorted(builtin_dir.glob("*.yaml")):
                lib._load_yaml(yaml_file)
        return lib

    def _load_yaml(self, path: Path) -> None:
        try:
            import yaml  # type: ignore
        except ImportError:
            logger.warning("PyYAML not installed; skipping pattern file %s", path)
            return
        try:
            with open(path) as f:
                data = yaml.safe_load(f) or []
            for entry in data:
                if not isinstance(entry, dict):
                    continue
                cpp = entry.get("cpp", "").strip()
                rust = entry.get("rust", "").strip()
                kind = entry.get("kind", "call").strip()
                if cpp and rust:
                    self.add(PatternEntry(cpp=cpp, rust=rust, kind=kind))
        except Exception as exc:
            logger.warning("Failed to load pattern file %s: %s", path, exc)

    # ── Mutation ───────────────────────────────────────────────────────────

    def add(self, entry: PatternEntry) -> None:
        self._by_cpp[entry.cpp] = entry
        self._by_kind.setdefault(entry.kind, []).append(entry)

    # ── Lookup ─────────────────────────────────────────────────────────────

    def lookup(self, cpp: str) -> Optional[PatternEntry]:
        """Exact-match lookup by C++ pattern string."""
        return self._by_cpp.get(cpp)

    def get_by_kind(self, kind: str) -> List[PatternEntry]:
        return list(self._by_kind.get(kind, []))

    def __len__(self) -> int:
        return len(self._by_cpp)

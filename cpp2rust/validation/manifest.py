"""Layer manifest schema, loader, and validator.

A layer is described entirely by a declarative ``manifest.yaml`` (layer-test-harness
capability). The harness reads manifests; nothing is inferred at runtime. The loader
REFUSES any layer missing its source set, inputs, or expected outputs.

Manifest schema (v8-layers/<layer>/manifest.yaml):

    layer: scanner                 # must match a taxonomy layer id
    compile_commands: ../../cpp2rust/bigint_compile_commands.json  # or 'source_set'
    source_set:                    # explicit file list (alternative to compile_commands)
      - codebase/src/parsing/scanner.cc
    module: bigint                 # optional --module filter passed to transpile
    output_format: token-stream    # documented expected-output format for this layer
    oracle:                        # optional differential oracle
      kind: v8-dump                # v8-dump | original-cpp | none
      command: ["d8", "--print-bytecode"]   # how to produce reference output
      normalize: [strip-addresses, strip-ids]
    groups:                        # grouping rules (v8-layer-taxonomy: grouped units)
      - name: parser-core
        members: [parser.cc, preparser.cc]
        independently_testable: false
    fixtures_dir: fixtures         # relative dir holding <name>.in / <name>.expected
    reuse_real_v8: true            # whether real V8 inputs/outputs were reused
    reuse_note: "lifted from cctest/test-parsing"
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional

import yaml


class ManifestError(ValueError):
    """Raised when a layer manifest is missing required elements or malformed."""


@dataclass(frozen=True)
class OracleSpec:
    kind: str = "none"                       # v8-dump | original-cpp | none
    command: List[str] = field(default_factory=list)
    normalize: List[str] = field(default_factory=list)

    @property
    def available(self) -> bool:
        return self.kind != "none" and bool(self.command)


@dataclass(frozen=True)
class GroupSpec:
    name: str
    members: List[str]
    independently_testable: bool = False


@dataclass(frozen=True)
class LayerManifest:
    layer: str
    manifest_path: Path
    output_format: str
    fixtures_dir: Path
    compile_commands: Optional[Path] = None
    source_set: List[str] = field(default_factory=list)
    module: Optional[str] = None
    oracle: OracleSpec = field(default_factory=OracleSpec)
    groups: List[GroupSpec] = field(default_factory=list)
    reuse_real_v8: Optional[bool] = None
    reuse_note: Optional[str] = None

    @property
    def has_source_set(self) -> bool:
        return bool(self.compile_commands) or bool(self.source_set)


def load_manifest(path: Path, repo_root: Optional[Path] = None) -> LayerManifest:
    """Load and validate a single layer manifest. Raises ManifestError if the
    manifest omits its source set, inputs (fixtures_dir), or expected outputs
    (output_format)."""
    repo_root = repo_root or path.resolve().parents[2]
    if not path.exists():
        raise ManifestError(f"manifest not found: {path}")
    data = yaml.safe_load(path.read_text()) or {}

    layer = data.get("layer")
    if not layer:
        raise ManifestError(f"{path}: manifest must declare 'layer'")

    def _resolve(rel: Optional[str]) -> Optional[Path]:
        if not rel:
            return None
        p = Path(rel)
        return p if p.is_absolute() else (path.parent / p).resolve()

    compile_commands = _resolve(data.get("compile_commands"))
    source_set = list(data.get("source_set") or [])
    output_format = (data.get("output_format") or "").strip()
    fixtures_dir = _resolve(data.get("fixtures_dir") or "fixtures")

    oracle_raw = data.get("oracle") or {}
    oracle = OracleSpec(
        kind=oracle_raw.get("kind", "none"),
        command=list(oracle_raw.get("command") or []),
        normalize=list(oracle_raw.get("normalize") or []),
    )

    groups = [
        GroupSpec(
            name=g["name"],
            members=list(g.get("members") or []),
            independently_testable=bool(g.get("independently_testable", False)),
        )
        for g in (data.get("groups") or [])
    ]

    manifest = LayerManifest(
        layer=str(layer),
        manifest_path=path,
        output_format=output_format,
        fixtures_dir=fixtures_dir,
        compile_commands=compile_commands,
        source_set=source_set,
        module=data.get("module"),
        oracle=oracle,
        groups=groups,
        reuse_real_v8=data.get("reuse_real_v8"),
        reuse_note=data.get("reuse_note"),
    )
    _validate_manifest(manifest)
    return manifest


def _validate_manifest(m: LayerManifest) -> None:
    """Enforce the three mandatory elements: source set, inputs, expected outputs."""
    problems: List[str] = []
    if not m.has_source_set:
        problems.append("missing source set (compile_commands or source_set)")
    if not m.output_format:
        problems.append("missing expected-output declaration (output_format)")
    if not m.fixtures_dir or not m.fixtures_dir.exists():
        problems.append(f"missing inputs (fixtures_dir does not exist: {m.fixtures_dir})")
    if problems:
        raise ManifestError(f"layer '{m.layer}' misconfigured: " + "; ".join(problems))


def discover_manifests(layers_root: Path) -> Dict[str, Path]:
    """Return {layer_id: manifest_path} for every <layer>/manifest.yaml found."""
    found: Dict[str, Path] = {}
    for manifest_path in sorted(layers_root.glob("*/manifest.yaml")):
        found[manifest_path.parent.name] = manifest_path
    return found

"""Suite orchestrator: run every declared layer and produce the scorecard.

Wires taxonomy + manifests + harness + report into one entry point used by the
`cpp2rust validate` CLI command.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Dict, List, Optional

from . import LAYERS_ROOT
from .harness import LayerResult, run_layer
from .manifest import discover_manifests, load_manifest
from .report import (
    Scorecard,
    build_scorecard,
    diff_baseline,
    load_baseline,
    render_human,
    save_baseline,
)
from .taxonomy import load_taxonomy


def default_baseline_path() -> Path:
    return LAYERS_ROOT / "baseline.json"


def run_suite(
    layers: Optional[List[str]] = None,
    build: bool = True,
    output_root: Optional[Path] = None,
    layers_root: Path = LAYERS_ROOT,
) -> Dict[str, LayerResult]:
    """Run the requested layers (default: all that have a manifest)."""
    manifests = discover_manifests(layers_root)
    if layers:
        manifests = {k: v for k, v in manifests.items() if k in layers}

    results: Dict[str, LayerResult] = {}
    for layer_id, manifest_path in manifests.items():
        manifest = load_manifest(manifest_path)
        results[layer_id] = run_layer(manifest, output_root=output_root, build=build)
    return results


def run_and_report(
    layers: Optional[List[str]] = None,
    build: bool = True,
    output_root: Optional[Path] = None,
    baseline_path: Optional[Path] = None,
    update_baseline: bool = False,
    as_json: bool = False,
    layers_root: Path = LAYERS_ROOT,
) -> int:
    """Run the suite, print the scorecard, and return an exit code.

    Exit code is non-zero only on genuine regressions or hard layer FAILs — a
    not-run/built-only/unimplemented layer is informational, not a failure.
    """
    taxonomy = load_taxonomy(layers_root / "taxonomy.yaml")
    results = run_suite(layers=layers, build=build, output_root=output_root,
                        layers_root=layers_root)
    scorecard = build_scorecard(taxonomy, results)

    bpath = baseline_path or default_baseline_path()
    baseline = load_baseline(bpath)
    regressions = diff_baseline(scorecard, baseline)

    if as_json:
        payload = scorecard.to_dict()
        payload["regressions"] = regressions.regressions
        payload["improvements"] = regressions.improvements
        payload["outstanding"] = regressions.outstanding
        print(json.dumps(payload, indent=2))
    else:
        print(render_human(scorecard, regressions))

    if update_baseline:
        save_baseline(scorecard, bpath)
        if not as_json:
            print(f"\nbaseline updated -> {bpath}")

    hard_fail = any(r.status == "fail" for r in scorecard.rows)
    return 1 if (regressions.has_regressions or hard_fail) else 0

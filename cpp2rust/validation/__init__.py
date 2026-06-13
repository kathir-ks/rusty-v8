"""cpp2rust layered validation harness.

Slices V8 into a canonical set of validation layers and runs a uniform
transpile -> build -> run -> judge cycle against per-layer golden fixtures,
with optional differential (oracle) comparison and an aggregate scorecard.

See openspec/specs/{v8-layer-taxonomy,layer-test-harness,golden-fixture-corpus,
differential-validation,validation-reporting} for the spec-level contract.
"""

from __future__ import annotations

from pathlib import Path

# Repo-root-relative location of the layer corpus tree.
LAYERS_ROOT = Path(__file__).resolve().parents[2] / "v8-layers"

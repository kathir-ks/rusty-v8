"""Validation scorecard and regression reporting (validation-reporting capability).

Produces a scorecard with one row per taxonomy layer — INCLUDING layers that have
no manifest or were not run, marked explicitly as ``unimplemented`` — an aggregate
maturity metric, and a regression diff against a stored baseline that distinguishes
genuine regressions (was passing, now failing) from never-passing layers.
"""

from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Dict, List, Optional

from .harness import LayerResult, LAYER_PASS
from .taxonomy import Taxonomy

# Row statuses surfaced in the scorecard.
ROW_PASS = "pass"
ROW_FAIL = "fail"
ROW_BUILT_ONLY = "built-only"
ROW_NOT_RUN = "not-run"
ROW_UNIMPLEMENTED = "unimplemented"


@dataclass
class ScoreRow:
    layer: str
    order: int
    status: str
    fixtures_run: int = 0
    fixtures_passed: int = 0
    differential_used: bool = False
    detail: str = ""


@dataclass
class Aggregate:
    layers_total: int
    layers_passing: int
    fixtures_total: int
    fixtures_passing: int

    @property
    def layer_fraction(self) -> float:
        return self.layers_passing / self.layers_total if self.layers_total else 0.0

    @property
    def fixture_fraction(self) -> float:
        return self.fixtures_passing / self.fixtures_total if self.fixtures_total else 0.0


@dataclass
class Scorecard:
    rows: List[ScoreRow]
    aggregate: Aggregate
    v8_revision: Optional[str] = None

    def to_dict(self) -> Dict:
        return {
            "v8_revision": self.v8_revision,
            "rows": [asdict(r) for r in self.rows],
            "aggregate": {
                "layers_total": self.aggregate.layers_total,
                "layers_passing": self.aggregate.layers_passing,
                "layer_fraction": round(self.aggregate.layer_fraction, 4),
                "fixtures_total": self.aggregate.fixtures_total,
                "fixtures_passing": self.aggregate.fixtures_passing,
                "fixture_fraction": round(self.aggregate.fixture_fraction, 4),
            },
        }


def build_scorecard(
    taxonomy: Taxonomy,
    results: Dict[str, LayerResult],
) -> Scorecard:
    """One row per taxonomy layer; layers absent from `results` are unimplemented."""
    rows: List[ScoreRow] = []
    layers_passing = fixtures_total = fixtures_passing = 0

    for layer in taxonomy.ordered():
        res = results.get(layer.id)
        if res is None:
            rows.append(ScoreRow(layer.id, layer.order, ROW_UNIMPLEMENTED,
                                 detail="no manifest / not run"))
            continue
        status = {
            LAYER_PASS: ROW_PASS,
        }.get(res.verdict, res.verdict)  # 'fail' / 'built-only' pass through
        row = ScoreRow(
            layer=layer.id,
            order=layer.order,
            status=status,
            fixtures_run=res.fixtures_run,
            fixtures_passed=res.fixtures_passed,
            differential_used=res.differential_used,
            detail=_summarize(res),
        )
        rows.append(row)
        if res.verdict == LAYER_PASS:
            layers_passing += 1
        fixtures_total += res.fixtures_run
        fixtures_passing += res.fixtures_passed

    aggregate = Aggregate(
        layers_total=len(taxonomy.layers),
        layers_passing=layers_passing,
        fixtures_total=fixtures_total,
        fixtures_passing=fixtures_passing,
    )
    return Scorecard(rows=rows, aggregate=aggregate, v8_revision=taxonomy.v8_revision)


def _summarize(res: LayerResult) -> str:
    failed = [s.name for s in res.stages if s.status == "fail"]
    if failed:
        return "failed stage(s): " + ", ".join(failed)
    skipped = [s.name for s in res.stages if s.status == "skip"]
    return ("skipped: " + ", ".join(skipped)) if skipped else "ok"


# ── baseline & regression ─────────────────────────────────────────────────

def save_baseline(scorecard: Scorecard, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(scorecard.to_dict(), indent=2) + "\n")


def load_baseline(path: Path) -> Optional[Dict]:
    if not path.exists():
        return None
    return json.loads(path.read_text())


@dataclass
class RegressionReport:
    regressions: List[str] = field(default_factory=list)   # were passing, now not
    improvements: List[str] = field(default_factory=list)   # were not passing, now passing
    outstanding: List[str] = field(default_factory=list)    # never passed, still not passing

    @property
    def has_regressions(self) -> bool:
        return bool(self.regressions)


def diff_baseline(current: Scorecard, baseline: Optional[Dict]) -> RegressionReport:
    """Compare current scorecard to a baseline, classifying each layer.

    A regression is a layer that PASSED in the baseline but does not now — kept
    distinct from layers that have never passed (outstanding).
    """
    report = RegressionReport()
    base_status: Dict[str, str] = {}
    if baseline:
        base_status = {r["layer"]: r["status"] for r in baseline.get("rows", [])}

    for row in current.rows:
        was_pass = base_status.get(row.layer) == ROW_PASS
        now_pass = row.status == ROW_PASS
        if was_pass and not now_pass:
            report.regressions.append(row.layer)
        elif not was_pass and now_pass:
            report.improvements.append(row.layer)
        elif not now_pass:
            report.outstanding.append(row.layer)
    return report


# ── rendering ─────────────────────────────────────────────────────────────

_ICON = {
    ROW_PASS: "PASS",
    ROW_FAIL: "FAIL",
    ROW_BUILT_ONLY: "BUILT",
    ROW_NOT_RUN: "PEND",
    ROW_UNIMPLEMENTED: "----",
}


def render_human(scorecard: Scorecard, regressions: Optional[RegressionReport] = None) -> str:
    lines: List[str] = []
    rev = scorecard.v8_revision or "unpinned"
    lines.append(f"cpp2rust V8 layered validation — V8 {rev}")
    lines.append("=" * 64)
    lines.append(f"{'#':>2}  {'layer':<16} {'status':<6} {'fixtures':>9}  diff")
    lines.append("-" * 64)
    for r in scorecard.rows:
        icon = _ICON.get(r.status, r.status)
        fx = f"{r.fixtures_passed}/{r.fixtures_run}"
        diff = "yes" if r.differential_used else "no"
        lines.append(f"{r.order:>2}  {r.layer:<16} {icon:<6} {fx:>9}  {diff}")
    lines.append("-" * 64)
    agg = scorecard.aggregate
    lines.append(
        f"maturity: {agg.layers_passing}/{agg.layers_total} layers passing "
        f"({agg.layer_fraction*100:.0f}%), "
        f"{agg.fixtures_passing}/{agg.fixtures_total} fixtures passing"
    )
    if regressions is not None:
        if regressions.regressions:
            lines.append("REGRESSIONS: " + ", ".join(regressions.regressions))
        if regressions.improvements:
            lines.append("improvements: " + ", ".join(regressions.improvements))
        if regressions.outstanding:
            lines.append("outstanding (never passed): " + ", ".join(regressions.outstanding))
    return "\n".join(lines)

"""Per-layer transpile -> build -> run -> judge harness (layer-test-harness capability).

A layer PASSES only if, for every input fixture: the source set transpiles, the
emitted Rust contains no todo!() (always-compiles invariant), the workspace
builds, the layer's runner produces output for the fixture, and that output
matches the fixture's expected output (after per-layer normalization).

Build-pass-but-not-behaviorally-validated is a first-class, honestly-reported
state ("built-only"), not a harness error — see design.md D-risks.
"""

from __future__ import annotations

import difflib
import subprocess
import tempfile
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Optional

from .manifest import LayerManifest
from .normalize import normalize
from . import LAYERS_ROOT

# Stage / fixture / layer status vocabulary.
PASS = "pass"
FAIL = "fail"
SKIP = "skip"

# Overall layer verdicts.
LAYER_PASS = "pass"            # every stage + every fixture passed
LAYER_FAIL = "fail"            # a hard failure (transpile/build/todo/mismatch)
LAYER_BUILT_ONLY = "built-only"  # transpiled+built clean but no runner to behaviorally validate
LAYER_NOT_RUN = "not-run"     # could not start (e.g. no compile_commands shim yet) — pending, not failing


@dataclass
class StageResult:
    name: str
    status: str
    detail: str = ""


@dataclass
class FixtureResult:
    name: str
    status: str
    detail: str = ""
    diff: str = ""


@dataclass
class LayerResult:
    layer: str
    verdict: str
    stages: List[StageResult] = field(default_factory=list)
    fixtures: List[FixtureResult] = field(default_factory=list)
    differential_used: bool = False
    output_dir: Optional[Path] = None

    @property
    def passed(self) -> bool:
        return self.verdict == LAYER_PASS

    @property
    def fixtures_run(self) -> int:
        return sum(1 for f in self.fixtures if f.status != SKIP)

    @property
    def fixtures_passed(self) -> int:
        return sum(1 for f in self.fixtures if f.status == PASS)


def _git_root() -> Path:
    return LAYERS_ROOT.parent


def run_layer(
    manifest: LayerManifest,
    output_root: Optional[Path] = None,
    build: bool = True,
    no_cache: bool = True,
) -> LayerResult:
    """Execute the full cycle for one layer and return a structured result."""
    from cpp2rust.cli import load_plugin
    from cpp2rust.main import run_transpile, find_todo_files

    result = LayerResult(layer=manifest.layer, verdict=LAYER_FAIL)
    out_base = output_root or Path(tempfile.mkdtemp(prefix=f"v8layer-{manifest.layer}-"))
    out_dir = out_base / manifest.layer
    result.output_dir = out_dir

    # ── Stage 1: transpile ────────────────────────────────────────────────
    cc = manifest.compile_commands
    if cc is None:
        result.stages.append(StageResult("transpile", SKIP,
            "source_set-only layers need a compile_commands shim (not yet generated)"))
        result.verdict = LAYER_NOT_RUN
        return result
    try:
        plugin = load_plugin("v8")
        run_transpile(
            plugin=plugin,
            compile_commands=cc,
            module_filter=manifest.module,
            output_dir=out_dir,
            phase=1,
            no_cache=no_cache,
            jobs=4,
        )
        result.stages.append(StageResult("transpile", PASS, str(out_dir)))
    except Exception as exc:  # transpile failure fails the layer
        result.stages.append(StageResult("transpile", FAIL, f"{type(exc).__name__}: {exc}"))
        result.verdict = LAYER_FAIL
        return result

    # ── Stage 2: always-compiles invariant (no todo!()) ───────────────────
    todo_files = find_todo_files(out_dir)
    if todo_files:
        result.stages.append(StageResult("no-todo", FAIL,
            f"{len(todo_files)} file(s) contain todo!()"))
        result.verdict = LAYER_FAIL
        return result
    result.stages.append(StageResult("no-todo", PASS))

    # ── Stage 3: build ────────────────────────────────────────────────────
    if build:
        proc = subprocess.run(
            ["cargo", "build", "--manifest-path", str(out_dir / "Cargo.toml")],
            capture_output=True, text=True,
        )
        if proc.returncode != 0:
            tail = (proc.stderr or "").strip().splitlines()[-15:]
            result.stages.append(StageResult("build", FAIL, "\n".join(tail)))
            result.verdict = LAYER_FAIL
            return result
        result.stages.append(StageResult("build", PASS))
    else:
        result.stages.append(StageResult("build", SKIP, "build disabled"))

    # ── Stage 4 + 5: run + judge each fixture ─────────────────────────────
    fixtures = _discover_fixtures(manifest)
    runner = _runner_command(manifest, out_dir)

    if not fixtures:
        result.stages.append(StageResult("fixtures", SKIP, "no fixtures declared"))
        result.verdict = LAYER_BUILT_ONLY
        return result

    if runner is None:
        # Built clean but no behavioral runner wired yet — honest "built-only".
        for fx in fixtures:
            result.fixtures.append(FixtureResult(fx.stem, SKIP, "no runner declared for layer"))
        result.stages.append(StageResult("run", SKIP, "no runner; behavioral validation pending"))
        result.verdict = LAYER_BUILT_ONLY
        return result

    result.differential_used = manifest.oracle.available
    all_pass = True
    for fx in fixtures:
        fr = _judge_fixture(fx, manifest, runner)
        result.fixtures.append(fr)
        if fr.status != PASS:
            all_pass = False
    result.stages.append(StageResult("run", PASS if all_pass else FAIL))
    result.verdict = LAYER_PASS if all_pass else LAYER_FAIL
    return result


# ── fixture helpers ───────────────────────────────────────────────────────

def _discover_fixtures(manifest: LayerManifest) -> List[Path]:
    """Fixtures are <name>.in files paired with <name>.expected in fixtures_dir."""
    if not manifest.fixtures_dir.exists():
        return []
    return sorted(manifest.fixtures_dir.glob("*.in"))


def _runner_command(manifest: LayerManifest, out_dir: Path) -> Optional[List[str]]:
    """A layer is behaviorally runnable only if it declares a runner binary.

    Phase-1 transpiled V8 is a library workspace with no per-layer driver, so by
    default there is no runner and the layer reports built-only. A manifest may
    declare ``runner: [<argv>]`` (with {input} placeholder) once a driver exists.
    """
    raw = getattr(manifest, "runner", None)
    if not raw:
        return None
    return list(raw)


def _judge_fixture(fx: Path, manifest: LayerManifest, runner: List[str]) -> FixtureResult:
    from .oracle import reference_output

    # Produce the transpiled-Rust output for this input.
    cmd = [a.replace("{input}", str(fx)) for a in runner]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
    except Exception as exc:
        return FixtureResult(fx.stem, FAIL, f"runner error: {exc}")
    if proc.returncode != 0:
        return FixtureResult(fx.stem, FAIL, f"runner exit {proc.returncode}: {proc.stderr[-300:]}")
    produced = normalize(proc.stdout, manifest.oracle.normalize)

    # Expected output: differential oracle when available, else golden file.
    oracle_label = "golden"
    if manifest.oracle.available:
        oracle_res = reference_output(manifest, fx)
        if oracle_res.available and oracle_res.reference is not None:
            expected = oracle_res.reference
            oracle_label = "oracle"
        else:
            # Declared oracle could not run here -> fall back to golden, note it.
            expected_path = fx.with_suffix(".expected")
            if not expected_path.exists():
                return FixtureResult(fx.stem, FAIL,
                    f"oracle unavailable ({oracle_res.detail}) and no golden expected")
            expected = normalize(expected_path.read_text(), manifest.oracle.normalize)
            oracle_label = "golden(oracle-unavailable)"
    else:
        expected_path = fx.with_suffix(".expected")
        if not expected_path.exists():
            return FixtureResult(fx.stem, FAIL, f"missing expected output: {expected_path.name}")
        expected = normalize(expected_path.read_text(), manifest.oracle.normalize)

    if produced == expected:
        return FixtureResult(fx.stem, PASS, detail=f"matched via {oracle_label}")
    diff = "\n".join(difflib.unified_diff(
        expected.splitlines(), produced.splitlines(),
        fromfile=f"expected({oracle_label})", tofile="produced", lineterm="",
    )[:40])
    return FixtureResult(fx.stem, FAIL, f"output mismatch ({oracle_label})", diff=diff)

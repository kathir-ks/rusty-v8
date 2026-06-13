"""Differential (oracle) validation (differential-validation capability).

When a layer declares an available oracle (V8's own dump, or the original C++
compiled and run), the reference output is produced fresh from the oracle on the
same input and compared against the transpiled Rust output after the layer's
declared normalization. Any difference that survives normalization is a real
divergence and fails the check.

When no oracle is available, callers fall back to the checked-in golden
``.expected`` file and the report must state that no differential oracle was used.
"""

from __future__ import annotations

import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from .manifest import LayerManifest
from .normalize import normalize


@dataclass
class OracleResult:
    available: bool
    reference: Optional[str] = None   # normalized reference output, if produced
    detail: str = ""


def oracle_available(manifest: LayerManifest) -> bool:
    return manifest.oracle.available


def reference_output(manifest: LayerManifest, fixture_in: Path) -> OracleResult:
    """Run the layer's oracle command on `fixture_in` and return normalized output.

    The command may contain an `{input}` placeholder (substituted with the
    fixture path); otherwise the fixture is piped to stdin.
    """
    if not manifest.oracle.available:
        return OracleResult(available=False, detail="no oracle declared for layer")

    cmd = manifest.oracle.command
    uses_placeholder = any("{input}" in a for a in cmd)
    argv = [a.replace("{input}", str(fixture_in)) for a in cmd]
    stdin_data = None if uses_placeholder else fixture_in.read_text()

    try:
        proc = subprocess.run(
            argv, input=stdin_data, capture_output=True, text=True, timeout=120
        )
    except FileNotFoundError as exc:
        # Oracle binary not present in this environment -> treat as unavailable.
        return OracleResult(available=False, detail=f"oracle command unavailable: {exc}")
    except Exception as exc:
        return OracleResult(available=False, detail=f"oracle error: {exc}")

    if proc.returncode != 0:
        return OracleResult(
            available=True, detail=f"oracle exit {proc.returncode}: {proc.stderr[-300:]}"
        )
    ref = normalize(proc.stdout, manifest.oracle.normalize)
    return OracleResult(available=True, reference=ref)

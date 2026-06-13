"""Output normalization rules (differential-validation capability).

Each layer declares which named rules to apply before comparison so that
incidental nondeterminism (pointer addresses, object ids, hash ordering) does
not cause spurious failures — while any difference that *survives* normalization
is treated as a real divergence.

Rules are intentionally narrow and explicit; prefer failing loud over
normalizing broadly.
"""

from __future__ import annotations

import re
from typing import Callable, Dict, List

# 0x-prefixed hex addresses -> <addr>
_ADDR = re.compile(r"0x[0-9a-fA-F]+")
# "#123" / "@123" style object/node ids -> #<id>
_ID = re.compile(r"[#@]\d+")
# trailing whitespace on each line
_TRAILING_WS = re.compile(r"[ \t]+$", re.MULTILINE)


def _strip_addresses(text: str) -> str:
    return _ADDR.sub("<addr>", text)


def _strip_ids(text: str) -> str:
    return _ID.sub("#<id>", text)


def _strip_trailing_ws(text: str) -> str:
    return _TRAILING_WS.sub("", text)


def _collapse_blank_lines(text: str) -> str:
    return re.sub(r"\n{3,}", "\n\n", text)


def _sort_lines(text: str) -> str:
    """Order-insensitive comparison: sort non-empty lines."""
    lines = [ln for ln in text.splitlines() if ln.strip()]
    return "\n".join(sorted(lines))


_RULES: Dict[str, Callable[[str], str]] = {
    "strip-addresses": _strip_addresses,
    "strip-ids": _strip_ids,
    "strip-trailing-ws": _strip_trailing_ws,
    "collapse-blank-lines": _collapse_blank_lines,
    "sort-lines": _sort_lines,
}

# Always applied as a baseline so trivial formatting never trips comparison.
_BASELINE = ["strip-trailing-ws"]


def available_rules() -> List[str]:
    return sorted(_RULES)


def normalize(text: str, rules: List[str]) -> str:
    """Apply the baseline rules plus the named rules in order.

    Unknown rule names raise KeyError so a typo in a manifest fails loudly
    rather than silently skipping normalization.
    """
    applied = list(_BASELINE)
    for r in rules:
        if r not in _RULES:
            raise KeyError(f"unknown normalization rule: {r!r}; known: {available_rules()}")
        if r not in applied:
            applied.append(r)
    out = text
    for r in applied:
        out = _RULES[r](out)
    return out.strip("\n")

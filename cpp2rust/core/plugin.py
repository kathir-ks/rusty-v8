"""Base plugin interface for cpp2rust.

Each codebase that cpp2rust targets provides a plugin that customises
type mappings, macro translations, prelude stubs, and module layout.
Plugins subclass CppPlugin and override the methods they need.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional


class CppPlugin:
    """Base class for cpp2rust codebase plugins.

    Subclass this and override any methods to customise translation for a
    specific C++ codebase.  All methods have safe no-op defaults so plugins
    only need to override what they actually customise.
    """

    # ── Type mapping ─────────────────────────────────────────────────────

    def map_type(self, cpp_type: str) -> Optional[str]:
        """Map a C++ type spelling to a Rust type string.

        Returns None to fall through to the generic type mapper.
        Returns a Rust type string (e.g. "u64", "Handle<T>") to override.
        """
        return None

    # ── Macro mapping ─────────────────────────────────────────────────────

    def map_macro(self, name: str, args: List[str]) -> Optional[str]:
        """Map a C++ macro invocation to a Rust expression string.

        name: macro name (e.g. "DCHECK")
        args: list of argument strings as they appeared in source
        Returns None to fall through to pattern library / raw fallback.
        Returns a Rust expression string to override.
        """
        return None

    # ── Prelude ──────────────────────────────────────────────────────────

    def prelude_lines(self) -> List[str]:
        """Return Rust source lines to prepend to every crate's prelude.rs.

        Typically: type aliases, struct definitions, constants that every
        generated crate needs.
        """
        return []

    def prelude_file(self) -> Optional[Path]:
        """Return path to a .rs file whose contents form the prelude.

        If both prelude_file() and prelude_lines() return something,
        prelude_file() wins.  Return None to use prelude_lines() instead.
        """
        return None

    # ── Module layout ────────────────────────────────────────────────────

    def crate_for_file(self, file_path: str) -> Optional[str]:
        """Return the Rust crate name for a given C++ source file path.

        file_path: absolute or project-relative path
        Returns None to fall back to default naming (directory basename).
        Returns a crate name string (e.g. "v8-bigint") to assign the file.
        """
        return None

    def module_layout(self) -> List[Dict[str, str]]:
        """Return list of {pattern, crate} rules for file-to-crate mapping.

        Rules are applied in order; first match wins.  Each rule has:
          pattern: a glob pattern relative to the codebase root
          crate:   the Rust crate name to assign matching files to
        """
        return []

    def skip_patterns(self) -> List[str]:
        """Return glob patterns for files to exclude from transpilation."""
        return [
            "**/test/**",
            "**/*_unittest*",
            "**/*_test.*",
            "**/testing/**",
        ]

    # ── Pattern library extensions ───────────────────────────────────────

    def extra_patterns(self) -> List[Dict[str, Any]]:
        """Return additional pattern library entries (same format as YAML).

        Plugin patterns have higher priority than built-in patterns.
        Each entry is a dict with keys: name, cpp_pattern, rust_emit,
        needs_unsafe (bool), and optionally requires (list of use-paths).
        """
        return []

    # ── Post-processing ──────────────────────────────────────────────────

    def post_process(self, source: str) -> str:
        """Apply codebase-specific post-processing to emitted Rust source.

        Called on each emitted .rs file after the main emitter runs.
        Default: return source unchanged.
        """
        return source

    # ── Clang flags ──────────────────────────────────────────────────────

    def extra_clang_args(self) -> List[str]:
        """Return extra Clang flags to add when parsing this codebase."""
        return []

    # ── Generic template declarations ─────────────────────────────────────

    def generic_templates(self) -> Dict[str, Dict[str, Any]]:
        """Declare which C++ templates should become Rust generics (Tier 1).

        Returns a dict mapping template name → config:
          bounds: list of trait bound strings (e.g. ["Send", "Sync"])
          wrapper: bool — True if this is a single-field newtype wrapper

        Example:
          {"Handle": {"bounds": [], "wrapper": True}}
        """
        return {}

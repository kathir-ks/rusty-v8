"""Template resolver for cpp2rust.

Resolves C++ template instantiations in extracted files.  Phase 1 uses a
simple structural approach: if a generic template class/function was fully
extracted, it passes through unchanged.  Monomorphisation (tier 3) is
deferred to Phase 2.
"""

from __future__ import annotations

import logging
from typing import List

logger = logging.getLogger(__name__)


class TemplateResolver:
    """Resolves generic templates in extracted IR files.

    Current implementation: structural pass-through (tier 1).
    Templates that cannot be resolved are left as-is with their generic
    parameters intact; the emitter will produce generic Rust structs/fns.
    """

    def __init__(self, plugin=None) -> None:
        self._plugin = plugin

    def resolve(self, extracted_files) -> List:
        """Apply template resolution to all extracted files.

        Returns the same list (possibly mutated in place) after resolution.
        """
        plugin_generics: set = set()
        if self._plugin is not None:
            plugin_generics = set(self._plugin.generic_templates())

        resolved = []
        for ef in extracted_files:
            resolved.append(self._resolve_file(ef, plugin_generics))

        logger.info("TemplateResolver: %d files processed", len(resolved))
        return resolved

    def _resolve_file(self, ef, plugin_generics: set):
        """Resolve templates in a single extracted file.  Pass-through for now."""
        return ef

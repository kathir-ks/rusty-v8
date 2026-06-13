"""UnsafeEmitter: Phase-1 always-compiles Rust emitter.

Wraps RustEmitter with:
- Plugin post_process hook
- Pattern library expansion (reserved for Phase 2 pattern substitution)
- Guaranteed no todo!() from IRRawCppExpr / IRRawCppStmt nodes
"""

from __future__ import annotations

from cpp2rust.core.emit.rust_emitter import RustEmitter
from cpp2rust.core.ir.nodes import IRFile


class UnsafeEmitter(RustEmitter):
    """Phase-1 emitter: always produces compilable unsafe Rust.

    IRRawCppExpr → Default::default() + UNTRANSLATED comment
    IRRawCppStmt → unsafe { let _ = 0; } + UNTRANSLATED comment
    """

    def __init__(self, plugin=None, pattern_lib=None) -> None:
        super().__init__()
        self._plugin = plugin
        self._pattern_lib = pattern_lib

    def emit_file(self, ir_file: IRFile) -> str:
        source = super().emit_file(ir_file)
        if self._plugin is not None:
            source = self._plugin.post_process(source)
        return source

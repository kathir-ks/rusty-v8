"""Main pipeline entry point for cpp2rust."""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import List, Optional

from cpp2rust.core.plugin import CppPlugin


def run_transpile(
    plugin: CppPlugin,
    compile_commands: Path,
    module_filter: Optional[str],
    output_dir: Path,
    phase: int,
    no_cache: bool,
    jobs: int,
) -> None:
    """Main transpilation pipeline."""
    from cpp2rust.core.extractor.ast_parser import ASTParser
    from cpp2rust.core.analysis.dep_wirer import DepWirer
    from cpp2rust.core.analysis.template_resolver import TemplateResolver
    from cpp2rust.core.emit.unsafe_emitter import UnsafeEmitter
    from cpp2rust.core.emit.cargo_emitter import CargoEmitter
    from cpp2rust.core.pattern_library import PatternLibrary
    from cpp2rust.core.ir.ir_builder import extracted_to_ir_file

    print(f"[cpp2rust] phase={phase} plugin={type(plugin).__name__} output={output_dir}")

    # Load compile_commands.json
    if not compile_commands.exists():
        print(f"Error: compile_commands.json not found at {compile_commands}", file=sys.stderr)
        sys.exit(1)

    with open(compile_commands) as f:
        commands = json.load(f)

    # Apply skip patterns and module filter
    skip_pats = plugin.skip_patterns()
    files = _filter_files(commands, plugin, module_filter, skip_pats)
    print(f"[cpp2rust] {len(files)} files to process")

    # Build pattern library
    pattern_lib = PatternLibrary.load_builtin()
    for entry in plugin.extra_patterns():
        pattern_lib.add(entry)

    # Extract C++ AST
    parser = ASTParser(
        extra_clang_args=plugin.extra_clang_args(),
        cache_dir=Path(".transpiler-cache"),
        no_cache=no_cache,
    )
    extracted_files = parser.parse_files(files, pattern_lib=pattern_lib, jobs=jobs)
    print(f"[cpp2rust] extraction complete: {len(extracted_files)} files")

    # Template resolution
    resolver = TemplateResolver(plugin)
    resolved = resolver.resolve(extracted_files)

    # Dependency wiring
    wirer = DepWirer(plugin)
    dep_graph = wirer.build_graph(extracted_files)

    # Emission
    output_dir.mkdir(parents=True, exist_ok=True)
    emitter = UnsafeEmitter(plugin, pattern_lib)
    cargo = CargoEmitter(plugin, dep_graph)

    for ef in resolved:
        crate_name = plugin.crate_for_file(ef.path) or _default_crate(ef.path)
        ir_file = extracted_to_ir_file(ef, plugin=plugin)
        rust_src = emitter.emit_file(ir_file)
        _write_rust_file(ef, crate_name, rust_src, output_dir)

    cargo.emit_workspace(output_dir)
    print(f"[cpp2rust] done → {output_dir}")


def find_todo_files(output_dir: Path) -> List[str]:
    """Return the list of .rs files under output_dir containing a todo!() token.

    Reusable, non-exiting core of the no-todo!() invariant check so callers
    (CLI check command, validation harness) share one implementation.
    """
    result = subprocess.run(
        ["grep", "-r", "--include=*.rs", "-l", "todo!()"],
        cwd=output_dir,
        capture_output=True,
        text=True,
    )
    out = result.stdout.strip()
    return out.split("\n") if out else []


def run_check(output_dir: Path, no_todo: bool) -> None:
    """Check the output workspace for invariant violations."""
    errors = 0

    if no_todo:
        files = find_todo_files(output_dir)
        if files:
            print(f"FAIL: {len(files)} files contain todo!():")
            for f in files:
                print(f"  {f}")
            errors += 1
        else:
            print("PASS: no todo!() found in output")

    sys.exit(1 if errors else 0)


def _filter_files(commands, plugin, module_filter, skip_pats):
    """Return list of (file_path, clang_args) tuples from compile_commands."""
    import fnmatch
    result = []
    for entry in commands:
        fpath = entry.get("file", "")
        if not fpath:
            continue

        # Apply skip patterns
        skip = False
        for pat in skip_pats:
            if fnmatch.fnmatch(fpath, pat) or fnmatch.fnmatch(
                os.path.basename(fpath), pat
            ):
                skip = True
                break
        if skip:
            continue

        # Apply module filter
        if module_filter:
            crate = plugin.crate_for_file(fpath) or _default_crate(fpath)
            if module_filter not in crate:
                continue

        # Parse arguments — strip compiler, output flags, and the source file itself
        args = entry.get("arguments", [])
        if not args and "command" in entry:
            args = entry["command"].split()

        clang_args = _clean_clang_args(args[1:] if args else [], fpath)
        result.append((fpath, clang_args))

    return result


def _clean_clang_args(args: list, source_file: str) -> list:
    """Strip flags that break libclang parsing (output flags, source file itself)."""
    result = []
    skip_next = False
    for a in args:
        if skip_next:
            skip_next = False
            continue
        if a in ("-c", "-MD", "-MMD", "-MF", "-MT", "-MQ"):
            if a in ("-MF", "-MT", "-MQ"):
                skip_next = True
            continue
        if a in ("-o",):
            skip_next = True
            continue
        if a == source_file:
            continue
        result.append(a)
    return result


def _default_crate(file_path: str) -> str:
    """Derive a crate name from a file path (last directory component)."""
    p = Path(file_path)
    for part in reversed(p.parts):
        if part not in ("src", "include", ".", ".."):
            return part.replace("_", "-")
    return "unknown"


def _write_rust_file(ef, crate_name: str, rust_src: str, output_dir: Path) -> None:
    """Write a generated Rust file to the output workspace."""
    src_name = Path(ef.path).stem + ".rs"
    dest = output_dir / "crates" / crate_name / "src" / src_name
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text(rust_src)

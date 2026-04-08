#!/usr/bin/env python3
"""CLI entry point for the C++ to Rust transpiler."""

import argparse
import json
import sys
import time
from pathlib import Path

from config import TranspilerConfig

import re

_INCLUDE_RE = re.compile(r'#include\s+"src/([^/]+)/')


def _scan_include_deps(files, current_module, config):
    """Scan C++ files for #include "src/MODULE/..." to find inter-module deps."""
    deps = set()
    for filepath in files:
        try:
            text = filepath.read_text(encoding="utf-8", errors="ignore")
            for m in _INCLUDE_RE.finditer(text):
                dep_module = m.group(1)
                if dep_module != current_module:
                    deps.add(config.module_to_crate_name(dep_module))
        except Exception:
            pass
    return sorted(deps)


def _break_dependency_cycles(all_deps):
    """Detect and break circular dependencies between crates.

    Modifies all_deps in place by removing back-edges from cycles.
    Returns a list of detected cycles for logging.
    """
    cycles_found = []

    # Simple cycle detection: for each pair (A→B), check if B→A exists
    edges_to_remove = []
    for crate_a, deps_a in all_deps.items():
        for crate_b in deps_a:
            if crate_b in all_deps and crate_a in all_deps[crate_b]:
                # Mutual dependency — break by removing the edge from the
                # crate that comes later alphabetically
                if crate_a > crate_b:
                    edges_to_remove.append((crate_a, crate_b))
                    cycles_found.append((crate_a, crate_b))

    for src, dst in edges_to_remove:
        if dst in all_deps.get(src, []):
            all_deps[src] = [d for d in all_deps[src] if d != dst]

    return cycles_found


def cmd_extract(args, config: TranspilerConfig):
    """Run Clang AST extraction on the codebase."""
    from extractor.ast_parser import ASTParser
    from extractor.type_collector import TypeCollector

    parser = ASTParser(config)

    if args.module:
        modules = [args.module]
    else:
        modules = config.get_v8_modules()

    print(f"Extracting AST for {len(modules)} modules...")
    for module in modules:
        files = config.get_module_files(module)
        if not files:
            print(f"  [{module}] no files found, skipping")
            continue
        print(f"  [{module}] {len(files)} files")
        extracted = parser.parse_module(module, files)

        # Cache results
        cache_path = config.cache_dir / "ast" / f"{module}.json"
        cache_path.parent.mkdir(parents=True, exist_ok=True)
        with open(cache_path, "w") as f:
            json.dump(extracted, f, indent=2, default=str)
        print(f"    → cached to {cache_path}")

    # Also extract include/ headers
    if not args.module:
        include_files = config.get_include_files()
        if include_files:
            print(f"  [include] {len(include_files)} headers")
            extracted = parser.parse_module("include", include_files)
            cache_path = config.cache_dir / "ast" / "include.json"
            cache_path.parent.mkdir(parents=True, exist_ok=True)
            with open(cache_path, "w") as f:
                json.dump(extracted, f, indent=2, default=str)


def cmd_analyze(args, config: TranspilerConfig):
    """Build type registry and dependency graph from extracted ASTs."""
    from ir.type_registry import TypeRegistry
    from ir.dependency_graph import DependencyGraph

    registry = TypeRegistry()
    dep_graph = DependencyGraph()

    # Load cached ASTs
    ast_cache = config.cache_dir / "ast"
    if not ast_cache.exists():
        print("Error: No cached ASTs found. Run 'extract' first.")
        sys.exit(1)

    for ast_file in sorted(ast_cache.glob("*.json")):
        module = ast_file.stem
        print(f"  Loading {module}...")
        with open(ast_file) as f:
            data = json.load(f)
        registry.load_module(module, data)
        dep_graph.add_module(module, data)

    # Save analysis
    analysis_dir = config.cache_dir / "analysis"
    analysis_dir.mkdir(parents=True, exist_ok=True)

    registry.save(analysis_dir / "type_registry.json")
    dep_graph.save(analysis_dir / "dependency_graph.json")

    print(f"\nType registry: {registry.type_count()} types")
    print(f"Dependency graph: {dep_graph.module_count()} modules")
    print(f"Conversion order: {dep_graph.topological_order()}")


def cmd_transpile(args, config: TranspilerConfig):
    """Run the full transpilation pipeline."""
    from extractor.ast_parser import ASTParser
    from ir.type_registry import TypeRegistry
    from ir.dependency_graph import DependencyGraph
    from mapper.type_mapper import TypeMapper
    from mapper.class_mapper import ClassMapper
    from mapper.stmt_mapper import StmtMapper
    from mapper.v8_mapper import V8Mapper
    from mapper.stdlib_mapper import StdlibMapper
    from mapper.template_mapper import TemplateMapper
    from codegen.rust_emitter import RustEmitter
    from codegen.module_emitter import ModuleEmitter
    from codegen.cargo_emitter import CargoEmitter

    # Initialize components
    type_mapper = TypeMapper()
    class_mapper = ClassMapper(type_mapper)
    stmt_mapper = StmtMapper(type_mapper)
    v8_mapper = V8Mapper()
    stdlib_mapper = StdlibMapper()
    template_mapper = TemplateMapper(type_mapper)
    rust_emitter = RustEmitter()
    module_emitter = ModuleEmitter(rust_emitter, config)
    cargo_emitter = CargoEmitter(config)

    parser = ASTParser(config)

    if args.module:
        modules = [args.module]
    else:
        modules = config.get_v8_modules()

    config.output_root.mkdir(parents=True, exist_ok=True)

    # Pre-scan: collect all inter-module dependencies and break cycles
    print(f"Scanning dependencies for {len(modules)} modules...")
    all_deps = {}
    module_files = {}
    for module in modules:
        files = config.get_module_files(module)
        if not files:
            continue
        module_files[module] = files
        crate_name = config.module_to_crate_name(module)
        deps = _scan_include_deps(files, module, config)
        all_deps[crate_name] = deps

    cycles = _break_dependency_cycles(all_deps)
    if cycles:
        print(f"  Broke {len(cycles)} dependency cycle(s): {cycles}")

    print(f"Transpiling {len(modules)} modules...")
    start = time.time()

    for module in modules:
        files = module_files.get(module)
        if not files:
            continue

        t0 = time.time()
        print(f"\n  [{module}] Parsing {len(files)} C++ files...")

        # Phase 1: Extract
        extract_bodies = not getattr(args, 'no_bodies', False)
        parsed = parser.parse_module(module, files, extract_bodies=extract_bodies)

        # Phase 2-3: Map to IR
        print(f"  [{module}] Mapping to Rust IR...")
        ir_module = class_mapper.map_module(module, parsed, config)

        # Phase 3.5: Use pre-scanned (cycle-free) dependencies
        crate_name = config.module_to_crate_name(module)
        ir_module.dependencies = all_deps.get(crate_name, [])

        # Phase 4: Emit Rust source + Cargo.toml
        print(f"  [{module}] Emitting Rust code...")
        module_emitter.emit_module(ir_module)
        cargo_emitter.emit_crate(ir_module)

        elapsed = time.time() - t0
        file_count = len(ir_module.files) if hasattr(ir_module, 'files') else 0
        print(f"  [{module}] Done ({elapsed:.1f}s, {file_count} Rust files)")

    # Emit workspace Cargo.toml
    cargo_emitter.emit_workspace(modules)

    total = time.time() - start
    print(f"\nTranspilation complete in {total:.1f}s")
    print(f"Output: {config.output_root}")


def cmd_validate(args, config: TranspilerConfig):
    """Run cargo check on generated output."""
    import subprocess

    output = config.output_root
    if not (output / "Cargo.toml").exists():
        print("Error: No Cargo.toml found. Run 'transpile' first.")
        sys.exit(1)

    print(f"Running cargo check in {output}...")
    result = subprocess.run(
        ["cargo", "check", "--workspace"],
        cwd=output,
        capture_output=True,
        text=True,
        timeout=300,
    )

    if result.returncode == 0:
        print("cargo check passed!")
    else:
        # Parse and summarize errors
        errors = result.stderr
        error_count = errors.count("error[E")
        warning_count = errors.count("warning:")
        print(f"cargo check failed: {error_count} errors, {warning_count} warnings")
        if args.verbose:
            print(errors)
        else:
            # Show first 50 lines
            lines = errors.split("\n")
            for line in lines[:50]:
                print(line)
            if len(lines) > 50:
                print(f"... ({len(lines) - 50} more lines, use --verbose)")


def cmd_status(args, config: TranspilerConfig):
    """Show transpilation progress."""
    modules = config.get_v8_modules()
    ast_cache = config.cache_dir / "ast"
    output = config.output_root

    print("Module status:")
    print(f"{'Module':<25} {'C++ Files':<12} {'AST Cached':<12} {'Rust Output':<12}")
    print("-" * 61)

    total_cpp = 0
    total_cached = 0
    total_rust = 0

    for module in modules:
        cpp_files = len(config.get_module_files(module))
        total_cpp += cpp_files

        cached = "yes" if (ast_cache / f"{module}.json").exists() else "no"
        if cached == "yes":
            total_cached += 1

        rust_dir = output / "crates" / config.module_to_crate_name(module)
        rust_files = len(list(rust_dir.rglob("*.rs"))) if rust_dir.exists() else 0
        total_rust += rust_files

        status = "done" if rust_files > 0 else ("extracted" if cached == "yes" else "pending")
        print(f"{module:<25} {cpp_files:<12} {cached:<12} {rust_files:<12} {status}")

    print("-" * 61)
    print(f"{'TOTAL':<25} {total_cpp:<12} {total_cached:<12} {total_rust:<12}")


def cmd_clean(args, config: TranspilerConfig):
    """Clean cached data and/or output."""
    import shutil

    if args.target in ("cache", "all"):
        if config.cache_dir.exists():
            shutil.rmtree(config.cache_dir)
            print(f"Cleaned cache: {config.cache_dir}")

    if args.target in ("output", "all"):
        if config.output_root.exists():
            shutil.rmtree(config.output_root)
            print(f"Cleaned output: {config.output_root}")


def main():
    parser = argparse.ArgumentParser(
        description="C++ to Rust transpiler for V8",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # extract
    p_extract = subparsers.add_parser("extract", help="Extract Clang AST from C++ sources")
    p_extract.add_argument("-m", "--module", help="Only extract a specific module")

    # analyze
    p_analyze = subparsers.add_parser("analyze", help="Build type registry and dependency graph")

    # transpile
    p_transpile = subparsers.add_parser("transpile", help="Run full transpilation pipeline")
    p_transpile.add_argument("-m", "--module", help="Only transpile a specific module")
    p_transpile.add_argument("--no-bodies", action="store_true",
                             help="Skip function body extraction (faster, bodies become todo!())")

    # validate
    p_validate = subparsers.add_parser("validate", help="Run cargo check on output")
    p_validate.add_argument("-v", "--verbose", action="store_true")

    # status
    p_status = subparsers.add_parser("status", help="Show transpilation progress")

    # clean
    p_clean = subparsers.add_parser("clean", help="Clean cache and/or output")
    p_clean.add_argument("target", choices=["cache", "output", "all"], default="all", nargs="?")

    args = parser.parse_args()
    config = TranspilerConfig()

    commands = {
        "extract": cmd_extract,
        "analyze": cmd_analyze,
        "transpile": cmd_transpile,
        "validate": cmd_validate,
        "status": cmd_status,
        "clean": cmd_clean,
    }

    commands[args.command](args, config)


if __name__ == "__main__":
    main()

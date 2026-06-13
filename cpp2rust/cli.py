"""Command-line interface for cpp2rust."""

from __future__ import annotations

import argparse
import importlib
import json
import sys
from pathlib import Path
from typing import Optional


def load_plugin(plugin_name: Optional[str]):
    """Import and instantiate a plugin by name.

    Looks for cpp2rust/plugins/<name>/plugin.py and returns an instance
    of the first CppPlugin subclass found there.  Falls back to the
    DefaultPlugin if no name is given.
    """
    from cpp2rust.core.plugin import CppPlugin

    if plugin_name is None:
        from cpp2rust.plugins.default import DefaultPlugin
        return DefaultPlugin()

    plugin_path = Path(__file__).parent / "plugins" / plugin_name / "plugin.py"
    if not plugin_path.exists():
        print(f"Error: plugin '{plugin_name}' not found at {plugin_path}", file=sys.stderr)
        sys.exit(1)

    spec = importlib.util.spec_from_file_location(
        f"cpp2rust.plugins.{plugin_name}.plugin", plugin_path
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    # Find the first CppPlugin subclass in the module
    for attr_name in dir(module):
        attr = getattr(module, attr_name)
        if (
            isinstance(attr, type)
            and issubclass(attr, CppPlugin)
            and attr is not CppPlugin
        ):
            return attr()

    print(f"Error: no CppPlugin subclass found in {plugin_path}", file=sys.stderr)
    sys.exit(1)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="cpp2rust",
        description="General-purpose C++ to Rust transpiler",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # ── transpile command ─────────────────────────────────────────────────
    tp = subparsers.add_parser("transpile", help="Transpile C++ source to Rust")
    tp.add_argument(
        "--plugin", "-p",
        metavar="NAME",
        default=None,
        help="Plugin name (e.g. v8). Loads plugins/<name>/plugin.py",
    )
    tp.add_argument(
        "--compile-commands", "-c",
        metavar="PATH",
        default="compile_commands.json",
        help="Path to compile_commands.json (default: ./compile_commands.json)",
    )
    tp.add_argument(
        "--module", "-m",
        metavar="MODULE",
        default=None,
        help="Limit transpilation to a single module/crate name (e.g. bigint)",
    )
    tp.add_argument(
        "--output", "-o",
        metavar="DIR",
        default="output/transpiled",
        help="Output directory for the Cargo workspace (default: output/transpiled)",
    )
    tp.add_argument(
        "--phase",
        type=int,
        choices=[1, 2],
        default=1,
        help="Transpilation phase: 1=unsafe (always compiles), 2=safe lift (default: 1)",
    )
    tp.add_argument(
        "--no-cache",
        action="store_true",
        help="Ignore cached extraction results and re-parse all files",
    )
    tp.add_argument(
        "--jobs", "-j",
        type=int,
        default=4,
        help="Number of parallel extraction workers (default: 4)",
    )

    # ── check command ─────────────────────────────────────────────────────
    ck = subparsers.add_parser("check", help="Check output for invariant violations")
    ck.add_argument("output_dir", metavar="DIR", help="Path to transpiled workspace")
    ck.add_argument(
        "--no-todo",
        action="store_true",
        default=True,
        help="Fail if any todo!() remains in output (default: enabled)",
    )

    # ── validate command ──────────────────────────────────────────────────
    vl = subparsers.add_parser(
        "validate", help="Run the layered V8 validation suite and print the scorecard"
    )
    vl.add_argument(
        "--layer", "-l", action="append", dest="layers", metavar="LAYER",
        help="Run only this layer (repeatable). Default: all layers with a manifest.",
    )
    vl.add_argument(
        "--no-build", action="store_true",
        help="Skip the cargo build stage (faster; transpile + no-todo only)",
    )
    vl.add_argument(
        "--baseline", metavar="PATH", default=None,
        help="Baseline scorecard JSON for regression detection (default: v8-layers/baseline.json)",
    )
    vl.add_argument(
        "--update-baseline", action="store_true",
        help="Write the current scorecard as the new baseline",
    )
    vl.add_argument(
        "--json", action="store_true", dest="as_json",
        help="Emit the scorecard as JSON instead of a human table",
    )

    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "transpile":
        from cpp2rust.main import run_transpile
        plugin = load_plugin(args.plugin)
        run_transpile(
            plugin=plugin,
            compile_commands=Path(args.compile_commands),
            module_filter=args.module,
            output_dir=Path(args.output),
            phase=args.phase,
            no_cache=args.no_cache,
            jobs=args.jobs,
        )

    elif args.command == "check":
        from cpp2rust.main import run_check
        run_check(output_dir=Path(args.output_dir), no_todo=args.no_todo)

    elif args.command == "validate":
        from cpp2rust.validation.suite import run_and_report
        code = run_and_report(
            layers=args.layers,
            build=not args.no_build,
            baseline_path=Path(args.baseline) if args.baseline else None,
            update_baseline=args.update_baseline,
            as_json=args.as_json,
        )
        sys.exit(code)


if __name__ == "__main__":
    main()

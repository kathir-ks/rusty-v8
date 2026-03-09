#!/usr/bin/env python3
"""V8 C++ to Rust Conversion Pipeline — CLI entry point."""

import argparse
import logging
import sys
from pathlib import Path

from converter.config import PipelineConfig
from converter.models.provider import create_provider
from converter.stages import analyze, graph, plan, convert, validate


def setup_logging(verbose: bool = False):
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(
        level=level,
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )


# ── Stage commands ──────────────────────────────────────────────────────


def cmd_analyze(config: PipelineConfig, args):
    results = analyze.run(config)
    modules = set(f.module for f in results.values())
    print(f"\nAnalyzed {len(results)} files across {len(modules)} modules")


def cmd_graph(config: PipelineConfig, args):
    analysis = analyze.run(config)
    modules = graph.run(config, analysis)
    print(f"\nBuilt dependency graph with {len(modules)} modules")


def cmd_plan(config: PipelineConfig, args):
    analysis = analyze.run(config)
    modules = graph.run(config, analysis)
    conversion_plan = plan.run(config, modules)
    print(f"\nConversion plan ready: {len(conversion_plan.ordered_modules)} modules")


def cmd_convert(config: PipelineConfig, args):
    provider = create_provider(
        config.model.provider,
        config.model.name,
        config.model.api_key_env,
        config.model.max_tokens,
    )

    analysis = analyze.run(config)
    modules = graph.run(config, analysis)
    conversion_plan = plan.run(config, modules)

    # Optionally filter to a single module
    if args.module:
        conversion_plan.ordered_modules = [
            m for m in conversion_plan.ordered_modules if m.name == args.module
        ]
        if not conversion_plan.ordered_modules:
            print(f"Module '{args.module}' not found")
            sys.exit(1)

    converted = convert.run(config, conversion_plan, provider)
    print(f"\nConverted {len(converted)} modules to {config.output_dir}")


def cmd_validate(config: PipelineConfig, args):
    provider = None
    if args.fix:
        provider = create_provider(
            config.model.provider,
            config.model.name,
            config.model.api_key_env,
            config.model.max_tokens,
        )

    result = validate.run(config, provider)
    if result["success"]:
        print("\nAll crates compile successfully!")
    else:
        print(f"\nValidation found {result.get('errors', '?')} errors")


def cmd_run(config: PipelineConfig, args):
    """Run the full pipeline end-to-end."""
    provider = create_provider(
        config.model.provider,
        config.model.name,
        config.model.api_key_env,
        config.model.max_tokens,
    )

    print("=" * 60)
    print("  Stage 1: Analyze")
    print("=" * 60)
    analysis = analyze.run(config)

    print("\n" + "=" * 60)
    print("  Stage 2: Build dependency graph")
    print("=" * 60)
    modules = graph.run(config, analysis)

    print("\n" + "=" * 60)
    print("  Stage 3: Create conversion plan")
    print("=" * 60)
    conversion_plan = plan.run(config, modules)

    print("\n" + "=" * 60)
    print("  Stage 4: Convert C++ -> Rust")
    print("=" * 60)
    converted = convert.run(config, conversion_plan, provider)

    print("\n" + "=" * 60)
    print("  Stage 5: Validate")
    print("=" * 60)
    result = validate.run(config, provider if args.fix else None)

    print("\n" + "=" * 60)
    print("  Pipeline Complete")
    print("=" * 60)
    print(f"  Modules converted : {len(converted)}/{len(conversion_plan.ordered_modules)}")
    print(f"  Compilation        : {'PASS' if result['success'] else 'FAIL'}")
    if not result["success"]:
        print(f"  Remaining errors   : {result.get('errors', '?')}")


def cmd_status(config: PipelineConfig, args):
    """Show current pipeline status."""
    cache_dir = config.cache_dir

    stage_files = {
        "analyze": cache_dir / "analysis_summary.json",
        "graph": cache_dir / "module_graph.json",
        "plan": cache_dir / "conversion_plan.json",
    }

    print("Pipeline Status")
    print("-" * 40)

    for stage, path in stage_files.items():
        status = "done" if path.exists() else "pending"
        print(f"  {stage:12s} : {status}")

    convert_dir = cache_dir / "convert"
    if convert_dir.exists():
        cached = list(convert_dir.glob("*.json"))
        print(f"  {'convert':12s} : {len(cached)} modules cached")
    else:
        print(f"  {'convert':12s} : pending")

    output_dir = config.output_dir
    if (output_dir / "Cargo.toml").exists():
        crates_dir = output_dir / "crates"
        crates = list(crates_dir.iterdir()) if crates_dir.exists() else []
        print(f"  {'output':12s} : {len(crates)} crates generated")
    else:
        print(f"  {'output':12s} : pending")


def cmd_clean(config: PipelineConfig, args):
    """Clear cached state for one or all stages."""
    import shutil

    if args.stage == "all":
        if config.cache_dir.exists():
            shutil.rmtree(config.cache_dir)
            print("Cleared all cached pipeline state")
    else:
        stage_dir = config.cache_dir / args.stage
        if stage_dir.exists():
            shutil.rmtree(stage_dir)
            print(f"Cleared cache for stage: {args.stage}")
        # Also remove the stage artifact
        artifacts = {
            "analyze": "analysis_summary.json",
            "graph": "module_graph.json",
            "plan": "conversion_plan.json",
        }
        if args.stage in artifacts:
            art = config.cache_dir / artifacts[args.stage]
            if art.exists():
                art.unlink()


# ── CLI definition ──────────────────────────────────────────────────────


def main():
    parser = argparse.ArgumentParser(
        description="V8 C++ to Rust Conversion Pipeline",
    )
    parser.add_argument(
        "-c", "--config", default="config.yaml", help="Config file path"
    )
    parser.add_argument(
        "-v", "--verbose", action="store_true", help="Verbose logging"
    )

    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("analyze", help="Stage 1: Analyze C++ source files")
    sub.add_parser("graph", help="Stage 2: Build dependency graph")
    sub.add_parser("plan", help="Stage 3: Create conversion plan")

    p_convert = sub.add_parser("convert", help="Stage 4: Convert C++ to Rust")
    p_convert.add_argument(
        "-m", "--module", help="Convert only this module (e.g. 'base')"
    )

    p_validate = sub.add_parser("validate", help="Stage 5: Validate Rust output")
    p_validate.add_argument(
        "--fix", action="store_true", help="Auto-fix errors using AI"
    )

    p_run = sub.add_parser("run", help="Run full pipeline (stages 1-5)")
    p_run.add_argument(
        "--fix", action="store_true", help="Auto-fix validation errors"
    )

    sub.add_parser("status", help="Show pipeline status")

    p_clean = sub.add_parser("clean", help="Clear cached pipeline state")
    p_clean.add_argument(
        "stage",
        nargs="?",
        default="all",
        help="Stage to clean (analyze/graph/plan/convert/all)",
    )

    args = parser.parse_args()
    setup_logging(args.verbose)

    config_path = Path(args.config)
    if not config_path.exists():
        print(f"Config file not found: {config_path}")
        sys.exit(1)

    config = PipelineConfig.from_yaml(config_path)

    commands = {
        "analyze": cmd_analyze,
        "graph": cmd_graph,
        "plan": cmd_plan,
        "convert": cmd_convert,
        "validate": cmd_validate,
        "run": cmd_run,
        "status": cmd_status,
        "clean": cmd_clean,
    }

    commands[args.command](config, args)


if __name__ == "__main__":
    main()

"""Stage 1: Analyze C++ source files and extract structural information.

This stage walks the V8 source tree and uses regex-based parsing to extract:
- #include dependencies
- namespace declarations
- class/struct declarations
- function signatures

Results are cached per-file for incremental re-runs.
"""

import json
import logging
from pathlib import Path

from converter.config import PipelineConfig
from converter.graph.types import FileInfo
from converter.utils.cpp_parser import find_source_files, parse_file
from converter.utils.cache import PipelineCache

logger = logging.getLogger(__name__)


def run(config: PipelineConfig) -> dict[str, FileInfo]:
    """Analyze all C++ source files and return a map of path -> FileInfo."""
    cache = PipelineCache(config.cache_dir)
    source_root = config.source.root

    files = find_source_files(
        source_root,
        config.source.extensions,
        config.source.exclude_patterns,
    )
    logger.info(f"Found {len(files)} source files")

    results: dict[str, FileInfo] = {}
    new_count = 0
    cached_count = 0

    for path in files:
        rel = path.relative_to(source_root).as_posix()

        cached = cache.get("analyze", rel)
        if cached:
            results[rel] = FileInfo.from_dict(cached)
            cached_count += 1
            continue

        info = parse_file(path, source_root)
        results[rel] = info
        cache.set("analyze", rel, info.to_dict())
        new_count += 1

    logger.info(f"Analysis complete: {new_count} new, {cached_count} cached")

    # Build and save summary
    summary: dict[str, dict] = {}
    for info in results.values():
        mod = info.module
        if mod not in summary:
            summary[mod] = {"files": 0, "classes": 0, "functions": 0, "headers": 0}
        summary[mod]["files"] += 1
        summary[mod]["classes"] += len(info.classes)
        summary[mod]["functions"] += len(info.functions)
        if info.is_header:
            summary[mod]["headers"] += 1

    summary_path = config.cache_dir / "analysis_summary.json"
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    with open(summary_path, "w") as f:
        json.dump({"total_files": len(results), "modules": summary}, f, indent=2)
    logger.info(f"Summary saved to {summary_path}")

    return results

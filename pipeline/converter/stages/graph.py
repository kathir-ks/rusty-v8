"""Stage 2: Build module-level dependency graph from analysis results.

Groups files into modules (one per V8 src/ subdirectory) and resolves
inter-module dependencies by analyzing #include paths.
"""

import json
import logging

from converter.config import PipelineConfig
from converter.graph.types import FileInfo, ModuleInfo
from converter.utils.cpp_parser import resolve_module

logger = logging.getLogger(__name__)


def to_crate_name(module: str) -> str:
    """Convert a V8 module name to a Rust crate name."""
    if module == "include":
        return "v8_api"
    return f"v8_{module.replace('-', '_')}"


def run(config: PipelineConfig, analysis: dict[str, FileInfo]) -> dict[str, ModuleInfo]:
    """Build module-level dependency graph from file analysis."""
    modules: dict[str, ModuleInfo] = {}

    # Group files by module
    for rel_path, info in analysis.items():
        mod_name = info.module
        if mod_name not in modules:
            modules[mod_name] = ModuleInfo(
                name=mod_name,
                crate_name=to_crate_name(mod_name),
            )
        modules[mod_name].files.append(info)

    # Resolve inter-module dependencies from #include paths
    for mod_name, mod_info in modules.items():
        for file_info in mod_info.files:
            for inc in file_info.includes:
                dep_module = resolve_module(inc)
                if dep_module and dep_module != mod_name and dep_module in modules:
                    mod_info.dependencies.add(dep_module)

    logger.info(f"Built graph with {len(modules)} modules:")
    for name, mod in sorted(modules.items()):
        deps = ", ".join(sorted(mod.dependencies)) or "(none)"
        logger.info(f"  {name}: {len(mod.files)} files, deps: [{deps}]")

    # Save graph
    graph_path = config.cache_dir / "module_graph.json"
    graph_path.parent.mkdir(parents=True, exist_ok=True)
    graph_data = {name: mod.to_dict() for name, mod in modules.items()}
    with open(graph_path, "w") as f:
        json.dump(graph_data, f, indent=2)
    logger.info(f"Graph saved to {graph_path}")

    return modules

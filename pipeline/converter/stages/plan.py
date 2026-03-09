"""Stage 3: Create a topologically-ordered conversion plan.

Modules are ordered so that dependencies are converted before dependents.
Cycles (common in V8) are detected and handled gracefully by breaking
ties with module size.
"""

import logging
from collections import deque

from converter.config import PipelineConfig
from converter.graph.types import ModuleInfo, ConversionPlan

logger = logging.getLogger(__name__)


def topological_sort(modules: dict[str, ModuleInfo]) -> list[str]:
    """Topological sort of modules. Handles cycles by appending them at the end."""
    in_degree: dict[str, int] = {name: 0 for name in modules}
    adj: dict[str, list[str]] = {name: [] for name in modules}

    for mod in modules.values():
        for dep in mod.dependencies:
            if dep in modules:
                adj[dep].append(mod.name)
                in_degree[mod.name] += 1

    # Kahn's algorithm
    queue = deque(name for name, deg in in_degree.items() if deg == 0)
    result = []

    while queue:
        node = queue.popleft()
        result.append(node)
        for neighbor in adj[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    # Remaining nodes are in cycles — add sorted by file count (smallest first)
    remaining = set(modules.keys()) - set(result)
    if remaining:
        logger.warning(f"Dependency cycles detected among {len(remaining)} modules: "
                       f"{', '.join(sorted(remaining))}")
        result.extend(sorted(remaining, key=lambda n: len(modules[n].files)))

    return result


def run(config: PipelineConfig, modules: dict[str, ModuleInfo]) -> ConversionPlan:
    """Create an ordered conversion plan."""
    order = topological_sort(modules)

    plan = ConversionPlan(
        ordered_modules=[modules[name] for name in order],
    )

    logger.info(f"Conversion order ({len(order)} modules):")
    for i, name in enumerate(order):
        mod = modules[name]
        logger.info(
            f"  {i + 1:3d}. {name} ({len(mod.files)} files, "
            f"{len(mod.dependencies)} deps)"
        )

    plan_path = config.cache_dir / "conversion_plan.json"
    plan.save(plan_path)
    logger.info(f"Plan saved to {plan_path}")

    return plan

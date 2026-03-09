"""Stage 4: Convert C++ modules to Rust using an AI model.

Converts modules in the order defined by the conversion plan, passing
already-converted dependency context to each subsequent module.
Files within large modules are batched to stay within token limits.
"""

import logging
import re
import time
from pathlib import Path

from converter.config import PipelineConfig
from converter.graph.types import ConversionPlan, ModuleInfo
from converter.models.provider import ModelProvider
from converter.models.prompts import SYSTEM_PROMPT, module_conversion_prompt
from converter.utils.cache import PipelineCache

logger = logging.getLogger(__name__)

FILE_MARKER_RE = re.compile(r'^// === FILE:\s*(.+?)\s*===\s*$', re.MULTILINE)


def split_output_files(response: str) -> dict[str, str]:
    """Split an AI response into individual Rust files by marker."""
    # Strip markdown fences if the model included them
    response = re.sub(r'^```\w*\n?', '', response, flags=re.MULTILINE)
    response = re.sub(r'^```\s*$', '', response, flags=re.MULTILINE)

    parts = FILE_MARKER_RE.split(response)
    files: dict[str, str] = {}

    if len(parts) < 2:
        # No markers — treat entire response as a single file
        stripped = response.strip()
        if stripped:
            files["output.rs"] = stripped
        return files

    for i in range(1, len(parts), 2):
        if i + 1 < len(parts):
            filename = parts[i].strip()
            content = parts[i + 1].strip()
            if content:
                files[filename] = content

    return files


def read_source_files(config: PipelineConfig, module: ModuleInfo) -> list[dict]:
    """Read C++ source files for a module, truncating oversized files."""
    source_root = config.source.root
    max_size = config.conversion.max_file_size
    files = []

    for file_info in module.files:
        path = source_root / file_info.path
        if not path.exists():
            continue
        try:
            content = path.read_text(encoding="utf-8", errors="replace")
            if len(content) > max_size:
                content = content[:max_size] + "\n// ... truncated ..."
            files.append({"path": file_info.path, "content": content})
        except Exception as e:
            logger.warning(f"Could not read {path}: {e}")

    return files


def build_dependency_context(
    module: ModuleInfo,
    converted: dict[str, dict[str, str]],
) -> str:
    """Build a context string of public signatures from already-converted deps."""
    context_parts = []

    for dep_name in sorted(module.dependencies):
        if dep_name not in converted:
            continue
        dep_files = converted[dep_name]
        for filename, content in dep_files.items():
            # Extract only pub items to keep context small
            pub_lines = [
                line for line in content.split("\n")
                if line.strip().startswith("pub ")
            ]
            if pub_lines:
                context_parts.append(f"// From crate v8_{dep_name} — {filename}")
                context_parts.append("\n".join(pub_lines[:50]))

    return "\n\n".join(context_parts)


def sanitize_filename(name: str) -> str:
    """Convert a C++ filename to a valid Rust module filename."""
    name = name.replace("-", "_").replace(".cc", "").replace(".h", "").replace(".cpp", "")
    # Strip directory components — keep only the leaf
    name = name.split("/")[-1]
    if not name.endswith(".rs"):
        name += ".rs"
    return name


def write_crate(
    config: PipelineConfig,
    module: ModuleInfo,
    files: dict[str, str],
):
    """Write a Rust crate to disk with Cargo.toml and source files."""
    crate_dir = config.output_dir / "crates" / module.crate_name
    src_dir = crate_dir / "src"
    src_dir.mkdir(parents=True, exist_ok=True)

    # Cargo.toml
    dep_lines = []
    for dep in sorted(module.dependencies):
        dep_crate = f"v8_{dep.replace('-', '_')}" if dep != "include" else "v8_api"
        dep_lines.append(f'{dep_crate} = {{ path = "../{dep_crate}" }}')
    deps_toml = "\n".join(dep_lines)

    cargo_toml = (
        f'[package]\nname = "{module.crate_name}"\nversion = "0.1.0"\n'
        f'edition = "2021"\n\n[dependencies]\n{deps_toml}\n'
    )
    (crate_dir / "Cargo.toml").write_text(cargo_toml)

    # Write source files
    mod_names = []
    for raw_filename, content in files.items():
        safe = sanitize_filename(raw_filename)
        mod_name = safe.replace(".rs", "")

        if mod_name not in ("lib", "mod"):
            mod_names.append(mod_name)

        (src_dir / safe).write_text(content)

    # lib.rs with module declarations
    lib_lines = ["// Auto-generated — do not edit by hand", ""]
    for mod_name in sorted(set(mod_names)):
        lib_lines.append(f"pub mod {mod_name};")
    lib_lines.append("")
    (src_dir / "lib.rs").write_text("\n".join(lib_lines))


def convert_module(
    config: PipelineConfig,
    module: ModuleInfo,
    provider: ModelProvider,
    converted: dict[str, dict[str, str]],
    cache: PipelineCache,
) -> dict[str, str]:
    """Convert a single module from C++ to Rust."""
    cached = cache.get("convert", module.name)
    if cached:
        logger.info(f"  [{module.name}] Using cached conversion")
        return cached

    source_files = read_source_files(config, module)
    if not source_files:
        logger.warning(f"  [{module.name}] No source files found, skipping")
        return {}

    batch_size = config.conversion.batch_size
    all_output_files: dict[str, str] = {}

    for batch_start in range(0, len(source_files), batch_size):
        batch = source_files[batch_start:batch_start + batch_size]
        batch_num = batch_start // batch_size + 1
        total_batches = (len(source_files) + batch_size - 1) // batch_size

        logger.info(
            f"  [{module.name}] batch {batch_num}/{total_batches} "
            f"({len(batch)} files)"
        )

        dep_context = build_dependency_context(module, converted)
        prompt = module_conversion_prompt(
            module_name=module.name,
            source_files=batch,
            dependency_context=dep_context,
            crate_name=module.crate_name,
        )

        for attempt in range(config.conversion.max_retries):
            try:
                response = provider.generate(prompt, system=SYSTEM_PROMPT)
                output_files = split_output_files(response)

                if output_files:
                    all_output_files.update(output_files)
                    break

                logger.warning(
                    f"  [{module.name}] Empty response, attempt {attempt + 1}"
                )
            except Exception as e:
                logger.error(
                    f"  [{module.name}] Error on attempt {attempt + 1}: {e}"
                )
                if attempt < config.conversion.max_retries - 1:
                    time.sleep(config.conversion.retry_delay * (attempt + 1))

    if all_output_files:
        write_crate(config, module, all_output_files)
        cache.set("convert", module.name, all_output_files)

    return all_output_files


def run(
    config: PipelineConfig,
    plan: ConversionPlan,
    provider: ModelProvider,
) -> dict[str, dict[str, str]]:
    """Convert all modules according to the plan."""
    cache = PipelineCache(config.cache_dir)
    converted: dict[str, dict[str, str]] = {}

    # Write workspace Cargo.toml
    workspace_dir = config.output_dir
    workspace_dir.mkdir(parents=True, exist_ok=True)

    members = ",\n    ".join(
        f'"crates/{m.crate_name}"' for m in plan.ordered_modules
    )
    workspace_toml = (
        '[workspace]\nresolver = "2"\nmembers = [\n'
        f'    {members}\n]\n'
    )
    (workspace_dir / "Cargo.toml").write_text(workspace_toml)

    total = len(plan.ordered_modules)
    for i, module in enumerate(plan.ordered_modules):
        logger.info(
            f"[{i + 1}/{total}] Module: {module.name} "
            f"({len(module.files)} files, {len(module.dependencies)} deps)"
        )

        result = convert_module(config, module, provider, converted, cache)
        if result:
            converted[module.name] = result
            logger.info(f"  [{module.name}] Done — {len(result)} Rust files")
        else:
            logger.warning(f"  [{module.name}] No output produced")

    logger.info(f"Conversion complete: {len(converted)}/{total} modules")
    return converted

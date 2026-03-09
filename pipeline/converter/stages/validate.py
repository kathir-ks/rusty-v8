"""Stage 5: Validate converted Rust code with cargo check.

Runs the Rust compiler on the output workspace and optionally uses the
AI provider to attempt automatic fixes on files with errors.
"""

import logging
import re
import subprocess
from pathlib import Path

from converter.config import PipelineConfig
from converter.models.provider import ModelProvider
from converter.models.prompts import SYSTEM_PROMPT, fix_errors_prompt

logger = logging.getLogger(__name__)


def cargo_check(workspace_dir: Path, crate: str | None = None) -> tuple[bool, str]:
    """Run cargo check and return (success, combined_output)."""
    cmd = ["cargo", "check"]
    if crate:
        cmd += ["-p", crate]
    else:
        cmd += ["--workspace"]

    try:
        result = subprocess.run(
            cmd,
            cwd=workspace_dir,
            capture_output=True,
            text=True,
            timeout=300,
        )
        output = (result.stderr + "\n" + result.stdout).strip()
        return result.returncode == 0, output
    except FileNotFoundError:
        return False, "cargo not found — install Rust toolchain: https://rustup.rs"
    except subprocess.TimeoutExpired:
        return False, "cargo check timed out (300s)"


def extract_errors_by_file(cargo_output: str) -> dict[str, list[str]]:
    """Group compiler errors by source file."""
    error_re = re.compile(r'error(?:\[E\d+\])?:.+?\n\s*-->\s*(.+?):(\d+):(\d+)')
    errors_by_file: dict[str, list[str]] = {}

    matches = list(error_re.finditer(cargo_output))
    for idx, match in enumerate(matches):
        file_path = match.group(1)
        start = match.start()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(cargo_output)
        error_block = cargo_output[start:end].strip()

        if file_path not in errors_by_file:
            errors_by_file[file_path] = []
        errors_by_file[file_path].append(error_block)

    return errors_by_file


def fix_file(
    file_path: Path,
    errors: list[str],
    provider: ModelProvider,
    max_retries: int = 2,
) -> bool:
    """Attempt to fix a single Rust file using AI."""
    if not file_path.exists():
        return False

    current_code = file_path.read_text()
    error_text = "\n\n".join(errors)

    for attempt in range(max_retries):
        logger.info(f"    Fix attempt {attempt + 1} for {file_path.name}")
        prompt = fix_errors_prompt(current_code, error_text)

        try:
            response = provider.generate(prompt, system=SYSTEM_PROMPT)
            cleaned = re.sub(r'^```\w*\n?', '', response, flags=re.MULTILINE)
            cleaned = re.sub(r'^```\s*$', '', cleaned, flags=re.MULTILINE).strip()

            if cleaned and len(cleaned) > 50:
                file_path.write_text(cleaned)
                return True
        except Exception as e:
            logger.error(f"    Fix error: {e}")

    return False


def run(config: PipelineConfig, provider: ModelProvider | None = None) -> dict:
    """Validate the workspace. If a provider is given, attempt auto-fixes."""
    workspace_dir = config.output_dir

    if not (workspace_dir / "Cargo.toml").exists():
        logger.error("No workspace Cargo.toml found — run the convert stage first.")
        return {"success": False, "error": "no workspace"}

    logger.info("Running cargo check --workspace ...")
    success, output = cargo_check(workspace_dir)

    if success:
        logger.info("All crates pass cargo check!")
        return {"success": True, "errors": 0}

    errors_by_file = extract_errors_by_file(output)
    total_errors = sum(len(e) for e in errors_by_file.values())
    logger.info(f"Found {total_errors} errors across {len(errors_by_file)} files")

    if provider:
        fixed = 0
        for file_path_str, errors in errors_by_file.items():
            abs_path = workspace_dir / file_path_str
            if abs_path.exists():
                if fix_file(abs_path, errors, provider):
                    fixed += 1

        logger.info(f"Attempted fixes on {fixed}/{len(errors_by_file)} files")

        # Re-check after fixes
        success, output = cargo_check(workspace_dir)
        if success:
            logger.info("All errors fixed!")
            return {"success": True, "errors": 0, "fixed": fixed}

        remaining = extract_errors_by_file(output)
        remaining_count = sum(len(e) for e in remaining.values())
        logger.info(f"Remaining errors: {remaining_count}")
        return {"success": False, "errors": remaining_count, "fixed": fixed}

    # Log a sample of errors
    for file_path, errors in list(errors_by_file.items())[:5]:
        logger.info(f"  {file_path}: {len(errors)} errors")
        for err in errors[:2]:
            for line in err.split("\n")[:3]:
                logger.info(f"    {line}")

    return {
        "success": False,
        "errors": total_errors,
        "files_with_errors": len(errors_by_file),
    }

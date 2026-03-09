import json
import hashlib
from pathlib import Path
from typing import Any, Optional


class PipelineCache:
    """File-based cache for pipeline state. Each stage gets its own subdirectory."""

    def __init__(self, cache_dir: Path):
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)

    def _key_path(self, stage: str, key: str) -> Path:
        safe_key = hashlib.sha256(key.encode()).hexdigest()[:16]
        stage_dir = self.cache_dir / stage
        stage_dir.mkdir(parents=True, exist_ok=True)
        return stage_dir / f"{safe_key}.json"

    def get(self, stage: str, key: str) -> Optional[dict]:
        path = self._key_path(stage, key)
        if path.exists():
            with open(path) as f:
                return json.load(f)
        return None

    def set(self, stage: str, key: str, value: Any):
        path = self._key_path(stage, key)
        with open(path, "w") as f:
            json.dump(value, f, indent=2)

    def has(self, stage: str, key: str) -> bool:
        return self._key_path(stage, key).exists()

    def clear_stage(self, stage: str):
        stage_dir = self.cache_dir / stage
        if stage_dir.exists():
            import shutil
            shutil.rmtree(stage_dir)

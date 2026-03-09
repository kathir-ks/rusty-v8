from dataclasses import dataclass, field
from pathlib import Path

import yaml


@dataclass
class SourceConfig:
    root: Path
    include_dir: str = "include"
    src_dir: str = "src"
    extensions: list[str] = field(default_factory=lambda: [".h", ".cc", ".cpp"])
    exclude_patterns: list[str] = field(default_factory=list)


@dataclass
class ModelConfig:
    provider: str = "claude"
    name: str = "claude-sonnet-4-20250514"
    api_key_env: str = "ANTHROPIC_API_KEY"
    max_tokens: int = 8192


@dataclass
class ConversionConfig:
    max_concurrent: int = 4
    max_retries: int = 3
    max_file_size: int = 150_000
    retry_delay: int = 5
    batch_size: int = 5


@dataclass
class PipelineConfig:
    source: SourceConfig
    output_dir: Path
    cache_dir: Path
    model: ModelConfig
    conversion: ConversionConfig

    @classmethod
    def from_yaml(cls, path: Path) -> "PipelineConfig":
        with open(path) as f:
            raw = yaml.safe_load(f)

        base = path.parent

        return cls(
            source=SourceConfig(
                root=(base / raw["source"]["root"]).resolve(),
                include_dir=raw["source"].get("include_dir", "include"),
                src_dir=raw["source"].get("src_dir", "src"),
                extensions=raw["source"].get("extensions", [".h", ".cc", ".cpp"]),
                exclude_patterns=raw["source"].get("exclude_patterns", []),
            ),
            output_dir=(base / raw["output"]["root"]).resolve(),
            cache_dir=(base / raw["cache"]["dir"]).resolve(),
            model=ModelConfig(
                provider=raw["model"].get("provider", "claude"),
                name=raw["model"].get("name", "claude-sonnet-4-20250514"),
                api_key_env=raw["model"].get("api_key_env", "ANTHROPIC_API_KEY"),
                max_tokens=raw["model"].get("max_tokens", 8192),
            ),
            conversion=ConversionConfig(
                max_concurrent=raw["conversion"].get("max_concurrent", 4),
                max_retries=raw["conversion"].get("max_retries", 3),
                max_file_size=raw["conversion"].get("max_file_size", 150_000),
                retry_delay=raw["conversion"].get("retry_delay", 5),
                batch_size=raw["conversion"].get("batch_size", 5),
            ),
        )

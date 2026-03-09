from dataclasses import dataclass, field
import json
from pathlib import Path


@dataclass
class FileInfo:
    path: str
    module: str
    includes: list[str] = field(default_factory=list)
    namespaces: list[str] = field(default_factory=list)
    classes: list[str] = field(default_factory=list)
    functions: list[str] = field(default_factory=list)
    is_header: bool = False

    def to_dict(self) -> dict:
        return {
            "path": self.path,
            "module": self.module,
            "includes": self.includes,
            "namespaces": self.namespaces,
            "classes": self.classes,
            "functions": self.functions,
            "is_header": self.is_header,
        }

    @classmethod
    def from_dict(cls, d: dict) -> "FileInfo":
        return cls(**d)


@dataclass
class ModuleInfo:
    name: str
    crate_name: str
    files: list[FileInfo] = field(default_factory=list)
    dependencies: set[str] = field(default_factory=set)

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "crate_name": self.crate_name,
            "files": [f.to_dict() for f in self.files],
            "dependencies": sorted(self.dependencies),
        }

    @classmethod
    def from_dict(cls, d: dict) -> "ModuleInfo":
        return cls(
            name=d["name"],
            crate_name=d["crate_name"],
            files=[FileInfo.from_dict(f) for f in d["files"]],
            dependencies=set(d["dependencies"]),
        )


@dataclass
class ConversionPlan:
    ordered_modules: list[ModuleInfo] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "ordered_modules": [m.to_dict() for m in self.ordered_modules],
        }

    @classmethod
    def from_dict(cls, d: dict) -> "ConversionPlan":
        return cls(
            ordered_modules=[ModuleInfo.from_dict(m) for m in d["ordered_modules"]],
        )

    def save(self, path: Path):
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w") as f:
            json.dump(self.to_dict(), f, indent=2)

    @classmethod
    def load(cls, path: Path) -> "ConversionPlan":
        with open(path) as f:
            return cls.from_dict(json.load(f))

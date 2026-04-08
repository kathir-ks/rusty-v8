"""Configuration for the C++ to Rust transpiler."""

import os
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional


@dataclass
class TranspilerConfig:
    """Main configuration for the transpiler."""

    # Root of the rusty-v8 repository
    repo_root: Path = field(default_factory=lambda: Path(__file__).parent.parent)

    # V8 C++ source root
    codebase_root: Path = field(init=False)

    # Output directory for generated Rust code
    output_root: Path = field(init=False)

    # Cache directory for intermediate results (JSON ASTs, etc.)
    cache_dir: Path = field(init=False)

    # Clang parsing configuration
    clang_std: str = "-std=c++17"
    clang_extra_flags: List[str] = field(default_factory=lambda: [
        "-xc++",                              # Treat .h files as C++
        "-DOFFICIAL_BUILD",
        "-DV8_COMPRESS_POINTERS",
        "-DV8_31BIT_SMIS_ON_64BIT_ARCH",
        "-DV8_ENABLE_SANDBOX",
        "-DV8_TARGET_ARCH_X64",
        "-DV8_OS_LINUX",
        "-DV8_OS_POSIX",
        "-DV8_HAVE_TARGET_OS",
        "-DOBJECT_PRINT",
        "-DV8_INTL_SUPPORT",
        "-DV8_ENABLE_WEBASSEMBLY",
        "-DDEBUG",
        "-DV8_ENABLE_CHECKS",
        # Export macros (defined empty so Clang doesn't complain)
        "-DV8_BASE_EXPORT=",
        "-DV8_EXPORT=",
        "-DV8_EXPORT_PRIVATE=",
        "-DV8_INLINE=inline",
        "-DV8_NOINLINE=",
        "-DV8_NODISCARD=",
        "-DV8_WARN_UNUSED_RESULT=",
    ])

    # File extensions to process
    header_extensions: List[str] = field(default_factory=lambda: [".h"])
    source_extensions: List[str] = field(default_factory=lambda: [".cc", ".cpp"])

    # Patterns to skip
    skip_patterns: List[str] = field(default_factory=lambda: [
        "**/test/**",
        "**/*_unittest*",
        "**/*_test.*",
        "**/testing/**",
        "**/fuzzilli/**",
        "**/d8/**",
        "**/torque/**",
    ])

    # Max file size to process (bytes)
    max_file_size: int = 300_000  # 300KB

    def __post_init__(self):
        self.codebase_root = self.repo_root / "codebase"
        self.output_root = self.repo_root / "output" / "transpiled"
        self.cache_dir = self.repo_root / ".transpiler-cache"

    @property
    def v8_src_dir(self) -> Path:
        return self.codebase_root / "src"

    @property
    def v8_include_dir(self) -> Path:
        return self.codebase_root / "include"

    @property
    def clang_include_paths(self) -> List[str]:
        """Include paths needed for Clang to parse V8 headers."""
        paths = [
            str(self.codebase_root),           # for #include "src/..."
            str(self.v8_include_dir),           # for #include "v8.h"
            str(self.v8_src_dir),               # for #include "base/..."
        ]
        # Add C++ standard library include paths (GCC)
        import glob
        for pattern in [
            "/usr/include/c++/*",
            "/usr/include/x86_64-linux-gnu/c++/*",
            "/usr/include/c++/*/backward",
            "/usr/lib/gcc/x86_64-linux-gnu/*/include",
        ]:
            for p in sorted(glob.glob(pattern)):
                paths.append(p)
        # System includes
        paths.extend([
            "/usr/local/include",
            "/usr/include/x86_64-linux-gnu",
            "/usr/include",
        ])
        return paths

    @property
    def clang_args(self) -> List[str]:
        """Full Clang argument list for parsing."""
        args = [self.clang_std]
        args.extend(f"-I{p}" for p in self.clang_include_paths)
        args.extend(self.clang_extra_flags)
        return args

    def get_v8_modules(self) -> List[str]:
        """List all V8 source modules (subdirectories of src/)."""
        modules = []
        if self.v8_src_dir.exists():
            for entry in sorted(self.v8_src_dir.iterdir()):
                if entry.is_dir() and not entry.name.startswith("."):
                    modules.append(entry.name)
        return modules

    def module_to_crate_name(self, module: str) -> str:
        """Convert a V8 module name to a Rust crate name."""
        return MODULE_CRATE_MAP.get(module, f"v8-{module.replace('_', '-')}")

    def crate_to_module_name(self, crate: str) -> str:
        """Convert a Rust crate name back to V8 module name."""
        for mod, cr in MODULE_CRATE_MAP.items():
            if cr == crate:
                return mod
        return crate.replace("v8-", "").replace("-", "_")

    def should_skip(self, path: Path) -> bool:
        """Check if a file path matches any skip pattern."""
        from fnmatch import fnmatch
        rel = str(path)
        for pattern in self.skip_patterns:
            if fnmatch(rel, pattern):
                return True
        return False

    def get_module_files(self, module: str) -> List[Path]:
        """Get all C++ source and header files for a module."""
        module_dir = self.v8_src_dir / module
        if not module_dir.exists():
            return []
        files = []
        for ext in self.header_extensions + self.source_extensions:
            for f in module_dir.rglob(f"*{ext}"):
                if not self.should_skip(f) and f.stat().st_size <= self.max_file_size:
                    files.append(f)
        return sorted(files)

    def get_include_files(self) -> List[Path]:
        """Get all public API header files from include/."""
        files = []
        if self.v8_include_dir.exists():
            for f in self.v8_include_dir.rglob("*.h"):
                if not self.should_skip(f) and f.stat().st_size <= self.max_file_size:
                    files.append(f)
        return sorted(files)


# V8 module name → Rust crate name mapping
MODULE_CRATE_MAP: Dict[str, str] = {
    "api": "v8-api",
    "asmjs": "v8-asmjs",
    "ast": "v8-ast",
    "base": "v8-base",
    "baseline": "v8-baseline",
    "bigint": "v8-bigint",
    "builtins": "v8-builtins",
    "codegen": "v8-codegen",
    "common": "v8-common",
    "compiler": "v8-compiler",
    "compiler-dispatcher": "v8-compiler-dispatcher",
    "date": "v8-date",
    "debug": "v8-debug",
    "deoptimizer": "v8-deoptimizer",
    "diagnostics": "v8-diagnostics",
    "execution": "v8-execution",
    "extensions": "v8-extensions",
    "flags": "v8-flags",
    "handles": "v8-handles",
    "heap": "v8-heap",
    "ic": "v8-ic",
    "init": "v8-init",
    "inspector": "v8-inspector",
    "interpreter": "v8-interpreter",
    "json": "v8-json",
    "libplatform": "v8-libplatform",
    "libsampler": "v8-libsampler",
    "logging": "v8-logging",
    "maglev": "v8-maglev",
    "numbers": "v8-numbers",
    "objects": "v8-objects",
    "parsing": "v8-parsing",
    "profiler": "v8-profiler",
    "regexp": "v8-regexp",
    "roots": "v8-roots",
    "runtime": "v8-runtime",
    "sandbox": "v8-sandbox",
    "snapshot": "v8-snapshot",
    "strings": "v8-strings",
    "tasks": "v8-tasks",
    "temporal": "v8-temporal",
    "tracing": "v8-tracing",
    "trap-handler": "v8-trap-handler",
    "utils": "v8-utils",
    "wasm": "v8-wasm",
    "zone": "v8-zone",
}

# Architecture mapping: C++ define → Rust cfg
ARCH_MAP: Dict[str, str] = {
    "V8_TARGET_ARCH_X64": 'target_arch = "x86_64"',
    "V8_TARGET_ARCH_IA32": 'target_arch = "x86"',
    "V8_TARGET_ARCH_ARM64": 'target_arch = "aarch64"',
    "V8_TARGET_ARCH_ARM": 'target_arch = "arm"',
    "V8_TARGET_ARCH_MIPS64": 'target_arch = "mips64"',
    "V8_TARGET_ARCH_PPC64": 'target_arch = "powerpc64"',
    "V8_TARGET_ARCH_S390X": 'target_arch = "s390x"',
    "V8_TARGET_ARCH_RISCV64": 'target_arch = "riscv64"',
    "V8_TARGET_ARCH_LOONG64": 'target_arch = "loongarch64"',
}

# OS mapping: C++ define → Rust cfg
OS_MAP: Dict[str, str] = {
    "V8_OS_LINUX": 'target_os = "linux"',
    "V8_OS_MACOS": 'target_os = "macos"',
    "V8_OS_WIN": 'target_os = "windows"',
    "V8_OS_ANDROID": 'target_os = "android"',
    "V8_OS_FUCHSIA": 'target_os = "fuchsia"',
    "V8_OS_FREEBSD": 'target_os = "freebsd"',
    "V8_OS_OPENBSD": 'target_os = "openbsd"',
}

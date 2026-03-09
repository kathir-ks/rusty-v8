import re
import fnmatch
from pathlib import Path

from converter.graph.types import FileInfo

# Regex patterns for C++ structure extraction
INCLUDE_RE = re.compile(r'#include\s+[<"]([^>"]+)[>"]')
NAMESPACE_RE = re.compile(r'namespace\s+(\w+)\s*\{')
CLASS_RE = re.compile(
    r'(?:class|struct)\s+(?:V8_EXPORT(?:_PRIVATE)?\s+)?(\w+)'
    r'(?:\s*(?:final\s*)?:\s*(?:public|protected|private)\s+(\w[\w:]*?))?'
    r'\s*\{'
)
FUNC_RE = re.compile(
    r'^(?:(?:static|virtual|inline|constexpr|V8_EXPORT(?:_PRIVATE)?'
    r'|V8_WARN_UNUSED_RESULT)\s+)*'
    r'([\w:*&<>\s]+?)\s+(\w+)\s*\([^)]*\)\s*'
    r'(?:const\s*)?(?:override\s*)?(?:=\s*0\s*)?[;{]',
    re.MULTILINE,
)

# Keywords that match the function regex but aren't functions
_NOT_FUNCTIONS = frozenset({"if", "while", "for", "switch", "return", "catch", "else"})


def resolve_module(include_path: str) -> str | None:
    """Map an include path to a module name.

    Examples:
        'src/compiler/foo.h' -> 'compiler'
        'include/v8-object.h' -> 'include'
    """
    parts = include_path.replace("\\", "/").split("/")
    if len(parts) >= 2 and parts[0] == "src":
        return parts[1]
    if len(parts) >= 1 and parts[0] == "include":
        return "include"
    return None


def parse_file(path: Path, source_root: Path) -> FileInfo:
    """Parse a C++ file and extract structural information."""
    rel = path.relative_to(source_root).as_posix()

    # Determine module from path
    parts = rel.split("/")
    if parts[0] == "src" and len(parts) > 1:
        module = parts[1]
    elif parts[0] == "include":
        module = "include"
    else:
        module = parts[0]

    try:
        content = path.read_text(encoding="utf-8", errors="replace")
    except Exception:
        return FileInfo(path=rel, module=module, is_header=path.suffix == ".h")

    includes = INCLUDE_RE.findall(content)
    namespaces = list(set(NAMESPACE_RE.findall(content)))
    classes = list({m[0] for m in CLASS_RE.findall(content)})

    functions = []
    for m in FUNC_RE.finditer(content):
        name = m.group(2)
        if name not in _NOT_FUNCTIONS:
            functions.append(name)

    return FileInfo(
        path=rel,
        module=module,
        includes=includes,
        namespaces=namespaces,
        classes=classes,
        functions=list(set(functions)),
        is_header=path.suffix == ".h",
    )


def find_source_files(
    source_root: Path,
    extensions: list[str],
    exclude: list[str],
) -> list[Path]:
    """Find all C++ source files under source_root, respecting exclude patterns."""
    files = []
    for ext in extensions:
        for p in source_root.rglob(f"*{ext}"):
            rel = p.relative_to(source_root).as_posix()
            if any(fnmatch.fnmatch(rel, pat) for pat in exclude):
                continue
            files.append(p)
    return sorted(files)

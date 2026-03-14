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

# Regex patterns for type inventory extraction (Feature C)
TYPEDEF_RE = re.compile(r'typedef\s+(.+?)\s+(\w+)\s*;')
USING_TYPE_RE = re.compile(r'using\s+(\w+)\s*=\s*([^;]+);')
ENUM_RE = re.compile(
    r'enum\s+(?:class\s+)?(\w+)\s*(?::\s*[\w:]+\s*)?\{([^}]*)\}',
    re.DOTALL,
)
CONST_DECL_RE = re.compile(
    r'(?:static\s+)?(?:constexpr\s+)?const(?:expr)?\s+'
    r'([\w:]+)\s+(k\w+)\s*=\s*([^;]{1,80});'
)
FORWARD_DECL_RE = re.compile(r'^(?:class|struct)\s+(\w+)\s*;', re.MULTILINE)

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


def extract_type_inventory(source_files: list[dict]) -> str:
    """Extract a condensed type inventory from C++ source files.

    Scans all files (especially headers) for typedefs, using-aliases,
    enum declarations, class/struct names, key constants, and forward
    declarations.  Returns a text summary that can be fed to the AI so
    every batch knows what types exist module-wide.
    """
    typedefs: list[str] = []
    enums: list[str] = []
    constants: list[str] = []
    classes: set[str] = set()
    forward_decls: set[str] = set()

    for f in source_files:
        content = f["content"]

        for m in TYPEDEF_RE.finditer(content):
            typedefs.append(f"typedef {m.group(1)} {m.group(2)};")

        for m in USING_TYPE_RE.finditer(content):
            # Skip 'using namespace ...' lines
            if m.group(2).strip().startswith("namespace"):
                continue
            typedefs.append(f"using {m.group(1)} = {m.group(2).strip()};")

        for m in ENUM_RE.finditer(content):
            name = m.group(1)
            body = m.group(2).strip()
            # Keep only enum name + first few variants to stay compact
            variants = [v.strip().split("=")[0].strip()
                        for v in body.split(",") if v.strip()][:8]
            suffix = ", ..." if len(body.split(",")) > 8 else ""
            enums.append(f"enum {name} {{ {', '.join(variants)}{suffix} }};")

        for m in CONST_DECL_RE.finditer(content):
            constants.append(f"const {m.group(1)} {m.group(2)} = {m.group(3).strip()};")

        for m in CLASS_RE.finditer(content):
            classes.add(m.group(1))

        for m in FORWARD_DECL_RE.finditer(content):
            forward_decls.add(m.group(1))

    # De-duplicate typedefs
    typedefs = list(dict.fromkeys(typedefs))

    sections: list[str] = []

    if typedefs:
        sections.append("// Type aliases\n" + "\n".join(typedefs[:60]))
    if enums:
        sections.append("// Enums\n" + "\n".join(enums[:40]))
    if constants:
        sections.append("// Key constants\n" + "\n".join(constants[:40]))
    if classes:
        sorted_classes = sorted(classes)
        sections.append(
            "// Classes/structs defined in this module\n"
            + "\n".join(f"class {c};" for c in sorted_classes[:80])
        )
    # Forward declarations that aren't already in classes
    extra_fwd = sorted(forward_decls - classes)
    if extra_fwd:
        sections.append(
            "// Forward declarations (defined elsewhere)\n"
            + "\n".join(f"class {c};" for c in extra_fwd[:40])
        )

    return "\n\n".join(sections)

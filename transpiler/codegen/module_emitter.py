"""Module structure emitter for the C++ to Rust transpiler.

Generates the Rust module (crate) directory structure from an IRModule,
including source files, lib.rs with module declarations, and mod.rs files
for nested sub-modules.
"""

from __future__ import annotations

import logging
import re
from collections import defaultdict
from pathlib import Path, PurePosixPath
from typing import TYPE_CHECKING, Dict, List, Optional, Set, Tuple

from ir.nodes import (
    IRFile, IRItem, IRModule, IRStruct, IRTrait, IREnum, IRTypeAlias,
    IRConst, IRFunction, IRImplBlock, IRUseDecl, AccessSpecifier,
)

if TYPE_CHECKING:
    from config import TranspilerConfig

logger = logging.getLogger(__name__)


class ModuleEmitter:
    """Generates the Rust module structure for a crate from an IRModule.

    For each IRModule, creates:
      crates/{crate_name}/
        Cargo.toml          (handled by CargoEmitter)
        src/
          lib.rs            pub mod + pub use declarations
          file1.rs          one per C++ header/source pair
          subdir/
            mod.rs          sub-module declarations
            file.rs
    """

    # Common V8 type stubs injected into every crate's lib.rs so that
    # cross-module type references compile.  These are opaque placeholder
    # types — they'll be replaced by real definitions once crate wiring
    # is implemented.
    _V8_TYPE_PRELUDE: List[str] = [
        "// ── Common V8 type stubs (placeholders for cross-crate types) ──",
        "",
        "/// V8 isolate — opaque runtime context.",
        "pub type Isolate = ();",
        "/// Raw memory address.",
        "pub type Address = usize;",
        "/// Tagged pointer wrapper.",
        "pub type Tagged<T> = T;",
        "/// Tagged raw value.",
        "pub type Tagged_t = usize;",
        "/// GC-safe handle.",
        "pub type Handle<T> = *mut T;",
        "/// Maybe-valid handle.",
        "pub type MaybeHandle<T> = Option<Handle<T>>;",
        "/// Direct GC handle.",
        "pub type DirectHandle<T> = *mut T;",
        "/// Maybe-valid direct handle.",
        "pub type MaybeDirectHandle<T> = Option<DirectHandle<T>>;",
        "/// V8 local handle.",
        "pub type Local<T> = *mut T;",
        "/// V8 Object base.",
        "pub type Object = ();",
        "/// Machine register.",
        "pub type Register = i32;",
        "/// SIMD128 register.",
        "pub type Simd128Register = i32;",
        "/// Vector register (ARM).",
        "pub type VRegister = i32;",
        "/// Memory operand.",
        "pub type MemOperand = usize;",
        "/// Assembler operand.",
        "pub type Operand = usize;",
        "/// Code label.",
        "pub type Label = usize;",
        "/// Opcode type.",
        "pub type Opcode = u32;",
        "/// Small integer.",
        "pub type Smi = isize;",
        "/// Heap object base.",
        "pub type HeapObject = ();",
        "/// Builtin function arguments.",
        "pub type BuiltinArguments = ();",
        "/// Code object reference.",
        "pub type Code = ();",
        "/// Map (hidden class) type.",
        "pub type Map = ();",
        "/// Context for execution.",
        "pub type Context = ();",
        "/// Native context.",
        "pub type NativeContext = ();",
        "/// JavaScript function.",
        "pub type JSFunction = ();",
        "/// JavaScript object.",
        "pub type JSObject = ();",
        "/// JavaScript receiver.",
        "pub type JSReceiver = ();",
        "/// JavaScript array.",
        "pub type JSArray = ();",
        "/// Fixed array type.",
        "pub type FixedArray = ();",
        "/// Byte array type.",
        "pub type ByteArray = ();",
        "/// V8 string type.",
        "pub type String = std::string::String;",
        "/// V8 name type.",
        "pub type Name = ();",
        "/// V8 symbol type.",
        "pub type Symbol = ();",
        "/// Feedback vector.",
        "pub type FeedbackVector = ();",
        "/// Shared function info.",
        "pub type SharedFunctionInfo = ();",
        "/// Script object.",
        "pub type Script = ();",
        "/// Scope info.",
        "pub type ScopeInfo = ();",
        "/// Bytecode array.",
        "pub type BytecodeArray = ();",
        "/// Runtime function arguments.",
        "pub type RuntimeArguments = ();",
        "/// FPU register.",
        "pub type FPURegister = i32;",
        "/// Double register.",
        "pub type DoubleRegister = i32;",
        "/// Condition code.",
        "pub type Condition = i32;",
        "/// Immediate value.",
        "pub type Immediate = i64;",
        "/// External reference.",
        "pub type ExternalReference = usize;",
        "/// Root index.",
        "pub type RootIndex = u32;",
        "/// Builtin enum.",
        "pub type Builtin = u32;",
        "/// Runtime function id.",
        "pub type RuntimeFunctionId = u32;",
        "/// Compiler node.",
        "pub type Node = ();",
        "/// Processing state (maglev).",
        "pub type ProcessingState = ();",
        "/// Compiler TNode.",
        "pub type TNode<T> = T;",
        "/// Typed array.",
        "pub type JSTypedArray = ();",
        "/// Any JS value.",
        "pub type JSAny = ();",
        "/// Elements kind.",
        "pub type ElementsKind = u32;",
        "/// Expression node.",
        "pub type Expression = ();",
        "/// Variable node.",
        "pub type Variable = ();",
        "/// Value type.",
        "pub type Value = ();",
        "/// Bigint digit type.",
        "pub type digit_t = u64;",
        "/// Bigint digits slice.",
        "pub type Digits = ();",
        "/// Bigint RW digits.",
        "pub type RWDigits = ();",
        "/// Inspector implementation.",
        "pub type V8InspectorImpl = ();",
        "/// CPU register (generic).",
        "pub type CPURegister = i32;",
        "/// Save FP regs mode.",
        "pub type SaveFPRegsMode = u32;",
        "/// XMM register (x64).",
        "pub type XMMRegister = i32;",
        "/// YMM register (x64).",
        "pub type YMMRegister = i32;",
        "/// 16-bit unicode char.",
        "pub type uc16 = u16;",
        "/// Maybe type.",
        "pub type Maybe<T> = Option<T>;",
        "/// Promise resolve/reject.",
        "pub type MaybeLocal<T> = Option<Local<T>>;",
        "/// Ptr compression cage base.",
        "pub type PtrComprCageBase = usize;",
        "/// Compilation info.",
        "pub type CompilationInfo = ();",
        "/// Factory for creating objects.",
        "pub type Factory = ();",
        "/// ReadOnlyRoots.",
        "pub type ReadOnlyRoots = ();",
        "/// Representation.",
        "pub type Representation = u32;",
        "/// LanguageMode.",
        "pub type LanguageMode = u32;",
        "/// Generic template parameter placeholders.",
        "pub type T = ();",
        "pub type K = ();",
        "pub type V = ();",
        "/// Single-letter register aliases (commented — may conflict with locals).",
        "// pub type X = i32;",
        "// pub type Y = i32;",
        "// pub type Z = i32;",
        "/// C++ ostream placeholder.",
        "pub type ostream = ();",
        "/// Bytecode enum.",
        "pub type Bytecode = u32;",
        "/// Operand type.",
        "pub type OperandType = u32;",
        "/// AST raw string.",
        "pub type AstRawString = ();",
        "/// Scope type.",
        "pub type Scope = ();",
        "/// Page allocator.",
        "pub type PageAllocator = ();",
        "/// Node type enum.",
        "pub type NodeType = u32;",
        "/// Root visitor.",
        "pub type RootVisitor = ();",
        "/// Zone type.",
        "pub type Zone = ();",
        "/// Zone list.",
        "pub type ZoneList<U> = Vec<U>;",
        "/// StdoutStream / OFStream placeholder.",
        "pub type StdoutStream = ();",
        "pub type OFStream = ();",
        "/// Digit bits constant.",
        "pub const kDigitBits: usize = 64;",
        "",
    ]

    def __init__(self, rust_emitter, config: TranspilerConfig) -> None:
        self._rust_emitter = rust_emitter
        self._config = config

    # ── Public API ────────────────────────────────────────────────────────

    def emit_module(self, ir_module: IRModule) -> Path:
        """Write all Rust source files for a single crate.

        Returns the crate root directory (e.g., output/transpiled/crates/v8-base/).
        """
        crate_dir = self._crate_dir(ir_module)
        src_dir = crate_dir / "src"
        src_dir.mkdir(parents=True, exist_ok=True)

        # Step 1: Merge .h / .cc pairs that map to the same .rs file.
        merged_files = self._merge_file_pairs(ir_module.files)

        # Step 2: Write each Rust source file.
        written_paths: List[str] = []
        for ir_file in merged_files:
            rel = self._write_source_file(ir_file, crate_dir)
            if rel is not None:
                written_paths.append(rel)

        # Step 3: Generate mod.rs for every intermediate sub-directory.
        self._generate_mod_files(written_paths, src_dir)

        # Step 4: Generate lib.rs at the top level.
        self._generate_lib_rs(ir_module, merged_files, written_paths, src_dir)

        logger.info(
            "Emitted crate %s: %d source files",
            ir_module.crate_name,
            len(written_paths),
        )
        return crate_dir

    # ── File merging ──────────────────────────────────────────────────────

    @staticmethod
    def _merge_file_pairs(files: List[IRFile]) -> List[IRFile]:
        """Merge IRFiles that originate from matching .h / .cc pairs.

        C++ typically splits declarations (.h) and definitions (.cc) for the
        same logical unit.  In Rust everything lives in a single file, so we
        merge items when the IR files map to the same output path (e.g. both
        ``src/bits.rs``).

        Deduplicates items by (type, name) so that a struct/const/function
        defined in both the .h and .cc isn't emitted twice.
        """
        by_path: Dict[str, IRFile] = {}
        for ir_file in files:
            key = ir_file.path
            if key in by_path:
                existing = by_path[key]
                # Merge: accumulate source files, uses, items, and cfg attrs
                existing.cpp_source_files.extend(ir_file.cpp_source_files)
                existing.uses.extend(ir_file.uses)
                # Deduplicate items by (type, name)
                existing_keys = set()
                for item in existing.items:
                    item_key = ModuleEmitter._item_dedup_key(item)
                    if item_key:
                        existing_keys.add(item_key)
                for item in ir_file.items:
                    item_key = ModuleEmitter._item_dedup_key(item)
                    if item_key and item_key in existing_keys:
                        continue  # skip duplicate
                    existing.items.append(item)
                    if item_key:
                        existing_keys.add(item_key)
                existing.cfg_attrs.extend(ir_file.cfg_attrs)
                if ir_file.module_doc and not existing.module_doc:
                    existing.module_doc = ir_file.module_doc
            else:
                by_path[key] = ir_file
        return list(by_path.values())

    @staticmethod
    def _item_dedup_key(item) -> Optional[str]:
        """Return a deduplication key for an item, or None if not dedup-able."""
        type_name = type(item).__name__
        name = getattr(item, "name", None)
        if name:
            # For impl blocks, include the trait name to distinguish inherent
            # impls from trait impls on the same struct.
            if isinstance(item, IRImplBlock):
                return f"{type_name}:{item.struct_name}:{item.trait_name or ''}"
            return f"{type_name}:{name}"
        return None

    # ── Source file writing ───────────────────────────────────────────────

    def _write_source_file(self, ir_file: IRFile, crate_dir: Path) -> Optional[str]:
        """Emit a single Rust source file and return its path relative to src/.

        Returns ``None`` if the file has no items to emit.
        """
        if not ir_file.items:
            return None

        # Compute destination.  IRFile.path is expected to look like
        # "src/foo.rs" or "src/subdir/bar.rs".
        rel_path = ir_file.path
        if not rel_path.startswith("src/"):
            rel_path = f"src/{rel_path}"

        dest = crate_dir / rel_path
        dest.parent.mkdir(parents=True, exist_ok=True)

        # Delegate to the RustEmitter to turn the IRFile into source text.
        rust_source = self._rust_emitter.emit_file(ir_file)

        dest.write_text(rust_source, encoding="utf-8")
        logger.debug("  wrote %s (%d bytes)", dest, len(rust_source))

        # Return path relative to src/ for module resolution.
        return str(PurePosixPath(rel_path).relative_to("src"))

    # ── lib.rs generation ─────────────────────────────────────────────────

    def _generate_lib_rs(
        self,
        ir_module: IRModule,
        merged_files: List[IRFile],
        written_paths: List[str],
        src_dir: Path,
    ) -> None:
        """Generate ``src/lib.rs`` with module declarations and re-exports."""
        lines: List[str] = []

        # Module-level doc comment.
        lines.append(f"//! V8 {ir_module.name} module")
        lines.append(f"//!")
        lines.append(f"//! Auto-generated by the C++ to Rust transpiler.")
        lines.append(f"//! Source module: `{ir_module.name}`")
        lines.append("")

        # Optional crate-level attributes — suppress common warnings in
        # generated code that aren't real errors.
        lines.append("#![allow(dead_code)]")
        lines.append("#![allow(unused_imports)]")
        lines.append("#![allow(unused_variables)]")
        lines.append("#![allow(unused_mut)]")
        lines.append("#![allow(unused_parens)]")
        lines.append("#![allow(unused_unsafe)]")
        lines.append("#![allow(unreachable_code)]")
        lines.append("#![allow(path_statements)]")
        lines.append("#![allow(non_camel_case_types)]")
        lines.append("#![allow(non_snake_case)]")
        lines.append("#![allow(non_upper_case_globals)]")
        lines.append("#![allow(clippy::all)]")
        lines.append("")

        # Common V8 type stubs — opaque placeholder types so cross-module
        # references compile.  Skip types that are defined locally in this
        # crate to avoid conflicts.
        local_types = self._collect_local_type_names(merged_files)
        for prelude_line in self._V8_TYPE_PRELUDE:
            # Check if this line defines a type that's already local
            if prelude_line.startswith("pub type "):
                type_name = prelude_line.split("pub type ")[1].split("<")[0].split("=")[0].strip()
                if type_name in local_types:
                    continue
            lines.append(prelude_line)
        lines.append("")

        # Re-export types from dependency crates so cross-module types
        # are available without explicit imports.
        if ir_module.dependencies:
            lines.append("// ── Cross-crate type imports ──")
            for dep_crate in sorted(ir_module.dependencies):
                # Convert crate name to Rust identifier (hyphens → underscores)
                dep_ident = dep_crate.replace("-", "_")
                lines.append(f"pub use {dep_ident}::*;")
            lines.append("")

        # Collect top-level modules and sub-directory modules from the written
        # file paths.  A path like ``foo.rs`` becomes ``pub mod foo;`` and a
        # path like ``platform/linux.rs`` means we need ``pub mod platform;``
        # (its contents are governed by ``platform/mod.rs``).
        top_level_mods, _subdir_mods = self._classify_modules(written_paths)

        # ``pub mod`` declarations (sorted for determinism).
        if top_level_mods:
            for mod_name in sorted(top_level_mods):
                lines.append(f"pub mod {mod_name};")
            lines.append("")

        # Re-exports for key public types (deduplicated).
        # Track symbol names globally to avoid E0252 name collisions when
        # multiple submodules export the same symbol name.
        reexports = self._collect_reexports(merged_files)
        seen_reexports: Set[str] = set()
        seen_symbols: Set[str] = set()  # bare symbol names already exported
        if reexports:
            for mod_name, symbols in sorted(reexports.items()):
                for symbol in sorted(set(symbols)):
                    key = f"{mod_name}::{symbol}"
                    if key not in seen_reexports and symbol not in seen_symbols:
                        seen_reexports.add(key)
                        seen_symbols.add(symbol)
                        lines.append(f"pub use {key};")
            lines.append("")

        lib_rs = src_dir / "lib.rs"
        lib_rs.write_text("\n".join(lines) + "\n", encoding="utf-8")
        logger.debug("  wrote %s", lib_rs)

    # ── mod.rs generation for sub-directories ─────────────────────────────

    def _generate_mod_files(
        self, written_paths: List[str], src_dir: Path
    ) -> None:
        """Create ``mod.rs`` for every intermediate sub-directory.

        For example if we have ``platform/linux.rs`` and ``platform/win32.rs``
        then ``platform/mod.rs`` is generated with::

            pub mod linux;
            pub mod win32;
        """
        # Build a mapping: directory -> set of child module names.
        dir_children: Dict[str, Set[str]] = defaultdict(set)

        for rel in written_paths:
            parts = PurePosixPath(rel).parts  # e.g. ("platform", "linux.rs")
            if len(parts) < 2:
                continue  # top-level file, handled by lib.rs
            # Register every intermediate directory.
            for depth in range(len(parts) - 1):
                parent = str(PurePosixPath(*parts[: depth + 1]))
                child = parts[depth + 1]
                if child.endswith(".rs"):
                    child = child[:-3]  # strip extension → module name
                dir_children[parent].add(child)

        # Write out each mod.rs.
        for dir_rel, children in sorted(dir_children.items()):
            mod_rs = src_dir / dir_rel / "mod.rs"
            mod_rs.parent.mkdir(parents=True, exist_ok=True)

            lines: List[str] = []
            lines.append(f"//! Sub-module: {dir_rel}")
            lines.append("")
            for child in sorted(children):
                lines.append(f"pub mod {child};")
            lines.append("")

            mod_rs.write_text("\n".join(lines) + "\n", encoding="utf-8")
            logger.debug("  wrote %s", mod_rs)

    # ── Local type name collection ───────────────────────────────────────

    @staticmethod
    def _collect_local_type_names(merged_files: List[IRFile]) -> Set[str]:
        """Collect all struct/enum/trait/type-alias names defined in this crate."""
        names: Set[str] = set()
        for ir_file in merged_files:
            for item in ir_file.items:
                if isinstance(item, (IRStruct, IRTrait, IREnum, IRTypeAlias)):
                    name = getattr(item, "name", None)
                    if name:
                        names.add(name)
        return names

    # ── Re-export collection ──────────────────────────────────────────────

    @staticmethod
    def _collect_reexports(
        merged_files: List[IRFile],
    ) -> Dict[str, List[str]]:
        """Determine which public symbols should be re-exported from lib.rs.

        We re-export public structs, traits, enums, and type aliases so that
        downstream crates can use ``v8_base::SomeType`` without reaching into
        sub-modules.
        """
        reexports: Dict[str, List[str]] = defaultdict(list)

        for ir_file in merged_files:
            mod_name = ModuleEmitter._path_to_mod_name(ir_file.path)
            if mod_name is None:
                continue

            for item in ir_file.items:
                name = ModuleEmitter._public_item_name(item)
                if name is not None:
                    reexports[mod_name].append(name)

        return dict(reexports)

    @staticmethod
    def _public_item_name(item: IRItem) -> Optional[str]:
        """Return the name of *item* if it should be publicly re-exported."""
        name: Optional[str] = None
        if isinstance(item, IRStruct):
            if item.visibility == AccessSpecifier.PUBLIC:
                name = item.name
        elif isinstance(item, IRTrait):
            if item.visibility == AccessSpecifier.PUBLIC:
                name = item.name
        elif isinstance(item, IREnum):
            if item.visibility == AccessSpecifier.PUBLIC:
                name = item.name
        elif isinstance(item, IRTypeAlias):
            if item.visibility == AccessSpecifier.PUBLIC:
                name = item.name
        elif isinstance(item, IRConst):
            if item.visibility == AccessSpecifier.PUBLIC:
                name = item.name
        # Functions, impl blocks, use decls, macros are not re-exported at
        # the crate level by default.

        # Filter out names that aren't valid Rust identifiers (e.g.
        # "(unnamed enum at ...)" from anonymous C++ enums).
        if name and not re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', name):
            return None
        return name

    # ── Module classification helpers ─────────────────────────────────────

    @staticmethod
    def _classify_modules(
        written_paths: List[str],
    ) -> Tuple[List[str], Dict[str, List[str]]]:
        """Classify written paths into top-level modules and sub-directory modules.

        Returns:
            top_level_mods: module names declared directly in lib.rs
            subdir_mods: mapping of sub-directory name -> child module names
        """
        top_level: Set[str] = set()
        subdirs: Dict[str, Set[str]] = defaultdict(set)

        for rel in written_paths:
            parts = PurePosixPath(rel).parts
            if len(parts) == 1:
                # Top-level file, e.g. "bits.rs"
                mod_name = parts[0]
                if mod_name.endswith(".rs"):
                    mod_name = mod_name[:-3]
                top_level.add(mod_name)
            else:
                # Nested file, e.g. "platform/linux.rs"
                # The top-level entry is the first directory component.
                top_level.add(parts[0])
                child = parts[-1]
                if child.endswith(".rs"):
                    child = child[:-3]
                subdirs[parts[0]].add(child)

        return sorted(top_level), {k: sorted(v) for k, v in sorted(subdirs.items())}

    @staticmethod
    def _path_to_mod_name(ir_path: str) -> Optional[str]:
        """Convert an IRFile.path like ``src/foo_bar.rs`` to module name ``foo_bar``.

        For nested paths like ``src/platform/linux.rs`` returns the full
        dotted-style module path ``platform::linux``.  Returns ``None`` for
        paths that cannot be mapped.
        """
        p = PurePosixPath(ir_path)
        # Strip leading "src/" if present.
        parts = list(p.parts)
        if parts and parts[0] == "src":
            parts = parts[1:]
        if not parts:
            return None
        # Strip .rs extension from the leaf.
        leaf = parts[-1]
        if leaf.endswith(".rs"):
            parts[-1] = leaf[:-3]
        return "::".join(parts)

    # ── Path helpers ──────────────────────────────────────────────────────

    def _crate_dir(self, ir_module: IRModule) -> Path:
        """Return the filesystem path for a crate's root directory."""
        return self._config.output_root / "crates" / ir_module.crate_name

    @staticmethod
    def _sanitize_mod_name(name: str) -> str:
        """Ensure *name* is a valid Rust module identifier.

        Replaces hyphens with underscores and strips characters that are
        not alphanumeric or underscore.
        """
        name = name.replace("-", "_")
        name = re.sub(r"[^a-zA-Z0-9_]", "_", name)
        # A module name must not start with a digit.
        if name and name[0].isdigit():
            name = f"_{name}"
        return name

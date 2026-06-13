## Context

The existing `transpiler/` (~13,500 lines of Python) works as a C++ → Rust transpiler for V8 specifically. Its core — `ir/nodes.py`, `extractor/ast_parser.py`, `extractor/stmt_converter.py` — is already generic (zero V8 references). The problem is everything surrounding it: the prelude is hardwired to V8 types, the fallback for unrecognized AST nodes emits `todo!()` (causing compilation failure), and type/macro mappings for V8 are embedded throughout.

The new `cpp2rust/` tool separates these concerns cleanly: a generic core pipeline, a declarative pattern library, and a plugin system for per-codebase customization. V8 becomes `cpp2rust/plugins/v8/`.

No mature C++ → Rust transpiler exists in the open-source ecosystem. c2rust handles C only (C++ is `wontfix`). This is genuinely novel.

## Goals / Non-Goals

**Goals:**
- Build a general C++ → Rust transpiler usable on any C++ codebase given `compile_commands.json`
- Phase 1: always-compiles guarantee — every C++ file produces Rust that `cargo build` accepts
- Plugin architecture: per-codebase YAML config + Python plugin class for type/macro mappings and prelude
- Pattern library: declarative YAML mapping of known C++ idioms to Rust, checked before raw fallback
- Correct Cargo.toml dependency graph auto-generated from `#include` relationships
- Template monomorphization using Clang's instantiation records
- Phase 2 ownership analysis: classify pointers, rewrite to `Box<T>` / `&T` / `Vec<T>` where provable
- V8 bigint as first end-to-end benchmark: pure Rust, no C++ dependency, measured vs original

**Non-Goals:**
- Idiomatic Rust output in Phase 1 (unsafe is acceptable; safety is Phase 2's job)
- Full V8 engine transpilation (GC/Handle<T>/Isolate redesign is out of scope)
- Replacing the Rust compiler or providing formal correctness proofs
- Support for C (use c2rust instead)

## Decisions

### D1: Phase 1 never emits `todo!()` — uses `Default::default()` + comment instead

**Decision:** When an AST node cannot be translated, emit `{ let _r: T = Default::default(); /* UNTRANSLATED: <cpp source> */ _r }` in an `unsafe` block. The code compiles and returns a zero/empty value. The `// UNTRANSLATED` comment makes every gap visible and greppable.

**Why not `todo!()`:** `todo!()` panics at runtime and — when used in expression position — can cause type inference failures that prevent compilation entirely. The goal of Phase 1 is a binary that compiles and runs.

**Why not `extern "C"` shim:** FFI overhead pollutes benchmark numbers. Pure Rust output keeps the benchmark clean.

**Alternative considered:** Keep `todo!()` as a marker but fix type inference issues. Rejected: the panic is still a runtime failure; Default::default() is strictly better for a working Phase 1.

---

### D2: Plugin system uses YAML config + Python class inheritance

**Decision:** Each plugin provides:
1. `config.yaml` — declarative: type mappings, macro mappings, module layout, prelude path
2. `plugin.py` — imperative: a Python class subclassing `CppPlugin` for complex patterns not expressible in YAML

**Why YAML + Python (not just Python):** Simple mappings (type aliases, macro substitutions) should be readable without running code. The YAML layer handles ~80% of cases; Python handles the 20% that need logic (e.g., V8's `Handle<T>` parameterized replacement).

**Why not a JSON schema or DSL:** YAML is the existing convention in the codebase (pattern library files), readable, and supports comments.

**Plugin interface:**
```python
class CppPlugin:
    def map_type(self, cpp_type: str) -> Optional[str]: ...
    def map_macro(self, name: str, args: List[str]) -> Optional[str]: ...
    def prelude_lines(self) -> List[str]: ...
    def crate_for_file(self, path: str) -> Optional[str]: ...
```

---

### D3: Template strategy — tiered (plugin-generic → structural → monomorphize)

**Decision:** Three tiers applied in order:
1. **Plugin-annotated**: If the plugin declares `Handle<T>` is a generic wrapper, emit `struct Handle<T>` with plugin-specified bounds. Used for well-understood templates.
2. **Structural**: Templates with simple container structure (one-field wrappers, pair-like) detected and emitted as Rust generics automatically.
3. **Monomorphize**: Everything else — enumerate Clang's instantiation records, emit one concrete Rust type/function per instantiation.

**Why not monomorphize-everything:** V8 alone has dozens of `Handle<T>` instantiations. Code explosion hurts readability and compile time. Plugin-generic handles the common case.

**Why not generics-everything:** C++ templates are duck-typed; converting to Rust generics requires inferring trait bounds which is an unsolved general problem. Monomorphization always works.

---

### D4: Dependency wiring from #include graph

**Decision:** The extractor already tracks `#include` directives in `ExtractedFile.includes`. A new `dep_wirer.py` pass:
1. Builds a file→crate map from the plugin's module layout
2. For each file's includes, walks the include list and resolves to crate names
3. Emits `[dependencies]` sections in each crate's `Cargo.toml`

**Why this fixes E0425:** The dominant error (11,758 unresolved names) is caused by types from crate A being used in crate B with no `use v8_a::*` or Cargo dependency declared. Fixing the Cargo.toml is the highest-leverage single change.

---

### D5: Ownership analysis operates on the C++ IR, not on emitted Rust

**Decision:** Phase 2 ownership analysis runs as an analysis pass on the IR (between extraction and emission), not as a post-processing pass on the emitted Rust source text.

**Why:** Post-processing Rust source is brittle (regex on code). The IR has the structured type information needed for ownership inference. CROWN's approach of analyzing Rust MIR is equivalent but requires the Rust compiler to be involved; our IR-level analysis is faster and more portable.

**Algorithm sketch (constraint propagation):**
- Every `*mut T` pointer gets a constraint variable
- Rules: if a pointer is stored in a struct field and the struct is `new`-allocated → OWNING; if a pointer is a function parameter that is never stored → BORROW; if pointer arithmetic is used → ARRAY; if two paths both write through the same pointer → ALIASED (cannot safe-ify)
- Fixed-point iteration resolves all constraints
- Unresolvable → stays `*mut T` in unsafe block

---

### D6: Directory layout — cpp2rust/ alongside transpiler/

**Decision:** Create `cpp2rust/` as a sibling to `transpiler/`. The old `transpiler/` is preserved as a reference baseline until the V8 bigint benchmark is validated.

**Migration path:** Once `cpp2rust` produces equivalent or better output for V8 bigint, `transpiler/` is archived. Full migration of all 49 crates follows incrementally.

---

### D7: Pattern library in YAML files under `cpp2rust/patterns/`

**Decision:** Known C++ → Rust idiom mappings stored as YAML, keyed by pattern name. Applied in `stmt_converter` after the structured handlers and before the raw fallback.

```yaml
# patterns/bit_ops.yaml
- name: bit_cast
  cpp_pattern: "base::bit_cast<{T}>({expr})"
  rust_emit: "std::mem::transmute::<_, {T}>({expr})"
  needs_unsafe: true

- name: count_leading_zeros_u32
  cpp_pattern: "__builtin_clz({expr})"
  rust_emit: "({expr} as u32).leading_zeros() as i32"
  needs_unsafe: false
```

Pattern files: `stdlib.yaml`, `bit_ops.yaml`, `memory.yaml`, `platform.yaml`, `v8_macros.yaml` (V8 plugin).

## Risks / Trade-offs

**[Risk] Clang template instantiation records may be incomplete for header-only templates**
→ Mitigation: Fall through to monomorphize-from-usage: scan all call sites that instantiate the template, collect concrete types, emit them. Worst case: a template with no instantiations is emitted as a Rust generic with no bounds (compiles, may not be callable).

**[Risk] Ownership analysis has false negatives — some safe patterns stay unsafe**
→ Mitigation: False negatives are safe (too-conservative = stays `*mut T` in unsafe). False positives (incorrectly classifying aliased pointer as owning) would be unsafe code that compiles but has UB. Mitigation: only rewrite to owned when the constraint solver reaches OWNING with no aliasing edges. Err on the side of staying unsafe.

**[Risk] Pattern library matches incorrectly (partial string match on C++ source)**
→ Mitigation: Patterns match against the Clang-normalized token stream, not raw source text. Template args are resolved before matching. Add integration tests for every pattern entry.

**[Risk] V8 bigint uses C++ features not covered by Phase 1**
→ Mitigation: The 5 known gaps (CXX_UNARY_EXPR/sizeof, TYPE_REF as expr, bit_cast, make_unique, inter-crate deps) are all addressed by this design. The fallback handles anything else — it compiles even if wrong.

**[Risk] Performance of the transpiler itself on 2,775 files**
→ Mitigation: ExtractedFile JSON is cached per-file with mtime invalidation (already implemented in rusty-v8). Incremental re-transpilation is fast.

## Migration Plan

1. Build `cpp2rust/core/` with ported clean files from `transpiler/`
2. Build V8 plugin (`cpp2rust/plugins/v8/`)
3. Run on bigint module, measure error count vs old transpiler
4. Fix gaps until `v8-bigint` crate compiles clean
5. Run benchmark, record results
6. Extend to `v8-base`, `v8-numbers` crates
7. Archive `transpiler/` once cpp2rust covers all 49 crates

**Rollback:** `transpiler/` is kept intact. Any regression → switch back to old tool. No user-facing API to break.

## Open Questions

- **Q1**: Should `cpp2rust` be published as a standalone tool (separate GitHub repo, crates.io) alongside the V8 benchmark? Decision needed before writing README / packaging.
- **Q2**: For V8's `Handle<T>` — should the plugin emit a generic `struct Handle<T>` backed by raw pointer, or use `NonNull<T>` for null-safety? Affects benchmark correctness for null-handle paths.
- **Q3**: Benchmark harness — use Criterion.rs for Rust side, Google Benchmark for C++ side? Or a single custom harness for apples-to-apples comparison?

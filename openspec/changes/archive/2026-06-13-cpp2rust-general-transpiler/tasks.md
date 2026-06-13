## 1. Project Scaffold

- [x] 1.1 Create `cpp2rust/` directory structure: `core/extractor/`, `core/ir/`, `core/analysis/`, `core/emit/`, `core/mapper/`, `plugins/v8/`, `patterns/`, `tests/unit/`, `tests/integration/`
- [x] 1.2 Create `cpp2rust/__init__.py` and `cpp2rust/core/__init__.py` package files
- [x] 1.3 Create `cpp2rust/core/plugin.py` with `CppPlugin` base class defining the interface: `map_type`, `map_macro`, `prelude_lines`, `crate_for_file`, `extra_patterns`
- [x] 1.4 Create `cpp2rust/cli.py` with argument parsing: `--plugin`, `--module`, `--compile-commands`, `--output`, `--phase` (1 or 2)
- [x] 1.5 Create `cpp2rust/main.py` as the entry point wiring CLI → pipeline

## 2. Port Generic Core (from transpiler/)

- [x] 2.1 Copy `transpiler/ir/nodes.py` → `cpp2rust/core/ir/nodes.py`; remove the 5 V8 references (kDigitBits comment, etc.) making it fully generic
- [x] 2.2 Copy `transpiler/ir/type_registry.py` → `cpp2rust/core/ir/type_registry.py` (zero changes needed)
- [x] 2.3 Copy `transpiler/ir/dependency_graph.py` → `cpp2rust/core/ir/dependency_graph.py` (zero changes needed)
- [x] 2.4 Copy `transpiler/extractor/ast_parser.py` → `cpp2rust/core/extractor/ast_parser.py`; refactor to accept `compile_commands.json` as primary input instead of `TranspilerConfig`
- [x] 2.5 Copy `transpiler/extractor/stmt_converter.py` → `cpp2rust/core/extractor/stmt_converter.py`; remove 5 V8-specific references; wire in pattern library hook (call pattern library when structured handlers miss)
- [x] 2.6 Copy `transpiler/extractor/include_resolver.py` → `cpp2rust/core/extractor/include_resolver.py`
- [x] 2.7 Copy generic parts of `transpiler/mapper/type_mapper.py` → `cpp2rust/core/mapper/type_mapper.py`; strip V8-specific mappings into V8 plugin
- [x] 2.8 Copy `transpiler/mapper/stdlib_mapper.py` → `cpp2rust/core/mapper/stdlib_mapper.py` (already generic)

## 3. Fix the Always-Compiles Invariant

- [x] 3.1 Add `IRRawCppExpr` node to `core/ir/nodes.py` with fields: `cpp_source: str`, `default_type: str = "i32"`, `comment: str`
- [x] 3.2 Add `IRRawCppStmt` node to `core/ir/nodes.py` with field: `cpp_source: str`
- [x] 3.3 In `stmt_converter.py`, change all `IRRawExpr` fallback paths to produce `IRRawCppExpr` instead; change all `IRRawStmt` paths to `IRRawCppStmt`
- [x] 3.4 In the unsafe emitter, implement `_emit_raw_cpp_expr()`: emit `unsafe { Default::default() /* UNTRANSLATED: <cpp_source> */ }` — never `todo!()`
- [x] 3.5 In the unsafe emitter, implement `_emit_raw_cpp_stmt()`: emit `unsafe { let _ = 0; /* UNTRANSLATED: <cpp_source> */ }` — never `todo!()`
- [x] 3.6 Add `grep -r "todo!()" output/transpiled/` to the test suite as a failing assertion — zero `todo!()` must be a hard invariant

## 4. Pattern Library

- [x] 4.1 Create `cpp2rust/core/pattern_library.py` with `PatternLibrary` class: loads YAML files, provides `match(cpp_token_stream) -> Optional[IRExpr]` method
- [x] 4.2 Create `cpp2rust/patterns/bit_ops.yaml` with entries: `bit_cast`, `__builtin_clz`, `__builtin_ctz`, `__builtin_popcount`, `__builtin_expect`
- [x] 4.3 Create `cpp2rust/patterns/memory.yaml` with entries: `std::make_unique`, `std::make_shared`, `std::move`, `std::forward`
- [x] 4.4 Create `cpp2rust/patterns/stdlib.yaml` with entries: `std::min`, `std::max`, `std::swap`, `std::fill`, `std::copy`, `memcpy`, `memset`, `memmove`
- [x] 4.5 Create `cpp2rust/patterns/platform.yaml` with entries: `alignof`, `sizeof` (type), `sizeof` (expr), `reinterpret_cast` (ptr), `static_cast` (numeric), `const_cast`
- [x] 4.6 Wire `PatternLibrary` into `stmt_converter.py`: after all structured CursorKind handlers, before falling back to `IRRawCppExpr`, call `pattern_library.match()`
- [x] 4.7 Write unit tests for each pattern entry: given a synthetic Clang cursor, assert the correct IRExpr is produced

## 5. Plugin System

- [x] 5.1 Implement plugin loader in `cli.py`: given `--plugin v8`, import `cpp2rust/plugins/v8/plugin.py` and instantiate its plugin class
- [x] 5.2 Implement YAML config loader: parse `plugins/v8/config.yaml`, apply `type_mappings` to the type mapper, `macro_mappings` to the pattern library, `skip_patterns` to file discovery
- [x] 5.3 Implement prelude injection in `core/emit/cargo_emitter.py`: write plugin's prelude to `crates/<name>/src/prelude.rs` and emit `pub mod prelude; pub use prelude::*;` at the top of `lib.rs`
- [x] 5.4 Implement `plugin.extra_patterns()` merge into the global pattern library (plugin patterns have higher priority than built-in patterns)

## 6. V8 Plugin

- [x] 6.1 Create `cpp2rust/plugins/v8/config.yaml` with: `type_mappings` for `digit_t`/`twodigit_t`/`signed_digit_t`, `module_layout` rules for all 49 V8 crates, `skip_patterns` for test/torque/d8 files
- [x] 6.2 Port `transpiler/mapper/v8_mapper.py` → `cpp2rust/plugins/v8/v8_mapper.py`; integrate as the plugin's `map_type` method
- [x] 6.3 Create `cpp2rust/plugins/v8/patterns.yaml` with V8 macro entries: `DCHECK`, `DCHECK_EQ/LT/LE/GT/GE/NE`, `CHECK`, `CHECK_EQ/*`, `SLOW_DCHECK`, `USE(x)`, `V8_LIKELY`, `V8_UNLIKELY`
- [x] 6.4 Create `cpp2rust/plugins/v8/prelude.rs` with: `digit_t`, `twodigit_t`, `signed_digit_t` type aliases; `Digits` struct; `RWDigits` struct; `Handle<T>` generic struct; `kDigitBits`/`kLog2DigitBits`/`kHalfDigitBits`/`kHalfDigitMask` constants
- [x] 6.5 Implement `V8Plugin.map_type()` for parameterized types: `Handle<T>` → `Handle<T>` (generic), `MaybeHandle<T>` → `Option<Handle<T>>`, `Tagged<T>` → `Tagged<T>`
- [x] 6.6 Write integration test: run V8 plugin on a small V8 snippet with DCHECK and Handle<T>, assert output matches expected Rust

## 7. Dependency Wirer

- [x] 7.1 Create `cpp2rust/core/analysis/dep_wirer.py` with `DepWirer` class
- [x] 7.2 Implement `DepWirer.build_graph(extracted_files, plugin)`: for each file's includes, resolve include paths to crate names using plugin's `crate_for_file` and the module layout; build adjacency list
- [x] 7.3 Implement `DepWirer.detect_cycles()`: topological sort; for any cycle, record merged-crate mapping and emit warning
- [x] 7.4 Implement `DepWirer.emit_cargo_tomls(output_dir)`: for each crate, write `Cargo.toml` with correct `[dependencies]` using `path = "../<dep>"` for workspace crates
- [x] 7.5 Implement workspace `Cargo.toml` regeneration: collect all crate names, write `[workspace] members = [...]`
- [x] 7.6 Write integration test: process bigint + base modules, assert `v8-bigint/Cargo.toml` contains `v8-base` dependency

## 8. Template Resolver

- [x] 8.1 Create `cpp2rust/core/analysis/template_resolver.py` with `TemplateResolver` class
- [x] 8.2 Implement instantiation collection: scan all `ExtractedClass` and `ExtractedFunction` entries with non-empty `template_params`; collect all concrete arg tuples from `TEMPLATE_REF` cursors in function bodies
- [x] 8.3 Implement Tier 1 (plugin-generic): check each template name against plugin's `generic_templates` config; if listed, emit as Rust generic with plugin-specified bounds
- [x] 8.4 Implement Tier 2 (structural detection): detect single-field wrapper templates and pair-like templates; emit as Rust generics automatically
- [x] 8.5 Implement Tier 3 (monomorphize): for remaining templates, emit one concrete Rust type/function per instantiation using `<TemplateName>_<ArgType>` naming
- [x] 8.6 Implement collision detection and numeric suffix resolution for monomorphized names
- [x] 8.7 Write unit tests: given a template with 3 instantiations (one plugin-generic, one structural, one monomorphized), assert all three emit correctly

## 9. Fill the 5 Known V8 Bigint Gaps

- [x] 9.1 Add `CursorKind.CXX_UNARY_EXPR` handler to `stmt_converter.py`: detect `sizeof(T)` and `sizeof(expr)`, emit `IRSizeofExpr`; pattern library entry for `sizeof(digit_t)` → `std::mem::size_of::<digit_t>()`
- [x] 9.2 Add `CursorKind.TYPE_REF` handler in expression position: when a `TYPE_REF` cursor appears as a child of a call expr or construct expr, extract the type name as `IRNameRef` with the type's spelling
- [x] 9.3 Add `bit_cast` pattern to `patterns/bit_ops.yaml` and verify it matches `base::bit_cast<uint64_t>(value)` in the bigint source
- [x] 9.4 Add `std::make_unique` pattern to `patterns/memory.yaml` and verify it matches the allocator usage in `accounting_allocator.cc`
- [x] 9.5 Run `cpp2rust transpile --plugin v8 --module bigint` and reduce output errors to zero; fix any remaining issues found

## 10. Phase 1 Unsafe Emitter (from transpiler/codegen/)

- [x] 10.1 Copy `transpiler/codegen/rust_emitter.py` → `cpp2rust/core/emit/unsafe_emitter.py`; strip V8-specific post-processing regexes (those move to V8 plugin's `post_process` hook)
- [x] 10.2 Implement `_emit_raw_cpp_expr` and `_emit_raw_cpp_stmt` in the unsafe emitter (task 3.4 / 3.5 above)
- [x] 10.3 Implement struct containment inheritance emission (task in phase1-emitter spec)
- [x] 10.4 Implement vtable struct emission for abstract classes
- [x] 10.5 Implement operator overload emission as free functions
- [x] 10.6 Add plugin `post_process(source: str) -> str` hook called after emission; move all V8 DCHECK/cerr regexes into `V8Plugin.post_process()`
- [x] 10.7 Rewrite `core/emit/cargo_emitter.py` to be config-driven (no hardwired module names); driven by plugin's module layout

## 11. V8 Bigint Compilation Target

- [x] 11.1 Run full transpilation of `codebase/src/bigint/` with cpp2rust V8 plugin
- [x] 11.2 Run `cargo check -p v8-bigint`; record remaining error count
- [x] 11.3 Fix all remaining compile errors in v8-bigint until `cargo check` exits 0
- [x] 11.4 Run `cargo build -p v8-bigint` and confirm it produces a `.rlib`
- [x] 11.5 Add a `cargo test -p v8-bigint` stub that exercises `digit_mul`, `inplace_add`, `inplace_sub`, `product_greater_than` with known inputs

## 12. Benchmark Harness

- [x] 12.1 Create `benchmark/` directory with `run_bigint_bench.py`, `bigint_bench.cc` (C++ side), and `bigint_bench_rust/` (Rust side using Criterion)
- [x] 12.2 Implement C++ benchmark using hand-written driver (not Google Benchmark dependency): measure wall-clock ns for Multiply, DivideSchoolbook, Add, ToString on 10/100/1000 digit inputs
- [x] 12.3 Implement Rust benchmark using Criterion.rs for the same operations and input sizes
- [x] 12.4 Implement `run_bigint_bench.py`: compile both, run both, collect JSON results, print comparison table (operation, N digits, C++ ns, Rust ns, speedup)
- [x] 12.5 Verify bit-identical results between C++ and Rust on 100 random test cases before recording timing

## 13. Tests and Validation

- [x] 13.1 Write unit tests in `tests/unit/` for `PatternLibrary.match()`: one test per pattern entry
- [x] 13.2 Write unit tests for `DepWirer`: test cycle detection, test Cargo.toml generation, test workspace member list
- [x] 13.3 Write unit tests for `TemplateResolver`: test all three tiers (plugin-generic, structural, monomorphize)
- [x] 13.4 Write unit tests for `OwnershipAnalysis`: OWNING, BORROW, ARRAY, ALIASED classifications on synthetic IR
- [x] 13.5 Write integration test: transpile `tests/integration/snippets/basic_class.cpp` and assert the Rust output contains expected struct/impl structure
- [x] 13.6 Write integration test: transpile `tests/integration/snippets/templates.cpp` with a simple template and assert monomorphized output
- [x] 13.7 Write integration test: confirm zero `todo!()` in any output file after transpiling bigint module
- [x] 13.8 Write integration test: `cargo check -p v8-bigint` exits 0 after transpilation (CI-runnable)
